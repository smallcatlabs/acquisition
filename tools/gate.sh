#!/usr/bin/env bash
# gate.sh — the quality gate (`AGENTS.md`, "Quality gate") as one command.
#
#   tools/gate.sh                 every stage, in the gate's order
#   tools/gate.sh --docs          the five stages that build no Rust, for a
#                                 change no test or build reads, which the
#                                 session establishes; when unsure, the
#                                 full gate
#   tools/gate.sh --only <stage>  one stage, by the name in the table
#
# Each stage runs bare, its output unfiltered, and its exit status is
# captured into its own slot: a stage piped through a filter reports the
# filter's exit, and BSD `grep -qv` once read "all green" over a failing
# line. A failing stage does not stop the others, and a stage name the
# script does not know fails rather than passing as a stage that ran. The table at the end,
# `stage  rc  seconds`, is the verdict, and the script exits 1 when any
# stage failed, so a commit is gated on this script's exit alone.
#
# The full gate ends with `artifacts`: the three executables a frontend
# spawns or a driver starts (`target/debug/acq`, `acqd`, `acq-mcp`) are
# hashed, `cargo build --workspace` runs, and the stage fails if the
# build compiled anything or any hash moved. A `--workspace` test run
# writes them as the plain build does (#88: one feature graph for both,
# and an integration test in every binary's crate, so cargo writes each
# binary on a test run), and the stage holds that at the boundary, what
# the test run leaves on disk. The doc stage writes none of them
# (measured, #88). It still performs the build, so a failing gate leaves
# the plain artifacts behind for any daemon started after it (C84, the
# artifact mismatch). CI runs the stages singly, for timing.
set -uo pipefail
cd "$(dirname "$0")/.."

names=(build test clippy fmt diff docs arch trade bothy doc artifacts)
DOCS=(diff docs arch trade bothy)

# The `artifacts` stage (the header): what the stages before it left in
# target/debug must be what the plain build writes.
ARTIFACTS=(acq acqd acq-mcp)

artifact_hash() {
  if [[ -f target/debug/$1 ]]; then shasum -a 256 "target/debug/$1" | cut -d' ' -f1; else echo absent; fi
}

artifacts() {
  local log before=() a i rc moved=0
  log=$(mktemp) || return 2
  for a in "${ARTIFACTS[@]}"; do before+=("$(artifact_hash "$a")"); done
  cargo build --workspace 2>"$log"
  rc=$?
  cat "$log" >&2
  if grep -E '^ *Compiling ' "$log" >/dev/null; then
    echo "gate: artifacts: the build compiled what the stages before it left behind:" >&2
    grep -E '^ *Compiling ' "$log" >&2
    moved=1
  fi
  rm -f "${log:?}"
  for i in "${!ARTIFACTS[@]}"; do
    a=${ARTIFACTS[$i]}
    if [[ $(artifact_hash "$a") != "${before[$i]}" ]]; then
      echo "gate: artifacts: target/debug/$a changed under cargo build --workspace: a test run wrote another form of it, or never wrote it" >&2
      moved=1
    fi
  done
  if ((moved)); then
    echo "gate: artifacts: a feature divergence shows in a diff of 'cargo tree --workspace -e no-dev,features --prefix none --format \"{p} {f}\"' and the same with '-e features'; a binary with no integration-test target is never written by a test run" >&2
    ((rc)) || rc=1
  fi
  return "$rc"
}

run_stage() {
  case $1 in
    build) cargo build --workspace ;;
    artifacts) artifacts ;;
    test) cargo nextest run --workspace --all-targets ;;
    clippy) cargo clippy --workspace --all-targets --all-features -- -D warnings ;;
    fmt) cargo fmt --all -- --check ;;
    diff) git diff --check ;;
    docs) tools/docs-check.sh ;;
    arch) tools/arch-check.sh ;;
    trade) tools/trade-check.sh ;;
    bothy) python3 -I tools/bothy.py check ;;
    doc) RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps ;;
    *) echo "gate.sh: no stage named '$1'" >&2; return 2 ;;
  esac
}

usage() { sed -n '4,9p' "$0" | sed 's/^# \{0,1\}//'; }

selected=()
case ${1:-} in
  '') selected=("${names[@]}") ;;
  --docs) selected=("${DOCS[@]}") ;;
  --only)
    for name in "${names[@]}"; do [[ ${2:-} == "$name" ]] && selected=("$2"); done
    if ((${#selected[@]} == 0)); then
      echo "gate.sh: --only takes one of: ${names[*]}" >&2; exit 2
    fi ;;
  -h | --help) usage; exit 0 ;;
  *) usage >&2; exit 2 ;;
esac

rcs=(); secs=()
for stage in "${selected[@]}"; do
  printf '\n==> gate: %s\n' "$stage"
  start=$SECONDS
  run_stage "$stage"
  rc=$?
  rcs+=("$rc"); secs+=($((SECONDS - start)))
done

failed=0
printf '\n%-9s %3s %7s\n' stage rc seconds
for i in "${!selected[@]}"; do
  printf '%-9s %3d %7d\n' "${selected[$i]}" "${rcs[$i]}" "${secs[$i]}"
  ((rcs[i] == 0)) || failed=1
done
if ((failed)); then echo 'gate: FAILED'; exit 1; fi
echo 'gate: green'
