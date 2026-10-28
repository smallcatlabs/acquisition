#!/usr/bin/env bash
# gate-breakers.sh — proves tools/gate.sh refuses what it claims to, by
# staging each refusal and watching it fail: a stage name it does not
# know, and the fixture switch set at any value (#87).
#
# A refusal is evidence only when it names the property, so each case
# states the words the gate must print; a case that passes states the
# words of the green verdict. A case that needs a stage to do something
# edits the gate's stage command by an exact-once substitution, as
# `arch-check-breakers.sh` edits its tool, and runs a selection that
# builds no Rust; an edit that does not apply, or leaves the gate
# unparseable, fails the suite instead of passing as a refusal. Every
# case restores the files it touched from copies and compares them byte
# for byte — never `git checkout`, which discards uncommitted work — and
# a trap restores on any exit. Run it when a stage or a refusal is added
# to gate.sh; CI runs it beside the gate, not in it.
set -uo pipefail
cd "$(dirname "$0")/.."
unset ACQ_UPDATE_FIXTURES   # the suite's own shell sets nothing a case did not

scratch=$(mktemp -d "${TMPDIR:-/tmp}/acq-gate-breakers.XXXXXX")
files=(tools/gate.sh)
for f in "${files[@]}"; do mkdir -p "$scratch/keep/$(dirname "$f")"; cp "$f" "$scratch/keep/$f"; done
restore() {
  local f
  for f in "${files[@]}"; do
    cp "$scratch/keep/$f" "$f"
    cmp -s "$scratch/keep/$f" "$f" || { echo "RESTORE FAILED: $f"; exit 2; }
  done
}
trap 'restore; rm -rf "$scratch"' EXIT

# ---- edits ---------------------------------------------------------------
stage_does() {  # stage_does <stage> <command>: the stage's command replaced, exact once
  python3 - "$1" "$2" <<'PY'
import pathlib, re, sys
p = pathlib.Path('tools/gate.sh'); s = p.read_text()
line = re.compile(r'(?m)^    ' + re.escape(sys.argv[1]) + r'\) .* ;;$')
assert len(line.findall(s)) == 1, 'no one stage line for ' + sys.argv[1]
p.write_text(line.sub(lambda m: '    ' + sys.argv[1] + ') ' + sys.argv[2] + ' ;;', s))
PY
}

# ---- the harness -----------------------------------------------------------
pass=0; failed=0
run_case() {  # run_case <name> <refuse|pass> <pattern the output must contain> <setup> <gate command...>
  local name=$1 want=$2 pattern=$3 setup=$4; shift 4
  local out status
  if ! $setup 2>"$scratch/setup.err"; then
    printf 'FAIL    %-52s the breaker did not apply: %s\n' "$name" "$(head -c 200 "$scratch/setup.err")"; failed=$((failed+1)); restore; return
  fi
  if ! bash -n tools/gate.sh 2>/dev/null; then
    printf 'FAIL    %-52s the breaker left the gate unparseable\n' "$name"; failed=$((failed+1)); restore; return
  fi
  out=$("$@" 2>&1); status=$?
  if [[ $want == refuse && $status -ne 0 && $out == *"$pattern"* ]] \
     || [[ $want == pass && $status -eq 0 && $out == *"$pattern"* ]]; then
    printf 'ok      %-52s %s: %s\n' "$name" "$want" "$(grep -m1 -F -- "$pattern" <<<"$out" | cut -c1-90)"; pass=$((pass+1))
  else
    printf 'FAIL    %-52s wanted %s with "%s"; exit %d, got:\n%s\n' "$name" "$want" "$pattern" "$status" "$(sed 's/^/          /' <<<"$out")"
    failed=$((failed+1))
  fi
  restore
}
none() { :; }
gate=tools/gate.sh

echo '== a stage the gate does not know'
run_case 'an unknown stage under --only'                         refuse '--only takes one of: fixtures' none $gate --only bogus
run_case 'an unknown flag'                                       refuse 'tools/gate.sh --only <stage>'  none $gate --bogus

echo '== the fixture switch: set at any value, the gate refuses (#87)'
FX='ACQ_UPDATE_FIXTURES is set in this shell'
run_case 'the switch at 0'                                       refuse "$FX (to '0')" none env ACQ_UPDATE_FIXTURES=0 $gate --only fixtures
run_case 'the switch empty'                                      refuse "$FX (to '')"  none env ACQ_UPDATE_FIXTURES= $gate --only fixtures
run_case 'the switch at 1'                                       refuse "$FX (to '1')" none env ACQ_UPDATE_FIXTURES=1 $gate --only fixtures
run_case 'the switch unset'                                      pass   'gate: green'  none $gate --only fixtures
# The stage that would rewrite a golden is any stage after the switch is
# read: the gate unsets it for all of them, under any selection.
sees_no_switch() { stage_does diff '[[ -z ${ACQ_UPDATE_FIXTURES+x} ]] || { echo "the stage saw the switch"; false; }'; }
run_case 'a stage run with the switch set sees it unset'         pass   'gate: green'  sees_no_switch env ACQ_UPDATE_FIXTURES=1 $gate --only diff
run_case 'the same stage under --docs'                           pass   'gate: green'  sees_no_switch env ACQ_UPDATE_FIXTURES=0 $gate --docs

echo
printf '%d ok, %d failed\n' "$pass" "$failed"
((failed == 0))
