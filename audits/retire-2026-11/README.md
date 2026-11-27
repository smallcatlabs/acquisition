# What the registry could retire: a census, 2026-11-27

Written for the session that will do the retiring, and for the owner
and the orderer who read the counts first. Read-only: nothing here
retired an entry, changed a test or filed an issue. Taken at
`50e28e68`, the close of step `process-337`, with 193 entries and 7
retired.

The question was how many of the registry's entries today's rules
would not enter, so that each could retire to a comment at the test
or check that holds it (the retired rule, `tools/bothy.py`), and what
that would cost. An earlier reader's guess from headlines was about
115.

## The short of it

1. **31 entries can retire as they stand.** One is free. Thirty need
   their reason written at the holder first, in the same commit.
2. **17 more can retire after small work**: 3 after a clause is cut,
   14 after a test that fits a table or harness the crate already
   has.
3. **145 stay.** 119 are the owner's under C170 (who decides). 12 are
   rules nothing but the entry holds. 14 need a new harness, a new
   seam, a product change or a rewording before any test could hold
   them.
4. **So this is housekeeping.** The 31 are 20.5 KB of the registry's
   130 KB, 16%; with the 17, 32 KB, 24%. It is worth one step, for a
   reason other than size: only 10 of the 93 entries whose tests were
   read have their reason written at the code, and a retirement is
   what writes it there.
5. **The guess of 115 was a guess about class, and class is where it
   was wrong.** Two readers classing every entry from its text put
   103 and 106 with the owner. Reading the tests then moved 16 more
   to him: each holds a promise about things not yet written that no
   test would catch.
6. **The larger weight is mechanism inside entries that stay.** 57
   kept entries carry mechanism clauses C192 (a property and its
   mechanism) sends to the code: 88 clauses by both class readers,
   118 by either, 9 to 11 KB.
7. **The readers found about thirty places where an entry's words and
   the code disagree**, listed under "What the readers noticed". Two
   may be faults in the product today.

## Why 31 and not 115

Counted, with judgment marked.

| Step | Entries | How it was found |
| --- | --- | --- |
| The registry | 193 | `python3 -I tools/bothy.py check` |
| Both text readers class it the owner's | 100 | `scripts/classes.py agree` |
| Classed a session's by at least one, so its tests were read | 93 | the same |
| Of those, the owner's by the reader who opened the code, where the two text readers split | 3 (C21, C40, C150) | `scripts/tally.py check` |
| A session's, with a first promise nothing can hold | 12 | `scripts/tally.py counts`, KU and SH-KU |
| A session's, needing a harness, a seam or a rewording | 14 | NT-complex |
| A session's, held or a small step from held | 64 | before the re-run |
| Of those, a promise on things not yet written that nothing catches | 16 | the re-run, `open` |
| Could go | 48 | 31 as they stand, 17 after small work |

What that says, as judgment.

- **The owner's classes are most of the registry**: 103 of 193 by
  class alone, most of them promises to GGG, to the owner's data, or
  boundaries between crates and processes. Today's rules enter those,
  so the census left them, and did not read their tests.
- **"Agent-facing design" is narrower than its headline count.** Of
  the 55 or 56 entries the text readers called design, the re-run
  found 13 are conventions every later verb, answer or page
  must follow (a count says what it counts, an answer is one object,
  every printed id is accepted back). Tests hold the things that
  exist. The next one is held by whoever read the entry. That is the
  one case where an entry does work no holder's comment can.
- **Entries were not written to be held clause by clause.** The 93
  read in depth have 444 clauses. Their readers found 267 held, 91
  partly or not held, 55 that no test could hold, and 31 that restate
  another entry. One clause in five is loose, which is why "held,
  reason aside" is 31 and not 60.
- **When they were entered does not explain it.** By the first commit
  that added each bullet (`scripts/born.py`, `reports/born.tsv`):

  | Entered | Entries | Owner's | Rule only the entry holds | Needs a harness | Could go |
  | --- | --- | --- | --- | --- | --- |
  | to 2026-09-02, when the registry was made | 63 | 45 | 1 | 2 | 15 |
  | 2026-09-03 to 10-11 | 47 | 26 | 2 | 2 | 17 |
  | 2026-10-12 to 10-19 | 9 | 4 | 0 | 2 | 3 |
  | 2026-10-20 to 11-08 | 49 | 28 | 3 | 6 | 12 |
  | 2026-11-09 on | 25 | 16 | 6 | 2 | 1 |

  The share that could go is a quarter to a third until 2026-11-08
  and almost none in the newest entries, which are the owner's or rules on how the work
  is run. The rules about what is entered were already keeping
  session design out by 2026-11-09. No change of rule is called for;
  what is left is the older design entries.

## The rule the readers judged by

Three briefs, kept as written: `CLASS-BRIEF.md` (whose an entry is,
and which clauses are mechanism), `BRIEF.md` (each clause held or not,
the reason at the code, what an unheld clause would take), and
`RERUN-BRIEF.md` (the one sharpened question). The fate is derived by
`scripts/tally.py`, never weighed by a reader.

| Fate | In the table | What it means |
| --- | --- | --- |
| Keep, the owner's | KO | its promise is to GGG, to the owner's data, a boundary, scope, a rule only people keep, or a promise on things not yet written that nothing catches |
| Shed, not retire | SH-KO, SH-KU | it stays, and carries clauses that are mechanism |
| Keep, unheld | KU | a session's, and its first promise is a rule on people or on future work that no test over this tree could fail on |
| Retire now | RN | a session's, every promise held by a test or check read by the reader, and the reason stated in a comment under the id |
| Retire after moving the reason | RM | the same, and the reason stands only in the entry |
| Needs a test first | NT-cut, NT-simple, NT-complex | a clause is not held; what it would take is its scope |

What I sharpened as the reads came in, each said once here.

- **Class is asked of the promise, never of how well it is held.** An
  entry promised to GGG and held by forty tests is still the owner's.
- **The owner's entries were not read in depth.** The pilot showed one
  reader fills its context on about seven entries, so reading the
  tests of all 193 would have taken some thirty readers. Their
  holding is not known; see "What was not checked".
- **Three shapes the fate steps did not decide**, which seven readers
  reported, are decided in `tally.py`'s `derive`: an entry whose
  first promise is unholdable is kept unheld; an unholdable clause
  further down leaves with the cut ones; an entry with no promise at
  all retires when the code states it.
- **`cross` became "a promise on things not yet written".** See the
  re-run, below.

## Counts per area and fate

`python3 -I audits/retire-2026-11/scripts/tally.py counts`

| area | KO | SH-KO | KU | SH-KU | RN | RM | NT-cut | NT-simple | NT-complex | all |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| cross-cutting | 9 | 6 | 2 | 0 | 0 | 0 | 0 | 0 | 1 | 18 |
| ci | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
| daemon | 9 | 15 | 0 | 1 | 1 | 7 | 0 | 3 | 2 | 38 |
| frontends | 8 | 5 | 0 | 0 | 0 | 2 | 0 | 0 | 5 | 20 |
| network | 8 | 10 | 1 | 0 | 0 | 2 | 0 | 0 | 1 | 22 |
| plans | 4 | 3 | 0 | 0 | 0 | 2 | 0 | 3 | 0 | 12 |
| pricing | 4 | 6 | 0 | 1 | 0 | 4 | 0 | 0 | 0 | 15 |
| process | 9 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 2 | 17 |
| search | 4 | 3 | 1 | 0 | 0 | 7 | 1 | 4 | 2 | 22 |
| store | 7 | 6 | 0 | 0 | 0 | 4 | 1 | 3 | 0 | 21 |
| trade | 0 | 2 | 0 | 0 | 0 | 2 | 1 | 1 | 1 | 7 |
| all | 62 | 57 | 10 | 2 | 1 | 30 | 3 | 14 | 14 | 193 |

By bytes of entry text: KO 38.1 KB, SH-KO 40.9, KU and SH-KU 6.8,
NT-complex 12.4, RN and RM 20.5, NT-cut and NT-simple 11.4, of 130.1.

The entries, by fate, for the ones that are not simply the owner's:

- **RN:** C152.
- **RM:** C5 C165 C166 C86 C160 C45 C49 · C121 C163 · C17 C161 ·
  C36 C77 · C69 C70 C81 C80 · C91 C104 C93 C94 C95 C105 C96 ·
  C54 C56 C62 C188 · C119 C141.
- **NT-cut:** C118 C57 C113.
- **NT-simple:** C6 C23 C27 · C126 C37 C76 · C90 C109 C92 C125 ·
  C29 C145 C60 · C142.
- **NT-complex:** C122 · C157 C158 · C11 C53 C115 C146 C162 · C159 ·
  C180 C192 · C97 C102 C114.
- **KU, SH-KU:** C153 C154 · C137 · C33 · C75 · C181 C185 C183 C174
  C178 C182 · C101.
- **The owner's by the re-run:** C3 C147 C51 C148 · C149 C143 C156
  C187 · C74 · C98 C100 C107 C117 · C63 · C99 C195.

`python3 -I audits/retire-2026-11/scripts/tally.py work` prints, for
each of these, the comment its reason would move to and each unheld
clause in its reader's words.

## The cost, in the units the first seven gave

What step `process-337` showed (`git show -s 51f2e4f9`, `50e28e68`):
seven retirements were one commit of comments, twelve minutes after
the form existed. Six of the seven needed their reason written at the
holder. Each had one holder file. The step took two cold reviews, and
neither found a fault in a retirement.

Against that:

- **Free: 1 of 31.** C152's holder already states the property and
  the reason. For the other 30 the reason is to be written: across
  the 48 that could go, the comment under the id states property and
  reason for 5, the property alone for 36, fragments for 7. That is
  the sample's 4 in 30, confirmed.
- **A moved comment: 30, and 3 more with a clause cut.** These are
  not the seven's size. The seven had one holder each. These name a
  median of three to four holder files each (a rough count of the
  files their held clauses cite, about 114 file mentions over 31
  entries), because their clauses are held in different crates. A
  retired line's *Held by:* names files that cite the id, and the
  reason goes to a comment at each. Six entries have a holder that
  catches a new thing only through a list kept by hand (C165, C163,
  C93, and C109, C125, C145 once tested): the comment belongs at the
  list.
- **A test first: 14 entries, 16 clauses.** Each fits a
  named table or harness with no new fixture and no product change.
  Nine clauses in eight entries are worth a test for their own sake.
  Seven clauses in six entries are worth one only to let a paragraph
  go.
- **Left as they are: 14.** Twenty-five of their clauses need a
  harness, a seam, a product change or a rewording.

One caution on the 30. Where two readers read the same clause, they
gave the same held-or-not verdict for 73 of 84. The second reader
found 3 clauses loose that the first called held. At that rate, about
six of the 135 clauses in the 31 would not survive a second read (my
estimate). That is what each batch's cold review is for.

## A proposed order

Nothing here is acted on before the owner and the orderer have read
the counts together.

**First, four batches of comments**, each one commit carrying its
`retired C<n>; held by <Y>` lines (C154, the retirement line), each
reviewed cold, since a retirement claims its holder holds every
clause. A batch is one area's holders, so one reviewer reads one
crate's tests.

| Batch | Entries | Holders mostly in | Read first |
| --- | --- | --- | --- |
| 1. Daemon | C152 C5 C165 C166 C86 C160 C45 C49 | the daemon's tests, the protocol's `wire.rs`, the client's `contract.rs` | `reports/first/daemon-1.md` to `daemon-3.md` |
| 2. Search and trade | C91 C104 C93 C94 C95 C105 C96, C119 C141, and the cut ones C118 C113 | the search crate's tests | `search-1.md` to `search-4.md`, `trade.md` |
| 3. Pricing, plans and store | C69 C70 C81 C80, C36 C77, C54 C56 C62 C188, and the cut one C57 | the plan and store crates' unit tests | `pricing.md`, `plans.md`, `store.md` |
| 4. Frontends and network | C121 C163, C17 C161 | the verbs crate's `documents.rs`, the limiter's `spec.rs` | `frontends-2.md`, `network.md` |

Marks on single entries, from the re-run and the second read:

- C96: the second reader found clause 5 loose (a league-less query
  over two live leagues is asserted nowhere). Write that test in the
  batch, or leave C96.
- C152 and C94: the re-run called one clause `open` at low
  confidence (a new page reading the journal's path from the words;
  a new derived value that sums lines in code). Read those two first.
- C49 and C17 have no promise at all, by their readers. They retire
  to the doc comment that already says them. C49's one clause would
  be `open` if it were read as a promise.
- C5 is cited by a "Not doing" line in `decisions/daemon.md`, which
  would then cite a retired id, and its words ("one level in use")
  are stale.
- C165, C163, C93: the comment goes at the hand-kept list that
  catches a new document or field.

**Then the tests worth writing for their own sake**, ranked by what a
breach costs. Each also lets its entry retire.

1. C6, clause 6: `acqd` refuses to start on a queue it cannot read.
   Only the restore's error is tested. The CLI's process harness.
2. C23, clause 2: cancelling a parent cancels its children. No test
   reads a child after its parent's cancel, and the children are
   sends. `daemon/tests/parents.rs`.
3. C126, clause 3: a failed leagues response does not replace the
   stored list. The store's unit rows for C126.
4. C60, clause 1: the realm is the request's, never the entry's
   `realm` field. No test gives an entry a realm that disagrees.
5. C145, clause 1: the wrong-typed character arrays and the listings'
   own arrays. Four of eleven seams are in the table.
6. C92, clause 4: two occurrences each meeting one condition of a
   group. The search crate's worked corpus.
7. C142, clauses 1 and 3: three of the four defence filters, and the
   label naming its field. The translate table.
8. C27, clause 2: a closed terminal (SIGHUP) beside Ctrl-C.

**Then the rest**, each when a step next amends it:

- the six paragraph-only tests (C37, C76, C90, C109, C125, C29), if
  the owner says on touch (question 1);
- the 14 that need a harness or a rewording;
- the 16 the re-run kept. Each stays for one clause. The rest of each
  could move to its holders now, or the clause could become structure
  and the entry then retires whole. The re-run reader named the
  structure for two: routing every CLI connect through
  `run::connect` would catch C3 clause 7 and C51 clause 5.

## What is the owner's to decide

**1. The tests that only let a paragraph go.** Six entries (C37, C76,
C90, C109, C125, C29) are each one small test away from retiring, and
nothing but the paragraph is gained: a breach would give nobody a
wrong answer. Written now, they are six assertions in tables that
exist, and six fewer entries. Left for the step that next
touches each, they cost nothing now and the entries stay read until
then. I would write them now, in the same batches as the eight tests
that are worth having, because the cost is an afternoon and the
alternative is a list somebody has to keep. The other answer is as
defensible, and it is his. The six are 3.8 KB of entry text.

**2. Shedding now, or as C192 says, when an entry is next amended.**
57 entries that stay carry mechanism: 88 clauses by both readers, 9
KB, about a tenth of the registry. C192 sheds them one entry at a
time as steps amend them, which so far has been slow. One pass would
move them to doc comments now. It is the larger saving here, and it
changes the words of rulings in his classes, which change only by his
word. The readers' marks are in `reports/class-a.md` and
`class-b.md`; they agreed on 129 of the 194 clauses either marked, so
a pass would need a read of each clause, not the list as it stands.

**3. C101 against C170.** C101's last clause says a change to what a
user types is recorded as a ruling before the code. C170 and
`AGENTS.md` say a query word is a session's, recorded in the commit
that builds it. Four readers pointed at it. One of the two is stale,
and which one is what a ruling means.

Whether to run the retiring step at all is the orderer's to put with
these. My answer is in point 4 of the short of it.

## Where the readers disagreed

**The class, from the text** (`scripts/classes.py agree`). Two readers,
all 193 entries: the same side, owner's or session's, for 184 (95%),
the same class for 177 (92%). The nine they split went to the reader
who opened the code: C165, C36, C101, C145, C54, C60 to a session;
C21, C40, C150 to the owner. Both said where the rule did not decide:
a protocol message's shape is at once a boundary and an answer's
shape (C4, C8, C9, C165, C166, C40); store facts are refetchable, so
their correctness is `data` to one reader and `mech` to the other
(C54, C60, C145); `run` against `rule` for seven process entries.

**The deep read, twice** (`scripts/tally.py agree`). 19 of the 93
entries, a seeded fifth, blind.

| Judgment | Agree | Of |
| --- | --- | --- |
| The fate each row derives | 15 | 19 |
| Retire after moving the reason | 5 | first 6, second 5 |
| Needs a simple test | 5 | first 7, second 6 |
| Needs a harness | 2 | first 2, second 2 |
| The owner's | 2 | first 2, second 5 |
| A clause is a promise or a mechanism | 75 | 84 clauses |
| A clause is held, partly, not, or unholdable | 73 | 84 clauses |
| A clause is not held | 13 | first 18, second 16 |
| Simple, complex or cut, where both list a clause | 8 | 13 clauses |
| Worth a test for its own sake, or only a paragraph | 9 | 13 clauses |

- **Three of the four differing fates were class**, and two of those
  were `cross`: C149 and C143, a session's design to the first reader
  and the owner's to the second. The fourth is C96, above.
- **The scope judgment agreed for 8 of 13**, which is low. The five
  differences are in three entries, and in each one reader found a
  cheap test that holds part of a clause and the other judged what
  holding all of it takes (C122 clauses 1 and 3, C149 clause 1), or
  the two disagreed whether the clause was a promise (C125 clauses 2
  and 4). Within the entries now called "needs a simple test", the
  five clauses read twice agree. I did not re-run scope: two of the
  three entries stay whichever reader is right. A "simple" in the
  table is one reader's estimate that a test fits. Whether it then
  holds the whole clause is for the builder and the cold review.

**The re-run.** The owner's fate was the one with low agreement, and
the readers said why: `cross` asked whether a new frontend could
break a promise unseen, and a promise is broken far more easily by a
new verb, answer, count or page. So the question was asked once more,
sharpened, of all 64 entries that could go: does a promise bind
things not yet written, and would anything fail? 30 are bound to one
behaviour that exists. 10 are caught by a type, a door or a check. 6
are caught only through a hand-kept list. 18 are open, 2 of them at
low confidence, which move nothing. The 16 went to the owner. The
re-run agrees with the second reader on both C149 and C143.

This was one reader per entry and was not read twice. It is the
judgment in this census I trust least, and it moved a quarter of the
entries that could go. One of its sixteen, C74, rests on a clause the
first reader had already called unholdable, and most rest on one
clause of several: C3 on its seventh of eight, C156 on its sixth of
eight.

## What the readers noticed

Faults in what the entries say, each from a reader's "Noticed", none
checked by a second reader. Filing them is the orderer's.

Maybe a fault in the product today:

- C53: `acq demo` prints cursor escapes to stdout with no terminal
  check (`crates/acquisition-cli/src/jobs_cmd.rs:703`), against
  "escapes only on a tty".
- C150: no test sets a proxy variable, and the reader did not verify
  that reqwest built without its default features honours them. The
  entry may be untrue as well as unheld.
- C3: with a mock-mode MCP job tool, a refusal for want of a login
  may not say a daemon was started. Low confidence, not driven.
- C121: the daemon's own refusals carry the kind and the words, not
  the `accounts` field the entry promises "alike".

An entry wider than, or stale against, the code:

- C5 says one priority level is in use; probes and logins use two
  more, and `set-priority` takes any.
- C97 says autocomplete is the one corpus read; `acq web`'s finders
  read the trade stat table.
- C115 says a count's request rides only when routes are asked; the
  code carries some unasked, by design.
- C95 says there is no grand total; the counts view answers one.
- C102 says the register is inherited with one exception; six of nine
  limits are printed, and one in other words.
- C147 names the environment-reading functions as the world-taking
  ones.
- C163 and C118 are marked `Unbuilt` and are built.
- C90 calls kind part of a line's identity; the counts view merges
  kinds.
- C69 cites one trade claim where its tests and module doc cite
  another.
- C187 calls the browser check a stage of the gate; it runs inside
  the `test` stage.
- C148's type is said, in its own doc, to prove more than it does.
- C178 and C180 say each close rewrites the road; the session-close
  skill says a step the road never named leaves it.
- C174 says a brief over 8,000 bytes is two steps; 88 of 162 briefs
  are over.
- C122 says a blind regeneration of a golden fails; only the CLI's
  transcript helper enforces named facts.
- C149 carries history ("taken at the alpha") that the entry rule
  refuses and docs-check passes.
- C101 against C170, question 3.

A *Pinned:* that does not hold what it names: C6 (the start's
refusal), C23 (the cascade), C60 (the entry's field), C86 (one test,
the words only), C96, C126, C92, C142, C145, C149, C12 and C20 by the
earlier sample.

A promise no entry states: the idle verdict holds the daemon while
counted hits sit in a policy window (C3's tests hold it; it protects
the limiter's picture); the token endpoint's probe exemption (stated
only at `gate.rs`); the transport crate's "no reqwest item in the
public API", held by review. C188 clause 3 is a fact about GGG with
no numbered claim in the ground truth.

Repeats: the MCP's spawn rule is stated in C13, C10, C88 and C3;
invariant 1 in C21, C22, C130, C150 and C161; C17 is invariant 2
again.

## What was not checked

- **The owner's 103 entries by class.** Whether their *Pinned:* holds
  every clause was not read. 23 of the 119 kept entries say `nothing
  yet`. The earlier sample found half of pinned entries only partly
  held. Reading them is a second census of about the same price.
- **Nothing was run.** No test, no mutation, no gate. "Runs in the
  gate" rests on a file sitting under a crate's tests or in `tools/`.
- **Every "held" is one reader's reading of an assertion**, with a
  fifth read twice. Readers name what they did not open at the foot
  of each report.
- **Windows.** Several holders are Unix only (C27, C147, C148).
- **The re-run**, as said above.
- **Mechanism clauses** were marked from the text by two readers who
  agreed on 129 of 194. No clause was checked against a doc comment
  for the 100 entries not read in depth.
- **The pilot.** `reports/first/trade.md` was read under the brief
  before two sentences were sharpened. Two of its verdicts are
  corrected in `tally.py` (`PILOT`) and said there; its classes for
  C99 and C113 were superseded by the class readers.
- **The count of holder files** is a rough count of file names in the
  readers' lines.
- Whether a builder would meet a retired entry's comment where it now
  meets the entry. The reading audit's finding is that builders reach
  rulings through the code about six times as often as through
  `decisions/`.

## How this was made

I read `decisions/process.md`, the retired rule, the close of
`process-337` and the earlier sample, wrote the briefs and the
scripts, and judged no entry myself. Opus readers did the reading,
read-only, at most five at a time.

| Pass | Readers | What each did |
| --- | --- | --- |
| Pilot | 1 | the trade area's 7 entries, to size a batch and try the form |
| Class | 2 | all 193 entries from their text, blind to each other |
| First read | 14 | the other 86 of the 93 entries either class reader called a session's, 4 to 11 each, clause by clause against the tests |
| Second read | 3 | a seeded fifth of those, 19 entries, blind |
| Re-run | 2 | the 64 that could go, one question each, from the first reports |

The brief asked for one reader per area file, about ten. It took
fifteen, with the pilot, because the pilot filled its context
(180,541 tokens at its peak) on seven entries and thirty clauses, so
an area of 22 entries could not be read by one reader without its
later verdicts thinning. Batches were cut at 34 clauses
(`scripts/classes.py batches`). Classing first, from the text, is
what kept it from thirty.

**The price: 22 reader runs, 3,302,024 tokens** by their completion
notices (`reports/tokens.tsv`), 129,030 to 192,737 each, three to
five and a half minutes each.

Reports are saved whole from the transcripts with
`audits/meta-2026-11/scripts/save_report.py`, under `reports/`:
`class-a.md` and `class-b.md`, `first/`, `second/`, `rerun-1.md`,
`rerun-1-c160.md` (a row the reader left out, asked for again) and
`rerun-2.md`. Line numbers in the reports are as of `50e28e68`.

Made again by: `scripts/registry.py` (clauses, numbered as bothy
counts them), `scripts/classes.py` (class agreement, the batches, the
seeded fifth, seed 20261127), `scripts/tally.py` (the table, the
counts, the agreement, the work), `scripts/born.py` (when each entry
was entered).

## The table

`python3 -I audits/retire-2026-11/scripts/tally.py table`. Class is
reader A, reader B, then the deep reader. Clauses are promises plus
mechanism: for an entry read in depth, by its reader; for the rest,
mechanism is what both class readers marked. Held is yes, partly, no,
unholdable, restated. Unheld is simple, complex, cut. Conf is the
deep reader's, or the two class readers'.

| id | area | class A / B / deep | fate | clauses P+M | held Y/P/N/U/R | reason | unheld s/c/x | worth own/para | re-run | conf | report |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| C1 | cross-cutting | edge / edge / – | KO | 8+0 | – | – | – | – | – | high / high | class |
| C2 | cross-cutting | edge / edge / – | KO | 2+0 | – | – | – | – | – | high / high | class |
| C4 | cross-cutting | edge / edge / – | KO | 2+0 | – | – | – | – | – | medium / medium | class |
| C12 | cross-cutting | edge / edge / – | KO | 2+0 | – | – | – | – | – | high / high | class |
| C14 | cross-cutting | ggg / ggg / – | SH-KO | 1+1 | – | – | – | – | – | high / medium | class |
| C15 | cross-cutting | edge / edge / – | SH-KO | 2+1 | – | – | – | – | – | medium / medium | class |
| C34 | cross-cutting | data / edge / – | KO | 2+0 | – | – | – | – | – | medium / medium | class |
| C35 | cross-cutting | data / data / – | SH-KO | 2+2 | – | – | – | – | – | high / high | class |
| C38 | cross-cutting | ggg / ggg / – | KO | 3+0 | – | – | – | – | – | medium / medium | class |
| C46 | cross-cutting | edge / edge / – | KO | 2+0 | – | – | – | – | – | high / high | class |
| C47 | cross-cutting | cross / cross / – | SH-KO | 2+1 | – | – | – | – | – | medium / medium | class |
| C48 | cross-cutting | edge / edge / – | KO | 2+0 | – | – | – | – | – | medium / high | class |
| C79 | cross-cutting | scope / scope / – | SH-KO | 4+1 | – | – | – | – | – | medium / high | class |
| C110 | cross-cutting | rule / rule / – | KO | 3+0 | – | – | – | – | – | medium / medium | class |
| C120 | cross-cutting | scope / scope / – | SH-KO | 2+1 | – | – | – | – | – | high / high | class |
| C122 | cross-cutting | check / check / check | NT-complex | 6+0 | 3/2/0/1/0 | Cite | 1/1/0 | 0/2 | – | medium | process |
| C153 | cross-cutting | run / run / check | KU | 5+0 | 0/0/0/5/0 | Prop | 0/0/0 | 0/0 | – | high | process |
| C154 | cross-cutting | run / run / run | KU | 3+0 | 0/0/0/3/0 | Prop | 0/0/0 | 0/0 | – | high | process |
| C155 | ci | ggg / ggg / – | SH-KO | 4+1 | – | – | – | – | – | medium / medium | class |
| C3 | daemon | design / design / design | SH-KO | 6+2 | 5/1/0/0/2 | Prop | 1/0/0 | 0/1 | open 7 | medium | daemon-1 |
| C5 | daemon | mech / mech / design | RM | 1+0 | 1/0/0/0/0 | Prop | 0/0/0 | 0/0 | bound | high | daemon-1 |
| C6 | daemon | design / mech / mech | NT-simple | 4+4 | 6/1/0/0/1 | Prop | 1/0/0 | 1/0 | bound | medium | daemon-1 |
| C7 | daemon | ggg / ggg / – | KO | 3+0 | – | – | – | – | – | high / high | class |
| C127 | daemon | ggg / ggg / – | SH-KO | 4+1 | – | – | – | – | – | high / high | class |
| C164 | daemon | ggg / ggg / – | KO | 5+0 | – | – | – | – | – | high / high | class |
| C165 | daemon | edge / design / design | RM | 3+1 | 3/0/0/0/1 | Prop | 0/0/0 | 0/0 | listed 3 | medium | daemon-1 |
| C8 | daemon | edge / edge / – | KO | 1+0 | – | – | – | – | – | medium / medium | class |
| C9 | daemon | edge / edge / – | KO | 1+0 | – | – | – | – | – | medium / medium | class |
| C85 | daemon | edge / edge / – | KO | 5+0 | – | – | – | – | – | high / high | class |
| C166 | daemon | design / design / design | RM | 7+0 | 7/0/0/0/0 | Prop | 0/0/0 | 0/0 | caught 7 | high | daemon-1 |
| C152 | daemon | design / design / design | RN | 4+1 | 5/0/0/0/0 | Y | 0/0/0 | 0/0 | open 5 (low) | high | daemon-2 |
| C10 | daemon | edge / edge / – | KO | 6+0 | – | – | – | – | – | high / medium | class |
| C84 | daemon | edge / edge / – | SH-KO | 3+2 | – | – | – | – | – | low / medium | class |
| C83 | daemon | ggg / edge / – | SH-KO | 6+2 | – | – | – | – | – | medium / medium | class |
| C112 | daemon | edge / edge / – | KO | 2+0 | – | – | – | – | – | medium / medium | class |
| C132 | daemon | ggg / ggg / – | SH-KO | 2+4 | – | – | – | – | – | medium / medium | class |
| C86 | daemon | design / design / design | RM | 2+0 | 2/0/0/0/0 | Prop | 0/0/0 | 0/0 | bound | high | daemon-2 |
| C147 | daemon | mech / mech / mech | SH-KO | 3+2 | 5/0/0/0/0 | Y | 0/0/0 | 0/0 | open 2 | medium | daemon-2 |
| C137 | daemon | check / check / check | SH-KU | 3+2 | 2/0/1/1/1 | Y | 0/1/0 | 0/1 | – | medium | daemon-2 |
| C157 | daemon | check / check / check | NT-complex | 3+5 | 3/2/0/3/0 | Prop | 0/1/1 | 0/1 | – | medium | daemon-2 |
| C158 | daemon | check / check / check | NT-complex | 4+0 | 0/4/0/0/0 | Prop | 0/4/0 | 0/4 | – | medium | daemon-2 |
| C160 | daemon | check / check / check | RM | 4+2 | 5/0/0/0/1 | Prop | 0/0/0 | 0/0 | caught 1 | high | daemon-3 |
| C82 | daemon | edge / edge / – | SH-KO | 4+1 | – | – | – | – | – | medium / medium | class |
| C23 | daemon | design / design / design | NT-simple | 3+0 | 2/1/0/0/0 | Cite | 1/0/0 | 1/0 | bound | medium | daemon-3 |
| C27 | daemon | design / design / design | NT-simple | 3+1 | 2/2/0/0/0 | Cite | 1/0/1 | 1/0 | bound | medium | daemon-3 |
| C133 | daemon | ggg / ggg / – | SH-KO | 4+4 | – | – | – | – | – | medium / medium | class |
| C134 | daemon | ggg / ggg / – | SH-KO | 3+3 | – | – | – | – | – | medium / medium | class |
| C45 | daemon | design / design / design | RM | 3+0 | 3/0/0/0/0 | Prop | 0/0/0 | 0/0 | bound | high | daemon-3 |
| C123 | daemon | scope / scope / – | SH-KO | 3+1 | – | – | – | – | – | low / medium | class |
| C31 | daemon | ggg / ggg / – | SH-KO | 3+1 | – | – | – | – | – | high / high | class |
| C49 | daemon | mech / mech / mech | RM | 0+2 | 1/0/1/0/0 | Prop | 0/0/1 | 0/0 | bound (low) | medium | daemon-3 |
| C50 | daemon | data / data / – | SH-KO | 3+2 | – | – | – | – | – | high / high | class |
| C51 | daemon | design / design / design | KO | 7+0 | 4/2/0/1/0 | Prop | 2/0/0 | 1/1 | open 5 | medium | daemon-3 |
| C135 | daemon | ggg / ggg / – | SH-KO | 2+3 | – | – | – | – | – | high / high | class |
| C136 | daemon | ggg / ggg / – | KO | 1+0 | – | – | – | – | – | medium / medium | class |
| C144 | daemon | ggg / data / – | SH-KO | 4+1 | – | – | – | – | – | medium / medium | class |
| C148 | daemon | mech / mech / mech | SH-KO | 5+1 | 4/2/0/0/0 | Prop | 2/0/0 | 0/2 | open 1 | medium | daemon-3 |
| C11 | frontends | design / design / design | NT-complex | 5+0 | 4/1/0/0/0 | Cite | 0/1/0 | 0/1 | – | medium | frontends-1 |
| C13 | frontends | edge / edge / – | KO | 3+0 | – | – | – | – | – | high / medium | class |
| C16 | frontends | edge / edge / – | SH-KO | 4+1 | – | – | – | – | – | medium / medium | class |
| C52 | frontends | data / data / – | KO | 2+0 | – | – | – | – | – | high / high | class |
| C53 | frontends | design / design / cross | NT-complex | 17+2 | 7/7/2/2/1 | Cite | 2/6/1 | 3/5 | – | medium | frontends-1 |
| C149 | frontends | design / design / design | SH-KO | 2+2 | 1/1/1/0/1 | Prop | 1/0/1 | 0/1 | open 1 | high | frontends-1 |
| C115 | frontends | design / design / design | NT-complex | 4+0 | 3/1/0/0/0 | Cite | 0/1/0 | 0/1 | – | medium | frontends-1 |
| C116 | frontends | cross / cross / – | KO | 3+0 | – | – | – | – | – | medium / medium | class |
| C121 | frontends | design / design / design | RM | 2+0 | 2/0/0/0/0 | Prop | 0/0/0 | 0/0 | caught 1 | high | frontends-2 |
| C138 | frontends | ggg / ggg / – | KO | 2+0 | – | – | – | – | – | low / medium | class |
| C139 | frontends | edge / edge / – | SH-KO | 1+1 | – | – | – | – | – | medium / high | class |
| C191 | frontends | scope / scope / – | KO | 3+0 | – | – | – | – | – | high / medium | class |
| C140 | frontends | ggg / ggg / – | KO | 2+0 | – | – | – | – | – | medium / high | class |
| C143 | frontends | design / design / design | KO | 5+0 | 4/1/0/0/0 | Prop | 1/0/0 | 0/1 | open 5 | high | frontends-2 |
| C146 | frontends | design / design / design | NT-complex | 4+1 | 0/4/0/0/1 | Prop | 2/2/0 | 3/1 | – | medium | frontends-2 |
| C162 | frontends | mech / mech / cross | NT-complex | 2+2 | 1/1/1/0/1 | Prop | 0/2/0 | 2/0 | – | medium | frontends-2 |
| C163 | frontends | design / design / design | RM | 3+2 | 4/0/0/0/1 | Prop | 0/0/0 | 0/0 | listed 5 | high | frontends-2 |
| C156 | frontends | design / design / design | SH-KO | 6+2 | 8/0/0/0/0 | Prop | 0/0/0 | 0/0 | open 6 | medium | frontends-2 |
| C168 | frontends | data / data / – | KO | 4+0 | – | – | – | – | – | medium / high | class |
| C187 | frontends | check / check / check | SH-KO | 2+2 | 3/1/0/0/0 | Prop | 0/0/1 | 0/0 | open 1 | medium | frontends-2 |
| C17 | network | mech / mech / mech | RM | 0+1 | 0/0/0/0/1 | Prop | 0/0/0 | 0/0 | bound | medium | network |
| C18 | network | ggg / ggg / – | SH-KO | 1+1 | – | – | – | – | – | medium / medium | class |
| C19 | network | ggg / ggg / – | KO | 3+0 | – | – | – | – | – | high / high | class |
| C20 | network | ggg / ggg / – | KO | 1+0 | – | – | – | – | – | high / high | class |
| C21 | network | mech / ggg / ggg | SH-KO | 2+2 | 2/1/0/0/1 | Prop | 0/0/1 | 0/0 | – | high | network |
| C22 | network | ggg / ggg / – | SH-KO | 4+2 | – | – | – | – | – | medium / high | class |
| C24 | network | ggg / ggg / – | KO | 2+0 | – | – | – | – | – | high / high | class |
| C87 | network | ggg / ggg / – | SH-KO | 5+1 | – | – | – | – | – | high / high | class |
| C25 | network | ggg / ggg / – | KO | 6+0 | – | – | – | – | – | medium / medium | class |
| C26 | network | ggg / ggg / – | SH-KO | 3+1 | – | – | – | – | – | high / high | class |
| C32 | network | ggg / ggg / – | SH-KO | 2+2 | – | – | – | – | – | high / high | class |
| C33 | network | check / check / check | KU | 3+0 | 1/0/0/2/0 | Prop | 0/0/0 | 0/0 | – | medium | network |
| C88 | network | ggg / ggg / – | SH-KO | 8+1 | – | – | – | – | – | medium / medium | class |
| C128 | network | ggg / ggg / – | SH-KO | 4+1 | – | – | – | – | – | high / high | class |
| C129 | network | ggg / ggg / – | KO | 5+0 | – | – | – | – | – | high / high | class |
| C130 | network | ggg / ggg / – | SH-KO | 3+1 | – | – | – | – | – | high / high | class |
| C131 | network | ggg / ggg / – | KO | 5+0 | – | – | – | – | – | high / high | class |
| C150 | network | design / ggg / ggg | SH-KO | 2+2 | 0/0/3/0/1 | Prop | 2/0/1 | 0/2 | – | medium | network |
| C159 | network | check / check / check | NT-complex | 1+3 | 2/1/0/0/1 | Prop | 0/1/0 | 0/1 | – | medium | network |
| C161 | network | check / check / check | RM | 1+5 | 3/0/1/0/2 | Prop | 0/0/1 | 0/0 | caught 2 | medium | network |
| C169 | network | ggg / ggg / – | KO | 4+0 | – | – | – | – | – | high / high | class |
| C190 | network | ggg / ggg / – | KO | 5+0 | – | – | – | – | – | high / high | class |
| C36 | plans | design / edge / design | RM | 4+2 | 4/0/0/0/2 | Cite | 0/0/0 | 0/0 | bound | medium | plans |
| C126 | plans | design / design / design | NT-simple | 4+0 | 3/1/0/0/0 | Y | 1/0/0 | 1/0 | bound | high | plans |
| C37 | plans | design / design / design | NT-simple | 4+0 | 3/1/0/0/0 | Prop | 1/0/0 | 0/1 | bound | high | plans |
| C39 | plans | edge / edge / – | KO | 3+0 | – | – | – | – | – | high / high | class |
| C40 | plans | edge / design / edge | KO | 3+0 | 1/1/0/1/0 | Y | 1/0/0 | 1/0 | – | medium | plans |
| C41 | plans | ggg / ggg / – | KO | 5+0 | – | – | – | – | – | medium / medium | class |
| C42 | plans | ggg / ggg / – | KO | 3+0 | – | – | – | – | – | medium / medium | class |
| C43 | plans | edge / edge / – | SH-KO | 1+3 | – | – | – | – | – | medium / medium | class |
| C44 | plans | data / data / – | SH-KO | 4+2 | – | – | – | – | – | medium / medium | class |
| C76 | plans | design / design / design | NT-simple | 2+1 | 1/1/0/0/1 | Prop | 1/0/0 | 0/1 | bound | medium | plans |
| C77 | plans | design / design / design | RM | 3+0 | 3/0/0/0/0 | Prop | 0/0/0 | 0/0 | bound | high | plans |
| C111 | plans | ggg / ggg / – | SH-KO | 6+1 | – | – | – | – | – | high / medium | class |
| C64 | pricing | data / data / – | SH-KO | 5+1 | – | – | – | – | – | medium / high | class |
| C65 | pricing | data / data / – | KO | 5+0 | – | – | – | – | – | high / high | class |
| C66 | pricing | data / data / – | SH-KO | 4+1 | – | – | – | – | – | high / high | class |
| C67 | pricing | data / data / – | SH-KO | 4+1 | – | – | – | – | – | medium / medium | class |
| C68 | pricing | data / edge / – | KO | 9+0 | – | – | – | – | – | medium / medium | class |
| C69 | pricing | design / design / design | RM | 5+1 | 5/0/0/0/1 | Prop | 0/0/0 | 0/0 | bound | high | pricing |
| C70 | pricing | design / design / design | RM | 4+1 | 5/0/0/0/0 | Prop | 0/0/0 | 0/0 | bound | medium | pricing |
| C71 | pricing | data / data / – | SH-KO | 3+1 | – | – | – | – | – | medium / medium | class |
| C72 | pricing | data / data / – | KO | 2+0 | – | – | – | – | – | high / high | class |
| C73 | pricing | data / data / – | KO | 7+0 | – | – | – | – | – | high / high | class |
| C74 | pricing | design / design / design | SH-KO | 5+2 | 3/1/0/1/2 | Prop | 1/0/0 | 0/1 | open 7 | medium | pricing |
| C75 | pricing | mech / mech / edge | SH-KU | 2+2 | 0/0/1/2/1 | Prop | 0/0/1 | 0/0 | – | medium | pricing |
| C78 | pricing | data / data / – | SH-KO | 3+2 | – | – | – | – | – | high / high | class |
| C81 | pricing | design / design / design | RM | 4+2 | 4/0/0/0/2 | Cite | 0/0/0 | 0/0 | bound | high | pricing |
| C80 | pricing | design / design / design | RM | 4+0 | 4/0/0/0/0 | Prop | 0/0/0 | 0/0 | bound | high | pricing |
| C170 | process | rule / rule / – | KO | 6+0 | – | – | – | – | – | high / high | class |
| C171 | process | rule / rule / – | KO | 5+0 | – | – | – | – | – | high / high | class |
| C179 | process | rule / rule / – | KO | 6+0 | – | – | – | – | – | high / high | class |
| C181 | process | run / run / run | KU | 2+0 | 0/0/0/2/0 | None | 0/0/0 | 0/0 | – | high | process |
| C172 | process | rule / rule / – | KO | 4+0 | – | – | – | – | – | high / high | class |
| C185 | process | run / run / run | KU | 2+0 | 0/0/0/2/0 | None | 0/0/0 | 0/0 | – | high | process |
| C183 | process | run / run / run | KU | 3+0 | 0/0/0/3/0 | None | 0/0/0 | 0/0 | – | high | process |
| C189 | process | rule / rule / – | KO | 4+0 | – | – | – | – | – | high / high | class |
| C173 | process | rule / rule / – | KO | 5+0 | – | – | – | – | – | medium / medium | class |
| C174 | process | run / run / run | KU | 5+0 | 0/0/1/4/0 | Prop | 0/1/0 | 0/1 | – | high | process |
| C178 | process | run / run / run | KU | 4+0 | 0/0/0/4/0 | Prop | 0/0/0 | 0/0 | – | high | process |
| C180 | process | run / run / run | NT-complex | 4+0 | 1/2/0/1/0 | Prop | 1/1/0 | 0/2 | – | high | process |
| C175 | process | rule / rule / – | KO | 4+0 | – | – | – | – | – | high / high | class |
| C176 | process | rule / rule / – | KO | 5+0 | – | – | – | – | – | high / high | class |
| C177 | process | rule / rule / – | KO | 4+0 | – | – | – | – | – | medium / medium | class |
| C182 | process | run / run / run | KU | 6+0 | 0/0/1/5/0 | Cite | 1/0/0 | 0/1 | – | high | process |
| C192 | process | run / run / run | NT-complex | 4+0 | 0/2/0/2/0 | Cite | 0/2/0 | 0/2 | – | medium | process |
| C89 | search | edge / edge / – | KO | 4+0 | – | – | – | – | – | high / high | class |
| C103 | search | edge / edge / – | SH-KO | 3+1 | – | – | – | – | – | medium / medium | class |
| C90 | search | design / design / design | NT-simple | 5+1 | 4/0/1/0/1 | Cite | 1/0/0 | 0/1 | bound | medium | search-1 |
| C91 | search | design / design / design | RM | 4+0 | 4/0/0/0/0 | Prop | 0/0/0 | 0/0 | bound | high | search-1 |
| C104 | search | design / design / design | RM | 4+0 | 4/0/0/0/0 | Prop | 0/0/0 | 0/0 | caught 1 | high | search-1 |
| C109 | search | design / design / design | NT-simple | 4+0 | 3/1/0/0/0 | Y | 1/0/0 | 0/1 | listed 4 | high | search-1 |
| C92 | search | design / design / design | NT-simple | 9+0 | 8/1/0/0/0 | Prop | 1/0/0 | 1/0 | bound | medium | search-1 |
| C93 | search | design / design / design | RM | 8+0 | 8/0/0/0/0 | Prop | 0/0/0 | 0/0 | listed 5 | high | search-2 |
| C94 | search | design / design / design | RM | 4+3 | 5/0/1/0/1 | Prop | 0/0/1 | 0/0 | open 5 (low) | medium | search-2 |
| C95 | search | design / design / design | RM | 5+0 | 5/0/0/0/0 | Prop | 0/0/0 | 0/0 | bound | high | search-2 |
| C105 | search | design / design / design | RM | 5+0 | 5/0/0/0/0 | Prop | 0/0/0 | 0/0 | bound | high | search-2 |
| C96 | search | design / design / design | RM | 6+0 | 6/0/0/0/0 | Cite | 0/0/0 | 0/0 | caught 3 | high | search-3 |
| C97 | search | design / design / design | NT-complex | 5+0 | 3/1/1/0/0 | Cite | 1/1/0 | 1/1 | – | medium | search-3 |
| C98 | search | design / design / cross | SH-KO | 4+1 | 4/0/0/0/1 | Prop | 0/0/0 | 0/0 | open 3 | medium | search-3 |
| C100 | search | design / design / design | KO | 6+0 | 4/2/0/0/0 | Prop | 2/0/0 | 1/1 | open 4 | medium | search-3 |
| C101 | search | rule / design / design | KU | 6+0 | 3/0/1/2/0 | Cite | 1/0/0 | 0/1 | – | medium | search-3 |
| C102 | search | design / design / design | NT-complex | 5+2 | 1/2/2/1/1 | Prop | 2/1/1 | 2/1 | – | medium | search-4 |
| C106 | search | scope / scope / – | SH-KO | 4+1 | – | – | – | – | – | medium / medium | class |
| C107 | search | design / design / design | KO | 5+0 | 0/2/0/3/0 | Y | 2/0/0 | 2/0 | open 3 | medium | search-4 |
| C117 | search | design / design / design | KO | 3+0 | 1/1/0/1/0 | Y | 1/0/0 | 1/0 | open 1 | high | search-4 |
| C118 | search | design / design / design | NT-cut | 6+0 | 5/0/0/1/0 | Cite | 0/0/0 | 0/0 | caught 3 | medium | search-4 |
| C125 | search | check / check / check | NT-simple | 3+1 | 2/2/0/0/0 | Prop | 2/0/0 | 0/2 | listed 1 | medium | search-4 |
| C28 | store | edge / edge / – | SH-KO | 4+1 | – | – | – | – | – | high / high | class |
| C29 | store | mech / mech / mech | NT-simple | 1+2 | 1/1/0/0/1 | Prop | 1/0/0 | 0/1 | bound | medium | store |
| C30 | store | data / data / – | KO | 3+0 | – | – | – | – | – | medium / medium | class |
| C145 | store | data / mech / mech | NT-simple | 4+2 | 5/1/0/0/0 | Y | 1/0/0 | 1/0 | listed 1 | high | store |
| C54 | store | data / mech / mech | RM | 5+2 | 7/0/0/0/0 | Prop | 0/0/0 | 0/0 | bound | medium | store |
| C55 | store | data / data / – | KO | 5+0 | – | – | – | – | – | medium / high | class |
| C56 | store | mech / mech / mech | RM | 2+0 | 2/0/0/0/0 | Prop | 0/0/0 | 0/0 | bound | high | store |
| C57 | store | mech / mech / mech | NT-cut | 2+2 | 3/0/1/0/0 | Prop | 0/0/1 | 0/0 | bound | high | store |
| C108 | store | edge / edge / – | SH-KO | 4+1 | – | – | – | – | – | high / high | class |
| C124 | store | data / data / – | SH-KO | 2+1 | – | – | – | – | – | high / high | class |
| C193 | store | edge / edge / – | SH-KO | 5+1 | – | – | – | – | – | high / medium | class |
| C167 | store | data / data / – | KO | 4+0 | – | – | – | – | – | medium / medium | class |
| C194 | store | data / data / – | KO | 3+0 | – | – | – | – | – | medium / medium | class |
| C151 | store | data / data / – | KO | 6+0 | – | – | – | – | – | high / high | class |
| C58 | store | ggg / cross / – | SH-KO | 2+5 | – | – | – | – | – | medium / medium | class |
| C59 | store | ggg / ggg / – | SH-KO | 4+1 | – | – | – | – | – | high / high | class |
| C60 | store | data / mech / mech | NT-simple | 2+0 | 1/1/0/0/0 | Y | 1/0/0 | 1/0 | caught 1 | high | store |
| C61 | store | ggg / ggg / – | KO | 1+0 | – | – | – | – | – | high / medium | class |
| C62 | store | design / mech / mech | RM | 1+2 | 2/0/1/0/0 | Prop | 0/0/1 | 0/0 | bound | medium | store |
| C63 | store | design / design / design | KO | 2+0 | 0/1/1/0/0 | Prop | 2/0/0 | 0/2 | open 1 | medium | store |
| C188 | store | design / design / mech | RM | 4+1 | 4/0/1/0/0 | Prop | 0/0/1 | 0/0 | bound | high | store |
| C99 | trade | design / design / edge | SH-KO | 7+1 | 5/1/0/2/0 | Cite | 1/0/0 | 0/1 | open 1 | medium | trade |
| C113 | trade | design / design / scope | NT-cut | 5+1 | 5/0/1/0/0 | Prop | 0/0/1 | 0/0 | caught 1 | medium | trade |
| C114 | trade | design / design / design | NT-complex | 3+0 | 1/2/0/0/0 | Prop | 1/1/0 | 0/2 | – | medium | trade |
| C119 | trade | design / design / design | RM | 2+1 | 3/0/0/0/0 | Prop | 0/0/0 | 0/0 | caught 1 | high | trade |
| C141 | trade | design / design / design | RM | 2+0 | 2/0/0/0/0 | Prop | 0/0/0 | 0/0 | bound | high | trade |
| C142 | trade | design / design / design | NT-simple | 4+0 | 2/2/0/0/0 | Prop | 2/0/0 | 2/0 | bound | high | trade |
| C195 | trade | design / design / design | SH-KO | 4+1 | 4/1/0/0/0 | Prop | 0/0/1 | 0/0 | open 3 | medium | trade |
