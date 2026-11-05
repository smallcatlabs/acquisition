//! Parent jobs and apply: the fan-out, and a parent's end against its
//! children's. The fan-out runs through the door (C158, a daemon scenario
//! crosses the daemon's frontend boundary only); apply's admission is
//! rows of the admission table (`queue.rs`); the cancels and the finish
//! are component tests, each forcing an interleaving through the
//! daemon's internals that no request reaches.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::door::*;
use super::harness::*;
use crate::daemon::*;

/// The apply parent (tracer step 7, C43: a plan's action set executed
/// exactly): one child per admitted tuple, no send of its own, no
/// expansion, with the children's responses recorded through the store
/// like any other fetch. Through the door: the mock, a live session for
/// Alice, the apply a client's `Submit` as her; its end and the children
/// read through `Result` and `List`, and the account's store through the
/// store's read API, as a frontend reads it while the daemon runs (C12).
#[test]
fn apply_fans_out_exactly_its_tuples_and_records_through_the_store() {
    let clock = Arc::new(ParkingClock::new());
    let sessions = live_sessions(&clock);
    let store_dir = TestPath::dir();
    let dir = store_dir.to_path_buf();
    Door::open(
        Server::Mock,
        clock,
        Arc::new(RecordingCredentialStore::default()),
    )
    .store_dir(dir.clone())
    .sessions(sessions)
    .run(|mut door| async move {
        let apply = Request::Submit {
            kind: "apply".into(),
            params: json!({ "jobs": [
                { "kind": "stashes", "params": { "league": "Standard" } },
                { "kind": "stash", "params": { "league": "Standard", "id": "dump", "deep": false } },
            ] }),
            priority: 0,
            submitted_by: "test".into(),
            account: Some("Alice#1234".into()),
        };
        let id = match door.request(apply).await {
            Response::Submitted { id } => id,
            other => panic!("the apply answered {other:?}"),
        };
        let (info, outcome) = door.await_terminal(id).await;
        let Outcome::Success { payload } = outcome else {
            panic!("apply failed: {outcome:?}")
        };
        assert_eq!(info.state, JobState::Done);
        assert_eq!(payload["requests"], json!(2));
        assert_eq!(payload["child_jobs"].as_array().unwrap().len(), 2);
        assert_eq!(payload["children"]["done"], json!(2));
        // Exactly the tuples became children — same kinds, same params,
        // same order (ids ascend in submission order), nothing more (the
        // probes the routes needed are daemon jobs, not children of the
        // apply) — and the payload's child_jobs echoes them in that order.
        let jobs = match door.request(Request::List).await {
            Response::Jobs { jobs } => jobs,
            other => panic!("the list answered {other:?}"),
        };
        let children: Vec<(JobId, String, Value, Option<String>)> = jobs
            .into_iter()
            .filter(|job| job.parent == Some(id))
            .map(|job| (job.id, job.kind, job.params, job.account))
            .collect();
        let tuples: Vec<(&str, &Value)> = children
            .iter()
            .map(|(_, kind, params, _)| (kind.as_str(), params))
            .collect();
        assert_eq!(
            tuples,
            [
                ("stashes", &json!({ "league": "Standard" })),
                (
                    "stash",
                    &json!({ "league": "Standard", "id": "dump", "deep": false })
                ),
            ],
            "children must be the admitted tuples verbatim, in order"
        );
        let child_ids: Vec<JobId> = children.iter().map(|(id, ..)| *id).collect();
        assert_eq!(payload["child_jobs"], json!(child_ids));
        // Children run as the parent's account, and their responses landed
        // in that account's store file.
        assert!(
            children
                .iter()
                .all(|(_, _, _, a)| a.as_deref() == Some("Alice#1234"))
        );
        let store = Store::open(&account_path(&dir, "Alice#1234")).unwrap();
        assert_eq!(store.status().unwrap().responses, 2);
        drop(store);
        door.finish().await;
    });
}

#[tokio::test]
async fn a_cancelled_parent_cannot_gain_children() {
    // The race: cancel() enumerates children while the fan-out loop
    // keeps submitting. submit_child now refuses under the same lock
    // cancel takes.
    let (daemon, log) = test_daemon(Boundary::Function, refused(), Arc::new(ParkingClock::new()));
    let parent = daemon
        .submit(
            "stash".into(),
            json!({ "league": "Standard", "id": "t1", "deep": true }),
            0,
            "t".into(),
            None,
        )
        .unwrap();
    {
        let mut s = daemon.shared.lock().unwrap();
        s.jobs.get_mut(&parent).unwrap().info.state = JobState::Running;
    }
    daemon.cancel(parent).unwrap();
    let err = daemon.submit_child(parent, "stash", json!({})).unwrap_err();
    assert!(err.message.contains("cancelled"), "{err}");
    finish_harness(None, &log);
}

#[tokio::test]
async fn a_parent_cancelled_after_its_last_child_is_not_a_success() {
    let (daemon, log) = test_daemon(Boundary::Function, refused(), Arc::new(ParkingClock::new()));
    let parent = daemon
        .submit(
            "stash".into(),
            json!({ "league": "Standard", "id": "t1", "deep": true }),
            0,
            "t".into(),
            None,
        )
        .unwrap();
    {
        let mut s = daemon.shared.lock().unwrap();
        s.jobs.get_mut(&parent).unwrap().info.state = JobState::Running;
    }
    let _child = daemon.submit_child(parent, "stash", json!({})).unwrap();
    // The cancellation lands after the fan-out submitted its last
    // child but before the parent's own outcome is installed.
    daemon.cancel(parent).unwrap();
    daemon.conclude(
        parent,
        Outcome::Success {
            payload: json!({ "tabs_listed": 1 }),
        },
    );
    let (info, outcome) = held(&daemon, parent).unwrap();
    assert_eq!(info.state, JobState::Cancelled, "never a success");
    assert!(matches!(outcome, Some(Outcome::Cancelled)));
    finish_harness(None, &log);
}

#[tokio::test]
async fn cancelled_children_do_not_make_a_parent_successful() {
    let (daemon, log) = test_daemon(Boundary::Function, refused(), Arc::new(ParkingClock::new()));
    let parent = daemon
        .submit(
            "stash".into(),
            json!({ "league": "Standard", "id": "t1", "deep": true }),
            0,
            "t".into(),
            None,
        )
        .unwrap();
    {
        let mut s = daemon.shared.lock().unwrap();
        s.jobs.get_mut(&parent).unwrap().info.state = JobState::Running;
    }
    let child = daemon.submit_child(parent, "stash", json!({})).unwrap();
    daemon.cancel(child).unwrap();
    daemon.conclude(parent, Outcome::Success { payload: json!({}) });
    let Outcome::Failure {
        error,
        kind: Some(FailureKind::Children),
        ..
    } = held(&daemon, parent).unwrap().1.unwrap()
    else {
        panic!("a parent with cancelled children must not succeed")
    };
    assert!(error.contains("cancelled"), "{error}");
    finish_harness(None, &log);
}

#[tokio::test]
async fn a_cancellation_pending_at_finish_wins_the_terminal_state() {
    // The gap this pins: a caller (conclude, maybe_finish_parent)
    // computes an outcome, releases the lock, and a cancel lands
    // before finish takes the final lock. The stale success must not
    // be written.
    let (daemon, log) = test_daemon(Boundary::Function, refused(), Arc::new(ParkingClock::new()));
    let id = daemon
        .submit("sleep".into(), json!({}), 0, "t".into(), None)
        .unwrap();
    {
        let mut s = daemon.shared.lock().unwrap();
        s.jobs.get_mut(&id).unwrap().info.state = JobState::Running;
    }
    daemon.cancel(id).unwrap();
    daemon.finish(
        id,
        Outcome::Success {
            payload: json!({ "stale": true }),
        },
    );
    let (info, outcome) = held(&daemon, id).unwrap();
    assert_eq!(info.state, JobState::Cancelled);
    assert!(matches!(outcome, Some(Outcome::Cancelled)));
    finish_harness(None, &log);
}
