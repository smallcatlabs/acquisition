#!/usr/bin/env python3
"""The stat table (C113, C106; K6, src/trade/translate.rs): every stat id of the
trade site's six hash categories, the categories that carry it and the text
the site displays for it — regenerated from the dated capture of the site's
stat vocabulary into the reviewed file the search crate ships. Step 2 of
search/TRADE-PLAN.md.

    python3 -I tools/trade-stats-table.py            # writes crates/acquisition-search/reference/trade-stats-v1.toml
    python3 -I tools/trade-stats-table.py --check    # exits 1 when the file on disk differs from what the capture gives

Input (read-only; committed; its sha256 is pinned below and in the track's
manifest, so a new capture is a new dated file, a new pin and a reviewed diff
of the table, never a silent regeneration):
  search/trade-query/data/stats-2026-09-12.json   the site's /api/trade/data/stats,
                                                   captured by the owner in a browser
                                                   (search/trade-query/MANIFEST.md)

Rules, each a reading of the capture and never a judgment (C106):
  1. The hash categories are the six the catalogue lands as a line
     (`stat.<category>` in reference/trade-catalogue-v1.toml): explicit,
     implicit, fractured, crafted, enchant, scourge. Every entry of each is
     a row, whatever the shape of its id (`stat_N`, `stat_N|N`,
     `indexable_skill_N`, `delirium_reward_<name>` …); the other eight
     categories are the catalogue's to refuse or translate, and no row here.
  2. A row is keyed by the id without its category, because one id's texts
     are the same under every category that carries it. That is measured
     here, never assumed: an id whose texts differ between two categories
     stops the script, and the table would be keyed by category and id.
  3. An id may carry several texts within one category, as the site lists it
     twice (a map's own wording beside the atlas's `your Maps`). The row
     keeps every text, in the capture's order; which one a clause matches is
     the translation's (step 4), never a choice made here.
  4. Order is the capture's: rows by the first entry that names the id,
     categories in the capture's category order.
Anything else the capture holds that these rules do not expect — an entry
whose `type` or id prefix is not its category, an option on a hash entry, an
empty text, one text twice under one id in one category, a hash category
missing or empty — stops the script. The counts the header prints are
measured at generation.
"""

import collections
import hashlib
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
CAPTURE = os.path.join(ROOT, "search", "trade-query", "data", "stats-2026-09-12.json")
CAPTURE_SHA256 = "b983f1ddf4d999a15db5ad4dbd5eb30d02bf0f1b4a8e23b4fb251f726af3063d"  # search/trade-query/MANIFEST.md
VERSION = "2026-09-12"  # the capture's date (the brief, step 2)
OUT = os.path.join(ROOT, "crates", "acquisition-search", "reference", "trade-stats-v1.toml")
HASH = ["explicit", "implicit", "fractured", "crafted", "enchant", "scourge"]


def toml_str(s):
    out = s.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n")
    if any(ord(c) < 0x20 or ord(c) == 0x7F for c in out):
        sys.exit(f"{s!r}: a control character this writer does not escape")
    return '"' + out + '"'


def shape(rest):
    return re.sub(r"\d+", "N", re.sub(r"^(pseudo_timeless_jewel|delirium_reward)_[a-z]+$", r"\1_<name>", rest))


def main():
    check = "--check" in sys.argv[1:]
    raw = open(CAPTURE, "rb").read()
    digest = hashlib.sha256(raw).hexdigest()
    if digest != CAPTURE_SHA256:
        sys.exit(f"{CAPTURE} is {digest}, not the pinned {CAPTURE_SHA256}: a new capture is a new dated file and a reviewed diff of the table")
    categories = json.loads(raw)["result"]
    order = [c["id"] for c in categories if c["id"] in HASH]
    for cat in HASH:
        if order.count(cat) != 1:
            sys.exit(f"the capture carries the hash category {cat!r} {order.count(cat)} times")

    texts = collections.defaultdict(lambda: collections.defaultdict(list))  # id -> category -> texts
    first = []  # ids by first appearance
    seen = set()
    entries = collections.Counter()
    shapes = collections.Counter()
    for c in categories:
        cat = c["id"]
        if cat not in HASH:
            continue
        if not c["entries"]:
            sys.exit(f"the hash category {cat!r} has no entry")
        for e in c["entries"]:
            prefix, dot, rest = e["id"].partition(".")
            if prefix != cat or not dot or not rest or e.get("type") != cat:
                sys.exit(f"{e['id']!r} under {cat!r}: its prefix or type is not its category")
            if "option" in e:
                sys.exit(f"{e['id']!r}: an option on a hash entry, which no rule reads")
            if not e["text"].strip():
                sys.exit(f"{e['id']!r}: an empty text")
            if e["text"] in texts[rest].get(cat, []):
                sys.exit(f"{e['id']!r}: the text {e['text']!r} twice")
            if rest not in seen:
                seen.add(rest)
                first.append(rest)
            if not texts[rest][cat]:
                shapes[shape(rest)] += 1
            texts[rest][cat].append(e["text"])
            entries[cat] += 1

    several_cats = 0
    several_texts = 0
    for rest in first:  # rule 2, measured
        lists = {tuple(v) for v in texts[rest].values()}
        if len(lists) > 1:
            sys.exit(f"{rest!r}: its texts differ between categories {dict(texts[rest])}: key the table by category and id")
        several_cats += len(texts[rest]) > 1
        several_texts += len(next(iter(lists))) > 1
    several_entries = sum(1 for rest in first for v in texts[rest].values() if len(v) > 1)

    collide = {}
    for cat in HASH:  # a text the site gives two ids or more (S66), for the reader
        by_text = collections.defaultdict(set)
        for rest in first:
            for t in texts[rest].get(cat, []):
                by_text[t].add(rest)
        collide[cat] = sum(1 for ids in by_text.values() if len(ids) > 1)

    pairs = sum(len(texts[r]) for r in first)
    out = [
        "# The stat table — the capture of 2026-09-12 (C113, C106; K6 in",
        "# src/trade/translate.rs), step 2 of search/TRADE-PLAN.md.",
        "#",
        "# Reference data: generated, committed, shipped inside the binary",
        "# (`include_str!` in `src/trade/stats.rs`), read-only, read by the",
        "# translation alone (C90: a stat id exists only at the trade boundary) and",
        "# cited by version in the basis of every answer that used it (C98).",
        "# Generated by tools/trade-stats-table.py from the site's stat vocabulary as",
        "# the owner captured it (search/trade-query/data/stats-2026-09-12.json;",
        "# SURFACES.md, the trade-site row), and reviewed as a diff at each new",
        "# capture: what the site lists, never a judgment. Its rules are the",
        "# script's docstring.",
        "#",
        "# What a row means: the site's query names a stat `<category>.<id>`; the",
        "# row `id` is carried by each category in `in`, and the site displays it",
        "# as each text in `texts` (a row break written `\\n`). An id is keyed",
        "# without its category because its texts are the same under every",
        "# category that carries it: measured, and the script stops where it is",
        "# not. The version is the capture's date.",
        "#",
        f"# Measured at generation: {len(first)} ids, {pairs} (category, id) pairs,",
        f"# {sum(entries.values())} entries — {', '.join(f'{cat} {entries[cat]}' for cat in order)};",
        f"# {several_cats} ids under more than one category, each with one list of texts",
        f"# under all of them; {several_texts} ids with two texts or more, in {several_entries} pairs.",
        "# Pairs by the id's shape: "
        + ", ".join(f"{k} {v}" for k, v in sorted(shapes.items(), key=lambda kv: (-kv[1], kv[0])))
        + ".",
        "# Texts the site gives two ids or more under one category (S66), read",
        "# exactly: " + ", ".join(f"{cat} {collide[cat]}" for cat in order) + ".",
        "",
        f"version = {toml_str(VERSION)}",
        f"source = {toml_str(f'the trade site /api/trade/data/stats, captured by the owner on {VERSION} (sha256 {CAPTURE_SHA256[:16]}): search/trade-query/data/stats-{VERSION}.json, SURFACES.md')}",
        'generated_by = "tools/trade-stats-table.py"',
        "",
    ]
    for rest in first:
        cats = [cat for cat in order if cat in texts[rest]]
        out.append("[[stat]]")
        out.append(f"id = {toml_str(rest)}")
        out.append(f"in = [{', '.join(toml_str(c) for c in cats)}]")
        out.append(f"texts = [{', '.join(toml_str(t) for t in texts[rest][cats[0]])}]")
        out.append("")
    text = "\n".join(out)
    if check:
        if not os.path.exists(OUT) or open(OUT, encoding="utf-8", newline="").read() != text:
            sys.exit(f"{OUT} differs from what the capture of {VERSION} gives: regenerate and review the diff")
        print(f"ok      {os.path.relpath(OUT, ROOT)} is what the capture gives")
        return
    with open(OUT, "w", encoding="utf-8", newline="\n") as f:
        f.write(text)
    print(f"{os.path.relpath(OUT, ROOT)}: {len(first)} ids, {pairs} pairs, {sum(entries.values())} entries, {several_cats} under several categories, {several_texts} with several texts")


if __name__ == "__main__":
    main()
