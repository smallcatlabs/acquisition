//! A tool's refusal, on every tool: the request was wrong, or the store
//! or policy cannot serve it, and the answer is a result marked
//! `isError` holding `error` and a `kind` — the instructions' promise,
//! which the search and pricing tools kept and the store, intent and
//! plan tools did not (the release seat's rerun, mcp-opus-2 calls 50 and
//! 46: an unknown account on `tabs`, an uncovered league on
//! `refresh_plan`, both JSON-RPC `-32603`). A call the server could not
//! serve stays a JSON-RPC error: a daemon's own refusal carries its
//! closed kind in `data` (C85).

mod harness;

use std::path::PathBuf;
use std::time::{Duration, Instant};

use acquisition_plan::put_sync_policy;
use acquisition_store::{Annotations, Endpoint, Index, Provenance, Store, account_path};
use harness::{Mcp, spawn_daemon};
use serde_json::{Value, json};

const USER: &str = "Alice#1234";
const UUID: &str = "u-refusal";

/// The scratch directory, removed on drop; declared before the daemon
/// and the server, so those are gone first.
struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// One account with a store and a sync policy covering Standard alone.
fn seed(base: &std::path::Path) {
    let mock = base.join("store").join("mock");
    std::fs::create_dir_all(&mock).unwrap();
    let at = acquisition_store::now();
    Index::load(&mock)
        .unwrap()
        .record_login(USER, UUID, false, at)
        .unwrap();
    Store::open(&account_path(&mock, USER))
        .unwrap()
        .record(
            &Endpoint::Profile,
            &json!({}),
            200,
            &json!({ "uuid": UUID, "name": USER }),
            at,
        )
        .unwrap();
    let mut a = Annotations::open_for(&mock, UUID).unwrap();
    put_sync_policy(
        &mut a,
        &json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard":
            { "tabs": "all", "max_age_seconds": 86400 } } } } }),
        None,
        &Provenance::via("test"),
    )
    .unwrap();
}

/// The result of a call the server answered, its document and kind
/// checked: a JSON-RPC error fails here, naming the tool.
fn refusal(mcp: &mut Mcp, tool: &str, args: Value, kind: &str) -> Value {
    let result = mcp.result(tool, args.clone());
    let doc = result["structuredContent"].clone();
    assert!(
        result["isError"] == json!(true) && doc["kind"] == json!(kind) && doc["error"].is_string(),
        "{tool} {args} is not a refusal of kind {kind}: {result}"
    );
    doc
}

#[test]
fn a_tool_refusal_is_a_result_marked_is_error_with_its_kind() {
    let base = std::env::temp_dir().join(format!("acq-mcp-refusal-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let _scratch = Scratch(base.clone());
    seed(&base);
    let mut mcp = Mcp::start(&base, &[]);

    // An account no store knows, on every tool that selects one: the
    // account's kind, the known names as readings.
    for (tool, mut args) in [
        ("tabs", json!({})),
        ("characters", json!({})),
        ("store_status", json!({})),
        ("item_events", json!({})),
        ("sync_policy", json!({})),
        (
            "set_sync_policy",
            json!({ "value": { "version": 3, "realms": {} } }),
        ),
        ("refresh_plan", json!({})),
    ] {
        args["account"] = json!("GERWARIK");
        let doc = refusal(&mut mcp, tool, args, "account_unknown");
        assert_eq!(doc["readings"], json!([USER]), "{tool}: {doc}");
    }

    // A league the policy does not cover: refused in the planner's words.
    let doc = refusal(
        &mut mcp,
        "refresh_plan",
        json!({ "league": "Allflame" }),
        "refused",
    );
    assert!(
        doc["error"]
            .as_str()
            .unwrap()
            .contains("does not cover league Allflame"),
        "{doc}"
    );

    // A parameter the tool cannot read.
    refusal(&mut mcp, "tabs", json!({ "realm": "pcx" }), "arguments");
    refusal(
        &mut mcp,
        "item_events",
        json!({ "hours": "a day" }),
        "arguments",
    );

    // A daemon's own refusal is still a JSON-RPC error with its kind.
    let _daemon = spawn_daemon(&base, &[]);
    let deadline = Instant::now() + Duration::from_secs(10);
    while mcp.expect_ok("daemon_status", json!({}))["running"] != json!(true) {
        assert!(Instant::now() < deadline, "the mock daemon did not come up");
        std::thread::sleep(Duration::from_millis(50));
    }
    let error = mcp.expect_rpc_error("job_status", json!({ "id": 999 }));
    assert_eq!(error["data"]["kind"], "unknown_job", "{error}");
    assert_eq!(error["message"], "no job 999", "{error}");
}
