"""Walk each top-level session in order; for every owner-typed message (origin=human), find the
last wrap_up (ask + actions) the assistant issued before it, and the last assistant text.
Classify the reply: button 1 (recommended), button 2, typed reply to an ask, or no pending ask.
Writes turns.jsonl."""
import json, glob, os, re, collections, datetime as dt
P = os.path.expanduser("~/.claude/projects/-Users-tom-Development-GitHub-smallcatlabs-acquisition/")
D = os.path.dirname(os.path.abspath(__file__))
SR = re.compile(r"<system-reminder>.*?</system-reminder>", re.S)
out = open(os.path.join(D, "turns.jsonl"), "w")
seen = set()
C = collections.Counter()
for f in sorted(glob.glob(P + "*.jsonl")):
    sid = os.path.basename(f)[:8]
    pend = None; last_text = ""; last_ts = None
    for l in open(f):
        try: e = json.loads(l)
        except Exception: continue
        t = e.get("type")
        if t == "assistant":
            for b in e["message"].get("content", []):
                if not isinstance(b, dict): continue
                if b.get("type") == "tool_use" and b.get("name", "").endswith("wrap_up"):
                    i = b["input"]; acts = i.get("actions") or []
                    if isinstance(acts, str):
                        try: acts = json.loads(acts)
                        except Exception: acts = []
                    labels = [a.get("label") or a.get("action") for a in acts if isinstance(a, dict)]
                    pend = {"ask": i.get("ask"), "labels": labels, "ts": e.get("timestamp")}
                    last_text = ""
                elif b.get("type") == "text":
                    last_text = b.get("text", ""); last_ts = e.get("timestamp")
            continue
        if t != "user" or "toolUseResult" in e or e.get("isMeta"): continue
        org = e.get("origin") or {}
        if org.get("kind") != "human": continue
        m = e["message"]["content"]
        if isinstance(m, list):
            if any(isinstance(b, dict) and b.get("type") == "tool_result" for b in m): continue
            m = "\n".join(b.get("text", "") for b in m if isinstance(b, dict) and b.get("type") == "text")
        m = SR.sub("", m).strip()
        if not m or "## The user forked" in m or m.startswith(("<command-", "<local-command")): continue
        k = (m[:300], (e.get("timestamp") or "")[:16])
        if k in seen: continue
        seen.add(k)
        rec = {"ts": e.get("timestamp"), "sid": sid, "text": m, "src": e.get("promptSource")}
        endq = last_text.strip().endswith("?") or bool(re.search(r"\?\s*$", last_text.strip()[-300:]))
        if pend and pend.get("labels"):
            lab = [x.lower().strip(" .") for x in pend["labels"] if x]
            mm = m.lower().strip(" .")
            if lab and mm == lab[0]: kind = "button1"
            elif mm in lab: kind = "button2"
            elif pend.get("ask"): kind = "typed-to-ask"
            else: kind = "after-recap"
            rec.update(ask=pend.get("ask"), labels=pend["labels"], ask_ts=pend["ts"])
        elif pend and pend.get("ask"):
            kind = "typed-to-ask"; rec.update(ask=pend.get("ask"), ask_ts=pend["ts"])
        elif endq: kind = "reply-to-text-q"; rec["q"] = last_text.strip()[-400:]; rec["ask_ts"] = last_ts
        else: kind = "unprompted"
        rec["kind"] = kind; C[kind] += 1
        rec["prev"] = last_text.strip()[-600:]
        out.write(json.dumps(rec) + "\n")
        pend = None; last_text = ""
print(C, sum(C.values()))
