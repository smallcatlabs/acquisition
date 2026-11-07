# 4-full's design prompt, read whole: a critique

Provenance: Fable (`claude-fable-5-1`), 2026-11-06, read-only at
`deec4e51` on `claude/4-full-design-prompt`. Nothing run, nothing
changed, no issue touched. Read directly: the prompt, `AGENTS.md`,
`GLOSSARY.md`, `decisions/daemon.md` and `frontends.md` whole,
`roadmap/GUI-NOTES.md` whole, `roadmap/reviews/time-seam-retrospective.md`
whole, `roadmap/README.md` (the rulings, the phase table, the
test-shape group), `4-0-design.md` §7 and §8, `4a-design.md`'s head,
`acquisition-protocol/src/protocol.rs` whole, the client's
`ConnectOptions` and `human_present`, the search crate's `Basis`,
`is_current` and `Holder` docs, the store's `revision`, `acq web`'s
module doc and router, and `watch_jobs`. Three subagent reads, each
checked where I use it: the client's `Door`, the daemon's `Life` and
the subscription's two planes; every wait and poll in the verbs, the
CLI, the dashboard, the MCP's tool texts and the web module; the fifty
open issues and the roadmap's rhythm. The owner's question was: sit
with the prompt, imagine the session that runs it, and say what should
change, in the prompt or elsewhere, so the GUI starts from a clean,
idiomatic, performant and future-compatible design.

## 1. What the session would produce, as the prompt stands

A brief of 500 to 800 lines in `4-0-design.md`'s form. Its substance,
predicted from the questions and the reading list: Root B weighed and
the subscription widened to carry auth events (the daemon's
`auth_changed` `Notify` already fires at eleven sites, `start.rs:602`,
`login.rs:135` and nine more, and nothing on the wire reads it); a
`Watch` machine in the client crate beside `Door`, with reconnect,
twenty to forty rows; a bounded wait for the MCP; a keeper ruling that
says the open subscription is the keeper; a spawn-policy ruling that
restates the status quo for `acq web` and words the windowless case
for a later Tauri wrapper; a `Searcher` in the verbs crate with a
one-second revision poll and an SSE line, close to `GUI-NOTES.md` §5;
and a two-slice build, search and liveness first, the queue and
reconnect second. A cold review would cut it three times, as the 4-0
and t5 briefs were cut, mostly on line numbers and on the size.

That brief would be good on the daemon hop and silent on the page. It
would answer "how does the Rust process learn things changed" in full
and "what is the page, and what may it hold" not at all, because no
question asks. The owner's own worry (frameworks move; the design must
survive a change of one) lives entirely in the question the prompt
does not ask. That is the main finding; the rest are smaller.

## 2. What the prompt gets right

- The two mornings as the acceptance test, the owner as the validating
  consumer (P2). Concrete, dated, honest about who uses the tool.
- Two seats, not averaged. The retrospective's §5 earned this.
- "Read the system as it is now." `GUI-NOTES.md` is three weeks and
  one phase old; §3's "no protocol change is needed" is still true for
  job events and false for auth, and the session should find that.
- The warning against a design that covers every issue, and the
  invitation to conclude the protocol needs little. Both are what a
  cold reviewer would otherwise have to say.
- #142 not a prerequisite, with the one rule that keeps the harness
  from growing a private world. Right call; t6 designed from a third
  consumer will be a better t6.
- Design only, commit only the brief, the mock under its skill. The
  boundaries are the project's and they are tight.

## 3. The page hop is missing, and it is where the framework question lives

There are two hops, and the prompt designs one:

1. daemon to Rust frontend: the protocol (C85), the client's door, the
   subscription;
2. Rust frontend to page: `acq web`'s private routes (C138), HTML
   today, SSE tomorrow.

Every question in the prompt is about hop 1. Hop 2 is where a
framework would enter: cards (#98), a dense virtualised table, a live
loop that re-renders keyed rows. 4-lite is server-rendered HTML with
one script of a few dozen lines (`picker.js`) and a meta-refresh on
the jobs page; 4-full is where someone reaches for Svelte, React or
htmx, and the choice then shapes the tests, the gate (Node or not),
the release and every later page.

The future-proof shape is a property, not a framework, and the
registry is one sentence short of holding it. C16 says the page is
presentation and evaluates nothing (semantics). What it does not say
is the state side: **the page holds no state the server cannot
rebuild from the page's URL and one feed line; a page's identity is
its URL; every document the page holds carries its basis, and the
basis is the only version there is.** Under that sentence the page's
whole input is the verbs' documents (which the tests already compare
with the CLI's `--json`, C16's pin) plus one stream of hints, and any
framework, or none, renders it. Swapping the framework is a
presentation rewrite with the goldens unchanged. Without that
sentence a framework brings its own store, its own cache keyed on its
own sequence number, and the page quietly becomes a second holder of
state the basis does not label. That is the invalid state the owner
asked to make unreachable, and it is one line of C16 away.

`decisions/frontends.md` already has the test rule in its "Not doing"
lines: a page rule that is semantics moves to the server, one that is
presentation gets a browser-driven check. The design should lift that
into the same amendment.

The toolchain itself (`GUI-NOTES.md` §9 question 7, "decide at the
first spike; lean to the smaller toolchain") should stay out of the
design brief and be decided inside the build by the dense-table step,
under a rule the owner can give now: the gate stays Node-free unless
a spike shows the form's complexity needs a framework; a vendored,
pinned virtualiser file is the first thing to try. The design's job
is to make that choice a leaf.

**Change:** a ninth question, "the page: what it holds, what it may
not, and the one feed", with a C16 amendment drafted; a line in
"Boundaries" that no framework is chosen in this session.

## 4. The basis is the version; Q5 should say so

Q5 asks how the GUI learns the store or the intent moved (#25), and
what each change costs. The system already has the right idea and the
prompt does not point at it: `Basis` (C98, `search/corpus.rs:158`)
labels every answer with the facts revision, the intent revision, the
build and every table version; `Corpus::is_current` (`:837`) compares
it before every answer; a continuation is refused across a change
(C100). Correctness is built. What is missing is liveness: nothing
tells a page to re-ask.

Three facts the session needs in front of it:

- The daemon cannot carry intent changes. An agent's `price_set`
  writes the annotation file through the plan crate under
  compare-and-swap (C35, C52) and the daemon never sees it; C34 makes
  that structural ("the daemon is blind to intent"). So a daemon event
  cannot be the whole signal, whatever the subscription carries. The
  Rust frontend watches the two revisions itself (`Store::revision`,
  one transaction; `Annotations::revision`, a sum over rows), and job
  events are a progress signal for the queue view, not a data signal.
  The one data signal to the page is "the basis moved", from one
  watcher in the Rust process.
- That watch is a wait whose outcome is one value: under C157's
  criterion it stays a driver effect with its bound inside, not a
  machine row. The session should not build a machine for it.
- #25 is a different ask. Its trigger is "an agent consumer shows two
  reads where one would do" (`area:pricing`): one read of two
  revisions for the agent's own compare-and-swap. The GUI's live loop
  neither needs nor closes it. The prompt lists #25 under "the live
  loop's data side", which invites a cursor the basis already is.

The rule for the brief, in one sentence: **the basis is the only
version; a feed line, a page and a continuation name it, and nothing
else counts.** Any SSE sequence number, page-side cache generation or
"last event id" is a second version and should be refused in review.

**Change:** reword Q5 around liveness, name the basis as the cursor,
move #25 to "settled elsewhere or not this slice", add C98, C100,
C104, C108, C34, C35 and C52 to "Read", and the search crate's
`Holder` to "the machines to copy" as the data side's pattern beside
`Door` and `Life`.

## 5. The unit to design is the resident, and the keeper is a connection

The prompt asks for "the subscriber as a machine" (Q4) and separately
for who keeps the daemon alive (Q3). The thing a long-lived frontend
holds is larger than a subscriber, and there are two such frontends
today, not one later: `acq web` and `acq-mcp` both hold a `Holder`,
both connect per call through a fresh door, neither holds a
subscription, and the MCP never spawns in real mode (C13) and never
calls `jobs::wait`. The developer seat's own question, "what keeps a
third frontend from becoming a third copy", is answered by naming the
unit once, in the verbs crate (C1: it needs the daemon and the store
together): a door policy, the subscription with reconnect, the held
corpus, the revision watch, and one output, a stream of hints. The web
server and the MCP each construct one; the CLI, a process per verb,
never does. The subscriber machine proper (reconnect, resubscribe,
snapshot-after-subscribe, identity on reconnect) is the client crate's
under C157, as `Door` is, and the resident drives it.

The keeper then falls out of C3 with no mechanism: a subscription is a
connection, every connection holds the idle verdict (`server.rs:107`,
`:671`), so the daemon lives while the GUI is open. The property worth
stating in the brief's first paragraph: **the GUI open is what enables
the agent**, because the MCP may not start a daemon in real mode and
may use one the human started. That is scenario 1's "kept alive while
either is there" made structural. The MCP holding a subscription of
its own is then a design choice with one consequence to weigh (the
daemon lives for the agent session's length). #1's three folded items
(the foreground contract, the MCP's offline job reads, the no-keyring
remedy) are each small and mostly not the GUI's; the design should say
which it closes.

`IDLE_REJUDGE` (`daemon.rs:180`, 5 s while any clause but time holds)
is a daemon-side row with its fix already proposed in the
retrospective (a `Notify` on close-to-zero and the limiter's quiet
instant). It is a cost, not a design question; the brief can carry it
as one build step.

**Change:** Q3 and Q4 merged around the resident; the keeper stated as
a hypothesis to confirm, not a thing to design; `watch_jobs`
(`main.rs:2820`) named as the reference, with its pid check between
the subscription and the request connection, and the subscribe gap
named (events before `subscribed` are never delivered, which the
snapshot after it covers).

## 6. Launch morning's real risk is the reload, and the prompt buries it

Scenario 2 is hundreds of child jobs landing over minutes under rate
limits. Each committed tab moves the facts revision; a resident
consumer pays one whole reload per commit (M5, `GUI-NOTES.md` §5:
492 ms at 22,721 items, 214 MB peak; #95 measured a resident growing
from 195 MB to 884 MB over twenty alternating loads, an allocator
effect, not a leak). With a half-second debounce and a two-to-three
second ceiling the GUI reloads every few seconds for the whole
refresh: tolerable in CPU, uncertain in memory, and every reload
re-asks every open page. The prompt touches this in Q5's last clause.
It is the one mechanism bet in the slice and the retrospective's own
lesson (§7: "a mechanism bet gets a spike") applies.

Two honest answers exist and the design may pick either: #32's
incremental refresh over `item_events` (the store already records
them; the pin is equality with a whole load), or a rule that the GUI
is behind during a refresh and says so, which the basis makes honest
(the page shows which revision it holds against the one the queue is
writing). The second is smaller and may be enough for the owner's
22,000 items; the first is the right long-run shape. The brief should
name a spike: a mock refresh of a hundred tabs against a resident,
measuring reloads, peak memory and the page's re-ask rate, before
either is chosen.

**Change:** Q5 gains the spike; the deliverable's build list names
its mechanism bets and their spikes, as `4-0-design.md` §6 did.

## 7. Wire cost, honestly, is fixtures and tests

The prompt says to count the cost of every wire change honestly, and
"honestly" needs one more sentence or the session will contort to
avoid a twenty-line variant. Under C10 and C82 every frontend ships
beside the one `acqd` it would spawn and a change moves the contract
revision for all of them together; there is no compatibility matrix
to protect. The cost of adding an `auth` event beside `event` on the
subscription is a `Response` variant, a wire fixture, the exhaustive
match in `tests/wire.rs`, and the dashboard's unpaged answer (#177)
left as it is. The cost of a bounded wait on the request plane is
higher than it looks: it puts a deadline on the request plane, which
C160 says must sleep through `Clock` and C137 says is pinned by forced
expiry, and C86's "a read answers in milliseconds" verdict needs an
exception for it. The MCP's one-call wait needs no wire change at all:
the verbs crate's resident subscribes, snapshots and awaits the
event or its bound. Q2's "which of its two shapes" should admit a
third answer, the bounded wait client-side.

**Change:** one sentence in "How to think" stating where wire cost
lives; Q2 reworded to allow the client-side wait.

## 8. The rebuild under a long-lived frontend

Q3 asks the right question and should sharpen it. The GUI is the first
frontend whose own executable goes stale while it runs. The sequence
the owner will meet daily during development: `cargo build`, then any
`acq` use verb (interactive, `replace: true`) replaces the daemon
under the GUI; the GUI's subscription drops; the GUI reconnects and
finds a daemon of a newer contract than the one it was compiled with;
its own door, also `replace: true` from a terminal, stops that daemon
and spawns the sibling `acqd` on disk, which is the new one, which
still mismatches, and the door reports `StillAfterRespawn` (#116's
path, `door.rs:419-458`). Correct, and ugly: the GUI killed a good
daemon to learn it is itself stale. The design should give the
subscriber machine a terminal state ("this frontend is older than its
daemon; restart it") that the page renders, and should weigh whether a
reconnect ever replaces (a first open may; a reconnect of a frontend
that already matched once should not). `human_present` is read once
at construction (C147) from stderr, so a server whose terminal closed
keeps its first answer; the design should say that is intended.

The account is a second such state. `acq web` resolves one account at
its start or first request and never moves (`web/mod.rs`, the module
doc). A GUI used every day meets a logout, or a second account's
login, while open; today it would serve the old account's store
silently until a submit is refused. Scenario 1's "the owner signs in"
is ambiguous too: in the GUI, or at a terminal with the GUI noticing?
C13 makes login a human's act through the CLI; the prompt should say
the GUI notices and never drives it, and that the account the page
serves is a page state, which the auth event then keeps honest.

**Change:** Q3 gains the rebuild sequence as a case to design for and
the account as a lifecycle state; scenario 1 says where the sign-in
happens.

## 9. The handoff (Q6) has a token problem and an out-of-band answer

"How does the agent hand the owner what it found, so it appears in
front of him in the GUI?" presumes in-band. Two facts make in-band
expensive: the agent on `acq-mcp` has no way to learn the web
server's URL, and the URL carries the per-launch token (C138) that
authorises `apply`, which causes GGG sends; handing an agent the token
is handing it the human door. The zero-build answer works today: the
agent says the query's canonical text (C104) or a trade URL (C113),
the owner pastes it into the form. The in-band answer that uses the
live loop already being built is a saved query as user-scoped intent
(#42, whose trigger is "the first user-scoped kind actually
written"), which the GUI would list through the same "basis moved"
feed as a price write. That is elegant and it is a store ruling, not a
GUI one. The prompt should permit "nothing new; measure the friction
in the owner's own mornings" as a valid answer and name #42 as the
in-band candidate, so the session does not invent a third channel.

**Change:** Q6 reworded; #42 added to the issue list.

## 10. Smaller gaps

- **The issue list.** Add #42 (above), #100 (its "likeliest trigger"
  is 4-full's live loop: a typed `JobKind`, `frame.rs` shared between
  client and daemon, which a subscriber machine in the client would
  touch), #176 (the pages print CLI text; a cleanup the live loop will
  walk through). #17 and #7 are plausibly met by C165's failure kinds
  and #178's wording; the prompt's "to check" is right. #2
  (multiplexing) does not fire under one subscription per server; the
  session can say so in "What not to do" rather than weigh it.
- **"Read" gaps.** Named in §4 and §5. Also `TEST-SHAPE.md` for what a
  frontend's process test may assert, since the resident's tests will
  be the first long-lived-process tests in the frontends.
- **Size.** Eight questions each wanting a sub-design is a build spec.
  A budget line: the brief fits `4-0-design.md`'s size; what overflows
  is an issue or a later brief. And the retrospective's complaint,
  that a brief's line numbers go stale the day they are written,
  argues for citing by symbol where one exists and by line only where
  none does.
- **Scope the first slice must not include**, said once so the session
  does not re-derive it: Tauri; login or logout from the page; a
  second account; the LAN; price writes from the page unless scenario
  1 needs them (it does not; the agent writes prices in scenario 2 and
  the GUI reads).
- **"One slice or two."** The prompt leads the witness, rightly:
  scenario 1 needs no daemon for its search at all (the corpus is the
  store's), so its gap is liveness and cards, and scenario 2's gap is
  the queue over events and reconnect. The session will find the same
  division; the prompt could say "the evidence may show two, in this
  order" and save a page.

## 11. What to change, in one list

In the prompt:

1. A ninth question on the page: what it holds, what it may not, the
   one feed, the basis as its version; a C16 amendment drafted; no
   framework chosen here; the toolchain decided by the dense-table
   step's spike under a Node-free gate.
2. Q5 reworded around liveness; the basis as the cursor; the reload as
   the mechanism bet with its spike; #25 moved out.
3. Q3 and Q4 merged around the resident (door, subscription with
   reconnect, holder, watch, one hint stream) in the verbs crate, the
   subscriber machine in the client crate; the keeper as the
   connection, a hypothesis to confirm; the rebuild sequence and the
   account as lifecycle states; the sign-in placed.
4. Q2 admitting the client-side bounded wait; one sentence on where
   wire cost lives.
5. Q6 admitting "nothing new", with #42 as the in-band candidate.
6. "Read" gains C98, C100, C104, C108, C34, C35, C52, `Holder`,
   `watch_jobs`, `TEST-SHAPE.md`; the issues gain #42, #100, #176.
7. A size budget and a "not in the first slice" list.

Elsewhere, decisions the owner could make now, each shrinking the
session:

- The page contract as a goal, in his words, before the session (the
  C16 amendment is the session's to draft, but "the page holds nothing
  the URL and the server cannot rebuild" is a boundary and his).
- The gate stays Node-free unless a spike inside the build proves
  otherwise.
- Login stays the CLI's; the GUI notices (C13 already says it; one
  line in the prompt closes the ambiguity).
- `roadmap/README.md` carries the 2026-11-06 reorder (4-full's first
  slice ahead of `alpha.2`); today only the prompt records it.

Nothing in the registry needs amending before the session runs. The
rulings it will draft (C16's state sentence, C85 widened, the keeper,
the spawn policy for a windowless frontend) are all its to draft and
his to rule, in the order the prompt already sets.

## 12. Outside advice

Not needed on the system: the questions are internal and the evidence
is in the tree. Worth it on the prompt: a Codex read of the revised
prompt, under the project's review prompt with the prompt as the
commit, costs minutes and is the rhythm the project already runs on
briefs. The one outside question that would pay is narrow and
empirical, not architectural: whether a vendored virtualiser with no
build step carries a 26-column dense table at 22,000 rows in the
browsers the owner uses, which is the dense-table step's spike and not
this design's.

## 13. The feature groups, read against the tree (added 2026-11-06)

The owner's brain dump of what users want, four groups, each placed
against what exists and what layer it needs. Most players are on
Windows; the owner can boot his Windows box, run a VM, or run WINE.

1. **Searches: create, modify, switch, save, edit.** The language has
   its text and tree spellings (C104) and the trade URL as a third
   (C113); the web form emits trade JSON; continuation exists (C100).
   Missing: a saved search, which is user-scoped intent (#42, the
   `user.db` park) and store-side. Switching is navigation once the
   URL is the search (§3's page contract). A saved search the MCP can
   list and write is also the in-band handoff of §9, in both
   directions. No daemon, no live loop. This is the first morning's
   group and the one the old app was used for.
2. **Refresh strategies.** The sync policy already names, per realm
   and league, tabs and characters as `all` or an id list with a
   freshness window (`acquisition-plan/src/lib.rs:403-430`), and the
   plan compiles under it (C38, C44). The web has an ad-hoc selection
   page and no policy page. The gap is a page over the policy, with
   compare-and-swap carried in the form as the plan envelope is; and
   watching an apply land is the live loop's jobs side. Scenario 2.
3. **Pricing for forum shops.** `price_set`, `price_clear`, `price_list`,
   `price_show`, `price_status`, tab listings and `shop_render` exist
   (C120: rendered, pasted by hand). 4b excluded price writes from the
   page. The gap is a shop page: item and tab prices with their
   revisions in the form (C52), the rendered pages to copy. An agent
   pricing beside the owner is the intent-revision feed, the live
   loop's data side, and the one place "the GUI sees the agent's
   writes" matters.
4. **Feedback from humans and agents.** New. The pattern exists in
   `acq diagnose` (a redacted document) and the seat (an agent's
   report of where the tool stopped it). An outward send (Sentry,
   GitHub) is a registered surface with permission on the access
   method (C79) and a privacy question (names, uuids, bodies) before
   it is a feature; the zero-build first form is a diagnose document
   and an issue template a person pastes, as `shop_render` is pasted.
   Not the GUI's first slice.

What this changes in the read above: three of the four groups are
pages over verbs that exist plus intent writes from the page, and the
substrate under groups 1 and 3 and the handoff is one ruling, #42's
user-scoped intent, which is the store's and the owner's. That is the
pre-existing decision whose making, not revisiting, enables the
abstraction. The live loop serves groups 2 and 3, not group 1. The
scenarios in the prompt should be re-cut on these groups: morning 1 is
group 1 with the handoff as a saved search; morning 2 is groups 2 and
3 together.

Windows: the GUI is a browser tab, so the server runs where the
browser is. Launch morning on the Windows box natively from the
release archive (the Windows gate is green on the VM); the daily
development loop on the Mac; WINE not at all for a daemon that holds
named pipes and a keyring. A browser tab beside the game, reached by
alt-tab, is the loopback shape's form factor and an argument for it
over a windowed shell.

## 14. Before the recut (added 2026-11-06)

Three facts checked for the recut, then the owner's word on the
timeline.

- **A saved search's home decides whether saving one reloads the
  corpus.** The annotation file (`annotations.rs`, module doc) scopes
  rows as item, character, tab, substash or account, with kinds
  `buyout`, `note`, `sync-policy`, typed at the write door; a `search`
  scope would inherit compare-and-swap, tombstones, backup and C151
  for nothing. But the basis carries that file's revision as a sum
  over every row (`Annotations::revision`), and the holder reloads
  whenever the basis moves, so a save or rename would cost a whole
  reload in the web server and in the MCP for a write that changes no
  answer. #42's draft takes a position: a `user.db` with its own
  revision inside the world directory (C83, so the harness isolates
  it), or the basis's intent component counting only the kinds that
  bear on answers. The park's own text (`decisions/search.md` at
  `5862a1e2`) says only the write is parked, since a query already
  travels as a value (C91).
- **The milestones as filed:** League 3.30 and alpha.2 due
  2026-11-25; 4-full due 2026-12-18.
- **The owner's word, 2026-11-06:** the timeline is disregarded. The
  C++ Acquisition still works and he supports it separately for the
  league; the Rust GUI is built well on its own clock, with no
  compromise in the engineering to meet a date. Consequences: the two
  mornings stay the acceptance test but are no longer tied to the
  league date; launch morning is rehearsed in the mock's new-league
  rehearsal (the `launch_morning` golden) and validated live at the
  next league or the next large refresh; the 4-full milestone's due
  date is advisory; #32 and the test shape of the resident are built
  properly where the spikes say so, not deferred; the owner's own
  sitting at `acq web` remains the design's first input and is no
  longer urgent. The ruling goes to `roadmap/README.md`'s list and
  to the prompt's "owner's decisions" at the recut.

## 15. One brief or three (added 2026-11-06)

The owner asked whether one brief was an assumption worth revisiting.
It was. A design brief's readers are the designing session, the cold
reviewer and the owner ruling, not the builders, who read build briefs
cut from it; all three do worse on one long document spanning two
areas. What held one brief together was the page contract, and that is
now a pre-ruling. So the prompt is a shared head and three parts in
series, each with a consumer that exists before its build: part 1
morning 1's pages and the saved search (frontends, search, store; the
owner with an agent); part 2 the subscriber and the wire (daemon and
client; the CLI's own waits retire); part 3 the resident and morning 2
(verbs and the web; a mock refresh of a hundred tabs, then live). The
cost is three review cycles and three ruling packets, and the chance
that part 3 finds part 2's wire one event short, which under C10 is a
fixture and a match arm.
