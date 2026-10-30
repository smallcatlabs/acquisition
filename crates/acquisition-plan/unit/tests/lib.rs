use super::*;
use acquisition_store::{Annotations, Endpoint, SYNC_POLICY_KIND, Store};

fn item(id: &str) -> Value {
    json!({ "id": id, "name": "Foo", "typeLine": "Imperial Bow", "baseType": "Imperial Bow", "x": 0, "y": 0 })
}

/// A store with its account identity on record, as every real store's
/// is after one login.
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

fn list(s: &mut Store, stashes: Value, at: i64) {
    s.record(
        &Endpoint::Stashes {
            realm: "pc".into(),
            league: "Standard".into(),
        },
        &json!({ "league": "Standard" }),
        200,
        &json!({ "stashes": stashes }),
        at,
    )
    .unwrap();
}

fn fetch(s: &mut Store, stash: Value, at: i64) {
    let id = stash["id"].as_str().unwrap().to_string();
    s.record(
        &Endpoint::Stash {
            realm: "pc".into(),
            league: "Standard".into(),
            id,
            sub: None,
        },
        &json!({ "league": "Standard" }),
        200,
        &json!({ "stash": stash }),
        at,
    )
    .unwrap();
}

fn policy(value: Value) -> SyncPolicy {
    SyncPolicy::from_value(&value).unwrap()
}

fn all_policy_value(max_age_seconds: u32) -> Value {
    json!({
        "version": 1,
        "leagues": { "Standard": { "tabs": "all", "max_age_seconds": max_age_seconds } }
    })
}

fn snapshot(s: &Store) -> RefreshSnapshot {
    let a = Annotations::open_memory_for("u-1").unwrap();
    s.refresh_snapshot("pc", "Standard", &a).unwrap()
}

/// Snapshot with `policy` installed as the stored sync-policy row —
/// the only way plans are made: from stored intent, at its revision.
fn snapshot_with(s: &Store, policy: &Value) -> RefreshSnapshot {
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    store_policy(&mut a, policy);
    s.refresh_snapshot("pc", "Standard", &a).unwrap()
}

/// The provenance every test write carries.
fn via_test() -> Provenance {
    Provenance::via("test")
}

/// Land `policy` as the stored sync-policy row through the typed door
/// — the only door there is.
fn store_policy(a: &mut Annotations, policy: &Value) -> AnnotationRow {
    a.put::<SyncPolicy>(
        SYNC_POLICY_SCOPE,
        SYNC_POLICY_KEY,
        policy,
        None,
        &via_test(),
    )
    .unwrap()
}

fn plan(s: &Store, policy: &Value, now: i64) -> RefreshPlan {
    plan_refresh("mock", &snapshot_with(s, policy), now).unwrap()
}

/// C36 — the sync policy is a per-account, inspectable declaration; a value that half-parses never lands (strict per stamped shape).
#[test]
fn the_policy_parses_strictly_and_versions_are_refused_not_guessed() {
    // The valid shapes: "all" and an explicit id list.
    let p = policy(json!({
        "version": 1,
        "leagues": {
            "Standard": { "tabs": "all", "max_age_seconds": 3600 },
            "Hardcore": { "tabs": ["t1", "s1"], "max_age_seconds": 60 },
        }
    }));
    // A v1 policy reads as v2 on realm pc — the only realm it could
    // have meant — while the stored value stays what was written.
    assert_eq!(p.version, SYNC_POLICY_VERSION);
    let pc = &p.realms[&Realm::Pc].leagues;
    assert_eq!(pc["Standard"].tabs, Some(Selection::All));
    assert_eq!(
        pc["Hardcore"].tabs,
        Some(Selection::Ids(vec!["t1".into(), "s1".into()]))
    );
    // The v2 shape: leagues under realms.
    let v2 = policy(json!({
        "version": 2,
        "realms": {
            "pc": { "leagues": { "Standard": { "tabs": "all", "max_age_seconds": 3600 } } },
            "xbox": { "leagues": { "Standard": { "tabs": ["x1"], "max_age_seconds": 60 } } },
        }
    }));
    assert_eq!(
        v2.realms[&Realm::Xbox].leagues["Standard"].tabs,
        Some(Selection::Ids(vec!["x1".into()]))
    );
    // Tabs under poe2 name a URL shape the stash endpoints do not
    // have (PoE1 only): a parse error, never a request.
    let err = SyncPolicy::from_value(&json!({
            "version": 2,
            "realms": { "poe2": { "leagues": { "Standard": { "tabs": "all", "max_age_seconds": 60 } } } }
        }))
        .unwrap_err();
    assert!(err.to_string().contains("poe2"), "{err}");
    // An unknown realm key, a v1 stamp on a v2 body, and a stray
    // top-level field under either stamp are all malformed — the
    // strict parse is per stamped shape, top level included.
    for bad in [
        json!({ "version": 2, "realms": { "ps5": { "leagues": {} } } }),
        json!({ "version": 1, "realms": { "pc": { "leagues": {} } } }),
        json!({ "version": 2, "leagues": {} }),
        json!({ "version": 1, "leagues": {}, "typo": true }),
        json!({ "version": 2, "realms": {}, "leagues": {} }),
    ] {
        let err = SyncPolicy::from_value(&bad).unwrap_err();
        assert!(
            matches!(err, PlanError::MalformedPolicy { .. }),
            "{bad}: {err}"
        );
    }
    // A typo'd field is malformed, never silently ignored — the policy
    // is intent, and intent that half-parses is worse than an error.
    let err = SyncPolicy::from_value(&json!({
        "version": 1,
        "leagues": { "Standard": { "tabs": "all", "max_age_secs": 60 } }
    }))
    .unwrap_err();
    assert!(matches!(err, PlanError::MalformedPolicy { .. }), "{err}");
    // So is a selection word that isn't "all", and a missing version.
    let err = SyncPolicy::from_value(&json!({
        "version": 1,
        "leagues": { "Standard": { "tabs": "everything", "max_age_seconds": 60 } }
    }))
    .unwrap_err();
    assert!(matches!(err, PlanError::MalformedPolicy { .. }), "{err}");
    let err = SyncPolicy::from_value(&json!({ "leagues": {} })).unwrap_err();
    assert!(matches!(err, PlanError::MalformedPolicy { .. }), "{err}");
    // A newer version is refused as such — checked before the shape,
    // so a v3 policy with v3 fields is not misreported as a typo.
    let err = SyncPolicy::from_value(&json!({
        "version": 4,
        "realms": {},
        "some_v4_field": true
    }))
    .unwrap_err();
    assert_eq!(
        err,
        PlanError::PolicyVersionUnsupported {
            found: 4,
            supported: 3
        }
    );
    // The policy round-trips (in its v2 form): what a frontend writes
    // is inspectable.
    let back: SyncPolicy = serde_json::from_value(serde_json::to_value(&p).unwrap()).unwrap();
    assert_eq!(back, p);
}

/// C66 — the policy through the store's generic: a v3 value must
/// re-serialize to exactly what was read, so the spellings the parse
/// normalizes away (`tabs: []` beside a facet that names work,
/// `characters: null`) are refused naming the path, never stored as a
/// second spelling of one intent; older stamps upgrade in memory and
/// are not held to it. The owner's stored row (2026-09-05: both
/// facets, an id list, v3) is canonical and unaffected.
#[test]
fn c66_a_current_schema_policy_must_be_canonical_and_older_ones_upgrade() {
    let canonical = json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard": {
            "characters": "all", "max_age_seconds": 3600, "tabs": ["5ba2e1880a", "421496994e"]
        } } } } });
    let p = SyncPolicy::from_value(&canonical).unwrap();
    assert_eq!(serde_json::to_value(&p).unwrap(), canonical);
    for (bad, path) in [
        (
            json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard": {
                    "tabs": [], "characters": "all", "max_age_seconds": 60 } } } } }),
            "/realms/pc/leagues/Standard/tabs",
        ),
        (
            json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard": {
                    "tabs": "all", "characters": null, "max_age_seconds": 60 } } } } }),
            "/realms/pc/leagues/Standard/characters",
        ),
    ] {
        let err = SyncPolicy::from_value(&bad).unwrap_err();
        assert!(
            matches!(&err, PlanError::MalformedPolicy { detail }
                    if detail.contains("canonical") && detail.contains(path)),
            "{bad}: {err}"
        );
        let mut a = Annotations::open_memory_for("u-1").unwrap();
        assert!(put_sync_policy(&mut a, &bad, None, PolicyWrite::Replace, &via_test()).is_err());
        assert!(a.get("account", "", SYNC_POLICY_KIND).unwrap().is_none());
    }
    // A v2 value upgrades (characters: none) and is stored as written.
    let v2 = json!({ "version": 2, "realms": { "pc": { "leagues": { "Standard": {
            "tabs": "all", "max_age_seconds": 60 } } } } });
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    let row = put_sync_policy(&mut a, &v2, None, PolicyWrite::Replace, &via_test()).unwrap();
    assert_eq!(row.value, v2);
    let (_, typed) = a.get_as::<SyncPolicy>("account", "").unwrap().unwrap();
    assert_eq!(typed.version, SYNC_POLICY_VERSION);
    assert!(
        typed.realms[&Realm::Pc].leagues["Standard"]
            .characters
            .is_none()
    );
}

#[test]
fn no_policy_and_uncovered_league_are_distinct_structured_errors() {
    let s = store();
    // No sync-policy row: nothing declared, nothing to compile.
    let err = plan_refresh("mock", &snapshot(&s), 1000).unwrap_err();
    assert_eq!(err, PlanError::NoSyncPolicy);
    // A policy that does not name the league authorizes nothing there.
    let hc_only = json!({
        "version": 1,
        "leagues": { "Hardcore": { "tabs": "all", "max_age_seconds": 60 } }
    });
    let err = plan_refresh("mock", &snapshot_with(&s, &hc_only), 1000).unwrap_err();
    assert_eq!(
        err,
        PlanError::LeagueNotCovered {
            realm: Realm::Pc,
            league: "Standard".into()
        }
    );
    // Coverage is per (realm, league): Standard on xbox does not cover
    // Standard on pc.
    let xbox_only = json!({
        "version": 2,
        "realms": { "xbox": { "leagues": { "Standard": { "tabs": "all", "max_age_seconds": 60 } } } }
    });
    let err = plan_refresh("mock", &snapshot_with(&s, &xbox_only), 1000).unwrap_err();
    assert!(
        matches!(
            err,
            PlanError::LeagueNotCovered {
                realm: Realm::Pc,
                ..
            }
        ),
        "{err}"
    );
    // A stored policy row a newer build wrote surfaces its version,
    // and the version gate is not bypassable by deserializing the
    // type directly instead of calling from_value. This build's own
    // write door refuses a v4 value (C66), so the row is placed on the
    // snapshot by hand, as a newer build would have stored it.
    let v4 = json!({ "version": 4, "realms": {} });
    let mut newer = snapshot(&s);
    newer.policy = Some(AnnotationRow {
        scope: SYNC_POLICY_SCOPE.into(),
        key: SYNC_POLICY_KEY.into(),
        kind: SYNC_POLICY_KIND.into(),
        value: v4.clone(),
        revision: 1,
        created_at: 1,
        updated_at: 1,
        written_via: "future-build".into(),
        actor: None,
    });
    let err = plan_refresh("mock", &newer, 1000).unwrap_err();
    assert_eq!(
        err,
        PlanError::PolicyVersionUnsupported {
            found: 4,
            supported: 3
        }
    );
    let err = serde_json::from_value::<SyncPolicy>(v4).unwrap_err();
    assert!(err.to_string().contains("version 4"), "{err}");
}

/// C38 — plans act only on facts on record: without a listing basis the plan has no membership authority, and discovery waits for the next plan.
#[test]
fn a_never_listed_league_plans_the_listing_alone() {
    let mut s = store();
    // A tab on record from a direct fetch, stale by any window — and
    // still not fetched: with no listing basis the plan has no
    // membership authority, so the listing goes alone and the tab is
    // reported as waiting (D5a's eventual reconciliation).
    fetch(
        &mut s,
        json!({ "id": "x1", "name": "Fetched", "type": "PremiumStash", "items": [item("i1")] }),
        10,
    );
    let plan = plan(&s, &all_policy_value(3600), 100_000);
    assert_eq!(
        plan.actions,
        vec![RefreshAction::ListStashes {
            realm: Realm::Pc,
            league: "Standard".into(),
            reason: ListingReason::NeverListed,
        }]
    );
    assert_eq!(
        plan.skipped_tabs,
        vec![SkippedTab {
            id: "x1".into(),
            name: "Fetched".into(),
            reason: SkipReason::AwaitingListing,
        }]
    );
    assert_eq!(plan.basis.stash_listing, None);
    assert_eq!(plan.logical_requests, 1);
    assert_eq!((plan.wire_sends.min, plan.wire_sends.max), (1, 3));
    assert_eq!(plan.wire_sends.prerequisites.len(), 2);
}

/// C36 — coverage plus freshness compile to minimal requests (here none); C38 — the envelope carries provider, uuid, operation, schema stamp, fact basis and the policy revision even when empty.
#[test]
fn fresh_facts_compile_to_an_empty_plan_that_still_cites_its_basis() {
    let mut s = store();
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    list(
        &mut s,
        json!([{ "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 }]),
        1000,
    );
    fetch(
        &mut s,
        json!({ "id": "t1", "name": "One", "type": "PremiumStash", "items": [item("i1")] }),
        1010,
    );
    let row = store_policy(
        &mut a,
        &json!({ "version": 1, "leagues": { "Standard": { "tabs": "all", "max_age_seconds": 3600 } } }),
    );
    let snap = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    let plan = plan_refresh("mock", &snap, 1100).unwrap();
    assert!(plan.actions.is_empty());
    assert_eq!(plan.logical_requests, 0);
    assert_eq!((plan.wire_sends.min, plan.wire_sends.max), (0, 0));
    assert!(plan.wire_sends.prerequisites.is_empty());
    // The plan shows its reasoning: the fresh tab is named as skipped.
    assert_eq!(
        plan.skipped_tabs,
        vec![SkippedTab {
            id: "t1".into(),
            name: "One".into(),
            reason: SkipReason::Fresh { age_seconds: 90 },
        }]
    );
    // And its basis: uuid, provider, listing row, policy revision.
    assert_eq!(plan.account_uuid, "u-1");
    assert_eq!(plan.provider, "mock");
    assert_eq!(plan.basis.stash_listing, snap.stash_listing);
    assert_eq!(plan.basis.policy_revision, Some(row.revision));
    assert_eq!(plan.basis.snapshot_taken_at, snap.taken_at);
    assert_eq!(plan.plan_schema, REFRESH_PLAN_SCHEMA);
    assert_eq!(plan.operation, "refresh");
}

/// C36 — the policy's window, not a schedule, decides what is fetched.
#[test]
fn staleness_forces_fetches_and_a_stale_listing_relists() {
    let mut s = store();
    list(
        &mut s,
        json!([
            { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 },
            { "id": "t2", "name": "Two", "type": "PremiumStash", "index": 1 },
        ]),
        1000,
    );
    fetch(
        &mut s,
        json!({ "id": "t1", "name": "One", "type": "PremiumStash", "items": [item("i1")] }),
        1010,
    );
    // At t=5000 with a 3600s window: the listing (age 4000) is stale,
    // t1 (age 3990) is stale, t2 was never fetched.
    let plan = plan(&s, &all_policy_value(3600), 5000);
    assert_eq!(
        plan.actions,
        vec![
            RefreshAction::ListStashes {
                realm: Realm::Pc,
                league: "Standard".into(),
                reason: ListingReason::Stale { age_seconds: 4000 },
            },
            RefreshAction::FetchTab {
                realm: Realm::Pc,
                league: "Standard".into(),
                id: "t1".into(),
                name: "One".into(),
                tab_type: "PremiumStash".into(),
                reason: FetchReason::Stale { age_seconds: 3990 },
            },
            RefreshAction::FetchTab {
                realm: Realm::Pc,
                league: "Standard".into(),
                id: "t2".into(),
                name: "Two".into(),
                tab_type: "PremiumStash".into(),
                reason: FetchReason::NeverFetched,
            },
        ]
    );
    assert_eq!(plan.logical_requests, 3);
    assert_eq!((plan.wire_sends.min, plan.wire_sends.max), (3, 9));
}

/// C36 — `metadata.items` counts are heuristic evidence: they prove a tab changed, never that it didn't.
#[test]
fn a_disagreeing_count_proves_change_and_an_agreeing_one_proves_nothing() {
    let mut s = store();
    list(
        &mut s,
        json!([{ "id": "m1", "name": "Maps", "type": "MapStash", "index": 0 }]),
        1000,
    );
    // Fetching the map tab lands the substash stub (metadata.items 2),
    // then the substash itself is fetched and holds one item.
    fetch(
        &mut s,
        json!({ "id": "m1", "name": "Maps", "type": "MapStash", "items": [], "children": [
                { "id": "s1", "name": "", "type": "MapStash", "parent": "m1",
                  "metadata": { "items": 2, "map": { "name": "Tier 16" } } } ] }),
        1010,
    );
    s.record(
            &Endpoint::Stash { realm: "pc".into(),
                league: "Standard".into(),
                id: "m1".into(),
                sub: Some("s1".into()),
            },
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stash": { "id": "s1", "name": "", "type": "MapStash", "parent": "m1", "items": [item("map1")] } }),
            1020,
        )
        .unwrap();
    // Everything is fresh at t=1100 — the stub's count (2 vs 1 held)
    // predates our fetch, so it proves nothing.
    let quiet = plan(&s, &all_policy_value(3600), 1100);
    assert!(quiet.actions.is_empty(), "{:?}", quiet.actions);
    // Re-fetching the parent stamps a newer stub whose count disagrees
    // with what we hold: proof of change, fetched though fresh by age —
    // and under the parent, as a substash.
    fetch(
        &mut s,
        json!({ "id": "m1", "name": "Maps", "type": "MapStash", "items": [], "children": [
                { "id": "s1", "name": "", "type": "MapStash", "parent": "m1",
                  "metadata": { "items": 3, "map": { "name": "Tier 16" } } } ] }),
        1050,
    );
    let changed = plan(&s, &all_policy_value(3600), 1100);
    assert_eq!(
        changed.actions,
        vec![RefreshAction::FetchSubstash {
            realm: Realm::Pc,
            league: "Standard".into(),
            parent: "m1".into(),
            id: "s1".into(),
            name: "".into(),
            tab_type: "MapStash".into(),
            reason: FetchReason::ListedCountDisagrees { listed: 3, held: 1 },
        }]
    );
    // An agreeing count proves nothing either way. A newer stub now
    // counts what we hold (1 and 1): inside the window nothing is
    // fetched for it, and past the window it cannot skip the stale
    // fetch — the heuristic proves change, never freshness.
    fetch(
        &mut s,
        json!({ "id": "m1", "name": "Maps", "type": "MapStash", "items": [], "children": [
                { "id": "s1", "name": "", "type": "MapStash", "parent": "m1",
                  "metadata": { "items": 1, "map": { "name": "Tier 16" } } } ] }),
        1060,
    );
    let agreeing = plan(&s, &all_policy_value(3600), 1100);
    assert!(agreeing.actions.is_empty(), "{:?}", agreeing.actions);
    let stale = plan(&s, &all_policy_value(3600), 9000);
    assert!(stale.actions.iter().any(|a| matches!(
        a,
        RefreshAction::FetchSubstash { id, reason: FetchReason::Stale { .. }, .. } if id == "s1"
    )));
}

#[test]
fn an_orphaned_substash_is_skipped_never_rendered_by_the_wrong_path() {
    let mut s = store();
    list(
        &mut s,
        json!([{ "id": "m1", "name": "Maps", "type": "MapStash", "index": 0 }]),
        1000,
    );
    fetch(
        &mut s,
        json!({ "id": "m1", "name": "Maps", "type": "MapStash", "items": [], "children": [
                { "id": "s1", "name": "", "type": "MapStash", "parent": "m1",
                  "metadata": { "items": 2 } } ] }),
        1010,
    );
    // A later listing retires m1; the store keeps the never-listed
    // substash row, whose recorded parent is now off the record. A
    // fetch by its own id would hit /stash/{league}/s1 — the wrong
    // endpoint — so the plan reports it instead of guessing.
    list(
        &mut s,
        json!([{ "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 }]),
        2000,
    );
    let plan = plan(&s, &all_policy_value(3600), 2100);
    assert!(
        !plan
            .actions
            .iter()
            .any(|a| matches!(a, RefreshAction::FetchTab { id, .. } if id == "s1")),
        "orphaned substash must not become a top-level fetch: {:?}",
        plan.actions
    );
    assert!(
        plan.skipped_tabs.contains(&SkippedTab {
            id: "s1".into(),
            name: "".into(),
            reason: SkipReason::OrphanedParent {
                parent: "m1".into()
            },
        }),
        "{:?}",
        plan.skipped_tabs
    );
}

#[test]
fn corrupt_store_timestamps_read_as_stale_never_panic_or_wrap() {
    let mut s = store();
    // Absurd timestamps a damaged file could hold: pre-fix, the age
    // subtraction overflowed (panic in debug, wrap-to-fresh in
    // release). They must saturate into "very stale".
    list(
        &mut s,
        json!([{ "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 }]),
        i64::MIN,
    );
    fetch(
        &mut s,
        json!({ "id": "t1", "name": "One", "type": "PremiumStash", "items": [] }),
        i64::MIN,
    );
    let plan = plan(&s, &all_policy_value(3600), 100);
    assert!(matches!(
        plan.actions[0],
        RefreshAction::ListStashes {
            realm: Realm::Pc,
            reason: ListingReason::Stale {
                age_seconds: i64::MAX
            },
            ..
        }
    ));
    assert!(plan.actions.iter().any(|a| matches!(
            a,
            RefreshAction::FetchTab { id, reason: FetchReason::Stale { age_seconds: i64::MAX }, .. } if id == "t1"
        )));
}

fn ids_policy_value(ids: &[&str], max_age_seconds: u32) -> Value {
    json!({
        "version": 1,
        "leagues": { "Standard": { "tabs": ids, "max_age_seconds": max_age_seconds } }
    })
}

/// C37 — a tab id covers the tab and its children; substashes are planned the cycle after their stubs land; an empty stub is skipped with its reason.
#[test]
fn a_named_map_tab_covers_its_substashes_the_cycle_after_its_first_fetch() {
    let mut s = store();
    list(
        &mut s,
        json!([
            { "id": "m1", "name": "Maps", "type": "MapStash", "index": 0 },
            { "id": "t1", "name": "Other", "type": "PremiumStash", "index": 1 },
        ]),
        1000,
    );
    let policy = ids_policy_value(&["m1"], 3600);
    // Cycle 1: the parent alone — no stubs are on record yet, and a
    // plan never expands itself (binding). t1 is outside the policy.
    let plan1 = plan(&s, &policy, 1100);
    assert_eq!(
        plan1.actions,
        vec![RefreshAction::FetchTab {
            realm: Realm::Pc,
            league: "Standard".into(),
            id: "m1".into(),
            name: "Maps".into(),
            tab_type: "MapStash".into(),
            reason: FetchReason::NeverFetched,
        }]
    );
    // The parent's fetch lands two stubs: one with items, one empty.
    fetch(
        &mut s,
        json!({ "id": "m1", "name": "Maps", "type": "MapStash", "items": [], "children": [
                { "id": "s1", "name": "", "type": "MapStash", "parent": "m1",
                  "metadata": { "items": 2, "map": { "name": "Tier 16" } } },
                { "id": "s0", "name": "", "type": "MapStash", "parent": "m1",
                  "metadata": { "items": 0, "map": { "name": "Tier 1" } } } ] }),
        1100,
    );
    // Cycle 2: the same policy now covers the substashes through their
    // parent; the empty stub is skipped with its reason, the parent is
    // fresh.
    let plan2 = plan(&s, &policy, 1200);
    assert_eq!(
        plan2.actions,
        vec![RefreshAction::FetchSubstash {
            realm: Realm::Pc,
            league: "Standard".into(),
            parent: "m1".into(),
            id: "s1".into(),
            name: "".into(),
            tab_type: "MapStash".into(),
            reason: FetchReason::NeverFetched,
        }]
    );
    assert_eq!(
        plan2.skipped_tabs,
        vec![
            SkippedTab {
                id: "m1".into(),
                name: "Maps".into(),
                reason: SkipReason::Fresh { age_seconds: 100 },
            },
            SkippedTab {
                id: "s0".into(),
                name: "".into(),
                reason: SkipReason::EmptyStub,
            },
        ]
    );
    // The new skip kind travels through the strict wire parse.
    let round = RefreshPlan::from_value(&serde_json::to_value(&plan2).unwrap()).unwrap();
    assert_eq!(round, plan2);
    // A child named directly is covered as before.
    let direct = plan(&s, &ids_policy_value(&["s1"], 3600), 1200);
    assert_eq!(direct.actions, plan2.actions);
    assert!(direct.skipped_tabs.is_empty());
    // An empty stub that nevertheless holds items (a stale 0 against a
    // real fetch) is not "empty": the disagreement arm decides.
    let mut s2 = store();
    list(
        &mut s2,
        json!([{ "id": "m1", "name": "Maps", "type": "MapStash", "index": 0 }]),
        1000,
    );
    fetch(
        &mut s2,
        json!({ "id": "m1", "name": "Maps", "type": "MapStash", "items": [], "children": [
                { "id": "s0", "name": "", "type": "MapStash", "parent": "m1",
                  "metadata": { "items": 1 } } ] }),
        1100,
    );
    s2.record(
        &Endpoint::Stash {
            realm: "pc".into(),
            league: "Standard".into(),
            id: "m1".into(),
            sub: Some("s0".into()),
        },
        &json!({ "league": "Standard" }),
        200,
        &json!({ "stash": { "id": "s0", "name": "", "type": "MapStash", "parent": "m1",
                                "metadata": { "items": 1 }, "items": [item("x")] } }),
        1110,
    )
    .unwrap();
    fetch(
        &mut s2,
        json!({ "id": "m1", "name": "Maps", "type": "MapStash", "items": [], "children": [
                { "id": "s0", "name": "", "type": "MapStash", "parent": "m1",
                  "metadata": { "items": 0 } } ] }),
        1120,
    );
    let p = plan(&s2, &ids_policy_value(&["m1"], 3600), 1200);
    assert!(
        matches!(
            p.actions.as_slice(),
            [RefreshAction::FetchSubstash { id, reason: FetchReason::ListedCountDisagrees { listed: 0, held: 1 }, .. }] if id == "s0"
        ),
        "{:?}",
        p.actions
    );
}

/// C37 — a folder's children are covered at once, since the listing carries them.
#[test]
fn a_named_folder_covers_its_children_at_once() {
    let mut s = store();
    list(
        &mut s,
        json!([
            { "id": "f1", "name": "Folder", "type": "Folder", "index": 0,
              "children": [ { "id": "c1", "name": "In folder", "type": "PremiumStash", "index": 1 } ] },
            { "id": "t1", "name": "Other", "type": "PremiumStash", "index": 2 },
        ]),
        1000,
    );
    // Folder children are in the listing, so naming the folder plans
    // them in the same cycle; the folder itself is skipped as before
    // and t1 stays outside the policy.
    let plan = plan(&s, &ids_policy_value(&["f1"], 3600), 1100);
    assert_eq!(
        plan.actions,
        vec![RefreshAction::FetchTab {
            realm: Realm::Pc,
            league: "Standard".into(),
            id: "c1".into(),
            name: "In folder".into(),
            tab_type: "PremiumStash".into(),
            reason: FetchReason::NeverFetched,
        }]
    );
    assert_eq!(
        plan.skipped_tabs,
        vec![SkippedTab {
            id: "f1".into(),
            name: "Folder".into(),
            reason: SkipReason::Folder,
        }]
    );
    assert!(plan.unknown_tabs.is_empty());
}

#[test]
fn folders_are_never_fetched_and_their_children_go_by_own_id() {
    let mut s = store();
    list(
        &mut s,
        json!([
            { "id": "f1", "name": "Folder", "type": "Folder", "index": 0,
              "children": [ { "id": "c1", "name": "In folder", "type": "PremiumStash", "index": 1 } ] },
        ]),
        1000,
    );
    let plan = plan(&s, &all_policy_value(3600), 1100);
    // The folder is skipped with its reason; the child is a plain tab
    // fetch by its own id (folders group, they don't contain).
    assert_eq!(
        plan.actions,
        vec![RefreshAction::FetchTab {
            realm: Realm::Pc,
            league: "Standard".into(),
            id: "c1".into(),
            name: "In folder".into(),
            tab_type: "PremiumStash".into(),
            reason: FetchReason::NeverFetched,
        }]
    );
    assert_eq!(
        plan.skipped_tabs,
        vec![SkippedTab {
            id: "f1".into(),
            name: "Folder".into(),
            reason: SkipReason::Folder,
        }]
    );
}

/// C38 — never an unreviewed addition: an id the facts lack is reported, not fetched.
#[test]
fn ids_the_facts_do_not_know_are_reported_never_invented() {
    let mut s = store();
    list(
        &mut s,
        json!([{ "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 }]),
        1000,
    );
    let p = json!({
        "version": 1,
        "leagues": { "Standard": { "tabs": ["t1", "ghost"], "max_age_seconds": 3600 } }
    });
    let plan = plan(&s, &p, 1100);
    // The known tab gets its action; the vanished (or mistyped) id is
    // reported, and no action names it (D5a: reported skipped).
    assert_eq!(plan.actions.len(), 1);
    assert!(matches!(
        &plan.actions[0],
        RefreshAction::FetchTab { id, .. } if id == "t1"
    ));
    assert_eq!(plan.unknown_tabs, vec!["ghost".to_string()]);
}

/// C43 — a plan's actions render as the daemon's own `(kind, params)` tuples, and a fetch never fans out.
#[test]
fn actions_decode_through_the_stores_job_vocabulary_decoder() {
    // Not literals-vs-literals: `Endpoint::from_job` is the store's
    // production decoder of the daemon's job vocabulary (it is what
    // `Daemon::record` classifies through), so a vocabulary change
    // that moves the store breaks this test even if this crate's
    // strings were left behind.
    let listing = RefreshAction::ListStashes {
        realm: Realm::Pc,
        league: "Standard".into(),
        reason: ListingReason::NeverListed,
    };
    let (kind, params) = listing.job();
    assert_eq!(
        Endpoint::from_job(kind, &params),
        Some(Endpoint::Stashes {
            realm: "pc".into(),
            league: "Standard".into()
        })
    );
    let tab = RefreshAction::FetchTab {
        realm: Realm::Pc,
        league: "Standard".into(),
        id: "t1".into(),
        name: "One".into(),
        tab_type: "PremiumStash".into(),
        reason: FetchReason::NeverFetched,
    };
    let (kind, params) = tab.job();
    assert_eq!(
        Endpoint::from_job(kind, &params),
        Some(Endpoint::Stash {
            realm: "pc".into(),
            league: "Standard".into(),
            id: "t1".into(),
            sub: None,
        })
    );
    // `deep` is not part of the endpoint; assert it separately — a
    // plan's fetch must never fan out (D5a).
    assert_eq!(params["deep"], json!(false));
    let substash = RefreshAction::FetchSubstash {
        realm: Realm::Pc,
        league: "Standard".into(),
        parent: "m1".into(),
        id: "s1".into(),
        name: "".into(),
        tab_type: "MapStash".into(),
        reason: FetchReason::NeverFetched,
    };
    let (kind, params) = substash.job();
    assert_eq!(
        Endpoint::from_job(kind, &params),
        Some(Endpoint::Stash {
            realm: "pc".into(),
            league: "Standard".into(),
            id: "m1".into(),
            sub: Some("s1".into()),
        })
    );
    assert_eq!(params["deep"], json!(false));
}

/// C38 — applying exactly the listed actions satisfies the plan (the offline half of the loop).
#[test]
fn applying_a_plans_actions_through_the_store_satisfies_the_plan() {
    // The offline half of apply: each action's job tuple, decoded by
    // the store's own vocabulary and recorded, produces facts that
    // the next plan finds fresh. If job() rendered anything the
    // record pipeline does not recognize as the planned tab, the
    // replan would still want work.
    let mut s = store();
    list(
        &mut s,
        json!([{ "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 }]),
        1000,
    );
    let p = all_policy_value(3600);
    let first = plan(&s, &p, 8000);
    assert_eq!(first.logical_requests, 2, "{:?}", first.actions);
    for action in &first.actions {
        let (kind, params) = action.job();
        let endpoint = Endpoint::from_job(kind, &params).unwrap();
        let body = match action {
            RefreshAction::ListStashes { .. } => {
                json!({ "stashes": [ { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 } ] })
            }
            _ => {
                json!({ "stash": { "id": "t1", "name": "One", "type": "PremiumStash", "items": [item("i1")] } })
            }
        };
        s.record(&endpoint, &params, 200, &body, 8000).unwrap();
    }
    let second = plan(&s, &p, 8000);
    assert!(second.actions.is_empty(), "{:?}", second.actions);
}

/// C38 — a Plan is serializable and immutable: what crosses a socket or lands in a journal parses back equal.
#[test]
fn the_plan_round_trips_through_serde_unchanged() {
    let mut s = store();
    list(
        &mut s,
        json!([
            { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 },
            { "id": "f1", "name": "Folder", "type": "Folder", "index": 1,
              "children": [ { "id": "c1", "name": "In folder", "type": "PremiumStash", "index": 2 } ] },
        ]),
        1000,
    );
    let p = json!({
        "version": 1,
        "leagues": { "Standard": { "tabs": ["t1", "f1", "c1", "ghost"], "max_age_seconds": 60 } }
    });
    let plan = plan(&s, &p, 5000);
    // A plan crosses sockets and lands in journals; serialization is
    // part of its contract, not a debug convenience.
    let json = serde_json::to_value(&plan).unwrap();
    assert_eq!(RefreshPlan::from_value(&json).unwrap(), plan);
    let back: RefreshPlan = serde_json::from_value(json).unwrap();
    assert_eq!(back, plan);
}

/// C40 — the quote is optional enrichment with its own observation time; C38 — it must speak about exactly this plan's work.
#[test]
fn a_quote_enriches_a_plan_optionally_and_must_speak_about_it() {
    use acquisition_protocol::protocol::{QuoteJob, QuoteScope};
    let mut s = store();
    list(
        &mut s,
        json!([{ "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 }]),
        1000,
    );
    let bare = plan(&s, &all_policy_value(60), 5000);
    assert_eq!(bare.quote, None, "compiling never fills the quote");
    assert_eq!(bare.logical_requests, 2, "{:?}", bare.actions);
    assert!(bare.account_name.is_some(), "the plan is account-bound");
    // The quote's verifiable work basis: exactly the plan's actions
    // rendered as job tuples, in order.
    let work: Vec<QuoteJob> = bare
        .actions
        .iter()
        .map(|a| {
            let (kind, params) = a.job();
            QuoteJob {
                kind: kind.into(),
                params,
            }
        })
        .collect();
    let quote = Quote {
        observed_at: 5000,
        provider: bare.provider.clone(),
        account: bare.account_name.clone(),
        halted: None,
        work: work.clone(),
        scopes: vec![QuoteScope {
            key: "stash-list".into(),
            endpoints: vec!["stash-list".into(), "stash".into()],
            requests: 2,
            queued_ahead: 0,
            policy: None,
            rules: Vec::new(),
            observed_seconds_ago: None,
            eta_seconds: None,
            notes: vec!["policy not yet learned".into()],
        }],
        not_covered: vec!["a HEAD probe before the first request (N16)".into()],
    };
    let enriched = bare.clone().with_quote(quote.clone()).unwrap();
    assert_eq!(enriched.quote.as_ref(), Some(&quote));
    // The enriched plan round-trips like everything else it carries.
    let json = serde_json::to_value(&enriched).unwrap();
    assert_eq!(RefreshPlan::from_value(&json).unwrap(), enriched);
    // A smuggled field inside the carried quote refuses at parse.
    let mut nested = json.clone();
    nested["quote"]["extra"] = json!(true);
    assert!(RefreshPlan::from_value(&nested).is_err());
    let mut nested = json.clone();
    nested["quote"]["scopes"][0]["surprise"] = json!(true);
    assert!(RefreshPlan::from_value(&nested).is_err());
    // A quote about another provider or account refuses — at attach
    // and at parse alike: it would mislead the review the plan is for.
    let mut foreign = quote.clone();
    foreign.provider = "ggg".into();
    assert!(matches!(
        bare.clone().with_quote(foreign),
        Err(PlanError::MalformedPlan { .. })
    ));
    let mut tampered = json.clone();
    tampered["quote"]["provider"] = json!("ggg");
    assert!(RefreshPlan::from_value(&tampered).is_err());
    let mut other_account = quote.clone();
    other_account.account = Some("mallory#9999".into());
    assert!(bare.clone().with_quote(other_account).is_err());
    // An accountless quote on an account-bound plan refuses too: it
    // projects someone else's limiter state (`null` is not a wildcard).
    let mut accountless = quote.clone();
    accountless.account = None;
    assert!(bare.clone().with_quote(accountless).is_err());
    // A quote that projects fewer requests than the plan authorizes
    // cannot dress the plan up — at attach and at parse alike.
    let mut partial = quote.clone();
    partial.scopes[0].requests = 1;
    let err = bare.clone().with_quote(partial).unwrap_err();
    assert!(err.to_string().contains("authorizes 2"), "{err}");
    let mut shrunk = json.clone();
    shrunk["quote"]["scopes"][0]["requests"] = json!(1);
    let err = RefreshPlan::from_value(&shrunk).unwrap_err();
    assert!(err.to_string().contains("authorizes 2"), "{err}");
    // Equal counts are not coverage: a quote for two *other* jobs of
    // the same size — or the right jobs in a different order — is not
    // a quote for this plan.
    let mut foreign_work = quote.clone();
    foreign_work.work = vec![
        QuoteJob {
            kind: "fetch".into(),
            params: json!({}),
        },
        QuoteJob {
            kind: "fetch".into(),
            params: json!({}),
        },
    ];
    let err = bare.clone().with_quote(foreign_work).unwrap_err();
    assert!(err.to_string().contains("not the plan's action"), "{err}");
    let mut reordered = quote.clone();
    reordered.work.swap(0, 1);
    assert!(bare.clone().with_quote(reordered).is_err());
    let mut swapped = json.clone();
    swapped["quote"]["work"] = serde_json::to_value([&work[1], &work[0]]).unwrap();
    assert!(RefreshPlan::from_value(&swapped).is_err());
    // Scope totals that overflow are a structured refusal, never a
    // wrap past validation or a panic (the no-panic rule covers a
    // malformed envelope too).
    let mut overflowing = quote;
    let extra = QuoteScope {
        requests: u64::MAX,
        ..overflowing.scopes[0].clone()
    };
    overflowing.scopes.push(extra);
    let err = bare.with_quote(overflowing).unwrap_err();
    assert!(err.to_string().contains("overflows"), "{err}");
}

/// C38 — a newer stamp, a smuggled field at any depth, a foreign league or realm refuse whole; C42 — a forged logical bound cannot be trusted away by admission budgeting.
#[test]
fn a_tampered_or_newer_plan_does_not_parse_into_a_trusted_envelope() {
    let mut s = store();
    list(
        &mut s,
        json!([{ "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 }]),
        1000,
    );
    let good = serde_json::to_value(plan(&s, &all_policy_value(60), 5000)).unwrap();
    // Sanity: 2 actions (re-list + fetch), and the untampered form parses.
    assert_eq!(good["logical_requests"], json!(2));
    assert!(RefreshPlan::from_value(&good).is_ok());
    // A newer schema reports as such — before any shape complaint —
    // and raw serde refuses it too (the validation is in the type,
    // not only in from_value).
    let mut newer = good.clone();
    newer["plan_schema"] = json!(REFRESH_PLAN_SCHEMA + 1);
    assert_eq!(
        RefreshPlan::from_value(&newer).unwrap_err(),
        PlanError::PlanSchemaUnsupported {
            found: REFRESH_PLAN_SCHEMA + 1,
            supported: REFRESH_PLAN_SCHEMA
        }
    );
    assert!(serde_json::from_value::<RefreshPlan>(newer).is_err());
    // Derived counts recompute or the envelope is refused: a forged
    // logical bound is exactly what admission-time budgeting (D8)
    // must not be able to trust away.
    let mut forged = good.clone();
    forged["logical_requests"] = json!(1);
    let err = RefreshPlan::from_value(&forged).unwrap_err();
    assert!(matches!(err, PlanError::MalformedPlan { .. }), "{err}");
    let mut forged = good.clone();
    forged["wire_sends"]["max"] = json!(999);
    assert!(RefreshPlan::from_value(&forged).is_err());
    // The prerequisites are part of the reviewed wire projection, not
    // display text: an emptied (or reworded) list refuses too.
    let mut forged = good.clone();
    forged["wire_sends"]["prerequisites"] = json!([]);
    let err = RefreshPlan::from_value(&forged).unwrap_err();
    assert!(err.to_string().contains("prerequisites"), "{err}");
    // A stray operation, a smuggled field, and an action for another
    // league are each refused whole.
    let mut wrong_op = good.clone();
    wrong_op["operation"] = json!("delete-everything");
    assert!(RefreshPlan::from_value(&wrong_op).is_err());
    let mut smuggled = good.clone();
    smuggled["extra_authorization"] = json!(true);
    assert!(RefreshPlan::from_value(&smuggled).is_err());
    // Unknown fields refuse at every depth, not only the top level:
    // inside the wire estimate, an action, a reason, and the basis
    // (its listing included).
    for path in [
        &["wire_sends", "extra"][..],
        &["actions", "0", "surprise"],
        &["actions", "0", "reason", "surprise"],
        &["basis", "extra"],
        &["basis", "stash_listing", "extra"],
    ] {
        let mut nested = good.clone();
        let mut spot = &mut nested;
        for key in &path[..path.len() - 1] {
            spot = match key.parse::<usize>() {
                Ok(i) => &mut spot[i],
                Err(_) => &mut spot[*key],
            };
        }
        spot[*path.last().unwrap()] = json!(true);
        assert!(
            RefreshPlan::from_value(&nested).is_err(),
            "smuggled field at {path:?} was accepted"
        );
    }
    let mut stray = good.clone();
    stray["actions"][0]["league"] = json!("Hardcore");
    let err = RefreshPlan::from_value(&stray).unwrap_err();
    assert!(err.to_string().contains("Hardcore"), "{err}");
    // …and an action for another realm: the envelope's realm binds
    // every action, so a pc plan cannot smuggle a console fetch.
    let mut stray = good.clone();
    stray["actions"][0]["realm"] = json!("xbox");
    let err = RefreshPlan::from_value(&stray).unwrap_err();
    assert!(err.to_string().contains("xbox"), "{err}");
    let mut unknown = good.clone();
    unknown["realm"] = json!("ps5");
    assert!(RefreshPlan::from_value(&unknown).is_err());
}

/// C52 — no blind replace: creating over a live row conflicts naming the revision to review; C36 — validated before it lands.
#[test]
fn put_sync_policy_validates_before_it_stores_and_is_a_cas() {
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    // A typo'd value is refused by the strict parse and nothing lands:
    // intent is never half-honored, on any frontend's write surface.
    let typo = json!({
        "version": 1,
        "leagues": { "Standard": { "tabs": "all", "max_age_secs": 60 } }
    });
    let err = put_sync_policy(&mut a, &typo, None, PolicyWrite::Replace, &via_test()).unwrap_err();
    assert!(matches!(err, PutPolicyError::Invalid(_)), "{err}");
    assert!(a.get("account", "", SYNC_POLICY_KIND).unwrap().is_none());
    // `None` creates; a second `None` over a live row is a conflict
    // naming the current revision — a caller (an agent especially)
    // never replaces intent it has not read.
    let value = all_policy_value(3600);
    let first = put_sync_policy(&mut a, &value, None, PolicyWrite::Replace, &via_test()).unwrap();
    assert_eq!(first.revision, 1);
    assert_eq!(first.written_via, "test");
    let err = put_sync_policy(&mut a, &value, None, PolicyWrite::Replace, &via_test()).unwrap_err();
    assert!(matches!(err, PutPolicyError::Store(_)), "{err}");
    assert!(err.to_string().contains("revision 1"), "{err}");
    // Naming the reviewed revision replaces it; naming a stale one
    // conflicts and the stored value is untouched.
    let second =
        put_sync_policy(&mut a, &value, Some(1), PolicyWrite::Replace, &via_test()).unwrap();
    assert_eq!(second.revision, 2);
    let err = put_sync_policy(
        &mut a,
        &all_policy_value(60),
        Some(1),
        PolicyWrite::Replace,
        &via_test(),
    )
    .unwrap_err();
    assert!(err.to_string().contains("revision 2"), "{err}");
    let held = a.get("account", "", SYNC_POLICY_KIND).unwrap().unwrap();
    assert_eq!(held.value, value);
}

/// C36, amended 2026-10-19 — a set replaces the realms it names and
/// keeps the rest: a poe2 policy written alone leaves the pc one
/// standing, a realm named with no leagues goes, and only `Replace`
/// writes the whole; one write is one revision, and the merge never
/// reaches past the compare-and-swap (C52).
#[test]
fn a_poe2_policy_set_alone_leaves_the_pc_policy_standing() {
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    let pc = json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard":
            { "tabs": "all", "max_age_seconds": 3600 } } } } });
    let poe2 = json!({ "version": 3, "realms": { "poe2": { "leagues": { "Standard":
            { "characters": "all", "max_age_seconds": 600 } } } } });
    let first = put_sync_policy(&mut a, &pc, None, PolicyWrite::Merge, &via_test()).unwrap();
    // Nothing to merge into: the first write is stored as written.
    assert_eq!(first.value, pc);
    let both = put_sync_policy(&mut a, &poe2, Some(1), PolicyWrite::Merge, &via_test()).unwrap();
    assert_eq!(both.revision, 2);
    let policy = SyncPolicy::from_value(&both.value).unwrap();
    assert!(
        policy.league(Realm::Pc, "Standard").is_some(),
        "{}",
        both.value
    );
    assert_eq!(
        policy
            .league(Realm::Poe2, "Standard")
            .unwrap()
            .max_age_seconds,
        600
    );
    // The realm named replaces that realm whole, its other leagues too.
    let pc_hc = json!({ "version": 3, "realms": { "pc": { "leagues": { "Hardcore":
            { "tabs": "all", "max_age_seconds": 3600 } } } } });
    let row = put_sync_policy(&mut a, &pc_hc, Some(2), PolicyWrite::Merge, &via_test()).unwrap();
    let policy = SyncPolicy::from_value(&row.value).unwrap();
    assert!(
        policy.league(Realm::Pc, "Standard").is_none(),
        "{}",
        row.value
    );
    assert!(
        policy.league(Realm::Pc, "Hardcore").is_some(),
        "{}",
        row.value
    );
    assert!(
        policy.league(Realm::Poe2, "Standard").is_some(),
        "{}",
        row.value
    );
    // A realm named with no leagues is removed; the others stay.
    let no_poe2 = json!({ "version": 3, "realms": { "poe2": { "leagues": {} } } });
    let row = put_sync_policy(&mut a, &no_poe2, Some(3), PolicyWrite::Merge, &via_test()).unwrap();
    assert_eq!(row.revision, 4);
    assert_eq!(row.value, pc_hc);
    // A stale or missing revision is the conflict it always was, and
    // nothing lands: the merge is made from the reviewed row alone.
    for stale in [None, Some(3), Some(5)] {
        let err =
            put_sync_policy(&mut a, &poe2, stale, PolicyWrite::Merge, &via_test()).unwrap_err();
        assert!(matches!(err, PutPolicyError::Store(_)), "{err}");
        assert!(err.to_string().contains("revision 4"), "{err}");
    }
    // `Replace` writes the whole: the pc realm the value omits is gone.
    let row = put_sync_policy(&mut a, &poe2, Some(4), PolicyWrite::Replace, &via_test()).unwrap();
    assert_eq!(row.revision, 5);
    assert_eq!(row.value, poe2);
}

/// Round 5's fix round, its review's finding 1: a sync policy at the
/// last revision an `i64` holds is refused in the policy's words,
/// never the store's, and nothing is written.
#[test]
fn a_policy_at_the_last_revision_is_refused_in_the_policys_words() {
    let dir = tempfile::tempdir().unwrap();
    let mut a = Annotations::open_for(dir.path(), "u-1").unwrap();
    let pc = json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard":
            { "tabs": "all", "max_age_seconds": 3600 } } } } });
    put_sync_policy(&mut a, &pc, None, PolicyWrite::Replace, &via_test()).unwrap();
    rusqlite::Connection::open(acquisition_store::annotations_path(dir.path(), "u-1"))
        .unwrap()
        .execute("UPDATE annotations SET revision = ?1", [i64::MAX])
        .unwrap();
    let err =
        put_sync_policy(&mut a, &pc, Some(i64::MAX), PolicyWrite::Merge, &via_test()).unwrap_err();
    assert_eq!(
        err.to_string(),
        format!(
            "the sync policy is at revision {}, the last a row can take: nothing was written",
            i64::MAX
        )
    );
    let row = a
        .get(SYNC_POLICY_SCOPE, SYNC_POLICY_KEY, SYNC_POLICY_KIND)
        .unwrap()
        .unwrap();
    assert_eq!((row.revision, row.value), (i64::MAX, pc));
}

/// Round 5 of the 2g build (the second cold review's finding 1): a
/// policy write over a row read beforehand names that row's revision,
/// so a row read at 1 and moved to 2 by another writer is the
/// conflict carrying 2 — never a write that lands and reports the row
/// read as replaced; the merge is made from the row passed, read once.
#[test]
fn a_policy_write_over_a_row_that_moved_since_it_was_read_is_the_conflict() {
    let mut a = Annotations::open_memory().unwrap();
    let pc = json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard":
            { "tabs": "all", "max_age_seconds": 3600 } } } } });
    let poe2 = json!({ "version": 3, "realms": { "poe2": { "leagues": { "Standard":
            { "characters": "all", "max_age_seconds": 3600 } } } } });
    let read = put_sync_policy(&mut a, &pc, None, PolicyWrite::Merge, &via_test()).unwrap();
    let moved = put_sync_policy(&mut a, &poe2, Some(1), PolicyWrite::Replace, &via_test()).unwrap();
    let err = put_sync_policy_over(&mut a, &poe2, Some(&read), PolicyWrite::Merge, &via_test())
        .unwrap_err();
    match err {
        PutPolicyError::Store(AnnotationError::Conflict { current }) => {
            assert_eq!(current.as_deref(), Some(&moved));
        }
        other => panic!("not the conflict: {other}"),
    }
    let landed =
        put_sync_policy_over(&mut a, &pc, Some(&moved), PolicyWrite::Merge, &via_test()).unwrap();
    assert_eq!(landed.revision, 3);
    let both = SyncPolicy::from_value(&landed.value).unwrap();
    assert_eq!(
        both.realms.len(),
        2,
        "merged into the row passed: {}",
        landed.value
    );
}

/// C36, amended 2026-10-19 — a realm named with no leagues goes from a
/// merge whether or not a policy is stored: the first write drops it
/// as a later one does.
#[test]
fn a_first_merge_drops_a_realm_named_with_no_leagues() {
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    let pc = json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard":
            { "tabs": "all", "max_age_seconds": 3600 } } } } });
    let mut with_empty = pc.clone();
    with_empty["realms"]["poe2"] = json!({ "leagues": {} });
    let first =
        put_sync_policy(&mut a, &with_empty, None, PolicyWrite::Merge, &via_test()).unwrap();
    assert_eq!(first.revision, 1);
    assert_eq!(first.value, pc);
}

/// A stored policy this build cannot read is never merged into: the
/// refusal names the whole write that still replaces it.
#[test]
fn a_policy_this_build_cannot_read_is_replaced_whole_never_merged() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let mut a = Annotations::open_for(&dir, "u-newer").unwrap();
    let pc = all_policy_value(3600);
    put_sync_policy(&mut a, &pc, None, PolicyWrite::Merge, &via_test()).unwrap();
    // The store's own door refuses a newer version at put, so the
    // newer build's value is planted under it, as that build would.
    rusqlite::Connection::open(acquisition_store::annotations_path(&dir, "u-newer"))
        .unwrap()
        .execute(
            "UPDATE annotations SET value = '{\"version\":99,\"realms\":{}}' \
                 WHERE kind = ?1",
            [SYNC_POLICY_KIND],
        )
        .unwrap();
    let err = put_sync_policy(&mut a, &pc, Some(1), PolicyWrite::Merge, &via_test()).unwrap_err();
    assert!(matches!(err, PutPolicyError::StoredUnreadable(_)), "{err}");
    assert!(err.to_string().contains("--replace"), "{err}");
    let row = put_sync_policy(&mut a, &pc, Some(1), PolicyWrite::Replace, &via_test()).unwrap();
    assert_eq!(row.revision, 2);
}

/// C44 — apply refuses a plan whose policy revision is no longer the stored one, checked against a fresh read; wrong uuid or provider never spends.
#[test]
fn check_spendable_is_the_shared_staleness_and_identity_gate() {
    // The plan derives from a rev-1 policy for account u-1 on mock.
    let plan = plan(&store(), &all_policy_value(3600), 2_000);
    assert_eq!(plan.basis.policy_revision, Some(1));
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    // Intent gone since the plan: refused, citing the derivation.
    let err = plan.check_spendable("mock", Some("u-1"), &a).unwrap_err();
    assert!(matches!(err, SpendError::PolicyGone { .. }), "{err}");
    // Intent standing at the plan's revision: spendable.
    put_sync_policy(
        &mut a,
        &all_policy_value(3600),
        None,
        PolicyWrite::Replace,
        &via_test(),
    )
    .unwrap();
    plan.check_spendable("mock", Some("u-1"), &a).unwrap();
    // Intent moved since the plan (the step-7 ruling): refused with
    // both revisions named, remedy = replan.
    put_sync_policy(
        &mut a,
        &all_policy_value(60),
        Some(1),
        PolicyWrite::Replace,
        &via_test(),
    )
    .unwrap();
    let err = plan.check_spendable("mock", Some("u-1"), &a).unwrap_err();
    assert!(matches!(err, SpendError::PolicyMoved { .. }), "{err}");
    let msg = err.to_string();
    assert!(
        msg.contains("revision 1") && msg.contains("revision 2") && msg.contains("replan"),
        "{msg}"
    );
    // A plan for another identity is never spent: wrong uuid (or no
    // uuid at all), wrong provider.
    let err = plan
        .check_spendable("mock", Some("u-other"), &a)
        .unwrap_err();
    assert!(matches!(err, SpendError::WrongAccount { .. }), "{err}");
    let err = plan.check_spendable("mock", None, &a).unwrap_err();
    assert!(err.to_string().contains("unmapped"), "{err}");
    let err = plan.check_spendable("ggg", Some("u-1"), &a).unwrap_err();
    assert!(matches!(err, SpendError::WrongProvider { .. }), "{err}");
}

/// C44 as amended — a plan compiled from an explicit selection
/// carries no policy revision and the apply checks none: it applies
/// with no policy row at all, while a policy plan whose row moved is
/// still refused; each named tab is fetched however fresh, under deep
/// a named map tab's pages with it; the envelope round-trips, and
/// naming both authorities, or neither, is refused.
#[test]
fn c44_a_selection_plan_is_authorized_by_its_selection() {
    let mut s = store();
    list(
        &mut s,
        json!([
            { "id": "m1", "name": "Maps", "type": "MapStash", "index": 0 },
            { "id": "t1", "name": "Other", "type": "PremiumStash", "index": 1 },
        ]),
        1000,
    );
    fetch(
        &mut s,
        json!({ "id": "m1", "name": "Maps", "type": "MapStash", "items": [], "children": [
                { "id": "s1", "name": "", "type": "MapStash", "parent": "m1",
                  "metadata": { "items": 2, "map": { "name": "Tier 16" } } } ] }),
        1100,
    );
    fetch(
        &mut s,
        json!({ "id": "t1", "name": "Other", "type": "PremiumStash", "items": [] }),
        1100,
    );
    let a = Annotations::open_memory_for("u-1").unwrap();
    let snap = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    // both tabs fresh a second ago, and named: fetched all the same
    let selection = PlanSelection {
        tabs: Some(Selection::Ids(vec!["m1".into(), "t1".into()])),
        characters: Vec::new(),
        deep: false,
    };
    let plan = plan_selection("mock", &snap, &selection, 1101).unwrap();
    // C76: over a listed league the listing it selects from comes first,
    // refreshed on every run, the named tabs fetched beside it
    assert_eq!(
        plan.actions[0],
        RefreshAction::ListStashes {
            realm: Realm::Pc,
            league: "Standard".into(),
            reason: ListingReason::Selected,
        }
    );
    let fetched: Vec<&str> = plan.actions[1..]
        .iter()
        .map(|a| match a {
            RefreshAction::FetchTab { id, reason, .. } => {
                assert_eq!(*reason, FetchReason::Selected);
                id.as_str()
            }
            other => panic!("{other:?}"),
        })
        .collect();
    assert_eq!(fetched, ["m1", "t1"]);
    assert_eq!(plan.basis.policy_revision, None);
    assert_eq!(plan.basis.selection.as_ref(), Some(&selection));
    // no policy row: spendable all the same
    plan.check_spendable("mock", Some("u-1"), &a).unwrap();
    // under deep, the named map tab's page on record comes with it
    let deep = PlanSelection {
        deep: true,
        ..selection.clone()
    };
    let plan = plan_selection("mock", &snap, &deep, 1101).unwrap();
    assert!(
        plan.actions.iter().any(|a| matches!(
            a,
            RefreshAction::FetchSubstash { parent, id, .. } if parent == "m1" && id == "s1"
        )),
        "{:?}",
        plan.actions
    );
    // the envelope round-trips; both authorities, or neither, refused
    let value = serde_json::to_value(&plan).unwrap();
    assert_eq!(RefreshPlan::from_value(&value).unwrap(), plan);
    let mut both = value.clone();
    both["basis"]["policy_revision"] = json!(1);
    assert!(RefreshPlan::from_value(&both).is_err());
    let mut neither = value;
    neither["basis"]
        .as_object_mut()
        .unwrap()
        .remove("selection");
    assert!(RefreshPlan::from_value(&neither).is_err());
    // a policy plan whose row moved is still refused
    let mut a = a;
    put_sync_policy(
        &mut a,
        &all_policy_value(3600),
        None,
        PolicyWrite::Replace,
        &via_test(),
    )
    .unwrap();
    let policy_plan = plan_refresh(
        "mock",
        &s.refresh_snapshot("pc", "Standard", &a).unwrap(),
        1200,
    )
    .unwrap();
    put_sync_policy(
        &mut a,
        &all_policy_value(60),
        Some(1),
        PolicyWrite::Replace,
        &via_test(),
    )
    .unwrap();
    let err = policy_plan
        .check_spendable("mock", Some("u-1"), &a)
        .unwrap_err();
    assert!(matches!(err, SpendError::PolicyMoved { .. }), "{err}");
}

/// C76 (one door: `--characters` among the selection flags) and C55 (a
/// character's id is its identity, its name its address): a
/// selection naming characters by id compiles to the realm's
/// character listing, refreshed as a selection's listing is, and a
/// fetch of each named character that the listing holds in the
/// league, addressed by its name, however fresh; a character not
/// named is not fetched, and an id the facts do not hold is reported,
/// never fetched.
#[test]
fn c76_a_selection_naming_characters_fetches_each_by_its_id() {
    let mut s = store();
    list_characters(
        &mut s,
        "pc",
        json!([
            entry("c1", "Alpha", "Standard", 10),
            entry("c2", "Beta", "Standard", 20),
        ]),
        1000,
    );
    fetch_character(&mut s, "pc", body("c1", "Alpha", "Standard", 10), 1100);
    let a = Annotations::open_memory_for("u-1").unwrap();
    let snap = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    let selection = PlanSelection {
        tabs: None,
        characters: vec!["c1".into(), "gone".into()],
        deep: false,
    };
    let plan = plan_selection("mock", &snap, &selection, 1101).unwrap();
    assert_eq!(
        plan.actions,
        [
            RefreshAction::ListCharacters {
                realm: Realm::Pc,
                reason: ListingReason::Selected,
            },
            RefreshAction::FetchCharacter {
                realm: Realm::Pc,
                league: "Standard".into(),
                id: "c1".into(),
                name: "Alpha".into(),
                reason: FetchReason::Selected,
            },
        ]
    );
    assert_eq!(plan.unknown_characters, ["gone"]);
    assert_eq!(plan.basis.selection.as_ref(), Some(&selection));
    assert_eq!(plan.basis.policy_revision, None);
}

/// C76 under the realm rule a policy keeps: a selection naming tabs
/// under a realm the stash endpoints do not take (poe2) is refused
/// before a plan exists, in the policy's own words, so no plan holds
/// stash work the daemon would refuse whole at the apply; characters
/// alone under that realm still compile.
#[test]
fn c76_a_selection_naming_tabs_under_poe2_is_refused_as_a_policy_is() {
    let s = store();
    let a = Annotations::open_memory_for("u-1").unwrap();
    let snap = s.refresh_snapshot("poe2", "Standard", &a).unwrap();
    for tabs in [Selection::All, Selection::Ids(vec!["t1".into()])] {
        let selection = PlanSelection {
            tabs: Some(tabs),
            characters: vec!["c1".into()],
            deep: false,
        };
        let err = plan_selection("mock", &snap, &selection, 1).unwrap_err();
        assert_eq!(err, PlanError::TabsNotTaken { realm: Realm::Poe2 });
        assert_eq!(
            err.to_string(),
            "tabs under realm poe2: the stash endpoints do not take it (PoE1 only)"
        );
    }
    let characters = PlanSelection {
        tabs: None,
        characters: vec!["c1".into()],
        deep: false,
    };
    let plan = plan_selection("mock", &snap, &characters, 1).unwrap();
    assert!(
        plan.actions
            .iter()
            .all(|a| matches!(a, RefreshAction::ListCharacters { .. })),
        "{:?}",
        plan.actions
    );
}

/// C111 as amended over a selection plan, by response and never the
/// clock: a tab fetched, then a selection compiled from a snapshot
/// taken in that same second, still sends it — the fetch landed in the
/// response the snapshot was read under, not after it; once the apply's
/// own fetches land, in later responses of the same second, the same
/// envelope applied again sends nothing. The second is staged, the
/// snapshot's `taken_at` set to the fetch's: no clock is read, and the
/// responses keep their real order.
#[test]
fn c111_a_selection_sends_what_was_fetched_in_its_own_second_and_once() {
    const SECOND: i64 = 2000;
    let mut s = store();
    list(
        &mut s,
        json!([{ "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 }]),
        1000,
    );
    let a = Annotations::open_memory_for("u-1").unwrap();
    fetch(
        &mut s,
        json!({ "id": "t1", "name": "One", "type": "PremiumStash", "items": [] }),
        SECOND,
    );
    let in_its_second = |s: &Store| {
        let mut snapshot = s.refresh_snapshot("pc", "Standard", &a).unwrap();
        snapshot.taken_at = SECOND;
        snapshot
    };
    let selection = PlanSelection {
        tabs: Some(Selection::Ids(vec!["t1".into()])),
        characters: Vec::new(),
        deep: false,
    };
    let plan = plan_selection("mock", &in_its_second(&s), &selection, SECOND).unwrap();
    assert_eq!(plan.basis.snapshot_taken_at, SECOND);
    let (unspent, spent) = plan.unspent(&in_its_second(&s), &[], "tom");
    assert_eq!((unspent.len(), spent.len()), (2, 0), "{spent:?}");
    // the apply's fetches land, in later responses, in the same second
    list(
        &mut s,
        json!([{ "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 }]),
        SECOND,
    );
    fetch(
        &mut s,
        json!({ "id": "t1", "name": "One", "type": "PremiumStash", "items": [] }),
        SECOND,
    );
    let (unspent, spent) = plan.unspent(&in_its_second(&s), &[], "tom");
    assert_eq!((unspent.len(), spent.len()), (0, 2));
}

/// C44 — **fact drift does not refuse**: the authorization is the
/// bounded action set, not a world-state assertion. A listing that
/// lands between compile and apply changes what the next plan will
/// say, never whether this one may be spent.
#[test]
fn c44_fact_drift_never_refuses() {
    let mut s = store();
    list(
        &mut s,
        json!([{ "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 }]),
        1000,
    );
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    put_sync_policy(
        &mut a,
        &all_policy_value(3600),
        None,
        PolicyWrite::Replace,
        &via_test(),
    )
    .unwrap();
    let plan = plan_refresh(
        "mock",
        &s.refresh_snapshot("pc", "Standard", &a).unwrap(),
        1100,
    )
    .unwrap();
    assert_eq!(plan.logical_requests, 1, "{:?}", plan.actions);
    // The world moves: a newer listing drops t1 and adds t2, and t1's
    // fetch lands. The plan's basis is now stale on every axis.
    list(
        &mut s,
        json!([{ "id": "t2", "name": "Two", "type": "PremiumStash", "index": 0 }]),
        1200,
    );
    fetch(
        &mut s,
        json!({ "id": "t1", "name": "One", "type": "PremiumStash", "items": [] }),
        1300,
    );
    let fresh = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    assert_ne!(fresh.stash_listing, plan.basis.stash_listing);
    // Still spendable: only intent gates. The next plan reconciles.
    plan.check_spendable("mock", Some("u-1"), &a).unwrap();
    let next = plan_refresh("mock", &fresh, 1400).unwrap();
    assert!(
        next.actions
            .iter()
            .any(|x| matches!(x, RefreshAction::FetchTab { id, .. } if id == "t2")),
        "{:?}",
        next.actions
    );
}

/// C43 — what apply submits is exactly the plan's tuples, plus the caller's logical budget (C42).
#[test]
fn apply_params_renders_exactly_the_plans_actions() {
    let plan = plan(&store(), &all_policy_value(3600), 2_000);
    assert!(!plan.actions.is_empty());
    let params = apply_params_of(&plan.actions, None);
    let jobs = params["jobs"].as_array().unwrap();
    assert_eq!(jobs.len(), plan.actions.len());
    for (job, action) in jobs.iter().zip(&plan.actions) {
        let (kind, action_params) = action.job();
        assert_eq!(job["kind"], json!(kind));
        assert_eq!(job["params"], action_params);
    }
    assert!(params.get("max_requests").is_none());
    assert_eq!(
        apply_params_of(&plan.actions, Some(3))["max_requests"],
        json!(3)
    );
}

/// C111 — an open apply of the account holds the requests its
/// `params.jobs` lists before its fan-out submits them, compared as
/// the store decodes a job; a child holding the same request is named
/// before it; a finished apply and another account's spend nothing.
#[test]
fn an_open_apply_spends_the_requests_it_lists_and_a_child_is_named_first() {
    use acquisition_protocol::job::JobState;
    let mut s = store();
    list(
        &mut s,
        json!([
            { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 },
            { "id": "t2", "name": "Two", "type": "PremiumStash", "index": 1 },
            { "id": "t3", "name": "Three", "type": "PremiumStash", "index": 2 },
        ]),
        1000,
    );
    let policy = all_policy_value(3600);
    let plan = plan(&s, &policy, 1400);
    assert_eq!(plan.actions.len(), 3, "{:?}", plan.actions);
    let facts = snapshot_with(&s, &policy);
    let id_of = |action: &RefreshAction| match action {
        RefreshAction::FetchTab { id, .. } => id.clone(),
        other => panic!("{other:?}"),
    };
    let (t1, t2, t3) = (
        id_of(&plan.actions[0]),
        id_of(&plan.actions[1]),
        id_of(&plan.actions[2]),
    );
    let job = |id, kind: &str, account: &str, state, parent, params| JobInfo {
        id,
        kind: kind.into(),
        state,
        priority: 0,
        submitted_by: "test".into(),
        eta_seconds: None,
        parent,
        retries: 0,
        account: Some(account.into()),
        params,
    };
    // The defaults omitted (realm, deep), as another door may spell it.
    let listed = |ids: &[&str]| {
        let jobs: Vec<Value> = ids
            .iter()
            .map(|id| json!({ "kind": "stash", "params": { "league": "Standard", "id": id } }))
            .collect();
        json!({ "jobs": jobs })
    };
    let live = [
        job(
            20,
            "apply",
            "tom",
            JobState::Waiting,
            None,
            listed(&[&t1, &t2]),
        ),
        job(
            21,
            "stash",
            "tom",
            JobState::Waiting,
            Some(20),
            plan.actions[1].job().1,
        ),
        job(22, "apply", "tom", JobState::Done, None, listed(&[&t3])),
        job(
            23,
            "apply",
            "someone-else",
            JobState::Running,
            None,
            listed(&[&t3]),
        ),
    ];
    let (unspent, spent) = plan.unspent(&facts, &live, "tom");
    assert_eq!(unspent, plan.actions[2..].to_vec());
    assert_eq!(
        spent,
        vec![
            Spent {
                action: plan.actions[0].clone(),
                by: SpentBy::Queued {
                    job_id: 20,
                    parent: None
                },
            },
            Spent {
                action: plan.actions[1].clone(),
                by: SpentBy::Queued {
                    job_id: 21,
                    parent: Some(20)
                },
            },
        ]
    );
}

/// C111 — an action is spent by its target's fetch in a response after
/// the one the plan's snapshot was read under, or by an open job of
/// the same account holding the same request; a fetch in the
/// snapshot's own response (whatever its second), a finished job and
/// another account's job spend nothing.
#[test]
fn an_action_is_spent_by_a_later_fetch_or_an_open_job_and_by_nothing_else() {
    use acquisition_protocol::job::JobState;
    let mut s = store();
    list(
        &mut s,
        json!([
            { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 },
            { "id": "t2", "name": "Two", "type": "PremiumStash", "index": 1 },
            { "id": "t3", "name": "Three", "type": "PremiumStash", "index": 2 },
        ]),
        1000,
    );
    let policy = all_policy_value(3600);
    let plan = plan(&s, &policy, 1400);
    assert_eq!(plan.actions.len(), 3, "{:?}", plan.actions);
    let taken = plan.basis.snapshot_taken_at;

    let facts = snapshot_with(&s, &policy);
    let (unspent, spent) = plan.unspent(&facts, &[], "tom");
    assert_eq!((unspent.len(), spent.len()), (3, 0));

    // C111 as amended, by response and never the clock: t1's fetch
    // landed in a response after the snapshot's; t2's in the snapshot's
    // own response, in the snapshot's own second — not after it
    let mut facts = facts;
    let key = |id: &str| format!("stash/pc/Standard/{id}/");
    let read_under = plan.basis.snapshot_response;
    let landed = |response_id| Fetched {
        response_id,
        fetched_at: taken,
    };
    facts.fetches.insert(key("t1"), landed(read_under + 1));
    facts.fetches.insert(key("t2"), landed(read_under));
    let job = |id, account: &str, state| JobInfo {
        id,
        kind: "stash".into(),
        state,
        priority: 0,
        submitted_by: "test".into(),
        eta_seconds: None,
        parent: Some(7),
        retries: 0,
        account: Some(account.into()),
        params: plan.actions[2].job().1,
    };
    let live = [
        job(8, "tom", JobState::Done),
        job(9, "someone-else", JobState::Waiting),
    ];
    let (unspent, spent) = plan.unspent(&facts, &live, "tom");
    assert_eq!(unspent, plan.actions[1..].to_vec());
    assert_eq!(
        spent,
        vec![Spent {
            action: plan.actions[0].clone(),
            by: SpentBy::Fetched {
                fetched_at: taken,
                fetched_at_age: None
            },
        }]
    );

    let live = [job(10, "tom", JobState::Waiting)];
    let (unspent, spent) = plan.unspent(&facts, &live, "tom");
    assert_eq!(unspent, plan.actions[1..2].to_vec());
    assert_eq!(
        spent[1].by,
        SpentBy::Queued {
            job_id: 10,
            parent: Some(7)
        }
    );
    assert_eq!(
        serde_json::to_value(&spent[1]).unwrap(),
        json!({ "action": plan.actions[2], "job_id": 10, "parent": 7 })
    );
}

/// C111 — a spent entry names the time of the response it was spent
/// by, never its target's current location's: a tab fetched after the
/// plan, then retired by a listing, is spent by that fetch, at the
/// fetch's own time (the fix round's review, its third finding).
#[test]
fn a_retired_tab_is_spent_at_its_real_fetch_time() {
    let mut s = store();
    list(
        &mut s,
        json!([{ "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 }]),
        1000,
    );
    let policy = all_policy_value(3600);
    let plan = plan(&s, &policy, 1400);
    assert_eq!(plan.actions.len(), 1, "{:?}", plan.actions);
    fetch(
        &mut s,
        json!({ "id": "t1", "name": "One", "type": "PremiumStash", "items": [] }),
        1500,
    );
    list(&mut s, json!([]), 1600);
    let facts = snapshot_with(&s, &policy);
    // the case staged: the retired tab is in no location to read
    assert!(facts.tabs.iter().all(|t| t.id != "t1"), "{:?}", facts.tabs);
    let (unspent, spent) = plan.unspent(&facts, &[], "tom");
    assert!(unspent.is_empty(), "{unspent:?}");
    assert_eq!(
        spent,
        vec![Spent {
            action: plan.actions[0].clone(),
            by: SpentBy::Fetched {
                fetched_at: 1500,
                fetched_at_age: None
            },
        }]
    );
}

/// C111 — a job is read by the request it names, not by how its params
/// are spelt: `acq stash <tab>` queues `"sub": null` where a plan's
/// action names no sub, and that open job spends the action (the
/// seat: an apply behind a waiting stash fetch queued it again).
#[test]
fn an_open_job_spelling_no_sub_as_null_spends_the_action_naming_none() {
    use acquisition_protocol::job::JobState;
    let mut s = store();
    list(
        &mut s,
        json!([
            { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 },
            { "id": "t2", "name": "Two", "type": "PremiumStash", "index": 1 },
        ]),
        1000,
    );
    let policy = all_policy_value(3600);
    let plan = plan(&s, &policy, 1400);
    assert_eq!(plan.actions.len(), 2, "{:?}", plan.actions);
    let (kind, mut params) = plan.actions[0].job();
    assert!(params.get("sub").is_none(), "{params}");
    params["sub"] = Value::Null;
    let live = [JobInfo {
        id: 30,
        kind: kind.into(),
        state: JobState::Waiting,
        priority: 0,
        submitted_by: "cli".into(),
        eta_seconds: None,
        parent: None,
        retries: 0,
        account: Some("tom".into()),
        params,
    }];
    let facts = snapshot_with(&s, &policy);
    let (unspent, spent) = plan.unspent(&facts, &live, "tom");
    assert_eq!(unspent, plan.actions[1..].to_vec());
    assert_eq!(
        spent,
        vec![Spent {
            action: plan.actions[0].clone(),
            by: SpentBy::Queued {
                job_id: 30,
                parent: None
            },
        }]
    );
}

/// Realm rides through: a plan for Standard on xbox lists and fetches
/// under xbox and its tuples say so explicitly; the pc snapshot of
/// the same league is not covered by an xbox-only policy.
#[test]
fn a_plan_is_for_one_realm_and_its_tuples_say_which() {
    let mut s = store();
    s.record(
            &Endpoint::Stashes {
                realm: "xbox".into(),
                league: "Standard".into(),
            },
            &json!({ "realm": "xbox", "league": "Standard" }),
            200,
            &json!({ "stashes": [ { "id": "x1", "name": "Console", "type": "PremiumStash", "index": 0 } ] }),
            1000,
        )
        .unwrap();
    let policy = json!({
        "version": 2,
        "realms": { "xbox": { "leagues": { "Standard": { "tabs": "all", "max_age_seconds": 60 } } } }
    });
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    store_policy(&mut a, &policy);
    let snap = s.refresh_snapshot("xbox", "Standard", &a).unwrap();
    let plan = plan_refresh("mock", &snap, 1500).unwrap();
    assert_eq!(plan.realm, Realm::Xbox);
    let jobs: Vec<(String, Value)> = plan
        .actions
        .iter()
        .map(|a| {
            let (k, p) = a.job();
            (k.into(), p)
        })
        .collect();
    assert_eq!(
        jobs,
        vec![
            (
                "stashes".into(),
                json!({ "realm": "xbox", "league": "Standard" })
            ),
            (
                "stash".into(),
                json!({ "realm": "xbox", "league": "Standard", "id": "x1", "deep": false })
            ),
        ]
    );
    assert_eq!(
        Endpoint::from_job(&jobs[1].0, &jobs[1].1),
        Some(Endpoint::Stash {
            realm: "xbox".into(),
            league: "Standard".into(),
            id: "x1".into(),
            sub: None
        })
    );
    let pc = s.refresh_snapshot("pc", "Standard", &a).unwrap();
    assert!(matches!(
        plan_refresh("mock", &pc, 1500).unwrap_err(),
        PlanError::LeagueNotCovered {
            realm: Realm::Pc,
            ..
        }
    ));
    // The envelope round-trips with its realm.
    let json = serde_json::to_value(&plan).unwrap();
    assert_eq!(json["realm"], "xbox");
    assert_eq!(RefreshPlan::from_value(&json).unwrap(), plan);
}

/// A tab a listing dropped and a later listing revived has no live
/// facts (they were retired with it) and no fetch on record, so the
/// plan fetches it again — the loop, not a manual step, restores it.
#[test]
fn a_revived_tab_is_planned_as_never_fetched() {
    let mut s = store();
    let t1 = json!([{ "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 }]);
    list(&mut s, t1.clone(), 1000);
    fetch(
        &mut s,
        json!({ "id": "t1", "name": "One", "type": "PremiumStash", "items": [item("i1")] }),
        1100,
    );
    list(&mut s, json!([]), 1200);
    list(&mut s, t1, 1300);
    let plan = plan(&s, &all_policy_value(3600), 1400);
    assert!(
        matches!(
            plan.actions[..],
            [RefreshAction::FetchTab { ref id, reason: FetchReason::NeverFetched, .. }] if id == "t1"
        ),
        "{:?}",
        plan.actions
    );
}

/// A map tab a listing dropped and a later listing revived: its
/// substashes kept their rows (orphaned, then re-parented) but lost
/// their facts with the parent, so the plan fetches the parent *and*
/// each substash again — nothing waits out a freshness window empty.
#[test]
fn a_revived_parent_replans_its_substashes() {
    let mut s = store();
    let m1 = json!([{ "id": "m1", "name": "Maps", "type": "MapStash", "index": 0 }]);
    list(&mut s, m1.clone(), 1000);
    fetch(
        &mut s,
        json!({ "id": "m1", "name": "Maps", "type": "MapStash", "items": [],
                "children": [ { "id": "s1", "name": "", "type": "MapStash", "parent": "m1", "metadata": { "items": 1 } } ] }),
        1100,
    );
    s.record(
            &Endpoint::Stash {
                realm: "pc".into(),
                league: "Standard".into(),
                id: "m1".into(),
                sub: Some("s1".into()),
            },
            &json!({ "league": "Standard", "id": "m1", "sub": "s1" }),
            200,
            &json!({ "stash": { "id": "s1", "name": "", "type": "MapStash", "parent": "m1", "items": [item("map1")] } }),
            1200,
        )
        .unwrap();
    list(&mut s, json!([]), 1300);
    list(&mut s, m1, 1400);
    let plan = plan(&s, &all_policy_value(3600), 1500);
    let mut kinds: Vec<(String, String)> = plan
        .actions
        .iter()
        .map(|a| match a {
            RefreshAction::FetchTab { id, reason, .. } => {
                (format!("tab {id}"), format!("{reason:?}"))
            }
            RefreshAction::FetchSubstash { id, reason, .. } => {
                (format!("sub {id}"), format!("{reason:?}"))
            }
            RefreshAction::ListStashes { .. } => ("list".into(), String::new()),
            other => panic!("no character work in a tabs-only policy: {other:?}"),
        })
        .collect();
    kinds.sort();
    assert_eq!(
        kinds,
        vec![
            ("sub s1".to_string(), "NeverFetched".to_string()),
            ("tab m1".to_string(), "NeverFetched".to_string()),
        ],
        "{:?}",
        plan.actions
    );
}

// ---- characters (step 3, 2026-09-02) ------------------------------------

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

fn entry(id: &str, name: &str, league: &str, experience: i64) -> Value {
    json!({ "id": id, "name": name, "league": league, "class": "Witch", "level": 90, "experience": experience })
}

fn body(id: &str, name: &str, league: &str, experience: i64) -> Value {
    let mut b = entry(id, name, league, experience);
    b["inventory"] = json!([item(&format!("{id}-i1"))]);
    b["equipment"] = json!([]);
    b
}

/// A v3 policy for pc/Standard with the given facets (`None` leaves
/// the facet out).
fn v3_policy_value(tabs: Option<Value>, characters: Option<Value>, max_age_seconds: u32) -> Value {
    let mut league = json!({ "max_age_seconds": max_age_seconds });
    if let Some(t) = tabs {
        league["tabs"] = t;
    }
    if let Some(c) = characters {
        league["characters"] = c;
    }
    json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard": league } } } })
}

fn character_actions(plan: &RefreshPlan) -> Vec<(String, String)> {
    plan.actions
        .iter()
        .filter_map(|a| match a {
            RefreshAction::ListCharacters { reason, .. } => {
                Some(("list".to_string(), format!("{reason:?}")))
            }
            RefreshAction::FetchCharacter { id, reason, .. } => {
                Some((id.clone(), format!("{reason:?}")))
            }
            _ => None,
        })
        .collect()
}

fn character_skips(plan: &RefreshPlan) -> Vec<(String, CharacterSkipReason)> {
    plan.skipped_characters
        .iter()
        .map(|s| (s.id.clone(), s.reason.clone()))
        .collect()
}

#[test]
fn policy_v3_covers_per_facet_and_refuses_an_entry_that_names_no_work() {
    // Both facets; a character-only entry; a tab-only entry — and a
    // character-only PoE2 entry is the ordinary shape.
    let p = policy(json!({
        "version": 3,
        "realms": {
            "pc": { "leagues": {
                "Standard": { "tabs": "all", "characters": ["c1"], "max_age_seconds": 60 },
                "Hardcore": { "characters": "all", "max_age_seconds": 60 },
                "Ruthless": { "tabs": ["t1"], "max_age_seconds": 60 },
            } },
            "poe2": { "leagues": { "Standard": { "characters": "all", "max_age_seconds": 60 } } },
        }
    }));
    assert_eq!(p.version, SYNC_POLICY_VERSION);
    let pc = &p.realms[&Realm::Pc].leagues;
    assert_eq!(
        (&pc["Standard"].tabs, &pc["Standard"].characters),
        (
            &Some(Selection::All),
            &Some(Selection::Ids(vec!["c1".into()]))
        )
    );
    assert_eq!(
        (&pc["Hardcore"].tabs, &pc["Hardcore"].characters),
        (&None, &Some(Selection::All))
    );
    assert_eq!(
        (&pc["Ruthless"].tabs, &pc["Ruthless"].characters),
        (&Some(Selection::Ids(vec!["t1".into()])), &None)
    );
    assert_eq!(p.realms[&Realm::Poe2].leagues["Standard"].tabs, None);
    // The in-memory form serializes as v3 without the absent facets,
    // and parses back equal.
    let json = serde_json::to_value(&p).unwrap();
    assert_eq!(json["version"], 3);
    assert!(
        json["realms"]["pc"]["leagues"]["Hardcore"]
            .get("tabs")
            .is_none()
    );
    assert_eq!(serde_json::from_value::<SyncPolicy>(json).unwrap(), p);
    // v1 and v2 upgrade to tab coverage plus no character coverage.
    let v1 = policy(all_policy_value(60));
    assert_eq!(v1.realms[&Realm::Pc].leagues["Standard"].characters, None);
    // Malformed shapes, each a structured error and never half-honored.
    for (bad, why) in [
        // Tabs under poe2 name a URL shape the stash endpoints lack,
        // whether alone or beside characters.
        (
            json!({ "version": 3, "realms": { "poe2": { "leagues": { "Standard": { "tabs": "all", "max_age_seconds": 60 } } } } }),
            "poe2",
        ),
        (
            json!({ "version": 3, "realms": { "poe2": { "leagues": { "Standard": { "tabs": ["t"], "characters": "all", "max_age_seconds": 60 } } } } }),
            "poe2",
        ),
        // Names no work: neither facet, both empty, one empty and the
        // other absent — judged after normalization; and a v2 entry
        // with an empty list is the same declaration.
        (
            json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard": { "max_age_seconds": 60 } } } } }),
            "names no work",
        ),
        (
            json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard": { "tabs": [], "characters": [], "max_age_seconds": 60 } } } } }),
            "names no work",
        ),
        (
            json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard": { "tabs": [], "max_age_seconds": 60 } } } } }),
            "names no work",
        ),
        (
            json!({ "version": 2, "realms": { "pc": { "leagues": { "Standard": { "tabs": [], "max_age_seconds": 60 } } } } }),
            "names no work",
        ),
        // A selection word other than "all", a stray field in the
        // entry, and `characters` on a v2 stamp (unknown there).
        (
            json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard": { "characters": "everything", "max_age_seconds": 60 } } } } }),
            "everything",
        ),
        (
            json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard": { "characters": "all", "max_age_secs": 60 } } } } }),
            "max_age_secs",
        ),
        (
            json!({ "version": 2, "realms": { "pc": { "leagues": { "Standard": { "tabs": "all", "characters": "all", "max_age_seconds": 60 } } } } }),
            "characters",
        ),
    ] {
        let err = SyncPolicy::from_value(&bad).unwrap_err();
        assert!(
            matches!(&err, PlanError::MalformedPolicy { detail } if detail.contains(why)),
            "{bad}: {err}"
        );
    }
}

/// Facets are independent: a character-only policy on a never-listed
/// league plans the character listing alone — no stash listing, and a
/// character on record from a direct fetch waits for the basis.
#[test]
fn a_never_listed_realm_plans_the_character_listing_alone() {
    let mut s = store();
    fetch_character(&mut s, "pc", body("c1", "Hero", "Standard", 100), 500);
    let p = v3_policy_value(None, Some(json!("all")), 60);
    let plan = plan(&s, &p, 9000);
    assert_eq!(
        character_actions(&plan),
        vec![("list".to_string(), "NeverListed".to_string())]
    );
    assert_eq!(plan.actions.len(), 1, "{:?}", plan.actions);
    assert_eq!(
        character_skips(&plan),
        vec![("c1".to_string(), CharacterSkipReason::AwaitingListing)]
    );
    assert!(plan.skipped_tabs.is_empty() && plan.unknown_tabs.is_empty());
    assert_eq!(
        (plan.basis.stash_listing, plan.basis.character_listing),
        (None, None)
    );
}

#[test]
fn characters_are_covered_by_exact_id_and_unknown_ids_are_reported() {
    let mut s = store();
    list_characters(
        &mut s,
        "pc",
        json!([
            entry("c1", "Hero", "Standard", 100),
            entry("c2", "Mule", "Standard", 5)
        ]),
        1000,
    );
    let p = v3_policy_value(None, Some(json!(["c1", "ghost"])), 3600);
    let plan = plan(&s, &p, 1500);
    assert_eq!(
        character_actions(&plan),
        vec![("c1".to_string(), "NeverFetched".to_string())]
    );
    assert!(
        character_skips(&plan).is_empty(),
        "{:?}",
        plan.skipped_characters
    );
    assert_eq!(plan.unknown_characters, vec!["ghost".to_string()]);
    assert_eq!(plan.basis.character_listing.unwrap().fetched_at, 1000);
    // The action's tuple is the listed name on the character route.
    let RefreshAction::FetchCharacter { name, league, .. } = &plan.actions[0] else {
        panic!("{:?}", plan.actions);
    };
    assert_eq!((name.as_str(), league.as_str()), ("Hero", "Standard"));
}

/// The three arms and the window, for a character: fresh inside the
/// window with an agreeing listing; a newer listing whose
/// `experience` differs proves play since; a same-second listing
/// proves nothing; a body without `experience` proves nothing; past
/// the window is stale whatever the listing says.
#[test]
fn character_freshness_reads_experience_forward_from_a_newer_listing() {
    let mut s = store();
    let p = v3_policy_value(None, Some(json!("all")), 3600);
    list_characters(
        &mut s,
        "pc",
        json!([entry("c1", "Hero", "Standard", 100)]),
        1000,
    );
    fetch_character(&mut s, "pc", body("c1", "Hero", "Standard", 100), 1100);
    assert_eq!(
        character_skips(&plan(&s, &p, 1200)),
        vec![(
            "c1".to_string(),
            CharacterSkipReason::Fresh { age_seconds: 100 }
        )]
    );
    // A newer listing reporting more experience: play since the fetch.
    list_characters(
        &mut s,
        "pc",
        json!([entry("c1", "Hero", "Standard", 150)]),
        1300,
    );
    assert_eq!(
        character_actions(&plan(&s, &p, 1400)),
        vec![(
            "c1".to_string(),
            "ListedExperienceDisagrees { listed: 150, held: 100 }".to_string()
        )]
    );
    // Fetched again, the listing agrees: fresh.
    fetch_character(&mut s, "pc", body("c1", "Hero", "Standard", 150), 1500);
    list_characters(
        &mut s,
        "pc",
        json!([entry("c1", "Hero", "Standard", 150)]),
        1600,
    );
    assert!(character_actions(&plan(&s, &p, 1700)).is_empty());
    // Same second as the fetch: the listing is not newer, so a
    // different value is no proof (the fetch may have landed after).
    list_characters(
        &mut s,
        "pc",
        json!([entry("c1", "Hero", "Standard", 999)]),
        1500,
    );
    assert!(character_actions(&plan(&s, &p, 1700)).is_empty());
    // A body without `experience` (an older build's fetch, a field GGG
    // dropped): no comparison, no proof.
    let mut no_exp = body("c1", "Hero", "Standard", 0);
    no_exp.as_object_mut().unwrap().remove("experience");
    fetch_character(&mut s, "pc", no_exp, 1800);
    list_characters(
        &mut s,
        "pc",
        json!([entry("c1", "Hero", "Standard", 1234)]),
        1900,
    );
    assert!(character_actions(&plan(&s, &p, 2000)).is_empty());
    // Past the window: stale, plainly.
    assert_eq!(
        character_actions(&plan(&s, &p, 1800 + 3601)),
        vec![("c1".to_string(), "Stale { age_seconds: 3601 }".to_string())]
    );
}

/// A Hardcore death: the listing moves the character to Standard (the
/// row's league is listing-owned), so the Hardcore plan no longer
/// sees it and the Standard plan fetches it — the body still says
/// Hardcore, and that disagreement is the arm.
#[test]
fn a_league_move_forces_a_fetch_in_the_new_league_and_leaves_the_old() {
    let mut s = store();
    let both = json!({ "version": 3, "realms": { "pc": { "leagues": {
            "Standard": { "characters": "all", "max_age_seconds": 3600 },
            "Hardcore": { "characters": "all", "max_age_seconds": 3600 },
        } } } });
    list_characters(
        &mut s,
        "pc",
        json!([entry("c1", "Hero", "Hardcore", 100)]),
        1000,
    );
    fetch_character(&mut s, "pc", body("c1", "Hero", "Hardcore", 100), 1100);
    list_characters(
        &mut s,
        "pc",
        json!([entry("c1", "Hero", "Standard", 100)]),
        1200,
    );
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    store_policy(&mut a, &both);
    let hc = plan_refresh(
        "mock",
        &s.refresh_snapshot("pc", "Hardcore", &a).unwrap(),
        1300,
    )
    .unwrap();
    assert!(
        hc.actions.is_empty() && hc.skipped_characters.is_empty(),
        "{hc:?}"
    );
    let std = plan_refresh(
        "mock",
        &s.refresh_snapshot("pc", "Standard", &a).unwrap(),
        1300,
    )
    .unwrap();
    assert_eq!(
        character_actions(&std),
        vec![(
            "c1".to_string(),
            "ListedLeagueDisagrees { listed: \"Standard\", held: \"Hardcore\" }".to_string()
        )]
    );
    let RefreshAction::FetchCharacter { league, .. } = &std.actions[0] else {
        panic!("{:?}", std.actions);
    };
    assert_eq!(league, "Standard");
}

/// Listed `deleted` / `expired` flags skip with a named reason, never
/// a fetch (a 404 hunt counts against the invalid-request threshold);
/// a character the listing gave no league is outside every league's
/// coverage and is reported as such — by id too, never as unknown.
#[test]
fn deleted_expired_and_league_less_characters_are_skipped_never_fetched() {
    let mut s = store();
    let mut gone = entry("c-del", "Gone", "Standard", 1);
    gone["deleted"] = json!(true);
    let mut old = entry("c-exp", "Old", "Standard", 1);
    old["expired"] = json!(true);
    let nowhere = json!({ "id": "c-none", "name": "Nowhere", "level": 1 });
    list_characters(
        &mut s,
        "pc",
        json!([entry("c1", "Hero", "Standard", 100), gone, old, nowhere]),
        1000,
    );
    let plan_all = plan(&s, &v3_policy_value(None, Some(json!("all")), 3600), 1500);
    assert_eq!(
        character_actions(&plan_all),
        vec![("c1".to_string(), "NeverFetched".to_string())]
    );
    let mut skips = character_skips(&plan_all);
    skips.sort_by(|a, b| a.0.cmp(&b.0));
    assert_eq!(
        skips,
        vec![
            ("c-del".to_string(), CharacterSkipReason::Deleted),
            ("c-exp".to_string(), CharacterSkipReason::Expired),
            ("c-none".to_string(), CharacterSkipReason::NoLeague),
        ]
    );
    let by_id = plan(
        &s,
        &v3_policy_value(None, Some(json!(["c-none"])), 3600),
        1500,
    );
    assert!(by_id.actions.is_empty());
    assert_eq!(
        character_skips(&by_id),
        vec![("c-none".to_string(), CharacterSkipReason::NoLeague)]
    );
    assert!(by_id.unknown_characters.is_empty());
    // The Hardcore plan of the same realm reports the league-less
    // character too — it is a realm fact no league key reaches.
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    let hc_policy = json!({ "version": 3, "realms": { "pc": { "leagues": {
            "Hardcore": { "characters": "all", "max_age_seconds": 3600 } } } } });
    store_policy(&mut a, &hc_policy);
    let hc = plan_refresh(
        "mock",
        &s.refresh_snapshot("pc", "Hardcore", &a).unwrap(),
        1500,
    )
    .unwrap();
    assert_eq!(
        character_skips(&hc),
        vec![("c-none".to_string(), CharacterSkipReason::NoLeague)]
    );
}

/// Dropped by one listing, revived by the next, no fetch between: the
/// store cleared `fetched_at`, so the plan fetches again — the loop
/// restores the facts, not a manual step. And a late fetch under the
/// retired row's name meanwhile is withheld and changes nothing here.
#[test]
fn a_revived_character_is_planned_as_never_fetched() {
    let mut s = store();
    let p = v3_policy_value(None, Some(json!("all")), 3600);
    list_characters(
        &mut s,
        "pc",
        json!([entry("c1", "Hero", "Standard", 100)]),
        1000,
    );
    fetch_character(&mut s, "pc", body("c1", "Hero", "Standard", 100), 1100);
    list_characters(&mut s, "pc", json!([]), 1200);
    let gone = plan(&s, &p, 1250);
    assert!(
        gone.actions.is_empty() && gone.skipped_characters.is_empty(),
        "{gone:?}"
    );
    // The late fetch at the retired location: withheld by the store.
    fetch_character(&mut s, "pc", body("c1", "Hero", "Standard", 120), 1260);
    let still_gone = plan(&s, &p, 1270);
    assert!(still_gone.actions.is_empty(), "{:?}", still_gone.actions);
    list_characters(
        &mut s,
        "pc",
        json!([entry("c1", "Hero", "Standard", 120)]),
        1300,
    );
    assert_eq!(
        character_actions(&plan(&s, &p, 1400)),
        vec![("c1".to_string(), "NeverFetched".to_string())]
    );
}

/// A deleted-and-recreated name: the listing names a new id, the old
/// row is retired with its facts, and the plan fetches the new id
/// (never fetched) by the same name — the old id is gone, and a
/// policy still naming it reports it unknown.
#[test]
fn a_recreated_name_plans_the_new_id_only() {
    let mut s = store();
    list_characters(
        &mut s,
        "pc",
        json!([entry("c1", "Hero", "Standard", 100)]),
        1000,
    );
    fetch_character(&mut s, "pc", body("c1", "Hero", "Standard", 100), 1100);
    list_characters(
        &mut s,
        "pc",
        json!([entry("c2", "Hero", "Standard", 0)]),
        1200,
    );
    let all = plan(&s, &v3_policy_value(None, Some(json!("all")), 3600), 1300);
    assert_eq!(
        character_actions(&all),
        vec![("c2".to_string(), "NeverFetched".to_string())]
    );
    let RefreshAction::FetchCharacter { name, .. } = &all.actions[0] else {
        panic!("{:?}", all.actions);
    };
    assert_eq!(name, "Hero");
    let old = plan(&s, &v3_policy_value(None, Some(json!(["c1"])), 3600), 1300);
    assert!(old.actions.is_empty());
    assert_eq!(old.unknown_characters, vec!["c1".to_string()]);
}

/// A pre-v4 row: no listed entry on record (the migration could not
/// recompute one), a fetched body present. It plans by the window
/// alone — no flags to read, no experience to compare — and panics on
/// nothing.
#[test]
fn a_character_row_without_a_listed_entry_plans_by_the_window_alone() {
    let mut s = store();
    list_characters(
        &mut s,
        "pc",
        json!([entry("c1", "Hero", "Standard", 100)]),
        1000,
    );
    fetch_character(&mut s, "pc", body("c1", "Hero", "Standard", 100), 1100);
    let p = v3_policy_value(None, Some(json!("all")), 3600);
    let mut snap = snapshot_with(&s, &p);
    snap.characters[0].listed = Value::Null;
    snap.characters[0].listed_at = Some(1200);
    let fresh = plan_refresh("mock", &snap, 1300).unwrap();
    assert_eq!(
        character_skips(&fresh),
        vec![(
            "c1".to_string(),
            CharacterSkipReason::Fresh { age_seconds: 200 }
        )]
    );
    let stale = plan_refresh("mock", &snap, 1100 + 3601).unwrap();
    assert_eq!(
        character_actions(&stale),
        vec![
            (
                "list".to_string(),
                "Stale { age_seconds: 3701 }".to_string()
            ),
            ("c1".to_string(), "Stale { age_seconds: 3601 }".to_string()),
        ]
    );
}

/// The character actions decode through the store's own job
/// vocabulary decoder, and a mixed plan's loop closes through the
/// store: two listings, then the fetches each facet owes, then empty.
#[test]
fn a_mixed_plan_closes_through_the_stores_vocabulary() {
    let list = RefreshAction::ListCharacters {
        realm: Realm::Poe2,
        reason: ListingReason::NeverListed,
    };
    let (kind, params) = list.job();
    assert_eq!(
        Endpoint::from_job(kind, &params),
        Some(Endpoint::Characters {
            realm: "poe2".into()
        })
    );
    let fetch = RefreshAction::FetchCharacter {
        realm: Realm::Pc,
        league: "Standard".into(),
        id: "c1".into(),
        name: "Hero".into(),
        reason: FetchReason::NeverFetched,
    };
    let (kind, params) = fetch.job();
    assert_eq!(params, json!({ "realm": "pc", "name": "Hero" }));
    assert_eq!(
        Endpoint::from_job(kind, &params),
        Some(Endpoint::Character {
            realm: "pc".into(),
            name: "Hero".into()
        })
    );

    let mut s = store();
    let p = v3_policy_value(Some(json!("all")), Some(json!("all")), 3600);
    let first = plan(&s, &p, 1000);
    assert!(
        matches!(
            first.actions[..],
            [
                RefreshAction::ListStashes { .. },
                RefreshAction::ListCharacters { .. }
            ]
        ),
        "{:?}",
        first.actions
    );
    let answer = |action: &RefreshAction| match action {
        RefreshAction::ListStashes { .. } => {
            json!({ "stashes": [ { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 } ] })
        }
        RefreshAction::ListCharacters { .. } => {
            json!({ "characters": [ entry("c1", "Hero", "Standard", 100) ] })
        }
        RefreshAction::FetchTab { .. } | RefreshAction::FetchSubstash { .. } => {
            json!({ "stash": { "id": "t1", "name": "One", "type": "PremiumStash", "items": [item("i1")] } })
        }
        RefreshAction::FetchCharacter { .. } => {
            json!({ "character": body("c1", "Hero", "Standard", 100) })
        }
    };
    let apply = |s: &mut Store, plan: &RefreshPlan, at: i64| {
        for action in &plan.actions {
            let (kind, params) = action.job();
            let endpoint = Endpoint::from_job(kind, &params).unwrap();
            s.record(&endpoint, &params, 200, &answer(action), at)
                .unwrap();
        }
    };
    apply(&mut s, &first, 1000);
    let second = plan(&s, &p, 1001);
    assert!(
        matches!(
            second.actions[..],
            [
                RefreshAction::FetchTab { .. },
                RefreshAction::FetchCharacter { .. }
            ]
        ),
        "{:?}",
        second.actions
    );
    // The envelope round-trips with both facets' bases and lists.
    let json = serde_json::to_value(&second).unwrap();
    assert!(
        json["basis"]["stash_listing"].is_object()
            && json["basis"]["character_listing"].is_object()
    );
    assert_eq!(RefreshPlan::from_value(&json).unwrap(), second);
    apply(&mut s, &second, 1001);
    let third = plan(&s, &p, 1002);
    assert!(third.actions.is_empty(), "{:?}", third.actions);
    assert_eq!(third.skipped_tabs.len(), 1);
    assert_eq!(third.skipped_characters.len(), 1);
}

/// The envelope's league check: a realm-wide `list_characters` is in
/// envelope for any league of its realm; a `fetch_character` for
/// another league is refused; a tab skip kind on a character skip is
/// malformed, as is a smuggled field there.
#[test]
fn realm_wide_actions_are_in_envelope_and_character_skips_parse_strictly() {
    let mut s = store();
    let mut gone = entry("c-del", "Gone", "Standard", 1);
    gone["deleted"] = json!(true);
    list_characters(
        &mut s,
        "pc",
        json!([entry("c1", "Hero", "Standard", 100), gone]),
        1000,
    );
    let p = v3_policy_value(None, Some(json!("all")), 60);
    let plan = plan(&s, &p, 5000);
    let good = serde_json::to_value(&plan).unwrap();
    assert_eq!(good["actions"][0]["action"], "list_characters");
    assert!(good["actions"][0].get("league").is_none());
    assert_eq!(good["skipped_characters"][0]["reason"]["kind"], "deleted");
    assert!(RefreshPlan::from_value(&good).is_ok());
    let mut stray = good.clone();
    stray["actions"][1]["league"] = json!("Hardcore");
    let err = RefreshPlan::from_value(&stray).unwrap_err();
    assert!(err.to_string().contains("Hardcore"), "{err}");
    let mut stray = good.clone();
    stray["actions"][0]["realm"] = json!("poe2");
    assert!(RefreshPlan::from_value(&stray).is_err());
    let mut wrong_kind = good.clone();
    wrong_kind["skipped_characters"][0]["reason"] = json!({ "kind": "folder" });
    assert!(RefreshPlan::from_value(&wrong_kind).is_err());
    for path in [
        &["skipped_characters", "0", "surprise"][..],
        &["skipped_characters", "0", "reason", "surprise"],
        &["basis", "character_listing", "extra"],
        &["actions", "0", "surprise"],
    ] {
        let mut nested = good.clone();
        let mut spot = &mut nested;
        for key in &path[..path.len() - 1] {
            spot = match key.parse::<usize>() {
                Ok(i) => &mut spot[i],
                Err(_) => &mut spot[*key],
            };
        }
        spot[*path.last().unwrap()] = json!(true);
        assert!(
            RefreshPlan::from_value(&nested).is_err(),
            "smuggled field at {path:?} was accepted"
        );
    }
}
