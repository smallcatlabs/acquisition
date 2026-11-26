#!/usr/bin/env python3
"""Coverage and starts. Run from the repo: python3 -I coverage.py events.json
- commits (since 2026-10-12) with no local transcript event within 15 min before them
- who opened each top-level session (first H vs first P/S)
- owner-wait gaps: how much of the total sits in gaps over 1 h
"""
import json, sys, subprocess, bisect, collections, datetime as dt
data = json.load(open(sys.argv[1]))
ts = sorted(e[0] for f in data for e in f["ev"])
log = subprocess.run(["git","log","--since=2026-10-12","--format=%ct %cn"],capture_output=True,text=True).stdout.split("\n")
n=0; orphan=0; orph_days=collections.Counter()
for l in log:
    if not l.strip(): continue
    t=int(l.split()[0]); n+=1
    i=bisect.bisect_right(ts,t+60)
    if i==0 or t-ts[i-1] > 900:
        orphan+=1; orph_days[dt.datetime.fromtimestamp(t).date().isoformat()]+=1
print(f"commits {n}; with no local transcript event in the 15 min before: {orphan}")
print(" by day:", sorted(orph_days.items())[-12:])
starts=collections.Counter(); wk=collections.defaultdict(collections.Counter)
for f in data:
    if f["sub"] or not f["ev"]: continue
    first=[e for e in f["ev"] if e[1] in ("H","P","N")]
    k = first[0][1] if first else "none"
    k = {"H":"owner","P":"another session","N":"notification","none":"no prompt"}[k]
    starts[k]+=1
    w=dt.datetime.fromtimestamp(f["ev"][0][0]).isocalendar()[1]; wk[w][k]+=1
print("top-level sessions by who opened them:", dict(starts))
for w in sorted(wk): print("  W%d"%w, dict(wk[w]))
# owner-wait gaps from any prior event
g=[]
for f in data:
    ev=f["ev"]
    for i in range(1,len(ev)):
        if ev[i][1]=="H": g.append(ev[i][0]-ev[i-1][0])
g=[x for x in g if x<=8*3600]
print(f"gaps before a human message (<=8h): n={len(g)} total {sum(g)/3600:.0f} h; in gaps >1h {sum(x for x in g if x>3600)/3600:.0f} h ({sum(1 for x in g if x>3600)} gaps); in gaps <=15min {sum(x for x in g if x<=900)/3600:.0f} h")
