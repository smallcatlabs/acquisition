//! Parent jobs and apply: the fan-out and its admission, and a parent's end
//! against its children's.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::*;
use std::sync::atomic::Ordering;

/// The apply parent (tracer step 7): a plan's action set executed
/// exactly — one child per admitted tuple, no send of its own, no
/// expansion — with the children's responses recorded through the
/// store like any other fetch.
#[tokio::test]
async fn apply_fans_out_exactly_its_tuples_and_records_through_the_store() {
    let base = mockggg::start().await.unwrap();
    let clock = Arc::new(ManualClock::new());
    let (mut daemon, log_path) = test_daemon(&base, clock);
    let store_dir = std::env::temp_dir().join(format!(
        "acq-apply-{}-{}",
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
    let id = daemon
        .submit(
            "apply".into(),
            json!({ "jobs": [
                { "kind": "stashes", "params": { "league": "Standard" } },
                { "kind": "stash", "params": { "league": "Standard", "id": "dump", "deep": false } },
            ] }),
            0,
            "test".into(),
            Some("Alice#1234".into()),
        )
        .unwrap();
    let (info, outcome) = wait_terminal(&daemon, id).await;
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
    let children: Vec<(JobId, String, Value, Option<String>)> = {
        let s = daemon.shared.lock().unwrap();
        let mut c: Vec<_> = s
            .jobs
            .values()
            .filter(|e| e.info.parent == Some(id))
            .map(|e| {
                (
                    e.info.id,
                    e.info.kind.clone(),
                    e.params.clone(),
                    e.info.account.clone(),
                )
            })
            .collect();
        c.sort_by_key(|(id, ..)| *id);
        c
    };
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
    let store = Store::open(&account_path(&store_dir, "Alice#1234")).unwrap();
    assert_eq!(store.status().unwrap().responses, 2);
    drop(store);
    finish_harness_mock(dispatcher, &log_path);
    let _ = std::fs::remove_dir_all(&store_dir);
}

/// C43 — admission is vocabulary, not meaning, at submit before a job id exists; C42 — a logical bound over `max_requests` refuses whole, nothing submitted.
/// Apply's admission (C43): vocabulary and
/// budget are checked at submit, before a job id exists, so a refusal
/// admits nothing — never a partial fan-out, and ids never advance.
#[tokio::test]
async fn apply_admission_refuses_bad_vocabulary_and_blown_budgets_whole() {
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon("http://127.0.0.1:9", clock);
    // The whole vocabulary (characters joined 2026-09-02): a character
    // list under poe2 is a legal tuple — the character family takes it.
    let ok_jobs = json!([
        { "kind": "stashes", "params": { "league": "Standard" } },
        { "kind": "stash", "params": { "league": "Standard", "id": "t1", "deep": false } },
        { "kind": "characters", "params": { "realm": "poe2" } },
        { "kind": "character", "params": { "realm": "pc", "name": "Exile" } },
    ]);
    let refusals: Vec<(Value, &str)> = vec![
        // The tuple list itself is required and non-empty.
        (json!({}), "needs a `jobs` array"),
        (json!({ "jobs": [] }), "empty"),
        // Only single-request kinds: a nested parent (apply), the retired
        // ad-hoc refresh, or any other kind would expand or sidestep the
        // reviewed set.
        (
            json!({ "jobs": [{ "kind": "refresh", "params": { "all": true } }] }),
            "not in the plan vocabulary",
        ),
        (
            json!({ "jobs": [{ "kind": "apply", "params": { "jobs": [] } }] }),
            "not in the plan vocabulary",
        ),
        (
            json!({ "jobs": [{ "kind": "sleep", "params": {} }] }),
            "not in the plan vocabulary",
        ),
        // A stash fetch that fans out expands the set; one without an
        // id could not have come from a plan.
        (
            json!({ "jobs": [{ "kind": "stash", "params": { "league": "Standard", "id": "m1", "deep": true } }] }),
            "never fans out",
        ),
        (
            json!({ "jobs": [{ "kind": "stash", "params": { "league": "Standard" } }] }),
            "needs an id",
        ),
        // A character fetch without a name could not have come from a
        // plan either.
        (
            json!({ "jobs": [{ "kind": "character", "params": { "realm": "pc" } }] }),
            "needs a name",
        ),
        // The budget: a bound under the tuple count refuses; a misread
        // bound refuses too, never "the limit was ignored".
        (
            json!({ "jobs": ok_jobs.clone(), "max_requests": 1 }),
            "exceeds the budget",
        ),
        (
            json!({ "jobs": ok_jobs.clone(), "max_requests": "ten" }),
            "non-negative integer",
        ),
    ];
    for (params, expected) in refusals {
        let err = daemon
            .submit("apply".into(), params.clone(), 0, "test".into(), None)
            .unwrap_err();
        assert!(err.message.contains(expected), "{params}: {err}");
        assert!(
            daemon.shared.lock().unwrap().jobs.is_empty(),
            "{params}: a refused apply must admit nothing"
        );
    }
    // A budget the plan fits inside admits (no dispatcher runs here,
    // so the job just sits waiting) — and it gets the daemon's first
    // id, proving the refusals above consumed none.
    let id = daemon
        .submit(
            "apply".into(),
            json!({ "jobs": ok_jobs, "max_requests": 4 }),
            0,
            "test".into(),
            None,
        )
        .unwrap();
    assert_eq!(id, 1, "refused applies must not advance job ids");
    assert_eq!(
        daemon.shared.lock().unwrap().jobs[&id].info.state,
        JobState::Waiting
    );
    finish_harness(None, &log_path);
}

#[tokio::test]
async fn a_cancelled_parent_cannot_gain_children() {
    // The race: cancel() enumerates children while the fan-out loop
    // keeps submitting. submit_child now refuses under the same lock
    // cancel takes.
    let (daemon, log) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
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
    let (daemon, log) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
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
    let (state, outcome) = {
        let s = daemon.shared.lock().unwrap();
        let e = &s.jobs[&parent];
        (e.info.state, e.outcome.clone())
    };
    assert_eq!(state, JobState::Cancelled, "never a success");
    assert!(matches!(outcome, Some(Outcome::Cancelled)));
    finish_harness(None, &log);
}

#[tokio::test]
async fn cancelled_children_do_not_make_a_parent_successful() {
    let (daemon, log) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
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
    let Outcome::Failure { error, .. } = daemon.shared.lock().unwrap().jobs[&parent]
        .outcome
        .clone()
        .unwrap()
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
    let (daemon, log) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
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
    let (state, outcome) = {
        let s = daemon.shared.lock().unwrap();
        let e = &s.jobs[&id];
        (e.info.state, e.outcome.clone())
    };
    assert_eq!(state, JobState::Cancelled);
    assert!(matches!(outcome, Some(Outcome::Cancelled)));
    finish_harness(None, &log);
}
