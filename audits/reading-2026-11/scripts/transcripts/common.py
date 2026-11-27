import json, os, re
HERE = os.path.dirname(os.path.abspath(__file__))
def load():
    return json.load(open(os.path.join(HERE, "events.json")))

EDIT_TOOLS = ("Edit", "Write", "NotebookEdit", "MultiEdit")
def strict_edit(x):
    return x["k"] == "tool" and x["name"] in EDIT_TOOLS and "/crates/" in (x["input"].get("file_path") or "")
WR = re.compile(r"apply_patch|python3?( -I)? - <<|python3?( -I)? <<|perl -0?pi|sed -i|cat >>? ?\S*crates/|tee \S*crates/|write_text|\.write\(")
TGT_LIT = re.compile(r"""(?:\b\w+\s*=\s*|open\(\s*|Path\(\s*|\[\s*)[rbf]?['"]([^'"\n]*crates/[^'"\n]+)['"]""")
def edit_targets(c):
    t = []
    t += re.findall(r"\*\*\* (?:Update|Add|Delete) File: (\S+)", c)
    for seg in re.split(r"[;&|\n]", c.split("<<")[0]):
        if re.search(r"\bsed -i|\bperl -0?p?i", seg):
            t += [w for w in seg.split() if "crates/" in w and not w.startswith(("s/", "'s", '"s'))]
        m = re.search(r"(?:cat|tee|echo[^>]*)\s*>>?\s*(\S+)", seg)
        if m: t.append(m.group(1))
        for m in re.finditer(r"python3?(?: -I)? (?:-\s+)?((?:\S*crates/\S+\s*)+)<<", seg + "<<"):
            t += m.group(1).split()
    if re.search(r"python3?( -I)? (- )?.*<<", c):
        body = c[c.index("<<"):]
        head = body[:600]
        t += TGT_LIT.findall(head)
        t += re.findall(r"(?:open|Path)\(\s*[rbf]?['\"]([^'\"\n]*crates/[^'\"\n]+)['\"]", body)
        # shell var assignments before the heredoc used as argv
        t += re.findall(r"(?:^|[;&\s])\w+=(\S*crates/\S+?)(?:\s|;|&|$)", c.split("<<")[0])
    return [x for x in t if "crates/" in x]
def broad_edit(x):
    if strict_edit(x): return True
    if x["k"] != "tool" or x["name"] != "Bash": return False
    c = x["input"].get("command", "")
    if "crates/" not in c: return False
    if re.match(r"\s*(cd \S+\s*(&&|;)\s*)?(cargo|git (commit|add|log|diff|show|grep))\b", c): return False
    if not WR.search(c): return False
    return bool(edit_targets(c))

GOV = [
    ("decisions", re.compile(r"decisions/([a-z-]+)\.md")),
    ("REVIEW-CHECKLIST", re.compile(r"REVIEW-CHECKLIST\.md")),
    ("roadmap/README", re.compile(r"roadmap/README\.md")),
    ("brief", re.compile(r"roadmap/briefs/[^\s'\"]+")),
    ("review-file", re.compile(r"roadmap/reviews/[^\s'\"]+")),
    ("TESTING-NOTES", re.compile(r"TESTING-NOTES\.md")),
    ("GLOSSARY", re.compile(r"GLOSSARY\.md")),
    ("CLI-REFERENCE", re.compile(r"CLI-REFERENCE\.md")),
    ("MCP-REFERENCE", re.compile(r"MCP-REFERENCE\.md")),
    ("search/README", re.compile(r"(?<![a-z])search/README\.md")),
    ("skill", re.compile(r"\.claude/skills/([a-z-]+)/SKILL\.md")),
    ("AGENTS", re.compile(r"(?<![A-Za-z/-])AGENTS\.md|acquisition/AGENTS\.md")),
    ("HISTORY", re.compile(r"HISTORY\.md")),
]
READCMD = re.compile(r"^\s*(cat|sed|head|tail|less|nl|awk|wc)\b|\|\s*(sed|head|tail)\b|;\s*(cat|sed|head|tail)\b|&&\s*(cat|sed|head|tail)\b|\n\s*(cat|sed|head|tail)\b")
def gov_reads(x):
    """Return list of (doc, mode) read by this call. mode: whole | range | grep."""
    out = []
    if x["k"] != "tool": return out
    if x["name"] == "Read":
        fp = x["input"].get("file_path") or ""
        whole = not x["input"].get("offset") and not x["input"].get("limit")
        for name, rx in GOV:
            m = rx.search(fp)
            if m:
                d = name + (":" + m.group(1) if name in ("decisions", "skill") else "")
                out.append((d, "whole" if whole else "range"))
        return out
    if x["name"] == "Bash":
        c = x["input"].get("command", "")
        if "bothy.py" in c and not READCMD.search(c): return out
        for name, rx in GOV:
            for m in rx.finditer(c):
                # decide the verb acting on it
                pre = c[:m.start()]
                seg = re.split(r"[|;&\n]", pre)[-1]
                if re.search(r"\b(cat|nl)\b", seg) and not re.search(r"cat\s*>", seg): mode = "whole"
                elif re.search(r"\b(sed -n|head|tail|awk)\b", seg): mode = "range"
                elif re.search(r"\b(grep|rg|git grep)\b", seg): mode = "grep"
                elif re.search(r"\bwc\b", seg): mode = "wc"
                else: continue
                d = name + (":" + m.group(1) if name in ("decisions", "skill") else "")
                out.append((d, mode))
    return out

def bothy(x):
    if x["k"] != "tool" or x["name"] != "Bash": return []
    c = x["input"].get("command", "")
    return re.findall(r"bothy\.py\s+(pack|show|list|check)([^\n|;&]*)", c)
