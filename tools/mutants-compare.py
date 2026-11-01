#!/usr/bin/env python3
"""Two cargo-mutants runs over `.cargo/mutants.toml`'s scope, compared
mutant by mutant (t5's close, #141; the design's round 0 asks that
corresponding mutants be compared and additions and removals classified
apart).

    python3 -I tools/mutants-compare.py BEFORE AFTER [--config-rev REV]

BEFORE and AFTER are directories holding a run's shards as the weekly
workflow uploads them (`mutants-shard-N/outcomes.json`, gitignored under
`runs/`). Every mutant is keyed by its file, its function and its
description, the name less its `file:line:col:` prefix, never by the
line, which drifts with any edit above it. A key may name several mutants
(the same operator twice in one function), so each run is a list that
keeps duplicates: within a key the mutants pair in source order, and
what is left over is an addition (after only) or a removal (before only).

Every mutant missed after falls in one of three lists:

- a listed exclusion whose line drifted: an `exclude_re` pattern of the
  config at REV (default HEAD) matches its name once the pattern's line
  is read as any line (its column kept: an edit above moves a line, not
  a column, and two listed lines of one function differ by column);
- a baseline survivor: its pair was missed before too;
- a survivor the change introduced: anything else, with what its pair's
  outcome was before, or "added".

Timeouts count as caught, as the config says; they are compared as their
own outcome all the same. Reads only; prints the comparison.
"""

import argparse
import collections
import glob
import json
import os
import re
import subprocess
import sys
import tomllib

OUTCOMES = {
    "CaughtMutant": "caught",
    "MissedMutant": "missed",
    "Timeout": "timeout",
    "Unviable": "unviable",
}
NAME = re.compile(r"^(?P<file>.*?):(?P<line>\d+):(?P<col>\d+): (?P<desc>.*)$", re.S)


def load(run_dir):
    """One run's mutants, each a dict, and its shards' headers."""
    paths = sorted(glob.glob(os.path.join(run_dir, "mutants-shard-*", "outcomes.json")))
    if not paths:
        sys.exit(f"mutants-compare: no mutants-shard-*/outcomes.json under {run_dir}")
    mutants, shards = [], []
    for path in paths:
        with open(path, encoding="utf-8") as handle:
            data = json.load(handle)
        header = {k: v for k, v in data.items() if k != "outcomes"}
        header["shard"] = os.path.basename(os.path.dirname(path))
        counted = collections.Counter()
        for outcome in data["outcomes"]:
            scenario = outcome["scenario"]
            if scenario == "Baseline":
                if outcome["summary"] != "Success":
                    sys.exit(f"mutants-compare: {path}: the unmutated tree did not pass")
                continue
            summary = OUTCOMES.get(outcome["summary"])
            if summary is None:
                sys.exit(f"mutants-compare: {path}: an outcome this script does not know: {outcome['summary']}")
            mutant = scenario["Mutant"]
            match = NAME.match(mutant["name"])
            if match is None or match["file"] != mutant["file"]:
                sys.exit(f"mutants-compare: {path}: a name this script cannot split: {mutant['name']}")
            function = (mutant.get("function") or {}).get("function_name") or "(no function)"
            mutants.append(
                {
                    "name": mutant["name"],
                    "file": mutant["file"],
                    "function": function,
                    "desc": match["desc"],
                    "at": (int(match["line"]), int(match["col"])),
                    "outcome": summary,
                    "shard": header["shard"],
                }
            )
            counted[summary] += 1
        for kind in ("missed", "caught", "timeout", "unviable"):
            if header.get(kind) != counted[kind]:
                sys.exit(f"mutants-compare: {path}: header says {kind} {header.get(kind)}, outcomes hold {counted[kind]}")
        if header.get("total_mutants") != sum(counted.values()):
            sys.exit(f"mutants-compare: {path}: header's total disagrees with its outcomes")
        shards.append(header)
    return mutants, shards


def by_key(mutants):
    groups = collections.defaultdict(list)
    for mutant in mutants:
        groups[(mutant["file"], mutant["function"], mutant["desc"])].append(mutant)
    for group in groups.values():
        group.sort(key=lambda m: m["at"])
    return groups


def exclusions(rev):
    text = subprocess.run(
        ["git", "show", f"{rev}:.cargo/mutants.toml"], check=True, capture_output=True, text=True
    ).stdout
    patterns = tomllib.loads(text).get("exclude_re", [])
    # The listed line read as any, its column kept: `limiter\.rs:685:26:`.
    return [(p, re.compile(re.sub(r":\d+:(\d+):", r":\\d+:\1:", p, count=1))) for p in patterns]


def where(mutant):
    return f"{os.path.basename(mutant['file'])}:{mutant['at'][0]}:{mutant['at'][1]}"


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("before")
    parser.add_argument("after")
    parser.add_argument("--config-rev", default="HEAD")
    args = parser.parse_args()

    runs = {}
    for label, run_dir in (("before", args.before), ("after", args.after)):
        mutants, shards = load(run_dir)
        runs[label] = mutants
        versions = sorted({s.get("cargo_mutants_version") for s in shards})
        print(f"{label}: {run_dir}, cargo-mutants {', '.join(map(str, versions))}")
        for s in shards:
            print(
                f"  {s['shard']}: missed {s['missed']}, caught {s['caught']}, timeout {s['timeout']},"
                f" unviable {s['unviable']}, total {s['total_mutants']}"
            )
        totals = collections.Counter(m["outcome"] for m in mutants)
        print(
            f"  inventory {len(mutants)}: missed {totals['missed']}, caught {totals['caught']},"
            f" timeout {totals['timeout']}, unviable {totals['unviable']}"
        )
        keys = by_key(mutants)
        print(f"  {len(keys)} keys, {sum(len(g) - 1 for g in keys.values() if len(g) > 1)} duplicates within a key")

    before, after = by_key(runs["before"]), by_key(runs["after"])
    pairs, added, removed = [], [], []
    for key in sorted(set(before) | set(after)):
        b, a = before.get(key, []), after.get(key, [])
        pairs += list(zip(b, a))
        added += a[len(b):]
        removed += b[len(a):]

    print(f"\npaired {len(pairs)}, added {len(added)}, removed {len(removed)}")
    moves = collections.Counter((b["outcome"], a["outcome"]) for b, a in pairs)
    print("outcomes of the paired, before -> after:")
    for (was, now), n in sorted(moves.items()):
        print(f"  {was:>8} -> {now:<8} {n}")
    changed = [(b, a) for b, a in pairs if b["outcome"] != a["outcome"]]
    if changed:
        print("paired mutants whose outcome changed:")
        for b, a in changed:
            print(f"  {b['outcome']} -> {a['outcome']}: {where(b)} -> {where(a)}: {a['desc']}")
    drift = collections.Counter(a["at"][0] - b["at"][0] for b, a in pairs)
    print("line drift of the paired (after less before): " + ", ".join(f"{d:+d} x{n}" for d, n in sorted(drift.items())))

    for label, group in (("added (after only)", added), ("removed (before only)", removed)):
        print(f"\n{label}: {len(group)}")
        by_function = collections.defaultdict(list)
        for m in group:
            by_function[(m["file"], m["function"])].append(m)
        for (file, function), ms in sorted(by_function.items()):
            print(f"  {file} {function}: {len(ms)}")
            for m in ms:
                print(f"    {where(m)} {m['outcome']}: {m['desc']}")

    listed = exclusions(args.config_rev)
    pair_of = {id(a): b for b, a in pairs}
    lists = {"listed, line drifted": [], "baseline survivor": [], "introduced": []}
    for a in sorted((m for m in runs["after"] if m["outcome"] == "missed"), key=lambda m: (m["file"], m["at"])):
        b = pair_of.get(id(a))
        was = b["outcome"] if b else "added"
        hits = [p for p, rx in listed if rx.search(a["name"])]
        exact = [p for p in hits if re.search(p, a["name"])]
        if len(hits) > 1:
            sys.exit(f"mutants-compare: {a['name']} matches {len(hits)} listed patterns once their lines are loosened")
        if hits and not exact:
            lists["listed, line drifted"].append((a, b, was, hits[0]))
        elif hits:
            sys.exit(f"mutants-compare: {a['name']} is missed though a pattern at --config-rev lists it exactly: was the run on another config?")
        elif was == "missed":
            lists["baseline survivor"].append((a, b, was, None))
        else:
            lists["introduced"].append((a, b, was, None))

    print(f"\nmissed after: {sum(len(v) for v in lists.values())}, against the config at {args.config_rev}")
    for label, rows in lists.items():
        print(f"{label}: {len(rows)}")
        for a, b, was, pattern in rows:
            line = f"  {where(a)} ({f'was {where(b)}, {was} before' if b else 'added'}): {a['desc']}"
            if pattern:
                line += f"\n    listed as {pattern}"
            print(line)


if __name__ == "__main__":
    main()
