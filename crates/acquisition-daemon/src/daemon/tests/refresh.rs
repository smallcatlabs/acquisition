//! The refresh grant: a rejected grant stops further refreshes until
//! logout, a transient failure marks nothing.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::*;

/// The login arm is `a_completed_login_clears_the_dead_grant_mark`.
#[tokio::test]
async fn rejected_refresh_grant_disables_further_refreshes_until_logout() {
    let (base, requests, server) = token_server_answering(vec!["400 Bad Request"]).await;
    let clock = Arc::new(ManualClock::new());
    // Rails off: this is product behavior, not a ladder rail (L0-R13).
    let (mut daemon, log_path) =
        test_daemon_with(Provider::mock(&base), clock, RailsConfig::default());
    let rails = daemon.choke.rails().clone();
    logged_in(&mut daemon);

    let first = daemon
        .valid_access_token(None, false, None)
        .await
        .unwrap_err();
    assert!(first.message.contains("400 Bad Request"), "{first}");
    assert_eq!(
        first.kind,
        ErrorKind::NotLoggedIn,
        "a rejected grant is a dead session"
    );
    assert_eq!(requests.lock().unwrap().len(), 1);
    assert!(
        rails
            .refresh_failed("test-user")
            .unwrap()
            .contains("HTTP 400")
    );

    let second = daemon
        .valid_access_token(None, false, None)
        .await
        .unwrap_err();
    assert!(
        second
            .message
            .contains("token refresh disabled for test-user: refresh token rejected with HTTP 400"),
        "{second}"
    );
    assert_eq!(second.kind, ErrorKind::NotLoggedIn);
    assert_eq!(
        requests.lock().unwrap().len(),
        1,
        "the dead token is not re-sent"
    );
    {
        let s = daemon.shared.lock().unwrap();
        assert_eq!(
            s.auth.one().refresh_token.as_deref(),
            Some("rt-old"),
            "nothing is deleted"
        );
        assert!(s.auth.one().refresh_flight.is_none());
    }

    daemon.logout(None).unwrap_or(());
    assert_eq!(
        rails.refresh_failed("test-user"),
        None,
        "logout clears the mark"
    );
    server.await.unwrap();
    assert_journal_matches_wire(&log_path, &wire_posts(&requests));
    finish_harness(None, &log_path);
}

#[tokio::test]
async fn transient_refresh_failures_do_not_mark_the_session() {
    let (base, requests, server) =
        token_server_answering(vec!["503 Service Unavailable", "200 OK"]).await;
    let clock = Arc::new(ManualClock::new());
    let (mut daemon, log_path) = test_daemon_with(Provider::mock(&base), clock, tripwire_config());
    let rails = daemon.choke.rails().clone();
    logged_in(&mut daemon);

    let first = daemon
        .valid_access_token(None, false, None)
        .await
        .unwrap_err();
    assert!(first.message.contains("503"), "{first}");
    assert_eq!(
        first.kind,
        ErrorKind::Upstream,
        "a 5xx is the provider, not the session"
    );
    assert_eq!(
        rails.refresh_failed("test-user"),
        None,
        "5xx is not a rejected grant"
    );
    // The tripwire did trip on the 503 (Cloudflare shape); clear it so
    // the retry can show the refresh path itself is untouched.
    assert!(rails.halted().unwrap().contains("503"));
    rails.reset_tripwire().unwrap();

    let (token, user) = daemon.valid_access_token(None, false, None).await.unwrap();
    assert_eq!((token.as_str(), user.as_str()), ("at-new", "test-user"));
    assert_eq!(requests.lock().unwrap().len(), 2);
    server.await.unwrap();
    assert_journal_matches_wire(&log_path, &wire_posts(&requests));
    finish_harness(None, &log_path);
}

/// C130: a token POST's 307 is refused upstream with the status and the
/// redirect's target shown only redacted. The body, which echoes the
/// target with its query, is not kept: the sentinel reaches neither the
/// refusal, the journal, the daemon log nor its recent errors.
#[tokio::test]
async fn a_token_3xx_is_refused_with_the_target_redacted_and_no_body() {
    const SENTINEL: &str = "c130-token-sentinel";
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let requests = Arc::new(Mutex::new(Vec::new()));
    let captured = requests.clone();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let request = mockggg::read_request(&mut stream).await.unwrap();
        assert_eq!(request.path, "/token");
        captured.lock().unwrap().push(request.body.clone());
        let target = format!("https://www.pathofexile.com/oauth/token?state={SENTINEL}");
        let headers = format!(
            "X-Rate-Limit-Policy: token-request-limit\r\nX-Rate-Limit-Rules: Ip\r\nX-Rate-Limit-Ip: 60:30:30\r\nX-Rate-Limit-Ip-State: 1:30:0\r\nLocation: {target}\r\n"
        );
        let body = format!(r#"{{"moved_to":"{target}"}}"#);
        mockggg::respond_with(
            &mut stream,
            "307 Temporary Redirect",
            "application/json",
            &headers,
            &body,
        )
        .await;
    });
    let clock = Arc::new(ManualClock::new());
    let (mut daemon, log_path) =
        test_daemon_with(Provider::mock(&base), clock, RailsConfig::default());
    logged_in(&mut daemon);

    let refusal = daemon
        .valid_access_token(None, false, None)
        .await
        .unwrap_err();
    assert_eq!(refusal.kind, ErrorKind::Upstream, "{refusal}");
    assert!(refusal.message.contains("307"), "{refusal}");
    assert!(
        !refusal.message.contains(SENTINEL),
        "the refusal kept it: {refusal}"
    );
    assert!(
        refusal
            .message
            .contains("https://www.pathofexile.com/oauth/token?…"),
        "the refusal names where the 307 pointed: {refusal}"
    );
    let journal = std::fs::read_to_string(journal_of(&log_path)).unwrap();
    assert!(
        !journal.contains(SENTINEL),
        "the journal kept it: {journal}"
    );
    let log = std::fs::read_to_string(&log_path).unwrap();
    assert!(!log.contains(SENTINEL), "the daemon log kept it: {log}");
    let errors = format!("{:?}", daemon.shared.lock().unwrap().errors);
    assert!(
        !errors.contains(SENTINEL),
        "recent errors kept it: {errors}"
    );
    assert_eq!(requests.lock().unwrap().len(), 1, "one send, no follow");
    server.await.unwrap();
    assert_journal_matches_wire(&log_path, &wire_posts(&requests));
    finish_harness(None, &log_path);
}

/// C129 over C24: Cloudflare's challenge on the token endpoint is no
/// rejected grant. The refresh sets the block mark and no dead-grant
/// mark; the refusal says Cloudflare answered and the daemon stopped,
/// and never sends the user to `acq auth`, whose code exchange would go
/// into the same block. The next ask sends nothing: the halt refuses it.
#[tokio::test]
async fn a_cloudflare_block_on_the_token_route_is_no_rejected_grant() {
    let challenge = ScriptedResponse {
        method: "POST",
        path: "/token",
        ..ScriptedResponse::cloudflare_403_challenge()
    };
    let (base, requests, server) = scripted_server(vec![challenge]).await;
    let clock = Arc::new(ManualClock::new());
    let (mut daemon, log_path) =
        test_daemon_with(Provider::mock(&base), clock, RailsConfig::default());
    let rails = daemon.choke.rails().clone();
    logged_in(&mut daemon);

    let refusal = daemon
        .valid_access_token(None, false, None)
        .await
        .unwrap_err();
    assert_eq!(
        refusal.kind,
        ErrorKind::Upstream,
        "a block is not a dead session: {refusal}"
    );
    assert!(
        refusal
            .message
            .contains("Cloudflare answered in GGG's place"),
        "{refusal}"
    );
    assert!(refusal.message.contains("reset-tripwire"), "{refusal}");
    assert!(!refusal.message.contains("acq auth"), "{refusal}");
    assert!(
        !refusal.message.contains("Just a moment"),
        "Cloudflare's page is no message of GGG's: {refusal}"
    );
    assert_eq!(
        rails.refresh_failed("test-user"),
        None,
        "a block is no rejected grant"
    );
    let halt = rails.halted().expect("the block mark is set");
    assert!(
        halt.contains("Cloudflare answered in GGG's place: 403 on POST oauth-token"),
        "{halt}"
    );
    assert_eq!(
        daemon
            .shared
            .lock()
            .unwrap()
            .auth
            .one()
            .refresh_token
            .as_deref(),
        Some("rt-old"),
        "the session stands"
    );

    let second = daemon
        .valid_access_token(None, false, None)
        .await
        .unwrap_err();
    assert_eq!(second.kind, ErrorKind::Upstream, "{second}");
    assert!(!second.message.contains("acq auth"), "{second}");
    server.await.unwrap();
    assert_eq!(requests.lock().unwrap().as_slice(), ["POST"]);
    assert_journal_matches_wire(&log_path, &requests.lock().unwrap());
    finish_harness(None, &log_path);
}

/// A token answer in the scripted server's shape: `POST /token`, the
/// given status and rate headers, and the rotated grant (or `body`).
fn token_answer(status: &'static str, headers: &str, body: Option<&str>) -> ScriptedResponse {
    ScriptedResponse {
        method: "POST",
        path: "/token",
        status,
        headers: headers.into(),
        body: body.map(str::to_string).unwrap_or_else(|| {
            json!({
                "access_token": "at-new",
                "refresh_token": "rt-rotated",
                "expires_in": 3600,
                "username": "test-user",
            })
            .to_string()
        }),
    }
}

const TOKEN_HEADERS: &str = "X-Rate-Limit-Policy: token-request-limit\r\nX-Rate-Limit-Rules: Ip\r\nX-Rate-Limit-Ip: 60:30:30\r\nX-Rate-Limit-Ip-State: 1:30:0\r\n";

/// A logged-in daemon facing `answers`, on the manual clock, its
/// credential store a recorder the test keeps.
async fn token_daemon(
    answers: Vec<ScriptedResponse>,
) -> (
    Arc<Daemon>,
    Arc<RecordingCredentialStore>,
    Arc<ManualClock>,
    PathBuf,
    Arc<Mutex<Vec<String>>>,
    tokio::task::JoinHandle<()>,
) {
    let (base, requests, server) = scripted_server(answers).await;
    let clock = Arc::new(ManualClock::new());
    let (mut daemon, log_path) =
        test_daemon_with(Provider::mock(&base), clock.clone(), RailsConfig::default());
    logged_in(&mut daemon);
    let credentials = Arc::new(RecordingCredentialStore::default());
    Arc::get_mut(&mut daemon).unwrap().credential_store = credentials.clone();
    (daemon, credentials, clock, log_path, requests, server)
}

fn session_refresh_token(daemon: &Daemon) -> Option<String> {
    daemon
        .shared
        .lock()
        .unwrap()
        .auth
        .one()
        .refresh_token
        .clone()
}

/// C131: a token 200 that carries its tokens but no rate-limit header at
/// all installs the rotated tokens, and the refresh succeeds on them;
/// the send stays a journaled protocol failure, and the unestablished
/// token route is held one timing bucket, so the next grant leaves only
/// after 60 s.
#[tokio::test]
async fn a_headerless_token_2xx_installs_its_tokens_and_holds_the_route() {
    let (daemon, credentials, clock, log_path, requests, server) = token_daemon(vec![
        token_answer("200 OK", "", None),
        token_answer("200 OK", TOKEN_HEADERS, None),
    ])
    .await;

    let (token, user) = daemon.valid_access_token(None, false, None).await.unwrap();
    assert_eq!((token.as_str(), user.as_str()), ("at-new", "test-user"));
    assert_eq!(
        session_refresh_token(&daemon).as_deref(),
        Some("rt-rotated"),
        "the rotated refresh token is the session's"
    );
    assert_eq!(
        credentials.saves.lock().unwrap().as_slice(),
        [(
            daemon.provider.keyring_service.to_string(),
            "rt-rotated".to_string(),
            "test-user".to_string()
        )],
        "the rotated refresh token is saved"
    );
    let first = journal_sends(&log_path);
    assert_eq!(first.len(), 1);
    assert_eq!(first[0]["ok"], false, "{}", first[0]);
    assert!(
        first[0]["error"]
            .as_str()
            .is_some_and(|error| error.contains("x-rate-limit-policy")),
        "the journal names the protocol failure: {}",
        first[0]
    );
    assert_eq!(requests.lock().unwrap().len(), 1, "one POST so far");

    // The access token expires on the wall clock alone, so the hold the
    // first answer installed is still ahead on the monotonic one.
    clock.laptop_sleep(Duration::from_secs(3600));
    let (token, _) = daemon.valid_access_token(None, false, None).await.unwrap();
    assert_eq!(token, "at-new");
    let sends = journal_sends(&log_path);
    assert_eq!(sends.len(), 2);
    assert_eq!(sends[1]["ok"], true, "{}", sends[1]);
    assert_eq!(
        sends[1]["wait_ms"], 60000,
        "the second grant waits one timing bucket: {}",
        sends[1]
    );
    server.await.unwrap();
    assert_journal_matches_wire(&log_path, &requests.lock().unwrap());
    finish_harness(None, &log_path);
}

/// C131's limit: a 200 that is no token response (a portal page) installs
/// nothing; the refresh fails upstream with the session standing.
#[tokio::test]
async fn a_token_2xx_that_is_no_token_response_installs_nothing() {
    let (daemon, credentials, _clock, log_path, requests, server) =
        token_daemon(vec![token_answer(
            "200 OK",
            "",
            Some("<!DOCTYPE html><html><body>Path of Exile</body></html>"),
        )])
        .await;

    let refusal = daemon
        .valid_access_token(None, false, None)
        .await
        .unwrap_err();
    assert_eq!(refusal.kind, ErrorKind::Upstream, "{refusal}");
    assert_eq!(
        session_refresh_token(&daemon).as_deref(),
        Some("rt-old"),
        "the session stands"
    );
    assert!(credentials.saves.lock().unwrap().is_empty());
    assert_eq!(daemon.choke.rails().refresh_failed("test-user"), None);
    let sends = journal_sends(&log_path);
    assert_eq!(sends.len(), 1);
    assert_eq!(sends[0]["ok"], false, "{}", sends[0]);
    server.await.unwrap();
    assert_journal_matches_wire(&log_path, &requests.lock().unwrap());
    finish_harness(None, &log_path);
}

/// C131's hold is for an answer with no rate-limit header at all: a token
/// 200 whose header set is present but malformed installs its tokens and
/// holds nothing, so the next grant goes out at once.
#[tokio::test]
async fn a_token_2xx_with_malformed_rate_headers_installs_its_tokens_without_a_hold() {
    let (daemon, credentials, clock, log_path, requests, server) = token_daemon(vec![
        token_answer(
            "200 OK",
            "X-Rate-Limit-Policy: token-request-limit\r\n",
            None,
        ),
        token_answer("200 OK", TOKEN_HEADERS, None),
    ])
    .await;

    let (token, _) = daemon.valid_access_token(None, false, None).await.unwrap();
    assert_eq!(token, "at-new");
    assert_eq!(
        session_refresh_token(&daemon).as_deref(),
        Some("rt-rotated")
    );
    assert_eq!(credentials.saves.lock().unwrap().len(), 1);

    clock.laptop_sleep(Duration::from_secs(3600));
    daemon.valid_access_token(None, false, None).await.unwrap();
    let sends = journal_sends(&log_path);
    assert_eq!(sends.len(), 2);
    assert_eq!(sends[0]["ok"], false, "{}", sends[0]);
    assert_eq!(sends[1]["wait_ms"], 0, "no hold: {}", sends[1]);
    server.await.unwrap();
    assert_journal_matches_wire(&log_path, &requests.lock().unwrap());
    finish_harness(None, &log_path);
}

/// C47 with C134: an `expires_in` no clock can add is GGG's input, not a
/// broken invariant. The refresh fails upstream before anything is
/// installed or saved, the session stands, and `shared` is not poisoned:
/// the next grant installs as usual. Before, the sum panicked under
/// `shared`, and the refresh owner's drop met the poisoned lock and
/// aborted the process with the rotated token already in the keyring.
#[tokio::test]
async fn an_out_of_range_expires_in_is_a_token_error() {
    let huge = json!({
        "access_token": "at-huge",
        "refresh_token": "rt-huge",
        "expires_in": u64::MAX,
        "username": "test-user",
    })
    .to_string();
    let (daemon, credentials, _clock, log_path, requests, server) = token_daemon(vec![
        token_answer("200 OK", TOKEN_HEADERS, Some(&huge)),
        token_answer("200 OK", TOKEN_HEADERS, None),
    ])
    .await;

    let refusal = daemon
        .valid_access_token(None, false, None)
        .await
        .unwrap_err();
    assert_eq!(refusal.kind, ErrorKind::Upstream, "{refusal}");
    assert!(refusal.message.contains("lifetime"), "{refusal}");
    assert!(!refusal.message.contains("at-huge"), "{refusal}");
    assert!(!refusal.message.contains("rt-huge"), "{refusal}");
    assert!(!daemon.shared.is_poisoned());
    assert_eq!(
        session_refresh_token(&daemon).as_deref(),
        Some("rt-old"),
        "the session stands"
    );
    assert!(
        credentials.saves.lock().unwrap().is_empty(),
        "nothing saved"
    );
    assert_eq!(daemon.choke.rails().refresh_failed("test-user"), None);

    let (token, _) = daemon.valid_access_token(None, false, None).await.unwrap();
    assert_eq!(token, "at-new", "a later grant installs");
    assert_eq!(
        session_refresh_token(&daemon).as_deref(),
        Some("rt-rotated")
    );
    server.await.unwrap();
    assert_journal_matches_wire(&log_path, &requests.lock().unwrap());
    finish_harness(None, &log_path);
}
