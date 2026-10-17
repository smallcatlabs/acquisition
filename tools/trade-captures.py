#!/usr/bin/env python3
"""The owner's captures as scrubbed extracts: search/pseudo-stats/data/captures.json
and search/pseudo-stats/data/bodies.json.

Reads search/pseudo-stats/raw/searches/<search>-search.json and <search>-fetch.json, with any
further page of the same search as <search>-fetch-2.json, -3 and so on (local
only: a fetch carries seller accounts and tokens; MANIFEST.md) and writes,
per search: its league (its row's), the query as the site returned it (the
id, decoded), whether it is the sheet's query, the response's counts, and
for each fetched item only what the evidence reads — its names, rarity, properties, the site's computed
`extended` figures, and every line with its domain, hash, flags and mods.
No item id, icon, seller, price, stash position or token leaves raw/.

Beside it, bodies.json holds each fetched item as the site sent it, for the
trade oracle (search/TRADE-PLAN.md, step 6), which enters it through the
store's ingest: a stash item's own keys, every one the site sent, less the
item id, every icon and the price note, which may not leave raw/, and less
`extended` and `pseudoMods`, the site's own computations, which no stash body
carries. The extract above reads a selection and reshapes it; an item rebuilt
from that selection loses what the search reads (`descrText`, which makes a
beast a beast; `ilvl`, `identified`, `sockets`, `corrupted`), so the oracle
reads this one alone. It holds the captures' searches by name, in fetch order,
and under `census` the owner's eight searches of 2026-09-13
(search/trade-query/raw/searches, MANIFEST.md there), whose committed census
reshapes its items as the extract does. One item to a line.

    tools/trade-captures.py             # write search/pseudo-stats/data/captures.json
    tools/trade-captures.py --check     # say what is in raw/ and write nothing
    tools/trade-captures.py --manifest  # print the manifest's rows for what is in raw/

A capture is read by `trade_query.capture`, the one reader of one: a search
file saved empty is a search not yet captured and is passed over, a listing
the site sent null for is counted and skipped. Nothing is written unless
both raw directories are present (`sources.require`). A capture
whose returned query is not its own row's is left out of the extract and named
with the row it does answer: a mislabelled capture is no evidence for its name.

The site rewrites an id on the way back: `"disabled":false` is dropped,
keys are reordered, an empty stats group is posted where the sheet has none.
A returned query is the sheet's when the two are equal once `trade_query.bare`
has read both — the one reader of a query's shape, shared with the splitter.
"""

import base64
import gzip
import hashlib
import json
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from sources import CAPTURES as RAW, CENSUS as CENSUS_RAW, require  # noqa: E402
from trade_query import asking, bare, capture, sheet  # noqa: E402

TRACK = Path(__file__).resolve().parents[1] / "search" / "pseudo-stats"
OUT = TRACK / "data" / "captures.json"
BODIES = TRACK / "data" / "bodies.json"
CENSUS = ["q1", "q2", "q2b", "q3", "q3b", "q4", "q5", "q6"]
# What leaves a body: what may not leave raw/, and what the site alone computes;
# an icon at any depth (a crucible tree's nodes carry their own)
NOT_A_BODY = ("id", "icon", "note", "extended", "pseudoMods")

ARRAYS = [
    "enchantMods", "implicitMods", "fracturedMods", "explicitMods", "craftedMods",
    "scourgeMods", "crucibleMods", "utilityMods", "veiledMods", "mutatedMods",
    "pseudoMods",
]
# Value patterns, never a bare word (search/README.md, "Traps"): a beast named
# Meatwhisper tripped the bare word on 2026-10-05 and was no seller's data.
SCRUBBED = ('"whisper": "', '"hideout_token": "', '"account": {', '"icon": "', '"stash": {', '"price": {')


def decode(search_id):
    padded = search_id + "=" * (-len(search_id) % 4)
    return json.loads(gzip.decompress(base64.urlsafe_b64decode(padded)))


def line(entry):
    if not isinstance(entry, dict):
        return {"description": entry}
    return {
        "description": entry["description"],
        "domain": entry.get("domain"),
        "hash": entry.get("hash"),
        "flags": entry.get("flags"),
        "mods": [
            {k: m.get(k) for k in ("name", "tier", "level", "magnitudes")}
            for m in entry.get("mods") or []
        ],
    }


def item(raw):
    extended = raw.get("extended", {})
    return {
        "name": raw.get("name"),
        "typeLine": raw.get("typeLine"),
        "baseType": raw.get("baseType"),
        "rarity": raw.get("rarity"),
        "frameType": raw.get("frameType"),
        "properties": [
            [p.get("type"), p.get("name"), [v[0] for v in p.get("values", [])]]
            for p in raw.get("properties", [])
        ],
        "extended": {k: v for k, v in extended.items() if k not in ("hashes", "text")},
        "lines": {a: [line(e) for e in raw[a]] for a in ARRAYS if a in raw},
    }


def body(raw):
    def iconless(v):
        if isinstance(v, dict):
            return {k: iconless(x) for k, x in v.items() if k != "icon"}
        if isinstance(v, list):
            return [iconless(x) for x in v]
        return v
    return {k: iconless(v) for k, v in raw.items() if k not in NOT_A_BODY}


def bodies_text(searches_bodies, census_bodies):
    """The bodies file: one item to a line, so a diff names the item."""
    out = ['{', ' "_generated": "by tools/trade-captures.py from raw/searches (MANIFEST.md) — do not edit",']
    for key, group in (("searches", searches_bodies), ("census", census_bodies)):
        out.append(f' "{key}": {{')
        names = list(group)
        for n, name in enumerate(names):
            items = group[name]
            out.append(f'  {json.dumps(name)}: [')
            out += [f'   {json.dumps(b, ensure_ascii=False)}' + ("," if i < len(items) - 1 else "")
                    for i, b in enumerate(items)]
            out.append('  ]' + ("," if n < len(names) - 1 else ""))
        out.append(' }' + ("," if key == "searches" else ""))
    out.append('}')
    return "\n".join(out) + "\n"


def searches():
    return sorted(p.name[: -len("-search.json")] for p in RAW.glob("*-search.json"))


def manifest():
    for path in sorted(RAW.glob("*.json")):
        body = path.read_bytes()
        stamp = time.strftime("%Y-%m-%d %H:%M", time.localtime(path.stat().st_mtime))
        print(f"| `raw/searches/{path.name}` | {len(body)} | {stamp} | `{hashlib.sha256(body).hexdigest()}` |")
    return 0


def main():
    if sys.argv[1:] == ["--manifest"]:
        return manifest()
    write = sys.argv[1:] != ["--check"]
    if sys.argv[1:] not in ([], ["--check"]):
        print(__doc__)
        return 2
    require(RAW, CENSUS_RAW)
    rows = sheet()
    out = {
        "_generated": "by tools/trade-captures.py from raw/searches (MANIFEST.md) — do not edit",
        "searches": {},
    }
    bodies = {}
    for name in searches():
        read = capture(RAW, name)
        if read is None:
            print(f"{name}: not yet captured (the file is empty)")
            continue
        answer, raws, unsent = read
        returned = decode(answer["id"])
        if name in rows and bare(returned) != bare(rows[name][1]):
            # the splitter named the capture in its row's league
            others = asking(rows[name][0], returned, rows)
            print(f"{name}: LEFT OUT — the capture answers "
                  f"{'row ' + others[0] if others else 'no row of the sheet'}, not {name}")
            continue
        entry = {
            # a search is its league and its query: the league is its row's
            "league": rows[name][0] if name in rows else None,
            "returned_query": returned,
            "sheet_query": "no such row in the sheet" if name not in rows else "the same",
            "search": {k: v for k, v in answer.items() if k not in ("id", "result")},
            "ids_returned": len(answer.get("result", [])),
            "fetched": [],
        }
        entry["fetched"] = [item(r) for r in raws]
        if raws:
            bodies[name] = [body(r) for r in raws]
        out["searches"][name] = entry
        print(f"{name}: sheet query {entry['sheet_query']}; total {answer.get('total')}; "
              f"{entry['ids_returned']} ids; {len(entry['fetched'])} items fetched"
              + (f", {unsent} listings the site sent no item for" if unsent else ""))
    census = {}
    for name in CENSUS:
        _, raws, _ = capture(CENSUS_RAW, name)
        if raws:
            census[name] = [body(r) for r in raws]
    print(f"census: {sum(len(b) for b in census.values())} items over {len(census)} searches")
    texts = {
        OUT: json.dumps(out, indent=1, ensure_ascii=False) + "\n",
        BODIES: bodies_text(bodies, census),
    }
    for path, text in texts.items():
        for bad in SCRUBBED:
            if bad in text:
                raise SystemExit(f"scrub guard: {bad!r} would leave raw/ in {path.name}")
    if not write:
        print("--check: nothing written")
        return 0
    for path, text in texts.items():
        path.write_text(text)
        print(f"wrote {path.relative_to(TRACK.parent.parent)}, {len(text.encode())} bytes; scrub guard passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
