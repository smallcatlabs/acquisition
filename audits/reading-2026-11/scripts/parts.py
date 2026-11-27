import re, glob, collections, sys
tot = collections.Counter(); per = {}
rows = []
for f in sorted(glob.glob('decisions/*.md')) + ['AGENTS.md']:
    area = f.split('/')[-1][:-3] if f != 'AGENTS.md' else 'cross-cutting'
    for line in open(f, encoding='utf-8'):
        m = re.match(r'- \*\*(C\d+) — (.*?)\*\* ?(.*)$', line.rstrip('\n'))
        if not m: continue
        cid, head, rest = m.groups()
        body, why, pinned = rest, '', ''
        if '*Pinned:*' in rest:
            rest, pinned = rest.split('*Pinned:*', 1)
        if '*Why:*' in rest:
            body, why = rest.split('*Why:*', 1)
        else:
            body = rest
        b = lambda s: len(s.encode())
        rows.append((area, cid, b(head), b(body), b(why), b(pinned), pinned.strip()))
import statistics
areas = collections.defaultdict(lambda: [0,0,0,0,0,0])
for a, cid, h, bo, w, p, ptxt in rows:
    r = areas[a]; r[0]+=1; r[1]+=h; r[2]+=bo; r[3]+=w; r[4]+=p
    if ptxt.lower().startswith('nothing yet'): r[5]+=1
print('area n head body why pinned nothing_yet')
T=[0]*6
for a, r in areas.items():
    print(a, *r); T=[x+y for x,y in zip(T,r)]
print('TOTAL', *T)
tb = sum(T[1:5]); print('shares head %.0f%% body %.0f%% why %.0f%% pinned %.0f%%' % tuple(100*x/tb for x in T[1:5]))
