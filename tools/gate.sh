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
# spawns or a driver starts (`acq`, `acqd`, `acq-mcp` in cargo's `debug`
# directory) are hashed, `cargo build --workspace` runs, and the stage
# fails if the build compiled anything or any hash moved. A
# `--workspace` test run writes them as the plain build does (#88: one
# feature graph for both, and an integration test in every binary's
# crate, so cargo writes each binary on a test run), and the stage holds
# that at the boundary, what the test run leaves on disk. The doc stage writes none of them
# (measured, #88). It still performs the build, unless an executable
# could not be hashed before it, so a failing gate leaves the plain
# artifacts behind for any daemon started after it (C84, the artifact
# mismatch). CI runs the stages singly, for timing.
#
# The full gate starts with `fixtures`: every golden regenerates when
# `ACQ_UPDATE_FIXTURES` is set at all, `0` and empty among its values,
# and the harness removes `ACQ_*` from the children a test spawns, never
# from the test that compares, so a gate run in a shell that exports it
# rewrote every golden and went green (#87). The script records the
# variable and unsets it before any stage of any selection, so no stage
# rewrites a golden, and the stage fails naming it.
set -uo pipefail
cd "$(dirname "$0")/.."

fixture_switch=${ACQ_UPDATE_FIXTURES+set}
fixture_value=${ACQ_UPDATE_FIXTURES-}
unset ACQ_UPDATE_FIXTURES

names=(fixtures build test clippy fmt diff docs arch trade bothy doc artifacts)
DOCS=(diff docs arch trade bothy)

# The `artifacts` stage (the header): what the stages before it left
# must be what the plain build writes. Equality counts only between
# valid digests of the files cargo itself reports writing: the paths
# hashed before the build are cargo's target directory (`cargo
# metadata`), the build runs with `--message-format=json`, and a binary
# it reports at another path (a configured or cross target) fails the
# stage by name; a hash command that fails, or a digest that is not 64
# hex characters, fails it before any comparison (#88, the reviews).
ARTIFACTS=(acq acqd acq-mcp)

# The SHA-256 of `$1`, or the stage's failure naming it.
artifact_hash() {
  local digest
  if ! digest=$(shasum -a 256 "$1" 2>/dev/null | cut -d' ' -f1) || [[ ! $digest =~ ^[0-9a-f]{64}$ ]]; then
    echo "gate: artifacts: no SHA-256 of $1 (absent, or the hash command failed)" >&2
    return 1
  fi
  echo "$digest"
}

artifacts() {
  local dir log json reported before=() digest a i rc path moved=0
  dir=$(cargo metadata --format-version 1 --no-deps |
    python3 -I -c 'import json, sys; print(json.load(sys.stdin)["target_directory"])') && [[ -n $dir ]] ||
    { echo "gate: artifacts: cargo metadata named no target directory" >&2; return 1; }
  dir=$dir/debug
  for a in "${ARTIFACTS[@]}"; do
    digest=$(artifact_hash "$dir/$a") || return 1
    before+=("$digest")
  done
  log=$(mktemp) && json=$(mktemp) || return 2
  cargo build --workspace --message-format=json 2>"$log" >"$json"
  rc=$?
  cat "$log" >&2
  if grep -E '^ *Compiling ' "$log" >/dev/null; then
    echo "gate: artifacts: the build compiled what the stages before it left behind:" >&2
    grep -E '^ *Compiling ' "$log" >&2
    moved=1
  fi
  reported=$(python3 -I -c '
import json, sys
for line in open(sys.argv[1]):
    m = json.loads(line) if line.startswith("{") else {}
    if m.get("reason") == "compiler-artifact" and m.get("executable") and "bin" in m["target"]["kind"]:
        print(m["target"]["name"], m["executable"])
' "$json")
  rm -f "${log:?}" "${json:?}"
  for i in "${!ARTIFACTS[@]}"; do
    a=${ARTIFACTS[$i]}
    path=$(awk -v n="$a" '$1 == n { print substr($0, length(n) + 2) }' <<<"$reported")
    if [[ $path != "$dir/$a" ]]; then
      echo "gate: artifacts: cargo reports writing $a at '${path:-nowhere}', not the $dir/$a this stage hashed: a configured or cross target writes elsewhere" >&2
      return 1
    fi
    digest=$(artifact_hash "$path") || return 1
    if [[ $digest != "${before[$i]}" ]]; then
      echo "gate: artifacts: $path changed under cargo build --workspace: a test run wrote another form of it, or never wrote it" >&2
      moved=1
    fi
  done
  if ((moved)); then
    echo "gate: artifacts: a feature divergence shows in a diff of 'cargo tree --workspace -e no-dev,features --prefix none --format \"{p} {f}\"' and the same with '-e features'; a binary with no integration-test target is never written by a test run" >&2
    ((rc)) || rc=1
  fi
  return "$rc"
}

# The `fixtures` stage (the header): the switch the script found set.
fixtures() {
  [[ -z $fixture_switch ]] && return 0
  echo "gate: fixtures: ACQ_UPDATE_FIXTURES is set in this shell (to '$fixture_value'): any value, 0 and empty among them, makes every golden rewrite its fixture and pass. This run's stages ran without it; unset it, and regenerate a golden only by the command its failure names." >&2
  return 1
}

run_stage() {
  case $1 in
    fixtures) fixtures ;;
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
