# Trace chosen steps: commits by kind, lead time, product lines, brief bytes, reviews.
import subprocess,os,re,json,datetime
R=os.environ.get('ACQ_REPO','.')
def git(*a): return subprocess.run(['git','-C',R,*a],capture_output=True,text=True).stdout
S={
 'web-312 (small)':'51d99a3d',
 'search-307 (small)':'697ed4f4 ab289d00',
 'web-308 (small)':'203bc655 4dff6b96',
 'daemon-320 (ord)':'9802fc2b 5218a364',
 'form-landing (ord)':'70397d7b 195bccef 800c594c a5b7720c 38251a92 86a63c9f 84357dbf c4550601 bcc08a89 6c515b9d',
 'form-values (ord)':'5bb3633d 893c9f3b eb0b8e88 e2e15bd6 57527408 2576d17f 2060bbae 911ed6eb 952fcc71 e9334dcc b1fe747a',
 'network-288 (ord)':'bb5ee22a 602a801b d25df6e9 b6d80758 e9bcae07 20ef1712 384d6bc4 fcc6d514 39947ef2',
 'step6-places (design)':'92f0be8e a472dbd9 f2d90e7e d7837ef7 104be7e5 1ee23513 db8e4f7e e9c89426 126c2e35 bd69b32c f73b8a12',
 'step6-grouped (design)':'6c476e51 b67138de e9cc2802 793526c4 e46b765a ced616c6 4aaf3a50 e18452c2 f36033f5 8cd6738c fc0f2f63 e3e50cdb e8e8374e 43fe5a98 2abaaab9 043726af',
 'form-editor (design)':'f05dd546 f2aa7adb 817be04d 9c790f5b fb98c070 f5352957 3d8d12ea 790199a9 310862a4 1eb2054c a6acf78c 49ff31dd 44a4063c',
}
def kind(h):
    msg=git('log','-1','--format=%B',h)
    files=[l.split('\t') for l in git('show','--numstat','--format=',h).splitlines() if l.strip()]
    paths=[f[2] for f in files]
    src=sum(int(a)+int(d) for a,d,p in files if a!='-' and re.match(r'crates/[^/]+/src/',p) and '/tests/' not in p and not p.endswith('tests.rs'))
    tst=sum(int(a)+int(d) for a,d,p in files if a!='-' and re.match(r'crates/[^/]+/(tests/|src/.*tests)',p))
    if re.search(r'^Step: ',msg,re.M) or re.search(r': closed|^close',msg.splitlines()[0]): k='close'
    elif paths and all(p.startswith('roadmap/reviews/') for p in paths): k='review'
    elif any(p.startswith('roadmap/briefs/') for p in paths) and src==0: k='brief'
    elif src>0: k='product'
    elif tst>0: k='test'
    elif paths and all(p.startswith(('roadmap/','decisions/','AGENTS','HISTORY')) for p in paths): k='road/rules'
    else: k='other'
    d=git('log','-1','--format=%aI',h).strip()
    return k,src,tst,d,paths
out={}
for name,hs in S.items():
    hs=hs.split()
    rows=[]
    for h in hs:
        anc=subprocess.run(['git','-C',R,'merge-base','--is-ancestor',h,'main']).returncode==0
        if not anc: continue
        rows.append((h,)+kind(h))
    ds=sorted(r[4] for r in rows)
    lead=(datetime.datetime.fromisoformat(ds[-1])-datetime.datetime.fromisoformat(ds[0])).total_seconds()/3600
    c={}
    for r in rows: c[r[1]]=c.get(r[1],0)+1
    src=sum(r[2] for r in rows); tst=sum(r[3] for r in rows)
    bb=[p for r in rows for p in r[5] if p.startswith('roadmap/briefs/') and not p.endswith('README.md')]
    bsz=0
    for p in set(bb):
        try: bsz=max(bsz,os.path.getsize(os.path.join(R,p)))
        except: pass
    print(f"{name}: n={len(rows)} lead={lead:.1f}h kinds={c} src_lines={src} test_lines={tst} brief={sorted(set(bb))} bytes={bsz}")
