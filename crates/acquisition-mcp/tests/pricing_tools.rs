//! Pricing over MCP (the release brief, step 1, at `d0ab421e`): `price_status`,
//! `price_list`, `price_show`, `shop_render`, `reference_currency`,
//! `price_set` and `price_clear` are adapters of `acquisition-plan`'s
//! pricing verbs, as `acq price`, `acq shop render` and `acq reference
//! currency` are (C46), so what a tool returns is the document the CLI's
//! `--json` prints for the same request — a refusal included, in a result
//! marked `isError` (C11, C53). The one rule the surfaces differ on is
//! the MCP's: a write over a row of the target's own names the revision
//! it replaces (C52's create-only form), where the CLI's default replaces
//! whatever is stored. Facts and intent are seeded through the store
//! crate; no daemon runs, and none appears.

use crate::harness;

use std::path::{Path, PathBuf};

use acquisition_plan::price::Buyout;
use acquisition_plan::{PolicyWrite, put_sync_policy};
use acquisition_store::{Annotations, Endpoint, Index, Provenance, Store, account_path};
use harness::{Mcp, acq, assert_spoken_as, no_daemon_appeared, scratch};
use serde_json::{Value, json};

const USER: &str = "Alice#1234";
const UUID: &str = "u-price";

/// The world `price_json.rs` seeds for the CLI, under the store root the
/// server and the `acq` beside it read: a folder holding a public priced
/// tab with two items (one noted, one with a row), a map tab with a
/// substash holding one item, a character wearing one; a row on the
/// folder and one naming nothing here; and a sync policy, so the render
/// reports its coverage.
fn seed(base: &Path) {
    let mock = base.join("store").join("mock");
    std::fs::create_dir_all(&mock).unwrap();
    let at = acquisition_store::now();
    let mut index = Index::load(&mock).unwrap();
    index.record_login(USER, UUID, false, at, &[]).unwrap();
    let mut store = Store::open(&account_path(&mock, USER)).unwrap();
    let record = |store: &mut Store, ep: Endpoint, params: Value, body: Value| {
        store.record(&ep, &params, 200, &body, at).unwrap();
    };
    record(
        &mut store,
        Endpoint::Profile,
        json!({}),
        json!({ "uuid": UUID, "name": USER }),
    );
    record(
        &mut store,
        Endpoint::Stashes {
            realm: "pc".into(),
            league: "Standard".into(),
        },
        json!({ "league": "Standard" }),
        json!({ "stashes": [
            { "id": "f1", "name": "Sale", "type": "Folder", "index": 0,
              "children": [ { "id": "c1", "name": "~price 3 chaos", "type": "PremiumStash", "index": 1, "metadata": { "public": true } } ] },
            { "id": "m1", "name": "Maps", "type": "MapStash", "index": 2 } ] }),
    );
    let item = |id: &str, note: Option<&str>| {
        let mut v = json!({ "id": id, "name": "", "typeLine": "Chaos Orb", "baseType": "Chaos Orb", "x": 0, "y": 0, "stackSize": 5 });
        if let Some(n) = note {
            v["note"] = json!(n);
        }
        v
    };
    let stash = |id: &str, sub: Option<&str>| Endpoint::Stash {
        realm: "pc".into(),
        league: "Standard".into(),
        id: id.into(),
        sub: sub.map(str::to_string),
    };
    record(
        &mut store,
        stash("c1", None),
        json!({}),
        json!({ "stash": { "id": "c1", "name": "~price 3 chaos", "type": "PremiumStash",
                            "items": [ item("i-noted", Some("~price 5 chaos")), item("i-plain", None) ] } }),
    );
    record(
        &mut store,
        stash("m1", None),
        json!({}),
        json!({ "stash": { "id": "m1", "name": "Maps", "type": "MapStash", "items": [],
                            "children": [ { "id": "s1", "name": "1", "type": "MapStash", "metadata": { "items": 1 } } ] } }),
    );
    record(
        &mut store,
        stash("m1", Some("s1")),
        json!({}),
        json!({ "stash": { "id": "s1", "name": "1", "type": "MapStash", "items": [ item("i-map", None) ] } }),
    );
    record(
        &mut store,
        Endpoint::Characters { realm: "pc".into() },
        json!({ "realm": "pc" }),
        json!({ "characters": [ { "id": "ch1", "name": "Exile", "league": "Standard" } ] }),
    );
    record(
        &mut store,
        Endpoint::Character {
            realm: "pc".into(),
            name: "Exile".into(),
        },
        json!({ "realm": "pc", "name": "Exile" }),
        json!({ "character": { "id": "ch1", "name": "Exile", "league": "Standard",
                                "equipment": [ item("i-worn", Some("~b/o 2 divine")) ], "inventory": [] } }),
    );
    let mut a = Annotations::open_for(&mock, UUID).unwrap();
    let via = Provenance::via("test");
    for (scope, key, value) in [
        (
            "item",
            "i-plain",
            json!({ "version": 1, "type": "exact", "amount": "3", "currency": "chaos" }),
        ),
        ("tab", "pc/f1", json!({ "version": 1, "type": "skip" })),
        (
            "item",
            "i-elsewhere",
            json!({ "version": 1, "type": "skip" }),
        ),
    ] {
        a.put::<Buyout>(scope, key, &value, None, &via).unwrap();
    }
    put_sync_policy(
        &mut a,
        &json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard":
            { "tabs": "all", "characters": "all", "max_age_seconds": 86400 } } } } }),
        None,
        PolicyWrite::Replace,
        &via,
    )
    .unwrap();
}

/// A document with the clock's own readings taken out — when the
/// snapshot was taken and the render made, a second apart at most
/// between the two processes, and every age said in words beside an
/// epoch (an `_age` key, C53: made at each answer's own time, so the two
/// processes' can differ by a second) — and, for a write's receipt, when
/// the row was written and through which surface (`mcp` or `cli`, by
/// design).
fn steady(mut doc: Value, keys: &[&str]) -> Value {
    fn strip(v: &mut Value, keys: &[&str]) {
        match v {
            Value::Object(map) => {
                for key in keys {
                    map.remove(*key);
                }
                map.retain(|key, _| !key.ends_with("_age"));
                for child in map.values_mut() {
                    strip(child, keys);
                }
            }
            Value::Array(items) => items.iter_mut().for_each(|c| strip(c, keys)),
            _ => {}
        }
    }
    strip(&mut doc, keys);
    doc
}

const CLOCK: &[&str] = &["taken_at", "rendered_at"];
/// A write's clock and channel, and `undo`, compared
/// apart: one offer each surface spells (round 5 of the 2g build).
const RECEIPT: &[&str] = &[
    "taken_at",
    "created_at",
    "updated_at",
    "written_via",
    "undo",
];

/// Round 5 of the 2g build: two receipts for one write offer one undo,
/// each spelled by its surface — the tool's a request naming the tool it
/// goes to where another's, the CLI's a command line — saying the same,
/// or both none, `no_undo` then compared with the rest.
fn undo_alike(by_tool: &Value, by_cli: &Value) {
    match (by_tool.get("undo"), by_cli.get("undo")) {
        (None, None) => {}
        (Some(tool), Some(cli)) => {
            assert_eq!(tool["says"], cli["says"], "{by_tool}\n{by_cli}");
            assert!(tool["request"].is_object(), "{by_tool}");
            assert!(
                cli["request"]
                    .as_str()
                    .is_some_and(|l| l.starts_with("acq price ")),
                "{by_cli}"
            );
        }
        _ => panic!("one surface offers an undo, the other none:\n{by_tool}\n{by_cli}"),
    }
}

/// What the CLI prints under `--json`, with whether it succeeded.
fn cli(base: &Path, args: &[&str]) -> (bool, Value) {
    let mut args = args.to_vec();
    args.push("--json");
    let out = acq(base, &args);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let doc = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!(
            "acq {args:?} printed no one JSON document ({e}):\n{stdout}\n{}",
            String::from_utf8_lossy(&out.stderr)
        )
    });
    (out.status.success(), doc)
}

/// What the tool returns: its document, with whether it succeeded (a
/// result not marked `isError`).
fn tool(mcp: &mut Mcp, name: &str, args: Value) -> (bool, Value) {
    let result = mcp.result(name, args);
    let ok = result["isError"] != json!(true);
    (ok, harness::document(&result))
}

/// C46, C11 — for every read the seven tools serve, the tool's document
/// is the CLI's `--json` for the same request: the answer whole, and a
/// refusal's `{"error": …}` word for word.
#[test]
fn each_pricing_read_is_the_document_the_cli_prints() {
    let dir = scratch("mcp-price-read");
    let base = &dir.0;
    seed(base);
    let mut mcp = Mcp::start(base, &[]);
    let cases: &[(&str, Value, &[&str], bool)] = &[
        ("price_status", json!({}), &["price", "status"], true),
        (
            "price_status",
            json!({ "expand": true }),
            &["price", "status", "--expand"],
            true,
        ),
        (
            "price_status",
            json!({ "league": "Hardcore" }),
            &["price", "status", "--league", "Hardcore"],
            true,
        ),
        ("price_list", json!({}), &["price", "list"], true),
        (
            "price_list",
            json!({ "effective": "none" }),
            &["price", "list", "--effective", "none"],
            true,
        ),
        (
            "price_list",
            json!({ "effective": "none", "limit": 1 }),
            &["price", "list", "--effective", "none", "--limit", "1"],
            true,
        ),
        (
            "price_list",
            json!({ "covered_by": "tab/pc/f1" }),
            &["price", "list", "--covered-by", "tab/pc/f1"],
            true,
        ),
        (
            "price_list",
            json!({ "in": "tab/pc/m1" }),
            &["price", "list", "--in", "tab/pc/m1"],
            true,
        ),
        (
            "price_list",
            json!({ "in": "tab/pc/gone" }),
            &["price", "list", "--in", "tab/pc/gone"],
            false,
        ),
        (
            "price_list",
            json!({ "relation": "maybe" }),
            &["price", "list", "--relation", "maybe"],
            false,
        ),
        (
            "price_show",
            json!({ "target": "item/i-plain" }),
            &["price", "show", "item/i-plain"],
            true,
        ),
        (
            "price_show",
            json!({ "target": "tab/pc/m1" }),
            &["price", "show", "tab/pc/m1"],
            true,
        ),
        (
            "price_show",
            json!({ "target": "tab/pc/m1", "limit": 0 }),
            &["price", "show", "tab/pc/m1", "--limit", "0"],
            true,
        ),
        (
            "price_show",
            json!({ "target": "tab/pc/m1", "limit": 0, "expand": true }),
            &["price", "show", "tab/pc/m1", "--limit", "0", "--expand"],
            true,
        ),
        (
            "price_show",
            json!({ "target": "i-plain" }),
            &["price", "show", "i-plain"],
            false,
        ),
        (
            "price_show",
            json!({ "target": "item/i-nowhere" }),
            &["price", "show", "item/i-nowhere"],
            false,
        ),
        ("shop_render", json!({}), &["shop", "render"], true),
        (
            "shop_render",
            json!({ "expand": true }),
            &["shop", "render", "--expand"],
            true,
        ),
        (
            "shop_render",
            json!({ "size": 2000, "page": 1 }),
            &["shop", "render", "--size", "2000", "--page", "1"],
            true,
        ),
        (
            "shop_render",
            json!({ "page": 9 }),
            &["shop", "render", "--page", "9"],
            false,
        ),
        (
            "reference_currency",
            json!({}),
            &["reference", "currency"],
            true,
        ),
        (
            "reference_currency",
            json!({ "word": "chaos" }),
            &["reference", "currency", "chaos"],
            true,
        ),
        (
            "reference_currency",
            json!({ "word": "Chaos" }),
            &["reference", "currency", "Chaos"],
            false,
        ),
    ];
    for (name, args, verb, succeeds) in cases {
        let (tool_ok, by_tool) = tool(&mut mcp, name, args.clone());
        let (cli_ok, by_cli) = cli(base, verb);
        assert_eq!(
            (tool_ok, cli_ok),
            (*succeeds, *succeeds),
            "{name} {args} / acq {verb:?}: {by_tool}\n{by_cli}"
        );
        assert_spoken_as(
            &steady(by_tool, CLOCK),
            &steady(by_cli, CLOCK),
            &format!("{name} {args} is not what acq {verb:?} --json prints"),
        );
    }

    // The template is text to the tool and a file to the CLI: one page.
    let template = "[spoiler=\"shop\"]\n[items]\n[/spoiler]";
    std::fs::write(base.join("template.txt"), template).unwrap();
    let path = base.join("template.txt");
    let (ok, by_tool) = tool(&mut mcp, "shop_render", json!({ "template": template }));
    assert!(ok, "{by_tool}");
    let (_, by_cli) = cli(
        base,
        &["shop", "render", "--template", path.to_str().unwrap()],
    );
    assert_spoken_as(
        &steady(by_tool, CLOCK),
        &steady(by_cli, CLOCK),
        "shop_render with a template",
    );
    assert!(no_daemon_appeared(base), "a pricing read reached a daemon");
}

/// C46, C53 — the release seat's rerun (mcp-opus-2, calls 37, 38): the
/// shop document listed every item left off for a reason, 160 KB on the
/// owner's league, with no bound short of `page`, which drops the
/// counts. With 60 items the game lists added to the seeded world, the
/// default document, the tool's and the CLI's the same, bounds the list
/// and counts them all in `left_out_total`; `expand` is `--expand`, and
/// both then list every one. The bound is the shop's (`acquisition-plan`).
#[test]
fn the_shop_document_bounds_what_it_leaves_out_alike_on_both_surfaces() {
    let dir = scratch("mcp-price-left-out");
    let base = &dir.0;
    seed(base);
    let mock = base.join("store").join("mock");
    let mut store = Store::open(&account_path(&mock, USER)).unwrap();
    let at = acquisition_store::now();
    store
        .record(
            &Endpoint::Stashes {
                realm: "pc".into(),
                league: "Standard".into(),
            },
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": [
                { "id": "f1", "name": "Sale", "type": "Folder", "index": 0,
                  "children": [ { "id": "c1", "name": "~price 3 chaos", "type": "PremiumStash", "index": 1, "metadata": { "public": true } } ] },
                { "id": "m1", "name": "Maps", "type": "MapStash", "index": 2 },
                { "id": "b1", "name": "Bulk", "type": "PremiumStash", "index": 3, "metadata": { "public": true } } ] }),
            at,
        )
        .unwrap();
    let bulk: Vec<Value> = (0..60)
        .map(|n| {
            json!({ "id": format!("i-bulk-{n}"), "name": "", "typeLine": "Chaos Orb",
                    "baseType": "Chaos Orb", "x": n % 12, "y": n / 12, "stackSize": 1,
                    "note": "~price 1 chaos" })
        })
        .collect();
    store
        .record(
            &Endpoint::Stash {
                realm: "pc".into(),
                league: "Standard".into(),
                id: "b1".into(),
                sub: None,
            },
            &json!({}),
            200,
            &json!({ "stash": { "id": "b1", "name": "Bulk", "type": "PremiumStash", "items": bulk } }),
            at,
        )
        .unwrap();
    drop(store);

    let mut mcp = Mcp::start(base, &[]);
    let mut totals = Vec::new();
    for (args, verb) in [
        (json!({}), &["shop", "render"][..]),
        (
            json!({ "expand": true }),
            &["shop", "render", "--expand"][..],
        ),
    ] {
        let (tool_ok, by_tool) = tool(&mut mcp, "shop_render", args.clone());
        let (cli_ok, by_cli) = cli(base, verb);
        assert!(tool_ok && cli_ok, "{by_tool}\n{by_cli}");
        assert_spoken_as(
            &steady(by_tool.clone(), CLOCK),
            &steady(by_cli, CLOCK),
            &format!("shop_render {args} is not what acq {verb:?} --json prints"),
        );
        let listed = by_tool["left_out"].as_array().map_or(0, Vec::len);
        let total = by_tool["left_out_total"].as_u64().unwrap_or_default() as usize;
        totals.push((listed, total));
    }
    // `expand` is the CLI's `--expand`: the default bounds the list (the
    // bound itself is the shop's, `acquisition-plan`), expanded it is whole
    let [(listed, total), (all_listed, all_total)] = totals[..] else {
        unreachable!()
    };
    assert!(listed < total, "the default listed all {total}");
    assert_eq!(all_listed, all_total, "expanded, every one is listed");
    assert!(no_daemon_appeared(base));
}

/// C46, C52 — a write's receipt is the CLI's for the same write, the
/// revision named: two worlds seeded alike, one written through the
/// tools and one through `acq`, step for step — a create, a replace, a
/// clear, and a stale revision refused with the CLI's words.
#[test]
fn each_pricing_write_is_the_receipt_the_cli_prints() {
    let (by_mcp, by_acq) = (
        scratch("mcp-price-write-mcp"),
        scratch("mcp-price-write-cli"),
    );
    seed(&by_mcp.0);
    seed(&by_acq.0);
    let mut mcp = Mcp::start(&by_mcp.0, &[]);
    let steps: &[(&str, Value, &[&str], bool)] = &[
        (
            "price_set",
            json!({ "target": "item/i-map", "type": "exact", "amount": "12.50", "currency": "chaos" }),
            &["price", "set", "item/i-map", "exact", "12.50", "chaos"],
            true,
        ),
        (
            "price_set",
            json!({ "target": "item/i-map", "type": "b/o", "amount": "1/5", "currency": "divine", "if_revision": 1 }),
            &[
                "price",
                "set",
                "item/i-map",
                "b/o",
                "1/5",
                "divine",
                "--if-revision",
                "1",
            ],
            true,
        ),
        (
            "price_set",
            json!({ "target": "item/i-map", "type": "skip", "if_revision": 1 }),
            &["price", "set", "item/i-map", "skip", "--if-revision", "1"],
            false,
        ),
        (
            "price_set",
            json!({ "target": "tab/pc/nope", "type": "skip" }),
            &["price", "set", "tab/pc/nope", "skip"],
            true,
        ),
        (
            "price_set",
            json!({ "target": "item/i-noted", "type": "exact", "amount": "1", "currency": "exa" }),
            &["price", "set", "item/i-noted", "exact", "1", "exa"],
            false,
        ),
        (
            "price_clear",
            json!({ "target": "item/i-map", "if_revision": 2 }),
            &["price", "clear", "item/i-map", "--if-revision", "2"],
            true,
        ),
        (
            "price_set",
            json!({ "target": "item/i-map", "type": "skip", "if_revision": 2 }),
            &["price", "set", "item/i-map", "skip", "--if-revision", "2"],
            false,
        ),
        (
            "price_clear",
            json!({ "target": "item/i-plain", "if_revision": 1 }),
            &["price", "clear", "item/i-plain", "--if-revision", "1"],
            true,
        ),
    ];
    for (name, args, verb, succeeds) in steps {
        let (tool_ok, by_tool) = tool(&mut mcp, name, args.clone());
        let (cli_ok, by_cli) = cli(&by_acq.0, verb);
        assert_eq!(
            (tool_ok, cli_ok),
            (*succeeds, *succeeds),
            "{name} {args} / acq {verb:?}: {by_tool}\n{by_cli}"
        );
        undo_alike(&by_tool, &by_cli);
        assert_eq!(
            steady(by_tool, RECEIPT),
            steady(by_cli, RECEIPT),
            "{name} {args} is not what acq {verb:?} --json prints"
        );
    }
    // The tool's rows are stamped as the server's, and a row on a target
    // the facts do not hold is read through the surface: `price_status`
    // lists it whole, and `price_show` refuses it saying so (C35).
    let (_, shown) = tool(&mut mcp, "price_show", json!({ "target": "tab/pc/nope" }));
    assert!(
        shown["error"]
            .as_str()
            .is_some_and(|e| e.contains("it has a price row of its own at revision 1")),
        "{shown}"
    );
    let (_, status) = tool(&mut mcp, "price_status", json!({}));
    let unmatched = status["rows"]["unmatched"].as_array().unwrap();
    let row = unmatched
        .iter()
        .find(|m| m["from"] == json!({ "scope": "tab", "realm": "pc", "id": "nope" }))
        .unwrap_or_else(|| panic!("the row is not listed whole: {status}"));
    assert_eq!(
        (&row["value"], &row["revision"], &row["written_via"]),
        (
            &json!({ "version": 1, "type": "skip" }),
            &json!(1),
            &json!("mcp")
        ),
        "{status}"
    );
    assert!(
        no_daemon_appeared(&by_mcp.0),
        "a pricing write reached a daemon"
    );
}

/// C52 — the MCP never replaces or clears a row it has not read: a set
/// or a clear naming no revision, over a row of the target's own, is
/// refused naming the current revision, and the row stands; with no row
/// of its own a clear says so, and a set creates. The same requests
/// through the CLI's default land — the MCP is the stricter.
#[test]
fn a_blind_replace_and_a_blind_clear_are_refused_naming_the_current_revision() {
    let dir = scratch("mcp-price-blind");
    let base = &dir.0;
    seed(base);
    let mut mcp = Mcp::start(base, &[]);
    let shown = |mcp: &mut Mcp| {
        let (ok, view) = tool(mcp, "price_show", json!({ "target": "item/i-plain" }));
        assert!(ok, "{view}");
        view["listing"]["manual"].clone()
    };
    let before = shown(&mut mcp);
    assert_eq!(before["revision"], json!(1), "{before}");
    assert_eq!(
        before["from"],
        json!({ "scope": "item", "id": "i-plain" }),
        "{before}"
    );

    let (ok, refusal) = tool(
        &mut mcp,
        "price_set",
        json!({ "target": "item/i-plain", "type": "skip" }),
    );
    assert!(!ok, "a blind replace landed: {refusal}");
    let said = refusal["error"].as_str().unwrap_or_default();
    assert!(
        said.contains("item/i-plain has a price row of its own at revision 1")
            && said.contains("names the revision it replaces (1)"),
        "{refusal}"
    );
    assert_eq!(
        shown(&mut mcp),
        before,
        "the refused replace changed the row"
    );

    let (ok, refusal) = tool(&mut mcp, "price_clear", json!({ "target": "item/i-plain" }));
    assert!(!ok, "a blind clear landed: {refusal}");
    let said = refusal["error"].as_str().unwrap_or_default();
    assert!(
        said.contains("item/i-plain has a price row of its own at revision 1")
            && said.contains("names the revision it replaces (1)"),
        "{refusal}"
    );
    assert_eq!(shown(&mut mcp), before, "the refused clear changed the row");

    // A target with no row of its own: a clear says which nothing, as the
    // CLI's does; a set creates with no revision named.
    let (ok, refusal) = tool(&mut mcp, "price_clear", json!({ "target": "item/i-map" }));
    assert!(!ok);
    let (_, by_cli) = cli(base, &["price", "clear", "item/i-map"]);
    assert_spoken_as(&refusal, &by_cli, "price_clear item/i-map");
    let (ok, receipt) = tool(
        &mut mcp,
        "price_set",
        json!({ "target": "item/i-map", "type": "no_price" }),
    );
    assert!(ok, "{receipt}");
    assert_eq!(receipt["prior"], Value::Null, "{receipt}");

    // The CLI's default is the blind form, over the same row.
    let (ok, receipt) = cli(base, &["price", "set", "item/i-plain", "skip"]);
    assert!(ok, "{receipt}");
    assert_eq!(receipt["prior"]["revision"], json!(1), "{receipt}");
    assert!(no_daemon_appeared(base));
}

/// C46 — the MCP opens for a verb what the CLI opens for it: a clear
/// reads and writes intent alone, so a facts database the CLI's clear
/// never opens cannot refuse the tool's. With the facts removed after
/// seeding, a revisioned clear lands on both surfaces alike and leaves no
/// facts file behind; with a file in their place that is no database, it
/// lands alike again and the file is left as it was.
#[test]
fn a_clear_lands_alike_on_both_surfaces_without_the_facts() {
    let (by_mcp, by_acq) = (
        scratch("mcp-price-clear-mcp"),
        scratch("mcp-price-clear-cli"),
    );
    seed(&by_mcp.0);
    seed(&by_acq.0);
    let facts = |base: &Path| account_path(&base.join("store").join("mock"), USER);
    let remove = |base: &Path| {
        let path = facts(base);
        for suffix in ["", "-wal", "-shm"] {
            let mut p = path.clone().into_os_string();
            p.push(suffix);
            let _ = std::fs::remove_file(PathBuf::from(p));
        }
        path
    };
    let mut mcp = Mcp::start(&by_mcp.0, &[]);
    let clear = |mcp: &mut Mcp, target: &str| {
        let (tool_ok, by_tool) = tool(
            mcp,
            "price_clear",
            json!({ "target": target, "if_revision": 1 }),
        );
        let (cli_ok, by_cli) = cli(&by_acq.0, &["price", "clear", target, "--if-revision", "1"]);
        assert!(tool_ok && cli_ok, "{by_tool}\n{by_cli}");
        undo_alike(&by_tool, &by_cli);
        assert_eq!(steady(by_tool, RECEIPT), steady(by_cli, RECEIPT));
    };

    let (gone_mcp, gone_acq) = (remove(&by_mcp.0), remove(&by_acq.0));
    clear(&mut mcp, "item/i-plain");
    assert!(
        !gone_mcp.exists() && !gone_acq.exists(),
        "a clear made a facts file"
    );

    for base in [&by_mcp.0, &by_acq.0] {
        std::fs::write(facts(base), "not a database").unwrap();
    }
    clear(&mut mcp, "tab/pc/f1");
    assert_eq!(
        std::fs::read_to_string(facts(&by_mcp.0)).unwrap(),
        "not a database"
    );
    assert!(no_daemon_appeared(&by_mcp.0));
}

/// The release seat (mcp-opus, mcp-sonnet): a cleared row took a
/// revision, and the next write's continued from it, said nowhere. The
/// pricing model says it, read once by name (`describe` with names
/// [`pricing`], #83), where the two write tools' texts said it before.
#[test]
fn the_price_write_tools_say_a_clear_takes_a_revision() {
    let dir = scratch("mcp-price-clear-revision");
    let mut mcp = Mcp::start(&dir.0, &[]);
    let result = mcp.result("describe", json!({ "names": ["pricing"] }));
    let said = result["content"][0]["text"].as_str().unwrap_or_default();
    assert!(
        said.contains(
            "A clear takes a revision too, so the next write's revision continues from it"
        ),
        "{said}"
    );
}

/// The readiness seat (`audits/readiness-2026-10/seat/`, calls 19 and
/// 33): a game note on an item beat the hand row on its tab, which the
/// item's `why` said and `describe`'s pricing model did not. The
/// `effective` entry says who decides: the most specific statement, the
/// game's on a tie (C69, C81).
#[test]
fn the_pricing_model_says_the_most_specific_statement_decides() {
    let dir = scratch("mcp-price-decides");
    let mut mcp = Mcp::start(&dir.0, &[]);
    let (ok, doc) = tool(&mut mcp, "describe", json!({ "names": ["effective"] }));
    assert!(ok, "{doc}");
    let said = doc["pricing"][0]["what"].as_str().unwrap_or_default();
    assert!(
        said.contains("The most specific statement decides, the game's on a tie")
            && said.contains("an item's note before a hand row on its tab"),
        "{doc}"
    );
}

/// The release seat's finding 9: a pricing refusal spoke the CLI's flags,
/// and the tool texts carried the translation ("--league is `league`, --in
/// is `in`"). The refusal says it: the shared words whole, then this
/// server's argument for each flag they speak (C143) — the conflict over
/// a row the target already has names `if_revision` after the flag, and
/// a container argument given an item names `in`.
#[test]
fn a_pricing_refusal_names_the_argument_after_each_flag_it_speaks() {
    let dir = scratch("mcp-price-spelled");
    let base = &dir.0;
    seed(base);
    let mut mcp = Mcp::start(base, &[]);
    let (ok, refusal) = tool(
        &mut mcp,
        "price_set",
        json!({ "target": "item/i-plain", "type": "skip" }),
    );
    assert!(!ok, "{refusal}");
    assert_eq!(refusal["kind"], "conflict", "{refusal}");
    let said = refusal["error"].as_str().unwrap_or_default();
    assert!(
        said.contains("names the revision it replaces (1) with `--if-revision 1`")
            && said.ends_with(
                "— here, `acq price show` is `price_show`, `--if-revision` is `if_revision`, \
                 `--account` is `account` (`scope.account` in a search request)"
            ),
        "{said}"
    );
    let (ok, refusal) = tool(&mut mcp, "price_list", json!({ "in": "item/i-plain" }));
    assert!(!ok, "{refusal}");
    let said = refusal["error"].as_str().unwrap_or_default();
    assert!(said.ends_with("— here, `--in` is `in`"), "{said}");
}

/// C53, the read economy (step 2g, round 2): past ten, `price_list` and
/// `price_show` list ten beside the whole counts and `expand` returns
/// every item, each the document `acq price … --json` prints for the same
/// request — over a league Big of one map tab with thirteen unpriced
/// items, which only `effective: "none"` selects. The listing rule is
/// the planner's (`listing.rs`); here, the adapter's mapping of `limit`
/// and `expand` past ten.
#[test]
fn price_list_and_price_show_list_ten_and_expand_returns_every_item_past_ten() {
    let dir = scratch("mcp-price-bound");
    let base = &dir.0;
    seed(base);
    let mock = base.join("store").join("mock");
    let mut store = Store::open(&account_path(&mock, USER)).unwrap();
    let at = acquisition_store::now();
    store
        .record(
            &Endpoint::Stashes {
                realm: "pc".into(),
                league: "Big".into(),
            },
            &json!({ "league": "Big" }),
            200,
            &json!({ "stashes": [ { "id": "b1", "name": "Many", "type": "MapStash", "index": 0 } ] }),
            at,
        )
        .unwrap();
    let items: Vec<Value> = (0..13)
        .map(|i| json!({ "id": format!("i-b{i:02}"), "name": "", "typeLine": "Chaos Orb", "baseType": "Chaos Orb", "x": i, "y": 0, "stackSize": 1 }))
        .collect();
    store
        .record(
            &Endpoint::Stash {
                realm: "pc".into(),
                league: "Big".into(),
                id: "b1".into(),
                sub: None,
            },
            &json!({ "league": "Big" }),
            200,
            &json!({ "stash": { "id": "b1", "name": "Many", "type": "MapStash", "items": items } }),
            at,
        )
        .unwrap();
    drop(store);
    let mut mcp = Mcp::start(base, &[]);
    let listed = |doc: &Value, key: &str| doc[key].as_array().unwrap().len();
    for (args, verb, key, n) in [
        (
            json!({ "league": "Big", "effective": "none" }),
            &["price", "list", "--league", "Big", "--effective", "none"][..],
            "items",
            10,
        ),
        (
            json!({ "league": "Big", "effective": "none", "expand": true }),
            &[
                "price",
                "list",
                "--league",
                "Big",
                "--effective",
                "none",
                "--expand",
            ][..],
            "items",
            13,
        ),
    ] {
        let (ok, by_tool) = tool(&mut mcp, "price_list", args.clone());
        assert!(ok, "{by_tool}");
        assert_eq!(listed(&by_tool, key), n, "{args}");
        assert_eq!(by_tool["total"], 13);
        assert_eq!(by_tool["counts"]["left_out"], 13 - n);
        let (_, by_cli) = cli(base, verb);
        assert_spoken_as(
            &steady(by_tool, CLOCK),
            &steady(by_cli, CLOCK),
            &format!("price_list {args} is not what acq {verb:?} --json prints"),
        );
    }
    for (args, verb, n) in [
        (
            json!({ "target": "tab/pc/b1" }),
            &["price", "show", "tab/pc/b1"][..],
            10,
        ),
        (
            json!({ "target": "tab/pc/b1", "expand": true }),
            &["price", "show", "tab/pc/b1", "--expand"][..],
            13,
        ),
    ] {
        let (ok, by_tool) = tool(&mut mcp, "price_show", args.clone());
        assert!(ok, "{by_tool}");
        assert_eq!(listed(&by_tool, "items_here"), n, "{args}");
        assert_eq!(by_tool["items_here_counts"]["total"], 13);
        let (_, by_cli) = cli(base, verb);
        assert_spoken_as(
            &steady(by_tool, CLOCK),
            &steady(by_cli, CLOCK),
            &format!("price_show {args} is not what acq {verb:?} --json prints"),
        );
    }
    assert!(no_daemon_appeared(base));
}

/// Round 4's review (P3): a conflict's `current` is the stored row,
/// data, never this server's prose — a row whose actor names a command
/// came back spelled. Over such a row a set at a stale revision is
/// `conflict`, its `current` exactly the stored row `price_show` prints
/// as `manual`, less the age words an answer adds (round 6's fix round).
#[test]
fn a_conflicts_current_is_the_row_price_show_prints_verbatim() {
    let dir = scratch("mcp-price-current");
    let base = &dir.0;
    seed(base);
    Annotations::open_for(&base.join("store").join("mock"), UUID)
        .unwrap()
        .put::<Buyout>(
            "item",
            "i-map",
            &json!({ "version": 1, "type": "skip" }),
            None,
            &Provenance::via("test").as_actor("acq search --count tab"),
        )
        .unwrap();
    let mut mcp = Mcp::start(base, &[]);
    let (ok, view) = tool(&mut mcp, "price_show", json!({ "target": "item/i-map" }));
    assert!(ok, "{view}");
    let mut manual = view["listing"]["manual"].clone();
    assert_eq!(manual["actor"], "acq search --count tab", "{view}");
    // less the words C53 says beside its `updated_at` on `show` (step 2g
    // round 6), which a refusal does not carry: round 6 touches no refusal
    assert!(
        manual
            .as_object_mut()
            .unwrap()
            .remove("updated_at_age")
            .is_some(),
        "{view}"
    );
    let (ok, refusal) = tool(
        &mut mcp,
        "price_set",
        json!({ "target": "item/i-map", "type": "no_price", "if_revision": 9 }),
    );
    assert!(!ok, "{refusal}");
    assert_eq!(refusal["kind"], "conflict", "{refusal}");
    assert_eq!(refusal["current"], manual, "{refusal}");
}

/// Round 5 of the 2g build (the receipt shape): a write's receipt on this
/// server carries `revision`, the one the next write names, and `undo`, a
/// request in the refusal's offer shape, naming the tool it goes to where
/// another's; sent back as printed it puts `prior` back — price_set's
/// create by price_clear naming the revision written, price_clear's by
/// price_set naming none (a create, landing past the tombstone), a
/// replace's by price_set naming the revision written.
#[test]
fn every_receipts_undo_sent_back_as_printed_puts_the_prior_back() {
    let dir = scratch("mcp-price-undo");
    seed(&dir.0);
    let mut mcp = Mcp::start(&dir.0, &[]);
    let sent = |mcp: &mut Mcp, called: &str, doc: &Value| -> (String, Value) {
        let undo = &doc["undo"];
        let to = undo["tool"].as_str().unwrap_or(called).to_string();
        assert_ne!(undo.get("tool"), Some(&json!(called)), "{doc}");
        let (ok, back) = tool(mcp, &to, undo["request"].clone());
        assert!(ok, "{to} {}: {back}", undo["request"]);
        (to, back)
    };
    let manual = |mcp: &mut Mcp| {
        tool(mcp, "price_show", json!({ "target": "item/i-map" })).1["listing"]["manual"].clone()
    };

    let (ok, created) = tool(
        &mut mcp,
        "price_set",
        json!({ "target": "item/i-map", "type": "exact", "amount": "12.5", "currency": "chaos" }),
    );
    assert!(ok, "{created}");
    assert_eq!(created["revision"], json!(1));
    assert_eq!(
        created["undo"],
        json!({
            "request": { "target": "item/i-map", "if_revision": 1, "account": USER },
            "says": "undoes it",
            "tool": "price_clear",
        })
    );
    let (to, cleared) = sent(&mut mcp, "price_set", &created);
    assert_eq!(to, "price_clear");
    assert_eq!(manual(&mut mcp), Value::Null);
    assert_eq!(
        (&cleared["written"], &cleared["revision"]),
        (&Value::Null, &json!(2))
    );
    assert_eq!(cleared["undo"]["tool"], json!("price_set"));
    assert_eq!(
        cleared["undo"]["request"],
        json!({ "target": "item/i-map", "type": "exact", "amount": "12.5",
                "currency": "chaos", "account": USER }),
        "a create names no revision"
    );
    let (_, recreated) = sent(&mut mcp, "price_clear", &cleared);
    assert_eq!(
        (&recreated["prior"], &recreated["revision"]),
        (&Value::Null, &json!(3))
    );
    assert_eq!(manual(&mut mcp)["value"], created["written"]["value"]);

    let (ok, replaced) = tool(
        &mut mcp,
        "price_set",
        json!({ "target": "item/i-map", "type": "skip", "if_revision": 3 }),
    );
    assert!(ok, "{replaced}");
    assert_eq!(
        replaced["undo"].get("tool"),
        None,
        "the same tool: {replaced}"
    );
    let (_, restored) = sent(&mut mcp, "price_set", &replaced);
    assert_eq!(restored["prior"], replaced["written"]);
    assert_eq!(manual(&mut mcp)["value"], created["written"]["value"]);
    assert!(no_daemon_appeared(&dir.0));
}

/// Round 5 of the 2g build (the receipt shape): `set_sync_policy`
/// answers the shape every write answers, its `undo` a request to the
/// same tool — the prior written back whole (`replace`) over the revision
/// this write left — which, sent back as printed, lands; a first policy
/// has none, and `no_undo` says why.
#[test]
fn a_policy_receipts_undo_sent_back_as_printed_puts_the_prior_back() {
    let dir = scratch("mcp-policy-undo");
    let mock = dir.0.join("store").join("mock");
    std::fs::create_dir_all(&mock).unwrap();
    Index::load(&mock)
        .unwrap()
        .record_login(USER, UUID, false, acquisition_store::now(), &[])
        .unwrap();
    let mut mcp = Mcp::start(&dir.0, &[]);
    let pc = json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard":
        { "tabs": "all", "max_age_seconds": 3600 } } } } });
    let poe2 = json!({ "version": 3, "realms": { "poe2": { "leagues": { "Standard":
        { "characters": "all", "max_age_seconds": 600 } } } } });
    let (ok, first) = tool(&mut mcp, "set_sync_policy", json!({ "value": pc }));
    assert!(ok, "{first}");
    assert_eq!(
        (&first["prior"], &first["revision"]),
        (&Value::Null, &json!(1))
    );
    assert_eq!(
        first["target"],
        json!({ "account": USER, "realms": ["pc"] })
    );
    assert_eq!(first.get("undo"), None, "{first}");
    assert!(first["no_undo"].is_string(), "{first}");
    let (ok, merged) = tool(
        &mut mcp,
        "set_sync_policy",
        json!({ "value": poe2, "if_revision": 1 }),
    );
    assert!(ok, "{merged}");
    assert_eq!(merged["prior"]["value"], pc);
    assert_eq!(
        merged["undo"],
        json!({
            "request": { "value": pc, "if_revision": 2, "replace": true, "account": USER },
            "says": "puts the prior policy back, whole",
        }),
        "the same tool, so no `tool`"
    );
    let (ok, back) = tool(
        &mut mcp,
        "set_sync_policy",
        merged["undo"]["request"].clone(),
    );
    assert!(ok, "{back}");
    assert_eq!((&back["revision"], &back["value"]), (&json!(3), &pc));
    assert_eq!(back["prior"], merged["written"]);
}
