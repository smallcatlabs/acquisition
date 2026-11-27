import sys, re, collections, statistics, subprocess
sys.path.insert(0, sys.argv[1])
from entries import E, parts
A=collections.defaultdict(list); ny=collections.Counter(); citing=0; cited=collections.Counter()
for cid,(area,line) in E.items():
    A[area].append(len(line.encode()))
    p=parts(line)
    if p and 'nothing yet' in p[4]: ny[area]+=1
    others=set(re.findall(r'\bC\d+\b', line))-{cid}
    if others: citing+=1
    for o in others: cited[o]+=1
print('entries',len(E))
for a in sorted(A):
    v=sorted(A[a]); print(f"{a:14} n={len(v):3} nothing_yet={ny[a]:2} bytes min={v[0]} median={int(statistics.median(v))} max={v[-1]} total={sum(v)}")
print('citing another entry:',citing)
print('top cited:',cited.most_common(10))
crates=subprocess.run(['git','grep','-h','-o','-w','-E','C[0-9]+','--','crates'],capture_output=True,text=True).stdout.split()
cs=set(crates)
missing=[c for c in E if c not in cs]
by=collections.Counter(E[c][0] for c in missing)
print('ids in no file under crates/:',len(missing),dict(by))
print(' '.join(sorted(missing,key=lambda x:int(x[1:]))))
