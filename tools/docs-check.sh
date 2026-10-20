#!/usr/bin/env bash
# docs-check.sh — the documentation half of the quality gate.
#
# Six checks, all mechanical (P4 and P5, `AGENTS.md`, "Principles": a
# rule that can be broken silently becomes a lint):
#
#   1. One byte budget, on the one file every session loads, `AGENTS.md`.
#      Growth past it is the signal that a narrative landed where a
#      ruling, a test or a commit message belongs (`AGENTS.md`, "Where
#      things live"). Moving text to its home is compliance, not gaming.
#      Past 90% the check says so without failing, so routing happens at
#      a session close and never as a side quest in the middle of a step.
#      Every other document is read on demand and has no budget.
#   2. The decision registry: one bullet per decision under a length
#      limit and with no history in it, a capped count of cross-cutting
#      ones in `AGENTS.md`, every cited id real, the uncited ones
#      reported.
#  2b. Cited commits: a hash in the governed files is a commit of this
#      history, and a history citation, `<path>` at `<hash>`, names the
#      path as it was at that commit.
#   3. Stale identifiers. A backticked code identifier in a control
#      document that no longer exists in the workspace is a parallel
#      description that has rotted. Checked: `Type::path` items,
#      CamelCase types, snake_case names with an underscore, ACQ_* knobs,
#      and *.rs / *.sh / *.py / *.sql file names.
#   4. The README's form: one line per verb in the tour, a row per knob.
#   5. The clone's hooks: `core.hooksPath` names `.githooks`, whose
#      commit-msg hook writes Claude's trailer as `Assisted-by:`. A note,
#      never a failure: a clone that only builds (CI) needs no hook, and
#      the setting is the clone's, so `AGENTS.md` says to make it once.
#   6. Document paths: a backticked document name in the governed files
#      names a file in this tree, or is cited at its commit.
#
# Exit 1 on any failure; the report names each offender.
set -euo pipefail
cd "$(dirname "$0")/.."

fail=0

# ---- 1. budgets (bytes) ---------------------------------------------------
budget() {
  local file=$1 limit=$2 size
  size=$(wc -c <"$file" | tr -d ' ')
  if ((size > limit)); then
    printf 'BUDGET  %-18s %7d > %7d bytes\n' "$file" "$size" "$limit"
    fail=1
  elif ((size * 10 > limit * 9)); then
    printf 'near    %-18s %7d / %7d bytes — over 90%%: route at session close, before a slice trips it\n' "$file" "$size" "$limit"
  else
    printf 'ok      %-18s %7d / %7d bytes\n' "$file" "$size" "$limit"
  fi
}
budget AGENTS.md 14000

# ---- 2. the decision registry ------------------------------------------
# Every decision is one bullet under a length limit (a narrative cannot fit,
# so the mechanism goes to the code); every `C<n>` cited anywhere exists in
# the registry; a decision nothing cites is reported (enforced by nothing is
# either a lint, a test, or a smell). "Anywhere" is the code, the tools, the
# skills, the control documents, the review checklist, the history and the
# roadmap — and never the registry's own cross-references, which are not
# a test, a doc or a tool naming the decision. Two kinds of roadmap file
# are read by neither this scan nor 2b's, being text no session may
# correct: a saved review (`roadmap/reviews/`, an outside model's answer
# verbatim, deleted at its phase's close) and the dated assessment
# (`roadmap/STATE-*.md`, corrected only when an outside review shows a
# sentence wrong).
SKIP_ROADMAP=(--exclude-dir=reviews --exclude='STATE-*.md')
# One matcher for a commit written as history, read by both scans that
# look for commits (the history scan below, the cited commits in 2b):
# "at", then 7 to 40 hex with or without backticks, letters alone
# included. Each scan had its own spelling, and an outside review of
# `484fce3b` found an unbackticked hash, and one of nine letters, that
# escaped both.
AT_HASH='\bat `?[0-9a-f]{7,40}\b`?'
ENTRY_LIMIT=800
reg=$(mktemp)
# entries are single lines in the registry (one bullet, no continuation),
# spread over AGENTS.md (cross-cutting only) and decisions/*.md (per area)
grep -hE '^- \*\*C[0-9]+ ' AGENTS.md decisions/*.md >"$reg"
dups=$(grep -oE '^- \*\*C[0-9]+' "$reg" | sort | uniq -d | sed 's/^- \*\*//' | tr '\n' ' ')
if [[ -n $dups ]]; then echo "DUPLICATE decision id across registry files: $dups"; fail=1; fi
CROSS_LIMIT=15
cross=$(grep -cE '^- \*\*C[0-9]+ ' AGENTS.md)
if ((cross > CROSS_LIMIT)); then
  printf 'CROSS   %-18s %5d always-loaded decisions > %d — move area rulings to decisions/<area>.md\n' AGENTS.md "$cross" "$CROSS_LIMIT"; fail=1
else
  printf 'ok      %-18s %5d always-loaded decisions (limit %d)\n' AGENTS.md "$cross" "$CROSS_LIMIT"
fi
over=0
while IFS= read -r line; do
  n=$(printf '%s' "$line" | wc -c | tr -d ' ')
  if ((n > ENTRY_LIMIT)); then
    printf 'ENTRY   %-18s %5d > %d bytes  %s\n' registry "$n" "$ENTRY_LIMIT" "$(printf '%s' "$line" | cut -c1-40)"
    over=$((over+1))
  fi
done <"$reg"
ids=$(grep -oE '^- \*\*C[0-9]+' "$reg" | sed 's/^- \*\*//' | sort -u)
count=$(printf '%s\n' "$ids" | grep -c .)
if ((over > 0)); then fail=1; else printf 'ok      %-18s %5d decisions, every entry within %d bytes\n' registry "$count" "$ENTRY_LIMIT"; fi
# An entry carries no history (the entry rule, `tools/bothy.py`'s
# docstring): no date, no commit, no owner's quote, no "Amended",
# "Decided" or "Ruled", no seat call number. `git log -S'C<n>'` and the
# commit messages hold the story; a marker is refused by name, with the
# entry that carries it. The words are matched as written, so a ruling
# may still say a thing is "ruled first". A bare word of hex letters
# alone is no commit; one written as history (`AT_HASH`, above) is.
history=$(python3 -I - "$reg" "$AT_HASH" <<'PY'
import re, sys
MARKERS = [
    ("a date", r"\b\d{4}-\d{2}-\d{2}\b"),
    ("a commit", r"\b(?=[0-9a-f]*[0-9])[0-9a-f]{7,40}\b|" + sys.argv[2]),
    ('"owner:"', r"(?i:\bowner:)"),
    ('"verbatim"', r"(?i:\bverbatim\b)"),
    ('"Amended"', r"\bAmended\b"),
    ('"Decided"', r"\bDecided\b"),
    ('"Ruled"', r"\bRuled\b"),
    ("a call number", r"\bcalls? \d+"),
]
for line in open(sys.argv[1], encoding="utf-8"):
    cid = re.match(r"- \*\*(C\d+)", line).group(1)
    for name, rx in MARKERS:
        m = re.search(rx, line)
        if m:
            print(f"HISTORY {cid:<18} carries {name} ({m.group(0)!r}): git holds the story; take it out of the entry")
PY
) || { echo "HISTORY the scan itself failed"; fail=1; }
if [[ -n $history ]]; then printf '%s\n' "$history"; fail=1; else printf 'ok      %-18s %5d decisions, none carrying a history marker\n' registry "$count"; fi
# AGENTS.md is read without its entry lines: it holds the cross-cutting
# entries, and an entry is not a citation of itself.
cited=$( { grep -rhoE '\bC[0-9]+\b' crates tools README.md search/DIGEST.md RUN-LEDGER.md TESTING-NOTES.md SURFACES.md search/MEASUREMENTS.md search/LEDGER.md REVIEW-CHECKLIST.md HISTORY.md roadmap .claude/skills 2>/dev/null \
  --include='*.rs' --include='*.sh' --include='*.py' --include='*.md' "${SKIP_ROADMAP[@]}"
  grep -vE '^- \*\*C[0-9]+ ' AGENTS.md | grep -oE '\bC[0-9]+\b' || true; } | sort -u)
unknown=$(comm -13 <(printf '%s\n' "$ids") <(printf '%s\n' "$cited") | grep . || true)
if [[ -n $unknown ]]; then
  printf 'UNKNOWN decision id cited outside the registry: %s\n' "$unknown" | tr '\n' ' '; echo
  fail=1
fi
uncited=$(comm -23 <(printf '%s\n' "$ids") <(printf '%s\n' "$cited") | tr '\n' ' ')
[[ -n $uncited ]] && printf 'note    uncited decisions (no test, doc, or tool names them): %s\n' "$uncited"
rm -f "$reg"

# ---- 2b. cited commits ----------------------------------------------------
# A backticked 7-8 hex token in the governed documents, and a hash
# written as history (`AT_HASH`, above), names a commit of this history
# — an ancestor of HEAD — or carries its repository as `<repo>@<hash>`
# (the search tracks' form for the RePoE clones). A hash
# that resolves only because another branch is fetched dies in a clone
# of this branch: five did on 2026-10-11, master-side twins of
# cherry-picked ground-truth commits and one review-branch commit.
#
# The governed files, one scope for the document scans: the roots below,
# their *.rs, *.sh, *.py and *.md files. The cited commits here and the
# history citations after them read the roadmap under SKIP_ROADMAP's two
# exclusions alone (its saved reviews, its dated assessment), as the
# registry's citations do; the document paths (6) read no roadmap file.
governed=(crates tools README.md search/DIGEST.md RUN-LEDGER.md TESTING-NOTES.md SURFACES.md search/MEASUREMENTS.md search/LEDGER.md AGENTS.md decisions .claude/skills REVIEW-CHECKLIST.md HISTORY.md roadmap)
revs=$(git rev-list HEAD)
dead=0; checked=0
while read -r h; do
  [[ -n $h ]] || continue
  checked=$((checked+1))
  if ! grep -q "^$h" <<<"$revs"; then
    where=$(grep -rlE --include='*.rs' --include='*.sh' --include='*.py' --include='*.md' "${SKIP_ROADMAP[@]}" -- "\`$h\`|\bat \`?$h\b" "${governed[@]}" 2>/dev/null | head -1 || true)
    printf 'COMMIT  %-18s `%s` is not in this history — cite the twin that is, or <repo>@%s\n' "$where" "$h" "$h"
    dead=$((dead+1))
  fi
done < <( { grep -rhoE '`[0-9a-f]{7,8}`' "${governed[@]}" --include='*.rs' --include='*.sh' --include='*.py' --include='*.md' "${SKIP_ROADMAP[@]}" 2>/dev/null
            grep -rhoE "$AT_HASH" "${governed[@]}" --include='*.rs' --include='*.sh' --include='*.py' --include='*.md' "${SKIP_ROADMAP[@]}" 2>/dev/null | sed 's/^at //'
          } | tr -d '`' | sort -u)
if ((dead > 0)); then fail=1; else printf 'ok      %-18s %5d cited commits, every one in this history\n' commits "$checked"; fi
# Every history citation, `<path>` at `<hash>` (or "whole at"), names
# the path as it was at that commit: `git cat-file` must find it there.
# The governance design's second read trial found nine of twelve naming
# the path as it is now, which the cited commit never held. A token with
# no slash and no extension is a name, not a path, and is not read. The
# roadmap is read under SKIP_ROADMAP's exclusions, as the cited commits
# are (the outside review of `1f074d5a`, finding 1).
if ! cites=$(python3 -I - "$AT_HASH" "${governed[@]}" <<'PY'
import os, re, subprocess, sys
at, roots = sys.argv[1], sys.argv[2:]
LEAD = r"(?:\s|//[!/]?|#)*"   # a line break, and a comment's leader, may fall between
cite = re.compile(r"`([^`\s]+)`,?" + LEAD + r"(?:whole" + LEAD + r")?(" + at + ")")
def files():   # SKIP_ROADMAP's exclusions, as grep applies them: a directory named reviews, a file STATE-*.md
    for r in roots:
        if os.path.isfile(r): yield r; continue
        for d, ds, fs in os.walk(r):
            ds[:] = sorted(x for x in ds if x not in ("target", "reviews"))
            yield from (os.path.join(d, f) for f in sorted(fs)
                        if f.endswith((".rs", ".sh", ".py", ".md")) and not re.fullmatch(r"STATE-.*\.md", f))
found = []
for f in files():
    text = open(f, encoding="utf-8").read()
    for m in cite.finditer(text):
        path, h = m.group(1).rstrip("/"), m.group(2)[3:].strip("`")
        if "/" in path or re.search(r"\.\w+$", path):
            found.append((f, text.count("\n", 0, m.start()) + 1, path, h))
answer = subprocess.run(["git", "cat-file", "--batch-check"], input="".join(f"{h}:{p}\n" for _, _, p, h in found),
                        capture_output=True, text=True, check=True).stdout.splitlines()
assert len(answer) == len(found), "git cat-file answered %d of %d" % (len(answer), len(found))
bad = [c for c, a in zip(found, answer) if not re.fullmatch(r"[0-9a-f]{40} \w+ \d+", a)]
for f, line, path, h in bad:
    print(f"CITE    {f}:{line} `{path}` is not at {h}: name the path as it was there")
if not bad:
    print(f"ok      {'citations':<18} {len(found):5d} history citations, each path there at its commit")
sys.exit(1 if bad else 0)
PY
); then fail=1; fi
printf '%s\n' "$cites"

# ---- 3. stale identifiers -----------------------------------------------
# The haystack is the code and its schemas, and the skills' own scripts
# (the seat's drivers live only under `.claude/skills`); docs never vouch for
# docs. The checked documents are the ones a session reads to act: the
# entry, the README, the review checklist (its "Held by" column names
# tests and mechanisms) and the skills.
hay=$(mktemp)
trap 'rm -f "$hay"' EXIT
find crates tools .claude/skills -type f \( -name '*.rs' -o -name '*.sh' -o -name '*.py' -o -name '*.sql' -o -name '*.toml' \) \
  -not -path '*/target/*' -print0 | xargs -0 cat >"$hay"
ls -R crates tools .claude/skills >>"$hay"

missing=0
for doc in AGENTS.md README.md REVIEW-CHECKLIST.md .claude/skills/*/SKILL.md; do
  # backticked tokens without spaces, then the shapes worth checking
  grep -o '`[^` ]\{3,\}`' "$doc" | tr -d '`' | sort -u | while read -r tok; do
    case "$tok" in
      *::*)                      needle=${tok##*::} ;;                # Store::record → record
      *.rs|*.sh|*.py|*.sql)      needle=$(basename "$tok") ;;
      ACQ_*)                     needle=${tok%%=*} ;;
      *_*)                       [[ $tok =~ ^[A-Za-z_][A-Za-z0-9_]*$ ]] || continue; needle=$tok ;;
      *)                         [[ $tok =~ ^[A-Z][a-z]+[A-Z][A-Za-z0-9]*$ ]] || continue; needle=$tok ;;
    esac
    needle=${needle%\(\)}
    # observed GGG data values quoted in rulings, not code
    case "$needle" in BodyArmour|DefaultAttackSkills|SkillSlots) continue ;; esac
    [[ -n $needle ]] || continue
    if ! grep -qF -- "$needle" "$hay"; then
      printf 'STALE   %-18s `%s`\n' "$doc" "$tok"
      echo x >>"$hay.miss"
    fi
  done
done
if [[ -f "$hay.miss" ]]; then
  missing=$(wc -l <"$hay.miss" | tr -d ' ')
  rm -f "$hay.miss"
fi
if ((missing > 0)); then
  echo "$missing stale identifier(s)"
  fail=1
else
  echo "ok      identifiers   every checked identifier exists in the workspace"
fi

# ---- 4. the README's form -----------------------------------------------
# The README is the index to what exists and how to reach it
# (note 13 at decda84e): the tour is one line per verb — a
# comment-only line is a group header after a blank line, never a
# continuation of a verb line (44 of those had accreted by 2026-09-07) —
# and the knob table is complete: every ACQ_* the crates read outside
# tests has a row. The tour's verbs and flags are pinned by
# acquisition-cli/tests/readme_tour.rs against the binary.
#
# Outside tests means the code a build ships: not a crate's `tests/`,
# not an item a column-0 `#[cfg(test)]` guards in `src/` (a `mod …;`
# line, or a block to its closing column-0 `}`), and not `tools/`. A
# knob read only there is development's, and its read site's doc is its
# home; the README is the users' (2026-10-19, the release seat's fix
# round, when three such rows left it).
tour_bad=$(awk '/^```sh/{f=1; prev=""; next} /^```/{f=0; next} f{ if ($0 ~ /^[[:space:]]*#/ && prev !~ /^[[:space:]]*$/) print "        line "NR": "$0; prev=$0 }' README.md)
if [[ -n $tour_bad ]]; then
  printf 'TOUR    %-18s a comment line continues a verb line — one line per verb, the rest is its --help:\n%s\n' README.md "$tour_bad"
  fail=1
else
  echo "ok      tour          one line per verb"
fi
code_knobs=$(find crates -name '*.rs' -not -path '*/tests/*' -not -path '*/target/*' -exec awk '
    FNR == 1 { skip = 0; attr = 0 }
    attr { attr = 0; if ($0 ~ /;[[:space:]]*$/) next; skip = 1; next }
    /^#\[cfg\(test\)\]/ { attr = 1; next }
    skip { if ($0 ~ /^}/) skip = 0; next }
    { print }' {} + \
  | grep -oE '"ACQ_[A-Z_]+"' | tr -d '"' | sort -u \
  | grep -vE '^ACQ_(CONTRACT_REVISION|SEARCH_BUILD)$')
readme_knobs=$(grep -oE 'ACQ_[A-Z_]+' README.md | sort -u)
missing_knobs=$(comm -23 <(printf '%s\n' "$code_knobs") <(printf '%s\n' "$readme_knobs") | tr '\n' ' ')
if [[ -n $missing_knobs ]]; then
  printf 'KNOB    %-18s read in the crates, no row in the knob table: %s\n' README.md "$missing_knobs"
  fail=1
else
  echo "ok      knobs         every ACQ_* the crates read has a README row"
fi

# ---- 5. the clone's hooks --------------------------------------------------
# Unset, git reads the empty .git/hooks and a commit keeps the trailer it
# was written with: two of step 2's commits did (2026-10-15).
if [[ $(git config --get core.hooksPath || true) == .githooks ]]; then
  printf 'ok      %-13s core.hooksPath is .githooks: the commit-msg hook writes the trailer\n' hooks
else
  echo "note    hooks         core.hooksPath is not .githooks, so the commit-msg hook does not run: git config core.hooksPath .githooks (AGENTS.md)"
fi

# ---- 6. document paths -----------------------------------------------------
# A backticked *.md name in the governed files names a document that
# exists — from the root, from the citing file's directory, or as the
# tail of a tracked path — unless it is written as history (`<path>` at
# `<hash>`, whose path 2b holds to that commit). The identifier scan
# reads code names and skips a document's: step 1d's brief named a
# deleted document in the review checklist as its failing input, and
# nothing refused it (the outside review of `202e4659`, finding 2). Not
# read: the roadmap, whose ledger and briefs tell deletions as their
# story (about eighty such names), and the search slice's ledger and
# digest, whose heads give the commit each of their pointers is read at. Three names are of no file in this tree, by design: RePoE's
# licence, a seat run's debrief, and a live run's friction notes.
if ! paths=$(python3 -I - "$AT_HASH" "${governed[@]}" <<'PY'
import os, re, subprocess, sys
at, roots = sys.argv[1], sys.argv[2:]
LEAD = r"(?:\s|//[!/]?|#)*"   # as 2b's citations
tok = re.compile(r"`([^`\s:]+\.md)`")
history = re.compile(r",?" + LEAD + r"(?:whole" + LEAD + r")?" + at)
DATED = {"search/LEDGER.md", "search/DIGEST.md"}
NOT_HERE = {"LICENSE.md", "DEBRIEF.md", "friction.md"}
# Tracked and present: `git ls-files` still lists a deletion not yet
# staged (the outside review of `1f074d5a`, finding 2).
tracked = [t for t in subprocess.run(["git", "ls-files"], capture_output=True, text=True, check=True).stdout.splitlines()
           if os.path.exists(t)]
tails = {"/".join(t.split("/")[i:]) for t in tracked for i in range(t.count("/") + 1)}
def files():
    for r in roots:
        if r == "roadmap" or r in DATED: continue
        if os.path.isfile(r): yield r; continue
        for d, ds, fs in os.walk(r):
            ds[:] = sorted(x for x in ds if x != "target")
            yield from (os.path.join(d, f) for f in sorted(fs) if f.endswith((".rs", ".sh", ".py", ".md")))
n = 0; bad = []
for f in files():
    text = open(f, encoding="utf-8").read()
    for m in tok.finditer(text):
        name = m.group(1)
        if history.match(text, m.end()) or re.search(r"[<>*{}$]", name) or name in NOT_HERE: continue
        n += 1
        if not (os.path.exists(name) or os.path.exists(os.path.join(os.path.dirname(f), name)) or name in tails):
            bad.append(f"PATH    {f}:{text.count(chr(10), 0, m.start()) + 1} `{name}` names no document in this tree: point at its home, or cite it as `<path>` at `<commit>`")
print("\n".join(bad) if bad else f"ok      {'paths':<13} {n} document names, each one in this tree or cited at its commit")
sys.exit(1 if bad else 0)
PY
); then fail=1; fi
printf '%s\n' "$paths"

exit $fail
