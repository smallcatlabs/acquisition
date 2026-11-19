//! The tracer's plan→apply→replan loop at process level (step 7): a real
//! daemon over the in-process mock provider, a scripted browserless login,
//! and the CLI as the only interface — the loop must *close*: each apply
//! executes exactly its plan's actions (C38, C43: one child per tuple,
//! `children.done` equals `requests`), discovery (substash stubs) waits
//! for the next plan instead of expanding the current one, the admission
//! budget refuses a plan whole and the next plan still owes every fetch
//! (C42), and after a bootstrap listing plus two reconciliation cycles
//! the plan is empty.
//! The policy names tabs and characters together (characters joined the
//! plan 2026-09-02): both listings in the bootstrap cycle, both facets'
//! fetches in the next, and the character facts on record at the end.
//!
//! Slow-ish on purpose (one daemon, ~21 mock sends, real waits): this is
//! the one test that proves the whole slice through the real binaries.

use std::path::Path;

use acquisition_plan::{RefreshAction, RefreshPlan};
use serde_json::{Value, json};

use crate::harness;

use harness::{Transcript, acq, golden, login_as_mock, sole_json};

/// `refresh --plan --json` as the planner's own validated envelope — what
/// stdout carries is what apply can trust.
fn plan(base: &Path) -> RefreshPlan {
    let out = acq(base, &["refresh", "--plan", "--json"]);
    assert!(out.status.success(), "{out:?}");
    RefreshPlan::from_value(&sole_json(&out)).expect("stdout is a valid plan envelope")
}

/// Apply and return the parent job's successful payload.
fn apply(base: &Path, args: &[&str]) -> Value {
    let out = acq(base, args);
    assert!(out.status.success(), "{out:?}");
    let outcome = sole_json(&out);
    assert_eq!(outcome["outcome"], json!("success"), "{outcome}");
    outcome["payload"].clone()
}

#[test]
fn the_plan_apply_replan_loop_closes_against_the_mock() {
    let scratch = harness::scratch("al");
    let base = scratch.0.clone();

    let _daemon = login_as_mock(&base, &[]);
    let out = acq(
        &base,
        &[
            "policy",
            "set",
            r#"{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":"all","characters":"all","max_age_seconds":3600}}}}}"#,
            "--json",
        ],
    );
    assert!(out.status.success(), "{out:?}");
    assert_eq!(sole_json(&out)["revision"], json!(1));

    // Cycle 1 — never listed on either route: the plan is the two
    // listings alone (no membership authority without a basis), and
    // applying it runs exactly two requests.
    let first = plan(&base);
    assert!(
        matches!(
            first.actions[..],
            [
                RefreshAction::ListStashes { .. },
                RefreshAction::ListCharacters { .. }
            ]
        ),
        "{:?}",
        first.actions
    );
    let payload = apply(&base, &["refresh", "--apply", "--json"]);
    assert_eq!(payload["requests"], json!(2));
    assert_eq!(payload["children"]["done"], json!(2));

    // Cycle 2 — both listings are on record: every listed tab wants its
    // first fetch (the mock lists 5 top-level tabs plus 2 folder children;
    // the folder itself is skipped, never fetched), so does each of the
    // mock's 2 pc characters, and nothing re-lists.
    let second = plan(&base);
    assert_eq!(second.logical_requests, 9, "{:?}", second.actions);
    let (tabs, characters): (Vec<_>, Vec<_>) = second
        .actions
        .iter()
        .partition(|a| matches!(a, RefreshAction::FetchTab { .. }));
    assert_eq!(tabs.len(), 7, "{:?}", second.actions);
    assert!(
        characters
            .iter()
            .all(|a| matches!(a, RefreshAction::FetchCharacter { .. }))
            && characters.len() == 2,
        "{:?}",
        second.actions
    );

    // The admission budget refuses this plan whole, before any child runs:
    // the daemon's error comes back through the CLI as a structured
    // failure, and the next plan still owes all 9 fetches.
    let out = acq(
        &base,
        &["refresh", "--apply", "--max-requests", "1", "--json"],
    );
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let err = sole_json(&out);
    assert!(
        err["error"]
            .as_str()
            .unwrap()
            .contains("exceeds the budget"),
        "{err}"
    );
    assert_eq!(plan(&base).logical_requests, 9);

    // The reviewed-envelope path: apply the exact plan that was printed.
    let reviewed = base.join("cycle2.json");
    std::fs::write(&reviewed, serde_json::to_vec(&second).unwrap()).unwrap();
    let apply_arg = format!("--apply={}", reviewed.display());
    let payload = apply(&base, &["refresh", &apply_arg, "--json"]);
    assert_eq!(payload["requests"], json!(9));
    assert_eq!(payload["children"]["done"], json!(9));

    // The character facts landed under the id-keyed rows: both fetched,
    // with items, readable with no daemon.
    let out = acq(&base, &["store", "characters", "--realm", "pc", "--json"]);
    assert!(out.status.success(), "{out:?}");
    // one document (C149): the counts beside the rows
    let listed = sole_json(&out);
    assert_eq!(listed["characters"], 2, "{listed}");
    assert_eq!(listed["fetched"], 2, "{listed}");
    let rows = listed["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 2, "{rows:?}");
    for row in rows {
        assert!(row["fetched_at"].is_i64(), "{row}");
        assert!(row["item_count"].as_i64().unwrap() > 0, "{row}");
    }

    // Cycle 3 — fetching the map/unique parents landed their substash
    // stubs, so discovery arrives one plan later (never as an expansion of
    // the plan that discovered it): 7 substashes, fetched under their
    // parents.
    let third = plan(&base);
    assert_eq!(third.logical_requests, 7, "{:?}", third.actions);
    assert!(
        third
            .actions
            .iter()
            .all(|a| matches!(a, RefreshAction::FetchSubstash { .. })),
        "{:?}",
        third.actions
    );
    let payload = apply(&base, &["refresh", "--apply", "--json"]);
    assert_eq!(payload["children"]["done"], json!(7));

    // Cycle 4 — closed: everything covered is fresh on both facets, and
    // the no-op apply spends nothing.
    let fourth = plan(&base);
    assert!(fourth.actions.is_empty(), "{:?}", fourth.actions);
    assert_eq!(
        fourth.skipped_characters.len(),
        2,
        "{:?}",
        fourth.skipped_characters
    );
    let out = acq(&base, &["refresh", "--apply", "--json"]);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(sole_json(&out)["requests"], json!(0));

    // `--characters` is a selection too (C76): the realm's character
    // listing and the one character named by its id, fresh as it is,
    // applied through the same apply kind
    let id = rows[0]["id"].as_str().expect("a character's id");
    let payload = apply(&base, &["refresh", "--characters", id, "--json"]);
    assert_eq!(payload["requests"], json!(2));
    assert_eq!(payload["children"]["done"], json!(2));

    let out = acq(&base, &["daemon", "stop"]);
    assert!(out.status.success(), "{out:?}");
}

/// A failed apply prints in the daemon's own words, its count sentence,
/// and the rows of the failed children its tally lists (C23): a plan
/// naming a tab the provider does not have fails that one child, through
/// the real pair of binaries.
#[test]
fn a_failed_apply_counts_its_failed_requests_from_the_daemons_own_words() {
    let scratch = harness::scratch("af");
    let base = scratch.0.clone();

    let _daemon = login_as_mock(&base, &[]);
    let policy = r#"{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":"all","max_age_seconds":3600}}}}}"#;
    assert!(acq(&base, &["policy", "set", policy]).status.success());
    assert!(acq(&base, &["stashes"]).status.success());
    let mut plan = sole_json(&acq(&base, &["refresh", "--plan", "--json"]));
    let planned = plan["logical_requests"].as_u64().expect("a count");
    assert!(planned > 1, "{plan}");
    // The quote names each job it priced, and a plan whose actions no
    // longer match it is refused as malformed: it goes with the edit.
    plan["actions"][0]["id"] = json!("no-such-tab");
    plan.as_object_mut().unwrap().shift_remove("quote");
    let file = base.join("plan.json");
    std::fs::write(&file, plan.to_string()).unwrap();

    // Stays a literal: a failed apply prints the daemon log's path, whose
    // directory is the world's hash, which no golden rule normalises.
    // The failure's report is on stderr, as `acq result` prints it below
    // (#101: one stream for a failed job's report on both verbs); stdout
    // keeps the submission and what the store took.
    let out = acq(&base, &["refresh", &format!("--apply={}", file.display())]);
    let said = String::from_utf8_lossy(&out.stderr).into_owned();
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stop = acq(&base, &["daemon", "stop"]);
    assert_eq!(out.status.code(), Some(1), "{stdout}{said}");
    assert!(
        said.contains(&format!(
            "failed: 1 of {planned} child jobs failed, {} done\n",
            planned - 1
        )),
        "stdout: {stdout}\nstderr: {said}"
    );
    assert!(said.contains("daemon log: "), "{said}");
    assert!(!stdout.contains("failed"), "{stdout}");
    assert!(stop.status.success(), "{stop:?}");
    // The failed child's row, under the daemon's sentence.
    assert!(said.contains("no-such-tab"), "{said}");

    // With the daemon stopped, the parent's result read from the ledger
    // prints the same: the daemon's sentence and the failed child's row
    // (C45), and its JSON details the child (C23).
    let parent = stdout
        .split("as job ")
        .nth(1)
        .and_then(|rest| rest.split_whitespace().next())
        .expect("the applying line names the job")
        .to_string();
    let read_back = format!(
        "failed: 1 of {planned} child jobs failed, {} done",
        planned - 1
    );
    let mut t = Transcript::new(&base);
    let out = t.run(&["result", &parent], &[&read_back, "no-such-tab"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let told = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        told.contains(&format!(
            "job {parent} failed: 1 of {planned} child jobs failed, {} done",
            planned - 1
        )),
        "{told}"
    );
    assert!(told.contains("no-such-tab"), "{told}");
    golden(file!(), "a_failed_applys_result_from_the_ledger", &t);
    let doc = sole_json(&acq(&base, &["result", &parent, "--json"]));
    let failed = doc["failed_children"].as_array().expect("failed_children");
    assert_eq!(failed.len(), 1, "{doc}");
    assert_eq!(
        failed[0]["job_id"], doc["children"]["failed_ids"][0],
        "{doc}"
    );
    assert!(failed[0]["error"].is_string(), "{doc}");
}

/// C190 as #291 amended it: an apply over a route closed at the cap
/// waits, its progress line saying the closure in the status's words and
/// when the trial goes; nothing ends it but its jobs or a person, whose
/// reset opens the route, and the same apply goes on. The closure is
/// staged in the world's rails state at the cap, its trial an hour off,
/// where a daemon keeps it across lifetimes. The test waits on the
/// apply's own output, then on its exit, each under the harness's
/// deadline, never on a sleep's length.
#[test]
fn c190_an_apply_over_a_route_closed_at_the_cap_waits_saying_the_closure() {
    use std::io::BufRead as _;
    let scratch = harness::scratch("acl");
    let base = scratch.0.clone();
    let login = login_as_mock(&base, &[]);
    let policy = r#"{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":"all","max_age_seconds":3600}}}}}"#;
    assert!(acq(&base, &["policy", "set", policy]).status.success());
    assert!(acq(&base, &["stashes"]).status.success());
    assert!(acq(&base, &["daemon", "stop"]).status.success());
    while sole_json(&acq(&base, &["daemon", "status", "--json"]))["running"] != false {
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    // The daemon the login started is stopped; the next is this test's.
    drop(login);
    let now = harness::now();
    std::fs::write(
        base.join("store").join("mock").join("rails.json"),
        format!(
            r#"{{"closed_routes":{{"stash@ExileTester":{{"invalid":13,"status":400,"closed_at":{now},"cooldown_secs":3600,"until":{}}}}}}}"#,
            now + 3600
        ),
    )
    .unwrap();
    // A login again, its daemon reading the staged state at its start.
    let _daemon = login_as_mock(&base, &[]);

    let mut apply = harness::command(&base, &["refresh", "--apply"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawning the apply");
    let stderr = apply.stderr.take().expect("piped stderr");
    let (lines, read) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for line in std::io::BufReader::new(stderr)
            .lines()
            .map_while(Result::ok)
        {
            if lines.send(line).is_err() {
                return;
            }
        }
    });
    let closed = loop {
        let line = read
            .recv_timeout(harness::HANG)
            .expect("the apply drew a progress line naming the closure");
        if line.contains("route closed:") {
            break line;
        }
    };
    for words in [
        "route closed: stash@ExileTester for 3600s",
        "GGG answered 13 requests in a row on it as invalid, the last 400",
        "one waiting job goes as the trial in ~60m",
    ] {
        assert!(closed.contains(words), "lacks {words:?}: {closed}");
    }
    // Still waiting: the apply has not ended, and its fetches wait.
    assert!(
        apply.try_wait().expect("the apply's state").is_none(),
        "the apply ended over a closed route"
    );
    let status = sole_json(&acq(&base, &["daemon", "status", "--json"]));
    assert!(
        status["jobs_waiting"].as_u64().is_some_and(|n| n > 0),
        "its fetches wait: {status}"
    );
    assert_eq!(
        status["rails"]["closed_routes"][0]["until"],
        json!(now + 3600),
        "{status}"
    );

    // The person's verb opens the route, and the same apply, its client
    // still waiting, goes on and finishes, under the harness's deadline.
    assert!(acq(&base, &["daemon", "reset-tripwire"]).status.success());
    let deadline = std::time::Instant::now() + harness::HANG;
    let ended = loop {
        if let Some(ended) = apply.try_wait().expect("the apply's state") {
            break ended;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "the apply did not finish after the reset"
        );
        std::thread::sleep(std::time::Duration::from_millis(20));
    };
    assert!(ended.success(), "the apply failed: {ended:?}");
    assert!(acq(&base, &["daemon", "stop"]).status.success());
}
