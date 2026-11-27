import json,collections,statistics,os
R=json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)),'builders.json')))
n=len(R); print("n",n)
for r in R:
    print(r['name'],r['start'][:10],'fe',r['fe'],'areas',r['areas'],'gov',r['govbytes'],'pack',len(r['pack']),'show',len(r['show']),'skills',r['skills'],'ctx',r['ctx'],'cum',r['cum'],'fp',r['fp_paths'],r['fp_readword'],'areaW',r['area_whole'],'areaAny',r['area_any'])
    print('    pre:',{d:(b,m) for d,(b,m) in r['pre'].items()})
C=collections.Counter(); W=collections.Counter()
for r in R:
    for d,(b,m) in r['pre'].items():
        if not (set(m)&{'whole','range'}): continue
        C[d]+=1
        if 'whole' in m: W[d]+=1
print("\nshare reading each doc before first edit (any mode / whole):")
for d,v in C.most_common(): print(f"  {d:30s} {v:3d} {100*v//n:3d}%  whole {W[d]}")
g=[r['govbytes'] for r in R]; print("govbytes median",statistics.median(g),"min",min(g),"max",max(g),"zero",sum(1 for x in g if x==0))
ctx=[r['ctx'] for r in R if r['ctx']]; print("ctx at first edit median",statistics.median(ctx),min(ctx),max(ctx))
cum=[r['cum'] for r in R if r['cum']]; print("cum input median",statistics.median(cum),min(cum),max(cum))
fe=[r['fe'] for r in R]; print("tool calls before first edit median",statistics.median(fe),min(fe),max(fe))
print("pack pre",sum(1 for r in R if r['pack']),"show pre",sum(1 for r in R if r['show']))
print("pack args",[a for r in R for a in r['pack']])
print("skills pre",collections.Counter(s for r in R for s in r['skills'])," all",collections.Counter(s for r in R for s in r['skills_all']))
fpy=[r for r in R if r['fp_paths']>0]; print("first prompt names paths",len(fpy),"of",n,"median paths",statistics.median([r['fp_paths'] for r in R]),"median among yes",statistics.median([r['fp_paths'] for r in fpy]) if fpy else None, "read+path",sum(r['fp_readword'] for r in R))
print("first prompt names gov docs",collections.Counter(d for r in R for d in r['fp_gov']))
both=sum(1 for r in R if r['area_whole'] and r['pack']); one=sum(1 for r in R if bool(r['area_whole'])!=bool(r['pack'])); nei=sum(1 for r in R if not r['area_whole'] and not r['pack'])
print("area whole AND pack",both,"one",one,"neither",nei, " area-whole only",sum(1 for r in R if r['area_whole'] and not r['pack']),"pack only",sum(1 for r in R if r['pack'] and not r['area_whole']))
print("area any mode",sum(r['area_any'] for r in R))
L=collections.Counter()
for r in R:
    for d in r['later']: L[d]+=1
print("\nread later (not before), sessions:",L.most_common())
T=collections.Counter(); K=collections.Counter()
for r in R:
    for kinds,cl in r['trig']:
        for k,a in kinds:
            K[k if k!='read' else 'read:'+a.split(':')[0]]+=1
            T[(k if k!='read' else 'read', cl)]+=1
print("post-edit kinds",K.most_common())
print("triggers",sorted(T.items()))

import random
po=[x for r in R for x in r['prev_other']]
print(len(po)); random.seed(1)
for x in random.sample(po,25): print('  ',x)
