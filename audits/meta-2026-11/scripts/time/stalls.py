#!/usr/bin/env python3
"""Stalls and loops. Run: python3 -I stalls.py events.json [since-date]"""
import json, sys, collections, datetime as dt
data = json.load(open(sys.argv[1])); SINCE = dt.datetime.fromisoformat(sys.argv[2] if len(sys.argv)>2 else "2026-10-12").timestamp()
bysid = collections.defaultdict(list)
for f in data: bysid[f["sid"]].append(f)
rep = collections.Counter(); rep_ex = []; red_chains = []; long_sess = []; nocommit = []; sleep_runs = []; sleep_sess = collections.Counter()
reruns_after_fail = 0
for sid, fs in bysid.items():
    top = [f for f in fs if not f["sub"]]
    if not top or not top[0]["ev"] or top[0]["ev"][0][0] < SINCE: continue
    committed = False; nA = 0
    for f in fs:
        ev = f["ev"]; cls_of = {}; cmd_of = {}
        bash_seq = []   # (cmd, cls, err, t)
        for t, k, p in ev:
            if k == "A":
                if not f["sub"]: nA += 1
                for tid, name, cls, cmd, bg in p["tu"]:
                    cls_of[tid] = cls; cmd_of[tid] = cmd
            if k == "R" and p["id"] in cmd_of and cmd_of[p["id"]]:
                c = cmd_of[p["id"]]; cl = cls_of[p["id"]]
                bash_seq.append((c, cl, p["err"], t))
                if cl == "git commit/push" and "commit" in c and not p["err"]: committed = True
        # identical command 3+ in a row
        i = 0
        while i < len(bash_seq):
            j = i
            while j + 1 < len(bash_seq) and bash_seq[j + 1][0] == bash_seq[i][0]: j += 1
            if j - i + 1 >= 3:
                rep[bash_seq[i][1]] += 1
                rep_ex.append((j - i + 1, bash_seq[i][1], sid[:8], dt.date.fromtimestamp(bash_seq[i][3]).isoformat(), (bash_seq[j][3]-bash_seq[i][3])/60))
            i = j + 1
        # full gate red chains: consecutive full-gate runs (other commands between) while red
        chain = 0; start = None
        for c, cl, err, t in bash_seq:
            if cl == "gate full":
                if err:
                    chain += 1; start = start or t
                else:
                    if chain: red_chains.append((chain, sid[:8], dt.date.fromtimestamp(start).isoformat(), (t-start)/60))
                    chain = 0; start = None
        if chain: red_chains.append((chain, sid[:8], dt.date.fromtimestamp(start).isoformat(), -1))
        # sleep/poll runs
        run = 0; rs = None; tot = 0
        for c, cl, err, t in bash_seq + [("", "", False, 0)]:
            if cl == "sleep/poll":
                run += 1; rs = rs or t
            else:
                if run >= 5: sleep_runs.append((run, sid[:8], dt.date.fromtimestamp(rs).isoformat()))
                run = 0; rs = None
    if nA > 300: long_sess.append((nA, sid[:8], dt.date.fromtimestamp(top[0]["ev"][0][0]).isoformat()))
    if not committed: nocommit.append(sid)
tops = sum(1 for s, fs in bysid.items() if any(not f["sub"] and f["ev"] and f["ev"][0][0] >= SINCE for f in fs))
print(f"top-level sessions: {tops}")
print(f"same command 3+ times in a row: {len(rep_ex)} runs; by class {dict(rep.most_common(8))}")
for x in sorted(rep_ex, reverse=True)[:5]: print("   ", x)
lens = collections.Counter(min(c[0], 5) for c in red_chains)
print(f"full-gate red chains: {len(red_chains)}; by length (5=5+) {dict(sorted(lens.items()))}")
for x in sorted(red_chains, reverse=True)[:5]: print("   ", x)
print(f"sessions over 300 assistant events: {len(long_sess)}")
for x in sorted(long_sess, reverse=True)[:5]: print("   ", x)
print(f"sessions with no successful git commit (incl. their subagents): {len(nocommit)}")
print(f"sleep/poll runs of 5+ consecutive calls: {len(sleep_runs)}")
for x in sorted(sleep_runs, reverse=True)[:5]: print("   ", x)
