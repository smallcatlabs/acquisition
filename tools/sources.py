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
sibling clone at the commit its manifest pins, any other input a path that
exists. Nothing degrades; an absence is a refusal naming every absent input.

    from sources import require, POE1, COPY
    require(POE1, COPY, ...)             # in main(), before the first read

    tools/sources.py --self-test         # refusals made here, nothing read

The clones sit beside this repository (`ACQ_CLONES` names another
directory); the raw directories are gitignored and local (each track's
MANIFEST.md lists what was saved).
"""

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
# new manifest row and a new commit here, then a reviewed regeneration.
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


def absent(path, clones=None):
    """Why an input cannot be read, or None when it can."""
    clones = CLONES if clones is None else clones
    path = Path(path)
    if not path.exists():
        return f"{path}: absent"
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


def require(*inputs, clones=None):
    """Refuse, naming every input that cannot be read, unless all can."""
    missing = [why for why in (absent(p, clones) for p in inputs) if why]
    if missing:
        raise SystemExit("refused, nothing written — an input is absent or not at its pin:\n  "
                         + "\n  ".join(missing))


def self_test():
    """The refusals, on directories made here: an absent path, a clone at no
    commit or another, a path inside a clone, all named at once; a present
    path passes."""
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
        cases = [
            ("a present path that is no clone passes", [present], []),
            ("an absent path is refused by name", [clones / "gone"], ["gone: absent"]),
            ("a clone at no commit is refused", [clones / "poe1"], ["poe1 is at no commit"]),
            ("a clone at another commit is refused", [clones / "awakened-poe-trade"], ["not the pinned ce551eb7"]),
            ("a path inside a clone is held to the clone's pin", [clones / "poe1" / ".git"], ["poe1 is at no commit"]),
            ("every absence is named at once", [clones / "gone", clones / "poe1", present],
             ["gone: absent", "poe1 is at no commit"]),
        ]
        for name, inputs, want in cases:
            try:
                require(*inputs, clones=clones)
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
