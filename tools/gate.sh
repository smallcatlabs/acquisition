#!/usr/bin/env bash
# gate.sh — the quality gate (`AGENTS.md`, "Quality gate") as one command.
#
#   tools/gate.sh                 every stage, in the gate's order
#   tools/gate.sh --docs          the five stages that build no Rust, for a
#                                 change no test or build reads, which the
#                                 session establishes; when unsure, the
#                                 full gate
#   tools/gate.sh --only <stage>  one stage, by the name in the table, or
#                                 doc, which CI runs
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
# The `winclippy` stage is clippy for the Windows target,
# `x86_64-pc-windows-msvc`, on whatever host runs the gate (the owner's
# ruling of 2026-10-29, `roadmap/README.md`): it lints the `cfg(windows)`
# code, which the host's `clippy` never compiles, so a Windows-only lint
# is seen before a push and not first in CI. It sees lints only: no test
# of that code runs, and nothing links. The C and C++ compilers and the
# archiver a build script calls for the target are `tools/windows-cc-stub.sh`,
# which writes each output it is named as an empty file; a stage that
# linked through it would be wrong by design, and clippy links nothing.
# On a Windows host (`OS` is `Windows_NT` in every process there, Git
# Bash's included) the stage names no stub: cargo calls the native MSVC
# tools, and could not execute a shell script as a compiler (the review
# of the step's build, finding 1).
# It builds into its own target directory, `target/winclippy`, so the
# host's executables, which `artifacts` hashes, are never touched, and a
# later run is warm (42 s cold, 2 s warm, measured on a 10-core M4). Where
# the target's standard library is not installed the stage fails and
# names `rustup target add`, never passing as a stage that did not run.
# CI does not run it: the Windows gate runs clippy natively, which sees
# the same lints and links.
#
# The script unsets `ACQ_UPDATE_FIXTURES` before any stage of any
# selection, so no stage rewrites a golden: every golden regenerates when
# it is set at all, `0` and empty among its values, and the harness
# removes `ACQ_*` from the children a test spawns, never from the test
# that compares, so a gate run in a shell that exports it rewrote every
# golden and went green (#87).
#
# The doc stage, `cargo doc` with warnings denied, is not in the full
# list: 80 s after any edit, serial, and the Linux gate's `doc` job runs
# it on every push (the owner's ruling of 2026-10-29, `roadmap/README.md`).
# `--only doc` runs it here.
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

unset ACQ_UPDATE_FIXTURES

names=(build test clippy winclippy fmt diff docs arch trade bothy features artifacts tree)
ONLY=(doc)   # selectable by --only, never in the full list
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

# The `features` stage (the header). The targets are the release's: every
# target triple `release.yml` names, anywhere in the file and in any YAML
# form, read as a token of a triple's shape (`<arch>-<vendor>-<os>[-<env>]`,
# the vendor `apple`, `pc` or `unknown`) over the file's whole text, so no
# structure is parsed and none can hide one. A triple named twice, or in a
# comment, is checked once more for nothing; one never named is not a
# release target. A file naming none fails the stage. A hand-written reader
# of the matrix answered for coverage and each review found the next shape
# it skipped, block rows, then an axis beside `include` (the reviews of
# #91). The graphs are the workspace's, as the build and the test run are:
# a pin in one binary's crate unifies into every binary of the build, so
# `acq`'s rustix pin also closes the split Linux's `acqd` had through
# keyring's zbus (#91's Do 1), which `-p acquisition-daemon` alone still
# shows and no stage builds.
RELEASE=.github/workflows/release.yml

# The release's targets, one a line, sorted and each once.
release_targets() {
  grep -oE '\b[a-z0-9_]+-(apple|pc|unknown)-[a-z0-9]+(-[a-z0-9]+)?\b' "$RELEASE" | sort -u
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
  targets=$(release_targets)
  [[ -n $targets ]] || { echo "gate: features: $RELEASE names no target triple, so the release's targets are unknown and nothing would be checked" >&2; return 1; }
  echo "gate: features: the release's $(wc -l <<<"$targets" | tr -d ' ') targets: "$targets
  for t in $targets; do
    out=$(feature_split "$t")
    case $? in
      0) echo "gate: features: $t: the build and the test build agree" ;;
      1) echo "gate: features: $t: a test build compiles these packages with other features than the build, so a test run writes another binary (C84, the artifact mismatch); pin the features in the workspace's feature pins, the one block in crates/acquisition-cli/Cargo.toml:" >&2
         echo "$out" >&2; rc=1 ;;
      *) echo "gate: features: $t: cargo tree could not read the feature graphs (its words above): no comparison was made, and nothing here says a pin is due" >&2; rc=1 ;;
    esac
  done
  return "$rc"
}

# The `winclippy` stage (the header).
WINDOWS_TARGET=x86_64-pc-windows-msvc
winclippy() {
  local installed stub triple stubs=()
  if ! installed=$(rustup target list --installed 2>/dev/null) || ! grep -qx "$WINDOWS_TARGET" <<<"$installed"; then
    echo "gate: winclippy: the $WINDOWS_TARGET target is not installed for this toolchain, so the Windows lints cannot run: rustup target add $WINDOWS_TARGET" >&2
    return 1
  fi
  stub=$PWD/tools/windows-cc-stub.sh
  triple=${WINDOWS_TARGET//-/_}
  [[ ${OS-} == Windows_NT ]] || stubs=("CC_$triple=$stub" "CXX_$triple=$stub" "AR_$triple=$stub")
  env ${stubs[@]+"${stubs[@]}"} CARGO_TARGET_DIR=target/winclippy \
    cargo clippy --keep-going --workspace --all-targets --all-features --target "$WINDOWS_TARGET" -- -D warnings
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
    tree) tree_stage ;;
    build) cargo build --workspace ;;
    artifacts) artifacts ;;
    features) features ;;
    test) cargo nextest run --workspace --all-targets ;;
    clippy) cargo clippy --workspace --all-targets --all-features -- -D warnings ;;
    winclippy) winclippy ;;
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

usage() { sed -n '4,10p' "$0" | sed 's/^# \{0,1\}//'; }

selected=()
case ${1:-} in
  '') selected=("${names[@]}") ;;
  --docs) selected=("${DOCS[@]}") ;;
  --only)
    for name in "${names[@]}" "${ONLY[@]}"; do [[ ${2:-} == "$name" ]] && selected=("$2"); done
    if ((${#selected[@]} == 0)); then
      echo "gate.sh: --only takes one of: ${names[*]} ${ONLY[*]}" >&2; exit 2
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
