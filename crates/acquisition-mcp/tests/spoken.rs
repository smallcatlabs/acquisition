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

use crate::harness;

use std::path::Path;

use acquisition_store::{Endpoint, Index, Store, account_path};
use harness::{Mcp, scratch};
use serde_json::{Value, json};

const USER: &str = "Alice#1234";

/// One account with a public tab of one item in pc's Standard and a tab
/// of one item in poe2's, so a search naming no realm is refused.
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

/// Every string of `doc` that speaks the CLI unspelled, with its path.
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
    ] {
        let result = mcp.result(tool, args.clone());
        assert_eq!(result["isError"], json!(refused), "{tool} {args}: {result}");
        let text = result["content"][0]["text"].as_str().unwrap_or_default();
        let doc: Value = serde_json::from_str(text).unwrap();
        // the text and the structured content are one document
        assert_eq!(doc, result["structuredContent"], "{tool} {args}");
        // the sweep reads something: every one but the account's (call
        // 46, which names `account` alone) speaks the CLI
        seen += usize::from(text.contains("— here, "));
        sweep(&doc, &format!("{tool} {args}"), &mut faults);
    }
    assert_eq!(
        seen, 6,
        "a shape no longer speaks the CLI: the sweep reads less"
    );
    assert!(faults.is_empty(), "{}", faults.join("\n"));
}
