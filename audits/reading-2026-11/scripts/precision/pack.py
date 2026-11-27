import subprocess, sys, re
# usage: pack.py file...  prints, for the union of pack --path outputs, whole ids, headline ids, bytes
whole=set(); head=set(); nb=0
for f in sys.argv[1:]:
    out=subprocess.run(['python3','-I','tools/bothy.py','pack','--path',f],capture_output=True,text=True).stdout
    nb+=len(out.encode())
    sec=None
    for line in out.splitlines():
        if line.startswith('## '):
            sec='whole' if 'cites' in line else ('head' if 'headlines' in line else 'other')
        m=re.match(r'- \*\*(C\d+) ',line)
        if m and sec=='whole': whole.add(m.group(1))
        m2=re.match(r'- (C\d+) — ',line)
        if m2 and sec=='head': head.add(m2.group(1))
        if line.startswith('Cited from `AGENTS.md`'):
            for i in re.findall(r'C\d+',line): whole.add(i+'*')
k=lambda s:int(s.strip('C*'))
print('bytes',nb); print('whole',len(whole),' '.join(sorted(whole,key=k))); print('head',len(head-{w.strip('*') for w in whole}),' '.join(sorted(head-{w.strip('*') for w in whole},key=k)))
