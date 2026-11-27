# The census of the registry: one question, asked again

You are a reader in a census of this repository's decision registry
(`AGENTS.md`, "Cross-cutting rulings", and `decisions/<area>.md`). The
owner wants every ruling that today's rules would not enter to retire
to a comment at the test or check that holds it. Readers have judged
each entry clause by clause, and your batch is entries the census
would let go: each is a session's design or mechanism, and its
clauses are held by tests, or would be after a small one.

One class did not hold up. `cross` was "a promise more than one
frontend or crate must each keep, where no single holder can state
it", tested by asking whether a new frontend could break it unseen.
The readers who opened the code used it where the readers of the text
did not, and said why: a promise is broken far more easily by a new
verb, answer, count or page in an existing frontend than by a new
frontend. So the test is sharpened and asked once more, of every
entry that would go.

**You change nothing.** No edit, no file written, no `cargo`, no test
run, no `acq`, no gate, no commit. You read: `Read` with an offset and
a limit, `git grep -n`, `python3 -I tools/bothy.py show C<n>`,
`python3 -I audits/retire-2026-11/scripts/registry.py clauses C<n>`,
and the first readers' reports under `audits/retire-2026-11/reports/first/`,
which give each clause's holder by file and line. Do not read
`reports/second/`. Open code only to check a door or a sweep; never a
test file whole.

## The question

For each entry: **does a promise in it bind things not yet written,
and if one does, would anything fail when a new thing broke it?**

1. Take each clause the first reader marked P. Write its breach as a
   new thing: a new verb, answer, document, count, text, page, tool,
   table, test, frontend or crate, written by a builder who never
   read this entry.
2. Ask what fails without being edited.

Then one verdict for the entry, the worst over its clauses, in this
order:

- `open` — a new thing could break a promise and nothing fails. The
  tests hold the things that exist; the next one is held by nobody
  but a reader of the entry. Name the clause and the new thing.
- `listed` — a new thing is caught only if its builder adds it to a
  list kept by hand (a sweep's inventory, a table of verbs, a list of
  documents). Name the list, file and line.
- `caught` — a new thing could break it and something fails by
  construction: a type the new thing must use, a door every answer
  passes, a sweep that finds its subjects itself, a check in `tools/`
  that reads the tree. Name it, file and line.
- `bound` — no new thing can break it. The promise is about one
  behaviour that exists (a field, a verb's meaning, an evaluator's
  rule, a wire shape), its tests hold that behaviour, and a breach is
  an edit that fails them.

A semantics that lives in one function every caller goes through is
`bound` or `caught`, not `open`: a new caller inherits it. A wording
or a shape each maker writes for itself is `open` unless a sweep or a
door reads every maker's output. "A new frontend could ignore the
shared crate and write its own" is not a breach to count: C46 (every
frontend is an adapter) and its check already refuse it.

## The report

Your final message is the report itself, nothing before it: one table,
your entries in the order given, then two short sections.

```
## Rows
| id | verdict | clause | the new thing that breaks it | what fails, or nothing | conf |
| C163 | listed | 1 | a new verb's document carrying a time with no words | the walk at crates/acquisition-verbs/tests/documents.rs:4031, once the document is added to `populated()` (:3025) | high |
| C141 | bound | - | - | crates/acquisition-search/tests/trade_decode.rs:243 holds the one decode | high |
```

- `clause`: the number of the clause that decides the verdict; `-` for `bound`.
- `conf`: `high`, `medium` or `low`.

Then `## Where the first reader's evidence did not stand`: any holder
you opened that does not assert what the report says. Then
`## Noticed`. An honest `low` is worth more than a confident guess.
