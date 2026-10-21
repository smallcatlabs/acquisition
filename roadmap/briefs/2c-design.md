# 2c — The journey test and real-shaped mock items (design)

A Fable design for step 2c of `roadmap/README.md`, drawn from three
Opus surveys of the tree at `8c10cdbe` (the mock's items and their
readers; the journey, its twin and what covers the ground already; the
job id, the daemon-backed literals and the normaliser). Every `file:line`
is at `8c10cdbe`. The build brief is `2c-build.md`. Nothing here is
built.

## 1. The problem, measured against the tree

**The mock's items reach half the tool.** The stand-in inside `acqd`
serves 45 items, 31 in fifteen stash tabs and 14 on characters, and all
but a few are `"Fake Item {i}"` with no rarity, mods, properties,
sockets or note (`crates/acquisition-daemon/src/mockggg.rs:250-258`).
Measured on a mock world: the plan, apply, tabs, price and shop verbs
work on them, and the search half does not. `rarity=rare` matches 0 of
45; `class:ring` leaves 39 of 45 undecided; the only items a query can
find are the two character helms. A seat, a user trying the tool, or a
journey test cannot reach search or price semantics through fetching:
the assessment's gap (`roadmap/STATE-2026-10-19.md:346`).

**Two mock faults print.** The one unique carries `frameTypeId: "rare"`
(lowercase, and the wrong word for frame 3; `mockggg.rs:394-396`, the
real spelling at `docs/design/network-ground-truth.md:833-835`), so
`show` prints `frame rare` on a unique. Character inventory items carry
`inventoryId: "Stash1"` (`:398`), so the shop renders
`location="Stash1" character=…`; the real value is `MainInventory`
(123 of 123 in the owner's redacted pricing snapshot). No test pins any
item's content: a grep for `Fake Item` or `-item[0-9]` finds nothing
outside `mockggg.rs`.

**No test walks the tool as a user does.** Process-level tests drive
the real binaries through login, plan, apply and store reads
(`apply_loop.rs`, `realm_wire.rs`, `characters_wire.rs`,
`daemon_stop_checkpoint.rs`, MCP `plan_loop.rs`), each asserting typed
fields on one slice. None runs login → policy → apply → tabs → search →
price → show → shop → clear → stop and holds what the user reads. No
transcript of a session is kept anywhere but seat reports.

**A product fault blocks the transcript.** Run the three-apply journey
twenty times and the third apply's `changed:` line reads
`+45 added at 13 locations` in nine runs and `+19 added at 7 locations`
in eleven. `StoreChanges::since` takes `started` from
`acquisition_store::now()`, whole seconds (`acquisition-verbs/src/refresh.rs:470`,
`acquisition-store/src/lib.rs:726-731`), and reads
`WHERE e.at >= ?1` (`lib.rs:1889`): two applies in one second count
the earlier one's events again. Users see this line.

**The daemon-backed texts stayed literals.** 2b converted the CLI's
wording to goldens where no daemon ran, and left "about 17" sites that
print a job id or another varying value as literals
(`2b-design.md:213`), with no list. The survey rebuilt it: 18 sites, in
`seat_faults.rs`, `apply_loop.rs`, `persisted_queue.rs`,
`error_kind.rs`, `daemon_observe.rs`. None of the 25 committed goldens
holds daemon-backed text. The normaliser has six hand-written rules in
the CLI adapter (`acquisition-cli/tests/harness/mod.rs:213-345`) and no
test of its own.

**The job id is a counter, stable by timing alone.** A decimal `u64`
from 1, minted under the daemon's lock (`daemon.rs:1755-1756`),
continued across restarts from the ledger (`store/src/jobs.rs:235-247`);
probes draw from the same counter. In twenty runs every id a client
command printed was identical (applies 2, 7, 19; children 3–4, 8–16,
20–26). Two probes minted together swap ids in about half the runs,
visible only in the `jobs` table. The order rests on the dispatcher
not running while the apply's child loop holds the lock
(`daemon.rs:1979-1982`, `:2652-2665`): a scheduler property, not a
contract.

**One journal line is wrong.** `TESTING-NOTES.md:87` says the first
send on every route but `oauth-token` is a HEAD. The daemon never
probes `profile` or `league` (`daemon.rs:1601-1606`), and every login's
journal opens with `GET profile`. The daemon's own checker never sees
that route (`:7183-7190`).

## 2. The shape

Four moves, each small, each pinned at the lowest crate that holds it
(C122), and one ruling. The owner's items were found by an Opus worker
under his word of 2026-10-21 (section 2.1); the export tool is in the
tree at `24fd3153`.

### 2.1 The mock serves the owner's own items from a committed fixture

A JSON file in the daemon crate, `crates/acquisition-daemon/src/mock-items.json`,
compiled in with `include_str!` (precedent: the class table
`acquisition-search/src/class.rs:11-14`, the price notes
`acquisition-plan/src/price_notes_fixture.rs:25`), keyed by tab id and
by `{character}-equipment` and `{character}-inv` for a character. The
thirteen bodies it takes are named by id in `2c-build.md`'s Input with
the export's digest.
`mock_items(tab, n)` (`mockggg.rs:250`, called at `:398` and `:462`)
returns the fixture's items for a key it holds and the Fake Items it
makes today for one it does not, so every tab the fixture does not
name is unchanged and the counts in `MOCK_TABS` (`:220-236`) stay the
listing's truth; a held key's top-level array has its entry's length,
and a test holds that. The stored item count does move: the store
lifts the armour's six gems into rows of their own, so a full sync
stores 37 items where it stored 31, and every test that asserts that
count is found by grep before the edit and named.

The bodies are the owner's own items, under his grant ("I have no
problems with embedding items from my stash into the testing if they
have useful traits"; "where you can't find an item you want, you can
synthesize it from scratch or modify an existing real item from my
store"), exported from his store by `crates/acquisition-store/examples/export-items.rs`
(`24fd3153`, an owner-run read in `redact-pricing`'s shape) and chosen
for the traits the journey needs (appendix A): a rare Coral Ring with
`+78 to maximum Life` and a crafted resistance line; Doryani's
Prototype, a six-linked unique body armour with six socketed gems, one
at level 20 and quality 20; a Mirror Shard stack of seven; a rare
`Map (Tier 16)`; a rare jewel noted `~price 10 chaos` in a tab listed
`public`; and for the character, Alpha's Howl as the equipped helm and
a unique pair of boots in the inventory, whose `inventoryId` is the
real `MainInventory`. Nothing in a body names the owner, a character or
an account: the fields are the item's (its GGG id, the icon's CDN path,
the note's text). Every body is kept as exported but for its place:
`x` and `y` set to fit the mock tab, and the gems nested back into the
armour's `socketedItems` as GGG sends them (the store lifts each gem
into its own row, so an export carries them apart). Where the owner's
store had held no item with a trait, one would have been hand-written
to the census (`search/DIGEST.md:22-37`, `:67`); none needed to be.
The shapes the real bodies show correct the design's first draft:
`flags` is present on a line only when set; a gem carries no `x`, `y`,
`inventoryId` or `rarity`; currency carries no `rarity` and `ilvl` 0;
a map's tier is in its base, `Map (Tier 16)`, and there is no Map Tier
property (S10); the frame words are `Rare`, `Unique`, `Gem`, `Currency`.
The two mock faults are fixed in passing: the helm is a unique in word
and frame, and inventory items carry `MainInventory`.

Not the trade captures (`search/pseudo-stats/data/bodies.json`): other
players' items in the trade site's line shape (`{description, domain,
hash, mods}`, no id, position or note), and the mock ships in `acqd`
to every user; `SURFACES.md` row 23 and C79 keep that surface a
governed input. Not hand-written bodies where a real one exists: every
hand-written fixture in the tree had drifted from the API's shape
(string lines where it sends objects), and a real body is the only
kind whose shape is certainly current. The `README.md:84-86` sentence
stays true: the stashes are still made up; the items in them are real.

**What the export taught about the store.** The owner's store is facts
schema v8 and HEAD builds v9 (`FACT_SCHEMA_VERSION`, `lib.rs:340`);
`Store::open` migrates an older file under a transaction, so a tool
that says it writes nothing writes on its first open of an older store.
The worker read a copy instead, and the owner's store was not opened.
Raised in section 6 for the owner; not this step's.

What a user and a seat meet changes: `acq search` on the mock answers.
The release seat's brief and the mock-session skill name no item, so
nothing else moves.

### 2.2 One journey, two doors

One sequence, the one a new user types in the order the README's tour
groups its verbs (`README.md:133-184`): `acq auth` (the scripted mock
login), `policy set`, `refresh --plan`, `refresh --apply` until
`nothing to do`, `tabs`, three searches the real-shaped items answer,
`price set`, `show`, `shop render`, `price clear`, `daemon stop`; the
exact commands and the facts each must print are appendix B.

**The CLI door** is a `Transcript` (2b's, `acquisition-cli/tests/harness/mod.rs:109-231`,
which already runs against a daemon) recorded as one golden,
`tests/goldens/journey/a_first_session.txt`, a module of `tests/main.rs`
since it sets no process environment. The daemon is started by the
harness before the login (`start_daemon`, so the test owns the pid and
the stop) with the journal path on the daemon's own command, so every
Transcript command keeps the isolation list's `ACQ_JOURNAL=0`
unchanged. The login runs outside the transcript (its JSON carries
random OAuth values). `daemon status` is not recorded: it varies in
eight fields (uptime, contract digest, the build's hash, the socket's
id, paths, counts); the test asserts its `--json` instead: running,
mock, compatible, and the pid the harness started, so a daemon that
died and respawned mid-journey, which would journal nothing, fails.

**The MCP door** logs in through the sibling `acq` with the shared
`login` (`acquisition-client/tests/harness/mod.rs:548`; the MCP adapter
already finds the sibling, `acquisition-mcp/tests/harness/mod.rs:61-62`),
so it needs no in-process session and sits in the MCP's `tests/main.rs`.
It drives `set_sync_policy`, then `refresh_plan`, `apply_plan`,
`job_status` until done and `job_result` per cycle (`finish_job`,
today private to `plan_loop.rs:49-61`, moves to the MCP adapter),
`tabs`, `search` (the currency request carrying the count view the
CLI's `--count base --sum stack` builds), `price_set`, `show`,
`shop_render`, `price_clear` with `if_revision` (the MCP's clear requires it where the CLI's does
not, `MCP-REFERENCE.md:7`; the twin names the difference in its doc
comment), `daemon_status`, and stops the daemon it owns. JSON is
asserted by field (2b's rule).

**One table of facts.** The twins read one std-only file beside the
harness, `acquisition-client/tests/harness/journey.rs`, included by
`#[path]` in both adapters: the policy, the queries, the price, and the
numbers both doors must see (requests per plan 2, 9, 7 then 0; tabs
listed, fetched and the item count; each query's total; the price
written and cleared; items and pages posted; the journal's expected
GETs per route). The CLI builds its `# holds` facts from the table and
keeps the golden for the wording; the MCP compares fields to the same
numbers. A change to the mock's items is one edit to the table and one
regeneration, read as a diff.

**The journal, by field.** A shared reader in the client harness
replaces the two private copies (`realm_wire.rs:54-69`,
`characters_wire.rs:64-78`; the journey would have been the third).
Both twins assert, over the daemon's `sends.jsonl`: a header and one
pid across every send; the first send per `(pid, route)` a HEAD except
`oauth-token` and the routes the daemon never probes, `profile` and
`league`; `counted == (method != HEAD)`; no send answered 429 and every
reported window `hits ≤ max`; the GETs per route equal to what the
plans named (one stash list, one character list, the characters and the
tabs), plus the token and the profile. `TESTING-NOTES.md:87` is
corrected to say which routes are never probed. Not asserted:
`wait_ms == 0` (the table's own caveat, `:89`); the user-agent, which
is not a journal field and the mock does not record; a retry through a
block, which the mock never serves (a rule over inputs that never carry
the case, `TESTING-NOTES.md:48-52`). Both are stated as out of the
journey's reach in the notes; the user-agent has no test anywhere
(only `ratelimit.rs:1500` and `provider.rs:14-18` name it), which is
raised in section 6 for the owner.

**The tour.** The README's tour is a catalogue of verb shapes, not a
sequence (`README.md:138-184`), and `readme_tour.rs` checks shape only.
The journey and the tour are tied by one check in the journey test:
every verb path the journey runs appears in a tour line, through the
parser `readme_tour.rs:112` already has. A verb the journey needs and
the README does not show fails here.

**What stays.** `apply_loop.rs`'s first test overlaps the journey's
plan-and-apply arc and keeps two checks of its own (the budget refusal,
`--apply=FILE`; `:103-124`) and typed fields; C38's *Pinned* names it
(`AGENTS.md`). It stays, the overlap accepted. `plan_loop.rs` pins the
MCP's gates and defers the loop to the CLI by its own header; it stays
in its own binary. `daemon_stop_checkpoint.rs`'s WAL property is its
own.

**Cost.** Measured by hand: the CLI journey about 2.6 s (the apply's
500 ms poll, `acquisition-verbs/src/jobs.rs:23`, is most of it), the
MCP twin about 1.1 s; two daemons, one per test, as the isolation list
requires. Against a 27 s test stage whose slowest test is 10.8 s, the
wall-time cost is expected nil; the brief asks for the measurement.

### 2.3 The normaliser: three rules, one test, nine conversions

Three rules join the six in the CLI adapter, each one function and one
line in `normalised` (`:215-231`):

- **job.** After `job `, `jobs `, or `acq result `, `acq status `,
  `acq cancel `, `acq priority `, a run of digits, and the digits of an
  en-dash range or a comma list after it, become `<jN>`: N counts
  distinct ids by first appearance across the whole transcript,
  command lines included, so `applying 9 requests as job 7` and the
  later `$ acq result 7` share `<j1>`, and `(jobs 8–16)` reads
  `(jobs <j2>–<j3>)`. Numbered, because the texts relate ids and a flat
  token would lose the relation the failed-apply transcript checks;
  `<jN>` rather than `<job N>` because the latter contains `job N` and
  a second pass would match it (idempotence is a tested property).
  Measured against the 25 committed goldens and the three references:
  zero matches (`job` appears in no golden); against 40 changed
  transcript lines, zero false matches; `26 jobs: 0 running`,
  `1 job:`, `child jobs failed`, `earlier finished jobs not listed`
  are left alone. Two source strings would match and are not job ids:
  `apply job {i}:` (`daemon.rs:4323-4339`) and `quoted job {i}`
  (`acquisition-plan/src/lib.rs:1212`) index a plan's actions; the
  build renames them `action {i}`, the word the plan's own text uses,
  under the owner's delegation (a refusal's wording).
- **pid.** `pid ` and digits become `pid <pid>`: `daemon stopped (pid N)`
  (`main.rs:1656`). Not `cli:<pid>` in the jobs table: its `by` column
  pads to the pid's width (`main.rs:2201-2235`), so the header shifts
  and a table is never recorded.
- **progress.** On stderr, of the lines that begin `job <id>` the
  rule keeps each job's last; the first progress line prints whatever
  state the first poll saw (`main.rs:1944-1981`), and the terminal line
  always prints: a simple job's `done`, an apply's children tally
  (`job <id> (alice): 2/2 done`, `children_line`, `main.rs:2010`), so a
  transcript holds that line whatever the timing.

**A test of the normaliser**, a `#[test]` module in the CLI adapter
(it compiles into `tests/main`): one case per rule from measured text,
the negatives above, and that normalising every committed golden
leaves it unchanged, which also guards every later rule against
rewriting `<scratch>`, `<age>`, `<jN>`.

**Conversions.** Of the 18 daemon-backed sites (appendix E), nine
become goldens with facts: the failed apply and its `result` from the
ledger (`apply_loop.rs:205-236`, one transcript, the parent id related
across both commands by `<j1>`), the cancelled job (`seat_faults.rs:328`),
the progress line (`:281`), the auth status without a keyring (`:74`),
the logout line (`:588`), the spent plan (`:823`), the apply's last
line and the `nothing to do` after it (`:452`, `:462`, which the
`changed:` fix makes deterministic), and the seeded ledger
(`persisted_queue.rs:59-60`). Nine stay literals, each with the reason
in its comment: the two tables whose header shifts with the pid's
width (`seat_faults.rs:371-376`, `:545-557`), the mid-fan-out kill that
is a race by design (`:989-993`), the single refusals (`:381`, `:1953`,
`error_kind.rs:35`), the log path with the socket's hash (`:395`), the
mismatch reports (`daemon_observe.rs`), and the version line, which no
daemon backs (`:1936`).

### 2.4 The `changed:` line is fixed by a watermark

`StoreChanges::since` takes an event-id watermark, not a second, so an
apply's `changed:` line counts no event committed before the apply was
submitted, the earlier events of the same second included; an event
another job commits while the apply runs is still counted, since
attributing events to an apply would need the store to carry the job,
which no fault asks for. The
store's `Status` gains `last_event: i64`, the greatest `item_events.id`
(`INTEGER PRIMARY KEY`, `schema.sql:127`, monotonic within one store;
`sqlite_sequence` is not involved, so the brief has the build confirm
that a rollback never reuses an id, and otherwise read `max(id)` at
`before` and events `WHERE e.id > ?1`); `events_since` keeps its
public shape for `item_events` and gains an `events_after(id, limit)`
sibling; `Submitted.started` goes, `Submitted.before` already carries
the status. The property is pinned once, in the verbs crate's
`tests/documents.rs` beside the `Landed` cases: two applies landing
within one second count their own events, staged by two ingests at the
same `at`. The journey's `changed:` facts then name the counts.

### 2.5 What the step does not touch

The daemon's limiter and the mock's rate policies (the journey stays
under the first-window bursts, `mockggg.rs:155-178`; a second login or
a second listing within ten seconds would wait, so the sequence has
one of each). The protocol. The `jobs` and `daemon status` renderers.
The seat skill. 2d's file splits.

## 3. What changes, by crate

| Crate | Changes |
| --- | --- |
| `acquisition-daemon` | `mock-items.json` compiled in; `mock_items` reads it; the helm's frame word and the inventory's `inventoryId`; one tab `public`; two renamed refusals (`action {i}`); a unit test that every fixture item has an id, a capitalised frame word, object mod lines, and that each substash's count matches |
| `acquisition-store` | `Status.last_event`, `events_after`; a unit test |
| `acquisition-verbs` | `StoreChanges::since` by watermark; `Submitted.started` removed; the two-applies-in-one-second case in `tests/documents.rs` |
| `acquisition-plan` | `quoted action {i}` |
| `acquisition-client` (tests) | `harness/journey.rs` (the table), the journal reader, `start_daemon` with the journal path |
| `acquisition-cli` (tests) | three rules, the normaliser's test module, the journey module and golden, the tour check, nine conversions, `login_as_mock` unchanged |
| `acquisition-mcp` (tests) | `finish_job` into the adapter; the twin in `tests/main.rs` |
| documents | `decisions/daemon.md` the ruling; `TESTING-NOTES.md` the probe line and a "The journey" paragraph; `REVIEW-CHECKLIST.md` a row; `README.md` unchanged |

## 4. The rulings this needs

**R1 — the mock's items.** For `decisions/daemon.md`, under "Daemon,
jobs, protocol", the owner's word (it touches what ships to every user
and which surfaces a fixture may draw on, C79):

> - **C<next> — The stand-in provider serves real-shaped items from a
>   committed fixture.** Its bodies are the owner's own items, exported
>   from his store under his grant and chosen for the traits the journey
>   needs, or hand-written to the census (`search/DIGEST.md`) where his
>   store holds none; compiled into `acqd`, never a path read at run time;
>   never a body from the trade site or another player. *Why:* a user, a
>   seat and the journey test meet search and price through fetching, on
>   bodies whose shape is the API's today, and the fixture's provenance
>   stays the owner's and the project's. *Pinned:* `acquisition-daemon`'s
>   fixture test; the journey goldens.

Measured at 657 bytes as one bullet (the entry rule's 800). The owner's
words of 2026-10-21 are quoted in section 2.1 and in commit 1's message.

**No ruling needed:** the journey's shape (a test, under C122); the
normaliser rules and their test (mechanism under C122); the watermark
(a fault fixed where it lives, C34 unchanged); the probe-line
correction (a finding); the renamed refusals (agent-facing wording,
under delegation, listed here for review); `finish_job` and the journal
reader moving into the harnesses (2b's one-harness rule). `AGENTS.md`
does not change; the budgets stand where the owner left them.

## 5. Alternatives weighed and declined

- **Hand-written bodies to the census claims.** The first draft's
  choice; declined once the owner opened his store: the tree's own
  hand-written fixtures had drifted from the API's shape, and the
  export cost one small example and one read of a copy.
- **The trade captures as the fixture.** Declined: C79 and other
  players' items shipping in `acqd`; and the wrong line shape.
- **A path knob for the fixture** (`ACQ_MOCK_ITEMS=<file>`). Declined:
  a README row and a file-read failure mode inside the daemon for a
  need no one has; `ACQ_MOCK_DEGRADED_HEAD` is the one precedent and it
  stages a fault, not data.
- **A receive log in the mock**, so the journal could be checked
  against the wire and the user-agent asserted. Declined here: the
  daemon's own test module has the wire oracle for scripted servers
  (`TESTING-NOTES.md:86`); the journey asserts the daemon's claim and
  says so. The user-agent gap is raised to the owner instead.
- **Recording `daemon status`** with `sock` and `up` rules. Declined:
  eight varying fields for one line of value; the `--json` assertion
  holds the same facts.
- **A flat `<job>` token.** Declined: the failed-apply transcript
  relates the apply's id to the `result` it reads; ordinals keep it.
- **Subsuming `apply_loop.rs` into the journey.** Declined: its typed
  fields and C38's *Pinned* are its own; the overlap costs two seconds.
- **The README tour as a verbatim first-session block the journey
  runs.** Declined: mock ids in the users' document; the one-way check
  (every journey verb is in the tour) ties them without that.
- **Normalising the `changed:` line.** Declined: the line is wrong for
  users; a rule would hide a fault.
- **One daemon shared by the twins.** Out: the isolation list gives
  each test its own world and nextest a process per test.

## 6. Risks and what holds them

- **A fixture body the store refuses.** A missing `id` on any item, a
  socketed gem included, refuses and rolls back the whole tab
  (`lib.rs:2054-2101`). Held by the daemon's fixture test and by the
  journey's `tabs` fact (every tab fetched, the item count).
- **A line no table knows.** An unknown mod line contributes nothing
  and errs nowhere (`totals.rs:19-20`); a template one character off
  fails silently. Held by `show` facts naming `unread  nothing` and the
  pseudo total's value, and a search fact naming each query's total.
- **Ids colliding across leagues.** The mock serves the same items in
  every league (`mockggg.rs:688`); the journey covers Standard alone.
- **The rate windows.** One login, one listing per kind; a second
  within ten seconds waits and prints a waiting line. Held by the
  sequence; the journey's facts would miss `nothing to do` if it ever
  did.
- **Timing in the golden.** Job ids by the `job` rule; the progress
  line by the `progress` rule; ages by 2b's rule; the pid by its rule;
  `daemon status` by field. The build runs the journey ten times and
  diffs before the golden is committed; a difference is a finding, not
  a rule.
- **The watermark's monotonicity.** `INTEGER PRIMARY KEY` is a rowid;
  SQLite reuses a rowid only after the greatest is deleted, and events
  are never deleted here (`lib.rs` has no delete on `item_events`;
  the brief has the build confirm). The two-ingests-in-one-second case
  pins the property either way.
- **A read that migrates.** `Store::open` migrates an older store on
  its first open; `export-items` and `redact-pricing` say they write
  nothing, which is false against a store behind HEAD's schema. Raised
  for the owner: a read-only open (or a refusal on an older schema) for
  the owner-run examples; a store ruling, not this step's.
- **The user-agent is unpinned** (invariant 4). Not this step's; raised
  for the owner: a unit test in the daemon crate on the request
  builder, or an issue for 2f's ultra review of daemon and network.
- **Golden size.** Today's journey normalises to 5.4 KB; with three
  narrow searches and two `show`s, 10–15 KB. The largest golden is
  7.5 KB; the total 66 KB.
- **The twins drift.** Both read one table; a number changed in one
  place is read by both; the review reads the golden's diff.

## 7. The owner's word

Awaiting the owner's word on R1 and on the design; the surveys'
findings below are his to weigh: the user-agent gap, and the renamed
refusals under delegation.

## Appendix A — the fixture's items (all `pc`, Standard; the owner's, as exported)

Every body as exported, but for `x`, `y` and the gems' nesting. Ids
are GGG's 64-hex. The exports are the build's input (`2c-build.md`).

| # | Item, mock tab | What the body carries | Reaches |
| --- | --- | --- | --- |
| 1 | Empyrean Knot, Coral Ring, rare, `dump` | implicit `+22 to maximum Life`; explicit `+16 to Strength`, `+78 to maximum Life`, `+66 to maximum Mana`, `+15% to all Elemental Resistances`, `+15% to Cold and Chaos Resistances` flagged `{"crafted": true}`; `requirements` Level 64; ilvl 83 | `class=Rings rarity=rare`, `"+# to maximum Life">=70`, `pseudo.total_res`, `--sort pseudo.total_life`; posts as `Stash2` |
| 2 | Doryani's Prototype, Saint's Hauberk, unique, `dump` | six sockets, one group (G R R R R B); `properties` Quality `+20%`, Armour, Energy Shield; `requirements`; `flavourText`; six gems nested in `socketedItems`: Smite (Level `20 (Max)`, `support: false`), Faster Attacks (21), Multistrike, Ruthless, two Awakened supports, each with Quality, `socket`, `colour`, some `corrupted` | `rarity=unique`, `linked(red>=4)`, `level>=20`, `quality>=20`, `class:gem`; the gems blocked as socketed by the shop |
| 3 | Mirror Shard ×7, `cur1` | frame 5 `Currency`, no `rarity`, `ilvl` 0, `stackSize` 7, `maxStackSize` 20, `properties` Stack Size `["7/20", 0]`, `descrText` | `class="Stackable Currency"`, `stack>=5`, `--sum stack`; posts |
| 4 | Dark Realm, `Map (Tier 16)`, rare, `m001` | `name` Dark Realm, `typeLine` and `baseType` `Map (Tier 16)`; `properties` Item Quantity `+52%`, Item Rarity `+31%`, Monster Pack Size `+20%` (no Map Tier property); implicit (a Citadel line with `\n`); five explicit lines; `monsterLevel` | `map_tier>=16`, `class=Maps`; blocked as a substash by the shop |
| 5 | Ghoul Joy, Viridian Jewel, rare, `old1` (listed `public`) | `note` `~price 10 chaos`; three explicit lines | `show`'s note and game price; `price list`'s game side; the shop omits it as listed by the game (C81) |
| 6 | Alpha's Howl, Sinner Tricorne, unique, StashHoarder's equipment | `inventoryId` `Helm`; four sockets; `properties` Evasion Rating; `requirements` | `rarity=unique` at a character; posts with a slot |
| 7 | Seven-League Step, Rawhide Boots, unique, StashHoarder's inventory | `inventoryId` `MainInventory`; four sockets in two groups | the inventory's real slot word |

The remaining tabs, and the other three inventory slots, keep their
Fake Items and counts (`dump` 5: the ring, the armour and three; `cur1`
3: the shard and two; `m001` 1; `old1` 2: the jewel and one).

## Appendix B — the journey, command by command

Each command's facts are drawn from the table; the counts here are
today's and move with the fixture.

| # | Command | Facts (the table's words) |
| --- | --- | --- |
| 0 | `acq auth --no-browser --json` (outside the transcript) | — |
| 1 | `policy set '<policy json>'` | `revision 1` |
| 2 | `refresh --plan` | `2 requests` |
| 3 | `refresh --apply` | `2 done`, `(alice): 2/2 done` (stderr) |
| 4 | `refresh --plan` | `9 requests` |
| 5 | `refresh --apply` | `9/9 done`, `added` |
| 6 | `refresh --apply` | `7/7 done`, `every covered entry is fresh` |
| 7 | `refresh --plan` | `nothing to do` |
| 8 | `tabs` | `15 tabs`, `14 fetched`, `37 items` (the six gems are rows of their own) |
| 9 | `search 'class=Rings rarity=rare'` | `1 match`, `Empyrean Knot` |
| 10 | `search '"+# to maximum Life">=70' --sort pseudo.total_life` | `2 matches`, the ring's total life and the armour's `89` |
| 11 | `search 'class="Stackable Currency"' --count base --sum stack` | `Mirror Shard`, `7` (a sum stands beside a count, `answer.rs:1052`) |
| 12 | `show <ring id>` | `unread  nothing`, `total_life`, `108` (22 + 78 + 16 × 0.5 by `totals-v6.toml:790-792`), `crafted` |
| 13 | `price set item/<ring id> exact 5 chaos` | `5 chaos`, the undo command |
| 14 | `show <ring id>` | `5 chaos` |
| 15 | `shop render` | `1 item to post on 1 page` |
| 16 | `price clear item/<ring id>` | `cleared` |
| 17 | `daemon status --json` (not recorded; by field) | running, mock, compatible, the harness's pid |
| 18 | `daemon stop` | `daemon stopped (pid <pid>)` |

The MCP twin: `set_sync_policy`; three cycles of `refresh_plan`,
`apply_plan`, `finish_job`; `refresh_plan` → 0; `tabs`; the three
`search` requests; `price_set`; `show`; `shop_render`; `price_clear`
with `if_revision`; `daemon_status`; the stop.

## Appendix C — the normaliser, rules in order after the step

1. CRLF to LF. 2. `<scratch>`. 3. `<age> ago`. 4. `<store>`, `<build>`
on a basis line. 5. `acq1.<token>`. 6. `<version>`. 7. `<jN>` (job).
8. `pid <pid>`. 9. the last progress line per job (stderr). A `# holds`
line is never normalised.

## Appendix D — measurements (this machine, 2026-10-21, at `8c10cdbe`)

| What | Value |
| --- | --- |
| mock items | 45 (31 stash, 14 character); `rarity=rare` 0 matched; `class:ring` 39 undecided |
| `changed:` over-count | 9 of 20 runs |
| job ids printed by client commands | identical in 20 of 20 runs; probe pairs swapped in about half, in the `jobs` table only |
| the journey by hand, CLI | 2.56–2.60 s (login 0.6, plan and apply 1.7, reads and stop 0.24) |
| the journey by hand, MCP | about 1.1 s |
| `job` rule over 25 goldens, 3 references | 0 matches; over 40 changed transcript lines, 0 false |
| today's journey transcript, normalised | 5,427 bytes |
| test stage under nextest | 27 s; slowest test 10.8 s |
| journal of one journey | 24 sends: POST token, GET profile, HEAD+GET stash-list, HEAD+GET character-list, HEAD+2 GET character, HEAD+14 GET stash; all 2xx |

## Appendix E — the daemon-backed sites, by action

**Golden (9):** `apply_loop.rs:205-236` (the failed apply and its
`result`); `seat_faults.rs:281` (progress), `:328` (cancelled), `:74`
(auth status, no keyring), `:452` and `:462` (the apply's last line,
`nothing to do`), `:588` (logout), `:823` (spent plan);
`persisted_queue.rs:59-60` (seeded ledger).

**Literal (9), the reason in the comment:** `seat_faults.rs:371-376`
and `:545-557` (a table's header pads to the pid), `:989-993` (a race
by design), `:381`, `:1953` and `error_kind.rs:35` (single refusals),
`:395` (the log path's socket hash), `daemon_observe.rs:84-353` (the
mismatch reports), `:1936` (not daemon-backed).
