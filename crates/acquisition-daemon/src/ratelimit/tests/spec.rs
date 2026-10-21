//! The limiter's spec (C33): the tables of headers and history against the
//! wait they must produce, and the choke point's sends against a local
//! server.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

// ---- the spec: test tables -------------------------------------------------
//
// Each row: a policy's headers as the server sent them, when counted
// responses arrived (seconds before "now"), and the wait the limiter must
// compute. Times are exact because the limiter's arithmetic is; the
// tolerance only absorbs float rounding.

use super::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};

/// A row of the pacing table.
struct Row {
    name: &'static str,
    claims: &'static str,
    headers: &'static [(&'static str, &'static str)],
    /// Seconds before now at which counted responses arrived, oldest
    /// first. The headers above are taken to be from the last one.
    history: &'static [f64],
    expect_wait: f64,
}

const CHAR_LIST: &[(&str, &str)] = &[
    ("x-rate-limit-policy", "character-list-request-limit"),
    ("x-rate-limit-rules", "Account"),
    ("x-rate-limit-account", "2:10:60,5:300:300"),
];

fn with_state(
    base: &[(&'static str, &'static str)],
    state: &'static str,
) -> Vec<(&'static str, &'static str)> {
    let mut v = base.to_vec();
    v.push(("x-rate-limit-account-state", state));
    v
}

fn parse(headers: &[(&str, &str)]) -> Result<Policy, PolicyParseError> {
    Policy::parse(|k| {
        headers
            .iter()
            .find(|(h, _)| *h == k)
            .map(|(_, v)| v.to_string())
    })
}

fn parse_single_window(
    limit_period: u64,
    limit_restriction: u64,
    state_period: u64,
    active_restriction: u64,
) -> Result<Policy, PolicyParseError> {
    let limit = format!("1:{limit_period}:{limit_restriction}");
    let state = format!("1:{state_period}:{active_restriction}");
    Policy::parse(|key| match key {
        "x-rate-limit-policy" => Some("bounded-policy".into()),
        "x-rate-limit-rules" => Some("Account".into()),
        "x-rate-limit-account" => Some(limit.clone()),
        "x-rate-limit-account-state" => Some(state.clone()),
        _ => None,
    })
}

fn retry(raw: Option<&str>) -> RetryAfter {
    parse_retry_after(|key| {
        (key == "retry-after")
            .then(|| raw.map(str::to_string))
            .flatten()
    })
}

async fn serve_one_raw(response: String) -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/test", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut request = [0; 4096];
        let _ = stream.read(&mut request).await.unwrap();
        stream.write_all(response.as_bytes()).await.unwrap();
    });
    (url, task)
}

fn raw_response(status: &str, rate_headers: &str, content_length: usize, body: &str) -> String {
    format!(
        "HTTP/1.1 {status}\r\n{rate_headers}Content-Type: application/json\r\nContent-Length: {content_length}\r\nConnection: close\r\n\r\n{body}"
    )
}

fn full_rate_headers(policy: &str, retry_after: Option<u64>) -> String {
    let retry_after = retry_after
        .map(|seconds| format!("Retry-After: {seconds}\r\n"))
        .unwrap_or_default();
    format!(
        "X-Rate-Limit-Policy: {policy}\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 2:10:60\r\nX-Rate-Limit-Account-State: 1:10:0\r\n{retry_after}"
    )
}

/// Replay a row: feed every history point as a counted response under
/// the row's policy, then ask for the wait at "now".
fn run(
    limiter: &mut Limiter,
    endpoint: &str,
    headers: &[(&str, &str)],
    history: &[f64],
    now: Instant,
) {
    let policy = parse(headers);
    for &ago in history {
        let at = now - Duration::from_secs_f64(ago);
        limiter
            .observe(endpoint, policy.clone(), serde_json::Value::Null, true, at)
            .expect("table rows carry Full matching policies");
    }
}

fn far_future() -> Instant {
    Instant::now() + Duration::from_secs(24 * 3600)
}

fn assert_wait(name: &str, got: Duration, expect: f64) {
    let got = got.as_secs_f64();
    assert!(
        (got - expect).abs() < 0.01,
        "{name}: expected wait {expect}s, limiter said {got}s"
    );
}

#[test]
fn n5_header_contract_parses() {
    let p = parse(&with_state(CHAR_LIST, "1:10:0,1:300:0")).expect("policy");
    assert_eq!(p.name, "character-list-request-limit");
    assert_eq!(p.rules.len(), 1);
    let r = &p.rules[0];
    assert_eq!(r.name, "Account");
    assert_eq!(
        r.limits,
        vec![
            Window {
                max_hits: 2,
                period_secs: 10,
                restriction_secs: 60
            },
            Window {
                max_hits: 5,
                period_secs: 300,
                restriction_secs: 300
            },
        ]
    );
    assert_eq!(
        r.state,
        vec![
            WindowState {
                hits: 1,
                period_secs: 10,
                restricted_secs: 0
            },
            WindowState {
                hits: 1,
                period_secs: 300,
                restricted_secs: 0
            },
        ]
    );
}

#[test]
fn retry_after_is_independent_of_full_policy_validity() {
    let mut headers = with_state(CHAR_LIST, "1:10:0,1:300:0");
    headers.push(("retry-after", "not-a-number"));
    assert!(parse(&headers).is_ok(), "Retry-After is not part of Full");
    assert_eq!(
        parse_retry_after(|key| headers
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, value)| value.to_string())),
        RetryAfter::Malformed {
            raw: "not-a-number".into()
        }
    );
}

#[test]
fn full_policy_accepts_observed_one_and_three_window_shapes() {
    let token = parse(&[
        ("x-rate-limit-policy", "token-request-limit"),
        ("x-rate-limit-rules", "Ip"),
        ("x-rate-limit-ip", "60:30:30"),
        ("x-rate-limit-ip-state", "2:30:0"),
    ])
    .unwrap();
    assert_eq!(token.rules[0].limits.len(), 1, "N33");

    let trade = parse(&[
        ("x-rate-limit-policy", "trade-policy"),
        ("x-rate-limit-rules", "Ip"),
        ("x-rate-limit-ip", "5:1:10,20:10:60,100:60:300"),
        ("x-rate-limit-ip-state", "1:1:0,1:10:0,1:60:0"),
    ])
    .unwrap();
    assert_eq!(trade.rules[0].limits.len(), 3, "N30");
}

#[test]
fn d8_full_header_grammar_rejects_every_partial_or_malformed_shape() {
    let cases: &[(&str, &[(&str, &str)])] = &[
        ("missing policy", &[]),
        ("empty policy", &[("x-rate-limit-policy", "  ")]),
        (
            "missing rules",
            &[("x-rate-limit-policy", "character-request-limit")],
        ),
        (
            "empty rules",
            &[("x-rate-limit-policy", "p"), ("x-rate-limit-rules", "")],
        ),
        (
            "empty rule name",
            &[
                ("x-rate-limit-policy", "p"),
                ("x-rate-limit-rules", "Account,"),
                ("x-rate-limit-account", "2:10:60"),
                ("x-rate-limit-account-state", "1:10:0"),
            ],
        ),
        (
            "missing rule state",
            &[
                ("x-rate-limit-policy", "p"),
                ("x-rate-limit-rules", "Account"),
                ("x-rate-limit-account", "2:10:60"),
            ],
        ),
        (
            "short triplet",
            &[
                ("x-rate-limit-policy", "p"),
                ("x-rate-limit-rules", "Account"),
                ("x-rate-limit-account", "2:10"),
                ("x-rate-limit-account-state", "1:10:0"),
            ],
        ),
        (
            "long triplet",
            &[
                ("x-rate-limit-policy", "p"),
                ("x-rate-limit-rules", "Account"),
                ("x-rate-limit-account", "2:10:60:9"),
                ("x-rate-limit-account-state", "1:10:0"),
            ],
        ),
        (
            "non-numeric triplet",
            &[
                ("x-rate-limit-policy", "p"),
                ("x-rate-limit-rules", "Account"),
                ("x-rate-limit-account", "two:10:60"),
                ("x-rate-limit-account-state", "1:10:0"),
            ],
        ),
        (
            "zero limit hits",
            &[
                ("x-rate-limit-policy", "p"),
                ("x-rate-limit-rules", "Account"),
                ("x-rate-limit-account", "0:10:60"),
                ("x-rate-limit-account-state", "0:10:0"),
            ],
        ),
        (
            "zero limit period",
            &[
                ("x-rate-limit-policy", "p"),
                ("x-rate-limit-rules", "Account"),
                ("x-rate-limit-account", "2:0:60"),
                ("x-rate-limit-account-state", "1:0:0"),
            ],
        ),
        (
            "zero state period",
            &[
                ("x-rate-limit-policy", "p"),
                ("x-rate-limit-rules", "Account"),
                ("x-rate-limit-account", "2:10:60"),
                ("x-rate-limit-account-state", "1:0:0"),
            ],
        ),
        (
            "hit count outside u32",
            &[
                ("x-rate-limit-policy", "p"),
                ("x-rate-limit-rules", "Account"),
                ("x-rate-limit-account", "4294967296:10:60"),
                ("x-rate-limit-account-state", "1:10:0"),
            ],
        ),
        (
            "window-count mismatch",
            &[
                ("x-rate-limit-policy", "p"),
                ("x-rate-limit-rules", "Account"),
                ("x-rate-limit-account", "2:10:60,5:300:300"),
                ("x-rate-limit-account-state", "1:10:0"),
            ],
        ),
        (
            "period mismatch",
            &[
                ("x-rate-limit-policy", "p"),
                ("x-rate-limit-rules", "Account"),
                ("x-rate-limit-account", "2:10:60"),
                ("x-rate-limit-account-state", "1:11:0"),
            ],
        ),
    ];

    for (name, headers) in cases {
        assert!(parse(headers).is_err(), "{name} unexpectedly parsed");
    }
}

#[test]
fn policy_deadline_fields_accept_the_operational_bounds() {
    let cases = [
        (
            "limit period",
            MAX_POLICY_PERIOD_SECS,
            0,
            MAX_POLICY_PERIOD_SECS,
            0,
        ),
        ("limit restriction", 1, MAX_POLICY_RESTRICTION_SECS, 1, 0),
        (
            "state period",
            MAX_POLICY_PERIOD_SECS,
            0,
            MAX_POLICY_PERIOD_SECS,
            0,
        ),
        ("active restriction", 1, 0, 1, MAX_POLICY_RESTRICTION_SECS),
    ];
    for (name, limit_period, limit_restriction, state_period, active_restriction) in cases {
        assert!(
            parse_single_window(
                limit_period,
                limit_restriction,
                state_period,
                active_restriction,
            )
            .is_ok(),
            "{name} maximum was rejected"
        );
    }
}

#[test]
fn policy_deadline_fields_reject_one_above_the_operational_bounds() {
    let cases = [
        (
            "limit period",
            MAX_POLICY_PERIOD_SECS + 1,
            0,
            MAX_POLICY_PERIOD_SECS,
            0,
        ),
        (
            "limit restriction",
            1,
            MAX_POLICY_RESTRICTION_SECS + 1,
            1,
            0,
        ),
        (
            "state period",
            MAX_POLICY_PERIOD_SECS,
            0,
            MAX_POLICY_PERIOD_SECS + 1,
            0,
        ),
        (
            "active restriction",
            1,
            0,
            1,
            MAX_POLICY_RESTRICTION_SECS + 1,
        ),
    ];
    for (name, limit_period, limit_restriction, state_period, active_restriction) in cases {
        assert!(
            matches!(
                parse_single_window(
                    limit_period,
                    limit_restriction,
                    state_period,
                    active_restriction,
                ),
                Err(PolicyParseError::OutOfRangeTriplet { .. })
            ),
            "{name} one-above value did not return PolicyParseError"
        );
    }
}

#[test]
fn observe_and_wait_for_are_total_at_and_above_deadline_bounds() {
    let now = Instant::now();
    let mut limiter = Limiter::new();
    limiter
        .observe(
            "/max-period",
            parse_single_window(
                MAX_POLICY_PERIOD_SECS,
                MAX_POLICY_RESTRICTION_SECS,
                MAX_POLICY_PERIOD_SECS,
                0,
            ),
            serde_json::Value::Null,
            true,
            now,
        )
        .expect("maximum period remains observable");
    assert!(limiter.wait_for("/max-period", now) > Duration::ZERO);

    limiter
        .observe(
            "/max-restriction",
            parse_single_window(
                1,
                MAX_POLICY_RESTRICTION_SECS,
                1,
                MAX_POLICY_RESTRICTION_SECS,
            ),
            serde_json::Value::Null,
            true,
            now,
        )
        .expect("maximum active restriction remains observable");
    assert!(limiter.wait_for("/max-restriction", now) > Duration::ZERO);

    let rejected = limiter.observe(
        "/rejected",
        parse_single_window(
            MAX_POLICY_PERIOD_SECS + 1,
            MAX_POLICY_RESTRICTION_SECS + 1,
            MAX_POLICY_PERIOD_SECS + 1,
            MAX_POLICY_RESTRICTION_SECS + 1,
        ),
        serde_json::Value::Null,
        true,
        now,
    );
    assert!(matches!(
        rejected,
        Err(PolicyObservationError::Parse(
            PolicyParseError::OutOfRangeTriplet { .. }
        ))
    ));
    assert_eq!(limiter.wait_for("/rejected", now), Duration::ZERO);
}

#[test]
fn pacing_table() {
    let rows: Vec<Row> = vec![
        Row {
            name: "unknown endpoint sends immediately",
            claims: "no headers seen yet — nothing to obey (HEAD-at-boot is a later step, N16)",
            headers: &[],
            history: &[],
            expect_wait: 0.0,
        },
        Row {
            name: "under both windows",
            claims: "N25 post-increment: 1 of 2 used → go",
            headers: with_state(CHAR_LIST, "1:10:0,1:300:0").leak(),
            history: &[0.0],
            expect_wait: 0.0,
        },
        Row {
            name: "initial window saturated",
            claims: "N25 lookback to the oldest counted hit (3s ago) + 10s period + 5s bucket (N12) + 1s (N13)",
            headers: with_state(CHAR_LIST, "2:10:0,2:300:0").leak(),
            history: &[3.0, 0.0],
            expect_wait: 10.0 + 5.0 + 1.0 - 3.0,
        },
        Row {
            name: "sustained window saturated",
            claims: "oldest of the last 5 (200s ago) + 300s + 60s bucket (N12, second window) + 1s",
            headers: with_state(CHAR_LIST, "1:10:0,5:300:0").leak(),
            history: &[200.0, 150.0, 100.0, 50.0, 0.0],
            expect_wait: 300.0 + 60.0 + 1.0 - 200.0,
        },
        Row {
            name: "both windows saturated → the later one wins",
            claims: "max over saturated windows",
            headers: with_state(CHAR_LIST, "2:10:0,5:300:0").leak(),
            history: &[200.0, 150.0, 100.0, 3.0, 0.0],
            expect_wait: 300.0 + 60.0 + 1.0 - 200.0,
        },
        Row {
            name: "Q4 danger case: stash-list sustained rule has a 60s period",
            claims: "positional classification pads the second window with 60s, not 5s (Q4 hypothesis, N12); the old 75s cutoff would say 66s",
            headers: &[
                ("x-rate-limit-policy", "stash-list-request-limit"),
                ("x-rate-limit-rules", "Account"),
                ("x-rate-limit-account", "10:15:60,30:60:300"),
                ("x-rate-limit-account-state", "10:15:0,30:60:0"),
            ],
            history: &[
                29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, //
                19.0, 18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, //
                9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0,
            ],
            // initial: oldest-of-10 (9s ago) + 15 + 5 + 1 = 12s; sustained: 29s ago + 60 + 60 + 1 = 92s
            expect_wait: 60.0 + 60.0 + 1.0 - 29.0,
        },
        Row {
            name: "server-side residue: saturated but history too short",
            claims: "N24 counters persist across restarts; assume unseen hits just happened → last response + period + bucket + 1s",
            headers: with_state(CHAR_LIST, "2:10:0,2:300:0").leak(),
            history: &[0.0],
            expect_wait: 10.0 + 5.0 + 1.0,
        },
        Row {
            name: "shared account: server says 2/2, our in-window history has 1",
            claims: "N23 rules are account-scoped — another tool's hit is in the count but not our history; the unknown hit is assumed recent, so the window frees from our known one: 0 + 10 + 5 + 1",
            headers: with_state(CHAR_LIST, "2:10:0,2:300:0").leak(),
            // Our older hit aged out of the 10s window long ago; the
            // positional lookback would have landed on it and said "go".
            history: &[303.0, 0.0],
            expect_wait: 10.0 + 5.0 + 1.0,
        },
        Row {
            name: "restriction active",
            claims: "restricted-for 30s from the last response + initial bucket + 1s",
            headers: with_state(CHAR_LIST, "2:10:30,2:300:0").leak(),
            history: &[5.0, 0.0],
            expect_wait: 30.0 + 5.0 + 1.0,
        },
        Row {
            name: "N26 observed burst: 15 sends at 0.2s spacing on stash-request-limit",
            claims: "N26: the long-wait send landed at history[max_hits-1] + period + bucket + buffer",
            headers: &[
                ("x-rate-limit-policy", "stash-request-limit"),
                ("x-rate-limit-rules", "Account"),
                ("x-rate-limit-account", "15:10:60,30:300:300"),
                ("x-rate-limit-account-state", "15:10:0,15:300:0"),
            ],
            history: &[
                2.8, 2.6, 2.4, 2.2, 2.0, 1.8, 1.6, 1.4, 1.2, 1.0, 0.8, 0.6, 0.4, 0.2, 0.0,
            ],
            expect_wait: 10.0 + 5.0 + 1.0 - 2.8,
        },
    ];

    for row in rows {
        let now = far_future();
        let mut limiter = Limiter::new();
        run(&mut limiter, "/ep", row.headers, row.history, now);
        let got = limiter.wait_for("/ep", now);
        assert_wait(
            &format!("{} [{}]", row.name, row.claims),
            got,
            row.expect_wait,
        );
    }
}

#[test]
fn n6_same_policy_name_shares_counters_across_endpoints() {
    let now = far_future();
    let mut l = Limiter::new();
    let p = parse(&with_state(CHAR_LIST, "2:10:0,2:300:0"));
    l.observe(
        "/character",
        p.clone(),
        serde_json::Value::Null,
        true,
        now - Duration::from_secs(4),
    )
    .unwrap();
    l.observe("/character/pc", p, serde_json::Value::Null, true, now)
        .unwrap();
    // Both endpoints see the same two-event history: oldest 4s ago.
    assert_wait(
        "shared",
        l.wait_for("/character", now),
        10.0 + 5.0 + 1.0 - 4.0,
    );
    assert_wait(
        "shared",
        l.wait_for("/character/pc", now),
        10.0 + 5.0 + 1.0 - 4.0,
    );
}

#[test]
fn n6_n7_different_policies_are_independent() {
    let now = far_future();
    let mut l = Limiter::new();
    l.observe(
        "/character",
        parse(&with_state(CHAR_LIST, "2:10:0,2:300:0")),
        serde_json::Value::Null,
        true,
        now,
    )
    .unwrap();
    let stash = parse(&[
        ("x-rate-limit-policy", "stash-list-request-limit"),
        ("x-rate-limit-rules", "Account"),
        ("x-rate-limit-account", "10:15:60,30:60:300"),
        ("x-rate-limit-account-state", "1:15:0,1:60:0"),
    ]);
    l.observe("/stash/Standard", stash, serde_json::Value::Null, true, now)
        .unwrap();
    assert!(l.wait_for("/character", now) > Duration::ZERO);
    assert_eq!(l.wait_for("/stash/Standard", now), Duration::ZERO);
}

#[test]
fn n24_uncounted_responses_do_not_enter_history() {
    let now = far_future();
    let mut l = Limiter::new();
    let p = parse(&with_state(CHAR_LIST, "0:10:0,0:300:0"));
    l.observe("/character", p, serde_json::Value::Null, false, now)
        .unwrap();
    assert_eq!(l.statuses(now)[0].history_len, 0);
    assert_eq!(l.wait_for("/character", now), Duration::ZERO);
}

#[test]
fn n23_shared_account_with_no_known_in_window_hits() {
    // Our hits are all older than the window; a HEAD at `now` (N24:
    // uncounted) reports the window saturated by someone else. Nothing
    // we know explains the count → assume it all just happened.
    let now = far_future();
    let mut l = Limiter::new();
    let p = parse(&with_state(CHAR_LIST, "2:10:0,2:300:0"));
    l.observe(
        "/character",
        p.clone(),
        serde_json::Value::Null,
        true,
        now - Duration::from_secs(30),
    )
    .unwrap();
    l.observe(
        "/character",
        p.clone(),
        serde_json::Value::Null,
        true,
        now - Duration::from_secs(25),
    )
    .unwrap();
    l.observe("/character", p, serde_json::Value::Null, false, now)
        .unwrap();
    assert_wait(
        "all unknown",
        l.wait_for("/character", now),
        10.0 + 5.0 + 1.0,
    );
}

#[test]
fn is_live_while_history_is_inside_a_window() {
    let now = far_future();
    let mut l = Limiter::new();
    assert!(!l.is_live(now));
    l.observe(
        "/character",
        parse(&with_state(CHAR_LIST, "1:10:0,1:300:0")),
        serde_json::Value::Null,
        true,
        now,
    )
    .unwrap();
    assert!(l.is_live(now));
    // Longest window is 300s: still live at +299s, not at +301s.
    assert!(l.is_live(now + Duration::from_secs(299)));
    assert!(!l.is_live(now + Duration::from_secs(301)));
}

#[test]
fn n9_new_definition_replaces_old() {
    let now = far_future();
    let mut l = Limiter::new();
    l.observe(
        "/character",
        parse(&with_state(CHAR_LIST, "2:10:0,2:300:0")),
        serde_json::Value::Null,
        true,
        now - Duration::from_secs(1),
    )
    .unwrap();
    assert!(l.wait_for("/character", now) > Duration::ZERO);
    // GGG loosens the policy: 4 per 10s now, and we've used 2.
    let loosened = parse(&[
        ("x-rate-limit-policy", "character-list-request-limit"),
        ("x-rate-limit-rules", "Account"),
        ("x-rate-limit-account", "4:10:60,5:300:300"),
        ("x-rate-limit-account-state", "2:10:0,2:300:0"),
    ]);
    l.observe("/character", loosened, serde_json::Value::Null, true, now)
        .unwrap();
    assert_eq!(l.wait_for("/character", now), Duration::ZERO);
}

#[test]
fn eta_simulates_the_pacing_rule_forward() {
    let now = far_future();
    let mut l = Limiter::new();
    // 2 per 10s, both used: this instant and 4s ago.
    run(
        &mut l,
        "/character",
        &with_state(CHAR_LIST, "2:10:0,2:300:0"),
        &[4.0, 0.0],
        now,
    );
    // Head of queue: 10 + 5 + 1 - 4 = 12s.
    assert_wait("head", l.eta_for("/character", 0, now), 12.0);
    // One ahead: the head lands at +12s. By then the hit at `now` is
    // 12s old — already outside the 10s window (the bucket padding is
    // what creates that slack) — so only the head's own hit counts and
    // the next request goes out right behind it. (First draft of this
    // row said 16s; the arithmetic, not the limiter, was wrong.)
    assert_wait("one ahead", l.eta_for("/character", 1, now), 12.0);
    // Two ahead: two hits at +12 fill the window → +12 + 10 + 5 + 1 = 28s.
    assert_wait("two ahead", l.eta_for("/character", 2, now), 28.0);
}

#[test]
fn eta_uses_policy_aware_bucket_selection() {
    let now = far_future();
    let token_headers = [
        ("x-rate-limit-policy", "token-request-limit"),
        ("x-rate-limit-rules", "Ip"),
        ("x-rate-limit-ip", "60:30:30"),
        ("x-rate-limit-ip-state", "1:30:0"),
    ];
    let generic_headers = [
        ("x-rate-limit-policy", "generic-single-window"),
        ("x-rate-limit-rules", "Ip"),
        ("x-rate-limit-ip", "60:30:30"),
        ("x-rate-limit-ip-state", "1:30:0"),
    ];

    let mut limiter = Limiter::new();
    run(&mut limiter, "oauth-token", &token_headers, &[0.0], now);
    assert_wait(
        "N33 token ETA uses the conservative 60s bucket",
        limiter.eta_for("oauth-token", 59, now),
        30.0 + 60.0 + 1.0,
    );

    let mut limiter = Limiter::new();
    run(&mut limiter, "/generic", &generic_headers, &[0.0], now);
    assert_wait(
        "non-token single-window ETA keeps the generic 5s bucket",
        limiter.eta_for("/generic", 59, now),
        30.0 + 5.0 + 1.0,
    );
}

/// Reported hits the local history never saw — a restart probe reads
/// counters this daemon didn't produce (N24), and other tools share
/// the account's windows (N23) — must pace the forward simulation
/// too, not only the head-of-queue wait. Breaker: drop the deficit
/// seeding in `eta_for` and "one ahead" reads 0.
#[test]
fn eta_counts_reported_hits_the_local_history_never_saw() {
    let now = far_future();
    let mut l = Limiter::new();
    // A probe teaches 1-of-2 used in the 10s window; history is empty.
    l.observe_probe(
        "/character",
        Ok(parse(&with_state(CHAR_LIST, "1:10:0,1:300:0")).unwrap()),
        serde_json::Value::Null,
        now,
    );
    assert_wait("head goes now", l.eta_for("/character", 0, now), 0.0);
    // The second request meets a window holding the probe-reported
    // hit plus the head's own: 10 + 5 + 1 = 16s, never 0.
    assert_wait("one ahead", l.eta_for("/character", 1, now), 16.0);
    // The deficit rides as a count, never materialized entries: a
    // reported 299-of-300 must not truncate at the 256-entry history
    // cap (pre-fix the second request read "free"), and a
    // pathological u32 count must neither loop nor stall. Single
    // 300-per-60s window, 299 used, empty history: the head goes now
    // and the second request waits 60 + 5 + 1.
    let mut l = Limiter::new();
    let big = [
        ("x-rate-limit-policy", "big-window"),
        ("x-rate-limit-rules", "Account"),
        ("x-rate-limit-account", "300:60:60"),
        ("x-rate-limit-account-state", "299:60:0"),
    ];
    l.observe_probe(
        "/big",
        Ok(parse(&big).unwrap()),
        serde_json::Value::Null,
        now,
    );
    assert_wait("head under 299/300", l.eta_for("/big", 0, now), 0.0);
    assert_wait("second under 299/300", l.eta_for("/big", 1, now), 66.0);
    let mut l = Limiter::new();
    let absurd = [
        ("x-rate-limit-policy", "absurd-window"),
        ("x-rate-limit-rules", "Account"),
        ("x-rate-limit-account", "4000000000:60:60"),
        ("x-rate-limit-account-state", "3999999999:60:0"),
    ];
    l.observe_probe(
        "/absurd",
        Ok(parse(&absurd).unwrap()),
        serde_json::Value::Null,
        now,
    );
    assert_wait(
        "head under an absurd count",
        l.eta_for("/absurd", 0, now),
        0.0,
    );
    assert_wait(
        "second under an absurd count",
        l.eta_for("/absurd", 1, now),
        66.0,
    );
}

#[test]
fn probes_teach_without_counting_and_degrade_on_n20_shapes() {
    let now = far_future();
    let mut l = Limiter::new();
    assert_eq!(l.endpoint_state("/character", now), EndpointState::Unknown);

    // A good probe: policy learned, nothing counted, state respected.
    l.observe_probe(
        "/character",
        parse(&with_state(CHAR_LIST, "2:10:0,2:300:0")).map_err(|error| error.to_string()),
        serde_json::Value::Null,
        now,
    );
    assert_eq!(
        l.endpoint_state("/character", now),
        EndpointState::Policy("character-list-request-limit".into())
    );
    assert_eq!(l.statuses(now)[0].history_len, 0);
    // Server says 2/2 from hits we never made → assume recent → 16s.
    assert_wait("residue via probe", l.wait_for("/character", now), 16.0);

    // N20 shapes: 2xx but no policy header, or (the Dec-2023 regression)
    // a policy name with no rule definitions → degraded for the cooldown.
    l.observe_probe(
        "/stash",
        parse(&[]).map_err(|error| error.to_string()),
        serde_json::Value::Null,
        now,
    );
    assert!(matches!(
        l.endpoint_state("/stash", now),
        EndpointState::Degraded { .. }
    ));
    l.observe_probe(
        "/stash/x",
        parse(&[("x-rate-limit-policy", "stash-request-limit")]).map_err(|error| error.to_string()),
        serde_json::Value::Null,
        now,
    );
    assert!(matches!(
        l.endpoint_state("/stash/x", now),
        EndpointState::Degraded { .. }
    ));
    assert_eq!(l.degraded_endpoints(now).len(), 2);
    assert_eq!(
        l.endpoint_state("/stash", now + PROBE_COOLDOWN),
        EndpointState::Unknown
    );

    // Transport/HTTP failure degrades too; login clears it.
    l.observe_probe(
        "/fetch",
        Err("HEAD returned 401".into()),
        serde_json::Value::Null,
        now,
    );
    assert!(matches!(
        l.endpoint_state("/fetch", now),
        EndpointState::Degraded { .. }
    ));
    l.forget_degraded();
    assert_eq!(l.endpoint_state("/fetch", now), EndpointState::Unknown);
    assert_eq!(
        l.endpoint_state("/character", now),
        EndpointState::Policy("character-list-request-limit".into())
    );
}

/// Non-Full headers cannot update the policy definition; the counted
/// send behind them is assumed counted, so every window's predicted
/// hits advance by exactly one.
fn assert_policy_unchanged_but_one_hit_assumed(after: &Policy, before: &Policy) {
    assert_eq!(after.name, before.name);
    assert_eq!(after.rules.len(), before.rules.len());
    for (a, b) in after.rules.iter().zip(&before.rules) {
        assert_eq!(a.name, b.name);
        assert_eq!(a.limits, b.limits, "non-Full headers cannot update policy");
        assert_eq!(a.state.len(), b.state.len());
        for (wa, wb) in a.state.iter().zip(&b.state) {
            assert_eq!(wa.hits, wb.hits + 1, "one assumed hit per window");
            assert_eq!(wa.period_secs, wb.period_secs);
            assert_eq!(wa.restricted_secs, wb.restricted_secs);
        }
    }
}

#[test]
fn malformed_observation_never_erases_a_known_policy() {
    let now = far_future();
    let mut l = Limiter::new();
    let raw = serde_json::json!({ "source": "established" });
    l.observe(
        "/character",
        parse(&with_state(CHAR_LIST, "1:10:0,1:300:0")),
        raw.clone(),
        true,
        now,
    )
    .unwrap();
    let original = l
        .policies
        .get("character-list-request-limit")
        .unwrap()
        .policy
        .clone();
    let result = l.observe("/character", parse(&[]), serde_json::Value::Null, true, now);
    assert!(matches!(result, Err(PolicyObservationError::Parse(_))));
    assert_eq!(
        l.endpoint_state("/character", now),
        EndpointState::Policy("character-list-request-limit".into())
    );
    let state = l.policies.get("character-list-request-limit").unwrap();
    assert_eq!(
        state.raw, raw,
        "non-Full headers cannot replace observation"
    );
    assert_eq!(state.history.len(), 2);
    // The definition is untouched; the prediction advances by the one
    // hit the server may have counted (2026-08-24, rung 10's 503).
    assert_policy_unchanged_but_one_hit_assumed(&state.policy, &original);

    let result = l.observe("/token", parse(&[]), serde_json::Value::Null, true, now);
    assert!(matches!(result, Err(PolicyObservationError::Parse(_))));
    assert_eq!(l.endpoint_state("/token", now), EndpointState::Unknown);
}

#[test]
fn mismatched_steady_state_observation_preserves_policy_and_topology() {
    let now = far_future();
    let mut limiter = Limiter::new();
    limiter
        .observe(
            "/character",
            parse(&with_state(CHAR_LIST, "1:10:0,1:300:0")),
            serde_json::json!({ "source": "established" }),
            true,
            now - Duration::from_secs(1),
        )
        .unwrap();
    let original = limiter
        .policies
        .get("character-list-request-limit")
        .unwrap()
        .policy
        .clone();

    let mismatch = parse(&[
        ("x-rate-limit-policy", "renamed-policy"),
        ("x-rate-limit-rules", "Account"),
        ("x-rate-limit-account", "2:10:60,5:300:300"),
        ("x-rate-limit-account-state", "2:10:0,2:300:0"),
    ]);
    assert!(matches!(
        limiter.observe(
            "/character",
            mismatch,
            serde_json::json!({ "source": "mismatch" }),
            true,
            now,
        ),
        Err(PolicyObservationError::PolicyMismatch { .. })
    ));

    assert_eq!(
        limiter.endpoint_state("/character", now),
        EndpointState::Policy("character-list-request-limit".into())
    );
    assert_eq!(limiter.policies.len(), 1);
    let state = limiter
        .policies
        .get("character-list-request-limit")
        .unwrap();
    assert_eq!(state.policy, original);
    assert_eq!(state.raw, serde_json::json!({ "source": "established" }));
    assert_eq!(
        state.history.len(),
        2,
        "the landed response was still counted"
    );
}

#[test]
fn policy_shape_changes_clear_history_but_value_changes_retain_it() {
    let now = far_future();
    let mut limiter = Limiter::new();
    limiter
        .observe(
            "/character",
            parse(&with_state(CHAR_LIST, "1:10:0,1:300:0")),
            serde_json::Value::Null,
            true,
            now - Duration::from_secs(2),
        )
        .unwrap();

    // Same ordered rule names and periods: N9's dynamic values replace
    // the definition without throwing away compatible hit history.
    limiter
        .observe(
            "/character",
            parse(&[
                ("x-rate-limit-policy", "character-list-request-limit"),
                ("x-rate-limit-rules", "Account"),
                ("x-rate-limit-account", "4:10:90,8:300:600"),
                ("x-rate-limit-account-state", "2:10:0,2:300:0"),
            ]),
            serde_json::Value::Null,
            true,
            now - Duration::from_secs(1),
        )
        .unwrap();
    assert_eq!(limiter.statuses(now)[0].history_len, 2);

    // A period change identifies a different counter shape. The just-
    // landed event was admitted under the old shape too, so none of the
    // old history is carried into the replacement (F65).
    limiter
        .observe(
            "/character",
            parse(&[
                ("x-rate-limit-policy", "character-list-request-limit"),
                ("x-rate-limit-rules", "Account"),
                ("x-rate-limit-account", "4:20:90,8:300:600"),
                ("x-rate-limit-account-state", "1:20:0,1:300:0"),
            ]),
            serde_json::Value::Null,
            true,
            now,
        )
        .unwrap();
    assert_eq!(limiter.statuses(now)[0].history_len, 0);

    let account_only = parse(&with_state(CHAR_LIST, "1:10:0,1:300:0")).unwrap();
    let account_and_ip = parse(&[
        ("x-rate-limit-policy", "character-list-request-limit"),
        ("x-rate-limit-rules", "Account,Ip"),
        ("x-rate-limit-account", "2:10:60,5:300:300"),
        ("x-rate-limit-account-state", "1:10:0,1:300:0"),
        ("x-rate-limit-ip", "4:10:60,10:300:300"),
        ("x-rate-limit-ip-state", "1:10:0,1:300:0"),
    ])
    .unwrap();
    assert!(!account_only.has_same_shape(&account_and_ip));
}

#[test]
fn retry_after_product_vectors_are_total_and_capped() {
    let cases = [
        (None, RetryAfter::Missing),
        (Some("0"), RetryAfter::Acceptable { seconds: 0 }),
        (Some(" 900 "), RetryAfter::Acceptable { seconds: 900 }),
        (Some("soon"), RetryAfter::Malformed { raw: "soon".into() }),
        (Some("-1"), RetryAfter::Negative { raw: "-1".into() }),
        (Some("901"), RetryAfter::OverCap { raw: "901".into() }),
        (
            Some("18446744073709551616"),
            RetryAfter::OverCap {
                raw: "18446744073709551616".into(),
            },
        ),
    ];
    for (raw, expected) in cases {
        assert_eq!(retry(raw), expected, "Retry-After vector {raw:?}");
    }
}

#[test]
fn retryable_429_records_before_classification_and_holds_despite_bad_policy() {
    let now = far_future();
    let mut limiter = Limiter::new();
    let raw = serde_json::json!({ "source": "established" });
    limiter
        .observe(
            "/character",
            parse(&with_state(CHAR_LIST, "1:10:0,1:300:0")),
            raw.clone(),
            true,
            now - Duration::from_secs(1),
        )
        .unwrap();
    let original = limiter
        .policies
        .get("character-list-request-limit")
        .unwrap()
        .policy
        .clone();
    let retry_after = retry(Some("2"));

    let observation = limiter.observe_landed(
        "/character",
        LandedObservation {
            policy: parse(&[]),
            retry_after: &retry_after,
            raw: serde_json::json!({ "source": "malformed-429" }),
            counted: true,
            status: 429,
            now,
        },
    );
    assert!(matches!(
        classify_response(Some(429), false, observation, &retry_after),
        ResponseClassification::RateLimited(RetryAfter::Acceptable { seconds: 2 })
    ));

    let state = limiter
        .policies
        .get("character-list-request-limit")
        .unwrap();
    assert_eq!(state.history.len(), 2, "the counted 429 is retained");
    assert_eq!(limiter.violation_count(), 1);
    assert_policy_unchanged_but_one_hit_assumed(&state.policy, &original);
    assert_eq!(
        state.raw, raw,
        "non-Full headers cannot replace observation"
    );
    assert_wait(
        "malformed-policy 429 hold",
        limiter.wait_for("/character", now),
        2.0 + 60.0 + 1.0,
    );
}

#[test]
fn retryable_429_holds_an_unknown_route_when_policy_headers_are_bad() {
    let now = far_future();
    let mut limiter = Limiter::new();
    let retry_after = retry(Some("2"));
    let observation = limiter.observe_landed(
        "oauth-token",
        LandedObservation {
            policy: parse(&[]),
            retry_after: &retry_after,
            raw: serde_json::Value::Null,
            counted: true,
            status: 429,
            now,
        },
    );

    assert!(observation.is_err());
    assert_eq!(
        limiter.endpoint_state("oauth-token", now),
        EndpointState::Unknown
    );
    assert_wait(
        "unknown-route malformed-policy 429 hold",
        limiter.wait_for("oauth-token", now),
        2.0 + 60.0 + 1.0,
    );
    assert_eq!(limiter.violation_count(), 1);
    assert!(limiter.is_live(now));
}

#[test]
fn token_policy_uses_the_frozen_conservative_single_window_bucket() {
    let now = far_future();
    let mut limiter = Limiter::new();
    limiter
        .observe(
            "oauth-token",
            parse(&[
                ("x-rate-limit-policy", "token-request-limit"),
                ("x-rate-limit-rules", "Ip"),
                ("x-rate-limit-ip", "60:30:30"),
                ("x-rate-limit-ip-state", "60:30:0"),
            ]),
            serde_json::Value::Null,
            true,
            now,
        )
        .unwrap();

    assert_wait(
        "N33 single-window token policy",
        limiter.wait_for("oauth-token", now),
        30.0 + 60.0 + 1.0,
    );
    assert_eq!(limiter.statuses(now)[0].rules[0].windows[0].bucket_secs, 60);
}

#[test]
fn unacceptable_retry_after_vectors_are_terminal_and_install_no_hold() {
    let cases = [
        retry(None),
        retry(Some("soon")),
        retry(Some("-1")),
        retry(Some("901")),
    ];
    for retry_after in cases {
        let now = far_future();
        let mut limiter = Limiter::new();
        limiter
            .observe(
                "/character",
                parse(&with_state(CHAR_LIST, "0:10:0,0:300:0")),
                serde_json::Value::Null,
                false,
                now,
            )
            .unwrap();
        let observation = limiter.observe_landed(
            "/character",
            LandedObservation {
                policy: parse(&[]),
                retry_after: &retry_after,
                raw: serde_json::Value::Null,
                counted: true,
                status: 429,
                now,
            },
        );
        let classification = classify_response(Some(429), false, observation, &retry_after);
        assert_eq!(
            classification,
            ResponseClassification::RateLimited(retry_after.clone())
        );
        assert!(!retry_after.is_acceptable());
        assert_eq!(limiter.wait_for("/character", now), Duration::ZERO);
        assert_eq!(limiter.statuses(now)[0].history_len, 1);
        assert_eq!(limiter.violation_count(), 1);
    }
}

#[test]
fn full_head_429_establishes_and_holds_without_a_get_attempt() {
    let now = far_future();
    let mut limiter = Limiter::new();
    let retry_after = retry(Some("0"));
    limiter
        .observe_probe_429(
            "/character",
            parse(&with_state(CHAR_LIST, "0:10:0,0:300:0")),
            &retry_after,
            serde_json::json!({ "retry-after": "0" }),
            now,
        )
        .unwrap();

    assert_eq!(
        limiter.endpoint_state("/character", now),
        EndpointState::Policy("character-list-request-limit".into())
    );
    assert_eq!(limiter.violation_count(), 1);
    assert_eq!(limiter.statuses(now)[0].history_len, 1);
    assert_wait(
        "Full HEAD 429",
        limiter.wait_for("/character", now),
        60.0 + 1.0,
    );
}

#[test]
fn every_unacceptable_full_head_429_is_a_setup_failure() {
    let cases = [
        retry(None),
        retry(Some("soon")),
        retry(Some("-1")),
        retry(Some("901")),
    ];
    for retry_after in cases {
        let now = far_future();
        let mut limiter = Limiter::new();
        let result = limiter.observe_probe_429(
            "/character",
            parse(&with_state(CHAR_LIST, "0:10:0,0:300:0")),
            &retry_after,
            serde_json::Value::Null,
            now,
        );
        assert!(result.is_err(), "{retry_after} unexpectedly established");
        assert!(matches!(
            limiter.endpoint_state("/character", now),
            EndpointState::Degraded { .. }
        ));
        assert!(limiter.statuses(now).is_empty());
        assert_eq!(limiter.violation_count(), 1);
    }
}

#[test]
fn non_full_head_429_is_setup_failure_even_with_acceptable_retry_after() {
    let now = far_future();
    let mut limiter = Limiter::new();
    let retry_after = retry(Some("120"));
    assert!(
        limiter
            .observe_probe_429(
                "/character",
                parse(&[]),
                &retry_after,
                serde_json::Value::Null,
                now,
            )
            .is_err()
    );
    let EndpointState::Degraded { until, .. } = limiter.endpoint_state("/character", now) else {
        panic!("non-Full HEAD 429 must degrade setup")
    };
    assert_eq!(until.duration_since(now), Duration::from_secs(120));
    assert_eq!(limiter.violation_count(), 1);
}

#[test]
fn response_classification_precedence_is_status_network_then_protocol() {
    let malformed = PolicyObservationError::Parse(PolicyParseError::MissingHeader {
        name: "x-rate-limit-policy".into(),
    });
    let retry_after = retry(Some("3"));
    assert_eq!(
        classify_response(Some(429), true, Err(malformed.clone()), &retry_after),
        ResponseClassification::RateLimited(retry_after.clone())
    );
    assert_eq!(
        classify_response(Some(500), true, Err(malformed.clone()), &retry_after),
        ResponseClassification::Http(500)
    );
    assert_eq!(
        classify_response(Some(200), true, Err(malformed.clone()), &retry_after),
        ResponseClassification::Network
    );
    assert_eq!(
        classify_response(Some(200), false, Err(malformed.clone()), &retry_after),
        ResponseClassification::Protocol(malformed)
    );
    assert_eq!(
        classify_response(Some(204), false, Ok(()), &retry_after),
        ResponseClassification::Success
    );
    assert_eq!(
        classify_response(None, true, Ok(()), &retry_after),
        ResponseClassification::Network
    );
}

#[tokio::test]
async fn full_200_with_truncated_body_is_network_after_observing_policy() {
    let route = "full-truncated";
    let raw = raw_response(
        "200 OK",
        &full_rate_headers("full-truncated-policy", None),
        100,
        "{}",
    );
    let (url, server) = serve_one_raw(raw).await;
    let choke = ChokePoint::new();

    let result = choke.get(route, &url, choke.now()).await;
    assert!(matches!(result, Err(SendError::Transport(_))));
    assert_eq!(
        choke.endpoint_state(route),
        EndpointState::Policy("full-truncated-policy".into()),
        "landed Full headers must update policy before body transfer fails"
    );
    let send = choke.recent_sends().pop().unwrap();
    assert!(!send.ok);
    assert!(send.outcome.contains("body transfer failure"));
    server.await.unwrap();
}

#[tokio::test]
async fn tripwire_halts_the_next_send_before_any_permit() {
    use crate::rails::{Rails, RailsConfig};
    let route = "tripwire-route";
    let raw = raw_response(
        "429 Too Many Requests",
        &full_rate_headers("tripwire-policy", Some(0)),
        2,
        "{}",
    );
    let (url, server) = serve_one_raw(raw).await;
    let rails = Arc::new(Rails::with_config(RailsConfig {
        tripwire: true,
        ..RailsConfig::default()
    }));
    let choke = ChokePoint::with_clock_and_rails(Arc::new(SystemClock), rails.clone());

    let landed = choke.get(route, &url, choke.now()).await.unwrap();
    assert_eq!(landed.status.as_u16(), 429);
    server.await.unwrap();
    assert!(rails.halted().unwrap().contains("429 on GET /test"));

    // No listener behind this URL: a send would fail as a transport
    // error, so a `Halted` result proves nothing was attempted.
    let unreachable = "http://127.0.0.1:9/never";
    let refused = choke
        .get(route, unreachable, choke.now())
        .await
        .err()
        .expect("halted");
    assert!(matches!(refused, SendError::Halted(_)), "{refused}");
    let head = choke
        .head(route, unreachable, None, choke.now())
        .await
        .unwrap_err();
    assert!(
        matches!(&head, ProbeError::Failed { reason, .. } if reason.contains("halted by live-test rails")),
        "{head}"
    );
    let post = choke
        .post_form("oauth-token", unreachable, &[("a", "b")], choke.now())
        .await
        .err()
        .expect("halted");
    assert!(matches!(post, SendError::Halted(_)), "{post}");
    assert_eq!(
        choke.recent_sends().len(),
        1,
        "refused sends are not recorded as sends"
    );
}

#[tokio::test]
async fn a_send_parked_in_the_gate_is_refused_after_a_trip() {
    use crate::rails::{Rails, RailsConfig, SendReport};
    // A HEAD holds the exclusive reservation while its response is
    // delayed; an ordinary GET parks in the gate behind it. The trip
    // happens while it is parked; it must come out as Halted unsent.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let (head_arrived_tx, head_arrived) = oneshot::channel();
    let (release_tx, release) = oneshot::channel();
    let hits = Arc::new(Mutex::new(Vec::new()));
    let seen = hits.clone();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut request = [0; 4096];
        let n = stream.read(&mut request).await.unwrap();
        seen.lock().unwrap().push(
            String::from_utf8_lossy(&request[..n])
                .lines()
                .next()
                .unwrap()
                .to_string(),
        );
        head_arrived_tx.send(()).unwrap();
        release.await.unwrap();
        let raw = raw_response(
            "204 No Content",
            &full_rate_headers("parked-policy", None),
            0,
            "",
        );
        stream.write_all(raw.as_bytes()).await.unwrap();
        // No second accept: a GET reaching the server would hang, and the
        // test's timeout would fail it.
    });
    let rails = Arc::new(Rails::with_config(RailsConfig {
        tripwire: true,
        ..RailsConfig::default()
    }));
    let choke = Arc::new(ChokePoint::with_clock_and_rails(
        Arc::new(SystemClock),
        rails.clone(),
    ));

    let head_choke = choke.clone();
    let head_url = format!("{base}/probe");
    let head = tokio::spawn(async move {
        head_choke
            .head("parked", &head_url, None, head_choke.now())
            .await
    });
    head_arrived.await.unwrap();

    let get_choke = choke.clone();
    let get_url = format!("{base}/data");
    let get = tokio::spawn(async move { get_choke.get("parked", &get_url, choke.now()).await });
    tokio::time::sleep(Duration::from_millis(50)).await; // let it park
    assert!(!get.is_finished(), "the GET must be parked behind the HEAD");

    rails.record(&SendReport {
        method: "GET",
        route: "elsewhere",
        url_path: "/elsewhere",
        status: Some(429),
        error: None,
        ok: false,
        counted: true,
        rate: &serde_json::Value::Null,
        headers: &serde_json::Value::Null,
        shape: None,
        wait: Duration::ZERO,
    });
    release_tx.send(()).unwrap();
    head.await.unwrap().unwrap();
    let refused = tokio::time::timeout(Duration::from_secs(3), get)
        .await
        .expect("parked GET must resolve promptly after the HEAD releases")
        .unwrap()
        .err()
        .expect("halted");
    assert!(matches!(refused, SendError::Halted(_)), "{refused}");
    assert_eq!(
        hits.lock().unwrap().len(),
        1,
        "only the HEAD reached the server"
    );
    server.await.unwrap();
}

/// A landed response with no usable policy headers is paced like a lost
/// send: the server may have counted it, and nothing says otherwise.
/// Before 2026-08-24 only the history point was recorded, so `wait_for`
/// read a window state that predated the send and let the next one go
/// immediately — a run of headerless 503s would not have been paced.
#[test]
fn headerless_counted_response_paces_as_if_counted() {
    let mut limiter = Limiter::new();
    let route = "stash";
    let now = Instant::now();
    // stash-request-limit at 14/15 in the 10 s window, as of now.
    limiter
        .observe(
            route,
            parse(&[
                ("x-rate-limit-policy", "stash-request-limit"),
                ("x-rate-limit-rules", "Account"),
                ("x-rate-limit-account", "15:10:60,30:300:300"),
                ("x-rate-limit-account-state", "14:10:0,20:300:0"),
            ]),
            serde_json::Value::Null,
            true,
            now,
        )
        .unwrap();
    assert_eq!(limiter.wait_for(route, now), Duration::ZERO);

    // The 15th send lands as a 503 with no headers at all.
    let error = limiter
        .observe(
            route,
            Policy::parse(|_| None),
            serde_json::Value::Null,
            true,
            now,
        )
        .unwrap_err();
    assert!(
        matches!(error, PolicyObservationError::Parse(_)),
        "{error:?}"
    );
    // Assumed counted: 15/15, window saturated, next send waits the
    // period + the 5 s bucket + 1 s from the only known in-window hit.
    let wait = limiter.wait_for(route, now);
    assert_eq!(wait, Duration::from_secs(10 + 5 + 1), "{wait:?}");
    // A HEAD (uncounted) with no headers changes nothing.
    let mut probe_only = Limiter::new();
    probe_only
        .observe(
            route,
            parse(&[
                ("x-rate-limit-policy", "stash-request-limit"),
                ("x-rate-limit-rules", "Account"),
                ("x-rate-limit-account", "15:10:60,30:300:300"),
                ("x-rate-limit-account-state", "14:10:0,20:300:0"),
            ]),
            serde_json::Value::Null,
            true,
            now,
        )
        .unwrap();
    let _ = probe_only.observe(
        route,
        Policy::parse(|_| None),
        serde_json::Value::Null,
        false,
        now,
    );
    assert_eq!(probe_only.wait_for(route, now), Duration::ZERO);
}

#[tokio::test]
async fn headerless_503_on_an_established_policy_paces_as_if_counted() {
    let route = "origin-503";
    let choke = ChokePoint::new();
    choke
        .limiter
        .lock()
        .unwrap()
        .observe(
            route,
            parse(&[
                ("x-rate-limit-policy", "origin-503-policy"),
                ("x-rate-limit-rules", "account"),
                ("x-rate-limit-account", "2:10:60"),
                ("x-rate-limit-account-state", "1:10:0"),
            ]),
            serde_json::Value::Null,
            false,
            choke.now(),
        )
        .unwrap();
    assert!(choke.check(route).is_ok());

    // Rung 10's shape: an HTML error page, no X-Rate-Limit-* at all.
    let body = "<html><center><h1>503 Service Temporarily Unavailable</h1></center><hr><center>openresty</center></html>";
    let (url, server) = serve_one_raw(format!(
        "HTTP/1.1 503 Service Unavailable\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    ))
    .await;
    let completed = choke.get(route, &url, choke.now()).await.expect("landed");
    assert_eq!(completed.status.as_u16(), 503);
    server.await.unwrap();
    let wait = choke.check(route).unwrap_err();
    assert!(
        wait > Duration::from_secs(5),
        "the headerless 503 is assumed counted; window must be saturated, got {wait:?}"
    );
}

#[tokio::test]
async fn transport_failure_on_an_established_policy_paces_as_if_counted() {
    let route = "lost-send";
    let choke = ChokePoint::new();
    // 2 per 10 s with one hit already reported: one more counted send
    // saturates the window.
    choke
        .limiter
        .lock()
        .unwrap()
        .observe(
            route,
            parse(&[
                ("x-rate-limit-policy", "lost-send-policy"),
                ("x-rate-limit-rules", "account"),
                ("x-rate-limit-account", "2:10:60"),
                ("x-rate-limit-account-state", "1:10:0"),
            ]),
            serde_json::Value::Null,
            false,
            choke.now(),
        )
        .unwrap();
    assert!(choke.check(route).is_ok());

    let refused = choke
        .get(route, "http://127.0.0.1:9/lost", choke.now())
        .await
        .err()
        .expect("transport failure");
    assert!(matches!(refused, SendError::Transport(_)), "{refused}");
    let wait = choke.check(route).unwrap_err();
    assert!(
        wait > Duration::from_secs(5),
        "the lost send is assumed counted; window must be saturated, got {wait:?}"
    );

    // A HEAD that fails in transport is not counted and adds nothing.
    let head_choke = ChokePoint::new();
    head_choke
        .limiter
        .lock()
        .unwrap()
        .observe(
            route,
            parse(&[
                ("x-rate-limit-policy", "lost-send-policy"),
                ("x-rate-limit-rules", "account"),
                ("x-rate-limit-account", "2:10:60"),
                ("x-rate-limit-account-state", "1:10:0"),
            ]),
            serde_json::Value::Null,
            false,
            head_choke.now(),
        )
        .unwrap();
    let _ = head_choke
        .head(route, "http://127.0.0.1:9/lost", None, choke.now())
        .await;
    assert_ne!(
        head_choke.endpoint_state(route),
        EndpointState::Policy("lost-send-policy".into()),
        "a failed HEAD degrades the route as before"
    );
}

#[tokio::test]
async fn malformed_200_with_truncated_body_is_network_and_preserves_policy() {
    let route = "malformed-truncated";
    let choke = ChokePoint::new();
    let established_raw = serde_json::json!({ "source": "established" });
    choke
        .limiter
        .lock()
        .unwrap()
        .observe(
            route,
            parse(&with_state(CHAR_LIST, "1:10:0,1:300:0")),
            established_raw.clone(),
            false,
            Instant::now(),
        )
        .unwrap();
    let (url, server) = serve_one_raw(raw_response("200 OK", "", 100, "{}")).await;

    let result = choke.get(route, &url, choke.now()).await;
    assert!(matches!(result, Err(SendError::Transport(_))));
    let status = choke.policy_statuses().pop().unwrap();
    assert_eq!(status.headers, established_raw);
    assert_eq!(
        choke.endpoint_state(route),
        EndpointState::Policy("character-list-request-limit".into())
    );
    server.await.unwrap();
}

#[tokio::test]
async fn complete_malformed_200_is_protocol() {
    let route = "malformed-complete";
    let (url, server) = serve_one_raw(raw_response("200 OK", "", 2, "{}")).await;
    let choke = ChokePoint::new();

    assert!(matches!(
        choke.get(route, &url, choke.now()).await,
        Err(SendError::Protocol(_))
    ));
    let send = choke.recent_sends().pop().unwrap();
    assert!(!send.ok);
    assert!(send.outcome.contains("protocol failure"));
    server.await.unwrap();
}

#[tokio::test]
async fn truncated_429_and_500_keep_status_and_record_transfer_evidence() {
    for (status_line, status, retry_after) in [
        ("429 Too Many Requests", 429, Some(0)),
        ("500 Internal Server Error", 500, None),
    ] {
        let route = format!("truncated-{status}");
        let raw = raw_response(
            status_line,
            &full_rate_headers(&format!("policy-{status}"), retry_after),
            100,
            "{}",
        );
        let (url, server) = serve_one_raw(raw).await;
        let choke = ChokePoint::new();

        let response = choke
            .get(&route, &url, choke.now())
            .await
            .expect("non-2xx status keeps precedence over body failure");
        assert_eq!(response.status.as_u16(), status);
        assert!(response.body.is_err());
        let send = choke.recent_sends().pop().unwrap();
        assert!(!send.ok);
        assert!(send.outcome.starts_with(status_line));
        assert!(send.outcome.contains("body transfer failure"));
        server.await.unwrap();
    }
}

#[tokio::test]
async fn common_gate_caps_gets_until_response_bodies_complete() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let (arrived_tx, mut arrived_rx) = mpsc::unbounded_channel();
    let mut release_senders = Vec::new();
    let mut release_receivers = Vec::new();
    for _ in 0..3 {
        let (sender, receiver) = oneshot::channel();
        release_senders.push(Some(sender));
        release_receivers.push(receiver);
    }
    let server = tokio::spawn(async move {
        let mut handlers = Vec::new();
        for (index, release) in release_receivers.into_iter().enumerate() {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = [0; 4096];
            let _ = stream.read(&mut request).await.unwrap();
            arrived_tx.send(index).unwrap();
            handlers.push(tokio::spawn(async move {
                let headers = full_rate_headers(&format!("body-policy-{index}"), None);
                let response = raw_response("200 OK", &headers, 2, "");
                stream.write_all(response.as_bytes()).await.unwrap();
                release.await.unwrap();
                stream.write_all(b"{}").await.unwrap();
            }));
        }
        for handler in handlers {
            handler.await.unwrap();
        }
    });

    let choke = Arc::new(ChokePoint::new());
    let sends: Vec<_> = (0..3)
        .map(|index| {
            let choke = choke.clone();
            let url = format!("{base}/{index}");
            tokio::spawn(async move {
                choke
                    .get(&format!("route-{index}"), &url, choke.now())
                    .await
            })
        })
        .collect();

    let first = arrived_rx.recv().await.unwrap();
    let second = arrived_rx.recv().await.unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(50), arrived_rx.recv())
            .await
            .is_err(),
        "a third actual send bypassed the global cap"
    );
    release_senders[first].take().unwrap().send(()).unwrap();
    let third = tokio::time::timeout(Duration::from_secs(1), arrived_rx.recv())
        .await
        .expect("a body completion releases the permit")
        .unwrap();
    for index in [second, third] {
        release_senders[index].take().unwrap().send(()).unwrap();
    }
    for send in sends {
        send.await.unwrap().unwrap();
    }
    server.await.unwrap();
}

#[tokio::test]
async fn head_waits_exclusively_for_an_incomplete_get_body() {
    let get_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let get_url = format!("http://{}/get", get_listener.local_addr().unwrap());
    let (get_arrived_tx, get_arrived) = oneshot::channel();
    let (release_get, release_get_rx) = oneshot::channel();
    let get_server = tokio::spawn(async move {
        let (mut stream, _) = get_listener.accept().await.unwrap();
        let mut request = [0; 4096];
        let _ = stream.read(&mut request).await.unwrap();
        let response = raw_response("200 OK", &full_rate_headers("ordinary-policy", None), 2, "");
        stream.write_all(response.as_bytes()).await.unwrap();
        get_arrived_tx.send(()).unwrap();
        release_get_rx.await.unwrap();
        stream.write_all(b"{}").await.unwrap();
    });

    let head_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let head_url = format!("http://{}/head", head_listener.local_addr().unwrap());
    let (head_arrived_tx, head_arrived) = oneshot::channel();
    let head_server = tokio::spawn(async move {
        let (mut stream, _) = head_listener.accept().await.unwrap();
        let request = crate::mockggg::read_request(&mut stream).await.unwrap();
        assert_eq!(request.method, "HEAD");
        head_arrived_tx.send(()).unwrap();
        let response = raw_response(
            "204 No Content",
            &full_rate_headers("head-policy", None),
            0,
            "",
        );
        stream.write_all(response.as_bytes()).await.unwrap();
    });

    let choke = Arc::new(ChokePoint::new());
    let get = {
        let choke = choke.clone();
        tokio::spawn(async move { choke.get("ordinary-route", &get_url, choke.now()).await })
    };
    get_arrived.await.unwrap();
    let head = {
        let choke = choke.clone();
        tokio::spawn(async move { choke.head("head-route", &head_url, None, choke.now()).await })
    };
    let mut head_arrived = Box::pin(head_arrived);
    assert!(
        tokio::time::timeout(Duration::from_millis(50), &mut head_arrived)
            .await
            .is_err(),
        "HEAD overlapped an ordinary response body"
    );
    release_get.send(()).unwrap();
    tokio::time::timeout(Duration::from_secs(1), &mut head_arrived)
        .await
        .expect("HEAD starts after the ordinary permit drains")
        .unwrap();

    get.await.unwrap().unwrap();
    head.await.unwrap().unwrap();
    get_server.await.unwrap();
    head_server.await.unwrap();
}

#[tokio::test]
async fn waiting_head_has_writer_preference_over_later_mixed_policy_send() {
    async fn held_get(
        policy: &'static str,
    ) -> (
        String,
        oneshot::Receiver<()>,
        oneshot::Sender<()>,
        tokio::task::JoinHandle<()>,
    ) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}/get", listener.local_addr().unwrap());
        let (arrived_tx, arrived) = oneshot::channel();
        let (release, release_rx) = oneshot::channel();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let request = crate::mockggg::read_request(&mut stream).await.unwrap();
            assert_eq!(request.method, "GET");
            let response = raw_response("200 OK", &full_rate_headers(policy, None), 2, "");
            stream.write_all(response.as_bytes()).await.unwrap();
            arrived_tx.send(()).unwrap();
            release_rx.await.unwrap();
            stream.write_all(b"{}").await.unwrap();
        });
        (url, arrived, release, server)
    }

    let (first_url, first_arrived, release_first, first_server) =
        held_get("writer-first-policy").await;
    let (second_url, second_arrived, release_second, second_server) =
        held_get("writer-second-policy").await;
    let choke = Arc::new(ChokePoint::new());
    let first = {
        let choke = choke.clone();
        tokio::spawn(async move {
            choke
                .get("writer-first-route", &first_url, choke.now())
                .await
        })
    };
    let second = {
        let choke = choke.clone();
        tokio::spawn(async move {
            choke
                .get("writer-second-route", &second_url, choke.now())
                .await
        })
    };
    first_arrived.await.unwrap();
    second_arrived.await.unwrap();

    let canceled_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let canceled_url = format!("http://{}/get", canceled_listener.local_addr().unwrap());
    let canceled = {
        let choke = choke.clone();
        tokio::spawn(async move {
            choke
                .get("writer-canceled-route", &canceled_url, choke.now())
                .await
        })
    };
    let mut canceled_accept = Box::pin(canceled_listener.accept());
    assert!(
        tokio::time::timeout(Duration::from_millis(50), &mut canceled_accept)
            .await
            .is_err(),
        "ordinary send bypassed the full gate before cancellation"
    );
    canceled.abort();
    match canceled.await {
        Err(error) => assert!(error.is_cancelled()),
        Ok(_) => panic!("canceled gate waiter completed normally"),
    }
    drop(canceled_accept);

    let head_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let head_url = format!("http://{}/head", head_listener.local_addr().unwrap());
    let (head_arrived_tx, head_arrived) = oneshot::channel();
    let (release_head, release_head_rx) = oneshot::channel();
    let head_server = tokio::spawn(async move {
        let (mut stream, _) = head_listener.accept().await.unwrap();
        let request = crate::mockggg::read_request(&mut stream).await.unwrap();
        assert_eq!(request.method, "HEAD");
        head_arrived_tx.send(()).unwrap();
        release_head_rx.await.unwrap();
        let response = raw_response(
            "204 No Content",
            &full_rate_headers("writer-head-policy", None),
            0,
            "",
        );
        stream.write_all(response.as_bytes()).await.unwrap();
    });
    let head = {
        let choke = choke.clone();
        tokio::spawn(async move {
            choke
                .head("writer-head-route", &head_url, None, choke.now())
                .await
        })
    };
    let mut head_arrived = Box::pin(head_arrived);
    assert!(
        tokio::time::timeout(Duration::from_millis(50), &mut head_arrived)
            .await
            .is_err(),
        "HEAD overlapped the two live ordinary bodies"
    );

    let (later_url, later_arrived, release_later, later_server) =
        held_get("writer-later-policy").await;
    let later = {
        let choke = choke.clone();
        tokio::spawn(async move {
            choke
                .get("writer-later-route", &later_url, choke.now())
                .await
        })
    };
    let mut later_arrived = Box::pin(later_arrived);
    assert!(
        tokio::time::timeout(Duration::from_millis(50), &mut later_arrived)
            .await
            .is_err(),
        "later ordinary send bypassed the full gate"
    );

    release_first.send(()).unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(50), &mut head_arrived)
            .await
            .is_err(),
        "exclusive HEAD started before every ordinary permit drained"
    );
    assert!(
        tokio::time::timeout(Duration::from_millis(50), &mut later_arrived)
            .await
            .is_err(),
        "later ordinary send bypassed a waiting HEAD writer"
    );

    release_second.send(()).unwrap();
    tokio::time::timeout(Duration::from_secs(1), &mut head_arrived)
        .await
        .expect("HEAD starts when the last ordinary body completes")
        .unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(50), &mut later_arrived)
            .await
            .is_err(),
        "later ordinary send overlapped the live HEAD writer"
    );

    release_head.send(()).unwrap();
    head.await.unwrap().unwrap();
    tokio::time::timeout(Duration::from_secs(1), &mut later_arrived)
        .await
        .expect("ordinary send resumes after the HEAD writer completes")
        .unwrap();
    release_later.send(()).unwrap();

    first.await.unwrap().unwrap();
    second.await.unwrap().unwrap();
    later.await.unwrap().unwrap();
    first_server.await.unwrap();
    second_server.await.unwrap();
    head_server.await.unwrap();
    later_server.await.unwrap();
}

#[test]
fn url_path_strips_host() {
    assert_eq!(
        url_path("https://api.pathofexile.com/stash/Standard"),
        "/stash/Standard"
    );
    assert_eq!(url_path("http://127.0.0.1:5555/character"), "/character");
}

// ---- scope keying (multi-account step 5) ----------------------------

fn char_list_state(state: &'static str) -> Vec<(&'static str, &'static str)> {
    with_state(CHAR_LIST, state)
}

/// The rung-11 property, in the limiter: `Account` rules are paced per
/// account. Alice fills `character-list-request-limit` (2 per 10 s);
/// Bob's first response on the same policy reports *his* count (0→1) —
/// which must not free Alice's window (the pre-keying overwrite path).
#[test]
fn account_rules_are_paced_per_account() {
    let now = far_future();
    let mut l = Limiter::new();
    let alice = endpoint_key("character-list", Some("Alice#1234"));
    let bob = endpoint_key("character-list", Some("Bob#0001"));
    run(
        &mut l,
        &alice,
        &char_list_state("2:10:0,2:300:0"),
        &[1.0, 0.5],
        now,
    );
    let alice_wait = l.wait_for(&alice, now);
    assert!(alice_wait > Duration::ZERO, "Alice's window is full");
    assert_eq!(
        l.wait_for(&bob, now),
        Duration::ZERO,
        "Bob is unknown, not held"
    );
    // Bob's probe + first send land, reporting his own counters.
    l.observe(
        &bob,
        parse(&char_list_state("0:10:0,0:300:0")),
        serde_json::Value::Null,
        false,
        now,
    )
    .unwrap();
    run(
        &mut l,
        &bob,
        &char_list_state("1:10:0,1:300:0"),
        &[0.0],
        now,
    );
    assert_eq!(
        l.wait_for(&alice, now),
        alice_wait,
        "Bob's counters did not touch Alice's state"
    );
    assert_eq!(l.wait_for(&bob, now), Duration::ZERO, "Bob has room");
    // Two states under one policy name, and the endpoint keys show which.
    let mut keys: Vec<String> = l.statuses(now).into_iter().map(|p| p.policy).collect();
    keys.sort();
    assert_eq!(
        keys,
        vec![
            "character-list-request-limit@Alice#1234".to_string(),
            "character-list-request-limit@Bob#0001".to_string()
        ]
    );
}

/// `Ip` rules are one counter for everyone behind this machine (N33's
/// token endpoint): two accounts' sends share it.
#[test]
fn ip_rules_share_state_across_accounts() {
    const TOKEN: &[(&str, &str)] = &[
        ("x-rate-limit-policy", "token-request-limit"),
        ("x-rate-limit-rules", "Ip"),
        ("x-rate-limit-ip", "2:30:30"),
        ("x-rate-limit-ip-state", "2:30:0"),
    ];
    // The daemon posts every account's token request on the bare
    // `oauth-token` key (auth.rs): an accounted key on a probe-less Ip
    // route would be Unknown — and unpaced — on its first use.
    let now = far_future();
    let mut l = Limiter::new();
    run(&mut l, "oauth-token", TOKEN, &[2.0, 1.0], now);
    assert!(
        l.wait_for("oauth-token", now) > Duration::ZERO,
        "Alice's two logins fill it"
    );
    run(&mut l, "oauth-token", TOKEN, &[0.5], now);
    assert!(
        l.wait_for("oauth-token", now) > Duration::ZERO,
        "Bob's login shares the Ip counter"
    );
    assert_eq!(
        l.statuses(now)
            .into_iter()
            .map(|p| p.policy)
            .collect::<Vec<_>>(),
        vec!["token-request-limit".to_string()],
        "one shared state, no account suffix"
    );
    // Sends that do carry accounts, on a route whose policy is
    // Ip-only: once each account's answer has named the policy there
    // is one state for both, as for a mixed policy below. (Before an
    // account's first answer its key is unknown and unpaced, which is
    // why the token request goes on the bare key.)
    let mut l = Limiter::new();
    for account in ["Alice#1234", "Bob#0001"] {
        run(
            &mut l,
            &endpoint_key("ip-route", Some(account)),
            TOKEN,
            &[1.0],
            now,
        );
    }
    assert_eq!(
        l.statuses(now).len(),
        1,
        "an Ip-only policy is not split by the key's account"
    );
}

/// A policy that mixes scopes is shared (conservative), and a send with
/// no account is always on the bare key.
#[test]
fn mixed_scope_and_accountless_sends_use_the_shared_state() {
    const MIXED: &[(&str, &str)] = &[
        ("x-rate-limit-policy", "mixed-policy"),
        ("x-rate-limit-rules", "Account,Ip"),
        ("x-rate-limit-account", "2:10:60"),
        ("x-rate-limit-account-state", "2:10:0"),
        ("x-rate-limit-ip", "50:10:60"),
        ("x-rate-limit-ip-state", "2:10:0"),
    ];
    let now = far_future();
    let mut l = Limiter::new();
    run(
        &mut l,
        &endpoint_key("mixed", Some("Alice#1234")),
        MIXED,
        &[1.0, 0.5],
        now,
    );
    run(
        &mut l,
        &endpoint_key("mixed", Some("Bob#0001")),
        MIXED,
        &[0.2],
        now,
    );
    assert_eq!(
        l.statuses(now).len(),
        1,
        "one shared state for a mixed-scope policy"
    );
    let mut l = Limiter::new();
    run(
        &mut l,
        "character-list",
        &char_list_state("2:10:0,2:300:0"),
        &[1.0],
        now,
    );
    assert_eq!(l.statuses(now)[0].policy, "character-list-request-limit");
    assert_eq!(
        split_endpoint_key("stash@Alice#1234"),
        ("stash", Some("Alice#1234"))
    );
    assert_eq!(split_endpoint_key("oauth-token"), ("oauth-token", None));
}
#[test]
fn a_declared_policyless_route_accepts_a_headerless_2xx_and_nothing_else_does() {
    let now = far_future();
    let mut l = Limiter::new();
    l.declare_policyless("profile");
    let none: &[(&str, &str)] = &[];
    let profile = endpoint_key("profile", Some("Alice#1234"));
    // Declared route, no headers at all: Policyless, unpaced, Ok.
    l.observe(&profile, parse(none), serde_json::json!({}), true, now)
        .expect("declared policyless");
    assert_eq!(l.endpoint_state(&profile, now), EndpointState::Policyless);
    assert_eq!(l.wait_for(&profile, now), Duration::ZERO);
    // The same answer on an undeclared route is still a protocol failure.
    let err = l
        .observe(
            "character-list",
            parse(none),
            serde_json::json!({}),
            true,
            now,
        )
        .unwrap_err();
    assert!(
        err.to_string().contains("missing x-rate-limit-policy"),
        "{err}"
    );
    // A partial header set on the declared route is not "no headers".
    let partial: &[(&str, &str)] = &[("x-rate-limit-rules", "Account")];
    let raw = serde_json::json!({ "x-rate-limit-rules": "Account" });
    assert!(l.observe(&profile, parse(partial), raw, true, now).is_err());
    // Headers appearing on the declared route are learned strictly.
    l.observe(
        &profile,
        parse(&char_list_state("1:10:0,1:300:0")),
        serde_json::Value::Null,
        true,
        now,
    )
    .expect("a real policy is learned");
    assert!(matches!(
        l.endpoint_state(&profile, now),
        EndpointState::Policy(_)
    ));
}
