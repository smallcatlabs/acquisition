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

use harness::{golden, offline_acq as acq, shell_words, sole_json};

const USER: &str = "Alice#1234";
const UUID: &str = "u-answers";

/// A world with one account whose facts `seed` writes.
fn world(base: &Path, seed: impl FnOnce(&mut Store, i64)) -> PathBuf {
    let mock = base.join("mock");
    std::fs::create_dir_all(&mock).unwrap();
    let at = harness::now();
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

/// A reader of a bare array counts its characters by eye (C149): the
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

/// The summary's line reads its counts from the document, made over
/// every location before the ten are cut, and the command it prints to
/// list every location carries the account, the window and the limit
/// exactly as asked — a fractional window too, never its rounded words —
/// and lists every one as printed.
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
                    // twelve seconds old or less, so the 324 s window below
                    // (`--hours 0.09`) holds every event past CI's 240 s
                    // test ceiling
                    at - 12 + n,
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

    // a fractional window: the words round it, the printed command
    // carries it exactly and reads the same twelve (a command printing
    // `--hours 0.0` would read an empty window)
    let out = acq(
        &base,
        &["store", "events", "--hours", "0.09", "--limit", "20"],
    );
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        text.starts_with("12 events at 12 locations in the last 0.1 h: +12 added\n"),
        "{text}"
    );
    assert!(
        text.contains("--hours 0.09 --limit 20 --summary --expand`"),
        "{text}"
    );
    let every = run_printed(&base, &text);
    assert_eq!(every.len(), 1, "{text}");
    assert_eq!(every[0]["locations"], 12, "{}", every[0]);
    assert_eq!(every[0]["rows"].as_array().map(Vec::len), Some(12));

    let listed = sole_json(&acq(&base, &["store", "events", "--json"]));
    assert_eq!(listed["limit"], LISTED, "{listed}");
    assert_eq!(listed["returned"], LISTED, "{listed}");
    assert_eq!(listed["limit_reached"], true, "{listed}");
    assert_eq!(listed["rows"].as_array().map(Vec::len), Some(LISTED));

    // C146 (a count says what it counts): a limit met is not proof of
    // more, so the counts are the returned events', the newest, said as
    // such, with what reads more; the default invocation's footer prints
    // the command that lists every location the returned events touched,
    // with the request's account, window and limit
    let text = |args: &[&str]| String::from_utf8_lossy(&acq(&base, args).stdout).to_string();
    let reached = "(the window may hold more; a larger `--limit` reads more events)";
    let default = text(&["store", "events"]);
    assert!(
        default.starts_with(&format!(
            "the newest 10 events at 10 locations in the last 24 h {reached}: +10 added\n"
        )),
        "{default}"
    );
    assert!(
        default.ends_with(
            "(`acq store events --account 'Alice#1234' --hours 24 --limit 10 --summary --expand` lists every location the returned events touched)\n"
        ),
        "{default}"
    );
    let every = run_printed(&base, &default);
    assert_eq!(every.len(), 1, "{default}");
    assert_eq!(every[0]["events"], 10, "{}", every[0]);
    assert_eq!(every[0]["locations"], 10, "{}", every[0]);
    // a window of exactly the limit's events sets it too: the words say
    // "may"
    let exactly = text(&["store", "events", "--limit", "12"]);
    assert!(
        exactly.starts_with(&format!(
            "the newest 12 events at 12 locations in the last 24 h {reached}: +12 added\n"
        )),
        "{exactly}"
    );
    // a non-default limit and window
    let five = text(&["store", "events", "--hours", "0.09", "--limit", "5"]);
    assert!(
        five.starts_with(&format!(
            "the newest 5 events at 5 locations in the last 0.1 h {reached}: +5 added\n"
        )),
        "{five}"
    );
    assert!(
        five.contains(
            "`acq store events --account 'Alice#1234' --hours 0.09 --limit 5 --summary --expand`"
        ),
        "{five}"
    );
    // a limit of zero reads none, and the window is not said empty
    let zero = text(&["store", "events", "--limit", "0"]);
    assert_eq!(
        zero,
        format!("the newest 0 events in the last 24 h {reached}\n"),
        "{zero}"
    );
    // the raw list says the same of its own count
    let raw = text(&["store", "events", "--expand"]);
    assert!(
        raw.ends_with(
            "10 event(s) (limit reached: the window may hold more; a larger `--limit` reads more events)\n"
        ),
        "{raw}"
    );
}

/// C146 (a count says what it counts and what the reader can do next):
/// a refused list shorter than the bodies on record says it shows part
/// of a whole and prints the command that lists them all, carrying the
/// account it read, run here as printed over a store with two accounts,
/// where a command with no selector is refused as ambiguous.
#[test]
fn a_refused_list_shorter_than_the_whole_says_so_and_how_to_list_them_all() {
    let scratch = harness::scratch("store-refused");
    let base = scratch.0.clone();
    let mock = world(&base, |store, at| {
        let endpoint = Endpoint::Stash {
            realm: "pc".into(),
            league: "Standard".into(),
            id: "t1".into(),
            sub: None,
        };
        for n in 0..3 {
            // an item with no `id`: refused, its body kept
            let body = json!({ "stash": { "id": "t1", "name": "t1", "type": "PremiumStash",
                "items": [ { "typeLine": format!("Nameless {n}") } ] } });
            assert!(
                store
                    .record(&endpoint, &json!({ "league": "Standard" }), 200, &body, at)
                    .is_err()
            );
        }
    });
    Index::load(&mock)
        .unwrap()
        .record_login("Bob#9", "u-bob", false, 1, &[])
        .unwrap();
    let out = acq(
        &base,
        &["store", "refused", "--account", USER, "--limit", "2"],
    );
    assert!(out.status.success(), "{out:?}");
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        text.ends_with(
            "2 refused bodies shown of 3 on record; `acq store refused --account 'Alice#1234' --limit 3` lists them all — `acq store refused <id>` prints one in full\n"
        ),
        "{text}"
    );
    // the continuation, run as printed; the one-body action after it
    // names a placeholder
    let continuation = text.split(" — ").next().unwrap();
    let all = run_printed(&base, continuation);
    assert_eq!(all.len(), 1, "{text}");
    assert_eq!(
        (&all[0]["returned"], &all[0]["total"]),
        (&json!(3), &json!(3)),
        "{}",
        all[0]
    );
    let out = acq(&base, &["store", "refused", "--account", USER]);
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        text.ends_with(
            "3 refused bodies on record — `acq store refused <id>` prints one in full\n"
        ),
        "{text}"
    );
}

/// C149: `accounts --json` and the MCP's `accounts` both print the layer's
/// document, never its inner array alone: ten accounts beside the count of all,
/// and the text says what it left out with the flag that lists every one.
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
    // C146: the time is written by a sign-in and by a token refresh's
    // persist alike, so an account whose last write was a refresh says
    // both writers
    let refreshed = harness::now() - 19 * 86_400;
    Index::load(&mock)
        .unwrap()
        .upsert("Exile00", false, refreshed, &[])
        .unwrap();
    let text = String::from_utf8_lossy(&acq(&base, &["accounts", "--expand"]).stdout).to_string();
    let row = text
        .lines()
        .find(|l| l.starts_with("Exile00 "))
        .unwrap_or_else(|| panic!("{text}"));
    assert!(
        row.contains(" last sign-in or token refresh 19d ago "),
        "{row}"
    );
}

/// The one test clock: two runs in one test read one instant, the
/// harness's, so a stored time's age is the same words in both and their
/// documents compare whole, never with the ages set aside. The time sits
/// 89 seconds before that instant, where one second more is `1m ago`.
#[test]
fn two_runs_in_one_test_say_one_age_and_their_documents_are_equal_whole() {
    let scratch = harness::scratch("store-one-clock");
    let base = scratch.0.clone();
    let mock = world(&base, |_, _| {});
    Index::load(&mock)
        .unwrap()
        .upsert(USER, false, harness::now() - 89, &[])
        .unwrap();
    let first = sole_json(&acq(&base, &["accounts", "--json"]));
    let second = sole_json(&acq(&base, &["accounts", "--json"]));
    assert_eq!(first["accounts"][0]["last_login_age"], "89s ago", "{first}");
    assert_eq!(first, second);
}

/// C149, the CLI's pin: each `--json` answer listed here is one object, never a
/// bare array or a bare null, read in a world where each would have been one —
/// the store's lists empty, no policy, the job list live: `store characters`,
/// `store events` and its `--summary`, `jobs` (and `--all`), `accounts`,
/// `policy show` and `store refused`, each with the counts its text prints.
#[test]
fn c149_every_json_answer_the_step_wrapped_is_one_object() {
    let scratch = harness::scratch("store-c149");
    let base = scratch.0.clone();
    let (_daemon, _pid) = harness::start_daemon(&base);
    let _login = harness::login(&base, "alice");
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
    assert_eq!(
        answer(&["policy", "show"]),
        json!({ "policy": null, "leagues_held": [] })
    );
    let refused = answer(&["store", "refused"]);
    assert_eq!(
        refused,
        json!({ "limit": 50, "returned": 0, "total": 0, "rows": [] }),
        "{refused}"
    );
}

/// The seeded world (`acquisition-client/tests/harness/seeded.rs`) read
/// through every leaf that answers from the store, with no daemon: each
/// is given its account, realm and league by name, exits 0, and answers
/// what the world holds — the three accounts, the realms and leagues, the
/// never-fetched tab, both prices, the saved search and the policy's
/// league held nowhere; the hostile tab name and note come back as
/// written. A leaf that needs a daemon (a fetch, `auth check`, `dash`),
/// writes (a set, a clear, `store rebuild`), serves (`web`) or reads
/// nothing the world wrote (`version`, `reference currency`, `jobs` and
/// `daemon status`, which read only a daemon's queue and rails) is not
/// read here.
#[test]
fn every_read_leaf_answers_over_the_seeded_world_with_no_daemon() {
    use harness::seeded::{
        ALICE, BOB, CAROL, HARDCORE, HOSTILE_ITEM, HOSTILE_NOTE, HOSTILE_TAB_NAME,
        NEVER_FETCHED_NAME, NEVER_FETCHED_TAB, POE2, PRICED_ITEM, PRICED_TAB, SAVED_ID, SAVED_NAME,
        SAVED_TEXT, SETTLERS, STANDARD, Seeded,
    };
    let scratch = harness::scratch("seeded-reads");
    let base = scratch.0.clone();
    Seeded::owner().write(&base, harness::now());
    let pc = ["--realm", "pc"];
    let standard = ["--league", STANDARD];
    let as_ = |account: &'static str, leaf: &[&'static str]| -> Vec<&'static str> {
        [&["--account", account][..], leaf].concat()
    };
    let price_tab = format!("tab/{PRICED_TAB}");
    let price_item = format!("item/{PRICED_ITEM}");
    let reads: Vec<(Vec<&str>, Vec<&str>)> = vec![
        (vec!["accounts"], vec![ALICE, BOB, CAROL]),
        (
            vec!["auth", "status"],
            vec![ALICE, BOB, CAROL, "not persisted"],
        ),
        // names are redacted, a tab's name is kept
        (
            as_(ALICE, &["diagnose"]),
            vec!["responses 15 ", HOSTILE_TAB_NAME],
        ),
        (
            as_(ALICE, &["leagues", "--stored"]),
            vec![
                "Standard (pc",
                "Hardcore (pc",
                "Standard (poe2",
                "Settlers (pc, no start given, named, nothing on disk)",
            ],
        ),
        (
            [
                as_(ALICE, &["tabs"]),
                pc.to_vec(),
                standard.to_vec(),
                vec!["--never-fetched"],
            ]
            .concat(),
            vec![NEVER_FETCHED_NAME, NEVER_FETCHED_TAB, "never"],
        ),
        (
            [
                as_(ALICE, &["tabs"]),
                pc.to_vec(),
                vec!["--league", HARDCORE],
            ]
            .concat(),
            vec![HOSTILE_TAB_NAME],
        ),
        (
            [as_(BOB, &["tabs"]), pc.to_vec(), standard.to_vec()].concat(),
            vec!["Bobs"],
        ),
        (
            [as_(CAROL, &["tabs"]), pc.to_vec(), standard.to_vec()].concat(),
            vec!["no tabs known"],
        ),
        (
            [as_(ALICE, &["search"]), pc.to_vec(), vec!["rarity=unique"]].concat(),
            vec!["Kaom's Heart"],
        ),
        (
            [as_(
                ALICE,
                &["search", "--realm", POE2, "line(template:spirit)"],
            )]
            .concat(),
            vec!["poe2 · live"],
        ),
        (
            [as_(BOB, &["search"]), pc.to_vec()].concat(),
            vec!["3 matches"],
        ),
        (
            as_(ALICE, &["show", HOSTILE_ITEM]),
            vec![HOSTILE_TAB_NAME, HOSTILE_NOTE],
        ),
        (as_(ALICE, &["store", "status"]), vec!["tabs 9"]),
        (
            [
                as_(ALICE, &["store", "characters"]),
                pc.to_vec(),
                standard.to_vec(),
            ]
            .concat(),
            vec!["Exile"],
        ),
        (as_(ALICE, &["store", "events"]), vec!["events"]),
        (as_(ALICE, &["store", "refused"]), vec!["0 refused bodies"]),
        (as_(CAROL, &["store", "status"]), vec!["responses 1 "]),
        (
            [
                as_(ALICE, &["refresh", "--plan"]),
                pc.to_vec(),
                standard.to_vec(),
            ]
            .concat(),
            vec!["policy 1"],
        ),
        (as_(ALICE, &["policy", "show"]), vec![SETTLERS]),
        (
            [
                as_(ALICE, &["price", "status"]),
                pc.to_vec(),
                standard.to_vec(),
            ]
            .concat(),
            vec!["1 decided by hand"],
        ),
        (
            [
                as_(ALICE, &["price", "list"]),
                pc.to_vec(),
                standard.to_vec(),
            ]
            .concat(),
            vec![PRICED_ITEM, "3 chaos"],
        ),
        (
            [
                as_(ALICE, &["price", "show"]),
                pc.to_vec(),
                standard.to_vec(),
                vec![price_item.as_str()],
            ]
            .concat(),
            vec!["3 chaos"],
        ),
        (
            [
                as_(ALICE, &["price", "show"]),
                pc.to_vec(),
                standard.to_vec(),
                vec![price_tab.as_str()],
            ]
            .concat(),
            vec!["by hand: skip"],
        ),
        (vec!["saved", "list"], vec![SAVED_NAME, SAVED_ID]),
        (
            vec!["saved", "show", SAVED_ID],
            vec![SAVED_NAME, SAVED_TEXT],
        ),
        (
            [
                as_(ALICE, &["shop", "render"]),
                pc.to_vec(),
                standard.to_vec(),
            ]
            .concat(),
            vec!["1 item to post"],
        ),
    ];
    for (args, holds) in &reads {
        let out = acq(&base, args);
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(
            out.status.success(),
            "acq {args:?}: {stdout}{}",
            String::from_utf8_lossy(&out.stderr)
        );
        for held in holds {
            assert!(
                stdout.contains(held),
                "acq {args:?} lacks `{held}`:\n{stdout}"
            );
        }
    }
    assert!(harness::no_daemon_appeared(&base));
}

/// The file at `path` with its `-wal` and `-shm`, each moved into `aside`.
fn move_aside(path: &Path, aside: &Path) {
    std::fs::create_dir_all(aside).unwrap();
    for file in acquisition_store::backup::sqlite_files(path) {
        if file.exists() {
            std::fs::rename(&file, aside.join(file.file_name().unwrap())).unwrap();
        }
    }
}

/// C201 — the backup and restore texts, read from their documents (C53):
/// a backup says what it kept, counted, what no backup holds and the
/// command that puts it back, which runs as printed; a restore says what
/// it put back, counted, and each file left standing with the three files
/// to move aside; a lacking file beside its own `-wal` stands; and the two
/// refusals a person meets in words, a directory that exists and one that
/// holds no manifest.
#[test]
fn backup_and_restore_say_what_they_kept_put_back_and_left_standing() {
    use harness::seeded::{ALICE, BOB, Seeded};
    let scratch = harness::scratch("backup-text");
    let base = scratch.0.clone();
    let mock = Seeded::owner().write(&base, harness::now());
    let dir = base.join("backup");
    let dir_word = dir.display().to_string();
    let mut t = harness::Transcript::offline(&base);
    let out = t.run(
        &["--account", ALICE, "store", "backup", &dir_word],
        &[
            "backed up Alice#1234 to ",
            "its facts (",
            "item events",
            "and its sync policy",
            "the world's saved searches (1)",
            "a backup holds no login and no job queue",
            "it puts back what a store lacks: acq store restore",
        ],
    );
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    let restore_line = text
        .lines()
        .find_map(|l| l.strip_prefix("it puts back what a store lacks: "))
        .expect("the restore command")
        .to_string();
    t.run(
        &["--account", ALICE, "store", "backup", &dir_word],
        &["backup exists", "name one that does not exist"],
    );
    let aside = base.join("aside");
    move_aside(&account_path(&mock, ALICE), &aside);
    move_aside(&acquisition_store::user_path(&mock), &aside);
    std::fs::write(
        acquisition_store::backup::sqlite_files(&acquisition_store::user_path(&mock))[1].clone(),
        b"",
    )
    .unwrap();
    t.run(
        &["--account", ALICE, "store", "restore", &dir_word],
        &[
            "put back 1 of 3 files from the backup of Alice#1234",
            "its facts (",
            "left standing 2",
            "move each aside with its -wal and -shm",
            "its intent:",
            "user.db-wal",
        ],
    );
    let interrupted = base.join("interrupted");
    std::fs::create_dir(&interrupted).unwrap();
    t.run(
        &[
            "--account",
            ALICE,
            "store",
            "restore",
            &interrupted.display().to_string(),
        ],
        &["holds no manifest", "interrupted", "removed and made again"],
    );
    // a facts file a development build wrote, below the released format:
    // no backup is made of it, and the refusal says what a move costs
    let bobs = account_path(&mock, BOB);
    rusqlite::Connection::open(&bobs)
        .unwrap()
        .pragma_update(None, "user_version", 7)
        .unwrap();
    let below = base.join("below");
    t.run(
        &[
            "--account",
            BOB,
            "store",
            "backup",
            &below.display().to_string(),
        ],
        &[
            "a development build wrote it",
            "move it and its -wal and -shm files aside",
            "the next refresh fetches what the account holds now",
            "but not the history the file held",
            "a backup of the account, restored",
        ],
    );
    assert!(!below.exists(), "a failed backup left its directory");
    golden(file!(), "backup_and_restore", &t);
    // the restore line runs as printed: the facts now stand, the rest too
    let ran = harness::offline_printed(&base, &restore_line)
        .output()
        .expect("the printed restore");
    assert!(ran.status.success(), "{restore_line}: {ran:?}");
    assert!(
        String::from_utf8_lossy(&ran.stdout).starts_with("put back none of 3 files"),
        "{ran:?}"
    );
}

/// C201, C193 — a store whose files are all gone, in a mock world with its
/// daemon: a backup taken while the daemon ingests; a restore under the
/// live daemon refused, nothing written; then the person's sequence: log
/// in, stop the daemon, a restore that names the facts file the login made
/// as standing, that file moved aside, and the restore again, which puts
/// it back; the store then reads whole, holding what the manifest counts.
#[test]
fn a_store_whose_files_are_gone_is_logged_into_and_restored() {
    let scratch = harness::scratch("backup-mock");
    let base = scratch.0.clone();
    let daemon = harness::login(&base, "alice");
    let mock = base.join("store").join("mock");
    // the listing, then the tabs; a third fetch ingests while the backup
    // runs
    for _ in 0..2 {
        let fetched = harness::acq(&base, &["refresh", "--all"]);
        assert!(fetched.status.success(), "{fetched:?}");
    }
    let saved = harness::acq(
        &base,
        &["saved", "set", "--name", "uniques", "rarity=unique"],
    );
    assert!(saved.status.success(), "{saved:?}");
    let mut ingesting = harness::command(&base, &["refresh", "--all"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("a refresh");
    let dir = base.join("backup");
    let made = harness::acq(
        &base,
        &["--json", "store", "backup", &dir.display().to_string()],
    );
    assert!(made.status.success(), "{made:?}");
    assert!(ingesting.wait().expect("the refresh").success());
    let manifest = sole_json(&made)["manifest"].clone();
    assert!(
        manifest["facts"]["items"].as_i64().unwrap() > 0,
        "{manifest}"
    );
    assert!(manifest["saved"].is_object(), "{manifest}");

    let restore = |base: &Path| {
        harness::acq(
            base,
            &["--json", "store", "restore", &dir.display().to_string()],
        )
    };
    let refused = restore(&base);
    assert_eq!(refused.status.code(), Some(1), "{refused:?}");
    let doc = sole_json(&refused);
    assert_eq!(doc["kind"], "refused", "{doc}");
    assert!(
        doc["error"].as_str().unwrap().contains("stop the daemon"),
        "{doc}"
    );

    // the whole store gone: the three files and their logs
    let stopped = harness::acq(&base, &["daemon", "stop"]);
    assert!(stopped.status.success(), "{stopped:?}");
    drop(daemon);
    let gone = base.join("gone");
    let uuid = manifest["account"]["uuid"].as_str().unwrap().to_string();
    let name = manifest["account"]["name"].as_str().unwrap().to_string();
    let facts = account_path(&mock, &name);
    for file in [
        facts.clone(),
        acquisition_store::annotations_path(&mock, &uuid),
        acquisition_store::user_path(&mock),
    ] {
        move_aside(&file, &gone);
    }
    // log in, stop the daemon: the login made the facts file
    let daemon = harness::login(&base, "alice");
    let stopped = harness::acq(&base, &["daemon", "stop"]);
    assert!(stopped.status.success(), "{stopped:?}");
    drop(daemon);
    assert!(facts.exists(), "the login made the facts file");
    let first = restore(&base);
    assert!(first.status.success(), "{first:?}");
    let first = sole_json(&first);
    let standing = first["standing"].as_array().unwrap();
    assert_eq!(standing.len(), 1, "{first}");
    assert_eq!(standing[0]["file"], "facts", "{first}");
    assert_eq!(
        standing[0]["move_aside"],
        json!(acquisition_store::backup::sqlite_files(&facts).map(|p| p.display().to_string())),
        "{first}"
    );
    move_aside(&facts, &base.join("login"));
    let second = restore(&base);
    assert!(second.status.success(), "{second:?}");
    let second = sole_json(&second);
    // the facts put back; the intent and the saved searches, put back by
    // the first, stand
    assert_eq!(
        second["restored"],
        json!([{ "file": "facts", "path": facts.display().to_string() }]),
        "{second}"
    );
    assert_eq!(
        second["standing"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s["file"].clone())
            .collect::<Vec<_>>(),
        [json!("intent"), json!("saved")],
        "{second}"
    );

    // whole: the restored store reads, holding what the manifest counts
    let status = harness::acq(&base, &["--json", "store", "status"]);
    assert!(status.status.success(), "{status:?}");
    let status = sole_json(&status);
    assert_eq!(
        (
            &status["items"],
            &status["items_removed"],
            &status["events"]
        ),
        (
            &manifest["facts"]["items"],
            &manifest["facts"]["removed_items"],
            &manifest["facts"]["events"]
        ),
        "{status}"
    );
    let stopped = harness::acq(&base, &["daemon", "stop"]);
    assert!(stopped.status.success(), "{stopped:?}");
}
