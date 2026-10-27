//! C148, the credential I/O off the shared lock: while a keyring save
//! stands, as a keychain prompt holds one, `hello` is answered, a send
//! past admission reads the token the refresh installed, the status
//! documents answer, and a second login is refused. Each schedule is
//! staged with the recorder's save held until the test releases it,
//! never with a clock (C137). Each wait on the daemon is bounded only so
//! that a regression fails instead of hanging: the property is that the
//! answer arrives while the save is still held, which the test releases
//! only after it has looked.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::kinds::SessionBearer;
use crate::daemon::*;
use tokio::sync::oneshot;

/// The liveness guard on each wait: far past any healthy answer, short
/// of the runner's own kill.
const ANSWERED_WITHIN: Duration = Duration::from_secs(30);

/// `work`'s answer, or `None` past [`ANSWERED_WITHIN`]. The work runs as
/// a task and the wait is a std channel's, on this thread: a daemon that
/// blocks a runtime worker on a lock can starve tokio's own timers, and a
/// guard on them then never fires.
fn answered<T: Send + 'static>(
    work: impl std::future::Future<Output = T> + Send + 'static,
) -> Option<T> {
    let (tx, rx) = std::sync::mpsc::channel();
    tokio::spawn(async move {
        let _ = tx.send(work.await);
    });
    rx.recv_timeout(ANSWERED_WITHIN).ok()
}

/// A save or a clear held at its start: `started` fires when it is
/// under way, and it goes on when `release` is sent (or dropped).
struct HeldSave {
    started: oneshot::Receiver<()>,
    release: std::sync::mpsc::Sender<()>,
}

/// Hold the next call `slot` (the recorder's `on_save` or `on_clear`)
/// runs until the test releases it.
fn hold_the_next(slot: &Mutex<Option<Box<dyn FnOnce() + Send>>>) -> HeldSave {
    let (started_tx, started) = oneshot::channel();
    let (release, released) = std::sync::mpsc::channel::<()>();
    *slot.lock().unwrap() = Some(Box::new(move || {
        let _ = started_tx.send(());
        let _ = released.recv();
    }));
    HeldSave { started, release }
}

/// Hold the next save `store` makes until the test releases it.
fn hold_the_next_save(store: &RecordingCredentialStore) -> HeldSave {
    hold_the_next(&store.on_save)
}

/// A logged-in harness daemon facing the account server (`rt-old` is
/// `test-user`'s grant), its access token `at-old`, its credential store
/// a recorder whose next save is held.
async fn saving_daemon() -> (
    Arc<Daemon>,
    Arc<RecordingCredentialStore>,
    HeldSave,
    Arc<Mutex<Vec<Sent>>>,
    String,
    TestPath,
) {
    let (base, sent_log, _arrived, _release) = account_server(("GET", "/never", 0)).await;
    let (mut daemon, log_path) = test_daemon(&base, Arc::new(ManualClock::new()));
    logged_in(&mut daemon);
    let store = Arc::new(RecordingCredentialStore::default());
    let held = hold_the_next_save(&store);
    Arc::get_mut(&mut daemon).unwrap().credential_store = store.clone();
    {
        let wall = daemon.choke.wall();
        let mut s = daemon.shared.lock().unwrap();
        let session = s.auth.one_mut();
        session.access_token = Some("at-old".into());
        session.access_expires_at = Some(wall + Duration::from_secs(3600));
    }
    (daemon, store, held, sent_log, base, log_path)
}

/// A refresh forced as `auth check` forces one, its landing held in the
/// save; returns once the save is under way.
async fn refresh_held_in_its_save(
    daemon: &Arc<Daemon>,
    held: &mut HeldSave,
) -> tokio::task::JoinHandle<Result<(String, String), Refusal>> {
    let refresh = {
        let daemon = daemon.clone();
        tokio::spawn(async move { daemon.valid_access_token(None, true, None).await })
    };
    (&mut held.started)
        .await
        .expect("the refresh reached its save");
    refresh
}

fn keyring_of(daemon: &Daemon, account: &str) -> String {
    daemon
        .shared
        .lock()
        .unwrap()
        .auth
        .find(account)
        .expect("the session is live")
        .keyring
        .clone()
}

/// C148: a refresh whose keyring save stands, as a keychain prompt holds
/// one, leaves `hello` answered: a client connecting over the socket is
/// greeted while the save is held, where it waited behind the lock the
/// save ran under and its handshake deadline called the daemon
/// unresponsive. Released, the save lands and the session's mark is `ok`.
#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn c148_hello_is_answered_while_a_refresh_saves() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
    let (daemon, store, mut held, sent_log, _base, log_path) = saving_daemon().await;
    let refresh = refresh_held_in_its_save(&daemon, &mut held).await;

    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("c148.sock");
    let (listener, bound) = crate::daemon::endpoint::Listener::bind(&path)
        .await
        .unwrap();
    let serving = tokio::spawn(daemon.clone().serve(listener, Duration::from_millis(1)));
    let socket = path.clone();
    let greeted = answered(async move {
        let stream = tokio::net::UnixStream::connect(&socket).await?;
        let (read, mut write) = tokio::io::split(stream);
        write.write_all(b"{\"req\":\"hello\"}\n").await?;
        let mut line = String::new();
        tokio::io::BufReader::new(read).read_line(&mut line).await?;
        std::io::Result::Ok(line)
    });
    let save_still_held = store.saves.lock().unwrap().is_empty();
    held.release.send(()).unwrap();

    let answer = greeted
        .expect("hello is answered while the save is held")
        .expect("the connection answered");
    assert!(
        save_still_held,
        "the save had not landed when hello was answered"
    );
    assert!(
        matches!(
            acquisition_protocol::protocol::BootstrapReply::read(answer.as_bytes()),
            Some(acquisition_protocol::protocol::BootstrapReply::Hello { .. })
        ),
        "{answer}"
    );
    let (token, _) = refresh.await.unwrap().expect("the refresh lands");
    assert_eq!(token, "at-test-user");
    assert_eq!(keyring_of(&daemon, "test-user"), "ok");
    serving.abort();
    bound.remove_if_still_ours();
    assert_journal_matches_wire(&log_path, &wire_methods(&sent_log));
    finish_harness(None, &log_path);
}

/// C148: a send past admission (the route's first, its HEAD probe),
/// holding its gate permit, reads its bearer from the session while a
/// refresh's save is held, and carries the token that refresh installed,
/// where it waited behind the lock the save ran under, its permit held.
/// Under the read the session's mark says the save has not landed;
/// released, it lands `ok`.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn c148_a_permit_holder_reads_the_installed_token_while_the_save_blocks() {
    let (daemon, _store, mut held, sent_log, base, log_path) = saving_daemon().await;
    let refresh = refresh_held_in_its_save(&daemon, &mut held).await;

    let carried = {
        let daemon = daemon.clone();
        let url = format!("{base}/profile");
        answered(async move {
            let bearer = SessionBearer::new(&daemon, "test-user", "at-old");
            let ready = daemon.choke.now();
            daemon
                .choke
                .head("c148-bearer", &url, Some(&bearer), ready, ALWAYS_WANTED)
                .await
                // The account server answers without rate headers, as
                // GGG serves the profile, so the probe may not parse;
                // what matters is that it went out, and with what.
                .err()
                .map(|e| e.to_string())
        })
    };
    let mark_under_the_read = daemon
        .shared
        .try_lock()
        .ok()
        .and_then(|s| s.auth.find("test-user").map(|x| x.keyring.clone()));
    held.release.send(()).unwrap();

    let refused = carried.expect("the bearer is read while the save is held");
    assert!(
        !matches!(refused.as_deref(), Some(e) if e.contains("rejected before it left the gate")),
        "the send went out: {refused:?}"
    );
    assert_eq!(
        sent_log.lock().unwrap().as_slice(),
        [sent("test-user", "POST"), sent("test-user", "HEAD")],
        "the probe carried the token the refresh installed"
    );
    assert_eq!(mark_under_the_read.as_deref(), Some("saving"));
    let (token, _) = refresh.await.unwrap().expect("the refresh lands");
    assert_eq!(token, "at-test-user");
    assert_eq!(keyring_of(&daemon, "test-user"), "ok");
    assert_journal_matches_wire(&log_path, &wire_methods(&sent_log));
    finish_harness(None, &log_path);
}

/// C148: a login whose keyring save stands leaves the status documents
/// answered, `auth status` saying the login is pending, and refuses a
/// second login while the first is being recorded, where both waited
/// behind the lock the save ran under. A second start then would have
/// cleared the flow mid-record: the credential saved, the session never
/// registered. Released, the first login registers its session.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn c148_status_answers_and_a_second_login_is_refused_while_a_login_lands() {
    let (daemon, log_path) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
    let store = Arc::new(RecordingCredentialStore::default());
    let mut held = hold_the_next_save(&store);
    let store_dir = TestPath::dir();
    let mut daemon = daemon;
    {
        let d = Arc::get_mut(&mut daemon).unwrap();
        d.credential_store = store.clone();
        d.store_dir = Some(store_dir.to_path_buf());
    }
    daemon.shared.lock().unwrap().auth.by_account.clear();
    let generation = daemon.begin_auth_flow();
    daemon
        .stage_auth_flow(
            generation,
            auth::TokenResponse {
                access_token: "at-alice".into(),
                refresh_token: "rt-alice".into(),
                expires_in: 3600,
                username: "Alice#1".into(),
            },
        )
        .expect("current flow stages");
    let landing = {
        let daemon = daemon.clone();
        tokio::task::spawn_blocking(move || daemon.complete_login(generation, "Alice#1", "u-a"))
    };
    (&mut held.started)
        .await
        .expect("the login reached its save");

    let answers = {
        let daemon = daemon.clone();
        answered(async move {
            let mut answers = Vec::new();
            for req in [
                Request::DaemonStatus,
                Request::AuthStatus,
                Request::AuthStart,
            ] {
                answers.push(daemon.handle_request(req, &mut None).await);
            }
            answers
        })
    };
    let save_still_held = store.saves.lock().unwrap().is_empty();
    held.release.send(()).unwrap();

    let [daemon_status, auth_status, second]: [Response; 3] = answers
        .expect("the status documents and the second start answer while the save is held")
        .try_into()
        .unwrap();
    assert!(
        save_still_held,
        "the save had not landed when they answered"
    );
    assert!(
        matches!(daemon_status, Response::DaemonStatus { .. }),
        "{daemon_status:?}"
    );
    assert!(
        matches!(auth_status, Response::Auth { pending: true, .. }),
        "{auth_status:?}"
    );
    match second {
        Response::Error { kind, message } => {
            assert_eq!(kind, ErrorKind::Refused);
            assert_eq!(message, "a login is being recorded; try again in a moment");
        }
        other => panic!("the second login was not refused: {other:?}"),
    }
    assert!(landing.await.unwrap(), "the first login registered");
    let s = daemon.shared.lock().unwrap();
    assert_eq!(s.auth.find("Alice#1").unwrap().keyring, "ok");
    assert_eq!(s.auth.flow_result, Some(Ok("Alice#1".to_string())));
    assert!(s.auth.pending.is_none());
    drop(s);
    finish_harness(None, &log_path);
}

/// C148, the review of r2, its second finding: a logout's keyring clears
/// and its not-persisted mark are the exit's drain's until they land.
/// Before, the drain looked only at `shared`, which the logout no longer
/// holds across its I/O, so a stop asked while a clear stood found
/// nothing in flight and exited before the mark was written, and the
/// next lifetime restored the account the user had logged out of. Every
/// landing is now counted from before its spawn until it returns.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn c148_a_stop_waits_for_a_logout_whose_clear_is_held() {
    let (mut daemon, log_path) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
    logged_in(&mut daemon);
    let store = Arc::new(RecordingCredentialStore::default());
    store
        .entries
        .lock()
        .unwrap()
        .insert("test-user".into(), "rt-old".into());
    let mut held = hold_the_next(&store.on_clear);
    let dir = TestPath::dir();
    Index::load(&dir)
        .unwrap()
        .record_login("test-user", "u-1", true, 1, &[])
        .unwrap();
    {
        let d = Arc::get_mut(&mut daemon).unwrap();
        d.credential_store = store.clone();
        d.store_dir = Some(dir.to_path_buf());
        d.exit = Exit::with_bound(ANSWERED_WITHIN);
    }
    let logout = {
        let daemon = daemon.clone();
        tokio::spawn(async move {
            daemon
                .handle_request(
                    Request::AuthLogout {
                        account: Some("test-user".into()),
                    },
                    &mut None,
                )
                .await
        })
    };
    (&mut held.started)
        .await
        .expect("the logout reached its clear");

    assert!(daemon.begin_exit("test: exit"));
    let drain = {
        let daemon = daemon.clone();
        tokio::spawn(async move { daemon.drain().await })
    };
    tokio::time::sleep(Duration::from_millis(100)).await;
    let waited = !drain.is_finished();
    held.release.send(()).unwrap();

    assert!(waited, "the drain waits for the logout's clear");
    assert_eq!(drain.await.unwrap(), None, "nothing was cut");
    assert!(
        matches!(logout.await.unwrap(), Response::Ack { .. }),
        "the logout landed"
    );
    assert!(
        !Index::load(&dir)
            .unwrap()
            .get("test-user")
            .unwrap()
            .persisted,
        "the not-persisted mark landed before the exit"
    );
    finish_harness(None, &log_path);
}

/// C148, the review of r2, its first finding: a refresh's published
/// token serves no caller that comes after it has left the session.
/// Before, the flight stayed open, and joinable, until the save landed:
/// a 401 on the installed token while the save was held cleared it from
/// the session (C87, a dead bearer), and the next lookup joined the open
/// flight and was handed the rejected token, so its send went stale
/// twice and the job gave up, and `auth check` could report success.
/// The flight now closes at the install; the next lookup refreshes, and
/// its landing waits for the held one.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn c148_a_token_rejected_while_its_save_is_held_is_not_handed_out_again() {
    let (daemon, _store, mut held, sent_log, _base, log_path) = saving_daemon().await;
    let refresh = refresh_held_in_its_save(&daemon, &mut held).await;
    assert_eq!(
        daemon.reject_access_token("at-test-user").as_deref(),
        Some("test-user"),
        "the installed token is rejected while its save is held"
    );

    let looked_up = Arc::new(AtomicBool::new(false));
    let lookup = {
        let daemon = daemon.clone();
        let looked_up = looked_up.clone();
        tokio::spawn(async move {
            let token = daemon.valid_access_token(None, false, None).await;
            looked_up.store(true, Ordering::SeqCst);
            token
        })
    };
    let posts = {
        let sent_log = sent_log.clone();
        move || {
            sent_log
                .lock()
                .unwrap()
                .iter()
                .filter(|(method, _)| method == "POST")
                .count()
        }
    };
    let waited = {
        let posts = posts.clone();
        answered(async move {
            while !looked_up.load(Ordering::SeqCst) && posts() < 2 {
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
    };
    let posts_while_held = posts();
    held.release.send(()).unwrap();

    waited.expect("the lookup refreshed or answered while the save was held");
    assert_eq!(
        posts_while_held, 2,
        "the next lookup sent a refresh of its own, not the rejected token"
    );
    let (token, _) = lookup.await.unwrap().expect("the second refresh lands");
    assert_eq!(token, "at-test-user");
    refresh.await.unwrap().expect("the first refresh lands");
    assert_journal_matches_wire(&log_path, &wire_methods(&sent_log));
    finish_harness(None, &log_path);
}

/// The panic hook's exit status in the child below, as `exit.rs`'s.
const LANDING_HOOK_ENDED: i32 = 86;
/// Set in the child's environment: the directory its hook writes to.
const LANDING_CHILD_DIR: &str = "ACQ_TEST_LANDING_PANIC_DIR";

fn landing_hook_ended() -> ! {
    std::process::exit(LANDING_HOOK_ENDED)
}

/// The child: the hook installed as `run` installs it, then job 7's
/// forced refresh, whose keyring save panics on its blocking thread.
fn landing_panic_child(dir: &std::path::Path) -> ! {
    let log = std::fs::File::create(dir.join("daemon.log")).unwrap();
    super::super::start::install_panic_hook(
        log,
        Some(dir.join("fault.json")),
        std::time::Instant::now(),
        landing_hook_ended,
    );
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let (base, _sent, _arrived, _release) = account_server(("GET", "/never", 0)).await;
        let (mut daemon, _log_path) = test_daemon(&base, Arc::new(ManualClock::new()));
        logged_in(&mut daemon);
        let store = RecordingCredentialStore::default();
        *store.on_save.lock().unwrap() = Some(Box::new(|| {
            panic!("a broken invariant in a landing, for the hook");
        }));
        Arc::get_mut(&mut daemon).unwrap().credential_store = Arc::new(store);
        let _ = RUNNING_JOB
            .scope(7, daemon.valid_access_token(None, true, None))
            .await;
    });
    // The landing's panic reached no hook that ended the process.
    std::process::exit(1)
}

/// C148, the review of r2, its third finding: a panic in a landing that
/// a job's task spawned names the job (C134: its fault marker fails the
/// job once on the next start instead of replaying it). Before, the
/// blocking thread ran outside the job's mark, the hook wrote no marker,
/// and the job that met the fault was replayed. Run as `exit.rs` runs
/// C134's: this test's own binary as a child process.
#[test]
fn c148_a_panic_in_a_jobs_landing_marks_the_job() {
    if let Some(dir) = std::env::var_os(LANDING_CHILD_DIR) {
        landing_panic_child(std::path::Path::new(&dir));
    }
    let dir = TestPath::dir();
    let module = module_path!()
        .split_once("::")
        .map_or(module_path!(), |(_, rest)| rest);
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            format!("{module}::c148_a_panic_in_a_jobs_landing_marks_the_job"),
            "--exact".into(),
            "--nocapture".into(),
            "--test-threads=1".into(),
        ])
        .env(LANDING_CHILD_DIR, &*dir)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(LANDING_HOOK_ENDED), "{status}");
    let log = std::fs::read_to_string(dir.join("daemon.log")).unwrap();
    assert!(log.contains("PANIC in job 7 at "), "{log}");
    assert!(
        log.contains("a broken invariant in a landing, for the hook"),
        "{log}"
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("fault.json"))
            .ok()
            .as_deref(),
        Some("{\"job\":7}\n"),
        "the marker names the job whose landing panicked"
    );
}
