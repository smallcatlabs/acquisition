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

// ---- goldens (C122) -------------------------------------------------------

/// A transcript of `acq`'s text commands for a golden: each command as
/// `$ acq <args>`, the facts it must hold as a `# holds` line, its stdout
/// and its stderr apart, and its exit status when not 0. Each command's
/// facts are checked against its own raw output as it runs, before any
/// normalising and whatever the update switch says, so a regeneration
/// that lost a fact from one command fails even where another still
/// prints it. A `--json` command is never recorded: JSON is the contract
/// and is asserted by field.
pub struct Transcript {
    base: PathBuf,
    offline: bool,
    text: String,
    facts: usize,
}

impl Transcript {
    /// Commands under [`command`]'s isolation, the world at `base/store`.
    pub fn new(base: &Path) -> Transcript {
        Transcript {
            base: base.to_path_buf(),
            offline: false,
            text: String::new(),
            facts: 0,
        }
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
        for fact in facts {
            assert!(
                stdout.contains(fact) || stderr.contains(fact),
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
            self.text.push_str(&shell_word(arg));
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

/// The normaliser's ten named rules, in order: CRLF to LF, `scratch`,
/// `log`, `age`, `basis`, `token` and the version line by line, then `job`,
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
            line = basis_rule(&line);
            line = token_rule(&line);
            line.replace(env!("CARGO_PKG_VERSION"), "<version>")
        })
        .collect();
    progress_rule(&pid_rule(&job_rule(&text)))
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

/// The rule `scratch`: the test's base directory, as given, canonicalised
/// and, on Windows, with its backslashes doubled as JSON writes them, is
/// `<scratch>`.
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
    text
}

/// The rule `log`: the world's id in a log path under the scratch,
/// `<scratch>/logs/` and twelve hex digits before the next separator, is
/// `<world>`. The id is a hash of the world's root (`World::id`), and a
/// test's root is a fresh scratch directory, so it differs run to run;
/// the provider and the file after it stay, so a log path that moves
/// still fails the golden.
fn log_rule(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find("<scratch>") {
        let after = &rest[at + "<scratch>".len()..];
        let id = ["/logs/", "\\logs\\", "\\\\logs\\\\"]
            .iter()
            .find(|sep| after.starts_with(**sep))
            .map(|sep| (sep.len(), &after[sep.len()..]))
            .filter(|(_, tail)| {
                tail.len() > 12
                    && tail.as_bytes()[..12]
                        .iter()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(b))
                    && matches!(tail.as_bytes()[12], b'/' | b'\\')
            });
        let Some((sep, _)) = id else {
            out.push_str(&rest[..at + "<scratch>".len()]);
            rest = after;
            continue;
        };
        out.push_str(&rest[..at + "<scratch>".len() + sep]);
        out.push_str("<world>");
        rest = &after[sep + 12..];
    }
    out.push_str(rest);
    out
}

/// The rule `age`: `<n>s ago`, `<n>m ago`, `<n>h ago` and `<n>d ago` are
/// `<age> ago`, since a seeded fetch grows older every day.
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
        if start < n {
            out.push_str(&head[..start]);
            out.push_str("<age> ago");
        } else {
            out.push_str(head);
            out.push_str(" ago");
        }
        rest = &rest[at + " ago".len()..];
    }
    out.push_str(rest);
    out
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
