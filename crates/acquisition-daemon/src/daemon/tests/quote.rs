//! The quote: its projection per scope and the queued jobs ahead.
//!
//! Rulings: `decisions/plans.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::*;

/// C40 — `quote` is its own request: read-only, non-reserving, per scope, with observation age and named prerequisites.
/// The `quote` protocol request (tracer step 5): a read-only,
/// non-reserving projection. A learned policy's scope carries
/// per-window headroom and a forward-simulated ETA; an unlearned
/// route is unquotable and says so; probes, OAuth, 429s, and
/// non-sending jobs are named as not covered instead of silently
/// omitted. Nothing is sent, enqueued, or remembered.
#[tokio::test]
async fn quote_projects_per_scope_reserves_nothing_and_names_what_it_cannot_see() {
    let clock = Arc::new(ManualClock::new());
    let base = mockggg::start_with_clock(clock.clone()).await.unwrap();
    let (daemon, log_path) = test_daemon(&base, clock);
    // Teach the mock's 5-per-10s fetch policy through the ordinary
    // probe path; `character-list` stays unlearned.
    daemon
        .choke
        .head("fetch", &format!("{base}/fetch"), None, daemon.choke.now())
        .await
        .unwrap();
    let sends_before = daemon.choke.recent_sends().len();

    let mut jobs: Vec<QuoteJob> = (0..8)
        .map(|_| QuoteJob {
            kind: "fetch".into(),
            params: json!({}),
        })
        .collect();
    jobs.push(QuoteJob {
        kind: "characters".into(),
        params: json!({}),
    });
    jobs.push(QuoteJob {
        kind: "sleep".into(),
        params: json!({ "seconds": 1.0 }),
    });
    let quote = match daemon
        .handle_request(
            Request::Quote {
                jobs,
                account: None,
            },
            &mut None,
        )
        .await
    {
        Response::Quote { quote } => quote,
        other => panic!("{other:?}"),
    };

    assert_eq!(quote.provider, "mock");
    assert!(quote.halted.is_none());
    // The work is echoed verbatim, in order — the quote's verifiable
    // basis for any carrier (a plan checks it against its actions).
    assert_eq!(quote.work.len(), 10);
    assert_eq!(quote.work[0].kind, "fetch");
    assert_eq!(quote.work[8].kind, "characters");
    assert_eq!(quote.work[9].kind, "sleep");
    let [unknown, learned] = quote.scopes.as_slice() else {
        panic!("expected two scopes: {:?}", quote.scopes);
    };
    // The learned route is keyed by its policy and quotes per-window
    // headroom; 8 sends through 5-per-10s cannot be immediate.
    assert_eq!(learned.key, "mock-fetch-request-limit");
    assert_eq!(learned.policy.as_deref(), Some("mock-fetch-request-limit"));
    assert_eq!(learned.endpoints, ["fetch"]);
    assert_eq!((learned.requests, learned.queued_ahead), (8, 0));
    let windows: Vec<(u32, u32)> = learned
        .rules
        .iter()
        .flat_map(|r| r.windows.iter().map(|w| (w.hits, w.max_hits)))
        .collect();
    assert!(
        windows.contains(&(0, 5)),
        "per-window headroom: {windows:?}"
    );
    assert!(
        learned.eta_seconds.is_some_and(|eta| eta >= 10),
        "{:?}",
        learned.eta_seconds
    );
    // The rules carry their own observation basis — they are as old
    // as the probe, not as fresh as the quote.
    assert!(learned.observed_seconds_ago.is_some());
    assert!(learned.notes.is_empty(), "{:?}", learned.notes);
    // The unlearned route is unquotable and says so, never guessed.
    assert_eq!(unknown.key, "character-list");
    assert_eq!(unknown.policy, None);
    assert_eq!(unknown.eta_seconds, None);
    assert_eq!(unknown.observed_seconds_ago, None);
    assert!(
        unknown.notes[0].contains("HEAD probe"),
        "{:?}",
        unknown.notes
    );
    // What the quote does not cover is named.
    let nc = quote.not_covered.join("\n");
    assert!(nc.contains("character-list"), "{nc}");
    assert!(nc.contains("OAuth token refresh"), "{nc}");
    assert!(nc.contains("429 re-sends"), "{nc}");
    assert!(nc.contains("`sleep` job(s) send nothing"), "{nc}");
    // Read-only and non-reserving: no job exists, nothing was sent.
    assert!(daemon.shared.lock().unwrap().jobs.is_empty());
    assert_eq!(daemon.choke.recent_sends().len(), sends_before);
    remove_harness_files(&log_path);
}

/// C40 — the projection counts the queue ahead and keys the same scopes a submit would.
#[tokio::test]
async fn quote_counts_queued_jobs_ahead_and_keys_scopes_per_account() {
    let clock = Arc::new(ManualClock::new());
    let base = mockggg::start_with_clock(clock.clone()).await.unwrap();
    let (daemon, log_path) = test_daemon(&base, clock);
    daemon.shared.lock().unwrap().auth.rename("", "Alice#1234");
    daemon
        .choke
        .head("fetch", &format!("{base}/fetch"), None, daemon.choke.now())
        .await
        .unwrap();
    // Three fetch jobs sit waiting (no dispatcher runs here).
    for _ in 0..3 {
        daemon
            .submit("fetch".into(), json!({}), 0, "test".into(), None)
            .unwrap();
    }
    // A refresh parent that already made its listing request and now
    // holds its deferred result while waiting for children: on the
    // stash-list scope, but no longer a future send.
    let parent = daemon
        .submit(
            "refresh".into(),
            json!({ "league": "Standard" }),
            0,
            "test".into(),
            Some("Alice#1234".into()),
        )
        .unwrap();
    {
        let mut s = daemon.shared.lock().unwrap();
        let e = s.jobs.get_mut(&parent).unwrap();
        e.info.state = JobState::Running;
        e.deferred = Some(Outcome::Success { payload: json!({}) });
    }
    let quote = match daemon
        .handle_request(
            Request::Quote {
                jobs: vec![
                    QuoteJob {
                        kind: "fetch".into(),
                        params: json!({}),
                    },
                    QuoteJob {
                        kind: "stashes".into(),
                        params: json!({ "league": "Standard" }),
                    },
                ],
                account: None,
            },
            &mut None,
        )
        .await
    {
        Response::Quote { quote } => quote,
        other => panic!("{other:?}"),
    };
    // The queue competes: waiting jobs on the scope go ahead of the
    // quoted work.
    let fetch = quote
        .scopes
        .iter()
        .find(|s| s.key == "mock-fetch-request-limit")
        .unwrap();
    assert_eq!((fetch.requests, fetch.queued_ahead), (1, 3));
    // An account-holding job keys its scope per account, exactly as
    // the limiter and dispatcher would — and the deferred parent on
    // that scope is not counted ahead: its own send already happened.
    let listing = quote
        .scopes
        .iter()
        .find(|s| s.key == "stash-list@Alice#1234")
        .unwrap_or_else(|| panic!("{:?}", quote.scopes));
    assert_eq!(listing.queued_ahead, 0);
    assert_eq!(quote.account.as_deref(), Some("Alice#1234"));
    // A selector naming no live session refuses the quote whole — the
    // same rules as Submit, so a quote never keys the wrong state.
    // Even an empty job list judges its selector: a zero-action quote
    // must not launder an unknown account into `account: null`.
    match daemon
        .handle_request(
            Request::Quote {
                jobs: Vec::new(),
                account: Some("Bob".into()),
            },
            &mut None,
        )
        .await
    {
        Response::Error { message, .. } => assert!(message.contains("Bob"), "{message}"),
        other => panic!("{other:?}"),
    }
    // Omitted with several sessions live is ambiguous too — even an
    // empty quote never guesses its headline account.
    daemon.shared.lock().unwrap().auth.by_account.insert(
        "Bob#9".into(),
        AuthSession {
            username: Some("Bob#9".into()),
            ..Default::default()
        },
    );
    match daemon
        .handle_request(
            Request::Quote {
                jobs: Vec::new(),
                account: None,
            },
            &mut None,
        )
        .await
    {
        Response::Error { message, .. } => assert!(message.contains("several"), "{message}"),
        other => panic!("{other:?}"),
    }
    remove_harness_files(&log_path);
}
