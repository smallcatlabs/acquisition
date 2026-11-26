import glob,json,os,re,collections
P=os.path.expanduser("~/.claude/projects/-Users-tom-Development-GitHub-smallcatlabs-acquisition/")
items={"process.md":r"decisions/process\.md","road":r"roadmap/README\.md","cold-review SKILL":r"cold-review/SKILL\.md","which codex":r"which codex","web/mod.rs":r"web/mod\.rs","TEMPLATE":r"briefs/TEMPLATE\.md","session-close SKILL":r"session-close/SKILL\.md"}
sess=collections.Counter(); tot=collections.Counter(); n=0; firstbytes=[]
for f in glob.glob(P+"*.jsonl"):
    lines=open(f).readlines()
    st=next((json.loads(l).get("timestamp") for l in lines if '"timestamp"' in l and json.loads(l).get("timestamp")),"") or ""
    if not ("2026-11-10"<=st[:10]<="2026-11-25"): continue
    n+=1; seen=set()
    for l in lines:
        e=json.loads(l)
        if e.get("type")!="assistant": continue
        for b in e["message"]["content"]:
            if isinstance(b,dict) and b.get("type")=="tool_use":
                t=json.dumps(b.get("input"))
                for k,p in items.items():
                    if re.search(p,t): tot[k]+=1; seen.add(k)
    for k in seen: sess[k]+=1
print("sessions",n)
for k in items: print(k, "sessions",sess[k],"calls",tot[k])
