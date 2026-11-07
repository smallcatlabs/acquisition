# 4-full's design: a GUI and an agent at one daemon, in three parts

*Draft prompt for three design sessions in series, to revise with the owner before the first runs. Kept on its own branch until the owner settles it. Recut 2026-11-06 on the feature groups and the critique beside it (`4-full-design-prompt-critique.md`), and split into three parts the same day: the shared head below, then one part per session. A session reads the head, its own part, and the closed design briefs of the parts before it, and nothing of the parts after.*

# The head, shared by every part

## Why these sessions exist
The owner wants to use Acquisition himself, every day, through a GUI and through his agents together. That is the fastest way to find out whether the tool is right, and the people most affected by the result are the owner and the agents who build and use it. Nobody else uses the project yet. The C++ Acquisition still works and he supports it separately for the league; the Rust GUI is built well on its own clock, with no compromise in the engineering to meet a date (his word, 2026-11-06).

Each session designs one part of 4-full and builds nothing. It ends in a design brief in the form of `roadmap/briefs/4-0-design.md`, with the rulings it needs drafted for the owner, reviewed cold per the rhythm in `roadmap/README.md`, then ruled, then built by Opus workers from build briefs cut from it. The parts run in series, each with a consumer that exists before its build: part 1 the owner at the GUI with an agent, part 2 the CLI's own waits, part 3 the owner on a mock refresh and then live. The page's contract comes before the wire across the parts, because the page decides what the Rust side must emit and what the Rust side can know decides what the wire must carry: design from the consumer inward, build from the daemon outward.

## What users want, and the two mornings
The owner's account of the feature groups, 2026-11-06. He does not have a solid feel for what users will want; the C++ app was used to find obscure items buried in long tabs and to manage forum shops. The groups, each placed against the tree:

1. **Searches**: create, modify, switch between, save and edit them, in service of finding specific items or groups of items. The language's text and tree spellings (C104), the trade URL (C113), the web form emitting trade JSON (C141) and continuation (C100) exist. Missing: the saved search, user-scoped intent (#42, ruled below), and pages that offer their own ways on (#176) and show the item as a card (#98). No daemon and no live loop. Part 1.
2. **Refresh strategies**: set up and manage stash and character refresh so a user need not refresh everything for an update. The sync policy already names, per realm and league, tabs and characters as `all` or an id list with a freshness window (`acquisition-plan/src/lib.rs`, `SyncPolicy`, `LeaguePolicy`), and the plan compiles under it (C38, C44). The web has an ad-hoc selection page and no policy page. Watching an apply land is the live loop's jobs side. Part 3, over part 2's wire.
3. **Pricing for forum shops**: item-based and tab-based prices, the rendered shop pasted by hand (C120). Every verb exists; 4b kept price writes off the page. Missing: a shop page with the revisions in its forms (C52). An agent pricing beside the owner is the live loop's data side. Part 3.
4. **Feedback** from humans and agents, through Sentry, GitHub or another channel. New. Out of every part: its first form is a diagnose document and an issue template a person pastes, and an outward send is a surface to register (C79) and a privacy question before it is a feature.

The design is judged by whether these two mornings work. They are the acceptance test, and the owner runs them himself as the validating consumer (P2). They are not tied to the league date; the second is rehearsed in the mock's new-league rehearsal (`acquisition-cli/tests/new_league.rs`, the `launch_morning` golden) and validated live at the next league or the next large refresh.

**1. A morning of searching** (part 1). The owner sits at the GUI looking for specific gear to test builds. He makes a search, narrows it, saves it under a name, switches to another, comes back. An agent beside him, over `acq-mcp`, helps: it proposes queries, narrows them, explains what it found, and saves one under a name so it appears in his list; he saves one so the agent can read it. Most of this is reading the stored corpus; refreshes are occasional, and a page reload showing them is enough.

**2. A morning of refreshing and pricing** (parts 2 and 3). The owner sets the policy for the league in the GUI, applies a plan and watches it land: hundreds of child jobs, rate limits that hold for minutes, failures with a way on, a daemon that may be replaced under him by a rebuild. Meanwhile an agent prices items beside him, and the GUI sees the agent's price writes without being told by hand; he prices tabs in the shop page and the agent sees his.

## The owner's decisions before part 1 runs
Rulings first. The first three are drafts in the registry's form for him to rule on, in his words, before part 1; every part takes them as fixed and may raise, not reopen, them. The rest are his words already given.

- **C16 amended, a sentence added (draft).** After "composes nothing itself (C46, shared semantics in Rust)": "A page holds no state the server cannot rebuild from the page's URL and one feed line: its identity is its URL, every document it holds carries its basis (C98), the basis is the only version there is, and the feed says only that a basis moved; a page rule that is semantics moves to the server and one that is presentation gets a browser-driven check." *Why:* a framework brings its own store and its own sequence number, and a page that holds state the basis does not label is a second version, the stale answer C48 forbids. *Pinned:* `acquisition-cli/tests/web.rs` (every page a golden of a verb's document), and whatever check part 1 finds for a page script.
- **C<next>, user-scoped intent (draft; the home of #42).** "User-scoped intent lives in one file of its own, `user.db`, inside the world (C83), written only through the store crate under revision compare-and-swap as annotations are (C35), with a revision of its own that no answer's basis carries. Its first kind is a saved search: a name, the canonical text (C104) and the scope; names, tags, the shop template, currency ratios and a last-posted basis are kinds to add as each is first written. It is per world, not per account: a search names no GGG id and runs over whichever account is chosen." *Why:* the annotation file's revision is a sum over every row and sits in every answer's basis, so a saved search there would reload every held corpus in `acq web` and `acq-mcp` for a write that changes no answer; and C35's file is keyed on GGG ids per account, which a search is not. *The alternative, his pick:* (b) a `search` scope in the annotation file, with the basis's intent component counting only the kinds that bear on answers; smaller, but it teaches the basis which kinds matter, a second interpreter of intent. The recommendation is the file of its own. *Pinned:* the store's tests for the new file, in the `c35_` tests' shape.
- **`decisions/frontends.md`, "Not doing" (draft).** A front-end build step (Node, a bundler) in the gate: not until a spike inside a build step shows a page's complexity needs a framework; a vendored, pinned script file is tried first. No framework is chosen in a design session.
- **Login stays the CLI's (C13).** The GUI never drives a login or a logout; it notices one and says so. The account a page serves is a lifecycle state of the frontend, not a constant of its process.
- 4-full's first slice moves ahead of `alpha.2` (2026-11-06); the two mornings above are the target; the timeline is disregarded (2026-11-06, quoted above); the three parts run in series, part 1 first.
- #142 (t6, one canonical mock world and the scenario corpus through both doors) is **not** a prerequisite. Every part runs on the existing harness, under one rule: it adds no private world or seed function of its own, only rows in the mock's one world. t6 follows, designed from the GUI's door as its third consumer rather than guessed before it exists. Say if a design shows this to be wrong.

## Your stake in this
You're a primary stakeholder twice over, and the design should be judged by whether it serves you:

- **As a user.** You are the agent in both mornings. What do you need from the tool to help a person who is looking at the same stash in a GUI? How do you hand him what you found, and read what he saved for you? How do you wait on a job that may hold for five minutes within one tool call, under a harness whose tool timeout you do not control? What would make you say something wrong to him?
- **As a developer.** Agents like you will build and maintain this, and cannot see a page. What shape keeps a third long-lived frontend from becoming a third copy of every rule? What can be pinned by tables of events instead of by timing, and what by a golden of a document instead of by eyes? What would you not want to inherit?

When the two seats want different things, say so plainly. Don't average them.

## How to think
Take your time. Read the system as it is now, not as the GUI notes describe it: they were written on 2026-10-18, before the time seam, the transport seam, the paged job list and the job record's times and failure kinds. Look for the pattern under the issues, not a list of fixes. The aim is for complexity to collapse into something simpler where practical: one unit a long-lived frontend holds in place of per-frontend waiting logic, one feed to a page in place of several polls, one version in place of several, one rule a reader can hold in their head.

Prefer shapes that make a wrong answer hard to write (AGENTS.md, "Structure over discipline"). Be suspicious of a design that covers every issue; say what each choice costs and where it strains. The basis is the only version: a feed line, a page and a continuation name it, and nothing else counts; an SSE sequence number, a page-side cache generation or a last-event id is a second version. Count the cost of every wire change honestly, and honestly means this: under C10 and C82 every frontend ships beside the one `acqd` it would spawn, so a wire change costs fixtures, the exhaustive matches and the dashboard's unpaged answer (#177), never compatibility. It is a valid conclusion that the protocol needs little change, if the evidence says so. A mechanism bet gets a spike, named in the build with what it measures. Cite by symbol where one exists and by line only where none does; a part's brief fits `4-0-design.md`'s size or less, and what overflows is an issue or a later part.

## Read, in every part
- `AGENTS.md` (loaded), `GLOSSARY.md`, `roadmap/README.md` (the step rhythm, the roles, phase 4's row), `TEST-SHAPE.md` (what a frontend's process test may assert).
- **The page contract above**, and C98 (the basis, `Basis` and `Corpus::is_current` in `acquisition-search/src/corpus.rs`), C100, C104, C108 (`Store::revision`), C34 and C35 (intent through the store alone; the daemon blind to it), C52, and the search crate's `Holder`, the one `acq web` and `acq-mcp` each hold.
- **Where a frontend speaks:** C46, C53, C143, C138, C139, C162, C163, C116; the "Not doing" lines in `decisions/frontends.md`.
- **The web frontend.** `crates/acquisition-cli/src/web/`, and `acquisition-cli/tests/web.rs`; `roadmap/briefs/4a-design.md` and `4b-build.md`, for what 4-lite built.
- The closed design briefs of the parts before yours.

## Deliverable, in every part
`roadmap/briefs/4-full-<part>-design.md`, every claim cited by symbol or path, commit, issue, ruling id or seat transcript. It ends with:
- the build, in steps ordered by risk, each with its consumer, what it retires (C154's form), the issues it closes by their own "Closes when", and its mechanism bets with their spikes;
- "Drafts for the owner": the rulings, each with your recommendation;
- "Raised for review": what a cold reviewer should press on, and what you found that belongs to a later part, said in one line each.

## Boundaries, in every part
- Design only. **Don't change code or rulings, and don't comment on, close or relabel issues.** Commit only the brief, and only when it's done.
- No framework is chosen here; the toolchain is the dense-table step's spike, in part 1's build.
- Group 4, feedback, is out of every part; say so in one line where it would otherwise tempt.
- The parts after yours are out: name what you found for them in "Raised for review" and design none of it.
- You may run the mock to see what a surface does today, under the mock-session skill. Follow the build-order rule. Never start a real-provider daemon or run a bare `acq`.
- No id in text a user would meet (C116), in any wording you propose.

# Part 1 — Morning 1: the page, the saved search and the handoff

Area: frontends, search, store. Consumer: the owner at `acq web` with an agent at `acq-mcp`, morning 1. No machine and no wire: nothing in this part waits on the daemon.

## Read, for part 1
- `roadmap/GUI-NOTES.md` §4 (the form over the trade translation), §6 (the table; server-windowed, keyed by item id) and §7 (the card gap: rows carrying every field, one maker with `show`).
- `decisions/search.md` whole and `decisions/trade.md` C113, C119, C141; `decisions/store.md` whole, C35's mechanism in `acquisition-store/src/annotations.rs` (the module doc: scopes, kinds, the write door, tombstones, export), since the ruled file copies its shape.
- The search crate's `answer.rs` (`Request`, `Asked::read`, `Rows`), `show.rs` (`Shown`), `derive.rs` (`Item`, what a card can show), the MCP's `search` and `show` tools and `describe`, the CLI's `search` and `show`.
- The issues: #42 (ruled above; the MCP tools, the CLI verb and the pages for it are this part's), #98 (cards; its "Closes when"), #176 (pages print the CLI's text; the routes no page has yet, listed in `roadmap/reviews/night-run-2026-11-04.md`), #19 (a default realm and league; adapters only, C96), #159 (describe's derived groups, if the form's picker meets it); and any open issue your reading adds.

## Questions, for part 1
1. **The page.** What a page holds, what it may not, and the one feed: its identity as its URL, every document with its basis, the feed saying only that a basis moved, the page re-asking what it owns. Morning 1 needs no feed; design the page so that part 3's feed changes nothing a page holds. What the browser-driven check covers and what the goldens cover. Where the dense table's virtualiser and the toolchain spike sit in the build, and nothing more about them here.
2. **The saved search.** Over the ruled file: the store's API in C35's shape, the CLI verb, the MCP tools (the session's, under C52 and C53), the pages: list, save, rename, open, delete. A saved search as a name for a URL under question 1. The handoff in both directions through it, with no new channel and no token shared: what the agent writes, what the owner sees, and the reverse.
3. **Cards and the pages' own ways on.** Rows that carry every field from the one maker `show` is (#98's "Closes when"), the card, the item page; every page spelling its ways on through `Paged` and not the CLI's text (#176). What exists and what the slice adds.
4. **What must exist for morning 1, and what can wait.** The slice, end to end, with the owner at the GUI and an agent at the MCP.
5. **The rulings**, in the registry's form, beyond the three in the head: any the saved search or the pages need, marked amendment or new.
6. **What not to do.** Shapes considered and rejected, and why.

# Part 2 — The subscriber and the wire

Area: daemon and client. Consumer: the CLI's own waits that exist today, `acq auth`'s poll and the watch that ends when the daemon goes, which retire under this part without any GUI. Read part 1's closed brief for the page's feed, which this part feeds nothing into directly: part 3 does.

## Read, for part 2
- **The starting sentence.** `roadmap/reviews/time-seam-retrospective.md`: Root B ("the door built the wait the frontends lack"), its two shapes, step G1 in §10, §11 question 5, and the `IDLE_REJUDGE` row in §3, which names the first long-lived subscriber as its trigger.
- **The rulings, whole, with their mechanisms in the code.** C85, C8, C10, C13, C3, C86, C147, C157, C160, C165, C166, C133; the "Not doing" lines in `decisions/daemon.md`, especially "a GUI-hosted daemon: refused; the GUI spawns `acqd` as the CLI does, and phase 4 records the spawn policy".
- **The machines to copy.** `crates/acquisition-client/src/door.rs` (`Door`, with its effective policy read once at construction and its replace path through `StillAfterRespawn`), `crates/acquisition-daemon/src/daemon/life.rs` (`Life`, and the idle verdict's clauses in `server.rs`: every open connection, a subscription included, holds it), the client's `Subscription` in `crates/acquisition-client/src/client.rs`, the daemon's `emit` and its broadcast channel of 256, and `auth_changed`, the `Notify` t5 added that fires at eleven sites and that nothing on the wire reads.
- **The reference subscriber.** `watch_jobs` in `crates/acquisition-cli/src/main.rs`: subscribe, then snapshot over a request connection, with its check that both connections reached the same daemon; events emitted before `subscribed` are never delivered, which the snapshot covers. `acquisition-cli/tests/watch_recovery.rs`.
- **What polls today.** `crates/acquisition-verbs/src/jobs.rs` (`wait`, 500 ms, no bound), `acq auth`'s wait (`login` in the CLI's `main.rs`, 500 ms), `acq dash` (250 ms), the MCP's job tool texts (`MCP-REFERENCE.md`, "poll job_status, then job_result"); the MCP connects per call, holds no subscription and never calls `wait`.
- The protocol: `crates/acquisition-protocol/src/protocol.rs` whole, `tests/wire.rs` and its fixtures.
- The issues: #1 (the keeper; read its three folded items and say which this part closes), #15 (reconnect), #3 (`spawned_by`), #2 (multiplexing; one subscription per server does not fire its trigger: reject or show otherwise), #177 (answers still whole after the paged list), #14 (results over a subtree), #100 (its "likeliest trigger" is this: a typed `JobKind`, `frame.rs` shared between client and daemon), #17 and #7 (possibly settled by C165's kinds and #178's wording; check and say).

## Questions, for part 2
1. **The subscriber as a machine.** In the client crate under C157 and C160, beside `Door` and `Life`: its states, events and effects, the table that pins it, where the criterion in C157 draws its edge. Reconnect with identity: after a rebuild, `acq` replaces the daemon under a long-lived frontend; the frontend reconnects, finds a newer contract, and its own door would stop that daemon and respawn the sibling, which still mismatches. Design the terminal state a frontend renders ("this frontend is older than its daemon") and say whether a reconnect ever replaces. `human_present` read once at construction (C147): say that is intended for a server whose terminal closed.
2. **The keeper.** Confirm or refute: the subscription is the keeper, since every connection holds the idle verdict, so a GUI open is what enables an agent in real mode (C13). `IDLE_REJUDGE` as one build row, with the retrospective's fix. The spawn policy for a frontend without a terminal, worded for a later shell and changing nothing for `acq web` from a terminal.
3. **The wire.** Which of Root B's shapes, with the third admitted: the subscription widened (auth events over `auth_changed`; a stopping event, if a page should say it; the account as a lifecycle state, a logout or a second login while a page is open), the bounded wait on the request plane, or the bounded wait in the verbs crate over the subscription with no wire change. What each frontend gains, what each costs in fixtures, in the daemon and in the tests; the dashboard's answer (#177) beside it. The agent's one-call wait is the first change or a bound under the harness's tool timeout, never "until done": say which shape gives it.
4. **What this part retires** in the CLI: the login poll, the watch's end, `wait`'s 500 ms, in C154's form.
5. **The rulings**, in the registry's form: the C85 amendment if any, the spawn policy, the keeper, anything else, marked amendment or new.
6. **What not to do.** Shapes considered and rejected, the standing decisions confirmed among them (C34, C13, C85's hint semantics).

# Part 3 — The resident and morning 2

Area: verbs and the web, with the MCP as the second consumer. Consumer: the owner on a mock refresh of a hundred tabs with an agent pricing beside him, then live at the next league or large refresh. Read parts 1 and 2's closed briefs: the page's feed as part 1 reserved it, the subscriber and the wire as part 2 built them.

## Read, for part 3
- `roadmap/GUI-NOTES.md` §5 (the live results loop, still mostly right: one `Arc<Corpus>`, two invalidation sources, a debounce, one SSE line, pages re-asking) and §8 (the foundation moves, and where the time and the memory go).
- `decisions/plans.md` C38, C44, C111; `decisions/pricing.md` C52, C65, C81; the sync policy (`acquisition-plan/src/lib.rs`, `SyncPolicy`), `refresh.rs` in the verbs (`compile`, `quote`, `apply`, `Opened::landed`), the pricing verbs and `shop_render`.
- The web's refresh pages and the plan envelope in its hidden field (the pattern the policy and shop pages copy for their revisions); the MCP's `Holder` and its per-call door in `acquisition-mcp/src/main.rs`.
- The measurements: `search/M3-LEDGER.md`, `search/MEASUREMENTS.md`, `roadmap/briefs/p82-held-corpus.md` (where the 480 ms goes; 214 MB peak), #95's rows (195 MB to 884 MB over twenty alternating loads).
- The issues: #32 (an incremental corpus update over `item_events`), #95 (memory per reload), #13 (a queue UI), #14 (results over a subtree, if part 2 did not settle it), #25 (an agent's two reads where one would do: check whether the resident's watch settles it or it stays the agent's).

## Questions, for part 3
1. **The resident.** The unit a long-lived frontend holds, in the verbs crate (C1): a door policy, part 2's subscriber, the held corpus, the watch over the facts, intent and user revisions, and one output, a stream of hints. `acq web` and `acq-mcp` as its two consumers now; the CLI never constructs one. The watch as a driver effect under C157's criterion, not a machine. Whether the MCP holds a subscription of its own, and what that costs. The resident's test shape: a web server, a mock daemon and a store write from a third process, under `TEST-SHAPE.md`'s rules.
2. **The feed.** One line to a page, saying only that a basis moved and which; the page re-asking what it owns; the browser's reconnect as a re-ask, never a resume. The queue view over the jobs list and part 2's events.
3. **Morning 2's pages.** The policy page over the sync policy with its revision in the form, the apply landing watched, the shop page with revisions (C52), the agent's writes seen through the feed and the owner's seen by the agent.
4. **The reload.** Each committed tab moves the facts revision and a resident pays one whole load (M5; §5's figures; #95's growth). The spike: a mock refresh of a hundred tabs against a resident, measuring reloads, peak memory and the page's re-ask rate; then #32's incremental refresh, or a page that says it is behind, which the basis makes honest; say which and why, built properly where the spike says so.
5. **What must exist for morning 2, and what can wait.**
6. **The rulings**, in the registry's form: anything the resident, the feed or the pages need, marked amendment or new.
7. **What not to do.** Shapes considered and rejected, the standing decisions confirmed among them (C98, C48, C139).
