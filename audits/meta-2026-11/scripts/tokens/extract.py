"""One pass over every transcript: per API call (deduplicated by message id) and per context item.
Writes calls.jsonl (one row per API call) and files.json (per-file facts). Prints no message text."""
import glob, json, os, re, collections, sys
P = os.path.expanduser("~/.claude/projects/-Users-tom-Development-GitHub-smallcatlabs-acquisition")
OUT = os.path.dirname(os.path.abspath(__file__))

def bash_class(cmd):
    c = cmd.strip()
    c = re.sub(r"^cd [^&;]+(&&|;)\s*", "", c)
    c = re.sub(r"^(\w+=\S+\s+)+", "", c)
    rules = [
        ("gate", r"tools/gate\.sh"),
        ("fixtures", r"ACQ_UPDATE_FIXTURES"),
        ("cargo test", r"cargo\s+(nextest|test)"),
        ("cargo build", r"cargo\s+build"),
        ("cargo other", r"cargo\s+"),
        ("codex", r"\bcodex\b"),
        ("checks/bothy", r"bothy\.py|tools/(docs|arch|trade)-check"),
        ("acq tool", r"(^|[\s/])acq(d|-mcp)?\s"),
        ("git commit/push", r"git\s+(-C \S+\s+)?(commit|push)\b"),
        ("gh", r"(^|\s)gh\s+"),
        ("git read", r"git\s+(-C \S+\s+)?(log|show|diff|status|blame|ls-files|grep|rev-parse|ls-tree)"),
        ("git other", r"^git\s"),
        ("sleep/poll", r"^(sleep|pgrep|until|lsof|ps|wait)\b"),
        ("read/search (cat,grep,sed..)", r"^(grep|rg|find|ls|wc|cat|sed|head|tail|awk|sort|for|while|echo|printf|du|stat|test|\[|nl|diff|jq)\b"),
        ("python", r"^python3?\b"),
        ("sqlite3", r"sqlite3"),
    ]
    for name, pat in rules:
        if re.search(pat, c):
            return name
    return "other shell"

def tool_class(name, inp):
    if name == "Bash":
        return "Bash:" + bash_class(inp.get("command", ""))
    if name in ("Edit", "Write", "MultiEdit", "NotebookEdit"):
        p = inp.get("file_path", "")
        if "/roadmap/reviews/" in p: return "Write:review file"
        return "Edit/Write"
    if name in ("Agent", "Task"): return "Agent (subagent result)"
    if name.startswith("mcp__"): return "MCP:" + name.split("__")[1][:20]
    return name

files = sorted(glob.glob(P + "/**/*.jsonl", recursive=True))
fo = open(os.path.join(OUT, "calls.jsonl"), "w")
filefacts = {}
for fi, f in enumerate(files):
    is_sub = "/subagents/" in f
    rel = f[len(P)+1:]
    sess = rel.split("/")[0].replace(".jsonl", "")
    calls = collections.OrderedDict()
    tid2call = {}
    items = []   # (call_index_at_insertion, kind, bytes)
    compacts = []
    shape = None
    errors = []
    first_human = None
    cost_last = None
    ncalls = 0
    def cur(): return len(calls)
    for line in open(f, errors="replace"):
        if line.startswith('{"type":"api-request-blob"') or '"type":"api-request"' in line[:40]:
            continue
        try: d = json.loads(line)
        except Exception: continue
        t = d.get("type")
        if t == "api-request-shape" and shape is None:
            sh = d.get("shape") or {}
            tools = sh.get("tools") or []
            shape = {"system": len(json.dumps(sh.get("system", ""))), "tools": len(json.dumps(tools)),
                     "ntools": len(tools),
                     "big": sorted(((len(json.dumps(x)), x.get("name", "?")) for x in tools), reverse=True)[:6]}
            continue
        if t == "cost-state":
            cost_last = d.get("totalCostUSD"); continue
        if t == "system":
            st = d.get("subtype")
            if st == "compact_boundary":
                cm = d.get("compactMetadata") or {}
                compacts.append((cur(), cm.get("trigger"), cm.get("preTokens"), cm.get("postTokens"), d.get("timestamp")))
                items.append((cur(), "__compact__", 0))
            elif st == "api_error":
                errors.append(("api_error", d.get("timestamp"), str(d.get("content", ""))[:0], json.dumps(d.get("error", d.get("cause", "")))[:160]))
            continue
        if t == "attachment":
            a = d.get("attachment") or {}
            r = d.get("rendered")
            if not r: continue   # not rendered into the context (e.g. prompt_snapshot, a local record of the prefix)
            size = sum(len(x.get("content", "")) if isinstance(x, dict) and isinstance(x.get("content"), str) else len(json.dumps(x)) for x in r) if isinstance(r, list) else len(json.dumps(r))
            items.append((cur(), "attach:" + str(a.get("type")), size))
            continue
        if t == "assistant":
            m = d.get("message") or {}
            mid = m.get("id") or d.get("uuid")
            if d.get("isApiErrorMessage"):
                txt = json.dumps(m.get("content"))
                kind = "usage-limit" if re.search(r"(usage|weekly|session) limit|limit reached|resets", txt, re.I) else ("rate-limit" if re.search(r"rate.?limit|429|overloaded|529", txt, re.I) else "other")
                errors.append(("api_error_msg", d.get("timestamp"), kind, ""))
                continue
            u = m.get("usage") or {}
            if mid not in calls:
                calls[mid] = {"id": mid, "i": len(calls), "model": m.get("model"), "ts": d.get("timestamp"),
                              "in": u.get("input_tokens", 0) or 0, "cr": u.get("cache_read_input_tokens", 0) or 0,
                              "cc": u.get("cache_creation_input_tokens", 0) or 0, "out": u.get("output_tokens", 0) or 0,
                              "tools": [], "text": 0}
            c = calls[mid]
            for b in (m.get("content") or []):
                if not isinstance(b, dict): continue
                if b.get("type") == "tool_use":
                    inp = b.get("input") or {}
                    tc = tool_class(b.get("name", "?"), inp)
                    c["tools"].append(tc)
                    tid2call[b.get("id")] = tc
                    items.append((c["i"] + 1, "model output (tool calls)", len(json.dumps(inp))))
                elif b.get("type") in ("text", "thinking"):
                    s = len(b.get("text", "") or b.get("thinking", "") or "")
                    c["text"] += s
                    items.append((c["i"] + 1, "model output (text/thinking)", s))
            continue
        if t == "user":
            m = d.get("message") or {}
            content = m.get("content")
            if d.get("isCompactSummary"):
                items.append((cur(), "compact summary", len(json.dumps(content)))); continue
            if isinstance(content, str):
                if first_human is None and not d.get("isMeta"): first_human = content
                items.append((cur(), "human/meta text", len(content))); continue
            for b in (content or []):
                if not isinstance(b, dict): continue
                if b.get("type") == "tool_result":
                    cc_ = b.get("content")
                    size = sum(len(x.get("text", "")) if x.get("type") == "text" else 1500*0 + len(json.dumps(x))//4 for x in cc_ if isinstance(x, dict)) if isinstance(cc_, list) else len(cc_ or "")
                    items.append((cur(), "result " + tid2call.get(b.get("tool_use_id"), "?"), size))
                elif b.get("type") == "text":
                    if first_human is None and not d.get("isMeta"): first_human = b.get("text", "")
                    items.append((cur(), "human/meta text", len(b.get("text", ""))))
                else:
                    items.append((cur(), "other block", len(json.dumps(b))))
    n = len(calls)
    # occupancy weight: an item inserted before call k (0-based index k) is read by calls k..(next compact-1)
    occ = collections.Counter(); raw = collections.Counter()
    comp_pos = sorted(p for p, k, _ in items if k == "__compact__") if False else sorted(c[0] for c in compacts)
    import bisect
    for pos, kind, size in items:
        if kind == "__compact__": continue
        j = bisect.bisect_right(comp_pos, pos - 1) if False else bisect.bisect_left(comp_pos, pos + 0)
        # first compaction at or after pos ends the item's life (items inserted at pos == compaction pos are post-compact)
        end = n
        for cp in comp_pos:
            if cp > pos: end = cp; break
        live = max(0, end - pos)
        occ[kind] += size * live; raw[kind] += size
    if shape: occ["fixed prefix (system + tool schemas)"] += (shape["system"] + shape["tools"]) * n; raw["fixed prefix (system + tool schemas)"] += shape["system"] + shape["tools"]
    first_items = collections.Counter()
    for pos, kind, size in items:
        if pos == 0 and kind != "__compact__": first_items[kind] += size
    for c in calls.values():
        c2 = dict(c); c2["file"] = rel; c2["sub"] = is_sub; c2["sess"] = sess; c2["n"] = n
        fo.write(json.dumps(c2) + "\n")
    # role guess from the first human prompt (keywords only; no text kept)
    fh = (first_human or "").lower()
    role = "?"
    for r, pat in [("seat", r"\bseat\b|blind agent"), ("review handler", r"cold.?review|review (answer|finding)|reviewer'?s? (answer|finding)|answer the findings"),
                   ("audit/research", r"\baudit|investigat|research|measure|read-only"), ("orchestrator", r"orchestrat|\bworkers?\b|drive the step|dispatch"),
                   ("builder", r"\bbrief\b|implement|build|step \d|fix\b|close"), ("design", r"design|ruling|propos|plan")]:
        if re.search(pat, fh): role = r; break
    filefacts[rel] = {"sub": is_sub, "sess": sess, "n": n, "shape": shape, "compacts": compacts, "errors": errors,
                      "cost": cost_last, "occ": dict(occ), "raw": dict(raw), "role": role, "first_items": dict(first_items), "fh_len": len(first_human or "")}
    if fi % 100 == 0: print(fi, file=sys.stderr)
json.dump(filefacts, open(os.path.join(OUT, "files.json"), "w"))
print("files", len(files))
