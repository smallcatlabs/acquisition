//! A league's tabs as `tabs` gives them on both surfaces (`src/tabs.rs`,
//! C115, C118), through the crate's boundary: the store's rows in, the
//! document out. The frontends' tests pin what their adapters add (the
//! text, the size of the MCP's answer, the arguments' mapping); the
//! listing rule is pinned here, once (C122).
//!
//! The fixture is the one the seats' finding was staged over: 41 tabs of
//! pc Standard, two premium tabs fetched with five items between them and
//! 39 of another type never fetched — more than `LISTED_WHOLE`, so the
//! listing is the `MOST_ITEMS` with the most items and the rest counted.

mod common;

use acquisition_search::tabs::{LISTED_WHOLE, MOST_ITEMS, Selection, Tabs};
use acquisition_store::Store;
use common::*;
use serde_json::{Value, json};

/// Rings (`t1`, two items) and Dump (`t2`, three) fetched, and 39 tabs of
/// `kind` never fetched.
fn league(kind: &str) -> Store {
    let mut s = store();
    let mut listed = vec![tab("t1", "Rings"), tab("t2", "Dump")];
    listed.extend(
        (0..39).map(|i| json!({ "id": format!("q{i}"), "name": format!("Q {i}"), "type": kind })),
    );
    list_tabs(&mut s, "pc", "Standard", json!(listed), 10);
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "t1",
        "Rings",
        vec![
            item("r1", "Doom Loop", "Two-Stone Ring", "Rare", json!({})),
            item("r2", "Rune Coil", "Two-Stone Ring", "Rare", json!({})),
        ],
        20,
    );
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "t2",
        "Dump",
        vec![
            item(
                "kaom",
                "Kaom's Heart",
                "Glorious Plate",
                "Unique",
                json!({}),
            ),
            item("plain", "Dull Turn", "Iron Ring", "Rare", json!({})),
            item("str", "Grim Knot", "Iron Ring", "Rare", json!({})),
        ],
        30,
    );
    s
}

fn tabs_of(s: &Store, select: &Selection, expand: bool) -> Value {
    let rows = s.tabs("pc", "Standard").unwrap();
    serde_json::to_value(Tabs::of("pc", "Standard", rows, select, expand)).unwrap()
}

fn listed(doc: &Value) -> Vec<&str> {
    doc["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["id"].as_str().unwrap())
        .collect()
}

/// Counts first, by type beside them, then the ten with the most items
/// and the rest counted, the cut naming `expand`; `expand` lists all.
#[test]
fn a_large_league_is_counted_first_and_lists_the_ten_with_the_most_items() {
    const { assert!(41 > LISTED_WHOLE && MOST_ITEMS == 10) };
    let s = league("QuadStash");
    let doc = tabs_of(&s, &Selection::default(), false);
    assert_eq!(
        (
            &doc["tabs"],
            &doc["fetched"],
            &doc["never_fetched"],
            &doc["folders"],
            &doc["items"]
        ),
        (&json!(41), &json!(2), &json!(39), &json!(0), &json!(5)),
        "{doc}"
    );
    assert_eq!(
        doc["by_type"],
        json!([
            { "type": "PremiumStash", "tabs": 2, "fetched": 2, "items": 5 },
            { "type": "QuadStash", "tabs": 39, "fetched": 0, "items": 0 },
        ])
    );
    assert_eq!(doc["listed"], "most_items");
    let ids = listed(&doc);
    assert_eq!(ids.len(), MOST_ITEMS);
    // the most items first, then by id
    assert_eq!(ids[..2], ["t2", "t1"]);
    assert_eq!(ids[2..4], ["q0", "q1"]);
    assert_eq!(
        (&doc["left_out"], &doc["left_out_needs"]),
        (&json!(31), &json!("expand"))
    );
    // no selection asked, none said
    assert!(
        doc.get("selection").is_none() && doc.get("selected").is_none(),
        "{doc}"
    );

    let all = tabs_of(&s, &Selection::default(), true);
    assert_eq!(all["listed"], "all");
    assert_eq!(listed(&all).len(), 41);
    assert_eq!(all["left_out"], 0);
    assert!(all.get("left_out_needs").is_none(), "{all}");
    // the counts are the league's either way
    assert_eq!(all["by_type"], doc["by_type"]);
}

/// A selection by type and by never-fetched narrows the rows under the
/// league's counts, `selected` counting them, and the same rule cuts
/// them; a type is GGG's, read in any case.
#[test]
fn a_selection_narrows_the_rows_under_the_leagues_counts() {
    let s = league("UniqueStash");
    let never = Selection {
        kind: Some("UniqueStash".into()),
        never_fetched: true,
    };
    let doc = tabs_of(&s, &never, false);
    assert_eq!(doc["tabs"], 41, "{doc}");
    assert_eq!(doc["selected"], 39, "{doc}");
    assert_eq!(
        doc["selection"],
        json!({ "type": "UniqueStash", "never_fetched": true })
    );
    assert_eq!(doc["listed"], "most_items");
    assert_eq!(listed(&doc).len(), MOST_ITEMS);
    assert_eq!(
        (&doc["left_out"], &doc["left_out_needs"]),
        (&json!(29), &json!("expand"))
    );
    let all = tabs_of(&s, &never, true);
    assert_eq!(listed(&all).len(), 39, "{all}");
    assert!(
        all["rows"]
            .as_array()
            .unwrap()
            .iter()
            .all(|r| r["fetched_at"].is_null() && r["type"] == "UniqueStash"),
        "{all}"
    );

    // a few selected are listed whole, whatever the league holds
    let premium = Selection {
        kind: Some("premiumstash".into()),
        never_fetched: false,
    };
    let doc = tabs_of(&s, &premium, false);
    assert_eq!(
        (&doc["selected"], &doc["listed"]),
        (&json!(2), &json!("all"))
    );
    assert_eq!(listed(&doc), ["t1", "t2"]);
    assert_eq!(doc["left_out"], 0);
    assert!(doc.get("left_out_needs").is_none(), "{doc}");

    // the never fetched alone: every type's
    let unfetched = Selection {
        kind: None,
        never_fetched: true,
    };
    let doc = tabs_of(&s, &unfetched, true);
    assert_eq!(doc["selected"], 39, "{doc}");
    assert_eq!(unfetched.words(), "never-fetched tabs");
    assert_eq!(never.words(), "never-fetched UniqueStash tabs");
}
