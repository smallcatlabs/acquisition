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

    let first = daemon.valid_access_token(None, false).await.unwrap_err();
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

    let second = daemon.valid_access_token(None, false).await.unwrap_err();
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

    let first = daemon.valid_access_token(None, false).await.unwrap_err();
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
    rails.reset_tripwire();

    let (token, user) = daemon.valid_access_token(None, false).await.unwrap();
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

    let refusal = daemon.valid_access_token(None, false).await.unwrap_err();
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

    let refusal = daemon.valid_access_token(None, false).await.unwrap_err();
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

    let second = daemon.valid_access_token(None, false).await.unwrap_err();
    assert_eq!(second.kind, ErrorKind::Upstream, "{second}");
    assert!(!second.message.contains("acq auth"), "{second}");
    server.await.unwrap();
    assert_eq!(requests.lock().unwrap().as_slice(), ["POST"]);
    assert_journal_matches_wire(&log_path, &requests.lock().unwrap());
    finish_harness(None, &log_path);
}
