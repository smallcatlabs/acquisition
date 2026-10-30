use super::*;
use crate::Endpoint;
use serde_json::{Value, json};

fn profile(s: &mut Store) {
    s.record(
        &Endpoint::Profile,
        &json!({}),
        200,
        &json!({ "uuid": "u-1", "name": "tom" }),
        1,
    )
    .unwrap();
}

fn item(id: &str, name: &str) -> Value {
    json!({ "id": id, "name": name, "typeLine": "Imperial Bow", "baseType": "Imperial Bow", "x": 0, "y": 0 })
}

fn list_tabs(s: &mut Store, realm: &str, league: &str, tabs: Value, at: i64) {
    s.record(
        &Endpoint::Stashes {
            realm: realm.into(),
            league: league.into(),
        },
        &json!({ "realm": realm, "league": league }),
        200,
        &json!({ "stashes": tabs }),
        at,
    )
    .unwrap();
}

fn fetch_tab(s: &mut Store, realm: &str, league: &str, id: &str, items: Vec<Value>, at: i64) {
    s.record(
        &Endpoint::Stash {
            realm: realm.into(),
            league: league.into(),
            id: id.into(),
            sub: None,
        },
        &json!({ "realm": realm, "league": league, "id": id }),
        200,
        &json!({ "stash": { "id": id, "name": id, "type": "PremiumStash", "items": items } }),
        at,
    )
    .unwrap();
}

fn list_characters(s: &mut Store, realm: &str, entries: Value, at: i64) {
    s.record(
        &Endpoint::Characters {
            realm: realm.into(),
        },
        &json!({ "realm": realm }),
        200,
        &json!({ "characters": entries }),
        at,
    )
    .unwrap();
}

fn fetch_character(s: &mut Store, realm: &str, character: Value, at: i64) {
    let name = character["name"].as_str().unwrap().to_string();
    s.record(
        &Endpoint::Character {
            realm: realm.into(),
            name: name.clone(),
        },
        &json!({ "realm": realm, "name": name }),
        200,
        &json!({ "character": character }),
        at,
    )
    .unwrap();
}

fn tab(id: &str) -> Value {
    json!({ "id": id, "name": id, "type": "PremiumStash" })
}

fn read_all(s: &Store, realm: RealmScope<'_>) -> (CorpusHeader, Vec<CorpusItem>) {
    s.read_corpus(realm, |header, items| {
        Ok((header.clone(), items.collect::<Result<Vec<_>>>()?))
    })
    .unwrap()
}

fn ids(items: &[CorpusItem]) -> Vec<&str> {
    items.iter().map(|i| i.id.as_str()).collect()
}

/// C108 — on request the read hands over the removed with when each
/// left: an item a fetch no longer had, and an item whose tab a
/// listing retired, at that tab, which the header then names and
/// marks. The live read is what it was, and says which ids it left
/// out.
#[test]
fn all_hands_over_the_removed_with_when_each_left_and_live_is_unchanged() {
    let mut s = Store::open_memory().unwrap();
    profile(&mut s);
    list_tabs(&mut s, "pc", "Standard", json!([tab("t1"), tab("t2")]), 10);
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "t1",
        vec![item("a", "Stays"), item("b", "Leaves")],
        20,
    );
    fetch_tab(&mut s, "pc", "Standard", "t2", vec![item("c", "Goes")], 30);
    fetch_tab(&mut s, "pc", "Standard", "t1", vec![item("a", "Stays")], 50);
    list_tabs(&mut s, "pc", "Standard", json!([tab("t1")]), 60);

    let (header, items) = read_all(&s, RealmScope::All);
    assert_eq!(ids(&items), ["a"]);
    assert!(items.iter().all(|i| i.removed_at.is_none()));
    let named: Vec<&str> = header.locations.iter().map(|l| l.id.as_str()).collect();
    assert_eq!(named, ["t1"]);
    assert!(header.locations.iter().all(|l| l.removed_at.is_none()));
    let gone: Vec<(&str, &str, i64)> = header
        .gone
        .iter()
        .map(|g| (g.kind.as_str(), g.id.as_str(), g.removed_at))
        .collect();
    assert_eq!(
        gone,
        [("item", "b", 50), ("item", "c", 60), ("stash", "t2", 60)]
    );

    let (header, items) = s
        .read_corpus_of(RealmScope::All, Membership::All, |header, items| {
            Ok((header.clone(), items.collect::<Result<Vec<_>>>()?))
        })
        .unwrap();
    let left: Vec<(&str, &str, Option<i64>)> = items
        .iter()
        .map(|i| (i.id.as_str(), i.location_id.as_str(), i.removed_at))
        .collect();
    assert_eq!(
        left,
        [
            ("a", "t1", None),
            ("b", "t1", Some(50)),
            ("c", "t2", Some(60))
        ]
    );
    assert!(
        items
            .iter()
            .all(|i| i.league.as_deref() == Some("Standard"))
    );
    let marked: Vec<(&str, Option<i64>)> = header
        .locations
        .iter()
        .map(|l| (l.id.as_str(), l.removed_at))
        .collect();
    assert_eq!(marked, [("t1", None), ("t2", Some(60))]);
    assert!(header.gone.is_empty());
    // one snapshot either way: the same revision
    assert_eq!(header.revision, s.revision().unwrap());
}

/// C108, C98 — the read is one snapshot: a fetch that lands part-way
/// through the items changes nothing the read then says, and the next
/// read carries a higher revision and the change.
#[test]
fn a_fetch_landing_mid_read_is_the_next_reads_and_never_this_ones() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("facts.db");
    let mut a = Store::open(&path).unwrap();
    profile(&mut a);
    list_tabs(&mut a, "pc", "Standard", json!([tab("t1"), tab("t2")]), 10);
    fetch_tab(
        &mut a,
        "pc",
        "Standard",
        "t1",
        vec![item("a", "Old"), item("b", "Gone Soon")],
        20,
    );
    let before = a.revision().unwrap();
    let mut b = Store::open(&path).unwrap();

    let (header, items) = a
        .read_corpus(RealmScope::All, |header, items| {
            // The second handle commits twice while this read is open.
            // Before the first item is stepped to — where a read with
            // no transaction would begin a newer snapshot than its
            // header's — `a` is rewritten, `b` leaves and `z` arrives;
            // part-way through the items, t2 is fetched for the first
            // time.
            fetch_tab(
                &mut b,
                "pc",
                "Standard",
                "t1",
                vec![item("a", "New"), item("z", "Arrived")],
                30,
            );
            let first = items.next().unwrap()?;
            fetch_tab(&mut b, "pc", "Standard", "t2", vec![item("c", "Late")], 31);
            assert!(b.revision().unwrap().response > before.response);
            let mut all = vec![first];
            for next in items {
                all.push(next?);
            }
            Ok((header.clone(), all))
        })
        .unwrap();
    assert_eq!(header.revision, before);
    assert_eq!(ids(&items), ["a", "b"]);
    assert!(items[0].body.contains("\"Old\""));
    let t =
        |h: &CorpusHeader, id: &str| h.locations.iter().find(|l| l.id == id).unwrap().fetched_at;
    assert_eq!((t(&header, "t1"), t(&header, "t2")), (Some(20), None));

    let (header, items) = read_all(&a, RealmScope::All);
    assert_eq!(header.revision.response, before.response + 2);
    assert_eq!(ids(&items), ["a", "z", "c"]);
    assert!(items[0].body.contains("\"New\""));
    assert_eq!((t(&header, "t1"), t(&header, "t2")), (Some(30), Some(31)));
}

/// C108 (S131) — league is joined as `read_items` joins it: a character
/// item takes the character's current listing league, never its own
/// stamp, and a league-less character's items carry none.
#[test]
fn a_character_items_league_is_the_listings_and_none_when_the_listing_gave_none() {
    let mut s = Store::open_memory().unwrap();
    profile(&mut s);
    list_characters(
        &mut s,
        "pc",
        json!([{ "id": "c1", "name": "Mover", "league": "Allflame" },
                   { "id": "c2", "name": "Nowhere" }]),
        10,
    );
    fetch_character(
        &mut s,
        "pc",
        json!({ "id": "c1", "name": "Mover", "league": "Allflame", "equipment": [item("helm", "Helm")] }),
        20,
    );
    fetch_character(
        &mut s,
        "pc",
        json!({ "id": "c2", "name": "Nowhere", "inventory": [item("scrap", "Scrap")] }),
        21,
    );
    // The league ends: the listing moves the character, and no fetch
    // has restamped its items.
    list_characters(
        &mut s,
        "pc",
        json!([{ "id": "c1", "name": "Mover", "league": "Standard" },
                   { "id": "c2", "name": "Nowhere" }]),
        30,
    );
    let stamp: Option<String> = s
        .conn
        .query_row("SELECT league FROM items WHERE id = 'helm'", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(stamp.as_deref(), Some("Allflame"));

    let (header, items) = read_all(&s, RealmScope::All);
    let league = |id: &str| items.iter().find(|i| i.id == id).unwrap().league.clone();
    assert_eq!(league("helm").as_deref(), Some("Standard"));
    assert_eq!(league("scrap"), None);
    let helm = items.iter().find(|i| i.id == "helm").unwrap();
    assert_eq!(
        (helm.location_kind.as_str(), helm.location_id.as_str()),
        ("character", "c1")
    );
    assert_eq!(helm.container.as_deref(), Some("equipment"));
    let of = |id: &str| header.locations.iter().find(|l| l.id == id).unwrap();
    assert_eq!(of("c1").league.as_deref(), Some("Standard"));
    assert_eq!(of("c2").league, None);
    assert_eq!(of("c1").name, "Mover");
}

/// C108, C54 — live is the item and its location both, a location is
/// its full coordinate, and the scope is one realm or all while the
/// realms are every realm the file holds.
#[test]
fn only_live_items_at_live_locations_by_full_coordinate_in_the_scope_asked() {
    let mut s = Store::open_memory().unwrap();
    profile(&mut s);
    list_tabs(&mut s, "pc", "Standard", json!([tab("t1"), tab("t2")]), 10);
    list_tabs(&mut s, "xbox", "Standard", json!([tab("t1")]), 11);
    let mut host = item("bow", "Bow");
    host["socketedItems"] = json!([item("gem", "Gem")]);
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "t1",
        vec![host, item("sold", "Sold")],
        20,
    );
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "t2",
        vec![item("doomed", "Doomed")],
        21,
    );
    fetch_tab(
        &mut s,
        "xbox",
        "Standard",
        "t1",
        vec![item("pad", "Pad")],
        22,
    );
    list_characters(
        &mut s,
        "poe2",
        json!([{ "id": "c9", "name": "Two", "league": "Standard" }]),
        23,
    );
    // `sold` leaves its tab; a listing retires t2 and its item with it.
    let mut host = item("bow", "Bow");
    host["socketedItems"] = json!([item("gem", "Gem")]);
    fetch_tab(&mut s, "pc", "Standard", "t1", vec![host], 30);
    list_tabs(&mut s, "pc", "Standard", json!([tab("t1")]), 31);

    let (header, items) = read_all(&s, RealmScope::All);
    assert_eq!(header.realms, ["pc", "poe2", "xbox"]);
    assert_eq!(
        (header.account_uuid.as_str(), header.account_name.as_deref()),
        ("u-1", Some("tom"))
    );
    // Location kind and id, then realm, then item id.
    assert_eq!(ids(&items), ["bow", "gem", "pad"]);
    assert_eq!(items[1].socketed_in.as_deref(), Some("bow"));
    assert!(!items[0].body.contains("socketedItems"));
    assert_eq!((items[0].first_seen, items[0].last_seen), (20, 30));
    let places: Vec<(&str, &str, &str)> = header
        .locations
        .iter()
        .map(|l| (l.realm.as_str(), l.kind.as_str(), l.id.as_str()))
        .collect();
    assert_eq!(
        places,
        [
            ("pc", "stash", "t1"),
            ("xbox", "stash", "t1"),
            ("poe2", "character", "c9")
        ]
    );

    let (header, items) = read_all(&s, RealmScope::One("xbox"));
    assert_eq!(header.realms, ["pc", "poe2", "xbox"]);
    assert_eq!(ids(&items), ["pad"]);
    assert_eq!(items[0].realm, "xbox");
    assert_eq!(header.locations.len(), 1);
}

/// C108, C54 — the header is what coverage is stated from: a folder is
/// told apart from a tab no fetch has reached, a substash whose tab a
/// listing retired is no live location, and the latest listing of each
/// list is carried, one that named nothing included.
#[test]
fn the_header_tells_folders_orphans_and_empty_listings_as_they_are() {
    let mut s = Store::open_memory().unwrap();
    profile(&mut s);
    let folder = json!({ "id": "f1", "name": "Sale", "type": "Folder",
                             "children": [{ "id": "c1", "name": "Bows", "type": "PremiumStash" }] });
    let maps = json!({ "id": "m1", "name": "Maps", "type": "MapStash" });
    list_tabs(
        &mut s,
        "pc",
        "Standard",
        json!([folder.clone(), maps, tab("t9")]),
        10,
    );
    fetch_tab(&mut s, "pc", "Standard", "c1", vec![item("bow", "Bow")], 20);
    let stash = |sub: Option<&str>| Endpoint::Stash {
        realm: "pc".into(),
        league: "Standard".into(),
        id: "m1".into(),
        sub: sub.map(str::to_string),
    };
    s.record(
            &stash(None),
            &json!({ "id": "m1" }),
            200,
            &json!({ "stash": { "id": "m1", "name": "Maps", "type": "MapStash", "items": [],
                                "children": [{ "id": "s1", "name": "Tier 1", "type": "MapStash" }] } }),
            21,
        )
        .unwrap();
    s.record(
            &stash(Some("s1")),
            &json!({ "id": "m1", "sub": "s1" }),
            200,
            &json!({ "stash": { "id": "s1", "name": "Tier 1", "type": "MapStash", "items": [item("map", "Map")] } }),
            22,
        )
        .unwrap();
    list_characters(&mut s, "pc", json!([]), 40);

    let (header, items) = read_all(&s, RealmScope::All);
    assert_eq!(ids(&items), ["bow", "map"]);
    let row = |h: &CorpusHeader, id: &str| h.locations.iter().find(|l| l.id == id).cloned();
    let f1 = row(&header, "f1").unwrap();
    assert_eq!(
        (f1.tab_type.as_deref(), f1.fetched_at),
        (Some("Folder"), None)
    );
    let c1 = row(&header, "c1").unwrap();
    assert_eq!(
        (c1.tab_type.as_deref(), c1.parent.as_deref(), c1.fetched_at),
        (Some("PremiumStash"), Some("f1"), Some(20))
    );
    assert_eq!(row(&header, "t9").unwrap().fetched_at, None);
    let s1 = row(&header, "s1").unwrap();
    assert_eq!(
        (s1.parent.as_deref(), s1.fetched_at),
        (Some("m1"), Some(22))
    );
    let seen: Vec<(&str, Option<&str>, &str, i64)> = header
        .listings
        .iter()
        .map(|l| {
            (
                l.realm.as_str(),
                l.league.as_deref(),
                l.kind.as_str(),
                l.fetched_at,
            )
        })
        .collect();
    assert_eq!(
        seen,
        [
            ("pc", Some("Standard"), "stash", 10),
            ("pc", None, "character", 40)
        ]
    );
    assert_eq!(header.listings[0].response_id, 2);

    // A listing drops the map tab: its substash keeps its row for the
    // planner's orphan report and is no live location here.
    list_tabs(&mut s, "pc", "Standard", json!([folder, tab("t9")]), 50);
    let kept: i64 = s
        .conn
        .query_row(
            "SELECT count(*) FROM tabs WHERE id = 's1' AND removed_at IS NULL",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(kept, 1);
    let (header, items) = read_all(&s, RealmScope::All);
    assert_eq!(ids(&items), ["bow"]);
    assert!(row(&header, "m1").is_none() && row(&header, "s1").is_none());
    assert_eq!(header.listings[0].fetched_at, 50);

    let (header, _) = read_all(&s, RealmScope::One("xbox"));
    assert!(header.listings.is_empty() && header.locations.is_empty());
}

/// C108, C98 — the revision never goes back and moves with every
/// committed record; a refusal, a rebuild and a reopen leave it. A new
/// write door on the facts file joins this test (the module doc's two
/// rules for a prune).
#[test]
fn the_revision_moves_with_every_record_and_with_nothing_else() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("facts.db");
    let mut s = Store::open(&path).unwrap();
    let empty = s.revision().unwrap();
    assert_eq!((empty.response, empty.facts_version), (0, 9));
    let mut last = empty.response;
    fn moved(s: &Store, last: &mut i64, what: &str) {
        let now = s.revision().unwrap().response;
        assert!(now > *last, "{what}: {last} -> {now}");
        *last = now;
    }
    profile(&mut s);
    moved(&s, &mut last, "profile");
    list_tabs(&mut s, "pc", "Standard", json!([tab("t1"), tab("t2")]), 10);
    moved(&s, &mut last, "a listing");
    fetch_tab(&mut s, "pc", "Standard", "t2", vec![item("a", "A")], 20);
    moved(&s, &mut last, "a fetch");
    list_tabs(&mut s, "pc", "Standard", json!([tab("t1")]), 30);
    moved(&s, &mut last, "a listing that retires a tab and its item");
    fetch_tab(&mut s, "pc", "Standard", "t2", vec![item("a", "A")], 40);
    moved(&s, &mut last, "a withheld fetch");

    let held = read_all(&s, RealmScope::All);
    let refused = s.record(
        &Endpoint::Stash {
            realm: "pc".into(),
            league: "Standard".into(),
            id: "t1".into(),
            sub: None,
        },
        &json!({ "id": "t1" }),
        200,
        &json!({ "stash": { "id": "t1", "items": [{ "name": "no id" }] } }),
        50,
    );
    assert!(refused.is_err());
    s.rebuild().unwrap();
    drop(s);
    let mut s = Store::open(&path).unwrap();
    assert_eq!(s.revision().unwrap().response, last);
    assert_eq!(read_all(&s, RealmScope::All), held);

    fetch_tab(&mut s, "pc", "Standard", "t1", vec![item("b", "B")], 60);
    moved(&s, &mut last, "the record after a refusal");
}

/// C108 — the read hands over ingest facts and the body as stored: no
/// derived column reaches it or orders it, and a body that does not
/// parse arrives as it is.
#[test]
fn derived_columns_never_reach_the_read_and_a_bad_body_arrives_as_text() {
    let mut s = Store::open_memory().unwrap();
    profile(&mut s);
    list_tabs(&mut s, "pc", "Standard", json!([tab("t1")]), 10);
    let at = |id: &str, x: i64| {
        let mut i = item(id, id);
        i["x"] = json!(x);
        i
    };
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "t1",
        vec![at("m", 5), at("k", 9), at("q", 1)],
        20,
    );
    let before = read_all(&s, RealmScope::All);
    assert_eq!(ids(&before.1), ["k", "m", "q"]);
    s.conn
        .execute("UPDATE items SET name = 'wrong', x = 100 - x, y = x", [])
        .unwrap();
    assert_eq!(read_all(&s, RealmScope::All), before);

    s.conn
        .execute("UPDATE items SET json = 'not json' WHERE id = 'm'", [])
        .unwrap();
    let (_, items) = read_all(&s, RealmScope::All);
    assert_eq!(items[1].body, "not json");
}

/// C108 — a facts file that records no account is refused, as the
/// snapshots refuse it: the basis names the account (C98).
#[test]
fn a_store_with_no_account_on_record_is_refused() {
    let s = Store::open_memory().unwrap();
    let err = s
        .read_corpus(RealmScope::All, |_, _| Ok(()))
        .unwrap_err()
        .to_string();
    assert!(err.contains("records no account identity"), "{err}");
}
