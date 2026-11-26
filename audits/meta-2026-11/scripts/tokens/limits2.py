"""Dates of human/meta messages naming a usage limit (no text printed), and activity edges around idle days."""
import glob, json, os, re, collections
P = os.path.expanduser("~/.claude/projects/-Users-tom-Development-GitHub-smallcatlabs-acquisition")
pat = re.compile(r"hit your|out of extra usage|Opus limit|weekly limit|usage limit|limit reached|resets at", re.I)
for f in glob.glob(P + "/*.jsonl"):
    for l in open(f, errors="replace"):
        if l.startswith('{"type":"api-request') or not pat.search(l): continue
        d = json.loads(l)
        if d.get("type") == "user":
            c = (d.get("message") or {}).get("content")
            if isinstance(c, list) and any(isinstance(b, dict) and b.get("type") == "tool_result" for b in c): continue
            s = c if isinstance(c, str) else json.dumps(c)
            m = pat.search(s)
            print(d.get("timestamp", "")[:16], "meta" if d.get("isMeta") else "human", "match:", m.group(0) if m else None, "starts-with-cmd:", s.strip()[:22].startswith("<local-command") or "<command-name>" in s[:200])
calls = [json.loads(l) for l in open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "calls.jsonl"))]
ts = sorted(c["ts"] for c in calls)
import datetime as dt
p = None
for t in ts:
    x = dt.datetime.fromisoformat(t.replace("Z", "+00:00"))
    if p and (x - p).total_seconds() > 14 * 3600: print("idle", p.isoformat()[:16], "->", x.isoformat()[:16], round((x - p).total_seconds() / 3600), "h")
    p = x
