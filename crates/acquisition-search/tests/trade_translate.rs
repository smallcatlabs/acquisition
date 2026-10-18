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
    #[serde(default)]
    others: BTreeMap<String, Other>,
}

/// An ambiguous clause's other reading as a case works it by hand
/// (C119): the fragment, and the case's items it adds and drops — by
/// their ids, `m<n>` and `f<n>` — against the text as run.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Other {
    reading: String,
    #[serde(default)]
    adds: Vec<String>,
    #[serde(default)]
    drops: Vec<String>,
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
                // a count or a weighted sum with no bound of its own writes
                // no node of its own (t37, t38), the latter where one stat
                // is its presence alone
                let unbounded = by_at.get(c.at.as_str()).is_some_and(|g| {
                    ["min", "max"]
                        .iter()
                        .all(|k| g.site["value"].get(k).is_none_or(Json::is_null))
                });
                let running = t
                    .clauses
                    .iter()
                    .filter(|f| f.at.starts_with(&inside) && f.status.runs())
                    .count();
                let ownless = matches!(group_row(&c.at), Some("stats.and" | "stats.if"))
                    || (group_row(&c.at) == Some("stats.count") && unbounded)
                    || (matches!(group_row(&c.at), Some("stats.weight" | "stats.weight2"))
                        && unbounded
                        && running == 1)
                    || (group_row(&c.at).is_some() && none_runs())
                    || (matches!(group_row(group_of), Some("stats.weight" | "stats.weight2"))
                        && !by_at.get(group_of).is_some_and(|g| {
                            ["min", "max"]
                                .iter()
                                .all(|k| g.site["value"].get(k).is_none_or(Json::is_null))
                        }));
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

/// C119: an ambiguous clause whose row carries its other reading says what
/// is at stake — every fixture's readings are the ones it works by hand,
/// and a clause with none carries none (C93: absent, never zero) — and,
/// asked over the fixture's items as the site's search, the answer counts
/// the items the reading adds and drops against the text as run, each
/// count's route returning exactly those items. The trade seat's T4, T5
/// and T6 are rows of their own: the `reduced` form under a minimum, the
/// sum with and without its scourge line, the Blight-ravaged maps beside
/// the blighted at the tier bound.
#[test]
fn c119_an_ambiguous_clause_says_what_its_other_reading_adds_and_drops() {
    let mut read = 0;
    for case in &cases() {
        let t = translate(&decode(&input(case)).unwrap()).unwrap();
        let readings: BTreeMap<String, String> = t
            .clauses
            .iter()
            .filter_map(|c| c.other.as_ref().map(|o| (c.at.clone(), o.text.clone())))
            .collect();
        let expected: BTreeMap<String, String> = case
            .others
            .iter()
            .map(|(at, o)| (at.clone(), o.reading.clone()))
            .collect();
        assert_eq!(readings, expected, "{}", case.name);
        for c in t.clauses.iter().filter(|c| c.other.is_some()) {
            assert_eq!(c.status, Status::Ambiguous, "{}: {}", case.name, c.at);
        }
        if case.others.is_empty() || (case.matches.is_empty() && case.fails.is_empty()) {
            continue;
        }
        let s = store_of(case);
        let corpus = load(&s, Some("pc"));
        let request: acquisition_search::Request = serde_json::from_value(json!({
            "scope": { "realm": "pc" },
            "query": { "trade": input(case) },
            "view": { "rows": { "limit": 0 } },
        }))
        .unwrap();
        // unasked: a stake's route is one a next step hangs on (C115)
        let a = as_json(&acquisition_search::answer(&corpus, &request).unwrap());
        for (at, other) in &case.others {
            let clause = a["query"]["trade"]["clauses"]
                .as_array()
                .unwrap()
                .iter()
                .find(|c| c["at"] == json!(at))
                .unwrap_or_else(|| panic!("{}: no clause {at}", case.name));
            let stake = &clause["other"];
            assert_eq!(stake["reading"], json!(other.reading), "{}", case.name);
            for (side, expected) in [("adds", &other.adds), ("drops", &other.drops)] {
                let count = &stake[side];
                assert_eq!(
                    count["count"],
                    json!(expected.len()),
                    "{}: {at} {side}: {stake}",
                    case.name
                );
                if expected.is_empty() {
                    assert!(count.get("request").is_none(), "{}: {count}", case.name);
                    continue;
                }
                let route: acquisition_search::Request =
                    serde_json::from_value(count["request"].clone()).unwrap();
                let mut sorted = expected.clone();
                sorted.sort();
                assert_eq!(
                    ids(&as_json(&answer(&corpus, &route).unwrap())),
                    sorted,
                    "{}: {at} {side}: {}",
                    case.name,
                    count["request"]
                );
                read += 1;
            }
        }
    }
    assert!(read >= 5, "{read} stakes read");
    // every row that carries a reading is read as one by some fixture
    let catalogue = acquisition_search::trade::catalogue::catalogue().unwrap();
    let carried: BTreeSet<&str> = catalogue
        .rows()
        .iter()
        .filter(|r| r.other.is_some())
        .map(|r| r.key.as_str())
        .collect();
    assert!(!carried.is_empty());
    let mut met: BTreeSet<&str> = BTreeSet::new();
    for case in &cases() {
        let t = translate(&decode(&input(case)).unwrap()).unwrap();
        for c in t.clauses.iter().filter(|c| c.other.is_some()) {
            met.extend(
                c.rows
                    .iter()
                    .filter_map(|r| carried.get(r.as_str()).copied()),
            );
        }
    }
    assert_eq!(met, carried, "rows whose reading no fixture reads");
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
/// catalogue header's count — alone 11,232 exact, 2,607 ambiguous, 229
/// unsupported, the two-text ids every one; with a bound 7,357, 6,482 and
/// 229, 4,461 of them a stat a form of which no sum holds (step 7e) — 49
/// pairs carry a refused form either way, 300 a wording the game data
/// gives, and 1,613 a number written as a digit. Every pair and the counts where
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
        (json!({}), [11_232, 2_607, 229], 0),
        (json!({ "min": 1 }), [7_357, 6_482, 229], 4_461),
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
        assert_eq!(read("stat.text.wording"), 300);
    }
}

/// A site query with a null-valued key, `nulled`, beside every key of
/// every object it holds — the query object's, each group's, each filter's
/// and each value's.
fn with_nulls(value: &Json) -> Json {
    match value {
        Json::Object(o) => {
            let mut out: serde_json::Map<String, Json> =
                o.iter().map(|(k, v)| (k.clone(), with_nulls(v))).collect();
            out.insert("nulled".to_string(), Json::Null);
            Json::Object(out)
        }
        Json::Array(a) => Json::Array(a.iter().map(with_nulls).collect()),
        other => other.clone(),
    }
}

/// What a record's citation looks like in a reader's words: a decision,
/// claim or contract id (`C96`, `S26`, `K6`), a sitting's search (`t55`,
/// `u065`, `e043`), a file's path, or the builder's own words for its
/// process. The trade seat met each in the report and could not read it
/// (2026-10-17: every run).
fn cites(text: &str) -> Vec<String> {
    let mut found: Vec<String> = text
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| {
            let mut chars = w.chars();
            let Some(first) = chars.next() else {
                return false;
            };
            let rest: String = chars.collect();
            !rest.is_empty()
                && rest.chars().all(|c| c.is_ascii_digit())
                && (("CSKNJV".contains(first) && rest.len() <= 3)
                    || ("tuvwef".contains(first) && (2..=3).contains(&rest.len())))
        })
        .map(str::to_string)
        .collect();
    for word in [
        "tests/",
        "decisions/",
        "reference/",
        "search/",
        ".toml",
        ".rs",
        ".md",
        "sitting",
        "census",
        "the owner",
        "verbatim",
        "the park",
        "capture",
    ] {
        if text.contains(word) {
            found.push(word.to_string());
        }
    }
    found
}

/// The report's words are a reader's: every row's `says`, and every note a
/// clause carries over every fixture, cite no record (the trade seat,
/// 2026-10-17: all four runs met `S26`, `t55`, `decisions/trade.md` in the
/// clause notes; cli-opus call 14, mcp-opus calls 11, 13, mcp-sonnet calls
/// 10–19). The row's `note` keeps its evidence for `--describe trade`.
#[test]
fn the_reports_words_cite_no_record() {
    let c = acquisition_search::trade::catalogue::catalogue().unwrap();
    for row in c.rows() {
        if let Some(says) = &row.says {
            assert_eq!(cites(says), Vec::<String>::new(), "{}: {says}", row.key);
        }
        assert_eq!(row.note.is_some(), row.says.is_some(), "{}", row.key);
    }
    for case in cases() {
        let t = translate(&decode(&input(&case)).unwrap()).unwrap();
        for clause in &t.clauses {
            for note in &clause.notes {
                assert_eq!(
                    cites(note),
                    Vec::<String>::new(),
                    "{}: {}: {note}",
                    case.name,
                    clause.at
                );
            }
        }
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
        // the report's words cite no record, whatever the query
        for clause in &t.clauses {
            for note in &clause.notes {
                prop_assert!(cites(note).is_empty(), "{}: {}: {:?}", clause.at, note, cites(note));
            }
        }
        // the tree the report carries is the tree the text parses to
        let report = serde_json::to_value(&t).unwrap();
        let carried: Node = from_json(&report["tree"]).unwrap();
        prop_assert_eq!(&carried, &t.tree);
        // a key sent null is absent wherever it stands (u059; the outside
        // review of 8f293ff3): the query with a null beside every key it
        // holds, and every filter's own, translates as the query
        let nulled = translate(&decode(&with_nulls(&query).to_string()).unwrap())
            .map_err(|e| TestCaseError::fail(format!("{query}: {e}")))?;
        prop_assert_eq!(&nulled.text, &t.text);
        prop_assert_eq!(&nulled.clauses, &t.clauses);
        prop_assert_eq!(nulled.fidelity, t.fidelity);
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
