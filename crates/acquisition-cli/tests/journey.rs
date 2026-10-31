//! The journey (C122, C123 — the mock's real-shaped items): one first
//! session a new user types, from login to `daemon stop`, through the
//! real pair of binaries over the mock, held as one golden transcript
//! whose facts come from the table the MCP's twin reads too
//! (`acquisition-client/tests/harness/journey.rs`). Every command but the
//! login and the status is recorded: write a policy, plan and apply until
//! the plan says nothing to do, list the tabs, three searches the
//! fixture's real items answer, price the ring, show it, render the shop,
//! clear the price, stop the daemon. Then, by field: `daemon status
//! --json` before the stop names the daemon the test started, the send
//! journal holds its invariants over the whole session, and every verb
//! the transcript ran is one the README's tour shows.
//!
//! The daemon is started by the test with the journal on its own command
//! alone (every `acq` keeps the isolation list's `ACQ_JOURNAL=0`), so the
//! test owns its pid: a daemon that died and respawned would journal
//! nothing and fail the status check. The login runs outside the
//! transcript, its JSON carrying random OAuth values.

use serde_json::json;

use crate::harness;
use crate::readme_tour;

use harness::journey::*;
use harness::{Transcript, acq, golden, sole_json};

/// One recorded command: its words, how many of them are the verb path
/// (the tour check's), and its facts.
struct Step {
    args: Vec<String>,
    verbs: usize,
    facts: Vec<String>,
}

fn step(args: &[&str], verbs: usize, facts: &[String]) -> Step {
    Step {
        args: args.iter().map(|a| a.to_string()).collect(),
        verbs,
        facts: facts.to_vec(),
    }
}

#[test]
fn a_first_session_reads_as_a_user_meets_it_and_the_journal_holds() {
    let scratch = harness::scratch("journey");
    let base = scratch.0.clone();
    let (knob, journal) = harness::journal_on(&base);
    let (mut daemon, pid) = harness::start_daemon_with(&base, &[(knob, journal.as_str())]);
    let _login = harness::login_as_mock(&base, &[]);

    let item = format!("item/{RING}");
    let [kind, amount, currency] = PRICE;
    let steps = [
        step(
            &["policy", "set", POLICY],
            2,
            &[format!("revision {POLICY_REVISION}")],
        ),
        step(
            &["refresh", "--plan"],
            1,
            &[format!("{} requests", PLAN_REQUESTS[0])],
        ),
        step(
            &["refresh", "--apply"],
            1,
            &[
                format!("{0} requests, {0} done", PLAN_REQUESTS[0]),
                format!("({ACCOUNT}): {0}/{0} done", PLAN_REQUESTS[0]),
            ],
        ),
        step(
            &["refresh", "--plan"],
            1,
            &[format!("{} requests", PLAN_REQUESTS[1])],
        ),
        step(
            &["refresh", "--apply"],
            1,
            &[
                format!("({ACCOUNT}): {0}/{0} done", PLAN_REQUESTS[1]),
                format!("{} requests more are planned now", PLAN_REQUESTS[2]),
                "added".into(),
            ],
        ),
        step(
            &["refresh", "--apply"],
            1,
            &[
                format!("({ACCOUNT}): {0}/{0} done", PLAN_REQUESTS[2]),
                "every covered entry is fresh".into(),
            ],
        ),
        step(&["refresh", "--plan"], 1, &["nothing to do".into()]),
        step(
            &["tabs"],
            1,
            &[format!(
                "{TABS_LISTED} tabs in Standard: {TABS_FETCHED} fetched, 1 folder (never fetched); {STASH_ITEMS} items"
            )],
        ),
        step(
            &["search", RING_QUERY],
            1,
            &[format!("total   {RING_MATCHES} match"), RING_NAME.into()],
        ),
        step(
            &["search", LIFE_QUERY, "--sort", LIFE_SORT],
            1,
            &[
                format!("total   {LIFE_MATCHES} matches"),
                format!("{ARMOUR_NAME} Saint's Hauberk"),
                format!("sorts by {LIFE_SORT} {ARMOUR_TOTAL_LIFE}"),
                format!("sorts by {LIFE_SORT} {RING_TOTAL_LIFE}"),
            ],
        ),
        step(
            &[
                "search",
                CURRENCY_QUERY,
                "--count",
                "base",
                "--sum",
                "stack",
            ],
            1,
            &[
                format!("total   {CURRENCY_MATCHES} match"),
                format!("sum     stack over every match: {SHARD_STACK}"),
                format!("{SHARD}   sum {SHARD_STACK}"),
            ],
        ),
        step(
            &["show", RING],
            1,
            &[
                "unread  nothing".into(),
                "explicit, crafted     +15% to Cold and Chaos Resistances".into(),
                "price   none".into(),
            ],
        ),
        step(
            &["price", "set", &item, kind, amount, currency],
            2,
            &[
                format!("{item}: {PRICE_SHOWN} (revision {PRICE_REVISION}), was unset"),
                format!(
                    "`acq price clear --account {ACCOUNT} {item} --if-revision {PRICE_REVISION}` undoes it"
                ),
            ],
        ),
        step(
            &["show", RING],
            1,
            &[format!("price   {PRICE_SHOWN} · manual")],
        ),
        step(
            &["shop", "render"],
            2,
            &[
                format!(
                    "{POSTED_ITEMS} item to post on {POSTED_PAGES} page; 1 omitted (1 the game lists)"
                ),
                format!("[spoiler=\" ~price {PRICE_SHOWN}\"]"),
            ],
        ),
        step(
            &["price", "clear", &item],
            2,
            &[format!(
                "{item}: cleared, was {PRICE_SHOWN} (revision {PRICE_REVISION})"
            )],
        ),
    ];

    let mut t = Transcript::new(&base).fetched_live();
    for s in &steps {
        let args: Vec<&str> = s.args.iter().map(String::as_str).collect();
        let facts: Vec<&str> = s.facts.iter().map(String::as_str).collect();
        let out = t.run(&args, &facts);
        assert!(out.status.success(), "acq {args:?}: {out:?}");
    }

    // The daemon that served the session is the one the test started.
    let status = sole_json(&acq(&base, &["daemon", "status", "--json"]));
    assert_eq!(
        (
            &status["running"],
            &status["provider"],
            &status["compatible"],
            &status["pid"]
        ),
        (&json!(true), &json!("mock"), &json!(true), &json!(pid)),
        "{status}"
    );

    let stop = step(&["daemon", "stop"], 2, &["daemon stopped (pid".into()]);
    let args: Vec<&str> = stop.args.iter().map(String::as_str).collect();
    let facts: Vec<&str> = stop.facts.iter().map(String::as_str).collect();
    assert!(t.run(&args, &facts).status.success());
    daemon.wait_exit(std::time::Duration::from_secs(120));
    golden(file!(), "a_first_session", &t);

    harness::assert_journal_holds(&base, pid, &GETS, TOKEN_POSTS);

    // Every verb path the journey ran is one the README's tour shows.
    let toured: Vec<Vec<String>> = readme_tour::tour_lines()
        .iter()
        .filter(|line| line.starts_with("acq "))
        .flat_map(|line| readme_tour::parse(line))
        .map(|(path, _)| path)
        .collect();
    for s in steps.iter().chain([&stop]) {
        let path = &s.args[..s.verbs];
        assert!(
            toured.iter().any(|p| p == path),
            "`acq {}` is in the journey and in no tour line of README.md",
            path.join(" ")
        );
    }
}
