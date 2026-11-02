#!/usr/bin/env python3
"""The daemon's component census (C158, a daemon scenario crosses the
daemon's frontend boundary only; a test that needs another boundary says
which and stays a component test): for each attributed test under
`crates/acquisition-daemon/src/daemon/tests`, whether it enters by the
door or says which boundary keeps it off (#152). Report-only, not a
gate stage until a second census wants one (evidence before ceremony).

Usage: python3 -I tools/daemon-tests-census.py [--all] [DIR]
       python3 -I tools/daemon-tests-census.py --self-test

DIR defaults to the daemon's test directory; another (an exported tree)
gives the census at another commit. Each test that says neither is
printed with its file and line, every test with `--all`; the totals
last. Exit 0 whatever the count.

The rules:

- Its scope is every `.rs` file in DIR, `door.rs` and `harness.rs`
  included.
- A test is an attributed function, counted once: `#[test]` or
  `#[tokio::test]` (with any arguments), with only other attributes
  between it and the `fn`; each attribute is read whole, from its `#[`
  to the `]` that balances it, however many lines it spans
  (`--self-test` runs the reader over a sample holding both spellings). No case loop is expanded: the census counts
  tests, never rows.
- What a test reaches is its body and every function of the same file
  its body calls, followed to any depth, cycle-safe (each function
  visited once per test). A function of another file is not followed,
  so a test that reaches a constructor only through another file's
  helper says neither until that helper takes the boundary from its
  caller (the value then appears at the test's own call).
- Door: what it reaches opens `Door::open(` or runs a builder,
  `.run(` taking a closure (`|…|` or `move |…|`), the one shape a
  `DoorBuilder` runs.
- Boundary: what it reaches calls `test_daemon(`, `test_daemon_with(` or
  `test_daemon_at(`, whose first argument is `Boundary::<Variant>`, or a
  parameter of the calling function that resolves, through the calls
  that reached it, to the value the test passed.
- Sentence: "a component test", "a component case" or "component table"
  (any case, across wrapped lines) in the test's own doc (the `///`
  lines above its attributes) or its file's module doc (`//!`). A
  helper's own doc is no sentence for the tests that call it.
- One class per test, the first that holds: door, boundary, sentence,
  neither. A test with a value and a sentence counts under the value.

Comments, strings and character literals are blanked before code is
read, so a brace or a call inside them moves nothing.
"""

import glob
import os
import re
import sys

CONSTRUCTORS = ("test_daemon", "test_daemon_with", "test_daemon_at")
SENTENCE = re.compile(r"\ba\s+component\s+(?:test|case)\b|\bcomponent\s+table\b", re.IGNORECASE)
ATTR_TEST = re.compile(r"\s*#\[(?:tokio::)?test\b")
ATTR = re.compile(r"\s*#\[")
FN = re.compile(r"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:<[^>{]*>)?\s*\(")
CALL = re.compile(r"(?<![\w.:])([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(")
DOOR = re.compile(r"\bDoor::open\s*\(|\.run\s*\(\s*(?:move\s*)?\|")
BOUNDARY = re.compile(r"Boundary::([A-Z][A-Za-z0-9_]*)")
IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")


def blank(text):
    """The text with comments, strings and char literals spaced out,
    newlines kept, so offsets and line numbers stand."""
    out = list(text)
    i, n = 0, len(text)

    def space(a, b):
        for k in range(a, b):
            if out[k] != "\n":
                out[k] = " "

    while i < n:
        c = text[i]
        if text.startswith("//", i):
            j = text.find("\n", i)
            j = n if j < 0 else j
            space(i, j)
            i = j
        elif text.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if text.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif text.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            space(i, j)
            i = j
        elif (m := re.compile(r'b?r(#*)"').match(text, i)) and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")):
            end = text.find('"' + m.group(1), m.end())
            j = n if end < 0 else end + 1 + len(m.group(1))
            space(i, j)
            i = j
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            space(i, j + 1)
            i = j + 1
        elif c == "'" and (m := re.compile(r"'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}|.)|[^\\'\n])'").match(text, i)):
            space(i, m.end())
            i = m.end()
        else:
            i += 1
    return "".join(out)


def matching(code, at, open_, close):
    """The offset just past the bracket closing the one at `at`."""
    depth = 0
    for k in range(at, len(code)):
        if code[k] == open_:
            depth += 1
        elif code[k] == close:
            depth -= 1
            if depth == 0:
                return k + 1
    return len(code)


def split_args(text):
    """Top-level comma-separated pieces of an argument list's inside."""
    parts, depth, start = [], 0, 0
    for k, c in enumerate(text):
        if c in "([{<":
            depth += 1
        elif c in ")]}>":
            depth -= 1
        elif c == "," and depth == 0:
            parts.append(text[start:k])
            start = k + 1
    parts.append(text[start:])
    return [p.strip() for p in parts if p.strip()]


def param_name(param):
    name = param.split(":", 1)[0].strip()
    name = re.sub(r"^(?:mut\s+|&\s*(?:mut\s+)?)", "", name)
    return name if IDENT.fullmatch(name) else None


class File:
    def __init__(self, path, text=None):
        self.path = path
        self.text = open(path, encoding="utf-8").read() if text is None else text
        self.lines = self.text.splitlines()
        self.code = blank(self.text)
        self.fns = {}  # name -> (params, body start, body end)
        for m in FN.finditer(self.code):
            popen = m.end() - 1
            pclose = matching(self.code, popen, "(", ")")
            brace = self.code.find("{", pclose)
            semi = self.code.find(";", pclose)
            if brace < 0 or (0 <= semi < brace):
                continue  # a declaration with no body
            params = [param_name(p) for p in split_args(self.code[popen + 1 : pclose - 1])]
            self.fns.setdefault(m.group(1), (params, brace, matching(self.code, brace, "{", "}"), m.start()))
        module = []
        for line in self.lines:
            if line.startswith("//!"):
                module.append(line[3:])
            elif line.strip() and module:
                break
        self.module_doc = " ".join(module)

    def line_of(self, offset):
        return self.text.count("\n", 0, offset) + 1

    def attributes(self):
        """{first line: last line} of each attribute, 0-based, read whole:
        from its `#[` to the `]` that balances it, across lines."""
        spans, code = {}, self.code.splitlines()
        at = 0
        while at < len(code):
            if ATTR.match(code[at]):
                depth, end = 0, at
                for end in range(at, len(code)):
                    depth += code[end].count("[") - code[end].count("]")
                    if depth <= 0:
                        break
                spans[at] = end
                at = end + 1
            else:
                at += 1
        return spans

    def tests(self):
        """(name, line, own doc) for each attributed test function."""
        found, code, spans = [], self.code.splitlines(), self.attributes()
        inside = {k for start, end in spans.items() for k in range(start, end + 1)}
        for at, end in spans.items():
            if not ATTR_TEST.match(code[at]):
                continue
            k = end + 1
            while k in spans:
                k = spans[k] + 1
            m = FN.search(code[k]) if k < len(code) else None
            if not m:
                continue
            doc, d = [], at - 1
            while d >= 0 and (self.lines[d].lstrip().startswith("///") or d in inside):
                if self.lines[d].lstrip().startswith("///"):
                    doc.insert(0, self.lines[d].lstrip()[3:])
                d -= 1
            found.append((m.group(1), k + 1, " ".join(doc)))
        return found

    def reach(self, name):
        """(door, set of boundary values) the function `name` reaches."""
        door, values, seen = False, set(), set()

        def resolve(expr, env):
            expr = expr.strip()
            if m := BOUNDARY.fullmatch(expr):
                return m.group(1)
            return env.get(expr)

        def visit(fn, env):
            nonlocal door
            if fn in seen or fn not in self.fns:
                return
            seen.add(fn)
            params, start, end, _ = self.fns[fn]
            body = self.code[start:end]
            if DOOR.search(body):
                door = True
            for m in CALL.finditer(body):
                callee = m.group(1)
                popen = start + m.end() - 1
                args = split_args(self.code[popen + 1 : matching(self.code, popen, "(", ")") - 1])
                if callee in CONSTRUCTORS:
                    value = resolve(args[0], env) if args else None
                    if value:
                        values.add(value)
                elif callee in self.fns and callee != fn:
                    callee_params = self.fns[callee][0]
                    inner = {}
                    for p, a in zip(callee_params, args):
                        if p and (v := resolve(a, env)):
                            inner[p] = v
                    visit(callee, inner)

        visit(name, {})
        return door, values


SAMPLE = '''\
/// A component test: its attribute spans three lines.
#[tokio::test(
    flavor = "multi_thread"
)]
async fn spanning() {}

#[test]
fn single() {}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[allow(
    dead_code
)]
async fn single_with_arguments() {}
'''


def self_test():
    """The reader over `SAMPLE`: an attribute spanning lines, a bare one
    and one with arguments under others, each counted once, by name."""
    f = File("sample.rs", SAMPLE)
    found = {name: doc for name, _, doc in f.tests()}
    expected = {"spanning", "single", "single_with_arguments"}
    ok = set(found) == expected and bool(SENTENCE.search(found.get("spanning", "")))
    print(f"self-test: found {sorted(found)}, expected {sorted(expected)}: {'ok' if ok else 'FAILED'}")
    return 0 if ok else 1


def main(argv):
    if "--self-test" in argv:
        return self_test()
    show_all = "--all" in argv
    rest = [a for a in argv if a != "--all"]
    root = rest[0] if rest else "crates/acquisition-daemon/src/daemon/tests"
    paths = sorted(glob.glob(os.path.join(root, "*.rs")))
    if not paths:
        print(f"no .rs files under {root}", file=sys.stderr)
        return 2
    totals = {"door": 0, "boundary": 0, "sentence": 0, "neither": 0}
    by_value = {}
    for path in paths:
        f = File(path)
        module_sentence = bool(SENTENCE.search(f.module_doc))
        for name, line, doc in f.tests():
            door, values = f.reach(name)
            if door:
                kind = "door"
            elif values:
                kind = "boundary"
                for v in values:
                    by_value[v] = by_value.get(v, 0) + 1
            elif SENTENCE.search(doc) or module_sentence:
                kind = "sentence"
            else:
                kind = "neither"
            totals[kind] += 1
            if show_all or kind == "neither":
                said = f"boundary {'+'.join(sorted(values))}" if kind == "boundary" else kind
                print(f"{os.path.basename(path)}:{line} {name} {said}")
    count = sum(totals.values())
    values = ", ".join(f"{v} {n}" for v, n in sorted(by_value.items()))
    print(
        f"{count} tests in {len(paths)} files: door {totals['door']}, "
        f"boundary {totals['boundary']}{f' ({values})' if values else ''}, "
        f"sentence {totals['sentence']}, neither {totals['neither']}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
