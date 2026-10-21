//! The login flow: the session registered only when the profile's uuid
//! lands, and every way a login fails whole.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::*;
use std::sync::atomic::Ordering;

// ---- tracer step 2: uuid-at-login (C50) ------------------------------

fn login_tokens(access: &str, user: &str) -> auth::TokenResponse {
    auth::TokenResponse {
        access_token: access.into(),
        refresh_token: format!("rt-{user}"),
        expires_in: 3600,
        username: user.into(),
    }
}

/// A daemon wired for login tests: a recording credential store, a
/// scratch store directory, and no pre-existing session.
fn login_daemon(base: &str) -> (Arc<Daemon>, Arc<RecordingCredentialStore>, PathBuf, PathBuf) {
    let clock = Arc::new(ManualClock::new());
    let rails = Arc::new(Rails::with_config_and_clock(
        RailsConfig::default(),
        clock.clone(),
    ));
    let credentials = Arc::new(RecordingCredentialStore::default());
    let (mut daemon, log_path) = test_daemon_scenario(
        Provider::mock(base),
        clock,
        rails,
        credentials.clone() as Arc<dyn CredentialStore>,
    );
    let store_dir = std::env::temp_dir().join(format!(
        "acq-login-{}-{}",
        std::process::id(),
        TEST_LOG_ID.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&store_dir);
    Arc::get_mut(&mut daemon).expect("fresh daemon").store_dir = Some(store_dir.clone());
    // The harness pre-installs an empty legacy session; a login test
    // starts logged out.
    daemon.shared.lock().unwrap().auth.by_account.clear();
    (daemon, credentials, store_dir, log_path)
}

async fn run_login(daemon: &Arc<Daemon>, access: &str, user: &str) {
    let generation = daemon.begin_auth_flow();
    let username = daemon
        .stage_auth_flow(generation, login_tokens(access, user))
        .expect("current flow stages");
    daemon.login_with_profile(generation, username).await;
}

fn login_result(daemon: &Daemon) -> (bool, Option<String>, Option<String>) {
    match daemon.auth_status() {
        Response::Auth {
            logged_in,
            login_ok,
            login_error,
            ..
        } => (logged_in, login_ok, login_error),
        other => panic!("unexpected response: {other:?}"),
    }
}

/// A login is not a login until the profile fetch delivers the account
/// uuid: until then nothing is registered — no session, no keyring
/// entry, no index row — and when it lands, everything is, with the
/// uuid (deterministic per username on the mock).
#[tokio::test]
async fn login_registers_the_session_only_when_the_profile_uuid_lands() {
    let base = mockggg::start().await.unwrap();
    let (daemon, credentials, store_dir, log_path) = login_daemon(&base);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());

    let generation = daemon.begin_auth_flow();
    let username = daemon
        .stage_auth_flow(generation, login_tokens("at-x.Alice#1234", "Alice#1234"))
        .expect("current flow stages");
    // Staged: the flow is still pending and nothing is registered —
    // the tokens sit in the staging slot, outside the session map.
    {
        let s = daemon.shared.lock().unwrap();
        assert_eq!(s.auth.pending, Some(generation));
        assert!(s.auth.by_account.is_empty(), "no session exists yet");
        assert!(s.auth.staging.is_some());
    }
    assert!(matches!(
        daemon.auth_status(),
        Response::Auth {
            logged_in: false,
            pending: true,
            login_ok: None,
            login_error: None,
            ..
        }
    ));
    assert!(credentials.saves.lock().unwrap().is_empty());
    assert!(!acquisition_store::index_path(&store_dir).exists());

    daemon.login_with_profile(generation, username).await;

    // Registered: session visible, keyring written, index row with
    // uuid, and the flow's own terminal result names the account.
    let index = Index::load(&store_dir).unwrap();
    let entry = index.get("Alice#1234").expect("indexed at login");
    let uuid = entry.uuid.clone().expect("uuid recorded at login");
    assert!(entry.persisted);
    {
        let s = daemon.shared.lock().unwrap();
        assert_eq!(s.auth.pending, None);
        assert!(s.auth.staging.is_none());
        assert_eq!(s.auth.last_login.as_deref(), Some("Alice#1234"));
        let session = s.auth.find("Alice#1234").expect("registered");
        assert_eq!(session.keyring, "ok");
        assert_eq!(session.uuid.as_deref(), Some(uuid.as_str()));
    }
    assert_eq!(credentials.saves.lock().unwrap().len(), 1);
    assert_eq!(
        login_result(&daemon),
        (true, Some("Alice#1234".into()), None)
    );

    // A re-login as the same account lands the same uuid: the mock's
    // uuids are deterministic per username, and the index keeps one
    // entry per account.
    run_login(&daemon, "at-y.Alice#1234", "Alice#1234").await;
    let index = Index::load(&store_dir).unwrap();
    assert_eq!(index.entries().len(), 1);
    assert_eq!(
        index.get("Alice#1234").unwrap().uuid.as_deref(),
        Some(uuid.as_str())
    );

    dispatcher.abort();
    let _ = std::fs::remove_dir_all(&store_dir);
    let _ = std::fs::remove_file(&log_path);
}

/// A login whose profile fetch fails fails whole: no provisional
/// identity — the staged tokens are dropped, no keyring entry is
/// minted, the index is untouched, and the flow's terminal result is
/// the failure.
#[tokio::test]
async fn a_login_whose_profile_fetch_fails_fails_whole() {
    // Nothing listens here: the profile GET dies in transport.
    let (daemon, credentials, store_dir, log_path) = login_daemon("http://127.0.0.1:1");
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());

    run_login(&daemon, "at-x.Alice#1234", "Alice#1234").await;

    {
        let s = daemon.shared.lock().unwrap();
        assert_eq!(s.auth.pending, None, "the flow is closed");
        assert!(s.auth.by_account.is_empty(), "no session appeared");
        assert!(s.auth.staging.is_none(), "the staged tokens are gone");
        assert!(
            s.errors
                .iter()
                .any(|(_, m)| m.contains("login failed for Alice#1234")),
            "the failure is reported"
        );
    }
    let (logged_in, ok, error) = login_result(&daemon);
    assert!(!logged_in && ok.is_none());
    assert!(
        error
            .as_deref()
            .is_some_and(|e| e.contains("profile fetch")),
        "{error:?}"
    );
    assert!(credentials.saves.lock().unwrap().is_empty());
    assert!(!acquisition_store::index_path(&store_dir).exists());

    dispatcher.abort();
    let _ = std::fs::remove_dir_all(&store_dir);
    let _ = std::fs::remove_file(&log_path);
}

/// An aborted *re-login* leaves the previously registered session
/// exactly as it was (staging never touches the session map), and the
/// flow's terminal result reports the failure even though `logged_in`
/// stays true — so `acq auth` cannot mistake the surviving session for
/// this login succeeding. The abort is called here directly: that a
/// profile fetch which really fails reaches it is not driven.
#[tokio::test]
async fn an_aborted_relogin_keeps_the_old_session_and_reports_the_failure() {
    let base = mockggg::start().await.unwrap();
    let (daemon, credentials, store_dir, log_path) = login_daemon(&base);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    run_login(&daemon, "at-x.Alice#1234", "Alice#1234").await;
    assert_eq!(login_result(&daemon).1.as_deref(), Some("Alice#1234"));
    let old_token = {
        let s = daemon.shared.lock().unwrap();
        s.auth.find("Alice#1234").unwrap().access_token.clone()
    };

    // A re-login aborted after staging, as a failed profile fetch
    // aborts it.
    let g2 = daemon.begin_auth_flow();
    daemon
        .stage_auth_flow(g2, login_tokens("at-y.Alice#1234", "Alice#1234"))
        .expect("current flow stages");
    daemon.abort_login(g2, "Alice#1234", "profile fetch failed: simulated");

    {
        let s = daemon.shared.lock().unwrap();
        let session = s.auth.find("Alice#1234").expect("old session survives");
        assert_eq!(
            session.access_token, old_token,
            "the old session is untouched — the staged tokens never reached it"
        );
        assert!(s.auth.staging.is_none());
    }
    let (logged_in, ok, error) = login_result(&daemon);
    assert!(logged_in, "the old session is still live");
    assert!(ok.is_none(), "the failed flow claims no success");
    assert!(error.as_deref().is_some_and(|e| e.contains("simulated")));
    // The keyring still holds exactly the first login's entry.
    assert_eq!(credentials.saves.lock().unwrap().len(), 1);

    dispatcher.abort();
    let _ = std::fs::remove_dir_all(&store_dir);
    let _ = std::fs::remove_file(&log_path);
}

/// A rejected refresh grant marks the account so the dead token is
/// never re-sent; only a completed login clears the mark, since until
/// then the keyring still held the rejected token.
#[tokio::test]
async fn a_completed_login_clears_the_dead_grant_mark() {
    let base = mockggg::start().await.unwrap();
    let (daemon, _credentials, store_dir, log_path) = login_daemon(&base);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let rails = daemon.choke.rails().clone();
    assert!(rails.mark_refresh_failed("Alice#1234", "HTTP 400"));
    assert!(rails.refresh_failed("Alice#1234").is_some());

    run_login(&daemon, "at-x.Alice#1234", "Alice#1234").await;
    assert_eq!(login_result(&daemon).1.as_deref(), Some("Alice#1234"));
    assert_eq!(rails.refresh_failed("Alice#1234"), None);

    dispatcher.abort();
    let _ = std::fs::remove_dir_all(&store_dir);
    let _ = std::fs::remove_file(&log_path);
}

/// A login the account index cannot record fails whole: the session
/// never becomes visible and the keyring entry is rolled back — a
/// session without its uuid mapping could neither be restored nor find
/// its annotation file.
#[tokio::test]
async fn a_login_the_index_cannot_record_fails_whole_and_rolls_back_the_keyring() {
    let base = mockggg::start().await.unwrap();
    let (mut daemon, credentials, store_dir, log_path) = login_daemon(&base);
    // A regular file where the store directory should be: every index
    // read and write fails.
    let blocker = store_dir.with_extension("blocker");
    std::fs::write(&blocker, "not a directory").unwrap();
    Arc::get_mut(&mut daemon).expect("fresh daemon").store_dir = Some(blocker.clone());
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());

    run_login(&daemon, "at-x.Alice#1234", "Alice#1234").await;

    {
        let s = daemon.shared.lock().unwrap();
        assert!(s.auth.by_account.is_empty(), "the session never appeared");
        assert!(s.auth.staging.is_none());
    }
    let (logged_in, ok, error) = login_result(&daemon);
    assert!(!logged_in && ok.is_none());
    assert!(
        error
            .as_deref()
            .is_some_and(|e| e.contains("account index")),
        "{error:?}"
    );
    // The keyring write happened and was rolled back.
    assert_eq!(credentials.saves.lock().unwrap().len(), 1);
    assert_eq!(
        credentials.cleared.lock().unwrap().as_slice(),
        ["Alice#1234"]
    );

    dispatcher.abort();
    let _ = std::fs::remove_file(&blocker);
    let _ = std::fs::remove_dir_all(&store_dir);
    let _ = std::fs::remove_file(&log_path);
}

/// A superseded flow can neither stage nor complete, and its staged
/// tokens do not linger.
#[tokio::test]
async fn a_superseded_login_cannot_complete_and_leaves_no_zombie() {
    let (daemon, credentials, _store_dir, log_path) = login_daemon("http://127.0.0.1:1");

    let g1 = daemon.begin_auth_flow();
    daemon
        .stage_auth_flow(g1, login_tokens("at-x.Alice#1234", "Alice#1234"))
        .expect("current flow stages");
    // The user restarted the login before the profile landed: the new
    // flow owns the login state from here.
    let g2 = daemon.begin_auth_flow();
    assert!(!daemon.complete_login(g1, "Alice#1234", "u-stale"));
    {
        let s = daemon.shared.lock().unwrap();
        assert!(s.auth.by_account.is_empty(), "no session appeared");
        assert!(s.auth.staging.is_none(), "no staged tokens linger");
        assert_eq!(s.auth.pending, Some(g2), "the new flow is untouched");
        assert!(
            s.auth.flow_result.is_none(),
            "the new flow's result is its own"
        );
    }
    assert!(credentials.saves.lock().unwrap().is_empty());
    // A stale token exchange cannot stage either.
    assert!(
        daemon
            .stage_auth_flow(g1, login_tokens("at-y.Alice#1234", "Alice#1234"))
            .is_none()
    );
    let _ = std::fs::remove_file(&log_path);
}
