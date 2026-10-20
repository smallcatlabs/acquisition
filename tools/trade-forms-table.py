#!/usr/bin/env python3
"""The game's forms of each trade stat (C113, C106; src/trade/forms.rs): for
every text of the stat table, the other ways the game displays the stat the
site files under that id — the opposite word below nothing, a number left
out, a plural, another wording — as the game's own stat descriptions state
them. Step 7e of search/TRADE-PLAN.md at 77666ea4; the owner, 2026-10-17, choosing "Game
data, rules beside" over the brief's blanket rules, which named forms the
game never displays (160 of the 372 `#% chance to` texts it joins).

    python3 -I tools/trade-forms-table.py            # writes crates/acquisition-search/reference/trade-game-forms-v1.toml
    python3 -I tools/trade-forms-table.py --check    # exits 1 when the file on disk differs from what the sources give

Inputs (read-only; tools/sources.py refuses an absent one or a clone off
its pin, so the table regenerates byte-for-byte where the clone is, and the
gate, which has none, never runs it):
  ../poe1/data/stat_translations.json            the PoE1 export (search/repoe/MANIFEST.md):
                                                  each game stat's English strings, the value
                                                  range each is shown for, how each number is
                                                  written, and the trade ids the export joins
  crates/acquisition-search/reference/trade-stats-v1.toml
                                                  the site's texts by id, which the forms are of

Rules, each a reading of the export and never a judgment (C106):
  1. A text is joined where an entry of the export names one of its ids
     under any hash category; its forms are every English string of every
     such entry, the string's `{i}` written `#`, markup reduced to its
     display half, and none the text's own template (case aside, a `+`
     before a `#` spelling). A text the export joins nowhere has no row,
     and the forms table's rules stand for it (`reference/trade-forms-v1.toml`).
     Not a form: a string that is, or holds as a row, the text of another
     id the same entry names — an entry of one value per id (a timeless
     jewel's nobles) groups other stats, not other displays; a string
     that is not the text's stat by its conditions (`one_stat`: every stat
     the two part on a number one of them shows, a `negated` condition the
     values outside its range) — another value of an enumeration, another
     stat's flag; and a string of several rows one of which is the text,
     which the line already is (C90: a mod over several rows is named by
     any one). A text whose own string the entry does not hold keeps its
     strings, `other`.
  2. An id the stat table lists with two texts is left out: the
     translation leaves it unsupported, and which text a string is of is
     not said.
  3. A form's kind, read in this order: `opposite` where a number of the
     string is shown through `negate` (the game's own below-nothing
     spelling), or the string is the text with its sign words flipped;
     `numberless` where it shows fewer `#`s than the text, its `number` the
     one value its condition allows where it allows one; `plural` where it
     is the text word for word but for a noun's or a verb's number
     (`second`/`seconds`, `Enemy`/`Enemies`, `is`/`are`), its `#`s where
     the text's are and no number written as a digit in place of one;
     `other` beside.
  4. Order: the stat table's, a text at its first id; forms by the
     export's order within an entry, entries by the export's.
"""

import json
import os
import re
import sys
import tomllib

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
sys.path.insert(0, HERE)
from sources import POE1, require  # noqa: E402  the clone, at its pin (search/repoe/MANIFEST.md)
EXPORT = POE1 / "data" / "stat_translations.json"
STATS = os.path.join(ROOT, "crates", "acquisition-search", "reference", "trade-stats-v1.toml")
OUT = os.path.join(ROOT, "crates", "acquisition-search", "reference", "trade-game-forms-v1.toml")
VERSION = 1
PAIRS = {"increased": "reduced", "reduced": "increased", "more": "less", "less": "more"}


def toml_str(s):
    out = s.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n")
    if any(ord(c) < 0x20 or ord(c) == 0x7F for c in out):
        sys.exit(f"{s!r}: a control character this writer does not escape")
    return '"' + out + '"'


NUMBER = re.compile(r"(?<![#\d)])[+-]?\d+(?:,\d{3})*(?:\.\d+)?|\d+(?:,\d{3})*(?:\.\d+)?")


def key(text):
    """A template as the deriver tells two apart: numbers `#`, a sign before
    a `#` dropped — the number's, never the template's (C90) — case aside
    (src/trade/forms.rs, `key`)."""
    t = NUMBER.sub("#", text)
    return re.sub(r"(?<![#\d)])[+-]#", "#", t).lower()


def display(string):
    """An export string as the game displays it: `{i}` a `#`, markup's
    display half (`[Fortify|Fortified]` is `Fortified`)."""
    t = re.sub(r"\{\d+\}", "#", string)
    return re.sub(r"\[([^\]|]*)\|([^\]]*)\]|\[([^\]]*)\]", lambda m: m.group(2) if m.group(2) is not None else m.group(3), t)


def word(w):
    """A word with its number taken off, for rule 3's `plural`."""
    w = w.lower()
    if w in ("is", "are"):
        return "is"
    if w.endswith("ies"):
        return w[:-3] + "y"
    if w.endswith("s") and not w.endswith("ss"):
        return w[:-1]
    return w


def meet(a, b):
    """Whether two conditions on one stat of an entry allow a value in
    common: each a range, `negated` the values outside it."""
    def span(c):
        return (c["min"] if c["min"] is not None else float("-inf"),
                c["max"] if c["max"] is not None else float("inf"),
                bool(c.get("negated")))
    (l1, h1, n1), (l2, h2, n2) = span(a), span(b)
    if not n1 and not n2:
        return max(l1, l2) <= min(h1, h2)
    if n1 and n2:
        return True
    (lo, hi), (other_lo, other_hi) = ((l1, h1), (l2, h2)) if n1 else ((l2, h2), (l1, h1))
    return other_lo < lo or other_hi > hi


def shows(s):
    """The stats of an entry a string displays the number of."""
    return {i for i, f in enumerate(s["format"]) if f != "ignore" and "{%d}" % i in s["string"]}


def one_stat(own, s):
    """Rule 1's identity: two strings of an entry display one stat where
    every stat their conditions part on is a number one of them shows —
    `reduced` beside `increased`, a number left out at a bound of it, a
    plural — and not where they part on a stat neither shows: another value
    of an enumeration (a map's league mechanic), another stat's flag (a
    warcry's infinite Power beside its counting)."""
    parted = {i for i, (a, b) in enumerate(zip(own["condition"], s["condition"])) if not meet(a, b)}
    return parted <= (shows(own) | shows(s))


def kind_of(text, form, shown_negated, count):
    """Rule 3."""
    if shown_negated or re.sub(r"\b(increased|reduced|more|less)\b", lambda m: PAIRS[m.group(1)], key(text)) == key(form):
        return "opposite"
    if count(form) < count(text):
        return "numberless"
    a, b = key(text).split(), key(form).split()
    # the same `#`s the site leaves open, not a number written as a digit
    same = text.count("#") == form.count("#")
    if same and len(a) == len(b) and [word(w) for w in a] == [word(w) for w in b]:
        return "plural"
    return "other"


def main():
    check = "--check" in sys.argv[1:]
    require(EXPORT)
    export = json.loads(EXPORT.read_text())
    stats = tomllib.load(open(STATS, "rb"))
    by_id = {}
    for at, entry in enumerate(export):
        for ts in entry.get("trade_stats") or []:
            by_id.setdefault(ts["id"].split(".", 1)[1], []).append(at)
    count = lambda t: key(t).count("#")
    rows = []
    seen = set()
    joined = with_forms = forms_total = 0
    kinds = {"opposite": 0, "numberless": 0, "plural": 0, "other": 0}
    for stat in stats["stat"]:
        if len(stat["texts"]) != 1:
            continue
        site = stat["texts"][0]
        text = site[: -len(" (Local)")] if site.endswith(" (Local)") else site
        if text in seen:
            # a text at its first id, its forms every entry of every id
            continue
        ids = [s["id"] for s in stats["stat"] if s["texts"] == [site] or s["texts"] == [text]]
        entries = sorted({at for i in ids for at in by_id.get(i, [])})
        if not entries:
            continue
        seen.add(text)
        joined += 1
        forms = []
        keys = {key(text)}
        for at in entries:
            english = export[at].get("English") or []
            others = {key(ts["text"]) for ts in export[at].get("trade_stats") or []
                      if ts["id"].split(".", 1)[1] not in ids}
            own = [x for x in english if key(display(x["string"])) == key(text)]
            for s in english:
                form = display(s["string"])
                rows_of = [key(row) for row in form.split("\n")]
                if key(form) in keys or others.intersection(rows_of + [key(form)]):
                    continue
                if len(rows_of) > 1 and key(text) in rows_of:
                    continue
                if own and not any(one_stat(o, s) for o in own):
                    continue
                keys.add(key(form))
                negated = any(h.startswith("negate") for hs in s["index_handlers"] for h in hs)
                kind = kind_of(text, form, negated, count)
                number = None
                if kind == "numberless":
                    # the one index the form leaves out, where its condition
                    # allows one value
                    own = [x for x in english if key(display(x["string"])) == key(text)]
                    if own:
                        left = [i for i, (f, g) in enumerate(zip(own[0]["format"], s["format"])) if f != "ignore" and g == "ignore"]
                        if len(left) == 1:
                            c = s["condition"][left[0]]
                            if c["min"] is not None and c["min"] == c["max"]:
                                number = c["min"]
                forms.append((form, kind, number))
                kinds[kind] += 1
        forms_total += len(forms)
        with_forms += bool(forms)
        rows.append((text, forms))
    head = f"""# The game's forms of each trade stat — v{VERSION} (C113, C106;
# src/trade/forms.rs), step 7e of search/TRADE-PLAN.md at 77666ea4.
#
# Reference data: generated, committed, shipped inside the binary
# (`include_str!` in src/trade/forms.rs), read-only, read by the translation
# alone (C90). Generated by tools/trade-forms-table.py from the PoE1 export
# at its pin (search/repoe/MANIFEST.md: poe1@e2bd511a, the game's
# stat_translations) joined to the stat table by id; its rules are the
# script's docstring. Game data (C106): what the game displays, never a
# judgment. The forms table beside it, by hand (reference/trade-forms-v1.toml),
# holds the rules for a text the export joins nowhere, the forms the captures
# show and the export does not, and the refusals by name.
#
# What a row means: the site's text (its ` (Local)` dropped) also displays
# as each form, of its kind — `opposite` below nothing, `numberless` a
# number left out (`number` the one value it stands for, where it is one),
# `plural`, `other`. A text with `forms = []` is joined and the game shows it
# no other way. A text with no row is joined nowhere.
#
# Measured at generation: {joined} texts joined, {with_forms} of them with a
# form; {forms_total} forms — {kinds['opposite']} opposite, {kinds['numberless']} numberless,
# {kinds['plural']} plural, {kinds['other']} other.

version = {VERSION}
stats = "{stats['version']}"
source = "the PoE1 export at poe1 e2bd511a (3.29.3.3), data/stat_translations.json, joined to the stat table by id"
generated_by = "tools/trade-forms-table.py"
"""
    out = [head]
    for text, forms in rows:
        out.append(f"\n[[text]]\ntext = {toml_str(text)}\n")
        if not forms:
            out.append("forms = []\n")
            continue
        out.append("forms = [\n")
        for form, kind, number in forms:
            n = f", number = {number}" if number is not None else ""
            out.append(f"    {{ form = {toml_str(form)}, kind = \"{kind}\"{n} }},\n")
        out.append("]\n")
    text = "".join(out)
    if check:
        on_disk = open(OUT).read() if os.path.exists(OUT) else ""
        if on_disk != text:
            sys.exit(f"{OUT} differs from what the sources give: run tools/trade-forms-table.py")
        print(f"ok {joined} texts joined, {forms_total} forms")
        return
    with open(OUT, "w") as f:
        f.write(text)
    print(f"wrote {OUT}: {joined} texts joined, {with_forms} with a form, {forms_total} forms {kinds}")


if __name__ == "__main__":
    main()
