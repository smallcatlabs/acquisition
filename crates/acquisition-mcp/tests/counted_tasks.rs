//! The counted tasks (C122, one harness; C153, a finding joins a
//! surface): ten things a player asks, each run through both doors by a
//! script a reader holding only the help would follow, and counted. The
//! golden, `tests/goldens/counted_tasks.txt`, is the reader's journal as
//! the send journal is GGG's: for each task and door the calls, the
//! refusals, the bytes read, whether a job was polled, and what the
//! reader ended up holding of the facts she wanted; then the row's calls,
//! each printed so that pasting it sends what the test sent. No output
//! text: wording has its own goldens. A row that gets worse or better
//! shows as this golden's diff; regenerate it with
//! `ACQ_UPDATE_FIXTURES=1 cargo nextest run -p acquisition-mcp --test main counted_tasks`
//! and read the diff.
//!
//! The rules a script keeps. The reader enforces each one it can:
//!
//! - A row starts having read its door's first turn (`acq --help`;
//!   `initialize` and `tools/list`), counted once as a row of its own.
//! - Every word of a call (a verb, a flag, a tool, an argument's name, a
//!   value, each word of a query) occurs in the task's own words, the
//!   first turn, or what the row was sent before that call; one that does
//!   not fails the test, naming it. Reading help is a call.
//! - A row ends at the first answer, not refused, to a call that asks her
//!   question. It goes on past a refusal: by the refusal's offer, as
//!   printed, where it offers a way on (the reader fails a next call that
//!   takes none), else by the plainest guess. A call before the end is
//!   there only to get a word a later call needs. No row ends on a
//!   refusal (the reader fails one that does).
//! - Where a reader would guess, the script takes the plainest guess and
//!   says so. She reads a verb's help when the first turn did not give her
//!   what the call needs, calls a verb bare when it did, and keeps an
//!   account or a realm once named.
//! - A refusal is a non-zero exit, a JSON-RPC error or a result marked as
//!   an error. A byte is one the reader was sent: a command's two streams;
//!   a tool's answer as its line is received, without the line's
//!   terminator. The test's scratch path counts as [`SCRATCH`]. Counts
//!   are rounded up to two significant figures. A job's polls are
//!   neither calls nor bytes: the golden says only whether there were
//!   any.
//! - A wanted fact is a value said of a subject: in the CLI's text, the
//!   subject's own row and its lines under their section; in the MCP's
//!   document, the subject's own entry with the keys above it. Each is
//!   checked against every answer the row was sent, and the golden says
//!   `reached` when one answer holds them all, `reached over N answers`
//!   when they came across several (N the fewest that together hold
//!   them), or what was missed and what came instead.
//!
//! The seeded tasks run over the seeded world (`harness::seeded`) with no
//! daemon, and hold that none appeared; the refresh runs over the mock,
//! logged in as the journeys log in, and holds the send journal's
//! invariants as they do.

use std::collections::HashSet;
use std::path::Path;
use std::time::{Duration, Instant};

use acquisition_search::answer::posix_quoted;
use serde_json::{Value, json};

use crate::harness;

use harness::Mcp;
use harness::journey::{ACCOUNT as MOCK_ACCOUNT, POLICY as MOCK_POLICY};
use harness::seeded::*;

/// What the test's scratch path counts as in a reader's bytes, so the
/// process's id in it moves no count.
const SCRATCH: &str = "<scratch>";

const GOLDEN: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/goldens/counted_tasks.txt"
);
const REGENERATE: &str =
    "ACQ_UPDATE_FIXTURES=1 cargo nextest run -p acquisition-mcp --test main counted_tasks";

// ---- the reader ------------------------------------------------------------

/// Which door a row goes through.
#[derive(Clone, Copy, PartialEq)]
enum Door {
    Cli,
    Mcp,
}

impl Door {
    fn name(self) -> &'static str {
        match self {
            Door::Cli => "cli",
            Door::Mcp => "mcp",
        }
    }
}

/// One call as the golden lists it.
struct Called {
    shown: String,
    refused: bool,
    bytes: usize,
}

/// One answer the row was sent: the text, and for the MCP its document.
struct Answer {
    text: String,
    doc: Value,
}

/// What a refusal offered, which the next call must take.
enum Owed {
    /// The words of a printed command, or of a flag and its value.
    Words(Vec<Vec<String>>),
    /// The requests of the MCP's offers.
    Requests(Vec<Value>),
}

/// A reader at one door: what it may say (the words it has been sent),
/// what it said, and every answer it was sent.
struct Reader<'a> {
    door: Door,
    base: &'a Path,
    mcp: Option<Mcp>,
    known: HashSet<String>,
    calls: Vec<Called>,
    answers: Vec<Answer>,
    owed: Option<Owed>,
    polled: bool,
}

/// The words of a text, as the rule reads them: split at anything but a
/// letter, a digit or one of `_ . # + % -`, a trailing full stop or comma
/// dropped, so a flag, a field name, an id and an account's name are
/// each one word.
fn words(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| !(c.is_alphanumeric() || "_.#+%-".contains(c)))
        .map(|w| w.trim_end_matches(['.', ',']))
        .filter(|w| !w.is_empty())
}

/// Every string, number and boolean of a JSON value, and its keys.
fn json_words(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::String(s) => out.extend(words(s).map(str::to_string)),
        Value::Number(n) => out.push(n.to_string()),
        Value::Bool(b) => out.push(b.to_string()),
        Value::Array(items) => items.iter().for_each(|v| json_words(v, out)),
        Value::Object(map) => {
            for (k, v) in map {
                out.extend(words(k).map(str::to_string));
                json_words(v, out);
            }
        }
        Value::Null => {}
    }
}

/// A command line printed for a POSIX shell: pasted, it sends these
/// words.
fn printed(args: &[&str]) -> String {
    let quoted: Vec<String> = args.iter().map(|a| posix_quoted(a)).collect();
    format!("acq {}", quoted.join(" "))
}

/// The words of a command line as a refusal prints it, for either
/// declared shell's quoting of the words these tasks meet: a single- or
/// double-quoted run is one word, its quotes dropped.
fn unquoted(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut word = String::new();
    let mut quote = None;
    let mut started = false;
    for c in line.chars() {
        match (quote, c) {
            (None, '\'' | '"') => {
                quote = Some(c);
                started = true;
            }
            (Some(q), c) if c == q => quote = None,
            (None, c) if c.is_whitespace() => {
                if started {
                    out.push(std::mem::take(&mut word));
                    started = false;
                }
            }
            (_, c) => {
                word.push(c);
                started = true;
            }
        }
    }
    if started {
        out.push(word);
    }
    out
}

/// The ways on a CLI refusal prints: each command in backticks, and each
/// flag with its value in parentheses (`(--realm pc)`).
fn cli_offers(text: &str) -> Vec<Vec<String>> {
    let mut offers = Vec::new();
    for (i, part) in text.split('`').enumerate() {
        if i % 2 == 1 && part.starts_with("acq ") {
            offers.push(unquoted(&part["acq ".len()..]));
        }
    }
    for part in text.split('(').skip(1) {
        if let Some(flag) = part.split(')').next()
            && flag.starts_with("--")
        {
            offers.push(unquoted(flag));
        }
    }
    offers
}

/// Rounded up to two significant figures.
fn coarse(bytes: usize) -> usize {
    let mut unit = 1;
    while bytes / unit >= 100 {
        unit *= 10;
    }
    bytes.div_ceil(unit) * unit
}

impl<'a> Reader<'a> {
    /// A reader that has read `task`'s own words and nothing else yet.
    fn new(door: Door, base: &'a Path, task: &Task, env: &[(&str, &str)]) -> Self {
        let mut reader = Reader {
            door,
            base,
            mcp: None,
            known: HashSet::new(),
            calls: Vec::new(),
            answers: Vec::new(),
            owed: None,
            polled: false,
        };
        reader.learn(&format!("{} {} {ALICE}", task.asks, task.knows));
        if door == Door::Mcp {
            reader.mcp = Some(Mcp::start(base, env));
        }
        reader
    }

    fn learn(&mut self, text: &str) {
        self.known.extend(words(text).map(str::to_string));
    }

    fn last(&self) -> &Answer {
        self.answers.last().expect("an answer")
    }

    /// Fail, naming the word, where a call says one its row was never sent.
    fn check(&self, shown: &str, said: &[String]) {
        for word in said {
            if !self.known.contains(word) {
                let what = if word.starts_with('-') {
                    "a flag its row has not read"
                } else {
                    "a word its row has not been sent"
                };
                panic!("{} `{shown}` uses `{word}`, {what}", self.door.name());
            }
        }
    }

    /// Record a call and its answer, and what a refusal offered.
    fn sent(&mut self, shown: String, refused: bool, bytes: usize, answer: Answer) {
        self.learn(&answer.text);
        self.owed = match (refused, self.door) {
            (false, _) => None,
            (true, Door::Cli) => Some(Owed::Words(cli_offers(&answer.text)))
                .filter(|o| !matches!(o, Owed::Words(w) if w.is_empty())),
            (true, Door::Mcp) => answer.doc["offers"].as_array().map(|offers| {
                Owed::Requests(offers.iter().map(|o| o["request"].clone()).collect())
            }),
        };
        self.calls.push(Called {
            shown,
            refused,
            bytes,
        });
        self.answers.push(answer);
    }

    /// The first turn, counted as a call of its own row and learnt.
    fn first_turn(&mut self) {
        match self.door {
            Door::Cli => {
                // where every reader starts: the one call no word rule
                // governs, since nothing has been read before it
                self.learn("--help");
                self.acq(&["--help"]);
            }
            Door::Mcp => {
                let mcp = self.mcp.as_mut().expect("an mcp reader");
                let init = (mcp.initialize.to_string(), mcp.initialize_bytes);
                let (list, bytes) = mcp.exchange("tools/list", json!({}));
                let list = list.to_string();
                let none = Value::Null;
                self.sent(
                    "initialize".into(),
                    false,
                    init.1,
                    Answer {
                        text: init.0,
                        doc: none.clone(),
                    },
                );
                self.sent(
                    "tools/list".into(),
                    false,
                    bytes,
                    Answer {
                        text: list,
                        doc: none,
                    },
                );
            }
        }
    }

    /// One `acq` command, its words checked first, and after a refusal
    /// that offered a way on, that it takes one.
    fn acq(&mut self, args: &[&str]) {
        assert!(self.door == Door::Cli, "acq from the mcp door");
        let shown = printed(args);
        let said: Vec<String> = args
            .iter()
            .flat_map(|a| words(a).map(str::to_string))
            .collect();
        self.check(&shown, &said);
        if let Some(Owed::Words(offers)) = &self.owed {
            let argv: Vec<String> = args.iter().map(|a| a.to_string()).collect();
            let takes = offers.iter().any(|offer| {
                *offer == argv || argv.windows(offer.len()).any(|w| w == offer.as_slice())
            });
            assert!(
                takes,
                "cli `{shown}` takes none of the ways on its refusal offered: {offers:?}"
            );
        }
        let out = harness::acq(self.base, args);
        let text = harness::text(&out);
        let bytes = text
            .replace(self.base.to_str().unwrap_or_default(), SCRATCH)
            .len();
        let answer = Answer {
            text,
            doc: Value::Null,
        };
        self.sent(shown, !out.status.success(), bytes, answer);
    }

    /// One tool call, its words checked first, and after a refusal that
    /// offered a way on, that it sends one.
    fn tool(&mut self, name: &str, args: Value) -> &Value {
        let shown = format!("{name} {args}");
        self.tool_shown(name, args, shown)
    }

    /// [`Reader::tool`], listed in the golden as `shown`: for a value the
    /// door minted in this row, named by where it came from.
    fn tool_shown(&mut self, name: &str, args: Value, shown: String) -> &Value {
        assert!(self.door == Door::Mcp, "a tool from the cli door");
        let mut said: Vec<String> = words(name).map(str::to_string).collect();
        json_words(&args, &mut said);
        self.check(&shown, &said);
        if let Some(Owed::Requests(offers)) = &self.owed {
            assert!(
                offers.contains(&args),
                "mcp `{shown}` sends none of the requests its refusal offered: {offers:?}"
            );
        }
        let mcp = self.mcp.as_mut().expect("an mcp reader");
        let (resp, bytes) = mcp.exchange("tools/call", json!({ "name": name, "arguments": args }));
        let (text, refused) = match resp.get("error") {
            Some(error) => (
                error["message"].as_str().unwrap_or_default().to_string(),
                true,
            ),
            None => (
                resp["result"]["content"][0]["text"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                resp["result"]["isError"] == json!(true),
            ),
        };
        harness::c143_a_text_speaks_no_frontend(name, &text);
        let scratch = self.base.to_str().unwrap_or_default();
        let bytes = bytes - (text.len() - text.replace(scratch, SCRATCH).len());
        let doc = serde_json::from_str(&text).unwrap_or(Value::Null);
        self.sent(shown, refused, bytes, Answer { text, doc });
        &self.last().doc
    }

    /// The request of the last refusal's offer that says `says`.
    fn offer(&self, says: &str) -> Value {
        self.last().doc["offers"]
            .as_array()
            .and_then(|offers| offers.iter().find(|o| o["says"] == json!(says)))
            .map(|o| o["request"].clone())
            .unwrap_or_else(|| panic!("no offer says {says:?}: {}", self.last().text))
    }

    /// Poll a job until it is terminal: neither a call nor bytes, only
    /// whether there were any (the polls are wall-clock).
    fn poll(&mut self, id: &Value) {
        let mcp = self.mcp.as_mut().expect("an mcp reader");
        let deadline = Instant::now() + Duration::from_secs(60);
        loop {
            self.polled = true;
            let (resp, _) = mcp.exchange(
                "tools/call",
                json!({ "name": "job_status", "arguments": { "id": id } }),
            );
            let status: Value = serde_json::from_str(
                resp["result"]["content"][0]["text"]
                    .as_str()
                    .unwrap_or("{}"),
            )
            .unwrap_or(Value::Null);
            match status["state"].as_str() {
                Some("done" | "failed" | "cancelled") => return,
                _ if Instant::now() > deadline => panic!("job never finished: {status}"),
                _ => std::thread::sleep(Duration::from_millis(100)),
            }
        }
    }
}

// ---- the facts -------------------------------------------------------------

/// What a fact is said of.
#[derive(Clone)]
enum Subject {
    /// An item, a tab or a league named, or an id: in the CLI's text the
    /// row that names it, with its lines and its section's label; in the
    /// MCP's document the list entry holding a field that is exactly it,
    /// with the keys above it, or the whole document where it is in no
    /// list.
    Named(String),
    /// A section of the answer: the CLI's label, the MCP's key.
    Part(&'static str),
    /// The answer as a whole: what a one-subject answer says.
    Whole,
}

/// One door's reading of a fact: where it is said, and the values that
/// must all be said there.
#[derive(Clone)]
struct Side {
    subject: Subject,
    values: Vec<String>,
}

/// One wanted fact, as each door says it; its values from the world's
/// constants.
#[derive(Clone)]
struct Fact {
    label: String,
    cli: Side,
    mcp: Side,
}

fn named(subject: &str, values: &[String]) -> Side {
    Side {
        subject: Subject::Named(subject.to_string()),
        values: values.to_vec(),
    }
}

fn part(subject: &'static str, values: &[String]) -> Side {
    Side {
        subject: Subject::Part(subject),
        values: values.to_vec(),
    }
}

fn whole(values: &[String]) -> Side {
    Side {
        subject: Subject::Whole,
        values: values.to_vec(),
    }
}

fn fact(label: impl Into<String>, cli: Side, mcp: Side) -> Fact {
    Fact {
        label: label.into(),
        cli,
        mcp,
    }
}

/// A CLI line's depth: its leading spaces, or for a line at the margin
/// the column where its text resumes after its first word, so a
/// section's rows are siblings and a row's lines its children.
fn depth(line: &str) -> usize {
    let lead = line.len() - line.trim_start().len();
    if lead > 0 {
        return lead;
    }
    match line.find(' ') {
        Some(at) => at + line[at..].len() - line[at..].trim_start().len(),
        None => line.len(),
    }
}

/// The label of the section a CLI line sits in: the last line at or
/// before it that opens with a word at the margin.
fn section<'t>(lines: &[&'t str], at: usize) -> &'t str {
    lines[..=at]
        .iter()
        .rev()
        .find(|l| !l.starts_with(' ') && !l.is_empty())
        .and_then(|l| l.split([' ', ':']).next())
        .unwrap_or("")
}

/// Where a door's answer says things of `subject`, each as one text.
fn contexts(door: Door, subject: &Subject, answer: &Answer) -> Vec<String> {
    match (door, subject) {
        (_, Subject::Whole) => vec![match door {
            Door::Cli => answer.text.clone(),
            Door::Mcp => answer.doc.to_string(),
        }],
        (Door::Cli, Subject::Part(label)) => {
            let lines: Vec<&str> = answer.text.lines().collect();
            let held: Vec<&str> = (0..lines.len())
                .filter(|&i| section(&lines, i) == *label)
                .map(|i| lines[i])
                .collect();
            match held.is_empty() {
                true => Vec::new(),
                false => vec![held.join("\n")],
            }
        }
        (Door::Cli, Subject::Named(name)) => {
            let lines: Vec<&str> = answer.text.lines().collect();
            // an answer whose first line opens with the subject is that
            // subject's, whole: one item shown
            if lines.first().is_some_and(|l| l.starts_with(name.as_str())) {
                return vec![answer.text.clone()];
            }
            let mut out = Vec::new();
            for (i, line) in lines.iter().enumerate() {
                if !line.contains(name.as_str()) {
                    continue;
                }
                // the row's own lines: indented past where the row's text
                // starts; a line at the margin opens another section
                let own = depth(line);
                let rest = lines[i + 1..]
                    .iter()
                    .take_while(|l| l.starts_with(' ') && depth(l) > own);
                let row: Vec<&str> = std::iter::once(*line).chain(rest.copied()).collect();
                out.push(format!("{} {}", section(&lines, i), row.join("\n")));
            }
            out
        }
        (Door::Mcp, Subject::Part(key)) => {
            let mut out = Vec::new();
            under_key(&answer.doc, key, &mut out);
            out
        }
        (Door::Mcp, Subject::Named(name)) => {
            let mut out = Vec::new();
            entries_naming(&answer.doc, name, &mut Vec::new(), &answer.doc, &mut out);
            out
        }
    }
}

/// Every value under `key`, anywhere in `doc`.
fn under_key(doc: &Value, key: &str, out: &mut Vec<String>) {
    match doc {
        Value::Object(map) => {
            for (k, v) in map {
                if k == key {
                    out.push(format!("{k} {v}"));
                }
                under_key(v, key, out);
            }
        }
        Value::Array(items) => items.iter().for_each(|v| under_key(v, key, out)),
        _ => {}
    }
}

/// Each object with a string field that is exactly `name`, as its list
/// entry (`entry`, the root where it is in no list) with the keys above.
fn entries_naming(
    doc: &Value,
    name: &str,
    keys: &mut Vec<String>,
    entry: &Value,
    out: &mut Vec<String>,
) {
    match doc {
        Value::Object(map) => {
            if map.values().any(|v| v.as_str() == Some(name)) {
                out.push(format!("{} {entry}", keys.join(" ")));
            }
            for (k, v) in map {
                keys.push(k.clone());
                entries_naming(v, name, keys, entry, out);
                keys.pop();
            }
        }
        Value::Array(items) => {
            for v in items {
                entries_naming(v, name, keys, v, out);
            }
        }
        _ => {}
    }
}

impl Fact {
    fn side(&self, door: Door) -> &Side {
        match door {
            Door::Cli => &self.cli,
            Door::Mcp => &self.mcp,
        }
    }

    fn held_by(&self, door: Door, answer: &Answer) -> bool {
        let side = self.side(door);
        contexts(door, &side.subject, answer)
            .iter()
            .any(|c| side.values.iter().all(|v| c.contains(v.as_str())))
    }

    /// What came instead, from the answers' own subjects: the subject
    /// named without the value, or not named at all.
    fn instead(&self, door: Door, answers: &[Answer]) -> String {
        let side = self.side(door);
        let said = answers
            .iter()
            .any(|a| !contexts(door, &side.subject, a).is_empty());
        match (&side.subject, said) {
            (Subject::Named(name), true) => format!("{name} named without it"),
            (Subject::Named(name), false) => format!("{name} not named"),
            (Subject::Part(key), true) => format!("its {key} without it"),
            (Subject::Part(key), false) => format!("no {key}"),
            (Subject::Whole, _) => "no answer says it".into(),
        }
    }
}

/// The answer column: `reached` when one answer holds every fact,
/// `reached over N answers` when N together do (the fewest), else each
/// missed fact and what came instead.
fn reached(task: &Task, door: Door, answers: &[Answer]) -> String {
    let holders: Vec<u64> = task
        .wanted
        .iter()
        .map(|f| {
            answers
                .iter()
                .enumerate()
                .filter(|(_, a)| f.held_by(door, a))
                .fold(0u64, |set, (i, _)| set | 1 << i)
        })
        .collect();
    let missed: Vec<&Fact> = task
        .wanted
        .iter()
        .zip(&holders)
        .filter(|(_, h)| **h == 0)
        .map(|(f, _)| f)
        .collect();
    if !missed.is_empty() {
        let missed: Vec<String> = missed
            .iter()
            .map(|f| format!("{}; got: {}", f.label, (task.instead)(door, f, answers)))
            .collect();
        return format!("missed: {}", missed.join(" | "));
    }
    let fewest = (1u64..1 << answers.len())
        .filter(|set| holders.iter().all(|h| h & set != 0))
        .map(|set| set.count_ones())
        .min()
        .unwrap_or(0);
    match fewest {
        1 => "reached".into(),
        n => format!("reached over {n} answers"),
    }
}

/// What came instead of a missed fact, by default: from its subject.
fn instead_of(door: Door, fact: &Fact, answers: &[Answer]) -> String {
    fact.instead(door, answers)
}

// ---- the tasks -------------------------------------------------------------

/// Which world a task runs over.
#[derive(Clone, Copy)]
enum World {
    /// `Seeded::owner()`.
    Whole,
    /// The owner's world with Alice's poe2 scope dropped.
    Pc,
    /// Alice alone, cut to her pc Hardcore.
    Hardcore,
    /// The mock's own world, logged in as the journeys log in.
    Mock,
}

impl World {
    fn seeded(self) -> Option<Seeded> {
        match self {
            World::Whole => Some(Seeded::owner()),
            World::Pc => Some(Seeded::owner().scopes(&[(PC, STANDARD), (PC, HARDCORE)])),
            World::Hardcore => Some(Seeded::owner().alone(ALICE).scopes(&[(PC, HARDCORE)])),
            World::Mock => None,
        }
    }
}

/// One task, in her words.
struct Task {
    n: usize,
    asks: &'static str,
    knows: &'static str,
    world: World,
    wanted: Vec<Fact>,
    /// What came instead of a missed fact, said from the answer's fields.
    instead: fn(Door, &Fact, &[Answer]) -> String,
    cli: fn(&mut Reader),
    mcp: fn(&mut Reader),
}

/// Hardcore's own items in the seeded world: `h1`'s one, the hostile
/// note's item; the tab `t-gone` is left out of the league's listing.
const HARDCORE_ITEMS: u64 = 1;

/// An item's life total as each door says it of the item.
fn life_total(name: &str, total: u64) -> Fact {
    fact(
        format!("{name}, {total} life"),
        named(name, &[format!("pseudo.total_life {total}")]),
        named(
            name,
            &[format!("\"name\":\"pseudo.total_life\",\"value\":{total}")],
        ),
    )
}

/// An item's life line as each door says it of the item.
fn life_line(name: &str, total: u64) -> Fact {
    let line = format!("+{total} to maximum Life");
    fact(
        format!("{name}, {line}"),
        named(name, std::slice::from_ref(&line)),
        named(name, &[format!("\"text\":\"{line}\"")]),
    )
}

/// The ring whose life total cannot be decided, said so of it.
/// The MCP names it by its id: an example's name holds its base too.
fn hex_band_undecided() -> Fact {
    let undecided = vec!["undecided".to_string()];
    fact(
        format!("{HEX_BAND} undecided"),
        named(HEX_BAND, &undecided),
        named(HEX_BAND_ID, &undecided),
    )
}

fn life_90() -> Vec<Fact> {
    vec![
        life_total(DOOM_LOOP, DOOM_LOOP_LIFE),
        life_total(RUNE_COIL, RUNE_COIL_LIFE),
        life_total(KAOM, KAOM_LIFE),
        hex_band_undecided(),
    ]
}

/// A league held, with its realm.
fn league_on(league: &str, realm: &str) -> Fact {
    fact(
        format!("{league} on {realm}"),
        named(league, &[format!("{league} ({realm}")]),
        named(league, &[format!("\"realm\":\"{realm}\"")]),
    )
}

/// The searches that find an item's id by its name, as tasks 5 and 9
/// make them: the help, the fields, then the name.
fn cli_find(r: &mut Reader, query: &str) {
    r.acq(&["search", "--help"]);
    // the help's place to start, which lists `name`
    r.acq(&["search", "--describe", "fields"]);
    r.acq(&["--account", ALICE, "search", query]);
    // her words name no game: everything
    r.acq(&["--account", ALICE, "search", "--realm", "all", query]);
}

fn mcp_find(r: &mut Reader, query: &str) {
    r.tool("describe", json!({ "names": ["fields"] }));
    r.tool(
        "search",
        json!({ "query": { "text": query }, "scope": { "account": ALICE } }),
    );
    // her words name no game: everything
    let all = r.offer("realm all");
    r.tool("search", all);
}

fn tasks() -> Vec<Task> {
    let s = |v: &str| v.to_string();
    let sockets_cli = {
        let mut groups: Vec<Vec<&str>> = Vec::new();
        for (group, colour) in KAOM_SOCKETS {
            match groups.get_mut(group as usize) {
                Some(g) => g.push(colour),
                None => groups.push(vec![colour]),
            }
        }
        groups
            .iter()
            .map(|g| g.join("-"))
            .collect::<Vec<_>>()
            .join(" ")
    };
    let sockets_mcp = json!(
        KAOM_SOCKETS
            .iter()
            .map(|(group, colour)| json!({ "group": group, "colour": colour }))
            .collect::<Vec<_>>()
    );
    vec![
        Task {
            n: 1,
            asks: "What do I hold, and how old is it?",
            knows: "",
            world: World::Whole,
            wanted: vec![
                league_on(STANDARD, PC),
                league_on(HARDCORE, PC),
                league_on(STANDARD, POE2),
                fact(
                    format!("the tab {NEVER_FETCHED_NAME} never fetched"),
                    named(NEVER_FETCHED_NAME, &[s("never")]),
                    named(NEVER_FETCHED_NAME, &[s("\"fetched_at\":null")]),
                ),
                fact("an age", whole(&[s(" ago")]), whole(&[s("_age\":\"")])),
            ],
            instead: instead_of,
            cli: |r| {
                // the first turn's line for what the store holds:
                // `leagues --stored`
                r.acq(&["leagues", "--stored"]);
                r.acq(&["--account", ALICE, "leagues", "--stored"]);
            },
            mcp: |r| {
                r.tool("leagues", json!({}));
                // no way on offered: the field it names, and the plainest value
                r.tool("leagues", json!({ "stored": true }));
                let alice = r.offer("the same request as Alice#1234");
                r.tool("leagues", alice);
            },
        },
        Task {
            n: 2,
            asks: "Which items have at least 90 life in all?",
            knows: "",
            world: World::Pc,
            wanted: life_90(),
            instead: instead_of,
            cli: |r| {
                // a query the first turn does not give: the verb's help
                r.acq(&["search", "--help"]);
                // which names the place to start
                r.acq(&["search", "--describe", "fields"]);
                // no total there: the help's pattern example, with her word
                r.acq(&["search", "--describe=~life"]);
                r.acq(&["search", "pseudo.total_life>=90"]);
                r.acq(&["--account", ALICE, "search", "pseudo.total_life>=90"]);
            },
            mcp: |r| {
                // the server's instructions: begin with search and no arguments
                r.tool("search", json!({}));
                let alice = r.offer("the same request as Alice#1234");
                r.tool("search", alice);
                // the instructions: names read entries whole, fields first
                r.tool("describe", json!({ "names": ["fields"] }));
                // no total there: describe's pattern example, with her word
                r.tool("describe", json!({ "names": ["~life"] }));
                r.tool(
                    "search",
                    json!({ "query": { "text": "pseudo.total_life>=90" },
                            "scope": { "account": ALICE } }),
                );
            },
        },
        Task {
            n: 3,
            asks: "What is in my tab Rings?",
            knows: "the name",
            world: World::Whole,
            wanted: [DOOM_LOOP, RUNE_COIL]
                .iter()
                .map(|ring| {
                    fact(
                        format!("{ring} in {RINGS_TAB_NAME}"),
                        named(ring, &[format!("/ {RINGS_TAB_NAME}")]),
                        named(ring, &[format!("\"name\":\"{RINGS_TAB_NAME}\"")]),
                    )
                })
                .collect(),
            instead: instead_of,
            cli: |r| {
                // her word: `tabs`
                r.acq(&["tabs"]);
                r.acq(&["--account", ALICE, "tabs"]);
            },
            mcp: |r| {
                r.tool("tabs", json!({}));
                let alice = r.offer("the same request as Alice#1234");
                r.tool("tabs", alice);
            },
        },
        Task {
            n: 4,
            asks: "Are the plain Chaos Orbs in my tab \"~price 3 chaos\" priced?",
            knows: "the tab's name",
            world: World::Whole,
            wanted: vec![fact(
                format!("{PRICED_AMOUNT} {PRICED_CURRENCY} on {PRICED_ITEM}"),
                named(PRICED_ITEM, &[format!("{PRICED_AMOUNT} {PRICED_CURRENCY}")]),
                named(
                    PRICED_ITEM,
                    &[
                        format!("\"amount\":\"{PRICED_AMOUNT}\""),
                        format!("\"{PRICED_CURRENCY}\""),
                    ],
                ),
            )],
            instead: instead_of,
            cli: |r| {
                // her word: `price`, the listing state
                r.acq(&["price"]);
                r.acq(&["--account", ALICE, "price"]);
            },
            mcp: |r| {
                r.tool("price_status", json!({}));
                let alice = r.offer("the same request as Alice#1234");
                r.tool("price_status", alice);
            },
        },
        Task {
            n: 5,
            asks: "Show me my Kaom's Heart whole",
            knows: "the name",
            world: World::Whole,
            wanted: vec![
                fact(
                    format!("{KAOM}, {KAOM_LIFE_LINE}"),
                    named(KAOM, &[s(KAOM_LIFE_LINE)]),
                    named(KAOM, &[format!("\"text\":\"{KAOM_LIFE_LINE}\"")]),
                ),
                fact(
                    format!("{KAOM}, its sockets {sockets_cli}"),
                    named(KAOM, std::slice::from_ref(&sockets_cli)),
                    named(KAOM, &[format!("\"sockets\":{sockets_mcp}")]),
                ),
            ],
            instead: instead_of,
            cli: |r| {
                // `show` takes an id: find it by her name for it
                r.acq(&["show", "--help"]);
                cli_find(r, "name=\"Kaom's Heart\"");
                r.acq(&["--account", ALICE, "show", "kaom"]);
            },
            mcp: |r| {
                // `show` takes an id: find it by her name for it
                mcp_find(r, "name=\"Kaom's Heart\"");
                r.tool("show", json!({ "id": "kaom", "account": ALICE }));
            },
        },
        Task {
            n: 6,
            asks: "Bring my stash up to date",
            knows: "",
            world: World::Mock,
            wanted: vec![fact(
                "a plan that names no request",
                whole(&[s("nothing to do")]),
                part("plan", &[s("\"logical_requests\":0")]),
            )],
            instead: instead_of,
            // Her question is asked by the plan: "until the plan says
            // nothing to do", the first turn's words. A plan that names
            // requests prints the apply as its way on, and each apply the
            // next plan, which the row takes as it takes a refusal's offer.
            cli: |r| {
                r.acq(&["refresh", "--plan"]);
                r.acq(&[
                    "policy",
                    "set",
                    "--account",
                    MOCK_ACCOUNT,
                    "--if-revision",
                    "0",
                    MOCK_POLICY,
                ]);
                let scope = [
                    "--account",
                    MOCK_ACCOUNT,
                    "--realm",
                    "pc",
                    "--league",
                    "Standard",
                ];
                for _ in 0..6 {
                    r.acq(&[&["refresh", "--plan"][..], &scope].concat());
                    if r.last().text.contains("nothing to do") {
                        break;
                    }
                    r.acq(&[&["refresh", "--apply"][..], &scope].concat());
                }
            },
            mcp: |r| {
                r.tool("refresh_plan", json!({}));
                // no way on offered: the policy as set_sync_policy's own
                // example gives it
                let policy: Value = serde_json::from_str(MOCK_POLICY).expect("JSON");
                r.tool("set_sync_policy", json!({ "value": policy }));
                for _ in 0..6 {
                    let plan = r.tool("refresh_plan", json!({}))["plan"].clone();
                    if plan["logical_requests"] == json!(0) {
                        break;
                    }
                    let job = r.tool_shown(
                        "apply_plan",
                        json!({ "plan": plan }),
                        "apply_plan {\"plan\":<the plan refresh_plan returned>}".into(),
                    )["job_id"]
                        .clone();
                    r.poll(&job);
                    r.tool_shown(
                        "job_result",
                        json!({ "id": job }),
                        "job_result {\"id\":<the job apply_plan returned>}".into(),
                    );
                }
            },
        },
        Task {
            n: 7,
            asks: "I play one league: what is priced?",
            knows: "",
            world: World::Hardcore,
            wanted: vec![fact(
                format!("{HARDCORE}, {HARDCORE_ITEMS} item"),
                named(HARDCORE, &[format!("{HARDCORE_ITEMS} item in {HARDCORE}")]),
                named(
                    HARDCORE,
                    &[format!("\"counts\":{{\"items\":{HARDCORE_ITEMS},")],
                ),
            )],
            // the league the answer is of, and its count of items
            instead: |door, _, answers| {
                let answer = answers.last().expect("an answer");
                match door {
                    Door::Cli => {
                        let head: Vec<&str> = answer.text.split_whitespace().take(4).collect();
                        format!("{}, {} item", head[3].trim_end_matches(':'), head[0])
                    }
                    Door::Mcp => format!(
                        "{}, {} item",
                        answer.doc["league"].as_str().unwrap_or_default(),
                        answer.doc["counts"]["items"]
                    ),
                }
            },
            cli: |r| {
                // her word: `price`, the listing state
                r.acq(&["price"]);
            },
            mcp: |r| {
                r.tool("price_status", json!({}));
            },
        },
        Task {
            n: 8,
            asks: "Which items have at least 90 life in all? I hold both games.",
            knows: "",
            world: World::Whole,
            wanted: [
                life_90(),
                vec![fact(
                    format!("{POE2} searched, said in the scope"),
                    part("scope", &[s("· all ·")]),
                    part("scope", &[s("\"realm\":\"all\"")]),
                )],
            ]
            .concat(),
            instead: instead_of,
            cli: |r| {
                r.acq(&["search", "--help"]);
                r.acq(&["search", "--describe", "fields"]);
                r.acq(&["search", "--describe=~life"]);
                r.acq(&["search", "pseudo.total_life>=90"]);
                r.acq(&["--account", ALICE, "search", "pseudo.total_life>=90"]);
                // both games: everything
                r.acq(&[
                    "--account",
                    ALICE,
                    "search",
                    "--realm",
                    "all",
                    "pseudo.total_life>=90",
                ]);
            },
            mcp: |r| {
                r.tool("search", json!({}));
                let alice = r.offer("the same request as Alice#1234");
                r.tool("search", alice);
                // both games: everything
                let all = r.offer("realm all");
                r.tool("search", all);
                r.tool("describe", json!({ "names": ["fields"] }));
                r.tool("describe", json!({ "names": ["~life"] }));
                // the scope she has learnt, as she keeps her account
                r.tool(
                    "search",
                    json!({ "query": { "text": "pseudo.total_life>=90" },
                            "scope": { "account": ALICE, "realm": "all" } }),
                );
            },
        },
        Task {
            n: 9,
            asks: "Price my Doom Loop at 10 chaos",
            knows: "the name, the price",
            world: World::Whole,
            wanted: vec![fact(
                "a read after the write says 10 chaos",
                whole(&[s("10 chaos")]),
                part("listing", &[s("\"amount\":\"10\""), s("\"chaos\"")]),
            )],
            instead: instead_of,
            // Her question, as the task's wanted column puts it, is asked by
            // the read after the write: the write is a step, as a word is.
            cli: |r| {
                r.acq(&["price", "--help"]);
                r.acq(&["price", "set", "--help"]);
                // a target is `item/<id>`: find the id by her name for it
                cli_find(r, "name=\"Doom Loop\"");
                r.acq(&[
                    "--account",
                    ALICE,
                    "price",
                    "set",
                    "item/r1",
                    "exact",
                    "10",
                    "chaos",
                ]);
                r.acq(&["--account", ALICE, "price", "show", "item/r1"]);
            },
            mcp: |r| {
                // a target is `item/<id>`: find the id by her name for it
                mcp_find(r, "name=\"Doom Loop\"");
                r.tool(
                    "price_set",
                    json!({ "target": "item/r1", "type": "exact", "amount": "10",
                            "currency": "chaos", "account": ALICE }),
                );
                r.tool(
                    "price_show",
                    json!({ "target": "item/r1", "account": ALICE }),
                );
            },
        },
        Task {
            n: 10,
            asks: "Run my saved search Life rings",
            knows: "its name",
            world: World::Whole,
            wanted: vec![
                life_line(DOOM_LOOP, DOOM_LOOP_LIFE),
                life_line(KAOM, KAOM_LIFE),
            ],
            instead: instead_of,
            cli: |r| {
                // the help says a saved search runs through `search --saved`
                r.acq(&["saved", "--help"]);
                r.acq(&["search", "--saved", "Life rings"]);
                r.acq(&["--account", ALICE, "search", "--saved=Life rings"]);
            },
            mcp: |r| {
                // the tools/list's search takes `saved`
                r.tool("search", json!({ "saved": "Life rings" }));
                let alice = r.offer("the same request as Alice#1234");
                r.tool("search", alice);
            },
        },
    ]
}

// ---- the run -----------------------------------------------------------------

/// One row's numbers and its calls, as the golden holds them.
struct Row {
    task: usize,
    door: Door,
    calls: Vec<Called>,
    polled: bool,
    answer: String,
}

/// Run one task's script at one door, the first turn read first and not
/// counted in the row.
fn script(task: &Task, door: Door, base: &Path, env: &[(&str, &str)]) -> Row {
    let mut reader = Reader::new(door, base, task, env);
    reader.first_turn();
    reader.calls.clear();
    reader.answers.clear();
    match door {
        Door::Cli => (task.cli)(&mut reader),
        Door::Mcp => (task.mcp)(&mut reader),
    }
    let last = reader.calls.last().expect("a row makes a call");
    assert!(
        !last.refused,
        "task {} through the {} ends on a refusal: `{}`",
        task.n,
        door.name(),
        last.shown
    );
    Row {
        task: task.n,
        door,
        answer: reached(task, door, &reader.answers),
        calls: reader.calls,
        polled: reader.polled,
    }
}

fn run_seeded(task: &Task, door: Door, seeded: &Seeded) -> Row {
    let scratch = harness::scratch("counted");
    let base = scratch.0.clone();
    seeded.write(&base.join("store"), harness::now());
    let row = script(task, door, &base, &[("ACQ_NO_SPAWN", "1")]);
    assert!(
        harness::no_daemon_appeared(&base),
        "task {} through the {} started a daemon",
        task.n,
        door.name()
    );
    row
}

fn run_mock(task: &Task, door: Door) -> Row {
    use harness::journey::{GETS, TOKEN_POSTS};
    let scratch = harness::scratch("countmk");
    let base = scratch.0.clone();
    let (knob, journal) = harness::journal_on(&base);
    let (mut daemon, pid) = harness::start_daemon_with(&base, &[(knob, journal.as_str())]);
    harness::login_as_mock(&base);
    let row = script(task, door, &base, &[]);
    let stop = harness::acq(&base, &["daemon", "stop"]);
    assert!(stop.status.success(), "{stop:?}");
    daemon.wait_exit(Duration::from_secs(120));
    harness::assert_journal_holds(&base, pid, &GETS, TOKEN_POSTS);
    row
}

fn first_turn_row(door: Door) -> Row {
    let scratch = harness::scratch("countft");
    let base = scratch.0.clone();
    Seeded::owner().write(&base.join("store"), harness::now());
    let none = Task {
        n: 0,
        asks: "",
        knows: "",
        world: World::Whole,
        wanted: Vec::new(),
        instead: instead_of,
        cli: |_| {},
        mcp: |_| {},
    };
    let mut reader = Reader::new(door, &base, &none, &[("ACQ_NO_SPAWN", "1")]);
    reader.first_turn();
    Row {
        task: 0,
        door,
        calls: reader.calls,
        polled: false,
        answer: "the first turn".into(),
    }
}

fn render(tasks: &[Task], rows: &[Row]) -> String {
    let mut out = format!(
        "# The counted tasks: what each task costs a reader through each door \
         (crates/acquisition-mcp/tests/counted_tasks.rs, whose doc holds the rules). \
         Bytes are rounded up to two significant figures; polls are left out of \
         calls and bytes; a call is printed so that pasting it sends what was sent, \
         a command for a POSIX shell. Regenerate with `{REGENERATE}` and read the \
         diff.\n\n\
         task  door  calls  refused  bytes    polled  answer\n"
    );
    for row in rows {
        let bytes: usize = row.calls.iter().map(|c| c.bytes).sum();
        let refused = row.calls.iter().filter(|c| c.refused).count();
        out.push_str(&format!(
            "{:<5} {:<5} {:<6} {:<8} {:<8} {:<7} {}\n",
            row.task,
            row.door.name(),
            row.calls.len(),
            refused,
            coarse(bytes),
            if row.polled { "yes" } else { "no" },
            row.answer
        ));
    }
    for row in rows {
        let wanted = tasks
            .iter()
            .find(|t| t.n == row.task)
            .map(|t| {
                let labels: Vec<&str> = t.wanted.iter().map(|f| f.label.as_str()).collect();
                format!("{} wanted: {}", t.asks, labels.join("; "))
            })
            .unwrap_or_else(|| "the first turn".into());
        out.push_str(&format!(
            "\n## {} {}: {wanted}\n",
            row.task,
            row.door.name()
        ));
        for call in &row.calls {
            out.push_str(&format!(
                "  {:<7} {:>6}  {}\n",
                if call.refused { "refused" } else { "" },
                coarse(call.bytes),
                call.shown
            ));
        }
    }
    out
}

#[test]
fn counted_tasks_through_both_doors() {
    let tasks = tasks();
    let mut rows = vec![first_turn_row(Door::Cli), first_turn_row(Door::Mcp)];
    for task in &tasks {
        for door in [Door::Cli, Door::Mcp] {
            rows.push(match task.world.seeded() {
                Some(seeded) => run_seeded(task, door, &seeded),
                None => run_mock(task, door),
            });
        }
    }
    let actual = render(&tasks, &rows);
    if std::env::var_os("ACQ_UPDATE_FIXTURES").is_some() {
        std::fs::write(GOLDEN, &actual).unwrap();
    }
    let expected = std::fs::read_to_string(GOLDEN).unwrap_or_else(|e| panic!("{GOLDEN}: {e}"));
    harness::same_text(&expected, &actual, GOLDEN, REGENERATE);
}

/// A reader at the CLI over the seeded world, its first turn read.
fn staged(base: &Path) -> Reader<'_> {
    Seeded::owner().write(&base.join("store"), harness::now());
    let task = &tasks()[0];
    let mut reader = Reader::new(Door::Cli, base, task, &[("ACQ_NO_SPAWN", "1")]);
    reader.first_turn();
    reader
}

/// A script that sorts before it has read the flag fails, naming it.
#[test]
#[should_panic(expected = "uses `--sort`, a flag its row has not read")]
fn a_call_with_a_flag_its_row_has_not_read_fails() {
    let scratch = harness::scratch("countfl");
    let mut reader = staged(&scratch.0);
    reader.acq(&["--account", ALICE, "search", "--sort", "ilvl"]);
}

/// A script that shows an item before any answer gave its id fails,
/// naming it: its author knew the id.
#[test]
#[should_panic(expected = "uses `kaom`, a word its row has not been sent")]
fn a_call_with_an_id_its_row_has_not_been_sent_fails() {
    let scratch = harness::scratch("countid");
    let mut reader = staged(&scratch.0);
    reader.acq(&["--account", ALICE, "show", "kaom"]);
}

/// A fact is its value said of its subject, never the words anywhere: an
/// answer that names Kaom's Heart with no sockets, says `undecided` of
/// another item than Hex Band, or names poe2 outside its scope holds none
/// of those facts, though each holds the words.
#[test]
fn a_fact_is_its_value_said_of_its_subject() {
    let mcp = |doc: Value| Answer {
        text: doc.to_string(),
        doc,
    };
    let cli = |text: &str| Answer {
        text: text.into(),
        doc: Value::Null,
    };
    let task = |n: usize| tasks().into_iter().find(|t| t.n == n).expect("a task");
    let sockets = &task(5).wanted[1];
    let socketless = mcp(json!({ "item": { "name": KAOM, "sockets": [] }, "lines": [] }));
    assert!(!sockets.held_by(Door::Mcp, &socketless));
    assert!(!sockets.held_by(
        Door::Cli,
        &cli(&format!("{KAOM} Glorious Plate\nsockets none"))
    ));

    let hex = &task(2).wanted[3];
    let other = mcp(json!({ "rows": [{ "name": HEX_BAND }],
                            "undecided_reasons": [{ "example": { "name": DOOM_LOOP } }] }));
    assert!(!hex.held_by(Door::Mcp, &other));
    let said = cli(&format!(
        "rows    {HEX_BAND} Two-Stone Ring\nundecided implicit lines unread — e.g. {DOOM_LOOP}"
    ));
    assert!(!hex.held_by(Door::Cli, &said));

    let poe2 = task(8).wanted.last().expect("the scope's fact").clone();
    let elsewhere = cli(&format!(
        "scope   account {ALICE} · pc · live\nundecided no totals table for realm all"
    ));
    assert!(!poe2.held_by(Door::Cli, &elsewhere));
    let pc = mcp(json!({ "scope": { "realm": "pc" }, "terms": [{ "realm": "all" }] }));
    assert!(!poe2.held_by(Door::Mcp, &pc));
}

/// A call is printed so that pasting it sends what the test sent: each
/// word comes back whole, a comparison and a name with a space among them.
#[test]
fn a_printed_call_sends_its_words() {
    for args in [
        &["search", "pseudo.total_life>=90"][..],
        &["--account", ALICE, "search", "--saved", "Life rings"],
        &["search", "name=\"Kaom's Heart\""],
        &["policy", "set", MOCK_POLICY],
    ] {
        let line = printed(args);
        let words = unquoted_posix(line.strip_prefix("acq ").expect("acq"));
        assert_eq!(words, args, "{line}");
    }
}

/// The words of a POSIX command line: single quotes literal, `'\''` a
/// quote inside them, double quotes and backslashes as the shell reads
/// them in these words.
fn unquoted_posix(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut word = String::new();
    let mut started = false;
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        match c {
            '\'' => {
                started = true;
                for q in chars.by_ref() {
                    if q == '\'' {
                        break;
                    }
                    word.push(q);
                }
            }
            '\\' => {
                started = true;
                word.extend(chars.next());
            }
            c if c.is_whitespace() => {
                if started {
                    out.push(std::mem::take(&mut word));
                    started = false;
                }
            }
            c => {
                started = true;
                word.push(c);
            }
        }
    }
    if started {
        out.push(word);
    }
    out
}
