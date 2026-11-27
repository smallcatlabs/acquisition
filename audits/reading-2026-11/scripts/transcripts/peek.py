import sys,os; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
S=load()
want=sys.argv[1:]
for s in S:
    b=os.path.basename(s["file"])[:8]
    if b not in want: continue
    tools=[x for x in s["ev"] if x["k"]=="tool"]
    for i,x in enumerate(tools):
        if broad_edit(x) and not strict_edit(x):
            c=x["input"]["command"]; print(b,i,repr(c[:300])); print('   ...',[m for m in re.findall(r"crates/[\w./-]+",c)][:4]); break
