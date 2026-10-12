# The contradiction read — findings

35 rows in `findings.tsv`, read at `d1583b2e`; every verdict cell is `open` and the owner's.
Counts: contradiction 11, stale-next 8, dangling 6, stale-shape 5, two-homes 5.
By confidence: high 9, medium 15, low 11.
Fix first: X2 (with X1 and X3, one fact: which repository authors a ground-truth claim),
X17 (session-close deletes a fired park that nothing has built, the week the shipping parks fired),
X11 (where a closed slice's open observations go; X12 and X13 are its two instances).
One cause covers X1 to X7: "authored master-side", seven passages against `AGENTS.md:66`.
What was left out is at the foot.

## The contradiction rows, and which side is believed

**X1.** `AGENTS.md:66` is right. `TESTING-NOTES.md:235` carries the August
rule (claims authored on the master-side branch, cherry-picked to the spike);
note 36 reversed it at the root swap (`brainstorming-notes/36-the-road-to-the-agent-release.md:119`,
"Facts about GGG are authored here now"). There is no master-side branch in
this repository to author on.

**X2.** `AGENTS.md:66` is right, for the same reason. The live-run skill's
"After" step (`.claude/skills/live-run/SKILL.md:89`) is the one a session
follows at the end of a live run, so it is the copy most likely to be acted on.

**X3.** `AGENTS.md:66` is right. `LIVE-TESTING.md:10` and its standing rule
at `LIVE-TESTING.md:46` both send a new claim master-side; the standing rule
is read before every first contact.

**X9.** `AGENTS.md:83` is right. `NETWORK-CLEANUP.md:30` says later work
keeps its five-command list green, and that list (`NETWORK-CLEANUP.md:26`)
names `cargo clippy -p acquisition-core`, a crate renamed to
`acquisition-daemon` at the daemon split (`DAEMON-SPLIT-SLICE.md:57`); the
list also lacks the build, the two document checks and rustdoc.

**X11.** `AGENTS.md:113` is believed: after closure an observation is an
area park with a trigger. The session-close skill
(`.claude/skills/session-close/SKILL.md:68`) still lists "observations still
open" as a part of a closed record. The two later records took the AGENTS
side (`SEARCH-SLICE.md:239`, each entry "became a park";
`DAEMON-SPLIT-SLICE.md:190`, "Where the open observations went"), so the
skill is the stale one.

**X12.** `AGENTS.md:113` is believed. `REFRESH-SLICE.md:107` keeps seven
observations as "data for the next slice that touches it", with no trigger
and no area home; one of them is already ruled (X14, C77 at
`decisions/plans.md:16`).

**X13.** `AGENTS.md:113` is believed. `PRICING-SLICE.md:239` keeps six
observations the same way. `SURFACES.md:24` cites that section as the home
of the forum's open matrix cells, so the pointer moves if they are routed.

**X17.** `CONTEXT.md:66` is right: a fired trigger deletes its entry in the
build's commit. `.claude/skills/session-close/SKILL.md:70` deletes a fired
park at a slice's close whether or not anything was built. The registry
holds parks that fired and stay on purpose (`decisions/plans.md:24`, "fired
2026-09-09 with nothing built"; `decisions/search.md:42`, "it stays
parked"), and the shipping-decision parks of `decisions/daemon.md:36` are in
that state now (X21): a session following the skill would delete them.

**X27.** `AGENTS.md:6` is believed, low confidence. Note 36's step 0
(`brainstorming-notes/36-the-road-to-the-agent-release.md:98`) says the line
becomes "read its status line"; it was drafted under the one-repository
ruling, reversed the same evening (the note's ruling 1, line 35). The ADR is
in `acquisition-qt` now, which an agent here cannot read, and AGENTS.md was
written at the swap. The note is the stale side on this one line.

**X32.** Neither side alone; low confidence. `CONTEXT.md:76` records
decisions after the code teaches, `CONTEXT.md:79` (P1) crystallizes before
building, and `CONTEXT.md:77` has design precede code. The practice since
the refresh slice is rulings before code (`REFRESH-SLICE.md:148`,
`PRICING-SLICE.md:281`, `DAEMON-SPLIT-SLICE.md:217`), so P1 describes what
is done; line 76 may mean findings and properties only, which is the owner's
to say. Both lines also say "here" and "this doc" for where a ruling lands,
which the area split made `decisions/<area>.md` (`AGENTS.md:106`).

**X34.** `LIVE-TESTING.md:31` is right, low confidence. The tripwire and the
ceiling are labelled "ladder-only" (`LIVE-TESTING.md:104`, `:123`) in a
section that says it states whether each rail outlives the ladder; the
ladder closed 2026-08-27 and both rails are what the standing rule and the
drivers arm today. The label reads as retired.

## Left out

- The titles "Rust playground" (`README.md:1`, `AGENTS.md:1`, the
  session-close skill's description) and the charter's "so the code stays
  replaceable" beside "This code ships" (`README.md:8`, `:16`): note 36
  step 0 says nothing else in the always-loaded set changes, so they read
  as kept on purpose.
- Park triggers that a step of note 36 fires where the park's text
  contradicts no other document: the parks read's.
- Pointers under `search/` that find no file (`search/legacy/REVIEW.md`
  from `search/README.md:90`): `search/README.md:72` sends every such
  pointer to `40b55e79`.
- Crate-relative paths written bare (`reference/…`, `examples/…`): the
  files exist under their crates.
- `SEARCH-SLICE.md:18` giving the rulings as C89–C107 (C109 came after the
  close) and `DAEMON-SPLIT-SLICE.md:24` "Seven crates": a record's state at
  closure.
- `TESTING-NOTES.md:73`, "the product still re-sends a dead grant", and
  `:165`, "the live ladder … stays human-run": each corrected within a few
  lines of itself.
- `tools/`, the workflow and crate docs for old-shape strings: not inputs.
