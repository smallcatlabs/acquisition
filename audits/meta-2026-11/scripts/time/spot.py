#!/usr/bin/env python3
"""Spot check one day: per local hour, distinct transcript files with events, events, A events, commits.
Run from the repo: python3 -I spot.py events_dedup.json 2026-11-18"""
import json, sys, subprocess, collections, datetime as dt
data = json.load(open(sys.argv[1])); day = sys.argv[2]
files = collections.defaultdict(set); evs = collections.Counter(); A = collections.Counter(); act=collections.defaultdict(set)
for f in data:
    ev=f["ev"]
    for i,(t, k, p) in enumerate(ev):
        d = dt.datetime.fromtimestamp(t)
        if d.date().isoformat() != day: continue
        files[d.hour].add((f["sid"][:8], f["sub"])); evs[d.hour] += 1; A[d.hour] += (k == "A")
        if i+1<len(ev) and ev[i+1][0]-t<300:
            for s in range(int(t), int(ev[i+1][0])): 
                dd=dt.datetime.fromtimestamp(s)
                if dd.date().isoformat()==day: act[dd.hour].add(s)
log = subprocess.run(["git","log","--since",day+"T00:00","--until",day+"T23:59:59","--format=%ct"],capture_output=True,text=True).stdout.split()
cm = collections.Counter(dt.datetime.fromtimestamp(int(x)).hour for x in log)
for h in range(24):
    if files[h] or cm[h]:
        print(f"{h:02d} files {len(files[h]):3d} (sub {sum(1 for x in files[h] if x[1]):3d}) events {evs[h]:5d} A {A[h]:5d} gap<5min-min {len(act[h])/60:5.1f} commits {cm[h]}")
