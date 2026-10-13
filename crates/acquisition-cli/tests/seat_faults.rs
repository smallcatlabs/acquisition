//! The faults the whole-CLI seat found (2026-10-13,
//! `audits/rehome-2026-10/seat/REPORT.md` at `7653f8b0`), each held by the
//! test named for it. A test here drives the real binary in a scratch
//! world against the mock; nothing reaches GGG.

use std::path::Path;

use acquisition_store::{Endpoint, Index, Store, account_path};
use serde_json::{Value, json};

mod harness;

use harness::{acq, sole_json, start_daemon, text};

/// One account's facts in two leagues of pc, seeded through the store
/// crate with no daemon: Standard holds tab `a1` with item `i-std`,
/// Allflame holds tab `b1` with item `i-all`.
fn seed_two_leagues(base: &Path) {
    const USER: &str = "Alice#1234";
    const UUID: &str = "u-seat";
    let mock = base.join("store").join("mock");
    std::fs::create_dir_all(&mock).unwrap();
    let at = acquisition_store::now();
    Index::load(&mock)
        .unwrap()
        .record_login(USER, UUID, false, at)
        .unwrap();
    let mut store = Store::open(&account_path(&mock, USER)).unwrap();
    let mut record = |ep: Endpoint, params: Value, body: Value| {
        store.record(&ep, &params, 200, &body, at).unwrap();
    };
    record(
        Endpoint::Profile,
        json!({}),
        json!({ "uuid": UUID, "name": USER }),
    );
    for (league, tab, item) in [("Standard", "a1", "i-std"), ("Allflame", "b1", "i-all")] {
        record(
            Endpoint::Stashes {
                realm: "pc".into(),
                league: league.into(),
            },
            json!({ "league": league }),
            json!({ "stashes": [ { "id": tab, "name": "Gear", "type": "PremiumStash", "index": 0 } ] }),
        );
        record(
            Endpoint::Stash {
                realm: "pc".into(),
                league: league.into(),
                id: tab.into(),
                sub: None,
            },
            json!({}),
            json!({ "stash": { "id": tab, "name": "Gear", "type": "PremiumStash", "items": [
                { "id": item, "name": "", "typeLine": "Chaos Orb", "baseType": "Chaos Orb", "x": 0, "y": 0 } ] } }),
        );
    }
}

/// A fresh daemon with nothing to restore said `keyring: ok` under
/// `ACQ_NO_KEYRING=1`: the status started as `ok` and changed only when a
/// save was tried, so the one line that says whether a login will outlive
/// the daemon was wrong until the first login.
#[test]
fn a_fresh_daemon_told_not_to_use_the_keyring_never_reports_it_ok() {
    let scratch = harness::scratch("seat-keyring");
    let base = scratch.0.clone();
    let (_daemon, _pid) = start_daemon(&base);

    let out = acq(&base, &["auth", "status"]);
    let said = text(&out);
    assert!(
        said.contains("keyring: unavailable: disabled by ACQ_NO_KEYRING"),
        "{said}"
    );
    assert!(!said.contains("keyring: ok"), "{said}");

    let status: Value = sole_json(&acq(&base, &["auth", "status", "--json"]));
    assert_eq!(
        status["keyring"], "unavailable: disabled by ACQ_NO_KEYRING",
        "{status}"
    );
}

/// A price target names no league, `set` takes none, and `show` read
/// Standard unless told: the command a `set` printed as its next step
/// failed for every target outside Standard, and `--league` on `set` was
/// a usage error. `show` and `list` now read the league the facts hold
/// their target in, and a wrong `--league` is refused naming the right
/// one.
#[test]
fn a_price_target_outside_standard_reads_back_as_its_set_printed() {
    let scratch = harness::scratch("seat-league");
    let base = scratch.0.clone();
    seed_two_leagues(&base);

    let out = acq(&base, &["price", "set", "tab/pc/b1", "exact", "5", "chaos"]);
    let said = text(&out);
    assert!(out.status.success(), "{said}");
    assert!(
        said.contains("next: `acq price show tab/pc/b1` reads it"),
        "{said}"
    );

    // The printed command, as printed.
    let out = acq(&base, &["price", "show", "tab/pc/b1"]);
    assert!(out.status.success(), "{}", text(&out));
    let view = sole_json(&acq(&base, &["price", "show", "tab/pc/b1", "--json"]));
    assert_eq!(view["league"], "Allflame", "{view}");

    // An item, and a container given to `list`, find their league too.
    let view = sole_json(&acq(&base, &["price", "show", "item/i-all", "--json"]));
    assert_eq!(view["league"], "Allflame", "{view}");
    let view = sole_json(&acq(
        &base,
        &["price", "list", "--in", "tab/pc/b1", "--json"],
    ));
    assert_eq!(view["league"], "Allflame", "{view}");

    // A league the target is not in is refused, naming the one it is in.
    let out = acq(
        &base,
        &["price", "show", "tab/pc/b1", "--league", "Standard"],
    );
    assert!(!out.status.success());
    let said = text(&out);
    assert!(
        said.contains("the facts hold it in Allflame: `--league` reads it there"),
        "{said}"
    );
}

/// `price status` counted another league's rows under "name nothing in
/// these facts", the words for an orphan: of the owner's 16 rows the 11
/// that priced Allflame read as lost in Standard, and the 5 that priced
/// Standard as lost in Allflame. A row the facts hold in another league
/// is counted as that; "names nothing" is left to the row that does.
#[test]
fn another_leagues_price_row_is_never_counted_as_naming_nothing() {
    let scratch = harness::scratch("seat-rows");
    let base = scratch.0.clone();
    seed_two_leagues(&base);
    for target in ["tab/pc/b1", "item/i-all", "tab/pc/gone"] {
        let out = acq(&base, &["price", "set", target, "exact", "5", "chaos"]);
        assert!(out.status.success(), "{}", text(&out));
    }

    let said = text(&acq(&base, &["price", "status"]));
    assert!(
        said.contains(
            "rows: 0 of 3 apply here; 1 name nothing in the facts, 0 unreadable, 2 for other leagues, 0 for other realms"
        ),
        "{said}"
    );
    let report = sole_json(&acq(&base, &["price", "status", "--json"]));
    assert_eq!(
        report["rows"]["other_league"],
        json!([{ "scope": "item", "id": "i-all" }, { "scope": "tab", "realm": "pc", "id": "b1" }]),
        "{report}"
    );
    assert_eq!(
        report["rows"]["unmatched"].as_array().map(Vec::len),
        Some(1)
    );

    let said = text(&acq(&base, &["price", "status", "--league", "Allflame"]));
    assert!(
        said.contains(
            "rows: 2 of 3 apply here; 1 name nothing in the facts, 0 unreadable, 0 for other leagues, 0 for other realms"
        ),
        "{said}"
    );
}
