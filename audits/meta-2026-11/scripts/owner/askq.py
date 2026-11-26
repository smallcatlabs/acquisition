"""Every AskUserQuestion call in top-level sessions and the answer that came back. Writes askq.jsonl."""
import json, glob, os
P = os.path.expanduser("~/.claude/projects/-Users-tom-Development-GitHub-smallcatlabs-acquisition/")
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "askq.jsonl")
calls = {}; res = {}
for f in sorted(glob.glob(P + "*.jsonl")):
    sid = os.path.basename(f)[:8]
    for l in open(f):
        try: e = json.loads(l)
        except Exception: continue
        c = (e.get("message") or {}).get("content")
        if not isinstance(c, list): continue
        for b in c:
            if not isinstance(b, dict): continue
            if b.get("type") == "tool_use" and b.get("name") == "AskUserQuestion":
                calls[b["id"]] = {"ts": e.get("timestamp"), "sid": sid, "input": b["input"]}
            if b.get("type") == "tool_result" and b.get("tool_use_id") in calls or (b.get("type") == "tool_result" and b.get("tool_use_id","").startswith("toolu")):
                if b.get("tool_use_id") in calls:
                    cc = b.get("content")
                    if isinstance(cc, list): cc = " ".join(x.get("text","") for x in cc if isinstance(x, dict))
                    res[b["tool_use_id"]] = {"ts": e.get("timestamp"), "text": cc, "tur": e.get("toolUseResult")}
with open(OUT, "w") as o:
    for k, v in calls.items():
        v["result"] = res.get(k); o.write(json.dumps(v) + "\n")
print(len(calls), sum(1 for k in calls if k in res))
