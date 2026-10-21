//! The item search over MCP (`search/LEDGER.md`, step 11b; the
//! rulings H1–H5 of `search/LEDGER.md`, "11 · the opening"): `search`,
//! `describe` and `show` are adapters of `acquisition-search` as `acq
//! search` and `acq show` are, so what a tool returns is the document the
//! CLI's `--json` prints — the answer whole on success, `{"error",
//! "kind", "readings", "offers"}` in a result marked `isError` on a
//! refusal (the owner, 2026-10-10) — and every request an answer or a
//! refusal holds is sent back as it was printed. The acceptance set's
//! agent questions, AQ1–AQ5, are asked through the tool over a fixture
//! whose every count is worked by hand below. Facts are seeded through
//! the store crate; no daemon runs.

mod harness;

use std::path::{Path, PathBuf};

use acquisition_store::{Endpoint, Index, Store, account_path};
use harness::{Mcp, acq, no_daemon_appeared};
use harness::{Scratch, scratch};
use serde_json::{Value, json};

const USER: &str = "Alice#1234";

/// A scratch world, a server over it, and the `acq` beside the server.
struct Seat {
    base: PathBuf,
    mcp: Mcp,
    files: usize,
    /// Removed last, after the server is gone.
    _scratch: Scratch,
}

/// Where the server and the `acq` beside it look for the mock's stores.
fn stores(base: &Path) -> PathBuf {
    base.join("store").join("mock")
}

fn item(id: &str, name: &str, base: &str, rarity: &str, more: Value) -> Value {
    let mut v = json!({ "id": id, "name": name, "typeLine": base, "baseType": base, "rarity": rarity,
                        "frameTypeId": rarity, "identified": true, "ilvl": 84, "x": 0, "y": 0 });
    for (k, value) in more.as_object().unwrap() {
        v[k] = value.clone();
    }
    v
}

fn record(store: &mut Store, ep: Endpoint, realm: &str, body: Value, at: i64) {
    store
        .record(
            &ep,
            &json!({ "realm": realm, "league": "Standard" }),
            200,
            &body,
            at,
        )
        .unwrap();
}

fn stash(realm: &str, id: &str) -> Endpoint {
    Endpoint::Stash {
        realm: realm.into(),
        league: "Standard".into(),
        id: id.into(),
        sub: None,
    }
}

/// Five pc items in two tabs of Standard, every count below worked from
/// this list:
///
/// | id | tab | rarity | base | lines |
/// | --- | --- | --- | --- | --- |
/// | `r1` | Rings | rare | Two-Stone Ring | life 95, fire res 65 |
/// | `r2` | Rings | rare | Two-Stone Ring | life 20 (implicit) and 75, fire res 40 |
/// | `kaom` | Dump | unique | Glorious Plate | life 500 |
/// | `plain` | Dump | rare | Iron Ring | mana 20 |
/// | `str` | Dump | rare | Iron Ring | strength 25 |
fn seed(base: &Path) {
    let dir = stores(base);
    std::fs::create_dir_all(&dir).unwrap();
    let mut index = Index::load(&dir).unwrap();
    index.record_login(USER, "u-search", false, 1).unwrap();
    let mut store = Store::open(&account_path(&dir, USER)).unwrap();
    record(
        &mut store,
        Endpoint::Profile,
        "pc",
        json!({ "uuid": "u-search", "name": USER }),
        1,
    );
    record(
        &mut store,
        Endpoint::Stashes {
            realm: "pc".into(),
            league: "Standard".into(),
        },
        "pc",
        json!({ "stashes": [
            { "id": "t1", "name": "Rings", "type": "PremiumStash" },
            { "id": "t2", "name": "Dump", "type": "PremiumStash" } ] }),
        10,
    );
    record(
        &mut store,
        stash("pc", "t1"),
        "pc",
        json!({ "stash": { "id": "t1", "name": "Rings", "type": "PremiumStash", "items": [
            item("r1", "Doom Loop", "Two-Stone Ring", "Rare", json!({ "explicitMods":
                ["+95 to maximum Life", "+65% to Fire Resistance"] })),
            item("r2", "Rune Coil", "Two-Stone Ring", "Rare", json!({ "implicitMods": ["+20 to maximum Life"],
                "explicitMods": ["+75 to maximum Life", "+40% to Fire Resistance"] })),
        ] } }),
        20,
    );
    record(
        &mut store,
        stash("pc", "t2"),
        "pc",
        json!({ "stash": { "id": "t2", "name": "Dump", "type": "PremiumStash", "items": [
            item("kaom", "Kaom's Heart", "Glorious Plate", "Unique", json!({ "explicitMods": ["+500 to maximum Life"] })),
            item("plain", "Dull Turn", "Iron Ring", "Rare", json!({ "explicitMods": ["+20 to maximum Mana"] })),
            item("str", "Grim Knot", "Iron Ring", "Rare", json!({ "explicitMods": ["+25 to Strength"] })),
        ] } }),
        30,
    );
}

/// A second account, with one item of its own.
fn seed_bob(base: &Path) {
    let dir = stores(base);
    let mut index = Index::load(&dir).unwrap();
    index.record_login("Bob#9", "u-bob", false, 2).unwrap();
    let mut store = Store::open(&account_path(&dir, "Bob#9")).unwrap();
    record(
        &mut store,
        Endpoint::Profile,
        "pc",
        json!({ "uuid": "u-bob", "name": "Bob#9" }),
        1,
    );
    record(
        &mut store,
        Endpoint::Stashes {
            realm: "pc".into(),
            league: "Standard".into(),
        },
        "pc",
        json!({ "stashes": [{ "id": "b1", "name": "Bobs", "type": "PremiumStash" }] }),
        10,
    );
    record(
        &mut store,
        stash("pc", "b1"),
        "pc",
        json!({ "stash": { "id": "b1", "name": "Bobs", "type": "PremiumStash", "items": [
            item("bob1", "Lone Band", "Iron Ring", "Rare", json!({})),
        ] } }),
        20,
    );
}

/// What the CLI's `--json` printed: the document, and whether it exited 0.
fn cli(base: &Path, args: &[&str]) -> (Value, bool) {
    let out = acq(base, &[&["--json"], args].concat());
    let stdout = String::from_utf8(out.stdout.clone()).unwrap();
    let doc = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!(
            "acq {args:?} printed no one JSON document ({e}):\n{stdout}\n{}",
            String::from_utf8_lossy(&out.stderr)
        )
    });
    (doc, out.status.success())
}

impl Seat {
    /// The seeded fixture and a server over it.
    fn new() -> Seat {
        let scratch = scratch("mcp-search");
        seed(&scratch.0);
        Seat::over(scratch)
    }

    fn over(scratch: Scratch) -> Seat {
        let base = scratch.0.clone();
        let mcp = Mcp::start(&base, &[]);
        Seat {
            base,
            mcp,
            files: 0,
            _scratch: scratch,
        }
    }

    /// One tool call: the document, and whether the result was marked an
    /// error. H4: the JSON whole and no text — the result's one content
    /// block is that same document, for a host that reads no structured
    /// content, and nothing else stands beside it.
    fn call(&mut self, tool: &str, args: &Value) -> (Value, bool) {
        let result = self.mcp.result(tool, args.clone());
        let doc = result["structuredContent"].clone();
        assert!(doc.is_object(), "{tool} {args}: {result}");
        let content = result["content"].as_array().unwrap();
        assert_eq!(content.len(), 1, "{tool} {args}: {result}");
        assert_eq!(
            serde_json::from_str::<Value>(content[0]["text"].as_str().unwrap()).unwrap(),
            doc,
            "{tool} {args}"
        );
        (doc, result["isError"] == json!(true))
    }

    /// A request through `search`: the document, and whether it was a
    /// refusal. That it is `acq search --request`'s is one test,
    /// `the_search_tool_is_acq_search_request`; both doors call the same
    /// functions (C46), so the rest ask the tool alone.
    fn search(&mut self, request: &Value) -> (Value, bool) {
        self.call("search", request)
    }

    /// A request through `search` held equal to the same request through
    /// `acq search --request`: one document, and a refusal on both or on
    /// neither.
    fn search_beside_the_cli(&mut self, request: &Value) -> (Value, bool) {
        self.files += 1;
        let file = self.base.join(format!("request-{}.json", self.files));
        std::fs::write(&file, request.to_string()).unwrap();
        let (printed, ok) = cli(&self.base, &["search", "--request", file.to_str().unwrap()]);
        let (doc, refused) = self.call("search", request);
        assert_eq!(doc, printed, "search {request}");
        assert_eq!(refused, !ok, "search {request}: {doc}");
        (doc, refused)
    }

    fn answer(&mut self, request: &Value) -> Value {
        let (doc, refused) = self.search(request);
        assert!(!refused, "search {request} was refused: {doc}");
        doc
    }

    fn refusal(&mut self, request: &Value) -> Value {
        let (doc, refused) = self.search(request);
        assert!(refused, "search {request} answered: {doc}");
        doc
    }
}

fn ids(answer: &Value) -> Vec<&str> {
    answer["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["id"].as_str().unwrap())
        .collect()
}

/// AQ3's adapter part: `show` forwards `against`, a query as a request
/// holds one, and the outcome is read back (r2's two life lines are 20
/// and 75: no one occurrence reaches 90). What a row names and a zero
/// answer offers are the search crate's (`answer.rs`).
#[test]
fn aq3_show_forwards_against_and_says_why_not() {
    let mut seat = Seat::new();
    let text = "\"+# to maximum Life\">=90";
    let (shown, refused) = seat.call("show", &json!({ "id": "r2", "against": { "text": text } }));
    assert!(!refused, "{shown}");
    assert_eq!(shown["against"]["outcome"], "no_match", "{shown}");
    assert_eq!(
        shown["against"]["query"]["text"], "line(\"# to maximum Life\" arg1>=90)",
        "{shown}"
    );
}

/// AQ4's adapter part: `show` with `body: true` is `acq show --body`,
/// the stored body verbatim.
#[test]
fn aq4_show_with_its_body_is_the_clis() {
    let mut seat = Seat::new();
    let (with_body, refused) = seat.call("show", &json!({ "id": "kaom", "body": true }));
    assert!(!refused, "{with_body}");
    assert!(
        with_body["body"].as_str().unwrap().contains("Kaom's Heart"),
        "{with_body}"
    );
    assert_eq!(with_body, cli(&seat.base, &["show", "kaom", "--body"]).0);
}

/// C109, C47: four hundred nested parentheses once aborted the server, which
/// is resident — a stack overflow, every later call unanswered. A query
/// nested past the limit is an authoring error like any other, the tree
/// as the text, and the server answers the next call.
#[test]
fn a_query_nested_too_deep_is_refused_and_the_server_answers_the_next_call() {
    let mut seat = Seat::new();
    for text in [
        format!("{}rarity=rare{}", "(".repeat(400), ")".repeat(400)),
        format!("{}ilvl>=3{}", "undecided(".repeat(102), ")".repeat(102)),
        format!("{}rarity=rare", "-".repeat(33)),
    ] {
        let e = seat.refusal(&json!({ "query": { "text": text } }));
        assert_eq!(e["kind"], "too_deep", "{e}");
        assert_eq!(
            ids(&seat.answer(&json!({ "query": { "text": "id:r1" } }))),
            ["r1"]
        );
    }
    let tree = (0..33).fold(
        json!({ "field": "rarity", "op": "=", "value": "rare" }),
        |inner, _| json!({ "not": inner }),
    );
    let e = seat.refusal(&json!({ "query": { "tree": tree } }));
    assert_eq!(e["kind"], "too_deep", "{e}");
    assert_eq!(
        ids(&seat.answer(&json!({ "query": { "text": "id:r1" } }))),
        ["r1"]
    );
}

/// An authoring error is the CLI's document in a result marked an error,
/// never a protocol error: its readings are what an agent reads next. One
/// on `search` and one on `show`; the kinds themselves are the search
/// crate's (`refusal.rs`).
#[test]
fn an_authoring_error_is_a_result_with_its_kind_and_readings() {
    let mut seat = Seat::new();
    let (e, is_error) = seat.search(&json!({ "query": { "text": "ring" } }));
    assert!(is_error, "{e}");
    assert_eq!(e["kind"], "bare_word", "{e}");
    assert!(!e["readings"].as_array().unwrap().is_empty(), "{e}");
    let (e, is_error) = seat.call(
        "show",
        &json!({ "id": "r1", "against": { "text": "ring" } }),
    );
    assert!(is_error, "{e}");
    assert_eq!(e["kind"], "bare_word", "{e}");
    assert!(!e["readings"].as_array().unwrap().is_empty(), "{e}");
}

/// A malformed `against`, one that is no query as a request holds one, is
/// refused before any store is read: `tree`, as the request's own
/// malformed query is, in a result marked an error.
#[test]
fn a_malformed_against_is_refused_as_a_tree() {
    let mut seat = Seat::new();
    for against in [
        json!({ "txt": "rarity=rare" }),
        json!(5),
        json!("rarity=rare"),
    ] {
        let (e, is_error) = seat.call("show", &json!({ "id": "r1", "against": against }));
        assert!(is_error, "{against}: {e}");
        assert_eq!(e["kind"], "tree", "{against}: {e}");
        assert!(
            e["error"]
                .as_str()
                .unwrap()
                .starts_with("`against` is a query as a request holds one"),
            "{against}: {e}"
        );
    }
}

/// `describe` is `acq search --describe`: the language whole, or the
/// entries named, with no store read — it answers over an empty world.
#[test]
fn describe_is_the_clis_and_reads_no_store() {
    let mut seat = Seat::over(scratch("mcp-search"));
    let (whole, refused) = seat.call("describe", &json!({}));
    assert!(!refused, "{whole}");
    assert_eq!(whole, cli(&seat.base, &["search", "--describe"]).0);
    let (some, _) = seat.call("describe", &json!({ "names": ["league", "total_res"] }));
    assert_eq!(
        some,
        cli(&seat.base, &["search", "--describe", "league,total_res"]).0
    );
    assert_ne!(some, whole);
    let (e, is_error) = seat.call("describe", &json!({ "names": ["leauge"] }));
    assert!(is_error, "{e}");
    assert_eq!(e["kind"], "unknown_name");
    assert_eq!(e, cli(&seat.base, &["search", "--describe", "leauge"]).0);
}

/// F10 of the agent's seat (R18): where no account's store is found the
/// refusal names the directory it looked in, on `search` and on `show`.
#[test]
fn a_missing_store_names_the_directory_it_looked_in() {
    let mut seat = Seat::over(scratch("mcp-search"));
    let looked = stores(&seat.base);
    for (tool, args) in [("search", json!({})), ("show", json!({ "id": "r1" }))] {
        let (e, is_error) = seat.call(tool, &args);
        assert!(is_error, "{tool}: {e}");
        assert_eq!(e["kind"], "not_logged_in", "{tool}: {e}");
        assert_eq!(e["accounts"], json!([]), "{tool}: {e}");
        assert!(
            e["error"]
                .as_str()
                .unwrap()
                .contains(looked.to_str().unwrap()),
            "{tool}: {e}"
        );
        // and names no login for an agent to run: that is a human's
        assert!(!e["error"].as_str().unwrap().contains("acq auth"), "{e}");
    }
    // the older store tools say the same of it: the refusal document,
    // decoded before the path is looked for in it — serialised, a
    // Windows path's backslashes are doubled, and the Windows gate of
    // 2026-10-19 missed the directory in the text it had named
    let message = seat.mcp.expect_err("store_status", json!({}));
    let e: Value = serde_json::from_str(&message).unwrap_or_else(|_| panic!("{message}"));
    assert_eq!(e["kind"], "not_logged_in", "{e}");
    let error = e["error"].as_str().unwrap();
    assert!(error.contains(looked.to_str().unwrap()), "{e}");
    assert!(!error.contains("acq auth"), "{e}");
}

/// H2: the account selector's mapping. `scope.account` selects the store
/// a search reads, a continuation carries it, and `show`'s `account`
/// selects by name or uuid. What an account that cannot be picked is
/// refused as is the verbs crate's (C121; `tool_refusals.rs` for this
/// surface's).
#[test]
fn the_requests_account_selects_the_store() {
    let scratch = scratch("mcp-search");
    let base = scratch.0.clone();
    seed(&base);
    seed_bob(&base);
    let mut seat = Seat::over(scratch);
    let (bob, refused) = seat.call("search", &json!({ "scope": { "account": "bob" } }));
    assert!(!refused, "{bob}");
    assert_eq!(ids(&bob), ["bob1"]);
    let (alice, _) = seat.call(
        "search",
        &json!({ "scope": { "account": USER }, "view": { "rows": { "limit": 2 } } }),
    );
    assert_eq!(ids(&alice), ["r1", "r2"]);
    // every request an answer holds names the account it was answered for
    let (second, refused) = seat.call("search", &alice["view"]["rows"]["next"]);
    assert!(!refused, "{second}");
    assert_eq!(ids(&second), ["kaom", "plain"]);
    let (shown, refused) = seat.call("show", &json!({ "id": "bob1", "account": "u-bob" }));
    assert!(!refused, "{shown}");
    assert_eq!(shown["item"]["facts"]["id"], "bob1");
}

/// No daemon is started by any of the three: none bound a socket in the
/// scratch world. (That none was reached for is the crate's edges: the
/// search links no client, `tools/arch-check.sh`.)
#[test]
fn no_tool_of_the_search_starts_a_daemon() {
    let mut seat = Seat::new();
    seat.answer(&json!({ "query": { "text": "rarity=rare" } }));
    seat.call("describe", &json!({}));
    seat.call("show", &json!({ "id": "r1" }));
    assert!(no_daemon_appeared(&seat.base));
}

/// Arguments a tool does not take are refused in the same document as
/// any refusal — `error` and `kind` in a result marked an error — and
/// a key it does not know is never read past.
#[test]
fn arguments_a_tool_does_not_take_are_refused_as_a_document() {
    let mut seat = Seat::new();
    for (tool, args) in [
        ("show", json!({})),
        ("show", json!({ "id": 5 })),
        ("show", json!({ "id": "r1", "body": "yes" })),
        (
            "show",
            json!({ "id": "r1", "query": { "text": "rarity=rare" } }),
        ),
        ("describe", json!({ "names": "league" })),
        ("describe", json!({ "name": ["league"] })),
    ] {
        let (e, is_error) = seat.call(tool, &args);
        assert!(is_error, "{tool} {args}: {e}");
        assert_eq!(e["kind"], "arguments", "{tool} {args}: {e}");
        assert!(e["error"].is_string(), "{tool} {args}: {e}");
    }
    // no arguments at all is every tool's empty object
    let result = seat.mcp.result("describe", Value::Null);
    assert!(result["isError"] != json!(true), "{result}");
    assert!(result["structuredContent"]["fields"].is_array(), "{result}");
}

/// `describe`'s text says how large the whole language is and which
/// sections are small: held here, so that the text is changed when the
/// language outgrows it.
#[test]
fn describe_is_as_large_as_its_text_says() {
    let mut seat = Seat::over(scratch("mcp-search"));
    let size = |seat: &mut Seat, names: Value| {
        let (doc, refused) = seat.call("describe", &json!({ "names": names }));
        assert!(!refused, "{names}: {doc}");
        doc.to_string().len()
    };
    let (whole, _) = seat.call("describe", &json!({}));
    assert!(whole.to_string().len() > 100_000);
    assert!(size(&mut seat, json!(["computed"])) > 50_000);
    // some 20 KB, the trade catalogue's rows (C113), each what the report
    // says of it since the trade seat (21,633 bytes then)
    assert!(size(&mut seat, json!(["trade"])) < 25_000);
    assert!(size(&mut seat, json!(["trade"])) > 18_000);
    // some 20 KB since the property fields joined it (C114)
    assert!(size(&mut seat, json!(["fields"])) < 24_000);
    assert!(size(&mut seat, json!(["fields"])) > 16_000);
    for section in [
        "line",
        "linked",
        "values",
        "composition",
        "operators",
        "slots",
        "counts",
        "explain",
    ] {
        assert!(size(&mut seat, json!([section])) < 4_000, "{section}");
    }
    let tools = seat.mcp.list_tools();
    let text = tools
        .iter()
        .find(|t| t["name"] == "describe")
        .and_then(|t| t["description"].as_str())
        .unwrap();
    // The sizes the text states are the ones measured above: a text
    // that says other numbers fails here and is checked again.
    for stated in [
        "The whole is over 100 KB",
        "fields (some 20 KB)",
        "(each under 4 KB)",
        "trade (some 20 KB)",
    ] {
        assert!(text.contains(stated), "{stated}: {text}");
    }
    for section in [
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
        "trade",
    ] {
        assert!(text.contains(section), "{section}: {text}");
    }
}

/// TRADE_ID of the CLI's `tests/search_json.rs`, the same search: the
/// status listing-only, the explicit life lines' sum at 70 or more, and a
/// crucible tree path in the remainder (a `count` with no bound until step
/// 7f made it the site's no-op).
const TRADE_ID: &str = "H4sIAAAAAAAA_5XM0QrCMAyF4XfJdZFhB2N7FRlSuwiB2JYtFUfJu5speO9t_pyvwSZB6gZTg1yEcoIJcmJKCOo-0dqlgewFLYW0gIM7seD6DbTYGV-FKZKcjsHVn8fR90PXe_t9Bq548A8ye-hUZ3U_L6410o3xL9SEWd8TZHM0ugAAAA";

/// C113, step 5 of the trade brief: `show`'s `against` takes a site search
/// as a query, its report beside the text, and only in the item's realm
/// (K6; the outside review of step 5: an xbox URL showed a pc item a
/// match). The request-level rules (a continuation keeps the site search,
/// an edited text, a realm beside a URL, a trade that is none) are the
/// search crate's (`next.rs`).
#[test]
fn a_site_search_is_a_query_and_its_report_stands_in_the_answer() {
    let mut seat = Seat::new();
    let url = format!("https://www.pathofexile.com/trade/search/Standard/{TRADE_ID}");
    let (first, refused) = seat.search(&json!({ "query": { "trade": url } }));
    assert!(!refused, "{first}");
    assert_eq!(first["total"]["matched"], 3);
    let (shown, refused) = seat.call(
        "show",
        &json!({ "id": "plain", "against": { "trade": url } }),
    );
    assert!(!refused, "{shown}");
    assert_eq!(shown["against"]["query"]["text"], first["query"]["text"]);
    assert_eq!(shown["against"]["query"]["trade"], first["query"]["trade"]);
    assert_eq!(shown["against"]["outcome"], "no_match");
    let xbox = url.replace("/search/", "/search/xbox/");
    let (e, refused) = seat.call("show", &json!({ "id": "r1", "against": { "trade": xbox } }));
    assert!(refused, "{e}");
    assert_eq!(e["kind"], "trade", "{e}");
}

/// A reader took a trade answer's `fidelity: exact` for "the site would
/// show me the same", reading "partial … the answer then not the site's"
/// as its converse (the trade seat: mcp-opus calls 1, 11, its worst
/// fault). The `query` field's text says what fidelity measures and that
/// the items are the store's, never the site's listings.
#[test]
fn a_trade_fidelity_is_never_said_to_be_the_sites_items() {
    let mut seat = Seat::new();
    let tools = seat.mcp.list_tools();
    let search = tools.iter().find(|t| t["name"] == "search").unwrap();
    let query = search["inputSchema"]["properties"]["query"]["description"]
        .as_str()
        .unwrap();
    // the fact words; the description whole is MCP-REFERENCE.md's
    for said in [
        "how far the text is the site's search",
        "Never how far the items are the site's",
    ] {
        assert!(query.contains(said), "{said}: {query}");
    }
}

/// `scope.never_fetched` was printed beside every answer and explained
/// nowhere: a reader could not say what it counts or that an answer is
/// blind to it (the trade seat: mcp-opus call 3, its fault 4; mcp-sonnet
/// call 4, its faults 3 and 5 and its debrief's answer 5).
#[test]
fn the_never_fetched_count_says_what_it_counts_and_what_no_answer_sees() {
    let mut seat = Seat::new();
    let tools = seat.mcp.list_tools();
    let search = tools.iter().find(|t| t["name"] == "search").unwrap();
    let text = search["description"].as_str().unwrap();
    for said in [
        "`never_fetched` counts the tabs, their pages and the characters",
        "no answer says what they hold",
    ] {
        assert!(text.contains(said), "{said}: {text}");
    }
}

/// A pattern term's `resolved` values were read as the answer's tier
/// breakdown, tier 13 and other leagues among them, and `scope.items` as
/// the matches' base (the trade seat: mcp-sonnet calls 4, 19, 20, its
/// faults 1 and 2). The text says each term is alone over the scope,
/// never over the matches, what `resolved` is, and what `items` counts.
#[test]
fn a_terms_counts_and_resolved_values_are_said_to_be_over_the_scope() {
    let mut seat = Seat::new();
    let tools = seat.mcp.list_tools();
    let search = tools.iter().find(|t| t["name"] == "search").unwrap();
    let text = search["description"].as_str().unwrap();
    for said in [
        "never over the matches, which are `total`",
        "`resolved` is what it matched there",
        "`items` every live item in scope before any term",
    ] {
        assert!(text.contains(said), "{said}: {text}");
    }
}

/// `against` gave `match` for a group and `matched` for each term, two
/// words a reader took for one outcome (the trade seat: mcp-opus call 37,
/// its fault 8). By design a term counts and a group is true or false;
/// the `show` text names both vocabularies.
#[test]
fn show_names_a_terms_outcomes_and_a_groups() {
    let mut seat = Seat::new();
    let tools = seat.mcp.list_tools();
    let show = tools.iter().find(|t| t["name"] == "show").unwrap();
    let text = show["description"].as_str().unwrap();
    for said in [
        "a term's `matched`, `failed`, `lacked` or `undecided`",
        "a group's `match`, `no_match` or `undecided`",
    ] {
        assert!(text.contains(said), "{said}: {text}");
    }
}

/// The keys of the JSON object a text opens with, in the order the text
/// writes them: what an agent reading the result as text meets first.
fn keys_in_order(text: &str) -> Vec<String> {
    let mut keys = Vec::new();
    let (mut depth, mut in_string, mut escaped) = (0usize, false, false);
    let mut word = String::new();
    let mut after_word = false;
    for c in text.chars() {
        if in_string {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
                after_word = depth == 1;
                continue;
            }
            if depth == 1 {
                word.push(c);
            }
            continue;
        }
        match c {
            '"' => {
                in_string = true;
                word.clear();
            }
            ':' if depth == 1 && after_word => keys.push(word.clone()),
            '{' | '[' => depth += 1,
            '}' | ']' => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }
        if c != '"' && !c.is_whitespace() {
            after_word = false;
        }
    }
    keys
}

/// C115: "I learned to scroll for `total`" (the trade seat, mcp-opus-2):
/// the result's text held the answer's keys in alphabetical order —
/// basis, query, rows, scope, terms, total, view — so the terms block and
/// the basis stood before the total. The text now holds them in the
/// order a reader needs: the query, the scope, the total, the rows and
/// the view with its way on, then the terms and the basis; a site
/// search's report opens the query, as it opens the terminal's answer.
#[test]
fn the_answers_text_leads_with_its_total_never_with_its_terms() {
    let mut seat = Seat::new();
    let text_of = |seat: &mut Seat, args: Value| -> String {
        let result = seat.mcp.result("search", args);
        result["content"][0]["text"].as_str().unwrap().to_string()
    };
    let text = text_of(&mut seat, json!({ "query": { "text": "class:ring" } }));
    assert_eq!(
        keys_in_order(&text),
        ["query", "scope", "total", "rows", "view", "terms", "basis"],
        "{text}"
    );
    let url = format!("https://www.pathofexile.com/trade/search/Standard/{TRADE_ID}");
    let text = text_of(&mut seat, json!({ "query": { "trade": url } }));
    let query = text
        .strip_prefix("{\"query\":")
        .unwrap_or_else(|| panic!("{text}"));
    assert_eq!(keys_in_order(query), ["trade", "text", "tree"], "{text}");
    let counted = text_of(
        &mut seat,
        json!({ "view": { "counts": { "keys": ["tab"] } } }),
    );
    assert_eq!(
        keys_in_order(&counted),
        ["query", "scope", "total", "rows", "view", "terms", "basis"],
        "{counted}"
    );
}

/// C115: `tabs` answered the owner's Standard in 745,672 bytes with no
/// limit, cut to its head by the door in three of the trade seat's runs
/// (mcp-opus-1 call 9, mcp-opus-2 call 10, mcp-sonnet-2 call 6). The
/// listing rule is the search crate's (`tabs.rs`); here, the adapter's
/// part over 41 tabs: the answer's size, the counts first in its text,
/// `expand` mapped, and the document `acq tabs --json` prints.
#[test]
fn tabs_counts_first_and_lists_the_ten_with_the_most_items() {
    let mut seat = Seat::new();
    forty_one_tabs(&seat.base, "QuadStash");
    let result = seat.mcp.result("tabs", json!({}));
    let text = result["content"][0]["text"].as_str().unwrap().to_string();
    assert!(text.len() < 10_000, "{} bytes", text.len());
    assert!(
        text.starts_with(
            r#"{"realm":"pc","league":"Standard","tabs":41,"fetched":2,"never_fetched":39,"#
        ),
        "{text}"
    );
    let (tabs, refused) = seat.call("tabs", &json!({}));
    assert!(!refused, "{tabs}");
    assert_eq!(tabs["rows"].as_array().unwrap().len(), 10);
    let (printed, ok) = cli(&seat.base, &["tabs"]);
    assert!(ok);
    assert_eq!(printed, tabs);

    let (all, _) = seat.call("tabs", &json!({ "expand": true }));
    assert_eq!(all["rows"].as_array().unwrap().len(), 41);
    let (printed, _) = cli(&seat.base, &["tabs", "--expand"]);
    assert_eq!(printed, all);
}

/// Pc Standard listed again as 41 tabs: Rings and Dump, and 39 of `kind`
/// never fetched.
fn forty_one_tabs(base: &Path, kind: &str) {
    let mut store = Store::open(&account_path(&stores(base), USER)).unwrap();
    let mut stashes = vec![
        json!({ "id": "t1", "name": "Rings", "type": "PremiumStash" }),
        json!({ "id": "t2", "name": "Dump", "type": "PremiumStash" }),
    ];
    stashes.extend(
        (0..39).map(|i| json!({ "id": format!("q{i}"), "name": format!("Q {i}"), "type": kind })),
    );
    record(
        &mut store,
        Endpoint::Stashes {
            realm: "pc".into(),
            league: "Standard".into(),
        },
        "pc",
        json!({ "stashes": stashes }),
        40,
    );
}

/// The search tool's words on a site search name the field a buyer's
/// view asks (`tab.public`), the three values `fidelity` takes, and a
/// sum as a sort value spanning every source: the re-sit's four runs
/// found `tab.public` from "public listings" never, met `ambiguous` as a
/// fidelity the text did not name (mcp-opus), and sorted by a sum the
/// text did not say spans sources (cli-opus, cli-sonnet).
#[test]
fn the_search_tools_words_name_tab_public_every_fidelity_and_a_sums_sources() {
    let mut seat = Seat::over(scratch("mcp-search"));
    let tools = seat.mcp.list_tools();
    let search = tools.iter().find(|t| t["name"] == "search").unwrap();
    let query = search["inputSchema"]["properties"]["query"]["description"]
        .as_str()
        .unwrap();
    for said in [
        "a tab's being `tab.public`",
        "exact, ambiguous where a clause ran as the site may not read it",
        "partial where the remainder is not empty",
    ] {
        assert!(query.contains(said), "{said}: {query}");
    }
    let view = search["inputSchema"]["properties"]["view"]["description"]
        .as_str()
        .unwrap();
    assert!(
        view.contains("spanning every source unless `source=` names one"),
        "{view}"
    );
}

/// The never fetched by name (the re-sit, every run's first or second
/// ask: mcp-opus calls 34-37, mcp-sonnet 18-23): `type` and
/// `never_fetched` map to the selection, held equal to `acq tabs --type …
/// --never-fetched --json`, and `expand` lists the selection whole; what
/// the selection holds is the search crate's (`tabs.rs`).
#[test]
fn tabs_lists_the_never_fetched_of_one_type() {
    let mut seat = Seat::new();
    forty_one_tabs(&seat.base, "UniqueStash");
    let (tabs, refused) = seat.call(
        "tabs",
        &json!({ "type": "UniqueStash", "never_fetched": true }),
    );
    assert!(!refused, "{tabs}");
    assert_eq!(
        tabs["selection"],
        json!({ "type": "UniqueStash", "never_fetched": true })
    );
    let (printed, ok) = cli(
        &seat.base,
        &["tabs", "--type", "UniqueStash", "--never-fetched"],
    );
    assert!(ok);
    assert_eq!(printed, tabs);
    let (all, _) = seat.call(
        "tabs",
        &json!({ "type": "UniqueStash", "never_fetched": true, "expand": true }),
    );
    assert_eq!(all["rows"].as_array().unwrap().len(), 39, "{all}");
}

/// A row's place carried no age (the trade seat: mcp-sonnet-2 calls 3, 14,
/// 24, its fault 4). C118: the place carries its location's `fetched_at`,
/// and the tool's text names it.
#[test]
fn a_rows_place_carries_when_its_location_was_fetched() {
    let mut seat = Seat::over(scratch("mcp-search"));
    let tools = seat.mcp.list_tools();
    let search = tools.iter().find(|t| t["name"] == "search").unwrap();
    let text = search["description"].as_str().unwrap();
    assert!(text.contains("(`place.fetched_at`)"), "{text}");
}

/// `scope.never_fetched` stood unsized and which tabs it counted could not
/// be learned (the trade seat: mcp-opus-2 calls 3, 10, its fault 2;
/// mcp-sonnet-2 reading 2,940 as items). C118: the tool's text says the
/// scope counts the never fetched by tab type as `tabs` counts a type, and
/// names `tabs` as the list.
#[test]
fn the_scope_counts_the_never_fetched_by_tab_type_and_names_the_list() {
    let mut seat = Seat::over(scratch("mcp-search"));
    let tools = seat.mcp.list_tools();
    let search = tools.iter().find(|t| t["name"] == "search").unwrap();
    let text = search["description"].as_str().unwrap();
    for said in [
        "`never_fetched_by_type` is the same by tab type",
        "`tabs` lists a league's tabs",
        "`never_fetched_leagues`",
    ] {
        assert!(text.contains(said), "{said}: {text}");
    }
}

/// The equality the frontends rest on, once (C46): a request through the
/// `search` tool is the document `acq search --request` prints, a refusal
/// on both or on neither, over the four kinds of request an agent sends:
/// one it wrote, one refused, a continuation and an offer, each sent back
/// as printed.
#[test]
fn the_search_tool_is_acq_search_request() {
    let mut seat = Seat::new();
    let (answer, refused) = seat.search_beside_the_cli(
        &json!({ "query": { "text": "rarity=rare" }, "view": { "rows": { "limit": 2 } } }),
    );
    assert!(!refused, "{answer}");
    assert_eq!(ids(&answer), ["r1", "r2"]);
    let (e, refused) = seat.search_beside_the_cli(&json!({ "query": { "text": "ring" } }));
    assert!(refused, "{e}");
    assert_eq!(e["kind"], "bare_word", "{e}");
    let (next, refused) = seat.search_beside_the_cli(&answer["view"]["rows"]["next"]);
    assert!(!refused, "{next}");
    assert_eq!(ids(&next), ["plain", "str"]);
    let (zero, _) = seat.search(&json!({ "query": { "text": "rarity=rare name:kaom" } }));
    assert_eq!(zero["total"]["matched"], 0, "{zero}");
    let (offered, refused) = seat.search_beside_the_cli(&zero["zero"]["offers"]["context"]);
    assert!(!refused, "{offered}");
    assert_eq!(offered["total"]["matched"], 5, "{offered}");
}

/// `item_events`' window: `hours` how far back, 24 by default, and
/// `limit` how many of the newest. The fixture's fetches landed in 1970,
/// so the default window holds none of their events and a window back to
/// then holds them, newest first.
#[test]
fn item_events_reads_back_its_hours_and_defaults_to_a_day() {
    let mut seat = Seat::new();
    // an array, not a document: read through the harness's `expect_ok`,
    // which takes the structured content whatever its shape
    let recent = seat.mcp.expect_ok("item_events", json!({}));
    assert_eq!(recent, json!([]), "{recent}");
    let all = seat.mcp.expect_ok("item_events", json!({ "hours": 1.0e9 }));
    let all = all.as_array().unwrap_or_else(|| panic!("{all}")).clone();
    assert_eq!(all.len(), 5, "{all:?}");
    let at: Vec<i64> = all.iter().map(|e| e["at"].as_i64().unwrap()).collect();
    assert!(at.windows(2).all(|w| w[0] >= w[1]), "newest first: {at:?}");
    let one = seat
        .mcp
        .expect_ok("item_events", json!({ "hours": 1.0e9, "limit": 1 }));
    assert_eq!(one.as_array().unwrap().len(), 1, "{one}");
    assert_eq!(one[0], all[0]);
}
