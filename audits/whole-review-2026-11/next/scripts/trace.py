#!/usr/bin/env python3
"""What agents in this repository actually did: a count over local session transcripts.

Reads every *.jsonl under the project's transcript directory (sessions and their
subagents) and prints tables. Reads only; prints no message text.

Run as: ACQ_TRANSCRIPTS=<the harness's transcript directory for this project> ACQ_REPO=<the clone> python3 -I trace.py 2026-10-12
"""
import collections
import datetime as dt
import glob
import json
import os
import re
import sys

P = os.path.expanduser(
    os.environ.get("ACQ_TRANSCRIPTS", "transcripts")
)
ROOT = os.path.abspath(os.environ.get("ACQ_REPO", ".")) + "/"
SINCE = sys.argv[1] if len(sys.argv) > 1 else "2026-10-12"

files = [f for f in glob.glob(P + "/**/*.jsonl", recursive=True)]


def ts(s):
    try:
        return dt.datetime.fromisoformat(s.replace("Z", "+00:00"))
    except Exception:
        return None


def rel(p):
    if not p:
        return "?"
    p = re.sub("^" + re.escape(ROOT.rstrip("/")) + r"(-[^/]+|/\.claude/worktrees/[^/]+)?/", "", p)
    p = p.replace(ROOT, "")
    return p


def bash_class(cmd):
    c = cmd.strip()
    c = re.sub(r"^cd [^&;]+(&&|;)\s*", "", c)
    c = re.sub(r"^(\w+=\S+\s+)+", "", c)
    rules = [
        ("gate.sh --docs", r"tools/gate\.sh\s+--docs"),
        ("gate.sh (full)", r"tools/gate\.sh"),
        ("cargo nextest/test", r"cargo\s+(nextest|test)"),
        ("cargo build", r"cargo\s+build"),
        ("cargo clippy", r"cargo\s+clippy"),
        ("cargo fmt", r"cargo\s+fmt"),
        ("cargo other", r"cargo\s+"),
        ("codex", r"\bcodex\b"),
        ("update fixtures", r"ACQ_UPDATE_FIXTURES"),
        ("bothy", r"bothy\.py"),
        ("docs/arch/trade check", r"tools/(docs|arch|trade)-check"),
        ("acq (the tool)", r"(^|[\s/])acq(-mcp)?\s"),
        ("gh", r"^gh\s|\sgh\s+(issue|pr|run|api)"),
        ("git commit", r"git\s+(-C \S+\s+)?commit"),
        ("git push", r"git\s+(-C \S+\s+)?push"),
        ("git log/show/diff/status", r"git\s+(-C \S+\s+)?(log|show|diff|status|blame|ls-files|grep|rev-parse|ls-tree)"),
        ("git other", r"^git\s"),
        ("grep/rg/find/ls/wc/cat/sed", r"^(grep|rg|find|ls|wc|cat|sed|head|tail|awk|sort|for|while|echo|printf|du|stat|test|\[)\b"),
        ("python", r"^python3?\b"),
        ("sqlite3", r"sqlite3"),
        ("sleep/pgrep/wait", r"^(sleep|pgrep|until|lsof|ps)\b"),
    ]
    for name, pat in rules:
        if re.search(pat, c):
            return name
    return "other"


tool_n = collections.Counter()
tool_out = collections.Counter()
read_n = collections.Counter()
read_sessions = collections.defaultdict(set)
read_bytes = collections.Counter()
bash_n = collections.Counter()
bash_secs = collections.Counter()
bash_out = collections.Counter()
bash_err = collections.Counter()
err_n = collections.Counter()
err_msgs = collections.Counter()
edit_n = collections.Counter()
models = collections.Counter()
tok = collections.defaultdict(lambda: collections.Counter())
pre_edit_bytes = []
pre_edit_reads = []
sessions = 0
subagents = 0
first_reads = collections.Counter()
gate_runs = []
cost = 0.0
session_kind = collections.Counter()
golden_edit_sessions = set()
fixture_cmd_sessions = set()
codex_sessions = set()
ctx_at_first_edit = []
big_results = collections.Counter()

for f in files:
    is_sub = "/subagents/" in f
    pend = {}
    seen_edit = False
    pre_b = 0
    pre_r = 0
    started = None
    counted = False
    reads_in_order = []
    last_usage = None
    try:
        fh = open(f, errors="replace")
    except Exception:
        continue
    for line in fh:
        if '"type"' not in line:
            continue
        # cheap filters
        if '"type":"api-request' in line or '"type": "api-request' in line:
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        t = d.get("type")
        if t == "cost-state" and not is_sub:
            cost_this = d.get("totalCostUSD") or 0
            # keep the last one per file
            d_cost = cost_this
            pend["__cost"] = d_cost
            continue
        if t not in ("assistant", "user"):
            continue
        when = ts(d.get("timestamp", ""))
        if when is None:
            continue
        if when.strftime("%Y-%m-%d") < SINCE:
            continue
        if not counted:
            counted = True
            if is_sub:
                subagents += 1
            else:
                sessions += 1
        m = d.get("message") or {}
        content = m.get("content")
        if t == "assistant":
            model = m.get("model") or "?"
            u = m.get("usage") or {}
            if u:
                models[model] += 1
                tok[model]["out"] += u.get("output_tokens", 0) or 0
                tok[model]["cache_read"] += u.get("cache_read_input_tokens", 0) or 0
                tok[model]["cache_write"] += u.get("cache_creation_input_tokens", 0) or 0
                tok[model]["in"] += u.get("input_tokens", 0) or 0
                last_usage = (u.get("cache_read_input_tokens", 0) or 0) + (u.get("cache_creation_input_tokens", 0) or 0) + (u.get("input_tokens", 0) or 0)
            if isinstance(content, list):
                for b in content:
                    if not isinstance(b, dict) or b.get("type") != "tool_use":
                        continue
                    name = b.get("name", "?")
                    inp = b.get("input") or {}
                    tool_n[name] += 1
                    key = None
                    if name == "Read":
                        key = rel(inp.get("file_path", ""))
                        read_n[key] += 1
                        read_sessions[key].add(f)
                        if len(reads_in_order) < 8:
                            reads_in_order.append(key)
                    elif name == "Bash":
                        cmd = inp.get("command", "")
                        key = bash_class(cmd)
                        bash_n[key] += 1
                        if "ACQ_UPDATE_FIXTURES" in cmd:
                            fixture_cmd_sessions.add(f)
                        if re.search(r"\bcodex\b", cmd):
                            codex_sessions.add(f)
                    elif name in ("Edit", "Write", "NotebookEdit"):
                        p = rel(inp.get("file_path", ""))
                        edit_n[p] += 1
                        if "/goldens/" in p or "/fixtures/" in p:
                            golden_edit_sessions.add(f)
                        if not seen_edit and not p.startswith("/private/tmp") and not p.startswith("/tmp") and "scratchpad" not in p and ".claude/projects" not in p:
                            seen_edit = True
                            pre_edit_bytes.append(pre_b)
                            pre_edit_reads.append(pre_r)
                            if last_usage:
                                ctx_at_first_edit.append(last_usage)
                    pend[b.get("id")] = (name, key, when)
        else:
            if isinstance(content, list):
                for b in content:
                    if not isinstance(b, dict) or b.get("type") != "tool_result":
                        continue
                    tid = b.get("tool_use_id")
                    name, key, t0 = pend.pop(tid, ("?", None, None))
                    c = b.get("content")
                    if isinstance(c, list):
                        size = sum(len(x.get("text", "")) for x in c if isinstance(x, dict))
                        text = " ".join(x.get("text", "") for x in c if isinstance(x, dict))
                    else:
                        size = len(c or "")
                        text = c or ""
                    tool_out[name] += size
                    if not seen_edit:
                        pre_b += size
                        if name == "Read":
                            pre_r += 1
                    if name == "Read" and key:
                        read_bytes[key] += size
                    if name == "Bash" and key:
                        bash_out[key] += size
                        if t0:
                            secs = (when - t0).total_seconds()
                            bash_secs[key] += secs
                            if key == "gate.sh (full)":
                                gate_runs.append(secs)
                    if b.get("is_error"):
                        err_n[name] += 1
                        if name == "Bash" and key:
                            bash_err[key] += 1
                        head = re.sub(r"\s+", " ", text)[:90]
                        head = re.sub(r"[0-9a-f]{7,}", "#", head)
                        head = re.sub(r"\d+", "N", head)
                        err_msgs[(name, head)] += 1
    for k in reads_in_order[:5]:
        first_reads[k] += 1
    if "__cost" in pend:
        cost += pend["__cost"]


def table(title, rows, head):
    print("\n## " + title)
    print(" | ".join(head))
    for r in rows:
        print(" | ".join(str(x) for x in r))


print(f"since {SINCE}: {sessions} sessions, {subagents} subagent transcripts, {len(files)} files; recorded cost ${cost:,.0f}")

table("Model turns and tokens (millions)", [
    (m, n, f"{tok[m]['out']/1e6:.1f}", f"{tok[m]['cache_write']/1e6:.1f}", f"{tok[m]['cache_read']/1e6:.0f}")
    for m, n in models.most_common(8)], ["model", "turns", "out", "cache write", "cache read"])

table("Tool calls", [(n, c, f"{tool_out[n]/1e6:.1f} MB", err_n[n]) for n, c in tool_n.most_common(22)],
      ["tool", "calls", "result bytes", "errors"])

table("Bash by kind", [(k, c, f"{bash_secs[k]/3600:.1f} h", f"{bash_secs[k]/max(c,1):.0f} s", f"{bash_out[k]/1e6:.1f} MB", bash_err[k])
                       for k, c in bash_n.most_common(30)], ["kind", "calls", "wall", "mean", "output", "errors"])

table("Files read most (calls, sessions, MB of result)", [
    (k, c, len(read_sessions[k]), f"{read_bytes[k]/1e6:.2f}") for k, c in read_n.most_common(45)],
    ["file", "reads", "transcripts", "MB"])

by_bytes = sorted(read_bytes.items(), key=lambda kv: -kv[1])[:30]
table("Files by bytes read", [(k, f"{v/1e6:.2f}", read_n[k]) for k, v in by_bytes], ["file", "MB", "reads"])

table("Among a transcript's first five reads", [(k, c) for k, c in first_reads.most_common(25)], ["file", "transcripts"])

table("Files edited most", [(k, c) for k, c in edit_n.most_common(40)], ["file", "edits"])

# edits by top-level area
area = collections.Counter()
for k, c in edit_n.items():
    parts = k.split("/")
    if parts[0] == "crates" and len(parts) > 2:
        a = "/".join(parts[:3]) if parts[2] in ("src", "tests", "unit") else "/".join(parts[:2])
    else:
        a = parts[0]
    area[a] += c
table("Edits by area", area.most_common(30), ["area", "edits"])

import statistics as st
if pre_edit_bytes:
    q = sorted(pre_edit_bytes)
    table("Tool-result bytes taken in before a transcript's first edit of a project file", [
        (len(q), f"{q[len(q)//4]/1e3:.0f} KB", f"{q[len(q)//2]/1e3:.0f} KB", f"{q[3*len(q)//4]/1e3:.0f} KB", f"{q[-1]/1e3:.0f} KB")],
        ["n", "p25", "median", "p75", "max"])
    r = sorted(pre_edit_reads)
    print("Read calls before first edit: median", r[len(r)//2], "p75", r[3*len(r)//4])
if ctx_at_first_edit:
    q = sorted(ctx_at_first_edit)
    print("context tokens at first edit: p25", q[len(q)//4], "median", q[len(q)//2], "p75", q[3*len(q)//4])
if gate_runs:
    g = sorted(gate_runs)
    print(f"\nfull gate runs seen: {len(g)}, median {g[len(g)//2]:.0f} s, p90 {g[int(len(g)*.9)]:.0f} s, total {sum(g)/3600:.1f} h")
print("transcripts that ran ACQ_UPDATE_FIXTURES:", len(fixture_cmd_sessions), "; that edited a golden/fixture by hand:", len(golden_edit_sessions), "; that ran codex:", len(codex_sessions))

table("Commonest errors", [(n, c, h) for (n, h), c in err_msgs.most_common(40)], ["tool", "count", "head"])
