"""Fit per-token prices per model from the sessions' own cost-state records (least squares, no intercept)."""
import glob, json, os, collections
P = os.path.expanduser("~/.claude/projects/-Users-tom-Development-GitHub-smallcatlabs-acquisition")
rows = collections.defaultdict(list)
for f in glob.glob(P + "/**/*.jsonl", recursive=True):
    last = None
    for l in open(f, errors="replace"):
        if '"cost-state"' not in l: continue
        try: d = json.loads(l)
        except: continue
        if d.get("type") == "cost-state": last = d
    if not last: continue
    for m, u in (last.get("modelUsage") or {}).items():
        x = [u.get("inputTokens",0), u.get("outputTokens",0), u.get("cacheReadInputTokens",0), u.get("cacheCreationInputTokens",0)]
        rows[m].append((x, u.get("costUSD",0)))
def solve(A, b):
    n=len(A); M=[A[i][:]+[b[i]] for i in range(n)]
    for c in range(n):
        p=max(range(c,n),key=lambda r:abs(M[r][c])); M[c],M[p]=M[p],M[c]
        if abs(M[c][c])<1e-30: return None
        for r in range(n):
            if r!=c:
                k=M[r][c]/M[c][c]; M[r]=[a-k*bb for a,bb in zip(M[r],M[c])]
    return [M[i][n]/M[i][i] for i in range(n)]
for m, rs in rows.items():
    tot = sum(c for _,c in rs)
    A=[[sum(x[i]*x[j] for x,_ in rs) for j in range(4)] for i in range(4)]
    b=[sum(x[i]*c for x,c in rs) for i in range(4)]
    s = solve(A,b)
    toks=[sum(x[i] for x,_ in rs) for i in range(4)]
    print(m, len(rs), f"${tot:,.0f}", "tokens(in,out,cr,cc)M", [round(t/1e6,1) for t in toks], "per-M price fit (in,out,cr,cc)", [round(p*1e6,3) for p in s] if s else None)
