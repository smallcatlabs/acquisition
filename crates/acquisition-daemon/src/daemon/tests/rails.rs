//! The rails: the tripwire, the ceiling and the halted refresh.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::*;

// ---- L0 live-test rails (`rails.rs`) ------------------------------------

/// Poll a job until `pred` holds on its snapshot (3 s budget).
async fn wait_until(daemon: &Daemon, id: JobId, pred: impl Fn(&JobInfo) -> bool) -> JobInfo {
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let info = daemon.shared.lock().unwrap().jobs[&id].info.clone();
            if pred(&info) {
                return info;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("job reached the expected state")
}

#[tokio::test]
async fn tripped_daemon_leaves_queued_jobs_waiting_until_reset() {
    // HEAD establishes, the first GET lands a 429 (tripping). Nothing
    // else reaches the server until the reset; then both queued jobs
    // go out (C25).
    let responses = vec![
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse::full("GET", "429 Too Many Requests", Some(0), "{}"),
        ScriptedResponse::full("GET", "200 OK", None, r#"{"ok":true}"#),
        ScriptedResponse::full("GET", "200 OK", None, r#"{"ok":true}"#),
    ];
    let (base, requests, server) = scripted_server(responses).await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) =
        test_daemon_with(Provider::mock(&base), clock.clone(), tripwire_config());
    let rails = daemon.choke.rails().clone();
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());

    let first = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let info = wait_until(&daemon, first, |i| i.retries == 1).await;
    assert_eq!(
        info.state,
        JobState::Waiting,
        "re-queued by the 429, then held"
    );
    assert!(
        rails.halted().unwrap().contains("429 on GET /fetch"),
        "{:?}",
        rails.halted()
    );
    let second = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(
        requests.lock().unwrap().as_slice(),
        ["HEAD", "GET"],
        "a tripped daemon sends nothing"
    );
    for id in [first, second] {
        let state = daemon.shared.lock().unwrap().jobs[&id].info.state;
        assert_eq!(state, JobState::Waiting, "job {id} waits out the halt");
    }
    let logged = {
        let s = daemon.shared.lock().unwrap();
        s.errors
            .iter()
            .any(|(_, m)| m.contains("LIVE-TEST RAILS TRIPPED"))
    };
    assert!(logged, "the trip is announced once in the error ring");

    // The reset wakes the queue; the limiter's own hold from the 429
    // still applies.
    daemon.reset_rails().unwrap();
    let (info, _) = wait_terminal(&daemon, first).await;
    assert_eq!(info.state, JobState::Done);
    let (info, _) = wait_terminal(&daemon, second).await;
    assert_eq!(info.state, JobState::Done);
    assert_eq!(
        requests.lock().unwrap().as_slice(),
        ["HEAD", "GET", "GET", "GET"]
    );
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
}

#[tokio::test]
async fn the_ceiling_halts_after_n_sends_and_holds_the_queue() {
    let responses = vec![
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse::full("GET", "200 OK", None, r#"{"ok":true}"#),
    ];
    let (base, requests, server) = scripted_server(responses).await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon_with(
        Provider::mock(&base),
        clock,
        RailsConfig {
            max_sends: Some(2),
            ..RailsConfig::default()
        },
    );
    let rails = daemon.choke.rails().clone();
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());

    let first = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let (info, _) = wait_terminal(&daemon, first).await;
    assert_eq!(
        info.state,
        JobState::Done,
        "HEAD + GET reach the ceiling exactly"
    );
    assert!(rails.halted().unwrap().contains("ceiling: 2 of 2"));

    let second = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    let state = daemon.shared.lock().unwrap().jobs[&second].info.state;
    assert_eq!(
        state,
        JobState::Waiting,
        "a ceiling halt holds the queue too"
    );
    assert_eq!(requests.lock().unwrap().len(), 2);
    assert_eq!(rails.status().sends, 2);
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
}

#[tokio::test]
async fn halted_daemon_refuses_refresh_before_any_send() {
    let (base, requests, server) = token_server_answering(vec![]).await;
    let clock = Arc::new(ManualClock::new());
    let (mut daemon, log_path) = test_daemon_with(Provider::mock(&base), clock, tripwire_config());
    let rails = daemon.choke.rails().clone();
    // GGG's 429 (it names its policy): a ladder trip, not a block (C129).
    let ggg = json!({ "x-rate-limit-policy": "character-list-request-limit" });
    rails.record(&crate::rails::SendReport {
        method: "HEAD",
        route: "character-list",
        url_path: "/character",
        status: Some(429),
        error: None,
        ok: false,
        counted: false,
        rate: &Value::Null,
        shape: None,
        block: false,
        unreachable: false,
        headers: &ggg,
        wait: Duration::ZERO,
    });
    logged_in(&mut daemon);
    let error = daemon
        .valid_access_token(None, false, None)
        .await
        .unwrap_err();
    assert!(
        error.message.contains("halted by live-test rails"),
        "{error}"
    );
    assert_eq!(
        error.kind,
        ErrorKind::Upstream,
        "a halted refresh is not a dead session"
    );
    assert!(requests.lock().unwrap().is_empty());
    server.abort();
    // The one journal line is the synthetic probe recorded above, not a
    // send; nothing else may have been written.
    assert_journal_matches_wire(&log_path, &["HEAD".to_string()]);
    finish_harness(None, &log_path);
}

/// A harness daemon on the ceiling, logged in as `test-user` with its
/// refresh due, its queue on a file-backed `daemon.db`.
fn ceiling_daemon(base: &str, max_sends: u64, db: &std::path::Path) -> (Arc<Daemon>, TestPath) {
    let (mut daemon, log_path) = test_daemon_with(
        Provider::mock(base),
        Arc::new(ManualClock::new()),
        RailsConfig {
            max_sends: Some(max_sends),
            ..RailsConfig::default()
        },
    );
    logged_in(&mut daemon);
    Arc::get_mut(&mut daemon).unwrap().jobs_db = Mutex::new(JobDb::open(db).unwrap());
    (daemon, log_path)
}

/// Poll until the daemon is halted and no job in `ids` is running: the
/// halt lands while a job runs, and the job settles after, waiting or
/// concluded (`assert_parked` says which).
async fn wait_halted_and_settled(daemon: &Daemon, ids: &[JobId]) {
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let settled = {
                let s = daemon.shared.lock().unwrap();
                ids.iter()
                    .all(|id| s.jobs[id].info.state != JobState::Running)
            };
            if settled && daemon.choke.rails().halted().is_some() {
                return;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("halted, and the jobs settled");
}

/// The job is waiting with no retry spent, in memory and in `daemon.db`
/// read through a connection of its own.
fn assert_parked(daemon: &Daemon, db: &std::path::Path, id: JobId) {
    let (info, outcome) = {
        let s = daemon.shared.lock().unwrap();
        (s.jobs[&id].info.clone(), s.jobs[&id].outcome.clone())
    };
    assert_eq!(
        info.state,
        JobState::Waiting,
        "job {id} in memory: {outcome:?}"
    );
    assert_eq!(info.retries, 0, "job {id}: a halt spends no retry");
    let row = JobDb::open(db).unwrap().get(id).unwrap().expect("the row");
    assert_eq!(row.state, JobState::Waiting.to_string(), "job {id} on disk");
    assert_eq!(row.retries, 0, "job {id} on disk");
    assert_eq!(row.outcome, None, "job {id} on disk");
}

/// C25: a halt that lands after the pick turns the job away; nothing
/// fails for want of a send. The ceiling of one is met by the job's own
/// refresh POST; its GET is then refused by the halt, and the job is
/// back to waiting with no retry spent, in memory and on disk. Nothing
/// else is sent, and the pick passes the job over until the reset: a
/// turned-away job is not picked again at once.
#[tokio::test]
async fn a_halt_after_the_pick_leaves_the_job_waiting() {
    let db = test_db_path();
    let (base, requests, server) = token_server_answering(vec!["200 OK"]).await;
    let (daemon, log_path) = ceiling_daemon(&base, 1, &db);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = daemon
        .submit(
            "profile".into(),
            json!({}),
            0,
            "test".into(),
            Some("test-user".into()),
        )
        .unwrap();
    wait_halted_and_settled(&daemon, &[id]).await;
    // Long enough for a failure, a second send or a re-pick to show.
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_parked(&daemon, &db, id);
    assert!(
        daemon
            .choke
            .rails()
            .halted()
            .unwrap()
            .contains("ceiling: 1 of 1"),
        "{:?}",
        daemon.choke.rails().halted()
    );
    assert!(daemon.pick_runnable().is_empty(), "the pick passes it over");
    server.await.unwrap();
    assert_eq!(wire_posts(&requests), ["POST"]);
    assert_journal_matches_wire(&log_path, &wire_posts(&requests));
    finish_harness(dispatcher, &log_path);
}

/// C25, the refresh itself refused: two accounts, each with a refresh
/// due, picked together. The token route admits one POST at a time; the
/// first meets the ceiling of one, and the second account's refresh,
/// already running, is refused by the halt. That job is turned away as
/// the first is (its GET refused): both wait, no retry spent, and the
/// wire holds the one POST.
#[tokio::test]
async fn a_halt_after_the_pick_leaves_a_refreshing_job_waiting() {
    let db = test_db_path();
    let (base, requests, server) = token_server_answering(vec!["200 OK"]).await;
    let (daemon, log_path) = ceiling_daemon(&base, 1, &db);
    daemon.shared.lock().unwrap().auth.by_account.insert(
        "other".into(),
        AuthSession {
            username: Some("other".into()),
            refresh_token: Some("rt-other".into()),
            ..Default::default()
        },
    );
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let ids: Vec<JobId> = ["test-user", "other"]
        .into_iter()
        .map(|account| {
            daemon
                .submit(
                    "profile".into(),
                    json!({}),
                    0,
                    "test".into(),
                    Some(account.into()),
                )
                .unwrap()
        })
        .collect();
    wait_halted_and_settled(&daemon, &ids).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    for &id in &ids {
        assert_parked(&daemon, &db, id);
    }
    assert!(
        daemon.pick_runnable().is_empty(),
        "the pick passes both over"
    );
    server.await.unwrap();
    assert_eq!(wire_posts(&requests), ["POST"]);
    assert_journal_matches_wire(&log_path, &wire_posts(&requests));
    finish_harness(dispatcher, &log_path);
}

/// C25 at a ceiling of none: the daemon is halted from its start, the
/// pick passes the job over, and it waits with nothing sent. (It never
/// runs, so this holds the pick's arm, not the turn-away after it.)
#[tokio::test]
async fn a_ceiling_of_none_leaves_the_job_waiting() {
    let db = test_db_path();
    let (base, requests, server) = token_server_answering(vec![]).await;
    let (daemon, log_path) = ceiling_daemon(&base, 0, &db);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = daemon
        .submit(
            "profile".into(),
            json!({}),
            0,
            "test".into(),
            Some("test-user".into()),
        )
        .unwrap();
    wait_halted_and_settled(&daemon, &[id]).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_parked(&daemon, &db, id);
    server.await.unwrap();
    assert!(requests.lock().unwrap().is_empty());
    assert_journal_matches_wire(&log_path, &[]);
    finish_harness(dispatcher, &log_path);
}

/// C129 on HEAD: a status only Cloudflare sends is the evidence, so a
/// probe answered 522 with no body (a HEAD has none) halts this daemon
/// lifetime as a GET's would; nothing more goes out, and no mark is left.
/// Like a blocked probe it is no probe failure (C25: nothing fails for
/// want of a send): the route learns nothing and is not degraded, and
/// the job waits. After `reset-tripwire` the route is probed afresh and
/// the job goes out.
#[tokio::test]
async fn a_head_522_halts_this_lifetime() {
    let (base, requests, server) = scripted_server(vec![
        ScriptedResponse {
            method: "HEAD",
            body: String::new(),
            ..ScriptedResponse::cloudflare_522()
        },
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
    tokio::time::timeout(Duration::from_secs(3), async {
        while daemon.choke.rails().halted().is_none() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("the HEAD 522 halted the daemon");
    // Long enough for a second probe or a GET, were either admitted.
    tokio::time::sleep(Duration::from_millis(100)).await;

    let halt = daemon.choke.rails().halted().unwrap();
    assert!(
        halt.contains("Cloudflare could not reach GGG: 522 on HEAD fetch at 2000-01-01T"),
        "{halt}"
    );
    assert!(halt.contains("this daemon lifetime"), "{halt}");
    assert_eq!(requests.lock().unwrap().as_slice(), ["HEAD"]);
    let sends = journal_sends(&log_path);
    assert_eq!(sends.len(), 1, "{sends:?}");
    assert_eq!(sends[0]["status"], 522);
    assert_eq!(
        daemon.shared.lock().unwrap().jobs[&id].info.state,
        JobState::Waiting,
        "the job waits on the halt"
    );
    assert_eq!(
        daemon.choke.endpoint_state("fetch"),
        EndpointState::Unknown,
        "Cloudflare's 5xx teaches the route nothing"
    );

    daemon.reset_rails().unwrap();
    let (info, _) = wait_terminal(&daemon, id).await;
    assert_eq!(info.state, JobState::Done, "{info:?}");
    assert_eq!(info.retries, 0, "{info:?}");
    assert_eq!(requests.lock().unwrap().as_slice(), ["HEAD", "HEAD", "GET"]);
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
}

/// C129 on the token route: a token POST answered by Cloudflare's 522
/// refuses in the halt's own words, quoting none of Cloudflare's page
/// (it is no word of GGG's on the grant), marks no dead grant, and the
/// next ask is refused by the halt without a send.
#[tokio::test]
async fn a_token_522_refuses_in_the_halts_words() {
    let (base, requests, server) = scripted_server(vec![ScriptedResponse {
        method: "POST",
        path: "/token",
        ..ScriptedResponse::cloudflare_522()
    }])
    .await;
    let clock = Arc::new(ManualClock::new());
    let (mut daemon, log_path) =
        test_daemon_with(Provider::mock(&base), clock, RailsConfig::default());
    let rails = daemon.choke.rails().clone();
    logged_in(&mut daemon);

    let refusal = daemon
        .valid_access_token(None, false, None)
        .await
        .unwrap_err();
    assert_eq!(refusal.kind, ErrorKind::Upstream, "{refusal}");
    let halt = rails.halted().expect("the 522 halted this lifetime");
    assert!(
        halt.contains("Cloudflare could not reach GGG: 522 on POST oauth-token"),
        "{halt}"
    );
    assert!(
        refusal.message.contains(&halt),
        "the halt's words, {halt:?}: {refusal}"
    );
    for page in ["Ray ID", "Connection timed out", "DOCTYPE", "unable to"] {
        assert!(!refusal.message.contains(page), "{page}: {refusal}");
    }
    assert!(!refusal.message.contains("acq auth"), "{refusal}");
    assert_eq!(rails.refresh_failed("test-user"), None, "no dead grant");

    let second = daemon
        .valid_access_token(None, false, None)
        .await
        .unwrap_err();
    assert_eq!(second.kind, ErrorKind::Upstream, "{second}");
    assert!(second.message.contains(&halt), "{second}");
    server.await.unwrap();
    assert_eq!(requests.lock().unwrap().as_slice(), ["POST"]);
    assert_journal_matches_wire(&log_path, &requests.lock().unwrap());
    finish_harness(None, &log_path);
}

/// C129 beside the tripwire: a GET answered by a 503 carrying
/// Cloudflare's page halts this lifetime and trips the persisted
/// tripwire. The job's failure speaks in the halt's words, which name
/// the trip a restart keeps, never "a restart clears it", and quotes
/// none of Cloudflare's page.
#[tokio::test]
async fn a_cloudflare_503_job_names_the_trip_and_quotes_no_page() {
    let (base, requests, server) = scripted_server(vec![
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse {
            status: "503 Service Unavailable",
            ..ScriptedResponse::cloudflare_522()
        },
    ])
    .await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon_with(Provider::mock(&base), clock, tripwire_config());
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let (info, outcome) = wait_terminal(&daemon, id).await;
    assert_eq!(info.state, JobState::Failed, "{info:?}");
    let Outcome::Failure { error, .. } = outcome else {
        panic!("the 503 did not fail its job")
    };
    let halt = daemon.choke.rails().halted().expect("halted");
    assert!(
        halt.starts_with("halted by live-test rails: 503 on GET /fetch"),
        "{halt}"
    );
    assert!(error.contains(&halt), "the halt's words, {halt:?}: {error}");
    assert!(!error.contains("restart"), "{error}");
    for page in ["Ray ID", "Connection timed out", "DOCTYPE", "unable to"] {
        assert!(!error.contains(page), "{page}: {error}");
    }
    assert_eq!(requests.lock().unwrap().as_slice(), ["HEAD", "GET"]);
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
}

/// C25's cancel arm: a cancel that lands while the job runs, and that
/// a halt then meets before the job's send, concludes the job cancelled.
/// It never returns to waiting, where the reset would send it. The job's
/// refresh POST is held on the wire and the job cancelled; the POST
/// lands and meets the ceiling of one; the job's GET is refused by the
/// halt, which comes before the cancel at the choke point, and the job,
/// parked by it, concludes cancelled, in memory and on disk.
#[tokio::test]
async fn a_cancel_before_the_halt_parks_the_job_concludes_it_cancelled() {
    let db = test_db_path();
    let (base, sent_log, arrived, release) = account_server(("POST", "/token", 0)).await;
    let (daemon, log_path) = ceiling_daemon(&base, 1, &db);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = daemon
        .submit(
            "profile".into(),
            json!({}),
            0,
            "test".into(),
            Some("test-user".into()),
        )
        .unwrap();
    arrived.await.unwrap();
    daemon.cancel(id).unwrap();
    release.send(()).unwrap();
    wait_halted_and_settled(&daemon, &[id]).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    let (state, outcome) = {
        let s = daemon.shared.lock().unwrap();
        (s.jobs[&id].info.state, s.jobs[&id].outcome.clone())
    };
    assert_eq!(state, JobState::Cancelled, "{outcome:?}");
    let row = JobDb::open(&db).unwrap().get(id).unwrap().expect("the row");
    assert_eq!(row.state, JobState::Cancelled.to_string(), "on disk");
    assert_eq!(
        sent_log.lock().unwrap().as_slice(),
        [sent("test-user", "POST")]
    );
    assert_journal_matches_wire(&log_path, &wire_methods(&sent_log));
    finish_harness(dispatcher, &log_path);
}

/// C25 with C129: a refresh answered by a Cloudflare block on the token
/// POST sets the block mark, and the job that asked is turned away to
/// wait for the reset, not failed: GGG never saw its request.
#[tokio::test]
async fn a_blocked_token_post_leaves_the_job_waiting() {
    let db = test_db_path();
    let challenge = ScriptedResponse {
        method: "POST",
        path: "/token",
        ..ScriptedResponse::cloudflare_403_challenge()
    };
    let (base, requests, server) = scripted_server(vec![challenge]).await;
    let (daemon, log_path) = ceiling_daemon(&base, 10, &db);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = daemon
        .submit(
            "profile".into(),
            json!({}),
            0,
            "test".into(),
            Some("test-user".into()),
        )
        .unwrap();
    wait_halted_and_settled(&daemon, &[id]).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_parked(&daemon, &db, id);
    assert!(daemon.pick_runnable().is_empty(), "the pick passes it over");
    server.await.unwrap();
    assert_eq!(requests.lock().unwrap().as_slice(), ["POST"]);
    finish_harness_wire(dispatcher, &log_path, &requests);
}

/// C25's pick-time hold on `whoami`, which has no route and may still
/// send a refresh: two accounts' `whoami`, each with a refresh due; the
/// first POST meets the ceiling of one and the other's refresh is refused
/// by the halt. That job waits, and the pick passes it over while halted:
/// no spin, and no second refresh.
#[tokio::test]
async fn a_parked_whoami_is_not_picked_again_while_halted() {
    let db = test_db_path();
    let (base, sent_log, _arrived, _release) = account_server(("NONE", "/", 0)).await;
    let (daemon, log_path) = ceiling_daemon(&base, 1, &db);
    daemon.shared.lock().unwrap().auth.by_account.insert(
        "other".into(),
        AuthSession {
            username: Some("other".into()),
            refresh_token: Some("rt-other".into()),
            ..Default::default()
        },
    );
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let ids: Vec<JobId> = ["test-user", "other"]
        .into_iter()
        .map(|account| {
            daemon
                .submit(
                    "whoami".into(),
                    json!({}),
                    0,
                    "test".into(),
                    Some(account.into()),
                )
                .unwrap()
        })
        .collect();
    wait_halted_and_settled(&daemon, &ids).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    // With the dispatcher stopped and no task left running, the pick
    // passes the turned-away job over.
    dispatcher.abort();
    tokio::time::timeout(Duration::from_secs(3), async {
        while !daemon.shared.lock().unwrap().active_jobs.is_empty() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("no job task left running");
    let parked: Vec<JobId> = {
        let s = daemon.shared.lock().unwrap();
        ids.iter()
            .copied()
            .filter(|id| s.jobs[id].info.state == JobState::Waiting)
            .collect()
    };
    assert_eq!(parked.len(), 1, "one whoami was turned away");
    assert_parked(&daemon, &db, parked[0]);
    assert!(daemon.pick_runnable().is_empty(), "the pick passes it over");
    assert_eq!(wire_methods(&sent_log), ["POST"], "no second refresh");
    assert_journal_matches_wire(&log_path, &wire_methods(&sent_log));
    finish_harness(None, &log_path);
}

/// A rails write that fails is never silent: the block mark this daemon
/// holds but could not write is logged as the daemon's error (a restart
/// would not keep it), and a reset it could not write is refused, the
/// halt standing in memory. The reset has a dead grant to keep (C24),
/// so it must write, not only remove: removing a file that cannot exist
/// succeeds on some platforms, but writing one fails on all. (Here the
/// mark never reached disk; that a failed reset leaves a written mark
/// standing rests on the store's atomic write, unpinned.)
#[tokio::test]
async fn a_rails_write_that_fails_is_logged_and_its_reset_refused() {
    let responses = vec![
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse::cloudflare_403_challenge(),
    ];
    let (base, requests, server) = scripted_server(responses).await;
    // Under a regular file, which no platform opens as a directory.
    let temp = tempfile::tempdir().unwrap();
    let blocker = temp.path().join("blocker");
    std::fs::write(&blocker, b"").unwrap();
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon_with(
        Provider::mock(&base),
        clock,
        RailsConfig {
            state_path: Some(blocker.join("rails.json")),
            ..RailsConfig::default()
        },
    );
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let (info, _) = wait_terminal(&daemon, id).await;
    assert_eq!(info.state, JobState::Failed);
    let errors = format!("{:?}", daemon.shared.lock().unwrap().errors);
    assert!(
        errors.contains("could not be written"),
        "the unwritten mark is logged: {errors}"
    );
    assert!(daemon.choke.rails().mark_refresh_failed("A#1", "HTTP 400"));

    let reply = daemon
        .handle_request(Request::ResetTripwire, &mut None)
        .await;
    let Response::Error { message, .. } = &reply else {
        panic!("a reset that did not reach disk is not reported done: {reply:?}")
    };
    assert!(message.contains("could not be written"), "{message}");
    assert!(
        daemon.choke.rails().halted().is_some(),
        "the halt stands in memory"
    );
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
}
