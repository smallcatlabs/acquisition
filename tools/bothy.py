#!/usr/bin/env python3
"""bothy — a linter and reader for a markdown decision registry.

    python3 -I tools/bothy.py check                 lint the registry; exit 1 on a fault
    python3 -I tools/bothy.py list [decisions|parks] [--area NAME]   the index: each headline whole
    python3 -I tools/bothy.py show C<n>             one entry, what it cites, what cites it, where code holds it
    python3 -I tools/bothy.py pack --area store     what to read before touching an area
    python3 -I tools/bothy.py pack --path crates/acquisition-store/src/lib.rs   the rulings the file cites, first
    python3 -I tools/bothy.py --root DIR ...        the tree to read (default: the script's parent directory)

The registry is the root file, `AGENTS.md` (the one file every session
loads), and `decisions/<area>.md`, one file per area: its rulings, then
"Not doing". From the root this script reads "What Acquisition is"
(the orientation), "Invariants" (numbered items, each to the next),
"Cross-cutting rulings" (the entries every area must know) and
"Principles": three bullets, each ending with the `P<n>` ids it folds,
so a citation of P3 resolves to the bullet that carries it. The areas
are the files under `decisions/`; there is no index to keep. This
script reads that grammar and never writes.

The entry rule. An entry is one bullet,
`- **C<n> — title.** ruling. *Why:* one sentence. *Pinned:* the test or check that holds it.`

1. Plain declaratives: a subject, a verb, one idea per sentence.
2. One *Why:*, the reason and nothing else.
3. *Pinned:* names the file and the test name or prefix that holds the
   property; a ruling nothing holds says `nothing yet`. There is no
   *Details:* (`show` lists where the code names the id) and no
   *Evidence:* (a claim is cited in the ruling or the *Why:* with its
   gloss).
4. A gloss of two to five words on every cited id, the first time it
   appears in the entry: `C53 (three output levels)`. The gloss helps
   the reader and binds nothing.
5. No history: no date, no commit, no owner's quote in any form (his
   words, or him named as the source: "(the owner, #166)", "Under the
   owner's word."), no "Decided", "Ruled", "Amended" or "Kept", no seat
   call number, no measurement, no note citation. `git log -S'C<n>'` and the commit messages hold
   it, and docs-check refuses the markers. A ruling whose build waits
   says `Unbuilt` and names its issue.
6. A shape, held by `check`, never a size. The headline (the bold
   sentence after the id) states the rule whole in at most 160 bytes.
   The headline and the ruling together are at most six clauses. A
   clause is counted on the text before *Why:*, each code span read as
   one word and parenthesised text left out, split at `. ` or `; ` and
   at a final `.` or `;`. An entry that needs more is two rulings: the
   second takes a new id, by the owner's ruling.
7. *Pinned:* names at most three files, and every name it ticks
   exists. A ticked token with a `/` or a file extension is a path, a
   file or a directory, written as the registry writes one: from the
   tree, from `crates/`, from one crate, or a bare file name that a
   file under `crates/`, `tools/` or `.github/` carries. Any other
   ticked token occurs in a file the same *Pinned:* names, or under a
   directory it names, as written or by its last `::` segment, so a
   prefix is written `c35_`; where no file is named, it occurs under
   `crates/`, `tools/` or `.github/`. Past three holders, keep a
   structural check, then the test at the lowest crate that holds the
   property, then a test whose name carries the id; `show` lists the
   rest.
8. `decisions/shape-baseline.tsv` is the entries that were over a limit
   of rule 6 when `check` first held the shape, one row each: id,
   headline bytes, clauses, tab-separated. The file only shrinks. A row
   lets its entry stay as far over a limit as the row says and no
   further; where a row's number is within the limit, the limit holds.
   An entry with no row meets both limits, so no row is ever added. A
   row whose entry now meets both is stale and is removed.
9. The owner's words stay the authority: an entry's ruling and its
   *Why:* change only by his ruling, and the commit that changes them
   quotes him. Its *Pinned:* is kept true by whoever moves a holder.

The invariants are held like entries: each numbered item of `AGENTS.md`
carries one *Pinned:*, under rule 7.

The not-doing rule. "Not doing" at the foot of an area file is one
bullet per thing someone would propose and the project declines:
`- <item>: not until <trigger>.`, or `- <item>: refused …` for what no
trigger reopens. The backlog is GitHub issues, never this list.

`pack --path` prints the entries the file cites by id, whole, then the
headlines of its area's other entries, then the area's "Not doing".
Neither `pack` form repeats `AGENTS.md`, which every session has
loaded. It routes a file by the `decisions/<area>.md` its module doc
names near the top; a file that names none is routed by the area intros
with a ticked token its path ends with, whole path components, or one
that starts with its crate, so a crate path in an intro routes every
unrouted file of that crate there (the 2d build: plans' intro names
`kinds.rs` bare for that reason). A token ending with the file's name is
not one: `search/README.md` names no other `README.md` (#106). A root
file the workspace reads, a `.toml` or `.lock` at the root that nothing
routes, reaches every member `Cargo.toml` lists, and is routed to the
areas their entry files name (#236: `pack --path Cargo.toml` refused).

`check` holds what `tools/docs-check.sh` does not: a cross-reference
inside the registry resolves; an entry has its *Why:* and its *Pinned:* and no
*Details:* or *Evidence:*; an entry keeps its shape or stays within its
baseline row; a *Pinned:*, an entry's or an invariant's, names at most
three files and only names that exist; a not-doing line states its
trigger or its refusal and is not struck through; a path the registry
names exists; a module doc that names an area names one that exists.
docs-check keeps the history markers, the citations from code, the
uncited ids and the count of cross-cutting rulings. The two do not
overlap; a fault has one reporter.

Faults print as `FAIL  <kind>  <file>:<line>  <what is wrong and what to do>`.
Notes print as `note  ...` and never fail. Standard library only.
`tools/bothy-breakers.sh` proves each refusal on a fixture; run it when
a check is added here.
"""
import argparse
import glob
import os
import re
import sys
import tomllib
from collections import defaultdict

DEC_RE = re.compile(r"^- \*\*(C\d+)\s+—\s+(.*?)\*\*\s*(.*)$")
DEC_START = re.compile(r"^- \*\*C\d")
PRIN_RE = re.compile(r"^- \*\*(.+?)\*\*\s*(.*)$")
PRIN_IDS = re.compile(r"\(((?:P\d+)(?:,\s*P\d+)*)\)\.?\s*$")
INV_RE = re.compile(r"^(\d+)\. \S")
ROOT = "AGENTS.md"
ID_RE = re.compile(r"\b([CP])(\d+)\b")
MARKER_RE = re.compile(r"\*(Why|Details|Pinned|Evidence):\*")
NOT_DOING_RE = re.compile(r"^(.+?):\s+(not until|refused)\b\s*(.*)$")
NOT_DOING = "Not doing"
TICK_RE = re.compile(r"`([^`\s]+)`")
# A path the registry names: has a `/`, and its last component has a file
# extension or its first component is a directory of the tree. Not an API
# route (`/profile`), not a key shape (`wanted/lot`), and not history
# (`spikes/rust-playground/search/BUILD-PLAN.md` at `453a4429`).
PATH_EXT = re.compile(r"\.(rs|md|sh|py|sql|json|toml|csv|txt)$")
HISTORY = re.compile(r"` at `[0-9a-f]{7,}`")
MODDOC_AREA = re.compile(r"decisions/(\w+)\.md")
# The entry's shape (the entry rule, 6 to 8). A *Pinned:* is read with its
# own two patterns: a ticked token there may hold a space (a table row's
# name), and a file is any token with a `/` or one of these extensions.
HEAD_LIMIT = 160
CLAUSE_LIMIT = 6
PIN_FILES = 3
BASELINE = "decisions/shape-baseline.tsv"
PIN_TICK = re.compile(r"`([^`]+)`")
PIN_PATH = re.compile(r"/|\.(rs|sh|py|toml|yml|yaml|md|txt|tsv|json)$")
SEARCHED = ("crates", "tools", ".github")
CODE_EXT = (".rs", ".sh", ".py", ".sql", ".toml")


class Registry:
    def __init__(self, root):
        self.root = root
        self.faults = []      # (kind, file, line, message)
        self.notes = []
        self.decisions = {}   # id -> dict
        self.order = []       # ids in file order
        self.parks = []       # the not-doing lines, each a dict
        self.principles = {}  # P<n> -> the bullet that carries it
        self.invariants = []  # (n, text: the item's lines, the line it starts on)
        self.areas = {}       # area -> {file, title, intro, sections: [(heading, line)], park_heading}
        self.orientation = ""
        self.cross_heading = ""
        self._read_root()
        for name in sorted(os.listdir(self.path("decisions"))) if os.path.isdir(self.path("decisions")) else []:
            if name.endswith(".md"):
                self._read_area(name[:-3])
        self.code = None
        self.searched = None  # every file under SEARCHED, walked once
        self._text = {}       # a file's text, read once

    def path(self, rel):
        return os.path.join(self.root, rel)

    def fail(self, kind, file, line, msg):
        self.faults.append((kind, file, line, msg))

    # ---- reading ---------------------------------------------------------
    def _lines(self, rel):
        with open(self.path(rel), encoding="utf-8") as fh:
            return [l.rstrip("\n") for l in fh]

    def _read_root(self):
        rel = ROOT
        if not os.path.exists(self.path(rel)):
            self.fail("missing", rel, 0, f"no {ROOT}: the registry starts there")
            return
        section = ""
        orient = []
        for n, line in enumerate(self._lines(rel), 1):
            if line.startswith("## "):
                section = line[3:]
                if section.startswith("Cross-cutting"):
                    self.cross_heading = line
                continue
            if section.startswith("What Acquisition is"):
                if line.strip():
                    orient.append(line)
                continue
            if section.startswith("Invariants"):
                if INV_RE.match(line):
                    self.invariants.append((int(line.split(".")[0]), line, n))
                elif line.startswith("   ") and line.strip() and self.invariants:
                    k, text, at = self.invariants[-1]
                    self.invariants[-1] = (k, text + "\n" + line, at)
                continue
            if section.startswith("Principles"):
                m = PRIN_RE.match(line)
                if m:
                    ids = PRIN_IDS.search(line)
                    if not ids:
                        self.fail("principle", rel, n, "a principle ends with the ids it folds, `(P1, P6).`; "
                                  "this one names none, so no citation can reach it")
                        continue
                    for pid in re.findall(r"P\d+", ids.group(1)):
                        self.principles[pid] = line[2:]
                continue
            self._entry(rel, "cross-cutting", n, line, section, False)
        self.orientation = "\n".join(orient)

    def _read_area(self, area):
        rel = f"decisions/{area}.md"
        lines = self._lines(rel)
        meta = {"file": rel, "title": "", "intro": "", "sections": [], "park_heading": "", "park_intro": ""}
        self.areas[area] = meta
        section = ""
        in_parked = False
        for n, line in enumerate(lines, 1):
            if line.startswith("# ") and not meta["title"]:
                meta["title"] = line[2:]
                continue
            if line.startswith("## "):
                section = line[3:]
                in_parked = section.startswith(NOT_DOING)
                if in_parked:
                    meta["park_heading"] = line
                else:
                    meta["sections"].append((line, n))
                continue
            if not section and line.strip() and not meta["intro"]:
                meta["intro"] = line
            if in_parked and not line.startswith("- ") and line.strip() and not meta["park_intro"]:
                meta["park_intro"] = line
            self._entry(rel, area, n, line, section, in_parked)

    def _entry(self, rel, area, n, line, section, in_parked):
        if in_parked and line.startswith("- "):
            body = line[2:]
            park = {"file": rel, "line": n, "area": area, "text": line, "body": body,
                    "item": None, "kind": None, "trigger": None,
                    "cites": sorted({a + b for a, b in ID_RE.findall(body)})}
            m = NOT_DOING_RE.match(body)
            if m:
                park["item"], park["kind"], park["trigger"] = m.group(1).strip(), m.group(2), m.group(3).strip()
            self.parks.append(park)
            return
        if in_parked:
            return
        m = DEC_RE.match(line)
        if DEC_START.match(line) and not m:
            self.fail("grammar", rel, n,
                      "starts like a decision but is not `- **C<n> — title.** ruling ...`; "
                      "fix the bullet or move the text out of the registry")
            return
        if not m:
            return
        cid, title, body = m.group(1), m.group(2), m.group(3)
        cites = sorted({a + b for a, b in ID_RE.findall(body)} - {cid})
        entry = {"id": cid, "file": rel, "line": n, "area": area, "section": section,
                 "title": title, "body": body, "text": line, "cites": cites,
                 "markers": [mk for mk in MARKER_RE.findall(body)]}
        if cid in self.decisions:
            first = self.decisions[cid]
            self.fail("duplicate", rel, n,
                      f"{cid} is also at {first['file']}:{first['line']}; an id is never reused, "
                      "so one of these takes the next free id")
            return
        self.decisions[cid] = entry
        self.order.append(cid)

    # ---- the code --------------------------------------------------------
    def read_code(self):
        """Where each id is named in the code and tools, and which module docs
        name which area. Walks `crates/` (not target) and `tools/`."""
        if self.code is not None:
            return self.code
        held = defaultdict(list)      # id -> [(file, line)]
        moddoc = defaultdict(list)    # area -> [file]
        files = []                    # every file under crates/ and tools/
        for top in ("crates", "tools"):
            for dirpath, dirs, names in os.walk(self.path(top)):
                dirs[:] = [d for d in dirs if d not in ("target", "__pycache__")]
                for name in names:
                    full = os.path.join(dirpath, name)
                    rel = os.path.relpath(full, self.root)
                    files.append(rel)
                    if not name.endswith(CODE_EXT):
                        continue
                    try:
                        with open(full, encoding="utf-8", errors="replace") as fh:
                            for n, line in enumerate(fh, 1):
                                for cid in re.findall(r"\bC\d+\b", line):
                                    held[cid].append((rel, n))
                                if n <= 200 and name.endswith(".rs"):
                                    for area in MODDOC_AREA.findall(line):
                                        if rel not in moddoc[area]:
                                            moddoc[area].append(rel)
                    except OSError:
                        pass
        self.code = {"held": held, "moddoc": moddoc, "files": files,
                     "by_base": defaultdict(list)}
        for rel in files:
            self.code["by_base"][os.path.basename(rel)].append(rel)
        return self.code

    def git_root(self):
        d = self.root
        while d != os.path.dirname(d):
            if os.path.exists(os.path.join(d, ".git")):
                return d
            d = os.path.dirname(d)
        return self.root

    def is_path(self, tok):
        """Whether a backticked token is a path this check should resolve."""
        if "/" not in tok or tok.startswith(("/", "http", "~", "-", "../")) or re.search(r"[<>*{}$]", tok):
            return False
        tok = tok.split("#")[0].split(":")[0]
        first, last = tok.split("/")[0], tok.rsplit("/", 1)[-1]
        return bool(PATH_EXT.search(last)) or os.path.isdir(self.path(first)) or os.path.isdir(os.path.join(self.git_root(), first))

    def resolve_path(self, tok):
        """A path the registry names, as the registry writes it: relative to
        the tree, to `crates/`, to one crate, or to the repository root."""
        tok = tok.split("#")[0].split(":")[0]
        bases = [self.root, self.path("crates"), self.git_root()]
        if os.path.isdir(self.path("crates")):   # `tests/wire.rs`, written after the crate's name
            bases += [self.path(os.path.join("crates", c)) for c in os.listdir(self.path("crates"))]
        return any(os.path.exists(os.path.join(base, tok)) for base in bases)

    def searched_files(self):
        """Every file under `crates/` (not target), `tools/` and `.github/`:
        where a *Pinned:* that names no file must find what it ticks, and
        where a bare file name is looked up."""
        if self.searched is None:
            self.searched = []
            for top in SEARCHED:
                self.searched += self._under(self.path(top))
        return self.searched

    def _under(self, top):
        found = []
        for dirpath, dirs, names in os.walk(top):
            dirs[:] = [d for d in dirs if d not in ("target", "__pycache__")]
            found += [os.path.join(dirpath, name) for name in sorted(names)]
        return found

    def text(self, full):
        if full not in self._text:
            try:
                with open(full, encoding="utf-8", errors="replace") as fh:
                    self._text[full] = fh.read()
            except OSError:
                self._text[full] = ""
        return self._text[full]

    def pinned_files(self, tok):
        """The files a path in a *Pinned:* names: the file itself, every file
        under a directory, or every file that carries a bare name. Empty
        means the path exists nowhere."""
        tok = tok.split("#")[0].split(":")[0]
        if not tok or tok.startswith(("/", "~")) or ".." in tok.split("/"):
            return []
        bases = [self.root, self.path("crates"), self.git_root()]
        if os.path.isdir(self.path("crates")):
            bases += [self.path(os.path.join("crates", c)) for c in sorted(os.listdir(self.path("crates")))]
        found = []
        for base in bases:
            full = os.path.join(base, tok)
            if os.path.isfile(full):
                found.append(full)
            elif os.path.isdir(full):
                found += self._under(full)
        if not found and "/" not in tok:
            found = [f for f in self.searched_files() if os.path.basename(f) == tok]
        return list(dict.fromkeys(found))

    def cited_by(self, rel):
        """The entries a file cites by id, in the registry's order."""
        full = self.path(rel)
        if not os.path.isfile(full):
            return []
        named = set(re.findall(r"\bC\d+\b", self.text(full)))
        return [i for i in self.order if i in named]

    def members(self):
        """The workspace manifest's members, as `Cargo.toml` lists them (a
        glob expanded); none where the root has no manifest."""
        full = self.path("Cargo.toml")
        if not os.path.isfile(full):
            return []
        with open(full, "rb") as fh:
            listed = tomllib.load(fh).get("workspace", {}).get("members", [])
        found = []
        for m in listed:
            for d in sorted(glob.glob(m, root_dir=self.root)) if glob.has_magic(m) else [m]:
                if os.path.isfile(self.path(f"{d}/Cargo.toml")) and d not in found:
                    found.append(d)
        return found

    def workspace_areas(self):
        """The areas a root file the workspace reads reaches: each member's,
        read off its entry file's module doc as any file's is."""
        found = []
        for m in self.members():
            for entry in (f"{m}/src/lib.rs", f"{m}/src/main.rs"):
                if os.path.isfile(self.path(entry)):
                    found += [a for a in self.area_of_path(entry) if a not in found]
                    break
        return found

    def area_of_path(self, rel):
        """The area a file belongs to: the module doc names `decisions/<area>.md`
        near the top, else the area whose intro names the file or its crate."""
        full = self.path(rel)
        found = []
        if os.path.exists(full):
            with open(full, encoding="utf-8", errors="replace") as fh:
                for n, line in enumerate(fh, 1):
                    if n > 200:
                        break
                    for area in MODDOC_AREA.findall(line):
                        if area in self.areas and area not in found:
                            found.append(area)
        if found:
            return found
        crate = rel.split("/")[1] if rel.startswith("crates/") and rel.count("/") > 1 else None
        for area, meta in self.areas.items():
            toks = TICK_RE.findall(meta["intro"])
            if any(rel == t or rel.endswith("/" + t) for t in toks) or (crate and any(t.startswith(crate) for t in toks)):
                found.append(area)
        return found


# ---- check -----------------------------------------------------------------
def section_text(body, marker):
    """The text of an entry's one `*<marker>:*` section, up to the next
    marker or the end, without its punctuation: empty means empty."""
    m = re.search(r"\*" + marker + r":\*(.*?)(?=\*(?:Why|Details|Pinned|Evidence):\*|$)", body)
    return bare(m.group(1)) if m else ""


def bare(text):
    """Text without its edge punctuation and whitespace of every kind
    (a tab, a no-break space): what is left is what was written."""
    return re.sub(r"^[\s.;,]+|[\s.;,]+$", "", text)


def clauses(title, body):
    """The clauses of an entry: its headline and its ruling, the text before
    `*Why:*`, each code span read as one word and parenthesised text left
    out, split at `. ` or `; ` and at a final `.` or `;`."""
    text = title + " " + body.partition("*Why:*")[0]
    text = re.sub(r"\([^)]*\)", "", re.sub(r"`[^`]*`", "X", text))
    return [c for c in re.split(r"[.;]\s+|[.;]$", text.strip()) if c.strip()]


def read_baseline(reg):
    """`decisions/shape-baseline.tsv`: id -> (headline bytes, clauses, line).
    No file is an empty baseline."""
    rows = {}
    if not os.path.exists(reg.path(BASELINE)):
        return rows
    for n, line in enumerate(reg._lines(BASELINE), 1):
        if not line.strip() or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) != 3 or not re.fullmatch(r"C\d+", parts[0]) or not (parts[1].isdigit() and parts[2].isdigit()):
            reg.fail("baseline", BASELINE, n, "a row is `C<n>`, the headline's bytes and the clauses, tab-separated; "
                     "fix the row, or remove it if its entry meets the limits")
        elif parts[0] in rows:
            reg.fail("baseline", BASELINE, n, f"{parts[0]} has a row at line {rows[parts[0]][2]} too; an entry has one row: remove this one")
        else:
            rows[parts[0]] = (int(parts[1]), int(parts[2]), n)
    return rows


SPLIT = "say it in less, or split the entry: the second ruling takes a new id, by the owner's ruling"


def check_shape(reg):
    """The entry rule's 6 and 8: a headline of at most 160 bytes and at most
    six clauses, or no further over either than the entry's baseline row."""
    base = read_baseline(reg)
    for i in reg.order:
        d = reg.decisions[i]
        where = (d["file"], d["line"])
        head, count = len(d["title"].encode()), len(clauses(d["title"], d["body"]))
        if i not in base:
            if head > HEAD_LIMIT:
                reg.fail("headline", *where, f"{i}'s headline is {head} bytes, over {HEAD_LIMIT}; a headline states the rule whole in one short sentence: {SPLIT}")
            if count > CLAUSE_LIMIT:
                reg.fail("clauses", *where, f"{i} has {count} clauses before its *Why:*, over {CLAUSE_LIMIT}; "
                         f"route a mechanism to the doc comment that cites {i}, or split the entry: the second ruling takes a new id, by the owner's ruling")
            continue
        row_head, row_count, row_line = base[i]
        if head <= HEAD_LIMIT and count <= CLAUSE_LIMIT:
            reg.fail("baseline", BASELINE, row_line, f"{i} now meets both limits ({head} bytes, {count} clauses), so its row is stale; remove the row")
            continue
        if head > max(HEAD_LIMIT, row_head):
            reg.fail("baseline", *where, f"{i}'s headline is {head} bytes, over the {max(HEAD_LIMIT, row_head)} its row in {BASELINE} allows; "
                     f"the baseline only shrinks, so the row stays as it is: {SPLIT}")
        if count > max(CLAUSE_LIMIT, row_count):
            reg.fail("baseline", *where, f"{i} has {count} clauses, over the {max(CLAUSE_LIMIT, row_count)} its row in {BASELINE} allows; "
                     f"the baseline only shrinks, so the row stays as it is: {SPLIT}")
    for i, (_, _, row_line) in base.items():
        if i not in reg.decisions:
            reg.fail("baseline", BASELINE, row_line, f"{i} has a row and no entry, so the row is stale; remove the row")


def check_pinned(reg, file, line, label, pinned):
    """The entry rule's 7, on the text after one `*Pinned:*`: at most three
    files, and every ticked name exists."""
    ticks = PIN_TICK.findall(pinned)
    paths = [t for t in ticks if PIN_PATH.search(t)]
    if len(paths) > PIN_FILES:
        reg.fail("pinned", file, line, f"{label}'s *Pinned:* names {len(paths)} files, over {PIN_FILES}; keep a structural check, "
                 "then the test at the lowest crate that holds the property, then a test whose name carries the id, and drop the rest: "
                 "`show` lists where the code names the id")
    files, missing = [], False
    for tok in paths:
        found = reg.pinned_files(tok)
        files += found
        if found:
            continue
        missing = True
        if " " not in tok and reg.is_path(tok):
            continue    # the path check below reports it: a fault has one reporter
        reg.fail("pinned", file, line, f"{label}'s *Pinned:* names `{tok}`, which is no file under the tree, crates/ or one crate, "
                 "and no file name under crates/, tools/ or .github/; name the file that holds the property now, or untick what is not a file")
    if missing:
        return    # what the missing file was to hold is judged once it is named
    for tok in ticks:
        if PIN_PATH.search(tok):
            continue
        needles = {tok, tok.rsplit("::", 1)[-1]}
        if not any(n in reg.text(f) for f in (files if paths else reg.searched_files()) for n in needles):
            where = "no file it names holds" if paths else "nothing under crates/, tools/ or .github/ holds"
            reg.fail("pinned", file, line, f"{label}'s *Pinned:* ticks `{tok}`, which {where}; write the test's or the check's name as the code has it now "
                     "(a prefix as `c35_`), name the file that holds it, or untick what is not a name")


def check(reg, show_notes=True):
    # every cross-reference resolves
    for i in reg.order:
        d = reg.decisions[i]
        for c in d["cites"]:
            if c[0] == "C" and c not in reg.decisions:
                reg.fail("cite", d["file"], d["line"], f"{i} names {c}, which no file defines; fix the id or add the decision")
            if c[0] == "P" and c not in reg.principles:
                reg.fail("cite", d["file"], d["line"], f"{i} names {c}, which AGENTS.md's principles do not define")
    for p in reg.parks:
        for c in p["cites"]:
            if c[0] == "C" and c not in reg.decisions:
                reg.fail("cite", p["file"], p["line"], f"a not-doing line names {c}, which no file defines")
            if c[0] == "P" and c not in reg.principles:
                reg.fail("cite", p["file"], p["line"], f"a not-doing line names {c}, which AGENTS.md's principles do not define")
    # every entry has its Why and its Pinned, and no Details or Evidence
    for i in reg.order:
        d = reg.decisions[i]
        where = (d["file"], d["line"])
        for mk in ("Why", "Pinned"):
            n = d["markers"].count(mk)
            if n == 0:
                reg.fail("entry", *where, f"{i} has no *{mk}:*; the entry rule (this script's docstring) asks one"
                         + (", `nothing yet` when nothing holds it" if mk == "Pinned" else ""))
            elif n > 1:
                reg.fail("entry", *where, f"{i} has {n} *{mk}:* sections; the entry rule asks exactly one: merge them")
            elif not section_text(d["body"], mk):
                reg.fail("entry", *where, f"{i} has an empty *{mk}:*; write "
                         + ("the reason" if mk == "Why" else "the test that holds it, or `nothing yet`"))
        for mk in ("Details", "Evidence"):
            if mk in d["markers"]:
                reg.fail("entry", *where, f"{i} carries *{mk}:*; the entry rule has none: `show {i}` lists where the code names it, "
                         "and evidence is cited in the ruling or the *Why:* with its gloss")
    # every entry keeps its shape, or stays within its baseline row
    check_shape(reg)
    # every *Pinned:*, an entry's or an invariant's, names at most three files and only names that exist
    for i in reg.order:
        d = reg.decisions[i]
        if d["markers"].count("Pinned") == 1:
            check_pinned(reg, d["file"], d["line"], i, d["body"].partition("*Pinned:*")[2])
    for k, text, at in reg.invariants:
        flat = " ".join(part.strip() for part in text.split("\n"))
        n = flat.count("*Pinned:*")
        if n != 1:
            reg.fail("invariant", ROOT, at, f"invariant {k} has {n or 'no'} *Pinned:*{' sections' if n else ''}; an invariant ends with one, naming the check or the test that fails "
                     "when it breaks, or `nothing yet`")
        elif not bare(flat.partition("*Pinned:*")[2]):
            reg.fail("invariant", ROOT, at, f"invariant {k} has an empty *Pinned:*; name the check or the test that fails when it breaks, or `nothing yet`")
        else:
            check_pinned(reg, ROOT, at, f"invariant {k}", flat.partition("*Pinned:*")[2])
    # every not-doing line states its trigger or its refusal, and is not struck through
    for p in reg.parks:
        where = (p["file"], p["line"])
        if p["item"] is None:
            reg.fail("notdoing", *where, "a not-doing line is `- <item>: not until <trigger>` or `- <item>: refused …`; "
                     "this one states neither, so it is re-argued every time it is read")
        elif p["kind"] == "not until" and not bare(p["trigger"]):
            reg.fail("notdoing", *where, "this line says `not until` and names no trigger")
        if "~~" in p["body"]:
            reg.fail("notdoing", *where, "a struck-through line is deleted, not kept; delete the bullet")
    # every path the registry names exists
    seen = set()
    for rel in [ROOT] + [m["file"] for m in reg.areas.values()]:
        if not os.path.exists(reg.path(rel)):
            continue
        for n, line in enumerate(reg._lines(rel), 1):
            for m in TICK_RE.finditer(line):
                tok = m.group(1)
                if not reg.is_path(tok) or (rel, tok) in seen or HISTORY.match(line, m.end() - 1):
                    continue
                seen.add((rel, tok))
                if not reg.resolve_path(tok):
                    reg.fail("path", rel, n, f"`{tok}` is named here and exists nowhere under the tree, crates/ or the repository; "
                             "point at the file that holds it now, or cite history as `<path>` at `<commit>`")
    # a module doc that names an area names one that exists
    code = reg.read_code()
    for area, files in code["moddoc"].items():
        if area not in reg.areas:
            for f in files:
                reg.fail("routing", f, 1, f"names decisions/{area}.md, which does not exist; name the area's file")
    # notes: soft shape, reported and never failed
    unrouted = [a for a in reg.areas if a not in code["moddoc"]]
    if unrouted:
        reg.notes.append("no module doc names " + ", ".join(f"decisions/{a}.md" for a in unrouted)
                         + " — `pack --path` reaches the area only through its intro's file list")
    # report
    for kind, file, line, msg in sorted(reg.faults, key=lambda f: (f[1], f[2], f[0])):
        print(f"FAIL  {kind:<9} {file}:{line}  {msg}")
    if show_notes:
        for note in reg.notes:
            print(f"note  {note}")
    if reg.faults:
        print(f"{len(reg.faults)} fault(s) in the registry")
        return 1
    print(f"ok    registry  {len(reg.decisions)} decisions in {1 + len(reg.areas)} files, {len(reg.parks)} not-doing lines, "
          f"{len(set(reg.principles.values()))} principles folding {len(reg.principles)} ids, "
          f"{len(reg.invariants)} invariants: every cross-reference resolves, every entry has its Why and its Pinned and keeps its shape, "
          "every Pinned names at most three files and only names that exist, every not-doing line states its trigger, every path named exists")
    return 0


# ---- list, show ------------------------------------------------------------
def short(s, n=96):
    return s if len(s) <= n else s[: n - 1] + "…"


def cmd_list(reg, what, area):
    if what in ("decisions", "all"):
        for i in reg.order:
            d = reg.decisions[i]
            if area and d["area"] != area:
                continue
            print(f"{i:<5} {d['area']:<14} {d['title']}")
    if what in ("parks", "all"):
        for p in reg.parks:
            if area and p["area"] != area:
                continue
            item = p["item"] or p["body"]
            print(f"not   {p['area']:<14} {item}  [{p['file']}:{p['line']}]")


def cmd_show(reg, cid):
    code = reg.read_code()
    if cid in reg.decisions:
        d = reg.decisions[cid]
        print(f"{d['file']}:{d['line']}  ({d['area']}; {d['section']})\n\n{d['text']}\n")
        if d["cites"]:
            print("Names: " + ", ".join(
                f"{c} ({short(reg.decisions[c]['title'], 60)})" if c in reg.decisions else
                f"{c} ({short(reg.principles.get(c, '?'), 60)})" for c in d["cites"]))
    elif cid in reg.principles:
        print(f"{ROOT}  {cid}: {reg.principles[cid]}\n")
    else:
        print(f"no entry {cid}; `list` shows the ids", file=sys.stderr)
        return 2
    named_by = [i for i in reg.order if cid in reg.decisions[i]["cites"]]
    parks_by = [p for p in reg.parks if cid in p["cites"]]
    if named_by or parks_by:
        print("Named by: " + ", ".join(named_by + [f"a not-doing line at {p['file']}:{p['line']}" for p in parks_by]))
    held = code["held"].get(cid, [])
    if held:
        print(f"Held in code ({len(held)} places): " + ", ".join(f"{f}:{n}" for f, n in held[:12]) + (" …" if len(held) > 12 else ""))
    else:
        print("Held in code: nowhere — no test, doc comment or tool names it")
    return 0


# ---- pack ------------------------------------------------------------------
NOT_REPEATED = ("`AGENTS.md` is not repeated here: every session has it loaded, with the invariants, the cross-cutting rulings "
                "and the principles. What follows is from `decisions/<area>.md`, which is the authority; "
                "`python3 -I tools/bothy.py show C<n>` prints one entry whole.")


def not_doing(reg, a):
    meta, parks = reg.areas[a], [p for p in reg.parks if p["area"] == a]
    if not parks:
        return []
    out = [(meta["park_heading"] or f"## {NOT_DOING}").replace("## ", f"## `{a}`: ", 1), ""]
    if meta["park_intro"]:
        out += [meta["park_intro"], ""]
    return out + [p["text"] for p in parks] + [""]


def cmd_pack(reg, area, path):
    if path:
        return pack_path(reg, path)
    if area not in reg.areas:
        print(f"no area {area}; areas: {', '.join(sorted(reg.areas))}", file=sys.stderr)
        return 2
    code = reg.read_code()
    meta = reg.areas[area]
    title = re.sub(r"^Decisions\s+—\s+", "", meta["title"])
    out = [f"# What to read before touching `{area}`", "", NOT_REPEATED, "",
           f"## Rulings for `{area}` — {title}", "", meta["intro"], ""]
    current = None
    for i in reg.order:
        d = reg.decisions[i]
        if d["area"] != area:
            continue
        if d["section"] != current:
            current = d["section"]
            out += [f"### {current}", ""]
        out.append(d["text"])
    out.append("")
    out += not_doing(reg, area)
    out += ["## Where each ruling is held (file:line that names it; the mechanism lives there)", ""]
    for i in reg.order:
        if reg.decisions[i]["area"] != area:
            continue
        firsts, seen = [], set()
        for f, n in code["held"].get(i, []):
            if f not in seen:
                seen.add(f)
                firsts.append(f"{f}:{n}")
        out.append(f"- {i}: " + (", ".join(firsts[:6]) + (" …" if len(firsts) > 6 else "") if firsts else "nothing names it yet"))
    out.append("")
    mods = sorted(code["moddoc"].get(area, []))
    if mods:
        out += ["## Module docs that name this area (read the one you will change)", ""] + [f"- {m}" for m in mods] + [""]
    print("\n".join(out))
    return 0


def pack_path(reg, path):
    """What a file's own citations lead to: the entries it cites, whole,
    then its area's other headlines, then the area's "Not doing"."""
    full = path if os.path.isabs(path) else reg.path(path)
    rel = os.path.relpath(os.path.abspath(full), reg.root)
    if rel.startswith("..") or not os.path.exists(full):
        print(f"no file {path} in {reg.root}: `pack --path` reads a file that exists, named from the repository's root. "
              f"`git ls-files '*{os.path.basename(path)}'` finds one by its name; `pack --area <area>` needs no file.", file=sys.stderr)
        return 2
    areas = reg.area_of_path(rel)
    # A root file the workspace reads (its manifest, its lock, a tool's
    # `.toml` beside them) is named by no module doc and no intro; it
    # reaches every member the manifest lists, so it is routed to theirs.
    reaches = not areas and "/" not in rel and rel.endswith((".toml", ".lock"))
    if reaches:
        areas = reg.workspace_areas()
    if not areas:
        print(f"no area governs {rel}: no module doc near its top names a decisions/<area>.md and no area's intro names it. "
              f"Areas: {', '.join(sorted(reg.areas))}. Use --area.", file=sys.stderr)
        return 2
    cited = reg.cited_by(rel)
    whole = [i for i in cited if reg.decisions[i]["area"] != "cross-cutting"]
    loaded = [i for i in cited if reg.decisions[i]["area"] == "cross-cutting"]
    out = [f"# What to read before touching `{rel}`", "", NOT_REPEATED, "", f"## The rulings `{rel}` cites", ""]
    out += [reg.decisions[i]["text"] for i in whole]
    if loaded:
        out += ([""] if whole else []) + ["Cited from `AGENTS.md`: " + ", ".join(loaded) + "."]
    if not cited:
        out.append("It cites none by id.")
    out.append("")
    if reaches:
        out += [f"`{rel}` is a root file the workspace reads, so it reaches every member the manifest lists: every "
                "cross-cutting ruling of `AGENTS.md` bears on it, C1's dependency edges first, and the areas its "
                "members name follow.", ""]
    for a in areas:
        meta = reg.areas[a]
        title = re.sub(r"^Decisions\s+—\s+", "", meta["title"])
        rest = [i for i in reg.order if reg.decisions[i]["area"] == a and i not in whole]
        if rest:
            out += [f"## The other rulings of `{a}` — {title} (headlines; `{meta['file']}` has each whole)", ""]
            out += [f"- {i} — {reg.decisions[i]['title']}" for i in rest] + [""]
        out += not_doing(reg, a)
    print("\n".join(out))
    return 0


# ---- main ------------------------------------------------------------------
def main(argv):
    ap = argparse.ArgumentParser(prog="bothy", description=__doc__.split("\n\n")[0], add_help=True)
    ap.add_argument("--root", help="the tree to read (default: the script's parent directory)")
    sub = ap.add_subparsers(dest="verb", required=True)
    c = sub.add_parser("check", help="lint the registry; exit 1 on a fault")
    c.add_argument("--no-notes", action="store_true", help="print faults only")
    l = sub.add_parser("list", help="one line per decision and not-doing line")
    l.add_argument("what", nargs="?", default="all", choices=["all", "decisions", "parks"])
    l.add_argument("--area")
    s = sub.add_parser("show", help="one entry, what it names, what names it, where code holds it")
    s.add_argument("id")
    p = sub.add_parser("pack", help="what to read before touching an area or a file")
    g = p.add_mutually_exclusive_group(required=True)
    g.add_argument("--area")
    g.add_argument("--path")
    args = ap.parse_args(argv)
    root = args.root or os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
    root = os.path.abspath(root)
    reg = Registry(root)
    if args.verb == "check":
        return check(reg, show_notes=not args.no_notes)
    if reg.faults:
        check(reg, show_notes=False)
        print("fix the registry before reading it", file=sys.stderr)
        return 1
    if args.verb == "list":
        return cmd_list(reg, args.what, args.area) or 0
    if args.verb == "show":
        return cmd_show(reg, args.id)
    if args.verb == "pack":
        return cmd_pack(reg, args.area, args.path)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
