//! The rails: the tripwire, the ceiling and the halted refresh, each a
//! scenario through the door (C158). A halt's bare cause is read through
//! `DaemonStatus.rails.halted`, its full words through a quote's `halted`
//! or the job's failure; a parked job through `Status` (Waiting, no retry
//! spent); "nothing picked" through `Door::settle`, which resolves only
//! at a park where nothing can run, so a dispatcher re-picking a turned
//! away job would never let it resolve (`Door::quiet` where a send is
//! held); the queue on disk through the store's observer read once the
//! door's run has returned and no daemon answers (C45).
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::door::*;
use super::harness::*;
use crate::daemon::*;
use acquisition_protocol::status::RailsStatus;
use acquisition_store::jobs::daemon_db_path;

// ---- L0 live-test rails (`rails.rs`) ------------------------------------

/// `DaemonStatus`'s rails, as a frontend reads them: `halted` is the
/// halt's bare cause.
async fn rails_status(door: &mut Door) -> RailsStatus {
    match door.request(Request::DaemonStatus {}).await {
        Response::DaemonStatus { rails, .. } => rails,
        other => panic!("the daemon's status answered {other:?}"),
    }
}

/// The halt's full words, as a quote carries them (`Quote.halted`): an
/// empty quote with the account omitted, on a daemon with one session.
async fn quote_halted(door: &mut Door) -> Option<String> {
    let quote = Request::Quote {
        jobs: vec![],
        account: None,
    };
    match door.request(quote).await {
        Response::Quote { quote } => quote.halted,
        other => panic!("the quote answered {other:?}"),
    }
}

/// `Dashboard`'s errors, newest first.
async fn dashboard_errors(door: &mut Door) -> Vec<String> {
    match door.request(Request::Dashboard {}).await {
        Response::Dashboard { errors, .. } => errors.into_iter().map(|e| e.message).collect(),
        other => panic!("the dashboard answered {other:?}"),
    }
}

/// Job `id` as `Status` reads it.
async fn status(door: &mut Door, id: JobId) -> JobInfo {
    match door.request(Request::Status { id }).await {
        Response::Status { job } => job,
        other => panic!("job {id}'s status answered {other:?}"),
    }
}

/// `ResetTripwire`, acknowledged.
async fn reset(door: &mut Door) {
    match door.request(Request::ResetTripwire {}).await {
        Response::Ack { .. } => {}
        other => panic!("the reset answered {other:?}"),
    }
}

/// `AuthCheck` for the sole session, refused: the refusal's kind and
/// words.
async fn auth_check_refused(door: &mut Door) -> (ErrorKind, String) {
    match door.request(Request::AuthCheck { account: None }).await {
        Response::Error { kind, message } => (kind, message),
        other => panic!("the auth check answered {other:?}"),
    }
}

/// C25: job `id` parked, as `Status` reads it: waiting, no retry spent.
async fn assert_waiting(door: &mut Door, id: JobId, what: &str) {
    let job = status(door, id).await;
    assert_eq!(job.state, JobState::Waiting, "{what}: job {id}");
    assert_eq!(job.retries, 0, "{what}: job {id}: a halt spends no retry");
}

/// C25 on disk: job `id`'s row waiting, no retry spent, no outcome, as
/// an observer reads the ledger under `dir` once the run is over.
fn assert_waiting_on_disk(dir: &std::path::Path, id: JobId, what: &str) {
    let row = on_disk(dir, id).unwrap_or_else(|| panic!("{what}: job {id}'s row"));
    assert_eq!(row.job.state, "waiting", "{what}: job {id} on disk");
    assert_eq!(row.job.retries, 0, "{what}: job {id} on disk");
    assert_eq!(row.outcome, None, "{what}: job {id} on disk");
}

/// The rails' config with a ceiling of `max_sends`.
fn ceiling(max_sends: u64) -> RailsConfig {
    RailsConfig {
        max_sends: Some(max_sends),
        ..RailsConfig::default()
    }
}

/// A door on `script` with the rails `config`, its clock and credential
/// store fresh.
fn script_door(script: Vec<ScriptedResponse>, config: RailsConfig) -> DoorBuilder {
    Door::open(
        Server::Script(script),
        Arc::new(ParkingClock::new()),
        Arc::new(RecordingCredentialStore::default()),
    )
    .rails(config)
}

/// C25: a trip leaves queued jobs waiting until the reset. The HEAD
/// establishes the route, the first GET lands a 429 under the tripwire
/// (tripping, the job re-queued with its retry spent); a second job is
/// queued, and once the runtime has settled nothing else reached the
/// wire and both wait. The trip is announced once in the error ring.
/// The reset wakes the queue; the limiter's own hold from the 429 still
/// applies to the next GET, and both jobs go out.
#[test]
fn tripped_daemon_leaves_queued_jobs_waiting_until_reset() {
    let script = vec![
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse::full("GET", "429 Too Many Requests", Some(0), "{}"),
        ScriptedResponse::full("GET", "200 OK", None, r#"{"ok":true}"#),
        ScriptedResponse::full("GET", "200 OK", None, r#"{"ok":true}"#),
    ];
    script_door(script, tripwire_config()).run(|mut door| async move {
        let first = door.submit("fetch", None).await;
        let job = door
            .await_job(first, |job| {
                job.retries == 1 && job.state == JobState::Waiting
            })
            .await;
        assert_eq!(
            job.state,
            JobState::Waiting,
            "re-queued by the 429, then held"
        );
        let halted = rails_status(&mut door).await.halted;
        assert!(
            halted
                .as_deref()
                .is_some_and(|h| h.contains("429 on GET /fetch")),
            "{halted:?}"
        );
        let second = door.submit("fetch", None).await;
        door.settle().await;
        assert_eq!(
            door.journal_methods(),
            ["HEAD", "GET"],
            "a tripped daemon sends nothing"
        );
        for id in [first, second] {
            let job = status(&mut door, id).await;
            assert_eq!(job.state, JobState::Waiting, "job {id} waits out the halt");
        }
        let announced = dashboard_errors(&mut door)
            .await
            .into_iter()
            .filter(|m| m.contains("LIVE-TEST RAILS TRIPPED"))
            .count();
        assert_eq!(announced, 1, "the trip is announced once in the error ring");

        reset(&mut door).await;
        for id in [first, second] {
            let (job, outcome) = door.await_terminal(id).await;
            assert_eq!(job.state, JobState::Done, "job {id}: {outcome:?}");
        }
        let sends: Vec<(String, u64)> = door
            .journal()
            .iter()
            .map(|send| {
                (
                    send["method"].as_str().unwrap().to_string(),
                    send["wait_ms"].as_u64().unwrap(),
                )
            })
            .collect();
        assert_eq!(
            sends,
            [
                ("HEAD".to_string(), 0),
                ("GET".to_string(), 0),
                ("GET".to_string(), full_hold_ms()),
                ("GET".to_string(), 0),
            ],
            "the 429's hold paces the first GET after the reset"
        );
        door.finish().await;
    });
}

/// C25: the ceiling halts after n sends and holds the queue. A ceiling
/// of two is met exactly by a job's HEAD and GET; a second job then
/// waits, nothing more is sent, and the rails count two sends.
#[test]
fn the_ceiling_halts_after_n_sends_and_holds_the_queue() {
    let script = vec![
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse::full("GET", "200 OK", None, r#"{"ok":true}"#),
    ];
    script_door(script, ceiling(2)).run(|mut door| async move {
        let first = door.submit("fetch", None).await;
        let (job, outcome) = door.await_terminal(first).await;
        assert_eq!(
            job.state,
            JobState::Done,
            "HEAD + GET reach the ceiling exactly: {outcome:?}"
        );
        let halted = rails_status(&mut door).await.halted;
        assert!(
            halted
                .as_deref()
                .is_some_and(|h| h.contains("ceiling: 2 of 2")),
            "{halted:?}"
        );
        let second = door.submit("fetch", None).await;
        door.settle().await;
        let job = status(&mut door, second).await;
        assert_eq!(
            job.state,
            JobState::Waiting,
            "a ceiling halt holds the queue too"
        );
        assert_eq!(rails_status(&mut door).await.sends, 2);
        assert_eq!(door.journal_methods(), ["HEAD", "GET"]);
        door.finish().await;
    });
}

/// C25 and C129: a halted daemon refuses a refresh before any send. A
/// job's probe is answered by GGG's 429 (it names its policy: a ladder
/// trip under the tripwire, not a block); then `AuthCheck`, whose
/// session's refresh is due, is refused in the halt's words, an
/// upstream refusal and not a dead session, and nothing more reaches
/// the wire: no token `POST`.
#[test]
fn halted_daemon_refuses_refresh_before_any_send() {
    let script = vec![ScriptedResponse::full(
        "HEAD",
        "429 Too Many Requests",
        Some(0),
        "",
    )];
    script_door(script, tripwire_config())
        .sessions(logged_in_sessions())
        .run(|mut door| async move {
            door.submit("fetch", None).await;
            door.settle().await;
            let halted = rails_status(&mut door).await.halted;
            assert!(
                halted.as_deref().is_some_and(|h| h.contains("429")),
                "the probe's 429 tripped the rails: {halted:?}"
            );
            let (kind, message) = auth_check_refused(&mut door).await;
            assert!(message.contains("halted by live-test rails"), "{message}");
            assert_eq!(
                kind,
                ErrorKind::Upstream,
                "a halted refresh is not a dead session: {message}"
            );
            assert_eq!(door.journal_methods(), ["HEAD"], "no refresh was sent");
            door.finish().await;
        });
}

/// One row of the ceiling's table (C25: a halt that lands after the pick
/// turns the job away; nothing fails for want of a send): the ceiling,
/// the refresh-due sessions and the accounts whose `profile` jobs are
/// submitted, the token answers the script holds, then the words the
/// halt's bare cause carries and the wire. Named for the test it
/// replaced.
struct Ceiling {
    name: &'static str,
    max_sends: u64,
    sessions: fn() -> Sessions,
    accounts: &'static [&'static str],
    script: Vec<ScriptedResponse>,
    halted: &'static str,
    wire: &'static [&'static str],
}

/// C25, the ceiling's table: each row's jobs are turned away and wait,
/// no retry spent, as `Status` reads them once the runtime has settled
/// (a turned-away job is not picked again at once: a re-pick would
/// never let it settle) and as an observer reads the ledger after the
/// run; the halt's bare cause carries the row's words, and the wire and
/// the journal hold the row's sends.
///
/// - A ceiling of one met by the job's own refresh `POST`: its `GET` is
///   then refused by the halt.
/// - Two accounts, each with a refresh due, picked together: the token
///   route admits one `POST` at a time; the first meets the ceiling of
///   one, and the second account's refresh, already running, is refused
///   by the halt. That job is turned away as the first is.
/// - A ceiling of none: the daemon is halted from its start, the pick
///   passes the job over, and it waits with nothing sent (the pick's
///   arm, not the turn-away after it).
#[test]
fn the_ceiling_turns_jobs_away_after_and_at_the_pick() {
    let rows = [
        Ceiling {
            name: "a_halt_after_the_pick_leaves_the_job_waiting",
            max_sends: 1,
            sessions: logged_in_sessions,
            accounts: &["test-user"],
            script: vec![token_answer("200 OK", TOKEN_HEADERS, None)],
            halted: "ceiling: 1 of 1",
            wire: &["POST"],
        },
        Ceiling {
            name: "a_halt_after_the_pick_leaves_a_refreshing_job_waiting",
            max_sends: 1,
            sessions: two_accounts,
            accounts: &["test-user", "other"],
            script: vec![token_answer("200 OK", TOKEN_HEADERS, None)],
            halted: "ceiling: 1 of 1",
            wire: &["POST"],
        },
        Ceiling {
            name: "a_ceiling_of_none_leaves_the_job_waiting",
            max_sends: 0,
            sessions: logged_in_sessions,
            accounts: &["test-user"],
            script: vec![],
            halted: "ceiling: 0 sends allowed",
            wire: &[],
        },
    ];
    for row in rows {
        let Ceiling {
            name,
            max_sends,
            sessions,
            accounts,
            script,
            halted,
            wire,
        } = row;
        let dir = TestPath::dir();
        let ids: Mutex<Vec<JobId>> = Mutex::default();
        let parked = &ids;
        script_door(script, ceiling(max_sends))
            .sessions(sessions())
            .jobs_db(JobDb::open(&daemon_db_path(&dir)).unwrap())
            .run(|mut door| async move {
                let mut ids = Vec::new();
                for &account in accounts {
                    ids.push(door.submit("profile", Some(account)).await);
                }
                door.settle().await;
                for &id in &ids {
                    assert_waiting(&mut door, id, name).await;
                }
                parked.lock().unwrap().extend(ids);
                let read = rails_status(&mut door).await.halted;
                assert!(
                    read.as_deref().is_some_and(|h| h.contains(halted)),
                    "{name}: the halt lacks {halted:?}: {read:?}"
                );
                let received: Vec<String> = door.received().into_iter().map(|r| r.method).collect();
                assert_eq!(received, wire, "{name}: the wire");
                assert_eq!(door.journal_methods(), wire, "{name}: the journal");
                door.finish().await;
            });
        for id in ids.into_inner().unwrap() {
            assert_waiting_on_disk(&dir, id, name);
        }
    }
}

/// C129 on HEAD: a status only Cloudflare sends is the evidence, so a
/// probe answered 522 with no body (a HEAD has none) halts this daemon
/// lifetime as a GET's would; nothing more goes out, and no mark is left.
/// Like a blocked probe it is no probe failure (C25: nothing fails for
/// want of a send): the route learns nothing and is not degraded, and
/// the job waits. After `reset-tripwire` the route is probed afresh and
/// the job goes out, no retry spent.
#[test]
fn a_head_522_halts_this_lifetime() {
    let script = vec![
        ScriptedResponse {
            method: "HEAD",
            body: String::new(),
            ..ScriptedResponse::cloudflare_522()
        },
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse::full("GET", "200 OK", None, r#"{"items":["after"]}"#),
    ];
    script_door(script, RailsConfig::default()).run(|mut door| async move {
        let id = door.submit("fetch", None).await;
        door.settle().await;
        let halted = rails_status(&mut door).await.halted;
        assert!(
            halted
                .as_deref()
                .is_some_and(|h| h
                    .contains("Cloudflare could not reach GGG: 522 on HEAD fetch at 2000-01-01T")),
            "{halted:?}"
        );
        let words = quote_halted(&mut door).await;
        assert!(
            words
                .as_deref()
                .is_some_and(|w| w.contains("this daemon lifetime")),
            "{words:?}"
        );
        let sends = door.journal();
        assert_eq!(sends.len(), 1, "{sends:?}");
        assert_eq!(sends[0]["status"], 522);
        let job = status(&mut door, id).await;
        assert_eq!(job.state, JobState::Waiting, "the job waits on the halt");
        let Response::Dashboard {
            degraded_endpoints, ..
        } = door.request(Request::Dashboard {}).await
        else {
            panic!("the dashboard did not answer")
        };
        assert!(
            degraded_endpoints.is_empty(),
            "Cloudflare's 5xx teaches the route nothing: {degraded_endpoints:?}"
        );

        reset(&mut door).await;
        let (job, outcome) = door.await_terminal(id).await;
        assert_eq!(job.state, JobState::Done, "{outcome:?}");
        assert_eq!(job.retries, 0, "{job:?}");
        assert_eq!(door.journal_methods(), ["HEAD", "HEAD", "GET"]);
        door.finish().await;
    });
}

/// C129 on the token route: a token POST answered by Cloudflare's 522
/// refuses in the halt's own words (the refusal prefixes them), quoting
/// none of Cloudflare's page (it is no word of GGG's on the grant),
/// marks no dead grant, and the next ask is refused by the halt without
/// a send.
#[test]
fn a_token_522_refuses_in_the_halts_words() {
    let script = vec![ScriptedResponse {
        method: "POST",
        path: "/token",
        ..ScriptedResponse::cloudflare_522()
    }];
    script_door(script, RailsConfig::default())
        .sessions(logged_in_sessions())
        .run(|mut door| async move {
            let (kind, refusal) = auth_check_refused(&mut door).await;
            assert_eq!(kind, ErrorKind::Upstream, "{refusal}");
            let rails = rails_status(&mut door).await;
            assert!(
                rails.halted.as_deref().is_some_and(
                    |h| h.contains("Cloudflare could not reach GGG: 522 on POST oauth-token")
                ),
                "the 522 halted this lifetime: {:?}",
                rails.halted
            );
            assert_eq!(rails.refresh_failed, None, "no dead grant");
            let halt = quote_halted(&mut door).await.expect("the halt's words");
            assert!(
                refusal.contains(&halt),
                "the halt's words, {halt:?}: {refusal}"
            );
            for page in ["Ray ID", "Connection timed out", "DOCTYPE", "unable to"] {
                assert!(!refusal.contains(page), "{page}: {refusal}");
            }
            assert!(!refusal.contains("acq auth"), "{refusal}");

            let (kind, second) = auth_check_refused(&mut door).await;
            assert_eq!(kind, ErrorKind::Upstream, "{second}");
            assert!(second.contains(&halt), "{second}");
            assert_eq!(door.journal_methods(), ["POST"]);
            door.finish().await;
        });
}

/// C129 beside the tripwire: a GET answered by a 503 carrying
/// Cloudflare's page halts this lifetime and trips the persisted
/// tripwire. The job's failure speaks in the halt's words, which name
/// the trip a restart keeps, never "a restart clears it", and quotes
/// none of Cloudflare's page.
#[test]
fn a_cloudflare_503_job_names_the_trip_and_quotes_no_page() {
    let script = vec![
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse {
            status: "503 Service Unavailable",
            ..ScriptedResponse::cloudflare_522()
        },
    ];
    script_door(script, tripwire_config()).run(|mut door| async move {
        let id = door.submit("fetch", None).await;
        let (job, outcome) = door.await_terminal(id).await;
        assert_eq!(job.state, JobState::Failed, "{job:?}");
        let Outcome::Failure { error, .. } = outcome else {
            panic!("the 503 did not fail its job: {outcome:?}")
        };
        let halt = quote_halted(&mut door).await.expect("halted");
        assert!(
            halt.starts_with("halted by live-test rails: 503 on GET /fetch"),
            "{halt}"
        );
        assert!(error.contains(&halt), "the halt's words, {halt:?}: {error}");
        assert!(!error.contains("restart"), "{error}");
        for page in ["Ray ID", "Connection timed out", "DOCTYPE", "unable to"] {
            assert!(!error.contains(page), "{page}: {error}");
        }
        assert_eq!(door.journal_methods(), ["HEAD", "GET"]);
        door.finish().await;
    });
}

/// C25's cancel arm: a cancel that lands while the job runs, and that a
/// halt then meets before the job's send, concludes the job cancelled.
/// It never returns to waiting, where the reset would send it. The job's
/// refresh POST is held on the wire and the job cancelled; once the
/// runtime is quiet the POST is released, lands and meets the ceiling of
/// one; the job's GET is refused by the halt, which comes before the
/// cancel at the choke point, and the job, parked by it, concludes
/// cancelled, through the protocol and, after the run, on disk.
#[test]
fn a_cancel_before_the_halt_parks_the_job_concludes_it_cancelled() {
    let dir = TestPath::dir();
    let ids: Mutex<Vec<JobId>> = Mutex::default();
    let cancelled = &ids;
    Door::open(
        Server::Account(("POST", "/token", 0)),
        Arc::new(ParkingClock::new()),
        Arc::new(RecordingCredentialStore::default()),
    )
    .rails(ceiling(1))
    .sessions(logged_in_sessions())
    .jobs_db(JobDb::open(&daemon_db_path(&dir)).unwrap())
    .run(|mut door| async move {
        let (arrived, release) = door.held();
        let id = door.submit("profile", Some("test-user")).await;
        cancelled.lock().unwrap().push(id);
        tokio::time::timeout(HANG, arrived)
            .await
            .expect("the job's refresh reached the server")
            .unwrap();
        match door.request(Request::Cancel { id }).await {
            Response::Ack { .. } => {}
            other => panic!("the cancel of job {id} answered {other:?}"),
        }
        door.quiet().await;
        release.send(()).unwrap();
        let (job, outcome) = door.await_terminal(id).await;
        assert_eq!(job.state, JobState::Cancelled, "{outcome:?}");
        let wire = door.sent();
        assert_eq!(wire, [sent("test-user", "POST")]);
        assert_eq!(door.journal_methods(), wire_methods_of(&wire));
        door.finish().await;
    });
    let id = ids.into_inner().unwrap()[0];
    let row = on_disk(&dir, id).expect("the row");
    assert_eq!(row.job.state, "cancelled", "on disk");
}

/// C25 with C129: a refresh answered by a Cloudflare block on the token
/// POST sets the block mark, and the job that asked is turned away to
/// wait for the reset, not failed: GGG never saw its request. It waits,
/// no retry spent, once the runtime has settled (the pick passes it
/// over) and on disk after the run.
#[test]
fn a_blocked_token_post_leaves_the_job_waiting() {
    let challenge = ScriptedResponse {
        method: "POST",
        path: "/token",
        ..ScriptedResponse::cloudflare_403_challenge()
    };
    let dir = TestPath::dir();
    let ids: Mutex<Vec<JobId>> = Mutex::default();
    let parked = &ids;
    script_door(vec![challenge], ceiling(10))
        .sessions(logged_in_sessions())
        .jobs_db(JobDb::open(&daemon_db_path(&dir)).unwrap())
        .run(|mut door| async move {
            let id = door.submit("profile", Some("test-user")).await;
            parked.lock().unwrap().push(id);
            door.settle().await;
            assert_waiting(&mut door, id, "the blocked refresh's job").await;
            let halted = rails_status(&mut door).await.halted;
            assert!(halted.is_some(), "the block halts the daemon");
            assert_eq!(door.journal_methods(), ["POST"]);
            door.finish().await;
        });
    let id = ids.into_inner().unwrap()[0];
    assert_waiting_on_disk(&dir, id, "the blocked refresh's job");
}

/// C25's pick-time hold on `whoami`, which has no route and may still
/// send a refresh: two accounts' `whoami`, each with a refresh due. The
/// first refresh `POST` is held at the server, which names the account
/// it is for; released, it meets the ceiling of one, its job runs to its
/// end (`whoami` pauses on the clock after its refresh, and a settle
/// resolves at the park that would move the clock to it, so its end is
/// awaited before the runtime is settled), and
/// the other's refresh is refused by the halt. That job waits, no retry
/// spent, and the pick passes it over while halted: the runtime settles
/// (no spin), and no second refresh reaches the wire.
#[test]
fn a_parked_whoami_is_not_picked_again_while_halted() {
    let dir = TestPath::dir();
    let ids: Mutex<Vec<JobId>> = Mutex::default();
    let parked = &ids;
    Door::open(
        Server::Account(("POST", "/token", 0)),
        Arc::new(ParkingClock::new()),
        Arc::new(RecordingCredentialStore::default()),
    )
    .rails(ceiling(1))
    .sessions(two_accounts())
    .jobs_db(JobDb::open(&daemon_db_path(&dir)).unwrap())
    .run(|mut door| async move {
        let (arrived, release) = door.held();
        let mut jobs = Vec::new();
        for account in ["test-user", "other"] {
            jobs.push((account, door.submit("whoami", Some(account)).await));
        }
        tokio::time::timeout(HANG, arrived)
            .await
            .expect("a refresh reached the server")
            .unwrap();
        let first = door.sent()[0].1.clone();
        let (landed, turned_away) = if first == jobs[0].0 {
            (jobs[0].1, jobs[1].1)
        } else {
            (jobs[1].1, jobs[0].1)
        };
        release.send(()).unwrap();
        let (job, outcome) = door.await_terminal(landed).await;
        assert_eq!(job.state, JobState::Done, "{first}'s whoami: {outcome:?}");
        door.settle().await;
        assert_waiting(&mut door, turned_away, "the turned-away whoami").await;
        parked.lock().unwrap().push(turned_away);
        let wire = door.sent();
        assert_eq!(wire, [sent(&first, "POST")], "no second refresh");
        assert_eq!(door.journal_methods(), wire_methods_of(&wire));
        door.finish().await;
    });
    let id = ids.into_inner().unwrap()[0];
    assert_waiting_on_disk(&dir, id, "the turned-away whoami");
}

/// A rails write that fails is never silent: the marks this daemon holds
/// but could not write, a dead grant's (a refresh `AuthCheck` asked,
/// rejected 400) and a block's (a job's GET answered by a Cloudflare
/// challenge), are each logged as the daemon's error (a restart would
/// not keep them), and a reset it could not write is refused, the halt
/// standing. The reset has the dead grant to keep (C24), so it must
/// write, not only remove: removing a file that cannot exist succeeds on
/// some platforms, but writing one fails on all. (Here no mark reached
/// disk; that a failed reset leaves a written mark standing rests on the
/// store's atomic write, unpinned.)
#[test]
fn a_rails_write_that_fails_is_logged_and_its_reset_refused() {
    // Under a regular file, which no platform opens as a directory.
    let temp = TestPath::dir();
    let blocker = temp.join("blocker");
    std::fs::write(&blocker, b"").unwrap();
    let script = vec![
        token_answer(
            "400 Bad Request",
            TOKEN_HEADERS,
            Some(r#"{"error":"invalid_grant"}"#),
        ),
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse::cloudflare_403_challenge(),
    ];
    let config = RailsConfig {
        state_path: Some(blocker.join("rails.json")),
        ..RailsConfig::default()
    };
    script_door(script, config)
        .sessions(logged_in_sessions())
        .run(|mut door| async move {
            let (_, rejected) = auth_check_refused(&mut door).await;
            let marked = rails_status(&mut door).await.refresh_failed;
            assert!(marked.is_some(), "the dead grant is marked: {rejected}");
            let id = door.submit("fetch", None).await;
            let (job, outcome) = door.await_terminal(id).await;
            assert_eq!(job.state, JobState::Failed, "{outcome:?}");
            let errors = dashboard_errors(&mut door).await;
            let unwritten = errors
                .iter()
                .filter(|m| m.contains("could not be written"))
                .count();
            assert_eq!(
                unwritten, 2,
                "the unwritten grant and block are each logged: {errors:?}"
            );

            let (kind, message) = match door.request(Request::ResetTripwire {}).await {
                Response::Error { kind, message } => (kind, message),
                other => {
                    panic!("a reset that did not reach disk is not reported done: {other:?}")
                }
            };
            assert_eq!(kind, ErrorKind::Internal, "{message}");
            assert!(message.contains("could not be written"), "{message}");
            assert!(
                rails_status(&mut door).await.halted.is_some(),
                "the halt stands"
            );
            door.finish().await;
        });
}
