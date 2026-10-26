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
# The full gate ends with `cargo build --workspace` again. The test and
# doc stages rewrite `target/debug/acq-mcp` in another form and never
# write `acqd`, so a daemon started after them is the artifact mismatch
# (C84); the last stage puts the plain artifacts back, and the rule holds
# by order rather than by memory. CI runs the stages singly, for timing.
set -uo pipefail
cd "$(dirname "$0")/.."

names=(build test clippy fmt diff docs arch trade bothy doc rebuild)
DOCS=(diff docs arch trade bothy)

run_stage() {
  case $1 in
    build | rebuild) cargo build --workspace ;;
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

case ${1:-} in
  '') selected=("${names[@]}") ;;
  --docs) selected=("${DOCS[@]}") ;;
  --only)
    for name in "${names[@]}"; do [[ ${2:-} == "$name" ]] && selected=("$2"); done
    if [[ -z ${selected+x} ]]; then
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
printf '\n%-8s %3s %7s\n' stage rc seconds
for i in "${!selected[@]}"; do
  printf '%-8s %3d %7d\n' "${selected[$i]}" "${rcs[$i]}" "${secs[$i]}"
  ((rcs[i] == 0)) || failed=1
done
if ((failed)); then echo 'gate: FAILED'; exit 1; fi
echo 'gate: green'
