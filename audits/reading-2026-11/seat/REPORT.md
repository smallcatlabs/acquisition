I followed the rules as written for all four scenarios, reading only. I edited nothing and ran nothing but the allowed read-only commands. I used no memory, and I did not open `audits/governance-2026-11/` or `audits/reading-2026-11/`; `GLOSSARY.md:133` points into the first of these, and I did not follow it.

## Summary
1. Doing work goes smoothly. `decisions/process.md` (15,463 B) answers almost every "may I / must I" question in one place, and `bothy pack` and `bothy show` are excellent.
2. Finding the next step is not smooth. The roadmap file hands me on to 58 KB of audit documents, and the owner's open decisions are not in the `trigger:ruling` queue the roadmap names as his queue.
3. Issue #200 is closed. The lowest open `trigger:step` issue in the list was #212, and it is not a small fix: the issue's own comment owes a brief and two reviews, even though C173 says a small fix needs no brief.
4. Who may change a ruling's words differs between `tools/bothy.py` (rules 6 and 9: only the owner, and the commit quotes him) and C170/C192 (a session writes and amends entries).
5. AGENTS.md gives two reading rules in one sentence (read the area's rulings file whole, and run `pack`) and says neither is enough on its own. For #212 that is 28.7 KB of pack plus 35.7 KB of area files.

## Scenario 1: Fix issue #200 (taken as #212)

**Path**
- Auto-loaded (all scenarios): `AGENTS.md` 20,942; `CLAUDE.md` 11; `~/.claude/CLAUDE.md` (I could not measure it; my shell was refused that path). About 22 KB.
- `gh issue view 200`: about 1.5 KB. Closed.
- `gh issue list --label trigger:step --limit 30`: about 3.5 KB. The lowest number shown is 212. Lower open ones may exist past the 30 shown.
- `gh issue view 212 --comments`: about 7 KB.
- `decisions/process.md`: 15,463. AGENTS sends you there "Before starting or closing a step".
- `roadmap/README.md`: 5,187. AGENTS: "read first for any step".
- `bothy pack --path crates/acquisition-store/src/lib.rs`: 28,739. AGENTS: "prints what to read before touching a file".
- `bothy show C193`: about 1 KB.
- `REVIEW-CHECKLIST.md`, the store and frontends sections: about 9.5 KB. AGENTS: "its section".
- `session-close/SKILL.md` 4,655 and `cold-review/SKILL.md` 4,243. `process.md:3` says "the mechanics are in the skills".
- `GLOSSARY.md`: 8,136. Read to resolve what "door" means.
- Owed but not read: `decisions/store.md` 15,586 and `decisions/frontends.md` 20,145 (AGENTS: "read its rulings file"; #212 has two areas), `roadmap/briefs/TEMPLATE.md` 5,274, and the module docs of `store_cmd.rs`, store `lib.rs`, the verbs crate and `arch-check.sh`.

**Bill:** auto-loaded 22 KB; mandatory 75 KB read plus 41 KB owed (116 KB, before the module docs); needed in practice 13 KB.

**Answers**
- **What to read:** "Before touching an area, read its rulings file, its section of `REVIEW-CHECKLIST.md`, and the module doc" (AGENTS, Workspace).
- **Brief or no brief:** C173 says "A small fix needs no brief." But #212's last comment says "A behaviour change, so its brief is reviewed and its build reviewed cold." It has five build items and a new type, so I would write a brief: Opus plans (C172), the brief stays under 8,000 B (C174), and it is reviewed with `BRIEF-REVIEW-PROMPT.md` at most twice (C176). The design itself is already ruled (C193, "As drafted"), so the owner does not rule on the brief (C173 requires that only for a design brief).
- **Review:** C175 ("A round that changes behaviour … is reviewed cold"). C177 and the cold-review skill: Codex, `codex exec --sandbox read-only "$(cat roadmap/briefs/REVIEW-PROMPT.md) The commit: <sha>." < /dev/null`, at most three rounds.
- **What to run:** `tools/gate.sh`, full ("when unsure, the full gate"). Before it, `cargo build --workspace` (AGENTS build order).
- **Commit message:** one concern per commit, with the story in the message (session-close §2). A line `retired <facts half of the door grep>; held by <reader type>` (C154; #212 item 5). Ending `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>` (C184). The close commit adds `Step: <brief filename>` (C178).
- **Commit and push:** yes, without asking. C171: "commits to `main` when the gate exits 0 and pushes … without asking"; "A push stands alone in its command." AGENTS says this grant stands over the harness default of committing only when asked. The two agree.
- **What else to write:** close #212, naming the close commit. Rewrite `roadmap/README.md`, since Next item 5 names #212 (C178, C180). C193's `Unbuilt: issue #212` and `Pinned: nothing yet` must change. No ledger row and no HISTORY line (HISTORY is only for a phase close, session-close §5).

**Stumbles**
- C173 against #212's comment (quoted above). The issue label does not say whether a brief is owed.
- Taking `Unbuilt` out of C193 edits the ruling text. bothy.py rule 9 says "an entry's ruling and its *Why:* change only by his ruling"; C170 says "a session writes and amends this registry's entries." I would follow C170, the newer text, but that is a guess.
- Step id: session-close says "for an issue with no brief its area and number (`search-179`)". #212 has two areas.
- GLOSSARY.md:11 says new text uses the specific word rather than "door". C124 and C193 ("the verbs' writing door") are new and still use "door".

## Scenario 2: The locked-store wording

**Path:** `git grep` for the lock wording (about 4 KB of output) found three candidates:
- `annotations.rs:658` ("… is busy: another process held it …"),
- the daemon's `start.rs:394` ("another daemon holds this world"),
- SQLite's raw "database is locked".

I took the first: it is the only one with a golden, `cli/tests/goldens/price_json/a_busy_intent_file.txt` (about 1.3 KB). Then `bothy list decisions --area frontends` (about 3 KB). Owed: `decisions/frontends.md` 20,145 and a pack for `annotations.rs` (about 28 KB, mostly the store pack again).

**Bill:** auto-loaded 22 KB; mandatory about 22 KB new on top of scenario 1, or about 84 KB from cold; in practice about 7 KB.

**Answers**
- **Source of truth:** `HELD` plus the `AnnotationError` Display impl in `crates/acquisition-store/src/annotations.rs:603–664`. It is one constant shared by several refusals, so the checklist row "A sibling sentence left standing" applies.
- **Regenerate:** `ACQ_UPDATE_FIXTURES=1 cargo nextest run -p acquisition-cli --test main <filter>` (AGENTS, Where things live). Edit the golden's `# holds` lines by hand and read the diff before committing; C122 says "a blind regeneration fails". Also `price_json.rs:2683` asserts the phrase.
- **Rulings on wording:** C116 (no id), C53 (a failure names target, cause and evidence), C146, C149, C153 (one golden per wording), C122.
- **Gate:** full. session-close §1: `--docs` only for "a change that no test and no build reads"; this changes Rust and a golden.
- **Whose decision:** the session's. C170 lists "the words of an answer" as agent-facing design.
- **What the close owes:** commit the story; review if C175 counts a wording change as "behaviour". I would say yes, but no sentence says so.

**Stumbles:** "store is locked" matches three refusals and nothing maps the owner's phrase to one of them. The owner's spoken request is neither an issue nor a brief, so C173 ("A step is an issue or a brief") and the step-id forms in session-close §3 give it no id. I could not tell whether a wording change needs a cold review.

## Scenario 3: Start the next step

**Path:** `roadmap/README.md` 5,187 (already read). Next item 1 sent me to `audits/whole-review-2026-11/NEXT.md`, 27,064 (I read about 14 KB of it). The road's "run as `audits/meta-2026-11/README.md` amends it" sent me there too: 31,447 (I read about 6 KB). Then `gh issue list --label trigger:ruling`, open (empty) and closed (about 2 KB), and a grep of `README.md`.

**Bill:** auto-loaded 22 KB; mandatory 63.7 KB (the road plus both programme files whole); in practice 2.5 KB. I could state the next step after about 47 KB actually read.

**The sentences that state it**
- `roadmap/README.md:39–41`: "The programme's later steps, as `NEXT.md` orders them: A4, then B1 to B6, each waiting on what that file names."
- `NEXT.md:66` (A4 row): "Sonnet | his word, for two of three".
- `NEXT.md:156`: "The README stops saying there is no UI. Now." `README.md:5` still says "there's no UI", so that line is the first actionable work.

**Answers**
- **Design:** `NEXT.md` §A4.
- **Brief:** none (no `next-a4.md`). Per `NEXT.md:66` it would be cut from `issues/` and `next/traces.md`.
- **Builder:** `NEXT.md` says Sonnet in lane 3. C172 says "Opus plans and builds".
- **Reviewer:** Codex (C177). A README-only change gets no review (C175: "only comments and documents gets no review").
- **Mine to decide:** the README line, and whether the reading rule is "registry wording". The meta review says "The registry's wording: agents own it" (meta:468).
- **Must wait:** filing the 33 drafted issues (owner's decision 6).

**Older against newer text**
- `NEXT.md` (Fable orchestrates, Sonnet builds) against C172 (process.md, newer).
- `NEXT.md:481` ("his grant of `gh` writes and pushes") against C171 (push without asking).
- `NEXT.md`'s "the reading rule is the owner's" against meta:468 and C170's amendment.

I took the newest commit (C170, b2cd41a8) as governing, by date alone; no rule says to.

**Other stumbles**
- The road says "In order. A session takes the first item" but then says "Where they stand against the items below is the owner's (its decision 1)". The file does not settle its own order.
- `roadmap/README.md:73` says "His queue is the open issues labelled `trigger:ruling`". That queue is empty, yet lines 76–77 list four of his decisions that are not filed as issues, against C179.
- "Lane": I could not tell whether I am one, or whether "only one lane at a time changes goldens" binds me.

## Scenario 4: A new boundary ruling

**Path:** `tools/bothy.py` docstring, lines 1–90 (about 6.3 KB of 42,224; AGENTS: "the form `tools/bothy.py`'s docstring states"). `bothy list` (about 10 KB), sorted by hand to find the highest id. A grep of `tools/docs-check.sh`. Owed: `decisions/daemon.md` 30,753, the protocol's rulings file per the workspace table.

**Bill:** auto-loaded 22 KB; mandatory about 47 KB; in practice about 4 KB.

**Answers**
- **Whose:** the owner's. C170: "His classes are an invariant, a boundary…". With two defensible answers, file an open `trigger:ruling` issue saying what waits, then wait (C179). With one clear answer, take it under delegation and file it closed, titled "Taken under delegation: …". #212's history shows the practice: draft the words and put them to him.
- **Where and when:** `decisions/daemon.md`, before the code (AGENTS: "recorded in `decisions/<area>.md` before the code that depends on it").
- **Id:** C201. The highest is C200 (`process.md:36`). `bothy list` prints by area, so its last line is not the highest id; I had to sort.
- **What the commit quotes:** his words (bothy rule 9), never inside the entry (rule 5, which bans an owner's quote, a date and "Decided"/"Ruled").
- **Checks:**
  - `bothy check`: headline at most 160 B, at most six clauses, a gloss on each cited id, `Pinned` naming at most three files that exist.
  - `docs-check.sh`: history markers, and the cross-cutting cap of 18.
- **Cross-cutting list:** involved only if this amends C12 ("A frontend consumes exactly two surfaces"). The list is at 18 now, so adding one means moving another out to its area file.

**Draft entry**

- **C201 — An observing frontend learns a live daemon's contract revision from its hello and never replaces it.** The observe connect policy reports a daemon of another revision as live and names its revision. It neither stops nor replaces that daemon. Replacement stays the use policy's, as C10 (single-version handshake) has it. Unbuilt: issue #212. *Why:* the verbs' writing door (C193, the frontends' facts door) must refuse a migration under a live daemon of another revision, and cannot tell which one it may not touch. *Pinned:* nothing yet.

The main sentences count five clauses. I did not check whether existing daemon rulings, such as C86, already cover this.

## Cross-cutting lists

**Nouns I could not resolve from GLOSSARY or the sentence**
- lane (`NEXT.md:338`)
- spike (C176)
- series and programme (C171; road:13)
- facts door and writing door (C193, C124: GLOSSARY says not to use "door" this way)
- law list and projector (`NEXT.md`)
- "A2's table" (`NEXT.md:352`)
- contract revision (resolved only through C84 in the pack)

**Stale pointers**
- `README.md:5` still says "no UI".
- AGENTS' invariant paths leave out `crates/`. They resolve under bothy rule 7, but AGENTS never says so.
- #212's first comment names the removed `acq store import`.
- `NEXT.md:399` names the memory `orchestrating-steps`, which is not in the tree.
- GLOSSARY:133 points into a forbidden audit directory (logged, not followed).
- The gate's stage list in AGENTS matches `gate.sh:102` exactly.

**Rules stated in more than one place**
- Commit and push: AGENTS, C171 and session-close §4 agree. `NEXT.md:481` differs.
- The commit trailer: AGENTS, C184, session-close §2 and the harness reminder agree.
- The owner's classes: AGENTS Hard rules and C170 agree.
- Ruling shape and who changes it: bothy rules 6 and 9 against C170/C192. They differ on owner-only changes and on whether a new id needs the owner.
- Review counts: C176, cold-review §1 and `NEXT.md` "Checks" agree.
- Retirement lines: C154, C182 and session-close agree.
- The owner's queue: C179 and session-close agree. The road's own lines 73 and 76 differ from each other.
- What to read before editing: AGENTS gives two rules in one sentence, with no precedence.

**Rules I could not classify as applying to me or not**
- C185, "a window he names": a fresh session cannot know.
- C186, "a cloud session": I assumed the Mac.
- The road's "test-shape group's t6".
- The lanes in `NEXT.md`.
- C175's "unless it cuts or moves prose across many files".
- Whether #212 counts as "a small fix" under C173.

**Bills by scenario**
| Scenario | Auto-loaded | Mandatory | Needed in practice |
| --- | --- | --- | --- |
| 1 | 22 KB | 116 KB | 13 KB |
| 2 (new on top of 1) | — | 22 KB | 7 KB |
| 3 | 22 KB | 64 KB | 3 KB |
| 4 | 22 KB | 47 KB | 4 KB |

## Ten changes to the instructions, ranked
1. Make the roadmap's "Next" state the actionable item itself, with its wait and its builder (for example "A4a, README line, no wait"), not a pointer to 58 KB of audit documents. (Scenario 3: 63.7 KB to resolve one line.)
2. File `NEXT.md`'s owner decisions as `trigger:ruling` issues, so "his queue" is the label alone. (The empty queue against road:76.)
3. Give AGENTS one reading rule, either the pack or the rulings file whole, and say which governs. (Scenario 1: 28.7 KB of pack plus 35.7 KB of area files owed.)
4. Make bothy rules 6 and 9 agree with C170/C192 on who amends an entry and who grants a new id. (Scenario 4, and removing `Unbuilt` at #212's close.)
5. Make a `trigger:step` issue say whether it owes a brief, or give C173 a test for "small". (#212's comment against C173.)
6. Give a step-id form for an owner's spoken request and for an issue with two areas. (Scenarios 1 and 2.)
7. Say whether a programme file can override C172's models and C171's push grant, or bring `NEXT.md` in line. (Scenario 3.)
8. Have `bothy list` print the next free id. (The hand sort.)
9. Add lane, spike, series and programme to GLOSSARY, and bring C124/C193's "door" in line with GLOSSARY:11.
10. Say in C175 whether a wording change counts as a behaviour change. (Scenario 2.)

## Three things I would keep exactly as they are
1. `bothy pack` and `show`, especially `show`'s "Named by" and "Held in code: nowhere".
2. A golden's header and its `# holds` lines: the file tells you how to regenerate it and what must survive.
3. C170's explicit class list together with C171's commit-without-asking. Most "may I" questions ended there in one sentence. #212's comments, which record the owner's ruling whole, made scenario 1's design question a lookup.
