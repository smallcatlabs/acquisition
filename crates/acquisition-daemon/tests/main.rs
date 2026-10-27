//! The daemon's integration tests, one binary (C122). They drive the
//! `acqd` cargo built for this run (`CARGO_BIN_EXE_acqd`) with std and
//! the crate's `tempfile` alone: the daemon links no client, so the
//! client's harness is not theirs, and nothing here means to start a
//! daemon, open a world or name a socket.

use std::io::Read;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

/// How long a refusal may take before the test calls it gone. It guards a
/// regression, a refusal dropped and a daemon started in its place, so
/// that `cargo test` does not wait out the daemon's idle hour; it is not
/// what the test asserts (C137, a deadline's contract is its expiry
/// behaviour): a refusal takes milliseconds.
const BOUND: Duration = Duration::from_secs(60);

/// The child, killed and reaped when the test ends however it ends.
struct Guard(Child);

impl Drop for Guard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// `acqd` takes no arguments: its knobs are the environment, the one door
/// to them (`src/main.rs`). Any argument is refused with status 2 before
/// the environment is read, naming the frontends that start it and the
/// environment that configures it. The child's environment is the mock
/// and a store and log under the test's own folder alone, so a regression
/// that dropped the refusal would start a mock daemon there, never the
/// real one, and the guard stops it.
#[test]
fn acqd_refuses_any_argument_and_names_its_door() {
    let scratch = tempfile::tempdir().expect("the test's folder");
    for args in [&["--version"][..], &["serve", "--store", "x"][..]] {
        let child = Command::new(env!("CARGO_BIN_EXE_acqd"))
            .args(args)
            .env_clear()
            .env("ACQ_PROVIDER", "mock")
            .env("ACQ_STORE_DIR", scratch.path().join("store"))
            .env("ACQ_LOG_DIR", scratch.path().join("log"))
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("start acqd");
        let mut child = Guard(child);
        let status = wait_bounded(&mut child.0, args);
        let mut stdout = String::new();
        let mut stderr = String::new();
        child
            .0
            .stdout
            .take()
            .expect("stdout")
            .read_to_string(&mut stdout)
            .expect("read stdout");
        child
            .0
            .stderr
            .take()
            .expect("stderr")
            .read_to_string(&mut stderr)
            .expect("read stderr");
        assert_eq!(status.code(), Some(2), "{args:?}: {stderr}");
        assert!(stdout.is_empty(), "{args:?}: stdout {stdout:?}");
        let got = format!("(got {args:?})");
        for said in [
            "takes no arguments",
            got.as_str(),
            "started by the frontend beside it (acq, acq-mcp)",
            "configured by the environment",
        ] {
            assert!(stderr.contains(said), "{args:?}: {said:?} not in {stderr}");
        }
    }
}

/// The child's exit, or the test's failure in words once [`BOUND`] has
/// passed (the guard then kills it).
fn wait_bounded(child: &mut Child, args: &[&str]) -> ExitStatus {
    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait().expect("poll acqd") {
            return status;
        }
        assert!(
            start.elapsed() < BOUND,
            "acqd {args:?} was still running after {BOUND:?}: it no longer refuses arguments, \
             and started as a daemon instead"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
}
