"""Extract per-session event lists for the governance-read audit. Writes events.json beside the scratch dir."""
import glob, json, os, re, sys
B = os.path.expanduser("~/.claude/projects/")
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "events.json")
SINCE = sys.argv[1] if len(sys.argv) > 1 else "2026-10-20"

def rtext(c):
    if isinstance(c, str): return c
    if isinstance(c, list):
        return "".join(b.get("text", "") if isinstance(b, dict) else str(b) for b in c)
    return ""

sessions = []
for d in glob.glob(B + "-Users-tom-Development-GitHub-smallcatlabs-acquisition*"):
    for f in glob.glob(d + "/**/*.jsonl", recursive=True):
        ev = []; results = {}; usage_seen = set(); cum = 0; first_prompt = None; start = end = None
        human_msgs = 0
        try:
            lines = open(f).readlines()
        except Exception:
            continue
        for l in lines:
            try: e = json.loads(l)
            except Exception: continue
            t = e.get("timestamp")
            if t: start = start or t; end = t
            m = e.get("message") or {}
            ty = e.get("type")
            if ty == "assistant":
                u = m.get("usage") or {}
                mid = m.get("id")
                ctx = (u.get("input_tokens") or 0) + (u.get("cache_read_input_tokens") or 0) + (u.get("cache_creation_input_tokens") or 0)
                if mid and mid not in usage_seen and u:
                    usage_seen.add(mid); cum += ctx
                for b in m.get("content") or []:
                    if isinstance(b, dict) and b.get("type") == "tool_use":
                        ev.append({"k": "tool", "id": b.get("id"), "name": b.get("name"), "input": b.get("input") or {}, "t": t, "ctx": ctx, "cum": cum})
            elif ty == "user":
                c = m.get("content")
                if isinstance(c, list):
                    for b in c:
                        if isinstance(b, dict) and b.get("type") == "tool_result":
                            txt = rtext(b.get("content"))
                            results[b.get("tool_use_id")] = (len(txt.encode()), txt[:4000], txt[-3000:], b.get("is_error"))
                txt = c if isinstance(c, str) else "".join(b.get("text", "") for b in (c or []) if isinstance(b, dict) and b.get("type") == "text")
                if txt: txt = re.sub(r"<system-reminder>.*?</system-reminder>", "", txt, flags=re.S).strip()
                if txt and not e.get("isMeta") and not e.get("isCompactSummary") and not txt.startswith("<"):
                    if first_prompt is None: first_prompt = txt
                    human_msgs += 1
                    ev.append({"k": "msg", "t": t, "len": len(txt), "text": txt[:6000]})
        if not end or end[:10] < SINCE: continue
        for x in ev:
            if x["k"] == "tool" and x["id"] in results:
                x["rb"], x["rhead"], x["rtail"], x["err"] = results[x["id"]]
        sessions.append({"file": f, "start": start, "end": end, "sub": "/subagents/" in f, "msgs": human_msgs, "first": first_prompt or "", "ev": ev})
json.dump(sessions, open(OUT, "w"))
print(len(sessions), "sessions ->", OUT)
