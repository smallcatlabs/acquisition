"""What the trade site does to a query on the way back, which row of the
sheet a search is, and what a capture of one holds, read in one place.

    tools/trade_query.py --self-test    # captures made here, read

The site rewrites a query it answers: every `"disabled": false` is dropped,
keys are reordered, and its page posts `stats: []` as one `and` group with
no filters (round i, 2026-10-01), and a socket filter with every colour it
was not given sent `null` (u059, 2026-10-16): a key sent null is a key not
sent. A returned or posted query is the sheet's
row when the two are equal once `bare()` has read both. The splitter
(`trade-split.py`) and the extract (`trade-captures.py`) both compare
queries, and each read them its own way until 2026-10-01, when four of
round i's eight searches were refused by one tool after the other had been
taught: one reader of a query's shape, here, and no copy.

A search is its league and its query (round w, 2026-10-16): the same query
in another league is another row. The rule was taught to the composer and
to the splitter's searches, and not to the splitter's fetches or the
extract, which went on finding a row by its query alone (the outside review
of step 7): every tool that asks which row a call or a capture answers asks
`asking()`, over the rows `sheet()` reads.

A capture is one unit, read here alone (the outside review of 606b4a17):
a search's answer and its fetch pages in order, `<name>-search.json`,
`<name>-fetch.json`, `-fetch-2.json` and on, as the splitter writes them
whole. Five scripts had each read the pages their own way, and a listing
the site sent null for (a listing gone between the search and the fetch:
u025, v092, v093) was a crash in one, an empty item in another, and a page
not written by the splitter. `capture()` reads every page, counts a null
listing and skips it, and refuses a listing its search did not return, so
every reader's n-th item is the same item.

Its one caller is the extract (`trade-captures.py`): every other script
reads the two files the extract writes in one run from one read of raw/,
paired by `fetched()` and nowhere else (the outside review of 555c4f58: a
script that joined a local capture to the committed extract by an item's
place and base took a replaced capture's marks for the old item's).
"""

import csv
import json
import sys
import tempfile
from pathlib import Path

DATA = Path(__file__).resolve().parents[1] / "search" / "pseudo-stats" / "data"
SHEET = DATA / "search-sheet.csv"
EXTRACT = DATA / "captures.json"
BODIES = DATA / "bodies.json"


def bare(node):
    """A query with every `"disabled": false` and every key sent null
    removed, and every stat group with no filters removed: a group of
    nothing is no group."""
    if isinstance(node, dict):
        out = {k: bare(v) for k, v in node.items() if not (k == "disabled" and v is False) and v is not None}
        if isinstance(out.get("stats"), list):
            out["stats"] = [g for g in out["stats"] if not (isinstance(g, dict) and g.get("filters") == [])]
        return out
    if isinstance(node, list):
        return [bare(v) for v in node]
    return node


def sheet(path=SHEET):
    """The sheet's searches in its order, {name: (league, query)}."""
    with path.open(newline="") as fh:
        return {r["search"]: (r["league"], json.loads(r["query"])) for r in csv.DictReader(fh)}


def asking(league, query, rows):
    """The names of the rows that ask a query in a league, in the sheet's order."""
    want = bare(query)
    return [n for n, (lg, q) in rows.items() if lg == league and bare(q) == want]


def capture(raw, name):
    """A search's capture: (its answer, every item fetched in order, how many
    listings the site sent null for); None where the search file is saved
    empty, a search not yet captured. A fetch page saved empty holds no
    listing. Refuses a listing that is not among the search's results."""
    raw = Path(raw)
    saved = (raw / f"{name}-search.json").read_text()
    if not saved.strip():
        return None
    answer = json.loads(saved)
    returned = set(answer.get("result") or [])
    pages = [raw / f"{name}-fetch.json"] + sorted(raw.glob(f"{name}-fetch-*.json"),
                                                  key=lambda p: int(p.stem.rsplit("-", 1)[1]))
    items, unsent = [], 0
    for page in pages:
        if not page.exists() or not page.stat().st_size:
            continue
        for listing in json.loads(page.read_text())["result"]:
            if listing is None:
                unsent += 1
            elif listing["id"] not in returned:
                raise ValueError(f"{page.name}: a listing its search {name} did not return")
            else:
                items.append(listing["item"])
    return answer, items, unsent


def fetched(group="searches", extract=EXTRACT, bodies=BODIES):
    """{name: [(item, body)]} over `group` ("searches" or "census"): each
    item fetched as the extract reads it and as the site sent it, in fetch
    order — the two files `trade-captures.py` writes together, paired here.
    Refuses two files that do not hold the same searches' items."""
    items = json.loads(Path(extract).read_text())[group]
    sent = json.loads(Path(bodies).read_text())[group]
    out = {}
    for name, entry in items.items():
        listed = entry["fetched"] if group == "searches" else entry
        if len(listed) != len(sent.get(name, [])):
            raise SystemExit(f"{name}: {Path(extract).name} holds {len(listed)} items and {Path(bodies).name} "
                             f"{len(sent.get(name, []))}: written apart (tools/trade-captures.py writes both)")
        out[name] = list(zip(listed, sent.get(name, [])))
    if sent.keys() - items.keys():
        raise SystemExit(f"{Path(bodies).name} holds searches {Path(extract).name} does not: written apart")
    return out


def self_test():
    """Captures made here, read by `capture`: a page of items and nulls (a
    crash in the extract, the outside review of 606b4a17), a page of nulls
    alone, a listing its search did not return, pages read in number order,
    a search or a page saved empty; and the two extracts paired, or refused
    when written apart."""
    def listing(i):
        return {"id": i, "item": {"baseType": i}}

    cases = [
        ("a page of items and nulls: the items in order, the nulls counted",
         {"s": ["a", "b", "c"]}, {"s": [[listing("a"), None, listing("c")]]}, "s", (["a", "c"], 1)),
        ("a page of nulls alone: no item, every listing counted",
         {"s": ["a", "b"]}, {"s": [[None, None]]}, "s", ([], 2)),
        ("a listing its search did not return is refused",
         {"s": ["a"]}, {"s": [[listing("a"), listing("z")]]}, "s", "did not return"),
        ("pages in number order, the tenth after the second",
         {"s": list("abcdefghij")}, {"s": [[listing(c)] for c in "abcdefghij"]}, "s", (list("abcdefghij"), 0)),
        ("a page saved empty holds no listing",
         {"s": ["a"]}, {"s": [""]}, "s", ([], 0)),
        ("a search saved empty is not yet captured",
         {"s": ""}, {}, "s", None),
    ]
    failed = 0
    for name, searches, pages, asked, want in cases:
        with tempfile.TemporaryDirectory() as tmp:
            raw = Path(tmp)
            for n, ids in searches.items():
                (raw / f"{n}-search.json").write_text("" if ids == "" else json.dumps({"id": "x", "result": ids}))
            for n, fetches in pages.items():
                for k, page in enumerate(fetches, start=1):
                    path = raw / (f"{n}-fetch.json" if k == 1 else f"{n}-fetch-{k}.json")
                    path.write_text("" if page == "" else json.dumps({"result": page}))
            try:
                read = capture(raw, asked)
                got = None if read is None else ([i["baseType"] for i in read[1]], read[2])
            except ValueError as refusal:
                got = str(refusal)
        if not (got == want or isinstance(want, str) and isinstance(got, str) and want in got):
            failed += 1
            print(f"DIFFER  {name}: {got}")
    # The two extracts paired: by place within a search, every search of both.
    pairs = [
        ("each item beside its body, in order",
         {"s": {"fetched": [{"i": 1}, {"i": 2}]}, "z": {"fetched": []}}, {"s": [{"b": 1}, {"b": 2}]},
         {"s": [({"i": 1}, {"b": 1}), ({"i": 2}, {"b": 2})], "z": []}),
        ("a search whose items and bodies differ in number is refused",
         {"s": {"fetched": [{"i": 1}]}}, {"s": [{"b": 1}, {"b": 2}]}, "written apart"),
        ("bodies of a search the extract does not hold are refused",
         {"s": {"fetched": []}}, {"t": [{"b": 1}]}, "written apart"),
    ]
    for name, items, bodies, want in pairs:
        with tempfile.TemporaryDirectory() as tmp:
            extract, sent = Path(tmp) / "e.json", Path(tmp) / "b.json"
            extract.write_text(json.dumps({"searches": items}))
            sent.write_text(json.dumps({"searches": bodies}))
            try:
                got = fetched("searches", extract, sent)
            except SystemExit as refusal:
                got = str(refusal)
        if not (got == want or isinstance(want, str) and isinstance(got, str) and want in got):
            failed += 1
            print(f"DIFFER  {name}: {got}")
    print(f"{len(cases)} captures made here read, {len(pairs)} extracts paired, {failed} differ")
    return 1 if failed else 0


if __name__ == "__main__":
    if sys.argv[1:] != ["--self-test"]:
        print(__doc__)
        sys.exit(2)
    sys.exit(self_test())
