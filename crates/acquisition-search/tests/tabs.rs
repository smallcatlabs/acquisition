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

use acquisition_search::places::Asking;
use acquisition_search::tabs::{Selection, Tabs};
use acquisition_store::Store;
use acquisition_store::bound::LISTED;
use common::*;
use serde_json::{Value, json};

/// The answer's time every document here is made at: three days after
/// the fixtures' fetches (C53: an age under test is made from a time
/// given, never the clock).
const NOW: i64 = 3 * 86_400 + 20;

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
    serde_json::to_value(Tabs::of("pc", "Standard", rows, select, None, expand, NOW)).unwrap()
}

fn listed(doc: &Value) -> Vec<&str> {
    doc["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["id"].as_str().unwrap())
        .collect()
}

/// C53 (an age the text prints stands in words beside its epoch): each
/// listed tab says its fetch's age at the answer's time under
/// `fetched_at_age`, the words `acq tabs` prints, `never` beside a null.
#[test]
fn a_listed_tabs_fetch_carries_its_age_in_words_beside_the_epoch() {
    let s = league("QuadStash");
    let doc = tabs_of(&s, &Selection::default(), false);
    let row = |id: &str| {
        doc["rows"]
            .as_array()
            .unwrap()
            .iter()
            .find(|t| t["id"] == id)
            .unwrap_or_else(|| panic!("{id}: {doc}"))
            .clone()
    };
    let (t1, t2, q0) = (row("t1"), row("t2"), row("q0"));
    assert_eq!(
        (&t1["fetched_at"], &t1["fetched_at_age"]),
        (&json!(20), &json!("3d ago"))
    );
    assert_eq!(
        (&t2["fetched_at"], &t2["fetched_at_age"]),
        (&json!(30), &json!("2d ago"))
    );
    assert_eq!(
        (&q0["fetched_at"], &q0["fetched_at_age"]),
        (&Value::Null, &json!("never"))
    );
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
            { "type": "PremiumStash", "tabs": 2, "fetched": 2, "never": 0, "items": 5 },
            { "type": "QuadStash", "tabs": 39, "fetched": 0, "never": 39, "items": 0 },
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

/// C53, the one rule: a league of thirty tabs or fewer, once listed
/// whole, lists the bound's ten with the most items like any other;
/// `limit` names more or fewer, zero included, and a limit past the
/// league lists it whole in the listing's order; `expand` is the whole
/// whatever `limit` says.
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
            NOW,
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

/// A league's places (`src/places.rs`) over a folder, a map tab with
/// pages, a page whose tab the store does not hold, a tab fetched empty
/// and one never fetched: the tree in stash order, each place's kind,
/// each parent's counts of what it holds, the league's counts as `tabs`
/// counts them; and each place's request answers the items the store
/// counts there, a map tab's with its pages', a folder none.
#[test]
fn the_places_are_the_stash_tree_in_order_and_each_request_lists_its_items() {
    use acquisition_search::places::Kind;
    let s = stash_tree();
    let places = places(&s, Listed::Tabs, "pc", "Standard", &Asking::default(), NOW).unwrap();
    let doc = serde_json::to_value(&places).unwrap();

    // the tree, in stash order, each place's kind
    fn shape(places: &[acquisition_search::places::Place]) -> Vec<(String, Kind, Vec<String>)> {
        places
            .iter()
            .map(|p| {
                (
                    p.id().to_string(),
                    p.place,
                    p.places.iter().map(|c| c.id().to_string()).collect(),
                )
            })
            .collect()
    }
    let id = |s: &str| s.to_string();
    assert_eq!(
        shape(&places.places),
        [
            (id("t1"), Kind::Tab, vec![]),
            (id("f1"), Kind::Folder, vec![id("c1"), id("c2")]),
            (id("m1"), Kind::Tab, vec![id("s1"), id("s2")]),
            (id("e1"), Kind::Tab, vec![]),
            (id("p1"), Kind::Page, vec![]),
        ],
        "{doc}"
    );
    let every = places.every();
    let at = |id: &str| *every.iter().find(|p| p.id() == id).unwrap();
    assert_eq!(at("s1").place, Kind::Page);
    assert_eq!(at("c1").place, Kind::Tab);
    // each parent counts what it holds and those of them never fetched;
    // a place that holds none carries no `holds`
    let holds = |id: &str| doc_place(&doc, id).get("holds").cloned();
    assert_eq!(
        holds("f1"),
        Some(json!({ "places": 2, "never_fetched": 1 }))
    );
    assert_eq!(
        holds("m1"),
        Some(json!({ "places": 2, "never_fetched": 1 }))
    );
    for none in ["t1", "e1", "p1", "s1", "c1"] {
        assert_eq!(holds(none), None, "{none}");
    }
    // the row whole, aged: never fetched and fetched-empty read apart
    assert_eq!(doc_place(&doc, "c2")["fetched_at_age"], "never");
    assert_eq!(doc_place(&doc, "e1")["fetched_items"], 0);
    assert!(doc_place(&doc, "c2")["fetched_items"].is_null());
    // the league's counts, once, as `tabs` counts them
    let (tabs, _) =
        Tabs::read(&s, "pc", "Standard", &Selection::default(), None, true, NOW).unwrap();
    let tabs = serde_json::to_value(tabs).unwrap();
    for key in [
        "tabs",
        "fetched",
        "never_fetched",
        "folders",
        "items",
        "by_type",
        "list_seen",
        "oldest_fetch",
        "newest_fetch_location",
    ] {
        assert_eq!(doc[key], tabs[key], "{key}");
    }

    // each place's request lists the items the store counts there: a tab
    // its own, a map tab its pages' too, a page its own; a folder none
    assert!(doc_place(&doc, "f1").get("request").is_none());
    let corpus = load(&s, Some("pc"));
    for (place, held) in [
        ("t1", &["r1", "r2"][..]),
        ("c1", &["o1"]),
        ("c2", &[]),
        ("e1", &[]),
        ("m1", &["map1", "map2"]),
        ("s1", &["map1", "map2"]),
        ("s2", &[]),
        ("p1", &["lost1"]),
    ] {
        let mut request = doc_place(&doc, place)["request"].clone();
        assert_eq!(request["scope"]["realm"], "pc", "{place}");
        request["view"] = json!({ "rows": { "limit": 100 } });
        let request: acquisition_search::Request = serde_json::from_value(request).unwrap();
        let answered = as_json(&answer(&corpus, &request).unwrap());
        assert_eq!(ids(&answered), held, "{place}");
    }
    // a store row's count is the place's own items: the map tab's are its
    // pages'
    assert_eq!(at("s1").item_count(), 2);
    assert_eq!(at("t1").item_count(), 2);

    // an item's place and each place holding it, top first:
    // the place an item's row names is the document's
    let placed = |item: &str| {
        let request: acquisition_search::Request = serde_json::from_value(json!({
            "query": { "text": format!("id:{item}") },
            "view": { "rows": { "limit": 1 } },
        }))
        .unwrap();
        let a = as_json(&answer(&corpus, &request).unwrap());
        a["rows"][0]["place"]["id"].as_str().unwrap().to_string()
    };
    for (item, path) in [
        ("r1", &["t1"][..]),
        ("o1", &["f1", "c1"]),
        ("map1", &["m1", "s1"]),
        ("lost1", &["p1"]),
    ] {
        assert_eq!(
            places.holders(&placed(item)).as_deref(),
            Some(path),
            "{item}"
        );
    }
    assert_eq!(places.holders("nowhere"), None);
}

/// A league's characters as places: the store's order, each its request
/// over its realm, no league term, the counts as `characters` counts
/// them.
#[test]
fn a_leagues_characters_are_places_each_with_its_request() {
    use acquisition_search::places::Kind;
    let mut s = store();
    list_characters(
        &mut s,
        "pc",
        json!([
            { "id": "ch1", "name": "Low", "league": "Standard", "level": 10 },
            { "id": "ch2", "name": "High", "league": "Standard", "level": 90 },
            { "id": "ch3", "name": "Elsewhere", "league": "Hardcore", "level": 50 },
        ]),
        10,
    );
    fetch_character(
        &mut s,
        "pc",
        json!({ "id": "ch2", "name": "High", "league": "Standard", "level": 90,
                "inventory": [item("bag", "Doom Loop", "Iron Ring", "Rare", json!({}))], "equipment": [] }),
        20,
    );
    let places = places(
        &s,
        Listed::Characters,
        "pc",
        "Standard",
        &Asking::default(),
        NOW,
    )
    .unwrap();
    let doc = serde_json::to_value(&places).unwrap();
    let order: Vec<&str> = places.places.iter().map(|p| p.id()).collect();
    assert_eq!(order, ["ch2", "ch1"], "the store's order: level first");
    assert!(places.places.iter().all(|p| p.place == Kind::Character));
    assert_eq!(
        (
            &doc["characters"],
            &doc["fetched"],
            &doc["never_fetched"],
            &doc["items"]
        ),
        (&json!(2), &json!(1), &json!(1), &json!(1)),
        "{doc}"
    );
    assert_eq!(doc_place(&doc, "ch2")["request"]["query"]["text"], "id:ch2");
    let corpus = load(&s, Some("pc"));
    let mut request = doc_place(&doc, "ch2")["request"].clone();
    request["view"] = json!({ "rows": { "limit": 100 } });
    let request: acquisition_search::Request = serde_json::from_value(request).unwrap();
    let a = as_json(&answer(&corpus, &request).unwrap());
    assert_eq!(ids(&a), ["bag"]);
    // the place an item's row names is the character's, held alone
    let place = a["rows"][0]["place"]["id"].as_str().unwrap();
    assert_eq!(places.holders(place).as_deref(), Some(&["ch2"][..]));
}

/// A place of a places document by its id, anywhere in the tree.
fn doc_place<'a>(doc: &'a Value, id: &str) -> &'a Value {
    fn find<'a>(places: &'a Value, id: &str) -> Option<&'a Value> {
        places.as_array()?.iter().find_map(|p| {
            (p["id"] == id)
                .then_some(p)
                .or_else(|| find(&p["places"], id))
        })
    }
    find(&doc["places"], id).unwrap_or_else(|| panic!("no place {id}: {doc}"))
}

/// The stash tree: a folder, a map tab with pages, a page whose tab the
/// store does not hold, a tab fetched empty and one never fetched.
fn stash_tree() -> Store {
    use acquisition_store::Endpoint;
    let mut s = store();
    list_tabs(
        &mut s,
        "pc",
        "Standard",
        json!([
            tab("t1", "Rings"),
            { "id": "f1", "name": "Old", "type": "Folder", "children": [
                { "id": "c1", "name": "Old 1", "type": "PremiumStash" },
                { "id": "c2", "name": "Old 2", "type": "PremiumStash" } ] },
            { "id": "m1", "name": "Maps", "type": "MapStash" },
            tab("e1", "Empty"),
        ]),
        10,
    );
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
    fetch_tab(&mut s, "pc", "Standard", "e1", "Empty", vec![], 20);
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "c1",
        "Old 1",
        vec![item("o1", "Grim Knot", "Iron Ring", "Rare", json!({}))],
        20,
    );
    let mut stash = |id: &str, sub: Option<&str>, body: Value| {
        s.record(
            &Endpoint::Stash {
                realm: "pc".into(),
                league: "Standard".into(),
                id: id.into(),
                sub: sub.map(str::to_string),
            },
            &json!({ "realm": "pc", "league": "Standard" }),
            200,
            &body,
            30,
        )
        .unwrap();
    };
    stash(
        "m1",
        None,
        json!({ "stash": { "id": "m1", "name": "Maps", "type": "MapStash", "items": [], "children": [
            { "id": "s1", "name": "Tier 16", "type": "MapStash", "parent": "m1", "metadata": { "items": 2 } },
            { "id": "s2", "name": "Tier 15", "type": "MapStash", "parent": "m1", "metadata": { "items": 1 } } ] } }),
    );
    stash(
        "m1",
        Some("s1"),
        json!({ "stash": { "id": "s1", "name": "Tier 16", "type": "MapStash", "parent": "m1", "items": [
            item("map1", "", "Crimson Temple Map", "Normal", json!({})),
            item("map2", "", "Crimson Temple Map", "Normal", json!({})) ] } }),
    );
    // a page whose tab the store never held
    stash(
        "x9",
        Some("p1"),
        json!({ "stash": { "id": "p1", "name": "Lost", "type": "UniqueStash", "parent": "x9", "items": [
            item("lost1", "Kaom's Heart", "Glorious Plate", "Unique", json!({})) ] } }),
    );
    s
}

/// What an answer's rows say of their order and values: the view a
/// place's request answers.
fn rows_view(a: &acquisition_search::Answer) -> (Vec<String>, Option<String>, bool) {
    match &a.view {
        acquisition_search::answer::ViewOut::Rows(rows) => {
            (rows.values.clone(), rows.sort.clone(), rows.desc)
        }
        other => panic!("no rows: {other:?}"),
    }
}

/// Each place's `matched` held to its own request: a place with a
/// request, that request's `total.matched`; a folder, its places' sum;
/// the top level's sum the document's `query.matched`; and the query's
/// own `total` its request's. Each request answers the query's own
/// values, sort and order. Returns the document.
fn held_to_their_requests(s: &Store, league: &str, asking: &Asking) -> Value {
    let places = places(s, Listed::Tabs, "pc", league, asking, NOW).unwrap();
    let doc = serde_json::to_value(&places).unwrap();
    let corpus = load(s, Some("pc"));
    let queried = places.query.as_ref().expect("a query");
    let own = answer(&corpus, &queried.request).unwrap();
    assert_eq!(queried.total.matched, own.total.matched, "{doc}");
    assert_eq!(places.basis.as_ref(), Some(&corpus.basis));
    for place in places.every() {
        let matched = place
            .matched
            .unwrap_or_else(|| panic!("{}: {doc}", place.id()));
        match &place.request {
            Some(request) => {
                let answered = answer(&corpus, request).unwrap();
                assert_eq!(answered.total.matched, matched, "{}: {doc}", place.id());
                assert_eq!(rows_view(&answered), rows_view(&own), "{}", place.id());
            }
            None => assert_eq!(
                place
                    .places
                    .iter()
                    .map(|p| p.matched.unwrap())
                    .sum::<usize>(),
                matched,
                "{}",
                place.id()
            ),
        }
    }
    let top: usize = places.places.iter().map(|p| p.matched.unwrap()).sum();
    assert_eq!(queried.matched, top, "{doc}");
    // the matches this league's places do not hold are said where they
    // stand, so the page's count and the rest make the query's whole
    let elsewhere: usize = queried.elsewhere.iter().map(|e| e.matched).sum();
    assert_eq!(queried.matched + elsewhere, queried.total.matched, "{doc}");
    doc
}

/// Under a query: each place's matches are its request's, a folder's
/// its places' sum, the top level's the document's, over the stash
/// tree's fixture with a map tab holding an item of its own beside its
/// pages, which says it as `own`; in a view that names a value, a sort
/// and an order too; and over one tab id listed under two spellings of
/// a league (`seat_faults.rs`), each spelling's place its own match
/// alone.
#[test]
fn under_a_query_each_place_counts_what_its_request_matches() {
    use acquisition_search::answer::Rows;
    use acquisition_store::Endpoint;
    let mut s = stash_tree();
    s.record(
        &Endpoint::Stash {
            realm: "pc".into(),
            league: "Standard".into(),
            id: "m1".into(),
            sub: None,
        },
        &json!({ "realm": "pc", "league": "Standard" }),
        200,
        &json!({ "stash": { "id": "m1", "name": "Maps", "type": "MapStash", "items": [
            item("own1", "", "Crimson Temple Map", "Normal", json!({})) ], "children": [
            { "id": "s1", "name": "Tier 16", "type": "MapStash", "parent": "m1", "metadata": { "items": 2 } },
            { "id": "s2", "name": "Tier 15", "type": "MapStash", "parent": "m1", "metadata": { "items": 1 } } ] } }),
        40,
    )
    .unwrap();
    let asked = |text: &str, view: Rows| Asking {
        query: Some(text.to_string()),
        view,
        ..Asking::default()
    };
    let doc = held_to_their_requests(&s, "Standard", &asked("rarity=normal", Rows::default()));
    let place = |id: &str| doc_place(&doc, id).clone();
    assert_eq!(
        (&place("m1")["matched"], &place("m1")["own"]),
        (&json!(3), &json!(1)),
        "the map tab's own item beside its pages': {doc}"
    );
    assert_eq!(place("s1")["matched"], 2);
    // `own` is a tab's with pages alone
    for none in ["t1", "s1", "p1", "c1"] {
        assert!(place(none).get("own").is_none(), "{none}");
    }
    assert_eq!(doc["query"]["text"], "rarity=normal");
    assert_eq!(doc["query"]["matched"], 3);
    let doc = held_to_their_requests(&s, "Standard", &asked("rarity=rare", Rows::default()));
    assert_eq!(
        doc_place(&doc, "f1")["matched"],
        1,
        "a folder, its tabs' sum"
    );
    assert_eq!(doc_place(&doc, "t1")["matched"], 2);
    let ordered = Rows {
        values: vec!["ilvl".to_string()],
        sort: Some("ilvl".to_string()),
        desc: true,
        ..Rows::default()
    };
    let doc = held_to_their_requests(&s, "Standard", &asked("ilvl>=1", ordered));
    assert_eq!(
        doc["query"]["request"]["view"]["rows"]["sort"], "ilvl",
        "{doc}"
    );

    // one tab id under two spellings of a league
    let mut s = store();
    for (league, id, at) in [("Standard", "a", 10), ("standard", "b", 11)] {
        list_tabs(&mut s, "pc", league, json!([tab("d1", "Dump")]), at);
        fetch_tab(
            &mut s,
            "pc",
            league,
            "d1",
            "Dump",
            vec![item(id, "Doom Loop", "Iron Ring", "Rare", json!({}))],
            at + 10,
        );
    }
    for league in ["Standard", "standard"] {
        let doc = held_to_their_requests(&s, league, &asked("rarity=rare", Rows::default()));
        assert_eq!(doc_place(&doc, "d1")["matched"], 1, "{league}: {doc}");
        assert_eq!(doc["query"]["total"]["matched"], 2, "every league's");
        let other = if league == "Standard" {
            "standard"
        } else {
            "Standard"
        };
        assert_eq!(
            doc["query"]["elsewhere"],
            json!([{ "place": "stash", "league": other, "matched": 1 }]),
            "the other spelling's: {doc}"
        );
    }
}

/// Which places a document lists, in order, and the folds: the top
/// level's, then each parent's, by its id.
fn folds(doc: &Value) -> (Vec<String>, Value, Vec<(String, Value)>) {
    let mut listed = Vec::new();
    let mut parents = Vec::new();
    fn walk(places: &Value, listed: &mut Vec<String>, parents: &mut Vec<(String, Value)>) {
        for p in places.as_array().into_iter().flatten() {
            let id = p["id"].as_str().unwrap().to_string();
            if p["listed"] == true {
                listed.push(id.clone());
            }
            if let Some(folded) = p.get("folded") {
                parents.push((id, folded.clone()));
            }
            walk(&p["places"], listed, parents);
        }
    }
    walk(&doc["places"], &mut listed, &mut parents);
    (listed, doc["folded"].clone(), parents)
}

/// The filter and the query fold: a place is listed where it passes
/// every field and, under a query, holds a match, or where it holds a
/// listed place; each level counts what it folds and of that the never
/// fetched. Unasked, no key of the fold stands, as in the places'
/// document. A `fetched` that is not `never`, a type on the characters,
/// and a query the search refuses are refused.
#[test]
fn a_filter_and_a_query_fold_the_places_the_page_does_not_list() {
    use acquisition_search::places::Filter;
    let s = stash_tree();
    let doc = |asking: &Asking| {
        serde_json::to_value(places(&s, Listed::Tabs, "pc", "Standard", asking, NOW).unwrap())
            .unwrap()
    };
    let unasked = doc(&Asking::default());
    for key in ["filter", "query", "basis", "folded"] {
        assert!(unasked.get(key).is_none(), "{key}: {unasked}");
    }
    for place in places(&s, Listed::Tabs, "pc", "Standard", &Asking::default(), NOW)
        .unwrap()
        .every()
    {
        let place = serde_json::to_value(place).unwrap();
        for key in ["listed", "folded", "matched", "own"] {
            assert!(place.get(key).is_none(), "{key}: {place}");
        }
    }
    let filtered = |name: Option<&str>, r#type: Option<&str>, fetched: Option<&str>| Asking {
        filter: Filter::read(name, r#type, fetched).unwrap(),
        ..Asking::default()
    };
    let id = |ids: &[&str]| ids.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let n = |places: usize, never: usize| json!({ "places": places, "never_fetched": never });
    // words the name holds, each, case aside: a folder listed for a tab
    // it holds
    let (listed, top, parents) = folds(&doc(&filtered(Some("old 2"), None, None)));
    assert_eq!(listed, id(&["f1", "c2"]));
    assert_eq!(top, n(4, 0), "t1, m1, e1, p1");
    assert_eq!(
        parents,
        [("f1".to_string(), n(1, 0)), ("m1".to_string(), n(2, 1))]
    );
    // the type, whole, case aside: a map tab and its pages
    let asked = doc(&filtered(None, Some("mapstash"), None));
    assert_eq!(asked["filter"], json!({ "type": "mapstash" }));
    let (listed, top, _) = folds(&asked);
    assert_eq!(listed, id(&["m1", "s1", "s2"]));
    assert_eq!(top, n(4, 0), "t1, f1, e1, p1");
    // never fetched: each with the parent that holds it
    let (listed, top, parents) = folds(&doc(&filtered(None, None, Some("never"))));
    assert_eq!(listed, id(&["f1", "c2", "m1", "s2"]));
    assert_eq!(top, n(3, 0), "t1, e1, p1");
    assert_eq!(
        parents,
        [("f1".to_string(), n(1, 0)), ("m1".to_string(), n(1, 0))]
    );
    // a query: the places that hold a match, the never fetched counted
    // where they fold
    let (listed, top, parents) = folds(&doc(&Asking {
        query: Some("rarity=rare".to_string()),
        ..Asking::default()
    }));
    assert_eq!(listed, id(&["t1", "f1", "c1"]));
    assert_eq!(top, n(3, 0), "m1, e1, p1");
    assert_eq!(
        parents,
        [("f1".to_string(), n(1, 1)), ("m1".to_string(), n(2, 1))]
    );
    // a filter and a query together: both
    let (listed, top, _) = folds(&doc(&Asking {
        query: Some("rarity=rare".to_string()),
        ..filtered(Some("rings"), None, None)
    }));
    assert_eq!(listed, id(&["t1"]));
    assert_eq!(top, n(4, 0));

    // refused: a fetch other than never, a character's type, a query the
    // search refuses, each as the search refuses it
    let kind = |asking: Result<Asking, acquisition_search::SearchError>, listed: Listed| {
        asking
            .and_then(|asking| places(&s, listed, "pc", "Standard", &asking, NOW))
            .unwrap_err()
            .kind()
    };
    assert_eq!(
        kind(
            Filter::read(None, None, Some("yesterday")).map(|filter| Asking {
                filter,
                ..Asking::default()
            }),
            Listed::Tabs
        ),
        "filter"
    );
    assert_eq!(
        kind(
            Ok(filtered(None, Some("Premium"), None)),
            Listed::Characters
        ),
        "filter"
    );
    let refused = kind(
        Ok(Asking {
            query: Some("ilvl>=".to_string()),
            ..Asking::default()
        }),
        Listed::Tabs,
    );
    let search = ask(&load(&s, Some("pc")), "ilvl>=").unwrap_err().kind();
    assert_eq!(refused, search);
}
