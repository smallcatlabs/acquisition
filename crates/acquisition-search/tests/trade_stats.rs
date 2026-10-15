//! The stat table (C113, C106; step 2 of `search/TRADE-PLAN.md`): the
//! shipped file loads at the capture's date, answers every entry of the
//! capture it was generated from and nothing the capture lacks, and the
//! loader refuses each defect a reviewer would, by the row it names. The
//! counts are the generator's, measured at generation and printed in the
//! file's header; the table regenerating byte-for-byte is
//! `tools/trade-stats-table.py --check`.

use std::collections::BTreeMap;

use acquisition_search::trade::stats::{
    self, HASH_CATEGORIES, STAT_TABLE_VERSION, StatTable, StatTableError,
};
use serde_json::Value;

/// The capture the table is generated from, committed beside the track.
const CAPTURE: &str = include_str!("../../../search/trade-query/data/stats-2026-09-12.json");

/// C113: the shipped table loads, is the version a basis will cite, and
/// holds what the generator measured.
#[test]
fn c113_the_shipped_stat_table_is_the_capture_of_2026_09_12() {
    let t = stats::table().unwrap();
    assert_eq!(t.version(), STAT_TABLE_VERSION);
    assert_eq!(STAT_TABLE_VERSION, "2026-09-12");
    assert!(
        t.source().contains("stats-2026-09-12.json"),
        "{}",
        t.source()
    );
    assert_eq!(t.ids(), 10303);
    // one id under five categories, one text under all of them
    let life = ["+# to maximum Life".to_string()];
    for category in ["explicit", "implicit", "fractured", "scourge", "crafted"] {
        assert_eq!(
            t.texts(&format!("{category}.stat_3299347043")),
            Some(&life[..])
        );
    }
    assert_eq!(t.texts("enchant.stat_3299347043"), None);
    assert_eq!(
        t.categories("stat_3299347043"),
        ["explicit", "implicit", "fractured", "scourge", "crafted"]
    );
    // an id the site lists twice in one category keeps both texts, in the
    // capture's order: a map's own wording, then the atlas's
    assert_eq!(
        t.texts("explicit.stat_1145451936").unwrap(),
        [
            "Area contains The Sacred Grove",
            "Your Maps have +#% chance to contain The Sacred Grove"
        ]
    );
    // a text of several rows, its breaks kept
    assert_eq!(
        t.texts("fractured.stat_1353527704").unwrap()[1],
        "When a fifth Impale is inflicted on a Player,\nImpales are removed to Reflect their Physical Damage multiplied\nby their remaining Hits to that Player and their Allies within 1.8 metres"
    );
    // ids of other shapes are rows like any
    assert_eq!(
        t.texts("explicit.indexable_support_73").unwrap(),
        ["Socketed Gems are Supported by Level # Ice Bite"]
    );
    assert_eq!(
        t.texts("enchant.delirium_reward_abyss").unwrap(),
        ["Delirium Reward Type: Abyss Items"]
    );
    assert!(t.texts("explicit.stat_4089743927|1|66").is_some());
    // what the table does not carry: a pseudo, a category that is no hash
    // category, an id no category carries, a stat with no category
    assert_eq!(t.texts("pseudo.pseudo_total_cold_resistance"), None);
    assert_eq!(t.texts("veiled.mod_63099"), None);
    assert_eq!(t.texts("explicit.stat_0"), None);
    assert_eq!(t.texts("stat_3299347043"), None);
    assert!(t.categories("stat_0").is_empty());
    // the file on disk is the one the loader reads
    assert_eq!(
        StatTable::parse(stats::STAT_TABLE_TOML).unwrap().ids(),
        t.ids()
    );
}

/// C113, C90: the table is the capture's six hash categories, whole and
/// no more. Every entry of each is answered with the texts that category
/// gives the id, in order; every id of the table is one the capture
/// gives, under exactly the categories it gives it; and no entry of the
/// other eight categories is answered.
#[test]
fn c113_the_stat_table_answers_every_hash_entry_of_the_capture_and_no_other() {
    let t = stats::table().unwrap();
    let capture: Value = serde_json::from_str(CAPTURE).unwrap();
    // id -> category -> texts, as the capture gives them
    let mut given: BTreeMap<&str, BTreeMap<&str, Vec<&str>>> = BTreeMap::new();
    let mut entries = 0;
    let mut other = 0;
    for category in capture["result"].as_array().unwrap() {
        let name = category["id"].as_str().unwrap();
        for entry in category["entries"].as_array().unwrap() {
            let stat = entry["id"].as_str().unwrap();
            let text = entry["text"].as_str().unwrap();
            if HASH_CATEGORIES.contains(&name) {
                let id = stat.strip_prefix(&format!("{name}.")).unwrap();
                given
                    .entry(id)
                    .or_default()
                    .entry(name)
                    .or_default()
                    .push(text);
                entries += 1;
            } else {
                assert_eq!(t.texts(stat), None, "{stat}");
                other += 1;
            }
        }
    }
    assert_eq!(entries, 14297);
    assert_eq!(other, 18187 - 14297);
    assert_eq!(given.len(), t.ids());
    for (id, under) in &given {
        assert_eq!(
            t.categories(id),
            HASH_CATEGORIES
                .iter()
                .copied()
                .filter(|c| under.contains_key(c))
                .collect::<Vec<_>>(),
            "{id}"
        );
        for (category, texts) in under {
            assert_eq!(
                t.texts(&format!("{category}.{id}")).unwrap(),
                texts.as_slice(),
                "{category}.{id}"
            );
        }
    }
}

const HEAD: &str = "version = \"2026-09-12\"\nsource = \"s\"\ngenerated_by = \"t\"\n";

/// A table of one id under every hash category.
fn one() -> String {
    format!(
        "{HEAD}[[stat]]\nid = \"stat_1\"\nin = [\"explicit\", \"implicit\", \"fractured\", \"enchant\", \"scourge\", \"crafted\"]\ntexts = [\"+# to maximum Life\"]\n"
    )
}

/// What the loader refuses, each by the row it names.
#[test]
fn c113_the_stat_table_loader_refuses_what_a_reviewer_would() {
    let t = StatTable::parse(&one()).unwrap();
    assert_eq!(t.ids(), 1);
    let id = || "stat_1".to_string();
    let second = |row: &str| format!("{}\n[[stat]]\n{row}\n", one());
    for (bad, expected) in [
        (
            one().replace("2026-09-12", "2026-10-01"),
            StatTableError::Version {
                found: "2026-10-01".to_string(),
                expected: STAT_TABLE_VERSION,
            },
        ),
        (
            one().replace("source = \"s\"", "source = \" \""),
            StatTableError::NoSource,
        ),
        (HEAD.to_string(), StatTableError::NoStat),
        (
            one().replace("id = \"stat_1\"", "id = \"\""),
            StatTableError::BadId(String::new()),
        ),
        (
            one().replace("id = \"stat_1\"", "id = \"explicit.stat_1\""),
            StatTableError::BadId("explicit.stat_1".to_string()),
        ),
        (
            one().replace("id = \"stat_1\"", "id = \"stat 1\""),
            StatTableError::BadId("stat 1".to_string()),
        ),
        (
            second("id = \"stat_1\"\nin = [\"explicit\"]\ntexts = [\"another\"]"),
            StatTableError::DuplicateId(id()),
        ),
        (
            second("id = \"stat_2\"\nin = []\ntexts = [\"x\"]"),
            StatTableError::NoCategory {
                id: "stat_2".to_string(),
            },
        ),
        (
            second("id = \"stat_2\"\nin = [\"pseudo\"]\ntexts = [\"x\"]"),
            StatTableError::UnknownCategory {
                id: "stat_2".to_string(),
                category: "pseudo".to_string(),
            },
        ),
        (
            second("id = \"stat_2\"\nin = [\"crafted\", \"crafted\"]\ntexts = [\"x\"]"),
            StatTableError::DuplicateCategory {
                id: "stat_2".to_string(),
                category: "crafted".to_string(),
            },
        ),
        (
            second("id = \"stat_2\"\nin = [\"crafted\"]\ntexts = []"),
            StatTableError::NoText {
                id: "stat_2".to_string(),
            },
        ),
        (
            second("id = \"stat_2\"\nin = [\"crafted\"]\ntexts = [\"x\", \" \"]"),
            StatTableError::EmptyText {
                id: "stat_2".to_string(),
            },
        ),
        (
            second("id = \"stat_2\"\nin = [\"crafted\"]\ntexts = [\"x\", \"x\"]"),
            StatTableError::DuplicateText {
                id: "stat_2".to_string(),
                text: "x".to_string(),
            },
        ),
        (
            one().replace(", \"scourge\"", ""),
            StatTableError::EmptyCategory("scourge"),
        ),
    ] {
        assert_eq!(StatTable::parse(&bad).unwrap_err(), expected, "{bad}");
        // each refusal names what a reviewer fixes
        assert!(!expected.to_string().is_empty());
    }
    // an unknown key, a missing one, a text that is no string: the file
    // does not parse
    for bad in [
        one().replace("texts =", "text ="),
        one().replace("generated_by = \"t\"\n", ""),
        one().replace("[\"+# to maximum Life\"]", "[7]"),
        "version = 1".to_string(),
    ] {
        assert!(
            matches!(StatTable::parse(&bad), Err(StatTableError::Parse(_))),
            "{bad}"
        );
    }
}
