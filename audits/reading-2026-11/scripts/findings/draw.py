import json,re,random,collections,sys
R='./roadmap/reviews/'
rows=json.load(open(sys.argv[1]))
F=[]
for r in rows:
    if r['kind']=='brief' or (r['added'] or '')[:10]<'2026-11-10': continue
    t=open(R+r['fn']).read()
    k=0
    for b in re.split(r'\n(?=\s*[-*] |\n)',t):
        m=re.search(r'\b(P[12])\b',b[:40])
        if m:
            k+=1; F.append(dict(fn=r['fn'],sev=m.group(1),n=k,round=r['round'],hash=r['hash'],added=r['added'][:10],text=re.sub(r'\s+',' ',b.strip())))
print('pool',len(F),collections.Counter(f['sev'] for f in F),'reviews',len({f['fn'] for f in F}))
SEED=20261126
random.seed(SEED)
order=F[:]; random.shuffle(order)
S=[];seen=set()
for f in order:
    if len(S)>=45: break
    if id(f) in seen: continue
    S.append(f); seen.add(id(f))
    for g in F:
        if g['fn']==f['fn'] and g['sev']=='P1' and id(g) not in seen:
            S.append(g); seen.add(id(g))
json.dump(S,open('sample.json','w'),indent=1)
print('sample',len(S),collections.Counter(f['sev'] for f in S))
for i,f in enumerate(S,1): print(f"#{i} {f['fn']} {f['sev']}#{f['n']} r{f['round']} {f['added']}")
