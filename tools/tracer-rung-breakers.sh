#!/bin/bash
# tracer-rung-breakers.sh — the tracer driver's promises about what a run
# leaves behind, each staged against the mock and required: a run with a
# selection of its own leaves the stored policy as it found it (a
# version 3 one, and one of an earlier version, put back as stored); a
# run cut between its policy write and the receipt still puts it back;
# another client's write is left alone and the run does not pass; a
# put-back the store refuses fails the run and prints a command that
# puts the policy back when run as printed; and a failed apply's own
# words are in the bundle. Each fault is one line changed in a copy of
# the driver (`drift`), never a knob in the driver.
#
#   tools/tracer-rung-breakers.sh
#
# No gate stage runs this: every case starts mock daemons, about ten
# seconds each. Run it by hand after touching tools/tracer-rung.sh, with
# no daemon up and nothing else building (it builds through the driver's
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

# run <driver args...>: sets OUT (both streams), RC and B (the bundle).
run() {
    OUT=$("$copy" --mock "$@" </dev/null 2>&1)
    RC=$?
    B=$(printf '%s\n' "$OUT" | sed -n 's/^bundle: \([^ ]*\).*/\1/p' | head -1)
    [ -d "$B" ] || { echo "FAIL the driver named no bundle:"; echo "$OUT" | tail -n 5; exit 1; }
}
world() { ACQ_PROVIDER=mock ACQ_NO_SPAWN=1 ACQ_STORE_DIR="$B/store" ACQ_LOG_DIR="$B/log" "$@"; }
stored() { world "$ACQ" --account tracer-rung policy show --json | jq -Sc '.policy.value'; }
found() { jq -Sc '.policy.value' "$B/policy-before.json"; }
passed() { printf '%s\n' "$OUT" | grep -q '^pass'; }

V2='{"version":2,"realms":{"pc":{"leagues":{"Standard":{"tabs":"all","max_age_seconds":3600}}}}}'
FOREIGN='{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":["dump"],"max_age_seconds":77}}}}}'
WRITE='        stop "the run'"'"'s policy was refused (policy-run.json)"'
PUT_BACK='restore_policy || true'

name="a run with its own selection leaves the stored policy as it found it"
drift
run --tabs cur1
if [ "$RC" = 0 ] && passed && [ "$(stored)" = "$(found)" ]; then ok "$name"; else bad "$name"; fi

name="a stored policy of an earlier version is put back as stored"
drift 'acq policy set "$POLICY" --json >"$RUN_DIR/policy-mock.json"' \
    "acq policy set '$V2' --replace --json >\"\$RUN_DIR/policy-mock.json\""
run --tabs cur1
if [ "$RC" = 0 ] && passed && [ "$(stored)" = "$(jq -Sc . <<<"$V2")" ]; then ok "$name"; else bad "$name"; fi

name="a run cut between its policy write and the receipt still puts the policy back"
drift "$WRITE" "$WRITE"$'\n''    : >"$RUN_DIR/policy-run.json"; exit 130'
run --tabs cur1
if [ "$RC" != 0 ] && ! passed && [ "$(stored)" = "$(found)" ]; then ok "$name"; else bad "$name"; fi

name="another client's write is left alone, and the run does not pass"
drift "$PUT_BACK" "acq policy set '$FOREIGN' --json >/dev/null"$'\n'"$PUT_BACK"
run --tabs cur1
if [ "$RC" != 0 ] && ! passed && [ "$(stored)" = "$(jq -Sc . <<<"$FOREIGN")" ] &&
    ! printf '%s\n' "$OUT" | grep -q -- '--if-revision'; then ok "$name"; else bad "$name"; fi

name="a put-back the store refuses fails the run and prints a command that runs as printed"
drift "$PUT_BACK" 'cp "$RUN_DIR/policy-before-value.json" "$RUN_DIR/kept.json"; echo garbage >"$RUN_DIR/policy-before-value.json"'$'\n'"$PUT_BACK"
run --tabs cur1
printed=$(printf '%s\n' "$OUT" | sed -n 's/^\*\*\*   \(.*policy set .*\)$/\1/p' | head -1)
if [ "$RC" != 0 ] && ! passed && [ -n "$printed" ] && [ "$(stored)" != "$(found)" ] &&
    cp "$B/kept.json" "$B/policy-before-value.json" && world bash -c "$printed" >/dev/null 2>&1 &&
    [ "$(stored)" = "$(found)" ]; then ok "$name"; else bad "$name"; fi

name="a failed apply's own words are in the bundle, and the policy is put back"
drift '--max-requests "$logical" 2>' '--max-requests 0 2>'
run --tabs cur1
if [ "$RC" != 0 ] && ! passed && [ -s "$B/apply-c1.err" ] && [ "$(stored)" = "$(found)" ]; then ok "$name"; else bad "$name"; fi

if [ "$fails" = 0 ]; then echo "tracer-rung breakers: all held"; else echo "tracer-rung breakers: $fails FAILED"; exit 1; fi
