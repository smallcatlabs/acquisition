//! The faults the whole-CLI seat found (2026-10-13,
//! `audits/rehome-2026-10/seat/REPORT.md` at `7653f8b0`), each held by the
//! test named for it. A test here drives the real binary in a scratch
//! world against the mock; nothing reaches GGG.

use std::path::Path;

use acquisition_store::{Endpoint, Index, Store, account_path};
use serde_json::{Value, json};

mod harness;

use harness::{acq, login, sole_json, start_daemon, text};

/// One account's facts in two leagues of pc, seeded through the store
/// crate with no daemon: Standard holds tab `a1` with item `i-std`,
/// Allflame holds tab `b1` with item `i-all`.
fn seed_two_leagues(base: &Path) {
    const USER: &str = "Alice#1234";
    const UUID: &str = "u-seat";
    let mock = base.join("store").join("mock");
    std::fs::create_dir_all(&mock).unwrap();
    let at = acquisition_store::now();
    Index::load(&mock)
        .unwrap()
        .record_login(USER, UUID, false, at)
        .unwrap();
    let mut store = Store::open(&account_path(&mock, USER)).unwrap();
    let mut record = |ep: Endpoint, params: Value, body: Value| {
        store.record(&ep, &params, 200, &body, at).unwrap();
    };
    record(
        Endpoint::Profile,
        json!({}),
        json!({ "uuid": UUID, "name": USER }),
    );
    for (league, tab, item) in [("Standard", "a1", "i-std"), ("Allflame", "b1", "i-all")] {
        record(
            Endpoint::Stashes {
                realm: "pc".into(),
                league: league.into(),
            },
            json!({ "league": league }),
            json!({ "stashes": [ { "id": tab, "name": "Gear", "type": "PremiumStash", "index": 0 } ] }),
        );
        record(
            Endpoint::Stash {
                realm: "pc".into(),
                league: league.into(),
                id: tab.into(),
                sub: None,
            },
            json!({}),
            json!({ "stash": { "id": tab, "name": "Gear", "type": "PremiumStash", "items": [
                { "id": item, "name": "", "typeLine": "Chaos Orb", "baseType": "Chaos Orb", "x": 0, "y": 0 } ] } }),
        );
    }
}

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

/// A price target names no league, `set` takes none, and `show` read
/// Standard unless told: the command a `set` printed as its next step
/// failed for every target outside Standard, and `--league` on `set` was
/// a usage error. `show` and `list` now read the league the facts hold
/// their target in, and a wrong `--league` is refused naming the right
/// one.
#[test]
fn a_price_target_outside_standard_reads_back_as_its_set_printed() {
    let scratch = harness::scratch("seat-league");
    let base = scratch.0.clone();
    seed_two_leagues(&base);

    let out = acq(&base, &["price", "set", "tab/pc/b1", "exact", "5", "chaos"]);
    let said = text(&out);
    assert!(out.status.success(), "{said}");
    assert!(
        said.contains("next: `acq price show tab/pc/b1` reads it"),
        "{said}"
    );

    // The printed command, as printed.
    let out = acq(&base, &["price", "show", "tab/pc/b1"]);
    assert!(out.status.success(), "{}", text(&out));
    let view = sole_json(&acq(&base, &["price", "show", "tab/pc/b1", "--json"]));
    assert_eq!(view["league"], "Allflame", "{view}");

    // An item, and a container given to `list`, find their league too.
    let view = sole_json(&acq(&base, &["price", "show", "item/i-all", "--json"]));
    assert_eq!(view["league"], "Allflame", "{view}");
    let view = sole_json(&acq(
        &base,
        &["price", "list", "--in", "tab/pc/b1", "--json"],
    ));
    assert_eq!(view["league"], "Allflame", "{view}");

    // A league the target is not in is refused, naming the one it is in.
    let out = acq(
        &base,
        &["price", "show", "tab/pc/b1", "--league", "Standard"],
    );
    assert!(!out.status.success());
    let said = text(&out);
    assert!(
        said.contains("the facts hold it in Allflame: `--league` reads it there"),
        "{said}"
    );
}

/// `price status` counted another league's rows under "name nothing in
/// these facts", the words for an orphan: of the owner's 16 rows the 11
/// that priced Allflame read as lost in Standard, and the 5 that priced
/// Standard as lost in Allflame. A row the facts hold in another league
/// is counted as that; "names nothing" is left to the row that does.
#[test]
fn another_leagues_price_row_is_never_counted_as_naming_nothing() {
    let scratch = harness::scratch("seat-rows");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    for target in ["tab/pc/b1", "item/i-all", "tab/pc/gone"] {
        let out = acq(&base, &["price", "set", target, "exact", "5", "chaos"]);
        assert!(out.status.success(), "{}", text(&out));
    }

    let said = text(&acq(&base, &["price", "status"]));
    assert!(
        said.contains(
            "rows: 0 of 3 apply here; 1 name nothing in the facts, 0 unreadable, 2 for other leagues, 0 for other realms"
        ),
        "{said}"
    );
    let report = sole_json(&acq(&base, &["price", "status", "--json"]));
    assert_eq!(
        report["rows"]["other_league"],
        json!([{ "scope": "item", "id": "i-all" }, { "scope": "tab", "realm": "pc", "id": "b1" }]),
        "{report}"
    );
    assert_eq!(
        report["rows"]["unmatched"].as_array().map(Vec::len),
        Some(1)
    );

    let said = text(&acq(&base, &["price", "status", "--league", "Allflame"]));
    assert!(
        said.contains(
            "rows: 2 of 3 apply here; 1 name nothing in the facts, 0 unreadable, 0 for other leagues, 0 for other realms"
        ),
        "{said}"
    );
}

/// `--json` was not total: a command line the parser refused printed
/// clap's text on stderr and nothing on stdout, so a program reading
/// stdout met an empty answer. It is the one document now, with
/// `kind: "usage"` and the parser's exit 2; without `--json` nothing
/// changes, and help is never a failure.
#[test]
fn a_command_line_the_parser_refuses_is_one_json_document_under_json() {
    let scratch = harness::scratch("seat-usage");
    let base = scratch.0.clone();

    let out = acq(
        &base,
        &[
            "price",
            "set",
            "tab/pc/b1",
            "exact",
            "5",
            "chaos",
            "--league",
            "Allflame",
            "--json",
        ],
    );
    assert_eq!(out.status.code(), Some(2), "{out:?}");
    let report = sole_json(&out);
    assert_eq!(report["kind"], "usage", "{report}");
    let message = report["error"].as_str().unwrap();
    assert!(
        message.starts_with("unexpected argument '--league'"),
        "{message}"
    );
    assert!(message.contains("Usage: acq price set"), "{message}");
    assert!(out.stderr.is_empty(), "{out:?}");

    // No verb at all is a refusal too.
    let out = acq(&base, &["--json", "no-such-verb"]);
    assert_eq!(out.status.code(), Some(2), "{out:?}");
    assert_eq!(sole_json(&out)["kind"], "usage");

    // Text mode is the parser's own, untouched.
    let out = acq(&base, &["price", "set", "--league", "Allflame"]);
    assert_eq!(out.status.code(), Some(2), "{out:?}");
    assert!(out.stdout.is_empty(), "{out:?}");
    assert!(String::from_utf8_lossy(&out.stderr).starts_with("error: "));

    // Help is an answer, in either mode.
    let out = acq(&base, &["price", "--help", "--json"]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    assert!(String::from_utf8_lossy(&out.stdout).contains("Usage: acq price"));
}

/// An account that could not be picked offline was a message with no
/// kind, and the accounts to pick from were inside the prose. The
/// document says which failure it is in the daemon's words for the same
/// remedy, and lists the accounts.
#[test]
fn an_account_that_cannot_be_picked_says_which_failure_and_lists_the_accounts() {
    let scratch = harness::scratch("seat-account");
    let base = scratch.0.clone();

    let out = acq(&base, &["tabs", "--json"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    assert_eq!(sole_json(&out)["kind"], "not_logged_in");

    seed_two_leagues(&base);
    Index::load(&base.join("store").join("mock"))
        .unwrap()
        .record_login("Bob#5678", "u-bob", false, acquisition_store::now())
        .unwrap();

    let out = acq(&base, &["tabs", "--json"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let report = sole_json(&out);
    assert_eq!(report["kind"], "ambiguous_account", "{report}");
    assert_eq!(
        report["accounts"],
        json!(["Alice#1234", "Bob#5678"]),
        "{report}"
    );

    let out = acq(&base, &["tabs", "--json", "--account", "Carol"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    assert_eq!(sole_json(&out)["kind"], "unknown_account");

    let out = acq(&base, &["tabs", "--json", "--account", "Alice"]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
}

fn stdout(out: &std::process::Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn stderr(out: &std::process::Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// A job verb in text mode printed its progress lines and its payload on
/// one stream, and nothing it printed said which account the job ran as:
/// with a store known beside the login, the job verbs took the login
/// silently while every offline verb refused. Stdout is the payload
/// alone; progress is stderr's and names the account; the JSON outcome
/// carries the job's id and account.
#[test]
fn a_job_verb_prints_its_answer_alone_and_says_whose_job_it_was() {
    let scratch = harness::scratch("seat-job");
    let base = scratch.0.clone();
    let (_daemon, _pid) = start_daemon(&base);
    login(&base, "alice");

    let out = acq(&base, &["profile"]);
    assert!(out.status.success(), "{}", text(&out));
    let payload: Value = serde_json::from_str(&stdout(&out))
        .unwrap_or_else(|e| panic!("stdout is not the payload alone ({e}):\n{}", stdout(&out)));
    assert_eq!(payload["body"]["name"], "alice", "{payload}");
    let progress = stderr(&out);
    assert!(progress.contains("(alice): done"), "{progress}");

    let outcome = sole_json(&acq(&base, &["profile", "--json"]));
    assert_eq!(outcome["outcome"], "success", "{outcome}");
    assert_eq!(outcome["account"], "alice", "{outcome}");
    assert!(outcome["job_id"].is_u64(), "{outcome}");
}

/// Nothing waited for a job: a `--detach`ed job was collected by polling
/// `result` until it stopped refusing. And `result` of a cancelled job
/// exited 0 with nothing a caller could use, where a waiting one exited 1.
/// `result --wait` blocks until the job is terminal; only a success
/// exits 0.
#[test]
fn a_detached_job_is_collected_by_waiting_and_a_cancelled_one_is_no_success() {
    let scratch = harness::scratch("seat-wait");
    let base = scratch.0.clone();
    let (_daemon, _pid) = start_daemon(&base);

    let submit = |seconds: f64| -> u64 {
        let params = format!("{{\"seconds\":{seconds}}}");
        let out = acq(
            &base,
            &["submit", "sleep", "--params", &params, "--detach", "--json"],
        );
        sole_json(&out)["job_id"].as_u64().expect("a job id")
    };

    let id = submit(1.5).to_string();
    let out = acq(&base, &["result", &id, "--json"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    assert_eq!(sole_json(&out)["kind"], "wrong_state");
    let out = acq(&base, &["result", &id, "--wait", "--json"]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    let outcome = sole_json(&out);
    assert_eq!(outcome["outcome"], "success", "{outcome}");
    assert_eq!(outcome["job_id"].to_string(), id, "{outcome}");

    let id = submit(60.0).to_string();
    assert!(acq(&base, &["cancel", &id]).status.success());
    let out = acq(&base, &["result", &id, "--wait", "--json"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    assert_eq!(sole_json(&out)["outcome"], "cancelled");
    let out = acq(&base, &["result", &id]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    assert!(out.stdout.is_empty(), "{out:?}");
    assert!(
        stderr(&out).contains(&format!("job {id} was cancelled")),
        "{out:?}"
    );
}

/// `status <id>` and `result <id>` said only "daemon is not running"
/// once the daemon had idled out, which it does sixty seconds after the
/// last job: a detached job's result was unreadable without starting a
/// daemon, and in real mode an agent's shell cannot start one. A finished
/// job's outcome is on disk and is answered from there, as `jobs` answers
/// the queue (C45); nothing is spawned.
#[test]
fn a_finished_jobs_result_is_answered_from_disk_with_no_daemon() {
    let scratch = harness::scratch("seat-disk");
    let base = scratch.0.clone();
    let (daemon, _pid) = start_daemon(&base);

    let out = acq(
        &base,
        &[
            "submit",
            "sleep",
            "--params",
            "{\"seconds\":0.1}",
            "--detach",
            "--json",
        ],
    );
    let id = sole_json(&out)["job_id"]
        .as_u64()
        .expect("a job id")
        .to_string();
    let live = sole_json(&acq(&base, &["result", &id, "--wait", "--json"]));
    assert!(acq(&base, &["daemon", "stop"]).status.success());
    drop(daemon);

    let out = acq(&base, &["result", &id, "--json"]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    assert_eq!(sole_json(&out), live, "the ledger's answer is the daemon's");

    let status = sole_json(&acq(&base, &["status", &id, "--json"]));
    assert_eq!(status["running"], false, "{status}");
    assert_eq!(status["persisted"]["state"], "done", "{status}");
    let said = stdout(&acq(&base, &["status", &id]));
    assert!(
        said.starts_with(&format!(
            "daemon is not running; job {id} as the ledger on disk records it"
        )),
        "{said}"
    );

    let out = acq(&base, &["result", "999"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    assert!(
        stderr(&out).contains("no job 999 on disk, and the daemon is not running"),
        "{out:?}"
    );

    // Nothing above started a daemon.
    let status = sole_json(&acq(&base, &["daemon", "status", "--json"]));
    assert_eq!(status["running"], false, "{status}");
}

/// `auth status` with no daemon was an error, "daemon is not running",
/// where the tour called the verb the local session: it could not say
/// what a daemon would start from. It answers from the index — which
/// accounts a job command will restore and which need a login — and
/// starts nothing.
#[test]
fn auth_status_with_no_daemon_says_what_a_daemon_would_restore() {
    let scratch = harness::scratch("seat-auth");
    let base = scratch.0.clone();
    seed_two_leagues(&base);

    let out = acq(&base, &["auth", "status"]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    let said = stdout(&out);
    assert!(
        said.starts_with("daemon is not running, so no session is live"),
        "{said}"
    );
    assert!(
        said.contains("Alice#1234: not persisted — `acq auth` logs in again"),
        "{said}"
    );

    let status = sole_json(&acq(&base, &["auth", "status", "--json"]));
    assert_eq!(status["running"], false, "{status}");
    assert_eq!(status["accounts"][0]["username"], "Alice#1234", "{status}");
    assert_eq!(status["accounts"][0]["persisted"], false, "{status}");

    let status = sole_json(&acq(&base, &["daemon", "status", "--json"]));
    assert_eq!(status["running"], false, "{status}");
}
