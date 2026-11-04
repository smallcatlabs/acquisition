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
The commits: 9a982ab2..50494341, round B of the answer-facts seat's issues, six commits (item 0 the two-clock golden, then #169, #171 in two commits, #170, #167), from roadmap/briefs/seat-issues-b.md at 9a982ab2 (the brief lists each item with its rule and its pin, and the calls that showed it; the calls' outputs are not in the tree, so judge the words against the brief's quotations). Check: item 0, every test that refetches after a seed and whose golden reads the facts line now records at one time, and no production change; #169, a page's answer on both surfaces carries `freshness` (its `facts`), `left_out`, `counts` and `policy` as the whole render does, whether keeping the page's own top-level keys beside the narrowed render (the brief forbids removing a key) leaves one fact read in two places (the page text twice, a page near 50 KB: shape 5 and 7), and whether the CLI's stdout stays pasteable with the render's lines on stderr; #171, a container row's set and clear receipts carry the counts `price show` prints from the same function and never a second count, item rows unchanged, what a clear's receipt says where the facts cannot be read (silence against C146), and whether opening the facts read-only on a clear can create or migrate a file; #170, `invalid_tab_names` and its items counted as `invalid_notes` is (public and not?), the `invalid_tab_name` route run as printed listing exactly those items, `invalid_notes` unchanged, describe and the tool text naming both; #167, `clear` offered beside `undo` only where the undo is not itself a clear, its revision the one the write left, `undo`/`no_undo`'s never-both-never-neither kept, the printed clear running as printed; the describe size bound raised from 8,000 to 9,000 bytes and the tool descriptions trimmed to fit 600 bytes (did a trim drop a fact a reader needed?); no JSON key renamed or removed; every regenerated golden line one of the items' words or a `# holds` line; both references changed for a reason the diff shows; nothing outside the items changed.
