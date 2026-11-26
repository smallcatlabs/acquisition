#!/usr/bin/env python3
"""Extract a compact event stream from every transcript (no message text kept).

Run: python3 -I extract.py <transcript dir> <out.json>
Each file -> {"sid","parent","sub","ev":[[t,kind,payload],...]}
kinds: A assistant event, R tool result, H human-typed, N task-notification,
P injected by another session/coordinator/peer, S system/meta user msg, C compaction continuation.
"""
import glob, json, os, re, sys, datetime as dt

P, OUT = sys.argv[1], sys.argv[2]

def ts(s):
    try: return dt.datetime.fromisoformat(s.replace("Z", "+00:00")).timestamp()
    except Exception: return None

def bash_class(cmd):
    c = cmd.strip()
    c = re.sub(r"^cd [^&;]+(&&|;)\s*", "", c)
    c = re.sub(r"^(\w+=\S+\s+)+", "", c)
    rules = [
        ("gate --docs", r"tools/gate\.sh\s+--docs"),
        ("gate full", r"tools/gate\.sh"),
        ("nextest/test", r"cargo\s+(nextest|test)"),
        ("cargo build", r"cargo\s+build"),
        ("cargo clippy/fmt/doc", r"cargo\s+(clippy|fmt|doc)"),
        ("cargo other", r"cargo\s+"),
        ("codex", r"\bcodex\b"),
        ("docs/arch/trade/bothy", r"tools/(docs|arch|trade)-check|bothy\.py"),
        ("acq tool", r"(^|[\s/])acq(-mcp|d)?\s"),
        ("gh", r"^gh\s|\sgh\s+(issue|pr|run|api)"),
        ("git commit/push", r"git\s+(-C \S+\s+)?(commit|push)"),
        ("git other", r"(^|\s)git\s"),
        ("sleep/poll", r"^(sleep|pgrep|until|while\s.*sleep|lsof|ps)\b|\bsleep\s+\d"),
        ("shell read", r"^(grep|rg|find|ls|wc|cat|sed|head|tail|awk|sort|for|echo|printf|du|stat|test|\[|diff|jq)\b"),
        ("python", r"^python3?\b"),
    ]
    for name, pat in rules:
        if re.search(pat, c): return name
    return "shell other"

def norm(cmd):
    c = re.sub(r"\s+", " ", cmd.strip())
    c = re.sub(r"/private/tmp/\S+", "TMP", c)
    return c[:200]

INJ = ("Background (from", "<side-sessions", "<cross-session", "Another Claude", "Task (from the sessi")
out = []
files = glob.glob(P + "/**/*.jsonl", recursive=True)
for f in files:
    sub = "/subagents/" in f
    rel = os.path.relpath(f, P)
    sid = rel.split("/")[0].replace(".jsonl", "")
    ev = []
    for line in open(f, errors="replace"):
        if '"type":"user"' not in line and '"type":"assistant"' not in line:
            continue
        try: e = json.loads(line)
        except Exception: continue
        ty = e.get("type"); t = ts(e.get("timestamp", ""))
        if t is None or ty not in ("user", "assistant"): continue
        msg = e.get("message") or {}
        cont = msg.get("content")
        if ty == "assistant":
            tus = []; has_text = False
            for b in cont or []:
                if b.get("type") == "tool_use":
                    inp = b.get("input") or {}
                    name = b.get("name")
                    cls = name; cmd = None; bg = False
                    if name == "Bash":
                        cmd = inp.get("command", ""); cls = bash_class(cmd); bg = bool(inp.get("run_in_background"))
                    elif name in ("Agent", "Task"):
                        cls = "subagent"
                    tus.append([b.get("id"), name, cls, norm(cmd) if cmd else None, bg])
                elif b.get("type") == "text" and (b.get("text") or "").strip():
                    has_text = True
            ev.append([t, "A", {"tu": tus, "txt": has_text, "m": msg.get("model"), "sr": msg.get("stop_reason")}])
            continue
        # user
        if isinstance(cont, list) and any(isinstance(b, dict) and b.get("type") == "tool_result" for b in cont):
            tur = e.get("toolUseResult")
            bgid = None
            if isinstance(tur, dict):
                bgid = tur.get("backgroundTaskId") or (tur.get("agentId") if tur.get("isAsync") else None)
            for b in cont:
                if isinstance(b, dict) and b.get("type") == "tool_result":
                    ev.append([t, "R", {"id": b.get("tool_use_id"), "err": bool(b.get("is_error")), "bg": bgid}])
            continue
        txt = cont if isinstance(cont, str) else " ".join(str(b.get("text", "")) for b in (cont or []) if isinstance(b, dict))
        o = e.get("origin") or {}
        ok = o.get("kind") if isinstance(o, dict) else None
        body = re.sub(r"(?s)<system-reminder>.*?</system-reminder>\s*", "", txt).lstrip()
        if ok == "human":
            k = "H"
        elif ok == "task-notification" or body.startswith("<task-notification") or body.startswith("[SYSTEM NOTIFI"):
            k = "N"
        elif ok in ("peer", "coordinator") or body.startswith(INJ) or body.startswith("<agent-message"):
            k = "P"
        elif body.startswith("This session is being continued"):
            k = "C"
        elif body.startswith("[Request interrupted"):
            k = "I"
        else:
            k = "S"
        pl = {}
        if k == "N":
            m = re.search(r"<task-id>([^<]+)</task-id>", body); pl["tid"] = m.group(1) if m else None
        if k == "H":
            pl["bash"] = body.startswith("<bash-input")
        ev.append([t, k, pl])
    ev.sort(key=lambda x: x[0])
    out.append({"sid": sid, "file": rel, "sub": sub, "ev": ev})
json.dump(out, open(OUT, "w"))
print(len(out), "files", sum(len(o["ev"]) for o in out), "events")
