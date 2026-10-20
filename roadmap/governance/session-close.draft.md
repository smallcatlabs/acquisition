---
name: session-close
description: End a working session on the Acquisition workspace — run the gate bare, commit with the story in the message, and if the session was a roadmap step or closed a phase, write its row. Use before the last commit of any session.
---

# Session close

## 1. Gate

Run the quality gate in `AGENTS.md`, "Quality gate", each command bare,
and read each exit status. A check piped through `grep` or `tail`
reports the filter's exit, not the check's; gate the commit on each
stage's own status variable, never on a `grep` of a summary (BSD
`grep -qv` once read "all green" over a failing line).

### Why build order matters

`cargo test --all-targets` never writes `target/debug/acqd` (no test in
the daemon crate asks for it) and rewrites `target/debug/acq-mcp` in
its all-targets form, a different artifact: proptest, the plan crate's
dev-dependency, unifies `num-traits/std` into the chrono that rmcp
links; `acq` and `acqd` have one form each; `cargo clippy` rewrites
none. So run the gate's `cargo build --workspace` before any smoke or
live run, and never run the drivers while the gate runs: their
preflight build replaces `acqd` under a live daemon, an artifact
mismatch (C84). An edit or a `cargo fmt` to any input of the contract
revision after the build leaves `acqd` on the old revision and every
daemon test failing "another contract": build again first. `cargo
build --workspace --release` and `cargo build --release -p
acquisition-cli` uplift two forms of `target/release/acq` from one
source, and a no-op build swaps them: a hash in a ledger names the form
that built it. The contract's inputs are listed in
[the revision build script](../../../crates/acquisition-protocol/build.rs).

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

A step ends with its ledger row in `roadmap/README.md` and its review
launched (`roadmap/README.md`, "Roles and rhythm"). A phase close adds
one `HISTORY.md` line, a `REVIEW-CHECKLIST.md` row for each shape that
came back during the phase, and deletes the phase's briefs and review
files, cited by commit.
