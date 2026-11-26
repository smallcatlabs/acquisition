import os
import json,sys,collections,datetime
rows=json.load(open(sys.argv[1]))
def wk(r):
    d=r['added'] or r['cdate']
    if not d: return '?'
    return 'W%02d'%datetime.date.fromisoformat(d[:10]).isocalendar()[1]
def mat(r): return (r['p1']+r['p2'])>0 if r['kind']!='brief' else (r['cwb']>0 or r['p1']+r['p2']>0)
def tab(key,flt=lambda r:True,label=''):
    t=collections.defaultdict(lambda:[0,0,0,0,0])
    for r in rows:
        if not flt(r): continue
        k=key(r); x=t[k]; x[0]+=1; x[1]+=mat(r); x[2]+=r['p1']; x[3]+=r['p2']; x[4]+=r['cwb']
    print('==',label)
    for k in sorted(t): n,m,p1,p2,cwb=t[k]; print(k,n,m,'%d%%'%(100*m/n),'P1',p1,'P2',p2,'cwb',cwb)
nb=lambda r:r['kind']!='brief'
tab(wk,nb,'build+fix by week: n, with P1/P2, %')
tab(wk,lambda r:r['kind']=='brief','brief by week (material = cwb or P1/2)')
tab(lambda r:min(r['round'],4),nb,'build+fix by round (4=4+)')
tab(lambda r:(wk(r)>='W46', min(r['round'],3)),nb,'build+fix by (since W46, round)')
tab(lambda r:min(r['round'],3),lambda r:r['kind']=='brief','brief by round')
tab(lambda r:r['kind'],label='by kind')
tab(lambda r:r['model'],nb,'build+fix by author model')
tab(lambda r:(wk(r)>='W46',r['model']),nb,'build+fix by since-W46, model')
tab(lambda r:(r['model'],r['kind']),label='model x kind')
tab(lambda r:(r['model'],min(r['round'],2)),lambda r:nb(r) and wk(r)>='W46','since W46 build+fix model x round(2=2+)')
print('bytes by kind',{k:sum(r['bytes'] for r in rows if r['kind']==k) for k in ('brief','build','fix')})
print('ungraded build/fix',sum(1 for r in rows if nb(r) and r['p1']+r['p2']+r['p3']==0))
print('no model',[r['fn'] for r in rows if r['model']=='none'][:10])
