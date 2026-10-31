//! C88 through the binaries: the real provider is the default and the
//! mock is opted into; the retired knob and an unknown provider word are
//! refused at start by `acq` and by `acqd`, naming the remedy; and a
//! real-mode daemon is started by a human — a use verb in real mode
//! without a terminal on stderr (every process here has its streams
//! piped, as an agent's shell, cron or ssh without a tty would) reports
//! absence and the remedy instead of spawning, while the mock spawns
//! freely from the same place. Nothing reaches GGG: the real-mode
//! commands here never get a daemon, by the rule under test, and the
//! world's root is checked to prove no spawn was attempted.

use crate::harness;

use harness::{acq, acqd, command, scratch, scratch_tmp, sockets_under, sole_json, text};

#[test]
fn c88_the_real_provider_is_the_default_and_the_retired_knob_is_refused() {
    let scratch = scratch("c88-knob");
    let base = &scratch.0;

    // Unset: ggg. The harness sets the mock; unset it to see the default.
    let out = command(base, &["version", "--json"])
        .env_remove("ACQ_PROVIDER")
        .output()
        .unwrap();
    assert!(out.status.success(), "{}", text(&out));
    assert_eq!(sole_json(&out)["provider"], "ggg");
    // Opted in: mock.
    let out = acq(base, &["version", "--json"]);
    assert_eq!(sole_json(&out)["provider"], "mock");

    // The retired knob, whatever its value, and an unknown word: refused
    // before any verb runs, in prose and as the total JSON error (C11).
    for (env, said) in [
        (("ACQ_GGG", "1"), "ACQ_GGG is no longer read"),
        (("ACQ_GGG", "0"), "ACQ_GGG is no longer read"),
        (("ACQ_PROVIDER", "Mock"), "names no provider"),
        // C156: a shell ACQ_SHELL does not take, with the same refusal
        (("ACQ_SHELL", "bash"), "names no shell"),
    ] {
        let out = command(base, &["version"])
            .env(env.0, env.1)
            .output()
            .unwrap();
        assert!(!out.status.success(), "{}", text(&out));
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.contains(said) && !stderr.contains("C88"),
            "{env:?}: {stderr}"
        );
        let out = command(base, &["version", "--json"])
            .env(env.0, env.1)
            .output()
            .unwrap();
        assert!(!out.status.success(), "{}", text(&out));
        let error = sole_json(&out)["error"].as_str().unwrap().to_string();
        assert!(error.contains(said), "{env:?}: {error}");
    }

    // The daemon refuses the same way, on its own stderr, before it makes
    // a world: no root, no log.
    let out = harness::daemon_command(&acqd(), base)
        .env("ACQ_GGG", "1")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2), "{}", text(&out));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.starts_with("acqd: ACQ_GGG is no longer read"),
        "{stderr}"
    );
    assert!(
        !base.join("store").exists(),
        "the refused daemon made a world"
    );
}

#[test]
fn c88_a_real_mode_daemon_is_started_from_a_terminal_and_the_mock_from_anywhere() {
    let scratch = scratch("c88-tty");
    let base = &scratch.0;

    // Real mode, no terminal on stderr (piped here): the use door observes
    // absence and names the remedy; nothing is spawned — the world's root
    // is never created (a spawn creates it first, C83).
    let out = command(base, &["profile"])
        .env("ACQ_PROVIDER", "ggg")
        .output()
        .unwrap();
    assert!(!out.status.success(), "{}", text(&out));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("started from a terminal")
            && !stderr.contains("C88")
            && stderr.contains("ACQ_PROVIDER=mock"),
        "{stderr}"
    );
    assert!(
        !base.join("store").exists(),
        "a real-mode use verb without a terminal created the world's root — a spawn"
    );
    let out = command(base, &["profile", "--json"])
        .env("ACQ_PROVIDER", "ggg")
        .output()
        .unwrap();
    assert!(!out.status.success(), "{}", text(&out));
    assert!(
        sole_json(&out)["error"]
            .as_str()
            .unwrap()
            .contains("started from a terminal"),
        "{}",
        text(&out)
    );

    // The mock, from the same non-terminal: spawned freely.
    let out = acq(
        base,
        &[
            "submit",
            "sleep",
            "--params",
            r#"{"seconds":0}"#,
            "--detach",
        ],
    );
    assert!(out.status.success(), "{}", text(&out));
    let status = sole_json(&acq(base, &["daemon", "status", "--json"]));
    assert_eq!(status["running"], true, "{status}");
    assert_eq!(status["provider"], "mock", "{status}");
    let out = acq(base, &["daemon", "stop", "--json"]);
    assert!(out.status.success(), "{}", text(&out));
}

/// A command that lazily starts a daemon hands it none of its own pipes:
/// a caller reading the command's output sees end-of-file when the
/// command exits, not when the daemon does. On Windows the daemon
/// inherited every inheritable handle of `acq`, the captured stdout among
/// them, so this read waited out the daemon's idle shutdown (found on the
/// first Windows run, 2026-10-14; `PLATFORMS-SLICE.md` at `b6197a82`, step 1).
#[test]
fn a_lazily_started_daemon_keeps_no_pipe_of_the_command_that_started_it() {
    let scratch = scratch("no-pipe");
    let base = &scratch.0;
    // The daemon this starts idles for 600 s, so a held pipe reads as
    // that idle exit: past the bound below, and past the test runner's
    // termination of a hung test (120 s in the default profile), so a
    // held pipe fails however slow the machine. The bound is a hang's,
    // never a measure of a healthy spawn (C137, the deadline's contract
    // is its expiry behaviour).
    let started = std::time::Instant::now();
    let out = command(
        base,
        &[
            "submit",
            "sleep",
            "--params",
            r#"{"seconds":0}"#,
            "--detach",
        ],
    )
    .env("ACQ_IDLE_SHUTDOWN", "600")
    .output()
    .expect("spawning acq");
    let took = started.elapsed();
    assert!(out.status.success(), "{}", text(&out));
    assert!(
        took < std::time::Duration::from_secs(60),
        "the output was read to its end only after {took:?}: the daemon held the pipe"
    );
    let status = sole_json(&acq(base, &["daemon", "status", "--json"]));
    assert_eq!(status["running"], true, "{status}");
    let out = acq(base, &["daemon", "stop", "--json"]);
    assert!(out.status.success(), "{}", text(&out));
}

/// C27 at the spawn: a Ctrl-C at the prompt that lazily started a daemon
/// cancels nothing. A terminal sends its SIGINT to the foreground job's
/// process group; here `acq` leads a group of its own, as a shell's job
/// does, and once it has started the daemon and returned, the group gets
/// the SIGINT. The daemon, which sets no handler and would die of it, is
/// still the one answering.
#[cfg(unix)]
#[test]
fn a_lazily_started_daemon_survives_a_ctrl_c_at_the_prompt_that_started_it() {
    use std::os::unix::process::CommandExt as _;
    use std::process::Command;
    let scratch = scratch("ctrl-c");
    let base = &scratch.0;
    let mut job = command(
        base,
        &[
            "submit",
            "sleep",
            "--params",
            r#"{"seconds":0}"#,
            "--detach",
        ],
    );
    job.process_group(0);
    let child = job.spawn().expect("spawning acq");
    let group = child.id();
    let out = child.wait_with_output().expect("acq ran");
    assert!(out.status.success(), "{}", text(&out));
    let before = sole_json(&acq(base, &["daemon", "status", "--json"]));
    assert_eq!(before["running"], true, "{before}");
    assert!(before["pid"].is_u64(), "{before}");

    // The terminal's Ctrl-C, to the job's group. Its own status is not
    // judged: with the daemon elsewhere the group is empty by now.
    let _ = Command::new("kill")
        .args(["-s", "INT", "--", &format!("-{group}")])
        .status();
    std::thread::sleep(std::time::Duration::from_millis(500));

    let after = sole_json(&acq(base, &["daemon", "status", "--json"]));
    assert_eq!(
        after["running"], true,
        "the daemon died of a SIGINT sent to the group of the command that started it: {after}"
    );
    assert_eq!(after["pid"], before["pid"], "{before} then {after}");
    let out = acq(base, &["daemon", "stop", "--json"]);
    assert!(out.status.success(), "{}", text(&out));
}

/// `daemon stop` returns once the daemon is gone, not while it exits: its
/// endpoint is already removed, so the next command finds no daemon. The
/// daemon answers `stopping`, then checkpoints and removes its endpoint;
/// a command in that window reached a dying daemon and read "daemon
/// closed the connection" (first seen on CI's Windows runner, 2026-10-14).
#[test]
fn a_stopped_daemon_is_gone_when_the_stop_returns() {
    let scratch = scratch("stop-gone");
    let base = &scratch.0;
    let out = acq(
        base,
        &[
            "submit",
            "sleep",
            "--params",
            r#"{"seconds":0}"#,
            "--detach",
        ],
    );
    assert!(out.status.success(), "{}", text(&out));
    assert_eq!(
        sockets_under(&scratch_tmp(base)).len(),
        1,
        "a daemon started"
    );
    let out = acq(base, &["daemon", "stop", "--json"]);
    assert!(out.status.success(), "{}", text(&out));
    assert!(
        sockets_under(&scratch_tmp(base)).is_empty(),
        "the stop returned with the daemon's endpoint still there"
    );
    let report = sole_json(&acq(base, &["status", "999", "--json"]));
    assert!(
        report["error"].as_str().unwrap().contains("not running"),
        "{report}"
    );
}
