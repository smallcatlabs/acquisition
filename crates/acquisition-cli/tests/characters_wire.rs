//! Characters in the refresh plan at the wire boundary, through the real
//! binaries and the send journal (the GGG-side contract surface,
//! TESTING-NOTES.md): a real daemon over the in-process mock, a scripted
//! browserless login, and the CLI as the only interface. The planner's
//! unit tests pin the plan; this pins what applying a character-only
//! PoE2 policy actually sends and files (C55–C62, ruled 2026-09-02):
//!
//! - a poe2 entry covering characters only is the ordinary v3 shape, and
//!   its plan is the realm's character listing alone, then the fetch;
//! - the sends go out on the realm-suffixed character routes
//!   (`character-list/poe2`, `character/poe2`), each probed before its
//!   first counted send, and nothing goes out on a stash route;
//! - the PoE2 body's `skills` array is lifted, so the skill item is on
//!   record at the character with container `skills`;
//! - the loop closes: the next plan is empty and applies with no daemon.

use std::path::Path;
use std::process::Output;

use acquisition_plan::{FetchReason, ListingReason, RefreshAction, RefreshPlan};
use serde_json::{Value, json};

use crate::harness;

use harness::sole_json;

/// The binary under test with the send journal at `base/sends.jsonl`, the
/// GGG-side surface this file reads (TESTING-NOTES.md): set on every
/// command, so a daemon any of them starts writes it.
fn acq(base: &Path, args: &[&str]) -> Output {
    harness::command(base, args)
        .env("ACQ_JOURNAL", harness::journal_path(base))
        .output()
        .expect("spawning acq")
}

/// The scripted mock login, its daemon journaling to `base/sends.jsonl`.
fn login_journaled(base: &Path) {
    let (knob, journal) = harness::journal_on(base);
    harness::login_as_mock(base, &[(knob, journal.as_str())]);
}

/// `refresh --plan --realm poe2 --json` as the planner's own validated
/// envelope.
fn plan(base: &Path) -> RefreshPlan {
    let out = acq(base, &["refresh", "--plan", "--realm", "poe2", "--json"]);
    assert!(out.status.success(), "{out:?}");
    RefreshPlan::from_value(&sole_json(&out)).expect("stdout is a valid plan envelope")
}

/// Apply the poe2 plan and return the parent job's successful payload.
fn apply(base: &Path) -> Value {
    let out = acq(base, &["refresh", "--apply", "--realm", "poe2", "--json"]);
    assert!(out.status.success(), "{out:?}");
    let outcome = sole_json(&out);
    assert_eq!(outcome["outcome"], json!("success"), "{outcome}");
    outcome["payload"].clone()
}

#[test]
fn a_character_only_poe2_policy_closes_its_loop_on_the_poe2_routes() {
    let scratch = harness::scratch("cw");
    let base = scratch.0.clone();
    login_journaled(&base);

    // A character-only PoE2 entry is the ordinary v3 shape; a tabs facet
    // under poe2 is refused at the write, nothing stored.
    let out = acq(
        &base,
        &[
            "policy",
            "set",
            r#"{"version":3,"realms":{"poe2":{"leagues":{"Standard":{"tabs":"all","max_age_seconds":3600}}}}}"#,
            "--json",
        ],
    );
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    assert!(
        sole_json(&out)["error"].as_str().unwrap().contains("poe2"),
        "{out:?}"
    );
    let out = acq(
        &base,
        &[
            "policy",
            "set",
            r#"{"version":3,"realms":{"poe2":{"leagues":{"Standard":{"characters":"all","max_age_seconds":3600}}}}}"#,
            "--json",
        ],
    );
    assert!(out.status.success(), "{out:?}");
    assert_eq!(sole_json(&out)["revision"], json!(1));

    // Cycle 1 — the realm's characters were never listed: the listing
    // alone, realm-wide (no league on the action), no stash work at all.
    let first = plan(&base);
    assert!(
        matches!(
            first.actions[..],
            [RefreshAction::ListCharacters {
                reason: ListingReason::NeverListed,
                ..
            }]
        ),
        "{:?}",
        first.actions
    );
    assert!(first.basis.stash_listing.is_none() && first.basis.character_listing.is_none());
    let payload = apply(&base);
    assert_eq!(payload["children"]["done"], json!(1));

    // Cycle 2 — the mock's one PoE2 character owes its first fetch, by
    // the listed name.
    let second = plan(&base);
    assert!(
        matches!(
            &second.actions[..],
            [RefreshAction::FetchCharacter { id, name, reason: FetchReason::NeverFetched, .. }]
                if id == "fake2001" && name == "SecondExile"
        ),
        "{:?}",
        second.actions
    );
    assert!(second.basis.character_listing.is_some());
    let payload = apply(&base);
    assert_eq!(payload["children"]["done"], json!(1));

    // The PoE2 body's `skills` array was lifted: the skill is an item at
    // the character (located by id, container `skills`), under poe2 only.
    let named = |realm: &str| {
        let out = acq(
            &base,
            &[
                "--json",
                "search",
                "--realm",
                realm,
                r#"typeline:"Falling Thunder""#,
            ],
        );
        assert!(out.status.success(), "{out:?}");
        sole_json(&out)
    };
    let found = named("poe2");
    let rows = found["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 1, "{rows:?}");
    let place = &rows[0]["place"];
    assert_eq!(place["kind"], "character");
    assert_eq!(place["id"], "fake2001");
    assert_eq!(place["container"], "skills");
    assert_eq!(place["realm"], "poe2");
    assert_eq!(named("pc")["total"]["matched"], 0);
    let out = acq(&base, &["store", "characters", "--realm", "poe2", "--json"]);
    let rows = sole_json(&out);
    assert_eq!(rows.as_array().unwrap().len(), 1, "{rows}");
    assert!(rows[0]["fetched_at"].is_i64() && rows[0]["item_count"].as_i64().unwrap() > 0);

    // Cycle 3 — closed: the character is fresh, the plan is empty, and the
    // no-op apply spends nothing.
    let third = plan(&base);
    assert!(third.actions.is_empty(), "{:?}", third.actions);
    assert_eq!(
        third.skipped_characters.len(),
        1,
        "{:?}",
        third.skipped_characters
    );
    let out = acq(&base, &["refresh", "--apply", "--realm", "poe2", "--json"]);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(sole_json(&out)["requests"], json!(0));

    // The journal: exactly the poe2 character routes, each probed first,
    // every send 2xx, and nothing on a stash route or a pc route.
    let all = harness::sends(&base);
    let account = all
        .iter()
        .find_map(|send| {
            send.route
                .strip_prefix("character-list/poe2@")
                .map(str::to_string)
        })
        .expect("a poe2 character listing was journaled");
    let on = |route: &str| -> Vec<(String, Option<u64>)> {
        all.iter()
            .filter(|send| send.route == route)
            .map(|send| (send.method.clone(), send.status))
            .collect()
    };
    assert_eq!(
        on(&format!("character-list/poe2@{account}")),
        vec![
            ("HEAD".to_string(), Some(204)),
            ("GET".to_string(), Some(200))
        ]
    );
    assert_eq!(
        on(&format!("character/poe2@{account}")),
        vec![
            ("HEAD".to_string(), Some(204)),
            ("GET".to_string(), Some(200))
        ]
    );
    let data_routes: Vec<&String> = all
        .iter()
        .map(|send| &send.route)
        .filter(|r| !r.starts_with("oauth-token") && !r.starts_with("profile"))
        .collect();
    assert!(
        data_routes.iter().all(|r| r.contains("/poe2@")),
        "a send left the poe2 character routes: {all:?}"
    );
    assert!(
        all.iter()
            .all(|send| matches!(send.status, Some(200..=299))),
        "a non-2xx send: {all:?}"
    );

    let out = acq(&base, &["daemon", "stop"]);
    assert!(out.status.success(), "{out:?}");
}
