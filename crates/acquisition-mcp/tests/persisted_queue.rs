//! C45 through the MCP binary: with no daemon to ask, the job tools answer
//! from the ledger on disk and spawn nothing. What the ledger's answers
//! hold is the verbs crate's (`acquisition-verbs/tests/daemon.rs`); here,
//! the adapter's part: a refusal is a result marked an error, and a
//! result is the document `acq result --json` prints. The ledger is
//! seeded through the store crate, the daemon's own writer.

use crate::harness;

use acquisition_store::jobs::{JobDb, JobRow, daemon_db_path};
use harness::Mcp;
use serde_json::json;

fn row(id: u64, state: &str, updated_at: i64) -> JobRow {
    JobRow {
        id,
        kind: "stash".into(),
        state: state.into(),
        priority: 5,
        submitted_by: "mcp".into(),
        parent: None,
        retries: 0,
        account: Some("Exile#1234".into()),
        params: json!({ "league": "Standard", "id": format!("tab{id}") }),
        outcome: None,
        deferred: None,
        cancel_requested: false,
        submitted_at: 1_700_000_000,
        updated_at,
    }
}

/// C45, one job on both surfaces: with no daemon to ask, `job_result` of
/// a finished job is the document `acq result --json` prints, and of an
/// unfinished one a result marked an error, `wrong_state`, with what to
/// run next.
#[test]
fn c45_one_job_is_read_from_disk_on_both_surfaces() {
    let scratch = harness::scratch("mcp-c45-one");
    let base = scratch.0.clone();

    let dir = base.join("store").join("mock");
    let db = JobDb::open(&daemon_db_path(&dir)).unwrap();
    db.upsert(&row(1, "waiting", 1_700_000_010)).unwrap();
    db.upsert(&JobRow {
        outcome: Some(json!({ "outcome": "success", "payload": { "fetched": 1 } })),
        ..row(3, "done", 1_700_000_040)
    })
    .unwrap();
    db.checkpoint().unwrap();
    drop(db);

    let mut mcp = Mcp::start(&base, &[]);
    let result = mcp.expect_ok("job_result", json!({ "id": 3 }));
    assert_eq!(result["job_id"], 3, "{result}");
    assert_eq!(
        json_of(&harness::acq(&base, &["result", "3", "--json"])),
        result
    );
    let refused = mcp.result("job_result", json!({ "id": 1 }));
    assert_eq!(refused["isError"], json!(true), "{refused}");
    assert_eq!(
        refused["structuredContent"]["kind"],
        json!("wrong_state"),
        "{refused}"
    );
    assert!(
        refused["structuredContent"]["next"].is_string(),
        "{refused}"
    );
    assert!(
        harness::no_daemon_appeared(&base),
        "a tool spawned a daemon"
    );
}

/// The one JSON document a CLI command printed.
fn json_of(out: &std::process::Output) -> serde_json::Value {
    serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
        panic!(
            "no one JSON document ({e}): {}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        )
    })
}
