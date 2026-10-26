//! The dead bearer: an access token the provider rejects is refreshed
//! before the next send, and no send carrying it leaves the gate after its
//! 401 lands.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::*;

/// A server that answers by *what was sent*, not by position in a
/// script: a stale bearer gets 401 on any route (on GETs only when
/// `probe_checks_bearer` is false — whether GGG's HEAD checks the
/// bearer is unobserved, so both shapes are driven), the token
/// endpoint rotates to a fresh one, everything else succeeds. Runs
/// until aborted. Scenario tests assert invariants over the journal,
/// so the server must not encode the expected sequence.
async fn bearer_aware_server(
    stale_bearer: &'static str,
    fresh_bearer: &'static str,
    probe_checks_bearer: bool,
) -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        loop {
            let (mut stream, _) = listener.accept().await.unwrap();
            let Some(request) = mockggg::read_request(&mut stream).await else {
                continue;
            };
            let bearer = request
                .headers
                .get("authorization")
                .and_then(|v| v.strip_prefix("Bearer "))
                .unwrap_or("");
            let api_headers = "X-Rate-Limit-Policy: scenario-policy\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 100:1:60\r\nX-Rate-Limit-Account-State: 0:1:0\r\n";
            let (status, headers, body) = if request.path == "/token" {
                let body = json!({
                    "access_token": fresh_bearer,
                    "refresh_token": "rt-rotated",
                    "expires_in": 3600,
                    "username": "scenario-user",
                })
                .to_string();
                (
                    "200 OK",
                    "X-Rate-Limit-Policy: token-request-limit\r\nX-Rate-Limit-Rules: Ip\r\nX-Rate-Limit-Ip: 60:30:30\r\nX-Rate-Limit-Ip-State: 1:30:0\r\n",
                    body,
                )
            } else if bearer == stale_bearer && (probe_checks_bearer || request.method != "HEAD") {
                (
                    "401 Unauthorized",
                    api_headers,
                    r#"{"error":"expired"}"#.to_string(),
                )
            } else if request.method == "HEAD" {
                ("204 No Content", api_headers, String::new())
            } else {
                ("200 OK", api_headers, r#"{"characters":[]}"#.to_string())
            };
            mockggg::respond_with(&mut stream, status, "application/json", headers, &body).await;
        }
    });
    (base, task)
}

/// Seconds since midnight from a journal `ts`; enough to reason about a
/// scenario that starts the day at the manual wall origin.
fn ts_seconds(line: &Value) -> u64 {
    let ts = line["ts"].as_str().expect("ts is a string");
    assert!(ts.starts_with("2000-01-01T"), "manual-clock stamp: {ts}");
    let hms: Vec<u64> = ts[11..19].split(':').map(|p| p.parse().unwrap()).collect();
    hms[0] * 3600 + hms[1] * 60 + hms[2]
}

/// A daemon holding a token the wall clock says is valid, which the
/// provider rejects (C87): one job meets the 401, and the journal
/// after it must read refresh, then sends under the new bearer — never
/// the rejected bearer again. `probe_checks_bearer` picks which send
/// meets the 401: the HEAD probe, or the GET behind a probe that
/// passed. Returns the journal's sends and the two jobs' terminal
/// states.
async fn rejected_access_token_scenario(
    probe_checks_bearer: bool,
) -> (Vec<Value>, (JobInfo, Outcome), (JobInfo, Outcome)) {
    let (base, server) = bearer_aware_server("at-old", "at-new", probe_checks_bearer).await;
    let clock = Arc::new(ManualClock::new());
    let log_path = test_log_path();
    let journal = journal_of(&log_path);
    let rails = Arc::new(Rails::with_config_and_clock(
        RailsConfig {
            journal_path: Some(journal.clone()),
            ..RailsConfig::default()
        },
        clock.clone(),
    ));
    let (daemon, log_path) = test_daemon_scenario_at(
        Provider::mock(&base),
        clock.clone(),
        rails,
        Arc::new(NoopCredentialStore),
        log_path,
    );
    {
        let mut s = daemon.shared.lock().unwrap();
        s.auth.rename("", "scenario-user");
        daemon.install_tokens_locked(
            s.auth.one_mut(),
            auth::TokenResponse {
                access_token: "at-old".into(),
                refresh_token: "rt-old".into(),
                expires_in: 3600,
                username: "scenario-user".into(),
            },
        );
    }
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let first = daemon
        .submit("characters".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let first = wait_terminal(&daemon, first).await;
    // A failed probe closes the route for the cooldown; the second job
    // is submitted once that has passed, so what it does is decided by
    // the session, not by the closed route.
    clock.advance(crate::ratelimit::PROBE_COOLDOWN + Duration::from_secs(1));
    let second = daemon
        .submit("characters".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let second = wait_terminal(&daemon, second).await;
    server.abort();
    let sends = journal_sends(&log_path);
    finish_harness(dispatcher, &log_path);
    (sends, first, second)
}

/// What both shapes must show (C87), past the journal oracles the
/// scenario's teardown ran: exactly one 401 in the whole run (the rejected
/// bearer is never sent again), the first job failed saying so, the
/// second job finished under the refreshed bearer.
fn assert_rejected_bearer_refreshed(
    sends: &[Value],
    first: &(JobInfo, Outcome),
    second: &(JobInfo, Outcome),
) {
    let unauthorized: Vec<usize> = sends
        .iter()
        .enumerate()
        .filter(|(_, l)| l["status"] == 401)
        .map(|(i, _)| i)
        .collect();
    assert_eq!(unauthorized.len(), 1, "exactly one 401: {sends:?}");
    let after = &sends[unauthorized[0] + 1];
    assert!(
        after["method"] == "POST" && after["route"] == "oauth-token",
        "the send after the 401 is the refresh: {sends:?}"
    );
    assert_eq!(first.0.state, JobState::Failed, "{sends:?}");
    let Outcome::Failure { error, .. } = &first.1 else {
        panic!("the job that met the 401 has a failure outcome")
    };
    assert!(
        error.contains("401") && error.contains("rejected") && error.contains("refreshes"),
        "the failure names the rejection: {error}"
    );
    assert_eq!(second.0.state, JobState::Done, "{:?}\n{sends:?}", second.1);
}

/// C87 on the probe: the HEAD carries the rejected bearer and lands
/// 401. Breaker (verified 2026-09-12): drop `reject_access_token` from
/// the probe job's failure arm and the second job's probe re-sends the
/// same bearer — a second 401, and the journal rule fails on it.
#[tokio::test]
async fn rejected_access_token_on_the_probe_is_refreshed_before_the_next_send() {
    let (sends, first, second) = rejected_access_token_scenario(true).await;
    assert_rejected_bearer_refreshed(&sends, &first, &second);
    let methods: Vec<&str> = sends
        .iter()
        .map(|l| l["method"].as_str().unwrap())
        .collect();
    assert_eq!(methods, ["HEAD", "POST", "HEAD", "GET"], "{sends:?}");
}

/// C87 on the GET: the probe passes without checking the bearer and
/// the GET behind it lands 401. Breaker (verified 2026-09-12): drop
/// `reject_access_token` from `api_get` and the second job's GET
/// re-sends the same bearer — a second 401, and the journal rule fails.
#[tokio::test]
async fn rejected_access_token_on_the_get_is_refreshed_before_the_next_send() {
    let (sends, first, second) = rejected_access_token_scenario(false).await;
    assert_rejected_bearer_refreshed(&sends, &first, &second);
    let methods: Vec<&str> = sends
        .iter()
        .map(|l| l["method"].as_str().unwrap())
        .collect();
    assert_eq!(methods, ["HEAD", "GET", "POST", "GET"], "{sends:?}");
}

/// A server for the concurrent shape of C87: HEAD always 204, one
/// policy per path so three routes hold three ordinary permits' worth
/// of contention; every HEAD and GET is recorded on arrival — method,
/// bearer, and whether this server had already *answered* a 401 by
/// then — and a GET carrying the stale bearer is held until
/// `release`, then 401. The token endpoint rotates to the fresh bearer.
async fn bearer_recording_server(
    stale_bearer: &'static str,
    fresh_bearer: &'static str,
) -> (
    String,
    tokio::task::JoinHandle<()>,
    Arc<Mutex<Vec<(String, String, bool)>>>,
    watch::Sender<bool>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let arrivals: Arc<Mutex<Vec<(String, String, bool)>>> = Arc::new(Mutex::new(Vec::new()));
    let answered_401 = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let (release, released) = watch::channel(false);
    let task = {
        let arrivals = arrivals.clone();
        tokio::spawn(async move {
            loop {
                let (mut stream, _) = listener.accept().await.unwrap();
                let arrivals = arrivals.clone();
                let answered_401 = answered_401.clone();
                let mut released = released.clone();
                tokio::spawn(async move {
                    let Some(request) = mockggg::read_request(&mut stream).await else {
                        return;
                    };
                    let bearer = request
                        .headers
                        .get("authorization")
                        .and_then(|v| v.strip_prefix("Bearer "))
                        .unwrap_or("")
                        .to_string();
                    let policy = format!("scenario{}", request.path.replace('/', "-"));
                    let api_headers = format!(
                        "X-Rate-Limit-Policy: {policy}\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 100:1:60\r\nX-Rate-Limit-Account-State: 0:1:0\r\n"
                    );
                    let (status, headers, body) = if request.path == "/token" {
                        (
                            "200 OK",
                            "X-Rate-Limit-Policy: token-request-limit\r\nX-Rate-Limit-Rules: Ip\r\nX-Rate-Limit-Ip: 60:30:30\r\nX-Rate-Limit-Ip-State: 1:30:0\r\n".to_string(),
                            json!({
                                "access_token": fresh_bearer,
                                "refresh_token": "rt-rotated",
                                "expires_in": 3600,
                                "username": "scenario-user",
                            })
                            .to_string(),
                        )
                    } else if request.method == "HEAD" {
                        arrivals.lock().unwrap().push((
                            "HEAD".into(),
                            bearer.clone(),
                            answered_401.load(std::sync::atomic::Ordering::SeqCst),
                        ));
                        ("204 No Content", api_headers, String::new())
                    } else {
                        arrivals.lock().unwrap().push((
                            "GET".into(),
                            bearer.clone(),
                            answered_401.load(std::sync::atomic::Ordering::SeqCst),
                        ));
                        if bearer == stale_bearer {
                            while !*released.borrow_and_update() {
                                if released.changed().await.is_err() {
                                    return;
                                }
                            }
                            answered_401.store(true, std::sync::atomic::Ordering::SeqCst);
                            (
                                "401 Unauthorized",
                                api_headers,
                                r#"{"error":"expired"}"#.to_string(),
                            )
                        } else {
                            ("200 OK", api_headers, r#"{"characters":[]}"#.to_string())
                        }
                    };
                    mockggg::respond_with(&mut stream, status, "application/json", &headers, &body)
                        .await;
                });
            }
        })
    };
    (base, task, arrivals, release)
}

/// C87 under the gate. Three jobs on three routes learn the same valid
/// token, two are admitted (the gate is two wide) and held by the
/// server, the third waits — at the gate with its GET when its route
/// was learned up front, or for the exclusive permit with its HEAD
/// probe when it was not — holding what it learned. The held two are
/// then answered 401. Before 2026-09-12 the third went out with the
/// rejected bearer — the session was cleared after the rejecting
/// send's permit had dropped, and the waiter had its copy. Now the
/// rejection is published under the permit and the waiter reads the
/// session after admission: it finds no token, releases its permit,
/// refreshes, and sends the new one. Returns the server's arrivals
/// (method, bearer, whether a 401 had been answered by then), the
/// journal's sends and the three jobs' outcomes.
async fn rejected_bearer_under_the_gate(
    learn_third_route: bool,
) -> (
    Vec<(String, String, bool)>,
    Vec<Value>,
    Vec<(JobInfo, Outcome)>,
) {
    let (base, server, arrivals, release) = bearer_recording_server("at-old", "at-new").await;
    let clock = Arc::new(ManualClock::new());
    let log_path = test_log_path();
    let journal = journal_of(&log_path);
    let rails = Arc::new(Rails::with_config_and_clock(
        RailsConfig {
            journal_path: Some(journal.clone()),
            ..RailsConfig::default()
        },
        clock.clone(),
    ));
    let (daemon, log_path) = test_daemon_scenario_at(
        Provider::mock(&base),
        clock.clone(),
        rails,
        Arc::new(NoopCredentialStore),
        log_path,
    );
    {
        let mut s = daemon.shared.lock().unwrap();
        s.auth.rename("", "scenario-user");
        daemon.install_tokens_locked(
            s.auth.one_mut(),
            auth::TokenResponse {
                access_token: "at-old".into(),
                refresh_token: "rt-old".into(),
                expires_in: 3600,
                username: "scenario-user".into(),
            },
        );
    }
    // The first two routes are always learned first, so their GETs
    // are what the server holds; the third is learned too, or left
    // for its job's probe to wait on the held GETs draining.
    let mut routes = vec![
        ("character-list", "/character"),
        ("character", "/character/x"),
    ];
    if learn_third_route {
        routes.push(("stash-list", "/stash/Standard"));
    }
    for (route, path) in routes {
        daemon
            .choke
            .head(
                route,
                &format!("{base}{path}"),
                None,
                daemon.choke.now(),
                ALWAYS_WANTED,
            )
            .await
            .unwrap();
    }
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let ids = [
        daemon
            .submit("characters".into(), json!({}), 0, "test".into(), None)
            .unwrap(),
        daemon
            .submit(
                "character".into(),
                json!({"name": "x"}),
                0,
                "test".into(),
                None,
            )
            .unwrap(),
        daemon
            .submit("stashes".into(), json!({}), 0, "test".into(), None)
            .unwrap(),
    ];
    // Two GETs in flight (held), the third send waiting: wait for the
    // two arrivals, then let the scheduler run until nothing else
    // can happen without the server answering.
    let gets = |arrivals: &Vec<(String, String, bool)>| {
        arrivals.iter().filter(|(m, _, _)| m == "GET").count()
    };
    tokio::time::timeout(Duration::from_secs(3), async {
        while gets(&arrivals.lock().unwrap()) < 2 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("two GETs reached the server");
    for _ in 0..200 {
        tokio::task::yield_now().await;
    }
    assert_eq!(
        gets(&arrivals.lock().unwrap()),
        2,
        "the gate is two wide: the third send must be waiting, not on the wire"
    );
    release.send(true).unwrap();
    let mut outcomes = Vec::new();
    for id in ids {
        outcomes.push(wait_terminal(&daemon, id).await);
    }
    server.abort();
    let sends = journal_sends(&log_path);
    finish_harness(dispatcher, &log_path);
    let arrivals = arrivals.lock().unwrap().clone();
    (arrivals, sends, outcomes)
}

/// What both waiting shapes must show, at the server where dispatch
/// order is visible and over the journal: no send carrying the
/// rejected bearer arrives after a 401 has been answered; the two
/// held GETs carried it; the refresh reached the wire; a send under
/// the refreshed bearer followed; the two held jobs failed on their
/// 401 (one of them saying the token was cleared) and the third job
/// never met one.
fn assert_rejected_bearer_not_dispatched(
    arrivals: &[(String, String, bool)],
    sends: &[Value],
    outcomes: &[(JobInfo, Outcome)],
) {
    assert!(
        !arrivals
            .iter()
            .any(|(_, bearer, after_401)| bearer == "at-old" && *after_401),
        "a send carried the rejected bearer after a 401 had been answered: {arrivals:?}"
    );
    assert_eq!(
        arrivals
            .iter()
            .filter(|(m, b, _)| m == "GET" && b == "at-old")
            .count(),
        2,
        "the two held GETs carried the old bearer: {arrivals:?}"
    );
    assert!(
        arrivals
            .iter()
            .any(|(m, b, after)| m == "GET" && b == "at-new" && *after),
        "the waiting job's GET went out under the refreshed bearer: {arrivals:?}"
    );
    assert!(
        sends
            .iter()
            .any(|l| l["method"] == "POST" && l["route"] == "oauth-token"),
        "the refresh reached the wire: {sends:?}"
    );
    // Whatever this server's body does to the third job afterwards is
    // not the point.
    let errors: Vec<&str> = outcomes
        .iter()
        .map(|(_, outcome)| match outcome {
            Outcome::Failure { error, .. } => error.as_str(),
            _ => "",
        })
        .collect();
    assert_eq!(
        errors.iter().filter(|e| e.contains("401")).count(),
        2,
        "the two held jobs failed on their 401: {errors:?}"
    );
    assert!(
        !errors[2].contains("401"),
        "the waiting job never met a 401: {}",
        errors[2]
    );
    assert!(
        errors.iter().any(|e| e.contains("the next send refreshes")),
        "the job whose 401 cleared the token says so: {errors:?}"
    );
}

/// The waiting send is a GET at the gate. Breakers (verified
/// 2026-09-12): make `SessionBearer::token` return `expected`
/// regardless, or drop the permit and yield before the choke's
/// `rejected` call — the third arrival carries the rejected bearer
/// and this fails. A drop with no await before the call passes here,
/// because this runtime is one thread and no waiter runs in that
/// window; the daemon's is not, so the call's place before the
/// permit's drop is held by reading, not by this test.
#[tokio::test]
async fn rejected_access_token_is_not_dispatched_after_its_401_under_the_gate() {
    let (arrivals, sends, outcomes) = rejected_bearer_under_the_gate(true).await;
    assert_rejected_bearer_not_dispatched(&arrivals, &sends, &outcomes);
}

/// The waiting send is the third route's HEAD probe, held for its
/// exclusive permit behind the two GETs. Once they land 401 it reads
/// the cleared session, releases the permit, refreshes and probes
/// under the new bearer: the probe on the wire carries `at-new`, and
/// in the journal it follows the token POST. Breaker (verified
/// 2026-09-12): read the bearer's token before `acquire_head` instead
/// of after — the probe carries the rejected bearer after the 401.
#[tokio::test]
async fn rejected_access_token_is_not_dispatched_by_a_waiting_probe_after_its_401() {
    let (arrivals, sends, outcomes) = rejected_bearer_under_the_gate(false).await;
    assert_rejected_bearer_not_dispatched(&arrivals, &sends, &outcomes);
    assert!(
        arrivals
            .iter()
            .any(|(m, b, after)| m == "HEAD" && b == "at-new" && *after),
        "the waiting probe went out under the refreshed bearer: {arrivals:?}"
    );
    let refresh = sends
        .iter()
        .position(|l| l["method"] == "POST" && l["route"] == "oauth-token")
        .expect("the refresh is journaled");
    let probe = sends
        .iter()
        .position(|l| l["method"] == "HEAD" && l["route"] == "stash-list")
        .expect("the third route's probe is journaled");
    assert!(
        probe > refresh,
        "the waiting probe went out after the refresh, not before: {sends:?}"
    );
}

/// R8 as a scenario (TESTING-NOTES, "the experiment"). The token was
/// issued for 3600 s; the lid closed for 1800 s and then 2000 s passed
/// normally. On the wall it is 3800 s later and the token is dead; on a
/// stopwatch only 2000 s have passed and it looks fine. The invariant is
/// over the journal, not the sequence: no send is answered 401, the
/// refresh happened on the wire, and the whole job finished within a
/// minute of virtual time.
///
/// Breaker (verified 2026-08-24): measure expiry from `self.choke.now()`
/// instead of `wall()` in `install_tokens_locked` / `valid_access_token`
/// and this fails on the 401 assertion.
#[tokio::test]
async fn expired_token_after_laptop_sleep_is_refreshed_before_any_send() {
    let (base, server) = bearer_aware_server("at-old", "at-new", true).await;
    let clock = Arc::new(ManualClock::new());
    let log_path = test_log_path();
    let journal = journal_of(&log_path);
    let rails = Arc::new(Rails::with_config_and_clock(
        RailsConfig {
            journal_path: Some(journal.clone()),
            ..RailsConfig::default()
        },
        clock.clone(),
    ));
    let (daemon, log_path) = test_daemon_scenario_at(
        Provider::mock(&base),
        clock.clone(),
        rails,
        Arc::new(NoopCredentialStore),
        log_path,
    );
    {
        let mut s = daemon.shared.lock().unwrap();
        // Key == username is the map's invariant; the scenario daemon
        // starts with one anonymous session.
        s.auth.rename("", "scenario-user");
        daemon.install_tokens_locked(
            s.auth.one_mut(),
            auth::TokenResponse {
                access_token: "at-old".into(),
                refresh_token: "rt-old".into(),
                expires_in: 3600,
                username: "scenario-user".into(),
            },
        );
    }
    clock.laptop_sleep(Duration::from_secs(1800));
    clock.advance(Duration::from_secs(2000));
    let scenario_start = 3800;

    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = daemon
        .submit("characters".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let (info, _) = wait_terminal(&daemon, id).await;
    server.abort();
    let lines = read_journal(&journal);
    finish_harness(dispatcher, &log_path);
    let header = &lines[0];
    assert_eq!(header["event"], "open");
    assert_eq!(header["clock"], "manual");
    assert_eq!(header["contract"], acquisition_protocol::CONTRACT_REVISION);
    assert_eq!(
        header["daemon"],
        Value::Null,
        "the harness runs from no executable"
    );
    assert_eq!(header["ts"], "2000-01-01T00:00:00.000Z");
    let sends = &lines[1..];

    assert!(
        sends.iter().all(|l| l["status"] != 401),
        "no send may be answered 401: {sends:?}"
    );
    assert_eq!(info.state, JobState::Done, "{sends:?}");
    assert!(
        sends
            .iter()
            .any(|l| l["method"] == "POST" && l["path"] == "/token"),
        "the refresh must reach the wire: {sends:?}"
    );
    for line in sends {
        let at = ts_seconds(line);
        assert!(
            (scenario_start..scenario_start + 60).contains(&at),
            "send outside the scenario's minute: {line}"
        );
    }
}

/// C129 over C87: a 401 that Cloudflare gave in GGG's place (it carries
/// `cf-mitigated`, and no GGG header) is no rejection of the bearer. The
/// block halts the daemon; after `reset-tripwire` the account's next send
/// carries the same token, and no refresh is spent on a token GGG never
/// judged. Breaker: report every 401 to the bearer in `get_bearer` and
/// the next send is a refresh POST, which rotates the refresh token for
/// nothing.
#[tokio::test]
async fn a_cloudflare_401_on_the_get_keeps_the_access_token() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let wire: Arc<Mutex<Vec<(String, String)>>> = Arc::default();
    let server = {
        let wire = wire.clone();
        tokio::spawn(async move {
            let mut gets = 0;
            loop {
                let (mut stream, _) = listener.accept().await.unwrap();
                let Some(request) = mockggg::read_request(&mut stream).await else {
                    continue;
                };
                let bearer = request
                    .headers
                    .get("authorization")
                    .and_then(|v| v.strip_prefix("Bearer "))
                    .unwrap_or("")
                    .to_string();
                wire.lock().unwrap().push((request.method.clone(), bearer));
                let api_headers = "X-Rate-Limit-Policy: scenario-policy\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 100:1:60\r\nX-Rate-Limit-Account-State: 0:1:0\r\n";
                let (status, headers, body) = if request.path == "/token" {
                    let body = json!({
                        "access_token": "at-new",
                        "refresh_token": "rt-rotated",
                        "expires_in": 3600,
                        "username": "scenario-user",
                    })
                    .to_string();
                    (
                        "200 OK",
                        "X-Rate-Limit-Policy: token-request-limit\r\nX-Rate-Limit-Rules: Ip\r\nX-Rate-Limit-Ip: 60:30:30\r\nX-Rate-Limit-Ip-State: 1:30:0\r\n",
                        body,
                    )
                } else if request.method == "HEAD" {
                    ("204 No Content", api_headers, String::new())
                } else {
                    gets += 1;
                    if gets == 1 {
                        (
                            "401 Unauthorized",
                            "cf-mitigated: challenge\r\n",
                            "<!DOCTYPE html><html><head><title>Just a moment...</title></head><body></body></html>".to_string(),
                        )
                    } else {
                        ("200 OK", api_headers, r#"{"characters":[]}"#.to_string())
                    }
                };
                mockggg::respond_with(&mut stream, status, "application/json", headers, &body)
                    .await;
            }
        })
    };
    let clock = Arc::new(ManualClock::new());
    let log_path = test_log_path();
    let journal = journal_of(&log_path);
    let rails = Arc::new(Rails::with_config_and_clock(
        RailsConfig {
            journal_path: Some(journal.clone()),
            ..RailsConfig::default()
        },
        clock.clone(),
    ));
    let (daemon, log_path) = test_daemon_scenario_at(
        Provider::mock(&base),
        clock.clone(),
        rails,
        Arc::new(NoopCredentialStore),
        log_path,
    );
    {
        let mut s = daemon.shared.lock().unwrap();
        s.auth.rename("", "scenario-user");
        daemon.install_tokens_locked(
            s.auth.one_mut(),
            auth::TokenResponse {
                access_token: "at-old".into(),
                refresh_token: "rt-old".into(),
                expires_in: 3600,
                username: "scenario-user".into(),
            },
        );
    }
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let first = daemon
        .submit("characters".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let (info, _) = wait_terminal(&daemon, first).await;
    assert_eq!(info.state, JobState::Failed);
    assert!(daemon.choke.rails().halted().is_some(), "the block halts");
    daemon.reset_rails().unwrap();
    let second = daemon
        .submit("characters".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let (info, outcome) = wait_terminal(&daemon, second).await;
    server.abort();
    let sends = journal_sends(&log_path);
    let wire = wire.lock().unwrap().clone();
    finish_harness(dispatcher, &log_path);
    assert!(
        sends.iter().all(|l| l["method"] != "POST"),
        "no refresh is spent after a block's 401: {sends:?}"
    );
    let gets: Vec<&str> = wire
        .iter()
        .filter(|(method, _)| method == "GET")
        .map(|(_, bearer)| bearer.as_str())
        .collect();
    assert_eq!(gets, ["at-old", "at-old"], "{wire:?}");
    assert_eq!(info.state, JobState::Done, "{outcome:?}\n{sends:?}");
}
