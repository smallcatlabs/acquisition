//! The daemon's integration tests, one binary (C122). They drive the
//! `acqd` cargo built for this run (`CARGO_BIN_EXE_acqd`) with std alone:
//! the daemon links no client, so the client's harness is not theirs, and
//! nothing here starts a daemon, opens a world or names a socket.

use std::path::PathBuf;
use std::process::Command;

/// `acqd` takes no arguments: its knobs are the environment, the one door
/// to them (`src/main.rs`). Any argument is refused with status 2 before
/// the environment is read, naming the frontends that start it and the
/// environment that configures it. The child's environment is the mock
/// and a store under this run's scratch alone, so a regression that
/// dropped the refusal would start a mock daemon there, never the real one.
#[test]
fn acqd_refuses_any_argument_and_names_its_door() {
    let scratch = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("acqd-door");
    for args in [&["--version"][..], &["serve", "--store", "x"][..]] {
        let out = Command::new(env!("CARGO_BIN_EXE_acqd"))
            .args(args)
            .env_clear()
            .env("ACQ_PROVIDER", "mock")
            .env("ACQ_STORE_DIR", scratch.join("store"))
            .env("ACQ_LOG_DIR", scratch.join("log"))
            .output()
            .expect("run acqd");
        let stderr = String::from_utf8_lossy(&out.stderr);
        let got = format!("(got {args:?})");
        assert_eq!(out.status.code(), Some(2), "{args:?}: {stderr}");
        assert!(
            out.stdout.is_empty(),
            "{args:?}: stdout {:?}",
            String::from_utf8_lossy(&out.stdout)
        );
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
