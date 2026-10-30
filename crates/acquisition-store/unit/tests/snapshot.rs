use super::*;
use crate::{Endpoint, annotations_path};
use serde_json::json;

fn listing_ep() -> Endpoint {
    Endpoint::Stashes {
        realm: "pc".into(),
        league: "Standard".into(),
    }
}

fn stash_ep(id: &str, sub: Option<&str>) -> Endpoint {
    Endpoint::Stash {
        realm: "pc".into(),
        league: "Standard".into(),
        id: id.into(),
        sub: sub.map(str::to_string),
    }
}

fn item(id: &str) -> Value {
    json!({ "id": id, "name": "Foo", "typeLine": "Imperial Bow", "baseType": "Imperial Bow", "x": 0, "y": 0 })
}

/// The flat scanner over the extract's array: a string reads with its
/// escapes, null is none, anything else is unread at its place alone
/// — a value nested past any decoder's depth beside a valid one loses
/// nothing (the step-9 review, second look) — and a text that is no
/// array of the fields reads as every field unread.
#[test]
fn body_strings_read_each_field_by_its_json_type_and_never_descend() {
    let read = |text: &str| body_strings::<2>(Some(text));
    assert_eq!(
        read(r#"["~price 5 chaos","Stash1"]"#),
        [
            (Some("~price 5 chaos".into()), false),
            (Some("Stash1".into()), false)
        ]
    );
    assert_eq!(
        read(r#"[null, "a \"quoted\", [comma] \\ slot"]"#),
        [
            (None, false),
            (Some("a \"quoted\", [comma] \\ slot".into()), false)
        ]
    );
    assert_eq!(
        read(r#"[5,{"x":[1,{"y":"]"}]}]"#),
        [(None, true), (None, true)]
    );
    let mut deep = String::from("\"~price 5 chaos\"");
    for _ in 0..200 {
        deep = format!("[{deep}]");
    }
    assert_eq!(
        read(&format!("[{deep},\"Stash1\"]")),
        [(None, true), (Some("Stash1".into()), false)]
    );
    assert_eq!(
        read(&format!("[\"~price 5 chaos\",{deep}]")),
        [(Some("~price 5 chaos".into()), false), (None, true)]
    );
    for broken in [
        "",
        "[",
        "[\"a\"]",
        "[\"a\",\"b\",\"c\"]",
        "{}",
        "[\"a\",\"b\"] x",
    ] {
        assert_eq!(read(broken), [(None, true), (None, true)], "{broken:?}");
    }
    assert_eq!(body_strings::<2>(None), [(None, false), (None, false)]);
}

/// A store whose account identity is on record, as every real store's
/// is after one login (`/profile` lands at login since 2026-08-31).
fn store() -> Store {
    let mut s = Store::open_memory().unwrap();
    s.record(
        &Endpoint::Profile,
        &json!({}),
        200,
        &json!({ "uuid": "u-1", "name": "tom" }),
        1,
    )
    .unwrap();
    s
}

/// C39 — the store exposes a neutral snapshot (bases, rows, the policy at its revision), never half a planner.
#[test]
fn the_snapshot_names_the_latest_listing_and_the_policy_revision() {
    let mut s = store();
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    // A direct `stashes` job may record with no league in its params;
    // that listing defaulted to Standard and must be found as such.
    s.record(
            &listing_ep(),
            &json!({}),
            200,
            &json!({ "stashes": [ { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 } ] }),
            100,
        )
        .unwrap();
    let first = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    let basis = first.stash_listing.unwrap();
    assert_eq!(basis.fetched_at, 100);
    assert!(first.policy.is_none());
    assert_eq!(first.account_uuid, "u-1");
    assert_eq!(first.account_name.as_deref(), Some("tom"));
    // The refresh parent records the same listing with normalized
    // params; a later listing replaces the basis, and the tab set is
    // stamped with the basis it belongs to.
    s.record(
            &listing_ep(),
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": [ { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 } ] }),
            200,
        )
        .unwrap();
    let via = crate::annotations::test_kinds::via_test();
    let policy = a
        .put::<crate::annotations::test_kinds::Policy>(
            "account",
            "",
            &json!({ "version": 1, "leagues": ["Standard"], "deep": false }),
            None,
            &via,
        )
        .unwrap();
    let snap = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    let later = snap.stash_listing.unwrap();
    assert!(later.response_id > basis.response_id);
    assert_eq!(later.fetched_at, 200);
    assert_eq!(snap.tabs[0].listed_response, Some(later.response_id));
    let row = snap.policy.unwrap();
    assert_eq!((row.revision, &row.value), (1, &policy.value));
    // A tombstoned policy is no policy — but its revision still gates
    // the next write, which is the annotation layer's business.
    a.delete("account", "", SYNC_POLICY_KIND, 1, &via).unwrap();
    assert!(
        s.refresh_snapshot("pc", "Standard", &a)
            .unwrap()
            .policy
            .is_none()
    );
}

#[test]
fn the_basis_is_per_league_and_absent_when_never_listed() {
    let mut s = store();
    let a = Annotations::open_memory_for("u-1").unwrap();
    s.record(
        &Endpoint::Stashes {
            realm: "pc".into(),
            league: "Hardcore".into(),
        },
        &json!({ "league": "Hardcore" }),
        200,
        &json!({ "stashes": [ { "id": "h1", "name": "HC", "type": "PremiumStash", "index": 0 } ] }),
        50,
    )
    .unwrap();
    // A tab fetched directly, with no listing for its league: it is in
    // the snapshot (fetched, never listed) and the basis stays None.
    s.record(
            &stash_ep("x1", None),
            &json!({ "league": "Standard", "id": "x1" }),
            200,
            &json!({ "stash": { "id": "x1", "name": "Fetched", "type": "PremiumStash", "items": [ item("i1") ] } }),
            60,
        )
        .unwrap();
    let std = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    assert!(std.stash_listing.is_none());
    assert_eq!(std.tabs.len(), 1);
    let x1 = &std.tabs[0];
    assert_eq!(
        (x1.id.as_str(), x1.listed_at, x1.fetched_at),
        ("x1", None, Some(60))
    );
    assert_eq!((x1.listed_response, x1.item_count), (None, 1));
    assert_eq!(x1.metadata, Value::Null);
    let hc = s.refresh_snapshot("pc", "Hardcore", &a).unwrap();
    assert_eq!(hc.stash_listing.unwrap().fetched_at, 50);
    assert_eq!(hc.tabs.len(), 1);
    assert_eq!(hc.tabs[0].id, "h1");
}

#[test]
fn metadata_rides_verbatim_and_removed_tabs_leave_the_snapshot() {
    let mut s = store();
    let a = Annotations::open_memory_for("u-1").unwrap();
    s.record(
            &listing_ep(),
            &json!({}),
            200,
            &json!({ "stashes": [
                { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0, "metadata": { "colour": "7c5436" } },
                { "id": "m1", "name": "Maps", "type": "MapStash", "index": 1 },
                { "id": "gone", "name": "Old", "type": "PremiumStash", "index": 2 },
            ]}),
            100,
        )
        .unwrap();
    // Fetching the map tab lists a substash stub whose metadata carries
    // the heuristic `items` count and the map name.
    s.record(
            &stash_ep("m1", None),
            &json!({ "league": "Standard", "id": "m1" }),
            200,
            &json!({ "stash": { "id": "m1", "name": "Maps", "type": "MapStash", "items": [], "children": [
                { "id": "s1", "name": "", "type": "MapStash", "parent": "m1",
                  "metadata": { "items": 1, "map": { "name": "Tier 16" } } } ] } }),
            110,
        )
        .unwrap();
    let snap = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    assert_eq!(
        snap.tabs.iter().map(|t| t.id.as_str()).collect::<Vec<_>>(),
        vec!["t1", "m1", "s1", "gone"]
    );
    let t1 = &snap.tabs[0];
    assert_eq!(t1.metadata, json!({ "colour": "7c5436" }));
    assert_eq!((t1.listed_at, t1.fetched_at), (Some(100), None));
    let s1 = snap.tabs.iter().find(|t| t.id == "s1").unwrap();
    assert_eq!(s1.parent.as_deref(), Some("m1"));
    assert_eq!(s1.metadata["items"], 1);
    assert_eq!(s1.metadata["map"]["name"], "Tier 16");
    // The next listing drops "gone": no longer coverage, so not in the
    // snapshot — while the never-listed substash survives.
    s.record(
            &listing_ep(),
            &json!({}),
            200,
            &json!({ "stashes": [
                { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0, "metadata": { "colour": "7c5436" } },
                { "id": "m1", "name": "Maps", "type": "MapStash", "index": 1 },
            ]}),
            200,
        )
        .unwrap();
    let ids: Vec<String> = s
        .refresh_snapshot("pc", "Standard", &a)
        .unwrap()
        .tabs
        .into_iter()
        .map(|t| t.id)
        .collect();
    assert_eq!(ids, vec!["t1", "m1", "s1"]);
}

#[test]
fn a_fetch_never_clobbers_the_listed_metadata() {
    let mut s = store();
    let a = Annotations::open_memory_for("u-1").unwrap();
    s.record(
            &listing_ep(),
            &json!({}),
            200,
            &json!({ "stashes": [
                { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0, "metadata": { "colour": "7c5436" } },
            ]}),
            100,
        )
        .unwrap();
    // The fetched body lands in `json` — with different metadata, as
    // GGG is free to send — and the listing's copy must survive it.
    s.record(
        &stash_ep("t1", None),
        &json!({ "league": "Standard", "id": "t1" }),
        200,
        &json!({ "stash": { "id": "t1", "name": "One", "type": "PremiumStash",
                "metadata": { "colour": "000000" }, "items": [ item("i1") ] } }),
        110,
    )
    .unwrap();
    let snap = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    let t1 = &snap.tabs[0];
    assert_eq!(t1.metadata, json!({ "colour": "7c5436" }));
    assert_eq!((t1.fetched_at, t1.item_count), (Some(110), 1));
    // The next listing refreshes the listed copy.
    s.record(
            &listing_ep(),
            &json!({}),
            200,
            &json!({ "stashes": [
                { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0, "metadata": { "colour": "ffffff" } },
            ]}),
            120,
        )
        .unwrap();
    let snap = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    assert_eq!(snap.tabs[0].metadata, json!({ "colour": "ffffff" }));
}

#[test]
fn two_listings_in_one_second_still_retire_dropped_tabs() {
    let mut s = store();
    let a = Annotations::open_memory_for("u-1").unwrap();
    let two = json!({ "stashes": [
        { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 },
        { "id": "t2", "name": "Two", "type": "PremiumStash", "index": 1 },
    ]});
    let one = json!({ "stashes": [
        { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 },
    ]});
    // Same unix second: membership must be per response, not per
    // clock tick, or t2 stays live while the basis says otherwise.
    s.record(&listing_ep(), &json!({}), 200, &two, 100).unwrap();
    s.record(&listing_ep(), &json!({}), 200, &one, 100).unwrap();
    let snap = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    let basis = snap.stash_listing.unwrap();
    assert_eq!(
        snap.tabs.iter().map(|t| t.id.as_str()).collect::<Vec<_>>(),
        vec!["t1"]
    );
    assert_eq!(snap.tabs[0].listed_response, Some(basis.response_id));
}

#[test]
fn a_listing_without_a_stashes_array_is_refused_whole() {
    let mut s = store();
    let a = Annotations::open_memory_for("u-1").unwrap();
    s.record(
            &listing_ep(),
            &json!({}),
            200,
            &json!({ "stashes": [ { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 } ] }),
            100,
        )
        .unwrap();
    // A 2xx with no `stashes` array (a maintenance page, a shape
    // change) is an error of the stable kind: no tab removed, no
    // basis minted.
    let err = s
        .record(
            &listing_ep(),
            &json!({}),
            200,
            &json!({ "error": "?" }),
            200,
        )
        .unwrap_err();
    assert!(
        err.downcast_ref::<crate::MalformedBody>().is_some(),
        "{err:#}"
    );
    let snap = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    assert_eq!(snap.stash_listing.unwrap().fetched_at, 100);
    assert_eq!(snap.tabs.len(), 1);
    // An array whose entries lack their identity is just as malformed:
    // ingesting it would retire every real tab. The error rolls the
    // whole transaction back — the id-less entry is not half-applied.
    let err = s
        .record(
            &listing_ep(),
            &json!({}),
            200,
            &json!({ "stashes": [ { "name": "NoId", "type": "PremiumStash" } ] }),
            300,
        )
        .unwrap_err();
    assert!(
        err.downcast_ref::<crate::MalformedBody>().is_some(),
        "{err:#}"
    );
    let snap = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    assert_eq!(snap.stash_listing.unwrap().fetched_at, 100);
    assert_eq!(snap.tabs.len(), 1);
    // Same rule for the character list: malformed is never "empty",
    // and an id-less (or name-less) entry poisons nothing.
    s.record(
        &Endpoint::Characters { realm: "pc".into() },
        &json!({}),
        200,
        &json!({ "characters": [ { "id": "c-hero", "name": "Hero", "league": "Standard" } ] }),
        100,
    )
    .unwrap();
    assert!(
        s.record(
            &Endpoint::Characters { realm: "pc".into() },
            &json!({}),
            200,
            &json!({}),
            200
        )
        .is_err()
    );
    assert!(
        s.record(
            &Endpoint::Characters { realm: "pc".into() },
            &json!({}),
            200,
            &json!({ "characters": [ { "league": "Standard" } ] }),
            200
        )
        .is_err()
    );
    assert!(
        s.record(
            &Endpoint::Characters { realm: "pc".into() },
            &json!({}),
            200,
            &json!({ "characters": [ { "id": "c-x", "league": "Standard" } ] }),
            200
        )
        .is_err()
    );
    assert_eq!(s.characters(None, None).unwrap().len(), 1);
}

#[test]
fn malformed_stored_listing_json_is_an_error_with_the_tab_address() {
    let mut s = store();
    let a = Annotations::open_memory_for("u-1").unwrap();
    s.record(
            &listing_ep(),
            &json!({}),
            200,
            &json!({ "stashes": [ { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 } ] }),
            100,
        )
        .unwrap();
    s.conn
        .execute("UPDATE tabs SET listed_json = 'not json'", [])
        .unwrap();
    let err = s.refresh_snapshot("pc", "Standard", &a).unwrap_err();
    assert!(err.to_string().contains("Standard/t1"), "{err:#}");
}

/// C38 — a plan's basis is one read binding facts and intent to one account uuid; C35 — the uuid lives inside the intent file.
#[test]
fn the_snapshot_binds_facts_and_intent_to_one_account() {
    let a = Annotations::open_memory_for("u-1").unwrap();
    // A facts file with no recorded account cannot bind intent.
    let s = Store::open_memory().unwrap();
    let err = s.refresh_snapshot("pc", "Standard", &a).unwrap_err();
    assert!(err.to_string().contains("no account identity"), "{err:#}");
    // A handle bound to another account's uuid is refused; a handle
    // never bound at all (raw open, no stored identity) is refused
    // too — never trusted to the caller.
    let s = store(); // records uuid u-1
    let other = Annotations::open_memory_for("u-2").unwrap();
    let err = s.refresh_snapshot("pc", "Standard", &other).unwrap_err();
    assert!(err.to_string().contains("u-2"), "{err:#}");
    let unbound = Annotations::open_memory().unwrap();
    let err = s.refresh_snapshot("pc", "Standard", &unbound).unwrap_err();
    assert!(err.to_string().contains("no account identity"), "{err:#}");
    // The account's own file is accepted — including when reopened
    // from its raw path, because the uuid lives inside the file.
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    drop(Annotations::open_for(&dir, "u-1").unwrap());
    let reopened = Annotations::open(&annotations_path(&dir, "u-1")).unwrap();
    assert_eq!(reopened.uuid(), Some("u-1"));
    let snap = s.refresh_snapshot("pc", "Standard", &reopened).unwrap();
    assert_eq!(snap.account_uuid, "u-1");
    // A copied/renamed file keeps its owner: u-2's database placed at
    // u-1's path still says u-2 and is refused.
    drop(Annotations::open_for(&dir, "u-2").unwrap());
    let dir2 = dir.join("elsewhere");
    std::fs::create_dir_all(&dir2).unwrap();
    std::fs::copy(
        annotations_path(&dir, "u-2"),
        annotations_path(&dir2, "u-1"),
    )
    .unwrap();
    let copied = Annotations::open(&annotations_path(&dir2, "u-1")).unwrap();
    assert_eq!(copied.uuid(), Some("u-2"));
    let err = s.refresh_snapshot("pc", "Standard", &copied).unwrap_err();
    assert!(err.to_string().contains("u-2"), "{err:#}");
}

/// The basis and the tab set are per (realm, league): a pre-realm
/// listing row (no realm in its params) answers for pc, and the
/// console realm's listing is its own.
#[test]
fn the_basis_and_the_tab_set_are_per_realm() {
    let mut s = store();
    let a = Annotations::open_memory_for("u-1").unwrap();
    s.record(
            &listing_ep(),
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": [ { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 } ] }),
            100,
        )
        .unwrap();
    s.record(
            &Endpoint::Stashes {
                realm: "xbox".into(),
                league: "Standard".into(),
            },
            &json!({ "realm": "xbox", "league": "Standard" }),
            200,
            &json!({ "stashes": [ { "id": "t1", "name": "Console", "type": "PremiumStash", "index": 0 } ] }),
            200,
        )
        .unwrap();
    let pc = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    let xbox = s.refresh_snapshot("xbox", "Standard", &a).unwrap();
    assert_eq!(
        (pc.realm.as_str(), pc.stash_listing.unwrap().fetched_at),
        ("pc", 100)
    );
    assert_eq!(
        (xbox.realm.as_str(), xbox.stash_listing.unwrap().fetched_at),
        ("xbox", 200)
    );
    assert_eq!(pc.tabs[0].name, "One");
    assert_eq!(xbox.tabs[0].name, "Console");
    assert!(
        s.refresh_snapshot("sony", "Standard", &a)
            .unwrap()
            .stash_listing
            .is_none()
    );
}

fn characters_ep(realm: &str) -> Endpoint {
    Endpoint::Characters {
        realm: realm.into(),
    }
}

fn list_characters(s: &mut Store, realm: &str, entries: Value, at: i64) {
    s.record(
        &characters_ep(realm),
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

/// The character basis is the realm's latest listing; the rows are
/// this league's plus the realm's league-less ones (no (realm, league)
/// policy can cover those, so every league plan of the realm sees
/// them); another realm's characters and basis are its own.
#[test]
fn the_character_basis_is_per_realm_and_the_rows_are_per_league_plus_league_less() {
    let mut s = store();
    let a = Annotations::open_memory_for("u-1").unwrap();
    list_characters(
        &mut s,
        "pc",
        json!([
            { "id": "c-std", "name": "Std", "league": "Standard", "level": 90, "experience": 100 },
            { "id": "c-hc", "name": "Hc", "league": "Hardcore", "level": 80 },
            { "id": "c-none", "name": "Nowhere", "level": 1 },
        ]),
        100,
    );
    list_characters(
        &mut s,
        "poe2",
        json!([ { "id": "c-p2", "name": "Second", "league": "Standard", "level": 41 } ]),
        200,
    );
    let std = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    let basis = std.character_listing.unwrap();
    assert_eq!((basis.fetched_at, std.stash_listing), (100, None));
    assert_eq!(
        std.characters
            .iter()
            .map(|c| c.id.as_str())
            .collect::<Vec<_>>(),
        vec!["c-none", "c-std"]
    );
    let c = std.characters.iter().find(|c| c.id == "c-std").unwrap();
    assert_eq!(
        (
            c.name.as_str(),
            c.league.as_deref(),
            c.listed_at,
            c.fetched_at
        ),
        ("Std", Some("Standard"), Some(100), None)
    );
    assert_eq!(c.listed_response, Some(basis.response_id));
    assert_eq!(c.listed["experience"], 100);
    assert_eq!(c.fetched, Value::Null);
    assert_eq!(
        std.characters
            .iter()
            .find(|c| c.id == "c-none")
            .unwrap()
            .league,
        None
    );
    let hc = s.refresh_snapshot("pc", "Hardcore", &a).unwrap();
    assert_eq!(
        hc.characters
            .iter()
            .map(|c| c.id.as_str())
            .collect::<Vec<_>>(),
        vec!["c-none", "c-hc"]
    );
    assert_eq!(hc.character_listing, Some(basis));
    let poe2 = s.refresh_snapshot("poe2", "Standard", &a).unwrap();
    assert_eq!(poe2.character_listing.unwrap().fetched_at, 200);
    assert_eq!(
        poe2.characters
            .iter()
            .map(|c| c.id.as_str())
            .collect::<Vec<_>>(),
        vec!["c-p2"]
    );
    let xbox = s.refresh_snapshot("xbox", "Standard", &a).unwrap();
    assert!(xbox.character_listing.is_none() && xbox.characters.is_empty());
}

/// A fetch fills `fetched` and never touches the listed entry; a drop
/// and revival by two listings (no fetch between) leaves the row
/// never-fetched with no body to read — the store's liveness rule,
/// visible to the planner as exactly three facts.
#[test]
fn a_character_fetch_fills_the_body_and_revival_clears_it() {
    let mut s = store();
    let a = Annotations::open_memory_for("u-1").unwrap();
    let hero = json!([{ "id": "c1", "name": "Hero", "league": "Standard", "level": 90, "experience": 100 }]);
    list_characters(&mut s, "pc", hero.clone(), 100);
    fetch_character(
        &mut s,
        "pc",
        json!({ "id": "c1", "name": "Hero", "league": "Standard", "level": 90, "experience": 150,
                    "inventory": [ item("i1") ] }),
        110,
    );
    let snap = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    let c = &snap.characters[0];
    assert_eq!(c.fetched_at, Some(110));
    assert_eq!(
        (&c.listed["experience"], &c.fetched["experience"]),
        (&json!(100), &json!(150))
    );
    assert_eq!(c.fetched["_split"]["inventory"], 1);
    assert!(
        c.fetched.get("inventory").is_none(),
        "the envelope, not the body"
    );
    // Dropped, then revived: never fetched, and no body offered.
    list_characters(&mut s, "pc", json!([]), 120);
    assert!(
        s.refresh_snapshot("pc", "Standard", &a)
            .unwrap()
            .characters
            .is_empty()
    );
    list_characters(&mut s, "pc", hero, 130);
    let snap = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    let c = &snap.characters[0];
    assert_eq!((c.fetched_at, &c.fetched), (None, &Value::Null));
    assert_eq!(c.listed_at, Some(130));
    assert_eq!(
        c.listed_response,
        Some(snap.character_listing.unwrap().response_id)
    );
}

/// Two character listings in one second: membership is per response,
/// and the rows are stamped to the basis the snapshot cites.
#[test]
fn two_character_listings_in_one_second_still_retire_dropped_characters() {
    let mut s = store();
    let a = Annotations::open_memory_for("u-1").unwrap();
    list_characters(
        &mut s,
        "pc",
        json!([
            { "id": "c1", "name": "One", "league": "Standard" },
            { "id": "c2", "name": "Two", "league": "Standard" },
        ]),
        100,
    );
    list_characters(
        &mut s,
        "pc",
        json!([ { "id": "c1", "name": "One", "league": "Standard" } ]),
        100,
    );
    let snap = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    let basis = snap.character_listing.unwrap();
    assert_eq!(
        snap.characters
            .iter()
            .map(|c| c.id.as_str())
            .collect::<Vec<_>>(),
        vec!["c1"]
    );
    assert_eq!(snap.characters[0].listed_response, Some(basis.response_id));
}

#[test]
fn malformed_stored_character_json_is_an_error_with_the_address() {
    let mut s = store();
    let a = Annotations::open_memory_for("u-1").unwrap();
    list_characters(
        &mut s,
        "pc",
        json!([ { "id": "c1", "name": "One", "league": "Standard" } ]),
        100,
    );
    s.conn
        .execute("UPDATE characters SET listed_json = 'not json'", [])
        .unwrap();
    let err = s.refresh_snapshot("pc", "Standard", &a).unwrap_err();
    assert!(err.to_string().contains("pc/c1"), "{err:#}");
}

/// C64 — the pricing snapshot is neutral: every live item at the
/// league's live locations with its note verbatim (a stash item's, a
/// character item's, a socketed gem's), removed items and other
/// leagues' items left out, every `buyout` row raw whatever its scope
/// and realm, other kinds left out. Nothing here says what a note
/// means or which row covers what.
#[test]
fn the_pricing_snapshot_carries_notes_and_buyout_rows_verbatim_and_live_items_only() {
    let mut s = store();
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    s.record(
            &listing_ep(),
            &json!({}),
            200,
            &json!({ "stashes": [
                { "id": "f1", "name": "Folder", "type": "Folder", "index": 0,
                  "children": [ { "id": "c1", "name": "~price 3 chaos", "type": "PremiumStash", "index": 1, "metadata": { "public": true } } ] },
                { "id": "m1", "name": "Maps", "type": "MapStash", "index": 2 } ] }),
            100,
        )
        .unwrap();
    let mut gem_holder = item("i-armour");
    gem_holder["socketedItems"] = json!([ { "id": "i-gem", "name": "", "typeLine": "Fireball", "baseType": "Fireball", "note": "~b/o 2 divine" } ]);
    let mut noted = item("i-noted");
    noted["note"] = json!("~price 5 chaos");
    noted["stackSize"] = json!(20);
    // a note that is no string: unread at the note, never a failed
    // read; an object is one too (SQLite's extract flattens it to text)
    let mut odd = item("i-odd");
    odd["note"] = json!(5);
    odd["inventoryId"] = json!(5);
    let mut objnote = item("i-objnote");
    objnote["note"] = json!({ "x": 1 });
    s.record(
            &stash_ep("c1", None),
            &json!({}),
            200,
            &json!({ "stash": { "id": "c1", "name": "~price 3 chaos", "type": "PremiumStash",
                                "items": [ noted.clone(), item("i-plain"), gem_holder.clone(), item("i-gone"), odd.clone(), objnote.clone() ] } }),
            110,
        )
        .unwrap();
    // A later fetch without `i-gone` removes it; it leaves the snapshot.
    s.record(
        &stash_ep("c1", None),
        &json!({}),
        200,
        &json!({ "stash": { "id": "c1", "name": "~price 3 chaos", "type": "PremiumStash",
                                "items": [ noted, item("i-plain"), gem_holder, odd, objnote ] } }),
        120,
    )
    .unwrap();
    list_characters(
        &mut s,
        "pc",
        json!([ { "id": "ch-1", "name": "Exile", "league": "Standard" },
                    { "id": "ch-2", "name": "Elsewhere", "league": "Hardcore" } ]),
        130,
    );
    let mut worn = item("i-worn");
    worn["note"] = json!("~price 2222 jewellers");
    worn["inventoryId"] = json!("BodyArmour");
    fetch_character(
        &mut s,
        "pc",
        json!({ "id": "ch-1", "name": "Exile", "league": "Standard", "equipment": [ worn ], "inventory": [] }),
        140,
    );
    fetch_character(
        &mut s,
        "pc",
        json!({ "id": "ch-2", "name": "Elsewhere", "league": "Hardcore", "equipment": [ item("i-hc") ], "inventory": [] }),
        141,
    );
    use crate::annotations::test_kinds::{Buyout, Note, via_test};
    let value = json!({ "version": 1, "type": "ignore" });
    for (scope, key) in [
        ("item", "i-plain"),
        ("tab", "pc/c1"),
        ("substash", "pc/m1/s1"),
        ("character", "ch-1"),
        ("tab", "xbox/c1"),
    ] {
        a.put::<Buyout>(scope, key, &value, None, &via_test())
            .unwrap();
    }
    a.put::<Note>(
        "item",
        "i-plain",
        &json!({ "version": 1 }),
        None,
        &via_test(),
    )
    .unwrap();

    let snap = s.pricing_snapshot("pc", "Standard", &a).unwrap();
    assert_eq!(snap.account_uuid, "u-1");
    assert_eq!(
        snap.tabs.iter().map(|t| t.id.as_str()).collect::<Vec<_>>(),
        ["f1", "c1", "m1"]
    );
    assert_eq!(snap.tabs[1].metadata, json!({ "public": true }));
    assert_eq!(
        snap.characters
            .iter()
            .map(|c| c.id.as_str())
            .collect::<Vec<_>>(),
        ["ch-1"]
    );
    let notes: Vec<(&str, &str, Option<&str>, bool)> = snap
        .items
        .iter()
        .map(|i| {
            (
                i.location_id.as_str(),
                i.id.as_str(),
                i.note.as_deref(),
                i.note_unread,
            )
        })
        .collect();
    assert_eq!(
        notes,
        [
            ("ch-1", "i-worn", Some("~price 2222 jewellers"), false),
            ("c1", "i-armour", None, false),
            ("c1", "i-noted", Some("~price 5 chaos"), false),
            ("c1", "i-objnote", None, true),
            ("c1", "i-odd", None, true),
            ("c1", "i-plain", None, false),
            ("c1", "i-gem", Some("~b/o 2 divine"), false),
        ]
    );
    let odd = snap.items.iter().find(|i| i.id == "i-odd").unwrap();
    assert!(odd.inventory_id.is_none() && odd.inventory_id_unread);
    let gem = snap.items.iter().find(|i| i.id == "i-gem").unwrap();
    assert_eq!(gem.socketed_in.as_deref(), Some("i-armour"));
    assert_eq!((gem.x, gem.y), (None, None));
    let noted = snap.items.iter().find(|i| i.id == "i-noted").unwrap();
    assert_eq!(noted.stack_size, Some(20));
    assert_eq!(noted.seen_response, Some(4)); // profile, listing, fetch, fetch
    assert_eq!(noted.container.as_deref(), Some("items"));
    let worn = snap.items.iter().find(|i| i.id == "i-worn").unwrap();
    assert_eq!(worn.container.as_deref(), Some("equipment"));
    // The slot a forum link names, verbatim; a socketed gem has none.
    assert_eq!(worn.inventory_id.as_deref(), Some("BodyArmour"));
    assert_eq!(gem.inventory_id, None);
    // Every buyout row, raw, whatever its scope or realm; the note
    // kind is not a price.
    let rows: Vec<(&str, &str)> = snap
        .buyouts
        .iter()
        .map(|r| (r.scope.as_str(), r.key.as_str()))
        .collect();
    assert_eq!(
        rows,
        [
            ("character", "ch-1"),
            ("item", "i-plain"),
            ("substash", "pc/m1/s1"),
            ("tab", "pc/c1"),
            ("tab", "xbox/c1"),
        ]
    );
    assert!(
        snap.buyouts
            .iter()
            .all(|r| r.kind == "buyout" && r.value == value)
    );
    assert_eq!(snap.buyouts[0].written_via, "test");
    // The same pairing rule as the refresh snapshot.
    let other = Annotations::open_memory_for("u-2").unwrap();
    let err = s.pricing_snapshot("pc", "Standard", &other).unwrap_err();
    assert!(err.to_string().contains("u-2"), "{err:#}");
    // Another league is another snapshot: its character and item, no
    // tabs, the same intent rows.
    let hc = s.pricing_snapshot("pc", "Hardcore", &a).unwrap();
    assert!(hc.tabs.is_empty());
    assert_eq!(hc.characters[0].id, "ch-2");
    assert_eq!(
        hc.items.iter().map(|i| i.id.as_str()).collect::<Vec<_>>(),
        ["i-hc"]
    );
    assert_eq!(hc.buyouts.len(), 5);
}
