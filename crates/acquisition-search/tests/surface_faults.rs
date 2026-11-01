//! Faults of the first surface found from outside (step 4b; an audit of
//! 2026-09-21 and its reviews to 2026-09-22) that the generated properties
//! had not met, each reproduced here as a failing test before it was
//! fixed: C92 (a sum is exact in any order), C93 (a number beyond reach is
//! unread at its slot; a reason is the open occurrence's own), C100 and
//! invariants 4 and 5 of the surface (a count has a route; every block is
//! bounded), C97 (`--describe` is the help).

use crate::common;

use acquisition_search::describe;
use common::generated::{fixture, request, run};
use serde_json::{Value, json};

fn lines(texts: &[&str]) -> Value {
    json!({ "explicitMods": texts })
}

/// A sum is its occurrences' sum, in whatever order the body lists them:
/// 0.1 + 0.2 + 0.3 is 0.6, never 0.6000000000000001, so `=0.6` matches
/// both items and both sort by one value. The mean of a ranged pair is a
/// sum too.
#[test]
fn c92_a_sum_of_decimals_is_exact_and_no_order_of_occurrences_changes_it() {
    let leech = |values: [&str; 3]| {
        let texts: Vec<String> = values
            .iter()
            .map(|v| format!("{v}% of Damage Leeched as Life"))
            .collect();
        json!({ "explicitMods": texts })
    };
    let (corpus, _) = fixture(vec![
        leech(["0.1", "0.2", "0.3"]),
        leech(["0.3", "0.2", "0.1"]),
        lines(&["Adds 0.1 to 0.2 Cold Damage"]),
    ]);
    let ids = |query: &str, sort: Option<&str>| -> Vec<(String, Value)> {
        let answer = run(&corpus, &request(query, sort, false, 10)).unwrap();
        let mut rows: Vec<(String, Value)> = answer["rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                (
                    r["id"].as_str().unwrap().to_string(),
                    r["sort"]["value"].clone(),
                )
            })
            .collect();
        rows.sort_by(|a, b| a.0.cmp(&b.0));
        rows
    };
    let sum = "sum(\"#% of Damage Leeched as Life\")";
    let matched: Vec<String> = ids(&format!("{sum}=0.6"), None)
        .into_iter()
        .map(|r| r.0)
        .collect();
    assert_eq!(matched, ["i0", "i1"]);
    let sorted = ids("\"#% of Damage Leeched as Life\"", Some(sum));
    assert_eq!(sorted[0].1, json!(0.6));
    assert_eq!(sorted[1].1, json!(0.6));
    // reaching a bound only together is a sum as well
    let together = run(
        &corpus,
        &request(
            "line(\"#% of Damage Leeched as Life\" arg1>=0.6)",
            None,
            false,
            10,
        ),
    )
    .unwrap();
    assert_eq!(together["terms"][0]["together"]["count"], 2);
    // (0.1 + 0.2) / 2 is 0.15
    let avg: Vec<String> = ids("line(\"Adds # to # Cold Damage\" avg=0.15)", None)
        .into_iter()
        .map(|r| r.0)
        .collect();
    assert_eq!(avg, ["i2"]);
}

/// A suggestion's count is the count of the term it offers (invariant 4):
/// `=` is any-case, so two spellings of one line are one suggestion, and
/// asking it returns as many items as it said.
#[test]
fn c100_a_suggestion_counts_what_its_term_returns() {
    let (corpus, _) = fixture(vec![
        lines(&["Gain 5 Life per Enemy Killed"]),
        lines(&["Gain 7 Life per enemy killed"]),
        lines(&[
            "Gain 9 Life per Enemy Killed",
            "Gain 1 Life per enemy killed",
        ]),
    ]);
    let answer = run(
        &corpus,
        &request("line(\"Gain # Life per Enemy Kiledd\")", None, false, 10),
    )
    .unwrap();
    let suggestions = answer["zero"]["resolved_to_nothing"][0]["suggestions"]
        .as_array()
        .unwrap();
    assert_eq!(
        suggestions.len(),
        1,
        "one line, however GGG spelled it: {suggestions:?}"
    );
    for s in suggestions {
        let asked = run(
            &corpus,
            &request(s["term"].as_str().unwrap(), None, false, 10),
        )
        .unwrap();
        assert_eq!(asked["total"]["matched"], s["items"], "{s}");
    }
    assert_eq!(suggestions[0]["items"], 3);
    // a field's values too
    let (corpus, _) = fixture(vec![
        json!({ "name": "Doom Knot" }),
        json!({ "name": "DOOM KNOT" }),
    ]);
    let answer = run(&corpus, &request("name=\"Doom Knott\"", None, false, 10)).unwrap();
    let suggestions = answer["zero"]["resolved_to_nothing"][0]["suggestions"]
        .as_array()
        .unwrap();
    assert_eq!(suggestions.len(), 1, "{suggestions:?}");
    assert_eq!(suggestions[0]["items"], 2);
}

/// A selector that resolved to nothing is said to, whether or not it has
/// words a suggestion could be scored against: a pattern has none, and is
/// listed with no suggestion.
#[test]
fn c100_a_pattern_that_resolved_to_nothing_is_in_the_zero_block() {
    let (corpus, _) = fixture(vec![
        lines(&["+5 to maximum Life"]),
        json!({ "name": "Doom Knot" }),
    ]);
    for (query, of) in [
        ("line(template:nothing)", "template"),
        ("line(template~\"nothing\")", "template"),
        ("sum(line(template~\"nothing\").arg1)>0", "template"),
        ("name~\"nothing\"", "name"),
        ("name:nothing", "name"),
        // found by the zero-block cross-check on its first day
        ("rarity:ma", "rarity"),
    ] {
        let answer = run(&corpus, &request(query, None, false, 10)).unwrap();
        let nothing = answer["zero"]["resolved_to_nothing"].as_array().unwrap();
        assert_eq!(nothing.len(), 1, "`{query}`: {nothing:?}");
        assert_eq!(nothing[0]["of"], of, "`{query}`");
    }
    // a pattern that found something is not
    let answer = run(
        &corpus,
        &request("line(template~\"life\" arg1>=90)", None, false, 10),
    )
    .unwrap();
    assert_eq!(answer["zero"]["resolved_to_nothing"], json!([]));
}

/// What a row shows of one term is bounded, and says what it left out
/// (invariant 5); `acq show <id>` is the route to the whole.
#[test]
fn c100_a_rows_evidence_is_bounded_and_says_what_it_left_out() {
    let forty: Vec<String> = (1..=40).map(|n| format!("+{n} to maximum Life")).collect();
    let (corpus, _) = fixture(vec![json!({ "explicitMods": forty })]);
    for (query, touched) in [
        ("line(template:life)", 40),
        ("sum(\"# to maximum Life\")>0", 41),
        ("text:life", 40),
    ] {
        let answer = run(&corpus, &request(query, None, false, 10)).unwrap();
        let matched = &answer["rows"][0]["matched"][0];
        let shows = matched["shows"].as_array().unwrap().len();
        assert!(shows <= 7, "`{query}` shows {shows}");
        assert_eq!(
            shows as u64 + matched["left_out"].as_u64().unwrap(),
            touched,
            "`{query}`: {matched}"
        );
    }
    // nothing left out is not said
    let (corpus, _) = fixture(vec![lines(&["+5 to maximum Life"])]);
    let answer = run(&corpus, &request("line(template:life)", None, false, 10)).unwrap();
    assert!(answer["rows"][0]["matched"][0].get("left_out").is_none());
}

/// `--describe` is the help C97 names: it says how terms compose and what
/// has a value, and a word it prints can be asked for alone.
#[test]
fn c97_describe_says_how_terms_compose_and_knows_the_words_it_prints() {
    let whole = serde_json::to_value(describe(&["all".to_string()]).unwrap()).unwrap();
    let names = |block: &str| -> Vec<String> {
        whole[block]
            .as_array()
            .unwrap_or_else(|| panic!("no `{block}` block"))
            .iter()
            .map(|n| n["name"].as_str().unwrap().to_string())
            .collect()
    };
    let composition = names("composition").join(" ");
    for word in ["and", "or", "not", "holds", "undecided", "true()"] {
        assert!(
            composition.contains(word),
            "composition lacks `{word}`: {composition}"
        );
    }
    let values = names("values").join(" ");
    for word in ["sum", "line(P).<slot>"] {
        assert!(values.contains(word), "values lacks `{word}`: {values}");
    }
    // every example the description prints is a query this build binds,
    // read from the whole: the index prints none, so the loop is held to
    // have checked some
    let mut checked = 0;
    for block in ["fields", "line", "composition", "values"] {
        for entry in whole[block].as_array().unwrap() {
            for example in entry["examples"].as_array().into_iter().flatten() {
                acquisition_search::parse_query(example.as_str().unwrap())
                    .unwrap_or_else(|e| panic!("{example}: {e}"));
                checked += 1;
            }
        }
    }
    assert!(checked > 0, "no example was checked: {whole}");
    for (word, block) in [
        ("arg1", "slots"),
        ("arg7", "slots"),
        ("avg", "slots"),
        ("=", "operators"),
        (">=", "operators"),
        ("holds", "composition"),
        ("sum", "values"),
    ] {
        let one = serde_json::to_value(describe(&[word.to_string()]).unwrap())
            .unwrap_or_else(|_| panic!("`{word}`"));
        assert_eq!(one[block].as_array().unwrap().len(), 1, "`{word}`: {one}");
    }
    // The league seat (cli calls 57, 58): the language's refusal spells
    // its calls with their `(` (`sum(`, `holds(`), and describe refused
    // `sum(` as nothing it knows. Every call the refusal spells reaches
    // the entry or the block that holds it, as its bare name does.
    let readings = acquisition_search::parse_query("nocall(x)")
        .unwrap_err()
        .readings;
    assert!(readings.contains(&"sum(".to_string()), "{readings:?}");
    for call in &readings {
        let spelled =
            describe(std::slice::from_ref(call)).unwrap_or_else(|e| panic!("`{call}`: {e}"));
        let bare = call.trim_end_matches('(');
        let named = describe(&[bare.to_string()]).unwrap_or_else(|e| panic!("`{bare}`: {e}"));
        assert_eq!(
            serde_json::to_value(spelled).unwrap(),
            serde_json::to_value(named).unwrap(),
            "`{call}` reads as `{bare}`"
        );
    }
}

/// Outside review, 2026-09-21: a number is read once, where the body is
/// read, and one the search does not read — more digits than any game
/// displays — is an unread slot of its line (C93), never a sum that is
/// exact in one order of occurrences and rounded in another. So both
/// orders answer alike, and say why.
#[test]
fn c93_a_number_beyond_reach_is_unread_where_it_is_read_and_no_order_matters() {
    let huge = "10000000000000000000000000000";
    let life = |n: &str| format!("{n} to maximum Life");
    let (corpus, _) = fixture(vec![
        json!({ "explicitMods": [life(&format!("+{huge}")), life(&format!("+{huge}")),
            life(&format!("-{huge}")), life(&format!("-{huge}")), life("+0.0000000001")] }),
        json!({ "explicitMods": [life(&format!("+{huge}")), life(&format!("-{huge}")),
            life(&format!("+{huge}")), life(&format!("-{huge}")), life("+0.0000000001")] }),
        lines(&[
            "+1234567890.1234 to maximum Life",
            "-1234567890.1234 to maximum Life",
            "+0.0001 to maximum Life",
        ]),
    ]);
    let sum = "sum(\"# to maximum Life\")";
    let answer = run(&corpus, &request(&format!("{sum}>0"), Some(sum), false, 10)).unwrap();
    assert_eq!(
        answer["terms"][0]["undecided"]["count"], 2,
        "{}",
        answer["terms"][0]
    );
    // the largest number read whole: ten digits and four decimals, exactly
    assert_eq!(answer["rows"].as_array().unwrap().len(), 1);
    assert_eq!(answer["rows"][0]["id"], "i2");
    assert_eq!(answer["rows"][0]["sort"]["value"], json!(0.0001));
    let why = &answer["total"]["undecided_reasons"][0];
    assert_eq!(why["unread"], "the numbers of explicit lines");
    assert!(why["problem"].as_str().unwrap().contains("digits"), "{why}");
    // its text is a witness all the same
    let phrase = run(&corpus, &request("\"maximum Life\"", None, false, 10)).unwrap();
    assert_eq!(phrase["total"]["matched"], 3);
}

/// The follow-up, 2: what is shown of why an item is undecided is bounded
/// by the item's unread parts, in the item's order — never by the order
/// the terms were written in, which a rewrite that changes no meaning may
/// change (invariant 7). On an `undecided( … )` row, that is; the answer's
/// own block is by reason since the first seat (V8), seven kinds here,
/// none left out, in the same order either way.
#[test]
fn c100_the_reasons_shown_are_the_items_first_six_however_the_terms_are_ordered() {
    let flags = [
        "corrupted",
        "identified",
        "split",
        "duplicated",
        "replica",
        "fractured",
        "mutated",
    ];
    let mut body = json!({});
    for flag in flags {
        body[flag] = json!("unread");
    }
    let (corpus, _) = fixture(vec![body]);
    let shown = |order: &[&str]| -> (Vec<String>, Value, Vec<String>, Value) {
        let terms: Vec<String> = order.iter().map(|f| format!("is:{f}")).collect();
        let probe = run(
            &corpus,
            &request(&format!("undecided({})", terms.join(" ")), None, false, 10),
        )
        .unwrap();
        let matched = &probe["rows"][0]["matched"][0];
        let mut unread: Vec<String> = matched["shows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| e["undecided"]["unread"].as_str().unwrap().to_string())
            .collect();
        unread.sort();
        let root = run(&corpus, &request(&terms.join(" "), None, false, 10)).unwrap();
        let why: Vec<String> = root["total"]["undecided_reasons"]
            .as_array()
            .unwrap()
            .iter()
            .map(|w| w["unread"].as_str().unwrap().to_string())
            .collect();
        (
            unread,
            matched["left_out"].clone(),
            why,
            root["total"]["reasons_left_out"].clone(),
        )
    };
    let forward = shown(&flags);
    let mut reversed = flags;
    reversed.reverse();
    assert_eq!(forward, shown(&reversed));
    assert_eq!(forward.0.len(), 6);
    assert_eq!(forward.1, json!(1));
    assert_eq!(forward.2.len(), 7);
    assert!(forward.3.is_null());
}

/// Outside review, 2026-09-22: a number the search does not read is an
/// unread *slot* — not an absent one, and not an unread array. What needs
/// the number is undecided, through a not as well; what needs the line's
/// text, template, source or sibling numbers keeps its answer (C93: only
/// the terms that needed it).
#[test]
fn c93_an_unread_number_is_unknown_to_what_asks_it_and_to_nothing_else() {
    let (corpus, _) = fixture(vec![
        lines(&["1.12345 to Spirit"]),
        lines(&["Adds 1 to 10000000000 Cold Damage"]),
    ]);
    // (matched, undecided at the root) of a query asked of one item
    let of = |id: &str, query: &str| -> (u64, u64) {
        let a = run(
            &corpus,
            &request(&format!("id:{id} ({query})"), None, false, 10),
        )
        .unwrap();
        (
            a["total"]["matched"].as_u64().unwrap(),
            a["total"]["undecided"]["count"].as_u64().unwrap(),
        )
    };
    const MATCHED: (u64, u64) = (1, 0);
    const UNDECIDED: (u64, u64) = (0, 1);
    const NO: (u64, u64) = (0, 0);
    for (id, query, want) in [
        ("i0", "line(\"# to Spirit\" arg1>=0)", UNDECIDED),
        ("i0", "line(\"# to Spirit\" -arg1>=0)", UNDECIDED),
        ("i0", "-line(\"# to Spirit\" arg1>=0)", UNDECIDED),
        ("i0", "sum(\"# to Spirit\")>=0", UNDECIDED),
        ("i0", "undecided(line(\"# to Spirit\").arg1)", MATCHED),
        // the line is there, and no other is
        ("i0", "\"# to Spirit\"", MATCHED),
        ("i0", "line(template:spirit source=explicit)", MATCHED),
        ("i0", "-line(template:life)", MATCHED),
        ("i0", "line(template:life arg1>=0)", NO),
        // a sibling number that was read is a witness, and a wrong one is a no
        ("i1", "line(\"Adds # to # Cold Damage\" arg1=1)", MATCHED),
        ("i1", "line(\"Adds # to # Cold Damage\" low=2)", NO),
        ("i1", "line(\"Adds # to # Cold Damage\" high>=5)", UNDECIDED),
        ("i1", "line(\"Adds # to # Cold Damage\" avg>=5)", UNDECIDED),
        ("i1", "line(\"Adds # to # Cold Damage\" low=2 high>=5)", NO),
    ] {
        assert_eq!(of(id, query), want, "{id}: `{query}`");
    }
    // and the item says which numbers, of which lines
    let a = run(
        &corpus,
        &request("line(\"# to Spirit\" arg1>=0)", None, false, 10),
    )
    .unwrap();
    let why = &a["total"]["undecided_reasons"][0];
    assert_eq!(why["unread"], "the numbers of explicit lines", "{why}");
}

/// The third review, 2: a sum past what a float holds exactly is still the
/// decimal sum, read once: nineteen of the largest numbers the search
/// reads equal the bound typed for them.
#[test]
fn c92_a_sum_at_the_edge_of_what_is_read_equals_its_typed_bound() {
    let nineteen: Vec<String> = (0..19)
        .map(|_| "9999999999.9997 to Spirit".to_string())
        .collect();
    let (corpus, _) = fixture(vec![json!({ "explicitMods": nineteen })]);
    let sum = "sum(\"# to Spirit\")";
    let a = run(
        &corpus,
        &request(&format!("{sum}=189999999999.9943"), Some(sum), false, 10),
    )
    .unwrap();
    assert_eq!(a["total"]["matched"], 1);
    assert_eq!(a["rows"][0]["sort"]["value"], json!(189999999999.9943));
}

/// Found by the completion property the day its generators learned to
/// write an unread number, and nothing to do with one: an item "reaches a
/// bound only together" when its occurrences' sum does — and a sum of no
/// occurrence is no such thing, though 0 is at least any bound of zero or
/// less. The count said 1 and its route, which asks for a selected line,
/// returned none (invariant 4).
#[test]
fn c92_nothing_reaches_a_bound_together_without_an_occurrence_that_counts() {
    let (corpus, _) = fixture(vec![
        json!({ "explicitMods": [{ "description": "Cannot be Frozen", "flags": "unread" }] }),
        lines(&["-3 to maximum Life", "-4 to maximum Life"]),
    ]);
    // the selector is open on the first item's one line, which names no
    // number: the group failed there, and nothing of it sums
    let a = run(
        &corpus,
        &request(
            "line(template~\"Frozen|Life\" is:crafted arg1>=-5)",
            None,
            false,
            10,
        ),
    )
    .unwrap();
    assert_eq!(a["terms"][0]["failed"]["count"], 1, "{}", a["terms"][0]);
    assert_eq!(a["terms"][0]["together"]["count"], 0);
    // two that do sum: -3 and -4 are each under -2, and no sum of them is over
    let a = run(
        &corpus,
        &request("line(template:life arg1>=-2)", None, false, 10),
    )
    .unwrap();
    assert_eq!(a["terms"][0]["together"]["count"], 0);
    let a = run(
        &corpus,
        &request("line(template:life arg1<=-5)", None, false, 10),
    )
    .unwrap();
    assert!(
        a["terms"][0].get("together").is_none(),
        "an upper bound: not applicable"
    );
}

/// Outside review, 2026-09-22: why an item is undecided names the occurrence
/// that left it open — never another of the same array that happens to be
/// unread too. Six Spirit lines nobody asked about come before the one
/// Life line that matters; the reason given is that line's, `[6]`, and
/// nothing relevant is left out. Both paths: the total's `why` and an
/// `undecided( … )` row.
#[test]
fn c93_the_reason_given_is_the_open_occurrences_own() {
    let unread_number = |of: &str| json!(format!("1.12345 to {of}"));
    let unread_flags =
        |of: &str| json!({ "description": format!("+1 to {of}"), "flags": "unread" });
    for (make, query, unread) in [
        (
            &unread_number as &dyn Fn(&str) -> Value,
            "line(template:life arg1>=0)",
            "the numbers of explicit lines",
        ),
        (
            &unread_flags,
            "line(template:life is:crafted)",
            "the flags of explicit lines",
        ),
    ] {
        let mut mods: Vec<Value> = (0..6).map(|_| make("Spirit")).collect();
        mods.push(make("maximum Life"));
        let (corpus, _) = fixture(vec![json!({ "explicitMods": mods })]);
        let total = run(&corpus, &request(query, None, false, 10)).unwrap();
        let probe = run(
            &corpus,
            &request(&format!("undecided({query})"), None, false, 10),
        )
        .unwrap();
        let row = &probe["rows"][0]["matched"][0];
        for (reasons, left_out) in [
            (
                total["total"]["undecided_reasons"].as_array().unwrap(),
                &total["total"]["reasons_left_out"],
            ),
            (row["shows"].as_array().unwrap(), &row["left_out"]),
        ] {
            assert_eq!(reasons.len(), 1, "`{query}`: {reasons:?}");
            let reason = reasons[0].get("undecided").unwrap_or(&reasons[0]);
            assert_eq!(reason["unread"], unread, "`{query}`");
            assert!(
                reason["problem"]
                    .as_str()
                    .unwrap()
                    .contains("explicitMods[6]"),
                "`{query}`: {reason}"
            );
            assert!(left_out.is_null(), "`{query}`: {left_out} left out");
        }
    }
    // a line open for its flags is not explained by its number, nor the
    // other way round
    let both = json!({ "description": "1.12345 to maximum Life", "flags": "unread" });
    let (corpus, _) = fixture(vec![json!({ "explicitMods": [both] })]);
    for (query, unread) in [
        (
            "line(template:life is:crafted)",
            "the flags of explicit lines",
        ),
        (
            "line(template:life arg1>=0)",
            "the numbers of explicit lines",
        ),
    ] {
        let a = run(&corpus, &request(query, None, false, 10)).unwrap();
        let why = a["total"]["undecided_reasons"].as_array().unwrap();
        assert_eq!(why.len(), 1, "`{query}`: {why:?}");
        assert_eq!(why[0]["unread"], unread, "`{query}`");
    }
}

/// C97 (`--describe` is the help): the sections a frontend lists are the
/// whole answer's own keys, and each asked for alone
/// answers with its key filled. The MCP's `names` listed ten and left out
/// `trade`, which its instructions tell an agent to send, and `totals`.
#[test]
fn c97_the_sections_listed_are_the_answers_keys_and_each_answers_alone() {
    let whole = serde_json::to_value(describe::describe(&["all".to_string()]).unwrap()).unwrap();
    let mut keys: Vec<&str> = whole
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    let mut sections = describe::SECTIONS.to_vec();
    keys.sort_unstable();
    sections.sort_unstable();
    assert_eq!(keys, sections);
    for section in describe::SECTIONS {
        let one = serde_json::to_value(describe::describe(&[section.to_string()]).unwrap())
            .unwrap_or_else(|e| panic!("{section}: {e}"));
        let filled = match &one[section] {
            Value::Array(entries) => !entries.is_empty(),
            Value::String(text) => !text.is_empty(),
            Value::Object(block) => !block.is_empty(),
            _ => false,
        };
        assert!(filled, "{section} alone answers without it: {one}");
    }
    // C53 (JSON is the contract): a narrowed answer keeps every key the
    // whole answer always carried, `totals` among them
    let fields =
        serde_json::to_value(describe::describe(&["fields".to_string()]).unwrap()).unwrap();
    assert_eq!(fields["totals"], whole["totals"], "{fields}");
    // the index's keys are the sections less `trade`, which it names by
    // its count of rows, and how to go on
    let index = serde_json::to_value(describe::describe(&[]).unwrap()).unwrap();
    let mut keys: Vec<&str> = index
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    let mut expected: Vec<&str> = describe::SECTIONS
        .into_iter()
        .filter(|s| *s != "trade")
        .chain(["trade_rows", "next"])
        .collect();
    keys.sort_unstable();
    expected.sort_unstable();
    assert_eq!(keys, expected);
}

/// C53 as amended (a document read at two densities is the decision view
/// where nothing is named): `describe` with nothing named is the index —
/// every entry the whole names, in the whole's order, at one line: its
/// name, its kind and its definition up to the first `: `, no values and
/// no examples; each category by its id and the site's text; the trade
/// catalogue by its count of rows; and how to go on. Every name it
/// prints answers whole when asked alone — an entry, a section, a
/// category's id — and a construct not built is still refused by name.
#[test]
fn c53_the_index_is_every_entry_at_one_line_and_each_name_it_prints_answers_whole() {
    let index = serde_json::to_value(describe::describe(&[]).unwrap()).unwrap();
    let whole = serde_json::to_value(describe::describe(&["all".to_string()]).unwrap()).unwrap();
    const ENTRIES: [&str; 11] = [
        "fields",
        "line",
        "linked",
        "values",
        "composition",
        "operators",
        "slots",
        "computed",
        "counts",
        "explain",
        "answer",
    ];
    let names = |d: &Value, section: &str| -> Vec<String> {
        d[section]
            .as_array()
            .unwrap_or_else(|| panic!("no `{section}`"))
            .iter()
            .map(|n| n["name"].as_str().unwrap().to_string())
            .collect()
    };
    let mut asked: Vec<(String, &str)> = Vec::new();
    for section in ENTRIES {
        assert_eq!(names(&index, section), names(&whole, section), "{section}");
        for (entry, full) in index[section]
            .as_array()
            .unwrap()
            .iter()
            .zip(whole[section].as_array().unwrap())
        {
            let what = entry["what"].as_str().unwrap();
            assert!(!what.contains(": "), "{section} {entry}");
            assert_eq!(
                what,
                describe::one_line(full["what"].as_str().unwrap()),
                "{section} {entry}"
            );
            assert_eq!(entry["kind"], full["kind"], "{section} {entry}");
            assert!(entry.get("values").is_none(), "{section} {entry}");
            assert!(entry.get("examples").is_none(), "{section} {entry}");
            asked.push((entry["name"].as_str().unwrap().to_string(), section));
        }
        asked.push((section.to_string(), section));
    }
    let ids: Vec<&str> = index["categories"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| {
            assert!(c.get("holds").is_none(), "{c}");
            assert!(c["text"].as_str().is_some_and(|t| !t.is_empty()), "{c}");
            c["id"].as_str().unwrap()
        })
        .collect();
    let whole_ids: Vec<&str> = whole["categories"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["id"].as_str().unwrap())
        .collect();
    assert_eq!(ids, whole_ids);
    assert!(index.get("trade").is_none(), "{index}");
    let rows = acquisition_search::trade::catalogue::catalogue()
        .unwrap()
        .rows()
        .len();
    assert_eq!(index["trade_rows"], rows);
    assert!(index["next"].as_str().is_some_and(|n| !n.is_empty()));
    assert_eq!(index["not_built"], whole["not_built"]);
    assert_eq!(index["limits"], whole["limits"]);
    // every name the index prints answers whole, alone
    for (name, section) in &asked {
        let one = serde_json::to_value(
            describe::describe(std::slice::from_ref(name))
                .unwrap_or_else(|e| panic!("`{name}`: {e}")),
        )
        .unwrap();
        assert!(one.get("next").is_none(), "`{name}`: {one}");
        let found = one[section]
            .as_array()
            .unwrap()
            .iter()
            .any(|e| whole[section].as_array().unwrap().contains(e));
        assert!(found, "`{name}` answers no entry of {section} whole: {one}");
    }
    for id in &ids {
        let one = serde_json::to_value(describe::describe(&[id.to_string()]).unwrap()).unwrap();
        let row = whole["categories"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["id"] == *id)
            .unwrap();
        assert!(
            one["categories"].as_array().unwrap().contains(row),
            "`{id}`: {one}"
        );
    }
    for unbuilt in acquisition_search::NOT_BUILT {
        let construct = unbuilt.construct.trim_end_matches([':', '*', '.']);
        let e = describe::describe(&[construct.to_string()]).unwrap_err();
        assert_eq!(e.kind(), "not_built", "`{construct}`: {e}");
    }
}

/// The entries of the eleven sections an answer holds, and the
/// categories, as JSON values: what a slice's count counts.
fn sliced(d: &Value) -> Vec<Value> {
    const ENTRIES: [&str; 11] = [
        "fields",
        "line",
        "linked",
        "values",
        "composition",
        "operators",
        "slots",
        "computed",
        "counts",
        "explain",
        "answer",
    ];
    ENTRIES
        .iter()
        .flat_map(|s| d[s].as_array().cloned().unwrap_or_default())
        .chain(d["categories"].as_array().cloned().unwrap_or_default())
        .collect()
}

/// `describe` with `items`, as JSON.
fn described(items: &[&str]) -> Value {
    let items: Vec<String> = items.iter().map(|i| i.to_string()).collect();
    serde_json::to_value(describe::describe(&items).unwrap_or_else(|e| panic!("{items:?}: {e}")))
        .unwrap()
}

/// #154, a slice by pattern: `~` and a pattern reads the language's
/// entries by name and by definition, the categories by id, text and what
/// makes one hold, and answers its matches whole where they are the
/// store's count or fewer (ten) and at one line each past it, counted.
/// `~resistance` matches ten and `~resist` eleven over the tables this
/// build ships (the eleventh the answer's `view`, whose definition names
/// `line:resist`): the bound seen from both sides.
#[test]
fn c53_a_pattern_answers_ten_matches_whole_and_eleven_at_one_line() {
    let whole = described(&["all"]);
    let every = sliced(&whole);
    for (pattern, count, is_whole) in [("~resistance", 10, true), ("~resist", 11, false)] {
        let d = described(&[pattern]);
        assert_eq!(
            d["patterns"],
            json!([{ "pattern": pattern, "matched": count, "whole": is_whole }]),
            "{pattern}"
        );
        let entries = sliced(&d);
        assert_eq!(entries.len(), count, "{pattern}: {d}");
        for entry in &entries {
            match is_whole {
                true => assert!(every.contains(entry), "{pattern}: not whole: {entry}"),
                false => {
                    let what = entry["what"].as_str().unwrap();
                    assert!(!what.contains(": "), "{pattern}: {entry}");
                    assert!(entry.get("examples").is_none(), "{pattern}: {entry}");
                }
            }
        }
        assert!(d["next"].as_str().is_some_and(|n| !n.is_empty()), "{d}");
        assert!(d.get("trade").is_none() && d.get("trade_rows").is_none());
    }
    // the wide pattern: lines only, its count every entry it matched
    let wide = described(&["~a"]);
    assert_eq!(wide["patterns"][0]["whole"], false);
    assert_eq!(wide["patterns"][0]["matched"], sliced(&wide).len());
    assert!(sliced(&wide).len() > 100, "{}", sliced(&wide).len());
    // case: any case unless the pattern says (?-i)
    assert_eq!(described(&["~RESIST"])["patterns"][0]["matched"], 11);
    let cased = described(&["~(?-i)Resist"]);
    for entry in sliced(&cased) {
        let full = every
            .iter()
            .find(|e| e["name"] == entry["name"] && e["kind"] == entry["kind"])
            .unwrap();
        assert!(
            full["name"].as_str().unwrap().contains("Resist")
                || full["what"].as_str().unwrap().contains("Resist"),
            "{entry}"
        );
    }
    assert!(
        cased["patterns"][0]["matched"].as_u64() < Some(11),
        "{cased}"
    );
}

/// #154: a pattern reads an entry's definition whole, not its one line —
/// `critical_strike_chance` says "weapon" after its first `: ` — and the
/// name and the definition each as its own string, so `^` and `$` are the
/// ends of one: `^GGG's` finds `name`, whose definition starts so, and
/// not `ilvl`, whose definition holds the words later and whose name does
/// not; `_res$` the computed values whose name ends so and `sum`, whose
/// definition does. A name beside a
/// pattern answers whole; the bare `~` is the operator's entry; the trade
/// catalogue's rows are never read.
#[test]
fn c97_a_pattern_reads_names_and_definitions_each_its_own_string() {
    let names = |d: &Value| -> Vec<String> {
        sliced(d)
            .iter()
            .map(|e| e["name"].as_str().or(e["id"].as_str()).unwrap().to_string())
            .collect()
    };
    let weapon = described(&["~weapon"]);
    assert!(names(&weapon).contains(&"critical_strike_chance".to_string()));
    let line = described(&[])["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == "critical_strike_chance")
        .unwrap()["what"]
        .clone();
    assert!(!line.as_str().unwrap().contains("weapon"), "{line}");

    let anchored = names(&described(&["~^GGG's"]));
    assert!(anchored.contains(&"name".to_string()), "{anchored:?}");
    assert!(!anchored.contains(&"ilvl".to_string()), "{anchored:?}");
    assert!(names(&described(&["~GGG's"])).contains(&"ilvl".to_string()));
    // `$` ends the name for the totals, and `sum`'s definition, which
    // ends on `pseudo.total_res`
    let ends = described(&["~_res$"]);
    assert_eq!(ends["patterns"][0]["whole"], true, "{ends}");
    for entry in sliced(&ends) {
        assert!(
            entry["name"].as_str().unwrap().ends_with("_res")
                || entry["what"].as_str().unwrap().ends_with("_res"),
            "{entry}"
        );
    }
    let ends = names(&ends);
    assert!(ends.contains(&"pseudo.total_res".to_string()), "{ends:?}");
    assert!(ends.contains(&"sum".to_string()), "{ends:?}");
    assert!(
        names(&described(&["~^pseudo\\.total"]))
            .iter()
            .all(|n| n.starts_with("pseudo.total"))
    );

    // a name beside a pattern, each read on its own
    let both = described(&["league", "~life"]);
    assert!(names(&both).contains(&"league".to_string()), "{both}");
    assert_eq!(both["patterns"][0]["pattern"], "~life");
    // the bare `~` is the operator's own entry, whole, as before
    let bare = described(&["~"]);
    assert!(bare.get("patterns").is_none(), "{bare}");
    assert_eq!(bare["operators"][0]["name"], "~");
    assert!(
        bare["operators"][0]["what"]
            .as_str()
            .unwrap()
            .contains(": ")
    );
    // the trade rows are a block, reached by its name: `pseudo_total`
    // keys a row and matches the computed values' definitions, never the row
    let total = described(&["~pseudo_total"]);
    assert!(total.get("trade").is_none(), "{total}");
    assert!(!sliced(&total).is_empty());
}

/// #154: a pattern that matches nothing names itself and what a pattern
/// reads, as `unknown_name` (the reader's move is a name's: ask again),
/// with no readings and one offer, `describe` with no names, the index;
/// one that does not compile is the compiler's own refusal.
#[test]
fn c53_a_pattern_that_matches_nothing_offers_the_index() {
    let e = describe::describe(&["~qqqqq".to_string()]).unwrap_err();
    assert_eq!(e.kind(), "unknown_name");
    let acquisition_search::SearchError::Scope {
        message, offers, ..
    } = &e
    else {
        panic!("{e}");
    };
    assert!(message.starts_with("`~qqqqq` matches nothing"), "{message}");
    assert_eq!(offers.len(), 1, "{e}");
    assert_eq!(offers[0].describe, Some(Vec::new()));
    assert!(offers[0].request.is_none());
    let json = e.to_json();
    assert!(json.get("readings").is_none(), "{json}");
    let e = describe::describe(&["~(".to_string()]).unwrap_err();
    assert_eq!(e.kind(), "bad_pattern");
    assert!(
        e.to_string()
            .starts_with("`~` takes a pattern in Rust regex syntax, and this is not one:"),
        "{e}"
    );
}

/// #83: the anatomy of an answer left the MCP's tool text for the
/// `answer` section, read on demand, so the section is held to the
/// answer: every key an answer prints — with rows, with a zero total and
/// its offers, under the corpus context with the query set aside — is an
/// entry's name or the head of one's path.
#[test]
fn the_answer_section_names_every_key_an_answer_prints() {
    let (corpus, _) = fixture(vec![lines(&["+20 to maximum Life"])]);
    let section = serde_json::to_value(describe::describe(&["answer".to_string()]).unwrap())
        .unwrap()["answer"]
        .clone();
    let named: Vec<&str> = section
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e["name"].as_str().unwrap())
        .collect();
    let mut matched = request("\"# to maximum Life\">=10", None, false, 10);
    let zero = request("\"# to maximum Life\">=100", None, false, 10);
    let mut corpus_context = request("\"# to maximum Life\">=100", None, false, 10);
    corpus_context["context"] = json!("corpus");
    matched["view"]["routes"] = json!(true);
    let mut seen = std::collections::BTreeSet::new();
    for asked in [&matched, &zero, &corpus_context] {
        let answer = run(&corpus, asked).unwrap_or_else(|e| panic!("{asked}: {e}"));
        seen.extend(answer.as_object().unwrap().keys().cloned());
    }
    for key in ["rows", "zero", "set_aside"] {
        assert!(
            seen.contains(key),
            "the answers printed no `{key}`: {seen:?}"
        );
    }
    for key in &seen {
        assert!(
            named
                .iter()
                .any(|n| n == key || n.split('.').next() == Some(key.as_str())),
            "the answer section names no `{key}`: {named:?}"
        );
    }
}
