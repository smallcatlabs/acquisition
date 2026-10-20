#!/usr/bin/env bash
# docs-check-breakers.sh — proves docs-check.sh refuses what it claims to,
# by breaking a document and watching it refuse: the registry's history
# scan (§2) and the cited commits (§2b), which read a commit written as
# history through one matcher, and the history citations (§2b), each
# path as it was at its commit. The dependency guard's cases and the group rule's moved with
# their checks to `arch-check-breakers.sh` (step 1e of the roadmap); the
# two suites do not cross.
#
# A refusal is evidence only when it names the property, so each case
# states the words the check must print. An edit that does not apply
# fails the suite instead of passing as a refusal. Every case restores
# the files it touched from copies and compares them byte for byte —
# never `git checkout`, which discards uncommitted work — and a trap
# restores on any exit. Run it when a check is added to docs-check.sh,
# not in the gate: it runs the check once per case.
set -uo pipefail
cd "$(dirname "$0")/.."

scratch=$(mktemp -d "${TMPDIR:-/tmp}/acq-docs-breakers.XXXXXX")
files=(decisions/plans.md REVIEW-CHECKLIST.md)
for f in "${files[@]}"; do mkdir -p "$scratch/keep/$(dirname "$f")"; cp "$f" "$scratch/keep/$f"; done
restore() {
  local f
  for f in "${files[@]}"; do
    cp "$scratch/keep/$f" "$f"
    cmp -s "$scratch/keep/$f" "$f" || { echo "RESTORE FAILED: $f"; exit 2; }
  done
}
trap 'restore; rm -rf "$scratch"' EXIT

# ---- the harness -----------------------------------------------------------
pass=0; failed=0
run_case() {  # run_case <name> <refuse|pass> <pattern the output must contain> <setup...>
  local name=$1 want=$2 pattern=$3; shift 3
  local out status
  if ! "$@" 2>"$scratch/setup.err"; then
    printf 'FAIL    %-52s the breaker did not apply: %s\n' "$name" "$(head -c 200 "$scratch/setup.err")"; failed=$((failed+1)); restore; return
  fi
  out=$(tools/docs-check.sh 2>&1); status=$?
  if [[ $want == refuse && $status -ne 0 && $out == *"$pattern"* ]] \
     || [[ $want == pass && $status -eq 0 && $out == *"$pattern"* ]]; then
    printf 'ok      %-52s %s: %s\n' "$name" "$want" "$(grep -m1 -F "$pattern" <<<"$out" | cut -c1-90)"; pass=$((pass+1))
  else
    printf 'FAIL    %-52s wanted %s with "%s"; exit %d, got:\n%s\n' "$name" "$want" "$pattern" "$status" "$(grep -vE '^(ok|note|near) ' <<<"$out" | sed 's/^/          /')"
    failed=$((failed+1))
  fi
  restore
}
none() { :; }

run_case 'clean tree'                                            pass   'none carrying a history marker' none

echo '== the registry: an entry carries no history (the entry rule)'
entry() {  # entry <old> <new>: an exact-once substitution in C77's entry
  python3 - "$1" "$2" <<'PY'
import pathlib, sys
p = pathlib.Path('decisions/plans.md'); s = p.read_text()
line = next(l for l in s.splitlines() if l.startswith('- **C77 '))
assert line.count(sys.argv[1]) == 1, 'edit anchor not found once in C77: ' + sys.argv[1][:60]
p.write_text(s.replace(line, line.replace(sys.argv[1], sys.argv[2])))
PY
}
W='at compile. *Why:*'
HEXWORD=abcdef; HEXWORD+=a   # a hash of hex letters alone, built so the cited-commit scan of tools/ never reads it
run_case 'a date'                                                refuse 'carries a date'          entry "$W" 'at compile, as of 2026-09-03. *Why:*'
run_case 'a commit'                                              refuse 'carries a commit'        entry "$W" 'at compile (at 16bba23e). *Why:*'
run_case 'a commit of hex letters alone'                          refuse 'carries a commit'        entry "$W" "at compile (at \`$HEXWORD\`). *Why:*"
run_case 'an owner quote'                                        refuse 'carries "owner:"'        entry "$W" 'at compile (owner: "agreed"). *Why:*'
run_case 'the word verbatim'                                     refuse 'carries "verbatim"'      entry "$W" 'at compile, quoted verbatim. *Why:*'
run_case 'Amended'                                               refuse 'carries "Amended"'       entry "$W" 'at compile. Amended after the seat. *Why:*'
run_case 'Decided'                                               refuse 'carries "Decided"'       entry "$W" 'at compile. Decided by the session. *Why:*'
run_case 'Ruled'                                                 refuse 'carries "Ruled"'         entry "$W" 'at compile. Ruled at the harvest. *Why:*'
run_case 'a seat call number'                                    refuse 'carries a call number'   entry "$W" 'at compile (cli-opus call 26). *Why:*'
run_case 'a lowercase "ruled first" and a count are no history'  pass   'none carrying a history marker' entry "$W" 'at compile, ruled first over 3,347 tabs. *Why:*'

echo '== a commit written as history: one matcher for both scans (the outside review of 484fce3b, finding 2)'
H7=fedcba; H7+=f     # a hash of letters, and one of nine, built here so the scans of tools/ never read them
H9=fedcba; H9+=fed
checklist() { printf '%s\n' "$1" >>REVIEW-CHECKLIST.md; }   # a line appended to a governed document
run_case 'an entry: at, letters alone, no backticks'             refuse 'carries a commit'        entry "$W" "at compile (at $H7). *Why:*"
run_case 'an entry: at, nine letters in backticks'               refuse 'carries a commit'        entry "$W" "at compile (at \`$H9\`). *Why:*"
run_case 'a document: at, letters alone, no backticks'           refuse "\`$H7\` is not in this history" checklist "A note at $H7."
run_case 'a document: at, nine letters in backticks'             refuse "\`$H9\` is not in this history" checklist "A note at \`$H9\`."

echo '== a history citation names the path as it was at its commit (the governance design'"'"'s second read trial)'
# Built with escaped backticks, so the source never holds a citation the
# scan of tools/ would read.
run_case 'the path as it is now, not as it was'                  refuse '`search/OBSERVATIONS.md` is not at 453a4429' checklist "The list is \`search/OBSERVATIONS.md\` at \`453a4429\`."
run_case 'the same, written "whole at"'                          refuse '`search/OBSERVATIONS.md` is not at 453a4429' checklist "The list is \`search/OBSERVATIONS.md\` whole at \`453a4429\`."
run_case 'the path as it was there'                              pass   'ok      citations'      checklist "The list is \`spikes/rust-playground/search/OBSERVATIONS.md\` at \`453a4429\`."

echo
printf '%d ok, %d failed\n' "$pass" "$failed"
((failed == 0))
