#!/usr/bin/env python3
"""The inputs a generator reads that the repository does not hold, and the
one refusal when any is absent.

A generator that read what happened to be on disk wrote something else when
an input was missing: category's expected.py, run without the Awakened PoE
Trade clone, turned 16 grouping memberships out under J12 and said "absent"
only in its log (the outside review of 606b4a17). Each script had decided
for itself what an absence meant — skip a row, compose a stand-in, read a
committed copy instead — and six of them kept a copy of the pin check, two
with a stale path to the clone. So: a generator names every input it reads
here, before it reads one, and writes nothing unless all are present — a
sibling clone at the commit its manifest pins, a raw directory holding what
its inventory records, any other input a path that exists. Nothing
degrades; an absence is a refusal naming every absent input.

A raw directory is held to its inventory as a clone is to its pin (the
outside review of 555c4f58): the extract, run over a restore that lacked
t01, wrote 1,149 searches for 1,150 and said nothing, since a missing file
and a search never captured looked alike. The inventory, committed beside
the track's data, lists every capture file its writer wrote with its
sha256 (`sha256sum`'s form: `shasum -a 256 -c` reads it in the directory);
a file it lists that is absent or other bytes, or a file it does not list,
is a refusal. Its writer is the splitter (`trade-split.py`, `record`); a
search the sheet asks that no file holds was never captured.

    from sources import require, POE1, COPY
    require(POE1, COPY, ...)             # in main(), before the first read

    tools/sources.py --self-test         # refusals made here, nothing read

The clones sit beside this repository (`ACQ_CLONES` names another
directory); the raw directories are gitignored and local (each track's
MANIFEST.md lists what was saved).
"""

import hashlib
import os
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SEARCH = ROOT / "search"
CLONES = Path(os.environ.get("ACQ_CLONES") or ROOT.parent)

# Each sibling clone at the commit its manifest pins (search/repoe/MANIFEST.md;
# awakened-poe-trade: search/prior-art/MANIFEST.md at 40b55e79). A pull is a
# new manifest row and a new commit here, then a reviewed regeneration, poe1's
# in the order tools/game-diff.py prints.
PINS = {
    "poe1": "e2bd511a0133bbe6c1ab548ef1285cb99f3cf0e9",
    "PathOfBuilding": "16de4b82d57f1c0de6eb40f37143c32d4da36a02",
    "awakened-poe-trade": "ce551eb7a9b704fbdcc2478eebb26be8f91786c7",
}
POE1 = CLONES / "poe1"
POB = CLONES / "PathOfBuilding"
APT = CLONES / "awakened-poe-trade"

# Local only, never committed (each directory's MANIFEST.md).
CAPTURES = SEARCH / "pseudo-stats" / "raw" / "searches"  # the sittings' captures
CENSUS = SEARCH / "trade-query" / "raw" / "searches"  # the owner's eight searches of 2026-09-13
COPY = SEARCH / "item-facts" / "raw" / "m3" / "store" / "mock" / "GERWARIC_7694.db"  # the owner's store copy
CORPUS = (SEARCH / "item-facts" / "raw" / "spike-GERWARIC_7694-2026-09-13.db",
          SEARCH / "item-facts" / "raw" / "cpp-userstore-GERWARIC_7694-2026-09-13.db")
# Each raw directory's inventory: the files its writer wrote, by digest.
INVENTORIES = {
    CAPTURES: SEARCH / "pseudo-stats" / "data" / "raw-searches.sha256",
    CENSUS: SEARCH / "trade-query" / "data" / "raw-searches.sha256",
}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def inventory(path):
    """{file name: sha256} as an inventory records them."""
    return {name: sha for sha, name in (line.split("  ", 1) for line in Path(path).read_text().splitlines())}


def record(path, wrote, removed=()):
    """Write an inventory with `wrote` ({file name: bytes}) set and `removed`
    gone, whole, through a rename: it is never part of one."""
    path = Path(path)
    held = inventory(path) if path.exists() else {}
    held.update({name: digest(data) for name, data in wrote.items()})
    for name in removed:
        held.pop(name, None)
    part = path.with_name(path.name + ".part")
    part.write_text("".join(f"{sha}  {name}\n" for name, sha in sorted(held.items())))
    os.replace(part, path)


def unlike(directory, path):
    """How a raw directory differs from its inventory: one line a file."""
    want = inventory(path)
    held = {p.name: p for p in Path(directory).glob("*.json")}
    out = [f"{directory / n}: absent, its inventory lists it" for n in sorted(want.keys() - held.keys())]
    out += [f"{directory / n}: not in its inventory ({path.name})" for n in sorted(held.keys() - want.keys())]
    out += [f"{directory / n}: other bytes than its inventory records" for n in sorted(want.keys() & held.keys())
            if digest(held[n].read_bytes()) != want[n]]
    return out


def absent(path, clones=None, inventories=None):
    """Why an input cannot be read, or None when it can."""
    clones = CLONES if clones is None else clones
    inventories = INVENTORIES if inventories is None else inventories
    path = Path(path)
    if not path.exists():
        return f"{path}: absent"
    if path in inventories:
        if not inventories[path].exists():
            return f"{path}: its inventory {inventories[path]} is absent"
        return "\n  ".join(unlike(path, inventories[path])) or None
    try:
        name = path.relative_to(clones).parts[0]
    except (ValueError, IndexError):
        return None
    if name not in PINS:
        return None
    git = subprocess.run(["git", "-C", str(clones / name), "rev-parse", "--verify", "-q", "HEAD"],
                         capture_output=True, text=True)
    head = git.stdout.strip() if git.returncode == 0 else ""
    if head != PINS[name]:
        return f"{clones / name} is at {head or 'no commit'}, not the pinned {PINS[name]}"
    return None


def require(*inputs, clones=None, inventories=None):
    """Refuse, naming every input that cannot be read, unless all can."""
    missing = [why for why in (absent(p, clones, inventories) for p in inputs) if why]
    if missing:
        raise SystemExit("refused, nothing written — an input is absent or not at its pin:\n  "
                         + "\n  ".join(missing))


def self_test():
    """The refusals, on directories made here: an absent path, a clone at no
    commit or another, a path inside a clone, all named at once; a present
    path passes. A raw directory missing a search or a page its inventory
    lists, holding other bytes or a file it does not list, is refused; one
    as recorded passes."""
    failed = 0
    with tempfile.TemporaryDirectory() as tmp:
        clones = Path(tmp)
        present = clones / "present.txt"
        present.write_text("")
        (clones / "poe1").mkdir()
        subprocess.run(["git", "init", "-q", str(clones / "poe1")], check=True)
        other = clones / "awakened-poe-trade"
        subprocess.run(["git", "init", "-q", str(other)], check=True)
        subprocess.run(["git", "-C", str(other), "-c", "user.name=t", "-c", "user.email=t@t",
                        "commit", "-q", "--allow-empty", "-m", "t"], check=True)
        raw = clones / "raw"
        raw.mkdir()
        listed = clones / "raw.sha256"
        unit = {"s-search.json": b"S", "s-fetch.json": b"F", "s-fetch-2.json": b"G", "e-search.json": b""}
        record(listed, unit)
        inventories = {raw: listed}

        def restored(**change):
            for p in raw.iterdir():
                p.unlink()
            for name, data in {**unit, **change}.items():
                if data is not None:
                    (raw / name).write_bytes(data)
            return raw

        cases = [
            ("a raw directory as its inventory records passes", [lambda: restored()], []),
            ("a search its inventory lists, absent, is refused", [lambda: restored(**{"s-search.json": None})],
             ["s-search.json: absent, its inventory lists it"]),
            ("a fetch page its inventory lists, absent, is refused", [lambda: restored(**{"s-fetch-2.json": None})],
             ["s-fetch-2.json: absent"]),
            ("a capture of other bytes is refused", [lambda: restored(**{"s-fetch.json": b"X"})],
             ["s-fetch.json: other bytes"]),
            ("a file its inventory does not list is refused", [lambda: restored(**{"t-search.json": b"T"})],
             ["t-search.json: not in its inventory"]),
            ("a present path that is no clone passes", [present], []),
            ("an absent path is refused by name", [clones / "gone"], ["gone: absent"]),
            ("a clone at no commit is refused", [clones / "poe1"], ["poe1 is at no commit"]),
            ("a clone at another commit is refused", [clones / "awakened-poe-trade"], ["not the pinned ce551eb7"]),
            ("a path inside a clone is held to the clone's pin", [clones / "poe1" / ".git"], ["poe1 is at no commit"]),
            ("every absence is named at once", [clones / "gone", clones / "poe1", present],
             ["gone: absent", "poe1 is at no commit"]),
        ]
        for name, inputs, want in cases:
            inputs = [i() if callable(i) else i for i in inputs]
            try:
                require(*inputs, clones=clones, inventories=inventories)
                said = ""
            except SystemExit as refusal:
                said = str(refusal)
            if bool(want) != bool(said) or any(w not in said for w in want):
                failed += 1
                print(f"DIFFER  {name}: {said or 'passed'}")
    print(f"{len(cases)} refusals made here, {failed} differ")
    return 1 if failed else 0


if __name__ == "__main__":
    if sys.argv[1:] != ["--self-test"]:
        print(__doc__)
        sys.exit(2)
    sys.exit(self_test())
