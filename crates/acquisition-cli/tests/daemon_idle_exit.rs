//! C3 with C137: a daemon exits when idle, and the idle knob at zero
//! forces that exit. Pinned here through the real binary over the
//! in-process mock, on the wiring alone, beside the signal's
//! (`daemon_signal_exit.rs`): the lifecycle's order is the table of
//! `acquisition-daemon`'s `daemon/tests/exit.rs`, and the driver's
//! deadline at zero its `c137_` tests. The test holds no connection, so
//! nothing but the restore's landing holds the daemon; the wait is a
//! hang's, never the exit's measure.

use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::harness;

use harness::{acqd, daemon_command, scratch, scratch_tmp, sockets_under, spawn_drained};

/// The daemon log under `dir`, wherever the world's id put it.
fn daemon_log(dir: &Path) -> Option<PathBuf> {
    for entry in std::fs::read_dir(dir).ok()?.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            if let Some(found) = daemon_log(&path) {
                return Some(found);
            }
        } else if path.file_name().is_some_and(|n| n == "daemon.log") {
            return Some(path);
        }
    }
    None
}

#[test]
fn c137_a_daemon_under_the_idle_knob_at_zero_exits_on_its_own() {
    let scratch = scratch("idle-zero");
    let base = scratch.0.clone();
    let acqd = acqd();
    let mut cmd = daemon_command(&acqd, &base);
    cmd.env("ACQ_IDLE_SHUTDOWN", "0");
    let (mut daemon, _said) = spawn_drained(cmd, &acqd);

    daemon.wait_exit(Duration::from_secs(120));
    let code = daemon
        .0
        .try_wait()
        .expect("try_wait")
        .and_then(|s| s.code());
    let log = daemon_log(&base.join("logs")).expect("the daemon wrote its log");
    let said = std::fs::read_to_string(&log).unwrap_or_default();
    assert_eq!(code, Some(0), "the daemon left by its exit:\n{said}");
    assert!(
        said.contains("idle timeout; exiting"),
        "the log names the idle exit:\n{said}"
    );
    assert!(
        said.contains("checkpointed on exit"),
        "the exit's leave checkpointed:\n{said}"
    );
    assert!(
        sockets_under(&scratch_tmp(&base)).is_empty(),
        "the socket is removed: {:?}",
        sockets_under(&scratch_tmp(&base))
    );
}
