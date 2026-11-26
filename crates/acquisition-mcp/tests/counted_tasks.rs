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
//! - A script is only the calls she would try, in order; the reader ends
//!   the row (the stop rule below). It goes on past a refusal by the
//!   refusal's offer, as printed, where it offers a way on (the reader
//!   fails a next call that takes none), else by the plainest guess. No
//!   row ends on a refusal, and a script with calls left after its row
//!   ended fails.
//! - Where a reader would guess, the script takes the plainest guess and
//!   says so. She reads a verb's help when the first turn did not give her
//!   what the call needs, calls a verb bare when it did, and keeps an
//!   account or a realm once named.
//! - A refusal is a non-zero exit, a JSON-RPC error or a result marked as
//!   an error. A byte is one the reader was sent: a command's two streams;
//!   a tool's answer as its line is received, without the line's
//!   terminator. A value the machine or the clock decides counts as a
//!   fixed placeholder: the test's scratch path as [`SCRATCH`], and in a
//!   row over the live daemon each age or duration as [`TIME`]. Counts
//!   are rounded up to two significant figures. A job's polls are
//!   neither calls nor bytes: the golden says only whether there were
//!   any.
//! - A wanted fact is a subject, what is asked of it, and the value
//!   wanted. An answer speaks of a fact when the subject's own row (the
//!   CLI's text) or entry (the MCP's document) carries what is asked,
//!   whatever its value. The row ends at the first answer, not refused,
//!   after which every fact has been spoken of, and each value is judged
//!   from the latest answer that spoke of it: `reached` when one answer
//!   holds them all, `reached over N answers` when N did, else what was
//!   missed and what came instead; a script that runs out first is
//!   missed.
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

/// What an age or a duration counts as in a row over the live daemon,
/// whose wall clock decides its digits.
const TIME: &str = "<time>";

/// `text` with each age or duration as [`TIME`]: a number at a word's
/// start with a time unit ending the word (`0s`, `12ms`, `3m`), or the
/// value of a key ending `seconds` (`"age_seconds":14`).
fn timeless(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(c) = rest.chars().next() {
        let number = rest.len() - rest.trim_start_matches(|c: char| c.is_ascii_digit()).len();
        if number > 0 && out.chars().last().is_none_or(|p| !p.is_alphanumeric()) {
            let after = &rest[number..];
            let unit = ["ms", "s", "m", "h", "d"].into_iter().find(|u| {
                after.starts_with(u) && !after[u.len()..].starts_with(char::is_alphanumeric)
            });
            match (unit, out.ends_with("seconds\":")) {
                (Some(u), _) => (out.push_str(TIME), rest = &after[u.len()..]),
                (None, true) => (out.push_str(TIME), rest = after),
                (None, false) => (out.push_str(&rest[..number]), rest = after),
            };
            continue;
        }
        out.push(c);
        rest = &rest[c.len_utf8()..];
    }
    out
}

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
    /// Whether the row is over the live daemon, whose clock decides ages.
    live: bool,
    /// The facts the row wants, and the latest answer that spoke of each.
    wanted: Vec<Fact>,
    spoken: Vec<Option<usize>>,
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
            live: false,
            wanted: Vec::new(),
            spoken: Vec::new(),
        };
        reader.learn(&format!("{} {} {ALICE}", task.asks, task.knows));
        if door == Door::Mcp {
            reader.mcp = Some(Mcp::start(base, env));
        }
        reader
    }

    /// The bytes of `text` a reader was sent, each value the machine or
    /// the clock decides counted as its placeholder.
    fn counted(&self, text: &str) -> usize {
        let text = text.replace(self.base.to_str().unwrap_or_default(), SCRATCH);
        if self.live {
            timeless(&text).len()
        } else {
            text.len()
        }
    }

    fn learn(&mut self, text: &str) {
        self.known.extend(words(text).map(str::to_string));
    }

    /// Whether the row has ended: every wanted fact spoken of.
    fn done(&self) -> bool {
        !self.wanted.is_empty() && self.spoken.iter().all(Option::is_some)
    }

    /// Fail a call the script makes after its row ended.
    fn calling(&self, shown: &str) {
        assert!(
            !self.done(),
            "{} `{shown}` is a call left after its row ended",
            self.door.name()
        );
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
        if !refused {
            let at = self.answers.len() - 1;
            for (fact, spoken) in self.wanted.iter().zip(&mut self.spoken) {
                if fact.spoken_by(self.door, &self.answers[at]) {
                    *spoken = Some(at);
                }
            }
        }
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
        self.calling(&shown);
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
        let bytes = self.counted(&text);
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
        self.calling(&shown);
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
        let bytes = bytes + self.counted(&text) - text.len();
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
    /// An item, a tab, a league or a target named, or an id: in the CLI's
    /// text the row that names it, with its indented lines and its
    /// section's label, or the whole answer when its first line opens
    /// with it; in the MCP's document the list entry holding a field that
    /// is exactly it, with the keys above it, or the whole document where
    /// it is in no list.
    Named(String),
    /// The answer as a whole.
    Whole,
}

/// One door's reading of a fact: where it is said, what is asked of it
/// (any of `asked`, whatever its value; none asked, the value itself),
/// and the value wanted (every one of `values`).
#[derive(Clone)]
struct Side {
    subject: Subject,
    asked: Vec<String>,
    values: Vec<String>,
}

/// One wanted fact, as each door says it; its values from the world's
/// constants.
#[derive(Clone)]
struct Fact {
    label: String,
    cli: Side,
    mcp: Side,
    /// Where it was never spoken of, what the last answer gave in its
    /// place, read from that answer's fields: a hook, since what is asked
    /// is a substring that holds whatever the value.
    got: Option<fn(Door, &Answer) -> String>,
}

/// A side of a fact: of the subject named, or of the whole answer.
fn side(subject: Option<&str>, asked: &[&str], values: &[String]) -> Side {
    Side {
        subject: subject.map_or(Subject::Whole, |s| Subject::Named(s.to_string())),
        asked: asked.iter().map(|a| a.to_string()).collect(),
        values: values.to_vec(),
    }
}

fn fact(label: impl Into<String>, cli: Side, mcp: Side) -> Fact {
    Fact {
        label: label.into(),
        cli,
        mcp,
        got: None,
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
        (Door::Cli, Subject::Whole) => vec![answer.text.clone()],
        (Door::Mcp, Subject::Whole) => vec![answer.doc.to_string()],
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
        (Door::Mcp, Subject::Named(name)) => {
            let mut out = Vec::new();
            entries_naming(&answer.doc, name, &mut Vec::new(), &answer.doc, &mut out);
            out
        }
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

    /// Whether the answer speaks of this fact: the subject's own row or
    /// entry carries what is asked, whatever its value.
    fn spoken_by(&self, door: Door, answer: &Answer) -> bool {
        let side = self.side(door);
        let asked = |c: &String| match side.asked.is_empty() {
            true => side.values.iter().all(|v| c.contains(v.as_str())),
            false => side.asked.iter().any(|a| c.contains(a.as_str())),
        };
        contexts(door, &side.subject, answer).iter().any(asked)
    }

    /// Whether the answer says the value wanted of the subject.
    fn held_by(&self, door: Door, answer: &Answer) -> bool {
        let side = self.side(door);
        contexts(door, &side.subject, answer)
            .iter()
            .any(|c| side.values.iter().all(|v| c.contains(v.as_str())))
    }
}

/// The answer column, each fact judged from the latest answer that spoke
/// of it: `reached` when one answer holds them all, `reached over N
/// answers` when N did, else each missed fact and what came instead.
fn judged(reader: &Reader) -> String {
    let mut by = Vec::new();
    let mut missed = Vec::new();
    for (fact, spoken) in reader.wanted.iter().zip(&reader.spoken) {
        match spoken {
            Some(at) if fact.held_by(reader.door, &reader.answers[*at]) => by.push(*at),
            Some(_) => missed.push(format!("{}; got: spoken of otherwise", fact.label)),
            None => {
                let last = reader.answers.last().expect("an answer");
                let got = fact
                    .got
                    .map_or("never spoken of".into(), |g| g(reader.door, last));
                missed.push(format!("{}; got: {got}", fact.label));
            }
        }
    }
    by.sort_unstable();
    by.dedup();
    match (missed.is_empty(), by.len()) {
        (false, _) => format!("missed: {}", missed.join(" | ")),
        (true, 1) => "reached".into(),
        (true, n) => format!("reached over {n} answers"),
    }
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
    cli: fn(&mut Reader),
    mcp: fn(&mut Reader),
}

/// Hardcore's own items in the seeded world: `h1`'s one, the hostile
/// note's item; the tab `t-gone` is left out of the league's listing.
const HARDCORE_ITEMS: u64 = 1;

/// The league a price status counted and its count of items: the MCP's
/// `league` and `counts.items`, the CLI's head line (`<count> item(s) in
/// <league>:`).
fn league_count(door: Door, answer: &Answer) -> String {
    let (doc, head) = (
        &answer.doc,
        answer.text.split_whitespace().collect::<Vec<_>>(),
    );
    match door {
        Door::Cli => format!("{}, {} item", head[3].trim_end_matches(':'), head[0]),
        Door::Mcp => format!(
            "{}, {} item",
            doc["league"].as_str().unwrap_or(""),
            doc["counts"]["items"]
        ),
    }
}

/// Task 2's facts, and task 8's: each item's life total, and the ring
/// whose total cannot be decided, said so of it (the MCP names it by its
/// id: an example's name holds its base too).
fn life_90() -> Vec<Fact> {
    let total = |name: &str, total: u64| {
        fact(
            format!("{name}, {total} life"),
            side(
                Some(name),
                &["pseudo.total_life "],
                &[format!("pseudo.total_life {total}")],
            ),
            side(
                Some(name),
                &["\"name\":\"pseudo.total_life\""],
                &[format!("\"name\":\"pseudo.total_life\",\"value\":{total}")],
            ),
        )
    };
    let undecided = ["undecided".to_string()];
    vec![
        total(DOOM_LOOP, DOOM_LOOP_LIFE),
        total(RUNE_COIL, RUNE_COIL_LIFE),
        total(KAOM, KAOM_LIFE),
        fact(
            format!("{HEX_BAND} undecided"),
            side(Some(HEX_BAND), &[], &undecided),
            side(Some(HEX_BAND_ID), &[], &undecided),
        ),
    ]
}

/// The calls that find an item's id by its name, as tasks 5 and 9 make
/// them: the help, the fields, then the name.
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
    // a league held, with its realm: the pair is the subject
    let league_on = |league: &str, realm: &str| {
        fact(
            format!("{league} on {realm}"),
            side(Some(league), &[], &[format!("{league} ({realm}")]),
            side(Some(league), &[], &[format!("\"realm\":\"{realm}\"")]),
        )
    };
    let life_line = |name: &str, total: u64| {
        let line = format!("+{total} to maximum Life");
        fact(
            format!("{name}, {line}"),
            side(
                Some(name),
                &["to maximum Life"],
                std::slice::from_ref(&line),
            ),
            side(
                Some(name),
                &["\"text\":\""],
                &[format!("\"text\":\"{line}\"")],
            ),
        )
    };
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
                    side(Some(NEVER_FETCHED_NAME), &["never", " ago"], &[s("never")]),
                    side(
                        Some(NEVER_FETCHED_NAME),
                        &["\"fetched_at\":"],
                        &[s("\"fetched_at\":null")],
                    ),
                ),
                fact(
                    "an age",
                    side(None, &[], &[s(" ago")]),
                    side(None, &[], &[s("_age\":\"")]),
                ),
            ],
            cli: |r| {
                // `store` takes a subcommand the first turn does not name
                r.acq(&["store", "--help"]);
                r.acq(&["store", "status"]);
                r.acq(&["--account", ALICE, "store", "status"]);
                // the first turn's `leagues --stored`: what the store holds
                r.acq(&["--account", ALICE, "leagues", "--stored"]);
                // `tabs` lists every tab with its age
                r.acq(&["--account", ALICE, "tabs"]);
            },
            mcp: |r| {
                r.tool("store_status", json!({}));
                let alice = r.offer("the same request as Alice#1234");
                r.tool("store_status", alice);
                r.tool("leagues", json!({ "account": ALICE }));
                // no way on offered: the field it names, and the plainest value
                r.tool("leagues", json!({ "account": ALICE, "stored": true }));
                r.tool("tabs", json!({ "account": ALICE }));
            },
        },
        Task {
            n: 2,
            asks: "Which items have at least 90 life in all?",
            knows: "",
            world: World::Pc,
            wanted: life_90(),
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
                        side(Some(ring), &["/ "], &[format!("/ {RINGS_TAB_NAME}")]),
                        side(
                            Some(ring),
                            &["\"place\":"],
                            &[format!("\"name\":\"{RINGS_TAB_NAME}\"")],
                        ),
                    )
                })
                .collect(),
            cli: |r| {
                // her word: `tabs`
                r.acq(&["tabs"]);
                r.acq(&["--account", ALICE, "tabs"]);
                // a count and no names: the search lists items
                r.acq(&["search", "--help"]);
                r.acq(&["search", "--describe", "fields"]);
                r.acq(&["--account", ALICE, "search", "tab=Rings"]);
                // the tabs she read were pc's
                r.acq(&["--account", ALICE, "search", "--realm", "pc", "tab=Rings"]);
            },
            mcp: |r| {
                r.tool("tabs", json!({}));
                let alice = r.offer("the same request as Alice#1234");
                r.tool("tabs", alice);
                r.tool("describe", json!({ "names": ["fields"] }));
                r.tool(
                    "search",
                    json!({ "query": { "text": "tab=Rings" }, "scope": { "account": ALICE } }),
                );
                // the tabs she read were pc's
                let pc = r.offer("realm pc");
                r.tool("search", pc);
            },
        },
        Task {
            n: 4,
            asks: "Are the plain Chaos Orbs in my tab \"~price 3 chaos\" priced?",
            knows: "the tab's name",
            world: World::Whole,
            wanted: vec![fact(
                format!("{PRICED_AMOUNT} {PRICED_CURRENCY} on {PRICED_ITEM}"),
                side(
                    Some(PRICED_ITEM),
                    &[PRICED_CURRENCY, "skip", "none"],
                    &[format!("{PRICED_AMOUNT} {PRICED_CURRENCY}")],
                ),
                side(
                    Some(PRICED_ITEM),
                    &["\"effective\":"],
                    &[
                        format!("\"amount\":\"{PRICED_AMOUNT}\""),
                        format!("\"{PRICED_CURRENCY}\""),
                    ],
                ),
            )],
            cli: |r| {
                // her word: `price`, the listing state
                r.acq(&["price"]);
                r.acq(&["--account", ALICE, "price"]);
                // counts and no items: the help names the list
                r.acq(&["price", "--help"]);
                r.acq(&["--account", ALICE, "price", "list"]);
            },
            mcp: |r| {
                r.tool("price_status", json!({}));
                let alice = r.offer("the same request as Alice#1234");
                r.tool("price_status", alice);
                // counts and no items: the tool that names them
                r.tool("price_list", json!({ "account": ALICE }));
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
                    side(Some(KAOM), &["lines "], &[s(KAOM_LIFE_LINE)]),
                    side(
                        Some(KAOM),
                        &["\"lines\":"],
                        &[format!("\"text\":\"{KAOM_LIFE_LINE}\"")],
                    ),
                ),
                fact(
                    format!("{KAOM}, its sockets {sockets_cli}"),
                    side(
                        Some(KAOM),
                        &["sockets "],
                        std::slice::from_ref(&sockets_cli),
                    ),
                    side(
                        Some(KAOM),
                        &["\"sockets\":"],
                        &[format!("\"sockets\":{sockets_mcp}")],
                    ),
                ),
            ],
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
                // the plan's own words of its league, not the apply's
                // "until the plan says nothing to do"
                side(None, &[], &[format!("nothing to do in {STANDARD}")]),
                side(None, &[], &[s("\"logical_requests\":0")]),
            )],
            // The first turn's words: plan and apply until the plan says
            // nothing to do. The script repeats the round; the reader ends
            // the row at the plan that names nothing.
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
                while !r.done() {
                    r.acq(&[&["refresh", "--plan"][..], &scope].concat());
                    if !r.done() {
                        r.acq(&[&["refresh", "--apply"][..], &scope].concat());
                    }
                }
            },
            mcp: |r| {
                r.tool("refresh_plan", json!({}));
                // no way on offered: the policy as set_sync_policy's own
                // example gives it
                let policy: Value = serde_json::from_str(MOCK_POLICY).expect("JSON");
                r.tool("set_sync_policy", json!({ "value": policy }));
                while !r.done() {
                    let plan = r.tool("refresh_plan", json!({}))["plan"].clone();
                    if r.done() {
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
            // the subject is the league the count is for
            wanted: vec![Fact {
                got: Some(league_count),
                ..fact(
                    format!("{HARDCORE}, {HARDCORE_ITEMS} item"),
                    side(
                        Some(HARDCORE),
                        &[&format!(" in {HARDCORE}:")],
                        &[format!("{HARDCORE_ITEMS} item in {HARDCORE}")],
                    ),
                    side(
                        Some(HARDCORE),
                        &["\"counts\":"],
                        &[format!("\"counts\":{{\"items\":{HARDCORE_ITEMS},")],
                    ),
                )
            }],
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
            // task 2's facts: what this row adds is in its calls
            wanted: life_90(),
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
            // said of the target the write names: its receipt answers her
            wanted: vec![fact(
                format!("{DOOM_LOOP} at 10 chaos"),
                side(
                    Some("item/r1"),
                    &["chaos", "skip", "none"],
                    &[s("10 chaos")],
                ),
                side(
                    Some("r1"),
                    &["\"written\":"],
                    &[s("\"amount\":\"10\""), s("\"chaos\"")],
                ),
            )],
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
            },
            mcp: |r| {
                // a target is `item/<id>`: find the id by her name for it
                mcp_find(r, "name=\"Doom Loop\"");
                r.tool(
                    "price_set",
                    json!({ "target": "item/r1", "type": "exact", "amount": "10",
                            "currency": "chaos", "account": ALICE }),
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
    reader.live = task.world.seeded().is_none();
    reader.wanted = task.wanted.clone();
    reader.spoken = vec![None; task.wanted.len()];
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
        answer: judged(&reader),
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

/// A value said of another subject does not count, as the lookup matches
/// each side's identity: in the CLI's text, Hex Band's own row beside a
/// section that says `undecided` of Doom Loop; in the MCP's document, the
/// row whose id is Hex Band's beside an undecided reason given for Doom
/// Loop's id. Each answer holds both words, and neither speaks of the fact.
#[test]
fn a_value_said_of_another_subject_does_not_count() {
    let fact = &life_90()[3];
    let cli = Answer {
        text: format!(
            "rows    {HEX_BAND} Two-Stone Ring\n            id {HEX_BAND_ID}\n\
             undecided implicit lines unread — e.g. {DOOM_LOOP} · id r1"
        ),
        doc: Value::Null,
    };
    let doc = json!({ "rows": [{ "id": HEX_BAND_ID, "name": HEX_BAND }],
                      "total": { "undecided_reasons": [{ "unread": "implicit lines",
                                 "example": { "id": "r1", "name": DOOM_LOOP } }] } });
    let mcp = Answer {
        text: doc.to_string(),
        doc,
    };
    for (door, answer) in [(Door::Cli, &cli), (Door::Mcp, &mcp)] {
        let words = [HEX_BAND, "undecided"]
            .iter()
            .all(|w| answer.text.contains(w));
        assert!(words && !fact.spoken_by(door, answer), "{}", door.name());
    }
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

/// A missed row says what came instead: task 7's answer for Standard reads
/// as its league and count, so another count changes the row.
#[test]
fn a_missed_count_says_the_league_and_count_it_got() {
    for items in [1, 2] {
        let text = format!("{items} item in {STANDARD}: 0 the game decides");
        let doc = json!({ "league": STANDARD, "counts": { "items": items } });
        let want = format!("{STANDARD}, {items} item");
        let mcp = Answer {
            text: doc.to_string(),
            doc,
        };
        let cli = Answer {
            text,
            doc: Value::Null,
        };
        assert_eq!(league_count(Door::Cli, &cli), want);
        assert_eq!(league_count(Door::Mcp, &mcp), want);
    }
}

/// An age or a duration the clock decides counts the same bytes whatever
/// its digits; a count that is no time keeps its own.
#[test]
fn an_age_or_a_duration_counts_the_same_bytes_whatever_its_digits() {
    let pairs = [
        (
            "job 2 done; submitted 0s ago, took 9ms",
            "job 2 done; submitted 125s ago, took 1250ms",
        ),
        (
            r#"{"age_seconds":4,"at_age":"1m ago"}"#,
            r#"{"age_seconds":4000,"at_age":"59m ago"}"#,
        ),
    ];
    for (one, other) in pairs {
        assert_eq!(
            timeless(one).len(),
            timeless(other).len(),
            "{one} / {other}"
        );
    }
    assert_eq!(timeless("Chaos Orb x5, 37 items"), "Chaos Orb x5, 37 items");
}
