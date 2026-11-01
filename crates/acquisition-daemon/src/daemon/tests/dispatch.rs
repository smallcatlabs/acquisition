//! The dispatcher on the wire. The scripted-server families (the 429
//! family, priority on a scheduling key, the headerless 503, Cloudflare's
//! blocks and 5xx, GGG's own 4xx and 5xx, a GET's 3xx, the HEAD 403) and
//! the wire twins of `responses.rs`'s classification table are rows of
//! the wire table through the door (C158, a scenario crosses the daemon's
//! frontend boundary only), at the foot. Through the door too: progress
//! beside an auth wait, malformed listings (the account's store read as
//! a frontend reads it) and declared route knowledge. The rest are
//! component tests, each saying the boundary it needs: progress beside
//! rate waits, a probe's 429 under a private login, a blocked probe (the
//! pick), the
//! mark on disk and the log's redaction, real mode's whoami, the cancels
//! at the choke point and the store that cannot record.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::door::*;
use super::harness::*;
use crate::daemon::*;
use crate::ratelimit::Clock;
use std::future::Future;
use std::pin::Pin;

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

/// `wait_ms` of every send, in journal order.
fn journal_waits(log_path: &std::path::Path) -> Vec<u64> {
    journal_sends(log_path)
        .iter()
        .map(|l| l["wait_ms"].as_u64().expect("wait_ms"))
        .collect()
}

/// C21 (an auth wait occupies no dispatcher slot): two jobs held behind
/// the one token request the server has not answered are both Running,
/// and an independent job is picked and completes beside them; released,
/// both end Done on the one refresh. Through the door: the delayed token
/// server holds the refresh of `old-user`'s `rt-old` (a refresh-due
/// session of its own name, no keyring word), the jobs a client's
/// `whoami`s; once the refresh has arrived and the runtime is quiet, both
/// read Running through `Status`, a zero-second sleep runs to its end,
/// and after the release the journal holds the one POST.
#[test]
fn dispatcher_auth_waits_do_not_cap_independent_job_progress() {
    let mut sessions = Sessions::with(AuthSession {
        refresh_token: Some("rt-old".into()),
        ..AuthSession::default()
    });
    sessions.rename("", "old-user");
    Door::open(
        Server::Token(Token::Delayed(
            "200 OK",
            token_body("at-new", "rt-rotated", "test-user"),
        )),
        Arc::new(ParkingClock::new()),
        Arc::new(NoopCredentialStore),
    )
    .sessions(sessions)
    .run(|mut door| async move {
        let (arrived, release) = door.held();
        let first = door.submit("whoami", None).await;
        let second = door.submit("whoami", None).await;
        arrived.await.expect("the refresh reached the token server");
        door.quiet().await;
        for id in [first, second] {
            door.await_job(id, |job| job.state == JobState::Running)
                .await;
        }
        let sleep = Request::Submit {
            kind: "sleep".into(),
            params: json!({ "seconds": 0.0 }),
            priority: 0,
            submitted_by: "test".into(),
            account: None,
        };
        let independent = match door.request(sleep).await {
            Response::Submitted { id } => id,
            other => panic!("the sleep answered {other:?}"),
        };
        let (info, _) = door.await_terminal(independent).await;
        assert_eq!(info.state, JobState::Done);

        release
            .send(())
            .expect("the token server waits for the release");
        for id in [first, second] {
            let (info, outcome) = door.await_terminal(id).await;
            assert_eq!(info.state, JobState::Done, "job {id}: {outcome:?}");
        }
        assert_eq!(door.journal_methods(), ["POST"], "the one refresh");
        door.finish().await;
    });
}

/// Two rate waits on two different keys stay parked on the clock while
/// an independent job is picked and completes: a wait holds its own key,
/// never the dispatcher. Then the clock moves, deadline by deadline,
/// until both waits end.
#[tokio::test]
async fn dispatcher_rate_waits_do_not_cap_independent_job_progress() {
    let (base, server) = saturated_probe_server().await;
    let clock = Arc::new(ParkingClock::new());
    let (daemon, log_path) = test_daemon(&base, clock.clone());
    daemon
        .choke
        .head(
            "character-list",
            &format!("{base}/character"),
            None,
            daemon.choke.now(),
            ALWAYS_WANTED,
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
            ALWAYS_WANTED,
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
    tokio::time::timeout(HANG, async {
        while clock.sleepers() < 2 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("the two jobs reached their rate waits");

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
    assert_eq!(clock.sleepers(), 2, "the two rate waits stayed parked");

    // The dispatcher pre-waits in one-second cancellation slices: each
    // deadline in turn, until both jobs end.
    let terminal = |id| state_of(&daemon, id).is_terminal();
    tokio::time::timeout(HANG, async {
        while !(terminal(characters) && terminal(stashes)) {
            clock.advance_to_next();
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("both rate waits ended");
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

/// Malformed listings fail the *job*, not just the store: a 2xx with
/// no `stashes` array is refused by the job's own guard, and a listing
/// whose entries lack ids is refused by the store (`MalformedBody`)
/// and propagated through `record` — neither reports success, and
/// neither leaves a response row a snapshot could cite as a basis.
/// Through the door: a script of the probe and the two listings, a live
/// session for Alice, two client submits; the account's store read
/// through the store's read API, as a frontend reads it while the daemon
/// runs (C12), the journal held equal to the script's wire at the finish.
#[test]
fn malformed_listings_fail_the_job_not_just_the_store() {
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
    let clock = Arc::new(ParkingClock::new());
    let sessions = live_sessions(&clock);
    let store_dir = TestPath::dir();
    let dir = store_dir.to_path_buf();
    Door::open(
        Server::Script(responses),
        clock,
        Arc::new(RecordingCredentialStore::default()),
    )
    .store_dir(dir.clone())
    .sessions(sessions)
    .run(|mut door| async move {
        let first = door.submit("stashes", Some("Alice#1234")).await;
        let (info, outcome) = door.await_terminal(first).await;
        assert_eq!(info.state, JobState::Failed);
        let Outcome::Failure { error, .. } = outcome else {
            panic!("missing array did not fail the job")
        };
        assert!(error.contains("stashes"), "{error}");
        let second = door.submit("stashes", Some("Alice#1234")).await;
        let (info, outcome) = door.await_terminal(second).await;
        assert_eq!(info.state, JobState::Failed);
        let Outcome::Failure { error, .. } = outcome else {
            panic!("id-less entry did not fail the job")
        };
        assert!(error.contains("malformed stashes response"), "{error}");
        // Nothing recorded either way: no tabs retired, no false basis.
        let store = Store::open(&account_path(&dir, "Alice#1234")).unwrap();
        assert_eq!(store.status().unwrap().responses, 0);
        drop(store);
        door.finish().await;
    });
}

/// The route knowledge declared for `/profile` and `/account/leagues`
/// (first contact 2026-08-30), held against the mock that mirrors GGG
/// (C32): neither is probed; `/profile`'s headerless 200 is accepted and
/// the endpoint becomes Policyless; `/account/leagues` teaches its policy
/// from the GET. Removing a declaration fails here, not live. Through
/// the door: a live session for Alice, her `profile` and `leagues`
/// submitted as a client submits them; the sends, the policyless
/// endpoints and the learned policies read from `Dashboard`, and the
/// journal.
#[test]
fn declared_route_knowledge_holds_against_the_mock() {
    let clock = Arc::new(ParkingClock::new());
    let sessions = live_sessions(&clock);
    Door::open(
        Server::Mock,
        clock,
        Arc::new(RecordingCredentialStore::default()),
    )
    .sessions(sessions)
    .run(|mut door| async move {
        for kind in ["profile", "leagues"] {
            let id = door.submit(kind, Some("Alice#1234")).await;
            let (info, outcome) = door.await_terminal(id).await;
            assert_eq!(info.state, JobState::Done, "{kind}: {outcome:?}");
        }
        let Response::Dashboard {
            sends,
            policyless_endpoints,
            policies,
            ..
        } = door.request(Request::Dashboard).await
        else {
            panic!("the dashboard did not answer")
        };
        let sends: Vec<(String, String)> = sends
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
        assert!(
            policyless_endpoints.contains(&"profile@Alice#1234".to_string()),
            "{policyless_endpoints:?}"
        );
        assert!(
            policies.iter().any(|status| {
                status.policy.starts_with("league-request-limit")
                    && status.endpoints.contains(&"league@Alice#1234".to_string())
            }),
            "{policies:?}"
        );
        let journal: Vec<(String, String)> = door
            .journal()
            .iter()
            .map(|send| {
                (
                    send["method"].as_str().expect("a method").to_string(),
                    send["path"].as_str().expect("a path").to_string(),
                )
            })
            .collect();
        assert_eq!(
            journal,
            [
                ("GET".to_string(), "/profile".to_string()),
                ("GET".to_string(), "/account/leagues".to_string()),
            ],
            "each route's first send is its GET"
        );
        door.finish().await;
    });
}

/// C19 for a probe: a HEAD's 429 that fails setup still holds its route
/// for its clamped `Retry-After` plus the retry bucket. A login clears the
/// setup failure's cooldown at once (`forget_degraded`, as `login` does),
/// so the next job's probe is held by the hold alone. A component test
/// (C158): the private `forget_degraded` stands in for that login.
#[tokio::test]
async fn a_head_429_that_fails_setup_holds_the_next_probe() {
    let full_missing = || ScriptedResponse::full("HEAD", "429 Too Many Requests", None, "");
    let cases = [
        ("Full, missing", full_missing(), full_hold_ms()),
        (
            "Full, over the cap",
            full_missing().with_raw_retry_after("901"),
            crate::ratelimit::RETRY_AFTER_CAP_SECS * 1000 + full_hold_ms(),
        ),
        (
            "non-Full",
            ScriptedResponse::malformed_head_429(),
            full_hold_ms(),
        ),
    ];
    for (name, rejected, hold) in cases {
        let responses = vec![
            rejected,
            ScriptedResponse::full("HEAD", "204 No Content", None, ""),
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
        daemon.choke.forget_degraded();
        let second = daemon
            .submit("fetch".into(), json!({}), 0, "test".into(), None)
            .unwrap();
        let (_, outcome) = wait_terminal(&daemon, second).await;
        assert_eq!(fetch_payload_marker(&outcome), "after", "{name}");
        assert_eq!(
            requests.lock().unwrap().as_slice(),
            ["HEAD", "HEAD", "GET"],
            "{name}"
        );
        assert_eq!(
            journal_waits(&log_path),
            [0, hold, 0],
            "{name}: the next probe waits the clamped Retry-After plus the bucket"
        );
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
/// second HEAD follows. The probe job itself waits too (C25: a failed
/// probe would fail every job waiting on its route), with no retry
/// spent, and the pick passes it over while halted. The 1015 still holds
/// the route for its `Retry-After` (C19), as a blocked GET's 429 does.
/// Once `reset-tripwire` clears the mark, the same probe goes out afresh,
/// behind that hold for the 1015 and at once for the challenge, and the
/// job goes out. A component test (C158): it reads the pick
/// (`pick_runnable`), the route's state (`endpoint_state`) and its hold
/// (`choke.check`), none of which crosses the protocol.
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
        // The block lands while the probe runs; the probe is then turned
        // away, back to waiting (C25), where it was a failure before.
        let probe = tokio::time::timeout(HANG, async {
            loop {
                let parked = daemon
                    .shared
                    .lock()
                    .unwrap()
                    .jobs
                    .values()
                    .find(|entry| {
                        entry.info.kind == "probe" && entry.info.state == JobState::Waiting
                    })
                    .map(|entry| entry.info.id);
                if let Some(probe) = parked
                    && daemon.choke.rails().halted().is_some()
                {
                    return probe;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("the probe was turned away");
        // Nothing more acts on the queue: with the dispatcher stopped and
        // no job task left running, only a pick could, and the pick passes
        // the turned-away probe over while halted. So no spin, no job
        // failed as "route degraded" and no re-probe, read below once the
        // queue is still. Then a fresh dispatcher takes the queue on.
        dispatcher.abort();
        tokio::time::timeout(HANG, async {
            while !daemon.shared.lock().unwrap().active_jobs.is_empty() {
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("no job task left running");
        assert!(
            daemon.pick_runnable().is_empty(),
            "{name}: the pick passes the probe over while halted"
        );

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
        let (state, retries) = {
            let s = daemon.shared.lock().unwrap();
            (s.jobs[&probe].info.state, s.jobs[&probe].info.retries)
        };
        assert_eq!(
            (state, retries),
            (JobState::Waiting, 0),
            "{name}: the probe waits on the halt"
        );
        let dispatcher = tokio::spawn(daemon.clone().dispatcher());
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
        daemon.reset_rails().unwrap();
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

/// C129, the mark on disk: a component case of the rails' state file,
/// which no frontend reads (a restart's report of the mark is the wire
/// table's second lifetime, the rows
/// `a_cloudflare_block_admits_nothing_and_survives_a_restart`). For each
/// block shape the file holds the mark, and none of the page's fragments
/// and neither header the response carried.
#[tokio::test]
async fn a_cloudflare_blocks_mark_on_disk_keeps_no_page_or_header() {
    for (name, block, _, _) in cloudflare_blocks() {
        let body = block.body.clone();
        let (base, requests, server) = scripted_server(vec![
            ScriptedResponse::full("HEAD", "204 No Content", None, ""),
            block,
        ])
        .await;
        let state = TestPath::dir();
        let state_path = state.join("c129-rails.json");
        let (daemon, log_path) = test_daemon_with(
            Provider::mock(&base),
            Arc::new(ManualClock::new()),
            RailsConfig {
                state_path: Some(state_path.clone()),
                ..RailsConfig::default()
            },
        );
        let dispatcher = tokio::spawn(daemon.clone().dispatcher());
        let id = daemon
            .submit("fetch".into(), json!({}), 0, "test".into(), None)
            .unwrap();
        assert_eq!(
            wait_terminal(&daemon, id).await.0.state,
            JobState::Failed,
            "{name}"
        );
        let persisted = std::fs::read_to_string(&state_path).unwrap();
        assert!(
            persisted.contains("Cloudflare answered in GGG's place"),
            "{name}: the mark is on disk: {persisted}"
        );
        for fragment in PAGE_FRAGMENTS
            .into_iter()
            .filter(|fragment| body.contains(fragment))
        {
            assert!(
                !persisted.contains(fragment),
                "{name}: the mark kept the body's {fragment:?}: {persisted}"
            );
        }
        for header in BLOCK_HEADERS {
            assert!(
                !persisted.contains(header),
                "{name}: the mark kept {header:?}: {persisted}"
            );
        }
        server.await.unwrap();
        finish_harness_wire(dispatcher, &log_path, &requests);
    }
}

/// C130, the daemon log: a component case of the log file, which no
/// frontend reads through the protocol (the job's failure, the journal
/// and `Dashboard.errors` are the wire table's row
/// `a_get_3xx_fails_its_job_with_the_target_redacted_and_no_body`). A
/// GET's 302 whose body and `Location` carry the sentinel leaves none of
/// it in the log.
#[tokio::test]
async fn a_get_3xx_keeps_its_target_out_of_the_daemon_log() {
    let (base, requests, server) = scripted_server(vec![
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        c130_redirect(),
    ])
    .await;
    let (daemon, log_path) = test_daemon(&base, Arc::new(ManualClock::new()));
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    assert_eq!(wait_terminal(&daemon, id).await.0.state, JobState::Failed);
    let log = std::fs::read_to_string(&log_path).unwrap();
    assert!(log.contains("302 Found"), "the log names the 302: {log}");
    assert!(
        !log.contains(C130_SENTINEL),
        "the daemon log kept it: {log}"
    );
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
}

// ---- C25: a cancel reaches a job parked at the choke point ----------------

/// The cancel family's door: the account server holding the request
/// `hold` names, the two accounts' sessions.
fn cancel_door(hold: (&'static str, &'static str, usize)) -> DoorBuilder {
    Door::open(
        Server::Account(hold),
        Arc::new(ParkingClock::new()),
        Arc::new(RecordingCredentialStore::default()),
    )
    .sessions(two_accounts())
}

/// `Request::Cancel` of job `id`, acknowledged.
async fn cancel(door: &mut Door, id: JobId) {
    match door.request(Request::Cancel { id }).await {
        Response::Ack { .. } => {}
        other => panic!("the cancel of job {id} answered {other:?}"),
    }
}

/// Job `id`'s outcome, its end awaited through the protocol.
async fn outcome_of(door: &mut Door, id: JobId) -> Outcome {
    door.await_terminal(id).await.1
}

/// C25: a cancel reaches a job parked Running in its refresh's wait on
/// the token route. `other` refreshes once, so the route's policy is
/// learned; the clock moves past the token that refresh installed, so
/// `other`'s second job refreshes again, and that POST is held on the
/// wire, holding the route's one permit; `test-user`'s job, already
/// running, owns its own refresh flight and waits behind it at the gate
/// (the runtime quiet under the held send). The job is cancelled and the
/// POST released: the job's refresh is turned away once the gate admits
/// it, nothing is sent for it, and it concludes cancelled. The next job
/// for the same account refreshes and goes out.
#[test]
fn a_cancel_reaches_a_job_waiting_on_the_token_route() {
    cancel_door(("POST", "/token", 1)).run(|mut door| async move {
        let (arrived, release) = door.held();
        let warm = door.submit("profile", Some("other")).await;
        assert!(matches!(
            outcome_of(&mut door, warm).await,
            Outcome::Success { .. }
        ));
        // Past the hour the account server's grant installs.
        door.advance(Duration::from_secs(3600));
        let first = door.submit("profile", Some("other")).await;
        tokio::time::timeout(HANG, arrived)
            .await
            .expect("other's second refresh reached the server")
            .unwrap();
        let parked = door.submit("profile", Some("test-user")).await;
        door.await_job(parked, |job| job.state == JobState::Running)
            .await;
        door.quiet().await;
        let warmed = [
            sent("other", "POST"),
            sent("other", "GET"),
            sent("other", "POST"),
        ];
        assert_eq!(door.sent(), warmed, "the parked refresh has sent nothing");
        cancel(&mut door, parked).await;
        release.send(()).unwrap();

        let outcome = outcome_of(&mut door, parked).await;
        assert!(matches!(outcome, Outcome::Cancelled), "{outcome:?}");
        assert!(matches!(
            outcome_of(&mut door, first).await,
            Outcome::Success { .. }
        ));
        let mut expected = warmed.to_vec();
        expected.push(sent("other", "GET"));
        assert_eq!(
            door.sent(),
            expected,
            "nothing was sent for the cancelled job"
        );
        let next = door.submit("profile", Some("test-user")).await;
        assert!(matches!(
            outcome_of(&mut door, next).await,
            Outcome::Success { .. }
        ));
        expected.extend([sent("test-user", "POST"), sent("test-user", "GET")]);
        assert_eq!(door.sent(), expected);
        assert_eq!(door.journal_methods(), wire_methods_of(&expected));
        door.finish().await;
    });
}

/// C25: a cancel reaches a job's own send at the choke point's admission.
/// The job is cancelled while its refresh's POST is on the wire; the POST,
/// already admitted, lands and its tokens are kept; the job's GET is
/// turned away before it is sent, and it concludes cancelled. The next
/// job goes out on the kept token, with no second refresh.
#[test]
fn a_cancel_during_the_refresh_turns_the_jobs_send_away() {
    cancel_door(("POST", "/token", 0)).run(|mut door| async move {
        let (arrived, release) = door.held();
        let id = door.submit("profile", Some("test-user")).await;
        tokio::time::timeout(HANG, arrived)
            .await
            .expect("the refresh reached the server")
            .unwrap();
        cancel(&mut door, id).await;
        release.send(()).unwrap();

        let outcome = outcome_of(&mut door, id).await;
        assert!(matches!(outcome, Outcome::Cancelled), "{outcome:?}");
        assert_eq!(
            door.sent(),
            [sent("test-user", "POST")],
            "the admitted refresh landed; the job's own GET was never sent"
        );
        let next = door.submit("profile", Some("test-user")).await;
        assert!(matches!(
            outcome_of(&mut door, next).await,
            Outcome::Success { .. }
        ));
        let wire = door.sent();
        assert_eq!(wire, [sent("test-user", "POST"), sent("test-user", "GET")]);
        assert_eq!(door.journal_methods(), wire_methods_of(&wire));
        door.finish().await;
    });
}

/// C25 stops at admission: a cancel that lands while the job's GET is on
/// the wire un-sends nothing. The GET lands and is journaled, and the job
/// concludes as a running job cancelled at its end always has: cancelled.
#[test]
fn a_cancel_after_admission_leaves_the_send_landed() {
    cancel_door(("GET", "/profile", 0)).run(|mut door| async move {
        let (arrived, release) = door.held();
        let id = door.submit("profile", Some("test-user")).await;
        tokio::time::timeout(HANG, arrived)
            .await
            .expect("the GET reached the server")
            .unwrap();
        cancel(&mut door, id).await;
        release.send(()).unwrap();

        let outcome = outcome_of(&mut door, id).await;
        assert!(matches!(outcome, Outcome::Cancelled), "{outcome:?}");
        let wire = door.sent();
        assert_eq!(wire, [sent("test-user", "POST"), sent("test-user", "GET")]);
        let journal = door.journal();
        assert_eq!(journal.len(), 2);
        assert_eq!(journal[1]["method"], "GET");
        assert_eq!(
            journal[1]["ok"], true,
            "the admitted GET landed: {journal:?}"
        );
        assert_eq!(door.journal_methods(), wire_methods_of(&wire));
        door.finish().await;
    });
}

/// C25 on a shared refresh: a cancel of one job never cancels the refresh
/// for another. `test-user`'s profile job owns the flight and its
/// `whoami` waits on it, both behind `other`'s POST on the token route
/// (each read joined by the runtime gone quiet under that held POST).
/// Cancelling the owner, the waiter, or both: one POST goes out while
/// either job still wants it (a cancelled owner's flight is called off and
/// the waiter asks again), the job still wanted goes out, the cancelled
/// one concludes cancelled with no send of its own, and when both are
/// cancelled the POST is never sent.
#[test]
fn a_cancel_of_one_job_on_a_shared_refresh_keeps_it_for_the_other() {
    for (cancel_owner, cancel_waiter) in [(true, false), (false, true), (true, true)] {
        let case = format!("owner cancelled {cancel_owner}, waiter cancelled {cancel_waiter}");
        cancel_door(("POST", "/token", 0)).run(move |mut door| async move {
            let (arrived, release) = door.held();
            let first = door.submit("profile", Some("other")).await;
            tokio::time::timeout(HANG, arrived)
                .await
                .expect("other's refresh reached the server")
                .unwrap();
            let owner = door.submit("profile", Some("test-user")).await;
            door.await_job(owner, |job| job.state == JobState::Running)
                .await;
            door.quiet().await;
            let waiter = door.submit("whoami", Some("test-user")).await;
            door.await_job(waiter, |job| job.state == JobState::Running)
                .await;
            door.quiet().await;
            if cancel_owner {
                cancel(&mut door, owner).await;
            }
            if cancel_waiter {
                cancel(&mut door, waiter).await;
            }
            release.send(()).unwrap();

            for (id, cancelled) in [(owner, cancel_owner), (waiter, cancel_waiter)] {
                let outcome = outcome_of(&mut door, id).await;
                if cancelled {
                    assert!(
                        matches!(outcome, Outcome::Cancelled),
                        "{case}: job {id}: {outcome:?}"
                    );
                } else {
                    assert!(
                        matches!(outcome, Outcome::Success { .. }),
                        "{case}: job {id}: {outcome:?}"
                    );
                }
            }
            outcome_of(&mut door, first).await;
            let wire = door.sent();
            let mut test_user: Vec<String> = wire
                .iter()
                .filter(|(_, account)| account == "test-user")
                .map(|(method, _)| method.clone())
                .collect();
            test_user.sort();
            let expected: &[&str] = match (cancel_owner, cancel_waiter) {
                (true, true) => &[],
                (true, false) => &["POST"],
                (false, _) => &["GET", "POST"],
            };
            assert_eq!(test_user, expected, "{case}");
            // `other`'s GET and `test-user`'s POST may cross on the wire,
            // so the journal is held to the wire as a count of each
            // method.
            let mut wire = wire_methods_of(&wire);
            let mut journal = door.journal_methods();
            wire.sort();
            journal.sort();
            assert_eq!(journal, wire, "{case}");
            door.finish().await;
        });
    }
}

// ---- C144: a store that cannot record fails the job, keeping the body ---

/// A facts file a newer daemon wrote: its schema version past this one's.
fn schema_too_new(dir: &std::path::Path, account: &str) {
    let conn = rusqlite::Connection::open(acquisition_store::account_path(dir, account)).unwrap();
    conn.pragma_update(None, "user_version", 9_999).unwrap();
}

fn kept_failure(outcome: &Outcome) -> (&str, &Value) {
    let Outcome::Failure {
        error,
        payload: Some(payload),
        ..
    } = outcome
    else {
        panic!("a failure keeping its body: {outcome:?}")
    };
    (error, payload)
}

/// C144 (the daemon reports no success it did not earn), issue #63: a
/// job whose response cannot be recorded after its send fails, where it
/// used to be logged and reported done — a facts file with a schema newer
/// than this daemon's, a store that refuses the write after it opened —
/// and the failure names the store's fault and keeps the params and the
/// fetched body as its payload, so the send is not wasted.
#[test]
fn c144_a_store_that_cannot_record_fails_the_job_and_keeps_the_body() {
    let (mut daemon, log_path) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
    let dir = TestPath::dir();
    Arc::get_mut(&mut daemon).unwrap().store_dir = Some(dir.to_path_buf());
    let params = json!({ "league": "Standard" });
    let body = json!({ "stashes": [ { "id": "t1", "name": "T", "type": "PremiumStash" } ] });

    // Unopenable: a schema newer than this daemon's.
    schema_too_new(&dir, "A#1");
    let failed = daemon
        .record(Some("A#1"), "stashes", &params, &body)
        .unwrap_err();
    let (error, payload) = kept_failure(&failed);
    assert!(
        error.contains("fetched, but the store could not record it")
            && error.contains("could not open"),
        "{error}"
    );
    assert_eq!(payload, &json!({ "params": params, "body": body }));

    // Unwritable after the open: the account's store is open read-only.
    let path = acquisition_store::account_path(&dir, "B#2");
    drop(Store::open(&path).unwrap());
    *daemon.store.lock().unwrap() = Some(("B#2".into(), Store::open_read_only(&path).unwrap()));
    let failed = daemon
        .record(Some("B#2"), "stashes", &params, &body)
        .unwrap_err();
    let (error, payload) = kept_failure(&failed);
    assert!(error.contains("recording failed"), "{error}");
    assert_eq!(payload["body"], body);

    // A malformed body stays the store's refusal, its evidence in `refused`.
    *daemon.store.lock().unwrap() = None;
    let refused = daemon
        .record(Some("B#2"), "stashes", &params, &json!({ "stashes": {} }))
        .unwrap_err();
    assert!(
        matches!(&refused, Outcome::Failure { payload: None, error, .. } if error.contains("malformed")),
        "{refused:?}"
    );
    drop(daemon);
    finish_harness(None, &log_path);
}

/// C144 (the daemon reports no success it did not earn), issue #90: a
/// kind the store knows no place for (`Endpoint::from_job` answers
/// `None`) fails the job naming the kind, where it used to return `Ok`
/// and the job ended done with nothing stored. No kind a client can
/// submit reaches this arm today (every caller of `record` is a kind the
/// store knows), so the pin is a direct call with a test-only kind, as
/// the test above is. The failure keeps the params and the body, and no
/// account file is created for a response nothing can be recorded under.
#[test]
fn c144_a_kind_the_store_cannot_place_fails_the_job_naming_the_kind() {
    let (mut daemon, log_path) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
    let dir = TestPath::dir();
    Arc::get_mut(&mut daemon).unwrap().store_dir = Some(dir.to_path_buf());
    let params = json!({ "league": "Standard", "id": "t1" });
    let body = json!({ "items": [] });

    let failed = daemon
        .record(Some("A#1"), "test-only-kind", &params, &body)
        .unwrap_err();
    let (error, payload) = kept_failure(&failed);
    assert!(
        error.contains("fetched, but the store could not record it")
            && error.contains("no place for a test-only-kind response"),
        "{error}"
    );
    assert_eq!(payload, &json!({ "params": params, "body": body }));
    assert!(
        !acquisition_store::account_path(&dir, "A#1").exists(),
        "no account file for a kind nothing can be recorded under"
    );
    assert!(daemon.store.lock().unwrap().is_none(), "no store opened");
    drop(daemon);
    finish_harness(None, &log_path);
}

/// C144 end to end: the job a store cannot record for ends failed, never
/// done, its outcome carrying the body GGG answered.
#[test]
fn c144_a_job_whose_store_cannot_open_ends_failed_with_its_body() {
    let store_dir = TestPath::dir();
    schema_too_new(&store_dir, "test-user");
    Door::open(
        Server::Account(("GET", "/never", 0)),
        Arc::new(ParkingClock::new()),
        Arc::new(RecordingCredentialStore::default()),
    )
    .sessions(logged_in_sessions())
    .store_dir(store_dir.to_path_buf())
    .run(|mut door| async move {
        let id = door.submit("profile", Some("test-user")).await;
        let (info, outcome) = door.await_terminal(id).await;
        assert_eq!(info.state, JobState::Failed, "{outcome:?}");
        let (_, payload) = kept_failure(&outcome);
        assert_eq!(payload["body"]["uuid"], "uuid-test-user");
        assert_eq!(
            door.sent(),
            [sent("test-user", "POST"), sent("test-user", "GET")]
        );
        door.finish().await;
    });
}

/// C144 (the daemon reports no success it did not earn), issue #63's
/// second comment: a restart after a failed write. A daemon on a
/// file-backed `daemon.db` whose account's facts file is newer than it
/// fails the profile job with the body kept (the test above, end to end);
/// a second lifetime on the same `daemon.db`, its queue restored as a
/// start restores it, restores no open job, since the table holds the
/// row as failed (C6: terminal rows are history, not the queue), answers
/// the id from the table as that failure with its body, lists no job and
/// so has nothing to replay: it journals nothing, and the account
/// server's record at the end is lifetime 1's two sends and nothing more,
/// read from the server, never from a daemon's report of itself.
#[test]
fn c144_a_restart_after_a_failed_record_keeps_the_failure_and_replays_nothing() {
    let db = test_db_path();
    let store_dir = TestPath::dir();
    schema_too_new(&store_dir, "test-user");
    let clock = Arc::new(ParkingClock::new());
    let parking = clock.clone();
    let dir = store_dir.to_path_buf();
    let path = db.to_path_buf();
    Door::open(
        Server::Account(("GET", "/never", 0)),
        clock,
        Arc::new(RecordingCredentialStore::default()),
    )
    .sessions(logged_in_sessions())
    .store_dir(store_dir.to_path_buf())
    .jobs_db(JobDb::open(&db).unwrap())
    .restore_jobs(Retention::default())
    .run(|mut door| async move {
        // Lifetime 1: the job fails after its send, its row written failed.
        let id = door.submit("profile", Some("test-user")).await;
        let (info, outcome) = door.await_terminal(id).await;
        assert_eq!(info.state, JobState::Failed, "{outcome:?}");

        // Lifetime 2: restored from the same file, its dispatcher running.
        let mut restarted = Door::open(
            Server::None,
            parking,
            Arc::new(RecordingCredentialStore::default()),
        )
        .sessions(logged_in_sessions())
        .store_dir(dir)
        .jobs_db(JobDb::open(&path).unwrap())
        .restore_jobs(Retention::default())
        .beside()
        .await;
        match restarted.request(Request::List).await {
            Response::Jobs { jobs } => assert!(jobs.is_empty(), "no job restored: {jobs:?}"),
            other => panic!("the list answered {other:?}"),
        }
        let stored = match restarted.request(Request::Result { id }).await {
            Response::Result { outcome, .. } => outcome,
            other => panic!("the failed job is answered from the table: {other:?}"),
        };
        let (error, payload) = kept_failure(&stored);
        assert!(error.contains("could not record it"), "{error}");
        assert_eq!(payload["body"]["uuid"], "uuid-test-user");
        assert!(restarted.journal().is_empty(), "the restart sent nothing");
        restarted.finish().await;

        assert_eq!(
            door.sent(),
            [sent("test-user", "POST"), sent("test-user", "GET")],
            "the server saw lifetime 1's sends and nothing more"
        );
        door.finish().await;
    });
}

// ---- C158: the wire table, through the door ----

/// One row of the wire table: a script the server answers, the steps a
/// frontend takes through the door in order, and what crosses the
/// boundary back. Every job the daemon lists at the row's end, in id
/// order (the submitted and the probes the daemon makes, as
/// `Request::List` lists them), ends as [`Job`] says; the journal's sends
/// are the row's as [`journal_projection`] reads them; each fragment of
/// `forbids` is absent from every surface it names; and the parking
/// clock moved at most `virtual_at_most` over the row. What the daemon's
/// status and dashboard answer mid-row is asserted by the step that reads
/// it ([`Step`]).
#[derive(Default)]
struct Row {
    /// What the row pins: where a ruling's *Pinned* names the test the
    /// row replaced, that test's name, then the case.
    name: String,
    script: Vec<ScriptedResponse>,
    /// The daemon starts with `test-user`'s session
    /// ([`logged_in_sessions`]), else with one empty session.
    logged_in: bool,
    steps: Vec<Step>,
    jobs: Vec<Job>,
    journal: Vec<Value>,
    forbids: Vec<Forbid>,
    virtual_at_most: Option<Duration>,
}

/// One step of a row, through the door, as a frontend takes it.
enum Step {
    /// Submits sent one after another, none awaited: jobs queued
    /// together. A submitted job is named after by its place among the
    /// row's submits.
    Send(Vec<Request>),
    /// The submitted jobs named, each awaited to the state given: a
    /// terminal one through `await_terminal`; a waiting one once the
    /// runtime has settled ([`Door::settle`]: the daemon has done all it
    /// can without time moving), through `await_job`.
    Await(Vec<(usize, JobState)>),
    /// `Request::Cancel` of the submitted job named, acknowledged.
    Cancel(usize),
    /// `Request::ResetTripwire`, acknowledged.
    ResetTripwire,
    /// `Request::DaemonStatus`: its `rails.halted` carries these words,
    /// or is `None`.
    DaemonStatus { halted: Option<String> },
    /// `Request::Dashboard`: its `degraded_endpoints` name these.
    Dashboard { degraded: Vec<&'static str> },
    /// A second lifetime while this one is live: a door opened beside
    /// this one on the same rails state path, facing no server, whose
    /// `DaemonStatus` `rails.halted` carries these words or is `None`;
    /// then finished.
    SecondLifetime { halted: Option<String> },
}

/// What a row expects of one job the daemon lists.
struct Job {
    kind: &'static str,
    end: JobState,
    retries: u32,
    /// The first item of its success's payload ([`fetch_payload_marker`]).
    marker: Option<&'static str>,
    /// Words its failure's error carries.
    carries: Vec<String>,
    /// Words its failure's error does not carry.
    lacks: Vec<String>,
}

impl Job {
    /// A `fetch` the row submitted, ending `end` with no retry.
    fn fetch(end: JobState) -> Job {
        Job {
            kind: "fetch",
            end,
            retries: 0,
            marker: None,
            carries: Vec::new(),
            lacks: Vec::new(),
        }
    }

    /// A probe the daemon made for a route's first use, ending `end`.
    fn probe(end: JobState) -> Job {
        Job {
            kind: "probe",
            ..Job::fetch(end)
        }
    }

    fn kind(self, kind: &'static str) -> Job {
        Job { kind, ..self }
    }

    fn retries(self, retries: u32) -> Job {
        Job { retries, ..self }
    }

    fn marker(self, marker: &'static str) -> Job {
        Job {
            marker: Some(marker),
            ..self
        }
    }

    fn carries(mut self, words: &[&str]) -> Job {
        self.carries.extend(words.iter().map(|w| w.to_string()));
        self
    }

    fn lacks(mut self, words: &[&str]) -> Job {
        self.lacks.extend(words.iter().map(|w| w.to_string()));
        self
    }
}

/// A fragment that must be absent from each surface it names.
struct Forbid {
    fragment: String,
    from: &'static [Surface],
}

/// A surface a [`Forbid`] names.
#[derive(Debug)]
enum Surface {
    /// The journal file the daemon wrote, its whole text, header
    /// included ([`Door::journal_text`]).
    Journal,
    /// `Dashboard.errors`, read at the row's end.
    DashboardErrors,
    /// Every listed job's failure words.
    JobErrors,
    /// Every `rails.halted` the row read, of either lifetime.
    Halts,
}

fn forbid(fragment: &str, from: &'static [Surface]) -> Forbid {
    Forbid {
        fragment: fragment.into(),
        from,
    }
}

/// A client's submit of `kind` with empty params, as a frontend sends it.
fn submit(kind: &str) -> Request {
    submit_at(kind, 0)
}

/// [`submit`] at `priority`.
fn submit_at(kind: &str, priority: Priority) -> Request {
    Request::Submit {
        kind: kind.into(),
        params: json!({}),
        priority,
        submitted_by: "test".into(),
        account: None,
    }
}

/// A client's submit of `kind` with `params` as `account`.
fn submit_as(kind: &str, params: Value, account: &str) -> Request {
    Request::Submit {
        kind: kind.into(),
        params,
        priority: 0,
        submitted_by: "test".into(),
        account: Some(account.into()),
    }
}

/// One `fetch` submitted and awaited to `end`.
fn fetched(end: JobState) -> Vec<Step> {
    vec![
        Step::Send(vec![submit("fetch")]),
        Step::Await(vec![(0, end)]),
    ]
}

/// One `fetch` submitted and awaited failed, the daemon not halted, and
/// a second `fetch` submitted and awaited landed.
fn not_halted_then_after() -> Vec<Step> {
    vec![
        Step::Send(vec![submit("fetch")]),
        Step::Await(vec![(0, JobState::Failed)]),
        Step::DaemonStatus { halted: None },
        Step::Send(vec![submit("fetch")]),
        Step::Await(vec![(1, JobState::Done)]),
    ]
}

/// Each send as a row compares it: method, path, status, ok, counted,
/// `wait_ms`, and the body's shape (`null` where the journal names none:
/// a 2xx, a send that never landed).
fn journal_projection(sends: &[Value]) -> Vec<Value> {
    sends
        .iter()
        .map(|l| {
            json!([
                l["method"],
                l["path"],
                l["status"],
                l["ok"],
                l["counted"],
                l["wait_ms"],
                l["shape"]
            ])
        })
        .collect()
}

/// C129's three block shapes, each with the status and the shape the
/// journal names: Cloudflare's documented 1015 (its one marker past the
/// body's first 300 characters), a challenge carrying `cf-mitigated`, and
/// a 429 with no header and no marker at all (N49).
fn cloudflare_blocks() -> [(&'static str, ScriptedResponse, u16, &'static str); 3] {
    let markerless_429 = ScriptedResponse {
        method: "GET",
        path: "/fetch",
        status: "429 Too Many Requests",
        headers: String::new(),
        body: "Too many requests from this client".into(),
    };
    [
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
    ]
}

/// Fragments of Cloudflare's pages that no failure, halt or mark keeps:
/// the page is Cloudflare's, no word of GGG's (C129).
const PAGE_FRAGMENTS: [&str; 6] = [
    "Ray ID",
    "<html",
    "<!DOCTYPE",
    "Just a moment",
    "rate limited",
    "Too many requests",
];

/// The headers a block's response carried, which no halt or mark keeps;
/// the journal keeps them as evidence on purpose (C129).
const BLOCK_HEADERS: [&str; 2] = ["challenge", "Retry-After"];

/// What C130's GET 302 echoes in its body and its `Location`'s query.
const C130_SENTINEL: &str = "c130-get-sentinel";

/// C130: a GET's 302 whose body and `Location` carry [`C130_SENTINEL`]
/// in the target's query.
fn c130_redirect() -> ScriptedResponse {
    let target = format!("https://www.pathofexile.com/login?code={C130_SENTINEL}");
    let mut redirect = ScriptedResponse::full(
        "GET",
        "302 Found",
        None,
        &format!(r#"<html><a href="{target}">Found</a></html>"#),
    );
    redirect
        .headers
        .push_str(&format!("Location: {target}\r\n"));
    redirect
}

/// The probe every `/fetch` row's first use sends, landed at once.
fn probe_sent() -> Value {
    json!(["HEAD", "/fetch", 204, true, false, 0, null])
}

/// The rows, each written from the ruling it pins and the assertions of
/// the test it replaced, never from a run of the door.
fn wire_rows() -> Vec<Row> {
    use JobState::{Cancelled, Done, Failed, Waiting};
    let hold = full_hold_ms();
    let mut rows = vec![
        // The 429 hold: one fetch; its probe; its GET answered 429 with
        // GGG's headers and `Retry-After: 0`, held the bucket pad and the
        // buffer (61 s); its GET again after the whole hold, landing.
        Row {
            name: "the 429 hold".into(),
            script: vec![
                ScriptedResponse::full("HEAD", "204 No Content", None, ""),
                ScriptedResponse::full("GET", "429 Too Many Requests", Some(0), "{}"),
                ScriptedResponse::full("GET", "200 OK", None, r#"{"items":["after the hold"]}"#),
            ],
            steps: fetched(Done),
            jobs: vec![
                Job::fetch(Done).retries(1).marker("after the hold"),
                Job::probe(Done),
            ],
            journal: vec![
                probe_sent(),
                json!(["GET", "/fetch", 429, false, true, 0, "unclassified"]),
                json!(["GET", "/fetch", 200, true, true, hold, null]),
            ],
            ..Row::default()
        },
        // C26: two 429s with GGG's headers re-queue the job, each retry
        // behind its `Retry-After` plus the bucket pad (C19), and the
        // third answer lands it, once.
        Row {
            name: "dispatcher_retries_429_429_success_exactly_three_times_and_completes_once"
                .into(),
            script: vec![
                ScriptedResponse::full("HEAD", "204 No Content", None, ""),
                ScriptedResponse::full("GET", "429 Too Many Requests", Some(0), "{}"),
                ScriptedResponse::full("GET", "429 Too Many Requests", Some(5), "{}"),
                ScriptedResponse::full("GET", "200 OK", None, r#"{"items":["done"]}"#),
            ],
            steps: fetched(Done),
            jobs: vec![
                Job::fetch(Done).retries(MAX_429_RETRIES).marker("done"),
                Job::probe(Done),
            ],
            journal: vec![
                probe_sent(),
                json!(["GET", "/fetch", 429, false, true, 0, "unclassified"]),
                json!(["GET", "/fetch", 429, false, true, hold, "unclassified"]),
                json!(["GET", "/fetch", 200, true, true, 5_000 + hold, null]),
            ],
            ..Row::default()
        },
        // C26: after `MAX_429_RETRIES` the third 429 fails the job with
        // the evidence; no fourth send, and the third's hold is not slept
        // out first: the clock moves the two holds the retries waited.
        Row {
            name: "dispatcher_exhausts_on_third_429_without_a_fourth_send_or_final_sleep".into(),
            script: vec![
                ScriptedResponse::full("HEAD", "204 No Content", None, ""),
                ScriptedResponse::full("GET", "429 Too Many Requests", Some(0), "{}"),
                ScriptedResponse::full("GET", "429 Too Many Requests", Some(0), "{}"),
                ScriptedResponse::full("GET", "429 Too Many Requests", Some(0), "{}"),
            ],
            steps: fetched(Failed),
            jobs: vec![
                Job::fetch(Failed)
                    .retries(MAX_429_RETRIES)
                    .carries(&["giving up"]),
                Job::probe(Done),
            ],
            journal: vec![
                probe_sent(),
                json!(["GET", "/fetch", 429, false, true, 0, "unclassified"]),
                json!(["GET", "/fetch", 429, false, true, hold, "unclassified"]),
                json!(["GET", "/fetch", 429, false, true, hold, "unclassified"]),
            ],
            virtual_at_most: Some(Duration::from_millis(2 * hold)),
            ..Row::default()
        },
        // C26: a re-queued job keeps its place; two queued together land
        // in submission order, each with its own answer.
        Row {
            name: "dispatcher_requeue_preserves_fifo_job_identity".into(),
            script: vec![
                ScriptedResponse::full("HEAD", "204 No Content", None, ""),
                ScriptedResponse::full("GET", "429 Too Many Requests", Some(0), "{}"),
                ScriptedResponse::full("GET", "200 OK", None, r#"{"items":["first"]}"#),
                ScriptedResponse::full("GET", "200 OK", None, r#"{"items":["second"]}"#),
            ],
            steps: vec![
                Step::Send(vec![submit("fetch"), submit("fetch")]),
                Step::Await(vec![(0, Done), (1, Done)]),
            ],
            jobs: vec![
                Job::fetch(Done).retries(1).marker("first"),
                Job::fetch(Done).marker("second"),
                Job::probe(Done),
            ],
            journal: vec![
                probe_sent(),
                json!(["GET", "/fetch", 429, false, true, 0, "unclassified"]),
                json!(["GET", "/fetch", 200, true, true, hold, null]),
                json!(["GET", "/fetch", 200, true, true, 0, null]),
            ],
            ..Row::default()
        },
        // C19 for a probe: a HEAD's 429 with GGG's full headers and an
        // acceptable `Retry-After` establishes the route under its hold;
        // the probe and the job each end with no retry, and the GET waits
        // the whole hold.
        Row {
            name: "full_acceptable_head_429_establishes_under_hold_without_job_retry".into(),
            script: vec![
                ScriptedResponse::full("HEAD", "429 Too Many Requests", Some(0), ""),
                ScriptedResponse::full("GET", "200 OK", None, r#"{"items":["after-probe"]}"#),
            ],
            steps: fetched(Done),
            jobs: vec![Job::fetch(Done).marker("after-probe"), Job::probe(Done)],
            journal: vec![
                json!(["HEAD", "/fetch", 429, false, false, 0, "unclassified"]),
                json!(["GET", "/fetch", 200, true, true, hold, null]),
            ],
            ..Row::default()
        },
    ];
    // C19: a landed GET 429 holds its route for its `Retry-After`, clamped
    // to the cap, plus the retry bucket, whatever the header carried.
    // Such a 429 is not retried, so a second job's send shows the hold:
    // 61 s after a missing, garbled or negative value, 961 s after an
    // over-cap one.
    let missing = || ScriptedResponse::full("GET", "429 Too Many Requests", None, "{}");
    for (case, rejected, held) in [
        ("missing", missing(), hold),
        ("garbled", missing().with_raw_retry_after("soon"), hold),
        ("negative", missing().with_raw_retry_after("-1"), hold),
        (
            "over the cap",
            missing().with_raw_retry_after("5000"),
            crate::ratelimit::RETRY_AFTER_CAP_SECS * 1000 + hold,
        ),
    ] {
        rows.push(Row {
            name: format!("a_get_429_without_a_usable_retry_after_holds_the_next_send: {case}"),
            script: vec![
                ScriptedResponse::full("HEAD", "204 No Content", None, ""),
                rejected,
                ScriptedResponse::full("GET", "200 OK", None, r#"{"items":["after"]}"#),
            ],
            steps: vec![
                Step::Send(vec![submit("fetch")]),
                Step::Await(vec![(0, Failed)]),
                Step::Send(vec![submit("fetch")]),
                Step::Await(vec![(1, Done)]),
            ],
            jobs: vec![
                Job::fetch(Failed),
                Job::probe(Done),
                Job::fetch(Done).marker("after"),
            ],
            journal: vec![
                probe_sent(),
                json!(["GET", "/fetch", 429, false, true, 0, "unclassified"]),
                json!(["GET", "/fetch", 200, true, true, held, null]),
            ],
            ..Row::default()
        });
    }
    // A HEAD 429 whose policy header is malformed, or whose `Retry-After`
    // is missing, fails setup: the probe and its job fail with no retry,
    // nothing more is sent, and the route is degraded under a cooldown.
    for (case, head) in [
        ("malformed", ScriptedResponse::malformed_head_429()),
        (
            "unacceptable",
            ScriptedResponse::full("HEAD", "429 Too Many Requests", None, ""),
        ),
    ] {
        rows.push(Row {
            name: format!("malformed_or_unacceptable_head_429_fails_under_cooldown: {case}"),
            script: vec![head],
            steps: fetched(Failed)
                .into_iter()
                .chain([Step::Dashboard {
                    degraded: vec!["fetch"],
                }])
                .collect(),
            jobs: vec![Job::fetch(Failed), Job::probe(Failed)],
            journal: vec![json!([
                "HEAD",
                "/fetch",
                429,
                false,
                false,
                0,
                "unclassified"
            ])],
            ..Row::default()
        });
    }
    // C129: a response Cloudflare gave in GGG's place is a stop. With the
    // tripwire off, the block fails its job at once (no re-queue, whatever
    // its `Retry-After`), in the halt's words with its remedy; it admits no
    // further send on any route (a second account's job, whose session
    // would need a refresh first, submitted only after the block landed,
    // since the gate is two wide, stays waiting with nothing on the wire
    // for it); and it leaves a mark that a later daemon on the same rails
    // state, also without the tripwire, reports. The halt names the
    // status, method, route and time, never the body: no fragment of the
    // page reaches the job's failure or either lifetime's halt, and no
    // header the response carried reaches the halt (the journal keeps the
    // headers as evidence on purpose). The mark's file on disk is the
    // component case `a_cloudflare_blocks_mark_on_disk_keeps_no_page_or_header`.
    // The three shapes: Cloudflare's documented 1015 (its one marker past
    // the body's first 300 characters), a challenge carrying
    // `cf-mitigated`, and a 429 with no header and no marker at all (N49).
    // The 1015 still holds its route for its `Retry-After` (C19): once the
    // sibling is cancelled and the mark reset, a further fetch waits 30 s
    // plus the retry bucket.
    for (case, block, status, shape) in cloudflare_blocks() {
        let cause =
            format!("Cloudflare answered in GGG's place: {status} on GET fetch at 2000-01-01T");
        let mut forbids: Vec<Forbid> = PAGE_FRAGMENTS
            .into_iter()
            .filter(|fragment| block.body.contains(fragment))
            .map(|fragment| forbid(fragment, &[Surface::JobErrors, Surface::Halts]))
            .collect();
        forbids.extend(BLOCK_HEADERS.map(|header| forbid(header, &[Surface::Halts])));
        let mut row = Row {
            name: format!("a_cloudflare_block_admits_nothing_and_survives_a_restart: {case}"),
            script: vec![
                ScriptedResponse::full("HEAD", "204 No Content", None, ""),
                block,
            ],
            logged_in: true,
            steps: vec![
                Step::Send(vec![submit("fetch")]),
                Step::Await(vec![(0, Failed)]),
                Step::Send(vec![submit_as(
                    "stashes",
                    json!({ "league": "Standard" }),
                    "test-user",
                )]),
                Step::Await(vec![(1, Waiting)]),
                Step::DaemonStatus {
                    halted: Some(cause.clone()),
                },
                Step::SecondLifetime {
                    halted: Some(cause),
                },
            ],
            jobs: vec![
                Job::fetch(Failed)
                    .carries(&["Cloudflare answered in GGG's place", "reset-tripwire"]),
                Job::probe(Done),
                Job::fetch(Waiting).kind("stashes"),
            ],
            journal: vec![
                probe_sent(),
                json!(["GET", "/fetch", status, false, true, 0, shape]),
            ],
            forbids,
            ..Row::default()
        };
        if case == "1015" {
            row.steps.extend([
                Step::Cancel(1),
                Step::ResetTripwire,
                Step::Send(vec![submit("fetch")]),
                Step::Await(vec![(2, Done)]),
            ]);
            row.script.push(ScriptedResponse::full(
                "GET",
                "200 OK",
                None,
                r#"{"items":["after the block"]}"#,
            ));
            row.jobs[2] = Job::fetch(Cancelled).kind("stashes");
            row.jobs.push(Job::fetch(Done).marker("after the block"));
            row.journal.push(json!([
                "GET",
                "/fetch",
                200,
                true,
                true,
                30_000 + hold,
                null
            ]));
        }
        rows.push(row);
    }
    // C129: a 5xx Cloudflare gave for an origin it could not reach, by
    // its status alone (the 522 with its page and with no body at all, a
    // 530) or by carrying Cloudflare's page with no GGG policy header (a
    // 503), fails its job at once, never retried (C26), in the halt's
    // words with its remedy and none of the page; nothing more is sent
    // this daemon lifetime (a sibling submitted after it landed stays
    // waiting). The halt is the lifetime's and leaves no mark: a second
    // lifetime on the same rails state reports none, read before the
    // reset (which clears the persisted marks too). `reset-tripwire`
    // lifts it, and the sibling goes out to the script's pending GET.
    let cloudflare_503 = ScriptedResponse {
        status: "503 Service Unavailable",
        ..ScriptedResponse::cloudflare_522()
    };
    let empty_522 = ScriptedResponse {
        body: String::new(),
        ..ScriptedResponse::cloudflare_522()
    };
    let empty_530 = ScriptedResponse {
        status: "530 Origin Unreachable",
        body: String::new(),
        ..ScriptedResponse::cloudflare_522()
    };
    for (case, unreachable, status, shape) in [
        ("522", ScriptedResponse::cloudflare_522(), 522, "cloudflare"),
        (
            "503 with Cloudflare's page",
            cloudflare_503,
            503,
            "cloudflare",
        ),
        ("522 with no body", empty_522, 522, "unclassified"),
        ("530 with no body", empty_530, 530, "unclassified"),
    ] {
        rows.push(Row {
            name: format!("a_cloudflare_5xx_halts_this_lifetime_and_leaves_no_mark: {case}"),
            script: vec![
                ScriptedResponse::full("HEAD", "204 No Content", None, ""),
                unreachable,
                ScriptedResponse::full("GET", "200 OK", None, r#"{"items":["after"]}"#),
            ],
            steps: vec![
                Step::Send(vec![submit("fetch")]),
                Step::Await(vec![(0, Failed)]),
                Step::Send(vec![submit("fetch")]),
                Step::Await(vec![(1, Waiting)]),
                Step::DaemonStatus {
                    halted: Some(format!(
                        "Cloudflare could not reach GGG: {status} on GET fetch at 2000-01-01T"
                    )),
                },
                Step::SecondLifetime { halted: None },
                Step::ResetTripwire,
                Step::Await(vec![(1, Done)]),
            ],
            jobs: vec![
                Job::fetch(Failed).carries(&[
                    "Cloudflare could not reach GGG",
                    "this daemon lifetime",
                    "reset-tripwire",
                ]),
                Job::probe(Done),
                Job::fetch(Done).marker("after"),
            ],
            journal: vec![
                probe_sent(),
                json!(["GET", "/fetch", status, false, true, 0, shape]),
                json!(["GET", "/fetch", 200, true, true, 0, null]),
            ],
            forbids: vec![
                forbid("Ray ID", &[Surface::JobErrors, Surface::Halts]),
                forbid("DOCTYPE", &[Surface::JobErrors]),
            ],
            ..Row::default()
        });
    }
    // C129's 5xx without Cloudflare's shape halts nothing: GGG's origin
    // 503 (N35's openresty page, no marker, no rate header) and a 5xx whose
    // body holds a marker beside GGG's policy header (GGG's JSON echoing a
    // name the user chose; a page Cloudflare serves in GGG's place cannot
    // carry GGG's headers) each fail their job, never retried, and the
    // next job's GET goes out.
    for (case, unshaped, status, shape) in [
        ("origin 503", ScriptedResponse::origin_503(), 503, "origin"),
        (
            "GGG's 500 echoing CloudflareFan",
            ScriptedResponse::full(
                "GET",
                "500 Internal Server Error",
                None,
                r#"{"error":{"code":0,"message":"Internal error for CloudflareFan, Ray ID: 8a1"}}"#,
            ),
            500,
            "cloudflare",
        ),
    ] {
        rows.push(Row {
            name: format!("a_5xx_without_cloudflares_shape_halts_nothing: {case}"),
            script: vec![
                ScriptedResponse::full("HEAD", "204 No Content", None, ""),
                unshaped,
                ScriptedResponse::full("GET", "200 OK", None, r#"{"items":["after"]}"#),
            ],
            steps: not_halted_then_after(),
            jobs: vec![
                Job::fetch(Failed).lacks(&["Cloudflare could not"]),
                Job::probe(Done),
                Job::fetch(Done).marker("after"),
            ],
            journal: vec![
                probe_sent(),
                json!(["GET", "/fetch", status, false, true, 0, shape]),
                json!(["GET", "/fetch", 200, true, true, 0, null]),
            ],
            ..Row::default()
        });
    }
    // C129's false positives: GGG's own 4xx is no block. A JSON 404 with
    // GGG's headers (its message saying "array id", which holds "ray id"
    // inside a word), a 403 that names its own auth error in
    // `WWW-Authenticate` and carries no `cf-mitigated` (its body mentioning
    // Cloudflare), and GGG's JSON echoing a name the user chose that holds
    // a page marker ("CloudflareFan", a league "Error 1015 SSF") beside
    // GGG's policy header, each fail their job and leave the daemon
    // sending: the next job's GET goes out.
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
    for (case, rejected, status, shape) in [
        (
            "GGG's 404",
            ScriptedResponse::full(
                "GET",
                "404 Not Found",
                None,
                r#"{"error":{"code":1,"message":"Resource not found: unknown array id"}}"#,
            ),
            404,
            "unclassified",
        ),
        ("an auth error", auth_error, 403, "cloudflare"),
        (
            "GGG's 404 echoing CloudflareFan",
            ScriptedResponse::full(
                "GET",
                "404 Not Found",
                None,
                r#"{"error":{"code":1,"message":"Character not found: CloudflareFan"}}"#,
            ),
            404,
            "cloudflare",
        ),
        (
            "GGG's 400 echoing a league",
            ScriptedResponse::full(
                "GET",
                "400 Bad Request",
                None,
                r#"{"error":{"code":2,"message":"Invalid league: Error 1015 SSF"}}"#,
            ),
            400,
            "cloudflare",
        ),
    ] {
        rows.push(Row {
            name: format!("ggg_4xx_answers_are_not_cloudflare_blocks: {case}"),
            script: vec![
                ScriptedResponse::full("HEAD", "204 No Content", None, ""),
                rejected,
                ScriptedResponse::full("GET", "200 OK", None, r#"{"items":["after"]}"#),
            ],
            steps: not_halted_then_after(),
            jobs: vec![
                Job::fetch(Failed).lacks(&["Cloudflare answered"]),
                Job::probe(Done),
                Job::fetch(Done).marker("after"),
            ],
            journal: vec![
                probe_sent(),
                json!(["GET", "/fetch", status, false, true, 0, shape]),
                json!(["GET", "/fetch", 200, true, true, 0, null]),
            ],
            ..Row::default()
        });
    }
    // C130: a GET's 302 fails its job with the status and the redirect's
    // target shown only redacted. The body, which echoes the target with
    // its query, is not kept: the sentinel reaches neither the job's
    // failure, the journal nor the daemon's recent errors. The daemon log
    // is the component case `a_get_3xx_keeps_its_target_out_of_the_daemon_log`.
    rows.push(Row {
        name: "a_get_3xx_fails_its_job_with_the_target_redacted_and_no_body".into(),
        script: vec![
            ScriptedResponse::full("HEAD", "204 No Content", None, ""),
            c130_redirect(),
        ],
        steps: fetched(Failed),
        jobs: vec![
            Job::fetch(Failed).carries(&["302 Found", "https://www.pathofexile.com/login?…"]),
            Job::probe(Done),
        ],
        journal: vec![
            probe_sent(),
            json!(["GET", "/fetch", 302, false, true, 0, "unclassified"]),
        ],
        forbids: vec![forbid(
            C130_SENTINEL,
            &[
                Surface::Journal,
                Surface::DashboardErrors,
                Surface::JobErrors,
            ],
        )],
        ..Row::default()
    });
    // C19, rung 10 (2026-08-24) with the tripwire off: a 503 with no rate
    // headers (N35's origin page) fails its job, never retried, and the
    // next job's send is paced as if the 503 counted: its window full, it
    // waits the 10 s period, the 5 s initial-window bucket and the 1 s
    // buffer, measured from the 503 itself.
    let policy = "X-Rate-Limit-Policy: dispatcher-test-policy\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 1:10:60\r\nX-Rate-Limit-Account-State: 0:10:0\r\n";
    rows.push(Row {
        name: "headerless_503_fails_its_job_and_paces_the_next_send".into(),
        script: vec![
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
        ],
        steps: vec![
            Step::Send(vec![submit("fetch"), submit("fetch")]),
            Step::Await(vec![(0, Failed), (1, Done)]),
        ],
        jobs: vec![
            Job::fetch(Failed).carries(&["origin error page"]),
            Job::fetch(Done).marker("done"),
            Job::probe(Done),
        ],
        journal: vec![
            probe_sent(),
            json!(["GET", "/fetch", 503, false, true, 0, "origin"]),
            json!(["GET", "/fetch", 200, true, true, 16_000, null]),
        ],
        ..Row::default()
    });
    // C26: a 403 or a 503 is never retried and never waited on (invariant
    // 3). The third case is rung 10's live 503 (2026-08-24), an origin
    // page, named as such and still never retried.
    const ORIGIN_503: &str = "<html><head><title>503 Service Temporarily Unavailable</title></head><body><center><h1>503 Service Temporarily Unavailable</h1></center><hr><center>openresty</center></body></html>";
    for (case, status_line, status, body, shape, words) in [
        (
            "403",
            "403 Forbidden",
            403,
            "{}",
            "unclassified",
            "unclassified body, possibly a Cloudflare block",
        ),
        (
            "503",
            "503 Service Unavailable",
            503,
            "{}",
            "unclassified",
            "unclassified body, possibly a Cloudflare block",
        ),
        (
            "rung 10's origin 503",
            "503 Service Unavailable",
            503,
            ORIGIN_503,
            "origin",
            "origin error page, not Cloudflare-shaped",
        ),
    ] {
        rows.push(Row {
            name: format!("dispatcher_never_retries_403_or_503: {case}"),
            script: vec![
                ScriptedResponse::full("HEAD", "204 No Content", None, ""),
                ScriptedResponse::full("GET", status_line, None, body),
            ],
            steps: fetched(Failed),
            jobs: vec![
                Job::fetch(Failed).carries(&["NOT retrying", words]),
                Job::probe(Done),
            ],
            journal: vec![
                probe_sent(),
                json!(["GET", "/fetch", status, false, true, 0, shape]),
            ],
            ..Row::default()
        });
    }
    // C5: on one scheduling key the higher priority goes out first,
    // whatever the order of submission: two jobs queued together, the later
    // at the higher priority, which holds the server's first answer.
    let policy = "X-Rate-Limit-Policy: dispatcher-test-policy\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 100:1:60\r\nX-Rate-Limit-Account-State: 0:1:0\r\n";
    let answer = |marker: &str| ScriptedResponse {
        method: "GET",
        path: "/fetch",
        status: "200 OK",
        headers: policy.into(),
        body: format!(r#"{{"items":["{marker}"]}}"#),
    };
    rows.push(Row {
        name: "the_higher_priority_job_on_a_scheduling_key_reaches_the_wire_first".into(),
        script: vec![
            ScriptedResponse {
                method: "HEAD",
                path: "/fetch",
                status: "204 No Content",
                headers: policy.into(),
                body: String::new(),
            },
            answer("first"),
            answer("second"),
        ],
        steps: vec![
            Step::Send(vec![submit_at("fetch", 1), submit_at("fetch", 9)]),
            Step::Await(vec![(1, Done), (0, Done)]),
        ],
        jobs: vec![
            Job::fetch(Done).marker("second"),
            Job::fetch(Done).marker("first"),
            Job::probe(Done),
        ],
        journal: vec![
            probe_sent(),
            json!(["GET", "/fetch", 200, true, true, 0, null]),
            json!(["GET", "/fetch", 200, true, true, 0, null]),
        ],
        ..Row::default()
    });
    // The narrow HEAD rule (C129, the owner's choice): a HEAD 403 with no
    // `cf-mitigated` is no block, even with no GGG header (`/profile`'s
    // HEAD is answered so, N38). It degrades the route as before, its job
    // fails as "route degraded", and nothing is halted.
    rows.push(Row {
        name: "a_head_403_without_cf_mitigated_still_degrades".into(),
        script: vec![ScriptedResponse {
            method: "HEAD",
            path: "/fetch",
            status: "403 Forbidden",
            headers: String::new(),
            body: String::new(),
        }],
        steps: fetched(Failed)
            .into_iter()
            .chain([
                Step::DaemonStatus { halted: None },
                Step::Dashboard {
                    degraded: vec!["fetch"],
                },
            ])
            .collect(),
        jobs: vec![
            Job::fetch(Failed).carries(&["degraded"]),
            Job::probe(Failed),
        ],
        journal: vec![json!([
            "HEAD",
            "/fetch",
            403,
            false,
            false,
            0,
            "unclassified"
        ])],
        ..Row::default()
    });
    // The wire twins of `responses.rs`, the classification's component
    // table (`roadmap/briefs/t5-design.md` §3): three of its judgements
    // staged at the boundary, each GET carrying GGG's headers. A twin
    // carries its cause's words, never the server's port.
    let ggg = ScriptedResponse::full("GET", "200 OK", None, "").headers;
    // A 200 whose body is cut short of its `Content-Length` is a transport
    // failure, in the transport path's words, never bad JSON: the body is
    // read before anything classifies it
    // (`body_read_transport_failure_is_not_recast_as_bad_json`). The send
    // is journaled not ok, with its status.
    rows.push(Row {
        name: "a_200_cut_short_is_a_transport_failure_not_bad_json".into(),
        script: vec![
            ScriptedResponse::full("HEAD", "204 No Content", None, ""),
            ScriptedResponse::raw(&format!(
                "HTTP/1.1 200 OK\r\n{ggg}Content-Type: application/json\r\nContent-Length: 64\r\n\r\n{{\"items\":[\"cut"
            )),
        ],
        steps: fetched(Failed),
        jobs: vec![
            Job::fetch(Failed)
                .carries(&["GET http://", "/fetch failed: ", "error decoding response body"])
                .lacks(&["bad JSON"]),
            Job::probe(Done),
        ],
        journal: vec![
            probe_sent(),
            json!(["GET", "/fetch", 200, false, true, 0, null]),
        ],
        ..Row::default()
    });
    // A 500 whose body is cut short keeps its status first and names the
    // body's failure beside it
    // (`http_status_keeps_precedence_over_error_body_transport_failure`).
    rows.push(Row {
        name: "a_500_cut_short_keeps_its_status_and_names_the_body_read_failure".into(),
        script: vec![
            ScriptedResponse::full("HEAD", "204 No Content", None, ""),
            ScriptedResponse::raw(&format!(
                "HTTP/1.1 500 Internal Server Error\r\n{ggg}Content-Type: application/json\r\nContent-Length: 64\r\n\r\n{{\"error\""
            )),
        ],
        steps: fetched(Failed),
        jobs: vec![
            Job::fetch(Failed).carries(&[
                "returned 500",
                "a body read transport failure: error decoding response body",
            ]),
            Job::probe(Done),
        ],
        journal: vec![
            probe_sent(),
            json!(["GET", "/fetch", 500, false, true, 0, "unclassified"]),
        ],
        ..Row::default()
    });
    // A 200 whose body is not JSON, with GGG's headers so the rate-limit
    // protocol arm does not fire first, is bad JSON
    // (`malformed_success_body_remains_a_json_failure`); the send landed.
    rows.push(Row {
        name: "a_200_whose_body_is_not_json_is_bad_json".into(),
        script: vec![
            ScriptedResponse::full("HEAD", "204 No Content", None, ""),
            ScriptedResponse::full("GET", "200 OK", None, "not json"),
        ],
        steps: fetched(Failed),
        jobs: vec![Job::fetch(Failed).carries(&["bad JSON"]), Job::probe(Done)],
        journal: vec![
            probe_sent(),
            json!(["GET", "/fetch", 200, true, true, 0, null]),
        ],
        ..Row::default()
    });
    rows
}

/// C158: every row of the wire table through the door on a parking
/// clock, the runtime's park hook moving virtual time whenever nothing
/// the scenario started can run. The runner subscribes to the job events
/// once, before any submit, as a frontend does: every job a row ends
/// published exactly one terminal event, and a job it leaves waiting
/// none; a subscription that lost events fails the row. It reads nothing
/// but the protocol's answers, the journal and its own clock. What the
/// hook did prints under `--no-capture`, asserted by nothing.
#[test]
fn c158_the_wire_table_through_the_door() {
    for row in wire_rows() {
        let Row {
            name,
            script,
            logged_in,
            steps,
            jobs,
            journal,
            forbids,
            virtual_at_most,
        } = row;
        let clock = Arc::new(ParkingClock::new());
        let origin = clock.now();
        let parking = clock.clone();
        // A row that reads a second lifetime gives both the same rails
        // state, where a restart finds a block's mark.
        let state = TestPath::dir();
        let state_path = steps
            .iter()
            .any(|step| matches!(step, Step::SecondLifetime { .. }))
            .then(|| state.join("rails.json"));
        let mut door = Door::open(
            Server::Script(script),
            clock,
            Arc::new(RecordingCredentialStore::default()),
        )
        .rails(RailsConfig {
            state_path: state_path.clone(),
            ..RailsConfig::default()
        });
        if logged_in {
            door = door.sessions(logged_in_sessions());
        }
        let row = name.clone();
        let hook = door.run(|mut door| async move {
            let mut events = door.subscribe().await;
            let mut submitted = Vec::new();
            let mut halts = Vec::new();
            for step in steps {
                match step {
                    Step::Send(requests) => {
                        for request in requests {
                            match door.request(request).await {
                                Response::Submitted { id } => submitted.push(id),
                                other => panic!("{name}: a submit answered {other:?}"),
                            }
                        }
                    }
                    Step::Await(wanted) => {
                        for (submit, state) in wanted {
                            let id = submitted[submit];
                            let reached = if state.is_terminal() {
                                door.await_terminal(id).await.0.state
                            } else {
                                door.settle().await;
                                door.await_job(id, |job| job.state != JobState::Running)
                                    .await
                                    .state
                            };
                            assert_eq!(reached, state, "{name}: submit {submit} (job {id})");
                        }
                    }
                    Step::Cancel(submit) => {
                        let id = submitted[submit];
                        let answer = door.request(Request::Cancel { id }).await;
                        assert!(matches!(answer, Response::Ack { .. }), "{name}: {answer:?}");
                    }
                    Step::ResetTripwire => {
                        let answer = door.request(Request::ResetTripwire).await;
                        assert!(matches!(answer, Response::Ack { .. }), "{name}: {answer:?}");
                    }
                    Step::DaemonStatus { halted } => {
                        let read = halted_of(door.request(Request::DaemonStatus).await);
                        assert_halted(&name, "this lifetime", read.as_deref(), halted.as_deref());
                        halts.extend(read);
                    }
                    Step::Dashboard { degraded } => {
                        let Response::Dashboard {
                            degraded_endpoints, ..
                        } = door.request(Request::Dashboard).await
                        else {
                            panic!("{name}: the dashboard did not answer")
                        };
                        let named: Vec<&str> = degraded_endpoints
                            .iter()
                            .map(|endpoint| endpoint.endpoint.as_str())
                            .collect();
                        assert_eq!(named, degraded, "{name}: the degraded endpoints");
                    }
                    Step::SecondLifetime { halted } => {
                        let mut second = Door::open(
                            Server::None,
                            Arc::new(ParkingClock::new()),
                            Arc::new(RecordingCredentialStore::default()),
                        )
                        .rails(RailsConfig {
                            state_path: state_path.clone(),
                            ..RailsConfig::default()
                        })
                        .beside()
                        .await;
                        let read = halted_of(second.request(Request::DaemonStatus).await);
                        second.finish().await;
                        assert_halted(
                            &name,
                            "a second lifetime",
                            read.as_deref(),
                            halted.as_deref(),
                        );
                        halts.extend(read);
                    }
                }
            }

            // Every job the daemon lists, against the row's, with the
            // terminal events the subscription carried.
            let listed = match door.request(Request::List).await {
                Response::Jobs { jobs } => jobs,
                other => panic!("{name}: the job list reads {other:?}"),
            };
            let mut ended: HashMap<JobId, usize> = HashMap::new();
            loop {
                match events.try_recv() {
                    Ok(job) if job.state.is_terminal() => *ended.entry(job.id).or_default() += 1,
                    Ok(_) => {}
                    Err(broadcast::error::TryRecvError::Empty) => break,
                    Err(error) => panic!("{name}: the subscription lost events: {error:?}"),
                }
            }
            assert_eq!(
                listed.len(),
                jobs.len(),
                "{name}: the jobs listed: {listed:?}"
            );
            let mut errors = Vec::new();
            for (job, want) in listed.iter().zip(&jobs) {
                let id = job.id;
                assert_eq!(
                    (job.kind.as_str(), job.state, job.retries),
                    (want.kind, want.end, want.retries),
                    "{name}: job {id}"
                );
                assert_eq!(
                    ended.remove(&id).unwrap_or(0),
                    usize::from(job.state.is_terminal()),
                    "{name}: job {id}'s terminal events"
                );
                if !job.state.is_terminal() {
                    assert!(
                        want.marker.is_none() && want.carries.is_empty() && want.lacks.is_empty(),
                        "{name}: job {id} has no outcome to read"
                    );
                    continue;
                }
                let outcome = match door.request(Request::Result { id }).await {
                    Response::Result { outcome, .. } => outcome,
                    other => panic!("{name}: job {id}'s result reads {other:?}"),
                };
                if let Some(marker) = want.marker {
                    assert_eq!(fetch_payload_marker(&outcome), marker, "{name}: job {id}");
                }
                match &outcome {
                    Outcome::Failure { error, .. } => {
                        for words in &want.carries {
                            assert!(
                                error.contains(words.as_str()),
                                "{name}: job {id} lacks {words:?}: {error}"
                            );
                        }
                        for words in &want.lacks {
                            assert!(
                                !error.contains(words.as_str()),
                                "{name}: job {id} carries {words:?}: {error}"
                            );
                        }
                        errors.push(error.clone());
                    }
                    _ => assert!(
                        want.carries.is_empty() && want.lacks.is_empty(),
                        "{name}: job {id} has no failure's words to read: {outcome:?}"
                    ),
                }
            }
            assert!(
                ended.is_empty(),
                "{name}: terminal events of jobs not listed: {ended:?}"
            );

            let sends = door.journal();
            assert_eq!(journal_projection(&sends), journal, "{name}: the journal");
            let dashboard_errors = match door.request(Request::Dashboard).await {
                Response::Dashboard { errors, .. } => serde_json::to_string(&errors).unwrap(),
                other => panic!("{name}: the dashboard reads {other:?}"),
            };
            let journal_text = door.journal_text();
            for Forbid { fragment, from } in &forbids {
                for surface in *from {
                    let texts: Vec<&str> = match surface {
                        Surface::Journal => vec![&journal_text],
                        Surface::DashboardErrors => vec![&dashboard_errors],
                        Surface::JobErrors => errors.iter().map(String::as_str).collect(),
                        Surface::Halts => halts.iter().map(String::as_str).collect(),
                    };
                    for text in texts {
                        assert!(
                            !text.contains(fragment.as_str()),
                            "{name}: {surface:?} keeps {fragment:?}: {text}"
                        );
                    }
                }
            }
            if let Some(bound) = virtual_at_most {
                let moved = parking.now() - origin;
                assert!(
                    moved <= bound,
                    "{name}: the clock moved {moved:?}, past {bound:?}"
                );
            }
            door.finish().await;
        });
        drop(state);
        eprintln!("{row}: the park hook: {hook:?}");
    }
}

/// `DaemonStatus`'s `rails.halted`.
fn halted_of(answer: Response) -> Option<String> {
    match answer {
        Response::DaemonStatus { rails, .. } => rails.halted,
        other => panic!("the daemon's status reads {other:?}"),
    }
}

/// A lifetime's halt as a row expects it: carrying `want`'s words, or none.
fn assert_halted(name: &str, lifetime: &str, read: Option<&str>, want: Option<&str>) {
    match want {
        Some(words) => assert!(
            read.is_some_and(|halted| halted.contains(words)),
            "{name}: {lifetime}'s halt lacks {words:?}: {read:?}"
        ),
        None => assert_eq!(read, None, "{name}: {lifetime} is halted"),
    }
}

/// C158, the blocking count's handoff (the cold review of t5 round 1,
/// `roadmap/reviews/t5-r1-5c75e380.md`): a blocking task stays counted
/// until a runtime task has observed its completion, so the park hook
/// leaves the clock while that completion is in hand and not yet
/// observed through. The test holds the observing task after it has seen
/// the closure's end (the daemon's `blocking_probe`), with a sleeper
/// parked on the clock, and waits for the runtime to park: the hook
/// counts the park as held by a blocking task and moves nothing. Released,
/// the count drops on the runtime, the handle answers the closure's
/// value, and the next park moves the clock to the sleeper. A component
/// test of the hook's rule (`door.daemon()`), on the door's runtime.
#[test]
fn c158_a_blocking_task_holds_the_clock_until_the_runtime_observes_its_end() {
    let clock = Arc::new(ParkingClock::new());
    let origin = clock.now();
    let parking = clock.clone();
    Door::open(
        Server::None,
        clock,
        Arc::new(RecordingCredentialStore::default()),
    )
    .run(|door| async move {
        let clock = parking;
        let daemon = door.daemon().clone();
        let (observed_tx, observed) = tokio::sync::oneshot::channel();
        let (go, go_rx) = tokio::sync::oneshot::channel();
        *daemon.blocking_probe.lock().unwrap() = Some((observed_tx, go_rx));
        let sleeper = {
            let clock = clock.clone();
            tokio::spawn(async move { clock.sleep(Duration::from_secs(10)).await })
        };
        let (release, released) = std::sync::mpsc::channel::<()>();
        let handle = daemon.blocking(move || {
            released.recv().unwrap();
            7
        });
        release.send(()).unwrap();
        tokio::time::timeout(HANG, observed)
            .await
            .expect("the completion was observed on the runtime")
            .unwrap();
        assert_eq!(
            daemon
                .blocking_in_flight
                .load(std::sync::atomic::Ordering::SeqCst),
            1,
            "the closure has returned and the count is still held"
        );

        // Wait, off the runtime, for the runtime's next park.
        let counters = door.counters();
        let before = *counters.lock().unwrap();
        let (parked_tx, parked) = tokio::sync::oneshot::channel();
        std::thread::spawn(move || {
            let ceiling = std::time::Instant::now() + HANG;
            while counters.lock().unwrap().held_by_blocking == before.held_by_blocking {
                assert!(
                    std::time::Instant::now() < ceiling,
                    "the runtime never parked"
                );
                std::thread::yield_now();
            }
            let _ = parked_tx.send(());
        });
        tokio::time::timeout(HANG, parked)
            .await
            .expect("the runtime parked")
            .unwrap();
        let held = *door.counters().lock().unwrap();
        assert_eq!(
            held.advances, before.advances,
            "no advance while the count is held: {held:?}"
        );
        assert_eq!(clock.now(), origin, "the clock stayed");
        assert_eq!(clock.sleepers(), 1, "the sleeper is still parked");

        go.send(()).unwrap();
        let value = tokio::time::timeout(HANG, handle)
            .await
            .expect("the handle answered")
            .unwrap();
        assert_eq!(value, 7);
        assert_eq!(
            daemon
                .blocking_in_flight
                .load(std::sync::atomic::Ordering::SeqCst),
            0
        );
        tokio::time::timeout(HANG, sleeper)
            .await
            .expect("the sleeper woke")
            .unwrap();
        assert_eq!(clock.now() - origin, Duration::from_secs(10));
        door.finish().await;
    });
}

// ---- C158: the clock a door scenario runs on ----

/// C158, the parking clock: each sleeper wakes at its own deadline, in
/// deadline order, never a summed overshoot; `advance` keeps
/// `ManualClock`'s meaning (both faces move); a dropped sleep is not
/// counted among the sleepers.
#[tokio::test]
async fn the_parking_clock_wakes_each_sleeper_at_its_own_deadline() {
    let clock = Arc::new(ParkingClock::new());
    let origin = clock.now();
    let order = Arc::new(Mutex::new(Vec::new()));
    let mut tasks = Vec::new();
    for secs in [3u64, 1, 2] {
        let (clock, order) = (clock.clone(), order.clone());
        tasks.push(tokio::spawn(async move {
            clock.sleep(Duration::from_secs(secs)).await;
            order.lock().unwrap().push((secs, clock.now() - origin));
        }));
    }
    tokio::time::timeout(HANG, async {
        while clock.sleepers() < 3 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("the three sleepers parked");
    {
        let mut cancelled = clock.sleep_until(origin + Duration::from_secs(10));
        assert!(futures_poll_once(cancelled.as_mut()).is_pending());
        assert_eq!(clock.sleepers(), 4);
    }
    assert_eq!(clock.sleepers(), 3, "a dropped sleep is not counted");
    tokio::time::timeout(HANG, async {
        while clock.advance_to_next().is_some() {
            while clock.sleepers() + order.lock().unwrap().len() < 3 {
                tokio::task::yield_now().await;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("every sleeper woke");
    tokio::time::timeout(HANG, async {
        for task in tasks {
            task.await.unwrap();
        }
    })
    .await
    .expect("every sleeper's task ended");
    assert_eq!(
        order.lock().unwrap().as_slice(),
        [
            (1, Duration::from_secs(1)),
            (2, Duration::from_secs(2)),
            (3, Duration::from_secs(3))
        ]
    );
    let wall = clock.wall();
    clock.advance(Duration::from_secs(5));
    assert_eq!(clock.now() - origin, Duration::from_secs(8));
    assert_eq!(clock.wall(), wall + Duration::from_secs(5));
}

/// Poll `future` once with a waker that does nothing.
fn futures_poll_once<F: Future + ?Sized>(future: Pin<&mut F>) -> std::task::Poll<F::Output> {
    let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
    future.poll(&mut cx)
}
