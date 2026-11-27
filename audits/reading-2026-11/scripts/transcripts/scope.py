import sys,os; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
import collections
S = load()
rows = []
for s in S:
    if s["end"][:10] < "2026-11-10": continue
    tools = [x for x in s["ev"] if x["k"] == "tool"]
    fe = next((i for i, x in enumerate(tools) if broad_edit(x)), None)
    fs = next((i for i, x in enumerate(tools) if strict_edit(x)), None)
    if fe is None: continue
    nstart = sum(1 for x in tools if x["name"] in ("mcp__ccd_session__start_session",))
    nagent = sum(1 for x in tools if x["name"] in ("Agent", "Task"))
    nsend = sum(1 for x in tools if "send_message" in x["name"] or x["name"] == "SendMessage")
    nedits = sum(1 for x in tools if broad_edit(x))
    rows.append(dict(f=s["file"], short=os.path.basename(s["file"])[:8], sub=s["sub"], start=s["start"][:16], end=s["end"][:16], msgs=s["msgs"], tools=len(tools), fe=fe, fs=fs, fid=tools[fe]["id"], nstart=nstart, nagent=nagent, nsend=nsend, nedits=nedits, flen=len(s["first"])))
print(len(rows), "files with a crates edit, end>=11-10")
by = collections.defaultdict(list)
for r in rows: by[r["fid"]].append(r)
print(len(by), "after fork dedupe by first-edit tool id")
for k, g in sorted(by.items(), key=lambda kv: kv[1][0]["start"]):
    r = max(g, key=lambda r: r["tools"])
    print(r["short"], "SUB" if r["sub"] else "top", r["start"], r["end"], "msgs", r["msgs"], "tools", r["tools"], "fe", r["fe"], "strict", r["fs"], "start_sess", r["nstart"], "agent", r["nagent"], "send", r["nsend"], "edits", r["nedits"], "forks", len(g), "flen", r["flen"])
