#!/usr/bin/env bash
# docs-check-breakers.sh — proves docs-check.sh refuses what it claims to,
# by breaking a document and watching it refuse: the registry's history
# scan (§2) and the cited commits (§2b), which read a commit written as
# history through one matcher, and the history citations (§2b), each
# path as it was at its commit, the document paths (§6), and the stale
# identifiers (§3) as they read the skills and the glossary. The dependency guard's cases and the group rule's moved with
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

# The review case and the golden case write, and every restore removes,
# one file each: a file already there is someone's, so the suite refuses
# to start.
golden_breaker=crates/acquisition-cli/tests/goldens/seat_faults/zz_breaker.txt
for f in roadmap/reviews/zz-breaker.md "$golden_breaker"; do
  if [[ -e $f ]]; then echo "refusing to start: $f exists, and the suite would delete it"; exit 2; fi
done
scratch=$(mktemp -d "${TMPDIR:-/tmp}/acq-docs-breakers.XXXXXX")
files=(AGENTS.md GLOSSARY.md decisions/plans.md REVIEW-CHECKLIST.md .claude/skills/seat/SKILL.md roadmap/README.md search/README.md)
for f in "${files[@]}"; do mkdir -p "$scratch/keep/$(dirname "$f")"; cp "$f" "$scratch/keep/$f"; done
restore() {
  local f
  for f in "${files[@]}"; do
    cp "$scratch/keep/$f" "$f"
    cmp -s "$scratch/keep/$f" "$f" || { echo "RESTORE FAILED: $f"; exit 2; }
  done
  rm -f roadmap/reviews/zz-breaker.md "$golden_breaker"
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

echo '== the budget and the entry limit note and never fail (the owner'"'"'s ruling of 2026-10-26)'
long_words=$(printf 'plain words %.0s' {1..70})
run_case 'an entry over the length limit'                        pass   'over the entry limit'   entry "$W" "at compile, $long_words. *Why:*"
pad_agents() { printf '\n%s\n' "$(printf 'x%.0s' {1..400})" >>AGENTS.md; }
run_case 'AGENTS.md over its budget'                              pass   'over its budget'        pad_agents

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
run_case 'the path as it is now, not as it was'                  refuse 'search/OBSERVATIONS.md` is not at 453a4429' checklist "The list is \`search/OBSERVATIONS.md\` at \`453a4429\`."
run_case 'the same, written "whole at"'                          refuse 'search/OBSERVATIONS.md` is not at 453a4429' checklist "The list is \`search/OBSERVATIONS.md\` whole at \`453a4429\`."
run_case 'the path as it was there'                              pass   'ok      citations'      checklist "The list is \`spikes/rust-playground/search/OBSERVATIONS.md\` at \`453a4429\`."
# The roadmap is read under the cited commits' two exclusions alone (the
# outside review of 1f074d5a, finding 1): its plan is read, a saved
# review, an outside model's answer verbatim, is not.
roadmap() { printf '%s\n' "$1" >>roadmap/README.md; }
review()  { printf '%s\n' "$1" >roadmap/reviews/zz-breaker.md; }
run_case 'the roadmap'"'"'s plan: the path as it is now'           refuse 'roadmap/README.md:' roadmap "The list is \`search/OBSERVATIONS.md\` at \`453a4429\`."
run_case 'a saved review: not read'                              pass   'ok      citations'      review "The list is \`search/OBSERVATIONS.md\` at \`453a4429\`."

echo '== a document name names a file in this tree (the outside review of 202e4659, finding 2)'
run_case 'a bare deleted document in the review checklist'       refuse 'LIVE-TESTING.md` names no document in this tree' checklist "Read \`LIVE-TESTING.md\` first."
run_case 'the same, cited at the commit that held it'            pass   'ok      paths'          checklist "Read \`LIVE-TESTING.md\` at \`b6197a82\` first."
# A deletion not yet staged: `git ls-files` still lists the file, so a
# pointer to it passed (the outside review of 1f074d5a, finding 2). The
# research-track skill names it; the copy restores it.
unstaged_deletion() { rm search/README.md; }
run_case 'a document deleted, the deletion not staged'           refuse 'search/README.md` names no document in this tree' unstaged_deletion

echo '== a skill names only tests and tools that exist (step 1d put the skills in the scan)'
skill() {  # skill <old> <new>: an exact-once substitution in the seat skill
  python3 - "$1" "$2" <<'PY'
import pathlib, sys
p = pathlib.Path('.claude/skills/seat/SKILL.md'); s = p.read_text()
assert s.count(sys.argv[1]) == 1, 'edit anchor not found once: ' + sys.argv[1][:60]
p.write_text(s.replace(sys.argv[1], sys.argv[2]))
PY
}
# The new names are built here: this file is in the scan's haystack, and
# a name written out in it would be found there.
N=flaws
run_case 'a skill names a test that does not exist'              refuse 'STALE   .claude/skills/seat/SKILL.md' skill 'tests/seat_faults.rs`' "tests/seat_$N.rs\`"
run_case 'a skill names a tool that does not exist'              refuse 'STALE   .claude/skills/seat/SKILL.md' skill '`seat-mcp.py`' "\`seat-mcp-$N.py\`"

glossary() { printf '%s\n' "$1" >>GLOSSARY.md; }
run_case 'the glossary names a tool that does not exist'         refuse 'STALE   GLOSSARY.md' glossary "- **x**: held by \`tools/gate-$N.sh\`."

echo '== a golden is named by its test file (C122)'
orphan() { printf '# an orphan\n' >"$golden_breaker"; }
run_case 'a golden no golden( call names'                        refuse "GOLDEN  $golden_breaker" orphan
run_case 'every golden named'                                    pass   'goldens, each named by its test file' none

echo
printf '%d ok, %d failed\n' "$pass" "$failed"
((failed == 0))
