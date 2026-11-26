"""Render one session transcript as an outline. Usage: outline.py <prefix> [--from N] [--full]"""
import glob, json, os, sys, re
P=os.path.expanduser("~/.claude/projects/-Users-tom-Development-GitHub-smallcatlabs-acquisition/")
pre=sys.argv[1]; full="--full" in sys.argv
f=[x for x in glob.glob(P+pre+"*.jsonl")][0]
HOME=os.path.expanduser("~")
def clean(s): return s.replace(HOME,"~").replace("\n"," ⏎ ")
i=0
for line in open(f):
    try: e=json.loads(line)
    except: continue
    ty=e.get("type"); m=e.get("message") or {}
    t=(e.get("timestamp") or "")[5:16]
    if ty not in ("user","assistant"): 
        if ty=="system" and e.get("subtype"): print(f"{t} SYS {e.get('subtype')} {clean(str(e.get('content',''))[:150])}")
        continue
    c=m.get("content"); i+=1
    if isinstance(c,str): c=[{"type":"text","text":c}]
    for b in c or []:
        if not isinstance(b,dict): continue
        k=b.get("type")
        if k=="text":
            n=3000 if full else 300
            who="USER" if ty=="user" else "A"
            if e.get("isCompactSummary"): who="COMPACT"
            print(f"{i} {t} {who}: {clean(b['text'][:n])}")
        elif k=="tool_use":
            inp=b.get("input",{})
            s=inp.get("command") or inp.get("file_path") or inp.get("prompt") or inp.get("description") or json.dumps(inp)
            print(f"{i} {t}   >{b['name']}: {clean(str(s)[:200])}")
        elif k=="tool_result":
            r=b.get("content"); 
            if isinstance(r,list): r=" ".join(x.get("text","") for x in r if isinstance(x,dict))
            r=str(r)
            err=" ERR" if b.get("is_error") else ""
            print(f"{i} {t}   <{len(r)}c{err} {clean(r[:80]) if err else ''}")
