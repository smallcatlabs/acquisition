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

/// C129 over C24, the login arm: Cloudflare's challenge on the code
/// exchange is no rejected grant. Driven from the browser's callback:
/// the flow fails with words that say Cloudflare answered and the daemon
/// stopped, not with Cloudflare's page or a word to log in again; the
/// block mark is set, and wire == journal == one POST.
#[tokio::test]
async fn a_cloudflare_block_on_the_code_exchange_is_no_rejected_grant() {
    let challenge = ScriptedResponse {
        method: "POST",
        path: "/token",
        ..ScriptedResponse::cloudflare_403_challenge()
    };
    let (base, requests, server) = scripted_server(vec![challenge]).await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon_with(Provider::mock(&base), clock, RailsConfig::default());
    daemon.shared.lock().unwrap().auth.by_account.clear();

    let authorize = url::Url::parse(&daemon.auth_start().await.unwrap()).unwrap();
    let query: HashMap<String, String> = authorize.query_pairs().into_owned().collect();
    let callback = format!(
        "{}?code=code-x&state={}",
        query["redirect_uri"], query["state"]
    );
    let answered = reqwest::get(&callback).await.unwrap();
    assert!(answered.status().is_success());
    let error = loop {
        if let (_, _, Some(error)) = login_result(&daemon) {
            break error;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    };

    assert!(
        error.contains("Cloudflare answered in GGG's place"),
        "{error}"
    );
    assert!(error.contains("reset-tripwire"), "{error}");
    assert!(!error.contains("acq auth"), "{error}");
    assert!(!error.contains("Just a moment"), "{error}");
    let rails = daemon.choke.rails();
    let halt = rails.halted().expect("the block mark is set");
    assert!(
        halt.contains("Cloudflare answered in GGG's place: 403 on POST oauth-token"),
        "{halt}"
    );
    server.await.unwrap();
    assert_eq!(requests.lock().unwrap().as_slice(), ["POST"]);
    assert_journal_matches_wire(&log_path, &requests.lock().unwrap());
    finish_harness(None, &log_path);
}

/// C129, the login arm's announcement: a block on the code exchange is
/// logged as the daemon's error when it lands, as a block on any other
/// send is, and is not logged again as current after `reset-tripwire`:
/// the next block's own cause is the one announced.
#[tokio::test]
async fn a_block_on_the_code_exchange_is_announced_when_it_lands() {
    let challenge = ScriptedResponse {
        method: "POST",
        path: "/token",
        ..ScriptedResponse::cloudflare_403_challenge()
    };
    let responses = vec![
        challenge,
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse::cloudflare_403_challenge(),
    ];
    let (base, requests, server) = scripted_server(responses).await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon_with(Provider::mock(&base), clock, RailsConfig::default());
    daemon.shared.lock().unwrap().auth.by_account.clear();
    let errors = |daemon: &Daemon| -> Vec<String> {
        let s = daemon.shared.lock().unwrap();
        s.errors
            .iter()
            .map(|(_, message)| message.clone())
            .collect()
    };

    let authorize = url::Url::parse(&daemon.auth_start().await.unwrap()).unwrap();
    let query: HashMap<String, String> = authorize.query_pairs().into_owned().collect();
    let callback = format!(
        "{}?code=code-x&state={}",
        query["redirect_uri"], query["state"]
    );
    assert!(reqwest::get(&callback).await.unwrap().status().is_success());
    while login_result(&daemon).2.is_none() {
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    let logged = errors(&daemon);
    assert!(
        logged
            .iter()
            .any(|e| e.starts_with("SENDS STOPPED: ") && e.contains("403 on POST oauth-token")),
        "the exchange's block is announced: {logged:?}"
    );

    daemon.reset_rails().unwrap();
    let before = errors(&daemon).len();
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let (info, _) = wait_terminal(&daemon, id).await;
    assert_eq!(info.state, JobState::Failed);
    let logged = errors(&daemon);
    let stopped: Vec<&String> = logged[before..]
        .iter()
        .filter(|e| e.starts_with("SENDS STOPPED: "))
        .collect();
    assert!(
        stopped.len() == 1 && stopped[0].contains("403 on GET fetch"),
        "after the reset, only the next block's own cause is announced: {:?}",
        &logged[before..]
    );
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
}

/// C25, the login arm: a flow a newer login superseded sends nothing.
/// Its callback still lands (the browser was already sent), but its code
/// exchange is turned away before the gate admits it: no POST reaches
/// the wire, the newer flow stays pending, and the journal agrees.
#[tokio::test]
async fn a_superseded_login_flow_sends_no_code_exchange() {
    let grant = ScriptedResponse {
        method: "POST",
        path: "/token",
        status: "200 OK",
        headers: String::new(),
        body: json!({
            "access_token": "at-stale",
            "refresh_token": "rt-stale",
            "expires_in": 3600,
            "username": "test-user",
        })
        .to_string(),
    };
    let (base, requests, server) = scripted_server(vec![grant]).await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon_with(Provider::mock(&base), clock, RailsConfig::default());
    daemon.shared.lock().unwrap().auth.by_account.clear();

    let first = url::Url::parse(&daemon.auth_start().await.unwrap()).unwrap();
    let _second = daemon.auth_start().await.unwrap();
    let query: HashMap<String, String> = first.query_pairs().into_owned().collect();
    let callback = format!(
        "{}?code=code-x&state={}",
        query["redirect_uri"], query["state"]
    );
    assert!(reqwest::get(&callback).await.unwrap().status().is_success());
    tokio::time::timeout(Duration::from_secs(5), async {
        while !std::fs::read_to_string(&log_path)
            .unwrap_or_default()
            .contains("stale auth flow completion ignored")
        {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("the superseded flow finished");

    assert!(
        requests.lock().unwrap().is_empty(),
        "the superseded flow sent its exchange: {:?}",
        requests.lock().unwrap()
    );
    assert_eq!(login_result(&daemon), (false, None, None));
    assert!(daemon.shared.lock().unwrap().auth.pending.is_some());
    assert_journal_matches_wire(&log_path, &requests.lock().unwrap());
    server.abort();
    finish_harness(None, &log_path);
}

/// C131, the login arm: a code exchange answered 200 with its tokens but
/// no rate-limit header at all installs them, and the login completes
/// on them; the exchange stays a journaled protocol failure.
#[tokio::test]
async fn a_headerless_code_exchange_2xx_installs_its_tokens() {
    let grant = ScriptedResponse {
        method: "POST",
        path: "/token",
        status: "200 OK",
        headers: String::new(),
        body: json!({
            "access_token": "at-new",
            "refresh_token": "rt-rotated",
            "expires_in": 3600,
            "username": "test-user",
        })
        .to_string(),
    };
    // GGG serves `/profile` without rate headers (C32).
    let profile = ScriptedResponse {
        method: "GET",
        path: "/profile",
        status: "200 OK",
        headers: String::new(),
        body: json!({ "uuid": "uuid-test-user", "name": "test-user" }).to_string(),
    };
    let (base, requests, server) = scripted_server(vec![grant, profile]).await;
    let clock = Arc::new(ManualClock::new());
    let log_path = test_log_path();
    let _ = std::fs::remove_file(journal_of(&log_path));
    let rails = Arc::new(Rails::with_config_and_clock(
        RailsConfig {
            journal_path: Some(journal_of(&log_path)),
            ..RailsConfig::default()
        },
        clock.clone(),
    ));
    let credentials = Arc::new(RecordingCredentialStore::default());
    let (mut daemon, log_path) = test_daemon_scenario_at(
        Provider::mock(&base),
        clock,
        rails,
        credentials.clone() as Arc<dyn CredentialStore>,
        log_path,
    );
    let store_dir = std::env::temp_dir().join(format!(
        "acq-login-{}-{}",
        std::process::id(),
        TEST_LOG_ID.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&store_dir);
    Arc::get_mut(&mut daemon).expect("fresh daemon").store_dir = Some(store_dir.clone());
    daemon.shared.lock().unwrap().auth.by_account.clear();
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());

    let authorize = url::Url::parse(&daemon.auth_start().await.unwrap()).unwrap();
    let query: HashMap<String, String> = authorize.query_pairs().into_owned().collect();
    let callback = format!(
        "{}?code=code-x&state={}",
        query["redirect_uri"], query["state"]
    );
    assert!(reqwest::get(&callback).await.unwrap().status().is_success());
    let (logged_in, ok, error) = tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let result = login_result(&daemon);
            if result.1.is_some() || result.2.is_some() {
                break result;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("the login finished");
    assert_eq!(
        (logged_in, ok.as_deref(), error.as_deref()),
        (true, Some("test-user"), None)
    );
    {
        let s = daemon.shared.lock().unwrap();
        let session = s.auth.find("test-user").expect("registered");
        assert_eq!(session.access_token.as_deref(), Some("at-new"));
        assert_eq!(session.refresh_token.as_deref(), Some("rt-rotated"));
    }
    assert_eq!(
        credentials
            .saves
            .lock()
            .unwrap()
            .iter()
            .map(|(_, token, user)| (token.as_str(), user.as_str()))
            .collect::<Vec<_>>(),
        [("rt-rotated", "test-user")]
    );
    let sends = journal_sends(&log_path);
    assert_eq!(sends[0]["method"], "POST");
    assert_eq!(sends[0]["ok"], false, "{}", sends[0]);
    server.await.unwrap();
    assert_journal_matches_wire(&log_path, &requests.lock().unwrap());
    finish_harness(dispatcher, &log_path);
    let _ = std::fs::remove_dir_all(&store_dir);
}

/// C47 with C134, the login arm: a code exchange whose `expires_in` no
/// clock can add fails the login as the provider's error, stages and
/// saves nothing, gives back its count for the exit's drain, and leaves
/// `shared` unpoisoned, so the daemon still answers. Before, the sum
/// panicked under `shared` in the exchange's task and wedged the daemon.
#[tokio::test]
async fn an_out_of_range_expires_in_is_a_token_error() {
    let grant = ScriptedResponse {
        method: "POST",
        path: "/token",
        status: "200 OK",
        headers: "X-Rate-Limit-Policy: token-request-limit\r\nX-Rate-Limit-Rules: Ip\r\nX-Rate-Limit-Ip: 60:30:30\r\nX-Rate-Limit-Ip-State: 1:30:0\r\n".into(),
        body: json!({
            "access_token": "at-huge",
            "refresh_token": "rt-huge",
            "expires_in": u64::MAX,
            "username": "test-user",
        })
        .to_string(),
    };
    let (base, requests, server) = scripted_server(vec![grant]).await;
    let (daemon, credentials, store_dir, log_path) = login_daemon(&base);

    let authorize = url::Url::parse(&daemon.auth_start().await.unwrap()).unwrap();
    let query: HashMap<String, String> = authorize.query_pairs().into_owned().collect();
    let callback = format!(
        "{}?code=code-x&state={}",
        query["redirect_uri"], query["state"]
    );
    assert!(reqwest::get(&callback).await.unwrap().status().is_success());
    let (logged_in, ok, error) = tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let result = login_result(&daemon);
            if result.1.is_some() || result.2.is_some() {
                break result;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("the login finished");
    assert_eq!((logged_in, ok.as_deref()), (false, None));
    let error = error.expect("the login failed");
    assert!(error.contains("token exchange failed"), "{error}");
    assert!(error.contains("lifetime"), "{error}");
    assert!(
        !error.contains("at-huge") && !error.contains("rt-huge"),
        "{error}"
    );
    assert!(!daemon.shared.is_poisoned());
    {
        let s = daemon.shared.lock().unwrap();
        assert!(s.auth.staging.is_none(), "nothing staged");
        assert_eq!(s.auth.exchanges, 0, "the exchange gave its count back");
    }
    assert!(
        credentials.saves.lock().unwrap().is_empty(),
        "nothing saved"
    );
    server.await.unwrap();
    assert_eq!(requests.lock().unwrap().as_slice(), ["POST"], "one POST");
    finish_harness(None, &log_path);
    let _ = std::fs::remove_dir_all(&store_dir);
}

/// C144 (a staged login token serves the login's own profile job alone),
/// issue #71: in the window between the token exchange and the login's
/// completion, a client's `profile` job for the account being logged in
/// sends on the account's session — a refresh, then its GET with the
/// session's token — never with the staged token, and writes nothing of
/// the login's. A client's profile job for an account with no session
/// fails `not_logged_in` and sends nothing. The login's own profile job,
/// submitted after, carries the staged token.
#[tokio::test]
async fn c144_a_client_profile_job_in_the_staging_window_never_gets_the_staged_token() {
    let (base, sent_log, _arrived, _release) = account_server(("GET", "/never", 0)).await;
    let (mut daemon, log_path) = test_daemon(&base, Arc::new(ManualClock::new()));
    logged_in(&mut daemon);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());

    let generation = daemon.begin_auth_flow();
    daemon
        .stage_auth_flow(generation, login_tokens("at-staged", "test-user"))
        .expect("current flow stages");
    let staged = |daemon: &Daemon| {
        daemon
            .shared
            .lock()
            .unwrap()
            .auth
            .staging
            .as_ref()
            .map(|st| st.username.clone())
    };
    assert_eq!(staged(&daemon).as_deref(), Some("test-user"));

    let client = daemon
        .submit(
            "profile".into(),
            json!({}),
            0,
            "cli:1".into(),
            Some("test-user".into()),
        )
        .unwrap();
    let (info, outcome) = wait_terminal(&daemon, client).await;
    assert_eq!(info.state, JobState::Done, "{outcome:?}");
    assert_eq!(
        sent_log.lock().unwrap().as_slice(),
        [sent("test-user", "POST"), sent("test-user", "GET")],
        "the client's fetch ran on the session: its refresh, then its GET"
    );
    {
        let s = daemon.shared.lock().unwrap();
        let staging = s.auth.staging.as_ref().expect("the login is still staged");
        assert_eq!(
            staging.profile_job, None,
            "the client's job was not the login's"
        );
        assert_eq!(
            s.auth.pending,
            Some(generation),
            "the login is still pending"
        );
    }

    // An account with no session: the staged slot is not its fallback.
    let generation_carol = daemon.begin_auth_flow();
    daemon
        .stage_auth_flow(generation_carol, login_tokens("at-staged", "carol"))
        .expect("current flow stages");
    let carol = daemon
        .submit(
            "profile".into(),
            json!({}),
            0,
            "cli:1".into(),
            Some("carol".into()),
        )
        .unwrap();
    let (info, outcome) = wait_terminal(&daemon, carol).await;
    assert_eq!(info.state, JobState::Failed, "{outcome:?}");
    assert_eq!(
        sent_log.lock().unwrap().len(),
        2,
        "nothing was sent for carol"
    );

    // The login's own profile job carries the staged token.
    let login = daemon
        .submit_login_profile(generation_carol, "carol")
        .unwrap();
    let (info, outcome) = wait_terminal(&daemon, login).await;
    assert_eq!(info.state, JobState::Done, "{outcome:?}");
    assert_eq!(
        sent_log.lock().unwrap().last(),
        Some(&sent("staged", "GET")),
        "the login's own fetch sent with the staged token"
    );
    finish_harness(dispatcher, &log_path);
}
