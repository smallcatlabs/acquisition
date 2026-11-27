import re,sys
sec=None;out={}
for i,l in enumerate(open(sys.argv[1]),1):
    if l.startswith('## '): sec=l[3:].strip(); out[sec]=[]; continue
    if sec and l.startswith('| **'):
        cells=[c.strip() for c in l.strip().strip('|').split('|')]
        held=cells[-1]
        hl=held.lower()
        rev = bool(re.search(r'\breview\b|\breading\b|\bread\b',hl))
        test = bool(re.search(r'`[a-z0-9_]+`|tests?/|\.rs|check|breaker|golden|lint|\.sh|type|clippy',hl))
        kind = 'test/check' if test and not (rev and not test) else 'review/reading'
        if rev and test: kind='mixed'
        if rev and not test: kind='review/reading'
        if not rev and not test: kind='other'
        out[sec].append((i,kind,held[:110]))
for s,rows in out.items():
    from collections import Counter
    c=Counter(k for _,k,_ in rows)
    print(f"{s}: {len(rows)} {dict(c)}")
    for i,k,h in rows:
        if k!='test/check': print('   ',i,k,'|',h)
