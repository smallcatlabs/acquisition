import sys,glob,json,os,re
sys.path.insert(0,os.path.dirname(__file__))
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)),"shares.py")).read().split("for pre in sys.argv")[0])
want=sys.argv[2] if len(sys.argv)>2 else "use"
for pre in sys.argv[1].split(","):
    f=glob.glob(P+pre+"*.jsonl")[0]
    for l in open(f):
        e=json.loads(l)
        if e.get("type")!="assistant": continue
        for b in e["message"]["content"]:
            if isinstance(b,dict) and b.get("type")=="tool_use" and cls(b["name"],b.get("input") or {})==want:
                i=b.get("input") or {}
                print(pre,e["timestamp"][5:16],b["name"][:25],(i.get("command") or i.get("url") or json.dumps(i))[:220].replace("\n"," ").replace(os.path.expanduser("~"),"~"))
