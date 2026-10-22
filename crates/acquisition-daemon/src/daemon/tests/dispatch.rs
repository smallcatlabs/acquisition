//! The dispatcher on the wire: independent progress under waits, priority
//! on a scheduling key, the headerless 503, malformed listings, declared
//! route knowledge, the 429 family and real mode's whoami.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::*;
use crate::ratelimit::Clock;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::Semaphore;

struct BlockingClock {
    now: Mutex<Instant>,
    origin: Mutex<Instant>,
    sleepers: AtomicUsize,
    releases: Semaphore,
}

impl BlockingClock {
    fn new() -> Self {
        let start = Instant::now();
        BlockingClock {
            now: Mutex::new(start),
            origin: Mutex::new(start),
            sleepers: AtomicUsize::new(0),
            releases: Semaphore::new(0),
        }
    }

    async fn wait_for_sleepers(&self, expected: usize) {
        tokio::time::timeout(Duration::from_secs(1), async {
            while self.sleepers.load(Ordering::SeqCst) < expected {
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("dispatcher jobs reached their limiter waits");
    }

    fn release(&self, count: usize) {
        self.releases.add_permits(count);
    }
}

impl Clock for BlockingClock {
    fn now(&self) -> Instant {
        *self.now.lock().unwrap()
    }

    fn wall(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH
            + MANUAL_WALL_ORIGIN
            + self.now().duration_since(*self.origin.lock().unwrap())
    }

    fn kind(&self) -> &'static str {
        "manual"
    }

    fn sleep(&self, duration: Duration) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            self.sleepers.fetch_add(1, Ordering::SeqCst);
            self.releases
                .acquire()
                .await
                .expect("test clock remains open")
                .forget();
            let mut now = self.now.lock().unwrap();
            *now = now.checked_add(duration).expect("bounded test deadline");
        })
    }
}

async fn saturated_probe_server() -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        for (path, policy) in [
            ("/character", "character-test-policy"),
            ("/stash/Standard", "stash-list-test-policy"),
        ] {
            let (mut stream, _) = listener.accept().await.unwrap();
            let request = mockggg::read_request(&mut stream).await.unwrap();
            assert_eq!(request.method, "HEAD");
            assert_eq!(request.path, path);
            let headers = format!(
                "X-Rate-Limit-Policy: {policy}\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 1:1:60\r\nX-Rate-Limit-Account-State: 1:1:0\r\n"
            );
            mockggg::respond_with(
                &mut stream,
                "204 No Content",
                "application/json",
                &headers,
                "",
            )
            .await;
        }
    });
    (base, task)
}

fn fetch_payload_marker(outcome: &Outcome) -> &str {
    let Outcome::Success { payload } = outcome else {
        panic!("expected successful fetch, got {outcome:?}")
    };
    payload["items"][0].as_str().unwrap()
}

fn terminal_event_count(receiver: &mut broadcast::Receiver<JobInfo>, id: JobId) -> usize {
    let mut count = 0;
    while let Ok(info) = receiver.try_recv() {
        if info.id == id && info.state.is_terminal() {
            count += 1;
        }
    }
    count
}

/// `wait_ms` of every send, in journal order.
fn journal_waits(log_path: &std::path::Path) -> Vec<u64> {
    journal_sends(log_path)
        .iter()
        .map(|l| l["wait_ms"].as_u64().expect("wait_ms"))
        .collect()
}

#[tokio::test]
async fn dispatcher_auth_waits_do_not_cap_independent_job_progress() {
    let (base, arrived, release, server) = delayed_token_server().await;
    let clock = Arc::new(ManualClock::new());
    let (mut daemon, log_path) = test_daemon(&base, clock);
    Arc::get_mut(&mut daemon).unwrap().credential_store = Arc::new(NoopCredentialStore);
    {
        let mut shared = daemon.shared.lock().unwrap();
        shared.auth.one_mut().refresh_token = Some("rt-old".into());
        shared.auth.rename("", "old-user");
    }
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let first = daemon
        .submit("whoami".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let second = daemon
        .submit("whoami".into(), json!({}), 0, "test".into(), None)
        .unwrap();

    // Both jobs are dispatched and held behind the one token request
    // the server has not answered — read as any client reads it, from
    // the job list (C21: an auth wait occupies no dispatcher slot).
    arrived.await.unwrap();
    tokio::time::timeout(Duration::from_secs(1), async {
        loop {
            let running = daemon
                .shared
                .lock()
                .unwrap()
                .list(&daemon)
                .into_iter()
                .filter(|job| job.state == JobState::Running)
                .count();
            if running == 2 {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("both jobs running behind the held refresh");

    let independent = daemon
        .submit(
            "sleep".into(),
            json!({ "seconds": 0.0 }),
            0,
            "test".into(),
            None,
        )
        .unwrap();
    let (info, _) = wait_terminal(&daemon, independent).await;
    assert_eq!(info.state, JobState::Done);

    release.send(()).unwrap();
    assert_eq!(wait_terminal(&daemon, first).await.0.state, JobState::Done);
    assert_eq!(wait_terminal(&daemon, second).await.0.state, JobState::Done);
    server.await.unwrap();
    finish_harness(dispatcher, &log_path);
}

#[tokio::test]
async fn dispatcher_rate_waits_do_not_cap_independent_job_progress() {
    let (base, server) = saturated_probe_server().await;
    let clock = Arc::new(BlockingClock::new());
    let (daemon, log_path) = test_daemon(&base, clock.clone());
    daemon
        .choke
        .head(
            "character-list",
            &format!("{base}/character"),
            None,
            daemon.choke.now(),
        )
        .await
        .unwrap();
    daemon
        .choke
        .head(
            "stash-list",
            &format!("{base}/stash/Standard"),
            None,
            daemon.choke.now(),
        )
        .await
        .unwrap();
    server.await.unwrap();

    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let characters = daemon
        .submit("characters".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let stashes = daemon
        .submit(
            "stashes".into(),
            json!({ "league": "Standard" }),
            0,
            "test".into(),
            None,
        )
        .unwrap();
    clock.wait_for_sleepers(2).await;

    let independent = daemon
        .submit(
            "sleep".into(),
            json!({ "seconds": 0.0 }),
            0,
            "test".into(),
            None,
        )
        .unwrap();
    let (info, _) = wait_terminal(&daemon, independent).await;
    assert_eq!(info.state, JobState::Done);

    // The dispatcher pre-waits in one-second cancellation slices.
    clock.release(20);
    assert_eq!(
        wait_terminal(&daemon, characters).await.0.state,
        JobState::Failed
    );
    assert_eq!(
        wait_terminal(&daemon, stashes).await.0.state,
        JobState::Failed
    );
    finish_harness(dispatcher, &log_path);
}

/// C5: on one scheduling key the higher priority goes out first,
/// whatever the order of submission — read at the wire: the server
/// answers its first GET "first", and the later-submitted,
/// higher-priority job is the one that holds it.
#[tokio::test]
async fn the_higher_priority_job_on_a_scheduling_key_reaches_the_wire_first() {
    let policy = "X-Rate-Limit-Policy: dispatcher-test-policy\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 100:1:60\r\nX-Rate-Limit-Account-State: 0:1:0\r\n";
    let get = |marker: &str| ScriptedResponse {
        method: "GET",
        path: "/fetch",
        status: "200 OK",
        headers: policy.into(),
        body: format!(r#"{{"items":["{marker}"]}}"#),
    };
    let responses = vec![
        ScriptedResponse {
            method: "HEAD",
            path: "/fetch",
            status: "204 No Content",
            headers: policy.into(),
            body: String::new(),
        },
        get("first"),
        get("second"),
    ];
    let (base, requests, server) = scripted_server(responses).await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon(&base, clock);
    // Both are queued before anything dispatches.
    let low = daemon
        .submit("fetch".into(), json!({}), 1, "test".into(), None)
        .unwrap();
    let high = daemon
        .submit("fetch".into(), json!({}), 9, "test".into(), None)
        .unwrap();
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());

    let (info, outcome) = wait_terminal(&daemon, high).await;
    assert_eq!(info.state, JobState::Done);
    assert_eq!(fetch_payload_marker(&outcome), "first");
    let (info, outcome) = wait_terminal(&daemon, low).await;
    assert_eq!(info.state, JobState::Done);
    assert_eq!(fetch_payload_marker(&outcome), "second");
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
}

/// Rung 10 (2026-08-24) with the tripwire off: a 503 with no rate
/// headers fails its job and is never retried, and the *next* job's send
/// is paced as if the 503 counted — visible as `wait_ms` in the journal.
#[tokio::test]
async fn headerless_503_fails_its_job_and_paces_the_next_send() {
    let policy = "X-Rate-Limit-Policy: dispatcher-test-policy\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 1:10:60\r\nX-Rate-Limit-Account-State: 0:10:0\r\n";
    let responses = vec![
        ScriptedResponse {
            method: "HEAD",
            path: "/fetch",
            status: "204 No Content",
            headers: policy.into(),
            body: String::new(),
        },
        ScriptedResponse {
            method: "GET",
            path: "/fetch",
            status: "503 Service Unavailable",
            headers: String::new(),
            body: "<html><center>openresty</center></html>".into(),
        },
        ScriptedResponse {
            method: "GET",
            path: "/fetch",
            status: "200 OK",
            headers: policy.replace("0:10:0", "1:10:0"),
            body: r#"{"items":["done"]}"#.into(),
        },
    ];
    let (base, requests, server) = scripted_server(responses).await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon(&base, clock);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let first = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let second = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();

    let (info, outcome) = wait_terminal(&daemon, first).await;
    assert_eq!(info.state, JobState::Failed);
    assert_eq!(info.retries, 0);
    let Outcome::Failure { error, .. } = outcome else {
        panic!("503 did not fail")
    };
    assert!(error.contains("origin error page"), "{error}");
    let (info, outcome) = wait_terminal(&daemon, second).await;
    assert_eq!(info.state, JobState::Done);
    assert_eq!(fetch_payload_marker(&outcome), "done");
    assert_eq!(requests.lock().unwrap().as_slice(), ["HEAD", "GET", "GET"]);
    let waits = journal_waits(&log_path);
    assert_eq!(waits[..2], [0, 0]);
    // 10 s period + the 5 s initial-window bucket + 1 s buffer, measured
    // from the 503 itself — the only known in-window hit.
    assert_eq!(
        waits[2], 16_000,
        "the send after a headerless 503 waits as if the 503 filled the window: {waits:?}"
    );
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
}

/// Malformed listings fail the *job*, not just the store: a 2xx with
/// no `stashes` array is refused by the job's own guard, and a listing
/// whose entries lack ids is refused by the store (`MalformedBody`)
/// and propagated through `record` — neither reports success, and
/// neither leaves a response row a snapshot could cite as a basis.
#[tokio::test]
async fn malformed_listings_fail_the_job_not_just_the_store() {
    let policy = "X-Rate-Limit-Policy: stash-list-test\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 100:1:60\r\nX-Rate-Limit-Account-State: 0:1:0\r\n";
    let responses = vec![
        ScriptedResponse {
            method: "HEAD",
            path: "/stash/Standard",
            status: "204 No Content",
            headers: policy.into(),
            body: String::new(),
        },
        ScriptedResponse {
            method: "GET",
            path: "/stash/Standard",
            status: "200 OK",
            headers: policy.replace("0:1:0", "1:1:0"),
            body: r#"{"error":"maintenance"}"#.into(),
        },
        ScriptedResponse {
            method: "GET",
            path: "/stash/Standard",
            status: "200 OK",
            headers: policy.replace("0:1:0", "2:1:0"),
            body: r#"{"stashes":[{"name":"NoId","type":"PremiumStash"}]}"#.into(),
        },
    ];
    let (base, requests, server) = scripted_server(responses).await;
    let clock = Arc::new(ManualClock::new());
    let (mut daemon, log_path) = test_daemon(&base, clock);
    let store_dir = std::env::temp_dir().join(format!(
        "acq-malformed-job-{}-{}",
        std::process::id(),
        TEST_LOG_ID.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&store_dir);
    Arc::get_mut(&mut daemon).expect("fresh daemon").store_dir = Some(store_dir.clone());
    {
        let mut s = daemon.shared.lock().unwrap();
        s.auth.rename("", "Alice#1234");
        let session = s.auth.one_mut();
        session.username = Some("Alice#1234".into());
        session.access_token = Some("at-test.Alice#1234".into());
        session.access_expires_at = Some(daemon.choke.wall() + Duration::from_secs(3600));
    }
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let account = Some("Alice#1234".to_string());
    let first = daemon
        .submit(
            "stashes".into(),
            json!({}),
            0,
            "test".into(),
            account.clone(),
        )
        .unwrap();
    let (info, outcome) = wait_terminal(&daemon, first).await;
    assert_eq!(info.state, JobState::Failed);
    let Outcome::Failure { error, .. } = outcome else {
        panic!("missing array did not fail the job")
    };
    assert!(error.contains("stashes"), "{error}");
    let second = daemon
        .submit("stashes".into(), json!({}), 0, "test".into(), account)
        .unwrap();
    let (info, outcome) = wait_terminal(&daemon, second).await;
    assert_eq!(info.state, JobState::Failed);
    let Outcome::Failure { error, .. } = outcome else {
        panic!("id-less entry did not fail the job")
    };
    assert!(error.contains("malformed stashes response"), "{error}");
    // Nothing recorded either way: no tabs retired, no false basis.
    let store = Store::open(&account_path(&store_dir, "Alice#1234")).unwrap();
    assert_eq!(store.status().unwrap().responses, 0);
    drop(store);
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
    let _ = std::fs::remove_dir_all(&store_dir);
}

/// The route knowledge declared for `/profile` and `/account/leagues`
/// (first contact 2026-08-30), held against the mock that mirrors GGG:
/// neither is probed; `/profile`'s headerless 200 is accepted and the
/// endpoint becomes Policyless; `/account/leagues` teaches its policy
/// from the GET. Removing a declaration fails here, not live.
#[tokio::test]
async fn declared_route_knowledge_holds_against_the_mock() {
    let base = mockggg::start().await.unwrap();
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon(&base, clock);
    {
        let mut s = daemon.shared.lock().unwrap();
        s.auth.rename("", "Alice#1234");
        let session = s.auth.one_mut();
        session.username = Some("Alice#1234".into());
        session.access_token = Some("at-test.Alice#1234".into());
        session.access_expires_at = Some(daemon.choke.wall() + Duration::from_secs(3600));
    }
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    // Resolution happens in the `Submit` handler; here the account is
    // passed as that handler would after resolving the sole session.
    let account = Some("Alice#1234".to_string());
    let profile = daemon
        .submit(
            "profile".into(),
            json!({}),
            0,
            "test".into(),
            account.clone(),
        )
        .unwrap();
    let (info, outcome) = wait_terminal(&daemon, profile).await;
    assert_eq!(info.state, JobState::Done, "{outcome:?}");
    let leagues = daemon
        .submit("leagues".into(), json!({}), 0, "test".into(), account)
        .unwrap();
    let (info, outcome) = wait_terminal(&daemon, leagues).await;
    assert_eq!(info.state, JobState::Done, "{outcome:?}");

    let sends: Vec<(String, String)> = daemon
        .choke
        .recent_sends()
        .into_iter()
        .map(|send| (url_path(&send.url), send.method))
        .collect();
    assert!(
        !sends.iter().any(|(_, method)| method == "HEAD"),
        "no-probe routes must not be probed: {sends:?}"
    );
    assert!(
        sends.contains(&("/profile".into(), "GET".into())),
        "{sends:?}"
    );
    assert!(
        sends.contains(&("/account/leagues".into(), "GET".into())),
        "{sends:?}"
    );
    assert_eq!(
        daemon.choke.endpoint_state("profile@Alice#1234"),
        EndpointState::Policyless
    );
    assert!(matches!(
        daemon.choke.endpoint_state("league@Alice#1234"),
        EndpointState::Policy(ref name) if name.starts_with("league-request-limit")
    ));
    finish_harness_mock(dispatcher, &log_path);
}

#[tokio::test]
async fn dispatcher_retries_429_429_success_exactly_three_times_and_completes_once() {
    let responses = vec![
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse::full("GET", "429 Too Many Requests", Some(0), "{}"),
        ScriptedResponse::full("GET", "429 Too Many Requests", Some(5), "{}"),
        ScriptedResponse::full("GET", "200 OK", None, r#"{"items":["done"]}"#),
    ];
    let (base, requests, server) = scripted_server(responses).await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon(&base, clock);
    let mut events = daemon.events.subscribe();
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();

    let (info, outcome) = wait_terminal(&daemon, id).await;
    assert_eq!(info.state, JobState::Done);
    assert_eq!(info.retries, MAX_429_RETRIES);
    assert_eq!(fetch_payload_marker(&outcome), "done");
    assert_eq!(
        requests.lock().unwrap().as_slice(),
        ["HEAD", "GET", "GET", "GET"]
    );
    assert_eq!(
        journal_waits(&log_path),
        [0, 0, full_hold_ms(), 5_000 + full_hold_ms()],
        "each retry waits its Retry-After plus the bucket pad"
    );
    assert_eq!(terminal_event_count(&mut events, id), 1);
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
}

#[tokio::test]
async fn dispatcher_exhausts_on_third_429_without_a_fourth_send_or_final_sleep() {
    let responses = vec![
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse::full("GET", "429 Too Many Requests", Some(0), "{}"),
        ScriptedResponse::full("GET", "429 Too Many Requests", Some(0), "{}"),
        ScriptedResponse::full("GET", "429 Too Many Requests", Some(0), "{}"),
    ];
    let (base, requests, server) = scripted_server(responses).await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon(&base, clock.clone());
    let started = clock.now();
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();

    let (info, outcome) = wait_terminal(&daemon, id).await;
    assert_eq!(info.state, JobState::Failed);
    let Outcome::Failure { error, .. } = outcome else {
        panic!("exhausted job did not fail")
    };
    assert!(error.contains("giving up"));
    assert!(
        clock.now() - started < Duration::from_millis(3 * full_hold_ms()),
        "the third 429's hold is not slept out before the job fails: {:?}",
        clock.now() - started
    );
    assert_eq!(requests.lock().unwrap().len(), 4, "HEAD plus three GETs");
    assert_eq!(
        journal_waits(&log_path),
        [0, 0, full_hold_ms(), full_hold_ms()],
        "only the two retryable attempts wait, each behind the full hold"
    );
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
}

/// C19: a landed GET 429 holds its route for its `Retry-After`, clamped
/// to the cap, plus the retry bucket, whatever the header carried. Such a
/// 429 is not retried, so a second job's send shows the hold: 61 s after
/// a missing or garbled value, 961 s after an over-cap one.
#[tokio::test]
async fn a_get_429_without_a_usable_retry_after_holds_the_next_send() {
    let missing = || ScriptedResponse::full("GET", "429 Too Many Requests", None, "{}");
    let cases = [
        ("missing", missing(), full_hold_ms()),
        (
            "garbled",
            missing().with_raw_retry_after("soon"),
            full_hold_ms(),
        ),
        (
            "over the cap",
            missing().with_raw_retry_after("5000"),
            crate::ratelimit::RETRY_AFTER_CAP_SECS * 1000 + full_hold_ms(),
        ),
    ];
    for (name, rejected, hold) in cases {
        let responses = vec![
            ScriptedResponse::full("HEAD", "204 No Content", None, ""),
            rejected,
            ScriptedResponse::full("GET", "200 OK", None, r#"{"items":["after"]}"#),
        ];
        let (base, requests, server) = scripted_server(responses).await;
        let clock = Arc::new(ManualClock::new());
        let (daemon, log_path) = test_daemon(&base, clock);
        let dispatcher = tokio::spawn(daemon.clone().dispatcher());
        let first = daemon
            .submit("fetch".into(), json!({}), 0, "test".into(), None)
            .unwrap();
        let (info, _) = wait_terminal(&daemon, first).await;
        assert_eq!(info.state, JobState::Failed, "{name}");
        assert_eq!(info.retries, 0, "{name}");
        let second = daemon
            .submit("fetch".into(), json!({}), 0, "test".into(), None)
            .unwrap();
        let (_, outcome) = wait_terminal(&daemon, second).await;
        assert_eq!(fetch_payload_marker(&outcome), "after", "{name}");
        assert_eq!(
            requests.lock().unwrap().as_slice(),
            ["HEAD", "GET", "GET"],
            "{name}"
        );
        assert_eq!(
            journal_waits(&log_path),
            [0, 0, hold],
            "{name}: the next send waits the clamped Retry-After plus the bucket"
        );
        server.await.unwrap();
        finish_harness_wire(dispatcher, &log_path, &requests);
    }
}

#[tokio::test]
async fn dispatcher_requeue_preserves_fifo_job_identity() {
    let responses = vec![
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse::full("GET", "429 Too Many Requests", Some(0), "{}"),
        ScriptedResponse::full("GET", "200 OK", None, r#"{"items":["first"]}"#),
        ScriptedResponse::full("GET", "200 OK", None, r#"{"items":["second"]}"#),
    ];
    let (base, requests, server) = scripted_server(responses).await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon(&base, clock);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let first = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let second = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();

    let (_, first_outcome) = wait_terminal(&daemon, first).await;
    let (_, second_outcome) = wait_terminal(&daemon, second).await;
    assert_eq!(fetch_payload_marker(&first_outcome), "first");
    assert_eq!(fetch_payload_marker(&second_outcome), "second");
    assert_eq!(requests.lock().unwrap().len(), 4);
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
}

#[tokio::test]
async fn dispatcher_never_retries_403_or_503() {
    // The third case is rung 10's live 503 (2026-08-24): an origin page
    // with no rate headers. Named as such, still never retried.
    const ORIGIN_503: &str = "<html><head><title>503 Service Temporarily Unavailable</title></head><body><center><h1>503 Service Temporarily Unavailable</h1></center><hr><center>openresty</center></body></html>";
    for (status, body, shape) in [
        (
            "403 Forbidden",
            "{}",
            "unclassified body, possibly a Cloudflare block",
        ),
        (
            "503 Service Unavailable",
            "{}",
            "unclassified body, possibly a Cloudflare block",
        ),
        (
            "503 Service Unavailable",
            ORIGIN_503,
            "origin error page, not Cloudflare-shaped",
        ),
    ] {
        let responses = vec![
            ScriptedResponse::full("HEAD", "204 No Content", None, ""),
            ScriptedResponse::full("GET", status, None, body),
        ];
        let (base, requests, server) = scripted_server(responses).await;
        let clock = Arc::new(ManualClock::new());
        let (daemon, log_path) = test_daemon(&base, clock.clone());
        let dispatcher = tokio::spawn(daemon.clone().dispatcher());
        let id = daemon
            .submit("fetch".into(), json!({}), 0, "test".into(), None)
            .unwrap();

        let (info, outcome) = wait_terminal(&daemon, id).await;
        assert_eq!(info.state, JobState::Failed);
        assert_eq!(info.retries, 0);
        let Outcome::Failure { error, .. } = outcome else {
            panic!("{status} did not fail")
        };
        assert!(error.contains("NOT retrying"));
        assert!(error.contains(shape), "{error}");
        assert_eq!(requests.lock().unwrap().as_slice(), ["HEAD", "GET"]);
        assert_eq!(
            journal_waits(&log_path),
            [0, 0],
            "a block is never waited on"
        );
        server.await.unwrap();
        finish_harness_wire(dispatcher, &log_path, &requests);
    }
}

/// C129: a response Cloudflare gave in GGG's place is a stop. With the
/// tripwire off, the block fails its job at once (no re-queue, whatever
/// its `Retry-After`), admits no further send on any route (a second
/// account's job, whose session would need a refresh first, stays
/// waiting and nothing reaches the wire for it), and leaves a mark in the
/// world that a later daemon, also without the tripwire, honours. The
/// mark names the status, method, route and time, never the body. The
/// three shapes: Cloudflare's documented 1015 (its one marker past the
/// 300 characters a failure keeps), a challenge carrying `cf-mitigated`,
/// and a 429 with no header and no marker at all (N49).
#[tokio::test]
async fn a_cloudflare_block_admits_nothing_and_survives_a_restart() {
    let markerless_429 = ScriptedResponse {
        method: "GET",
        path: "/fetch",
        status: "429 Too Many Requests",
        headers: String::new(),
        body: "Too many requests from this client".into(),
    };
    for (name, block, status, shape) in [
        (
            "1015",
            ScriptedResponse::cloudflare_1015_429(),
            429,
            "cloudflare",
        ),
        (
            "challenge",
            ScriptedResponse::cloudflare_403_challenge(),
            403,
            "unclassified",
        ),
        ("markerless 429", markerless_429, 429, "unclassified"),
    ] {
        let body = block.body.clone();
        let responses = vec![
            ScriptedResponse::full("HEAD", "204 No Content", None, ""),
            block,
        ];
        let (base, requests, server) = scripted_server(responses).await;
        let clock = Arc::new(ManualClock::new());
        let state_path = std::env::temp_dir().join(format!(
            "acq-c129-rails-{}-{}.json",
            std::process::id(),
            TEST_LOG_ID.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = std::fs::remove_file(&state_path);
        let (mut daemon, log_path) = test_daemon_with(
            Provider::mock(&base),
            clock,
            RailsConfig {
                state_path: Some(state_path.clone()),
                ..RailsConfig::default()
            },
        );
        assert!(!daemon.choke.rails().config().tripwire);
        logged_in(&mut daemon);
        let dispatcher = tokio::spawn(daemon.clone().dispatcher());
        let first = daemon
            .submit("fetch".into(), json!({}), 0, "test".into(), None)
            .unwrap();
        let (info, outcome) = wait_terminal(&daemon, first).await;
        assert_eq!(info.state, JobState::Failed, "{name}");
        assert_eq!(info.retries, 0, "{name}: a block is never re-queued");
        let Outcome::Failure { error, .. } = outcome else {
            panic!("{name}: the block did not fail its job")
        };
        assert!(
            error.contains("Cloudflare answered in GGG's place"),
            "{name}: {error}"
        );

        // Submitted only after the block landed: the gate is two wide, and
        // a sibling submitted earlier could be admitted beside it.
        let second = daemon
            .submit(
                "stashes".into(),
                json!({}),
                0,
                "test".into(),
                Some("test-user".into()),
            )
            .unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;
        let state = daemon.shared.lock().unwrap().jobs[&second].info.state;
        assert_eq!(
            state,
            JobState::Waiting,
            "{name}: the sibling stays waiting"
        );
        assert!(
            daemon.choke.rails().halted().is_some(),
            "{name}: this daemon is halted"
        );
        // A blocked 429 still holds its route for its `Retry-After` (C19),
        // as a blocked HEAD's does (`a_blocked_probe_is_no_probe_failure`).
        if name == "1015" {
            let held = daemon.choke.check("fetch").err();
            assert!(
                held.is_some_and(|wait| wait >= Duration::from_secs(30)),
                "{name}: {held:?}"
            );
        }
        assert_eq!(
            requests.lock().unwrap().as_slice(),
            ["HEAD", "GET"],
            "{name}"
        );
        server.await.unwrap();
        let sends = journal_sends(&log_path);
        assert_eq!(sends[1]["status"], status, "{name}");
        assert_eq!(sends[1]["shape"], shape, "{name}: {}", sends[1]);

        // A later daemon, the tripwire still off, honours the mark.
        let rebuilt = Rails::with_config(RailsConfig {
            state_path: Some(state_path.clone()),
            ..RailsConfig::default()
        });
        let cause = rebuilt.halted().expect("the mark survives the daemon");
        assert!(
            cause.contains(&format!(
                "Cloudflare answered in GGG's place: {status} on GET fetch at 2000-01-01T"
            )),
            "{name}: {cause}"
        );
        assert!(cause.contains("reset-tripwire"), "{name}: {cause}");
        let persisted = std::fs::read_to_string(&state_path).unwrap();
        let fragments = [
            "Ray ID",
            "<html",
            "<!DOCTYPE",
            "Just a moment",
            "rate limited",
            "Too many requests",
        ];
        for evidence in [&cause, &persisted] {
            for fragment in fragments.iter().filter(|fragment| body.contains(*fragment)) {
                assert!(
                    !evidence.contains(fragment),
                    "{name}: the mark kept the body's {fragment:?}: {evidence}"
                );
            }
            for header in ["challenge", "Retry-After"] {
                assert!(!evidence.contains(header), "{name}: {evidence}");
            }
        }
        drop(rebuilt);
        finish_harness_wire(dispatcher, &log_path, &requests);
        let _ = std::fs::remove_file(&state_path);
    }
}

/// C129's false positives: GGG's own 4xx is no block. A JSON 404 with
/// GGG's headers (its message saying "array id", which holds "ray id"
/// inside a word), a 403 that names its own auth error in
/// `WWW-Authenticate` and carries no `cf-mitigated` (its body mentioning
/// Cloudflare), and GGG's JSON echoing a name the user chose that holds a
/// page marker ("CloudflareFan", a league "Error 1015 SSF") beside GGG's
/// policy header, each fail their job and leave the daemon sending: the
/// next job's GET goes out.
#[tokio::test]
async fn ggg_4xx_answers_are_not_cloudflare_blocks() {
    let auth_error = {
        let mut response = ScriptedResponse::full(
            "GET",
            "403 Forbidden",
            None,
            r#"{"error":"insufficient_scope","error_description":"see the Cloudflare-fronted docs"}"#,
        );
        response.headers.push_str(
            "WWW-Authenticate: Bearer realm=\"pathofexile:production\", error=\"insufficient_scope\"\r\n",
        );
        response
    };
    for (name, rejected) in [
        (
            "GGG's 404",
            ScriptedResponse::full(
                "GET",
                "404 Not Found",
                None,
                r#"{"error":{"code":1,"message":"Resource not found: unknown array id"}}"#,
            ),
        ),
        ("an auth error", auth_error),
        (
            "GGG's 404 echoing CloudflareFan",
            ScriptedResponse::full(
                "GET",
                "404 Not Found",
                None,
                r#"{"error":{"code":1,"message":"Character not found: CloudflareFan"}}"#,
            ),
        ),
        (
            "GGG's 400 echoing a league",
            ScriptedResponse::full(
                "GET",
                "400 Bad Request",
                None,
                r#"{"error":{"code":2,"message":"Invalid league: Error 1015 SSF"}}"#,
            ),
        ),
    ] {
        let responses = vec![
            ScriptedResponse::full("HEAD", "204 No Content", None, ""),
            rejected,
            ScriptedResponse::full("GET", "200 OK", None, r#"{"items":["after"]}"#),
        ];
        let (base, requests, server) = scripted_server(responses).await;
        let clock = Arc::new(ManualClock::new());
        let (daemon, log_path) = test_daemon(&base, clock);
        let dispatcher = tokio::spawn(daemon.clone().dispatcher());
        let first = daemon
            .submit("fetch".into(), json!({}), 0, "test".into(), None)
            .unwrap();
        let (info, outcome) = wait_terminal(&daemon, first).await;
        assert_eq!(info.state, JobState::Failed, "{name}");
        let Outcome::Failure { error, .. } = outcome else {
            panic!("{name}: did not fail")
        };
        assert!(!error.contains("Cloudflare answered"), "{name}: {error}");
        assert_eq!(daemon.choke.rails().halted(), None, "{name}");
        let second = daemon
            .submit("fetch".into(), json!({}), 0, "test".into(), None)
            .unwrap();
        let (_, outcome) = wait_terminal(&daemon, second).await;
        assert_eq!(fetch_payload_marker(&outcome), "after", "{name}");
        assert_eq!(
            requests.lock().unwrap().as_slice(),
            ["HEAD", "GET", "GET"],
            "{name}"
        );
        server.await.unwrap();
        finish_harness_wire(dispatcher, &log_path, &requests);
    }
}

/// C130: a GET's 302 fails its job with the status and the redirect's
/// target shown only redacted. The body, which echoes the target with its
/// query, is not kept: the sentinel reaches neither the job's failure,
/// the journal, the daemon log nor its recent errors.
#[tokio::test]
async fn a_get_3xx_fails_its_job_with_the_target_redacted_and_no_body() {
    const SENTINEL: &str = "c130-get-sentinel";
    let target = format!("https://www.pathofexile.com/login?code={SENTINEL}");
    let mut redirect = ScriptedResponse::full(
        "GET",
        "302 Found",
        None,
        &format!(r#"<html><a href="{target}">Found</a></html>"#),
    );
    redirect
        .headers
        .push_str(&format!("Location: {target}\r\n"));
    let responses = vec![
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        redirect,
    ];
    let (base, requests, server) = scripted_server(responses).await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon(&base, clock);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();

    let (info, outcome) = wait_terminal(&daemon, id).await;
    assert_eq!(info.state, JobState::Failed);
    assert_eq!(info.retries, 0);
    let Outcome::Failure { error, .. } = outcome else {
        panic!("a 302 did not fail its job")
    };
    assert!(error.contains("302 Found"), "{error}");
    assert!(!error.contains(SENTINEL), "the failure kept it: {error}");
    assert!(
        error.contains("https://www.pathofexile.com/login?…"),
        "the failure names where the 302 pointed: {error}"
    );
    let journal = std::fs::read_to_string(journal_of(&log_path)).unwrap();
    assert!(
        !journal.contains(SENTINEL),
        "the journal kept it: {journal}"
    );
    let log = std::fs::read_to_string(&log_path).unwrap();
    assert!(!log.contains(SENTINEL), "the daemon log kept it: {log}");
    let errors = format!("{:?}", daemon.shared.lock().unwrap().errors);
    assert!(
        !errors.contains(SENTINEL),
        "recent errors kept it: {errors}"
    );
    assert_eq!(requests.lock().unwrap().as_slice(), ["HEAD", "GET"]);
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
}

#[tokio::test]
async fn full_acceptable_head_429_establishes_under_hold_without_job_retry() {
    let responses = vec![
        ScriptedResponse::full("HEAD", "429 Too Many Requests", Some(0), ""),
        ScriptedResponse::full("GET", "200 OK", None, r#"{"items":["after-probe"]}"#),
    ];
    let (base, requests, server) = scripted_server(responses).await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon(&base, clock.clone());
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();

    let (info, outcome) = wait_terminal(&daemon, id).await;
    assert_eq!(info.state, JobState::Done);
    assert_eq!(info.retries, 0);
    assert_eq!(fetch_payload_marker(&outcome), "after-probe");
    let probe = daemon
        .shared
        .lock()
        .unwrap()
        .jobs
        .values()
        .find(|entry| entry.info.kind == "probe")
        .unwrap()
        .info
        .clone();
    assert_eq!(probe.state, JobState::Done);
    assert_eq!(probe.retries, 0);
    assert_eq!(requests.lock().unwrap().as_slice(), ["HEAD", "GET"]);
    assert_eq!(
        journal_waits(&log_path),
        [0, full_hold_ms()],
        "the GET waits behind the probe's 429 hold"
    );
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
}

#[tokio::test]
async fn malformed_or_unacceptable_head_429_fails_under_cooldown() {
    for head in [
        ScriptedResponse::malformed_head_429(),
        ScriptedResponse::full("HEAD", "429 Too Many Requests", None, ""),
    ] {
        let (base, requests, server) = scripted_server(vec![head]).await;
        let clock = Arc::new(ManualClock::new());
        let (daemon, log_path) = test_daemon(&base, clock);
        let dispatcher = tokio::spawn(daemon.clone().dispatcher());
        let id = daemon
            .submit("fetch".into(), json!({}), 0, "test".into(), None)
            .unwrap();

        let (info, _) = wait_terminal(&daemon, id).await;
        assert_eq!(info.state, JobState::Failed);
        assert_eq!(info.retries, 0);
        assert!(matches!(
            daemon.choke.endpoint_state("fetch"),
            EndpointState::Degraded { .. }
        ));
        assert_eq!(requests.lock().unwrap().as_slice(), ["HEAD"]);
        let probe = daemon
            .shared
            .lock()
            .unwrap()
            .jobs
            .values()
            .find(|entry| entry.info.kind == "probe")
            .unwrap()
            .info
            .clone();
        assert_eq!(probe.state, JobState::Failed);
        assert_eq!(probe.retries, 0);
        server.await.unwrap();
        finish_harness_wire(dispatcher, &log_path, &requests);
    }
}

#[tokio::test]
async fn whoami_is_refused_in_real_mode_without_a_token_request() {
    // Real in name only: every URL is the local token server's, so
    // nothing here can reach GGG. The session holds a refresh token
    // and no access token — a refresh is due, and without the refusal
    // whoami's first act is the token POST this server would answer.
    let (base, requests, server) = token_server_answering(vec!["200 OK"]).await;
    let provider = Provider {
        name: acquisition_protocol::provider::GGG,
        ..Provider::mock(&base)
    };
    assert!(provider.is_real());
    let clock = Arc::new(ManualClock::new());
    let (mut daemon, log_path) = test_daemon_with(provider, clock, RailsConfig::default());
    logged_in(&mut daemon);
    let outcome = daemon
        .execute_inner(1, "whoami", json!({}), None, None, daemon.choke.now())
        .await
        .unwrap();
    assert!(
        requests.lock().unwrap().is_empty(),
        "whoami in real mode cost a token request: {outcome:?}"
    );
    let Outcome::Failure { error, .. } = outcome else {
        panic!("whoami ran in real mode")
    };
    assert!(error.contains("mock-only"), "{error}");
    assert!(daemon.choke.recent_sends().is_empty());
    server.abort();
    assert_journal_matches_wire(&log_path, &wire_posts(&requests));
    assert!(journal_sends(&log_path).is_empty());
    finish_harness(None, &log_path);
}

/// C129 on HEAD: a probe Cloudflare answers is no probe failure. Two
/// shapes, each with no GGG header (a HEAD has no body, so no page marker
/// counts): the 1015's 429 with `Retry-After: 30` (N48), and a 403
/// carrying `cf-mitigated`. Either sets the block mark; the route learns
/// no policy and is not degraded; the job that asked for the probe stays
/// waiting under the halt rather than failing as "route degraded"; and no
/// second HEAD follows. The 1015 still holds the route for its
/// `Retry-After` (C19), as a blocked GET's 429 does. Once
/// `reset-tripwire` clears the mark, the route is probed afresh, behind
/// that hold for the 1015 and at once for the challenge, and the job
/// goes out.
#[tokio::test]
async fn a_blocked_probe_is_no_probe_failure() {
    let head_1015 = ScriptedResponse {
        method: "HEAD",
        path: "/fetch",
        status: "429 Too Many Requests",
        headers: "Retry-After: 30\r\n".into(),
        body: String::new(),
    };
    let head_challenge = ScriptedResponse {
        method: "HEAD",
        body: String::new(),
        ..ScriptedResponse::cloudflare_403_challenge()
    };
    for (name, head, status) in [("1015", head_1015, 429), ("challenge", head_challenge, 403)] {
        let (base, requests, server) = scripted_server(vec![
            head,
            ScriptedResponse::full("HEAD", "204 No Content", None, ""),
            ScriptedResponse::full("GET", "200 OK", None, r#"{"items":["after"]}"#),
        ])
        .await;
        let clock = Arc::new(ManualClock::new());
        let (daemon, log_path) = test_daemon(&base, clock);
        let dispatcher = tokio::spawn(daemon.clone().dispatcher());
        let id = daemon
            .submit("fetch".into(), json!({}), 0, "test".into(), None)
            .unwrap();
        let probe = tokio::time::timeout(Duration::from_secs(3), async {
            loop {
                let done = daemon
                    .shared
                    .lock()
                    .unwrap()
                    .jobs
                    .values()
                    .find(|entry| entry.info.kind == "probe" && entry.info.state.is_terminal())
                    .map(|entry| entry.info.id);
                if let Some(probe) = done {
                    return probe;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("the probe concluded");
        // Long enough for a degraded route to fail its job, or a re-probe.
        tokio::time::sleep(Duration::from_millis(100)).await;

        let halt = daemon
            .choke
            .rails()
            .halted()
            .expect("the block mark is set");
        assert!(
            halt.contains(&format!(
                "Cloudflare answered in GGG's place: {status} on HEAD fetch"
            )),
            "{name}: {halt}"
        );
        assert_eq!(
            daemon.choke.endpoint_state("fetch"),
            EndpointState::Unknown,
            "{name}: a block teaches the route nothing"
        );
        let state = daemon.shared.lock().unwrap().jobs[&id].info.state;
        assert_eq!(
            state,
            JobState::Waiting,
            "{name}: the job waits on the halt"
        );
        assert_ne!(probe, id);
        assert_eq!(requests.lock().unwrap().as_slice(), ["HEAD"], "{name}");

        let held = daemon.choke.check("fetch").err();
        if status == 429 {
            assert!(
                held.is_some_and(|wait| wait >= Duration::from_secs(30)),
                "{name}: the route is held for Cloudflare's Retry-After: {held:?}"
            );
        } else {
            assert_eq!(held, None, "{name}: a 403 installs no hold");
        }

        // The manual clock advances through the probe's own wait.
        daemon.reset_rails();
        let (info, _) = wait_terminal(&daemon, id).await;
        assert_eq!(info.state, JobState::Done, "{name}");
        assert_eq!(
            requests.lock().unwrap().as_slice(),
            ["HEAD", "HEAD", "GET"],
            "{name}"
        );
        let hold = if status == 429 {
            30_000 + full_hold_ms()
        } else {
            0
        };
        assert_eq!(
            journal_waits(&log_path),
            [0, hold, 0],
            "{name}: the fresh probe waits out the hold, and only it"
        );
        server.await.unwrap();
        finish_harness_wire(dispatcher, &log_path, &requests);
    }
}

/// The narrow HEAD rule (C129, the owner's choice): a HEAD 403 with no
/// `cf-mitigated` is no block, even with no GGG header (`/profile`'s HEAD
/// is answered so, N38). It degrades the route as before, its job fails
/// as "route degraded", and nothing is halted.
#[tokio::test]
async fn a_head_403_without_cf_mitigated_still_degrades() {
    let head_403 = ScriptedResponse {
        method: "HEAD",
        path: "/fetch",
        status: "403 Forbidden",
        headers: String::new(),
        body: String::new(),
    };
    let (base, requests, server) = scripted_server(vec![head_403]).await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon(&base, clock);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let (info, outcome) = wait_terminal(&daemon, id).await;
    assert_eq!(info.state, JobState::Failed);
    let Outcome::Failure { error, .. } = outcome else {
        panic!("the job did not fail")
    };
    assert!(error.contains("degraded"), "{error}");
    assert!(matches!(
        daemon.choke.endpoint_state("fetch"),
        EndpointState::Degraded { .. }
    ));
    assert_eq!(daemon.choke.rails().halted(), None);
    assert_eq!(requests.lock().unwrap().as_slice(), ["HEAD"]);
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
}
