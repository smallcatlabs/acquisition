//! The persisted queue: a restart, a restore, and a queue write that fails.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::*;
use std::sync::atomic::Ordering;

// ---- persistence (C6) ------------------------------------------------

fn test_db_path() -> PathBuf {
    std::env::temp_dir().join(format!(
        "acquisition-jobs-{}-{}.db",
        std::process::id(),
        TEST_LOG_ID.fetch_add(1, Ordering::Relaxed)
    ))
}

/// A harness daemon on a file-backed `daemon.db` (test daemons default
/// to a throwaway in-memory one), restored from it like `run` does.
fn persisting_daemon(base: &str, db_path: &std::path::Path) -> (Arc<Daemon>, PathBuf) {
    let (mut daemon, log_path) = test_daemon(base, Arc::new(ManualClock::new()));
    Arc::get_mut(&mut daemon).unwrap().jobs_db = Mutex::new(JobDb::open(db_path).unwrap());
    daemon.restore_jobs(Retention::default()).unwrap();
    (daemon, log_path)
}

fn remove_db(path: &std::path::Path) {
    for suffix in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{suffix}", path.display()));
    }
}

/// Make every later statement on the daemon's queue fail: the table
/// is dropped through a second connection to the same file, as a
/// neighbour corrupting `daemon.db` under a running daemon would, so
/// the queue-failure handling meets a real failure and the store
/// crate needs no test-only door (its `test-hooks` feature, retired
/// 2026-09-11: a feature unified into `--all-targets` builds gave
/// every binary two forms).
fn break_queue(db: &std::path::Path) {
    rusqlite::Connection::open(db)
        .unwrap()
        .execute_batch("DROP TABLE jobs")
        .unwrap();
}

#[tokio::test]
async fn the_queue_and_results_survive_a_daemon_restart() {
    let db = test_db_path();
    remove_db(&db);
    // Lifetime 1: one job finished, one running, one waiting. No
    // dispatcher runs — the running state is written the way
    // `process` writes it — so nothing of this lifetime survives the
    // drop to write behind lifetime 2's back.
    let (daemon, log1) = persisting_daemon("http://127.0.0.1:1", &db);
    let done = daemon
        .submit(
            "sleep".into(),
            json!({ "seconds": 0.01 }),
            0,
            "a".into(),
            None,
        )
        .unwrap();
    daemon.start_and_finish(
        done,
        Outcome::Success {
            payload: json!({ "slept_seconds": 0.01 }),
        },
    );
    let running = daemon
        .submit(
            "sleep".into(),
            json!({ "seconds": 0.3 }),
            2,
            "b".into(),
            None,
        )
        .unwrap();
    {
        let mut s = daemon.shared.lock().unwrap();
        let entry = s.jobs.get_mut(&running).unwrap();
        entry.info.state = JobState::Running;
        daemon.persist(entry);
    }
    let waiting = daemon
        .submit(
            "sleep".into(),
            json!({ "seconds": 0.01 }),
            1,
            "c".into(),
            None,
        )
        .unwrap();
    daemon.set_priority(waiting, 5).unwrap();
    drop(daemon);

    // Lifetime 2: the running job is re-queued, the waiting one kept
    // with its reprioritization, ids continue, and the finished job's
    // result is still answerable over the protocol.
    let (daemon, log2) = persisting_daemon("http://127.0.0.1:1", &db);
    let jobs = daemon.shared.lock().unwrap().list(&daemon);
    let summary: Vec<(JobId, JobState, Priority)> =
        jobs.iter().map(|j| (j.id, j.state, j.priority)).collect();
    assert_eq!(
        summary,
        [
            (running, JobState::Waiting, 2),
            (waiting, JobState::Waiting, 5)
        ],
        "open jobs come back; terminal ones stay in the table"
    );
    assert_eq!(jobs[0].submitted_by, "b", "every field rides along");
    let fresh = daemon
        .submit(
            "sleep".into(),
            json!({ "seconds": 0.01 }),
            0,
            "d".into(),
            None,
        )
        .unwrap();
    assert_eq!(fresh, waiting + 1, "ids continue across the restart");
    match daemon
        .handle_request(Request::Result { id: done }, &mut None)
        .await
    {
        Response::Result {
            outcome: Outcome::Success { payload },
            ..
        } => {
            assert_eq!(payload["slept_seconds"], json!(0.01))
        }
        other => panic!("previous lifetime's result not served: {other:?}"),
    }
    match daemon
        .handle_request(Request::Result { id: 999 }, &mut None)
        .await
    {
        Response::Error { message, .. } => assert_eq!(message, "no job 999"),
        other => panic!("{other:?}"),
    }

    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    for id in [running, waiting, fresh] {
        let (info, _) = wait_terminal(&daemon, id).await;
        assert_eq!(info.state, JobState::Done, "job {id} runs to completion");
    }
    dispatcher.abort();
    // Lifetime 3 has nothing to restore.
    drop(daemon);
    let (daemon, log3) = persisting_daemon("http://127.0.0.1:1", &db);
    assert!(daemon.shared.lock().unwrap().jobs.is_empty());
    for p in [log1, log2, log3] {
        finish_harness(None, &p);
    }
    remove_db(&db);
}

#[tokio::test]
async fn restore_drops_probes_and_finishes_a_parent_whose_children_are_done() {
    let db = test_db_path();
    remove_db(&db);
    {
        let db = JobDb::open(&db).unwrap();
        let row = |id: u64, kind: &str, state: &str, parent: Option<u64>| {
            acquisition_store::jobs::JobRow {
                id,
                kind: kind.into(),
                state: state.into(),
                priority: 0,
                submitted_by: "t".into(),
                parent,
                retries: 0,
                account: Some("A#1".into()),
                params: json!({ "route": "fetch@A#1", "league": "Standard" }),
                outcome: (state == "done").then(|| json!({ "outcome": "success", "payload": {} })),
                deferred: None,
                cancel_requested: false,
                submitted_at: 0,
                updated_at: 0,
            }
        };
        // A refresh whose last child finished just before the daemon
        // died: its own result is held, deferred.
        let mut parent = row(1, "refresh", "running", None);
        parent.deferred = Some(json!({ "outcome": "success", "payload": { "tabs_listed": 1 } }));
        db.upsert(&parent).unwrap();
        db.upsert(&row(2, "stash", "done", Some(1))).unwrap();
        // Per-lifetime: a probe from the previous daemon.
        db.upsert(&row(3, "probe", "waiting", None)).unwrap();
        // A running job that was asked to cancel: it never ran again.
        let mut cancelling = row(4, "sleep", "running", None);
        cancelling.cancel_requested = true;
        db.upsert(&cancelling).unwrap();
        // Running on a declared no-probe route: a replay would send
        // against an empty limiter, so it is not replayed.
        db.upsert(&row(5, "leagues", "running", None)).unwrap();
        // Mid-fan-out: children submitted, held result not yet
        // written. Re-running would duplicate the children.
        db.upsert(&row(6, "refresh", "running", None)).unwrap();
        db.upsert(&row(7, "stash", "waiting", Some(6))).unwrap();
    }
    let (daemon, log) = persisting_daemon("http://127.0.0.1:1", &db);
    let jobs = daemon.shared.lock().unwrap().list(&daemon);
    let states: Vec<(JobId, JobState)> = jobs.iter().map(|j| (j.id, j.state)).collect();
    assert_eq!(
        states,
        [
            (1, JobState::Done),
            (2, JobState::Done),
            (4, JobState::Cancelled),
            (5, JobState::Failed),
            (6, JobState::Running),
            (7, JobState::Waiting)
        ],
        "no-probe replays fail; a mid-fan-out parent holds for its children"
    );
    let Outcome::Failure { error, .. } = daemon.shared.lock().unwrap().jobs[&5]
        .outcome
        .clone()
        .unwrap()
    else {
        panic!("the leagues replay did not fail")
    };
    assert!(error.contains("not replayed"), "{error}");
    // The held parent finishes when its remaining child does — as
    // interrupted, never success: the full child set is unknown.
    daemon.cancel(7).unwrap();
    let Outcome::Failure { error, children } = daemon.shared.lock().unwrap().jobs[&6]
        .outcome
        .clone()
        .unwrap()
    else {
        panic!("held parent must finish as interrupted, not success")
    };
    assert!(error.contains("mid fan-out"), "{error}");
    // C23: the deferred failure keeps its message and gains the tally.
    assert_eq!(
        children,
        Some(Children {
            done: 0,
            failed: 0,
            cancelled: 1,
            failed_ids: vec![],
        }),
        "the interrupted parent carries its children's tally as a field"
    );
    let Outcome::Success { payload } = daemon.shared.lock().unwrap().jobs[&1]
        .outcome
        .clone()
        .unwrap()
    else {
        panic!("parent did not finish with its held result")
    };
    assert_eq!(payload["tabs_listed"], json!(1));
    assert_eq!(payload["children"]["done"], json!(1));
    assert!(
        JobDb::open(&db).unwrap().get(3).unwrap().is_none(),
        "the probe row is gone from the table too"
    );
    assert_eq!(daemon.shared.lock().unwrap().next_id, 8);
    finish_harness(None, &log);
    remove_db(&db);
}

#[tokio::test]
async fn a_failed_queue_write_refuses_new_jobs_and_stops_dispatch() {
    let db = test_db_path();
    remove_db(&db);
    let (daemon, log) = persisting_daemon("http://127.0.0.1:1", &db);
    daemon
        .submit("sleep".into(), json!({}), 0, "t".into(), None)
        .unwrap();
    break_queue(&db);
    let err = daemon
        .submit("sleep".into(), json!({}), 0, "t".into(), None)
        .unwrap_err();
    assert!(err.message.contains("refuses new jobs"), "{err}");
    assert_eq!(err.kind, ErrorKind::QueueFailed);
    assert_eq!(
        daemon.shared.lock().unwrap().next_id,
        2,
        "the refused submit rolled its id back"
    );
    assert!(
        daemon.pick_runnable().is_empty(),
        "a failed queue dispatches nothing"
    );
    assert!(
        !Daemon::has_live_jobs(&daemon.shared.lock().unwrap(), true),
        "the parked queue does not hold the daemon up"
    );
    finish_harness(None, &log);
    remove_db(&db);
}

#[tokio::test]
async fn a_broken_queue_is_fatal_to_restore() {
    let db = test_db_path();
    remove_db(&db);
    let (daemon, log) = persisting_daemon("http://127.0.0.1:1", &db);
    break_queue(&db);
    let err = daemon.restore_jobs(Retention::default()).unwrap_err();
    assert!(err.to_string().contains("could not read"), "{err:#}");
    finish_harness(None, &log);
    remove_db(&db);
}

#[tokio::test]
async fn a_transition_that_cannot_persist_does_not_run() {
    // waiting → running is the transition that gates a send: if its
    // write fails, disk still says waiting, and after a restart a
    // no-probe job would replay blind. The job must not run.
    let db = test_db_path();
    remove_db(&db);
    let (daemon, log) = persisting_daemon("http://127.0.0.1:1", &db);
    let id = daemon
        .submit(
            "sleep".into(),
            json!({ "seconds": 0.01 }),
            0,
            "t".into(),
            None,
        )
        .unwrap();
    break_queue(&db);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    tokio::time::sleep(Duration::from_millis(100)).await;
    let state = daemon.shared.lock().unwrap().jobs[&id].info.state;
    assert_eq!(
        state,
        JobState::Waiting,
        "the job reverted instead of running"
    );
    assert!(
        daemon.queue_failed().is_some(),
        "the failed write tripped the queue"
    );
    finish_harness(dispatcher, &log);
    remove_db(&db);
}

#[tokio::test]
async fn cancel_and_reprioritize_report_a_failed_queue_write() {
    let db = test_db_path();
    remove_db(&db);
    let (daemon, log) = persisting_daemon("http://127.0.0.1:1", &db);
    let id = daemon
        .submit("sleep".into(), json!({}), 3, "t".into(), None)
        .unwrap();
    break_queue(&db);
    let err = daemon.set_priority(id, 9).unwrap_err();
    assert!(err.message.contains("priority unchanged"), "{err}");
    assert_eq!(err.kind, ErrorKind::QueueFailed);
    assert_eq!(daemon.shared.lock().unwrap().jobs[&id].info.priority, 3);
    let err = daemon.cancel(id).unwrap_err();
    assert!(err.message.contains("may not survive"), "{err}");
    assert_eq!(err.kind, ErrorKind::QueueFailed);
    assert_eq!(
        daemon.shared.lock().unwrap().jobs[&id].info.state,
        JobState::Cancelled,
        "the cancellation still holds for this lifetime"
    );
    finish_harness(None, &log);
    remove_db(&db);
}

#[tokio::test]
async fn a_result_read_failure_is_reported_not_no_job() {
    let db = test_db_path();
    remove_db(&db);
    let (daemon, log) = persisting_daemon("http://127.0.0.1:1", &db);
    break_queue(&db);
    match daemon
        .handle_request(Request::Result { id: 42 }, &mut None)
        .await
    {
        Response::Error { message, .. } => assert!(
            message.contains("could not read the persisted queue"),
            "{message}"
        ),
        other => panic!("{other:?}"),
    }
    finish_harness(None, &log);
    remove_db(&db);
}
