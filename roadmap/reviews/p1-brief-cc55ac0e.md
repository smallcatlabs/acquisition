# Review of cc55ac0e, the p1 brief — a blind Fable reader, 2026-10-28

Launched by the orchestrating session with the brief, the README, the template, p0's brief and the issues named, and none of the session's own analysis. Read-only. Saved verbatim.

**Fit to run as written: not quite.** Nothing in it is dangerous, but three of its fates rest on reasons the tree does not hold, two of its Do items change owner-held lists without a packet question, and two of its own facts (the ledger-row count, the `area:tools` label against `AGENTS.md`'s definition) are wrong; a half-hour edit before the owner reads the packet makes it fit.

## Findings, most severe first

1. **Do 4 (brief lines 236-237) adds #91 and #11 to the owner's pull-forward order without asking.** `roadmap/README.md:147-152` says the list is "the owner's word of 2026-10-26 ... in order ... an item leaves the list when its step closes or the owner strikes it"; the routing table (line 146) gives #11 the fate "the pull-forward list, after #91", a fate the Input (lines 58-64) does not define, and #91 appears nowhere in the README today (`grep '#91' roadmap/README.md` is empty). Adding two items to an ordered list the owner set is scope, his under AGENTS.md "Rulings first". Smallest change: a packet question (q) "#91 and #11 join the pull-forward list after #89, in that order?", and the table row's fate becomes "packet (q)".

2. **Do 1 (d)'s recommendation `area:tools` "whose rulings live nowhere" contradicts `AGENTS.md:128`**, where an `area:*` label is defined as "the area file it belongs to". Either the recommendation names the AGENTS.md line it changes, or it recommends the second option (`area:process`, home `roadmap/README.md` "A step" and `tools/`), which fits the definition. Smallest change: one clause in (d) saying which, and that "Where things live" gains the words if `area:tools` is chosen. (Neither label exists yet; the label list is the eight `area:*` plus the five `trigger:*`.)

3. **Table row "the `not_an_item` sweep shape not seen to fail first" (line 118): the drop reason is unsupported.** It says "the template's done-when already asks that a pin be red before its fix"; `TEMPLATE.md:37-52` says "fixed by one rule and pinned, or named as not taken", and no word of TEMPLATE.md or REVIEW-PROMPT.md asks that a pin be seen red (grep `red|fail` over TEMPLATE.md finds only "failing inputs" and "fix round"). Smallest change: either route it as "packet (n)" beside the other TEMPLATE.md sentence, recommending one clause ("a pin is seen red on the tree before its fix lands, the report says so"), or change the reason to "a lapse of one round, the pin stands" alone.

4. **Table row "From h1 ... #16's first instance" (line 147): "drop: each on its issue" is false for #16.** `gh issue view 16` has no comment; the body is the park's one line; only `HISTORY.md:24` says "#16 in part". Smallest change: fate "on #16", one comment naming the instance fixed and its commit (the h1 commits under `efc6e745..0b199df3`).

5. **Issue C (line 72) carries a sentence the tree does not support: "the MCP `search` description naming `view.rows.fields`".** `grep -rn 'rows\.fields' MCP-REFERENCE.md crates/acquisition-mcp` finds nothing, and `git log -S'view.rows.fields'` touches only roadmap briefs (e11768e2, 3691766e, cc55ac0e): the phrase was never in the MCP's text. The brief lets the worker "name in the report any line the tree disproved", but the fixed input should not ship a known-stale clause. Smallest change: strike that clause from C and the row at line 138 becomes "drop: not in the description (the tree, `MCP-REFERENCE.md`)".

6. **Do 1 (b) and the commit message count "nine" rows written after the ruling that list each commit; the README has eleven.** Rows after p0's (`README.md:412-422`): r1, r2, r3, r4, r5, the C135 round, #82, #83, #84, #88, 2g; ten of them name each commit, r1 names a range plus one. A count in a brief is measured (TEMPLATE.md:42-45). Smallest change: "the eleven written after it".

7. **Do 4 names the sections it makes true but omits three places (a) and (c) change, and the done-when cannot catch them:** the first paragraph (`README.md:6-7`, "one line per step closed", false once the ledger moves), "A step" item 5 (`README.md:121-123`, "writes the step's ledger line ... puts what the step left open under 'Open' or in an issue", which (a) narrows to four homes and (c) moves to `LEDGER.md`), and "Starting a session" item 2 (`README.md:116-117`, "its step's ledger line written"). Smallest change: a Do 4 sentence "the intro, 'A step' items 2 and 5 and 'Starting a session' say the ledger's file and (a)'s homes".

8. **Done-when line 3 (lines 268-272) is not checkable from the commit alone**: "`gh issue view` shows each new issue with its labels and milestone, each 'on #N' is a comment" is GitHub state; TEMPLATE.md:38 asks for "its tree or its message". Smallest change: "the close message lists each letter's issue number and each comment's issue, and the README's 'Open' holds no routed item", with the `gh` check the orchestrating session's.

9. **Table row "the live drivers under `tools/` edited and not run" → "on #67" is a misfit.** #67's closing condition is "each is a numbered claim or an explicit unknown in ground truth"; a note that two drivers await their first run is neither, and `tools/` is out of scope for the step anyway. Smallest change: fate "mine" (the owner's list: "run `tools/tracer-rung.sh` and `tools/persist-check.sh` at the next live run; edited at 2g, not run"), which is where the README's own words put it ("the owner's next live run their first check").

10. **Do 4 rewrites "The owner's rulings for this road" ("narration cut") without saying the quotations stay verbatim.** The section's value is the owner's words (`README.md:14, 21-25, 39-45, 50-51`); a worker cutting narration can trim a quote. Smallest change: "his quoted words unchanged, the narration around them cut".

11. **Packet (p) (STATE-2026-10-19.md) is not an "Open" observation and its recommendation is the status quo, already ruled** ("a phase close deletes nothing", README.md:54; the brief's own Not in scope line 283). A question whose every answer leaves the tree as it is costs the owner a read for nothing. Smallest change: drop (p); the fate is already "Not in scope".

12. **Packet (b) re-asks a ruling of 2026-10-26** (README.md:52-53: "a ledger row is a commit range and a close date"). Applying it to older rows is the application, the delegated kind. Nitpick-level; fold (b) into (c) as one question about the file.

## Observations the routing table misses or misroutes

- Ledger row #88 (`README.md:421`) says "the retirement of the build-order rule's second half ... the owner's, under 'Open' with its text" — no such Open paragraph exists today (retired at `a6a3b698`); the ledger rewrite (Do 3) should drop the clause and the brief should say so, since the worker is told the table is complete.
- Ledger row 4a (`README.md:408`) "the AGENTS budget at 15,235 of 15,277 bytes, under 'Open'" duplicates packet (g) with a stale number (15,241 today); one home.
- "A step" item 2 (`README.md:99-102`, "the build's last commit carries the step's ledger line") and item 5 (`:121`, "the close ... writes the step's ledger line") state the ledger line twice with two writers; one of them is wrong.
- `README.md:138`, row 2: "2h ... which slips before 2g does" is stale now 2g is closed; Do 4 says "reduced to 2h" but not that the slip sentence goes.
- The phase table's dates disagree with the milestones by a day (3b "2026-11-26" vs due 2026-11-25; 4-full "2026-12-19" vs 2026-12-18; PoE2 "2026-12-11" vs 2026-12-10), and phase 2 ("2026-12-05") has no milestone; the brief's Read (line 50) calls them duplicates. Removing them is right; the worker needs to know phase 2's "Closes by" cell has no milestone name to take.
- The "committed coverage apart from evidence the tests pass; a fix round's acceptance apart from the sitting" drop (line 142) is half-supported: `10f24fd9` added "not verified once" and "judge against those findings, not the brief's build lines" to REVIEW-PROMPT.md:38-43, which answers the first clause and only touches the second (the sitting is never named). Acceptable as a drop if the reason says "the first in the prompt; the second a sentence for (j)".
- Packet (e) counts "the two trade-stat ids at `poe1@ee2084de`" is fine (`ee2084de` is a commit in `../poe1`), but a worker without the sibling clone cannot verify it; the row's comment on #53 should say "the sibling clone's commit".

## What the sweep misses (item 5)

- `HISTORY.md:22-23`: the phase 0 and phase 1 rows cite "`roadmap/README.md`'s ledger" in their commits column; after Do 3 both point at a section that no longer exists. Two edits to `roadmap/LEDGER.md`.
- `GLOSSARY.md:72-74`, "ledger: ... the roadmap's ledger for closed steps" — should name `roadmap/LEDGER.md` after the move; and it omits `search/LEDGER.md`, a fourth ledger the seat skill cites (`.claude/skills/seat/SKILL.md:12`), while the brief's Goal and Read say "the other two ledgers". Say three, or say which two share the newest-end reading.
- `README.md:113-115`, "Starting a session": names `STATE-2026-10-19.md` and `GUI-NOTES.md` as the dated files; `STATE-2026-10-26.md` and `TIME-SEAM.md` are the same kind (the brief's own Not in scope lists four).
- `README.md:65-78`, "Roles": "An Opus session runs a step ... Independent steps may run as Opus workers in parallel, each in its own git worktree" duplicates TEMPLATE.md's worktree bullet (lines 63-65), which Do 5 keeps; one of them.
- `REVIEW-PROMPT.md:12` is one 140-character line ("Then these shapes, which the records have taught...") in a file otherwise wrapped at 72; a formatting nit the owner's file is due.
- `.claude/skills/session-close/SKILL.md:20-28` ("Why build order matters") still carries two sentences about release uplift and "a hash in a ledger names the form that built it" that nothing in the gate now depends on; the brief's (g)-style retirement question could include it. Nitpick.

## What I did not check and why

- I did not run `tools/docs-check.sh` or the gate: read-only brief. So "the five entries over 800 bytes and C73 uncited" (Not in scope, line 287) is unverified; C73 is cited only by `decisions/pricing.md` itself and the parks audit, which is consistent with "uncited".
- I did not count the "2d wrote nineteen" *Pinned:* paths (packet h): today `decisions/daemon.md` has 18 and `network.md` 11 such lines, so the number is at best historical; I did not trace it to 2d's commits.
- I did not verify the commit-message counts ("100 of 199 commits", "nine comments"): the first is close (`f1a29952..5f62d548` is 200 commits), the second is wrong by my count of the table (seven "on #N" rows: #53, #63, #64, #67, #70 twice, #87), but neither is in the brief.
- I did not open every closed issue a drop cites beyond #84, #88, #82; the ones cited by number for "on #N" (#53, #63, #64, #67, #70, #87) I read whole and each fits except #67 (finding 9).
- Every other hash the brief cites resolves (`5f62d548`, `10f24fd9`, `1bf30803`, `ea0b0255`, `f8ffb1ec`, `b350125b`, `893677a3`, `39b4d9e1`, `b1588df3`); the two review files named exist; `1f524d15:roadmap/parks-triage.md` and `0704a8f7:roadmap/governance/PARKS.md` (the Housekeeping citations Do 4 moves) resolve. Every backticked identifier exists except `JobKind` (a proposed type, also absent from the tree; the README uses it the same way) and `view.rows.fields` (finding 5). `STATE-2026-10-19.md` is 94,285 bytes; `AGENTS.md` is 15,241 against `budget AGENTS.md 15277` (`tools/docs-check.sh:55`); the 2g row is 3,811 bytes with its newline.

## Nitpicks

Findings 11 and 12; the issue letters skip I, K and L so "twelve lettered A to O" reads as fifteen; the milestone column writes "4-full" where the milestone is "4-full, the GUI"; the done-when's "under 13,000 bytes" is a target, not a measurement (TEMPLATE.md:42-45 wants the brief to say where the number came from — the 8 KB a step needs plus the standing sections, say); `grep -c '2026-1[12]-'` is 5 today, so that line is checkable, but `grep -c` exits 1 at zero matches, which a `set -e` shell in the worker's hands will read as failure.

## What got in the way

- The prompt asked me to judge "every backticked code name must exist in the workspace" while the README and brief name things to be built (`JobKind`) in the same backticks; a convention (plain text for a proposed name) would make that check mechanical.
- The routing table keys rows by the Open paragraph's heading words, which are not the README's exact bold text in four cases ("Tooling issues carry no area label" is; "From r3, the eviction" is; "The owner's" appears twice); a line number per paragraph, as of `5f62d548`, would have made the walk a diff rather than a read.
- The brief's Input says the owner "may strike or move a line", but the table has no row ids; a numbered table lets him answer "strike 23".
- Verifying `poe1@ee2084de` needs the sibling clone, which an outside reviewer may not have; a brief that cites a sibling's commit should say so in the citation.
