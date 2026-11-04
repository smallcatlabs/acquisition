You are the outside reviewer of one committed step of the Acquisition
Rust workspace. The commit is named at the end of this prompt: read it
with `git show <sha>` and `git log -1 <sha>`, and review it against the
brief its message names (a file under roadmap/briefs/). A range is one
round of one brief: read that brief once for the range. Read; do not
build or run anything; the tree is read-only to you.

Review the diff and the callers it affects for plain defects first:
behavioural regressions, arithmetic and error-path failures,
cancellation and concurrency, persistence, and anything that bypasses
an invariant (`AGENTS.md` names them; the rate-limit choke point and
the single gate above all). Tell client input (a request a frontend
sends) apart from persisted input (what the store wrote earlier) and
upstream input (a GGG response): each is trusted differently, and a
finding names which it met. Then the area's rows of
`REVIEW-CHECKLIST.md`, the shapes of fault that came back. Then these
shapes, which the records have taught because each came back until it
was named:

1. A claim wider than its evidence: a sentence written for the common
   case and read as a rule; a number recalled rather than measured.
2. A fix staging the nearest failure, not the named one: a test green
   for a reason other than the one the brief states.
3. A sibling sentence left standing: the fix made where the fault was
   met and not where the same words stand elsewhere (the other
   surface, the other verb, the reference, the README).
4. A refusal that says what is wrong and not what to do.
5. A document unbounded by default.
6. A printed command that does not run as printed.
7. One fact read in two places, so that a fix in one leaves the other.
8. Evidence read off whatever is on disk rather than the fixed input
   the brief names.
9. A fixture regenerated blind: every changed file under `tests/goldens/`
   or a `*-REFERENCE.md` is read as a diff of what a user now reads, and a
   change no source change in the commit explains is a finding.
10. A pin kept where a lower pin or a structural check holds it; and,
   for each `retired X; held by Y` line in a message, Y missing at
   that commit, run neither by the gate nor by CI, or holding less
   than X held.

Then whether the brief's "done when" lines are actually met by what
landed; a line that needs execution to check is reported as "not
verified", never guessed. The worker's report is not supplied to
you, by design: the commit is the evidence and the report its
author's account, and the session that reads both holds them
against each other. Judge the done-when against the commit alone.
Execution evidence (the gate's exit statuses, the test count) is in
that report and never in a commit; a commit message may carry what
the commit measured, which is its author's claim and not
verification; so say "not verified" once, in one line, and spend no
paragraph on it. When the range named is a fix
round, its done-when is each finding of the review it answers fixed
by one rule and pinned, or named as not taken and why, and the gate
green; judge it against those findings, not the brief's build lines.
A sitting the build still awaits (the owner in a browser, a live run)
is not the fix round's to meet and is never a finding against it.

Report in this form, nothing else:
- One line: the commit, and whether the brief's done-when holds.
- Findings, most severe first, each: file:line, the shape number if
  one applies, what is wrong in one sentence, the concrete failing
  input or reading, and the smallest fix you believe sufficient. Where
  several findings share one cause, say so once.
- What you did not check and why.
- Which findings you would call nitpicks for cases unlikely in practice.
- Anything about the brief, the plan or this prompt that got in your
  way or that you would change, a sentence each.
The commits: dcd8abcb..e3b5c217, the fix round of round A of the answer-facts seat's issues, three commits, one per finding of the review at roadmap/reviews/seat-issues-a-a5e6b58c.md (the brief is roadmap/briefs/seat-issues-a.md). The round's orders, given by message and not in the tree: finding 1, an offer replaces exactly its span, the span covering the minus the offer carries, pinned by pasting the offer into the original query at the span; finding 2, one reader of the query's `league=` terms comparing the field name as the binder does, used by both the tabs pointer and `fixed_league` (the scope's list age), `fixed_league`'s meaning unchanged, and a league under a negation never the pointer's league even where the query names no other; finding 3, the count-key refusal's filter and describe's read `takes_has`, the priced exclusion kept, no behaviour change. The worker widened finding 1: since a refusal's offers share one span, every offer under a typed minus now carries it (`-has:priced`, `-name:priced`, `-"priced"`), inside a line group too. Check: every offer of every bare-word refusal pasted over its span means what it says, including `not` spelled as a word (does `not` become `-`, and is that the same query?), a minus followed by spaces, nesting, and a refusal with no span; whether any other refusal that carries a span and readings now disagrees; the one league reader's three positions (and-ed, or-ed, negated) against the binder's own tree for `undecided( )`, `holds( )`, a double negation and a negated group; the list age for `LEAGUE=` and every other `fixed_league` caller; the two filters' output unchanged; every regenerated golden line one of the findings' words; nothing outside the three findings changed.
