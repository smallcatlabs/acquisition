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

/// C144 (every keyring result reaches the words), issue #61, the review
/// of h1, its second finding, and the owner's ruling on the tail round:
/// `auth logout` says what the keyring did, text and `--json`, from the
/// answer's `keyring` field. The mock world runs with the keyring off
/// (`ACQ_NO_KEYRING`), where a clear tries nothing: a logout drops the
/// session, exits 0 and says the keyring is off and nothing was cleared;
/// a forget (a logout of a known account that is not live) says the same;
/// a logout with no session left is the refusal's own document.
#[test]
fn c144_logout_json_says_what_the_keyring_did() {
    let scratch = harness::scratch("logout");
    let base = scratch.0.clone();
    let _daemon = harness::login(&base, "Alice");

    let out = acq(&base, &["auth", "logout", "--json"]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    let report = sole_json(&out);
    assert_eq!(report["logged_out"], true, "{report}");
    assert_eq!(report["keyring"], "off", "{report}");

    let out = acq(&base, &["auth", "logout", "--json"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let report = sole_json(&out);
    assert_eq!(report["kind"], "not_logged_in", "{report}");

    let out = acq(&base, &["auth", "logout", "--account", "Alice"]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    let said = String::from_utf8_lossy(&out.stdout);
    assert_eq!(said.trim(), "logged out (keyring off, nothing cleared)");

    let _login = harness::login(&base, "Alice");
    let out = acq(&base, &["auth", "logout"]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    assert!(out.stderr.is_empty(), "{out:?}");
    let said = String::from_utf8_lossy(&out.stdout);
    assert_eq!(said.trim(), "logged out (keyring off, nothing cleared)");
}

/// C144, the review of the h1 fix round, its fifth finding: `--json`
/// reads the keyring's outcome from the answer's field, never from the
/// daemon's sentence. The mock username `keyring entry cleared`, logged
/// out with the keyring off, answers `keyring: "off"`; the fix round
/// searched the sentence, found the username's words in it and said
/// `cleared` although nothing was.
#[test]
fn c144_logout_json_reads_the_keyring_field_not_the_sentence() {
    let scratch = harness::scratch("forged");
    let base = scratch.0.clone();
    let _daemon = harness::login(&base, "keyring+entry+cleared");
    let out = acq(&base, &["auth", "logout", "--json"]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    let report = sole_json(&out);
    assert_eq!(report["keyring"], "off", "{report}");
}

/// C156: a printed command pasted into cmd.exe keeps its single quotes and
/// splits its quoted words, so the parser refuses it; on Windows the
/// refusal then names the shell the commands are for (the review of
/// words' fix round, its second finding: the hint ran only after a
/// command parsed). Elsewhere no shell leaves such words, and nothing is
/// said.
#[test]
fn c156_a_refused_cmd_paste_names_the_shell_the_commands_are_for() {
    let scratch = harness::scratch("cmd-paste");
    let out = harness::offline_acq(&scratch.0, &["search", "'rarity=rare", "is:corrupted'"]);
    assert_eq!(out.status.code(), Some(2), "{out:?}");
    let said = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        said.contains("cmd.exe") && said.contains("PowerShell 7.3 or newer"),
        cfg!(windows),
        "{said}"
    );
}

/// C144's retry command runs as printed, the review of h1, its seventh
/// finding (part of issue #16, printed commands quoted for the shell):
/// the account `Alice Smith` is named by its uuid, which no shell splits
/// or quotes (C156; the review of words' fix round, its first finding:
/// a quoted username followed the daemon's shell, not the paster's), so
/// the command `sh` reads back is one `--account`, which clap parses and
/// the daemon resolves; a bare `Alice Smith` was two words and clap
/// refused the line. The command is
/// run by `sh` exactly as printed, with this build's `acq` first on the
/// path, in the same mock world.
#[cfg(unix)]
#[test]
fn c144_the_printed_logout_command_runs_as_printed() {
    let scratch = harness::scratch("retry");
    let base = scratch.0.clone();
    let _daemon = harness::login(&base, "Alice+Smith");
    // A keyring-off logout succeeds and prints no command; one whose
    // index cannot be read fails, and its words carry the command that
    // clears the entry with the keyring on. The index is unreadable for
    // that one logout.
    let index = base.join("store").join("mock").join("accounts.json");
    assert!(index.exists(), "{index:?}");
    unreadable(&index, true);
    let out = acq(&base, &["auth", "logout"]);
    unreadable(&index, false);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let said = String::from_utf8_lossy(&out.stderr).into_owned();
    let printed = said
        .split_once("with it on, `")
        .and_then(|(_, rest)| rest.split_once('`'))
        .map(|(command, _)| command.to_string())
        .unwrap_or_else(|| panic!("no command in the words: {said}"));
    // the account by its uuid, a word no shell quotes, so the line runs
    // as printed in whichever shell it is pasted into (C156)
    let uuid = acquisition_store::Index::load(&base.join("store").join("mock"))
        .unwrap()
        .resolve(Some("Alice Smith"))
        .unwrap()
        .uuid
        .clone()
        .expect("a login records the uuid");
    assert_eq!(printed, format!("acq auth logout --account {uuid}"));

    let bin = harness::acq_exe().parent().unwrap();
    let path = std::env::join_paths(std::iter::once(bin.to_path_buf()).chain(
        std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()),
    ))
    .unwrap();
    let mut sh = harness::command_of(std::path::Path::new("/bin/sh"), &base, &["-c", &printed]);
    let out = sh.env("PATH", path).output().expect("running sh");
    let said = String::from_utf8_lossy(&out.stderr);
    assert_ne!(out.status.code(), Some(2), "clap refused the line: {said}");
    assert_eq!(
        out.status.code(),
        Some(0),
        "the daemon resolved the account: {said}"
    );
    assert_eq!(
        String::from_utf8_lossy(&out.stdout).trim(),
        "logged out (keyring off, nothing cleared)"
    );
}

/// A file made unreadable, or readable again.
#[cfg(unix)]
fn unreadable(file: &std::path::Path, shut: bool) {
    use std::os::unix::fs::PermissionsExt;
    let mode = if shut { 0o000 } else { 0o644 };
    std::fs::set_permissions(file, std::fs::Permissions::from_mode(mode)).unwrap();
}

/// The login's wait is the daemon's (#68): under `ACQ_LOGIN_WITHIN=0` on
/// the `acq auth` that starts the daemon, the browser wait expires at
/// once and the CLI ends with the daemon's words, never with a bound of
/// its own; a late callback can no longer be given up on by the CLI
/// while the daemon still registers the session.
#[test]
fn the_login_ends_with_the_daemons_words_not_a_bound_of_its_own() {
    let scratch = harness::scratch("within");
    let base = scratch.0.clone();
    let _daemon = harness::stop_on_drop(&base);
    let out = harness::command(&base, &["auth", "--no-browser"])
        .env("ACQ_LOGIN_WITHIN", "0")
        .output()
        .expect("spawning acq auth");
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("login failed: the browser did not complete the login within 0 s"),
        "{stderr}"
    );
    assert!(stderr.contains("run `acq auth` again"), "{stderr}");
    assert!(!stderr.contains("5 minutes"), "{stderr}");
}
