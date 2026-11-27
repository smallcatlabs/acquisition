"""The two class readers' tables, compared, and the deep readers' batches cut from them.

usage, from the tree's root:
  python3 -I audits/retire-2026-11/scripts/classes.py agree A.md B.md      agreement, and each entry the two class differently
  python3 -I audits/retire-2026-11/scripts/classes.py batches A.md B.md    the deep readers' batches and the seeded fifth for the second read

An entry is read in depth when either reader calls it a session's
(design, mech, check, run). Batches follow the registry's order within
an area and hold at most CLAUSES clauses, the size at which the pilot
reader's context was full. Reads only.
"""
import math, os, random, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import registry

OWNER = {"ggg", "data", "edge", "scope", "rule", "cross"}
SESSION = {"design", "mech", "check", "run"}
CLAUSES = 34
SEED = 20261127


def rows(path):
    out = {}
    for line in open(path, encoding="utf-8"):
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if re.fullmatch(r"C\d+", cells[0]) and len(cells) >= 6:
            nums = lambda s: set() if s in ("-", "") else {int(x) for x in re.findall(r"\d+", s)}
            out[cells[0]] = {"class": cells[1], "M": nums(cells[2]), "restated": nums(cells[3]),
                             "conf": cells[4], "else": cells[5], "says": cells[6] if len(cells) > 6 else ""}
    return out


def side(c):
    return "owner" if c in OWNER else "session" if c in SESSION else "?"


def deep_set(a, b):
    return [c for c in registry.REG.order if side(a[c]["class"]) == "session" or side(b[c]["class"]) == "session"]


def batches(ids):
    """Runs of the registry's order, an area at a time, of at most CLAUSES clauses."""
    reg = registry.REG
    count = lambda c: len(registry.clauses(reg.decisions[c]))
    by = {}
    for c in ids:
        area = reg.decisions[c]["area"]
        by.setdefault("process" if area == "ci" else area, []).append(c)
    out = []
    for area, cs in by.items():
        n = max(1, math.ceil(sum(count(c) for c in cs) / CLAUSES))
        target = sum(count(c) for c in cs) / n
        runs, cur, acc = [], [], 0
        for c in cs:
            if cur and acc + count(c) / 2 > target and len(runs) < n - 1:
                runs.append(cur); cur, acc = [], 0
            cur.append(c); acc += count(c)
        runs.append(cur)
        for k, r in enumerate(runs, 1):
            out.append((f"{area}-{k}" if len(runs) > 1 else area, r, sum(count(c) for c in r)))
    return out


def second(ids):
    """A fifth of the deep set, by area with largest remainders, seeded, in
    batches of at most CLAUSES clauses."""
    reg = registry.REG
    count = lambda c: len(registry.clauses(reg.decisions[c]))
    by = {}
    for c in ids:
        by.setdefault(reg.decisions[c]["area"], []).append(c)
    want = round(len(ids) / 5)
    quota = {a: want * len(v) / len(ids) for a, v in by.items()}
    take = {a: math.floor(q) for a, q in quota.items()}
    for a in sorted(by, key=lambda a: -(quota[a] - take[a]))[:want - sum(take.values())]:
        take[a] += 1
    rnd = random.Random(SEED)
    picked = []
    for a in sorted(by):
        picked += rnd.sample(sorted(by[a], key=lambda c: int(c[1:])), take[a])
    rnd.shuffle(picked)
    n = max(1, math.ceil(sum(count(c) for c in picked) / CLAUSES))
    size = math.ceil(len(picked) / n)
    return [(f"second-{k + 1}", sorted(picked[k * size:(k + 1) * size], key=lambda c: int(c[1:])),
             sum(count(c) for c in picked[k * size:(k + 1) * size])) for k in range(n)]


def main(argv):
    if len(argv) != 3 or argv[0] not in ("agree", "batches"):
        sys.exit(__doc__)
    a, b = rows(argv[1]), rows(argv[2])
    order = registry.REG.order
    missing = [c for c in order if c not in a or c not in b]
    if missing:
        sys.exit(f"rows missing: {missing}")
    if argv[0] == "agree":
        same_side = [c for c in order if side(a[c]["class"]) == side(b[c]["class"])]
        same_class = [c for c in order if a[c]["class"] == b[c]["class"]]
        print(f"entries {len(order)}; same side {len(same_side)} ({100 * len(same_side) / len(order):.0f}%); same class {len(same_class)} ({100 * len(same_class) / len(order):.0f}%)")
        for name, r in (("A", a), ("B", b)):
            n = sum(side(r[c]["class"]) == "session" for c in order)
            print(f"{name}: session's {n}, owner's {len(order) - n}")
        both = sum(side(a[c]["class"]) == "session" == side(b[c]["class"]) for c in order)
        print(f"both session's {both}; either {len(deep_set(a, b))}; both owner's {len(order) - len(deep_set(a, b))}")
        clauses = sum(len(registry.clauses(registry.REG.decisions[c])) for c in order)
        ma = sum(len(a[c]["M"]) for c in order); mb = sum(len(b[c]["M"]) for c in order)
        mboth = sum(len(a[c]["M"] & b[c]["M"]) for c in order)
        print(f"clauses {clauses}; M by A {ma}, by B {mb}, by both {mboth}, by either {ma + mb - mboth}")
        print("\nside differs:")
        for c in order:
            if side(a[c]["class"]) != side(b[c]["class"]):
                print(f"  {c}\tA {a[c]['class']} ({a[c]['conf']}, else {a[c]['else']})\tB {b[c]['class']} ({b[c]['conf']}, else {b[c]['else']})")
        print("\nsame side, class differs:")
        for c in order:
            if side(a[c]["class"]) == side(b[c]["class"]) and a[c]["class"] != b[c]["class"]:
                print(f"  {c}\tA {a[c]['class']}\tB {b[c]['class']}")
    else:
        deep = deep_set(a, b)
        for name, ids, n in batches(deep) + second(deep):
            print(f"{name}\t{len(ids)}\t{n}\t{' '.join(ids)}")


if __name__ == "__main__":
    main(sys.argv[1:])
