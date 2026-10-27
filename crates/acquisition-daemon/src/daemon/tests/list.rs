//! `List`, `Status` and the dashboard's jobs: each job's eta is what a
//! per-job walk of the queue would say, and the whole list costs one
//! queue order and one forward simulation per route.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::*;

/// The per-job snapshot as it stood before `List` computed the queue
/// once (C4: a job's eta is part of its state): the queue order rebuilt
/// for one job, the same-route jobs ahead of it counted by walking that
/// order, and its route's simulation run to that count. The oracle the
/// one-pass `list` and `snapshot` are held to.
fn per_job_snapshot(s: &Shared, daemon: &Daemon, id: JobId) -> Option<JobInfo> {
    let queue = s.queue_order();
    let entry = s.jobs.get(&id)?;
    let mut info = entry.info.clone();
    info.eta_seconds = if info.state == JobState::Waiting {
        match daemon.keyed_route_for(&info.kind, &entry.info.params, info.account.as_deref()) {
            Some((route, _)) => {
                let ahead = queue
                    .iter()
                    .take_while(|&&q| q != id)
                    .filter(|q| {
                        s.jobs
                            .get(q)
                            .and_then(|e| {
                                daemon.keyed_route_for(
                                    &e.info.kind,
                                    &e.info.params,
                                    e.info.account.as_deref(),
                                )
                            })
                            .is_some_and(|(r, _)| r == route)
                    })
                    .count();
                Some(daemon.choke.eta_for(&route, ahead as u32).as_secs())
            }
            None => Some(0),
        }
    } else {
        None
    };
    Some(info)
}

/// A daemon whose mock `fetch` policy is learned, for `alice` too, so
/// the queue's etas are forward simulations rather than zeros.
async fn learned_daemon() -> Arc<Daemon> {
    let clock = Arc::new(ManualClock::new());
    let base = mockggg::start_with_clock(clock.clone()).await.unwrap();
    let (daemon, _log_path) = test_daemon(&base, clock);
    for route in ["fetch", "fetch@alice"] {
        daemon
            .choke
            .head(
                route,
                &format!("{base}/fetch"),
                None,
                daemon.choke.now(),
                ALWAYS_WANTED,
            )
            .await
            .unwrap();
    }
    daemon
}

fn submit(daemon: &Daemon, kind: &str, params: Value, priority: Priority, account: Option<&str>) {
    daemon
        .submit(
            kind.into(),
            params,
            priority,
            "list-test".into(),
            account.map(str::to_string),
        )
        .unwrap();
}

fn as_json(jobs: &[JobInfo]) -> String {
    serde_json::to_string(jobs).unwrap()
}

/// The one-pass `list` reports, byte for byte, what the per-job walk
/// reports, on a queue that mixes a learned route, the same route under
/// another account, an unlearned route and a kind that sends nothing,
/// across priorities, with running, finished, failed and cancelled jobs
/// among the waiting; `status` agrees for every job.
#[tokio::test]
async fn list_matches_the_per_job_walk_on_a_mixed_queue() {
    let daemon = learned_daemon().await;
    let priorities = [0, 5, 1, 10, 5, 0, 1];
    for i in 0..84 {
        let priority = priorities[i % priorities.len()];
        match i % 6 {
            0 | 3 => submit(&daemon, "fetch", json!({}), priority, None),
            1 => submit(&daemon, "fetch", json!({}), priority, Some("alice")),
            2 => submit(&daemon, "characters", json!({}), priority, None),
            4 => submit(&daemon, "sleep", json!({ "seconds": 1.0 }), priority, None),
            _ => submit(&daemon, "leagues", json!({}), priority, None),
        }
    }
    {
        let mut s = daemon.shared.lock().unwrap();
        for (id, state) in [
            (2, JobState::Running),
            (7, JobState::Running),
            (11, JobState::Done),
            (16, JobState::Failed),
            (23, JobState::Cancelled),
            (40, JobState::Running),
            (61, JobState::Done),
        ] {
            s.jobs.get_mut(&id).unwrap().info.state = state;
        }
    }

    let s = daemon.shared.lock().unwrap();
    let mut ids: Vec<JobId> = s.jobs.keys().copied().collect();
    ids.sort();
    let walked: Vec<JobInfo> = ids
        .iter()
        .filter_map(|&id| per_job_snapshot(&s, &daemon, id))
        .collect();
    let listed = s.list(&daemon);
    assert_eq!(as_json(&listed), as_json(&walked));
    for &id in &ids {
        let status = s.snapshot(&daemon, id).unwrap();
        let oracle = per_job_snapshot(&s, &daemon, id).unwrap();
        assert_eq!(as_json(&[status]), as_json(&[oracle]), "job {id}");
    }

    // The queue is not trivial: a learned route's deep jobs wait, and
    // only waiting jobs carry an eta.
    let etas: Vec<u64> = listed.iter().filter_map(|j| j.eta_seconds).collect();
    assert!(etas.iter().any(|&eta| eta >= 60), "{etas:?}");
    assert!(etas.contains(&0), "{etas:?}");
    assert!(
        listed
            .iter()
            .all(|j| (j.state == JobState::Waiting) == j.eta_seconds.is_some()),
        "{listed:?}"
    );
}

/// `List` over 20,000 waiting jobs, nine in ten on a learned route,
/// answers well inside a generous bound (about 0.3 s in a debug build).
/// The per-job walk took 22 s for 5,000 jobs on an unlearned route in a
/// debug build, and 18 s for only 1,000 on a learned one, whose per-job
/// simulation grew with the cube of the queue; one simulation that kept
/// every simulated send still took 5.7 s for 20,000. All of it under
/// `shared`, so the dispatcher and every other request waited behind
/// one listing. The listing runs on its own thread so a slow one fails
/// at the bound, not when it ends.
#[tokio::test]
async fn list_of_twenty_thousand_jobs_answers_inside_the_bound() {
    const JOBS: usize = 20_000;
    const BOUND: Duration = Duration::from_secs(2);
    let daemon = learned_daemon().await;
    for i in 0..JOBS {
        let kind = if i % 10 == 9 { "characters" } else { "fetch" };
        submit(&daemon, kind, json!({}), (i % 3) as Priority, None);
    }

    let (done, finished) = std::sync::mpsc::channel();
    let listing = daemon.clone();
    std::thread::spawn(move || {
        let started = std::time::Instant::now();
        let jobs = listing.shared.lock().unwrap().list(&listing);
        let _ = done.send((jobs.len(), started.elapsed()));
    });
    match finished.recv_timeout(BOUND) {
        Ok((listed, took)) => {
            assert_eq!(listed, JOBS);
            eprintln!("List of {JOBS} jobs took {took:?}");
        }
        Err(_) => panic!("List of {JOBS} jobs did not answer within {BOUND:?}"),
    }
}
