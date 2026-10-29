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
# The `features` stage holds the same property at its cause, on every
# target a release builds, and builds nothing: for each, the plain
# build's feature graph (`cargo tree -e no-dev,features`) against the
# test build's (`-e features`), package by package, name and version,
# failing on each package whose features differ and naming both sets.
# `artifacts` sees only this host's binaries, after a test run; this
# stage sees every target's graph in a second, so CI runs it and a
# dependency bump that splits a binary is red on its own checks (#128,
# item 2). A split is closed by a pin in the binary's crate, as
# `acquisition-mcp` and `acquisition-cli` hold (#88, #125, #124).
#
# The full gate starts with `fixtures`: every golden regenerates when
# `ACQ_UPDATE_FIXTURES` is set at all, `0` and empty among its values,
# and the harness removes `ACQ_*` from the children a test spawns, never
# from the test that compares, so a gate run in a shell that exports it
# rewrote every golden and went green (#87). The script records the
# variable and unsets it before any stage of any selection, so no stage
# rewrites a golden, and the stage fails naming it.
#
# The full gate ends with `tree`: the working tree its stages leave must
# be the one they found (#87). A session gates its uncommitted work, so
# the stage compares the tree before the first stage with the tree after
# the last, never against a clean checkout (CI's checkout is clean, and
# its step is the literal `git status --porcelain` check): tracked files
# and the untracked ones `.gitignore` does not hide, by content, so a
# golden already edited and rewritten again counts. A file a test must
# write belongs under its scratch or an ignored path.
set -uo pipefail
cd "$(dirname "$0")/.."

fixture_switch=${ACQ_UPDATE_FIXTURES+set}
fixture_value=${ACQ_UPDATE_FIXTURES-}
unset ACQ_UPDATE_FIXTURES

names=(fixtures build test clippy fmt diff docs arch trade bothy features doc artifacts tree)
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
    echo "gate: artifacts: tools/gate.sh --only features names each package a test build compiles with other features; a binary with no integration-test target is never written by a test run" >&2
    ((rc)) || rc=1
  fi
  return "$rc"
}

# The `features` stage (the header). The targets are the release's,
# read from its build matrix, so a target added there is checked here.
# The graphs are the workspace's, as the build and the test run are: a
# pin in one binary's crate unifies into every binary of the build, so
# `acq`'s rustix pin also closes the split Linux's `acqd` had through
# keyring's zbus (#91's Do 1), which `-p acquisition-daemon` alone still
# shows and no stage builds.
RELEASE=.github/workflows/release.yml

# The release's targets, one a line, from the `build` job's
# `matrix.include`: each row must be the inline `- { os: …, target: … }`
# the file writes, and a row in any other form is refused by name, so the
# stage never goes green over a target it could not read (the review of
# #91, its first finding).
release_targets() {
  python3 -I -c '
import re, sys
lines = open(sys.argv[1]).read().split("\n")
def indent(l): return len(l) - len(l.lstrip(" "))
job = next((i for i, l in enumerate(lines) if re.match(r"^  build:\s*$", l)), None)
inc = next((i for i in range(job + 1, len(lines)) if lines[i].strip() == "include:"), None) if job is not None else None
if inc is None:
    sys.exit(sys.argv[1] + ": no jobs.build.strategy.matrix.include")
targets, bad = [], []
for l in lines[inc + 1:]:
    if not l.strip() or l.strip().startswith("#"):
        continue
    if indent(l) <= indent(lines[inc]):
        break
    m = re.match(r"^\s*- \{ os: [^,{}]+, target: ([a-z0-9_-]+) \}\s*$", l)
    (targets.append(m[1]) if m else bad.append(l.strip()))
if bad or not targets:
    sys.exit(sys.argv[1] + ": a matrix row not in the form - { os: <runner>, target: <triple> }, so its target would go unchecked: " + "; ".join(bad or ["none listed"]))
print("\n".join(targets))
' "$RELEASE"
}

# Each package of the target's two feature graphs, `<name> <version>`,
# whose feature sets differ, with both: exit 1 when any does, and 2 when
# a graph could not be read, which is cargo's failure and no split.
feature_split() {
  local graphs rc
  graphs=$(mktemp -d) || return 2
  if ! cargo tree --workspace --target "$1" -e no-dev,features --prefix none --format '{p} {f}' >"$graphs/build" ||
    ! cargo tree --workspace --target "$1" -e features --prefix none --format '{p} {f}' >"$graphs/test"; then
    rm -rf "${graphs:?}"
    return 2
  fi
  python3 -I -c '
import re, sys
def graph(path):
    seen = {}
    for line in open(path):
        m = re.match(r"^(\S+) v(\S+)(?: \([^)]*\))? ?(\S*?)(?: \(\*\))?$", line.rstrip("\n"))
        if m:
            seen.setdefault(f"{m[1]} {m[2]}", set()).add(m[3])
    return seen
build, test = graph(sys.argv[1]), graph(sys.argv[2])
split = [p for p in sorted(build.keys() & test.keys()) if build[p] != test[p]]
for p in split:
    print(p + ": build " + " | ".join(sorted(build[p])) + "; test " + " | ".join(sorted(test[p])))
sys.exit(2 if not build or not test else 1 if split else 0)
' "$graphs/build" "$graphs/test"
  rc=$?
  rm -rf "${graphs:?}"
  return "$rc"
}

features() {
  local targets t out rc=0
  targets=$(release_targets) || { echo "gate: features: the release's targets could not be read: the line above names why" >&2; return 1; }
  for t in $targets; do
    out=$(feature_split "$t")
    case $? in
      0) echo "gate: features: $t: the build and the test build agree" ;;
      1) echo "gate: features: $t: a test build compiles these packages with other features than the build, so a test run writes another binary (C84, the artifact mismatch); pin the features in a binary's crate, as acquisition-cli and acquisition-mcp do:" >&2
         echo "$out" >&2; rc=1 ;;
      *) echo "gate: features: $t: cargo tree could not read the feature graphs (its words above): no comparison was made, and nothing here says a pin is due" >&2; rc=1 ;;
    esac
  done
  return "$rc"
}

# The `fixtures` stage (the header): the switch the script found set.
fixtures() {
  [[ -z $fixture_switch ]] && return 0
  echo "gate: fixtures: ACQ_UPDATE_FIXTURES is set in this shell (to '$fixture_value'): any value, 0 and empty among them, makes every golden rewrite its fixture and pass. This run's stages ran without it; unset it, and regenerate a golden only by the command its failure names." >&2
  return 1
}

# The `tree` stage (the header). A snapshot is the tree object of the
# working tree, written through a copy of the index (`git add -A`, then
# `git write-tree`), so the real index is never touched and `git diff`
# names what moved; it leaves loose blobs, which `git gc` prunes. A
# snapshot that cannot be taken fails the stage, never passes it.
tree_snapshot() {
  local index tree rc
  index=$(mktemp) || return 1
  cp "$(git rev-parse --git-path index)" "$index" 2>/dev/null || rm -f "$index"
  GIT_INDEX_FILE=$index git add -A >/dev/null && tree=$(GIT_INDEX_FILE=$index git write-tree)
  rc=$?
  rm -f "$index"
  ((rc == 0)) && [[ $tree =~ ^[0-9a-f]{40}([0-9a-f]{24})?$ ]] && echo "$tree"
}

tree_stage() {
  local after
  if [[ -z $tree_before ]]; then
    echo "gate: tree: no snapshot of the working tree before the first stage (git add -A or git write-tree failed through a copy of the index): what the stages wrote cannot be told" >&2
    return 1
  fi
  if ! after=$(tree_snapshot); then
    echo "gate: tree: no snapshot of the working tree after the last stage (git add -A or git write-tree failed through a copy of the index)" >&2
    return 1
  fi
  [[ $after == "$tree_before" ]] && return 0
  echo "gate: tree: the working tree changed while the gate ran: a stage wrote these paths (a test regenerating a golden, or writing a file git neither tracks nor ignores), or another hand edited them. Put back what a stage wrote, and move a file a test writes under its scratch or an ignored path:" >&2
  git diff --name-status "$tree_before" "$after" >&2
  return 1
}

run_stage() {
  case $1 in
    fixtures) fixtures ;;
    tree) tree_stage ;;
    build) cargo build --workspace ;;
    artifacts) artifacts ;;
    features) features ;;
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

tree_before=''
[[ " ${selected[*]} " == *' tree '* ]] && tree_before=$(tree_snapshot)

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
