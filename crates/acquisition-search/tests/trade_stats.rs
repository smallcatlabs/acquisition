//! The stat table (C113, C106): the shipped file loads at the capture's
//! date, answers every entry of the capture it was generated from and
//! nothing the capture lacks, and the loader refuses each defect a reviewer
//! would, by the row it names. The counts are the generator's, measured at
//! generation and printed in the file's header; the table regenerating
//! byte-for-byte is `tools/trade-stats-table.py --check`.

use std::collections::BTreeMap;

use acquisition_search::trade::stats::{
    self, HASH_CATEGORIES, STAT_TABLE_VERSION, StatTable, StatTableError,
};
use serde_json::Value;

/// The capture the table is generated from, committed beside the track.
const CAPTURE: &str = include_str!("../../../search/trade-query/data/stats-2026-11-17.json");

/// C113: the shipped table loads, is the version a basis will cite, and
/// holds what the generator measured.
#[test]
fn c113_the_shipped_stat_table_is_the_capture_of_2026_11_17() {
    let t = stats::table().unwrap();
    assert_eq!(t.version(), STAT_TABLE_VERSION);
    assert_eq!(STAT_TABLE_VERSION, "2026-11-17");
    assert!(
        t.source().contains("stats-2026-11-17.json"),
        "{}",
        t.source()
    );
    assert_eq!(t.ids(), 10886);
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
    assert_eq!(entries, 14920);
    assert_eq!(other, 18647 - 14920);
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

const HEAD: &str = "version = \"2026-11-17\"\nsource = \"s\"\ngenerated_by = \"t\"\n";

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
            one().replace("2026-11-17", "2026-10-01"),
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
    // an id is what every id of the capture is, ASCII letters, digits, `_`
    // and `|`: each of these is refused by it, spelled in the file as TOML
    // spells it and named in the refusal as the loader read it
    for (spelled, read) in [
        ("", ""),
        ("explicit.stat_1", "explicit.stat_1"),
        ("stat 1", "stat 1"),
        ("stat_1\\t", "stat_1\t"),
        ("stat_1\\n", "stat_1\n"),
        ("\\t \\n", "\t \n"),
        ("stat\\u0001", "stat\u{1}"),
        ("stat\\u00a0_1", "stat\u{a0}_1"),
        ("stät_1", "stät_1"),
    ] {
        let bad = one().replace("id = \"stat_1\"", &format!("id = \"{spelled}\""));
        assert_eq!(
            StatTable::parse(&bad).unwrap_err(),
            StatTableError::BadId(read.to_string()),
            "{spelled}"
        );
    }
    assert!(stats::is_id("stat_4089743927|1|66"));
    assert!(stats::is_id("indexable_support_73"));
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

/// The picker's listing (`trade/vocabulary.rs`): the entries that hold
/// every word asked, case aside, in a text or in their group's name,
/// the totals table's pseudo group first and the stat table's after it
/// in its order, the first `limit` with what the translation makes of
/// each alone and with a bound, the rest counted; a `~` before the ask
/// changes nothing; an empty ask is every entry of both tables, and a
/// stat of two texts is listed under both and unsupported, as the
/// translation reports it.
#[test]
fn the_picker_lists_the_stat_vocabulary_with_each_fate() {
    use acquisition_search::trade::translate::Status;
    use acquisition_search::trade::vocabulary::{MAX_LISTED, vocabulary};
    let life = vocabulary("Maximum  LIFE", 2).unwrap();
    assert_eq!(life.stats, STAT_TABLE_VERSION);
    let listed: Vec<(&str, &str, Status, Status)> = life
        .entries
        .iter()
        .map(|e| (e.id.as_str(), e.group, e.alone, e.bounded))
        .collect();
    assert_eq!(
        listed,
        [
            (
                "pseudo.pseudo_total_life",
                "pseudo",
                Status::Exact,
                Status::Exact
            ),
            (
                "explicit.stat_3299347043",
                "explicit",
                Status::Exact,
                Status::Exact
            ),
        ]
    );
    assert_eq!(life.entries[0].texts, ["+# total maximum Life"]);
    assert_eq!(life.entries[1].texts, ["+# to maximum Life"]);
    assert_eq!(
        life.left_out, 141,
        "every category's maximum Life past the two"
    );
    // an empty ask is both tables whole, the limit capped: the stat
    // table's 14,700 and the totals table's 82 totals and 4 readings
    let every = vocabulary("", usize::MAX).unwrap();
    assert_eq!(every.entries.len(), MAX_LISTED);
    assert_eq!(every.entries.len() + every.left_out, 14_700 + 86);
    assert_eq!(
        every.totals,
        acquisition_search::totals::table().unwrap().version()
    );
    // the site's mark and a kind's word (the owner's sitting of
    // 2026-11-10): a `~` before the ask changes nothing, and a word that
    // stands in a group's name holds for that group's entries
    let ids = |ask: &str| -> Vec<String> {
        let listed = vocabulary(ask, 10).unwrap();
        listed.entries.into_iter().map(|e| e.id).collect()
    };
    assert_eq!(ids("~Maximum LIFE"), ids("Maximum LIFE"));
    let pseudo_life = ids("pseudo life");
    assert_eq!(
        pseudo_life,
        [
            "pseudo.pseudo_total_life",
            "pseudo.pseudo_total_life_regen",
            "pseudo.pseudo_percent_life_regen",
            "pseudo.pseudo_physical_attack_damage_leeched_as_life",
        ]
    );
    assert_eq!(ids("~pseudo life"), pseudo_life);
    // a piece of a word is a piece of any text: `pse` stands in
    // "pseudo" and in "corpses", so the four come first and the
    // explicit lines of corpses and Life after them
    let pieces = ids("~lif pse");
    assert_eq!(pieces[..4], pseudo_life[..]);
    assert!(pieces[4..].iter().all(|id| id.starts_with("explicit.")));
    assert_eq!(pieces, ids("lif pse"));
    // a bare word lists the pseudo group before the explicit lines
    let bare = vocabulary("life", 5).unwrap();
    let groups: Vec<&str> = bare.entries.iter().map(|e| e.group).collect();
    assert_eq!(groups, ["pseudo", "pseudo", "pseudo", "pseudo", "explicit"]);
    let fractured = vocabulary("frac life", 10).unwrap();
    assert_eq!(fractured.entries.len(), 10);
    assert!(fractured.entries.iter().all(|e| e.group == "fractured"));
    // a stat of two texts: either text finds it, and its fate is the report's
    let grove = vocabulary("Sacred Grove", 10).unwrap();
    let two = grove
        .entries
        .iter()
        .find(|e| e.id == "explicit.stat_1145451936")
        .unwrap();
    assert_eq!(two.texts.len(), 2);
    assert_eq!(
        (two.alone, two.bounded),
        (Status::Unsupported, Status::Unsupported)
    );
    // nothing held is nothing listed and nothing counted
    let none = vocabulary("zzzz qqqq", 10).unwrap();
    assert!(none.entries.is_empty() && none.left_out == 0);
    // a filled form's rows by their ids, in the order given: each entry as
    // a listing gives it, its fate the same, and an id no table holds none
    let by_id = acquisition_search::trade::vocabulary::entries(&[
        "explicit.stat_1145451936",
        "explicit.stat_9999999999",
        "pseudo.pseudo_total_life",
        "nonsense",
    ])
    .unwrap();
    let found: Vec<Option<(&str, &str, Status, Status)>> = by_id
        .iter()
        .map(|e| {
            e.as_ref()
                .map(|e| (e.id.as_str(), e.group, e.alone, e.bounded))
        })
        .collect();
    assert_eq!(
        found,
        [
            Some((
                "explicit.stat_1145451936",
                "explicit",
                two.alone,
                two.bounded
            )),
            None,
            Some((
                "pseudo.pseudo_total_life",
                "pseudo",
                Status::Exact,
                Status::Exact
            )),
            None,
        ]
    );
    assert_eq!(by_id[0].as_ref().unwrap().texts, two.texts);
    assert_eq!(by_id[2].as_ref().unwrap().texts, ["+# total maximum Life"]);
}

/// The value listing (`trade/vocabulary.rs`, `values`; the value finder,
/// `roadmap/briefs/4-full-1-form-values.md`): every number field the form
/// renders as a box and every stat of the vocabulary is put to the one
/// function, and the listing is exactly the entries it answers a value for,
/// in order, both ways; every value it offers is taken by the search as an
/// order, refused by none. Its limit lists the first and counts the rest.
#[test]
fn the_value_listing_is_every_entry_with_a_value_and_each_one_orders() {
    use acquisition_search::trade::catalogue::catalogue;
    use acquisition_search::trade::vocabulary::{Valued, value_of, values};
    use acquisition_search::{Request, totals};
    // every entry asked, as the form and the vocabulary name them
    let mut every: Vec<(String, bool)> = catalogue()
        .unwrap()
        .rows()
        .iter()
        .filter(|row| {
            row.text.is_some()
                && row
                    .shape
                    .as_deref()
                    .is_some_and(|s| s.starts_with("minmax") || s == "sockets")
        })
        .map(|row| (row.key.clone(), true))
        .collect();
    let totals = totals::table().unwrap();
    every.extend(
        totals
            .totals()
            .iter()
            .map(|t| (&t.site, &t.text))
            .chain(totals.readings().iter().map(|r| (&r.site, &r.text)))
            .filter(|(site, text)| site.is_some() && text.is_some())
            .map(|(site, _)| (format!("pseudo.{}", site.as_deref().unwrap()), false)),
    );
    every.extend(
        stats::table()
            .unwrap()
            .entries()
            .map(|(category, id, _)| (format!("{category}.{id}"), false)),
    );
    let valued: Vec<(String, String)> = every
        .iter()
        .filter_map(|(id, field)| {
            let entry = match field {
                true => Valued::Field(id),
                false => Valued::Stat(id),
            };
            Some((id.clone(), value_of(entry)?))
        })
        .collect();
    let listed = values("", usize::MAX).unwrap();
    assert_eq!(listed.left_out, 0);
    let ids: Vec<(String, String)> = listed
        .entries
        .iter()
        .map(|e| (e.id.clone(), e.value.clone()))
        .collect();
    assert!(
        ids == valued,
        "the listing is the entries with a value, in order"
    );
    assert!(
        listed.entries.iter().any(|e| e.group == "filter")
            && listed.entries.iter().any(|e| e.group == "pseudo")
            && listed.entries.iter().any(|e| e.group == "explicit"),
        "the fields, the totals and the stats"
    );
    // the limit: the first listed, the rest counted after the filter
    let fifty = values("", 50).unwrap();
    assert_eq!(fifty.entries, listed.entries[..50]);
    assert_eq!(fifty.left_out, listed.entries.len() - 50);
    // every value offered is taken as an order: read as a request's sort
    // and answered over an empty store, refused by none
    let corpus = crate::common::load(&crate::common::store(), None);
    let mut seen = std::collections::HashSet::new();
    for entry in &listed.entries {
        if !seen.insert(entry.value.as_str()) {
            continue;
        }
        let request: Request = serde_json::from_value(serde_json::json!({
            "query": { "text": "" }, "view": { "rows": { "sort": entry.value } }
        }))
        .unwrap();
        if let Err(e) = acquisition_search::answer(&corpus, &request) {
            panic!("{} orders by `{}`: {}", entry.id, entry.value, e.to_json());
        }
    }
}

/// The one function the listing and a line's value share: the value the
/// translation compares for a bounded filter on one entry, of any kind —
/// a field's, a pseudo total's computed value, a stat's sum — and none for
/// a stat with no number or an entry no table holds.
#[test]
fn an_entrys_value_is_what_the_translation_compares_for_it_bounded() {
    use acquisition_search::trade::vocabulary::{Valued, value_of, values};
    assert_eq!(
        value_of(Valued::Field("misc_filters.ilvl")).as_deref(),
        Some("ilvl")
    );
    // the sitting's total elemental resistance, by the site's own words
    assert_eq!(
        value_of(Valued::Stat("pseudo.pseudo_total_elemental_resistance")).as_deref(),
        Some("pseudo.total_ele_res")
    );
    assert_eq!(
        value_of(Valued::Stat("explicit.stat_3299347043")).as_deref(),
        Some("sum(line(\"# to maximum Life\" source=explicit).arg1)")
    );
    // a stat with none: "Cannot be Frozen" holds no number
    assert_eq!(value_of(Valued::Stat("explicit.stat_876831634")), None);
    assert_eq!(value_of(Valued::Field("misc_filters.no_such")), None);
    assert_eq!(value_of(Valued::Field("socket_filters.links")), None);
    // found by the site's words, the field's label and the stat's text
    let ilvl = values("item level", 5).unwrap();
    assert_eq!(ilvl.entries[0].id, "misc_filters.ilvl");
    assert_eq!(ilvl.entries[0].texts, ["Item Level"]);
    assert_eq!(ilvl.entries[0].group, "filter");
    let frozen = values("~cannot be frozen", 50).unwrap();
    assert!(
        frozen
            .entries
            .iter()
            .all(|e| e.id != "explicit.stat_876831634")
    );
}
