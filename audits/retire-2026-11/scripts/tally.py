"""The census's table: every entry's fate, from the class readers and the deep readers.

usage, from the tree's root:
  python3 -I audits/retire-2026-11/scripts/tally.py table            the table, one row per entry (markdown)
  python3 -I audits/retire-2026-11/scripts/tally.py counts           counts per area and fate, and the cost columns
  python3 -I audits/retire-2026-11/scripts/tally.py agree            the second read against the first, per judgment
  python3 -I audits/retire-2026-11/scripts/tally.py work             by fate, each entry's comment to move to and its unheld clauses, in the reader's words
  python3 -I audits/retire-2026-11/scripts/tally.py check            where a reader's fate is not the one its own row derives

Reads `reports/class-a.md`, `reports/class-b.md`, `reports/first/*.md`
and `reports/second/*.md` beside this directory (or under the
directory `CENSUS_REPORTS` names). Writes nothing.

The fate is derived, never weighed (`BRIEF.md`, "The fate"):
- the side is the class readers' where they agree; where they differ,
  the deep reader's class decides, as the third read and the one that
  opened the code;
- an owner's entry is KO, or SH-KO where both class readers mark a
  clause M;
- a session's entry takes the fate its deep reader's row derives;
- an entry that would go (RN, RM, NT-cut, NT-simple) and that the
  re-run found `open` (a promise on things not yet written, which no
  door, sweep or type would catch) is the owner's after all, by the
  sharpened `cross` (`RERUN-BRIEF.md`): KO, or SH-KO with M clauses.
  An `open` its reader marked `low` moves nothing and is shown.

Two verdicts of the pilot reader (`reports/first/trade.md`) were made
under a sentence the brief then sharpened: C99 clause 2 and C113
clause 1 call "no network" partly held because the dependency edge
does not refuse the standard library's sockets. The brief as the other
readers had it calls that edge Y. PILOT says so, and `load` applies it.
"""
import glob, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import registry, classes

HERE = os.path.dirname(os.path.abspath(__file__))
REPORTS = os.environ.get("CENSUS_REPORTS") or os.path.join(HERE, "..", "reports")
REG = registry.REG
PILOT = {("C99", 2), ("C113", 1)}   # clauses held by the HTTP-client edge, read as P by the pilot
FATES = ["KO", "SH-KO", "KU", "SH-KU", "RN", "RM", "NT-cut", "NT-simple", "NT-complex"]


def ints(cell):
    return [int(x) for x in re.findall(r"\d+", cell)]


def deep_rows(path):
    """A deep report's `## Rows` table and its per-entry blocks."""
    out, text = {}, open(path, encoding="utf-8").read()
    for line in text.splitlines():
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if not (re.fullmatch(r"C\d+", cells[0]) and len(cells) >= 10):
            continue
        held = ints(cells[4]) + [0] * 5
        scope = ints(cells[7]) + [0] * 3
        worth = ints(cells[8]) + [0] * 2
        out[cells[0]] = {"class": cells[1], "P": int(cells[2]), "M": int(cells[3]),
                         "Y": held[0], "Pt": held[1], "N": held[2], "U": held[3], "R": held[4],
                         "reason": cells[5], "fate": cells[6].split()[0], "simple": scope[0], "complex": scope[1],
                         "cut": scope[2], "own": worth[0], "para": worth[1], "conf": cells[9],
                         "report": os.path.basename(path), "clauses": {}, "unheld": {}, "said": {}, "moves": ""}
    cur, zone = None, ""
    for line in text.splitlines():
        m = re.match(r"\s*### (C\d+)\b", line)
        if m:
            cur, zone = out.get(m.group(1)), ""
            continue
        if line.strip().startswith("## "):
            cur = None
        if cur is None:
            continue
        m = re.match(r"\s*-\s*(class|clauses|unheld|reason|fate|confidence)\b", line)
        if m:
            zone = m.group(1)
        m = re.match(r"\s*(\d+)\.\s*([PM])\s*·\s*([YPNUR])\b", line)
        if m and zone == "clauses":
            cur["clauses"][int(m.group(1))] = (m.group(2), m.group(3))
        if zone == "unheld":
            # one clause a line, several on a line apart by `;`, or `1, 3 simple` for two alike
            for nums, scope, worth in re.findall(r"((?:\d+\s*(?:,|and)\s*)*\d+)\s+(simple|complex|cut)\b(?:\s*·\s*(ggg|data|wrong|para)\b)?", line):
                for n in re.findall(r"\d+", nums):
                    cur["unheld"][int(n)] = (scope, worth)
                    cur["said"][int(n)] = re.sub(r"^\s*(?:-\s*)?(?:unheld:\s*)?", "", line).strip()
        m = re.search(r"moves to\s+(.*)$", line)
        if m and zone == "reason":
            cur["moves"] = m.group(1).strip()
    return out


def derive(row):
    """The fate a deep row's own counts give a session's entry.

    The brief's four steps left three shapes undecided, which the
    readers reported; they are decided here, once, for every row:
    - an entry whose first promise is one nothing can hold (U: a rule on
      people, or on what is built next) is KU, whatever else a check
      holds, because the entry is the only thing that says it;
    - a U clause further down, beside clauses that are held, leaves with
      the cut ones: it is no test to write, and it moves to the holder's
      comment or to a "Not doing" line when the entry retires;
    - an entry with no promise at all (every clause M) is judged as
      step 3 judges: held or stated at the code, it retires.
    """
    promises = [held for _, (kind, held) in sorted(row["clauses"].items()) if kind == "P"]
    all_u = row["P"] and row["U"] >= row["P"] and not (row["Y"] or row["Pt"] or row["N"])
    if all_u or (promises and promises[0] == "U"):
        return "SH-KU" if row["M"] else "KU"
    if row["complex"]:
        return "NT-complex"
    if row["simple"]:
        return "NT-simple"
    if row["fate"] in ("RN", "RM") and not row["U"]:
        # an M clause held nowhere but stated at the code does not hold an entry back
        return "RN" if row["reason"] == "Y" else "RM"
    if row["cut"] or row["U"] or row["Pt"] or row["N"]:
        return "NT-cut"
    return "RN" if row["reason"] == "Y" else "RM"


def load():
    a = classes.rows(os.path.join(REPORTS, "class-a.md"))
    b = classes.rows(os.path.join(REPORTS, "class-b.md"))
    first, second = {}, {}
    for p in sorted(glob.glob(os.path.join(REPORTS, "first", "*.md"))):
        first.update(deep_rows(p))
    for p in sorted(glob.glob(os.path.join(REPORTS, "second", "*.md"))):
        second.update(deep_rows(p))
    for cid, n in PILOT:
        d = first[cid]
        if d["clauses"].get(n, ("", ""))[1] == "P":
            d["clauses"][n] = (d["clauses"][n][0], "Y")
            scope, worth = d["unheld"].pop(n)
            d["said"].pop(n, None)
            d["Pt"] -= 1; d["Y"] += 1; d[scope] -= 1
            d["own" if worth != "para" else "para"] -= 1
    return a, b, first, second


def rerun():
    """The re-run's verdict per entry: (verdict, clause, the new thing, what fails)."""
    out = {}
    for p in sorted(glob.glob(os.path.join(REPORTS, "rerun-*.md"))):
        for line in open(p, encoding="utf-8"):
            cells = [c.strip() for c in line.strip().strip("|").split("|")]
            if re.fullmatch(r"C\d+", cells[0]) and len(cells) >= 6 and cells[1] in ("open", "listed", "caught", "bound"):
                out[cells[0]] = (cells[1], cells[2], cells[3], cells[4], cells[5])
    return out


RERUN = None


def fate_of(cid, a, b, first):
    """(fate, side, how the side was settled)."""
    sa, sb = classes.side(a[cid]["class"]), classes.side(b[cid]["class"])
    d = first.get(cid)
    if sa == sb:
        side, how = sa, "both"
    elif d:
        side, how = classes.side(d["class"]), "third"
    else:
        side, how = "owner", "unread"
    if side == "owner":
        return ("SH-KO" if a[cid]["M"] & b[cid]["M"] else "KO"), side, how
    if not d:
        return "?", side, how
    fate = derive(d)
    global RERUN
    if RERUN is None:
        RERUN = rerun()
    r = RERUN.get(cid)
    if fate in ("RN", "RM", "NT-cut", "NT-simple") and r and r[0] == "open" and r[4] != "low":
        return ("SH-KO" if d["M"] else "KO"), "owner", "rerun"
    return fate, side, how


def area(cid):
    return REG.decisions[cid]["area"]


def main(argv):
    a, b, first, second = load()
    verb = argv[0] if argv else ""
    if verb == "table":
        fate_of(REG.order[0], a, b, first)
        print("| id | area | class A / B / deep | fate | clauses P+M | held Y/P/N/U/R | reason | unheld s/c/x | worth own/para | re-run | conf | report |")
        print("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |")
        for cid in REG.order:
            fate, side, how = fate_of(cid, a, b, first)
            d = first.get(cid)
            n = len(registry.clauses(REG.decisions[cid]))
            r = (RERUN or {}).get(cid)
            rr = "–" if not r else r[0] + (f" {r[1]}" if r[1] != "-" else "") + (" (low)" if r[4] == "low" else "")
            if d:
                print(f"| {cid} | {area(cid)} | {a[cid]['class']} / {b[cid]['class']} / {d['class']} | {fate} | {d['P']}+{d['M']} | "
                      f"{d['Y']}/{d['Pt']}/{d['N']}/{d['U']}/{d['R']} | {d['reason']} | {d['simple']}/{d['complex']}/{d['cut']} | "
                      f"{d['own']}/{d['para']} | {rr} | {d['conf']} | {d['report'][:-3]} |")
            else:
                m = len(a[cid]["M"] & b[cid]["M"])
                print(f"| {cid} | {area(cid)} | {a[cid]['class']} / {b[cid]['class']} / – | {fate} | {n - m}+{m} | – | – | – | – | – | "
                      f"{a[cid]['conf']} / {b[cid]['conf']} | class |")
    elif verb == "counts":
        areas = []
        for cid in REG.order:
            if area(cid) not in areas:
                areas.append(area(cid))
        grid = {ar: dict.fromkeys(FATES + ["?"], 0) for ar in areas}
        for cid in REG.order:
            grid[area(cid)][fate_of(cid, a, b, first)[0]] += 1
        cols = [f for f in FATES + ["?"] if any(grid[ar][f] for ar in areas)]
        print("| area | " + " | ".join(cols) + " | all |")
        print("| --- |" + " --- |" * (len(cols) + 1))
        for ar in areas:
            print(f"| {ar} | " + " | ".join(str(grid[ar][f]) for f in cols) + f" | {sum(grid[ar].values())} |")
        print("| all | " + " | ".join(str(sum(grid[ar][f] for ar in areas)) for f in cols) + f" | {len(REG.order)} |")
        deep = [c for c in REG.order if c in first and fate_of(c, a, b, first)[1] == "session"]
        un = [(c, n, s, w) for c in deep for n, (s, w) in sorted(first[c]["unheld"].items())]
        print(f"\ndeep-read session's entries {len(deep)}; unheld clauses listed {len(un)}: "
              + ", ".join(f"{k} {sum(1 for u in un if u[2] == k)}" for k in ("simple", "complex", "cut")))
        for k in ("simple", "complex"):
            ws = [u[3] for u in un if u[2] == k]
            print(f"  {k}: " + ", ".join(f"{w or 'unsaid'} {ws.count(w)}" for w in sorted(set(ws))))
        shed = [c for c in REG.order if fate_of(c, a, b, first)[0] == "SH-KO"]
        print(f"shed: {len(shed)} entries, M clauses by both class readers {sum(len(a[c]['M'] & b[c]['M']) for c in shed)}, "
              f"by either {sum(len(a[c]['M'] | b[c]['M']) for c in shed)}")
    elif verb == "work":
        # what each entry that is not simply kept would take, by fate, in the readers' own words
        for fate in ("RN", "RM", "NT-cut", "NT-simple", "NT-complex", "KU", "SH-KU"):
            ids = [c for c in REG.order if fate_of(c, a, b, first)[0] == fate]
            print(f"\n## {fate} ({len(ids)})\n")
            for c in ids:
                d = first[c]
                print(f"- **{c}** ({area(c)}, `{d['report'][:-3]}`), reason {d['reason']}: moves to {d['moves'] or '?'}")
                seen = set()
                for n in sorted(d["said"]):
                    if d["said"][n] not in seen:
                        seen.add(d["said"][n])
                        print(f"  - {d['said'][n]}")
    elif verb == "check":
        for cid in REG.order:
            d = first.get(cid)
            if not d:
                continue
            fate, side, how = fate_of(cid, a, b, first)
            said = d["fate"]
            said_full = said if said != "NT" else ("NT-complex" if d["complex"] else "NT-simple" if d["simple"] else "NT-cut")
            notes = []
            if side == "session" and said_full != fate:
                notes.append(f"reader says {said_full}, row derives {fate}")
            if side == "owner":
                notes.append(f"deep-read but owner's by {how} (reader's fate {said_full})")
            if d["P"] + d["M"] != len(registry.clauses(REG.decisions[cid])):
                notes.append(f"P+M {d['P'] + d['M']} but {len(registry.clauses(REG.decisions[cid]))} clauses")
            if len(d["clauses"]) != d["P"] + d["M"]:
                notes.append(f"{len(d['clauses'])} clause lines parsed")
            if notes:
                print(cid, d["report"], "; ".join(notes))
        missing = [c for c in REG.order if fate_of(c, a, b, first)[0] == "?"]
        print("unread session's entries:", " ".join(missing) or "none")
    elif verb == "agree":
        both = [c for c in REG.order if c in first and c in second]
        print(f"entries read twice: {len(both)}")
        full = lambda d: derive(d) if classes.side(d["class"]) == "session" else "owner"
        same = [c for c in both if full(first[c]) == full(second[c])]
        print(f"fate, as each row derives it: {len(same)} of {len(both)} agree")
        coarse = lambda f: "retire" if f in ("RN", "RM") else "test" if f.startswith("NT") else f
        print(f"fate, coarse (retire / needs a test / keep unheld / owner's): {sum(coarse(full(first[c])) == coarse(full(second[c])) for c in both)} of {len(both)}")
        for name in ("RN", "RM", "NT-cut", "NT-simple", "NT-complex", "KU", "SH-KU", "owner"):
            f1 = [c for c in both if full(first[c]) == name]
            f2 = [c for c in both if full(second[c]) == name]
            if f1 or f2:
                print(f"  {name}: first {len(f1)}, second {len(f2)}, both {len(set(f1) & set(f2))}")
        print(f"side: {sum(classes.side(first[c]['class']) == classes.side(second[c]['class']) for c in both)} of {len(both)}")
        print(f"reason at the code: {sum(first[c]['reason'] == second[c]['reason'] for c in both)} of {len(both)}")
        kind = held = n = 0
        loose1 = loose2 = looseboth = 0
        for c in both:
            for i in set(first[c]["clauses"]) & set(second[c]["clauses"]):
                k1, h1 = first[c]["clauses"][i]; k2, h2 = second[c]["clauses"][i]
                n += 1; kind += k1 == k2; held += h1 == h2
                l1, l2 = h1 in "PN", h2 in "PN"
                loose1 += l1; loose2 += l2; looseboth += l1 and l2
        print(f"clauses read twice: {n}; same kind (P or M) {kind}; same held verdict {held}")
        print(f"clauses not held (P or N): first {loose1}, second {loose2}, both {looseboth}")
        s = [(c, i) for c in both for i in set(first[c]['unheld']) & set(second[c]['unheld'])]
        print(f"clauses both list as unheld: {len(s)}; same scope {sum(first[c]['unheld'][i][0] == second[c]['unheld'][i][0] for c, i in s)}; "
              f"same worth {sum(first[c]['unheld'][i][1] == second[c]['unheld'][i][1] for c, i in s)}")
        print("\nper entry (first | second):")
        for c in both:
            mark = "" if full(first[c]) == full(second[c]) else "   <-- differs"
            print(f"  {c}: {first[c]['class']} {full(first[c])} {first[c]['reason']} | {second[c]['class']} {full(second[c])} {second[c]['reason']}{mark}")
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main(sys.argv[1:])
