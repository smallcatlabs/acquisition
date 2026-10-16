//! The oracle of missed matches (C113, K6; step 6 of `search/TRADE-PLAN.md`):
//! every search the site answered with fetched items, translated, and the
//! items it returned entered through the store's ingest and asked the
//! translated text. The captures hold what the site returned and never what
//! it excluded, so this sees missed matches alone; excess matches are step
//! 4's guard and fixtures.
//!
//! An item entered is the item as the site sent it
//! (`search/pseudo-stats/data/bodies.json`, `tools/trade-captures.py`): an
//! item rebuilt from the extract's selection of it lost what the search
//! reads — the bestiary text that makes a beast a beast, the item level —
//! and 3,438 of the items missed where 497 do as sent (measured at the
//! step's opening, 2026-10-16).
//!
//! Every item that the text does not match is a disagreement, and an item
//! the text leaves undecided is one too, of its own kind, never a match.
//! Each is a row of `trade-oracle.toml`, naming the top-level clauses that
//! refuse it — each clause's term asked alone, so a row says which clause
//! and never guesses — and the finding that answers it: a fact of the
//! site's captures, the owner's ruling, or a question for the sitting
//! (step 7). The rows are the expected set exactly: a new disagreement and
//! a vanished one each fail. A row of a fact of the site lies on a clause
//! the report does not call exact, its catalogue row named, so that no
//! clause is read as exact against what the captures show.

mod common;

use std::collections::{BTreeMap, BTreeSet};

use acquisition_search::{Request, Translation, answer, decode_json, translate};
use common::*;
use serde::Deserialize;
use serde_json::{Value, json};

const CAPTURES: &str = include_str!("../../../search/pseudo-stats/data/captures.json");
const BODIES: &str = include_str!("../../../search/pseudo-stats/data/bodies.json");
const CENSUS: &str = include_str!("../../../search/trade-query/data/fetch-census.json");
const EXPECTED: &str = include_str!("trade-oracle.toml");

#[derive(Deserialize)]
struct Expected {
    finding: Vec<Finding>,
    row: Vec<Row>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Finding {
    key: String,
    /// `site`, `ruling` or `sitting`.
    home: String,
    cites: String,
    /// For a fact of the site: the catalogue row its clause names.
    #[serde(default)]
    row: Option<String>,
    /// For a question of the sitting: what it is to settle.
    #[serde(default)]
    asks: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Row {
    search: String,
    items: Vec<usize>,
    outcome: String,
    clauses: Vec<String>,
    finding: String,
}

/// One search: its name, its query as the site took it, its items as sent.
struct Search {
    name: String,
    query: Value,
    bodies: Vec<Value>,
}

fn searches() -> Vec<Search> {
    let captures: Value = serde_json::from_str(CAPTURES).unwrap();
    let bodies: Value = serde_json::from_str(BODIES).unwrap();
    let census: Value = serde_json::from_str(CENSUS).unwrap();
    let of = |group: &str, name: &str| -> Vec<Value> {
        bodies[group][name].as_array().cloned().unwrap_or_default()
    };
    let mut out = Vec::new();
    for (name, capture) in captures["searches"].as_object().unwrap() {
        let bodies = of("searches", name);
        assert_eq!(
            bodies.len(),
            capture["fetched"].as_array().unwrap().len(),
            "{name}: the bodies and the extract hold one fetch"
        );
        out.push(Search {
            name: name.clone(),
            query: capture["returned_query"].clone(),
            bodies,
        });
    }
    for (name, q) in census["queries"].as_object().unwrap() {
        let bodies = of("census", name);
        assert_eq!(
            bodies.len(),
            q["fetched"].as_array().map_or(0, Vec::len),
            "{name}: the bodies and the census hold one fetch"
        );
        out.push(Search {
            name: name.clone(),
            query: q["request"].clone(),
            bodies,
        });
    }
    out
}

/// The indices of the items a text matches, over a corpus of one search's
/// items, each entered under its index.
fn matching(corpus: &acquisition_search::Corpus, text: &str) -> BTreeSet<usize> {
    let request: Request = serde_json::from_value(
        json!({ "query": { "text": text }, "view": { "rows": { "limit": 1000 } } }),
    )
    .unwrap();
    let answer = as_json(&answer(corpus, &request).unwrap_or_else(|e| panic!("{text}: {e}")));
    answer["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["id"].as_str().unwrap().parse().unwrap())
        .collect()
}

/// A disagreement: the search, the item, `missed` or `undecided`, and the
/// top-level clauses that refuse it alone.
type Disagreement = (String, usize, String, Vec<String>);

fn disagreements(search: &Search, t: &Translation) -> Vec<Disagreement> {
    let mut s = store();
    list_tabs(&mut s, "pc", "Standard", json!([tab("t", "Fetched")]), 10);
    let bodies = search
        .bodies
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let mut body = b.clone();
            body["id"] = json!(i.to_string());
            body["x"] = json!(i);
            body["y"] = json!(0);
            body
        })
        .collect();
    fetch_tab(&mut s, "pc", "Standard", "t", "Fetched", bodies, 20);
    let corpus = load(&s, Some("pc"));
    let all: BTreeSet<usize> = (0..search.bodies.len()).collect();
    let matched = matching(&corpus, &t.text);
    if matched == all {
        return Vec::new();
    }
    let undecided = if t.text.is_empty() {
        BTreeSet::new()
    } else {
        matching(&corpus, &format!("undecided({})", t.text))
    };
    // each top-level clause's term alone: what refuses an item
    let top: Vec<(String, BTreeSet<usize>)> = t
        .clauses
        .iter()
        .filter_map(|c| {
            let path = c.path.as_ref()?;
            let term = c.term.as_ref()?;
            (!path.contains('.')).then(|| (path.clone(), matching(&corpus, term)))
        })
        .collect();
    all.difference(&matched)
        .map(|&i| {
            let outcome = if undecided.contains(&i) {
                "undecided"
            } else {
                "missed"
            };
            let clauses = top
                .iter()
                .filter(|(_, m)| !m.contains(&i))
                .map(|(p, _)| p.clone())
                .collect();
            (search.name.clone(), i, outcome.to_string(), clauses)
        })
        .collect()
}

/// Step 6's acceptance: over the 451 captures with ten fetched items and
/// the 78 with one to nine, and the owner's seven searches that fetched
/// any, every one whose translation has no remainder runs, and every item
/// the site returned that the text does not match is a row of the expected
/// set, which is exactly what is found. The 335 captures with no item, and
/// q2b, which found none, are excluded by name; so are the 18 whose
/// translation has a remainder — 13 weighted groups of mixed weights,
/// outside the brief, and the 5 crucible-node searches.
#[test]
fn c113_every_item_the_site_returned_is_matched_or_a_named_row() {
    let expected: Expected = toml::from_str(EXPECTED).unwrap();
    let findings: BTreeMap<&str, &Finding> = expected
        .finding
        .iter()
        .map(|f| (f.key.as_str(), f))
        .collect();
    assert_eq!(findings.len(), expected.finding.len(), "a finding twice");
    for f in &expected.finding {
        assert!(
            ["site", "ruling", "sitting"].contains(&f.home.as_str()),
            "{}: home {}",
            f.key,
            f.home
        );
        assert!(!f.cites.is_empty(), "{}", f.key);
        assert_eq!(
            f.home == "site",
            f.row.is_some(),
            "{}: a fact names its row",
            f.key
        );
        assert_eq!(
            f.home == "sitting",
            f.asks.is_some(),
            "{}: a question asks",
            f.key
        );
    }
    let mut want: BTreeMap<(String, usize), (String, Vec<String>, &str)> = BTreeMap::new();
    for r in &expected.row {
        assert!(
            findings.contains_key(r.finding.as_str()),
            "{}: {}",
            r.search,
            r.finding
        );
        for &i in &r.items {
            let before = want.insert(
                (r.search.clone(), i),
                (r.outcome.clone(), r.clauses.clone(), r.finding.as_str()),
            );
            assert!(before.is_none(), "{} item {i} in two rows", r.search);
        }
    }
    let used: BTreeSet<&str> = expected.row.iter().map(|r| r.finding.as_str()).collect();
    assert_eq!(used.len(), findings.len(), "a finding no row cites");

    let (mut none, mut partial, mut ran, mut items, mut matched) =
        (Vec::new(), Vec::new(), 0, 0, 0);
    let mut found: BTreeMap<(String, usize), (String, Vec<String>)> = BTreeMap::new();
    let mut translations: BTreeMap<String, Translation> = BTreeMap::new();
    for search in searches() {
        if search.bodies.is_empty() {
            none.push(search.name);
            continue;
        }
        let t = translate(&decode_json(&search.query).unwrap()).unwrap();
        if !t.remainder.is_empty() {
            partial.push(search.name);
            continue;
        }
        ran += 1;
        items += search.bodies.len();
        let missed = disagreements(&search, &t);
        matched += search.bodies.len() - missed.len();
        for (name, i, outcome, clauses) in missed {
            found.insert((name, i), (outcome, clauses));
        }
        translations.insert(search.name, t);
    }
    assert_eq!((none.len(), ran, items, matched), (336, 518, 4733, 4236));
    assert!(none.contains(&"q2b".to_string()));
    partial.sort();
    assert_eq!(
        partial,
        [
            "d01", "d02", "f011", "f012", "f017", "f018", "f019", "h004", "n01", "n02", "n03",
            "n04", "n05", "n17", "n20", "n21", "n22", "n23"
        ]
    );

    let got: BTreeMap<(String, usize), (String, Vec<String>)> = want
        .iter()
        .map(|(k, (o, c, _))| (k.clone(), (o.clone(), c.clone())))
        .collect();
    let new: Vec<_> = found
        .iter()
        .filter(|(k, v)| got.get(*k) != Some(*v))
        .collect();
    let gone: Vec<_> = got
        .iter()
        .filter(|(k, v)| found.get(*k) != Some(*v))
        .collect();
    assert!(
        new.is_empty() && gone.is_empty(),
        "disagreements no row names: {new:#?}\nrows no longer found: {gone:#?}"
    );

    // a fact of the site lies on a clause the report does not call exact
    for ((search, i), (_, clauses, finding)) in &want {
        let Some(row) = &findings[finding].row else {
            continue;
        };
        let t = &translations[search];
        for clause in clauses {
            let under = |c: &&acquisition_search::trade::translate::Clause| {
                c.path
                    .as_deref()
                    .is_some_and(|p| p == clause || p.starts_with(&format!("{clause}.")))
            };
            assert!(
                t.clauses.iter().filter(under).any(|c| c.rows.contains(row)
                    && c.status == acquisition_search::trade::translate::Status::Ambiguous),
                "{search} item {i}: no clause under {clause} is ambiguous by {row}"
            );
        }
    }
}
