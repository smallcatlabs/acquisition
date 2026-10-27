//! The N6 stress: mixed policies, refresh rotation and cancellation under
//! the limit.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::*;

async fn wait_for_refresh_waiters(daemon: &Daemon, expected_at_least: usize) {
    tokio::time::timeout(Duration::from_secs(1), async {
        loop {
            let receivers = daemon
                .shared
                .lock()
                .unwrap()
                .auth
                .one()
                .refresh_flight
                .as_ref()
                .map_or(0, |flight| flight.result.receiver_count());
            if receivers >= expected_at_least {
                return;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("concurrent API callers joined the refresh flight");
}

#[tokio::test]
async fn n6_integration_stress_mixes_policies_refresh_rotation_and_cancellation_under_the_limit() {
    let clock = Arc::new(ManualClock::new());
    let api_base = mockggg::start_with_clock(clock.clone()).await.unwrap();
    let (token_base, token_arrived, release_token, token_server) = delayed_token_server().await;
    let (mut daemon, log_path) = test_daemon(&api_base, clock.clone());
    let credential_store = Arc::new(RecordingCredentialStore::default());
    let daemon_mut = Arc::get_mut(&mut daemon).unwrap();
    daemon_mut.credential_store = credential_store.clone();
    daemon_mut.provider.token_url = format!("{token_base}/token");
    {
        let mut shared = daemon.shared.lock().unwrap();
        shared.auth.one_mut().refresh_token = Some("rt-old".into());
        shared.auth.rename("", "old-user");
        shared.auth.one_mut().access_token = Some("at-established".into());
        shared.auth.one_mut().access_expires_at = Some(clock.wall() + Duration::from_secs(3600));
    }

    let dispatcher = tokio::spawn(daemon.clone().dispatcher());

    // Establish every authenticated route before expiry so the refresh
    // phase is not serialized behind the one-at-a-time probe key.
    let established_routes = [
        daemon
            .submit("characters".into(), json!({}), 0, "test".into(), None)
            .unwrap(),
        daemon
            .submit(
                "stashes".into(),
                json!({ "league": "Standard" }),
                0,
                "test".into(),
                None,
            )
            .unwrap(),
        daemon
            .submit(
                "stash".into(),
                json!({ "league": "Standard", "id": "cur1" }),
                0,
                "test".into(),
                None,
            )
            .unwrap(),
    ];
    for id in established_routes {
        let (info, outcome) = wait_terminal(&daemon, id).await;
        assert_eq!(
            info.state,
            JobState::Done,
            "route-establishment job {id}: {outcome:?}"
        );
    }
    {
        let mut shared = daemon.shared.lock().unwrap();
        // Expired on the daemon's clock, not the machine's.
        shared.auth.one_mut().access_expires_at = Some(clock.wall());
    }

    // These jobs now have different learned scheduling keys, so all three
    // can enter valid_access_token while the localhost token body is held.
    let characters = daemon
        .submit("characters".into(), json!({}), 0, "test".into(), None)
        .unwrap();
    let stashes = daemon
        .submit(
            "stashes".into(),
            json!({ "league": "Standard" }),
            0,
            "test".into(),
            None,
        )
        .unwrap();
    let stash = daemon
        .submit(
            "stash".into(),
            json!({ "league": "Standard", "id": "cur1" }),
            0,
            "test".into(),
            None,
        )
        .unwrap();
    let fetches: Vec<_> = (0..7)
        .map(|sequence| {
            daemon
                .submit(
                    "fetch".into(),
                    json!({ "sequence": sequence }),
                    0,
                    "test".into(),
                    None,
                )
                .unwrap()
        })
        .collect();
    let cancelled = *fetches.last().unwrap();
    daemon.cancel(cancelled).unwrap();

    token_arrived.await.unwrap();
    wait_for_refresh_waiters(&daemon, 2).await;
    release_token.send(()).unwrap();

    for id in [characters, stashes, stash] {
        let (info, outcome) = wait_terminal(&daemon, id).await;
        assert_eq!(
            info.state,
            JobState::Done,
            "shared-refresh API caller {id}: {outcome:?}"
        );
    }
    token_server.await.unwrap();

    let mut done = 0;
    let mut failed = 0;
    let mut cancelled_count = 0;
    for id in fetches {
        let (info, outcome) = wait_terminal(&daemon, id).await;
        match info.state {
            JobState::Done => done += 1,
            JobState::Failed => {
                failed += 1;
                assert_eq!(info.retries, MAX_429_RETRIES);
                let Outcome::Failure { error, .. } = outcome else {
                    panic!("failed fetch {id} had non-failure outcome")
                };
                assert!(error.contains("giving up"));
            }
            JobState::Cancelled => cancelled_count += 1,
            state => panic!("fetch {id} stopped in nonterminal state {state}"),
        }
    }
    // The counting mock makes N4 non-vacuous: every line's state is the
    // mock's own count, so "never over the limit", which the mock
    // teardown runs, pins the product.
    let sends = journal_sends(&log_path);
    let fetch_waits: Vec<u64> = sends
        .iter()
        .filter(|s| s["route"] == "fetch" && s["method"] == "GET")
        .map(|s| s["wait_ms"].as_u64().unwrap())
        .collect();
    assert!(
        fetch_waits.len() == 6 && fetch_waits[..5].iter().all(|&w| w == 0) && fetch_waits[5] > 0,
        "five fetches fill the 5-per-10 s window at once and the sixth is held: {fetch_waits:?}"
    );
    // Before 2026-08-24 this read (5, 1, 1): the mock's windows expired
    // in real time while the daemon's expired in virtual time, so the
    // sixth fetch's retries always met a still-restricted mock. On one
    // clock the limiter holds before the sixth send and nothing
    // violates (N4). 429 recovery is pinned by the scripted tests.
    assert_eq!((done, failed, cancelled_count), (6, 0, 1));

    let (refresh_token, access_token, refresh_flight) = {
        let shared = daemon.shared.lock().unwrap();
        (
            shared.auth.one().refresh_token.clone().unwrap(),
            shared.auth.one().access_token.clone().unwrap(),
            shared.auth.one().refresh_flight.is_some(),
        )
    };
    assert_eq!(refresh_token, "rt-rotated");
    assert_eq!(access_token, "at-new");
    assert!(!refresh_flight);
    let saves = saved_refresh_tokens(&credential_store.saves.lock().unwrap());
    assert_eq!(saves.len(), 1, "rotated refresh token persisted once");
    assert_eq!(saves[0].0, "acq-mock");
    assert_eq!(saves[0].1, "rt-rotated");
    assert_eq!(saves[0].2, "test-user");

    let jobs = daemon.job_list().unwrap();
    let probes: Vec<_> = jobs.iter().filter(|job| job.kind == "probe").collect();
    assert_eq!(probes.len(), 4, "one probe per exercised API route");
    assert!(probes.iter().all(|job| job.state == JobState::Done));

    let sends = daemon.choke.recent_sends();
    assert_eq!(sends.iter().filter(|send| send.method == "HEAD").count(), 4);
    assert_eq!(
        sends
            .iter()
            .filter(|send| send.method == "POST" && send.endpoint == "oauth-token")
            .count(),
        1
    );
    assert_eq!(
        sends
            .iter()
            .filter(|send| send.method == "GET" && send.endpoint == "fetch")
            .count(),
        6,
        "six fetches, none retried"
    );
    assert_eq!(daemon.choke.actual_send_occupancy(), (0, 2));

    let mut policies: Vec<_> = daemon
        .choke
        .policy_statuses()
        .into_iter()
        .map(|status| status.policy)
        .collect();
    policies.sort();
    assert_eq!(
        policies,
        [
            "character-list-request-limit",
            "mock-fetch-request-limit",
            "stash-list-request-limit",
            "stash-request-limit",
            "token-request-limit",
        ]
    );

    finish_harness_mock(dispatcher, &log_path);
}
