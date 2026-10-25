//! The moved world (step 4b of the roadmap, `roadmap/briefs/4b-build.md`,
//! Do 9): what a new league brings first — a base the class table does
//! not hold, a mechanic's line in an array of its own with a flag that is
//! no yes or no, and a site search naming a stat the table lacks — is
//! answered and shown, never refused, each carrying the state a reader is
//! told it by: undecided with its reason, the flag unknown and the part
//! unread, the clause not run and the answer not the site's. The web
//! UI's badges render these states and nothing else (the CLI's
//! `tests/web.rs`).

use crate::common;

use acquisition_search::trade::translate::Fidelity;
use acquisition_search::{Classed, Request};
use acquisition_store::Store;
use common::*;
use serde_json::{Value, json};

/// A league's tab holding a ring, a base the class table lacks, and a
/// mechanic's item whose line carries a flag that is no boolean.
fn launch() -> Store {
    let mut s = store();
    list_tabs(&mut s, "pc", "Standard", json!([tab("w1", "Launch")]), 10);
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "w1",
        "Launch",
        vec![
            item(
                "ring",
                "Doom Loop",
                "Two-Stone Ring",
                "Rare",
                json!({ "explicitMods": ["+95 to maximum Life"] }),
            ),
            item(
                "new",
                "Brood Lure",
                "Wombgift Chalice",
                "Rare",
                json!({ "explicitMods": ["+40 to maximum Life"] }),
            ),
            item(
                "mech",
                "Echo Seed",
                "Two-Stone Ring",
                "Rare",
                json!({ "wombgiftMods": [
                { "description": "Grants 3 Echoes of the Brood", "flags": { "echoing": "partly" } } ] }),
            ),
        ],
        20,
    );
    s
}

fn asked(s: &Store, request: Value) -> Value {
    let request: Request = serde_json::from_value(request).unwrap();
    as_json(&answer(&load(s, Some("pc")), &request).unwrap())
}

#[test]
fn the_moved_world_is_shown_and_answered_with_its_state_never_refused() {
    let s = launch();
    // a new base: shown, its class undecided with the reason
    let new = show(&s, "new", false).unwrap();
    match &new.class {
        Classed::Open(why) => {
            assert!(why.problem.contains("Wombgift Chalice"), "{}", why.problem)
        }
        other => panic!("a base the table lacks is classed {other:?}"),
    }
    assert_eq!(new.lines.len(), 1);
    // asked by class, it is undecided with that reason, the rest answered
    let rings = asked(&s, json!({ "query": { "text": "class:rings" } }));
    assert_eq!(rings["total"]["matched"], 2, "{}", rings["total"]);
    assert_eq!(
        rings["total"]["undecided"]["count"], 1,
        "{}",
        rings["total"]
    );
    let reason = &rings["total"]["undecided_reasons"][0];
    assert_eq!(reason["example"]["id"], "new", "{reason}");
    // a mechanic's array: its line shown, the flag unknown, the part unread
    let mech = show(&s, "mech", false).unwrap();
    let line = &mech.lines[0].line;
    assert_eq!(line.source, "wombgift");
    assert_eq!(line.text, "Grants 3 Echoes of the Brood");
    assert_eq!(line.flags_unknown, ["echoing"]);
    assert_eq!(mech.item.unread.len(), 1, "{:?}", mech.item.unread);
    // a site search naming a stat the table lacks: answered, every item
    // in scope, the clause not run and the answer not the site's
    let untranslated = asked(
        &s,
        json!({ "query": { "trade": { "query": { "stats": [{ "type": "and", "filters": [
            { "id": "explicit.stat_9999999999" } ] }] } } } }),
    );
    assert_eq!(untranslated["total"]["matched"], 3);
    let trade = &untranslated["query"]["trade"];
    assert_eq!(
        trade["fidelity"],
        serde_json::to_value(Fidelity::Partial).unwrap()
    );
    assert_eq!(trade["remainder"], json!(["stats.0.0"]), "{trade}");
}
