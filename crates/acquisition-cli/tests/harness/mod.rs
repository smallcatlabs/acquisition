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

/// The journey's one table of facts, shared with the MCP's twin.
#[path = "../../../acquisition-client/tests/harness/journey.rs"]
pub mod journey;

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

/// A command a text prints, run as a person pastes it: through `sh`, its
/// leading `acq` the binary under test, under the isolation of `base`
/// (the world at `base/store`).
pub fn printed(base: &Path, command: &str) -> Command {
    let mut cmd = sh_printed(command);
    isolate(&mut cmd, base);
    cmd
}

/// [`printed`] over a world seeded by hand, as [`offline_command`]: the
/// world is `world` itself, and no daemon is spawned.
pub fn offline_printed(world: &Path, command: &str) -> Command {
    let mut cmd = sh_printed(command);
    isolate_at(&mut cmd, world, world);
    cmd.env("ACQ_NO_SPAWN", "1");
    cmd
}

fn sh_printed(command: &str) -> Command {
    let rest = command
        .strip_prefix("acq ")
        .unwrap_or_else(|| panic!("not an acq command: {command}"));
    let mut cmd = Command::new("sh");
    cmd.arg("-c")
        .arg(format!("\"$ACQ\" {rest}"))
        .env("ACQ", acq_exe());
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

/// [`start_daemon`] with `env` on the daemon's own command alone (the
/// journal, [`journal_on`]).
pub fn start_daemon_with(base: &Path, env: &[(&str, &str)]) -> (Proc, u64) {
    shared::start_daemon_with(acq_exe(), &acqd(), base, env)
}

/// The daemon a command under `base` spawned lazily, stopped when the
/// test ends, failure included: a test owns what it starts, and a daemon
/// left to its idle exit holds its database, lock and log open past the
/// scratch's removal. Every harness login returns one ([`login`]), so a
/// test that logs in cannot forget it; [`stop_on_drop`] is for a daemon
/// another command spawned. Held after the scratch, so it drops first. A
/// stop with no daemon to stop is nothing.
///
/// After the stop the guard counts what is left by the harness's own
/// accounting, the endpoints bound under the test's scratch runtime
/// directory ([`sockets_under`]): a daemon's socket derives from its
/// world into that directory and nothing else binds there (C83). The
/// stop waits for the daemon's connection to close, for at most the
/// client's `STOPPED_WITHIN` (20 s), and returns success past that bound
/// all the same. On Unix the daemon removes its socket before it exits,
/// so the first look finds it gone; on Windows a pipe's name lasts until
/// the system has closed the last handle of the ended process, which can
/// come after the stop's own connection ends (#125's Windows run
/// 40886043255, two stops returned in seconds with the pipe still
/// listed). So the guard looks again until the endpoints are gone, for at
/// most `STOPPED_WITHIN` more: a liveness bound on a path that takes
/// milliseconds, never the assertion (C137), and a daemon that stayed is
/// still named when it passes. It counts endpoints, not processes: a
/// daemon that removed its endpoint and lingered would not be seen. One
/// left fails a test that is not already failing, naming it; a failing
/// one prints it beside the failure, since a second panic in a drop
/// aborts.
#[must_use = "the guard stops the daemon its login started when the test ends; \
              bind it to a named local, or drop it where the test says why"]
pub struct StopOnDrop(Option<PathBuf>);

pub fn stop_on_drop(base: &Path) -> StopOnDrop {
    StopOnDrop(Some(base.to_path_buf()))
}

impl Drop for StopOnDrop {
    fn drop(&mut self) {
        let Some(base) = &self.0 else {
            return;
        };
        let _ = acq(base, &["daemon", "stop"]);
        let deadline = std::time::Instant::now() + acquisition_client::client::STOPPED_WITHIN;
        let left = loop {
            let left = sockets_under(&scratch_tmp(base));
            if left.is_empty() || std::time::Instant::now() >= deadline {
                break left;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        };
        if left.is_empty() {
            return;
        }
        let said = format!(
            "a daemon of the test under {} outlived its stop: {left:?}",
            base.display()
        );
        if std::thread::panicking() {
            eprintln!("{said}");
        } else {
            panic!("{said}");
        }
    }
}

/// The guard for the daemon a login under `base` is about to start: inert
/// when a daemon already runs in the world, the test's own
/// ([`start_daemon`]) or an earlier login's, since that one is not this
/// login's to stop. Asked of `acq daemon status --json`, which never
/// spawns, and not read off the runtime directory: on Unix a killed
/// daemon's socket file outlives it.
fn guard_for_login(base: &Path) -> StopOnDrop {
    let status = sole_json(&acq(base, &["daemon", "status", "--json"]));
    if status["running"] == true {
        StopOnDrop(None)
    } else {
        stop_on_drop(base)
    }
}

/// The scripted mock login as `user`, returning the guard that stops the
/// daemon it started ([`StopOnDrop`]).
pub fn login(base: &Path, user: &str) -> StopOnDrop {
    login_with(base, user, &[])
}

/// [`login`] with `env` set on the `acq auth` that starts the daemon: a
/// knob the daemon reads at its start goes here.
pub fn login_with(base: &Path, user: &str, env: &[(&str, &str)]) -> StopOnDrop {
    let guard = guard_for_login(base);
    shared::login(acq_exe(), base, Some(user), env);
    guard
}

/// The scripted mock login as the mock's own user, `env` set on the `acq
/// auth` that starts the daemon.
pub fn login_as_mock(base: &Path, env: &[(&str, &str)]) -> StopOnDrop {
    let guard = guard_for_login(base);
    shared::login(acq_exe(), base, None, env);
    guard
}

// ---- goldens (C122) -------------------------------------------------------

/// A transcript of `acq`'s text commands for a golden: each command as
/// `$ acq <args>`, the facts it must hold as a `# holds` line, its stdout
/// and its stderr apart, and its exit status when not 0. Each command's
/// facts are checked against its own raw output as it runs, before any
/// normalising and whatever the update switch says, so a regeneration
/// that lost a fact from one command fails even where another still
/// prints it; a path under a placeholder may be written in a fact with
/// forward slashes, as the golden writes it ([`placeholder_paths`]), so
/// one fact holds on Windows (#125). A `--json` command is never recorded: JSON is the contract
/// and is asserted by field.
pub struct Transcript {
    base: PathBuf,
    offline: bool,
    text: String,
    facts: usize,
    /// Knobs set over the isolation list on every command, `None`
    /// removing one ([`Transcript::knob`]).
    knobs: Vec<(String, Option<String>)>,
}

impl Transcript {
    /// Commands under [`command`]'s isolation, the world at `base/store`.
    pub fn new(base: &Path) -> Transcript {
        Transcript {
            base: base.to_path_buf(),
            offline: false,
            text: String::new(),
            facts: 0,
            knobs: Vec::new(),
        }
    }

    /// Every command after this sets `key` to `value` over the isolation
    /// list, or removes it with `None` (`ACQ_JOURNAL`, which the list
    /// sets to `0`, unset so a command reads the world's default).
    pub fn knob(mut self, key: &str, value: Option<&str>) -> Transcript {
        self.knobs
            .push((key.to_string(), value.map(str::to_string)));
        self
    }

    /// Commands under [`offline_command`]'s, the world `world` itself.
    pub fn offline(world: &Path) -> Transcript {
        Transcript {
            offline: true,
            ..Transcript::new(world)
        }
    }

    /// Run `acq <args>`, check that each of `facts` stands in its stdout or
    /// its stderr, record it, and hand back its output.
    pub fn run(&mut self, args: &[&str], facts: &[&str]) -> Output {
        self.run_fed(args, None, facts)
    }

    /// [`Transcript::run`] with `stdin` fed to the command, recorded
    /// beside it.
    pub fn run_fed(&mut self, args: &[&str], stdin: Option<&str>, facts: &[&str]) -> Output {
        assert!(
            !args.contains(&"--json"),
            "acq {args:?}: a --json command is asserted by field, never recorded in a golden"
        );
        let mut cmd = if self.offline {
            offline_command(&self.base, args)
        } else {
            command(&self.base, args)
        };
        for (key, value) in &self.knobs {
            match value {
                Some(value) => cmd.env(key, value),
                None => cmd.env_remove(key),
            };
        }
        let out = match stdin {
            None => cmd.output().expect("spawning acq"),
            Some(fed) => {
                use std::io::Write as _;
                let mut child = cmd
                    .stdin(std::process::Stdio::piped())
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .spawn()
                    .expect("spawning acq");
                child
                    .stdin
                    .take()
                    .expect("piped")
                    .write_all(fed.as_bytes())
                    .expect("feeding acq");
                child.wait_with_output().expect("acq's output")
            }
        };
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        let (shown_out, shown_err) = (placeholder_paths(&stdout), placeholder_paths(&stderr));
        for fact in facts {
            assert!(
                [&stdout, &stderr, &shown_out, &shown_err]
                    .iter()
                    .any(|text| text.contains(fact)),
                "acq {}: the fact {fact:?} is not in what it printed:\n--- stdout\n{stdout}--- stderr\n{stderr}",
                args.join(" ")
            );
        }
        self.facts += facts.len();
        if !self.text.is_empty() {
            self.text.push('\n');
        }
        self.text.push_str("$ acq");
        for arg in args {
            self.text.push(' ');
            self.text.push_str(&shown_word(arg, &self.base));
        }
        self.text.push('\n');
        if let Some(fed) = stdin {
            self.text.push_str(&format!("--- stdin\n{fed}\n"));
        }
        for fact in facts {
            self.text.push_str(&format!("# holds {fact:?}\n"));
        }
        for (label, stream) in [("stdout", &stdout), ("stderr", &stderr)] {
            if !stream.is_empty() {
                self.text.push_str(&format!("--- {label}\n{stream}"));
                if !stream.ends_with('\n') {
                    self.text.push_str("\n(no final newline)\n");
                }
            }
        }
        match out.status.code() {
            Some(0) => {}
            Some(code) => self.text.push_str(&format!("--- exit {code}\n")),
            None => self.text.push_str("--- exit by a signal\n"),
        }
        out
    }

    /// The transcript under the normaliser's named rules ([`normalise`]).
    fn normalised(&self) -> String {
        normalise(&self.text, &self.base)
    }
}

/// The normaliser's eleven named rules, in order: CRLF to LF, `scratch`
/// (which a command's words take before their quoting, [`shown_word`]),
/// `log`, `age`, `eta`, `basis`, `token` and `build` line by line, then `job`,
/// `pid` and `progress` over the whole text, since a job's ordinal is
/// counted across the transcript and a job's progress lines are read
/// together. A `# holds` line is the fact as checked and is kept as
/// written. Normalising a normalised text changes nothing (the test
/// module `normaliser.rs` holds that over every committed golden).
pub fn normalise(text: &str, base: &Path) -> String {
    let text: String = text
        .replace("\r\n", "\n")
        .split_inclusive('\n')
        .map(|line| {
            if is_fact(line) {
                return line.to_string();
            }
            let mut line = scratch_rule(line, base);
            line = log_rule(&line);
            line = age_rule(&line);
            line = eta_rule(&line);
            line = basis_rule(&line);
            line = token_rule(&line);
            build_rule(&line)
        })
        .collect();
    progress_rule(&pid_rule(&job_rule(&text)))
}

/// The rule `build`: this build's package version is `<version>`, its
/// contract revision `<contract>` (a digest of the contract's sources,
/// which moves with every edit to them) and the platform as `acq
/// diagnose` names it, `<platform>` (the golden is one text on every
/// runner).
fn build_rule(line: &str) -> String {
    let platform = format!("{} {}", std::env::consts::OS, std::env::consts::ARCH);
    line.replace(env!("CARGO_PKG_VERSION"), "<version>")
        .replace(acquisition_protocol::CONTRACT_REVISION, "<contract>")
        .replace(&platform, "<platform>")
}

/// A `# holds` line: never normalised.
fn is_fact(line: &str) -> bool {
    line.starts_with("# holds ")
}

/// Whether `text` at byte `at` starts a word: the text's start, or after
/// a character that is not a letter, a digit or `_`.
fn word_starts(text: &str, at: usize) -> bool {
    text[..at]
        .chars()
        .next_back()
        .is_none_or(|c| !(c.is_alphanumeric() || c == '_'))
}

/// The run of ASCII digits at the start of `text` when a word ends after
/// it (none when a letter, a digit or `_` follows), else 0 bytes.
fn digits_word(text: &str) -> usize {
    let n = text.bytes().take_while(u8::is_ascii_digit).count();
    let ends = text[n..]
        .chars()
        .next()
        .is_none_or(|c| !(c.is_alphanumeric() || c == '_'));
    if n > 0 && ends { n } else { 0 }
}

/// The rule `job`: a job id after `job `, `jobs `, `acq result `,
/// `acq status `, `acq cancel ` or `acq priority `, and each id of an
/// en-dash range or a comma list after it, is `<jN>`, N counting distinct
/// ids by first appearance across the whole transcript, command lines
/// included: `applying 9 requests as job 7` and a later `$ acq result 7`
/// share `<j1>`, and `(jobs 8–16)` reads `(jobs <j2>–<j3>)`. Numbered,
/// since a transcript relates its ids; `<jN>`, never `<job N>`, which a
/// second pass would match. A daemon's job counter is stable by timing
/// alone.
fn job_rule(text: &str) -> String {
    const PREFIXES: [&str; 6] = [
        "acq priority ",
        "acq result ",
        "acq status ",
        "acq cancel ",
        "jobs ",
        "job ",
    ];
    let mut ids: Vec<String> = Vec::new();
    let mut ordinal = |id: &str| -> String {
        let n = match ids.iter().position(|known| known == id) {
            Some(i) => i + 1,
            None => {
                ids.push(id.to_string());
                ids.len()
            }
        };
        format!("<j{n}>")
    };
    let mut out = String::with_capacity(text.len());
    for line in text.split_inclusive('\n') {
        if is_fact(line) {
            out.push_str(line);
            continue;
        }
        let mut at = 0;
        while at < line.len() {
            let rest = &line[at..];
            let prefix = PREFIXES
                .iter()
                .find(|p| rest.starts_with(**p) && word_starts(line, at));
            let Some(prefix) = prefix else {
                let c = rest.chars().next().expect("a character");
                out.push(c);
                at += c.len_utf8();
                continue;
            };
            let mut from = at + prefix.len();
            let n = digits_word(&line[from..]);
            if n == 0 {
                out.push_str(prefix);
                at = from;
                continue;
            }
            out.push_str(prefix);
            out.push_str(&ordinal(&line[from..from + n]));
            from += n;
            // the ids of a range or a list after it
            loop {
                let rest = &line[from..];
                let joint = ["–", ", "].into_iter().find(|j| rest.starts_with(j));
                let Some(joint) = joint else { break };
                let n = digits_word(&rest[joint.len()..]);
                if n == 0 {
                    break;
                }
                out.push_str(joint);
                out.push_str(&ordinal(&rest[joint.len()..joint.len() + n]));
                from += joint.len() + n;
            }
            at = from;
        }
    }
    out
}

/// The rule `pid`: `pid ` and its digits are `pid <pid>`, as `daemon
/// stopped (pid 58345)` prints it; a process id is the system's.
fn pid_rule(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for line in text.split_inclusive('\n') {
        if is_fact(line) {
            out.push_str(line);
            continue;
        }
        let mut rest = line;
        while let Some(at) = rest.find("pid ") {
            let after = &rest[at + "pid ".len()..];
            let n = digits_word(after);
            out.push_str(&rest[..at + "pid ".len()]);
            if n > 0 && word_starts(rest, at) {
                out.push_str("<pid>");
                rest = &after[n..];
            } else {
                rest = after;
            }
        }
        out.push_str(rest);
    }
    out
}

/// The rule `progress`: of a command's stderr lines that begin `job
/// <id>`, each job's last is kept and the earlier ones go. The first
/// progress line prints whatever state the first poll saw; the last is
/// the terminal one, a job's `done` or an apply's children tally, and
/// always prints.
fn progress_rule(text: &str) -> String {
    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let job_of = |line: &str| -> Option<String> {
        let rest = line.strip_prefix("job ")?;
        let end = rest.find([' ', ':']).unwrap_or(rest.len());
        (end > 0).then(|| rest[..end].to_string())
    };
    let mut keep = vec![true; lines.len()];
    let mut i = 0;
    while i < lines.len() {
        if lines[i] != "--- stderr\n" {
            i += 1;
            continue;
        }
        let start = i + 1;
        let mut end = start;
        while end < lines.len()
            && !lines[end].starts_with("--- ")
            && !lines[end].starts_with("$ acq")
        {
            end += 1;
        }
        let mut last: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for (at, line) in lines.iter().enumerate().take(end).skip(start) {
            if let Some(job) = job_of(line) {
                last.insert(job, at);
            }
        }
        for (at, line) in lines.iter().enumerate().take(end).skip(start) {
            if let Some(job) = job_of(line) {
                keep[at] = last[&job] == at;
            }
        }
        i = end;
    }
    lines
        .iter()
        .zip(keep)
        .filter(|(_, kept)| *kept)
        .map(|(line, _)| *line)
        .collect()
}

/// The words of a printed command, as a shell would split them: on
/// spaces outside single quotes, the quotes taken off (the frontends
/// quote with `shell_quoted`, single quotes alone).
pub fn shell_words(command: &str) -> Vec<String> {
    let (mut words, mut word, mut quoted) = (Vec::new(), String::new(), false);
    for c in command.chars() {
        match c {
            '\'' => quoted = !quoted,
            ' ' if !quoted => {
                if !word.is_empty() {
                    words.push(std::mem::take(&mut word));
                }
            }
            c => word.push(c),
        }
    }
    if !word.is_empty() {
        words.push(word);
    }
    words
}

/// A word as a shell reads it back: bare where it is safe, else quoted.
fn shell_word(word: &str) -> String {
    if !word.is_empty()
        && word
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "_-./=:,@%+".contains(c))
    {
        word.to_string()
    } else {
        format!("'{}'", word.replace('\'', r"'\''"))
    }
}

/// A command's word as the transcript shows it: the `scratch` rule runs
/// on the word first, and the quoting is judged on that canonical form,
/// `<scratch>` standing for a path of safe characters, so a separator
/// that provoked quoting on one platform (Windows' backslash) provokes
/// none: `--apply=<scratch>/plan.json` on every platform.
pub fn shown_word(word: &str, base: &Path) -> String {
    let shown = scratch_rule(word, base);
    let judged = shown.replace("<scratch>", "scratch");
    if shell_word(&judged) == judged {
        shown
    } else {
        shell_word(&shown)
    }
}

/// The rule `scratch`: the test's base directory, as given, canonicalised
/// and, on Windows, with its backslashes doubled as JSON writes them, is
/// `<scratch>`; and a path under it is written with forward slashes to
/// its end (whitespace, a quote, `)`, `,` or `;`), whatever separator it
/// was read with (one backslash, or two as JSON writes them), so one
/// golden reads the same on Windows; so is a path under the placeholders
/// `acq diagnose` prints in place of the store and log directories,
/// `<store>` and `<logs>`.
fn scratch_rule(text: &str, base: &Path) -> String {
    let mut spellings = vec![base.display().to_string()];
    if let Ok(canonical) = std::fs::canonicalize(base) {
        spellings.push(canonical.display().to_string());
    }
    if cfg!(windows) {
        let doubled: Vec<String> = spellings.iter().map(|s| s.replace('\\', r"\\")).collect();
        spellings.extend(doubled);
    }
    // the longest first, so a canonical path is never half replaced
    spellings.sort_by_key(|s| std::cmp::Reverse(s.len()));
    let mut text = text.to_string();
    for spelling in spellings {
        text = text.replace(&spelling, "<scratch>");
    }
    placeholder_paths(&text)
}

/// Each path under `<scratch>`, `<store>` or `<logs>` written with forward
/// slashes to its end ([`forward_slashes_under`]): the scratch rule's
/// second half, which a [`Transcript`]'s facts are also read under.
fn placeholder_paths(text: &str) -> String {
    ["<scratch>", "<store>", "<logs>"]
        .iter()
        .fold(text.to_string(), |text, placeholder| {
            forward_slashes_under(&text, placeholder)
        })
}

/// Each path that starts `placeholder` and a separator, written with
/// forward slashes to its end; any other `placeholder` is left as it is.
fn forward_slashes_under(text: &str, placeholder: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find(placeholder) {
        out.push_str(&rest[..at + placeholder.len()]);
        let tail = &rest[at + placeholder.len()..];
        if !tail.starts_with(['\\', '/']) {
            rest = tail;
            continue;
        }
        let end = tail
            .find(|c: char| c.is_whitespace() || matches!(c, '"' | '\'' | ')' | ',' | ';'))
            .unwrap_or(tail.len());
        out.push_str(&tail[..end].replace("\\\\", "/").replace('\\', "/"));
        rest = &tail[end..];
    }
    out.push_str(rest);
    out
}

/// The rule `log`: a log path under the scratch, `<scratch>`, a
/// separator, `logs`, a separator and the world's id (twelve hex digits)
/// before the next separator, is `<scratch>/logs/<world>/` and the rest
/// of the path to its end, written with forward slashes whatever
/// separator it was read with (one backslash, or two as JSON writes
/// them), so a golden reads the same on Windows. The id is a hash of the
/// world's root (`World::id`), and a test's root is a fresh scratch
/// directory, so it differs run to run; the provider and the file after
/// it stay, so a log path that moves still fails the golden, and a path
/// not under the scratch is never touched.
fn log_rule(text: &str) -> String {
    const SEPARATORS: [&str; 3] = ["\\\\", "\\", "/"];
    let separator = |s: &str| {
        SEPARATORS
            .iter()
            .find(|sep| s.starts_with(**sep))
            .map(|sep| sep.len())
    };
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find("<scratch>") {
        out.push_str(&rest[..at + "<scratch>".len()]);
        let after = &rest[at + "<scratch>".len()..];
        let recognised = separator(after).and_then(|one| {
            let logs = after[one..].strip_prefix("logs")?;
            let two = separator(logs)?;
            let id = logs[two..].get(..12)?;
            let hex = id
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
            let three = separator(&logs[two + 12..])?;
            hex.then_some(after.len() - logs.len() + two + 12 + three)
        });
        let Some(consumed) = recognised else {
            rest = after;
            continue;
        };
        out.push_str("/logs/<world>/");
        // the rest of the path, to its end, with forward slashes
        let tail = &after[consumed..];
        let end = tail
            .find(|c: char| c.is_whitespace() || matches!(c, '"' | '\'' | ')' | ',' | ';'))
            .unwrap_or(tail.len());
        out.push_str(&tail[..end].replace("\\\\", "/").replace('\\', "/"));
        rest = &tail[end..];
    }
    out.push_str(rest);
    out
}

/// The rule `age`: `<n>s ago`, `<n>m ago`, `<n>h ago` and `<n>d ago` are
/// `<age> ago`, since a seeded fetch grows older every day. An age padded
/// in a table's cell, two spaces or more after it, keeps the cell's width,
/// the age and those spaces, so `9s ago` and `10s ago` in one column read
/// the same (issue #127), never with fewer than one space; one space after
/// an age is prose or a separator and stays one.
fn age_rule(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find(" ago") {
        let head = &rest[..at];
        let bytes = head.as_bytes();
        let n = bytes.len();
        let mut start = n;
        if n >= 2 && matches!(bytes[n - 1], b's' | b'm' | b'h' | b'd') {
            let mut i = n - 1;
            while i > 0 && bytes[i - 1].is_ascii_digit() {
                i -= 1;
            }
            let bounded = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
            if i < n - 1 && bounded {
                start = i;
            }
        }
        rest = &rest[at + " ago".len()..];
        if start < n {
            out.push_str(&head[..start]);
            out.push_str("<age> ago");
            let pad = rest.len() - rest.trim_start_matches(' ').len();
            if pad > 1 {
                let cell = n - start + " ago".len() + pad;
                out.push_str(&" ".repeat(cell.saturating_sub("<age> ago".len()).max(1)));
                rest = &rest[pad..];
            }
        } else {
            out.push_str(head);
            out.push_str(" ago");
        }
    }
    out.push_str(rest);
    out
}

/// The rule `eta`: a line that ends ` — ~<n>s`, a plan quote's wait, ends
/// ` — ~<eta>s`. The wait is the window's length less the real age of its
/// oldest send, so it moves with the run's speed; its exact seconds are
/// the limiter's unit tests', at fixed instants. Only the line's end is
/// read, behind the dash the quote writes, so a `~<n>s` elsewhere (a
/// token's lifetime, a job's wait) is left as it is, and `ready` stays.
fn eta_rule(text: &str) -> String {
    let (body, newline) = match text.strip_suffix('\n') {
        Some(body) => (body, "\n"),
        None => (text, ""),
    };
    let Some(head) = body.strip_suffix('s') else {
        return text.to_string();
    };
    let digits = head.len() - head.trim_end_matches(|c: char| c.is_ascii_digit()).len();
    match head[..head.len() - digits].strip_suffix(" — ~") {
        Some(before) if digits > 0 => format!("{before} — ~<eta>s{newline}"),
        _ => text.to_string(),
    }
}

/// The rule `basis`: the basis line's store and build digests are
/// `<store>` and `<build>`; the build's changes with every source edit.
fn basis_rule(text: &str) -> String {
    let digest = |line: &str, word: &str, with: &str| -> String {
        let key = format!("{word} ");
        match line.find(&key) {
            Some(at) => {
                let from = at + key.len();
                let to = line[from..].find(' ').map_or(line.len(), |n| from + n);
                format!("{}{with}{}", &line[..from], &line[to..])
            }
            None => line.to_string(),
        }
    };
    text.split_inclusive('\n')
        .map(|line| {
            if line.starts_with("basis   store ") {
                digest(&digest(line, "store", "<store>"), "build", "<build>")
            } else {
                line.to_string()
            }
        })
        .collect()
}

/// The rule `token`: a continuation token, `acq1.` and its encoding, is
/// `acq1.<token>`.
fn token_rule(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find("acq1.") {
        out.push_str(&rest[..at]);
        let after = &rest[at + "acq1.".len()..];
        let n = after
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
            .map(char::len_utf8)
            .sum::<usize>();
        out.push_str(if n > 0 { "acq1.<token>" } else { "acq1." });
        rest = &after[n..];
    }
    out.push_str(rest);
    out
}

/// Compare `transcript` with the golden `<stem>/<name>.txt` under
/// `tests/goldens/` (`file` is the calling test file's `file!()`), or
/// write it under `ACQ_UPDATE_FIXTURES=1`. A transcript whose commands
/// carry no fact is refused, whatever the switch: a golden names what a
/// reader must find (C122).
pub fn golden(file: &str, name: &str, transcript: &Transcript) {
    assert!(
        transcript.facts > 0,
        "golden {name}: no command carries a fact; name the words a reader must find (C122)"
    );
    let stem = Path::new(file)
        .file_stem()
        .and_then(|s| s.to_str())
        .expect("a test file's stem");
    let shown = format!("crates/acquisition-cli/tests/goldens/{stem}/{name}.txt");
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/goldens")
        .join(stem)
        .join(format!("{name}.txt"));
    let actual = format!(
        "# golden {stem}/{name}: acq's text, normalised (C122). Each `# holds` line is a fact \
         checked in that command's own output; regenerate under ACQ_UPDATE_FIXTURES=1 and read \
         the diff.\n\n{}",
        transcript.normalised()
    );
    if std::env::var_os("ACQ_UPDATE_FIXTURES").is_some() {
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("the goldens directory");
        std::fs::write(&path, &actual).expect("writing the golden");
        return;
    }
    let expected = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{shown}: {e}"));
    same_text(
        &expected,
        &actual,
        &shown,
        &format!("ACQ_UPDATE_FIXTURES=1 cargo test -p acquisition-cli --test main {stem}"),
    );
}

// ---- the web UI (C138, C139) ----------------------------------------------

/// One HTTP response, as a test reads it: the status, the headers with
/// their names lowercased, and the body.
pub struct Reply {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Reply {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }
}

/// `acq web` running, the test owning its process: the address and the
/// token read from the URL it printed first on stdout.
pub struct WebUi {
    pub proc: Proc,
    pub addr: String,
    pub token: String,
    pub url: String,
    _stdout: std::io::BufReader<std::process::ChildStdout>,
}

/// Start `acq web` from `cmd` (a [`command`] or [`offline_command`] with
/// its arguments) and read the URL it prints: no wait but the hang
/// ceiling (C137), since the line comes or the test is ended by it.
pub fn start_web(mut cmd: Command) -> WebUi {
    use std::io::BufRead as _;
    let child = cmd
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("spawning acq web");
    let mut proc = Proc(child, acq_exe().to_path_buf());
    let mut stdout = std::io::BufReader::new(proc.0.stdout.take().expect("piped stdout"));
    let mut first = String::new();
    stdout.read_line(&mut first).expect("acq web's first line");
    let url = match serde_json::from_str::<serde_json::Value>(first.trim()) {
        Ok(doc) => doc["url"].as_str().unwrap_or_default().to_string(),
        Err(_) => first.trim().to_string(),
    };
    let rest = url
        .strip_prefix("http://")
        .unwrap_or_else(|| panic!("acq web printed no URL first: {first:?}"));
    let (addr, path) = rest.split_once('/').expect("a path after the address");
    WebUi {
        addr: addr.to_string(),
        token: path.trim_end_matches('/').to_string(),
        url: url.clone(),
        proc,
        _stdout: stdout,
    }
}

impl WebUi {
    /// A request written by hand: the method, the whole path, the headers
    /// beside `Host` (which `host` sets, the server's own when `None`),
    /// and a body.
    pub fn raw(
        &self,
        method: &str,
        path: &str,
        host: Option<&str>,
        headers: &[(&str, &str)],
        body: &str,
    ) -> Reply {
        use std::io::{Read as _, Write as _};
        let mut stream = std::net::TcpStream::connect(&self.addr).expect("connecting to acq web");
        let mut request = format!(
            "{method} {path} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n",
            host.unwrap_or(&self.addr)
        );
        for (k, v) in headers {
            request.push_str(&format!("{k}: {v}\r\n"));
        }
        if !body.is_empty() || method == "POST" {
            request.push_str(&format!("Content-Length: {}\r\n", body.len()));
        }
        request.push_str("\r\n");
        request.push_str(body);
        stream
            .write_all(request.as_bytes())
            .expect("writing the request");
        let mut response = Vec::new();
        stream
            .read_to_end(&mut response)
            .expect("reading the response");
        let response = String::from_utf8_lossy(&response).into_owned();
        let (head, body) = response
            .split_once("\r\n\r\n")
            .unwrap_or((response.as_str(), ""));
        let mut lines = head.lines();
        let status = lines
            .next()
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| panic!("an unparseable response:\n{response}"));
        let headers: Vec<(String, String)> = lines
            .filter_map(|l| l.split_once(": "))
            .map(|(k, v)| (k.to_ascii_lowercase(), v.to_string()))
            .collect();
        let chunked = headers
            .iter()
            .any(|(k, v)| k == "transfer-encoding" && v.contains("chunked"));
        let body = if chunked {
            unchunk(body)
        } else {
            body.to_string()
        };
        Reply {
            status,
            headers,
            body,
        }
    }

    /// GET a page by its path under the token (`""` the form, `"jobs"`).
    pub fn get(&self, path: &str) -> Reply {
        self.raw("GET", &format!("/{}/{path}", self.token), None, &[], "")
    }

    /// The document a page renders, as the server serves it to a request
    /// that accepts JSON.
    pub fn get_json(&self, path: &str) -> serde_json::Value {
        let reply = self.raw(
            "GET",
            &format!("/{}/{path}", self.token),
            None,
            &[("Accept", "application/json")],
            "",
        );
        assert_eq!(reply.status, 200, "GET {path}: {}", reply.body);
        serde_json::from_str(&reply.body).unwrap_or_else(|e| panic!("{path}: {e}: {}", reply.body))
    }

    /// POST a form to a path under the token, from the server's own page
    /// (its Origin), as a browser posts one.
    pub fn post(&self, path: &str, form: &[(&str, &str)]) -> Reply {
        let body = form
            .iter()
            .map(|(k, v)| format!("{}={}", form_encoded(k), form_encoded(v)))
            .collect::<Vec<_>>()
            .join("&");
        let origin = format!("http://{}", self.addr);
        self.raw(
            "POST",
            &format!("/{}/{path}", self.token),
            None,
            &[
                ("Origin", origin.as_str()),
                ("Content-Type", "application/x-www-form-urlencoded"),
            ],
            &body,
        )
    }
}

fn unchunk(body: &str) -> String {
    let mut out = String::new();
    let mut rest = body;
    while let Some((size, after)) = rest.split_once("\r\n") {
        let n = usize::from_str_radix(size.trim(), 16).unwrap_or(0);
        if n == 0 || after.len() < n {
            break;
        }
        out.push_str(&after[..n]);
        rest = after[n..].trim_start_matches("\r\n");
    }
    out
}

/// A form field's value as a browser encodes it.
pub fn form_encoded(text: &str) -> String {
    let mut out = String::new();
    for b in text.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'*' => {
                out.push(b as char)
            }
            b' ' => out.push('+'),
            b => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// The pages of a golden (C122): each page as `GET <path>` or `POST
/// <path>`, the facts it must hold as `# holds` lines — checked against
/// the page's own body as it is recorded — then its status where it is
/// not 200 and its `<main>`, the token, the address and a plan page's
/// envelope masked and the transcript's rules applied.
pub struct Pages {
    base: PathBuf,
    web_token: String,
    web_addr: String,
    text: String,
    facts: usize,
}

impl Pages {
    pub fn new(base: &Path, web: &WebUi) -> Pages {
        Pages {
            base: base.to_path_buf(),
            web_token: web.token.clone(),
            web_addr: web.addr.clone(),
            text: String::new(),
            facts: 0,
        }
    }

    /// Record a page under `label` (`GET search?…`), checking each fact
    /// stands in its body.
    pub fn page(&mut self, label: &str, reply: &Reply, facts: &[&str]) {
        for fact in facts {
            assert!(
                reply.body.contains(fact),
                "{label}: the fact {fact:?} is not on the page:\n{}",
                reply.body
            );
        }
        self.facts += facts.len();
        let main = match (reply.body.find("<main>"), reply.body.find("</main>")) {
            (Some(open), Some(close)) => &reply.body[open..close + "</main>".len()],
            _ => reply.body.as_str(),
        };
        if !self.text.is_empty() {
            self.text.push('\n');
        }
        self.text.push_str(&format!("{label}\n"));
        for fact in facts {
            self.text.push_str(&format!("# holds {fact:?}\n"));
        }
        if reply.status != 200 {
            self.text
                .push_str(&format!("--- status {}\n", reply.status));
        }
        let mut masked = main
            .replace(&self.web_token, "<token>")
            .replace(&self.web_addr, "<addr>");
        // a plan page's envelope is the plan document whole, its times
        // the clock's: `refresh --plan --json` is where it is asserted
        let field = "name=\"plan\" value=\"";
        if let Some(at) = masked.find(field) {
            let from = at + field.len();
            if let Some(len) = masked[from..].find('"') {
                masked.replace_range(from..from + len, "<the plan envelope>");
            }
        }
        self.text.push_str(&masked);
        self.text.push('\n');
    }
}

/// Compare `pages` with the golden `<stem>/<name>.txt` under
/// `tests/goldens/` (`file` the calling test file's `file!()`), or write
/// it under `ACQ_UPDATE_FIXTURES=1`; as [`golden`], a recording with no
/// fact is refused.
pub fn page_golden(file: &str, name: &str, pages: &Pages) {
    assert!(
        pages.facts > 0,
        "page golden {name}: no page carries a fact; name the words a reader must find (C122)"
    );
    let stem = Path::new(file)
        .file_stem()
        .and_then(|s| s.to_str())
        .expect("a test file's stem");
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/goldens")
        .join(stem)
        .join(format!("{name}.txt"));
    let shown = format!("crates/acquisition-cli/tests/goldens/{stem}/{name}.txt");
    let actual = format!(
        "# golden {stem}/{name}: acq web's pages, each page's <main>, normalised (C122). Each \
         `# holds` line is a fact checked in that page as served; regenerate under \
         ACQ_UPDATE_FIXTURES=1 and read the diff.\n\n{}",
        normalise(&job_tables(&pid_words(&pages.text)), &pages.base)
    );
    if std::env::var_os("ACQ_UPDATE_FIXTURES").is_some() {
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("the goldens directory");
        std::fs::write(&path, &actual).expect("writing the golden");
        return;
    }
    let expected = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{shown}: {e}"));
    same_text(
        &expected,
        &actual,
        &shown,
        &format!("ACQ_UPDATE_FIXTURES=1 cargo test -p acquisition-cli --test main {stem}"),
    );
}

/// A job table's columns, each gap one of two spaces: a column is as
/// wide as its widest cell, and the submitter's cell holds a process id
/// whose digits the system picks.
fn job_tables(text: &str) -> String {
    let mut inside = false;
    let mut out = String::with_capacity(text.len());
    for line in text.split_inclusive('\n') {
        if line.starts_with("id  parent") {
            inside = true;
        } else if inside {
            // the table ends at its summary line, or at a line no row starts
            let row = line.starts_with(' ') || line.starts_with(|c: char| c.is_ascii_digit());
            let summary = line
                .split_whitespace()
                .nth(1)
                .is_some_and(|w| w == "jobs:" || w == "job:");
            inside = row && !summary;
        }
        if inside {
            let words: Vec<&str> = line
                .split("  ")
                .map(str::trim)
                .filter(|w| !w.is_empty())
                .collect();
            out.push_str(&words.join("  "));
            out.push('\n');
        } else {
            out.push_str(line);
        }
    }
    out
}

/// A submitter's process id as a job table prints it, `web:4113` or
/// `cli:4090`, masked: a process id is the system's.
fn pid_words(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    loop {
        let next = ["web:", "cli:"]
            .iter()
            .filter_map(|w| rest.find(w).map(|at| (at, w.len())))
            .min();
        let Some((at, len)) = next else {
            out.push_str(rest);
            return out;
        };
        out.push_str(&rest[..at + len]);
        let after = &rest[at + len..];
        let digits = after.bytes().take_while(u8::is_ascii_digit).count();
        if digits > 0 {
            out.push_str("<pid>");
        }
        rest = &after[digits..];
    }
}
