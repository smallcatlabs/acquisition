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

/// A save held at its start: `started` fires when a save is under way,
/// and the save goes on when `release` is sent (or dropped).
struct HeldSave {
    started: oneshot::Receiver<()>,
    release: std::sync::mpsc::Sender<()>,
}

/// Hold the next save `store` makes until the test releases it.
fn hold_the_next_save(store: &RecordingCredentialStore) -> HeldSave {
    let (started_tx, started) = oneshot::channel();
    let (release, released) = std::sync::mpsc::channel::<()>();
    *store.on_save.lock().unwrap() = Some(Box::new(move || {
        let _ = started_tx.send(());
        let _ = released.recv();
    }));
    HeldSave { started, release }
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

/// C148: a send past admission, holding its gate permit, reads its
/// bearer from the session while a refresh's save is held, and carries
/// the token that refresh installed, where it waited behind the lock the
/// save ran under, its permit held. Under the read the session's mark
/// says the save has not landed; released, it lands `ok`.
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
                .get_bearer("c148-bearer", &url, &bearer, ready, ALWAYS_WANTED)
                .await
                .map(|landed| landed.status.as_u16())
        })
    };
    let mark_under_the_read = daemon
        .shared
        .try_lock()
        .ok()
        .and_then(|s| s.auth.find("test-user").map(|x| x.keyring.clone()));
    held.release.send(()).unwrap();

    let status = carried
        .expect("the bearer is read while the save is held")
        .expect("the send went out");
    assert_eq!(status, 200);
    assert_eq!(
        sent_log.lock().unwrap().as_slice(),
        [sent("test-user", "POST"), sent("test-user", "GET")],
        "the GET carried the token the refresh installed"
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
