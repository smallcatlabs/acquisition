//! The faults the whole-CLI seat found (2026-10-13,
//! `audits/rehome-2026-10/seat/REPORT.md` at `7653f8b0`) and the trade
//! import's seat (2026-10-17, `audits/trade-2026-10/seat/`), each held by
//! the test named for it. A test here drives the real binary in a scratch
//! world against the mock; nothing reaches GGG.

use std::path::{Path, PathBuf};

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
        said.contains("next: `acq price show --account 'Alice#1234' tab/pc/b1` reads it"),
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

    // Nothing above started a daemon — and with none running the status
    // still says where the last one logged, which only a running daemon
    // used to print.
    let status = sole_json(&acq(&base, &["daemon", "status", "--json"]));
    assert_eq!(status["running"], false, "{status}");
    let log = status["log"].as_str().expect("the last daemon's log");
    assert!(
        std::path::Path::new(log).ends_with("mock/daemon.log"),
        "{status}"
    );
    assert!(stdout(&acq(&base, &["daemon", "status"])).contains(&format!("last log: {log}")));
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

/// "Everything in one league" took two rounds and nothing said so: a map
/// or unique tab's substashes can be planned only once the tab has been
/// fetched, and the first apply ended on its tally. An apply now ends by
/// compiling the policy again and saying what is still planned, as a
/// number in JSON and as the next action in text, until nothing is.
#[test]
fn an_apply_ends_by_saying_what_a_new_plan_would_still_fetch() {
    let scratch = harness::scratch("seat-rounds");
    let base = scratch.0.clone();
    let (_daemon, _pid) = start_daemon(&base);
    login(&base, "alice");
    let policy = r#"{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":"all","characters":"all","max_age_seconds":3600}}}}}"#;
    assert!(acq(&base, &["policy", "set", policy]).status.success());
    // The listings a plan compiles from.
    assert!(acq(&base, &["stashes"]).status.success());
    assert!(acq(&base, &["characters"]).status.success());

    let out = acq(&base, &["refresh", "--apply"]);
    assert!(out.status.success(), "{}", text(&out));
    let said = stdout(&out);
    let last = said.lines().last().unwrap_or_default();
    assert!(
        last.starts_with("next: `acq refresh --plan` — ") && last.contains(" more are planned now"),
        "{said}"
    );

    let outcome = sole_json(&acq(&base, &["refresh", "--apply", "--json"]));
    assert_eq!(outcome["outcome"], "success", "{outcome}");
    assert_eq!(outcome["remaining_requests"], 0, "{outcome}");
    assert_eq!(outcome["account"], "alice", "{outcome}");
    assert!(outcome["job_id"].is_u64(), "{outcome}");

    let said = stdout(&acq(&base, &["refresh", "--plan"]));
    assert!(said.contains("nothing to do"), "{said}");
}

/// `search --describe` was 117,523 bytes with no way to read it in parts
/// but by knowing an entry's name, and the names were listed only in the
/// error for a wrong one. Nine tenths of the page is the computed values'
/// and the categories' definitions. Bare, it is the index: every name
/// with one line, under what a host shows whole, ending on how to read
/// an entry; `all` is the page as it was, and JSON stays whole.
#[test]
fn describe_is_an_index_that_names_every_entry_and_says_how_to_read_one() {
    let scratch = harness::scratch("seat-describe");
    let base = scratch.0.clone();

    let index = stdout(&acq(&base, &["search", "--describe"]));
    assert!(index.len() < 50_000, "the index is {} bytes", index.len());
    let life = index
        .lines()
        .find(|l| l.trim_start().starts_with("pseudo.total_life "))
        .expect("the index names pseudo.total_life");
    assert!(life.ends_with("(`+# total maximum Life`)"), "{life}");
    assert!(
        index.contains("\n  accessory.ring "),
        "the index names a category"
    );
    let last = index.lines().last().unwrap_or_default();
    assert!(
        last.starts_with("more: this is the index") && last.contains("`--describe all`"),
        "{last}"
    );

    // An entry named is whole, and so is everything under `all` and JSON.
    let entry = stdout(&acq(&base, &["search", "--describe", "pseudo.total_life"]));
    assert!(
        entry.contains("the sum over the item's lines of"),
        "{entry}"
    );
    let all = stdout(&acq(&base, &["search", "--describe", "all"]));
    assert!(all.len() > 100_000 && all.contains("the sum over the item's lines of"));
    assert!(!all.contains("more: this is the index"));
    let whole = sole_json(&acq(&base, &["search", "--describe", "--json"]));
    let defined = whole["computed"][0]["what"].as_str().unwrap_or_default();
    assert!(
        defined.contains("the sum over the item's lines of"),
        "{defined}"
    );
}

/// `acq jobs` listed every finished job of the daemon's lifetime above
/// the ones still open, 48 rows where the question was what waits. It
/// lists the open jobs and the ten most recent finished, counts all of
/// them on its last line, and says how many it left out and how to see
/// them; JSON is every job, as before.
#[test]
fn jobs_lists_what_is_open_and_counts_the_finished_it_leaves_out() {
    let scratch = harness::scratch("seat-jobs");
    let base = scratch.0.clone();
    let (_daemon, _pid) = start_daemon(&base);

    let mut last = String::new();
    for _ in 0..12 {
        let out = acq(
            &base,
            &[
                "submit",
                "sleep",
                "--params",
                "{\"seconds\":0}",
                "--detach",
                "--json",
            ],
        );
        last = sole_json(&out)["job_id"]
            .as_u64()
            .expect("a job id")
            .to_string();
    }
    for id in 1..=12 {
        let out = acq(&base, &["result", &id.to_string(), "--wait", "--json"]);
        assert!(out.status.success(), "{out:?}");
    }
    assert_eq!(last, "12");

    let said = stdout(&acq(&base, &["jobs"]));
    let rows = said.lines().filter(|l| l.contains(" sleep ")).count();
    assert_eq!(rows, 10, "{said}");
    assert!(
        !said.contains("\n 1  "),
        "the earliest is left out:\n{said}"
    );
    assert!(
        said.contains("12 jobs: 0 running, 0 waiting, 12 done"),
        "{said}"
    );
    assert!(
        said.ends_with("2 earlier finished jobs not listed (`acq jobs --all`)\n"),
        "{said}"
    );

    let said = stdout(&acq(&base, &["jobs", "--all"]));
    assert_eq!(
        said.lines().filter(|l| l.contains(" sleep ")).count(),
        12,
        "{said}"
    );
    assert!(!said.contains("not listed"), "{said}");

    let all = sole_json(&acq(&base, &["jobs", "--json"]));
    assert_eq!(all.as_array().map(Vec::len), Some(12), "{all}");
}

/// `daemon stop` said one line, and the next job command said "not
/// logged in": a session the keyring does not hold lives in the daemon
/// alone, so stopping it is a logout, and nothing said so. The stop names
/// the accounts it logged out.
#[test]
fn stopping_a_daemon_says_which_logins_ended_with_it() {
    let scratch = harness::scratch("seat-stop");
    let base = scratch.0.clone();
    let (daemon, _pid) = start_daemon(&base);
    login(&base, "alice");

    let out = acq(&base, &["daemon", "stop"]);
    let said = stdout(&out);
    assert!(out.status.success(), "{}", text(&out));
    assert!(
        said.contains("logged out with it: alice — not in the keyring"),
        "{said}"
    );
    drop(daemon);

    // With no session there is nothing to say.
    let (_daemon, _pid) = start_daemon(&base);
    let stopped = sole_json(&acq(&base, &["daemon", "stop", "--json"]));
    assert_eq!(stopped["sessions_ended"], json!([]), "{stopped}");
}

/// `acq tabs` printed every tab of the league with its only summary on
/// the last line: 355 KB for the owner's Standard, which two seats met —
/// the first turned away, the second filled its context. The counts are
/// the first line, and a league of more than thirty tabs is counted by
/// type with its ten largest; `--expand` is every tab. `--json` is the
/// same document (C115): its tabs are `rows`, cut the same way, every one
/// under `--expand` — the refresh-run script reads them there (the review
/// of note 38 at `16bba23e`, step 1, reproduced its abort on the old flat list).
#[test]
fn tabs_says_its_counts_first_and_counts_a_large_league() {
    let scratch = harness::scratch("seat-tabs");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let said = stdout(&acq(&base, &["tabs"]));
    assert!(
        said.starts_with("1 tabs in Standard: 1 fetched; 1 items\n"),
        "{said}"
    );
    assert!(
        said.contains("a1"),
        "a small league is listed whole:\n{said}"
    );

    // Forty more tabs listed and never fetched.
    let mock = base.join("store").join("mock");
    let mut store = Store::open(&account_path(&mock, "Alice#1234")).unwrap();
    let stashes: Vec<Value> = (0..41)
        .map(|i| {
            if i == 0 {
                json!({ "id": "a1", "name": "Gear", "type": "PremiumStash", "index": 0 })
            } else {
                json!({ "id": format!("q{i}"), "name": format!("Dump {i}"), "type": "QuadStash", "index": i })
            }
        })
        .collect();
    store
        .record(
            &Endpoint::Stashes {
                realm: "pc".into(),
                league: "Standard".into(),
            },
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": stashes }),
            acquisition_store::now(),
        )
        .unwrap();
    drop(store);

    let said = stdout(&acq(&base, &["tabs"]));
    let lines: Vec<&str> = said.lines().collect();
    assert_eq!(
        lines[0], "41 tabs in Standard: 1 fetched, 40 never fetched; 1 items",
        "{said}"
    );
    assert!(
        lines
            .iter()
            .any(|l| l.starts_with("QuadStash") && l.contains("40")),
        "{said}"
    );
    assert!(
        lines.len() < 25,
        "a large league is counted, never listed:\n{said}"
    );
    assert!(
        lines
            .last()
            .unwrap()
            .starts_with("next: `acq tabs --expand` lists all 41"),
        "{said}"
    );
    let all = stdout(&acq(&base, &["tabs", "--expand"]));
    assert!(all.contains("q40") && all.lines().count() > 41, "{all}");
    let cut = sole_json(&acq(&base, &["tabs", "--json"]));
    assert_eq!(cut["tabs"], 41, "{cut}");
    assert_eq!(cut["rows"].as_array().unwrap().len(), 10, "{cut}");
    assert_eq!(cut["left_out"], 31, "{cut}");
    assert_eq!(cut["left_out_needs"], "expand", "{cut}");
    let whole = sole_json(&acq(&base, &["tabs", "--json", "--expand"]));
    let rows = whole["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 41, "{whole}");
    assert_eq!(whole["left_out"], 0, "{whole}");
    let q40 = rows.iter().find(|r| r["id"] == "q40").unwrap();
    for field in ["id", "name", "type", "item_count", "parent", "fetched_at"] {
        assert!(q40.get(field).is_some(), "{field} missing: {q40}");
    }
    assert_eq!(q40["type"], "QuadStash", "{q40}");
    assert!(
        q40["fetched_at"].is_null() && q40["parent"].is_null(),
        "{q40}"
    );

    // The next commands keep the league shown: they named Standard's
    // defaults whatever league the summary was of.
    let mut store = Store::open(&account_path(&mock, "Alice#1234")).unwrap();
    store
        .record(
            &Endpoint::Stashes {
                realm: "pc".into(),
                league: "Hard Mode".into(),
            },
            &json!({ "league": "Hard Mode" }),
            200,
            &json!({ "stashes": stashes }),
            acquisition_store::now(),
        )
        .unwrap();
    drop(store);
    let said = stdout(&acq(&base, &["tabs", "--league", "Hard Mode"]));
    let next = said.lines().last().unwrap_or_default();
    assert!(
        next.contains("`acq tabs --expand --league 'Hard Mode'` lists all 41"),
        "{said}"
    );
    assert!(
        next.contains(r#"`acq search --realm pc 'league="Hard Mode"' --count tab`"#),
        "{said}"
    );
    let all = stdout(&acq(&base, &["tabs", "--expand", "--league", "Hard Mode"]));
    assert!(all.contains("q40") && all.lines().count() > 41, "{all}");
    let counted = acq(
        &base,
        &[
            "search",
            "--realm",
            "pc",
            r#"league="Hard Mode""#,
            "--count",
            "tab",
        ],
    );
    assert!(counted.status.success(), "{}", text(&counted));
}

/// Every run of the re-sit, told "0 matches" for a unique the person
/// owns, could say that 437 of 521 unique-stash tabs were never fetched
/// and not which — `tabs` listed a league whole or its ten largest, with
/// no selection (cli-opus calls 36–39, mcp-opus 34–37, cli-sonnet 20–22,
/// mcp-sonnet 18–23; the search area's park, fired). `--type` and
/// `--never-fetched` select the rows under the league's counts, the
/// same cut and `--expand` listing the rest; the scope line's pointer
/// lists the never fetched.
#[test]
fn tabs_lists_the_never_fetched_of_one_type() {
    let scratch = harness::scratch("seat-tabs-never");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let mock = base.join("store").join("mock");
    let mut store = Store::open(&account_path(&mock, "Alice#1234")).unwrap();
    let mut stashes: Vec<Value> =
        vec![json!({ "id": "a1", "name": "Gear", "type": "PremiumStash", "index": 0 })];
    stashes.extend((1..=40).map(
        |i| json!({ "id": format!("u{i}"), "name": format!("Uniques {i}"), "type": "UniqueStash", "index": i }),
    ));
    stashes.push(json!({ "id": "f1", "name": "Folder", "type": "Folder", "index": 41 }));
    store
        .record(
            &Endpoint::Stashes {
                realm: "pc".into(),
                league: "Standard".into(),
            },
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": stashes }),
            acquisition_store::now(),
        )
        .unwrap();
    drop(store);
    let cut = sole_json(&acq(
        &base,
        &["tabs", "--json", "--type", "uniquestash", "--never-fetched"],
    ));
    // the league's counts stay the league's
    assert_eq!(cut["tabs"], 42, "{cut}");
    assert_eq!(cut["never_fetched"], 40, "{cut}");
    assert_eq!(
        cut["selection"],
        json!({ "type": "uniquestash", "never_fetched": true }),
        "{cut}"
    );
    assert_eq!(cut["selected"], 40, "{cut}");
    assert_eq!(cut["rows"].as_array().unwrap().len(), 10, "{cut}");
    assert_eq!(cut["left_out"], 30, "{cut}");
    assert_eq!(cut["left_out_needs"], "expand", "{cut}");
    let all = sole_json(&acq(
        &base,
        &[
            "tabs",
            "--json",
            "--type",
            "UniqueStash",
            "--never-fetched",
            "--expand",
        ],
    ));
    let rows = all["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 40, "{all}");
    assert!(
        rows.iter()
            .all(|r| r["type"] == "UniqueStash" && r["fetched_at"].is_null()),
        "{all}"
    );
    assert_eq!(all["left_out"], 0, "{all}");
    // a folder is never fetched and holds nothing: not a tab to refresh
    let never = sole_json(&acq(&base, &["tabs", "--json", "--never-fetched"]));
    assert_eq!(never["selected"], 40, "{never}");
    let none = sole_json(&acq(
        &base,
        &[
            "tabs",
            "--json",
            "--type",
            "PremiumStash",
            "--never-fetched",
        ],
    ));
    assert_eq!(none["selected"], 0, "{none}");
    assert_eq!(none["rows"], json!([]), "{none}");
    // the text: the league's counts, the selection in words, the rows,
    // and what lists the rest
    let said = stdout(&acq(
        &base,
        &["tabs", "--type", "UniqueStash", "--never-fetched"],
    ));
    let lines: Vec<&str> = said.lines().collect();
    assert_eq!(
        lines[0],
        "42 tabs in Standard: 1 fetched, 40 never fetched, 1 folder (never fetched); 1 items",
        "{said}"
    );
    assert_eq!(lines[1], "40 never-fetched UniqueStash tabs", "{said}");
    assert!(
        lines.last().unwrap()
            == &"10 of 40 shown: `acq tabs --expand --type UniqueStash --never-fetched` lists all",
        "{said}"
    );
    let all = stdout(&acq(
        &base,
        &[
            "tabs",
            "--type",
            "UniqueStash",
            "--never-fetched",
            "--expand",
        ],
    ));
    assert!(all.contains("u40") && !all.contains("shown:"), "{all}");
}

/// The scope line said "location list seen 41d ago" under a
/// `league=Standard` query whose own list was 3 days old: the age was
/// the realm's oldest list, another league's (the re-sit, mcp-opus
/// fault 2, read against `tabs` and `refresh --plan` and never
/// resolved). Under a query that fixes one league the age is that
/// league's list and the characters'; with none fixed, the realm's
/// oldest, as before.
#[test]
fn the_scope_lines_list_age_is_the_querys_leagues() {
    let scratch = harness::scratch("seat-list-age");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let mock = base.join("store").join("mock");
    let mut store = Store::open(&account_path(&mock, "Alice#1234")).unwrap();
    let later = acquisition_store::now() + 600;
    store
        .record(
            &Endpoint::Stashes {
                realm: "pc".into(),
                league: "Standard".into(),
            },
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": [ { "id": "a1", "name": "Gear", "type": "PremiumStash", "index": 0 } ] }),
            later,
        )
        .unwrap();
    drop(store);
    let fixed = sole_json(&acq(
        &base,
        &["search", "--realm", "pc", "--json", "league=Standard"],
    ));
    assert_eq!(fixed["scope"]["list_seen"], later, "{}", fixed["scope"]);
    // a refinement parenthesizes the old text and appends a term, and
    // fixes the same league (review 2 of the re-sit)
    let refined = sole_json(&acq(
        &base,
        &[
            "search",
            "--realm",
            "pc",
            "--json",
            "(league=Standard rarity=rare) ilvl>=1",
        ],
    ));
    assert_eq!(refined["scope"]["list_seen"], later, "{}", refined["scope"]);
    let whole = sole_json(&acq(&base, &["search", "--realm", "pc", "--json"]));
    let oldest = whole["scope"]["list_seen"].as_i64().unwrap();
    assert!(oldest < later, "{}", whole["scope"]);
    // a league no list names keeps the realm's age, never none
    let unknown = sole_json(&acq(
        &base,
        &["search", "--realm", "pc", "--json", "league=Nowhere"],
    ));
    assert_eq!(
        unknown["scope"]["list_seen"], oldest,
        "{}",
        unknown["scope"]
    );
}

/// `show` printed `price   none (none)` for an unpriced item — the
/// listing state's kind beside its status — which read as two empty
/// fields until `price show` explained it (the re-sit, cli-opus call 41,
/// its fault 7). The line says why none, in the words its JSON carries.
#[test]
fn shows_price_line_says_why_none() {
    let scratch = harness::scratch("seat-price-why");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let shown = stdout(&acq(&base, &["show", "i-std"]));
    let line = shown
        .lines()
        .find(|l| l.starts_with("price   "))
        .unwrap_or_else(|| panic!("{shown}"));
    assert!(
        line.starts_with("price   none (nothing applies) · why: acq price show "),
        "{line}"
    );
    assert!(!line.contains("(none)"), "{line}");
    let doc = sole_json(&acq(&base, &["show", "--json", "i-std"]));
    assert_eq!(doc["price"]["why"], "nothing applies", "{}", doc["price"]);
}

/// A daemon with `alice` logged in, a Standard policy over every tab and
/// the stash listing on record; the plan compiled from it is written to
/// `plan.json` under `base` and returned beside its path.
fn a_saved_plan(base: &Path) -> (String, Value) {
    login(base, "alice");
    let policy = r#"{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":"all","max_age_seconds":3600}}}}}"#;
    assert!(acq(base, &["policy", "set", policy]).status.success());
    assert!(acq(base, &["stashes"]).status.success());
    let out = acq(base, &["refresh", "--plan", "--json"]);
    let plan = sole_json(&out);
    let path = base.join("plan.json");
    std::fs::write(&path, &out.stdout).unwrap();
    (format!("--apply={}", path.display()), plan)
}

/// C111. A plan file applied twice fetched twice: the same command run
/// again spent seven requests on tabs seconds fresh, and the limiter held
/// the next round 292 s. A plan spends each request once, so the second
/// apply sends nothing, says what spent the plan, and is no failure.
#[test]
fn a_plan_file_applied_twice_sends_nothing_the_second_time() {
    let scratch = harness::scratch("seat-replay");
    let base = scratch.0.clone();
    let (_daemon, _pid) = start_daemon(&base);
    let (apply, plan) = a_saved_plan(&base);
    let planned = plan["logical_requests"].as_u64().expect("a count");
    assert!(planned > 1, "{plan}");

    let first = sole_json(&acq(&base, &["refresh", &apply, "--json"]));
    assert_eq!(first["outcome"], "success", "{first}");
    assert_eq!(first["payload"]["requests"], planned, "{first}");
    assert_eq!(first["spent"], json!([]), "{first}");

    let out = acq(&base, &["refresh", &apply, "--json"]);
    assert!(out.status.success(), "{}", text(&out));
    let second = sole_json(&out);
    assert_eq!(second["applied"], false, "{second}");
    assert_eq!(second["requests"], 0, "{second}");
    assert!(second.get("job_id").is_none(), "{second}");
    let spent = second["spent"].as_array().expect("a list");
    assert_eq!(spent.len() as u64, planned, "{second}");
    for (entry, action) in spent.iter().zip(plan["actions"].as_array().unwrap()) {
        assert_eq!(&entry["action"], action, "{second}");
        assert!(entry["fetched_at"].is_i64(), "{second}");
    }

    let out = acq(&base, &["refresh", &apply]);
    assert!(out.status.success(), "{}", text(&out));
    let said = stdout(&out);
    assert!(
        said.starts_with(&format!(
            "nothing to do: every request of this plan is spent — {planned} fetched after it was compiled, the latest "
        )),
        "{said}"
    );

    // One apply's worth of fetches is all the daemon ever ran.
    let jobs = sole_json(&acq(&base, &["jobs", "--all", "--json"]));
    let jobs = jobs.as_array().expect("a list of jobs");
    let applies = jobs.iter().filter(|j| j["kind"] == "apply").count();
    assert_eq!(applies, 1, "{jobs:?}");
}

/// C111, across doors: a fetch `acq stash` queued spends the plan's
/// action for that tab. The verb spells its request with `"sub": null`
/// where a plan's omits `sub`, and the guard compared the two as JSON,
/// so an apply behind a waiting `acq stash` queued the tab a second time.
#[test]
fn an_apply_behind_a_waiting_stash_fetch_does_not_queue_it_again() {
    let scratch = harness::scratch("seat-door");
    let base = scratch.0.clone();
    let (_daemon, _pid) = start_daemon(&base);
    let (apply, plan) = a_saved_plan(&base);
    let tab = plan["actions"][0]["id"]
        .as_str()
        .expect("a tab id")
        .to_string();
    let other = plan["actions"][1]["id"].as_str().expect("a second tab");

    // Spend the stash route's burst so the next fetches wait — on
    // another tab, which spends that tab's action and not this one's.
    let params = json!({ "realm": "pc", "league": "Standard", "id": other, "deep": false });
    for _ in 0..20 {
        let out = acq(
            &base,
            &[
                "submit",
                "stash",
                "--params",
                &params.to_string(),
                "--detach",
                "--json",
            ],
        );
        assert!(out.status.success(), "{}", text(&out));
    }
    let open_jobs = || {
        sole_json(&acq(&base, &["jobs", "--all", "--json"]))
            .as_array()
            .expect("a list of jobs")
            .clone()
    };
    // Run `args` until `found` lists its job, then leave its client:
    // the job stays.
    let held = |args: &[&str], found: &dyn Fn(&[Value]) -> Option<Value>| {
        let mut child = harness::command(&base, args)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("spawning acq");
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        let job = loop {
            if let Some(job) = found(&open_jobs()) {
                break job;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "never listed: {args:?}"
            );
            std::thread::sleep(std::time::Duration::from_millis(50));
        };
        let _ = child.kill();
        let _ = child.wait();
        job
    };

    held(&["stash", &tab], &|jobs| {
        jobs.iter()
            .find(|j| j["kind"] == "stash" && j["params"].get("sub") == Some(&Value::Null))
            .cloned()
    });

    let applied = held(&["refresh", &apply, "--json"], &|jobs| {
        jobs.iter().find(|j| j["kind"] == "apply").cloned()
    });
    let sent: Vec<&Value> = applied["params"]["jobs"]
        .as_array()
        .expect("the apply's requests")
        .iter()
        .map(|job| &job["params"]["id"])
        .collect();
    assert!(
        !sent.contains(&&json!(tab)),
        "the apply queued the tab `acq stash` holds: {applied}"
    );
    assert_eq!(
        sent.len() + 2,
        plan["actions"].as_array().map_or(0, Vec::len),
        "{applied}"
    );
}

/// C111, the retry: an apply whose caller gave up while its fetches
/// waited on the limiter is run again. The first apply's requests are
/// still queued, so the store shows nothing fetched; the daemon's own
/// list is what says they are spent, and the second apply names the job
/// to wait for instead of queueing every request a second time.
#[test]
fn an_apply_retried_while_its_requests_wait_queues_nothing_more() {
    let scratch = harness::scratch("seat-retry");
    let base = scratch.0.clone();
    let (_daemon, _pid) = start_daemon(&base);
    let (apply, plan) = a_saved_plan(&base);
    let planned = plan["logical_requests"].as_u64().expect("a count");

    // Spend the stash route's burst so the apply's fetches wait.
    let tab = plan["actions"][0]["id"].as_str().expect("a tab id");
    let params = json!({ "realm": "pc", "league": "Standard", "id": tab, "deep": false });
    for _ in 0..20 {
        let out = acq(
            &base,
            &[
                "submit",
                "stash",
                "--params",
                &params.to_string(),
                "--detach",
                "--json",
            ],
        );
        assert!(out.status.success(), "{}", text(&out));
    }

    // The caller that gives up: the apply is started and never read.
    let mut first = harness::command(&base, &["refresh", &apply, "--json"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawning acq");
    let open_jobs = || {
        sole_json(&acq(&base, &["jobs", "--all", "--json"]))
            .as_array()
            .expect("a list of jobs")
            .clone()
    };
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    let parent = loop {
        let jobs = open_jobs();
        let fanned_out = jobs.iter().find(|j| j["kind"] == "apply").filter(|apply| {
            let children = jobs.iter().filter(|j| j["parent"] == apply["id"]).count();
            children
                == apply["params"]["jobs"]
                    .as_array()
                    .map_or(usize::MAX, Vec::len)
        });
        if let Some(apply) = fanned_out {
            break apply["id"].as_u64().expect("an id");
        }
        assert!(
            std::time::Instant::now() < deadline,
            "the apply never fanned out: {jobs:?}"
        );
        std::thread::sleep(std::time::Duration::from_millis(50));
    };
    let _ = first.kill();
    let _ = first.wait();

    let out = acq(&base, &["refresh", &apply, "--json"]);
    assert!(out.status.success(), "{}", text(&out));
    let second = sole_json(&out);
    assert_eq!(second["applied"], false, "{second}");
    let spent = second["spent"].as_array().expect("a list");
    assert_eq!(spent.len() as u64, planned, "{second}");
    let queued: Vec<&Value> = spent.iter().filter(|s| s["job_id"].is_u64()).collect();
    assert!(!queued.is_empty(), "nothing was still waiting: {second}");
    assert!(queued.iter().any(|s| s["parent"] == parent), "{second}");
    assert!(second["remaining_requests"].is_null(), "{second}");
    assert!(
        second["note"]
            .as_str()
            .unwrap_or_default()
            .contains(&format!("(`acq result {parent} --wait`)")),
        "{second}"
    );
    let applies = open_jobs().iter().filter(|j| j["kind"] == "apply").count();
    assert_eq!(applies, 1, "the retry queued a second apply");
}

/// A reader took `exact` for "the site would show me the same": the
/// translation's fidelity is how far the text is the site's search, and
/// the items are the store's, worn and stashed, priced or not, where the
/// site shows public listings (the trade seat: mcp-opus calls 1, 11, its
/// worst fault; cli-sonnet's debrief, answer 4). Every translation says
/// so under its first line, an exact one too, and each clause with no
/// stash meaning is named with its note, not by its key alone (cli-sonnet
/// calls 6, 8).
#[test]
fn a_trade_answer_says_its_items_are_the_stores_whatever_its_fidelity() {
    let scratch = harness::scratch("seat-trade-over");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let mut child = harness::command(&base, &["search", "--realm", "pc", "--trade-file", "-"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    use std::io::Write as _;
    child
        .stdin
        .take()
        .unwrap()
        .write_all(br#"{"query": {"status": {"option": "online"}, "type": "Chaos Orb"}}"#)
        .unwrap();
    let out = child.wait_with_output().unwrap();
    let shown = stdout(&out);
    let lines: Vec<&str> = shown.lines().collect();
    assert!(
        lines[0].starts_with("trade   from the query JSON: exact"),
        "{shown}"
    );
    assert_eq!(
        lines[1],
        "        over      the items the store has fetched, priced or not — what it never fetched is \
         the scope's: the site shows public listings, a tab's being tab.public, so the search is the \
         site's and the items are yours",
        "{shown}"
    );
    assert!(
        lines[2].starts_with(
            "        reported  status  Online, available or securable is the seller's state"
        ),
        "{shown}"
    );
}

/// The never-fetched were named under a zero total alone, so an answer
/// of N said nothing of what it could not see (the trade seat: cli-sonnet
/// call 6, its fault 4). Every total over a scope with a never-fetched
/// location says so.
#[test]
fn every_total_says_what_was_never_fetched_not_a_zeros_alone() {
    let scratch = harness::scratch("seat-never");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    // a third tab, listed and never fetched
    let mock = base.join("store").join("mock");
    let mut store = Store::open(&account_path(&mock, "Alice#1234")).unwrap();
    store
        .record(
            &Endpoint::Stashes {
                realm: "pc".into(),
                league: "Standard".into(),
            },
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": [ { "id": "a1", "name": "Gear", "type": "PremiumStash", "index": 0 },
                                  { "id": "a2", "name": "Never", "type": "PremiumStash", "index": 1 } ] }),
            acquisition_store::now(),
        )
        .unwrap();
    drop(store);
    for (query, total) in [
        ("league=Standard", "total   1 match"),
        ("league=Nowhere", "total   0 matches"),
    ] {
        let shown = stdout(&acq(&base, &["search", "--realm", "pc", query]));
        let lines: Vec<&str> = shown.lines().collect();
        let at = lines
            .iter()
            .position(|l| *l == total)
            .unwrap_or_else(|| panic!("{shown}"));
        assert_eq!(
            lines[at + 1],
            "            1 location was never fetched: nothing here says what they hold",
            "{shown}"
        );
    }
}

/// A map search printed the flat "2940 locations never fetched" a ring
/// search printed, beside a map tab 61 of 2,550 fetched, and pointed at
/// `--view locations`, which is not built (the trade seat: cli-opus-2
/// calls 4, 40, its fault 3; cli-sonnet-2 calls 3, 6, its fault 1). C118:
/// the scope line names the types with the most never fetched, counts
/// the rest, and points at the list that exists.
#[test]
fn the_scope_line_names_the_never_fetched_by_tab_type_and_points_at_tabs() {
    let scratch = harness::scratch("seat-by-type");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let mock = base.join("store").join("mock");
    let mut store = Store::open(&account_path(&mock, "Alice#1234")).unwrap();
    store
        .record(
            &Endpoint::Stashes {
                realm: "pc".into(),
                league: "Standard".into(),
            },
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": [ { "id": "a1", "name": "Gear", "type": "PremiumStash", "index": 0 },
                                  { "id": "m1", "name": "Maps", "type": "MapStash", "index": 1 },
                                  { "id": "m2", "name": "More maps", "type": "MapStash", "index": 2 },
                                  { "id": "m3", "name": "Old maps", "type": "MapStash", "index": 3 },
                                  { "id": "u1", "name": "Uniques", "type": "UniqueStash", "index": 4 },
                                  { "id": "u2", "name": "More uniques", "type": "UniqueStash", "index": 5 },
                                  { "id": "g1", "name": "Gems", "type": "GemStash", "index": 6 },
                                  { "id": "c1", "name": "Coin", "type": "CurrencyStash", "index": 7 },
                                  { "id": "e1", "name": "Essence", "type": "EssenceStash", "index": 8 } ] }),
            acquisition_store::now(),
        )
        .unwrap();
    drop(store);
    let shown = stdout(&acq(&base, &["search", "--realm", "pc", "league=Standard"]));
    let line = shown
        .lines()
        .find(|l| l.contains("never fetched:"))
        .unwrap_or_else(|| panic!("{shown}"));
    assert!(
        line.starts_with(
            "        8 locations never fetched: MapStash 3 of 3, UniqueStash 2 of 2, CurrencyStash 1 of 1, 2 more in 2 other types · location list seen "
        ),
        "{shown}"
    );
    assert!(
        line.ends_with(" · each tab: acq tabs --account 'Alice#1234' --realm pc --league Standard --never-fetched"),
        "{shown}"
    );
    // the pointer runs as printed, and lists the never fetched
    let out = acq(
        &base,
        &[
            "tabs",
            "--account",
            "Alice#1234",
            "--realm",
            "pc",
            "--never-fetched",
        ],
    );
    assert!(out.status.success(), "{}", stdout(&out));
    assert!(!shown.contains("--view locations"), "{shown}");
}

/// The scope line's `tabs` pointer named no league, so it listed
/// Standard while the never fetched the scope counted sat in another
/// league, and a reader who followed it found none of them (the external
/// review of step 2). C118: the pointer names the league the query fixes,
/// or, where it fixes none, the league holding the most never-fetched
/// tabs, with the others counted; run as printed, it lists them.
#[test]
fn the_tabs_pointer_names_the_league_its_never_fetched_tabs_are_in() {
    let scratch = harness::scratch("seat-pointer-league");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let mock = base.join("store").join("mock");
    let mut store = Store::open(&account_path(&mock, "Alice#1234")).unwrap();
    for (league, tabs) in [
        (
            "Standard",
            json!([ { "id": "a1", "name": "Gear", "type": "PremiumStash", "index": 0 },
                             { "id": "a2", "name": "Never here", "type": "PremiumStash", "index": 1 } ]),
        ),
        (
            "Allflame",
            json!([ { "id": "b1", "name": "Gear", "type": "PremiumStash", "index": 0 },
                             { "id": "b2", "name": "Maps", "type": "MapStash", "index": 1 },
                             { "id": "b3", "name": "Uniques", "type": "UniqueStash", "index": 2 } ]),
        ),
    ] {
        store
            .record(
                &Endpoint::Stashes {
                    realm: "pc".into(),
                    league: league.into(),
                },
                &json!({ "league": league }),
                200,
                &json!({ "stashes": tabs }),
                acquisition_store::now(),
            )
            .unwrap();
    }
    drop(store);
    let pointer = |query: &str| {
        let shown = stdout(&acq(&base, &["search", "--realm", "pc", query]));
        let line = shown
            .lines()
            .find(|l| l.contains("never fetched:"))
            .unwrap_or_else(|| panic!("{shown}"))
            .to_string();
        line[line
            .find(" · each tab: ")
            .unwrap_or_else(|| panic!("{line}"))..]
            .to_string()
    };
    const ALLFLAME: &str =
        " · each tab: acq tabs --account 'Alice#1234' --realm pc --league Allflame --never-fetched";
    assert_eq!(pointer("league=Allflame"), ALLFLAME);
    assert_eq!(
        pointer("league=Standard"),
        " · each tab: acq tabs --account 'Alice#1234' --realm pc --league Standard --never-fetched"
    );
    // no league fixed: the league holding the most, the others counted
    assert_eq!(
        pointer("rarity=normal"),
        format!("{ALLFLAME} · 1 other league holds never-fetched tabs")
    );
    // run as printed, the command lists the never-fetched tabs it points at
    let args: Vec<String> = ALLFLAME
        .trim_start_matches(" · each tab: acq ")
        .split(' ')
        .map(|w| w.trim_matches('\'').to_string())
        .collect();
    let args: Vec<&str> = ["--json"]
        .into_iter()
        .chain(args.iter().map(String::as_str))
        .collect();
    let listed = sole_json(&acq(&base, &args));
    let never: Vec<&str> = listed["rows"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|t| t["fetched_at"].is_null())
        .map(|t| t["id"].as_str().unwrap())
        .collect();
    assert_eq!(never, ["b2", "b3"], "{listed}");
}

/// A term's counts were read as the answer's: "26 matched" for the
/// totem line where the answer held one shield, a term's 168 beside a
/// total of 160 (the trade seat: cli-opus calls 18, 19, its fault 8;
/// cli-sonnet call 14, its fault 5). The terms block says each term is
/// counted alone over the scope and never over the matches.
#[test]
fn the_terms_block_says_it_is_never_over_the_matches() {
    let scratch = harness::scratch("seat-terms");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let shown = stdout(&acq(&base, &["search", "--realm", "pc", "league=Standard"]));
    assert!(
        shown.lines().any(|l| l
            == "terms   each term counted alone over all live pc items in all leagues, never over the matches"),
        "{shown}"
    );
}

/// A row sorted by a sum said `sorts by 78` beside the term's explicit
/// sum of 70, and both CLI runners took the order for the filter's (the
/// trade seat: cli-opus call 21, its fault 2; cli-sonnet call 15, its
/// fault 1). The row names the sort's own value before its number, so
/// the sum over every source and the term's over the explicit stand
/// apart.
#[test]
fn a_row_sorted_by_a_value_no_field_names_says_which_value() {
    // the help says what a sum spans (the re-sit: cli-opus fault 4,
    // cli-sonnet fault 2, each sorting by `sum("T")` for the filter's
    // explicit figure)
    let help = stdout(&acq(
        &harness::scratch("seat-sort-help").0,
        &["search", "--help"],
    ));
    assert!(
        help.contains("A sum spans every source the line has"),
        "{help}"
    );
    assert!(help.contains("source=explicit).arg1)'"), "{help}");
    let scratch = harness::scratch("seat-sort");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let mock = base.join("store").join("mock");
    let mut store = Store::open(&account_path(&mock, "Alice#1234")).unwrap();
    store
        .record(
            &Endpoint::Stash {
                realm: "pc".into(),
                league: "Standard".into(),
                id: "a1".into(),
                sub: None,
            },
            &json!({}),
            200,
            &json!({ "stash": { "id": "a1", "name": "Gear", "type": "PremiumStash", "items": [
                { "id": "ring", "name": "Doom Loop", "typeLine": "Two-Stone Ring", "baseType": "Two-Stone Ring",
                  "rarity": "Rare", "frameTypeId": 2, "identified": true, "ilvl": 84, "x": 0, "y": 0,
                  "implicitMods": ["+8 to maximum Life"], "explicitMods": ["+70 to maximum Life"] } ] } }),
            acquisition_store::now(),
        )
        .unwrap();
    drop(store);
    let shown = stdout(&acq(
        &base,
        &[
            "search",
            "--realm",
            "pc",
            r##"sum(line("# to maximum Life" source=explicit).arg1)>=30"##,
            "--sort",
            r##"sum("# to maximum Life")"##,
            "--desc",
        ],
    ));
    assert!(
        shown.contains(r##"sorts by sum(line("# to maximum Life").arg1) 78"##),
        "{shown}"
    );
    assert!(
        shown.contains(r##"sum(line("# to maximum Life" source=explicit).arg1) 70"##),
        "{shown}"
    );
}

/// A term's path as long as its column ran into the term:
/// `0.3.0pseudo.total_dex>=20` (the trade seat: cli-opus call 13, its
/// fault 9). The column is as wide as the longest path and a space.
#[test]
fn a_long_term_path_never_runs_into_its_term() {
    let scratch = harness::scratch("seat-path");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let shown = stdout(&acq(
        &base,
        &[
            "search",
            "--realm",
            "pc",
            "--",
            "(league=Standard -(stack>=1 ilvl>=1)) -has:tab",
        ],
    ));
    let paths: Vec<&str> = shown
        .lines()
        .skip_while(|l| !l.starts_with("terms   "))
        .skip(1)
        .take_while(|l| l.starts_with("  "))
        .collect();
    assert!(
        paths.iter().any(|l| l.starts_with("  0.1.0.0 stack>=1")),
        "{shown}"
    );
    for l in &paths {
        let path = l.trim_start().split(' ').next().unwrap();
        assert!(path.chars().all(|c| c.is_ascii_digit() || c == '.'), "{l}");
    }
}

/// A row named the tab and never the cell, which the person needs to
/// pick the item up (the trade seat: cli-opus call 30, its fault 3;
/// cli-sonnet call 23, its fault 2). The place's text ends with it.
#[test]
fn a_rows_place_says_which_cell_the_item_sits_in() {
    let scratch = harness::scratch("seat-cell");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let shown = stdout(&acq(&base, &["search", "--realm", "pc", "league=Standard"]));
    assert!(shown.contains("Standard / Gear · x 0, y 0"), "{shown}");
    let shown = stdout(&acq(&base, &["show", "i-std"]));
    assert!(shown.contains("Standard / Gear · x 0, y 0"), "{shown}");
}

/// A row and `show` said where an item sits and never how old that is,
/// while `tabs` printed every tab's age (the trade seat: cli-opus-2 call
/// 46, its fault 2, for "where do I pick it up"). C118: the place ends
/// with its location's fetch age, in the words `tabs` prints an age in.
#[test]
fn a_rows_place_says_how_long_ago_its_location_was_fetched() {
    let scratch = harness::scratch("seat-age");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    // a tab fetched three days ago
    let mock = base.join("store").join("mock");
    let mut store = Store::open(&account_path(&mock, "Alice#1234")).unwrap();
    let then = acquisition_store::now() - 3 * 86_400 - 60;
    store
        .record(
            &Endpoint::Stash {
                realm: "pc".into(),
                league: "Standard".into(),
                id: "a1".into(),
                sub: None,
            },
            &json!({}),
            200,
            &json!({ "stash": { "id": "a1", "name": "Gear", "type": "PremiumStash", "items": [
                { "id": "i-std", "name": "", "typeLine": "Chaos Orb", "baseType": "Chaos Orb", "x": 0, "y": 0 } ] } }),
            then,
        )
        .unwrap();
    drop(store);
    let shown = stdout(&acq(&base, &["search", "--realm", "pc", "league=Standard"]));
    assert!(
        shown.contains("Standard / Gear · x 0, y 0 · fetched 3d ago"),
        "{shown}"
    );
    let shown = stdout(&acq(&base, &["show", "i-std"]));
    assert!(
        shown.contains("Standard / Gear · x 0, y 0 · fetched 3d ago"),
        "{shown}"
    );
}

/// `price show` refused the bare id every other verb prints, and the
/// refusal named the grammar alone (the trade seat: cli-opus call 31,
/// its fault 6). An item's id and a character's are alike, so neither is
/// guessed: the refusal offers both, the id filled in.
#[test]
fn a_bare_id_as_a_price_target_is_offered_as_an_item_and_as_a_character() {
    let scratch = harness::scratch("seat-price-id");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let out = acq(&base, &["price", "show", "i-std"]);
    assert!(!out.status.success());
    let said = stderr(&out);
    assert!(
        said.contains("; as an item, item/i-std; as a character, character/i-std"),
        "{said}"
    );
}

/// `store status` printed `leagues 0` beside tabs in seven leagues, and
/// its tabs across every realm and league beside `tabs`'s count of one
/// (the trade seat: cli-opus calls 3, 5, its fault 7). The line says what
/// each counts: the account's league list, which this store never
/// fetched, and the tabs of every realm and league.
#[test]
fn store_status_says_what_its_league_and_tab_counts_count() {
    let scratch = harness::scratch("seat-status");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let shown = stdout(&acq(&base, &["store", "status"]));
    assert!(
        shown.contains("league list never fetched (`acq leagues`)  characters 0  tabs 2 (every realm and league)"),
        "{shown}"
    );
}

/// A unique tab's page with no name printed as its bare id after its tab,
/// and the second round's runners could not tell the id was the tab's
/// page (the trade seat: cli-opus-2 calls 29, 30, its fault 4; mcp-opus-2
/// call 29, its fault 4). The text says `page`.
#[test]
fn a_tabs_page_with_no_name_is_said_to_be_its_page() {
    let scratch = harness::scratch("seat-page");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let mock = base.join("store").join("mock");
    let mut store = Store::open(&account_path(&mock, "Alice#1234")).unwrap();
    let at = acquisition_store::now();
    let sub = |sub: Option<&str>| Endpoint::Stash {
        realm: "pc".into(),
        league: "Standard".into(),
        id: "a1".into(),
        sub: sub.map(str::to_string),
    };
    store
        .record(
            &sub(None),
            &json!({}),
            200,
            &json!({ "stash": { "id": "a1", "name": "Gear", "type": "UniqueStash", "items": [],
                                "children": [{ "id": "p1", "name": "", "type": "UniqueStash" }] } }),
            at,
        )
        .unwrap();
    store
        .record(
            &sub(Some("p1")),
            &json!({}),
            200,
            &json!({ "stash": { "id": "p1", "name": "", "type": "UniqueStash", "items": [
                { "id": "kaom", "name": "Kaom's Heart", "typeLine": "Glorious Plate", "baseType": "Glorious Plate",
                  "rarity": "Unique", "frameTypeId": 3, "identified": true, "x": 1, "y": 2 } ] } }),
            at,
        )
        .unwrap();
    drop(store);
    let shown = stdout(&acq(&base, &["search", "--realm", "pc", "rarity=unique"]));
    assert!(
        shown.contains("Standard / Gear / page p1 · x 1, y 2"),
        "{shown}"
    );
}

/// C115: the answer led with its basis and its terms block, and the
/// total stood below every term (the trade seat: mcp-opus-2, "I learned
/// to scroll for `total`"; three of four second-round runs re-read the
/// terms block). The text says the query, the scope and the total first,
/// then the rows and how to go on, and only then the terms, the routes and
/// the basis; the JSON's keys come in the same order.
#[test]
fn the_answer_leads_with_its_total_and_ends_with_its_basis() {
    let scratch = harness::scratch("seat-order");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let shown = stdout(&acq(
        &base,
        &["search", "--realm", "pc", "league=Standard", "--limit", "0"],
    ));
    let at = |head: &str| {
        shown
            .lines()
            .position(|l| l.starts_with(head))
            .unwrap_or_else(|| panic!("no `{head}` line:\n{shown}"))
    };
    let order = [
        at("query "),
        at("scope "),
        at("total "),
        at("more "),
        at("terms "),
        at("basis "),
    ];
    assert!(order.windows(2).all(|w| w[0] < w[1]), "{order:?}\n{shown}");
    assert_eq!(at("basis "), shown.lines().count() - 1, "{shown}");

    let printed = stdout(&acq(
        &base,
        &["search", "--realm", "pc", "league=Standard", "--json"],
    ));
    let key = |k: &str| {
        printed
            .find(&format!("\n  \"{k}\":"))
            .unwrap_or_else(|| panic!("no top-level `{k}`:\n{printed}"))
    };
    let keys = [
        key("query"),
        key("scope"),
        key("total"),
        key("rows"),
        key("view"),
        key("terms"),
        key("basis"),
    ];
    assert!(keys.windows(2).all(|w| w[0] < w[1]), "{keys:?}\n{printed}");
}

/// C115: `--routes` was the text's alone — the JSON held every count's
/// request whatever was asked, a third of an answer's bytes (the trade
/// seat: mcp-sonnet-1 call 8, 13,693 bytes for 17 numbers). The JSON
/// now carries what the text prints: the routes a next step hangs on,
/// unasked; every count's under `--routes`, which a continuation keeps
/// and which asks the same beside `--next`.
#[test]
fn a_count_holds_its_request_only_where_the_routes_are_asked() {
    let scratch = harness::scratch("seat-routes");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let query = "league=Standard or league=Allflame";
    let shown = stdout(&acq(&base, &["search", "--realm", "pc", query]));
    assert!(
        shown
            .lines()
            .any(|l| l == "routes  each returns exactly the members counted (4 more: --routes)"),
        "{shown}"
    );
    let plain = sole_json(&acq(
        &base,
        &["--json", "search", "--realm", "pc", query, "--limit", "1"],
    ));
    assert_eq!(plain["terms"][0]["failed"]["count"], 1);
    assert!(
        plain["terms"][0]["failed"].get("request").is_none(),
        "{plain}"
    );
    let asked = sole_json(&acq(
        &base,
        &[
            "--json", "search", "--realm", "pc", query, "--limit", "1", "--routes",
        ],
    ));
    assert!(
        asked["terms"][0]["failed"]["request"].is_object(),
        "{asked}"
    );
    assert_eq!(
        asked["view"]["rows"]["next"]["request"]["view"]["routes"],
        true
    );
    let shown = stdout(&acq(&base, &["search", "--realm", "pc", query, "--routes"]));
    assert!(
        shown
            .lines()
            .any(|l| l == "routes  each returns exactly the members counted"),
        "{shown}"
    );

    // beside a continuation, `--routes` asks them of the answer it runs
    let next: acquisition_search::answer::Request =
        serde_json::from_value(plain["view"]["rows"]["next"]["request"].clone()).unwrap();
    let token = &next.token();
    let next = sole_json(&acq(&base, &["--json", "search", "--next", token]));
    assert!(
        next["terms"][0]["failed"].get("request").is_none(),
        "{next}"
    );
    let next = sole_json(&acq(
        &base,
        &["--json", "search", "--next", token, "--routes"],
    ));
    assert!(next["terms"][0]["failed"]["request"].is_object(), "{next}");
}

/// C119: both second-round Opus runs measured an ambiguous clause by hand
/// at three or four calls (the trade seat: cli-opus-2 calls 36–38 for the
/// scourge line, 42–43 for the Blight-ravaged maps). The clause's line
/// says what its other reading adds and drops, and the routes block
/// carries the request for each it changes, unasked: here the seat's
/// `increased Rarity` at 20, read the other way without its `reduced`
/// form, beside a text two stats share, which has no other reading. (The
/// maps the seat asked by carry none since round x read a Blight-ravaged
/// map as no blighted one.)
#[test]
fn an_ambiguous_clauses_line_says_what_its_other_reading_adds_and_drops() {
    let scratch = harness::scratch("seat-trade-stake");
    let base = scratch.0.clone();
    let mock = base.join("store").join("mock");
    std::fs::create_dir_all(&mock).unwrap();
    let at = acquisition_store::now();
    Index::load(&mock)
        .unwrap()
        .record_login("Alice#1234", "u-seat", false, at)
        .unwrap();
    let mut store = Store::open(&account_path(&mock, "Alice#1234")).unwrap();
    let mut record = |ep: Endpoint, params: Value, body: Value| {
        store.record(&ep, &params, 200, &body, at).unwrap();
    };
    record(
        Endpoint::Profile,
        json!({}),
        json!({ "uuid": "u-seat", "name": "Alice#1234" }),
    );
    let pc = || ("pc".to_string(), "Standard".to_string());
    record(
        Endpoint::Stashes {
            realm: pc().0,
            league: pc().1,
        },
        json!({ "league": "Standard" }),
        json!({ "stashes": [ { "id": "m1", "name": "Rings", "type": "PremiumStash", "index": 0 } ] }),
    );
    let ring = |id: &str, rarity: &[&str]| {
        let mut lines: Vec<&str> = rarity.to_vec();
        lines.push("Recover 2% of Mana on Kill");
        json!({ "id": id, "name": "", "typeLine": "Iron Ring", "baseType": "Iron Ring", "frameTypeId": 2, "x": 0, "y": 0, "explicitMods": lines })
    };
    record(
        Endpoint::Stash {
            realm: pc().0,
            league: pc().1,
            id: "m1".into(),
            sub: None,
        },
        json!({}),
        json!({ "stash": { "id": "m1", "name": "Rings", "type": "PremiumStash", "items": [
            ring("plain", &["25% increased Rarity of Items found"]),
            ring("both", &["30% increased Rarity of Items found", "10% reduced Rarity of Items found"]) ] } }),
    );
    let mut child = harness::command(&base, &["search", "--realm", "pc", "--trade-file", "-"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    use std::io::Write as _;
    child
        .stdin
        .take()
        .unwrap()
        .write_all(
            br#"{"query": {"stats": [{"type": "and", "filters": [{"id": "explicit.stat_3917489142", "value": {"min": 20}}, {"id": "explicit.stat_1030153674"}]}]}}"#,
        )
        .unwrap();
    let out = child.wait_with_output().unwrap();
    let shown = stdout(&out);
    assert!(out.status.success(), "{shown}");
    let line = |starts: &str| {
        shown
            .lines()
            .find(|l| l.trim_start().starts_with(starts))
            .unwrap_or_else(|| panic!("no line {starts}: {shown}"))
            .to_string()
    };
    assert!(
        line("ambiguous stats.0.0").ends_with(" · read the other way: adds 0, drops 1"),
        "{shown}"
    );
    // a clause with no other reading says nothing of one (C93)
    assert!(
        !line("ambiguous stats.0.1").contains("other way"),
        "{shown}"
    );
    let routes: Vec<&str> = shown
        .lines()
        .skip_while(|l| !l.starts_with("routes"))
        .collect();
    let at = routes
        .iter()
        .position(|l| *l == "  clause stats.0.0 read the other way, drops (1), over the scope:")
        .unwrap_or_else(|| panic!("{shown}"));
    assert_eq!(
        routes[at + 1],
        "    acq search --account 'Alice#1234' --realm pc 'sum(line(\"#% increased Rarity of Items found\" source=explicit).arg1)>=20 line(\"#% reduced Rarity of Items found\" source=explicit) line((\"Recover #% of Mana on Kill\" or \"Lose #% of Mana on Kill\") source=explicit)'",
        "{shown}"
    );
    assert!(!shown.contains("read the other way, adds"), "{shown}");
}

/// `(Remove-only)` stood on tab names with nothing to say what it is
/// (the trade seat: cli-sonnet-2 never found out, debrief 6; mcp-opus-1
/// counted it among the jargon). C117: where tabs are listed, the mark is
/// said once — GGG's, on a tab whose items can only be taken out — and
/// the field a search asks it by is named; a list with no such tab says
/// nothing of it.
#[test]
fn the_tabs_list_says_once_what_the_remove_only_mark_is() {
    let scratch = harness::scratch("seat-remove-only");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let said = "a name ending (Remove-only) is GGG's mark on a tab whose items can only be taken out; a search asks it as tab.remove_only";
    let plain = stdout(&acq(&base, &["tabs"]));
    assert!(!plain.contains("Remove-only"), "{plain}");
    let mock = base.join("store").join("mock");
    let mut store = Store::open(&account_path(&mock, "Alice#1234")).unwrap();
    store
        .record(
            &Endpoint::Stashes {
                realm: "pc".into(),
                league: "Standard".into(),
            },
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": [
                { "id": "a1", "name": "Gear", "type": "PremiumStash", "index": 0 },
                { "id": "a2", "name": "Dump (Remove-only)", "type": "PremiumStash", "index": 1 },
                { "id": "a3", "name": "Gems (Remove-only)", "type": "GemStash", "index": 2 } ] }),
            acquisition_store::now(),
        )
        .unwrap();
    drop(store);
    let listed = stdout(&acq(&base, &["tabs"]));
    assert_eq!(listed.matches(said).count(), 1, "{listed}");
    // and a search asks it by that field: both fetched tabs, `Gear` in
    // each league, carry no mark
    let found = sole_json(&acq(
        &base,
        &["--json", "search", "--realm", "pc", "tab.remove_only=false"],
    ));
    assert_eq!(found["total"]["matched"], 2, "{found}");
}

/// `--limit` beside `--count` showed a table and no rows without a word
/// why: the one flag is a table's length there (the trade seat: cli-sonnet
/// in both rounds; cli-sonnet-2 call 7, its fault 5, "--count should say
/// that rows are suppressed"). Where the two meet, the answer says so; a
/// count with no `--limit` typed says nothing of it.
#[test]
fn a_limit_beside_a_count_says_a_count_shows_no_rows() {
    let scratch = harness::scratch("seat-count-limit");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let said = "limit   5, the values a table lists: a count shows no rows";
    let counted = stdout(&acq(
        &base,
        &[
            "search", "--realm", "pc", "--count", "league", "--limit", "5",
        ],
    ));
    assert_eq!(counted.matches(said).count(), 1, "{counted}");
    let crossed = stdout(&acq(
        &base,
        &[
            "search",
            "--realm",
            "pc",
            "--cross",
            "league,tab",
            "--limit",
            "5",
        ],
    ));
    assert!(crossed.contains(said), "{crossed}");
    let bare = stdout(&acq(
        &base,
        &["search", "--realm", "pc", "--count", "league"],
    ));
    assert!(!bare.contains("a count shows no rows"), "{bare}");
}

/// `show` printed an item's price, or `price none (none)`, and nothing of
/// the target form `price show` takes: cli-sonnet-2 (call 24, its fault
/// 6) learned `item/<id>` from the refusal of a bare id. The price line
/// ends with the command that says why, which runs as printed — opening
/// as every printed command does, with the account the item was shown
/// for and its realm, since a bare `price show` would select another
/// account where two are logged in and read `pc` for an Xbox item (the
/// external review of note 38 at `16bba23e`, step 4).
#[test]
fn show_prints_the_price_show_command_for_its_item() {
    let scratch = harness::scratch("seat-show-target");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let mock = base.join("store").join("mock");
    // a second account, so no account is the default
    Index::load(&mock)
        .unwrap()
        .record_login("Bob#5678", "u-bob", false, acquisition_store::now())
        .unwrap();
    // and an Xbox item of Alice's, whose realm is not the default
    let mut store = Store::open(&account_path(&mock, "Alice#1234")).unwrap();
    let at = acquisition_store::now();
    store
        .record(
            &Endpoint::Stashes {
                realm: "xbox".into(),
                league: "Standard".into(),
            },
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": [ { "id": "x1", "name": "Gear", "type": "PremiumStash", "index": 0 } ] }),
            at,
        )
        .unwrap();
    store
        .record(
            &Endpoint::Stash {
                realm: "xbox".into(),
                league: "Standard".into(),
                id: "x1".into(),
                sub: None,
            },
            &json!({}),
            200,
            &json!({ "stash": { "id": "x1", "name": "Gear", "type": "PremiumStash", "items": [
                { "id": "i-xb", "name": "", "typeLine": "Chaos Orb", "baseType": "Chaos Orb", "x": 0, "y": 0 } ] } }),
            at,
        )
        .unwrap();
    drop(store);
    for (id, realm) in [("i-std", "pc"), ("i-xb", "xbox")] {
        let shown = stdout(&acq(&base, &["show", "--account", "Alice", id]));
        let line = shown
            .lines()
            .find(|l| l.starts_with("price   "))
            .unwrap_or_else(|| panic!("{shown}"));
        let command = line
            .split(" · why: ")
            .nth(1)
            .unwrap_or_else(|| panic!("{line}"));
        assert_eq!(
            command,
            format!("acq price show --account u-seat --realm {realm} item/{id}"),
            "{line}"
        );
        let args: Vec<&str> = command.split(' ').skip(1).collect();
        let out = acq(&base, &args);
        assert!(out.status.success(), "{}", stderr(&out));
        assert!(
            stdout(&out).starts_with(&format!("item/{id}")),
            "{}",
            stdout(&out)
        );
    }
}

/// The trade header said the search ran "over the items you hold, in
/// every place", and cli-sonnet-2 read it as complete while 546 of 3,486
/// locations were fetched (its debrief, answer 4). The words say the
/// items the store has fetched and send the reader to the scope, which
/// counts the never fetched by type beside them.
#[test]
fn the_trade_headers_over_line_never_reads_as_full_coverage() {
    let scratch = harness::scratch("seat-trade-coverage");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let mock = base.join("store").join("mock");
    let mut store = Store::open(&account_path(&mock, "Alice#1234")).unwrap();
    store
        .record(
            &Endpoint::Stashes {
                realm: "pc".into(),
                league: "Standard".into(),
            },
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": [
                { "id": "a1", "name": "Gear", "type": "PremiumStash", "index": 0 },
                { "id": "m1", "name": "Maps", "type": "MapStash", "index": 1 } ] }),
            acquisition_store::now(),
        )
        .unwrap();
    drop(store);
    let shown = stdout(&acq(
        &base,
        &[
            "search",
            "--realm",
            "pc",
            "--trade",
            r#"{"query": {"type": "Chaos Orb"}}"#,
        ],
    ));
    let over = shown
        .lines()
        .find(|l| l.trim_start().starts_with("over "))
        .unwrap_or_else(|| panic!("{shown}"));
    assert!(!over.contains("every place"), "{over}");
    assert!(over.contains("the items the store has fetched"), "{over}");
    assert!(
        over.contains("what it never fetched is the scope's"),
        "{over}"
    );
    // the words "public listings" name the field a buyer's view asks: no
    // run of the re-sit found `tab.public` from them (the Sonnet runs
    // never, the Opus runs through a refusal or a 20 KB describe)
    assert!(over.contains("a tab's being tab.public"), "{over}");
    assert!(shown.contains("never fetched: MapStash 1 of 1"), "{shown}");
}

/// `--view locations` was refused by name as not built, and since the
/// scope line points at `tabs` nothing pointed at it; two runs of the
/// trade seat had followed it to "not built" (cli-sonnet-2 calls 3 and
/// 6, its fault 1). The list it stood for is `tabs` (C118), and the
/// never fetched by name is a park on `tabs`, not a search view: the
/// flag is gone, `describe` lists it nowhere, and the help names no view.
#[test]
fn the_locations_view_is_gone_since_tabs_is_the_list() {
    let scratch = harness::scratch("seat-no-view");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let refused = acq(&base, &["search", "--view", "locations"]);
    assert!(!refused.status.success());
    assert!(stderr(&refused).contains("--view"), "{}", stderr(&refused));
    assert!(
        !stderr(&refused).contains("not built"),
        "{}",
        stderr(&refused)
    );
    let help = stdout(&acq(&base, &["search", "--help"]));
    assert!(!help.contains("--view"), "{help}");
    let described = sole_json(&acq(&base, &["--json", "search", "--describe"]));
    let unbuilt: Vec<&str> = described["not_built"]
        .as_array()
        .unwrap()
        .iter()
        .map(|n| n["construct"].as_str().unwrap())
        .collect();
    assert!(
        !unbuilt.iter().any(|c| c.contains("locations")),
        "{unbuilt:?}"
    );
}

/// The release seat (cli-opus call 8): `acq version`'s text line named
/// the build and its contract and not the provider, so a person could
/// not tell from it whether the real API was in play; the JSON said it.
/// The line names the provider.
#[test]
fn the_version_line_names_the_provider() {
    let base = std::env::temp_dir().join(format!("acq-version-provider-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let out = acq(&base, &["version"]);
    let _ = std::fs::remove_dir_all(&base);
    let first = text(&out).lines().next().unwrap_or_default().to_string();
    assert!(
        first.starts_with("acq ") && first.contains("provider mock"),
        "{first:?}"
    );
}

/// The release seat (cli-opus calls 100 to 101, cli-sonnet 92 to 93): a
/// job verb with no login started a daemon, was refused "not logged in"
/// and said nothing of the daemon it left running. The spawn is the
/// ruling's; the refusal says a daemon started and how to see or end it.
#[test]
fn a_job_verb_refused_for_want_of_a_login_says_a_daemon_started() {
    let dir = harness::scratch("spawned-unsaid");
    let base = &dir.0;
    let out = acq(base, &["profile"]);
    assert!(!out.status.success());
    let said = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        said.contains("not logged in") && said.contains("a daemon was started"),
        "{said}"
    );
    let status = sole_json(&acq(base, &["daemon", "status", "--json"]));
    assert_eq!(status["running"], json!(true), "{status}");
    let stopped = sole_json(&acq(base, &["daemon", "stop", "--json"]));
    assert_eq!(stopped["stopped"], json!(true), "{stopped}");
}

/// The commands a receipt's `next:` line prints between backticks.
fn printed_in_next(said: &str) -> Vec<String> {
    let line = said
        .lines()
        .find(|l| l.starts_with("next: "))
        .unwrap_or_else(|| panic!("no next line: {said}"));
    line.split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect()
}

/// A printed command, run as a person pastes it: through `sh`, in the
/// world, no account from the environment.
fn run_as_printed(base: &Path, command: &str) -> std::process::Output {
    assert!(command.starts_with("acq "), "{command}");
    let mut cmd = std::process::Command::new("sh");
    cmd.arg("-c")
        .arg(command.replacen("acq ", "\"$ACQ\" ", 1))
        .env("ACQ", env!("CARGO_BIN_EXE_acq"));
    harness::isolate(&mut cmd, base);
    cmd.output().unwrap()
}

/// The release seat (cli-opus calls 61 to 73, cli-sonnet 55 to 64): the
/// undo a price receipt printed was refused once a second account was
/// known, since it carried no `--account`; a first write printed no undo
/// though the help promised one; and a clear's undo said nothing of why
/// it names no revision. With two accounts on disk, every undo line runs
/// as printed and puts the row back: a create's is the clear naming the
/// revision it wrote, a replace's the prior value over the revision it
/// wrote, a clear's a create, which says so.
#[test]
fn every_undo_a_price_receipt_prints_runs_as_printed_with_a_second_account_known() {
    let scratch = harness::scratch("seat-undo");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    Index::load(&base.join("store").join("mock"))
        .unwrap()
        .record_login("Bob#9", "u-bob", false, acquisition_store::now())
        .unwrap();
    let account = ["--account", "Alice#1234"];
    let price = |base: &Path| -> Value {
        let mut args = vec!["price", "show", "item/i-std", "--json"];
        args.extend(account);
        sole_json(&acq(base, &args))["listing"]["manual"].clone()
    };
    let undo = |said: &str| -> String {
        let undo: Vec<String> = printed_in_next(said)
            .into_iter()
            .filter(|c| !c.starts_with("acq price show"))
            .collect();
        let [undo] = &undo[..] else {
            panic!("one undo in the next line: {said}");
        };
        assert!(undo.contains("--account "), "{undo}");
        let out = run_as_printed(&base, undo);
        assert!(out.status.success(), "{undo}: {}", text(&out));
        undo.clone()
    };

    // A create: its undo clears what it wrote, by the revision it wrote.
    let mut args = vec!["price", "set", "item/i-std", "exact", "2", "divine"];
    args.extend(account);
    let out = acq(&base, &args);
    assert!(out.status.success(), "{}", text(&out));
    let printed = undo(&text(&out));
    assert!(
        printed.contains("price clear") && printed.contains("--if-revision 1"),
        "{printed}"
    );
    assert_eq!(price(&base), Value::Null);

    // A replace: its undo puts the prior back over the revision it wrote.
    let mut args = vec!["price", "set", "item/i-std", "exact", "2", "divine"];
    args.extend(account);
    assert!(acq(&base, &args).status.success());
    let mut args = vec![
        "price",
        "set",
        "item/i-std",
        "exact",
        "3",
        "divine",
        "--if-revision",
        "3",
    ];
    args.extend(account);
    let out = acq(&base, &args);
    assert!(out.status.success(), "{}", text(&out));
    undo(&text(&out));
    let now = price(&base);
    assert_eq!(now["value"]["amount"], json!("2"), "{now}");

    // A clear: its undo is a create, and the line says why it names no
    // revision.
    let rev = now["revision"].as_i64().unwrap().to_string();
    let mut args = vec!["price", "clear", "item/i-std", "--if-revision", &rev];
    args.extend(account);
    let out = acq(&base, &args);
    let said = text(&out);
    assert!(out.status.success(), "{said}");
    assert!(said.contains("which names no revision"), "{said}");
    undo(&said);
    assert_eq!(price(&base)["value"]["amount"], json!("2"));
}

/// The release seat (mcp-opus, mcp-sonnet, cli-sonnet call 79): a
/// cleared row took a revision, and the next write's continued from it
/// (a create at 4 after a clear at 3), said nowhere, so a run read the
/// numbers as a fault. The two writes' help says it.
#[test]
fn the_price_writes_help_says_a_clear_takes_a_revision() {
    let scratch = harness::scratch("seat-clear-revision");
    for verb in ["set", "clear"] {
        let out = acq(&scratch.0, &["price", verb, "--help"]);
        let said = text(&out).split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(
            said.contains(
                "A clear takes a revision too, so the next write's revision continues from it"
            ),
            "price {verb} --help: {said}"
        );
    }
}

/// The release seat (cli-opus calls 54, 56): `price list`'s item lines
/// had no header, so `conflict game 5 chaos 999 chaos` left the reader
/// to infer which column was the hand price and which the game's. A
/// header line names the columns, once, above the first item line.
#[test]
fn price_lists_columns_are_named_by_a_header_line() {
    let scratch = harness::scratch("seat-list-header");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let out = acq(
        &base,
        &["price", "set", "item/i-std", "exact", "5", "chaos"],
    );
    assert!(out.status.success(), "{}", text(&out));
    let out = acq(&base, &["price", "list"]);
    let said = text(&out);
    assert!(out.status.success(), "{said}");
    let lines: Vec<&str> = said.lines().collect();
    let item = lines
        .iter()
        .position(|l| l.ends_with("i-std"))
        .unwrap_or_else(|| panic!("no item line: {said}"));
    let header = lines
        .iter()
        .position(|l| {
            let words: Vec<&str> = l.split_whitespace().collect();
            words.starts_with(&["relation", "wins", "by", "hand", "in", "game", "item", "id"])
        })
        .unwrap_or_else(|| panic!("no header line: {said}"));
    assert!(header < item, "{said}");
    assert_eq!(said.matches("relation").count(), 1, "{said}");
}

/// One account logged in on the mock, with its index entry and an empty
/// facts file: enough for a price write on a target the facts do not
/// hold, which lands as the ruling has it.
fn seed_one_account(base: &Path, user: &str, uuid: &str) -> PathBuf {
    let mock = base.join("store").join("mock");
    std::fs::create_dir_all(&mock).unwrap();
    let mut index = Index::load(&mock).unwrap();
    index
        .record_login(user, uuid, false, acquisition_store::now())
        .unwrap();
    Store::open(&account_path(&mock, user)).unwrap();
    mock
}

/// Review 8 of the release (2026-10-19): an undo printed while one
/// account was known named no account, so once a second was known the
/// pasted undo was refused as ambiguous. The undo names the account the
/// write ran as, whether given or implied.
#[test]
fn an_undo_names_the_account_the_write_ran_as_even_when_implied() {
    let dir = harness::scratch("undo-implied-account");
    let base = &dir.0;
    let mock = seed_one_account(base, "Alice#1234", "u-alice");
    let out = acq(
        base,
        &["price", "set", "item/i-undo", "exact", "1", "chaos"],
    );
    assert!(out.status.success(), "{}", text(&out));
    let said = text(&out);
    let undo = printed_in_next(&said)
        .into_iter()
        .find(|c| c.contains("price clear"))
        .unwrap_or_else(|| panic!("no clear in the next line: {said}"));
    // A second account arrives after the write, before the undo is pasted.
    Index::load(&mock)
        .unwrap()
        .record_login("Bob#1", "u-bob", false, acquisition_store::now())
        .unwrap();
    let run = run_as_printed(base, &undo);
    assert!(
        run.status.success(),
        "the undo `{undo}` was refused: {}",
        String::from_utf8_lossy(&run.stderr)
    );
}

/// Review 8: a target with a space wrote fine and its printed undo split
/// at the space. Every word of a printed command is shell-quoted where
/// a shell would split or read it.
#[test]
fn an_undo_quotes_a_target_a_shell_would_split() {
    let dir = harness::scratch("undo-quoted-target");
    let base = &dir.0;
    seed_one_account(base, "Alice#1234", "u-alice");
    let out = acq(base, &["price", "set", "item/review test", "skip"]);
    assert!(out.status.success(), "{}", text(&out));
    let said = text(&out);
    let undo = printed_in_next(&said)
        .into_iter()
        .find(|c| c.contains("price clear"))
        .unwrap_or_else(|| panic!("no clear in the next line: {said}"));
    let run = run_as_printed(base, &undo);
    assert!(
        run.status.success(),
        "the undo `{undo}` was refused: {}",
        String::from_utf8_lossy(&run.stderr)
    );
}

/// The rerun of the release seat (cli-opus-2 call 48): `price status
/// --league Allflame` ended with a next step that named no league, so
/// the printed command read Standard. The next step carries the account
/// it ran as and the league and realm where they are not the defaults,
/// and runs as printed.
#[test]
fn the_status_next_step_runs_as_printed_in_its_league() {
    let scratch = harness::scratch("status-next-league");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    let out = acq(&base, &["price", "status", "--league", "Allflame"]);
    let said = text(&out);
    assert!(out.status.success(), "{said}");
    let next = printed_in_next(&said)
        .into_iter()
        .find(|c| c.contains("price list"))
        .unwrap_or_else(|| panic!("no list command in the next line: {said}"));
    assert!(next.contains("--league Allflame"), "{next}");
    let run = run_as_printed(&base, &next);
    assert!(
        run.status.success(),
        "`{next}`: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        String::from_utf8_lossy(&run.stdout).contains("Allflame"),
        "{}",
        String::from_utf8_lossy(&run.stdout)
    );
}
