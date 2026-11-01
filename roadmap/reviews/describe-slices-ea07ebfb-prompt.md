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


The range: eb9f29b9^..ea07ebfb, two commits, the build of roadmap/briefs/describe-slices.md (its fourth cut, 6b8e81ce; the ruling it rests on is C53 as amended at 453aa437). Judge the brief's done-when and both issues' "Closes when" (#153: an index form on the MCP, the same index the CLI's bare --describe prints, bounded to a name and a line; #154: a pattern answers the matching entries, the same set on each surface, nothing matched says so and offers the index, the golden holds the wording, the help and the tour say how). Pay particular attention to: the index and `all` resolved once in the search crate and each frontend passing its list through (settled 1); the CLI's --describe value split in code, a pattern taking the rest of its value (settled 1); the one-line rule and the pattern compiler each in one place (shape 7); the offer path through Offered, searched and Way::Describe, with argv at the terminal and {} on the MCP (settled 5; C156); the tier at LISTED seen from both sides; the category id as a name; the bare `~` kept as the operator's name; the echoed pattern as payload on the MCP; every changed golden and reference read as a diff of what a user now reads (shape 9), the new 33 KB golden among them; the MCP's instructions and tool text under surface_size.rs's limits, and whether what was cut from the `names` doc (the list of every section) loses a fact an agent needs; no wording pinned twice (C153); and whether a reader of the old `describe(&[])` as the whole was left unconverted anywhere.

The worker's report is not supplied, by design, except its "Raised for review" list, verbatim, which the session holds against your findings:

- The index's golden (`goldens/search_json/describe_at_a_terminal.txt`) holds the whole index text, about 33 KB plus the three slice commands, since a Transcript records a command's whole output and only the bare `--describe` prints the index's foot. Every change to any entry's first line now regenerates it. A golden of a smaller command was not possible for the foot; a reviewer may prefer the foot pinned by an in-code assertion instead (that would be the foot's words held outside a golden, which C153 discourages).
- The `names` doc no longer lists every section (the bound); the guarantee that the MCP names every section now rests on the index (whose keys are the sections, pinned in the search crate). Worth the owner's look against #83's reason for the list.
- The instructions' describe line is 142 bytes against 144 before; the instructions total sits at the 1,000-byte bound. The next words change there has no room.
- On the MCP the refusal for a pattern adds "; pricing, daemon and store are read by name alone" to the search crate's message, as the unknown-name refusal already appended the own names: the refusal is not spoken-as the CLI's (no parity test asserts it, and the brief asked none).
- The CLI's text index now prints the totals table's provenance (about 400 bytes) over the computed block, and so does every slice that holds a computed value: text is the function of the JSON, which carries `totals` in every answer. The old index left it out under a hand-written head.
- `~(` beside `all` is refused (patterns are compiled before `all` returns): consistent, but `all` is then not "everything whatever else is typed".
- The brief's list of `describe(&[])` readers missed `acquisition-search/tests/seat_faults.rs:644` (`f1_…`); it was converted to `all` too, named in the commit.

