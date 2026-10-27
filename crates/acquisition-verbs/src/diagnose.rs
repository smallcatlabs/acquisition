//! `acq diagnose` (#79): one document for a bug report, composed of the
//! documents the verbs already produce (C12, two surfaces: nothing here
//! opens a third door) and redacted by one function, [`Redactor::redact`].
//!
//! The sections, each its own verb's document: the build (`acq version
//! --json`'s document and the platform), the daemon (`daemon status`'s,
//! through the observe door: never a spawn, C10), the store (`store
//! status`'s counts for every account the index knows, opened read-only:
//! a diagnosis migrates nothing, C124's door), the rails (the daemon's
//! report when this client's daemon runs, else the world's file), and the
//! tails of the send journal and the daemon log. A section that could not
//! be read says why in its place (C146), and the document is total: a
//! diagnosis that fails is no diagnosis, so building one never fails.
//!
//! What it never prints (`SECURITY.md`): a refresh or access token. The
//! journal holds none by construction (`rails.rs`) and the log is written
//! to hold none; the redaction is a belt over both, and it also takes out
//! what a public issue should not carry: the account names and uuids
//! (from the index, the journal's route keys and the rails' marks), the
//! store root, this world's log directory and the home directory. Each is
//! replaced by a placeholder that tells instances apart (`<account 1>`,
//! `<account 2>`, `<token 1>`), so a reader can still see that two sends
//! were one account's. Character, league and tab names are kept: they are
//! what a fault is usually about.

use std::collections::BTreeMap;
use std::io::{Read as _, Seek as _, SeekFrom};
use std::path::{Path, PathBuf};

use acquisition_protocol::protocol::Response;
use acquisition_protocol::status::RailsStatus;
use acquisition_store::world::{RailsState, World};
use acquisition_store::{Status, Store};
use serde::Serialize;
use serde_json::{Value, json};

use crate::daemon::DaemonReport;
use crate::frontend::Frontend;

/// How many lines of each file's end the document carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tails {
    pub journal: usize,
    pub log: usize,
}

impl Tails {
    /// The defaults: a refresh's worth of sends, and the log lines around
    /// them.
    pub const DEFAULT: Tails = Tails {
        journal: 50,
        log: 100,
    };

    /// Both tails at `n` lines (`--lines N`).
    pub fn both(n: usize) -> Tails {
        Tails { journal: n, log: n }
    }
}

/// The end of one file: its last lines, and whether earlier ones were
/// left out.
#[derive(Clone, Debug, Serialize)]
pub struct Tail<T> {
    pub path: PathBuf,
    pub lines: Vec<T>,
    pub earlier_left_out: bool,
}

/// One line of the journal's tail: a send's JSON line as written, or why
/// a line is not one (a line torn by a daemon that died mid-write). A
/// line that is not JSON is reported, never pasted.
#[derive(Clone, Debug)]
pub enum JournalLine {
    Send(Value),
    Unread(String),
}

impl Serialize for JournalLine {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            JournalLine::Send(line) => line.serialize(s),
            JournalLine::Unread(why) => json!({ "unread": why }).serialize(s),
        }
    }
}

/// The rails as one look found them.
#[derive(Clone, Debug)]
pub enum Rails {
    /// This client's daemon runs: its report (`daemon status`'s rails).
    Daemon(RailsStatus),
    /// No daemon of this client's runs: the world's rails file, `None`
    /// when there is none (nothing held).
    Disk {
        path: PathBuf,
        state: Option<RailsState>,
    },
}

/// One account's store counts, or why they were not read.
#[derive(Clone, Debug)]
pub struct StoreRow {
    pub username: String,
    pub status: Result<Status, String>,
}

/// The diagnosis, unredacted: each section a verb's document or why it
/// was not read. Printed only through [`Diagnosis::document`] (JSON) or
/// [`Diagnosis::redact_text`] (a frontend's text), both redacted.
pub struct Diagnosis {
    /// `acq version --json`'s document with the platform beside it.
    pub version: Value,
    pub daemon: Result<DaemonReport, String>,
    pub store: Result<Vec<StoreRow>, String>,
    pub rails: Result<Rails, String>,
    pub journal: Result<Tail<JournalLine>, String>,
    pub log: Result<Tail<String>, String>,
    redactor: Redactor,
}

/// How many replacements of each kind a printed document carries.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Redactions {
    /// Account names and uuids.
    pub accounts: usize,
    /// The store root, this world's log directory and the home directory.
    pub paths: usize,
    /// Token-shaped strings.
    pub tokens: usize,
}

/// `acq version --json`'s document: what this build is, and the sibling
/// `acqd` a job command would start, as found on disk (C84: the
/// candidate, not the daemon running).
pub fn version_document() -> Value {
    let sibling = acquisition_client::artifact::sibling();
    let mut report = json!({
        "version": acquisition_protocol::VERSION,
        "contract": acquisition_protocol::CONTRACT_REVISION,
        "provider": acquisition_protocol::provider::wanted(),
        "acqd": sibling.as_ref().ok().map(|s| json!({
            "path": s.identity.path,
            "len": s.identity.len,
            "mtime_ns": s.identity.mtime_ns,
        })),
    });
    if let Err(e) = &sibling {
        report["acqd_absent"] = json!(e.to_string());
    }
    report
}

/// The platform this build runs on, as the standard library names it.
pub fn platform() -> String {
    format!("{} {}", std::env::consts::OS, std::env::consts::ARCH)
}

impl Frontend {
    /// #79: everything a bug report needs, in one look that never spawns
    /// or replaces a daemon (C10) and never writes the store (C124's
    /// read-only door). Never fails: a section that could not be read
    /// carries why.
    pub async fn diagnose(&self, tails: Tails) -> Diagnosis {
        let mut version = version_document();
        version["platform"] = json!(platform());

        let daemon = self.daemon().await.map_err(|fault| fault.to_string());
        let world = World::observe_at(&self.root);

        let known = self.accounts().map_err(|f| f.to_string());
        let store = known.as_ref().map_err(Clone::clone).map(|known| {
            known
                .accounts
                .iter()
                .map(|row| StoreRow {
                    status: Store::open_read_only(&row.store)
                        .and_then(|store| store.status())
                        .map_err(|e| format!("{e:#}")),
                    username: row.username.clone(),
                })
                .collect()
        });

        // The running daemon reports its own rails, journal and log
        // (never recomputed from this shell's environment, which may
        // differ: review 2026-09-11); with none, the world's own files.
        let running = match &daemon {
            Ok(DaemonReport::Running { status, .. }) => match status.as_ref() {
                Response::DaemonStatus { rails, log, .. } => Some((rails.clone(), log.clone())),
                _ => None,
            },
            _ => None,
        };
        let no_world = |e: &acquisition_store::world::WorldError| {
            format!("no world to read at {}: {}", e.intended.display(), e.io)
        };
        let (rails, journal_path, log_path) = match running {
            Some((rails, log)) => (
                Ok(Rails::Daemon(rails.clone())),
                rails.journal.map(PathBuf::from).ok_or_else(|| {
                    "the daemon running keeps no journal: it was started with ACQ_JOURNAL=0"
                        .to_string()
                }),
                log.map(PathBuf::from)
                    .ok_or_else(|| "the daemon running reports no log file".to_string()),
            ),
            None => match &world {
                Ok(world) => (
                    rails_on_disk(&world.rails_state_path(self.provider)),
                    Ok(world.journal_path(self.provider)),
                    Ok(world.log_path(self.provider)),
                ),
                Err(e) => (Err(no_world(e)), Err(no_world(e)), Err(no_world(e))),
            },
        };
        let journal = journal_path.and_then(|path| {
            let (lines, earlier_left_out) = tail(&path, tails.journal)?;
            let lines = lines
                .iter()
                .enumerate()
                .map(|(at, line)| match serde_json::from_str::<Value>(line) {
                    Ok(send) => JournalLine::Send(send),
                    Err(e) => JournalLine::Unread(format!(
                        "line {} of this tail is not JSON ({e}), so it is left out: a daemon that stopped mid-write leaves its last line torn",
                        at + 1
                    )),
                })
                .collect();
            Ok(Tail {
                path,
                lines,
                earlier_left_out,
            })
        });
        let log = log_path.and_then(|path| {
            let (lines, earlier_left_out) = tail(&path, tails.log)?;
            Ok(Tail {
                path,
                lines,
                earlier_left_out,
            })
        });

        // Every account name the document can carry: the index's, the
        // journal's route keys (`stash@Alice#1234`) and the rails' marks,
        // so an account the index no longer names is still taken out.
        let mut accounts: Vec<(String, Option<String>)> = Vec::new();
        let mut add = |name: &str, uuid: Option<&str>| {
            if !name.is_empty() && !accounts.iter().any(|(known, _)| known == name) {
                accounts.push((name.to_string(), uuid.map(str::to_string)));
            }
        };
        if let Ok(known) = &known {
            for row in &known.accounts {
                add(&row.username, row.uuid.as_deref());
            }
        }
        if let Ok(tail) = &journal {
            for line in &tail.lines {
                if let JournalLine::Send(send) = line
                    && let Some((_, name)) = send["route"].as_str().and_then(|r| r.split_once('@'))
                {
                    add(name, None);
                }
            }
        }
        if let Ok(Rails::Disk {
            state: Some(state), ..
        }) = &rails
        {
            let mut marked: Vec<&String> = state.refresh_failed_by_account.keys().collect();
            marked.sort();
            for name in marked {
                add(name, None);
            }
        }

        // Absolute paths alone: a relative root (`ACQ_STORE_DIR=store`)
        // would take the word wherever it stands.
        let mut paths = vec![(self.root.clone(), "<store>")];
        if let Ok(world) = &world {
            paths.push((world.root().to_path_buf(), "<store>"));
            if let Some(dir) = world.log_dir(self.provider).parent() {
                paths.push((dir.to_path_buf(), "<logs>"));
            }
        }
        if let Some(home) = std::env::home_dir() {
            paths.push((home, "~"));
        }

        Diagnosis {
            version,
            daemon,
            store,
            rails,
            journal,
            log,
            redactor: Redactor::new(&accounts, &paths),
        }
    }
}

/// The world's rails file as the store's one format reads it (C1: the
/// reset's own reader); none is nothing held.
fn rails_on_disk(path: &Path) -> Result<Rails, String> {
    match std::fs::read_to_string(path) {
        Ok(text) => serde_json::from_str::<RailsState>(&text)
            .map(|state| Rails::Disk {
                path: path.to_path_buf(),
                state: Some(state),
            })
            .map_err(|e| {
                format!(
                    "{} is not the rails state this build reads ({e}); a daemon refuses to start over it, and `acq daemon reset-tripwire` removes it",
                    path.display()
                )
            }),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Rails::Disk {
            path: path.to_path_buf(),
            state: None,
        }),
        Err(e) => Err(format!("{} could not be read: {e}", path.display())),
    }
}

/// The last `n` lines of the file at `path`, read from its end so a
/// journal `ACQ_JOURNAL` keeps unbounded costs no more than its tail, and
/// whether earlier lines were left out.
fn tail(path: &Path, n: usize) -> Result<(Vec<String>, bool), String> {
    let mut file = std::fs::File::open(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            format!("no file at {} yet: nothing has written it", path.display())
        } else {
            format!("{} could not be read: {e}", path.display())
        }
    })?;
    let read = |e: std::io::Error| format!("{} could not be read: {e}", path.display());
    let len = file.metadata().map_err(read)?.len();
    const CHUNK: u64 = 64 * 1024;
    let mut start = len;
    let mut bytes: Vec<u8> = Vec::new();
    // Back a chunk at a time until the bytes hold more than `n` line
    // ends (the extra one starts the first line kept) or the file's start.
    while start > 0 && bytes.iter().filter(|b| **b == b'\n').count() <= n {
        let from = start.saturating_sub(CHUNK);
        let mut chunk = vec![0; (start - from) as usize];
        file.seek(SeekFrom::Start(from)).map_err(read)?;
        file.read_exact(&mut chunk).map_err(read)?;
        chunk.extend_from_slice(&bytes);
        bytes = chunk;
        start = from;
    }
    let text = String::from_utf8_lossy(&bytes);
    let mut lines: Vec<&str> = text.lines().collect();
    // A partial first line, when the read began mid-file, is not a line.
    let mut earlier_left_out = start > 0;
    if start > 0 && !lines.is_empty() {
        lines.remove(0);
    }
    if lines.len() > n {
        lines.drain(..lines.len() - n);
        earlier_left_out = true;
    }
    Ok((
        lines.into_iter().map(str::to_string).collect(),
        earlier_left_out,
    ))
}

impl Diagnosis {
    /// The whole document as JSON, every string and key redacted, with
    /// the count of what was replaced.
    pub fn document(&self) -> Value {
        let section = |read: Result<Value, &String>| match read {
            Ok(value) => value,
            Err(why) => json!({ "unread": why }),
        };
        let store = self.store.as_ref().map(|rows| {
            json!(
                rows.iter()
                    .map(|row| match &row.status {
                        Ok(status) => json!({ "account": row.username, "status": status }),
                        Err(why) => json!({ "account": row.username, "unread": why }),
                    })
                    .collect::<Vec<_>>()
            )
        });
        let rails = self.rails.as_ref().map(|rails| match rails {
            Rails::Daemon(status) => {
                let mut doc = json!(status);
                doc["from"] = json!("daemon");
                doc
            }
            Rails::Disk { path, state } => {
                json!({ "from": "disk", "path": path, "state": state })
            }
        });
        let mut document = json!({
            "version": self.version,
            "daemon": section(self.daemon.as_ref().map(DaemonReport::document)),
            "store": section(store),
            "rails": section(rails),
            "journal": section(self.journal.as_ref().map(|t| json!(t))),
            "log": section(self.log.as_ref().map(|t| json!(t))),
        });
        let mut redactor = self.redactor.clone();
        redactor.redact_value(&mut document);
        document["redactions"] = json!(redactor.made);
        document
    }

    /// A frontend's text of this diagnosis, redacted whole, and the count
    /// of what was replaced in it.
    pub fn redact_text(&self, text: &str) -> (String, Redactions) {
        let mut redactor = self.redactor.clone();
        let text = redactor.redact(text);
        (text, redactor.made)
    }
}

/// The names a token is carried under: the OAuth exchange's query and
/// form fields, the headers, the game's session cookie. A token is the
/// value after one of them, wherever it stands: the name bare or quoted,
/// optional spaces, `=` or `:`, optional spaces, then the value, to its
/// closing quote when quoted, else to the next delimiter
/// (`refresh_token=…`, `refresh_token="…"`, `{"code": "…"}`). The same
/// list is the JSON keys whose string value is a token whatever it looks
/// like. A value of digits alone is no token (a GGG error's `"code": 2`),
/// nor a scheme word before its token (`Authorization: Bearer …`), nor a
/// challenge's parameter (`is_parameter`).
const TOKEN_KEYS: [&str; 10] = [
    "access_token",
    "refresh_token",
    "code",
    "code_verifier",
    "client_secret",
    "poesessid",
    "authorization",
    "cookie",
    "set-cookie",
    "x-access-token",
];

/// The schemes a token follows after a space (`Bearer <token>`).
const TOKEN_SCHEMES: [&str; 1] = ["bearer"];

/// Where `rest` holds a token, when it starts one at a word's start: the
/// length of what is kept before it (the name and separator, or the
/// scheme) and of the token itself.
fn token_at(rest: &str) -> Option<(usize, usize)> {
    let starts = |text: &str, word: &str| {
        text.get(..word.len())
            .is_some_and(|head| head.eq_ignore_ascii_case(word))
    };
    let spaces = |text: &str| text.len() - text.trim_start_matches(' ').len();
    for scheme in TOKEN_SCHEMES {
        if starts(rest, scheme) {
            let after = &rest[scheme.len()..];
            let gap = spaces(after);
            if gap > 0 {
                let head = scheme.len() + gap;
                let value = unquoted_len(&rest[head..]);
                let next = rest[head + value..].chars().next();
                return is_token(&rest[head..head + value], next).then_some((head, value));
            }
        }
    }
    let quote = rest.chars().next().filter(|c| matches!(c, '"' | '\''));
    let body = &rest[quote.map_or(0, char::len_utf8)..];
    let key = TOKEN_KEYS.iter().find(|key| {
        starts(body, key)
            && match quote {
                Some(q) => body[key.len()..].starts_with(q),
                None => !body[key.len()..]
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '-'),
            }
    })?;
    let mut head = rest.len() - body.len() + key.len() + quote.map_or(0, char::len_utf8);
    head += spaces(&rest[head..]);
    if !rest[head..].starts_with(['=', ':']) {
        return None;
    }
    head += 1;
    head += spaces(&rest[head..]);
    let value = match rest[head..].chars().next() {
        Some(q @ ('"' | '\'')) => {
            head += 1;
            rest[head..].find(q).unwrap_or(rest.len() - head)
        }
        _ => unquoted_len(&rest[head..]),
    };
    let next = rest[head + value..].chars().next();
    is_token(&rest[head..head + value], next).then_some((head, value))
}

/// An unquoted value's length: to the first delimiter.
fn unquoted_len(text: &str) -> usize {
    text.chars()
        .take_while(|c| !ends_token(*c))
        .map(char::len_utf8)
        .sum()
}

/// Whether a value after a mark is a token: not empty, not digits alone,
/// not a scheme word, not a challenge's parameter.
fn is_token(value: &str, next: Option<char>) -> bool {
    !value.is_empty()
        && !value.chars().all(|c| c.is_ascii_digit())
        && !TOKEN_SCHEMES.iter().any(|s| value.eq_ignore_ascii_case(s))
        && !is_parameter(value, next)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Account,
    Path,
}

/// One string to replace wherever it stands.
#[derive(Clone, Debug)]
struct Needle {
    text: String,
    placeholder: String,
    kind: Kind,
}

/// The one redaction: its needles, the tokens met so far and the count of
/// what it replaced.
#[derive(Clone, Debug)]
pub struct Redactor {
    needles: Vec<Needle>,
    tokens: BTreeMap<String, usize>,
    made: Redactions,
}

impl Redactor {
    /// Over `accounts` (each name with its uuid when known, in the order
    /// the placeholders number them) and `paths` (each with its
    /// placeholder). An account is matched as its name, the name as its
    /// store file spells it, the name without its `#discriminator` (three
    /// characters or more), and its uuid, each a whole word; a path
    /// wherever it stands. The longest needle is tried first, so a path
    /// under another goes to the nearer placeholder.
    pub fn new(accounts: &[(String, Option<String>)], paths: &[(PathBuf, &str)]) -> Redactor {
        let mut needles = Vec::new();
        for (at, (name, uuid)) in accounts.iter().enumerate() {
            let placeholder = format!("<account {}>", at + 1);
            let file = acquisition_store::account_path(Path::new(""), name)
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned());
            let bare = name.split_once('#').map(|(bare, _)| bare.to_string());
            for text in [
                Some(name.clone()),
                file,
                bare.filter(|b| b.chars().count() >= 3),
            ]
            .into_iter()
            .flatten()
            {
                needles.push(Needle {
                    text,
                    placeholder: placeholder.clone(),
                    kind: Kind::Account,
                });
            }
            if let Some(uuid) = uuid {
                needles.push(Needle {
                    text: uuid.clone(),
                    placeholder: format!("<account {} uuid>", at + 1),
                    kind: Kind::Account,
                });
            }
        }
        for (path, placeholder) in paths.iter().filter(|(p, _)| p.is_absolute()) {
            let mut spellings = vec![path.clone()];
            if let Ok(canonical) = path.canonicalize() {
                spellings.push(canonical);
            }
            for spelling in spellings {
                let text = spelling.display().to_string();
                // A root is never a placeholder's worth: `/` alone would
                // take every separator.
                if text.len() < 2 {
                    continue;
                }
                // A Windows path inside a JSON string in a log line.
                if text.contains('\\') {
                    needles.push(Needle {
                        text: text.replace('\\', r"\\"),
                        placeholder: placeholder.to_string(),
                        kind: Kind::Path,
                    });
                }
                needles.push(Needle {
                    text,
                    placeholder: placeholder.to_string(),
                    kind: Kind::Path,
                });
            }
        }
        needles.sort_by_key(|n| std::cmp::Reverse(n.text.len()));
        needles.dedup_by(|a, b| a.text == b.text);
        Redactor {
            needles,
            tokens: BTreeMap::new(),
            made: Redactions::default(),
        }
    }

    /// The redaction: `text` with every account name and uuid, every path
    /// this redactor holds and every token-shaped string replaced by its
    /// placeholder, in one pass from the left, so a placeholder is never
    /// matched again.
    pub fn redact(&mut self, text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        let mut at = 0;
        'scan: while at < text.len() {
            let rest = &text[at..];
            let starts_word = word_edge(text[..at].chars().next_back());
            for needle in &self.needles {
                if !rest.starts_with(&needle.text) {
                    continue;
                }
                if needle.kind == Kind::Account
                    && !(starts_word && word_edge(rest[needle.text.len()..].chars().next()))
                {
                    continue;
                }
                out.push_str(&needle.placeholder);
                match needle.kind {
                    Kind::Account => self.made.accounts += 1,
                    Kind::Path => self.made.paths += 1,
                }
                at += needle.text.len();
                continue 'scan;
            }
            if starts_word && let Some((head, value)) = token_at(rest) {
                out.push_str(&rest[..head]);
                let token = &rest[head..head + value];
                out.push_str(&self.token(token));
                at += head + value;
                continue 'scan;
            }
            let Some(c) = rest.chars().next() else { break };
            out.push(c);
            at += c.len_utf8();
        }
        out
    }

    /// The placeholder for one token, numbered by first appearance.
    fn token(&mut self, token: &str) -> String {
        self.made.tokens += 1;
        let next = self.tokens.len() + 1;
        let n = *self.tokens.entry(token.to_string()).or_insert(next);
        format!("<token {n}>")
    }

    /// [`Redactor::redact`] over every string and key of a JSON value; a
    /// string under a token's key is a token whatever it looks like.
    pub fn redact_value(&mut self, value: &mut Value) {
        match value {
            Value::String(text) => *text = self.redact(text),
            Value::Array(items) => items.iter_mut().for_each(|v| self.redact_value(v)),
            Value::Object(map) => {
                let entries = std::mem::take(map);
                for (key, mut v) in entries {
                    match &v {
                        Value::String(token)
                            if !token.is_empty()
                                && TOKEN_KEYS.contains(&key.to_ascii_lowercase().as_str()) =>
                        {
                            let token = token.clone();
                            v = Value::String(self.token(&token));
                        }
                        _ => self.redact_value(&mut v),
                    }
                    map.insert(self.redact(&key), v);
                }
            }
            _ => {}
        }
    }

    /// What this redactor has replaced so far.
    pub fn made(&self) -> Redactions {
        self.made
    }
}

/// Whether the character beside a match leaves it a whole word: none, or
/// one that is not a letter, a digit or `_`.
fn word_edge(c: Option<char>) -> bool {
    !c.is_some_and(|c| c.is_alphanumeric() || c == '_')
}

/// A challenge's parameter after a scheme (`Bearer error=…`, which a
/// 401's `www-authenticate` carries and the journal keeps as evidence):
/// a name of letters and `_`, `=`, and a value, quoted or not (`next` is
/// the character after the run). A token's base64 padding ends it, so
/// `abc==` before a delimiter is a token still.
fn is_parameter(text: &str, next: Option<char>) -> bool {
    text.split_once('=').is_some_and(|(name, value)| {
        !name.is_empty()
            && name.chars().all(|c| c.is_ascii_alphabetic() || c == '_')
            && (!value.trim_start_matches('=').is_empty() || next == Some('"'))
    })
}

/// What ends a token after its mark.
fn ends_token(c: char) -> bool {
    c.is_whitespace()
        || matches!(
            c,
            '&' | '"' | '\'' | ';' | ',' | '#' | ')' | '}' | ']' | '<' | '>'
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn redactor() -> Redactor {
        Redactor::new(
            &[
                ("Alice#1234".into(), Some("u-alice-uuid".into())),
                ("Bob#5678".into(), None),
            ],
            &[
                (PathBuf::from("/srv/acq/store"), "<store>"),
                (PathBuf::from("/home/al"), "~"),
            ],
        )
    }

    /// An account is a whole word: its name inside another word stays,
    /// and the store file's spelling of it goes.
    #[test]
    fn an_account_is_replaced_as_a_whole_word_in_each_spelling() {
        let mut r = redactor();
        assert_eq!(
            r.redact("route stash@Alice#1234 and stash@Bob#5678; Alicex stays; Alice goes"),
            "route stash@<account 1> and stash@<account 2>; Alicex stays; <account 1> goes"
        );
        assert_eq!(
            r.redact("/srv/acq/store/mock/Alice_1234.db opened (u-alice-uuid)"),
            "<store>/mock/<account 1>.db opened (<account 1 uuid>)"
        );
        assert_eq!(
            r.made(),
            Redactions {
                accounts: 5,
                paths: 1,
                tokens: 0
            }
        );
    }

    /// The nearer path wins, and a placeholder is never matched again.
    #[test]
    fn a_path_under_another_takes_the_nearer_placeholder() {
        let mut r = Redactor::new(
            &[("store#1".into(), None)],
            &[
                (PathBuf::from("/home/al/s"), "<store>"),
                (PathBuf::from("/home/al"), "~"),
            ],
        );
        assert_eq!(r.redact("/home/al/s/x and /home/al/y"), "<store>/x and ~/y");
    }

    /// Each token-shaped string is replaced after its mark, one number per
    /// distinct value; a mark with nothing after it is left.
    #[test]
    fn a_token_after_its_mark_is_numbered_by_value() {
        let mut r = redactor();
        assert_eq!(
            r.redact("GET /x?code=abc&state=s Authorization: Bearer xyz; again code=abc; code=&x"),
            "GET /x?code=<token 1>&state=s Authorization: Bearer <token 2>; again code=<token 1>; code=&x"
        );
        assert_eq!(
            r.redact("Bearer error=\"invalid_token\" and Bearer pad==; done"),
            "Bearer error=\"invalid_token\" and Bearer <token 3>; done"
        );
        // The review's two lines (r4, finding 1): a quoted value after
        // `=`, and JSON's spacing; a GGG error's numeric code stays, and
        // the cookie's name inside a header's value is still found.
        assert_eq!(
            r.redact(r#"refresh_token="q1" then {"code": "q2"} and {"error":{"code": 2}}"#),
            r#"refresh_token="<token 4>" then {"code": "<token 5>"} and {"error":{"code": 2}}"#
        );
        assert_eq!(
            r.redact("set-cookie: POESESSID=c1; HttpOnly"),
            "set-cookie: POESESSID=<token 6>; HttpOnly"
        );
        let mut value =
            json!({ "access_token": "plain", "nested": [{ "Alice#1234": "refresh_token=r1" }] });
        r.redact_value(&mut value);
        assert_eq!(
            value,
            json!({ "access_token": "<token 7>", "nested": [{ "<account 1>": "refresh_token=<token 8>" }] })
        );
    }

    /// The tail keeps the last lines whole and says when earlier ones were
    /// left out, across a chunk's edge.
    #[test]
    fn a_tail_keeps_the_last_lines_and_says_what_it_left_out() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("log");
        let text: String = (0..20_000).map(|i| format!("line {i}\n")).collect();
        std::fs::write(&path, text).unwrap();
        let (lines, left) = tail(&path, 3).unwrap();
        assert_eq!(
            (lines, left),
            (
                vec![
                    "line 19997".into(),
                    "line 19998".into(),
                    "line 19999".into()
                ],
                true
            )
        );
        let (lines, left) = tail(&path, 30_000).unwrap();
        assert_eq!((lines.len(), left), (20_000, false));
        let (lines, left) = tail(&path, 0).unwrap();
        assert_eq!((lines.len(), left), (0, true));
        assert!(
            tail(&dir.path().join("none"), 3)
                .unwrap_err()
                .contains("no file at")
        );
    }
}
