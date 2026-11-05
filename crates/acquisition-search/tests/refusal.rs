//! The refusal walk (the crate's `lib.rs`, "The plan's rules",
//! rule 1) and invariant 3 of the surface: every construct of the
//! language either evaluates or is refused by its name on the one list
//! `--describe` prints — never the unknown-name error, never undecided,
//! never an empty answer — and a query's validity never depends on the
//! corpus, so the whole language corpus is asked of a store holding
//! nothing.

use crate::common;

use acquisition_search::{ErrorKind, NOT_BUILT, SearchError, answer, describe};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Language {
    case: Vec<Case>,
    value: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
    construct: String,
    text: String,
    canonical: Option<String>,
}

fn language() -> Language {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/language.toml");
    toml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

/// The construct a refusal names, which must be one of the list's.
fn refused(e: &SearchError) -> Option<&'static str> {
    let SearchError::Language(e) = e else {
        return None;
    };
    if e.kind != ErrorKind::NotBuilt {
        return None;
    }
    NOT_BUILT
        .iter()
        .map(|n| n.construct)
        .find(|c| e.message.starts_with(&format!("not built: {c} (")))
}

#[test]
fn every_construct_of_the_reference_evaluates_or_is_refused_by_its_name_on_the_list() {
    let store = common::store();
    let corpus = common::load(&store, None);
    let language = language();
    let (mut evaluated, mut refusals) = (0, std::collections::BTreeSet::new());
    let mut stray: Vec<String> = Vec::new();
    let queries = language
        .case
        .iter()
        .map(|c| (c, json!({ "query": { "text": c.text } })));
    let sorts = language
        .value
        .iter()
        .map(|c| (c, json!({ "view": { "rows": { "sort": c.text } } })));
    for (case, request) in queries.chain(sorts) {
        if case.canonical.is_none() {
            continue; // an authoring error of the grammar: step 1's
        }
        match answer(&corpus, &serde_json::from_value(request).unwrap()) {
            // invariant 3: valid over a store holding nothing, and the
            // nothing is said — a total, never an error
            Ok(answer) => {
                assert_eq!(
                    answer.total.matched, 0,
                    "{}: `{}`",
                    case.construct, case.text
                );
                evaluated += 1;
            }
            Err(e) => match refused(&e) {
                Some(construct) => {
                    refusals.insert(construct);
                }
                None => stray.push(format!("{}: `{}`: {e}", case.construct, case.text)),
            },
        }
    }
    assert!(
        stray.is_empty(),
        "neither evaluated nor refused by a listed name:\n{}",
        stray.join("\n")
    );
    assert!(evaluated > 0);
    // nothing the corpus can reach is on the list since step 9d-A built
    // `pseudo.defence_pct`, the last construct of the query it held: what
    // is left there is a request's words, a frontend's to refuse by the
    // same function (the list whole: explain.rs)
    assert!(refusals.is_empty(), "{refusals:?}");
}

#[test]
fn the_list_the_walk_checks_is_the_list_describe_prints() {
    let printed = serde_json::to_value(describe(&["all".to_string()]).unwrap()).unwrap();
    let printed: Vec<&str> = printed["not_built"]
        .as_array()
        .unwrap()
        .iter()
        .map(|n| n["construct"].as_str().unwrap())
        .collect();
    let list: Vec<&str> = NOT_BUILT.iter().map(|n| n.construct).collect();
    assert_eq!(printed, list);
}

/// S53 (C102): an item's class is not a field of the body — it is a
/// derivation the search owns, from the class table (C106) — and the
/// limit stays stated in the register's words; `class:ring` is a valid
/// selector over a store holding nothing (invariant 3), and the table's
/// definition is what `--describe class` prints.
#[test]
fn c102_s53_a_class_is_a_derivation_the_search_owns_and_says_so() {
    let store = common::store();
    let corpus = common::load(&store, None);
    let a = common::ask(&corpus, "class:ring").unwrap();
    assert_eq!(a.total.matched, 0);
    let printed = serde_json::to_value(describe(&["all".to_string()]).unwrap()).unwrap();
    let limits: Vec<&str> = printed["limits"]
        .as_array()
        .unwrap()
        .iter()
        .map(|l| l["said"].as_str().unwrap())
        .collect();
    // the wording since the first seat (F1): what step 6 made true
    assert!(limits.contains(
        &"an item's class is a derivation the search owns — read from its base by the class table, never a fact GGG gives — and an item the table cannot class is undecided with its reason"
    ));
    let class = serde_json::to_value(describe(&["class".to_string()]).unwrap()).unwrap();
    let what = class["fields"][0]["what"].as_str().unwrap();
    assert!(
        what.contains("classes v1") && what.contains("RePoE"),
        "{what}"
    );
    assert!(
        class["fields"][0]["values"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v == "Rings")
    );
}

#[test]
fn a_name_the_language_does_not_know_is_an_authoring_error_with_near_names() {
    let store = common::store();
    let corpus = common::load(&store, None);
    // `readings` is the head of what is offered, in order: a bare word is
    // a closed value's first where it is one, then a name (a Sonnet run of
    // the re-sit took a phrase reading and searched line text for a unique,
    // mcp-sonnet calls 18 to 21), and every bare word's readings run.
    for (text, kind, readings) in [
        ("rarty=rare", "unknown_name", &["rarity=rare"][..]),
        ("rarity=rar", "unknown_value", &["rarity=rare"]),
        ("is:corupted", "unknown_name", &["is:corrupted"]),
        ("ilvl:84", "operator_mismatch", &["ilvl=84"]),
        ("name~\"(\"", "bad_pattern", &[]),
        ("line(is:corrupted)", "unknown_name", &[]),
        ("is:crafted", "unknown_name", &["line(is:crafted)"]),
        ("rare", "bare_word", &["rarity=rare", "name:rare"]),
        ("Loreweave", "bare_word", &["name:Loreweave"]),
        ("corrupted", "bare_word", &["is:corrupted"]),
        ("line(fractured)", "bare_word", &["is:fractured"]),
        // a field `has:` takes is offered with it first (#174; the
        // answer-facts seat's CLI run 2, calls 66, 67); under a minus the
        // reader typed every offer carries it, its span taking it in
        ("priced", "bare_word", &["has:priced", "name:priced"]),
        (
            "links=6 league=Standard -priced",
            "bare_word",
            &["-has:priced", "-name:priced"],
        ),
        ("not Priced", "bare_word", &["-has:priced"]),
        (
            "links=6 league=Standard -is:priced",
            "unknown_name",
            &["has:priced"],
        ),
    ] {
        let e = common::ask(&corpus, text).unwrap_err();
        let json = e.to_json();
        assert_eq!(json["kind"], kind, "`{text}`: {e}");
        for (at, reading) in readings.iter().enumerate() {
            assert_eq!(json["readings"][at], *reading, "`{text}`: {e}");
        }
        if kind == "bare_word" {
            let first = json["readings"][0].as_str().unwrap();
            common::ask(&corpus, first).unwrap_or_else(|e| panic!("`{first}`: {e}"));
            // every offer replaces exactly its span: pasted there, it
            // parses (the review of round A, finding 1)
            for reading in json["readings"].as_array().unwrap() {
                let pasted = pasted(text, &json["span"], reading.as_str().unwrap());
                acquisition_search::parse_query(&pasted)
                    .unwrap_or_else(|e| panic!("`{text}` as `{pasted}`: {e}"));
            }
        }
    }
    // and means what it offers: a minus the reader typed is in the span,
    // never twice in the query (`--has:priced`, the opposite)
    for (text, meant) in [
        (
            "links=6 league=Standard -priced",
            "links=6 league=Standard -has:priced",
        ),
        ("not Priced", "-has:priced"),
        ("links=6 -  priced", "links=6 -has:priced"),
        ("-corrupted", "-is:corrupted"),
        ("-(priced)", "-(has:priced)"),
        ("line(-fractured)", "line(-is:fractured)"),
    ] {
        let json = common::ask(&corpus, text).unwrap_err().to_json();
        let first = json["readings"][0].as_str().unwrap();
        assert_eq!(
            pasted(text, &json["span"], first),
            meant,
            "`{text}`: {json}"
        );
    }
}

/// A refusal's first offer pasted over its span in the text it refused.
fn pasted(text: &str, span: &serde_json::Value, reading: &str) -> String {
    let at = |i: usize| span[i].as_u64().unwrap() as usize;
    format!("{}{reading}{}", &text[..at(0)], &text[at(1)..])
}
