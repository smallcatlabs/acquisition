# The sitting's report

`result.json`'s `result`, whole, from the run of 2026-11-10 on `517836f8` (`README.md` beside this file). The runner's words; paths are the clone's.

---

# Governance seat report: four scenarios walked cold (read-only)

**Summary**
1. The rules got me to a correct path in all four scenarios. `decisions/process.md` (10 KB) does most of the work. `bothy.py pack` is the best single tool here.
2. The worst friction is in scenario 2. Nothing maps a user-facing text to where it lives. "Store is locked" has at least three candidate sources, and I found them only by grepping the code.
3. The rules contradict each other in a few places: whether a close always rewrites the road file, what "next" means (the road file has two "next" lists), whether an entry may carry the owner's words, and the step-id format (never defined).
4. One hidden trap: the count of cross-cutting rulings in `AGENTS.md` is capped at 18 by `docs-check.sh:91`, and there are already 18. `AGENTS.md` doesn't say so.
5. Total reading: about 20 KB auto-loaded, plus roughly 50 to 70 KB per scenario. About a fifth of that was spent resolving ambiguity rather than on reading a rule required.

No MEMORY.md appeared in my context, so I relied on none. `~/.claude/CLAUDE.md` was auto-loaded but says nothing about the project.

---

## Scenario 1: "Fix issue #200"

**Path**
| # | Read | Bytes | Sent by |
|---|---|---|---|
| 0 | AGENTS.md (+CLAUDE.md 11) | 19,135 | auto |
| 1 | `gh issue view 200` (open, area:frontends, trigger:step) | ~1,900 | prompt |
| 2 | decisions/process.md | 10,097 | AGENTS "how a step is reviewed and closed is `decisions/process.md`" |
| 3 | `bothy pack --path Cargo.toml` → **refused**: "no area governs Cargo.toml… Use --area" | 0.2 KB | AGENTS "`bothy.py pack --path <file>` prints what to read" |
| 4 | `bothy pack --area frontends` | 23,629 | the issue's area label |
| 5 | REVIEW-CHECKLIST.md preamble + frontends section | ~3,000 of 27,035 | AGENTS "its section of REVIEW-CHECKLIST.md" |
| 6 | roadmap/README.md | 4,754 | process.md intro, C180 |
| 7 | session-close SKILL | 4,209 | AGENTS Procedures |
| 8 | cold-review SKILL | 3,346 | C175 |
| 9 | HISTORY.md | 4,342 | ambiguity: does a close owe a line? |
| 10 | gate.sh grep for `features` | ~1,000 | the issue's "the gate's `features` stage" |
| 11 | `git log --grep='^Step: '` and the 517836f8 message | ~2,600 | ambiguity: step id; README rewrite |
| 12 | `gh issue view 179 --json state` | 100 | ambiguity: who closes the issue |

**Bill:** 19.1 KB auto, 50.0 KB required by rule, 8.1 KB to resolve ambiguity.

**Answers**
- **Brief?** No. C173: "A small fix needs no brief, since its issue states the goal and the done-when."
- **What to read:** the frontends pack and its checklist row (`REVIEW-CHECKLIST.md:111`). There is no module doc for `Cargo.toml`, and `pack --path` can't route a root file, so I picked the area from the issue's label.
- **Who builds:** Opus, per C172.
- **Review:** probably owed. C175 covers "a round that changes behaviour"; key order on every surface is behaviour if any golden moves. The reviewer is Codex with the issue text appended (cold-review §2), under C177. If no golden changes, it's unclear whether a feature flag alone counts.
- **What to run:** `tools/gate.sh` in full (session-close §1: "when unsure, the full gate"). The goldens must be regenerated "under `ACQ_UPDATE_FIXTURES=1`" (C122). **No document gives the actual command.** Before the push, run `--only doc` if doc comments change.
- **Commit message:** the story; `retired <row>; held by <Y>` (C154 and C182); `Co-Authored-By:`, which the hook rewrites (C184).
  - Stumble: C154's Y must be "a test, a golden, a check, a type or a dependency edge". The issue wants "the feature" named as the holder, and a crate feature is none of those five.
- **Commit and push:** yes, without asking (C171: "commits to `main`… pushes… without asking", "straight to `main` with no branch"). This contradicts my harness ("Commit or push only when the user asks… If on the default branch, branch first"). C183 says "the tree governs" over memory, but says nothing about the harness.
- **The close:** one commit with a `Step:` trailer, a check of the done-when, the retired and pinned lists, and the count of open `trigger:ruling` issues (14). No ledger row (C178), and no HISTORY line (session-close §5 adds one for a phase only).
- **Closing the issue:** no rule says to. Precedent shows #179 was closed by hand.

**Stumbles**
- **Step id format is never defined.** It appears only as `<step id>` (session-close:67) and `<step>` (cold-review:18). I inferred `frontends-200` from `search-179`. This matters because cold-review counts reviews by grepping that name.
- **Does the close rewrite the road file?** session-close:69 says it "rewrites `roadmap/README.md`". The 517836f8 close says "the road's state is unchanged… so roadmap/README.md is not rewritten." I followed the precedent.
- **Who takes #200?** README:70 says "Issues labelled `trigger:step` are taken by the next step in their area." The owner named #200 directly. I took the owner's word, but no rule says a named issue counts as its own step.
- **Where does the feature go?** gate.sh:234 sends feature pins to "the one block in crates/acquisition-cli/Cargo.toml". The issue says "the workspace manifest". This is a code question, but it left me unsure which manifest to edit.

---

## Scenario 2: "Make the locked-store refusal clearer"

**Path:** GLOSSARY.md 7,634 (to check "golden" and "door"). Then code greps, about 5 KB (`is locked`, `busy`, `LockError`). Then `world.rs:860-910`, about 1.5 KB. Then `bothy pack --path crates/acquisition-store/src/world.rs`, 15,325. I reused the frontends pack, process.md and session-close from scenario 1.

**Bill:** 19.1 KB auto, about 57 KB required by rule (two packs, process.md, session-close, checklist section), about 14 KB to resolve ambiguity.

**Answers**
- **Source of truth: not found by following any rule.** Grepping turned up three candidates:
  - `LockError`'s Display, "`<path>` is held by another process (pid N)" (`crates/acquisition-store/src/world.rs:881`)
  - the daemon's pipe text (`crates/acquisition-daemon/src/daemon/endpoint.rs:60`)
  - SQLite "refused as busy", governed by the README knobs `ACQ_STORE_WAIT` and `ACQ_ANNOTATION_WAIT`

  The world-lock text is asserted with `msg.contains(...)` (`crates/acquisition-cli/tests/world.rs:610`), not a golden. Under C153 ("a sentence literal elsewhere asserts only what no golden holds") it should probably become a golden. I would ask the owner which message he meant. That's one question, not a ruling.
- **Rulings that constrain the wording:**
  - C116: no ids or store names in a refusal
  - C143: the request's own words, never a frontend's flag
  - C53: "A failure names target, cause and evidence"
  - C11: the failure document
  - C122: wording is a golden naming the facts a reader must find
  - C153: one golden per wording
  - C146, if a pid or a count counts as "a count"
- **Owner's or mine?** Mine. AGENTS.md and C170 both list "the words of an answer" as the session's. I read a refusal as an answer, but no rule says so directly.
- **Which gate?** The full gate, because Rust source changes. session-close:11: "`--docs` serves a change that no test and no build reads."
- **Review?** Unclear. Is wording "behaviour" under C175? I'd say yes, since a refusal is behaviour at the boundary.
- **The close:** C173 says "A step is an issue or a brief". A one-line request from the owner has neither. Do I open an issue first so the close has a `Step:` to name? I'd guess a plain commit, no close.
- Running `acq` to see the text would require the mock-session skill (2,978 bytes), per the AGENTS hard rule.

**Stumbles:** there is no "where a text lives" map. AGENTS.md "Where things live" covers verbs, flags, knobs and install steps, but not refusals. `ACQ_UPDATE_FIXTURES=1` is named three times and its command is never given.

---

## Scenario 3: "Start the next step"

**Path:** AGENTS.md (auto), then roadmap/README.md 4,754 (AGENTS: "The road… read first for any step"), then process.md (already read). Then `gh issue view 229` (~2 KB), `gh issue view 216` (~2.2 KB), `gh issue list --label trigger:ruling` (~1.8 KB), and cold-review (already read). Two pointers I logged and did not follow: README:30 → `audits/governance-2026-11/README.md`, and README:36 → `audits/governance-2026-11/seat/BRIEF.md`.

**Bytes before I could name the next step with confidence:** about 39 KB (19.1 KB auto, 14.9 KB required, 6 KB of issues to resolve the conflict).

**The sentences that state it, and how they conflict:**
- README:34–37, under "Now": "Next, each for a fresh session, one at a time in this tree: 1. A cold agent sits… (#228) 2. The agent's memory is brought under C183 (#229)."
- README:45–50, under "Next": "In order. A session takes the first item it can… 1. What holds the invariants: #216…, #217…, #218…"

**How I resolved it:** #228 is this sitting, already running. #229 is about the private memory on the owner's machine, which a worker elsewhere can't reach (and it's under the memory this exercise told me to ignore). So by "the first item it can", the next step is **#216**. C180's own wording, "a session takes its first item", has no "it can", which would point to #229 instead. The README is the newer text, and I followed it.

**Answers for #216:**
- **Design:** the issue itself is the design. It has a done-when, so no brief is needed (C173).
- **Builder:** Opus (C172).
- **Review:** it changes a tool (arch-check), so a C175 review is owed: Codex with the issue text appended, following the cold-review skill's command.
- **What I may decide:** the check's mechanism.
- **What I must wait for:** the done-when edits invariant 1's `Pinned:` line in `AGENTS.md`. Is that "an invariant" or "the words of this registry", both owner's classes under C170? Bothy rule 9 says "*Pinned:* is kept true by whoever moves a holder", so it's mine. That answer took two documents to find.
- **Related queue item:** #219, on invariant 2, is open in the same area.

---

## Scenario 4: a new boundary ruling

**Path:** `tools/bothy.py` docstring (lines 1–103, ~6 KB; AGENTS: "in the form `tools/bothy.py`'s docstring states"). `bothy pack --area daemon`, filtered (daemon.md is 30,327 bytes; its intro claims "the protocol"). `tools/docs-check.sh:85–164`, ~5 KB, to find out what "the count of cross-cutting rulings" in process.md's Not-doing list means.

**Bill:** 19.1 KB auto, about 46 KB required, about 6 KB to resolve ambiguity.

**Answers**
- **Whose call?** The owner's. C170 lists "a boundary". If it has one clear answer, I can take it under delegation and file a `trigger:ruling` issue (C179), and the work goes on. If it has two defensible answers, it waits for him.
- **When?** Before the code. AGENTS: "recorded in `decisions/<area>.md` before the code that depends on it."
- **Where?** `decisions/daemon.md`, under "Daemon, jobs, protocol".
- **Id:** C186. That's the highest id plus one, which I found with grep. No rule says how ids are assigned.
- **What the commit must quote:**
  - Rule 9: "an entry's ruling and its *Why:* change only by his ruling, and the commit that changes them quotes him." For a new entry taken under delegation, there is nothing of his to quote. This is a gap.
  - AGENTS "Where things live" says "the owner's word first". Bothy rule 5 says "no owner's quote".
  - The registry itself carries quotes: C53 has "(the owner's word: "Yes, the index.")" and C165 has "(the owner, #165)".
  - docs-check only matches `owner:` (line 112), so those pass. The docstring and the check disagree.
- **Which checks read it:** `bothy check` (shape, headline under 160 bytes, six clauses or fewer, *Pinned:* paths exist, cross-references resolve). docs-check (duplicate ids, history markers, unknown or uncited ids, the cross-cutting cap). Both run in the gate's `bothy` and `docs` stages.
- **Cross-cutting?** Arguably yes, since every frontend must know it. But `CROSS_LIMIT=18` and AGENTS.md already holds exactly 18. Adding it fails the gate, and moving one out is the owner's call.

**Draft entry (proposed, not written):**
> `- **C186 — A frontend reads a protocol message's fields by name and assumes nothing of their order.** The daemon may write a message's keys in any order. A frontend that renders fields in order takes that order from the protocol type, never from the wire. *Why:* key order on the wire follows how a message was built, which one dependency feature can change for every crate at once. *Pinned:* nothing yet.`

---

## Cross-cutting lists

**Working nouns I couldn't resolve from GLOSSARY.md or the sentence**
- "phase" (session-close §5, GLOSSARY "close (a phase's)"). It's used but never defined.
- "group" ("the test-shape group", README:56; "answer-facts group", HISTORY).
- "4-full", "morning" (README:38, 52).
- "his queue". This one does resolve, via C179.
- "under the owner's delegation" (C146). Partly resolves via C170.
- "the stopping rule". session-close:42 gives this gloss for C153, but C153's title says nothing about stopping.
- "this tree" (README:34). Does a seat-world copy count?
- "sitting". GLOSSARY defines it as a trade-site session, but #228 and this exercise use it for a governance seat.
- "the five-call door" (HISTORY).
- "door". GLOSSARY says new text should not use it, yet AGENTS.md does (the crate table; C46 "the frontend door grep"; C122 "the door").

**Stale or mismatched pointers**
- GLOSSARY "step… with one ledger line" and "fold… its ledger lines". C178 says "no file retells it".
- session-close:69 "rewrites roadmap/README.md", against precedent 517836f8.
- Bothy rule 5 "no owner's quote" against the entries C53, C121, C165 and C168, and against docs-check's narrower regex.
- AGENTS "the owner's word first" against rule 5.
- C154's list of holders has no place for a crate feature, which #200 asks for.
- `pack --path Cargo.toml` refuses a root file.
- `ACQ_UPDATE_FIXTURES=1` is named three times; the command is never given.
- The pins in AGENTS.md leave out the `crates/` prefix. This is legal under rule 7, but I had to check it.

**Rules stated in more than one place**
- The trailer (AGENTS hooks note, C184, session-close §2): **agree**.
- The small-fix "no brief" rule (C173 and the precedent close): **agree**.
- What gets reviewed (C175, the cold-review description): **agree**, and the skill rightly defers to C175.
- Agent-facing design (AGENTS hard rule, C170): **agree**. AGENTS adds "entered in the registry only when the close finds it a property another area must know".
- Build order (AGENTS, C185, session-close): **agree**.
- Retirement (C154, C182, session-close): **mostly agree**. C154 covers "a test or a check", while C182 covers rows and lines.
- The next item (C180 "its first item", README "the first item it can"): **differ**.
- Commit and push (C171, the harness): **differ**.

**Rules I couldn't tell applied to me**
- C185, "a window he names": how would a session know?
- README "one at a time in this tree".
- "taken by the next step in their area", when the owner names an issue directly.
- C177 "on the owner's account": does a worker on another machine have Codex?
- C175 "behaviour": does a feature flag count? A refusal's wording?
- "the test-shape group's t6".

**Reading bill per scenario (auto / required by rule / ambiguity)**
| Scenario | Auto | Required | Ambiguity |
|---|---|---|---|
| 1. Fix #200 | 19.1 KB | 50.0 KB | 8.1 KB |
| 2. Locked-store wording | 19.1 KB | ~57 KB | ~14 KB |
| 3. Next step | 19.1 KB | 14.9 KB | 6 KB |
| 4. New ruling | 19.1 KB | ~46 KB | ~6 KB |

---

## Ten changes to the instructions, ranked by what they'd have saved

1. **Define the step-id format** in C178 or session-close §3 (e.g. `<area>-<issue>` or the brief's name). Cold-review's counting depends on it. *(Scenario 1, step-id stumble.)*
2. **Make the road file one ordered "Next" list**, marking items that need the owner's machine or memory. *(Scenario 3, the two conflicting lists.)*
3. **Add "a refusal or answer text" to AGENTS "Where things live"**, saying where it lives and which golden holds it, plus the exact regeneration command. *(Scenario 2, about 14 KB of grepping.)*
4. **State the 18-ruling cross-cutting cap in AGENTS.md** next to the list, and say that it's full. *(Scenario 4.)*
5. **Reconcile "the owner's word first" with rule 5**, and make docs-check enforce what the docstring says. *(Scenario 4.)*
6. **session-close §3:** "rewrites `roadmap/README.md` when the step changed it", and "closes its issue". *(Scenario 1, precedent vs skill, and the issue close.)*
7. **C175:** say whether wording-only changes and dependency or feature changes count as "behaviour". *(Scenarios 1 and 2.)*
8. **C154:** add "a crate feature" to the list of holders, or route rows to C182's wording. *(Scenario 1.)*
9. **GLOSSARY:** define phase, group, step, round and slice against each other; fix step's "ledger line"; resolve "door" against AGENTS.md's own use. *(Throughout.)*
10. **One sentence in AGENTS.md or C171 on precedence over the harness defaults** ("commit only when asked", "branch first"), and on an owner-named issue being its own step. *(Scenario 1.)*

## Three things to keep exactly as they are

1. **`bothy.py pack`**: one command gives the area's rulings plus a file:line index of where each ruling is held. It shortened scenarios 1, 2 and 4.
2. **`decisions/process.md`**: ten short rulings, each answering one "who, when, how" question. C173's first sentence settled the brief question in a single read.
3. **Close commits as the record** (`git log --grep='^Step: '`). The 517836f8 message answered three ambiguities the rules left open (step id, whether to rewrite the road file, the ruling count).
