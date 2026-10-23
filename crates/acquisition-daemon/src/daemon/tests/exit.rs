//! C133, the one exit: what the drain waits for, what it turns away, what
//! the bound cuts, and the refusal and the idle verdict around it. Each
//! test runs a harness daemon with a short bound on tokio time and never
//! calls `leave`, which ends the process. C134, the panic's end: the hook
//! that writes the log line and the fault marker, and the restore that
//! fails the marker's job once, or closes its probe's route for the
//! cooldown. C134, the listener: an accept error is
//! served through, logged with a backoff.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::server::InFlight;
use crate::daemon::*;

/// Long enough for the drains that must finish, short beside the tests'
/// own timeouts.
const BOUND: Duration = Duration::from_secs(3);

fn test_db_path() -> PathBuf {
    std::env::temp_dir().join(format!(
        "acquisition-exit-jobs-{}-{}.db",
        std::process::id(),
        TEST_LOG_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ))
}

fn remove_db(path: &std::path::Path) {
    for suffix in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{suffix}", path.display()));
    }
}

/// A logged-in harness daemon on the manual clock, its queue on a
/// file-backed `daemon.db`, its exit bounded by `bound`.
fn exit_daemon(base: &str, bound: Duration) -> (Arc<Daemon>, PathBuf, PathBuf) {
    exit_daemon_on(base, bound, Arc::new(ManualClock::new()))
}

/// [`exit_daemon`] on `clock`.
fn exit_daemon_on(
    base: &str,
    bound: Duration,
    clock: Arc<dyn crate::ratelimit::Clock>,
) -> (Arc<Daemon>, PathBuf, PathBuf) {
    let db = test_db_path();
    remove_db(&db);
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

fn state_of(daemon: &Daemon, id: JobId) -> JobState {
    daemon.shared.lock().unwrap().jobs[&id].info.state
}

async fn wait_state(daemon: &Daemon, id: JobId, state: JobState) {
    tokio::time::timeout(Duration::from_secs(3), async {
        while state_of(daemon, id) != state {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap_or_else(|_| panic!("job {id} reached {state}"));
}

/// The drain, on a task of its own, so a test can see it still waiting.
fn spawn_drain(daemon: &Arc<Daemon>) -> tokio::task::JoinHandle<Option<InFlight>> {
    let daemon = daemon.clone();
    tokio::spawn(async move { daemon.drain().await })
}

/// Long enough for a drain that does not wait to have returned.
async fn settle() {
    tokio::time::sleep(Duration::from_millis(100)).await;
}

/// C133: a job whose GET is on the wire when the exit begins is waited
/// for. The server answers only after the drain has begun; the drain
/// returns with nothing cut, the journal equals the wire, and the job's
/// row is terminal on disk.
#[tokio::test]
async fn exit_waits_for_a_landed_body_to_persist() {
    let (base, sent_log, arrived, release) = account_server(("GET", "/profile", 0)).await;
    let (daemon, log_path, db) = exit_daemon(&base, BOUND);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = submit_for(&daemon, "profile", json!({}), Some("test-user"));
    arrived.await.unwrap();

    assert!(daemon.begin_exit("test: exit"));
    let drain = spawn_drain(&daemon);
    settle().await;
    assert!(
        !drain.is_finished(),
        "the drain waits for the job on the wire"
    );
    release.send(()).unwrap();
    assert_eq!(drain.await.unwrap(), None, "nothing was cut");

    assert_eq!(row_of(&db, id).state, JobState::Done.to_string());
    assert_eq!(
        sent_log.lock().unwrap().as_slice(),
        [sent("test-user", "POST"), sent("test-user", "GET")]
    );
    assert_journal_matches_wire(&log_path, &wire_methods(&sent_log));
    finish_harness(dispatcher, &log_path);
    remove_db(&db);
}

/// C133: a token refresh on the wire when the exit begins is waited for.
/// The token server answers only after the drain has begun; the rotated
/// refresh token is saved, the journal holds the one POST, and the
/// caller waiting on the refresh gets its token.
#[tokio::test]
async fn exit_waits_for_a_refresh_and_installs_the_token() {
    let (base, arrived, release, server) = delayed_token_server().await;
    let (mut daemon, log_path, db) = exit_daemon(&base, BOUND);
    let store = Arc::new(RecordingCredentialStore::default());
    Arc::get_mut(&mut daemon).unwrap().credential_store = store.clone();
    let waiter = {
        let daemon = daemon.clone();
        tokio::spawn(async move {
            daemon
                .valid_access_token(Some("test-user"), true, None)
                .await
        })
    };
    arrived.await.unwrap();

    assert!(daemon.begin_exit("test: exit"));
    let drain = spawn_drain(&daemon);
    settle().await;
    assert!(!drain.is_finished(), "the drain waits for the refresh");
    release.send(()).unwrap();
    assert_eq!(drain.await.unwrap(), None, "nothing was cut");

    let (token, _) = waiter.await.unwrap().expect("the waiter got its token");
    assert_eq!(token, "at-new");
    let saves = store.saves.lock().unwrap().clone();
    assert!(
        saves.iter().any(|(_, token, _)| token == "rt-rotated"),
        "the rotated refresh token was saved: {saves:?}"
    );
    server.await.unwrap();
    assert_journal_matches_wire(&log_path, &["POST".to_string()]);
    finish_harness(None, &log_path);
    remove_db(&db);
}

/// Two accounts, with `other`'s refresh held on the wire and
/// `test-user`'s profile job parked Running behind it at the token
/// route's gate. Returns the parked job, `other`'s job whose POST is held,
/// and what was sent before the park.
struct Parked {
    daemon: Arc<Daemon>,
    log_path: PathBuf,
    db: PathBuf,
    dispatcher: tokio::task::JoinHandle<()>,
    sent_log: Arc<Mutex<Vec<Sent>>>,
    release: tokio::sync::oneshot::Sender<()>,
    parked: JobId,
    first: JobId,
    warmed: Vec<Sent>,
}

async fn park_a_job_at_the_token_route() -> Parked {
    let (base, sent_log, arrived, release) = account_server(("POST", "/token", 1)).await;
    let (daemon, log_path, db) = exit_daemon(&base, BOUND);
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
    // Long enough for the parked refresh to reach the gate's queue.
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
/// for is turned away before its GET. Nothing more is sent.
#[tokio::test]
async fn exit_turns_away_a_job_parked_past_running() {
    let p = park_a_job_at_the_token_route().await;
    assert!(p.daemon.begin_exit("test: exit"));
    let drain = spawn_drain(&p.daemon);
    p.release.send(()).unwrap();
    assert_eq!(drain.await.unwrap(), None, "nothing was cut");

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
    remove_db(&p.db);
}

/// C133: the dispatcher starts nothing while the daemon exits. A job the
/// exit turned away is not picked again (the drain would spin, a queue
/// write per pass), and a job queued afterwards, even one of a kind that
/// never sends, is not started.
#[tokio::test]
async fn a_turned_away_job_is_not_picked_again_while_exiting() {
    let p = park_a_job_at_the_token_route().await;
    assert!(p.daemon.begin_exit("test: exit"));
    let drain = spawn_drain(&p.daemon);
    p.release.send(()).unwrap();
    assert_eq!(drain.await.unwrap(), None);
    assert_eq!(state_of(&p.daemon, p.parked), JobState::Waiting);

    let local = submit_for(&p.daemon, "sleep", json!({ "seconds": 0 }), None);
    settle().await;
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
    remove_db(&p.db);
}

/// C133: a job whose task does not end is cut at the bound, on tokio
/// time: the drain reports it, and its row is left running on disk for
/// the next daemon (C6).
#[tokio::test]
async fn exit_cuts_a_stuck_job_at_the_bound() {
    let bound = Duration::from_millis(300);
    let (daemon, log_path, db) = exit_daemon("http://127.0.0.1:1", bound);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = submit_for(&daemon, "sleep", json!({ "seconds": 60 }), None);
    wait_state(&daemon, id, JobState::Running).await;

    let began = std::time::Instant::now();
    assert!(daemon.begin_exit("test: exit"));
    let cut = tokio::time::timeout(Duration::from_secs(5), daemon.drain())
        .await
        .expect("the bound ends the drain");
    let waited = began.elapsed();
    assert_eq!(
        cut,
        Some(InFlight {
            jobs: vec![id],
            ..InFlight::default()
        })
    );
    assert!(
        waited >= bound && waited < Duration::from_secs(3),
        "the drain ran to its bound: {waited:?}"
    );
    assert_eq!(row_of(&db, id).state, JobState::Running.to_string());
    let log = std::fs::read_to_string(&log_path).unwrap();
    assert!(
        log.contains(&format!("cut job {id} (left running on disk")),
        "the log names the cut job:\n{log}"
    );
    finish_harness(dispatcher, &log_path);
    remove_db(&db);
}

/// C133: once the exit has begun a client's submit is refused, as
/// `refused`, in words that say to try again; the same submit was
/// accepted before.
#[tokio::test]
async fn a_submit_after_the_exit_begins_is_refused_as_stopping() {
    let (daemon, log_path, db) = exit_daemon("http://127.0.0.1:1", BOUND);
    let submit = || Request::Submit {
        kind: "sleep".into(),
        params: json!({ "seconds": 0 }),
        priority: 0,
        submitted_by: "test".into(),
        account: None,
    };
    assert!(matches!(
        daemon.handle_request(submit(), &mut None).await,
        Response::Submitted { .. }
    ));
    assert!(daemon.begin_exit("test: exit"));
    match daemon.handle_request(submit(), &mut None).await {
        Response::Error { kind, message } => {
            assert_eq!(kind, ErrorKind::Refused);
            assert_eq!(message, "the daemon is stopping; try again in a moment");
        }
        other => panic!("expected the stopping refusal, got {other:?}"),
    }
    finish_harness(None, &log_path);
    remove_db(&db);
}

/// C133: a login in progress holds the idle verdict. Its browser step and
/// code exchange belong to no connection and no job, and an idle exit
/// in the middle would lose the login.
#[test]
fn the_idle_verdict_holds_while_a_login_is_pending() {
    let clock = Arc::new(ManualClock::new());
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
/// the first's bound where it was.
#[tokio::test]
async fn a_second_begin_exit_returns_at_once() {
    let (daemon, log_path, db) = exit_daemon("http://127.0.0.1:1", BOUND);
    assert!(daemon.begin_exit("test: the first cause"));
    let deadline = *daemon.exit.deadline.get().expect("the bound is armed");
    tokio::time::sleep(Duration::from_millis(20)).await;
    assert!(!daemon.begin_exit("test: the second cause"));
    assert_eq!(daemon.exit.deadline.get(), Some(&deadline));
    let log = std::fs::read_to_string(&log_path).unwrap();
    assert!(
        !log.contains("the second cause"),
        "the second cause began nothing:\n{log}"
    );
    finish_harness(None, &log_path);
    remove_db(&db);
}

/// C133: the bound is hard. A keyring prompt holds `shared`, and tasks
/// that take it (here the idle verdict, twice) block both workers of the
/// runtime, so no tokio timer runs and the drain cannot reach its bound.
/// The bound's own thread still ends the exit at the bound and its grace,
/// with the lock still held.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_bound_ends_the_exit_though_every_worker_is_blocked() {
    let bound = Duration::from_millis(300);
    let grace = Duration::from_millis(200);
    let (mut daemon, log_path, db) = exit_daemon("http://127.0.0.1:1", bound);
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
    let holder = {
        let daemon = daemon.clone();
        std::thread::spawn(move || {
            let _shared = daemon.shared.lock().unwrap();
            held_tx.send(()).unwrap();
            std::thread::sleep(Duration::from_millis(2500));
        })
    };
    held_rx.recv().unwrap();
    let blocked: Vec<_> = (0..2)
        .map(|_| {
            let daemon = daemon.clone();
            tokio::spawn(async move { daemon.is_idle(Duration::from_secs(60)) })
        })
        .collect();
    let drain = spawn_drain(&daemon);
    std::thread::sleep(Duration::from_millis(100));

    let began = std::time::Instant::now();
    assert!(daemon.begin_exit("test: exit"));
    let (why, code) = ended_rx
        .recv_timeout(Duration::from_secs(2))
        .expect("the bound ended the exit with every worker blocked");
    let waited = began.elapsed();
    assert!(
        waited >= bound + grace && waited < Duration::from_millis(1500),
        "the end came at the bound and its grace: {waited:?}"
    );
    assert!(
        daemon.shared.try_lock().is_err(),
        "the prompt still held the lock"
    );
    assert_eq!(code, 1);
    assert!(why.contains("had not left"), "{why}");
    assert!(why.contains("with no checkpoint"), "{why}");

    holder.join().unwrap();
    for task in blocked {
        task.await.unwrap();
    }
    let _ = drain.await;
    finish_harness(None, &log_path);
    remove_db(&db);
}

/// C133 with C25: a job picked before the exit and held at the limiter
/// before its send (here behind a 429's hold, on the system clock, where
/// the dispatcher's pacing loop really waits) is turned away: the drain
/// does not wait out the hold, cuts nothing, and the job is waiting in
/// memory and on disk with only the 429's retry spent.
#[tokio::test]
async fn exit_turns_away_a_job_held_at_the_limiter() {
    let responses = vec![
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse::full("GET", "429 Too Many Requests", Some(30), "{}"),
    ];
    let (base, requests, server) = scripted_server(responses).await;
    let (daemon, log_path, db) =
        exit_daemon_on(&base, BOUND, Arc::new(crate::ratelimit::SystemClock));
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = submit_for(&daemon, "fetch", json!({}), None);
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            {
                let s = daemon.shared.lock().unwrap();
                let info = &s.jobs[&id].info;
                if info.retries == 1
                    && info.state == JobState::Waiting
                    && s.active_jobs.contains_key(&id)
                {
                    break;
                }
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("the job is held at the limiter after its 429");
    server.await.unwrap();

    let began = std::time::Instant::now();
    assert!(daemon.begin_exit("test: exit"));
    let cut = tokio::time::timeout(BOUND + Duration::from_secs(1), daemon.drain())
        .await
        .expect("the drain ends");
    let waited = began.elapsed();
    assert_eq!(cut, None, "nothing was cut");
    assert!(
        waited < Duration::from_secs(2),
        "the drain did not wait out the hold: {waited:?}"
    );
    let info = daemon.shared.lock().unwrap().jobs[&id].info.clone();
    assert_eq!(info.state, JobState::Waiting);
    assert_eq!(info.retries, 1, "the exit spent no retry");
    let row = row_of(&db, id);
    assert_eq!(row.state, JobState::Waiting.to_string());
    assert_eq!(row.retries, 1);
    assert_eq!(requests.lock().unwrap().as_slice(), ["HEAD", "GET"]);
    finish_harness(dispatcher, &log_path);
    remove_db(&db);
}

/// C133: a login's code exchange on the wire when the exit begins is
/// waited for: the token server answers only after the drain has begun,
/// and the drain returns, with nothing cut, once the exchange has landed.
#[tokio::test]
async fn exit_waits_for_a_login_code_exchange() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let (arrived_tx, arrived) = tokio::sync::oneshot::channel();
    let (release, release_rx) = tokio::sync::oneshot::channel::<()>();
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
        let headers = concat!(
            "X-Rate-Limit-Policy: token-request-limit\r\n",
            "X-Rate-Limit-Rules: Ip\r\n",
            "X-Rate-Limit-Ip: 60:30:30\r\n",
            "X-Rate-Limit-Ip-State: 1:30:0\r\n",
        );
        mockggg::respond_with(&mut stream, "200 OK", "application/json", headers, &body).await;
    });
    let (daemon, log_path, db) = exit_daemon(&base, BOUND);

    let authorize = url::Url::parse(&daemon.auth_start().await.unwrap()).unwrap();
    let query: HashMap<String, String> = authorize.query_pairs().into_owned().collect();
    let callback = format!(
        "{}?code=code-x&state={}",
        query["redirect_uri"], query["state"]
    );
    assert!(reqwest::get(&callback).await.unwrap().status().is_success());
    arrived.await.unwrap();

    assert!(daemon.begin_exit("test: exit"));
    let drain = spawn_drain(&daemon);
    settle().await;
    assert!(
        !drain.is_finished(),
        "the drain waits for the code exchange"
    );
    release.send(()).unwrap();
    assert_eq!(drain.await.unwrap(), None, "nothing was cut");
    assert_eq!(daemon.shared.lock().unwrap().auth.exchanges, 0);
    server.await.unwrap();
    assert_journal_matches_wire(&log_path, &["POST".to_string()]);
    finish_harness(None, &log_path);
    remove_db(&db);
}

// ---- C134: a panic ends the process, and its job is sent once ----------

/// A harness daemon serving a world (C134: only a daemon with a world
/// reads or writes a fault marker), its queue on `<dir>/daemon.db` in the
/// provider directory beside the marker, restored from it as `run` does.
fn world_daemon(dir: &std::path::Path) -> (Arc<Daemon>, PathBuf) {
    world_daemon_at(dir, "http://127.0.0.1:1")
}

/// [`world_daemon`] facing the server at `base`.
fn world_daemon_at(dir: &std::path::Path, base: &str) -> (Arc<Daemon>, PathBuf) {
    world_daemon_on(
        dir,
        base,
        Arc::new(ManualClock::new()),
        RailsConfig::default(),
    )
}

/// [`world_daemon_at`] on `clock`, under the rails `config`.
fn world_daemon_on(
    dir: &std::path::Path,
    base: &str,
    clock: Arc<ManualClock>,
    config: RailsConfig,
) -> (Arc<Daemon>, PathBuf) {
    let (mut daemon, log_path) = test_daemon_with(Provider::mock(base), clock, config);
    {
        let d = Arc::get_mut(&mut daemon).unwrap();
        d.world = Some(dir.display().to_string());
        d.store_dir = Some(dir.to_path_buf());
        d.jobs_db = Mutex::new(JobDb::open(&acquisition_store::jobs::daemon_db_path(dir)).unwrap());
    }
    daemon.restore_jobs(Retention::default()).unwrap();
    (daemon, log_path)
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
/// it, instead of replaying it (C6); the marker is gone after. Another
/// running job is replayed as C6 says and a waiting one is untouched. A
/// second restore changes nothing: the failed job stays failed.
#[tokio::test]
async fn a_fault_marker_fails_its_job_once() {
    let dir = std::env::temp_dir().join(format!(
        "acquisition-fault-{}-{}",
        std::process::id(),
        TEST_LOG_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let db = acquisition_store::jobs::daemon_db_path(&dir);
    {
        let db = JobDb::open(&db).unwrap();
        db.upsert(&sleep_row(1, "running")).unwrap();
        db.upsert(&sleep_row(2, "running")).unwrap();
        db.upsert(&sleep_row(3, "waiting")).unwrap();
    }
    let marker = dir.join("fault.json");
    std::fs::write(&marker, "{\"job\":1}\n").unwrap();

    let (daemon, log1) = world_daemon(&dir);
    let states = |daemon: &Daemon| -> Vec<(JobId, JobState)> {
        daemon
            .shared
            .lock()
            .unwrap()
            .list(daemon)
            .iter()
            .map(|j| (j.id, j.state))
            .collect()
    };
    assert_eq!(
        states(&daemon),
        [
            (1, JobState::Failed),
            (2, JobState::Waiting),
            (3, JobState::Waiting)
        ],
        "the marker's job fails; the rest restore as C6 says"
    );
    let failed = |row: acquisition_store::jobs::JobRow| -> String {
        assert_eq!(row.state, "failed");
        row.outcome.expect("a failed row has its outcome")["error"]
            .as_str()
            .expect("the failure's words")
            .to_string()
    };
    let error = failed(row_of(&db, 1));
    assert!(
        error.contains("interrupted by a fault in the daemon"),
        "{error}"
    );
    assert!(error.contains("not replayed"), "{error}");
    assert!(!marker.exists(), "the marker is removed once read");
    drop(daemon);

    let (daemon, log2) = world_daemon(&dir);
    assert_eq!(
        states(&daemon),
        [(2, JobState::Waiting), (3, JobState::Waiting)],
        "a second restore changes nothing"
    );
    assert_eq!(
        failed(row_of(&db, 1)),
        error,
        "the failure stands as written"
    );
    drop(daemon);
    for log in [log1, log2] {
        finish_harness(None, &log);
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// The queue a probe's panic leaves in `dir`: the probe on `fetch`,
/// recorded running, the `fetch` job waiting behind it, and the marker
/// naming the probe. Returns the marker's path.
fn seed_faulted_probe(dir: &std::path::Path, base: &str) -> PathBuf {
    let db = JobDb::open(&acquisition_store::jobs::daemon_db_path(dir)).unwrap();
    db.upsert(&acquisition_store::jobs::JobRow {
        kind: "probe".into(),
        params: json!({ "route": "fetch", "url": format!("{base}/fetch") }),
        ..sleep_row(1, "running")
    })
    .unwrap();
    db.upsert(&acquisition_store::jobs::JobRow {
        kind: "fetch".into(),
        params: json!({}),
        ..sleep_row(2, "waiting")
    })
    .unwrap();
    let marker = dir.join("fault.json");
    std::fs::write(&marker, "{\"job\":1}\n").unwrap();
    marker
}

/// C134: a marker naming a probe closes that probe's route for its
/// cooldown, so the job waiting on the route fails instead of recreating
/// the probe, and nothing is sent in the lifetime after the fault: no
/// HEAD, no GET. The job's failure names the closed route, and the
/// marker is gone; a job submitted on the route in the cooldown fails as
/// the route's degraded. The server would answer the probe and the GET,
/// so a restore that let the job recreate its probe sends both.
#[tokio::test]
async fn a_faulted_probe_closes_its_route() {
    let dir = std::env::temp_dir().join(format!(
        "acquisition-fault-probe-{}-{}",
        std::process::id(),
        TEST_LOG_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let (base, wire, server) = scripted_server(vec![
        ScriptedResponse::full("HEAD", "200 OK", None, ""),
        ScriptedResponse::full("GET", "200 OK", None, "{}"),
    ])
    .await;
    let marker = seed_faulted_probe(&dir, &base);

    let (daemon, log_path) = world_daemon_at(&dir, &base);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let (info, outcome) = wait_terminal(&daemon, 2).await;
    assert_eq!(info.state, JobState::Failed, "{outcome:?}");
    let error = match &outcome {
        Outcome::Failure { error, .. } => error.clone(),
        other => panic!("the waiting job failed: {other:?}"),
    };
    assert!(error.contains("route fetch"), "{error}");
    assert!(
        error.contains("a probe on this route broke the daemon"),
        "{error}"
    );
    assert!(!marker.exists(), "the marker is removed once read");
    let submitted = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let (info, outcome) = wait_terminal(&daemon, submitted).await;
    assert_eq!(info.state, JobState::Failed, "{outcome:?}");
    assert!(
        matches!(&outcome, Outcome::Failure { error, .. }
            if error.contains("route fetch is degraded")
                && error.contains("a probe on this route broke the daemon")),
        "a job submitted in the cooldown fails on the closed route: {outcome:?}"
    );
    assert!(
        journal_sends(&log_path).is_empty(),
        "the lifetime after the fault sent: {:?}",
        journal_sends(&log_path)
    );
    assert!(wire.lock().unwrap().is_empty(), "the server was reached");
    server.abort();
    finish_harness(dispatcher, &log_path);
    let _ = std::fs::remove_dir_all(&dir);
}

/// C134: the jobs a faulted probe's route holds fail at the restore, not
/// only while the closure lasts. A daemon that starts halted (a trip
/// persisted by the last lifetime) dispatches nothing; once the cooldown
/// has passed and the halt is reset, the route is unknown again, and a
/// job still waiting would recreate the probe and send. The job is
/// already failed, in words naming the closed route, and nothing is sent.
#[tokio::test]
async fn a_faulted_probe_fails_its_route_jobs_even_when_halted() {
    let dir = std::env::temp_dir().join(format!(
        "acquisition-fault-probe-halted-{}-{}",
        std::process::id(),
        TEST_LOG_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let (base, wire, server) = scripted_server(vec![
        ScriptedResponse::full("HEAD", "200 OK", None, ""),
        ScriptedResponse::full("GET", "200 OK", None, "{}"),
    ])
    .await;
    let marker = seed_faulted_probe(&dir, &base);
    let rails_state = dir.join("rails.json");
    std::fs::write(&rails_state, r#"{"tripped":"429 on GET /fetch"}"#).unwrap();
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = world_daemon_on(
        &dir,
        &base,
        clock.clone(),
        RailsConfig {
            state_path: Some(rails_state),
            ..tripwire_config()
        },
    );
    assert!(
        daemon.rails().halted().is_some(),
        "the daemon starts halted"
    );
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    settle().await;
    clock.advance(crate::ratelimit::PROBE_COOLDOWN + Duration::from_secs(1));
    daemon.reset_rails().unwrap();
    let (info, outcome) = wait_terminal(&daemon, 2).await;
    assert_eq!(info.state, JobState::Failed, "{outcome:?}");
    let error = match &outcome {
        Outcome::Failure { error, .. } => error.clone(),
        other => panic!("the waiting job failed: {other:?}"),
    };
    assert!(error.contains("route fetch"), "{error}");
    assert!(
        error.contains("a probe on this route broke the daemon"),
        "{error}"
    );
    assert_eq!(
        row_of(&acquisition_store::jobs::daemon_db_path(&dir), 2).state,
        "failed",
        "the failure is on disk"
    );
    assert!(!marker.exists(), "the marker is removed once read");
    settle().await;
    assert!(
        journal_sends(&log_path).is_empty(),
        "the lifetime after the fault sent: {:?}",
        journal_sends(&log_path)
    );
    assert!(wire.lock().unwrap().is_empty(), "the server was reached");
    server.abort();
    finish_harness(dispatcher, &log_path);
    let _ = std::fs::remove_dir_all(&dir);
}

/// C134: a fault whose failure the queue could not write keeps its
/// marker, for the row still says the job was open: the next restore
/// fails the job instead of replaying it, and only then is the marker
/// removed. The write is refused by a trigger staged on the queue file.
#[tokio::test]
async fn a_fault_marker_outlives_a_failure_the_queue_could_not_write() {
    let dir = std::env::temp_dir().join(format!(
        "acquisition-fault-kept-{}-{}",
        std::process::id(),
        TEST_LOG_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
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
    let _ = std::fs::remove_dir_all(&dir);
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
fn run_panic_hook_child(at: &str) -> (PathBuf, std::process::ExitStatus) {
    let dir = std::env::temp_dir().join(format!(
        "acquisition-panic-hook-{}-{}",
        std::process::id(),
        TEST_LOG_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
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
        .env(HOOK_CHILD_DIR, &dir)
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
    let _ = std::fs::remove_dir_all(&dir);

    let (dir, status) = run_panic_hook_child("free");
    assert_eq!(status.code(), Some(HOOK_ENDED), "{status}");
    let log = std::fs::read_to_string(dir.join("daemon.log")).unwrap();
    assert!(log.contains("PANIC outside any job at "), "{log}");
    assert!(!dir.join("fault.json").exists(), "no job, no marker");
    let _ = std::fs::remove_dir_all(&dir);
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
/// next connection is answered, and nothing exits.
#[cfg(unix)]
#[tokio::test]
async fn a_listener_error_is_served_through() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
    let (daemon, log_path) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
    let path = std::env::temp_dir().join(format!(
        "acq-listener-{}-{}.sock",
        std::process::id(),
        TEST_LOG_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    let _ = std::fs::remove_file(&path);
    let (listener, bound) = crate::daemon::endpoint::Listener::bind(&path)
        .await
        .unwrap();
    let faulty = Faulty {
        errors: [std::io::Error::from(std::io::ErrorKind::ConnectionAborted)].into(),
        inner: Some(listener),
    };
    let serving = tokio::spawn(daemon.clone().serve(faulty, Duration::from_millis(1)));

    let answer = tokio::time::timeout(Duration::from_secs(5), async {
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
    let (daemon, log_path) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
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
    tokio::time::timeout(Duration::from_secs(10), async {
        while !accept_failures(&log_path)
            .iter()
            .any(|l| l.contains(&aborted))
        {
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
