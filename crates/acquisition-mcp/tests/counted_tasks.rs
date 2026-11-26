//! The counted tasks (C122, one harness; C153, a finding joins a
//! surface): ten things a player asks, each run through both doors by a
//! script a reader holding only the help would follow, and counted. The
//! golden, `tests/goldens/counted_tasks.txt`, is the reader's journal as
//! the send journal is GGG's: for each task and door the calls, the
//! refusals, the bytes read, whether a job was polled, and the facts wanted
//! beside whether the row's last answer holds them, then the row's calls.
//! No output text: wording has its own goldens. A row that gets worse or
//! better shows as this golden's diff; regenerate it with
//! `ACQ_UPDATE_FIXTURES=1 cargo nextest run -p acquisition-mcp --test main counted_tasks`
//! and read the diff.
//!
//! The rules a script keeps, which the reader enforces:
//!
//! - A row starts having read its door's first turn (`acq --help`;
//!   `initialize` and `tools/list`), counted once as a row of its own.
//! - Every word of a call (a verb, a flag, a tool, an argument's name, a
//!   value, each word of a query) occurs in the task's own words, the
//!   first turn, or what the row was sent before that call; one that does
//!   not fails the test, naming it. Reading help is a call.
//! - Where a reader would guess, the script takes the plainest guess and
//!   says so; after a refusal that offers a way on, the next call is the
//!   offer as printed. A reader reads a verb's help when the first turn
//!   did not give her what the call needs, and calls a verb bare when it
//!   did.
//! - A refusal is a non-zero exit, a JSON-RPC error or a result marked as
//!   an error. Bytes are what the reader was sent: both streams of a
//!   command, the answer's line as received; rounded up to two
//!   significant figures. A job's polls are neither calls nor bytes: the
//!   golden says only whether there were any.
//!
//! The seeded tasks run over the seeded world (`harness::seeded`) with no
//! daemon, and hold that none appeared; the refresh runs over the mock,
//! logged in as the journeys log in, and holds the send journal's
//! invariants as they do.

use std::collections::HashSet;
use std::path::Path;
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use crate::harness;

use harness::Mcp;
use harness::seeded::*;

/// What the test's scratch path counts as in a reader's bytes.
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

/// A reader at one door: what it may say (the words it has been sent),
/// what it said, and what it was last sent.
struct Reader<'a> {
    door: Door,
    base: &'a Path,
    mcp: Option<Mcp>,
    known: HashSet<String>,
    calls: Vec<Called>,
    last: String,
    last_doc: Value,
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
            last: String::new(),
            last_doc: Value::Null,
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
                self.learn(&init.0);
                self.learn(&list.to_string());
                self.calls.push(Called {
                    shown: "initialize".into(),
                    refused: false,
                    bytes: init.1,
                });
                self.calls.push(Called {
                    shown: "tools/list".into(),
                    refused: false,
                    bytes,
                });
            }
        }
    }

    /// One `acq` command, its words checked first.
    fn acq(&mut self, args: &[&str]) -> &str {
        assert!(self.door == Door::Cli, "acq from the mcp door");
        let shown = format!("acq {}", args.join(" "));
        let said: Vec<String> = args
            .iter()
            .flat_map(|a| words(a).map(str::to_string))
            .collect();
        self.check(&shown, &said);
        let out = harness::acq(self.base, args);
        let text = harness::text(&out);
        self.calls.push(Called {
            shown,
            refused: !out.status.success(),
            bytes: self.counted(&text),
        });
        self.learn(&text);
        self.last = text;
        &self.last
    }

    /// The bytes a reader was sent, the test's scratch path counted as
    /// [`SCRATCH`], so the process's id in it moves no count.
    fn counted(&self, text: &str) -> usize {
        text.replace(self.base.to_str().unwrap_or_default(), SCRATCH)
            .len()
    }

    /// One tool call, its words checked first; the document it carries
    /// (or the error's message) is the row's last answer.
    fn tool(&mut self, name: &str, args: Value) -> &Value {
        self.tool_shown(name, args, None)
    }

    /// [`Reader::tool`], listed in the golden as `shown_as`: for an
    /// argument sent back whole from an answer, which carries its instant.
    fn tool_shown(&mut self, name: &str, args: Value, shown_as: Option<String>) -> &Value {
        assert!(self.door == Door::Mcp, "a tool from the cli door");
        let shown = format!("{name} {args}");
        let mut said: Vec<String> = words(name).map(str::to_string).collect();
        json_words(&args, &mut said);
        self.check(&shown, &said);
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
        self.calls.push(Called {
            shown: shown_as.unwrap_or(shown),
            refused,
            bytes,
        });
        self.learn(&text);
        self.last_doc = serde_json::from_str(&text).unwrap_or(Value::Null);
        self.last = text;
        &self.last_doc
    }

    /// The request of the last refusal's offer that says `says`, sent back
    /// as printed.
    fn offer(&self, says: &str) -> Value {
        self.last_doc["offers"]
            .as_array()
            .and_then(|offers| offers.iter().find(|o| o["says"] == json!(says)))
            .map(|o| o["request"].clone())
            .unwrap_or_else(|| panic!("no offer says {says:?}: {}", self.last))
    }

    /// The script's premise where it guesses after a refusal: the refusal
    /// offered no way on.
    fn offers_none(&self) {
        assert!(
            self.last_doc.get("offers").is_none(),
            "a refusal that offers a way on, which the next call must take: {}",
            self.last
        );
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

/// One wanted fact: what the golden calls it, and whether a door's last
/// answer holds it (the text, and for the MCP its document).
#[derive(Clone, Copy)]
struct Fact {
    label: &'static str,
    holds: fn(Door, &str, &Value) -> bool,
}

/// One task, in her words.
struct Task {
    n: usize,
    asks: &'static str,
    knows: &'static str,
    world: World,
    wanted: &'static [Fact],
    cli: fn(&mut Reader),
    mcp: fn(&mut Reader),
}

/// Whether some line of a CLI answer, or some entry of a list in an MCP
/// document, holds every needle.
fn together(door: Door, text: &str, doc: &Value, needles: &[&str]) -> bool {
    match door {
        Door::Cli => text.lines().any(|l| needles.iter().all(|n| l.contains(n))),
        Door::Mcp => entry_holds(doc, needles),
    }
}

fn entry_holds(doc: &Value, needles: &[&str]) -> bool {
    match doc {
        Value::Array(items) => items.iter().any(|v| {
            let entry = v.to_string();
            needles.iter().all(|n| entry.contains(n)) || entry_holds(v, needles)
        }),
        Value::Object(map) => map.values().any(|v| entry_holds(v, needles)),
        _ => false,
    }
}

/// Every needle in the last answer.
fn says(text: &str, needles: &[&str]) -> bool {
    needles.iter().all(|n| text.contains(n))
}

/// Hardcore's own items in the seeded world: `h1`'s one, the hostile note's
/// item; the tab `t-gone` is left out of the league's listing.
const HARDCORE_ITEMS: usize = 1;

/// The account the mock's own world logs in as, and its policy as a
/// refusal prints it: the journeys' (`harness::journey`).
use harness::journey::{ACCOUNT as MOCK_ACCOUNT, POLICY as MOCK_POLICY};

/// The items whose life lines sum to 90 or more in her pc world: `r1`
/// (95), `r2` (20 implicit and 75) and Kaom's Heart (500); `r3`'s
/// implicit cannot be read.
const LIFE_90: [Fact; 4] = [
    Fact {
        label: "Doom Loop",
        holds: |_, t, _| t.contains("Doom Loop"),
    },
    Fact {
        label: "Rune Coil",
        holds: |_, t, _| t.contains("Rune Coil"),
    },
    Fact {
        label: "Kaom's Heart",
        holds: |_, t, _| t.contains("Kaom's Heart"),
    },
    Fact {
        label: "Hex Band said undecided",
        holds: |_, t, _| says(t, &["Hex Band", "undecided"]),
    },
];

const LIFE_90_BOTH: [Fact; 5] = [
    LIFE_90[0],
    LIFE_90[1],
    LIFE_90[2],
    LIFE_90[3],
    Fact {
        label: "poe2 said answered or left out",
        holds: |_, t, _| says(t, &[POE2, "undecided"]),
    },
];

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
    let all = r.offer("realm all");
    r.tool("search", all);
}

fn tasks() -> Vec<Task> {
    vec![
        Task {
            n: 1,
            asks: "What do I hold, and how old is it?",
            knows: "",
            world: World::Whole,
            wanted: &[
                Fact {
                    label: "her three leagues, each with its realm",
                    holds: |_, t, _| says(t, &[HARDCORE, STANDARD, POE2, PC]),
                },
                Fact {
                    label: "the tab Never as never fetched",
                    holds: |d, t, doc| match d {
                        Door::Cli => together(d, t, doc, &[NEVER_FETCHED_NAME, "never"]),
                        Door::Mcp => {
                            together(d, t, doc, &["\"name\":\"Never\"", "\"fetched_at\":null"])
                        }
                    },
                },
                Fact {
                    label: "an age",
                    holds: |_, t, _| t.contains(" ago"),
                },
            ],
            cli: |r| {
                // `store` takes a subcommand the first turn does not name
                r.acq(&["store", "--help"]);
                r.acq(&["store", "status"]);
                r.acq(&["--account", ALICE, "store", "status"]);
                // counts that name no league: the first turn's `leagues
                // --stored` says what the store holds of each
                r.acq(&["--account", ALICE, "leagues", "--stored"]);
                // nor any tab: `tabs` lists every tab with its age
                r.acq(&["--account", ALICE, "tabs"]);
            },
            mcp: |r| {
                r.tool("store_status", json!({}));
                let alice = r.offer("the same request as Alice#1234");
                r.tool("store_status", alice);
                // as the CLI's reader: the leagues, then the tabs
                r.tool("leagues", json!({ "account": ALICE }));
                // no way on offered: the field it names, and the plainest value
                r.offers_none();
                r.tool("leagues", json!({ "account": ALICE, "stored": true }));
                r.tool("tabs", json!({ "account": ALICE }));
            },
        },
        Task {
            n: 2,
            asks: "Which items have at least 90 life in all?",
            knows: "",
            world: World::Pc,
            wanted: &LIFE_90,
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
            wanted: &[LIFE_90[0], LIFE_90[1]],
            cli: |r| {
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
                    json!({ "query": { "text": "tab=Rings" },
                                         "scope": { "account": ALICE } }),
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
            wanted: &[Fact {
                label: "3 chaos on the plain orbs",
                holds: |d, t, doc| match d {
                    Door::Cli => together(d, t, doc, &[PRICED_ITEM, "3 chaos"]),
                    Door::Mcp => together(d, t, doc, &[PRICED_ITEM, "\"amount\":\"3\"", "chaos"]),
                },
            }],
            cli: |r| {
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
            wanted: &[
                Fact {
                    label: "its life line",
                    holds: |_, t, _| t.contains("+500 to maximum Life"),
                },
                Fact {
                    label: "its sockets",
                    holds: |d, t, _| match d {
                        Door::Cli => t.contains("R-R-G B"),
                        Door::Mcp => t.contains("\"sockets\""),
                    },
                },
            ],
            cli: |r| {
                r.acq(&["show", "--help"]);
                // `show` takes an id: find it by her name for it
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
            wanted: &[Fact {
                label: "a plan that names no request",
                holds: |d, t, doc| match d {
                    Door::Cli => t.contains("nothing to do"),
                    Door::Mcp => doc["plan"]["logical_requests"] == json!(0),
                },
            }],
            cli: |r| {
                // the first turn: `acq policy set` once, then `--plan` and
                // `--apply` until the plan says nothing to do
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
                    if r.last.contains("nothing to do") {
                        break;
                    }
                    r.acq(&[&["refresh", "--apply"][..], &scope].concat());
                }
            },
            mcp: |r| {
                r.tool("refresh_plan", json!({}));
                // no way on offered: the policy as set_sync_policy's own
                // example gives it
                r.offers_none();
                let policy: Value = serde_json::from_str(MOCK_POLICY).expect("JSON");
                r.tool("set_sync_policy", json!({ "value": policy }));
                for _ in 0..6 {
                    let plan = r.tool("refresh_plan", json!({}))["plan"].clone();
                    if plan["logical_requests"] == json!(0) {
                        break;
                    }
                    // the plan sent back as refresh_plan returned it
                    let job = r.tool_shown(
                        "apply_plan",
                        json!({ "plan": plan }),
                        Some("apply_plan {\"plan\": <the plan refresh_plan returned>}".into()),
                    )["job_id"]
                        .clone();
                    r.poll(&job);
                    r.tool("job_result", json!({ "id": job }));
                }
            },
        },
        Task {
            n: 7,
            asks: "I play one league: what is priced?",
            knows: "",
            world: World::Hardcore,
            wanted: &[Fact {
                label: "Hardcore's own count of items",
                holds: |d, t, doc| match d {
                    Door::Cli => t.contains(&format!("{HARDCORE_ITEMS} item in {HARDCORE}")),
                    Door::Mcp => {
                        doc["league"] == json!(HARDCORE)
                            && doc["counts"]["items"] == json!(HARDCORE_ITEMS)
                    }
                },
            }],
            cli: |r| {
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
            wanted: &LIFE_90_BOTH,
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
                // the scope she has learnt, as the CLI's reader keeps her
                // account once named
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
            wanted: &[Fact {
                label: "a read after the write says 10 chaos",
                holds: |d, t, _| match d {
                    Door::Cli => t.contains("10 chaos"),
                    Door::Mcp => says(t, &["\"amount\":\"10\"", "chaos"]),
                },
            }],
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
                // the read after the write
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
                // the read after the write
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
            wanted: &[LIFE_90[0], LIFE_90[2]],
            cli: |r| {
                r.acq(&["saved", "--help"]);
                // the help says a saved search runs through `search --saved`
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

fn run_seeded(task: &Task, door: Door, seeded: &Seeded) -> Row {
    let scratch = harness::scratch("counted");
    let base = scratch.0.clone();
    seeded.write(&base.join("store"), harness::now());
    let mut reader = Reader::new(door, &base, task, &[("ACQ_NO_SPAWN", "1")]);
    reader.first_turn();
    reader.calls.clear();
    match door {
        Door::Cli => (task.cli)(&mut reader),
        Door::Mcp => (task.mcp)(&mut reader),
    }
    let row = finish(task, door, reader);
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
    let mut reader = Reader::new(door, &base, task, &[]);
    reader.first_turn();
    reader.calls.clear();
    match door {
        Door::Cli => (task.cli)(&mut reader),
        Door::Mcp => (task.mcp)(&mut reader),
    }
    let row = finish(task, door, reader);
    let stop = harness::acq(&base, &["daemon", "stop"]);
    assert!(stop.status.success(), "{stop:?}");
    daemon.wait_exit(Duration::from_secs(120));
    harness::assert_journal_holds(&base, pid, &GETS, TOKEN_POSTS);
    row
}

fn finish(task: &Task, door: Door, reader: Reader) -> Row {
    let missed: Vec<&str> = task
        .wanted
        .iter()
        .filter(|f| !(f.holds)(door, &reader.last, &reader.last_doc))
        .map(|f| f.label)
        .collect();
    let answer = if missed.is_empty() {
        "reached".to_string()
    } else {
        format!("missed: {}", missed.join("; "))
    };
    Row {
        task: task.n,
        door,
        calls: reader.calls,
        polled: reader.polled,
        answer,
    }
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
        wanted: &[],
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
         (crates/acquisition-mcp/tests/counted_tasks.rs). Bytes are rounded up to \
         two significant figures; polls are left out of calls and bytes. \
         Regenerate with `{REGENERATE}` and read the diff.\n\n\
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
                let labels: Vec<&str> = t.wanted.iter().map(|f| f.label).collect();
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
