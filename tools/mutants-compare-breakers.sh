#!/usr/bin/env bash
# mutants-compare-breakers.sh — proves tools/mutants-compare.py reads a
# cargo-mutants run whole or refuses it (t5 round 6's two reviews): a run
# is the weekly workflow's shard set, every shard present, none other,
# every one ended, and each header agreeing with its outcomes. Each case
# stages synthetic runs, a few mutants per shard in outcomes.json as
# cargo-mutants 27.1.0 writes it, under a scratch directory this suite
# owns; the real artifacts under runs/ are never read.
#
# The shard count is read from .github/workflows/weekly.yml at HEAD, as
# the script reads it (`--shard ${{ matrix.shard }}/N`), so the cases
# stage the workflow's count and never one of their own. The positive
# case's missed mutant on a drifted listed line is built from the first
# plain `exclude_re` pattern of .cargo/mutants.toml at HEAD, its line
# moved, so the case follows the config.
#
# A refusal is evidence only when it names the property: each case
# requires a non-zero exit and the words that name what it broke, and the
# positive case must compare and print its expected counts and lists, so
# a case whose staging did not apply fails the suite rather than passing
# as a refusal.
#
# Run it by hand when tools/mutants-compare.py, weekly.yml's shard split
# or cargo-mutants' version changes; CI runs it in the gate workflow's
# breakers job. Reads only the tree; writes only its scratch directory.
set -uo pipefail
cd "$(dirname "$0")/.."

scratch=$(mktemp -d "${TMPDIR:-/tmp}/acq-mutants-breakers.XXXXXX")
trap 'rm -rf "$scratch"' EXIT

shards=$(git show HEAD:.github/workflows/weekly.yml | sed -nE 's/.*--shard \$\{\{ matrix\.shard \}\}\/([0-9]+).*/\1/p')
if [[ ! $shards =~ ^[0-9]+$ ]] || ((shards < 2)); then
  echo "FAIL    weekly.yml at HEAD holds no single --shard split of two or more: '$shards'"; exit 1
fi

# stage <dir> <run: before|after> <edit>: write a whole run of $shards
# shards, then apply one edit (none, missing, extra, endnull, endabsent,
# miscount), asserting it applied.
stage() {
  python3 -I - "$1" "$2" "$3" "$shards" <<'PY'
import json, os, re, subprocess, sys, tomllib
out, run, edit, n = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4])
DAEMON = "crates/acquisition-daemon/src/"

def mutant(file, line, col, desc, function):
    return {"name": f"{file}:{line}:{col}: {desc}", "package": "acquisition-daemon", "file": file,
            "function": {"function_name": function, "return_type": "",
                         "span": {"start": {"line": line, "column": 1}, "end": {"line": line + 1, "column": 1}}},
            "span": {"start": {"line": line, "column": col}, "end": {"line": line, "column": col + 1}},
            "replacement": "", "genre": "BinaryOperator"}

# A listed exclusion, its line moved: the first pattern with no group.
config = tomllib.loads(subprocess.run(["git", "show", "HEAD:.cargo/mutants.toml"],
                                      check=True, capture_output=True, text=True).stdout)
plain = [p for p in config["exclude_re"] if "(" not in p.replace("\\(", "")]
m = re.match(r"^(\w+)\\\.rs:(\d+):(\d+): (.*)\$$", plain[0])
assert m, plain[0]
listed_desc = m.group(4).replace("\\(", "(").replace("\\)", ")").replace("\\.", ".")
listed_fn = listed_desc.rsplit(" in ", 1)[1]
listed_file = DAEMON + ("ratelimit/" if m.group(1) != "gate" else "") + m.group(1) + ".rs"

# (mutant, outcome before, outcome after)
rows = [(mutant(DAEMON + "gate.rs", 10 + k, 5, f"replace f{k} -> bool with true in f{k}", f"f{k}"),
         "CaughtMutant", "CaughtMutant") for k in range(2 * n)]
rows += [
    (mutant(listed_file, int(m.group(2)) + 100, int(m.group(3)), listed_desc, listed_fn), "MissedMutant", "MissedMutant"),
    (mutant(DAEMON + "gate.rs", 200, 9, "replace survivor -> bool with true in survivor", "survivor"), "MissedMutant", "MissedMutant"),
    (mutant(DAEMON + "gate.rs", 210, 9, "replace introduced -> bool with true in introduced", "introduced"), "CaughtMutant", "MissedMutant"),
    (mutant(DAEMON + "gate.rs", 220, 9, "replace slow -> bool with true in slow", "slow"), "Timeout", "Timeout"),
    (mutant(DAEMON + "gate.rs", 230, 9, "replace odd -> bool with true in odd", "odd"), "Unviable", "Unviable"),
]
KIND = {"CaughtMutant": "caught", "MissedMutant": "missed", "Timeout": "timeout", "Unviable": "unviable"}

def write(index, members):
    outcomes = [{"scenario": "Baseline", "summary": "Success", "log_path": "log/baseline.log",
                 "diff_path": None, "phase_results": []}]
    header = {"total_mutants": len(members), "missed": 0, "caught": 0, "timeout": 0, "unviable": 0, "success": 0}
    for mut, summary in members:
        outcomes.append({"scenario": {"Mutant": mut}, "summary": summary, "log_path": "log/x.log",
                         "diff_path": "diff/x.diff", "phase_results": []})
        header[KIND[summary]] += 1
    data = {"outcomes": outcomes, **header, "start_time": "2026-11-01T00:00:00Z",
            "end_time": "2026-11-01T01:00:00Z", "cargo_mutants_version": "27.1.0"}
    os.makedirs(f"{out}/mutants-shard-{index}")
    with open(f"{out}/mutants-shard-{index}/outcomes.json", "w") as f:
        json.dump(data, f, indent=2)

for i in range(n):
    write(i, [(mut, before if run == "before" else after)
              for k, (mut, before, after) in enumerate(rows) if k % n == i])

def edit_shard(i, change):
    path = f"{out}/mutants-shard-{i}/outcomes.json"
    data = json.load(open(path)); change(data); json.dump(data, open(path, "w"), indent=2)

if edit == "missing":
    p = f"{out}/mutants-shard-{n - 1}"
    for f in os.listdir(p): os.unlink(f"{p}/{f}")
    os.rmdir(p)
elif edit == "extra":
    write(n, [])
elif edit == "endnull":
    edit_shard(1, lambda d: d.update(end_time=None))
elif edit == "endabsent":
    edit_shard(1, lambda d: d.pop("end_time"))
elif edit == "miscount":
    def drop(d):
        d["outcomes"].pop()  # its last mutant gone, the header unchanged
    edit_shard(0, drop)
else:
    assert edit == "none", edit
PY
}

bad=0
compare() {  # compare <case>: the script over $scratch/<case>/{before,after}
  python3 -I tools/mutants-compare.py "$scratch/$1/before" "$scratch/$1/after" --before-rev HEAD --config-rev HEAD 2>&1
}

# The one positive case: a whole pair compares, every mutant paired.
mkdir -p "$scratch/whole"
stage "$scratch/whole/before" before none && stage "$scratch/whole/after" after none || { echo "FAIL    staging the whole pair"; exit 1; }
total=$((2 * shards + 5))
out=$(compare whole); rc=$?
want=("paired $total, added 0, removed 0" "listed, line drifted: 1" "baseline survivor: 1" "introduced: 1"
      "replace introduced -> bool with true in introduced" "missed after: 3")
miss=""
for w in "${want[@]}"; do [[ $out == *"$w"* ]] || miss+=" [$w]"; done
if ((rc == 0)) && [[ -z $miss ]]; then
  echo "ok      a whole pair of $shards shards compares: $total paired, one in each list"
else
  echo "FAIL    the whole pair (exit $rc), lacking:$miss"; echo "$out"; bad=1
fi

expect() {  # expect <case> <edit> <words that name the property>
  mkdir -p "$scratch/$1"
  stage "$scratch/$1/before" before none && stage "$scratch/$1/after" after "$2" || { echo "FAIL    $1: staging"; bad=1; return; }
  local out rc
  out=$(compare "$1"); rc=$?
  if ((rc != 0)) && [[ $out == *"$3"* ]]; then
    printf 'ok      %-10s refused: %s\n' "$1" "$3"
  else
    echo "FAIL    $1 (exit $rc), wanted: $3"; echo "$out" | tail -5; bad=1
  fi
}

last=$((shards - 1))
expect missing   missing   "is not a whole run of $shards shards: missing mutants-shard-$last"
expect extra     extra     "not the workflow's mutants-shard-$shards"
expect endnull   endnull   "mutants-shard-1 has not ended (no end_time)"
expect endabsent endabsent "mutants-shard-1 has not ended (no end_time)"
expect miscount  miscount  "outcomes.json: header says"

if ((bad)); then echo "mutants-compare breakers: FAILED"; exit 1; fi
echo "mutants-compare breakers: all held"
