## Verdict: NARROWED

**The hypothesis holds for process, registry wording, agent-facing mechanics and sequencing. It breaks on what a human player sees, and asking does not catch over-built designs either.** The deciding number: **none of the 17 decisions taken under delegation were reversed** (#221's eleven entries, #235, #265, #278, #284, #295, plus the #225 items; `gh api .../issues/N/events` shows no reopen). Against that, **he reversed all 4 of #224's standing defaults on user-facing behaviour.** These were defaults that already ran as default-with-veto, and he vetoed every one.

## Evidence

**Volume** (`volume.py`). 1,429 typed owner messages over 40 active days and 168 sessions. That is a median of 34 a day and 5 a session, with a median length of 77 characters. By ISO week (W42 to W48): 303, 254, 358, 167, 126, 126, 95. Volume roughly halved after the governance rulings of 11-09/10. Whether C171/C179 caused that is my judgment, not shown.

**Where his attention goes** (`shares.py`, ordered keyword classes, checked against samples; approximate):

| Share | What it is |
|---|---|
| ≈35% | Go-aheads (14% the recommended button, 21% typed) |
| 24% | Questions he puts to the session |
| 16% | Other instructions or context |
| 9% | Steering (simplicity, redirection, correction) |
| 6% | Pasting reviews or prompts between sessions |
| 6% | Reporting what he ran or saw |
| 4% | Opening sessions |

Of his 340 questions, about 111 are about how to run the agents (new session or here? which model? series or parallel?). Another 45 ask for the session's own opinion, 32 ask for an explanation and 20 ask for status.

**Questions put to him** (`buttons.py`, `typed.py`, `askclass.py`, `askq_score.py`):

- **End-of-turn asks** (from 10-24, when the ask buttons appear): 445 asks. Per week (W43 to W48): 26, 167, 104, 46, 53, 49.
  - He took the recommended button 195 times and the second button 6 times. He typed a plain accept 91 times, so **64% were accepted as recommended**.
  - 77 replies (17%) asked a question back instead of choosing.
  - 76 replies were something else. By hand, about a third of those (~25) changed the substance.
  - Median time from ask to answer: 3.4 minutes (p90: 53 minutes).
  - By keyword, 124 asks were commit or push go-aheads and about 197 were start, continue or build go-aheads. About 72% of asks fall in none of C170's classes.
- **Push and commit asks after C171:** they went from 46 a week (W44) and 29 (W45) to 6, 2, then 0. No reversal followed. Delegating push lost nothing visible.
- **AskUserQuestion:** 24 calls holding 38 sub-questions (I recoded #224's multi-select by hand).
  - 27 of 36 answered (75%) took the recommendation.
  - 7 substantively changed it: item images moved from Out to In; a Stack column instead of the Item cell; two stat-wordings marked "unsupported until the sitting"; the weapon-crit row sent to a sitting; #224's four; #212 amended with two refusals; the park's trigger marked fired.
- **Ruling issues:** 23, all closed. Median time to answer is about 11 hours, with a split:
  - Single questions were answered in minutes to hours.
  - The batch lists (#221, #224, #225, #233, #235, #199) sat **11 to 14 days**.
  - The same items, put to him one at a time with a recommendation on 11-21, were all answered in about 25 minutes (04:02 to 04:28).

**What kind of thing his departures were.** About 27 cases across all sources, classed by my judgment:

- **About 12 are what a player sees:**
  - item images in
  - the Stack column
  - #224's daemon start with no login, the missing policy clear verb, and pages that print CLI remedies
  - the main account and `ACQ_ACCOUNT` removed
  - "open" as the default paging behaviour
  - acq web's direction (C191)
  - "also support the equivalent stat"
  - the export of trade-site URLs
  - the GUI pulled ahead of 3.30

  **C170 gives this class to nobody.** Its owner classes are invariant, boundary, registry words, gate stages, and his real store or the live API. Its session class is *agent*-facing design.
- **About 6 are process or registry wording.** Examples: cut C172's why-sentence; "at least three" reviews; his own C137 text; #240 not approved. These are small and cheap to reverse later.
- **About 4 are design-shape simplifications:** #246, C188 shrunk, no enforcement check, and #264 ("build, observe, iterate instead of front loading the design work"). The money case is **#246**. He *approved* the main account as recommended. It was built, went through three cold reviews, then he reversed it: "Let's step back for a second." Asking did not protect it, and neither would a veto. What caught it was his "is there something simpler?" after the step stopped.
- **2 went to evidence instead of a guess** (sittings), and **1 was a boundary** (#212's refusals).

What default-with-veto would have lost:
- **Player-facing choices:** the four #224 defaults stood for 11 days unread. A veto only works if he *meets* the thing.
- **Shape choices:** nothing extra, because asking didn't catch them either.
- **Process and wording:** nothing.

**Five recurring steering themes** (`themes.py`, keyword proxies over 1,333 non-pasted messages):

1. **How to run the agents (154):** "Would you advise a new session for the next rounds, or continuing here as we have?"
2. **The session's own opinion (105):** "what's your recomendation?"
3. **Simplicity (93), spread evenly across weeks** (16, 17, 22, 11, 7, 13, 7): "Are we addressing the findings by finding simpler invariants and design choices that eliminate errors, or by patching edge cases?"
4. **Step back, stop, redirect (71):** "Let's step back for a second."
5. **Plain words (40):** "can you explain the rulings i need to make in nontechnical terms?"

Corrections that came back after a rule was written to prevent them:
- **Simplicity** still recurs after the memory note of 10-18, C173's "one rule" and the 11-11 memory. C188 (11-17) and "The simpler rule" (11-21) came after all three.
- **Worktrees:** the memory says none unless two writers build at once. He still said "do not use a worktree for it" and asked "does the way i work need worktrees?" (11-23).
- **Autonomy grants** (20) continue after C170: "I trust your judgement", "you are in charge".

**What he asks for that the method doesn't give:**
- **Status.** He asked "where are we" or "what's left" 16 times despite the road's "Now" section, and once "Where can I find the things you need from me for each item?"
- **Plain explanations**, and disclaimers that he can't judge: "I'm not a database expert", "the documentation is too technical and complex for me to understand well enough to make a good decision".
- **Fewer, judged questions.** He keeps turning a binary ask back into "what's your objective assessment".

**What he reads.** Mostly the session's last message. He refers to issues in 57 messages and the roadmap in 63, but many of those are relayed prompts. "I've read the review" appears 12 times, all in October. The unread batch lists show issues are read only when queued to him one at a time.

**The four step-backs** (`stepbacks.txt`). It is the same complaint each time, accumulated mass:
- 10-20: "documentation is a scattered mess … testing … probably needlessly complex"
- 10-26: "carrying a lot of inertia and mass"
- 11-10: "I suspect it has accumulated some cruft"
- 11-25: "question if what we thought we needed when we started is not what we actually needed"

Each time he names the agent the primary stakeholder and hands it the judgment. On 11-26 he asks outright whether the session considered "changing how I decide things for you".

## What nobody named

1. **His vetoes concentrate in a class C170 doesn't list:** what a human player sees. C170 reserves registry words and gate stages for him, where he never reversed a delegated decision. It leaves player-facing behaviour unowned, and he reversed 4 of 4 there.
2. **Each step-back against cruft produced more rules.** `decisions/process.md` went from nothing to 24 rulings, in 15 commits since 11-09. The complaint returned 15 days later.
3. **He is the transport between sessions.** He pastes reviews and prompts (91 messages), is asked to click session cards (22 messages mention cards), and gets "give me the prompt … to start it myself". That is the sessions' plumbing, routed through him.
4. **He accepts recommended options he cannot evaluate**, and says so. A high approval rate measures trust, not review. So "he approves most of what reaches him" does not mean asking was useless. It means the ask was the wrong instrument: his real contribution is a "simpler?" question nobody asks before reaching him.

## The one change

Stop asking go-aheads for sequencing, such as "Start step N's build?" or "Design step N+1?". On 11-23/24 there were about 35 of these in a row, all accepted. Instead, take one approval per series. He already granted exactly this when it was offered: "have the stat fix briefed and built without asking again? → Yes, go ahead". Spend the attention returned on three things:

- **Player-facing choices:** put them to him as a sitting at the running page, never as a list. His departures concentrate here, and it is the class he can actually judge.
- **His C170 classes:** one question at a time, with a recommendation and a plain-words sentence.
- **Before any design ask:** the session writes "the simplest shape I considered and why it isn't enough". This is his most repeated question, and the one that changed outcomes (#246, C188).

Default-with-veto is right for process, wording, agent-facing design and sequencing. It is wrong for player-facing behaviour unless the veto happens at a sitting, and for safety boundaries (#212).

## Limits

- Before 10-24 there were no ask buttons, so prose questions he answered there are poorly detected (59 found by heuristic).
- Some formulaic "typed" replies are probably app suggestions he accepted.
- The ~25 "changed" among the 76 "other" replies is a hand estimate.
- I didn't read subagent transcripts or worktree-named transcript directories.
- Themes are keyword proxies.

Re-read for the top three claims: #224's comment of 2026-11-21 together with `askq_dump.txt`; the reopen events on the delegated issues; the weekly push-ask counts against C171's text.

## Scripts

All in `<scratchpad>/reader-c/`, each run as `python3 -I <script>`:

- `extract.py` (writes `msgs.jsonl`)
- `askq.py` and `askq_score.py` (write `askq_dump.txt`)
- `buttons.py` (writes `turns.jsonl`)
- `typed.py` (writes `typed_other.txt`)
- `askclass.py`
- `volume.py`
- `themes.py` (writes `themes_samples.txt`)
- `reads.py`
- `shares.py`

Also there: `rulings.txt`, written by an inline `gh issue view` loop, and `stepbacks.txt`, written by an inline python command.
