//! Process-level pin of the `--json` stdout contract for the pricing
//! surface (C53: text is a function of JSON — the documents `acq price
//! list|show --json` print are the [`ListView`] and [`ShowView`] the
//! text renderers read, whole; `status --json` is the report without
//! its listings). Facts and intent are seeded through the store crate,
//! as `plan_json.rs` does, so no daemon runs: `ACQ_NO_SPAWN=1` makes any
//! contact an error.

use std::path::{Path, PathBuf};
use std::process::Output;

use acquisition_plan::listing::{ListView, ListingReport, ShowView};
use acquisition_plan::price::{Buyout, PriceWrite};
use acquisition_store::{
    Annotations, Endpoint, Index, IntentValue, Provenance, Store, account_path,
};
use serde_json::{Value, json};

use crate::harness;

use harness::{Transcript, golden, offline_acq as acq, sole_json};

const USER: &str = "Alice#1234";
const UUID: &str = "u-price";

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
    index.record_login(USER, UUID, false, at, &[]).unwrap();
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
    let scratch = harness::scratch("price-json");
    let base = scratch.0.clone();
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
    let mut t = Transcript::offline(&base);
    t.run(
        &["price", "list", "--in", "tab/pc/f1", "--relation", "none"],
        &["none match for Standard with relation none in tab/pc/f1"],
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
    t.run(
        &["price", "show", "tab/pc/f1"],
        &["items covered through children: 2 items"],
    );

    // A container not on record, or another realm's address, is a
    // refusal naming the facts — never an empty selection.
    for (args, said) in [
        (
            &["price", "list", "--in", "tab/pc/nope"][..],
            "tab/pc/nope is not in the facts for Standard",
        ),
        (
            &["price", "list", "--in", "tab/xbox/c1", "--realm", "pc"][..],
            "tab/xbox/c1 is a xbox address; pc was given",
        ),
        (
            &["price", "list", "--in", "tab/xbox/c1"][..],
            "not in the facts for xbox/Standard",
        ),
    ] {
        let out = t.run(args, &[said]);
        assert!(!out.status.success(), "{args:?}");
    }

    // A league with no facts still shows the intent rows (C35).
    t.run(
        &["price", "status", "--league", "Hardcore"],
        &["no items on record for Hardcore", "2 for other leagues"],
    );
    golden(file!(), "list_show_and_refusals", &t);
}

/// The write receipt (C78): `set --json` and `clear --json` print the
/// [`PriceWrite`] — the row written and the row it replaced — `show`
/// reads the write back, a stale `--if-revision` is a conflict naming the
/// current revision with nothing landed, a target with no row refuses a
/// clear by saying so, and a new price never names a retired tag.
#[test]
fn price_set_and_clear_print_the_receipt_and_show_reads_it_back() {
    let scratch = harness::scratch("price-write");
    let base = scratch.0.clone();
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
             next: `acq price show --account 'Alice#1234' item/i-typo` reads it beside the game side; \
             `acq price clear --account 'Alice#1234' item/i-typo --if-revision 3` undoes it\n"
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
    let mut t = Transcript::offline(&base);
    let out = t.run(
        &["price", "set", "item/i-map", "skip", "--if-revision", "1"],
        &["item/i-map has a price at revision 2 (re-read and retry)"],
    );
    assert!(!out.status.success());
    let (value, revision) =
        manual(&acq(&base, &["price", "show", "item/i-map", "--json"])).unwrap();
    assert_eq!((value.to_string(), revision), ("1/5 divine b/o".into(), 2));

    // A retired tag is refused for a new price, in words.
    let out = t.run(
        &["price", "set", "item/i-map", "exact", "1", "chisel"],
        &["currency \"chisel\" is retired"],
    );
    assert!(!out.status.success());

    // The text receipt of a clear ends with the command that puts it back.
    let out = t.run(
        &["price", "clear", "item/i-map"],
        &[
            "item/i-map: cleared, was 1/5 divine b/o (revision 2)",
            "puts it back",
        ],
    );
    assert!(out.status.success(), "{}", stderr(&out));
    assert!(manual(&acq(&base, &["price", "show", "item/i-map", "--json"])).is_none());
    let out = t.run(
        &["price", "clear", "item/i-map"],
        &["item/i-map's price was cleared (revision 3): nothing to clear"],
    );
    assert!(!out.status.success());
    golden(file!(), "a_write_walk_refused", &t);
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
}

/// The release seat: `price list` had no bound, so a run asking for a
/// league's conflicts got 49 to 67 KB, cut by its harness after the
/// first item. `--limit N` holds the first N in listing order, the
/// document saying how many the selection holds (`total`) and how many
/// it lists (`listed`), the text "N of M", and the containers are the
/// listed items' alone.
#[test]
fn price_list_is_bounded_by_a_limit_and_says_how_many_of_how_many() {
    let scratch = harness::scratch("price-limit");
    let base = scratch.0.clone();
    seed(&base);
    let whole = sole_json(&acq(
        &base,
        &["price", "list", "--effective", "none", "--json"],
    ));
    let all = whole["items"].as_array().unwrap().len();
    assert!(all >= 2, "{whole}");
    // the seed's selection is within the default, the store's bound (C53),
    // so the unlimited answer is the whole of it
    assert!(all <= acquisition_store::bound::LISTED, "{whole}");
    assert_eq!(whole["total"], json!(all));
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
    let scratch = harness::scratch("price-conflict");
    let base = scratch.0.clone();
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
    let scratch = harness::scratch("price-newer");
    let base = scratch.0.clone();
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

    let mut t = Transcript::offline(&base);
    for blind in [
        &["price", "set", "item/i-map", "skip"][..],
        &["price", "clear", "item/i-map"][..],
    ] {
        let out = t.run(
            blind,
            &[
                "item/i-map holds a value this build cannot read (revision 1: buyout declares version 2, newer than this build's v1)",
                "`--if-revision 1` replaces it deliberately",
            ],
        );
        assert!(!out.status.success(), "{blind:?} must refuse");
    }
    let a = Annotations::open_for(&mock, UUID).unwrap();
    let row = a.get("item", "i-map", "buyout").unwrap().unwrap();
    assert_eq!((row.revision, &row.value), (1, &newer), "untouched");
    drop(a);

    // The reviewed path: the receipt shows the JSON and promises no undo.
    let out = t.run(
        &["price", "set", "item/i-map", "skip", "--if-revision", "1"],
        &[
            &format!("was an unreadable value {newer} (revision 1)"),
            "the prior cannot be put back: this build cannot read it",
        ],
    );
    assert!(out.status.success(), "{}", stderr(&out));
    golden(file!(), "an_unreadable_row_refused_blind", &t);
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
}

/// C35, C53 — a row on a target the facts do not hold is intent to look
/// at, so `status` lists it whole, in the manual side's shape (the value,
/// the row's own target, its revision, when and through what it was
/// written) where it once listed the bare address; the text prints the
/// value and revision on the row's line; and `show`, a subject of the
/// facts, refuses the target saying it has a row of its own and naming
/// the command that prints the row — run here as the refusal types it,
/// with the realm the target carries, since `status` reads pc without
/// one and counts an xbox row under `other_realm`; `list --in` refuses
/// such a container the same way.
#[test]
fn a_row_on_a_target_the_facts_do_not_hold_is_listed_whole_and_show_says_where() {
    let scratch = harness::scratch("price-orphan");
    let base = scratch.0.clone();
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
        let words: Vec<&str> = command
            .split_whitespace()
            .skip(1)
            .map(|w| w.trim_matches('\''))
            .collect();
        let out = acq(&base, &words);
        let text = String::from_utf8_lossy(&out.stdout);
        assert!(
            out.status.success()
                && text.contains("rows naming nothing in the facts:\n")
                && text.contains(line),
            "`{command}` printed:\n{text}"
        );
    }
    // `list` refuses a container with a row of its own as `show` does,
    // naming the command that prints the row (C146; issue #11).
    let out = acq(
        &base,
        &["price", "list", "--in", "tab/xbox/orphan", "--json"],
    );
    assert!(!out.status.success());
    let said = sole_json(&out)["error"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    assert!(
        said.contains("tab/xbox/orphan is not in the facts for xbox/Standard")
            && said.contains(
                "it has a price row of its own at revision 1: `acq price status --expand --account 'Alice#1234' --realm xbox` lists it"
            ),
        "{said}"
    );
    // A target with no row of its own is refused as before.
    let out = acq(&base, &["price", "show", "item/i-nowhere", "--json"]);
    let said = sole_json(&out)["error"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    assert!(!said.contains("price row of its own"), "{said}");
}

/// The h2 review's first finding: with two accounts known, the command
/// a refusal names for a row of the target's own dropped the account the
/// verb ran as, and so was itself refused as ambiguous. Both callers of
/// the shared refusal, `show` and `list --in`, now name it, and the
/// command runs as printed (C121, an account that cannot be picked is
/// refused; C146, a count says what it counts).
#[test]
fn a_rows_refusal_names_the_account_it_ran_as_so_its_command_runs_with_two_known() {
    let scratch = harness::scratch("price-two-accounts");
    let base = scratch.0.clone();
    let mock = seed(&base);
    Index::load(&mock)
        .unwrap()
        .record_login("Bob#5678", "u-bob", false, acquisition_store::now(), &[])
        .unwrap();
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
    // Naming no account is ambiguous now, as the old command was.
    let out = acq(&base, &["price", "status", "--expand", "--realm", "xbox"]);
    assert!(!out.status.success(), "{}", stderr(&out));

    for args in [
        &["price", "show", "tab/xbox/orphan"][..],
        &["price", "list", "--in", "tab/xbox/orphan"][..],
    ] {
        let mut args = args.to_vec();
        args.extend(["--account", USER, "--json"]);
        let out = acq(&base, &args);
        assert!(!out.status.success());
        let said = sole_json(&out)["error"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let command = said
            .split('`')
            .find(|w| w.starts_with("acq price status"))
            .unwrap_or_else(|| panic!("{args:?}: no `acq price status …` command in {said}"));
        assert_eq!(
            command, "acq price status --expand --account 'Alice#1234' --realm xbox",
            "{args:?}"
        );
        let words: Vec<&str> = command
            .split_whitespace()
            .skip(1)
            .map(|w| w.trim_matches('\''))
            .collect();
        let out = acq(&base, &words);
        let text = String::from_utf8_lossy(&out.stdout);
        assert!(
            out.status.success() && text.contains("  tab/xbox/orphan: skip, revision 1\n"),
            "{args:?}: `{command}` printed:\n{text}{}",
            stderr(&out)
        );
    }
}

/// The h2 fix round's review: a row on a target another league's facts
/// hold was refused with `status --expand` in the league asked, which
/// counts the row under "other league" and never displays it. Both
/// callers of the shared refusal now name that league and print a `show`
/// there, as the account the verb ran as, and the command displays the
/// row as printed (C146, a count says what it counts; C121, an account
/// that cannot be picked is refused).
#[test]
fn a_rows_refusal_in_another_league_prints_the_show_that_displays_it() {
    let scratch = harness::scratch("price-other-league");
    let base = scratch.0.clone();
    let mock = seed(&base);
    Index::load(&mock)
        .unwrap()
        .record_login("Bob#5678", "u-bob", false, acquisition_store::now(), &[])
        .unwrap();
    let mut store = Store::open(&account_path(&mock, USER)).unwrap();
    store
        .record(
            &Endpoint::Stashes {
                realm: "xbox".into(),
                league: "Allflame".into(),
            },
            &json!({ "league": "Allflame" }),
            200,
            &json!({ "stashes": [ { "id": "t", "name": "Box", "type": "PremiumStash", "index": 0 } ] }),
            acquisition_store::now(),
        )
        .unwrap();
    Annotations::open_for(&mock, UUID)
        .unwrap()
        .put::<Buyout>(
            "tab",
            "xbox/t",
            &json!({ "version": 1, "type": "skip" }),
            None,
            &Provenance::via("test"),
        )
        .unwrap();

    for args in [
        &["price", "show", "tab/xbox/t"][..],
        &["price", "list", "--in", "tab/xbox/t"][..],
    ] {
        let mut args = args.to_vec();
        args.extend(["--account", "Alice", "--league", "Standard", "--json"]);
        let out = acq(&base, &args);
        assert!(!out.status.success());
        let said = sole_json(&out)["error"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        assert!(
            said.contains("tab/xbox/t is not in the facts for xbox/Standard")
                && said.contains(
                    "the facts hold it in Allflame: `--league` reads it there; \
                     it has a price row of its own at revision 1 in Allflame: "
                )
                && !said.contains("acq price status"),
            "{args:?}: {said}"
        );
        let command = said
            .split('`')
            .find(|w| w.starts_with("acq price show"))
            .unwrap_or_else(|| panic!("{args:?}: no `acq price show …` command in {said}"));
        assert_eq!(
            command, "acq price show tab/xbox/t --account 'Alice#1234' --league Allflame",
            "{args:?}"
        );
        let words: Vec<&str> = command
            .split_whitespace()
            .skip(1)
            .map(|w| w.trim_matches('\''))
            .collect();
        let out = acq(&base, &words);
        let text = String::from_utf8_lossy(&out.stdout);
        assert!(
            out.status.success()
                && text.starts_with("tab/xbox/t: Box (PremiumStash)\n")
                && text.contains("by hand: skip (own row, revision 1, "),
            "{args:?}: `{command}` printed:\n{text}{}",
            stderr(&out)
        );
    }
}

/// Review 8 of the release (2026-10-19): `price list --limit 0` said
/// "none listed by hand or in game" where the JSON said `total: 3,
/// listed: 0`. The text says none of how many, and why.
#[test]
fn price_list_with_a_limit_of_zero_says_none_of_the_total() {
    let scratch = harness::scratch("price-limit-zero");
    let base = scratch.0.clone();
    seed(&base);
    let doc = sole_json(&acq(&base, &["price", "list", "--limit", "0", "--json"]));
    let total = doc["total"].as_u64().unwrap_or(0);
    assert!(total > 0 && doc["listed"] == json!(0), "{doc}");
    let out = acq(&base, &["price", "list", "--limit", "0"]);
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        text.contains(&format!("0 of {total} items")) && !text.contains("none listed by hand"),
        "{text}"
    );
}

/// A `buyout` as an older build wrote it, the writer's rule not run: how
/// a row citing a since-retired tag got there (C67 exempts a stored row).
#[derive(serde::Serialize)]
#[serde(transparent)]
struct LegacyBuyout(Value);

impl IntentValue for LegacyBuyout {
    const KIND: &'static str = "buyout";
    const VERSION: i64 = 1;
    fn parse(value: &Value) -> std::result::Result<Self, String> {
        Buyout::parse(value)?;
        Ok(LegacyBuyout(value.clone()))
    }
}

/// Review 8: a replaced prior naming a retired currency was promised
/// back by the receipt's undo line, which the write refuses (C67). The
/// line promises no undo and says why, as it does for a value this
/// build cannot read.
#[test]
fn a_prior_naming_a_retired_currency_promises_no_undo() {
    let scratch = harness::scratch("price-retired");
    let base = scratch.0.clone();
    let mock = seed(&base);
    Annotations::open_for(&mock, UUID)
        .unwrap()
        .put::<LegacyBuyout>(
            "item",
            "i-map",
            &json!({ "version": 1, "type": "exact", "amount": "3", "currency": "chisel" }),
            None,
            &Provenance::via("old-build"),
        )
        .unwrap();
    let out = acq(
        &base,
        &["price", "set", "item/i-map", "exact", "2", "chaos"],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        !text.contains("3 chisel --if-revision") && text.contains("retired"),
        "{text}"
    );
}

/// The rerun of the release seat (cli-opus-2 calls 51 to 54): a named
/// `--limit` kept twelve items and printed the group's header alone,
/// since a group over ten is counted. A limit is the person's own bound:
/// the items it keeps are listed.
#[test]
fn a_named_limit_lists_the_items_it_bounds() {
    let scratch = harness::scratch("price-limit-lists");
    let base = scratch.0.clone();
    let mock = seed(&base);
    let mut store = Store::open(&account_path(&mock, USER)).unwrap();
    let at = acquisition_store::now();
    let item = |n: usize| {
        json!({ "id": format!("i-big-{n:02}"), "name": "", "typeLine": "Chaos Orb", "baseType": "Chaos Orb",
                "x": n, "y": 0, "stackSize": 1, "note": "~price 1 chaos" })
    };
    store
        .record(
            &Endpoint::Stashes { realm: "pc".into(), league: "Standard".into() },
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": [
                { "id": "f1", "name": "Sale", "type": "Folder", "index": 0,
                  "children": [ { "id": "c1", "name": "~price 3 chaos", "type": "PremiumStash", "index": 1, "metadata": { "public": true } } ] },
                { "id": "m1", "name": "Maps", "type": "MapStash", "index": 2 },
                { "id": "big", "name": "Big", "type": "PremiumStash", "index": 3, "metadata": { "public": true } } ] }),
            at,
        )
        .unwrap();
    store
        .record(
            &Endpoint::Stash {
                realm: "pc".into(),
                league: "Standard".into(),
                id: "big".into(),
                sub: None,
            },
            &json!({}),
            200,
            &json!({ "stash": { "id": "big", "name": "Big", "type": "PremiumStash",
                                "items": (0..12).map(item).collect::<Vec<_>>() } }),
            at,
        )
        .unwrap();
    drop(store);
    let out = acq(
        &base,
        &["price", "list", "--in", "tab/pc/big", "--limit", "12"],
    );
    assert!(out.status.success(), "{}", stderr(&out));
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    let lines = text.lines().filter(|l| l.contains("i-big-")).count();
    assert_eq!(lines, 12, "{text}");
}

/// The league Big beside the seed's Standard: a map tab `b1` of thirteen
/// items of its own over a substash `s9` of fourteen, every item unpriced
/// — two sets past the read economy's bound of ten (C53).
fn big_league(mock: &Path) {
    let at = acquisition_store::now();
    let mut store = Store::open(&account_path(mock, USER)).unwrap();
    let record = |store: &mut Store, ep: Endpoint, body: Value| {
        store.record(&ep, &json!({}), 200, &body, at).unwrap();
    };
    let items = |prefix: &str, n: usize| -> Vec<Value> {
        (0..n)
            .map(|i| json!({ "id": format!("{prefix}{i:02}"), "name": "", "typeLine": "Chaos Orb", "baseType": "Chaos Orb", "x": i, "y": 0, "stackSize": 1 }))
            .collect()
    };
    record(
        &mut store,
        Endpoint::Stashes {
            realm: "pc".into(),
            league: "Big".into(),
        },
        json!({ "stashes": [ { "id": "b1", "name": "Many", "type": "MapStash", "index": 0 } ] }),
    );
    let stash = |sub: Option<&str>| Endpoint::Stash {
        realm: "pc".into(),
        league: "Big".into(),
        id: "b1".into(),
        sub: sub.map(str::to_string),
    };
    record(
        &mut store,
        stash(None),
        json!({ "stash": { "id": "b1", "name": "Many", "type": "MapStash", "items": items("i-b", 13),
                            "children": [ { "id": "s9", "name": "9", "type": "MapStash", "metadata": { "items": 14 } } ] } }),
    );
    record(
        &mut store,
        stash(Some("s9")),
        json!({ "stash": { "id": "s9", "name": "9", "type": "MapStash", "items": items("i-s", 14) } }),
    );
}

/// C53, the read economy (step 2g, round 2): `price show` of a container
/// whose two sets are past ten lists ten of each beside the sets' whole
/// counts, `--limit` lists that many, and `--expand`, the route the text
/// names first, lists every item the default left out; the text counts
/// from the document's counts, never from the rows it lists (the cold
/// review of the brief, findings 6 and 7).
#[test]
fn price_show_lists_ten_of_each_set_beside_the_whole_counts_and_expand_lists_every_one() {
    use acquisition_store::bound::LISTED;
    let scratch = harness::scratch("price-show-bound");
    let base = scratch.0.clone();
    let mock = seed(&base);
    big_league(&mock);
    let shown = |args: &[&str]| -> ShowView {
        let out = acq(
            &base,
            &[&["price", "show", "tab/pc/b1", "--json"], args].concat(),
        );
        assert!(out.status.success(), "{}", stderr(&out));
        serde_json::from_value(sole_json(&out)).unwrap()
    };
    let ids = |set: &[acquisition_plan::listing::Listing]| -> Vec<String> {
        set.iter().map(|l| l.subject.target.to_string()).collect()
    };
    let view = shown(&[]);
    assert_eq!(view.header.league, "Big");
    assert_eq!(
        (view.items_here.len(), view.items_covered_below.len()),
        (LISTED, LISTED)
    );
    let (here, below) = (&view.items_here_counts, &view.items_covered_below_counts);
    assert_eq!(
        (here.total, here.returned, here.left_out),
        (13, LISTED, 13 - LISTED)
    );
    assert_eq!(
        (below.total, below.returned, below.left_out),
        (14, LISTED, 14 - LISTED)
    );
    assert_eq!(here.by_relation.get("none"), Some(&13), "{here:?}");

    let more = shown(&["--limit", "12"]);
    assert_eq!(
        (more.items_here.len(), more.items_covered_below.len()),
        (12, 12)
    );

    let all = shown(&["--expand"]);
    for (default, every) in [
        (&view.items_here, &all.items_here),
        (&view.items_covered_below, &all.items_covered_below),
    ] {
        let every = ids(every);
        let omitted: Vec<String> = every
            .iter()
            .filter(|id| !ids(default).contains(id))
            .cloned()
            .collect();
        assert!(!omitted.is_empty());
        assert!(omitted.iter().all(|id| every.contains(id)));
    }
    assert_eq!(
        (all.items_here.len(), all.items_covered_below.len()),
        (13, 14)
    );

    // the text: the whole sets' counts, and the audit route first
    let mut t = Transcript::offline(&base);
    t.run(
        &["price", "show", "tab/pc/b1"],
        &[
            "items here: 13 items — 13 unlisted",
            "items covered through children: 14 items — 14 unlisted",
            "next: `acq price show --account 'Alice#1234' --league Big tab/pc/b1 --expand` lists every item of both sets; `acq price list --account 'Alice#1234' --league Big --in tab/pc/b1 --expand` is the filtered route",
        ],
    );
    let out = acq(&base, &["price", "show", "tab/pc/b1", "--expand"]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert_eq!(text.matches("Chaos Orb x1 ").count(), 27, "{text}");
}

/// C53, the read economy (step 2g, round 2; the second cold review of the
/// brief, finding 10): `price list` lists ten of a selection past ten and
/// counts the whole, and `--expand` reaches the planner, so `--json`
/// returns every item, where it widened the text alone.
#[test]
fn price_list_lists_ten_and_expand_returns_every_item_of_a_selection_past_ten() {
    use acquisition_store::bound::LISTED;
    let scratch = harness::scratch("price-list-bound");
    let base = scratch.0.clone();
    let mock = seed(&base);
    big_league(&mock);
    let listed = |args: &[&str]| -> ListView {
        let out = acq(
            &base,
            &[
                &[
                    "price",
                    "list",
                    "--league",
                    "Big",
                    "--effective",
                    "none",
                    "--json",
                ],
                args,
            ]
            .concat(),
        );
        assert!(out.status.success(), "{}", stderr(&out));
        serde_json::from_value(sole_json(&out)).unwrap()
    };
    let view = listed(&[]);
    assert_eq!(
        (view.total, view.listed, view.items.len()),
        (27, LISTED, LISTED)
    );
    assert_eq!((view.counts.total, view.counts.left_out), (27, 27 - LISTED));
    assert_eq!(view.counts.by_relation.get("none"), Some(&27));
    let all = listed(&["--expand"]);
    assert_eq!((all.total, all.listed, all.items.len()), (27, 27, 27));
    assert!(all.filter.expand);
    assert_eq!(&all.items[..LISTED], &view.items[..]);

    let out = acq(
        &base,
        &["price", "list", "--league", "Big", "--effective", "none"],
    );
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.starts_with("10 of 27 items listed in Big decided by none: 27 unlisted; the 10 listed in 1 container\n"),
        "{text}"
    );
    assert!(
        text.ends_with("next: `--expand` lists every item; `acq price show item/<id>` is one\n"),
        "{text}"
    );
}

/// The words of a printed command, as a shell would split them: on
/// spaces outside single quotes, the quotes taken off (the frontends
/// quote with `shell_quoted`, single quotes alone).
fn shell_words(command: &str) -> Vec<String> {
    let (mut words, mut word, mut quoted) = (Vec::new(), String::new(), false);
    for c in command.chars() {
        match c {
            '\'' => quoted = !quoted,
            ' ' if !quoted => {
                if !word.is_empty() {
                    words.push(std::mem::take(&mut word));
                }
            }
            c => word.push(c),
        }
    }
    if !word.is_empty() {
        words.push(word);
    }
    words
}

/// Every `acq price …` command a text prints between backticks that
/// runs as typed: a placeholder (`<id>`, a bare `N`) is the reader's to
/// fill, never run.
fn printed_price_commands(text: &str) -> Vec<String> {
    text.split('`')
        .skip(1)
        .step_by(2)
        .filter(|c| c.starts_with("acq price "))
        .filter(|c| !c.contains('<') && !c.split(' ').any(|w| w == "N"))
        .map(str::to_string)
        .collect()
}

/// A printed command run as printed, offline, under `--json`, with its
/// document.
fn run_printed(base: &Path, command: &str) -> Value {
    let words = shell_words(command);
    assert_eq!(words[0], "acq", "{command}");
    let mut args: Vec<&str> = words[1..].iter().map(String::as_str).collect();
    args.push("--json");
    let out = acq(base, &args);
    assert!(
        out.status.success(),
        "`{command}` does not run as printed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        stderr(&out)
    );
    sole_json(&out)
}

/// Step 2g's round-2 fix rule (the review of round 2, both findings): a
/// printed `acq price` command carries the scope the answer resolved —
/// the account it ran as, the realm and league where not the defaults —
/// and, where it promises each or every item, `--expand`, since the
/// default lists ten (C53). Staged with two accounts known, over the
/// league Big (a map tab of 13 over a substash of 14), an xbox tab of 12
/// and the league Clash, eleven items whose hand price conflicts with
/// the game's: the show's audit command recovers both whole sets, the
/// realm's too, the status's conflict remedy lists all eleven, and every
/// printed price command of these answers runs as printed.
#[test]
fn every_printed_price_command_carries_its_scope_and_runs_as_printed() {
    let scratch = harness::scratch("price-printed");
    let base = scratch.0.clone();
    let mock = seed(&base);
    big_league(&mock);
    let at = acquisition_store::now();
    Index::load(&mock)
        .unwrap()
        .record_login("Bob#5678", "u-bob", false, at, &[])
        .unwrap();
    let mut store = Store::open(&account_path(&mock, USER)).unwrap();
    let item = |id: String, x: usize, note: Option<&str>| {
        let mut v = json!({ "id": id, "name": "", "typeLine": "Chaos Orb", "baseType": "Chaos Orb", "x": x, "y": 0, "stackSize": 1 });
        if let Some(n) = note {
            v["note"] = json!(n);
        }
        v
    };
    let mut record = |realm: &str, league: &str, id: Option<&str>, body: Value| {
        let params = json!({ "realm": realm, "league": league });
        let ep = match id {
            None => Endpoint::Stashes {
                realm: realm.into(),
                league: league.into(),
            },
            Some(id) => Endpoint::Stash {
                realm: realm.into(),
                league: league.into(),
                id: id.into(),
                sub: None,
            },
        };
        store.record(&ep, &params, 200, &body, at).unwrap();
    };
    record(
        "xbox",
        "Standard",
        None,
        json!({ "stashes": [ { "id": "x1", "name": "Box", "type": "PremiumStash", "index": 0 } ] }),
    );
    record(
        "xbox",
        "Standard",
        Some("x1"),
        json!({ "stash": { "id": "x1", "name": "Box", "type": "PremiumStash",
                           "items": (0..12).map(|i| item(format!("i-x{i:02}"), i, None)).collect::<Vec<_>>() } }),
    );
    record(
        "pc",
        "Clash",
        None,
        json!({ "stashes": [ { "id": "k1", "name": "Clash", "type": "PremiumStash", "index": 0, "metadata": { "public": true } } ] }),
    );
    record(
        "pc",
        "Clash",
        Some("k1"),
        json!({ "stash": { "id": "k1", "name": "Clash", "type": "PremiumStash",
                           "items": (0..11).map(|i| item(format!("i-k{i:02}"), i, Some("~price 5 chaos"))).collect::<Vec<_>>() } }),
    );
    drop(store);
    let mut a = Annotations::open_for(&mock, UUID).unwrap();
    for i in 0..11 {
        a.put::<Buyout>(
            "item",
            &format!("i-k{i:02}"),
            &json!({ "version": 1, "type": "exact", "amount": "3", "currency": "chaos" }),
            None,
            &Provenance::via("test"),
        )
        .unwrap();
    }
    drop(a);

    let account = "--account";
    let text = |args: &[&str]| -> String {
        let out = acq(&base, &[&["price"][..], args, &[account, USER]].concat());
        assert!(out.status.success(), "{args:?}: {}", stderr(&out));
        String::from_utf8_lossy(&out.stdout).into_owned()
    };
    let shop = {
        let out = acq(&base, &["shop", "render", "--league", "Big", account, USER]);
        assert!(out.status.success(), "{}", stderr(&out));
        String::from_utf8_lossy(&out.stdout).into_owned()
    };
    let answers = [
        text(&["show", "tab/pc/b1"]),
        text(&["show", "tab/xbox/x1"]),
        text(&["show", "item/i-b00"]),
        text(&["status", "--league", "Clash"]),
        text(&["status"]),
        text(&["list", "--league", "Big", "--relation", "agree"]),
        shop,
    ];

    // the show's audit command: both whole sets, in the league shown
    let audit = format!("acq price show --account '{USER}' --league Big tab/pc/b1 --expand");
    assert!(answers[0].contains(&audit), "{}", answers[0]);
    let doc = run_printed(&base, &audit);
    assert_eq!(
        (
            doc["items_here"].as_array().unwrap().len(),
            doc["items_covered_below"].as_array().unwrap().len()
        ),
        (13, 14),
        "{doc}"
    );
    // the filtered route keeps the league too, every item it promises
    let filtered =
        format!("acq price list --account '{USER}' --league Big --in tab/pc/b1 --expand");
    assert!(answers[0].contains(&filtered), "{}", answers[0]);
    // and a realm not the default
    let audit = format!("acq price show --account '{USER}' --realm xbox tab/xbox/x1 --expand");
    assert!(answers[1].contains(&audit), "{}", answers[1]);
    let doc = run_printed(&base, &audit);
    assert_eq!(doc["items_here"].as_array().unwrap().len(), 12, "{doc}");

    // eleven conflicts: the remedy names each, all eleven
    let remedy =
        format!("acq price list --account '{USER}' --league Clash --relation conflict --expand");
    assert!(answers[3].contains(&remedy), "{}", answers[3]);
    let doc = run_printed(&base, &remedy);
    assert_eq!(
        (&doc["total"], doc["items"].as_array().unwrap().len()),
        (&json!(11), 11),
        "{doc}"
    );

    // the sweep: every printed price command of these answers runs
    let mut ran = 0;
    for answer in &answers {
        for command in printed_price_commands(answer) {
            assert!(
                command.contains(&format!("--account '{USER}'")),
                "`{command}` drops the account"
            );
            run_printed(&base, &command);
            ran += 1;
        }
    }
    assert!(ran >= 8, "{ran} printed commands run");
}
