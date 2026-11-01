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


The range: 4e53584d, one commit, the fix round answering the cold review of the describe-slices build (roadmap/reviews/describe-slices-ea07ebfb.md, two findings: the pattern example quoted for POSIX alone, which cmd.exe keeps as typed; the index's and the slice's prose pinned by literal in three tests beside the golden that holds it). Its brief is roadmap/briefs/describe-slices.md. Judge it against those two findings: each fixed by one rule and pinned, or named as not taken and why. For the first: is `--describe=~resist` in fact portable under every shell C156 declares (posix, pwsh, powershell5, cmd), and does the pin in error_kind.rs run the example as printed through each shell the harness finds; is the README tour line's new form (`acq search --describe[=name,…|=~pattern|=all]`) what readme_tour.rs resolves and what a reader of the tour expects; every sibling sentence carried (shape 3). For the second: does any literal assertion on describe's prose remain outside the golden, and are the two the worker kept on purpose (the bad-pattern words in surface_faults.rs; the MCP's appended "pricing, daemon and store are read by name alone" in search_tools.rs) rightly kept, since no golden holds them, or should a golden hold them (C122). The golden's `# holds` facts live in search_json.rs beside the golden file: say whether that is one holder or two.

The worker's raised list, verbatim:
- **The `# holds` facts live in the test, not the golden file.** A rewording of `next` touches the golden and its own facts in `search_json.rs`. Those facts are the golden's words, so I read this as meeting the rule, but it is two files, not one.
- **Two assertions I kept on purpose:**
  - `surface_faults.rs` still holds the bad-pattern words ("`~` takes a pattern in Rust regex syntax…"), because no golden holds them.
  - `search_tools.rs` still holds the MCP's own appended "pricing, daemon and store are read by name alone", because it is the adapter's addition and no golden holds it either.
- **The golden's command lines still show `'~^sockets'`.** That is how the test typed its arguments when recording, not an example the tool prints.
- **The brief's Do 6** asks for "a quoted example (`--describe '~resist'`)". The review suggests the brief should allow a portable or shell-specific example instead. The brief is unchanged by this round.
