# The whole-CLI seat (note 36, step 1) — 2026-10-13

Two runs of one brief (`BRIEF.md` at `d1a10dd4`; the first run's at
`2e1d8e51`) by a subagent blind to everything but the brief and the
README's tour, driving the release `acq` through the seat skill's door
(`.claude/skills/seat/seat.py`, which began here) in a
mock world over a copy of the owner's store. `REPORT-1.md` and
`calls-1.csv` are the first run (172 commands, the tree at `25332463`);
`REPORT.md` and `calls.csv` the rerun after the fix round (238 commands,
the tree at `c5f3b4a2`). The outputs are under gitignored `runs/`. Each
report's section 4, what the tour and the help did not say, is the
release README's input.

What each fault became. A fix is held by the test of
`crates/acquisition-cli/tests/seat_faults.rs` its commit names.

| Run | Fault | Fate |
| --- | --- | --- |
| 1 | account defaulting differs by verb family | kept as C51 has it, made visible: `73e49bac` |
| 1 | price verbs scoped to a league, inconsistently | fixed: `9cb2086d` |
| 1 | `--json` not total on a refused command line | fixed, C11 amended: `1c43a6f5` |
| 1 | one apply is not a sync | the apply says what is left: `23c5ac50`; the fan-out stays parked (`decisions/plans.md`) |
| 1 | `status`, `result`, `auth status` need a daemon | answered from disk: `73e49bac`; the MCP's parked (`decisions/frontends.md`) |
| 1 | `search --describe` is 117 KB | bare is an index: `54102b85` (touches how C97 is met) |
| 1 | another league's rows read as orphans | fixed: `9cb2086d` |
| 1 | nothing waits for a job; `jobs` unfiltered | `result --wait`: `73e49bac`; `jobs`: `c5f3b4a2` |
| 1 | the policy's fields in no help | fixed: `23c5ac50` |
| 1 | "keyring: ok" with the keyring disabled | fixed: `647d8dcf` |
| 1 | an empty-titled spoiler on a shop page | by design: the render's no-price row |
| 1 | a cancelled job's `result` exits 0 | fixed, C11 amended: `73e49bac` |
| 1 | progress and payload on one stream | fixed: `73e49bac` |
| 1, 2 | ids nobody outside the repository can look up | parked, fired; built with the release's text (`decisions/frontends.md`) |
| 1, 2 | not every failure carries a `kind` | usage and account kinds added: `1c43a6f5`; the taxonomy park stands |
| 2 | a plan file applied twice fetches twice | C111: a plan spends each request once; a replay sends nothing and says what spent it (after a design study the owner asked for): `188b0ae3` |
| 2 | `cancel` on a running job "does not stop it" | it does, at the job's next check; the words fixed: `76ef839b` |
| 2 | `daemon stop` ends an unpersisted login silently | fixed: `76ef839b` |
| 2 | a job verb's JSON differs with `--json` | by design (C53), now said in the help: `76ef839b` |
| 2 | `acq tabs` unbounded, its summary last | fixed: `76ef839b` |
| 2 | a league planned fresh after its items moved | the mock's: one item id in two leagues |
| 2 | `price set` on an unknown target exits 0 | by ruling (C64), with the note it prints |
| 2 | a row's revision restarts after a clear | not shown by the calls cited; run 1 saw it keep counting |
| 2 | a token's short life reads as a logout to come | the words fixed: `76ef839b` |

Not reviewed by the session: the task sections' counts, and all but
some twenty of the 410 calls. The rerun's rule breaches are its own
"Left out": it listed README's headings to find the tour, and read one
harness spill file of the tool's output.
