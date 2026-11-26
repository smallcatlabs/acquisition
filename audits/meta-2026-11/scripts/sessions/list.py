import glob, json, os, sys
P=os.path.expanduser("~/.claude/projects/-Users-tom-Development-GitHub-smallcatlabs-acquisition/")
rows=[]
for f in glob.glob(P+"*.jsonl"):
    first=None; start=None; end=None; n=0; nuser=0; model=None; tools=0; sub=0
    for line in open(f):
        try: e=json.loads(line)
        except: continue
        t=e.get("timestamp")
        if t:
            start=start or t; end=t
        ty=e.get("type")
        m=e.get("message") or {}
        if ty=="assistant":
            n+=1; model=model or m.get("model")
            for b in m.get("content") or []:
                if isinstance(b,dict) and b.get("type")=="tool_use":
                    tools+=1
                    if b.get("name") in ("Agent","Task"): sub+=1
        if ty=="user":
            c=m.get("content")
            txt=None
            if isinstance(c,str): txt=c
            elif isinstance(c,list):
                for b in c:
                    if isinstance(b,dict) and b.get("type")=="text": txt=b["text"]; break
            if txt and not e.get("isMeta"):
                nuser+=1
                if first is None and not txt.startswith("<"): first=txt
    if start and "2026-11-10"<=start[:10]<="2026-11-25":
        subdir=f[:-6]
        nsub=len(glob.glob(subdir+"/**/*.jsonl",recursive=True)) if os.path.isdir(subdir) else 0
        rows.append((start[:16],end[:16] if end else "",os.path.basename(f)[:8],n,nuser,tools,sub,nsub,model,(first or "").replace("\n"," ")[:200]))
rows.sort()
for r in rows: print(" | ".join(map(str,r)))
print(len(rows))
