//! The MCP's `daemon_status` through the binary (C10, C84, C83): absence
//! is a state; a running daemon this server may use is reported judged
//! against the `acqd` beside `acq-mcp`, that sibling under `wanted`; a
//! daemon of another provider is reported as incompatible with the
//! dimension named, and never replaced. `ACQ_PROVIDER=ggg` appears only on the observing server, which
//! cannot spawn in real mode by construction: nothing reaches GGG.

use crate::harness;

use std::path::Path;
use std::time::{Duration, Instant};

use acquisition_store::jobs::{JobDb, JobRow, daemon_db_path};
use harness::{Mcp, spawn_daemon};
use serde_json::json;

/// The report's shape, its vitals and identity keys side by side, is the
/// verbs crate's (`acquisition-verbs/tests/documents.rs`); here, what
/// this server adds: absence is a state and no daemon appears for it; a
/// running daemon is judged against the `acqd` beside `acq-mcp`, which the
/// report names under `wanted`; and a server wanting another provider
/// reports the daemon incompatible and never replaces it.
#[test]
fn c84_daemon_status_reports_the_identity_and_the_sibling_it_is_judged_against() {
    let scratch = harness::scratch("mcp-status");
    let base = scratch.0.clone();
    let acqd = acquisition_client::locator::beside(Path::new(env!("CARGO_BIN_EXE_acq-mcp")))
        .unwrap_or_else(|e| panic!("{e}"))
        .canonicalize()
        .unwrap();

    let mut mcp = Mcp::start(&base, &[]);
    assert_eq!(
        mcp.expect_ok("daemon_status", json!({})),
        json!({ "running": false, "persisted": null })
    );
    assert!(
        harness::no_daemon_appeared(&base),
        "daemon_status spawned a daemon"
    );

    let daemon = spawn_daemon(&base, &[]);
    let deadline = Instant::now() + Duration::from_secs(10);
    let status = loop {
        let status = mcp.expect_ok("daemon_status", json!({}));
        if status["running"] == json!(true) {
            break status;
        }
        assert!(
            Instant::now() < deadline,
            "the mock daemon ({}) did not come up",
            acqd.display()
        );
        std::thread::sleep(Duration::from_millis(50));
    };
    assert_eq!(status["compatible"], true, "{status}");
    assert_eq!(
        Path::new(status["wanted"]["acqd"]["path"].as_str().unwrap())
            .canonicalize()
            .unwrap(),
        acqd,
        "{status}"
    );
    assert_eq!(status["artifact_relation"], "same_file", "{status}");
    let pid = status["pid"].as_u64().unwrap();
    assert_eq!(pid, u64::from(daemon.id()), "{status}");

    let mut real = Mcp::start(&base, &[("ACQ_PROVIDER", "ggg")]);
    let report = real.expect_ok("daemon_status", json!({}));
    assert_eq!(report["compatible"], false, "{report}");
    assert_eq!(report["pid"], pid, "{report}");
    assert_eq!(report["provider_matches"], false, "{report}");
    assert_eq!(report["wanted"]["provider"], "ggg", "{report}");
    let again = mcp.expect_ok("daemon_status", json!({}));
    assert_eq!(again["pid"], pid, "the daemon was replaced: {again}");
}

/// The release seat (mcp-opus calls 40 to 42, mcp-sonnet 42 to 45): a
/// job tool with no daemon said what was wrong and not what to do. Every
/// such refusal names the command a person runs to start one, the
/// README's line, in every mode.
#[test]
fn a_job_tool_with_no_daemon_names_the_command_that_starts_one() {
    let scratch = harness::scratch("mcp-remedy");
    let base = scratch.0.clone();
    // A host that forbids a spawn, as real mode does by this server's own
    // policy: the refusal for want of a daemon, in words.
    let mut mcp = Mcp::start(&base, &[("ACQ_NO_SPAWN", "1")]);
    // The tools that act refuse for want of a daemon; the observers
    // answer from the ledger (C45), below.
    for (tool, args) in [
        ("submit_job", json!({ "kind": "leagues" })),
        ("cancel_job", json!({ "id": 1 })),
    ] {
        // The approved form: a result marked isError, `kind: "no_daemon"`,
        // with what a person does — never a protocol error (review 8).
        let result = mcp.result(tool, args);
        let doc = &result["structuredContent"];
        assert!(
            result["isError"] == json!(true)
                && doc["kind"] == json!("no_daemon")
                && doc["next"]
                    .as_str()
                    .is_some_and(|n| n.contains("acq auth check")),
            "{tool} with no daemon is not a no_daemon refusal naming the remedy: {result}"
        );
    }
    // A job the ledger does not hold: the daemon's word for an id it
    // does not remember, with what a person does.
    let result = mcp.result("job_status", json!({ "id": 1 }));
    let doc = &result["structuredContent"];
    assert!(
        result["isError"] == json!(true)
            && doc["kind"] == json!("unknown_job")
            && doc["next"]
                .as_str()
                .is_some_and(|n| n.contains("acq auth check")),
        "a job the ledger does not hold is not unknown_job with next: {result}"
    );
    // A job the ledger holds: its row as recorded, running false, next.
    let db = JobDb::open(&daemon_db_path(&base.join("store").join("mock"))).unwrap();
    db.upsert(&JobRow {
        id: 1,
        kind: "leagues".into(),
        state: "waiting".into(),
        priority: 0,
        submitted_by: "mcp".into(),
        parent: None,
        retries: 0,
        account: None,
        params: json!({}),
        outcome: None,
        deferred: None,
        cancel_requested: false,
        submitted_at: 1_700_000_000,
        updated_at: 1_700_000_001,
    })
    .unwrap();
    db.checkpoint().unwrap();
    drop(db);
    let doc = mcp.expect_ok("job_status", json!({ "id": 1 }));
    assert_eq!(doc["running"], json!(false), "{doc}");
    assert_eq!(doc["persisted"]["id"], json!(1), "{doc}");
    assert_eq!(doc["persisted"]["state"], json!("waiting"), "{doc}");
    assert!(
        doc["next"]
            .as_str()
            .is_some_and(|n| n.contains("acq auth check")),
        "{doc}"
    );
}

/// The release seat's rerun (mcp-opus-2 calls 45, 47): the remedy named
/// `ACQ_IDLE_SHUTDOWN=3600 acq auth check` and nothing of the world the
/// server runs in, so typed as printed beside a mock server with its
/// store elsewhere it started a daemon for another provider and store.
/// The printed command carries the provider when it is not the default
/// and the store directory when the server has one set, quoted for a
/// shell — here the harness's mock world, and a real-mode server over
/// the same directory, which names no provider.
#[test]
fn the_no_daemon_remedy_names_the_provider_and_store_the_server_runs_with() {
    let scratch = harness::scratch("mcp-remedy-world");
    let base = scratch.0.clone();
    let store = acquisition_search::answer::shell_quoted(&base.join("store").display().to_string());
    for (provider, prefix) in [
        ("mock", format!("ACQ_PROVIDER=mock ACQ_STORE_DIR={store} ")),
        ("ggg", format!("ACQ_STORE_DIR={store} ")),
    ] {
        let mut mcp = Mcp::start(&base, &[("ACQ_PROVIDER", provider), ("ACQ_NO_SPAWN", "1")]);
        let result = mcp.result("cancel_job", json!({ "id": 1 }));
        let next = result["structuredContent"]["next"]
            .as_str()
            .unwrap_or_default();
        let command = format!("`{prefix}acq auth check`");
        assert!(
            next.contains(&command),
            "{provider}: the remedy is not {command} as the server's world needs it: {result}"
        );
    }
    assert!(
        harness::no_daemon_appeared(&base),
        "a remedy's refusal started a daemon"
    );
}
