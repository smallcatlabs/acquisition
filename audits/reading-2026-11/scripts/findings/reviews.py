# Parse every saved review: step key, kind, reviewed commit, its author model, add date, findings.
import os, re, subprocess, json, collections, datetime, sys
R=os.environ.get('ACQ_REPO','.')
os.chdir(R)
def git(*a): return subprocess.run(['git',*a],capture_output=True,text=True).stdout
# add dates of review files
adds={}
cur=None
for line in git('log','--diff-filter=A','--format=@%H %aI','--name-only','--','roadmap/reviews').splitlines():
    if line.startswith('@'): cur=line[1:].split()
    elif line.strip().startswith('roadmap/reviews/'):
        adds.setdefault(line.strip().split('/')[-1],cur)  # log is newest first; keep first-seen? want oldest
# oldest add: recompute keeping last
adds={}
for line in git('log','--diff-filter=A','--format=@%H %aI','--name-only','--','roadmap/reviews').splitlines():
    if line.startswith('@'): cur=line[1:].split()
    elif line.strip().startswith('roadmap/reviews/'):
        adds[line.strip().split('/')[-1]]=cur
rows=[]
sev=re.compile(r'\*\*\s*\[?(P[123])\b|^\s*[-*]\s*\[?(P[123])\b|^#+\s*(P[123])\b',re.M)
for fn in sorted(os.listdir('roadmap/reviews')):
    if not fn.endswith('.md') or fn.endswith('-prompt.md'): continue
    m=re.match(r'(.+?)-([0-9a-f]{7,12})\.md$',fn)
    if not m: continue
    base,h=m.groups()
    kind='build'
    if re.search(r'-brief$',base): kind='brief'; step=base[:-6]
    elif re.search(r'-fixes$',base): kind='fix'; step=base[:-6]
    else: step=base
    txt=open('roadmap/reviews/'+fn,encoding='utf-8',errors='replace').read()
    c=collections.Counter()
    for a,b,d in sev.findall(txt): c[a or b or d]+=1
    cwb=len(re.findall(r'changes what gets built',txt,re.I))
    info=git('log','-1','--format=%H|%cI|%(trailers:key=Assisted-by,valueonly,separator=;)',h).strip()
    full,cdate,model=(info.split('|')+['','',''])[:3] if info else ('','','')
    mm='fable' if 'Fable' in model else 'opus' if 'Opus' in model else 'sonnet' if 'Sonnet' in model else 'none'
    ad=adds.get(fn,['',''])
    rows.append(dict(fn=fn,step=step,kind=kind,hash=h,full=full,cdate=cdate,model=mm,added=ad[1],addcommit=ad[0],
        p1=c['P1'],p2=c['P2'],p3=c['P3'],cwb=cwb,bytes=len(txt.encode())))
# rounds: within step+kind group (brief separate; build+fix together) ordered by reviewed commit date
groups=collections.defaultdict(list)
for r in rows:
    groups[(r['step'],'brief' if r['kind']=='brief' else 'build')].append(r)
for g in groups.values():
    g.sort(key=lambda r:(r['cdate'] or r['added'] or ''))
    for i,r in enumerate(g): r['round']=i+1
json.dump(rows,open('/tmp/reading-2026-11/findings/rows.json','w'),indent=0)
print(len(rows))
