# league-seat-fixes — the league seat's first fix round

## Goal
The faults the league seat's first run found (Phase 3's first seat,
`audits/league-2026-11/seat/`, run 1: `REPORT-cli-1.md`,
`REPORT-mcp-1.md`, their `calls-*-1.csv` and `DEBRIEF-*-1.md`), each
fixed by one rule and pinned, so the rerun meets none of them. Both
runs finished every task; what stopped them is the words and the
printed commands around the league loop and the price receipts, which
the owner's live run of League 3.30 will meet first.

## Read
- `AGENTS.md` (C11, the failure document and its offers; C116, no id in
  user-facing text; C122, a wording is one golden, regenerated under
  `ACQ_UPDATE_FIXTURES=1` and naming the facts a reader must find;
  C153, a finding joins the surface that holds it; C156, a printed
  command runs as printed in the shell it is quoted for).
- `decisions/frontends.md`: C11, C53, C121 (one word for an account that
  cannot be picked: kept, not this round's), C146, C149; `decisions/pricing.md`
  (C69, C78, C81). `REVIEW-CHECKLIST.md`, "Frontends and the surface an
  agent meets" and "Pricing and intent".
- The seat skill, step 8: one commit per fault, each held by a test in
  `crates/acquisition-cli/tests/seat_faults.rs` named for the fault
  (read its head and `seed_two_leagues`; the harness's `login`,
  `start_daemon`, `Transcript`, `golden`, `sole_json` in
  `crates/acquisition-cli/tests/harness/mod.rs`); the MCP's twin pins in
  `crates/acquisition-mcp/tests/` (`tool_refusals.rs`, `pricing_tools.rs`,
  `plan_loop.rs`, `spoken.rs`).
- The printed-command machinery: `crates/acquisition-verbs/src/refusal.rs`
  (`Way`, `Surface::request`, `offer`; the CLI and MCP spellings in each
  frontend's surface impl), `crates/acquisition-verbs/src/receipt.rs`
  (`price_undo`), `acquisition_search::answer::shell_quoted`.
- The sites, by fault, under Input.

## Input
The last push's CI on `main` at `43bb0958`: `Gate (Linux)` run
41291833070 and `Gate (Windows)` run 41291833225, both success.

The faults, each with the calls that show it (the CLI's are rows of
`calls-cli-1.csv`, the MCP's of `calls-mcp-1.csv`, their outputs under
the worlds' `calls/`), read against the calls by the orchestrating
session before this brief was written:

1. **Printed next steps that do not run as printed.** `policy set`'s
   "preview the work with `acq refresh --plan`" (`plan_cmd.rs` ~173,
   CLI call 24) names neither the account nor a league, so as printed
   it plans Standard and is refused (call 26); `refresh --plan`'s
   `next: acq refresh --apply --league Rehearsal` (`plan_cmd.rs` ~848,
   call 27) and `refresh --apply`'s `next: acq refresh --plan …`
   (`remaining_line`, ~489; calls 48, 49) name no account, so with two
   accounts known they are refused (call 28); `tabs`'s two next commands
   (`store_cmd.rs` `tabs_next`, calls 43, 44) the same. The price
   receipts already carry `--account` always (calls 75, 76, 78: the
   release seat's rule, `5e2daaa0`).
2. **A plan for a league the policy does not cover says what is wrong
   and not what to do.** `PlanError::LeagueNotCovered` (`acquisition-plan/src/lib.rs`
   ~305, raised ~2095 where `policy.league(realm, league)` is none;
   `policy.leagues()` ~737 holds the covered ones) prints "the sync
   policy does not cover league Standard on realm pc" (CLI call 26) with
   no league named and no offer; the MCP's `refresh_plan` meets the same
   `Fault::Plan` (`fault.rs` ~249, the document in `refusal.rs`).
3. **A price conflict refusal names the revision and offers nothing.**
   `refused_write` (`acquisition-plan/src/pricing.rs` ~640): a live row
   at another revision, "has a price at revision 2 (re-read and retry)"
   (CLI call 83, MCP call 59); a cleared row met by a set, "was cleared
   (revision 3); a new price names no revision" (CLI calls 98, 99; MCP
   call 68) while the clear's own receipt prints the create as
   `--if-revision 0` (call 84) and the words step made 0 the create-only
   form on both surfaces (`8ae99e86`). After a replace, the create's
   undo is stale and the replace's undo puts the prior back, so no
   printed command removes the row (CLI calls 76, 78, 83, 84; MCP calls
   51, 53, 59, 60): both runs built the clear by hand from the refusal.
4. **A price set on an item the shop cannot post says nothing of it.**
   `price set` on an item in a map or unique substash lands silently
   (CLI call 76, MCP call 51); only `shop render` says "blocked
   (substash)" (CLI call 82, MCP call 57: `Cell::Substash`,
   `acquisition-plan/src/shop.rs` ~311 the rule's words, ~728 the
   reading of an item's `location` as `PriceTarget::Substash`). The
   facts hold an item's place (`items.location_kind`, `location_id`;
   `Store::places_of`, `acquisition-store/src/lib.rs` ~2170, reads it);
   `pricing::set` (~759) reads `in_facts` before the write and nothing
   else of the place.
5. **`price status`'s invalid-notes line claims more than it knows.**
   "1 note starts like a price and does not read as one (no effect: the
   tab's name applies)" (`price_cmd.rs` ~401; the doc on
   `listing.rs` ~524) for an item whose tab "reads none" (CLI calls 65,
   69): the tab's name applied nothing.
6. **`diagnose` replaces an account name inside a longer word.**
   `~/seat-worlds/…` printed as `~/<account 2>-worlds/…` (CLI call 109):
   `word_edge` (`acquisition-verbs/src/diagnose.rs` ~811) counts `-` as
   an edge, so `seat-worlds` reads as the word `seat`; the unit test at
   ~872 pins that `Alicex` stays.
7. **`--describe sum(` is refused** ("`sum(` is nothing describe knows",
   CLI call 58) after the language's own refusal spelled the calls as
   `sum(`, `holds(`, `linked(` (call 57): `describe.rs` ~735 looks a
   name up as typed; the MCP's `describe` is the same function.
8. **`realm_needed` names `all` and offers it nowhere**: "pc | poe2, or
   all" with offers for pc and poe2 alone (`corpus.rs` ~1008,
   `error.rs` `offering_realms` ~209; CLI call 40, MCP call 25).
9. **The apply's next line names a cause it does not know.** "5
   requests more are planned now (what this apply revealed, such as a
   map or unique tab's substashes, or left undone)" (`remaining_line`,
   `plan_cmd.rs` ~489; CLI calls 48, 49) when the five had gone stale
   under a 60 s age. Held by three goldens: `goldens/seat_faults/an_apply_ends_on_what_is_still_planned.txt`,
   `goldens/seat_faults/a_spent_plan_applied_again.txt`,
   `goldens/journey/a_first_session.txt`, and `goldens/new_league/launch_morning.txt`.
10. **README sentences wider than the tool.** "Your agent": the daemon
    "exits an hour after its last connection" (README ~135), a default
    the knob changes (MCP call 5: 14400 s); "The tour"'s "Every command
    takes `--json`, and it is total: a failure is `{"error":…,"kind":…}`
    … its `offers` …" (~202) while a failed job's `--json` is its
    outcome record, `{job_id, account, outcome: "failure", error}` with
    no kind and no offers (CLI call 97; the `--help` says so); "A new
    league" (~230) says how to add a league to a policy and not how to
    write the first one so it cannot overwrite (CLI call 24, MCP call 9:
    both runs found `--if-revision 0` / `if_revision: 0` in the help and
    the tool text alone).

Not this round's, each with its home: a failed job's record with no
`kind` or `next` (#7); a tab's `price set` receipt counting nothing of
what the row decides (#48); `describe` whole over the door's cut and the
computed names unindexed on the MCP (#104, #105); refused bodies
unlisted on the MCP (#18); `not_logged_in` for a misspelled account
(C121, kept, raised to the owner at the close); a refused API command
starting a daemon (C3, kept: the refusal says so); the mock's tab
`index` 0 in a fetched body; the pristine store's growth on first open
(the build's migration). The orchestrating session files the comments.

## Do
One commit per fault, in this order; each fix one rule, never per
case; each held by a test named for the fault in `seat_faults.rs`, with
the MCP's twin where the fault stands on both surfaces (C122: a
behaviour is pinned once at the lowest crate that holds it, and a
frontend's test pins what its adapter adds, the spelling).

1. **Fault 1.** Every printed command names the account the verb ran as,
   quoted by `shell_quoted` as the price receipts are, and the scope the
   verb resolved: `policy set`'s next names `--account` and, per league
   the write named (for each realm written, `--realm` where it is not the
   default), `--league`, one command per league; `refresh --plan`'s
   `next: acq refresh --apply …` and `refresh --apply`'s `next: acq
   refresh --plan …` carry `--account`; `tabs`'s two carry `--account`
   (its `count` is a search request, which already names the account in
   its scope when spelled — check). Spell them through the existing
   machinery where a `Way` exists or is cheap to add (`Way::RefreshPlan`,
   `Way::RefreshApply`, `Way::Tabs` exists), so one place quotes for the
   declared shell (C156) and the MCP never prints a CLI line (the
   `spoken.rs` sweep). Pin: a world with one logged-in account and one
   store-only account (the seat's world), each printed next run as
   printed through `Transcript` and succeeding; the goldens that hold
   these lines regenerated and read as a diff.
2. **Fault 2.** `PlanError::LeagueNotCovered` carries the leagues the
   policy covers on that realm (additive field, filled where it is
   raised); the refusal's words name them ("the sync policy covers
   Rehearsal on realm pc, not Standard: name one with `--league`") and
   the document offers the plan for each covered league (CLI: a `next`
   or `offers` with `argv`, as the words step's offers; MCP: `offers`
   with `league` in the request). Pin on both surfaces.
3. **Fault 3.** A conflict refusal offers the request re-aimed at the
   revision it names: `Refused` carries what was asked (the verb and,
   for a set, its value), and the document offers the same write with
   `--if-revision <stored>` / `if_revision: <stored>` for a live row, and
   for a cleared row met by a set the same write with `--if-revision 0`
   / `if_revision: 0`, its words "a new price names revision 0, or
   none" (the create-only form, `8ae99e86`). The clear on a cleared row
   keeps its words. The receipts' undo lines stay as they are: the way
   back to nothing after a replace is the stale clear's refusal, which
   now offers the clear at the current revision. Pin: create, replace,
   the create's printed undo run as printed → refused with the offer;
   the offer run → cleared; a set at a wrong revision over a cleared
   row → the offer at 0 lands; on both surfaces.
4. **Fault 4.** `price set` and `price_set` on an item whose place the
   facts hold in a map or unique substash (and on a substash target)
   say so in the receipt: a `note` field (additive, absent otherwise)
   carrying the shop's own rule words (`Cell::Substash`'s, one fact in
   one place: expose the sentence from `shop.rs` and reuse it), and the
   CLI's text receipt a `note:` line. The write lands as before (C69:
   the row stands; the shop's policy is the shop's). Read the place as
   the shop reads it (`listing.rs`'s reading of an item's location), not
   by a second rule. Pin on both surfaces with an item in a substash of
   a seeded store; `describe`'s "pricing" entry gains the sentence.
5. **Fault 5.** The invalid-notes line says what the count knows: "(no
   effect: what stands above it applies, where anything does)", the
   same change to `listing.rs`'s doc; the literal in `price_cmd.rs`'s
   unit test updated.
6. **Fault 6.** The redactor replaces a name where it stands as a whole
   word, and a hyphen joins a word as an underscore does: `word_edge`
   counts `-` as no edge. Pin in the unit test: `seat-worlds` stays,
   `seat.db`, `stash@seat` and a quoted `"seat"` are replaced.
7. **Fault 7.** `describe` takes a call name with its `(`, as the
   language's refusals spell one, so `--describe sum(` and `holds(`
   reach the entry that holds them. Pin in the search crate's describe
   tests.
8. **Fault 8.** `offering_realms` offers `all` beside each realm, its
   reading "realm all"; the CLI's readings and the MCP's offers both
   gain it (`asked_by` sets `Realm::All`). Pin in
   `acquisition-search/tests/offers.rs`'s `a_realm_needed_offers_the_request_in_each_realm`.
9. **Fault 9.** The apply's next line names every cause the count can
   have: "(what this apply revealed, such as a map or unique tab's
   substashes, what it left undone, or what went stale since)"; the
   four goldens regenerated under `ACQ_UPDATE_FIXTURES=1` and read as a
   diff of what a user reads, the only change the words.
10. **Fault 10.** The README: "exits an hour (`ACQ_IDLE_SHUTDOWN`) after
    its last connection…"; the `--json` sentence gains "a job that fails
    is its outcome record, `outcome: "failure"` with `error`, not a
    refusal"; "A new league" says the first policy is written with
    `--if-revision 0`, which lands only where none is stored (the MCP's
    `if_revision: 0` the same), before the line on adding a league.
    `readme_tour.rs` runs the tour; read its checks before editing the
    tour block.
11. Regenerate both references (`ACQ_UPDATE_FIXTURES=1`) where help or
    tool text changed; read every changed golden and reference as a
    diff; name in the report each change no source change explains.
12. The full gate (`tools/gate.sh`) bare on the round's tree before the
    last commit; each commit message names this brief and the fault's
    calls.

## Done when
- Each of the ten faults is fixed by one rule and pinned as Do says, or
  named in the report as not taken and why.
- In a world with a logged-in account and a store-only account, the
  printed next of `policy set`, `refresh --plan`, `refresh --apply` and
  `tabs` runs as printed (a test in `seat_faults.rs`).
- `refresh --plan` on an uncovered league names the covered leagues and
  offers each, on both surfaces (tests).
- A price conflict refusal offers the write at the stored revision, or
  at 0 over a cleared row, on both surfaces (tests), and the create's
  stale undo recovers through the offer alone.
- A substash item's `price set` receipt carries the shop's rule words on
  both surfaces (tests).
- The redactor leaves `seat-worlds` whole (test); `--describe sum(`
  answers (test); `realm_needed` offers `all` (test).
- The invalid-notes line, the apply's next line and the three README
  sentences read as Do says; the goldens and references regenerated
  with no change a source change does not explain.
- `tools/gate.sh` green on the round's tree.

## Not in scope
- A `kind` or `next` on a failed job's record (#7); receipt counts
  (#48); the `describe` index's size (#104, #105); `store refused` on
  the MCP (#18); any rename of a JSON key (C146); `not_logged_in`'s
  word (C121, the owner's); a daemon started by a refused job verb
  (C3).
- A second undo line on a replace's receipt: the conflict's offer is
  the one rule.

## Rules
`TEMPLATE.md`. What differs:
- You work in the main checkout
  (`/Users/tom/Development/GitHub/smallcatlabs/acquisition`), on
  `main`, never a branch; your scratch is a `league-fixes/`
  subdirectory of your session's scratchpad. No other build or gate
  runs during the round. Commit after each fault's targeted tests;
  never push.
- Every `acq` you run is in a mock world (the mock-session skill);
  never a bare `acq`, never a real-mode daemon. The seat's worlds under
  `~/seat-worlds/league-2026-11/` are the record: read their `calls/`,
  run nothing in them.
- Touch nothing tracked while the gate runs.
- A ruling (an invariant, a boundary, the real store, the real API) is
  the owner's: stop and report rather than decide it. Agent-facing
  wording is yours: record each choice in its commit and list them in
  the report.
- The round changes product code, so the orchestrating session reviews
  it and a cold Codex review follows; write for that reader.

## Report
`TEMPLATE.md`, plus: the agent-facing wording decisions, one line each;
anything you judged narrow or left for review.
