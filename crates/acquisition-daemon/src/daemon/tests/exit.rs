//! C133, the one exit: what the drain waits for, what it turns away, what
//! the bound cuts, and the refusal and the idle verdict around it. The
//! scenarios whose inputs and outputs are the frontend boundary's run
//! through the door (C158, a daemon scenario crosses the daemon's
//! frontend boundary only), in virtual time on a parking clock the
//! runtime moves; the rest are component tests, each saying which other
//! boundary it needs, on a parking clock they drive by hand
//! ([`drive_drain`]). None calls `leave`, which ends the process: the
//! leave is recorded. C134, the panic's end: the hook
//! that writes the log line and the fault marker, and the restore that
//! fails the marker's job once, or closes its probe's route for the
//! cooldown. C134, the listener: an accept error is
//! served through, logged with a backoff.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::door::*;
use super::harness::*;
use crate::daemon::life::{Cause, Effect, Event, Life, State, Timer};
use crate::daemon::server::{InFlight, LifeProbe};
use crate::daemon::*;

/// Long enough for the drains that must finish, short beside a hold the
/// drain must not wait out (the limiter's 30 s). Virtual: on the
/// daemon's clock.
const BOUND: Duration = Duration::from_secs(3);

/// A logged-in harness daemon on a parking clock nothing drives (a
/// deadline already due is ready at once; any other parks until the test
/// moves the clock), its queue on a file-backed `daemon.db`, its exit
/// bounded by `bound`.
fn exit_daemon(base: &str, bound: Duration) -> (Arc<Daemon>, TestPath, TestPath) {
    exit_daemon_on(base, bound, Arc::new(ParkingClock::new()))
}

/// A client's submit, as `Request::Submit` carries it.
fn submit_request(kind: &str, params: Value, account: Option<&str>) -> Request {
    Request::Submit {
        kind: kind.into(),
        params,
        priority: 0,
        submitted_by: "test".into(),
        account: account.map(str::to_string),
    }
}

/// The job a submit answered with.
fn submitted(response: Response) -> JobId {
    match response {
        Response::Submitted { id } => id,
        other => panic!("expected a submitted job, got {other:?}"),
    }
}

/// Job `id` as the protocol's `Status` reads it.
async fn status_of(door: &mut Door, id: JobId) -> JobInfo {
    match door.request(Request::Status { id }).await {
        Response::Status { job } => job,
        other => panic!("job {id}'s status reads {other:?}"),
    }
}

/// The journal's methods, in order.
fn journal_methods(door: &Door) -> Vec<String> {
    door.journal()
        .iter()
        .map(|send| {
            send["method"]
                .as_str()
                .expect("a journal method")
                .to_string()
        })
        .collect()
}

/// [`exit_daemon`] on `clock`.
fn exit_daemon_on(
    base: &str,
    bound: Duration,
    clock: Arc<dyn crate::ratelimit::Clock>,
) -> (Arc<Daemon>, TestPath, TestPath) {
    let db = test_db_path();
    let (mut daemon, log_path) = test_daemon(base, clock);
    logged_in(&mut daemon);
    let d = Arc::get_mut(&mut daemon).unwrap();
    d.jobs_db = Mutex::new(JobDb::open(&db).unwrap());
    d.exit = Exit::with_bound(bound);
    (daemon, log_path, db)
}

fn submit_for(daemon: &Daemon, kind: &str, params: Value, account: Option<&str>) -> JobId {
    daemon
        .submit(
            kind.into(),
            params,
            0,
            "test".into(),
            account.map(str::to_string),
        )
        .unwrap()
}

/// The job's row as `daemon.db` holds it, read through a connection of
/// its own.
fn row_of(db: &std::path::Path, id: JobId) -> acquisition_store::jobs::JobRow {
    JobDb::open(db).unwrap().get(id).unwrap().expect("the row")
}

async fn wait_state(daemon: &Daemon, id: JobId, state: JobState) {
    tokio::time::timeout(HANG, async {
        while state_of(daemon, id) != state {
            // The poll's interval: a component test on a plain runtime has no
            // park hook to wake it when the state it waits for holds.
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap_or_else(|_| panic!("job {id} reached {state}"));
}

/// The drain ([`drain`]), on a task of its own, so a test can see it
/// still waiting.
fn spawn_drain(daemon: &Arc<Daemon>) -> tokio::task::JoinHandle<Option<InFlight>> {
    tokio::spawn(drain(daemon))
}

/// C133: a job whose GET is on the wire when the exit begins is waited
/// for. Through the door: the account server holds the GET; the stop
/// comes while it is held, so the send in flight keeps the clock still
/// while the drain parks its look; released, the body lands, the exit
/// leaves before its bound on the clock (a drain cut at `HELD` reads
/// the bound), and the job is Done by the protocol with its POST and GET
/// in the journal, the journal equal to the wire.
#[test]
fn exit_waits_for_a_landed_body_to_persist() {
    let clock = Arc::new(ParkingClock::new());
    let parking = clock.clone();
    let hook = Door::open(
        Server::Account(("GET", "/profile", 0)),
        clock,
        Arc::new(RecordingCredentialStore::default()),
    )
    .sessions(logged_in_sessions())
    .exit_bound(HELD)
    .life(IDLE_SHUTDOWN)
    .run(|mut door| async move {
        let clock = parking;
        let Fixture::Account {
            sent: wire,
            arrived,
            release,
        } = door.fixture()
        else {
            unreachable!("the door faces the account server")
        };
        let (wire, arrived, release) = (wire.clone(), arrived.take(), release.take());
        let id = submitted(
            door.request(submit_request("profile", json!({}), Some("test-user")))
                .await,
        );
        tokio::time::timeout(HANG, arrived.unwrap())
            .await
            .expect("the GET reached the server")
            .unwrap();

        let stopped = clock.now();
        door.stop();
        release.unwrap().send(()).unwrap();
        door.await_leave().await;
        let drained = clock.now() - stopped;
        assert!(
            drained < HELD,
            "a clean drain leaves before its bound: {drained:?}"
        );

        assert_eq!(status_of(&mut door, id).await.state, JobState::Done);
        assert_eq!(
            wire.lock().unwrap().as_slice(),
            [sent("test-user", "POST"), sent("test-user", "GET")]
        );
        assert_eq!(journal_methods(&door), wire_methods(&wire));
        door.finish().await;
    });
    eprintln!("a held drain: the park hook: {hook:?}");
}

/// C133: a token refresh on the wire when the exit begins is waited for.
/// Through the door: a client's `auth check` asks the refresh, which the
/// delayed token server holds; the stop comes while it is held;
/// released, the token lands and the exit leaves before its bound on the
/// clock (a drain cut at `HELD` reads the bound). `auth status`, read
/// at the leave, has the access token's expiry moved (none before the
/// refresh), the check is answered, and the journal holds the one POST.
#[test]
fn exit_waits_for_a_refresh_and_installs_the_token() {
    let clock = Arc::new(ParkingClock::new());
    let parking = clock.clone();
    Door::open(
        Server::Token(Token::Delayed(
            "200 OK",
            token_body("at-new", "rt-rotated", "test-user"),
        )),
        clock,
        Arc::new(RecordingCredentialStore::default()),
    )
    .sessions(logged_in_sessions())
    .exit_bound(HELD)
    .life(IDLE_SHUTDOWN)
    .run(|mut door| async move {
        let clock = parking;
        let before = door.await_auth(|_| true).await;
        let Fixture::Delayed {
            arrived, release, ..
        } = door.fixture()
        else {
            unreachable!("the door faces the delayed token server")
        };
        let (arrived, release) = (arrived.take(), release.take());
        let check = door.spawn_request(Request::AuthCheck { account: None });
        tokio::time::timeout(HANG, arrived.unwrap())
            .await
            .expect("the refresh reached the server")
            .unwrap();

        let stopped = clock.now();
        door.stop();
        release.unwrap().send(()).unwrap();
        door.await_leave().await;
        let drained = clock.now() - stopped;
        assert!(
            drained < HELD,
            "a clean drain leaves before its bound: {drained:?}"
        );

        let after = door.await_auth(|_| true).await;
        assert_eq!(before.access_expires_in_seconds, None, "{before:?}");
        assert!(
            after
                .access_expires_in_seconds
                .is_some_and(|left| left > 3500),
            "the refreshed token's hour, installed by the leave: {after:?}"
        );
        let checked = tokio::time::timeout(HANG, check)
            .await
            .expect("the check is answered")
            .unwrap();
        assert!(
            matches!(
                checked,
                Response::Auth {
                    logged_in: true,
                    ..
                }
            ),
            "{checked:?}"
        );
        assert_eq!(journal_methods(&door), ["POST"]);
        door.finish().await;
    });
}

/// Two accounts, with `other`'s refresh held on the wire and
/// `test-user`'s profile job parked Running behind it at the token
/// route's gate. Returns the parked job, `other`'s job whose POST is held,
/// and what was sent before the park, with the parking clock the drain is
/// driven on.
struct Parked {
    clock: Arc<ParkingClock>,
    daemon: Arc<Daemon>,
    log_path: TestPath,
    db: TestPath,
    dispatcher: tokio::task::JoinHandle<()>,
    sent_log: Arc<Mutex<Vec<Sent>>>,
    release: tokio::sync::oneshot::Sender<()>,
    parked: JobId,
    first: JobId,
    warmed: Vec<Sent>,
}

async fn park_a_job_at_the_token_route() -> Parked {
    let (base, sent_log, arrived, release) = account_server(("POST", "/token", 1)).await;
    let clock = Arc::new(ParkingClock::new());
    let (daemon, log_path, db) = exit_daemon_on(&base, BOUND, clock.clone());
    daemon.shared.lock().unwrap().auth.by_account.insert(
        "other".into(),
        AuthSession {
            username: Some("other".into()),
            refresh_token: Some("rt-other".into()),
            ..Default::default()
        },
    );
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let warm = submit_for(&daemon, "profile", json!({}), Some("other"));
    wait_state(&daemon, warm, JobState::Done).await;
    daemon
        .shared
        .lock()
        .unwrap()
        .auth
        .find_mut("other")
        .unwrap()
        .access_token = None;
    let first = submit_for(&daemon, "profile", json!({}), Some("other"));
    arrived.await.unwrap();
    let parked = submit_for(&daemon, "profile", json!({}), Some("test-user"));
    wait_state(&daemon, parked, JobState::Running).await;
    // Long enough for the parked refresh to reach the gate's queue. It
    // stays a sleep: the exact signal is the door's `quiet` (the runtime
    // parked under the held POST), and this fixture's two users are
    // component tests of the exit's park on a plain runtime, which has no
    // park hook.
    tokio::time::sleep(Duration::from_millis(50)).await;
    let warmed = vec![
        sent("other", "POST"),
        sent("other", "GET"),
        sent("other", "POST"),
    ];
    assert_eq!(
        sent_log.lock().unwrap().as_slice(),
        warmed,
        "the parked refresh has sent nothing"
    );
    Parked {
        clock,
        daemon,
        log_path,
        db,
        dispatcher,
        sent_log,
        release,
        parked,
        first,
        warmed,
    }
}

/// C133 with C25: a job parked Running before its send, here behind
/// another account's refresh at the token route's gate, is turned away
/// by the exit and returns to waiting with no retry spent, in memory and
/// on disk. The held POST, already on the wire, lands; the job it was
/// for is turned away before its GET. Nothing more is sent. A component
/// test (C158): it stages the park through the daemon's internals and
/// reads `active_jobs`' outcome in memory and the rows on disk.
#[tokio::test]
async fn exit_turns_away_a_job_parked_past_running() {
    let p = park_a_job_at_the_token_route().await;
    assert!(p.daemon.begin_exit("test: exit"));
    let drain = spawn_drain(&p.daemon);
    p.release.send(()).unwrap();
    let cut = drive_drain(&p.clock, &p.daemon, drain).await.unwrap();
    assert_eq!(cut, None, "nothing was cut");

    for id in [p.parked, p.first] {
        let info = p.daemon.shared.lock().unwrap().jobs[&id].info.clone();
        assert_eq!(info.state, JobState::Waiting, "job {id} in memory");
        assert_eq!(info.retries, 0, "job {id}: no retry spent");
        let row = row_of(&p.db, id);
        assert_eq!(row.state, JobState::Waiting.to_string(), "job {id} on disk");
        assert_eq!(row.retries, 0, "job {id} on disk");
        assert_eq!(row.outcome, None, "job {id} on disk");
    }
    assert_eq!(
        p.sent_log.lock().unwrap().as_slice(),
        p.warmed,
        "nothing was sent after the exit began"
    );
    assert_journal_matches_wire(&p.log_path, &wire_methods(&p.sent_log));
    finish_harness(p.dispatcher, &p.log_path);
}

/// C133: the dispatcher starts nothing while the daemon exits. A job the
/// exit turned away is not picked again (the drain would spin, a queue
/// write per pass), and a job queued afterwards, even one of a kind that
/// never sends, is not started. A component test (C158): the job queued
/// after the stop is an internal submit, which the protocol refuses, and
/// the pick is read directly.
#[tokio::test]
async fn a_turned_away_job_is_not_picked_again_while_exiting() {
    let p = park_a_job_at_the_token_route().await;
    assert!(p.daemon.begin_exit("test: exit"));
    let drain = spawn_drain(&p.daemon);
    p.release.send(()).unwrap();
    assert_eq!(drive_drain(&p.clock, &p.daemon, drain).await.unwrap(), None);
    assert_eq!(state_of(&p.daemon, p.parked), JobState::Waiting);

    let local = submit_for(&p.daemon, "sleep", json!({ "seconds": 0 }), None);
    assert!(
        p.daemon.pick_runnable().is_empty(),
        "the pick takes nothing"
    );
    assert_eq!(state_of(&p.daemon, local), JobState::Waiting);
    let log = std::fs::read_to_string(&p.log_path).unwrap();
    for id in [p.parked, local] {
        let runs = log.matches(&format!("job {id} -> running")).count();
        assert!(runs <= 1, "job {id} ran {runs} times:\n{log}");
    }
    assert_journal_matches_wire(&p.log_path, &wire_methods(&p.sent_log));
    finish_harness(p.dispatcher, &p.log_path);
}

/// C133: a job whose task does not end is cut at the bound. Through the
/// door: a `sleep` job of a minute, real time by its kind and counted by
/// nothing the hook reads, is Running when the stop comes; the drain's
/// looks find it each time, virtual time reaches the bound, and the exit
/// leaves with the job still Running by the protocol (left on disk for
/// the next daemon, C6), the bound reached on the daemon's clock. That
/// the cut comes at the bound and not before is the table's row (the
/// drain's deadline cuts) and
/// `c137_the_drain_bound_at_zero_cuts_what_is_in_flight`'s wiring.
#[test]
fn exit_cuts_a_stuck_job_at_the_bound() {
    let bound = Duration::from_millis(300);
    let clock = Arc::new(ParkingClock::new());
    let parking = clock.clone();
    let hook = Door::open(
        Server::None,
        clock,
        Arc::new(RecordingCredentialStore::default()),
    )
    .exit_bound(bound)
    .life(IDLE_SHUTDOWN)
    .run(|mut door| async move {
        let clock = parking;
        let id = submitted(
            door.request(submit_request("sleep", json!({ "seconds": 60 }), None))
                .await,
        );
        door.await_job(id, |job| job.state == JobState::Running)
            .await;

        let began = clock.now();
        door.stop();
        door.await_leave().await;

        assert_eq!(status_of(&mut door, id).await.state, JobState::Running);
        let waited = clock.now() - began;
        assert!(waited >= bound, "the drain ran to its bound: {waited:?}");
        assert_eq!(journal_methods(&door), Vec::<String>::new());
        door.finish().await;
    });
    eprintln!("the cut: the park hook: {hook:?}");
}

/// C133: once the exit has begun a client's submit is refused, as
/// `refused`, in words that say to try again; the same submit was
/// accepted before. Through the door: the submit, the stop, the submit
/// again, the exit's leave.
#[test]
fn a_submit_after_the_exit_begins_is_refused_as_stopping() {
    Door::open(
        Server::None,
        Arc::new(ParkingClock::new()),
        Arc::new(RecordingCredentialStore::default()),
    )
    .exit_bound(BOUND)
    .life(IDLE_SHUTDOWN)
    .run(|mut door| async move {
        let submit = || submit_request("sleep", json!({ "seconds": 0 }), None);
        assert!(matches!(
            door.request(submit()).await,
            Response::Submitted { .. }
        ));
        door.stop();
        match door.request(submit()).await {
            Response::Error { kind, message } => {
                assert_eq!(kind, ErrorKind::Refused);
                assert_eq!(message, "the daemon is stopping; try again in a moment");
            }
            other => panic!("expected the stopping refusal, got {other:?}"),
        }
        door.await_leave().await;
        door.finish().await;
    });
}

/// C133: a login in progress holds the idle verdict. Its browser step and
/// code exchange belong to no connection and no job, and an idle exit
/// in the middle would lose the login.
#[test]
fn the_idle_verdict_holds_while_a_login_is_pending() {
    let clock = Arc::new(ParkingClock::new());
    let (daemon, log_path) = test_daemon("http://127.0.0.1:1", clock.clone());
    let knob = Duration::from_secs(60);
    clock.advance(knob);
    assert!(daemon.is_idle(knob), "nothing holds the daemon");
    daemon.begin_auth_flow();
    assert!(!daemon.is_idle(knob), "a pending login holds it");
    daemon.shared.lock().unwrap().auth.pending = None;
    assert!(daemon.is_idle(knob), "the login over, it is idle again");
    finish_harness(None, &log_path);
}

/// C133: the exit begins once. A second cause returns at once and leaves
/// the first's bound where it was. A component test (C158): it reads the
/// exit's private deadline, an instant on the daemon's clock.
#[tokio::test]
async fn a_second_begin_exit_returns_at_once() {
    let (daemon, log_path, _db) = exit_daemon("http://127.0.0.1:1", BOUND);
    assert!(daemon.begin_exit("test: the first cause"));
    let deadline = *daemon.exit.deadline.get().expect("the bound is armed");
    assert!(!daemon.begin_exit("test: the second cause"));
    assert_eq!(daemon.exit.deadline.get(), Some(&deadline));
    let log = std::fs::read_to_string(&log_path).unwrap();
    assert!(
        !log.contains("the second cause"),
        "the second cause began nothing:\n{log}"
    );
    finish_harness(None, &log_path);
}

/// C133: the bound is hard. A keyring prompt holds `shared`, and tasks
/// that take it (here the idle verdict, twice) block both workers of the
/// runtime, so no tokio timer runs and the drain cannot reach its bound.
/// The bound's own thread still ends the exit, not before the bound and
/// its grace, with the lock still held until the test lets it go. The
/// bound thread is real by design, and this is its one pin. A component
/// test (C158): several workers, and the bound's own OS thread on the
/// wall's time. The drain's first look is registered while the lock is
/// held, before the blocking tasks saturate the workers; the drain is
/// driven by hand once the holder lets go ([`drive_drain`]).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_bound_ends_the_exit_though_every_worker_is_blocked() {
    let bound = Duration::from_millis(300);
    let grace = Duration::from_millis(200);
    let clock = Arc::new(ParkingClock::new());
    let (mut daemon, log_path, _db) = exit_daemon_on("http://127.0.0.1:1", bound, clock.clone());
    let (ended_tx, ended_rx) = std::sync::mpsc::channel();
    {
        let d = Arc::get_mut(&mut daemon).unwrap();
        d.exit.grace = grace;
        let ended_tx = Mutex::new(ended_tx);
        let end: EndAtOnce = Arc::new(move |why: &str, code: i32| {
            let _ = ended_tx.lock().unwrap().send((why.to_string(), code));
        });
        let _ = d.exit.end.set(end);
    }
    // The prompt: `shared` held on a thread of its own.
    let (held_tx, held_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel::<()>();
    let holder = {
        let daemon = daemon.clone();
        std::thread::spawn(move || {
            let _shared = daemon.shared.lock().unwrap();
            held_tx.send(()).unwrap();
            let _ = release_rx.recv();
        })
    };
    held_rx.recv().unwrap();
    let drain = spawn_drain(&daemon);
    // The first look parks, `shared` being held, before the workers do.
    await_a_look(&clock).await;
    let blocked: Vec<_> = (0..2)
        .map(|_| {
            let daemon = daemon.clone();
            tokio::spawn(async move { daemon.is_idle(Duration::from_secs(60)) })
        })
        .collect();
    // It stays a thread sleep: the runtime's two workers are blocked on
    // `shared`, so no task can signal that both reached the lock; long
    // enough for both to.
    std::thread::sleep(Duration::from_millis(100));

    let began = std::time::Instant::now();
    assert!(daemon.begin_exit("test: exit"));
    let (why, code) = ended_rx
        .recv_timeout(HANG)
        .expect("the bound ended the exit with every worker blocked");
    let waited = began.elapsed();
    assert!(
        waited >= bound + grace,
        "the end came at the bound and its grace: {waited:?}"
    );
    assert!(
        daemon.shared.try_lock().is_err(),
        "the prompt still held the lock"
    );
    release_tx.send(()).unwrap();
    assert_eq!(code, 1);
    assert!(why.contains("had not left"), "{why}");
    assert!(why.contains("with no checkpoint"), "{why}");

    holder.join().unwrap();
    for task in blocked {
        task.await.unwrap();
    }
    let _ = drive_drain(&clock, &daemon, drain).await;
    finish_harness(None, &log_path);
}

/// C133 with C25: a job picked before the exit and held at the limiter
/// before its send (here behind a 429's hold, whose pacing slices park on
/// the daemon's clock) is turned away: the drain does not wait out the
/// hold, and the exit leaves before its `BOUND` on the clock (a drain that
/// waited out the 30 s hold would be cut at it), the job Waiting with only
/// the 429's retry spent, in memory and on disk, and nothing sent but
/// the probe and the one GET. The stop comes only once the state it
/// interrupts is seen: the job requeued with its retry, picked again and
/// its pacing wait registered on the clock. A component test (C158): it
/// reads `active_jobs`, the clock's sleepers and the queue's row through
/// a connection of its own, and drives the clock by hand: to the pacing
/// wait's own deadline, below the bound, then through [`drive_drain`].
#[tokio::test]
async fn exit_turns_away_a_job_held_at_the_limiter() {
    let (base, requests, server) = scripted_server(vec![
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse::full("GET", "429 Too Many Requests", Some(30), "{}"),
    ])
    .await;
    let clock = Arc::new(ParkingClock::new());
    let (daemon, log_path, db) = exit_daemon_on(&base, BOUND, clock.clone());
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = submit_for(&daemon, "fetch", json!({}), None);
    tokio::time::timeout(HANG, async {
        loop {
            let held = {
                let s = daemon.shared.lock().unwrap();
                let info = &s.jobs[&id].info;
                info.retries == 1
                    && info.state == JobState::Waiting
                    && s.active_jobs.contains_key(&id)
            };
            if held && clock.sleepers() >= 1 {
                break;
            }
            // The poll's interval: a component test on a plain runtime has no
            // park hook to wake it when the state it waits for holds.
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    })
    .await
    .expect("the job is held at the limiter's pacing wait after its 429");
    server.await.unwrap();
    assert_eq!(clock.sleepers(), 1, "only the pacing wait is parked");
    let pacing = clock.next_deadline().expect("the pacing wait's deadline");

    let stopped = clock.now();
    assert!(daemon.begin_exit("test: exit"));
    assert!(
        pacing < stopped + BOUND,
        "the pacing wait wakes before the bound"
    );
    clock.advance_to(pacing);
    let cut = drive_drain(&clock, &daemon, drain(&daemon)).await;
    let drained = clock.now() - stopped;

    assert_eq!(cut, None, "nothing was cut");
    assert!(
        drained < BOUND,
        "the drain did not wait out the hold: it left after {drained:?}"
    );
    let info = daemon.shared.lock().unwrap().jobs[&id].info.clone();
    assert_eq!(info.state, JobState::Waiting);
    assert_eq!(info.retries, 1, "the exit spent no retry");
    let row = row_of(&db, id);
    assert_eq!(row.state, JobState::Waiting.to_string(), "on disk");
    assert_eq!(row.retries, 1, "the 429's retry, on disk");
    let log = std::fs::read_to_string(&log_path).unwrap();
    assert!(
        log.contains(&format!(
            "job {id} (fetch): turned away before its send by the exit"
        )),
        "the held pacing task was turned away:\n{log}"
    );
    assert_eq!(requests.lock().unwrap().as_slice(), ["HEAD", "GET"]);
    assert_journal_matches_wire(&log_path, &requests.lock().unwrap());
    finish_harness(dispatcher, &log_path);
}

/// C133: a login's code exchange on the wire when the exit begins is
/// waited for. Through the door: the login is asked through the
/// protocol, the browser's callback reaches the daemon's own listener,
/// and the scenario's own token server holds the exchange; the stop
/// comes while it is held; released, the exchange lands and the exit
/// leaves before its bound on the clock (a drain cut at `HELD` reads the
/// bound). The clock is held across the callback, a wait no count of the
/// daemon's sees. The journal holds the exchange's POST, and the profile job
/// the login queued after the stop is left Waiting by the protocol (a
/// stopping daemon starts nothing, and a waiting profile does not hold
/// the drain). That the login then registers is the staged login's
/// component test,
/// `c133_a_stop_between_the_profile_and_the_login_waits_for_the_login`.
#[test]
fn exit_waits_for_a_login_code_exchange() {
    let clock = Arc::new(ParkingClock::new());
    let parking = clock.clone();
    let (arrived_tx, arrived) = tokio::sync::oneshot::channel();
    let (release, release_rx) = tokio::sync::oneshot::channel::<()>();
    // The scenario's own server, which holds the exchange: bound and
    // spawned inside the door's runtime; its barriers are the scenario's.
    let (served_tx, served) = tokio::sync::oneshot::channel();
    let own = Server::own(move || async move {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let request = mockggg::read_request(&mut stream).await.unwrap();
            assert_eq!(
                (request.method.as_str(), request.path.as_str()),
                ("POST", "/token")
            );
            assert!(request.body.contains("grant_type=authorization_code"));
            arrived_tx.send(()).unwrap();
            release_rx.await.unwrap();
            let body = json!({
                "access_token": "at-new",
                "refresh_token": "rt-new",
                "expires_in": 3600,
                "username": "test-user",
            })
            .to_string();
            let headers = TOKEN_HEADERS;
            mockggg::respond_with(&mut stream, "200 OK", "application/json", headers, &body).await;
        });
        served_tx.send(server).unwrap();
        base
    });
    Door::open(own, clock, Arc::new(RecordingCredentialStore::default()))
        .exit_bound(HELD)
        .life(IDLE_SHUTDOWN)
        .run(|mut door| async move {
            let clock = parking;
            let server = served.await.expect("the door built the scenario's server");

            // The callback and the exchange's way to the wire are waits no
            // count of the daemon's sees: the clock is held across them.
            let hold = door.hold_clock();
            let url = match door.request(Request::AuthStart).await {
                Response::AuthUrl { authorize_url } => authorize_url,
                other => panic!("expected the authorize URL, got {other:?}"),
            };
            let authorize = url::Url::parse(&url).unwrap();
            let query: HashMap<String, String> = authorize.query_pairs().into_owned().collect();
            let callback = format!(
                "{}?code=code-x&state={}",
                query["redirect_uri"], query["state"]
            );
            assert!(reqwest::get(&callback).await.unwrap().status().is_success());
            tokio::time::timeout(HANG, arrived)
                .await
                .expect("the exchange reached the server")
                .unwrap();
            drop(hold);

            let stopped = clock.now();
            door.stop();
            release.send(()).unwrap();
            door.await_leave().await;
            let drained = clock.now() - stopped;
            assert!(
                drained < HELD,
                "a clean drain leaves before its bound: {drained:?}"
            );

            let jobs = match door.request(Request::List).await {
                Response::Jobs { jobs } => jobs,
                other => panic!("expected the job list, got {other:?}"),
            };
            let profiles: Vec<_> = jobs.iter().filter(|job| job.kind == "profile").collect();
            assert_eq!(
                profiles.len(),
                1,
                "the login queued its profile job: {jobs:?}"
            );
            assert_eq!(profiles[0].state, JobState::Waiting, "{jobs:?}");
            tokio::time::timeout(HANG, server)
                .await
                .expect("the token server was answered")
                .unwrap();
            assert_eq!(journal_methods(&door), ["POST"]);
            door.finish().await;
        });
}

/// C133 (a pending login has the bound to land), issue #59: a stop that
/// lands after the login's profile job finished and before the login
/// registered waits for the login. The gap is forced: the profile job
/// runs to done, and `complete_login` is not called until the drain has
/// been seen waiting on the login; then the drain returns, with nothing
/// cut, and the session is registered. A component test (C158): the
/// login is staged and completed by hand, through the daemon's internals.
#[tokio::test]
async fn c133_a_stop_between_the_profile_and_the_login_waits_for_the_login() {
    let (base, sent_log, _arrived, _release) = account_server(("GET", "/never", 0)).await;
    let clock = Arc::new(ParkingClock::new());
    let (daemon, log_path, _db) = exit_daemon_on(&base, HELD, clock.clone());
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let generation = daemon.begin_auth_flow();
    daemon
        .stage_auth_flow(
            generation,
            auth::TokenResponse {
                access_token: "at-newcomer".into(),
                refresh_token: "rt-newcomer".into(),
                expires_in: 3600,
                username: "newcomer".into(),
            },
        )
        .expect("current flow stages");
    assert!(
        InFlight::of(&daemon.shared.lock().unwrap()).login,
        "a staged login not yet submitted is the drain's"
    );
    let profile = daemon.submit_login_profile(generation, "newcomer").unwrap();
    wait_state(&daemon, profile, JobState::Done).await;

    assert!(daemon.begin_exit("test: exit"));
    let drain = spawn_drain(&daemon);
    // The drain's look parks on the clock, the login in flight.
    await_a_look(&clock).await;
    assert!(
        !drain.is_finished(),
        "the drain waits for the login whose profile landed"
    );
    assert!(InFlight::of(&daemon.shared.lock().unwrap()).login);
    assert!(daemon.complete_login(generation, "newcomer", "uuid-newcomer"));
    assert_eq!(
        drive_drain(&clock, &daemon, drain).await.unwrap(),
        None,
        "nothing was cut"
    );
    assert!(
        daemon
            .shared
            .lock()
            .unwrap()
            .auth
            .find("newcomer")
            .is_some(),
        "the login registered"
    );
    assert_eq!(
        sent_log.lock().unwrap().as_slice(),
        [sent("newcomer", "GET")]
    );
    finish_harness(dispatcher, &log_path);
}

/// C133: a login whose profile job a stop turned away to waiting is not
/// the drain's: the job is on disk for the next lifetime, which fails it
/// as a lost login's (C50), and the drain does not stall to its bound.
#[test]
fn a_login_whose_profile_job_waits_is_not_held_by_the_drain() {
    let (daemon, log_path) = test_daemon("http://127.0.0.1:1", Arc::new(ParkingClock::new()));
    let generation = daemon.begin_auth_flow();
    daemon
        .stage_auth_flow(
            generation,
            auth::TokenResponse {
                access_token: "at-newcomer".into(),
                refresh_token: "rt-newcomer".into(),
                expires_in: 3600,
                username: "newcomer".into(),
            },
        )
        .expect("current flow stages");
    daemon.submit_login_profile(generation, "newcomer").unwrap();
    assert!(!InFlight::of(&daemon.shared.lock().unwrap()).login);
    finish_harness(None, &log_path);
}

// ---- C157: the lifecycle as a table of event scripts -------------------

/// One transition's row: the script, the effects the rulings expect
/// (written from C3, C133 and the code's comments, never from a run),
/// and where the lifecycle stands after.
struct Row {
    name: &'static str,
    script: Vec<Event>,
    expect: Vec<Effect>,
    state: State,
    cause: Option<Cause>,
}

fn table() -> Vec<Row> {
    use Effect::{Arm, AwaitQuiet, Begin, Cancel, Cut, Judge, Leave, StopAdmitting};
    let signal = |won: bool| Event::Signal {
        won,
        why: "SIGTERM received; exiting".into(),
    };
    let stop = |won: bool| Event::StopRequested { won };
    let armed = |won: bool| Event::ExitArmed { won };
    let begun = || vec![Cancel(Timer::Idle), Arm(Timer::Drain), AwaitQuiet];
    // From the start: the idle deadline armed.
    let serving = |then: Vec<Effect>| {
        let mut effects = vec![Arm(Timer::Idle)];
        effects.extend(then);
        effects
    };
    // The exit begun by a stop, then `script`, asking for `then`.
    let stopped = |script: Vec<Event>, then: Vec<Effect>| {
        let mut events = vec![stop(true)];
        events.extend(script);
        let mut effects = serving(begun());
        effects.extend(then);
        (events, effects)
    };
    // The exit begun by the idle verdict, then `script`, asking nothing.
    let idled = |script: Vec<Event>| {
        let mut events = vec![Event::Deadline(Timer::Idle), Event::Idle, armed(true)];
        events.extend(script);
        let mut effects = serving(vec![Judge, Begin]);
        effects.extend(begun());
        (events, effects)
    };
    let row = |name, (script, expect): (Vec<Event>, Vec<Effect>), state, cause| Row {
        name,
        script,
        expect,
        state,
        cause,
    };
    let serving_row = |name, script: Vec<Event>, then: Vec<Effect>| Row {
        name,
        script,
        expect: serving(then),
        state: State::Serving,
        cause: None,
    };
    let by_stop = Some(Cause::Stop);
    let every_event = || {
        vec![
            Event::Deadline(Timer::Idle),
            Event::Deadline(Timer::Drain),
            Event::Idle,
            Event::Busy,
            armed(true),
            armed(false),
            stop(true),
            stop(false),
            signal(true),
            signal(false),
            Event::Quiet,
        ]
    };
    vec![
        // Serving: the idle deadline and the verdict (C3).
        serving_row(
            "serving: the idle deadline judged busy is armed again",
            vec![Event::Deadline(Timer::Idle), Event::Busy],
            vec![Judge, Arm(Timer::Idle)],
        ),
        row(
            "serving: judged idle, the swap won, the exit is begun by idle",
            idled(vec![]),
            State::Draining,
            Some(Cause::Idle),
        ),
        row(
            "serving: judged idle, the swap lost, nothing until the winner's stop begins it",
            (
                vec![
                    Event::Deadline(Timer::Idle),
                    Event::Idle,
                    armed(false),
                    stop(true),
                ],
                serving(vec![
                    Judge,
                    Begin,
                    Cancel(Timer::Idle),
                    Arm(Timer::Drain),
                    AwaitQuiet,
                ]),
            ),
            State::Draining,
            by_stop,
        ),
        // Serving: the first cause (C133), its swap made at its ingress.
        row(
            "serving: a stop begins the exit",
            stopped(vec![], vec![]),
            State::Draining,
            by_stop,
        ),
        row(
            "serving: a signal begins the exit, its admission stop first",
            (
                vec![signal(true)],
                serving(vec![
                    StopAdmitting("SIGTERM received; exiting".into()),
                    Cancel(Timer::Idle),
                    Arm(Timer::Drain),
                    AwaitQuiet,
                ]),
            ),
            State::Draining,
            Some(Cause::Signal),
        ),
        serving_row(
            "serving: a stop whose swap lost does nothing, its winner's event to come",
            vec![stop(false)],
            vec![],
        ),
        serving_row(
            "serving: a signal whose swap lost does nothing, its winner's event to come",
            vec![signal(false)],
            vec![],
        ),
        serving_row(
            "serving: a quiet or a drain deadline before the exit does nothing",
            vec![Event::Quiet, Event::Deadline(Timer::Drain)],
            vec![],
        ),
        // Draining: the second cause does nothing (C133, one exit), and
        // nothing leaves until quiet or the drain's deadline.
        row(
            "draining: a stop after idle, its swap lost, does nothing",
            idled(vec![stop(false)]),
            State::Draining,
            Some(Cause::Idle),
        ),
        row(
            "draining: a stop after idle reported won does nothing",
            idled(vec![stop(true)]),
            State::Draining,
            Some(Cause::Idle),
        ),
        row(
            "draining: a stop after a stop, its swap lost, does nothing",
            stopped(vec![stop(false)], vec![]),
            State::Draining,
            by_stop,
        ),
        row(
            "draining: a stop after a stop reported won does nothing",
            stopped(vec![stop(true)], vec![]),
            State::Draining,
            by_stop,
        ),
        row(
            "draining: a signal after a stop, its swap lost, does nothing",
            stopped(vec![signal(false)], vec![]),
            State::Draining,
            by_stop,
        ),
        row(
            "draining: a signal after a stop reported won does nothing",
            stopped(vec![signal(true)], vec![]),
            State::Draining,
            by_stop,
        ),
        row(
            "draining: a late idle deadline, verdict or exit armed does nothing",
            stopped(
                vec![
                    Event::Deadline(Timer::Idle),
                    Event::Idle,
                    Event::Busy,
                    armed(true),
                    armed(false),
                ],
                vec![],
            ),
            State::Draining,
            by_stop,
        ),
        // Draining: the drain against its bound (C133), cut or clean.
        row(
            "draining: quiet before the drain's deadline leaves clean",
            stopped(vec![Event::Quiet], vec![Cancel(Timer::Drain), Leave]),
            State::Done,
            by_stop,
        ),
        row(
            "draining: the drain's deadline cuts then leaves",
            stopped(vec![Event::Deadline(Timer::Drain)], vec![Cut, Leave]),
            State::Done,
            by_stop,
        ),
        // Done: the leave is the last act.
        row(
            "done: every event after a clean leave does nothing",
            stopped(
                [vec![Event::Quiet], every_event()].concat(),
                vec![Cancel(Timer::Drain), Leave],
            ),
            State::Done,
            by_stop,
        ),
        row(
            "done: every event after a cut does nothing",
            stopped(
                [vec![Event::Deadline(Timer::Drain)], every_event()].concat(),
                vec![Cut, Leave],
            ),
            State::Done,
            by_stop,
        ),
    ]
}

/// C157: the daemon's lifecycle, one row per transition, with no clock:
/// its run time is the runner's report, asserted by nothing (C137).
#[test]
fn c157_the_lifecycle_is_a_table_of_event_scripts() {
    for row in table() {
        let mut machine = Life::new();
        assert_eq!(
            drive(&mut machine, &row.script),
            row.expect,
            "the row {:?}",
            row.name
        );
        assert_eq!(machine.state(), row.state, "the row {:?}", row.name);
        assert_eq!(machine.cause(), row.cause, "the row {:?}", row.name);
    }
}

/// Every row has a name of its own, so a failure names one transition.
#[test]
fn every_row_of_the_lifecycle_table_is_named_once() {
    let rows = table();
    let mut names: Vec<_> = rows.iter().map(|r| r.name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), rows.len());
}

// ---- C157: the lifecycle's driver, wired to a daemon --------------------
//
// Component tests (C158): each drives the driver through the test's
// probe (`LifeProbe`) or its recorded leave, on a parking clock nobody
// moves. A knob or a bound at zero is a deadline already due, ready on
// the sleep's first poll, so none advances the clock.

/// C137 (the knob at zero forces the expiry) with C3: the driver at the
/// idle knob zero leaves a quiet daemon (no connection, no job, no login,
/// the restore landed), by the idle exit's words, then the drain's. That
/// a busy verdict arms the deadline again is the table's row. A
/// component test (C158): the driver on its own, read through its leave
/// and the log.
#[tokio::test]
async fn c137_the_idle_deadline_at_zero_leaves_a_quiet_daemon() {
    let (daemon, log_path, _db) = exit_daemon("http://127.0.0.1:1", BOUND);
    let (life, leave) = spawn_life(&daemon, Duration::ZERO);
    let cut = tokio::time::timeout(HANG, leave)
        .await
        .expect("the idle deadline at zero leaves")
        .expect("the driver reached its leave");
    assert_eq!(cut, None, "nothing was in flight");
    life.await.unwrap();
    let log = std::fs::read_to_string(&log_path).unwrap();
    let idle = log
        .find("idle timeout; exiting")
        .unwrap_or_else(|| panic!("the idle exit's words:\n{log}"));
    let quiet = log
        .find("exit: nothing in flight")
        .unwrap_or_else(|| panic!("the drain's words:\n{log}"));
    assert!(idle < quiet, "the idle exit, then the drain:\n{log}");
    finish_harness(None, &log_path);
}

/// C3 with C157: at the idle knob zero, a daemon with a connection open is
/// judged busy, the driver has stepped that verdict, and the exit has not
/// begun. A component test (C158): the connection is a count set by
/// hand, the verdict read through the driver's probe; the test stops
/// after the first busy verdict, before any re-judge.
#[tokio::test]
async fn c3_a_connection_holds_a_daemon_under_the_idle_knob_at_zero() {
    let (daemon, log_path, _db) = exit_daemon("http://127.0.0.1:1", BOUND);
    daemon.connections.store(1, Ordering::SeqCst);
    let _ = daemon.life_probe.set(LifeProbe::default());
    let probe = daemon.life_probe.get().unwrap();
    let (_life, mut leave) = spawn_life(&daemon, Duration::ZERO);
    tokio::time::timeout(HANG, probe.busy.notified())
        .await
        .expect("the driver stepped a busy verdict");
    assert!(!daemon.exit.begun(), "the exit has not begun");
    assert!(
        matches!(
            leave.try_recv(),
            Err(tokio::sync::oneshot::error::TryRecvError::Empty)
        ),
        "a connection holds the daemon"
    );
    finish_harness(None, &log_path);
}

/// A connection admitted as `handle_conn` admits one: counted, then
/// stamped; and, when `closes`, closed again as it closes.
fn connect(daemon: &Daemon, closes: bool) {
    daemon.connections.admit();
    daemon.mark_activity(daemon.choke.now());
    if closes {
        daemon.mark_activity(daemon.choke.now());
        daemon.connections.close();
    }
}

/// C157 with C3, the idle commit claims the connections' word its verdict
/// judged: the judge answers idle; before the driver commits, a client is
/// admitted (and, when `closes`, closes again); the claim fails, the
/// driver steps the verdict as busy, and the exit does not begin. A
/// component test (C158): the admission is forced between the verdict and
/// the claim through the driver's probe.
async fn an_admission_before_the_claim_begins_no_exit(closes: bool) {
    let (daemon, log_path, _db) = exit_daemon("http://127.0.0.1:1", BOUND);
    let _ = daemon.life_probe.set(LifeProbe {
        hold_idle: true,
        ..LifeProbe::default()
    });
    let probe = daemon.life_probe.get().unwrap();
    let (_life, mut leave) = spawn_life(&daemon, Duration::ZERO);
    tokio::time::timeout(HANG, probe.idle.notified())
        .await
        .expect("the judge answered idle");
    connect(&daemon, closes);
    probe.go.notify_one();
    tokio::time::timeout(HANG, probe.busy.notified())
        .await
        .expect("the overtaken verdict was stepped as busy");
    assert!(!daemon.exit.begun(), "the stale verdict began no exit");
    assert!(!daemon.connections.claimed(), "the claim failed");
    assert!(
        matches!(
            leave.try_recv(),
            Err(tokio::sync::oneshot::error::TryRecvError::Empty)
        ),
        "the daemon serves on"
    );
    let log = std::fs::read_to_string(&log_path).unwrap();
    assert!(!log.contains("idle timeout; exiting"), "{log}");
    finish_harness(None, &log_path);
}

#[tokio::test]
async fn c157_an_idle_verdict_overtaken_by_a_connection_begins_no_exit() {
    an_admission_before_the_claim_begins_no_exit(false).await;
}

#[tokio::test]
async fn c157_an_idle_verdict_overtaken_by_a_connection_that_closed_begins_no_exit() {
    an_admission_before_the_claim_begins_no_exit(true).await;
}

/// C157 with C133, the other side of the claim: a client admitted after
/// the idle commit's claim and before its swap meets a daemon whose exit
/// stands. Its submit is refused as stopping before the swap and after
/// it; the driver, released, begins the exit and leaves. A component
/// test (C158): the admission is forced between the claim and the swap
/// through the driver's probe.
#[tokio::test]
async fn c157_a_connection_admitted_after_the_idle_claim_meets_the_exit() {
    let (daemon, log_path, _db) = exit_daemon("http://127.0.0.1:1", BOUND);
    let _ = daemon.life_probe.set(LifeProbe {
        hold_claim: true,
        ..LifeProbe::default()
    });
    let probe = daemon.life_probe.get().unwrap();
    let (life, leave) = spawn_life(&daemon, Duration::ZERO);
    tokio::time::timeout(HANG, probe.claimed.notified())
        .await
        .expect("the idle commit claimed the word");
    connect(&daemon, false);
    let submit = || Request::Submit {
        kind: "sleep".into(),
        params: json!({ "seconds": 0 }),
        priority: 0,
        submitted_by: "test".into(),
        account: None,
    };
    let stopping = |response: Response| match response {
        Response::Error { kind, message } => {
            assert_eq!(kind, ErrorKind::Refused);
            assert_eq!(message, "the daemon is stopping; try again in a moment");
        }
        other => panic!("expected the stopping refusal, got {other:?}"),
    };
    assert!(!daemon.exit.begun(), "the swap has not come yet");
    stopping(daemon.handle_request(submit(), &mut None).await);

    probe.go.notify_one();
    let cut = tokio::time::timeout(HANG, leave)
        .await
        .expect("the claimed exit leaves")
        .expect("the driver reached its leave");
    assert_eq!(cut, None, "nothing was in flight");
    life.await.unwrap();
    assert!(daemon.exit.begun(), "the claim stood: the exit began");
    stopping(daemon.handle_request(submit(), &mut None).await);
    let log = std::fs::read_to_string(&log_path).unwrap();
    assert!(log.contains("idle timeout; exiting"), "{log}");
    finish_harness(None, &log_path);
}

/// C137 (the bound at zero forces the expiry) with C133: a job running
/// when a stop is requested, the bound zero: the driver logs the cut,
/// naming the job, then leaves; the job's row is left running on disk
/// (C6). The table's row holds the order; this test, the wiring. A
/// component test (C158): the driver's leave read as what it cut, the
/// row on disk.
#[tokio::test]
async fn c137_the_drain_bound_at_zero_cuts_what_is_in_flight() {
    let (daemon, log_path, db) = exit_daemon("http://127.0.0.1:1", Duration::ZERO);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = submit_for(&daemon, "sleep", json!({ "seconds": 60 }), None);
    wait_state(&daemon, id, JobState::Running).await;

    let (life, leave) = spawn_life(&daemon, IDLE_SHUTDOWN);
    assert!(daemon.request_stop(), "the stop began the exit");
    let cut = tokio::time::timeout(HANG, leave)
        .await
        .expect("the bound at zero ends the drain")
        .expect("the driver reached its leave");
    assert_eq!(
        cut,
        Some(InFlight {
            jobs: vec![id],
            ..InFlight::default()
        })
    );
    life.await.unwrap();
    let log = std::fs::read_to_string(&log_path).unwrap();
    assert!(
        log.contains(&format!("cut job {id} (left running on disk")),
        "the log names the cut job:\n{log}"
    );
    assert!(log.contains("stop requested; exiting"), "{log}");
    assert_eq!(row_of(&db, id).state, JobState::Running.to_string());
    finish_harness(dispatcher, &log_path);
}

/// C133 (every cause reaches the one exit, none behind a lock) with C157:
/// a thread holds `shared` and keeps it while the driver, at the idle
/// knob zero, judges the verdict behind it; a signal's event, its swap
/// made as the watcher makes it, stops admission (the rails stopping, the
/// words logged) before the lock is let go. The watcher itself is Unix's
/// and not under test here: the event goes through the driver's own
/// channel. A component test (C158): several workers, one of them held
/// behind `shared`. The drain's first look is registered while the lock
/// is held; the drain is driven by hand once the lock is let go
/// ([`drive_drain`]).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn c133_a_signal_stops_admission_while_shared_is_held() {
    let clock = Arc::new(ParkingClock::new());
    let (daemon, log_path, _db) = exit_daemon_on("http://127.0.0.1:1", BOUND, clock.clone());
    let (held_tx, held_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel::<()>();
    let holder = {
        let daemon = daemon.clone();
        std::thread::spawn(move || {
            let _shared = daemon.shared.lock().unwrap();
            held_tx.send(()).unwrap();
            let _ = release_rx.recv();
        })
    };
    held_rx.recv().unwrap();
    let _ = daemon.life_probe.set(LifeProbe::default());
    let probe = daemon.life_probe.get().unwrap();
    let (life, leave) = spawn_life(&daemon, Duration::ZERO);
    // The driver's judge has started, its thread about to wait on the
    // held lock: the loop is judging when the signal comes.
    tokio::time::timeout(HANG, probe.judging.notified())
        .await
        .expect("the driver's judge started");

    let why = "SIGTERM received; exiting".to_string();
    let won = daemon.exit.arm(&why, daemon.choke.now());
    daemon.exit.hand(Event::Signal {
        won,
        why: why.clone(),
    });
    let stopped = tokio::time::timeout(HANG, async {
        while daemon.rails().halted().is_none()
            || !std::fs::read_to_string(&log_path).unwrap().contains(&why)
        {
            // The poll's interval: a component test on a plain runtime has no
            // park hook to wake it when the state it waits for holds.
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await;
    // The drain's first look parks behind the held lock.
    await_a_look(&clock).await;
    let still_held = daemon.shared.try_lock().is_err();
    release_tx.send(()).unwrap();
    holder.join().unwrap();

    assert!(won, "the signal began the exit");
    assert!(
        stopped.is_ok(),
        "admission stopped, and the words logged, while the lock was held"
    );
    assert!(still_held, "the lock was held throughout");
    let cut = drive_drain(&clock, &daemon, leave)
        .await
        .expect("the driver reached its leave");
    assert_eq!(cut, None, "nothing was in flight");
    life.await.unwrap();
    let log = std::fs::read_to_string(&log_path).unwrap();
    assert!(
        !log.contains("idle timeout; exiting"),
        "the late verdict began nothing:\n{log}"
    );
    finish_harness(None, &log_path);
}

// ---- C134: a panic ends the process, and its job is sent once ----------

/// A harness daemon serving a world (C134: only a daemon with a world
/// reads or writes a fault marker), its queue on `<dir>/daemon.db` in the
/// provider directory beside the marker, restored from it as `run` does:
/// the component test's below, which reads the rows through a connection
/// of its own between lifetimes and refuses a write with a trigger.
fn world_daemon(dir: &std::path::Path) -> (Arc<Daemon>, TestPath) {
    let (mut daemon, log_path) = test_daemon("http://127.0.0.1:1", Arc::new(ParkingClock::new()));
    {
        let d = Arc::get_mut(&mut daemon).unwrap();
        d.world = Some(dir.display().to_string());
        d.store_dir = Some(dir.to_path_buf());
        d.jobs_db = Mutex::new(JobDb::open(&acquisition_store::jobs::daemon_db_path(dir)).unwrap());
    }
    daemon.restore_jobs(Retention::default()).unwrap();
    (daemon, log_path)
}

/// A door serving the world at `dir` ([`DoorBuilder::world`]; C134: a
/// daemon with a world reads the fault marker beside `daemon.db`), its
/// queue that directory's `daemon.db`, restored as a start restores it
/// before the dispatcher starts; facing the account server holding
/// nothing, which would answer a probe and a GET alike, so any send the
/// lifetime made is on its record ([`Door::sent`]) and in the journal.
fn fault_door(dir: &std::path::Path) -> DoorBuilder {
    Door::open(
        Server::Account(("GET", "/never", 0)),
        Arc::new(ParkingClock::new()),
        Arc::new(RecordingCredentialStore::default()),
    )
    .world(dir.display().to_string(), dir.to_path_buf())
    .jobs_db(JobDb::open(&acquisition_store::jobs::daemon_db_path(dir)).unwrap())
    .restore_jobs(Retention::default())
}

/// The marker a panic in job 1 leaves (`install_panic_hook` writes it):
/// a staged input, as the previous lifetime's rows are.
fn stage_fault_marker(dir: &std::path::Path) {
    std::fs::write(dir.join("fault.json"), "{\"job\":1}\n").unwrap();
}

/// The jobs `List` answers, each by its id and state.
async fn listed_states(door: &mut Door) -> Vec<(JobId, JobState)> {
    match door.request(Request::List).await {
        Response::Jobs { jobs } => jobs.iter().map(|job| (job.id, job.state)).collect(),
        other => panic!("the list answered {other:?}"),
    }
}

/// Job `id`'s failure in its words, as `Result` answers it.
async fn failure_of(door: &mut Door, id: JobId) -> String {
    match door.request(Request::Result { id }).await {
        Response::Result {
            outcome: Outcome::Failure { error, .. },
            ..
        } => error,
        other => panic!("job {id}'s result reads {other:?}"),
    }
}

fn sleep_row(id: JobId, state: &str) -> acquisition_store::jobs::JobRow {
    acquisition_store::jobs::JobRow {
        id,
        kind: "sleep".into(),
        state: state.into(),
        priority: 0,
        submitted_by: "t".into(),
        parent: None,
        retries: 0,
        account: None,
        params: json!({ "seconds": 0.01 }),
        outcome: None,
        deferred: None,
        cancel_requested: false,
        submitted_at: 0,
        updated_at: 0,
    }
}

/// C134: a panic's marker names the job it ran in, and the next lifetime
/// fails that job, in words that say a fault in the daemon interrupted
/// it, instead of replaying it (C6). Another running job is replayed as
/// C6 says and a waiting one is untouched. A second lifetime on the same
/// directory changes nothing: the failed job stays failed, in the words
/// first written. Through the door ([`fault_door`]): the marker and the
/// previous lifetime's rows are staged inputs, each lifetime its own
/// door's run, its restore held unlanded so the replayed sleeps do not
/// run; read through `List` and `Result`, and by the observer once each
/// run is over (C45). That the marker is gone once read is
/// `a_fault_marker_outlives_a_failure_the_queue_could_not_write`'s.
#[test]
fn a_fault_marker_fails_its_job_once() {
    let dir = TestPath::dir();
    {
        let db = JobDb::open(&acquisition_store::jobs::daemon_db_path(&dir)).unwrap();
        db.upsert(&sleep_row(1, "running")).unwrap();
        db.upsert(&sleep_row(2, "running")).unwrap();
        db.upsert(&sleep_row(3, "waiting")).unwrap();
    }
    stage_fault_marker(&dir);

    let mut first = String::new();
    let written = &mut first;
    fault_door(&dir)
        .restore_landed(false)
        .run(|mut door| async move {
            assert_eq!(
                listed_states(&mut door).await,
                [
                    (1, JobState::Failed),
                    (2, JobState::Waiting),
                    (3, JobState::Waiting)
                ],
                "the marker's job fails; the rest restore as C6 says"
            );
            let error = failure_of(&mut door, 1).await;
            assert!(
                error.contains("interrupted by a fault in the daemon"),
                "{error}"
            );
            assert!(error.contains("not replayed"), "{error}");
            *written = error;
            door.finish().await;
        });
    let state = |id| on_disk(&dir, id).expect("the row").job.state;
    assert_eq!(state(1), "failed", "the failure is on disk");

    let first = &first;
    fault_door(&dir)
        .restore_landed(false)
        .run(|mut door| async move {
            assert_eq!(
                listed_states(&mut door).await,
                [(2, JobState::Waiting), (3, JobState::Waiting)],
                "a second restore changes nothing"
            );
            assert_eq!(
                &failure_of(&mut door, 1).await,
                first,
                "the failure stands as written"
            );
            door.finish().await;
        });
    assert_eq!(state(1), "failed", "the failure stays on disk");
}

/// The queue a probe's panic leaves in `dir`: the probe on `fetch`,
/// recorded running, the `fetch` job waiting behind it, and the marker
/// naming the probe. The probe's URL is its row's alone: a restore drops
/// probes, never sending one.
fn seed_faulted_probe(dir: &std::path::Path) {
    let db = JobDb::open(&acquisition_store::jobs::daemon_db_path(dir)).unwrap();
    db.upsert(&acquisition_store::jobs::JobRow {
        kind: "probe".into(),
        params: json!({ "route": "fetch", "url": "http://127.0.0.1:1/fetch" }),
        ..sleep_row(1, "running")
    })
    .unwrap();
    db.upsert(&acquisition_store::jobs::JobRow {
        kind: "fetch".into(),
        params: json!({}),
        ..sleep_row(2, "waiting")
    })
    .unwrap();
    stage_fault_marker(dir);
}

/// Job 2 of [`seed_faulted_probe`]'s queue, awaited to its end: failed,
/// its words naming the closed route and why it is closed.
async fn assert_failed_on_the_closed_route(door: &mut Door) {
    let (info, outcome) = door.await_terminal(2).await;
    assert_eq!(info.state, JobState::Failed, "{outcome:?}");
    let Outcome::Failure { error, .. } = &outcome else {
        panic!("the waiting job failed: {outcome:?}")
    };
    assert!(error.contains("route fetch"), "{error}");
    assert!(
        error.contains("a probe on this route broke the daemon"),
        "{error}"
    );
}

/// C134: a marker naming a probe closes that probe's route for its
/// cooldown, so the job waiting on the route fails instead of recreating
/// the probe, and nothing is sent in the lifetime after the fault: no
/// HEAD, no GET. The job's failure names the closed route; a job
/// submitted on the route in the cooldown fails as the route's degraded.
/// Through the door ([`fault_door`], the dispatcher running): the
/// probe's row, the job's and the marker are staged inputs; the account
/// server would answer the probe and the GET, so a restore that let the
/// job recreate its probe would put both on its record and the journal.
#[test]
fn a_faulted_probe_closes_its_route() {
    let dir = TestPath::dir();
    seed_faulted_probe(&dir);
    fault_door(&dir).run(|mut door| async move {
        assert_failed_on_the_closed_route(&mut door).await;
        let submitted = door.submit("fetch", None).await;
        let (info, outcome) = door.await_terminal(submitted).await;
        assert_eq!(info.state, JobState::Failed, "{outcome:?}");
        assert!(
            matches!(&outcome, Outcome::Failure { error, .. }
                if error.contains("route fetch is degraded")
                    && error.contains("a probe on this route broke the daemon")),
            "a job submitted in the cooldown fails on the closed route: {outcome:?}"
        );
        assert!(
            door.journal().is_empty(),
            "the lifetime after the fault sent: {:?}",
            door.journal()
        );
        assert!(door.sent().is_empty(), "the server was reached");
        door.finish().await;
    });
}

/// C134: the jobs a faulted probe's route holds fail at the restore, not
/// only while the closure lasts. A daemon that starts halted (a trip
/// persisted by the last lifetime) dispatches nothing; once the cooldown
/// has passed and the halt is reset, the route is unknown again, and a
/// job still waiting would recreate the probe and send. The job is
/// already failed, in words naming the closed route, and nothing is
/// sent. Through the door ([`fault_door`]): the trip is a staged input
/// too (`rails.json` beside the queue), read through `DaemonStatus`; the
/// cooldown passes by the scenario's hand, since nothing parks on it; the
/// reset is `ResetTripwire`'s; the row read on disk by the observer once
/// the run is over (C45).
#[test]
fn a_faulted_probe_fails_its_route_jobs_even_when_halted() {
    let dir = TestPath::dir();
    seed_faulted_probe(&dir);
    let rails_state = dir.join("rails.json");
    std::fs::write(&rails_state, r#"{"tripped":"429 on GET /fetch"}"#).unwrap();
    fault_door(&dir)
        .rails(RailsConfig {
            state_path: Some(rails_state),
            ..tripwire_config()
        })
        .run(|mut door| async move {
            let halted = match door.request(Request::DaemonStatus).await {
                Response::DaemonStatus { rails, .. } => rails.halted,
                other => panic!("the daemon's status reads {other:?}"),
            };
            assert!(halted.is_some(), "the daemon starts halted");
            assert_failed_on_the_closed_route(&mut door).await;
            door.settle().await;
            door.advance(crate::ratelimit::PROBE_COOLDOWN + Duration::from_secs(1));
            let reset = door.request(Request::ResetTripwire).await;
            assert!(matches!(reset, Response::Ack { .. }), "{reset:?}");
            door.settle().await;
            assert!(
                door.journal().is_empty(),
                "the lifetime after the fault sent: {:?}",
                door.journal()
            );
            assert!(door.sent().is_empty(), "the server was reached");
            door.finish().await;
        });
    assert_eq!(
        on_disk(&dir, 2).expect("the row").job.state,
        "failed",
        "the failure is on disk"
    );
}

/// C134: a fault whose failure the queue could not write keeps its
/// marker, for the row still says the job was open: the next restore
/// fails the job instead of replaying it, and only then is the marker
/// removed. The write is refused by a trigger staged on the queue file.
/// The marker's removal after a restore that wrote its failure is this
/// test's to pin, for the scenarios above: the marker is the hook's file
/// beside `daemon.db`, which no frontend reads. A component test (C158):
/// two lifetimes read through a connection of the test's own to the
/// queue file, between them the trigger dropped.
#[tokio::test]
async fn a_fault_marker_outlives_a_failure_the_queue_could_not_write() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let db = acquisition_store::jobs::daemon_db_path(&dir);
    JobDb::open(&db)
        .unwrap()
        .upsert(&sleep_row(1, "running"))
        .unwrap();
    let refuse = |sql: &str| {
        rusqlite::Connection::open(&db)
            .unwrap()
            .execute_batch(sql)
            .unwrap();
    };
    refuse(
        "CREATE TRIGGER staged_refusal BEFORE INSERT ON jobs WHEN NEW.state = 'failed'
         BEGIN SELECT RAISE(ABORT, 'staged refusal'); END;",
    );
    let marker = dir.join("fault.json");
    std::fs::write(&marker, "{\"job\":1}\n").unwrap();

    let (daemon, log1) = world_daemon(&dir);
    assert_eq!(row_of(&db, 1).state, "running", "the failure was refused");
    assert!(
        marker.exists(),
        "the marker was removed with the failure unwritten"
    );
    drop(daemon);

    refuse("DROP TRIGGER staged_refusal;");
    let (daemon, log2) = world_daemon(&dir);
    assert_eq!(row_of(&db, 1).state, "failed", "the job was replayed");
    assert!(!marker.exists(), "the marker outlived the written failure");
    drop(daemon);
    for log in [log1, log2] {
        finish_harness(None, &log);
    }
}

/// The child's exit status: the hook reached its end. `run` passes
/// `std::process::abort`; the test passes this, so a passing run leaves
/// no crash report behind.
const HOOK_ENDED: i32 = 86;

/// Set in the child to the directory it writes in, and to where it
/// panics: `job` inside a job's task, `free` outside any.
const HOOK_CHILD_DIR: &str = "ACQ_TEST_PANIC_HOOK_DIR";
const HOOK_CHILD_WHERE: &str = "ACQ_TEST_PANIC_HOOK_WHERE";

fn hook_ended() -> ! {
    std::process::exit(HOOK_ENDED)
}

/// The child: the hook installed as `run` installs it, then a panic on a
/// tokio worker, inside job 7's task or outside any. (The hook holds no
/// `Daemon`, so it can take no daemon lock; that is by construction, not
/// by this test.)
fn panic_hook_child(dir: &std::path::Path) -> ! {
    let log = std::fs::File::create(dir.join("daemon.log")).unwrap();
    super::super::start::install_panic_hook(
        log,
        Some(dir.join("fault.json")),
        std::time::Instant::now(),
        hook_ended,
    );
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    let in_job = std::env::var(HOOK_CHILD_WHERE).as_deref() == Ok("job");
    runtime.block_on(async move {
        let task = async move {
            panic!("a broken invariant, for the hook");
        };
        let _ = if in_job {
            tokio::spawn(RUNNING_JOB.scope(7, task)).await
        } else {
            tokio::spawn(task).await
        };
    });
    // A task's panic that tokio caught: the hook did not end the process.
    std::process::exit(1)
}

/// Run this test's own binary as the child, panicking `at`.
fn run_panic_hook_child(at: &str) -> (TestPath, std::process::ExitStatus) {
    let dir = TestPath::dir();
    let module = module_path!()
        .split_once("::")
        .map_or(module_path!(), |(_, rest)| rest);
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            format!("{module}::a_panic_is_logged_marks_its_job_and_ends_the_process"),
            "--exact".into(),
            "--nocapture".into(),
            "--test-threads=1".into(),
        ])
        .env(HOOK_CHILD_DIR, &*dir)
        .env(HOOK_CHILD_WHERE, at)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .unwrap();
    (dir, status)
}

/// C134: a panic in a job's task writes one log line naming the job, the
/// place and the words, writes the marker naming the job (and nothing
/// else), and ends the process: tokio's catch of a task's panic never
/// runs. A panic outside any job is logged the same and leaves no marker.
#[test]
fn a_panic_is_logged_marks_its_job_and_ends_the_process() {
    if let Some(dir) = std::env::var_os(HOOK_CHILD_DIR) {
        panic_hook_child(std::path::Path::new(&dir));
    }

    let (dir, status) = run_panic_hook_child("job");
    assert_eq!(status.code(), Some(HOOK_ENDED), "{status}");
    let log = std::fs::read_to_string(dir.join("daemon.log")).unwrap();
    assert_eq!(log.lines().count(), 1, "{log}");
    assert!(log.contains("PANIC in job 7 at "), "{log}");
    assert!(log.contains("exit.rs:"), "the place: {log}");
    assert!(log.contains("a broken invariant, for the hook"), "{log}");
    assert!(log.contains("the fault marker"), "{log}");
    assert_eq!(
        std::fs::read_to_string(dir.join("fault.json")).unwrap(),
        "{\"job\":7}\n",
        "the marker names the job and carries nothing else"
    );

    let (dir, status) = run_panic_hook_child("free");
    assert_eq!(status.code(), Some(HOOK_ENDED), "{status}");
    let log = std::fs::read_to_string(dir.join("daemon.log")).unwrap();
    assert!(log.contains("PANIC outside any job at "), "{log}");
    assert!(!dir.join("fault.json").exists(), "no job, no marker");
}

/// C134's seam: errors put in front of a real listener, or of none (a
/// listener that never connects anyone).
#[cfg(unix)]
struct Faulty {
    errors: std::collections::VecDeque<std::io::Error>,
    inner: Option<crate::daemon::endpoint::Listener>,
}

#[cfg(unix)]
impl crate::daemon::endpoint::Accept for Faulty {
    fn accept(
        &mut self,
    ) -> impl Future<Output = std::io::Result<crate::daemon::endpoint::Stream>> + Send {
        let error = self.errors.pop_front();
        let inner = &mut self.inner;
        async move {
            match (error, inner) {
                (Some(e), _) => Err(e),
                (None, Some(listener)) => listener.accept().await,
                (None, None) => std::future::pending().await,
            }
        }
    }
}

#[cfg(unix)]
fn accept_failures(log_path: &std::path::Path) -> Vec<String> {
    std::fs::read_to_string(log_path)
        .unwrap_or_default()
        .lines()
        .filter(|l| l.contains("accept failed"))
        .map(str::to_string)
        .collect()
}

/// C134: an error from `accept` is logged once and served through: the
/// next connection is answered, and nothing exits. The pause's cap is
/// zero: the backoff sleeps on the daemon's clock (C160), which nothing
/// moves in a component test, and a zero pause is ready at once.
#[cfg(unix)]
#[tokio::test]
async fn a_listener_error_is_served_through() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
    let (daemon, log_path) = test_daemon("http://127.0.0.1:1", Arc::new(ParkingClock::new()));
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("listener.sock");
    let (listener, bound) = crate::daemon::endpoint::Listener::bind(&path)
        .await
        .unwrap();
    let faulty = Faulty {
        errors: [std::io::Error::from(std::io::ErrorKind::ConnectionAborted)].into(),
        inner: Some(listener),
    };
    let serving = tokio::spawn(daemon.clone().serve(faulty, Duration::ZERO));

    let answer = tokio::time::timeout(HANG, async {
        let stream = tokio::net::UnixStream::connect(&path).await?;
        let (read, mut write) = tokio::io::split(stream);
        write.write_all(b"{\"req\":\"hello\"}\n").await?;
        let mut line = String::new();
        tokio::io::BufReader::new(read).read_line(&mut line).await?;
        std::io::Result::Ok(line)
    })
    .await
    .expect("the next connection is answered in time")
    .expect("the next connection is answered");
    assert!(
        matches!(
            acquisition_protocol::protocol::BootstrapReply::read(answer.as_bytes()),
            Some(acquisition_protocol::protocol::BootstrapReply::Hello { .. })
        ),
        "{answer}"
    );
    assert!(!serving.is_finished(), "the accept loop serves on");
    assert!(!daemon.exit.begun(), "nothing exited");
    let failures = accept_failures(&log_path);
    assert_eq!(failures.len(), 1, "logged once: {failures:?}");

    serving.abort();
    bound.remove_if_still_ours();
    finish_harness(None, &log_path);
}

/// C134: a run of errors writes a bounded number of log lines, each kind
/// reported the first time, and the loop serves on through it.
#[cfg(unix)]
#[tokio::test]
async fn a_run_of_listener_errors_logs_a_bounded_number_of_lines() {
    let (daemon, log_path) = test_daemon("http://127.0.0.1:1", Arc::new(ParkingClock::new()));
    let mut errors: std::collections::VecDeque<std::io::Error> = (0..1000)
        .map(|_| std::io::Error::other("too many open files"))
        .collect();
    errors.push_back(std::io::Error::from(std::io::ErrorKind::ConnectionAborted));
    let faulty = Faulty {
        errors,
        inner: None,
    };
    let serving = tokio::spawn(daemon.clone().serve(faulty, Duration::ZERO));
    let aborted = std::io::Error::from(std::io::ErrorKind::ConnectionAborted).to_string();
    tokio::time::timeout(HANG, async {
        while !accept_failures(&log_path)
            .iter()
            .any(|l| l.contains(&aborted))
        {
            // The poll's interval: a component test on a plain runtime has no
            // park hook to wake it when the state it waits for holds.
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("the run of errors is served through to its end");
    assert!(!serving.is_finished(), "the accept loop serves on");
    serving.abort();

    let failures = accept_failures(&log_path);
    assert!(
        (3..=12).contains(&failures.len()),
        "a thousand errors in a row, a handful of lines: {failures:?}"
    );
    assert_eq!(
        failures.iter().filter(|l| l.contains(&aborted)).count(),
        1,
        "a new kind is reported the first time: {failures:?}"
    );
    assert!(
        failures.iter().any(|l| l.contains("512 times")),
        "a later line says how many: {failures:?}"
    );
    assert!(!daemon.exit.begun(), "nothing exited");
    finish_harness(None, &log_path);
}
