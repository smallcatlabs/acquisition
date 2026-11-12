//! A row of a mod displayed over several rows (the build plan, step 9c4;
//! C90, C92, C93, C97), through the crate's boundary: an exact template
//! names a mod by its whole text or by a row of it, the mod stays one
//! occurrence, a slot word names the named row's numbers, and what is
//! unread of one row is unread of that row. A total's row names a row the
//! same way, which `tests/pseudo.rs` holds on the trade site's captures.
//! Every count is worked by hand from the fixture below, and every route
//! followed.

use crate::common;

use std::collections::BTreeSet;

use acquisition_search::{
    Collection, ErrorKind, Member, Node, Number, Op, Request, Value as TreeValue, check,
};
use acquisition_store::Store;
use common::answer;
use common::*;
use serde_json::{Value, json};

const THREAD: &str = "Passive Skills in Radius can be Allocated without being connected to your tree\n#% to all Elemental Resistances\nPassage";
/// As a query writes it: a row break is `\n` inside the quotes.
const THREAD_TYPED: &str = r#""Passive Skills in Radius can be Allocated without being connected to your tree\n#% to all Elemental Resistances\nPassage""#;
const ALL_RES: &str = "#% to all Elemental Resistances";
const LIFE: &str = "# to maximum Life";

/// Seven items in pc Standard, `Rows` (w1), every line explicit.
///
/// `thread`, Thread of Hope as the trade site's capture gives it: one row
/// alone, then a mod of three rows whose second is -17 to all elemental;
/// `ring`, 12 to all elemental and 30 life; `taken`, a sentence wrapped
/// over two rows, 36 in the second; `pair`, a crafted mod of two rows,
/// cold damage 3 to 9 and 40 life, and 25 life alone; `long`, a mod of two
/// rows, life with five decimals and 5 cold resistance; `twice`, a mod
/// whose two rows are life, 3 and 7, GGG's two spellings of it; `plain`,
/// 95 life.
fn stash() -> Store {
    let mut s = store();
    list_tabs(&mut s, "pc", "Standard", json!([tab("w1", "Rows")]), 10);
    let rare = |id: &str, mods: Value| {
        item(
            id,
            &format!("Item {id}"),
            "Iron Ring",
            "Rare",
            json!({ "explicitMods": mods }),
        )
    };
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "w1",
        "Rows",
        vec![
            item(
                "thread",
                "Thread of Hope",
                "Crimson Jewel",
                "Unique",
                json!({ "explicitMods": [
                    "Only affects Passives in Small Ring",
                    "Passive Skills in Radius can be Allocated without being connected to your tree\n-17% to all Elemental Resistances\nPassage",
                ] }),
            ),
            rare(
                "ring",
                json!(["+12% to all Elemental Resistances", "+30 to maximum Life"]),
            ),
            rare(
                "taken",
                json!([
                    "Enemies you Kill that are affected by Elemental Ailments\ngrant 36% increased Flask Charges"
                ]),
            ),
            rare(
                "pair",
                json!([
                    { "description": "Adds 3 to 9 Cold Damage\n+40 to maximum Life", "flags": { "crafted": true } },
                    "+25 to maximum Life",
                ]),
            ),
            rare(
                "long",
                json!(["+1.12345 to maximum Life\n+5% to Cold Resistance"]),
            ),
            rare("twice", json!(["+3 to maximum Life\n+7 to Maximum life"])),
            rare("plain", json!(["+95 to maximum Life"])),
        ],
        20,
    );
    s
}

fn asked(s: &Store, text: &str) -> Value {
    as_json(&ask(&load(s, Some("pc")), text).unwrap())
}

fn found(s: &Store, text: &str) -> Vec<String> {
    ids(&asked(s, text))
}

fn view(s: &Store, text: &str, view: Value) -> Value {
    let request: Request =
        serde_json::from_value(json!({ "query": { "text": text }, "view": view })).unwrap();
    as_json(&answer(&load(s, Some("pc")), &request).unwrap())
}

fn counts(term: &Value) -> [u64; 4] {
    ["matched", "failed", "lacked", "undecided"].map(|k| term[k]["count"].as_u64().unwrap())
}

/// The ids a count's route returns, which must be as many as it counted.
fn follow(s: &Store, count: &Value) -> Vec<String> {
    let mut route = count["request"].clone();
    route["view"]["rows"]["limit"] = json!(100);
    let request = serde_json::from_value(route).unwrap();
    let routed = as_json(&answer(&load(s, Some("pc")), &request).unwrap());
    assert_eq!(routed["total"]["matched"], count["count"], "{count}");
    ids(&routed)
}

fn set(ids: &[&str]) -> BTreeSet<String> {
    ids.iter().map(|id| id.to_string()).collect()
}

fn bucket<'a>(table: &'a Value, value: &str) -> &'a Value {
    table["buckets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|b| b["bucket"] == "value" && b["value"] == value)
        .unwrap_or_else(|| panic!("no bucket `{value}` in {table}"))
}

/// A table's buckets as (label, count), in the order printed — the
/// computed values a narrowing matches apart, which `tests/pseudo.rs`
/// pins.
fn shape(table: &Value) -> Vec<(String, u64)> {
    table["buckets"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|b| b["bucket"] != "computed")
        .map(|b| {
            let label = match b["bucket"].as_str().unwrap() {
                "value" => b["value"].as_str().unwrap().to_string(),
                other => format!("({other})"),
            };
            (label, b["count"].as_u64().unwrap())
        })
        .collect()
}

fn shaped(pairs: &[(&str, u64)]) -> Vec<(String, u64)> {
    pairs.iter().map(|(l, n)| (l.to_string(), *n)).collect()
}

/// C90 (owner, 2026-09-27, of a line's term as of a total: "From my side
/// the answer is yes."): an exact template names a mod by a row of it, in
/// any case, and its whole text still names it; `template:` and
/// `template~` test the whole text, across its rows. Naming is the
/// occurrence's, so a not of a row's template is a no on the mod.
#[test]
fn c90_an_exact_template_names_a_mod_by_its_whole_text_or_by_a_row() {
    let s = stash();
    let found = |text: &str| found(&s, text);
    assert_eq!(found(&format!("line(\"{ALL_RES}\")")), ["ring", "thread"]);
    assert_eq!(found(&format!("\"{ALL_RES}\"<0")), ["thread"]);
    assert_eq!(found(&format!("\"{ALL_RES}\">=10")), ["ring"]);
    assert_eq!(found(&format!("line({THREAD_TYPED})")), ["thread"]);
    assert_eq!(found(&format!("line({THREAD_TYPED} arg1=-17)")), ["thread"]);
    // a row without a number is a name too, and a wrapped sentence's row
    assert_eq!(found("line(template=\"PASSAGE\")"), ["thread"]);
    assert_eq!(found("\"grant #% increased Flask Charges\">=36"), ["taken"]);
    assert_eq!(
        found("line(\"Enemies you Kill that are affected by Elemental Ailments\")"),
        ["taken"]
    );
    // two rows are no name: a name is the whole text or one row
    assert_eq!(
        found(
            "line(\"Passive Skills in Radius can be Allocated without being connected to your tree\\n#% to all Elemental Resistances\")"
        ),
        Vec::<String>::new()
    );
    // the item has no such line, by a row as by the whole text
    assert_eq!(
        found(&format!("-\"{ALL_RES}\"")),
        ["long", "pair", "plain", "taken", "twice"]
    );
    // a line of resistance that is not that one: the mod it names is out
    assert_eq!(
        found(&format!("line(template:resistance -\"{ALL_RES}\")")),
        ["long"]
    );
    // `:` and `~` reach across the rows of the whole text, and a pattern's
    // ends are the whole text's
    assert_eq!(found("line(template:\"Life\\n#% to Cold\")"), ["long"]);
    assert_eq!(found("line(template~\"^#% to all\")"), ["ring"]);

    // the terms block, every count routed: ring, pair, long, twice and
    // plain carry a life line; pair's row is 40 and plain's line 95; ring's
    // is 30; long's and twice's numbers are unread
    let a = asked(&s, &format!("\"{LIFE}\">=40"));
    let term = &a["terms"][0];
    assert_eq!(counts(term), [2, 1, 2, 2]);
    assert_eq!(follow(&s, &term["matched"]), ["pair", "plain"]);
    assert_eq!(follow(&s, &term["failed"]), ["ring"]);
    assert_eq!(follow(&s, &term["lacked"]), ["taken", "thread"]);
    assert_eq!(follow(&s, &term["undecided"]), ["long", "twice"]);
}

/// `derive.rs` (owner, 2026-09-19: "it makes sense they are
/// a single occurrence"): a mod named by a row is one occurrence — its
/// source and its flags are its rows', a row shows it whole, and it is
/// shown once however many of its rows a term names.
#[test]
fn a_mod_named_by_a_row_is_one_occurrence() {
    let s = stash();
    let found = |text: &str| found(&s, text);
    assert_eq!(found(&format!("line(\"{LIFE}\" is:crafted)")), ["pair"]);
    assert_eq!(
        found("line(\"Adds # to # Cold Damage\" is:crafted source=explicit)"),
        ["pair"]
    );
    assert_eq!(
        found(&format!("line(\"{LIFE}\" source=implicit)")),
        Vec::<String>::new()
    );
    // two templates, two rows of the one mod
    assert_eq!(
        found(&format!("line(\"Adds # to # Cold Damage\" \"{LIFE}\")")),
        ["pair"]
    );
    let a = asked(&s, &format!("line(\"{LIFE}\")"));
    let row = a["rows"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["id"] == "pair")
        .unwrap();
    assert_eq!(
        row["matched"][0]["shows"],
        json!([
            { "line": { "source": "explicit", "flags": ["crafted"],
                "text": "Adds 3 to 9 Cold Damage\n+40 to maximum Life" } },
            { "line": { "source": "explicit", "text": "+25 to maximum Life" } },
        ])
    );
    let twice = a["rows"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["id"] == "twice")
        .unwrap();
    assert_eq!(twice["matched"][0]["shows"].as_array().unwrap().len(), 1);
}

/// C92, `template.rs`: named by a row, a slot word names that
/// row's numbers — `arg1` is the row's first number, whatever the rows
/// before it display — and named by its whole text, the mod's in order.
/// A comparison, a sum, a largest and the together count read the same
/// number.
#[test]
fn c92_a_slot_names_the_numbers_of_the_row_named() {
    let s = stash();
    let found = |text: &str| found(&s, text);
    // pair's life is the second row's first number, the mod's third
    assert_eq!(found(&format!("\"{LIFE}\"=40")), ["pair"]);
    assert_eq!(found(&format!("\"{LIFE}\"=3")), Vec::<String>::new());
    assert_eq!(
        found("line(\"Adds # to # Cold Damage\" low>=3 high<=9)"),
        ["pair"]
    );
    assert_eq!(found("\"Adds # to # Cold Damage\".avg=6"), ["pair"]);
    let whole = "\"Adds # to # Cold Damage\\n# to maximum Life\"";
    assert_eq!(found(&format!("line({whole} arg3=40 low=3)")), ["pair"]);
    // a slot is checked against the template typed: a row of one number
    // has no third
    let e = ask(&load(&s, Some("pc")), &format!("line(\"{LIFE}\" arg3=40)"))
        .unwrap_err()
        .to_json();
    assert_eq!(e["kind"], "slot_unknown", "{e}");
    // two rows named together name no one row, and a number beside them
    // is refused (`c92_a_number_beside_two_rows_…`, below)
    let both = format!("\"Adds # to # Cold Damage\" \"{LIFE}\"");
    let e = ask(&load(&s, Some("pc")), &format!("line({both} arg3=40)"))
        .unwrap_err()
        .to_json();
    assert_eq!(e["kind"], "slot_of_two_rows", "{e}");
    // under an or the row a template names on the occurrence: pair's life
    // row, plain's line; long's two rows are both named, and its first
    // number is unread
    let either = format!("line((\"{LIFE}\" or \"#% to Cold Resistance\") arg1>=40)");
    let a = asked(&s, &either);
    assert_eq!(ids(&a), ["pair", "plain"]);
    assert_eq!(follow(&s, &a["terms"][0]["undecided"]), ["long", "twice"]);

    // the item's sum: 40 and 25
    assert_eq!(found(&format!("sum(\"{LIFE}\")=65")), ["pair"]);
    // together: neither of pair's reaches 60 and the two do; ring's 30
    // does not
    let a = asked(&s, &format!("\"{LIFE}\">=60"));
    let term = &a["terms"][0];
    assert_eq!(counts(term), [1, 2, 2, 2]);
    assert_eq!(follow(&s, &term["failed"]), ["pair", "ring"]);
    assert_eq!(term["together"]["count"], 1);
    assert_eq!(follow(&s, &term["together"]), ["pair"]);

    // the largest: plain 95, pair's row 40, ring 30; long's and twice's
    // are not established, and the two others have none
    let sorted = view(
        &s,
        "",
        json!({ "rows": { "sort": format!("line(\"{LIFE}\").arg1"), "desc": true } }),
    );
    let rows: Vec<(&str, &Value, &Value)> = sorted["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (
                r["id"].as_str().unwrap(),
                &sort_value(r)["value"],
                &sort_value(r)["status"],
            )
        })
        .collect();
    assert_eq!(
        rows[..3],
        [
            ("plain", &json!(95), &Value::Null),
            ("pair", &json!(40), &Value::Null),
            ("ring", &json!(30), &Value::Null),
        ]
    );
    let open: BTreeSet<String> = rows
        .iter()
        .filter(|(_, _, status)| **status == "incomplete")
        .map(|(id, _, _)| id.to_string())
        .collect();
    assert_eq!(open, set(&["long", "twice"]));
    assert_eq!(
        sort_value(&sorted["rows"][1])["shows"][0]["line"]["text"],
        "Adds 3 to 9 Cold Damage\n+40 to maximum Life"
    );
}

/// C91, C92 (the outside review of step 9c4, finding 1): a comparison
/// reads the numbers of the part the quoted templates conjoined with it
/// name, so an alternative beside it — one that holds nowhere, one that
/// names another row, one with no template — changes no number it reads.
/// The group is its alternatives spread out, each read on its own, as the
/// same alternatives are at the item's level.
#[test]
fn c92_an_alternative_changes_no_number_a_comparison_reads() {
    let s = stash();
    let found = |text: &str| found(&s, text);
    let life = format!("\"{LIFE}\" arg1=40");
    assert_eq!(found(&format!("line({life})")), ["pair"]);
    for beside in [
        "arg1=999",
        "\"Adds # to # Cold Damage\" source=implicit",
        "\"Adds # to # Cold Damage\" -\"Adds # to # Cold Damage\"",
        "\"#% to Cold Resistance\" arg1=999",
        "false()",
    ] {
        assert_eq!(
            found(&format!("line(({life}) or ({beside}))")),
            ["pair"],
            "{beside}"
        );
        assert_eq!(
            found(&format!("line(({beside}) or ({life}))")),
            ["pair"],
            "{beside}"
        );
        // as the two are at the item's level
        assert_eq!(
            found(&format!("line({life}) or line({beside})")),
            ["pair"],
            "{beside}"
        );
    }
    // each alternative reads its own part: the mod's first number is 3,
    // its cold damage row's 3 to 9, its life row's 40 — and twice's mod,
    // read in order, begins with 3 too
    assert_eq!(
        found(&format!("line((\"{LIFE}\" arg1=41) or arg1=3)")),
        ["pair", "twice"]
    );
    assert_eq!(
        found(&format!(
            "line((\"{LIFE}\" arg1=41) or (\"Adds # to # Cold Damage\" high=9))"
        )),
        ["pair"]
    );
    assert_eq!(
        found(&format!("line((\"{LIFE}\" arg1=3) or arg1=40)")),
        Vec::<String>::new()
    );
    // a not of two together is either's not, and of either is both's:
    // thread's resistance is -17 and explicit, ring's 12
    assert_eq!(
        found(&format!("line(\"{ALL_RES}\" -(arg1=-17 source=implicit))")),
        ["ring", "thread"]
    );
    assert_eq!(
        found(&format!(
            "line(\"{ALL_RES}\" -(arg1=-17 or source=implicit))"
        )),
        ["ring"]
    );
    // a template beside the alternatives is conjoined with each
    assert_eq!(
        found(&format!(
            "line(\"{LIFE}\" (arg1=40 or arg1=999) is:crafted)"
        )),
        ["pair"]
    );
    // a sum and a largest read each part an alternative names: pair's mod
    // is 3 and 40 under the two, and its own life line 25
    assert_eq!(
        found(&format!(
            "sum(line(\"{LIFE}\" or \"Adds # to # Cold Damage\").arg1)=68"
        )),
        ["pair"]
    );
    let sorted = view(
        &s,
        "id:pair",
        json!({ "rows": { "sort": format!("line(\"Adds # to # Cold Damage\" or \"{LIFE}\").arg1") } }),
    );
    assert_eq!(sort_value(&sorted["rows"][0])["value"], 40);
}

const COLD: &str = "Adds # to # Cold Damage";
const FIRE: &str = "Adds # to # Fire Damage";
/// As a query writes it, `one`'s mod by its whole text.
const COLD_LIFE_TYPED: &str = r#""Adds # to # Cold Damage\n# to maximum Life""#;

/// Three items of one mod each, in pc Standard, `Both` (b1): `one`, cold
/// damage 3 to 9 then 40 life; `other`, 40 life then cold damage 3 to 9;
/// `ranged`, cold damage 3 to 9 then fire damage 5 to 7.
fn both_stash() -> Store {
    let mut s = store();
    list_tabs(&mut s, "pc", "Standard", json!([tab("b1", "Both")]), 10);
    let one = |id: &str, line: &str| {
        item(
            id,
            &format!("Item {id}"),
            "Iron Ring",
            "Rare",
            json!({ "explicitMods": [line] }),
        )
    };
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "b1",
        "Both",
        vec![
            one("one", "Adds 3 to 9 Cold Damage\n+40 to maximum Life"),
            one("other", "+40 to maximum Life\nAdds 3 to 9 Cold Damage"),
            one("ranged", "Adds 3 to 9 Cold Damage\nAdds 5 to 7 Fire Damage"),
        ],
        20,
    );
    s
}

/// C91, C92 (owner, 2026-09-28: "refuse"; the outside review of step
/// 9c4, its fourth look): a number word beside two rows of a mod named
/// together has no one row to read, and which is never guessed — the
/// term is refused however it is spelled, as a text and as a tree, and
/// each reading offered is one this build answers. Read of the mod's
/// numbers in order, as it was, a row named as a condition moved what
/// the number beside it read: `one` left `"# to maximum Life" arg1>=40`
/// when the cold damage it shows was named, and a not of cold damage's
/// `high` held on `ranged`, whose `high` is 9.
#[test]
fn c92_a_number_beside_two_rows_named_together_is_refused() {
    let s = both_stash();
    let c = load(&s, Some("pc"));
    let found = |text: &str| found(&s, text);
    let none = Vec::<String>::new();
    let refused = |text: &str| -> Value {
        let e = match ask(&c, text) {
            Ok(_) => panic!("`{text}` is answered"),
            Err(e) => e.to_json(),
        };
        assert_eq!(e["kind"], "slot_of_two_rows", "`{text}`: {e}");
        // a reading is a text this build answers
        for reading in e["readings"].as_array().into_iter().flatten() {
            let reading = reading.as_str().unwrap();
            if let Err(e) = ask(&c, reading) {
                panic!("`{text}` offers `{reading}`, which is refused: {e}");
            }
        }
        e
    };
    let (life, cold, fire) = (
        format!("\"{LIFE}\""),
        format!("\"{COLD}\""),
        format!("\"{FIRE}\""),
    );
    // in any order, under any parentheses, beside an alternative that
    // holds nowhere or one that names the same row again
    for text in [
        format!("line({life} {cold} arg1>=40)"),
        format!("line({cold} arg1>=40 {life})"),
        format!("line(({life} arg1>=40) {cold})"),
        format!("line({life} ({cold} or false()) arg1>=40)"),
        format!("line({life} ({life} or {cold}) arg1=3)"),
        format!("line(({life} {cold} arg1=3) or {fire})"),
        // a doubled not is none, and names what it holds
        format!("line(--{life} {cold} arg1>=40)"),
        // a not is read with what is conjoined with it
        format!("line({fire} -({cold} high=9))"),
        format!("line({fire} -(is:crafted or -({cold} high=9)))"),
        // a value, compared or asked
        format!("sum(line({life} {cold}).arg1)>=1"),
        format!("line({life} {cold}).arg1>=40"),
        format!("undecided(line({life} {cold}).arg1)"),
        format!("-line({life} {cold} arg1>=40) rarity=rare"),
    ] {
        let e = refused(&text);
        assert_eq!(e["readings"].as_array().map(Vec::len), Some(2), "{text}");
    }
    // what is offered: the row whose number is meant stays quoted, and
    // the other is asked of the mod's text
    let e = refused(&format!("line({life} {cold} arg1>=40)"));
    assert_eq!(
        e["readings"],
        json!([
            format!("line({life} template:{cold} arg1>=40)"),
            format!("line(template:{life} {cold} arg1>=40)"),
        ])
    );
    assert!(
        e["error"].as_str().unwrap().contains(&life)
            && e["error"].as_str().unwrap().contains(&cold),
        "{e}"
    );
    assert_eq!(
        found(&format!("line({life} template:{cold} arg1>=40)")),
        ["one", "other"]
    );
    assert_eq!(
        found(&format!("line(template:{life} {cold} arg1>=40)")),
        none
    );
    // and under the not: fire's high is 7, cold's is 9
    let e = refused(&format!("line({fire} -({cold} high=9))"));
    assert_eq!(
        e["readings"],
        json!([
            format!("line({fire} -(template:{cold} high=9))"),
            format!("line(template:{fire} -({cold} high=9))"),
        ])
    );
    assert_eq!(
        found(&format!("line({fire} -(template:{cold} high=9))")),
        ["ranged"]
    );
    assert_eq!(
        found(&format!("line(template:{fire} -({cold} high=9))")),
        none
    );
    // a reading the build would refuse is not offered: with life asked
    // of the text, fire and cold are two rows still
    let e = refused(&format!("line(({life} or {fire}) {cold} arg1>=1)"));
    assert_eq!(
        e["readings"],
        json!([format!("line(({life} or {fire}) template:{cold} arg1>=1)")])
    );
    // what a row is sorted by
    let sort = format!("line({life} {cold}).arg1");
    let request: Request = serde_json::from_value(
        json!({ "query": { "text": "" }, "view": { "rows": { "sort": sort } } }),
    )
    .unwrap();
    let e = match answer(&c, &request) {
        Ok(_) => panic!("`{sort}` sorts"),
        Err(e) => e.to_json(),
    };
    assert_eq!(e["kind"], "slot_of_two_rows", "{e}");
    assert_eq!(
        e["readings"],
        json!([
            format!("line({life} template:{cold}).arg1"),
            format!("line(template:{life} {cold}).arg1"),
        ])
    );
    // a tree is refused as its text is
    let test = |attr: &str, op: Op, value: TreeValue| Member::Test {
        attr: attr.to_string(),
        op,
        value,
    };
    let tree = Node::Members {
        of: Collection::Lines,
        where_: Box::new(Member::All(vec![
            test("template", Op::Eq, TreeValue::Text(LIFE.to_string())),
            test("template", Op::Eq, TreeValue::Text(COLD.to_string())),
            test("arg1", Op::Ge, TreeValue::Number(Number::Int(40))),
        ])),
    };
    let e = check(&tree).unwrap_err();
    assert_eq!(e.kind, ErrorKind::SlotOfTwoRows);
    assert_eq!(
        e.readings,
        [
            format!("line({life} template:{cold} arg1>=40)"),
            format!("line(template:{life} {cold} arg1>=40)"),
        ]
    );
}

/// C92, `template.rs` (the same look, finding 1): what is not
/// refused is read as it was — two rows named with no number beside
/// them, alternatives each naming its own row, a row asked of the mod's
/// text, a not of a name, a mod named by its whole text beside a row of
/// it — and a slot is checked against the one quoted template among the
/// group's conjuncts where that template alone says what the slot reads:
/// beside the mod's whole text the numbers are the mod's, and the check
/// that took `arg3` for the row's refused a term the evaluator reads.
#[test]
fn c92_a_slot_is_checked_against_the_template_that_says_what_it_reads() {
    let s = both_stash();
    let found = |text: &str| found(&s, text);
    let none = Vec::<String>::new();
    let (life, cold, fire) = (
        format!("\"{LIFE}\""),
        format!("\"{COLD}\""),
        format!("\"{FIRE}\""),
    );
    assert_eq!(found(&format!("line({life} {cold})")), ["one", "other"]);
    assert_eq!(
        found(&format!("line(({life} or {cold}) arg1>=40)")),
        ["one", "other"]
    );
    assert_eq!(
        found(&format!("line(({life} {cold}) or ({fire} high=7))")),
        ["one", "other", "ranged"]
    );
    // GGG's two spellings of one row are one name
    assert_eq!(
        found("line(\"# to maximum Life\" \"# to Maximum life\" arg1>=40)"),
        ["one", "other"]
    );
    // a row asked of the mod's text names nothing, and moves no number
    assert_eq!(
        found(&format!("line({life} template:\"cold damage\" arg1>=40)")),
        ["one", "other"]
    );
    assert_eq!(
        found(&format!("line({life} template:\"cold damage\" arg1=3)")),
        none
    );
    // a not of a name is of the occurrence, and names nothing
    assert_eq!(
        found(&format!("line({life} -{fire} arg1=40)")),
        ["one", "other"]
    );
    assert_eq!(found(&format!("line({life} -{cold} arg1=40)")), none);
    // named by its whole text, the mod's numbers in order, a row of it
    // named beside or not
    let whole = COLD_LIFE_TYPED;
    assert_eq!(found(&format!("line({whole} arg3=40)")), ["one"]);
    assert_eq!(found(&format!("line({whole} {life} arg3=40)")), ["one"]);
    assert_eq!(
        found(&format!("line({life} ({whole} or false()) arg3=40)")),
        ["one"]
    );
    assert_eq!(
        found(&format!("line({life} -({whole} arg3=40))")),
        ["other"]
    );
    // the row alone says what the slot reads, wherever the slot sits
    for text in [
        format!("line({life} arg3=40)"),
        format!("line({life} -(high=9))"),
        format!("line({life} (high=9 or true()))"),
        format!("line({life} -{cold} high=9)"),
        format!("line({life} template:\"cold damage\" high=9)"),
    ] {
        let e = ask(&load(&s, Some("pc")), &text).unwrap_err().to_json();
        assert_eq!(e["kind"], "slot_unknown", "`{text}`: {e}");
    }
}

/// What a slot is read beside is weighed from the query, and a reading
/// is made by the group that was asked and by no reading of it: a
/// hundred rows named in five alternations and twenty rows named
/// together are each weighed in no time, and none of their readings is
/// offered, two rows being named in each still. A slot sixteen groups
/// deep — thirty-two levels, the deepest a query nests (C109) — is
/// weighed within the bound too, which at that depth holds a cost that
/// quadruples by the group and not one that doubles: the limit is what
/// bounds the second, at about a second in this build.
#[test]
fn what_a_slot_is_read_beside_costs_the_groups_size() {
    let s = both_stash();
    let c = load(&s, Some("pc"));
    let name = |group: u8, n: u8| {
        format!(
            "\"# to {}{}\"",
            char::from(b'A' + group),
            char::from(b'a' + n)
        )
    };
    let alternation = |group: u8| {
        let names: Vec<String> = (0..20).map(|n| name(group, n)).collect();
        format!("({})", names.join(" or "))
    };
    let wide: Vec<String> = (0..5).map(alternation).collect();
    let together: Vec<String> = (0..20).map(|n| name(0, n)).collect();
    let mut deep = format!("\"{LIFE}\" arg1>=40");
    for _ in 0..16 {
        deep = format!("(({deep}) or false()) -is:fractured");
    }
    let started = std::time::Instant::now();
    for text in [
        format!("line({} arg1>=1)", wide.join(" ")),
        format!("line({} arg1>=1)", together.join(" ")),
    ] {
        let e = ask(&c, &text).unwrap_err().to_json();
        assert_eq!(e["kind"], "slot_of_two_rows", "{e}");
        assert_eq!(e.get("readings"), None, "{e}");
    }
    assert_eq!(found(&s, &format!("line({deep})")), ["one", "other"]);
    assert!(
        started.elapsed() < std::time::Duration::from_secs(20),
        "{:?}",
        started.elapsed()
    );
    println!("weighed in {:?}", started.elapsed());
}

/// Four items of one line each, in pc Standard, `Parts` (p1): `two`,
/// cold damage 3 to 9 and 40 life in one mod; `first`, 40 life and 5 cold
/// resistance in one mod; `equal`, 40 life and 40 cold resistance in one
/// mod; `alone`, 40 life.
fn parts_stash() -> Store {
    let mut s = store();
    list_tabs(&mut s, "pc", "Standard", json!([tab("p1", "Parts")]), 10);
    let one = |id: &str, line: &str| {
        item(
            id,
            &format!("Item {id}"),
            "Iron Ring",
            "Rare",
            json!({ "explicitMods": [line] }),
        )
    };
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "p1",
        "Parts",
        vec![
            one("two", "Adds 3 to 9 Cold Damage\n+40 to maximum Life"),
            one("first", "+40 to maximum Life\n+5% to Cold Resistance"),
            one("equal", "+40 to maximum Life\n+40% to Cold Resistance"),
            one("alone", "+40 to maximum Life"),
        ],
        20,
    );
    s
}

/// C91, C93 (the outside review of step 9c4, its second look, finding
/// 1): a not is of what it holds, read as it is read there — the
/// templates inside it say what its comparisons read, as they do outside
/// one, and those conjoined with the not say it where it has none. On an
/// occurrence that is read, a group or its not holds, never both and
/// never neither.
#[test]
fn c93_a_not_is_of_what_it_holds_read_as_it_is_read_there() {
    let s = parts_stash();
    let found = |text: &str| found(&s, text);
    let all = ["alone", "equal", "first", "two"];
    let none = Vec::<String>::new();
    let life_is = |n: i32| format!("\"{LIFE}\" arg1={n}");
    assert_eq!(found(&format!("line({})", life_is(40))), all);
    assert_eq!(found(&format!("line(-({}))", life_is(40))), none);
    // two's first number is 3, and its life is not
    assert_eq!(found(&format!("line({})", life_is(3))), none);
    assert_eq!(found(&format!("line(-({}))", life_is(3))), all);
    for n in [3, 40] {
        let (is, not) = (life_is(n), format!("-({})", life_is(n)));
        assert_eq!(found(&format!("line(({is}) or {not})")), all, "{n}");
        assert_eq!(found(&format!("line({not} or ({is}))")), all, "{n}");
        assert_eq!(found(&format!("line(({is}) {not})")), none, "{n}");
        assert_eq!(found(&format!("line({is}) or line({not})")), all, "{n}");
        // a doubled not is none, and a template under one still says
        // what the comparison beside it reads
        assert_eq!(
            found(&format!("line(-{not})")),
            found(&format!("line({is})")),
            "{n}"
        );
        assert_eq!(
            found(&format!("line(--\"{LIFE}\" arg1={n})")),
            found(&format!("line({is})")),
            "{n}"
        );
    }
    // the template conjoined with the not says what it reads
    assert_eq!(found(&format!("line(\"{LIFE}\" -(arg1=40))")), none);
    assert_eq!(found(&format!("line(\"{LIFE}\" -(arg1=3))")), all);
    assert_eq!(
        found("line(\"#% to Cold Resistance\" -(arg1=5 or arg1=3))"),
        ["equal"]
    );
    // a not inside a not: first's cold resistance is 5, equal's 40, and
    // the two others have none
    assert_eq!(
        found("line(-(\"#% to Cold Resistance\" -(arg1=5)))"),
        ["alone", "first", "two"]
    );
    // every count of the not, routed
    let a = asked(&s, &format!("line(-({}))", life_is(40)));
    let term = &a["terms"][0];
    assert_eq!(counts(term)[0], 0);
    assert_eq!(counts(term)[3], 0);
    for kind in ["failed", "lacked"] {
        if term[kind]["count"] != 0 {
            follow(&s, &term[kind]);
        }
    }
}

/// C92, C95 (the same look, finding 2): a number of a mod is added
/// once, however many alternatives read it — the mod's first number and
/// its first row's first are one number — and two numbers of two rows are
/// two, equal or not.
#[test]
fn c95_a_number_two_alternatives_read_is_added_once() {
    let s = parts_stash();
    let found = |text: &str| found(&s, text);
    // by words alone, the mod's numbers in order: two's first is 3
    assert_eq!(
        found("sum(line(template:life).arg1)=40"),
        ["alone", "equal", "first"]
    );
    assert_eq!(found("sum(line(template:life).arg1)=3"), ["two"]);
    // the row named and the mod's first number are one number, but on
    // two, whose life is its third
    let either = format!("\"{LIFE}\" or template:life");
    assert_eq!(
        found(&format!("sum(line({either}).arg1)=40")),
        ["alone", "equal", "first"]
    );
    assert_eq!(found(&format!("sum(line({either}).arg1)=43")), ["two"]);
    assert_eq!(
        found(&format!("sum(line({either}).arg1)=80")),
        Vec::<String>::new()
    );
    // two rows are two numbers, equal or not
    let rows = format!("\"{LIFE}\" or \"#% to Cold Resistance\"");
    assert_eq!(found(&format!("sum(line({rows}).arg1)=80")), ["equal"]);
    assert_eq!(found(&format!("sum(line({rows}).arg1)=45")), ["first"]);
    // together: one number of 40 reaches no 60, and two's 40 and 3 reach 42
    let a = asked(&s, &format!("line(({either}) arg1>=60)"));
    assert_eq!(counts(&a["terms"][0]), [0, 4, 0, 0]);
    assert_eq!(a["terms"][0]["together"]["count"], 0);
    let a = asked(&s, &format!("line(({either}) arg1>=42)"));
    assert_eq!(a["terms"][0]["together"]["count"], 1);
    assert_eq!(follow(&s, &a["terms"][0]["together"]), ["two"]);
    // what a row sorts by is the number, once
    let sorted = view(
        &s,
        "id:first",
        json!({ "rows": { "sort": format!("sum(line({either}).arg1)") } }),
    );
    assert_eq!(sort_value(&sorted["rows"][0])["value"], 40);

    // a number added is added, whatever an alternative that may hold
    // would read: 40 life beside a cold resistance the search does not
    // read, which leaves the second alternative open and the sum whole
    let mut s = store();
    list_tabs(&mut s, "pc", "Standard", json!([tab("o1", "Open")]), 10);
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "o1",
        "Open",
        vec![item(
            "half",
            "Item half",
            "Iron Ring",
            "Rare",
            json!({ "explicitMods": ["+40 to maximum Life\n+5.12345% to Cold Resistance"] }),
        )],
        20,
    );
    let may = format!("\"{LIFE}\" or (template:life arg2>=1)");
    assert_eq!(
        self::found(
            &s,
            &format!("line({may}) undecided(line(template:life arg2>=1))")
        ),
        ["half"]
    );
    assert_eq!(
        self::found(&s, &format!("sum(line({may}).arg1)=40")),
        ["half"]
    );
    // and one no alternative that holds reads is open
    assert_eq!(
        self::found(&s, "undecided(sum(line(template:life arg2>=1).arg1))"),
        ["half"]
    );
}

/// C93, rule 8 of the plan (the outside review of step 9c4, its third
/// look, finding 2): a row whose template another row of its mod
/// displays has no one place among the mod's numbers — its number is
/// either's — so nothing read elsewhere is known to be it, and what it
/// leaves open stays open beside a number that is added. twice's two
/// rows are life, 3 and 7.
#[test]
fn c93_a_number_that_is_either_rows_is_no_number_added_already() {
    let s = stash();
    let found = |text: &str| found(&s, &format!("id:twice {text}"));
    let none = Vec::<String>::new();
    // by words alone, the mod's numbers in order
    assert_eq!(found("sum(line(template:life).arg1)=3"), ["twice"]);
    // by the row named, either's
    assert_eq!(found(&format!("undecided(sum(\"{LIFE}\"))")), ["twice"]);
    // by both: 3 is added, and the row named may be the 7
    let either = format!("sum(line(\"{LIFE}\" or template:life).arg1)");
    assert_eq!(found(&format!("undecided({either})")), ["twice"]);
    for n in [3, 6, 7, 10] {
        assert_eq!(found(&format!("{either}={n}")), none, "{n}");
    }
    let sorted = view(&s, "id:twice", json!({ "rows": { "sort": either } }));
    assert_eq!(
        scalar(sort_value(&sorted["rows"][0])),
        json!({ "value": 3, "status": "incomplete" })
    );
    // and the comparison is open on it: its 3 is under 5, and the row
    // named may be the 7. Over the seven items ring, pair and plain
    // match, thread and taken have no such line, and long's life is
    // unread
    let a = asked(&s, &format!("line((\"{LIFE}\" or template:life) arg1>=5)"));
    let term = &a["terms"][0];
    assert_eq!(counts(term), [3, 0, 2, 2]);
    assert_eq!(follow(&s, &term["undecided"]), ["long", "twice"]);
    assert_eq!(term["together"]["count"], 0);
}

/// A group costs its size (the same look, finding 1): a not inside a
/// not inside a not is weighed once for each, where each had weighed
/// all it held again for every part a template may name — ten deep took
/// 2.7 s of a release build on one item, four times eight deep's.
/// Fifteen deep is no time — thirty-one levels, a not and its group two
/// each and the first group's own one more, as deep as a query nests
/// (`tree::MAX_DEPTH`) — and an even number of nots is none.
#[test]
fn a_not_inside_a_not_costs_its_size() {
    let s = parts_stash();
    let mut inner = format!("\"{LIFE}\" arg1=40");
    let mut answers = Vec::new();
    let started = std::time::Instant::now();
    for depth in 0..=15 {
        if depth % 5 == 0 || depth == 14 {
            answers.push((depth, found(&s, &format!("line({inner})"))));
        }
        // the first group holds two conditions and is parenthesised; a
        // not after it is one term, whose parentheses would be a level
        // and no node
        inner = if depth == 0 {
            format!("-(true() ({inner}))")
        } else {
            format!("-(true() {inner})")
        };
    }
    let all = ["alone", "equal", "first", "two"]
        .map(String::from)
        .to_vec();
    assert_eq!(
        answers,
        [
            (0, all.clone()),
            (5, Vec::new()),
            (10, all.clone()),
            (14, all),
            (15, Vec::new()),
        ]
    );
    assert!(
        started.elapsed() < std::time::Duration::from_secs(20),
        "{:?}",
        started.elapsed()
    );
}

/// Rule 8 of the plan, C93: what is unread of one row is unread of that
/// row. A number the search does not read in the first row leaves the
/// second row's comparison decided and the first's open, with its reason;
/// the text is a witness of the line all the same. Two rows displaying
/// one template leave that row's number open, and say so.
#[test]
fn c93_a_row_is_open_by_its_own_number_and_no_other() {
    let s = stash();
    let found = |text: &str| found(&s, text);
    assert_eq!(found("\"#% to Cold Resistance\"=5"), ["long"]);
    assert_eq!(
        found(&format!("line(\"{LIFE}\")")),
        ["long", "pair", "plain", "ring", "twice"]
    );
    let open = asked(&s, &format!("undecided(\"{LIFE}\">=1)"));
    assert_eq!(ids(&open), ["long", "twice"]);
    let why = |id: &str| -> Vec<(String, String)> {
        let row = open["rows"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["id"] == id)
            .unwrap();
        row["matched"][0]["shows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| {
                (
                    e["undecided"]["unread"].as_str().unwrap().to_string(),
                    e["undecided"]["problem"].as_str().unwrap().to_string(),
                )
            })
            .collect()
    };
    assert_eq!(
        why("long"),
        [(
            "the numbers of explicit lines".to_string(),
            "`explicitMods[0]`: a number with more digits than the search reads: ten whole, four decimals".to_string()
        )]
    );
    assert_eq!(
        why("twice"),
        [(
            "the numbers of explicit lines".to_string(),
            "`explicitMods[0]`: two rows of this mod display one template, `# to maximum Life`: a number named by that row is either's".to_string()
        )]
    );
    // named by its whole text, the mod's numbers in order: the second is
    // read, the first is not
    let whole = "\"# to maximum Life\\n#% to Cold Resistance\"";
    assert_eq!(found(&format!("line({whole} arg2=5)")), ["long"]);
    let a = asked(&s, &format!("line({whole} arg1>=1)"));
    assert_eq!(counts(&a["terms"][0]), [0, 0, 6, 1]);
    // twice's own numbers stand where the mod is named whole
    assert_eq!(
        found("line(\"# to maximum Life\\n# to maximum Life\" arg1=3 arg2=7)"),
        ["twice"]
    );
    // a total reads the row: long's cold resistance is 5, decided, and
    // its life is open
    assert_eq!(found("pseudo.total_cold_res=5"), ["long"]);
    assert_eq!(found("undecided(pseudo.total_life)"), ["long", "twice"]);
}

/// C97, C105: the vocabulary lists a mod by its whole text, and a row's
/// count is its term's matched count — a template's row counts the mods
/// it names by a row too, with their numbers, sources and flags. A row of
/// a mod that no mod displays alone — a wrapped sentence's — is no row of
/// the vocabulary.
#[test]
fn c97_the_vocabulary_counts_a_template_as_its_term_matches() {
    let s = stash();
    let a = view(
        &s,
        "",
        json!({ "counts": { "keys": ["line"], "limit": 20 } }),
    );
    let table = &a["view"]["counts"]["tables"][0];
    assert_eq!(
        shape(table),
        shaped(&[
            (LIFE, 5),
            (ALL_RES, 2),
            ("# to maximum Life\n# to Maximum life", 1),
            ("# to maximum Life\n#% to Cold Resistance", 1),
            ("Adds # to # Cold Damage\n# to maximum Life", 1),
            (
                "Enemies you Kill that are affected by Elemental Ailments\ngrant #% increased Flask Charges",
                1
            ),
            ("Only affects Passives in Small Ring", 1),
            (THREAD, 1),
        ])
    );
    // every row's route returns what it counted, its kinds' too
    for b in table["buckets"].as_array().unwrap() {
        follow(&s, b);
        for kind in b["sources"].as_array().into_iter().flatten() {
            follow(&s, kind);
        }
        for kind in b["flags"].as_array().into_iter().flatten() {
            follow(&s, kind);
        }
    }
    let all = bucket(table, ALL_RES);
    assert_eq!(all["term"], format!("line(\"{ALL_RES}\")"));
    assert_eq!(follow(&s, all), ["ring", "thread"]);
    assert_eq!(
        all["slots"],
        json!([{ "slot": "arg1", "min": -17, "max": 12 }])
    );
    let life = bucket(table, LIFE);
    assert_eq!(follow(&s, life), ["long", "pair", "plain", "ring", "twice"]);
    // pair's 25 and plain's 95, a row's 40 between; long's and twice's
    // were not read
    assert_eq!(
        life["slots"],
        json!([{ "slot": "arg1", "min": 25, "max": 95, "incomplete": true }])
    );
    assert_eq!(life["flags"][0]["kind"], "crafted");
    assert_eq!(follow(&s, &life["flags"][0]), ["pair"]);
    assert_eq!(life["sources"][0]["count"], 5);
    // a mod's own row reads the mod's numbers in order
    assert_eq!(
        bucket(table, "Adds # to # Cold Damage\n# to maximum Life")["slots"],
        json!([
            { "slot": "low", "min": 3, "max": 3 },
            { "slot": "high", "min": 9, "max": 9 },
            { "slot": "arg3", "min": 40, "max": 40 },
        ])
    );

    // narrowed by words: the whole texts that hold them
    let a = view(&s, "", json!({ "counts": { "keys": ["line:resist"] } }));
    let table = &a["view"]["counts"]["tables"][0];
    assert_eq!(
        shape(table),
        shaped(&[
            (ALL_RES, 2),
            ("# to maximum Life\n#% to Cold Resistance", 1),
            (THREAD, 1),
            ("(none)", 4),
        ])
    );
    for b in table["buckets"].as_array().unwrap() {
        if b["bucket"] != "computed" {
            follow(&s, b);
        }
    }
    // narrowed by a pattern whose ends are the whole text's: the template
    // is listed by ring's line, and counted as its term matches — thread
    // by its row, which the narrowing's own term does not select
    let a = view(&s, "", json!({ "counts": { "keys": ["line~^#% to all"] } }));
    let table = &a["view"]["counts"]["tables"][0];
    assert_eq!(shape(table), shaped(&[(ALL_RES, 2), ("(none)", 6)]));
    assert_eq!(follow(&s, bucket(table, ALL_RES)), ["ring", "thread"]);
}

/// C100, invariant 2: a quoted template resolves to itself and lists
/// nothing where it named a row — and lists the spellings where it found
/// two, a row's among them; a `:` selector resolves to whole texts. A
/// suggestion's count is its term's (invariant 4), the mods it names by
/// a row counted.
#[test]
fn c100_a_quoted_template_resolves_to_the_names_it_found() {
    let s = stash();
    let a = asked(&s, &format!("line(\"{ALL_RES}\")"));
    assert!(a["terms"][0].get("resolved").is_none(), "{}", a["terms"][0]);
    // GGG's two spellings, one of them a row of twice alone
    let a = asked(&s, &format!("line(\"{LIFE}\")"));
    assert_eq!(
        a["terms"][0]["resolved"],
        json!({ "of": "template", "values": [
            { "value": "# to maximum Life", "items": 5 },
            { "value": "# to Maximum life", "items": 1 },
        ], "more": 0 })
    );
    let a = asked(&s, "line(template:resistances)");
    assert_eq!(
        a["terms"][0]["resolved"],
        json!({ "of": "template", "values": [
            { "value": ALL_RES, "items": 1 },
            { "value": THREAD, "items": 1 },
        ], "more": 0 })
    );
    let a = asked(&s, "line(\"#% to all Elemental Resistance\")");
    assert_eq!(a["total"]["matched"], 0);
    let suggestions = a["zero"]["resolved_to_nothing"][0]["suggestions"]
        .as_array()
        .unwrap();
    assert!(!suggestions.is_empty());
    for suggested in suggestions {
        let term = suggested["term"].as_str().unwrap();
        assert_eq!(
            asked(&s, term)["total"]["matched"],
            suggested["items"],
            "{suggested}"
        );
    }
    let of = |value: &str| {
        suggestions
            .iter()
            .find(|s| s["value"] == value)
            .unwrap_or_else(|| panic!("no suggestion `{value}` in {suggestions:?}"))
    };
    assert_eq!(of(ALL_RES)["items"], 2);
    assert_eq!(of(THREAD)["items"], 1);
}

/// `show` prints a mod of several rows with each row as the search reads
/// it: its template and its numbers.
#[test]
fn show_gives_a_mod_its_rows() {
    let s = stash();
    let shown = serde_json::to_value(show(&s, "pair", false).unwrap()).unwrap();
    assert_eq!(
        shown["lines"][0]["rows"],
        json!([
            { "template": "Adds # to # Cold Damage", "numbers": [3, 9] },
            { "template": "# to maximum Life", "numbers": [40] },
        ])
    );
    assert_eq!(
        shown["lines"][0]["template"],
        "Adds # to # Cold Damage\n# to maximum Life"
    );
    assert_eq!(shown["lines"][1].get("rows"), None);
}

/// C100 (the caller may name fields) and C103 (one deriver per fact): a
/// row asked for `item` carries what `show` carries of the same id at the
/// same basis — the item, its class, its category, its price and its
/// lines with their slots — made by `show`'s one maker. Over `stash()`'s
/// mods of several rows, beside an item priced by a price written to the
/// intent file, an item whose base no class table holds, and, under
/// membership `all`, an item that left; one intent file and one `now`
/// for both. A row's `place` is its own (the corpus names a socketed
/// item's parent), and is not compared.
#[test]
fn c100_a_rows_item_is_the_one_show_makes() {
    use acquisition_plan::price::{Buyout, PriceTarget, set_buyout};
    use acquisition_search::answer::Membership;
    let mut s = stash();
    list_tabs(
        &mut s,
        "pc",
        "Standard",
        json!([tab("w1", "Rows"), tab("w2", "More")]),
        30,
    );
    let more = |gone: bool| {
        let mut items = vec![
            item(
                "priced",
                "Priced Loop",
                "Iron Ring",
                "Rare",
                json!({ "explicitMods": ["+40 to maximum Life"] }),
            ),
            item("odd", "Odd Thing", "Nonesuch Base", "Rare", json!({})),
        ];
        if gone {
            items.push(item("gone", "Gone Loop", "Iron Ring", "Rare", json!({})));
        }
        items
    };
    fetch_tab(&mut s, "pc", "Standard", "w2", "More", more(true), 40);
    fetch_tab(&mut s, "pc", "Standard", "w2", "More", more(false), 50);
    let mut intent = intent(&s);
    let target: PriceTarget = "item/priced".parse().unwrap();
    set_buyout(
        &mut intent,
        &target,
        &Buyout::Exact(acquisition_plan::price::Price {
            amount: "5".parse().unwrap(),
            currency: "chaos".into(),
        }),
        None,
        &acquisition_store::Provenance::via("test"),
    )
    .unwrap();
    let corpus = acquisition_search::Corpus::load_of(
        &s,
        &intent,
        Some(&acquisition_search::Realm::parse("pc").unwrap()),
        Membership::All,
    )
    .unwrap();
    let request: Request = serde_json::from_value(json!({
        "scope": { "membership": "all" },
        "view": { "rows": { "fields": ["icon", "item"], "limit": 100 } },
    }))
    .unwrap();
    let now = acquisition_store::now();
    let a = serde_json::to_value(answer(&corpus, &request).unwrap().aged(now)).unwrap();
    let rows = a["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 10, "{a}");
    let mut seen = BTreeSet::new();
    for row in rows {
        let id = row["id"].as_str().unwrap();
        let shown = acquisition_search::show(&s, &intent, id, false)
            .unwrap()
            .aged(now);
        let shown = serde_json::to_value(shown).unwrap();
        assert_eq!(a["basis"], shown["basis"], "{id}");
        for key in ["item", "class", "category", "price", "lines"] {
            assert_eq!(row[key], shown[key], "{id}: {key}");
        }
        seen.insert(id.to_string());
    }
    // the fixture's cases are among the rows compared
    let row = |id: &str| rows.iter().find(|r| r["id"] == id).unwrap();
    assert_eq!(
        row("priced")["price"]["text"],
        "5 chaos",
        "{}",
        row("priced")
    );
    assert!(row("odd")["class"].get("open").is_some(), "{}", row("odd"));
    assert!(
        row("gone")["item"]["facts"]["removed_at"].is_i64(),
        "{}",
        row("gone")
    );
    assert_eq!(
        row("pair")["lines"][0]["rows"].as_array().map(Vec::len),
        Some(2)
    );
    assert!(seen.contains("thread"), "{seen:?}");
}

/// C53's line of 2026-11-08: under `item` a rows answer holds at most the
/// item rows' bound whatever its `limit` names, says the limit it applied
/// and the one asked,
/// and continues the rest at it, asking the same fields; every row it
/// holds carries its item whole. Without `item` the limit asked stands.
#[test]
fn an_item_row_view_holds_at_most_the_cap_and_continues() {
    use acquisition_store::bound::ITEM_ROWS;
    let mut s = store();
    list_tabs(&mut s, "pc", "Standard", json!([tab("w1", "Many")]), 10);
    let items = (0..=ITEM_ROWS)
        .map(|n| {
            item(
                &format!("i{n}"),
                &format!("Item {n}"),
                "Iron Ring",
                "Rare",
                json!({}),
            )
        })
        .collect();
    fetch_tab(&mut s, "pc", "Standard", "w1", "Many", items, 20);
    let past = ITEM_ROWS + 50;
    let carried = view(
        &s,
        "",
        json!({ "rows": { "fields": ["item"], "limit": past } }),
    );
    let rows = carried["rows"].as_array().unwrap();
    assert_eq!(rows.len(), ITEM_ROWS);
    assert!(
        rows.iter()
            .all(|r| r["lines"].is_array() && r["item"].is_object())
    );
    let said = &carried["view"]["rows"];
    assert_eq!(said["limit"], ITEM_ROWS, "{said}");
    assert_eq!(said["limit_asked"], past, "{said}");
    assert_eq!(said["left_out"], 1, "{said}");
    let next = &said["next"]["request"];
    assert_eq!(next["view"]["rows"]["limit"], ITEM_ROWS, "{next}");
    assert_eq!(next["view"]["rows"]["fields"], json!(["item"]), "{next}");
    let request: Request = serde_json::from_value(next.clone()).unwrap();
    let after = as_json(&answer(&load(&s, Some("pc")), &request).unwrap());
    let last = after["rows"].as_array().unwrap();
    assert_eq!(last.len(), 1, "{after}");
    assert!(last[0]["item"].is_object(), "{after}");
    // a smaller limit stands as asked; without `item`, every row asked
    let small = view(
        &s,
        "",
        json!({ "rows": { "fields": ["item"], "limit": 3 } }),
    );
    assert_eq!(small["rows"].as_array().unwrap().len(), 3);
    let compact = view(
        &s,
        "",
        json!({ "rows": { "fields": ["icon"], "limit": past } }),
    );
    assert_eq!(compact["rows"].as_array().unwrap().len(), ITEM_ROWS + 1);
    assert_eq!(compact["view"]["rows"]["limit"], past);
    // what was asked is said only where it was lowered: not where a limit
    // stands, nor on the continuation, which asks the applied limit
    for stood in [&small, &compact, &after] {
        assert!(
            stood["view"]["rows"].get("limit_asked").is_none(),
            "{stood}"
        );
    }
}

/// C140 and C100's named fields: a rows view asked for `fields: ["icon"]`
/// puts on each row the URL of the item's art its body gives — the one
/// `show` prints, from the one maker — and null where the body gives
/// none; unasked, a row carries none; a continuation goes on asking it;
/// a field no view knows is refused.
#[test]
fn c140_a_row_carries_the_icon_when_the_view_names_it() {
    const ART: &str = "https://web.poecdn.com/gen/image/ring.png";
    let mut s = store();
    list_tabs(&mut s, "pc", "Standard", json!([tab("w1", "Art")]), 10);
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "w1",
        "Art",
        vec![
            item(
                "drawn",
                "Drawn",
                "Iron Ring",
                "Rare",
                json!({ "icon": ART }),
            ),
            item("bare", "Bare", "Iron Ring", "Rare", json!({})),
        ],
        20,
    );
    let rows = |v: &Value| v["rows"].as_array().unwrap().clone();
    let asked = view(
        &s,
        "class:ring",
        json!({ "rows": { "fields": ["icon"], "sort": "ilvl" } }),
    );
    let by_id = |id: &str| {
        rows(&asked)
            .into_iter()
            .find(|r| r["id"] == id)
            .unwrap_or_else(|| panic!("{id}: {asked}"))
    };
    assert_eq!(by_id("drawn")["icon"], ART);
    assert_eq!(by_id("bare")["icon"], Value::Null);
    assert!(by_id("bare").as_object().unwrap().contains_key("icon"));
    // the one maker: show prints the same URL
    let shown = serde_json::to_value(show(&s, "drawn", false).unwrap()).unwrap();
    assert_eq!(shown["item"]["icon"], ART);
    // the view as it ran says the fields it asked
    assert_eq!(asked["view"]["rows"]["fields"], json!(["icon"]), "{asked}");
    // unasked, a row is compact, and the view names none
    let compact = view(&s, "class:ring", json!({ "rows": {} }));
    assert!(
        rows(&compact).iter().all(|r| r.get("icon").is_none()),
        "{compact}"
    );
    assert!(compact["view"]["rows"].get("fields").is_none(), "{compact}");
    // a continuation asks what its answer asked
    let first = view(
        &s,
        "class:ring",
        json!({ "rows": { "fields": ["icon"], "limit": 1 } }),
    );
    let next = &first["view"]["rows"]["next"]["request"];
    assert_eq!(next["view"]["rows"]["fields"], json!(["icon"]), "{first}");
    let request: Request = serde_json::from_value(next.clone()).unwrap();
    let after = as_json(&answer(&load(&s, Some("pc")), &request).unwrap());
    assert!(
        rows(&after)[0].as_object().unwrap().contains_key("icon"),
        "{after}"
    );
    // a field no view knows is refused
    let unknown =
        serde_json::from_value::<Request>(json!({ "view": { "rows": { "fields": ["price"] } } }));
    assert!(unknown.is_err());
    // and by the frontends' reader of a typed list, naming both fields;
    // a word named twice is one
    use acquisition_search::answer::RowField;
    let e = RowField::list("icon,price").unwrap_err();
    assert_eq!(e.kind, ErrorKind::UnknownName, "{}", e.message);
    assert_eq!(
        e.message,
        "`price` is no field a row carries: `icon`, the URL of the item's art, or `item`, the item as `show` gives it"
    );
    assert_eq!(
        RowField::list("item, ICON,item,").unwrap(),
        [RowField::Item, RowField::Icon]
    );
    // a list that names no field is refused, never read as none
    for empty in ["", " ", ",", " , "] {
        let e = RowField::list(empty).unwrap_err();
        assert_eq!(e.kind, ErrorKind::UnknownName, "{empty:?}");
        assert_eq!(
            e.message,
            "`fields` names no field: `icon`, the URL of the item's art, or `item`, the item as `show` gives it"
        );
    }
}

// ---- values on rows (4-full part 1, step 6.1; #38) ----------------------

/// The values a query compares, in its order, each once, and the one
/// that orders the rows where no sort is named (`values.rs`): a row each,
/// the query, the values the answer says it used, and its order — the
/// value and largest first, or none for the store's.
#[test]
fn the_values_a_query_compares_and_the_one_that_orders_it() {
    let corpus = load(&worked(), Some("pc"));
    let life = format!("line(\"{LIFE}\").arg1");
    let explicit = format!("line(\"{LIFE}\" source=explicit).arg1");
    let summed = format!("sum(line(\"{LIFE}\").arg1)");
    // the query, the values said, and the order: the value and whether
    // largest first
    type Row<'a> = (String, Vec<&'a str>, Option<(&'a str, bool)>);
    let table: Vec<Row> = vec![
        // a number field, best first by its operator
        ("ilvl>=80".into(), vec!["ilvl"], Some(("ilvl", true))),
        ("ilvl>80".into(), vec!["ilvl"], Some(("ilvl", true))),
        ("ilvl<=80".into(), vec!["ilvl"], Some(("ilvl", false))),
        ("ilvl<80".into(), vec!["ilvl"], Some(("ilvl", false))),
        // equality, a range and a value compared both ways: no direction
        ("ilvl=80".into(), vec!["ilvl"], None),
        ("ilvl=80..84".into(), vec!["ilvl"], None),
        ("ilvl>=80 ilvl<=84".into(), vec!["ilvl"], None),
        // a duplicate is one value, its direction kept
        (
            "ilvl>=80 ilvl>=82".into(),
            vec!["ilvl"],
            Some(("ilvl", true)),
        ),
        // under a not, the column and no direction; two nots cancel
        ("-ilvl>=80".into(), vec!["ilvl"], None),
        ("-(-ilvl>=80)".into(), vec!["ilvl"], Some(("ilvl", true))),
        // a computed value and a sum
        (
            "pseudo.total_res>=80".into(),
            vec!["pseudo.total_res"],
            Some(("pseudo.total_res", true)),
        ),
        (
            format!("sum(\"{LIFE}\")<=90"),
            vec![&summed],
            Some((&summed, false)),
        ),
        // a line's slot: the occurrence, under the group's selector
        (format!("\"{LIFE}\">=90"), vec![&life], Some((&life, true))),
        (
            format!("line(\"{LIFE}\" source=explicit arg1>=90)"),
            vec![&explicit],
            Some((&explicit, true)),
        ),
        (format!("line(\"{LIFE}\" -arg1>=90)"), vec![&life], None),
        // several: columns, and the store's order
        (
            "ilvl>=80 or pseudo.total_res>=10".into(),
            vec!["ilvl", "pseudo.total_res"],
            None,
        ),
        (
            format!("pseudo.total_res>=10 \"{LIFE}\">=90"),
            vec!["pseudo.total_res", &life],
            None,
        ),
        // what compares no value
        ("linked(size>=4)".into(), vec![], None),
        ("undecided(ilvl)".into(), vec![], None),
        ("rarity=rare has:ilvl -is:corrupted".into(), vec![], None),
        ("".into(), vec![], None),
    ];
    for (query, values, order) in table {
        let request: Request =
            serde_json::from_value(json!({ "query": { "text": query } })).unwrap();
        let a = as_json(&answer(&corpus, &request).unwrap());
        let rows = &a["view"]["rows"];
        let said: Vec<&str> = rows["values"]
            .as_array()
            .map(|v| v.iter().map(|t| t.as_str().unwrap()).collect())
            .unwrap_or_default();
        assert_eq!(said, values, "`{query}`: {rows}");
        assert_eq!(
            rows["values_from_query"].as_bool().unwrap_or(false),
            !values.is_empty(),
            "`{query}`: {rows}"
        );
        match order {
            Some((sort, desc)) => {
                assert_eq!(rows["sort"], sort, "`{query}`: {rows}");
                assert_eq!(
                    rows["desc"].as_bool().unwrap_or(false),
                    desc,
                    "`{query}`: {rows}"
                );
                assert_eq!(rows["sort_from_query"], true, "`{query}`: {rows}");
            }
            None => {
                assert!(rows.get("sort").is_none(), "`{query}`: {rows}");
                assert!(rows.get("sort_from_query").is_none(), "`{query}`: {rows}");
            }
        }
        for row in a["rows"].as_array().unwrap() {
            let carried = row
                .get("values")
                .and_then(Value::as_array)
                .map_or(0, Vec::len);
            assert_eq!(carried, values.len(), "`{query}`: {row}");
        }
    }
}

/// #38, the re-sit's case: a query that filters on the explicit sum of
/// rarity is ordered by that sum, largest first, unnamed — not by the sum
/// over every source, which put a 41 explicit with 16 implicit above a 51
/// — and the answer says the value it ordered by came from the query.
#[test]
fn issue_38_a_filtered_sum_orders_the_rows_unnamed() {
    const RARITY: &str = "#% increased Rarity of Items found";
    let mut s = store();
    list_tabs(&mut s, "pc", "Standard", json!([tab("w1", "Rows")]), 10);
    let amulet = |id: &str, explicit: i64, implicit: Option<i64>| {
        let mut more =
            json!({ "explicitMods": [format!("{explicit}% increased Rarity of Items found")] });
        if let Some(implicit) = implicit {
            more["implicitMods"] = json!([format!("{implicit}% increased Rarity of Items found")]);
        }
        item(id, &format!("Amulet {id}"), "Gold Amulet", "Rare", more)
    };
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "w1",
        "Rows",
        vec![
            amulet("ventor", 32, Some(11)),
            amulet("torc", 41, Some(16)),
            amulet("reaper", 38, None),
            amulet("gyre", 51, None),
            amulet("low", 20, Some(30)),
        ],
        20,
    );
    let corpus = load(&s, Some("pc"));
    let explicit = format!("sum(line(\"{RARITY}\" source=explicit).arg1)");
    let request: Request = serde_json::from_value(json!({
        "query": { "text": format!("{explicit}>=30") },
    }))
    .unwrap();
    let a = as_json(&answer(&corpus, &request).unwrap());
    assert_eq!(
        ids_in_order(&a),
        ["gyre", "torc", "reaper", "ventor"],
        "{a}"
    );
    let rows = &a["view"]["rows"];
    assert_eq!(rows["sort"], explicit.as_str());
    assert_eq!(rows["desc"], true);
    assert_eq!(rows["sort_from_query"], true);
    assert_eq!(rows["values"], json!([explicit]));
    let first: Vec<&Value> = a["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| &sort_value(r)["value"])
        .collect();
    assert_eq!(first, [&json!(51), &json!(41), &json!(38), &json!(32)]);
    // `none` declines the order, and `desc` beside it is refused
    let unordered: Request = serde_json::from_value(json!({
        "query": { "text": format!("{explicit}>=30") },
        "view": { "rows": { "sort": "None" } },
    }))
    .unwrap();
    let b = as_json(&answer(&corpus, &unordered).unwrap());
    // the store's order: by id here
    assert_eq!(
        ids_in_order(&b),
        ["gyre", "reaper", "torc", "ventor"],
        "{b}"
    );
    assert_eq!(b["view"]["rows"]["sort"], "none");
    assert!(b["view"]["rows"].get("sort_from_query").is_none());
    assert_eq!(b["view"]["rows"]["values"], json!([explicit]));
    let refused: Request = serde_json::from_value(json!({
        "query": { "text": format!("{explicit}>=30") },
        "view": { "rows": { "sort": "none", "desc": true } },
    }))
    .unwrap();
    assert_eq!(
        answer(&corpus, &refused).unwrap_err().to_json()["kind"],
        "view"
    );
}

/// Values named: each row carries them in the order named, each once; a
/// sort the list lacks joins it at the end; the query's own order still
/// stands, and more than the bound is refused naming it.
#[test]
fn values_named_are_carried_in_order_and_a_sort_joins_them() {
    let corpus = load(&worked(), Some("pc"));
    let asked = |view: Value| {
        let request: Request =
            serde_json::from_value(json!({ "query": { "text": "ilvl>=1" }, "view": view }))
                .unwrap();
        answer(&corpus, &request).map(|a| as_json(&a))
    };
    let a = asked(json!({ "rows": { "values": ["pseudo.total_res", "ilvl", "ilvl"] } })).unwrap();
    let rows = &a["view"]["rows"];
    // named, so not the query's; the query's one value orders them still
    assert_eq!(
        rows["values"],
        json!(["pseudo.total_res", "ilvl"]),
        "{rows}"
    );
    assert!(rows.get("values_from_query").is_none(), "{rows}");
    assert_eq!(rows["sort"], "ilvl");
    assert_eq!(rows["sort_from_query"], true);
    for row in a["rows"].as_array().unwrap() {
        assert_eq!(row["values"].as_array().unwrap().len(), 2, "{row}");
    }
    let b = asked(json!({ "rows": { "values": ["pseudo.total_res"], "sort": "stack" } })).unwrap();
    assert_eq!(
        b["view"]["rows"]["values"],
        json!(["pseudo.total_res", "stack"])
    );
    assert!(b["view"]["rows"].get("sort_from_query").is_none());
    // an adapter's flat arguments: values beside a count or explain are
    // refused, as a sort is
    use acquisition_search::answer::{Rows, Shape};
    let valued = || Rows {
        values: vec!["ilvl".into()],
        ..Rows::default()
    };
    let count = Shape::of(Some(vec!["rarity".into()]), None, None, None, valued());
    assert_eq!(count.unwrap_err().kind, ErrorKind::View);
    let explain = Shape::of(None, None, None, Some("0".into()), valued());
    assert_eq!(explain.unwrap_err().kind, ErrorKind::View);
}

/// The ids of an answer's rows, in its order.
fn ids_in_order(a: &Value) -> Vec<String> {
    a["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["id"].as_str().unwrap().to_string())
        .collect()
}

/// The bound on a row's values, held once on the list a row carries
/// (the cold reviews at 1ab468eb and 849ccbab, each of which found it
/// missed on another path): every way a list is filled — named alone,
/// the query's alone, either with a named sort or the query's — and the
/// continuation of each, which fills it again the same. Named past
/// twelve, the sort among them, is refused; the query's is cut from its
/// end and counted, the sort kept.
#[test]
fn a_rows_values_are_twelve_at_most_however_the_list_is_filled() {
    let corpus = load(&worked(), Some("pc"));
    let sums: Vec<String> = ('a'..='m')
        .map(|word| format!("sum(line(\"# to maximum {word}\").arg1)"))
        .collect();
    let compared = |n: usize| {
        sums[..n]
            .iter()
            .map(|sum| format!("{sum}>=0"))
            .collect::<Vec<_>>()
            .join(" ")
    };
    // the query, the rows view, and what is carried: refused, or the
    // number of values, the last one where it is the sort, and how many of
    // the query's were left out
    type Case<'a> = (
        &'a str,
        String,
        Value,
        Option<(usize, Option<&'a str>, u64)>,
    );
    let one = "ilvl>=1".to_string();
    let none = String::new();
    let cases: Vec<Case> = vec![
        (
            "named alone",
            none.clone(),
            json!({ "values": sums[..12] }),
            Some((12, None, 0)),
        ),
        (
            "named alone, past it",
            none.clone(),
            json!({ "values": sums[..13] }),
            None,
        ),
        (
            "the query's alone",
            compared(12),
            json!({}),
            Some((12, None, 0)),
        ),
        (
            "the query's alone, past it",
            compared(13),
            json!({}),
            Some((12, None, 1)),
        ),
        (
            "the query's one, its own order",
            one.clone(),
            json!({}),
            Some((1, Some("ilvl"), 0)),
        ),
        (
            "named and a named sort among them",
            none.clone(),
            json!({ "values": sums[..12], "sort": sums[0] }),
            Some((12, None, 0)),
        ),
        (
            "named and a named sort outside them",
            none.clone(),
            json!({ "values": sums[..11], "sort": "ilvl" }),
            Some((12, Some("ilvl"), 0)),
        ),
        (
            "named and a named sort outside them, past it",
            none.clone(),
            json!({ "values": sums[..12], "sort": "ilvl" }),
            None,
        ),
        (
            "named and the query's sort",
            one.clone(),
            json!({ "values": sums[..11] }),
            Some((12, Some("ilvl"), 0)),
        ),
        (
            "named and the query's sort, past it",
            one.clone(),
            json!({ "values": sums[..12] }),
            None,
        ),
        (
            "named, the query's sort declined",
            one.clone(),
            json!({ "values": sums[..12], "sort": "none" }),
            Some((12, None, 0)),
        ),
        (
            "the query's and a named sort",
            compared(12),
            json!({ "sort": "ilvl" }),
            Some((12, Some("ilvl"), 1)),
        ),
        (
            "the query's past it and a named sort",
            compared(13),
            json!({ "sort": "ilvl" }),
            Some((12, Some("ilvl"), 2)),
        ),
    ];
    for (case, query, rows, expected) in cases {
        let ask = |rows: Value| -> Result<Value, Value> {
            let request: Request = serde_json::from_value(
                json!({ "query": { "text": query }, "view": { "rows": rows } }),
            )
            .unwrap();
            request.check().map_err(|e| json!({ "check": e.kind }))?;
            answer(&corpus, &request)
                .map(|a| as_json(&a))
                .map_err(|e| e.to_json())
        };
        let mut first = rows.clone();
        first["limit"] = json!(2);
        let a = ask(first);
        let Some((len, last, left_out)) = expected else {
            let refused = a.unwrap_err();
            assert_eq!(
                refused,
                json!({ "check": ErrorKind::View }),
                "{case}: refused as a view before a store opens"
            );
            continue;
        };
        let a = a.unwrap_or_else(|e| panic!("{case}: {e}"));
        let said = &a["view"]["rows"];
        let values = said["values"].as_array().unwrap();
        assert_eq!(values.len(), len, "{case}: {said}");
        if let Some(last) = last {
            assert_eq!(values[len - 1], last, "{case}: {said}");
        }
        assert_eq!(
            said["values_left_out"].as_u64().unwrap_or(0),
            left_out,
            "{case}: {said}"
        );
        for row in a["rows"].as_array().unwrap() {
            assert_eq!(
                row["values"].as_array().unwrap().len(),
                len,
                "{case}: {row}"
            );
        }
        // the continuation fills the list again, the same
        let next: Request = serde_json::from_value(said["next"]["request"].clone())
            .unwrap_or_else(|e| panic!("{case}: no continuation: {e} {said}"));
        let b = as_json(&answer(&corpus, &next).unwrap());
        for key in [
            "values",
            "values_left_out",
            "sort",
            "desc",
            "values_from_query",
            "sort_from_query",
        ] {
            assert_eq!(b["view"]["rows"].get(key), said.get(key), "{case}: {key}");
        }
    }
}

/// Items whose displayed rows carry number fields (4-full part 1, step
/// 6.4c), beside this file's mods of several rows: `armour`, a body armour
/// of item level 84 displaying quality and armour, requiring level 64 and
/// 16 Str, two life lines of one source (30 and 40) and a ranged physical
/// line (10 to 20), and a line the site's text writes a digit of (15%
/// Phasing for 4 seconds, the review at 4a1e21c3, its first finding);
/// `gem`, a gem at level 20 of its maximum, requiring level 70; `stack`, a
/// currency stack of 3 of 10; `blueprint`, 1 of 3 wings revealed (its
/// second finding); `kit`, three lines the fix round's review named — a
/// `+` the site writes before a `#`, a digit its text writes as another
/// number, a plural form the bound sums with the text.
fn displayed() -> Store {
    let mut s = stash();
    list_tabs(
        &mut s,
        "pc",
        "Standard",
        json!([tab("w1", "Rows"), tab("w2", "Shown")]),
        30,
    );
    let property = |name: &str, value: &str, mode: i64| json!({ "name": name, "values": [[value, 0]], "displayMode": mode });
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "w2",
        "Shown",
        vec![
            item(
                "armour",
                "Shown Coat",
                "Glorious Plate",
                "Rare",
                json!({
                    "properties": [property("Quality", "+20%", 0), property("Armour", "553", 0)],
                    "requirements": [property("Level", "64", 0), property("Str", "16", 1)],
                    "explicitMods": [
                        "+30 to maximum Life",
                        "+40 to maximum Life",
                        "Adds 10 to 20 Physical Damage",
                        "15% chance to gain Phasing for 4 seconds on Kill",
                    ],
                }),
            ),
            item(
                "gem",
                "",
                "Vaal Grace",
                "Gem",
                json!({
                    "properties": [property("Level", "20 (Max)", 0)],
                    "requirements": [property("Level", "70", 0)],
                }),
            ),
            item(
                "stack",
                "",
                "Chaos Orb",
                "Currency",
                json!({ "stackSize": 3, "properties": [property("Stack Size", "3/10", 0)] }),
            ),
            item(
                "kit",
                "Kit Belt",
                "Leather Belt",
                "Rare",
                json!({ "explicitMods": [
                    "+5 Life per 4 Dexterity",
                    "15% chance to gain Phasing for 6 seconds on Kill",
                    "Drops Burning Ground while moving, dealing 2500 Fire Damage per second for 4 seconds",
                ] }),
            ),
            item(
                "blueprint",
                "",
                "Blueprint: Tunnels",
                "Normal",
                json!({ "properties": [property("Wings Revealed", "1/3", 0)] }),
            ),
        ],
        40,
    );
    s
}

/// 4-full part 1, step 6.4c (Decision 5): every value a shown item
/// carries — a line's `value`, each of `number_fields` — binds as a sort
/// and, asked of that item, is the number it stands beside: a field's,
/// or the line's slot of that name summed over every occurrence its group
/// selects, a mod of several rows by the row it names, and `avg` where
/// the template's numbers are a mean pair. Each displayed row's pieces
/// join to the row's own text, the number fields' pieces carrying their
/// fields' texts.
#[test]
fn every_value_a_shown_item_carries_is_the_number_it_stands_beside() {
    let s = displayed();
    let corpus = load(&s, Some("pc"));
    let every = ids(&asked(&s, ""));
    assert!(every.len() >= 10, "{every:?}");
    let mut lines_valued = 0;
    for id in &every {
        let shown = show(&s, id, false).unwrap();
        let mut texts: Vec<(String, f64)> = Vec::new();
        for l in &shown.lines {
            // none where the slot a bound compares reads no number: a line
            // with no number, or one whose number is unread
            let Some(value) = &l.value else {
                assert!(
                    l.slots.is_empty() || l.slots.iter().any(|(_, n)| n.is_null()),
                    "{id}: a line whose numbers are read has no value: {}",
                    l.line.text
                );
                continue;
            };
            lines_valued += 1;
            let slot = value
                .strip_suffix(')')
                .and_then(|v| v.rsplit_once('.'))
                .map(|(_, slot)| slot.to_string())
                .unwrap();
            // the slot of that name, summed over the occurrences the
            // line's group selects: by its whole template, or by a row
            let number = |slot: acquisition_search::derive::Slot| match slot {
                acquisition_search::derive::Slot::Is(n) => n,
                other => panic!("{id}: {value} reads {other:?}"),
            };
            let mut sum = 0.0;
            for other in shown
                .lines
                .iter()
                .filter(|o| o.line.source == l.line.source)
            {
                if other.line.template == l.line.template {
                    sum += number(other.line.whole().slot(&slot));
                }
                for row in other
                    .line
                    .rows
                    .iter()
                    .filter(|r| r.template == l.line.template)
                {
                    sum += number(row.read().slot(&slot));
                }
            }
            texts.push((value.clone(), sum));
        }
        for (field, number) in &shown.number_fields {
            texts.push((field.clone(), number.as_f64().unwrap()));
        }
        // two lines of one group carry one value, which a list names once
        let mut seen = BTreeSet::new();
        texts.retain(|(text, _)| seen.insert(text.clone()));
        for chunk in texts.chunks(acquisition_search::answer::VALUES) {
            let values: Vec<&String> = chunk.iter().map(|(text, _)| text).collect();
            let request: Request = serde_json::from_value(json!({
                "query": { "text": format!("id:{id}") },
                "view": { "rows": { "values": values, "sort": values[0] } },
            }))
            .unwrap();
            let answered = as_json(&answer(&corpus, &request).unwrap());
            let row = &answered["rows"][0];
            for (n, (text, number)) in chunk.iter().enumerate() {
                let valued = &row["values"][n];
                assert!(valued["status"].is_null(), "{id} {text}: {valued}");
                let got = valued["value"].as_f64().unwrap();
                assert!(
                    (got - number).abs() < 1e-9,
                    "{id} {text}: {got} for {number}"
                );
            }
        }
        // each displayed row is its pieces joined, a field's piece its text
        let displays = shown.displays();
        let joined = |parts: &[acquisition_search::show::Part]| {
            parts.iter().map(|p| p.text.as_str()).collect::<String>()
        };
        for (at, property) in shown.item.properties.iter().enumerate() {
            assert_eq!(joined(&displays.property_row(at)), property.text);
        }
        if let Some(level) = &shown.item.item_level {
            assert_eq!(&joined(&displays.item_level_row()), level);
        }
        if let Some(requires) = &shown.item.requires {
            assert_eq!(&joined(&displays.requires_row()), requires);
        }
    }
    assert!(lines_valued >= 10, "{lines_valued}");
    // the coat: its displayed numbers, each its field, in the item's order
    let coat = show(&s, "armour", false).unwrap();
    assert_eq!(
        serde_json::to_value(&coat.number_fields).unwrap(),
        json!({ "quality": 20, "armour": 553, "ilvl": 84, "reqlevel": 64, "str": 16 })
    );
    let pieces = |parts: Vec<acquisition_search::show::Part>| {
        parts
            .into_iter()
            .map(|p| (p.text, p.value))
            .collect::<Vec<_>>()
    };
    let some = |s: &str| Some(s.to_string());
    assert_eq!(
        pieces(coat.displays().requires_row()),
        [
            ("Requires ".to_string(), None),
            ("Level".to_string(), None),
            (" ".to_string(), None),
            ("64".to_string(), some("reqlevel")),
            (", ".to_string(), None),
            ("16".to_string(), some("str")),
            (" ".to_string(), None),
            ("Str".to_string(), None),
        ]
    );
    assert_eq!(
        pieces(coat.displays().property_row(1)),
        [
            ("Armour".to_string(), None),
            (": ".to_string(), None),
            ("553".to_string(), some("armour")),
        ]
    );
    let value = |id: &str, text: &str| {
        let shown = show(&s, id, false).unwrap();
        let line = shown.lines.iter().find(|l| l.line.text == text).unwrap();
        line.value.clone().unwrap()
    };
    assert_eq!(
        value("armour", "Adds 10 to 20 Physical Damage"),
        r##"sum(line("Adds # to # Physical Damage" source=explicit).avg)"##
    );
    assert_eq!(
        value("armour", "+30 to maximum Life"),
        r##"sum(line("# to maximum Life" source=explicit).arg1)"##
    );
    // a gem's level is read beside its mark, `20 (Max)`; a stack's
    // `Stack Size: 3/10` is no property the table reads (`stack` is the
    // body's own count, displayed nowhere alone), so it links nothing
    let gem = show(&s, "gem", false).unwrap();
    assert_eq!(
        serde_json::to_value(&gem.number_fields).unwrap(),
        json!({ "level": 20, "ilvl": 84, "reqlevel": 70 }),
    );
    let stack = show(&s, "stack", false).unwrap();
    assert_eq!(
        serde_json::to_value(&stack.number_fields).unwrap(),
        json!({ "ilvl": 84 }),
    );
    // an `a/b` both of whose sides a field reads: two numbers, each its
    // own piece and its own field (the review at 4a1e21c3, its second
    // finding)
    let blueprint = show(&s, "blueprint", false).unwrap();
    assert_eq!(
        serde_json::to_value(&blueprint.number_fields).unwrap(),
        json!({ "wings_revealed": 1, "wings_revealed.of": 3, "ilvl": 84 }),
    );
    assert_eq!(
        pieces(blueprint.displays().property_row(0)),
        [
            ("Wings Revealed".to_string(), None),
            (": ".to_string(), None),
            ("1".to_string(), some("wings_revealed")),
            ("/".to_string(), None),
            ("3".to_string(), some("wings_revealed.of")),
        ]
    );
}

/// Step 6.4c: a trade filter on a stat and that stat's line on an item
/// name one value, so a click on a stat the form filtered lands on its
/// column and adds no twin: life, and a text the site writes a digit of,
/// whose written number the filter does not compare (the review at
/// 4a1e21c3, its first finding: the line's two numbers are no mean pair
/// to the site); and the three the fix round's review at 9fe51220 found
/// the stat's line found too narrowly: a `+` before the site's `#`, the
/// site's digit where the item shows another number, a plural form the
/// bound sums with the text — each a line the stat's group selects.
#[test]
fn a_trade_filter_on_a_stat_and_its_line_on_an_item_name_one_value() {
    let s = displayed();
    for (stat, id, text) in [
        ("explicit.stat_3299347043", "plain", "+95 to maximum Life"),
        (
            "explicit.stat_2918708827",
            "armour",
            "15% chance to gain Phasing for 4 seconds on Kill",
        ),
        ("explicit.stat_2042405614", "kit", "+5 Life per 4 Dexterity"),
        (
            "explicit.stat_2918708827",
            "kit",
            "15% chance to gain Phasing for 6 seconds on Kill",
        ),
        (
            "explicit.stat_3595254837",
            "kit",
            "Drops Burning Ground while moving, dealing 2500 Fire Damage per second for 4 seconds",
        ),
    ] {
        let request: Request = serde_json::from_value(json!({ "query": { "trade": { "query": {
            "stats": [{ "type": "and", "filters": [
                { "id": stat, "value": { "min": 10 } },
            ] }] } } } }))
        .unwrap();
        let answered = as_json(&answer(&load(&s, Some("pc")), &request).unwrap());
        let values = &answered["view"]["rows"]["values"];
        let shown = show(&s, id, false).unwrap();
        let line = shown.lines.iter().find(|l| l.line.text == text).unwrap();
        assert_eq!(values, &json!([line.value.clone().unwrap()]), "{answered}");
    }
}
