//! The oracle of missed matches (C113, K6): every search the site answered
//! with fetched items, translated, and the items it returned entered
//! through the store's ingest and asked the translated text. The captures
//! hold what the site returned and never what it excluded, so this sees
//! missed matches alone; excess matches are the translation's guard and
//! fixtures.
//!
//! An item entered is the item as the site sent it
//! (`search/pseudo-stats/data/bodies.json`, `tools/trade-captures.py`): an
//! item rebuilt from the extract's selection of it would lose what the
//! search reads — the bestiary text that makes a beast a beast, the item
//! level.
//!
//! Every item that the text does not match is a disagreement, and an item
//! the text leaves undecided is one too, of its own kind, never a match.
//! Each is a row of `trade-oracle.toml`, naming the top-level clauses that
//! refuse it — each clause's term asked alone, so a row says which clause
//! and never guesses — and the finding that answers it: a fact of the
//! site's captures, the owner's ruling, a question for a sitting, a
//! patch's rewording, or an item whose body lacks what the site matched
//! (an unidentified unique's name, its clause the name's). The
//! rows are the expected set exactly: a new disagreement and a vanished one
//! each fail. A fact of the site lies where the item meets it — the clauses
//! under the refusing one whose term alone the item meets, the eldritch
//! line a `not` excludes — and the report calls each of them ambiguous by
//! the fact's catalogue row, so that no clause is read as exact against
//! what the captures show, and no labelled sibling stands in for an
//! unlabelled one.

use crate::common;

use std::collections::{BTreeMap, BTreeSet};

use acquisition_search::trade::translate::Status;
use acquisition_search::{Request, Translation, answer, decode_json, translate};
use common::*;
use serde::Deserialize;
use serde_json::{Value, json};

const CAPTURES: &str = include_str!("../../../search/pseudo-stats/data/captures.json");
const BODIES: &str = include_str!("../../../search/pseudo-stats/data/bodies.json");
const CENSUS: &str = include_str!("../../../search/trade-query/data/fetch-census.json");
const EXPECTED: &str = include_str!("trade-oracle.toml");
/// The owner's searches were made in Standard
/// (`search/trade-query/MANIFEST.md`); their census names no league.
const CENSUS_LEAGUE: &str = "Standard";

#[derive(Deserialize)]
struct Expected {
    finding: Vec<Finding>,
    row: Vec<Row>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Finding {
    key: String,
    /// `site`, `ruling`, `sitting`, `patch` or `body`.
    home: String,
    cites: String,
    /// The catalogue row that labels its clause ambiguous.
    #[serde(default)]
    row: Option<String>,
    /// A ruling by which the owner chose to differ from the site: no row.
    #[serde(default)]
    differs: bool,
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

/// One search: its name, its league and its query as the site took them,
/// its items as sent. A search is its league and its query: the league is
/// translated as the URL would name it, so a translation that writes
/// another misses every item.
struct Search {
    name: String,
    league: String,
    query: Value,
    bodies: Vec<Value>,
}

fn searches() -> Vec<Search> {
    let captures: Value = serde_json::from_str(CAPTURES).unwrap();
    let bodies: Value = serde_json::from_str(BODIES).unwrap();
    let census: Value = serde_json::from_str(CENSUS).unwrap();
    // One generation's two files: each names the raw it was made from, and
    // a pair whose counts agree may still be a new item beside an old body.
    assert!(captures["_raw"].is_string(), "the extract names no raw");
    assert_eq!(
        captures["_raw"], bodies["_raw"],
        "the extract and the bodies were made from other raw: written apart"
    );
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
            league: capture["league"]
                .as_str()
                .unwrap_or_else(|| panic!("{name}: the extract names no league"))
                .to_string(),
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
            league: CENSUS_LEAGUE.to_string(),
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

/// An item the text does not match.
struct Disagreement {
    search: String,
    item: usize,
    /// `missed` or `undecided`.
    outcome: String,
    /// The top-level clauses whose term alone refuses the item.
    refused_by: Vec<String>,
    /// The clauses under those whose term alone the item meets: where a
    /// fact of the site lies — the line a `not` excludes is the one the
    /// item carries, never a sibling it does not.
    met: Vec<String>,
}

fn disagreements(search: &Search, t: &Translation) -> Vec<Disagreement> {
    let mut s = store();
    // each item in the league it was listed in, as the site sent it
    let mut by_league: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for (i, b) in search.bodies.iter().enumerate() {
        let mut body = b.clone();
        body["id"] = json!(i.to_string());
        body["x"] = json!(i);
        body["y"] = json!(0);
        let league = b["league"]
            .as_str()
            .unwrap_or_else(|| panic!("{} item {i}: no league", search.name));
        by_league.entry(league.to_string()).or_default().push(body);
    }
    for (league, bodies) in by_league {
        list_tabs(&mut s, "pc", &league, json!([tab("t", "Fetched")]), 10);
        fetch_tab(&mut s, "pc", &league, "t", "Fetched", bodies, 20);
    }
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
    // every clause's term alone, by its path
    let alone: Vec<(String, BTreeSet<usize>)> = t
        .clauses
        .iter()
        .filter_map(|c| Some((c.path.clone()?, matching(&corpus, c.term.as_ref()?))))
        .collect();
    all.difference(&matched)
        .map(|&i| {
            let outcome = if undecided.contains(&i) {
                "undecided"
            } else {
                "missed"
            };
            let refused_by: Vec<String> = alone
                .iter()
                .filter(|(p, m)| !p.contains('.') && !m.contains(&i))
                .map(|(p, _)| p.clone())
                .collect();
            let met = alone
                .iter()
                .filter(|(p, m)| {
                    m.contains(&i) && refused_by.iter().any(|r| p.starts_with(&format!("{r}.")))
                })
                .map(|(p, _)| p.clone())
                .collect();
            Disagreement {
                search: search.name.clone(),
                item: i,
                outcome: outcome.to_string(),
                refused_by,
                met,
            }
        })
        .collect()
}

/// The acceptance: over the captures with fetched items — the 529 of the
/// pseudo track's sittings, the 260 of rounds t to w — and the owner's
/// seven searches that fetched any, every one whose translation has no
/// remainder runs, and every item the site returned that the text does not
/// match is a row of the expected set, which is exactly what is found. The
/// searches with no item (335, q2b, and 26 of rounds t to w) are excluded
/// by name; so are the 31 whose translation has a remainder — 13 weighted
/// groups of mixed weights, outside the brief, 15 crucible-node searches,
/// and the questions the build leaves unsupported: two ids of two texts and
/// the damage filter. Five ran partial (t37, t38, t55, t56, u059). Of the 31
/// y searches, 11 have no item, y10 is a sixteenth crucible-node search,
/// and 19 run. Of the 17 of rounds zb and zc, 4 have no item and 13 run,
/// and of round ze's 4, ze02 has none and 3 run (the test below).
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
            ["site", "ruling", "sitting", "patch", "body"].contains(&f.home.as_str()),
            "{}: home {}",
            f.key,
            f.home
        );
        assert!(!f.cites.is_empty(), "{}", f.key);
        // every disagreement under a label, but where the owner chose to
        // differ from the site
        assert_eq!(
            f.row.is_none(),
            f.differs || ["patch", "body"].contains(&f.home.as_str()),
            "{}: a finding names its row, or differs by a ruling, or is a patch's or a body's",
            f.key
        );
        assert!(
            f.home == "ruling" || !f.differs,
            "{}: only a ruling differs",
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
    let mut met: BTreeMap<(String, usize), Vec<String>> = BTreeMap::new();
    let mut translations: BTreeMap<String, Translation> = BTreeMap::new();
    for search in searches() {
        if search.bodies.is_empty() {
            none.push(search.name);
            continue;
        }
        let mut decoded = decode_json(&search.query).unwrap();
        decoded.league = Some(search.league.clone());
        let t = translate(&decoded).unwrap();
        if !t.remainder.is_empty() {
            partial.push(search.name);
            continue;
        }
        ran += 1;
        items += search.bodies.len();
        let missed = disagreements(&search, &t);
        matched += search.bodies.len() - missed.len();
        for d in missed {
            found.insert((d.search.clone(), d.item), (d.outcome, d.refused_by));
            met.insert((d.search, d.item), d.met);
        }
        translations.insert(search.name, t);
    }
    assert_eq!((none.len(), ran, items, matched), (385, 816, 7669, 7039));
    assert!(none.contains(&"q2b".to_string()));
    partial.sort();
    assert_eq!(
        partial,
        [
            "d01", "d02", "f011", "f012", "f017", "f018", "f019", "h004", "n01", "n02", "n03",
            "n04", "n05", "n17", "n20", "n21", "n22", "n23", "t35", "t36", "t43", "v001", "v006",
            "v013", "v023", "v029", "v036", "v040", "v041", "v049", "v050", "y10"
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

    // a patch's: every search a row of it names asks a stat 3.30 moved,
    // so the item's wording predates the text the table gives
    let moved_at_330 = stats_moved_at_330();
    for r in expected
        .row
        .iter()
        .filter(|r| findings[r.finding.as_str()].home == "patch")
    {
        let query = &searches()
            .into_iter()
            .find(|s| s.name == r.search)
            .unwrap()
            .query;
        let ids: Vec<&str> = query["stats"]
            .as_array()
            .into_iter()
            .flatten()
            .flat_map(|g| g["filters"].as_array().into_iter().flatten())
            .filter_map(|f| f["id"].as_str())
            .collect();
        assert!(
            ids.iter().any(|id| moved_at_330.contains(*id)),
            "{}: a patch's row asks no stat 3.30 moved: {ids:?}",
            r.search
        );
        assert!(
            captured_before_330(&r.search),
            "{}: a patch's row is a capture taken after 3.30",
            r.search
        );
    }

    // a body's: every item a row of it names is unidentified as GGG sent
    // it, and the one clause that refuses it is the name's, which the
    // site matched on what the body does not carry
    for r in expected
        .row
        .iter()
        .filter(|r| findings[r.finding.as_str()].home == "body")
    {
        let search = searches().into_iter().find(|s| s.name == r.search).unwrap();
        for &i in &r.items {
            assert_eq!(
                search.bodies[i]["identified"],
                json!(false),
                "{} item {i}: a body's row is an identified item",
                r.search
            );
        }
        let t = &translations[&r.search];
        for path in &r.clauses {
            let at = &t
                .clauses
                .iter()
                .find(|c| c.path.as_ref() == Some(path))
                .unwrap()
                .at;
            assert_eq!(at, "name", "{}: a body's row is refused by {at}", r.search);
        }
    }

    // a ruling or a question: the clause that refuses the item, or a filter
    // under it, is ambiguous by the finding's row — no row of the set is a
    // clause reported exact
    for ((search, i), (_, refused_by, finding)) in &want {
        let f = findings[finding];
        let Some(row) = f.row.as_ref().filter(|_| f.home != "site") else {
            continue;
        };
        let t = &translations[search];
        for path in refused_by {
            let at = &t
                .clauses
                .iter()
                .find(|c| c.path.as_ref() == Some(path))
                .unwrap()
                .at;
            let labelled = t.clauses.iter().any(|c| {
                (c.at == *at || c.at.starts_with(&format!("{at}.")))
                    && c.status == Status::Ambiguous
                    && c.rows.contains(row)
            });
            assert!(
                labelled,
                "{search} item {i}: {path} is refused and nothing under it is ambiguous by {row}"
            );
        }
    }

    // a fact of the site lies on the clauses the item meets under the one
    // that refuses it, and the report calls each ambiguous by the fact's row
    for ((search, i), (_, _, finding)) in &want {
        let Some(row) = findings[finding]
            .row
            .as_ref()
            .filter(|_| findings[finding].home == "site")
        else {
            continue;
        };
        let met = &met[&(search.clone(), *i)];
        assert!(
            !met.is_empty(),
            "{search} item {i}: no clause the item meets"
        );
        for path in met {
            let c = translations[search]
                .clauses
                .iter()
                .find(|c| c.path.as_ref() == Some(path))
                .unwrap();
            assert!(
                c.rows.contains(row) && c.status == Status::Ambiguous,
                "{search} item {i}: {path} {:?} is {:?}, not ambiguous by {row}",
                c.term,
                c.status
            );
        }
    }
}

/// #185: the abyssal sockets as the site reads them (T36 to T38 and T42 in
/// `docs/design/trade-ground-truth.md`) — the Sockets and Link Groups
/// filters' `a`, a Sockets `max` of 0, and the stat `Has # Abyssal
/// Sockets` under implicit and explicit, with no bound and with one, its
/// singular `Has 1 Abyssal Socket` one under a bound. Each of zb01
/// to zb14, zc01, zc02 and ze01 to ze04 translates with no remainder,
/// those with no item among them, and every item the site fetched for one
/// matches its text; zc03's ten, a range's `max` of 0, are the
/// `quality-max-zero` finding's (#309).
#[test]
fn abyssal_sockets_each_search_of_the_sitting_runs_and_its_items_match() {
    let asked: Vec<String> = (1..=14)
        .map(|n| format!("zb{n:02}"))
        .chain(["zc01".to_string(), "zc02".to_string()])
        .chain((1..=4).map(|n| format!("ze{n:02}")))
        .collect();
    let searches: Vec<Search> = searches()
        .into_iter()
        .filter(|s| asked.contains(&s.name))
        .collect();
    let found: Vec<&str> = searches.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(found, asked, "every search of the sitting is captured");
    let mut fetched = 0;
    for search in &searches {
        let mut decoded = decode_json(&search.query).unwrap();
        decoded.league = Some(search.league.clone());
        let t = translate(&decoded).unwrap();
        assert!(
            t.remainder.is_empty(),
            "{}: `{}` leaves {:?}",
            search.name,
            t.text,
            t.remainder
        );
        let missed = disagreements(search, &t);
        assert!(
            missed.is_empty(),
            "{}: `{}` misses items {:?}",
            search.name,
            t.text,
            missed.iter().map(|d| d.item).collect::<Vec<_>>()
        );
        fetched += search.bodies.len();
    }
    // zb02, zb03, zb09, zb13 and ze02 found nothing; the fifteen others
    // fetched ten each (zc03 is the finding's)
    let empty: Vec<&str> = searches
        .iter()
        .filter(|s| s.bodies.is_empty())
        .map(|s| s.name.as_str())
        .collect();
    assert_eq!(empty, ["zb02", "zb03", "zb09", "zb13", "ze02"]);
    assert_eq!(fetched, 150);
}
