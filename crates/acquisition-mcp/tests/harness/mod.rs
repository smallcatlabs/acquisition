//! Shared process-test harness: spawn `acq-mcp` in an isolated
//! environment and speak newline-delimited JSON-RPC to the server's
//! stdio; start the daemon it would itself spawn — the `acqd` beside it
//! (C82) — under the same isolation, owning the pid.
#![allow(dead_code)] // each test binary uses the slice it needs

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, channel};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

/// Spawn the `acq-mcp` binary with the scratch socket/store under `base`,
/// the live-run knobs scrubbed, and `extra_env` applied last (so a test
/// can opt into e.g. `ACQ_PROVIDER=ggg` deliberately).
pub fn spawn(
    base: &Path,
    args: &[&str],
    extra_env: &[(&str, &str)],
    stdio: fn() -> Stdio,
) -> Child {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_acq-mcp"));
    cmd.args(args);
    isolate(&mut cmd, base, extra_env, stdio);
    cmd.spawn().expect("spawning acq-mcp")
}

/// The daemon `acq-mcp` would lazily spawn in mock mode: the `acqd`
/// beside the binary under test (C82). A missing one fails here, before
/// the test runs, naming the build step; the test owns the pid. The
/// tests also drive the client in-process, which judges the daemon's
/// artifact against the `acqd` beside *this* executable (C84) — a test
/// executable in `deps/` has none — so a symlink of that name is placed
/// beside it, pointing one level up at the same file: the test
/// executable names the daemon its build wrote on both paths (C82's
/// clause for test executables).
pub fn spawn_daemon(base: &Path, extra_env: &[(&str, &str)]) -> Daemon {
    let acqd = acquisition_client::locator::beside(Path::new(env!("CARGO_BIN_EXE_acq-mcp")))
        .unwrap_or_else(|e| panic!("{e}"));
    let exe = std::env::current_exe().expect("current exe");
    let link = exe
        .parent()
        .expect("a parent")
        .join(acquisition_client::locator::ACQD);
    let target = Path::new("..").join(acquisition_client::locator::ACQD);
    if std::fs::read_link(&link).ok().as_deref() != Some(target.as_path()) {
        let _ = std::fs::remove_file(&link);
        #[cfg(unix)]
        let placed = std::os::unix::fs::symlink(&target, &link);
        // Windows: a file symlink needs Developer Mode or elevation (CI's
        // runner is elevated; `PLATFORMS-SLICE.md`, observations).
        #[cfg(windows)]
        let placed = std::os::windows::fs::symlink_file(&target, &link);
        placed
            .unwrap_or_else(|e| panic!("placing {} -> {}: {e}", link.display(), target.display()));
    }
    assert_eq!(
        link.canonicalize().expect("the sibling resolves"),
        acqd.canonicalize().expect("the daemon resolves"),
        "the sibling the in-process client sees is not the daemon acq-mcp would spawn"
    );
    let mut cmd = Command::new(&acqd);
    isolate(&mut cmd, base, extra_env, Stdio::null);
    let child = cmd
        .spawn()
        .unwrap_or_else(|e| panic!("spawning {}: {e}", acqd.display()));
    Daemon(child, acqd)
}

/// Run the `acq` beside the server under test (C82: one build wrote
/// both), under the same isolation: what a tool's JSON is held equal to.
/// A missing one fails here, naming the build step.
pub fn acq(base: &Path, args: &[&str]) -> std::process::Output {
    let acq = Path::new(env!("CARGO_BIN_EXE_acq-mcp"))
        .parent()
        .expect("a parent")
        .join(format!("acq{}", std::env::consts::EXE_SUFFIX));
    assert!(
        acq.exists(),
        "no acq beside acq-mcp at {}: run `cargo build --workspace` first (C82)",
        acq.display()
    );
    let mut cmd = Command::new(&acq);
    cmd.args(args);
    isolate(&mut cmd, base, &[("ACQ_NO_SPAWN", "1")], Stdio::piped);
    cmd.stderr(Stdio::piped());
    cmd.output().expect("running acq")
}

/// No daemon came up under `base`: nothing bound a socket in the
/// test's scratch runtime directory (C83: a daemon's socket derives from
/// its world into that directory, and nothing else binds there).
pub fn no_daemon_appeared(base: &Path) -> bool {
    sockets_under(&scratch_tmp(base)).is_empty()
}

/// Every Unix socket under `dir`, recursively.
#[cfg(unix)]
pub fn sockets_under(dir: &Path) -> Vec<PathBuf> {
    use std::os::unix::fs::FileTypeExt;
    let mut found = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return found;
    };
    for entry in entries.flatten() {
        let kind = entry.file_type().unwrap();
        if kind.is_dir() {
            found.extend(sockets_under(&entry.path()));
        } else if kind.is_socket() {
            found.push(entry.path());
        }
    }
    found.sort();
    found
}

/// Windows: there are no socket files, so the endpoints "under `dir`"
/// are the live pipes (C112) of the worlds a test made beside it — every
/// directory up to two levels under `dir`'s parent, the test's scratch
/// base, taken as a root, and its pipe kept when the pipe namespace lists
/// it. Each test has its own base, so tests running at once never see
/// each other's.
#[cfg(windows)]
pub fn sockets_under(dir: &Path) -> Vec<PathBuf> {
    fn dirs(at: &Path, depth: u32, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(at) else {
            return;
        };
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                out.push(entry.path());
                if depth > 1 {
                    dirs(&entry.path(), depth - 1, out);
                }
            }
        }
    }
    let Some(base) = dir.parent() else {
        return Vec::new();
    };
    let mut roots = Vec::new();
    dirs(base, 2, &mut roots);
    let mut found: Vec<PathBuf> = roots
        .iter()
        .filter_map(|root| acquisition_store::world::World::at(root).ok())
        .filter_map(|world| world.socket_path().ok())
        .filter(|pipe| acquisition_client::endpoint::exists(pipe))
        .collect();
    found.sort();
    found.dedup();
    found
}

/// The scratch temp and runtime directory of one test, under `base`:
/// `TMPDIR` (macOS's runtime fallback) and
/// `XDG_RUNTIME_DIR` (Linux's runtime directory) both point here, so the
/// sockets the daemons bind — and, for a daemon a test kills rather than
/// stops, the socket files it leaves — never touch the user's own
/// runtime directory (C83). Created here, since the runtime directory
/// is made under an existing parent.
pub fn scratch_tmp(base: &std::path::Path) -> std::path::PathBuf {
    let tmp = base.join("tmp");
    let _ = std::fs::create_dir_all(&tmp);
    tmp
}

/// The daemon a test started: killed and waited for on drop, whichever
/// way the test ends — a timed-out answer must not leave it behind
/// (review 2026-09-11) — and named while a test is panicking (C82: a
/// failure says which daemon ran).
pub struct Daemon(Child, PathBuf);

impl Daemon {
    pub fn id(&self) -> u32 {
        self.0.id()
    }
}

impl Drop for Daemon {
    fn drop(&mut self) {
        if std::thread::panicking() {
            eprintln!(
                "the daemon under test was {} (pid {})",
                self.1.display(),
                self.0.id()
            );
        }
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// The scratch socket and store under `base`, the live-run knobs
/// scrubbed, `extra_env` applied last.
fn isolate(cmd: &mut Command, base: &Path, extra_env: &[(&str, &str)], stdio: fn() -> Stdio) {
    cmd.env("ACQ_STORE_DIR", base.join("store"))
        .env("TMPDIR", scratch_tmp(base))
        .env("XDG_RUNTIME_DIR", scratch_tmp(base))
        .env("ACQ_LOG_DIR", base.join("logs"))
        .env("ACQ_NO_KEYRING", "1")
        .stdin(stdio())
        .stdout(stdio())
        .stderr(Stdio::null());
    for var in [
        "ACQ_GGG",
        "ACQ_ACCOUNT",
        "ACQ_TRIPWIRE",
        "ACQ_MAX_SENDS",
        "ACQ_JOURNAL",
        "ACQ_IDLE_SHUTDOWN",
        "ACQ_NO_SPAWN",
    ] {
        cmd.env_remove(var);
    }
    cmd.env("ACQ_PROVIDER", "mock");
    for (key, value) in extra_env {
        cmd.env(key, value);
    }
}

/// How long one JSON-RPC answer may take before the test fails naming
/// the method: a server or daemon that stops answering must fail the
/// gate, never hang it (review 2026-09-11).
const ANSWER_WITHIN: Duration = Duration::from_secs(30);

/// A newline-delimited JSON-RPC conversation with the MCP server's stdio.
/// The server's stdout is read on a thread into a channel, so every wait
/// for an answer is bounded; the server is killed and waited for on
/// drop, so a wedged one does not outlive its test.
pub struct Mcp {
    pub child: Child,
    stdin: ChildStdin,
    lines: Receiver<String>,
    next_id: i64,
    /// The server's `instructions` from `initialize`.
    pub instructions: String,
}

impl Drop for Mcp {
    fn drop(&mut self) {
        if std::thread::panicking() {
            eprintln!(
                "the MCP server under test was {} (pid {})",
                env!("CARGO_BIN_EXE_acq-mcp"),
                self.child.id()
            );
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl Mcp {
    pub fn start(base: &Path, extra_env: &[(&str, &str)]) -> Mcp {
        let mut child = spawn(base, &[], extra_env, Stdio::piped);
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let (tx, lines) = channel();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                let Ok(line) = line else { break };
                if tx.send(line).is_err() {
                    break;
                }
            }
        });
        let mut mcp = Mcp {
            child,
            stdin,
            lines,
            next_id: 0,
            instructions: String::new(),
        };
        let init = mcp.rpc(
            "initialize",
            json!({
                "protocolVersion": "2025-06-18",
                "capabilities": {},
                "clientInfo": { "name": "acq-mcp-test", "version": "0" }
            }),
        );
        assert!(init.get("result").is_some(), "initialize failed: {init}");
        mcp.instructions = init["result"]["instructions"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        mcp.send(&json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }));
        mcp
    }

    /// `tools/list`: every tool the server advertises, as sent.
    pub fn list_tools(&mut self) -> Vec<Value> {
        let resp = self.rpc("tools/list", json!({}));
        resp["result"]["tools"]
            .as_array()
            .unwrap_or_else(|| panic!("tools/list: {resp}"))
            .clone()
    }

    fn send(&mut self, msg: &Value) {
        let mut line = msg.to_string();
        line.push('\n');
        self.stdin.write_all(line.as_bytes()).unwrap();
        self.stdin.flush().unwrap();
    }

    fn rpc(&mut self, method: &str, params: Value) -> Value {
        self.next_id += 1;
        let id = self.next_id;
        self.send(&json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params }));
        let deadline = Instant::now() + ANSWER_WITHIN;
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            let line = self.lines.recv_timeout(left).unwrap_or_else(|e| {
                panic!("no answer to {method} within {ANSWER_WITHIN:?} ({e}); the MCP server, or the daemon it asked, stopped answering")
            });
            let msg: Value = serde_json::from_str(&line)
                .unwrap_or_else(|e| panic!("not JSON-RPC ({e}): {line}"));
            if msg["id"] == json!(id) {
                return msg;
            }
        }
    }

    /// Call one tool: `Ok(structured result)` or `Err(message)` whether
    /// the failure came back as a JSON-RPC error or an isError result.
    pub fn call(&mut self, tool: &str, args: Value) -> Result<Value, String> {
        let resp = self.rpc("tools/call", json!({ "name": tool, "arguments": args }));
        if let Some(error) = resp.get("error") {
            return Err(error["message"].as_str().unwrap_or_default().to_string());
        }
        let result = &resp["result"];
        let text = || {
            result["content"][0]["text"]
                .as_str()
                .unwrap_or_default()
                .to_string()
        };
        if result["isError"] == json!(true) {
            return Err(text());
        }
        match result.get("structuredContent") {
            Some(v) => Ok(v.clone()),
            None => Ok(serde_json::from_str(&text())
                .unwrap_or_else(|e| panic!("unstructured tool result ({e}): {resp}"))),
        }
    }

    /// Call one tool and hand back its result object whole — `content`,
    /// `structuredContent`, `isError` — for a test of the result's own
    /// shape. A JSON-RPC error fails here: the call was not served.
    pub fn result(&mut self, tool: &str, args: Value) -> Value {
        let resp = self.rpc("tools/call", json!({ "name": tool, "arguments": args }));
        resp.get("result")
            .cloned()
            .unwrap_or_else(|| panic!("{tool} was refused at the protocol level: {resp}"))
    }

    pub fn expect_ok(&mut self, tool: &str, args: Value) -> Value {
        self.call(tool, args.clone())
            .unwrap_or_else(|e| panic!("{tool} {args} failed: {e}"))
    }

    /// The whole JSON-RPC error object (`code`, `message`, `data`) of a
    /// call the server refused at the protocol level.
    pub fn expect_rpc_error(&mut self, tool: &str, args: Value) -> Value {
        let resp = self.rpc("tools/call", json!({ "name": tool, "arguments": args }));
        resp.get("error")
            .cloned()
            .unwrap_or_else(|| panic!("{tool} did not fail at the protocol level: {resp}"))
    }

    pub fn expect_err(&mut self, tool: &str, args: Value) -> String {
        match self.call(tool, args) {
            Err(e) => e,
            Ok(v) => panic!("{tool} unexpectedly succeeded: {v}"),
        }
    }
}
