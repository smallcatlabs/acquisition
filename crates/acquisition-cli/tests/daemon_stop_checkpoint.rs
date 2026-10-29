//! Plan step 7, item 4 (`PRICING-SLICE.md` at `b6197a82`): the daemon leaves by
//! `process::exit`, so its SQLite connections are never closed and the
//! write-ahead logs stayed uncheckpointed beside the facts file and
//! `daemon.db` (the price-notes run, 2026-09-04:
//! `tools/notes-check.py` at `aacd6a51` had to read through the WAL,
//! `tools/census.py` at `aacd6a51` refuses one). Pinned here through the
//! real binaries over the in-process mock: after a fetch has landed
//! facts, `acq daemon stop` leaves no `-wal` pages behind either file. The pre-stop assertion keeps the check honest —
//! the WAL holds pages before the stop, so an empty one after it is the
//! checkpoint's doing.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde_json::Value;

use crate::harness;

use harness::{acq, login_as_mock, sole_json};

/// The `-wal` beside a database file: its size, or `None` when absent.
fn wal_len(db: &Path) -> Option<u64> {
    let mut wal = db.as_os_str().to_owned();
    wal.push("-wal");
    std::fs::metadata(wal).ok().map(|m| m.len())
}

#[test]
fn daemon_stop_checkpoints_the_facts_file_and_the_queue() {
    let scratch = harness::scratch("ckpt");
    let base = scratch.0.clone();
    login_as_mock(&base, &[]);

    // A listing lands facts in the account's file through the daemon.
    let out = acq(&base, &["stashes", "--json"]);
    assert!(out.status.success(), "{out:?}");
    let listing = sole_json(&out);
    assert!(
        !listing["payload"]["stashes"]
            .as_array()
            .unwrap_or_else(|| panic!("{listing}"))
            .is_empty(),
        "the mock lists tabs"
    );
    let mock = base.join("store").join("mock");
    let facts: PathBuf = std::fs::read_dir(&mock)
        .unwrap()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .find(|p| {
            p.extension().is_some_and(|x| x == "db")
                && p.file_name().is_some_and(|n| n != "daemon.db")
                && !p.to_string_lossy().ends_with(".annotations.db")
        })
        .expect("the account's facts file");
    let queue = mock.join("daemon.db");
    // The daemon holds both open; their WALs carry what it wrote.
    assert!(
        wal_len(&facts).is_some_and(|n| n > 0),
        "before the stop the facts WAL holds pages: {:?}",
        wal_len(&facts)
    );
    assert!(
        wal_len(&queue).is_some_and(|n| n > 0),
        "before the stop the queue WAL holds pages: {:?}",
        wal_len(&queue)
    );

    let out = acq(&base, &["daemon", "stop"]);
    assert!(out.status.success(), "{out:?}");
    // The daemon checkpoints, removes its socket, then exits: wait until
    // nothing answers on this world's socket any more.
    let deadline = Instant::now() + Duration::from_secs(120);
    loop {
        let status = sole_json(&acq(&base, &["daemon", "status", "--json"]));
        if status["running"] == Value::Bool(false) {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "the daemon did not exit: {status}"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    assert_eq!(
        wal_len(&facts).unwrap_or(0),
        0,
        "the facts WAL after the stop: {:?} bytes",
        wal_len(&facts)
    );
    assert_eq!(
        wal_len(&queue).unwrap_or(0),
        0,
        "the queue WAL after the stop: {:?} bytes",
        wal_len(&queue)
    );
}
