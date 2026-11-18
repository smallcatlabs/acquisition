#!/bin/bash
# tracer-rung.sh — the evidence run of the refresh loop: the stored sync
# policy applied cycle by cycle (plan → apply → replan) until the plan is
# empty, each cycle on a fresh daemon under the tripwire and an exact
# send ceiling, the journal verified after (tools/tracer-verify.py).
#
# This is a live run's driver (C189): for a change to the path a send
# takes, or a first start on a platform. Refreshing a store is `acq
# refresh` or `acq web`, never this.
#
#   tools/tracer-rung.sh --account SEL [--realm R] [--league L] [--cycles K] \
#                        [--tabs all|id,...] [--characters all|id,...] [--max-age S]   # live, from a terminal
#   tools/tracer-rung.sh --mock [the same]                                            # rehearsal, any shell
#
# With no --tabs and no --characters it applies the stored policy as it
# stands. With either, the run covers that selection of the league and
# nothing else of it, stale after --max-age seconds (300), and the
# stored policy is put back when the run ends, however it ends: a run
# leaves stored intent as it found it (policy-before.json is the copy),
# or fails saying it did not and how to put it back.
# Live, it logs nobody in (`acq auth`). Mock, it makes a scratch world
# under runs/mock/, logs a mock account in, stores a policy covering
# every tab and character of the league, and logs out.
#
# The terminal gets what a person at it needs: the plan as `acq` renders
# it, one question per cycle, the apply's own progress line, one line of
# result. Everything else is the bundle, runs/<date>-tracer/, which the
# ledger row is written from:
#   provenance.json            HEAD, the contract revision, both executables' hashes (C84)
#   plan-cN-offline.json       the plan compiled with no daemon: the ceiling's source
#   plan-cN.json               the same plan with the daemon's quote: the file applied
#   apply-cN.txt, apply-cN.err the apply's report, and its progress lines and any refusal
#   jobs-cN.json               every job of the lifetime
#   daemon-cN-status.json      the daemon's own report at the halt
#   cycles.tsv                 per cycle: lifetime, requests, probes, ceiling, token POSTs, quote
#   sends.jsonl, daemon.log    the run's journal and log
#   summary.txt, verify.sh     the verifier's per-send verdict, and the command that reproduces it
#
# What a cycle holds, and stops the run when it does not:
# - The ceiling is exact: one HEAD probe per route the plan touches, one
#   GET per action, and one token POST only when the access token the
#   daemon restores would not outlast the cycle (C135). A token that
#   would lapse mid-cycle is waited out first, so the refresh is the
#   cycle's first send and never a guess.
# - The plan applied is the plan the ceiling was derived from, plus the
#   quote; compiling it and quoting it send nothing.
# - The apply executes every action, and the daemon ends halted on the
#   ceiling with exactly that many sends counted and journaled.
# - An empty plan's apply runs with no daemon and contacts nothing.
set -euo pipefail

here=$(cd "$(dirname "$0")/.." && pwd)
ACQ="$here/target/debug/acq"
ACQD="$here/target/debug/acqd"   # the daemon beside acq (C82); the driver starts it and owns the pid

usage="usage: tracer-rung.sh [--mock] [--account SEL] [--realm R] [--league L] [--cycles K] [--tabs all|id,...] [--characters all|id,...] [--max-age S]"
MODE=live
ACCOUNT=
REALM=pc
LEAGUE=Standard
CYCLES=4
TABS=
CHARACTERS=
MAX_AGE=300
while [ $# -gt 0 ]; do
    case "$1" in
    --mock) MODE=mock; shift ;;
    --account) ACCOUNT=${2:?--account needs a value}; shift 2 ;;
    --realm) REALM=${2:?--realm needs a value}; shift 2 ;;
    --league) LEAGUE=${2:?--league needs a value}; shift 2 ;;
    --cycles) CYCLES=${2:?--cycles needs a value}; shift 2 ;;
    --tabs) TABS=${2:?--tabs needs a value}; shift 2 ;;
    --characters) CHARACTERS=${2:?--characters needs a value}; shift 2 ;;
    --max-age) MAX_AGE=${2:?--max-age needs a value}; shift 2 ;;
    -*) echo "$usage" >&2; exit 2 ;;
    *)
        echo "refusing: '$1' is no flag. A run's own tabs are --tabs all|id,...; with none named it applies the stored policy." >&2
        exit 2 ;;
    esac
done
command -v jq >/dev/null || { echo "jq is required" >&2; exit 2; }
command -v python3 >/dev/null || { echo "python3 is required" >&2; exit 2; }
INTERACTIVE=0
if [ -t 0 ]; then INTERACTIVE=1; fi
if [ "$MODE" = live ] && [ "$INTERACTIVE" = 0 ]; then
    echo "refusing: stdin is not a terminal — a live run is a person's, and each cycle waits for one" >&2
    exit 2
fi

# ---- before any daemon ------------------------------------------------------

# Isolation knobs left over from other work would silently redirect this
# run; the rails knobs are set per daemon below, so a leftover is dropped.
for v in ACQ_STORE_DIR ACQ_LOG_DIR ACQ_NO_KEYRING ACQ_NO_SPAWN ACQ_JOURNAL; do
    if [ -n "${!v:-}" ]; then
        echo "refusing: $v is set in this shell (leftover from other work); unset it first" >&2
        exit 2
    fi
done
unset ACQ_GGG ACQ_PROVIDER ACQ_TRIPWIRE ACQ_MAX_SENDS ACQ_IDLE_SHUTDOWN   # ACQ_GGG is refused by every binary (C88)

RUN_START=$(date +%s)
RUN_DIR="$here/runs/$(date -u +%F)-tracer"
# rehearsals live apart from the evidence the ledger cites
if [ "$MODE" = mock ]; then RUN_DIR="$here/runs/mock/$(date -u +%F)-tracer"; fi
# one directory per attempt
if [ -d "$RUN_DIR" ] && [ -n "$(ls -A "$RUN_DIR")" ]; then
    RUN_DIR="$RUN_DIR-$(date -u +%H%M%S)"
fi
mkdir -p "$RUN_DIR"

# The socket derives from the world (C83): the owner's in live mode, the
# run's scratch store in mock. The run's journal and log are written
# into the bundle, so both are this run's alone.
if [ "$MODE" = live ]; then
    export ACQ_PROVIDER=ggg   # the default (C88), said so the run's environment reads whole
else
    export ACQ_PROVIDER=mock
    export ACQ_STORE_DIR="$RUN_DIR/store"
fi
JOURNAL="$RUN_DIR/sends.jsonl"
export ACQ_LOG_DIR="$RUN_DIR/log"
daemon_log() { find "$RUN_DIR/log" -name daemon.log -type f 2>/dev/null | head -1; }

# No spawn, a clean tree, no daemon, a locked build, and provenance.json
# (tools/preflight.sh). With no daemon up and no spawn, none can appear.
. "$here/tools/preflight.sh"
preflight

ACCT=()
if [ -n "$ACCOUNT" ]; then ACCT=(--account "$ACCOUNT"); fi
acq() { "$ACQ" ${ACCT[@]+"${ACCT[@]}"} "$@"; }
status_json() { "$ACQ" daemon status --json 2>/dev/null || echo '{}'; }
daemon_up() { [ "$(status_json | jq -r '.pid // empty')" != "" ]; }
journal_size() { if [ -f "$JOURNAL" ]; then wc -c <"$JOURNAL" | tr -d ' '; else echo 0; fi; }
# Sends journaled since a byte offset (event lines excluded).
sends_since() { tail -c +$(($1 + 1)) "$JOURNAL" 2>/dev/null | grep -c '"method"' || true; }
span() { if [ "$1" -lt 90 ]; then echo "$1 s"; else echo "$((($1 + 30) / 60)) min"; fi; }
some() { if [ "$1" = 1 ]; then echo "1 $2"; else echo "$1 $2s"; fi; }
stop() { echo "*** $*" >&2; exit 1; }

# The stored policy, put back as it was: the run's own coverage never
# outlives the run. One lifecycle. It is owed (1) from before the run's
# write is attempted, so a run cut between the write and its receipt
# still owes it. The put-back reads the store, never a receipt: nothing
# is owed where the stored revision is the one the run found; the run's
# write is undone only where the store holds exactly it, one revision on,
# over that revision; anything else is another client's edit and is left
# alone. It is settled (0) only by one of the first two. A put-back that
# did not happen (2) is said once, with the command that does it, and
# fails the run.
POLICY_OWED=0
restore_policy() {
    [ "$POLICY_OWED" = 2 ] && return 1
    [ "$POLICY_OWED" = 1 ] || return 0
    POLICY_OWED=2
    local now rev
    if ! now=$(acq policy show --json 2>"$RUN_DIR/policy-restored.err"); then
        echo "*** the stored policy could not be read (policy-restored.err), so it was NOT put back." >&2
        echo "*** The policy before the run is $RUN_DIR/policy-before-value.json; \`acq policy show\` says what stands." >&2
        return 1
    fi
    rev=$(jq '.policy.revision // 0' <<<"$now")
    if [ "$rev" = "$BEFORE_REVISION" ]; then
        POLICY_OWED=0   # the run's write never landed
        return 0
    fi
    if [ "$rev" != $((BEFORE_REVISION + 1)) ] || [ "$(jq -S '.policy.value' <<<"$now")" != "$(jq -S . <<<"$RUN_POLICY")" ]; then
        echo "*** the stored policy (revision $rev) is not the one this run wrote: another client wrote it since, and its edit is left as it is." >&2
        echo "*** The policy before the run is $RUN_DIR/policy-before-value.json; \`acq policy show\` says what stands." >&2
        return 1
    fi
    if acq policy set "@$RUN_DIR/policy-before-value.json" --replace --if-revision "$rev" --json \
        >"$RUN_DIR/policy-restored.json" 2>"$RUN_DIR/policy-restored.err"; then
        POLICY_OWED=0
        echo "policy: the stored policy is back as it was before the run"
        return 0
    fi
    echo "*** the stored policy was NOT put back (policy-restored.err): it is still this run's selection." >&2
    echo "*** Put it back by hand, over the revision the run wrote:" >&2
    printf '***   %s\n' "$(printf '%q ' "$ACQ" ${ACCT[@]+"${ACCT[@]}"} policy set "@$RUN_DIR/policy-before-value.json" --replace --if-revision "$rev")" >&2
    return 1
}

DAEMON_STARTED=0
COMPLETED=0
cleanup() {
    local rc=$?
    restore_policy || rc=1
    [ "$COMPLETED" = 1 ] && exit "$rc"
    echo ""
    if [ "$DAEMON_STARTED" = 0 ]; then
        echo "*** stopped before any daemon started; nothing was sent."
    elif daemon_up; then
        # Ctrl-C reaches the daemon too, and it drains and leaves by itself (C133).
        echo "*** stopped with a daemon up. After Ctrl-C it drains and leaves within 18 s;"
        echo "*** otherwise \`acq daemon stop\` ends it. Its jobs stay on disk: \`acq jobs\`, then"
        echo "*** \`acq cancel\` what should not resume (the live-run skill, \"If something trips\")."
    else
        echo "*** stopped; no daemon is up. The queue is on disk: \`acq jobs\` reads it."
    fi
    if [ "$MODE" = mock ]; then echo "*** this run's world: ACQ_PROVIDER=mock ACQ_STORE_DIR=$ACQ_STORE_DIR"; fi
    echo "*** what was written is in $RUN_DIR"
    exit "$rc"
}
trap cleanup EXIT

spawn_daemon() { # <max_sends> <outfile>
    DAEMON_STARTED=1
    env ACQ_TRIPWIRE=1 ACQ_MAX_SENDS="$1" ACQ_IDLE_SHUTDOWN=600 ACQ_JOURNAL="$JOURNAL" \
        "$ACQD" >"$RUN_DIR/$2" 2>&1 &
    for _ in $(seq 1 100); do
        daemon_up && return 0
        sleep 0.1
    done
    echo "the daemon did not come up; see $RUN_DIR/$2 and $(daemon_log)" >&2
    return 1
}

stop_daemon() {
    "$ACQ" daemon stop >/dev/null 2>&1 || true
    for _ in $(seq 1 100); do
        daemon_up || return 0
        sleep 0.1
    done
    echo "the daemon did not stop" >&2
    return 1
}

# The daemon's own report, saved per lifetime after its sends: the socket
# it was reached on, its world, contract and artifact (C83, C84), the
# rails at the halt. A report that names no socket is no daemon's.
save_status() { # <tag>
    status_json >"$RUN_DIR/daemon-$1-status.json"
    [ -n "$(jq -r '.socket // empty' "$RUN_DIR/daemon-$1-status.json")" ] ||
        stop "$1: no daemon reported on this world's socket (daemon-$1-status.json)"
}

# The rails after a lifetime's sends, from the daemon that sent them. The
# ceiling is exact, so the bound is reached at the last planned send: the
# tripwire armed, the ceiling the plan's, that many sends counted and
# journaled, and a ceiling halt in force. A journal count that merely
# matches is no evidence the rail was there.
check_rails() { # <what> <journal offset at its start> <expected sends>
    local st armed max sends halted journaled ok=1
    st=$(status_json)
    armed=$(jq -r '.rails.tripwire_enabled // false' <<<"$st")
    max=$(jq -r '.rails.max_sends // "none"' <<<"$st")
    sends=$(jq -r '.rails.sends // "unknown"' <<<"$st")
    halted=$(jq -r '.rails.halted // empty' <<<"$st")
    journaled=$(sends_since "$2")
    [ "$armed" = true ] || { echo "*** $1: the daemon reports the tripwire not armed" >&2; ok=0; }
    [ "$max" = "$3" ] || { echo "*** $1: the daemon's ceiling is $max, the plan's is $3" >&2; ok=0; }
    [ "$sends" = "$3" ] || { echo "*** $1: the daemon counted $sends send(s), $3 planned" >&2; ok=0; }
    [ "$journaled" = "$3" ] || { echo "*** $1: $journaled send(s) journaled, $3 planned" >&2; ok=0; }
    case "$halted" in
    "") echo "*** $1: no halt in force — the ceiling was not reached" >&2; ok=0 ;;
    *ceiling*) ;;
    *)
        echo "*** the tripwire tripped during $1: $halted" >&2
        echo "*** stop here. Read $JOURNAL; wait 360 s before any reset (the live-run skill)." >&2
        return 1 ;;
    esac
    [ "$ok" = 1 ] || { echo "*** $1 did not end on its ceiling; read $JOURNAL, and \`acq jobs\` for what was refused" >&2; return 1; }
}

# A failed apply ends the run with what a passing one keeps: the jobs, the
# daemon's report, its log. Then the daemon is stopped: the queue is on
# disk (C45), and a daemon left up would only refuse the next run.
fail_apply() { # <tag> <journal offset at cycle start> <ceiling>
    "$ACQ" jobs >&2 || true
    "$ACQ" jobs --all --json >"$RUN_DIR/jobs-$1.json" 2>/dev/null || true
    check_rails "cycle ${1#c}" "$2" "$3" || true
    status_json >"$RUN_DIR/daemon-$1-status.json"
    if [ -n "$(daemon_log)" ]; then cp "$(daemon_log)" "$RUN_DIR/daemon.log"; fi
    stop_daemon || true
    stop "cycle ${1#c} failed; the daemon is stopped and its evidence is in $RUN_DIR"
}

echo "tracer rung ($MODE): $ver at $tip, $REALM $LEAGUE, up to $(some "$CYCLES" cycle)"
echo "bundle: $RUN_DIR"

# A queue an earlier daemon left on disk resumes in the first daemon this
# run starts, under no plan of the run's, and spends its ceiling. `jobs`
# reads the queue with no daemon (C45).
open_jobs=$("$ACQ" jobs --json | jq 'if .persisted == null then 0 else .persisted.total end')
if [ "$open_jobs" != 0 ]; then
    echo "refusing: $open_jobs job(s) an earlier daemon left on disk would resume in this run's first daemon:" >&2
    "$ACQ" jobs >&2 || true
    echo "cancel them first (the live-run skill, \"If something trips\"), then run again" >&2
    exit 2
fi

# ---- mock only: a login and a stored policy in the scratch world -------------

LOGIN_LIFETIME=0
if [ "$MODE" = mock ]; then
    login_offset=$(journal_size)
    spawn_daemon 2 daemon-login.out
    "$ACQ" auth --no-browser --json >"$RUN_DIR/auth.json" 2>"$RUN_DIR/auth.err" &
    AUTH_PID=$!
    URL=
    for _ in $(seq 1 50); do
        URL=$(jq -r '.authorize_url // empty' "$RUN_DIR/auth.json" 2>/dev/null | head -1 || true)
        [ -n "$URL" ] && break
        sleep 0.1
    done
    [ -n "$URL" ] || stop "the mock login printed no authorize_url (auth.json, auth.err)"
    # unquoted on purpose: bash 3.2 keeps a quoted replacement's backslash
    APPROVE=${URL/\/authorize?/\/approve?}
    curl -sL "$APPROVE&user=tracer-rung" >/dev/null || stop "the mock login's approval was refused ($APPROVE)"
    wait "$AUTH_PID" || stop "the mock login did not complete (auth.json, auth.err)"
    check_rails "the login" "$login_offset" 2
    save_status login
    stop_daemon
    LOGIN_LIFETIME=1
    # stashes are PoE1 only: a poe2 entry covers characters alone
    facets='{"tabs": "all", "characters": "all"}'
    if [ "$REALM" = poe2 ]; then facets='{"characters": "all"}'; fi
    POLICY=$(jq -nc --arg r "$REALM" --arg l "$LEAGUE" --argjson f "$facets" \
        '{version: 3, realms: {($r): {leagues: {($l): ({max_age_seconds: 3600} + $f)}}}}')
    acq policy set "$POLICY" --json >"$RUN_DIR/policy-mock.json"
fi

# ---- the run's own coverage, when it names one -----------------------------------

if [ -n "$TABS$CHARACTERS" ]; then
    selection() { if [ "$1" = all ]; then echo '"all"'; else jq -Rc 'split(",") | map(select(. != ""))' <<<"$1"; fi; }
    entry=$(jq -nc --argjson a "$MAX_AGE" '{max_age_seconds: $a}')
    if [ -n "$TABS" ]; then entry=$(jq -c --argjson t "$(selection "$TABS")" '. + {tabs: $t}' <<<"$entry"); fi
    if [ -n "$CHARACTERS" ]; then entry=$(jq -c --argjson c "$(selection "$CHARACTERS")" '. + {characters: $c}' <<<"$entry"); fi
    acq policy show --json >"$RUN_DIR/policy-before.json"
    [ "$(jq '.policy == null' "$RUN_DIR/policy-before.json")" = false ] ||
        stop "no policy is stored, so there is none to put back after the run: \`acq policy set\` one first"
    jq '.policy.value' "$RUN_DIR/policy-before.json" >"$RUN_DIR/policy-before-value.json"
    # `policy set` replaces each realm it names whole, so the realm is
    # written with every league the stored policy keeps in it and this
    # run's league set.
    POLICY=$(jq -c --arg r "$REALM" --arg l "$LEAGUE" --argjson e "$entry" \
        '{version: 3, realms: {($r): {leagues: ((.policy.value.realms[$r].leagues // {}) + {($l): $e})}}}' \
        "$RUN_DIR/policy-before.json")
    # What the store holds once the write lands: the value before, this
    # realm replaced. The put-back undoes exactly this and nothing else.
    RUN_POLICY=$(jq -c --arg r "$REALM" --argjson p "$POLICY" '.policy.value | .realms[$r] = $p.realms[$r]' "$RUN_DIR/policy-before.json")
    BEFORE_REVISION=$(jq '.policy.revision' "$RUN_DIR/policy-before.json")
    POLICY_OWED=1
    acq policy set "$POLICY" --if-revision "$BEFORE_REVISION" --json >"$RUN_DIR/policy-run.json" ||
        stop "the run's policy was refused (policy-run.json)"
    echo "policy: for this run $REALM $LEAGUE covers ${TABS:+tabs $TABS}${TABS:+${CHARACTERS:+ and }}${CHARACTERS:+characters $CHARACTERS}, stale after $MAX_AGE s; the stored policy is put back when the run ends"
fi

# ---- the cycles ---------------------------------------------------------------

# What two envelopes must agree on to be the same plan: everything but
# the quote, the two timestamps a compile stamps afresh, and the ages
# inside reasons, which advance with the clock (the reason's kind stays).
plan_identity() {
    jq -S 'del(.quote, .generated_at, .basis.snapshot_taken_at)
           | walk(if type == "object" then del(.age_seconds) else . end)' "$1"
}
count() { jq --arg k "$2" '[.actions[] | select(.action == $k)] | length' "$1"; }
TTY=0
if [ -t 1 ] && [ -t 2 ]; then TTY=1; fi
progress() { # <file>: stderr lines, kept, and transient on a terminal
    local line
    while IFS= read -r line; do
        printf '%s\n' "$line" >>"$1"
        if [ "$TTY" = 1 ]; then printf '\r\033[2K%s' "$line" >&2; else printf '%s\n' "$line" >&2; fi
    done
}
report() { # stdout lines, over whatever transient line stands
    local line
    while IFS= read -r line; do
        if [ "$TTY" = 1 ]; then printf '\r\033[2K%s\n' "$line"; else printf '%s\n' "$line"; fi
    done
}
# The seconds a restored access token must have left beyond the cycle's
# estimate to be trusted through it (C135). A send refreshes a token with
# 5 s or less left, so one that would lapse inside the cycle refreshes at
# a send no plan can name; the driver waits it out instead.
TOKEN_MARGIN=30
# The acting account's access token, its seconds left as the daemon
# reports them once the start's keyring restore is over; empty when the
# daemon holds none for it.
token_seconds() {
    local st=
    for _ in $(seq 1 600); do
        st=$("$ACQ" auth status --json 2>/dev/null || echo '{}')
        [ "$(jq -r '.keyring // empty' <<<"$st")" = restoring ] || break
        sleep 0.1
    done
    jq -r --arg u "$ACTING" '
        [.accounts[]? | select(.username == $u)]
        | if length == 1 then (.[0].access_expires_in_seconds // empty) else empty end' <<<"$st"
}

# One row per cycle for the verifier: cycle, lifetime index in the
# journal, requests, probes, ceiling, token POSTs planned, quote outcome.
CYCLE_ROWS="$RUN_DIR/cycles.tsv"
: >"$CYCLE_ROWS"
LIFETIMES=$LOGIN_LIFETIME
CLOSED=0
for c in $(seq 1 "$CYCLES"); do
    tag="c$c"
    offline="$RUN_DIR/plan-$tag-offline.json"
    plan="$RUN_DIR/plan-$tag.json"

    # The plan with no daemon: the ceiling's source.
    daemon_up && stop "a daemon is up before cycle $c's plan"
    acq refresh --plan --realm "$REALM" --league "$LEAGUE" --json >"$offline" 2>"$RUN_DIR/plan-$tag-offline.note" || {
        cat "$RUN_DIR/plan-$tag-offline.note" >&2; exit 1; }
    daemon_up && stop "a daemon appeared while cycle $c's plan compiled: the plan spawned one"
    [ "$(jq -r '.quote == null' "$offline")" = true ] || stop "cycle $c's plan carries a quote with no daemon up"
    ACTING=$(jq -r '.account_name' "$offline")
    logical=$(jq -r '.logical_requests' "$offline")

    if [ "$logical" = 0 ]; then
        if [ "$c" = 1 ]; then
            acq refresh --plan="$offline"
            echo "refusing: the stored policy asks for nothing in $REALM $LEAGUE that is not fresh, so there is no run." >&2
            echo "\`acq policy show\` says what it covers; \`acq policy set\` changes it." >&2
            exit 2
        fi
        # The closed loop: an empty plan applies as a no-op that contacts nothing.
        acq refresh --apply --realm "$REALM" --league "$LEAGUE" --json >"$RUN_DIR/apply-$tag-noop.json"
        [ "$(jq -r '.requests' "$RUN_DIR/apply-$tag-noop.json")" = 0 ] || stop "the empty plan's apply reported requests"
        daemon_up && stop "the empty plan's apply spawned a daemon"
        echo ""
        echo "cycle $c: nothing left to fetch; the empty plan's apply started no daemon"
        CLOSED=$c
        break
    fi

    lists=$(count "$offline" list_stashes)
    stashes=$(($(count "$offline" fetch_tab) + $(count "$offline" fetch_substash)))
    clists=$(count "$offline" list_characters)
    chars=$(count "$offline" fetch_character)
    [ $((lists + stashes + clists + chars)) = "$logical" ] || stop "cycle $c's plan carries an action kind this driver does not count"
    [ "$(jq -r '.wire_sends.min' "$offline")" = "$logical" ] || stop "cycle $c's plan: its wire minimum is not its request count"
    # One HEAD per route a lifetime first touches: the stash list, the
    # stash fetch, the character list, the character fetch.
    probes=0
    for n in "$lists" "$stashes" "$clists" "$chars"; do
        if [ "$n" -gt 0 ]; then probes=$((probes + 1)); fi
    done
    # The cycle's seconds on the wire, over-estimated on purpose: a ~343 s
    # hold per 30 GETs on one policy and a ~15 s hold per 15, a minute of
    # slack. The stash and character policies pace side by side, so the
    # cycle lasts the longer one.
    est=5
    if [ "$MODE" = live ]; then
        gets=$stashes
        if [ "$chars" -gt "$gets" ]; then gets=$chars; fi
        est=$(((gets / 30) * 343 + (gets / 15) * 15 + 60))
    fi
    # What a cycle lands at its start is the oldest fact when the next
    # plan compiles, so the window must outlive the cycle with margin, or
    # the next plan fetches it all again and the loop cannot close.
    window=$(jq -r '.max_age_seconds' "$offline")
    if [ $((est * 2)) -gt "$window" ]; then
        echo "refusing: cycle $c would take about $(span "$est") and the policy's window is $(span "$window"): what it lands would be stale before the next plan." >&2
        echo "A window of at least $((est * 2)) s lets the loop close (--max-age with a selection, or the stored policy's max_age_seconds)." >&2
        exit 2
    fi
    need=$((est + TOKEN_MARGIN))

    # The daemon, under the ceiling its token calls for.
    posts=0
    ceiling=$((probes + logical))
    cycle_offset=$(journal_size)
    spawn_daemon "$ceiling" "daemon-$tag.out"
    LIFETIMES=$((LIFETIMES + 1))
    left=$(token_seconds)
    if [ -z "$left" ] || [ "$left" -lt "$need" ]; then
        # Stopped before it sent anything, and replaced by one whose
        # ceiling carries the refresh's POST: a token read, its own
        # lifetime in the journal.
        stop_daemon
        [ "$(sends_since "$cycle_offset")" = 0 ] || stop "the token read sent something"
        mv "$RUN_DIR/daemon-$tag.out" "$RUN_DIR/daemon-$tag-read.out"
        if [ -n "$left" ] && [ "$left" -gt 3 ]; then
            echo ""
            echo "cycle $c: the access token has $(span "$left") left and the cycle needs $(span "$need")."
            echo "Waiting until $(date -v "+$((left - 3))S" +%H:%M:%S 2>/dev/null || date -d "+$((left - 3)) seconds" +%H:%M:%S) for it to lapse, so the cycle's first send is its refresh."
            sleep $((left - 3))
        fi
        posts=1
        ceiling=$((1 + probes + logical))
        spawn_daemon "$ceiling" "daemon-$tag.out"
        LIFETIMES=$((LIFETIMES + 1))
    fi

    # The same plan with the daemon's quote: the file that is applied.
    acq refresh --plan --realm "$REALM" --league "$LEAGUE" --json >"$plan" 2>"$RUN_DIR/plan-$tag.note" || {
        cat "$RUN_DIR/plan-$tag.note" >&2; stop_daemon; exit 1; }
    if [ "$(plan_identity "$offline")" != "$(plan_identity "$plan")" ]; then
        echo "*** cycle $c: the plan compiled with the daemon up is not the offline plan plus a quote:" >&2
        diff <(plan_identity "$offline") <(plan_identity "$plan") >&2 || true
        stop_daemon
        exit 1
    fi
    [ "$(sends_since "$cycle_offset")" = 0 ] || { stop_daemon; stop "cycle $c: the quote sent something"; }
    if [ "$(jq -r '.quote != null' "$plan")" = true ]; then
        quote="quoted"
    else
        quote="unquoted: $(tr -d '\n' <"$RUN_DIR/plan-$tag.note")"
    fi

    echo ""
    echo "cycle $c: the plan"
    echo ""
    # the plan's own `next:` line names a command this driver is about to run
    acq refresh --plan="$plan" | grep -v '^next: ' | sed 's/^/    /'
    echo ""
    refresh_words="no token refresh"
    if [ "$posts" = 1 ]; then refresh_words="1 token refresh"; fi
    echo "cycle $c: $(some "$logical" request) under a ceiling of $ceiling sends ($(some "$probes" probe), $refresh_words), about $(span "$est")."
    if [ "$INTERACTIVE" = 1 ]; then
        read -r -p "Enter to apply, Ctrl-C to stop: "
    fi

    # The apply's stdout is its report and its stderr its progress line
    # and, when it fails, its reason: both are kept, line by line. On a
    # terminal a stderr line is drawn in place and a stdout line over it.
    : >"$RUN_DIR/apply-$tag.err"
    mkfifo "$RUN_DIR/apply.fifo"
    progress "$RUN_DIR/apply-$tag.err" <"$RUN_DIR/apply.fifo" &
    progress_pid=$!
    applied=0
    acq refresh --apply="$plan" --max-requests "$logical" 2>"$RUN_DIR/apply.fifo" | tee "$RUN_DIR/apply-$tag.txt" | report || applied=$?
    wait "$progress_pid" || true
    rm -f "$RUN_DIR/apply.fifo"
    if [ "$TTY" = 1 ]; then printf '\r\033[2K' >&2; fi
    if [ "$applied" != 0 ]; then
        echo "*** cycle $c: the apply failed. Its own words (apply-$tag.err):" >&2
        tail -n 20 "$RUN_DIR/apply-$tag.err" >&2
        fail_apply "$tag" "$cycle_offset" "$ceiling"
    fi
    # Every job of the lifetime (`--all`: the default lists ten of each).
    # The apply executed the plan when its parent is done and every action
    # is a done child of it.
    "$ACQ" jobs --all --json >"$RUN_DIR/jobs-$tag.json"
    parent=$(jq '[.rows[] | select(.kind == "apply" and .parent == null)] | max_by(.id) // {}' "$RUN_DIR/jobs-$tag.json")
    parent_id=$(jq -r '.id // empty' <<<"$parent")
    done_n=0
    if [ -n "$parent_id" ]; then
        done_n=$(jq --argjson p "$parent_id" '[.rows[] | select(.parent == $p and .state == "done")] | length' "$RUN_DIR/jobs-$tag.json")
    fi
    if [ "$(jq -r '.state // empty' <<<"$parent")" != "done" ] || [ "$done_n" != "$logical" ]; then
        echo "*** cycle $c: the apply did not execute exactly its plan: $done_n of $logical action(s) done under job ${parent_id:-none}" >&2
        fail_apply "$tag" "$cycle_offset" "$ceiling"
    fi
    check_rails "cycle $c" "$cycle_offset" "$ceiling"
    save_status "$tag"
    stop_daemon
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$c" "$LIFETIMES" "$logical" "$probes" "$ceiling" "$posts" "$quote" >>"$CYCLE_ROWS"
    echo "cycle $c: $logical of $logical applied; the daemon halted on its ceiling at $ceiling of $ceiling sends"
done
if [ "$CLOSED" = 0 ]; then
    echo ""
    echo "the plan is not empty after $(some "$CYCLES" cycle): \`acq refresh --plan\` says what is left. Recorded as such, not as a failure."
fi

# A put-back that fails has said so; the readback and the verdict on the
# sends still follow, and the run then ends without its pass.
restore_policy || true

# ---- the store, read back (no daemon) -------------------------------------------

# Since the run started, plus an hour of margin. The default lists ten
# events, so the limit is explicit, and reaching it stops the run: the
# evidence would be cut short.
hours=$(python3 -c "import time; print((time.time() - $RUN_START) / 3600 + 1)")
acq store status >"$RUN_DIR/store-status.txt" 2>&1
acq store events --hours "$hours" --limit 1000000 --summary --expand >"$RUN_DIR/store-events.txt"
[ "$(acq store events --hours "$hours" --limit 1000000 --json | jq '.limit_reached')" = false ] ||
    stop "the item events since the run started reached the readback's limit: store-events.txt is cut short"
if [ "$MODE" = mock ]; then
    spawn_daemon 2 daemon-logout.out
    acq auth logout >/dev/null 2>&1 || true
    stop_daemon
fi
# No daemon is left; what remains is reading.
COMPLETED=1

# ---- the verdict ------------------------------------------------------------------

# The verification runs on the bundle's journal from byte 0 through the
# bundle's own copy of the verifier, so verify.sh reproduces the verdict
# wherever the bundle lives and whatever the tree's verifier becomes.
[ -f "$JOURNAL" ] || stop "no journal was written at $JOURNAL"
if [ -n "$(daemon_log)" ]; then cp "$(daemon_log)" "$RUN_DIR/daemon.log"; fi
# Which daemon sent: every lifetime's header against provenance.json (C84).
provenance_matches_journal "$JOURNAL" >"$RUN_DIR/provenance.txt"
cp "$here/tools/tracer-verify.py" "$RUN_DIR/tracer-verify.py"
(cd "$RUN_DIR" && shasum -a 256 tracer-verify.py sends.jsonl cycles.tsv provenance.json >checksums.sha256)
cat >"$RUN_DIR/verify.sh" <<EOS
#!/bin/sh
# Re-verify this bundle: $(basename "$RUN_DIR"), $ver, tip $tip (provenance.json).
# Uses the verifier copied into the bundle, never the working tree's.
cd "\$(dirname "\$0")" && shasum -a 256 -c checksums.sha256 >/dev/null &&
    python3 ./tracer-verify.py sends.jsonl 0 cycles.tsv $LOGIN_LIFETIME $CLOSED $MODE $REALM
EOS
chmod +x "$RUN_DIR/verify.sh"

echo ""
if ! "$RUN_DIR/verify.sh" >"$RUN_DIR/summary.txt"; then
    cat "$RUN_DIR/summary.txt"
    stop "the journal does not verify; evidence in $RUN_DIR"
fi
python3 "$RUN_DIR/tracer-verify.py" "$JOURNAL" 0 "$CYCLE_ROWS" "$LOGIN_LIFETIME" "$CLOSED" "$MODE" "$REALM" brief |
    sed -n '/^lifetime /,/^$/p'
if [ "$POLICY_OWED" != 0 ]; then
    stop "every send verified against its plan, but the stored policy is not as the run found it (said above): no pass. Bundle: $RUN_DIR"
fi
if [ "$CLOSED" != 0 ]; then
    echo "pass: the loop closed at cycle $CLOSED, every send verified against its plan."
else
    echo "pass on the wire: every send verified against its plan; the loop did not close in $(some "$CYCLES" cycle)."
fi
echo "bundle: $RUN_DIR (summary.txt is the verdict per send; ./verify.sh reproduces it)"
if [ "$MODE" = live ]; then
    echo "next: tell the session the run is done; it writes the RUN-LEDGER.md row from the bundle."
fi
