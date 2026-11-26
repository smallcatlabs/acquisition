"""Cap counterfactual variant: a restart re-reads the prefix from a shared cache and writes only the handoff."""
import json, collections, os
OUT = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(OUT, "analyse.py")).read().split("print(f\"calls {len(calls)}")[0])
def cap_cost(CAP, HAND=20e3, prefix_cached=True):
    tot = 0; byf = collections.defaultdict(list)
    for c in calls: byf[c["file"]].append(c)
    for k, cs in byf.items():
        base = cs[0]["ctx"]; off = 0
        for c in cs:
            p = PR[fam(c["model"])]
            if c["ctx"] - off < base: off = 0
            eff = c["ctx"] - off; extra = 0
            if eff > CAP and c["ctx"] > base + HAND:
                off = c["ctx"] - (base + HAND); eff = base + HAND; extra = HAND if prefix_cached else eff
            cr = max(0, c["cr"] - (c["ctx"] - eff))
            tot += (c["in"]*p[0] + c["out"]*p[1] + cr*p[2] + c["cc"]*p[3] + extra*(p[3]-p[2])) / 1e6
    return tot
for CAP in (100e3, 150e3, 200e3):
    x = cap_cost(CAP); print(f"restart at {CAP/1e3:.0f}k, prefix from shared cache: ${x:,.0f} ({(x-T)/T*100:+.0f}%)")
