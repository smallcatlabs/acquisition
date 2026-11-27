#!/usr/bin/env bash
# bothy-breakers.sh — proves `tools/bothy.py check` refuses each fault it
# claims to: a small registry is written to a temporary tree, passes clean,
# and then each break in turn must be refused with the fault's kind named.
# Run it when a check is added to bothy.py.
#
# Decision ids are written `@n` here and become `Cn` when the fixture is
# written or an edit is applied, so a tree's citation grep over `tools/`
# (acquisition's docs-check.sh) never reads this file's ids as citations.
set -euo pipefail
ids() { sed 's/@\([0-9]\)/C\1/g'; }
here=$(cd "$(dirname "$0")" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
fixture() {  # a clean registry: AGENTS.md, one area, one module doc
  rm -rf "$tmp"/*; mkdir -p "$tmp/decisions" "$tmp/crates/acquisition-store/src" "$tmp/tools"
  ids >"$tmp/AGENTS.md" <<'CTX'
# Test — agent entry point

## What Acquisition is

A test registry.

## Invariants

1. **Nothing leaves
   the tree.** *Pinned:*
   nothing yet.

## Cross-cutting rulings

- **@1 — Every rule has one home.** *Why:* two homes rot (P1). *Pinned:* nothing yet.

## Principles

- **Structure over discipline.** A rule is a check (P1, P2).
CTX
  ids >"$tmp/decisions/store.md" <<'STO'
# Decisions — Store

The rulings that bind the store (`acquisition-store/src/lib.rs`). Read this before touching that code.

## Store

- **@2 — Facts are kept as received.** *Why:* a derivation can be redone (@1). *Pinned:* `store::record` in `acquisition-store/src/lib.rs`.
- **@3 — The store sends nothing.** *Why:* a read observes. *Pinned:* `tests/read.rs`.

## Not doing

- Pruning: not until the owner asks.
- A second store: refused; one store per world.
STO
  ids >"$tmp/crates/acquisition-store/src/lib.rs" <<'LIB'
//! The store. Rulings: `decisions/store.md`, @2, @3.
pub fn record() {}
LIB
  mkdir -p "$tmp/crates/acquisition-store/tests"; echo '// @3' | ids >"$tmp/crates/acquisition-store/tests/read.rs"
}
run() { python3 -I "$here/bothy.py" --root "$tmp" check --no-notes 2>&1 || true; }
fixture
out=$(run)
if [[ $out != ok* ]]; then echo "the clean fixture must pass:"; echo "$out"; exit 1; fi
echo "ok      clean fixture passes"
n=0; bad=0
expect() {  # expect <kind> <python edit over the fixture> [words the refusal must carry]
  local kind=$1 edit=${2//@/C} says=${3:-} out
  says=${says//@/C}
  fixture
  python3 -I - "$tmp" <<PY
import pathlib, sys
root = pathlib.Path(sys.argv[1])
$edit
PY
  out=$(run)
  n=$((n+1))
  if [[ $out == *"FAIL  $kind"* && $out == *"$says"* ]]; then
    printf 'ok      %-10s refused%s\n' "$kind" "${says:+: $says}"
  else
    printf 'MISSED  %-10s not refused:\n%s\n' "$kind" "$out"; bad=1
  fi
}
expect duplicate 'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("@3 —", "@2 —"))'
expect grammar   'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("- **@3 — The store sends nothing.**", "- **@3 The store sends nothing**"))'
expect cite      'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("(@1)", "(@7)"))'
expect cite      'p=root/"AGENTS.md"; p.write_text(p.read_text().replace("(P1)", "(P4)"))'
expect cite      'p=root/"AGENTS.md"; p.write_text(p.read_text().replace("(P1, P2)", "(P2)"))'
expect principle 'p=root/"AGENTS.md"; p.write_text(p.read_text().replace(" (P1, P2).", "."))'
expect missing   '(root/"AGENTS.md").unlink()'
expect cite      'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("not until the owner asks.", "not until the owner asks (@8)."))'
expect entry     'p=root/"decisions/store.md"; p.write_text(p.read_text().replace(" *Why:* a read observes.", ""))'
expect entry     'p=root/"decisions/store.md"; p.write_text(p.read_text().replace(" *Pinned:* `tests/read.rs`.", ""))'
expect entry     'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("*Pinned:* `tests/read.rs`.", "*Details:* the store doc. *Pinned:* `tests/read.rs`."))'
expect entry     'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("*Why:* a read observes.", "*Why:*"))'
expect entry     'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("*Pinned:* `tests/read.rs`.", "*Pinned:* ."))'
expect entry     'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("*Why:* a read observes.", "*Why:*\t"))'
expect entry     'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("*Pinned:* `tests/read.rs`.", "*Pinned:* \u00a0."))'
expect entry     'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("*Why:* a read observes.", "*Why:* a read observes. *Why:* and sends nothing."))'
expect entry     'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("*Pinned:* `tests/read.rs`.", "*Pinned:* `tests/read.rs`. *Pinned:* nothing yet."))'
expect notdoing  'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("- Pruning: not until the owner asks.", "- Pruning, a verb, some day."))'
expect notdoing  'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("- Pruning: not until the owner asks.", "- Pruning: later, when the owner asks."))'
expect notdoing  'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("- Pruning: not until the owner asks.", "- Pruning: not until."))'
expect notdoing  'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("- Pruning: not until the owner asks.", "- ~~Pruning: not until the owner asks.~~"))'
# the retired rule: @3 retires to one line under "Retired", held by the
# test file that cites it. RETIRE takes the held-by text.
RETIRE='p=root/"decisions/store.md"; s=p.read_text(); entry=next(l for l in s.splitlines() if l.startswith("- **@3 ")); p.write_text(s.replace(entry + "\n", "") + "\n## Retired\n\n- **@3 — The store sends nothing.** *Held by:* " + HELD + ".\n")'
held() { printf 'HELD=%s; %s' "$1" "$RETIRE"; }
expect retired   "$(held '"`crates/acquisition-store/tests/gone.rs`"')" 'is no file under the tree'
expect retired   "$(held '"`tools/x.sh`"'); (root/'tools/x.sh').write_text('# no id here\\n')" 'does not cite @3'
expect retired   "$(held '"`decisions/store.md`"')" 'which is the registry'
expect retired   "$(held '"the store test"')" 'ticked paths alone'
expect retired   "$(held '"`crates/acquisition-store/tests/read.rs`, tools/gone.sh. *Why:* a read observes"')" 'ticked paths alone'
expect retired   "$(held '"`crates/acquisition-store/tests/read.rs`"'); p.write_text(p.read_text().replace('## Store\\n\\n', '## Store\\n\\n' + entry + '\\n'))" 'never both'
expect retired   "$(held '"`crates/acquisition-store/tests/read.rs`"'); p.write_text(p.read_text().replace('.** *Held by:*', '.** It sends nothing. *Why:* a read observes. *Held by:*'))" 'the headline and its holders alone'
expect duplicate "$(held '"`crates/acquisition-store/tests/read.rs`"'); p.write_text(p.read_text() + '- **@3 — The store sends nothing.** *Held by:* \`crates/acquisition-store/tests/read.rs\`.\\n')" 'an id retires once'
expect cite      "$(held '"`crates/acquisition-store/tests/read.rs`"'); p.write_text(p.read_text().replace('sends nothing.** *Held', 'sends nothing (@9).** *Held'))" 'a retired line names'
expect path      'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("`tests/read.rs`", "`tests/gone.rs`"))'
expect path      'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("`acquisition-store/src/lib.rs`", "`acquisition-store/src/old.rs`"))'
expect routing   'p=root/"crates/acquisition-store/src/lib.rs"; p.write_text(p.read_text().replace("decisions/store.md", "decisions/stores.md"))'
# the entry's shape (the entry rule, 6 to 8): a headline of at most 160
# bytes, at most six clauses, and a baseline row that lets an entry stay as
# far over as it was and no further. LONG makes a 175-byte headline.
LONG='p=root/"decisions/store.md"; p.write_text(p.read_text().replace("The store sends nothing.", "The store sends nothing, " + "and reads " * 13 + "and reads all of it."))'
BASE='(root/"decisions/shape-baseline.tsv").write_text'
expect headline  "$LONG" 'is 175 bytes, over 160'
expect clauses   'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("nothing.** *Why:* a read", "nothing.** One. Two; three (four. five). Four `a. b`. Five. Six. *Why:* a read"))' 'has 7 clauses'
expect baseline  "$LONG; $BASE('# id\\n@3\\t165\\t1\\n')" 'over the 165 its row'
expect baseline  "$BASE('@3\\t170\\t1\\n')" 'stale; remove the row'
expect baseline  "$BASE('@9\\t170\\t1\\n')" 'no entry'
expect baseline  "$LONG; $BASE('@3 170 1\\n')" 'tab-separated'
# a *Pinned:* names at most three files, and every name it ticks exists
expect pinned    'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("*Pinned:* `tests/read.rs`.", "*Pinned:* `tests/read.rs`, `acquisition-store/src/lib.rs`, `crates/acquisition-store/tests/read.rs`, `crates/acquisition-store/src/lib.rs`."))' 'names 4 files'
expect pinned    'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("`store::record`", "`store::recorded`"))' 'no file it names holds'
expect pinned    'p=root/"AGENTS.md"; p.write_text(p.read_text().replace("(P1). *Pinned:* nothing yet.", "(P1). *Pinned:* the `c1_` tests."))' 'nothing under crates/, tools/ or .github/ holds'
expect pinned    'p=root/"decisions/store.md"; p.write_text(p.read_text().replace("*Pinned:* `tests/read.rs`.", "*Pinned:* the tests of `gone.rs`."))' 'is no file'
expect invariant 'p=root/"AGENTS.md"; p.write_text(p.read_text().replace(" *Pinned:*\n   nothing yet.", ""))' 'has no *Pinned:*'
expect pinned    'p=root/"AGENTS.md"; p.write_text(p.read_text().replace("   nothing yet.", "   `tests/read.rs`, `no_such_test`."))' 'invariant 1'
# what must pass: an entry as far over as its baseline row, and a bare
# file name, a directory and a prefix in a *Pinned:*
fixture
python3 -I - "$tmp" <<'PY'
import pathlib, sys
root = pathlib.Path(sys.argv[1])
p = root / "decisions/store.md"
p.write_text(p.read_text().replace("The store sends nothing.", "The store sends nothing, " + "and reads " * 13 + "and reads all of it.")
             .replace("*Pinned:* `tests/read.rs`.", "*Pinned:* the `rec` function of `lib.rs`, under `acquisition-store/src/`."))
(root / "decisions/shape-baseline.tsv").write_text("# id\theadline bytes\tclauses\nC3\t175\t1\n")
PY
out=$(run); n=$((n+1))
if [[ $out == ok* ]]; then echo "ok      an entry at its baseline row passes, and a bare name, a directory and a prefix are names"; else echo "WRONG   a kept shape or a true name was refused:"; echo "$out"; bad=1; fi
# what must pass: history, an API route, a key shape
fixture
python3 -I - "$tmp" <<'PY'
import pathlib, sys
p = pathlib.Path(sys.argv[1]) / "decisions/store.md"
p.write_text(p.read_text().replace("Read this before", "History: `spikes/rust-playground/search/BUILD-PLAN.md` at `453a4429`; the route `/profile`; a `wanted/lot` ratio. Read this before"))
PY
out=$(run); n=$((n+1))
if [[ $out == ok* ]]; then echo "ok      history, a route and a key shape are not paths"; else echo "WRONG   a non-path was refused:"; echo "$out"; bad=1; fi
# what must pass: the highest id retired, held by the test that cites it,
# and cited by an entry and a module doc; `list` marks it and counts it,
# so the next id is still one past it, `show` prints its line and where
# its holder cites it, and `pack` prints it as no headline.
fixture
ids <<PY | python3 -I - "$tmp"
import pathlib, sys
root = pathlib.Path(sys.argv[1])
HELD = "\`crates/acquisition-store/tests/read.rs\`"
$RETIRE
p.write_text(p.read_text().replace("(@1)", "(@1, @3)"))
PY
out=$(run); n=$((n+1))
if [[ $out == ok*'2 decisions and 1 retired'* ]]; then echo "ok      retired   a retired line held by a file that cites it passes, and an entry may cite it"; else echo "WRONG   a retired line was refused:"; echo "$out"; bad=1; fi
bothy() { python3 -I "$here/bothy.py" --root "$tmp" "$@" 2>&1; }
n=$((n+1))
if out=$(bothy list) && [[ $out == *$'\nC3    store          retired: The store sends nothing.'*'2 decisions, 1 retired, 2 not-doing lines' ]]; then echo "ok      list      a retired id is listed, marked, and counted"; else echo "MISSED  list      a retired id is not listed and counted:"; echo "$out"; bad=1; fi
n=$((n+1))
if out=$(bothy show "$(ids <<<@3)") && [[ $out == *'- **C3 — The store sends nothing.** *Held by:*'*'tests/read.rs: line 1'*'Named by: C2'* ]]; then echo "ok      show      a retired id shows its line, its holder's citing line and what names it"; else echo "MISSED  show      a retired id does not show:"; echo "$out"; bad=1; fi
n=$((n+1))
if out=$(bothy show "$(ids <<<@2)") && [[ $out == *"$(ids <<<'Names: @1 (Every rule has one home.), @3 (retired: The store sends nothing.)')"* ]]; then echo "ok      show      an entry that names a retired id labels it"; else echo "MISSED  show      a retired id an entry names is not labelled:"; echo "$out"; bad=1; fi
n=$((n+1))
if out=$(bothy pack --path crates/acquisition-store/src/lib.rs) && [[ $out == *'- **C2 — Facts'* && $out != *'C3 —'* ]]; then echo "ok      pack      a retired line is no headline"; else echo "MISSED  pack      a retired line is printed:"; echo "$out"; bad=1; fi
# pack's routing by an intro: a ticked path routes the file whose path
# ends with it, whole components, and no file that merely shares its last
# name (else `pack --path README.md` would print the search rulings, whose
# intro names `search/README.md`).
fixture
python3 -I - "$tmp" <<'PY'
import pathlib, sys
root = pathlib.Path(sys.argv[1])
p = root / "decisions/store.md"
p.write_text(p.read_text().replace("Read this before", "Its notes: `" + "notes/README.md`. Read this before"))   # split: docs-check reads this file
for f in ("README.md", "notes/README.md"):
    (root / f).parent.mkdir(parents=True, exist_ok=True); (root / f).write_text("# a readme\n")
PY
pack() { python3 -I "$here/bothy.py" --root "$tmp" pack --path "$1" 2>&1; }
n=$((n+1))
if out=$(pack notes/README.md) && [[ $out == *'rulings of `store`'* ]]; then echo "ok      routing   the path an intro names is routed"; else echo "MISSED  routing   notes/README.md not routed to store:"; echo "$out"; bad=1; fi
n=$((n+1))
if out=$(pack README.md) && [[ $out == *'No area governs `README.md`'* && $out != *'rulings of `store`'* ]]; then echo "ok      routing   a file sharing only its last name is not"; else echo "MISSED  routing   README.md routed by a basename:"; echo "$out"; bad=1; fi
# pack --path leads with what the file cites and repeats nothing of
# AGENTS.md; a path that does not exist is refused.
fixture
n=$((n+1))
if out=$(pack crates/acquisition-store/src/lib.rs) && [[ $out == *'cites'*'- **C3 — The store sends nothing.**'* && $out != *'A test registry.'* && $out != *'Every rule has one home'* ]]; then echo "ok      pack      the cited entries lead, and AGENTS.md is not repeated"; else echo "MISSED  pack      the cited entries do not lead, or AGENTS.md is repeated:"; echo "$out"; bad=1; fi
n=$((n+1))
if ! out=$(pack crates/acquisition-store/src/gone.rs) && [[ $out == *'no file crates/acquisition-store/src/gone.rs'* ]]; then echo "ok      pack      a path that does not exist is refused"; else echo "MISSED  pack      a path that does not exist was routed:"; echo "$out"; bad=1; fi
# A root file the workspace reads is routed to the areas its members name,
# and a root file nothing routes is refused.
fixture
printf '[workspace]\nmembers = ["crates/acquisition-store"]\n' >"$tmp/Cargo.toml"
printf '[package]\nname = "acquisition-store"\n' >"$tmp/crates/acquisition-store/Cargo.toml"
echo "# a readme" >"$tmp/README.md"
n=$((n+1))
if out=$(pack Cargo.toml) && [[ $out == *'reaches every member the manifest lists'*'rulings of `store`'* && $out != *'Every rule has one home'* ]]; then echo "ok      pack      the workspace manifest is routed to its members' areas"; else echo "MISSED  pack      Cargo.toml not routed to its members' areas:"; echo "$out"; bad=1; fi
n=$((n+1))
if out=$(pack README.md) && [[ $out == *'No area governs `README.md`'* && $out != *'reaches every member'* ]]; then echo "ok      pack      a root file the workspace does not read is not routed to its members"; else echo "MISSED  pack      README.md routed as a workspace file:"; echo "$out"; bad=1; fi
# Several files print each entry once: whole where any of them cites it,
# else one headline, and each area's "Not doing" once. other.rs cites @2
# alone and read.rs @3 alone, so each alone prints the other's headline.
fixture
echo '//! More of the store (`decisions/store.md`), @2.' | ids >"$tmp/crates/acquisition-store/src/other.rs"
packs() { python3 -I "$here/bothy.py" --root "$tmp" pack --path "$@" 2>&1; }
both=$(packs crates/acquisition-store/src/other.rs crates/acquisition-store/tests/read.rs) || both=""
once=1
for f in crates/acquisition-store/src/other.rs crates/acquisition-store/tests/read.rs; do
  for e in $(packs "$f" | grep -oE '^- \*\*C[0-9]+ ' | grep -oE 'C[0-9]+'); do
    [[ $both == *"- **$e —"* ]] || once=0
  done
done
for e in C2 C3; do [[ $(grep -cE "^- (\*\*)?$e —" <<<"$both") == 1 ]] || once=0; done
[[ $(grep -c '^- Pruning' <<<"$both") == 1 ]] || once=0
n=$((n+1))
if [[ $once == 1 ]]; then echo "ok      pack      several files print each entry once, whole where one cites it, and the area's Not doing once"; else echo "MISSED  pack      several files repeat an entry or lose one whole:"; echo "$both"; bad=1; fi
n=$((n+1))
if ! out=$(packs crates/acquisition-store/src/other.rs crates/acquisition-store/src/gone.rs) && [[ $out == *'no file crates/acquisition-store/src/gone.rs'* ]]; then echo "ok      pack      one missing file among several is refused, named"; else echo "MISSED  pack      a missing file among several was not refused:"; echo "$out"; bad=1; fi
# A file under tools/ that no area governs is answered, not refused: with
# the areas of what it cites, or saying that it cites nothing.
echo '# checks @3' | ids >"$tmp/tools/x.sh"; echo '# checks nothing' >"$tmp/tools/y.sh"
n=$((n+1))
if out=$(pack tools/x.sh) && [[ $out == *'- **C3 —'*'No area governs `tools/x.sh`'*'falls in `store`'* ]]; then echo "ok      pack      an ungoverned tool is answered with the areas of what it cites"; else echo "MISSED  pack      an ungoverned tool citing a ruling was refused or named no area:"; echo "$out"; bad=1; fi
n=$((n+1))
if out=$(pack tools/y.sh) && [[ $out == *'No area governs `tools/y.sh`'*'cites no ruling by id'* ]]; then echo "ok      pack      an ungoverned tool citing nothing is answered in words"; else echo "MISSED  pack      an ungoverned tool citing nothing was refused:"; echo "$out"; bad=1; fi
# A file that cites only a retired id holds it: the pack says so, with the
# id's area, and never that the file cites nothing.
ids <<PY | python3 -I - "$tmp"
import pathlib, sys
root = pathlib.Path(sys.argv[1])
HELD = "\`tools/z.sh\`"
(root/"tools/z.sh").write_text("# holds @3: the store sends nothing\n")
$RETIRE
PY
n=$((n+1))
if out=$(pack tools/z.sh) && [[ $out == *'C3 (store), held by `tools/z.sh`'*'falls in `store`'* && $out != *'cites no ruling'* && $out != *'C3 —'* ]]; then echo "ok      pack      a file holding only a retired id is told so, with the id's area"; else echo "MISSED  pack      a retired id a file holds is reported as nothing cited:"; echo "$out"; bad=1; fi
fixture
echo '//! More of the store (`decisions/store.md`), @2.' | ids >"$tmp/crates/acquisition-store/src/other.rs"
echo '# checks nothing' >"$tmp/tools/y.sh"
# Each pack's last line says its size in bytes, that line included.
for args in "--path crates/acquisition-store/src/other.rs crates/acquisition-store/tests/read.rs" "--path tools/y.sh" "--area store"; do
  out=$(python3 -I "$here/bothy.py" --root "$tmp" pack $args 2>&1; echo x); out=${out%x}
  size=$(printf '%s' "$out" | wc -c | tr -d ' ')
  n=$((n+1))
  if [[ $(printf '%s' "$out" | tail -n 1) == "$size bytes, this line included." ]]; then echo "ok      pack      pack $args ends with its size, $size bytes"; else echo "MISSED  pack      pack $args does not end with its size ($size bytes):"; printf '%s' "$out" | tail -n 2; bad=1; fi
done
# ...counted as written, whatever stdout's text layer would do: under a
# stdout that writes `\r\n` in cp1252, as a Windows console does, the
# count still matches the bytes.
out=$(python3 -I -c 'import io, runpy, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="cp1252", errors="replace", newline="\r\n")
sys.argv = [sys.argv[1], "--root", sys.argv[2], "pack", "--area", "store"]
runpy.run_path(sys.argv[0], run_name="__main__")' "$here/bothy.py" "$tmp" 2>&1; echo x); out=${out%x}
size=$(printf '%s' "$out" | wc -c | tr -d ' ')
n=$((n+1))
if [[ $(printf '%s' "$out" | tail -n 1) == "$size bytes, this line included." && $out != *$'\r'* ]]; then echo "ok      pack      a stdout that translates newlines changes nothing: $size bytes, as written"; else echo "MISSED  pack      the size line counts what the text layer did not write ($size bytes):"; printf '%s' "$out" | tail -n 2 | od -c | tail -3; bad=1; fi
echo "$n cases"
exit $bad
