//! The property fields (C114, C101, C93; step 3 of
//! `search/TRADE-PLAN.md` at `77666ea4`), through the crate's boundary: the shipped
//! table and what its loader refuses; every field asked every way a
//! number is — a comparison, `has:` and its not, `undecided( … )`, the
//! sort, a count's sum and a count's key — on an item that displays its
//! property, one that lacks it and one that displays it as no number;
//! the q20 rule's arithmetic; `map_tier`; `pseudo.edps`; and the q20
//! rule against the trade site's own figures, every captured row of it.
//! Every count and value is worked by hand from the spec beside it.

use crate::common;

use acquisition_search::properties::{self, PropertyTable, PropertyTableError};
use acquisition_search::{PROPERTY_TABLE_VERSION, Request, answer, describe};
use acquisition_store::Store;
use common::*;
use serde_json::{Value, json};

// ---- the table -------------------------------------------------------------------------------

/// C114: the shipped table loads at the version a basis cites, holds the
/// 39 fields the catalogue names over a property or the base and the four
/// displayed defences beside their `.q20` twins — every one
/// of them printed by `--describe fields` with its definition and the
/// table's version — and a basis cites it.
#[test]
fn c114_the_shipped_property_table_is_every_field_the_catalogue_names() {
    let t = properties::table().unwrap();
    assert_eq!(t.version(), PROPERTY_TABLE_VERSION);
    assert_eq!(PROPERTY_TABLE_VERSION, 1);
    let names: Vec<&str> = t.fields().iter().map(|f| f.name()).collect();
    assert_eq!(names.len(), 43);
    assert_eq!(
        PropertyTable::parse(properties::PROPERTY_TABLE_TOML)
            .unwrap()
            .fields()
            .len(),
        43
    );
    let d = serde_json::to_value(describe(&["fields".to_string()]).unwrap()).unwrap();
    for name in &names {
        let entry = d["fields"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["name"] == *name)
            .unwrap_or_else(|| panic!("--describe fields lacks {name}"));
        assert_eq!(entry["kind"], "number", "{name}");
        let what = entry["what"].as_str().unwrap();
        assert!(what.ends_with("(properties v1)"), "{name}: {what}");
        assert_eq!(
            entry["examples"],
            json!([format!("{name}>=1"), format!("-has:{name}")]),
            "{name}"
        );
    }
    // a name answers to itself, and a dotted one to its head
    let one = serde_json::to_value(describe(&["armour.q20".to_string()]).unwrap()).unwrap();
    assert_eq!(one["fields"][0]["name"], "armour.q20");
    // every basis cites the table
    let a = as_json(&ask(&load(&specs_store(&SPECS[0]), Some("pc")), "").unwrap());
    assert_eq!(a["basis"]["properties"], 1);
}

const HEAD: &str = "version = 1\nsource = \"s\"\n";

fn row(name: &str, body: &str) -> String {
    format!("[[field]]\nname = \"{name}\"\n{body}\ndefinition = \"d\"\n")
}

/// The rows the search's own values read by name, which every table
/// carries: the quality, and the attack speed.
const READ: &str = concat!(
    "[[field]]\nname = \"quality\"\narray = \"properties\"\nproperty = [\"Quality\"]\nreads = \"number\"\nwhole = true\ndefinition = \"d\"\n",
    "[[field]]\nname = \"attacks_per_second\"\narray = \"properties\"\nproperty = [\"Attacks per Second\"]\nreads = \"number\"\ndefinition = \"d\"\n",
);

/// What the loader refuses, each by the row it names.
#[test]
fn c114_the_property_table_loader_refuses_what_a_reviewer_would() {
    let refused = |text: String| PropertyTable::parse(&text).unwrap_err();
    let field = |name: &str| name.to_string();
    let plain = "array = \"properties\"\nproperty = [\"Armour\"]\nreads = \"number\"";
    // the shape of a good table, to break one part at a time
    assert!(PropertyTable::parse(&format!("{HEAD}{READ}{}", row("a", plain))).is_ok());
    assert_eq!(
        refused(format!("version = 2\nsource = \"s\"\n{}", row("a", plain))),
        PropertyTableError::Version {
            found: 2,
            expected: 1
        }
    );
    assert_eq!(
        refused(format!("version = 1\nsource = \" \"\n{}", row("a", plain))),
        PropertyTableError::NoSource
    );
    assert_eq!(refused(HEAD.to_string()), PropertyTableError::NoField);
    for bad in ["", "Armour", "a b", "1a", "a.", ".a", "a..b", "a-b", "é"] {
        assert_eq!(
            refused(format!("{HEAD}{}", row(bad, plain))),
            PropertyTableError::BadName(field(bad)),
            "{bad:?}"
        );
    }
    // a word the parser reads as something else is no field name
    for reserved in ["or", "not", "has", "is"] {
        assert_eq!(
            refused(format!("{HEAD}{}", row(reserved, plain))),
            PropertyTableError::BadName(field(reserved)),
            "{reserved}"
        );
    }
    // a field the language has, and a word it reads in a field's place:
    // `line` is the vocabulary's key
    for taken in ["ilvl", "sockets.red", "stack", "line", "linked", "sum"] {
        assert_eq!(
            refused(format!("{HEAD}{}", row(taken, plain))),
            PropertyTableError::Taken(field(taken)),
            "{taken}"
        );
    }
    assert_eq!(
        refused(format!(
            "{HEAD}{}{}",
            row("a", plain),
            row(
                "a",
                "array = \"properties\"\nproperty = [\"Ward\"]\nreads = \"number\""
            )
        )),
        PropertyTableError::DuplicateName(field("a"))
    );
    assert_eq!(
        refused(format!(
            "{HEAD}{}",
            row(
                "a",
                "array = \"properties\"\nproperty = [\"Armour\"]\nreads = \"sum\""
            )
        )),
        PropertyTableError::UnknownReads {
            field: field("a"),
            reads: "sum".to_string()
        }
    );
    let shape = |body: &str, what: &'static str| {
        assert_eq!(
            refused(format!("{HEAD}{}", row("a", body))),
            PropertyTableError::Shape {
                field: field("a"),
                what
            },
            "{body}"
        );
    };
    shape(
        "reads = \"tier\"\narray = \"properties\"",
        "reads the base and names a property",
    );
    shape(
        "reads = \"tier\"\nproperty = [\"Map Tier\"]",
        "reads the base and names a property",
    );
    shape(
        "reads = \"number\"\nproperty = [\"Armour\"]",
        "names no array",
    );
    shape(
        "reads = \"number\"\narray = \"properties\"",
        "names no property",
    );
    shape(
        "reads = \"number\"\narray = \"properties\"\nproperty = [\" \"]",
        "names an empty property",
    );
    shape(
        "reads = \"numerator\"\narray = \"properties\"\nproperty = [\"Wings Revealed\"]\nplus = true",
        "names a `+`, a whole number or a mark, which only `number` reads",
    );
    shape(
        "reads = \"percent\"\narray = \"properties\"\nproperty = [\"Experience\"]\nunit = \"%\"",
        "names a unit, which only `number` reads",
    );
    shape(&format!("{plain}\nunit = \"\""), "names an empty unit");
    shape(&format!("{plain}\nmarks = [\"\"]"), "names an empty mark");
    shape(
        &format!("{plain}\nselect = {{ value = 1, is = \" \" }}"),
        "selects by an empty word",
    );
    shape(
        &format!("{plain}\nvalue = 1\nselect = {{ value = 1, is = \"Agility\" }}"),
        "selects by the value it reads",
    );
    assert_eq!(
        refused(format!(
            "{HEAD}{}",
            row(
                "a",
                "array = \"skills\"\nproperty = [\"Armour\"]\nreads = \"number\""
            )
        )),
        PropertyTableError::UnknownArray {
            field: field("a"),
            array: "skills".to_string()
        }
    );
    assert_eq!(
        refused(format!(
            "{HEAD}{}",
            row(
                "a",
                "array = \"requirements\"\nproperty = [\"Str\", \"str\"]\nreads = \"number\""
            )
        )),
        PropertyTableError::DuplicateProperty {
            field: field("a"),
            property: "str".to_string()
        }
    );
    assert_eq!(
        refused(format!(
            "{HEAD}{}{}",
            row("a", plain),
            row(
                "b",
                "array = \"properties\"\nproperty = [\"armour\"]\nreads = \"number\""
            )
        )),
        PropertyTableError::SameReading {
            field: field("b"),
            other: field("a")
        }
    );
    // the rows the search's own values read by name: the quality, read
    // by the q20 rows and the percentile, and the attack speed, read by
    // the damage fields; each a number of its property
    let quality = "array = \"properties\"\nproperty = [\"Quality\"]\nreads = \"number\"";
    let speed = "array = \"properties\"\nproperty = [\"Attacks per Second\"]\nreads = \"number\"";
    let no_row =
        |row: &'static str, property: &'static str| PropertyTableError::NoRow { row, property };
    assert_eq!(
        refused(format!("{HEAD}{}", row("a", plain))),
        no_row("quality", "Quality")
    );
    assert_eq!(
        refused(format!(
            "{HEAD}{}{}",
            row("quality", quality),
            row("a", plain)
        )),
        no_row("attacks_per_second", "Attacks per Second")
    );
    assert_eq!(
        refused(format!(
            "{HEAD}{}{}",
            row(
                "quality",
                "array = \"properties\"\nproperty = [\"Armour\"]\nreads = \"number\""
            ),
            row("attacks_per_second", speed)
        )),
        no_row("quality", "Quality")
    );
    assert_eq!(
        refused(format!(
            "{HEAD}{}{}",
            row("quality", quality),
            row("attacks_per_second", &speed.replace("number", "q20"))
        )),
        no_row("attacks_per_second", "Attacks per Second")
    );
    assert_eq!(
        refused(format!(
            "{HEAD}[[field]]\nname = \"a\"\n{plain}\ndefinition = \" \"\n"
        )),
        PropertyTableError::NoDefinition(field("a"))
    );
    for broken in [
        "version = 1\n",
        "version = \"1\"\nsource = \"s\"\n",
        "version = 1\nsource = \"s\"\n[[field]]\nname = \"a\"\n",
        &format!("{HEAD}{}", row("a", &format!("{plain}\nextra = 1"))),
    ] {
        assert!(
            matches!(refused(broken.to_string()), PropertyTableError::Parse(_)),
            "{broken}"
        );
    }
}

// ---- every field, every way -----------------------------------------------------------------

/// A property as GGG gives it: its name, its values, how it is displayed.
fn prop(name: &str, values: &[&str], mode: i64) -> Value {
    json!({
        "name": name,
        "values": values.iter().map(|v| json!([v, 0])).collect::<Vec<_>>(),
        "displayMode": mode,
    })
}

/// One field asked of three items: `has`, whose body adds what `shows`
/// gives and whose field is `value`, worked by hand; `odd`, whose body
/// adds what `opens` gives, which leaves the field unread; and `plain`, a
/// ring that displays nothing, which lacks every field.
struct Spec {
    field: &'static str,
    shows: fn() -> Value,
    value: f64,
    opens: fn() -> Value,
}

fn props(list: Vec<Value>) -> Value {
    json!({ "properties": list })
}

fn reqs(list: Vec<Value>) -> Value {
    json!({ "requirements": list })
}

fn heist(level: &'static str, skill: &'static str) -> Value {
    props(vec![prop("Requires {1} (Level {0})", &[level, skill], 3)])
}

const SPECS: &[Spec] = &[
    Spec {
        field: "attacks_per_second",
        shows: || props(vec![prop("Attacks per Second", &["1.20"], 0)]),
        value: 1.2,
        opens: || props(vec![prop("Attacks per Second", &["fast"], 0)]),
    },
    Spec {
        field: "critical_strike_chance",
        shows: || props(vec![prop("Critical Strike Chance", &["7.50%"], 0)]),
        value: 7.5,
        opens: || props(vec![prop("Critical Strike Chance", &["7.50%%"], 0)]),
    },
    Spec {
        field: "chance_to_block",
        shows: || props(vec![prop("Chance to Block", &["24%"], 0)]),
        value: 24.0,
        // a whole number's row: a decimal is displayed as no whole number
        opens: || props(vec![prop("Chance to Block", &["24.5%"], 0)]),
    },
    // 383 at 10 %: 383 × 120 ÷ 110 = 417.8…, 418
    Spec {
        field: "armour.q20",
        shows: || {
            props(vec![
                prop("Quality", &["+10%"], 0),
                prop("Armour", &["383"], 0),
            ])
        },
        value: 418.0,
        // the quality it reads could not be
        opens: || {
            props(vec![
                prop("Quality", &["+ten%"], 0),
                prop("Armour", &["383"], 0),
            ])
        },
    },
    // no quality displayed: 100 × 120 ÷ 100
    Spec {
        field: "evasion_rating.q20",
        shows: || props(vec![prop("Evasion Rating", &["100"], 0)]),
        value: 120.0,
        opens: || props(vec![prop("Evasion Rating", &["1e3"], 0)]),
    },
    // at 28 % the display stands
    Spec {
        field: "energy_shield.q20",
        shows: || {
            props(vec![
                prop("Quality", &["+28%"], 0),
                prop("Energy Shield", &["146"], 0),
            ])
        },
        value: 146.0,
        opens: || {
            props(vec![
                prop("Energy Shield", &["146"], 0),
                prop("Energy Shield", &["146"], 0),
            ])
        },
    },
    // under the enchant the display stands
    Spec {
        field: "ward.q20",
        shows: || {
            json!({
                "properties": [prop("Quality", &["+5%"], 0), prop("Ward", &["100"], 0)],
                "enchantMods": ["Quality does not increase Defences"],
            })
        },
        value: 100.0,
        // below 20 % with an array of lines unread, which may hold it
        opens: || {
            json!({
                "properties": [prop("Quality", &["+5%"], 0), prop("Ward", &["100"], 0)],
                "enchantMods": "no array",
            })
        },
    },
    // the displayed defences, as the item stands: the display itself,
    // whatever the quality (the 4b sitting round)
    Spec {
        field: "armour",
        shows: || {
            props(vec![
                prop("Quality", &["+10%"], 0),
                prop("Armour", &["383"], 0),
            ])
        },
        value: 383.0,
        opens: || props(vec![prop("Armour", &["383.5"], 0)]),
    },
    Spec {
        field: "evasion_rating",
        shows: || props(vec![prop("Evasion Rating", &["100"], 0)]),
        value: 100.0,
        opens: || props(vec![prop("Evasion Rating", &["1e3"], 0)]),
    },
    Spec {
        field: "energy_shield",
        shows: || {
            props(vec![
                prop("Quality", &["+28%"], 0),
                prop("Energy Shield", &["146"], 0),
            ])
        },
        value: 146.0,
        opens: || {
            props(vec![
                prop("Energy Shield", &["146"], 0),
                prop("Energy Shield", &["146"], 0),
            ])
        },
    },
    Spec {
        field: "ward",
        shows: || props(vec![prop("Ward", &["100"], 0)]),
        value: 100.0,
        opens: || props(vec![prop("Ward", &["one hundred"], 0)]),
    },
    Spec {
        field: "str",
        shows: || reqs(vec![prop("Str", &["155"], 1)]),
        value: 155.0,
        // displayed under both its names: more than once
        opens: || {
            reqs(vec![
                prop("Str", &["155"], 1),
                prop("Strength", &["155"], 1),
            ])
        },
    },
    Spec {
        field: "dex",
        shows: || reqs(vec![prop("Dexterity", &["120"], 1)]),
        value: 120.0,
        opens: || reqs(vec![prop("Dex", &["12a"], 1)]),
    },
    Spec {
        field: "int",
        shows: || reqs(vec![prop("Int", &["14"], 1)]),
        value: 14.0,
        // an element named as the field's and not read
        opens: || {
            reqs(vec![
                json!({ "name": "Int", "values": "14", "displayMode": 1 }),
            ])
        },
    },
    Spec {
        field: "map_tier",
        shows: || json!({ "baseType": "Blighted Map (Tier 13)" }),
        value: 13.0,
        opens: || json!({ "baseType": 13 }),
    },
    Spec {
        field: "area_level",
        shows: || props(vec![prop("Area Level", &["83"], 0)]),
        value: 83.0,
        opens: || props(vec![prop("Area Level", &["eighty"], 0)]),
    },
    Spec {
        field: "monster_pack_size",
        shows: || props(vec![prop("Monster Pack Size", &["+23%"], 0)]),
        value: 23.0,
        opens: || props(vec![prop("Monster Pack Size", &["-23%"], 0)]),
    },
    Spec {
        field: "item_quantity",
        shows: || props(vec![prop("Item Quantity", &["+61%"], 0)]),
        value: 61.0,
        opens: || props(vec![prop("Item Quantity", &["++61%"], 0)]),
    },
    Spec {
        field: "item_rarity",
        shows: || props(vec![prop("Item Rarity", &["+36%"], 0)]),
        value: 36.0,
        opens: || props(vec![prop("Item Rarity", &["+36", "+1"], 0)]),
    },
    Spec {
        field: "memory_strands",
        shows: || props(vec![prop("Memory Strands", &["30"], 0)]),
        value: 30.0,
        opens: || props(vec![prop("Memory Strands", &[""], 0)]),
    },
    Spec {
        field: "wings_revealed",
        shows: || props(vec![prop("Wings Revealed", &["1/3"], 0)]),
        value: 1.0,
        opens: || props(vec![prop("Wings Revealed", &["1/x"], 0)]),
    },
    Spec {
        field: "wings_revealed.of",
        shows: || props(vec![prop("Wings Revealed", &["1/3"], 0)]),
        value: 3.0,
        opens: || props(vec![prop("Wings Revealed", &["3"], 0)]),
    },
    Spec {
        field: "escape_routes_revealed",
        shows: || props(vec![prop("Escape Routes Revealed", &["7/8"], 0)]),
        value: 7.0,
        opens: || props(vec![prop("Escape Routes Revealed", &["7/8/9"], 0)]),
    },
    Spec {
        field: "escape_routes_revealed.of",
        shows: || props(vec![prop("Escape Routes Revealed", &["7/8"], 0)]),
        value: 8.0,
        opens: || props(vec![prop("Escape Routes Revealed", &["7/-8"], 0)]),
    },
    Spec {
        field: "reward_rooms_revealed",
        shows: || props(vec![prop("Reward Rooms Revealed", &["17/28"], 0)]),
        value: 17.0,
        opens: || props(vec![prop("Reward Rooms Revealed", &["1.5/28"], 0)]),
    },
    Spec {
        field: "reward_rooms_revealed.of",
        shows: || props(vec![prop("Reward Rooms Revealed", &["17/28"], 0)]),
        value: 28.0,
        opens: || props(vec![prop("Reward Rooms Revealed", &["17/"], 0)]),
    },
    Spec {
        field: "lockpicking",
        shows: || heist("3", "Lockpicking"),
        value: 3.0,
        opens: || heist("x", "Lockpicking"),
    },
    Spec {
        field: "brute_force",
        shows: || heist("4", "Brute Force"),
        value: 4.0,
        opens: || heist("4.0", "Brute Force"),
    },
    Spec {
        field: "perception",
        shows: || heist("5", "Perception"),
        value: 5.0,
        // the value that says whose it is, missing: it may be this one
        opens: || props(vec![prop("Requires {1} (Level {0})", &["5"], 3)]),
    },
    Spec {
        field: "demolition",
        shows: || heist("1", "Demolition"),
        value: 1.0,
        opens: || {
            props(vec![
                prop("Requires {1} (Level {0})", &["1", "Demolition"], 3),
                prop("Requires {1} (Level {0})", &["2", "Demolition"], 3),
            ])
        },
    },
    Spec {
        field: "counter_thaumaturgy",
        shows: || heist("2", "Counter-Thaumaturgy"),
        value: 2.0,
        opens: || heist("two", "Counter-Thaumaturgy"),
    },
    Spec {
        field: "trap_disarmament",
        shows: || heist("6", "Trap Disarmament"),
        value: 6.0,
        opens: || heist("", "Trap Disarmament"),
    },
    Spec {
        field: "agility",
        shows: || heist("7", "Agility"),
        value: 7.0,
        opens: || heist("+7", "Agility"),
    },
    Spec {
        field: "deception",
        shows: || heist("8", "Deception"),
        value: 8.0,
        opens: || heist("8%", "Deception"),
    },
    Spec {
        field: "engineering",
        shows: || heist("9", "Engineering"),
        value: 9.0,
        // an element of its name that could not be read may be any skill's
        opens: || {
            props(vec![
                json!({ "name": "Requires {1} (Level {0})", "values": [["9"]], "displayMode": 3 }),
            ])
        },
    },
    Spec {
        field: "resolve",
        shows: || props(vec![prop("Resolve: {0}/{1}", &["225", "300"], 3)]),
        value: 225.0,
        opens: || props(vec![prop("Resolve: {0}/{1}", &["a", "300"], 3)]),
    },
    Spec {
        field: "resolve.of",
        shows: || props(vec![prop("Resolve: {0}/{1}", &["225", "300"], 3)]),
        value: 300.0,
        // no second value to read
        opens: || props(vec![prop("Resolve: {0}/{1}", &["225"], 3)]),
    },
    // a value of nothing is a value, and the item has it
    Spec {
        field: "inspiration",
        shows: || props(vec![prop("Inspiration: {0}", &["0"], 3)]),
        value: 0.0,
        opens: || props(vec![prop("Inspiration: {0}", &["none"], 3)]),
    },
    Spec {
        field: "aureus",
        shows: || props(vec![prop("Aureus: {0}", &["400"], 3)]),
        value: 400.0,
        opens: || props(vec![prop("Aureus: {0}", &["4,000"], 3)]),
    },
    Spec {
        field: "quality",
        shows: || props(vec![prop("Quality", &["+20%"], 0)]),
        value: 20.0,
        opens: || props(vec![prop("Quality", &["+20.5%"], 0)]),
    },
    Spec {
        field: "level",
        shows: || props(vec![prop("Level", &["20 (Max)"], 0)]),
        value: 20.0,
        opens: || props(vec![prop("Level", &["20 (Min)"], 0)]),
    },
    // 1 of 3: 33.333…, cut to five decimals
    Spec {
        field: "experience",
        shows: || json!({ "additionalProperties": [prop("Experience", &["1/3"], 2)] }),
        value: 33.33333,
        opens: || json!({ "additionalProperties": [prop("Experience", &["1/0"], 2)] }),
    },
    Spec {
        field: "stored_experience",
        shows: || props(vec![prop("Stored Experience: {0}", &["350000000"], 3)]),
        value: 350_000_000.0,
        opens: || props(vec![prop("Stored Experience: {0}", &["3.5e8"], 3)]),
    },
];

/// The three items of one spec, in one tab.
fn specs_store(spec: &Spec) -> Store {
    let mut s = store();
    list_tabs(&mut s, "pc", "Standard", json!([tab("p1", "Props")]), 10);
    let rare =
        |id: &str, base: &str, more: Value| item(id, &format!("Item {id}"), base, "Rare", more);
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "p1",
        "Props",
        vec![
            rare("has", "Base", (spec.shows)()),
            rare("odd", "Base", (spec.opens)()),
            rare("plain", "Iron Ring", json!({})),
        ],
        20,
    );
    s
}

/// An answer to a query, under a view; `{}` is the default view.
fn view(s: &Store, text: &str, view: Value) -> Value {
    let mut request = json!({ "query": { "text": text } });
    if view != json!({}) {
        request["view"] = view.clone();
    }
    let request: Request = serde_json::from_value(request).unwrap();
    match answer(&load(s, Some("pc")), &request) {
        Ok(a) => as_json(&a),
        Err(e) => panic!("`{text}` {view}: {e}"),
    }
}

fn counts(a: &Value) -> [u64; 4] {
    let term = &a["terms"][0];
    ["matched", "failed", "lacked", "undecided"].map(|k| term[k]["count"].as_u64().unwrap())
}

/// A number as the answer prints it: whole where it is.
fn number(n: f64) -> Value {
    match acquisition_search::Number::from_f64(n) {
        acquisition_search::Number::Int(i) => json!(i),
        acquisition_search::Number::Float(f) => json!(f),
    }
}

/// C114, C93: each field on its three items, asked every way a number is
/// — the value the spec worked by hand on `has`, nothing on `plain`, and
/// `odd` left open with a reason — each count routed where the term's is.
#[test]
fn c114_every_field_is_asked_every_way_a_number_is() {
    assert_eq!(SPECS.len(), 43);
    let table: Vec<&str> = properties::table()
        .unwrap()
        .fields()
        .iter()
        .map(|f| f.name())
        .collect();
    assert_eq!(
        SPECS.iter().map(|s| s.field).collect::<Vec<_>>(),
        table,
        "a spec for every row, in the table's order"
    );
    for spec in SPECS {
        let f = spec.field;
        let v = number(spec.value);
        let s = specs_store(spec);
        let at = |op: &str| format!("{f}{op}{v}");
        // a comparison: matched on `has`, failed where it does not hold,
        // lacked on `plain`, open on `odd`
        let a = view(&s, &at(">="), json!({}));
        assert_eq!(counts(&a), [1, 0, 1, 1], "{}", at(">="));
        assert_eq!(ids(&a), ["has"], "{}", at(">="));
        // the row shows the field's value, then the property it read
        let shows = &a["rows"][0]["matched"][0]["shows"];
        assert_eq!(
            shows[0],
            json!({ "value": { "name": f, "value": v } }),
            "{f}: {shows}"
        );
        assert_eq!(counts(&view(&s, &at(">"), json!({}))), [0, 1, 1, 1], "{f}");
        assert_eq!(counts(&view(&s, &at("="), json!({}))), [1, 0, 1, 1], "{f}");
        assert_eq!(counts(&view(&s, &at("<"), json!({}))), [0, 1, 1, 1], "{f}");
        // presence, and its not
        let has = view(&s, &format!("has:{f}"), json!({}));
        assert_eq!(counts(&has), [1, 0, 1, 1], "has:{f}");
        assert_eq!(
            ids(&view(&s, &format!("-has:{f}"), json!({}))),
            ["plain"],
            "{f}"
        );
        // undecided: `odd` alone, with the reason the reader gave
        let open = view(&s, &format!("undecided({f})"), json!({}));
        assert_eq!(ids(&open), ["odd"], "undecided({f})");
        // the sort: `has` by its value, `odd` incomplete, `plain` last
        let sorted = view(&s, "", json!({ "rows": { "sort": f, "desc": true } }));
        let rows: Vec<(&str, Value)> = sorted["rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| (r["id"].as_str().unwrap(), scalar(&r["sort"])))
            .collect();
        assert_eq!(rows[0], ("has", json!({ "value": v })), "{f}");
        assert!(
            rows[1..].contains(&("odd", json!({ "status": "incomplete" }))),
            "{f}: {rows:?}"
        );
        // a count's sum: `has`'s value, `plain` lacking, `odd` unread
        let summed = view(
            &s,
            "",
            json!({ "counts": { "keys": ["rarity"], "sum": f } }),
        );
        assert_eq!(
            summed["view"]["counts"]["sum"],
            json!({ "name": f, "value": v, "lacking": 1, "incomplete": true, "unread": 1 }),
            "{f}"
        );
        // a count's key: one bucket each, the value's, none and undecided
        let keyed = view(&s, "", json!({ "counts": { "keys": [f] } }));
        let mut buckets: Vec<(String, u64)> = keyed["view"]["counts"]["tables"][0]["buckets"]
            .as_array()
            .unwrap()
            .iter()
            .map(|b| {
                let label = match &b["value"] {
                    Value::Null => b["bucket"].as_str().unwrap().to_string(),
                    value => value.to_string().trim_matches('"').to_string(),
                };
                (label, b["count"].as_u64().unwrap())
            })
            .collect();
        buckets.sort();
        let mut expected = vec![
            (v.to_string(), 1),
            ("none".to_string(), 1),
            ("undecided".to_string(), 1),
        ];
        expected.sort();
        assert_eq!(buckets, expected, "--count {f}");
    }
}

// ---- the q20 rule, map_tier, edps ---------------------------------------------------------------

/// The displayed defences beside the site's reading (the 4b sitting
/// round): at 0 %, 12 % and 20 % quality the displayed field is the
/// display itself, and the `.q20` twin is the display moved to 20 %.
#[test]
fn the_displayed_defences_stand_beside_their_q20_twins() {
    for (quality, display, q20) in [
        (None, 300, 360),
        (Some("+12%"), 336, 360),
        (Some("+20%"), 360, 360),
    ] {
        let mut list = vec![prop("Armour", &[&display.to_string()], 0)];
        if let Some(q) = quality {
            list.insert(0, prop("Quality", &[q], 0));
        }
        let body = props(list);
        assert_eq!(
            sort_of("armour", body.clone()),
            json!({ "value": display }),
            "{quality:?}"
        );
        assert_eq!(
            sort_of("armour.q20", body),
            json!({ "value": q20 }),
            "{quality:?}"
        );
    }
}

/// One armour item's `armour.q20`, as the sort reads it.
fn q20_of(more: Value) -> Value {
    sort_of("armour.q20", more)
}

/// One item's value of a field, as the sort reads it.
fn sort_of(field: &str, more: Value) -> Value {
    let mut s = store();
    list_tabs(&mut s, "pc", "Standard", json!([tab("a1", "A")]), 10);
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "a1",
        "A",
        vec![item("x", "X", "Base", "Rare", more)],
        20,
    );
    let a = view(&s, "", json!({ "rows": { "sort": field } }));
    scalar(&a["rows"][0]["sort"])
}

/// S26, the q20 rule's arithmetic, worked by hand: the display × 120 ÷
/// (100 + the quality), rounded half up from the fraction; an exact half
/// rounds up; at 20 % exactly the display; the enchant anywhere, a row of
/// a mod displayed over several among them; and what opens it.
#[test]
fn s26_the_q20_rule_rounds_a_fraction_half_up() {
    let armour = |display: &str, quality: Option<&str>| {
        let mut list = vec![prop("Armour", &[display], 0)];
        if let Some(q) = quality {
            list.insert(0, prop("Quality", &[q], 0));
        }
        list
    };
    // 7 at 12 %: 7 × 120 ÷ 112 = 7.5 exactly, up to 8
    assert_eq!(
        q20_of(props(armour("7", Some("+12%")))),
        json!({ "value": 8 })
    );
    // 100 at 5 %: 114.28…, 114; 105 at nothing: 126
    assert_eq!(
        q20_of(props(armour("100", Some("+5%")))),
        json!({ "value": 114 })
    );
    assert_eq!(q20_of(props(armour("105", None))), json!({ "value": 126 }));
    // 0 % displayed is no quality: the same
    assert_eq!(
        q20_of(props(armour("105", Some("+0%")))),
        json!({ "value": 126 })
    );
    // at 20 % and over, the display
    assert_eq!(
        q20_of(props(armour("300", Some("+20%")))),
        json!({ "value": 300 })
    );
    assert_eq!(
        q20_of(props(armour("300", Some("+23%")))),
        json!({ "value": 300 })
    );
    // the `+` and the `%` each read where written
    assert_eq!(
        q20_of(props(armour("100", Some("5")))),
        json!({ "value": 114 })
    );
    // the enchant in any array, and as a row of a mod of several rows
    let enchanted = |array: &str, line: &str| {
        let mut more = props(armour("100", Some("+5%")));
        more[array] = json!([line]);
        q20_of(more)
    };
    assert_eq!(
        enchanted("enchantMods", "Quality does not increase Defences"),
        json!({ "value": 100 })
    );
    assert_eq!(
        enchanted("explicitMods", "Quality does not increase Defences"),
        json!({ "value": 100 })
    );
    assert_eq!(
        enchanted(
            "explicitMods",
            "+10 to maximum Life\nQuality does not increase Defences"
        ),
        json!({ "value": 100 })
    );
    // another line of its words is no enchant
    assert_eq!(
        enchanted("enchantMods", "Quality does not increase Damage"),
        json!({ "value": 114 })
    );
    // a lines array unread opens it below 20 %, and not at 20 %
    let mut unread = props(armour("100", Some("+5%")));
    unread["implicitMods"] = json!("no array");
    assert_eq!(q20_of(unread), json!({ "status": "incomplete" }));
    let mut unread = props(armour("100", Some("+20%")));
    unread["implicitMods"] = json!("no array");
    assert_eq!(q20_of(unread), json!({ "value": 100 }));
    // a quality displayed twice, or as no whole number, opens it
    let mut twice = armour("100", Some("+5%"));
    twice.insert(0, prop("Quality", &["+5%"], 0));
    assert_eq!(q20_of(props(twice)), json!({ "status": "incomplete" }));
    // an item displaying no armour lacks it, whatever its quality
    assert_eq!(
        q20_of(props(vec![prop("Quality", &["+bad"], 0)])),
        json!({ "status": "no satisfying occurrence" })
    );
}

/// C101, C114: `map_tier` is the number a base's ` (Tier N)` carries, a
/// unique map's whose base carries one among them, and lacked on a base
/// with none; `pseudo.edps` is the attacks per second times the mean of
/// `Elemental Damage`, each range's mean summed.
#[test]
fn c114_the_tier_is_the_bases_and_edps_the_elemental_ranges() {
    let mut s = store();
    list_tabs(&mut s, "pc", "Standard", json!([tab("m1", "Maps")]), 10);
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "m1",
        "Maps",
        vec![
            item("t16", "", "Map (Tier 16)", "Normal", json!({})),
            item(
                "blight",
                "",
                "Blight-ravaged Map (Tier 3)",
                "Rare",
                json!({}),
            ),
            item(
                "unique",
                "Maelström of Chaos",
                "Map (Tier 16)",
                "Unique",
                json!({}),
            ),
            item("other", "", "Shaper Guardian Map", "Normal", json!({})),
            item("not", "", "Map (Tier x)", "Normal", json!({})),
            // 38-71 and 57-108 at 1.25: 1.25 × (54.5 + 82.5) = 171.25
            item(
                "sword",
                "Edge",
                "Rusted Sword",
                "Rare",
                props(vec![
                    prop("Physical Damage", &["59-88"], 0),
                    prop("Elemental Damage", &["38-71", "57-108"], 0),
                    prop("Attacks per Second", &["1.25"], 0),
                ]),
            ),
            // physical alone: no edps
            item(
                "club",
                "Club",
                "Driftwood Club",
                "Rare",
                props(vec![
                    prop("Physical Damage", &["5-10"], 0),
                    prop("Attacks per Second", &["1.4"], 0),
                ]),
            ),
        ],
        20,
    );
    assert_eq!(ids(&view(&s, "map_tier=16", json!({}))), ["t16", "unique"]);
    assert_eq!(ids(&view(&s, "map_tier<16", json!({}))), ["blight"]);
    let a = view(&s, "has:map_tier", json!({}));
    assert_eq!(counts(&a), [3, 0, 4, 0]);
    let a = view(&s, "pseudo.edps>0", json!({}));
    assert_eq!(ids(&a), ["sword"]);
    let sorted = view(
        &s,
        "",
        json!({ "rows": { "sort": "pseudo.edps", "desc": true } }),
    );
    assert_eq!(
        scalar(&sorted["rows"][0]["sort"]),
        json!({ "value": 171.25 })
    );
    assert_eq!(ids(&view(&s, "-has:pseudo.edps", json!({}))).len(), 6);
}

/// S26, rule 8: the q20 rule's display stands at 20 % or more or under
/// the enchant, so either known settles it, whatever is unread of the
/// other: under the enchant, a quality that could not be read, or an
/// array of lines that is no array, leaves the display the value. Where
/// neither settles it, what is unread opens it, both reasons named.
#[test]
fn rule_8_a_known_enchant_settles_q20_whatever_else_is_unread() {
    let unread_quality = json!({ "name": "Quality", "values": 7, "displayMode": 0 });
    let enchant = json!(["Quality does not increase Defences"]);
    // under the enchant, the quality unread
    assert_eq!(
        q20_of(json!({
            "properties": [unread_quality, prop("Armour", &["100"], 0)],
            "enchantMods": enchant,
        })),
        json!({ "value": 100 })
    );
    // under the enchant, another array of lines unread
    assert_eq!(
        q20_of(json!({
            "properties": [prop("Quality", &["+5%"], 0), prop("Armour", &["100"], 0)],
            "enchantMods": enchant,
            "implicitMods": "no array",
        })),
        json!({ "value": 100 })
    );
    // neither settles it: the quality unread and no enchant read
    assert_eq!(
        q20_of(json!({ "properties": [unread_quality, prop("Armour", &["100"], 0)] })),
        json!({ "status": "incomplete" })
    );
    // the quality unread and an array that may hold the enchant: both
    // are the reasons
    let mut s = store();
    list_tabs(&mut s, "pc", "Standard", json!([tab("a1", "A")]), 10);
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "a1",
        "A",
        vec![item(
            "x",
            "X",
            "Base",
            "Rare",
            json!({
                "properties": [unread_quality, prop("Armour", &["100"], 0)],
                "enchantMods": 7,
            }),
        )],
        20,
    );
    let a = view(&s, "has:armour.q20", json!({}));
    let reasons: Vec<&str> = a["total"]["undecided_reasons"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["unread"].as_str().unwrap())
        .collect();
    assert_eq!(reasons, ["`properties`", "enchant lines"], "{a}");
}

/// Rule 10: the damage fields read the attack speed as the
/// `attacks_per_second` field reads it, and each damage property as the
/// one so named, so a property displayed twice leaves every reader of it
/// open, in either order — never the first taken.
#[test]
fn rule_10_the_damage_fields_and_attacks_per_second_read_one_attack_speed() {
    let twice = |first: &str, second: &str| {
        props(vec![
            prop("Attacks per Second", &[first], 0),
            prop("Attacks per Second", &[second], 0),
            prop("Elemental Damage", &["10-20"], 0),
        ])
    };
    for more in [twice("1", "2"), twice("2", "1")] {
        for field in ["attacks_per_second", "pseudo.edps", "pseudo.dps"] {
            assert_eq!(
                sort_of(field, more.clone()),
                json!({ "status": "incomplete" }),
                "{field}"
            );
        }
    }
    // a damage property displayed twice
    let more = props(vec![
        prop("Attacks per Second", &["1"], 0),
        prop("Physical Damage", &["10-20"], 0),
        prop("Physical Damage", &["30-40"], 0),
    ]);
    for field in ["pseudo.pdps", "pseudo.dps"] {
        assert_eq!(
            sort_of(field, more.clone()),
            json!({ "status": "incomplete" }),
            "{field}"
        );
    }
    assert_eq!(sort_of("attacks_per_second", more), json!({ "value": 1 }));
    // once each, read: 1.5 × 15
    assert_eq!(
        sort_of(
            "pseudo.edps",
            props(vec![
                prop("Attacks per Second", &["1.5"], 0),
                prop("Elemental Damage", &["10-20"], 0),
            ])
        ),
        json!({ "value": 22.5 })
    );
}

// ---- the q20 rule against the site ------------------------------------------------------------

const CAPTURES: &str = include_str!("../../../search/pseudo-stats/data/captures.json");
const CENSUS: &str = include_str!("../../../search/trade-query/data/fetch-census.json");

/// The site's figure for each defence, by its `extended` key.
const DEFENCES: [(&str, &str, &str); 4] = [
    ("ar", "Armour", "armour.q20"),
    ("ev", "Evasion Rating", "evasion_rating.q20"),
    ("es", "Energy Shield", "energy_shield.q20"),
    ("ward", "Ward", "ward.q20"),
];

/// How the build's q20 fields stand against the site's own figures over a
/// list of fetched listings: (rows equal, rows one off, rows where the
/// quality is 20 % or more or the enchant stands, all of which equal).
fn against_the_site(fetched: Vec<&Value>) -> (usize, usize, usize, usize) {
    let mut bodies = Vec::new();
    // an item, its field, the site's figure, whether the display stands
    let mut figures: Vec<(String, &str, i64, bool)> = Vec::new();
    for (i, listing) in fetched.into_iter().enumerate() {
        let extended = &listing["extended"];
        if !DEFENCES
            .iter()
            .any(|(key, _, _)| extended.get(key).is_some())
        {
            continue;
        }
        let id = format!("x{i}");
        let properties: Vec<Value> = listing["properties"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| {
                let values: Vec<&str> = p[2]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap())
                    .collect();
                prop(p[1].as_str().unwrap(), &values, 0)
            })
            .collect();
        let quality = properties
            .iter()
            .find(|p| p["name"] == "Quality")
            .map_or(0, |p| {
                p["values"][0][0]
                    .as_str()
                    .unwrap()
                    .trim_matches(['+', '%'])
                    .parse::<i64>()
                    .unwrap()
            });
        let mut more = json!({ "properties": properties });
        let mut enchant = false;
        for (array, lines) in listing["lines"].as_object().unwrap() {
            if array == "pseudoMods" {
                continue;
            }
            enchant |= lines.as_array().unwrap().iter().any(|l| {
                l["description"]
                    .as_str()
                    .is_some_and(|d| d.contains("Quality does not increase Defences"))
            });
            more[array] = lines.clone();
        }
        for (key, _, field) in DEFENCES {
            if let Some(site) = extended.get(key) {
                figures.push((
                    id.clone(),
                    field,
                    site.as_i64().unwrap(),
                    quality >= 20 || enchant,
                ));
            }
        }
        let base = listing["baseType"]
            .as_str()
            .or(listing["typeLine"].as_str())
            .unwrap();
        bodies.push(item(
            &id,
            listing["name"].as_str().unwrap_or(""),
            base,
            "Rare",
            more,
        ));
    }
    let mut s = store();
    list_tabs(&mut s, "pc", "Standard", json!([tab("w", "Whole")]), 10);
    fetch_tab(&mut s, "pc", "Standard", "w", "Whole", bodies, 20);
    let corpus = load(&s, Some("pc"));
    let mut by: std::collections::BTreeMap<&str, std::collections::BTreeMap<String, Value>> =
        Default::default();
    let (mut equal, mut one_off, mut stands, mut faults) = (0, 0, 0, Vec::new());
    for (id, field, site, display_stands) in &figures {
        let read = &by.entry(field).or_insert_with(|| {
            let request: Request = serde_json::from_value(
                json!({ "query": { "text": "" }, "view": { "rows": { "sort": field, "limit": 100_000 } } }),
            )
            .unwrap();
            as_json(&answer(&corpus, &request).unwrap())["rows"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| (r["id"].as_str().unwrap().to_string(), scalar(&r["sort"])))
                .collect()
        })[id];
        let Some(value) = read["value"].as_i64() else {
            faults.push(format!("{id} {field}: no value, {read}"));
            continue;
        };
        match (value - site).abs() {
            0 => equal += 1,
            1 => one_off += 1,
            _ => faults.push(format!("{id} {field}: the site {site}, the build {value}")),
        }
        if *display_stands {
            stands += 1;
            if value != *site {
                faults.push(format!(
                    "{id} {field}: the display stands, the site {site}, the build {value}"
                ));
            }
        }
    }
    assert!(faults.is_empty(), "{}", faults.join("\n"));
    (equal, one_off, stands, figures.len())
}

/// S26 against the trade site (the table's header): every row of the
/// site's `extended.ar|ev|es|ward` over the listings the sittings' 1,168
/// captures fetched, each listing entered whole, is within one of the
/// field — 1,987 of 2,284 equal and 297 one off — and equal on all 423
/// rows at 20 % quality or more or under the enchant, where the display
/// stands. Over the owner's eight searches, 32 of 37 equal and 5 one off.
/// The brief's "812 rows" was stated before it was measured: the 864
/// captures of the pseudo track held 1,313 (1,148 equal, 165 one off, 164
/// standing); step 7's sitting added 874 (2026-10-16), and round x's
/// armour and boots 97 more (2026-10-18), each listing entered within one.
#[test]
fn s26_the_q20_fields_are_within_one_of_every_captured_figure() {
    let captures: Value = serde_json::from_str(CAPTURES).unwrap();
    let fetched: Vec<&Value> = captures["searches"]
        .as_object()
        .unwrap()
        .values()
        .flat_map(|s| s["fetched"].as_array().unwrap())
        .collect();
    assert_eq!(fetched.len(), 7511);
    assert_eq!(against_the_site(fetched), (1987, 297, 423, 2284));

    let census: Value = serde_json::from_str(CENSUS).unwrap();
    let fetched: Vec<&Value> = census["queries"]
        .as_object()
        .unwrap()
        .values()
        .flat_map(|q| q["fetched"].as_array().into_iter().flatten())
        .collect();
    assert_eq!(fetched.len(), 70);
    let (equal, one_off, _, rows) = against_the_site(fetched);
    assert_eq!((equal, one_off, rows), (32, 5, 37));
}
