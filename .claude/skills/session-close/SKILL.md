---
name: session-close
description: End a working session on the Acquisition workspace — run the gate bare, commit with the story in the message, and if the session closed a step or a phase, write its close commit and rewrite the road's state. Use before the last commit of any session.
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

`tools/gate.sh` has an `artifacts` stage: it hashes `acq`,
`acqd` and `acq-mcp`, runs `cargo build --workspace`, and fails if the
build compiled anything or a hash moved, so a test run leaves the
executables that build writes (C84). `cargo build --workspace
--release` and `cargo build --release -p acquisition-cli` uplift two
forms of `target/release/acq` from one source, and a no-op build swaps
them, so a hash in a ledger names the form that built it.

## 2. Commit

One commit per concern. The message carries the story (what changed,
what the review found, what the run showed), because `git log` is
where the next session reads it. A size stated in the message is
measured with `wc -c` after the last edit, never recalled. A deleted
text is cited as `<path>` at `<commit>`.

A test or a check deleted is one line per retirement, `retired X;
held by Y`, Y running in the gate or in CI at that commit (C154, the
retirement rule). The close lists every such line from the step's
commits beside what the step pinned, and asks what the step's pins
now let it retire (C153, the stopping rule).

The message ends with `Co-Authored-By: Claude <model>
<noreply@anthropic.com>`, which the commit-msg hook writes as
`Assisted-by:` (C184, the commit trailer). The hook adds nothing when
the line is missing, so read the trailer after a commit:
`git log -1 --format='%(trailers)'`.

What else a session writes back, and where, is `AGENTS.md`, "Where
things live": a ruling to its area file, a finding to a test or the
message, work not done to an issue, a user-facing change to the help
and the regenerated references. Nothing else needs a home.

## 3. A step's close

The close is one commit, and its message is the step's record
(`decisions/process.md`, C178). It:

- checks the brief's or the issue's done-when line by line against
  the tree;
- answers each item the builder raised: taken, filed as an issue, or
  not taken and why;
- names what was retired and what was left open, each with its home
  (an issue, a "Not doing" line, or the reason);
- ends with the trailer `Step: <step id>`, above the `Co-Authored-By:`
  line, so `git log --grep='^Step: '` lists the closed steps;
- rewrites `roadmap/README.md`: "Now", "Next" and "Waiting on the
  owner" as they stand after this step, with nothing of what they
  said before.

A decision in the owner's classes that the step needs or took is an
issue labelled `trigger:ruling` (C179), opened before the close, and
the close says how many such issues are open
(`gh issue list --label trigger:ruling`). Nothing is left "for his
later read".

## 4. Before a push

A push stands alone in its command. When the range changes doc
comments, run `tools/gate.sh --only doc` first: CI runs that stage and
the full gate does not.

## 5. A phase close

Add the phase's line to `HISTORY.md`. Read the process text
(`roadmap/briefs/TEMPLATE.md`, the review prompts, the skills,
`REVIEW-CHECKLIST.md`) for lines that a type, a check or a ruling now
holds, and retire each with its `retired X; held by Y` line (C182).
Then `cargo clean` in each checkout whose work the phase folded, after
its last gate: cargo never collects a target directory, and the main
checkout's had grown to 124 GB by 2026-10-25.
