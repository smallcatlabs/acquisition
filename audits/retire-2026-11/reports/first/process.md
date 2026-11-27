### C122 — A behaviour is pinned once, at
- class: check — how the suite is written: one harness, one binary per crate, goldens, nextest; promised to later sessions writing tests
- clauses:
  1. P · P · the second half (process tests share one harness) is held by tools/arch-check.sh:667-689, check 9, which refuses `CARGO_BIN_EXE_` or any `Command::new(` outside `tests/harness/` except `kill` and `ps`. The first half (pinned once, at the lowest crate) is not held: no check can tell when two tests pin the same behaviour
  2. P · U · a frontend's tests pin only what its adapter adds: a rule on test writers; looked in arch-check.sh checks 8 and 9 and docs-check.sh check 7; nothing reads what a frontend test asserts
  3. P · P · crates/acquisition-cli/tests/harness/mod.rs:1232 and :1733 refuse a transcript or page golden with no fact, and :590-597 assert each fact in the command's own output whatever the switch, so a blind regeneration fails there. Loose: cli_reference.rs:122, mcp_reference.rs:85 and counted_tasks.rs:1469 write their goldens under the switch with no fact list I could see, so a blind regeneration of those passes
  4. P · Y · tools/arch-check.sh:653-689, check 9: outside the harness, a test that spawns a workspace binary or a non-allowlisted command is refused. The harness applies the one list (crates/acquisition-client/tests/harness/mod.rs:11-22, :202-211). The daemon crate is excepted by name
  5. P · Y · tools/arch-check.sh:592-648, check 8: at most one plain root per crate's `tests/`; a file that sets the process environment may not be a module of it; `[[test]]` entries must match the roots exactly
  6. P · Y · tools/gate.sh:301 runs `cargo nextest run --workspace --all-targets`, and .config/nextest.toml sets `retries = 0` in both profiles; CI runs the `ci` profile (gate-linux.yml:100, gate-windows.yml:80). A config value is its own holder, the same footing as the retired C196 and C197
- unheld: 1 complex · para · "pinned once at the lowest crate" is a judgment over the whole suite; no seam lets a check recognise a second pin of one behaviour
- unheld: 3 simple · para · give cli_reference.rs's golden (crates/acquisition-cli/tests/cli_reference.rs:122) a fact list asserted before the write, as the CLI harness's `golden` does
- reason: Cite · tools/arch-check.sh:593 (one binary, so the dependencies link once: the 72-binaries half), :653 (the isolation list), .config/nextest.toml:1 and crates/acquisition-client/tests/harness/mod.rs:1. No comment states the wording half ("three edits in three crates") · moves to crates/acquisition-client/tests/harness/mod.rs:1
- fate: NT · complex
- confidence: medium · I did not read whether counted_tasks' rows carry facts of their own, nor the MCP harness

### C153 — A finding joins the surface that
- class: check — how findings become tests and goldens; promised to later sessions writing tests
- clauses:
  1. P · U · a rule on people; the session-close skill states it (.claude/skills/session-close/SKILL.md:40-42), which is no check. arch-check check 8 limits binaries, not new module files
  2. P · U · same; TESTING-NOTES.md:136-138 restates it; no check reads whether a new test file could have been a row
  3. P · U · one golden per wording: docs-check.sh:392-410, check 7, refuses only orphan goldens; nothing can tell two goldens holding one wording
  4. P · U · test comments follow it (crates/acquisition-mcp/tests/tool_refusals.rs:373, crates/acquisition-store/unit/tests/world.rs:70), but no check reads sentence literals against goldens
  5. P · U · the close message's content; no hook or check reads `Step:` commits (git grep "Step:" in .githooks, tools, .github: nothing)
- unheld: none
- reason: Prop · TESTING-NOTES.md:136-138 and :167-169 state the property, not the reason · moves to TESTING-NOTES.md:134 ("The suite's shape"), since no holder exists
- fate: KU
- confidence: high · I did not read every test that cites C153 past the cited line

### C154 — A test or a check is
- class: run — how a test or check is retired; promised to later sessions and reviewers reading git log
- clauses:
  1. P · U · a commit-message rule; no hook reads `retired X; held by Y` (git grep in .githooks, tools, .github finds only the registry's own retired rule in bothy, which is about registry entries, not tests)
  2. P · U · same; the session-close skill states it (.claude/skills/session-close/SKILL.md:38-40)
  3. P · U · a rule on workers; TESTING-NOTES.md:172 restates it
- unheld: none
- reason: Prop · .claude/skills/session-close/SKILL.md:38-40 states the property, not the reason (P6 had no procedure) · moves to .claude/skills/session-close/SKILL.md:38
- fate: KU
- confidence: high · nothing unread that bears

### C181 — A milestone's date is advisory, and
- class: run — how the schedule is weighed against engineering; promised to the owner and later sessions
- clauses:
  1. P · U · a rule on people; no file outside the registry cites C181
  2. P · U · same
- unheld: none
- reason: None · no citation outside decisions/process.md · moves to nowhere: there is no holder
- fate: KU
- confidence: high · none

### C185 — Builds run one at a time
- class: run — when builds and measurements may run on the machine; promised to the owner and to measurements
- clauses:
  1. P · U · tools/gate.sh has no lock (grepped lock, "one at a time", second gate). Cargo's own target-directory lock serialises single cargo commands, not a gate's run of them, and nothing keeps a measurement alone; AGENTS.md's hard rule restates part of it
  2. P · U · the owner's window is outside the tree
- unheld: none
- reason: None · no citation outside the registry · moves to nowhere
- fate: KU
- confidence: high · none

### C183 — A rule about how the project
- class: run — where process rules live; promised to the owner, reviewers and sessions on other machines
- clauses:
  1. P · U · private memory is outside the tree; no check can read it
  2. P · U · same
  3. P · U · same
- unheld: none
- reason: None · cited only by decisions/process.md:42's "Not doing" line · moves to nowhere
- fate: KU
- confidence: high · none

### C174 — A brief carries rules and commands,
- class: run — what a brief holds; promised to builders and reviewers
- clauses:
  1. P · U · roadmap/briefs/TEMPLATE.md:3-4 states it; it is a template, not a check
  2. P · U · TEMPLATE.md's headings (Goal, Inputs, Decisions, Find first, Done when, Not in scope); nothing checks that a brief has them
  3. P · U · the builder's report is outside the tree
  4. P · U · a judgment on prose
  5. P · N · no check of brief size (grepped 8000, "8,000" and briefs in tools, .githooks, .github). A loop of `wc -c` over roadmap/briefs/*.md finds 88 of 162 briefs over 8,000 bytes
- unheld: 5 complex · para · a cap needs a baseline for the 88 briefs already over it, and the clause reads as a diagnosis ("is carrying the tree or is two steps"), not a cap
- reason: Prop · roadmap/briefs/TEMPLATE.md:3-10 states the property, not the reason (stale account of the tree) · moves to roadmap/briefs/TEMPLATE.md:3
- fate: NT · complex
- confidence: high · I did not read any brief but the template

### C178 — A step's close is one commit
- class: run — what a step's close commit is; promised to later sessions reading the ledger
- clauses:
  1. P · U · no hook or check reads the `Step:` trailer (git grep "Step:" in .githooks, tools, .github: nothing); `git log --grep='^Step: '` finds 83 commits
  2. P · U · message content; the session-close skill §3 states it (.claude/skills/session-close/SKILL.md:59-71)
  3. P · U · "no file retells it": no check; roadmap/LEDGER.md stands, per a "Not doing" line
  4. P · U · no check ties a `Step:` commit to an edit of roadmap/README.md
- unheld: none
- reason: Prop · .claude/skills/session-close/SKILL.md:59-71 states the property, not the reason · moves to .claude/skills/session-close/SKILL.md:59
- fate: KU
- confidence: high · none

### C180 — `roadmap/README.md` says what is happening
- class: run — what the state file holds; promised to every session that reads it first
- clauses:
  1. P · P · the cap (tools/docs-check.sh:47-55) forces old text out, which loosely forces a rewrite. Nothing checks the Now, Next and Waiting-on-the-owner sections, or that a close rewrites the file
  2. P · P · the cap bounds how much history the file can hold; nothing refuses history under the cap (check 6 reads the file for document paths only, docs-check.sh:369)
  3. P · U · the order of "Next" and a session taking its first item are judgments on people
  4. P · Y · tools/docs-check.sh:47-55 fails when the file is over 6000 bytes; staged by tools/docs-check-breakers.sh:106-108, which CI runs (gate-linux.yml:215)
- unheld: 1 simple · para · a line in docs-check.sh check 1 asserting the three headings, with a case in docs-check-breakers.sh beside the cap's
- unheld: 2 complex · para · "no history" has no marker a check could read short of the registry's word list, which a state file's prose would trip
- reason: Prop · tools/docs-check.sh:44-46 and :7-10 state the property and that growth past the cap is history; they do not give the Why (a file only appended to buried its current sentence) · moves to tools/docs-check.sh:44
- fate: NT · complex
- confidence: high · I did not read roadmap/README.md's body

### C182 — A rule, a checklist row or
- class: run — how process rules are added and retired; promised to later sessions and the owner
- clauses:
  1. P · U · a rule on people adding lines; no check
  2. P · U · same
  3. P · U · same
  4. P · N · no check counts REVIEW-CHECKLIST.md rows: docs-check.sh reads it for ids and history (:156, :188, :259), and the breakers append to it (docs-check-breakers.sh:135, :145) without any row cap. REVIEW-CHECKLIST.md:7-9 states the rule
  5. P · U · no checker refuses an entry whose subject is a check; it is a judgment on future entries
  6. P · U · its first half restates C154; the phase-close read is a rule on people (.claude/skills/session-close/SKILL.md:93-96)
- unheld: 4 simple · para · a row-count cap in docs-check.sh, as check 2 caps the cross-cutting count, with a breaker in docs-check-breakers.sh beside the `checklist()` helper (:145)
- reason: Cite · .githooks/pre-push:6, REVIEW-CHECKLIST.md:9 and .claude/skills/session-close/SKILL.md:96 cite it in fragments · moves to REVIEW-CHECKLIST.md:7
- fate: NT · simple
- confidence: high · I did not read docs-check check 2's cap code itself

### C192 — A registry entry states a property,
- class: run — how a registry entry is written; promised to later sessions and the owner
- clauses:
  1. P · P · tools/bothy.py:40-47 (rule 6) and :548-573 cap an entry at 160 headline bytes and six clauses, which bounds how much mechanism fits. Nothing tells a property from a mechanism
  2. P · U · whether a mechanism sentence sits at the code under the id is a judgment; no check
  3. P · P · tools/bothy.py:548-573 refuses an entry that grows past its baseline row and a stale row (staged in tools/bothy-breakers.sh:120-129). Loose: an amendment that does not shed passes while it stays within its row, and "takes no new id" is not read
  4. P · U · a second property is a judgment
- unheld: 1 complex · para · telling a property from a mechanism is not mechanical
- unheld: 3 complex · para · enforcing "shed when next amended" needs the check to see that an amendment happened (history), which is a new seam for bothy
- reason: Cite · tools/bothy.py:46 cites it in a fragment of rule 6 · moves to tools/bothy.py:40
- fate: NT · complex
- confidence: medium · I did not read bothy's clause counter, only its baseline code and the breaker lines

## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| C122 | check | 6 | 0 | 3/2/0/1/0 | Cite | NT | 1/1/0 | 0/2 | medium |
| C153 | check | 5 | 0 | 0/0/0/5/0 | Prop | KU | 0/0/0 | 0/0 | high |
| C154 | run | 3 | 0 | 0/0/0/3/0 | Prop | KU | 0/0/0 | 0/0 | high |
| C181 | run | 2 | 0 | 0/0/0/2/0 | None | KU | 0/0/0 | 0/0 | high |
| C185 | run | 2 | 0 | 0/0/0/2/0 | None | KU | 0/0/0 | 0/0 | high |
| C183 | run | 3 | 0 | 0/0/0/3/0 | None | KU | 0/0/0 | 0/0 | high |
| C174 | run | 5 | 0 | 0/0/1/4/0 | Prop | NT | 0/1/0 | 0/1 | high |
| C178 | run | 4 | 0 | 0/0/0/4/0 | Prop | KU | 0/0/0 | 0/0 | high |
| C180 | run | 4 | 0 | 1/2/0/1/0 | Prop | NT | 1/1/0 | 0/2 | high |
| C182 | run | 6 | 0 | 0/0/1/5/0 | Cite | NT | 1/0/0 | 0/1 | high |
| C192 | run | 4 | 0 | 0/2/0/2/0 | Cite | NT | 0/2/0 | 0/2 | medium |

## What I did not check
- C122.3: whether counted_tasks.rs's task rows carry facts of their own; the MCP harness; whether any golden outside the CLI's carries wording a user meets beyond the two references.
- C122.4: I did not prove that a test in the plain binary (one that calls no `session(`) never reaches a child with an inherited environment, beyond check 9's spawn rule.
- C182.4: docs-check's cross-cutting cap code, which I name as the model for a row cap.
- C192: bothy's clause counter; I read only the baseline code and the breaker lines.
- Nothing was run: the checks' own breakers and the gate were not exercised.

## Noticed
- C178.4 and C180.1 against .claude/skills/session-close/SKILL.md:69-71: the entries say each close rewrites roadmap/README.md, while the skill says a step the road never named leaves it unchanged and the message says so (step 2: the text and its procedure disagree).
- C174.5: 88 of the 162 files in roadmap/briefs/ are over 8,000 bytes, so the clause reads as advice the tree mostly breaks (step 2/3).
- C122 *Pinned:* names tools/arch-check.sh, .config/nextest.toml and the client harness, which hold clauses 4-6. Clause 3's fact rule lives in crates/acquisition-cli/tests/harness/mod.rs:1232/:1733, which it does not name; docs-check.sh check 7 (orphan goldens, :392-410) also cites C122 and is not named (step 2).
- C122.6: "never retried" is held only by the value `retries = 0` in a config nothing checks. The brief's step 2 does not say whether a config value counts as a holder; I followed the precedent of the retired C196 and C197, held by gate.sh itself.
- C122.1 has one half held and one half unholdable. The brief's rule does not decide between P and U for such a clause, and the choice moves the fate between NT and KU (steps 2 and the fate).
- C182.6 restates C154 in its first half and adds a new promise in its second. For process entries the class brief says every clause is P, while its restatement rule would make that half M · R (step 1).
