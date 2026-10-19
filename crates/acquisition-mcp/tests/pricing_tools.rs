//! Pricing over MCP (`RELEASE-PLAN.md`, step 1): `price_status`,
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

mod harness;

use std::path::{Path, PathBuf};

use acquisition_plan::price::Buyout;
use acquisition_plan::put_sync_policy;
use acquisition_store::{Annotations, Endpoint, Index, Provenance, Store, account_path};
use harness::{Mcp, acq, no_daemon_appeared};
use serde_json::{Value, json};

const USER: &str = "Alice#1234";
const UUID: &str = "u-price";

/// The scratch directory, removed on drop whichever way the test ends.
struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn scratch(tag: &str) -> Scratch {
    let base = std::env::temp_dir().join(format!(
        "acq-mcp-price-{tag}-{}-{}",
        std::process::id(),
        acquisition_store::now()
    ));
    // a process id comes round again: never open what an earlier run left
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    Scratch(base)
}

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
    index.record_login(USER, UUID, false, at).unwrap();
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
        &via,
    )
    .unwrap();
}

/// A document with the clock's own readings taken out — when the
/// snapshot was taken and the render made, a second apart at most
/// between the two processes — and, for a write's receipt, when the row
/// was written and through which surface (`mcp` or `cli`, by design).
fn steady(mut doc: Value, keys: &[&str]) -> Value {
    fn strip(v: &mut Value, keys: &[&str]) {
        match v {
            Value::Object(map) => {
                for key in keys {
                    map.remove(*key);
                }
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
const WRITE: &[&str] = &["taken_at", "created_at", "updated_at", "written_via"];

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

/// What the tool returns: its structured content, with whether it
/// succeeded (a result not marked `isError`).
fn tool(mcp: &mut Mcp, name: &str, args: Value) -> (bool, Value) {
    let result = mcp.result(name, args);
    let ok = result["isError"] != json!(true);
    let doc = result
        .get("structuredContent")
        .cloned()
        .unwrap_or_else(|| panic!("{name} returned no document: {result}"));
    (ok, doc)
}

/// C46, C11 — for every read the seven tools serve, the tool's document
/// is the CLI's `--json` for the same request: the answer whole, and a
/// refusal's `{"error": …}` word for word.
#[test]
fn each_pricing_read_is_the_document_the_cli_prints() {
    let dir = scratch("read");
    let base = &dir.0;
    seed(base);
    let mut mcp = Mcp::start(base, &[]);
    let cases: &[(&str, Value, &[&str], bool)] = &[
        ("price_status", json!({}), &["price", "status"], true),
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
        assert_eq!(
            steady(by_tool, CLOCK),
            steady(by_cli, CLOCK),
            "{name} {args} is not what acq {verb:?} --json prints"
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
    assert_eq!(steady(by_tool, CLOCK), steady(by_cli, CLOCK));
    assert!(no_daemon_appeared(base), "a pricing read reached a daemon");
}

/// C46, C52 — a write's receipt is the CLI's for the same write, the
/// revision named: two worlds seeded alike, one written through the
/// tools and one through `acq`, step for step — a create, a replace, a
/// clear, and a stale revision refused with the CLI's words.
#[test]
fn each_pricing_write_is_the_receipt_the_cli_prints() {
    let (by_mcp, by_acq) = (scratch("write-mcp"), scratch("write-cli"));
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
        assert_eq!(
            steady(by_tool, WRITE),
            steady(by_cli, WRITE),
            "{name} {args} is not what acq {verb:?} --json prints"
        );
    }
    // The tool's rows are stamped as the server's.
    let (_, shown) = tool(&mut mcp, "price_show", json!({ "target": "tab/pc/nope" }));
    assert!(shown["error"].is_string(), "{shown}");
    let (_, status) = tool(&mut mcp, "price_status", json!({}));
    let unmatched = status["rows"]["unmatched"].as_array().unwrap();
    assert!(
        unmatched.contains(&json!({ "scope": "tab", "realm": "pc", "id": "nope" })),
        "{status}"
    );
    let row = Annotations::open_for(&by_mcp.0.join("store").join("mock"), UUID)
        .unwrap()
        .get("tab", "pc/nope", "buyout")
        .unwrap()
        .unwrap();
    assert_eq!(row.written_via, "mcp");
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
    let dir = scratch("blind");
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
    assert_eq!(refusal, by_cli);
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
