"""Weekly commit classes: what each commit touches."""
import subprocess, re, collections, sys
out = subprocess.run(["git","log","--since=2026-10-01","--format=@%H\t%ad\t%s","--date=format:%G-W%V","--name-only"],capture_output=True,text=True).stdout
commits=[]; cur=None
for line in out.splitlines():
    if line.startswith("@"):
        h,wk,subj=line[1:].split("\t",2); cur={"h":h,"wk":wk,"s":subj,"f":[]}; commits.append(cur)
    elif line.strip(): cur["f"].append(line.strip())
def cls(c):
    f=c["f"]
    def any_(p): return any(re.search(p,x) for x in f)
    crate_src = any_(r"(^|/)acquisition-[a-z]+/src/") 
    crate_tests = any_(r"(^|/)acquisition-[a-z]+/tests/")
    if crate_src: return "product (crate src)"
    if crate_tests: return "tests only"
    if any_(r"^tools/|^\.github/|^\.githooks/|^\.config/|^\.cargo/|Cargo\.(toml|lock)$"): return "tools/ci/build"
    if f and all(re.search(r"^roadmap/reviews/",x) for x in f): return "saved review only"
    if f and all(re.search(r"^roadmap/",x) for x in f): return "road/brief only"
    if f and all(re.search(r"^(audits|search|runs|spikes|docs)/",x) for x in f): return "audit/research/docs dirs"
    if f and all(re.search(r"\.md$",x) for x in f): return "governance/docs md"
    return "other"
tab=collections.defaultdict(collections.Counter)
for c in commits: tab[c["wk"]][cls(c)]+=1
classes=["product (crate src)","tests only","tools/ci/build","saved review only","road/brief only","audit/research/docs dirs","governance/docs md","other"]
print("week\t"+"\t".join(classes)+"\ttotal")
for wk in sorted(tab):
    print(wk+"\t"+"\t".join(str(tab[wk][k]) for k in classes)+"\t"+str(sum(tab[wk].values())))
