//! C45 through the MCP binary: `list_jobs`, `job_status`, `job_result`
//! and `daemon_status` with no daemon to ask answer from the ledger on
//! disk, and spawn nothing. The ledger is seeded through the store crate,
//! the daemon's own writer.

mod harness;

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

#[test]
fn c45_the_tools_read_the_queue_on_disk_when_no_daemon_answers() {
    let scratch = harness::scratch("mcp-c45");
    let base = scratch.0.clone();

    let dir = base.join("store").join("mock");
    let db = JobDb::open(&daemon_db_path(&dir)).unwrap();
    db.upsert(&row(1, "waiting", 1_700_000_010)).unwrap();
    db.upsert(&row(2, "running", 1_700_000_030)).unwrap();
    db.upsert(&row(3, "cancelled", 1_700_000_040)).unwrap();
    db.checkpoint().unwrap();
    drop(db);

    let mut mcp = Mcp::start(&base, &[]);
    let report = mcp.expect_ok("list_jobs", json!({}));
    assert_eq!(report["running"], json!(false), "{report}");
    let ids: Vec<u64> = report["persisted"]["jobs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|j| j["id"].as_u64().unwrap())
        .collect();
    assert_eq!(ids, vec![1, 2], "{report}");
    // The picture's age is the newest write of any row, the cancelled
    // one included: it says how stale the ledger is, not the open rows.
    assert_eq!(report["persisted"]["written_at"], json!(1_700_000_040));

    assert_eq!(
        mcp.expect_ok("daemon_status", json!({})),
        json!({
            "running": false,
            "persisted": { "waiting": 1, "recorded_running": 1, "written_at": 1_700_000_040 }
        })
    );
    assert!(
        harness::no_daemon_appeared(&base),
        "a tool spawned a daemon"
    );
}

/// C45, one job and the last log, on both surfaces: with no daemon to
/// ask, `job_status` answers the ledger's row with `next`, `job_result`
/// a finished job's outcome from disk with its id and account (the
/// document `acq result --json` prints, a failed parent's with its failed
/// children detailed), an unfinished one is refused as
/// `wrong_state`, and the daemon status names the log this world's last
/// daemon wrote, as `acq daemon status --json` does.
#[test]
fn c45_one_job_and_the_last_log_are_read_from_disk_on_both_surfaces() {
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
    // A parent whose one child failed, as the daemon leaves them: the
    // tally on the parent, the child's own error on the child.
    db.upsert(&JobRow {
        kind: "apply".into(),
        params: json!({ "jobs": [] }),
        outcome: Some(
            json!({ "outcome": "failure", "error": "1 of 1 child jobs failed, 0 done",
            "children": { "done": 0, "failed": 1, "cancelled": 0, "failed_ids": [5] } }),
        ),
        ..row(4, "failed", 1_700_000_050)
    })
    .unwrap();
    db.upsert(&JobRow {
        parent: Some(4),
        outcome: Some(
            json!({ "outcome": "failure", "error": "GET /stash/Standard/tab5 returned 404" }),
        ),
        ..row(5, "failed", 1_700_000_045)
    })
    .unwrap();
    db.checkpoint().unwrap();
    drop(db);
    // The log a daemon of this world would have written, where it writes
    // it: `<ACQ_LOG_DIR>/<the world's id>/<provider>/daemon.log`.
    let world = acquisition_store::world::World::at(&base.join("store")).unwrap();
    let log = base
        .join("logs")
        .join(world.id())
        .join("mock")
        .join("daemon.log");
    std::fs::create_dir_all(log.parent().unwrap()).unwrap();
    std::fs::write(&log, "the last daemon's words\n").unwrap();

    let mut mcp = Mcp::start(&base, &[]);
    let status = mcp.expect_ok("daemon_status", json!({}));
    assert_eq!(status["running"], json!(false), "{status}");
    let named = std::path::PathBuf::from(status["log"].as_str().expect("a log"));
    assert_eq!(
        std::fs::canonicalize(&named).unwrap(),
        std::fs::canonicalize(&log).unwrap(),
        "{status}"
    );
    let cli = json_of(&harness::acq(&base, &["daemon", "status", "--json"]));
    assert_eq!(cli["log"], status["log"], "{cli}");

    let job = mcp.expect_ok("job_status", json!({ "id": 1 }));
    assert_eq!(job["running"], json!(false), "{job}");
    assert_eq!(job["persisted"]["state"], json!("waiting"), "{job}");
    assert!(job["next"].is_string(), "{job}");

    let result = mcp.expect_ok("job_result", json!({ "id": 3 }));
    assert_eq!(
        result,
        json!({ "job_id": 3, "account": "Exile#1234", "outcome": "success", "payload": { "fetched": 1 } })
    );
    assert_eq!(
        json_of(&harness::acq(&base, &["result", "3", "--json"])),
        result
    );
    // A failed parent's result details its failed child from the ledger,
    // in the one call: kind, target and its own error (C23, C45).
    let parent = mcp.expect_ok("job_result", json!({ "id": 4 }));
    assert_eq!(
        parent["failed_children"],
        json!([{ "job_id": 5, "kind": "stash", "target": "Standard/tab5",
                 "error": "GET /stash/Standard/tab5 returned 404" }]),
        "{parent}"
    );
    assert_eq!(
        json_of(&harness::acq(&base, &["result", "4", "--json"])),
        parent
    );
    let refused = mcp.result("job_result", json!({ "id": 1 }));
    assert_eq!(refused["isError"], json!(true), "{refused}");
    assert_eq!(
        refused["structuredContent"]["kind"],
        json!("wrong_state"),
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
