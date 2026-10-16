#!/usr/bin/env python3
"""Write the sheet of trade searches the owner runs: search/pseudo-stats/data/search-sheet.csv.

A search is a query object; the site's search id is that object, serialized
compactly, gzipped and base64url-encoded without padding (trade-query F9), so
a link is composed here and opened by a human in a browser. Nothing in this
script touches the network (C79; SURFACES.md, the trade site's rows: access
method `browser`).

    tools/trade-sheet.py                # the pilot, the checks and the rounds: what the owner is asked to run
    tools/trade-sheet.py --method if    # the pilot and the batch, one search per candidate line
    tools/trade-sheet.py --method and   # the pilot and the batch, one search per pair
    tools/trade-sheet.py --self-test    # compose each captured request and compare ids
    tools/trade-sheet.py --verify       # decode every link in the sheet, compare its query

The checks ask the site of every listed item at once whether a pseudo's rows
are complete and sound (`checks` and `round_two`, below, the second written
from tools/trade_rows.py); the pilot showed the method, and the committed sheet is
the pilot and the checks. A generated search pins the version of the rows it
was composed from and says so in what it decides. The batch is the form before it,
kept one command away and not run: 264 searches, one line at a time.

The batch is generated from search/pseudo-stats/data/candidates.csv (search/pseudo-stats/scripts/candidates.py),
in two forms, since the pilot's answer was not in: `if` — one search per
candidate line, every pseudo it might feed in one `if` group (p2 asks whether
an `if` group shows a value at all); `and` — one search per pair, the pseudo
required as in p1. Rows are ordered by what a search is worth: the pilot, then
lines whose best pair is `sources-differ`, `text-only`, `one-source`,
`sources-agree`, then the percentile's own cases (PERCENTILE_CASES), then
the first pass's open question 5 (R1).

The self-test reads `search/trade-query/raw/searches/` (local only, never
committed): for every `<q>-request.json` with a `<q>-search.json` beside it,
the id composed from the request must equal the id the site returned. It
skips, saying so, where the captures are absent.

Each search names what it decides. A search whose answer would read the same
under two hypotheses is not a search: every one carries a control — a pseudo
the returned items are known to have — so that an absent value is the site's
answer and never the method's failure.
"""

import base64
import csv
import json
import re
import struct
import sys
import zlib
from pathlib import Path

TRACK = Path(__file__).resolve().parents[1] / "search" / "pseudo-stats"
OUT = TRACK / "data" / "search-sheet.csv"
CAPTURED = TRACK.parent / "trade-query" / "raw" / "searches"

SITE = "https://www.pathofexile.com/trade/search"
LEAGUE = "Standard"

# The ten bytes every captured id opens with: gzip, deflate, no flags, no
# time, no extra flags, OS 3.
GZIP_HEADER = bytes.fromhex("1f8b0800000000000003")


def compose(query):
    """The site's id for a query object."""
    body = json.dumps(query, separators=(",", ":"), ensure_ascii=False).encode()
    deflate = zlib.compressobj(6, zlib.DEFLATED, -15)
    packed = deflate.compress(body) + deflate.flush()
    trailer = struct.pack("<II", zlib.crc32(body), len(body))
    whole = GZIP_HEADER + packed + trailer
    return base64.urlsafe_b64encode(whole).decode().rstrip("=")


def link(query, league=LEAGUE):
    return f"{SITE}/{league}/{compose(query)}"


def query(stats, rarity="rare"):
    """A query in the shape the owner's captures pin (trade-query F1)."""
    return {
        "status": {"option": "any"},
        "stats": stats,
        "filters": {
            "type_filters": {
                "disabled": False,
                "filters": {"rarity": {"option": rarity}},
            },
            "trade_filters": {
                "disabled": True,
                "filters": {"sale_type": {"option": "any"}},
            },
        },
    }


def group(kind, ids, value=None):
    out = {"type": kind, "filters": [{"id": i, "disabled": False} for i in ids]}
    if value is not None:
        out["value"] = value
    return out


# The pilot: two searches that settle the method before the batch is written.
# Both are new to the site, so either opening shows a composed link is taken.
PILOT = [
    {
        "search": "p1",
        "decides": (
            "the method: a composed link opens. The evidence: whether "
            "`+# to Strength and Intelligence` counts toward "
            "`+# total maximum Life`, and at what weight — the total less "
            "the item's life lines and half its Strength lines is the answer"
        ),
        "control": "pseudo.pseudo_total_life is required, so every item shows it",
        "query": query(
            [
                group("and", ["pseudo.pseudo_total_life"]),
                # Two ids display this one text (trade-query F4), so the line
                # goes out as a count of at least one (the contract detail, C99).
                group(
                    "count",
                    ["explicit.stat_1535626285", "explicit.stat_2543977012"],
                    {"min": 1},
                ),
            ]
        ),
    },
    {
        "search": "p2",
        "decides": (
            "the method: an `if` group shows a pseudo's value without "
            "filtering by it. The evidence: whether "
            "`#% increased Attack and Cast Speed` counts toward "
            "`+#% total Attack Speed` and `+#% total Cast Speed`"
        ),
        "control": (
            "every item carries `+#% to Fire Resistance`, so "
            "pseudo.pseudo_total_fire_resistance shown and a speed total "
            "absent is the site's answer; none of the three shown means an "
            "`if` group shows nothing"
        ),
        "query": query(
            [
                group(
                    "and",
                    ["explicit.stat_2672805335", "explicit.stat_3372524247"],
                ),
                group(
                    "if",
                    [
                        "pseudo.pseudo_total_fire_resistance",
                        "pseudo.pseudo_total_attack_speed",
                        "pseudo.pseudo_total_cast_speed",
                    ],
                ),
            ]
        ),
    },
]

CANDIDATES = TRACK / "data" / "candidates.csv"
FETCH_CENSUS = "search/trade-query/data/fetch-census.json"

# The controls: a line the site was seen counting under a pseudo, so that an
# item carrying the line shows the pseudo. Each names what it also feeds, so
# a candidate the control feeds is read by value, never by presence.
CONTROLS = {
    "strength": {
        "line": "explicit.stat_4080418644",
        "text": "+# to Strength",
        "pseudo": "pseudo.pseudo_total_life",
        "evidence": f"{FETCH_CENSUS} queries.q4.fetched[7]: +9 to Strength counted at 0.5",
        "feeds": {"pseudo.pseudo_total_life", "pseudo.pseudo_total_strength"},
    },
    "fire": {
        "line": "explicit.stat_3372524247",
        "text": "+#% to Fire Resistance",
        "pseudo": "pseudo.pseudo_total_fire_resistance",
        "evidence": f"{FETCH_CENSUS} queries.q5: counted on all ten items",
        "feeds": {"pseudo.pseudo_total_fire_resistance", "pseudo.pseudo_total_elemental_resistance",
                  "pseudo.pseudo_total_resistance"},
    },
}
PERCENTILE = "pseudo.pseudo_base_defence_percentile"
PERCENTILE_CONTROL = (
    "extended.base_defence_percentile: every armour item fetched carries it "
    f"(30 of 30, {FETCH_CENSUS}); an armour item returned without it is the site's answer"
)
CATEGORY_ORDER = ["explicit", "implicit", "enchant", "crafted", "fractured", "scourge", "crucible"]
STATUS_ORDER = ["sources-differ", "text-only", "one-source", "sources-agree"]


def batch_query(stats, filters=None, item_type=None):
    """A query in the pilot's shape without a rarity: a line can be on any."""
    out = {"status": {"option": "any"}}
    if item_type:
        out["type"] = item_type
    out["stats"] = stats
    out["filters"] = dict(filters or {})
    out["filters"]["trade_filters"] = {"disabled": True, "filters": {"sale_type": {"option": "any"}}}
    return out


def category(option):
    return {"type_filters": {"disabled": False, "filters": {"category": {"option": option}}}}


def percentile_filter(minimum=1):
    return {"armour_filters": {"disabled": False, "filters": {"base_defence_percentile": {"min": minimum}}}}


def line_filter(stat_ids):
    """The line as a filter: one id in an `and` group; a text two ids display
    goes out as a `count` of at least one (C99; p1). The ids are those of one
    category: explicit when the line has one, else the first the line has."""
    ids = [i for i in stat_ids.split(";") if i]
    for cat in CATEGORY_ORDER + sorted({i.split(".")[0] for i in ids}):
        chosen = sorted(i for i in ids if i.split(".")[0] == cat)
        if chosen:
            break
    kind, value = ("and", None) if len(chosen) == 1 else ("count", {"min": 1})
    out = group(kind, chosen, value)
    # an id the capture lists as `<id>|<option>` is one option of a stat:
    # it goes out as the stat with that option (APT pathofexile-trade.ts L101)
    for f in out["filters"]:
        base, sep, option = f["id"].partition("|")
        if sep:
            f["id"] = base
            f["value"] = {"option": int(option) if option.isdigit() else option}
    return out, chosen


def read_candidates():
    with CANDIDATES.open(newline="") as fh:
        rows = list(csv.DictReader(fh))
    lines = {}
    for r in rows:
        if not r["search"] or r["search"].startswith("none") or r["note"].startswith("family:"):
            continue
        line_id, _, k = r["search"].partition(".")
        e = lines.setdefault(line_id, {"stat_ids": r["stat_ids"], "pairs": {}, "templates": set(),
                                       "status": [], "items": 0, "families": 0})
        e["templates"].add(r["template"])
        e["status"].append(STATUS_ORDER.index(r["status"]))
        e["items"] += int(r["corpus_items"])
        if r["note"].startswith("represents"):
            e["families"] += 1
        pair = e["pairs"].setdefault(k or "1", {"pseudo": r["pseudo"], "text": r["pseudo_text"],
                                                "status": r["status"]})
        if pair["pseudo"] != r["pseudo"]:
            raise SystemExit(f"{r['search']} names two pseudos")
    return lines


def pick_control(pseudos):
    """The control that feeds the fewest of the candidates."""
    return min(CONTROLS, key=lambda c: (len(CONTROLS[c]["feeds"] & set(pseudos)), c != "strength"))


def control_text(name, read_by_value):
    c = CONTROLS[name]
    text = (f"{c['pseudo']} shown: `{c['text']}` is required, and the site counted it "
            f"({c['evidence']}); a candidate absent beside it is the site's answer")
    if read_by_value:
        text += "; the control also feeds " + ", ".join(sorted(read_by_value)) + ", read by value"
    text += ". No result decides nothing: rerun with the other control"
    return text


def line_names(e):
    return " / ".join(f"`{t}`" for t in sorted(e["templates"]))


def if_searches(lines):
    out = []
    for line_id, e in lines.items():
        if line_id.startswith("P"):
            continue
        pseudos = [e["pairs"][k]["pseudo"] for k in sorted(e["pairs"], key=int)]
        name = pick_control(pseudos)
        c = CONTROLS[name]
        lf, _ = line_filter(e["stat_ids"])
        shown = [c["pseudo"]] + [p for p in pseudos if p != c["pseudo"]]
        stats = [lf, group("and", [c["line"]]), group("if", shown)]
        fam = f" (represents {e['families']} families)" if e["families"] else ""
        out.append({
            "search": line_id,
            "decides": (f"whether {line_names(e)}{fam} counts toward each of: "
                        + "; ".join(f"`{e['pairs'][k]['text']}` ({line_id}.{k}, "
                                    f"{e['pairs'][k]['status']})" for k in sorted(e["pairs"], key=int))
                        + ". A pseudo shown, less the item's other lines it is known to count, is the "
                          "line's weight; a pseudo absent while the control shows is not fed"),
            "control": control_text(name, c["feeds"] & set(pseudos)),
            "query": batch_query(stats),
            "rank": (min(e["status"]), -e["items"], line_id),
        })
    return out


def and_searches(lines):
    out = []
    for line_id, e in lines.items():
        if line_id.startswith("P"):
            continue
        lf, _ = line_filter(e["stat_ids"])
        for k in sorted(e["pairs"], key=int):
            pair = e["pairs"][k]
            pid = pair["pseudo"]
            if pid == CONTROLS["strength"]["pseudo"]:
                stats = [group("and", [pid]), lf]
                control = f"{pid} is required, so every item shows it (p1)"
            else:
                name = pick_control([pid])
                c = CONTROLS[name]
                stats = [lf, group("and", [c["line"], c["pseudo"], pid])]
                control = control_text(name, c["feeds"] & {pid})
            out.append({
                "search": f"{line_id}.{k}",
                "decides": (f"whether {line_names(e)} counts toward `{pair['text']}` ({pair['status']}): "
                            "the pseudo's value less the item's other lines it is known to count is the "
                            "line's weight; no result, with the line common on the site, means not fed"),
                "control": control,
                "query": batch_query(stats),
                "rank": (STATUS_ORDER.index(pair["status"]), -e["items"], line_id, int(k)),
            })
    return out


def percentile_line_searches(lines):
    out = []
    for line_id, e in lines.items():
        if not line_id.startswith("P"):
            continue
        lf, _ = line_filter(e["stat_ids"])
        pair = e["pairs"]["1"]
        out.append({
            "search": line_id,
            "decides": (f"whether the site reads {line_names(e)} into `#% Base Defence Percentile` "
                        f"({pair['status']}): recover each roll as search/pseudo-stats/scripts/percentile.py does, with the "
                        "line read and without; the reading that gives the site's value is the answer. "
                        "No result: the line is not seen on armour with a percentile above 0"),
            "control": PERCENTILE_CONTROL,
            "query": batch_query([lf], {**category("armour"), **percentile_filter()}),
            "rank": (STATUS_ORDER.index(pair["status"]), -e["items"], line_id),
        })
    return out


# What no capture reaches (README, the percentile): each case is fetched and
# scored by search/pseudo-stats/scripts/percentile.py's reading.
PERCENTILE_CASES = [
    ("C1", "three rolls averaged: a base with armour, evasion and energy shield",
     batch_query([], percentile_filter(), "Sacrificial Garb")),
    ("C2", "ward read like the other defences: a ward base",
     batch_query([], percentile_filter(), "Runic Crown")),
    ("C3", "a shield's defences (the base also carries block, no roll)",
     batch_query([], {**category("armour.shield"), **percentile_filter()})),
    ("C4", "a helmet (no helmet captured)",
     batch_query([], {**category("armour.helmet"), **percentile_filter()})),
    ("C5", "gloves (none captured)",
     batch_query([], {**category("armour.gloves"), **percentile_filter()})),
    ("C6", "quality above 20: the display's multiplier past the site's 20% figure",
     batch_query([], {**category("armour"), **percentile_filter(),
                      "misc_filters": {"disabled": False, "filters": {"quality": {"min": 21}}}})),
    ("C7", "`Quality does not increase Defences` (the one enchant that zeroes the quality term)",
     batch_query([group("and", ["enchant.stat_2677401098"])], {**category("armour"), **percentile_filter()})),
    ("C8", "a unique's base (APT reads a unique's own base, Parser.ts L1267)",
     batch_query([], {"type_filters": {"disabled": False, "filters": {
         "category": {"option": "armour.chest"}, "rarity": {"option": "unique"}}},
         **percentile_filter()})),
]


# First-pass open question 5: does `# total Resistances` count lines or
# resistance types? One all-elemental line decides it: 1 or 3.
OPEN_QUESTION_SEARCHES = [
    {
        "search": "R1",
        "decides": (
            "first-pass open question 5: whether `# total Resistances` and `# total Elemental "
            "Resistances` count lines or resistances — on an item whose one resistance line is "
            "`+#% to all Elemental Resistances`, 1 is lines and 3 is resistances"
        ),
        "control": (
            "pseudo.pseudo_total_fire_resistance shown: the all-elemental line is required, and the "
            f"site counted it toward fire ({FETCH_CENSUS} queries.q5.fetched[0]); a count absent "
            "beside it is the site's answer. Read only items with no other resistance line"
        ),
        "query": batch_query([
            group("and", ["explicit.stat_2901986750"]),
            group("if", ["pseudo.pseudo_total_fire_resistance", "pseudo.pseudo_count_resistances",
                         "pseudo.pseudo_count_elemental_resistances"]),
        ]),
    },
]


def percentile_case_searches():
    return [{"search": s, "decides": f"the percentile's formula on {what}", "control": PERCENTILE_CONTROL,
             "query": q} for s, what, q in PERCENTILE_CASES]


def batch(method):
    lines = read_candidates()
    main_rows = if_searches(lines) if method == "if" else and_searches(lines)
    ranked = sorted(main_rows + percentile_line_searches(lines), key=lambda r: r["rank"])
    return ranked + percentile_case_searches() + OPEN_QUESTION_SEARCHES


STATS = TRACK.parent / "trade-query" / "data" / "stats-2026-09-12.json"


def ids_of(*texts):
    """Every id, in every category, that displays one of the texts."""
    wanted = set(texts)
    found = {t: [] for t in texts}
    for g in json.loads(STATS.read_text())["result"]:
        if g["id"] == "pseudo":
            continue
        for e in g["entries"]:
            if e["text"] in wanted:
                found[e["text"]].append(e["id"])
    for t, ids in found.items():
        if not ids:
            raise SystemExit(f"no stat displays {t!r}")
    return [i for t in texts for i in found[t]]


# The site as the checker. A pseudo's rows are complete when no listed item
# shows the pseudo while carrying none of them, and sound when no listed item
# carries one while showing no pseudo: each is one search over everything
# listed, where the batch above asks one line at a time of ten items. A
# search that finds nothing proves nothing until the same search, with one
# row taken out, is seen to find something: every check has its mutant.
LIFE = "pseudo.pseudo_total_life"
LIFE_ROWS = ["+# to maximum Life", "+# to Strength", "+# to Strength and Dexterity",
             "+# to Strength and Intelligence", "+# to all Attributes"]
SPEED_ROWS = ["#% increased Attack Speed", "#% increased Attack Speed (Local)"]


def checks():
    return [
        {
            "search": "c1",
            "decides": (
                "complete: whether any listed item shows `+# total maximum Life` while "
                "carrying none of the five lines p1 and q4 evidenced. None found: the rows "
                "are all the site counts. Any found: its lines name what is missing"
            ),
            "control": "c2, this search with one row taken out, must find items",
            "query": batch_query([group("and", [LIFE]), group("not", ids_of(*LIFE_ROWS))]),
        },
        {
            "search": "c2",
            "decides": (
                "c1's mutant: the same search with `+# to all Attributes` left out of the "
                "`not` group. It must find items, each carrying that line and no other row, "
                "each showing half of it"
            ),
            "control": "the mutant is the control: nothing found means a `not` group of this size decides nothing",
            "query": batch_query([group("and", [LIFE]),
                                  group("not", ids_of(*[t for t in LIFE_ROWS if t != "+# to all Attributes"]))]),
        },
        {
            "search": "c3",
            "decides": (
                "sound: whether any listed item carries one of the five lines while showing "
                "no `+# total maximum Life` — the pseudo inside a `not` group. None found: "
                "every row is counted wherever it appears"
            ),
            "control": "c4, this search with an uncounted line among the rows, must find items",
            "query": batch_query([group("count", ids_of(*LIFE_ROWS), {"min": 1}),
                                  group("not", [LIFE])]),
        },
        {
            "search": "c4",
            "decides": (
                "c3's mutant: `#% increased maximum Life`, which p1 showed uncounted, added "
                "to the rows. It must find items carrying that line and no counted one"
            ),
            "control": "the mutant is the control: nothing found means a pseudo inside a `not` group decides nothing",
            "query": batch_query([group("count", ids_of(*LIFE_ROWS, "#% increased maximum Life"), {"min": 1}),
                                  group("not", [LIFE])]),
        },
        {
            "search": "c5",
            "decides": (
                "the method: every member of an `if` group displays, not the first alone "
                "(p2's control was its group's first). Every item carries a cold and a fire "
                "resistance line, so both totals must show, the second as well as the first"
            ),
            "control": "`+#% to Cold Resistance` and `+#% to Fire Resistance` are both required; the site counted fire (q5, p2)",
            "query": batch_query([
                group("and", ["explicit.stat_4220027924", "explicit.stat_3372524247"]),
                group("if", ["pseudo.pseudo_total_cold_resistance",
                             "pseudo.pseudo_total_fire_resistance"]),
            ]),
        },
        {
            "search": "c6",
            "decides": (
                "complete, for a total the build ships: whether any listed item shows "
                "`+#% total Attack Speed` while carrying no `#% increased Attack Speed` "
                "line, local or not — the shipped total's one row. What is found names "
                "what else the site counts, the combined speed line of p2 among the candidates"
            ),
            "control": "c1 and c2 show what a `not` group over a line's every id finds",
            "query": batch_query([group("and", ["pseudo.pseudo_total_attack_speed"]),
                                  group("not", ids_of(*SPEED_ROWS))]),
        },
    ]


sys.path.insert(0, str(Path(__file__).resolve().parent))
import trade_rows as table  # noqa: E402


def weighted(version, ids, extra=(), at_least=0.5):
    """A version's rows as a `weight2` group: each id at its row's weight, the
    sum at least the smallest a row can give. Where a `count` of the rows finds
    every item whose lines cancel (c3: 3,338 found, most of them a sum of
    nothing), the site's own sum leaves those out."""
    filters = []
    for (t, which), w in version["rows"].items():
        for i in ids[t]:
            if (i not in version["never"] and table.means(which, i)) or i in extra:
                filters.append({"id": i, "value": {"weight": float(w)}, "disabled": False})
    return {"type": "weight2", "filters": filters, "value": {"min": at_least}}


def complete(search, name, number, version, ids, texts):
    text = texts[version["pseudo"]]
    counted = table.ids_of(version, ids)
    stats = [group("and", [version["pseudo"]])]
    if counted:
        stats.append(group("not", counted))
        decides = (f"{name} v{number}, complete: whether any listed item shows `{text}` while carrying none of "
                   f"its {len(version['rows'])} rows ({len(counted)} ids). None found: the rows "
                   "are all the site counts. Any found: its lines name what is missing")
    else:
        decides = (f"{name} v{number}, what carries `{text}`: no line displays the pseudo's text, so the items "
                   "found name its first rows")
    return {
        "search": search,
        "decides": decides,
        "control": "c1 and c2: a `not` group over every id of a row finds what it should",
        "query": batch_query(stats),
    }


def round_two():
    """d01–d24. Each pins the version of the rows it was composed from, so a
    later version changes no search here: a capture answers the row it names."""
    ids, texts = table.stats()
    versions = table.versions()
    life = versions["total_life"][1]
    speed = versions["total_attack_speed"][1]
    out = [
        {
            "search": "d01",
            "decides": (
                "total_life v2, sound, by the site's own sum: whether any listed item's rows sum to a half "
                "or more while it shows no `+# total maximum Life` — the rows weighted in a "
                "`weight2` group, the twin id c3 found left out. None found: c3's two kinds "
                "were all there is"
            ),
            "control": "d02, this search with the twin id among the rows, must find items",
            "query": batch_query([weighted(life, ids), group("not", [life["pseudo"]])]),
        },
        {
            "search": "d02",
            "decides": (
                "d01's mutant: the twin id `explicit.stat_2543977012` weighted with the rows. "
                "It must find That Which Was Taken, which c3 showed carrying the line and no total"
            ),
            "control": "the mutant is the control: nothing found means a `weight2` group beside a `not` decides nothing",
            "query": batch_query([weighted(life, ids, extra=life["never"]),
                                  group("not", [life["pseudo"]])]),
        },
        complete("d03", "total_attack_speed", 2, speed, ids, texts),
    ]
    order = [f"adds_{k}{scope}" for k in ("fire", "elemental", "damage", "cold", "lightning",
                                          "physical", "chaos")
             for scope in table.SCOPES]
    for number, name in enumerate(order, start=4):
        out.append(complete(f"d{number:02d}", name, 1, versions[name][0], ids, texts))
    return out


def sound(search, name, number, version, ids, texts, at_least=0.5):
    """Whether any listed item carries a row while showing no pseudo. A total
    whose lines carry a sign is asked by the site's own sum, which leaves out
    most items whose lines cancel (d01 found one where c3's count found 3,338);
    a ranged pseudo, whose lines have none, by a count of its rows."""
    text = texts[version["pseudo"]]
    if version["reads"] == "avg":
        rows = group("count", table.ids_of(version, ids), {"min": 1})
        how = "a count of its rows"
    else:
        rows = weighted(version, ids, at_least=at_least)
        how = "the site's own sum of its rows, " + ("a half" if at_least == 0.5 else str(at_least)) + " or more"
    return {
        "search": search,
        "decides": (f"{name} v{number}, sound: whether any listed item carries a row — {how} "
                    f"— while showing no `{text}`. None found: every row is counted wherever "
                    "it appears. Any found: a row, a twin or an id the site does not count"),
        "control": "c4 and d02: a pseudo inside a `not` group, beside a count or a sum, finds what it should",
        "query": batch_query([rows, group("not", [version["pseudo"]])]),
    }


def round_three():
    """e001 on: every pseudo in scope at its latest rows when this round was
    written, complete and sound, less what a capture has already closed —
    total life (c1, d01) and the five plain ranged pseudos' completeness
    (d04, d13, d16, d19, d22). Each pins its version."""
    ids, texts = table.stats()
    versions = table.versions()
    pinned = {name: 2 for name in versions if name.startswith("adds_")}
    pinned.update({f"adds_{kind}": 1 for kind in table.TYPES})
    pinned["total_attack_speed"] = 3
    order = [f"adds_{k}{scope}" for k in ("fire", "cold", "lightning", "physical", "chaos",
                                          "elemental", "damage")
             for scope in table.SCOPES]
    order += ["total_attack_speed"]
    order += [n for n in versions if n not in order and n != "total_life" and n not in table.OTHER]
    out = []
    for name in order:
        number = pinned.get(name, 1)
        version = versions[name][number - 1]
        closed = name in ("adds_fire", "adds_cold", "adds_lightning", "adds_physical", "adds_chaos")
        if not closed:
            out.append(complete(f"e{len(out) + 1:03d}", name, number, version, ids, texts))
        out.append(sound(f"e{len(out) + 1:03d}", name, number, version, ids, texts))
    return out


# The most ids a `weight2` group has been seen taken with: the site answers a
# query's cost as `complexity`, 56 and 4 an id for a weighted group beside a
# `not` (96 at 10 ids, 128 at 18, 148 at 23), and refused 41 and 45 ids as too
# complex (e047, e051). A count or a `not` of 71 ids was taken.
WEIGHTED_AT_MOST = 20

# The pseudos whose rows moved after round three was written, each at the
# version round four asks of: written out, so that a later version changes no
# search here.
ROUND_FOUR = {
    "total_cold_res": 3,
    "total_fire_res": 3,
    "total_lightning_res": 3,
    "total_ele_res": 3,
    "total_chaos_res": 3,
    "total_res": 3,
    "total_str": 2,
    "total_int": 2,
    "total_cast_speed": 2,
    "total_increased_phys": 3,
    "total_spell_crit": 2,
    "total_skill_gem_levels": 2,
    "adds_physical": 2,
    "adds_lightning": 2,
    "adds_cold": 2,
    "adds_fire": 2,
    "adds_chaos": 2,
    "adds_elemental": 3,
    "adds_damage": 3,
    "adds_physical_to_attacks": 3,
    "adds_lightning_to_attacks": 3,
    "adds_cold_to_attacks": 3,
    "adds_fire_to_attacks": 3,
    "adds_chaos_to_attacks": 3,
    "adds_elemental_to_attacks": 3,
    "adds_damage_to_attacks": 3,
    "adds_physical_to_spells": 3,
    "adds_lightning_to_spells": 3,
    "adds_cold_to_spells": 3,
    "adds_fire_to_spells": 3,
    "adds_chaos_to_spells": 3,
    "adds_elemental_to_spells": 3,
    "adds_damage_to_spells": 3,
}


def round_four():
    """f001 on: the pseudos of ROUND_FOUR complete and sound at their pinned
    versions; a total whose weighted rows are more than the site takes in one
    search is asked in parts, each part sound alone."""
    ids, texts = table.stats()
    versions = table.versions()
    out = []
    for name, number in ROUND_FOUR.items():
        version = versions[name][number - 1]
        out.append(complete(f"f{len(out) + 1:03d}", name, number, version, ids, texts))
        whole = sound("", name, number, version, ids, texts)
        group_ = whole["query"]["stats"][0]
        if group_["type"] != "weight2" or len(group_["filters"]) <= WEIGHTED_AT_MOST:
            whole["search"] = f"f{len(out) + 1:03d}"
            out.append(whole)
            continue
        parts = [group_["filters"][i:i + WEIGHTED_AT_MOST]
                 for i in range(0, len(group_["filters"]), WEIGHTED_AT_MOST)]
        for index, part in enumerate(parts, start=1):
            out.append({
                "search": f"f{len(out) + 1:03d}",
                "decides": whole["decides"].replace(
                    ", sound:", f", sound, part {index} of {len(parts)} of its ids:"),
                "control": whole["control"],
                "query": batch_query([dict(group_, filters=part), group("not", [version["pseudo"]])]),
            })
    return out


# The plan's step 9c2, the site's other totals: each at the version round
# five asks of, written out. A pseudo with a row whose eldritch forms the site
# lists is asked at the version that holds them, the rule being ruled.
ROUND_FIVE = {
    "count_res": 1,
    "count_ele_res": 1,
    "total_all_ele_res": 1,
    "total_all_attributes": 1,
    "total_mana": 1,
    "total_energy_shield": 1,
    "total_increased_energy_shield": 1,
    "increased_movement_speed": 2,
    "global_crit_chance": 1,
    "global_crit_multi": 1,
    "increased_ele_damage": 1,
    "increased_lightning_damage": 2,
    "increased_cold_damage": 2,
    "increased_fire_damage": 2,
    "increased_spell_damage": 2,
    "increased_lightning_spell_damage": 1,
    "increased_cold_spell_damage": 1,
    "increased_fire_spell_damage": 1,
    "increased_lightning_attack_damage": 1,
    "increased_cold_attack_damage": 1,
    "increased_fire_attack_damage": 1,
    "increased_ele_attack_damage": 1,
    "increased_rarity": 1,
    "increased_burning_damage": 1,
    "life_regen": 1,
    "life_regen_pct": 1,
    "phys_attack_life_leech": 1,
    "phys_attack_mana_leech": 1,
    "increased_mana_regen": 2,
}
# The smallest a row can give where it is under a half: a leech line is
# displayed to a hundredth of a percent.
SMALLEST = {"phys_attack_life_leech": 0.01, "phys_attack_mana_leech": 0.01}


def round_five():
    """g001 on: the pseudos of ROUND_FIVE at their pinned versions — complete
    and sound where a version has rows, and where it has none the one search
    of what carries the pseudo, whose items name its first rows."""
    ids, texts = table.stats()
    versions = table.versions()
    if set(ROUND_FIVE) != set(table.OTHER):
        raise SystemExit("round five pins what tools/trade_rows.py does not name, or leaves one out")
    out = []
    for name, number in ROUND_FIVE.items():
        version = versions[name][number - 1]
        out.append(complete(f"g{len(out) + 1:03d}", name, number, version, ids, texts))
        if not version["rows"]:
            continue
        check = sound(f"g{len(out) + 1:03d}", name, number, version, ids, texts,
                      at_least=SMALLEST.get(name, 0.5))
        if len(check["query"]["stats"][0]["filters"]) > WEIGHTED_AT_MOST:
            raise SystemExit(f"{name}: more weighted ids than the site takes; ask it in parts")
        out.append(check)
    return out


# Round six: the pseudos whose rows round five's captures moved, each at the
# version asked, written out. Rows read off ten fetched items are a fit; these
# searches are its test. `increased_rarity` gained a row under an id it already
# asked, so its searches would be g033 and g034 again and are not written.
ROUND_SIX = {
    "total_all_attributes": 2,
    "total_mana": 2,
    "total_energy_shield": 2,
    "total_increased_energy_shield": 3,
    "global_crit_chance": 2,
    "global_crit_multi": 2,
    "increased_lightning_damage": 3,
    "increased_cold_damage": 3,
    "increased_fire_damage": 3,
    "increased_lightning_spell_damage": 3,
    "increased_cold_spell_damage": 3,
    "increased_fire_spell_damage": 3,
    "increased_lightning_attack_damage": 3,
    "increased_cold_attack_damage": 3,
    "increased_fire_attack_damage": 3,
    "increased_ele_attack_damage": 2,
    "increased_burning_damage": 3,
    "life_regen": 2,
    "life_regen_pct": 2,
}
SMALLEST.update({"life_regen": 0.1, "life_regen_pct": 0.01})
# The most ids a weighted group was taken with beside a `not`: d01, at a
# complexity of 148.
WEIGHTED_TAKEN = 23
# A row's `reduced` spelling is under the row's id, a number below nothing
# (g007, g034): asked of three totals the build ships and three of round
# five's, each at the version named.
BELOW_NOTHING = {
    "total_attack_speed": 3,
    "total_cast_speed": 2,
    "total_increased_phys": 4,
    "increased_movement_speed": 2,
    "increased_mana_regen": 2,
    "global_crit_chance": 2,
}


def round_six():
    """h001 on: round five's fits tested, complete and sound; the three
    readings that are no sum of lines asked where round five left them on one
    item or none; the mutant round five's new bound owes; and a row's
    `reduced` spelling, asked below nothing."""
    ids, texts = table.stats()
    versions = table.versions()
    out = []

    def name_next():
        return f"h{len(out) + 1:03d}"

    for name, number in ROUND_SIX.items():
        version = versions[name][number - 1]
        out.append(complete(name_next(), name, number, version, ids, texts))
        check = sound(name_next(), name, number, version, ids, texts,
                      at_least=SMALLEST.get(name, 0.5))
        if len(check["query"]["stats"][0]["filters"]) > WEIGHTED_TAKEN:
            raise SystemExit(f"{name}: more weighted ids than the site has taken; ask it in parts")
        out.append(check)

    all_ele = "pseudo.pseudo_total_all_elemental_resistances"
    line = ids_of("+#% to all Elemental Resistances") + [
        i for prefix in table.PRESENCE for i in ids.get(prefix + "#% to all Elemental Resistances", [])]
    out.append({
        "search": name_next(),
        "decides": (f"total_all_ele_res, the least of the three elemental totals: items showing "
                    f"`{texts[all_ele]}` while carrying no `+#% to all Elemental Resistances` — g003 "
                    "fetched one. Each shows its least elemental total, or the reading is wrong"),
        "control": "g003: the pseudo alone finds items; c1 and c2: a `not` group finds what it should",
        "query": batch_query([group("and", [all_ele]), group("not", line)]),
    })
    count = "pseudo.pseudo_count_resistances"
    out.append({
        "search": name_next(),
        "decides": (f"count_res, whether `{texts[count]}` counts chaos: items carrying "
                    "`+#% to Chaos Resistance`, the count required and the elemental count and the "
                    "chaos total shown beside it. No item round five fetched carries the line"),
        "control": ("the chaos total is shown in the `if` group: an item showing it and a count "
                    "equal to its elemental count is the site leaving chaos out"),
        "query": batch_query([
            group("and", [count]),
            group("count", ids_of("+#% to Chaos Resistance"), {"min": 1}),
            group("if", ["pseudo.pseudo_count_elemental_resistances",
                         "pseudo.pseudo_total_chaos_resistance"]),
        ]),
    })

    life = versions["phys_attack_life_leech"][0]
    mana = versions["phys_attack_mana_leech"][0]
    both = weighted(life, ids, at_least=SMALLEST["phys_attack_life_leech"])
    both["filters"] += [{"id": i, "value": {"weight": 1.0}, "disabled": False}
                        for i in table.ids_of(mana, ids)]
    out.append({
        "search": name_next(),
        "decides": ("g040's mutant: the mana leech's ids weighted with the life leech's rows, at "
                    "a hundredth or more. It must find items carrying "
                    f"`{texts[mana['pseudo']]}` and no life leech, each showing no "
                    f"`{texts[life['pseudo']]}`"),
        "control": ("the mutant is the control: nothing found means a sum of a hundredth or more "
                    "beside a `not` decides nothing, and g040 and g042 with it"),
        "query": batch_query([both, group("not", [life["pseudo"]])]),
    })

    for name, number in BELOW_NOTHING.items():
        version = versions[name][number - 1]
        text = texts[version["pseudo"]]
        below = dict(weighted(version, ids), value={"max": -0.5})
        shows = name_next()
        out.append({
            "search": shows,
            "decides": (f"{name} v{number}, a `reduced` line: items whose rows sum to a half below "
                        f"nothing or less, by the site's own sum, `{text}` required. Each shows a "
                        "total below nothing, or the site does not count the line so"),
            "control": "g007: Carnage Heart shows -25 over `25% reduced maximum Energy Shield`",
            "query": batch_query([group("and", [version["pseudo"]]), below]),
        })
        out.append({
            "search": name_next(),
            "decides": (f"{name} v{number}, sound below nothing: whether any listed item's rows sum "
                        f"to a half below nothing or less while it shows no `{text}`. None found: "
                        "a `reduced` line is counted wherever it appears"),
            "control": f"{shows}, the same sum with the pseudo required, must find items",
            "query": batch_query([below, group("not", [version["pseudo"]])]),
        })
    return out


# Round i (before step 9d): the percentile's shapes no capture reaches, and
# the ranged family's one weapon question. PERCENTILE_CASES was read against
# the sittings' captures, 543 distinct armour items scored by
# search/pseudo-stats/scripts/percentile.py: helmets, gloves, shields, uniques,
# quality above 20 and a defence the base lacks are answered there and not
# asked again (search/pseudo-stats/data/percentile-shapes.csv, which also names
# the cases C9 on). What each search here decides is read by percentile.py.
# The plain ranged pseudos at the version asked, written out.
ROUND_I_PLAIN = {
    "adds_physical": 2,
    "adds_lightning": 2,
    "adds_cold": 2,
    "adds_fire": 2,
    "adds_chaos": 2,
}
# The texts a local defence line shares with a global stat: the site lists
# each under the global stat without `(Local)` (C14 of percentile-shapes.py).
DEFENCE_TWINS = ["+# to Armour", "+# to Evasion Rating", "+# to maximum Energy Shield",
                 "#% increased Armour", "#% increased Evasion Rating"]


def round_i():
    """i01–i08: C1, C7, C10 and its mutant, C14, the ranged family on weapons
    (sound, its mutant and its control), then C2."""
    ids, texts = table.stats()
    versions = table.versions()
    quality = {"misc_filters": {"disabled": False, "filters": {"quality": {"min": 1}}}}
    armour = category("armour")
    plain = [versions[name][number - 1] for name, number in ROUND_I_PLAIN.items()]
    pseudos = [v["pseudo"] for v in plain]
    global_ids = [i for v in plain for i in table.ids_of(v, ids)]
    local_ids = [i for name in ROUND_I_PLAIN for i in ids[f"Adds # to # {name[5:].capitalize()} Damage"]
                 if table.means("local", i)]
    pinned = ", ".join(f"{name} v{number}" for name, number in ROUND_I_PLAIN.items())
    rows = [
        ("i01",
         "C1, the percentile over three defences: Sacrificial Garb, a base with armour, evasion and energy "
         "shield (the copy's three-defence items are this base). The site's value against the average of the "
         "three recovered rolls, and against min, max and the first type, by percentile.py; none captured yet",
         PERCENTILE_CONTROL,
         batch_query([], percentile_filter(), "Sacrificial Garb")),
        ("i02",
         "C7, `Quality does not increase Defences`, on armour with quality: whether the site recovers the roll "
         "with the quality term dropped, for its percentile and for its 20%-quality figures (`extended.ar|ev|es`); "
         "percentile.py reads the enchant as the quality term's zero. None captured yet; the copy holds two",
         PERCENTILE_CONTROL,
         batch_query([group("and", ["enchant.stat_2677401098"])], {**armour, **percentile_filter(), **quality})),
        ("i03",
         "C10, a roll over the base's maximum: whether the site ever shows a percentile over 100 (the contract "
         "detail under C101 reads such an item over 100). Any found: the site does not clamp, and percentile.py's "
         "`beyond_range` names the roll. None found: the site clamps, or lists no such item — i04 says the filter "
         "finds what it should",
         "i04, this search at 100, must find items: the captures hold one at 100 (h015)",
         batch_query([], {**armour, **percentile_filter(101)})),
        ("i04",
         "i03's mutant: armour at a percentile of 100 or more. It must find items; each fetched is scored, and "
         "an item whose display no roll in range gives is a roll over the maximum shown clamped",
         "the mutant is the control: nothing found means the percentile filter decides nothing",
         batch_query([], {**armour, **percentile_filter(100)})),
        ("i05",
         "C14, a defence line on armour under its global twin — a line a private item shows exactly as the local "
         f"one: {', '.join('`' + t + '`' for t in DEFENCE_TWINS)}, each under every id displaying the text without "
         "`(Local)`. Whether the site's value leaves the line out (percentile.py by its ids reproduces it) or reads "
         "it (only the text reading does). The copy holds one such trace; none captured",
         PERCENTILE_CONTROL,
         batch_query([group("count", ids_of(*DEFENCE_TWINS), {"min": 1})], {**armour, **percentile_filter()})),
        ("i06",
         f"the ranged family on weapons, sound, at {pinned}: whether any weapon carries the unsuffixed "
         "`Adds # to # <type> Damage` under the stat without `(Local)` while showing none of the five plain "
         "pseudos. None found: the site counts that stat toward the plain pseudo on a weapon as anywhere else. "
         "Any found: a weapon whose global line the site does not count. Asked because 36 of the copy's weapon "
         "lines on uniques are ones the export cannot say local or global (data/ranged-uniques.csv)",
         "i07, the mutant, must find items; i08 says whether any weapon carries the stat at all",
         batch_query([group("count", global_ids, {"min": 1}), group("not", pseudos)], category("weapon"))),
        ("i07",
         "i06's mutant: the `(Local)` twins in place of the global ids. It must find weapons, each showing no "
         "plain pseudo, as round three found (e016, e021, e027, e033)",
         "the mutant is the control: nothing found means a `not` of the plain pseudos beside a count decides nothing",
         batch_query([group("count", local_ids, {"min": 1}), group("not", pseudos)], category("weapon"))),
        ("i08",
         "i06's control: weapons carrying the unsuffixed line under the stat without `(Local)`, the five plain "
         "pseudos shown. Any found: the site lists a weapon's line as global, and the build, which reads a weapon's "
         "line as its own (the plan, 9d), would not count what the site counts; each item names its unique. None "
         "found: no listed weapon carries the global stat, and the class decides as the site does",
         "i07: a count beside the weapon category finds weapons",
         batch_query([group("count", global_ids, {"min": 1}), group("if", pseudos)], category("weapon"))),
        ("i09",
         "C2, ward read as the other defences are, on any base with ward (two captured, both reproduced; the copy "
         "holds thirteen): more rolls against the ward range of base-defences.csv",
         PERCENTILE_CONTROL,
         batch_query([], {**armour, "armour_filters": {"disabled": False, "filters": {
             "ward": {"min": 1}, "base_defence_percentile": {"min": 1}}}})),
    ]
    return [{"search": s, "decides": d, "control": c, "query": q} for s, d, c, q in rows]


# Round j (before step 9d): the category track's open questions 1 to 6
# (search/category/README.md), holes J1-J4 of the build plan. Each type is
# written out; search/category/scripts/round-j.py reads the rows back and says
# what the export and the owner's copy hold of each. A category search finding
# nothing proves nothing until the same type under another id finds something:
# each such search names its pair, the pair being its control.
ROUND_J_BEASTS = {  # one red, one yellow under the lure and the mods readings (category F3)
    "Farric Ursa": "red",
    "Dune Hellion": "yellow",
}
BEAST_LEAVES = ["monster.beast", "monster.yellowbeast", "monster.redbeast"]
# Open question 2: a grouping, a base of a class in doubt (the copy's most
# held base of the class; the export's one Fishing Rod), and the class's own leaf.
ROUND_J_MEMBERS = [
    ("weapon.onemelee", "Opal Sceptre", "weapon.sceptre"),
    ("weapon.onemelee", "Ezomyte Dagger", "weapon.runedagger"),
    ("weapon.twomelee", "Imperial Staff", "weapon.basestaff"),
    ("weapon.twomelee", "Iron Staff", "weapon.warstaff"),
    ("weapon.twomelee", "Fishing Rod", "weapon.rod"),
    ("weapon.one", "Imbued Wand", "weapon.wand"),
    ("weapon.dagger", "Ezomyte Dagger", "weapon.runedagger"),
    ("weapon.onemace", "Opal Sceptre", "weapon.sceptre"),
    ("weapon.staff", "Iron Staff", "weapon.warstaff"),
]


def round_j():
    """j01–j07 a beast's leaf (J1), j08–j09 invitations (J3), j10–j13 what `map`
    holds (J2), j14–j28 the groupings' members (J4), j29–j32 breachstones and
    Growing Wombgift, j33–j34 `memoryline`, j35–j38 the wiki's named red
    beasts under the two colour leaves, which separate the lure reading from
    the mods reading (J1; the copy cannot)."""
    def at(option, item_type=None):
        return batch_query([], category(option) if option else None, item_type)

    rows = []
    n = 0

    def add(decides, control, query):
        nonlocal n
        n += 1
        rows.append({"search": f"j{n:02d}", "decides": decides, "control": control, "query": query})
        return f"j{n:02d}"

    # J1: the six searches of open question 1, then the seventh
    first = n + 1
    for base, colour in ROUND_J_BEASTS.items():
        for leaf in BEAST_LEAVES:
            add(f"J1, open question 1: whether `{leaf}` holds `{base}`, {colour} under the lure and the mods readings "
                "(the wiki's account, the owner's quote). Found under its colour's leaf and not the other: the two "
                "readings stand against the frame reading, which calls every beast yellow; found under `monster.beast`: "
                "Captured Beast holds that colour",
                f"the other two leaves with `{base}`, j{first:02d}–j{first + 5:02d}: one of the three finds it, or the type "
                "has no listing and its three answers say nothing",
                at(leaf, base))
    add("J1, the seventh: `monster.beast` alone — whether Captured Beast holds both colours: its fetched items' bases "
        "read by the lure and mods readings (both colours among them: it holds both), its total beside j01–j06",
        f"j{first:02d} and j{first + 3:02d}: `monster.beast` with a red and with a yellow base",
        at("monster.beast"))
    # J3: open question 4
    j = add("J3, open question 4: whether `map.invitation` (\"Maven's Invitation\") holds `Polaric Invitation`, on "
            "which the copy's 165 invitations hang. Found: the export's name rule stands. Not found, its pair finding "
            "it: the rule is wrong for every invitation not Maven's",
            "the pair, the next search: the type with no category",
            at("map.invitation", "Polaric Invitation"))
    add(f"{j}'s pair: `Polaric Invitation` with no category — that it is listed at all",
        f"the pair is the control: nothing found means the type has no listing and {j} says nothing",
        at(None, "Polaric Invitation"))
    # J2: open question 3
    j = add("J2, open question 3: whether `map` (\"Map\") holds `Inscribed Ultimatum` itself — a Misc Map Item under "
            "no child of `map`, so a parent holding items of its own and not only the union of its leaves",
            "the pair, the next search: the type with no category (the export puts it under no other id)",
            at("map", "Inscribed Ultimatum"))
    add(f"{j}'s pair: `Inscribed Ultimatum` with no category — that it is listed at all",
        f"the pair is the control: nothing found means the type has no listing and {j} says nothing",
        at(None, "Inscribed Ultimatum"))
    j = add("J2, open question 3: whether `map` holds `Winged Bestiary Scarab`, a `map.scarab` base — whether a "
            "parent holds what sits under it by the dot",
            "the pair, the next search: the type under `map.scarab`",
            at("map", "Winged Bestiary Scarab"))
    add(f"{j}'s pair: `Winged Bestiary Scarab` under `map.scarab`, the export's leaf for it",
        f"the pair is the control: nothing found means the type has no listing and {j} says nothing",
        at("map.scarab", "Winged Bestiary Scarab"))
    # J4: open question 2 — the leaves first, each the pair of the groupings' searches of its type
    leaf_row = {}
    for grouping, base, leaf in ROUND_J_MEMBERS:
        if (leaf, base) not in leaf_row:
            leaf_row[(leaf, base)] = add(
                f"J4's pair: `{base}` under `{leaf}`, the export's leaf for its class — that it is listed there "
                "(also the site's leaf-level answer for that class, which only the coarse list checked)",
                "the pair is the control of the groupings' searches of this type: nothing found means they say nothing",
                at(leaf, base))
    for grouping, base, leaf in ROUND_J_MEMBERS:
        add(f"J4, open question 2: whether the grouping `{grouping}` holds `{leaf}`, asked by `{base}`. Found: "
            f"a member; not found, its pair finding it: not a member",
            f"the pair, {leaf_row[(leaf, base)]}: `{base}` under `{leaf}`",
            at(grouping, base))
    # open question 5
    j = add("Open question 5: whether `map.breachstone` holds `Xoph's Breachstone` (the export puts the five "
            "breachstones under `map` and `map.breachstone` both)",
            "the pair, the next search: the type under `map`",
            at("map.breachstone", "Xoph's Breachstone"))
    add(f"{j}'s pair: `Xoph's Breachstone` under `map` — whether `map` holds it too (nested by the dot, or the "
        "export's second id)",
        f"{j}: nothing under both means the type has no listing and neither says anything",
        at("map", "Xoph's Breachstone"))
    j = add("Open question 5: whether `wombgift` holds `Growing Wombgift`, as the site's coarse list says, against the "
            "export's class, which puts it under `graft`",
            "the pair, the next search: the type under `graft`",
            at("wombgift", "Growing Wombgift"))
    add(f"{j}'s pair: `Growing Wombgift` under `graft`, the export's id for it",
        f"{j}: nothing under both means the type has no listing and neither says anything",
        at("graft", "Growing Wombgift"))
    # open question 6
    j = add("Open question 6: what `memoryline` (\"Memory Line\") holds, checked by the export alone: its fetched "
            "items' base types against the export's four Memories. None found: the leaf holds no listing",
            "the next search: a Memory with no category; nothing under both means no Memory is listed",
            at("memoryline"))
    add(f"{j}'s mutant: `Alva's Memory`, the first of the export's four Memories by name, with no category — whether "
        "a Memory is listed at all (none of the four is on the site's coarse list)",
        f"{j}: a Memory found here and nothing there says the leaf does not hold it",
        at(None, "Alva's Memory"))
    # J1, j35-j38: the wiki's named red beasts, which separate the lure reading from the mods reading
    # where the copy cannot (category F3: the two agree on all 158 of its beasts). The four ", First of
    # the " names are matched to the Spirit Beasts by the quote's "First Ones" only — the owner's to
    # confirm; the first of them by name is asked.
    first = n + 1
    for base, why in (("Black Mórrigan", "named by the wiki (\"The Black Mórrigan\")"),
                      ("Craiceann, First of the Deep", "the first by name of the site's four \", First of the \" "
                       "names, matched to the wiki's Spirit Beasts by the quote's \"First Ones\" (the owner's to confirm)")):
        for leaf in ("monster.redbeast", "monster.yellowbeast"):
            add(f"J1, lure against mods: whether `{leaf}` holds `{base}`, {why}. The lure reading calls it yellow; "
                "the mods reading cannot be measured for it on the copy, which holds none; the wiki names it red. "
                "Found under red and not yellow: the lure reading is wrong for it and the wiki's account stands; "
                "under yellow and not red: the lure reading stands against the wiki",
                f"the other leaf with `{base}`, j{first:02d} and j{first + 1:02d}: one of the two finds it, or the type "
                "has no listing under either and both answers say nothing",
                at(leaf, base))
        first += 2
    # J2, j39-j40: j12 and j13 found nothing on both sides (the site lists no Winged Bestiary Scarab), so
    # that pair said nothing; the same two categories ask again with a scarab the site's `map` list and
    # the copy both hold, chosen by search/category/scripts/round-j-read.py (the copy's most held, ties by name).
    j = add("J2, open question 3, j12 again: whether `map` holds `Anarchy Scarab`, a `map.scarab` base — whether "
            "a parent holds what sits under it by the dot. j12 and j13 found nothing on both sides",
            "the pair, the next search: the type under `map.scarab`",
            at("map", "Anarchy Scarab"))
    add(f"{j}'s pair, j13 again: `Anarchy Scarab` under `map.scarab`, the export's leaf for it",
        f"the pair is the control: nothing found means the type has no listing and {j} says nothing",
        at("map.scarab", "Anarchy Scarab"))
    return rows


# Round k (before step 9d): J8's one search and its mutant, then every beast's
# colour from the site (search/category/BRIEF.md at 213578b1). No published list
# gives a beast's colour, so each name of the site's `monster` list is asked
# under `monster.yellowbeast`; a name not found is asked under
# `monster.redbeast` in the round composed after this sitting. The copy's
# bases come first, written out; search/category/scripts/round-k.py reads the
# rows back and fails when this list is not beasts.csv's.
ROUND_K_COPY_BEASTS = [  # search/category/data/beasts.csv's bases, in its order (the copy's 70)
    'Alpine Devil', 'Barrow Ape', 'Bearded Devil', 'Blood Ape', 'Bone Cruncher', 'Brine Vassal',
    'Carnage Ape', 'Cave Beast', 'Chimeric Croaker', 'Colossus Crusher', 'Craicic Maw',
    'Craicic Sand Spitter', 'Craicic Savage Crab', 'Craicic Shield Crab', 'Craicic Squid', 'Craicic Watcher',
    'Crustacean Sniper', 'Cursed Spawn', 'Dread Primate', 'Dune Hellion', 'Enraptured Beast',
    'Enslaved Hellion', 'Farric Frost Hellion Alpha', 'Farric Goatman', 'Farric Goliath', 'Farric Lynx Alpha',
    'Farric Ursa', 'Fenumal Devourer', 'Fenumal Scorpion', 'Fenumal Scrabbler', 'Fenumal Widow',
    'Fighting Bull', 'Flame Hellion', 'Forest Beast', 'Freezing Wolf', 'Fury Hound', 'Gluttonous Gull',
    'Goatman', 'Goatman Fire-raiser', 'Goatman Shaman', 'Granite Eater', 'Gravel Eater', 'Host Chieftain',
    'Infested Crawler', "Merveil's Attendant", "Merveil's Blessed", "Merveil's Chosen", "Merveil's Favoured",
    "Merveil's Retainer", 'Mindless Scavenger', 'Mountain Hellion', 'Mountain Lynx', 'Pitbull Demon',
    'Porcupine Goliath', 'Rock Spitter', 'Rooster Fiend', 'Rotting Vulture', 'Saqawine Blood Viper',
    'Saqawine Chimeral', 'Saqawine Cobra', 'Scavenging Vulture', 'Sewage Crawler', 'Shackled Hellion',
    'Singing Siren', 'Skeletal Beast', 'Snow Wolf', 'Soulless Watcher', 'Vaulting Croaker', 'Venomous Spawn',
    'Waste Lurcher',
]
MONSTERS = TRACK.parent / "trade-query" / "data" / "items-2026-09-12.json"


def round_k():
    """k001–k002 J8: a defence twin text as an implicit under the `(Local)` stat
    on armour, and its mutant under the global stat; k003 on, one search per
    name of the site's `monster` list under `monster.yellowbeast`, the copy's
    bases first, then the rest by name."""
    implicit = {}
    for g in json.loads(STATS.read_text())["result"]:
        if g["id"] == "implicit":
            for e in g["entries"]:
                implicit.setdefault(e["text"], []).append(e["id"])
    local_ids = [i for t in DEFENCE_TWINS for i in implicit.get(t + " (Local)", [])]
    global_ids = [i for t in DEFENCE_TWINS for i in implicit.get(t, [])]
    if len(local_ids) != len(DEFENCE_TWINS) or not global_ids:
        raise SystemExit("round k: the implicit twin ids are not what stats-2026-09-12.json listed when it was written")
    twins = ", ".join("`" + t + "`" for t in DEFENCE_TWINS)
    armour = {**category("armour"), **percentile_filter()}
    rows = [
        ("k001",
         f"J8 (pseudo-stats open question 9): armour carrying a defence twin text — {twins} — as an implicit under "
         f"the `(Local)` stat ({len(local_ids)} implicit ids), the percentile required. Found: a local defence line "
         "sits as an implicit, and the array alone does not separate the twins. None found, beside k002 finding "
         "items: on listed armour an implicit twin is the global stat, and synthesised implicits stay the limit",
         f"k002, the mutant, must find items; and {PERCENTILE_CONTROL}",
         batch_query([group("count", local_ids, {"min": 1})], armour)),
        ("k002",
         f"k001's mutant: the same texts as an implicit under the global stat ({len(global_ids)} implicit ids), the "
         "percentile required. It must find the ten i05 found, each an eldritch implicit",
         "i05: its ten items carry the twin as an eldritch implicit under these global ids "
         "(search/pseudo-stats/data/defence-twins.csv); nothing found here means an implicit `count` beside the "
         "percentile decides nothing, and k001 with it",
         batch_query([group("count", global_ids, {"min": 1})], armour)),
    ]
    site = [e["type"] for c in json.loads(MONSTERS.read_text())["result"] if c["id"] == "monster" for e in c["entries"]]
    if len(set(site)) != len(site) or not set(ROUND_K_COPY_BEASTS) <= set(site):
        raise SystemExit("round k: the site's monster list has a name twice, or lacks one of the copy's bases")
    order = ROUND_K_COPY_BEASTS + sorted(n for n in site if n not in set(ROUND_K_COPY_BEASTS))
    for number, name in enumerate(order, start=3):
        held = "a base of the copy" if name in ROUND_K_COPY_BEASTS else "on no beast of the copy"
        rows.append((
            f"k{number:03d}",
            f"J1, the colour table: whether the site lists `{name}` ({held}) as a Yellow Beast. Found: yellow by the "
            "site's own word (C106, clause (a)), this capture its row's citation. None found: its pair decides",
            f"its pair, by rule: `{name}` under `monster.redbeast`, in the round composed after this sitting for "
            "every name this round does not find; nothing under both means the name has no listing",
            batch_query([], category("monster.yellowbeast"), name)))
    return [{"search": s, "decides": d, "control": c, "query": q} for s, d, c, q in rows]


# Round l (before step 9d): the second pass of every beast's colour
# (search/category/BRIEF.md at 213578b1). Each name round k did not find under
# Yellow Beast is asked under `monster.redbeast`, the same type, in round k's
# order (the copy's bases first); its round k search is its pair. Written out
# from search/category/scripts/round-k-read.py's beast-colours.csv.
ROUND_L_NAMES = [  # search/category/data/beast-colours.csv's `unknown` names, each with its round k search, in round k's order
    ('Craicic Maw', 'k013'), ('Craicic Sand Spitter', 'k014'), ('Craicic Savage Crab', 'k015'),
    ('Craicic Shield Crab', 'k016'), ('Craicic Squid', 'k017'), ('Craicic Watcher', 'k018'),
    ('Farric Frost Hellion Alpha', 'k025'), ('Farric Goatman', 'k026'), ('Farric Goliath', 'k027'),
    ('Farric Lynx Alpha', 'k028'), ('Farric Ursa', 'k029'), ('Fenumal Devourer', 'k030'),
    ('Fenumal Scorpion', 'k031'), ('Fenumal Scrabbler', 'k032'), ('Fenumal Widow', 'k033'),
    ('Saqawine Blood Viper', 'k060'), ('Saqawine Chimeral', 'k061'), ('Saqawine Cobra', 'k062'),
    ('Aidan the Frenzied', 'k076'), ('Albino Rhoa', 'k077'), ('Amarissa, Daughter of Merveil', 'k080'),
    ('Ambrosia, Daughter of Merveil', 'k081'), ('Black Death', 'k093'), ('Black Mórrigan', 'k094'),
    ('Bleached Crustacean', 'k099'), ('Blood Morpher', 'k101'), ('Boulderback', 'k105'),
    ('Brinecrack', 'k107'), ('Brood Princess', 'k108'), ('Carrion Minion', 'k113'),
    ('Cave Crustacean', 'k116'), ('Common Frog', 'k126'), ('Corrupted Spitter', 'k130'),
    ('Craiceann, First of the Deep', 'k131'), ('Craicic Croaker', 'k132'), ('Craicic Spider Crab', 'k133'),
    ('Craicic Vassal', 'k134'), ('Crazed Flamebeast', 'k136'), ('Drifting Eye', 'k145'),
    ('Eyepecker', 'k156'), ('Farric Ape', 'k157'), ('Farric Chieftain', 'k158'),
    ('Farric Flame Hellion Alpha', 'k159'), ('Farric Gargantuan', 'k160'), ('Farric Magma Hound', 'k161'),
    ('Farric Pit Hound', 'k162'), ('Farric Taurus', 'k163'), ('Farric Tiger Alpha', 'k164'),
    ('Farric Wolf Alpha', 'k165'), ('Farrul, First of the Plains', 'k166'), ('Fathom Screamer', 'k168'),
    ('Fenumal Hybrid Arachnid', 'k169'), ('Fenumal Plagued Arachnid', 'k170'), ('Fenumal Queen', 'k171'),
    ('Fenumus, First of the Night', 'k172'), ('Fused Crawler', 'k179'),
    ('Hollowskull, the Willing Host', 'k188'), ('Infected Adder', 'k192'), ('Infected Ambusher', 'k193'),
    ('Infected Beast', 'k194'), ('Infected Watcher', 'k196'), ('Infested Sniper', 'k204'),
    ('Invading Crustacean', 'k210'), ('Lowlands Hopper', 'k216'), ("Maligaro's Inspiration", 'k218'),
    ("Maligaro's Muse", 'k219'), ('Nadia the Soothing', 'k244'), ('Oozeback Bloom', 'k249'),
    ('Paradisae Venenum', 'k250'), ('Primal Crushclaw', 'k260'), ('Primal Cystcaller', 'k261'),
    ('Primal Rhex Matriarch', 'k262'), ('Putrid Chimeral', 'k266'), ('Putrid Serpent', 'k267'),
    ('Putrid Weaver', 'k268'), ("Q'uru", 'k269'), ('Rabid Broodqueen', 'k271'), ('Rabid Minion', 'k273'),
    ('Rabid Rhoa', 'k274'), ("Raihara, Tukohama's Loyal", 'k275'), ('Rocco, the Bloodthirsty', 'k280'),
    ('Saqawal, First of the Sky', 'k287'), ('Saqawine Retch', 'k288'), ('Saqawine Rhex', 'k289'),
    ('Saqawine Rhoa', 'k290'), ('Saqawine Vulture', 'k291'), ('Scrabbling Spitter', 'k295'),
    ('Sickly Spinner', 'k301'), ('Steelchaw', 'k312'), ('The Basilisk', 'k324'),
    ('The Burning Menace', 'k325'), ('The Cadaver Bull', 'k326'), ('The Conqueror Wurm', 'k327'),
    ('The Dweller of the Deep', 'k329'), ('The Faun', 'k331'), ('The Great White Beast', 'k332'),
    ('The Great White Bones', 'k333'), ('The Hundred Foot Shadow', 'k334'), ('The Weaver', 'k336'),
    ('Thistlesage', 'k338'), ('Tunnelworm', 'k342'), ('Ungulath', 'k345'), ('Unstable Larva', 'k346'),
    ('Ventarus', 'k348'), ('Vivid Abberarach', 'k352'), ('Vivid Vulture', 'k353'), ('Vivid Watcher', 'k354'),
    ('Wild Brambleback', 'k357'), ('Wild Bristle Matron', 'k358'), ('Wild Hellion Alpha', 'k360'),
    ('Zombie Rhoa', 'k363'),
]

def round_l():
    """l001 on: one search per name of ROUND_L_NAMES under `monster.redbeast`."""
    rows = []
    for number, (name, pair) in enumerate(ROUND_L_NAMES, start=1):
        rows.append({
            "search": f"l{number:03d}",
            "decides": (f"J1, the colour table: whether the site lists `{name}` as a Red Beast, {pair} having found it "
                        "under no Yellow Beast. Found: red by the site's own word (C106, clause (a)), this capture its "
                        "row's citation. None found, beside its pair's nothing: the name has no listing and its colour "
                        "stays unknown"),
            "control": f"its pair, {pair}: `{name}` under `monster.yellowbeast`, found nothing",
            "query": batch_query([], category("monster.redbeast"), name),
        })
    return rows


def round_m():
    """m01–m04 the tattoo name rule (the look at 9d-B: 40 of the class table's
    100 tattoo bases do not begin `Tattoo of `), m05–m07 `Veiled Scarab`, which
    the export classes Stackable Currency, m08 a beast under `currency` (J10,
    ruled on the owner's word and asked by no capture)."""
    def at(option, item_type):
        return batch_query([], category(option) if option else None, item_type)

    rows = []

    def add(decides, control, query):
        rows.append({"search": f"m{len(rows) + 1:02d}", "decides": decides, "control": control, "query": query})
        return rows[-1]["search"]

    for base in ("Honoured Tattoo of the Oak", "Journey Tattoo of the Body"):
        m = add(f"the tattoo name rule: whether `currency.tattoo` (\"Tattoo\") holds `{base}`, a Stackable Currency "
                "base that holds the word and does not begin `Tattoo of `. Found: the rule is the word, not the "
                "prefix. Not found, its pair finding it: the prefix stands for this name",
                "the pair, the next search: the type with no category",
                at("currency.tattoo", base))
        add(f"{m}'s pair: `{base}` with no category — that it is listed at all",
            f"the pair is the control: nothing found means the type has no listing and {m} says nothing",
            at(None, base))
    m = add("whether `map.scarab` (\"Scarab\") holds `Veiled Scarab`, which the export classes Stackable Currency "
            "and the scarab rule, a Map Fragment's, never sees. Found: a scarab by its name whatever its class",
            "m07, the type with no category",
            at("map.scarab", "Veiled Scarab"))
    add(f"whether `currency` (\"Any Currency\") holds `Veiled Scarab`, as its class says; read beside {m}",
        "m07, the type with no category",
        at("currency", "Veiled Scarab"))
    add(f"the pair of {m} and the search after it: `Veiled Scarab` with no category — that it is listed at all",
        "the pair is the control: nothing found means the type has no listing and the two say nothing",
        at(None, "Veiled Scarab"))
    add("J10: whether `currency` holds `Farric Ursa`, a captured beast, whose class is Stackable Currency (J9). "
        "Not found: the owner's word (\"Trade searches for the category \"Any Currency\" filter out beasts.\") has "
        "its capture. Found: J10 is brought back to him",
        "l011: `Farric Ursa` under `monster.redbeast`, found 3655 on 2026-10-04",
        at("currency", "Farric Ursa"))
    return rows


# Round n (before 9d-C's sitting): what the two reads left to the site
# (search/pseudo-stats/README.md, "Before the sitting: the domain, and the twin
# from the export"). Each pseudo at the version asked, written out.
ROUND_N = {
    "increased_spell_damage": 3,
    "adds_fire_to_attacks": 3,
    "adds_fire": 2,
    "increased_movement_speed": 3,
    "total_chaos_res": 3,
    "adds_cold": 2,
    "total_life": 2,
    "total_res": 3,
}
# A crucible entry of the stats file is a whole node, its numbers and its tier
# in the text and a row to a line: the nodes that display a row, counted with
# each number read as `#` and the tier dropped. A stats file that counts
# another number is another round's.
ROUND_N_NODES = {"#% increased Spell Damage": 40, "Adds # to # Fire Damage": 64}
# The flask base whose own line (`utilityMods`) is the total's row on the copy.
ROUND_N_FLASKS = [("increased_movement_speed", "Quicksilver Flask", "#% increased Movement Speed"),
                  ("total_chaos_res", "Amethyst Flask", "+#% to Chaos Resistance")]
# The categories that hold the copy's eight implicit lines off a weapon.
ROUND_N_OFF_WEAPON = [("armour.helmet", "helmet"), ("accessory.ring", "ring")]
# A thin domain and the total asked sound over its ids of that domain alone.
ROUND_N_DOMAINS = [("fractured", "total_life"), ("enchant", "increased_spell_damage"),
                   ("scourge", "total_res"), ("crafted", "total_res")]
RANGED_LOCAL = [f"Adds # to # {kind.capitalize()} Damage (Local)"
                for kind in ("physical", "lightning", "cold", "fire", "chaos")]


def round_n():
    """n01–n02 a crucible line under a total; n03–n05 a weapon's crucible
    `Adds # to # Fire Damage`, its own or the other twin; n06–n09 a flask's own
    line; n10–n15 the implicit twin off a weapon; n16–n23 the four thin
    domains. A search that can find nothing is followed by its mutant, the
    pseudo shown (`if`) where the search refused it or required it."""
    ids, texts = table.stats()
    versions = table.versions()

    def at(name):
        return versions[name][ROUND_N[name] - 1]

    def pin(name):
        return f"{name} v{ROUND_N[name]}"

    def nodes(template):
        """The crucible nodes that display a row of this template."""
        found = []
        for g in json.loads(STATS.read_text())["result"]:
            if g["id"] != "crucible":
                continue
            for e in g["entries"]:
                lines = re.sub(r" \(Tier \d+\)$", "", e["text"]).split("\n")
                if template in [re.sub(r"\d+(?:\.\d+)?", "#", line) for line in lines]:
                    found.append(e["id"])
        if len(found) != ROUND_N_NODES[template]:
            raise SystemExit(f"{len(found)} crucible nodes display {template!r}, not {ROUND_N_NODES[template]}")
        return found

    rows = []

    def add(decides, control, query):
        rows.append({"search": f"n{len(rows) + 1:02d}", "decides": decides, "control": control, "query": query})
        return rows[-1]["search"]

    # 1. A crucible line under a total that is no ranged one.
    spell = at("increased_spell_damage")
    spell_text = texts[spell["pseudo"]]
    spell_nodes = nodes("#% increased Spell Damage")
    spell_rows = table.ids_of(spell, ids)
    carried = (f"a crucible node displaying `#% increased Spell Damage` ({len(spell_nodes)} nodes, a count) and "
               f"none of the total's rows ({len(spell_rows)} ids, a `not`)")
    n = add(f"{pin('increased_spell_damage')}, a crucible line: whether any listed item carries {carried} while showing "
            f"no `{spell_text}`. None found, the mutant finding items: the pseudo counts a crucible line. Any found: "
            "it does not count that node's, and the item's lines say whether a `reduced` node cancels it",
            "the next search, the mutant, must find items",
            batch_query([group("count", spell_nodes, {"min": 1}), group("not", spell_rows),
                         group("not", [spell["pseudo"]])]))
    add(f"{n}'s mutant: the pseudo shown (`if`) where {n} refused it. It must find items, each carrying {carried}; "
        "the ten fetched show the pseudo's value beside the node's line, or none. None found: no such item is listed, "
        f"or a count of crucible nodes finds nothing, and {n} says nothing",
        f"the mutant is the control: nothing found means {n} decides nothing",
        batch_query([group("count", spell_nodes, {"min": 1}), group("not", spell_rows),
                     group("if", [spell["pseudo"]])]))

    # 2. A weapon's crucible `Adds # to # Fire Damage`: its own, the other twin, or not counted.
    attacks, plain = at("adds_fire_to_attacks"), at("adds_fire")
    attacks_text, plain_text = texts[attacks["pseudo"]], texts[plain["pseudo"]]
    fire_nodes = nodes("Adds # to # Fire Damage")
    fire_rows = table.ids_of(attacks, ids)
    if not set(table.ids_of(plain, ids)) <= set(fire_rows):
        raise SystemExit("the plain fire pseudo has a row the attacks' has not")
    carried = (f"a crucible node displaying `Adds # to # Fire Damage` ({len(fire_nodes)} nodes, a count) and none of "
               f"the rows of {pin('adds_fire_to_attacks')} ({len(fire_rows)} ids, a `not`; those of {pin('adds_fire')} "
               "are among them)")
    weapon = category("weapon")
    a = f"n{len(rows) + 1:02d}"
    b = f"n{len(rows) + 2:02d}"
    m = f"n{len(rows) + 3:02d}"
    readings = (f"Read with its pair: {a} none and {b} found, the weapon's own (the attacks' pseudo alone); both none, "
                f"the other twin (the plain too); both found, not counted; {a} found and {b} none, counted by the plain "
                "and not the attacks', which no row says")
    add(f"a weapon's crucible line, the attacks' pseudo: whether any weapon carries {carried} while showing no "
        f"`{attacks_text}`. None found, {m} finding items: the node's line is counted. Any found: it is not. {readings}",
        f"{m}, the mutant, must find items",
        batch_query([group("count", fire_nodes, {"min": 1}), group("not", fire_rows),
                     group("not", [attacks["pseudo"]])], weapon))
    add(f"a weapon's crucible line, the plain pseudo: whether any weapon carries {carried} while showing no "
        f"`{plain_text}`. None found, {m} finding items: the node's line is the other twin. Any found: it is the "
        f"weapon's own, or not counted. {readings}",
        f"{m}, the mutant, must find items",
        batch_query([group("count", fire_nodes, {"min": 1}), group("not", fire_rows),
                     group("not", [plain["pseudo"]])], weapon))
    add(f"the mutant of {a} and {b}: the two pseudos shown (`if`) where each refused one. It must find weapons, each "
        f"carrying {carried}; the ten fetched show `{attacks_text}` and `{plain_text}` beside the node's line, or "
        f"neither. None found: no such weapon is listed, and {a} and {b} say nothing",
        f"the mutant is the control: nothing found means {a} and {b} decide nothing",
        batch_query([group("count", fire_nodes, {"min": 1}), group("not", fire_rows),
                     group("if", [attacks["pseudo"], plain["pseudo"]])], weapon))

    # 3. A flask's own line.
    for name, base, line in ROUND_N_FLASKS:
        pseudo = at(name)["pseudo"]
        m = f"n{len(rows) + 2:02d}"
        n = add(f"{pin(name)}, a flask's own line: whether any listed flask shows `{texts[pseudo]}`. Any found: a "
                f"pseudo is shown on a flask, and its lines say from which. None found, {m} finding flasks: no flask "
                f"shows it, and a flask's own `{line}` is not counted",
                f"{m}, the mutant narrowed to the base that carries the line, must find items",
                batch_query([group("and", [pseudo])], category("flask")))
        add(f"{n}'s mutant, narrowed: `{base}`, whose own line is `{line}` on the copy, the pseudo shown (`if`) "
            f"where {n} required it. It must find flasks; the ten fetched show what the site files the line under, "
            f"and `{texts[pseudo]}` beside it or none. None found: the category finds no flask, and {n} says nothing",
            f"the mutant is the control: nothing found means {n} decides nothing",
            batch_query([group("if", [pseudo])], category("flask"), base))

    # 4. The implicit twin off a weapon.
    cold = at("adds_cold")
    cold_text = texts[cold["pseudo"]]
    other = [i for i in ids_of("Adds # to # Cold Damage") if i.startswith("implicit.")]
    own = [i for i in ids_of(*RANGED_LOCAL) if i.startswith("implicit.")]
    for option, word in ROUND_N_OFF_WEAPON:
        m = f"n{len(rows) + 2:02d}"
        n = add(f"{pin('adds_cold')}, the implicit twin off a weapon: whether any {word} carries `Adds # to # Cold Damage` "
                f"as an implicit under the stat without `(Local)` (ids: {len(other)}, a count) while showing no "
                f"`{cold_text}`. None found, {m} finding items: the plain pseudo counts the implicit off a weapon, "
                "as the reading says. Any found: it does not count that item's",
                f"{m}, the mutant, must find items",
                batch_query([group("count", other, {"min": 1}), group("not", [cold["pseudo"]])], category(option)))
        add(f"{n}'s mutant: the pseudo shown (`if`) where {n} refused it. It must find a {word} carrying the implicit; "
            f"the ten fetched show `{cold_text}` beside the line, or none. None found: no {word} carries the stat, "
            f"and {n} says nothing",
            f"the mutant is the control: nothing found means {n} decides nothing",
            batch_query([group("count", other, {"min": 1}), group("if", [cold["pseudo"]])], category(option)))
        add(f"whether any {word} carries an unsuffixed `Adds # to # <type> Damage` as an implicit under the `(Local)` "
            f"stat at all ({len(own)} ids, the five types, a count), `{cold_text}` shown where there is one. None "
            f"found, {m} finding items: off a weapon the implicit is never the `(Local)` stat's, and the class "
            "decides as the site does. Any found: an item that is no weapon carries a weapon's own stat; each names "
            "its base, and what it shows says whether the site counts it",
            f"{m}, this search with the stat without `(Local)` in place of the `(Local)` ids, must find items",
            batch_query([group("count", own, {"min": 1}), group("if", [cold["pseudo"]])], category(option)))

    # 5. The thin domains.
    for domain, name in ROUND_N_DOMAINS:
        version = at(name)
        text = texts[version["pseudo"]]
        rows_of = weighted(version, ids)
        rows_of["filters"] = [f for f in rows_of["filters"] if f["id"].startswith(domain + ".")]
        if not 0 < len(rows_of["filters"]) <= WEIGHTED_AT_MOST:
            raise SystemExit(f"{name} has {len(rows_of['filters'])} {domain} ids")
        m = f"n{len(rows) + 2:02d}"
        n = add(f"{pin(name)}, sound over its {domain} lines: whether any listed item's {domain} rows — the site's "
                f"own sum of its ids of that domain ({len(rows_of['filters'])}), a half or more — show no `{text}`. "
                f"None found, {m} finding items: {domain} lines are counted wherever they appear. Any found: one of "
                f"its {domain} lines is not counted, or lines of another domain cancel it; the item's lines say which",
                f"{m}, the mutant, must find items; c4 and d02 for a pseudo in a `not` beside a sum",
                batch_query([rows_of, group("not", [version["pseudo"]])]))
        add(f"{n}'s mutant: the pseudo shown (`if`) where {n} refused it. It must find items whose {domain} rows "
            f"sum so; the ten fetched show `{text}` beside the line. None found: no item lists such a line, and {n} "
            "says nothing",
            f"the mutant is the control: nothing found means {n} decides nothing",
            batch_query([rows_of, group("if", [version["pseudo"]])]))
    return rows


# Round t (the trade import, step 7 of search/TRADE-PLAN.md): what the
# translation leaves to the site — the questions of the trade oracle's
# sitting findings (crates/acquisition-search/tests/trade-oracle.toml, `asks`)
# and the catalogue rows a sitting is to settle
# (crates/acquisition-search/reference/trade-catalogue-v1.toml). Each search
# is decided by what it finds beside its pair, or by the ten items it
# fetches, read against the catalogue's reading; the items each names are
# ones the captures hold (search/pseudo-stats/data/bodies.json).


def round_t():
    """t01–t12 a stat displayed otherwise than its listed text: its number
    read below nothing, as -100, as 100, as 1; t13–t20 an explicit id on a
    crafted or fractured line, and a stat's lines summed; t21–t23 a scourge
    line in a weighted sum; t24–t26 a `not` with a value; t27–t32 which
    number a text compares, its `#` or a digit it writes; t33–t36 two ids of
    one text, and an id of two texts; t37–t40 a group with no bound; t41–t56
    the property filters, the free text and the marks; t57 t12's control. A search that can
    find nothing names its pair or its mutant; one read by its ten fetched
    says what each reading would show."""
    rows = []

    def add(decides, control, q):
        rows.append({"search": f"t{len(rows) + 1:02d}", "decides": decides, "control": control, "query": q})
        return rows[-1]["search"]

    def stat(sid, value=None):
        f = {"id": sid, "disabled": False}
        if value is not None:
            f["value"] = value
        return f

    def stats(kind, filters, value=None):
        out = {"type": kind, "filters": filters}
        if value is not None:
            out["value"] = value
        return out

    def ask(groups, filters=None, item_type=None, name=None, term=None):
        q = batch_query(groups, filters, item_type)
        if name:
            q = {"status": q["status"], "name": name, **{k: v for k, v in q.items() if k != "status"}}
        if term:
            q = {"status": q["status"], "term": term, **{k: v for k, v in q.items() if k != "status"}}
        return q

    def filt(group_id, **filters):
        return {group_id: {"disabled": False, "filters": filters}}

    def merge(*parts):
        out = {}
        for p in parts:
            out.update(p)
        return out

    # 1. A stat displayed otherwise than its listed text. The captures' line
    # hashes file each form under the id (the oracle's findings), so presence
    # is shown; what each search asks is the number the site reads.
    speed = "explicit.stat_681332047"  # `#% increased Attack Speed`
    add("reduced-spelling, presence: whether `The Anvil` (Amber Amulet), whose line is `10% reduced Attack Speed` "
            f"filed under {speed} (`#% increased Attack Speed`), is found by that id with no bound. Found: the id's "
            "presence holds the `reduced` spelling",
            "t05, The Anvil with no stat, must find it",
            ask([stats("and", [stat(speed)])], item_type="Amber Amulet", name="The Anvil"))
    add(f"reduced-spelling, the number: The Anvil under {speed} with `max` -5. Found, t03 none: the site reads "
        "`10% reduced` as -10 under the `increased` id. Not found: it reads no negative there",
        "t01, the same id with no bound, finds it",
        ask([stats("and", [stat(speed, {"max": -5})])], item_type="Amber Amulet", name="The Anvil"))
    add(f"reduced-spelling, the number's mutant: `max` -11, which -10 does not meet. None found beside the search "
        "before it finding: -10 exactly, read from the text. Found: the site's number is not the text's",
        "t01 finds it",
        ask([stats("and", [stat(speed, {"max": -11})])], item_type="Amber Amulet", name="The Anvil"))
    mana = "explicit.stat_474294393"  # `#% reduced Mana Cost of Skills`
    add(f"the inverse spelling: whether `The Blood Reaper` (Headsman Axe), whose line `#% increased Mana Cost of "
        f"Skills` is filed under {mana} (listed `#% reduced Mana Cost of Skills`), is found with `max` -1. Found: the "
        "`increased` spelling of a `reduced` text is read below nothing, the rule of the search before it turned "
        "round. Not found, its pair finding it: the site reads it otherwise",
        "t06, The Blood Reaper with no stat, must find it",
        ask([stats("and", [stat(mana, {"max": -1})])], item_type="Headsman Axe", name="The Blood Reaper"))
    add("the pair of The Anvil's and The Blood Reaper's searches: each unique by its name with no stat — that it is "
        "listed at all. This one The Anvil",
        "the pair is the control: nothing found means the searches beside it say nothing",
        ask([], item_type="Amber Amulet", name="The Anvil"))
    add("the pair, The Blood Reaper by its name with no stat",
        "the pair is the control",
        ask([], item_type="Headsman Axe", name="The Blood Reaper"))
    phys = "explicit.stat_1509134228"  # `#% increased Physical Damage`
    add(f"no-physical-damage, the number: `Storm Cloud` (Long Bow), whose `No Physical Damage` is filed under "
            f"{phys} (`#% increased Physical Damage`), with `max` -99. Found, t08 none: the site reads it as "
            "-100. None: as no number below -99",
            "t09, Storm Cloud under the id with no bound, must find it",
            ask([stats("and", [stat(phys, {"max": -99})])], item_type="Long Bow", name="Storm Cloud"))
    add(f"no-physical-damage, the mutant: `max` -101. None found beside t07 finding: -100 exactly",
        "t09 finds it",
        ask([stats("and", [stat(phys, {"max": -101})])], item_type="Long Bow", name="Storm Cloud"))
    add("no-physical-damage, presence: Storm Cloud under the id with no bound. Found: the id's presence holds "
        "`No Physical Damage`; not found, Storm Cloud is unlisted and the two before say nothing",
        "this search is the control of the two before it",
        ask([stats("and", [stat(phys)])], item_type="Long Bow", name="Storm Cloud"))
    ire = "enchant.stat_620045439"  # `#% chance to Trigger Commandment of Ire when Hit`
    add(f"an elided 100 %: {ire}, listed `#% chance to Trigger Commandment of Ire when Hit`, displayed `Trigger "
        "Commandment of Ire when Hit` on the captures' Runic Gauntlets, with `min` 100. Found: the line with no "
        "number is read as 100; the ten fetched show whether each displays the number or not",
        "t11, the id with no bound, must find items",
        ask([stats("and", [stat(ire, {"min": 100})])]))
    add(f"the elided 100 %'s control: {ire} with no bound; the ten fetched show the line's forms",
        "the control",
        ask([stats("and", [stat(ire)])]))
    pierce = "implicit.stat_2067062068"  # `Projectiles Pierce # additional Targets`
    add(f"`an` for one: {pierce}, listed `Projectiles Pierce # additional Targets`, displayed `Projectiles Pierce an "
        "additional Target`, with `min` 1 and `max` 1. Found, the ten fetched displaying `an additional Target`: "
        "the word is read as 1",
        "t57, the id with no bound, must find items",
        ask([stats("and", [stat(pierce, {"min": 1, "max": 1})])]))

    # 2. An explicit id on a crafted or fractured line (K6), and a stat's lines summed (the oracle's q6).
    life = "3299347043"  # `+# to maximum Life`
    for domain in ("crafted", "fractured"):
        m = f"t{len(rows) + 2:02d}"
        add(f"an explicit id on a {domain} line: items carrying `{domain}.stat_{life}` (Life) and none of "
            f"`explicit.stat_{life}`, a `not`. None found, {m} finding items: the explicit id holds the {domain} "
            f"line (every such item excluded). Found: it does not, and the fetched items' lines show a {domain} "
            "Life line with no explicit one",
            f"{m}, the {domain} id alone, must find items",
            ask([stats("and", [stat(f"{domain}.stat_{life}")]), stats("not", [stat(f"explicit.stat_{life}")])]))
        add(f"the {domain} id alone: that items carry a {domain} Life line at all",
            "the control of the search before it",
            ask([stats("and", [stat(f"{domain}.stat_{life}")])]))
    for domain, bound, below in (("fractured", 150, 100), ("crafted", 150, 100)):
        add(f"stat-summed: chests under `explicit.stat_{life}` with `min` {bound} and `{domain}.stat_{life}` with "
            f"`max` {below}: read the ten fetched. An item none of whose Life lines reaches {bound} while they do "
            f"together: the site compares the item's sum of the stat's lines (q6: +40 and +117 fractured at `min` "
            "150). Every item with one line of the bound: one line is compared, and q6 is another reading",
            f"q6, the owner's, item 8 found so; the {domain} id's control above",
            ask([stats("and", [stat(f"explicit.stat_{life}", {"min": bound}), stat(f"{domain}.stat_{life}", {"max": below})])],
                category("armour.chest")))
    add(f"stat-summed, q6's own item at the edge: chests under `explicit.stat_{life}` with `min` 157 and "
        f"`fractured.stat_{life}` with `max` 117 and `min` 117. Found with +40 and +117 among them: the sum (157); "
        "read beside t20",
        "t20, the same at `min` 158, must find no such item",
        ask([stats("and", [stat(f"explicit.stat_{life}", {"min": 157}), stat(f"fractured.stat_{life}", {"min": 117, "max": 117})])],
            category("armour.chest")))
    add(f"stat-summed, the mutant: `min` 158. An item of +40 and +117 found here: the site's number is not the sum",
        "the search before it",
        ask([stats("and", [stat(f"explicit.stat_{life}", {"min": 158}), stat(f"fractured.stat_{life}", {"min": 117, "max": 117})])],
            category("armour.chest")))

    # 3. A scourge line in a weighted sum (e043, f005).
    fire = "3372524247"  # `+#% to Fire Resistance`
    add(f"scourge-in-sum, the plain filter: `scourge.stat_{fire}` with `max` -1. Found: a plain filter reads the "
        "scourge downside below nothing (as e043's `-28% to Fire Resistance`)",
        "t23, the scourge id with `min` 1, must find items",
        ask([stats("and", [stat(f"scourge.stat_{fire}", {"max": -1})])]))
    add(f"scourge-in-sum, the weighted group: `scourge.stat_{fire}` alone in a `weight2` with `max` -1. None found "
        "beside the search before it finding: a weighted sum leaves the scourge downside out, as e043's 28 over "
        "+28 and -28 says. Found: it counts it, and e043's 28 is another reading",
        "the search before it",
        ask([stats("weight2", [stat(f"scourge.stat_{fire}", {"weight": 1})], {"max": -1})]))
    add(f"scourge-in-sum's control: `scourge.stat_{fire}` with `min` 1, the upside",
        "the control",
        ask([stats("and", [stat(f"scourge.stat_{fire}", {"min": 1})])]))

    # 4. A `not` with a value (K6).
    m = "t25"
    add(f"a `not` with a value: rings under `explicit.stat_{life}` and a `not` of the same id with `min` 100. Found, "
        "the ten carrying Life under 100: the `not` excludes the bound's meeting, never the presence. None found, "
        f"{m} finding: it excludes the stat's presence",
        f"{m}, the `not` at `min` 1000, must find rings",
        ask([stats("and", [stat(f"explicit.stat_{life}")]), stats("not", [stat(f"explicit.stat_{life}", {"min": 100})])],
            category("accessory.ring")))
    add(f"the `not` with a value, a bound no ring meets (`min` 100 in the search before is now 1000)",
        "the control of the search before it",
        ask([stats("and", [stat(f"explicit.stat_{life}")]), stats("not", [stat(f"explicit.stat_{life}", {"min": 1000})])],
            category("accessory.ring")))
    add("the `not` with a value's own control: rings under the id alone",
        "the control",
        ask([stats("and", [stat(f"explicit.stat_{life}")])], category("accessory.ring")))

    # 5. Which number a text compares: its `#` of several, or a digit it writes (K6; `stat.text.many`, `stat.text.numbers`).
    ground = "explicit.stat_3595254837"  # `Drops Burning Ground while moving, dealing # Fire Damage per second for # second`
    add(f"which `#` is compared: {ground}, `Drops Burning Ground while moving, dealing # Fire Damage per second for # "
        "second`, with `min` 1000. Found, t28 none: the first `#`, the damage (the captures' Titan Greaves: 2500 "
        "for 4 seconds)",
        "t29, the id with no bound, must find items",
        ask([stats("and", [stat(ground, {"min": 1000})])]))
    add("which `#` is compared, the mutant: `min` 3 and `max` 5. Found, the search before none: the second, the "
        "seconds. Both found: neither alone",
        "t29, the id with no bound",
        ask([stats("and", [stat(ground, {"min": 3, "max": 5})])]))
    add(f"the control: {ground} with no bound; the ten fetched show both numbers",
        "the control",
        ask([stats("and", [stat(ground)])]))
    caged = "explicit.stat_2594215131"  # `#% increased Physical Weapon Damage per 10 Strength`
    add(f"a digit the text writes: {caged}, `#% increased Physical Weapon Damage per 10 Strength` (Pillar of the "
        "Caged God, 16 on the captures), with `min` 15. Found, t31 none: the bound compares the text's `#`, as "
        "the catalogue reads it (`stat.text.numbers`). None, the next found: it compares the 10",
        "t32, the id with no bound, must find items",
        ask([stats("and", [stat(caged, {"min": 15})])]))
    add("a digit the text writes, the mutant: `max` 12",
        "the search before it",
        ask([stats("and", [stat(caged, {"max": 12})])]))
    add(f"the control: {caged} with no bound; the ten fetched show the line's numbers",
        "the control",
        ask([stats("and", [stat(caged)])]))

    # 6. Two ids of one text (S66; q2, q2b), and an id of two texts (`stat.text.two`).
    add("q2b's id, `explicit.stat_492027537` (`+#% chance to Suppress Spell Damage`, as q2's), on any item: q2b "
        "found none on boots. None here: the id is never filed, and the text is q2's id alone. Found: the ten show "
        "where",
        "t34, q2's id on any item, finds items",
        ask([stats("and", [stat("explicit.stat_492027537")])]))
    add("q2's id, `explicit.stat_3680664274`, on any item: the control of the search before it",
        "the control",
        ask([stats("and", [stat("explicit.stat_3680664274")])]))
    for sid, texts in (("explicit.stat_124877078", "`Unique Boss deals #% increased Damage` and `Map Bosses deal #% increased Damage`"),
                       ("explicit.stat_3126771445", "`#% increased number of Rare Monsters` and `#% increased number of Rare Monsters in your Maps`")):
        add(f"an id of two texts: {sid}, listed as {texts}; read the ten fetched — which wording each displays, and "
            "on what. Both wordings among them: the id holds both, and the translation may write it as either. One: "
            "the other is another item's",
            "the search finds items, or the id is never filed",
            ask([stats("and", [stat(sid)])]))

    # 7. A group with no bound (the catalogue: unsupported, the site's reading unasked).
    res = "explicit.stat_4220027924"  # `+#% to Cold Resistance`
    add(f"a `count` with no bound: rings under a `count` of `explicit.stat_{life}` and {res}, no `min` or `max`. "
        "Read the ten: every one carrying both, the count is at least the number of its filters; one with either, "
        "at least one; any ring, no restriction",
        "t39, the same rings under the two in an `and`",
        ask([stats("count", [stat(f"explicit.stat_{life}"), stat(res)])], category("accessory.ring")))
    add(f"a weighted sum with no bound: the two in a `weight2`, each weight 1, no `min` or `max`; read the ten as "
        "the search before it",
        "t39, the same rings under the two in an `and`",
        ask([stats("weight2", [stat(f"explicit.stat_{life}", {"weight": 1}), stat(res, {"weight": 1})])],
            category("accessory.ring")))
    add("the two in an `and`: rings carrying both, the control of the two before",
        "the control",
        ask([stats("and", [stat(f"explicit.stat_{life}"), stat(res)])], category("accessory.ring")))
    add("rings with no stat: what the site lists first, beside the three before",
        "the control",
        ask([], category("accessory.ring")))

    # 8. The property filters, the free text and the marks (the catalogue's rows the sitting settles).
    add("the site's dps and quality: two-handed melee weapons, `weapon_filters.pdps` `min` 400. Read the ten: one "
        "whose displayed Physical Damage at its own quality gives under 400, and at 20 % 400 or more: the site's "
        "figure puts 20 % quality in. Every one 400 or more as displayed: as displayed, or every one at 20 %",
        "the ten fetched carry their own quality and damage",
        ask([], merge(category("weapon.twomelee"), filt("weapon_filters", pdps={"min": 400}))))
    add("as the search before, `weapon_filters.dps` `min` 500 over the elemental and the physical",
        "the ten fetched carry their own quality and damage",
        ask([], merge(category("weapon.twomelee"), filt("weapon_filters", dps={"min": 500}))))
    add("the site's damage filter, `weapon_filters.damage` `min` 200 on bows: read the ten — which displayed "
        "damages the site's figure sums (physical, elemental, chaos; the average or the top)",
        "the ten fetched carry their damage",
        ask([], merge(category("weapon.bow"), filt("weapon_filters", damage={"min": 200}))))
    add("`weapon_filters.crit` on a gem: skill gems with `crit` `min` 5. Found: the weapon's filter reads a gem's "
        "displayed `Critical Strike Chance` (the catalogue's reading); none, the next found: it does not",
        "t45, skill gems with no filter, finds gems",
        ask([], merge(category("gem.activegem"), filt("weapon_filters", crit={"min": 5}))))
    add("skill gems with no filter: the control of the search before",
        "the control",
        ask([], category("gem.activegem")))
    add("`map_filters.area_level` on a vessel: `Blood-filled Vessel` with `area_level` `min` 1. Found: the filter "
        "reads the vessel's `Monster Level`; none, the next found: it does not",
        "t47, the vessel with no filter",
        ask([], filt("map_filters", area_level={"min": 1}), "Blood-filled Vessel"))
    add("`Blood-filled Vessel` with no filter: the control of the search before",
        "the control",
        ask([], None, "Blood-filled Vessel"))
    add("the heist skills on a brooch or a cloak: heist equipment with `heist_lockpicking` `min` 1. Read the ten: "
        "a brooch or cloak requiring `Level n in Lockpicking` among them: the filter reads that shape too",
        "the ten fetched carry their requirements",
        ask([], merge(category("heistequipment"), filt("heist_filters", heist_lockpicking={"min": 1}))))
    add("`misc_filters.quality` on a catalysed ring: rings with `quality` `min` 1. Read the ten: a ring whose only "
        "quality is `Quality (Resistance Modifiers)` or another of its kind: the filter folds them in",
        "the ten fetched carry their properties",
        ask([], merge(category("accessory.ring"), filt("misc_filters", quality={"min": 1}))))
    add("the free text: `term` `Anvil`. Read the ten: The Anvil by its name, and any base or other name holding the "
        "word: what the box matches",
        "t51, the same mid-word",
        ask([], term="Anvil"))
    add("the free text, mid-word: `term` `nvil`. Found The Anvil: the box is a contains test, as the catalogue "
        "writes it (`name:` or `typeline:`); none: a word or a prefix",
        "the search before it finds The Anvil",
        ask([], term="nvil"))
    add("`rarity` `uniquefoil`: read the ten — whether each carries the body's relic mark (`isRelic`) and what "
        "else sets it apart",
        "t53, `rarity` `unique`",
        ask([], filt("type_filters", rarity={"option": "uniquefoil"})))
    add("`rarity` `unique`: the control of the search before — what the ten of a plain unique carry",
        "the control",
        ask([], filt("type_filters", rarity={"option": "unique"})))
    add("`misc_filters.crucible_item` `true`: read the ten — whether each has a crucible tree with a node "
        "allocated, or a tree alone",
        "the ten fetched carry their tree",
        ask([], filt("misc_filters", crucible_item={"option": "true"})))
    add("`map_filters.map_blighted` `true` on `Map (Tier 13)`: read the ten — each typeline's `Blighted` prefix, "
        "and whether any is not; with the next, whether the filter is the prefix or a mark",
        "t56, the same base with `map_blighted` `false`",
        ask([], filt("map_filters", map_blighted={"option": "true"}), "Map (Tier 13)"))
    add("the same base with `map_blighted` `false`: none with the prefix among the ten, beside the search before",
        "the search before it",
        ask([], filt("map_filters", map_blighted={"option": "false"}), "Map (Tier 13)"))
    # t12's control, added after the round was numbered
    add(f"`an` for one, the control of t12: {pierce} with no bound; the ten fetched show the line's forms",
        "the control",
        ask([stats("and", [stat(pierce)])]))
    # every name above is written out: the round is its length
    if len(rows) != 57:
        raise SystemExit(f"round t has {len(rows)} searches, its names written for 57")
    return rows


class Composed:
    """A round's rows, composed against every row before it: a query the
    sheet already holds is not composed again — the recorder keeps a link
    under one name, so a second row of it would go unread — and its search
    is the earlier one, a question of this round merged into that row's
    `decides`. A `{next}` in a row names the next search composed, whatever
    its name, a repeat's earlier one included."""

    def __init__(self, letter, width, earlier):
        self.letter, self.width, self.rows = letter, width, []
        self.known = {json.dumps(r["query"], sort_keys=True): r["search"] for r in earlier}
        self.mine = {}
        self.waiting = []

    def add(self, decides, control, q):
        key = json.dumps(q, sort_keys=True)
        if key in self.known:
            name = self.known[key]
            if name in self.mine:
                row = self.mine[name]
                row["decides"] += f"; also: {decides}"
        else:
            name = f"{self.letter}{len(self.rows) + 1:0{self.width}d}"
            row = {"search": name, "decides": decides, "control": control, "query": q}
            self.rows.append(row)
            self.known[key] = name
            self.mine[name] = row
        for waiting in self.waiting:
            for field in ("decides", "control"):
                waiting[field] = waiting[field].replace("{next}", name)
        self.waiting = []
        if name in self.mine and "{next}" in self.mine[name]["decides"] + self.mine[name]["control"]:
            self.waiting.append(self.mine[name])
        return name

    def done(self):
        if self.waiting:
            raise SystemExit(f"{self.waiting[0]['search']} names a next search none follows")
        return self.rows


# Round u (the trade import, step 7): coverage. The oracle (step 6) checks
# the translation only on the clauses the captured searches used: measured
# at step 7a, 64 of the catalogue's 94 rows that run and 53 of its 82
# category options had met no item the site returned, so their status
# rested on hand-written fixtures alone. A search here exercises each, with
# a bound that restricts where the row takes one; nothing is decided by
# reading it — at step 7d the oracle asks the translated text of every item
# returned, and an item it misses is a row. Rows round t already exercises
# are left to it.
COVERAGE_CATEGORIES = [
    "weapon.bow", "weapon.claw", "weapon.basedagger", "weapon.oneaxe", "weapon.basesword", "weapon.rapier",
    "weapon.basemace", "weapon.twoaxe", "weapon.twomace", "weapon.twosword", "armour.gloves", "armour.shield",
    "armour.quiver", "accessory", "accessory.belt", "accessory.trinket", "gem", "gem.activegem", "gem.supportgem",
    "gem.supportgemplus", "jewel", "jewel.base", "jewel.abyss", "jewel.cluster", "map.fragment", "map.invitation",
    "leaguestone", "memoryline", "card", "heistequipment", "heistequipment.heistweapon", "heistequipment.heisttool",
    "heistequipment.heistutility", "heistequipment.heistreward", "heistmission", "heistmission.contract",
    "heistmission.blueprint", "logbook", "sanctum.research", "sanctum.relic", "tincture", "corpse", "idol", "graft",
    "wombgift", "enshrouded", "chart", "currency.piece", "currency.resonator", "currency.fossil",
    "currency.incubator", "currency.heistobjective", "currency.omen",
]
# (row, category or None, filter group, filter, value): the bounds are the
# builder's, each one most listings of the category fail and some meet.
COVERAGE_ROWS = [
    ("type_filters.rarity=nonunique", "armour.helmet", "type_filters", "rarity", {"option": "nonunique"}),
    ("weapon_filters.aps", "weapon.claw", "weapon_filters", "aps", {"min": 1.7}),
    ("weapon_filters.edps", "weapon.bow", "weapon_filters", "edps", {"min": 200}),
    ("armour_filters.ar", "armour.chest", "armour_filters", "ar", {"min": 1500}),
    ("armour_filters.ev", "armour.chest", "armour_filters", "ev", {"min": 1500}),
    ("armour_filters.es", "armour.chest", "armour_filters", "es", {"min": 400}),
    ("armour_filters.block", "armour.shield", "armour_filters", "block", {"min": 28}),
    ("socket_filters.sockets", "armour.chest", "socket_filters", "sockets", {"min": 6, "r": 2}),
    ("socket_filters.links", "armour.chest", "socket_filters", "links", {"min": 6}),
    ("req_filters.lvl", "weapon.onesword", "req_filters", "lvl", {"max": 20}),
    ("req_filters.str", "armour.chest", "req_filters", "str", {"min": 150}),
    ("req_filters.dex", "armour.boots", "req_filters", "dex", {"min": 150}),
    ("req_filters.int", "armour.helmet", "req_filters", "int", {"min": 150}),
    ("map_filters.map_tier", "map", "map_filters", "map_tier", {"min": 16}),
    ("map_filters.map_packsize", "map", "map_filters", "map_packsize", {"min": 30}),
    ("map_filters.map_iiq", "map", "map_filters", "map_iiq", {"min": 80}),
    ("map_filters.map_iir", "map", "map_filters", "map_iir", {"min": 80}),
    ("heist_filters.heist_wings", "heistmission.blueprint", "heist_filters", "heist_wings", {"min": 2}),
    ("heist_filters.heist_max_wings", "heistmission.blueprint", "heist_filters", "heist_max_wings", {"min": 3}),
    ("heist_filters.heist_escape_routes", "heistmission.blueprint", "heist_filters", "heist_escape_routes", {"min": 2}),
    ("heist_filters.heist_max_escape_routes", "heistmission.blueprint", "heist_filters", "heist_max_escape_routes", {"min": 3}),
    ("heist_filters.heist_reward_rooms", "heistmission.blueprint", "heist_filters", "heist_reward_rooms", {"min": 2}),
    ("heist_filters.heist_max_reward_rooms", "heistmission.blueprint", "heist_filters", "heist_max_reward_rooms", {"min": 3}),
    ("heist_filters.heist_brute_force", "heistmission.contract", "heist_filters", "heist_brute_force", {"min": 3}),
    ("heist_filters.heist_perception", "heistmission.contract", "heist_filters", "heist_perception", {"min": 3}),
    ("heist_filters.heist_demolition", "heistmission.contract", "heist_filters", "heist_demolition", {"min": 3}),
    ("heist_filters.heist_counter_thaumaturgy", "heistmission.contract", "heist_filters", "heist_counter_thaumaturgy", {"min": 3}),
    ("heist_filters.heist_trap_disarmament", "heistmission.contract", "heist_filters", "heist_trap_disarmament", {"min": 3}),
    ("heist_filters.heist_agility", "heistmission.contract", "heist_filters", "heist_agility", {"min": 3}),
    ("heist_filters.heist_deception", "heistmission.contract", "heist_filters", "heist_deception", {"min": 3}),
    ("heist_filters.heist_engineering", "heistmission.contract", "heist_filters", "heist_engineering", {"min": 3}),
    ("heist_filters.heist_lockpicking", "heistmission.contract", "heist_filters", "heist_lockpicking", {"min": 3}),
    ("sanctum_filters.sanctum_resolve", "sanctum.research", "sanctum_filters", "sanctum_resolve", {"min": 40}),
    ("sanctum_filters.sanctum_max_resolve", "sanctum.research", "sanctum_filters", "sanctum_max_resolve", {"min": 60}),
    ("sanctum_filters.sanctum_inspiration", "sanctum.research", "sanctum_filters", "sanctum_inspiration", {"min": 10}),
    ("sanctum_filters.sanctum_gold", "sanctum.research", "sanctum_filters", "sanctum_gold", {"min": 100}),
    ("misc_filters.ilvl", "armour.helmet", "misc_filters", "ilvl", {"min": 86}),
    ("misc_filters.gem_level", "gem.activegem", "misc_filters", "gem_level", {"min": 20}),
    ("misc_filters.gem_level_progress", "gem.activegem", "misc_filters", "gem_level_progress", {"min": 50}),
    ("misc_filters.gem_vaal", "gem.activegem", "misc_filters", "gem_vaal", {"option": "true"}),
    ("misc_filters.gem_vaal=false", "gem.activegem", "misc_filters", "gem_vaal", {"option": "false"}),
    ("misc_filters.fractured_item", "armour.boots", "misc_filters", "fractured_item", {"option": "true"}),
    ("misc_filters.synthesised_item", "accessory.ring", "misc_filters", "synthesised_item", {"option": "true"}),
    ("misc_filters.searing_item", "armour.helmet", "misc_filters", "searing_item", {"option": "true"}),
    ("misc_filters.tangled_item", "armour.gloves", "misc_filters", "tangled_item", {"option": "true"}),
    ("misc_filters.identified=false", "accessory.ring", "misc_filters", "identified", {"option": "false"}),
    ("misc_filters.corrupted", "jewel", "misc_filters", "corrupted", {"option": "true"}),
    ("misc_filters.corrupted=false", "jewel", "misc_filters", "corrupted", {"option": "false"}),
    ("misc_filters.split", None, "misc_filters", "split", {"option": "true"}),
    ("misc_filters.crafted", "weapon", "misc_filters", "crafted", {"option": "true"}),
    ("misc_filters.veiled", None, "misc_filters", "veiled", {"option": "true"}),
    ("misc_filters.vestigial", None, "misc_filters", "vestigial", {"option": "true"}),
    ("misc_filters.memory_level", "memoryline", "misc_filters", "memory_level", {"min": 2}),
    ("misc_filters.stored_experience", None, "misc_filters", "stored_experience", {"min": 1}),
    ("misc_filters.stack_size", "currency", "misc_filters", "stack_size", {"min": 10}),
    ("misc_filters.mutated", None, "misc_filters", "mutated", {"option": "true"}),
]


def round_u(earlier):
    """A category alone, each no capture reached; then each filter row no
    capture used, under a category that lists it. A search the sheet holds
    already is that one (`Composed`)."""
    composed = Composed("u", 3, earlier)

    def add(decides, q):
        composed.add(decides, "the oracle (tests/trade_oracle.rs): every item returned is matched by the translated "
                     "text, or a row", q)

    for option in COVERAGE_CATEGORIES:
        add(f"coverage, `type_filters.category={option}`: the category alone, which no captured search used",
            batch_query([], category(option)))
    for row, option, group_id, name, value in COVERAGE_ROWS:
        filters = dict(category(option)) if option else {}
        if group_id == "type_filters":
            filters["type_filters"]["filters"][name] = value
        else:
            filters[group_id] = {"disabled": False, "filters": {name: value}}
        where = f" under `{option}`" if option else ""
        add(f"coverage, `{row}` at {json.dumps(value)}{where}: a row no captured search used",
            batch_query([], filters))
    return composed.done()


# Round v (step 7, the park "what no sitting has asked the site",
# decisions/search.md): the questions the search's tracks left to the next
# sitting, each where its track wrote it — the ranged family's and the
# percentile's in their copy files' `a_sitting_would_ask`, the four
# readings' and the name rules' in the README entries the park names
# (search/OBSERVATIONS.md at 453a4429). Each is read by its track's own
# script over the captures at step 7d; a search here composes the question
# and names what the fetched items show.
RANGED_COPY = TRACK / "data" / "ranged-expected-copy.csv"
COUNT_AT_MOST = 64
PERCENTILE_COPY = TRACK / "data" / "percentile-expected-copy.csv"
# A category the site answers for an item that is no weapon: a line `to
# Attacks` or `to Spells` is an accessory's, an armour's or a jewel's, and a
# plain one off a weapon is the accessory's implicit the copy holds.
NOT_A_WEAPON = "accessory"
PERCENTILE_MARKS = {
    "neither": {"synthesised_item": {"option": "false"}, "searing_item": {"option": "false"},
                "tangled_item": {"option": "false"}},
    "eldritch": {"searing_item": {"option": "true"}},
    "synthesised": {"synthesised_item": {"option": "true"}},
}
# The name rules (categories-v1.toml, `rule`): each leaf with a base its rule
# places, the tattoo's asked at round m. The invitation's is The Atlas: the
# category README's Elderslayers is in no list the site gave (items, 2026-09-12).
NAME_RULES = [
    ("gem.supportgemplus", "Awakened Chain Support", "starts `Awakened `"),
    ("jewel.cluster", "Large Cluster Jewel", "ends `Cluster Jewel`"),
    ("map.invitation", "Maven's Invitation: The Atlas", "starts `Maven's Invitation`"),
    ("map.scarab", "Ritual Scarab of Abundance", "holds `Scarab`, here inside the name"),
    ("currency.fossil", "Dense Fossil", "ends ` Fossil`"),
    ("currency.omen", "Omen of Amelioration", "starts `Omen of `"),
]
# The category README's "one search each would settle" (category, type).
CATEGORY_SETTLES = [
    ("map", "Polaric Invitation", "J3: an invitation not the Maven's takes `map`"),
    ("map", "Voidborn Reliquary Key", "a key the export classes otherwise under `map`"),
    ("currency", "Chaos Orb", "`currency` holds the plainest currency"),
    ("currency", "Dense Fossil", "`currency` holds a fossil beside `currency.fossil`"),
    ("graft", "Battering Uulgraft", "J13: a graft under `graft`"),
    ("weapon.one", "Imperial Claw", "a claw under the one-handed grouping"),
]


def copy_questions(path):
    rows = [r for r in csv.reader(path.open(newline="")) if r and not r[0].startswith("#")]
    header = rows[0]
    return [dict(zip(header, r)) for r in rows[1:] if dict(zip(header, r)).get("a_sitting_would_ask")]


def site_ids(template, domain):
    """The ids of a template under one domain, the `+` spelling the site writes tried too."""
    out = []
    for text in (template, "+" + template):
        try:
            out += [i for i in ids_of(text) if i.startswith(domain + ".")]
        except SystemExit:
            pass
    return out


def crucible_nodes(template):
    found = []
    for g in json.loads(STATS.read_text())["result"]:
        if g["id"] != "crucible":
            continue
        for e in g["entries"]:
            lines = re.sub(r" \(Tier \d+\)$", "", e["text"]).split("\n")
            if template in [re.sub(r"\d+(?:\.\d+)?", "#", line) for line in lines]:
                found.append(e["id"])
    return found


def round_v(earlier):
    """v001 on: the ranged family's questions (ranged-expected-copy.csv), the
    percentile's (percentile-expected-copy.csv), the four readings' three
    unasked cases, the name rules with their pairs, and the category
    README's settling searches with theirs."""
    composed = Composed("v", 3, earlier)
    add = composed.add

    # 1. The ranged family: does the pseudo show for a line of this array, flag and side.
    for d in copy_questions(RANGED_COPY):
        template, side, array, flag = d["total_or_template"], d["status_or_side"], d["cause_or_array"], d["flags"]
        pseudos = sorted(set(re.findall(r"pseudo\.pseudo_[a-z_]+", d["a_sitting_would_ask"])))
        if array == "crucibleMods":
            ids = crucible_nodes(template)
        else:
            domain = flag or {"implicitMods": "implicit", "enchantMods": "enchant", "explicitMods": "explicit"}[array]
            ids = site_ids(template, domain)
            if side == "weapon":
                ids += site_ids(template + " (Local)", domain)
        if not ids or not pseudos:
            raise SystemExit(f"the ranged question on {template!r} ({side}, {array}, {flag}) names no id or no pseudo")
        where = category("weapon") if side == "weapon" else (
            None if (" to Attacks" in template or " to Spells" in template) else category(NOT_A_WEAPON))
        # a count no larger than the largest the site has answered (64 ids, n03–n05)
        parts = [ids[i:i + COUNT_AT_MOST] for i in range(0, len(ids), COUNT_AT_MOST)]
        for n, part in enumerate(parts, 1):
            of = f", part {n} of {len(parts)}" if len(parts) > 1 else ""
            add(f"the ranged family ({RANGED_COPY.name}): {d['a_sitting_would_ask']} The ten fetched carry the line "
                f"({len(part)} ids{of}, a count) and show the pseudos named (`if`) or none",
                "read by scripts/ranged-expected.py over the captures, cell by cell (`captured_cells`)",
                batch_query([group("count", part, {"min": 1}), group("if", pseudos)], where))

    # 2. The percentile: does the site show the reading's value for this group of defence lines.
    for d in copy_questions(PERCENTILE_COPY):
        template, array, flag, mark = (d["class_or_template_or_shape"], d["array"], d["flags"], d["mark"])
        if not array:
            continue  # the roll cases, composed below
        domain = flag if flag in ("crafted", "fractured") else {"implicitMods": "implicit", "explicitMods": "explicit"}[array]
        # an armour piece's own defence line is the local stat, its twin the global
        ids = site_ids(template, domain) + site_ids(template + " (Local)", domain)
        if not ids:
            raise SystemExit(f"the percentile question on {template!r} names no id")
        filters = dict(category("armour"))
        marks = dict(PERCENTILE_MARKS[mark])
        if flag == "mutated":
            marks["mutated"] = {"option": "true"}
        filters["misc_filters"] = {"disabled": False, "filters": marks}
        add(f"the percentile ({PERCENTILE_COPY.name}): {d['a_sitting_would_ask']} The ten fetched carry the line "
            f"({len(ids)} ids, a count) and the site's `base_defence_percentile`",
            "read by scripts/percentile-expected.py over the captures, cell by cell",
            batch_query([group("count", ids, {"min": 1})], filters))
    add("the percentile, a roll over the base's maximum (C10; J34): armour with `base_defence_percentile` `min` "
        "101. Found: the site shows over 100 (i03 found none listed), and the ten say which display gives it",
        "a found item is its own evidence; none found leaves C10 where i03 did",
        batch_query([], {**category("armour"), **percentile_filter(101)}))
    add("the percentile, a roll under the base's minimum (S3): armour with `base_defence_percentile` `max` -1. "
        "Found: what the site shows of such an item",
        "a found item is its own evidence",
        batch_query([], {**category("armour"),
                         "armour_filters": {"disabled": False, "filters": {"base_defence_percentile": {"max": -1}}}}))

    # 3. The four readings: the cases no sitting asked (search/OBSERVATIONS.md at 453a4429, "since 9c").
    totals = ["pseudo.pseudo_total_fire_resistance", "pseudo.pseudo_total_cold_resistance",
              "pseudo.pseudo_total_lightning_resistance", "pseudo.pseudo_total_chaos_resistance"]
    counts = ["pseudo.pseudo_count_resistances", "pseudo.pseudo_count_elemental_resistances"]
    least_res, least_attr = "pseudo.pseudo_total_all_elemental_resistances", "pseudo.pseudo_total_all_attributes"
    add("the readings: whether an item showing all four resistance totals ever shows no "
        "`pseudo_count_resistances`. None found, {next} finding: never",
        "{next}, the count shown (`if`) where this refuses it, must find items",
        batch_query([group("and", totals), group("not", [counts[0]])]))
    add("the readings' mutant: the four totals required, the count shown (`if`)",
        "the mutant is the control",
        batch_query([group("and", totals), group("if", [counts[0]])]))
    add("the readings: the two counts of a ring showing none of the four totals — rings carrying no resistance "
        "total, the counts shown (`if`): read the ten, a count of 0 or none",
        "the ten fetched show the counts, or nothing",
        batch_query([group("not", totals), group("if", counts)], category("accessory.ring")))
    add("the readings: the elemental least of an item showing one of its three totals — fire required, cold and "
        "lightning refused, the least shown (`if`): read the ten, a value or none",
        "the ten fetched show it, or nothing",
        batch_query([group("and", totals[:1]), group("not", totals[1:3]), group("if", [least_res])]))
    add("the readings: the elemental least of an item showing two — fire and cold required, lightning refused, "
        "the least shown (`if`)",
        "the ten fetched show it, or nothing",
        batch_query([group("and", totals[:2]), group("not", totals[2:3]), group("if", [least_res])]))
    add("the readings: the attributes' least of an item showing one or two of its totals — Strength required, "
        "Dexterity refused, the least shown (`if`)",
        "the ten fetched show it, or nothing",
        batch_query([group("and", ["pseudo.pseudo_total_strength"]), group("not", ["pseudo.pseudo_total_dexterity"]),
                     group("if", [least_attr])]))

    # 4. The name rules, each with its pair (round m's form).
    for option, base, rule in NAME_RULES:
        add(f"the name rule of `{option}` ({rule}): whether the leaf holds `{base}`. Found: the rule places it as the "
            "site does. Not found, its pair finding it: the rule is too wide for this name",
            "{next}, the type with no category, must find it",
            batch_query([], category(option), base))
        add(f"the pair: `{base}` with no category — that it is listed at all",
            "the pair is the control",
            batch_query([], None, base))
    # 5. The category README's settling searches, each with its pair.
    for option, base, why in CATEGORY_SETTLES:
        add(f"the category track: whether `{option}` holds `{base}` ({why}). Found: as the table rules; not found, "
            "its pair finding it: the table is brought back",
            "{next}, the type with no category, must find it",
            batch_query([], category(option), base))
        add(f"the pair: `{base}` with no category", "the pair is the control", batch_query([], None, base))
    return composed.done()


def report():
    """b1, b2: the two searches a report of c3's twin id rests on."""
    twin = "explicit.stat_2543977012"
    return [
        {
            "search": "b1",
            "decides": (
                "the report: `+# to Strength and Intelligence` under the id That Which Was "
                "Taken carries, with `+# total maximum Life` required. c3 says none will be found"
            ),
            "control": "b2, the line alone, finds the items b1 should have",
            "query": batch_query([group("and", [twin, "pseudo.pseudo_total_life"])]),
        },
        {
            "search": "b2",
            "decides": "the report's control: the same line with no total asked, which finds the jewels",
            "control": "c3 fetched five of them",
            "query": batch_query([group("and", [twin])]),
        },
    ]


def decode(link_text):
    token = link_text.rsplit("/", 1)[1]
    raw = base64.urlsafe_b64decode(token + "=" * (-len(token) % 4))
    return json.loads(zlib.decompress(raw, 16 + zlib.MAX_WBITS))


def verify():
    bad = 0
    with OUT.open(newline="") as fh:
        rows = list(csv.DictReader(fh))
    for row in rows:
        if decode(row["link"]) != json.loads(row["query"]):
            bad += 1
            print(f"DIFFER  {row['search']}")
        if not row["control"].strip():
            bad += 1
            print(f"NO CONTROL  {row['search']}")
    print(f"{len(rows)} links decoded, {bad} differ from their query or lack a control")
    return 1 if bad else 0


def self_test():
    requests = sorted(CAPTURED.glob("*-request.json"))
    if not requests:
        print(f"skipped: no captures under {CAPTURED} (raw/ is local only)")
        return 0
    failed = 0
    checked = 0
    for request in requests:
        answer = request.with_name(request.name.replace("-request", "-search"))
        if not answer.exists() or answer.stat().st_size == 0:
            continue
        want = json.loads(answer.read_text())["id"]
        got = compose(json.loads(request.read_text())["query"])
        checked += 1
        if got != want:
            failed += 1
            print(f"DIFFER  {request.name}")
    print(f"{checked} captured ids composed, {failed} differ")
    return 1 if failed else 0


def main():
    args = sys.argv[1:]
    if args == ["--self-test"]:
        return self_test()
    if args == ["--verify"]:
        return verify()
    if args == []:
        method = "site"
    elif args == ["--method", "if"]:
        method = "if"
    elif args == ["--method", "and"]:
        method = "and"
    else:
        print(__doc__)
        return 2
    searches = PILOT + (checks() + round_two() + report() + round_three() + round_four()
                        + round_five() + OPEN_QUESTION_SEARCHES + round_six() + round_i() + round_j() + round_k() + round_l() + round_m()
                        + round_n() + round_t()
                        if method == "site" else batch(method))
    if method == "site":
        searches += round_u(searches)
        searches += round_v(searches)
    seen = set()
    with OUT.open("w", newline="") as out:
        sheet = csv.writer(out, lineterminator="\n")
        sheet.writerow(["search", "decides", "control", "league", "link", "query"])
        for row in searches:
            if row["search"] in seen:
                raise SystemExit(f"two searches named {row['search']}")
            seen.add(row["search"])
            body = json.dumps(row["query"], separators=(",", ":"), ensure_ascii=False)
            sheet.writerow(
                [
                    row["search"],
                    row["decides"],
                    row["control"],
                    LEAGUE,
                    link(row["query"]),
                    body,
                ]
            )
            if method == "site":
                print(f"{row['search']}  {link(row['query'])}")
    kinds = {"trade": 0, "park": 0, "pilot": 0, "check": 0, "line": 0, "pair": 0, "percentile line": 0,
             "percentile case": 0, "open question": 0}
    for row in searches:
        s = row["search"]
        kind = ("trade" if s.startswith(("t", "u"))
                else "park" if s.startswith("v")
                else "pilot" if s.startswith("p")
                else "check" if s.startswith(("c", "d", "b", "e", "f", "g", "h"))
                else "percentile case" if s.startswith("C")
                else "open question" if s.startswith("R")
                else "percentile line" if s.startswith("P") else "pair" if "." in s else "line")
        kinds[kind] += 1
    print(f"--method {method}: " + ", ".join(f"{n} {k}" for k, n in kinds.items() if n))
    print(f"{len(searches)} searches written to {OUT.relative_to(TRACK.parent.parent)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
