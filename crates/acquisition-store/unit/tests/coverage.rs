use super::*;

const PC: &str = "pc";

fn tab(league: &str, id: &str, tab_type: &str, fetched_at: Option<i64>) -> LocationRow {
    LocationRow {
        realm: PC.into(),
        league: Some(league.into()),
        kind: "stash".into(),
        id: id.into(),
        parent: None,
        name: format!("tab {id}"),
        tab_type: Some(tab_type.into()),
        listed_at: Some(100),
        fetched_at,
        removed_at: None,
    }
}

fn character(league: Option<&str>, id: &str, fetched_at: Option<i64>) -> LocationRow {
    LocationRow {
        realm: PC.into(),
        league: league.map(Into::into),
        kind: "character".into(),
        id: id.into(),
        parent: None,
        name: format!("char {id}"),
        tab_type: None,
        listed_at: Some(100),
        fetched_at,
        removed_at: None,
    }
}

fn listing(realm: &str, league: Option<&str>, fetched_at: i64) -> ListingSeen {
    ListingSeen {
        realm: realm.into(),
        league: league.map(Into::into),
        kind: if league.is_some() {
            "stash"
        } else {
            "character"
        }
        .into(),
        response_id: fetched_at,
        fetched_at,
    }
}

/// The places with their counts, every count given.
fn places(rows: &[(LocationRow, i64)]) -> Vec<(&LocationRow, i64)> {
    rows.iter().map(|(l, n)| (l, *n)).collect()
}

fn over(rows: &[(LocationRow, i64)]) -> Coverage {
    of(RealmScope::All, &places(rows), &[], &[PC.to_string()], 0)
}

fn location(id: &str) -> Option<String> {
    Some(id.to_string())
}

#[test]
fn c162_an_empty_set_has_no_fetch_and_no_list() {
    let coverage = of(RealmScope::All, &[], &[], &[], 0);
    assert_eq!(
        coverage,
        Coverage {
            items: 0,
            removed: None,
            realms_held: vec![],
            leagues: vec![],
            fetched: 0,
            never_fetched: 0,
            never_fetched_by_type: vec![],
            never_fetched_leagues: vec![],
            folders: 0,
            oldest_fetch: None,
            oldest_fetch_age: None,
            oldest_fetch_location: None,
            newest_fetch: None,
            newest_fetch_age: None,
            newest_fetch_location: None,
            list_seen: None,
            list_seen_age: None,
        }
    );
}

#[test]
fn c162_one_never_fetched_tab_is_its_type_never_fetched() {
    let coverage = over(&[(tab("Std", "a", "PremiumStash", None), 0)]);
    assert_eq!((coverage.fetched, coverage.never_fetched), (0, 1));
    assert_eq!(
        coverage.never_fetched_by_type,
        vec![TabType {
            kind: "PremiumStash".into(),
            tabs: 1,
            fetched: 0,
            items: 0,
        }]
    );
    assert_eq!(coverage.never_fetched_leagues, vec!["Std".to_string()]);
    assert_eq!(coverage.oldest_fetch, None);
}

#[test]
fn c162_a_folder_is_counted_apart_in_no_type_and_never_a_fetch() {
    let coverage = over(&[
        (tab("Std", "f", "Folder", None), 0),
        (tab("Std", "a", "NormalStash", Some(50)), 2),
    ]);
    assert_eq!(coverage.folders, 1);
    assert_eq!((coverage.fetched, coverage.never_fetched), (1, 0));
    assert!(coverage.never_fetched_by_type.is_empty());
    assert!(coverage.never_fetched_leagues.is_empty());
    // a folder fetched in error is still never a fetch
    let fetched_folder = over(&[(tab("Std", "f", "Folder", Some(10)), 0)]);
    assert_eq!(fetched_folder.oldest_fetch, None);
    assert_eq!(fetched_folder.fetched, 0);
}

#[test]
fn c162_on_a_tie_the_oldest_and_the_newest_are_the_first_in_the_order_given() {
    let a = tab("Std", "a", "NormalStash", Some(70));
    let b = tab("Std", "b", "NormalStash", Some(70));
    let forward = over(&[(a.clone(), 1), (b.clone(), 1)]);
    let at = |l: &Option<FetchedLocation>| l.as_ref().map(|l| l.id.clone());
    assert_eq!(forward.oldest_fetch, Some(70));
    assert_eq!(forward.newest_fetch, Some(70));
    assert_eq!(at(&forward.oldest_fetch_location), location("a"));
    assert_eq!(at(&forward.newest_fetch_location), location("a"));
    // the same places in the other order: the choice follows the order
    let reversed = over(&[(b, 1), (a, 1)]);
    assert_eq!(at(&reversed.oldest_fetch_location), location("b"));
    assert_eq!(at(&reversed.newest_fetch_location), location("b"));
}

#[test]
fn c162_a_type_whose_every_location_is_fetched_is_not_listed_never_fetched() {
    let coverage = over(&[
        (tab("Std", "a", "NormalStash", Some(10)), 3),
        (tab("Std", "b", "NormalStash", Some(20)), 4),
        (tab("Std", "c", "MapStash", None), 0),
    ]);
    let kinds: Vec<&str> = coverage
        .never_fetched_by_type
        .iter()
        .map(|t| t.kind.as_str())
        .collect();
    assert_eq!(kinds, vec!["MapStash"]);
}

#[test]
fn c162_a_character_is_its_own_type_and_no_leagues_never_fetched() {
    let coverage = over(&[
        (character(Some("Hc"), "x", Some(40)), 5),
        (character(Some("Hc"), "y", None), 0),
    ]);
    assert_eq!(coverage.leagues, vec!["Hc".to_string()]);
    assert_eq!(
        coverage.never_fetched_by_type,
        vec![TabType {
            kind: "character".into(),
            tabs: 2,
            fetched: 1,
            items: 5,
        }]
    );
    // a never-fetched character counts into no league's never fetched
    assert!(coverage.never_fetched_leagues.is_empty());
    assert_eq!(
        coverage.oldest_fetch_location,
        Some(FetchedLocation {
            kind: "character".into(),
            id: "x".into(),
            name: Some("char x".into()),
            league: Some("Hc".into()),
        })
    );
}

#[test]
fn c162_a_substash_is_a_location_of_its_own_with_its_own_fetch() {
    let parent = tab("Std", "p", "UniqueStash", Some(30));
    let substash = LocationRow {
        parent: Some("p".into()),
        ..tab("Std", "p-1", "UniqueStash", Some(80))
    };
    let coverage = over(&[(parent, 0), (substash, 2)]);
    assert_eq!((coverage.fetched, coverage.never_fetched), (2, 0));
    assert_eq!(coverage.oldest_fetch_location.map(|l| l.id), location("p"));
    assert_eq!(coverage.newest_fetch, Some(80));
    assert_eq!(
        coverage.newest_fetch_location.map(|l| l.id),
        location("p-1")
    );
}

#[test]
fn c162_leagues_are_every_locations_sorted_once_folders_included() {
    let coverage = over(&[
        (tab("Std", "a", "NormalStash", Some(1)), 0),
        (tab("Hc", "b", "NormalStash", Some(1)), 0),
        (tab("Std", "c", "NormalStash", None), 0),
        (tab("Abyss", "f", "Folder", None), 0),
        (character(None, "x", None), 0),
    ]);
    assert_eq!(coverage.leagues, vec!["Abyss", "Hc", "Std"]);
}

#[test]
fn c162_a_blank_name_is_no_name() {
    let blank = LocationRow {
        name: "  ".into(),
        ..tab("Std", "a", "NormalStash", Some(5))
    };
    let coverage = over(&[(blank, 0)]);
    assert_eq!(
        coverage.oldest_fetch_location,
        Some(FetchedLocation {
            kind: "stash".into(),
            id: "a".into(),
            name: None,
            league: Some("Std".into()),
        })
    );
}

#[test]
fn c162_lists_seen_are_the_scopes_realm_or_every_realms() {
    // pc holds the locations; xbox holds only a listing, older
    let rows = [(tab("Std", "a", "NormalStash", Some(50)), 1)];
    let listings = [listing("pc", Some("Std"), 300), listing("xbox", None, 200)];
    let realms = ["pc".to_string(), "xbox".to_string()];
    let one = of(RealmScope::One("pc"), &places(&rows), &listings, &realms, 1);
    assert_eq!(one.list_seen, Some(300));
    let all = of(RealmScope::All, &places(&rows), &listings, &realms, 1);
    assert_eq!(all.list_seen, Some(200));
    // the oldest of the realm's lists
    let two = [listing("pc", Some("Std"), 300), listing("pc", None, 250)];
    let pc = of(RealmScope::One("pc"), &places(&rows), &two, &realms, 1);
    assert_eq!(pc.list_seen, Some(250));
}

#[test]
fn c162_the_most_never_fetched_come_first_ties_by_name() {
    let coverage = over(&[
        (tab("Zed", "z1", "MapStash", None), 0),
        (tab("Zed", "z2", "MapStash", None), 0),
        (tab("Alpha", "a1", "CurrencyStash", None), 0),
        (tab("Beta", "b1", "BlightStash", None), 0),
        (tab("Beta", "b2", "NormalStash", Some(5)), 9),
    ]);
    let kinds: Vec<&str> = coverage
        .never_fetched_by_type
        .iter()
        .map(|t| t.kind.as_str())
        .collect();
    assert_eq!(kinds, vec!["MapStash", "BlightStash", "CurrencyStash"]);
    assert_eq!(coverage.never_fetched_leagues, vec!["Zed", "Alpha", "Beta"]);
}

#[test]
fn c162_a_stash_location_with_no_type_counts_under_the_empty_type() {
    let untyped = LocationRow {
        tab_type: None,
        ..tab("Std", "a", "", None)
    };
    let coverage = over(&[(untyped, 0)]);
    assert_eq!(
        coverage.never_fetched_by_type,
        vec![TabType {
            kind: String::new(),
            tabs: 1,
            fetched: 0,
            items: 0,
        }]
    );
}

#[test]
fn c162_a_partly_fetched_type_is_listed_with_its_every_locations_items() {
    let coverage = over(&[
        (tab("Std", "a", "QuadStash", Some(10)), 7),
        (tab("Std", "b", "QuadStash", Some(20)), 5),
        (tab("Std", "c", "QuadStash", None), 0),
    ]);
    assert_eq!(
        coverage.never_fetched_by_type,
        vec![TabType {
            kind: "QuadStash".into(),
            tabs: 3,
            fetched: 2,
            items: 12,
        }]
    );
}

#[test]
fn c162_the_item_total_and_the_realms_held_are_passed_through() {
    let rows = [(tab("Std", "a", "NormalStash", Some(5)), 2)];
    let realms = ["pc".to_string(), "sony".to_string()];
    let coverage = of(RealmScope::One("pc"), &places(&rows), &[], &realms, 40);
    assert_eq!(coverage.items, 40);
    assert_eq!(coverage.realms_held, vec!["pc", "sony"]);
    assert_eq!(coverage.removed, None);
}

#[test]
fn c162_by_type_counts_the_most_items_first_ties_by_name() {
    let counted = by_type([
        ("NormalStash", true, 3),
        ("Folder", false, 0),
        ("MapStash", true, 10),
        ("CurrencyStash", false, 0),
        ("NormalStash", false, 0),
        ("Bags", true, 3),
    ]);
    let rows: Vec<(&str, usize, usize, i64)> = counted
        .iter()
        .map(|t| (t.kind.as_str(), t.tabs, t.fetched, t.items))
        .collect();
    assert_eq!(
        rows,
        vec![
            ("MapStash", 1, 1, 10),
            ("Bags", 1, 1, 3),
            ("NormalStash", 2, 1, 3),
            ("CurrencyStash", 1, 0, 0),
            // a folder given is a type like any other
            ("Folder", 1, 0, 0),
        ]
    );
}

#[test]
fn c53_coverage_ages_its_fetches_and_lists_where_there_are_any() {
    let now = 10_000;
    let mut none = of(RealmScope::All, &[], &[], &[], 0);
    none.aged(now);
    assert_eq!(none.oldest_fetch_age, None);
    assert_eq!(none.newest_fetch_age, None);
    assert_eq!(none.list_seen_age.as_deref(), Some("never"));

    let rows = [
        (tab("Std", "a", "NormalStash", Some(now - 30)), 1),
        (tab("Std", "b", "NormalStash", Some(now - 7_200)), 1),
    ];
    let listings = [listing("pc", Some("Std"), now - 3 * 86_400)];
    let mut some = of(RealmScope::All, &places(&rows), &listings, &[], 2);
    some.aged(now);
    assert_eq!(some.oldest_fetch_age.as_deref(), Some("2h ago"));
    assert_eq!(some.newest_fetch_age.as_deref(), Some("30s ago"));
    assert_eq!(some.list_seen_age.as_deref(), Some("3d ago"));
}
