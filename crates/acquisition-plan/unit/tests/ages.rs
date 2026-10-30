use super::*;

/// C53 (an age the text prints stands in words beside its epoch,
/// made at the answer's own time): a league note says the list's
/// fetch, `never` where none was fetched, and a fetch that spent a
/// plan's request says its own, each at the time given.
#[test]
fn c53_a_note_and_a_spent_fetch_say_their_epochs_as_ages() {
    let now = 1_000 + 2 * 86_400;
    let list = LeagueList {
        fetched_at: 1_000,
        ids: vec!["Standard".into()],
    };
    let notes = league_notes(["Mirage", "Standard"], Some(&list), now);
    assert_eq!(notes.len(), 1, "{notes:?}");
    let note = serde_json::to_value(&notes[0]).unwrap();
    assert_eq!(note["list_fetched_at"], 1_000);
    assert_eq!(note["list_fetched_at_age"], "2d ago", "{note}");
    let unread = league_notes(["Mirage"], None, now);
    assert_eq!(unread[0].list_fetched_at_age, "never");

    let spent = Spent {
        action: RefreshAction::ListStashes {
            realm: Realm::Pc,
            league: "Standard".into(),
            reason: ListingReason::NeverListed,
        },
        by: SpentBy::Fetched {
            fetched_at: now - 30,
            fetched_at_age: None,
        },
    };
    let unaged = serde_json::to_value(&spent).unwrap();
    assert!(unaged.get("fetched_at_age").is_none(), "{unaged}");
    let aged = serde_json::to_value(spent.aged(now)).unwrap();
    assert_eq!(aged["fetched_at_age"], "30s ago", "{aged}");
}

/// C53: a plan's note that the realm's character listing names no
/// character in the league says when that listing was fetched, in
/// words at the time given, as the note's text does.
#[test]
fn c53_the_no_characters_note_says_its_listings_age() {
    let mut s = acquisition_store::Store::open_memory().unwrap();
    s.record(
        &acquisition_store::Endpoint::Profile,
        &serde_json::json!({}),
        200,
        &serde_json::json!({ "uuid": "u-1", "name": "tom" }),
        1,
    )
    .unwrap();
    s.record(
        &acquisition_store::Endpoint::Characters { realm: "pc".into() },
        &serde_json::json!({}),
        200,
        &serde_json::json!({ "characters": [
                { "id": "c1", "name": "Elsewhere", "league": "Mirage" } ] }),
        100,
    )
    .unwrap();
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    a.put::<SyncPolicy>(
        SYNC_POLICY_SCOPE,
        SYNC_POLICY_KEY,
        &serde_json::json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard":
                { "tabs": "all", "characters": "all", "max_age_seconds": 3600 } } } } }),
        None,
        &Provenance::via("test"),
    )
    .unwrap();
    let snapshot = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    let notes = plan_notes(&snapshot, 100 + 5 * 3_600);
    assert!(notes.no_characters_listed, "{notes:?}");
    assert_eq!(
        notes.character_listing_fetched_at_age.as_deref(),
        Some("5h ago")
    );
}
