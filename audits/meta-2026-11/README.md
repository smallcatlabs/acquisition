# The method, reviewed: what to change before the programme

Written by Fable on 2026-11-25, at `5b96edc0`, by a session that had
not written `audits/whole-review-2026-11/NEXT.md` and read it last.
The owner asked whether a change to how the work is run (the method,
the tools, the documents, how he decides, the technology) would do
more than the code-level programme that file plans. This is the
answer, the evidence under it, and three small changes to try.

The first reader is the owner. The second is the session that runs
the programme. Paths are from the repository's root. A number is mine
where it stands beside its command; otherwise it is a reader's, and
the reader's report is named.

## To the owner, in plain words

**The answer.** No change at this level beats starting the programme.
Start it. Three small adjustments are worth trying on its first
steps, and one thing found here should be fixed before any of it: no
backup of your annotations or your items' history can be made from
the tool today.

Three things surprised me.

**The slowdown everyone measured is mostly your hours.** Product
commits per week fell from 299 to 70, and the last two reviews read
that as the method getting heavier. For every hour an agent was
running, the project landed about as much in its slowest week as in
its fastest. What fell was the hours: 78 a week at the end of
October, 14 in the third week of November. Agents work when you are
here. So what limits the project is how many hours agents run, and
you have named what limits those: when you are present to click a
card, your usage limits, and a session's context.

**The method asks one question well and leaves three to you.** It is
good at "is what we built faulty?". "Is there something simpler?",
"what will a player see?" and "is this worth anything to someone?"
have come from you nearly every time: your sittings, your
step-backs, your "let's step back for a second". Meanwhile, of 445
times a session ended its turn by asking you something, 49 asked you
to rule on anything. Most of the rest asked whether to start, go on,
commit or push.

**No one outside has used the new tool and told us how it went.**
The request that started the rewrite came from outside, and the C++
app's users' reports were read once, in October. Since then every
signal has been made inside.

The skeptic's verdict on my first draft was that a small adjustment
is earned and a redesign of the method is not. I agree, and the
draft's larger claims are cut. What it broke is the section "After
the skeptic".

## What was found

Ten readers each took a hypothesis to break; their reports are in
`reports/`. I re-ran or re-read what this page leans on.

| Hypothesis | Verdict | The number, and what it does not show |
| --- | --- | --- |
| Process weight per step limits what lands | Narrowed | Product commits per hour an agent ran: 3.8, 3.6, 2.2, 5.0, 2.6 for weeks 44 to 48, over 78, 41, 30, 14 and 17.5 hours (`reports/where-time-goes.md`). The weekly fall is hours. Whether an hour carries too much ceremony was not split out |
| The reading path is the method's cost | Narrowed | As a bill: all 102 KB of governance gone from every call would cut about 6%; 68% is spent after a session passes 200k tokens (`reports/where-tokens-go.md`). Its cost in wrong turns was not measured |
| What reaches the owner he would approve anyway | Narrowed | None of 17 decisions taken under his delegation was reversed; all 4 standing defaults about what a user sees were. A yes he could not evaluate measures trust, not agreement (`reports/owner-interface.md`) |
| One review at the end would catch what matters | Narrowed | 33 of 113 serious build findings since 2026-11-10 came in a second or later round; 26 of the 27 that cite a place fall within 15 lines of what the fix changed (`reports/step-anatomy.md`) |
| The missing instrument is an outside user | Narrowed | No report of the Rust tool in use by anyone but the owner. The alpha's 23 downloads may include his own test machines (`reports/product-frame.md`) |
| Rust for the core is not worth questioning | Holds, for the language | Nothing found against it; its cost is build and test time. The daemon split, SQLite and the count of frontends were not examined (`reports/technology.md`) |
| The pages should not be hand-built strings | Narrowed | Two costs. Churn: the whole page is the unit of navigation and of test, and 76% of golden bytes are whole pages. Risk: escaping is one function called by convention about 300 times (`reports/technology.md`) |
| The project guards users it does not have | Narrowed | The cut was made on 2026-11-20 and under 2% of source serves a state another build wrote. A sixth of sampled serious findings are still about such states (`reports/absent-users.md`) |

Four things follow.

1. **Commits per week measured the owner's hours.**
   `scripts/time/weekly.py` gives the hours and `scripts/commits.py`
   the commits. Commits that only save a review or edit the road, a
   brief or a governance document have been 51% to 57% of all commits
   in every week since the end of October. Neither count says what
   was delivered, and a commit that touches a comment in a crate
   counts as product.
2. **A build's review rounds grow what they review.** In twelve
   sessions read whole, four concepts grew through their rounds and
   were then cut: the main account, the job with eight endings, the
   route closed for good, the form's tick boxes. Each cut came from
   outside the loop, most at the owner's word, and no builder
   proposed removing anything (`reports/sessions-read-whole.md`).
   The previous review named this of the build prompt
   (`audits/whole-review-2026-11/reports/design-theses.md`). The
   brief's review prompt does ask for a simpler shape; the build's
   does not.
3. **His reversals fall where the rules send no question.** What a
   person sees at the tool is the session's to decide (C170: a
   session decides everything that is not in his classes). It is
   also where he has reversed most: item images, the stack column,
   the main account, the four defaults of issue 224. Those four were
   put to him as a list and stood unread for eleven days. By their
   titles, 13 or 14 of his 23 ruling issues were instead about the
   registry's or the process's own text
   (`gh issue list --label trigger:ruling --state all`, my count).
4. **Step-backs have ended in rulings.** `decisions/process.md` was
   created on 2026-11-09 and holds 24. That commit also deleted more
   text than it added, so a count of rulings is not a count of
   burden. But this is the fifth step-back in two months, and it
   adds no ruling: its changes amend sentences that exist.

## Three changes

Each is small, a trial first, and the owner's to decide, because
each amends a process ruling. None makes the programme wait.

### 1. One yes for a series

**In plain words.** When you have said yes to a piece of work, the
agents run all of it. They stop asking "shall I start the next
step?". They post a line as each step closes, which needs no reply,
so you can still see it moving and step in.

- **What changes.** An approved series (a group of steps, a fix
  round) is run to its end on one approval. A session comes back
  before the end only for a decision in the owner's classes, or when
  a step stops. As each step closes it says, in a line or two of
  plain words, what a player or an agent can now do, what was added
  and what was removed. It asks nothing.
- **What goes.** The ask to start or continue: 197 of 445
  end-of-turn asks since 2026-10-24. The precedent is the grant to
  commit and push on green (C171): those asks went from 46 a week to
  none and nothing was reversed.
- **The number.** Asks to start or continue, per week.
  `python3 -I audits/meta-2026-11/scripts/owner/extract.py`, then
  `buttons.py` and `askclass.py` beside it. They class by keyword;
  read the share, not the digit.
- **The trial.** The programme's first three steps run as one series
  on one yes. Counted after: how often he was asked anything, how
  many of those asks were his to decide, and whether he had to ask
  what was happening.
- **What it loses.** Each ask was also a moment at which he could
  change his mind or catch something early. The line at each close
  is meant to keep that. A short trial cannot show whether unattended
  running stays reliable over weeks.
- **What it costs.** One sentence in C171. This harness has refused
  to let a session start another on a turn an automated event began,
  which is one cause of the asks; the trial finds out whether it
  still does.
- **Undone by** reverting the sentence.
- **His three reasons to care when a step starts** (his answer,
  2026-11-25), and what the series does about each. *A session's
  context:* the orchestrator decides whether a step reuses a session
  or starts a fresh one, and says which in its line. *His presence
  to click a card, which the app does not allow from a phone:* the
  series uses no card after its first; one worker is started at the
  yes and messaged for each step. *His usage limits:* the series
  stops cleanly at a limit and its last line says where it is.

### 2. A question arrives in plain words

**In plain words.** When something is yours to decide, it reaches
you as one question, in words about the app, the game or the player:
what happens under each answer, what the agent recommends, and the
simplest thing it considered. You should not have to translate it
before you can judge it.

- **What changes.** In the rule for how a question reaches him
  (C179), a question states in plain words the options that were
  really considered, what each one costs and buys in this case, and
  what the session recommends. Where the session found a way of
  seeing the problem that makes it smaller, it leads with that. One
  question at a time, never a list. He asked for this form on
  2026-11-25: a plain recommendation alone does not show him that
  the thinking was done, and a reframing that simplifies is the
  sign he looks for.
- **What goes.** The list of decisions: the six that were filed
  waited 11 to 14 days, and the same items put singly were all
  answered within 25 minutes. And the question in the agents' own
  terms, which he has said is where his effort goes.
- **The number.** There is no good one today. The nearest is how
  often he replies to an ask with a question of his own: 77 of 445,
  by a script that looks for a question mark
  (`scripts/owner/typed.py`), which also counts questions that are
  not confusion. His own word is the better evidence.
- **The trial.** It starts on this page. Four of the decisions
  `NEXT.md` hands him are rewritten below under this rule. If he can
  answer them without asking what they mean, the form works.
- **What it loses.** Precision, sometimes: a plain question can hide
  the case that makes it hard. So the question still names the test
  or the ruling it rests on, after the plain words, for the session
  that acts on his answer. And it can turn into a form: three
  options listed because the form has three slots. An option that
  was not really weighed is left out.
- **What it costs.** One sentence in C179.
- **Undone by** reverting the sentence.

### 3. The build's review asks what could go, and one round is tested

**In plain words.** Keep the outside review; it finds real faults.
Change one sentence so the reviewer also says what could be removed
instead of fixed. And find out, without risking anything, whether a
brief's second review is worth its cost.

- **What changes.** The build review prompt asks for "the smallest
  fix you believe sufficient" (`roadmap/briefs/REVIEW-PROMPT.md:69`).
  It would ask for the rule the fix must hold, or what could be
  removed instead. And when a second round on one step still finds
  faults, the session reads for what could be removed so the
  findings cannot occur, before a third round. That read is in one
  agent's private memory today and is applied after the fact; it
  moves into the cold-review skill, where a rule about the work
  belongs (C183).
- **The test of the second brief review.** For the next four briefs
  that get a second review, it is run as today and its answer left
  unread. The build goes ahead from the brief as the first review
  left it. When the build's own first review is in, the sealed
  answer is opened and each of its items is marked: already handled,
  or still standing. Standing items are fixed then. If most were
  handled, a brief gets one review from then on (one number in
  C176). If not, nothing changes.
- **What goes.** One sentence of the prompt and one private memory
  note now. Later, perhaps, a review round: second brief reviews
  raised something 14 times in 15 since 2026-11-10, as all 22 first
  reviews did, so the review's verdict says nothing and only its
  items do.
- **What stays.** The first review of a build, which found 71% of
  the serious faults. The review of each fix round. The first review
  of a brief, which caught a tab that could authorize another
  league's tab. A saved review in its own commit, before the commit
  that answers it: my draft proposed folding the two, and the
  skeptic is right that this removes commits and no work, and loses
  the record of a finding that went unanswered.
- **The number.** For the sealed test: items standing when opened,
  of items raised. For the removal question: lines removed within a
  week of being built, 2,888 in November for two concepts that took
  about 4,000 to build (`git show --shortstat fd7fc5fb 9ca22041`). The second cannot be shown by a short trial; it is
  read when the programme ends. The guard: serious findings per
  first build review must not rise
  (`audits/meta-2026-11/scripts/step/agg.py`, run after `reviews.py`).
- **What it loses.** On up to four steps, a fault the second brief
  review would have stopped is built first and fixed after. The
  removal read is judgment work that was not asked before.
- **Undone by** reverting two edits and stopping the test.

## A question for the owner, put the new way

**Who owns the wording of the agents' rulebook?** Today any change to
the words of a ruling waits for you. You have said that managing the
rulings is tedious and mostly by agents, for agents. Of 17
decisions agents took on your behalf and filed for your read, most
of them rewordings of entries, you reversed none.

- *If agents own the wording:* you rule on what a rule means, in
  your own words, and the session writes the entry. Wording changes
  stop reaching you.
- *If it stays as it is:* every rewording is yours to read.
- *Recommended:* agents own it. The five invariants and the
  boundaries stay yours whatever you answer.
- *What you would lose:* the one check that an entry says what you
  meant. A test can hold what the code does; nothing holds that the
  sentence matches your intent.
- *The simplest alternative:* change nothing and have agents explain
  each rewording in plain words. That keeps the reading with you.

This is one sentence of C170. I have not folded it into the three
changes, because it moves authority and the others do not.

**Answered, 2026-11-25:** "i agree agents should own their own
rulebook." C170 is amended in the commit that records this.

## One act that is the owner's

Use by a person with a purpose has redirected this project more than
any review: his refresh of 2026-11-18 found 90 refused requests in a
row, and his sitting of 2026-11-24 found 14 stops in a series the
method had closed. There is little of it, and none from outside.

- **One ask.** The request "Headless Support" on the C++ app's
  tracker (issue 187 there, open since 2026-07-31) is the outside
  request this rewrite answers. A reply was drafted in October
  (`audits/release-2026-10/cpp-issues.md`); what was posted, I did
  not establish. Its author could be asked to point an agent at
  `acq-mcp` and say whether the first useful answer came, and after
  how many calls. It needs no release and freezes nothing. Whether
  and how to ask is his alone. His answer: that person is busy with
  a new job, and might be drawn back when Path of Exile 2 launches
  next month.
- **His own use.** His answer: he does not use the tool much and is
  not a serious player, which makes him "a poor proxy for human
  users, but the best we have" until the tool is good enough to
  offer for beta testing.
- **What to count.** Not issues by author: a reply he relays would
  leave that at zero. Count reports of use from outside, and whether
  the task was done.
- **Not ten users.** A wider alpha now would cost more than it
  teaches: the surface is still being cut, and support would fall on
  one person.

## What it means for `NEXT.md`

The programme is sound. Start it; nothing here is a reason to wait.

1. **Start with A1; the second lane is that file's to use.** My
   draft said one lane, partly because I had misread the owner as
   not wanting more to land. He does. What still stands against a
   second lane is only that its trial compiled and ran no test, so
   its first use, for A3, is itself a trial: one build at a time
   until a lane's own gate has gone green once.
2. **Four of its seven decisions, in plain words.** For him to
   answer, or to send back as unclear:
   - *The order.* Shall the agents spend about a week first on two
     tools for checking their own work, a realistic test stash and a
     count of how many tries common player tasks take, before more
     GUI is built? The GUI's design can go on beside it.
     Recommended: yes.
   - *A league nobody named.* A player who plays only the current
     league runs `acq price` with no league and gets an empty
     answer, because the tool assumes Standard. Proposed: if items
     are held in one league, use it; if in several, refuse and list
     them. Not yet answered: a login that holds nothing. The simpler
     shape, always requiring the league, costs a single-league
     player a flag on every call.
   - *A renamed account.* If he renames his account at GGG, the tool
     starts a new file. The old file stays on disk, but the tool no
     longer reads it, so the history of his items is out of reach.
     Should the history follow the rename? Recommended: yes.
   - *A request GGG counts under another rule.* When GGG's answer
     names a different rate-limit rule from the one the tool
     expected, the tool does not count that request against the rule
     GGG named. Count it? Recommended: yes; the cost is sometimes
     waiting a little longer.
   - The other three stay as that file has them: a prototype to sit
     at, which waits on A1; filing its 33 drafted issues, which
     needs his word and his grant of writes to the worker; and how
     much of the rulebook an agent reads before editing, which is
     registry wording and so turns on the question above.
3. **A2's task list is shown to him.** It is a list of things a
   player wants. He can strike what no player would ask; he has said
   he cannot supply what users want, so it needs no approval and
   keeps the tasks that real use turned up.
4. **A backup first.** See the follow-ups. It is not a substitute
   for B6: a backup recovers the history, and only B6 keeps it in
   reach after a rename.
5. **B3 proceeds; B4 in two parts.** Moving the shared words out of
   the CLI is needed under every option the technology reader
   weighed. B4's first slice, a test that a page escapes hostile
   text, was found independently by that reader and should not wait.
   Its larger claim, that 795 KB of goldens fall to a tenth, rests
   on a projection that loses classes, titles, order and open state;
   check what it loses on one golden before the target is taken.

## The map

Everything considered. *Now* is a change, the question or the act
above. *Follow-up* is a fault to file, not a change of method.

| Area | Considered | Found | When |
| --- | --- | --- | --- |
| Deciding | Asks to start or continue | 197 of 445 asks | Now (1) |
| Deciding | The form of a question | He translates before he can judge | Now (2) |
| Deciding | Lists of decisions | Waited 11 to 14 days; singly, 25 minutes | Now (2) |
| Deciding | Who owns the registry's wording | His today; all 17 delegated decisions stood | Now (the question) |
| Deciding | Moving what a person sees into his classes | Would widen what he must approve; the skeptic stopped it | Never as an approval; shown at a sitting |
| Deciding | A standing view of the work's state | He asked "where are we" 16 times | Later, after change 1's line is tried |
| Deciding | The owner as transport between sessions | 6% of his messages paste a review or a prompt | Later; partly the harness's |
| Process | Process weight as the limit on output | Output follows hours; ceremony inside an hour unmeasured | Later; the hours come first |
| Process | First build review, fix-round reviews | Find the faults | Keep |
| Process | A brief's second review | Always raises something; worth unknown | Now (3), as a test |
| Process | "The smallest fix" and the removal read | Review grows what it reviews | Now (3) |
| Process | A commit to save each review | 26% of commits, little work, and a record worth keeping | Keep |
| Process | A cross-vendor reviewer | One preprint says a fresh same-family reviewer does as well on code | Later; not enough to change it |
| Process | The guards' own cost | 29% of sampled serious findings are about the method's own tools | Later; issue 196 holds the start |
| Process | Standing numbers in place of step-backs | Each audit re-derives its own | Later; the scripts here are a start |
| Documents | The reading path | 6% of the bill; other costs unmeasured | Later |
| Documents | Registry entries moved to the code | Already ruled (C192); nothing moved | Later; the previous review's question 6 |
| Documents | Byte caps | 21 of 29 briefs end within 500 bytes of the cap; trims in 10 of 12 sessions | Later |
| Documents | Orchestration knowledge in private memory | 47 KB, one agent's | Later, but for the one note in change 3 |
| Documents | The saved reviews | On no reading path | Keep |
| Tooling | Hours agents run with nobody present | 16% to 19% of active time; nothing ran in 58% of the last two weeks | Now (1) |
| Tooling | A second build lane | Cheap to make; untested under a real gate | The programme's own trial, on A3 |
| Tooling | Swarms | He cannot afford them | Never |
| Tooling | Cost of a session, against his usage limits | Starts at 77k tokens, three quarters of it the harness's tool descriptions; 68% of cost comes after 200k | Next after the trials; the series reports its cost for each step |
| Tooling | Mechanical turns on large models | 14.6% of cost | Later; the programme's bet on Sonnet tests it |
| Tooling | Agent definitions, hooks, scripted workflows | Exist; would hold what memory holds now | Later; nothing needs them yet |
| Tooling | Issue trackers built for agents | The tracker here is labelled, young and agrees with the road | Never |
| Tooling | The harness's own code review | Same family reviewing its own build | Never as the outside review |
| Technology | Rust for the core | Holds | Never |
| Technology | The pages' markup | The unit and the escaping are the costs | Later; a prototype to sit at |
| Technology | The daemon split, SQLite, three frontends | Not examined | Not checked |
| Technology | The Python tooling | Rightly Python | Follow-up, one check |
| Project | Issues, labels, the road | Work; "user" has come to mean an imagined user | Later |
| Product | A report of use from outside | None | Now (his act) |
| Product | A wider alpha | Costs more than it teaches today | Later |
| Product | Rigor for absent users | The cut is made; a sixth of findings remain | Later |
| Product | The irreplaceable data | No verb makes a backup; two messages advise deleting it | Follow-up, first |

## Follow-ups this turned up

Faults, not method. Three are filed, at the owner's word of
2026-11-26; the other two are held elsewhere.

- **No verb makes a backup** (issue 329). The registry promises a backup of the
  owner's annotations (C35). The functions exist and are tested
  nine times, and no verb reaches them:
  `git grep -n -E '\.export\(' -- 'crates/*/src'` finds no caller
  outside tests, and `CLI-REFERENCE.md` has no backup or export. The
  facts' history has no backup function at all. A fix is a verb, a
  restore, and a rehearsal of both.
- **Two messages tell a user to delete the history** (issue 330).
  They say facts
  are refetchable (`crates/acquisition-store/src/lib.rs:260`,
  `crates/acquisition-store/src/index.rs:213`). The current state
  is; the history is not.
- **Escaping on the pages is a convention.** No test feeds a hostile
  name through a page (`reports/technology.md`). The programme's B4
  holds the fix.
- **A skipped check prints `ok`** (issue 331). The check that generated tables
  match their generators is skipped where the game's export is not
  cloned, which is everywhere but this Mac, and says `ok`
  (`tools/trade-check.sh:74`).
- **`audits/whole-review-2026-11/next/traces.md` overstated its turn
  counts about 2.4 times.** It counted transcript events, not calls,
  and copied sessions twice (`reports/where-tokens-go.md`).
  Corrected by its own session in `c8a36c54`.

## After the skeptic

Codex was given the first draft and asked to break it; its answer is
whole in `dissent.md`. I checked its citations; they hold.

| The draft said | What it met | Now |
| --- | --- | --- |
| A change of method should come before the programme | Its own evidence points at use and visibility, not rules; a redesign is not earned | Start the programme; three small trials ride on it |
| Review only adds, and no one had named it | The brief's review prompt asks for a simpler shape, a brief review did propose a removal, and the previous review named the ratchet | Narrowed to the build's rounds and to builders not acting on it |
| What a person sees belongs to nobody, so make it his | C170 gives a session "everything else". Making it his would widen his approvals and slow the pages | Dropped. Shown at a sitting, as now |
| Take the registry's wording off his list, inside change 2 | It moves authority; the plain question does not. A test cannot hold that a sentence matches his intent | Split out as a question, with that loss stated |
| One review of a brief; fold each saved review into its answer | Second rounds found real things; folding commits is repackaging and loses a record; A1 against A2 compares two different tasks | The sealed test; saved reviews stay |
| Go-aheads are 35% of what he types | The class counts "fix", "cut" and "thanks" | 197 of 445 asks, by the asks' own class |
| 17% of asks drew a question back, so he could not understand them | The script looks for a question mark | Kept as the nearest number, called that |
| Seven decisions become four | A prototype is still a decision; one yes does not grant issue writes | Four rewritten, three left as they are |
| Outside issues by author is the number | A reply he relays leaves it at zero | Reports of use, and whether the task was done |
| No backup can be made | The functions exist; no verb exposes them | "No verb makes a backup" |
| The eight verdicts, as stated | Each leaned on one measure further than it reaches | Each now says what its number does not show |

One more of its points turned out to matter. It said low concurrency
does not show that parallel work has no value. I kept "one lane"
then, on my reading of the owner's answer; that reading was wrong,
and the next section says what replaced it.

## After the owner read it

His answers of 2026-11-25, and what each changed here.

- **I had misread one answer.** I asked what he would do "if agents
  landed twice as much per week", and he answered about output. He
  meant the words he has to read, not commits, features or fixes. My
  draft concluded that speed was not wanted and proposed nothing to
  serve it. That was wrong and is corrected above: speed matters,
  the limit on it is the hours agents run, and change 1 is the one
  that serves it.
- **Who it is for.** In his words, "the most important stakeholders
  are longstanding users of the c++ app who will feel abandoned if
  we can't reach feature and performance parity, as well as new
  players from Path of Exile 2", a player base he puts at four to
  ten times larger. Some of both will be "more AI forward and will
  hack acquisition with computer use or forks if we do not give them
  an elegant agentic interface." He notes the three pull against
  each other, and that each is a user of the product. His own use
  and a study of the method matter where they serve those users.
  This bears on the order of `NEXT.md` and of the road, which this
  page does not settle: parity for the first group is a GUI at the
  C++ app's speed, and nothing on the road builds that yet.
- **The three trials: "yes to all."** With the form of a question
  changed as change 2 now says.
- **The registry's wording: agents own it.** C170 is amended.
- **A watch he asked for.** When a decision is put to him he wants
  to see thinking that is about this case, and he watches for cruft
  and creep, "although that's slipped by both of us on the
  governance side with the rulings."

## What I did not check

- The published prompts the owner named. The page likely needs a
  sign-in; I did not open it.
- Whether the harness still refuses a session start on an event's
  turn. Change 1's trial finds out.
- What was posted on the C++ tracker's issue 187, and who the 23
  downloads were.
- How much of an active hour is ceremony. No reader split it.
- The reader of his messages classed them by keyword. I re-ran its
  scripts and read its samples; I did not re-class them.
- Sessions in the cloud and in the two guests are in no transcript
  here. Commits suggest little is missing: 2,531 of 2,550 since
  2026-10-12 have a local session beside them.
- The trade tooling and the sittings, beyond their size. The
  previous review tried to cut them and its skeptic stopped it.
- The outward reader's sources are a few first-hand accounts and
  several vendor pages, some read only in summary. On how a single
  owner keeps decisions without becoming the bottleneck it found
  nothing, and I have only this project's own record.
- This revision has had no second skeptic.

## How this was made

Seven questions went to the owner first. He answered five before
the draft and the rest after it; "After the owner read it" says
what the later answers corrected. Ten readers ran, nine Opus and one the harness's guide:
eight handed a hypothesis to break, two looking outward with no
detail of this project in any query. Their reports are whole in
`reports/`, saved from their transcripts by `scripts/save_report.py`
with the scratch directory's path shortened. Their scripts are in
`scripts/`: `time/` (where the hours go), `tokens/`, `owner/` (his
messages and the asks put to him), `step/` (reviews and steps) and
`sessions/` (twelve sessions read whole). All but `step/` read the
session transcripts on the owner's Mac; some print message text, and
all write working files beside themselves that `.gitignore` there
keeps out of the tree. `NEXT.md` was read after the picture was
formed. Codex then tried to break the draft.
