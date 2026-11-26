"""How fast a transcript's context grows: calls to pass 200k, tokens added per call; second routes for the top numbers."""
import json, collections, os
OUT = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(OUT, "analyse.py")).read().split("print(f\"calls {len(calls)}")[0])
byf = collections.defaultdict(list)
for c in calls: byf[c["file"]].append(c)
reach = []; grow = []
for k, cs in byf.items():
    for c in cs:
        if c["ctx"] >= 200e3: reach.append((files[k]["sub"], c["i"])); break
    if len(cs) > 20: grow.append((cs[-1]["ctx"] - cs[0]["ctx"]) / len(cs) if not files[k]["compacts"] else None)
for s in (False, True):
    r = sorted(i for sub, i in reach if sub == s); print("sub" if s else "session", "passes 200k: n", len(r), "at call p25/p50/p75", r[len(r)//4], r[len(r)//2], r[3*len(r)//4])
g = sorted(x for x in grow if x is not None); print("tokens added per call, uncompacted transcripts >20 calls: p25/p50/p75", int(g[len(g)//4]), int(g[len(g)//2]), int(g[3*len(g)//4]))
# second route on the share above 200k: cache-read tokens only, unpriced
cr = sum(c["cr"] for c in calls); print("share of cache-read tokens in calls >=200k:", round(sum(c["cr"] for c in calls if c["ctx"] >= 200e3)/cr*100, 1))
# second route on dedup: unique (file, message id) already; count assistant events per call
MECH2 = {"Bash:gate", "Bash:git commit/push", "Write:review file", "Bash:fixtures", "Bash:sleep/poll", "Monitor"}
mc = [c for c in calls if c["tools"] and all(t in MECH2 for t in c["tools"])]
print("narrow mechanical (gate, commit/push, fixtures, sleep, monitor, review file):", len(mc), round(len(mc)/len(calls)*100, 1), "% calls", round(sum(c['$'] for c in mc)/T*100, 1), "% cost")
