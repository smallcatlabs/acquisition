import glob,json,os,re,sys,collections
P=os.path.expanduser("~/.claude/projects/-Users-tom-Development-GitHub-smallcatlabs-acquisition/")
S="9a3787d4 a11c2f5a a4d7711c 06bd26e1 0d117731 8967aea7 d99afed5 a85ecac9 06672fb0 94888d72 42c6057d a34d1a05".split()
pats={k:re.compile(v,re.I) for k,v in {
 "trim_to_cap":r"trim|over the (bound|cap|limit)|over by \d|under the [\d,]+[- ]byte|bytes, just under|byte cap|6000-byte|8,000",
 "codex_run":r"^codex exec|; codex exec|&& codex exec|\(codex exec",
 "break_to_see_fail":r"breaking (it|the|only)|fails without|break(ing)? the fix",
 "golden_read":r"goldens? read|reading (each|the) golden|golden diff",
 "docs_check_fail":r"docs[- ]check (fail|refus)|failed the docs check|docs gate (fail|refus)|bothy.*(fail|refus)|linter",
 "screenshot":r"screenshot",
 "rule_cite":r"\bC1\d\d\b",
}.items()}
for s in S:
    f=glob.glob(P+s+"*.jsonl")[0]; C=collections.Counter()
    for l in open(f):
        e=json.loads(l)
        if e.get("type")!="assistant": continue
        for b in e["message"]["content"]:
            if not isinstance(b,dict): continue
            t=b.get("text") if b.get("type")=="text" else (json.dumps(b.get("input")) if b.get("type")=="tool_use" else "")
            if not t: continue
            for k,p in pats.items():
                if k=="codex_run":
                    if b.get("type")=="tool_use" and "codex exec" in t: C[k]+=1
                elif k=="rule_cite":
                    if b.get("type")=="text": C[k]+=len(p.findall(t))
                elif p.search(t): C[k]+=1
    print(s, dict(C))
