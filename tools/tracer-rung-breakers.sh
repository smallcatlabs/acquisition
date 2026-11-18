#!/bin/bash
# tracer-rung-breakers.sh — the tracer driver's promises about what a run
# leaves behind, each staged against the mock and required: a run writes
# no policy, so the stored one stands at the revision the run found; a
# selection asked of the driver is refused before anything is made; and
# a failed apply's own words are in the bundle, with the policy still as
# found. A fault is one line changed in a copy of the driver (`drift`),
# never a knob in the driver.
#
#   tools/tracer-rung-breakers.sh
#
# No gate stage runs this: a case starts mock daemons, some seconds
# each. Run it by hand after touching tools/tracer-rung.sh, with no
# daemon up and nothing else building (it builds through the driver's
# preflight). Bundles go to runs/mock/ as a rehearsal's do.
set -uo pipefail
here=$(cd "$(dirname "$0")/.." && pwd)
ACQ="$here/target/debug/acq"
# The copy runs from tools/, where the driver finds the workspace.
copy="$here/tools/.tracer-rung-case.sh"
trap 'rm -f "$copy"' EXIT
for v in ACQ_STORE_DIR ACQ_LOG_DIR ACQ_NO_KEYRING ACQ_NO_SPAWN ACQ_JOURNAL ACQ_PROVIDER; do unset "$v"; done

fails=0
ok() { echo "ok   $1"; }
bad() { echo "FAIL $1"; echo "$OUT" | grep -v '^    ' | tail -n 12 | sed 's/^/     /'; fails=$((fails + 1)); }

# drift <old> <new>: the driver with one text replaced, which must occur
# exactly once; no arguments is the driver as it is.
drift() {
    python3 -I - "$here/tools/tracer-rung.sh" "$copy" "$@" <<'PY' || { echo "FAIL the drift found no single place for: ${1:-}"; exit 1; }
import sys
src, dst, *pair = sys.argv[1:]
text = open(src).read()
if pair:
    old, new = pair
    if text.count(old) != 1:
        sys.exit(1)
    text = text.replace(old, new)
open(dst, "w").write(text)
PY
    chmod +x "$copy"
}

# run <driver args...>: sets OUT (both streams), RC and B (the bundle, or
# empty where the driver named none).
run() {
    OUT=$("$copy" --mock "$@" </dev/null 2>&1)
    RC=$?
    B=$(printf '%s\n' "$OUT" | sed -n 's/^bundle: \([^ ]*\).*/\1/p' | head -1)
}
world() { ACQ_PROVIDER=mock ACQ_NO_SPAWN=1 ACQ_STORE_DIR="$B/store" ACQ_LOG_DIR="$B/log" "$@"; }
# The stored policy, revision and value, and the one the rehearsal's own
# setup stored before the run began.
stored() { world "$ACQ" --account tracer-rung policy show --json | jq -Sc '.policy | {revision, value}'; }
found() { jq -Sc '{revision, value}' "$B/policy-mock.json"; }
passed() { printf '%s\n' "$OUT" | grep -q '^pass'; }

name="a run writes no policy: the stored one stands at the revision the run found"
drift
run
if [ "$RC" = 0 ] && passed && [ -d "$B" ] && [ "$(stored)" = "$(found)" ]; then ok "$name"; else bad "$name"; fi

name="a selection asked of the driver is refused before anything is made"
for flag in --tabs --characters --max-age; do
    run "$flag" cur1
    if [ "$RC" = 2 ] && [ -z "$B" ] && [[ $OUT == *"acq policy set"* ]]; then ok "$name ($flag)"; else bad "$name ($flag)"; fi
done

name="a failed apply's own words are in the bundle, and the policy is as found"
drift '--max-requests "$logical" 2>' '--max-requests 0 2>'
run
if [ "$RC" != 0 ] && ! passed && [ -s "$B/apply-c1.err" ] && [ "$(stored)" = "$(found)" ]; then ok "$name"; else bad "$name"; fi

if [ "$fails" = 0 ]; then echo "tracer-rung breakers: all held"; else echo "tracer-rung breakers: $fails FAILED"; exit 1; fi
