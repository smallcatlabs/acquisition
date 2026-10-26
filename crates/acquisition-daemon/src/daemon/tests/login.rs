//! The login flow: the session registered only when the profile's uuid
//! lands, and every way a login fails whole.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::*;
use acquisition_protocol::protocol::KeyringOutcome;

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
fn login_daemon(
    base: &str,
) -> (
    Arc<Daemon>,
    Arc<RecordingCredentialStore>,
    TestPath,
    TestPath,
) {
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
    let store_dir = TestPath::dir();
    Arc::get_mut(&mut daemon).expect("fresh daemon").store_dir = Some(store_dir.to_path_buf());
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
    let (daemon, credentials, store_dir, _log_path) = login_daemon(&base);
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
}

/// A login whose profile fetch fails fails whole: no provisional
/// identity — the staged tokens are dropped, no keyring entry is
/// minted, the index is untouched, and the flow's terminal result is
/// the failure.
#[tokio::test]
async fn a_login_whose_profile_fetch_fails_fails_whole() {
    // Nothing listens here: the profile GET dies in transport.
    let (daemon, credentials, store_dir, _log_path) = login_daemon("http://127.0.0.1:1");
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
    let (daemon, credentials, _store_dir, _log_path) = login_daemon(&base);
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
}

/// A rejected refresh grant marks the account so the dead token is
/// never re-sent; only a completed login clears the mark, since until
/// then the keyring still held the rejected token.
#[tokio::test]
async fn a_completed_login_clears_the_dead_grant_mark() {
    let base = mockggg::start().await.unwrap();
    let (daemon, _credentials, _store_dir, _log_path) = login_daemon(&base);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let rails = daemon.choke.rails().clone();
    assert!(rails.mark_refresh_failed("Alice#1234", "HTTP 400"));
    assert!(rails.refresh_failed("Alice#1234").is_some());

    run_login(&daemon, "at-x.Alice#1234", "Alice#1234").await;
    assert_eq!(login_result(&daemon).1.as_deref(), Some("Alice#1234"));
    assert_eq!(rails.refresh_failed("Alice#1234"), None);

    dispatcher.abort();
}

/// A login the account index cannot record fails whole: the session
/// never becomes visible and no keyring entry is left (the index cannot
/// hold the name, so none is saved) — a session without its uuid
/// mapping could neither be restored nor find its annotation file.
#[tokio::test]
async fn a_login_the_index_cannot_record_fails_whole_and_leaves_no_keyring_entry() {
    let base = mockggg::start().await.unwrap();
    let (daemon, credentials, store_dir, _log_path) = login_daemon(&base);
    // A directory where the index file should be: every index read and
    // write fails, while the profile's facts file opens and records (a
    // store that cannot record fails the profile job first, C144).
    std::fs::create_dir_all(acquisition_store::index_path(&store_dir)).unwrap();
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
    // C144: the index could not hold the name first, so no keyring write
    // happened and there is nothing to roll back.
    assert!(credentials.saves.lock().unwrap().is_empty());
    assert!(credentials.entries.lock().unwrap().is_empty());

    dispatcher.abort();
    drop(daemon);
}

/// A superseded flow can neither stage nor complete, and its staged
/// tokens do not linger.
#[tokio::test]
async fn a_superseded_login_cannot_complete_and_leaves_no_zombie() {
    let (daemon, credentials, _store_dir, _log_path) = login_daemon("http://127.0.0.1:1");

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
    let store_dir = TestPath::dir();
    Arc::get_mut(&mut daemon).expect("fresh daemon").store_dir = Some(store_dir.to_path_buf());
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
    let (daemon, credentials, _store_dir, log_path) = login_daemon(&base);

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

// ---- C144: every credential-store result reaches the caller ----------

/// A logged-in harness daemon whose credential store is `store`, holding
/// test-user's entry, with an account index in a scratch directory.
fn keyring_daemon(
    store: RecordingCredentialStore,
) -> (
    Arc<Daemon>,
    Arc<RecordingCredentialStore>,
    TestPath,
    TestPath,
) {
    let (mut daemon, log_path) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
    logged_in(&mut daemon);
    store
        .entries
        .lock()
        .unwrap()
        .insert("test-user".into(), "rt-old".into());
    let store = Arc::new(store);
    let dir = TestPath::dir();
    Index::load(&dir)
        .unwrap()
        .record_login("test-user", "u-1", true, 1, &[])
        .unwrap();
    let d = Arc::get_mut(&mut daemon).unwrap();
    d.credential_store = store.clone();
    d.store_dir = Some(dir.to_path_buf());
    (daemon, store, dir, log_path)
}

async fn logout_answer(daemon: &Arc<Daemon>, account: Option<&str>) -> Response {
    daemon
        .handle_request(
            Request::AuthLogout {
                account: account.map(str::to_string),
            },
            &mut None,
        )
        .await
}

/// C144, issue #61: a logout whose keyring clear fails answers that on
/// the wire — `internal`, in words that say the session was dropped and
/// the entry was not, and the request that tries again — where it used to
/// answer success. The index marks the account not persisted either way,
/// so no later start restores the session; the retry, a logout of the
/// now not-live account, clears the entry once the keyring works.
#[tokio::test]
async fn c144_a_logout_whose_keyring_clear_fails_says_so_on_the_wire() {
    let (daemon, store, dir, log_path) = keyring_daemon(RecordingCredentialStore::failing(
        "clear",
        "keychain locked",
    ));

    let answer = logout_answer(&daemon, None).await;
    let Response::Error { kind, message } = &answer else {
        panic!("the failed clear is the answer: {answer:?}");
    };
    assert_eq!(*kind, ErrorKind::Internal);
    for fact in [
        "logged out test-user (session dropped, not restored at start), but",
        "its keyring entry could not be cleared: keychain locked",
        "`acq auth logout --account test-user` tries again",
    ] {
        assert!(message.contains(fact), "{fact:?} in {message}");
    }
    assert!(
        daemon
            .shared
            .lock()
            .unwrap()
            .auth
            .find("test-user")
            .is_none()
    );
    assert!(
        !Index::load(&dir)
            .unwrap()
            .get("test-user")
            .unwrap()
            .persisted
    );
    assert!(store.entries.lock().unwrap().contains_key("test-user"));

    // The keyring works again: the retry reaches the entry.
    *store.fail_clear.lock().unwrap() = None;
    assert!(matches!(
        logout_answer(&daemon, Some("test-user")).await,
        Response::Ack {
            keyring: Some(KeyringOutcome::Cleared)
        }
    ));
    assert!(store.entries.lock().unwrap().is_empty());
    finish_harness(None, &log_path);
}

/// C144: a logout whose clear works is acknowledged with the keyring's
/// outcome as data (the review of the h1 fix round, its fifth finding: no
/// frontend reads it from a sentence), and the entry is gone.
#[tokio::test]
async fn a_logout_clears_the_keyring_entry_and_acks() {
    let (daemon, store, _dir, log_path) = keyring_daemon(RecordingCredentialStore::default());
    assert!(matches!(
        logout_answer(&daemon, None).await,
        Response::Ack {
            keyring: Some(KeyringOutcome::Cleared)
        }
    ));
    assert!(store.entries.lock().unwrap().is_empty());
    assert_eq!(store.cleared.lock().unwrap().as_slice(), ["test-user"]);
    finish_harness(None, &log_path);
}

/// C144, issue #61: forget clears the entry whether or not the index
/// marks it persisted — a failed save or a failed logout clear leaves an
/// entry the mark does not show — and its log says what the clear did:
/// a failed clear is the answer, never "cleared".
#[tokio::test]
async fn c144_forget_clears_an_unmarked_entry_and_reports_a_failed_clear() {
    let (daemon, store, dir, log_path) = keyring_daemon(RecordingCredentialStore::default());
    daemon.shared.lock().unwrap().auth.by_account.clear();
    Index::load(&dir)
        .unwrap()
        .set_persisted("test-user", false)
        .unwrap();

    daemon.logout(Some("test-user")).unwrap();
    assert!(
        store.entries.lock().unwrap().is_empty(),
        "an unmarked entry is cleared"
    );

    store
        .entries
        .lock()
        .unwrap()
        .insert("test-user".into(), "rt-old".into());
    *store.fail_clear.lock().unwrap() = Some("keychain locked".into());
    let refusal = daemon.logout(Some("test-user")).unwrap_err();
    assert_eq!(refusal.kind, ErrorKind::Internal);
    assert!(
        refusal
            .message
            .contains("its keyring entry could not be cleared: keychain locked"),
        "{refusal}"
    );
    let log = std::fs::read_to_string(&log_path).unwrap();
    assert_eq!(
        log.matches("forgot account test-user (keyring entry cleared)")
            .count(),
        1,
        "only the clear that worked is logged as cleared:\n{log}"
    );
    finish_harness(None, &log_path);
}

/// C144, issue #61: restore reads through the credential store, so a load
/// that fails reaches the keyring status `auth status` reports and the
/// log, and the account is not restored; an entry the store holds comes
/// back as a session; an entry it lacks is logged.
#[test]
fn c144_restore_reads_through_the_credential_store_and_reports_a_failed_load() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    {
        let mut index = Index::load(&dir).unwrap();
        index.record_login("alice", "u-a", true, 2, &[]).unwrap();
        index.record_login("bob", "u-b", true, 1, &[]).unwrap();
    }

    let store = RecordingCredentialStore::default();
    store
        .entries
        .lock()
        .unwrap()
        .insert("alice".into(), "rt-a".into());
    let mut log = Vec::new();
    let sessions = start::restore_sessions(&store, "acq-mock", &dir, None, &mut log);
    assert_eq!(sessions.keyring, "ok");
    assert_eq!(
        sessions.find("alice").unwrap().refresh_token.as_deref(),
        Some("rt-a")
    );
    assert!(sessions.find("bob").is_none());
    let log = String::from_utf8(log).unwrap();
    assert!(
        log.contains("index lists bob as persisted but the keyring has no entry"),
        "{log}"
    );

    let failing = RecordingCredentialStore::failing("load", "keychain locked");
    let mut log = Vec::new();
    let sessions = start::restore_sessions(&failing, "acq-mock", &dir, None, &mut log);
    assert!(sessions.by_account.is_empty());
    assert_eq!(sessions.keyring, "unavailable: keychain locked");
    let log = String::from_utf8(log).unwrap();
    assert!(
        log.contains("keyring read for alice failed: keychain locked"),
        "{log}"
    );
}

/// A login of `user` staged to the point its profile's uuid lands: the
/// flow begun, the tokens in the staging slot. Returns the generation.
fn staged_login(daemon: &Daemon, user: &str) -> u64 {
    let generation = daemon.begin_auth_flow();
    daemon
        .stage_auth_flow(generation, login_tokens(&format!("at-x.{user}"), user))
        .expect("current flow stages");
    generation
}

/// C144 (a failed save clears the entry it would have replaced), the
/// review of h1, its fourth finding: a re-login goes through the persist
/// a refresh uses, so a re-login whose save fails clears the previous
/// login's entry instead of leaving it behind while the index says the
/// account is not persisted.
#[tokio::test]
async fn c144_a_re_login_whose_save_fails_clears_the_previous_entry() {
    let (daemon, credentials, store_dir, _log_path) = login_daemon("http://127.0.0.1:1");
    Index::load(&store_dir)
        .unwrap()
        .record_login("Alice#1", "u-a", true, 1, &[])
        .unwrap();
    credentials
        .entries
        .lock()
        .unwrap()
        .insert("Alice#1".into(), "rt-previous".into());
    *credentials.fail_save.lock().unwrap() = Some("keychain locked".into());

    let generation = staged_login(&daemon, "Alice#1");
    assert!(daemon.complete_login(generation, "Alice#1", "u-a"));
    assert!(
        credentials.entries.lock().unwrap().is_empty(),
        "the previous login's entry is cleared"
    );
    assert_eq!(credentials.cleared.lock().unwrap().as_slice(), ["Alice#1"]);
    assert!(
        !Index::load(&store_dir)
            .unwrap()
            .get("Alice#1")
            .unwrap()
            .persisted
    );
    assert_eq!(
        daemon
            .shared
            .lock()
            .unwrap()
            .auth
            .find("Alice#1")
            .unwrap()
            .keyring,
        "unavailable: keychain locked"
    );
}

/// C144 (a rename clears the entry it leaves behind), the review of h1,
/// its fourth finding: a re-login under a renamed username moves the
/// uuid's index entry to the new name and clears the entry under the
/// name the index mapped the uuid to, which nothing could reach again.
#[tokio::test]
async fn c144_a_re_login_under_a_renamed_username_clears_the_old_names_entry() {
    let (daemon, credentials, store_dir, _log_path) = login_daemon("http://127.0.0.1:1");
    Index::load(&store_dir)
        .unwrap()
        .record_login("Old#1", "u-a", true, 1, &[])
        .unwrap();
    credentials
        .entries
        .lock()
        .unwrap()
        .insert("Old#1".into(), "rt-old".into());

    let generation = staged_login(&daemon, "New#1");
    assert!(daemon.complete_login(generation, "New#1", "u-a"));
    assert_eq!(
        credentials.entries.lock().unwrap().clone(),
        HashMap::from([("New#1".to_string(), "rt-New#1".to_string())]),
        "one entry, under the new name"
    );
    assert_eq!(credentials.cleared.lock().unwrap().as_slice(), ["Old#1"]);
    let index = Index::load(&store_dir).unwrap();
    assert!(index.get("Old#1").is_none());
    let entry = index.get("New#1").unwrap();
    assert_eq!(entry.uuid.as_deref(), Some("u-a"));
    assert!(entry.persisted);
}

/// C144 (no success the daemon did not earn), the review of h1, its
/// second finding, and the owner's ruling on the tail round: with the
/// keyring turned off (`ACQ_NO_KEYRING`) a clear tries nothing, so it is
/// never reported as one. Logged in with the keyring on, then restarted
/// with it off, a logout of the known account drops the session and
/// marks it not persisted, and its answer is an `ack` whose `keyring` is
/// `off` — a success that says nothing was cleared, where the fix round
/// answered an error and the build before it acknowledged "keyring
/// cleared" over the entry still there. Forget answers the same.
#[tokio::test]
async fn c144_a_logout_with_the_keyring_off_says_nothing_was_cleared() {
    let store = RecordingCredentialStore::default();
    *store.keyring_off.lock().unwrap() = Some("disabled by ACQ_NO_KEYRING".into());
    let (daemon, store, dir, log_path) = keyring_daemon(store);

    for answer in [
        logout_answer(&daemon, None).await,
        logout_answer(&daemon, Some("test-user")).await,
    ] {
        assert!(
            matches!(
                answer,
                Response::Ack {
                    keyring: Some(KeyringOutcome::Off)
                }
            ),
            "a clear that did not happen is said as data: {answer:?}"
        );
    }
    assert!(daemon.shared.lock().unwrap().auth.by_account.is_empty());
    assert!(
        !Index::load(&dir)
            .unwrap()
            .get("test-user")
            .unwrap()
            .persisted
    );
    assert!(
        store.entries.lock().unwrap().contains_key("test-user"),
        "the entry a run with the keyring on saved is still there"
    );
    finish_harness(None, &log_path);
}

/// C144 (every result reaches the words), the review of h1, its fifth
/// finding: forget says "not restored at start" only when its index
/// write marked the account not persisted. With the index directory
/// unwritable and the keyring clear failing, both restoration inputs
/// stand, and the answer says so, where it used to claim the account
/// would not be restored. Logout's index failure reaches its answer the
/// same way: a logout whose clear worked but whose mark was not written
/// is no `ack`.
#[cfg(unix)]
#[tokio::test]
async fn c144_forget_and_logout_say_what_the_index_write_did() {
    let (daemon, store, dir, log_path) = keyring_daemon(RecordingCredentialStore::failing(
        "clear",
        "keychain locked",
    ));
    daemon.shared.lock().unwrap().auth.by_account.clear();
    let frozen = freeze(&dir);
    let refusal = daemon.logout(Some("test-user")).unwrap_err();
    drop(frozen);
    assert_eq!(refusal.kind, ErrorKind::Internal);
    let message = refusal.message;
    assert!(!message.contains("not restored at start"), "{message}");
    for fact in [
        "forgot account test-user, but the account index could not mark it not persisted (",
        "so a start may still try to restore it",
        "its keyring entry could not be cleared: keychain locked",
    ] {
        assert!(message.contains(fact), "{fact:?} in {message}");
    }
    assert!(
        Index::load(&dir)
            .unwrap()
            .get("test-user")
            .unwrap()
            .persisted
    );

    // A live session whose clear works and whose mark cannot be written.
    *store.fail_clear.lock().unwrap() = None;
    logged_in_again(&daemon);
    let frozen = freeze(&dir);
    let answer = logout_answer(&daemon, None).await;
    drop(frozen);
    let Response::Error { kind, message } = &answer else {
        panic!("an unwritten mark is not an ack: {answer:?}");
    };
    assert_eq!(*kind, ErrorKind::Internal);
    assert!(
        message.contains(
            "logged out test-user (session dropped, keyring entry cleared), but the account index could not mark it not persisted ("
        ),
        "{message}"
    );
    finish_harness(None, &log_path);
}

/// C144 (one critical section for a forget), the review of the h1 fix
/// round, its third finding: a forget's live-or-known decision, its
/// clears and its not-persisted mark are one hold of the lock a login
/// persists under, so a login of the same account lands before the forget
/// or after it, never between. The login is staged to land at the
/// forget's clear as soon as the lock lets it: at once if the lock is
/// free there, else once the forget has answered. Before, the forget
/// cleared outside the lock, the login landed there — its credential
/// saved, the account marked persisted, its session live — and the
/// forget's late mark then said not persisted under a live session: a
/// start would not restore the account the user had just logged in to,
/// and the forget had acknowledged a logout that did not hold.
#[tokio::test]
async fn c144_a_login_cannot_land_between_a_forgets_clear_and_its_mark() {
    let (daemon, credentials, store_dir, _log_path) = login_daemon("http://127.0.0.1:1");
    Index::load(&store_dir)
        .unwrap()
        .record_login("Alice#1", "u-a", true, 1, &[])
        .unwrap();
    credentials
        .entries
        .lock()
        .unwrap()
        .insert("Alice#1".into(), "rt-previous".into());
    let generation = staged_login(&daemon, "Alice#1");
    let deferred = Arc::new(Mutex::new(false));
    {
        let weak = Arc::downgrade(&daemon);
        let deferred = deferred.clone();
        *credentials.on_clear.lock().unwrap() = Some(Box::new(move || {
            let daemon = weak.upgrade().expect("the daemon outlives the forget");
            let free = daemon.shared.try_lock().is_ok();
            if free {
                assert!(daemon.complete_login(generation, "Alice#1", "u-a"));
            } else {
                *deferred.lock().unwrap() = true;
            }
        }));
    }

    let answer = logout_answer(&daemon, Some("Alice#1")).await;
    if *deferred.lock().unwrap() {
        assert!(daemon.complete_login(generation, "Alice#1", "u-a"));
    }
    assert!(
        !matches!(answer, Response::Error { .. }),
        "the forget held: {answer:?}"
    );
    let live = daemon.shared.lock().unwrap().auth.find("Alice#1").is_some();
    assert!(live, "the login landed");
    let entry = Index::load(&store_dir)
        .unwrap()
        .get("Alice#1")
        .cloned()
        .unwrap();
    assert!(
        entry.persisted,
        "a live session's account is restored at start: {entry:?}"
    );
    assert_eq!(
        credentials
            .entries
            .lock()
            .unwrap()
            .get("Alice#1")
            .map(String::as_str),
        Some("rt-Alice#1"),
        "the login's credential stands"
    );
}

/// `test-user`'s session back in the daemon, as `logged_in` installs it.
fn logged_in_again(daemon: &Daemon) {
    let mut s = daemon.shared.lock().unwrap();
    s.auth.replace(AuthSession {
        username: Some("test-user".into()),
        refresh_token: Some("rt-old".into()),
        keyring: "ok".into(),
        ..AuthSession::default()
    });
}
