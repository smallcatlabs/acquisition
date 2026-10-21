//! The idle verdict: the clock, the connections, the queue and the
//! limiter's history.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::*;

#[tokio::test]
async fn a_halted_daemon_with_only_waiting_jobs_is_idle() {
    let (daemon, log) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
    let id = daemon
        .submit("fetch".into(), json!({}), 0, "t".into(), None)
        .unwrap();
    {
        let s = daemon.shared.lock().unwrap();
        assert!(
            Daemon::has_live_jobs(&s, false),
            "a waiting job keeps an unhalted daemon up"
        );
        assert!(
            !Daemon::has_live_jobs(&s, true),
            "…but not a halted one: the queue is on disk"
        );
    }
    daemon
        .shared
        .lock()
        .unwrap()
        .jobs
        .get_mut(&id)
        .unwrap()
        .info
        .state = JobState::Running;
    assert!(
        Daemon::has_live_jobs(&daemon.shared.lock().unwrap(), true),
        "a running job keeps even a halted daemon up"
    );
    remove_harness_files(&log);
}

/// C3's idle verdict, read on the daemon's clock: a fresh daemon is
/// not idle until the knob elapses; a connection, a waiting job and
/// any activity each hold it; a rails halt lets a daemon idle out over
/// its waiting queue, which stays on disk (C25).
#[test]
fn idle_verdict_follows_the_clock_the_connections_and_the_queue() {
    let clock = Arc::new(ManualClock::new());
    let (daemon, log) = test_daemon_with(
        Provider::mock("http://127.0.0.1:1"),
        clock.clone(),
        tripwire_config(),
    );
    let knob = Duration::from_secs(60);
    assert!(!daemon.is_idle(knob), "a fresh daemon's activity is now");
    clock.advance(Duration::from_secs(59));
    assert!(!daemon.is_idle(knob), "one second short of the knob");
    clock.advance(Duration::from_secs(1));
    assert!(daemon.is_idle(knob), "the knob elapsed on the clock");

    daemon.shared.lock().unwrap().connections = 1;
    assert!(!daemon.is_idle(knob), "a connection holds the daemon");
    daemon.shared.lock().unwrap().connections = 0;
    assert!(daemon.is_idle(knob));

    let id = daemon
        .submit("fetch".into(), json!({}), 0, "t".into(), None)
        .unwrap();
    assert!(!daemon.is_idle(knob), "a waiting job holds the daemon");
    daemon.cancel(id).unwrap();
    assert!(
        !daemon.is_idle(knob),
        "the submit was activity: the knob starts over"
    );
    clock.advance(knob);
    assert!(daemon.is_idle(knob), "nothing live, the knob elapsed again");

    let waiting = daemon
        .submit("fetch".into(), json!({}), 0, "t".into(), None)
        .unwrap();
    clock.advance(knob);
    assert!(
        !daemon.is_idle(knob),
        "the waiting job holds an unhalted daemon"
    );
    daemon.choke.rails().record(&crate::rails::SendReport {
        method: "GET",
        route: "fetch",
        url_path: "/fetch",
        status: Some(429),
        error: None,
        ok: false,
        counted: true,
        rate: &Value::Null,
        shape: None,
        headers: &serde_json::Value::Null,
        wait: Duration::ZERO,
    });
    assert!(daemon.rails().halted().is_some(), "the tripwire tripped");
    assert!(
        daemon.is_idle(knob),
        "a halted daemon idles out over its waiting queue"
    );
    assert_eq!(
        daemon.shared.lock().unwrap().jobs[&waiting].info.state,
        JobState::Waiting,
        "the queue is still there for the successor"
    );
    remove_harness_files(&log);
}

/// The verdict's fourth clause: a counted hit inside a policy window
/// holds an otherwise idle daemon until the window passes on the clock
/// (C3: "stays up to 300 s"), so a successor never has to assume the
/// worst about hits it cannot see.
#[tokio::test]
async fn idle_verdict_waits_for_limiter_history_to_leave_the_window() {
    let policy = "X-Rate-Limit-Policy: dispatcher-test-policy\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 100:300:60\r\nX-Rate-Limit-Account-State: 0:300:0\r\n";
    let responses = vec![
        ScriptedResponse {
            method: "HEAD",
            path: "/fetch",
            status: "204 No Content",
            headers: policy.into(),
            body: String::new(),
        },
        ScriptedResponse {
            method: "GET",
            path: "/fetch",
            status: "200 OK",
            headers: policy.replace("0:300:0", "1:300:0"),
            body: r#"{"items":["done"]}"#.into(),
        },
    ];
    let (base, requests, server) = scripted_server(responses).await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon(&base, clock.clone());
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let id = daemon
        .submit("fetch".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let (info, _) = wait_terminal(&daemon, id).await;
    assert_eq!(info.state, JobState::Done);
    server.await.unwrap();

    let knob = Duration::from_secs(60);
    clock.advance(knob);
    assert!(
        daemon.choke.is_live() && !daemon.is_idle(knob),
        "a hit inside the 300 s window holds the daemon past the knob"
    );
    clock.advance(Duration::from_secs(300));
    assert!(
        !daemon.choke.is_live() && daemon.is_idle(knob),
        "the window passed on the clock: nothing left to keep"
    );
    finish_harness_wire(dispatcher, &log_path, &requests);
}
