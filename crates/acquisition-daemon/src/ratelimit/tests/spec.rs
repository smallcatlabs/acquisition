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

/// The API base of a choke point whose test sends no bearer: a port
/// nothing listens on.
const NO_BEARER_API: &str = "http://127.0.0.1:9";

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

/// The token endpoint's one-window shape (N33), as observed.
const TOKEN_SHAPE: &[(&str, &str)] = &[
    ("x-rate-limit-policy", "token-request-limit"),
    ("x-rate-limit-rules", "Ip"),
    ("x-rate-limit-ip", "60:30:30"),
    ("x-rate-limit-ip-state", "2:30:0"),
];

/// The trade site's three-window shape (N30), as observed.
const TRADE_SHAPE: &[(&str, &str)] = &[
    ("x-rate-limit-policy", "trade-policy"),
    ("x-rate-limit-rules", "Ip"),
    ("x-rate-limit-ip", "5:1:10,20:10:60,100:60:300"),
    ("x-rate-limit-ip-state", "1:1:0,1:10:0,1:60:0"),
];

#[test]
fn full_policy_accepts_observed_one_and_three_window_shapes() {
    let token = parse(TOKEN_SHAPE).unwrap();
    assert_eq!(token.rules[0].limits.len(), 1, "N33");

    let trade = parse(TRADE_SHAPE).unwrap();
    assert_eq!(trade.rules[0].limits.len(), 3, "N30");
}

/// Every partial or malformed header set D8 names, each refused whole.
const D8_REJECTED: &[(&str, &[(&str, &str)])] = &[
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

#[test]
fn d8_full_header_grammar_rejects_every_partial_or_malformed_shape() {
    for (name, headers) in D8_REJECTED {
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

/// The pacing rows: a policy's headers and history against the wait.
fn pacing_rows() -> Vec<Row> {
    vec![
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
    ]
}

#[test]
fn pacing_table() {
    for row in pacing_rows() {
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

/// `etas_for` is `eta_for` at every queue position, and forgetting
/// simulated sends older than the longest window changes no estimate:
/// held to the simulation that keeps every send, over queues long
/// enough for the windows to slide many times, under a two-window
/// policy with local history, a seeded deficit, the token policy's
/// single window and a 299-of-300 window.
#[test]
fn etas_for_every_position_match_the_simulation_that_keeps_every_send() {
    let now = far_future();
    let mut cases: Vec<(Limiter, &str)> = Vec::new();

    let mut l = Limiter::new();
    run(
        &mut l,
        "/character",
        &with_state(CHAR_LIST, "2:10:0,2:300:0"),
        &[4.0, 0.0],
        now,
    );
    cases.push((l, "/character"));

    let mut l = Limiter::new();
    l.observe_probe(
        "/character",
        Ok(parse(&with_state(CHAR_LIST, "1:10:0,4:300:0")).unwrap()),
        serde_json::Value::Null,
        now,
    );
    cases.push((l, "/character"));

    let mut l = Limiter::new();
    let token = [
        ("x-rate-limit-policy", "token-request-limit"),
        ("x-rate-limit-rules", "Ip"),
        ("x-rate-limit-ip", "60:30:30"),
        ("x-rate-limit-ip-state", "1:30:0"),
    ];
    run(&mut l, "oauth-token", &token, &[0.0], now);
    cases.push((l, "oauth-token"));

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
    cases.push((l, "/big"));

    for (l, endpoint) in &cases {
        let etas = l.etas_for(endpoint, 700, now);
        assert_eq!(
            etas,
            l.etas_for_keeping_every_send(endpoint, 700, now),
            "{endpoint}"
        );
        for k in [0, 1, 2, 57, 299, 300, 699] {
            assert_eq!(
                etas[k],
                l.eta_for(endpoint, k as u32, now),
                "{endpoint} at {k}"
            );
        }
        assert!(etas.windows(2).all(|w| w[0] <= w[1]), "{endpoint}");
        assert!(etas[699] > etas[0], "{endpoint}: {:?}", etas[699]);
    }
    assert!(Limiter::new().etas_for("/unknown", 3, now) == [Duration::ZERO; 3]);
    assert!(cases[0].0.etas_for("/character", 0, now).is_empty());
}

/// A fixed-seed SplitMix64: the fuzz below is the same on every run and
/// every machine, with no RNG dependency.
struct SplitMix(u64);

impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Uniform in `0..=max`.
    fn upto(&mut self, max: u64) -> u64 {
        self.next() % (max + 1)
    }

    fn pick<T: Copy>(&mut self, from: &[T]) -> T {
        from[self.upto(from.len() as u64 - 1) as usize]
    }

    fn chance(&mut self, percent: u64) -> bool {
        self.upto(99) < percent
    }
}

/// A policy the parser would refuse as well as accept: one or two rules
/// of one to three windows, periods 0 to 1800 s, max hits including 0,
/// reported hits around the limit and some active restrictions.
fn fuzz_policy(rng: &mut SplitMix, name: &str) -> Policy {
    let rules = (0..1 + rng.upto(1))
        .map(|i| {
            let windows = 1 + rng.upto(2);
            let mut limits = Vec::new();
            let mut state = Vec::new();
            for _ in 0..windows {
                let period_secs = if rng.chance(60) {
                    rng.pick(&[0, 1, 2, 5, 10, 30, 60, 300, 1800])
                } else {
                    rng.upto(1800)
                };
                let max_hits = if rng.chance(60) {
                    rng.pick(&[0, 1, 2, 3, 5, 10, 30])
                } else {
                    rng.upto(40) as u32
                };
                limits.push(Window {
                    max_hits,
                    period_secs,
                    restriction_secs: rng.upto(600),
                });
                state.push(WindowState {
                    hits: rng.upto(u64::from(max_hits) + 3) as u32,
                    period_secs,
                    restricted_secs: if rng.chance(20) { 1 + rng.upto(119) } else { 0 },
                });
            }
            Rule {
                name: ["Account", "Ip"][i as usize].to_string(),
                limits,
                state,
            }
        })
        .collect();
    Policy {
        name: name.to_string(),
        rules,
    }
}

/// The same policy under its name with new reported state and limits:
/// a dynamic change within its shape (history kept, N9), or with one
/// period moved, a new shape (history discarded, F65).
fn fuzz_rekey(rng: &mut SplitMix, policy: &Policy) -> Policy {
    let mut next = policy.clone();
    let reshape = rng.chance(30);
    for rule in &mut next.rules {
        for (limit, state) in rule.limits.iter_mut().zip(&mut rule.state) {
            limit.max_hits = rng.upto(40) as u32;
            state.hits = rng.upto(u64::from(limit.max_hits) + 3) as u32;
            if reshape && rng.chance(50) {
                limit.period_secs = rng.upto(1800);
                state.period_secs = limit.period_secs;
            }
        }
    }
    next
}

/// `etas_for`'s stale-send drop held to the simulation that keeps every
/// send, over limiter states drawn from a fixed seed: one or two rules,
/// one to three windows, periods 0 to 1800 s and max hits including 0,
/// reported hits and active restrictions, the token policy's name,
/// counted history out of order and some of it after `now`, rekeys of
/// the policy under its name mid-history, 429 holds, uncounted probes
/// and lost sends. The four hand-picked states above passed a drop made
/// 3% too eager; this fuzz fails it (the independent check of the linear
/// `List`, attack 3).
#[test]
fn etas_for_match_the_simulation_that_keeps_every_send_over_fuzzed_states() {
    const STATES: usize = 1_000;
    const POSITIONS: usize = 160;
    let now = far_future();
    let mut rng = SplitMix(0x5eed_2f0c_a11e_7a5e);
    let endpoint = "/fuzz";
    for case in 0..STATES {
        let name = rng.pick(&["fuzz-policy", "token-request-limit", "stash-request-limit"]);
        let mut policy = fuzz_policy(&mut rng, name);
        let mut limiter = Limiter::new();
        // An offset from now: mostly the past, as far back as twice the
        // longest window, and sometimes a little after now.
        let at = |rng: &mut SplitMix| {
            if rng.chance(10) {
                now + Duration::from_millis(rng.upto(60_000))
            } else {
                now - Duration::from_millis(rng.upto(3_600_000))
            }
        };
        let first = at(&mut rng);
        if rng.chance(30) {
            limiter.observe_probe(endpoint, Ok(policy.clone()), serde_json::Value::Null, first);
        } else {
            let _ = limiter.observe(
                endpoint,
                Ok(policy.clone()),
                serde_json::Value::Null,
                true,
                first,
            );
        }
        for _ in 0..rng.upto(40) {
            let when = at(&mut rng);
            match rng.upto(9) {
                0 => {
                    policy = fuzz_rekey(&mut rng, &policy);
                    let _ = limiter.observe(
                        endpoint,
                        Ok(policy.clone()),
                        serde_json::Value::Null,
                        true,
                        when,
                    );
                }
                1 => limiter.note_lost_send(endpoint, when),
                2 => {
                    let retry_after = if rng.chance(70) {
                        RetryAfter::Acceptable {
                            seconds: rng.upto(120),
                        }
                    } else {
                        RetryAfter::Missing
                    };
                    let _ = limiter.observe_landed(
                        endpoint,
                        LandedObservation {
                            policy: Ok(policy.clone()),
                            retry_after: &retry_after,
                            raw: serde_json::Value::Null,
                            counted: true,
                            status: 429,
                            now: when,
                        },
                    );
                }
                3 => {
                    let _ = limiter.observe(
                        endpoint,
                        Ok(policy.clone()),
                        serde_json::Value::Null,
                        false,
                        when,
                    );
                }
                _ => {
                    let _ = limiter.observe(
                        endpoint,
                        Ok(policy.clone()),
                        serde_json::Value::Null,
                        true,
                        when,
                    );
                }
            }
        }
        let etas = limiter.etas_for(endpoint, POSITIONS, now);
        assert_eq!(
            etas,
            limiter.etas_for_keeping_every_send(endpoint, POSITIONS, now),
            "state {case}: {policy:?}"
        );
        for k in [0, 1, 7, 99, POSITIONS - 1] {
            assert_eq!(
                etas[k],
                limiter.eta_for(endpoint, k as u32, now),
                "state {case} at {k}: {policy:?}"
            );
        }
    }
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
    // C129: GGG's 429 whose rate headers are unusable is still GGG's and
    // stays retryable; one with no `X-Rate-Limit-Policy` at all (this
    // one's headers) is Cloudflare's, a block, never retried. Either way
    // the limiter has already recorded it and holds (below).
    let ggg = Verdict::of(
        429,
        &serde_json::json!({ "x-rate-limit-policy": "character-list-request-limit" }),
        None,
    );
    assert!(!ggg.block);
    assert!(matches!(
        classify_response(
            Some(429),
            false,
            observation.clone(),
            &retry_after,
            ggg.block
        ),
        ResponseClassification::RateLimited(RetryAfter::Acceptable { seconds: 2 })
    ));
    let headerless = Verdict::of(429, &serde_json::json!({}), None);
    assert!(headerless.block);
    assert_eq!(
        classify_response(
            Some(429),
            false,
            observation,
            &retry_after,
            headerless.block
        ),
        ResponseClassification::Blocked
    );

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

/// C19: a landed 429 holds its route for its `Retry-After`, clamped to the
/// cap, plus the retry bucket, whatever the header carried. A missing,
/// malformed or negative value counts as zero; an over-cap one as the cap.
/// The classification is untouched: these stay terminal.
#[test]
fn every_landed_429_holds_at_least_the_retry_bucket() {
    let floor = RETRY_BUCKET_PAD + BUFFER;
    let capped = Duration::from_secs(RETRY_AFTER_CAP_SECS) + floor;
    assert_eq!(floor, Duration::from_secs(61));
    assert_eq!(capped, Duration::from_secs(961));
    // The status reports what the header carried (invariant 2), never
    // the hold's arithmetic: no number for a missing or bad value, the
    // over-cap number as sent.
    let cases = [
        (retry(None), floor, None),
        (retry(Some("soon")), floor, None),
        (retry(Some("-1")), floor, None),
        (retry(Some("901")), capped, Some(901)),
        (retry(Some("5000")), capped, Some(5000)),
    ];
    for (retry_after, hold, reported) in cases {
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
        // GGG's 429 (it names its policy, C129) whose other rate headers
        // are missing: what C19 governs. A 429 with no policy header at
        // all is Cloudflare's, a block.
        let headers = [("x-rate-limit-policy", "character-list-request-limit")];
        let observation = limiter.observe_landed(
            "/character",
            LandedObservation {
                policy: parse(&headers),
                retry_after: &retry_after,
                raw: serde_json::Value::Null,
                counted: true,
                status: 429,
                now,
            },
        );
        let verdict = Verdict::of(
            429,
            &serde_json::json!({ "x-rate-limit-policy": "character-list-request-limit" }),
            None,
        );
        let classification =
            classify_response(Some(429), false, observation, &retry_after, verdict.block);
        assert_eq!(
            classification,
            ResponseClassification::RateLimited(retry_after.clone())
        );
        assert!(!retry_after.is_acceptable());
        assert_eq!(
            limiter.wait_for("/character", now),
            hold,
            "the hold for {retry_after}"
        );
        assert_eq!(
            limiter.statuses(now)[0].retry_after_secs,
            reported,
            "the status's retry-after for {retry_after}"
        );
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

/// C19 for a HEAD: a probe's 429 holds its route as a GET's does, its
/// `Retry-After` clamped to the cap plus the retry bucket, whatever its
/// policy headers or `Retry-After` carried. The setup failure's cooldown
/// is a separate effect: the hold outlasts it, and a login that clears
/// the cooldown leaves the hold standing.
#[test]
fn every_head_429_holds_its_route_at_least_the_retry_bucket() {
    let floor = (RETRY_BUCKET_PAD + BUFFER).as_secs();
    let capped = RETRY_AFTER_CAP_SECS + floor;
    let full = || parse(&with_state(CHAR_LIST, "0:10:0,0:300:0"));
    let non_full = || parse(&[("x-rate-limit-policy", "character-list-request-limit")]);
    let cases = [
        ("Full, missing", full(), retry(None), floor),
        ("Full, garbled", full(), retry(Some("soon")), floor),
        ("Full, negative", full(), retry(Some("-1")), floor),
        ("Full, over the cap", full(), retry(Some("901")), capped),
        ("non-Full, missing", non_full(), retry(None), floor),
        (
            "non-Full, acceptable",
            non_full(),
            retry(Some("120")),
            120 + floor,
        ),
        (
            "non-Full, over the cap",
            non_full(),
            retry(Some("901")),
            capped,
        ),
    ];
    for (name, policy, retry_after, hold) in cases {
        let now = far_future();
        let mut limiter = Limiter::new();
        assert!(
            limiter
                .observe_probe_429(
                    "/character",
                    policy,
                    &retry_after,
                    serde_json::Value::Null,
                    now
                )
                .is_err(),
            "{name}: unexpectedly established"
        );
        assert!(
            matches!(
                limiter.endpoint_state("/character", now),
                EndpointState::Degraded { .. }
            ),
            "{name}: the setup failure still degrades the route"
        );
        assert_eq!(
            limiter.wait_for("/character", now),
            Duration::from_secs(hold),
            "{name}: the next send on the route waits the clamped Retry-After plus the bucket"
        );
        limiter.forget_degraded();
        assert_eq!(
            limiter.endpoint_state("/character", now),
            EndpointState::Unknown,
            "{name}"
        );
        assert_eq!(
            limiter.wait_for("/character", now),
            Duration::from_secs(hold),
            "{name}: clearing the cooldown leaves the hold"
        );
    }
}

#[test]
fn response_classification_precedence_is_status_network_then_protocol() {
    // GGG's headers name the policy and lack the rest: the observation
    // fails, and the response is still GGG's.
    let malformed = PolicyObservationError::Parse(PolicyParseError::MissingHeader {
        name: "x-rate-limit-rules".into(),
    });
    let ggg = Verdict::of(
        429,
        &serde_json::json!({ "x-rate-limit-policy": "p" }),
        None,
    );
    let cloudflare = Verdict::of(429, &serde_json::json!({}), None);
    let retry_after = retry(Some("3"));
    // C129: a block comes first; a 429 that is Cloudflare's is no rate
    // limit of GGG's, whatever its transfer or its headers did.
    assert_eq!(
        classify_response(
            Some(429),
            true,
            Err(malformed.clone()),
            &retry_after,
            cloudflare.block
        ),
        ResponseClassification::Blocked
    );
    assert_eq!(
        classify_response(
            Some(429),
            true,
            Err(malformed.clone()),
            &retry_after,
            ggg.block
        ),
        ResponseClassification::RateLimited(retry_after.clone())
    );
    assert_eq!(
        classify_response(Some(500), true, Err(malformed.clone()), &retry_after, false),
        ResponseClassification::Http(500)
    );
    assert_eq!(
        classify_response(Some(200), true, Err(malformed.clone()), &retry_after, false),
        ResponseClassification::Network
    );
    assert_eq!(
        classify_response(
            Some(200),
            false,
            Err(malformed.clone()),
            &retry_after,
            false
        ),
        ResponseClassification::Protocol(malformed)
    );
    assert_eq!(
        classify_response(Some(204), false, Ok(()), &retry_after, false),
        ResponseClassification::Success
    );
    assert_eq!(
        classify_response(None, true, Ok(()), &retry_after, false),
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

    let result = choke.get(route, &url, choke.now(), ALWAYS_WANTED).await;
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
    let choke =
        ChokePoint::with_clock_and_rails(Arc::new(SystemClock), rails.clone(), NO_BEARER_API);

    let landed = choke
        .get(route, &url, choke.now(), ALWAYS_WANTED)
        .await
        .unwrap();
    assert_eq!(landed.status.as_u16(), 429);
    server.await.unwrap();
    assert!(rails.halted().unwrap().contains("429 on GET /test"));

    // No listener behind this URL: a send would fail as a transport
    // error, so a `Halted` result proves nothing was attempted.
    let unreachable = "http://127.0.0.1:9/never";
    let refused = choke
        .get(route, unreachable, choke.now(), ALWAYS_WANTED)
        .await
        .err()
        .expect("halted");
    assert!(matches!(refused, SendError::Halted(_)), "{refused}");
    let head = choke
        .head(route, unreachable, None, choke.now(), ALWAYS_WANTED)
        .await
        .unwrap_err();
    assert!(
        matches!(&head, ProbeError::Failed { reason, .. } if reason.contains("halted by live-test rails")),
        "{head}"
    );
    let post = choke
        .post_form(
            "oauth-token",
            unreachable,
            &[("a", "b")],
            choke.now(),
            ALWAYS_WANTED,
        )
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
        NO_BEARER_API,
    ));

    let head_choke = choke.clone();
    let head_url = format!("{base}/probe");
    let head = tokio::spawn(async move {
        head_choke
            .head("parked", &head_url, None, head_choke.now(), ALWAYS_WANTED)
            .await
    });
    head_arrived.await.unwrap();

    let get_choke = choke.clone();
    let get_url = format!("{base}/data");
    let get = tokio::spawn(async move {
        get_choke
            .get("parked", &get_url, choke.now(), ALWAYS_WANTED)
            .await
    });
    tokio::time::sleep(Duration::from_millis(50)).await; // let it park
    assert!(!get.is_finished(), "the GET must be parked behind the HEAD");

    // GGG's 429 (it names its policy): a ladder trip, not a block (C129).
    let ggg = serde_json::json!({ "x-rate-limit-policy": "elsewhere-policy" });
    rails.record(&SendReport {
        method: "GET",
        route: "elsewhere",
        url_path: "/elsewhere",
        status: Some(429),
        error: None,
        ok: false,
        counted: true,
        rate: &serde_json::Value::Null,
        headers: &ggg,
        shape: None,
        block: false,
        unreachable: false,
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
    let completed = choke
        .get(route, &url, choke.now(), ALWAYS_WANTED)
        .await
        .expect("landed");
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
        .get(route, "http://127.0.0.1:9/lost", choke.now(), ALWAYS_WANTED)
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
        .head(
            route,
            "http://127.0.0.1:9/lost",
            None,
            choke.now(),
            ALWAYS_WANTED,
        )
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

    let result = choke.get(route, &url, choke.now(), ALWAYS_WANTED).await;
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
        choke.get(route, &url, choke.now(), ALWAYS_WANTED).await,
        Err(SendError::Protocol(_))
    ));
    let send = choke.recent_sends().pop().unwrap();
    assert!(!send.ok);
    assert!(send.outcome.contains("protocol failure"));
    server.await.unwrap();
}

/// C131's limit: a GET's protocol failure keeps nothing. A 200 carrying
/// a token-shaped body and no rate-limit header at all, on a route not
/// yet established, is still an error with no body, and installs no hold;
/// the token route's keep and hold are the token route's alone.
#[tokio::test]
async fn a_headerless_get_2xx_keeps_nothing_and_takes_no_hold() {
    let route = "headerless-get";
    let body = r#"{"access_token":"at-x","refresh_token":"rt-x","expires_in":3600,"username":"u"}"#;
    let (url, server) = serve_one_raw(raw_response("200 OK", "", body.len(), body)).await;
    let choke = ChokePoint::new();

    assert!(matches!(
        choke.get(route, &url, choke.now(), ALWAYS_WANTED).await,
        Err(SendError::Protocol(_))
    ));
    assert_eq!(choke.check(route), Ok(()), "a GET takes no hold");
    assert!(!choke.recent_sends().pop().unwrap().ok);
    server.await.unwrap();
}

/// A server that answers its first request with what `first` builds from
/// its base URL and every later one with a 200 carrying full policy
/// headers, counting the requests that reach it. Every answer closes its
/// connection, so a followed redirect is a new connection and is counted.
async fn counting_server(
    first: impl FnOnce(&str) -> String,
) -> (
    String,
    Arc<std::sync::atomic::AtomicUsize>,
    tokio::task::JoinHandle<()>,
) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let first = first(&base);
    let requests = Arc::new(AtomicUsize::new(0));
    let seen = requests.clone();
    let task = tokio::spawn(async move {
        loop {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = [0; 4096];
            let _ = stream.read(&mut request).await.unwrap();
            let answer = if seen.fetch_add(1, Ordering::SeqCst) == 0 {
                first.clone()
            } else {
                raw_response("200 OK", &full_rate_headers("c130-after", None), 2, "{}")
            };
            stream.write_all(answer.as_bytes()).await.unwrap();
        }
    });
    (base, requests, task)
}

/// A 3xx carrying full policy headers and a `Location`.
fn redirect_response(status: &str, location: &str) -> String {
    raw_response(
        status,
        &format!(
            "{}Location: {location}\r\n",
            full_rate_headers("c130-policy", None)
        ),
        0,
        "",
    )
}

/// C130: the one client follows no redirect. A 307 on the token POST and
/// a 302 on a GET each land as the response they are: one request reaches
/// the server, the choke point records one send, and the journal writes
/// one line whose headers keep the `location` with its query reduced to a
/// mark. A bearer GET whose 302 points at another port, or at another
/// path on its own origin, sends nothing more.
#[tokio::test]
async fn a_3xx_is_one_send_one_journal_line_and_no_second_request() {
    use crate::rails::RailsConfig;
    use std::sync::atomic::Ordering;
    const SECRET: &str = "c130-sentinel-code";

    for (method, status_line, status) in [
        ("POST", "307 Temporary Redirect", 307),
        ("GET", "302 Found", 302),
    ] {
        let (base, requests, server) = counting_server(|base| {
            redirect_response(status_line, &format!("{base}/elsewhere?code={SECRET}"))
        })
        .await;
        let temp = tempfile::tempdir().unwrap();
        let journal = temp.path().join("c130-journal.jsonl");
        let rails = Arc::new(Rails::with_config(RailsConfig {
            journal_path: Some(journal.clone()),
            ..RailsConfig::default()
        }));
        let choke = ChokePoint::with_clock_and_rails(Arc::new(SystemClock), rails, NO_BEARER_API);

        let url = format!("{base}/start");
        let landed = match method {
            "POST" => {
                choke
                    .post_form(
                        "oauth-token",
                        &url,
                        &[
                            ("grant_type", "refresh_token"),
                            ("refresh_token", "c130-rt"),
                        ],
                        choke.now(),
                        ALWAYS_WANTED,
                    )
                    .await
            }
            _ => {
                choke
                    .get("c130-get", &url, choke.now(), ALWAYS_WANTED)
                    .await
            }
        };
        assert_eq!(
            requests.load(Ordering::SeqCst),
            1,
            "a {status} on {method} reached the server more than once: the redirect was followed"
        );
        let landed = landed.unwrap_or_else(|error| panic!("{method} {status}: {error}"));
        assert_eq!(landed.status.as_u16(), status, "{method}");
        let sends = choke.recent_sends();
        assert_eq!(sends.len(), 1, "{method}: one send recorded");
        assert!(!sends[0].ok, "{method}: a 3xx is not a success");

        let text = std::fs::read_to_string(&journal).unwrap();
        let lines: Vec<serde_json::Value> = text
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert_eq!(lines.len(), 2, "the header and one send: {text}");
        assert_eq!(lines[1]["method"], method);
        assert_eq!(lines[1]["status"], status);
        assert_eq!(
            lines[1]["headers"]["location"],
            format!("{base}/elsewhere?…"),
            "{text}"
        );
        assert!(!text.contains(SECRET), "the journal kept the query: {text}");
        server.abort();
    }

    // The bearer arm: a 302 to another port of the same host (the residual
    // past C128's check, which judges only the URL a caller hands in), and
    // to another path of the API origin itself.
    let foreign = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let foreign_port = foreign.local_addr().unwrap().port();
    for elsewhere in ["foreign", "own"] {
        let (base, requests, server) = counting_server(|base| {
            let location = match elsewhere {
                "foreign" => format!("http://127.0.0.1:{foreign_port}/x"),
                _ => format!("{base}/x"),
            };
            redirect_response("302 Found", &location)
        })
        .await;
        let choke = ChokePoint::with_clock_and_rails(
            Arc::new(SystemClock),
            Arc::new(Rails::disabled()),
            &base,
        );
        let bearer = CountingBearer(std::sync::atomic::AtomicUsize::new(0));
        let landed = choke
            .get_bearer(
                "c130-bearer",
                &format!("{base}/profile"),
                &bearer,
                choke.now(),
                ALWAYS_WANTED,
            )
            .await;
        assert_eq!(
            requests.load(Ordering::SeqCst),
            1,
            "a bearer GET's 302 to the {elsewhere} location was followed"
        );
        assert!(
            tokio::time::timeout(Duration::from_millis(200), foreign.accept())
                .await
                .is_err(),
            "a bearer GET's 302 to the {elsewhere} location reached the foreign port"
        );
        assert_eq!(landed.expect("the 302 lands").status.as_u16(), 302);
        assert_eq!(choke.recent_sends().len(), 1, "{elsewhere}: one send");
        server.abort();
    }
}

#[test]
fn a_journaled_location_keeps_its_origin_and_path_and_marks_the_rest() {
    // Every arm is judged before the assert, so a red names them all.
    let mut wrong = Vec::new();
    for (raw, kept) in [
        (
            "https://api.example/oauth/token",
            "https://api.example/oauth/token",
        ),
        (
            "https://api.example:8443/a?code=s",
            "https://api.example:8443/a?…",
        ),
        (
            "https://u:p@api.example/a#access_token=s",
            "https://api.example/a#…",
        ),
        ("/relative?refresh_token=s#t", "/relative?…#…"),
        ("/relative#t?x", "/relative#…"),
        // A scheme-relative reference names its own host: shown, with
        // its user information dropped like an absolute one's.
        ("//u:p@evil.example/x?code=s", "//evil.example/x?…"),
        ("//evil.example/x?code=s", "//evil.example/x?…"),
        ("//evil.example:8443/x", "//evil.example:8443/x"),
        ("\\\\evil\\cb?code=s", "//evil/cb?…"),
        ("https:\\\\evil\\cb?code=s", "https://evil/cb?…"),
        // A path-relative one is the path it resolves to.
        ("cb/next?code=s", "/cb/next?…"),
        ("https://secretuser@h/", "https://h/"),
        (
            "https://bücher.example/x?y",
            "https://xn--bcher-kva.example/x?…",
        ),
        ("  https://h/a\tb?code=s", "https://h/ab?…"),
        // A host-less scheme has no origin to show: the scheme alone.
        ("data:text/plain,secret", "data:…"),
        ("mailto:secret@h", "mailto:…"),
        ("urn:secret", "urn:…"),
        ("https://[bad/x?code=s", "<unreadable location>"),
    ] {
        let got = redacted_location(raw);
        if got != kept {
            wrong.push(format!("{raw:?} gave {got:?}, not {kept:?}"));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    // A location past the bound is cut, and its marks survive the cut.
    let long = format!("https://h/{}?code=s", "a".repeat(200_000));
    let kept = redacted_location(&long);
    assert!(kept.starts_with("https://h/aaa"), "{kept}");
    assert!(kept.ends_with("…?…"), "{kept}");
    assert_eq!(kept.chars().count(), LOCATION_CHARS + 3, "{kept}");
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
            .get(&route, &url, choke.now(), ALWAYS_WANTED)
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
                    .get(&format!("route-{index}"), &url, choke.now(), ALWAYS_WANTED)
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
        tokio::spawn(async move {
            choke
                .get("ordinary-route", &get_url, choke.now(), ALWAYS_WANTED)
                .await
        })
    };
    get_arrived.await.unwrap();
    let head = {
        let choke = choke.clone();
        tokio::spawn(async move {
            choke
                .head("head-route", &head_url, None, choke.now(), ALWAYS_WANTED)
                .await
        })
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
                .get("writer-first-route", &first_url, choke.now(), ALWAYS_WANTED)
                .await
        })
    };
    let second = {
        let choke = choke.clone();
        tokio::spawn(async move {
            choke
                .get(
                    "writer-second-route",
                    &second_url,
                    choke.now(),
                    ALWAYS_WANTED,
                )
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
                .get(
                    "writer-canceled-route",
                    &canceled_url,
                    choke.now(),
                    ALWAYS_WANTED,
                )
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
                .head(
                    "writer-head-route",
                    &head_url,
                    None,
                    choke.now(),
                    ALWAYS_WANTED,
                )
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
                .get("writer-later-route", &later_url, choke.now(), ALWAYS_WANTED)
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

/// A bearer that counts how often its token was read.
struct CountingBearer(std::sync::atomic::AtomicUsize);

impl Bearer for CountingBearer {
    fn token(&self) -> Option<String> {
        self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Some("c128-token".into())
    }
    fn rejected(&self, _token: &str) {}
}

/// C128: a bearer goes only to the provider's API origin. A GET or a HEAD
/// carrying one to another port, another host name or another scheme is
/// refused before any permit, the token is never read, and no connection
/// reaches the foreign server or the API's own listener (the other host
/// name and the other scheme point at it). The same bearer to the API
/// origin still goes out, carrying the token.
#[tokio::test]
async fn c128_a_bearer_goes_only_to_the_api_origin_and_nothing_reaches_either_server() {
    let api = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let foreign = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let api_port = api.local_addr().unwrap().port();
    let foreign_port = foreign.local_addr().unwrap().port();
    let choke = ChokePoint::with_clock_and_rails(
        Arc::new(SystemClock),
        Arc::new(Rails::disabled()),
        &format!("http://127.0.0.1:{api_port}"),
    );
    let bearer = CountingBearer(std::sync::atomic::AtomicUsize::new(0));
    let off_origin = [
        format!("http://127.0.0.1:{foreign_port}/profile"),
        format!("http://localhost:{api_port}/profile"),
        format!("https://127.0.0.1:{api_port}/profile"),
    ];
    let bound = Duration::from_secs(5);
    for url in &off_origin {
        let get = tokio::time::timeout(
            bound,
            choke.get_bearer("c128", url, &bearer, choke.now(), ALWAYS_WANTED),
        )
        .await
        .unwrap_or_else(|_| panic!("a bearer GET to {url} was not refused before sending"));
        assert!(
            matches!(&get, Err(SendError::OffOrigin(_))),
            "a bearer GET to {url}: {:?}",
            get.as_ref().err()
        );
        let head = tokio::time::timeout(
            bound,
            choke.head(
                "c128",
                url,
                Some(&bearer as &dyn Bearer),
                choke.now(),
                ALWAYS_WANTED,
            ),
        )
        .await
        .unwrap_or_else(|_| panic!("a bearer HEAD to {url} was not refused before sending"));
        assert!(
            matches!(&head, Err(ProbeError::OffOrigin(_))),
            "a bearer HEAD to {url}: {:?}",
            head.as_ref().err()
        );
    }
    assert_eq!(
        bearer.0.load(std::sync::atomic::Ordering::SeqCst),
        0,
        "the token was read for a send that was refused"
    );
    for (name, listener) in [("the foreign server", &foreign), ("the API's", &api)] {
        assert!(
            tokio::time::timeout(Duration::from_millis(200), listener.accept())
                .await
                .is_err(),
            "a refused send reached {name} listener"
        );
    }
    assert!(
        choke.recent_sends().is_empty(),
        "a refused send was recorded"
    );

    // The control: the API origin admits the same bearer.
    let server = tokio::spawn(async move {
        let (mut stream, _) = api.accept().await.unwrap();
        let mut request = [0; 4096];
        let n = stream.read(&mut request).await.unwrap();
        let request = String::from_utf8_lossy(&request[..n]).to_ascii_lowercase();
        stream
            .write_all(
                raw_response("200 OK", &full_rate_headers("c128-policy", None), 2, "{}").as_bytes(),
            )
            .await
            .unwrap();
        request
    });
    let landed = choke
        .get_bearer(
            "c128-control",
            &format!("http://127.0.0.1:{api_port}/profile"),
            &bearer,
            choke.now(),
            ALWAYS_WANTED,
        )
        .await
        .expect("a bearer GET to the API origin goes out");
    assert_eq!(landed.status.as_u16(), 200);
    let request = server.await.unwrap();
    assert!(
        request.contains("authorization: bearer c128-token"),
        "the API origin's send carried no bearer: {request}"
    );
}

// ---- generated header sets (#85) ------------------------------------------
//
// The tables above are the cases someone thought of; this property is the
// ones nobody did. A header set is drawn either from a table row with one
// value replaced, or whole, its fields from the values where the grammar
// and the arithmetic branch: zero, the caps and one past them, the edges
// of `u32` and `u64`, a sign, a padded or empty field, a wrong field count.
// What no set may do (upstream input, a GGG response; `REVIEW-CHECKLIST.md`,
// "A parsed value that overflows"): panic in the parse, be accepted outside
// the caps the parse claims, or panic in the limiter that observes it and
// is read after it.

mod generated_headers {
    use super::*;
    use crate::generated::config;
    use proptest::prelude::*;

    type HeaderSet = Vec<(String, String)>;

    /// The values where the grammar and the arithmetic branch.
    const EDGE_FIELDS: &[&str] = &[
        "0",
        "1",
        "10",
        "300",
        "900",
        "901",
        "86400",
        "86401",
        "4294967295",
        "4294967296",
        "18446744073709551615",
        "18446744073709551616",
        "99999999999999999999",
        "-1",
        "+1",
        "",
        " 7 ",
        "007",
        "1.5",
        "x",
    ];

    fn field() -> impl Strategy<Value = String> {
        prop_oneof![
            3 => (0u64..=700).prop_map(|n| n.to_string()),
            3 => prop::sample::select(EDGE_FIELDS).prop_map(str::to_string),
            1 => any::<u64>().prop_map(|n| n.to_string()),
            1 => "[ 0-9a-z+.:-]{0,6}",
        ]
    }

    fn triplet() -> impl Strategy<Value = String> {
        prop_oneof![
            6 => prop::collection::vec(field(), 3),
            1 => prop::collection::vec(field(), 2..=4),
        ]
        .prop_map(|fields| fields.join(":"))
    }

    fn windows() -> impl Strategy<Value = String> {
        (
            prop::collection::vec(triplet(), 1..=3),
            prop::bool::weighted(0.1),
        )
            .prop_map(|(triplets, trailing)| {
                let mut value = triplets.join(",");
                if trailing {
                    value.push(',');
                }
                value
            })
    }

    /// A header set drawn whole: a policy name, one to three rules (mixed
    /// case, padded, empty or repeated), each with a limit and a state
    /// header present nine times in ten, and a `Retry-After` half the time.
    fn whole_set() -> impl Strategy<Value = HeaderSet> {
        let names = prop::sample::select(&["Account", "Ip", "Client", "account", " Ip", ""][..]);
        (
            prop::sample::select(&["character-list-request-limit", "p", "  ", ""][..]),
            prop::collection::vec(names, 1..=3),
            prop::option::of(field()),
        )
            .prop_flat_map(|(policy, rules, retry)| {
                let windows = prop::collection::vec(
                    (
                        prop::option::weighted(0.9, windows()),
                        prop::option::weighted(0.9, windows()),
                    ),
                    rules.len(),
                );
                (Just(policy), Just(rules), Just(retry), windows)
            })
            .prop_map(|(policy, rules, retry, windows)| {
                let mut set = vec![
                    ("x-rate-limit-policy".to_string(), policy.to_string()),
                    ("x-rate-limit-rules".to_string(), rules.join(",")),
                ];
                for (rule, (limit, state)) in rules.iter().zip(windows) {
                    let key = rule.trim().to_ascii_lowercase();
                    if let Some(limit) = limit {
                        set.push((format!("x-rate-limit-{key}"), limit));
                    }
                    if let Some(state) = state {
                        set.push((format!("x-rate-limit-{key}-state"), state));
                    }
                }
                if let Some(retry) = retry {
                    set.push(("retry-after".to_string(), retry));
                }
                set
            })
    }

    /// Every header set the tables above hold: the N5 contract, the
    /// observed one- and three-window shapes, D8's refusals and the pacing
    /// rows.
    fn table_sets() -> Vec<HeaderSet> {
        let own = |set: &[(&str, &str)]| -> HeaderSet {
            set.iter()
                .map(|(name, value)| (name.to_string(), value.to_string()))
                .collect()
        };
        let mut sets = vec![
            own(&with_state(CHAR_LIST, "1:10:0,1:300:0")),
            own(TOKEN_SHAPE),
            own(TRADE_SHAPE),
        ];
        sets.extend(D8_REJECTED.iter().map(|(_, set)| own(set)));
        sets.extend(pacing_rows().iter().map(|row| own(row.headers)));
        sets
    }

    /// How a table row is changed: one field of one triplet replaced, so
    /// the rest of the set stays valid and the field alone decides, in the
    /// limit header and, when `paired`, at the same place in its state
    /// header (a period must match across the two to be accepted); or a
    /// whole value replaced by a field or a window list.
    #[derive(Debug, Clone)]
    enum Mutation {
        Field {
            at: prop::sample::Index,
            within: prop::sample::Index,
            value: String,
            paired: bool,
        },
        Value(prop::sample::Index, String),
    }

    fn mutation() -> impl Strategy<Value = Mutation> {
        prop_oneof![
            3 => (
                any::<prop::sample::Index>(),
                any::<prop::sample::Index>(),
                field(),
                any::<bool>(),
            )
                .prop_map(|(at, within, value, paired)| Mutation::Field {
                    at,
                    within,
                    value,
                    paired,
                }),
            1 => (any::<prop::sample::Index>(), prop_oneof![field(), windows()])
                .prop_map(|(at, value)| Mutation::Value(at, value)),
        ]
    }

    /// `raw` with its field at `within` replaced by `value`, counting the
    /// fields across both separators: ':' within a triplet, ',' between.
    fn replace_field(raw: &str, within: prop::sample::Index, value: &str) -> String {
        let fields: Vec<&str> = raw.split([':', ',']).collect();
        let mut separators = raw.chars().filter(|c| *c == ':' || *c == ',');
        let target = within.index(fields.len());
        let mut out = String::new();
        for (index, field) in fields.into_iter().enumerate() {
            out.push_str(if index == target { value } else { field });
            if let Some(separator) = separators.next() {
                out.push(separator);
            }
        }
        out
    }

    /// A table row with one mutation, and a drawn `Retry-After` one time
    /// in five.
    fn mutated_row() -> impl Strategy<Value = HeaderSet> {
        (
            prop::sample::select(table_sets()),
            prop::option::weighted(0.9, mutation()),
            prop::option::weighted(0.2, field()),
        )
            .prop_map(|(mut set, mutation, retry)| {
                match mutation {
                    _ if set.is_empty() => {}
                    None => {}
                    Some(Mutation::Value(at, value)) => {
                        let at = at.index(set.len());
                        set[at].1 = value;
                    }
                    Some(Mutation::Field {
                        at,
                        within,
                        value,
                        paired,
                    }) => {
                        let at = at.index(set.len());
                        let name = set[at].0.clone();
                        set[at].1 = replace_field(&set[at].1, within, &value);
                        let partner = match name.strip_suffix("-state") {
                            Some(limit) => limit.to_string(),
                            None => format!("{name}-state"),
                        };
                        if paired
                            && let Some(other) = set.iter_mut().find(|(key, _)| *key == partner)
                        {
                            other.1 = replace_field(&other.1, within, &value);
                        }
                    }
                }
                if let Some(retry) = retry {
                    set.push(("retry-after".to_string(), retry));
                }
                set
            })
    }

    fn header_set() -> impl Strategy<Value = HeaderSet> {
        prop_oneof![mutated_row(), whole_set()]
    }

    fn get(set: &HeaderSet) -> impl Fn(&str) -> Option<String> + '_ {
        |name| {
            set.iter()
                .find(|(key, _)| key == name)
                .map(|(_, value)| value.clone())
        }
    }

    /// The set as the choke point's snapshot keeps it.
    fn raw(set: &HeaderSet) -> serde_json::Value {
        serde_json::Value::Object(
            set.iter()
                .filter(|(name, _)| name.starts_with("x-rate-limit") || name == "retry-after")
                .map(|(name, value)| (name.clone(), value.clone().into()))
                .collect(),
        )
    }

    /// What `Policy::parse` claims of every policy it accepts.
    fn assert_within_caps(policy: &Policy) {
        assert!(
            !policy.name.trim().is_empty(),
            "an accepted policy has a name"
        );
        assert!(!policy.rules.is_empty(), "an accepted policy has a rule");
        for rule in &policy.rules {
            assert!(!rule.name.is_empty(), "an accepted rule has a name");
            assert!(!rule.limits.is_empty(), "rule {} has no window", rule.name);
            assert_eq!(rule.limits.len(), rule.state.len(), "rule {}", rule.name);
            for (limit, state) in rule.limits.iter().zip(&rule.state) {
                assert!(limit.max_hits > 0, "{limit:?}");
                assert!(
                    (1..=MAX_POLICY_PERIOD_SECS).contains(&limit.period_secs),
                    "{limit:?}"
                );
                assert!(
                    limit.restriction_secs <= MAX_POLICY_RESTRICTION_SECS,
                    "{limit:?}"
                );
                assert_eq!(limit.period_secs, state.period_secs, "{state:?}");
                assert!(
                    state.restricted_secs <= MAX_POLICY_RESTRICTION_SECS,
                    "{state:?}"
                );
            }
        }
    }

    /// Every read the daemon makes of a limiter, at one instant.
    fn read_all(limiter: &Limiter, endpoint: &str, ahead: u32, at: Instant) {
        let _ = limiter.wait_for(endpoint, at);
        let _ = limiter.eta_for(endpoint, ahead, at);
        let _ = limiter.etas_for(endpoint, ahead as usize, at);
        let _ = limiter.statuses(at);
        let _ = limiter.project(endpoint, ahead, at);
        let _ = limiter.endpoint_state(endpoint, at);
        let _ = limiter.degraded_endpoints(at);
        let _ = limiter.is_live(at);
    }

    proptest! {
        #![proptest_config(config(2_048, 100_000))]

        /// A header set, then a second as the next response on the route:
        /// each parsed, the first observed as the history before now, the
        /// second landed as a 200 or a 429, a probe's 429 on a second
        /// route, and the limiter read at now and at a drawn later instant.
        #[test]
        fn a_generated_header_set_parses_within_its_caps_and_the_limiter_survives_it(
            first in header_set(),
            second in header_set(),
            history in prop::collection::vec(0u64..=4_000, 0..=8),
            status in prop::sample::select(&[200u16, 429][..]),
            ahead in 0u32..=48,
            later in 0u64..=3 * MAX_POLICY_PERIOD_SECS,
        ) {
            let now = far_future();
            let mut limiter = Limiter::new();
            let first_policy = Policy::parse(get(&first));
            let second_policy = Policy::parse(get(&second));
            for policy in [&first_policy, &second_policy].into_iter().flatten() {
                assert_within_caps(policy);
            }
            let mut ago = history;
            ago.sort_unstable_by(|a, b| b.cmp(a));
            for ago in ago {
                let _ = limiter.observe(
                    "/ep",
                    first_policy.clone(),
                    raw(&first),
                    true,
                    now - Duration::from_secs(ago),
                );
            }
            let retry_after = parse_retry_after(get(&second));
            let _ = retry_after.carried_secs();
            let _ = limiter.observe_landed(
                "/ep",
                LandedObservation {
                    policy: second_policy.clone(),
                    retry_after: &retry_after,
                    raw: raw(&second),
                    counted: true,
                    status,
                    now,
                },
            );
            let _ = limiter.observe_probe_429(
                "/probe",
                second_policy,
                &retry_after,
                raw(&second),
                now,
            );
            for endpoint in ["/ep", "/probe"] {
                read_all(&limiter, endpoint, ahead, now);
                read_all(&limiter, endpoint, ahead, now + Duration::from_secs(later));
            }
        }
    }
}
