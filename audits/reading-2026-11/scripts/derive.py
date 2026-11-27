import re, glob, subprocess, collections
ents = {}
for f in sorted(glob.glob('decisions/*.md')) + ['AGENTS.md']:
    for line in open(f, encoding='utf-8'):
        m = re.match(r'- \*\*(C\d+) — ', line)
        if m:
            p = line.split('*Pinned:*',1)[1].strip() if '*Pinned:*' in line else ''
            ents[m.group(1)] = (f, p)
def files(cid):
    out = subprocess.run(['git','grep','-l','-w','-i',cid,'--','crates','tools','.github','.githooks'],capture_output=True,text=True).stdout.split()
    return out
held_no_cite = []; kinds = collections.Counter()
for cid,(f,p) in ents.items():
    fs = files(cid)
    test = [x for x in fs if '/tests/' in x or x.endswith('tests.rs') or '/unit/' in x]
    tool = [x for x in fs if x.startswith('tools/') or x.startswith('.git')]
    # id-named test fn anywhere (inline #[cfg(test)] too)
    named = subprocess.run(['git','grep','-l','-E',r'fn %s_' % cid.lower(),'--','crates'],capture_output=True,text=True).stdout.split()
    unheld = p.lower().startswith('nothing yet')
    k = ('unheld' if unheld else 'pinned') + ':' + ('test-or-tool-cites' if (test or tool or named) else ('src-only' if fs else 'no-cite'))
    kinds[k]+=1
    if not unheld and not (test or tool or named): held_no_cite.append(cid)
for k,v in sorted(kinds.items()): print(v,k)
print('pinned but no test/tool file cites the id:', ' '.join(held_no_cite))
