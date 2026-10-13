//! The faults the whole-CLI seat found (2026-10-13,
//! `audits/rehome-2026-10/seat/REPORT.md` at `7653f8b0`), each held by the
//! test named for it. A test here drives the real binary in a scratch
//! world against the mock; nothing reaches GGG.

use serde_json::Value;

mod harness;

use harness::{acq, sole_json, start_daemon, text};

/// A fresh daemon with nothing to restore said `keyring: ok` under
/// `ACQ_NO_KEYRING=1`: the status started as `ok` and changed only when a
/// save was tried, so the one line that says whether a login will outlive
/// the daemon was wrong until the first login.
#[test]
fn a_fresh_daemon_told_not_to_use_the_keyring_never_reports_it_ok() {
    let scratch = harness::scratch("seat-keyring");
    let base = scratch.0.clone();
    let (_daemon, _pid) = start_daemon(&base);

    let out = acq(&base, &["auth", "status"]);
    let said = text(&out);
    assert!(
        said.contains("keyring: unavailable: disabled by ACQ_NO_KEYRING"),
        "{said}"
    );
    assert!(!said.contains("keyring: ok"), "{said}");

    let status: Value = sole_json(&acq(&base, &["auth", "status", "--json"]));
    assert_eq!(
        status["keyring"], "unavailable: disabled by ACQ_NO_KEYRING",
        "{status}"
    );
}
