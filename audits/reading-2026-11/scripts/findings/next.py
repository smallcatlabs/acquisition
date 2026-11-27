import json,re,subprocess,collections
os_R='.'
def git(*a): return subprocess.run(['git','-C',os_R,*a],capture_output=True,text=True).stdout
rows=json.load(open('rows.json')); S=json.load(open('sample.json'))
by={r['fn']:r for r in rows}
for fn in dict.fromkeys(f['fn'] for f in S):
    r=by[fn]
    grp=sorted([x for x in rows if x['step']==r['step'] and x['kind']!='brief'],key=lambda x:x['round'])
    nxt=[x for x in grp if x['round']==r['round']+1]
    t=open(os_R+'/roadmap/reviews/'+fn).read()
    cids=sorted(set(re.findall(r'\bC\d{1,3}\b',t)))
    # commits after review add, grep the review file name
    add=r['addcommit']
    ans=git('log','--format=%h %ad %s','--date=short',f'{add}..HEAD','--grep',fn.replace('.md','')).strip().splitlines()[:3]
    print(f"{fn} add={add[:8]} step={r['step']} r{r['round']} cids={cids} next={[x['fn'] for x in nxt]}")
    for a in ans: print('   grep:',a)
