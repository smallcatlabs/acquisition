//! The forms table (C113, C106; step 7e of `search/TRADE-PLAN.md`, "The
//! choice before 7e"): the other displays of a stat the trade site files
//! under one id, read by the translation alone.
//!
//! - **The vocabulary is walked** and every form the game, a rule or a row
//!   makes that is itself a text the site lists, and every text a rule
//!   meets carrying both words of a pair, is refused by name in the table
//!   — no more and no fewer. The brief carried note 37's counts (7 of 372
//!   `#% chance to`, 2 of 286 `have #% chance to`, 4 texts with both sign
//!   words), measured on the raw texts with case kept and every text under
//!   the rules; the rules as built stand for the texts the export joins
//!   nowhere, and what they reach and refuse there is pinned here.
//! - **The loader refuses what a reviewer would**, by the row it names.
//! - **The captures hold the table**: every line the site filed under an
//!   id the translation runs, in every listing it sent, is met by that
//!   id's presence term, but where the id's clause is ambiguous by a
//!   refusal, its form left to the site.

mod common;

use std::collections::{BTreeMap, BTreeSet};

use acquisition_search::trade::forms::{
    self, FORMS_TOML, FORMS_VERSION, FormsError, FormsTable, GAME_FORMS_TOML,
};
use acquisition_search::trade::stats;
use acquisition_search::trade::translate::Status;
use acquisition_search::{Request, Translation, answer, decode_json, translate};
use common::*;
use serde_json::{Value, json};

const BODIES: &str = include_str!("../../../search/pseudo-stats/data/bodies.json");

/// The choice before 7e: the shipped table loads at its version and the
/// stat table's date; over the stat table's distinct texts, the game's
/// forms reach 2,655 of the 6,841 the export joins, and the rules the
/// texts it joins nowhere — the opposite words 618, `#% chance to` 24,
/// `have #% chance to` 193, `have a` 22 — and the rows 13; and the
/// refusals are exactly what the walk finds: 6 forms of the game's that
/// are listed texts, 1 a row's, 5 by `have #% chance to` and 2 by `have a`,
/// none by `#% chance to` and no text a rule meets with both words of a
/// pair.
#[test]
fn the_choice_before_7e_every_collision_is_refused_by_name() {
    let stats = stats::table().unwrap();
    let table = forms::table().unwrap();
    assert_eq!(table.version(), FORMS_VERSION);
    let walk = table.walk(stats);
    let reach: Vec<(&str, usize)> = walk.applies.iter().map(|(r, n)| (r.as_str(), *n)).collect();
    assert_eq!(
        reach,
        [
            ("opposite", 618),
            ("chance", 24),
            ("have", 193),
            ("have_a", 22),
            ("game", 2655),
            ("form", 13)
        ]
    );
    let refused: BTreeSet<(String, String, Option<String>)> = table
        .refused()
        .iter()
        .map(|r| (r.text.clone(), r.rule.clone(), r.form.clone()))
        .collect();
    assert_eq!(refused.len(), table.refused().len(), "a refusal twice");
    let must: BTreeSet<(String, String, Option<String>)> = walk.must_refuse.into_iter().collect();
    let unnamed: Vec<_> = must.difference(&refused).collect();
    let stale: Vec<_> = refused.difference(&must).collect();
    assert!(
        unnamed.is_empty() && stale.is_empty(),
        "collisions the table does not refuse: {unnamed:#?}\nrefusals no collision asks: {stale:#?}"
    );
    let by_rule = |rule: &str, form: bool| {
        refused
            .iter()
            .filter(|(_, r, f)| r == rule && f.is_some() == form)
            .count()
    };
    assert_eq!(
        [
            by_rule("game", true),
            by_rule("form", true),
            by_rule("have", true),
            by_rule("have_a", true),
            by_rule("chance", true),
            by_rule("opposite", false),
            by_rule("opposite", true),
        ],
        [6, 1, 5, 2, 0, 0, 0]
    );
}

/// The loader refuses each defect by the row it names.
#[test]
fn the_forms_table_loader_refuses_what_a_reviewer_would() {
    let stats = stats::table().unwrap();
    let head = "version = 1\nstats = \"2026-09-12\"\nsource = \"s\"\n";
    // a game file joining nothing, so a hand row stands alone
    const GAME: &str =
        "version = 1\nstats = \"2026-09-12\"\nsource = \"s\"\ngenerated_by = \"g\"\n";
    let parse = |body: &str| FormsTable::parse(&format!("{head}{body}"), GAME, stats);
    assert!(parse("").is_ok());
    assert!(matches!(
        FormsTable::parse(
            &FORMS_TOML.replacen("version = 1", "version = 2", 1),
            GAME_FORMS_TOML,
            stats
        ),
        Err(FormsError::Version { found: 2, .. })
    ));
    assert!(matches!(
        FormsTable::parse(
            &FORMS_TOML.replacen("stats = \"2026-09-12\"", "stats = \"2026-09-13\"", 1),
            GAME_FORMS_TOML,
            stats
        ),
        Err(FormsError::Stats { .. })
    ));
    assert!(matches!(
        FormsTable::parse(
            "version = 1\nstats = \"2026-09-12\"\nsource = \" \"\n",
            GAME,
            stats
        ),
        Err(FormsError::NoSource)
    ));
    let pair = |words: &str| parse(&format!("[[opposite]]\nwords = {words}\n"));
    assert!(matches!(pair("[\"increased\"]"), Err(FormsError::Pair(_))));
    assert!(matches!(
        pair("[\"in creased\", \"x\"]"),
        Err(FormsError::Pair(_))
    ));
    assert!(matches!(
        parse(
            "[[opposite]]\nwords = [\"more\", \"less\"]\n[[opposite]]\nwords = [\"less\", \"fewer\"]\n"
        ),
        Err(FormsError::Pair(_))
    ));
    let elision = |body: &str| parse(&format!("[[elision]]\n{body}\n"));
    assert!(matches!(
        elision("key = \"x\"\ndrop = \"chance to \"\nnumber = 100"),
        Err(FormsError::Elision(_))
    ));
    assert!(matches!(
        elision("key = \"x\"\ndrop = \"#% chance to \"\nat = \"end\"\nnumber = 100"),
        Err(FormsError::Elision(_))
    ));
    assert!(matches!(
        elision("key = \"opposite\"\ndrop = \"#% chance to \"\nnumber = 100"),
        Err(FormsError::Elision(_))
    ));
    let form = |text: &str, form: &str, kind: &str, more: &str| {
        parse(&format!(
            "[[form]]\ntext = {text:?}\nform = {form:?}\nkind = {kind:?}\n{more}\n"
        ))
    };
    let pierce = "Projectiles Pierce # additional Targets";
    assert!(
        form(
            pierce,
            "Projectiles Pierce an additional Target",
            "numberless",
            "number = 1"
        )
        .is_ok()
    );
    assert!(matches!(
        form("No such text", "x", "other", ""),
        Err(FormsError::NotListed(_))
    ));
    assert!(matches!(
        form(
            pierce,
            "Projectiles Pierce # additional targets",
            "other",
            ""
        ),
        Err(FormsError::Form {
            why: "is its own text",
            ..
        })
    ));
    assert!(matches!(
        form(pierce, "x", "odd", ""),
        Err(FormsError::Form { .. })
    ));
    assert!(matches!(
        form(pierce, "x", "other", "number = 1"),
        Err(FormsError::Form { .. })
    ));
    assert!(matches!(
        form(
            pierce,
            "Projectiles Pierce an additional Target",
            "plural",
            ""
        ),
        Err(FormsError::Form { .. })
    ));
    let twice = format!(
        "[[form]]\ntext = {pierce:?}\nform = \"A\"\nkind = \"other\"\n[[form]]\ntext = {pierce:?}\nform = \"a\"\nkind = \"other\"\n"
    );
    assert!(matches!(
        parse(&twice),
        Err(FormsError::Form {
            why: "is given twice",
            ..
        })
    ));
    let refused = |rule: &str, form: Option<&str>| {
        let form = form.map_or(String::new(), |f| format!("form = {f:?}\n"));
        parse(&format!(
            "[[elision]]\nkey = \"chance\"\ndrop = \"#% chance to \"\nat = \"start\"\nnumber = 100\n[[refused]]\ntext = \"#% chance to Curse Enemies with Enfeeble on Hit\"\nrule = {rule:?}\n{form}"
        ))
    };
    assert!(refused("chance", Some("Curse Enemies with Enfeeble on Hit")).is_ok());
    assert!(matches!(
        refused("nothing", Some("Curse Enemies with Enfeeble on Hit")),
        Err(FormsError::Refused { .. })
    ));
    assert!(matches!(
        refused("chance", None),
        Err(FormsError::Refused { .. })
    ));
    assert!(matches!(
        refused("opposite", Some("Curse Enemies with Enfeeble on Hit")),
        Err(FormsError::Refused { .. })
    ));
    assert!(matches!(
        refused("chance", Some("No such text")),
        Err(FormsError::NotListed(_))
    ));
}

/// The game's forms first, the rules beside (the owner, 2026-10-17): a
/// text the export joins takes its forms and no rule — `#% chance to
/// Suppress Spell Damage` has none, where the brief's elision would have
/// named `Suppress Spell Damage`, a display the game never shows — and a
/// text it joins nowhere takes the rules; a row the export already gives
/// is refused, and so is a game file of another version.
#[test]
fn the_game_s_forms_stand_where_the_export_joins_and_the_rules_where_it_does_not() {
    let stats = stats::table().unwrap();
    let table = forms::table().unwrap();
    let texts = |site: &str| -> Vec<(String, &'static str)> {
        table
            .of(site)
            .known
            .into_iter()
            .map(|f| (f.text, f.kind.word()))
            .collect()
    };
    assert_eq!(texts("+#% chance to Suppress Spell Damage"), []);
    assert_eq!(
        texts("#% chance to Trigger Commandment of Ire when Hit"),
        [(
            "Trigger Commandment of Ire when Hit".to_string(),
            "numberless"
        )]
    );
    assert_eq!(
        texts("#% increased Physical Damage"),
        [
            ("No Physical Damage".to_string(), "numberless"),
            ("#% reduced Physical Damage".to_string(), "opposite")
        ]
    );
    // the same text, joined nowhere, takes the rules
    let bare = "version = 1\nstats = \"2026-09-12\"\nsource = \"s\"\ngenerated_by = \"g\"\n";
    let rules = FormsTable::parse(FORMS_TOML, bare, stats).unwrap();
    let ruled: Vec<String> = rules
        .of("+#% chance to Suppress Spell Damage")
        .known
        .into_iter()
        .map(|f| f.text)
        .collect();
    assert_eq!(ruled, ["Suppress Spell Damage"]);
    // a row the export gives is refused
    let row = format!(
        "{}\n[[form]]\ntext = \"#% increased Physical Damage\"\nform = \"No Physical Damage\"\nkind = \"numberless\"\n",
        FORMS_TOML.split("\n[[form]]").next().unwrap()
    );
    assert!(matches!(
        FormsTable::parse(&row, GAME_FORMS_TOML, stats),
        Err(FormsError::Form {
            why: "is one the game's forms give",
            ..
        })
    ));
    assert!(matches!(
        FormsTable::parse(
            FORMS_TOML,
            &GAME_FORMS_TOML.replacen("version = 1", "version = 2", 1),
            stats
        ),
        Err(FormsError::Version { found: 2, .. })
    ));
    let joined = |forms: &str| {
        FormsTable::parse(
            FORMS_TOML.split("\n[[form]]").next().unwrap(),
            &format!("{bare}[[text]]\ntext = \"Has # Abyssal Sockets\"\nforms = {forms}\n"),
            stats,
        )
    };
    assert!(joined("[]").is_ok());
    assert!(matches!(
        joined("[{ form = \"Has 1 Abyssal Socket\", kind = \"plural\" }]"),
        Err(FormsError::Form { .. })
    ));
    assert!(matches!(
        FormsTable::parse(
            FORMS_TOML.split("\n[[form]]").next().unwrap(),
            &format!("{bare}[[text]]\ntext = \"No such text\"\nforms = []\n"),
            stats
        ),
        Err(FormsError::NotListed(_))
    ));
}

/// The captures hold the table: over every listing the site sent (the
/// sittings' and the owner's census, each search's items a corpus of
/// their own), every one of the 21,616 lines under an id of the six hash
/// categories is met by the id's presence term, translated alone — 21,555
/// — but 56 under an id the translation leaves unsupported (an id of two
/// texts) and 5 under an id whose clause is ambiguous by a refusal, its form
/// left to the site (the Staves, Legacy and Global Critical Strike Chance
/// suffixes, and two Shocks within `1.5 metres`).
#[test]
fn the_choice_before_7e_every_captured_line_is_met_by_its_ids_presence() {
    let bodies: Value = serde_json::from_str(BODIES).unwrap();
    let mut presence: BTreeMap<String, Translation> = BTreeMap::new();
    let (mut lines, mut met, mut unsupported, mut refused) = (0, 0, 0, 0);
    let mut faults: Vec<String> = Vec::new();
    for group in ["searches", "census"] {
        for (name, items) in bodies[group].as_object().unwrap() {
            let items = items.as_array().unwrap();
            // each item under its index, every line's id by the items it is on
            let mut on: BTreeMap<String, BTreeSet<usize>> = BTreeMap::new();
            let mut by_league: BTreeMap<String, Vec<Value>> = BTreeMap::new();
            for (i, item) in items.iter().enumerate() {
                for (key, mods) in item.as_object().unwrap() {
                    if !key.ends_with("Mods") {
                        continue;
                    }
                    for m in mods.as_array().unwrap() {
                        let Some(id) = m["hash"].as_str().and_then(|h| h.strip_prefix("stat."))
                        else {
                            continue;
                        };
                        if stats::HASH_CATEGORIES.contains(&id.split('.').next().unwrap()) {
                            lines += 1;
                            on.entry(id.to_string()).or_default().insert(i);
                        }
                    }
                }
                let mut body = item.clone();
                body["id"] = json!(i.to_string());
                body["x"] = json!(i);
                body["y"] = json!(0);
                let league = item["league"].as_str().unwrap().to_string();
                by_league.entry(league).or_default().push(body);
            }
            let mut s = store();
            for (league, bodies) in by_league {
                list_tabs(&mut s, "pc", &league, json!([tab("t", "Fetched")]), 10);
                fetch_tab(&mut s, "pc", &league, "t", "Fetched", bodies, 20);
            }
            let corpus = load(&s, Some("pc"));
            for (id, holding) in on {
                let t = presence.entry(id.clone()).or_insert_with(|| {
                    let q = json!({ "query": { "stats": [{ "type": "and", "filters": [{ "id": id }] }] } });
                    translate(&decode_json(&q).unwrap()).unwrap()
                });
                // a line per item the id is on, however many it carries
                let count = |items: &BTreeSet<usize>| -> usize {
                    items
                        .iter()
                        .map(|&i| lines_of(&items_of(&bodies, group, name)[i], &id))
                        .sum()
                };
                if !t.remainder.is_empty() {
                    unsupported += count(&holding);
                    continue;
                }
                let matched = matching(&corpus, &t.text);
                let (hit, missed): (BTreeSet<usize>, BTreeSet<usize>) =
                    holding.iter().partition(|i| matched.contains(i));
                met += count(&hit);
                if missed.is_empty() {
                    continue;
                }
                let clause = &t.clauses.iter().find(|c| c.at == "stats.0.0").unwrap();
                if clause.status == Status::Ambiguous
                    && clause.rows.iter().any(|r| r == "stat.text.refused")
                {
                    refused += count(&missed);
                } else {
                    faults.push(format!("{name} {id} items {missed:?}: `{}`", t.text));
                }
            }
        }
    }
    assert!(faults.is_empty(), "{}", faults.join("\n"));
    assert_eq!((lines, met, unsupported, refused), (21_616, 21_555, 56, 5));
}

fn items_of<'a>(bodies: &'a Value, group: &str, name: &str) -> &'a [Value] {
    bodies[group][name].as_array().unwrap()
}

/// How many of an item's lines the site filed under an id.
fn lines_of(item: &Value, id: &str) -> usize {
    let hash = format!("stat.{id}");
    item.as_object()
        .unwrap()
        .iter()
        .filter(|(k, _)| k.ends_with("Mods"))
        .flat_map(|(_, mods)| mods.as_array().unwrap())
        .filter(|m| m["hash"].as_str() == Some(hash.as_str()))
        .count()
}

/// The indices of the items a text matches.
fn matching(corpus: &acquisition_search::Corpus, text: &str) -> BTreeSet<usize> {
    let request: Request = serde_json::from_value(
        json!({ "query": { "text": text }, "view": { "rows": { "limit": 1000 } } }),
    )
    .unwrap();
    as_json(&answer(corpus, &request).unwrap_or_else(|e| panic!("{text}: {e}")))["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["id"].as_str().unwrap().parse().unwrap())
        .collect()
}
