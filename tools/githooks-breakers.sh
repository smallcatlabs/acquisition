#!/usr/bin/env bash
# githooks-breakers.sh — proves the commit hooks under `.githooks/` hold
# C184 (the commit trailer), each case in a fresh scratch repository whose
# `core.hooksPath` is this tree's `.githooks`. In an agent's shell
# (`CLAUDECODE` set, at any value) a commit, a merge or a revert whose
# message names no assistant is refused, the refusal prints the line to
# add, and nothing is committed; Claude's `Co-Authored-By:` line is
# written as `Assisted-by:`. In a person's shell (`CLAUDECODE` unset) the
# hooks only rewrite that line, and a commit naming no assistant lands.
#
# git runs `commit-msg` for a commit and a merge but never for a revert
# or a cherry-pick, which `prepare-commit-msg` holds, so the revert's
# cases are what prove that hook. A refusal is evidence only when it
# names the property, so each refused case requires the printed line and
# an unmoved `HEAD`. The user's own git configuration is not read
# (`GIT_CONFIG_GLOBAL`), so a signing key or a global hooks path changes
# nothing here. Run it when a hook changes; CI runs it beside the gate.
set -uo pipefail
hooks=$(cd "$(dirname "$0")/../.githooks" && pwd)
scratch=$(mktemp -d "${TMPDIR:-/tmp}/acq-githooks-breakers.XXXXXX")
trap 'rm -rf "$scratch"' EXIT
export GIT_CONFIG_GLOBAL=$scratch/gitconfig GIT_CONFIG_NOSYSTEM=1
: >"$GIT_CONFIG_GLOBAL"
unset CLAUDECODE GIT_EDITOR   # each case sets what it needs

LINE='Co-Authored-By: Claude <model> <noreply@anthropic.com>'   # what a refusal prints
CLAUDE='Co-Authored-By: Claude Breaker 1.0 <noreply@anthropic.com>'
WRITTEN='Assisted-by: Claude Breaker 1.0 <noreply@anthropic.com>'
HUMAN='Co-Authored-By: A Person <person@example.com>'

# A repository with a base commit on main and one on `side`, made in a
# person's shell; the case runs in it.
n=0
fresh() {
  n=$((n+1)); repo=$scratch/r$n
  git init -q -b main "$repo" && cd "$repo" &&
    git config user.name breaker && git config user.email breaker@example.com &&
    git config core.hooksPath "$hooks" &&
    echo base >f && git add f && git commit -qm base &&
    git checkout -qb side && echo side >g && git add g && git commit -qm side &&
    git checkout -q main && echo main >>f && git commit -qam main
}

# ---- the harness -----------------------------------------------------------
pass=0; failed=0
clip() { python3 -Ic 'import sys; print(sys.stdin.readline().rstrip("\n")[:90])'; }
# run_case <name> <refused|landed> <who: agent, agent-empty, person> <shell command> [<line the message must hold>...]
# A commit's command makes its own change first: the fixture leaves none.
run_case() {
  local name=$1 want=$2 who=$3 cmd=$4; shift 4
  local out status before after msg wrong=''
  if ! fresh >"$scratch/setup.err" 2>&1; then
    printf 'FAIL    %-52s the scratch repository was not made: %s\n' "$name" "$(head -c 200 "$scratch/setup.err")"; failed=$((failed+1)); return
  fi
  before=$(git rev-parse HEAD)
  case $who in
    agent)       out=$(CLAUDECODE=1 bash -c "$cmd" 2>&1); status=$? ;;
    agent-empty) out=$(CLAUDECODE= bash -c "$cmd" 2>&1); status=$? ;;
    person)      out=$(bash -c "$cmd" 2>&1); status=$? ;;
  esac
  after=$(git rev-parse HEAD); msg=$(git log -1 --format=%B)
  if [[ $want == refused ]]; then
    ((status != 0)) || wrong+=' it exited 0;'
    [[ $after == "$before" ]] || wrong+=' HEAD moved;'
    [[ $out == *"$LINE"* ]] || wrong+=' the refusal did not print the line;'
  else
    ((status == 0)) || wrong+=" it exited $status;"
    [[ $after != "$before" ]] || wrong+=' HEAD did not move;'
    for l in "$@"; do grep -qxF -- "$l" <<<"$msg" || wrong+=" the message lacks '$l';"; done
    ! grep -q '^Co-Authored-By: Claude' <<<"$msg" || wrong+=" Claude's Co-Authored-By line is left;"
  fi
  if [[ -z $wrong ]]; then
    printf 'ok      %-52s %s: %s\n' "$name" "$want" "$( ([[ $want == refused ]] && printf '%s\n' "$out" || printf '%s\n' "$msg") | clip)"; pass=$((pass+1))
  else
    printf 'FAIL    %-52s wanted %s, but%s exit %d, got:\n%s\n' "$name" "$want" "$wrong" "$status" "$(sed 's/^/          /' <<<"$out")"
    failed=$((failed+1))
  fi
}
nl=$'\n'

echo '== an agent'"'"'s shell: a commit naming no assistant is refused (C184)'
run_case 'a commit -m'                                    refused agent       'echo x >>f && git commit -qam "a change"'
run_case 'a commit written in an editor'                  refused agent       'echo x >>f && GIT_EDITOR="f() { printf \"by an editor\n\" >\"\$1\"; }; f" git commit -qa'
run_case 'a commit with --no-verify'                      refused agent       'echo x >>f && git commit -q --no-verify -am "a change"'
run_case 'CLAUDECODE set empty'                           refused agent-empty 'echo x >>f && git commit -qam "a change"'
run_case 'a merge'                                        refused agent       'git merge -q --no-ff -m "merge side" side'
run_case 'a revert'                                       refused agent       'git revert --no-edit HEAD'
run_case 'a cherry-pick'                                  refused agent       'git cherry-pick side'

echo '== an agent'"'"'s shell: the line is written as Assisted-by:'
run_case 'a commit -m with the line'                      landed  agent       "echo x >>f && git commit -qam 'a change${nl}${nl}${CLAUDE}'" "$WRITTEN"
run_case 'a merge with the line'                          landed  agent       "git merge -q --no-ff -m 'merge side${nl}${nl}${CLAUDE}' side" "$WRITTEN"
run_case 'a revert by --no-commit, then the line'         landed  agent       "git revert --no-commit HEAD && git commit -qm 'Revert main${nl}${nl}${CLAUDE}'" "$WRITTEN"

echo '== a person'"'"'s shell: the hooks only rewrite'
run_case 'a commit naming no assistant'                   landed  person      'echo x >>f && git commit -qam "a change"' 'a change'
run_case 'a human co-author kept, Claude'"'"'s line rewritten'  landed  person      "echo x >>f && git commit -qam 'a change${nl}${nl}${HUMAN}${nl}${CLAUDE}'" "$HUMAN" "$WRITTEN"
run_case 'a revert naming no assistant'                   landed  person      'git revert --no-edit HEAD'

echo
printf '%d ok, %d failed\n' "$pass" "$failed"
((failed == 0))
