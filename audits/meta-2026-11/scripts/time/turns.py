#!/usr/bin/env python3
"""Per human message: how long the session then ran on its own (H -> last event before the next H
or session end, gaps over 3 h cut), and the gap the owner then took. Top-level files only.
Run: python3 -I turns.py events_dedup.json START"""
import json, sys, statistics as st, datetime as dt
data = json.load(open(sys.argv[1])); S = dt.datetime.fromisoformat(sys.argv[2]).timestamp()
runs = []; waits = []
for f in data:
    if f["sub"]: continue
    ev = [e for e in f["ev"] if e[0] >= S]
    hs = [i for i, e in enumerate(ev) if e[1] == "H"]
    for j, i in enumerate(hs):
        end = hs[j + 1] if j + 1 < len(hs) else len(ev)
        last = ev[i][0]
        for k in range(i + 1, end):
            if ev[k][0] - last > 3 * 3600: break
            last = ev[k][0]
        runs.append(last - ev[i][0])
        if j + 1 < len(hs): waits.append(ev[hs[j + 1]][0] - last)
q = lambda x, p: sorted(x)[int(p * (len(x) - 1))]
print(f"human msgs {len(runs)}: run after each: median {q(runs,.5)/60:.1f} min p75 {q(runs,.75)/60:.1f} p90 {q(runs,.9)/60:.1f} max {max(runs)/3600:.1f} h; total {sum(runs)/3600:.0f} h")
print(f"  share of runs under 5 min {sum(1 for r in runs if r<300)/len(runs):.0%}, over 30 min {sum(1 for r in runs if r>1800)/len(runs):.0%}; hours in runs over 30 min {sum(r for r in runs if r>1800)/3600:.0f} h")
w = [x for x in waits if x <= 8 * 3600]
print(f"owner reply gap (<=8h): median {q(w,.5)/60:.1f} min; total {sum(w)/3600:.0f} h; within 15 min {sum(x for x in w if x<=900)/3600:.0f} h ({sum(1 for x in w if x<=900)} replies)")
