#!/usr/bin/env python3
"""The test read's fixed input: every test function in the workspace, one
row each, split into the four groups the read's runners take.

Run from the repository root: python3 -I audits/rehome-2026-10/tests/inventory.py
Writes data/inventory-<group>.tsv beside this file: path, line, name, kind.
"""
import pathlib
import re

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[2]
GROUPS = {
    "daemon": ["acquisition-daemon"],
    "core": ["acquisition-store", "acquisition-plan", "acquisition-protocol", "acquisition-client"],
    "search": ["acquisition-search"],
    "frontends": ["acquisition-cli", "acquisition-mcp"],
}
ATTR = re.compile(r"^\s*#\[(tokio::)?test\b")
FN = re.compile(r"^\s*(?:pub\s+)?(?:async\s+)?fn\s+([A-Za-z0-9_]+)")
PROPTEST = re.compile(r"^\s*proptest!\s*\{")


def tests_of(path):
    lines = path.read_text(encoding="utf-8").splitlines()
    in_proptest = False
    for i, line in enumerate(lines):
        if PROPTEST.match(line):
            in_proptest = True
        elif in_proptest and line.startswith("}"):
            in_proptest = False
        if not ATTR.match(line):
            continue
        for j in range(i + 1, min(i + 12, len(lines))):
            m = FN.match(lines[j])
            if m:
                kind = "proptest" if in_proptest else ("async" if "tokio" in line else "test")
                yield j + 1, m.group(1), kind
                break


total = 0
for group, crates in GROUPS.items():
    rows = []
    for crate in crates:
        for path in sorted((ROOT / "crates" / crate).rglob("*.rs")):
            rel = path.relative_to(ROOT)
            for line, name, kind in tests_of(path):
                rows.append(f"{rel}\t{line}\t{name}\t{kind}")
    out = HERE / "data" / f"inventory-{group}.tsv"
    out.write_text("path\tline\tname\tkind\n" + "\n".join(rows) + "\n", encoding="utf-8")
    print(f"{group}\t{len(rows)}")
    total += len(rows)
print(f"total\t{total}")
