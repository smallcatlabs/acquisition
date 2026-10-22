#!/usr/bin/env python3
"""What changed in the game between two exports, and what each shipped table
does about it (C125, the tables name their export; C106, C68): the five files
of the PoE1 export the tables draw on, read from the owner's `poe1` clone at
two refs, keyed by the export's own ids.

    python3 -I tools/game-diff.py                       # the pin (tools/sources.py) against the clone's HEAD
    python3 -I tools/game-diff.py --old REF --new REF   # any two refs the clone resolves (origin/master after a fetch)
    python3 -I tools/game-diff.py --all                 # every name, not the first twenty of a list
    python3 -I tools/game-diff.py --json                # the whole document
    python3 -I tools/game-diff.py --self-test           # the diff and the drift line on repositories made here
    python3 -I tools/game-diff.py --check               # the drift line: the clone and the shipped headers against the pin

Read-only and offline: the clone is read through `git show <ref>:<path>`,
never checked out, never fetched, and never through `sources.require`, whose
pin check would refuse the run this tool is for. Nothing is written.

Keys. `base_items.json`, `gems.json`, `mods.json` and `item_classes.json` by
their object keys; `stat_translations.json` by the sorted tuple of each
entry's `ids`. Per file: added, removed, renamed (the same key with a new
name: `name`, a gem's `display_name`, a translation's English strings), and
changed, counted by the top-level field that differs. A rename in place is a
rename, never a removal and an addition. Under the translations, the trade
stat ids added and removed, since the forms table joins on them.

The reading for the tables, printed after the counts, is the regeneration
procedure's one home: which table each file feeds, which generator remakes
it, in the order that holds, and the pin bumped first.

The drift line (`--check`) passes when no clone sits beside the tree, saying
so; passes when the clone is at the pin and every shipped header names the
pin and the clone's game version; fails, naming what to do, when the clone
is at another commit, when a header's pin or game is not the clone's, or
when the pinned export lags the game version it names.
"""

import argparse
import json
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
sys.path.insert(0, str(HERE))
from sources import CLONES, PINS  # noqa: E402  the clones and their pins (search/repoe/MANIFEST.md)

REFERENCE = ROOT / "crates" / "acquisition-search" / "reference"
# The shipped tables whose header names an export of the clone.
HEADERS = [REFERENCE / "classes-v1.toml", REFERENCE / "defences-v1.toml", REFERENCE / "trade-game-forms-v1.toml"]
BOUND = 20

# file, what keys it, the field that names an entry
FILES = [
    ("base_items.json", "metadata id", "name"),
    ("item_classes.json", "class id", "name"),
    ("gems.json", "gem id", "display_name"),
    ("mods.json", "mod id", "name"),
    ("stat_translations.json", "stat ids", "English"),
]

# The shipped tables in the order they regenerate, each with the export files
# it reads and what a change means for it. The procedure's one home.
TABLES = [
    ("classes-v1", "tools/class-table.py", ["base_items.json", "gems.json", "item_classes.json"],
     "first: base names under the game's class names; a new class or a renamed base is a row"),
    ("defences-v1", "tools/defence-table.py", ["base_items.json"],
     "the ranges of `properties`; it checks itself against search/repoe/data/base-defences.csv, "
     "whose generator is not in the tree (deleted at c756bd38), so a change to a range stops it"),
    ("categories-v1", "tools/category-table.py", ["base_items.json", "gems.json", "item_classes.json"],
     "after the class table, which it reads, so the class table's inputs are its own; it refuses a "
     "class it cannot place, which a new class is, and checks its name rules against the bases"),
    ("trade-stats-v1", "tools/trade-stats-table.py", [],
     "the site's stat capture, the owner's (a sitting), not the export: no diff here moves it"),
    ("trade-game-forms-v1", "tools/trade-forms-table.py", ["stat_translations.json"],
     "after the stat table, which it reads; it joins the translations' trade stat ids to it"),
]
PROCEDURE = ("bump PINS in tools/sources.py and the clone row of search/repoe/MANIFEST.md first, "
             "since every generator refuses a clone off its pin; run the generators in this order, "
             "each then with --check; commit the pin, the manifest and the tables together")


def git(clone, *args):
    out = subprocess.run(["git", "-C", str(clone), *args], capture_output=True)
    return out.stdout if out.returncode == 0 else None


def resolve(clone, ref):
    out = git(clone, "rev-parse", "--verify", "-q", f"{ref}^{{commit}}")
    if out is None:
        raise SystemExit(f"{clone}: no commit {ref}")
    return out.decode().strip()


def show(clone, sha, path):
    return git(clone, "show", f"{sha}:{path}")


def versions(clone, sha):
    """The game version at a ref, the version last exported where the file
    exists, and which it read."""
    game = (show(clone, sha, "version.txt") or b"").decode().strip()
    exported = show(clone, sha, "exported-version.txt")
    if exported is not None:
        return {"game": game, "exported": exported.decode().strip(), "read": "exported-version.txt"}
    subject = (git(clone, "log", "-1", "--format=%s", sha) or b"").decode().strip()
    m = re.search(r"\bExport (\S+)", subject)
    return {"game": game, "exported": m.group(1) if m else None,
            "read": f"version.txt and the commit subject ({subject!r}); exported-version.txt is absent"}


def keyed(name, data):
    if name != "stat_translations.json":
        return data
    out = {}
    for entry in data:
        key = " ".join(sorted(entry.get("ids") or []))
        while key in out:  # two entries over one id set keep both
            key += " +"
        out[key] = entry
    return out


def title(name, entry):
    if name == "stat_translations.json":
        return tuple(s.get("string") for s in entry.get("English") or [])
    if name == "gems.json":
        return entry.get("display_name") or (entry.get("base_item") or {}).get("display_name")
    return entry.get("name")


def one_line(text):
    return str(text).replace("\n", " / ")


def variants(old, new):
    """A translation's English strings that changed, by position: each
    `'old' -> 'new'`, a string added or dropped against nothing."""
    pairs = []
    for i in range(max(len(old), len(new))):
        a = old[i] if i < len(old) else None
        b = new[i] if i < len(new) else None
        if a != b:
            pairs.append(f"{i}: {one_line(a) if a is not None else 'nothing'!r} -> "
                         f"{one_line(b) if b is not None else 'nothing'!r}")
    return "; ".join(pairs)


def label(name, key, entry):
    t = title(name, entry)
    if name == "stat_translations.json":
        return f"{key}: {one_line(t[0]) if t else ''}"
    return f"{key} ({t})" if t else key


def trade_ids(data):
    return {ts["id"] for entry in data for ts in entry.get("trade_stats") or []}


def diff_file(name, old, new):
    a, b = keyed(name, old), keyed(name, new)
    added = sorted(b.keys() - a.keys())
    removed = sorted(a.keys() - b.keys())
    renamed, changed, by_field = [], [], {}
    for k in sorted(a.keys() & b.keys()):
        x, y = a[k], b[k]
        if x == y:
            continue
        fields = sorted(f for f in set(x) | set(y) if x.get(f) != y.get(f))
        changed.append(k)
        for f in fields:
            by_field[f] = by_field.get(f, 0) + 1
        if title(name, x) != title(name, y):
            renamed.append({"key": k, "old": title(name, x), "new": title(name, y)})
    out = {
        "file": name,
        "added": [label(name, k, b[k]) for k in added],
        "removed": [label(name, k, a[k]) for k in removed],
        "renamed": renamed,
        "changed": len(changed),
        "changed_by_field": dict(sorted(by_field.items(), key=lambda kv: (-kv[1], kv[0]))),
    }
    if name == "stat_translations.json":
        ia, ib = trade_ids(old), trade_ids(new)
        out["trade_stats_added"] = sorted(ib - ia)
        out["trade_stats_removed"] = sorted(ia - ib)
    return out


def diff(clone, old_ref, new_ref):
    old, new = resolve(clone, old_ref), resolve(clone, new_ref)
    doc = {"clone": str(clone), "old": {"ref": old_ref, "commit": old, **versions(clone, old)},
           "new": {"ref": new_ref, "commit": new, **versions(clone, new)}, "files": []}
    for name, _, _ in FILES:
        path = f"data/{name}"
        a, b = show(clone, old, path), show(clone, new, path)
        if a is None or b is None:
            doc["files"].append({"file": name, "absent_at": [r for r, x in ((old, a), (new, b)) if x is None]})
            continue
        doc["files"].append(diff_file(name, json.loads(a), json.loads(b)) if a != b else
                            {"file": name, "added": [], "removed": [], "renamed": [], "changed": 0,
                             "changed_by_field": {},
                             **({"trade_stats_added": [], "trade_stats_removed": []}
                                if name == "stat_translations.json" else {})})
    moved = {f["file"] for f in doc["files"] if f.get("absent_at") or f["added"] or f["removed"] or f["changed"]}
    doc["tables"] = [{"table": t, "generator": g, "reads": reads, "inputs_moved": sorted(moved & set(reads)),
                      "means": means} for t, g, reads, means in TABLES]
    doc["mods"] = "mods.json feeds no table; a new mod's text counts toward no named total (src/totals.rs)"
    doc["procedure"] = PROCEDURE
    return doc


def lag(side):
    if side["exported"] is not None and side["exported"] != side["game"]:
        return (f"  {side['commit'][:8]}: the fork is between a game version and its export: "
                f"version {side['game']}, last exported {side['exported']}")
    return None


def bounded(items, everything):
    shown = items if everything else items[:BOUND]
    rest = len(items) - len(shown)
    return shown, rest


def render(doc, everything):
    out = []
    o, n = doc["old"], doc["new"]
    out.append(f"game-diff: {doc['clone']}")
    for side, s in (("old", o), ("new", n)):
        out.append(f"  {side} {s['ref']} = {s['commit'][:8]}: version {s['game']}, exported {s['exported']} "
                   f"(read from {s['read']})")
    for s in (o, n):
        if line := lag(s):
            out.append(line)
    for f in doc["files"]:
        key = next(k for name, k, _ in FILES if name == f["file"])
        out.append("")
        if f.get("absent_at"):
            out.append(f"{f['file']}: absent at {', '.join(c[:8] for c in f['absent_at'])}")
            continue
        head = (f"{f['file']} (by {key}): +{len(f['added'])} -{len(f['removed'])}, "
                f"{len(f['renamed'])} renamed, {f['changed']} changed")
        if "trade_stats_added" in f:
            head += f"; trade stat ids +{len(f['trade_stats_added'])} -{len(f['trade_stats_removed'])}"
        out.append(head)
        if f["changed_by_field"]:
            out.append("  changed by field: " + ", ".join(f"{k} {v}" for k, v in f["changed_by_field"].items()))
        lists = [("added", f["added"]), ("removed", f["removed"]),
                 ("renamed", [f"{r['key']}: {r['old']!r} -> {r['new']!r}" if not isinstance(r["old"], tuple)
                              else f"{r['key']}: {variants(r['old'], r['new'])}" for r in f["renamed"]])]
        if "trade_stats_added" in f:
            lists += [("trade stat ids added", f["trade_stats_added"]),
                      ("trade stat ids removed", f["trade_stats_removed"])]
        for what, items in lists:
            if not items:
                continue
            shown, rest = bounded(items, everything)
            out.append(f"  {what}:")
            out += [f"    {x}" for x in shown]
            if rest:
                out.append(f"    and {rest} more (--all)")
    out.append("")
    out.append("For the tables, in the order they regenerate:")
    for i, t in enumerate(doc["tables"], 1):
        moved = (f"its inputs moved ({', '.join(t['inputs_moved'])})" if t["inputs_moved"]
                 else "no input of it moved" if t["reads"] else "reads no export file")
        out.append(f"  {i}. {t['table']} ({t['generator']}): {moved}; {t['means']}")
    out.append(f"  {doc['mods']}")
    out.append(f"  A pull: {doc['procedure']}.")
    return "\n".join(out)


# ---- the drift line ----------------------------------------------------------

def header(path):
    """The pin and the game version a shipped table's header names."""
    text = Path(path).read_text(encoding="utf-8")
    top = text.split("\n[[", 1)[0]
    source = re.search(r'^source = "(.*)"$', top, re.M)
    game = re.search(r'^game = "(.*)"$', top, re.M)
    s = source.group(1) if source else ""
    pin = re.search(r"(?:@ |poe1 )([0-9a-f]{8})\b", s)
    named = game.group(1) if game else (re.search(r"\((?:game )?(\d+(?:\.\d+)+)\)", s) or [None, None])[1]
    return (pin.group(1) if pin else None), named


def check(clones=None, pins=None, headers=None):
    """The drift line: (passed, the line)."""
    clones = CLONES if clones is None else Path(clones)
    pin = (PINS if pins is None else pins)["poe1"]
    headers = HEADERS if headers is None else headers
    named = {Path(h).name: header(h) for h in headers}
    clone = clones / "poe1"
    games = sorted({g for _, g in named.values() if g})
    if not (clone / ".git").exists():
        return True, (f"drift: no poe1 clone beside the tree; tables at {', '.join(games) or 'no game named'} "
                      f"from {pin[:8]}, unread against the game")
    head = (git(clone, "rev-parse", "--verify", "-q", "HEAD") or b"").decode().strip()
    if head != pin:
        return False, (f"drift: the clone {clone} is at {head[:8] or 'no commit'}, past the pin {pin[:8]}: "
                       f"read what changed with tools/game-diff.py; then {PROCEDURE}; "
                       f"or check the pin out again (git -C {clone} checkout {pin[:8]})")
    v = versions(clone, pin)
    faults = []
    for name, (p, g) in named.items():
        if p != pin[:8]:
            faults.append(f"{name} names the export at {p or 'no pin'}, not the pin {pin[:8]}")
        if g != v["game"]:
            faults.append(f"{name} names game {g or 'none'}, the clone's is {v['game']}")
    if v["exported"] is not None and v["exported"] != v["game"]:
        faults.append(f"the pinned export lags the game it names: version {v['game']}, "
                      f"last exported {v['exported']}, so the tables were drawn from a lagging export")
    if faults:
        return False, "drift: " + "; ".join(faults) + f"; regenerate in the order tools/game-diff.py prints"
    return True, f"drift: the clone at the pin {pin[:8]}, game {v['game']}; {len(named)} headers name it"


# ---- the self-test -----------------------------------------------------------

def self_test():
    """Two snapshots of the five files in a repository made here, staging one
    base added, one removed, one renamed in place, one changed field and one
    trade stat id added under an existing translation; then the drift line's
    three outcomes: no clone, a clone at the pin, a clone off it, and a
    header naming another game."""
    failed, made = [], []

    def expect(name, ok):
        made.append(name)
        print(f"{'ok' if ok else 'DIFFER'}  {name}")
        if not ok:
            failed.append(name)

    with tempfile.TemporaryDirectory() as tmp:
        clones = Path(tmp) / "clones"
        repo = clones / "poe1"
        (repo / "data").mkdir(parents=True)
        g = ["git", "-C", str(repo), "-c", "user.name=t", "-c", "user.email=t@t"]
        subprocess.run(["git", "init", "-q", str(repo)], check=True)

        def snapshot(bases, translations, version, message):
            files = {
                "base_items.json": bases,
                "item_classes.json": {"Ring": {"name": "Rings"}},
                "gems.json": {"Fireball": {"display_name": "Fireball"}},
                "mods.json": {"Strength1": {"name": "", "stats": [1]}},
                "stat_translations.json": translations,
            }
            for name, data in files.items():
                (repo / "data" / name).write_text(json.dumps(data))
            (repo / "version.txt").write_text(version)
            (repo / "exported-version.txt").write_text(version)
            subprocess.run(g + ["add", "-A"], check=True)
            subprocess.run(g + ["commit", "-q", "-m", message], check=True)
            return resolve(repo, "HEAD")

        ring = {"name": "Iron Ring", "item_class": "Ring", "tags": ["ring"]}
        old = snapshot(
            {"Ring1": ring, "Ring2": {**ring, "name": "Coral Ring"}, "Ring3": {**ring, "name": "Paua Ring"},
             "Ring4": {**ring, "name": "Gold Ring"}},
            [{"ids": ["life"], "English": [{"string": "+{0} to maximum Life"}],
              "trade_stats": [{"id": "explicit.stat_1"}]},
             {"ids": ["es"], "English": [{"string": "A"}, {"string": "B"}]}],
            "1.0.0.1", "Export 1.0.0.1")
        new = snapshot(
            {"Ring1": ring, "Ring2": {**ring, "name": "Coral Loop"}, "Ring4": {**ring, "name": "Gold Ring",
                                                                                 "tags": ["ring", "gold"]},
             "Ring5": {**ring, "name": "Moonstone Ring"}},
            [{"ids": ["life"], "English": [{"string": "+{0} to maximum Life"}],
              "trade_stats": [{"id": "explicit.stat_1"}, {"id": "fractured.stat_1"}]},
             {"ids": ["es"], "English": [{"string": "A"}, {"string": "C"}]}],
            "1.0.0.2", "Export 1.0.0.2")

        doc = diff(repo, old, new)
        f = {x["file"]: x for x in doc["files"]}
        bases, st = f["base_items.json"], f["stat_translations.json"]
        expect("a base added is named", bases["added"] == ["Ring5 (Moonstone Ring)"])
        expect("a base removed is named", bases["removed"] == ["Ring3 (Paua Ring)"])
        expect("a base renamed in place is a rename, not a removal and an addition",
               bases["renamed"] == [{"key": "Ring2", "old": "Coral Ring", "new": "Coral Loop"}])
        expect("a changed field is counted by field", bases["changed"] == 2
               and bases["changed_by_field"] == {"name": 1, "tags": 1})
        expect("a trade stat id added under an existing translation is named",
               st["trade_stats_added"] == ["fractured.stat_1"] and st["added"] == [] and st["changed"] == 2)
        expect("an untouched file reports nothing", f["mods.json"]["changed"] == 0 and not f["mods.json"]["added"])
        text = render(doc, False)
        expect("a translation whose second string changed shows that string, not its unchanged first",
               "es: 1: 'B' -> 'C'" in text)
        categories = next(t for t in doc["tables"] if t["table"] == "categories-v1")
        classes_moved = next(t for t in doc["tables"] if t["table"] == "classes-v1")["inputs_moved"]
        expect("a base-only change moves the category table, which reads the class table",
               categories["inputs_moved"] == ["base_items.json"] == classes_moved)
        expect("the reading names the class table first and the forms table after the stat table",
               text.index("1. classes-v1") < text.index("3. categories-v1") < text.index("5. trade-game-forms-v1")
               and "bump PINS" in text)

        # the drift line
        headers = []
        for name, source, game in [("classes-v1.toml", f"RePoE poe1 export @ {new[:8]} (game 1.0.0.2): x", "1.0.0.2"),
                                   ("trade-game-forms-v1.toml", f"the PoE1 export at poe1 {new[:8]} (1.0.0.2), x", None)]:
            h = Path(tmp) / name
            h.write_text(f'version = 1\nsource = "{source}"\n' + (f'game = "{game}"\n' if game else "") + "\n[[x]]\n")
            headers.append(h)
        empty = Path(tmp) / "empty"
        empty.mkdir()
        passed, line = check(empty, {"poe1": new}, headers)
        expect(f"--check, no clone: passes, saying so ({line})", passed and "no poe1 clone" in line)
        passed, line = check(clones, {"poe1": new}, headers)
        expect(f"--check, the clone at the pin: passes ({line})", passed and "at the pin" in line)
        passed, line = check(clones, {"poe1": old}, headers)
        expect("--check, a clone at another commit than the pin: fails, naming the recovery",
               not passed and "past the pin" in line and "bump PINS" in line and "check the pin out" in line)
        bad = Path(tmp) / "defences-v1.toml"
        bad.write_text(f'source = "RePoE poe1 export @ {new[:8]} (game 1.0.0.1): x"\ngame = "1.0.0.1"\n')
        passed, line = check(clones, {"poe1": new}, headers + [bad])
        expect("--check, a header whose game is not the clone's: fails, naming it",
               not passed and "defences-v1.toml names game 1.0.0.1" in line)
        (repo / "exported-version.txt").write_text("1.0.0.1")
        subprocess.run(g + ["commit", "-q", "-am", "Version 1.0.0.3"], check=True)
        lagging = resolve(repo, "HEAD")
        subprocess.run(g + ["commit", "-q", "--allow-empty", "-m", "t"], check=True)
        subprocess.run(g + ["checkout", "-q", lagging], check=True)
        passed, line = check(clones, {"poe1": lagging}, headers)
        expect("--check, a pinned export that lags its game: fails, saying so", not passed and "lags" in line)
    print(f"{len(made)} cases made here, {len(failed)} differ")
    return 1 if failed else 0


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--old", default=PINS["poe1"], help="the older ref (default: the pin in tools/sources.py)")
    ap.add_argument("--new", default="HEAD", help="the newer ref (default: the clone's HEAD)")
    ap.add_argument("--all", action="store_true", help="every name, not the first twenty of a list")
    ap.add_argument("--json", action="store_true", help="the whole document as JSON")
    ap.add_argument("--self-test", action="store_true", help="the diff and the drift line on repositories made here")
    ap.add_argument("--check", action="store_true", help="the drift line: the clone and the headers against the pin")
    args = ap.parse_args()
    if args.self_test:
        sys.exit(self_test())
    if args.check:
        passed, line = check()
        print(("ok      " if passed else "") + line)
        sys.exit(0 if passed else 1)
    clone = CLONES / "poe1"
    if not (clone / ".git").exists():
        sys.exit(f"no poe1 clone at {clone} (ACQ_CLONES names another directory)")
    doc = diff(clone, args.old.removeprefix("poe1@"), args.new.removeprefix("poe1@"))
    print(json.dumps(doc, indent=1, ensure_ascii=False) if args.json else render(doc, args.all))


if __name__ == "__main__":
    main()
