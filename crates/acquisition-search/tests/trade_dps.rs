//! The damage at 20 % quality against the trade site (C101; step 7e of
//! `search/TRADE-PLAN.md`, the oracle's finding `dps-at-quality`): the
//! site's Physical DPS and Damage per Second put 20 % quality in (t41,
//! t42), and its own figures for every weapon it sent are in the captures
//! (`extended.pdps`, `dps`, `edps`). Each listing is entered as the site
//! sent it (`search/pseudo-stats/data/bodies.json`), the build's
//! `pseudo.pdps_q20`, `pseudo.dps_q20` and `pseudo.edps` read by a sort,
//! and each set against the site's.
//!
//! The site works from the rolls the display has rounded, as it does for a
//! defence (S26; `tests/properties.rs`), so the physical range at 20 % is
//! within one of its own at the mean, and never claims to be its number:
//! the catalogue calls the two filters ambiguous. Quality raises no
//! elemental damage, and `pseudo.edps` is the site's figure exactly.

mod common;

use std::collections::BTreeMap;

use acquisition_search::{Request, answer};
use common::*;
use serde_json::{Value, json};

const CAPTURES: &str = include_str!("../../../search/pseudo-stats/data/captures.json");
const BODIES: &str = include_str!("../../../search/pseudo-stats/data/bodies.json");

/// A sort's value on every row, by the row's id.
fn values(corpus: &acquisition_search::Corpus, field: &str) -> BTreeMap<String, Value> {
    let request: Request = serde_json::from_value(
        json!({ "query": { "text": "" }, "view": { "rows": { "sort": field, "limit": 100_000 } } }),
    )
    .unwrap();
    as_json(&answer(corpus, &request).unwrap())["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| (r["id"].as_str().unwrap().to_string(), scalar(&r["sort"])))
        .collect()
}

/// The step 7e acceptance for `dps-at-quality`: over every listing the
/// sittings fetched that carries the site's dps — 1,195 weapons, 1,095 of
/// them displaying physical damage and an attack speed — the build's
/// physical mean at 20 % quality is the site's on 902, a half off on 181
/// and one off on 12, and never further (the site's figure over the attack
/// speed); `dps_q20`, the elemental and chaos ranges as displayed, stands
/// to the site's `dps` with the same spread; and `pseudo.edps` is the
/// site's `edps` on every one of the 1,195, to the site's two decimals.
#[test]
fn dps_at_quality_the_q20_damage_is_within_one_of_every_captured_figure() {
    let captures: Value = serde_json::from_str(CAPTURES).unwrap();
    let bodies: Value = serde_json::from_str(BODIES).unwrap();
    assert_eq!(captures["_raw"], bodies["_raw"], "one generation");
    let mut s = store();
    let mut by_league: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    // the site's three figures and the attack speed, by the listing's id
    let mut site: BTreeMap<String, (f64, f64, f64)> = BTreeMap::new();
    for (name, search) in captures["searches"].as_object().unwrap() {
        // a search that fetched nothing has no bodies
        let sent = bodies["searches"][name]
            .as_array()
            .cloned()
            .unwrap_or_default();
        for (i, listing) in search["fetched"].as_array().unwrap().iter().enumerate() {
            let extended = &listing["extended"];
            let Some(pdps) = extended["pdps"].as_f64() else {
                continue;
            };
            let id = format!("{name}-{i}");
            site.insert(
                id.clone(),
                (
                    pdps,
                    extended["dps"].as_f64().unwrap(),
                    extended["edps"].as_f64().unwrap(),
                ),
            );
            let mut body = sent[i].clone();
            body["id"] = json!(id);
            body["x"] = json!(0);
            body["y"] = json!(0);
            let league = body["league"].as_str().unwrap().to_string();
            by_league.entry(league).or_default().push(body);
        }
    }
    assert_eq!(site.len(), 1195);
    for (league, items) in by_league {
        list_tabs(&mut s, "pc", &league, json!([tab("t", "Fetched")]), 10);
        fetch_tab(&mut s, "pc", &league, "t", "Fetched", items, 20);
    }
    let corpus = load(&s, Some("pc"));
    let (pdps, dps, edps, aps) = (
        values(&corpus, "pseudo.pdps_q20"),
        values(&corpus, "pseudo.dps_q20"),
        values(&corpus, "pseudo.edps"),
        values(&corpus, "attacks_per_second"),
    );
    // how far the build's mean is from the site's, in halves of a point:
    // pdps's, and dps's apart, since the site works each from its rolls
    let mut off: BTreeMap<i64, usize> = BTreeMap::new();
    let mut dps_off: BTreeMap<i64, usize> = BTreeMap::new();
    let mut faults: Vec<String> = Vec::new();
    let mut weapons = 0;
    for (id, (site_pdps, site_dps, site_edps)) in &site {
        // the site prints two decimals, rounded in floating point
        let built_edps = edps[id]["value"].as_f64();
        if built_edps.is_some_and(|e| (e - site_edps).abs() > 0.0051)
            || (built_edps.is_none() && *site_edps != 0.0)
        {
            faults.push(format!(
                "{id} edps: the site {site_edps}, the build {built_edps:?}"
            ));
        }
        let (Some(p), Some(speed)) = (pdps[id]["value"].as_f64(), aps[id]["value"].as_f64()) else {
            // no physical damage or no attack speed: the site's is nothing
            if *site_pdps != 0.0 {
                faults.push(format!("{id} pdps: the site {site_pdps}, the build none"));
            }
            continue;
        };
        weapons += 1;
        let halves = ((p - site_pdps) / speed * 2.0).round() as i64;
        *off.entry(halves).or_default() += 1;
        let d = dps[id]["value"].as_f64().unwrap();
        *dps_off
            .entry(((d - site_dps) / speed * 2.0).round() as i64)
            .or_default() += 1;
    }
    assert!(faults.is_empty(), "{}", faults.join("\n"));
    assert_eq!(weapons, 1095);
    let spread = |off: &BTreeMap<i64, usize>| -> [usize; 4] {
        let within = |h: std::ops::RangeInclusive<i64>| -> usize {
            off.iter()
                .filter(|(k, _)| h.contains(k))
                .map(|(_, n)| n)
                .sum()
        };
        [
            within(0..=0),
            within(-1..=1) - within(0..=0),
            within(-2..=2) - within(-1..=1),
            weapons - within(-2..=2),
        ]
    };
    assert_eq!(spread(&off), [902, 181, 12, 0], "pdps {off:?}");
    assert_eq!(spread(&dps_off), [902, 181, 12, 0], "dps {dps_off:?}");
}

/// The enchant `Quality does not increase Physical Damage` (the outside
/// review of 7e): under it the display is the damage at 20 % whatever the
/// quality — every captured weapon carrying it is at 20 % or more, so this
/// asks below — and the quality and the enchant are read before either
/// decides (rule 8): an unread array of lines leaves the value open below
/// 20 % alone, where the enchant could change it.
#[test]
fn dps_at_quality_the_enchant_keeps_the_display_below_twenty_percent() {
    let weapon = |id: &str, quality: &str, more: Value| {
        let mut body = json!({ "properties": [
            { "name": "Quality", "values": [[quality, 1]], "displayMode": 0 },
            { "name": "Physical Damage", "values": [["18-73", 1]], "displayMode": 0 },
            { "name": "Attacks per Second", "values": [["2.00", 1]], "displayMode": 0 }
        ] });
        for (k, v) in more.as_object().unwrap() {
            body[k] = v.clone();
        }
        item(id, "", "Ezomyte Dagger", "Rare", body)
    };
    let enchant = json!({ "enchantMods": ["Quality does not increase Physical Damage"] });
    let unread = json!({ "enchantMods": "unreadable" });
    let mut s = store();
    list_tabs(&mut s, "pc", "Standard", json!([tab("t", "Tab")]), 10);
    fetch_tab(
        &mut s,
        "pc",
        "Standard",
        "t",
        "Tab",
        vec![
            weapon("held", "+10%", enchant.clone()),
            weapon("bare", "+10%", json!({})),
            weapon("open", "+10%", unread.clone()),
            weapon("full", "+20%", unread),
        ],
        20,
    );
    let corpus = load(&s, Some("pc"));
    let (q20, shown) = (
        values(&corpus, "pseudo.pdps_q20"),
        values(&corpus, "pseudo.pdps"),
    );
    // worked by hand: the display's mean 45.5 × 2 = 91; at 20 % from 10 %
    // the ends 18 × 120 ÷ 110 = 19.6… → 20 and 73 × 120 ÷ 110 = 79.6… → 80,
    // their mean 50 × 2 = 100
    assert_eq!(shown["held"]["value"], 91);
    assert_eq!(q20["held"]["value"], 91);
    assert_eq!(q20["bare"]["value"], 100);
    assert_eq!(q20["full"]["value"], 91);
    // the one left open is undecided, never lacked
    let request: Request = serde_json::from_value(
        json!({ "query": { "text": "undecided(pseudo.pdps_q20)" }, "view": { "rows": { "limit": 10 } } }),
    )
    .unwrap();
    let open: Vec<String> = as_json(&answer(&corpus, &request).unwrap())["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["id"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(open, ["open"]);
}
