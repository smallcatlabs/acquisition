"""Where do usage-limit phrases occur: event type, flags, timestamp. Prints no text."""
import glob, json, os, re, collections
P = os.path.expanduser("~/.claude/projects/-Users-tom-Development-GitHub-smallcatlabs-acquisition")
pat = re.compile(r"hit your|out of extra usage|Opus limit|weekly limit|usage limit|limit reached|resets at", re.I)
out = collections.Counter(); when = []
for f in glob.glob(P + "/**/*.jsonl", recursive=True):
    for l in open(f, errors="replace"):
        if l.startswith('{"type":"api-request') or not pat.search(l): continue
        try: d = json.loads(l)
        except: continue
        t = d.get("type"); m = d.get("message") or {}
        c = m.get("content")
        kind = t
        if t == "user":
            if isinstance(c, list) and any(isinstance(b, dict) and b.get("type") == "tool_result" for b in c): kind = "user:tool_result"
            elif d.get("isMeta"): kind = "user:meta"
            else: kind = "user:human"
        if t == "assistant": kind = "assistant:" + ("apierror" if d.get("isApiErrorMessage") else ("synthetic" if m.get("model") == "<synthetic>" else "model"))
        if t == "system": kind = "system:" + str(d.get("subtype"))
        out[kind] += 1
        if kind in ("assistant:apierror", "assistant:synthetic", "system:api_error", "system:informational", "user:meta") or kind.startswith("system"):
            when.append((d.get("timestamp", "")[:16], kind, "/subagents/" in f))
print(out.most_common()); print(sorted(when))
