#!/usr/bin/env bash
# trade-check-breakers.sh — proves trade-check.sh refuses what its table
# checks claim to, by editing a committed table and watching the stage
# refuse it by name. Six of the check's stages are self-tests,
# each staging its own refusals on inputs it makes; four hold a committed
# file to what its script composes (`sheet`, `category`, `totals`,
# `stats`), and those stage nothing of their own, so their cases are here.
# The drift line's refusals are staged by `game-diff.py --self-test`.
#
# The clone's three tables (C198, the gate checks the generated tables)
# are checked only where the `poe1` clone is: with it, a case stages a
# table that has left its generator and the check refuses it; with none,
# that case is skipped with a line and the suite passes. Two cases run
# everywhere, each with `ACQ_CLONES` at a folder of the suite's own: none
# there, the checks skipped with a line and the exit unchanged; a clone
# there at no commit, refused by the pin. No case needs a copy of the
# export.
#
# A refusal is evidence only when it names the property, so each case
# states the words the check must print. An edit that does not apply
# fails the suite instead of passing as a refusal. Every case restores the
# files it touched from copies and compares them byte for byte — never
# `git checkout`, which discards uncommitted work — and a trap restores on
# any exit. Run it when a stage is added to trade-check.sh; CI runs it
# beside the gate, not in it.
set -uo pipefail
cd "$(dirname "$0")/.."

scratch=$(mktemp -d "${TMPDIR:-/tmp}/acq-trade-breakers.XXXXXX")
ref=crates/acquisition-search/reference
files=(search/pseudo-stats/data/search-sheet.csv "$ref/categories-v1.toml" "$ref/totals-v6.toml" "$ref/trade-stats-v1.toml" "$ref/classes-v1.toml")
# Copied and restored with their times (`cp -p`): the search crate
# compiles the tables in, and cargo reads them by their time, so a table
# put back newer than the build would rebuild that crate and every crate
# above it, and their docs, on the next gate.
for f in "${files[@]}"; do mkdir -p "$scratch/keep/$(dirname "$f")"; cp -p "$f" "$scratch/keep/$f"; done
restore() {
  local f
  for f in "${files[@]}"; do
    cp -p "$scratch/keep/$f" "$f"
    cmp -s "$scratch/keep/$f" "$f" || { echo "RESTORE FAILED: $f"; exit 2; }
  done
}
trap 'restore; rm -rf "$scratch"' EXIT

# ---- the harness -----------------------------------------------------------
pass=0; failed=0; skipped=0
# The refusal's first line, cut to 90 characters, never inside one: `cut -c`
# cuts bytes under the C locale, and half an em dash made the saved output
# binary to grep (tools hygiene's report).
clip() { python3 -Ic 'import sys; print(sys.stdin.readline().rstrip("\n")[:90])'; }
case_env=()   # set by a setup: the environment the check runs under, for that case alone
run_case() {  # run_case <name> <refuse|pass> <pattern the output must contain> <setup...>
  local name=$1 want=$2 pattern=$3; shift 3
  local out status
  case_env=()
  if ! "$@" 2>"$scratch/setup.err"; then
    printf 'FAIL    %-52s the breaker did not apply: %s\n' "$name" "$(head -c 200 "$scratch/setup.err")"; failed=$((failed+1)); restore; return
  fi
  out=$(env ${case_env[@]+"${case_env[@]}"} tools/trade-check.sh 2>&1); status=$?
  if [[ $want == refuse && $status -ne 0 && $out == *"$pattern"* ]] \
     || [[ $want == pass && $status -eq 0 && $out == *"$pattern"* ]]; then
    printf 'ok      %-52s %s: %s\n' "$name" "$want" "$(grep -m1 -F -- "$pattern" <<<"$out" | clip)"; pass=$((pass+1))
  else
    printf 'FAIL    %-52s wanted %s with "%s"; exit %d, got:\n%s\n' "$name" "$want" "$pattern" "$status" "$(grep -v '^ok ' <<<"$out" | sed 's/^/          /')"
    failed=$((failed+1))
  fi
  restore
}
none() { :; }
edit() {  # edit <file> <old> <new>: an exact-once substitution
  python3 - "$@" <<'PY'
import pathlib, sys
p = pathlib.Path(sys.argv[1]); s = p.read_text(encoding="utf-8")
assert s.count(sys.argv[2]) == 1, 'edit anchor not found once in ' + sys.argv[1] + ': ' + sys.argv[2][:60]
p.write_text(s.replace(sys.argv[2], sys.argv[3]), encoding="utf-8")
PY
}
append() { printf '%s\n' "$2" >>"$1"; }   # append <file> <line>
# Where the clone is and its pin, read from tools/sources.py, as the check reads them.
read -r poe1 pin < <(python3 -I -c 'import sys; sys.path.insert(0, "tools"); from sources import POE1, PINS; print(POE1, PINS["poe1"])')
no_clones() { mkdir -p "$scratch/none" && case_env=("ACQ_CLONES=$scratch/none"); }
clone_at_no_commit() { mkdir -p "$scratch/bare" && git init -q "$scratch/bare/poe1" && case_env=("ACQ_CLONES=$scratch/bare"); }

run_case 'clean tree'                                            pass   'ok      drift' none

echo '== a committed table is what its script composes'
# A sat search's line is never edited (the sheet is appended to): its
# words changed in place are refused by its name.
run_case 'the sheet: a committed line edited'                     refuse 'CHANGED  x18: the committed line is not what the script composes' \
  edit search/pseudo-stats/data/search-sheet.csv 'x18,"a level requirement' 'x18,"the level requirement'
run_case 'the category table: a line added'                       refuse 'categories-v1.toml differs from what the sources give' append "$ref/categories-v1.toml" '# an edit by hand'
run_case 'the totals table: a line added'                         refuse 'totals-v6.toml differs from what the sources give'     append "$ref/totals-v6.toml" '# an edit by hand'
run_case 'the stat table: a line added'                           refuse 'trade-stats-v1.toml differs from what the capture of' append "$ref/trade-stats-v1.toml" '# an edit by hand'

echo '== the tables the clone gives are what their generators give, where the clone is (C198)'
if [[ -e $poe1 ]]; then
  run_case 'the class table: a line added'                        refuse 'classes-v1.toml differs from what the clone at' append "$ref/classes-v1.toml" '# an edit by hand'
else
  printf 'skip    %-52s no poe1 clone at %s\n' 'the class table: a line added' "$poe1"; skipped=$((skipped+1))
fi
run_case 'no clone: the three skipped, the exit unchanged'        pass   "none/poe1: the class, defence and forms tables' checks are skipped" no_clones
run_case 'a clone at no commit: refused by its pin'               refuse "poe1 is at no commit, not the pinned $pin" clone_at_no_commit

echo
printf '%d ok, %d failed, %d skipped\n' "$pass" "$failed" "$skipped"
((failed == 0))
