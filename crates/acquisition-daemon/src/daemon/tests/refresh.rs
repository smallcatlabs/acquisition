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
    remove_harness_files(&log_path);
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
    remove_harness_files(&log_path);
}
