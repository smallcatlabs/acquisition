"""Context composition (occupancy-weighted), fixed prefix, errors, the corrected cap counterfactual."""
import json, collections, os, statistics as st
OUT = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(OUT, "analyse.py")).read().split("print(f\"calls {len(calls)}")[0])
# composition
occ = collections.Counter(); raw = collections.Counter()
for k, v in files.items():
    for kk, x in v["occ"].items(): occ[kk] += x
    for kk, x in v["raw"].items(): raw[kk] += x
def group(k):
    if k.startswith("result Bash:"): return "shell: " + k[12:]
    if k.startswith("attach:"): return "attachment: " + k[7:]
    if k.startswith("result "): return "tool: " + k[7:]
    return k
g = collections.Counter(); gr = collections.Counter()
for k, x in occ.items(): g[group(k)] += x
for k, x in raw.items(): gr[group(k)] += x
O = sum(g.values())
print("## What fills the growing part of the context (bytes x calls that re-read them, until compaction)")
print("total occupancy-weighted", f"{O/1e12:.2f} TB-reads ~= {O/3.6/1e9:.2f} B tokens at 3.6 B/token; transcript context read {TT/1e9:.2f} B tokens")
for k, x in g.most_common(28): print(f"  {k}: {x/O*100:.1f}%  (raw {gr[k]/1e6:.1f} MB)")
# fixed prefix
print("\n## Fixed prefix")
sh = [v["shape"] for v in files.values() if v["shape"] and not v["sub"]]
ss = [v["shape"] for v in files.values() if v["shape"] and v["sub"]]
for name, L in (("sessions", sh), ("subagents", ss)):
    if not L: continue
    print(name, len(L), "system bytes p50", sorted(x["system"] for x in L)[len(L)//2], "tool schema bytes p50", sorted(x["tools"] for x in L)[len(L)//2], "max", max(x["tools"] for x in L), "ntools p50", sorted(x["ntools"] for x in L)[len(L)//2])
big = collections.Counter()
for x in sh:
    for b, n in x["big"]: big[n] += b
print("largest tool schemas summed over sessions (KB):", [(n, round(b/1e3/len(sh), 1)) for n, b in big.most_common(8)])
fst = {}
for c in calls: fst.setdefault(c["file"], c)
print("first call: context tokens p25/p50/p75 (sessions)", [sorted(fst[k]["ctx"] for k in fst if not files[k]["sub"])[int(q*(sum(1 for k in fst if not files[k]['sub'])-1))] for q in (.25,.5,.75)])
print("first call: context tokens p25/p50/p75 (subagents)", [sorted(fst[k]["ctx"] for k in fst if files[k]["sub"])[int(q*(sum(1 for k in fst if files[k]['sub'])-1))] for q in (.25,.5,.75)])
# errors
print("\n## API errors")
e = collections.Counter(); days = collections.Counter()
for v in files.values():
    for x in v["errors"]:
        e[(x[0], x[2])] += 1; days[(x[1] or "")[:10]] += 1
print(e, sorted(days.items()))
# corrected cap
def cap_cost(CAP, HAND=20e3):
    tot = 0; restarts = 0
    byf = collections.defaultdict(list)
    for c in calls: byf[c["file"]].append(c)
    for k, cs in byf.items():
        base = cs[0]["ctx"]; off = 0
        for c in cs:
            p = PR[fam(c["model"])]
            if c["ctx"] - off < base: off = 0          # a compaction already dropped it
            eff = c["ctx"] - off; extra = 0
            if eff > CAP and c["ctx"] > base + HAND:
                off = c["ctx"] - (base + HAND); eff = base + HAND; extra = eff; restarts += 1
            red = c["ctx"] - eff
            cr = max(0, c["cr"] - red)
            tot += (c["in"]*p[0] + c["out"]*p[1] + cr*p[2] + c["cc"]*p[3] + extra*(p[3]-p[2])) / 1e6
    return tot, restarts
for CAP in (150e3, 200e3, 100e3):
    x, r = cap_cost(CAP); print(f"(i) restart fresh (prefix + 20k handoff) whenever context passes {CAP/1e3:.0f}k: ${x:,.0f} ({(x-T)/T*100:+.0f}%), restarts {r}")
