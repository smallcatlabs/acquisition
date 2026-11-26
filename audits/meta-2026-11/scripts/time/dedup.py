#!/usr/bin/env python3
"""Drop events a forked or resumed transcript copied from an earlier one.
Run: python3 -I dedup.py events.json events_dedup.json
Key: (timestamp, kind, tool ids / result id). The earliest-starting file keeps it."""
import json, sys
data = json.load(open(sys.argv[1]))
data.sort(key=lambda f: (f["ev"][0][0] if f["ev"] else 9e18, f["sub"]))
seen = set(); dropped = 0; total = 0
def key(e):
    t, k, p = e
    if k == "A": return (t, k, tuple(x[0] for x in p["tu"]), p.get("txt"))
    if k == "R": return (t, k, p["id"])
    return (t, k)
for f in data:
    keep = []
    for e in f["ev"]:
        total += 1; kk = key(e)
        if kk in seen: dropped += 1; continue
        seen.add(kk); keep.append(e)
    f["ev"] = keep
json.dump(data, open(sys.argv[2], "w"))
print(f"dropped {dropped} of {total} events as copies")
