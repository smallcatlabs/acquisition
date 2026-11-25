#!/bin/zsh
# Trial: does a copy-on-write clone of the tree start a second build lane warm?
S="$1"; R="$2"; L="$S/lane2"
exec > "$S/lane.out" 2>&1
avail() { df -k /System/Volumes/Data | awk 'NR==2{print $4}'; }
a0=$(avail)
t0=$(date +%s)
mkdir -p "$L"
# source and git by clone; only the debug cache of target
( cd "$R" && for x in $(ls -A | grep -v -e '^target$' -e '^search$' -e '^runs$' -e '^audits$'); do cp -cR "$x" "$L/"; done )
mkdir -p "$L/target"; cp -cR "$R/target/debug" "$L/target/debug"; cp -c "$R/target/CACHEDIR.TAG" "$L/target/" 2>/dev/null
t1=$(date +%s); a1=$(avail)
echo "clone: $((t1-t0)) s, disk used $(( (a0-a1)/1024 )) MB"
cd "$L" || exit 1
git status --short | head -3
/usr/bin/time -p cargo build --workspace --offline 2>&1 | tail -6
t2=$(date +%s); a2=$(avail)
echo "first build in the clone: $((t2-t1)) s, disk used so far $(( (a0-a2)/1024 )) MB"
/usr/bin/time -p cargo nextest run --workspace --all-targets --no-run --offline 2>&1 | tail -5
t3=$(date +%s); a3=$(avail)
echo "test binaries built: $((t3-t2)) s, disk used so far $(( (a0-a3)/1024 )) MB"
# an edit in one frontend file, then the incremental cost
echo '// lane trial' >> crates/acquisition-cli/src/dash.rs
/usr/bin/time -p cargo build --workspace --offline 2>&1 | tail -4
t4=$(date +%s)
echo "incremental after one cli edit: $((t4-t3)) s"
cd "$S" && rm -rf "$L"
a4=$(avail)
echo "removed; disk back to within $(( (a0-a4)/1024 )) MB"
echo DONE
