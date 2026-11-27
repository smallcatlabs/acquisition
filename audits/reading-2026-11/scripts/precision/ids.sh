#!/bin/zsh
# usage: ids.sh <name> <first-commit> "<all commits>" "<extra files: briefs, reviews>"
name=$1; first=$2; commits=($=3); files=($=4)
ids(){ grep -oE '\bC[0-9]{1,3}\b' | sort -u -V | tr '\n' ' '; }
echo "== $name"
echo -n " msgs: "; for c in $commits; do git log -1 --format=%B $c; done | ids; echo
echo -n " diff+: "; for c in $commits; do git show --format= $c | grep '^+' ; done | ids; echo
echo -n " diff ctx(all lines of touched hunks): "; for c in $commits; do git show --format= -U0 $c | grep -E '^[-+ ]' ; done | ids; echo
for f in $files; do echo -n " $f: "; cat $f | ids; echo; done
