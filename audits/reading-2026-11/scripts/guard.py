"""The guard's numbers: serious findings per first build review, and
review rounds per step, for steps whose first build review was saved
in a window.

usage, from the repository's root:
    mkdir -p /tmp/reading-2026-11/findings
    python3 -I audits/reading-2026-11/scripts/findings/reviews.py
    python3 -I audits/reading-2026-11/scripts/guard.py 2026-11-10 [until]

A serious finding is one marked P1 or P2, counted by `reviews.py`'s
pattern over the saved review's text. Reviews of briefs are left out.
"""
import collections, json, statistics, sys
since = sys.argv[1]
until = sys.argv[2] if len(sys.argv) > 2 else "9999"
rows = json.load(open("/tmp/reading-2026-11/findings/rows.json"))
steps = collections.defaultdict(list)
for r in rows:
    if r["kind"] != "brief":
        steps[r["step"]].append(r)
picked = []
for step, rs in steps.items():
    rs.sort(key=lambda r: r["round"])
    first = rs[0]
    if first["kind"] != "build" or not (since <= first["added"][:10] < until):
        continue
    picked.append((step, first["p1"] + first["p2"], first["p1"], len(rs)))
n = len(picked)
serious = [p[1] for p in picked]
rounds = [p[3] for p in picked]
print(f"steps whose first build review was saved from {since}" + (f" to {until}" if until != "9999" else "") + f": {n}")
print(f"serious findings in the first build review: total {sum(serious)}, mean {sum(serious)/n:.2f}, median {statistics.median(serious)}")
print(f"first build reviews with no serious finding: {sum(1 for s in serious if s == 0)} of {n}")
print(f"first build reviews with a P1: {sum(1 for p in picked if p[2])} of {n}")
print(f"reviews per step (the build's and its fix rounds'): mean {sum(rounds)/n:.2f}, median {statistics.median(rounds)}")
print("steps by reviews taken: " + ", ".join(f"{k} review{'s' if k > 1 else ''}: {v}" for k, v in sorted(collections.Counter(rounds).items())))
