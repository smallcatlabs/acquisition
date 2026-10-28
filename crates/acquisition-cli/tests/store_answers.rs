//! The store's reads as the CLI answers them, one document each (C149,
//! every answer one object, its lists counted beside their rows; C53, the
//! read economy): what the adapter adds to the verbs' documents — the
//! text read from the document's counts, and the printed command that
//! lists every row, which runs as printed. The documents' counts and
//! bounds are pinned once, in the verbs crate.
//!
//! Facts are seeded through the store crate directly, so no daemon runs
//! (`ACQ_NO_SPAWN` makes any contact an error).

use std::path::{Path, PathBuf};

use acquisition_store::bound::LISTED;
use acquisition_store::{Endpoint, Index, Store, account_path};
use serde_json::{Value, json};

use crate::harness;

use harness::{offline_acq as acq, shell_words, sole_json};

const USER: &str = "Alice#1234";
const UUID: &str = "u-answers";

/// A world with one account whose facts `seed` writes.
fn world(base: &Path, seed: impl FnOnce(&mut Store, i64)) -> PathBuf {
    let mock = base.join("mock");
    std::fs::create_dir_all(&mock).unwrap();
    let at = acquisition_store::now();
    let mut index = Index::load(&mock).unwrap();
    index.record_login(USER, UUID, false, at, &[]).unwrap();
    let mut store = Store::open(&account_path(&mock, USER)).unwrap();
    store
        .record(
            &Endpoint::Profile,
            &json!({}),
            200,
            &json!({ "uuid": UUID, "name": USER }),
            at,
        )
        .unwrap();
    seed(&mut store, at);
    mock
}

/// The text's commands between backticks that start with `acq `, each
/// run as a shell would split it, its JSON returned.
fn run_printed(base: &Path, text: &str) -> Vec<Value> {
    text.split('`')
        .skip(1)
        .step_by(2)
        .filter(|c| c.starts_with("acq "))
        .map(|c| {
            let mut words = shell_words(c);
            words.remove(0);
            words.push("--json".into());
            let words: Vec<&str> = words.iter().map(String::as_str).collect();
            let out = acq(base, &words);
            assert!(out.status.success(), "`{c}`: {out:?}");
            sole_json(&out)
        })
        .collect()
}

/// The seat counted 65 characters by eye from a bare array (C149): the
/// text lists ten and reads its footer from the document's counts, made
/// over all of them, and the command it prints to list every one carries
/// the account and the selection and lists every one, as printed.
#[test]
fn store_characters_lists_ten_beside_its_counts_and_prints_the_command_for_all() {
    let scratch = harness::scratch("store-characters");
    let base = scratch.0.clone();
    world(&base, |store, at| {
        let characters: Vec<Value> = (0..13)
            .map(|n| {
                json!({ "id": format!("c{n:02}"), "name": format!("Exile{n:02}"),
                             "league": "Standard", "class": "Witch", "level": 90 - n })
            })
            .collect();
        store
            .record(
                &Endpoint::Characters { realm: "pc".into() },
                &json!({ "realm": "pc" }),
                200,
                &json!({ "characters": characters }),
                at,
            )
            .unwrap();
    });
    let out = acq(&base, &["store", "characters", "--league", "Standard"]);
    assert!(out.status.success(), "{out:?}");
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    let rows = text.lines().filter(|l| l.starts_with("Exile")).count();
    assert_eq!(rows, LISTED, "{text}");
    assert!(
        text.contains("13 characters: 0 fetched, 13 never fetched; 0 items\n"),
        "{text}"
    );
    assert!(
        text.ends_with(
            "10 of 13 listed: `acq store characters --account 'Alice#1234' --league Standard --expand` lists every one\n"
        ),
        "{text}"
    );
    let every = run_printed(&base, &text);
    assert_eq!(every.len(), 1, "{text}");
    assert_eq!(every[0]["characters"], 13, "{}", every[0]);
    assert_eq!(every[0]["rows"].as_array().map(Vec::len), Some(13));
    assert_eq!(every[0]["left_out"], 0);

    let json = sole_json(&acq(&base, &["store", "characters", "--json"]));
    assert_eq!(json["characters"], 13, "{json}");
    assert_eq!(json["never_fetched"], 13, "{json}");
    assert_eq!(json["rows"].as_array().map(Vec::len), Some(LISTED));
    assert_eq!(json["left_out"], 3, "{json}");
    let two = sole_json(&acq(
        &base,
        &["store", "characters", "--limit", "2", "--json"],
    ));
    assert_eq!(two["rows"][1]["id"], "c01", "{two}");
    assert_eq!(two["left_out"], 11, "{two}");
}

/// The second review of the brief, finding 4: the summary's line reads
/// its counts from the document, made over every location before the
/// ten are cut, and the command it prints to list every location carries
/// the account, the window and the limit, and lists every one as printed.
/// The event list is ten by default, its limit said reached.
#[test]
fn store_events_summary_counts_every_location_and_prints_the_command_for_all() {
    let scratch = harness::scratch("store-events");
    let base = scratch.0.clone();
    world(&base, |store, at| {
        for n in 0..12 {
            let tab = format!("t{n:02}");
            store
                .record(
                    &Endpoint::Stash {
                        realm: "pc".into(),
                        league: "Standard".into(),
                        id: tab.clone(),
                        sub: None,
                    },
                    &json!({ "league": "Standard" }),
                    200,
                    &json!({ "stash": { "id": tab, "name": tab, "type": "PremiumStash", "items": [
                        { "id": format!("i{n:02}"), "typeLine": "Iron Ring", "baseType": "Iron Ring",
                          "frameType": 0, "identified": true, "ilvl": 1, "x": 0, "y": 0 } ] } }),
                    at - 60 + n,
                )
                .unwrap();
        }
    });
    let out = acq(&base, &["store", "events", "--limit", "20"]);
    assert!(out.status.success(), "{out:?}");
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        text.starts_with("12 events at 12 locations in the last 24 h: +12 added\n"),
        "{text}"
    );
    assert_eq!(
        text.lines().filter(|l| l.contains("stash/pc/")).count(),
        LISTED,
        "{text}"
    );
    assert!(
        text.contains(
            "10 of 12 locations listed: `acq store events --account 'Alice#1234' --hours 24 --limit 20 --summary --expand` lists every one\n"
        ),
        "{text}"
    );
    let every = run_printed(&base, &text);
    assert_eq!(every.len(), 1, "{text}");
    assert_eq!(every[0]["locations"], 12, "{}", every[0]);
    assert_eq!(every[0]["rows"].as_array().map(Vec::len), Some(12));
    assert_eq!(every[0]["by_kind"]["added"], 12);

    let listed = sole_json(&acq(&base, &["store", "events", "--json"]));
    assert_eq!(listed["limit"], LISTED, "{listed}");
    assert_eq!(listed["returned"], LISTED, "{listed}");
    assert_eq!(listed["limit_reached"], true, "{listed}");
    assert_eq!(listed["rows"].as_array().map(Vec::len), Some(LISTED));
}

/// C149: `accounts --json` printed the inner array where the MCP's
/// `accounts` answered the object; both print the layer's document now,
/// ten accounts beside the count of all, and the text says what it left
/// out with the flag that lists every one.
#[test]
fn accounts_json_is_the_layers_document_and_the_text_counts_what_it_left_out() {
    let scratch = harness::scratch("store-accounts");
    let base = scratch.0.clone();
    let mock = world(&base, |_, _| {});
    for n in 0..11 {
        Index::load(&mock)
            .unwrap()
            .record_login(&format!("Exile{n:02}"), &format!("u-{n}"), false, 1, &[])
            .unwrap();
    }
    let listed = sole_json(&acq(&base, &["accounts", "--json"]));
    assert!(listed.is_object(), "{listed}");
    assert_eq!(listed["provider"], "mock", "{listed}");
    assert_eq!(listed["total"], 12, "{listed}");
    assert_eq!(listed["accounts"].as_array().map(Vec::len), Some(LISTED));
    assert_eq!(listed["left_out"], 2, "{listed}");
    let all = sole_json(&acq(&base, &["accounts", "--expand", "--json"]));
    assert_eq!(all["accounts"].as_array().map(Vec::len), Some(12), "{all}");
    let text = String::from_utf8_lossy(&acq(&base, &["accounts"]).stdout).to_string();
    assert!(
        text.ends_with("10 of 12 accounts listed: `acq accounts --expand` lists every one\n"),
        "{text}"
    );
}

/// C149, the CLI's pin: every `--json` answer this step wrapped is one
/// object, never a bare array or a bare null, read in a world where each
/// would have been one — the store's lists empty, no policy, the job list
/// live: `store characters`, `store events` and its `--summary`, `jobs`
/// (and `--all`), `accounts`, `policy show` and `store refused`, each
/// with the counts its text prints.
#[test]
fn c149_every_json_answer_the_step_wrapped_is_one_object() {
    let scratch = harness::scratch("store-c149");
    let base = scratch.0.clone();
    let (_daemon, _pid) = harness::start_daemon(&base);
    harness::login(&base, "alice");
    let answer = |args: &[&str]| -> Value {
        let out = harness::acq(&base, &[args, &["--json"]].concat());
        assert!(out.status.success(), "{args:?}: {out:?}");
        let doc = sole_json(&out);
        assert!(doc.is_object(), "{args:?}: {doc}");
        doc
    };
    let characters = answer(&["store", "characters"]);
    for key in [
        "characters",
        "fetched",
        "empty_bodies",
        "never_fetched",
        "items",
        "left_out",
    ] {
        assert!(characters[key].is_number(), "{key}: {characters}");
    }
    assert!(characters["rows"].is_array(), "{characters}");
    let events = answer(&["store", "events"]);
    assert_eq!(events["limit"], LISTED, "{events}");
    assert!(events["limit_reached"].is_boolean(), "{events}");
    let summary = answer(&["store", "events", "--summary"]);
    for key in ["events", "locations", "left_out"] {
        assert!(summary[key].is_number(), "{key}: {summary}");
    }
    assert!(summary["by_kind"].is_object(), "{summary}");
    for args in [&["jobs"][..], &["jobs", "--all"]] {
        let jobs = answer(args);
        assert_eq!(jobs["running"], true, "{jobs}");
        assert!(jobs["total"].is_number(), "{jobs}");
        assert!(jobs["by_state"]["done"].is_number(), "{jobs}");
        assert!(jobs["rows"].is_array(), "{jobs}");
    }
    let accounts = answer(&["accounts"]);
    assert_eq!(accounts["total"], 1, "{accounts}");
    assert_eq!(answer(&["policy", "show"]), json!({ "policy": null }));
    let refused = answer(&["store", "refused"]);
    assert_eq!(
        refused,
        json!({ "limit": 50, "returned": 0, "rows": [] }),
        "{refused}"
    );
}
