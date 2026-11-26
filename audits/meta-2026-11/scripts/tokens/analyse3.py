"""First-call composition, cache-write (re-cache) cost by idle gap, daily activity, cost-state cross-check."""
import json, collections, os, datetime as dt
OUT = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(OUT, "analyse.py")).read().split("print(f\"calls {len(calls)}")[0])
fi = collections.Counter(); nf = 0
for k, v in files.items():
    if v["sub"] or not v["shape"]: continue
    nf += 1
    fi["system prompt"] += v["shape"]["system"]; fi["tool schemas"] += v["shape"]["tools"]
    for kk, x in v["first_items"].items(): fi[kk] += x
print("## First call of a session, mean bytes", nf)
for k, x in fi.most_common(14): print(f"  {k}: {x/nf/1e3:.1f} KB")
# cache writes
def ts(s): return dt.datetime.fromisoformat(s.replace("Z", "+00:00"))
byf = collections.defaultdict(list)
for c in calls: byf[c["file"]].append(c)
gb = collections.defaultdict(lambda: [0, 0.0, 0])
ccost = 0
for k, cs in byf.items():
    prev = None
    for c in cs:
        p = PR[fam(c["model"])]
        w = c["cc"] * p[3] / 1e6; ccost += w
        gap = (ts(c["ts"]) - ts(prev["ts"])).total_seconds() if prev else -1
        b = "first call" if prev is None else ("<5 min" if gap < 300 else ("5-60 min" if gap < 3600 else ">1 h"))
        big = c["cc"] >= 50e3
        g = gb[(b, big)]; g[0] += 1; g[1] += w; g[2] += c["cc"]
        prev = c
print(f"\n## Cache writes: ${ccost:,.0f} of ${T:,.0f}")
for key in sorted(gb): print(" ", key, "calls", gb[key][0], f"${gb[key][1]:.0f}", f"mean write {gb[key][2]/gb[key][0]/1e3:.0f}k")
# daily activity and cost
day = collections.defaultdict(lambda: [0, 0.0, set()])
for c in calls:
    d = c["ts"][:10]; day[d][0] += 1; day[d][1] += c["$"]; day[d][2].add(int(c["ts"][11:13]))
print("\n## Days: calls, $, active hours")
for d in sorted(day): print(" ", d, day[d][0], f"${day[d][1]:.0f}", len(day[d][2]))
# cost-state cross-check: top-level files with a cost record, priced top-level calls vs record
rec = 0; mine = 0; mine_sub = 0
for k, v in files.items():
    if v["sub"] or v["cost"] is None: continue
    rec += v["cost"]
    mine += sum(c["$"] for c in byf.get(k, []))
    mine_sub += sum(c["$"] for kk in byf if files[kk]["sub"] and files[kk]["sess"] == v["sess"] for c in byf[kk])
print(f"\ncost-state recorded ${rec:,.0f}; priced same sessions' own calls ${mine:,.0f}; plus their subagents ${mine+mine_sub:,.0f}")
