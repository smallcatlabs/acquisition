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
- `daemon.rs`: 9,894 lines, 342 items and 1,212 comment lines by the
  checker, 69 tests (`#[test]` and `#[tokio::test]`); `ratelimit.rs`:
  4,509 lines, 200 items and 432 comment lines, 47 tests; the crate:
  147 tests.
- The module-doc blocks and their sizes, which must survive whole:
  `daemon.rs:18-21` C6 (4,985 bytes), `:22-25` C23 (1,156), `:26-29`
  C31 (2,129), `:30-33` C32 (1,200), `:34-37` C43 (1,566), `:38-110`
  C49–C51 (4,994); `ratelimit.rs:33-36` C18 (1,026), `:37-39` C19
  (859). `gate.rs` is not touched.
- The checker, appendix A below, saved to your scratch as
  `move-check.py` and run as the appendix says. Its failing inputs are
  the appendix's five, each checked by the design session on this tree;
  stage one of them once (a signature given a parameter), see the
  `FAIL` line, revert it, and say so in the first move commit's message.
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
   `ok … 0 differences`. Edit the citations: `decisions/network.md`
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
quoted with its counts, the `pub(super)` tokens listed, the doc blocks' bytes confirmed
(`wc -c` over the moved lines), the citations edited. The full gate
bare before each (`cargo build --workspace` first, the nextest line,
then the rest), each stage's own exit status on its own line, the
commit inside an `if` on the ten statuses.

## Done when

- `crates/acquisition-daemon/src/ratelimit.rs` and `daemon.rs` exist
  beside `ratelimit/` and `daemon/`; `daemon/tests/` holds the
  fourteen files of the design's table; of the files this step
  creates or keeps as a root, no production file is over 1,000 lines
  and no test file over 1,400 but `ratelimit/spec.rs` (`mockggg.rs` and
  `rails.rs` are not this step's); the per-file line counts are in
  commit 3's message.
- Each move commit's message quotes the checker's line, `ok … 0
  differences`, with its item and comment counts; the checker rerun on
  each commit's tree prints the same.
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
  file by a path from `crates/` that `bothy check` resolves
  (`acquisition-daemon/src/daemon/tests/idle.rs`, never
  `daemon/tests/idle.rs`: `tools/bothy.py:269-276`).
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
"""A pure-move check over a file split: every item of the old file is
in the new files as it was, and nothing else is.

An item is a declaration at the top of a file or inside an `impl`,
`mod` or `trait` block: a function with its body, a struct, an enum, a
constant, a type alias, a trait method's signature. Its text is read in
order, so a statement swapped with its neighbour, a changed literal or
a changed signature is a different item; string and character literals
are kept byte for byte, and whitespace outside them is collapsed so
rustfmt's reflow at a new nesting depth changes nothing. Comment lines
(`//`, `///`) are a multiset of their own, so a doc paragraph may be
reattached to the item it describes. Set aside, because a split must
change them: blank lines; module docs (`//!`); `use` and `mod`
declarations; `#[cfg(test)]` and `#[path]` attributes; the opening and
closing lines of an `impl`, `mod` or `trait` block (the wrappers a split
multiplies); a visibility token (`pub`, `pub(crate)`, `pub(super)`) at
the start of an item.

    python3 -I move-check.py <old> <new>...
    python3 -I move-check.py <(git show <sha>:crates/acquisition-daemon/src/daemon.rs) \
        crates/acquisition-daemon/src/daemon.rs crates/acquisition-daemon/src/daemon/*.rs

Prints what is missing from the new files and what they add that the
old file lacked, items and comments; exit 1 on either. Standard
library only. Braces inside strings and comments are masked before the
nesting depth is counted; the same reading is applied to both sides,
so a misread cancels unless the text differs.
"""
import re
import sys
from collections import Counter

SET_ASIDE = re.compile(
    r"^\s*(//!|(pub(\((crate|super)\))?\s+)?(use|mod)\s[^{]*;\s*$|#\[cfg\(test\)\]|#\[path\s*=)"
)
WRAPPER = re.compile(
    r"^\s*(pub(\((crate|super)\))?\s+)?(unsafe\s+)?(impl|mod|trait)\b"
)
VIS = re.compile(r"^(\s*)pub(\((crate|super|self|in [\w:]+)\))?\s+")
STRING = re.compile(
    r'(r#*"(?:[^"]|"(?!#))*"#*|"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])\')', re.S
)
WS = re.compile(r"\s+")
TRAILING_COMMA = re.compile(r",(?=[\)\]\}>])")


def mask(line):
    """The line with string and char literals replaced by `S` and a
    trailing `//` comment removed, for counting braces."""
    out = STRING.sub("S", line)
    i = out.find("//")
    return out[:i] if i >= 0 else out


def normalise(text):
    """Whitespace collapsed outside literals; a trailing comma before a
    closer dropped (rustfmt drops it when it joins lines)."""
    parts = STRING.split(text)
    for i in range(0, len(parts), 2):
        parts[i] = TRAILING_COMMA.sub("", WS.sub("", parts[i]))
    return "".join(parts)


def read(path):
    items, comments = Counter(), Counter()
    with open(path, encoding="utf-8") as fh:
        lines = [l.rstrip("\n") for l in fh]
    depth = 0
    pdepth = 0           # parens and brackets open
    wrappers = []        # depths at which a wrapper opened
    pending = None       # a wrapper header whose `{` has not come yet
    cur = []             # the item being gathered
    cur_depth = None     # its base depth

    def close_item():
        nonlocal cur, cur_depth
        if cur:
            text = "\n".join(cur)
            text = VIS.sub(r"\1", text, count=1)
            items[normalise(text)] += 1
        cur, cur_depth = [], None

    for raw in lines:
        s = raw.strip()
        if not s:
            continue
        if s.startswith("//"):
            if not s.startswith("//!"):
                comments[s] += 1
            continue
        masked = mask(raw)
        opens, closes = masked.count("{"), masked.count("}")
        before = depth
        depth = depth + opens - closes
        pdepth += masked.count("(") + masked.count("[") - masked.count(")") - masked.count("]")
        balanced = depth == (cur_depth if cur_depth is not None else before) and pdepth == 0
        if pending is not None:
            if depth > pending:
                wrappers.append(pending)
                pending = None
            continue
        if cur_depth is None:
            # a line that closes a wrapper
            if wrappers and depth == wrappers[-1] and closes and s.startswith("}"):
                wrappers.pop()
                continue
            if SET_ASIDE.match(raw):
                continue
            if WRAPPER.match(raw):
                if depth > before:
                    wrappers.append(before)
                elif not opens:
                    pending = before   # `impl X for Y` with its `{` on a later line
                continue               # `impl X for Y {}` opens and closes here
            cur, cur_depth = [raw], before
            if balanced and (s.endswith(";") or s.endswith("}")):
                close_item()
            continue
        cur.append(raw)
        if balanced and (s.endswith("}") or s.endswith(";")):
            close_item()
    close_item()
    return items, comments


old_items, old_comments = read(sys.argv[1])
new_items, new_comments = Counter(), Counter()
for p in sys.argv[2:]:
    i, c = read(p)
    new_items.update(i)
    new_comments.update(c)

bad = 0
for label, old, new in (("item", old_items, new_items), ("comment", old_comments, new_comments)):
    for k, n in sorted((old - new).items()):
        print(f"MISSING {label} x{n}: {k[:200]}")
        bad += n
    for k, n in sorted((new - old).items()):
        print(f"ADDED   {label} x{n}: {k[:200]}")
        bad += n
print(f"{'ok' if not bad else 'FAIL'}  {sum(old_items.values())} items and {sum(old_comments.values())} comment lines before, "
      f"{sum(new_items.values())} and {sum(new_comments.values())} after, {bad} differences")
sys.exit(1 if bad else 0)
```

Run it on each commit's tree before the gate; quote its last line in
the message. Checked by the design session on this tree: the limiter
cut into the five files of the design (every file formatted by rustfmt,
the spec dedented) and the daemon's one `impl Daemon` cut into three
files both report 0 differences; a changed string literal, the
`RateLimited` and `Network` returns of `classify_response` swapped, two
adjacent `let` statements swapped, a signature given a parameter and a
comment line deleted each report a difference and exit 1; a method
widened to `pub(super)`, a doc paragraph reattached to an item in
another file and an item placed in another file than its neighbour each
report 0. What it cannot see: a change inside a `use` or `mod` line,
or in a `//!` line; the compiler sees the first, the done-when's byte
counts the second.
