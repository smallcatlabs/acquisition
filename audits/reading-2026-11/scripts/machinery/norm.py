import re,sys
NORM=re.compile(r"\b(never|must|always|only|do not|don't|refuse[sd]?|before|first|is the session's|is his|is the owner's|stands alone|takes no|run[s]? |read[s]? |write[s]? |commit[s]? |keep|no \w+ (is|may)|at most|not until)\b",re.I)
for f in sys.argv[1:]:
    t=open(f,encoding='utf-8').read()
    t=re.sub(r"```.*?```"," ",t,flags=re.S)
    t=re.sub(r"\s+"," ",t)
    sents=[s for s in re.split(r"(?<=[.!?:])\s+(?=[A-Z*`\-0-9])",t) if len(s)>15]
    n=sum(1 for s in sents if NORM.search(s))
    print(f"{n:4d} / {len(sents):4d}  {f}")
