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
    daemon.reset_rails();
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
    let error = daemon.valid_access_token(None, false).await.unwrap_err();
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

/// A `daemon.db` for one test, beside the harness's logs.
fn test_db_path() -> PathBuf {
    std::env::temp_dir().join(format!(
        "acquisition-rails-jobs-{}-{}.db",
        std::process::id(),
        TEST_LOG_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ))
}

fn remove_db(path: &std::path::Path) {
    for suffix in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{suffix}", path.display()));
    }
}

/// A harness daemon on the ceiling, logged in as `test-user` with its
/// refresh due, its queue on a file-backed `daemon.db`.
fn ceiling_daemon(base: &str, max_sends: u64, db: &std::path::Path) -> (Arc<Daemon>, PathBuf) {
    remove_db(db);
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
    remove_db(&db);
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
    remove_db(&db);
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
    remove_db(&db);
}
