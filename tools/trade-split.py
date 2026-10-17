#!/usr/bin/env python3
"""Split one browser recording of a sitting into its captures, each named by
the search it answers.

The owner opens the sheet's links with the browser's network panel recording,
then exports the panel once as a HAR file. This script reads that file and
writes search/pseudo-stats/raw/searches/<search>-search.json and <search>-fetch.json (further
pages -fetch-2.json, -3), as saving each response by hand would have. No name
is typed: a search is the row of the sheet its league and its query ask, and a
fetch is the search's it follows (`walk`, the one reading of a recording).
Nothing here touches the network (C79; SURFACES.md: a person opens
each link).

    tools/trade-split.py <file.har>            # say what the recording holds, write nothing
    tools/trade-split.py <file.har> --write    # write the captures
    tools/trade-split.py <file.har> --write --replace   # and overwrite a capture that differs
    tools/trade-split.py <file.har> --shape    # write search/pseudo-stats/data/recording-shape.csv
    tools/trade-split.py <folder>              # the recorder's files, read as one recording
    tools/trade-split.py --self-test           # split and write recordings made here

**The recorder's folder is one recording.** `tools/sitting-recorder`, loaded
in the browser, saves each search and each fetch as it is answered, a file of
one entry each, holding what this script reads and no cookie. Given a folder,
this script reads every `.har` and `.json` file in it as one recording, in
the order the calls were made.

**A recording may hold no bodies.** The browser can drop a response's body
and keep its request, its size and its headers (the first recording, 2026-09-26:
26 searches, no body in 2,260 entries). What is left is the recording's shape:
which row each search's request asked, how many bytes answered, and whether the
page went on to fetch items — it fetches none where a search found nothing.
`--shape` writes that, scrubbed, as provisional evidence: a search answered
in a few hundred bytes that no fetch followed found nothing, by the page's
behaviour and not by the site's word.

**A HAR file can hold the session's cookie.** This script reads response
bodies and the site's rate-limit headers and nothing else, copies no request
and no other header, and says whether the file holds a cookie at all. Export
the recording sanitized where the browser offers it, keep it under raw/ (never
committed), and delete it once the captures are written.

**A capture is one unit.** A search and every fetch of it are written
together or not at all, each page as the site sent it — a fetch of listings
the site sent null for too, which the one reader of a capture counts
(`trade_query.capture`). A capture on disk that differs is kept whole, and
the run exits 1; under --replace every file of it is removed before the new
one is written, so no page of an old capture outlives it (the outside review
of 606b4a17: a capture was compared and kept file by file, and a run that
kept some exited 0).

What it refuses: a search response whose query is no row of the sheet (named,
not written); a fetch of no search in the recording, or of searches of two
rows with no item to tell them apart; a search with a fetch that recorded no
answer; a row answered twice with different results (the later one kept,
whole, only under --replace); a capture already on disk with other files.
"""

import base64
import csv
import gzip
import json
import sys
import tempfile
import urllib.parse
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from trade_query import asking, bare, sheet  # noqa: E402

TRACK = Path(__file__).resolve().parents[1] / "search" / "pseudo-stats"
RAW = TRACK / "raw" / "searches"
SHAPE = TRACK / "data" / "recording-shape.csv"

# A search response under this many bytes lists a handful of results at most
# (an id is 64 characters; a response of a hundred is near 7,000 bytes). With
# no fetch after it, it lists none: the page fetches whatever a search finds.
SMALL = 1000


def decode(search_id):
    padded = search_id + "=" * (-len(search_id) % 4)
    return json.loads(gzip.decompress(base64.urlsafe_b64decode(padded)))


def body(entry):
    content = entry["response"].get("content", {})
    text = content.get("text")
    if text is None:
        return None
    if content.get("encoding") == "base64":
        return base64.b64decode(text)
    return text.encode()


def limits(entry):
    return {
        h["name"].lower(): h["value"]
        for h in entry["response"].get("headers", [])
        if h["name"].lower().startswith("x-rate-limit") or h["name"].lower() == "retry-after"
    }


def holds_cookie(har):
    for entry in har["log"]["entries"]:
        for side in ("request", "response"):
            if entry[side].get("cookies"):
                return True
            for h in entry[side].get("headers", []):
                if h["name"].lower() in ("cookie", "set-cookie") and h["value"]:
                    return True
    return False


def league_of(path):
    """The league a search's own URL names: `/api/trade/search/<league>`."""
    return urllib.parse.unquote(path.rsplit("/", 1)[1])


def walk(har, rows):
    """Every trade call in the order made: the one reading of a recording,
    which the captures and the shape are both views of.

    A search is the row its league and its query ask (`trade_query.asking`).
    A fetch names its search's id and no league, so it is its search's: the
    latest search before it that asks the query its id decodes to, among
    those whose results hold its items where it has any. Asked of the sheet
    by its query alone, a fetch of round w's Allflame charts was u045's,
    Standard's (the outside review of step 7). A fetch with no item to tell
    them apart, after searches of two rows asking its query — one query in
    two leagues, a recording with no bodies — is ambiguous and nobody's."""
    calls = []
    taken = set()
    for entry in har["log"]["entries"]:
        url = urllib.parse.urlparse(entry["request"]["url"])
        search = "/api/trade/search/" in url.path
        if not search and "/api/trade/fetch/" not in url.path:
            continue
        status = entry["response"]["status"]
        raw = body(entry)
        answer = json.loads(raw) if status == 200 and raw is not None else None
        call = {"at": entry.get("startedDateTime"), "kind": "search" if search else "fetch",
                "url": entry["request"]["url"].split("?")[0], "status": status,
                "bytes": raw if answer is not None else None,
                "size": entry["response"].get("content", {}).get("size"), "body": raw is not None,
                "limits": limits(entry), "row": None, "why": None}
        try:
            if search:
                posted = (entry["request"].get("postData") or {}).get("text")
                call["query"] = decode(answer["id"]) if answer else json.loads(posted)["query"]
            else:
                call["query"] = decode(urllib.parse.parse_qs(url.query)["query"][0])
        except (ValueError, KeyError, TypeError, OSError):
            call["query"] = None
        if search:
            names = [] if call["query"] is None else asking(league_of(url.path), call["query"], rows)
            # One query asked by several rows (round k re-asked four of round j's,
            # 2026-10-04): the search is named for the row whose capture on disk is
            # this answer, else the first row that has none yet, on disk or in this
            # recording, so every sitting's answer is kept under the row that asked
            # it that day and a recording splits the same the second time; a query
            # no row asks is refused.
            mine = [n for n in names if n not in taken and raw is not None
                    and (RAW / f"{n}-search.json").exists() and (RAW / f"{n}-search.json").read_bytes() == raw]
            free = [n for n in names if n not in taken and not (RAW / f"{n}-search.json").exists()]
            call.update(row=(mine or free or names or [None])[0], ids=set(answer.get("result", [])) if answer else set(),
                        total=answer.get("total") if answer else None, fetches=[])
            taken.add(call["row"])
        else:
            call["asked"] = len(url.path.rsplit("/", 1)[1].split(","))
            call["items"] = {r["id"] for r in answer.get("result", []) if r} if answer else set()
            want = None if call["query"] is None else bare(call["query"])
            mine = [c for c in calls if c["kind"] == "search" and c["query"] is not None and want is not None
                    and bare(c["query"]) == want and (not call["items"] or call["items"] <= c["ids"])]
            if not mine:
                call["why"] = "a fetch belonging to no search in the recording"
            elif not call["items"] and len({c["row"] for c in mine}) > 1:
                call["why"] = ("ambiguous: searches of " + ", ".join(sorted({c["row"] or "no row" for c in mine}))
                               + " ask its query, and it holds no item to tell them apart")
            else:
                owner = mine[-1]
                call["row"] = owner["row"]
                owner["fetches"].append(call)
        calls.append(call)
    return calls


def self_test():
    """The fetch's search, held on recordings made here (the outside review
    of step 7): one query asked in two leagues, a fetch after each, with
    bodies and without; a fetch of no search; a fetch the site sent no item
    for; a query two captured rows ask, split again. Each over a raw
    directory made here."""
    query = {"query": {"status": {"option": "any"}, "type": "Chart"}}
    token = base64.urlsafe_b64encode(gzip.compress(json.dumps(query["query"]).encode())).decode().rstrip("=")
    other = base64.urlsafe_b64encode(gzip.compress(b'{"type":"Other"}')).decode().rstrip("=")
    rows = {"self-a": ("Standard", query["query"]), "self-b": ("Allflame", query["query"]),
            "self-c": ("Standard", query["query"])}

    def search(league, ids, bodies=True):
        text = json.dumps({"id": token, "result": ids, "total": len(ids)}) if bodies else None
        return {"startedDateTime": "", "request": {"url": f"https://x/api/trade/search/{league}",
                                                   "postData": {"text": json.dumps(query)}},
                "response": {"status": 200, "content": {"size": 1, **({"text": text} if text else {})}}}

    def fetch(ids, sent=True, of=token, bodies=True):
        text = json.dumps({"result": [{"id": i} if sent else None for i in ids]}) if bodies else None
        return {"startedDateTime": "", "request": {"url": f"https://x/api/trade/fetch/{','.join(ids)}?query={of}"},
                "response": {"status": 200, "content": {"size": 1, **({"text": text} if text else {})}}}

    def rows_of(entries, on_disk=None):
        global RAW
        kept, RAW = RAW, Path(tempfile.mkdtemp())
        try:
            for f, data in (on_disk or {}).items():
                (RAW / f).write_text(data)
            return [(c["kind"], c["row"], bool(c["why"])) for c in walk({"log": {"entries": entries}}, rows)]
        finally:
            for f in RAW.iterdir():
                f.unlink()
            RAW.rmdir()
            RAW = kept

    cases = [
        ("a fetch is the search's it follows, in its league",
         [search("Standard", ["a1"]), search("Allflame", ["b1"]), fetch(["b1"])],
         [("search", "self-a", False), ("search", "self-b", False), ("fetch", "self-b", False)]),
        ("its items say which, the other league's search later",
         [search("Standard", ["a1"]), search("Allflame", ["b1"]), fetch(["a1"])],
         [("search", "self-a", False), ("search", "self-b", False), ("fetch", "self-a", False)]),
        ("no bodies, two leagues' searches of its query: ambiguous, nobody's",
         [search("Standard", [], False), search("Allflame", [], False), fetch(["x"], bodies=False)],
         [("search", "self-a", False), ("search", "self-b", False), ("fetch", None, True)]),
        ("no bodies, one search of its query: that search's",
         [search("Allflame", [], False), fetch(["x"], bodies=False)],
         [("search", "self-b", False), ("fetch", "self-b", False)]),
        ("a fetch of no search in the recording",
         [search("Allflame", ["b1"]), fetch(["b1"], of=other)],
         [("search", "self-b", False), ("fetch", None, True)]),
        ("a fetch the site sent no item for: its search's",
         [search("Allflame", ["b1"]), fetch(["b1"], sent=False)],
         [("search", "self-b", False), ("fetch", "self-b", False)]),
        ("a query two rows ask, both captured: the row whose capture is this answer, split again",
         [search("Standard", ["c1"])],
         [("search", "self-c", False)],
         {"self-a-search.json": json.dumps({"id": token, "result": ["a1"], "total": 1}),
          "self-c-search.json": json.dumps({"id": token, "result": ["c1"], "total": 1})}),
    ]
    failed = 0
    for name, entries, want, *on_disk in cases:
        got = rows_of(entries, *on_disk)
        if got != want:
            failed += 1
            print(f"DIFFER  {name}: {got}")

    # Writing a capture whole (the outside review of 606b4a17): (case, on
    # disk before, the units, replace, on disk after, captures written, rows kept)
    a = {"r-search.json": b"A", "r-fetch.json": b"a1", "r-fetch-2.json": b"a2"}
    b = {"r-search.json": b"B", "r-fetch.json": b"b1"}
    writes = [
        ("the same capture is left, nothing written", a, {"r": a}, False, a, 0, []),
        ("a capture that differs is kept whole, a refusal", a, {"r": b}, False, a, 0, ["r"]),
        ("under --replace no page of the old capture outlives it", a, {"r": b}, True, b, 1, []),
        ("a search saved empty is no capture yet", {"r-search.json": b""}, {"r": b}, False, b, 1, []),
        ("a page of listings sent null is a page of its capture", {}, {"r": {"r-search.json": b"A",
         "r-fetch.json": b'{"result":[null]}'}}, False, {"r-search.json": b"A", "r-fetch.json": b'{"result":[null]}'}, 1, []),
    ]
    for name, before, units, replace, after, want_written, want_kept in writes:
        with tempfile.TemporaryDirectory() as tmp:
            raw = Path(tmp)
            for f, data in before.items():
                (raw / f).write_bytes(data)
            got = write(units, raw, replace)
            disk = {p.name: p.read_bytes() for p in raw.iterdir()}
        if got != (want_written, want_kept) or disk != after:
            failed += 1
            print(f"DIFFER  {name}: wrote {got}, on disk {sorted(disk)}")

    # Composing a capture whole: every fetch is a page, a fetch with no answer
    # leaves the search unwritten, a second answer is refused unless --replace.
    def found(row, *fetches):
        return {"row": row, "at": "", "total": 1, "bytes": b"S" + row.encode(),
                "fetches": [{"bytes": f} for f in fetches]}
    composes = [
        ("every fetch a page, a page of nulls among them",
         [found("r", b"f1", b'{"result":[null]}')], False, {"r": ["r-search.json", "r-fetch.json", "r-fetch-2.json"]}, 0),
        ("a fetch that recorded no answer leaves its search unwritten",
         [found("r", b"f1", None)], False, {}, 1),
        ("a row answered twice with other results: the first kept, refused",
         [found("r", b"f1", b"f2"), found("r", b"g1")], False, {"r": ["r-search.json", "r-fetch.json", "r-fetch-2.json"]}, 1),
        ("under --replace the later answer whole, no page of the first",
         [found("r", b"f1", b"f2"), found("r", b"g1")], True, {"r": ["r-search.json", "r-fetch.json"]}, 0),
    ]
    for name, searches, replace, want, want_refused in composes:
        units, refusals = compose(searches, replace)
        if {r: list(u) for r, u in units.items()} != want or len(refusals) != want_refused:
            failed += 1
            print(f"DIFFER  {name}: {units}, {refusals}")
    print(f"{len(cases)} recordings made here split, {len(writes) + len(composes)} captures composed and written, "
          f"{failed} differ")
    return 1 if failed else 0


def recording(path):
    """A HAR file, or a folder of them as one recording in the order of its calls."""
    if not path.is_dir():
        return json.loads(path.read_text())
    # The browser names a saved file by its type: the recorder's arrive as .json.
    entries = [e for part in sorted(path.iterdir()) if part.suffix in (".har", ".json")
               for e in json.loads(part.read_text())["log"]["entries"]]
    return {"log": {"entries": sorted(entries, key=lambda e: e.get("startedDateTime") or "")}}


def peak(calls):
    """Per policy and window, the most the sitting used of what the site allows."""
    most = {}
    for call in calls:
        policy = call["limits"].get("x-rate-limit-policy")
        for rule in call["limits"].get("x-rate-limit-rules", "").split(","):
            allowed = call["limits"].get(f"x-rate-limit-{rule.lower()}", "")
            state = call["limits"].get(f"x-rate-limit-{rule.lower()}-state", "")
            for limit, used in zip(allowed.split(","), state.split(",")):
                if limit.count(":") != 2 or used.count(":") != 2:
                    continue
                hits, window, _ = limit.split(":")
                key = (policy, rule, int(window), int(hits))
                most[key] = max(most.get(key, 0), int(used.split(":")[0]))
    return most



def compose(found, replace=False):
    """Each row's capture as one unit, {row: {file name: bytes}}: its search
    and every fetch of it as the site sent it, in order, a fetch of listings
    the site sent null for among them — what such a page means is the
    reader's to say (`trade_query.capture`), not this script's. A search
    whose query is no row, a search with a fetch that recorded no answer (a
    part of a capture is none), and a row answered twice with other results
    are refused, the first answer kept — the later one whole under
    `replace`."""
    units, refusals = {}, []
    for s in found:
        row = s["row"]
        if row is None:
            refusals.append(f"REFUSED  {s['at']}  a search whose query is no row of the sheet (total {s['total']})")
            continue
        if any(f["bytes"] is None for f in s["fetches"]):
            refusals.append(f"REFUSED  {row}  a fetch of it recorded no answer: its capture would be part of one")
            continue
        unit = {f"{row}-search.json": s["bytes"]}
        for page, fetch in enumerate(s["fetches"], start=1):
            unit[f"{row}-fetch.json" if page == 1 else f"{row}-fetch-{page}.json"] = fetch["bytes"]
        if row in units and units[row] != unit and not replace:
            refusals.append(f"REFUSED  {row}  answered twice, with other results; the first is kept")
            continue
        units[row] = unit
    return units, refusals


def held(raw, row):
    """A row's capture on disk, {file name: bytes}: its search and every fetch page."""
    paths = [raw / f"{row}-search.json", raw / f"{row}-fetch.json", *raw.glob(f"{row}-fetch-*.json")]
    return {p.name: p.read_bytes() for p in paths if p.exists()}


def write(units, raw, replace):
    """Write each unit whole: a capture on disk with the same files is left,
    one with others is kept whole unless `replace`, which removes every file
    of it first, so no page of the old one outlives it. A search saved empty
    is a search not yet captured (`trade_query.capture`). Returns (captures
    written, rows kept)."""
    raw.mkdir(parents=True, exist_ok=True)
    written, kept = 0, []
    for row, unit in units.items():
        old = held(raw, row)
        if old == unit:
            continue
        if old.get(f"{row}-search.json") and not replace:
            kept.append(row)
            continue
        for name in old:
            (raw / name).unlink()
        for name, data in unit.items():
            (raw / name).write_bytes(data)
        written += 1
    return written, kept


def main():
    args = sys.argv[1:]
    if args == ["--self-test"]:
        return self_test()
    flags = {a for a in args if a.startswith("--")}
    files = [a for a in args if not a.startswith("--")]
    if len(files) != 1 or flags - {"--write", "--replace", "--shape"}:
        print(__doc__)
        return 2
    har = recording(Path(files[0]))
    rows = sheet()
    calls = walk(har, rows)
    found = [c for c in calls if c["kind"] == "search" and c["bytes"] is not None]
    strays = [c for c in calls if c["kind"] == "fetch" and c["why"]]
    problems = [f"{c['at']}  status {c['status']}  {c['url'][:80]}" + ("" if c["body"] else "  (no body recorded)")
                for c in calls if c["bytes"] is None]

    print("the recording holds a cookie: " + (
        "YES — keep it under raw/ and delete it once the captures are written"
        if holds_cookie(har) else "no"))
    units, refusals = compose(found, "--replace" in flags)
    refused = len(refusals)
    for line in refusals:
        print(line)
    for row, unit in units.items():
        pages = [unit[n] for n in unit if "-fetch" in n]
        empty = sum(1 for page in pages if not any(json.loads(page).get("result") or []))
        total = json.loads(unit[f"{row}-search.json"]).get("total")
        print(f"{row:<8} total {total}; {len(pages)} fetch" + ("" if len(pages) == 1 else "es")
              + (f", {empty} of them of listings the site sent no item for" if empty else "")
              + ("" if pages or not total else "  — results and no fetch recorded"))
    for stray in strays:
        print(f"REFUSED  {stray['at']}  {stray['why']}")
    bodiless = [c for c in calls if not c["body"]]
    if len(problems) > 6 and len(bodiless) == len(calls):
        print(f"PROBLEM  no body in any of the {len(calls)} trade calls recorded")
    else:
        for p in problems:
            print(f"PROBLEM  {p}")
    if bodiless:
        print("the recording's shape — what each search's request asked, and what followed:")
        for c in calls:
            if c["kind"] == "search":
                print(f"    {c['row'] or 'no row':<8} status {c['status']}  {c['size']:>6} bytes  "
                      + (f"refused" if c["status"] != 200
                         else f"{sum(f['asked'] for f in c['fetches'])} items fetched" if c["fetches"]
                         else "no fetch followed: nothing found" if (c["size"] or 0) < SMALL
                         else "results, and no fetch recorded")
                      + ("" if c["body"] else "  (no body)"))
    missing = [n for n in rows if n not in units and not (RAW / f"{n}-search.json").exists()]
    print(f"{len(found)} searches in the recording; rows of the sheet with no capture anywhere: "
          + (", ".join(missing) if missing else "none"))

    most = peak(calls)
    if most:
        print("the most the sitting used of what the site allows, per window:")
        for (policy, rule, window, hits), used in sorted(most.items()):
            print(f"    {policy}  {rule:<8} {used:>3} of {hits:>4} in {window} s")
    slowed = [c for c in calls if c["status"] == 429 or "retry-after" in c["limits"]]
    if slowed:
        print(f"THE SITE ASKED THE SITTING TO SLOW DOWN, {len(slowed)} times, first at {slowed[0]['at']}")

    if "--shape" in flags:
        with SHAPE.open("w", newline="") as out:
            table = csv.writer(out, lineterminator="\n")
            table.writerow(["recording", "at", "call", "row", "status", "response_bytes",
                            "items_asked", "body"])
            for c in calls:
                table.writerow([Path(files[0]).name, c["at"], c["kind"], c["row"] or "",
                                c["status"], c["size"], c.get("asked") or "", "yes" if c["body"] else "no"])
        print(f"{len(calls)} calls written to {SHAPE.relative_to(TRACK.parent.parent)}")

    if "--write" not in flags:
        print("nothing written (say --write)")
        return 1 if refused or strays or problems else 0
    written, kept = write(units, RAW, "--replace" in flags)
    for row in kept:
        print(f"KEPT     {row}: its capture on disk differs, and is kept whole (say --replace)")
    print(f"{written} captures written under {RAW.relative_to(TRACK.parent.parent)}")
    return 1 if refused or kept or strays or problems else 0

if __name__ == "__main__":
    sys.exit(main())
