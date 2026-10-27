//! `acq diagnose` (#79): what the CLI adds to the layer's diagnosis, its
//! text — every section in the words its own verb prints, the two tails
//! last, the whole redacted before it is printed — and its exit status,
//! 0 whatever it found. The document and the redaction are pinned in the
//! verbs crate (`acquisition-verbs/tests/daemon.rs`, C122); here, that the
//! text carries none of what the test planted either, held as a golden
//! whose facts name each section.

use std::path::Path;

use acquisition_store::world::World;
use acquisition_store::{Endpoint, Index, Store, account_path};
use serde_json::json;

use crate::harness;

use harness::{Transcript, acq, golden, no_daemon_appeared, sole_json};

/// What the test plants: the account's name and uuid in the index, the
/// store file's name and the log, and three tokens in the log.
const USER: &str = "Alice#1234";
const UUID: &str = "u-diagnose-cli";
const TOKEN: &str = "sentinel-code-7f3a";
/// The review's two token shapes (r4, finding 1): a quoted value after
/// `=`, and a JSON body's spacing.
const QUOTED: &str = "sentinel-refresh-q1";
const SPACED: &str = "sentinel-code-q2";

/// One account with its profile on record, and the log and journal a
/// last daemon of this world would have left under the isolation's log
/// directory: two sends of two accounts and a torn last line.
fn seed(base: &Path) {
    let mock = base.join("store").join("mock");
    std::fs::create_dir_all(&mock).unwrap();
    Index::load(&mock)
        .unwrap()
        .record_login(USER, UUID, false, 1_000, &[])
        .unwrap();
    let store_file = account_path(&mock, USER);
    Store::open(&store_file)
        .unwrap()
        .record(
            &Endpoint::Profile,
            &json!({}),
            200,
            &json!({ "uuid": UUID, "name": USER }),
            1_000,
        )
        .unwrap();
    let world = World::observe_at(&base.join("store")).unwrap();
    let logs = base.join("logs").join(world.id()).join("mock");
    std::fs::create_dir_all(&logs).unwrap();
    std::fs::write(
        logs.join("daemon.log"),
        format!(
            "[    0s] world: {}\n\
             [    5s] store: {} opened for {USER}\n\
             [    5s] logged in as {USER} ({UUID})\n\
             [    6s] callback /auth/path-of-exile?code={TOKEN}&state=s\n\
             [    6s] refresh_token=\"{QUOTED}\"\n\
             [    6s] body {{\"code\": \"{SPACED}\"}}\n",
            world.root().display(),
            store_file.display(),
        ),
    )
    .unwrap();
    let send = |account: &str| {
        json!({ "method": "GET", "route": format!("stash@{account}"), "path": "/stash/Standard/a1",
                "status": 200, "counted": true, "wait_ms": 0 })
        .to_string()
    };
    std::fs::write(
        logs.join("sends.jsonl"),
        format!(
            "{}\n{}\n{{\"route\":\"stash@{USER}",
            send(USER),
            send("Bob#5678")
        ),
    )
    .unwrap();
}

/// #79: with no daemon, every section prints — the ledger's picture with
/// the last log, the store's counts, the world's rails file, the journal
/// with its torn line reported, the log — and nothing planted is in the
/// text; `--json` is the whole document; both exit 0 and start nothing.
#[test]
fn a_diagnosis_with_no_daemon_prints_every_section_and_nothing_planted() {
    let scratch = harness::scratch("diagnose");
    let base = scratch.0.clone();
    seed(&base);

    // The isolation list turns journals off (`ACQ_JOURNAL=0`); unset,
    // the command reads the world's default journal, where `seed` wrote.
    let mut t = Transcript::new(&base).knob("ACQ_JOURNAL", None);
    let out = t.run(
        &["diagnose"],
        &[
            "Replaced before printing: 7 occurrences of account names and uuids",
            "3 occurrences of token-shaped strings",
            "== daemon\ndaemon is not running; no queue on disk\nlast log: <logs>",
            "responses 1",
            "holds\nnothing: no block mark, no trip, no account's refresh disabled",
            "== journal: the last 3 lines of <logs>/mock/sends.jsonl (the world's)",
            "\"route\":\"stash@<account 1>\"",
            "\"route\":\"stash@<account 2>\"",
            "(not read: line 3 of this tail is not JSON",
            "logged in as <account 1> (<account 1 uuid>)",
            "?code=<token 1>&state=s",
            "refresh_token=\"<token 2>\"",
            "{\"code\": \"<token 3>\"}",
        ],
    );
    assert!(out.status.success(), "{out:?}");
    let printed = String::from_utf8_lossy(&out.stdout);
    for planted in [
        USER,
        "Alice",
        "Alice_1234",
        "Bob#5678",
        UUID,
        TOKEN,
        QUOTED,
        SPACED,
    ] {
        assert!(
            !printed.contains(planted),
            "the text holds {planted:?}, which the test planted in the index, the store file's \
             name, the log (the account, its uuid, a code, a quoted refresh token and a JSON \
             body's code) and the journal's route keys:\n{printed}"
        );
    }

    let document = sole_json(&acq(&base, &["diagnose", "--lines", "2", "--json"]));
    for section in [
        "version",
        "daemon",
        "store",
        "rails",
        "journal",
        "log",
        "redactions",
    ] {
        assert!(!document[section].is_null(), "no {section}: {document:#}");
    }
    assert_eq!(
        (
            document["log"]["lines"].as_array().map(Vec::len),
            document["log"]["earlier_left_out"].clone()
        ),
        (Some(2), json!(true)),
        "`--lines 2` bounds the log's tail: {document:#}"
    );
    // Under the isolation's `ACQ_JOURNAL=0`, journals are off in this
    // shell, and the section says so rather than read the world's.
    assert!(
        document["journal"]["unread"]
            .as_str()
            .is_some_and(|why| why.contains("ACQ_JOURNAL=0")),
        "{document:#}"
    );
    assert!(no_daemon_appeared(&base), "a diagnosis started a daemon");
    golden(file!(), "no_daemon", &t);
}

/// #79: a world nothing has written yet is no failure: each section that
/// has nothing to read says why, and the command exits 0.
#[test]
fn a_diagnosis_of_an_empty_world_says_why_each_part_is_unread_and_exits_0() {
    let scratch = harness::scratch("diagnose-empty");
    let base = scratch.0.clone();
    let out = acq(&base, &["diagnose"]);
    assert!(out.status.success(), "{out:?}");
    let printed = String::from_utf8_lossy(&out.stdout);
    for fact in [
        "no account has logged in in this world",
        "== rails\nnot read: no world to read at",
        "== journal\nnot read: no world to read at",
        "== log\nnot read: no world to read at",
    ] {
        assert!(printed.contains(fact), "{fact:?} is not in:\n{printed}");
    }
    assert!(no_daemon_appeared(&base), "a diagnosis started a daemon");
}
