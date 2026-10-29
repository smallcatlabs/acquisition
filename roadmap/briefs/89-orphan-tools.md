# 89 — six tools nothing calls: a caller each, or deleted

Issue #89, from the 2026-10-26 assessment (`roadmap/STATE-2026-10-26.md`,
the appendix's "Orphan tools" row). The last of item 1 of "Pull-forward"
(`roadmap/README.md`) before #115, after #86's close at `9adad710`. The
rhythm is "A step". No fate below touches a release, what blocks one, or
a procedure the owner follows, so the step needs no ruling and has no
Do 1 of that kind.

## Goal

Every tool under `tools/` has something that runs it, a home that says
who runs it and when, or is gone with the way back cited. Six tools have
neither today: `census.py`, `notes-check.py`, `site-listings.py`,
`site-join.py`, `sign-acqd-breakers.sh` and `windows-checks.ps1`. And
the shipped-table generators that read committed inputs only are run by
the gate, as `category-table.py --check` already is, so a table and its
generator cannot drift apart unseen; the ones that read the `poe1` clone
keep the regeneration procedure as their caller, with the reason stated
where a reader of the gate looks for it. A tool kept with no caller is
work a session spends finding out whether it still runs; a tool deleted
with its commit cited costs nothing until someone wants it.

## Read

- Issue #89 (`gh issue view 89 --json body,comments`): the six, the
  generators, "kept with a caller named, run on a schedule, or deleted
  and cited at its commit". No comments. Issue #112 beside it: two
  repoe scripts type the `poe1` pin into their extracts' first line; not
  this step's (Not in scope).
- `AGENTS.md`: "Where things live", history ("A deleted text is cited
  as `<path>` at `<commit>`"); C48, raw SQL is not a surface; C79, a
  governed surface is read by a human and tooling only reads what the
  human recorded; the hard rule that a bare `acq` opens the owner's
  store.
- `tools/trade-check.sh`: the loop of `name tool flag` entries (38-50)
  and its header's story, number 9; `category-table.py --check` is the
  precedent, added when "the reading that closed step 7d found it six
  citations behind ... which nothing had checked" (14-16).
- `tools/game-diff.py`: `TABLES` (67-81), the shipped tables in the
  order they regenerate, each with its generator; `PROCEDURE` (82-84):
  "run the generators in this order, each then with --check"; the drift
  line, `--check`, which passes with no clone and otherwise holds the
  clone and three shipped headers to the pin (docstring, 31-35), and is
  in `trade-check.sh` already.
- `tools/sources.py`: `require`, the one refusal of an absent clone or
  one off its pin (132-137); `PINS["poe1"]` is `poe1@e2bd511a`.
- The six generators' docstrings, for what each reads: `totals-table.py`
  (three committed files under `search/`), `trade-stats-table.py` (one
  committed capture, its sha256 pinned), `category-table.py` ("all
  committed; nothing is fetched and no clone is read"),
  `class-table.py`, `defence-table.py` and `trade-forms-table.py` (the
  `poe1` clone, through `require`). `trade-forms-table.py`'s docstring
  already says "the gate, which has none, never runs it".
- `roadmap/README.md`, the phase table's 3b row: the regeneration at a
  post-launch pin, "every generator's `--check` green": the procedure's
  scheduled run.
- `.github/workflows/gate-linux.yml`: the `trade` step runs
  `tools/trade-check.sh` (129-130), with no sibling clone on the runner;
  the comment over the `breakers` job, "sign-acqd-breakers.sh is not
  here: it needs macOS's codesign" (144). `gate-windows.yml` runs no
  `trade` stage. `weekly.yml` (#86): the `mutants` job on
  `ubuntu-latest`, a Monday schedule and dispatch. `release.yml`: the
  macOS jobs `sign-macos` and `keyring-macos`, both needed by `draft`
  (218).
- `tools/sign-acqd.sh` (its header names the breakers, 37) and
  `tools/preflight.sh:56-60`, which runs it in every live driver's
  preflight: the breakers guard a tool on the owner's live path.
- `crates/acquisition-plan/examples/listing-report.rs:1-15`: the oracle
  comparison, our side, read by `site-join.py` "against the table
  `tools/site-listings.py` made". `SURFACES.md:22`, the trade site's
  row: access method `browser`, used "on demand: the bulk-exchange item
  list (issue 23), the seller-account search as the oracle for the
  listing state". Issue #23 is open (`area:pricing`, `trigger:user`).
  `REVIEW-CHECKLIST.md:49`: "A join that hides the condition it tests",
  pinned by `tools/site-join.py --self-test`, which nothing runs.
- `crates/acquisition-client/src/client.rs:1969-1974`, the breakaway
  flag's comment citing `tools/windows-checks.ps1 job`; `C112`, the
  derived endpoint, pinned by `acquisition-cli/tests/world.rs`
  (`decisions/daemon.md:19`); `PLATFORMS-SLICE.md`, the study the script
  served, deleted at `202e4659` when the slice closed.
- `crates/acquisition-store/src/lib.rs:1334-1342`,
  `crates/acquisition-cli/tests/daemon_stop_checkpoint.rs:1-10` and
  `crates/acquisition-plan/reference/currency-v1.toml:56`: the three
  places outside `tools/` that name `census.py` or `notes-check.py`.
- `CLI-REFERENCE.md:625`, `acq price show`: "both sides with their
  causes, the raw note beside the parse" for an item, a character or a
  tab: the reading `notes-check.py` made by raw SQL, through the read
  API.
- `tools/docs-check.sh`, 2b (174-209): a history citation `` `<path>` at
  `<hash>` `` must name the path as it was at that commit; 6 (306-349)
  checks backticked `*.md` names only, so a deleted tool's name is
  found by grep, not by the gate.

## Input

Measured on this brief's tree (`9adad710`), before the work, on this
machine (10 cores):

- Every mention of the six, by `git grep -n -F <name>`. Outside the tool
  itself and the dated records (the two `STATE-*.md` files, the reviews,
  `RUN-LEDGER.md:66`): `census.py` in `lib.rs:1339`,
  `daemon_stop_checkpoint.rs:5`, `currency-v1.toml:56` and
  `notes-check.py:4`; `notes-check.py` in `lib.rs:1339` and
  `daemon_stop_checkpoint.rs:4`; `site-listings.py` in
  `listing-report.rs:10`, `site-join.py:6` and the `about` lines of the
  three JSON files it wrote under `crates/acquisition-plan/reference/`;
  `site-join.py` in `REVIEW-CHECKLIST.md:49` and `listing-report.rs:9`;
  `sign-acqd-breakers.sh` in `gate-linux.yml:144` and `sign-acqd.sh:37`;
  `windows-checks.ps1` in `client.rs:1971`. The other `census.py` hits
  are `search/item-facts/scripts/census.py` and its readers, another
  tool.
- The generators' `--check` on a bare copy of the tree (`git archive
  HEAD`) with `ACQ_CLONES` naming an empty directory: `totals-table.py`
  exit 0 in 0.03 s, `trade-stats-table.py` exit 0 in 0.1 s,
  `category-table.py` exit 0 in 0.04 s; `class-table.py`,
  `defence-table.py` and `trade-forms-table.py` exit 1, refused by
  `require`. So the first two read committed inputs only, as the issue
  says, and the gate can run them on a runner with no clone.
- In the tree, with `../poe1` at the pin: all six exit 0;
  `class-table.py` 0.25 s, `defence-table.py` 0.1 s,
  `trade-forms-table.py` 8.77 s. `tools/trade-check.sh` whole: 2.9 s.
- `tools/site-join.py --self-test`: exit 0, "the reason precedence
  holds over 6 site rows and 7 store items", 0.06 s.
- `tools/sign-acqd-breakers.sh` with `ACQ_CODESIGN_IDENTITY` unset, over
  the built daemon: "all breakers hold", 0.14 s, the real-identity
  cases skipped. A cold debug build of `acqd` alone (`cargo build
  --locked -p acquisition-daemon --bin acqd` into an empty target
  directory): 30 s wall, 214 s CPU, 132 crates.
- This session's shell exports the owner's `ACQ_CODESIGN_IDENTITY`: a
  breaker run there signs scratch copies with his Developer ID.

## Do

1. **The two committed-input generators and the join's self-test join
   `trade-check.sh`'s loop**: `totals tools/totals-table.py --check`,
   `stats tools/trade-stats-table.py --check`, `sitejoin
   tools/site-join.py --self-test`. The header's story gains #89's
   sentence, and says why the three clone-reading generators are not
   there: they read the `poe1` clone, which the CI runner lacks and
   `require` refuses, so their caller is the procedure `game-diff.py`
   prints, run at a pin bump (phase 3b), and between bumps the drift
   line, already in the loop, holds their headers to the pin. That is
   the "stated reason" the issue allows for the class, defence and
   forms generators; `trade-stats-table.py` gets a caller.
2. **`census.py` and `notes-check.py` deleted.** Both are the pricing
   slice's one-off reads of the owner's files by raw SQL (C48), done:
   the census on 2026-09-03 is cited by date in the currency table's
   sources; `notes-check.py`'s two questions are answered as T6, T9 and
   T10 (its docstring), it opens the owner's real store by default, and
   `acq price show` now prints a target's raw note through the read
   API. Each mention outside the dated records cites the path at the
   commit before the deletion. One commit.
3. **`windows-checks.ps1` deleted.** The platforms study it served
   closed with `PLATFORMS-SLICE.md` (deleted at `202e4659`); its checks
   need two Windows accounts and an administrator, which no runner
   plays, and C112 is pinned by `world.rs`. `client.rs`'s comment cites
   the measurement at the commit before the deletion. One commit.
4. **`sign-acqd-breakers.sh` weekly on macOS**: a `sign-acqd` job in
   `weekly.yml` on `macos-latest`, the pinned toolchain, the rust cache,
   `cargo build --locked -p acquisition-daemon --bin acqd`, then the
   breakers. `ACQ_CODESIGN_IDENTITY` is unset on a runner, so it runs
   the refusals the 2026-09-12 audit asked for and skips the real
   identity. Cost: one macOS job a week, no minutes on a public
   repository; a cold build of the daemon alone is 30 s here on ten
   cores, more on the runner's three, cached by the next week. It blocks
   nothing and touches no release. `gate-linux.yml:144` and the
   breakers' header ("Run by hand after touching the script") name
   where it runs. Not the local gate: the owner's shell exports his
   identity, and the gate would sign with it on every session's run.
   Not `release.yml`'s `keyring-macos`: that makes a release wait on a
   development tool, which is a ruling for no gain.
5. **`site-listings.py` kept, its callers named.** It reads pages the
   owner saved in a browser (C79), so nothing automated can run it; its
   callers are the two on-demand reads `SURFACES.md`'s trade-site row
   registers: the oracle comparison `listing-report.rs` writes down, and
   #23's exchange list (`--exchange-items`). Its docstring names both;
   `site-join.py`, its other half, is in the gate by Do 1.
6. `tools/gate.sh --docs` after the brief; `cargo build --workspace`,
   then the full `tools/gate.sh` in the background, its log under this
   session's scratch; one commit per concern (Do 1 to 5), each message
   naming this brief; then "Current step" as built, review pending; then
   the Codex review over the range.

## Done when

- `tools/trade-check.sh`'s loop runs `totals-table.py --check`,
  `trade-stats-table.py --check` and `site-join.py --self-test`, and its
  header states why `class-table.py`, `defence-table.py` and
  `trade-forms-table.py` are not run there and where they are.
- `tools/census.py`, `tools/notes-check.py` and `tools/windows-checks.ps1`
  are absent, and `git grep` finds each name only in the dated records
  (the `STATE-*.md` files, `roadmap/reviews/`, `RUN-LEDGER.md`), in
  this brief, or cited `` `<path>` at `<commit>` `` with a commit that
  holds the file, which `tools/docs-check.sh`'s 2b checks.
- `.github/workflows/weekly.yml` has a macOS job that builds `acqd` and
  runs `tools/sign-acqd-breakers.sh`; `gate-linux.yml`'s comment and
  the breakers' header name it.
- `tools/site-listings.py`'s docstring names its two callers.
- Each of the six tools ends in one of the issue's three fates, and the
  close message lists them.
- What only a push shows (the macOS job's first run and its time) is
  named in the close with how the owner checks it.
- The gate is green on the step's tree, each stage by its own exit
  status.

## Not in scope

- #112, the two repoe generators' typed pin: its trigger is phase 3b,
  and no edit here touches either script.
- Running the clone-reading generators in the gate where the clone sits:
  9.1 s on every local gate, 8.8 of it `trade-forms-table.py`; raised in
  the report, not built.
- A self-test for `site-listings.py`: a new mechanism for a tool run on
  demand; raised if the review asks for one.
- The other tools under `tools/` and `search/*/scripts/`: the issue
  names six; a sweep of the rest is its own issue if the review finds
  another orphan.
- Rewriting dated records (`STATE-*.md`, reviews, `RUN-LEDGER.md` rows):
  they name what existed on their date.
- Running any workflow: that needs a push, which is the owner's.

## Rules

`TEMPLATE.md`. The step runs in the main tree on `main`, one session;
scratch under this session's scratchpad in a subdirectory named `89`.
Kill only the processes you started. No `acq` runs; nothing sends to GGG
or fetches the trade site, and no tool that reads a governed surface's
capture is run against one. `sign-acqd-breakers.sh` runs only with
`ACQ_CODESIGN_IDENTITY` unset. Never push.

## Report

`TEMPLATE.md`.
