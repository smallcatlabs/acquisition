# 4a — the 4-lite design: a renderer over the verbs, and its rulings

## Goal

The rulings 4-lite needs are in the registry and the 4-lite build
brief is written, so the build starts the day the owner rules. 4-lite
is the owner's call of 2026-10-24, widened on 2026-10-25: a loopback
web UI in users' hands for league 3.30, with the live-update loop
(SSE, the subscriber, reconnect) deferred to 4-full after the time
seam (phase 4-0, `roadmap/TIME-SEAM.md`). Its shape, settled with the
owner on 2026-10-25 before this brief was run: **a renderer, not an
app**. Every page is one document a verb already produces, rendered
as HTML the way the CLI renders it as text (C53, text is a function of
the JSON); the page carries no semantics (C46, shared semantics in
Rust). Two groups of pages: the search pages, shaped like the trade
site's search — the form, the results table, the counts — which need
no daemon; and the refresh pages — the tabs and characters listed with
a selection, the plan with its quote, the jobs — which walk the
review-then-apply loop the CLI already has (C38, a plan is a binding
authorization envelope). This is a design brief: it ends in rulings
and a build brief, not code.

## Read

- `roadmap/GUI-NOTES.md`: sections 2.3 (the loopback shape and its
  costs), 4 (the form over the trade translation), 7 (the card gap,
  which 4-lite does not close), 9 (the ruling packet, re-cut below)
  and 11. Section 12's build sequence is superseded by this brief's
  Do 6; sections 5, 6 and 8 are 4-full's.
- `roadmap/TIME-SEAM.md`: section 4's "not in 4-0" line and C137's
  template rule (a deadline is pinned by forcing its expiry), which
  bind 4-lite's tests from its first commit.
- `decisions/frontends.md` whole; `decisions/trade.md` C113 (a URL's
  league is a term; an id or JSON carries none) and C119; `decisions/search.md`
  C96 (realm is scope), C98 (the consumer holds the corpus and checks
  currency before every answer), C100 (one answer shape, with `next`);
  `decisions/plans.md` C38, C44 (apply refuses a moved policy
  revision), C111 (each request spent once); `decisions/pricing.md`
  C65 (intent writes carry their channel); `AGENTS.md` C1 (the edges),
  C12 (two surfaces), C46, C88 (the real provider and the human), C120
  (never posts), C122 (pinned once, at the lowest crate).
- The tree the pages render: `acq tabs`, `acq characters`, `acq search
  --trade`, `acq refresh --plan` and `--apply=FILE`, `acq jobs`
  (`CLI-REFERENCE.md`); `crates/acquisition-verbs/src/frontend.rs`
  (the `Frontend`, its `via` channel with `web` reserved, the two
  doors), `refresh.rs` (`compile` over a snapshot, `quote`, `apply`),
  `jobs.rs`; `crates/acquisition-search/src/answer.rs` (`Request`,
  `Asked::read`, `Rows { limit, sort, after }`), `trade.rs`
  (`decode_json`, `Decoded { realm: None, league: None }` for JSON);
  `crates/acquisition-mcp/src/main.rs`, the `search` tool, the thin
  wrapper every route copies; `search/trade-query/data/filters-2026-09-12.json`,
  the twelve groups the form is generated from (17 KB).
- `acquisition-cli/tests/apply_loop.rs` and `plan_json.rs` (C38's pins,
  which the refresh pages inherit), `acquisition-mcp/tests/pricing_tools.rs`
  (the equality-with-the-CLI shape the adapter's tests take).
- Issue #1, the keeper bundle: read its first paragraph only, to
  confirm it is for frontends that connect without a terminal.

## Input

Fixed: the owner's rulings of 2026-10-19 (loopback web UI first,
Tauri a later wrapper) and 2026-10-24 (4-lite before the league;
2g and 2h after phase 3; the time seam before 4-full); the owner's
word of 2026-10-25 (the renderer shape; the refresh pages in 4-lite;
the live loop still out). The ruling packet of `GUI-NOTES.md` section
9, re-cut below. The league date: phase 3's window, which 4-lite must
beat.

Facts checked against the tree on 2026-10-25, which the design rests
on: `search`, `show`, `tabs`, `accounts` and `characters` read the
store and never open a door; `refresh --plan` compiles offline and a
running daemon adds its quote; `--apply=FILE` reads a reviewed
envelope and spends each request once; `Frontend::human` spawns in
real mode when stderr is a terminal (C88), which a server launched
from a terminal has; the trade JSON input carries no league and no
term may stand beside it (C113), where the form must scope to one
league.

## Do

1. **State the shape first, in the build brief's Goal**: every page
   renders one verb document; the form's field names are the trade
   JSON's own paths from the committed filter dump, folded by the
   server into the object `decode_json` reads; no page evaluates,
   sorts, filters or composes anything itself (C46). Where a page
   wants something no document carries, that is a verb or search
   change first, never page work.
2. **Put the packet to the owner, question by question, his answer
   verbatim into the registry under the entry rule.** The questions
   that remain his for 4-lite:
   - Packet 1, C16's amendment: web rendering served from a Rust
     frontend over loopback; Tauri the shell to add if a windowed app
     is wanted.
   - Packet 2, the listener's security, written at full stakes once:
     the port can cause GGG sends through apply, so the ruling is
     written for that and not twice. The recommendation: 127.0.0.1 on
     a random port; a per-launch token in the URL path so every link
     and form action carries it and no other page can forge one; Host
     and Origin checked against its own; no CORS; mutation by POST
     only; the routes private to the page and unversioned, so they are
     no third surface under C12. What bounds the worst case is
     structure already in force, named in the Why: invariant 1 and the
     limiter govern every send, and a plan authorizes only the user's
     own tabs (C38). Binding beyond loopback is parked with its trigger.
     The apply route does not exist before this ruling does.
   - Packet 5, images: out; a "Not doing" line with its trigger (a
     `SURFACES.md` row before pictures).
   - **New: where the frontend lives.** An `acq web` verb in the CLI
     crate, or a sixth crate and binary. The lean is the verb for
     4-lite: users have `acq`, the door and the browser opener are
     there, the mock-session and seat skills run unchanged, no new
     release artifact or Gatekeeper story; the cost is axum in the
     CLI's build over the hyper and tower already locked. A later
     Tauri wrapper spawns `acq web` either way (`GUI-NOTES.md` section
     3: a frontend is a binary with no library target). The owner's,
     because it is a C1 edge, an arch-check entry and a release
     artifact.
   Not his for 4-lite, and why, each named in the build brief: packet
   3, the spawn policy, is 4-full's (a terminal-launched server is the
   CLI's case under C88; a windowless launch does not exist yet);
   packet 6, the table's shape, needs no ruling (the table is the
   answer's rows at a limit with `next`, C100; the server-windowed
   ruling waits for the dense table); packet 8, sort, is the rows
   view's `sort` as it stands; packet 9, the listing-only filters,
   render disabled with the catalogue's sentence, agent-facing design
   under delegation; packet 7, the toolchain, is answered by the shape
   (no build step; ES modules served as written; one small script for
   the stat picker at most); packet 10 is overtaken.
3. **The league beside the trade JSON** (packet 4's inbound half).
   The form emits JSON and must scope to one league, and C113 lets no
   term stand beside JSON. Decide the smallest spelling: the trade
   input carries a league (and realm) beside the query object, under
   C113's own logic that a URL's league is a term — a C113 amendment,
   agent-facing design under delegation, listed for the owner's review
   — against the two-step alternative (translate to text, then run the
   text with `league:` added, losing the clause fates). The lean is
   the amendment. The encoder (packet 4's outbound half) stays parked.
4. **The corpus**: the server holds one corpus and checks `is_current`
   before every answer, which is C98 as written and pinned; no new
   ruling. Name it in the build brief so the build does not load per
   request as the MCP does. The MCP's own move stays 4-full's.
5. **The refresh pages over the plan loop**, each a render of a
   document that exists: the tabs page (`tabs` and `characters`, no
   daemon, a checkbox per tab and character: the explicit selection
   `acq refresh` requires); the plan page (the envelope with the
   daemon's quote, as `--plan` renders it; the envelope in a hidden
   field, since a plan is serializable and immutable); apply (the
   envelope posted back whole, as `--apply=FILE`), answered by the
   jobs page (the `jobs` document, a meta-refresh tag while any job is
   non-terminal: the dumb poll, no script). C44 and C111 do the safety
   work: a moved policy is refused, a double-click or a reload sends
   nothing. Decide the one wrinkle: a selection is ad-hoc, and the
   ad-hoc kind is marked to retire once nothing submits it; the lean
   is to compile the selection as a plan (a transient coverage in the
   snapshot `compile` takes), so review and apply stay one road, and to
   say what C44's revision check means for a plan no stored policy
   authorized. The door is `Frontend::human("web")`; `web` becomes a
   written channel (C65).
6. **Re-cut the build sequence** for the build brief: the listener
   under the security ruling, Ctrl-C its whole lifecycle (no lock, no
   reopen, no idle exit: a terminal-launched server is a dev server);
   the search pages (the form from the filter dump, the fold, the
   results table with `next`, the counts, the clause fates as the
   answer reports them, the listing-only filters disabled); the
   refresh pages (Do 5); the README tour line and the generated
   references. No SSE, no subscriber, no `Searcher` loop, no revision
   push, no card (section 7's gap stays 4-full's).
7. **Write the 4-lite build brief from `TEMPLATE.md`**, its tests in
   C122's shape and nothing new: the fold equals the CLI's `--json`
   document for the same input over the fixture store (a form
   submission against `search --trade`, a selection against `refresh
   --plan --json`); each page a wording golden under
   `ACQ_UPDATE_FIXTURES=1` naming the facts a reader must find (a
   count, a row, a badge, a quote line); the listener through the
   shared harness in a mock world (the URL read from stdout; without
   the token refused; a foreign Host refused; with it answered); the
   apply loop in a mock world, waiting through the `wait` verb and the
   hang ceiling alone. Two done-when lines this file fixes in advance:
   every test obeys C137's rule (no wall-clock bound a slow machine can
   miss), and the moved-world review passes — every unknown-game-content
   path (a new item class, an untranslated mod, the league mechanic's
   items, exercised with 2e's game-diff and rehearsal machinery)
   degrades to display with a badge, never a refusal or an empty
   answer, because launch users meet exactly those items first. The
   badge renders a state the answer already carries (undecided with
   its reason, unknown flags, an unread body), so the review's pins
   live in the search crate (C122, the lowest crate) and the adapter's
   test asserts only that the badge renders that state; a path that
   refuses is a search-crate fix first, never the adapter's.

## Done when

- Each packet question 4-lite needs has the owner's answer in the
  registry, under 800 bytes each, bothy green; each question set aside
  is named in the build brief with its reason and its phase.
- The frontend's home and the league spelling are decided and written
  down, the first in the registry, the second as a C113 amendment or
  the reason it was not needed.
- The 4-lite build brief exists, its Read checked against the tree,
  its done-when carrying the C137 line and the moved-world review with
  its placement in the search crate.
- The corpus decision is written in the build brief with its reason.
- This brief's open items are answered in the design session's close,
  each taken, filed or declined by name; `roadmap/README.md`'s phase
  row and "Current step", which still say "read-only", are brought to
  the owner's word there.

## Not in scope

- Code, including spikes: a timeboxed spike the design wants is named
  in the build brief it writes. None is expected: the form is
  generated from a 17 KB data file with twelve groups, and every page
  renders a document that exists.
- The subscriber, SSE, reconnect, live invalidation, the keeper bundle
  (#1, for frontends that connect without a terminal), the spawn
  policy for a windowless launch, the held corpus in `acq-mcp`, the
  card and the dense table: 4-full, after 4-0 (`roadmap/TIME-SEAM.md`).
- Binding beyond loopback, images, posting anywhere (C120 — never):
  each parked with its trigger in the rulings this design writes.
- Price writes and policy edits from the page: an intent write from a
  browser is a design of its own, after the refresh pages have been
  used.

## Rules

`TEMPLATE.md`.

## Report

`TEMPLATE.md`; the design session reports to the owner directly.
