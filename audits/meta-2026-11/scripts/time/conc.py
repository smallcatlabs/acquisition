#!/usr/bin/env python3
"""Per ISO week: concurrency over active minutes (top-level sessions with a <5-min gap or a pending
foreground tool), human messages, and a second route to active hours: distinct minutes holding any
raw transcript line with a timestamp (all files, all event types).
Run: python3 -I conc.py events_dedup.json <transcript dir>"""
import json, sys, glob, re, collections, datetime as dt
data = json.load(open(sys.argv[1]))
wk = lambda t: dt.datetime.fromtimestamp(t).isocalendar()[1]
mins = collections.defaultdict(set); hum = collections.Counter(); hum_short = collections.Counter()
for f in data:
    ev = f["ev"]
    for i, (t, k, p) in enumerate(ev):
        if k == "H": hum[wk(t)] += 1
        if f["sub"] or i + 1 >= len(ev): continue
        t2, k2, _ = ev[i + 1]
        if k2 in ("A", "R") and (t2 - t < 300 or k2 == "R"):
            if t2 - t > 3 * 3600: continue
            for m in range(int(t // 60), int(t2 // 60) + 1): mins[m].add(f["sid"])
per = collections.defaultdict(list)
for m, s in mins.items(): per[wk(m * 60)].append(len(s))
raw = collections.defaultdict(set)
rx = re.compile(r'"timestamp":"(2026-1[01]-\d\dT\d\d:\d\d)')
for fp in glob.glob(sys.argv[2] + "/**/*.jsonl", recursive=True):
    for line in open(fp, errors="replace"):
        m = rx.search(line[:4000])
        if m: raw[m.group(1)].add(1)
rawwk = collections.Counter()
for k in raw:
    d = dt.datetime.fromisoformat(k + ":00+00:00"); 
    if d >= dt.datetime(2026,10,12,tzinfo=dt.timezone.utc): rawwk[d.astimezone().isocalendar()[1]] += 1
for w in sorted(per):
    c = per[w]
    print(f"W{w} active-min {len(c)/60:5.1f} h  mean conc {sum(c)/len(c):.2f}  share >=2 sessions {sum(1 for x in c if x>=2)/len(c):.0%}  peak {max(c)}  human msgs {hum[w]}  raw-minute route {rawwk[w]/60:5.1f} h")
