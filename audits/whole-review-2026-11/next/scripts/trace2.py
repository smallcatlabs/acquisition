#!/usr/bin/env python3
"""Second pass: which files agents read and write through the shell, and how edits are made."""
import collections, datetime as dt, glob, json, os, re, subprocess, sys

P = os.path.expanduser(os.environ.get("ACQ_TRANSCRIPTS", "transcripts"))
ROOT = os.path.abspath(os.environ.get("ACQ_REPO", "."))
SINCE = sys.argv[1] if len(sys.argv) > 1 else "2026-10-12"
tracked = set(subprocess.run(["git", "-C", ROOT, "ls-files"], capture_output=True, text=True).stdout.split("\n"))
# every path that ever existed, so renamed files still count
ever = set(subprocess.run(["git", "-C", ROOT, "log", "--since=2026-10-01", "--name-only", "--pretty=format:"], capture_output=True, text=True).stdout.split("\n"))
known = tracked | ever
PATH = re.compile(r"[A-Za-z0-9_.\-/]+\.(?:md|rs|sh|py|toml|txt|json|yml|tsv|sql|js|css|html)\b|AGENTS\.md")

mention = collections.Counter()      # commands naming the file
mention_t = collections.defaultdict(set)
solo_bytes = collections.Counter()   # output bytes of commands naming exactly one file
cls_bytes = collections.Counter()
cls_calls = collections.Counter()
writes = collections.Counter()
write_kind = collections.Counter()
sessions_with_first = collections.Counter()
weekly = collections.defaultdict(collections.Counter)


def area(p):
    parts = p.split("/")
    if parts[0] == "crates" and len(parts) > 2:
        kind = "tests" if ("tests" in parts or "unit" in parts) else "src"
        if "goldens" in parts or "fixtures" in parts:
            kind = "goldens"
        return parts[1].replace("acquisition-", "") + "/" + kind
    if parts[0] == "roadmap" and len(parts) > 1 and parts[1] in ("briefs", "reviews"):
        return "roadmap/" + parts[1]
    return parts[0]


def norm(tok):
    tok = tok.replace(ROOT + "/", "")
    tok = re.sub(r"^\./", "", tok)
    if tok in known:
        return tok
    return None


for f in glob.glob(P + "/**/*.jsonl", recursive=True):
    pend = {}
    first = []
    for line in open(f, errors="replace"):
        if '"tool_use"' not in line and '"tool_result"' not in line:
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("type") not in ("assistant", "user"):
            continue
        when = d.get("timestamp", "")[:10]
        if when < SINCE:
            continue
        content = (d.get("message") or {}).get("content")
        if not isinstance(content, list):
            continue
        for b in content:
            if not isinstance(b, dict):
                continue
            if b.get("type") == "tool_use" and b.get("name") == "Bash":
                cmd = (b.get("input") or {}).get("command", "")
                paths = []
                for m in PATH.finditer(cmd):
                    n = norm(m.group(0))
                    if n and n not in paths:
                        paths.append(n)
                is_write = bool(re.search(r"(>\s*[A-Za-z./\"$]|\bsed -i|perl -[0-9a-z]*i|\.write\(|write_text|open\([^)]*['\"][wa]['\"]|\btee\b|apply_patch|git apply|\bpatch\b)", cmd)) and not re.search(r">\s*/dev/null", cmd.replace("2>/dev/null", ""))
                wk = None
                if is_write:
                    if re.search(r"python3?\b", cmd) and re.search(r"\.write\(|write_text|open\(", cmd):
                        wk = "python rewrite"
                    elif re.search(r"sed -i|perl -[0-9a-z]*i", cmd):
                        wk = "sed/perl in place"
                    elif re.search(r"git apply|apply_patch|\bpatch\b", cmd):
                        wk = "patch"
                    elif re.search(r"cat\s*>\s*\S+\s*<<|<<\s*['\"]?\w+['\"]?\s*>\s*\S+|>\s*\S+\s*<<", cmd):
                        wk = "heredoc"
                    else:
                        wk = "redirect"
                pend[b.get("id")] = (paths, wk, when)
                for p in paths:
                    mention[p] += 1
                    mention_t[p].add(f)
                    if len(first) < 12 and p not in first:
                        first.append(p)
                if wk:
                    write_kind[wk] += 1
                    for p in paths:
                        if wk in ("python rewrite", "sed/perl in place", "patch", "heredoc"):
                            writes[p] += 1
            elif b.get("type") == "tool_result":
                got = pend.pop(b.get("tool_use_id"), None)
                if not got:
                    continue
                paths, wk, when = got
                c = b.get("content")
                size = sum(len(x.get("text", "")) for x in c if isinstance(x, dict)) if isinstance(c, list) else len(c or "")
                if paths:
                    share = size / len(paths)
                    for p in paths:
                        a = area(p)
                        cls_bytes[a] += share
                        cls_calls[a] += 1
                        y, w, _ = dt.date.fromisoformat(when).isocalendar()
                        weekly[f"W{w}"][a] += share
                    if len(paths) == 1:
                        solo_bytes[paths[0]] += size
    for p in first:
        sessions_with_first[p] += 1


def table(title, rows, head):
    print("\n## " + title)
    print(" | ".join(head))
    for r in rows:
        print(" | ".join(str(x) for x in r))


tot = sum(cls_bytes.values())
table("Shell output bytes by the area of the files a command names", [
    (a, f"{b/1e6:.1f} MB", f"{100*b/tot:.0f}%", cls_calls[a]) for a, b in cls_bytes.most_common(40)], ["area", "bytes", "share", "commands"])

table("Files named by the most shell commands", [
    (p, c, len(mention_t[p]), f"{solo_bytes[p]/1e6:.2f}") for p, c in mention.most_common(60)], ["file", "commands", "transcripts", "MB when alone"])

table("Files among the first dozen a transcript names in the shell", [
    (p, c) for p, c in sessions_with_first.most_common(30)], ["file", "transcripts"])

table("How the shell writes files", write_kind.most_common(), ["kind", "commands"])
table("Files written through the shell most", writes.most_common(40), ["file", "commands"])
aw = collections.Counter()
for p, c in writes.items():
    aw[area(p)] += c
table("Shell writes by area", aw.most_common(30), ["area", "commands"])
