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
    let at = harness::now();
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
    // one body the store refused as malformed: an item with no id
    let at = harness::now();
    // a string the spelling would rewrite, were the body prose
    let nameless = json!({ "hint": "--limit", "stash": { "id": "t1", "name": "One", "type": "PremiumStash",
        "items": [{ "typeLine": "Iron Ring", "baseType": "Iron Ring", "x": 0, "y": 0 }] } });
    Store::open(&account_path(&base.join("store").join("mock"), USER))
        .unwrap()
        .record(
            &Endpoint::Stash {
                realm: "pc".into(),
                league: "Standard".into(),
                id: "t1".into(),
                sub: None,
            },
            &json!({ "league": "Standard" }),
            200,
            &nameless,
            at,
        )
        .unwrap_err();
    let mut mcp = Mcp::start(&base, &[]);

    // `store_refused` lists it beside its `total`, reads it whole by its
    // row id, and refuses an id the store holds none under
    let listed = harness::document(&mcp.result("store_refused", json!({})));
    assert_eq!(
        (&listed["returned"], &listed["limit"], &listed["total"]),
        (&json!(1), &json!(10), &json!(1)),
        "{listed}"
    );
    assert!(listed["rows"][0]["body"].is_null(), "{listed}");
    let id = listed["rows"][0]["id"].clone();
    let one = harness::document(&mcp.result("store_refused", json!({ "id": id })));
    assert_eq!(
        one["body"], nameless,
        "evidence passes through verbatim: {one}"
    );
    assert!(one["fetched_at_age"].is_string(), "{one}");
    let doc = refusal(
        &mut mcp,
        "store_refused",
        json!({ "id": 9999 }),
        "refused_body_not_found",
    );
    // the request's words, its way on the list as the account (C143)
    assert_eq!(
        (&doc["error"], &doc["offers"]),
        (
            &json!("no refused body with id 9999; their list names each by its id"),
            &json!([{
                "request": { "account": USER },
                "says": "the refused bodies, each with its id",
            }])
        ),
        "{doc}"
    );

    // An account no login made, on every tool that selects one: the
    // protocol's word for it (C121 as amended; #168, MCP call 56), the
    // known names as accounts.
    for (tool, mut args) in [
        ("tabs", json!({})),
        ("characters", json!({})),
        ("store_status", json!({})),
        ("store_refused", json!({})),
        ("item_events", json!({})),
        ("sync_policy", json!({})),
        (
            "set_sync_policy",
            json!({ "value": { "version": 3, "realms": {} } }),
        ),
        ("refresh_plan", json!({})),
    ] {
        args["account"] = json!("GERWARIK");
        let doc = refusal(&mut mcp, tool, args, "unknown_account");
        assert_eq!(doc["accounts"], json!([USER]), "{tool}: {doc}");
    }

    // A league the policy does not cover: refused in the planner's words,
    // which name the leagues it covers, and the plan of each offered (the
    // league seat, mcp call 9's sibling of cli call 26), sent back as
    // printed to the same tool.
    let doc = refusal(
        &mut mcp,
        "refresh_plan",
        json!({ "league": "Allflame" }),
        "refused",
    );
    assert!(
        doc["error"].as_str().unwrap().starts_with(
            "the sync policy covers Standard on realm pc, not Allflame: `league` names one"
        ),
        "{doc}"
    );
    assert_eq!(
        doc["offers"],
        json!([{
            "request": { "account": USER, "realm": "pc", "league": "Standard" },
            "says": "the plan of pc Standard, which the policy covers",
        }]),
        "{doc}"
    );
    let planned = mcp.expect_ok("refresh_plan", doc["offers"][0]["request"].clone());
    assert_eq!(planned["plan"]["league"], json!("Standard"), "{planned}");
    // eleven covered (the review of the fix round, finding 2): ten named
    // and offered, the rest counted, and the tool that lists them all
    let leagues: serde_json::Map<String, Value> = (1..=11)
        .map(|n| {
            (
                format!("L{n:02}"),
                json!({ "tabs": "all", "max_age_seconds": 3600 }),
            )
        })
        .collect();
    mcp.expect_ok(
        "set_sync_policy",
        json!({ "value": { "version": 3, "realms": { "pc": { "leagues": leagues } } },
                "if_revision": 1 }),
    );
    let doc = refusal(
        &mut mcp,
        "refresh_plan",
        json!({ "league": "Allflame" }),
        "refused",
    );
    assert!(
        doc["error"].as_str().unwrap().starts_with(
            "the sync policy covers L01, L02, L03, L04, L05, L06, L07, L08, L09, L10 and 1 more \
             on realm pc, not Allflame: `league` names one; sync_policy with `account` \
             Alice#1234 lists them all"
        ),
        "{doc}"
    );
    assert_eq!(doc["offers"].as_array().map(Vec::len), Some(10), "{doc}");

    // A parameter the tool cannot read.
    refusal(&mut mcp, "tabs", json!({ "realm": "pcx" }), "arguments");
    refusal(
        &mut mcp,
        "item_events",
        json!({ "hours": "a day" }),
        "arguments",
    );

    // The schema advertises `null` for every optional argument (the trim
    // of #104 was declined), so an explicit `null` is an absent one: the
    // same answer, a store tool's refusal included, never `arguments`.
    assert_eq!(
        mcp.expect_ok("list_jobs", json!({ "expand": null })),
        mcp.expect_ok("list_jobs", json!({})),
    );
    let nulled = json!({ "league": null, "limit": null, "expand": null, "type": null,
        "never_fetched": null, "realm": null, "account": null });
    let (with, without) = (mcp.call("tabs", nulled), mcp.call("tabs", json!({})));
    assert_eq!(with, without);
    assert!(
        !format!("{with:?}").contains("not arguments the tool takes"),
        "{with:?}"
    );

    // A daemon's own refusal is still a JSON-RPC error with its kind.
    let _daemon = spawn_daemon(&base, &[]);
    let deadline = Instant::now() + Duration::from_secs(120);
    while mcp.expect_ok("daemon_status", json!({}))["running"] != json!(true) {
        assert!(Instant::now() < deadline, "the mock daemon did not come up");
        std::thread::sleep(Duration::from_millis(50));
    }
    let error = mcp.expect_rpc_error("job_status", json!({ "id": 999 }));
    assert_eq!(error["data"]["kind"], "unknown_job", "{error}");
    assert_eq!(error["message"], "no job 999", "{error}");
    // the daemon tells the account kinds apart alike (C121 as amended): a
    // name no login made, and an account the index holds with no session
    for (account, kind) in [("GERWARIK", "unknown_account"), (USER, "not_logged_in")] {
        let error = mcp.expect_rpc_error(
            "submit_job",
            json!({ "kind": "profile", "account": account }),
        );
        assert_eq!(error["data"]["kind"], kind, "{account}: {error}");
    }
}

/// C127 (one list of the kinds a client may submit): `submit_job`'s
/// `kind` names the kinds the daemon's own refusal of an unknown kind
/// lists, word for word, so the description cannot name a kind the door
/// refuses (`refresh`, retired) or leave out one it admits (`apply`), as
/// it once did both.
/// Every tool refuses an argument it does not take, as `arguments`, the
/// key named: walked over `tools/list`, so a tool added without it fails
/// here and not in a seat (the review of #18's build at c1af7364, P2:
/// `store_refused` read `{"acount": …}` as no account named).
#[test]
fn every_tool_refuses_an_argument_it_does_not_take() {
    let scratch = harness::scratch("mcp-unknown-arg");
    let base = scratch.0.clone();
    seed(&base);
    let mut mcp = Mcp::start(&base, &[]);
    let tools = mcp.list_tools();
    assert!(tools.len() >= 27, "tools/list returned {}", tools.len());
    for tool in tools {
        let name = tool["name"].as_str().unwrap().to_string();
        // `search` reads its request with the search crate's own reader
        // (H1), which refuses a key it does not know as a tree
        let kind = if name == "search" {
            "tree"
        } else {
            "arguments"
        };
        let doc = refusal(&mut mcp, &name, json!({ "acount": USER }), kind);
        assert!(
            doc["error"].as_str().unwrap().contains("acount"),
            "{name} names the key it refused: {doc}"
        );
    }
}

#[test]
fn submit_jobs_kind_names_the_kinds_the_daemons_refusal_lists() {
    let scratch = harness::scratch("mcp-kinds");
    let base = scratch.0.clone();
    let mut mcp = Mcp::start(&base, &[]);
    let _daemon = spawn_daemon(&base, &[]);
    let deadline = Instant::now() + Duration::from_secs(120);
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

/// C164 (#166; the answer-facts seat, MCP run 1 calls 57 to 60: a stash
/// job for `rh-nope`, a tab the stored listing never named, was admitted
/// and its 404 spent a request): with the mock's pc Standard listing
/// stored, `submit_job` of a stash for a tab it does not name is refused
/// at submit, kind `refused` in the daemon's data with a message (its
/// words are the CLI's golden `unlisted_tab`'s), and no job is admitted;
/// a tab it names is admitted and fetched.
#[test]
fn c164_a_stash_job_for_a_tab_the_stored_listing_does_not_name_is_refused_at_submit() {
    let scratch = harness::scratch("mcp-unlisted");
    let base = scratch.0.clone();
    let (mut daemon, _pid) = harness::start_daemon_with(&base, &[]);
    harness::login_as_mock(&base);
    let mut mcp = Mcp::start(&base, &[]);

    // C165 (#165, MCP run 1 calls 59 and 60): with no listing stored the
    // stash job is admitted and GGG's 404 fails it; its result says the
    // kind, `not_found`, and offers the league's listing as submit_job's
    // own arguments, sent back as printed
    let lost = mcp.expect_ok(
        "submit_job",
        json!({ "kind": "stash", "params": { "league": "Standard", "id": "no-such-tab" } }),
    );
    let deadline = Instant::now() + Duration::from_secs(60);
    while !matches!(
        mcp.expect_ok("job_status", json!({ "id": lost["job_id"] }))["state"].as_str(),
        Some("failed")
    ) {
        assert!(Instant::now() < deadline, "the 404 never failed its job");
        std::thread::sleep(Duration::from_millis(100));
    }
    let failed = mcp.expect_ok("job_result", json!({ "id": lost["job_id"] }));
    assert_eq!(failed["kind"], "not_found", "{failed}");
    let offers = failed["offers"].as_array().cloned().unwrap_or_default();
    assert_eq!(offers.len(), 1, "{failed}");
    assert_eq!(offers[0]["tool"], "submit_job", "{failed}");
    assert_eq!(offers[0]["request"]["kind"], "stashes", "{failed}");
    let listing = mcp.expect_ok("submit_job", offers[0]["request"].clone());
    let outcome = harness::finish_job(&mut mcp, &listing["job_id"]);
    assert_eq!(outcome["outcome"], "success", "{outcome}");
    let named = outcome["payload"]["stashes"][0]["id"].clone();
    assert!(named.is_string(), "{outcome}");

    let error = mcp.expect_rpc_error(
        "submit_job",
        json!({ "kind": "stash", "params": { "league": "Standard", "id": "nope" } }),
    );
    assert_eq!(error["data"]["kind"], "refused", "{error}");
    // the words are held once, by the CLI's golden (C153; the review of
    // round D, finding 4): here, that the refusal says something
    assert!(
        error["message"].as_str().is_some_and(|m| !m.is_empty()),
        "{error}"
    );
    let jobs = mcp.expect_ok("list_jobs", json!({ "expand": true }));
    let rows = jobs["rows"].as_array().cloned().unwrap_or_default();
    assert!(
        rows.iter().all(|job| job["params"]["id"] != json!("nope")),
        "the refused stash job was admitted: {jobs}"
    );

    let fetched = mcp.expect_ok(
        "submit_job",
        json!({ "kind": "stash", "params": { "league": "Standard", "id": named } }),
    );
    let outcome = harness::finish_job(&mut mcp, &fetched["job_id"]);
    assert_eq!(outcome["outcome"], "success", "{outcome}");
    // C165, live: the result and the status say when, each time beside
    // its words, a finish no earlier than the start
    let status = mcp.expect_ok("job_status", json!({ "id": fetched["job_id"] }));
    for doc in [&outcome, &status] {
        for time in ["submitted_at", "started_at", "finished_at"] {
            assert!(doc[time].is_i64(), "{time}: {doc}");
            assert!(doc[format!("{time}_age")].is_string(), "{time}: {doc}");
        }
        assert!(
            doc["finished_at"].as_i64() >= doc["started_at"].as_i64(),
            "{doc}"
        );
    }
    let listed = mcp.expect_ok("list_jobs", json!({}));
    let row = listed["rows"]
        .as_array()
        .and_then(|rows| rows.iter().find(|r| r["id"] == fetched["job_id"]))
        .cloned()
        .unwrap_or_else(|| panic!("{listed}"));
    assert!(row["finished_at_age"].is_string(), "{row}");

    drop(mcp);
    let stop = harness::acq(&base, &["daemon", "stop"]);
    assert!(stop.status.success(), "{stop:?}");
    daemon.wait_exit(Duration::from_secs(120));
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
    let at = harness::now();
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
/// verb took them from the address and the facts. A tab's id shown offers
/// the search of what is in it, said in the request's words and never a
/// terminal's command (#97).
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

    // a typo'd account: the same call as each name, its own kind (C121)
    let asked = json!({ "account": "Alicia#1234", "realm": "xbox", "league": "Allflame" });
    let doc = refusal(&mut mcp, "tabs", asked.clone(), "unknown_account");
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
        "unknown_account",
    );
    let offer = &doc["offers"][0];
    assert_eq!(offer["request"]["scope"]["account"], USER, "{doc}");
    assert_eq!(offer["request"]["query"]["text"], "class:currency", "{doc}");
    let found = answered(&mut mcp, "search", offer);
    assert_eq!(found["rows"][0]["id"], "ix", "{found}");

    // a tab's id: the search of what is in it, the `search` tool's
    let doc = refusal(
        &mut mcp,
        "show",
        json!({ "id": "tx", "account": USER }),
        "not_an_item",
    );
    assert!(!doc.to_string().contains("acq "), "{doc}");
    let offers = doc["offers"].as_array().unwrap();
    assert_eq!(offers.len(), 1, "{doc}");
    assert_eq!(offers[0]["tool"], "search", "{doc}");
    assert_eq!(offers[0]["says"], "the search of what is in the tab `Box`");
    let found = answered(&mut mcp, "show", &offers[0]);
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
    // in the realm the verb took when it named the league alone (pc): its
    // show where the facts hold it, which answers; named neither, the
    // verb reads where the facts hold it (the review of round B's fix
    // round)
    let bare = answered(
        &mut mcp,
        "price_show",
        &json!({ "request": { "target": "item/ix", "account": USER } }),
    );
    assert_eq!(
        (&bare["realm"], &bare["league"]),
        (&json!("xbox"), &json!("Allflame")),
        "{bare}"
    );
    for asked in [
        json!({ "target": "item/ix", "account": USER, "realm": "pc", "league": "Allflame" }),
        json!({ "target": "item/ix", "account": USER, "league": "Allflame" }),
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
    let doc = refusal(&mut mcp, "search", sent.clone(), "unknown_account");
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
    let at = harness::now();
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
/// with both stores holding items, every offer is answered. And the same
/// continuation across its own store's change is `basis_changed`,
/// offering the request from its start in the request's words, never a
/// terminal's command (#97).
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
    let doc = refusal(&mut mcp, "search", next.clone(), "unknown_account");
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

    two_items(&base, USER, "ic");
    next["request"]["scope"]["account"] = json!(USER);
    let doc = refusal(&mut mcp, "search", next, "basis_changed");
    assert!(!doc.to_string().contains("acq "), "{doc}");
    let offers = doc["offers"].as_array().unwrap();
    assert_eq!(offers.len(), 1, "{doc}");
    assert_eq!(offers[0]["says"], "the same request from its start");
    assert!(offers[0].get("tool").is_none(), "the same tool: {doc}");
    let result = mcp.result("search", offers[0]["request"].clone());
    assert_ne!(result["isError"], json!(true), "{result}");
    let answer = &harness::document(&result);
    assert_eq!(answer["view"]["rows"]["returned"], 1, "{answer}");
}

/// The review of #97's Part A, its finding P2: the rule every refusal is
/// held to (C143, the harness's `c143_a_text_speaks_no_frontend`)
/// reads an offer's `says` and a refusal's `next`, which are words,
/// while each offer's `request` stays payload, sent back as printed; a
/// person's act is a command wherever it stands.
#[test]
fn the_refusal_rule_reads_an_offers_says_and_the_next_step() {
    let found = |doc: Value| harness::frontend_speech(&doc);
    let offer = |says: &str| {
        json!({ "error": "refused", "kind": "not_found", "offers": [{
            "request": { "query": { "text": "name:\"acq search --count\"" } },
            "says": says,
            "tool": "search",
        }] })
    };
    assert_eq!(
        found(offer("acq price show item/x reads it")).as_deref(),
        Some("acq price show item/x reads it"),
        "an offer's says"
    );
    assert_eq!(
        found(
            json!({ "error": "no daemon", "kind": "no_daemon", "next": "acq search --realm pc" })
        )
        .as_deref(),
        Some("acq search --realm pc"),
        "a refusal's next"
    );
    assert_eq!(
        found(offer("every item of pc")),
        None,
        "a request is payload"
    );
    assert_eq!(
        found(json!({ "error": "no daemon", "kind": "no_daemon",
                      "next": "ACQ_STORE_DIR=/s acq auth check" })),
        None,
        "a person's act"
    );
}

/// The review of #97's fix round and Part B, its finding P2: an
/// answer's generated prose is read wherever it stands — a receipt's
/// `note`, `no_undo` and `clear.says`, an unavailable count's
/// `reads[].says` — since the rule reads every string but named payload
/// and stored data, so a field added later is read by default.
#[test]
fn the_text_rule_reads_every_string_an_answer_generates() {
    let found = |doc: Value| harness::frontend_speech(&doc);
    let unread = [
        (
            "note",
            json!({ "write": {}, "note": "acq price show item/i-map" }),
        ),
        (
            "no_undo",
            json!({ "write": {}, "no_undo": "--if-revision 2 restores it" }),
        ),
        (
            "clear.says",
            json!({ "clear": { "request": {}, "says": "acq price clear item/i-map" } }),
        ),
        (
            "counts_unavailable.reads[].says",
            json!({ "counts_unavailable": { "reads": [{ "says": "acq price status --expand" }] } }),
        ),
    ]
    .into_iter()
    .filter(|(_, doc)| found(doc.clone()).is_none())
    .map(|(key, _)| key)
    .collect::<Vec<_>>();
    assert!(unread.is_empty(), "unread: {unread:?}");
    // data stays data: a stored row's actor and a request sent back
    assert_eq!(
        found(json!({ "current": { "actor": "acq search --count tab" },
                      "undo": { "request": "acq price clear item/x", "says": "the clear" } })),
        None
    );
}
