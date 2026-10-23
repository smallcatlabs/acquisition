# 2f — The review of the daemon, the store and the network, and its fix rounds

Written after the fact, on 2026-10-23, from the step as it ran. No
brief existed while 2f ran: its design and running record were working
notes that stayed local, outside this repository. This file states
what ran and the done-when the close commit checks. Everything it
cites is a commit, an issue or a file in this repository, except the
scan's own shape and counts, which come from the working notes kept
outside it. Written after the step, "How it ran" stands in for
TEMPLATE's Read, Input and Do.

## Goal

The daemon, the store and the network are read whole for faults, and
every fault that could make the daemon send to GGG unasked or without
bound is fixed by one rule, recorded first, pinned by a test that was
seen red. Everything else found has a home: an issue, or a line
answered in the close. It mattered because one finding let any client
send the user's bearer to a host of its choosing (`718f1cde`, C127,
the door's kind table).

## How it ran, against the plan

The roadmap planned `/code-review ultra` over the three crates. An
ultrareview reads a branch's diff against the default branch, under a
cap of 8,000 changed lines by default at the time (its documentation);
the daemon, store, client and
protocol crates were 38,144 lines at `3e53a45e` (JSON fixtures
excluded), so the plan could not run as written. What ran instead:

1. **A static scan at `3e53a45e`**, the night of 2026-10-21: eight Opus
   readers, one lens each, every report checked by a second reader,
   synthesised by Fable into fourteen root-cause groups, then three
   deep reads of the top hotspots. A Message Batches kit was built
   and never submitted. The scan's reports stayed local.
2. **The owner's rulings** on three packets drawn from the scan, each
   checked against the code by a second model before he read it.
3. **Fix rounds**: each unit that changes behaviour built with a test
   seen red, checked by an agent that did not write it, then gated;
   each round reviewed by Codex, and round A and round C's network half
   by a free ultrareview each.
4. **The stopping rule** (the owner, 2026-10-22): build now only what
   could make the daemon send to GGG unasked or unbounded; the rest is
   an issue or a line. Applied to everything left after round C, it
   gave round D and the issues of 2026-10-23.

The draft's round B (tests and documents) did not run: under the
stopping rule it became #64 and #65, and #51 took the invariant pins.

## The rounds

| Round | PR | Commits on `main` | What it built |
| --- | --- | --- | --- |
| A, the submit door | #54 | `718f1cde` to `12395d33` | C127 (a client submits only the kinds the table lists, every param judged at submit, refusals bounded); C128 (a bearer goes only to the API origin); a lazily started daemon in its own process group on Unix (C27); the exit's checkpoints release their lock, and the lock order on `struct Daemon` (`6ab1a924`) |
| C, the network half | #57 | `76be73d6` to `2fa31fc6` | the rulings commit; C135 (unbuilt, #55); C19's 429 floor; C130 (no redirect); C129 (a Cloudflare answer is a stop); C25's reach; C131 (the token 2xx); the ultrareview's fixes |
| C, the exit half | #58 | `0c4a0829` to `b1d9cf3c` | C133 (one exit, a bounded drain, signals); C134 (a panic aborts with a fault marker; a listener failure never ends the daemon); C132 (the real-mode lock's path); the owner's amendments (`1d72cf4f`) |
| D, the lost login | #60 | `26160d48` to `cf2137ef` | C50 amended (a lost login's profile fetch sends nothing in a later lifetime); C136 (the submitter `daemon` is the daemon's own); the harness's credential-store default (`2a9d5aa4`) |

Round C was split into two PRs because the whole round was past the
ultrareview's cap. The third free ultrareview is kept for the GUI work
(the owner, 2026-10-23).

## The rulings it recorded

- New: C127 (`718f1cde`), C128 (`c429c2a8`) and C136 (`26160d48`),
  under the owner's delegation; C129 to C134 (`76be73d6`, the owner's,
  accepted 2026-10-22); C135 (`f86c9b7e`, the owner's, unbuilt).
- Amended: C19 (a headerless send paced as counted; every landed 429
  holds its route), C22 (the frozen C++ design as a property source;
  on the token route its D8 yields to C131), C24 (a rejected grant is
  dead; a block is not one), C25 (a halt leaves jobs waiting, now at
  any point before the send), C26 (a 429 re-queues; one without GGG's
  headers is a block), all at `76be73d6`; C27 (a vanished client
  cancels nothing; the daemon in its own process group, `69c7eede`);
  C130 widened (`2b68f3d5`); C133 and C134 (`1d72cf4f`); C50 (the
  account key is the profile uuid; a lost login's fetch dies with its
  lifetime, `26160d48`).
- Ground truth: N48 and N49, Cloudflare's own documentation, and Q7's
  questions (`76be73d6`).

## Done when

Each line is checked by the close commit's message against the tree,
`git` or GitHub.

- C127 to C136 are entries of `decisions/daemon.md` and
  `decisions/network.md`; C135 says "Unbuilt" and names #55;
  `python3 -I tools/bothy.py check` exits 0.
- `git rev-list --count 3e53a45e..cf2137ef` is 57, the four PRs'
  ranges above, and #54, #57, #58 and #60 are merged.
- Both GitHub gates, Linux and Windows, passed on `cf2137ef`'s push to
  `main`.
- The seven reviews are whole at `b1588df3` and deleted by the close;
  `roadmap/reviews/` holds no `2f-` file.
- #61 to #72 are open; #51 is retitled to invariants 4 and 5; #56
  carries `trigger:gui`; #55 carries `trigger:user`; #59 is open for
  its first part alone.
- `roadmap/README.md`: the phases row marks 2f closed; "Open" has no
  "For 2f" entry, and each former item's home is named in the close
  message; "Current step" names 2g; the ledger has a 2f row.
- `REVIEW-CHECKLIST.md` names 2f where its faults came back.
- The gate's code stages hold by GitHub's Linux and Windows gates on
  `cf2137ef` (runs 39934930767, 39934930723); the close changes
  documents only, and its own `tools/docs-check.sh`,
  `python3 -I tools/bothy.py check` and `git diff --check` are 0.

## Not in scope

What the stopping rule left: every issue above, the architecture rows
deferred past 2f (under "Open"), and the residuals the checks named,
answered in the close. `HISTORY.md` waits for the phase close.

## Rules

`TEMPLATE.md`; the step ran before this brief, so no rule differs.

## Report

`TEMPLATE.md`. No single worker's report exists: the close answers
what the round workers, their checkers and the reviews raised that no
issue holds.
