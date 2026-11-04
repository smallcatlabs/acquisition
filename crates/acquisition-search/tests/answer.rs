//! The crate's boundary (rule 3 of the plan, the crate's `lib.rs`): a request in, an
//! answer out, as JSON. The worked example whole (`answer.rs`), every count
//! as printed there and worked by hand; the route property; invariants 2
//! and 6 of the surface; C93's composition; C92's sort scalar; C96's
//! scope; C98's basis at the answer's boundary.

use crate::common;

use acquisition_search::answer::Membership;
use acquisition_search::{Corpus, Realm};
use acquisition_store::Store;
use common::answer;
use common::*;
use serde_json::{Value, json};

const LIFE: &str = "+# to maximum Life";

fn counts(term: &Value) -> [u64; 4] {
    ["matched", "failed", "lacked", "undecided"].map(|k| term[k]["count"].as_u64().unwrap())
}

/// Run a count's route as the request it is.
fn follow(corpus: &Corpus, count: &Value) -> Vec<String> {
    let request = serde_json::from_value(count["request"].clone()).unwrap();
    let routed = as_json(&answer(corpus, &request).unwrap());
    assert_eq!(
        routed["total"]["matched"], count["count"],
        "{}",
        count["request"]
    );
    assert_eq!(count["denominator"], "scope");
    ids(&routed)
}

#[test]
fn c100_the_worked_example_whole_counts_as_printed_there() {
    let s = worked();
    let corpus = load(&s, Some("pc"));
    let a = as_json(&ask(&corpus, WORKED).unwrap());

    // the canonical query, as text and as tree (C104)
    assert_eq!(
        a["query"]["text"],
        r##"league=Standard class=Rings rarity=rare line("# to maximum Life" arg1>=90) pseudo.total_res>=60"##
    );
    assert!(a["query"]["tree"]["all"].is_array());

    // the scope says what was searched (C96), the basis what it was read at (C98)
    let scope = &a["scope"];
    assert_eq!(scope["account"], json!({ "uuid": "u-1", "name": "A" }));
    assert_eq!(
        (&scope["realm"], &scope["membership"]),
        (&json!("pc"), &json!("live"))
    );
    assert_eq!(
        (&scope["items"], &scope["fetched"], &scope["never_fetched"]),
        (&json!(7), &json!(2), &json!(1))
    );
    assert_eq!(
        (
            &scope["oldest_fetch"],
            &scope["newest_fetch"],
            &scope["list_seen"]
        ),
        (&json!(20), &json!(30), &json!(10))
    );
    // each names the location it is the fetch of (#11)
    assert_eq!(
        (
            &scope["oldest_fetch_location"]["kind"],
            &scope["oldest_fetch_location"]["league"]
        ),
        (&json!("stash"), &json!("Standard"))
    );
    assert_ne!(
        scope["oldest_fetch_location"]["id"],
        scope["newest_fetch_location"]["id"]
    );
    assert_eq!(scope["leagues"], json!(["Standard"]));
    assert_eq!(a["basis"]["account"], "u-1");
    assert_eq!(
        a["basis"]["snapshot"]["response"],
        s.revision().unwrap().response
    );
    assert_eq!(
        (&a["basis"]["classes"], &a["basis"]["totals"]),
        (&json!(1), &json!(6))
    );

    // each term over the seven, independently (C93)
    let terms = a["terms"].as_array().unwrap();
    let paths: Vec<&str> = terms.iter().map(|t| t["path"].as_str().unwrap()).collect();
    assert_eq!(paths, ["0", "1", "2", "3", "4"]);
    assert_eq!(counts(&terms[0]), [7, 0, 0, 0]);
    assert_eq!(counts(&terms[1]), [6, 0, 0, 1]); // r5's base is in no class table
    assert_eq!(counts(&terms[2]), [6, 1, 0, 0]); // r4 is magic
    // r2 holds 20 and 75: failed, and the one that reaches 90 only
    // together; r7 has no life line; r6's explicit 95 is a witness
    assert_eq!(counts(&terms[3]), [5, 1, 1, 0]);
    assert_eq!(terms[3]["together"]["count"], 1);
    assert_eq!(
        (
            &terms[3]["together"]["slot"],
            &terms[3]["together"]["bound"]
        ),
        (&json!("arg1"), &json!(90))
    );
    // r3 totals 55; r6's total is an incomplete subtotal
    assert_eq!(counts(&terms[4]), [5, 1, 0, 1]);
    assert!(
        terms[0].get("together").is_none(),
        "not applicable is absent, never zero"
    );

    // r1; r5 and r6 are undecided at the root and are no match (C93), and
    // the answer says why by reason, each with its example, and what
    // might help — hints, never guarantees (V8)
    assert_eq!(a["total"]["matched"], 1);
    assert_eq!(a["total"]["undecided"]["count"], 2);
    assert_eq!(ids(&a), ["r1"]);
    let why = &a["total"]["undecided_reasons"][0];
    assert_eq!(why["example"]["id"], "r5");
    assert_eq!(why["unread"], "the class: base not in the table");
    assert!(
        why["hint"]
            .as_str()
            .unwrap()
            .contains("a refresh will not help; a reference update may")
    );
    let why = &a["total"]["undecided_reasons"][1];
    assert_eq!(why["example"]["id"], "r6");
    assert_eq!(why["unread"], "implicit lines");
    assert!(why["hint"].as_str().unwrap().contains("may help"));

    // a row: the header, the place by name and id, the lines the query touched
    let r1 = &a["rows"][0];
    assert_eq!(
        (&r1["id"], &r1["base"], &r1["rarity"]),
        (&json!("r1"), &json!("Two-Stone Ring"), &json!("Rare"))
    );
    assert_eq!(r1["place"]["league"], "Standard");
    assert_eq!(
        (&r1["place"]["name"], &r1["place"]["id"]),
        (&json!("Rings"), &json!("t1"))
    );
    let shows: Vec<&Value> = r1["matched"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|m| m["shows"].as_array().unwrap())
        .collect();
    assert!(
        shows
            .contains(&&json!({ "line": { "source": "explicit", "text": "+95 to maximum Life" } }))
    );
    assert!(shows.contains(&&json!({ "value": { "name": "pseudo.total_res", "value": 65 } })));
    assert!(a.get("zero").is_none());

    // the routes the example names, by their members
    assert_eq!(follow(&corpus, &terms[3]["failed"]), ["r2"]);
    assert_eq!(
        terms[3]["failed"]["request"]["query"]["text"],
        r##"line("# to maximum Life") -line("# to maximum Life" arg1>=90)"##
    );
    assert_eq!(follow(&corpus, &terms[3]["together"]), ["r2"]);
    assert_eq!(follow(&corpus, &terms[3]["lacked"]), ["r7"]);
    assert_eq!(follow(&corpus, &terms[1]["undecided"]), ["r5"]);
    assert_eq!(follow(&corpus, &terms[4]["undecided"]), ["r6"]);
    assert_eq!(follow(&corpus, &a["total"]["undecided"]), ["r5", "r6"]);
    // the basis is stated once, on the answer (B3)
    assert!(terms[3]["failed"].get("counted_at").is_none());
}

/// Invariant 4, and rule 5 of the plan: every count an answer shows has a
/// route, the route is a request this build runs, and it returns exactly
/// the members counted — per term the four routes are disjoint and cover
/// the scope.
#[test]
fn the_route_property_every_count_routes_to_exactly_its_members() {
    let s = worked();
    let corpus = load(&s, Some("pc"));
    for text in [
        WORKED,
        "",
        r##"rarity=rare or "+# to maximum Life">=100"##,
        r##"-rarity=magic -line("# to maximum Life" arg1>=90)"##,
        r##"holds(rarity=rare, "+# to maximum Life">=95, sum("+#% to Fire Resistance")>=65)>=2"##,
        r##"undecided(sum("+#% to Fire Resistance")>=60) or tab:dump"##,
        r##"undecided(pseudo.total_res) or (pseudo.total_fire_res=..60 -pseudo.total_res>=65)"##,
        r##"line(template:life source=implicit) name:r2 has:note -has:ilvl is:identified "two-stone""##,
        r##"line("# to maximum Life" (arg1>=100 or arg1<=20))"##,
        r##"character:mover league:hard ilvl=80..86 id:t2 true() false()"##,
        // comparisons nested where a selector by syntax would lose them
        r##"line(template:life (source=explicit arg1>=90))"##,
        r##"line((template:life arg1>=96) or (template:resistance arg1>=65))"##,
        r##"line(template:life -(arg1>=90 source=implicit))"##,
    ] {
        let a = as_json(&ask(&corpus, text).unwrap());
        for term in a["terms"].as_array().unwrap() {
            let mut all: Vec<String> = Vec::new();
            for kind in ["matched", "failed", "lacked", "undecided"] {
                let count = &term[kind];
                if count["count"] == 0 {
                    assert!(count.get("request").is_none());
                    continue;
                }
                all.extend(follow(&corpus, count));
            }
            all.sort();
            let whole: Vec<String> = ["r1", "r2", "r3", "r4", "r5", "r6", "r7"]
                .map(String::from)
                .to_vec();
            assert_eq!(
                all, whole,
                "`{}` in `{text}`: its four routes partition the scope",
                term["term"]
            );
            if term["together"]["count"].as_u64().is_some_and(|n| n > 0) {
                follow(&corpus, &term["together"]);
            }
        }
        if a["total"]["undecided"]["count"] != 0 {
            follow(&corpus, &a["total"]["undecided"]);
        }
        // every command an answer would print is one the parser takes
        let text = a["query"]["text"].as_str().unwrap();
        assert_eq!(as_json(&ask(&corpus, text).unwrap())["query"]["text"], text);
    }
}

/// Invariant 2: a `:` or `~` selector prints back as authored, with what
/// it resolved to beside it.
#[test]
fn invariant_2_a_selector_is_printed_as_authored_with_its_binding_beside_it() {
    let s = worked();
    let corpus = load(&s, Some("pc"));
    let a = as_json(
        &ask(
            &corpus,
            r##"base:ring line(template~"resist" arg1>=60) rarity:ra"##,
        )
        .unwrap(),
    );
    assert_eq!(
        a["query"]["text"],
        r##"base:ring line(template~"resist" arg1>=60) rarity:ra"##
    );
    let terms = a["terms"].as_array().unwrap();
    assert_eq!(
        terms[0]["resolved"],
        json!({ "of": "base", "values": [{ "value": "Two-Stone Ring", "items": 6 }, { "value": "Mystery Ring", "items": 1 }], "more": 0 })
    );
    assert_eq!(
        terms[1]["resolved"],
        json!({ "of": "template", "values": [{ "value": "#% to Fire Resistance", "items": 7 }], "more": 0 })
    );
    // `ra` picks `rare` among the legal values and no other
    assert_eq!(
        terms[2]["resolved"]["values"],
        json!([{ "value": "Rare", "items": 6 }])
    );
    // an exact selector resolves to nothing but itself
    let a = as_json(&ask(&corpus, "base=\"Two-Stone Ring\"").unwrap());
    assert!(a["terms"][0].get("resolved").is_none());
}

/// Invariant 6: search is a pure read — every table of the facts file is
/// the same before and after.
#[test]
fn invariant_6_a_search_writes_nothing() {
    fn dump(path: &std::path::Path) -> Vec<String> {
        let conn = rusqlite::Connection::open(path).unwrap();
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        let mut out = Vec::new();
        for table in tables {
            let mut stmt = conn.prepare(&format!("SELECT * FROM \"{table}\"")).unwrap();
            let n = stmt.column_count();
            let mut rows = stmt.query([]).unwrap();
            while let Some(row) = rows.next().unwrap() {
                let cells: Vec<String> = (0..n)
                    .map(|i| format!("{:?}", row.get_ref(i).unwrap()))
                    .collect();
                out.push(format!("{table}: {}", cells.join(" | ")));
            }
        }
        out
    }
    let dir = tempfile::tempdir().unwrap();
    let s = worked_in(&dir);
    let path = s.path().to_path_buf();
    let before = dump(&path);
    assert!(before.len() > 10);
    let corpus = load(&s, None);
    for text in [WORKED, "", r##"name="nothing here""##] {
        ask(&corpus, text).unwrap();
    }
    common::show(&s, "r1", true).unwrap();
    assert_eq!(dump(&path), before);
}

/// C93 — composition over an undecided term, asked of r6, whose sum is an
/// incomplete subtotal.
#[test]
fn c93_undecided_composes_and_is_never_a_match() {
    let s = worked();
    let corpus = load(&s, Some("pc"));
    let open = r##"sum("+#% to Fire Resistance")>=60"##;
    let of_r6 = |text: &str| {
        let a = as_json(&ask(&corpus, &format!("id:r6 ({text})")).unwrap());
        (
            a["total"]["matched"].as_u64().unwrap(),
            a["total"]["undecided"]["count"].as_u64().unwrap(),
        )
    };
    assert_eq!(of_r6(&format!("{open} rarity=rare")), (0, 1));
    assert_eq!(
        of_r6(&format!("{open} or rarity=rare")),
        (1, 0),
        "true or undecided is true"
    );
    assert_eq!(
        of_r6(&format!("{open} rarity=magic")),
        (0, 0),
        "false and undecided is false"
    );
    assert_eq!(
        of_r6(&format!("-{open} rarity=rare")),
        (0, 1),
        "not undecided is undecided"
    );
    // one sure, one open: the count is in [1, 2]
    assert_eq!(of_r6(&format!("holds(rarity=rare, {open})>=1")), (1, 0));
    assert_eq!(of_r6(&format!("holds(rarity=rare, {open})>=2")), (0, 1));
    assert_eq!(of_r6(&format!("holds(rarity=rare, {open})<=1")), (0, 1));
    assert_eq!(of_r6(&format!("holds(rarity=magic, {open})>=2")), (0, 0));
    // asked for by name it is decided, and known absence does not satisfy it
    assert_eq!(of_r6(&format!("undecided({open}) rarity=rare")), (1, 0));
    assert_eq!(of_r6("undecided(name) or undecided(league)"), (0, 0));
    // a readable line is a witness; a readable miss beside an unread array is not
    assert_eq!(of_r6(r##""+# to maximum Life">=90 rarity=rare"##), (1, 0));
    assert_eq!(of_r6(r##""+# to maximum Life">=96 rarity=rare"##), (0, 1));
    // the group's own source rules the unread array out
    assert_eq!(
        of_r6(r##"line("# to maximum Life" source=explicit arg1>=96) or rarity=magic"##),
        (0, 0)
    );
    // a phrase: a hit in what is readable matches; no hit is undecided
    assert_eq!(of_r6(r##""maximum life" rarity=rare"##), (1, 0));
    assert_eq!(of_r6(r##""chaos" rarity=rare"##), (0, 1));
}

/// C92 — the sort scalar is the largest satisfying occurrence; an item
/// with none sorts last in either direction, its status shown.
#[test]
fn c92_the_sort_scalar_and_an_item_without_one_last_either_way() {
    let s = worked();
    let corpus = load(&s, Some("pc"));
    let sorted = |desc: bool| {
        let request = serde_json::from_value(json!({
            "scope": { "realm": "pc" },
            "query": { "text": "rarity=rare" },
            "view": { "rows": { "sort": format!("line(\"{LIFE}\").arg1"), "desc": desc } },
        }))
        .unwrap();
        let a = as_json(&answer(&corpus, &request).unwrap());
        assert_eq!(
            a["view"]["rows"]["sort"],
            "line(\"# to maximum Life\").arg1"
        );
        let rows: Vec<(String, Value)> = a["rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| (r["id"].as_str().unwrap().to_string(), r["sort"].clone()))
            .collect();
        rows
    };
    let down = sorted(true);
    let order: Vec<&str> = down.iter().map(|(id, _)| id.as_str()).collect();
    // r3 100; r1 and r5 95 in the store's order; r2's largest is 75; r6's
    // 95 is no largest while its implicit array is unread; r7 has none
    assert_eq!(order, ["r3", "r1", "r5", "r2", "r6", "r7"]);
    // a row says what it sorts by since the first seat (F6): the
    // occurrence whose slot is the largest, beside the number
    assert_eq!(down[3].1["value"], 75);
    assert_eq!(down[3].1["shows"][0]["line"]["text"], "+75 to maximum Life");
    assert_eq!(down[4].1["value"], 95);
    assert_eq!(down[4].1["status"], "incomplete");
    assert_eq!(down[5].1, json!({ "status": "no satisfying occurrence" }));
    let up = sorted(false);
    let order: Vec<&str> = up.iter().map(|(id, _)| id.as_str()).collect();
    assert_eq!(order, ["r2", "r1", "r5", "r3", "r6", "r7"]);

    // the limit counts what it leaves out, and says how to go on
    let request = serde_json::from_value(json!({ "view": { "rows": { "limit": 2 } } })).unwrap();
    let a = as_json(&answer(&corpus, &request).unwrap());
    let mut rows = a["view"]["rows"].clone();
    let next = rows.as_object_mut().unwrap().remove("next").unwrap();
    assert_eq!(rows, json!({ "limit": 2, "returned": 2, "left_out": 5 }));
    assert_eq!(next["request"]["view"]["rows"]["after"], 2);
    assert_eq!(a["total"]["matched"], 7);
}

/// C96 — over a store holding more than one realm a search names one; all
/// is a scope of its own; a realm the store lacks is a valid scope of
/// nothing, and says so.
#[test]
fn c96_the_realm_is_the_scope_and_is_always_resolved() {
    let mut s = worked();
    // one realm held: none named resolves to it, and the answer prints it
    assert_eq!(load(&s, None).realm, Realm::One("pc".to_string()));
    list_tabs(&mut s, "xbox", "Standard", json!([tab("x1", "Box")]), 40);
    fetch_tab(
        &mut s,
        "xbox",
        "Standard",
        "x1",
        "Box",
        vec![ring("rx", "Rare", json!({}))],
        41,
    );
    let e = Corpus::load(&s, &intent(&s), None).unwrap_err();
    assert_eq!(e.to_json()["kind"], "realm_needed");
    assert_eq!(
        e.to_json()["readings"],
        json!(["realm pc", "realm xbox", "realm all"])
    );
    let all = load(&s, Some("all"));
    let a = as_json(&ask(&all, "").unwrap());
    assert_eq!(
        (&a["scope"]["realm"], &a["total"]["matched"]),
        (&json!("all"), &json!(8))
    );
    assert_eq!(
        a["terms"],
        json!([]),
        "the empty query: every item in scope"
    );
    let pc = as_json(&ask(&load(&s, Some("pc")), "").unwrap());
    assert_eq!(
        (&pc["scope"]["items"], &pc["scope"]["fetched"]),
        (&json!(7), &json!(2))
    );
    // a route carries its realm resolved
    let xbox = as_json(&ask(&load(&s, Some("xbox")), "rarity=rare").unwrap());
    assert_eq!(
        xbox["terms"][0]["matched"]["request"]["scope"],
        json!({ "account": "A", "realm": "xbox", "membership": "live" })
    );
    let sony = as_json(&ask(&load(&s, Some("sony")), "").unwrap());
    assert_eq!(
        (&sony["scope"]["items"], &sony["scope"]["realm_not_held"]),
        (&json!(0), &json!(true))
    );
    assert_eq!(
        Realm::parse("ps4").unwrap_err().to_json()["kind"],
        "realm_unknown"
    );
    // a request bound to another account, or another realm than the corpus holds, is refused
    let other = serde_json::from_value(json!({ "scope": { "account": "B" } })).unwrap();
    assert_eq!(
        answer(&all, &other).unwrap_err().to_json()["kind"],
        "account_mismatch"
    );
    let other = serde_json::from_value(json!({ "scope": { "realm": "pc" } })).unwrap();
    assert_eq!(
        answer(&all, &other).unwrap_err().to_json()["kind"],
        "realm_mismatch"
    );
}

/// C98, at the answer's boundary: an answer from a held corpus describes
/// the state its basis names, whatever landed since; the check before
/// every answer sees the change, and the next load carries it.
#[test]
fn c98_a_held_corpus_answers_at_its_basis_and_the_check_sees_the_change() {
    let dir = tempfile::tempdir().unwrap();
    let s = worked_in(&dir);
    let path = s.path().to_path_buf();
    let intent = intent(&s);
    let held = load_with(&s, &intent, Some("pc"));
    assert!(held.is_current(&s, &intent).unwrap());
    let mut other = Store::open(&path).unwrap();
    fetch_tab(
        &mut other,
        "pc",
        "Standard",
        "t3",
        "Never",
        vec![ring("late", "Rare", json!({}))],
        50,
    );

    let a = as_json(&ask(&held, "").unwrap());
    assert_eq!(
        (&a["total"]["matched"], &a["scope"]["never_fetched"]),
        (&json!(7), &json!(1))
    );
    assert_eq!(a["basis"], serde_json::to_value(&held.basis).unwrap());
    assert!(!held.is_current(&s, &intent).unwrap());

    let next = load(&s, Some("pc"));
    assert!(next.basis.snapshot.response > held.basis.snapshot.response);
    let a = as_json(&ask(&next, "").unwrap());
    assert_eq!(
        (&a["total"]["matched"], &a["scope"]["never_fetched"]),
        (&json!(8), &json!(0))
    );
}

/// C98 — the basis names the store (outside audit, 2026-09-20;
/// owner, 2026-09-20: "(a') now and park (c)"): two facts files of one
/// account at one revision are two stores, their bases differ, and a
/// corpus read from one is not current against the other.
#[test]
fn c98_the_basis_names_the_store_and_the_check_tells_two_files_apart() {
    let (dir_a, dir_b) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    let (a, b) = (worked_in(&dir_a), worked_in(&dir_b));
    assert_eq!(a.revision().unwrap(), b.revision().unwrap());
    let intent = intent(&a);
    let (of_a, of_b) = (
        load_with(&a, &intent, Some("pc")),
        load_with(&b, &intent, Some("pc")),
    );
    assert_eq!(of_a.basis.account, of_b.basis.account);
    assert_ne!(of_a.basis.store, of_b.basis.store);
    assert!(of_a.is_current(&a, &intent).unwrap() && !of_a.is_current(&b, &intent).unwrap());
    // twelve hex digits, no path: the same file under another handle is the same store
    let again = load(&Store::open(a.path()).unwrap(), Some("pc"));
    assert_eq!(again.basis, of_a.basis);
    assert!(
        of_a.basis.store.len() == 12 && of_a.basis.store.chars().all(|c| c.is_ascii_hexdigit())
    );
    let answered = as_json(&ask(&of_a, "").unwrap());
    assert_eq!(answered["basis"]["store"], of_a.basis.store.as_str());
    assert!(
        !answered
            .to_string()
            .contains(a.path().parent().unwrap().to_str().unwrap())
    );
}

/// C98 — one holder for every resident frontend (`Holder`, #82): two
/// asks over an unchanged store are answered over one corpus; a tab
/// recorded between them hands back another, at a higher snapshot,
/// counting the new item; another membership loads anew and replaces;
/// a realm is the one the load resolves, a site search's URL's where the
/// scope names none, so a request naming no realm over a store of two is
/// refused, never answered over the URL's corpus; and a second store is
/// never answered from the first's corpus. No step reads a clock.
#[test]
fn c98_the_holder_reuses_one_corpus_while_its_basis_and_scope_hold() {
    use acquisition_search::{Holder, Request};
    use std::sync::Arc;
    let asked = |request: Value| serde_json::from_value::<Request>(request).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let s = worked_in(&dir);
    let intent = intent(&s);
    let holder = Holder::new();
    let live = asked(json!({}));

    let first = holder.corpus(&live, &s, &intent).unwrap();
    let again = holder.corpus(&live, &s, &intent).unwrap();
    assert!(Arc::ptr_eq(&first, &again), "an unchanged store loads once");

    let mut other = Store::open(s.path()).unwrap();
    fetch_tab(
        &mut other,
        "pc",
        "Standard",
        "t3",
        "Never",
        vec![ring("late", "Rare", json!({}))],
        50,
    );
    let moved = holder.corpus(&live, &s, &intent).unwrap();
    assert!(
        !Arc::ptr_eq(&first, &moved),
        "a recorded tab moves the basis"
    );
    assert!(moved.basis.snapshot.response > first.basis.snapshot.response);
    assert_eq!(
        as_json(&answer(&moved, &live).unwrap())["total"]["matched"],
        json!(8)
    );

    let all = asked(json!({ "scope": { "membership": "all" } }));
    let of_all = holder.corpus(&all, &s, &intent).unwrap();
    assert!(!Arc::ptr_eq(&moved, &of_all));
    assert_eq!(
        of_all.membership,
        acquisition_search::answer::Membership::All
    );
    let back = holder.corpus(&live, &s, &intent).unwrap();
    assert!(
        !Arc::ptr_eq(&moved, &back),
        "one slot: the other scope replaced it"
    );

    // a second realm, then nothing moves between the two asks
    list_tabs(
        &mut other,
        "xbox",
        "Standard",
        json!([tab("x1", "Box")]),
        60,
    );
    fetch_tab(
        &mut other,
        "xbox",
        "Standard",
        "x1",
        "Box",
        vec![ring("rx", "Rare", json!({}))],
        61,
    );
    const TRADE_ID: &str = "H4sIAAAAAAAA_5XM0QrCMAyF4XfJdZFhB2N7FRlSuwiB2JYtFUfJu5speO9t_pyvwSZB6gZTg1yEcoIJcmJKCOo-0dqlgewFLYW0gIM7seD6DbTYGV-FKZKcjsHVn8fR90PXe_t9Bq548A8ye-hUZ3U_L6410o3xL9SEWd8TZHM0ugAAAA";
    let url = format!("https://www.pathofexile.com/trade/search/Standard/{TRADE_ID}");
    let site = asked(json!({ "query": { "trade": url } }));
    assert_eq!(
        holder.corpus(&site, &s, &intent).unwrap().realm,
        Realm::One("pc".to_string())
    );
    let e = holder.corpus(&live, &s, &intent).unwrap_err();
    assert_eq!(e.to_json()["kind"], "realm_needed", "{}", e.to_json());

    let pc = asked(json!({ "scope": { "realm": "pc" } }));
    let of_s = holder.corpus(&pc, &s, &intent).unwrap();
    let dir_b = tempfile::tempdir().unwrap();
    let b = worked_in(&dir_b);
    let of_b = holder.corpus(&pc, &b, &intent).unwrap();
    assert!(!Arc::ptr_eq(&of_s, &of_b));
    assert_ne!(of_s.basis.store, of_b.basis.store);
    assert_eq!(of_b.basis, load_with(&b, &intent, Some("pc")).basis);
}

/// C104 — a request sends the text or the tree, and the answer returns both.
#[test]
fn c104_a_tree_is_accepted_and_the_answer_returns_both() {
    let s = worked();
    let corpus = load(&s, Some("pc"));
    let by_text = as_json(&ask(&corpus, WORKED).unwrap());
    let request =
        serde_json::from_value(json!({ "query": { "tree": by_text["query"]["tree"] } })).unwrap();
    let by_tree = as_json(&answer(&corpus, &request).unwrap());
    assert_eq!(by_tree, by_text);
    let both = serde_json::from_value(
        json!({ "query": { "text": "", "tree": by_text["query"]["tree"] } }),
    )
    .unwrap();
    assert_eq!(
        answer(&corpus, &both).unwrap_err().to_json()["kind"],
        "tree"
    );
    assert!(serde_json::from_value::<acquisition_search::Request>(json!({ "views": {} })).is_err());
}

/// C115: an answer weighed 31 KB by default on the owner's store, over
/// every seat runner's harness cut, a count's request a third of it — 13,693
/// bytes for 17 numbers (the trade seat, mcp-sonnet-1 call 8). Unasked, it
/// carries the routes a next step hangs on — what is undecided, what holds
/// only together, what each term matched where nothing matched, what a key
/// could not place — and every other count is its number alone; asked,
/// every count that is not zero holds its route, and the numbers are the
/// same either way.
/// A vocabulary's computed values are placed as its templates are: a
/// default `line:resist` count kept five of their requests when only
/// `value` buckets shed theirs (the review of note 38 at `16bba23e`, step 1), the CLI
/// printing those routes without `--routes`.
#[test]
fn c115_a_computed_bucket_sheds_its_route_as_a_value_does() {
    let s = worked();
    let corpus = load(&s, Some("pc"));
    let request = serde_json::from_value(json!({
        "query": { "text": "rarity=rare" },
        "view": { "counts": { "keys": ["line:resist"] } }
    }))
    .unwrap();
    let a = as_json(&acquisition_search::answer(&corpus, &request).unwrap());
    let buckets = a["view"]["counts"]["tables"][0]["buckets"]
        .as_array()
        .unwrap();
    assert!(
        buckets.iter().any(|b| b["bucket"] == "computed"),
        "the fixture must carry a computed value:\n{a}"
    );
    for b in buckets {
        let placed = b["bucket"] != "none" && b["bucket"] != "undecided";
        assert_eq!(
            b.get("request").is_none(),
            placed,
            "{} bucket {}: {b}",
            b["bucket"],
            b["value"]
        );
    }
}

#[test]
fn c115_an_answer_unasked_carries_only_the_routes_a_next_step_hangs_on() {
    let s = worked();
    let corpus = load(&s, Some("pc"));
    // the crate's own answer, not this suite's, which asks every route
    let asked = |text: &str, view: Value| -> Value {
        let request =
            serde_json::from_value(json!({ "query": { "text": text }, "view": view })).unwrap();
        as_json(&acquisition_search::answer(&corpus, &request).unwrap())
    };
    let routed = |count: &Value| count.get("request").is_some();
    let a = asked(WORKED, json!({ "rows": {} }));
    for term in a["terms"].as_array().unwrap() {
        for k in ["matched", "failed", "lacked"] {
            assert!(
                !routed(&term[k]),
                "{k} of term {}: {}",
                term["path"],
                term[k]
            );
        }
    }
    assert!(routed(&a["terms"][1]["undecided"]), "{}", a["terms"][1]);
    assert!(routed(&a["terms"][4]["undecided"]), "{}", a["terms"][4]);
    assert!(routed(&a["terms"][3]["together"]), "{}", a["terms"][3]);
    assert!(routed(&a["total"]["undecided"]), "{}", a["total"]);

    let every = asked(WORKED, json!({ "rows": {}, "routes": true }));
    for (term, all) in a["terms"]
        .as_array()
        .unwrap()
        .iter()
        .zip(every["terms"].as_array().unwrap())
    {
        assert_eq!(counts(term), counts(all), "{}", term["path"]);
        for k in ["matched", "failed", "lacked", "undecided"] {
            assert_eq!(routed(&all[k]), all[k]["count"] != 0, "{k}: {}", all[k]);
        }
    }
    assert_eq!(follow(&corpus, &every["terms"][2]["failed"]), ["r4"]);

    // where nothing matched, what each term matched alone is the way on
    let none = asked("rarity=rare rarity=magic", json!({ "rows": {} }));
    assert_eq!(none["total"]["matched"], 0);
    for term in none["terms"].as_array().unwrap() {
        assert!(routed(&term["matched"]), "{term}");
        assert!(!routed(&term["failed"]), "{term}");
    }

    // a count: a value's bucket is its number, what the key could not
    // place keeps its route — and both have theirs when asked
    let by_class = asked("", json!({ "counts": { "keys": ["class"] } }));
    let buckets = by_class["view"]["counts"]["tables"][0]["buckets"]
        .as_array()
        .unwrap();
    assert_eq!(buckets.len(), 2, "{buckets:?}");
    assert_eq!(buckets[0]["bucket"], "value");
    assert!(!routed(&buckets[0]), "{}", buckets[0]);
    assert_eq!(buckets[1]["bucket"], "undecided");
    assert!(routed(&buckets[1]), "{}", buckets[1]);
    let by_class = asked(
        "",
        json!({ "counts": { "keys": ["class"] }, "routes": true }),
    );
    assert!(routed(
        &by_class["view"]["counts"]["tables"][0]["buckets"][0]
    ));
}

/// `scope.list_seen` under a query that fixes one league is that league's
/// list (and the characters'), never the realm's oldest (the re-sit,
/// mcp-opus fault 2): Standard's tabs were listed at 10, an empty
/// Allflame listing at 5. A refinement, the old text parenthesised and a
/// term appended, fixes the same league through its nested conjunction
/// (`fixed_league`, `src/answer.rs`). A league no list names keeps the
/// realm's oldest, never none.
#[test]
fn the_scopes_list_age_under_a_query_fixing_one_league_is_that_leagues() {
    let mut s = worked();
    list_tabs(&mut s, "pc", "Allflame", json!([]), 5);
    let corpus = load(&s, Some("pc"));
    let fixed = as_json(&ask(&corpus, "league=Standard").unwrap());
    assert_eq!(fixed["scope"]["list_seen"], 10, "{}", fixed["scope"]);
    let refined = as_json(&ask(&corpus, "(league=Standard rarity=rare) ilvl>=1").unwrap());
    assert_eq!(refined["scope"]["list_seen"], 10, "{}", refined["scope"]);
    let whole = as_json(&ask(&corpus, "").unwrap());
    assert_eq!(whole["scope"]["list_seen"], 5, "{}", whole["scope"]);
    let unknown = as_json(&ask(&corpus, "league=Nowhere").unwrap());
    assert_eq!(unknown["scope"]["list_seen"], 5, "{}", unknown["scope"]);
}

/// C115, a row as slim as the decision view allows (C53, C100): a row says
/// each term it touched by its path, which the terms block names, and the
/// lines it touched with their values, and nothing else of the term; a
/// line has `flags` only where it has any; `next` is the request alone,
/// never a terminal's token beside it.
#[test]
fn a_touched_term_on_a_row_is_its_path_its_lines_and_what_it_left_out() {
    let s = worked();
    let corpus = load(&s, Some("pc"));
    let request = serde_json::from_value(json!({
        "query": { "text": "\"+# to maximum Life\">=90" },
        "view": { "rows": { "limit": 1 } },
    }))
    .unwrap();
    let a = as_json(&acquisition_search::answer(&corpus, &request).unwrap());
    assert_eq!(a["rows"][0]["id"], "r1", "{a}");
    let touched = a["rows"][0]["matched"].as_array().unwrap();
    assert!(!touched.is_empty(), "{a}");
    for t in touched {
        let keys: Vec<&String> = t.as_object().unwrap().keys().collect();
        assert!(
            keys.iter()
                .all(|k| ["path", "shows", "left_out"].contains(&k.as_str())),
            "{t}"
        );
    }
    let life = touched.iter().find(|t| t["path"] == "0").unwrap();
    assert_eq!(
        life["shows"],
        json!([{ "line": { "source": "explicit", "text": "+95 to maximum Life" } }])
    );
    assert_eq!(
        a["terms"][0]["term"],
        "line(\"# to maximum Life\" arg1>=90)"
    );
    let next = &a["view"]["rows"]["next"];
    assert!(
        next.get("token").is_none() && next["request"].is_object(),
        "{next}"
    );
}

/// The words a printed command's quoting is pinned on, each beside its
/// quoting for every shell C156 names: one table, `shell-words.json`,
/// which the daemon's twin and the CLI's run through each shell read too
/// (C1 keeps the functions two, not their cases).
fn shell_words() -> Vec<(String, [(acquisition_search::answer::Shell, String); 4])> {
    use acquisition_search::answer::Shell;
    let rows: Vec<Value> = serde_json::from_str(include_str!("shell-words.json")).unwrap();
    rows.iter()
        .map(|row| {
            let column = |name: &str| row[name].as_str().unwrap().to_string();
            (
                column("word"),
                Shell::DECLARED.map(|(name, shell)| (shell, column(name))),
            )
        })
        .collect()
}

/// #16, C156 — a printed command's words are quoted for the shell it
/// will be pasted into: a POSIX shell's single quotes, `'\''` for a
/// quote inside; PowerShell's, a quote inside doubled (the typographic
/// ones too), `@` and `,` quoted, and a bare word PowerShell would read
/// as a number it prints another way quoted; Windows PowerShell 5.1's,
/// PowerShell's over the C runtime's escapes; cmd.exe's, the C
/// runtime's double quotes with every character cmd reads escaped by a
/// caret. The shell is `ACQ_SHELL`'s, else PowerShell on Windows
/// outside Git Bash and a POSIX shell elsewhere.
#[test]
fn a_printed_word_is_quoted_for_the_declared_shell() {
    use acquisition_search::answer::{Shell, shell_quoted};
    assert_eq!(Shell::detected(true, false), Shell::Pwsh);
    assert_eq!(Shell::detected(true, true), Shell::Posix);
    assert_eq!(Shell::detected(false, false), Shell::Posix);
    assert_eq!(Shell::detected(false, true), Shell::Posix);
    for (name, shell) in Shell::DECLARED {
        assert_eq!(Shell::named(name), Some(shell));
    }
    assert_eq!(Shell::named("bash"), None);
    assert_eq!(Shell::named("PWSH"), None);
    for (word, quoted) in shell_words() {
        for (shell, expected) in &quoted {
            assert_eq!(&shell.quoted(&word), expected, "{shell:?}: {word}");
        }
        // the shell is the environment's, which this process may declare
        // or run under Git Bash
        let here = Shell::of_environment();
        let expected = &quoted.iter().find(|(shell, _)| *shell == here).unwrap().1;
        assert_eq!(&shell_quoted(&word), expected, "{word}");
    }
}

// ---- the matches' fetch span on `total` (#155; C118, C162) ----------------------------------

const DAY: i64 = 86_400;
/// The answer's time for the span's ages: eight days after the first
/// fetch of [`three_tabs`].
const AT: i64 = 1_000 + 8 * DAY;
/// The span's keys on `total`, beside `matched`.
const SPAN: [&str; 9] = [
    "places",
    "oldest_fetch",
    "oldest_fetch_age",
    "oldest_fetch_location",
    "newest_fetch",
    "newest_fetch_age",
    "newest_fetch_location",
    "list_seen",
    "list_seen_age",
];

/// The span's keys `total` carries, and nothing else of it.
fn subject(a: &Value) -> Value {
    Value::Object(
        SPAN.iter()
            .filter_map(|k| a["total"].get(*k).map(|v| (k.to_string(), v.clone())))
            .collect(),
    )
}

fn stash(id: &str, name: &str, league: &str) -> Value {
    json!({ "kind": "stash", "id": id, "name": name, "realm": "pc", "league": league })
}

fn asked_at(corpus: &Corpus, request: Value) -> Value {
    let request = serde_json::from_value(request).unwrap();
    as_json(&answer(corpus, &request).unwrap().aged(AT))
}

/// Standard's list seen at 500; `Old` (t3) fetched at 1,000 holding a
/// magic ring; `Rings` (t1) two days later holding three rares; `Dump`
/// (t2) five days later holding one.
fn three_tabs() -> Store {
    let mut s = store();
    list_tabs(
        &mut s,
        "pc",
        "Standard",
        json!([tab("t1", "Rings"), tab("t2", "Dump"), tab("t3", "Old")]),
        500,
    );
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "t3",
        "Old",
        vec![ring("m1", "Magic", json!({}))],
        1_000,
    );
    let rares = ["r1", "r2", "r4"].map(|id| ring(id, "Rare", json!({})));
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "t1",
        "Rings",
        rares.to_vec(),
        1_000 + 2 * DAY,
    );
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "t2",
        "Dump",
        vec![ring("r3", "Rare", json!({}))],
        1_000 + 5 * DAY,
    );
    s
}

/// The seat quoted the scope's ages for the matches (#155): `total` says
/// the matches' own, over their two locations, on the rows however
/// limited, on the counts and on the cross view alike, and the scope's
/// oldest is its own older location's.
#[test]
fn c162_total_carries_the_matches_fetch_span_on_every_view() {
    let corpus = load(&three_tabs(), Some("pc"));
    let expected = json!({
        "places": 2,
        "oldest_fetch": 1_000 + 2 * DAY,
        "oldest_fetch_age": "6d ago",
        "oldest_fetch_location": stash("t1", "Rings", "Standard"),
        "newest_fetch": 1_000 + 5 * DAY,
        "newest_fetch_age": "3d ago",
        "newest_fetch_location": stash("t2", "Dump", "Standard"),
        "list_seen": 500,
        "list_seen_age": "8d ago",
    });
    let query = json!({ "text": "rarity=rare" });
    for view in [
        json!({}),
        json!({ "rows": { "limit": 1 } }),
        json!({ "counts": { "keys": ["league"] } }),
        json!({ "cross": { "keys": ["league", "rarity"] } }),
    ] {
        let a = asked_at(&corpus, json!({ "query": query, "view": view }));
        assert_eq!(a["total"]["matched"], 4, "{view}");
        assert_eq!(subject(&a), expected, "{view}");
        // the scope's oldest is the older location no match is in
        assert_eq!(
            (
                &a["scope"]["oldest_fetch"],
                &a["scope"]["oldest_fetch_location"]["id"]
            ),
            (&json!(1_000), &json!("t3")),
            "{view}"
        );
    }
}

#[test]
fn c162_matches_in_one_location_are_one_place_its_fetch_both_ways() {
    let corpus = load(&three_tabs(), Some("pc"));
    let a = asked_at(&corpus, json!({ "query": { "text": "id:r3" } }));
    let dump = stash("t2", "Dump", "Standard");
    assert_eq!(a["total"]["places"], 1);
    assert_eq!(
        (
            &a["total"]["oldest_fetch"],
            &a["total"]["oldest_fetch_location"]
        ),
        (&json!(1_000 + 5 * DAY), &dump)
    );
    assert_eq!(
        (
            &a["total"]["newest_fetch"],
            &a["total"]["newest_fetch_location"]
        ),
        (&json!(1_000 + 5 * DAY), &dump)
    );
}

#[test]
fn c162_a_location_holding_three_matches_is_counted_once() {
    let corpus = load(&three_tabs(), Some("pc"));
    let a = asked_at(
        &corpus,
        json!({ "query": { "text": "rarity=rare -id:r3" } }),
    );
    assert_eq!(
        (&a["total"]["matched"], &a["total"]["places"]),
        (&json!(3), &json!(1))
    );
    assert_eq!(
        a["total"]["oldest_fetch_location"],
        stash("t1", "Rings", "Standard")
    );
    assert_eq!(
        a["total"]["oldest_fetch_location"],
        a["total"]["newest_fetch_location"]
    );
}

/// Nothing matched: no span, and no place.
#[test]
fn c162_no_match_has_no_places_and_no_span() {
    let corpus = load(&three_tabs(), Some("pc"));
    let a = asked_at(&corpus, json!({ "query": { "text": "rarity=unique" } }));
    assert_eq!(subject(&a), json!({ "places": 0 }));
}

/// A socketed item sits where its holder does: the two are one place.
#[test]
fn c162_a_socketed_match_is_its_holders_place() {
    let mut s = store();
    list_tabs(&mut s, "pc", "Standard", json!([tab("t1", "Gear")]), 10);
    let gem = item("g1", "", "Fireball", "Gem", json!({}));
    let holder = ring("r1", "Rare", json!({ "socketedItems": [gem] }));
    fetch_tab(&mut s, "pc", "Standard", "t1", "Gear", vec![holder], 20);
    let corpus = load(&s, Some("pc"));
    let a = asked_at(&corpus, json!({ "query": { "text": "" } }));
    assert_eq!(ids(&a), ["g1", "r1"]);
    assert_eq!(a["total"]["places"], 1);
    assert_eq!(
        a["total"]["oldest_fetch_location"],
        stash("t1", "Gear", "Standard")
    );
}

/// A tab id recurs across leagues and is two places; a character's does
/// not, so one character matched in two of its leagues is one place.
#[test]
fn c162_places_are_keyed_as_the_corpus_keys_a_location() {
    let mut s = store();
    for (league, item_id, at) in [("Old", "r-old", 20), ("Standard", "r-std", 40)] {
        list_tabs(&mut s, "pc", league, json!([tab("x1", "Shared")]), at - 5);
        fetch_tab(
            &mut s,
            "pc",
            league,
            "x1",
            "Shared",
            vec![ring(item_id, "Rare", json!({}))],
            at,
        );
    }
    let a = asked_at(
        &load(&s, Some("pc")),
        json!({ "query": { "text": "rarity=rare" } }),
    );
    assert_eq!(a["total"]["places"], 2);
    assert_eq!(
        (
            &a["total"]["oldest_fetch_location"],
            &a["total"]["newest_fetch_location"]
        ),
        (
            &stash("x1", "Shared", "Old"),
            &stash("x1", "Shared", "Standard")
        )
    );
    // both tab lists name a place of the set: the older is Old's
    assert_eq!(a["total"]["list_seen"], 15);

    let mut s = store();
    let in_league = |league: &str, equipment: Vec<Value>| json!({ "id": "c1", "name": "Mover", "league": league, "equipment": equipment });
    let listed = |league: &str| json!([{ "id": "c1", "name": "Mover", "league": league }]);
    list_characters(&mut s, "pc", listed("Settlers"), 10);
    let both = vec![
        ring("kept", "Rare", json!({})),
        ring("gone", "Rare", json!({})),
    ];
    fetch_character(&mut s, "pc", in_league("Settlers", both), 20);
    list_characters(&mut s, "pc", listed("Standard"), 30);
    let both = vec![
        ring("kept", "Rare", json!({})),
        ring("left", "Rare", json!({})),
    ];
    fetch_character(&mut s, "pc", in_league("Standard", both), 40);
    fetch_character(
        &mut s,
        "pc",
        in_league("Standard", vec![ring("kept", "Rare", json!({}))]),
        50,
    );
    // under both, matches in each of the character's two leagues
    for (membership, matched) in [
        (Membership::All, &["gone", "kept", "left"][..]),
        (Membership::Removed, &["gone", "left"][..]),
    ] {
        let corpus = Corpus::load_of(
            &s,
            &intent(&s),
            Some(&Realm::parse("pc").unwrap()),
            membership,
        )
        .unwrap();
        let a = asked_at(
            &corpus,
            json!({ "scope": { "membership": membership }, "query": { "text": "rarity=rare" } }),
        );
        assert_eq!(a["scope"]["membership"], json!(membership));
        assert_eq!(ids(&a), matched, "{membership:?}");
        let mut leagues: Vec<&str> = a["rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["place"]["league"].as_str().unwrap())
            .collect();
        leagues.sort();
        leagues.dedup();
        assert_eq!(leagues, ["Settlers", "Standard"], "{membership:?}");
        assert_eq!(a["total"]["places"], 1, "{membership:?}");
        assert_eq!(a["total"]["oldest_fetch_location"]["kind"], "character");
        assert_eq!(
            a["total"]["list_seen"], 30,
            "{membership:?}: the character list"
        );
    }
}

/// Under `all`, a retired tab keeps its last fetch, so its removed item's
/// place stands inside the span; a retired tab's substashes lose theirs,
/// so their removed items' places are counted and fetched by none.
#[test]
fn c162_a_retired_places_fetch_is_its_locations_last_or_none() {
    let mut s = store();
    list_tabs(
        &mut s,
        "pc",
        "Standard",
        json!([tab("t1", "Rings"), tab("t2", "Gone"), { "id": "m1", "name": "Maps", "type": "MapStash" }]),
        10,
    );
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "t2",
        "Gone",
        vec![ring("c", "Magic", json!({}))],
        30,
    );
    let stash_of = |sub: Option<&str>| acquisition_store::Endpoint::Stash {
        realm: "pc".into(),
        league: "Standard".into(),
        id: "m1".into(),
        sub: sub.map(str::to_string),
    };
    s.record(
        &stash_of(None),
        &json!({ "id": "m1" }),
        200,
        &json!({ "stash": { "id": "m1", "name": "Maps", "type": "MapStash", "items": [], "children": [
            { "id": "s1", "name": "Tier 1", "type": "MapStash" },
            { "id": "s2", "name": "Tier 2", "type": "MapStash" } ] } }),
        31,
    )
    .unwrap();
    for (sub, id) in [("s1", "x1"), ("s2", "x2")] {
        s.record(
            &stash_of(Some(sub)),
            &json!({ "id": "m1", "sub": sub }),
            200,
            &json!({ "stash": { "id": sub, "name": sub, "type": "MapStash", "items": [ring(id, "Rare", json!({}))] } }),
            32,
        )
        .unwrap();
    }
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "t1",
        "Rings",
        vec![ring("a", "Rare", json!({}))],
        50,
    );
    // Gone and Maps leave the list: Maps' substashes with it
    list_tabs(&mut s, "pc", "Standard", json!([tab("t1", "Rings")]), 60);
    let over = |membership: Membership, text: &str| {
        let corpus = Corpus::load_of(
            &s,
            &intent(&s),
            Some(&Realm::parse("pc").unwrap()),
            membership,
        )
        .unwrap();
        asked_at(
            &corpus,
            json!({ "scope": { "membership": membership }, "query": { "text": text } }),
        )
    };

    // the retired tab's fetch is kept: the oldest, apart from the scope's
    let a = over(Membership::All, "-id:x1 -id:x2");
    assert_eq!(ids(&a), ["a", "c"]);
    assert_eq!(a["total"]["places"], 2);
    assert_eq!(
        (
            &a["total"]["oldest_fetch"],
            &a["total"]["oldest_fetch_location"]
        ),
        (&json!(30), &stash("t2", "Gone", "Standard"))
    );
    assert_eq!(a["total"]["newest_fetch"], 50);
    assert_eq!(a["scope"]["oldest_fetch"], 50);

    // a cleared substash is a place, outside the fetch extremes
    let a = over(Membership::All, "rarity=rare");
    assert_eq!(ids(&a), ["a", "x1", "x2"]);
    assert_eq!(a["total"]["places"], 3);
    assert_eq!(
        (&a["total"]["oldest_fetch"], &a["total"]["newest_fetch"]),
        (&json!(50), &json!(50))
    );

    // such places alone: the span with no fetch, and the list that names them
    for membership in [Membership::All, Membership::Removed] {
        let a = over(membership, "id:x1 or id:x2");
        assert_eq!(
            subject(&a),
            json!({
                "places": 2,
                "oldest_fetch": null,
                "newest_fetch": null,
                "list_seen": 60,
                "list_seen_age": "8d ago",
            }),
            "{membership:?}"
        );
    }
}

/// Over every realm, the matches' lists are their own realm's: an older
/// list of the same league in another realm is the scope's alone.
#[test]
fn c162_the_matches_lists_are_their_realms_over_every_realm() {
    let mut s = store();
    list_tabs(&mut s, "pc", "Standard", json!([tab("t1", "Rings")]), 300);
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "t1",
        "Rings",
        vec![ring("r1", "Rare", json!({}))],
        310,
    );
    list_tabs(&mut s, "xbox", "Standard", json!([tab("x1", "Box")]), 100);
    fetch_tab(
        &mut s,
        "xbox",
        "Standard",
        "x1",
        "Box",
        vec![ring("m1", "Magic", json!({}))],
        110,
    );
    let a = asked_at(
        &load(&s, Some("all")),
        json!({ "query": { "text": "rarity=rare" } }),
    );
    assert_eq!(a["scope"]["realm"], "all");
    assert_eq!(
        (&a["total"]["list_seen"], &a["scope"]["list_seen"]),
        (&json!(300), &json!(100))
    );
}

/// The lists that name the matches' places: a stash place's league's tab
/// list, and the character list where a character is among them; the
/// scope's narrowing under a fixed league is its own.
#[test]
fn c162_the_matches_lists_are_those_that_name_their_places() {
    let mut s = store();
    list_tabs(&mut s, "pc", "Other", json!([tab("o1", "Elsewhere")]), 100);
    list_characters(
        &mut s,
        "pc",
        json!([{ "id": "c1", "name": "Mover", "league": "Standard" }]),
        200,
    );
    list_tabs(&mut s, "pc", "Standard", json!([tab("t1", "Rings")]), 300);
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "t1",
        "Rings",
        vec![ring("s1", "Rare", json!({}))],
        400,
    );
    fetch_character(
        &mut s,
        "pc",
        json!({ "id": "c1", "name": "Mover", "league": "Standard",
                "equipment": [ring("ch1", "Magic", json!({}))] }),
        410,
    );
    fetch_tab(
        &mut s,
        "pc",
        "Other",
        "o1",
        "Elsewhere",
        vec![ring("o-r", "Rare", json!({}))],
        420,
    );
    let corpus = load(&s, Some("pc"));
    // a stash item and a character item: the older of the two lists
    let a = asked_at(&corpus, json!({ "query": { "text": "league=Standard" } }));
    assert_eq!(ids(&a), ["ch1", "s1"]);
    assert_eq!(
        (&a["total"]["places"], &a["total"]["list_seen"]),
        (&json!(2), &json!(200))
    );
    assert_eq!(a["scope"]["list_seen"], 200);
    // stash items of one league alone: that league's tab list, where the
    // scope narrows to the league's lists, the characters' among them
    let a = asked_at(
        &corpus,
        json!({ "query": { "text": "league=Standard rarity=rare" } }),
    );
    assert_eq!(ids(&a), ["s1"]);
    assert_eq!(
        (&a["total"]["list_seen"], &a["scope"]["list_seen"]),
        (&json!(300), &json!(200))
    );
}
