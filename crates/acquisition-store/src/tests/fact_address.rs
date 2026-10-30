use super::*;
use serde_json::json;

/// `Store::holds` answers by the store's identity, league-less: an
/// item and a character by id, a top-level tab and a folder's child
/// as `Tab`, a map tab's substash only as `Substash` under its parent;
/// a removed item still counts (intent on it is kept, C35); an id the
/// facts never saw does not.
#[test]
fn holds_answers_by_identity_across_leagues_and_removal() {
    let mut s = Store::open_memory().unwrap();
    let stashes = Endpoint::Stashes {
        realm: "pc".into(),
        league: "Standard".into(),
    };
    s.record(
            &stashes,
            &json!({}),
            200,
            &json!({ "stashes": [
                { "id": "f1", "name": "Folder", "type": "Folder", "index": 0,
                  "children": [ { "id": "c1", "name": "In folder", "type": "PremiumStash", "index": 1 } ] },
                { "id": "m1", "name": "Maps", "type": "MapStash", "index": 2,
                  "children": [ { "id": "s1", "name": "", "type": "MapStash", "parent": "m1",
                                  "metadata": { "map": { "name": "Tier 16" } } } ] } ] }),
            10,
        )
        .unwrap();
    let stash = |id: &str| Endpoint::Stash {
        realm: "pc".into(),
        league: "Standard".into(),
        id: id.into(),
        sub: None,
    };
    let item = |id: &str| json!({ "id": id, "typeLine": "Chaos Orb", "baseType": "Chaos Orb", "x": 0, "y": 0 });
    s.record(
            &stash("c1"),
            &json!({}),
            200,
            &json!({ "stash": { "id": "c1", "name": "In folder", "type": "PremiumStash", "items": [item("i1")] } }),
            11,
        )
        .unwrap();
    s.record(
            &stash("c1"),
            &json!({}),
            200,
            &json!({ "stash": { "id": "c1", "name": "In folder", "type": "PremiumStash", "items": [] } }),
            12,
        )
        .unwrap();
    s.record(
        &Endpoint::Characters { realm: "pc".into() },
        &json!({}),
        200,
        &json!({ "characters": [ { "id": "ch1", "name": "Exile", "league": "Hardcore" } ] }),
        13,
    )
    .unwrap();
    assert!(s.item("i1").unwrap().unwrap().removed_at.is_some());
    for (address, held) in [
        (FactAddress::Item { id: "i1" }, true),
        (FactAddress::Item { id: "i-never" }, false),
        (FactAddress::Character { id: "ch1" }, true),
        (FactAddress::Character { id: "Exile" }, false),
        (
            FactAddress::Tab {
                realm: "pc",
                id: "f1",
            },
            true,
        ),
        (
            FactAddress::Tab {
                realm: "pc",
                id: "c1",
            },
            true,
        ),
        (
            FactAddress::Tab {
                realm: "pc",
                id: "m1",
            },
            true,
        ),
        (
            FactAddress::Tab {
                realm: "pc",
                id: "s1",
            },
            false,
        ),
        (
            FactAddress::Tab {
                realm: "poe2",
                id: "m1",
            },
            false,
        ),
        (
            FactAddress::Substash {
                realm: "pc",
                parent: "m1",
                id: "s1",
            },
            true,
        ),
        // a folder's child is a tab, never a substash of the folder
        (
            FactAddress::Substash {
                realm: "pc",
                parent: "f1",
                id: "c1",
            },
            false,
        ),
        (
            FactAddress::Substash {
                realm: "pc",
                parent: "m1",
                id: "s-never",
            },
            false,
        ),
    ] {
        assert_eq!(s.holds(&address).unwrap(), held, "{address:?}");
    }
}
