#!/usr/bin/env bash
# committer-check.sh <before> <after> — CI's report of C186
# (`decisions/process.md`, the committer on `main`): a push to `main`
# that carries a commit whose committer is Claude fails the run. The
# plan offers no server-side rule that refuses such a push, and the
# pre-push hook is skipped by `--no-verify` or a clone without
# `core.hooksPath`; this check cannot refuse the push, only make it a
# red run. The Committer workflow (`.github/workflows/committer.yml`)
# runs it on each push to `main` with the push's `before` and `after`.
#
# The commits read are those after `before` when the clone holds it, and
# otherwise every commit `after` reaches (a new branch's `before` is all
# zeros; a force push's may be gone). The test of a committer is the
# hook's own, `.githooks/claude-committers`. A shallow clone, or a
# history git cannot read, fails the run: it is never passed as clean.
set -uo pipefail
# The repository read is the one the check runs in; the matcher is this
# tree's.
matcher=$(cd "$(dirname "$0")/.." && pwd)/.githooks/claude-committers
if (($# != 2)); then echo "usage: tools/committer-check.sh <before> <after>" >&2; exit 2; fi
before=$1 after=$2
if [[ $(git rev-parse --is-shallow-repository) == true ]]; then
  echo "committer-check: this clone is shallow and cannot read every commit the push carries; check out the whole history (fetch-depth: 0)"
  exit 2
fi
if [[ $before == *[!0]* ]] && git cat-file -e "$before^{commit}" 2>/dev/null; then
  range="$before..$after"
else
  range=$after
fi
found=$("$matcher" "$range")
case $? in
  0) echo "ok      committer    no commit in $range has a Claude committer (C186)"; exit 0 ;;
  1)
    echo "COMMITTER main took a commit whose committer is Claude (C186), which the pre-push hook refuses; it was pushed past the hook:"
    printf '%s\n' "$found"
    echo "main's history now holds it. Rewriting main is the owner's call: its ruleset blocks a force push."
    exit 1 ;;
  *) echo "committer-check: git could not read every commit in $range; nothing is passed as clean"; exit 2 ;;
esac
