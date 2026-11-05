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

/// The `acq-mcp` binary under the isolation of `base`, not yet spawned.
pub fn command(base: &Path, args: &[&str]) -> Command {
    let mut cmd = Command::new(exe());
    cmd.args(args);
    isolate(&mut cmd, base);
    cmd
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
    let mut cmd = command(base, args);
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
    /// The `initialize` result, as sent.
    pub initialize: Value,
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
            initialize: Value::Null,
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
        mcp.initialize = init["result"].clone();
        mcp.send(&json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }));
        mcp
    }

    /// `tools/list`: every tool the server advertises, as sent.
    pub fn list_tools(&mut self) -> Vec<Value> {
        self.list_tools_result()["tools"]
            .as_array()
            .unwrap_or_else(|| panic!("tools/list"))
            .clone()
    }

    /// `tools/list`'s `result`, as sent.
    pub fn list_tools_result(&mut self) -> Value {
        let resp = self.rpc("tools/list", json!({}));
        resp["result"].clone()
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

    /// Call one tool: `Ok(the document)`, its text block parsed, or
    /// `Err(message)` whether the failure came back as a JSON-RPC error or
    /// an isError result.
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
        c143_a_text_speaks_no_frontend(tool, &text());
        if result["isError"] == json!(true) {
            return Err(text());
        }
        Ok(serde_json::from_str(&text())
            .unwrap_or_else(|e| panic!("a tool result whose text is no document ({e}): {resp}")))
    }

    /// Call one tool and hand back its result object whole — `content`,
    /// `isError` — for a test of the result's own shape ([`document`]
    /// reads its document). A JSON-RPC error fails here: the call was not served.
    pub fn result(&mut self, tool: &str, args: Value) -> Value {
        let resp = self.rpc("tools/call", json!({ "name": tool, "arguments": args }));
        let result = resp
            .get("result")
            .cloned()
            .unwrap_or_else(|| panic!("{tool} was refused at the protocol level: {resp}"));
        let text = result["content"][0]["text"].as_str().unwrap_or_default();
        c143_a_text_speaks_no_frontend(tool, text);
        result
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

/// The commands that are a person's act, which a refusal names as
/// commands (C143's exception): the person's commands in the server's
/// `SPOKEN`, longest first where one begins another.
const PERSONS: [&str; 6] = [
    "acq daemon reset-tripwire",
    "acq daemon stop",
    "acq auth check",
    "acq auth status",
    "acq auth",
    "acq profile",
];

/// C143, a shared text speaks the request's own words, never a
/// frontend's flag: every text a test meets, through [`Mcp::call`] or
/// [`Mcp::result`], is held free of a CLI command (`acq …`) and a flag
/// (`--league`) but a person's act ([`PERSONS`]), so each test that
/// produces an answer or a refusal holds the rule over it (#97). Every
/// string is read but those under a key that holds data ([`DATA`]), so
/// prose a field adds later is read by default (the review of #97's fix
/// round and Part B, P2).
#[track_caller]
pub fn c143_a_text_speaks_no_frontend(tool: &str, text: &str) {
    let doc: Value = serde_json::from_str(text).unwrap_or_else(|_| json!(text));
    if let Some(said) = frontend_speech(&doc) {
        panic!("{tool}'s text speaks the CLI at {said:?}: {text}");
    }
}

/// Where a document's prose first speaks a CLI command or flag outside a
/// person's act, the words from there.
pub fn frontend_speech(doc: &Value) -> Option<String> {
    let mut prose = Vec::new();
    prose_of(doc, &mut prose);
    prose
        .into_iter()
        .find_map(|words| frontend_word(words).map(|at| words[at..].chars().take(60).collect()))
}

/// The keys whose value is data, never the tool's words: what an agent
/// sends back as printed (an offer's `request` and `argv`, a query and
/// its params, a route, a continuation's token), what it typed, echoed
/// (a query's `term` and an explain offer's `node`), what it types as written (an example, a reading,
/// a pattern), and what is stored or
/// upstream (a row as it stands, a row's `actor`, the names known, a
/// body GGG sent).
const DATA: [&str; 15] = [
    "request", "argv", "query", "params", "route", "token", "term", "node", "examples", "readings",
    "patterns", "current", "actor", "accounts", "body",
];

/// Every string of `doc` that is the tool's words: each one not under a
/// [`DATA`] key, an offer's `says` and a refusal's `next` among them
/// (the review of #97's Part A, P2).
fn prose_of<'a>(doc: &'a Value, out: &mut Vec<&'a str>) {
    match doc {
        Value::String(s) => out.push(s),
        Value::Array(items) => items.iter().for_each(|v| prose_of(v, out)),
        Value::Object(map) => map
            .iter()
            .filter(|(k, _)| !DATA.contains(&k.as_str()))
            .for_each(|(_, v)| prose_of(v, out)),
        _ => {}
    }
}

/// Where `words` first speak a CLI command or flag outside a person's
/// act, if anywhere.
fn frontend_word(words: &str) -> Option<usize> {
    let bytes = words.as_bytes();
    let starts_word = |at: usize| {
        at == 0 || !(bytes[at - 1].is_ascii_alphanumeric() || matches!(bytes[at - 1], b'_' | b'-'))
    };
    words.char_indices().map(|(at, _)| at).find(|&at| {
        let rest = &words[at..];
        let flag =
            rest.starts_with("--") && rest.as_bytes().get(2).is_some_and(u8::is_ascii_lowercase);
        let command = rest.starts_with("acq ") && !PERSONS.iter().any(|p| rest.starts_with(p));
        (flag || command) && starts_word(at)
    })
}

/// The document a tool's result carries: its one text block, parsed (the
/// server sends no structured content; `spoken.rs` pins that).
pub fn document(result: &Value) -> Value {
    let text = result["content"][0]["text"].as_str().unwrap_or_default();
    serde_json::from_str(text)
        .unwrap_or_else(|e| panic!("a tool result whose text is no document ({e}): {result}"))
}

/// The keys whose value is payload (`PAYLOAD` in the server): what an
/// agent sends back as printed or types as written, never spelled.
const PAYLOAD: [&str; 11] = [
    "offers", "request", "query", "params", "next", "route", "examples", "readings", "current",
    "accounts", "patterns",
];

/// C143: the server's document is the CLI's with this server's spelling
/// after the shared words — the same shape, and each string of prose the
/// CLI's whole, or the CLI's followed by ` — here, ` and the spelling.
/// Payload is the CLI's exactly: every value under a `PAYLOAD` key, a
/// refusal's `readings` among them — but a refusal's `offers`, each a
/// request as its surface sends it (an MCP tool's arguments, naming the
/// tool; a terminal's command line), which hold the same offers saying
/// the same words (round 4 of the 2g build: the adapter spells the
/// request). The first string that differs otherwise, with its path, or
/// `None`.
pub fn spoken_otherwise(server: &Value, cli: &Value, path: &str) -> Option<String> {
    spoken_under(server, cli, path, false)
}

fn spoken_under(server: &Value, cli: &Value, path: &str, payload: bool) -> Option<String> {
    match (server, cli) {
        (Value::String(s), Value::String(c)) => (s != c
            && (payload
                || !s
                    .strip_prefix(c.as_str())
                    .is_some_and(|rest| rest.starts_with(" — here, "))))
        .then(|| {
            format!(
                "{path}: {s:?} is not {c:?} spelled{}",
                if payload { " (payload: exactly)" } else { "" }
            )
        }),
        (Value::Array(s), Value::Array(c)) if s.len() == c.len() => s
            .iter()
            .zip(c)
            .enumerate()
            .find_map(|(i, (s, c))| spoken_under(s, c, &format!("{path}[{i}]"), payload)),
        (Value::Object(s), Value::Object(c))
            if s.keys().collect::<Vec<_>>() == c.keys().collect::<Vec<_>>() =>
        {
            s.iter().find_map(|(k, v)| {
                // an age's words are made at each answer's own time, the
                // epoch beside them the fact (`unaged`)
                if k.ends_with("_age") {
                    return None;
                }
                let here = format!("{path}.{k}");
                if k == "offers" && s.contains_key("kind") {
                    return offered_otherwise(v, &c[k], &here);
                }
                let under = payload || PAYLOAD.contains(&k.as_str());
                spoken_under(v, &c[k], &here, under)
            })
        }
        _ => (server != cli).then(|| format!("{path}: {server} is not {cli}")),
    }
}

/// A refusal's offers on two surfaces: as many, each saying the same.
fn offered_otherwise(server: &Value, cli: &Value, path: &str) -> Option<String> {
    let says = |v: &Value| -> Vec<Value> {
        v.as_array()
            .map(|offers| offers.iter().map(|o| o["says"].clone()).collect())
            .unwrap_or_default()
    };
    (says(server) != says(cli)).then(|| format!("{path}: {server} does not say {cli}"))
}

/// [`spoken_otherwise`] as an assertion.
#[track_caller]
pub fn assert_spoken_as(server: &Value, cli: &Value, what: &str) {
    if let Some(differs) = spoken_otherwise(server, cli, "") {
        panic!("{what}: {differs}");
    }
}
