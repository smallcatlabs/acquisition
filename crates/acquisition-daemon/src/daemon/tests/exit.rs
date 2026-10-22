//! C133, the one exit: what the drain waits for, what it turns away, what
//! the bound cuts, and the refusal and the idle verdict around it. Each
//! test runs a harness daemon with a short bound on tokio time and never
//! calls `leave`, which ends the process.
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
