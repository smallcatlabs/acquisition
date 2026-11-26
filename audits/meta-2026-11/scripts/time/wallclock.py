#!/usr/bin/env python3
"""Where the wall-clock went. Reads events.json from extract.py.
Run: python3 -I wallclock.py events.json START END   (dates local, END exclusive)
Gap labelling, per transcript file, between consecutive events e1 -> e2 (g = gap):
  next is assistant           -> model   (g>=300s -> 'late model', i.e. API/limit/unknown)
  next is a tool result       -> tool:<class of that tool> (AskUserQuestion/ExitPlanMode -> owner)
  next is human-typed         -> owner   (g<=CAP; above CAP -> dormant)
  next is task-notification   -> bg:<class of the backgrounded task>
  next is injected by session -> other-session
  else                        -> model if g<300 else idle
Wall-clock per second, priority model > tool/bg > owner > other-session > nothing.
"""
import json, sys, collections, datetime as dt, statistics as st

EV, START, END = sys.argv[1], sys.argv[2], sys.argv[3]
CAP = 8 * 3600
tz = dt.datetime.now().astimezone().tzinfo
T0 = dt.datetime.fromisoformat(START).replace(tzinfo=tz).timestamp()
T1 = dt.datetime.fromisoformat(END).replace(tzinfo=tz).timestamp()
N = int(T1 - T0)
data = json.load(open(EV))
OWNER_TOOLS = {"AskUserQuestion", "ExitPlanMode"}
arr = {k: bytearray(N) for k in ("model", "tool", "owner", "other")}
human_ts = []
sum_by = collections.Counter()           # seconds summed across files (not union)
tool_by = collections.Counter()
tool_n = collections.Counter()
owner_gaps = []                           # finished turn -> human
minute_sessions = collections.defaultdict(set)
late_model = []
def mark(k, a, b):
    a = max(int(a - T0), 0); b = min(int(b - T0), N)
    if b > a: arr[k][a:b] = b"\x01" * (b - a)

for fobj in data:
    ev = [e for e in fobj["ev"] if T0 <= e[0] < T1]
    if not ev: continue
    cls_of = {}; bgcls = {}; pending = set(); last_bg_launch = None
    for i, (t, k, p) in enumerate(ev):
        if k == "H": human_ts.append(t)
        if k == "A":
            for tid, name, cls, cmd, bg in p["tu"]:
                cls_of[tid] = cls; pending.add(tid)
                if bg or cls == "subagent": last_bg_launch = (tid, cls)
        if k == "R":
            pending.discard(p["id"])
            if p.get("bg"): bgcls[p["bg"]] = cls_of.get(p["id"], "?")
        if i + 1 >= len(ev): break
        t2, k2, p2 = ev[i + 1]; g = t2 - t
        if g <= 0: continue
        if k2 == "A":
            lab = "model" if g < 300 else "late"
            if lab == "late": late_model.append(g)
            if lab == "late": lab = "model"   # counted as model but listed separately
        elif k2 == "R":
            c = cls_of.get(p2["id"], "?")
            if c in OWNER_TOOLS: lab = "owner"
            else:
                lab = "tool"; tool_by[c] += g; tool_n[c] += 1
        elif k2 == "H":
            lab = "owner" if g <= CAP else None
            if k == "A" and not p["tu"]: owner_gaps.append(g)
        elif k2 == "N":
            c = "bg:" + str(bgcls.get(p2.get("tid"), "?"))
            if g > 3 * 3600: lab = None   # a notification days later is no wait
            else: lab = "tool"; tool_by[c] += g; tool_n[c] += 1
        elif k2 == "P":
            lab = "other" if g <= CAP else None
        else:
            lab = "model" if g < 300 else None
        if lab is None: continue
        sum_by[lab] += g
        mark(lab, t, t2)
        if lab in ("model", "tool") and not fobj["sub"]:
            for m in range(int(t // 60), int(t2 // 60) + 1):
                minute_sessions[m].add(fobj["sid"])

human_ts.sort()
# per-second state
import bisect
state = collections.Counter(); per_day = collections.defaultdict(collections.Counter)
unatt = 0; act_total = 0
hum_set = set(int(h - T0) for h in human_ts if T0 <= h < T1)
# last human second before each second: sweep
last_h = -10**9; hs = sorted(hum_set); hi = 0
for s in range(N):
    while hi < len(hs) and hs[hi] <= s: last_h = hs[hi]; hi += 1
    if arr["model"][s]: st_ = "a model"
    elif arr["tool"][s]: st_ = "b tool"
    elif arr["owner"][s]: st_ = "c owner"
    elif arr["other"][s]: st_ = "d other-session"
    else: st_ = "e nothing"
    state[st_] += 1
    day = dt.datetime.fromtimestamp(T0 + s, tz).date()
    per_day[day][st_] += 1
    if st_ in ("a model", "b tool"):
        act_total += 1
        if s - last_h > 3600: unatt += 1

print(f"window {START}..{END}  {N/3600:.0f} h")
print("\nWall-clock partition (union, priority model>tool>owner>other):")
for k in sorted(state): print(f"  {k:18s} {state[k]/3600:8.1f} h  {100*state[k]/N:5.1f}%")
print(f"\nagent-active seconds with no human message in previous 60 min: {unatt/3600:.1f} h of {act_total/3600:.1f} h ({100*unatt/max(act_total,1):.0f}%)")
print("\nSummed across transcripts (not union), hours:")
for k, v in sum_by.most_common(): print(f"  {k:8s} {v/3600:8.1f}")
print(f"  late model gaps (>=5 min before an assistant event): {len(late_model)}, {sum(late_model)/3600:.1f} h")
print("\nTool wait by class (summed, hours, calls):")
for k, v in tool_by.most_common(22): print(f"  {k:24s} {v/3600:7.1f} h  {tool_n[k]:6d}")
og = sorted(owner_gaps)
def q(x, p): return x[min(len(x) - 1, int(p * len(x)))]
if og:
    print(f"\nFinished turn -> human message: n={len(og)} median {q(og,.5)/60:.1f} min p75 {q(og,.75)/60:.1f} p90 {q(og,.9)/60:.0f} min; "
          f">1h {sum(1 for x in og if x>3600)}; >8h {sum(1 for x in og if x>CAP)}; sum<=8h {sum(x for x in og if x<=CAP)/3600:.1f} h; sum all {sum(og)/3600:.0f} h")
print("\nPer day: active h (model|tool), owner-only h, nothing h, owner-present clock hours, peak/mean concurrency (top-level sessions)")
hh = collections.defaultdict(set)
for h in human_ts:
    if T0 <= h < T1:
        d = dt.datetime.fromtimestamp(h, tz); hh[d.date()].add(d.hour)
conc = collections.defaultdict(list)
for m, s in minute_sessions.items():
    d = dt.datetime.fromtimestamp(m * 60, tz).date(); conc[d].append(len(s))
rows = []
for d in sorted(per_day):
    c = per_day[d]; act = (c["a model"] + c["b tool"]) / 3600
    cc = conc.get(d, [0])
    rows.append((d, act, c["c owner"] / 3600, c["e nothing"] / 3600, len(hh[d]), max(cc), st.mean(cc)))
    print(f"  {d} {d.strftime('%a')} act {act:5.1f}  owner {c['c owner']/3600:4.1f}  nothing {c['e nothing']/3600:5.1f}  present {len(hh[d]):2d}  peak {max(cc):2d} mean {st.mean(cc):.1f}")
print("\nMeans per day: active %.1f h, nothing %.1f h, owner-present clock hours %.1f, peak conc %.1f, mean conc %.2f" % (
    st.mean(r[1] for r in rows), st.mean(r[3] for r in rows), st.mean(r[4] for r in rows), st.mean(r[5] for r in rows), st.mean(r[6] for r in rows)))
# active by local hour of day
hod = collections.Counter(); hodh = collections.Counter()
for s in range(0, N, 60):
    if arr["model"][s] or arr["tool"][s]:
        hod[dt.datetime.fromtimestamp(T0 + s, tz).hour] += 1
for h in human_ts:
    if T0 <= h < T1: hodh[dt.datetime.fromtimestamp(h, tz).hour] += 1
days = len(rows)
print("\nHour of day: active-minute share / human msgs")
print("  " + " ".join(f"{h:02d}:{100*hod[h]/(60*days):3.0f}%/{hodh[h]}" for h in range(24)))
