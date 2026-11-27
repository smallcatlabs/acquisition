import sys, os; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
import collections, datetime as dt
S=load(); seen=set(); W=collections.defaultdict(collections.Counter); sess=collections.defaultdict(set)
args=collections.Counter()
for s in S:
    for x in s['ev']:
        if x['k']!='tool' or x['id'] in seen or not x['t'] or x['t'][:10]<'2026-10-20': continue
        seen.add(x['id'])
        for v,a in bothy(x):
            if v in('pack','show'):
                d=dt.date.fromisoformat(x['t'][:10]); wk=d-dt.timedelta(days=d.weekday())
                W[str(wk)][v]+=1; sess[(str(wk),v)].add(s['file'])
                args[(v,re.sub(r'\S*crates/\S*','<path>',a.strip())[:40])]+=1
for wk in sorted(W): print(wk, dict(W[wk]), 'sessions pack',len(sess[(wk,'pack')]),'show',len(sess[(wk,'show')]))
print(args.most_common(15))
# total tool calls per week for scale
T=collections.Counter(); seen2=set()
for s in S:
    for x in s['ev']:
        if x['k']=='tool' and x['t'] and x['id'] not in seen2 and x['t'][:10]>='2026-10-20':
            seen2.add(x['id']); d=dt.date.fromisoformat(x['t'][:10]); T[str(d-dt.timedelta(days=d.weekday()))]+=1
print(sorted(T.items()))
