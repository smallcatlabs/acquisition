//! Process-level pin of `acq shop render` (C74, C72, C53): the `--json`
//! document is the [`ShopRender`] the text is a function of, `--page N`
//! prints exactly one page's text, a template file wraps each page, and
//! nothing contacts a daemon (`ACQ_NO_SPAWN=1` makes any contact an
//! error). Facts and intent are seeded through the store crate, as
//! `price_json.rs` does.

use std::path::{Path, PathBuf};
use std::process::Output;

use acquisition_plan::price::Buyout;
use acquisition_plan::shop::{Cell, Page, ShopRender, Verdict};
use acquisition_plan::{PolicyWrite, SyncPolicy, put_sync_policy};
use acquisition_store::{Annotations, Endpoint, Index, Provenance, Store, account_path};
use serde_json::{Value, json};

use crate::harness;

use harness::{Transcript, golden, offline_acq as acq, sole_json};

const USER: &str = "Alice#1234";
const UUID: &str = "u-shop";

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// A folder holding a public priced tab with three items (one noted, so
/// the game wins the tie; one with its own row, which beats the tab's
/// name; one plain, which the tab name lists), a public unpriced tab
/// with one item priced by hand at a ratio, a map tab with a substash
/// holding one hand-priced item, a character wearing a hand-priced
/// item; a policy covering the folder only.
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
    record(
        &mut store,
        Endpoint::Stashes {
            realm: "pc".into(),
            league: "Standard".into(),
        },
        json!({ "league": "Standard" }),
        json!({ "stashes": [
            { "id": "f1", "name": "Sale", "type": "Folder", "index": 0,
              "children": [ { "id": "c1", "name": "~price 3 chaos", "type": "PremiumStash", "index": 1, "metadata": { "public": true } } ] },
            { "id": "p1", "name": "Plain", "type": "PremiumStash", "index": 2, "metadata": { "public": true } },
            { "id": "m1", "name": "Maps", "type": "MapStash", "index": 3 } ] }),
    );
    let item = |id: &str, x: i64, note: Option<&str>| {
        let mut v = json!({ "id": id, "name": "", "typeLine": "Chaos Orb", "baseType": "Chaos Orb", "x": x, "y": 0, "stackSize": 5, "inventoryId": "Stash1" });
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
                            "items": [ item("i-noted", 0, Some("~price 5 chaos")), item("i-hand", 1, None), item("i-plain", 2, None) ] } }),
    );
    record(
        &mut store,
        stash("p1", None),
        json!({}),
        json!({ "stash": { "id": "p1", "name": "Plain", "type": "PremiumStash",
                            "items": [ item("i-ratio", 0, None) ] } }),
    );
    record(
        &mut store,
        stash("m1", None),
        json!({}),
        json!({ "stash": { "id": "m1", "name": "Maps", "type": "MapStash", "items": [],
                            "children": [ { "id": "s1", "name": "1", "type": "MapStash", "index": 4, "metadata": { "items": 1 } } ] } }),
    );
    record(
        &mut store,
        stash("m1", Some("s1")),
        json!({}),
        json!({ "stash": { "id": "s1", "name": "1", "type": "MapStash", "items": [ item("i-map", 0, None) ] } }),
    );
    record(
        &mut store,
        Endpoint::Characters { realm: "pc".into() },
        json!({ "realm": "pc" }),
        json!({ "characters": [ { "id": "ch1", "name": "Exile", "league": "Standard" } ] }),
    );
    let mut worn = item("i-worn", 0, None);
    worn["inventoryId"] = json!("BodyArmour");
    record(
        &mut store,
        Endpoint::Character {
            realm: "pc".into(),
            name: "Exile".into(),
        },
        json!({ "realm": "pc", "name": "Exile" }),
        json!({ "character": { "id": "ch1", "name": "Exile", "league": "Standard",
                                "equipment": [ worn ], "inventory": [] } }),
    );
    let mut a = Annotations::open_for(&mock, UUID).unwrap();
    let via = Provenance::via("test");
    let put = |a: &mut Annotations, scope: &str, key: &str, value: Value| {
        a.put::<Buyout>(scope, key, &value, None, &via).unwrap();
    };
    put(
        &mut a,
        "item",
        "i-hand",
        json!({ "version": 1, "type": "exact", "amount": "12.5", "currency": "chaos" }),
    );
    put(
        &mut a,
        "item",
        "i-ratio",
        json!({ "version": 1, "type": "negotiable", "amount": "1/5", "currency": "divine" }),
    );
    put(
        &mut a,
        "item",
        "i-map",
        json!({ "version": 1, "type": "exact", "amount": "1", "currency": "chaos" }),
    );
    put(
        &mut a,
        "item",
        "i-worn",
        json!({ "version": 1, "type": "negotiable", "amount": "2", "currency": "divine" }),
    );
    let policy = json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard": {
        "tabs": ["f1"], "max_age_seconds": 3600 } } } } });
    let _: SyncPolicy = SyncPolicy::from_value(&policy).unwrap();
    put_sync_policy(&mut a, &policy, None, PolicyWrite::Replace, &via).unwrap();
    mock
}

#[test]
fn shop_render_json_is_the_document_the_text_and_the_page_read() {
    let scratch = harness::scratch("shop-json");
    let base = scratch.0.clone();
    seed(&base);

    let out = acq(&base, &["shop", "render", "--json"]);
    assert!(out.status.success(), "{}", stderr(&out));
    let r: ShopRender = serde_json::from_value(sole_json(&out)).unwrap();
    assert_eq!(r.counts.items, 6);
    assert_eq!(r.counts.posted, 2);
    assert_eq!(r.counts.omitted, 2);
    assert_eq!(r.counts.blocked, 2);
    assert_eq!(r.counts.off_page, 0);
    assert_eq!(r.counts.pages, 1);
    let cells: Vec<(String, Cell)> = r
        .posted
        .iter()
        .map(|p| (p.target.to_string(), p.cell))
        .chain(r.left_out.iter().map(|l| (l.target.to_string(), l.cell)))
        .collect();
    let cell_of = |id: &str| {
        cells
            .iter()
            .find(|(t, _)| t == &format!("item/{id}"))
            .map(|(_, c)| *c)
            .unwrap_or_else(|| panic!("{id} not in {cells:?}"))
    };
    assert_eq!(cell_of("i-hand"), Cell::StashItem);
    assert_eq!(cell_of("i-worn"), Cell::CharacterItem);
    assert_eq!(cell_of("i-noted"), Cell::GameLists);
    assert_eq!(cell_of("i-plain"), Cell::GameLists);
    assert_eq!(cell_of("i-ratio"), Cell::Ratio);
    assert_eq!(cell_of("i-map"), Cell::Substash);
    assert!(r.left_out.iter().all(|l| l.verdict == l.cell.verdict()));
    // The page: the stash item by its tab's rank among the website's tabs
    // (the folder takes no number), the character
    // item by its slot, each under its price's spoiler, `~price` first.
    let page_text = "[spoiler=\"Shop Post 1 of 1 (2 items)\"]\n\
                     [spoiler=\" ~price 12.5 chaos\"][linkItem realm=\"pc\" location=\"Stash1\" league=\"Standard\" x=\"1\" y=\"0\"][/spoiler]\n\
                     [spoiler=\" ~b/o 2 divine\"][linkItem realm=\"pc\" location=\"BodyArmour\" character=\"Exile\" x=\"0\" y=\"0\"][/spoiler]\n\
                     [/spoiler]\n";
    assert_eq!(r.pages.len(), 1);
    assert_eq!(r.pages[0].text, page_text);
    assert_eq!(r.pages[0].items, 2);
    // C146: the page names the tab its `Stash1` links write; the
    // character's link names its character itself.
    assert_eq!(r.pages[0].tabs.len(), 1);
    assert_eq!(r.pages[0].tabs[0].stash, "Stash1");
    assert_eq!(r.pages[0].tabs[0].name, "~price 3 chaos");
    // C72: the character is outside the policy (tabs only), the folder
    // covers c1; nothing is stale yet; the refresh's count is the plan's.
    assert_eq!(r.freshness.policy, "covers");
    assert_eq!(r.freshness.window_seconds, Some(3600));
    assert_eq!(
        r.freshness
            .uncovered
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["character/ch1"]
    );
    assert!(r.freshness.stale.is_empty());
    assert!(r.freshness.refresh_requests.is_some());

    // The text is a function of the document: the one line, the
    // coverage line, the page under its label with its tabs, the next
    // action.
    let mut t = Transcript::offline(&base);
    let out = t.run(
        &["shop", "render"],
        &[
            "2 items to post on 1 page",
            "coverage: 1 container on the page outside the sync policy (revision 1): character/ch1",
        ],
    );
    assert!(out.status.success(), "{}", stderr(&out));
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.contains(&format!(
            "page 1 of 1: {} characters, 2 items\n  Stash1 is the tab \"~price 3 chaos\" (tab/pc/c1): 1 item\n{page_text}\n",
            page_text.chars().count()
        )),
        "{text}"
    );
    golden(file!(), "shop_render", &t);

    // --page N is the page's text and nothing else; --json the record.
    let out = acq(&base, &["shop", "render", "--page", "1"]);
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(String::from_utf8_lossy(&out.stdout), page_text);
    let out = acq(&base, &["shop", "render", "--page", "1", "--json"]);
    let page: Page = serde_json::from_value(sole_json(&out)).unwrap();
    assert_eq!(page.text, page_text);
    let out = acq(&base, &["shop", "render", "--page", "2"]);
    assert!(!out.status.success());
    assert!(
        stderr(&out).contains("no page 2: the render has 1 page"),
        "{}",
        stderr(&out)
    );

    // A template file wraps each page; one without the token refuses.
    let template = base.join("shop.txt");
    std::fs::write(&template, "Shop\n[spoiler]\n[items][/spoiler]\n").unwrap();
    let out = acq(
        &base,
        &[
            "shop",
            "render",
            "--page",
            "1",
            "--template",
            template.to_str().unwrap(),
        ],
    );
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        format!("Shop\n[spoiler]\n{page_text}[/spoiler]\n")
    );
    std::fs::write(&template, "no token\n").unwrap();
    let out = acq(
        &base,
        &["shop", "render", "--template", template.to_str().unwrap()],
    );
    assert!(!out.status.success());
    assert!(
        stderr(&out).contains("must hold [items] exactly once (0 found)"),
        "{}",
        stderr(&out)
    );

    // A size under one entry: everything postable is blocked as
    // page_size, and the line says so.
    let out = acq(&base, &["shop", "render", "--size", "40", "--json"]);
    assert!(out.status.success(), "{}", stderr(&out));
    let r: ShopRender = serde_json::from_value(sole_json(&out)).unwrap();
    assert_eq!(r.counts.posted, 0);
    assert_eq!(r.counts.by_cell[&Cell::PageSize], 2);
    assert!(r.pages.is_empty());
    assert!(
        r.policy.iter().any(|row| row.cell == Cell::PageSize
            && row.verdict == Verdict::Block
            && row.count == 2)
    );
    let out = acq(&base, &["shop", "render", "--size", "40"]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.starts_with("nothing to paste for Standard: 6 items — 2 omitted"),
        "{text}"
    );
}
