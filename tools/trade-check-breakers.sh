#!/usr/bin/env bash
# trade-check-breakers.sh — proves trade-check.sh refuses what its table
# checks claim to, by editing a committed table and watching the stage
# refuse it by name (#106). Six of the check's stages are self-tests,
# each staging its own refusals on inputs it makes; four hold a committed
# file to what its script composes (`sheet`, `category`, `totals`,
# `stats`), and those stage nothing of their own, so their cases are here.
# The drift line's refusals are staged by `game-diff.py --self-test`.
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
files=(search/pseudo-stats/data/search-sheet.csv "$ref/categories-v1.toml" "$ref/totals-v6.toml" "$ref/trade-stats-v1.toml")
for f in "${files[@]}"; do mkdir -p "$scratch/keep/$(dirname "$f")"; cp "$f" "$scratch/keep/$f"; done
restore() {
  local f
  for f in "${files[@]}"; do
    cp "$scratch/keep/$f" "$f"
    cmp -s "$scratch/keep/$f" "$f" || { echo "RESTORE FAILED: $f"; exit 2; }
  done
}
trap 'restore; rm -rf "$scratch"' EXIT

# ---- the harness -----------------------------------------------------------
pass=0; failed=0
run_case() {  # run_case <name> <refuse|pass> <pattern the output must contain> <setup...>
  local name=$1 want=$2 pattern=$3; shift 3
  local out status
  if ! "$@" 2>"$scratch/setup.err"; then
    printf 'FAIL    %-52s the breaker did not apply: %s\n' "$name" "$(head -c 200 "$scratch/setup.err")"; failed=$((failed+1)); restore; return
  fi
  out=$(tools/trade-check.sh 2>&1); status=$?
  if [[ $want == refuse && $status -ne 0 && $out == *"$pattern"* ]] \
     || [[ $want == pass && $status -eq 0 && $out == *"$pattern"* ]]; then
    printf 'ok      %-52s %s: %s\n' "$name" "$want" "$(grep -m1 -F -- "$pattern" <<<"$out" | cut -c1-90)"; pass=$((pass+1))
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

run_case 'clean tree'                                            pass   'ok      drift' none

echo '== a committed table is what its script composes'
# A sat search's line is never edited (the sheet is appended to): its
# words changed in place are refused by its name.
run_case 'the sheet: a committed line edited'                     refuse 'CHANGED  x18: the committed line is not what the script composes' \
  edit search/pseudo-stats/data/search-sheet.csv 'x18,"a level requirement' 'x18,"the level requirement'
run_case 'the category table: a line added'                       refuse 'categories-v1.toml differs from what the sources give' append "$ref/categories-v1.toml" '# an edit by hand'
run_case 'the totals table: a line added'                         refuse 'totals-v6.toml differs from what the sources give'     append "$ref/totals-v6.toml" '# an edit by hand'
run_case 'the stat table: a line added'                           refuse 'trade-stats-v1.toml differs from what the capture of' append "$ref/trade-stats-v1.toml" '# an edit by hand'

echo
printf '%d ok, %d failed\n' "$pass" "$failed"
((failed == 0))
