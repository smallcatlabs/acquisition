//! The translation (C113, K6; `trade/translate.rs`), step 4 of
//! `search/TRADE-PLAN.md`, through the crate's boundary: a decoded site
//! search in, the text and the report out, the text asked of items.
//!
//! - **Every fixture** (`trade-translate.toml`, its expected texts worked by
//!   hand from the catalogue's rows): the text, the scope, every clause's
//!   status, the fidelity and the remainder; the guard, checked here
//!   apart from the module's own check; a reported-only clause absent
//!   from the text and present in the report; and the text asked of its
//!   items — every match matches and no fail does, a fail being what a
//!   weaker reading would take.
//! - **Every catalogue row has a fixture**: the rows the fixtures' reports
//!   read, and the few no clause reads (the decoder's, the empty group).
//! - **Every stat id of the table translates**, alone and with a bound,
//!   and the statuses are what the catalogue's header counts.
//! - **A generated site query** — filters, groups and ids drawn from the
//!   catalogue and the stat table — translates, its text binds, its
//!   print reads back to the same tree, and the guard holds; and as a
//!   request's query, alone and beside its own text, it reads as the
//!   same translation (step 5).

mod common;

use std::collections::{BTreeMap, BTreeSet};

use acquisition_search::answer::QueryInput;
use acquisition_search::trade::translate::{Status, Translation};
use acquisition_search::tree::{self, Node};
use acquisition_search::{ErrorKind, decode, from_json, parse_query, print, translate};
use common::generated::{config, site_query};
use common::*;
use proptest::prelude::*;
use serde::Deserialize;
use serde_json::Value as Json;
use serde_json::{Value, json};

#[derive(Deserialize)]
struct Cases {
    case: Vec<Case>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    name: String,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    query: Option<String>,
    #[serde(default)]
    covers: Vec<String>,
    #[serde(default)]
    realm: Option<String>,
    text: String,
    fidelity: String,
    statuses: BTreeMap<String, String>,
    #[serde(default)]
    matches: Vec<String>,
    #[serde(default)]
    fails: Vec<String>,
}

#[derive(Deserialize)]
struct Captured {
    search: Vec<Search>,
}

#[derive(Deserialize)]
struct Search {
    name: String,
    id: String,
}

fn cases() -> Vec<Case> {
    toml::from_str::<Cases>(include_str!("trade-translate.toml"))
        .unwrap()
        .case
}

/// A case's input: its URL with each `{q…}` the captured id, or its JSON.
fn input(case: &Case) -> String {
    match (&case.url, &case.query) {
        (Some(url), None) => {
            let captured: Captured = toml::from_str(include_str!("trade-decode.toml")).unwrap();
            let mut url = url.clone();
            // the longest names first, so `{q2b}` is never read as `{q2}`
            let mut searches = captured.search;
            searches.sort_by_key(|s| std::cmp::Reverse(s.name.len()));
            for s in searches {
                url = url.replace(&format!("{{{}}}", s.name), &s.id);
            }
            url
        }
        (None, Some(query)) => query.clone(),
        _ => panic!("{}: a URL or a query, one", case.name),
    }
}

fn word(status: Status) -> String {
    serde_json::to_value(status)
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}

/// K6's guard, read off the report alone (the module's own check is
/// apart): every running clause owns one path of the tree or is one the
/// tree gives none — an `and` or `if` group, a group none of whose
/// filters runs, a filter inside a weighted sum; no path twice; every path of the tree under a clause's; a clause
/// that does not run owns none and writes nothing.
fn guard(t: &Translation) -> Result<(), String> {
    let report = serde_json::to_value(t).unwrap();
    let root = from_json(&report["tree"]).map_err(|e| e.to_string())?;
    let printed: BTreeMap<String, String> = tree::nodes(&root)
        .into_iter()
        .map(|p| (p.path, p.text))
        .collect();
    let under = |p: &str, o: &str| o.is_empty() || p == o || p.starts_with(&format!("{o}."));
    let by_at: BTreeMap<&str, &acquisition_search::trade::translate::Clause> =
        t.clauses.iter().map(|c| (c.at.as_str(), c)).collect();
    // a group's clause is the one whose deciding row is a group type's
    let group_row = |at: &str| {
        by_at
            .get(at)
            .and_then(|c| c.rows.first())
            .filter(|r| r.starts_with("stats."))
            .map(String::as_str)
    };
    let mut owned: BTreeSet<&str> = BTreeSet::new();
    for c in &t.clauses {
        if !c.status.runs() {
            if c.path.is_some() || c.term.is_some() {
                return Err(format!("{} does not run and writes {:?}", c.at, c.term));
            }
            continue;
        }
        let group_of = c.at.rsplit_once('.').map(|(g, _)| g).unwrap_or("");
        match &c.path {
            Some(path) => {
                let text = if path.is_empty() {
                    print(&root)
                } else {
                    printed
                        .get(path)
                        .cloned()
                        .ok_or_else(|| format!("{} owns {path:?}, no path of the tree", c.at))?
                };
                if !owned.insert(path) {
                    return Err(format!("{path:?} owned twice"));
                }
                // a filter of a group that writes a node lies inside it
                if c.at.starts_with("stats.")
                    && let Some(gp) = by_at.get(group_of).and_then(|g| g.path.as_deref())
                    && (path == gp || !under(path, gp))
                {
                    return Err(format!(
                        "{} at {path:?} is outside its group's {gp:?}",
                        c.at
                    ));
                }
                if c.term.as_deref() != Some(text.as_str()) {
                    return Err(format!("{}'s term is not its node's text", c.at));
                }
            }
            None => {
                let inside = format!("{}.", c.at);
                let none_runs = || {
                    !t.clauses
                        .iter()
                        .any(|f| f.at.starts_with(&inside) && f.status.runs())
                };
                let ownless = matches!(group_row(&c.at), Some("stats.and" | "stats.if"))
                    || (group_row(&c.at).is_some() && none_runs())
                    || matches!(group_row(group_of), Some("stats.weight" | "stats.weight2"));
                if !ownless {
                    return Err(format!("{} runs and owns no path", c.at));
                }
            }
        }
    }
    let owner_of = |p: &str| {
        owned.contains("")
            || owned.contains(p)
            || p.match_indices('.').any(|(at, _)| owned.contains(&p[..at]))
    };
    if let Some(stray) = printed.keys().find(|p| !owner_of(p)) {
        return Err(format!("{stray:?} is no clause's"));
    }
    Ok(())
}

/// What every translation holds, fixture or generated: the text binds and
/// reads back to the tree it was printed from; the remainder is the
/// unsupported clauses; the fidelity is the worst status; the guard.
fn holds(t: &Translation) -> Result<(), String> {
    let bound = parse_query(&t.text).map_err(|e| format!("`{}` is refused: {e}", t.text))?;
    if bound.tree() != &t.tree {
        return Err(format!("`{}` does not read back as its tree", t.text));
    }
    if print(bound.tree()) != t.text {
        return Err(format!("`{}` does not print back as itself", t.text));
    }
    let unsupported: Vec<&str> = t
        .clauses
        .iter()
        .filter(|c| c.status == Status::Unsupported)
        .map(|c| c.at.as_str())
        .collect();
    if t.remainder.iter().map(String::as_str).collect::<Vec<_>>() != unsupported {
        return Err("the remainder is not the unsupported clauses".to_string());
    }
    let fidelity = if !unsupported.is_empty() {
        "partial"
    } else if t.clauses.iter().any(|c| c.status == Status::Ambiguous) {
        "ambiguous"
    } else {
        "exact"
    };
    let said = serde_json::to_value(t.fidelity).unwrap();
    if said != fidelity {
        return Err(format!(
            "fidelity {said}, where the statuses say {fidelity}"
        ));
    }
    guard(t)
}

/// The fixture's items, each in its league's tab, ids `m<n>` and `f<n>`.
fn store_of(case: &Case) -> acquisition_store::Store {
    let mut s = store();
    let mut by_league: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for (prefix, bodies) in [("m", &case.matches), ("f", &case.fails)] {
        for (n, body) in bodies.iter().enumerate() {
            let mut more: Value = serde_json::from_str(body)
                .unwrap_or_else(|e| panic!("{}: an item is no JSON: {e}", case.name));
            let league = more
                .as_object_mut()
                .unwrap()
                .remove("_league")
                .and_then(|l| l.as_str().map(str::to_string))
                .unwrap_or_else(|| "Standard".to_string());
            let id = format!("{prefix}{n}");
            by_league
                .entry(league)
                .or_default()
                .push(item(&id, "", "Iron Ring", "Rare", more));
        }
    }
    for (league, items) in by_league {
        let tab_id = format!("t-{league}");
        list_tabs(&mut s, "pc", &league, json!([tab(&tab_id, "Tab")]), 10);
        fetch_tab(&mut s, "pc", &league, &tab_id, "Tab", items, 20);
    }
    s
}

/// C113, K6: every fixture translates to its text, its scope, its
/// statuses and its fidelity, worked by hand; the guard holds; and the
/// text, asked of the fixture's items, matches every match and no fail.
#[test]
fn c113_every_fixture_translates_as_worked_by_hand() {
    let cases = cases();
    assert!(cases.len() >= 80, "{} cases", cases.len());
    for case in &cases {
        let decoded = decode(&input(case)).unwrap_or_else(|e| panic!("{}: {e}", case.name));
        let t = translate(&decoded).unwrap_or_else(|e| panic!("{}: {e}", case.name));
        assert_eq!(t.text, case.text, "{}", case.name);
        assert_eq!(t.realm, case.realm, "{}", case.name);
        let statuses: BTreeMap<String, String> = t
            .clauses
            .iter()
            .map(|c| (c.at.clone(), word(c.status)))
            .collect();
        assert_eq!(
            statuses.len(),
            t.clauses.len(),
            "{}: an `at` twice",
            case.name
        );
        assert_eq!(statuses, case.statuses, "{}", case.name);
        assert_eq!(
            serde_json::to_value(t.fidelity).unwrap(),
            case.fidelity,
            "{}",
            case.name
        );
        holds(&t).unwrap_or_else(|e| panic!("{}: {e}", case.name));
        // a clause that does not run is in the report and nowhere in the
        // text; every clause that runs writes its term into it
        for c in &t.clauses {
            match &c.term {
                Some(term) => assert!(t.text.contains(term.as_str()), "{}: {}", case.name, c.at),
                None => assert!(
                    !c.status.runs() || c.path.is_none(),
                    "{}: {}",
                    case.name,
                    c.at
                ),
            }
            if matches!(c.status, Status::Ambiguous | Status::Unsupported) {
                assert!(!c.notes.is_empty(), "{}: {} has no note", case.name, c.at);
            }
        }
        if case.matches.is_empty() && case.fails.is_empty() {
            continue;
        }
        let s = store_of(case);
        let a = as_json(&ask(&load(&s, Some("pc")), &t.text).unwrap());
        let expected: Vec<String> = (0..case.matches.len()).map(|n| format!("m{n}")).collect();
        assert_eq!(ids(&a), expected, "{}: `{}`", case.name, t.text);
        assert_eq!(
            a["undecided"]["count"].as_u64().unwrap_or(0),
            0,
            "{}",
            case.name
        );
    }
}

/// C113: one test walks the catalogue — every row is read by some
/// fixture's report, or is one the decoder holds or the empty group,
/// which a fixture shows by what it covers; and a PoE2 URL is refused by
/// name before anything translates (`url.poe2`).
#[test]
fn c113_every_catalogue_row_has_a_fixture() {
    const UNREAD: [&str; 5] = ["url.realm", "url.id", "json", "query.stats", "url.poe2"];
    let mut read: BTreeSet<String> = BTreeSet::new();
    for case in cases() {
        let t = translate(&decode(&input(&case)).unwrap()).unwrap();
        for c in &t.clauses {
            read.extend(c.rows.iter().cloned());
        }
        for covered in &case.covers {
            assert!(
                UNREAD.contains(&covered.as_str()),
                "{}: {covered}",
                case.name
            );
            read.insert(covered.clone());
        }
    }
    let e = decode("https://www.pathofexile.com/trade2/search/poe2/Standard/x").unwrap_err();
    assert_eq!(e.kind, ErrorKind::Trade);
    assert!(e.message.contains("PoE1 only"), "{e}");
    read.insert("url.poe2".to_string());
    let catalogue = acquisition_search::trade::catalogue::catalogue().unwrap();
    let rows: BTreeSet<String> = catalogue.rows().iter().map(|r| r.key.clone()).collect();
    let missing: Vec<&String> = rows.difference(&read).collect();
    assert!(missing.is_empty(), "rows no fixture reads: {missing:?}");
    let unknown: Vec<&String> = read.difference(&rows).collect();
    assert!(unknown.is_empty(), "rows the catalogue lacks: {unknown:?}");
}

#[derive(Deserialize)]
struct Table {
    stat: Vec<Stat>,
}

#[derive(Deserialize)]
struct Stat {
    id: String,
    #[serde(rename = "in")]
    categories: Vec<String>,
}

/// C113, K6: every one of the stat table's category and id pairs
/// translates, alone and with a bound — each a filter of one `and`
/// group — and holds what every translation holds; the statuses are the
/// catalogue header's count — alone 11,474 exact, 2,365 ambiguous, 229
/// unsupported, the two-text ids every one; with a bound 7,546, 6,293 and
/// 229, 4,464 of them a stat a form of which no sum holds (step 7e) — 49
/// pairs carry a refused form either way, and 1,613 a number written as a
/// digit. Every pair and the counts where
/// `ACQ_CASES=full` (CI); here a sixteenth of them, a fresh one each run,
/// since a debug build binds the whole table's text in half a minute.
#[test]
fn c113_every_stat_id_of_the_table_translates() {
    let table: Table = toml::from_str(include_str!("../reference/trade-stats-v1.toml")).unwrap();
    let ids: Vec<String> = table
        .stat
        .iter()
        .flat_map(|s| s.categories.iter().map(move |c| format!("{c}.{}", s.id)))
        .collect();
    assert_eq!(ids.len(), 14_068);
    let stride = common::generated::cases(16, 1) as usize;
    let offset = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as usize
        % stride;
    let share: Vec<&String> = ids.iter().skip(offset).step_by(stride).collect();
    for (value, statuses, forms) in [
        (json!({}), [11_474, 2_365, 229], 0),
        (json!({ "min": 1 }), [7_546, 6_293, 229], 4_464),
    ] {
        let filters: Vec<Value> = share
            .iter()
            .map(|id| json!({ "id": id, "value": value }))
            .collect();
        let query = json!({ "stats": [{ "type": "and", "filters": filters }] });
        let t = translate(&decode(&query.to_string()).unwrap()).unwrap();
        holds(&t).unwrap_or_else(|e| panic!("every {stride}th pair from {offset}: {e}"));
        let each = &t.clauses[1..];
        assert_eq!(each.len(), share.len());
        if let Some(refused) = each
            .iter()
            .find(|c| c.rows.iter().any(|r| r == "query.unknown"))
        {
            panic!("{} is refused: {:?}", refused.at, refused.notes);
        }
        if stride > 1 {
            continue;
        }
        let count = |status: Status| each.iter().filter(|c| c.status == status).count();
        let read = |row: &str| {
            each.iter()
                .filter(|c| c.rows.iter().any(|r| r == row))
                .count()
        };
        assert_eq!(
            [
                count(Status::Exact),
                count(Status::Ambiguous),
                count(Status::Unsupported)
            ],
            statuses,
            "{value}"
        );
        assert_eq!(read("stat.text.two"), 229);
        assert_eq!(read("stat.text.numbers"), 1_613);
        assert_eq!(read("stat.text.forms"), forms, "{value}");
        assert_eq!(read("stat.text.refused"), 49);
    }
}

proptest! {
    #![proptest_config(config(64, 512))]

    /// C113, K6: a generated site query translates; its text binds and
    /// prints back as itself, its tree reads back, and the guard holds.
    #[test]
    fn c113_a_generated_site_query_translates_and_reads_back(query in site_query()) {
        let decoded = decode(&query.to_string()).map_err(|e| TestCaseError::fail(format!("{query}: {e}")))?;
        let t = translate(&decoded).map_err(|e| TestCaseError::fail(format!("{query}: {e}")))?;
        holds(&t).map_err(|e| TestCaseError::fail(format!("{query}: {e}")))?;
        // the tree the report carries is the tree the text parses to
        let report = serde_json::to_value(&t).unwrap();
        let carried: Node = from_json(&report["tree"]).unwrap();
        prop_assert_eq!(&carried, &t.tree);
        // as a request's query (step 5): the site search alone, as an
        // object and as its text, and beside the text it translates to —
        // what a printed request and a continuation send — read as the
        // translation, its report this build's
        for trade in [query.clone(), Json::String(query.to_string())] {
            for text in [None, Some(t.text.clone())] {
                let input = QueryInput { text, tree: None, trade: Some(trade.clone()) };
                let read = input
                    .read(None)
                    .map_err(|e| TestCaseError::fail(format!("{query}: {e}")))?;
                prop_assert_eq!(read.query.text(), t.text.clone());
                let again = read.translation.unwrap();
                prop_assert_eq!(again.clauses, t.clauses.clone());
                prop_assert_eq!(again.fidelity, t.fidelity);
            }
        }
    }
}
