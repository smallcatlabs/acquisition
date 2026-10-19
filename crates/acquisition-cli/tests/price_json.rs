//! Process-level pin of the `--json` stdout contract for the pricing
//! surface (C53: text is a function of JSON — the documents `acq price
//! list|show --json` print are the [`ListView`] and [`ShowView`] the
//! text renderers read, whole; `status --json` is the report without
//! its listings). Facts and intent are seeded through the store crate,
//! as `plan_json.rs` does, so no daemon runs: `ACQ_NO_SPAWN=1` makes any
//! contact an error.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use acquisition_plan::listing::{ListView, ListingReport, ShowView};
use acquisition_plan::price::{Buyout, PriceWrite};
use acquisition_store::{
    Annotations, Endpoint, Index, IntentValue, Provenance, Store, account_path,
};
use serde_json::{Value, json};

const USER: &str = "Alice#1234";
const UUID: &str = "u-price";

fn acq(base: &Path, args: &[&str]) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_acq"));
    cmd.args(args)
        .env("ACQ_STORE_DIR", base)
        .env("ACQ_NO_KEYRING", "1")
        .env("ACQ_NO_SPAWN", "1");
    for var in [
        "ACQ_GGG",
        "ACQ_ACCOUNT",
        "ACQ_TRIPWIRE",
        "ACQ_MAX_SENDS",
        "ACQ_JOURNAL",
        "ACQ_IDLE_SHUTDOWN",
    ] {
        cmd.env_remove(var);
    }
    cmd.env("ACQ_PROVIDER", "mock");
    cmd.output().expect("spawning acq")
}

fn sole_json(out: &Output) -> Value {
    let stdout = String::from_utf8(out.stdout.clone()).expect("stdout is UTF-8");
    serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("stdout is not exactly one JSON document ({e}):\n{stdout}"))
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// A folder holding a public priced tab with two items (one noted, one
/// with a row), a map tab with a substash holding one item, a character
/// wearing one; a row on the folder and one naming nothing here.
fn seed(base: &Path) -> PathBuf {
    let mock = base.join("mock");
    std::fs::create_dir_all(&mock).unwrap();
    let at = acquisition_store::now();
    let mut index = Index::load(&mock).unwrap();
    index.record_login(USER, UUID, false, at).unwrap();
    let mut store = Store::open(&account_path(&mock, USER)).unwrap();
    let record = |store: &mut Store, ep: Endpoint, params: Value, body: Value| {
        store.record(&ep, &params, 200, &body, at).unwrap();
    };
    record(
        &mut store,
        Endpoint::Profile,
        json!({}),
        json!({ "uuid": UUID, "name": USER }),
    );
    let stashes = |realm: &str, league: &str| Endpoint::Stashes {
        realm: realm.into(),
        league: league.into(),
    };
    record(
        &mut store,
        stashes("pc", "Standard"),
        json!({ "league": "Standard" }),
        json!({ "stashes": [
            { "id": "f1", "name": "Sale", "type": "Folder", "index": 0,
              "children": [ { "id": "c1", "name": "~price 3 chaos", "type": "PremiumStash", "index": 1, "metadata": { "public": true } } ] },
            { "id": "m1", "name": "Maps", "type": "MapStash", "index": 2 } ] }),
    );
    let item = |id: &str, note: Option<&str>| {
        let mut v = json!({ "id": id, "name": "", "typeLine": "Chaos Orb", "baseType": "Chaos Orb", "x": 0, "y": 0, "stackSize": 5 });
        if let Some(n) = note {
            v["note"] = json!(n);
        }
        v
    };
    let stash = |id: &str, sub: Option<&str>| Endpoint::Stash {
        realm: "pc".into(),
        league: "Standard".into(),
        id: id.into(),
        sub: sub.map(str::to_string),
    };
    record(
        &mut store,
        stash("c1", None),
        json!({}),
        json!({ "stash": { "id": "c1", "name": "~price 3 chaos", "type": "PremiumStash",
                            "items": [ item("i-noted", Some("~price 5 chaos")), item("i-plain", None) ] } }),
    );
    record(
        &mut store,
        stash("m1", None),
        json!({}),
        json!({ "stash": { "id": "m1", "name": "Maps", "type": "MapStash", "items": [],
                            "children": [ { "id": "s1", "name": "1", "type": "MapStash", "metadata": { "items": 1 } } ] } }),
    );
    record(
        &mut store,
        stash("m1", Some("s1")),
        json!({}),
        json!({ "stash": { "id": "s1", "name": "1", "type": "MapStash", "items": [ item("i-map", None) ] } }),
    );
    record(
        &mut store,
        Endpoint::Characters { realm: "pc".into() },
        json!({ "realm": "pc" }),
        json!({ "characters": [ { "id": "ch1", "name": "Exile", "league": "Standard" } ] }),
    );
    record(
        &mut store,
        Endpoint::Character {
            realm: "pc".into(),
            name: "Exile".into(),
        },
        json!({ "realm": "pc", "name": "Exile" }),
        json!({ "character": { "id": "ch1", "name": "Exile", "league": "Standard",
                                "equipment": [ item("i-worn", Some("~b/o 2 divine")) ], "inventory": [] } }),
    );
    let mut a = Annotations::open_for(&mock, UUID).unwrap();
    let via = Provenance::via("test");
    a.put::<Buyout>(
        "item",
        "i-plain",
        &json!({ "version": 1, "type": "exact", "amount": "3", "currency": "chaos" }),
        None,
        &via,
    )
    .unwrap();
    a.put::<Buyout>(
        "tab",
        "pc/f1",
        &json!({ "version": 1, "type": "skip" }),
        None,
        &via,
    )
    .unwrap();
    a.put::<Buyout>(
        "item",
        "i-elsewhere",
        &json!({ "version": 1, "type": "skip" }),
        None,
        &via,
    )
    .unwrap();
    mock
}

#[test]
fn price_json_documents_are_the_views_the_text_reads() {
    let base = std::env::temp_dir().join(format!(
        "acq-price-json-{}-{}",
        std::process::id(),
        acquisition_store::now()
    ));
    seed(&base);

    // status: the report without listings, re-read exactly.
    let out = acq(&base, &["price", "status", "--json"]);
    assert!(out.status.success(), "{}", stderr(&out));
    let report: ListingReport = serde_json::from_value(sole_json(&out)).unwrap();
    assert!(report.listings.is_empty());
    assert_eq!(report.counts.items, 4);
    assert_eq!(report.counts.containers, 5);
    assert_eq!(report.rows.total, 3);
    assert_eq!(report.rows.applied, 2);
    assert_eq!(report.rows.unmatched.len(), 1);

    // list: the view carries the containers its text names.
    let out = acq(&base, &["price", "list", "--json"]);
    assert!(out.status.success(), "{}", stderr(&out));
    let view: ListView = serde_json::from_value(sole_json(&out)).unwrap();
    // The unpriced map item and the character's noted item are relation
    // `none` (a character's items are not indexed, C81): left out by default.
    assert_eq!(view.items.len(), 2);
    // The containers on the selected items' chains, in the report's
    // order: the folder (on the chain), the tab, the character.
    let containers: Vec<&str> = view
        .containers
        .iter()
        .map(|c| c.subject.name.as_str())
        .collect();
    assert_eq!(containers, ["Sale", "~price 3 chaos"]);
    assert_eq!(view.items_on_record, 4);
    // The same selection in text names the same containers, in order.
    let out = acq(&base, &["price", "list"]);
    let text = String::from_utf8_lossy(&out.stdout);
    let pos = |s: &str| {
        text.find(s)
            .unwrap_or_else(|| panic!("{s:?} not in\n{text}"))
    };
    assert!(
        pos("~price 3 chaos (3 chaos, public)") < pos("next:"),
        "{text}"
    );
    assert!(!text.contains("Sale  "), "{text}");
    // The residue items under `--relation none`: a substash labelled under
    // its parent's name, which the view carries; the character last.
    let out = acq(&base, &["price", "list", "--relation", "none"]);
    let text = String::from_utf8_lossy(&out.stdout);
    let pos = |s: &str| {
        text.find(s)
            .unwrap_or_else(|| panic!("{s:?} not in\n{text}"))
    };
    assert!(
        pos("Maps / 1 (not public)  1 item: 1 unlisted  substash/pc/m1/s1")
            < pos("Exile  1 item: 1 unlisted"),
        "{text}"
    );

    // --covered-by a folder: its tab's items, through the chain (C70);
    // --in the folder: nothing physically there.
    let out = acq(
        &base,
        &["price", "list", "--covered-by", "tab/pc/f1", "--json"],
    );
    assert!(out.status.success(), "{}", stderr(&out));
    let view: ListView = serde_json::from_value(sole_json(&out)).unwrap();
    let ids: Vec<String> = view
        .items
        .iter()
        .map(|l| l.subject.target.to_string())
        .collect();
    assert_eq!(ids, ["item/i-noted", "item/i-plain"]);
    let out = acq(
        &base,
        &["price", "list", "--in", "tab/pc/f1", "--relation", "none"],
    );
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.starts_with(
            "4 items on record, none match for Standard with relation none in tab/pc/f1\n"
        ),
        "{text}"
    );

    // show: an item's view carries its container; a folder's carries the
    // covered set; both re-read exactly.
    let out = acq(&base, &["price", "show", "item/i-noted", "--json"]);
    assert!(out.status.success(), "{}", stderr(&out));
    let view: ShowView = serde_json::from_value(sole_json(&out)).unwrap();
    assert_eq!(
        view.container.as_ref().map(|c| c.subject.name.as_str()),
        Some("~price 3 chaos")
    );
    assert_eq!(
        view.listing.game.note.as_ref().map(|n| n.text.as_str()),
        Some("~price 5 chaos")
    );
    assert_eq!(
        view.listing.manual.as_ref().map(|m| m.from.to_string()),
        Some("tab/pc/f1".into())
    );
    // C81: the note beats the folder's row; the site's view of it is the price.
    assert_eq!(
        view.listing.effective.side,
        Some(acquisition_plan::listing::Side::Game)
    );
    assert_eq!(view.listing.effective.to_string(), "5 chaos");
    let out = acq(&base, &["price", "show", "tab/pc/f1", "--json"]);
    let view: ShowView = serde_json::from_value(sole_json(&out)).unwrap();
    assert!(view.items_here.is_empty());
    assert_eq!(view.items_covered_below.len(), 2);
    let out = acq(&base, &["price", "show", "tab/pc/f1"]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.contains("items here: none on record\nitems covered through children: 2 items"),
        "{text}"
    );

    // A container not on record, or another realm's address, is a
    // refusal naming the facts — never an empty selection.
    let out = acq(&base, &["price", "list", "--in", "tab/pc/nope"]);
    assert!(!out.status.success());
    assert!(
        stderr(&out).contains("tab/pc/nope is not in the facts for Standard"),
        "{}",
        stderr(&out)
    );
    let out = acq(
        &base,
        &["price", "list", "--in", "tab/xbox/c1", "--realm", "pc"],
    );
    assert!(!out.status.success());
    assert!(
        stderr(&out).contains("tab/xbox/c1 is a xbox address; pc was given"),
        "{}",
        stderr(&out)
    );
    let out = acq(&base, &["price", "list", "--in", "tab/xbox/c1"]);
    assert!(!out.status.success());
    assert!(
        stderr(&out).contains("not in the facts for xbox/Standard"),
        "{}",
        stderr(&out)
    );

    // A league with no facts still shows the intent rows (C35).
    let out = acq(&base, &["price", "status", "--league", "Hardcore"]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.starts_with(
            "no items on record for Hardcore: 0 tabs and characters listed, none fetched with items\n\
             rows: 0 of 3 apply here; 1 name nothing in the facts, 0 unreadable, 2 for other leagues, 0 for other realms\n"
        ),
        "{text}"
    );

    let _ = std::fs::remove_dir_all(&base);
}

/// The write receipt (C78): `set --json` and `clear --json` print the
/// [`PriceWrite`] — the row written and the row it replaced — `show`
/// reads the write back, a stale `--if-revision` is a conflict naming the
/// current revision with nothing landed, a target with no row refuses a
/// clear by saying so, and a new price never names a retired tag.
#[test]
fn price_set_and_clear_print_the_receipt_and_show_reads_it_back() {
    let base = std::env::temp_dir().join(format!(
        "acq-price-write-{}-{}",
        std::process::id(),
        acquisition_store::now()
    ));
    seed(&base);
    let receipt = |out: &Output| -> PriceWrite {
        assert!(out.status.success(), "{}", stderr(out));
        serde_json::from_value(sole_json(out)).unwrap()
    };
    let manual = |out: &Output| -> Option<(Buyout, i64)> {
        assert!(out.status.success(), "{}", stderr(out));
        let view: ShowView = serde_json::from_value(sole_json(out)).unwrap();
        view.listing.manual.map(|m| (m.value, m.revision))
    };

    // A create: the amount lands canonical, the channel is the CLI's.
    let out = acq(
        &base,
        &[
            "price",
            "set",
            "item/i-map",
            "exact",
            "12.50",
            "chaos",
            "--json",
        ],
    );
    let w = receipt(&out);
    assert_eq!(w.target.to_string(), "item/i-map");
    assert!(w.prior.is_none());
    let written = w.written.unwrap();
    // The receipt says whether the facts hold the target (`in_facts`); a
    // target they do not is noted and lands all the same (C64; the
    // reading-1 question, answered 2026-09-12): true on the seeded item
    // above, false on a typo'd id, in JSON and as the text's note line.
    assert_eq!(sole_json(&out)["in_facts"], json!(true));
    let out = acq(
        &base,
        &[
            "price",
            "set",
            "item/i-typo",
            "exact",
            "1",
            "chaos",
            "--json",
        ],
    );
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(sole_json(&out)["in_facts"], json!(false));
    let out = acq(&base, &["price", "clear", "item/i-typo"]);
    assert!(out.status.success(), "{}", stderr(&out));
    let out = acq(
        &base,
        &["price", "set", "item/i-typo", "exact", "1", "chaos"],
    );
    assert!(out.status.success(), "{}", stderr(&out));
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.starts_with("item/i-typo: 1 chaos (revision "),
        "{text}"
    );
    assert!(
        text.ends_with(
            "), was unset\n\
             note: the facts hold no item/i-typo (not fetched yet, or a typo); the row stands, \
             and `acq price status` counts it under \"name nothing in the facts\"\n\
             next: `acq price show item/i-typo` reads it beside the game side\n"
        ),
        "{text}"
    );
    let out = acq(&base, &["price", "clear", "item/i-typo"]);
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!((written.revision, written.written_via.as_str()), (1, "cli"));
    assert_eq!(
        written.value,
        json!({ "version": 1, "type": "exact", "amount": "12.5", "currency": "chaos" })
    );
    let (value, revision) =
        manual(&acq(&base, &["price", "show", "item/i-map", "--json"])).unwrap();
    assert_eq!((value.to_string(), revision), ("12.5 chaos".into(), 1));

    // A replacement at the reviewed revision returns the prior row.
    let w = receipt(&acq(
        &base,
        &[
            "price",
            "set",
            "item/i-map",
            "b/o",
            "1/5",
            "divine",
            "--if-revision",
            "1",
            "--json",
        ],
    ));
    assert_eq!(w.prior.as_ref().map(|r| r.revision), Some(1));
    assert_eq!(w.prior.unwrap().value, written.value);
    assert_eq!(w.written.as_ref().map(|r| r.revision), Some(2));

    // A stale revision conflicts, naming the current one; nothing landed.
    let out = acq(
        &base,
        &["price", "set", "item/i-map", "skip", "--if-revision", "1"],
    );
    assert!(!out.status.success());
    assert!(
        stderr(&out).contains("item/i-map has a price at revision 2 (re-read and retry)"),
        "{}",
        stderr(&out)
    );
    let (value, revision) =
        manual(&acq(&base, &["price", "show", "item/i-map", "--json"])).unwrap();
    assert_eq!((value.to_string(), revision), ("1/5 divine b/o".into(), 2));

    // A retired tag is refused for a new price, in words.
    let out = acq(
        &base,
        &["price", "set", "item/i-map", "exact", "1", "chisel"],
    );
    assert!(!out.status.success());
    assert!(
        stderr(&out).contains("currency \"chisel\" is retired"),
        "{}",
        stderr(&out)
    );

    // The text receipt of a clear ends with the command that puts it back.
    let out = acq(&base, &["price", "clear", "item/i-map"]);
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "item/i-map: cleared, was 1/5 divine b/o (revision 2)\n\
         next: `acq price set item/i-map negotiable 1/5 divine` puts it back\n"
    );
    assert!(manual(&acq(&base, &["price", "show", "item/i-map", "--json"])).is_none());
    let out = acq(&base, &["price", "clear", "item/i-map"]);
    assert!(!out.status.success());
    assert!(
        stderr(&out).contains("item/i-map's price was cleared (revision 3): nothing to clear"),
        "{}",
        stderr(&out)
    );
    let out = acq(&base, &["price", "status", "--json"]);
    let report: ListingReport = serde_json::from_value(sole_json(&out)).unwrap();
    assert_eq!(
        report.rows.total, 3,
        "the seed's rows, the cleared one gone"
    );

    // Putting it back is a create over the tombstone: the revision carries on.
    let w = receipt(&acq(
        &base,
        &[
            "price",
            "set",
            "item/i-map",
            "negotiable",
            "1/5",
            "divine",
            "--json",
        ],
    ));
    assert!(w.prior.is_none());
    assert_eq!(w.written.map(|r| r.revision), Some(4));

    let _ = std::fs::remove_dir_all(&base);
}

/// The release seat: `price list` had no bound, so a run asking for a
/// league's conflicts got 49 to 67 KB, cut by its harness after the
/// first item. `--limit N` holds the first N in listing order, the
/// document saying how many the selection holds (`total`) and how many
/// it lists (`listed`), the text "N of M", and the containers are the
/// listed items' alone.
#[test]
fn price_list_is_bounded_by_a_limit_and_says_how_many_of_how_many() {
    let base = std::env::temp_dir().join(format!(
        "acq-price-limit-{}-{}",
        std::process::id(),
        acquisition_store::now()
    ));
    let _ = std::fs::remove_dir_all(&base);
    seed(&base);
    let whole = sole_json(&acq(
        &base,
        &["price", "list", "--effective", "none", "--json"],
    ));
    let all = whole["items"].as_array().unwrap().len();
    assert!(all >= 2, "{whole}");
    let out = acq(
        &base,
        &[
            "price",
            "list",
            "--effective",
            "none",
            "--limit",
            "1",
            "--json",
        ],
    );
    assert!(out.status.success(), "{}", stderr(&out));
    let limited = sole_json(&out);
    assert_eq!(limited["items"].as_array().unwrap().len(), 1, "{limited}");
    assert_eq!(limited["items"][0], whole["items"][0]);
    assert_eq!(
        (&limited["total"], &limited["listed"]),
        (&json!(all), &json!(1))
    );
    assert_eq!(limited["filter"]["limit"], json!(1));
    let first_chain = limited["items"][0]["chain"].as_array().unwrap().clone();
    for c in limited["containers"].as_array().unwrap() {
        assert!(first_chain.contains(&c["subject"]["target"]), "{c}");
    }
    let view: ListView = serde_json::from_value(limited).unwrap();
    assert_eq!(view.items.len(), 1);

    let out = acq(
        &base,
        &["price", "list", "--effective", "none", "--limit", "1"],
    );
    let said = String::from_utf8_lossy(&out.stdout);
    assert!(
        said.starts_with(&format!("1 of {all} items listed in Standard")),
        "{said}"
    );
    assert!(
        said.contains(&format!("`--limit {all}` lists every one")),
        "{said}"
    );
    let _ = std::fs::remove_dir_all(&base);
}

/// The release seat: a write refused for its revision had no `kind`,
/// the current revision only in its prose, and the store's words —
/// `annotation item/…/buyout is at revision 4`, and on a cleared row
/// `annotation no longer exists`, which hid the clear and the revision
/// it still counts. Staged as met: a stale revision over a live row, and
/// a revision named over a cleared one; each is `kind: "conflict"` with
/// the current `revision`, in the verb's words. A target the facts do
/// not hold is `kind: "not_found"`.
#[test]
fn a_write_refused_for_its_revision_is_a_conflict_naming_it_in_the_verbs_words() {
    let base = std::env::temp_dir().join(format!(
        "acq-price-conflict-{}-{}",
        std::process::id(),
        acquisition_store::now()
    ));
    let _ = std::fs::remove_dir_all(&base);
    seed(&base);
    let ok = |args: &[&str]| {
        let out = acq(&base, args);
        assert!(out.status.success(), "{args:?}: {}", stderr(&out));
    };
    let refused = |args: &[&str]| -> Value {
        let out = acq(&base, args);
        assert!(!out.status.success(), "{args:?} landed");
        let doc = sole_json(&out);
        let said = doc["error"].as_str().unwrap_or_default().to_string();
        for word in ["annotation", "buyout", "item/i-map/"] {
            assert!(!said.contains(word), "{said}");
        }
        doc
    };
    ok(&["price", "set", "item/i-map", "exact", "2", "divine"]);
    ok(&[
        "price",
        "set",
        "item/i-map",
        "exact",
        "3",
        "divine",
        "--if-revision",
        "1",
    ]);
    let doc = refused(&[
        "price",
        "set",
        "item/i-map",
        "exact",
        "7",
        "divine",
        "--if-revision",
        "1",
        "--json",
    ]);
    assert_eq!(
        doc,
        json!({
            "error": "item/i-map has a price at revision 2 (re-read and retry)",
            "kind": "conflict",
            "revision": 2,
        })
    );
    let doc = refused(&[
        "price",
        "clear",
        "item/i-map",
        "--if-revision",
        "1",
        "--json",
    ]);
    assert_eq!(
        (&doc["kind"], &doc["revision"]),
        (&json!("conflict"), &json!(2))
    );

    ok(&["price", "clear", "item/i-map", "--if-revision", "2"]);
    let doc = refused(&[
        "price",
        "set",
        "item/i-map",
        "exact",
        "1",
        "divine",
        "--if-revision",
        "99",
        "--json",
    ]);
    assert_eq!(
        doc,
        json!({
            "error": "item/i-map's price was cleared (revision 3); a new price names no revision",
            "kind": "conflict",
            "revision": 3,
        })
    );
    let doc = refused(&["price", "clear", "item/i-map", "--json"]);
    assert_eq!(
        (&doc["kind"], &doc["revision"]),
        (&json!("conflict"), &json!(3))
    );
    assert!(
        doc["error"]
            .as_str()
            .unwrap()
            .starts_with("item/i-map's price was cleared (revision 3)"),
        "{doc}"
    );
    let doc = refused(&["price", "show", "item/i-nowhere", "--json"]);
    assert_eq!(doc["kind"], json!("not_found"), "{doc}");
    let _ = std::fs::remove_dir_all(&base);
}

/// A `buyout` row as a newer build would write it: this build's parse
/// refuses the stamp, the listing state reports it unreadable, and the
/// blind `set` and `clear` must refuse to touch it.
#[derive(serde::Serialize)]
#[serde(transparent)]
struct NewerBuyout(Value);

impl IntentValue for NewerBuyout {
    const KIND: &'static str = "buyout";
    const VERSION: i64 = 99;
    fn parse(value: &Value) -> Result<Self, String> {
        Ok(NewerBuyout(value.clone()))
    }
}

/// A row this build cannot read is never replaced or cleared blind: both
/// refuse naming the revision and the reviewed path, and the row is
/// untouched; `--if-revision` replaces it deliberately, and the receipt
/// then shows the prior as its JSON and promises no undo.
#[test]
fn price_set_and_clear_refuse_a_row_this_build_cannot_read_unless_the_revision_is_named() {
    let base = std::env::temp_dir().join(format!(
        "acq-price-newer-{}-{}",
        std::process::id(),
        acquisition_store::now()
    ));
    let mock = seed(&base);
    let newer = json!({ "version": 2, "type": "auction", "reserve": "5 chaos" });
    {
        let mut a = Annotations::open_for(&mock, UUID).unwrap();
        a.put::<NewerBuyout>("item", "i-map", &newer, None, &Provenance::via("future"))
            .unwrap();
    }
    let out = acq(&base, &["price", "status", "--json"]);
    let report: ListingReport = serde_json::from_value(sole_json(&out)).unwrap();
    assert_eq!(report.rows.unreadable.len(), 1);

    for blind in [
        vec!["price", "set", "item/i-map", "skip"],
        vec!["price", "clear", "item/i-map"],
    ] {
        let out = acq(&base, &blind);
        assert!(!out.status.success(), "{blind:?} must refuse");
        let err = stderr(&out);
        assert!(
            err.contains("item/i-map holds a value this build cannot read (revision 1: buyout declares version 2, newer than this build's v1)")
                && err.contains("`--if-revision 1` replaces it deliberately"),
            "{blind:?}: {err}"
        );
    }
    let a = Annotations::open_for(&mock, UUID).unwrap();
    let row = a.get("item", "i-map", "buyout").unwrap().unwrap();
    assert_eq!((row.revision, &row.value), (1, &newer), "untouched");
    drop(a);

    // The reviewed path: the receipt shows the JSON and promises no undo.
    let out = acq(
        &base,
        &["price", "set", "item/i-map", "skip", "--if-revision", "1"],
    );
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        format!(
            "item/i-map: skip (revision 2), was an unreadable value {newer} (revision 1)\n\
             next: `acq price show item/i-map` reads it beside the game side; \
             the prior cannot be put back by this build (its JSON is in --json)\n"
        )
    );
    let out = acq(
        &base,
        &[
            "price",
            "set",
            "item/i-map",
            "skip",
            "--if-revision",
            "2",
            "--json",
        ],
    );
    let w: PriceWrite = serde_json::from_value(sole_json(&out)).unwrap();
    assert_eq!(w.prior.map(|r| r.revision), Some(2));

    let _ = std::fs::remove_dir_all(&base);
}

/// C35, C53 — a row on a target the facts do not hold is intent to look
/// at, so `status` lists it whole, in the manual side's shape (the value,
/// the row's own target, its revision, when and through what it was
/// written) where it once listed the bare address; the text prints the
/// value and revision on the row's line; and `show`, a subject of the
/// facts, refuses the target saying it has a row of its own and naming
/// the command that prints the row — run here as the refusal types it,
/// with the realm the target carries, since `status` reads pc without
/// one and counts an xbox row under `other_realm`.
#[test]
fn a_row_on_a_target_the_facts_do_not_hold_is_listed_whole_and_show_says_where() {
    let base = std::env::temp_dir().join(format!(
        "acq-price-orphan-{}-{}",
        std::process::id(),
        acquisition_store::now()
    ));
    let mock = seed(&base);
    Annotations::open_for(&mock, UUID)
        .unwrap()
        .put::<Buyout>(
            "tab",
            "xbox/orphan",
            &json!({ "version": 1, "type": "skip" }),
            None,
            &Provenance::via("test"),
        )
        .unwrap();

    let report = sole_json(&acq(&base, &["price", "status", "--json"]));
    let unmatched = &report["rows"]["unmatched"];
    let updated_at = unmatched[0]["updated_at"].as_i64();
    assert!(updated_at.is_some(), "{unmatched}");
    assert_eq!(
        unmatched,
        &json!([{
            "value": { "version": 1, "type": "skip" },
            "from": { "scope": "item", "id": "i-elsewhere" },
            "inherited": false,
            "revision": 1,
            "updated_at": updated_at,
            "written_via": "test",
        }]),
        "{report}"
    );

    // The refusal's command, run as typed, prints the row's line.
    for (target, said_where, line) in [
        (
            "item/i-elsewhere",
            "item/i-elsewhere is not in the facts for Standard",
            "  item/i-elsewhere: skip, revision 1\n",
        ),
        (
            "tab/xbox/orphan",
            "tab/xbox/orphan is not in the facts for xbox/Standard",
            "  tab/xbox/orphan: skip, revision 1\n",
        ),
    ] {
        let out = acq(&base, &["price", "show", target, "--json"]);
        assert!(!out.status.success());
        let said = sole_json(&out)["error"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        assert!(
            said.contains(said_where)
                && said.contains("it has a price row of its own at revision 1"),
            "{said}"
        );
        let command = said
            .split('`')
            .find(|w| w.starts_with("acq price status"))
            .unwrap_or_else(|| panic!("no `acq price status …` command in {said}"));
        let words: Vec<&str> = command.split_whitespace().skip(1).collect();
        let out = acq(&base, &words);
        let text = String::from_utf8_lossy(&out.stdout);
        assert!(
            out.status.success()
                && text.contains("rows naming nothing in the facts:\n")
                && text.contains(line),
            "`{command}` printed:\n{text}"
        );
    }
    // A target with no row of its own is refused as before.
    let out = acq(&base, &["price", "show", "item/i-nowhere", "--json"]);
    let said = sole_json(&out)["error"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    assert!(!said.contains("price row of its own"), "{said}");

    let _ = std::fs::remove_dir_all(&base);
}
