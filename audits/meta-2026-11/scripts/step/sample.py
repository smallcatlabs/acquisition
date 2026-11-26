import os
# Extract P1/P2 findings from build/fix reviews added since 2026-11-10; sample 30 spread across steps (seed 11).
import json,re,random,collections
R=os.environ.get('ACQ_REPO','.')+'/roadmap/reviews/'
rows=json.load(open('rows.json'))
F=[]
for r in rows:
    if r['kind']=='brief' or (r['added'] or '')[:10]<'2026-11-10': continue
    t=open(R+r['fn']).read()
    for b in re.split(r'\n(?=\s*[-*] |\n)',t):
        m=re.search(r'\b(P[12])\b',b[:40])
        if m: F.append((r['fn'],m.group(1),r['round'],r['model'],re.sub(r'\s+',' ',b.strip())))
print('total P1/P2 findings since 11-10:',len(F), collections.Counter(f[1] for f in F))
random.seed(11)
by=collections.defaultdict(list)
for f in F: by[f[0].rsplit('-',1)[0].replace('-fixes','')].append(f)
steps=list(by); random.shuffle(steps)
S=[]
i=0
while len(S)<30:
    for s in steps:
        if len(by[s])>i and len(S)<30: S.append(random.choice(by[s]) if i==0 else by[s][i])
    i+=1
for n,f in enumerate(S,1): print(f'\n#{n} {f[0]} {f[1]} round{f[2]} {f[3]}\n{f[4][:700]}')
