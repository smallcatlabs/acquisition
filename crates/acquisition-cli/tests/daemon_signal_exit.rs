//! C133: a terminating signal runs the exit `daemon stop` runs, and a
//! second one ends it at once. Pinned here through the real binaries over
//! the in-process mock, on the wiring alone: the drain's properties are
//! pinned in-process (`acquisition-daemon`'s `daemon/tests/exit.rs`).
//! Unix only: a detached Windows daemon receives no console signal, and
//! `TerminateProcess` cannot be caught. Each test signals the pid of the
//! daemon it started, never a process group (nextest's is the test's).

#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use serde_json::Value;

use crate::harness;

use harness::{acq, login_as_mock, scratch_tmp, sockets_under, sole_json, start_daemon};

/// The `-wal` beside a database file: its size, or `None` when absent.
fn wal_len(db: &Path) -> Option<u64> {
    let mut wal = db.as_os_str().to_owned();
    wal.push("-wal");
    std::fs::metadata(wal).ok().map(|m| m.len())
}

/// Send `signal` to `pid` alone.
fn signal(pid: u64, signal: &str) {
    let out = Command::new("kill")
        .args([&format!("-{signal}"), &pid.to_string()])
        .output()
        .expect("spawning kill");
    assert!(out.status.success(), "kill -{signal} {pid}: {out:?}");
}

/// The daemon's log path, from `daemon status`.
fn log_path(base: &Path) -> PathBuf {
    let status = sole_json(&acq(base, &["daemon", "status", "--json"]));
    PathBuf::from(status["log"].as_str().unwrap_or_else(|| panic!("{status}")))
}

fn log_text(log: &Path) -> String {
    std::fs::read_to_string(log).unwrap_or_default()
}

/// A mock `sleep` of `seconds`, detached, once the daemon reports it
/// running.
fn running_sleep(base: &Path, seconds: u32) -> String {
    let params = format!("{{\"seconds\":{seconds}}}");
    let out = acq(
        base,
        &["submit", "sleep", "--params", &params, "--detach", "--json"],
    );
    assert!(out.status.success(), "{out:?}");
    let id = sole_json(&out)["job_id"]
        .as_u64()
        .expect("a job id")
        .to_string();
    let deadline = Instant::now() + Duration::from_secs(120);
    loop {
        let status = sole_json(&acq(base, &["status", &id, "--json"]));
        if status["state"] == "running" {
            return id;
        }
        assert!(Instant::now() < deadline, "the job never ran: {status}");
        std::thread::sleep(Duration::from_millis(50));
    }
}

/// Wait until `log` holds `needle`.
fn wait_for_line(log: &Path, needle: &str, within: Duration) {
    let deadline = Instant::now() + within;
    while !log_text(log).contains(needle) {
        assert!(
            Instant::now() < deadline,
            "the daemon log never said {needle:?}:\n{}",
            log_text(log)
        );
        std::thread::sleep(Duration::from_millis(20));
    }
}

#[test]
fn c133_a_sigterm_drains_the_running_job_checkpoints_and_leaves() {
    let scratch = harness::scratch("sigterm");
    let base = scratch.0.clone();
    let (mut daemon, pid) = start_daemon(&base);
    let _login = login_as_mock(&base, &[]);
    // Facts in the account's file, so its WAL holds pages to checkpoint.
    let out = acq(&base, &["stashes", "--json"]);
    assert!(out.status.success(), "{out:?}");
    let log = log_path(&base);
    let id = running_sleep(&base, 2);

    signal(pid, "TERM");
    // The drain lets the two-second job land; the bound is 15 s. The wait
    // is a hang's, never the drain's measure: the log and the exit code
    // say how it ended (C137, the deadline's contract is its expiry
    // behaviour).
    daemon.wait_exit(Duration::from_secs(120));
    let code = daemon
        .0
        .try_wait()
        .expect("try_wait")
        .and_then(|s| s.code());
    let said = log_text(&log);
    assert_eq!(
        code,
        Some(0),
        "the daemon left by its exit, not by the signal:\n{said}"
    );
    assert!(
        said.contains("SIGTERM"),
        "the log names the signal:\n{said}"
    );
    assert!(
        said.contains(&format!("job {id} -> done")),
        "the running job landed before the exit:\n{said}"
    );
    assert!(
        said.contains("exit: nothing in flight"),
        "the drain saw nothing left:\n{said}"
    );
    assert!(
        sockets_under(&scratch_tmp(&base)).is_empty(),
        "the socket is removed: {:?}",
        sockets_under(&scratch_tmp(&base))
    );
    let mock = base.join("store").join("mock");
    for entry in std::fs::read_dir(&mock).unwrap().filter_map(Result::ok) {
        let path = entry.path();
        if path.extension().is_some_and(|x| x == "db") {
            assert_eq!(
                wal_len(&path).unwrap_or(0),
                0,
                "no WAL pages beside {} after the exit",
                path.display()
            );
        }
    }
    let status = sole_json(&acq(&base, &["daemon", "status", "--json"]));
    assert_eq!(status["running"], Value::Bool(false), "{status}");
}

#[test]
fn c133_a_second_sigterm_during_the_drain_exits_at_once() {
    let scratch = harness::scratch("sigterm2");
    let base = scratch.0.clone();
    let (mut daemon, pid) = start_daemon(&base);
    let log = log_path(&base);
    // A job that holds the drain past its bound.
    running_sleep(&base, 120);

    signal(pid, "TERM");
    wait_for_line(&log, "SIGTERM", Duration::from_secs(120));
    signal(pid, "TERM");
    // "At once" is told by the words, not the clock: the drain's own end
    // at its bound logs no second signal and checkpoints (C137, the
    // deadline's contract is its expiry behaviour).
    daemon.wait_exit(Duration::from_secs(120));
    let code = daemon
        .0
        .try_wait()
        .expect("try_wait")
        .and_then(|s| s.code());
    let said = log_text(&log);
    assert!(
        code.is_some_and(|c| c != 0),
        "a nonzero exit of the daemon's own, not the signal's: {code:?}\n{said}"
    );
    assert!(
        said.contains("a second signal"),
        "the log says the second signal ended it:\n{said}"
    );
    assert!(
        !said.contains("checkpointed on exit"),
        "no checkpoint on the way out:\n{said}"
    );
}
