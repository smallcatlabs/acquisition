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
The commits: 1562eb27..a7395a13, round C of the answer-facts seat's issues, two commits (#172, then #168's text half), from roadmap/briefs/seat-issues-c.md at 1562eb27 (the brief lists each item with its rule and its pin, and the calls that showed it; the calls' outputs are not in the tree, so judge the words against the brief's quotations). Check: #172, `store status`/`store_status` and `tabs`/the `tabs` tool (and the web refresh page that reads the same document) carry their facts through the store's one coverage derivation and the shared formatter, with no surface computing an age or an extreme for itself (C162); the store-wide block over every realm and league and the league's over its tabs alone, the characters apart; whether `Tabs::read` reading the tabs and their coverage in two store reads can give a facts line that disagrees with the rows it heads (a write between them), and whether `store status`'s does the same; the keys named as the other surfaces name them and none renamed; the web page golden's new mask of the fetch clause (`Pages::fetched_live`) hiding nothing the round should have pinned; the describe store bound raised from 6,000 to 7,000 bytes and the describe tool's sizes as measured; #168's text half, one ready command per account from the JSON offers' own source, each running as printed in a POSIX shell (quoting, `--league=` spelling, the global `--account` placed where clap takes it), capped at ten with the rest counted and `acq accounts --expand` printed, the kind `ambiguous_account` and the sentence unchanged, and whether the uncapped sentence above the capped commands or the `UnknownAccount` refusal (names known, no commands in text) is a sibling left standing (shape 3); every regenerated golden line one of the items' words or a `# holds` line; both references changed for a reason the diff shows; nothing outside the items changed.
