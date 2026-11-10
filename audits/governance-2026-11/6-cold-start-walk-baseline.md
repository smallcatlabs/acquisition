**Blind sitting at the governance system: report**

**Summary**
1. The rules are dense but mostly findable. AGENTS.md routes well to the README's "A step", `briefs/TEMPLATE.md`, the session-close skill, REVIEW-PROMPT and the bothy entry rule, and those four agree on the step rhythm.
2. The biggest cost is `roadmap/README.md` (59,270 B). Its governing "Current step" sentence is the last line of a 26 KB section, and that section holds several earlier "Next, …" sentences it has since overridden. I had to read all of it to name the next step.
3. Commit and push permission is the one rule I could not settle. AGENTS.md says "a commit or push waits for his word". The template says "Each concern may commit after its own targeted tests". A grant scoped to an earlier step allows pushing at the close.
4. A small issue fix (#200) has no explicit "may just fix it" path. In practice it needs a brief, because REVIEW-PROMPT reviews "against the brief its message names".
5. I found at least five stale statements. The worst is a ruling that says the `artifacts` gate stage was retired; it was reinstated at 45cb2946, and the gate still runs it.

I did not rely on MEMORY.md. It is in my context, and I noticed overlaps such as the Codex command and "Closes #N", but I took every answer below from the tree. I checked that `codex` is installed with `which`.

---

## Scenario 1: "Fix issue #200"
#200 is open and labelled `area:frontends` and `trigger:step`. It asks to turn on serde_json's `preserve_order` and retire the checklist row about key order.

**Path (bytes)**
1. AGENTS.md 16,689 and CLAUDE.md 11 (auto-loaded).
2. `gh issue view 200` 1,737 (the owner's line).
3. `bothy.py pack --path Cargo.toml` (AGENTS: "prints what to read before touching a file"). It refused: "No area governs Cargo.toml … Use --area."
4. `bothy.py pack --area frontends` 33,428. About 16.7 KB of it repeats AGENTS.md; the new part is `decisions/frontends.md` (18,696).
5. REVIEW-CHECKLIST.md, the frontends section (lines 103 onward), 6,799 (AGENTS: "its section of REVIEW-CHECKLIST.md").
6. roadmap/README.md: "Roles" and "A step" 5,608 (sent there by the template and session-close), then the rulings list 23,827, to find the commit, push and review grants.
7. `briefs/TEMPLATE.md` 6,284 and `session-close/SKILL.md` 2,568.
8. `briefs/REVIEW-PROMPT.md` 3,888 (named in Roles).
9. `tools/gate.sh` header 5,631, to check the stages the issue names (`features`, `artifacts`).
10. LEDGER.md head and tail, about 2,700.
11. `git log -S` on README and gate.sh, to settle the `artifacts` contradiction.

**Answers**
- **What to read.** AGENTS.md:96–97 says: "read its rulings file, its section of REVIEW-CHECKLIST.md, and the module doc you will change". Cargo.toml has no module doc and pack names no area, so I fell back on the issue's label (frontends). In practice a change to the workspace manifest touches every area.
- **Brief or just fix?** Nothing exempts an issue fix. README:394 says "Brief, build, gate, commit, review, close. This is the one statement of the rhythm". Pull-forward (README:458) says each item "takes a brief from `briefs/TEMPLATE.md` written by the session that starts it". REVIEW-PROMPT:3 reviews "against the brief its message names", and TEMPLATE Rules require "A commit message names this brief's file". The precedent is the night run, which wrote one brief per issue (README:840ff). **Answer: write a brief.** Under TEMPLATE "Input", that means reading the last push's CI first (`gh run list … "Gate (Linux)"` and the same for Windows). I did not run it here.
- **Review, and by whom?** README "A step" 3: "The review runs over each committed step". The change touches non-test code and goldens, so it needs one. Roles: "Codex reviews, read-only, launched by the session that made the commit: `codex exec --sandbox read-only "$(cat roadmap/briefs/REVIEW-PROMPT.md) The commit: <sha>."`". The findings are saved at `roadmap/reviews/<step>-<sha>.md` and committed on their own, and the owner reads them before any fix (step 4). The ruling of 2026-11-09 counted a sub-agent review "Codex not being installed", which reads as a one-off.
- **What to run.** The full gate, `tools/gate.sh` (AGENTS "The gate decides"; session-close §1). Goldens regenerate under `ACQ_UPDATE_FIXTURES=1` (C122), but no document gives the exact command. The gate unsets the variable, so it is a separate nextest run. Each regenerated golden is then read as a diff (REVIEW-PROMPT shape 9).
- **Commit message.** It carries:
  - the story, with sizes measured after the last edit (session-close §2);
  - the brief's file name (TEMPLATE);
  - each id named in words on first use (TEMPLATE: "C84, the artifact mismatch");
  - the line `retired <checklist row>; held by serde_json preserve_order` (C154, and the issue's done-when).

  The hook rewrites the Claude trailer to `Assisted-by:` (`.githooks/commit-msg`), so the attribution conflict I suspected is not one. Every commit must be signed (TEMPLATE: "signed, as every commit here is").
- **May I commit and push?** The rules conflict:
  - AGENTS.md:83: "A ruling (…) is his, recorded in `decisions/<area>.md` before the code that depends on it; a commit or push waits for his word." It is unclear whether the clause covers only rulings or every commit.
  - TEMPLATE Rules: "Each concern may commit after its own targeted tests".
  - README:300ff, the grant of 2026-11-01: "commits on main … and pushes at the step's close". It is scoped to the stuck-verdict step.

  I resolved it as: commit locally, and push only on his word.
- **What else to write.** The close commit checks the done-when line by line and lists the agent-facing design decisions. It writes a LEDGER.md row and sets "Current step" (A step 5). HISTORY.md only at a phase close (step 6). Closing the issue is not stated anywhere.

**Stumbles**
- `pack --path` cannot place a workspace manifest.
- C154 covers "a test or a check". It does not say whether a REVIEW-CHECKLIST row counts; only the issue says so. The checklist's own intro has no retirement rule.
- "Current step" must be set by "the build's last commit" (A step 2). But the current value is the owner's live morning, and overwriting it for an off-road issue felt wrong. No rule covers an off-road issue.
- The `trigger:step` label means "the next step in its area takes it" (README:349), but road steps are not keyed by area.

---

## Scenario 2: "The locked-store refusal is confusing"

**Path:** AGENTS (auto); `grep` sweeps for "locked" and "busy"; `crates/acquisition-store/src/world.rs`:863–900 (about 1,500); `crates/acquisition-store/src/annotations.rs`:631; `crates/acquisition-cli/tests/world.rs`:592–615 (about 1,500); `decisions/frontends.md` 18,696 (already read); `decisions/store.md` 12,682 (mandatory, because the text lives in the store crate); gate.sh header (already read); session-close (already read).

**Answers**
- **Source of truth.** There are two candidates, and I would have to ask the owner which he means:
  - `LockError`'s Display in `world.rs`:880–896: "{path} is held by another process (pid N)".
  - `annotations.rs`:631: "annotation file is busy: another writer held it…". This one appears to break C116, which says no store name such as `annotation` "stand[s] in a refusal". That is a finding in its own right.
- **Goldens.** No golden holds the lock text. It is pinned by a sentence literal, `assert!(msg.contains("held by another process"))`, in `crates/acquisition-cli/tests/world.rs`:610. That conflicts with C153 ("A surface's wording is held by one golden"). The change should move it to a golden and then regenerate under `ACQ_UPDATE_FIXTURES=1` (C122). The references change only if help text changes. The way to find the affected tests is the TEMPLATE "Read" rule: "names every test that asserts on it… found by grepping".
- **Rulings that constrain the wording:**
  - C116: no ids, no store names.
  - C143: the request's words, not a frontend's flag.
  - C53: "A failure names target, cause and evidence".
  - C11: `{"error":…}` with exit 1.
  - C146, if a count appears.
  - REVIEW-PROMPT shape 4: "a refusal that says what is wrong and not what to do".
- **Which gate.** The full gate. session-close: `--docs` "serves a change that no test and no build reads … when unsure, the full gate". A test reads this text.
- **Whose decision.** Mine. AGENTS:85 makes "the words of an answer" the session's: "recorded in the commit that builds it, listed at the close". One doubt: AGENTS lists "anything touching the owner's real store" as a ruling. I read that as covering behaviour, not wording.
- **The close.** Retirement line: `retired the literal in world.rs; held by golden …` (C153 and C154). It lists the wording as an agent-facing decision for his review.

**Stumble:** "the refusal when a store is locked" maps to two texts in two areas, and no glossary entry covers "lock". `world` in GLOSSARY.md mentions a lock but does not name the refusal.

---

## Scenario 3: "Start the next step"

**Path:** AGENTS (auto) → `roadmap/README.md` 59,270, all of it → `briefs/4-full-1-design.md` §2 and §7 (about 4,500 of 23,757) → LEDGER.md tail (about 1,500) → RUN-LEDGER.md tail (about 2,000, to check whether the morning has happened) → `gh issue list --label trigger:step`.

**Bytes before I could name the next step with confidence:** about 63,000 beyond the auto-loaded files. That is the whole README, because the governing sentence is its last line, plus the RUN-LEDGER tail to confirm no morning row exists.

**The governing sentence** (README:914–917): "Current step: the owner's morning at `acq web` in his own store under the live-run skill; step 6, the dense table, follows the morning, its spike first (§7 step 6)."

**Answers**
- **The next step is not mine.** It is the owner's live morning; the last RUN-LEDGER row is dated 2026-11-03. After it comes step 6.
- **Design:** `briefs/4-full-1-design.md` §2 ("The dense table and the toolchain") and §7 step 6. No step-6 brief exists.
- **Who:**
  - The spike comes first, "in the owner's browsers"; his part waits for him.
  - Then the brief, written by the session, or "comes to the owner with two shapes".
  - Then the build, by an Opus session (Roles; the model ruling of 2026-11-01: "use opus 5.5 … unless you see a strong argument for … Fable").
  - Then the Codex review with the Roles command.
- **What I may decide:** agent-facing design. A ruling (two shapes, or the browser-check driver if it adds a CI job) goes to him.
- **What I must wait for:** his morning, and his browsers for the spike.

**Older text against newer:**
- Earlier sentences in Current step have since been overridden by later ones but still sit in the same section: README:538 "Next, Phase 3, the owner's league run, at his word", and :552 again.
- The phases table (row 4) says "after `alpha.2`, 4-full". The ruling of 2026-11-06 says "4-full's first slice moves ahead of `alpha.2`".
- Pull-forward items 1 and 2 (4-0 and the test-shape tranche) are closed but still listed, against "An item leaves the list when its step closes".
- I resolved each by date order: the newest dated ruling governs, and so does the last sentence of Current step.

**Gap:** The 2026-11-09 work (#204 and the new labels) appears in the rulings list but not in Current step or LEDGER.md. There are ten open `trigger:step` issues, and no sentence says whether a session may take one while the road waits on the owner.

---

## Scenario 4: a new boundary ruling

**Path:** AGENTS (already read); `tools/bothy.py` docstring 4,098 (AGENTS: "in the form `tools/bothy.py`'s docstring states"); `decisions/daemon.md` 32,421 (the protocol crate's rulings file per the crate table; I read the parts around C85); `bothy show C85` (about 1,000); `bothy list` for the highest id; `docs-check.sh` grep (about 1,000).

**Answers**
- **Whose:** the owner's. AGENTS:81: "A ruling (an invariant, a boundary, …) is his"; C12: "A third channel is a protocol or store change, ruled first".
- **When:** before the code. It is the brief's Do 1: "A ruling the step needs and does not have is the first item, brought to the owner before the code" (TEMPLATE).
- **Where:** `decisions/daemon.md` (protocol) or `decisions/frontends.md` (documents, C53 and C115). I chose frontends.md, because key order is a property of the verbs' documents; the choice was a guess. Bothy rules 1–7 set the form.
- **Id:** C170. No rule says how ids are minted; I took the next number after `bothy list` (C169 is the highest).
- **What the commit quotes:** him (bothy rule 7: "the commit that changes it quotes him"). Precedent also adds a dated line to the README rulings list ("Each in his words, with the commit that landed it").
- **Checks:**
  - `bothy check`: cross-references, *Why:* and *Pinned:*, paths exist.
  - `docs-check.sh`: entry limit of 800 B, history markers, uncited ids, citations from code.
  - TEMPLATE: the drafted line passes `bothy check` before the brief is committed.
- **Cross-cutting list:** only if "every area must know" it. If so, AGENTS.md is already over its budget (16,689 > 16,600; docs-check only prints a note). Its byte budget, and earlier its entry count, has only grown by his word.

**Draft entry** (my estimate is about 620 B; I did not measure it):
`- **C170 — A document's keys reach every surface in the order its maker built them.** A struct keeps its declared order and a map its insertion order, on the wire, in a golden and in a reference alike; a frontend prints keys in the order it receives them and reorders none. A reader still finds a key by its name, and no key's position is a contract a program may branch on (C53, three output levels). *Why:* an answer leads with what bears on the question (C115, the answer leads) only if its order survives every serialization. *Pinned:* `acquisition-verbs/tests/documents.rs` (`a_receipt_keeps_its_documents_key_order_and_its_undo_after_it`); Unbuilt, #200.`

---

## Cross-cutting lists

**Working nouns not in GLOSSARY.md:** packet (README:32, 40); cut / re-cut (of a brief, README:228); cold review (README:240); Do 1 (README:61); orchestrating session (README:67); spike (README:226); morning 1 / the two mornings (README:242); the bound (README:868); the group / the group's pace (README:98, 117); keystone (phases table); the human door (C139). Two more are defined only by narrative: "Fable" (Roles) and "residue" (README:290).

**Stale pointers**
1. README:111–113, ruling of 2026-10-29: "`artifacts` retired, held by `features` … with the Build order rule's words in AGENTS.md following it". The stage was reinstated at 45cb2946 ("the artifacts stage kept — not retired"), the gate still runs it (gate.sh:93), and AGENTS says "held by the gate's `artifacts` stage". The ruling text was never corrected.
2. AGENTS "Quality gate" lists nine commands. gate.sh runs thirteen stages, adding `winclippy`, `features`, `artifacts` and `tree`; AGENTS covers `artifacts` only as "the build again".
3. Phases table row 4 versus the ruling of 2026-11-06.
4. Pull-forward items 1 and 2 are closed but still listed.
5. Current step holds outdated "Next, …" sentences, and it omits the 2026-11-09 work.
6. `bothy pack --path` refuses Cargo.toml, although AGENTS presents pack as the per-file router.

**Rules stated in several places**
- *Agree:* the CI-read rule (README ruling of 2026-10-29, A step 1, TEMPLATE Input).
- *Agree:* retirement (C154 in AGENTS, session-close §2, TEMPLATE, REVIEW-PROMPT shape 10).
- *Agree:* agent-facing design is the session's (AGENTS:85, README ruling of 2026-10-26, TEMPLATE Do, A step 5).
- *Agree:* `--docs` (AGENTS "the five that build no Rust", gate.sh header, session-close).
- *Differ:* commit and push permission (AGENTS:83, TEMPLATE Rules, the 2026-11-01 grant).
- *Differ:* the reviewer: Codex (Roles) against a sub-agent (2026-11-09, a one-off).

**Rules I could not classify:**
- The test-shape group's review rule (README:97–103, "binds the test-shape group (#137 to #143)").
- "The group's pace" (README:117).
- The stuck-verdict grant (README:300ff).
- "The machine is his during a league sitting": is one on now?
- "One step at a time in the daemon" (README:296).
- The night run's "each `trigger:*` waived" (README:841).

**Reading bill**

| Scenario | Auto-loaded | Mandatory by rule | Needed to resolve ambiguity |
| --- | --- | --- | --- |
| 1 | 16.7 K | about 45.6 K (issue, frontends.md, checklist section, A step and Roles, TEMPLATE, session-close, REVIEW-PROMPT) | about 33 K (README rulings list, gate.sh header, ledgers, git log) |
| 2 | 16.7 K | about 44 K (frontends.md, store.md, two checklist sections, the code doc) | about 12 K |
| 3 | 16.7 K | 59.3 K (the README) | about 8 K (design §7, LEDGER and RUN-LEDGER tails, issue list) |
| 4 | 16.7 K | about 37.5 K (bothy docstring, daemon.md) | about 2 K |

**Ten changes to the instructions, ranked**
1. Put the current step in the first ten lines of the roadmap README, and move the "Current step" narrative to LEDGER.md. This would have saved about 55 KB in scenario 3.
2. State whether "a commit or push waits for his word" covers every commit or only rulings, and make the 2026-11-01 grant standing or explicitly scoped (scenario 1, the commit and push question).
3. Add an "off-road issue" path to "A step": whether it needs a brief, whether it sets Current step, how the issue is closed (scenario 1).
4. Correct the reversed `artifacts` ruling in place, dated, as the README's own Starting-a-session rule asks (stale pointer 1).
5. Give the AGENTS quality-gate list the gate's real stage list, or point it at `gate.sh --help` (stale pointer 2).
6. Say whether C154 covers REVIEW-CHECKLIST rows (scenario 1).
7. Scope or expire the group-scoped rules (the test-shape review rule, the pace rule) with an end line (the unclassifiable list).
8. Give the exact golden-regeneration command once, in AGENTS "Where things live" (scenarios 1 and 2).
9. State an id-minting rule ("next unused C<n>; `bothy list` shows it") in the bothy docstring (scenario 4).
10. Add packet, cut, cold review, Do N, orchestrating session and spike to GLOSSARY.md (the unresolved nouns), and say what `trigger:step` means when the road is not keyed by area (scenario 3).

**Three things to keep exactly as they are**
1. `bothy.py pack --area`: one command assembles exactly what the rules say to read, verbatim.
2. README "A step" as the single statement of the rhythm, with the template and session-close pointing to it rather than restating it. That is why scenario 1's step sequence came out the same from every document.
3. REVIEW-PROMPT's numbered shapes and the bothy entry rule: concrete, checkable, and they answered scenarios 2 and 4 directly.
