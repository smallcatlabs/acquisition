import sys, os; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
import collections
H=os.path.dirname(os.path.abspath(__file__))
B=json.load(open(H+'/builders.json')); names={b['name'] for b in B}
S=load()
# pack/show sessions since 11-10 by role
role=collections.Counter();
for s in S:
    if s['end'][:10]<'2026-11-10': continue
    n=os.path.basename(s['file'])[:8]
    k=[v for x in s['ev'] for v,a in bothy(x) if v in('pack','show') and x['t'] and x['t'][:10]>='2026-11-10']
    if k: role[('builder' if n in names else 'sub' if s['sub'] else 'other', tuple(sorted(set(k))))]+=1
print('sessions since 11-10 calling pack/show, by role:',role)
# area files read anytime by builders (whole/range)
A=collections.Counter(); AW=collections.Counter()
for s in S:
    n=os.path.basename(s['file'])[:8]
    if n not in names: continue
    got=set(); gw=set()
    for x in s['ev']:
        for d,m in gov_reads(x):
            if m in('whole','range'): got.add(d)
            if m=='whole': gw.add(d)
    for d in got: A[d]+=1
    for d in gw: AW[d]+=1
print('builders reading (whole/range) at any time:',sorted(A.items(),key=lambda kv:-kv[1]))
print('whole at any time:',sorted(AW.items(),key=lambda kv:-kv[1]))
# trigger by target
T=collections.defaultdict(collections.Counter); G=collections.Counter(); ids=collections.Counter()
for b in B:
    hasg=False
    for kinds,cl in b['trig']:
        for k,a in kinds:
            tgt=k if k!='read' else a.split(':')[0] if a.startswith(('decisions','skill')) else a
            if k=='read' and a.startswith('decisions'): tgt='decisions:'+('process' if a.endswith('process') else 'area')
            T[tgt][cl]+=1
            if k=='grepC': hasg=True; ids.update(a.split(','))
    G[hasg]+=1
for t,c in T.items(): print(t, sum(c.values()), dict(c.most_common()))
print('builders with post-edit C-id grep',G, 'top ids',ids.most_common(12))
print('show calls by builders:',[(b['name'],a) for b in B for kinds,cl in b['trig'] for k,a in kinds if k=='show'])
