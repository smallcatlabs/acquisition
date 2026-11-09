# Common brief: recover the minimal semantic model of one area of Acquisition

Repository: /home/user/acquisition (Rust workspace, read AGENTS.md first, ~3 min).
You are READ-ONLY. Do not edit, build, run `acq`/`acqd`, or run cargo. Do not commit.
Use Bash (cat/sed -n/grep/find) to read. Write your report to the file named in your brief
AND return the same text as your final message.

The registry (`decisions/*.md`) and `REVIEW-CHECKLIST.md` state what the code is SUPPOSED to do and
what pins it. Your job is to recover what the code ACTUALLY does, from the code, and to report where
the two differ or where a property is held by nothing structural. Every claim cites `path:line`.
Read the production code first; use tests only to see what IS pinned. A ruling whose "Pinned:" says
"nothing yet" is a prime candidate for an invariant enforced by hope: check whether a type, a lint,
an arch-check line, or a test holds it anyway, and say which.

Produce these sections, in this order, terse, tables where they fit:

## 1. State
Every value in your area that changes what a user sees (on a terminal, in JSON, in the web page,
in an MCP answer, in the journal/ledger they can read). For each: where it lives (memory / SQLite
table / file / keyring / env / process), who writes it, who reads it, whether it is a source of truth
or a derived mirror/cache.

## 2. Transitions
The state machines and mutation paths. For each transition: trigger, precondition, effect.
Then classify: which operations COMMUTE (order of two does not matter), which are IDEMPOTENT
(applying twice = once), which are ORDER-DEPENDENT. Name the specific pairs. Be concrete:
e.g. "restore then submit" vs "submit then restore".

## 3. Dependencies
For each externally visible EFFECT (a send to GGG, a disk write, a keyring write, a socket reply,
a printed line, a process exit): what must be true before it fires, and where that precondition
is checked. Flag any effect whose precondition is checked in a different task/thread/process than
the one that fires it (TOCTOU).

## 4. Findings
### 4a. Nondeterminism in awaits, retries and event order
`tokio::select!`, `join!`, spawned tasks racing on shared state, lock-then-await gaps, retries whose
count/backoff depends on wall time, events delivered in an order the receiver assumes but nothing
guarantees, HashMap iteration order reaching output, SQLite statement order across two transactions,
anything a client observes that depends on scheduling. For each: the race, the two orders, what a
user would see in each.
### 4b. Invariants enforced by hope
A property the docs/comments/registry claim that no type, lint, check, or test holds; or one held
only by a comment/discipline ("callers must..."). Include "Pinned: nothing yet" rulings in your area
that you verified are indeed unpinned, and any `// SAFETY`/"must hold"/"callers must" comments.
### 4c. One fact stored in two places, drifting
The same fact kept in two fields/tables/files/caches with no single writer or no CAS/sync, or
derived twice by two readers (two interpreters). Say which is authoritative and how they can disagree.
### 4d. Work that never reaches the screen
Values computed, fields populated, events emitted, table columns written, counters incremented
that no frontend prints, no JSON carries, no test reads, or that are overwritten before any read.
Also: dead branches, unreachable match arms, flags with one call site always passing the same value.

## 5. Refactors: provably equivalent vs trust me
List refactors that would simplify your area (ones you see as you read, and any the roadmap/briefs/
"Not doing" lines mention for your area). Split into:
- PROVABLY EQUIVALENT: a mechanical change whose equivalence a type check, an existing test table,
  or a pure-function property would establish. Say what establishes it.
- TRUST ME: a change whose equivalence rests on a judgment. For EACH, give the SMALLEST concrete
  input (a request, a header set, a byte sequence, a file state, an event order) that would break
  it, i.e. the case where old and new behaviour differ.

Keep the whole report under ~3000 words. Precision over coverage: ten verified findings beat
thirty guesses. Say "verified" or "suspected" on each finding.
