//! The daemon's knobs as the process environment sets them, read through
//! the daemon's own readers. A binary of its own (C122): it writes the
//! process environment, which would leak into every test beside it.
//!
//! Two knobs guard what the invariants protect, and each was a surviving
//! mutant of the weekly run until this test (#218). `ACQ_NO_KEYRING`
//! keeps every mock session and test daemon out of the OS keyring, the
//! refresh token's home (invariant 5): with it set, a save, a load and a
//! clear each refuse without reaching the keyring. `ACQ_TRIPWIRE` and
//! `ACQ_MAX_SENDS` arm a live run's rails (invariant 3's halt among
//! them): the words each takes, and a value neither takes, which is
//! warned of and leaves the rail off.
//!
//! One test, since the variables are the process's: each step sets what
//! it reads, and nothing else in this binary reads them.

use acquisition_daemon::auth::{self, NotCleared};
use acquisition_daemon::rails::RailsConfig;
use std::path::Path;

/// Sets `name` to `value`, or removes it.
fn knob(name: &str, value: Option<&str>) {
    // SAFETY: this binary's one test is its one thread that touches the
    // environment, and it reads only what it has just set.
    unsafe {
        match value {
            Some(value) => std::env::set_var(name, value),
            None => std::env::remove_var(name),
        }
    }
}

/// The rails a start would build with `ACQ_TRIPWIRE` and `ACQ_MAX_SENDS`
/// as given.
fn rails_with(tripwire: Option<&str>, max_sends: Option<&str>) -> RailsConfig {
    knob("ACQ_TRIPWIRE", tripwire);
    knob("ACQ_MAX_SENDS", max_sends);
    knob("ACQ_JOURNAL", None);
    RailsConfig::from_env(Path::new("world/rails.json"), Path::new("logs/journal"))
}

#[test]
fn the_daemons_knobs_read_as_the_environment_sets_them() {
    // The keyring, off: nothing is tried, and nothing is said done.
    knob("ACQ_NO_KEYRING", Some("1"));
    let off = auth::keyring_disabled().expect("the keyring is off");
    assert!(off.contains("ACQ_NO_KEYRING"), "{off}");
    let service = "acquisition-knobs-test";
    let saved = auth::keyring_save(service, &auth::Secret::from("rt-knob"), "knob-user");
    assert_eq!(saved, Err(off.to_string()));
    assert_eq!(
        auth::keyring_load(service, "knob-user").map(|found| found.is_some()),
        Err(off.to_string())
    );
    assert_eq!(
        auth::keyring_clear(service, "knob-user"),
        Err(NotCleared::Off(off.to_string()))
    );
    knob("ACQ_NO_KEYRING", None);
    assert_eq!(auth::keyring_disabled(), None, "the keyring is on unset");

    // The tripwire: the words that arm it, the words that leave it off,
    // and a value it does not take, warned of and off.
    for on in ["1", "true", "yes", "on", " on "] {
        let rails = rails_with(Some(on), None);
        assert!(rails.tripwire, "ACQ_TRIPWIRE={on:?}");
        assert_eq!(rails.warnings, Vec::<String>::new(), "ACQ_TRIPWIRE={on:?}");
    }
    for off in ["", "0", "false", "no", "off"] {
        let rails = rails_with(Some(off), None);
        assert!(!rails.tripwire, "ACQ_TRIPWIRE={off:?}");
        assert_eq!(rails.warnings, Vec::<String>::new(), "ACQ_TRIPWIRE={off:?}");
    }
    let unread = rails_with(Some("maybe"), None);
    assert!(!unread.tripwire);
    assert_eq!(
        unread.warnings,
        ["ACQ_TRIPWIRE=\"maybe\" not understood; tripwire is OFF"]
    );
    let unset = rails_with(None, None);
    assert!(!unset.tripwire);

    // The ceiling: a number, or a warning and none.
    assert_eq!(rails_with(None, Some(" 5 ")).max_sends, Some(5));
    let unread = rails_with(None, Some("ten"));
    assert_eq!(unread.max_sends, None);
    assert_eq!(
        unread.warnings,
        ["ACQ_MAX_SENDS=\"ten\" is not a number; no ceiling"]
    );

    // The world's state path is the one the start gives, and the journal
    // the default when `ACQ_JOURNAL` says nothing.
    let rails = rails_with(None, None);
    assert_eq!(
        rails.state_path.as_deref(),
        Some(Path::new("world/rails.json"))
    );
    assert_eq!(
        rails.journal_path.as_deref(),
        Some(Path::new("logs/journal"))
    );
}
