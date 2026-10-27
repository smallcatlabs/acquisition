---
name: session-close
description: End a working session on the Acquisition workspace — run the gate bare, commit with the story in the message, and if the session was a roadmap step or closed a phase, write its row. Use before the last commit of any session.
---

# Session close

## 1. Gate

Run `tools/gate.sh` and gate the commit on its exit status, never on a
filtered grep of its output. `--docs` serves a change that no test and
no build reads, which the session establishes before using it (tests
read `README.md`, the two references and `.gitattributes`, among
others); when unsure, the full gate. While it runs, edit nothing
tracked: the search's build name digests crate sources at build time
and again at test time, so an edit mid-run fails the run
(`acquisition-search/tests/build_name.rs`); draft the next change in
scratch.

### Why build order matters

The test and doc stages never write `target/debug/acqd` and rewrite
`acq-mcp` in another form, and an edit to a
[contract input](../../../crates/acquisition-protocol/build.rs) leaves
`acqd` on the old revision, so `tools/gate.sh` ends with `cargo build
--workspace` and a daemon is never started after a test run or such an
edit without that build (C84, the artifact mismatch). Apart from that
rule: `cargo build --workspace --release` and `cargo build --release -p
acquisition-cli` uplift two forms of `target/release/acq` from one
source, and a no-op build swaps them, so a hash in a ledger names the
form that built it.

## 2. Commit

One commit per concern. The message carries the story (what changed,
what the review found, what the run showed), because `git log` is
where the next session reads it. A size stated in the message is
measured with `wc -c` after the last edit, never recalled. A deleted
text is cited as `<path>` at `<commit>`.

What else a session writes back, and where, is `AGENTS.md`, "Where
things live": a ruling to its area file, a finding to a test or the
message, work not done to an issue, a user-facing change to the help
and the regenerated references. Nothing else needs a home.

## 3. A roadmap step or a phase close

The step close and the phase close are `roadmap/README.md`, "A step",
stated once there.

At a phase close, `cargo clean` in each checkout whose work the phase
folded, after its last gate: cargo never collects a target directory,
and the main checkout's had grown to 124 GB by 2026-10-25.
