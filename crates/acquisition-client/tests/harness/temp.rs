//! The temp and runtime directory variables, named once (C122): the
//! harness's isolation list points them into a test's scratch, and the
//! client's unit tests, which move the environment in-process under
//! their own lock, include this file by `#[path]` without the harness
//! beside it, so a test that moves the temp directory knows every
//! platform's reader the harness knows.
#![allow(dead_code)]

/// Every variable a platform's temp or runtime directory is read from:
/// `TMPDIR` on macOS, `XDG_RUNTIME_DIR` on Linux, and `TEMP` and `TMP`,
/// which `std::env::temp_dir` reads on Windows.
pub const TEMP_VARS: [&str; 4] = ["TMPDIR", "TEMP", "TMP", "XDG_RUNTIME_DIR"];
