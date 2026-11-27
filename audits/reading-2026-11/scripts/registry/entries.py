import re, subprocess, glob, collections, sys, os
R='.'
os.chdir(R)
files = ['AGENTS.md']+sorted(glob.glob('decisions/*.md'))
E={}
for f in files:
    area = 'cross-cutting' if f=='AGENTS.md' else os.path.basename(f)[:-3]
    for line in open(f):
        m = re.match(r'- \*\*(C\d+) — ', line)
        if m: E[m.group(1)] = (area, line.rstrip('\n'))
def parts(line):
    m = re.match(r'- \*\*(C\d+) — (.*?)\*\*(.*?)\*Why:\*(.*?)\*Pinned:\*(.*)$', line)
    return m.groups() if m else None
def gg(args):
    r = subprocess.run(['git','grep']+args, capture_output=True, text=True)
    return r.stdout
def cites(cid):
    out = gg(['-c','-w',cid])
    t=collections.Counter()
    for l in out.splitlines():
        p,c = l.rsplit(':',1); c=int(c)
        if p.startswith('crates/'):
            if '/tests/' in p or p.endswith('tests.rs') or '/unit/' in p or '/goldens/' in p: t['tests']+=c
            else: t['src']+=c
        elif p=='AGENTS.md' or p.startswith('decisions/') or p in ('REVIEW-CHECKLIST.md','TESTING-NOTES.md') or p.startswith('.claude/skills/'): t['docs']+=c
        elif p.startswith('roadmap/reviews/'): t['reviews']+=c
        else: t['other']+=c
    return t
if __name__=='__main__':
    for cid in sys.argv[1:]:
        area,line = E[cid]
        cid_,head,body,why,pinned = parts(line)
        sents = [s for s in re.split(r'(?<=[.;])\s+(?=[A-Z`(])', body.strip()) if s.strip()]
        ticks = re.findall(r'`([^`]+)`', pinned)
        ex=[]
        for t in ticks:
            name = t.split('/')[-1] if '/' in t else t
            hit = bool(gg(['-l','-F',name, '--','crates','tools','.github','.config','.cargo'])) or os.path.exists(t) or bool(glob.glob('crates/'+t))
            ex.append(f"{t}:{'Y' if hit else 'N'}")
        print(cid, area, 'bytes',len(line.encode()), 'sents',len(sents), 'pinned_ticks',len(ticks), ex, dict(cites(cid)))
