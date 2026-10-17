"""category: what the three scripts share — the pinned inputs, the readings, the copy.

Not run on its own. Every input is read-only, and each reader here requires what it reads
(tools/sources.py): an absent input, or a clone at another commit, is a refusal, never a reading
of what is there.

The export's class -> trade id map and its name rules are not restated here: they are imported
from search/repoe/scripts/base-taxonomy.py (CLASS_TO_TRADE, NAME_RULES, PARENTS, predict), so the
rule lives in one place and this track measures it.
"""

import collections
import csv
import importlib.util
import json
import os
import re
import sqlite3
import sys
import tomllib

HERE = os.path.dirname(os.path.abspath(__file__))
TRACK = os.path.dirname(HERE)
SEARCH = os.path.dirname(TRACK)
ROOT = os.path.dirname(SEARCH)
DATA = os.path.join(TRACK, "data")

sys.path.insert(0, os.path.join(ROOT, "tools"))
from sources import APT, COPY, POE1, require  # noqa: E402  the clones, pinned; the copy, local

TQ = os.path.join(SEARCH, "trade-query", "data")
TABLE = os.path.join(ROOT, "crates", "acquisition-search", "reference", "classes-v1.toml")
TAXONOMY = os.path.join(SEARCH, "repoe", "scripts", "base-taxonomy.py")

# the three constants the class table's reader uses (crates/acquisition-search/src/class.rs, lines 113-121)
MAP_PREFIXES = ["Blighted ", "Blight-ravaged "]
GEM_CLASSES = {"Skill Gems", "Support Gems"}
QUEST_CLASS = "Quest Items"

BEASTS = ["monster.beast", "monster.yellowbeast", "monster.redbeast"]
BEAST_HELP = "Right-click to add this to your bestiary."  # the body's descrText; APT's BEAST_HELP (client_strings.js line 57)


def taxonomy():
    spec = importlib.util.spec_from_file_location("base_taxonomy", TAXONOMY)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def write_csv(name, header_line, columns, rows):
    os.makedirs(DATA, exist_ok=True)
    with open(os.path.join(DATA, name), "w", newline="") as f:
        f.write(header_line + "\n")
        w = csv.writer(f, lineterminator="\n")
        w.writerow(columns)
        w.writerows(rows)


def site_filters():
    """[(id, text)] in the filter's order, the empty 'Any' option dropped."""
    filters = json.load(open(os.path.join(TQ, "filters-2026-09-12.json")))["result"]
    return [(o["id"], o["text"]) for g in filters for f in g["filters"] if f["id"] == "category"
            for o in f["option"]["options"] if o["id"]]


def site_coarse():
    """base type -> set of coarse category ids (items-2026-09-12.json), and the coarse ids with labels."""
    items = json.load(open(os.path.join(TQ, "items-2026-09-12.json")))["result"]
    by_type = collections.defaultdict(set)
    for c in items:
        for e in c["entries"]:
            by_type[e["type"]].add(c["id"])
    return by_type, [(c["id"], c["label"], len(c["entries"])) for c in items]


def structure(ids):
    """The site's ids as its data shapes them: a prefix parent has another id under `<it>.`;
    a grouping is an id whose members no dot names (weapon.one, weapon.dagger, ...): an id with a
    dot whose own prefix is a parent and that is not a leaf of the export. Returned: kind per id."""
    idset = set(ids)
    prefix_parents = {i for i in ids if any(j.startswith(i + ".") for j in ids)}
    return prefix_parents, idset


class Export:
    """The pinned export: base name -> internal classes (every release state, as the class table
    reads it), transfigured gems by display name (rule 2 of tools/class-table.py), and the classes."""

    def __init__(self):
        require(POE1)
        d = os.path.join(POE1, "data")
        self.bases = json.load(open(os.path.join(d, "base_items.json")))
        self.classes = json.load(open(os.path.join(d, "item_classes.json")))
        gems = json.load(open(os.path.join(d, "gems.json")))
        self.mods = json.load(open(os.path.join(d, "mods.json")))
        self.by_name = collections.defaultdict(set)
        for b in self.bases.values():
            if b["name"]:
                self.by_name[b["name"]].add(b["item_class"])
        for g in gems.values():
            bi = g.get("base_item")
            if not bi:
                continue
            name = (g.get("active_skill") or {}).get("display_name") or bi["display_name"]
            if name:
                self.by_name[name].add(self.bases[bi["id"]]["item_class"])

    def display(self, cls):
        return self.classes[cls]["name"] or ""


def load_table():
    t = tomllib.load(open(TABLE, "rb"))
    by_base = collections.defaultdict(set)
    for c in t["class"]:
        for b in c["bases"]:
            by_base[b].add(c["name"])
    return t, by_base


def shown(raw):
    """derive::shown (crates/acquisition-search/src/derive.rs, line 1150): `<style>{X}` -> X, `[A|B]` -> B."""
    text = (raw or "").replace("\r\n", "\n").replace("\r", "\n")
    out, i, open_ = [], 0, 0
    while i < len(text):
        c = text[i]
        if c == "<":
            end = text.find(">", i)
            if end >= 0 and text[end + 1:end + 2] == "{":
                open_ += 1
                i = end + 2
                continue
        if c == "}" and open_ > 0:
            open_ -= 1
        else:
            out.append(c)
        i += 1
    s = "".join(out)
    return re.sub(r"\[([^\]]*)\]", lambda m: m.group(1).rsplit("|", 1)[-1], s)


def copy_items():
    """Every live pc item of the copy, read-only: (id, league, location_kind, json dict)."""
    require(COPY)
    db = sqlite3.connect(f"file:{COPY}?mode=ro", uri=True)
    rows = db.execute("SELECT id, league, location_kind, json FROM items WHERE realm = 'pc' AND removed_at IS NULL ORDER BY id").fetchall()
    db.close()
    return [(i, lg, lk, json.loads(js)) for i, lg, lk, js in rows]


def is_beast(body):
    return body.get("descrText") == BEAST_HELP


def classes_of(export, base, frame):
    """The export's internal classes of a base as the class table's reader picks them
    (class.rs lines 370-460): exact name, else past a blight prefix; several -> the frame picks.
    Returns (classes, how)."""
    found = set(export.by_name.get(base, ()))
    how = "base"
    if not found:
        for p in MAP_PREFIXES:
            if base.startswith(p) and len(export.by_name.get(base[len(p):], ())) == 1:
                found, how = set(export.by_name[base[len(p):]]), "base past a blight prefix"
                break
    if len(found) > 1 and frame:
        disp = {c: export.display(c) for c in found}
        if frame.lower() == "gem":
            picked = {c for c in found if disp[c] in GEM_CLASSES}
        elif frame.lower() == "quest":
            picked = {c for c in found if disp[c] == QUEST_CLASS}
        else:
            picked = {c for c in found if disp[c] not in GEM_CLASSES and disp[c] != QUEST_CLASS}
        if picked:
            found, how = picked, how + ", frame picks"
    return found, how


def place(export, tax, body):
    """The readings on one item: the ids it is placed under, and why.
    Returns (ids, basis, classes, reason) where reason names a residue kind or ''."""
    base = shown(body.get("baseType"))
    if is_beast(body):
        return set(BEASTS), "body rule (descrText)", set(), "a beast: which of the three leaves is not settled on this machine"
    if not base:
        return set(), "", set(), "no base"
    cls, how = classes_of(export, base, body.get("frameTypeId"))
    if not cls:
        return set(), "", set(), "no base in any table"
    ids, bases_ = set(), set()
    for c in cls:
        fine, basis = tax.predict(c, base)
        if fine:
            ids.add(fine)
            bases_.add(basis)
    if not ids:
        return set(), how, cls, "a class with no id"
    if len(ids) > 1:
        return ids, how, cls, "a base under several ids"
    return ids, how + "; " + "/".join(sorted(bases_)), cls, ""


def apt():
    """Awakened PoE Trade: base name -> trade id, by items.ndjson's craftable.category through
    CATEGORY_TO_TRADE_ID (pathofexile-trade.ts) and the ItemCategory enum (parser/meta.ts).
    Returns (placements {name: id}, unmapped {name: category}, the map {category string: id}, beasts [names])."""
    require(APT)
    meta = open(os.path.join(APT, "renderer", "src", "parser", "meta.ts")).read()
    enum = dict(re.findall(r"^\s+(\w+) = '([^']+)'", meta.split("export enum ItemCategory")[1].split("}")[0], re.M))
    trade_ts = open(os.path.join(APT, "renderer", "src", "web", "price-check", "trade", "pathofexile-trade.ts")).read()
    block = trade_ts.split("export const CATEGORY_TO_TRADE_ID = new Map([")[1].split("])")[0]
    cmap = {enum[k]: v for k, v in re.findall(r"\[ItemCategory\.(\w+), '([^']+)'\]", block)}
    place_, unmapped, beasts = {}, {}, []
    for line in open(os.path.join(APT, "renderer", "public", "data", "en", "items.ndjson")):
        j = json.loads(line)
        if j.get("namespace") == "CAPTURED_BEAST":
            beasts.append(j["name"])
        cat = (j.get("craftable") or {}).get("category")
        if not cat:
            continue
        if cat in cmap:
            place_[j["name"]] = cmap[cat]
        else:
            unmapped[j["name"]] = cat
    return place_, unmapped, cmap, beasts
