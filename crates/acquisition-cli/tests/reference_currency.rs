//! Process-level pin of `acq reference currency` (C68: reference data is
//! enumerable by every surface built and cited by version wherever used;
//! C11: `--json` is the contract, a failure is `{"error": …}` on stdout
//! with exit 1). No store, no daemon: the socket points into an
//! empty temp dir and `ACQ_NO_SPAWN=1` makes any contact an error, so
//! the command must answer from the binary alone.

use crate::harness;

use harness::{Transcript, golden, offline_acq as acq, sole_json};

#[test]
fn c68_the_table_is_enumerable_by_version_with_no_store_and_no_daemon() {
    let tmp = harness::scratch("reference");
    let out = acq(&tmp.0, &["--json", "reference", "currency"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let v = sole_json(&out);
    assert_eq!(v["version"], 1);
    let rows = v["currency"].as_array().unwrap();
    assert_eq!(rows.len(), 42);
    assert!(
        rows.iter()
            .all(|r| !r["evidence"].as_array().unwrap().is_empty())
    );
    assert_eq!(
        rows.iter().filter(|r| r.get("retired").is_some()).count(),
        3
    );

    let mut t = Transcript::offline(&tmp.0);
    let text = t.run(
        &["reference", "currency"],
        &[
            "39 currencies the game writes, 3 retired",
            "retired 2026-09-04",
        ],
    );
    assert!(text.status.success());
    let stdout = String::from_utf8(text.stdout).unwrap();
    assert!(
        stdout
            .lines()
            .any(|l| l.contains("exalted") && l.contains("also exa"))
    );
    assert!(
        !stdout.contains("game:2026"),
        "evidence is the audit view: {stdout}"
    );

    golden(file!(), "the_table", &t);

    let audit = acq(&tmp.0, &["reference", "currency", "--expand"]);
    let stdout = String::from_utf8(audit.stdout).unwrap();
    assert!(
        stdout.contains("game:2026-09-04 note `~price 999 chaos`"),
        "{stdout}"
    );
    assert!(stdout.contains("sources:"), "{stdout}");
}

#[test]
fn c68_one_word_resolves_exactly_or_fails_naming_the_version() {
    let tmp = harness::scratch("reference");
    let out = acq(&tmp.0, &["--json", "reference", "currency", "exa"]);
    assert!(out.status.success());
    let v = sole_json(&out);
    assert_eq!(v["word"], "exa");
    assert_eq!(v["version"], 1);
    assert_eq!(v["currency"]["tag"], "exalted");
    assert_eq!(v["currency"]["emit"], "exalted");
    let mut t = Transcript::offline(&tmp.0);

    t.run(
        &["reference", "currency", "exa"],
        &["exa is an alias of exalted: currency table v1"],
    );

    let miss = acq(&tmp.0, &["--json", "reference", "currency", "Chaos"]);
    assert_eq!(miss.status.code(), Some(1));
    let v = sole_json(&miss);
    let err = v["error"].as_str().unwrap();
    assert!(
        err.contains("\"Chaos\"") && err.contains("v1") && err.contains("acq reference currency"),
        "{err}"
    );

    let miss = t.run(
        &["reference", "currency", "Chaos"],
        &["not a currency word"],
    );
    assert_eq!(miss.status.code(), Some(1));
    assert!(miss.stdout.is_empty());
    golden(file!(), "one_word_resolved_or_refused", &t);
}
