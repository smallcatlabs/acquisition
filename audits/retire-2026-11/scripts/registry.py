"""The registry as the census reads it: every entry, its area, its numbered clauses.

usage, from the tree's root:
  python3 -I audits/retire-2026-11/scripts/registry.py clauses C19 C20 ...   each entry's clauses, numbered, with its Why and its Pinned
  python3 -I audits/retire-2026-11/scripts/registry.py clauses all           the same for every entry, in the registry's order
  python3 -I audits/retire-2026-11/scripts/registry.py batches               the first readers' batches and the second readers' seeded fifth

Clauses are split where `tools/bothy.py` splits them (the entry rule, 6:
the text before *Why:*, code spans read as one word, parenthesised text
left out, split at `. ` or `; `), but printed with their own words, so
two readers number the same clause the same way. Clause 1 is the
headline, or its first clause where the headline has several.
Reads only; writes nothing.
"""
import importlib.util, math, os, random, re, sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", ".."))
spec = importlib.util.spec_from_file_location("bothy", os.path.join(ROOT, "tools", "bothy.py"))
bothy = importlib.util.module_from_spec(spec)
spec.loader.exec_module(bothy)
REG = bothy.Registry(ROOT)
SEED = 20261127      # the second read's seed
BATCH = 13           # the most entries one first reader takes


def mask(text):
    """The text with each code span and each parenthesis filled, at its own
    length, so a split point found here is a split point in the text."""
    fill = lambda m: "x" * len(m.group(0))
    return re.sub(r"\([^)]*\)", fill, re.sub(r"`[^`]*`", fill, text))


def clauses(entry):
    text = (entry["title"] + " " + entry["body"].partition("*Why:*")[0]).strip()
    out, at = [], 0
    for m in re.finditer(r"[.;]\s+|[.;]$", mask(text)):
        piece = text[at:m.start() + 1].strip()
        if piece.strip(".; "):
            out.append(piece)
        at = m.end()
    if text[at:].strip(".; "):
        out.append(text[at:].strip())
    return out


def areas():
    by = {}
    for cid in REG.order:
        by.setdefault(REG.decisions[cid]["area"], []).append(cid)
    return by


def first_batches():
    """Each area in file order, cut into equal runs of at most BATCH; ci
    rides with process."""
    by = areas()
    if "ci" in by:
        by["process"] = by.pop("ci") + by["process"]
    out = []
    for area, ids in by.items():
        n = math.ceil(len(ids) / BATCH)
        size = math.ceil(len(ids) / n)
        for k in range(n):
            out.append((f"{area}-{k + 1}" if n > 1 else area, ids[k * size:(k + 1) * size]))
    return out


def second_sample():
    """A fifth of the entries, by area with largest remainders, seeded."""
    by = areas()
    total = sum(len(v) for v in by.values())
    want = round(total / 5)
    quota = {a: want * len(v) / total for a, v in by.items()}
    take = {a: math.floor(q) for a, q in quota.items()}
    for a in sorted(by, key=lambda a: -(quota[a] - take[a]))[:want - sum(take.values())]:
        take[a] += 1
    rnd = random.Random(SEED)
    picked = []
    for a in sorted(by):
        picked += sorted(rnd.sample(sorted(by[a], key=lambda c: int(c[1:])), take[a]), key=lambda c: int(c[1:]))
    rnd.shuffle(picked)
    n = math.ceil(len(picked) / BATCH)
    size = math.ceil(len(picked) / n)
    return [(f"second-{k + 1}", sorted(picked[k * size:(k + 1) * size], key=lambda c: int(c[1:]))) for k in range(n)]


def main(argv):
    if argv[:1] == ["clauses"]:
        for cid in (REG.order if argv[1:] == ["all"] else argv[1:]):
            e = REG.decisions[cid]
            cl = clauses(e)
            want = len(bothy.clauses(e["title"], e["body"]))
            print(f"## {cid}  ({e['file']}:{e['line']}, area {e['area']}, {len(cl)} clauses)")
            if len(cl) != want:
                print(f"   note: bothy counts {want}")
            for i, c in enumerate(cl, 1):
                print(f"  {i}. {c}")
            print(f"  Why: {bothy.section_text(e['body'], 'Why')}")
            print(f"  Pinned: {bothy.section_text(e['body'], 'Pinned')}")
            print()
    elif argv[:1] == ["batches"]:
        for name, ids in first_batches() + second_sample():
            print(f"{name}\t{len(ids)}\t{' '.join(ids)}")
    elif argv[:1] == ["check"]:
        bad = [c for c in REG.order if len(clauses(REG.decisions[c])) != len(bothy.clauses(REG.decisions[c]["title"], REG.decisions[c]["body"]))]
        print(len(REG.order), "entries;", sum(len(clauses(REG.decisions[c])) for c in REG.order), "clauses; differing from bothy:", bad)
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main(sys.argv[1:])
