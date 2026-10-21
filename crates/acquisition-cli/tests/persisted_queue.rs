//! C45 through the real binary: an observer with no daemon to ask reads
//! the queue from the ledger on disk, and spawns nothing. What `jobs` and
//! `daemon status` answer from it is the verbs crate's
//! (`acquisition-verbs/tests/daemon.rs`); here, the text a person reads.
//! The ledger is seeded through the store crate (the daemon's own
//! writer): two waiting rows (one a child), one recorded running, one
//! done that must not appear.
//! And C6's retention beside it: a result a previous daemon left is
//! served to `result --wait` as to `result`.

use acquisition_store::jobs::{JobDb, JobRow, daemon_db_path};
use serde_json::json;

use crate::harness;

use harness::{acq, scratch_tmp, sockets_under, sole_json, start_daemon, text};

fn row(id: u64, state: &str, parent: Option<u64>, updated_at: i64) -> JobRow {
    JobRow {
        id,
        kind: "stash".into(),
        state: state.into(),
        priority: 5,
        submitted_by: "cli".into(),
        parent,
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
fn c45_an_observer_with_no_daemon_reads_the_queue_on_disk() {
    let scratch = harness::scratch("c45");
    let base = scratch.0.clone();

    // The ledger as a mock-mode daemon leaves it: under the world's
    // provider directory (C83).
    let dir = base.join("store").join("mock");
    let db = JobDb::open(&daemon_db_path(&dir)).unwrap();
    db.upsert(&row(1, "waiting", None, 1_700_000_010)).unwrap();
    db.upsert(&row(2, "running", None, 1_700_000_030)).unwrap();
    db.upsert(&row(3, "waiting", Some(2), 1_700_000_020))
        .unwrap();
    db.upsert(&row(4, "done", None, 1_700_000_005)).unwrap();
    db.checkpoint().unwrap();
    drop(db);

    // The text form: the counts and the shared table, no ETA column
    // filled.
    let out = acq(&base, &["jobs"]);
    assert!(out.status.success(), "{out:?}");
    let shown = text(&out);
    assert!(shown.contains("2 waiting, 1 recorded running"), "{shown}");
    assert!(shown.contains("Standard/tab3"), "{shown}");
    assert!(!shown.contains("tab4"), "a done row was listed: {shown}");
    let out = acq(&base, &["daemon", "status", "--json"]);
    assert!(out.status.success(), "{out:?}");

    // Nothing spawned: no socket derived into the scratch runtime
    // directory, and the ledger is untouched.
    assert!(
        sockets_under(&scratch_tmp(&base)).is_empty(),
        "an observer spawned"
    );
    let db = JobDb::open(&daemon_db_path(&dir)).unwrap();
    assert_eq!(db.load().unwrap().len(), 4);
}

/// C6, retention: a job finished by a previous daemon is no longer in
/// the queue, so it has no status, but its result is still served.
/// `result --wait` asked for the status first and refused such a job as
/// unknown while `result` printed it.
#[test]
fn result_wait_serves_a_previous_daemons_result() {
    let scratch = harness::scratch("rwait");
    let base = scratch.0.clone();
    let (daemon, _) = start_daemon(&base);
    let out = acq(
        &base,
        &["submit", "sleep", "--params", r#"{"seconds":0}"#, "--json"],
    );
    assert!(out.status.success(), "{}", text(&out));
    let id = sole_json(&out)["job_id"].as_u64().expect("the job's id");
    assert!(acq(&base, &["daemon", "stop"]).status.success());
    drop(daemon);

    let (_daemon, _) = start_daemon(&base);
    let plain = acq(&base, &["result", &id.to_string(), "--json"]);
    assert!(plain.status.success(), "{}", text(&plain));
    let waited = acq(&base, &["result", &id.to_string(), "--wait", "--json"]);
    assert!(waited.status.success(), "{}", text(&waited));
    assert_eq!(sole_json(&waited), sole_json(&plain));
}
