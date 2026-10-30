# map-before-seam — the docs say what their own file holds, and no script types the pin (#65, #134)

## Goal
Group A of the owner's 2026-10-30 open-issues review ("Closures, then
A and B"). The time seam's design session (4-0, `roadmap/TIME-SEAM.md`)
will read the daemon's module docs as its map. Today 39 files carry
copies of rulings ("Decisions as recorded" blocks), and some of those
copies have drifted from the registry and from the code. Five doc
comments cite a `CONTEXT` file that no longer exists. When this step
is done, each module doc states the mechanism its own file holds and
cites the registry by id (#65). Separately, every research script under
`search/` builds its poe1 pin from `tools/sources.py`'s `PINS` rather
than typing it (#134). Without that, phase 3b's regeneration after the
3.30 launch would label a fresh table with the old export.

## Read
- `AGENTS.md`, "Where things live": a ruling's mechanism is a doc comment
  on the code, headed by the id; the ruling itself is in
  `decisions/<area>.md`.
- Issue #65 and its two comments (`gh issue view 65 --comments`): the
  list of fixes and its done-when.
- Issue #134: the done-when, and C125 (a table names the export it was
  drawn from, read from the clone, never typed) in `decisions/search.md`.
- `search/repoe/scripts/base-defences.py`: the form to copy, a header
  built from `PINS['poe1'][:8]` (`tools/sources.py`).
- `tools/docs-check.sh` and `python3 -I tools/bothy.py check`: what they
  read in module docs. Learn this before cutting a block, so the cut
  doesn't strand a citation the checks count.

## Input
Measured on `main` at `4ec81eb5` (re-read before cutting; the lines move):

- **"Decisions as recorded" blocks, 39 files**:
  `acquisition-cli/src/plan_cmd.rs`, `acquisition-client/src/{client,locator}.rs`,
  `acquisition-daemon/src/{daemon,gate,rails}.rs`,
  `acquisition-daemon/src/daemon/{dispatch,kinds,queue,routes}.rs`,
  `acquisition-daemon/src/ratelimit/limiter.rs`, `acquisition-mcp/src/main.rs`,
  `acquisition-plan/src/{currency,game_side,lib,listing,price,shop}.rs`,
  `acquisition-protocol/src/{protocol,realm}.rs`,
  `acquisition-search/src/{answer,category,counts,eval,group,lib,price,properties,pseudo,sockets,totals,trade}.rs`,
  `acquisition-search/src/trade/{forms,translate}.rs`,
  `acquisition-store/src/{annotations,corpus,jobs,lib,world}.rs`.
- **`CONTEXT invariant` cites, 5**: `daemon.rs:938`, `rails.rs:715`,
  `provider.rs:5`, `provider.rs:13`, `ratelimit/choke.rs:226` (all under
  `crates/acquisition-daemon/src/`). Each means an `AGENTS.md` invariant
  by number; check the number still matches.
- **#65's single lines**, each still present:
  - `daemon/queue.rs:10`: "`waiting` jobs resume", which C134 and C50
    have made wider than the code. Point to `restore_jobs`'s doc instead.
  - `daemon/routes.rs:10`: "Only `/profile` is declared", where C32 and
    `NO_PROBE_ROUTES` name two routes.
  - `daemon/routes.rs:138`: "pending GGG's word on both endpoints",
    which N38 and N39 have settled.
  - `gate.rs:3`: "N3" means the deleted playground's work package, not
    ground-truth N3.
  - `ratelimit/limiter.rs:131`: `last_response` "when the Full policy
    was observed"; `assume_counted` also sets it.
  - `ratelimit.rs:13`: cites C15 for "the only `reqwest::Client`", which
    C15 does not say.
  - `acquisition-store/src/annotations.rs:35`: names `decisions/plans.md`,
    which holds none of its blocks.
  - `daemon/queue.rs:106`: the restore doc says the faulted probe's route
    stays closed. A completed login lifts the closure (`login.rs`,
    `forget_degraded`), so narrow the doc to the code.
  - `acquisition-daemon/src/lib.rs:5`: places sessions in `auth.rs`;
    find where they live and say that.
  - `daemon/queue.rs:751`: the `KINDS` doc says "every kind the daemon
    runs, in one place". Say what it holds, and name the other places a
    kind must be added (#65's third comment lists them). Fix the doc
    only; collapsing those places is #100's.
- **The registry's own wrong sentence**: C32 in `decisions/network.md:17`
  says "lives in one place". Changing a registry sentence is the owner's.
  Draft the corrected sentence in your report; don't commit it.
- **#134, typed poe1 pins** (`grep -rn e2bd511a --include='*.py' .`, 15
  lines). The issue's five scripts, `search/category/scripts/{round-k,round-j,round-j-read,expected}.py`
  and `search/pseudo-stats/scripts/percentile-shapes.py`, plus three the
  issue missed: `search/pseudo-stats/scripts/{candidates,defence-twins,percentile}.py`.
  `tools/sources.py:53` is `PINS` itself and stays. Read each hit: a
  header written into an output gets `PINS['poe1'][:8]`; a docstring
  names where the pin is read from; a hit that records a past fact
  stays, with the reason in your report.
- **CI at `3c3cabad`**: `Gate (Linux)` run 41140424622 success,
  `Gate (Windows)` run 41140424614 success.

## Do
1. #134 first, one commit: the eight scripts read the pin from `PINS`.
   Run each script's `--check` if it has one; otherwise import it
   (`python3 -I -c` with the right path) to show it still loads.
2. #65's single lines and the five `CONTEXT` cites, one commit.
3. The 39 "Decisions as recorded" blocks, by crate (one commit per
   crate is fine). In each block, cut every sentence that restates a
   ruling's text or *Why*, and keep the id. Keep every sentence about
   how this file does it (names, order, bounds, the reason a branch
   exists). When a block restates a ruling and the code has moved
   (`queue.rs:10`), the registry text wins and the doc points to the
   code. A sentence that is only in the doc and is neither registry nor
   mechanism (history, a rationale the registry lacks) goes in your
   report, not the bin.
4. `tools/gate.sh` once, on the final tree.

## Done when
- No file under `crates/` contains "Decisions as recorded" or
  "CONTEXT invariant".
- Each of #65's single lines above is fixed or cut, as the commit
  message names.
- `grep -rn e2bd511a --include='*.py' search` finds only lines that
  record a past fact, each named in the message.
- Doc comments only under `crates/`: `git diff 4ec81eb5 -- crates`
  changes no line outside a `//!` or `///` comment (check with a script
  and quote its output in the report).
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope
- Changing the registry (`decisions/*.md`): the C32 draft goes to the
  owner through the report.
- #100's kind vocabulary, #44's "folder" and the exempt list's three
  copies (now #141's): code, not docs.
- Rewording a module doc beyond cutting and the listed fixes.

## Rules
`TEMPLATE.md`. What differs:
- You work in the main checkout (`/Users/tom/Development/GitHub/smallcatlabs/acquisition`),
  with your scratch under
  `/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/70b56735-1662-47f4-9f40-9a9bbfd8aaf9/scratchpad/a/`.
  No other build runs while you work. Commit; never push.
- Rounds 2 and 3 are documentation only and get no review. Round 1
  changes script code, so the orchestrating session reviews it.

## Report
`TEMPLATE.md`.
