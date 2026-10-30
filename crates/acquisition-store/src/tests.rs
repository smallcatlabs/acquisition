use super::*;

/// `status` said `leagues: 0` beside tabs in seven leagues, a bare
/// number (the re-sit, mcp-sonnet call 3, its fault 7): `league_list`
/// says whether the list was ever fetched, and a fetch that listed none
/// is a fetch (review 2 of the re-sit), its count still none.
#[test]
fn the_league_list_is_never_fetched_until_a_fetch_even_one_that_lists_none() {
    let mut s = Store::open_memory().unwrap();
    let status = s.status().unwrap();
    assert_eq!(
        (status.leagues, status.league_list.as_str()),
        (0, "never fetched")
    );
    s.record(
        &Endpoint::Leagues,
        &json!({}),
        200,
        &json!({ "leagues": [] }),
        50,
    )
    .unwrap();
    let status = s.status().unwrap();
    assert_eq!(
        (status.leagues, status.league_list.as_str()),
        (0, "fetched")
    );
}

fn leagues(ids: &[&str]) -> Value {
    json!({ "leagues": ids.iter().map(|id| json!({ "id": id, "realm": "pc" })).collect::<Vec<_>>() })
}

/// C126 (a league the list lacks is a note): the list is the latest
/// response's, so a league the game retired is not held, though the
/// `leagues` table, which upserts, still has its row.
#[test]
fn a_league_that_left_the_list_is_not_held() {
    let mut s = Store::open_memory().unwrap();
    s.record(
        &Endpoint::Leagues,
        &json!({}),
        200,
        &leagues(&["Standard", "Mercenaries"]),
        50,
    )
    .unwrap();
    s.record(
        &Endpoint::Leagues,
        &json!({}),
        200,
        &leagues(&["Standard", "Rehearsal"]),
        60,
    )
    .unwrap();
    assert_eq!(
        s.league_list().unwrap(),
        Some(LeagueList {
            fetched_at: 60,
            ids: vec!["Standard".into(), "Rehearsal".into()]
        })
    );
    assert_eq!(
        s.status().unwrap().leagues,
        3,
        "the table keeps the retired row"
    );
}

/// C126: the list is the response recorded last, never the one whose
/// fetch time is largest: a clock set back between two fetches still
/// reads the later list.
#[test]
fn the_later_list_is_read_though_its_clock_ran_back() {
    let mut s = Store::open_memory().unwrap();
    s.record(
        &Endpoint::Leagues,
        &json!({}),
        200,
        &leagues(&["Standard", "Mercenaries"]),
        100,
    )
    .unwrap();
    s.record(
        &Endpoint::Leagues,
        &json!({}),
        200,
        &leagues(&["Standard", "Rehearsal"]),
        99,
    )
    .unwrap();
    assert_eq!(
        s.league_list().unwrap(),
        Some(LeagueList {
            fetched_at: 99,
            ids: vec!["Standard".into(), "Rehearsal".into()]
        })
    );
}

/// C126: no list before a fetch; a fetch that listed none is a list,
/// fetched, holding nothing.
#[test]
fn a_list_fetched_empty_is_a_fetched_list() {
    let mut s = Store::open_memory().unwrap();
    assert_eq!(s.league_list().unwrap(), None);
    s.record(&Endpoint::Leagues, &json!({}), 200, &leagues(&[]), 70)
        .unwrap();
    assert_eq!(
        s.league_list().unwrap(),
        Some(LeagueList {
            fetched_at: 70,
            ids: vec![]
        })
    );
}

fn item(id: &str, name: &str, x: i64) -> Value {
    json!({ "id": id, "name": name, "typeLine": "Imperial Bow", "baseType": "Imperial Bow", "x": x, "y": 0, "w": 2, "h": 4, "inventoryId": "Stash1", "league": "Standard" })
}

fn stash(id: &str, items: Vec<Value>) -> Value {
    json!({ "stash": { "id": id, "name": format!("tab {id}"), "type": "PremiumStash", "index": 0, "metadata": { "colour": "7c5436" }, "items": items } })
}

fn stash_ep(id: &str) -> Endpoint {
    Endpoint::Stash {
        realm: "pc".into(),
        league: "Standard".into(),
        id: id.into(),
        sub: None,
    }
}

#[test]
fn stash_items_are_lifted_and_the_envelope_keeps_the_count() {
    let mut s = Store::open_memory().unwrap();
    let ing = s
        .record(
            &stash_ep("a"),
            &json!({"league":"Standard","id":"a"}),
            200,
            &stash("a", vec![item("i1", "Foo", 0), item("i2", "Bar", 2)]),
            100,
        )
        .unwrap();
    assert_eq!((ing.items, ing.added), (2, 2));
    let env: String = s
        .conn
        .query_row(
            "SELECT envelope FROM responses WHERE id = ?1",
            [ing.response_id],
            |r| r.get(0),
        )
        .unwrap();
    let env: Value = serde_json::from_str(&env).unwrap();
    assert!(env["stash"].get("items").is_none());
    assert_eq!(env["stash"]["_split"]["items"], 2);
    assert_eq!(env["stash"]["metadata"]["colour"], "7c5436");
    let tabs = s.tabs("pc", "Standard").unwrap();
    assert_eq!(tabs.len(), 1);
    assert_eq!((tabs[0].id.as_str(), tabs[0].item_count), ("a", 2));
    assert!(s.item("i1").unwrap().unwrap().removed_at.is_none());
}

#[test]
fn move_change_remove_become_events() {
    let mut s = Store::open_memory().unwrap();
    let p = json!({});
    s.record(
        &stash_ep("a"),
        &p,
        200,
        &stash("a", vec![item("i1", "Foo", 0), item("i2", "Bar", 2)]),
        100,
    )
    .unwrap();
    s.record(&stash_ep("b"), &p, 200, &stash("b", vec![]), 100)
        .unwrap();
    // i2 moves a→b (b fetched first: appears in b as moved), i1 changes.
    let ing = s
        .record(
            &stash_ep("b"),
            &p,
            200,
            &stash("b", vec![item("i2", "Bar", 5)]),
            200,
        )
        .unwrap();
    assert_eq!((ing.moved, ing.changed, ing.added), (1, 1, 0));
    let mut i1 = item("i1", "Foo", 0);
    i1["note"] = json!("~b/o 1 chaos");
    let ing = s
        .record(&stash_ep("a"), &p, 200, &stash("a", vec![i1]), 201)
        .unwrap();
    assert_eq!((ing.changed, ing.removed, ing.moved), (1, 0, 0));
    let ev = s.events_since(200, 100).unwrap();
    let kinds: Vec<_> = ev
        .iter()
        .map(|e| (e.item_id.as_str(), e.kind.as_str()))
        .collect();
    // newest first: the change at 201, then the two events of the
    // ingest at 200 in their reverse order
    assert_eq!(
        kinds,
        vec![("i1", "changed"), ("i2", "changed"), ("i2", "moved")]
    );
    assert_eq!(ev[2].from_location.as_deref(), Some("stash/pc/Standard/a"));
    // Now i1 disappears from a entirely.
    let ing = s
        .record(&stash_ep("a"), &p, 200, &stash("a", vec![]), 300)
        .unwrap();
    assert_eq!(ing.removed, 1);
    assert_eq!(s.item("i1").unwrap().unwrap().removed_at, Some(300));
    // And comes back: a move event from its removed state.
    let ing = s
        .record(
            &stash_ep("b"),
            &p,
            200,
            &stash("b", vec![item("i2", "Bar", 5), item("i1", "Foo", 0)]),
            400,
        )
        .unwrap();
    assert_eq!((ing.moved, ing.changed), (1, 1));
    assert_eq!(s.status().unwrap().items, 2);
}

#[test]
fn an_item_without_an_id_is_malformed_and_poisons_nothing() {
    let mut s = Store::open_memory().unwrap();
    let p = json!({});
    s.record(
        &stash_ep("a"),
        &p,
        200,
        &stash("a", vec![item("i1", "Foo", 0)]),
        100,
    )
    .unwrap();
    // A fetch whose items lack ids is refused whole: the held item is
    // neither removed nor half-replaced, and no response row lands.
    // What does land is the body itself, verbatim, in `refused` — the
    // error names the row and the item's position — so the finding
    // can be read without another fetch (PoE2 first contact,
    // 2026-09-02: four refusals and nothing to look at).
    let body = stash("a", vec![json!({ "name": "NoId", "typeLine": "?" })]);
    let err = s.record(&stash_ep("a"), &p, 200, &body, 200).unwrap_err();
    let malformed = err
        .downcast_ref::<MalformedBody>()
        .unwrap_or_else(|| panic!("{err:#}"));
    assert_eq!(
        malformed.at,
        Some(ItemAt {
            array: "items",
            index: 0,
            socketed: None
        })
    );
    assert_eq!(malformed.kept, Some(1));
    assert_eq!(
        err.to_string(),
        "malformed stash response: missing an `id` on an item at `items[0]` — body kept verbatim (`acq store refused 1`)"
    );
    assert!(s.item("i1").unwrap().unwrap().removed_at.is_none());
    let st = s.status().unwrap();
    assert_eq!((st.responses, st.refused_bodies), (1, 1));
    let kept = s.refused(1).unwrap().unwrap();
    assert_eq!(kept.body, Some(body));
    assert_eq!(
        (kept.endpoint.as_str(), kept.fetched_at, kept.status),
        ("stash", 200, 200)
    );
    assert_eq!(
        kept.reason,
        "malformed stash response: missing an `id` on an item at `items[0]`"
    );
    let listed = s.refused_list(10).unwrap();
    assert_eq!(listed.len(), 1);
    assert!(listed[0].body.is_none());
    assert!(s.refused(2).unwrap().is_none());
    // Same for a socketed gem without an id, at its own position.
    let mut bow = item("bow", "Bow", 0);
    bow["socketedItems"] = json!([{ "id": "g0" }, { "typeLine": "Nameless" }]);
    let err = s
        .record(
            &stash_ep("a"),
            &p,
            200,
            &stash("a", vec![item("i1", "Foo", 0), bow]),
            300,
        )
        .unwrap_err();
    let malformed = err
        .downcast_ref::<MalformedBody>()
        .unwrap_or_else(|| panic!("{err:#}"));
    assert_eq!(
        malformed.at,
        Some(ItemAt {
            array: "items",
            index: 1,
            socketed: Some(1)
        })
    );
    assert_eq!(malformed.kept, Some(2));
    assert!(s.item("i1").unwrap().unwrap().removed_at.is_none());
    assert!(s.item("bow").unwrap().is_none());
    // A refused character body names the array it was in — the
    // question a PoE2 `skills` refusal left open.
    let err = s
            .record(
                &Endpoint::Character {
                    realm: "poe2".into(),
                    name: "Hero".into(),
                },
                &json!({ "realm": "poe2", "name": "Hero" }),
                200,
                &json!({ "character": { "id": "c1", "name": "Hero", "equipment": [ item("eq1", "Helm", 0) ], "skills": [ item("s0", "Spark", 0), { "typeLine": "Default Attack" } ] } }),
                400,
            )
            .unwrap_err();
    let malformed = err
        .downcast_ref::<MalformedBody>()
        .unwrap_or_else(|| panic!("{err:#}"));
    assert_eq!(
        malformed.at,
        Some(ItemAt {
            array: "skills",
            index: 1,
            socketed: None
        })
    );
    assert_eq!(s.status().unwrap().refused_bodies, 3);
    assert!(s.characters(None, None).unwrap().is_empty());
    assert_eq!(s.refused(3).unwrap().unwrap().params["realm"], "poe2");
}

/// C145 (a present key of the wrong type is malformed; an absent key is
/// empty), issue #62: a body whose item-bearing key is present as
/// anything but an array is refused as `MalformedBody`, kept verbatim
/// in `refused` (C30, a refused body is evidence), and retires
/// nothing — where it used to read as an empty list that retired every
/// item, substash or league the key's location held.
#[test]
fn c145_a_present_key_of_the_wrong_type_is_malformed_and_retires_nothing() {
    let mut s = Store::open_memory().unwrap();
    let p = json!({});
    let hero = Endpoint::Character {
        realm: "pc".into(),
        name: "Hero".into(),
    };
    let mut gem = item("g1", "Gem", 0);
    gem["socketedItems"] = json!([item("g2", "Support", 0)]);
    let mut bow = item("eq1", "Bow", 0);
    bow["socketedItems"] = json!([gem]);
    let maps = json!({ "stash": { "id": "m", "name": "Maps", "type": "MapStash",
            "items": [], "children": [ { "id": "m-1", "name": "", "type": "MapStash" } ] } });
    s.record(
        &stash_ep("a"),
        &p,
        200,
        &stash("a", vec![item("i1", "Foo", 0)]),
        100,
    )
    .unwrap();
    s.record(&stash_ep("m"), &p, 200, &maps, 100).unwrap();
    s.record(
        &hero,
        &p,
        200,
        &json!({ "character": { "id": "c-hero", "name": "Hero", "equipment": [bow] } }),
        100,
    )
    .unwrap();
    s.record(&Endpoint::Leagues, &p, 200, &leagues(&["Standard"]), 100)
        .unwrap();
    let before = s.status().unwrap();
    let list_before = s.league_list().unwrap();

    let mut deep = item("eq1", "Bow", 0);
    deep["socketedItems"] = json!([{ "id": "g1", "typeLine": "Gem", "socketedItems": 7 }]);
    let cases: Vec<(Endpoint, Value, &str)> = vec![
        (
            stash_ep("a"),
            json!({ "stash": { "id": "a", "name": "tab a", "type": "PremiumStash", "items": {} } }),
            "malformed stash response: missing an `items` array",
        ),
        (
            stash_ep("m"),
            json!({ "stash": { "id": "m", "name": "Maps", "type": "MapStash", "items": [], "children": "m-1" } }),
            "malformed stash response: missing a `children` array",
        ),
        (
            hero.clone(),
            json!({ "character": { "id": "c-hero", "name": "Hero", "equipment": [deep] } }),
            "malformed character response: missing a `socketedItems` array at `equipment[0].socketedItems[0]`",
        ),
        (
            Endpoint::Leagues,
            json!({ "leagues": { "id": "Standard" } }),
            "malformed leagues response: missing a `leagues` array",
        ),
        (
            stash_ep("a"),
            json!({ "stash": { "id": "b", "name": "tab b", "type": "PremiumStash", "items": [] } }),
            "malformed stash response: missing a `stash.id` naming the tab the request named",
        ),
    ];
    for (n, (ep, body, words)) in cases.into_iter().enumerate() {
        let err = s.record(&ep, &p, 200, &body, 200).unwrap_err();
        let malformed = err
            .downcast_ref::<MalformedBody>()
            .unwrap_or_else(|| panic!("{err:#}"));
        let row = n as i64 + 1;
        assert_eq!(malformed.kept, Some(row), "{err}");
        assert!(err.to_string().starts_with(words), "{err}");
        assert_eq!(s.refused(row).unwrap().unwrap().body, Some(body));
    }
    let after = s.status().unwrap();
    assert_eq!(after.responses, before.responses, "no response row landed");
    assert_eq!(after.items, before.items, "no item was retired");
    assert_eq!(after.refused_bodies, 5);
    for live in ["i1", "eq1", "g1", "g2"] {
        assert!(
            s.item(live).unwrap().unwrap().removed_at.is_none(),
            "{live}"
        );
    }
    let substash: Option<i64> = s
        .conn
        .query_row("SELECT removed_at FROM tabs WHERE id = 'm-1'", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(substash, None, "the substash stands");
    assert_eq!(
        s.league_list().unwrap(),
        list_before,
        "the league list stands"
    );
}

/// C145: an absent key is the documented empty — a tab fetched with no
/// `children` lists no substash, and a body with no `items` holds none.
#[test]
fn c145_an_absent_key_is_the_documented_empty() {
    let mut s = Store::open_memory().unwrap();
    let body = json!({ "stash": { "id": "f", "name": "Folder", "type": "Folder" } });
    let ing = s.record(&stash_ep("f"), &json!({}), 200, &body, 1).unwrap();
    assert_eq!(ing.items, 0);
}

/// C145 (an absent key is the documented empty), the review of h1, its
/// sixth finding: a character fetch whose item arrays are all absent
/// reconciles its location as a fetch with empty arrays does, so the
/// items it no longer holds are retired. Before, no seam was pushed
/// and `i` stayed live after a fetch that showed the character empty,
/// where an explicit `equipment: []` retired it.
#[test]
fn c145_a_character_fetch_with_every_item_array_absent_retires_its_items() {
    let mut s = Store::open_memory().unwrap();
    let p = json!({});
    let ep = Endpoint::Character {
        realm: "pc".into(),
        name: "Hero".into(),
    };
    s.record(
        &ep,
        &p,
        200,
        &json!({ "character": { "id": "c", "equipment": [ item("i", "Helm", 0) ] } }),
        100,
    )
    .unwrap();
    assert!(s.item("i").unwrap().unwrap().removed_at.is_none());

    let ing = s
        .record(&ep, &p, 200, &json!({ "character": { "id": "c" } }), 200)
        .unwrap();
    assert_eq!(ing.removed, 1, "{ing:?}");
    assert_eq!(s.item("i").unwrap().unwrap().removed_at, Some(200));
}

/// C145: the id check recurses as the ingest does: a gem two levels
/// down without an id is refused before anything lands, at the
/// position of the item and the first-level gem it is under.
#[test]
fn c145_the_id_check_recurses_as_the_ingest_does() {
    let mut s = Store::open_memory().unwrap();
    let mut bow = item("bow", "Bow", 0);
    bow["socketedItems"] = json!([{ "id": "g1", "typeLine": "Gem",
            "socketedItems": [ { "typeLine": "Nameless" } ] }]);
    let err = s
        .record(&stash_ep("a"), &json!({}), 200, &stash("a", vec![bow]), 1)
        .unwrap_err();
    let malformed = err
        .downcast_ref::<MalformedBody>()
        .unwrap_or_else(|| panic!("{err:#}"));
    assert_eq!(malformed.missing, "an `id` on an item");
    assert_eq!(
        malformed.at,
        Some(ItemAt {
            array: "items",
            index: 0,
            socketed: Some(0)
        })
    );
    assert!(s.item("bow").unwrap().is_none(), "nothing landed");
}

/// C145: a refusal whose evidence cannot be kept — the `refused` insert
/// fails — is still `MalformedBody`, saying the body is lost, never
/// the raw storage error the daemon would not recognise as a refusal.
#[test]
fn c145_a_refusal_whose_evidence_cannot_be_kept_stays_malformed() {
    let mut s = Store::open_memory().unwrap();
    s.conn
            .execute_batch(
                "CREATE TRIGGER no_evidence BEFORE INSERT ON refused BEGIN SELECT RAISE(ABORT, 'disk full'); END",
            )
            .unwrap();
    let body = json!({ "stash": { "id": "a", "items": {} } });
    let err = s
        .record(&stash_ep("a"), &json!({}), 200, &body, 1)
        .unwrap_err();
    let malformed = err
        .downcast_ref::<MalformedBody>()
        .unwrap_or_else(|| panic!("a refusal, not the storage error: {err:#}"));
    assert_eq!((malformed.kept, malformed.not_kept), (None, true));
    assert_eq!(
        err.to_string(),
        "malformed stash response: missing an `items` array — the body could not be kept as evidence"
    );
}

#[test]
fn volatile_fields_are_not_changes() {
    let mut s = Store::open_memory().unwrap();
    let p = json!({});
    let mut a = item("i1", "Veiled", 0);
    a["veiledMods"] = json!(["Prefix04"]);
    s.record(&stash_ep("a"), &p, 200, &stash("a", vec![a.clone()]), 1)
        .unwrap();
    a["veiledMods"] = json!(["Prefix01"]);
    let ing = s
        .record(&stash_ep("a"), &p, 200, &stash("a", vec![a]), 2)
        .unwrap();
    assert_eq!(ing.changed, 0);
}

#[test]
fn socketed_gems_are_rows_of_their_own() {
    let mut s = Store::open_memory().unwrap();
    let mut bow = item("bow", "Bow", 0);
    bow["socketedItems"] = json!([{ "id": "gem1", "typeLine": "Determination", "baseType": "Determination", "socket": 0 }]);
    let ing = s
        .record(&stash_ep("a"), &json!({}), 200, &stash("a", vec![bow]), 1)
        .unwrap();
    assert_eq!(ing.items, 2);
    let gem = s.item("gem1").unwrap().unwrap();
    assert_eq!(gem.socketed_in.as_deref(), Some("bow"));
    assert_eq!(gem.location_id, "a");
    assert!(
        s.item("bow")
            .unwrap()
            .unwrap()
            .json
            .get("socketedItems")
            .is_none()
    );
    assert!(gem.removed_at.is_none());
    // Re-fetch with the gem unsocketed and gone: the gem is removed, the bow unchanged.
    let ing = s
        .record(
            &stash_ep("a"),
            &json!({}),
            200,
            &stash("a", vec![item("bow", "Bow", 0)]),
            2,
        )
        .unwrap();
    assert_eq!((ing.removed, ing.changed), (1, 0));
}

#[test]
fn stash_list_and_substashes_fill_the_tabs_table() {
    let mut s = Store::open_memory().unwrap();
    let list = json!({ "stashes": [
        { "id": "f1", "name": "Folder", "type": "Folder", "index": 0, "children": [ { "id": "c1", "name": "In folder", "type": "PremiumStash", "index": 1 } ] },
        { "id": "m1", "name": "Maps", "type": "MapStash", "index": 2 },
        { "id": "gone", "name": "Old", "type": "PremiumStash", "index": 3 },
    ]});
    let ep = Endpoint::Stashes {
        realm: "pc".into(),
        league: "Standard".into(),
    };
    s.record(&ep, &json!({}), 200, &list, 10).unwrap();
    let tabs = s.tabs("pc", "Standard").unwrap();
    assert_eq!(
        tabs.iter().map(|t| t.id.as_str()).collect::<Vec<_>>(),
        vec!["f1", "c1", "m1", "gone"]
    );
    assert_eq!(tabs[1].parent.as_deref(), Some("f1"));
    // Fetching the map tab lists substash stubs; fetching one substash lands items under it.
    let map = json!({ "stash": { "id": "m1", "name": "Maps", "type": "MapStash", "items": [], "children": [
            { "id": "s1", "name": "", "type": "MapStash", "parent": "m1", "metadata": { "items": 1, "map": { "name": "Tier 16" } } } ] } });
    s.record(&stash_ep("m1"), &json!({}), 200, &map, 11)
        .unwrap();
    let sub = Endpoint::Stash {
        realm: "pc".into(),
        league: "Standard".into(),
        id: "m1".into(),
        sub: Some("s1".into()),
    };
    let ing = s.record(&sub, &json!({}), 200, &json!({ "stash": { "id": "s1", "name": "", "type": "MapStash", "parent": "m1", "items": [item("map1", "", 0)] } }), 12).unwrap();
    assert_eq!(ing.added, 1);
    let tabs = s.tabs("pc", "Standard").unwrap();
    let s1 = tabs.iter().find(|t| t.id == "s1").unwrap();
    assert_eq!((s1.parent.as_deref(), s1.item_count), (Some("m1"), 1));
    // The next list drops "gone": removed, but the substash (never listed) survives.
    let list2 = json!({ "stashes": [ { "id": "f1", "name": "Folder", "type": "Folder", "index": 0, "children": [ { "id": "c1", "name": "In folder", "type": "PremiumStash", "index": 1 } ] }, { "id": "m1", "name": "Maps", "type": "MapStash", "index": 2 } ] });
    s.record(&ep, &json!({}), 200, &list2, 20).unwrap();
    let ids: Vec<_> = s
        .tabs("pc", "Standard")
        .unwrap()
        .into_iter()
        .map(|t| t.id)
        .collect();
    assert_eq!(ids, vec!["f1", "c1", "m1", "s1"]);
}

#[test]
fn character_arrays_share_one_location() {
    let mut s = Store::open_memory().unwrap();
    let body = json!({ "character": { "id": "c-hero", "name": "Hero", "league": "Standard", "class": "Witch", "level": 90,
            "inventory": [ item("inv1", "Bag", 0) ], "equipment": [ item("eq1", "Helm", 0) ], "jewels": [] } });
    let ep = Endpoint::Character {
        realm: "pc".into(),
        name: "Hero".into(),
    };
    let ing = s
        .record(&ep, &json!({"name":"Hero"}), 200, &body, 1)
        .unwrap();
    assert_eq!(ing.added, 2);
    let inv = s.item("inv1").unwrap().unwrap();
    assert_eq!(
        (inv.location_kind.as_str(), inv.location_id.as_str()),
        ("character", "c-hero")
    );
    assert_eq!(inv.container.as_deref(), Some("inventory"));
    let ing = s.record(&ep, &json!({"name":"Hero"}), 200, &json!({ "character": { "id": "c-hero", "name": "Hero", "league": "Standard", "inventory": [ item("inv1", "Bag", 0) ], "equipment": [] } }), 2).unwrap();
    assert_eq!(ing.removed, 1);
    let json: String = s
        .conn
        .query_row("SELECT json FROM characters WHERE id = 'c-hero'", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert!(json.contains("\"_split\"") && !json.contains("\"inv1\""));
}

#[test]
fn leagues_profile_characters_list() {
    let mut s = Store::open_memory().unwrap();
    s.record(
        &Endpoint::Leagues,
        &json!({}),
        200,
        &json!({ "leagues": [ { "id": "Standard" }, { "id": "Hardcore" } ] }),
        1,
    )
    .unwrap();
    s.record(
        &Endpoint::Profile,
        &json!({}),
        200,
        &json!({ "uuid": "u-1", "name": "tom" }),
        1,
    )
    .unwrap();
    s.record(&Endpoint::Characters { realm: "pc".into() }, &json!({}), 200, &json!({ "characters": [ { "id": "c-a", "name": "A", "class": "Witch", "level": 3, "league": "Standard" } ] }), 1).unwrap();
    let st = s.status().unwrap();
    assert_eq!((st.leagues, st.characters, st.responses), (2, 1, 3));
    s.record(
        &Endpoint::Characters { realm: "pc".into() },
        &json!({}),
        200,
        &json!({ "characters": [] }),
        2,
    )
    .unwrap();
    assert_eq!(s.status().unwrap().characters, 0);
}

#[test]
fn characters_query_lists_live_rows_with_item_counts() {
    let mut s = Store::open_memory().unwrap();
    s.record(
        &Endpoint::Characters { realm: "pc".into() },
        &json!({}),
        200,
        &json!({ "characters": [
            { "id": "c-hero", "name": "Hero", "class": "Witch", "level": 90, "league": "Standard" },
            { "id": "c-mule", "name": "Mule", "class": "Scion", "level": 3, "league": "Hardcore" },
        ] }),
        1,
    )
    .unwrap();
    // Fetch fills fetched_at and lifts items into the character location.
    s.record(
            &Endpoint::Character { realm: "pc".into(),
                name: "Hero".into(),
            },
            &json!({"name":"Hero"}),
            200,
            &json!({ "character": { "id": "c-hero", "name": "Hero", "league": "Standard", "class": "Witch", "level": 90,
                "equipment": [ item("eq1", "Bow", 0) ], "inventory": [ item("inv1", "Bag", 1) ] } }),
            2,
        )
        .unwrap();
    let all = s.characters(None, None).unwrap();
    assert_eq!(
        all.iter()
            .map(|c| (c.name.as_str(), c.item_count, c.fetched_at.is_some()))
            .collect::<Vec<_>>(),
        vec![("Mule", 0, false), ("Hero", 2, true)]
    );
    let std_only = s.characters(None, Some("Standard")).unwrap();
    assert_eq!(std_only.len(), 1);
    assert_eq!(std_only[0].name, "Hero");
    // A character no longer listed disappears from the query.
    s.record(
        &Endpoint::Characters { realm: "pc".into() },
        &json!({}),
        200,
        &json!({ "characters": [
            { "id": "c-hero", "name": "Hero", "class": "Witch", "level": 90, "league": "Standard" },
        ] }),
        3,
    )
    .unwrap();
    assert_eq!(s.characters(None, None).unwrap().len(), 1);
}

#[test]
fn rebuild_reextracts_columns() {
    let mut s = Store::open_memory().unwrap();
    s.record(
        &stash_ep("a"),
        &json!({}),
        200,
        &stash("a", vec![item("i1", "Foo", 0)]),
        1,
    )
    .unwrap();
    s.conn
        .execute("UPDATE items SET name = 'wrong'", [])
        .unwrap();
    assert_eq!(s.rebuild().unwrap(), 1);
    assert_eq!(s.item("i1").unwrap().unwrap().name, "Foo");
}

/// C35 — no fact-side event ever deletes intent; an orphaned annotation is kept and surfaceable.
#[test]
fn intent_survives_fact_removal_and_surfaces_as_orphaned() {
    let mut s = Store::open_memory().unwrap();
    let mut a = Annotations::open_memory().unwrap();
    s.record(
        &stash_ep("a"),
        &json!({}),
        200,
        &stash("a", vec![item("i1", "Foo", 0)]),
        100,
    )
    .unwrap();
    let via = crate::annotations::test_kinds::via_test();
    a.put::<crate::annotations::test_kinds::Buyout>(
        "item",
        "i1",
        &json!({"version": 1, "price": "1 divine"}),
        None,
        &via,
    )
    .unwrap();
    assert!(s.orphaned_item_annotations(&a).unwrap().rows.is_empty());
    // The item disappears from its tab: the fact side records a removal
    // and touches no intent — the annotation stays, now orphaned.
    s.record(&stash_ep("a"), &json!({}), 200, &stash("a", vec![]), 200)
        .unwrap();
    let orphaned = s.orphaned_item_annotations(&a).unwrap().rows;
    assert_eq!(orphaned.len(), 1);
    assert_eq!(orphaned[0].key, "i1");
    assert_eq!(a.get("item", "i1", "buyout").unwrap().unwrap().revision, 1);
    // The item comes back: the same annotation is attached again.
    s.record(
        &stash_ep("a"),
        &json!({}),
        200,
        &stash("a", vec![item("i1", "Foo", 0)]),
        300,
    )
    .unwrap();
    assert!(s.orphaned_item_annotations(&a).unwrap().rows.is_empty());
    // An annotation on an item this store never saw is orphaned too.
    a.put::<crate::annotations::test_kinds::Note>(
        "item",
        "ghost",
        &json!({"version": 1, "text": "?"}),
        None,
        &via,
    )
    .unwrap();
    assert_eq!(s.orphaned_item_annotations(&a).unwrap().rows.len(), 1);
    // C151 (the second review of #63, its fifth finding): an orphan
    // this build cannot read is named beside the readable ones, never
    // dropped; one on a live item is attached and named nowhere here.
    a.put::<crate::annotations::test_kinds::Note>(
        "item",
        "i1",
        &json!({"version": 1, "text": "kept"}),
        None,
        &via,
    )
    .unwrap();
    for key in ["ghost", "i1"] {
        a.conn
            .execute(
                "UPDATE annotations SET value = '{nope' WHERE key = ?1 AND kind = 'note'",
                [key],
            )
            .unwrap();
    }
    let orphaned = s.orphaned_item_annotations(&a).unwrap();
    assert!(orphaned.rows.is_empty(), "{:?}", orphaned.rows);
    let keys: Vec<&str> = orphaned.unreadable.iter().map(|r| r.key.as_str()).collect();
    assert_eq!(
        keys,
        ["ghost"],
        "the unreadable orphan, and not the attached one"
    );
    assert_eq!(orphaned.unreadable[0].kind, "note");
}

#[test]
fn files_from_before_the_listing_columns_are_migrated_in_place() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let path = dir.join("old.db");
    {
        // A tabs table as the schema had it before listed_json /
        // listed_response, with one live row.
        let conn = Connection::open(&path).unwrap();
        conn.execute_batch(
                "CREATE TABLE tabs (
                    league TEXT NOT NULL, id TEXT NOT NULL, parent TEXT, name TEXT, type TEXT,
                    idx INTEGER, json TEXT NOT NULL, listed_at INTEGER, fetched_at INTEGER,
                    removed_at INTEGER, PRIMARY KEY (league, id));
                 INSERT INTO tabs (league, id, name, type, idx, json, listed_at)
                 VALUES ('Standard', 'old1', 'Old', 'PremiumStash', 0, '{}', 5);
                 CREATE TABLE items (
                    id TEXT PRIMARY KEY, league TEXT, location_kind TEXT NOT NULL, location_id TEXT NOT NULL,
                    socketed_in TEXT, name TEXT, type_line TEXT, base_type TEXT, rarity TEXT, stack_size INTEGER,
                    x INTEGER, y INTEGER, w INTEGER, h INTEGER, json TEXT NOT NULL,
                    first_seen INTEGER NOT NULL, last_seen INTEGER NOT NULL, removed_at INTEGER);
                 INSERT INTO items (id, league, location_kind, location_id, name, type_line, base_type, json, first_seen, last_seen)
                 VALUES ('old-item', 'Standard', 'stash', 'old1', 'Foo', 'Imperial Bow', 'Imperial Bow', '{\"id\":\"old-item\"}', 5, 5);
                 CREATE TABLE characters (
                    name TEXT PRIMARY KEY, league TEXT, class TEXT, level INTEGER, json TEXT NOT NULL,
                    listed_at INTEGER, fetched_at INTEGER, removed_at INTEGER);
                 INSERT INTO characters (name, league, json, listed_at, fetched_at)
                 VALUES ('Hero', 'Standard', '{\"id\":\"c-hero\",\"name\":\"Hero\",\"_split\":{}}', 5, 6);
                 INSERT INTO characters (name, league, json, listed_at) VALUES ('Ghost', 'Standard', '{\"name\":\"Ghost\"}', 5);
                 INSERT INTO items (id, league, location_kind, location_id, name, type_line, base_type, json, first_seen, last_seen)
                 VALUES ('hero-item', 'Standard', 'character', 'Hero', 'Bar', 'Bow', 'Bow', '{\"id\":\"hero-item\"}', 6, 6);
                 INSERT INTO items (id, league, location_kind, location_id, name, type_line, base_type, json, first_seen, last_seen)
                 VALUES ('ghost-item', 'Standard', 'character', 'Ghost', 'Baz', 'Bow', 'Bow', '{\"id\":\"ghost-item\"}', 6, 6);
                 CREATE TABLE responses (id INTEGER PRIMARY KEY, endpoint TEXT NOT NULL, params TEXT NOT NULL,
                    fetched_at INTEGER NOT NULL, status INTEGER NOT NULL, envelope TEXT NOT NULL, item_count INTEGER NOT NULL);
                 INSERT INTO responses (id, endpoint, params, fetched_at, status, envelope, item_count)
                 VALUES (7, 'characters', '{}', 5, 200, '{\"characters\":[{\"id\":\"c-hero\",\"name\":\"Hero\",\"league\":\"Standard\"}]}', 0);",
            )
            .unwrap();
    }
    let mut s = Store::open(&path).unwrap();
    let tabs = s.tabs("pc", "Standard").unwrap();
    assert_eq!(
        (tabs[0].id.as_str(), tabs[0].realm.as_str()),
        ("old1", "pc")
    );
    // Pre-realm items are pc too — not null: the tab's count and the
    // item's own row both say so (review finding 2026-09-02).
    assert_eq!(tabs[0].item_count, 1);
    assert_eq!(s.item("old-item").unwrap().unwrap().realm, "pc");
    assert_eq!(
        s.item("old-item").unwrap().unwrap().container.as_deref(),
        Some("items")
    );
    // v4: characters rekeyed by the id in their json — the row without
    // one is dropped and its items retired; the other's items move to
    // its id (container unknown: not in the json, so left NULL).
    let chars = s.characters(None, None).unwrap();
    assert_eq!(chars.len(), 1);
    let hero = &chars[0];
    assert_eq!(
        (
            hero.id.as_str(),
            hero.name.as_str(),
            hero.realm.as_str(),
            hero.fetched_at,
            hero.item_count
        ),
        ("c-hero", "Hero", "pc", Some(6), 1)
    );
    let hero_item = s.item("hero-item").unwrap().unwrap();
    assert_eq!(
        (
            hero_item.location_id.as_str(),
            hero_item.container.as_deref()
        ),
        ("c-hero", None)
    );
    assert!(s.item("ghost-item").unwrap().unwrap().removed_at.is_some());
    // Membership is re-stamped from the latest listing on record, so
    // a basis a planner cites has its rows stamped to it.
    let (listed_response, listed_json): (Option<i64>, Option<String>) = s
        .conn
        .query_row(
            "SELECT listed_response, listed_json FROM characters WHERE id = 'c-hero'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(listed_response, Some(7));
    assert!(listed_json.unwrap().contains("\"Hero\""));
    // The migrated file is stamped with the current schema version.
    let v: i64 = s
        .conn
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap();
    assert_eq!(v, FACT_SCHEMA_VERSION);
    // The next listing stamps membership per response id and retires
    // the pre-migration row it dropped.
    s.record(
            &Endpoint::Stashes { realm: "pc".into(),
                league: "Standard".into(),
            },
            &json!({}),
            200,
            &json!({ "stashes": [ { "id": "new1", "name": "New", "type": "PremiumStash", "index": 0 } ] }),
            10,
        )
        .unwrap();
    let ids: Vec<String> = s
        .tabs("pc", "Standard")
        .unwrap()
        .into_iter()
        .map(|t| t.id)
        .collect();
    assert_eq!(ids, vec!["new1"]);
    // A file stamped newer than this build is refused, not guessed at.
    s.conn.pragma_update(None, "user_version", 99).unwrap();
    drop(s);
    let err = Store::open(&path).err().unwrap();
    assert_eq!(
        err.downcast_ref::<SchemaTooNew>(),
        Some(&SchemaTooNew {
            found: 99,
            supported: FACT_SCHEMA_VERSION
        })
    );
}

/// Facts v8 (B4): a v7 file loses `items_names` and gains
/// `items_removed`, every row where it was; a second open changes
/// nothing, and a fresh file has the same two.
#[test]
fn v7_to_v8_swaps_the_index_and_moves_no_row() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let path = dir.join("facts.db");
    let indexes = |s: &Store| -> Vec<String> {
        let mut stmt = s
                .conn
                .prepare("SELECT name FROM sqlite_master WHERE type = 'index' AND tbl_name = 'items' AND name NOT LIKE 'sqlite_%' ORDER BY name")
                .unwrap();
        stmt.query_map([], |r| r.get(0))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap()
    };
    let rows = |s: &Store| -> Vec<(String, Option<i64>)> {
        let mut stmt = s
            .conn
            .prepare("SELECT id, removed_at FROM items ORDER BY id")
            .unwrap();
        stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap()
    };
    let mut s = Store::open(&path).unwrap();
    let fresh = indexes(&s);
    assert_eq!(
        fresh,
        [
            "items_live",
            "items_live_at",
            "items_location",
            "items_removed",
            "items_socketed_in"
        ]
    );
    let both = stash("a", vec![item("i1", "Foo", 0), item("i2", "Bar", 2)]);
    s.record(&stash_ep("a"), &json!({}), 200, &both, 100)
        .unwrap();
    let one = stash("a", vec![item("i1", "Foo", 0)]);
    s.record(&stash_ep("a"), &json!({}), 200, &one, 200)
        .unwrap();
    let before = rows(&s);
    assert_eq!(
        before,
        [("i1".to_string(), None), ("i2".to_string(), Some(200))]
    );
    // the file as a v7 build left it
    s.conn
        .execute_batch(
            "DROP INDEX items_removed;
                 CREATE INDEX items_names ON items (name, type_line, base_type);
                 PRAGMA user_version = 7;",
        )
        .unwrap();
    drop(s);
    for _ in 0..2 {
        let s = Store::open(&path).unwrap();
        assert_eq!(indexes(&s), fresh);
        assert_eq!(rows(&s), before);
        assert_eq!(s.revision().unwrap().facts_version, FACT_SCHEMA_VERSION);
    }
}

/// Facts v9 (0a): a v8 file gains `items_live` and `items_live_at`,
/// the live items' partial indexes, and no row moves; a second open
/// changes nothing.
#[test]
fn v8_to_v9_adds_the_live_indexes_and_moves_no_row() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let path = dir.join("facts.db");
    let live_index = |s: &Store| -> Vec<String> {
        let mut stmt = s
                .conn
                .prepare("SELECT sql FROM sqlite_master WHERE type = 'index' AND name LIKE 'items_live%' ORDER BY name")
                .unwrap();
        stmt.query_map([], |r| r.get(0))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap()
    };
    let rows = |s: &Store| -> Vec<(String, Option<i64>)> {
        let mut stmt = s
            .conn
            .prepare("SELECT id, removed_at FROM items ORDER BY id")
            .unwrap();
        stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap()
    };
    let mut s = Store::open(&path).unwrap();
    let fresh = live_index(&s);
    assert_eq!(fresh.len(), 2, "{fresh:?}");
    assert!(
        fresh
            .iter()
            .all(|sql| sql.contains("WHERE removed_at IS NULL")),
        "{fresh:?}"
    );
    let both = stash("a", vec![item("i1", "Foo", 0), item("i2", "Bar", 2)]);
    s.record(&stash_ep("a"), &json!({}), 200, &both, 100)
        .unwrap();
    let one = stash("a", vec![item("i1", "Foo", 0)]);
    s.record(&stash_ep("a"), &json!({}), 200, &one, 200)
        .unwrap();
    let before = rows(&s);
    // the file as a v8 build left it
    s.conn
        .execute_batch("DROP INDEX items_live; DROP INDEX items_live_at; PRAGMA user_version = 8;")
        .unwrap();
    drop(s);
    for _ in 0..2 {
        let s = Store::open(&path).unwrap();
        assert_eq!(live_index(&s), fresh);
        assert_eq!(rows(&s), before);
        assert_eq!(s.revision().unwrap().facts_version, 9);
    }
}

/// 0a: a reader's open of a current file takes no write lock, so it
/// opens while another connection holds one — an ingest in progress
/// — where it used to wait out the busy timeout and fail; a file that
/// must migrate still waits for the lock, and acts on the version it
/// reads under it, never the one it read before.
#[test]
fn a_current_file_opens_while_a_writer_holds_the_lock() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let path = dir.join("facts.db");
    drop(Store::open(&path).unwrap());
    let writer = Connection::open(&path).unwrap();
    writer.execute_batch("BEGIN IMMEDIATE;").unwrap();
    // No wait, observed rather than timed (issue #111): a busy handler
    // that counts and refuses, so an open that wanted the lock fails
    // at once with the count above zero.
    static REFUSED: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    fn refuse(_attempts: i32) -> bool {
        REFUSED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        false
    }
    let conn = Connection::open(&path).unwrap();
    conn.busy_handler(Some(refuse)).unwrap();
    let s = Store::init(conn, path.clone());
    assert_eq!(
        REFUSED.load(std::sync::atomic::Ordering::SeqCst),
        0,
        "the open waited on the writer: {:?}",
        s.as_ref().err()
    );
    let s = s.unwrap();
    assert_eq!(s.revision().unwrap().facts_version, FACT_SCHEMA_VERSION);
    drop(s);
    // A file to migrate waits for the writer, then sees its work: two
    // openers racing one v8 file, the writer standing in for the
    // first. The opener reads v8 before the lock; the writer stamps
    // the file newer than this build before it lets go, so an opener
    // that acted on its first read would migrate it, and one that
    // reads again under the lock refuses it.
    writer
        .execute_batch(
            "DROP INDEX items_live; DROP INDEX items_live_at; PRAGMA user_version = 8; COMMIT;",
        )
        .unwrap();
    writer
        .execute_batch("BEGIN IMMEDIATE; PRAGMA user_version = 99;")
        .unwrap();
    // The wait is forced, never timed (issue #111): the opener's busy
    // handler is the signal that it stands behind the writer's lock,
    // and the writer commits only after that signal, as #63's pin
    // below does. An opener that took no lock finishes without the
    // signal, and the assertion after the join says so.
    static WAITS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    fn noticed(_attempts: i32) -> bool {
        WAITS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        std::thread::yield_now();
        true
    }
    let conn = Connection::open(&path).unwrap();
    conn.busy_handler(Some(noticed)).unwrap();
    let opened = std::thread::scope(|scope| {
        let opener = scope.spawn(|| Store::init(conn, path.clone()).map(drop));
        while WAITS.load(std::sync::atomic::Ordering::SeqCst) == 0 && !opener.is_finished() {
            std::thread::yield_now();
        }
        writer.execute_batch("COMMIT;").unwrap();
        opener.join().unwrap()
    });
    assert!(
        WAITS.load(std::sync::atomic::Ordering::SeqCst) > 0,
        "a migrating open took no lock"
    );
    assert_eq!(
        opened.unwrap_err().downcast_ref::<SchemaTooNew>(),
        Some(&SchemaTooNew {
            found: 99,
            supported: FACT_SCHEMA_VERSION
        })
    );
}

/// Issue #63 (`roadmap/briefs/63-store-remainder.md`, Do 2): `record`
/// and `rebuild` take the write lock before their first read (BEGIN
/// IMMEDIATE), so a second writer on the file — `acq store rebuild`
/// or `store import` beside an ingesting daemon — makes the ingest
/// wait, never fail its upgrade from read to write. Both halves of
/// the SQLite behaviour are pinned. The fault: a deferred transaction
/// that has read, a write committed beside it, then its own write,
/// which SQLite refuses at once (`SQLITE_BUSY_SNAPSHOT`, which no busy
/// handler retries). The wait: a connection holding BEGIN IMMEDIATE
/// while `record` and then `rebuild` begin; each waits and lands once
/// the holder commits. The interleaving is forced, never timed: the
/// store connection's busy handler is the signal that the wait is
/// real, and the holder's commit follows that signal. No sleep. With
/// `record` flipped back to a deferred transaction the test fails at
/// "record began behind a held write lock": SQLite answers a reader
/// promoting itself to a writer beside a held write lock with busy at
/// once, never invoking the busy handler (its deadlock rule), so the
/// ingest failed where it now waits.
#[test]
fn a_writer_beside_the_ingest_makes_it_wait_never_fail_its_upgrade() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("facts.db");
    {
        let mut s = Store::open(&path).unwrap();
        s.record(
            &stash_ep("a"),
            &json!({}),
            200,
            &stash("a", vec![item("i1", "Foo", 0)]),
            100,
        )
        .unwrap();
    }

    // The fault SQLite has: a deferred writer behind a committed one.
    let deferred = Connection::open(&path).unwrap();
    deferred
        .busy_timeout(std::time::Duration::from_secs(5))
        .unwrap();
    deferred.execute_batch("BEGIN").unwrap();
    let read: i64 = deferred
        .query_row("SELECT COUNT(*) FROM items", [], |r| r.get(0))
        .unwrap();
    assert_eq!(read, 1);
    let beside = Connection::open(&path).unwrap();
    beside
        .execute_batch("BEGIN IMMEDIATE; UPDATE items SET name = 'Bar'; COMMIT;")
        .unwrap();
    let refused = deferred
        .execute("UPDATE items SET name = 'Baz'", [])
        .unwrap_err();
    assert!(
        matches!(
            &refused,
            rusqlite::Error::SqliteFailure(f, _) if f.code == rusqlite::ErrorCode::DatabaseBusy
        ),
        "a deferred write behind a committed one is refused, not retried: {refused:?}"
    );
    deferred.execute_batch("ROLLBACK").unwrap();
    drop(deferred);

    // The wait BEGIN IMMEDIATE buys. The busy handler is a plain fn,
    // so its signal is a static count of its invocations.
    static WAITS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    fn noticed(_attempts: i32) -> bool {
        WAITS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        std::thread::yield_now();
        true
    }
    let mut s = Store::open(&path).unwrap();
    s.conn.busy_handler(Some(noticed)).unwrap();
    let holder = Connection::open(&path).unwrap();
    for round in ["record", "rebuild"] {
        holder
            .execute_batch("BEGIN IMMEDIATE; UPDATE items SET name = 'Held';")
            .unwrap();
        let before = WAITS.load(std::sync::atomic::Ordering::SeqCst);
        let landed = std::thread::scope(|scope| {
            let waiter = scope.spawn(|| match round {
                "record" => s
                    .record(
                        &stash_ep("a"),
                        &json!({}),
                        200,
                        &stash("a", vec![item("i1", "Foo", 0), item("i2", "Bar", 2)]),
                        200,
                    )
                    .map(|ingest| ingest.items),
                _ => s.rebuild(),
            });
            // The signal: the waiter's busy handler has run, so it is
            // behind the holder's lock. A waiter that finished without
            // it did not wait, and the assertion below says so.
            while WAITS.load(std::sync::atomic::Ordering::SeqCst) == before && !waiter.is_finished()
            {
                std::thread::yield_now();
            }
            holder.execute_batch("COMMIT").unwrap();
            waiter.join().unwrap()
        });
        assert!(
            WAITS.load(std::sync::atomic::Ordering::SeqCst) > before,
            "{round} began behind a held write lock"
        );
        assert_eq!(
            landed.unwrap(),
            2,
            "{round} landed after the holder released: an ingest of two, a rebuild of two"
        );
    }
    let names: Vec<String> = {
        let mut stmt = s
            .conn
            .prepare("SELECT name FROM items ORDER BY id")
            .unwrap();
        stmt.query_map([], |r| r.get(0))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap()
    };
    assert_eq!(
        names,
        ["Foo", "Bar"],
        "the rebuild re-extracted what the ingest wrote"
    );
}

/// Issue #63 (`roadmap/briefs/63-store-remainder.md`, Do 4): the
/// version is read before anything is written. A facts file in
/// rollback-journal mode stamped newer than this build is refused
/// exactly as found: its journal mode still `delete` after, no `-wal`
/// beside it — where `ensure_wal` used to run before the gate and
/// would have rewritten the header of a file this build may not touch.
#[test]
fn a_newer_file_is_refused_before_its_journal_mode_is_touched() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("facts.db");
    {
        let raw = Connection::open(&path).unwrap();
        raw.execute_batch("CREATE TABLE future (x); PRAGMA user_version = 99;")
            .unwrap();
        let mode: String = raw
            .query_row("PRAGMA journal_mode", [], |r| r.get(0))
            .unwrap();
        assert_eq!(mode, "delete", "staged in rollback-journal mode");
    }
    let refused = Store::open(&path).map(drop).unwrap_err();
    assert_eq!(
        refused.downcast_ref::<SchemaTooNew>(),
        Some(&SchemaTooNew {
            found: 99,
            supported: FACT_SCHEMA_VERSION
        })
    );
    let raw = Connection::open(&path).unwrap();
    let mode: String = raw
        .query_row("PRAGMA journal_mode", [], |r| r.get(0))
        .unwrap();
    assert_eq!(mode, "delete", "the refused file keeps its journal mode");
    assert!(
        !path.with_extension("db-wal").exists(),
        "no WAL side file was written beside a refused file"
    );
}

/// The review of #63's build, its first finding: a publish to a bare
/// relative filename has a parent of `Some("")`, and the directory
/// sync that follows it opened "" and failed after the file was
/// published. The helper takes an empty parent as the current
/// directory; `None` and a real directory stand as before. The
/// export's own call is one line on `dest.parent()`, so the helper
/// pin holds it; an end-to-end export to a bare filename would set
/// the process's working directory, process state that needs its own
/// test binary (C122) for a property this pin already holds.
#[test]
fn a_publish_to_a_bare_filename_syncs_the_current_directory() {
    let bare = Path::new("backup.db").parent();
    assert_eq!(
        bare,
        Some(Path::new("")),
        "a bare filename's parent is the empty path"
    );
    sync_dir(bare).unwrap();
    sync_dir(None).unwrap();
    let temp = tempfile::tempdir().unwrap();
    sync_dir(temp.path().join("accounts.json").parent()).unwrap();
}

#[test]
fn endpoint_from_job_kinds() {
    assert_eq!(
        Endpoint::from_job("stash", &json!({"league":"Standard","id":"a","sub":"b"})),
        Some(Endpoint::Stash {
            realm: "pc".into(),
            league: "Standard".into(),
            id: "a".into(),
            sub: Some("b".into())
        })
    );
    assert_eq!(Endpoint::from_job("stash", &json!({})), None);
    assert_eq!(Endpoint::from_job("probe", &json!({})), None);
    assert_eq!(
        Endpoint::from_job("stashes", &json!({})),
        Some(Endpoint::Stashes {
            realm: "pc".into(),
            league: "Standard".into()
        })
    );
}

/// Ten thousand items: the unchanged re-ingest reports no change. Both
/// ingests' times are printed, never asserted (C137: no test asserts a
/// wall-clock bound a slow machine can miss), and nothing here
/// observes how many transactions either took.
#[test]
fn a_large_tab_reingested_unchanged_reports_nothing() {
    let mut s = Store::open_memory().unwrap();
    let items: Vec<Value> = (0..10_000)
        .map(|i| item(&format!("i{i}"), "Chaos Orb", i % 24))
        .collect();
    let t = std::time::Instant::now();
    let ing = s
        .record(
            &stash_ep("quad"),
            &json!({}),
            200,
            &stash("quad", items.clone()),
            1,
        )
        .unwrap();
    let first = t.elapsed();
    assert_eq!(ing.added, 10_000);
    let t = std::time::Instant::now();
    let ing = s
        .record(&stash_ep("quad"), &json!({}), 200, &stash("quad", items), 2)
        .unwrap();
    let second = t.elapsed();
    assert_eq!((ing.added, ing.changed, ing.removed), (0, 0, 0));
    eprintln!("10k items: first ingest {first:?}, unchanged re-ingest {second:?}");
}

/// Realm is the coordinate above league (C58): the
/// same league and tab id under two realms are two rows, a listing
/// retires only its own realm's tabs, items carry the request's
/// realm, and a character listing never retires another realm's.
#[test]
fn realms_keep_the_same_league_and_id_apart() {
    let mut s = Store::open_memory().unwrap();
    let list = |realm: &str| Endpoint::Stashes {
        realm: realm.into(),
        league: "Standard".into(),
    };
    let body =
        json!({ "stashes": [ { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 } ] });
    s.record(&list("pc"), &json!({}), 200, &body, 10).unwrap();
    s.record(
        &list("xbox"),
        &json!({ "realm": "xbox", "league": "Standard" }),
        200,
        &body,
        11,
    )
    .unwrap();
    assert_eq!(s.tabs("pc", "Standard").unwrap().len(), 1);
    assert_eq!(s.tabs("xbox", "Standard").unwrap()[0].realm, "xbox");
    // An empty xbox listing retires the xbox row only.
    s.record(
        &list("xbox"),
        &json!({ "realm": "xbox" }),
        200,
        &json!({ "stashes": [] }),
        12,
    )
    .unwrap();
    assert!(s.tabs("xbox", "Standard").unwrap().is_empty());
    assert_eq!(s.tabs("pc", "Standard").unwrap()[0].id, "t1");
    // Items are stamped with the request's realm; search filters on it.
    let ing = s
            .record(
                &Endpoint::Stash {
                    realm: "sony".into(),
                    league: "Standard".into(),
                    id: "s9".into(),
                    sub: None,
                },
                &json!({ "realm": "sony", "league": "Standard", "id": "s9" }),
                200,
                &json!({ "stash": { "id": "s9", "name": "Sony", "type": "PremiumStash", "items": [ item("i1", "Foo", 0) ] } }),
                13,
            )
            .unwrap();
    assert_eq!(ing.added, 1);
    assert_eq!(s.item("i1").unwrap().unwrap().realm, "sony");
    // A poe2 character listing leaves pc's characters alone.
    s.record(
        &Endpoint::Characters { realm: "pc".into() },
        &json!({}),
        200,
        &json!({ "characters": [ { "id": "c-a", "name": "A", "league": "Standard" } ] }),
        20,
    )
    .unwrap();
    s.record(
        &Endpoint::Characters {
            realm: "poe2".into(),
        },
        &json!({ "realm": "poe2" }),
        200,
        &json!({ "characters": [ { "id": "c-b", "name": "B", "league": "Standard" } ] }),
        21,
    )
    .unwrap();
    let names: Vec<(String, String)> = s
        .characters(None, None)
        .unwrap()
        .into_iter()
        .map(|c| (c.realm, c.name))
        .collect();
    assert_eq!(
        names,
        vec![("pc".into(), "A".into()), ("poe2".into(), "B".into())]
    );
    assert_eq!(s.characters(Some("poe2"), None).unwrap().len(), 1);
}

/// Identity is the id; the name is the address (C55): a rename keeps
/// the row and its items (no false moves), a deleted-and-recreated
/// name is a new row that has never been fetched with the old one
/// retired, the listing owns `league`, and a fetched body without an
/// id has no row to land on.
#[test]
fn the_character_key_is_the_id_and_the_name_is_the_address() {
    let mut s = Store::open_memory().unwrap();
    let list = Endpoint::Characters { realm: "pc".into() };
    let fetch = |name: &str| Endpoint::Character {
        realm: "pc".into(),
        name: name.into(),
    };
    s.record(&list, &json!({}), 200, &json!({ "characters": [ { "id": "c1", "name": "Hero", "league": "Standard", "level": 90 } ] }), 10).unwrap();
    let ing = s
            .record(
                &fetch("Hero"),
                &json!({ "name": "Hero" }),
                200,
                &json!({ "character": { "id": "c1", "name": "Hero", "league": "Standard", "equipment": [ item("eq1", "Helm", 0) ] } }),
                11,
            )
            .unwrap();
    assert_eq!(ing.added, 1);
    // Renamed: the listing names the same id under a new name.
    s.record(&list, &json!({}), 200, &json!({ "characters": [ { "id": "c1", "name": "Champion", "league": "Standard", "level": 91 } ] }), 20).unwrap();
    let rows = s.characters(None, None).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(
        (
            rows[0].id.as_str(),
            rows[0].name.as_str(),
            rows[0].item_count,
            rows[0].fetched_at
        ),
        ("c1", "Champion", 1, Some(11))
    );
    // Fetching by the new name lands on the same row; nothing moved.
    let ing = s
            .record(
                &fetch("Champion"),
                &json!({ "name": "Champion" }),
                200,
                &json!({ "character": { "id": "c1", "name": "Champion", "league": "Standard", "equipment": [ item("eq1", "Helm", 0) ] } }),
                21,
            )
            .unwrap();
    assert_eq!(
        (ing.added, ing.moved, ing.changed, ing.removed),
        (0, 0, 0, 0)
    );
    assert_eq!(s.item("eq1").unwrap().unwrap().location_id, "c1");
    // A fetch authorized under the old address that lands after the
    // rename (separate routes, concurrent sends) must not roll the
    // address back: the listing owns name/class/level once it has
    // named the row; the body's own say stays in json.
    s.record(
            &fetch("Hero"),
            &json!({ "name": "Hero" }),
            200,
            &json!({ "character": { "id": "c1", "name": "Hero", "league": "Standard", "level": 90, "equipment": [ item("eq1", "Helm", 0) ] } }),
            22,
        )
        .unwrap();
    let row = &s.characters(None, None).unwrap()[0];
    assert_eq!((row.name.as_str(), row.level), ("Champion", Some(91)));
    let json: String = s
        .conn
        .query_row("SELECT json FROM characters WHERE id = 'c1'", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert!(json.contains("\"Hero\""));
    // The listing owns league: a later fetch saying otherwise does not
    // move the coverage coordinate (its body keeps its own say).
    s.record(
        &list,
        &json!({}),
        200,
        &json!({ "characters": [ { "id": "c1", "name": "Champion", "league": "Hardcore" } ] }),
        30,
    )
    .unwrap();
    s.record(
            &fetch("Champion"),
            &json!({ "name": "Champion" }),
            200,
            &json!({ "character": { "id": "c1", "name": "Champion", "league": "Standard", "equipment": [ item("eq1", "Helm", 0) ] } }),
            31,
        )
        .unwrap();
    assert_eq!(
        s.characters(None, None).unwrap()[0].league.as_deref(),
        Some("Hardcore")
    );
    // …and its items take the row's coordinate, not the body's.
    assert_eq!(
        s.item("eq1").unwrap().unwrap().league.as_deref(),
        Some("Hardcore")
    );
    // Deleted and recreated under the old name: a new id, never
    // fetched, and the old row retired — a name-keyed store would have
    // inherited c1's freshness and never fetched the new one.
    s.record(
            &fetch("Champion"),
            &json!({ "name": "Champion" }),
            200,
            &json!({ "character": { "id": "c1", "name": "Champion", "league": "Standard", "equipment": [ item("eq1", "Helm", 0) ] } }),
            35,
        )
        .unwrap();
    let ing = s
        .record(
            &list,
            &json!({}),
            200,
            &json!({ "characters": [ { "id": "c2", "name": "Hero", "league": "Standard" } ] }),
            40,
        )
        .unwrap();
    let rows = s.characters(None, None).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(
        (rows[0].id.as_str(), rows[0].fetched_at, rows[0].item_count),
        ("c2", None, 0)
    );
    // The retired character's items went with it: retired, with a
    // removed event.
    assert_eq!(ing.removed, 1);
    assert!(s.item("eq1").unwrap().unwrap().removed_at.is_some());
    let removed_events: i64 = s
            .conn
            .query_row(
                "SELECT count(*) FROM item_events WHERE item_id = 'eq1' AND kind = 'removed' AND from_location = 'character/pc/c1' AND at = 40",
                [],
                |r| r.get(0),
            )
            .unwrap();
    assert_eq!(removed_events, 1);
    let err = s
        .record(
            &fetch("Hero"),
            &json!({ "name": "Hero" }),
            200,
            &json!({ "character": { "name": "Hero" } }),
            41,
        )
        .unwrap_err();
    assert!(err.downcast_ref::<MalformedBody>().is_some(), "{err:#}");
}

/// Every item row records the array it came from: a guardian's helm
/// and the character's own are distinguishable only by that, and a
/// move between arrays is a `changed` event even with identical json.
#[test]
fn container_is_an_ingest_fact_and_moving_between_arrays_is_a_change() {
    let mut s = Store::open_memory().unwrap();
    let ep = Endpoint::Character {
        realm: "pc".into(),
        name: "Hero".into(),
    };
    let helm = |id: &str| json!({ "id": id, "name": "", "typeLine": "Iron Hat", "baseType": "Iron Hat", "inventoryId": "Helm", "x": 0, "y": 0 });
    s.record(
            &ep,
            &json!({ "name": "Hero" }),
            200,
            &json!({ "character": { "id": "c1", "name": "Hero",
                "equipment": [ helm("own") ], "guardian": [ helm("pet") ], "skills": [ item("sk", "Skill", 0) ] } }),
            1,
        )
        .unwrap();
    let own = s.item("own").unwrap().unwrap();
    let pet = s.item("pet").unwrap().unwrap();
    assert_eq!(own.container.as_deref(), Some("equipment"));
    assert_eq!(pet.container.as_deref(), Some("guardian"));
    assert_eq!(
        s.item("sk").unwrap().unwrap().container.as_deref(),
        Some("skills")
    );
    assert_eq!(own.location_id, pet.location_id);
    // Swap the helms between the arrays: json identical, containers
    // differ — changed, not moved (one location).
    let ing = s
            .record(
                &ep,
                &json!({ "name": "Hero" }),
                200,
                &json!({ "character": { "id": "c1", "name": "Hero",
                    "equipment": [ helm("pet") ], "guardian": [ helm("own") ], "skills": [ item("sk", "Skill", 0) ] } }),
                2,
            )
            .unwrap();
    assert_eq!((ing.moved, ing.changed, ing.removed), (0, 2, 0));
    assert_eq!(
        s.item("own").unwrap().unwrap().container.as_deref(),
        Some("guardian")
    );
    // A stash item's container is `items`.
    s.record(
        &stash_ep("t1"),
        &json!({}),
        200,
        &stash("t1", vec![item("i1", "Foo", 0)]),
        3,
    )
    .unwrap();
    assert_eq!(
        s.item("i1").unwrap().unwrap().container.as_deref(),
        Some("items")
    );
}

/// The drift tripwire: an item-shaped array the store does not lift
/// is counted into the envelope and the status — never a failure, and
/// never an item row. Arrays of non-items are left alone.
/// PoE2 first contact (2026-09-02): a weapon that grants a skill
/// carries it as an id-less gem in its `socketedItems`, GGG repeats
/// the identical object as `skills[0]`, and a support socketed into
/// the granted skill is id-less too. Ruled (a): the subtree is a
/// property of the host — left in place, counted, never a row, never
/// a refusal — and every other id-less shape is still malformed.
#[test]
fn an_item_granted_skill_stays_in_place_and_is_not_an_item() {
    let mut s = Store::open_memory().unwrap();
    let ep = Endpoint::Character {
        realm: "poe2".into(),
        name: "Necro".into(),
    };
    let p = json!({ "realm": "poe2", "name": "Necro" });
    let granted = json!({ "typeLine": "Skeletal Warrior", "baseType": "Skeletal Warrior", "frameType": 4, "frameTypeId": "Gem",
            "support": false, "sockets": [ { "group": 0, "type": "gem" } ],
            "socketedItems": [ { "typeLine": "Meat Shield I", "frameType": 4, "frameTypeId": "Gem", "support": true } ] });
    let mut sceptre = item("sceptre", "Rattling Sceptre", 0);
    sceptre["sockets"] = json!([]);
    sceptre["socketedItems"] = json!([granted.clone()]);
    let mut focus = item("focus", "Woven Focus", 1);
    focus["socketedItems"] = json!([ { "id": "rune", "typeLine": "Desert Rune", "frameType": 5, "frameTypeId": "Currency" } ]);
    let mut spark = item("spark", "Spark", 0);
    spark["inventoryId"] = json!("SkillSlots");
    spark["socketedItems"] = json!([item("sup", "Pierce I", 0)]);
    let body = json!({ "character": { "id": "n1", "name": "Necro", "league": "Standard", "realm": "poe2",
            "equipment": [ sceptre, focus ],
            "skills": [ granted.clone(), spark ] } });
    let ing = s.record(&ep, &p, 200, &body, 10).unwrap();
    // Rows: the sceptre, the focus and its rune, the real skill gem
    // and its support. Nothing for the granted skill or its support.
    assert_eq!((ing.items, ing.added, ing.granted), (5, 5, 2));
    assert_eq!(
        s.item("rune").unwrap().unwrap().socketed_in.as_deref(),
        Some("focus")
    );
    assert_eq!(
        s.item("sup").unwrap().unwrap().socketed_in.as_deref(),
        Some("spark")
    );
    let granted_rows: i64 = s
            .conn
            .query_row(
                "SELECT count(*) FROM items WHERE type_line LIKE '%Skeletal%' OR type_line LIKE '%Meat Shield%'",
                [],
                |r| r.get(0),
            )
            .unwrap();
    assert_eq!(granted_rows, 0);
    // The host's json keeps the granted subtree; the rune was lifted
    // out of the focus as usual.
    let sceptre_row = s.item("sceptre").unwrap().unwrap();
    assert_eq!(sceptre_row.json["socketedItems"], json!([granted]));
    assert!(
        s.item("focus")
            .unwrap()
            .unwrap()
            .json
            .get("socketedItems")
            .is_none()
    );
    // The envelope keeps the granted copy under `skills`, splits the
    // lifted one, and counts both positions; the drift tripwire does
    // not mistake the kept array for an unknown one.
    let stored: String = s
        .conn
        .query_row("SELECT json FROM characters WHERE id = 'n1'", [], |r| {
            r.get(0)
        })
        .unwrap();
    let c: Value = serde_json::from_str(&stored).unwrap();
    assert_eq!(c["skills"], json!([granted]));
    assert_eq!(c["_split"], json!({ "equipment": 2, "skills": 1 }));
    assert_eq!(c["_granted"], json!({ "equipment": 1, "skills": 1 }));
    assert!(c.get("_unlifted").is_none());
    let st = s.status().unwrap();
    // One skill shown in two places is two entries: the count is of
    // entries, and every surface says so (C146; the h2 review).
    assert_eq!((st.items, st.granted_skills, st.refused_bodies), (5, 2, 0));
    // A support swapped inside the granted skill is a change on the
    // host — the only row that records it.
    let mut body2 = body.clone();
    body2["character"]["equipment"][0]["socketedItems"][0]["socketedItems"][0]["typeLine"] =
        json!("Feeding Frenzy I");
    let ing = s.record(&ep, &p, 200, &body2, 20).unwrap();
    assert_eq!((ing.items, ing.changed, ing.granted), (5, 1, 2));
    // Strictness elsewhere is untouched: an id-less gem at the top of
    // `equipment` (not a socketed position, not `skills`) and an
    // id-less non-gem socketed anywhere are still refused.
    let mut loose = body.clone();
    loose["character"]["equipment"]
        .as_array_mut()
        .unwrap()
        .push(granted.clone());
    let err = s.record(&ep, &p, 200, &loose, 30).unwrap_err();
    let m = err
        .downcast_ref::<MalformedBody>()
        .unwrap_or_else(|| panic!("{err:#}"));
    assert_eq!(
        m.at.map(|a| (a.array, a.index, a.socketed)),
        Some(("equipment", 2, None))
    );
    let mut junk = body.clone();
    junk["character"]["equipment"][1]["socketedItems"][0] =
        json!({ "typeLine": "No Id Rune", "frameTypeId": "Currency" });
    let err = s.record(&ep, &p, 200, &junk, 31).unwrap_err();
    let m = err
        .downcast_ref::<MalformedBody>()
        .unwrap_or_else(|| panic!("{err:#}"));
    assert_eq!(
        m.at.map(|a| (a.array, a.index, a.socketed)),
        Some(("equipment", 1, Some(0)))
    );
    assert_eq!(s.status().unwrap().refused_bodies, 2);
    // A withheld body counts item facts the same way: the granted
    // subtree is not one.
    assert_eq!(count_item_facts(body["character"].get("equipment")), 3);
    assert_eq!(count_item_facts(body["character"].get("skills")), 2);
}

#[test]
fn an_unknown_item_array_trips_the_drift_counter_without_failing() {
    let mut s = Store::open_memory().unwrap();
    let ep = Endpoint::Character {
        realm: "pc".into(),
        name: "Hero".into(),
    };
    let ing = s
        .record(
            &ep,
            &json!({ "name": "Hero" }),
            200,
            &json!({ "character": { "id": "c1", "name": "Hero",
                    "equipment": [ item("eq1", "Helm", 0) ],
                    "pets": [ item("p1", "Cat", 0), item("p2", "Dog", 0) ],
                    "passives": [1, 2, 3], "notes": [] } }),
            1,
        )
        .unwrap();
    assert_eq!((ing.items, ing.unlifted), (1, 2));
    assert!(s.item("p1").unwrap().is_none());
    assert_eq!(s.status().unwrap().unlifted_items, 2);
    let json: String = s
        .conn
        .query_row("SELECT json FROM characters WHERE id = 'c1'", [], |r| {
            r.get(0)
        })
        .unwrap();
    let v: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["_unlifted"], json!({ "pets": 2 }));
    assert_eq!(v["_split"], json!({ "equipment": 1 }));
    assert_eq!(v["passives"], json!([1, 2, 3]));
    // The count is the latest fetch's, per live character — a repeat
    // does not double it, and a fetch without the array clears it.
    s.record(
            &ep,
            &json!({ "name": "Hero" }),
            200,
            &json!({ "character": { "id": "c1", "name": "Hero",
                "equipment": [ item("eq1", "Helm", 0) ], "pets": [ item("p1", "Cat", 0), item("p2", "Dog", 0) ] } }),
            2,
        )
        .unwrap();
    assert_eq!(s.status().unwrap().unlifted_items, 2);
    s.record(
            &ep,
            &json!({ "name": "Hero" }),
            200,
            &json!({ "character": { "id": "c1", "name": "Hero", "equipment": [ item("eq1", "Helm", 0) ] } }),
            3,
        )
        .unwrap();
    assert_eq!(s.status().unwrap().unlifted_items, 0);
}

/// Membership belongs to the listing: a fetch that lands after a newer
/// listing retired its location records the body but revives nothing
/// — the row stays retired and its item facts are withheld — until a
/// listing names it again; then the next fetch lands them.
#[test]
fn a_late_fetch_never_revives_a_location_a_listing_retired() {
    let mut s = Store::open_memory().unwrap();
    let list = Endpoint::Characters { realm: "pc".into() };
    let fetch = Endpoint::Character {
        realm: "pc".into(),
        name: "Hero".into(),
    };
    let body = json!({ "character": { "id": "c1", "name": "Hero", "league": "Standard", "equipment": [ item("eq1", "Helm", 0) ] } });
    let named = json!({ "characters": [ { "id": "c1", "name": "Hero", "league": "Standard" } ] });
    s.record(&list, &json!({}), 200, &named, 10).unwrap();
    s.record(&fetch, &json!({ "name": "Hero" }), 200, &body, 11)
        .unwrap();
    s.record(&list, &json!({}), 200, &json!({ "characters": [] }), 20)
        .unwrap();
    let ing = s
        .record(&fetch, &json!({ "name": "Hero" }), 200, &body, 21)
        .unwrap();
    assert_eq!(
        (ing.items, ing.added, ing.moved, ing.withheld),
        (0, 0, 0, Some(1))
    );
    assert!(s.characters(None, None).unwrap().is_empty());
    assert!(s.item("eq1").unwrap().unwrap().removed_at.is_some());
    // The row was not touched (its fetched_at is still the live
    // fetch's), and the whole body — arrays included — sits verbatim
    // on the response row, marked withheld.
    let fetched_at: i64 = s
        .conn
        .query_row(
            "SELECT fetched_at FROM characters WHERE id = 'c1'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(fetched_at, 11);
    let (held, envelope): (i64, String) = s
            .conn
            .query_row(
                "SELECT withheld, envelope FROM responses WHERE endpoint = 'character' AND fetched_at = 21",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
    assert_eq!(held, 1);
    let envelope: Value = serde_json::from_str(&envelope).unwrap();
    assert_eq!(envelope["character"]["equipment"][0]["id"], "eq1");
    assert!(envelope["character"].get("_split").is_none());
    let st = s.status().unwrap();
    assert_eq!((st.withheld_responses, st.withheld_items), (1, 1));
    // A listing naming it again revives the row — with no fetch on
    // record, since its facts were retired with it — and the next
    // fetch lands the items (a reappearance is a move).
    s.record(&list, &json!({}), 200, &named, 30).unwrap();
    let revived = &s.characters(None, None).unwrap()[0];
    assert_eq!((revived.fetched_at, revived.item_count), (None, 0));
    let ing = s
        .record(&fetch, &json!({ "name": "Hero" }), 200, &body, 31)
        .unwrap();
    assert_eq!((ing.items, ing.moved, ing.withheld), (1, 1, None));
    let row = &s.characters(None, None).unwrap()[0];
    assert_eq!((row.fetched_at, row.item_count), (Some(31), 1));
    // The same rule for a tab.
    let listing = Endpoint::Stashes {
        realm: "pc".into(),
        league: "Standard".into(),
    };
    let one =
        json!({ "stashes": [ { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 } ] });
    s.record(&listing, &json!({}), 200, &one, 40).unwrap();
    s.record(
        &stash_ep("t1"),
        &json!({}),
        200,
        &stash("t1", vec![item("i1", "Foo", 0)]),
        41,
    )
    .unwrap();
    s.record(&listing, &json!({}), 200, &json!({ "stashes": [] }), 50)
        .unwrap();
    let ing = s
        .record(
            &stash_ep("t1"),
            &json!({}),
            200,
            &stash("t1", vec![item("i1", "Foo", 0)]),
            51,
        )
        .unwrap();
    assert_eq!((ing.items, ing.withheld), (0, Some(1)));
    assert!(s.tabs("pc", "Standard").unwrap().is_empty());
    assert!(s.item("i1").unwrap().unwrap().removed_at.is_some());
    // Revived by a listing: no fetch on record, so the next plan
    // fetches it.
    s.record(&listing, &json!({}), 200, &one, 60).unwrap();
    let revived = &s.tabs("pc", "Standard").unwrap()[0];
    assert_eq!((revived.fetched_at, revived.item_count), (None, 0));
}

/// A location is its full coordinate: the same tab id under two realms
/// is two locations, so a fetch of one never retires the other's items
/// and counts stay apart; an item that turns up under the other realm's
/// tab has moved.
#[test]
fn the_same_tab_id_under_two_realms_is_two_locations() {
    let mut s = Store::open_memory().unwrap();
    let ep = |realm: &str| Endpoint::Stash {
        realm: realm.into(),
        league: "Standard".into(),
        id: "t1".into(),
        sub: None,
    };
    s.record(
        &ep("pc"),
        &json!({}),
        200,
        &stash("t1", vec![item("i1", "Foo", 0)]),
        1,
    )
    .unwrap();
    let ing = s
        .record(
            &ep("xbox"),
            &json!({}),
            200,
            &stash("t1", vec![item("x1", "Bar", 0)]),
            2,
        )
        .unwrap();
    assert_eq!((ing.added, ing.removed), (1, 0));
    assert!(s.item("i1").unwrap().unwrap().removed_at.is_none());
    assert_eq!(s.tabs("pc", "Standard").unwrap()[0].item_count, 1);
    assert_eq!(s.tabs("xbox", "Standard").unwrap()[0].item_count, 1);
    let ing = s
        .record(
            &ep("xbox"),
            &json!({}),
            200,
            &stash("t1", vec![item("x1", "Bar", 0), item("i1", "Foo", 0)]),
            3,
        )
        .unwrap();
    assert_eq!(ing.moved, 1);
    assert_eq!(s.tabs("pc", "Standard").unwrap()[0].item_count, 0);
    assert_eq!(s.tabs("xbox", "Standard").unwrap()[0].item_count, 2);
    // The event names both coordinates in full.
    let (from, to): (String, String) = s
            .conn
            .query_row(
                "SELECT from_location, to_location FROM item_events WHERE item_id = 'i1' AND kind = 'moved'",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
    assert_eq!(
        (from.as_str(), to.as_str()),
        ("stash/pc/Standard/t1", "stash/xbox/Standard/t1")
    );
}

/// A parent tab's fetch is the listing of its substashes: a stub it no
/// longer carries is retired with its items; and when a listing
/// retires the parent itself, the substash rows stay (orphaned, for
/// the planner's report) but their items are not live inventory.
#[test]
fn substash_membership_follows_the_parents_fetch_and_the_parents_fate() {
    let mut s = Store::open_memory().unwrap();
    let parent = |children: &[&str]| {
        json!({ "stash": { "id": "m1", "name": "Maps", "type": "MapStash", "items": [],
                "children": children.iter().map(|c| json!({ "id": c, "name": "", "type": "MapStash", "parent": "m1", "metadata": { "items": 1 } })).collect::<Vec<_>>() } })
    };
    let sub = |id: &str| Endpoint::Stash {
        realm: "pc".into(),
        league: "Standard".into(),
        id: "m1".into(),
        sub: Some(id.into()),
    };
    let sub_body = |id: &str, item_id: &str| json!({ "stash": { "id": id, "name": "", "type": "MapStash", "parent": "m1", "items": [ item(item_id, "Map", 0) ] } });
    let listing = Endpoint::Stashes {
        realm: "pc".into(),
        league: "Standard".into(),
    };
    s.record(
        &listing,
        &json!({}),
        200,
        &json!({ "stashes": [ { "id": "m1", "name": "Maps", "type": "MapStash", "index": 0 } ] }),
        1,
    )
    .unwrap();
    s.record(&stash_ep("m1"), &json!({}), 200, &parent(&["s1", "s2"]), 2)
        .unwrap();
    s.record(&sub("s1"), &json!({}), 200, &sub_body("s1", "map1"), 3)
        .unwrap();
    // The parent no longer lists s1: retired, with its item.
    let ing = s
        .record(&stash_ep("m1"), &json!({}), 200, &parent(&["s2"]), 4)
        .unwrap();
    assert_eq!(ing.removed, 1);
    let ids: Vec<String> = s
        .tabs("pc", "Standard")
        .unwrap()
        .into_iter()
        .map(|t| t.id)
        .collect();
    assert_eq!(ids, vec!["m1", "s2"]);
    assert!(s.item("map1").unwrap().unwrap().removed_at.is_some());
    // s2 gets an item; then the listing drops the parent: s2's row
    // stays (orphaned) and its item is no longer live.
    s.record(&sub("s2"), &json!({}), 200, &sub_body("s2", "map2"), 5)
        .unwrap();
    let ing = s
        .record(&listing, &json!({}), 200, &json!({ "stashes": [] }), 6)
        .unwrap();
    assert_eq!(ing.removed, 1);
    let ids: Vec<String> = s
        .tabs("pc", "Standard")
        .unwrap()
        .into_iter()
        .map(|t| t.id)
        .collect();
    assert_eq!(ids, vec!["s2"]);
    assert!(s.item("map2").unwrap().unwrap().removed_at.is_some());
    let s2 = &s.tabs("pc", "Standard").unwrap()[0];
    // …and its freshness went with its facts: no fetch on record.
    assert_eq!((s2.item_count, s2.fetched_at), (0, None));
    // With the parent retired, a late fetch of the substash is
    // withheld (its parent is not live), and a late fetch of the
    // parent neither revives it nor rewrites its children: the orphan
    // report stays exactly as the listing left it.
    let ing = s
        .record(&sub("s2"), &json!({}), 200, &sub_body("s2", "map2"), 7)
        .unwrap();
    assert_eq!((ing.items, ing.withheld), (0, Some(1)));
    assert!(s.item("map2").unwrap().unwrap().removed_at.is_some());
    let ing = s
        .record(&stash_ep("m1"), &json!({}), 200, &parent(&["s9"]), 8)
        .unwrap();
    assert_eq!((ing.items, ing.withheld), (0, Some(0)));
    let ids: Vec<String> = s
        .tabs("pc", "Standard")
        .unwrap()
        .into_iter()
        .map(|t| t.id)
        .collect();
    assert_eq!(ids, vec!["s2"]);
}

/// Removal events name exactly the items this response retired: two
/// responses in one second retiring different items at one location
/// each get their own event, never a duplicate attributed to the
/// later one.
#[test]
fn removal_events_come_from_the_update_not_the_clock() {
    let mut s = Store::open_memory().unwrap();
    let at = 10;
    for items in [
        vec![item("i1", "A", 0), item("i2", "B", 1), item("i3", "C", 2)],
        vec![item("i1", "A", 0), item("i2", "B", 1)],
        vec![item("i1", "A", 0)],
    ] {
        s.record(&stash_ep("t1"), &json!({}), 200, &stash("t1", items), at)
            .unwrap();
    }
    let removed: Vec<(String, i64)> = {
        let mut stmt = s
            .conn
            .prepare(
                "SELECT item_id, response_id FROM item_events WHERE kind = 'removed' ORDER BY id",
            )
            .unwrap();
        let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?))).unwrap();
        rows.collect::<Result<_, _>>().unwrap()
    };
    assert_eq!(removed, vec![("i3".to_string(), 2), ("i2".to_string(), 3)]);
}

/// `withheld` is a marker as well as a count: a withheld fetch of an
/// empty location is still marked (`Some(0)`), the count is every item
/// fact the body carried (socketed gems included), and the
/// malformed-body contract holds for a withheld body too — an id-less
/// item is refused and nothing is written, not even the response row.
#[test]
fn a_withheld_fetch_is_marked_counted_exactly_and_still_validated() {
    let mut s = Store::open_memory().unwrap();
    let listing = Endpoint::Stashes {
        realm: "pc".into(),
        league: "Standard".into(),
    };
    s.record(
            &listing,
            &json!({}),
            200,
            &json!({ "stashes": [ { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 } ] }),
            1,
        )
        .unwrap();
    s.record(&listing, &json!({}), 200, &json!({ "stashes": [] }), 2)
        .unwrap();
    // Empty, withheld: marked, count 0.
    let ing = s
        .record(&stash_ep("t1"), &json!({}), 200, &stash("t1", vec![]), 3)
        .unwrap();
    assert_eq!(ing.withheld, Some(0));
    // A bow with a socketed gem: two item facts withheld.
    let mut bow = item("bow", "Bow", 0);
    bow["socketedItems"] = json!([ { "id": "gem", "typeLine": "Gem", "baseType": "Gem" } ]);
    let ing = s
        .record(&stash_ep("t1"), &json!({}), 200, &stash("t1", vec![bow]), 4)
        .unwrap();
    assert_eq!(ing.withheld, Some(2));
    let st = s.status().unwrap();
    assert_eq!((st.withheld_responses, st.withheld_items), (2, 2));
    // An ordinary response is not marked.
    let ing = s
        .record(&listing, &json!({}), 200, &json!({ "stashes": [] }), 5)
        .unwrap();
    assert_eq!(ing.withheld, None);
    // Malformed stays malformed at a retired location.
    let responses_before = s.status().unwrap().responses;
    let err = s
        .record(
            &stash_ep("t1"),
            &json!({}),
            200,
            &json!({ "stash": { "id": "t1", "items": [ { "typeLine": "NoId" } ] } }),
            6,
        )
        .unwrap_err();
    assert!(err.downcast_ref::<MalformedBody>().is_some(), "{err:#}");
    let err = s
        .record(
            &stash_ep("t1"),
            &json!({}),
            200,
            &json!({ "stash": { "id": "t1", "items": [], "children": [ { "name": "NoId" } ] } }),
            7,
        )
        .unwrap_err();
    assert!(err.downcast_ref::<MalformedBody>().is_some(), "{err:#}");
    assert_eq!(s.status().unwrap().responses, responses_before);
}

/// A v5 file's `withheld` (NOT NULL DEFAULT 0) is rebuilt nullable:
/// a positive count stays a withheld mark, 0 becomes an ordinary
/// response (v5 could not tell an empty withheld fetch apart).
#[test]
fn a_v5_withheld_column_is_rebuilt_nullable() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let path = dir.join("v5.db");
    {
        let conn = Connection::open(&path).unwrap();
        conn.execute_batch(
                "CREATE TABLE responses (id INTEGER PRIMARY KEY, endpoint TEXT NOT NULL, params TEXT NOT NULL,
                    fetched_at INTEGER NOT NULL, status INTEGER NOT NULL, envelope TEXT NOT NULL, item_count INTEGER NOT NULL,
                    withheld INTEGER NOT NULL DEFAULT 0);
                 INSERT INTO responses (endpoint, params, fetched_at, status, envelope, item_count, withheld)
                 VALUES ('stash', '{}', 1, 200, '{}', 0, 2), ('stash', '{}', 2, 200, '{}', 3, 0);
                 PRAGMA user_version = 5;",
            )
            .unwrap();
    }
    let s = Store::open(&path).unwrap();
    let rows: Vec<Option<i64>> = {
        let mut stmt = s
            .conn
            .prepare("SELECT withheld FROM responses ORDER BY id")
            .unwrap();
        let rows = stmt.query_map([], |r| r.get(0)).unwrap();
        rows.collect::<Result<_, _>>().unwrap()
    };
    assert_eq!(rows, vec![Some(2), None]);
    let st = s.status().unwrap();
    assert_eq!((st.withheld_responses, st.withheld_items), (1, 2));
    drop(s);
}

/// The bytes of a file, hashed: what "left as it was found" is judged by.
fn file_hash(path: &Path) -> Vec<u8> {
    use sha2::Digest;
    sha2::Sha256::digest(std::fs::read(path).unwrap()).to_vec()
}

/// C124 (a read never migrates): a file one version behind (a v8
/// file, without the v9 live indexes) and one a version ahead are
/// each refused by the read-only door with `SchemaNotCurrent`, in
/// words that name both versions and the remedy, and the file's bytes
/// are the same after the attempt. The control: `Store::open` on the
/// behind file migrates it and its hash moves, so an unchanged hash
/// is the door's doing and not a hash blind to a migration. A missing
/// file is refused and not created.
#[test]
fn c124_a_read_only_open_refuses_another_version_and_leaves_the_file_unchanged() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path();
    let missing = dir.join("missing.db");
    assert!(Store::open_read_only(&missing).is_err());
    assert!(!missing.exists(), "a read-only open created the file");
    for (stamp, ddl) in [
        (
            FACT_SCHEMA_VERSION - 1,
            "DROP INDEX items_live; DROP INDEX items_live_at;",
        ),
        (FACT_SCHEMA_VERSION + 1, ""),
    ] {
        let path = dir.join(format!("v{stamp}.db"));
        drop(Store::open(&path).unwrap());
        {
            let c = Connection::open(&path).unwrap();
            c.execute_batch(ddl).unwrap();
            c.pragma_update(None, "user_version", stamp).unwrap();
        }
        let before = file_hash(&path);
        let err = Store::open_read_only(&path).err().unwrap();
        assert_eq!(
            err.downcast_ref::<SchemaNotCurrent>(),
            Some(&SchemaNotCurrent {
                file: "facts",
                found: stamp,
                supported: FACT_SCHEMA_VERSION,
                remedy: MIGRATE_REMEDY,
            })
        );
        let text = format!("{err:#}");
        assert!(
            text.contains(&format!("schema version {stamp}"))
                && text.contains(&format!("this build's version {FACT_SCHEMA_VERSION}")),
            "{text}"
        );
        let remedy = if stamp < FACT_SCHEMA_VERSION {
            "open it with `acq` once to migrate, or use the build that wrote it"
        } else {
            "use the build that wrote it"
        };
        assert!(text.contains(remedy), "{text}");
        assert_eq!(
            file_hash(&path),
            before,
            "the refused v{stamp} file changed"
        );
        if stamp < FACT_SCHEMA_VERSION {
            drop(Store::open(&path).unwrap());
            assert_ne!(
                file_hash(&path),
                before,
                "the control migration left no mark"
            );
        }
    }
    // An unstamped file (empty: no Acquisition store is here) is
    // refused with the next step, and stays empty.
    let empty = dir.join("empty.db");
    std::fs::write(&empty, b"").unwrap();
    let err = Store::open_read_only(&empty).err().unwrap();
    assert_eq!(
        err.downcast_ref::<SchemaNotCurrent>().map(|e| e.found),
        Some(0)
    );
    let text = format!("{err:#}");
    assert!(
        text.contains("carries no schema version")
            && text
                .contains("check that `--dir` and `--account` name an existing Acquisition store"),
        "{text}"
    );
    assert!(
        std::fs::read(&empty).unwrap().is_empty(),
        "the empty file was written"
    );
}

/// C124: a file at this build's version opens through the read-only
/// door and reads; a write through the handle is SQLite's refusal;
/// the file's bytes are the same after.
#[test]
fn c124_a_read_only_open_reads_a_current_file_and_writes_nothing() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path();
    let path = dir.join("current.db");
    {
        let mut s = Store::open(&path).unwrap();
        s.record(
            &stash_ep("a"),
            &json!({"league":"Standard","id":"a"}),
            200,
            &stash("a", vec![item("i1", "Foo", 0)]),
            100,
        )
        .unwrap();
    }
    let before = file_hash(&path);
    let mut ro = Store::open_read_only(&path).unwrap();
    assert_eq!(ro.item("i1").unwrap().unwrap().name, "Foo");
    assert!(
        ro.record(
            &stash_ep("a"),
            &json!({"league":"Standard","id":"a"}),
            200,
            &stash("a", vec![]),
            200,
        )
        .is_err(),
        "a write through the read-only door landed"
    );
    drop(ro);
    assert_eq!(file_hash(&path), before, "the current file changed");
}
