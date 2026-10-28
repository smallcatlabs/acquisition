//! The MCP speaks its own words (C143, the shared words first and an
//! adapter's spelling after them): the readiness seat
//! (`audits/readiness-2026-10/seat/`) met the CLI's commands and flags in
//! a refusal's readings (call 6, `--realm pc`), a refusal's next step
//! (call 47, `acq search`, `acq tabs`), the shop's rule text (call 35,
//! `acq price show`) and `describe`'s entries (calls 5, 12, 13:
//! `acq store refused`, `--sort`, `--count`), and translated each by
//! hand. The sweep reads the seat's call shapes and two more refusals
//! over a store of two realms, and holds one fact of every string the
//! server sends: a CLI command (`acq …`) or flag (`--…`) it speaks has
//! this server's spelling beside it in the same string — "`--realm` is
//! …", "`acq store refused` has …" — or the string gives the command to
//! a person at a terminal. An entry's name (`name`, `construct`), which
//! is how the entry is asked for, stays as the CLI spells it and its
//! spelling stands in the entry's `what`.
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

/// Where a string speaks a CLI command or flag with no spelling beside
/// it: each word the string speaks, in the string's own words.
fn unspelled(said: &str) -> Vec<String> {
    let person = ["a person", "a human", "terminal", "the CLI"]
        .iter()
        .any(|p| said.contains(p));
    let spelled = |word: &str| {
        said.contains(&format!("`{word}` is ")) || said.contains(&format!("`{word}` has "))
    };
    let mut found = Vec::new();
    let bytes = said.as_bytes();
    let word_char = |b: u8| b.is_ascii_alphanumeric() || b == b'-' || b == b'_';
    for (at, _) in said.match_indices("acq ") {
        if at > 0 && word_char(bytes[at - 1]) {
            continue;
        }
        // the command and each shorter reading of it: `acq price show x`
        // is `acq price show`, `acq price`, …
        let words: Vec<&str> = said[at + 4..]
            .split(|c: char| !(c.is_ascii_lowercase() || c == '-'))
            .take_while(|w| !w.is_empty())
            .take(3)
            .collect();
        if words.is_empty() {
            continue;
        }
        let ok = (1..=words.len())
            .rev()
            .any(|n| spelled(&format!("acq {}", words[..n].join(" "))));
        if !ok && !person {
            found.push(format!("acq {}", words.join(" ")));
        }
    }
    for (at, _) in said.match_indices("--") {
        if at > 0 && word_char(bytes[at - 1]) {
            continue;
        }
        let flag: String = said[at + 2..]
            .chars()
            .take_while(|&c| c.is_ascii_lowercase() || c == '-')
            .collect();
        if flag.is_empty() {
            continue;
        }
        let flag = format!("--{flag}");
        if !spelled(&flag) && !person {
            found.push(flag);
        }
    }
    found
}

/// The keys whose value is payload, never spelled and never swept.
const PAYLOAD: [&str; 11] = [
    "offers", "request", "query", "params", "next", "route", "examples", "readings", "current",
    "accounts", "undo",
];

/// Every string of `doc`'s prose that speaks the CLI unspelled, with its
/// path.
fn sweep(doc: &Value, path: &str, out: &mut Vec<String>) {
    match doc {
        Value::String(s) => {
            for word in unspelled(s) {
                out.push(format!("{path}: {word} in {s:?}"));
            }
        }
        Value::Array(items) => {
            for (i, v) in items.iter().enumerate() {
                sweep(v, &format!("{path}[{i}]"), out);
            }
        }
        Value::Object(fields) => {
            for (k, v) in fields {
                if PAYLOAD.contains(&k.as_str()) {
                    continue;
                }
                match (k.as_str(), v) {
                    // a name is asked by: its spelling stands in `what`
                    ("name" | "construct", Value::String(name)) => {
                        let what = fields.get("what").and_then(Value::as_str).unwrap_or("");
                        for word in unspelled(&format!("{name} {what}")) {
                            out.push(format!("{path}.{k}: {word} in {name:?} / {what:?}"));
                        }
                    }
                    _ => sweep(v, &format!("{path}.{k}"), out),
                }
            }
        }
        _ => {}
    }
}

#[test]
fn every_text_the_server_sends_spells_the_cli_it_speaks() {
    let dir = scratch("mcp-spoken");
    seed(&dir.0);
    let mut mcp = Mcp::start(&dir.0, &[]);
    let mut faults = Vec::new();
    let mut seen = 0;
    for (tool, args, refused) in [
        // the seat's shapes: describe whole (calls 5, 12, 13), a search
        // naming no realm over two (call 6), the shop (call 35), an
        // account that is none (call 46), a target the facts lack (47)
        ("describe", json!({}), false),
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
        // the sweep reads something: every one speaks the CLI but the
        // account's (call 46, which names `account` alone), the realms'
        // (their readings the request's words, C143) and the gone tab's
        // (its reading a command, payload beside its offer)
        seen += usize::from(text.contains("— here, "));
        sweep(&doc, &format!("{tool} {args}"), &mut faults);
    }
    assert_eq!(
        seen, 5,
        "a shape no longer speaks the CLI: the sweep reads less"
    );
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
}
