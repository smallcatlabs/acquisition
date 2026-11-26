#!/usr/bin/env python3
"""Lead time per closed step: first related commit -> close commit.
Run from the repo: python3 -I leadtime.py 15
Related: subject names the slug, or an issue (#N) the close names, or the commit touches
roadmap/briefs/<slug>*.md or roadmap/reviews/<slug>*; looked for within 14 days before the close.
Also counts how many related commits are reviews:/briefs:/close (process) vs other."""
import subprocess, re, sys, statistics as st, datetime as dt
K = int(sys.argv[1]) if len(sys.argv) > 1 else 15
def git(*a): return subprocess.run(["git", *a], capture_output=True, text=True).stdout
closes = [l.split("\x1f") for l in git("log", "--grep=^Step: ", "--format=%H\x1f%ct\x1f%s").strip().split("\n")]
allc = git("log", "--since=2026-10-01", "--name-only", "--format=@@%H\x1f%ct\x1f%s").split("@@")[1:]
commits = []
for blk in allc:
    head, *files = blk.strip().split("\n")
    h, t, s = head.split("\x1f"); commits.append((h, int(t), s, [f for f in files if f]))
import json, os
EVP = os.environ.get("EVENTS")
act = []
if EVP:
    for f in json.load(open(EVP)):
        ev = f["ev"]
        for i in range(len(ev) - 1):
            if ev[i + 1][0] - ev[i][0] < 300 and ev[i+1][1] in ("A","R"): act.append((ev[i][0], ev[i + 1][0]))
    act.sort()
def active_h(a, b):
    tot = 0; cur = None
    for x, y in act:
        x = max(x, a); y = min(y, b)
        if y <= x: continue
        if cur and x <= cur[1]: cur = (cur[0], max(cur[1], y))
        else:
            if cur: tot += cur[1] - cur[0]
            cur = (x, y)
    if cur: tot += cur[1] - cur[0]
    return tot / 3600
rows = []
for h, t, s in closes[:K]:
    t = int(t)
    m = re.match(r"(?:close: )?([\w.-]+?)(?::| closed)", s)
    slug = m.group(1) if m else None
    if slug == "roadmap": slug = None
    issues = set(re.findall(r"#(\d+)", s))
    pats = []
    if slug: pats.append(re.compile(re.escape(slug)))
    for i in issues: pats.append(re.compile(r"#%s\b" % i))
    rel = []
    for ch, ct, cs, files in commits:
        if ct > t or ct < t - 14 * 86400: continue
        hit = any(p.search(cs) for p in pats) or (slug and any(slug in f for f in files if f.startswith("roadmap/")))
        if hit: rel.append((ct, cs))
    if not rel: continue
    first = min(c[0] for c in rel)
    proc = sum(1 for _, cs in rel if re.match(r"(reviews|briefs|roadmap|close)\b|.*closed —", cs))
    rows.append(((t - first) / 3600, active_h(first, t), len(rel), proc, dt.datetime.fromtimestamp(t).strftime("%m-%d"), (slug or "#" + ",".join(sorted(issues)))[:28]))
rows = [r for r in rows if r[5] != "close"]
for r in rows: print(f"  {r[4]} {r[5]:28s} lead {r[0]:6.1f} h  machine-active {r[1]:5.1f} h  commits {r[2]:3d}  of which brief/review/close {r[3]}")
L = [r[0] for r in rows]; C = [r[2] for r in rows]; AH = [r[1] for r in rows]
print(f"machine-active within lead: median {st.median(AH):.1f} h range {min(AH):.1f}-{max(AH):.1f}")
print(f"n={len(rows)} lead median {st.median(L):.1f} h range {min(L):.1f}-{max(L):.1f}; commits median {st.median(C)} range {min(C)}-{max(C)}; process share {sum(r[3] for r in rows)/sum(C):.0%}")
