//! The MCP's adapter over the shared process harness (C122): the harness
//! itself is `acquisition-client/tests/harness/mod.rs`, included here by
//! `#[path]` and re-exported whole. What stays here has one consumer:
//! `acq-mcp` spawned under the harness's one isolation list, the daemon
//! it would itself spawn (the `acqd` beside it, C82) started under the
//! same list with the test owning the pid, the `acq` beside it run with
//! no spawn, `run_to_exit`'s bounded reaping and the `Mcp` stdio client
//! speaking newline-delimited JSON-RPC.
#![allow(dead_code, unused_imports)] // each test binary uses the slice it needs

#[path = "../../../acquisition-client/tests/harness/mod.rs"]
mod shared;

/// The journey's one table of facts, shared with the CLI's golden.
#[path = "../../../acquisition-client/tests/harness/journey.rs"]
pub mod journey;

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, channel};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

pub use shared::*;

/// The server under test.
pub fn exe() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_acq-mcp"))
}

/// Spawn the `acq-mcp` binary under the isolation of `base`, stderr to
/// null, and `extra_env` applied last (so a test can opt into e.g.
/// `ACQ_PROVIDER=ggg` deliberately).
pub fn spawn(
    base: &Path,
    args: &[&str],
    extra_env: &[(&str, &str)],
    stdio: fn() -> Stdio,
) -> Child {
    let mut cmd = Command::new(exe());
    cmd.args(args);
    isolate(&mut cmd, base);
    cmd.stdin(stdio()).stdout(stdio()).stderr(Stdio::null());
    cmd.envs(extra_env.iter().copied());
    cmd.spawn().expect("spawning acq-mcp")
}

/// The daemon `acq-mcp` would lazily spawn in mock mode, the `acqd` beside
/// it, started directly so the test owns the pid, `extra_env` applied
/// last. The tests also drive the client in-process, so the daemon's
/// name is linked beside this executable first (C84).
pub fn spawn_daemon(base: &Path, extra_env: &[(&str, &str)]) -> Proc {
    let acqd = acqd_beside(exe());
    link_beside_this_test(&acqd);
    let mut cmd = daemon_command(&acqd, base);
    cmd.envs(extra_env.iter().copied());
    spawn_drained(cmd, &acqd).0
}

/// The daemon `acq-mcp` would spawn, started by the test with `env` on
/// its own command alone (the journal, [`journal_on`]), returned with its
/// pid once the `acq` beside the server sees it running, compatible and
/// mock.
pub fn start_daemon_with(base: &Path, env: &[(&str, &str)]) -> (Proc, u64) {
    shared::start_daemon_with(&sibling(exe(), "acq"), &acqd_beside(exe()), base, env)
}

/// The scripted mock login through the `acq` beside the server (C82: one
/// build wrote both), as the mock's own user.
pub fn login_as_mock(base: &Path) {
    shared::login(&sibling(exe(), "acq"), base, None, &[]);
}

/// Poll `job_status` until the job is terminal, then return `job_result`'s
/// outcome: the MCP idiom, a submit returns an id and the agent polls.
pub fn finish_job(mcp: &mut Mcp, id: &Value) -> Value {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        let status = mcp.expect_ok("job_status", json!({ "id": id }));
        match status["state"].as_str() {
            Some("done") => break,
            Some("failed") | Some("cancelled") => panic!("job did not succeed: {status}"),
            _ if Instant::now() > deadline => panic!("job never finished: {status}"),
            _ => std::thread::sleep(Duration::from_millis(100)),
        }
    }
    mcp.expect_ok("job_result", json!({ "id": id }))
}

/// Run the `acq` beside the server under test (C82: one build wrote
/// both) under the same isolation, never spawning a daemon: what a
/// tool's JSON is held equal to.
pub fn acq(base: &Path, args: &[&str]) -> std::process::Output {
    let mut cmd = Command::new(sibling(exe(), "acq"));
    cmd.args(args);
    offline(&mut cmd, base);
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    cmd.output().expect("running acq")
}

/// How long one JSON-RPC answer may take before the test fails naming
/// the method: a server or daemon that stops answering must fail the
/// gate, never hang it (review 2026-09-11).
const ANSWER_WITHIN: Duration = Duration::from_secs(30);

/// Run the server over `input`, then close its stdin, and return what it
/// wrote to stdout. It must exit within [`ANSWER_WITHIN`]: one that does
/// not is killed and the test fails, never hanging the gate; the child
/// is killed and waited for on any unwinding too.
pub fn run_to_exit(base: &Path, input: &[u8]) -> String {
    struct Reaped(Child);
    impl Drop for Reaped {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    let mut child = Reaped(spawn(base, &[], &[], Stdio::piped));
    let mut stdin = child.0.stdin.take().unwrap();
    let mut stdout = child.0.stdout.take().unwrap();
    let (tx, out) = channel();
    std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let _ = std::io::Read::read_to_end(&mut stdout, &mut bytes);
        let _ = tx.send(bytes);
    });
    stdin.write_all(input).unwrap();
    drop(stdin);
    let deadline = Instant::now() + ANSWER_WITHIN;
    while child.0.try_wait().unwrap().is_none() {
        assert!(
            Instant::now() < deadline,
            "the server did not exit within {ANSWER_WITHIN:?} of its stdin closing"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    let bytes = out
        .recv_timeout(ANSWER_WITHIN)
        .expect("the server's stdout did not close after it exited");
    String::from_utf8_lossy(&bytes).into_owned()
}

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

    /// Send one line as written and wait for the message answering `id`
    /// (`Value::Null` for a reply to a line whose id could not be read).
    pub fn raw(&mut self, line: &str, id: &Value) -> Value {
        self.stdin
            .write_all(format!("{line}\n").as_bytes())
            .unwrap();
        self.stdin.flush().unwrap();
        let deadline = Instant::now() + ANSWER_WITHIN;
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            let line = self
                .lines
                .recv_timeout(left)
                .unwrap_or_else(|e| panic!("no answer to id {id} within {ANSWER_WITHIN:?} ({e})"));
            let msg: Value = serde_json::from_str(&line)
                .unwrap_or_else(|e| panic!("not JSON-RPC ({e}): {line}"));
            if &msg["id"] == id {
                return msg;
            }
        }
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

/// C143: the server's document is the CLI's with this server's spelling
/// after the shared words — the same shape, and each string the CLI's
/// whole, or the CLI's followed by ` — here, ` and the spelling. The
/// first string that differs otherwise, with its path, or `None`.
pub fn spoken_otherwise(server: &Value, cli: &Value, path: &str) -> Option<String> {
    match (server, cli) {
        (Value::String(s), Value::String(c)) => (s != c
            && !s
                .strip_prefix(c.as_str())
                .is_some_and(|rest| rest.starts_with(" — here, ")))
        .then(|| format!("{path}: {s:?} is not {c:?} spelled")),
        (Value::Array(s), Value::Array(c)) if s.len() == c.len() => s
            .iter()
            .zip(c)
            .enumerate()
            .find_map(|(i, (s, c))| spoken_otherwise(s, c, &format!("{path}[{i}]"))),
        (Value::Object(s), Value::Object(c))
            if s.keys().collect::<Vec<_>>() == c.keys().collect::<Vec<_>>() =>
        {
            s.iter()
                .find_map(|(k, v)| spoken_otherwise(v, &c[k], &format!("{path}.{k}")))
        }
        _ => (server != cli).then(|| format!("{path}: {server} is not {cli}")),
    }
}

/// [`spoken_otherwise`] as an assertion.
#[track_caller]
pub fn assert_spoken_as(server: &Value, cli: &Value, what: &str) {
    if let Some(differs) = spoken_otherwise(server, cli, "") {
        panic!("{what}: {differs}");
    }
}
