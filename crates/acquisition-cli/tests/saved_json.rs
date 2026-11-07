//! `acq saved` and `acq search --saved` at a terminal (C167, user-scoped
//! intent in the world's `user.db`; 4-full part 1, step 1): what the
//! adapter adds to the verbs' saved search — the parse of each spelling
//! of `set`, the text, the refusal's shape and exit, every printed
//! command running as printed, and `--saved` beside the words it
//! excludes. The verbs' semantics is pinned once, in
//! `acquisition-verbs/tests/documents.rs` (C122). The world is seeded
//! through the store crate as `price_json.rs` seeds it, so no daemon
//! runs: `ACQ_NO_SPAWN=1` makes any contact an error.

use std::path::{Path, PathBuf};
use std::process::Output;

use acquisition_store::{Provenance, SAVED_SEARCH_SCOPE, SavedSearch, User};
use serde_json::{Value, json};

use crate::harness;

use harness::{Transcript, golden, offline_acq as acq, shell_words, sole_json};

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// `acq <args> --json`, its one document.
fn json_of(base: &Path, args: &[&str]) -> Value {
    let mut args = args.to_vec();
    args.push("--json");
    sole_json(&acq(base, &args))
}

/// A printed command run as printed: its words as a shell splits them,
/// `--json` added (a JSON pin may add it), its document; it must answer.
fn run_printed(base: &Path, line: &str) -> Value {
    let words = shell_words(line);
    assert_eq!(words[0], "acq", "{line}");
    let mut args: Vec<&str> = words[1..].iter().map(String::as_str).collect();
    args.push("--json");
    let out = acq(base, &args);
    assert!(
        out.status.success(),
        "`{line}` does not run as printed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        stderr(&out)
    );
    sole_json(&out)
}

/// A saved search of `name` written through the store's door at `id`,
/// as another writer would.
fn put(mock: &Path, id: &str, name: &str) {
    User::open_for(mock)
        .unwrap()
        .put::<SavedSearch>(
            SAVED_SEARCH_SCOPE,
            id,
            &json!({ "version": 1, "name": name, "text": "rarity=rare",
                     "scope": { "realm": "pc", "membership": "live" } }),
            None,
            &Provenance::via("test"),
        )
        .unwrap();
}

/// The world of `price_json.rs`: one account, its pc facts.
fn world(base: &Path) -> PathBuf {
    crate::price_json::seed(base)
}

/// The id of the one saved search named `name` the list holds.
fn id_named(base: &Path, name: &str) -> String {
    let listed = json_of(base, &["saved", "list", "--expand"]);
    let ids: Vec<&str> = listed["searches"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| row["name"] == name)
        .map(|row| row["id"].as_str().unwrap())
        .collect();
    let [id] = ids.as_slice() else {
        panic!("{name}: {listed}")
    };
    id.to_string()
}

/// The three spellings of `set`, each one parse — a create carrying its
/// query and a name with no `--id`, an edit by `--id` and a query, a
/// rename by `--id` and `--name` with no query and no `--if-revision` —
/// with the list, the show, the clear and each refusal, as a person
/// reads them; `--json` is the verbs' document whole.
#[test]
fn saved_searches_are_listed_shown_set_and_cleared_at_a_terminal() {
    let scratch = harness::scratch("saved");
    let base = scratch.0.clone();
    let mock = world(&base);
    let mut t = Transcript::offline(&base);
    t.run(
        &["saved"],
        &["no saved searches", "acq saved set --name NAME"],
    );
    t.run(
        &[
            "saved",
            "set",
            "--name",
            "rings",
            "rarity=rare   class:rings",
        ],
        &[
            "rings over pc live: rarity=rare class:rings (revision 1), was unset",
            "undoes it",
        ],
    );
    let id = id_named(&base, "rings");
    t.placeholder(&id, "id");
    put(&mock, "0000000000a2", "amulets");
    let listed = t.run(&["saved", "list"], &["2 saved searches", "0000000000a2"]);
    assert!(String::from_utf8_lossy(&listed.stdout).contains(&id));
    t.run(
        &["saved", "show", &id],
        &[
            "scope: pc live",
            "query: rarity=rare class:rings",
            "revision 1",
        ],
    );
    t.run(
        &["saved", "set", "--id", &id, "rarity=unique", "--if-revision", "1"],
        &["rarity=unique (revision 2), was rings over pc live: rarity=rare class:rings (revision 1)"],
    );
    t.run(
        &["saved", "set", "--id", &id, "--name", "unique rings"],
        &[
            "unique rings over pc live: rarity=unique (revision 3), was rings over pc live: \
             rarity=unique (revision 2)",
            "puts the prior back, whole",
        ],
    );
    put(&mock, "0000000000a3", "amulets");
    for (args, facts) in [
        (
            &["saved", "show", "amulets"][..],
            &[
                "2 saved searches are named amulets",
                "acq saved show 0000000000a2",
            ][..],
        ),
        (
            &["saved", "show", "nothing"][..],
            &[
                "no saved search has the id or the name nothing",
                "acq saved list",
            ][..],
        ),
        (
            &[
                "saved",
                "set",
                "--id",
                &id,
                "--name",
                "x",
                "--if-revision",
                "1",
            ][..],
            &["stands at revision 3", "--if-revision 3"][..],
        ),
        (
            &[
                "saved",
                "set",
                "--name",
                "x",
                "rarity=rare",
                "--if-revision",
                "1",
            ][..],
            &["`--if-revision` names the revision of the saved search `--id` names"][..],
        ),
        (
            &["saved", "set", "--name", "x", "rarity=="][..],
            &["found `=`"][..],
        ),
        (
            &["saved", "clear", "not-an-id"][..],
            &["is no saved search's id"][..],
        ),
    ] {
        let out = t.run(args, facts);
        assert_eq!(out.status.code(), Some(1), "{args:?}");
    }
    t.run(
        &["saved", "clear", &id],
        &["cleared at revision 4", "--if-revision 0", "puts it back"],
    );
    // --json is the verbs' document whole
    let frontend = acquisition_verbs::Frontend {
        via: "cli",
        door: acquisition_verbs::Door::Human,
        provider: "mock",
        root: base.clone(),
        journal: acquisition_verbs::JournalKnob::Default,
    };
    assert_eq!(
        json_of(&base, &["saved", "list"]),
        serde_json::to_value(acquisition_verbs::saved::list(&frontend, None, false).unwrap())
            .unwrap()
    );
    assert_eq!(
        json_of(&base, &["saved", "show", "0000000000a2"]),
        serde_json::to_value(acquisition_verbs::saved::show(&frontend, "0000000000a2").unwrap())
            .unwrap()
    );

    // the review of fix round 1 (c0fb7673): a `user.db` holding the facts
    // of an account named `user`, at the literal path a build before the
    // account path's escape kept them, refused with the rename; the
    // rename the words say frees the file
    std::fs::remove_file(mock.join("user.db")).unwrap();
    for left in ["user.db-wal", "user.db-shm"] {
        assert!(!mock.join(left).exists(), "{left}");
    }
    let mut legacy = acquisition_store::Store::open(&mock.join("user.db")).unwrap();
    legacy
        .record(
            &acquisition_store::Endpoint::Profile,
            &json!({}),
            200,
            &json!({ "uuid": "u", "name": "user" }),
            1_000,
        )
        .unwrap();
    drop(legacy);
    let out = t.run(
        &["saved"],
        &[
            "holds the facts of an account named user",
            "Rename",
            "~user.db",
        ],
    );
    assert_eq!(out.status.code(), Some(1));
    std::fs::rename(mock.join("user.db"), mock.join("~user.db")).unwrap();
    t.run(&["saved"], &["no saved searches"]);
    golden(file!(), "list_show_set_and_clear", &t);
}

/// Every command a receipt or a refusal of `acq saved` prints runs as
/// printed and answers: a create's undo, a clear's (which recreates over
/// the tombstone), a rename's whose name starts with `-` and whose text
/// is the language's not; a conflict's write again; an ambiguous name's
/// opens by id; a name none holds, the list.
#[test]
fn every_command_a_saved_receipt_or_refusal_prints_runs_as_printed() {
    let scratch = harness::scratch("savedrun");
    let base = scratch.0.clone();
    let mock = world(&base);
    let line = |doc: &Value, key: &str| harness::offered_line(&doc[key]);
    let created = json_of(&base, &["saved", "set", "--name", "rings", "rarity=rare"]);
    let id = created["target"]["id"].as_str().unwrap().to_string();
    assert_eq!(
        created["written"]["scope"]["realm"], "pc",
        "the one realm held"
    );
    let undone = run_printed(&base, &line(&created, "undo"));
    assert!(
        undone["written"].is_null(),
        "the create's undo clears it: {undone}"
    );
    let back = run_printed(&base, &line(&undone, "undo"));
    assert_eq!(
        back["written"]["name"], "rings",
        "the clear's undo recreates"
    );
    let edited = json_of(&base, &["saved", "set", "--id", &id, " -is:corrupted"]);
    let not = edited["written"]["text"].as_str().unwrap().to_string();
    assert!(not.starts_with('-'), "{edited}");
    let renamed = json_of(&base, &["saved", "set", "--id", &id, "--name=-rings"]);
    assert_eq!(renamed["written"]["name"], "-rings");
    let restored = run_printed(&base, &line(&renamed, "undo"));
    assert_eq!(
        (&restored["written"]["name"], &restored["written"]["text"]),
        (&json!("rings"), &json!(not))
    );
    // the edit's undo names the revision the rename moved past: refused
    // as a conflict, whose write again runs as printed and lands
    let stale = shell_words(&line(&edited, "undo"));
    let mut args: Vec<&str> = stale[1..].iter().map(String::as_str).collect();
    args.push("--json");
    let conflict = sole_json(&acq(&base, &args));
    assert_eq!(
        (&conflict["kind"], &conflict["revision"]),
        (&json!("conflict"), &restored["revision"]),
        "{conflict}"
    );
    let again = run_printed(&base, &harness::offered_line(&conflict["offers"][0]));
    assert_eq!(again["written"]["text"], "rarity=rare");
    put(&mock, "0000000000a2", "amulets");
    put(&mock, "0000000000a3", "amulets");
    let ambiguous = json_of(&base, &["search", "--saved", "amulets"]);
    assert_eq!(ambiguous["kind"], "ambiguous_saved_search", "{ambiguous}");
    for offer in ambiguous["offers"].as_array().unwrap() {
        let request = harness::offered_line(offer);
        assert!(
            request.starts_with("acq search --saved 0000000000a"),
            "{request}"
        );
        let answer = run_printed(&base, &request);
        assert_eq!(answer["query"]["text"], "rarity=rare", "{answer}");
    }
    let shown = json_of(&base, &["saved", "show", "amulets"]);
    for offer in shown["offers"].as_array().unwrap() {
        assert_eq!(
            run_printed(&base, &harness::offered_line(offer))["name"],
            "amulets"
        );
    }
    let missing = json_of(&base, &["saved", "show", "nothing"]);
    assert_eq!(missing["kind"], "saved_search_not_found");
    let listed = run_printed(&base, &harness::offered_line(&missing["offers"][0]));
    assert_eq!(listed["total"], 3);
}

/// `acq search --saved` runs the row's request: equal to `acq search`
/// with its text and its scope typed, view flags and `--account` beside
/// it; a query, `--query-file`, `--trade`, `--trade-file`, `--realm` or
/// `--membership` beside it is refused as a continuation refuses them,
/// and `--next` and `--request` refuse it as they refuse a query.
#[test]
fn search_saved_runs_the_rows_request_and_refuses_each_word_of_scope_or_query_beside_it() {
    let scratch = harness::scratch("savedsrc");
    let base = scratch.0.clone();
    world(&base);
    let created = json_of(
        &base,
        &[
            "saved",
            "set",
            "--name",
            "chaos",
            "--membership",
            "all",
            "typeLine=\"Chaos Orb\"",
        ],
    );
    let id = created["target"]["id"].as_str().unwrap().to_string();
    let text = created["written"]["text"].as_str().unwrap().to_string();
    for view in [&[][..], &["--limit", "1"][..], &["--count", "rarity"][..]] {
        let saved: Vec<&str> = [&["search", "--saved", id.as_str()][..], view].concat();
        let typed: Vec<&str> = [
            &[
                "search",
                "--realm",
                "pc",
                "--membership",
                "all",
                text.as_str(),
            ][..],
            view,
        ]
        .concat();
        let (by_saved, by_typing) = (json_of(&base, &saved), json_of(&base, &typed));
        assert!(by_saved.get("error").is_none(), "{by_saved}");
        assert_eq!(by_saved, by_typing, "{view:?}");
    }
    let by_name = json_of(
        &base,
        &["--account", "Alice#1234", "search", "--saved", "chaos"],
    );
    assert_eq!(
        by_name["total"],
        json_of(&base, &["search", "--saved", &id])["total"]
    );
    let file = base.join("words.txt");
    std::fs::write(&file, "rarity=rare").unwrap();
    let file = file.display().to_string();
    for (beside, words) in [
        ("a query", &["rarity=rare"][..]),
        ("--query-file", &["--query-file", file.as_str()][..]),
        ("--trade", &["--trade", "abc"][..]),
        ("--trade-file", &["--trade-file", file.as_str()][..]),
        ("--realm", &["--realm", "pc"][..]),
        ("--membership", &["--membership", "live"][..]),
    ] {
        let args: Vec<&str> = [&["search", "--saved", id.as_str()][..], words].concat();
        let doc = json_of(&base, &args);
        assert_eq!(doc["kind"], "view", "{beside}: {doc}");
        assert!(
            doc["error"]
                .as_str()
                .unwrap()
                .contains(&format!("{beside} has nowhere to stand beside it")),
            "{beside}: {doc}"
        );
    }
    for whole in [&["--next", "acq1.x"][..], &["--request", file.as_str()][..]] {
        let args: Vec<&str> = [&["search", "--saved", id.as_str()][..], whole].concat();
        let doc = json_of(&base, &args);
        assert!(
            doc["error"]
                .as_str()
                .unwrap()
                .contains("--saved has nowhere to stand beside it"),
            "{whole:?}: {doc}"
        );
    }
}

/// The commands a text prints between backticks, in order.
fn printed_commands(text: &str) -> Vec<String> {
    text.split('`')
        .skip(1)
        .step_by(2)
        .filter(|c| c.starts_with("acq "))
        .map(str::to_string)
        .collect()
}

/// The review of the build (39f3b7dc), finding 4: with two accounts
/// known and `ACQ_ACCOUNT` unset, the text of `acq --account <name> saved
/// show` and of a write prints its run as that account, and the run and
/// the clear it prints run as printed. Finding 2 at the terminal: a
/// conflict over a row a later build wrote, at revision 3 under
/// `--if-revision 1`, offers the row's show, which runs as printed.
#[test]
fn the_printed_run_carries_the_account_and_every_offer_runs_with_two_accounts() {
    let scratch = harness::scratch("savedacc");
    let base = scratch.0.clone();
    let mock = world(&base);
    acquisition_store::Index::load(&mock)
        .unwrap()
        .record_login("Bob#9", "u-bob", false, harness::now(), &[])
        .unwrap();
    let alice = "Alice#1234";
    let written = acq(
        &base,
        &[
            "--account",
            alice,
            "saved",
            "set",
            "--name",
            "rings",
            "--realm",
            "pc",
            "rarity=rare",
        ],
    );
    assert!(written.status.success(), "{}", stderr(&written));
    let id = id_named(&base, "rings");
    let shown = acq(&base, &["--account", alice, "saved", "show", &id]);
    let text = String::from_utf8_lossy(&shown.stdout).into_owned()
        + &String::from_utf8_lossy(&written.stdout);
    let commands = printed_commands(&text);
    let runs: Vec<&String> = commands.iter().filter(|c| c.contains("--saved")).collect();
    assert_eq!(runs.len(), 2, "{text}");
    for run in runs {
        assert!(run.starts_with("acq --account"), "{run}");
        let answer = run_printed(&base, run);
        assert!(answer.get("error").is_none(), "{run}: {answer}");
    }
    let clear = commands
        .iter()
        .find(|c| c.starts_with("acq saved clear"))
        .unwrap_or_else(|| panic!("{text}"));
    assert!(run_printed(&base, clear)["written"].is_null());

    // a row a later build wrote, at revision 3: the conflict offers its show
    let back = acq(
        &base,
        &[
            "saved",
            "set",
            "--name",
            "amulets",
            "--realm",
            "pc",
            "rarity=rare",
        ],
    );
    assert!(back.status.success());
    let id = id_named(&base, "amulets");
    assert!(
        acq(&base, &["saved", "set", "--id", &id, "--name", "x"])
            .status
            .success()
    );
    User::open_for(&mock)
        .unwrap()
        .put::<NewerSaved>(
            SAVED_SEARCH_SCOPE,
            &id,
            &json!({ "version": 2, "name": "x", "text": "x", "scope": {} }),
            Some(2),
            &Provenance::via("future"),
        )
        .unwrap();
    let conflict = json_of(
        &base,
        &[
            "saved",
            "set",
            "--id",
            &id,
            "--name",
            "renamed",
            "--if-revision",
            "1",
        ],
    );
    assert_eq!(
        (&conflict["kind"], &conflict["revision"]),
        (&json!("conflict"), &json!(3)),
        "{conflict}"
    );
    let [offer] = conflict["offers"].as_array().unwrap().as_slice() else {
        panic!("{conflict}")
    };
    let line = harness::offered_line(offer);
    assert_eq!(line, format!("acq saved show {id}"));
    assert_eq!(run_printed(&base, &line)["revision"], 3);
}

/// A saved search as a later build would write it, through the store's
/// door: a kind of the same name whose parse takes any shape.
#[derive(serde::Serialize)]
#[serde(transparent)]
struct NewerSaved(Value);

impl acquisition_store::IntentValue for NewerSaved {
    const KIND: &'static str = "saved-search";
    const VERSION: i64 = 2;
    fn parse(value: &Value) -> Result<Self, String> {
        Ok(NewerSaved(value.clone()))
    }
}
