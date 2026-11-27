#!/bin/zsh
# usage: packall.zsh "<commits>" "<engaged ids>"
P=/tmp/reading-2026-11/precision
commits=($=1); eng=($=2)
files=($(for c in $commits; do git show --name-only --format= $c; done | sort -u | grep -E '\.(rs|js|css|sh|toml)$' | while read f; do [ -e "$f" ] && echo $f; done))
echo "files packed: ${#files}"
python3 -I $P/pack.py $files > $P/last.txt
head -1 $P/last.txt
w=$(sed -n 2p $P/last.txt); h=$(sed -n 3p $P/last.txt)
for i in $eng; do
  if echo " $w " | grep -qE " $i\*? "; then s=whole; elif echo " $h " | grep -q " $i "; then s=HEADLINE; else s=absent; fi
  echo "  $i $s"
done
echo "  whole count: $(echo $w | awk '{print $2}') headline count: $(echo $h | awk '{print $2}')"
