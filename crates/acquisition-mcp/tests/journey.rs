//! The journey's MCP twin (C122, C123 — the mock's real-shaped items):
//! the first session the CLI's golden transcript holds
//! (`acquisition-cli/tests/journey.rs`), through the tools an agent
//! calls, each field compared to the one table both doors read
//! (`acquisition-client/tests/harness/journey.rs`). The daemon is started
//! by the test with the journal on its own command alone, the login runs
//! through the `acq` beside the server, and the journal is checked by
//! field after the stop, through the shared reader, as the CLI's is.
//!
//! One difference from the CLI's door, by the tools' own contract:
//! `price_clear` requires `if_revision`, the revision of the row it
//! removes, where `acq price clear` takes the row as it stands
//! (`MCP-REFERENCE.md`); the twin sends the revision `price_set` wrote.

use std::time::Duration;

use serde_json::{Value, json};

use crate::harness;

use harness::Mcp;
use harness::journey::*;

#[test]
fn a_first_session_through_the_tools_holds_the_journeys_facts() {
    let scratch = harness::scratch("mjourney");
    let base = scratch.0.clone();
    let (knob, journal) = harness::journal_on(&base);
    let (mut daemon, pid) = harness::start_daemon_with(&base, &[(knob, journal.as_str())]);
    harness::login_as_mock(&base);
    let mut mcp = Mcp::start(&base, &[]);

    let policy: Value = serde_json::from_str(POLICY).expect("the table's policy is JSON");
    let written = mcp.expect_ok("set_sync_policy", json!({ "value": policy }));
    assert_eq!(written["revision"], json!(POLICY_REVISION), "{written}");

    // plan and apply until the plan names nothing
    for requests in PLAN_REQUESTS {
        let plan = mcp.expect_ok("refresh_plan", json!({}));
        assert_eq!(plan["plan"]["logical_requests"], json!(requests), "{plan}");
        let applied = mcp.expect_ok("apply_plan", json!({ "plan": plan["plan"] }));
        assert_eq!(applied["requests"], json!(requests), "{applied}");
        let outcome = harness::finish_job(&mut mcp, &applied["job_id"]);
        assert_eq!(outcome["outcome"], "success", "{outcome}");
        assert_eq!(
            outcome["payload"]["children"]["done"],
            json!(requests),
            "{outcome}"
        );
    }
    let plan = mcp.expect_ok("refresh_plan", json!({}));
    assert_eq!(plan["plan"]["logical_requests"], json!(0), "{plan}");

    let tabs = mcp.expect_ok("tabs", json!({}));
    assert_eq!(
        (&tabs["tabs"], &tabs["fetched"], &tabs["items"]),
        (
            &json!(TABS_LISTED),
            &json!(TABS_FETCHED),
            &json!(STASH_ITEMS)
        ),
        "{tabs}"
    );

    let rings = mcp.expect_ok("search", json!({ "query": { "text": RING_QUERY } }));
    assert_eq!(rings["total"]["matched"], json!(RING_MATCHES), "{rings}");
    assert_eq!(
        (&rings["rows"][0]["id"], &rings["rows"][0]["name"]),
        (&json!(RING), &json!(RING_NAME)),
        "{rings}"
    );

    let life = mcp.expect_ok(
        "search",
        json!({ "query": { "text": LIFE_QUERY }, "view": { "rows": { "sort": LIFE_SORT } } }),
    );
    assert_eq!(life["total"]["matched"], json!(LIFE_MATCHES), "{life}");
    let sorted: Vec<(&Value, &Value)> = life["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .map(|row| (&row["name"], &row["sort"]["value"]))
        .collect();
    assert_eq!(
        sorted,
        [
            (&json!(ARMOUR_NAME), &json!(ARMOUR_TOTAL_LIFE)),
            (&json!(RING_NAME), &json!(RING_TOTAL_LIFE)),
        ],
        "{life}"
    );

    // the count view the CLI's `--count base --sum stack` builds
    let currency = mcp.expect_ok(
        "search",
        json!({ "query": { "text": CURRENCY_QUERY },
                "view": { "counts": { "keys": ["base"], "sum": "stack" } } }),
    );
    assert_eq!(
        currency["total"]["matched"],
        json!(CURRENCY_MATCHES),
        "{currency}"
    );
    let counts = &currency["view"]["counts"];
    assert_eq!(counts["sum"]["value"], json!(SHARD_STACK), "{currency}");
    assert_eq!(
        counts["tables"][0]["buckets"][0]["value"],
        json!(SHARD),
        "{currency}"
    );

    let target = format!("item/{RING}");
    let [kind, amount, currency_tag] = PRICE;
    let set = mcp.expect_ok(
        "price_set",
        json!({ "target": target, "type": kind, "amount": amount, "currency": currency_tag }),
    );
    assert_eq!(set["written"]["revision"], json!(PRICE_REVISION), "{set}");

    let shown = mcp.expect_ok("show", json!({ "id": RING }));
    assert_eq!(shown["item"]["unread"], json!([]), "{shown}");
    assert!(
        shown["lines"]
            .as_array()
            .expect("lines")
            .iter()
            .any(|line| line["flags"] == json!(["crafted"])),
        "{shown}"
    );
    assert_eq!(
        (&shown["price"]["kind"], &shown["price"]["currency"]),
        (&json!(kind), &json!(currency_tag)),
        "{shown}"
    );

    let shop = mcp.expect_ok("shop_render", json!({}));
    assert_eq!(
        (&shop["counts"]["posted"], &shop["counts"]["pages"]),
        (&json!(POSTED_ITEMS), &json!(POSTED_PAGES)),
        "{shop}"
    );

    let cleared = mcp.expect_ok(
        "price_clear",
        json!({ "target": target, "if_revision": PRICE_REVISION }),
    );
    assert_eq!(
        cleared["prior"]["revision"],
        json!(PRICE_REVISION),
        "{cleared}"
    );
    assert_eq!(cleared["written"], Value::Null, "{cleared}");

    let status = mcp.expect_ok("daemon_status", json!({}));
    assert_eq!(
        (
            &status["running"],
            &status["provider"],
            &status["compatible"],
            &status["pid"]
        ),
        (&json!(true), &json!("mock"), &json!(true), &json!(pid)),
        "{status}"
    );

    drop(mcp);
    let stop = harness::acq(&base, &["daemon", "stop"]);
    assert!(stop.status.success(), "{stop:?}");
    daemon.wait_exit(Duration::from_secs(10));
    harness::assert_journal_holds(&base, pid, &GETS, TOKEN_POSTS);
}
