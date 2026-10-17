//! The trade catalogue (C113, K6; step 0 of `search/TRADE-PLAN.md`, its
//! loader step 3's): the shipped file loads, holds a row for every filter,
//! stat category and group type the site's captures list, and names only
//! fields this build knows — the property fields among them, on the build
//! that writes them (C114) — and the loader refuses each defect a reviewer
//! would, by the row it names. The counts are the file's header's,
//! measured by script and again here.

use std::collections::BTreeSet;

use acquisition_search::parse_query;
use acquisition_search::trade::catalogue::{
    self, CATALOGUE_VERSION, Catalogue, CatalogueError, Disposition,
};
use serde_json::Value;

const FILTERS: &str = include_str!("../../../search/trade-query/data/filters-2026-09-12.json");
const STATS: &str = include_str!("../../../search/trade-query/data/stats-2026-09-12.json");
const GRAMMAR: &str = include_str!("../../../search/trade-query/data/grammar.json");

/// C113: the shipped catalogue loads at the version a basis will cite and
/// holds what its header says, status by status.
#[test]
fn c113_the_shipped_catalogue_loads_and_is_its_headers_count() {
    let c = catalogue::catalogue().unwrap();
    assert_eq!(c.version(), CATALOGUE_VERSION);
    assert_eq!(c.rows().len(), 143);
    let count = |d: Disposition| c.rows().iter().filter(|r| r.disposition == d).count();
    assert_eq!(
        [
            count(Disposition::Exact),
            count(Disposition::Ambiguous),
            count(Disposition::Unsupported),
            count(Disposition::ListingOnly)
        ],
        [76, 21, 37, 9]
    );
    // the four defence filters ambiguous since step 3 measured the q20
    // rule against the site (`tests/properties.rs`), and the digit rule no
    // capture shows (owner, 2026-10-16), and since step 6 an eldritch form
    // in a `not`, which the site's captures let through
    // (`tests/trade_oracle.rs`); since step 7e the damage at 20 % quality,
    // within one of the site's (`tests/trade_dps.rs`), a stat a form of
    // which no sum holds and a refused form (the park "A named stat
    // value"), and a map whose base names no tier (u065); since 7e's
    // outside review, another wording the game data gives, never vouched for
    for key in [
        "stat.text.numbers",
        "stat.text.eldritch",
        "stat.text.forms",
        "stat.text.refused",
        "stat.text.wording",
        "armour_filters.ar",
        "armour_filters.ev",
        "armour_filters.es",
        "armour_filters.ward",
        "weapon_filters.crit",
        "weapon_filters.dps",
        "weapon_filters.pdps",
        "map_filters.map_tier",
    ] {
        let row = c.row(key).unwrap();
        assert_eq!(row.disposition, Disposition::Ambiguous, "{key}");
    }
    // the rows the sitting of 2026-10-16 settled, built at step 7e: an
    // explicit id holds the stat's crafted and fractured lines (t13, t15),
    // a vessel's Monster Level (t46) and a heist item's own requirement
    // (t48) are the site's, and the elemental damage is its own figure
    for key in [
        "stat.explicit",
        "weapon_filters.edps",
        "map_filters.area_level",
        "heist_filters.heist_lockpicking",
        "heist_filters.heist_brute_force",
        "heist_filters.heist_perception",
        "heist_filters.heist_demolition",
        "heist_filters.heist_counter_thaumaturgy",
        "heist_filters.heist_trap_disarmament",
        "heist_filters.heist_agility",
        "heist_filters.heist_deception",
        "heist_filters.heist_engineering",
    ] {
        let row = c.row(key).unwrap();
        assert_eq!(row.disposition, Disposition::Exact, "{key}");
    }
    // no requirement meets a maximum (u061): the four requirement rows
    for key in [
        "req_filters.lvl",
        "req_filters.str",
        "req_filters.dex",
        "req_filters.int",
    ] {
        assert!(c.row(key).unwrap().lacked_meets_maximum, "{key}");
    }
    // the owner, 2026-10-15: "Unsupported until the sitting"
    assert_eq!(
        c.row("stat.text.two").unwrap().disposition,
        Disposition::Unsupported
    );
    assert_eq!(
        Catalogue::parse(catalogue::CATALOGUE_TOML)
            .unwrap()
            .rows()
            .len(),
        143
    );
}

/// C113, K6: every filter of the filters capture, every stat category of
/// the stats capture and every stat group type of the site's grammar has
/// its row — the status filter as the query's `status`.
#[test]
fn c113_every_clause_the_captures_list_has_a_row() {
    let c = catalogue::catalogue().unwrap();
    let filters: Value = serde_json::from_str(FILTERS).unwrap();
    let mut seen = 0;
    for group in filters["result"].as_array().unwrap() {
        let group_id = group["id"].as_str().unwrap();
        for filter in group["filters"].as_array().unwrap() {
            let id = filter["id"].as_str().unwrap();
            let key = if group_id == "status_filters" {
                format!("query.{id}")
            } else {
                format!("{group_id}.{id}")
            };
            assert!(c.row(&key).is_some(), "no row for {key}");
            seen += 1;
        }
    }
    assert_eq!(seen, 93);
    let stats: Value = serde_json::from_str(STATS).unwrap();
    let categories: Vec<&str> = stats["result"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["id"].as_str().unwrap())
        .collect();
    assert_eq!(categories.len(), 14);
    for category in categories {
        assert!(c.row(&format!("stat.{category}")).is_some(), "{category}");
    }
    let grammar: Value = serde_json::from_str(GRAMMAR).unwrap();
    let types: Vec<&str> = grammar["stat_groups"]
        .as_array()
        .unwrap()
        .iter()
        .map(|g| g["type"].as_str().unwrap())
        .collect();
    assert_eq!(types.len(), 8);
    for kind in types {
        assert!(c.row(&format!("stats.{kind}")).is_some(), "{kind}");
    }
}

/// C114: every field the catalogue names is one the language binds — the
/// loader's check, armed on the build that writes the property fields —
/// and those step 3 built are exactly the property table's and
/// `pseudo.edps`.
#[test]
fn c114_every_field_the_catalogue_names_binds() {
    let c = catalogue::catalogue().unwrap();
    let named: BTreeSet<&str> = c
        .rows()
        .iter()
        .flat_map(|r| r.fields.iter().map(String::as_str))
        .collect();
    assert_eq!(named.len(), 58);
    for field in &named {
        let term = match *field {
            "is" => "is:corrupted".to_string(),
            field => format!("has:{field}"),
        };
        assert!(
            parse_query(&term).is_ok(),
            "{term}: {:?}",
            parse_query(&term)
        );
    }
    // the fields the language had before step 3 …
    let before: BTreeSet<&str> = [
        "league",
        "name",
        "typeline",
        "base",
        "category",
        "rarity",
        "is",
        "pseudo.defence_pct",
        "sockets",
        "sockets.red",
        "sockets.green",
        "sockets.blue",
        "sockets.white",
        "reqlevel",
        "ilvl",
        "stack",
    ]
    .into_iter()
    .collect();
    assert!(before.is_subset(&named));
    // … and the 42 built since: step 3's 40, the property table's 39 and
    // one computed value, and step 7e's two, the damage at 20 % quality the
    // dps rows land on in place of `pseudo.dps` and `pseudo.pdps` (t41, t42)
    let built: BTreeSet<&str> = named.difference(&before).copied().collect();
    assert_eq!(built.len(), 42);
    let mut table: BTreeSet<&str> = acquisition_search::properties::table()
        .unwrap()
        .fields()
        .iter()
        .map(|f| f.name())
        .collect();
    table.extend(["pseudo.edps", "pseudo.dps_q20", "pseudo.pdps_q20"]);
    assert_eq!(built, table);
}

const HEAD: &str = "version = 1\nfilters = \"2026-09-12\"\nstats = \"2026-09-12\"\n";

fn row(key: &str, body: &str) -> String {
    format!("[[row]]\nkey = \"{key}\"\n{body}\n")
}

/// What the loader refuses, each by the row it names.
#[test]
fn c113_the_catalogue_loader_refuses_what_a_reviewer_would() {
    let good = row(
        "misc_filters.ilvl",
        "text = \"Item Level\"\nshape = \"minmax\"\ndisposition = \"exact\"\nlands = \"ilvl{bounds}\"\nfields = [\"ilvl\"]",
    );
    assert_eq!(
        Catalogue::parse(&format!("{HEAD}{good}"))
            .unwrap()
            .rows()
            .len(),
        1
    );
    let refused = |text: String| Catalogue::parse(&text).unwrap_err();
    let key = || "a.b".to_string();
    let one = |body: &str| format!("{HEAD}{}", row("a.b", body));
    assert_eq!(
        refused(format!(
            "{}{good}",
            HEAD.replace("version = 1", "version = 2")
        )),
        CatalogueError::Version {
            found: 2,
            expected: 1
        }
    );
    assert_eq!(
        refused(format!(
            "{}{good}",
            HEAD.replace("filters = \"2026-09-12\"", "filters = \"2026-10-01\"")
        )),
        CatalogueError::Capture {
            which: "filters",
            found: "2026-10-01".to_string(),
            expected: "2026-09-12"
        }
    );
    assert_eq!(
        refused(format!(
            "{}{good}",
            HEAD.replace("stats = \"2026-09-12\"", "stats = \"2026-10-01\"")
        )),
        CatalogueError::Capture {
            which: "stats",
            found: "2026-10-01".to_string(),
            expected: "2026-09-12"
        }
    );
    assert_eq!(refused(HEAD.to_string()), CatalogueError::NoRow);
    for bad in ["", "A.b", "a b", "a..b", ".a", "a.", "a=b=c", "a.b=", "a-b"] {
        assert_eq!(
            refused(format!("{HEAD}{}", row(bad, "disposition = \"exact\""))),
            CatalogueError::BadKey(bad.to_string()),
            "{bad:?}"
        );
    }
    assert!(Catalogue::parse(&one("disposition = \"exact\"")).is_ok());
    assert!(
        Catalogue::parse(&format!(
            "{HEAD}{}",
            row("a.b=c_1", "disposition = \"exact\"")
        ))
        .is_ok()
    );
    assert_eq!(
        refused(format!("{HEAD}{good}{good}")),
        CatalogueError::DuplicateKey("misc_filters.ilvl".to_string())
    );
    assert_eq!(
        refused(one("disposition = \"disabled\"")),
        CatalogueError::UnknownDisposition {
            key: key(),
            disposition: "disabled".to_string()
        }
    );
    assert_eq!(
        refused(one("disposition = \"exact\"\nshape = \"range\"")),
        CatalogueError::UnknownShape {
            key: key(),
            shape: "range".to_string()
        }
    );
    for what in ["ambiguous", "unsupported"] {
        assert_eq!(
            refused(one(&format!("disposition = \"{what}\""))),
            CatalogueError::NoNote(key()),
            "{what}"
        );
    }
    for (body, what) in [
        ("disposition = \"exact\"\ntext = \" \"", "text"),
        ("disposition = \"exact\"\nnote = \"\"", "note"),
        ("disposition = \"exact\"\nlands = \" \"", "landing"),
    ] {
        assert_eq!(
            refused(one(body)),
            CatalogueError::Empty { key: key(), what },
            "{body}"
        );
    }
    for what in ["unsupported", "listing_only"] {
        assert_eq!(
            refused(one(&format!(
                "disposition = \"{what}\"\nnote = \"n\"\nlands = \"ilvl{{bounds}}\""
            ))),
            CatalogueError::LandsWithoutRunning(key()),
            "{what}"
        );
    }
    assert_eq!(
        refused(one("disposition = \"exact\"\nlands = \"ilvl>={least}\"")),
        CatalogueError::UnknownPlaceholder {
            key: key(),
            placeholder: "least".to_string()
        }
    );
    // a brace never closed is no placeholder the translation fills
    assert_eq!(
        refused(one("disposition = \"exact\"\nlands = \"ilvl>={min\"")),
        CatalogueError::UnknownPlaceholder {
            key: key(),
            placeholder: "{min".to_string(),
        }
    );
    // a placeholder inside quotes, where the printer's quoting of what
    // fills it cannot reach (step 4); a quote escaped inside a quoted
    // string does not end it
    for lands in ["'league=\"{value}\"'", "'name=\"a\\\" {value}\"'"] {
        assert_eq!(
            refused(one(&format!("disposition = \"exact\"\nlands = {lands}"))),
            CatalogueError::QuotedPlaceholder {
                key: key(),
                placeholder: "value".to_string()
            },
            "{lands}"
        );
    }
    // bare, or beside a quoted text that holds none, it stands
    for lands in ["'name={value} typeline~\"^a\"'", "'name=\"a\\\\\" {value}'"] {
        assert!(
            Catalogue::parse(&one(&format!("disposition = \"exact\"\nlands = {lands}"))).is_ok(),
            "{lands}"
        );
    }
    assert_eq!(
        refused(one("disposition = \"exact\"\nfields = [\"ilvl\"]")),
        CatalogueError::FieldsWithoutLands(key())
    );
    // a field named and not written — `ilvl` is no part of `reqlevel`
    assert_eq!(
        refused(one(
            "disposition = \"exact\"\nlands = \"reqlevel{bounds}\"\nfields = [\"ilvl\", \"reqlevel\"]"
        )),
        CatalogueError::FieldNotLanded {
            key: key(),
            field: "ilvl".to_string()
        }
    );
    assert_eq!(
        refused(one(
            "disposition = \"exact\"\nlands = \"armour.q2{bounds}\"\nfields = [\"armour.q2\"]"
        )),
        CatalogueError::UnknownField {
            key: key(),
            field: "armour.q2".to_string()
        }
    );
    // a computed value the build does not define, and `is` as a field
    assert_eq!(
        refused(one(
            "disposition = \"exact\"\nlands = \"pseudo.adps{bounds}\"\nfields = [\"pseudo.adps\"]"
        )),
        CatalogueError::UnknownField {
            key: key(),
            field: "pseudo.adps".to_string()
        }
    );
    assert!(
        Catalogue::parse(&one(
            "disposition = \"exact\"\nlands = \"is:split armour.q20{bounds} pseudo.edps>={r}\"\nfields = [\"is\", \"armour.q20\", \"pseudo.edps\"]"
        ))
        .is_ok()
    );
    for broken in [
        "version = 1\n".to_string(),
        one("disposition = \"exact\"\nwhat = 1"),
        format!("{HEAD}[[row]]\ndisposition = \"exact\"\n"),
    ] {
        assert!(
            matches!(refused(broken.clone()), CatalogueError::Parse(_)),
            "{broken}"
        );
    }
}
