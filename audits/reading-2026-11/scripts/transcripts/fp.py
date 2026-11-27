import sys, os; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
import collections
H=os.path.dirname(os.path.abspath(__file__))
B={b['name']:b for b in json.load(open(H+'/builders.json'))}
S=load(); named=collections.Counter(); obeyed=collections.Counter(); packnamed=0; packobeyed=0; checklist=0; checkobeyed=0
gtarget=collections.Counter()
for s in S:
    n=os.path.basename(s['file'])[:8]
    if n not in B or s['sub']: continue
    b=B[n]; fp=s['first']
    for a in set(re.findall(r"decisions/([a-z-]+)\.md",fp)):
        named[a]+=1
        if ('decisions:'+a) in b['pre'] and set(b['pre']['decisions:'+a][1])&{'whole','range'}: obeyed[a]+=1
    if 'bothy' in fp and 'pack' in fp:
        packnamed+=1; packobeyed+= bool(b['pack'])
    if 'REVIEW-CHECKLIST' in fp:
        checklist+=1; checkobeyed+= 'REVIEW-CHECKLIST' in b['pre'] and bool(set(b['pre']['REVIEW-CHECKLIST'][1])&{'whole','range','grep'})
    for x in s['ev']:
        if x['k']=='tool' and x['name']=='Bash':
            c=x['input'].get('command','')
            if 'grep' in c and re.search(r"(?<![-\w])C\d{1,3}\b",c):
                gtarget['decisions/' if 'decisions' in c else 'crates/' if 'crates' in c or 'acquisition-' in c else 'git log' if 'git log' in c else 'other']+=1
print('first prompts naming decisions/<x>.md:',named,'read before edit:',obeyed)
print('prompts naming bothy pack',packnamed,'ran pack before edit',packobeyed)
print('prompts naming REVIEW-CHECKLIST',checklist,'read/grepped before edit',checkobeyed)
print('C-id grep targets',gtarget)
