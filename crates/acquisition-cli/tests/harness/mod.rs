//! The CLI's adapter over the shared process harness (C122): the harness
//! itself is `acquisition-client/tests/harness/mod.rs`, included here by
//! `#[path]` and re-exported whole; this file binds it to `acq`, the
//! binary under test, under the names the CLI's tests call. The `acqd`
//! is the one beside `acq` (C82), and every child takes the harness's one
//! isolation list (the world at `base/store` unless a test names
//! another).
#![allow(dead_code, unused_imports)] // each test binary uses the slice it needs

#[path = "../../../acquisition-client/tests/harness/mod.rs"]
mod shared;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

pub use shared::*;

/// The binary under test.
pub fn acq_exe() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_acq"))
}

/// The binary under test, isolated under `base`.
pub fn command(base: &Path, args: &[&str]) -> Command {
    command_of(acq_exe(), base, args)
}

/// The same isolation for an `acq` at another path (a copy staged for
/// the artifact dimension).
pub fn command_of(exe: &Path, base: &Path, args: &[&str]) -> Command {
    let mut cmd = Command::new(exe);
    cmd.args(args);
    isolate(&mut cmd, base);
    cmd
}

/// [`isolate`] with the world at `base/<store>`: for tests that run two
/// worlds under one scratch directory.
pub fn isolate_in(cmd: &mut Command, base: &Path, store: &str) {
    isolate_at(cmd, &base.join(store), base);
}

/// The binary under test with the world at `base/<store>`.
pub fn command_in(base: &Path, store: &str, args: &[&str]) -> Command {
    let mut cmd = Command::new(acq_exe());
    cmd.args(args);
    isolate_in(&mut cmd, base, store);
    cmd
}

pub fn acq(base: &Path, args: &[&str]) -> Output {
    command(base, args).output().expect("spawning acq")
}

/// The binary under test over a world seeded by hand, the world being
/// `world` itself (its runtime and log directories inside it), never
/// spawning a daemon.
pub fn offline_command(world: &Path, args: &[&str]) -> Command {
    let mut cmd = Command::new(acq_exe());
    cmd.args(args);
    isolate_at(&mut cmd, world, world);
    cmd.env("ACQ_NO_SPAWN", "1");
    cmd
}

/// [`offline_command`], run to its end.
pub fn offline_acq(world: &Path, args: &[&str]) -> Output {
    offline_command(world, args).output().expect("spawning acq")
}

/// The daemon `acq` would itself spawn: the `acqd` beside it.
pub fn acqd() -> PathBuf {
    acqd_beside(acq_exe())
}

/// The mock daemon under `base`, the test owning its pid, once `acq
/// daemon status` sees it running, compatible and mock.
pub fn start_daemon(base: &Path) -> (Proc, u64) {
    shared::start_daemon(acq_exe(), &acqd(), base)
}

/// The scripted mock login as `user`.
pub fn login(base: &Path, user: &str) {
    login_with(base, user, &[]);
}

/// [`login`] with `env` set on the `acq auth` that starts the daemon: a
/// knob the daemon reads at its start goes here.
pub fn login_with(base: &Path, user: &str, env: &[(&str, &str)]) {
    shared::login(acq_exe(), base, Some(user), env);
}

/// The scripted mock login as the mock's own user, `env` set on the `acq
/// auth` that starts the daemon.
pub fn login_as_mock(base: &Path, env: &[(&str, &str)]) {
    shared::login(acq_exe(), base, None, env);
}
