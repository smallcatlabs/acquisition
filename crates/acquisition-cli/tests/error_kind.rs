//! A daemon refusal through the binary carries its closed kind beside
//! the message under `--json` (C85; additive under C53), and the text
//! mode prints the message alone: the kind is for a program to branch on,
//! never a prefix on prose.

use crate::harness;

use harness::{acq, sole_json, start_daemon};

#[test]
fn c85_a_refusal_carries_its_kind_in_json_and_only_its_message_in_text() {
    let scratch = harness::scratch("kind");
    let base = scratch.0.clone();
    let (_daemon, _) = start_daemon(&base);

    let out = acq(&base, &["status", "999", "--json"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    assert_eq!(
        sole_json(&out),
        serde_json::json!({ "error": "no job 999", "kind": "unknown_job" })
    );

    let out = acq(&base, &["stashes", "--json"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let report = sole_json(&out);
    assert_eq!(report["kind"], "not_logged_in", "{report}");
    assert!(
        report["error"].as_str().unwrap().contains("acq auth"),
        "{report}"
    );

    // Stays a literal: one refusal line, held whole by this equality.
    let out = acq(&base, &["status", "999"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(stderr.trim(), "Error: no job 999", "{stderr}");

    // No daemon to ask: the ledger holds no such job, in the daemon's
    // word for an id it does not remember, with the command that starts
    // a daemon (C45).
    let _ = acq(&base, &["daemon", "stop"]);
    let out = acq(&base, &["status", "999", "--json"]);
    let report = sole_json(&out);
    assert_eq!(report["kind"], "unknown_job", "{report}");
    assert!(report["next"].is_string(), "{report}");
    assert!(
        report["error"].as_str().unwrap().contains("not running"),
        "{report}"
    );
}

/// C144 (every keyring result reaches the words), issue #61 and the
/// review of h1, its second finding: `auth logout` says what the keyring
/// did, text and `--json`. The mock world runs with the keyring off
/// (`ACQ_NO_KEYRING`), where a clear tries nothing: a logout drops the
/// session and says the keyring is off and nothing was cleared — an
/// entry a run with the keyring on saved may still be there — where it
/// used to say "keyring cleared"; a forget (a logout of a known account
/// that is not live) says the same; `--json` carries `not cleared` on the
/// refusal's own document, whatever refused it. An acknowledged clear is
/// pinned in the daemon's tests.
#[test]
fn c144_logout_json_says_what_the_keyring_did() {
    let scratch = harness::scratch("logout");
    let base = scratch.0.clone();
    let off = "the keyring is off (disabled by ACQ_NO_KEYRING), so no keyring entry was cleared";
    harness::login(&base, "Alice");

    let out = acq(&base, &["auth", "logout", "--json"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let report = sole_json(&out);
    assert_eq!(report["kind"], "internal", "{report}");
    assert_eq!(report["keyring"], "not cleared", "{report}");
    let said = report["error"].as_str().unwrap();
    for fact in [
        "logged out Alice (session dropped, not restored at start), but",
        off,
    ] {
        assert!(said.contains(fact), "{fact:?} in {report}");
    }

    let out = acq(&base, &["auth", "logout", "--json"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let report = sole_json(&out);
    assert_eq!(report["kind"], "not_logged_in", "{report}");
    assert_eq!(report["keyring"], "not cleared", "{report}");

    let out = acq(&base, &["auth", "logout", "--account", "Alice"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let said = String::from_utf8_lossy(&out.stderr);
    for fact in ["forgot account Alice (not restored at start), but", off] {
        assert!(said.contains(fact), "{fact:?} in {said}");
    }

    harness::login(&base, "Alice");
    let out = acq(&base, &["auth", "logout"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    assert!(out.stdout.is_empty(), "{out:?}");
    let said = String::from_utf8_lossy(&out.stderr);
    for fact in [
        "logged out Alice (session dropped, not restored at start), but",
        off,
    ] {
        assert!(said.contains(fact), "{fact:?} in {said}");
    }
}
