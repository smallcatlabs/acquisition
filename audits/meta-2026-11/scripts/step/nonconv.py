import os
# For P1/P2 findings in round>=2 build reviews since 11-10: is the cited file:line inside a hunk the reviewed fix commit itself changed (+-15 lines)?
import json,re,subprocess
R=os.environ.get('ACQ_REPO','.')
rows=json.load(open('rows.json'))
def hunks(rng):
    out=subprocess.run(['git','-C',R,'diff','-U0',*rng],capture_output=True,text=True).stdout
    H={};cur=None
    for l in out.splitlines():
        if l.startswith('+++ b/'): cur=l[6:]
        m=re.match(r'@@ -\S+ \+(\d+)(?:,(\d+))? @@',l)
        if m and cur: s=int(m.group(1)); n=int(m.group(2) or 1); H.setdefault(cur,[]).append((s,s+max(n,1)))
    return H
tot=inn=infile=0; per={}
for r in rows:
    if r['kind']=='brief' or r['round']<2 or (r['added'] or '')[:10]<'2026-11-10': continue
    t=open(R+'/roadmap/reviews/'+r['fn']).read()
    first=t.split('\n',3)
    m=re.search(r'([0-9a-f]{7,40})\^?\.\.([0-9a-f]{7,40})',t[:600])
    rng=[m.group(1)+('^' if '^..' in t[:600] else ''),m.group(2)] if m else [r['hash']+'^',r['hash']]
    H=hunks(rng)
    for b in re.split(r'\n(?=\s*[-*] )',t):
        if not re.search(r'\bP[12]\b',b[:40]): continue
        tot+=1
        loc=re.search(r'([\w./-]+\.(?:rs|sh|py|md|js|css|toml|yml)):(\d+)',b)
        hit='noloc'
        if loc:
            f,ln=loc.group(1),int(loc.group(2))
            fs=[k for k in H if k.endswith(f) or f.endswith(k)]
            if fs:
                infile+=1
                if any(a-15<=ln<=b2+15 for k in fs for a,b2 in H[k]): inn+=1; hit='in-hunk'
                else: hit='in-file'
            else: hit='outside'
        per.setdefault(hit,[]).append(r['fn'])
print('round>=2 P1/P2 findings since 11-10:',tot)
for k,v in per.items(): print(k,len(v))
