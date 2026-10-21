# 2d — The daemon and limiter file splits (build)

The build brief for the design in `2d-design.md`, which the worker
reads first and whole. A worker reads `roadmap/README.md`, then this
brief, then what it names, and nothing else unless the work turns it
up.

## Goal

`crates/acquisition-daemon/src/daemon.rs` and `ratelimit.rs` are each
a root with children in a directory of their name, every statement of
the old files in one of the new files unchanged, the daemon's tests in
a parallel tree of themed files, the limiter's spec one file, each file
routed to its area by its own module doc, and every pin in the registry
naming the file that holds its test by a path the check resolves.
Nothing behaves differently: the same 147 tests pass, the same bytes
go on the wire, and a script holds that the move was a move.

## Read

- `2d-design.md` whole: section 2 is the shape, appendix A the
  daemon's items by destination with the `pub(super)` list, appendix B
  the citations to edit. Section 7 is what you must not fix.
- `AGENTS.md`: the build order (build before any daemon, again after
  tests), the gate, "Tests pin behaviour at boundaries". The commit
  order below is the gate's, every time.
- `decisions/daemon.md`, `decisions/network.md`, `decisions/plans.md`:
  the *Pinned:* fields appendix B lists, and the three intros. The
  entry rule is `tools/bothy.py`'s docstring (rule 3: *Pinned:* names
  the file and the test); nothing but the file half of a pin changes.
- `crates/acquisition-search/src/trade.rs:1-60`: the one directory
  module in the tree, the layout to copy (a root file, children in a
  directory of its name, no `mod.rs`).
- `crates/acquisition-daemon/src/lib.rs`: the crate's module list and
  the `Rulings:` line (`:13`), the form every new file's module doc
  takes.
- `TESTING-NOTES.md`, "The journal is the contract surface" and "The
  suite's shape": the sentence to correct (`:81-82`) and the harness
  sentence (`:171`).
- `.config/nextest.toml`: no filter names a test or a binary, so the
  module paths may change.

## Input

- The tree at `446223ff`. The line ranges in the design's appendices
  are at that commit; you confirm each against the tree before you
  move it and name any range the design misplaces in your report.
- `daemon.rs`: 9,894 lines, 7,481 statements by the checker, 69 tests
  (`#[test]` and `#[tokio::test]`); `ratelimit.rs`: 4,509 lines, 2,539
  statements, 47 tests; the crate: 147 tests.
- The module-doc blocks and their sizes, which must survive whole:
  `daemon.rs:18-21` C6 (4,985 bytes), `:22-25` C23 (1,156), `:26-29`
  C31 (2,129), `:30-33` C32 (1,200), `:34-37` C43 (1,566), `:38-110`
  C49–C51 (4,994); `ratelimit.rs:33-36` C18 (1,026), `:37-39` C19
  (859). `gate.rs` is not touched.
- The checker, appendix A below, saved to your scratch as
  `move-check.py` and run as the appendix says. Its own failing input:
  one signature given an extra parameter prints `MISSING x1` and
  `ADDED x1` and exits 1 (checked by the design session on this tree).
- `bothy check`'s failing input for the pins: a *Pinned:* path with a
  slash to a file that does not exist prints `FAIL path` and exits 1;
  stage it once on one field, see it fail, revert it, and say so in the
  message of the commit that edits the pins.
- The Windows guest, if it is up: `ssh acq-win` (the Mac's ssh config
  names it; user `acqtest`, clone at `C:\Users\acqtest\src\acquisition`,
  the toolchain picked by `rust-toolchain.toml`). Update it by
  `git push acq-win:src/acquisition main:refs/heads/mac`, then on the
  guest `git merge --ff-only mac`, then
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  and `cargo build --workspace`. If the guest does not answer, say so
  in the report and leave the Windows gate to the owner's push.

## Do

1. **The limiter.** Create `crates/acquisition-daemon/src/ratelimit/`
   with `policy.rs`, `limiter.rs`, `choke.rs` and `spec.rs` as the
   design's section 2.3 places them; `ratelimit.rs` keeps the module
   doc's first 24 lines, holds `Clock`, `SystemClock` and `url_path`,
   declares the children (`spec` under `#[cfg(test)]`) and re-exports
   each production child whole (`pub use policy::*;` and so on), so
   every `crate::ratelimit::` path in `auth.rs`, `mockggg.rs`,
   `rails.rs` and the daemon compiles unchanged. Move the C18 and C19
   blocks under `limiter.rs`'s module doc. Each child's module doc:
   one sentence of what it holds, then `Rulings: \`decisions/network.md\`;
   the cross-cutting ones are \`AGENTS.md\`'s.` Widen the twelve the
   design names (section 2.3) to `pub(super)`. Reattach the four
   limiter paragraphs the design's section 2.4 lists, and no other.
   Run the checker: `python3 -I move-check.py <(git show
   446223ff:crates/acquisition-daemon/src/ratelimit.rs)
   crates/acquisition-daemon/src/ratelimit.rs
   crates/acquisition-daemon/src/ratelimit/*.rs`; it must print
   `ok … 0 missing, 0 added`. Edit the citations: `decisions/network.md`
   C17, C18, C19 (its limiter half), C20, C21 (its limiter half), C32
   (its limiter half), C33; `decisions/daemon.md` C31;
   `REVIEW-CHECKLIST.md:15-16`; `.claude/skills/live-run/SKILL.md:38`;
   `crates/acquisition-protocol/src/status.rs:4`;
   `tools/tracer-verify.py:21,68`; `lib.rs:3`. Gate, commit.
2. **The daemon's tests.** Create `daemon/tests.rs` (declared
   `#[cfg(test)] mod tests;` in `daemon.rs`, declaring the fourteen
   files of the design's section 2.2 table) and `daemon/tests/` with
   those files, each test and helper to the file the table names, the
   diagnostics test included; the three `#[cfg(test)]` hooks on
   `Sessions` stay where they are. Each test file opens with
   `use crate::daemon::*;` and, where it needs the harness,
   `use super::harness::*;`; a name the glob makes ambiguous is
   imported by its path instead. The harness's items are `pub(super)`.
   Checker: `crates/acquisition-daemon/src/daemon.rs` at `446223ff` against `daemon.rs` and
   `daemon/tests/*.rs` (and `daemon/tests.rs`). Edit the pins the
   design's appendix B gives for the daemon's tests (`daemon.md` C3,
   C6, C23, C50; `network.md` C19, C24, C87, C25, C26, C32, their daemon
   halves; `plans.md` C40, C42, C43), the comment at `daemon.rs:4243`,
   `TESTING-NOTES.md:171`, and the sentence at `TESTING-NOTES.md:81-82`
   to the design's section 7, item 1, naming
   `crates/acquisition-daemon/src/daemon/tests/harness.rs` as the
   oracles' home. Gate, commit.
3. **The daemon.** Create the ten children of the design's section
   2.1 table under `daemon/`, each holding the items appendix A gives
   it, each opening with `use super::*;` and a module doc of one
   sentence and its `Rulings:` line (two areas where the table names
   two); the root keeps what appendix A leaves it, the module doc's
   first nine lines with the C31 and C49–C51 blocks under them, this
   table in words (one line per child), the `mod` lines, and
   `pub use start::{run, rotate_if_over, DIAGNOSTIC_CAP_BYTES};` so
   `main.rs:39` compiles unchanged. Widen exactly the methods appendix
   A's last column names, `pub(super)`, and any the compiler then
   names, each listed in the message. Reattach the two daemon
   paragraphs of the design's section 2.4, and no other. Checker: the
   `daemon.rs` of commit 2 (`git show <commit 2>:…/daemon.rs`) against
   `daemon.rs` and `daemon/*.rs`. Edit the rest of appendix B: the three
   intros, `lib.rs:7`, `README.md:234-235,246`, `wire.rs:225`, the two
   `world.rs` comments, `persist-verify.py:78`, `tracer-verify.py:43`,
   `SKILL.md:48`. The ledger row for 2d in `roadmap/README.md` and its
   "Current step" (2d built, awaiting its review) ride in this commit.
   Gate, commit.
4. **Windows**, as the Input says: the guest's clippy and build on the
   tree of commit 3, their exit statuses in the report; or the sentence
   that the guest was down.
5. **The review**, as `roadmap/README.md` "Roles and rhythm" says, over
   `<commit 1>^..<commit 3>`, saved verbatim at
   `roadmap/reviews/2d-<commit 3 sha>.md`, committed alone with
   `git diff --check`, no finding fixed.

Commits: three, in that order, each message naming this brief and
telling the story: which ranges of the old file each new file holds
(so `git blame -C -C` has its pointer), the checker's `ok` line
quoted, the `pub(super)` tokens listed, the doc blocks' bytes confirmed
(`wc -c` over the moved lines), the citations edited. The full gate
bare before each (`cargo build --workspace` first, the nextest line,
then the rest), each stage's own exit status on its own line, the
commit inside an `if` on the ten statuses.

## Done when

- `crates/acquisition-daemon/src/ratelimit.rs` and `daemon.rs` exist
  beside `ratelimit/` and `daemon/`; `daemon/tests/` holds the
  fourteen files of the design's table; no production file under
  `crates/acquisition-daemon/src/` is over 1,000 lines, no test file
  over 1,400 but `ratelimit/spec.rs`; the per-file line counts are in
  commit 3's message.
- Each move commit's message quotes the checker's line, `ok … 0
  missing, 0 added`, with its statement count; the checker rerun by the
  reviewer on each commit's tree prints the same.
- `cargo nextest run -p acquisition-daemon` reports 147 tests, all
  green; `grep -rc '#\[test\]\|#\[tokio::test\]' crates/acquisition-daemon/src`
  summed is 147.
- Every `## C<n>` heading of the three module-doc copies appears once
  across `crates/acquisition-daemon/src/**/*.rs`, each block at the
  Input's byte size; `grep -rn "Decisions as recorded"` finds each
  file that holds a block and no other.
- Every new `.rs` file's module doc names its area, and
  `python3 -I tools/bothy.py pack --path <file>` prints that area's
  rulings and no other area's for each (two for `dispatch.rs`,
  `kinds.rs`); `bothy check` is green.
- `grep -n '`daemon.rs`\|`ratelimit.rs`' decisions/*.md REVIEW-CHECKLIST.md TESTING-NOTES.md .claude/skills/*/SKILL.md`
  finds nothing; every *Pinned:* field appendix B lists names its
  file by a path with a slash that exists.
- `main.rs`, `auth.rs`, `mockggg.rs`, `rails.rs`, `gate.rs` and every
  other crate are byte-identical to `446223ff` except the comments
  appendix B names.
- The gate green at each of the three commits on the step's own tree;
  the Windows clippy and build green on the guest, or the report says
  the guest was down.

## Not in scope

- Any change to a statement: the stale comment at `daemon.rs:1168`,
  the lock inversion in `exit_process`, the oracles' coverage and the
  probe-first exemption, the two harnesses' twins, `execute_inner`'s
  arms, the copies' drift (the design's section 7; each has a home).
  A paragraph reattached outside section 2.4's six is an edit.
- A rename of any item, file or test beyond the names the design
  gives the new files; a helper deduplicated.
- `mockggg.rs`, `rails.rs`, `gate.rs`, `auth.rs`, `provider.rs`,
  `artifact.rs`, `frame.rs`; `AGENTS.md`; `HISTORY.md`; any other
  crate's code.
- A bothy rule for slashed pins (raised to the owner).
- A kept `tools/move-check.py`.

## Rules

- Every `acq` runs in a mock world: `ACQ_PROVIDER=mock` and
  `ACQ_STORE_DIR` under your scratch. Never the real provider. This
  step needs no `acq` at all.
- The main tree and your own scratch subdirectory, named in the
  launch; kill only the processes you started.
- The gate (`AGENTS.md`, "Quality gate") before the commit; the commit
  gated on each stage's own exit status, never on a filtered grep. Never
  run the drivers while the gate runs.
- One commit per concern; the story in the message, which names this
  brief's file; a size stated in a message is measured after the last
  edit.
- A finding is a line in the report; never a new document, never a
  fix in a move commit.
- A draft the brief names is taken verbatim unless the tree disproves
  a sentence of it; each correction is named in the report.
- After the last commit, launch the review (`roadmap/README.md`, "Roles
  and rhythm"), save its findings verbatim at
  `roadmap/reviews/2d-<sha>.md` and quote them in your report; do not
  fix them before the owner has read them.

## Report

Returned to the session that launched you, in this order, each a short
paragraph or a table:
1. What landed: the commits, one line each.
2. What the gate said, as exit statuses per stage, per commit; the
   Windows guest's statuses.
3. Measurements: the per-file line counts before and after; the
   checker's three lines; the `pub(super)` count per file.
4. What you left out and why.
5. What surprised you, and what you would retract from your own report.
6. Raised for review: anything about the brief, the plan, the tools or
   this way of working that got in your way or that you think is wrong,
   and what would have made the job easier. Unstructured, a sentence
   each; the session that reads it answers every item in the ledger
   (taken, filed, or not taken and why).

## Appendix A — the checker

Save as `move-check.py` in your scratch. Standard library only.

```python
#!/usr/bin/env python3
"""A pure-move check over a file split: the code of the old file equals
the code of the new files, as a multiset of statements, after the things
a split must change are set aside. Set aside: blank lines; `use`, `mod`
and `pub use` declarations; module docs (`//!`); `#[cfg(test)]` and
`#[path]` attributes; a visibility token (`pub`, `pub(crate)`,
`pub(super)`) at the start of a declaration. Insensitive to rustfmt's
reflow when an item moves to another nesting depth: whitespace is
removed, a code run is cut into statements at `;`, `{` and `}`, and a
comma rustfmt drops when it joins lines (before `)`, `]`, `>`, `}`) is
dropped on both sides. A comment line is one statement of its own.

    python3 -I move-check.py <old> <new>...
    python3 -I move-check.py <(git show <sha>:crates/acquisition-daemon/src/daemon.rs) \
        crates/acquisition-daemon/src/daemon.rs crates/acquisition-daemon/src/daemon/*.rs

Prints what is missing from the new files and what they add that the
old file lacked; exit 1 on either. Standard library only.
"""
import re
import sys
from collections import Counter

SET_ASIDE = re.compile(
    r"^\s*(//!|(pub(\((crate|super)\))?\s+)?(use|mod)\s|#\[cfg\(test\)\]|#\[path\s*=)"
)
VIS = re.compile(r"^(\s*)pub(\((crate|super|self|in [\w:]+)\))?\s+")
WS = re.compile(r"\s+")
TRAILING_COMMA = re.compile(r",(?=[\)\]\}>])")


def statements(path):
    out = Counter()
    code = []

    def flush():
        if not code:
            return
        run = TRAILING_COMMA.sub("", WS.sub("", "".join(code)))
        for unit in re.split(r"(?<=[;{}])", run):
            if unit:
                out[unit] += 1
        code.clear()

    with open(path, encoding="utf-8") as fh:
        for raw in fh:
            line = raw.rstrip("\n")
            if not line.strip() or SET_ASIDE.match(line):
                continue
            line = VIS.sub(r"\1", line).strip()
            if line.startswith("//"):
                flush()
                out[line] += 1
            else:
                code.append(line)
    flush()
    return out


old = statements(sys.argv[1])
new = Counter()
for p in sys.argv[2:]:
    new.update(statements(p))
missing, added = old - new, new - old
for k, n in sorted(missing.items()):
    print(f"MISSING x{n}: {k[:160]}")
for k, n in sorted(added.items()):
    print(f"ADDED   x{n}: {k[:160]}")
verdict = "ok" if not (missing or added) else "FAIL"
print(f"{verdict}  {sum(old.values())} statements before, {sum(new.values())} after, "
      f"{sum(missing.values())} missing, {sum(added.values())} added")
sys.exit(1 if (missing or added) else 0)
```

Run it on each commit's tree before the gate; quote its last line in
the message. Rustfmt's reflow of a dedented test is invisible to it
(checked on this tree: 58 reflowed lines, 0 reported); an edited
statement is not.
