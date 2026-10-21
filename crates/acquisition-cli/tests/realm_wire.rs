//! Realm at the wire boundary, through the real binaries and the send
//! journal (the GGG-side contract surface, TESTING-NOTES.md): a real daemon
//! over the in-process mock, a scripted browserless login, and the CLI as
//! the only interface. The unit tests pin `route_for`; this pins what the
//! daemon actually sends and files (review request, 2026-09-02):
//!
//! - an explicit `pc` and an omitted realm are the same route and the
//!   same result — every pc send is byte-identical to the pre-realm ones;
//! - xbox work goes out on its own routes (`stash-list/xbox`, `stash/xbox`),
//!   each probed before its first counted send;
//! - the mock's empty console listing lands nothing under xbox while pc's
//!   tabs stay pc's — the store keeps realms apart;
//! - a realm the stash family does not take (poe2) is refused at admission,
//!   with nothing journaled.

use std::path::Path;
use std::process::Output;

use serde_json::{Value, json};

mod harness;

use harness::sole_json;

/// The binary under test with the send journal at `base/sends.jsonl`, the
/// GGG-side surface this file reads (TESTING-NOTES.md): set on every
/// command, so a daemon any of them starts writes it.
fn acq(base: &Path, args: &[&str]) -> Output {
    harness::command(base, args)
        .env("ACQ_JOURNAL", base.join("sends.jsonl"))
        .output()
        .expect("spawning acq")
}

/// The scripted mock login, its daemon journaling to `base/sends.jsonl`.
fn login_journaled(base: &Path) {
    let journal = base.join("sends.jsonl");
    harness::login_as_mock(
        base,
        &[("ACQ_JOURNAL", journal.to_str().expect("a UTF-8 path"))],
    );
}

/// A job command's successful payload.
fn payload(base: &Path, args: &[&str]) -> Value {
    let out = acq(base, args);
    assert!(out.status.success(), "{out:?}");
    let outcome = sole_json(&out);
    assert_eq!(outcome["outcome"], json!("success"), "{outcome}");
    outcome["payload"].clone()
}

/// The journal's sends as `(method, route, status)`, in order.
fn sends(base: &Path) -> Vec<(String, String, Option<u64>)> {
    let text = std::fs::read_to_string(base.join("sends.jsonl")).expect("journal");
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Value>(l).expect("journal line"))
        .filter(|v| v.get("route").is_some())
        .map(|v| {
            (
                v["method"].as_str().unwrap().to_string(),
                v["route"].as_str().unwrap().to_string(),
                v["status"].as_u64(),
            )
        })
        .collect()
}

#[test]
fn realm_is_on_the_wire_exactly_as_ruled() {
    let scratch = harness::scratch("rw");
    let base = scratch.0.clone();
    login_journaled(&base);

    // Explicit pc and omitted realm: the same route, the same listing.
    let omitted = payload(
        &base,
        &[
            "submit",
            "stashes",
            "--params",
            r#"{"league":"Standard"}"#,
            "--json",
        ],
    );
    let explicit = payload(
        &base,
        &[
            "submit",
            "stashes",
            "--params",
            r#"{"realm":"pc","league":"Standard"}"#,
            "--json",
        ],
    );
    assert_eq!(omitted["stashes"], explicit["stashes"]);
    assert!(!omitted["stashes"].as_array().unwrap().is_empty());

    // xbox: its own route, the mock's empty console listing, and a tab
    // fetch the mock answers 404 (the job fails honestly; the send is
    // journaled on the xbox route).
    let xbox = payload(&base, &["stashes", "--realm", "xbox", "--json"]);
    assert_eq!(xbox["stashes"], json!([]));
    let out = acq(&base, &["stash", "cur1", "--realm", "xbox", "--json"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");

    // poe2: the stash family does not take it — refused at admission,
    // before a job id exists.
    let out = acq(&base, &["stashes", "--realm", "poe2", "--json"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let err = sole_json(&out);
    assert!(
        err["error"]
            .as_str()
            .unwrap()
            .contains("stashes endpoints do not take realm poe2"),
        "{err}"
    );

    // The store keeps realms apart: pc's tabs are on record under pc only.
    let pc_tabs = sole_json(&acq(&base, &["tabs", "--json"]));
    assert!(!pc_tabs["rows"].as_array().unwrap().is_empty(), "{pc_tabs}");
    let xbox_tabs = sole_json(&acq(&base, &["tabs", "--realm", "xbox", "--json"]));
    assert_eq!(
        (&xbox_tabs["tabs"], &xbox_tabs["rows"]),
        (&json!(0), &json!([]))
    );

    // The journal: pc sends on the bare routes (two GETs on one route —
    // explicit and omitted are indistinguishable), xbox sends on the
    // suffixed ones with a HEAD probe before each route's first GET,
    // and no poe2 send at all.
    let all = sends(&base);
    let account = all
        .iter()
        .find_map(|(_, r, _)| r.strip_prefix("stash-list@").map(str::to_string))
        .expect("a pc listing was journaled");
    let on = |route: &str| -> Vec<(String, Option<u64>)> {
        all.iter()
            .filter(|(_, r, _)| r == route)
            .map(|(m, _, st)| (m.clone(), *st))
            .collect()
    };
    assert_eq!(
        on(&format!("stash-list@{account}")),
        vec![
            ("HEAD".to_string(), Some(204)),
            ("GET".to_string(), Some(200)),
            ("GET".to_string(), Some(200)),
        ]
    );
    assert_eq!(
        on(&format!("stash-list/xbox@{account}")),
        vec![
            ("HEAD".to_string(), Some(204)),
            ("GET".to_string(), Some(200))
        ]
    );
    assert_eq!(
        on(&format!("stash/xbox@{account}")),
        vec![
            ("HEAD".to_string(), Some(204)),
            ("GET".to_string(), Some(404))
        ]
    );
    assert!(
        all.iter().all(|(_, r, _)| !r.contains("poe2")),
        "a poe2 send was journaled: {all:?}"
    );

    let out = acq(&base, &["daemon", "stop"]);
    assert!(out.status.success(), "{out:?}");
}
