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
The commits: e2736ccf..a5e6b58c, round A of the answer-facts seat's issues, three commits, one per issue (#174, #173, #36), from roadmap/briefs/seat-issues-a.md at e2736ccf (the brief lists each issue with its rule and its pin, and the calls that showed it; the calls' outputs are not in the tree, so judge the words against the brief's quotations). Check: #174, a word naming a field `has:` takes offered as `has:<word>` first in both the bare-word refusal (parse.rs) and the `is:` refusal (bind.rs), from one source (whatever the commit names as "a field has: takes", and whether `has:`'s own binding and every other reader of that fact now read it, with nothing left deciding it a second way), the minus carried where the reader typed one, and whether an offer carrying a minus agrees with the refusal's span (an agent replacing the span with the offer must get the query the offer means); #173, a counts table keyed by `league` and a cross with `league` as either key carrying the scope's leagues no bucket holds, on both surfaces (JSON, the CLI's text, the MCP's tool text and gloss, `describe`), counted over every value and not only the limit's, a league split by realm counted as held when any realm's bucket holds it, absent where every league has a bucket, and whether "the scope's leagues" is the same set the answer's `scope.leagues` prints; #36, the tabs pointer naming a league the query names through and/or and never one under a negation, `other_leagues` counting the named leagues that hold never-fetched tabs, none where none does, the realm's most where the query names none, `fixed_league` and the scope's list age unchanged; every regenerated golden line one of the three issues' words or a `# holds` line; both references unchanged or changed for a reason the diff shows; nothing outside the three changed.
