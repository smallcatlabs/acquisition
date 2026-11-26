"""Extract the owner's typed messages from top-level session transcripts.
Writes msgs.jsonl (ts, session, text, origin_kind) next to this script."""
import json, glob, os, re, sys
P = os.path.expanduser("~/.claude/projects/-Users-tom-Development-GitHub-smallcatlabs-acquisition/")
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "msgs.jsonl")
SR = re.compile(r"<system-reminder>.*?</system-reminder>", re.S)
DROP_PREFIX = ("This session is being continued", "<command-", "<local-command", "<task-notification",
               "Another Claude session", "Base directory for this skill", "Background (from the session",
               "[Request interrupted", "Caveat:", "<bash-", "<user-memory-input")
seen = set(); n = 0; stats = {"kept":0, "drop":0}
with open(OUT, "w") as o:
    for f in sorted(glob.glob(P + "*.jsonl")):
        sid = os.path.basename(f)[:8]
        for l in open(f):
            try: e = json.loads(l)
            except Exception: continue
            if e.get("type") != "user" or "toolUseResult" in e or e.get("isMeta") or e.get("isSidechain"): continue
            org = e.get("origin") or {}
            if org.get("kind") not in (None, "human"): stats["drop"] += 1; continue
            m = e["message"]["content"]
            if isinstance(m, list):
                if any(isinstance(b, dict) and b.get("type") == "tool_result" for b in m): continue
                t = "\n".join(b.get("text", "") for b in m if isinstance(b, dict) and b.get("type") == "text")
            else: t = m
            t = SR.sub("", t).strip()
            if "## The user forked this session" in t: continue
            if not t or t.startswith(DROP_PREFIX): stats["drop"] += 1; continue
            key = (e.get("uuid"),)
            if key in seen: continue
            seen.add(key)
            # dedupe identical text at same timestamp minute (resumed/forked sessions copy history)
            k2 = (t[:300], e.get("timestamp", "")[:16])
            if k2 in seen: continue
            seen.add(k2)
            o.write(json.dumps({"ts": e.get("timestamp"), "sid": sid, "text": t, "origin": org.get("kind"), "src": e.get("promptSource")}) + "\n")
            stats["kept"] += 1
print(stats, OUT)
