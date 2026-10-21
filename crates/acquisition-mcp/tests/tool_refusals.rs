//! A tool's refusal, on every tool: the request was wrong, or the store
//! or policy cannot serve it, and the answer is a result marked
//! `isError` holding `error` and a `kind` — the instructions' promise,
//! which the search and pricing tools kept and the store, intent and
//! plan tools did not (the release seat's rerun, mcp-opus-2 calls 50 and
//! 46: an unknown account on `tabs`, an uncovered league on
//! `refresh_plan`, both JSON-RPC `-32603`). A call the server could not
//! serve stays a JSON-RPC error: a daemon's own refusal carries its
//! closed kind in `data` (C85).

use crate::harness;

use std::time::{Duration, Instant};

use acquisition_plan::{PolicyWrite, put_sync_policy};
use acquisition_store::{Annotations, Endpoint, Index, Provenance, Store, account_path};
use harness::{Mcp, spawn_daemon};
use serde_json::{Value, json};

const USER: &str = "Alice#1234";
const UUID: &str = "u-refusal";

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
        PolicyWrite::Replace,
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
    let scratch = harness::scratch("mcp-refusal");
    let base = scratch.0.clone();
    seed(&base);
    let mut mcp = Mcp::start(&base, &[]);

    // An account no store knows, on every tool that selects one: the
    // protocol's word for it (C121), the known names as accounts.
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
        let doc = refusal(&mut mcp, tool, args, "not_logged_in");
        assert_eq!(doc["accounts"], json!([USER]), "{tool}: {doc}");
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

/// A request the server cannot read gets its own reply, and the server
/// answers the next call: a query tree of 123 nots or 62 groups, or any
/// message past 127 JSON levels, was dropped before any tool saw it,
/// the caller waiting on it (the frontends park, now built). A tool call
/// nested past the limit is a refusal like any other, `arguments`, its
/// words naming the limit; a line that is not JSON is a parse error with
/// a null id.
#[test]
fn a_request_the_server_cannot_read_gets_a_reply_and_the_next_call_one_too() {
    let scratch = harness::scratch("mcp-unreadable");
    let base = scratch.0.clone();
    seed(&base);
    let mut mcp = Mcp::start(&base, &[]);
    let term = json!({ "field": "rarity", "op": "=", "value": "rare" });
    let nots = (0..123).fold(term.clone(), |inner, _| json!({ "not": inner }));
    let groups = (0..62).fold(term, |inner, _| json!({ "all": [inner] }));
    let deep = (0..128).fold(json!(1), |inner, _| json!([inner]));
    for (n, arguments) in [
        json!({ "query": { "tree": nots } }),
        json!({ "query": { "tree": groups } }),
        json!({ "query": { "text": "rarity=rare" }, "deep": deep }),
    ]
    .into_iter()
    .enumerate()
    {
        let id = json!(format!("deep-{n}"));
        let line = json!({ "jsonrpc": "2.0", "id": id, "method": "tools/call",
            "params": { "name": "search", "arguments": arguments } });
        let reply = mcp.raw(&line.to_string(), &id);
        let doc = &reply["result"]["structuredContent"];
        assert_eq!(reply["result"]["isError"], json!(true), "{reply}");
        assert_eq!(doc["kind"], json!("arguments"), "{reply}");
        assert!(
            doc["error"].as_str().unwrap().contains("127 levels"),
            "{reply}"
        );
        // the next call is answered, its own
        let next = mcp.result("sync_policy", json!({}));
        assert!(next["structuredContent"]["policy"].is_object(), "{next}");
    }
    let reply = mcp.raw(r#"{"jsonrpc":"2.0","id":99,"method":"#, &Value::Null);
    assert_eq!(reply["error"]["code"], json!(-32700), "{reply}");
    let next = mcp.result("sync_policy", json!({}));
    assert!(next["structuredContent"]["policy"].is_object(), "{next}");
}

/// A line `serde_json` refuses never reaches `rmcp`, which would drop it:
/// a deep line cut short is a parse error with a null id, even with its
/// id and method read first, and a line behind a UTF-8 byte order mark
/// is read as `rmcp` reads it (the 0c review's inputs).
#[test]
fn a_deep_line_cut_short_and_a_line_behind_a_byte_order_mark_each_get_their_reply() {
    let scratch = harness::scratch("mcp-cut-short");
    let base = scratch.0.clone();
    seed(&base);
    let mut mcp = Mcp::start(&base, &[]);
    for line in [
        format!(r#"{{"params":{}"#, "[".repeat(128)),
        format!(
            r#"{{"jsonrpc":"2.0","id":"cut","method":"tools/call","params":{}"#,
            "[".repeat(128)
        ),
    ] {
        let reply = mcp.raw(&line, &Value::Null);
        assert_eq!(reply["error"]["code"], json!(-32700), "{reply}");
    }
    let id = json!("bom");
    let line = json!({ "jsonrpc": "2.0", "id": id, "method": "tools/call",
        "params": { "name": "sync_policy", "arguments": {} } });
    let reply = mcp.raw(&format!("\u{feff}{line}"), &id);
    assert!(
        reply["result"]["structuredContent"]["policy"].is_object(),
        "{reply}"
    );
}

/// The server's writer is awaited however the service ends: a line
/// answered before initialisation, then stdin closed, still puts its
/// parse error on stdout. A server that never exits on stdin closing
/// fails the test within the harness's deadline rather than hanging it.
#[test]
fn a_reply_queued_before_initialisation_reaches_stdout_when_stdin_closes() {
    let scratch = harness::scratch("mcp-early-eof");
    let base = scratch.0.clone();
    let stdout = harness::run_to_exit(&base, b"not JSON\n");
    let reply: Value = serde_json::from_str(stdout.lines().next().unwrap_or_default())
        .unwrap_or_else(|e| panic!("no reply on stdout ({e}): {stdout:?}"));
    assert_eq!(reply["id"], Value::Null, "{reply}");
    assert_eq!(reply["error"]["code"], json!(-32700), "{reply}");
}
