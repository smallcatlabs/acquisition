//! The one real-GGG-mode rule left at the MCP boundary after the agent
//! traffic ruling (C14: agents may use the gate; the daemon enforces
//! GGG's rules): `acq-mcp` **never spawns a daemon in ggg mode** (C13). A
//! real-mode daemon is a human's act via the CLI (keychain, browser).
//! Pinned in the mode it protects, with no daemon and no login: the
//! spending tools must fail by reporting the absent daemon, and the
//! scratch socket must never come into existence (a regression that
//! lazy-spawned here would create it). `submit_job` is driven with
//! nothing else present; `apply_plan` needs an account's store on disk
//! to get past its offline gates to the daemon's door, so one is seeded
//! under the scratch root — written by the store crate, sent by no one.

use crate::harness;

use std::path::Path;

use acquisition_plan::RefreshPlan;
use acquisition_store::{Endpoint, Index, Store, account_path};
use harness::Mcp;
use serde_json::{Value, json};

const USER: &str = "Refusal#0001";
const UUID: &str = "00000000-0000-4000-8000-00000000c013";

const POLICY: &str =
    r#"{"version":1,"leagues":{"Standard":{"tabs":"all","max_age_seconds":3600}}}"#;

/// What a login leaves on disk for provider `ggg`, with no login and no
/// daemon: the index entry and the facts file's account identity (the
/// one profile response a login lands). Nothing is listed, so a policy
/// over the league plans the stash listing — one request owed.
fn seed_ggg_account(base: &Path) {
    let dir = base.join("store").join("ggg");
    std::fs::create_dir_all(&dir).unwrap();
    let at = acquisition_store::now();
    let mut index = Index::load(&dir).unwrap();
    index.record_login(USER, UUID, false, at).unwrap();
    let mut store = Store::open(&account_path(&dir, USER)).unwrap();
    store
        .record(
            &Endpoint::Profile,
            &json!({}),
            200,
            &json!({ "uuid": UUID, "name": USER }),
            at,
        )
        .unwrap();
}

#[test]
fn ggg_mode_never_spawns_a_daemon_for_the_spending_tools() {
    let scratch = harness::scratch("m8g");
    let base = scratch.0.clone();

    let mut mcp = Mcp::start(&base, &[("ACQ_PROVIDER", "ggg")]);

    // submit_job: no daemon is running and none may be started here, so
    // the call fails on the absent daemon — not on the mode — and the
    // message names this server's own policy and the human's remedy,
    // never a spawn "on demand" (review, 2026-09-10).
    let msg = mcp.expect_err("submit_job", json!({ "kind": "stashes" }));
    assert!(
        !msg.contains("deferred"),
        "the lifted deferral is still being cited: {msg}"
    );
    assert!(
        msg.contains("never starts one") && msg.contains("acq auth check"),
        "the absence must name the policy and the README's remedy: {msg}"
    );
    assert!(!msg.contains("on demand"), "{msg}");
    assert!(!msg.contains("ACQ_NO_SPAWN"), "{msg}");
    assert!(
        harness::no_daemon_appeared(&base),
        "submit_job in ggg mode spawned a daemon (socket appeared): {msg}"
    );

    // apply_plan: a plan that passes every offline gate — a known
    // account, the stored policy's revision, provider ggg, one request
    // owed — so the call reaches the daemon's door. Declaring the intent
    // and compiling the plan are offline in either mode (C41) and raise
    // no daemon either.
    seed_ggg_account(&base);
    let policy: Value = serde_json::from_str(POLICY).unwrap();
    mcp.expect_ok("set_sync_policy", json!({ "value": policy }));
    let planned = mcp.expect_ok("refresh_plan", json!({}));
    let plan = RefreshPlan::from_value(&planned["plan"])
        .expect("refresh_plan returned an invalid envelope");
    assert_eq!(plan.provider, "ggg", "{planned}");
    assert!(
        !plan.actions.is_empty(),
        "an empty plan is a no-op before the daemon's door: {planned}"
    );
    assert!(
        harness::no_daemon_appeared(&base),
        "refresh_plan in ggg mode spawned a daemon (socket appeared): {planned}"
    );

    // At the door it fails as submit_job does: on the absent daemon, by
    // this server's own policy, with the human's remedy — and none starts.
    let msg = mcp.expect_err("apply_plan", json!({ "plan": planned["plan"] }));
    assert!(
        msg.contains("never starts one") && msg.contains("acq auth check"),
        "the absence must name the policy and the README's remedy: {msg}"
    );
    assert!(!msg.contains("on demand"), "{msg}");
    assert!(!msg.contains("ACQ_NO_SPAWN"), "{msg}");
    assert!(
        harness::no_daemon_appeared(&base),
        "apply_plan in ggg mode spawned a daemon (socket appeared): {msg}"
    );

    let _ = mcp.child.kill();

    // The other absence: mock mode, where this server would spawn, under
    // `ACQ_NO_SPAWN=1` — the knob is named, not the policy.
    let mut mcp = Mcp::start(&base, &[("ACQ_NO_SPAWN", "1")]);
    let msg = mcp.expect_err("submit_job", json!({ "kind": "stashes" }));
    assert!(msg.contains("ACQ_NO_SPAWN"), "{msg}");
    assert!(!msg.contains("never starts one"), "{msg}");
    assert!(
        harness::no_daemon_appeared(&base),
        "submit_job under ACQ_NO_SPAWN spawned a daemon: {msg}"
    );
    let _ = mcp.child.kill();
    let _ = std::fs::remove_dir_all(&base);
}
