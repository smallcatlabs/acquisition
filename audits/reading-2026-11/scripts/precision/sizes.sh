#!/bin/zsh
# usage: sizes.sh <commit> <area>...   prints bytes and entry count of decisions/<area>.md at commit^, plus checklist bytes/rows
c=$1; shift
for a in "$@"; do
  f=decisions/$a.md
  b=$(git show $c^:$f | wc -c | tr -d ' ')
  n=$(git show $c^:$f | grep -cE '^- \*\*C[0-9]+')
  echo "$c $f bytes=$b entries=$n"
done
for x in AGENTS.md roadmap/README.md decisions/process.md REVIEW-CHECKLIST.md; do
  echo "$c $x bytes=$(git show $c^:$x | wc -c | tr -d ' ') entries=$(git show $c^:$x | grep -cE '^- \*\*C[0-9]+')"
done
git show $c^:REVIEW-CHECKLIST.md | awk '/^## /{s=$0} /^- /{n[s]++} {b[s]+=length($0)+1} END{for(k in n) print "  CK", k, "rows="n[k], "bytes="b[k]}'
