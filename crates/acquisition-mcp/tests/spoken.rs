//! The MCP speaks its own words (C143: a shared text speaks the
//! request's own words, never a frontend's flag): the readiness seat
//! (`audits/readiness-2026-10/seat/`) met the CLI's commands and flags in
//! a refusal's readings (call 6, `--realm pc`), a refusal's next step
//! (call 47, `acq search`, `acq tabs`), the shop's rule text (call 35,
//! `acq price show`) and `describe`'s entries (calls 5, 12, 13:
//! `acq store refused`, `--sort`, `--count`), and translated each by
//! hand. They were spelled after the shared words first; since #97 the
//! shared words are the request's. The sweep reads the seat's call
//! shapes and two more refusals over a store of two realms, and holds
//! every string the server sends that is not payload free of a CLI
//! command (`acq …`) and flag (`--…`), a person's act apart, with no
//! spelling after it.
//!
//! Payload is never spelled (the fix round's review, P2): an offer's
//! request, a query, a continuation, an example, a refusal's readings —
//! what an agent sends back as printed. The sweep reads
//! prose alone, and the round trip sends a refusal's offer back with a
//! query literal that speaks a CLI command and holds it accepted.

use crate::harness;

use std::path::Path;

use acquisition_store::{Endpoint, Index, Store, account_path};
use harness::{Mcp, scratch};
use serde_json::{Value, json};

const USER: &str = "Alice#1234";

/// One account with a public tab of one item in pc's Standard and a tab
/// of one item in poe2's, so a search naming no realm is refused; and a
/// tab, `t-gone`, fetched and then left out of pc's listing, so it is no
/// longer in the account and `show` of its id is refused as no item.
fn seed(base: &Path) {
    let dir = base.join("store").join("mock");
    std::fs::create_dir_all(&dir).unwrap();
    Index::load(&dir)
        .unwrap()
        .record_login(USER, "u-spoken", false, 1, &[])
        .unwrap();
    let mut store = Store::open(&account_path(&dir, USER)).unwrap();
    store
        .record(
            &Endpoint::Profile,
            &json!({}),
            200,
            &json!({ "uuid": "u-spoken", "name": USER }),
            1,
        )
        .unwrap();
    let gone = json!({ "id": "i-gone", "name": "", "typeLine": "Chaos Orb",
                       "baseType": "Chaos Orb", "x": 0, "y": 0, "stackSize": 1 });
    store
        .record(
            &Endpoint::Stash {
                realm: "pc".into(),
                league: "Standard".into(),
                id: "t-gone".into(),
                sub: None,
            },
            &json!({ "realm": "pc", "league": "Standard" }),
            200,
            &json!({ "stash": { "id": "t-gone", "name": "Old", "type": "PremiumStash",
                                 "items": [gone] } }),
            5,
        )
        .unwrap();
    store
        .record(
            &Endpoint::Stashes {
                realm: "pc".into(),
                league: "Standard".into(),
            },
            &json!({ "realm": "pc", "league": "Standard" }),
            200,
            &json!({ "stashes": [ { "id": "t-pc", "name": "Sale", "type": "PremiumStash" } ] }),
            8,
        )
        .unwrap();
    for realm in ["pc", "poe2"] {
        let item = json!({ "id": format!("i-{realm}"), "name": "", "typeLine": "Chaos Orb",
                           "baseType": "Chaos Orb", "x": 0, "y": 0, "stackSize": 5 });
        store
            .record(
                &Endpoint::Stash {
                    realm: realm.into(),
                    league: "Standard".into(),
                    id: format!("t-{realm}"),
                    sub: None,
                },
                &json!({ "realm": realm, "league": "Standard" }),
                200,
                &json!({ "stash": { "id": format!("t-{realm}"), "name": "Sale",
                                     "type": "PremiumStash", "items": [item] } }),
                10,
            )
            .unwrap();
    }
}

/// Every shape below is read whole, every string not payload, and none
/// speaks the CLI (C143, as the harness's rule reads an answer's words;
/// #97): the spelling `— here, …` that this file once swept for has
/// nothing left to spell in them.
#[test]
fn every_text_the_server_sends_speaks_its_own_words() {
    let dir = scratch("mcp-spoken");
    seed(&dir.0);
    let mut mcp = Mcp::start(&dir.0, &[]);
    let mut faults = Vec::new();
    for (tool, args, refused) in [
        // the seat's shapes: describe, the index and whole (calls 5, 12,
        // 13; the index since C53's amendment), a search
        // naming no realm over two (call 6), the shop (call 35), an
        // account that is none (call 46), a target the facts lack (47)
        ("describe", json!({}), false),
        ("describe", json!({ "names": ["all"] }), false),
        ("search", json!({}), true),
        ("shop_render", json!({ "realm": "pc" }), false),
        ("tabs", json!({ "account": "Alicia#1234" }), true),
        ("price_show", json!({ "target": "tab/pc/ffffffffff" }), true),
        // and two more: a name describe does not know lists every name,
        // the flags among them; a clear of a row there is not
        ("describe", json!({ "names": ["nonsense"] }), true),
        (
            "price_clear",
            json!({ "target": "tab/pc/t-pc", "if_revision": 1 }),
            true,
        ),
        // the review's: a tab no longer in the account, shown, is no
        // item, and its reading is a search under `--membership all`
        ("show", json!({ "id": "t-gone" }), true),
    ] {
        let result = mcp.result(tool, args.clone());
        assert_eq!(result["isError"], json!(refused), "{tool} {args}: {result}");
        let text = result["content"][0]["text"].as_str().unwrap_or_default();
        let doc: Value = serde_json::from_str(text).unwrap();
        // the text is the one document: no structured content is sent, an
        // answer's or a refusal's, so no host hands the model a copy with
        // its keys sorted in the text's place (2g round 7, measured)
        assert_eq!(result.get("structuredContent"), None, "{tool} {args}");
        // nothing is spelled after the shared words, and no string not
        // payload speaks the CLI
        assert!(!text.contains("— here, "), "{tool} {args}: {text}");
        if let Some(said) = harness::frontend_speech(&doc) {
            faults.push(format!("{tool} {args}: {said}"));
        }
    }
    assert!(faults.is_empty(), "{}", faults.join("\n"));
}

/// The fix round's review, P2 (shape 6, a printed request that does not
/// run as printed): the spelling reached into a refusal's offers, so a
/// query literal speaking a CLI command gained a suffix and the offer no
/// longer ran. Over two realms a search naming none is refused with an
/// offer per realm; the offer's request holds the query exactly as sent,
/// and sent back as printed it is answered.
#[test]
fn a_refusals_offer_sent_back_as_printed_is_answered() {
    let dir = scratch("mcp-spoken-offer");
    seed(&dir.0);
    let mut mcp = Mcp::start(&dir.0, &[]);
    let text = "name:\"acq search --count\"";
    let result = mcp.result("search", json!({ "query": { "text": text } }));
    assert_eq!(result["isError"], json!(true), "{result}");
    let refusal = &harness::document(&result);
    assert_eq!(refusal["kind"], "realm_needed", "{refusal}");
    let offer = refusal["offers"][0]["request"].clone();
    assert_eq!(offer["query"]["text"], json!(text), "{refusal}");
    let answered = mcp.result("search", offer.clone());
    assert_ne!(answered["isError"], json!(true), "{offer}: {answered}");
    assert_eq!(
        harness::document(&answered)["query"]["text"],
        json!(text),
        "{answered}"
    );
    // the words name `all` beside the realms, and an offer sends it (the
    // league seat, mcp call 25)
    let all = refusal["offers"]
        .as_array()
        .and_then(|offers| offers.last())
        .map(|offer| offer["request"].clone())
        .unwrap_or_default();
    assert_eq!(all["scope"]["realm"], json!("all"), "{refusal}");
    let answered = mcp.result("search", all.clone());
    assert_ne!(answered["isError"], json!(true), "{all}: {answered}");
}
