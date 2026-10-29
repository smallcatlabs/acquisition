//! The guard a harness login returns (#115): it stops the daemon that
//! login started, and only that one. A failing test's daemon is pinned
//! stopped by `readme_tour.rs`'s
//! `a_daemon_the_recipe_starts_is_stopped_when_its_test_fails`, whose
//! guard is the recipe login's; this file pins the other half.

use crate::harness;

use harness::{acq, login, sole_json, start_daemon};

/// A login against a daemon the test started itself gets a guard that
/// stops nothing: the daemon is the test's `Proc`, ended by its own drop,
/// and a guard that stopped it would end it under the test's feet.
#[test]
fn a_login_against_a_daemon_the_test_owns_gets_a_guard_that_stops_nothing() {
    let scratch = harness::scratch("owned");
    let base = scratch.0.clone();
    let (_daemon, pid) = start_daemon(&base);
    drop(login(&base, "Alice"));
    let status = sole_json(&acq(&base, &["daemon", "status", "--json"]));
    assert_eq!(
        (&status["running"], &status["pid"]),
        (&serde_json::json!(true), &serde_json::json!(pid)),
        "the test's own daemon was stopped by a login's guard: {status}"
    );
}
