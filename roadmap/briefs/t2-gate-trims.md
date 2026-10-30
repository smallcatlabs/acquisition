# t2 — the gate trims and the real-time waits

Issue #138. The test-shape group's second step (`roadmap/README.md`,
"Pull-forward", item 2; `roadmap/TEST-SHAPE.md` §1 and §2 item 6),
after t1's close at `75c0bc44`. The rhythm is "A step". Do 1, the
gate's stage list, is ruled: the owner's "Accept as recommended" of
2026-10-29, in the README's rulings list.

## Goal

The gate after one store edit spends its time on what it must: the
doc stage leaves the local full gate for a CI job that, today, does
not exist at all; two stages whose property another check holds are
retired by C154 (the retirement rule: `retired X; held by Y`, Y
running in the gate or in CI); and the slow tests stop waiting on wall
time they do not test. Each retirement names its holder in its commit,
and the gate's stage table before and after, on this machine, is in
the commits.

## Read

- `AGENTS.md`: C153 (a finding joins the surface that holds it), C154,
  C84 (the artifact mismatch: a daemon run from a binary another build
  wrote), C122 (a behaviour pinned once; nextest, a process per test),
  C137 (a deadline's contract is its expiry behaviour, no wall-clock
  bound asserted); the "Build order" hard rule, line 60.
- `TESTING-NOTES.md`, "The suite's shape".
- `tools/gate.sh`, its header (the stages, `fixtures` at 64, `artifacts`
  at 19 and 90 to 151, `features` at 32); `tools/gate-breakers.sh`.
- `.github/workflows/gate-linux.yml`, the gate job and its comment at
  132 to 137 naming `artifacts`.
- `roadmap/TEST-SHAPE.md` §1, the measurements this step repeats.

## Input

The sites, read at `75c0bc44`:

| Site | What | Fate (the ruling) |
| --- | --- | --- |
| `tools/gate.sh:87` | `names=(fixtures build test clippy winclippy fmt diff docs arch trade bothy features doc artifacts tree)` | `fixtures`, `doc`, `artifacts` leave the full list; `doc` stays selectable by `--only` |
| `tools/gate.sh:64-70`, `241-246`, `281` | the `fixtures` header, function and case | retired; the `unset` at 82 to 84 kept |
| `tools/gate.sh:19-31`, `37`, `57`, `90-151`, `284` | the `artifacts` header, the hash helper and function, the case, and the sentences in other stages' headers naming it | retired, held by `features` (every release target, in the gate and in CI) and the workspace's feature pins, one block in `acquisition-cli`'s manifest |
| `tools/gate.sh:295` | `doc)` | kept for `--only doc` |
| `tools/gate-breakers.sh:106-111` | four `fixtures` cases ("the switch at 0", "empty", "at 1", "unset") | retired with the stage, held by 115 and 116, staged at 112 to 114 ("a stage run with the switch set sees it unset", "the same stage under --docs"), which stay |
| `tools/gate-breakers.sh:103` | the unknown-stage refusal's text, `--only takes one of: fixtures` | the pattern follows the new list |
| `tools/gate-breakers.sh` | no `artifacts` case exists | nothing to retire |
| `.github/workflows/gate-linux.yml` | no `cargo doc` step anywhere in CI; the comment at 132 to 137 names `artifacts` | a parallel job running `tools/gate.sh --only doc`; the comment follows |
| `AGENTS.md:60-65` | "held by the gate's last stage" | the sentence names `features` (the owner's ruling covers it) |
| `AGENTS.md:146-163` | "Quality gate" lists `cargo doc` among the gate's commands, and line 161 says the gate runs them "then the build again" | `cargo doc` under a line saying CI runs it, `--only doc` locally; "then the build again" goes with `artifacts` |
| `.claude/skills/session-close/SKILL.md:20-28` | "Why build order matters", the `artifacts` stage | rewritten to `features`; the README's "Open" item on it (line 251) leaves "Open" at the close |
| clippy's `--all-features` (`tools/gate.sh`, both gate workflows) | kept | the ruling: not retired |

The waits: `TEST-SHAPE.md` §1 named five tests over 5 s, 52 of 237
test-seconds: the new-league plan on the mock's real stash limiter
(`acquisition-cli/tests/new_league.rs`, 18 s), the web refresh loop
(`acquisition-cli/tests/web.rs`, 17 s), two trade tests each parsing
a 17 MB capture (12 s), four provider tests on the sleep job's 3 s
default (`acquisition-cli/tests/provider_default.rs`, 12 s). The build
measures them again first (Do 2) and works from its own list.

The last push's CI: `Gate (Linux)` 40998406288 and `Gate (Windows)`
40998406312, on `9d3930d8`; the build reads both before its first
commit and names their conclusions in it. The six commits after
`9d3930d8` are unpushed documents.

## Do

1. Ruled (the README, 2026-10-29).
2. **Measure before.** The machine alone, in the owner's window: the
   full gate warm (run it twice, take the second), then `touch
   crates/acquisition-store/src/lib.rs` and the full gate again; save
   both stage tables. `cargo nextest run --workspace --all-targets`
   warm with its slow-test report: every test over 5 s, its time, and
   what it waits on. Nothing committed; the numbers go in the commits
   below.
3. **The doc stage to CI.** A job in `gate-linux.yml` beside the gate
   job, running `tools/gate.sh --only doc`; the full list drops it;
   `AGENTS.md`'s "Quality gate" says so. One commit: `retired the doc
   stage from the local full gate; held by the Linux gate's doc job`.
4. **`fixtures` and `artifacts`.** One commit each, with their breaker
   cases, header sentences and the sibling sentences of the Input
   table, each carrying its `retired X; held by Y` line.
5. **The waits**, one commit per cause. A wait the test does not test
   goes: the sleep job's default passed short where the job's length
   is not the property; a capture parsed once per process (a
   `OnceLock`) or both tests in one; the limiter's and the refresh
   loop's waits forced where their expiry is the property (C137), and
   left with a line in the report where only 4-0's clock seam
   (`roadmap/TIME-SEAM.md`) can remove them. A fast-window policy on
   the mock is non-test code: allowed, and it sends the round to
   review.
6. **Measure after**, as Do 2, on the step's tree; the full gate's exit
   status gates the "built" commit, whose message carries both tables
   and the before and after of each wait.

## Done when

- The full list in `tools/gate.sh` holds neither `fixtures`, `doc` nor
  `artifacts`; `--only doc` runs; `gate-linux.yml` runs the doc job.
- Each retirement is a `retired X; held by Y` line in its commit, and
  every sibling sentence of the Input table follows.
- The "built" commit carries the stage table warm and after one store
  touch, before and after, and each wait's seconds before and after.
- `tools/gate.sh` and `tools/gate-breakers.sh` exit 0 on the step's
  tree; the Linux gate on `main` green with the doc job is checked
  after the owner's push, in the close.

## Not in scope

Clippy's `--all-features` (ruled kept). The breakers as positive
controls (t3, #139). The compile cost and the inline tests (t4, #140).
The daemon's real-time tests beyond the five (the seam, #131).

## Rules

`TEMPLATE.md`. What differs:
- The review rule of the group (the README, #137's ruling (c)): this
  step changes `tools/gate.sh` and a workflow, so its build is
  reviewed; a fix round that touches only tests or documents is not.
- Measurements and builds run only in the window the owner names; the
  machine is his during a league sitting.
- `touch` stages the store edit: no content change, git stays clean.
