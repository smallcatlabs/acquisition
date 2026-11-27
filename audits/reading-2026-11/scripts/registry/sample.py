import subprocess, random, collections, math
out = subprocess.run(['python3','-I','tools/bothy.py','list'],capture_output=True,text=True,cwd='.').stdout
by = collections.defaultdict(list)
for l in out.splitlines():
    p = l.split()
    if len(p)>=2 and p[0].startswith('C'): by[p[1]].append(p[0])
areas = ['daemon','network','store','frontends','search','plans','pricing','trade']
N = sum(len(by[a]) for a in areas)
quota = {a: 26*len(by[a])/N for a in areas}
k = {a: math.floor(q) for a,q in quota.items()}
rem = 26-sum(k.values())
for a in sorted(areas, key=lambda a: -(quota[a]-k[a]))[:rem]: k[a]+=1
random.seed(20261126)
s = {}
for a in areas:
    ids = sorted(by[a], key=lambda x:int(x[1:]))
    s[a] = sorted(random.sample(ids, k[a]), key=lambda x:int(x[1:]))
ids = sorted(by['cross-cutting'], key=lambda x:int(x[1:]))
s['cross-cutting'] = sorted(random.sample(ids,4), key=lambda x:int(x[1:]))
print(k)
for a,v in s.items(): print(a, ' '.join(v))
