//! C6 as amended 2026-10-27: a concluded tree leaves memory, and the table
//! answers for it. Each test is named for the schedule it stages; none
//! runs a dispatcher or a clock — the jobs are concluded by hand, as the
//! dispatcher's `finish` and `conclude` would.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::*;

/// What a client reads for `req`, as JSON: the daemon's answer whole.
async fn answer(daemon: &Arc<Daemon>, req: Request) -> Value {
    serde_json::to_value(daemon.handle_request(req, &mut None).await).unwrap()
}

fn in_memory(daemon: &Daemon, id: JobId) -> bool {
    daemon.shared.lock().unwrap().jobs.contains_key(&id)
}

/// A parent fanned out to two children, the parent running.
fn fan_out(daemon: &Daemon) -> (JobId, JobId, JobId) {
    let parent = daemon
        .submit(
            "stash".into(),
            json!({ "league": "Standard", "id": "t1", "deep": true }),
            0,
            "t".into(),
            None,
        )
        .unwrap();
    daemon
        .shared
        .lock()
        .unwrap()
        .jobs
        .get_mut(&parent)
        .unwrap()
        .info
        .state = JobState::Running;
    let quick = daemon
        .submit_child(parent, "stash", json!({ "id": "a" }))
        .unwrap();
    let slow = daemon
        .submit_child(parent, "stash", json!({ "id": "b" }))
        .unwrap();
    (parent, quick, slow)
}

fn done(tabs: u64) -> Outcome {
    Outcome::Success {
        payload: json!({ "tabs_listed": tabs }),
    }
}

/// A parent whose one child is still slow: the quick child concluded and
/// the parent's own result is held, and an eviction attempted on either
/// finds the tree live, so all three stay in memory (the parent's tally
/// scans its children). When the slow child concludes, the parent does,
/// and the whole tree leaves, its tally counting both children.
#[tokio::test]
async fn a_parent_with_one_slow_child_keeps_its_tree_in_memory_until_it_concludes() {
    let (daemon, log) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
    let (parent, quick, slow) = fan_out(&daemon);
    daemon.finish(quick, done(1));
    daemon.conclude(parent, done(2));
    {
        let mut s = daemon.shared.lock().unwrap();
        daemon.evict_tree(&mut s, quick);
        daemon.evict_tree(&mut s, parent);
    }
    for id in [parent, quick, slow] {
        assert!(in_memory(&daemon, id), "job {id} left with its tree live");
    }
    assert_eq!(state_of(&daemon, parent), JobState::Running);

    daemon.finish(slow, done(1));
    for id in [parent, quick, slow] {
        assert!(
            !in_memory(&daemon, id),
            "job {id} stayed after its tree concluded"
        );
    }
    let (info, outcome) = held(&daemon, parent).unwrap();
    assert_eq!(info.state, JobState::Done);
    let Some(Outcome::Success { payload }) = outcome else {
        panic!("the parent did not succeed: {outcome:?}")
    };
    assert_eq!(payload["children"]["done"], json!(2), "{payload}");
    finish_harness(None, &log);
}

/// A tree concluded, persisted and evicted answers `status`, `result`,
/// the list, `cancel` and `set-priority` from the table in the documents
/// memory gave: a child's answers read while its tree was live are the
/// same after; the parent's status is its last event, the one clients
/// saw as it concluded.
#[tokio::test]
async fn an_evicted_tree_answers_from_the_table_as_memory_did() {
    let (daemon, log) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
    let (parent, quick, slow) = fan_out(&daemon);
    daemon.finish(quick, done(1));
    daemon.conclude(parent, done(2));
    let status = || Request::Status { id: quick };
    let result = || Request::Result { id: quick };
    let before = (
        answer(&daemon, status()).await,
        answer(&daemon, result()).await,
    );
    assert_eq!(before.0["resp"], json!("status"), "{}", before.0);
    assert_eq!(before.1["resp"], json!("result"), "{}", before.1);
    let mut events = daemon.events.subscribe();

    daemon.finish(slow, done(1));
    assert!(!in_memory(&daemon, parent) && !in_memory(&daemon, quick));
    assert_eq!(
        (
            answer(&daemon, status()).await,
            answer(&daemon, result()).await
        ),
        before,
        "the evicted child's status and result"
    );
    let mut last = None;
    while let Ok(info) = events.try_recv() {
        if info.id == parent {
            last = Some(info);
        }
    }
    let last = last.expect("the parent's conclusion was an event");
    assert_eq!(
        answer(&daemon, Request::Status { id: parent }).await,
        json!({ "resp": "status", "job": last }),
        "the evicted parent's status is its last event"
    );
    let parent_result = answer(&daemon, Request::Result { id: parent }).await;
    assert_eq!(
        parent_result["outcome"]["payload"]["children"]["done"],
        json!(2),
        "{parent_result}"
    );
    let listed: Vec<(JobId, JobState)> = daemon
        .job_list()
        .unwrap()
        .iter()
        .map(|job| (job.id, job.state))
        .collect();
    assert_eq!(
        listed,
        [
            (parent, JobState::Done),
            (quick, JobState::Done),
            (slow, JobState::Done)
        ],
        "the list names the evicted tree"
    );
    let cancel = daemon.cancel(parent).unwrap_err();
    assert_eq!(
        (cancel.kind, cancel.message.as_str()),
        (
            ErrorKind::WrongState,
            format!("job {parent} already done").as_str()
        )
    );
    let priority = daemon.set_priority(quick, 3).unwrap_err();
    assert_eq!(
        (priority.kind, priority.message.as_str()),
        (
            ErrorKind::WrongState,
            format!("job {quick} is done, not waiting").as_str()
        )
    );
    assert_eq!(
        answer(&daemon, Request::Status { id: slow + 1 }).await["kind"],
        json!("unknown_job"),
        "an id never issued is still no job"
    );
    finish_harness(None, &log);
}

/// A thousand single jobs concluded leave none in memory: the daemon's
/// memory is bounded by its live work, not by its lifetime, while the
/// list still names every one of them, finished, and any of them
/// answers from the table.
#[tokio::test]
async fn a_thousand_single_jobs_concluded_leave_none_in_memory() {
    let (daemon, log) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
    let ids: Vec<JobId> = (0..1000)
        .map(|_| {
            daemon
                .submit("sleep".into(), json!({}), 0, "t".into(), None)
                .unwrap()
        })
        .collect();
    for &id in &ids {
        daemon.start_and_finish(id, done(0));
    }
    assert!(
        daemon.shared.lock().unwrap().jobs.is_empty(),
        "every concluded job left memory"
    );
    let listed = daemon.job_list().unwrap();
    assert_eq!(listed.len(), 1000);
    assert!(listed.iter().all(|job| job.state == JobState::Done));
    assert_eq!(
        listed.iter().map(|job| job.id).collect::<Vec<_>>(),
        ids,
        "by id"
    );
    for id in [ids[0], ids[499], ids[999]] {
        let (info, outcome) = held(&daemon, id).unwrap();
        assert_eq!(info.state, JobState::Done);
        assert!(
            matches!(outcome, Some(Outcome::Success { .. })),
            "{outcome:?}"
        );
    }
    finish_harness(None, &log);
}
