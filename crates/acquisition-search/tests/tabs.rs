//! A league's tabs as `tabs` gives them on both surfaces (`src/tabs.rs`,
//! C115, C118), through the crate's boundary: the store's rows in, the
//! document out. The frontends' tests pin what their adapters add (the
//! text, the size of the MCP's answer, the arguments' mapping); the
//! listing rule is pinned here, once (C122).
//!
//! The fixture is the one the seats' finding was staged over: 41 tabs of
//! pc Standard, two premium tabs fetched with five items between them and
//! 39 of another type never fetched — more than the read economy's bound
//! (`acquisition_store::bound::LISTED`, C53), so the listing is that many
//! with the most items and the rest counted.

use crate::common;

use acquisition_search::tabs::{Selection, Tabs};
use acquisition_store::Store;
use acquisition_store::bound::LISTED;
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
    serde_json::to_value(Tabs::of("pc", "Standard", rows, select, None, expand)).unwrap()
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
    const { assert!(41 > LISTED + 2) };
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
    assert_eq!(ids.len(), LISTED);
    // the most items first, then by id
    assert_eq!(ids[..2], ["t2", "t1"]);
    assert_eq!(ids[2..4], ["q0", "q1"]);
    assert_eq!(
        (&doc["left_out"], &doc["left_out_needs"]),
        (&json!(41 - LISTED), &json!("expand"))
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

/// C53, the one rule (step 2g): a league of thirty tabs or fewer, once
/// listed whole, lists the bound's ten with the most items like any
/// other; `limit` names more or fewer, zero included, and a limit past
/// the league lists it whole in the listing's order; `expand` is the
/// whole whatever `limit` says.
#[test]
fn a_league_of_thirty_or_fewer_lists_the_ten_with_the_most_items_and_limit_names_more() {
    // Rings and Dump fetched, and eighteen tabs never fetched: twenty
    let mut s = store();
    let mut tabs = vec![tab("t1", "Rings"), tab("t2", "Dump")];
    tabs.extend((0..18).map(
        |i| json!({ "id": format!("q{i:02}"), "name": format!("Q {i}"), "type": "QuadStash" }),
    ));
    list_tabs(&mut s, "pc", "Standard", json!(tabs), 10);
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "t1",
        "Rings",
        vec![item("r1", "Doom Loop", "Two-Stone Ring", "Rare", json!({}))],
        20,
    );
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "t2",
        "Dump",
        vec![
            item("d1", "Grim Knot", "Iron Ring", "Rare", json!({})),
            item("d2", "Dull Turn", "Iron Ring", "Rare", json!({})),
        ],
        30,
    );
    const { assert!(20 > LISTED && 20 <= 30) };
    let at = |limit: Option<usize>, expand: bool| {
        let rows = s.tabs("pc", "Standard").unwrap();
        serde_json::to_value(Tabs::of(
            "pc",
            "Standard",
            rows,
            &Selection::default(),
            limit,
            expand,
        ))
        .unwrap()
    };

    let doc = at(None, false);
    assert_eq!(doc["tabs"], 20, "{doc}");
    assert_eq!(doc["listed"], "most_items");
    let ids = listed(&doc);
    assert_eq!(ids.len(), LISTED);
    assert_eq!(
        ids[..3],
        ["t2", "t1", "q00"],
        "the most items first, then by id"
    );
    assert_eq!(
        (&doc["left_out"], &doc["left_out_needs"]),
        (&json!(20 - LISTED), &json!("expand"))
    );

    let more = at(Some(15), false);
    assert_eq!((listed(&more).len(), &more["left_out"]), (15, &json!(5)));
    assert_eq!(more["listed"], "most_items");
    let fewer = at(Some(1), false);
    assert_eq!(listed(&fewer), ["t2"]);
    assert_eq!(fewer["left_out"], 19);
    let none = at(Some(0), false);
    assert!(listed(&none).is_empty(), "{none}");
    assert_eq!(none["left_out"], 20);

    // past the league: whole, in the listing's order, nothing to expand
    let past = at(Some(25), false);
    assert_eq!(past["listed"], "all");
    assert_eq!(listed(&past)[..3], ["t1", "t2", "q00"]);
    assert_eq!(past["left_out"], 0);
    assert!(past.get("left_out_needs").is_none(), "{past}");
    // expand is the whole whatever the limit
    assert_eq!(at(Some(2), true)["rows"], past["rows"]);
    assert_eq!(at(None, true)["rows"], past["rows"]);
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
    assert_eq!(listed(&doc).len(), LISTED);
    assert_eq!(
        (&doc["left_out"], &doc["left_out_needs"]),
        (&json!(39 - LISTED), &json!("expand"))
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

/// The document's groups (the 4b sitting round's review, its fourth
/// finding): asked, each listed tab is a group in the rows' order, a map
/// tab's pages nested under it, a folder's children tabs of their own;
/// unasked, no `groups` key, the flat answer unchanged.
#[test]
fn a_map_tabs_pages_are_grouped_under_it_when_asked() {
    use acquisition_store::Endpoint;
    let mut s = store();
    list_tabs(
        &mut s,
        "pc",
        "Standard",
        json!([
            { "id": "m1", "name": "Maps", "type": "MapStash" },
            { "id": "f1", "name": "Old", "type": "Folder", "children": [
                { "id": "c1", "name": "Old 1", "type": "PremiumStash" } ] },
            tab("t1", "Rings"),
        ]),
        10,
    );
    s.record(
        &Endpoint::Stash {
            realm: "pc".into(),
            league: "Standard".into(),
            id: "m1".into(),
            sub: None,
        },
        &json!({ "realm": "pc", "league": "Standard" }),
        200,
        &json!({ "stash": { "id": "m1", "name": "Maps", "type": "MapStash", "items": [], "children": [
            { "id": "s1", "name": "", "type": "MapStash", "parent": "m1", "metadata": { "items": 2 } },
            { "id": "s2", "name": "", "type": "MapStash", "parent": "m1", "metadata": { "items": 1 } } ] } }),
        20,
    )
    .unwrap();
    let rows = s.tabs("pc", "Standard").unwrap();
    let flat = serde_json::to_value(Tabs::of(
        "pc",
        "Standard",
        rows.clone(),
        &Selection::default(),
        None,
        true,
    ))
    .unwrap();
    assert!(flat.get("groups").is_none(), "{flat}");
    let grouped = serde_json::to_value(
        Tabs::of("pc", "Standard", rows, &Selection::default(), None, true).grouped(),
    )
    .unwrap();
    assert_eq!(grouped["rows"], flat["rows"], "the flat rows stand");
    let groups: Vec<(String, Vec<String>)> = grouped["groups"]
        .as_array()
        .unwrap()
        .iter()
        .map(|g| {
            (
                g["tab"]["id"].as_str().unwrap().to_string(),
                g.get("pages")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .map(|p| p["id"].as_str().unwrap().to_string())
                    .collect(),
            )
        })
        .collect();
    let id = |s: &str| s.to_string();
    assert!(
        groups.contains(&(id("m1"), vec![id("s1"), id("s2")])),
        "{groups:?}"
    );
    for alone in ["f1", "c1", "t1"] {
        assert!(
            groups.contains(&(id(alone), Vec::new())),
            "{alone}: {groups:?}"
        );
    }
    assert_eq!(groups.len(), 4, "{groups:?}");
}
