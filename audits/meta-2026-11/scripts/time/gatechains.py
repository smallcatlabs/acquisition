#!/usr/bin/env python3
"""Full gate runs from runs/gate/verdicts.tsv: red runs, and red->red chains (consecutive full runs,
in time order across the clone, before a green). Columns: ts sha dirty mode stage status seconds.
Run from the repo: python3 -I gatechains.py"""
import collections, datetime as dt
runs = collections.OrderedDict()
for l in open("runs/gate/verdicts.tsv"):
    c = l.rstrip("\n").split("\t")
    if len(c) < 7: continue
    key = (c[0], c[1], c[3])
    runs.setdefault(key, []).append(c[5])
full = [(k, v) for k, v in runs.items() if k[2] == "full"]
red = [(k, any(x not in ("0", "skip", "-") for x in v)) for k, v in full]
chains = []; cur = 0; start = None
for k, r in red:
    if r: cur += 1; start = start or k[0]
    else:
        if cur: chains.append((cur, start))
        cur = 0; start = None
print(f"full runs {len(full)}, red {sum(1 for _, r in red if r)}; red streaks before a green: {collections.Counter(min(c,4) for c,_ in chains)} (4=4+)")
print(" longest:", sorted(chains, reverse=True)[:3])
print(" modes seen:", collections.Counter(k[2] for k in runs))
