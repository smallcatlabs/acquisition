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
The commits: a50ac14c..accf270f, round D of the answer-facts seat's issues, one commit (#166), from roadmap/briefs/seat-issues-d.md and the new ruling C164 in decisions/daemon.md, both at a50ac14c (the brief gives the rule and its pin, and the calls that showed it; the calls' outputs are not in the tree, so judge the words against the brief's quotations). Check: invariant 1, the check at the client door reads the store and sends nothing, and no path admits a stash job for an unlisted tab while the daemon's own submissions (an apply's children, a deep fetch's substashes) stay outside the door; C164's cases, a tab the stored listing does not name or names as gone refused, a league with no stored listing admitted, and whether "gone" uses the store's one live-tab test (`TAB_GONE`, a retired parent's substash included); the `sub` rule (judged only where the stored rows name the tab's substashes) and what a substash id sent as `id`, a folder, a never-fetched tab or a store that cannot be read does; whether the daemon reads the store through its read API and raw SQL stays in the store crate (C48, C1's edges); the account's store opened or locked at submit without creating a file or racing an ingest, and with the right account after resolution; the refusal's words, the listing's age on the store's one clock (C53), the client value quoted bounded (C127), C143's request terms, and whether leaving the per-surface offers out (the brief asked for them where a refused submit offers a way on) is right or a sibling left standing; the `realm_wire.rs` assertion that pinned the 404 now pinning the refusal, and nothing it held lost; the describe daemon bound and the describe tool's text cut to fit 600 bytes, as measured; every regenerated golden line the item's words or a `# holds` line; MCP-REFERENCE changed for a reason the diff shows; nothing outside the item changed.
