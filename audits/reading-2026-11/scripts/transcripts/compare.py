import sys, os; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
import collections, statistics
H=os.path.dirname(os.path.abspath(__file__))
rows=json.load(open(H+'/rows.json')); B=json.load(open(H+'/builders.json'))
fn2={r['fn']:r for r in rows}
bystep=collections.defaultdict(list)
for r in rows:
    if r['kind']!='brief': bystep[r['step']].append(r)
S={os.path.basename(s['file'])[:8]:s for s in load()}
out=[]
for b in B:
    s=S[b['name']]
    named=collections.Counter()
    for x in s['ev']:
        if x['k']!='tool': continue
        c=json.dumps(x['input'])
        for f in re.findall(r"roadmap/reviews/([\w.-]+?-[0-9a-f]{7,12}\.md)",c):
            if f in fn2 and fn2[f]['kind']!='brief': named[fn2[f]['step']]+=1
    if not named: out.append((b['name'],b['area_whole'],None,None,None)); continue
    step=named.most_common(1)[0][0]
    g=sorted(bystep[step],key=lambda r:r['round'])
    first=g[0]
    out.append((b['name'],b['area_whole'],step,len(g),first['p1'],first['p2']))
for o in out: print(o)
for grp in (True,False):
    xs=[o for o in out if o[1]==grp and o[2]]
    print('area_whole',grp,'n',len(xs),'of',sum(1 for o in out if o[1]==grp),'rounds median',statistics.median([o[3] for o in xs]) if xs else None,'rounds',sorted(o[3] for o in xs),'first P1 total',sum(o[4] for o in xs),'sessions with first-review P1>0',sum(1 for o in xs if o[4]>0))
