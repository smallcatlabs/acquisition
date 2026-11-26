"""Classify each tool call of a session into a coarse activity, print shares. Usage: shares.py prefix..."""
import glob, json, os, sys, re, collections
P=os.path.expanduser("~/.claude/projects/-Users-tom-Development-GitHub-smallcatlabs-acquisition/")
RULES=r"AGENTS\.md|decisions/|roadmap/|REVIEW-CHECKLIST|GLOSSARY|TESTING-NOTES|SKILL\.md|\.claude/skills|memory/|bothy\.py (pack|list|show)|HISTORY\.md|README\.md|briefs/|process\.md|LEDGER"
def cls(name,inp):
    s=json.dumps(inp)
    cmd=inp.get("command","") if isinstance(inp,dict) else ""
    if name in("Edit","Write","NotebookEdit"):
        fp=inp.get("file_path","")
        if re.search(r"roadmap/|HISTORY|LEDGER|decisions/|memory/|reviews/|audits/",fp): return "record"
        return "edit"
    if name in("Agent","Task","SendMessage") or name.startswith("mcp__ccd_session__start") or "send_message" in name: return "delegate"
    if name in("Monitor","TaskOutput","ScheduleWakeup") or "list_events" in name or "get_session" in name or "read_terminal" in name or "list_sessions" in name: return "wait"
    if name=="Read":
        fp=inp.get("file_path","")
        if re.search(RULES,fp): return "orient"
        if "tasks/" in fp or "/tmp/" in fp or "scratchpad" in fp: return "wait"
        return "readcode"
    if name in("Grep","Glob"): return "readcode"
    if name.startswith("mcp__Claude_Browser") : return "use"
    if name=="Skill": return "orient"
    if name in("mcp__ccd_turn__update_plan","mcp__ccd_turn__wrap_up","mcp__ccd_session__mark_chapter"): return "ceremony"
    if name!="Bash": return "other:"+name[:30]
    c=cmd
    if re.search(r"\bsleep\b|until |tail -f|while .*do|TaskOutput|wait\b",c): return "wait"
    if re.search(r"codex|cold-review|reviews/|claude -p|--print",c): return "review"
    if re.search(r"git (commit|push|tag)|gh (issue|pr) (create|comment|edit|close)|gh issue|git notes",c): return "record"
    if re.search(r"\bacq(d|-mcp)?\b(?!uisition)|ACQ_PROVIDER|curl .*127\.0\.0\.1|curl .*localhost|target/(debug|release)/acq",c) and not re.search(r"cargo",c): return "use"
    if re.search(r"cargo|gate\.sh|nextest|docs-check|arch-check|trade-check|bothy\.py check",c): return "buildtest"
    if re.search(r"git (diff|status|log|show)",c) and not re.search(r"crates/",c): return "git-read"
    if re.search(RULES,c) and not re.search(r"crates/",c): return "orient"
    if re.search(r"apply_patch|python3? - <<|perl -pi|sed -i|cat >>? ?crates|cat >>? ?[a-z]+/|tee ",c) and "scratchpad" not in c.split("<<")[0]: return "edit"
    return "readcode"
for pre in sys.argv[1:]:
    f=glob.glob(P+pre+"*.jsonl")[0]
    C=collections.Counter(); users=[]; tot=0; atext=0; compacts=0
    for line in open(f):
        try:e=json.loads(line)
        except: continue
        m=e.get("message") or {}; c=m.get("content")
        if e.get("isCompactSummary"): compacts+=1
        if e.get("type")=="assistant":
            for b in c or []:
                if isinstance(b,dict) and b.get("type")=="tool_use": C[cls(b["name"],b.get("input") or {})]+=1; tot+=1
                elif isinstance(b,dict) and b.get("type")=="text": atext+=1
        if e.get("type")=="user" and not e.get("isMeta") and not e.get("isCompactSummary"):
            if isinstance(c,str): users.append(e["timestamp"][5:16])
            elif isinstance(c,list) and any(isinstance(b,dict) and b.get("type")=="text" and not b["text"].startswith("<") for b in c): users.append(e["timestamp"][5:16])
    print(pre, "tools",tot,"compactions",compacts,"usermsgs",len(users))
    print("  ", ", ".join(f"{k} {100*v//max(tot,1)}%" for k,v in C.most_common()))
