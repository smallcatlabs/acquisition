//! The world (C83) through the binaries: the world lock and the
//! real-mode lock, each staged with a second daemon that refuses to start
//! and names the holder; a client on another world refusing the daemon
//! and never replacing it (C83, C10); the rails state moving into the
//! world once so a persisted trip survives (the tripwire's file used to
//! sit in a directory macOS clears at reboot); and the diagnostics under
//! the log directory, bounded by rotation.
//!
//! `ACQ_PROVIDER=ggg` appears here on two daemons the real-mode lock test
//! starts directly — the owner asked for that lock's process test
//! (2026-09-11) — and on the commands that observe and stop them. Nothing
//! reaches GGG: the daemons start with no session (`ACQ_NO_KEYRING=1`,
//! an empty scratch store, so nothing to restore), nothing is submitted
//! to them, and belt and braces, `ACQ_TRIPWIRE=1 ACQ_MAX_SENDS=0` halts
//! every send before it could happen. The real-mode lock is per OS user
//! at a fixed path in the user's own data directory (C132), which no
//! variable the harness moves can relocate, so this test takes the
//! owner's real lock: it contends with any real-mode daemon of his — a
//! live driver's, or the one a bare `acq` spawns and leaves idling — and
//! with this test in a second gate, which is why `AGENTS.md` forbids a
//! bare `acq` or a second gate while the gate runs; the failure names the
//! holder's pid. Both of its daemons are killed when it ends, however it
//! ends, so it never leaves the lock held.

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

use serde_json::Value;

use crate::harness;

use harness::{Proc, TEMP_VARS, acqd, scratch, scratch_tmp, sockets_under, sole_json, text};

/// `acq` under one isolation: a store root (the world, whose socket
/// derives from it, C83) and a log directory the caller names under
/// `base`.
fn command(base: &Path, store: &str, args: &[&str]) -> Command {
    harness::command_in(base, store, args)
}

fn acq(base: &Path, store: &str, args: &[&str]) -> Output {
    command(base, store, args).output().expect("spawning acq")
}

/// The daemon's command under the same isolation as `command`, its
/// stderr piped so a refusal can be read (a lazy spawn's would go to the
/// log; a driver captures stderr as this does).
fn daemon_command(base: &Path, store: &str) -> Command {
    let mut cmd = Command::new(acqd());
    harness::isolate_in(&mut cmd, base, store);
    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    cmd
}

/// Start a daemon and wait until `acq daemon status` (under `env`) sees
/// it running; returns it with its pid.
fn start_daemon(base: &Path, store: &str, env: &[(&str, &str)]) -> (Proc, u64) {
    let mut cmd = daemon_command(base, store);
    for (k, v) in env {
        cmd.env(k, v);
    }
    let child = cmd
        .spawn()
        .unwrap_or_else(|e| panic!("spawning {}: {e}", acqd().display()));
    let daemon = Proc(child, acqd());
    let deadline = Instant::now() + Duration::from_secs(120);
    loop {
        let mut status = command(base, store, &["daemon", "status", "--json"]);
        for (k, v) in env {
            status.env(k, v);
        }
        let out = status.output().unwrap();
        let status = sole_json(&out);
        if let Some(pid) = status["pid"].as_u64() {
            assert_eq!(status["compatible"], true, "{status}");
            return (daemon, pid);
        }
        assert!(
            Instant::now() < deadline,
            "the daemon ({}) did not come up",
            acqd().display()
        );
        std::thread::sleep(Duration::from_millis(50));
    }
}

/// Start a daemon that is expected to refuse: wait for it to exit and
/// return its exit status and stderr.
fn refused_daemon(mut cmd: Command) -> (std::process::ExitStatus, String) {
    let child = cmd.spawn().expect("spawning acqd");
    let deadline = Instant::now() + Duration::from_secs(120);
    let mut daemon = Proc(child, acqd());
    loop {
        if let Some(status) = daemon.0.try_wait().unwrap() {
            let mut stderr = String::new();
            if let Some(mut pipe) = daemon.0.stderr.take() {
                use std::io::Read as _;
                let _ = pipe.read_to_string(&mut stderr);
            }
            return (status, stderr);
        }
        assert!(
            Instant::now() < deadline,
            "the second daemon did not exit: it should have refused"
        );
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn daemon_log(base: &Path, store: &str, provider: &str) -> PathBuf {
    let world = acquisition_store::world::World::observe_at(&base.join(store)).expect("the world");
    base.join("logs")
        .join(world.id())
        .join(provider)
        .join("daemon.log")
}

/// `acq` under the isolation plus `env`, applied last.
fn acq_in(base: &Path, store: &str, args: &[&str], env: &[(&str, &str)]) -> Output {
    let mut cmd = command(base, store, args);
    for (k, v) in env {
        cmd.env(k, v);
    }
    cmd.output().expect("spawning acq")
}

/// Wait until `acq daemon status` under `env` says nothing is running.
fn wait_stopped(base: &Path, store: &str, env: &[(&str, &str)]) {
    let deadline = Instant::now() + Duration::from_secs(120);
    loop {
        let status = sole_json(&acq_in(base, store, &["daemon", "status", "--json"], env));
        if status["running"] == Value::Bool(false) {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "the daemon did not exit: {status}"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
}

/// C83, C6: a second daemon on the same world refuses to start, naming
/// the holder by pid and the lock by path, in its stderr and in the
/// world's log; the first daemon is untouched.
#[test]
fn c83_a_second_daemon_on_the_same_world_refuses_naming_the_holder() {
    let scratch = scratch("lock");
    let base = &scratch.0;
    let (_first, pid) = start_daemon(base, "store", &[]);
    let lock = base
        .join("store")
        .canonicalize()
        .unwrap()
        .join("daemon.lock");
    assert!(lock.is_file(), "the world lock file exists");

    // The incumbent's log, grown past the cap while it runs: a contender
    // must not rotate it from under the incumbent (review 2026-09-11 —
    // the locks come before the log is touched).
    let log_path = daemon_log(base, "store", "mock");
    // A write handle: Windows lets an append-only one change no length.
    std::fs::OpenOptions::new()
        .write(true)
        .open(&log_path)
        .unwrap()
        .set_len(1u64 << 30)
        .unwrap();

    // Another socket, the same store root: refused on the lock, not on
    // the socket.
    let (status, stderr) = refused_daemon(daemon_command(base, "store"));
    eprintln!("the world lock's refusal, verbatim:\n{stderr}");
    assert!(!status.success(), "the second daemon started: {stderr}");
    assert!(
        !PathBuf::from(format!("{}.1", log_path.display())).exists(),
        "the contender rotated the incumbent's log"
    );
    assert!(
        std::fs::metadata(&log_path).unwrap().len() >= 1u64 << 30,
        "the incumbent's log was replaced"
    );
    assert!(
        stderr.contains("another daemon holds this world")
            && stderr.contains(&format!("pid {pid}"))
            && stderr.contains("daemon.lock")
            && stderr.contains("one daemon per world"),
        "{stderr}"
    );
    // The refusal is at the end of a file that is mostly a hole: read
    // the tail only (review 2026-09-11 — a gate must not read a gibibyte
    // into memory).
    let log = {
        use std::io::{Read as _, Seek as _, SeekFrom};
        let mut file = std::fs::File::open(&log_path).unwrap();
        let len = file.metadata().unwrap().len();
        file.seek(SeekFrom::Start(len.saturating_sub(4096)))
            .unwrap();
        let mut tail = Vec::new();
        file.read_to_end(&mut tail).unwrap();
        String::from_utf8_lossy(&tail).into_owned()
    };
    assert!(
        log.contains("STARTUP: another daemon holds this world")
            && log.contains(&format!("pid {pid}")),
        "the refusal reached the incumbent's log (appended, not rotated): {}",
        log.trim_start_matches('\0')
    );
    // A contender under another log directory whose world/provider
    // directory exists but holds no log: refused the same way, and it
    // creates no file there (round 21).
    let other_logs = base.join("other-logs");
    let other_log = other_logs
        .join(
            acquisition_store::world::World::observe_at(&base.join("store"))
                .unwrap()
                .id(),
        )
        .join("mock")
        .join("daemon.log");
    std::fs::create_dir_all(other_log.parent().unwrap()).unwrap();
    let mut contender = daemon_command(base, "store");
    contender.env("ACQ_LOG_DIR", &other_logs);
    let (status, stderr) = refused_daemon(contender);
    assert!(
        !status.success() && stderr.contains("another daemon holds this world"),
        "{stderr}"
    );
    assert!(
        !other_log.exists(),
        "a refused contender created a log under its own directory"
    );

    // The first daemon still answers, and it is the same one.
    let out = acq(base, "store", &["daemon", "status", "--json"]);
    let status = sole_json(&out);
    assert_eq!(status["pid"], pid, "{status}");
    assert_eq!(
        status["world"],
        acquisition_store::world::canonical(&base.join("store"))
            .unwrap()
            .display()
            .to_string(),
        "{status}"
    );
    let out = acq(base, "store", &["daemon", "stop", "--json"]);
    assert!(out.status.success(), "{out:?}");
}

/// C83, C31, C132: a second real-mode daemon for this OS user refuses to
/// start whatever its world and whatever its temp, runtime and home
/// variables — `TMPDIR`, `TEMP`, `TMP`, `XDG_RUNTIME_DIR`, `HOME`,
/// `XDG_DATA_HOME`, `USERPROFILE` and `LOCALAPPDATA` all moved from the
/// first's, so a lock derived from any of them would be a second lock —
/// naming the holder and the real-mode lock; a mock daemon on
/// that other world starts, since the lock is real mode's. Nothing reaches
/// GGG (module doc), and the first daemon has sent nothing after the
/// refusal.
#[test]
fn c83_c31_a_second_real_mode_daemon_for_this_user_refuses_naming_the_holder() {
    let scratch = scratch("real");
    let base = &scratch.0;
    let real = [
        ("ACQ_PROVIDER", "ggg"),
        ("ACQ_TRIPWIRE", "1"),
        ("ACQ_MAX_SENDS", "0"),
    ];
    let (_first, pid) = start_daemon(base, "one", &real);
    let out = command(base, "one", &["daemon", "status", "--json"])
        .env("ACQ_PROVIDER", "ggg")
        .output()
        .unwrap();
    let status = sole_json(&out);
    assert_eq!(status["provider"], "ggg", "{status}");
    // Belt and braces (module doc): the ceiling is zero, so `Rails::halted`
    // refuses every send before it could happen (`rails.rs`); the status
    // document reports a halt only once a send has tripped one, so it
    // shows the ceiling, not a cause.
    assert_eq!(status["rails"]["max_sends"], 0, "{status}");
    assert_eq!(status["rails"]["tripwire_enabled"], true, "{status}");
    assert_eq!(status["rails"]["sends"], 0, "{status}");

    // Another world, another socket, real mode, and every variable a temp,
    // runtime, home or data directory is read from moved elsewhere (C132):
    // refused on the real-mode lock all the same, and the refusal names
    // it. The moved home exists, so a lock read from it would be made
    // there and the contender would start.
    // No longer than the harness's own `tmp`, so the socket a contender
    // would bind there stays under the cap (`harness::socket_fits_under`).
    let elsewhere = base.join("t2");
    std::fs::create_dir_all(&elsewhere).unwrap();
    assert_ne!(elsewhere, scratch_tmp(base));
    let mut second = daemon_command(base, "two");
    for (k, v) in &real {
        second.env(k, v);
    }
    for k in TEMP_VARS
        .into_iter()
        .chain(["HOME", "XDG_DATA_HOME", "USERPROFILE", "LOCALAPPDATA"])
    {
        second.env(k, &elsewhere);
    }
    let (status, stderr) = refused_daemon(second);
    eprintln!("the real-mode lock's refusal, verbatim:\n{stderr}");
    assert!(
        !status.success(),
        "the second real-mode daemon started: {stderr}"
    );
    assert!(
        stderr.contains("another real-mode daemon is running for this OS user")
            && stderr.contains(&format!("pid {pid}"))
            && stderr.contains("ggg.lock")
            && stderr.contains("one real-mode daemon per OS user"),
        "{stderr}"
    );
    // The lock it met is not under its own moved directories: the path
    // derives from the OS user's recorded home, not from them.
    assert!(
        !stderr.contains(&elsewhere.display().to_string()),
        "the refusal named a lock under the moved directories: {stderr}"
    );
    // No send: the first daemon's count is still zero.
    let out = command(base, "one", &["daemon", "status", "--json"])
        .env("ACQ_PROVIDER", "ggg")
        .output()
        .unwrap();
    let status = sole_json(&out);
    assert_eq!(status["pid"], pid, "{status}");
    assert_eq!(status["rails"]["sends"], 0, "{status}");
    // The refused contender held no lock, so it made nothing in its
    // world's log directory (review 2026-09-11): its refusal is on stderr
    // alone — there was no log to append to yet.
    assert!(
        !daemon_log(base, "two", "ggg").exists(),
        "a refused contender created its world's log"
    );

    // The same other world in mock mode starts: the lock is real mode's.
    let (_mock, mock_pid) = start_daemon(base, "two", &[]);
    assert_ne!(mock_pid, pid);
    let out = acq(base, "two", &["daemon", "status", "--json"]);
    assert_eq!(sole_json(&out)["provider"], "mock");

    let out = command(base, "one", &["daemon", "stop", "--json"])
        .env("ACQ_PROVIDER", "ggg")
        .output()
        .unwrap();
    assert!(out.status.success(), "{out:?}");
    let out = acq(base, "two", &["daemon", "stop", "--json"]);
    assert!(out.status.success(), "{out:?}");
}

/// C83, C10, through the binaries: the socket derives from the world,
/// so a shell on another world does not find this daemon at all — its
/// status says not running, a reading verb says so, and a job command
/// starts a daemon of its own on its own socket rather than replacing
/// this one; two worlds run two daemons at once, each shell seeing its
/// own; a shell whose root does not exist has no world, no socket, and
/// creates none by observing; `daemon stop` stops only the shell's own.
#[test]
fn c83_a_shell_on_another_world_has_its_own_rendezvous_and_never_replaces_this_daemon() {
    let scratch = scratch("other");
    let base = &scratch.0;
    std::fs::create_dir_all(base.join("elsewhere")).unwrap();
    let (_daemon, pid) = start_daemon(base, "store", &[]);
    let home = sole_json(&acq(base, "store", &["daemon", "status", "--json"]));
    let home_socket = PathBuf::from(home["socket"].as_str().unwrap());
    assert_eq!(sockets_under(&scratch_tmp(base)), vec![home_socket.clone()]);

    // From another world: not running, and a reading verb says so — with
    // nothing on disk there to read (C45: a root, no ledger).
    let absent = serde_json::json!({ "running": false, "persisted": null });
    let out = acq(base, "elsewhere", &["daemon", "status", "--json"]);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(sole_json(&out), absent);
    let out = acq(base, "elsewhere", &["jobs", "--json"]);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(sole_json(&out), absent);
    assert_eq!(
        sockets_under(&scratch_tmp(base)),
        vec![home_socket.clone()],
        "a reading verb spawned a daemon"
    );

    // A job command from there starts that world's daemon — on its own
    // socket, beside this one, which it never touches. (`profile` fails
    // on the login, not on the door; the daemon it spawned stays up.)
    let out = acq(base, "elsewhere", &["profile", "--json"]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let msg = sole_json(&out)["error"].as_str().unwrap().to_string();
    assert!(
        !msg.contains("not running") && !msg.contains("another world"),
        "the job command did not reach a daemon of its own: {msg}"
    );
    let out = acq(base, "elsewhere", &["daemon", "status", "--json"]);
    let other = sole_json(&out);
    let other_pid = other["pid"].as_u64().expect("a daemon for the other world");
    assert_ne!(other_pid, pid, "{other}");
    assert_eq!(other["compatible"], true, "{other}");
    let other_socket = PathBuf::from(other["socket"].as_str().unwrap());
    assert_ne!(other_socket, home_socket, "another world, another socket");
    let mut both = vec![home_socket.clone(), other_socket.clone()];
    both.sort();
    assert_eq!(
        sockets_under(&scratch_tmp(base)),
        both,
        "two worlds, two sockets"
    );
    assert_eq!(
        other["world"],
        acquisition_store::world::canonical(&base.join("elsewhere"))
            .unwrap()
            .display()
            .to_string(),
        "{other}"
    );
    let out = acq(base, "store", &["daemon", "status", "--json"]);
    let status = sole_json(&out);
    assert_eq!(status["pid"], pid, "the daemon was replaced: {status}");
    assert_eq!(status["compatible"], true, "{status}");
    assert_eq!(
        status["socket"],
        home_socket.display().to_string(),
        "{status}"
    );

    // No root: no world, no socket, nothing created.
    let out = acq(base, "nowhere", &["daemon", "status", "--json"]);
    assert_eq!(
        sole_json(&out),
        serde_json::json!({ "running": false, "persisted": null })
    );
    assert!(
        !base.join("nowhere").exists(),
        "observation created the root"
    );

    // Each shell stops its own and only its own.
    let out = acq(base, "elsewhere", &["daemon", "stop", "--json"]);
    assert!(out.status.success(), "{out:?}");
    let stopped = sole_json(&out);
    assert_eq!(stopped["pid"], other_pid, "{stopped}");
    assert_eq!(stopped["compatible"], true, "{stopped}");
    wait_stopped(base, "elsewhere", &[]);
    let out = acq(base, "store", &["daemon", "status", "--json"]);
    assert_eq!(
        sole_json(&out)["pid"],
        pid,
        "the other world's stop reached this daemon"
    );
    let out = acq(base, "store", &["daemon", "stop", "--json"]);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(sole_json(&out)["pid"], pid);
}

/// C83: the rails state persists in the world — a trip written there
/// halts the next daemon on that root; `reset-tripwire` with no daemon
/// clears the world's file, clears an empty directory in its place (the
/// daemon refuses to start over one, naming the verb) and names a
/// non-empty one for the hand.
#[test]
fn c83_the_rails_state_lives_in_the_world_and_reset_tripwire_clears_it() {
    let scratch = scratch("rails");
    let base = &scratch.0;
    let env = [("ACQ_TRIPWIRE", "1")];
    let current = base.join("store").join("mock").join("rails.json");
    std::fs::create_dir_all(current.parent().unwrap()).unwrap();
    std::fs::write(
        &current,
        r#"{"tripped":"429 on GET /stash/Standard (staged trip)"}"#,
    )
    .unwrap();

    let (_daemon, _pid) = start_daemon(base, "store", &env);
    let out = acq_in(base, "store", &["daemon", "status", "--json"], &env);
    let status = sole_json(&out);
    assert_eq!(status["rails"]["tripwire_enabled"], true, "{status}");
    assert!(
        status["rails"]["halted"]
            .as_str()
            .is_some_and(|h| h.contains("staged trip")),
        "the trip in the world was not honoured: {status}"
    );
    let out = acq_in(base, "store", &["daemon", "stop", "--json"], &env);
    assert!(out.status.success(), "{out:?}");
    wait_stopped(base, "store", &env);

    // With no daemon, the reset clears the world's file so the next
    // daemon starts clear.
    let out = acq_in(base, "store", &["daemon", "reset-tripwire", "--json"], &env);
    assert!(out.status.success(), "{out:?}");
    let cleared = sole_json(&out);
    assert_eq!(cleared["cleared"], true, "{cleared}");
    // The file is gone, so compare its directory (canonical on both
    // sides) and its name.
    let state = PathBuf::from(cleared["state"].as_str().unwrap());
    assert_eq!(
        state.parent().unwrap().canonicalize().unwrap(),
        current.parent().unwrap().canonicalize().unwrap(),
        "{cleared}"
    );
    assert_eq!(state.file_name(), current.file_name(), "{cleared}");
    assert!(!current.exists(), "the world's rails state was not cleared");
    let out = acq_in(base, "store", &["daemon", "reset-tripwire", "--json"], &env);
    assert_eq!(sole_json(&out)["cleared"], false);

    // A directory in the state file's place (round 21): the daemon
    // refuses to start over it, naming this verb; the verb clears an
    // empty one and names a non-empty one for the hand.
    std::fs::create_dir(&current).unwrap();
    let mut cmd = daemon_command(base, "store");
    for (k, v) in &env {
        cmd.env(k, v);
    }
    let (status, stderr) = refused_daemon(cmd);
    assert!(
        !status.success()
            && stderr.contains("could not be read")
            && stderr.contains("reset-tripwire"),
        "{stderr}"
    );
    let out = acq_in(base, "store", &["daemon", "reset-tripwire", "--json"], &env);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(sole_json(&out)["cleared"], true);
    assert!(!current.exists(), "the empty directory was cleared");
    std::fs::create_dir_all(current.join("inside")).unwrap();
    let out = acq_in(base, "store", &["daemon", "reset-tripwire", "--json"], &env);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let msg = sole_json(&out)["error"].as_str().unwrap().to_string();
    assert!(
        msg.contains("not empty") && msg.contains("by hand"),
        "{msg}"
    );
    assert!(
        current.join("inside").exists(),
        "a non-empty directory was touched"
    );
    std::fs::remove_dir_all(&current).unwrap();
}

/// C24, C129: the offline reset clears what a reset clears, the trip and
/// the block mark, and keeps the dead-grant marks, which persist across
/// restarts until `acq auth` or logout; only the halt goes.
#[test]
fn c24_the_offline_reset_clears_the_halt_and_keeps_the_dead_grant_marks() {
    let scratch = scratch("rails-owners");
    let base = &scratch.0;
    let current = base.join("store").join("mock").join("rails.json");
    std::fs::create_dir_all(current.parent().unwrap()).unwrap();
    std::fs::write(
        &current,
        r#"{"tripped":"429 on GET /stash/Standard","blocked":"Cloudflare answered in GGG's place: 403 on GET stash@A#1 at 2026-10-22T00:00:00.000Z","refresh_failed_by_account":{"A#1":"400 invalid_grant"}}"#,
    )
    .unwrap();
    let out = acq_in(base, "store", &["daemon", "reset-tripwire", "--json"], &[]);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(sole_json(&out)["cleared"], true);
    let held: Value = serde_json::from_str(&std::fs::read_to_string(&current).unwrap()).unwrap();
    assert!(held.get("blocked").is_none_or(|b| b.is_null()), "{held}");
    assert!(held.get("tripped").is_none_or(|t| t.is_null()), "{held}");
    assert_eq!(
        held["refresh_failed_by_account"]["A#1"], "400 invalid_grant",
        "the dead-grant mark was lost: {held}"
    );
    // Nothing left to clear: the marks stay, and the verb says so.
    let out = acq_in(base, "store", &["daemon", "reset-tripwire", "--json"], &[]);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(sole_json(&out)["cleared"], false);
    assert!(current.exists(), "the dead-grant marks keep the file");
    assert_eq!(sole_json(&out)["unreadable"], false);
    // A file nobody can read is still the verb's to remove: the daemon
    // refuses to start over it, naming this verb. The words say the file
    // was removed unread, marks and all, never that a halt was cleared.
    std::fs::write(&current, "{corrupt").unwrap();
    let out = acq_in(base, "store", &["daemon", "reset-tripwire", "--json"], &[]);
    assert!(out.status.success(), "{out:?}");
    let removed = sole_json(&out);
    assert_eq!(removed["cleared"], true, "{removed}");
    assert_eq!(removed["unreadable"], true, "{removed}");
    assert!(!current.exists(), "the unreadable state was not removed");
    std::fs::write(&current, "{corrupt").unwrap();
    let out = acq_in(base, "store", &["daemon", "reset-tripwire"], &[]);
    assert!(out.status.success(), "{out:?}");
    let said = String::from_utf8_lossy(&out.stdout);
    assert!(
        said.contains("removed")
            && said.contains("could not be read")
            && said.contains("dead-grant marks")
            && !said.contains("cleared the rails halt"),
        "{said}"
    );
    assert!(!current.exists(), "the unreadable state was not removed");
}

/// C83: the offline reset holds the world lock across its read and its
/// write, so no daemon can start between them and have its marks
/// overwritten; while another process holds the lock, it refuses and
/// leaves the file as it was.
#[test]
fn the_offline_reset_holds_the_world_lock() {
    let scratch = scratch("rails-lock");
    let base = &scratch.0;
    let current = base.join("store").join("mock").join("rails.json");
    std::fs::create_dir_all(current.parent().unwrap()).unwrap();
    let staged = r#"{"tripped":"429 on GET /stash/Standard"}"#;
    std::fs::write(&current, staged).unwrap();
    let world = acquisition_store::world::World::observe_at(&base.join("store")).unwrap();
    let held = acquisition_store::world::Lock::acquire(&world.lock_path()).unwrap();
    let out = acq_in(base, "store", &["daemon", "reset-tripwire", "--json"], &[]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let msg = sole_json(&out)["error"].as_str().unwrap().to_string();
    assert!(msg.contains("held by another process"), "{msg}");
    assert_eq!(std::fs::read_to_string(&current).unwrap(), staged);
    drop(held);
    let out = acq_in(base, "store", &["daemon", "reset-tripwire", "--json"], &[]);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(sole_json(&out)["cleared"], true);
    assert!(!current.exists());
}

/// `path` at `mode` until this drops, then back at the mode it had, so
/// the folder can go however the test ends. The daemon harness's
/// `with_mode`, which this crate's tests cannot reach.
#[cfg(unix)]
struct WithMode<'a> {
    path: &'a std::path::Path,
    was: std::fs::Permissions,
}

#[cfg(unix)]
fn with_mode(path: &std::path::Path, mode: u32) -> WithMode<'_> {
    use std::os::unix::fs::PermissionsExt;
    let was = std::fs::metadata(path).unwrap().permissions();
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).unwrap();
    WithMode { path, was }
}

#[cfg(unix)]
impl Drop for WithMode<'_> {
    fn drop(&mut self) {
        let _ = std::fs::set_permissions(self.path, self.was.clone());
    }
}

/// The offline reset names a lock it could not open for what failed,
/// never as a daemon that took the world: a read-only world with no
/// daemon in it fails to create its lock file, and the file is left.
#[cfg(unix)]
#[test]
fn the_offline_reset_names_a_lock_it_could_not_open() {
    let scratch = scratch("rails-lock-io");
    let base = &scratch.0;
    let root = base.join("store");
    let current = root.join("mock").join("rails.json");
    std::fs::create_dir_all(current.parent().unwrap()).unwrap();
    let staged = r#"{"tripped":"429 on GET /stash/Standard"}"#;
    std::fs::write(&current, staged).unwrap();
    let read_only = with_mode(&root, 0o555);
    let out = acq_in(base, "store", &["daemon", "reset-tripwire", "--json"], &[]);
    drop(read_only);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let msg = sole_json(&out)["error"].as_str().unwrap().to_string();
    assert!(
        msg.contains("could not take the lock")
            && msg.contains("cleared nothing")
            && !msg.contains("a daemon took this world"),
        "{msg}"
    );
    assert_eq!(std::fs::read_to_string(&current).unwrap(), staged);
}

/// The halt's advice follows its kind: a tripwire's persists in the world
/// and waits out the post-violation time before `reset-tripwire`; a
/// ceiling is the lifetime's alone, so `daemon status` sends the reader to
/// the next daemon, never to the reset (seen 2026-10-15 on a first
/// contact that met its ceiling exactly); a trip beside a ceiling met is
/// advised as both, for the reset lifts the trip alone.
#[test]
fn a_ceiling_halt_is_advised_as_the_lifetimes_and_a_trip_as_a_reset() {
    let scratch = scratch("halts");
    let base = &scratch.0;

    // The mock's open fetch probes its route first: that one send meets a
    // ceiling of one exactly, and the fetch waits behind the halt.
    let ceiling = [("ACQ_TRIPWIRE", "1"), ("ACQ_MAX_SENDS", "1")];
    let (_daemon, _pid) = start_daemon(base, "store", &ceiling);
    let out = acq_in(base, "store", &["submit", "fetch", "--detach"], &ceiling);
    assert!(out.status.success(), "{out:?}");
    let deadline = Instant::now() + Duration::from_secs(120);
    loop {
        let status = sole_json(&acq_in(
            base,
            "store",
            &["daemon", "status", "--json"],
            &ceiling,
        ));
        if status["rails"]["halted"].is_string() {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "the ceiling never halted: {status}"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    let said = text(&acq_in(base, "store", &["daemon", "status"], &ceiling));
    assert!(
        said.contains("HALTED: ceiling")
            && said.contains("the next one starts fresh")
            && !said.contains("reset-tripwire"),
        "{said}"
    );
    let out = acq_in(base, "store", &["daemon", "stop", "--json"], &ceiling);
    assert!(out.status.success(), "{out:?}");
    wait_stopped(base, "store", &ceiling);

    let trip = [("ACQ_TRIPWIRE", "1")];
    let current = base.join("store").join("mock").join("rails.json");
    std::fs::create_dir_all(current.parent().unwrap()).unwrap();
    std::fs::write(
        &current,
        r#"{"tripped":"429 on GET /stash/Standard (staged trip)"}"#,
    )
    .unwrap();
    let (_daemon, _pid) = start_daemon(base, "store", &trip);
    let said = text(&acq_in(base, "store", &["daemon", "status"], &trip));
    assert!(
        said.contains("staged trip")
            && said.contains("reset-tripwire")
            && said.contains("post-violation"),
        "{said}"
    );
    assert!(!said.contains("ceiling is met too"), "{said}");
    let out = acq_in(base, "store", &["daemon", "stop", "--json"], &trip);
    assert!(out.status.success(), "{out:?}");
    wait_stopped(base, "store", &trip);

    // The trip beside a ceiling already met: the reset lifts the trip
    // alone, so the advice names the ceiling and the next daemon too.
    let both = [("ACQ_TRIPWIRE", "1"), ("ACQ_MAX_SENDS", "0")];
    let (_daemon, _pid) = start_daemon(base, "store", &both);
    let said = text(&acq_in(base, "store", &["daemon", "status"], &both));
    assert!(
        said.contains("staged trip")
            && said.contains("ceiling is met too, and a reset does not lift it")
            && said.contains("the next daemon"),
        "{said}"
    );
    let out = acq_in(base, "store", &["daemon", "stop", "--json"], &both);
    assert!(out.status.success(), "{out:?}");
    wait_stopped(base, "store", &both);
}

/// C83: the log and the default journal live under the log directory,
/// one subdirectory per world and provider, beside a `world` file naming
/// the root; they are bounded — a log over the cap is rotated once at
/// the daemon's start to `daemon.log.1`, and `daemon status` reports the
/// paths.
#[test]
fn c83_diagnostics_live_under_the_log_directory_and_are_bounded() {
    let scratch = scratch("logs");
    let base = &scratch.0;
    std::fs::create_dir_all(base.join("store")).unwrap();
    let log = daemon_log(base, "store", "mock");
    std::fs::create_dir_all(log.parent().unwrap()).unwrap();
    // A log the last lifetime left over the cap — a gibibyte, sparse, so
    // the test stages no bytes and pins no number: the length is what
    // rotation reads (the cap is the daemon's, `daemon/start.rs`).
    let over = 1u64 << 30;
    std::fs::File::create(&log).unwrap().set_len(over).unwrap();

    // This daemon keeps the default journal (no `ACQ_JOURNAL`).
    let mut cmd = daemon_command(base, "store");
    cmd.env_remove("ACQ_JOURNAL");
    let child = cmd.spawn().unwrap();
    let _daemon = Proc(child, acqd());
    let deadline = Instant::now() + Duration::from_secs(120);
    let status = loop {
        let out = acq(base, "store", &["daemon", "status", "--json"]);
        let status = sole_json(&out);
        if status["pid"].is_number() {
            break status;
        }
        assert!(Instant::now() < deadline, "the daemon did not come up");
        std::thread::sleep(Duration::from_millis(50));
    };
    assert_eq!(status["log"], log.display().to_string(), "{status}");
    assert_eq!(
        status["rails"]["journal"],
        log.with_file_name("sends.jsonl").display().to_string(),
        "{status}"
    );
    // The path is the daemon's own report: a shell with another log
    // directory, or none, still learns the file the daemon opened
    // (review 2026-09-11).
    for other in [Some(base.join("elsewhere-logs")), None] {
        let mut cmd = command(base, "store", &["daemon", "status", "--json"]);
        match &other {
            Some(dir) => {
                cmd.env("ACQ_LOG_DIR", dir);
            }
            None => {
                cmd.env_remove("ACQ_LOG_DIR");
            }
        }
        let out = cmd.output().unwrap();
        let status = sole_json(&out);
        assert_eq!(
            status["log"],
            log.display().to_string(),
            "{other:?}: {status}"
        );
        let out = {
            let mut cmd = command(base, "store", &["daemon", "status"]);
            match &other {
                Some(dir) => {
                    cmd.env("ACQ_LOG_DIR", dir);
                }
                None => {
                    cmd.env_remove("ACQ_LOG_DIR");
                }
            }
            cmd.output().unwrap()
        };
        assert!(
            text(&out).contains(&format!("log:    {}", log.display())),
            "{other:?}: {}",
            text(&out)
        );
    }
    let previous = PathBuf::from(format!("{}.1", log.display()));
    assert_eq!(
        std::fs::metadata(&previous).unwrap().len(),
        over,
        "the over-cap log became the previous generation"
    );
    let now = std::fs::read_to_string(&log).unwrap();
    assert!(
        now.len() < 4096,
        "the current log is fresh: {} bytes",
        now.len()
    );
    assert!(
        now.contains("log: rotated") && now.contains("daemon.log.1"),
        "{now}"
    );
    assert!(now.contains("world: "), "{now}");
    let marker = log.parent().unwrap().parent().unwrap().join("world");
    assert_eq!(
        std::fs::read_to_string(&marker).unwrap().trim(),
        acquisition_store::world::canonical(&base.join("store"))
            .unwrap()
            .display()
            .to_string()
    );
    let out = acq(base, "store", &["daemon", "stop", "--json"]);
    assert!(out.status.success(), "{out:?}");

    // A log directory that is not valid UTF-8 refuses the start, before
    // anything is made: the path crosses the wire exactly or not at all
    // (round 21's pin).
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        let odd = std::ffi::OsString::from(std::ffi::OsStr::from_bytes(b"/tmp/acq-\xff-logs"));
        let mut cmd = daemon_command(base, "store");
        cmd.env("ACQ_LOG_DIR", &odd);
        let (status, stderr) = refused_daemon(cmd);
        assert!(
            !status.success()
                && stderr.contains("not valid UTF-8")
                && stderr.contains("ACQ_LOG_DIR"),
            "{stderr}"
        );
        assert!(
            sockets_under(&scratch_tmp(base)).is_empty(),
            "the refused daemon bound the world's socket"
        );
    }
}

/// C83: a socket path over the cap is refused by name — by the daemon
/// before it binds, and by a shell before it connects — rather than by
/// `bind`'s `ENAMETOOLONG`. Staged with a runtime directory deep enough
/// to push the path past 103 bytes. Unix only: a named pipe's name is
/// fixed-length and lives in no directory (C112).
#[cfg(unix)]
#[test]
fn c83_a_socket_path_over_the_cap_is_refused_by_name() {
    let scratch = scratch("cap");
    let base = &scratch.0;
    let deep = base.join("tmp").join("x".repeat(96));
    std::fs::create_dir_all(&deep).unwrap();
    let deep_s = deep.display().to_string();
    let env = [
        ("TMPDIR", deep_s.as_str()),
        ("XDG_RUNTIME_DIR", deep_s.as_str()),
    ];
    let mut cmd = daemon_command(base, "store");
    for (k, v) in &env {
        cmd.env(k, v);
    }
    let (status, stderr) = refused_daemon(cmd);
    assert!(
        !status.success() && stderr.contains("at most 103"),
        "{stderr}"
    );
    assert!(sockets_under(&deep).is_empty());
    let out = acq_in(base, "store", &["daemon", "status", "--json"], &env);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let msg = sole_json(&out)["error"].as_str().unwrap().to_string();
    assert!(msg.contains("at most 103"), "{msg}");
}

/// C83: a runtime directory the environment names in bytes that are not
/// UTF-8 (Linux permits it in `XDG_RUNTIME_DIR`; `TMPDIR` anywhere)
/// cannot name a socket in a report, so the daemon refuses to start
/// naming the variable, and every JSON surface that would carry the
/// socket — `daemon status`, `daemon stop` — answers C11's total
/// `{"error": …}` with exit 1, never a panic (review 2026-09-11).
#[cfg(unix)]
#[test]
fn c83_a_runtime_directory_that_is_not_utf8_is_refused_by_name_and_never_panics_a_report() {
    use std::os::unix::ffi::OsStrExt;
    let scratch = scratch("utf8");
    let base = &scratch.0;
    std::fs::create_dir_all(base.join("store")).unwrap();
    let mut odd = base.join("tmp").into_os_string();
    odd.push(std::ffi::OsStr::from_bytes(b"-\xff"));
    // The daemon: refused before anything binds.
    let mut cmd = daemon_command(base, "store");
    cmd.env("TMPDIR", &odd).env("XDG_RUNTIME_DIR", &odd);
    let (status, stderr) = refused_daemon(cmd);
    assert!(
        !status.success()
            && stderr.contains("not valid UTF-8")
            && (stderr.contains("XDG_RUNTIME_DIR") || stderr.contains("TMPDIR")),
        "{stderr}"
    );
    // The shells: a total JSON error, exit 1, on both surfaces.
    for verb in [
        &["daemon", "status", "--json"][..],
        &["daemon", "stop", "--json"][..],
    ] {
        let mut cmd = command(base, "store", verb);
        cmd.env("TMPDIR", &odd).env("XDG_RUNTIME_DIR", &odd);
        let out = cmd.output().unwrap();
        assert_eq!(out.status.code(), Some(1), "{verb:?}: {out:?}");
        let text = String::from_utf8_lossy(&out.stderr).into_owned();
        assert!(!text.contains("panicked"), "{verb:?}: {text}");
        let msg = sole_json(&out)["error"].as_str().unwrap().to_string();
        assert!(msg.contains("not valid UTF-8"), "{verb:?}: {msg}");
    }
}
