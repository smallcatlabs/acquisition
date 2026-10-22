//! Process-level pin of the `--json` stdout contract for the policy/plan
//! surface (C44 end to end: a moved policy revision refuses apply with
//! the remedy named; C38: a tampered envelope never reaches the daemon;
//! C41: plan and no-op apply run with no daemon at all — `ACQ_NO_SPAWN`
//! makes any contact an error): the CLI is itself an API (C11),
//! step 7's apply will consume `refresh --plan --json`, and only a spawned
//! binary can prove what actually lands on stdout — the in-process tests
//! cannot see it.
//!
//! Facts are seeded through the store crate directly (the same crate the
//! daemon writes through), so no daemon runs: the socket points into an
//! empty temp dir and `ACQ_NO_SPAWN=1` makes any spawn attempt an error
//! rather than a stray process.

use std::path::{Path, PathBuf};

use acquisition_plan::RefreshPlan;
use acquisition_store::{Endpoint, Index, Store, account_path};
use serde_json::{Value, json};

use crate::harness;

use harness::{offline_acq as acq, sole_json};

const USER: &str = "Alice#1234";
const UUID: &str = "u-e2e";

fn seed_store(base: &Path) -> PathBuf {
    let mock = base.join("mock");
    std::fs::create_dir_all(&mock).unwrap();
    // Seed at the current clock: the plan below compiles at real time, and
    // a fresh listing keeps the expected action set to exactly one
    // never-fetched tab (an ancient listing would add a re-list).
    let at = acquisition_store::now();
    let mut index = Index::load(&mock).unwrap();
    index.record_login(USER, UUID, false, at).unwrap();
    let mut store = Store::open(&account_path(&mock, USER)).unwrap();
    store
        .record(
            &Endpoint::Profile,
            &json!({}),
            200,
            &json!({ "uuid": UUID, "name": USER }),
            at,
        )
        .unwrap();
    store
        .record(
            &Endpoint::Stashes {
                realm: "pc".into(),
                league: "Standard".into(),
            },
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": [
                { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 },
            ] }),
            at,
        )
        .unwrap();
    mock
}

/// `--if-revision 0` writes only where no policy stands (revisions start
/// at 1), so a driver that read none writes under compare-and-swap too:
/// on an empty world it writes revision 1, and a second write under 0 is
/// refused naming the stored revision, as a wrong revision is, leaving
/// revision 1 standing.
#[test]
fn a_policy_written_under_revision_0_is_created_only_where_none_stands() {
    let scratch = harness::scratch("create0");
    let base = scratch.0.clone();
    seed_store(&base);
    let first = r#"{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":"all","max_age_seconds":3600}}}}}"#;
    let out = acq(
        &base,
        &["policy", "set", first, "--if-revision", "0", "--json"],
    );
    assert!(out.status.success(), "{out:?}");
    assert_eq!(sole_json(&out)["revision"], json!(1));

    let second = r#"{"version":3,"realms":{"pc":{"leagues":{"Hardcore":{"tabs":"all","max_age_seconds":3600}}}}}"#;
    let out = acq(
        &base,
        &["policy", "set", second, "--if-revision", "0", "--json"],
    );
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let refused = sole_json(&out);
    assert_eq!(refused["kind"], json!("refused"), "{refused}");
    assert!(
        refused["error"]
            .as_str()
            .unwrap()
            .contains("is at revision 1"),
        "{refused}"
    );
    let shown = sole_json(&acq(&base, &["policy", "show", "--json"]));
    assert_eq!(shown["revision"], json!(1));
    assert_eq!(
        shown["value"],
        serde_json::from_str::<Value>(first).unwrap()
    );
}

#[test]
fn json_stdout_is_the_contract_surface() {
    let scratch = harness::scratch("e2e");
    let base = scratch.0.clone();
    seed_store(&base);

    // No policy yet: `policy show --json` answers null, exit 0 — absent
    // intent is data, not an error.
    let out = acq(&base, &["policy", "show", "--json"]);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(sole_json(&out), Value::Null);

    // `--json` is total: a refusal is {"error": …} on stdout with exit 1,
    // not prose on stderr.
    let out = acq(&base, &["refresh", "--plan", "--json"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let err = sole_json(&out);
    assert!(
        err["error"].as_str().unwrap().contains("no sync policy"),
        "{err}"
    );

    // A typo'd policy is refused as {"error": …} naming the field, and
    // stores nothing (the show below still answers null).
    let out = acq(
        &base,
        &[
            "policy",
            "set",
            r#"{"version":1,"leagues":{"Standard":{"tabs":"all","max_age_secs":60}}}"#,
            "--json",
        ],
    );
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    assert!(
        sole_json(&out)["error"]
            .as_str()
            .unwrap()
            .contains("max_age_secs")
    );
    let out = acq(&base, &["policy", "show", "--json"]);
    assert_eq!(sole_json(&out), Value::Null);

    // A valid write answers the stored row, and C126's note on a league
    // the fetched league list does not hold: here no list was ever
    // fetched. Show answers the same row.
    let policy = r#"{"version":1,"leagues":{"Standard":{"tabs":"all","max_age_seconds":3600}}}"#;
    let out = acq(&base, &["policy", "set", policy, "--json"]);
    assert!(out.status.success(), "{out:?}");
    let mut row = sole_json(&out);
    assert_eq!(
        row.as_object_mut().unwrap().remove("league_notes"),
        Some(json!([{ "league": "Standard", "list_fetched_at": null }]))
    );
    assert_eq!(row["revision"], json!(1));
    assert_eq!(row["value"], serde_json::from_str::<Value>(policy).unwrap());
    // C65: the row says which channel wrote it; no actor was claimed.
    assert_eq!(row["written_via"], json!("cli"));
    assert!(row.get("actor").is_none(), "{row}");
    let out = acq(&base, &["policy", "show", "--json"]);
    assert_eq!(sole_json(&out), row);

    // A conflicting --if-revision is {"error": …} naming the current
    // revision, exit 1.
    let out = acq(
        &base,
        &["policy", "set", policy, "--if-revision", "7", "--json"],
    );
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    assert!(
        sole_json(&out)["error"]
            .as_str()
            .unwrap()
            .contains("revision 1")
    );

    // The plan: stdout is exactly one JSON document that the planner's own
    // validating parse accepts — schema stamp, recomputed derived
    // quantities and all — so what a pipe receives is what apply can
    // trust. No daemon means no quote, and the reason goes to stderr,
    // never into the envelope.
    let out = acq(&base, &["refresh", "--plan", "--json"]);
    assert!(out.status.success(), "{out:?}");
    let envelope = sole_json(&out);
    let plan = RefreshPlan::from_value(&envelope)
        .unwrap_or_else(|e| panic!("stdout is not a valid plan envelope ({e}):\n{envelope:#}"));
    assert_eq!(plan.league, "Standard");
    assert_eq!(plan.account_uuid, UUID);
    assert_eq!(plan.account_name.as_deref(), Some(USER));
    assert_eq!(plan.provider, "mock");
    assert_eq!(plan.basis.policy_revision, 1);
    // The seeded facts: one listed, never-fetched tab (the listing itself
    // is fresh at plan time, so the fetch is the single action).
    assert_eq!(plan.logical_requests, 1, "{:?}", plan.actions);
    assert_eq!(plan.quote, None);
    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(stderr.contains("no quote"), "{stderr}");

    // ---- the apply gates that run before any daemon contact ----
    // (ACQ_NO_SPAWN=1 stands for the whole test, so any path that reached
    // the daemon would fail with its distinct "forbids starting" error —
    // the assertions below prove each gate fires first.)

    // The step-7 staleness ruling, end to end: the reviewed envelope cites
    // revision 1; a policy write moves intent to revision 2; applying the
    // stale plan is a structured refusal naming both revisions.
    let stale_plan = base.join("stale-plan.json");
    std::fs::write(&stale_plan, serde_json::to_vec(&envelope).unwrap()).unwrap();
    let out = acq(&base, &["policy", "set", policy, "--json"]);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(sole_json(&out)["revision"], json!(2));
    let apply_arg = format!("--apply={}", stale_plan.display());
    let out = acq(&base, &["refresh", &apply_arg, "--json"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let err = sole_json(&out);
    let msg = err["error"].as_str().unwrap();
    assert!(
        msg.contains("revision 1") && msg.contains("revision 2") && msg.contains("replan"),
        "{msg}"
    );

    // A plan at the current revision passes the gate — and then needs the
    // daemon, which this harness forbids: the refusal is the connect
    // policy's, proving apply spends only through a daemon.
    let out = acq(&base, &["refresh", "--apply", "--json"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let msg = sole_json(&out);
    let msg = msg["error"].as_str().unwrap();
    assert!(msg.contains("ACQ_NO_SPAWN"), "{msg}");

    // A tampered envelope never reaches the daemon either: the planner's
    // validating parse refuses it whole (forged counts are exactly what
    // admission budgeting must not trust).
    let mut forged: Value = envelope.clone();
    forged["logical_requests"] = json!(0);
    let forged_path = base.join("forged-plan.json");
    std::fs::write(&forged_path, serde_json::to_vec(&forged).unwrap()).unwrap();
    let apply_arg = format!("--apply={}", forged_path.display());
    let out = acq(&base, &["refresh", &apply_arg, "--json"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let err = sole_json(&out);
    assert!(
        err["error"].as_str().unwrap().contains("malformed"),
        "{err}"
    );

    // An empty plan applies as a no-op without any daemon at all: fetch
    // the one tab through the store crate (as the daemon would record it),
    // and the compiled plan has nothing to do.
    let mock = base.join("mock");
    let mut store = Store::open(&account_path(&mock, USER)).unwrap();
    store
        .record(
            &Endpoint::Stash {
                realm: "pc".into(),
                league: "Standard".into(),
                id: "t1".into(),
                sub: None,
            },
            &json!({ "league": "Standard", "id": "t1", "deep": false }),
            200,
            &json!({ "stash": { "id": "t1", "name": "One", "type": "PremiumStash", "items": [] } }),
            acquisition_store::now(),
        )
        .unwrap();
    drop(store);
    let out = acq(&base, &["refresh", "--apply", "--json"]);
    assert!(out.status.success(), "{out:?}");
    let done = sole_json(&out);
    assert_eq!(done["requests"], json!(0));
    assert!(
        done["note"].as_str().unwrap().contains("nothing to do"),
        "{done}"
    );
}

/// C36, amended 2026-10-19 — `policy set` writes the realms it names and
/// keeps the rest: a poe2 policy set alone leaves the pc one standing
/// (the README's first known gap until then), a realm named with no
/// leagues is removed, and `--replace` alone writes the whole policy.
#[test]
fn a_poe2_policy_set_alone_leaves_the_pc_policy_standing() {
    let scratch = harness::scratch("e2e-merge");
    let base = scratch.0.clone();
    seed_store(&base);
    let pc = r#"{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":"all","max_age_seconds":3600}}}}}"#;
    let poe2 = r#"{"version":3,"realms":{"poe2":{"leagues":{"Standard":{"characters":"all","max_age_seconds":600}}}}}"#;
    let no_poe2 = r#"{"version":3,"realms":{"poe2":{"leagues":{}}}}"#;
    let set = |args: &[&str]| {
        let out = acq(&base, &[&["policy", "set"], args, &["--json"]].concat());
        assert!(out.status.success(), "{out:?}");
        sole_json(&out)
    };
    assert_eq!(set(&[pc])["revision"], json!(1));
    let both = set(&[poe2]);
    assert_eq!(both["revision"], json!(2));
    assert_eq!(
        both["value"]["realms"]["pc"],
        serde_json::from_str::<Value>(pc).unwrap()["realms"]["pc"],
        "{both}"
    );
    assert!(both["value"]["realms"]["poe2"].is_object(), "{both}");
    // The pc plan still compiles: its coverage survived the poe2 write.
    let out = acq(&base, &["refresh", "--plan", "--json"]);
    assert!(out.status.success(), "{out:?}");
    let gone = set(&[no_poe2]);
    assert_eq!(gone["revision"], json!(3));
    assert_eq!(gone["value"], serde_json::from_str::<Value>(pc).unwrap());
    // `--replace` writes the whole: the pc realm the value omits is gone.
    let replaced = set(&[poe2, "--replace"]);
    assert_eq!(replaced["revision"], json!(4));
    assert_eq!(
        replaced["value"],
        serde_json::from_str::<Value>(poe2).unwrap()
    );
}
