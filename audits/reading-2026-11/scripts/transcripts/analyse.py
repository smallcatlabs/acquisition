import sys, os; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
import collections, statistics, datetime as dt
S = load()
AREA = {"acquisition-protocol": ["daemon"], "acquisition-daemon": ["daemon", "network"], "acquisition-client": ["daemon"],
        "acquisition-transport": ["network"], "acquisition-store": ["store", "daemon"], "acquisition-plan": ["plans", "pricing"],
        "acquisition-search": ["search", "trade"], "acquisition-verbs": ["frontends"], "acquisition-cli": ["frontends"], "acquisition-mcp": ["frontends"]}
PATHRX = re.compile(r"(?:[\w.-]+/)+[\w.-]+\.(?:md|rs|py|sh|toml|json|tsv|yml|html)|\b(?:AGENTS|REVIEW-CHECKLIST|TESTING-NOTES|GLOSSARY|CLI-REFERENCE|MCP-REFERENCE|HISTORY|README)\.md")
CID = re.compile(r"(?<![-\w])C\d{1,3}\b")

def crate_of(x):
    s = json.dumps(x["input"])
    return set(re.findall(r"crates/(acquisition-[a-z]+)", s))

# builders
cands = collections.defaultdict(list)
for s in S:
    if s["end"][:10] < "2026-11-10": continue
    tools = [x for x in s["ev"] if x["k"] == "tool"]
    fe = next((i for i, x in enumerate(tools) if broad_edit(x)), None)
    if fe is None: continue
    cands[tools[fe]["id"]].append(s)
builders, excluded, subs = [], [], []
for k, g in cands.items():
    s = max(g, key=lambda s: len(s["ev"]))
    tools = [x for x in s["ev"] if x["k"] == "tool"]
    launches = sum(1 for x in tools if x["name"] in ("mcp__ccd_session__start_session", "Agent", "Task"))
    if s["sub"]: subs.append(s); continue
    if s["msgs"] >= 6 or launches >= 4: excluded.append((os.path.basename(s["file"])[:8], s["msgs"], launches)); continue
    builders.append(s)
print("builders", len(builders), "excluded", excluded, "subagents", len(subs))

def analyse(s):
    ev = s["ev"]; tools = [x for x in ev if x["k"] == "tool"]
    fe = next(i for i, x in enumerate(tools) if broad_edit(x))
    edits = [x for x in tools if broad_edit(x)]
    crates = set().union(*[crate_of(x) for x in edits])
    areas = set(a for c in crates for a in AREA.get(c, []))
    pre = tools[:fe]; post = tools[fe:]
    r = dict(name=os.path.basename(s["file"])[:8], start=s["start"], fe=fe, ntools=len(tools), crates=crates, areas=areas,
             ctx=tools[fe].get("ctx"), cum=tools[fe].get("cum"))
    reads = collections.defaultdict(lambda: [0, set()])
    for x in pre:
        gr = gov_reads(x); rd = [g for g in gr if g[1] in ("whole", "range")]
        for d, mode in gr:
            if mode in ("whole", "range"): reads[d][0] += x.get("rb", 0) // max(1, len(rd))
            reads[d][1].add(mode)
    r["pre"] = {d: (b, m) for d, (b, m) in reads.items()}
    r["govbytes"] = sum(b for b, m in r["pre"].values())
    r["prev_other"] = []
    r["pack"] = [a for x in pre for v, a in bothy(x) if v == "pack"]
    r["show"] = [a for x in pre for v, a in bothy(x) if v == "show"]
    r["skills"] = [x["input"].get("skill") for x in pre if x["name"] == "Skill"]
    r["skills_all"] = [x["input"].get("skill") for x in tools if x["name"] == "Skill"]
    fp = s["first"]
    paths = set(PATHRX.findall(fp))
    r["fp_paths"] = len(paths)
    r["fp_gov"] = sorted(set(d for p in paths for d, rx in GOV if rx.search(p)))
    r["fp_len"] = len(fp)
    r["fp_readword"] = bool(re.search(r"\bread\b", fp, re.I)) and len(paths) > 0
    # area file read whole pre-edit
    r["area_whole"] = any(("decisions:" + a) in r["pre"] and "whole" in r["pre"]["decisions:" + a][1] for a in areas)
    r["area_any"] = any(("decisions:" + a) in r["pre"] for a in areas)
    r["any_decisions_whole"] = any(d.startswith("decisions:") and "whole" in m for d, (b, m) in r["pre"].items())
    # post-edit
    later = collections.Counter(); trig = []
    seq = [e for e in ev]
    idx_fe = seq.index(tools[fe])
    for j in range(idx_fe, len(seq)):
        x = seq[j]
        if x["k"] != "tool": continue
        kinds = []
        for d, mode in gov_reads(x):
            if d not in r["pre"]: later[d] += 1
            kinds.append(("read", d))
        for v, a in bothy(x):
            if v in ("show", "pack"): kinds.append((v, a.strip()))
        if x["name"] == "Bash" and re.search(r"grep", x["input"].get("command", "")) and CID.search(x["input"].get("command", "")):
            kinds.append(("grepC", ",".join(sorted(set(CID.findall(x["input"]["command"]))))))
        if x["name"] == "Grep" and CID.search(x["input"].get("pattern", "")):
            kinds.append(("grepC", x["input"]["pattern"][:20]))
        if not kinds: continue
        # trigger: nearest preceding non-same-kind event
        prev = None
        for k in range(j - 1, max(idx_fe - 1, j - 4), -1):
            p = seq[k]
            if p["k"] == "msg": prev = p; break
            if p["k"] == "tool" and not gov_reads(p) and not [1 for v, a in bothy(p) if v == "show"]: prev = p; break
        cl = classify(prev, kinds)
        trig.append((kinds, cl))
        if cl == "other" and prev is not None: r["prev_other"].append((prev["name"], (prev["input"].get("command") or prev["input"].get("file_path") or prev["input"].get("pattern") or "")[:90]))
    r["later"] = later; r["trig"] = trig
    return r

def classify(p, kinds):
    if p is None: return "chain"
    if p["k"] == "msg": return "message"
    c = p["input"].get("command", "") if p["name"] == "Bash" else ""
    res = (p.get("rhead", "") or "") + (p.get("rtail", "") or "")
    ids = set(a for k, a in kinds if k in ("show", "grepC"))
    if re.search(r"codex|roadmap/reviews/", c) or "roadmap/reviews/" in json.dumps(p["input"]): return "review"
    if re.search(r"gate\.sh|docs-check|arch-check|trade-check|bothy\.py check|clippy|cargo doc", c) and re.search(r"FAIL|error|✗|exit [1-9]|failed", res): return "gate-fail"
    if re.search(r"nextest|cargo test", c) and re.search(r"FAIL|panicked|failed", res): return "test-fail"
    if re.search(r"git (commit|push)", c) and re.search(r"refus|error|fail|hook", res, re.I): return "hook-refusal"
    if re.search(r"\[P[0-3]\]|\bP[0-3]\b.*finding|[Ff]indings?:|severity|^#+ .*[Rr]eview", res[:3000], re.M) or re.search(r"review", c[:300], re.I): return "review"
    if CID.search(res): return "id-seen-in-result"
    if p["name"] in ("Edit", "Write") or broad_edit(p): return "after-edit"
    if p["name"] == "Skill": return "skill"
    if p["name"] in ("Monitor",) or re.search(r"\bsleep\b|\btail\b", c[:200]): return "wait/monitor"
    if p["name"].startswith("mcp__ccd"): return "app-card"
    if re.search(r"git (grep|log|show|diff)|\bgrep\b|\brg\b", c) or p["name"] in ("Grep", "Glob"): return "code-search"
    if p["name"] == "Read" or re.search(r"\b(sed -n|cat|head)\b", c): return "code-read"
    return "other"

R = [analyse(s) for s in builders]
json.dump([{k: (sorted(v) if isinstance(v, set) else v) for k, v in r.items() if k not in ("later",)} | {"later": dict(r["later"])} for r in R],
          open(os.path.join(HERE, "builders.json"), "w"), default=lambda o: sorted(o) if isinstance(o, set) else str(o))
print("written")
