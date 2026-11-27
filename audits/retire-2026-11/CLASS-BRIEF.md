# The census of the registry: the class of each entry

You are one of two readers who, blind to each other, say for every
entry of this repository's decision registry (`AGENTS.md`,
"Cross-cutting rulings", and `decisions/<area>.md`; 193 entries) whose
it is under today's rules, and which of its clauses are mechanism.
The owner wants every ruling that today's rules would not enter to
retire. Your answer decides which entries later readers open the tests
for, so an entry you call a session's is read in depth, and one both
of you call the owner's stays an entry.

**You change nothing.** No edit, no file written, no `cargo`, no test
run, no `acq`, no gate, no commit. You judge from the entries' own
text. Where the text does not decide, `python3 -I tools/bothy.py show C<n>`
lists where code and tests name the id, and you may open what it
names. Read nothing under `audits/retire-2026-11/` but this file and
the output of its `scripts/registry.py`.

## Read first

- `decisions/process.md`: C170 (who decides), C182 (structure before a
  rule), C192 (a property and its mechanism), and the "Retired" section
  at its foot: seven rulings that have retired, for the kind that does.
- `AGENTS.md`: the invariants and "Rulings first, findings after".
- The entries: `python3 -I audits/retire-2026-11/scripts/registry.py clauses all`
  prints every entry's clauses, numbered, with its *Why:* and its
  *Pinned:*. Use those numbers.

## 1. Each clause: a promise or a mechanism

- **P, a promise:** somebody outside the code that implements it
  relies on it: GGG, a user, an agent at a frontend, the owner's data,
  another crate or process, a later session designing the next thing.
- **M, a mechanism:** it names a knob, a type, a constant, a count, a
  file or a function, or says how the code does it, and nobody outside
  that code would notice a different way of keeping the same promise
  (C192). A number a user or GGG sees is P; a number only the code
  sees is M.
- **A restatement is M.** A clause that only says again what another
  entry, an invariant or a "Not doing" line already states, with or
  without citing it, is M, and gives its entry no class.

For an entry about how the work is run (a process ruling), every
clause is P: C192 is about code.

## 2. The class: whose it is under today's rules

Strip the entry to its own promises, the headline's first, and ask who
is promised. Take the first class that fits.

Owner's classes (C170). Today's rules enter these:

- `ggg` — GGG or the relationship: what the daemon sends, to where,
  how often, whether it retries, what it presents, where a token goes
  or rests. An invariant, or what guards one.
- `data` — the owner's real store: that intent, or a fact a user
  cannot refetch, is never lost, clobbered or silently changed.
- `edge` — a structural boundary: which crate, process or layer may
  link, call, own or mutate what; that a contract exists between two
  of them and how it may change.
- `scope` — what the product does or refuses to do at all, including
  a surface it will not touch. The product's, not a verb's: "the tool
  never posts to the forum" is scope; "this verb refuses that input"
  is design.
- `rule` — what only people keep: who decides, how many reviews a
  thing gets, when a session may commit or push, what a live run is.
- `cross` — a promise more than one frontend or crate must each keep,
  where no single holder can state it. The test: **a new frontend or
  crate could break it and no existing test or check would fail.** If
  the behaviour lives in one shared crate that every frontend
  inherits, or one check walks every frontend, it is not `cross`.

A session's. Today's rules do not enter these:

- `design` — agent-facing design: a verb, a flag, an MCP tool, a query
  word, the words or the shape of an answer, a page.
- `mech` — how one crate does its work, with no promise beyond it.
- `check` — the subject of the entry is a check, a hook, a gate stage,
  a workflow, a test harness or a tool, and the entry says what that
  machine does or how tests are written. The seven retired rulings
  were of this class.
- `run` — how the work is run, outside what `rule` names.

An entry whose own promises fall in two classes takes the owner's one.
A promise that is only a restatement gives no class. The class is
about the promise, never about how well it is held: a `ggg` entry held
by forty tests is still `ggg`, and a `design` entry nothing holds is
still `design`.

## The report

Your final message is the report itself, with nothing before it: one
table, every entry in the order the script prints them, then two short
sections.

```
## Rows
| id | class | M | restated | conf | else | who is promised what |
| C19 | ggg | - | - | high | - | GGG: a send after a headerless answer is paced as counted |
| C141 | design | - | - | high | - | an agent or a page: the league may ride beside the query |
```

- `M`: the numbers of the clauses that are M, comma-separated; `-` for none.
- `restated`: those of the M clauses that are restatements; `-` for none.
- `conf`: `high`, `medium` or `low`.
- `else`: where `conf` is not `high`, the other class it could be.
- the last column: one line, the promisee, a colon, the promise.

Then `## Where the rule did not decide`: each entry you could have
classed two ways, the two classes and the sentence of the rule that
left it open. Then `## Noticed`: anything else a later reader should
know. An honest `low` is worth more than a confident guess.
