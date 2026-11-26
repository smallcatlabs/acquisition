import os
# Per step since 2026-11-10: rounds, P1/P2 by round, last round's yield; share of P1/P2 in round 1.
import json,collections
rows=[r for r in json.load(open('rows.json')) if (r['added'] or '')[:10]>='2026-11-10']
st=collections.defaultdict(list)
for r in rows: st[r['step']].append(r)
n=0;r1=0;later=0;lastfind=0;multi=0;briefsteps=0;bsteps_cwb=0
rounds=collections.Counter(); brounds=collections.Counter()
for k,v in st.items():
    b=[r for r in v if r['kind']!='brief']; br=[r for r in v if r['kind']=='brief']
    if br: briefsteps+=1; brounds[len(br)]+=1
    if not b: continue
    n+=1; rounds[len(b)]+=1
    b.sort(key=lambda r:r['round'])
    r1+=b[0]['p1']+b[0]['p2']; later+=sum(r['p1']+r['p2'] for r in b[1:])
    if len(b)>1:
        multi+=1
        if b[-1]['p1']+b[-1]['p2']>0: lastfind+=1
print('steps with build reviews',n,'rounds dist',sorted(rounds.items()))
print('P1/P2 in round1',r1,'later',later)
print('multi-round steps',multi,'whose last round still found P1/P2',lastfind)
print('steps with brief reviews',briefsteps,'brief rounds dist',sorted(brounds.items()))
print('brief reviews since 11-10',sum(1 for r in rows if r['kind']=='brief'),'bytes',sum(r['bytes'] for r in rows if r['kind']=='brief'),'build+fix bytes',sum(r['bytes'] for r in rows if r['kind']!='brief'))
print('round1 reviews with zero P1/P2', sum(1 for k,v in st.items() for r in v if r['kind']!='brief' and r['round']==1 and r['p1']+r['p2']==0))
