import os
# Since 11-10: build round-1 yield and total build rounds, for steps whose brief was reviewed vs not.
import json,collections
rows=[r for r in json.load(open('rows.json')) if (r['added'] or '')[:10]>='2026-11-10']
st=collections.defaultdict(list)
for r in rows: st[r['step']].append(r)
agg={True:[0,0,0,0,0],False:[0,0,0,0,0]}
for k,v in st.items():
    b=sorted([r for r in v if r['kind']!='brief'],key=lambda r:r['round'])
    if not b: continue
    hb=any(r['kind']=='brief' for r in v)
    a=agg[hb]; a[0]+=1; a[1]+=(b[0]['p1']+b[0]['p2']>0); a[2]+=b[0]['p1']; a[3]+=len(b); a[4]+=sum(r['p1']+r['p2'] for r in b)
for hb,a in agg.items(): print('brief reviewed' if hb else 'no brief review', 'steps',a[0],'r1 with P1/P2',a[1],'%d%%'%(100*a[1]/a[0]),'r1 P1s',a[2],'build rounds/step %.2f'%(a[3]/a[0]),'P1+P2/step %.2f'%(a[4]/a[0]))
