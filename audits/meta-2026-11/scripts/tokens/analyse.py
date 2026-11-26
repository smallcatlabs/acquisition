"""Tables from calls.jsonl and files.json. Prices are per million tokens (in, out, cache read, cache write), fitted
from the sessions' cost-state records by prices.py and rounded."""
import json, collections, datetime as dt, statistics as st, os
OUT = os.path.dirname(os.path.abspath(__file__))
PR = {"opus": (5, 25, 0.20, 5), "fable": (10, 50, 0.25, 16), "sonnet": (3, 15, 0.13, 2.7), "haiku": (1, 5, 0.06, 1)}
def fam(m):
    for k in PR:
        if k in (m or ""): return k
    return None
def cost(c, prices=None):
    p = prices or PR.get(fam(c["model"]))
    if not p: return 0
    return (c["in"]*p[0] + c["out"]*p[1] + c["cr"]*p[2] + c["cc"]*p[3]) / 1e6
calls = [json.loads(l) for l in open(os.path.join(OUT, "calls.jsonl"))]
calls = [c for c in calls if fam(c["model"])]
# a resumed or forked session copies earlier calls into its own file: keep each API call once, in the longest file holding it
_best = {}
for c in calls:
    b = _best.get(c["id"])
    if b is None or c["n"] > b["n"]: _best[c["id"]] = c
_n0 = len(calls); calls = [c for c in calls if _best[c["id"]] is c]
print("calls before cross-file dedupe", _n0, "after", len(calls))
files = json.load(open(os.path.join(OUT, "files.json")))
for c in calls: c["ctx"] = c["in"] + c["cr"] + c["cc"]; c["$"] = cost(c)
T = sum(c["$"] for c in calls); TT = sum(c["ctx"] for c in calls)
print(f"calls {len(calls)}  priced cost ${T:,.0f}  context tokens read {TT/1e9:.2f} B  output {sum(c['out'] for c in calls)/1e6:.1f} M")
print("cost split: cache read", round(sum(c['cr']*PR[fam(c['model'])][2] for c in calls)/1e6/T*100), "% cache write", round(sum(c['cc']*PR[fam(c['model'])][3] for c in calls)/1e6/T*100), "% output", round(sum(c['out']*PR[fam(c['model'])][1] for c in calls)/1e6/T*100), "%")

print("\n## Context per call by model")
by = collections.defaultdict(list)
for c in calls: by[fam(c["model"])].append(c)
for m, cs in by.items():
    x = sorted(c["ctx"] for c in cs)
    q = lambda p: x[int(p*(len(x)-1))]
    print(m, len(cs), "calls; p10/p50/p90/max", q(.1)//1000, q(.5)//1000, q(.9)//1000, x[-1]//1000, "k; mean", int(st.mean(x))//1000, "k; cost $", round(sum(c['$'] for c in cs)))
print("\n## Buckets (share of calls / share of cost / share of context tokens)")
B = [(0, 50e3, "<50k"), (50e3, 100e3, "50-100k"), (100e3, 200e3, "100-200k"), (200e3, 400e3, "200-400k"), (400e3, 1e9, ">400k")]
for lo, hi, name in B:
    cs = [c for c in calls if lo <= c["ctx"] < hi]
    print(name, f"{len(cs)/len(calls)*100:.1f}%  {sum(c['$'] for c in cs)/T*100:.1f}%  {sum(c['ctx'] for c in cs)/TT*100:.1f}%")
over200 = [c for c in calls if c["ctx"] >= 200e3]
print("calls at >=200k:", len(over200), "cost share", round(sum(c['$'] for c in over200)/T*100,1))
# calls after a file first passed 200k
first = {}
after = 0; after_cost = 0
for c in calls:
    k = c["file"]
    if k in first: after += 1; after_cost += c["$"]
    elif c["ctx"] >= 200e3: first[k] = c["i"]
print("calls made after their transcript first passed 200k:", after, f"({after/len(calls)*100:.1f}%), cost share {after_cost/T*100:.1f}%; transcripts that passed 200k:", len(first))

print("\n## Compactions")
comp = [(k, v["compacts"]) for k, v in files.items() if v["compacts"]]
allc = [c for _, cs in comp for c in cs]
trig = collections.Counter(c[1] for c in allc)
pre = sorted(c[2] or 0 for c in allc)
print("compactions", len(allc), "in", len(comp), "transcripts of", len(files), trig, "preTokens median", pre[len(pre)//2] if pre else 0, "max", pre[-1] if pre else 0)

print("\n## Session shapes (top-level sessions with their subagents rolled separately)")
per = collections.defaultdict(lambda: {"n": 0, "$": 0, "ctx": 0, "first": None, "last": None, "model": collections.Counter()})
for c in calls:
    p = per[c["file"]]; p["n"] += 1; p["$"] += c["$"]; p["ctx"] += c["ctx"]; p["model"][fam(c["model"])] += 1
    p["first"] = p["first"] or c["ts"]; p["last"] = c["ts"]
tops = {k: v for k, v in per.items() if not files[k]["sub"]}
subs = {k: v for k, v in per.items() if files[k]["sub"]}
ns = sorted(v["n"] for v in tops.values())
print("top-level sessions", len(tops), "calls p50/p75/p90/max", ns[len(ns)//2], ns[3*len(ns)//4], ns[int(.9*len(ns))], ns[-1])
for lo, hi in [(0, 50), (50, 200), (200, 500), (500, 1000), (1000, 99999)]:
    vs = [v for v in tops.values() if lo <= v["n"] < hi]
    print(f"  sessions with {lo}-{hi} calls: {len(vs)}, cost share {sum(v['$'] for v in vs)/T*100:.1f}%")
print("subagent transcripts", len(subs), "cost share", round(sum(v['$'] for v in subs.values())/T*100, 1), "% ; calls share", round(sum(v['n'] for v in subs.values())/len(calls)*100, 1), "%")
print("\nTen largest top-level sessions by cost:")
for k, v in sorted(tops.items(), key=lambda kv: -kv[1]["$"])[:10]:
    f = files[k]
    print(f"  {v['first'][:10]}..{v['last'][:10]}  calls {v['n']}  ${v['$']:.0f}  mean ctx {v['ctx']/v['n']/1e3:.0f}k  compactions {len(f['compacts'])}  role~{f['role']}  models {dict(v['model'])}  subagents {sum(1 for kk in subs if files[kk]['sess']==f['sess'])}")

print("\n## Subagents by model")
sm = collections.defaultdict(lambda: [0, 0, 0, 0])
for k, v in subs.items():
    m = v["model"].most_common(1)[0][0]; s = sm[m]; s[0] += 1; s[1] += v["n"]; s[2] += v["$"]; s[3] += v["ctx"]
for m, s in sm.items(): print(m, "transcripts", s[0], "calls", s[1], f"cost ${s[2]:.0f}", f"mean ctx {s[3]/max(s[1],1)/1e3:.0f}k")
# do subagents build?
kinds = collections.Counter()
for k, v in subs.items():
    tl = [t for c in calls if c["file"] == k for t in c["tools"]] if False else None
sub_tools = collections.defaultdict(collections.Counter)
for c in calls:
    if c["sub"]:
        for t in c["tools"]: sub_tools[c["file"]][t] += 1
kind = collections.Counter(); kind_cost = collections.Counter()
for k, v in subs.items():
    tc = sub_tools[k]
    if tc["Bash:git commit/push"]: kd = "commits"
    elif tc["Edit/Write"] or tc["Bash:cargo build"] or tc["Bash:cargo test"] or tc["Bash:gate"]: kd = "edits or builds"
    else: kd = "reads only"
    kind[kd] += 1; kind_cost[kd] += v["$"]
print("subagent kinds", dict(kind), {k: round(x) for k, x in kind_cost.items()})

print("\n## Mechanical calls (every tool call of the call is one of these kinds)")
MECH = {"Bash:gate", "Bash:git commit/push", "Write:review file", "Bash:fixtures", "Bash:sleep/poll", "Bash:gh", "Monitor", "Bash:checks/bothy", "TaskOutput", "ScheduleWakeup"}
mc = [c for c in calls if c["tools"] and all(t in MECH for t in c["tools"])]
print("mechanical calls", len(mc), f"{len(mc)/len(calls)*100:.1f}% of calls, {sum(c['$'] for c in mc)/T*100:.1f}% of cost; mean ctx {st.mean(c['ctx'] for c in mc)/1e3:.0f}k; on opus/fable {sum(1 for c in mc if fam(c['model']) in ('opus','fable'))}")
mk = collections.Counter(); mk_cost = collections.Counter()
for c in mc:
    for t in set(c["tools"]): mk[t] += 1; mk_cost[t] += c["$"]
for t, n in mk.most_common(): print(f"  {t}: {n} calls, ${mk_cost[t]:.0f}")
nt = [c for c in calls if not c["tools"]]
print("calls with no tool call (text answers/ends):", len(nt), f"{sum(c['$'] for c in nt)/T*100:.1f}% cost")

print("\n## Counterfactuals (arithmetic on recorded calls, not measurement)")
# (i) cap: a transcript restarts fresh when its context would exceed CAP; a restart re-pays the prefix (first call ctx) as cache write and reads prefix+growth since restart
def cap_cost(CAP):
    tot = 0
    byf = collections.defaultdict(list)
    for c in calls: byf[c["file"]].append(c)
    for k, cs in byf.items():
        base = cs[0]["ctx"]  # prefix at first call
        offset = 0
        for c in cs:
            eff = c["ctx"] - offset
            if eff > CAP:
                offset = c["ctx"] - base * 2  # new session: prefix + a handoff of the same size
                eff = c["ctx"] - offset
                extra_write = eff
            else:
                extra_write = 0
            scale = eff / c["ctx"] if c["ctx"] else 1
            p = PR[fam(c["model"])]
            tot += (c["in"]*p[0] + c["out"]*p[1] + (c["cr"]*scale)*p[2] + (c["cc"])*p[3] + extra_write*p[3]) / 1e6
    return tot
for CAP in (150e3, 100e3):
    x = cap_cost(CAP); print(f"(i) cap at {CAP/1e3:.0f}k: ${x:,.0f} ({(x-T)/T*100:+.0f}%)")
# (ii) mechanical calls on Sonnet at a 30k fresh context
x = T - sum(c["$"] for c in mc) + sum(cost({**c, "cr": 30e3, "cc": 0, "in": 0}, PR["sonnet"]) for c in mc)
print(f"(ii) mechanical calls on sonnet at 30k: ${x:,.0f} ({(x-T)/T*100:+.0f}%)")
# (iii) prefix halved: estimate prefix per transcript as its first call's context; every call reads prefix/2 less
x = 0
fstctx = {}
for c in calls: fstctx.setdefault(c["file"], c["ctx"])
for c in calls:
    cut = min(fstctx[c["file"]] / 2, c["cr"])
    p = PR[fam(c["model"])]
    x += c["$"] - cut * p[2] / 1e6
print(f"(iii) fixed prefix halved: ${x:,.0f} ({(x-T)/T*100:+.0f}%)")
# (iv) governance reading path of 102 KB (~26k tokens) removed from every call
x = sum(c["$"] - min(26e3, c["cr"]) * PR[fam(c["model"])][2] / 1e6 for c in calls)
print(f"(iv) 26k tokens (102 KB) of documents gone from every call: ${x:,.0f} ({(x-T)/T*100:+.0f}%)")
fp = sorted(fstctx[k] for k in fstctx if not files[k]["sub"]); fs = sorted(fstctx[k] for k in fstctx if files[k]["sub"])
print("first-call context: sessions p50", fp[len(fp)//2], "subagents p50", fs[len(fs)//2])

print("\n## Per ISO week: cost, calls, mean ctx")
wk = collections.defaultdict(lambda: [0, 0, 0])
for c in calls:
    d = dt.datetime.fromisoformat(c["ts"].replace("Z", "+00:00")); w = "%d-W%02d" % d.isocalendar()[:2]
    wk[w][0] += c["$"]; wk[w][1] += 1; wk[w][2] += c["ctx"]
json.dump({w: v for w, v in wk.items()}, open(os.path.join(OUT, "weeks.json"), "w"))
for w in sorted(wk): print(w, f"${wk[w][0]:.0f}", wk[w][1], f"{wk[w][2]/wk[w][1]/1e3:.0f}k")
