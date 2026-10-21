//! The rails: the tripwire, the ceiling and the halted refresh.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::*;

// ---- L0 live-test rails (`rails.rs`) ------------------------------------

/// Poll a job until `pred` holds on its snapshot (3 s budget).
async fn wait_until(daemon: &Daemon, id: JobId, pred: impl Fn(&JobInfo) -> bool) -> JobInfo {
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let info = daemon.shared.lock().unwrap().jobs[&id].info.clone();
            if pred(&info) {
                return info;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("job reached the expected state")
}

#[tokio::test]
async fn tripped_daemon_leaves_queued_jobs_waiting_until_reset() {
    // HEAD establishes, the first GET lands a 429 (tripping). Nothing
    // else reaches the server until the reset; then both queued jobs
    // go out (C25).
    let responses = vec![
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse::full("GET", "429 Too Many Requests", Some(0), "{}"),
        ScriptedResponse::full("GET", "200 OK", None, r#"{"ok":true}"#),
        ScriptedResponse::full("GET", "200 OK", None, r#"{"ok":true}"#),
    ];
    let (base, requests, server) = scripted_server(responses).await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) =
        test_daemon_with(Provider::mock(&base), clock.clone(), tripwire_config());
    let rails = daemon.choke.rails().clone();
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());

    let first = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let info = wait_until(&daemon, first, |i| i.retries == 1).await;
    assert_eq!(
        info.state,
        JobState::Waiting,
        "re-queued by the 429, then held"
    );
    assert!(
        rails.halted().unwrap().contains("429 on GET /fetch"),
        "{:?}",
        rails.halted()
    );
    let second = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(
        requests.lock().unwrap().as_slice(),
        ["HEAD", "GET"],
        "a tripped daemon sends nothing"
    );
    for id in [first, second] {
        let state = daemon.shared.lock().unwrap().jobs[&id].info.state;
        assert_eq!(state, JobState::Waiting, "job {id} waits out the halt");
    }
    let logged = {
        let s = daemon.shared.lock().unwrap();
        s.errors
            .iter()
            .any(|(_, m)| m.contains("LIVE-TEST RAILS TRIPPED"))
    };
    assert!(logged, "the trip is announced once in the error ring");

    // The reset wakes the queue; the limiter's own hold from the 429
    // still applies.
    daemon.reset_rails();
    let (info, _) = wait_terminal(&daemon, first).await;
    assert_eq!(info.state, JobState::Done);
    let (info, _) = wait_terminal(&daemon, second).await;
    assert_eq!(info.state, JobState::Done);
    assert_eq!(
        requests.lock().unwrap().as_slice(),
        ["HEAD", "GET", "GET", "GET"]
    );
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
}

#[tokio::test]
async fn the_ceiling_halts_after_n_sends_and_holds_the_queue() {
    let responses = vec![
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse::full("GET", "200 OK", None, r#"{"ok":true}"#),
    ];
    let (base, requests, server) = scripted_server(responses).await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon_with(
        Provider::mock(&base),
        clock,
        RailsConfig {
            max_sends: Some(2),
            ..RailsConfig::default()
        },
    );
    let rails = daemon.choke.rails().clone();
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());

    let first = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let (info, _) = wait_terminal(&daemon, first).await;
    assert_eq!(
        info.state,
        JobState::Done,
        "HEAD + GET reach the ceiling exactly"
    );
    assert!(rails.halted().unwrap().contains("ceiling: 2 of 2"));

    let second = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    let state = daemon.shared.lock().unwrap().jobs[&second].info.state;
    assert_eq!(
        state,
        JobState::Waiting,
        "a ceiling halt holds the queue too"
    );
    assert_eq!(requests.lock().unwrap().len(), 2);
    assert_eq!(rails.status().sends, 2);
    server.await.unwrap();
    finish_harness_wire(dispatcher, &log_path, &requests);
}

#[tokio::test]
async fn halted_daemon_refuses_refresh_before_any_send() {
    let (base, requests, server) = token_server_answering(vec![]).await;
    let clock = Arc::new(ManualClock::new());
    let (mut daemon, log_path) = test_daemon_with(Provider::mock(&base), clock, tripwire_config());
    let rails = daemon.choke.rails().clone();
    rails.record(&crate::rails::SendReport {
        method: "HEAD",
        route: "character-list",
        url_path: "/character",
        status: Some(429),
        error: None,
        ok: false,
        counted: false,
        rate: &Value::Null,
        shape: None,
        headers: &serde_json::Value::Null,
        wait: Duration::ZERO,
    });
    logged_in(&mut daemon);
    let error = daemon.valid_access_token(None, false).await.unwrap_err();
    assert!(
        error.message.contains("halted by live-test rails"),
        "{error}"
    );
    assert_eq!(
        error.kind,
        ErrorKind::Upstream,
        "a halted refresh is not a dead session"
    );
    assert!(requests.lock().unwrap().is_empty());
    server.abort();
    // The one journal line is the synthetic probe recorded above, not a
    // send; nothing else may have been written.
    assert_journal_matches_wire(&log_path, &["HEAD".to_string()]);
    finish_harness(None, &log_path);
}
