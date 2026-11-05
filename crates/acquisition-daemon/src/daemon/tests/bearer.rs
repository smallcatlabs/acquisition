//! The dead bearer: an access token the provider rejects is refreshed
//! before the next send, and no send carrying it leaves the gate after its
//! 401 lands.
//!
//! C158, a scenario crosses the daemon's frontend boundary only: every
//! scenario here is a door facing a server the scenario builds itself
//! ([`Server::own`]), each as its own doc says: the bearer-aware server
//! answers by the bearer sent and records nothing, the recording server
//! answers by path and bearer and records every request, the Cloudflare
//! server answers its first GET with a 401 by arrival; the session the
//! provider will reject is seeded through the door's sessions with its
//! expiry on the clock's wall, as an install writes it; a job is a
//! client's submit awaited through the protocol; a restart is a second
//! door that restores as a start does ([`DoorBuilder::restore`]). What a
//! scenario reads is the protocol's answers, the journal, its server's
//! record and its own credential store.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::door::*;
use super::harness::*;
use crate::daemon::*;
use crate::transport::{Reply, reply};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::{mpsc, oneshot};

/// The bearer a request carried, without `Bearer `; empty for none.
fn bearer_of(request: &mockggg::HttpRequest) -> String {
    request
        .headers
        .get("authorization")
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or("")
        .to_string()
}

/// The token route's answer to any grant: the fresh bearer, rotated.
fn rotated(fresh_bearer: &str) -> (&'static str, String, String) {
    (
        "200 OK",
        TOKEN_HEADERS.to_string(),
        token_body(fresh_bearer, "rt-rotated", "scenario-user"),
    )
}

/// An API answer's rate headers under `policy`.
fn api_headers(policy: &str) -> String {
    format!(
        "X-Rate-Limit-Policy: {policy}\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 100:1:60\r\nX-Rate-Limit-Account-State: 0:1:0\r\n"
    )
}

/// A server that answers by *what was sent*, not by position in a
/// script: a stale bearer gets 401 on any route (on GETs only when
/// `probe_checks_bearer` is false — whether GGG's HEAD checks the
/// bearer is unobserved, so both shapes are driven), the token
/// endpoint rotates to a fresh one, everything else succeeds. The
/// scenario's own transport ([`Server::own`]). Scenario tests assert
/// invariants over the journal, so the server must not encode the
/// expected sequence.
fn bearer_aware_server(
    stale_bearer: &'static str,
    fresh_bearer: &'static str,
    probe_checks_bearer: bool,
) -> Server {
    Server::own(move |request: mockggg::HttpRequest| {
        let bearer = bearer_of(&request);
        let api = api_headers("scenario-policy");
        let (status, headers, body) = if request.path == "/token" {
            rotated(fresh_bearer)
        } else if bearer == stale_bearer && (probe_checks_bearer || request.method != "HEAD") {
            (
                "401 Unauthorized",
                api,
                r#"{"error":"expired"}"#.to_string(),
            )
        } else if request.method == "HEAD" {
            ("204 No Content", api, String::new())
        } else {
            ("200 OK", api, r#"{"characters":[]}"#.to_string())
        };
        let answer = reply(status, &headers, &body);
        async move { answer }
    })
}

/// `scenario-user` holding `at-old`, issued an hour before it expires on
/// `clock`'s wall (as an install writes the expiry), beside `rt-old`.
fn stale_session(clock: &ParkingClock) -> Sessions {
    let mut sessions = Sessions::with(AuthSession {
        access_token: Some("at-old".into()),
        access_expires_at: Some(clock.wall() + Duration::from_secs(3600)),
        refresh_token: Some("rt-old".into()),
        keyring: "ok".into(),
        ..AuthSession::default()
    });
    sessions.rename("", "scenario-user");
    sessions
}

/// Seconds since midnight from a journal `ts`; enough to reason about a
/// scenario that starts the day at the manual wall origin.
fn ts_seconds(line: &Value) -> u64 {
    let ts = line["ts"].as_str().expect("ts is a string");
    assert!(ts.starts_with("2000-01-01T"), "manual-clock stamp: {ts}");
    let hms: Vec<u64> = ts[11..19].split(':').map(|p| p.parse().unwrap()).collect();
    hms[0] * 3600 + hms[1] * 60 + hms[2]
}

/// A client's submit of `kind` with `params`, as a frontend sends it.
async fn submit_with(door: &mut Door, kind: &str, params: Value) -> JobId {
    let submit = Request::Submit {
        kind: kind.into(),
        params,
        priority: 0,
        submitted_by: "test".into(),
        account: None,
    };
    match door.request(submit).await {
        Response::Submitted { id } => id,
        other => panic!("a submit of {kind} answered {other:?}"),
    }
}

/// A client's `characters` job, submitted and awaited to its end.
async fn characters_job(door: &mut Door) -> (JobInfo, Outcome) {
    let id = door.submit("characters", None).await;
    door.await_terminal(id).await
}

/// The methods of a journal's sends, in order.
fn methods_of(sends: &[Value]) -> Vec<&str> {
    sends
        .iter()
        .map(|l| l["method"].as_str().unwrap())
        .collect()
}

/// A daemon holding a token the wall clock says is valid, which the
/// provider rejects (C87): one job meets the 401, and the journal
/// after it must read refresh, then sends under the new bearer — never
/// the rejected bearer again. `probe_checks_bearer` picks which send
/// meets the 401: the HEAD probe, or the GET behind a probe that
/// passed; the journal's methods are `methods`.
fn rejected_access_token_scenario(probe_checks_bearer: bool, methods: [&str; 4]) {
    let clock = Arc::new(ParkingClock::new());
    let sessions = stale_session(&clock);
    Door::open(
        bearer_aware_server("at-old", "at-new", probe_checks_bearer),
        clock,
        Arc::new(NoopCredentialStore),
    )
    .sessions(sessions)
    .run(|mut door| async move {
        let first = characters_job(&mut door).await;
        // The degraded route's cooldown (`PROBE_COOLDOWN`): a failed probe
        // closes the route that long, and nothing parks on its end, so the
        // hand crosses it; the second job is then decided by the session,
        // not by the closed route. Where the GET met the 401 the probe
        // passed, and the move crosses nothing the second job meets.
        door.advance(crate::ratelimit::PROBE_COOLDOWN + Duration::from_secs(1));
        let second = characters_job(&mut door).await;
        let sends = door.journal();
        assert_rejected_bearer_refreshed(&sends, &first, &second);
        assert_eq!(methods_of(&sends), methods, "{sends:?}");
        door.finish().await;
    });
}

/// What both shapes must show (C87), past the journal oracles the
/// scenario's finish runs: exactly one 401 in the whole run (the rejected
/// bearer is never sent again), the send after it the refresh, the first
/// job failed saying so, the second job finished under the refreshed
/// bearer.
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
#[test]
fn rejected_access_token_on_the_probe_is_refreshed_before_the_next_send() {
    rejected_access_token_scenario(true, ["HEAD", "POST", "HEAD", "GET"]);
}

/// C87 on the GET: the probe passes without checking the bearer and
/// the GET behind it lands 401. Breaker (verified 2026-09-12): drop
/// `reject_access_token` from `api_get` and the second job's GET
/// re-sends the same bearer — a second 401, and the journal rule fails.
#[test]
fn rejected_access_token_on_the_get_is_refreshed_before_the_next_send() {
    rejected_access_token_scenario(false, ["HEAD", "GET", "POST", "GET"]);
}

/// One arrival at [`bearer_recording_server`] in its held phase: the
/// method, the bearer, and whether the server had already *answered* a
/// 401 by then.
type Arrival = (String, String, bool);

/// What the scenario holds of [`bearer_recording_server`]: its arrivals
/// in the held phase, the switch into that phase, a message per held GET
/// as it arrives (the arrival barrier) and the release of the held GETs.
struct Recording {
    arrivals: Arc<Mutex<Vec<Arrival>>>,
    holding: Arc<AtomicBool>,
    held: mpsc::UnboundedReceiver<()>,
    release: watch::Sender<bool>,
}

/// A server for the concurrent shape of C87: HEAD always 204, one
/// policy per path so three routes hold three ordinary permits' worth
/// of contention, every GET answered. Until `holding` is set it records
/// nothing and holds nothing: the jobs that learn the routes run
/// through it. Once set, every HEAD and GET is recorded on arrival —
/// method, bearer, and whether this server had already *answered* a 401
/// by then — and a GET carrying the stale bearer is announced on `held`
/// and held until `release`, then answered 401. The token endpoint
/// rotates to the fresh bearer. The scenario's own transport; its halves
/// come back through `recording`.
fn bearer_recording_server(
    stale_bearer: &'static str,
    fresh_bearer: &'static str,
    recording: oneshot::Sender<Recording>,
) -> Server {
    let arrivals: Arc<Mutex<Vec<Arrival>>> = Arc::default();
    let holding = Arc::new(AtomicBool::new(false));
    let (held_tx, held) = mpsc::unbounded_channel();
    let answered_401 = Arc::new(AtomicBool::new(false));
    let (release, released) = watch::channel(false);
    let _ = recording.send(Recording {
        arrivals: arrivals.clone(),
        holding: holding.clone(),
        held,
        release,
    });
    Server::own(move |request: mockggg::HttpRequest| {
        let bearer = bearer_of(&request);
        let api = api_headers(&format!("scenario{}", request.path.replace('/', "-")));
        let held_phase = holding.load(Ordering::SeqCst);
        if request.path != "/token" && held_phase {
            arrivals.lock().unwrap().push((
                request.method.clone(),
                bearer.clone(),
                answered_401.load(Ordering::SeqCst),
            ));
        }
        let held = request.path != "/token"
            && request.method != "HEAD"
            && held_phase
            && bearer == stale_bearer;
        if held {
            let _ = held_tx.send(());
        }
        let (answered_401, mut released) = (answered_401.clone(), released.clone());
        let method = request.method.clone();
        let path = request.path.clone();
        async move {
            let (status, headers, body) = if path == "/token" {
                rotated(fresh_bearer)
            } else if method == "HEAD" {
                ("204 No Content", api, String::new())
            } else if held {
                while !*released.borrow_and_update() {
                    if released.changed().await.is_err() {
                        return Reply::Failed("the held request's release was dropped".into());
                    }
                }
                answered_401.store(true, Ordering::SeqCst);
                (
                    "401 Unauthorized",
                    api,
                    r#"{"error":"expired"}"#.to_string(),
                )
            } else {
                (
                    "200 OK",
                    api,
                    r#"{"characters":[],"stashes":[]}"#.to_string(),
                )
            };
            reply(status, &headers, &body)
        }
    })
}

/// C87 under the gate. Three jobs on three routes hold the same valid
/// token; two are admitted (the gate is two wide) and held by the
/// server, the third waits — at the gate with its GET when its route
/// was learned up front, or for the exclusive permit with its HEAD
/// probe when it was not — holding what it learned. The held two are
/// then answered 401. Before 2026-09-12 the third went out with the
/// rejected bearer — the session was cleared after the rejecting
/// send's permit had dropped, and the waiter had its copy. Now the
/// rejection is published under the permit and the waiter reads the
/// session after admission: it finds no token, releases its permit,
/// refreshes, and sends the new one. The routes are learned by jobs of
/// their own, answered before the server holds anything. Returns the
/// server's arrivals in the held phase, the journal's sends from it and
/// the three jobs' outcomes.
fn rejected_bearer_under_the_gate(
    learn_third_route: bool,
    assert: impl FnOnce(&[Arrival], &[Value], &[(JobInfo, Outcome)]) + 'static,
) {
    let clock = Arc::new(ParkingClock::new());
    let sessions = stale_session(&clock);
    let (recording, recorded) = oneshot::channel();
    Door::open(
        bearer_recording_server("at-old", "at-new", recording),
        clock,
        Arc::new(NoopCredentialStore),
    )
    .sessions(sessions)
    .run(|mut door| async move {
        let Recording {
            arrivals,
            holding,
            mut held,
            release,
        } = recorded
            .await
            .expect("the door built the scenario's server");
        let jobs = [
            ("characters", json!({})),
            ("character", json!({"name": "x"})),
            ("stashes", json!({})),
        ];
        // The first two routes are always learned first, so their GETs
        // are what the server holds; the third is learned too, or left
        // for its job's probe to wait on the held GETs draining. A route
        // is learned by a job of its own, its probe and its GET answered.
        let learned = if learn_third_route { 3 } else { 2 };
        for (kind, params) in &jobs[..learned] {
            let id = submit_with(&mut door, kind, params.clone()).await;
            let (job, outcome) = door.await_terminal(id).await;
            assert_eq!(
                job.state,
                JobState::Done,
                "{kind} learns its route: {outcome:?}"
            );
        }
        let before = door.journal().len();

        holding.store(true, Ordering::SeqCst);
        let mut ids = Vec::new();
        for (kind, params) in jobs {
            ids.push(submit_with(&mut door, kind, params).await);
        }
        // The arrival barrier: two GETs held at the server; then the
        // runtime quiet, every task parked on what it waits for (the
        // third send at the gate or its probe at the exclusive permit).
        for _ in 0..2 {
            door.await_behind_own_holds("awaiting two GETs held at the server", held.recv())
                .await
                .expect("the server announces each held GET");
        }
        door.quiet().await;
        let gets = |arrivals: &[Arrival]| arrivals.iter().filter(|(m, _, _)| m == "GET").count();
        assert_eq!(
            gets(&arrivals.lock().unwrap()),
            2,
            "the gate is two wide: the third send must be waiting, not on the wire"
        );
        release.send(true).unwrap();
        let mut outcomes = Vec::new();
        for id in ids {
            outcomes.push(door.await_terminal(id).await);
        }
        let sends = door.journal().split_off(before);
        let arrivals = arrivals.lock().unwrap().clone();
        assert(&arrivals, &sends, &outcomes);
        door.finish().await;
    });
}

/// What both waiting shapes must show, at the server where dispatch
/// order is visible and over the journal: no send carrying the
/// rejected bearer arrives after a 401 has been answered; the two
/// held GETs carried it; the refresh reached the wire; a send under
/// the refreshed bearer followed; the two held jobs failed on their
/// 401 (one of them saying the token was cleared) and the third job
/// never met one.
fn assert_rejected_bearer_not_dispatched(
    arrivals: &[Arrival],
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
#[test]
fn rejected_access_token_is_not_dispatched_after_its_401_under_the_gate() {
    rejected_bearer_under_the_gate(true, assert_rejected_bearer_not_dispatched);
}

/// The waiting send is the third route's HEAD probe, held for its
/// exclusive permit behind the two GETs. Once they land 401 it reads
/// the cleared session, releases the permit, refreshes and probes
/// under the new bearer: the probe on the wire carries `at-new`, and
/// in the journal it follows the token POST. Breaker (verified
/// 2026-09-12): read the bearer's token before `acquire_head` instead
/// of after — the probe carries the rejected bearer after the 401.
#[test]
fn rejected_access_token_is_not_dispatched_by_a_waiting_probe_after_its_401() {
    rejected_bearer_under_the_gate(false, |arrivals, sends, outcomes| {
        assert_rejected_bearer_not_dispatched(arrivals, sends, outcomes);
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
            .position(|l| {
                l["method"] == "HEAD"
                    && crate::ratelimit::split_endpoint_key(l["route"].as_str().unwrap()).0
                        == "stash-list"
            })
            .expect("the third route's probe is journaled");
        assert!(
            probe > refresh,
            "the waiting probe went out after the refresh, not before: {sends:?}"
        );
    });
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
#[test]
fn expired_token_after_laptop_sleep_is_refreshed_before_any_send() {
    let clock = Arc::new(ParkingClock::new());
    let sessions = stale_session(&clock);
    Door::open(
        bearer_aware_server("at-old", "at-new", true),
        clock,
        Arc::new(NoopCredentialStore),
    )
    .sessions(sessions)
    .run(|mut door| async move {
        // The access token's expiry on the wall (an hour past the
        // origin): the lid closes 1800 s, the wall moving alone, then
        // 2000 s pass on both faces. The wall is 3800 s on, past the
        // expiry; the monotonic face 2000 s. Nothing parks on the expiry.
        door.laptop_sleep(Duration::from_secs(1800));
        door.advance(Duration::from_secs(2000));
        let scenario_start = 3800;

        let (info, _) = characters_job(&mut door).await;
        let text = door.journal_text();
        let header: Value =
            serde_json::from_str(text.lines().next().expect("the journal's header"))
                .expect("the header is JSON");
        assert_eq!(header["event"], "open");
        assert_eq!(header["clock"], "manual");
        assert_eq!(header["contract"], acquisition_protocol::CONTRACT_REVISION);
        assert_eq!(
            header["daemon"],
            Value::Null,
            "the harness runs from no executable"
        );
        assert_eq!(header["ts"], "2000-01-01T00:00:00.000Z");
        let sends = door.journal();

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
        for line in &sends {
            let at = ts_seconds(line);
            assert!(
                (scenario_start..scenario_start + 60).contains(&at),
                "send outside the scenario's minute: {line}"
            );
        }
        door.finish().await;
    });
}

/// A server whose first GET is Cloudflare's 401 in GGG's place (it
/// carries `cf-mitigated`, and no GGG header) and every later GET a
/// 200; HEAD 204, the token endpoint rotating. It records each request's
/// method and bearer into `wire`. The scenario's own transport.
fn cloudflare_401_server(wire: Arc<Mutex<Vec<(String, String)>>>) -> Server {
    let gets = Arc::new(Mutex::new(0));
    Server::own(move |request: mockggg::HttpRequest| {
        wire.lock()
            .unwrap()
            .push((request.method.clone(), bearer_of(&request)));
        let api = api_headers("scenario-policy");
        let (status, headers, body) = if request.path == "/token" {
            rotated("at-new")
        } else if request.method == "HEAD" {
            ("204 No Content", api, String::new())
        } else {
            let mut gets = gets.lock().unwrap();
            *gets += 1;
            if *gets == 1 {
                (
                    "401 Unauthorized",
                    "cf-mitigated: challenge\r\n".to_string(),
                    "<!DOCTYPE html><html><head><title>Just a moment...</title></head><body></body></html>".to_string(),
                )
            } else {
                ("200 OK", api, r#"{"characters":[]}"#.to_string())
            }
        };
        let answer = reply(status, &headers, &body);
        async move { answer }
    })
}

/// C129 over C87: a 401 that Cloudflare gave in GGG's place (it carries
/// `cf-mitigated`, and no GGG header) is no rejection of the bearer. The
/// block halts the daemon (`DaemonStatus.rails.halted`); after
/// `ResetTripwire` the account's next send carries the same token, and
/// no refresh is spent on a token GGG never judged. Breaker: report every
/// 401 to the bearer in `get_bearer` and the next send is a refresh POST,
/// which rotates the refresh token for nothing.
#[test]
fn a_cloudflare_401_on_the_get_keeps_the_access_token() {
    let clock = Arc::new(ParkingClock::new());
    let sessions = stale_session(&clock);
    let wire: Arc<Mutex<Vec<(String, String)>>> = Arc::default();
    let recorded = wire.clone();
    Door::open(
        cloudflare_401_server(recorded),
        clock,
        Arc::new(NoopCredentialStore),
    )
    .sessions(sessions)
    .run(|mut door| async move {
        let (info, _) = characters_job(&mut door).await;
        assert_eq!(info.state, JobState::Failed);
        let halted = match door.request(Request::DaemonStatus {}).await {
            Response::DaemonStatus { rails, .. } => rails.halted,
            other => panic!("the daemon's status reads {other:?}"),
        };
        assert!(halted.is_some(), "the block halts");
        let reset = door.request(Request::ResetTripwire {}).await;
        assert!(matches!(reset, Response::Ack { .. }), "{reset:?}");
        let (info, outcome) = characters_job(&mut door).await;
        let sends = door.journal();
        assert!(
            sends.iter().all(|l| l["method"] != "POST"),
            "no refresh is spent after a block's 401: {sends:?}"
        );
        let wire = wire.lock().unwrap().clone();
        let gets: Vec<&str> = wire
            .iter()
            .filter(|(method, _)| method == "GET")
            .map(|(_, bearer)| bearer.as_str())
            .collect();
        assert_eq!(gets, ["at-old", "at-old"], "{wire:?}");
        assert_eq!(info.state, JobState::Done, "{outcome:?}\n{sends:?}");
        door.finish().await;
    });
}

/// One row of the restart table: which send meets the 401, and whether
/// the index cannot be read while it lands.
struct Restart {
    /// The test the row replaced, by name.
    name: &'static str,
    probe_checks_bearer: bool,
    frozen_index: bool,
}

/// C87 across a restart, with C135: a restored access token the
/// provider rejects is cleared from the keyring entry as from the
/// session, so the next lifetime restores the refresh token alone and
/// refreshes before any send; the rejected bearer is never sent again.
/// Two doors in sequence on one store directory and one credential
/// store, each restoring as a start does. The first restores `at-kept`,
/// unexpired, and its first job meets the 401 (on the probe or the GET,
/// by `probe_checks_bearer`): its journal holds exactly that one 401,
/// and the store's entry keeps `rt-kept` with no access token. The
/// second's journal reads the refresh first, then the probe and the
/// GET, and no 401. With `frozen_index`, the index cannot be read until
/// the first lifetime is done (#110): bytes that are no index stand in
/// `accounts.json` while the 401 lands, and the index is put back for
/// the second; the index-first hold guards a name, and this rewrite adds
/// none.
#[test]
fn a_token_rejected_is_not_restored_after_a_restart() {
    let rows = [
        Restart {
            name: "a_token_rejected_on_the_probe_is_not_restored_after_a_restart",
            probe_checks_bearer: true,
            frozen_index: false,
        },
        Restart {
            name: "a_token_rejected_on_the_get_is_not_restored_after_a_restart",
            probe_checks_bearer: false,
            frozen_index: false,
        },
        Restart {
            name: "a_token_rejected_under_an_unreadable_index_is_not_restored_after_a_restart",
            probe_checks_bearer: false,
            frozen_index: true,
        },
    ];
    for Restart {
        name,
        probe_checks_bearer,
        frozen_index,
    } in rows
    {
        let clock = Arc::new(ParkingClock::new());
        let store_dir = TestPath::dir();
        Index::load(&store_dir)
            .unwrap()
            .record_login("scenario-user", "u-1", true, 1, &[])
            .unwrap();
        let index_path = store_dir.join("accounts.json");
        let index_bytes = std::fs::read(&index_path).unwrap();
        let kept = auth::KeyringEntry::new(
            "rt-kept".into(),
            Some(("at-kept".into(), clock.wall() + Duration::from_secs(3600))),
        );
        let store = Arc::new(RecordingCredentialStore::default());
        store.entries.lock().unwrap().insert(
            "scenario-user".into(),
            kept.to_secret().exposed_for_test().to_string(),
        );
        let lifetime = |clock: &Arc<ParkingClock>| {
            Door::open(
                bearer_aware_server("at-kept", "at-new", probe_checks_bearer),
                clock.clone(),
                store.clone(),
            )
            .store_dir(store_dir.to_path_buf())
            .restore()
        };

        let (read, wall, dir) = (store.clone(), clock.clone(), store_dir.to_path_buf());
        lifetime(&clock).run(|mut door| async move {
            if frozen_index {
                std::fs::write(dir.join("accounts.json"), b"not an index").unwrap();
                assert!(
                    Index::load(&dir).is_err(),
                    "{name}: the index is unreadable"
                );
            }
            let (info, _) = characters_job(&mut door).await;
            assert_eq!(info.state, JobState::Failed, "{name}: the job met the 401");
            let entry = auth::KeyringEntry::from_secret(
                &read.entries.lock().unwrap()["scenario-user"]
                    .as_str()
                    .into(),
                wall.wall(),
            );
            assert_eq!(entry.refresh_token, "rt-kept", "{name}");
            assert_eq!(
                entry.access_token, None,
                "{name}: the rejected token left the entry"
            );
            let sends = door.journal();
            assert_eq!(
                sends.iter().filter(|l| l["status"] == 401).count(),
                1,
                "{name}: {sends:?}"
            );
            door.finish().await;
        });
        std::fs::write(&index_path, &index_bytes).unwrap();

        lifetime(&clock).run(|mut door| async move {
            // What the job files is not this scenario's: the sends are.
            characters_job(&mut door).await;
            let sends = door.journal();
            assert_eq!(
                methods_of(&sends),
                ["POST", "HEAD", "GET"],
                "{name}: the restarted daemon refreshes before any send: {sends:?}"
            );
            assert_eq!(sends[0]["route"], "oauth-token", "{name}: {sends:?}");
            assert!(
                sends.iter().all(|l| l["status"] != 401),
                "{name}: the rejected bearer is never sent again: {sends:?}"
            );
            door.finish().await;
        });
    }
}
