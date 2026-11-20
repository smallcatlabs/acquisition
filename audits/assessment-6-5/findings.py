#!/usr/bin/env python3
"""The assessment's first read (#293): every finding of the reviews under
roadmap/reviews/ first committed on or after SINCE, by step, by area, by
file and by shape, beside each step's close and its retirement lines.

Run from the repository root: python3 -I audits/assessment-6-5/findings.py

A finding is a line of a build review that opens with P1, P2 or P3 and a
place. Three build reviews use another form and are counted by their
numbered heads, ungraded. A brief's review is counted by its numbered or
"Changes what gets built" items, and its items are not tabulated by file:
they are against the brief. The saved prompts (`-prompt.md`) are skipped.
"""
import collections
import re
import subprocess

SINCE = "2026-11-06"  # the first review of 4-full part 1, the GUI's first slice
FINDING = re.compile(
    r"(?m)^\s*(?:[-*]|\d+\.)?\s*\**\s*\[?(P[123])\]?\s*[—–-]*\s*"
    r"(\[[^\]]+\]\(([^)]+)\)|`[^`]+`|[\w./-]+:\d+[\w./:-]*|commit message[:\d]*)([^\n]*)"
)


def git(*args):
    return subprocess.run(["git", *args], capture_output=True, text=True, check=True).stdout


def step_of(name):
    name = re.sub(r"-[0-9a-f]{8}\.md$", "", name)
    return re.sub(r"-(fixes|backout-fixes|backout|build|brief)$", "", name)


def area_of(path):
    if path.startswith("crates/"):
        crate = path.split("/")[1].replace("acquisition-", "")
        kind = "tests" if ("/tests/" in path or "/unit/" in path) else "src"
        return f"{crate} {kind}"
    if path.startswith(("tools/", ".githooks", ".github")):
        return "tools, hooks, workflows"
    if path.startswith("audits/"):
        return "audits (the spike's rig, a seat's record)"
    return "documents, rulings, skills, messages"


def reviews():
    out = []
    for name in sorted(git("ls-files", "roadmap/reviews").split()):
        base = name.rsplit("/", 1)[1]
        if base.endswith("-prompt.md") or not re.search(r"-[0-9a-f]{8}\.md$", base):
            continue
        added = git("log", "--diff-filter=A", "--format=%ad", "--date=short", "-1", "--", name).strip()
        if added >= SINCE:
            out.append((added, base))
    return sorted(out)


def closes():
    """Step trailer -> (sha, the close's `retired X; held by Y` lines, or what it says instead)."""
    out = {}
    log = git("log", f"--since={SINCE}", "--grep=^Step: ", "--format=%x00%h%n%B")
    for entry in log.split("\x00")[1:]:
        sha, body = entry.split("\n", 1)
        step = re.search(r"(?m)^Step: (\S+)", body)
        lines = len(re.findall(r"(?m)^[ -]*retired .*; held by", body))
        nothing = bool(
            re.search(r"(?i)retired[^.;\n]*: *(nothing|none)|retired nothing|nothing was retired", body)
        )
        said = "says none" if nothing else ("in prose" if re.search(r"(?i)retired", body) else "none stated")
        if step:
            out[step.group(1)] = (sha, str(lines) if lines else said)
    return out


def main():
    steps = collections.OrderedDict()
    findings = []
    for added, base in reviews():
        text = open(f"roadmap/reviews/{base}").read()
        row = steps.setdefault(
            step_of(base),
            dict(first=added, brief=0, build=0, P1=0, P2=0, P3=0, ungraded=0, items=0, clean=0),
        )
        if "-brief-" in base or base.startswith("4-full-1-design"):
            row["brief"] += 1
            numbered = len(set(re.findall(r"(?m)^(\d+)\.\s+\*\*", text)))
            marked = len(re.findall(r"(?i)changes what gets built", text)) + len(
                re.findall(r"(?m)^[-*\d. ]*\**Wording", text)
            )
            row["items"] += max(numbered, marked)
            continue
        row["build"] += 1
        seen = set()
        for sev, loc, link, rest in FINDING.findall(text):
            path = re.sub(r"^/.*?/acquisition/", "", link or loc.strip("`"))
            path = re.sub(r"^\.claude/worktrees/[^/]+/", "", path)
            path = re.sub(r":\d+.*$", "", path)
            if (sev, path, rest[:60]) in seen:
                continue
            seen.add((sev, path, rest[:60]))
            shapes = re.findall(r"\d+", " ".join(re.findall(r"shapes?\s+([\d,/ and]+)", rest[:80])))
            findings.append((base, sev, path, shapes))
            row[sev] += 1
        if not seen:
            ungraded = len(re.findall(r"(?m)^\*\*\d+\.", text))
            row["ungraded"] += ungraded
            row["clean"] += ungraded == 0

    closed = closes()
    alias = {}
    print("## By step\n")
    print("| Step | First review | Brief reviews (items) | Build reviews | P1 | P2 | P3 | Ungraded | Reviews with no finding | Close | Retirement lines |")
    print("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |")
    for step, r in steps.items():
        sha, retired = closed.get(alias.get(step, step), ("", ""))
        brief = f"{r['brief']} ({r['items']})" if r["brief"] else ""
        cells = [f"`{step}`", r["first"][5:], brief] + [
            str(r[k] or "") for k in ("build", "P1", "P2", "P3", "ungraded", "clean")
        ]
        print("| " + " | ".join(cells + [f"`{sha}`" if sha else "", retired]) + " |")
    total = lambda k: sum(r[k] for r in steps.values())
    print(
        f"\n{len(steps)} steps, {total('brief')} brief reviews ({total('items')} items), "
        f"{total('build')} build reviews, {len(findings)} graded findings "
        f"(P1 {total('P1')}, P2 {total('P2')}, P3 {total('P3')}), {total('ungraded')} ungraded."
    )

    print("\n## By area\n\n| Area | Findings | P1 | P2 | P3 |\n| --- | --- | --- | --- | --- |")
    by_area = collections.defaultdict(collections.Counter)
    for _, sev, path, _ in findings:
        by_area[area_of(path)][sev] += 1
    for area, c in sorted(by_area.items(), key=lambda kv: -sum(kv[1].values())):
        print(f"| {area} | {sum(c.values())} | {c['P1']} | {c['P2']} | {c['P3']} |")

    print("\n## By file, three findings or more\n\n| File | Findings | Steps |\n| --- | --- | --- |")
    by_file = collections.defaultdict(list)
    for base, _, path, _ in findings:
        by_file[path].append(step_of(base))
    for path, where in sorted(by_file.items(), key=lambda kv: -len(kv[1])):
        if len(where) >= 3:
            named = ", ".join(f"`{s}`" for s in sorted(set(where)))
            print(f"| `{path}` | {len(where)} | {named} |")

    print("\n## By shape\n\n| Shape of the review prompt | Findings citing it |\n| --- | --- |")
    by_shape = collections.Counter()
    for _, _, _, shapes in findings:
        for shape in shapes or ["none cited"]:
            by_shape[shape] += 1
    for shape, n in by_shape.most_common():
        print(f"| {shape} | {n} |")


if __name__ == "__main__":
    main()
