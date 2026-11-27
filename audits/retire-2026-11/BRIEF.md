# The census of the registry: a reader's brief

You are one reader of a census of this repository's decision registry
(`AGENTS.md`, "Cross-cutting rulings", and `decisions/<area>.md`). The
registry holds 193 numbered rulings and none but seven has ever
retired. The owner wants every ruling that today's rules would not
enter to retire, so that later sessions keep track of less. Nobody
knows how many can, or what it costs. Two readers have classed every
entry from its text; your batch is entries at least one of them called
a session's, so each could retire if its tests hold it. You judge
them one at a time by the rule below, reading the tests, and return
one report.

**You change nothing.** No edit, no file written, no `cargo`, no test
run, no `acq`, no gate, no commit. You read: `Read`, `git grep`,
`git log`, `python3 -I tools/bothy.py show C<n>`, and this audit's
`scripts/registry.py`. Under `audits/retire-2026-11/` read this file
and `CLASS-BRIEF.md` and nothing else: another reader is judging some
of your entries blind, and you are blind to them.

Read narrowly. Find a test with `git grep -n`, then `Read` it with an
offset and a limit around what you found; never read a test file, a
fixture or a golden whole. Your context is the budget, and a verdict
made after it runs out is worth nothing.

## Read first

- `decisions/process.md`: C170 (who decides), C182 (structure before a
  rule), C192 (a property and its mechanism), and the "Retired" section
  at its foot.
- `tools/bothy.py`'s docstring: the entry rule and the retired rule.
- `AGENTS.md`: the invariants, "Rulings first, findings after", and
  C122 (a behaviour is pinned once, at the lowest crate that holds it).
- `git show -s 51f2e4f9`: the seven retirements, for what a holder's
  comment looks like once the reason has moved to it.

## For each entry

Run `python3 -I audits/retire-2026-11/scripts/registry.py clauses C<n> ...`
for your batch. It prints each entry's clauses, numbered. Use those
numbers. Then `python3 -I tools/bothy.py show C<n>` lists where code
and tests name the id. Then answer five things, in this order.

### 1. Each clause: a promise or a mechanism

P or M, as `CLASS-BRIEF.md`, section 1, states them (a restatement of
another entry, an invariant or a "Not doing" line is M). Having read
the code, you know better than a reader of the text whether anybody
outside it relies on a clause: say what you find.

### 2. Each clause: is it held

Open the test or the check and read its body. A name is not evidence.
Start from the entry's *Pinned:*, then `show`'s citations, then
`git grep` for the clause's own words in test names and assertions.

- **Y:** you read an assertion, a refusal or a check that fails if
  this clause is broken, and it runs in the gate (`tools/gate.sh`, a
  nextest test, a `tools/*-check.sh`) or in CI. Give its file and line
  and say in a few words what it asserts.
- **P, partly:** a test exercises the clause but would pass on some
  breach of it, or holds one half. Say which half is loose.
- **N:** you looked and found nothing. Say where you looked.
- **U, unbuilt or unholdable:** the code does not exist yet (the entry
  says `Unbuilt`), or the clause is a rule on people or on future work
  that no test or check over this tree could fail on.

A type or a dependency edge that makes the breach unwritable counts as
Y: name it. A grep check that an ordinary respelling would pass is P.
A breach that would take building around the structure on purpose (an
HTTP client over the standard library's sockets, where the edge refuses
every HTTP crate) is not a respelling: the edge is Y.

A clause that restates another entry is held where that entry is held;
do not chase it: mark it `M · R` and name the entry.

### 3. Each clause that is P or N in step 2: what it would take

- **simple:** a test that fits an existing table or harness in the
  lowest crate that holds the behaviour, with no new fixture, no new
  harness and no change to product code. Name the table or harness
  (file, and the table or helper).
- **complex:** it needs a new harness, a new seam or a product change,
  or the clause's meaning is itself unclear. Say which, in one line.
- **cut:** the clause is M and no caller, frontend or user relies on it
  as a promise, so the honest fix is to delete it from the entry and
  leave it to the doc comment. Say what shows nothing relies on it. An
  M clause that is not held is always `cut`; a P clause never is.

For each `simple` or `complex`, say whether the test is worth writing
**for its own sake** — a breach would harm the GGG relationship
(`ggg`), lose or clobber the owner's data (`data`), or give a user or
an agent a wrong answer (`wrong`) — or **only to let a paragraph go**
(`para`).

### 4. The reason at the code

Look at the comment under the id at the holder, and at the doc comment
where `show` says the code names the id.

- **Y:** one comment that cites the id states the property and the
  substance of the entry's *Why:*. Give file and line.
- **Prop:** the property is stated, the reason is not.
- **Cite:** the id is cited in fragments; no one comment states the
  property.
- **None:** no file outside the registry cites the id.

Then name the one comment the reason would move to if the entry
retired (file and line): the doc of the lowest test that holds it, or
the comment on the check.

### 5. The class: whose it is under today's rules

One of the ten codes of `CLASS-BRIEF.md`, section 2, by its rule, now
that you have read the code. For `cross`, name the frontend or crate
that could break the promise unseen, or the one holder that sees them
all.

## The fate

Derive it; do not weigh it.

1. Class is an owner's one. **KO** (keep, the owner's) if it has no M
   clause; **SH-KO** (shed) if it has one or more: it stays an entry
   and its M clauses go to the code. Step 2 and 3 are still recorded.
2. Class is a session's, and every P clause is U. **KU** (keep,
   unheld), or **SH-KU** if it also has M clauses.
3. Class is a session's, every P clause is Y, and every M clause is Y,
   R, or already stated in a doc comment. **RN** (retire now) if step 4
   is Y; **RM** (retire after moving the reason) if it is not.
4. Otherwise **NT** (needs a test first): a P clause is P or N, or an
   M clause is neither held nor stated at the code. Its scope is the
   heaviest among its unheld clauses: `complex`, then `simple`, then
   `cut`.

## The report

Return the report as your final message, in exactly this form: one
block per entry, then the table. No preamble, no summary of the batch
beyond the last section.

```
### C<n> — <the headline's first six words>
- class: <code> — <one line: who is promised what>
- clauses:
  1. P|M · Y|P|N|U|R · <file:line and what it asserts, or where you looked, or the entry it restates>
  2. ...
- unheld: <clause number> simple|complex|cut · ggg|data|wrong|para · <the table or harness, or why complex, or what shows nothing relies on it>   (one line per unheld clause; "none")
- reason: Y|Prop|Cite|None · <file:line> · moves to <file:line>
- fate: KO|SH-KO|KU|SH-KU|RN|RM|NT · <for NT: simple|complex|cut>
- confidence: high|medium|low · <what you did not read>
```

Then:

```
## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| C19 | ggg | 3 | 0 | 2/1/0/0/0 | Prop | KO | 1/0/0 | 1/0 | high |
```

- `P`, `M`: the count of clauses of each kind; they sum to the script's count.
- `held Y/P/N/U/R`: counts over all clauses, P and M together.
- `scope s/c/x`: counts of unheld clauses judged simple, complex, cut.
- `worth own/para`: counts over the simple and complex ones.

End with a section `## What I did not check`, and one `## Noticed`, for
an entry whose text disagrees with the code, a *Pinned:* that does not
hold what it names, or a rule above that would not decide an entry:
say which entry and which step.

Read the tests. An honest `N` or `low` is worth more than a `Y` taken
from a name.
