//! `List`, `Status` and the dashboard's jobs: each job's eta is what a
//! per-job walk of the queue would say, and the whole list costs one
//! queue order and one forward simulation per route. C166: `List` is a
//! page of one state, newest first, cut by its row limit and its bytes,
//! beside the counts of the whole list.
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
    let clock = Arc::new(ParkingClock::new());
    let (transport, _mock) = crate::transport::mock(&Hold::new(), clock.clone());
    let base = BASE;
    let (daemon, _log_path) = test_daemon(Boundary::Function, transport, clock);
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
/// answers (about 0.3 s in a debug build, printed, asserted by nothing:
/// C137, no wall-clock upper bound a slow machine can miss).
/// The per-job walk took 22 s for 5,000 jobs on an unlearned route in a
/// debug build, and 18 s for only 1,000 on a learned one, whose per-job
/// simulation grew with the cube of the queue; one simulation that kept
/// every simulated send still took 5.7 s for 20,000. All of it under
/// `shared`, so the dispatcher and every other request waited behind
/// one listing. The listing runs on its own thread, so a hung one fails
/// at the hang ceiling, not never.
#[tokio::test]
async fn list_of_twenty_thousand_jobs_answers() {
    const JOBS: usize = 20_000;
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
    match finished.recv_timeout(HANG) {
        Ok((listed, took)) => {
            assert_eq!(listed, JOBS);
            eprintln!("List of {JOBS} jobs took {took:?}");
        }
        Err(_) => panic!("List of {JOBS} jobs did not answer within {HANG:?}"),
    }
}

/// One `List` answer to `request`, a frame as a client writes it: the
/// page's ids and states, its `next` and its counts.
async fn page(
    daemon: &Arc<Daemon>,
    request: Value,
) -> (Vec<(JobId, JobState)>, Option<JobId>, ListCounts) {
    let request: Request = serde_json::from_value(request).unwrap();
    match daemon.handle_request(request, &mut None).await {
        Response::Jobs { jobs, next, counts } => (
            jobs.iter().map(|job| (job.id, job.state)).collect(),
            next,
            counts,
        ),
        other => panic!("the list answered {other:?}"),
    }
}

/// C166 (the job list is paged): a bare `list` is the open jobs, newest
/// first; `finished` reads the finished, those that left memory from the
/// table; `all` pages every job once by `after`, the last page with no
/// `next`; and every answer counts the whole list, the evicted jobs
/// included, whatever its page leaves out. A limit outside 1 to the
/// maximum is refused.
#[tokio::test]
async fn c166_a_page_is_one_state_newest_first_and_the_counts_are_the_whole_lists() {
    use JobState::*;
    let (daemon, log) = test_daemon(Boundary::Function, refused(), Arc::new(ParkingClock::new()));
    let ids: Vec<JobId> = (0..8)
        .map(|_| {
            daemon
                .submit("sleep".into(), json!({}), 0, "t".into(), None)
                .unwrap()
        })
        .collect();
    assert_eq!(ids, [1, 2, 3, 4, 5, 6, 7, 8]);
    for id in [1, 2, 3] {
        daemon.start_and_finish(id, Outcome::Success { payload: json!({}) });
    }
    daemon.start_and_finish(4, Outcome::failure(FailureKind::Internal, "broken"));
    daemon.cancel(5).unwrap();
    daemon
        .shared
        .lock()
        .unwrap()
        .jobs
        .get_mut(&6)
        .unwrap()
        .info
        .state = Running;
    let held: Vec<JobId> = {
        let mut held: Vec<JobId> = daemon.shared.lock().unwrap().jobs.keys().copied().collect();
        held.sort_unstable();
        held
    };
    assert_eq!(
        held,
        [6, 7, 8],
        "the finished left memory: the table answers for them"
    );
    let whole = ListCounts {
        total: 8,
        running: 1,
        waiting: 2,
        done: 3,
        failed: 1,
        cancelled: 1,
        // a sleep sends nothing: it starts now, which is no wait
        soonest_eta_seconds: None,
        latest_eta_seconds: None,
    };

    let bare = page(&daemon, json!({ "req": "list" })).await;
    assert_eq!(
        bare,
        (
            vec![(8, Waiting), (7, Waiting), (6, Running)],
            None,
            whole.clone()
        )
    );
    let finished = page(&daemon, json!({ "req": "list", "state": "finished" })).await;
    assert_eq!(
        finished,
        (
            vec![(5, Cancelled), (4, Failed), (3, Done), (2, Done), (1, Done)],
            None,
            whole.clone()
        )
    );
    let mut pages = Vec::new();
    let mut after: Option<JobId> = None;
    loop {
        let mut request = json!({ "req": "list", "state": "all", "limit": 3 });
        if let Some(after) = after {
            request["after"] = json!(after);
        }
        let (jobs, next, counts) = page(&daemon, request).await;
        assert_eq!(counts, whole, "each page counts the whole list");
        pages.push(jobs.iter().map(|(id, _)| *id).collect::<Vec<_>>());
        match next {
            Some(next) => after = Some(next),
            None => break,
        }
    }
    assert_eq!(pages, [vec![8, 7, 6], vec![5, 4, 3], vec![2, 1]]);
    let finished_after = page(
        &daemon,
        json!({ "req": "list", "state": "finished", "limit": 2, "after": 4 }),
    )
    .await;
    assert_eq!(finished_after.0, [(3, Done), (2, Done)]);
    assert_eq!(finished_after.1, Some(2), "a job remains below the page");

    for limit in [0, LIST_LIMIT_MAX + 1] {
        let refused = daemon
            .handle_request(
                Request::List {
                    state: ListState::Open,
                    limit,
                    after: None,
                },
                &mut None,
            )
            .await;
        match refused {
            Response::Error { kind, message } => {
                assert_eq!(kind, ErrorKind::BadRequest, "{message}");
                assert_eq!(
                    message,
                    format!("a list page holds 1 to {LIST_LIMIT_MAX} jobs; {limit} were asked for")
                );
            }
            other => panic!("a limit of {limit} answered {other:?}"),
        }
    }
    finish_harness(None, &log);
}

/// C166: a page stops at its byte budget before its row limit, and
/// carries one row however small the budget. A page is chosen by each
/// row's bound (`row_bound`): each job's params are 10,000 bytes as JSON
/// and its kind and submitter six, so its bound is 10,000, 36 and 512,
/// and with its comma 10,549: a budget of 25,000 bytes holds two rows of
/// a limit of ten, and one of 5,000 bytes holds one; `next` resumes after
/// the last row held.
#[tokio::test]
async fn c166_a_page_stops_at_its_byte_budget_before_its_limit() {
    let (daemon, log) = test_daemon(Boundary::Function, refused(), Arc::new(ParkingClock::new()));
    // {"pad":"…"} is the pad's length and ten bytes
    let params = json!({ "pad": "x".repeat(9_990) });
    assert_eq!(serde_json::to_string(&params).unwrap().len(), 10_000);
    for _ in 0..4 {
        daemon
            .submit("sleep".into(), params.clone(), 0, "t".into(), None)
            .unwrap();
    }
    let ids = |page: &crate::daemon::queue::JobPage| {
        page.jobs.iter().map(|job| job.id).collect::<Vec<_>>()
    };
    let two = daemon
        .job_page_within(ListState::Open, 10, None, 25_000)
        .unwrap();
    assert_eq!((ids(&two), two.next), (vec![4, 3], Some(3)));
    let rest = daemon
        .job_page_within(ListState::Open, 10, two.next, 25_000)
        .unwrap();
    assert_eq!((ids(&rest), rest.next), (vec![2, 1], None));
    let one = daemon
        .job_page_within(ListState::Open, 10, None, 5_000)
        .unwrap();
    assert_eq!((ids(&one), one.next), (vec![4], Some(4)));
    assert_eq!(one.counts.total, 4, "the counts are the whole list's");
    finish_harness(None, &log);
}

/// C166, the review of fef5eadb, P1: a page is chosen before its rows
/// are built, and no row past it is built, from memory or from the
/// table: the count is of every row made, a clone from memory or a whole
/// row read from the table, kept or thrown away (the review of
/// d9be0d31: a count of the rows returned could not fail). Six jobs of 10,000 bytes of params each, the three oldest
/// finished and gone from memory: a page of `all` under a budget of two
/// rows builds two (both memory's), one of `finished` with a limit of one
/// builds one (the table's), and one reaching across both sources under
/// a budget of three builds three.
#[tokio::test]
async fn c166_no_row_past_a_page_is_built() {
    use std::sync::atomic::Ordering;
    let (daemon, log) = test_daemon(Boundary::Function, refused(), Arc::new(ParkingClock::new()));
    let params = json!({ "pad": "x".repeat(9_990) });
    for _ in 0..6 {
        daemon
            .submit("sleep".into(), params.clone(), 0, "t".into(), None)
            .unwrap();
    }
    for id in [1, 2, 3] {
        daemon.start_and_finish(id, Outcome::Success { payload: json!({}) });
    }
    let built = |state, limit, after, budget| {
        let before = daemon.list_rows_built.load(Ordering::Relaxed);
        let page = daemon.job_page_within(state, limit, after, budget).unwrap();
        let ids: Vec<JobId> = page.jobs.iter().map(|job| job.id).collect();
        (
            ids,
            page.next,
            daemon.list_rows_built.load(Ordering::Relaxed) - before,
        )
    };
    assert_eq!(
        built(ListState::All, 10, None, 25_000),
        (vec![6, 5], Some(5), 2)
    );
    assert_eq!(
        built(
            ListState::Finished,
            1,
            None,
            crate::daemon::queue::LIST_PAGE_BUDGET
        ),
        (vec![3], Some(3), 1)
    );
    assert_eq!(
        built(ListState::All, 10, Some(5), 35_000),
        (vec![4, 3, 2], Some(2), 3)
    );
    finish_harness(None, &log);
}

/// C166: a row's bound holds its widest row: every number at its widest,
/// every string of control characters (six bytes each as JSON), the
/// times set, params of nested escapes.
#[test]
fn c166_a_rows_bound_holds_its_widest_row() {
    let text = "\u{1}".repeat(128);
    let widest = JobInfo {
        id: JobId::MAX,
        kind: text.clone(),
        state: JobState::Cancelled,
        priority: Priority::MAX,
        submitted_by: text.clone(),
        eta_seconds: Some(u64::MAX),
        parent: Some(JobId::MAX),
        retries: u32::MAX,
        account: Some(text),
        params: json!({ "s": "\u{1}\"\\".repeat(100), "n": [i64::MIN, u64::MAX] }),
        times: JobTimes {
            submitted_at: Some(i64::MIN),
            started_at: Some(i64::MIN),
            finished_at: Some(i64::MIN),
            ..JobTimes::default()
        },
    };
    let row = crate::daemon::queue::json_len(&widest);
    let bound = crate::daemon::queue::info_bound(&widest);
    assert!(
        row <= bound,
        "a row of {row} bytes over its bound of {bound}"
    );
    // the fixed part alone is held with room
    let bare = JobInfo {
        kind: String::new(),
        submitted_by: String::new(),
        account: None,
        params: json!(null),
        ..widest
    };
    let fixed = crate::daemon::queue::json_len(&bare);
    assert!(fixed < 400, "the fixed part is {fixed} bytes");
}
