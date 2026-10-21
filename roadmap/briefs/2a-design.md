# 2a — The shared function layer: the design

A Fable design, 2026-10-21, from `2a-function-layer.md`. Nothing here is
built or ruled: the rulings below are drafts for the owner, and the
build brief is `2a-build.md`. This file is deleted at the phase close
with the briefs; its reasoning stays in the commit that lands it.

What was read: the two frontends whole where the brief pointed (the
plan, apply, quote, account, opener, daemon-status, persisted-queue and
job code on both sides), `pricing.rs` as the shape that works, the
protocol's `ErrorKind`, `Quote` and `Outcome`, the client's doors and
`ConnectOptions`, the planner's public flow functions, the store's
account index, the GUI notes (sections 3, 5, 8, 9 and 12), the arch
check, and an Opus inventory of every duplicate with its pins. A second
Opus read the first draft of this file against the tree and found
twelve faults; each is fixed below and the ones that changed the shape
are named in section 5.

## 1. The problem, measured against the tree

The brief names seven duplicates. The tree has those and more; each is
a place where one fact is read in two places (the review prompt's shape
7), and several have drifted:

| Semantics | CLI | MCP | Drift |
| --- | --- | --- | --- |
| the plan compile, no-policy case | `plan_cmd.rs` twice | `main.rs` once | the remedy's words |
| the quote attempt | `plan_cmd.rs` | `main.rs` | the no-daemon note's words |
| the plan answer | the bare envelope, the note on stderr | `{plan, quote_note}` | two renderings of one document, by C11's stdout rule (section 3) |
| the apply loop | `refresh_apply` | `apply_plan` | the MCP lacks `skipped` and `remaining_requests`; its all-spent `note` is fixed text where the CLI's is computed; the CLI's submitted answer is the landed outcome, the MCP's the id |
| the account resolver | `store_cmd::resolve` over a process-global selector | `resolve` per call | three kinds against three other kinds; `accounts` against `readings`; the MCP's unknown-account list is every name, the CLI gives none |
| the opener (facts, intent, both) | eight sites over five files | three | the no-uuid words; the MCP marks `store`, the CLI has no kind |
| daemon status | four arms | four arms, a verbatim copy of ten identity keys | the CLI's absent arm carries `log`, the MCP's does not |
| the queue with no daemon | `jobs`, `status`, `result` answer from disk | `list_jobs` does; `job_status` and `job_result` refuse `no_daemon` | a C45 (the ledger answers) fault on the MCP |
| a job's result | outcome with `job_id` and `account` | the outcome alone | two shapes |
| the parent failure | `parse_children_failure` reads the daemon's sentence back | not read | a protocol field hiding in a string |
| `accounts` | rows with `store` | rows without | one field |
| the policy write with no revision | reads the current revision and replaces | create-only | the frontend default lives in the frontend, where pricing hands it in as `Expect` |
| `item_events --summary` | a CLI-only derivation | none | a derivation in a frontend |
| the default league | `"Standard"` as a literal, many times | twice | `pricing::DEFAULT_LEAGUE` exists |
| the realm's refusal | `parse_realm` | `realm_param`, `price_realm` | three wordings of one sentence |

Two things make this more than untidiness. First, the GUI (phase 4) is
a third frontend, and the loopback server's routes are "thin wrappers
over the shared layer"; built on today's tree it would be a third copy
of each row above. Second, the frontends carry process-global state
(the CLI's selector and account are statics) that a long-lived server
holding several accounts cannot use.

## 2. The shape

One new crate, `acquisition-verbs`, holding the verbs as every
frontend asks them, in `pricing.rs`'s image: typed inputs, typed
documents out, nothing printed, no account resolved inside a verb.
Three ideas carry the design.

**Values, not globals.** A frontend builds one `Frontend` value saying
who is calling (its channel name for provenance, its door policy) and
what world it is in (the provider and store root from the environment,
checked once). From it an `Account` is resolved, and from an `Account`
the facts and intent are opened into an `Opened`. Every verb takes one
of these. The CLI builds a `Frontend` per process, the MCP one per
server, the GUI one per server with an `Opened` per account. A static
cannot survive this shape, so the selector and account statics go.

**One document per answer, one type per document.** Each verb returns
a Rust type that implements `Serialize`; the MCP serialises it, the
CLI's `--json` serialises it, and the CLI's text is a function of the
same value. Two frontends cannot print two shapes of one answer because
neither names a field. Where an answer has several arms (apply did
nothing, apply was all spent, apply submitted) the type is an enum, and
each frontend's renderer matches it exhaustively, so an arm added later
cannot go unrendered.

**One closed fault type, one sorter per frontend.** Every verb fails
with `Fault`, an enum with a variant per cause the layer knows. It
knows its wire `kind` (the protocol's word where the protocol has one),
the fields a failure document carries beside the message (`accounts`,
`next`), and its message, which says what is wrong. Each frontend has
one function from `Fault` to its failure document, matching the enum
exhaustively and adding, per variant, the remedy in its own verbs (the
CLI says `acq policy set`, the MCP says `set_sync_policy`). The MCP's
five marker types and the CLI's chain of downcasts go; a cause the
layer adds later cannot be printed without a kind and a remedy, because
the match will not compile.

### 2.1 The crate and its edges (a C1 amendment, the owner's)

`acquisition-verbs` links the client, the protocol, the store, the
planner and the search crates, tokio (for the quote's deadline and the
job wait), serde, serde_json and anyhow; never the daemon. The frontends
link it, and still link the planner and the search crate directly for
the documents and verbs those already hold (pricing stays in the plan
crate under C75, the search's verbs in the search crate); nothing below
it links it. The rule that decides what moves into the new crate: **a
verb that needs the daemon and the store together, or that every verb
shares (the frontend's identity, the account, the opener, the world),
lives in `acquisition-verbs`; a verb whose semantics is one crate's
stays in that crate.** The GUI's routes call all three, as the MCP's
tools do.

The second option the brief offered (the flow in the plan crate over a
trait the client implements) was weighed and declined: the planner may
not link the client and the client may not link the planner (the arch
check refuses both, for the reasons C39 and the split's packet state),
so the trait's implementation would live in each frontend, which is the
duplication under another name; and the flow needs a runtime for its
deadlines, which the planner has none of.

The arch check needs no new forbid entries for the edges: every lower
crate's list already refuses one of the crates the verbs crate links,
at any depth, so a planner, client, search or store that linked the
verbs crate is refused today, and the daemon's `only_self` and its
client forbid cover both directions for it. The check gains a comment
naming the crate in the edge table, and C46's structural pin (section
4): a grep, in the frontends' sources, for the doors into semantics
that only the layer may open.

### 2.2 The types

Every field is public, so a test builds a `Frontend` or an `Account`
by hand over a fixture store and needs no environment and no lock; the
environment-reading constructors are for the binaries. The async verbs
take `&mut self`: `Store` and `Annotations` each hold a SQLite
connection, which is `Send` and not `Sync`, so a future holding
`&Opened` across an await is not `Send`, and `rmcp` requires `Send`
tool futures (a `&mut Opened` is `Send`). A GUI that holds an `Opened`
for its lifetime wraps it in a lock; that is phase 4's.

```rust
// Who is asking, and in what world. Built once per process or server.
pub struct Frontend {
    pub via: &'static str,       // "cli", "mcp", later "web": the provenance channel (C65)
    pub door: Door,              // the connect policy (C10, C13)
    pub provider: &'static str,  // ACQ_PROVIDER, checked (C88)
    pub dir: PathBuf,            // store_dir(provider), computed once from the same environment the client reads
}
pub enum Door { Human, Agent }   // Human: interactive, replaces a mismatch, spawns; Agent: never replaces, spawns in mock only
impl Frontend {
    pub fn human(via) -> Result<Frontend, Fault>;   // check_environment (C88), then from the environment
    pub fn agent(via) -> Result<Frontend, Fault>;
    pub fn accounts(&self) -> Result<Accounts, Fault>;                   // the index listing, each row with its store path
    pub fn account(&self, selector: Option<&str>) -> Result<Account, Fault>;
    pub async fn connect(&self) -> Result<Client, Fault>;               // the use door under self.door; the CLI reads client.spawned()
    pub async fn attach(&self) -> Result<Client, Fault>;                // the observation door, or the Fault that says why not
    pub async fn daemon(&self) -> Result<DaemonReport, Fault>;          // the four arms, one document
    pub async fn jobs(&self) -> Result<Jobs, Fault>;                    // live, or the ledger (C45)
    pub async fn job(&self, id: u64) -> Result<Job, Fault>;             // live, or the ledger
    pub async fn result(&self, id: u64) -> Result<JobResult, Fault>;    // live, or a terminal outcome from the ledger; with job_id and account
    pub async fn submit(&self, client: &mut Client, kind, params, priority, account: Option<&str>) -> Result<u64, Fault>;
        // the selector passes through to the daemon unresolved, as today (C3: a job verb with no login still spawns and is refused by the daemon)
    pub async fn wait(&self, client: &mut Client, id: u64, progress: Option<&mut (dyn FnMut(&Progress) + Send)>) -> Result<Outcome, Fault>;
    pub fn start_command(&self) -> String;   // the shell line that starts a daemon in this world, as printed
    pub fn realm(word: Option<&str>) -> Result<Realm, Fault>;           // the one refusal sentence for a realm
}

// A resolved account. Only Frontend::account makes one in the binaries.
pub struct Account { pub frontend: Frontend, pub entry: AccountEntry }
impl Account {
    pub fn facts(&self) -> Result<Store, Fault>;
    pub fn intent(&self) -> Result<Annotations, Fault>;  // Fault::NoUuid when the entry predates uuid-at-login
    pub fn open(&self) -> Result<Opened, Fault>;         // both
}
pub struct Opened { pub account: Account, pub facts: Store, pub intent: Annotations }

// The refresh flow, over an Opened.
impl Opened {
    pub fn policy(&self) -> Result<Option<AnnotationRow>, Fault>;
    pub fn set_policy(&mut self, value: &Value, if_revision: Option<i64>, expect: Expect, write: PolicyWrite) -> Result<AnnotationRow, Fault>;
        // if_revision None: Expect::Current reads the stored revision and refuses a row this build cannot read (as pricing does: C35, intent is irreplaceable); Expect::Create passes None (C52)
    pub fn compile(&self, realm, league, now) -> Result<RefreshPlan, Fault>;    // Fault::NoSyncPolicy
    pub async fn plan(&mut self, realm, league) -> Result<Planned, Fault>;       // compile, then quote
    pub async fn apply(&mut self, plan: &RefreshPlan, max_requests: Option<u64>) -> Result<Apply, Fault>;
    pub async fn landed(&mut self, client: &mut Client, submitted: &Submitted, progress) -> Result<Landed, Fault>;
    pub fn events(&self, since, limit) -> Result<Vec<EventRow>, Fault>;
    pub fn events_summary(&self, since, limit) -> Result<EventsSummary, Fault>;  // the CLI's summariser, moved
}
pub async fn quote(frontend: &Frontend, plan: RefreshPlan) -> Planned;   // never fails: the plan is the deliverable; observes only, 5 s deadline

pub struct Planned { pub plan: RefreshPlan, pub quote_note: Option<String> }   // the note from one function, one sentence per cause

#[serde(untagged)]
pub enum Apply {
    Nothing   { applied: bool /* false */, requests: u64 /* 0 */, note: String, skipped: Skipped },
    Spent     { applied: bool, requests: u64, note: String, spent: Vec<Spent>, skipped: Skipped, remaining_requests: Option<u64> },
    Submitted (Submitted),
}
pub struct Submitted {
    pub job_id: u64, pub account: String, pub requests: usize, pub spent: Vec<Spent>, pub skipped: Skipped,
    #[serde(skip)] pub before: Status, #[serde(skip)] pub started: i64,   // what landed() needs: the store's state at submit
}
pub struct Landed { #[serde(flatten)] pub outcome: Outcome, pub job_id: u64, pub account: String, pub store_changes: StoreChanges, pub remaining_requests: Option<u64>, pub spent: Vec<Spent> }
pub struct Skipped { pub tabs: usize, pub characters: usize }

pub enum DaemonReport {
    Absent { persisted: Option<PersistedSummary>, log: Option<PathBuf> },
    Incompatible(Box<DaemonId>),
    Unresponsive(client::Unresponsive),
    Running { status: Response /* the DaemonStatus variant; any other is Fault::Transport */, identity: DaemonId },
}   // one Serialize producing today's flat JSON (running, compatible, responsive, resp, the identity keys)
pub enum Jobs { Live(Vec<JobInfo>), Absent { persisted: Option<Persisted> } }
pub enum Job  { Live(JobInfo),      Absent { persisted: PersistedJob, next: String } }   // next: start_command()
pub struct JobResult { pub job_id: u64, #[serde(skip_serializing_if = "Option::is_none")] pub account: Option<String>, #[serde(flatten)] pub outcome: Outcome }
pub struct Progress { pub job: JobInfo, pub children: Option<ChildTally> }   // what the CLI's progress line renders; the layer lists the children

pub struct Accounts { pub provider: String, pub accounts: Vec<AccountRow> }  // AccountRow: username, last_login, persisted, uuid, store
```

Two notes on the flow. The apply loop is C111's as the CLI has it
(unspent against the facts; connect only if something is left; the
daemon's list; the facts read again; submit), with the all-spent note
computed as the CLI computes it today, in words both readers accept
(the facts: how many fetched since and the latest's age, how many held
by which jobs; no shell command inside it, the CLI's text adds its
own). `Submitted` carries, unserialised, the store status and the
clock at submit, because the store-change summary is a difference of
two statuses and cannot be recomputed from a timestamp; `landed` is
the process that submitted, which today is the CLI and later the GUI.

### 2.3 The fault type

```rust
pub enum Fault {
    Environment(String),                                        // kind refused: ACQ_PROVIDER or a leftover knob (C88)
    NoAccounts { dir: PathBuf },                                // kind not_logged_in, accounts: []
    UnknownAccount { selector: String, known: Vec<String> },    // kind not_logged_in, accounts: known
    AmbiguousAccount { candidates: Vec<String> },               // kind ambiguous_account, accounts: candidates
    NoUuid { username: String },                                // kind refused: a login predating uuid-at-login
    Unopened(anyhow::Error),                                    // kind store: a facts, intent or index file would not open
    NoSyncPolicy { username: String },                          // kind refused
    Plan(PlanError),                                            // kind refused: a malformed plan or policy, an uncovered league
    Spend(SpendError),                                          // kind refused: a gone or moved policy, the wrong provider or account
    Policy(PutPolicyError),                                     // kind refused (a revision conflict's `conflict` kind is step 2g's, filed from 0b)
    NoDaemon { may_spawn: bool },                               // kind no_daemon; next: start_command()
    Door(ConnectError),                                         // the use door did not open: the MCP's JSON-RPC data.connect, the CLI's sentence
    AnotherDaemon(Box<DaemonId>),                               // observed, not this client's: never replaced by an observer (C10)
    Unresponsive(client::Unresponsive),                         // observed, no hello within the deadline (C86)
    Daemon(DaemonError),                                        // kind: its ErrorKind word (C85)
    Transport(anyhow::Error),                                   // a closed socket, an answer of the wrong shape
    Other(anyhow::Error),                                       // kind refused: a store read that failed, the rest
}
impl Fault {
    pub fn kind(&self) -> &'static str;         // the wire word; ErrorKind::as_str for the protocol's
    pub fn fields(&self) -> Map<String, Value>; // kind, and accounts / next where the variant has them
}
```

`Display` says what is wrong and names the human's command where the
remedy is a human's act (a login). The variants whose remedies the two
surfaces word differently are five: the account pair (the CLI says
`--account` or `ACQ_ACCOUNT`, the MCP the `account` argument,
`scope.account` on search), the missing policy (`acq policy set`
against `set_sync_policy`), a gone or moved policy (`acq refresh
--plan` against `refresh_plan`), and no daemon (the CLI's door spawns,
the MCP's `next` is a person's command). Each frontend words those in
its one match; a `Remedy` enum beside the sorter was drafted and cut as
a second table saying the same thing (section 5).

What stays in the frontends: the CLI's `usage` kind (clap's), the
MCP's `arguments` kind (its own argument reader), pricing's `conflict`
and `not_found` and the search crate's refusals (already shared,
sorted today by their own helpers; step 2g folds them into one shape).
The MCP keeps the decision of which faults are JSON-RPC errors (a
daemon's own refusal, a door, another or an unresponsive daemon, the
transport) and which are results marked `isError`; that decision is
one match on `Fault`. `Other` is the escape hatch for what the layer
does not yet name; the sorter gives it `refused`, as the MCP does today.

### 2.4 The children, typed

`Outcome::Failure` gains `children: Option<Children>` beside `error`,
where `Children { done, failed, cancelled, failed_ids }` is the summary
the daemon already attaches, as JSON, to a parent's success payload.
The daemon fills it on both parent-failure arms (children failed;
children cancelled); the sentence stays for a reader. `Outcome::Failure
{` is written at 52 sites across the daemon and the CLI, so the
protocol gains an `Outcome::failure(message)` constructor and every
pattern gains `..`. The CLI's `report_apply` reads the field and
`parse_children_failure` is deleted. The protocol source moves, so the
contract revision moves: every frontend and the daemon rebuild together
(C84). The wire fixture pins one sample per response variant and its
`Result` sample is a success, so the build adds an `Outcome` sample for
a failure carrying `children` beside it, within the fixture's own rules.

## 3. What each frontend becomes

The CLI keeps clap, every renderer (`render_plan` and its thousand
lines stay where they are), the text of its refusals and remedies, the
progress line, the tables, `dash` and `jobs --watch`, the `SPAWNED` note
(set from the client the layer's `connect` returns), and one
`fault_report` function. It loses both apply loops' orchestration,
`try_quote`, `open_intent`, the opener copies in `search_cmd.rs`,
`price_cmd.rs`, `shop_cmd.rs` and `store_cmd.rs` (their verbs stay;
they open through `Account`), `store_cmd::resolve` and its static,
`error_report`'s downcast chain, `persisted_as_info`, the children
parser, `summarize_events` (moved), the `"Standard"` literals and the
realm sentence. Its renderer tests in `plan_cmd.rs` build their plan
through the layer's `compile` instead of the planner's.

The MCP keeps `rmcp`, `transport.rs`, `Args`, `told`, the tool
descriptions, and one `failed` function over `Fault`. It loses
`try_quote`, `apply_plan`'s body, `open_intent`, `open_store`,
`open_search`, `open_account_intent`, `resolve`, `refusal`,
`NoAccounts`, `NoDaemon`, `Unopened`, `Unserved`, `start_one`,
`connect`, `attach`, `observe`, `client_of`, `realm_param`,
`price_realm` and the ten copied identity keys.

Agent-facing changes this makes, each the session's under the owner's
delegation and listed here for his review; every one is additive or a
rename of a kind, as C53 (JSON changes are additive) asks:

- the MCP's `account_needed` becomes `ambiguous_account`;
  `account_unknown` and `no_accounts` become `not_logged_in`; the known
  names ride as `accounts` on all three (the CLI's field), `readings`
  no longer appears on an account refusal;
- the CLI's `unknown_account` becomes `not_logged_in` with `accounts`;
- `apply_plan` gains `skipped` on every arm and `remaining_requests`
  on the all-spent arm, and its all-spent `note` is computed;
- `job_result` carries `job_id` and `account` beside the outcome, and
  answers a terminal job from the ledger with no daemon; `job_status`
  with no daemon answers the ledger's row as `running: false` with
  `next` (the command that starts a daemon) instead of refusing;
  `submit_job` and `cancel_job` still refuse `no_daemon`, since they
  act;
- `daemon_status` with no daemon carries `log` when a log exists;
- the MCP's `accounts` rows gain `store`;
- the quote's absent-daemon note is one sentence on both surfaces;
- a blind CLI policy write (`policy set` with no `--if-revision`) over a
  stored policy this build cannot read is refused, as a blind price
  write is.

Two documents keep two renderings on purpose. The CLI's `refresh
--plan --json` stays the bare envelope on stdout with the quote note on
stderr, by C11's own rule (stdout is pipeable into `--apply --plan=-`,
and the envelope's validating parse would refuse a wrapper): it prints
the `plan` member of the shared `Planned`. The CLI's `policy show
--json` stays the row or null and `accounts --json` the array, as C53
asks; the MCP wraps each in an object because a tool result cannot be
null or a bare array. A wrapper around a shared document is
adaptation; a field computed by one surface is not, and none remains.

## 4. The rulings this needs

Drafts as the bullets would stand, measured against the entry rule's
800 bytes. The owner's word first on each; the build waits on 1 and 4
(boundaries), and records 2, 3 and 5 under his delegation, listed for
review. `AGENTS.md` grows by about 450 bytes (C1, C46 and the new
crate's row in the workspace table) against 185 free; the owner gave
his temporary permission on 2026-10-21 to raise the budget in
`tools/docs-check.sh` for this phase, and he will reassess it himself.

1. **C1, amended** (`AGENTS.md`, the owner's; 706 bytes):

   > - **C1 — The workspace is library-centric and the daemon is its own artifact.** `acqd` is the only GGG sender and links the protocol crate and the store, nothing else; the store links none of the workspace crates; the protocol crate links serde alone; the verbs crate holds what needs the daemon and the store together and links client, protocol, store, planner and search; a frontend links those and the verbs crate as it needs them, never the daemon; nothing below the verbs crate links it, and neither the daemon nor the planner links the search. *Why:* shared logic is written and tested once, and the edges are refused by a check, not held by discipline. *Pinned:* `tools/arch-check.sh` (the edges).

   And the workspace table gains a row: `acquisition-verbs` — the
   verbs that need the daemon and the store together, and every
   frontend's account, world and door — `decisions/frontends.md`.

2. **A new frontends ruling, the next id** (`decisions/frontends.md`,
   under the delegation; 776 bytes):

   > - **C<next> — An account that cannot be picked is refused in the protocol's words on every surface: `not_logged_in` when no account is known or the selector names none the index holds, `ambiguous_account` when it names several or names none among several, each with `accounts`, the names known or to pick from.** *Why:* the daemon already answers a named account it has no session for as `not_logged_in` (C85, the closed kinds), so one word for one mistake lets an agent branch the same way whether the verb went to the store or the daemon. *Pinned:* `acquisition-cli/tests/seat_faults.rs` (`an_account_that_cannot_be_picked_`), `acquisition-mcp/tests/tool_refusals.rs` (`a_tool_refusal_is_`), `acquisition-mcp/tests/search_tools.rs` (`the_requests_account_selects_the_store`).

3. **C46, amended Pinned** (`AGENTS.md`, under the delegation; the
   ruling's words stand; 532 bytes):

   > *Pinned:* `tools/arch-check.sh` (the frontend door grep: no frontend opens a store, picks an account or compiles a plan itself), `acquisition-verbs/tests/`, `acquisition-mcp/tests/pricing_tools.rs` (the tools call the functions the CLI calls).

   The grep refuses, in `crates/acquisition-cli/src` and
   `crates/acquisition-mcp/src`, the names `Index::load`, `Store::open`,
   `Annotations::open_for`, `plan_refresh`, `check_spendable`,
   `apply_params_of`, `.quote(`, `Persisted::observe` and
   `Client::connect`; the list lives in the script with a breaker case
   per name, as C34's intent grep does. `Client::observe` is not on it:
   `auth status`, `daemon stop` and `reset-tripwire` observe with no
   semantics to move, and a pass-through added for a grep is mechanism.

4. **C23, amended** (`decisions/daemon.md`, the owner's: the protocol
   is his; 697 bytes):

   > - **C23 — Work that needs many requests is a parent job that submits child jobs.** A parent finishes when its last descendant does, gives up its dispatcher task and scheduling key while it waits, and cancels its descendants when cancelled; its failure carries its children's tally (done, failed, cancelled, the failed ids) as a field, never as prose a client reads back. *Why:* the queue, dispatcher, priorities, ETAs and events already work per job, so children get them free. *Pinned:* `daemon.rs`, `cancelled_children_do_not_make_a_parent_successful`, `a_cancelled_parent_cannot_gain_children`; `acquisition-protocol/tests/wire.rs` (the failure sample); `acquisition-cli/tests/apply_loop.rs`.

5. **C45, amended** (`decisions/daemon.md`, under the delegation; 716
   bytes): the verb list reads "`acq jobs`, `status` and `result`,
   `daemon status` and the MCP's `list_jobs`, `job_status`,
   `job_result` and `daemon_status`", and *Pinned:* adds
   `acquisition-verbs/tests/`.

No new ruling for the layer's existence beyond C1: C46 already says
every frontend is an adapter; what it lacked was structure, which 3
gives it. No ruling for `Fault`: it is mechanism, and lives as doc
comments headed by the ids it serves (C11, C85, C116).

## 5. Alternatives weighed and declined

Cut after the second read, each a mechanism that removed no code:

- **A `Remedy` enum beside `Fault`,** with one words table per
  frontend. The sorter already matches every variant, so the remedy's
  words live in that match; a second closed enum restated it.
- **`submitted_at` on the submitted document,** so any process could
  compute the landed summary later. False: the store-change summary
  subtracts two store statuses, which a timestamp cannot recover.
- **`Frontend.expect`,** the blind-write default as a field. It is the
  door restated; `set_policy` takes `Expect` as pricing's verbs do.
- **A `QuoteNote` enum.** Seven variants that serialise as text; one
  function returning the sentence does the same.
- **Reshaping the CLI's `accounts` and `policy show` JSON** to the
  MCP's object forms. Neither additive nor a rename, so against C53,
  and it removed no code; the MCP's wrappers are adaptation.
- **New forbid entries for the planner, client, search and store.**
  Each list already refuses, at any depth, a crate the verbs crate
  links; five entries and five breakers would guard what is refused.
- **`item_events` gaining `summary: true` on the MCP.** No consumer
  has asked (P2); the summariser moves, the argument waits.

Declined in the first draft and still:

- **A tenth `ErrorKind`, `unknown_account`.** Keeps a typo apart from
  a missing login by kind. Declined: the daemon cannot tell them apart
  (a named account with no session is `not_logged_in` by its own doc),
  so the offline resolver would disagree with the daemon on the same
  mistake; the `accounts` field tells the two apart for any reader.
- **The MCP's `apply_plan` waiting for the job** so both surfaces
  return `Landed`. Declined: a tool call holding for a five-minute
  rate-limit wait is the shape the job model exists to avoid (C4). Two
  documents, submitted and landed, name the two moments honestly.
- **Wrapping the CLI's `--json` plan in `{plan, quote_note}`.**
  Declined: stdout is piped into `--apply --plan=-`, and the envelope's
  validating parse would refuse the wrapper; `plan_json.rs` pins it.
- **Moving pricing into the new crate** for one home for all verbs.
  Declined: C75 places it, it needs no daemon, and the move is churn
  without a property gained.
- **Typing the ledger's job state in the store.** The store cannot
  link the protocol (the arch check), so `JobState` cannot appear
  there; the one mapping lives in the layer instead.
- **A trait over the client in the planner** (the brief's option b):
  section 2.1.

## 6. Risks and what holds them

- **The contract revision moves** (the children field): every daemon
  test and the CLI's and MCP's process tests rebuild; a daemon left
  running from before the build is an artifact mismatch (C84). The
  worker builds before any test, as `AGENTS.md` orders.
- **Kind renames break agents keying on the old words.** There are no
  users yet (the alpha is not a beta); the reference regenerates; the
  rulings record the words.
- **The layer's tests need a mock daemon in-process.** The client's
  contract tests already do this: `session` (one lock, the
  environment set under it), `acqd_for_tests` and `place_link` (the
  daemon the gate built, linked beside the test executable),
  `start_daemon`, and `login` through the mock's approve page over
  reqwest as a dev-dependency; about 150 lines, copied, which step 2b's
  one harness absorbs. The pure tests (the fault kinds and fields, the
  ledger mapping, the apply arms over a fixture store) build their
  values directly and need neither.
- **`Frontend.dir` is a copy** of what the client derives from the
  environment at connect time. Both read the same environment; a
  `Frontend` is built after the environment is final, which the
  binaries do at start and the tests do under the lock.
- **The arch-check grep is a list of names.** A rename in the client
  or planner that the grep does not follow lets a door back in. The
  breaker suite stages each name; the list is short and each name is a
  public function whose rename is itself a review event.
- **Behaviour the layer must not change:** a job verb with no login
  still spawns a daemon and is refused by it (C3), so `submit` passes
  the selector through to the daemon as the CLI does today and resolves
  nothing offline; only a verb that opens a store resolves first.
- **Scope pressure.** The inventory found more than the brief listed.
  Everything in section 1 is in the build brief; the owner strikes
  rows, the worker does not add any. Pricing's and the search's verbs,
  the GUI server and the daemon split stay out.

## 7. The owner's word

Asked: yes or no on each ruling in section 4 and the crate's name
(`acquisition-verbs`, the word `pricing.rs` already uses for this
idea); and whether `job_status` with no daemon answers the ledger's row
as drawn or keeps the release seat's `no_daemon` refusal. Answered
2026-10-21, verbatim: "All rulings are approved. Proceed." The name
stands and the drawn answer stands, carrying the seat's `next`.
