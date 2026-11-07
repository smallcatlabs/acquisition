//! The saved search over MCP (4-full part 1, step 2): `saved_searches`,
//! `saved_search_set` and `saved_search_clear` are adapters of the verbs'
//! `saved` functions, as `acq saved` is, and `search` takes `saved` as
//! `acq search --saved` does (C46), so what a tool returns is the
//! document the CLI's `--json` prints for the same request, a refusal
//! included, in a result marked `isError`. What the adapter adds is
//! pinned here, in `pricing_tools.rs`' shape: the CLI beside the server
//! over one world each, its helpers reused; the stricter rule (a write
//! naming no revision is a create, C52); `search`'s beside-rule; and
//! every way a receipt or a refusal offers, sent back as printed. The
//! facts are `search_tools.rs`' and `spoken.rs`' seeds; the saved
//! searches are the world's (C167), written through the two surfaces.
//! No daemon runs, and none appears. What the verbs' `documents.rs`
//! pins is not pinned again: the merge at a revision, the mint, the
//! eleven-row bound, the unreadable row.

use crate::harness;
use crate::pricing_tools::{RECEIPT, cli, offers_alike, steady, tool, undo_alike};
use crate::{search_tools, spoken};

use std::path::Path;

use harness::{Mcp, assert_spoken_as, no_daemon_appeared, scratch};
use serde_json::{Value, json};

/// The id a write names, as a receipt holds it.
fn id_of(receipt: &Value) -> String {
    receipt["target"]["id"]
        .as_str()
        .unwrap_or_else(|| panic!("a receipt with no id: {receipt}"))
        .to_string()
}

/// `doc` with each id of `ids` replaced by one placeholder: two worlds
/// mint their ids at random, so two receipts of one create differ there
/// by design.
fn masked(doc: &Value, ids: &[&str]) -> Value {
    let mut text = doc.to_string();
    for id in ids {
        text = text.replace(id, "MINTED");
    }
    serde_json::from_str(&text).unwrap()
}

/// A create's receipt: its minted id, checked to be the same at every
/// place it stands (`target.id`, `written.id`, the undo's clear, a tool's
/// arguments or a command line).
fn minted(receipt: &Value) -> String {
    let id = id_of(receipt);
    assert_eq!(receipt["written"]["id"], json!(id), "{receipt}");
    let undo = &receipt["undo"]["request"];
    assert!(
        undo.get("id") == Some(&json!(id)) || undo.as_str().is_some_and(|l| l.contains(&id)),
        "the undo names the id written: {receipt}"
    );
    id
}

/// A tool's answer, which must not be a refusal.
fn answered(mcp: &mut Mcp, name: &str, args: Value) -> Value {
    let (ok, doc) = tool(mcp, name, args.clone());
    assert!(ok, "{name} {args} was refused: {doc}");
    doc
}

/// A tool's refusal of `kind`.
fn refused(mcp: &mut Mcp, name: &str, args: Value, kind: &str) -> Value {
    let (ok, doc) = tool(mcp, name, args.clone());
    assert!(!ok, "{name} {args} answered: {doc}");
    assert_eq!(doc["kind"], json!(kind), "{name} {args}: {doc}");
    doc
}

/// An offer sent back as printed: its request to the tool it names, or
/// to `called`, the tool that made it.
fn sent(mcp: &mut Mcp, called: &str, offer: &Value) -> (bool, Value) {
    let to = offer["tool"].as_str().unwrap_or(called).to_string();
    assert_ne!(offer.get("tool"), Some(&json!(called)), "{offer}");
    tool(mcp, &to, offer["request"].clone())
}

/// The CLI's document, which must not be a refusal.
fn printed(base: &Path, args: &[&str]) -> Value {
    let (ok, doc) = cli(base, args);
    assert!(ok, "acq {args:?} was refused: {doc}");
    doc
}

/// C46 — `saved_searches` bare, with `limit` and with `expand` over more
/// than ten rows, by id and by name, is the document `acq saved list` or
/// `acq saved show` prints; `search` with `saved`, by id and by name and
/// with a view, is `acq search --saved`'s, and `search` with the row's
/// text and scope.
#[test]
fn each_saved_read_is_the_document_the_cli_prints() {
    let dir = scratch("mcp-saved-read");
    let base = &dir.0;
    search_tools::seed(base);
    for _ in 0..11 {
        printed(base, &["saved", "set", "--name", "dup", "rarity=rare"]);
    }
    let rings = id_of(&printed(
        base,
        &["saved", "set", "--name", "rings", "class:ring"],
    ));
    let mut mcp = Mcp::start(base, &[]);
    for (args, verb) in [
        (json!({}), &["saved", "list"][..]),
        (
            json!({ "limit": 3 }),
            &["saved", "list", "--limit", "3"][..],
        ),
        (
            json!({ "expand": true }),
            &["saved", "list", "--expand"][..],
        ),
        (
            json!({ "saved": rings }),
            &["saved", "show", rings.as_str()][..],
        ),
        (json!({ "saved": "rings" }), &["saved", "show", "rings"][..]),
    ] {
        let by_tool = answered(&mut mcp, "saved_searches", args.clone());
        assert_eq!(
            by_tool,
            printed(base, verb),
            "saved_searches {args} is not acq {verb:?}"
        );
    }
    let listed = answered(&mut mcp, "saved_searches", json!({}));
    assert_eq!(
        (&listed["total"], &listed["left_out"]),
        (&json!(12), &json!(2)),
        "{listed}"
    );

    let row = answered(&mut mcp, "saved_searches", json!({ "saved": rings }));
    for (args, verb) in [
        (
            json!({ "saved": rings }),
            vec!["search", "--saved", rings.as_str()],
        ),
        (
            json!({ "saved": "rings", "view": { "rows": { "limit": 1 } } }),
            vec!["search", "--saved", "rings", "--limit", "1"],
        ),
    ] {
        let by_tool = answered(&mut mcp, "search", args.clone());
        assert_spoken_as(&by_tool, &printed(base, &verb), &format!("search {args}"));
        let mut plain = json!({ "query": { "text": row["text"] }, "scope": row["scope"] });
        if let Some(view) = args.get("view") {
            plain["view"] = view.clone();
        }
        assert_eq!(
            by_tool,
            answered(&mut mcp, "search", plain.clone()),
            "search {args} is not search {plain}"
        );
    }
    assert!(no_daemon_appeared(base));
}

/// C46 and C52 — each write's receipt is the one `acq saved set` or `acq
/// saved clear` prints for the same write, the channel and the undo
/// compared apart, the undo saying the same: a create, a write by id at a
/// revision, a rename, a create over every realm, a clear, and the
/// conflict of a write at a revision that moved, with the row as it
/// stands. Each world mints its own ids; each is checked where it stands
/// and masked, and every later write uses its own world's.
#[test]
fn each_saved_write_is_the_receipt_the_cli_prints() {
    let (by_mcp, by_acq) = (
        scratch("mcp-saved-write-mcp"),
        scratch("mcp-saved-write-cli"),
    );
    search_tools::seed(&by_mcp.0);
    search_tools::seed(&by_acq.0);
    let mut mcp = Mcp::start(&by_mcp.0, &[]);
    let alike = |by_tool: Value, by_cli: Value, ids: [&str; 2], what: &str| {
        let (by_tool, by_cli) = (masked(&by_tool, &ids), masked(&by_cli, &ids));
        undo_alike(&by_tool, &by_cli, "acq saved ");
        let (by_tool, by_cli) = offers_alike(by_tool, by_cli);
        assert_eq!(
            steady(by_tool, RECEIPT),
            steady(by_cli, RECEIPT),
            "{what} is not what the CLI prints"
        );
    };

    let created = answered(
        &mut mcp,
        "saved_search_set",
        json!({ "name": "rings", "text": "class:ring" }),
    );
    let by_cli = printed(
        &by_acq.0,
        &["saved", "set", "--name", "rings", "class:ring"],
    );
    let (m, c) = (minted(&created), minted(&by_cli));
    assert_eq!(created["undo"]["tool"], json!("saved_search_clear"));
    alike(created, by_cli, [&m, &c], "a create");

    let steps: [(Value, Vec<&str>, &str); 3] = [
        (
            json!({ "id": m, "if_revision": 1, "text": "class:ring rarity=rare" }),
            vec![
                "saved",
                "set",
                "--id",
                c.as_str(),
                "--if-revision",
                "1",
                "class:ring rarity=rare",
            ],
            "a write by id at a revision",
        ),
        (
            json!({ "id": m, "if_revision": 2, "name": "my rings" }),
            vec![
                "saved",
                "set",
                "--id",
                c.as_str(),
                "--if-revision",
                "2",
                "--name",
                "my rings",
            ],
            "a rename",
        ),
        (
            json!({ "id": m, "if_revision": 1, "name": "stale" }),
            vec![
                "saved",
                "set",
                "--id",
                c.as_str(),
                "--if-revision",
                "1",
                "--name",
                "stale",
            ],
            "a write at a revision that moved",
        ),
    ];
    for (args, verb, what) in steps {
        let (tool_ok, by_tool) = tool(&mut mcp, "saved_search_set", args.clone());
        let (cli_ok, by_cli) = cli(&by_acq.0, &verb);
        assert_eq!(tool_ok, cli_ok, "{what}: {by_tool}\n{by_cli}");
        if !tool_ok {
            assert_eq!(by_tool["kind"], json!("conflict"), "{by_tool}");
            assert_eq!(by_tool["revision"], json!(3), "{by_tool}");
            assert_eq!(by_tool["current"]["name"], json!("my rings"), "{by_tool}");
        }
        alike(by_tool, by_cli, [&m, &c], what);
    }

    let everywhere = answered(
        &mut mcp,
        "saved_search_set",
        json!({ "name": "all of it", "text": "rarity=unique", "realm": "all" }),
    );
    let by_cli = printed(
        &by_acq.0,
        &[
            "saved",
            "set",
            "--name",
            "all of it",
            "--realm",
            "all",
            "rarity=unique",
        ],
    );
    let (m2, c2) = (minted(&everywhere), minted(&by_cli));
    assert_eq!(everywhere["written"]["scope"]["realm"], json!("all"));
    alike(everywhere, by_cli, [&m2, &c2], "a create over every realm");

    let cleared = answered(
        &mut mcp,
        "saved_search_clear",
        json!({ "id": m, "if_revision": 3 }),
    );
    let by_cli = printed(
        &by_acq.0,
        &["saved", "clear", c.as_str(), "--if-revision", "3"],
    );
    assert_eq!(cleared["undo"]["tool"], json!("saved_search_set"));
    alike(cleared, by_cli, [&m, &c], "a clear");
    assert!(no_daemon_appeared(&by_mcp.0));
}

/// The refusals the CLI can make for the same request are its documents,
/// each spelled for this server: `saved_search_not_found` on a name and
/// on a cleared id, `ambiguous_saved_search` from `saved_searches` and
/// from `search`, `realm_needed` on a create naming no realm over a store
/// of two, and a word beside `saved` in the kind the CLI's refusal of a
/// flag beside `--saved` carries.
#[test]
fn each_saved_refusal_is_the_one_the_cli_prints() {
    let dir = scratch("mcp-saved-refusals");
    let base = &dir.0;
    search_tools::seed(base);
    for _ in 0..2 {
        printed(base, &["saved", "set", "--name", "dup", "rarity=rare"]);
    }
    let gone = id_of(&printed(
        base,
        &["saved", "set", "--name", "rings", "class:ring"],
    ));
    printed(base, &["saved", "clear", gone.as_str()]);
    let mut mcp = Mcp::start(base, &[]);
    for (name, args, verb, kind) in [
        (
            "saved_searches",
            json!({ "saved": "nope" }),
            vec!["saved", "show", "nope"],
            "saved_search_not_found",
        ),
        (
            "search",
            json!({ "saved": gone }),
            vec!["search", "--saved", gone.as_str()],
            "saved_search_not_found",
        ),
        (
            "saved_searches",
            json!({ "saved": "dup" }),
            vec!["saved", "show", "dup"],
            "ambiguous_saved_search",
        ),
        (
            "search",
            json!({ "saved": "dup" }),
            vec!["search", "--saved", "dup"],
            "ambiguous_saved_search",
        ),
    ] {
        let by_tool = refused(&mut mcp, name, args.clone(), kind);
        let (ok, by_cli) = cli(base, &verb);
        assert!(!ok, "acq {verb:?}: {by_cli}");
        let (by_tool, by_cli) = offers_alike(by_tool, by_cli);
        assert_eq!(by_tool, by_cli, "{name} {args} is not acq {verb:?}");
    }

    // the beside-rule's kind is the CLI's for `--realm` beside `--saved`
    let (_, by_cli) = cli(base, &["search", "--saved", "dup", "--realm", "pc"]);
    let by_tool = refused(
        &mut mcp,
        "search",
        json!({ "saved": "dup", "scope": { "realm": "pc" } }),
        by_cli["kind"].as_str().unwrap(),
    );
    assert_eq!(
        by_tool["error"],
        json!(
            "`saved` runs a saved search whole, its scope and its query: `scope.realm` has \
             nowhere to stand beside it"
        )
    );
    drop(mcp);

    // a create naming no realm over a store of two
    let two = scratch("mcp-saved-realms");
    spoken::seed(&two.0);
    let mut mcp = Mcp::start(&two.0, &[]);
    let by_tool = refused(
        &mut mcp,
        "saved_search_set",
        json!({ "name": "anything", "text": "rarity=rare" }),
        "realm_needed",
    );
    let (ok, by_cli) = cli(
        &two.0,
        &["saved", "set", "--name", "anything", "rarity=rare"],
    );
    assert!(!ok, "{by_cli}");
    let (by_tool, by_cli) = offers_alike(by_tool, by_cli);
    assert_eq!(by_tool, by_cli, "realm_needed is not the CLI's");
    assert!(no_daemon_appeared(base) && no_daemon_appeared(&two.0));
}

/// The refusals the MCP adds (C52, C53, the beside-rule): a set at an id
/// naming no revision over a live row is a create, refused as `conflict`
/// with nothing written; with a name alone, the verbs' `arguments`; a
/// revision with no id, `arguments`, and 0 with none a create; a clear
/// naming no revision, `arguments` naming it, and at 0 `conflict`;
/// `saved` beside `limit` or `expand`; each word of a request beside
/// `saved` on `search`, naming the word; and a key the reader does not
/// know, or reads ill-formed, beside `saved` refused by the reader as it
/// is without `saved`.
#[test]
fn the_mcp_refuses_what_it_does_not_take_naming_it() {
    let dir = scratch("mcp-saved-own");
    let base = &dir.0;
    search_tools::seed(base);
    let mut mcp = Mcp::start(base, &[]);
    let row = answered(
        &mut mcp,
        "saved_search_set",
        json!({ "name": "rings", "text": "class:ring" }),
    );
    let id = id_of(&row);
    let shown = |mcp: &mut Mcp| answered(mcp, "saved_searches", json!({ "saved": id }));
    let before = shown(&mut mcp);

    let blind = refused(
        &mut mcp,
        "saved_search_set",
        json!({ "id": id, "name": "other", "text": "rarity=rare", "realm": "pc",
                "membership": "live" }),
        "conflict",
    );
    assert_eq!(blind["revision"], json!(1), "{blind}");
    assert_eq!(shown(&mut mcp), before, "a blind write wrote nothing");
    let named = refused(
        &mut mcp,
        "saved_search_set",
        json!({ "id": id, "name": "other" }),
        "arguments",
    );
    assert!(
        named["error"]
            .as_str()
            .is_some_and(|e| e.contains("names the whole search")),
        "{named}"
    );
    refused(
        &mut mcp,
        "saved_search_set",
        json!({ "if_revision": 4, "name": "x", "text": "rarity=rare" }),
        "arguments",
    );
    let zero = answered(
        &mut mcp,
        "saved_search_set",
        json!({ "if_revision": 0, "name": "zero", "text": "rarity=rare" }),
    );
    assert_eq!(
        (&zero["prior"], &zero["revision"]),
        (&Value::Null, &json!(1)),
        "0 is the create sentinel: {zero}"
    );

    let missing = refused(
        &mut mcp,
        "saved_search_clear",
        json!({ "id": id }),
        "arguments",
    );
    assert!(
        missing["error"]
            .as_str()
            .is_some_and(|e| e.contains("if_revision")),
        "{missing}"
    );
    let at_zero = refused(
        &mut mcp,
        "saved_search_clear",
        json!({ "id": id, "if_revision": 0 }),
        "conflict",
    );
    assert_eq!(at_zero["revision"], json!(1), "{at_zero}");
    assert_eq!(
        at_zero["offers"][0]["request"],
        json!({ "id": id, "if_revision": 1 }),
        "{at_zero}"
    );
    assert_eq!(shown(&mut mcp), before, "a refused clear cleared nothing");

    for word in ["limit", "expand"] {
        let args = match word {
            "limit" => json!({ "saved": id, "limit": 3 }),
            _ => json!({ "saved": id, "expand": true }),
        };
        let doc = refused(&mut mcp, "saved_searches", args, "arguments");
        assert!(
            doc["error"]
                .as_str()
                .is_some_and(|e| e.contains("`saved`") && e.contains(&format!("`{word}`"))),
            "{doc}"
        );
    }

    for (word, beside) in [
        ("query", json!({ "query": { "text": "rarity=rare" } })),
        ("scope.realm", json!({ "scope": { "realm": "pc" } })),
        (
            "scope.membership",
            json!({ "scope": { "membership": "all" } }),
        ),
        ("request", json!({ "request": {} })),
        ("basis", json!({ "basis": {} })),
        ("count", json!({ "count": 1 })),
        ("counted_at", json!({ "counted_at": {} })),
    ] {
        let mut args = beside;
        args["saved"] = json!(id);
        let doc = refused(&mut mcp, "search", args, "view");
        assert!(
            doc["error"]
                .as_str()
                .is_some_and(|e| e.contains(&format!("`{word}` has nowhere to stand"))),
            "{word}: {doc}"
        );
    }

    for scope in [json!({ "acount": "Alice#1234" }), json!({ "account": 5 })] {
        let without = tool(&mut mcp, "search", json!({ "scope": scope })).1;
        let (ok, with) = tool(&mut mcp, "search", json!({ "saved": id, "scope": scope }));
        assert!(!ok, "{with}");
        assert_eq!(
            with, without,
            "the reader refuses {scope} beside `saved` alike"
        );
    }
    assert!(no_daemon_appeared(base));
}

/// Every way a receipt or a refusal offers is a request this server
/// takes, sent back as printed: each undo lands and puts the prior back;
/// each `ambiguous_saved_search` offer answers, from `saved_searches` and
/// from `search`, a name eleven rows share offering every row too;
/// `saved_search_not_found`'s list answers; a conflict's write lands;
/// each `realm_needed` create lands; and a run naming no account over two
/// is refused `ambiguous_account`, each offer answering.
#[test]
fn every_saved_offer_sent_back_as_printed_answers() {
    let dir = scratch("mcp-saved-offers");
    let base = &dir.0;
    search_tools::seed(base);
    let mut mcp = Mcp::start(base, &[]);
    let row = |mcp: &mut Mcp, id: &str| answered(mcp, "saved_searches", json!({ "saved": id }));

    // the undo of each write
    let created = answered(
        &mut mcp,
        "saved_search_set",
        json!({ "name": "rings", "text": "class:ring" }),
    );
    let id = id_of(&created);
    let (ok, cleared) = sent(&mut mcp, "saved_search_set", &created["undo"]);
    assert!(ok, "{cleared}");
    assert_eq!(cleared["written"], Value::Null, "{cleared}");
    let (ok, recreated) = sent(&mut mcp, "saved_search_clear", &cleared["undo"]);
    assert!(ok, "{recreated}");
    assert_eq!(
        (&row(&mut mcp, &id)["text"], &recreated["revision"]),
        (&json!("class:ring"), &json!(3)),
        "the count continues past the clear: {recreated}"
    );
    let renamed = answered(
        &mut mcp,
        "saved_search_set",
        json!({ "id": id, "if_revision": 3, "name": "renamed" }),
    );
    let (ok, restored) = sent(&mut mcp, "saved_search_set", &renamed["undo"]);
    assert!(ok, "{restored}");
    assert_eq!(row(&mut mcp, &id)["name"], json!("rings"));

    // a conflict's write again, at the revision that stands
    let stale = refused(
        &mut mcp,
        "saved_search_set",
        json!({ "id": id, "if_revision": 1, "name": "again" }),
        "conflict",
    );
    let (ok, landed) = sent(&mut mcp, "saved_search_set", &stale["offers"][0]);
    assert!(ok, "{landed}");
    assert_eq!(row(&mut mcp, &id)["name"], json!("again"));

    // the list a name none holds offers
    let none = refused(
        &mut mcp,
        "search",
        json!({ "saved": "nope" }),
        "saved_search_not_found",
    );
    let (ok, listed) = sent(&mut mcp, "search", &none["offers"][0]);
    assert!(ok, "{listed}");
    assert_eq!(listed["total"], json!(1), "{listed}");

    // a name eleven rows share: ten by id, and every row
    for _ in 0..11 {
        answered(
            &mut mcp,
            "saved_search_set",
            json!({ "name": "dup", "text": "rarity=rare" }),
        );
    }
    for called in ["saved_searches", "search"] {
        let doc = refused(
            &mut mcp,
            called,
            json!({ "saved": "dup" }),
            "ambiguous_saved_search",
        );
        let offers = doc["offers"].as_array().unwrap();
        assert_eq!(offers.len(), 11, "{doc}");
        for offer in offers {
            let (ok, answer) = sent(&mut mcp, called, offer);
            assert!(ok, "{called} {offer}: {answer}");
        }
        let every = offers.last().unwrap();
        assert_eq!(
            (&every["request"], every["tool"].as_str().unwrap_or(called)),
            (&json!({ "expand": true }), "saved_searches"),
            "{doc}"
        );
        let (_, all) = sent(&mut mcp, called, every);
        assert_eq!(all["searches"].as_array().map(Vec::len), Some(12), "{all}");
    }
    drop(mcp);

    // each realm a create could be saved over, over a store of two
    let two = scratch("mcp-saved-offers-realms");
    spoken::seed(&two.0);
    let mut mcp = Mcp::start(&two.0, &[]);
    let needed = refused(
        &mut mcp,
        "saved_search_set",
        json!({ "name": "anything", "text": "rarity=rare" }),
        "realm_needed",
    );
    let offers = needed["offers"].as_array().unwrap();
    assert!(offers.len() >= 2, "{needed}");
    for offer in offers {
        let (ok, landed) = sent(&mut mcp, "saved_search_set", offer);
        assert!(ok, "{offer}: {landed}");
        assert_eq!(
            landed["written"]["scope"]["realm"],
            offer["request"]["realm"]
        );
    }
    drop(mcp);

    // a run naming no account over two accounts
    search_tools::seed_bob(base);
    let mut mcp = Mcp::start(base, &[]);
    let ambiguous = refused(
        &mut mcp,
        "search",
        json!({ "saved": id, "view": { "rows": { "limit": 1 } } }),
        "ambiguous_account",
    );
    let offers = ambiguous["offers"].as_array().unwrap();
    assert_eq!(offers.len(), 2, "{ambiguous}");
    for offer in offers {
        assert_eq!(offer["request"]["saved"], json!(id), "{offer}");
        let (ok, answer) = sent(&mut mcp, "search", offer);
        assert!(ok, "{offer}: {answer}");
    }
    assert!(no_daemon_appeared(base) && no_daemon_appeared(&two.0));
}

/// Morning 1's handoff, both ways, with no new channel: a search the
/// owner saves at a terminal is listed by `saved_searches` and run by
/// `search`; one the agent saves is listed by `acq saved list`, its
/// channel `mcp`.
#[test]
fn the_handoff_runs_both_ways() {
    let dir = scratch("mcp-saved-handoff");
    let base = &dir.0;
    search_tools::seed(base);
    let owners = id_of(&printed(
        base,
        &["saved", "set", "--name", "rings", "class:ring"],
    ));
    let mut mcp = Mcp::start(base, &[]);
    let listed = answered(&mut mcp, "saved_searches", json!({}));
    let row = &listed["searches"][0];
    assert_eq!(
        (&row["id"], &row["written_via"]),
        (&json!(owners), &json!("cli")),
        "{listed}"
    );
    let ran = answered(&mut mcp, "search", json!({ "saved": "rings" }));
    let plain = answered(
        &mut mcp,
        "search",
        json!({ "query": { "text": "class:ring" }, "scope": { "realm": "pc", "membership": "live" } }),
    );
    assert_eq!(ran, plain, "the owner's search runs as written");

    let agents = id_of(&answered(
        &mut mcp,
        "saved_search_set",
        json!({ "name": "uniques", "text": "rarity=unique" }),
    ));
    let by_cli = printed(base, &["saved", "list"]);
    let row = by_cli["searches"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["id"] == json!(agents))
        .unwrap_or_else(|| panic!("the agent's search is not listed: {by_cli}"));
    assert_eq!(row["written_via"], json!("mcp"), "{row}");
    assert!(no_daemon_appeared(base));
}
