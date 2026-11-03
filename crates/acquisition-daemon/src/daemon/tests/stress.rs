//! The N6 stress: mixed policies, refresh rotation and cancellation under
//! the limit.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::door::*;
use super::harness::*;
use crate::daemon::*;

/// An internal submit of `kind` with `params`, no account named, as
/// the stress has always submitted (a client's submit names the sole
/// session's account, which scopes the account-ruled policies' state).
fn submit(door: &Door, kind: &str, params: Value) -> JobId {
    door.daemon()
        .submit(kind.into(), params, 0, "test".into(), None)
        .unwrap()
}

/// The refresh flight's joined callers, read under `shared`.
fn refresh_waiters(daemon: &Daemon) -> usize {
    daemon
        .shared
        .lock()
        .unwrap()
        .auth
        .one()
        .refresh_flight
        .as_ref()
        .map_or(0, |flight| flight.result.receiver_count())
}

/// N6, the integration stress: mixed policies, a refresh rotation and a
/// cancellation under the limit, against the counting mock. Through the
/// door, on [`Server::ByPath`]: the token route to the delayed token
/// transport, every other request to the mock's in-process face, both
/// on the door's hold (the twin this doc once called not drawn was an
/// `Own` relay holding the token POST in front of the mock; this joins
/// the two transports by path instead). A component test (C158): it
/// seeds the session and its expiry under `shared` and reads private
/// state on the door's daemon. Its private reads, by their twins at the
/// boundary: the send
/// counts (the journal, `Dashboard.sends`), the occupancy
/// (`DaemonStatus.in_flight`), the policies (`Dashboard.policies`), the
/// probes (`List`) and the refresh token (the credential store's save,
/// read here too) have exact ones; the access token (the mock records no
/// bearer) and the refresh flight have none.
#[test]
fn n6_integration_stress_mixes_policies_refresh_rotation_and_cancellation_under_the_limit() {
    let clock = Arc::new(ParkingClock::new());
    let credential_store = Arc::new(RecordingCredentialStore::default());
    let saved = credential_store.clone();
    Door::open(
        Server::ByPath(Token::Delayed(
            "200 OK",
            token_body("at-new", "rt-rotated", "test-user"),
        )),
        clock.clone(),
        credential_store,
    )
    .run(|mut door| async move {
        let credential_store = saved;
        let daemon = door.daemon().clone();
        let (token_arrived, release_token) = door.held();
        {
            let mut shared = daemon.shared.lock().unwrap();
            shared.auth.one_mut().refresh_token = Some("rt-old".into());
            shared.auth.rename("", "old-user");
            shared.auth.one_mut().access_token = Some("at-established".into());
            shared.auth.one_mut().access_expires_at =
                Some(clock.wall() + Duration::from_secs(3600));
        }

        // Establish every authenticated route before expiry so the refresh
        // phase is not serialized behind the one-at-a-time probe key.
        // No refresh is due yet, so a request the door holds while they
        // run is the token transport's, released only below: their ends
        // are awaited behind the scenario's own holds (C160 as amended).
        let established_routes = [
            submit(&door, "characters", json!({})),
            submit(&door, "stashes", json!({ "league": "Standard" })),
            submit(
                &door,
                "stash",
                json!({ "league": "Standard", "id": "cur1" }),
            ),
        ];
        for id in established_routes {
            let (info, outcome) = door.await_terminal_behind_own_holds(id).await;
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
        // can enter valid_access_token while the token body is held.
        let characters = submit(&door, "characters", json!({}));
        let stashes = submit(&door, "stashes", json!({ "league": "Standard" }));
        let stash = submit(
            &door,
            "stash",
            json!({ "league": "Standard", "id": "cur1" }),
        );
        let mut fetches = Vec::new();
        for sequence in 0..7 {
            fetches.push(submit(&door, "fetch", json!({ "sequence": sequence })));
        }
        let cancelled = *fetches.last().unwrap();
        daemon.cancel(cancelled).unwrap();

        // The refresh held on the wire, the runtime quiet under it: every
        // API caller has answered or parked on what it waits for, the
        // refresh flight's result among them (C160 as amended: the arrival
        // fails where the held token is all that is pending).
        door.await_behind_own_holds("awaiting the refresh's arrival", token_arrived)
            .await
            .expect("the token transport announces the refresh");
        door.quiet().await;
        let waiters = refresh_waiters(&daemon);
        assert!(
            waiters >= 2,
            "concurrent API callers joined the refresh flight: {waiters}"
        );
        release_token.send(()).unwrap();

        for id in [characters, stashes, stash] {
            let (info, outcome) = door.await_terminal(id).await;
            assert_eq!(
                info.state,
                JobState::Done,
                "shared-refresh API caller {id}: {outcome:?}"
            );
        }
        let Fixture::ByPath { ends, .. } = door.fixture() else {
            unreachable!("the door faces the mock and the token transport")
        };
        assert_whole(ends);

        // The hook moves the clock across the 5-per-10 s window the first
        // five fetches fill, which parks the sixth until a slot frees.
        let mut done = 0;
        let mut failed = 0;
        let mut cancelled_count = 0;
        for id in fetches {
            let (info, outcome) = door.await_terminal(id).await;
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
        let sends = door.journal();
        let fetch_waits: Vec<u64> = sends
            .iter()
            .filter(|s| s["route"] == "fetch" && s["method"] == "GET")
            .map(|s| s["wait_ms"].as_u64().unwrap())
            .collect();
        assert!(
            fetch_waits.len() == 6
                && fetch_waits[..5].iter().all(|&w| w == 0)
                && fetch_waits[5] > 0,
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

        door.finish().await;
    });
}
