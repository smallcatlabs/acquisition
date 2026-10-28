//! A tool's refusal, on every tool: the request was wrong, or the store
//! or policy cannot serve it, and the answer is a result marked
//! `isError` holding `error` and a `kind` — the instructions' promise,
//! which the search and pricing tools kept and the store, intent and
//! plan tools did not (the release seat's rerun, mcp-opus-2 calls 50 and
//! 46: an unknown account on `tabs`, an uncovered league on
//! `refresh_plan`, both JSON-RPC `-32603`). A call the server could not
//! serve stays a JSON-RPC error: a daemon's own refusal carries its
//! closed kind in `data` (C85).

use crate::harness;

use std::time::{Duration, Instant};

use acquisition_plan::{PolicyWrite, put_sync_policy};
use acquisition_store::{Annotations, Endpoint, Index, Provenance, Store, account_path};
use harness::{Mcp, spawn_daemon};
use serde_json::{Value, json};

const USER: &str = "Alice#1234";
const UUID: &str = "u-refusal";

/// One account with a store and a sync policy covering Standard alone.
fn seed(base: &std::path::Path) {
    let mock = base.join("store").join("mock");
    std::fs::create_dir_all(&mock).unwrap();
    let at = acquisition_store::now();
    Index::load(&mock)
        .unwrap()
        .record_login(USER, UUID, false, at, &[])
        .unwrap();
    Store::open(&account_path(&mock, USER))
        .unwrap()
        .record(
            &Endpoint::Profile,
            &json!({}),
            200,
            &json!({ "uuid": UUID, "name": USER }),
            at,
        )
        .unwrap();
    let mut a = Annotations::open_for(&mock, UUID).unwrap();
    put_sync_policy(
        &mut a,
        &json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard":
            { "tabs": "all", "max_age_seconds": 86400 } } } } }),
        None,
        PolicyWrite::Replace,
        &Provenance::via("test"),
    )
    .unwrap();
}

/// The result of a call the server answered, its document and kind
/// checked: a JSON-RPC error fails here, naming the tool.
fn refusal(mcp: &mut Mcp, tool: &str, args: Value, kind: &str) -> Value {
    let result = mcp.result(tool, args.clone());
    let doc = harness::document(&result);
    assert!(
        result["isError"] == json!(true) && doc["kind"] == json!(kind) && doc["error"].is_string(),
        "{tool} {args} is not a refusal of kind {kind}: {result}"
    );
    doc
}

#[test]
fn a_tool_refusal_is_a_result_marked_is_error_with_its_kind() {
    let scratch = harness::scratch("mcp-refusal");
    let base = scratch.0.clone();
    seed(&base);
    let mut mcp = Mcp::start(&base, &[]);

    // An account no store knows, on every tool that selects one: the
    // protocol's word for it (C121), the known names as accounts.
    for (tool, mut args) in [
        ("tabs", json!({})),
        ("characters", json!({})),
        ("store_status", json!({})),
        ("item_events", json!({})),
        ("sync_policy", json!({})),
        (
            "set_sync_policy",
            json!({ "value": { "version": 3, "realms": {} } }),
        ),
        ("refresh_plan", json!({})),
    ] {
        args["account"] = json!("GERWARIK");
        let doc = refusal(&mut mcp, tool, args, "not_logged_in");
        assert_eq!(doc["accounts"], json!([USER]), "{tool}: {doc}");
    }

    // A league the policy does not cover: refused in the planner's words.
    let doc = refusal(
        &mut mcp,
        "refresh_plan",
        json!({ "league": "Allflame" }),
        "refused",
    );
    assert!(
        doc["error"]
            .as_str()
            .unwrap()
            .contains("does not cover league Allflame"),
        "{doc}"
    );

    // A parameter the tool cannot read.
    refusal(&mut mcp, "tabs", json!({ "realm": "pcx" }), "arguments");
    refusal(
        &mut mcp,
        "item_events",
        json!({ "hours": "a day" }),
        "arguments",
    );

    // A daemon's own refusal is still a JSON-RPC error with its kind.
    let _daemon = spawn_daemon(&base, &[]);
    let deadline = Instant::now() + Duration::from_secs(10);
    while mcp.expect_ok("daemon_status", json!({}))["running"] != json!(true) {
        assert!(Instant::now() < deadline, "the mock daemon did not come up");
        std::thread::sleep(Duration::from_millis(50));
    }
    let error = mcp.expect_rpc_error("job_status", json!({ "id": 999 }));
    assert_eq!(error["data"]["kind"], "unknown_job", "{error}");
    assert_eq!(error["message"], "no job 999", "{error}");
}

/// C127 (one list of the kinds a client may submit): `submit_job`'s
/// `kind` names the kinds the daemon's own refusal of an unknown kind
/// lists, word for word, so the description cannot name a kind the door
/// refuses (`refresh`, retired) or leave out one it admits (`apply`), as
/// it once did both.
#[test]
fn submit_jobs_kind_names_the_kinds_the_daemons_refusal_lists() {
    let scratch = harness::scratch("mcp-kinds");
    let base = scratch.0.clone();
    let mut mcp = Mcp::start(&base, &[]);
    let _daemon = spawn_daemon(&base, &[]);
    let deadline = Instant::now() + Duration::from_secs(10);
    while mcp.expect_ok("daemon_status", json!({}))["running"] != json!(true) {
        assert!(Instant::now() < deadline, "the mock daemon did not come up");
        std::thread::sleep(Duration::from_millis(50));
    }
    let error = mcp.expect_rpc_error("submit_job", json!({ "kind": "refresh" }));
    assert_eq!(error["data"]["kind"], "refused", "{error}");
    let message = error["message"].as_str().unwrap_or_default();
    let (_, listed) = message
        .split_once("a client may submit: ")
        .unwrap_or_else(|| panic!("the refusal lists no kinds: {message}"));
    let tools = mcp.list_tools();
    let submit = tools
        .iter()
        .find(|t| t["name"] == "submit_job")
        .expect("submit_job is a tool");
    let described = submit["inputSchema"]["properties"]["kind"]["description"]
        .as_str()
        .unwrap_or_default();
    assert!(
        described.contains(&format!("one a client may submit: {listed}.")),
        "the description: {described}\nthe refusal: {message}"
    );
    assert!(!described.contains("refresh"), "{described}");
}

/// A request the server cannot read gets its own reply, and the server
/// answers the next call: a query tree of 123 nots or 62 groups, or any
/// message past 127 JSON levels, was dropped before any tool saw it,
/// the caller waiting on it (the frontends park, now built). A tool call
/// nested past the limit is a refusal like any other, `arguments`, its
/// words naming the limit; a line that is not JSON is a parse error with
/// a null id.
#[test]
fn a_request_the_server_cannot_read_gets_a_reply_and_the_next_call_one_too() {
    let scratch = harness::scratch("mcp-unreadable");
    let base = scratch.0.clone();
    seed(&base);
    let mut mcp = Mcp::start(&base, &[]);
    let term = json!({ "field": "rarity", "op": "=", "value": "rare" });
    let nots = (0..123).fold(term.clone(), |inner, _| json!({ "not": inner }));
    let groups = (0..62).fold(term, |inner, _| json!({ "all": [inner] }));
    let deep = (0..128).fold(json!(1), |inner, _| json!([inner]));
    for (n, arguments) in [
        json!({ "query": { "tree": nots } }),
        json!({ "query": { "tree": groups } }),
        json!({ "query": { "text": "rarity=rare" }, "deep": deep }),
    ]
    .into_iter()
    .enumerate()
    {
        let id = json!(format!("deep-{n}"));
        let line = json!({ "jsonrpc": "2.0", "id": id, "method": "tools/call",
            "params": { "name": "search", "arguments": arguments } });
        let reply = mcp.raw(&line.to_string(), &id);
        let doc = &harness::document(&reply["result"]);
        assert_eq!(reply["result"]["isError"], json!(true), "{reply}");
        assert_eq!(doc["kind"], json!("arguments"), "{reply}");
        assert!(
            doc["error"].as_str().unwrap().contains("127 levels"),
            "{reply}"
        );
        // the next call is answered, its own
        let next = mcp.result("sync_policy", json!({}));
        assert!(harness::document(&next)["policy"].is_object(), "{next}");
    }
    let reply = mcp.raw(r#"{"jsonrpc":"2.0","id":99,"method":"#, &Value::Null);
    assert_eq!(reply["error"]["code"], json!(-32700), "{reply}");
    let next = mcp.result("sync_policy", json!({}));
    assert!(harness::document(&next)["policy"].is_object(), "{next}");
}

/// A line `serde_json` refuses never reaches `rmcp`, which would drop it:
/// a deep line cut short is a parse error with a null id, even with its
/// id and method read first, and a line behind a UTF-8 byte order mark
/// is read as `rmcp` reads it (the 0c review's inputs).
#[test]
fn a_deep_line_cut_short_and_a_line_behind_a_byte_order_mark_each_get_their_reply() {
    let scratch = harness::scratch("mcp-cut-short");
    let base = scratch.0.clone();
    seed(&base);
    let mut mcp = Mcp::start(&base, &[]);
    for line in [
        format!(r#"{{"params":{}"#, "[".repeat(128)),
        format!(
            r#"{{"jsonrpc":"2.0","id":"cut","method":"tools/call","params":{}"#,
            "[".repeat(128)
        ),
    ] {
        let reply = mcp.raw(&line, &Value::Null);
        assert_eq!(reply["error"]["code"], json!(-32700), "{reply}");
    }
    let id = json!("bom");
    let line = json!({ "jsonrpc": "2.0", "id": id, "method": "tools/call",
        "params": { "name": "sync_policy", "arguments": {} } });
    let reply = mcp.raw(&format!("\u{feff}{line}"), &id);
    assert!(
        harness::document(&reply["result"])["policy"].is_object(),
        "{reply}"
    );
}

/// The server's writer is awaited however the service ends: a line
/// answered before initialisation, then stdin closed, still puts its
/// parse error on stdout. A server that never exits on stdin closing
/// fails the test within the harness's deadline rather than hanging it.
#[test]
fn a_reply_queued_before_initialisation_reaches_stdout_when_stdin_closes() {
    let scratch = harness::scratch("mcp-early-eof");
    let base = scratch.0.clone();
    let stdout = harness::run_to_exit(&base, b"not JSON\n");
    let reply: Value = serde_json::from_str(stdout.lines().next().unwrap_or_default())
        .unwrap_or_else(|e| panic!("no reply on stdout ({e}): {stdout:?}"));
    assert_eq!(reply["id"], Value::Null, "{reply}");
    assert_eq!(reply["error"]["code"], json!(-32700), "{reply}");
}

/// Two accounts known — Alice, whose facts hold one tab `tx` with one
/// item `ix` in xbox's Allflame, and Bob, with none — so a request
/// naming no account is ambiguous and every offer must name one.
fn seed_two(base: &std::path::Path) {
    let mock = base.join("store").join("mock");
    std::fs::create_dir_all(&mock).unwrap();
    let at = acquisition_store::now();
    for (name, uuid) in [(USER, UUID), ("Bob#5678", "u-bob")] {
        Index::load(&mock)
            .unwrap()
            .record_login(name, uuid, false, at, &[])
            .unwrap();
        Store::open(&account_path(&mock, name))
            .unwrap()
            .record(
                &Endpoint::Profile,
                &json!({}),
                200,
                &json!({ "uuid": uuid, "name": name }),
                at,
            )
            .unwrap();
    }
    let mut store = Store::open(&account_path(&mock, USER)).unwrap();
    store
        .record(
            &Endpoint::Stashes {
                realm: "xbox".into(),
                league: "Allflame".into(),
            },
            &json!({ "league": "Allflame" }),
            200,
            &json!({ "stashes": [ { "id": "tx", "name": "Box", "type": "PremiumStash", "index": 0 } ] }),
            at,
        )
        .unwrap();
    store
        .record(
            &Endpoint::Stash {
                realm: "xbox".into(),
                league: "Allflame".into(),
                id: "tx".into(),
                sub: None,
            },
            &json!({}),
            200,
            &json!({ "stash": { "id": "tx", "name": "Box", "type": "PremiumStash", "items": [
                { "id": "ix", "name": "", "typeLine": "Chaos Orb", "baseType": "Chaos Orb",
                  "x": 0, "y": 0, "stackSize": 5 } ] } }),
            at,
        )
        .unwrap();
}

/// Round 4 of the 2g build (the refusal shape; the cold reviews' findings
/// 8 and 6): a refusal's offers are requests an agent sends back as
/// printed, and each is answered. An account typed wrong offers the same
/// call as each name known, on a store tool and on `search` (whose
/// account is `scope.account`); a target the facts do not hold offers the
/// tools that would find it, named and with the scope it was looked for
/// in, whether the request named the non-default realm and league or the
/// verb took them from the address and the facts.
#[test]
fn every_offer_a_refusal_makes_is_answered_as_sent_back() {
    let scratch = harness::scratch("mcp-offers");
    let base = scratch.0.clone();
    seed_two(&base);
    let mut mcp = Mcp::start(&base, &[]);
    let answered = |mcp: &mut Mcp, refused: &str, offer: &Value| -> Value {
        let tool = offer["tool"].as_str().unwrap_or(refused).to_string();
        let result = mcp.result(&tool, offer["request"].clone());
        assert_ne!(result["isError"], json!(true), "{offer}: {result}");
        harness::document(&result)
    };

    // a typo'd account: the same call as each name, the kind kept (C121)
    let asked = json!({ "account": "Alicia#1234", "realm": "xbox", "league": "Allflame" });
    let doc = refusal(&mut mcp, "tabs", asked.clone(), "not_logged_in");
    assert_eq!(doc["accounts"], json!([USER, "Bob#5678"]), "{doc}");
    let offers = doc["offers"].as_array().unwrap();
    assert_eq!(offers.len(), 2, "{doc}");
    for (offer, name) in offers.iter().zip([USER, "Bob#5678"]) {
        let mut sent = asked.clone();
        sent["account"] = json!(name);
        assert_eq!(offer["request"], sent, "{doc}");
        assert!(offer.get("tool").is_none(), "the same tool: {offer}");
        answered(&mut mcp, "tabs", offer);
    }
    let shown = answered(&mut mcp, "tabs", &offers[0]);
    assert_eq!(shown["rows"][0]["id"], "tx", "{shown}");
    let doc = refusal(
        &mut mcp,
        "search",
        json!({ "scope": { "account": "Alicia" }, "query": { "text": "class:currency" } }),
        "not_logged_in",
    );
    let offer = &doc["offers"][0];
    assert_eq!(offer["request"]["scope"]["account"], USER, "{doc}");
    assert_eq!(offer["request"]["query"]["text"], "class:currency", "{doc}");
    let found = answered(&mut mcp, "search", offer);
    assert_eq!(found["rows"][0]["id"], "ix", "{found}");

    // a target held nowhere, its realm and league named: the search of
    // the realm and the league's tabs, each answering in that scope
    let doc = refusal(
        &mut mcp,
        "price_show",
        json!({ "target": "tab/xbox/nope", "account": USER, "realm": "xbox", "league": "Allflame" }),
        "not_found",
    );
    let offers = doc["offers"].as_array().unwrap();
    let tools: Vec<&Value> = offers.iter().map(|o| &o["tool"]).collect();
    assert_eq!(tools, [&json!("search"), &json!("tabs")], "{doc}");
    assert_eq!(
        offers[1]["request"],
        json!({ "account": USER, "realm": "xbox", "league": "Allflame" })
    );
    let found = answered(&mut mcp, "price_show", &offers[0]);
    assert_eq!(found["rows"][0]["id"], "ix", "{found}");
    let tabs = answered(&mut mcp, "price_show", &offers[1]);
    assert_eq!(tabs["rows"][0]["id"], "tx", "{tabs}");

    // an item looked for in the realm and league the request named, and
    // in those the verb took when it named neither (pc, the facts'
    // Allflame): its show where the facts hold it, which answers
    for asked in [
        json!({ "target": "item/ix", "account": USER, "realm": "pc", "league": "Allflame" }),
        json!({ "target": "item/ix", "account": USER }),
    ] {
        let doc = refusal(&mut mcp, "price_show", asked.clone(), "not_found");
        let offers = doc["offers"].as_array().unwrap();
        assert_eq!(offers.len(), 1, "{asked}: {doc}");
        assert_eq!(
            offers[0]["request"],
            json!({ "target": "item/ix", "account": USER, "realm": "xbox", "league": "Allflame" }),
            "{asked}"
        );
        assert!(offers[0].get("tool").is_none(), "{doc}");
        let shown = answered(&mut mcp, "price_show", &offers[0]);
        assert_eq!(shown["listing"]["subject"]["target"]["id"], "ix", "{shown}");
    }
}

/// Round 4's review (P2): the account offer on `search` re-made the
/// request alone, so a route sent with its `count` came back without it
/// and the retry lost the comparison a route reports. The offer is the
/// arguments as sent, the account inside the request alone changed: a
/// route of `count` 12 over one item, retried, still says what it was.
#[test]
fn a_search_offered_again_as_another_account_keeps_its_route() {
    let scratch = harness::scratch("mcp-offer-route");
    let base = scratch.0.clone();
    seed_two(&base);
    let mut mcp = Mcp::start(&base, &[]);
    let sent = json!({
        "request": { "scope": { "account": "Alicia", "realm": "xbox" }, "query": { "text": "" } },
        "count": 12,
    });
    let doc = refusal(&mut mcp, "search", sent.clone(), "not_logged_in");
    let offer = &doc["offers"][0];
    let mut expected = sent.clone();
    expected["request"]["scope"]["account"] = json!(USER);
    assert_eq!(offer["request"], expected, "{doc}");
    let result = mcp.result("search", offer["request"].clone());
    assert_ne!(result["isError"], json!(true), "{result}");
    let answer = &harness::document(&result);
    assert_eq!(answer["total"]["matched"], 1, "{answer}");
    assert_eq!(answer["before"]["count"], 12, "{answer}");
}

/// `account`'s facts in `base` gain xbox Allflame's tab `t2` with two
/// items: what a continuation of one row needs, and Bob's store a store
/// that holds items.
fn two_items(base: &std::path::Path, account: &str, prefix: &str) {
    let mock = base.join("store").join("mock");
    let at = acquisition_store::now();
    let mut store = Store::open(&account_path(&mock, account)).unwrap();
    store
        .record(
            &Endpoint::Stashes {
                realm: "xbox".into(),
                league: "Allflame".into(),
            },
            &json!({ "league": "Allflame" }),
            200,
            &json!({ "stashes": [
                { "id": "tx", "name": "Box", "type": "PremiumStash", "index": 0 },
                { "id": "t2", "name": "Two", "type": "PremiumStash", "index": 1 } ] }),
            at,
        )
        .unwrap();
    let item = |id: String, x: i64| {
        json!({ "id": id, "name": "", "typeLine": "Chaos Orb", "baseType": "Chaos Orb",
                "x": x, "y": 0, "stackSize": 5 })
    };
    store
        .record(
            &Endpoint::Stash {
                realm: "xbox".into(),
                league: "Allflame".into(),
                id: "t2".into(),
                sub: None,
            },
            &json!({}),
            200,
            &json!({ "stash": { "id": "t2", "name": "Two", "type": "PremiumStash", "items": [
                item(format!("{prefix}1"), 0), item(format!("{prefix}2"), 1) ] } }),
            at,
        )
        .unwrap();
}

/// Round 4's second fix round (its fix round's review, finding 2): an
/// account offer for a continuation kept its place, another store's
/// basis, so Bob's was refused as `basis_changed`. A repeat as another
/// account drops the place and says it is the request from its start;
/// with both stores holding items, every offer is answered.
#[test]
fn a_continuation_offered_as_another_account_is_its_request_from_its_start() {
    let scratch = harness::scratch("mcp-offer-next");
    let base = scratch.0.clone();
    seed_two(&base);
    two_items(&base, USER, "ia");
    two_items(&base, "Bob#5678", "ib");
    let mut mcp = Mcp::start(&base, &[]);
    let first = mcp.result(
        "search",
        json!({ "scope": { "account": USER, "realm": "xbox" }, "view": { "rows": { "limit": 1 } } }),
    );
    let mut next = harness::document(&first)["view"]["rows"]["next"].clone();
    assert!(next.is_object(), "no continuation: {first}");
    next["request"]["scope"]["account"] = json!("Alicia");
    let doc = refusal(&mut mcp, "search", next, "not_logged_in");
    let offers = doc["offers"].as_array().unwrap();
    assert_eq!(offers.len(), 2, "{doc}");
    for (offer, name) in offers.iter().zip([USER, "Bob#5678"]) {
        assert_eq!(
            offer["says"],
            format!("the same request from its start, as {name}"),
            "{doc}"
        );
        let request = &offer["request"]["request"];
        assert!(request.get("basis").is_none(), "{offer}");
        assert!(request["view"]["rows"].get("after").is_none(), "{offer}");
        let result = mcp.result("search", offer["request"].clone());
        assert_ne!(result["isError"], json!(true), "{offer}: {result}");
        let answer = &harness::document(&result);
        assert_eq!(answer["scope"]["account"]["name"], name, "{answer}");
        assert_eq!(answer["view"]["rows"]["returned"], 1, "{answer}");
    }
}
