import sys,glob,json,os,re,datetime as dt
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)),"shares.py")).read().split("for pre in sys.argv")[0])
def T(s): return dt.datetime.fromisoformat(s.replace("Z","+00:00"))
for pre in sys.argv[1:]:
    f=glob.glob(P+pre+"*.jsonl")[0]; i=0; start=None; fe=None; end=None; commits=0; codex=[]; crates_commits=0
    for l in open(f):
        e=json.loads(l); t=e.get("timestamp")
        if t: start=start or t; end=t
        if e.get("type")!="assistant": continue
        for b in e["message"]["content"]:
            if isinstance(b,dict) and b.get("type")=="tool_use":
                i+=1; inp=b.get("input") or {}; c=cls(b["name"],inp)
                cmd=inp.get("command","")
                if c=="edit" and fe is None and ("crates/" in json.dumps(inp)): fe=(i,t)
                if "git commit" in cmd: commits+=1
                if "codex exec" in cmd: codex.append(t)
    print(pre,"calls",i,"first code edit at call",fe[0] if fe else None,"after",(T(fe[1])-T(start)) if fe else None,"span",T(end)-T(start),"commit cmds",commits,"codex launches",len(codex))
