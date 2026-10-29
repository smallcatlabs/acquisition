#!/usr/bin/env bash
# gate-breakers.sh — proves tools/gate.sh refuses what it claims to, by
# staging each refusal and watching it fail: a stage name it does not
# know, the fixture switch set at any value, a working tree its
# stages changed (#87), a feature pin taken out (#124), a release
# matrix row the features stage cannot read, and cargo failing under it
# (the review of #91).
#
# A refusal is evidence only when it names the property, so each case
# states the words the gate must print; a case that passes states the
# words of the green verdict. A case that needs a stage to do something
# edits the gate's stage command by an exact-once substitution, as
# `arch-check-breakers.sh` edits its tool, and runs a selection that
# builds no Rust; an edit that does not apply, or leaves the gate
# unparseable, fails the suite instead of passing as a refusal. Every
# case restores the files it touched from copies and compares them byte
# for byte — never `git checkout`, which discards uncommitted work — and
# a trap restores on any exit. Run it when a stage or a refusal is added
# to gate.sh; CI runs it beside the gate, not in it.
set -uo pipefail
cd "$(dirname "$0")/.."
unset ACQ_UPDATE_FIXTURES   # the suite's own shell sets nothing a case did not

# The paths the tree cases create and every restore removes, named once:
# a path already there is someone's, so the suite refuses to start.
owned=(zz-gate-breaker.txt target/zz-gate-breaker.txt)
for p in "${owned[@]}"; do
  if [[ -e $p || -L $p ]]; then
    echo "refusing to start: $p exists, and the suite would delete it on every restore"; exit 2
  fi
done

scratch=$(mktemp -d "${TMPDIR:-/tmp}/acq-gate-breakers.XXXXXX")
files=(tools/gate.sh GLOSSARY.md crates/acquisition-cli/Cargo.toml Cargo.lock .github/workflows/release.yml)
for f in "${files[@]}"; do mkdir -p "$scratch/keep/$(dirname "$f")"; cp "$f" "$scratch/keep/$f"; done
restore() {
  local f
  for f in "${files[@]}"; do
    cp "$scratch/keep/$f" "$f"
    cmp -s "$scratch/keep/$f" "$f" || { echo "RESTORE FAILED: $f"; exit 2; }
  done
  rm -f "${owned[@]}"
}
trap 'restore; rm -rf "$scratch"' EXIT

# ---- edits ---------------------------------------------------------------
stage_does() {  # stage_does <stage> <command>: the stage's command replaced, exact once
  python3 - "$1" "$2" <<'PY'
import pathlib, re, sys
p = pathlib.Path('tools/gate.sh'); s = p.read_text()
line = re.compile(r'(?m)^    ' + re.escape(sys.argv[1]) + r'\) .* ;;$')
assert len(line.findall(s)) == 1, 'no one stage line for ' + sys.argv[1]
p.write_text(line.sub(lambda m: '    ' + sys.argv[1] + ') ' + sys.argv[2] + ' ;;', s))
PY
}
gate_edit() {  # gate_edit <old> <new>: an exact-once substitution in gate.sh
  python3 - "$1" "$2" <<'PY'
import pathlib, sys
p = pathlib.Path('tools/gate.sh'); s = p.read_text()
assert s.count(sys.argv[1]) == 1, 'edit anchor not found once: ' + sys.argv[1][:60]
p.write_text(s.replace(sys.argv[1], sys.argv[2]))
PY
}
docs_are() {  # docs_are <stage>...: the --docs selection replaced, so a case builds no Rust
  python3 - "$*" <<'PY'
import pathlib, sys
p = pathlib.Path('tools/gate.sh'); s = p.read_text(); a = 'DOCS=(diff docs arch trade bothy)\n'
assert s.count(a) == 1, 'the DOCS line is not there once'
p.write_text(s.replace(a, 'DOCS=(' + sys.argv[1] + ')\n'))
PY
}

# ---- the harness -----------------------------------------------------------
pass=0; failed=0
run_case() {  # run_case <name> <refuse|pass> <pattern the output must contain> <setup> <gate command...>
  local name=$1 want=$2 pattern=$3 setup=$4; shift 4
  local out status
  if ! $setup 2>"$scratch/setup.err"; then
    printf 'FAIL    %-52s the breaker did not apply: %s\n' "$name" "$(head -c 200 "$scratch/setup.err")"; failed=$((failed+1)); restore; return
  fi
  if ! bash -n tools/gate.sh 2>/dev/null; then
    printf 'FAIL    %-52s the breaker left the gate unparseable\n' "$name"; failed=$((failed+1)); restore; return
  fi
  out=$("$@" 2>&1); status=$?
  if [[ $want == refuse && $status -ne 0 && $out == *"$pattern"* ]] \
     || [[ $want == pass && $status -eq 0 && $out == *"$pattern"* ]]; then
    printf 'ok      %-52s %s: %s\n' "$name" "$want" "$(grep -m1 -F -- "$pattern" <<<"$out" | cut -c1-90)"; pass=$((pass+1))
  else
    printf 'FAIL    %-52s wanted %s with "%s"; exit %d, got:\n%s\n' "$name" "$want" "$pattern" "$status" "$(sed 's/^/          /' <<<"$out")"
    failed=$((failed+1))
  fi
  restore
}
none() { :; }
gate=tools/gate.sh

echo '== a stage the gate does not know'
run_case 'an unknown stage under --only'                         refuse '--only takes one of: fixtures' none $gate --only bogus
run_case 'an unknown flag'                                       refuse 'tools/gate.sh --only <stage>'  none $gate --bogus

echo '== the fixture switch: set at any value, the gate refuses (#87)'
FX='ACQ_UPDATE_FIXTURES is set in this shell'
run_case 'the switch at 0'                                       refuse "$FX (to '0')" none env ACQ_UPDATE_FIXTURES=0 $gate --only fixtures
run_case 'the switch empty'                                      refuse "$FX (to '')"  none env ACQ_UPDATE_FIXTURES= $gate --only fixtures
run_case 'the switch at 1'                                       refuse "$FX (to '1')" none env ACQ_UPDATE_FIXTURES=1 $gate --only fixtures
run_case 'the switch unset'                                      pass   'gate: green'  none $gate --only fixtures
# The stage that would rewrite a golden is any stage after the switch is
# read: the gate unsets it for all of them, under any selection.
sees_no_switch() { stage_does diff '[[ -z ${ACQ_UPDATE_FIXTURES+x} ]] || { echo "the stage saw the switch"; false; }'; }
run_case 'a stage run with the switch set sees it unset'         pass   'gate: green'  sees_no_switch env ACQ_UPDATE_FIXTURES=1 $gate --only diff
run_case 'the same stage under --docs'                           pass   'gate: green'  sees_no_switch env ACQ_UPDATE_FIXTURES=0 $gate --docs

echo '== the tree: a stage that changed it fails the gate, naming each path (#87)'
# The `diff` stage stands for any stage, a test run among them; the
# selection is `diff` then `tree`.
writes() { docs_are diff tree && stage_does diff "$1"; }   # writes <the stage's command>
dirty_glossary() { printf 'a session edit\n' >>GLOSSARY.md; }
appends()        { writes 'printf x >>GLOSSARY.md'; }
creates()        { writes 'printf x >zz-gate-breaker.txt'; }
deletes()        { writes 'rm GLOSSARY.md'; }
ignored()        { writes 'mkdir -p target && printf x >target/zz-gate-breaker.txt'; }   # a fresh clone has no target/
rewrites_dirty() { dirty_glossary && appends; }
leaves_dirty()   { dirty_glossary && writes true; }
run_case 'a stage dirties a clean tracked file'                  refuse $'M\tGLOSSARY.md'         appends        $gate --docs
run_case 'a stage creates a file git does not ignore'            refuse $'A\tzz-gate-breaker.txt' creates        $gate --docs
run_case 'a stage deletes a tracked file'                        refuse $'D\tGLOSSARY.md'         deletes        $gate --docs
run_case 'a stage rewrites a file already dirty'                 refuse $'M\tGLOSSARY.md'         rewrites_dirty $gate --docs
run_case 'a tree dirty before the gate, left as found'           pass   'gate: green'              leaves_dirty   $gate --docs
run_case 'a stage writes under an ignored path'                  pass   'gate: green'              ignored        $gate --docs
no_snapshot() { gate_edit 'git write-tree)' 'git write-treee)'; }
run_case 'a snapshot that cannot be taken'                       refuse 'no snapshot of the working tree before the first stage' no_snapshot $gate --only tree
run_case 'the tree stage alone, on the tree as it stands'        pass   'gate: green'              none           $gate --only tree

echo '== the feature graphs: a test build that compiles a package otherwise fails, naming it (#124)'
# The pin #124 needed, taken out: cargo then drops its line from the
# lock, which the restore puts back with the manifest.
unpinned() {
  python3 - <<'PY'
import pathlib, re
p = pathlib.Path('crates/acquisition-cli/Cargo.toml'); s = p.read_text()
pin = re.compile(r'(?m)^rustix = .*\n')
assert len(pin.findall(s)) == 1, 'no one rustix pin'
p.write_text(pin.sub('', s))
PY
}
run_case 'the rustix pin taken out'                              refuse 'rustix 1.1.4: build'      unpinned       $gate --only features
# The release's targets are every triple release.yml names, in any YAML
# form (the reviews of #91): each case checks exactly the set it names.
FIVE="the release's 5 targets: aarch64-apple-darwin aarch64-unknown-linux-gnu x86_64-apple-darwin x86_64-pc-windows-msvc x86_64-unknown-linux-gnu"
SIX="the release's 6 targets: aarch64-apple-darwin aarch64-pc-windows-msvc aarch64-unknown-linux-gnu x86_64-apple-darwin x86_64-pc-windows-msvc x86_64-unknown-linux-gnu"
release_edit() {  # release_edit <old> <new>: an exact-once substitution in release.yml
  python3 - "$1" "$2" <<'PY'
import pathlib, sys
p = pathlib.Path('.github/workflows/release.yml'); s = p.read_text()
assert s.count(sys.argv[1]) == 1, 'edit anchor not found once: ' + sys.argv[1][:60]
p.write_text(s.replace(sys.argv[1], sys.argv[2]))
PY
}
WINDOWS_ROW='          - { os: windows-latest, target: x86_64-pc-windows-msvc }'
block_row() { release_edit "$WINDOWS_ROW" $'          - os: windows-latest\n            target: x86_64-pc-windows-msvc'; }
axis() { release_edit '        include:' $'        target: [aarch64-pc-windows-msvc]\n        include:'; }
no_triple() { printf 'name: Release\non: workflow_dispatch\n' >.github/workflows/release.yml; }
run_case 'the release rows inline, as they stand'                pass   "$FIVE"                    none           $gate --only features
run_case 'a release row in block form beside inline ones'        pass   "$FIVE"                    block_row      $gate --only features
run_case 'a matrix axis beside include'                          pass   "$SIX"                     axis           $gate --only features
run_case 'a release file naming no triple'                       refuse 'names no target triple'   no_triple      $gate --only features
# cargo failing under the stage is its failure, never a split to pin.
cargo_fails() { gate_edit '--target "$1" -e no-dev,features' '--target "zz-no-such-target" -e no-dev,features'; }
run_case 'cargo tree failing under the stage'                    refuse 'cargo tree could not read the feature graphs' cargo_fails $gate --only features

echo
printf '%d ok, %d failed\n' "$pass" "$failed"
((failed == 0))
