import os
# Brief reviews since 11-10: each "changes what gets built" item's first 230 chars, for judgment.
import json,re
R=os.environ.get('ACQ_REPO','.')+'/roadmap/reviews/'
rows=sorted([r for r in json.load(open('rows.json')) if r['kind']=='brief' and (r['added'] or '')[:10]>='2026-11-10'],key=lambda r:r['added'])
for r in rows:
    t=open(R+r['fn']).read()
    items=[re.sub(r'\s+',' ',b)[:230] for b in re.split(r'\n(?=\s*[-*] )',t) if re.search('changes what gets built',b,re.I)]
    print(f"\n## {r['fn']} r{r['round']} {r['bytes']}B added {r['added'][:16]} items={len(items)}")
    for i in items: print('  *',i)
