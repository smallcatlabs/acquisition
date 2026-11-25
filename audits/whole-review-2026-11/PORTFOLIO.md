# The portfolio

Ten pages for the owner, ranked, then what died, then one page on what
I would do differently if this project began today. Written by the
orchestrator at the end of the review, 2026-11-25.

The owner said, while the review ran, that it could stay with finding
and understanding and leave solutions to a later session. So each page
leads with what is true and how it is known. The direction on each is a
few lines, not a design.

## How this was made

Nine Opus investigators each read one track whole
(`reports/investigators/`). I re-ran or re-read the claims each page
leans on; where I did not, the page says so. Three trials ran
(`trials/`), none landing anything in `crates/`. Then Codex, a model of
another family, was asked to break the ten theses as I first wrote them
(`THESES.md`), and its answers are whole in `dissent/`. I checked its
counter-claims against the code. They held.

| Thesis as first written | After the skeptic |
| --- | --- |
| One document, doors that spell | Narrowed (page 4) |
| The daily GUI is in-place updates; strike C191 | Narrowed to a question (page 3) |
| Scope is everything held | Narrowed to reads (page 2) |
| Two speeds: prove the law, sit the shell | **Died**; its trial stands alone (page 1), its measurements on page 10 |
| Number only law | Narrowed (page 8) |
| The queue is memory | **Died** |
| One value expression | Narrowed (page 6) |
| Stop sitting for exactness | **Died** |
| The account is its uuid | Narrowed (page 5) |
| One home for shared words | Narrowed (page 9) |

Three of ten died. I count that as the review working.

## The questions that are the owner's

Seven, each one decision. None is filed; they wait for his word.

1. After a counted send whose answer names another policy, does the
   limiter count the hit? An existing test pins that it does not
   (page 1).
2. May a read that names no realm or league read everything the account
   holds, and say so (page 2)?
3. Before parts 2 and 3 are designed, does he want a prototype of
   in-place updates on his own store to sit at (page 3)?
4. Does he want a design brief for documents that carry their words and
   their ways on (page 4)?
5. Is the facts' history kept state, or disposable with the words
   saying so (page 5)?
6. May a registry entry that states mechanism or a session's design
   move to where it binds when it is next amended, its id staying
   resolvable (page 8)?
7. May the store link the protocol crate, and only it (page 9)?

---

## 1. The gate and the limiter each hold a fault that a two-second search finds

**What is true.** The send gate keeps a set of the policies in flight
beside the permits themselves, and three `debug_assert!`s keep the two
in step. A search of every event sequence up to five events, against an
oracle that never reads the gate's bookkeeping, finds this in a release
build in 127 ms:

```
[Acquire(0, a), Acquire(1, a), Acquire(2, b), Rekey(a → b), Release(0)]
```

Two sends are then in flight on one policy. It is #206, open since
2026-11-09; the assertion that guards it is compiled out of the binary
that ships. A second search finds, in three events, that a counted send
whose answer names another policy leaves the limiter asking for no wait
while the server's window is full. That one stands only as a comment on
#66, and a test in `spec.rs` asserts the prediction is unchanged, so it
pins the missing hit as intended. An Opus agent wrote the searches; I
ran both again myself to depth 6 (`trials/admission-explorer.md`).

**Why it matters.** This is the code that holds invariant 1 and the
standing with GGG. It is also the best-tested code in the tree: 4,715
lines of spec tables, journal oracles, weekly mutants. None of that
asked this question, because every one of those tests is a scenario
someone thought of. Outside the two faults, the agent's run to depth 8,
103 million sequences, found nothing, which is the first evidence of
that kind this code has had.

**Not known.** Whether GGG can present either input. A route learning
that it shares a policy with another route is how same-name policies
work (N6), so the first is not far-fetched.

**Direction.** Derive "this policy is busy" from the live permits and
delete the set. Handle "counted and not reconciled" once, after the
match, so no arm can skip it. Keep the search as a test: about 330
lines and five seconds. Do not rewrite admission as one machine; the
investigator who read all of it argued that down, and I agree
(`findings/admission.md`).

**The dissent.** A bounded search is not a proof, and the trial's
module prints what it finds and passes. Both true. It also never drove
the choke point, the rails or a 429.

**Whose.** Fixing #206 is a session's. The limiter's behaviour on a
mismatch touches invariant 2 and an existing pin: question 1.

## 2. Scope is resolved three ways, and the first answer costs round trips

**What is true.** I asked the tool which of the owner's items have 90
life, knowing only `acq --help`. It took six tries
(`findings/ground-truth.md`). An investigator did five everyday tasks
cold: 11 calls, 2 scope refusals, about 51 KB of help read.

The cause is not the help's size. The account refuses when more than
one is known. The realm refuses in `search` and is silently pc
everywhere else. The league is every league in `search` and silently
"Standard" elsewhere: 36 lines in 10 files. The lesson of the main
account was applied in one verb as a refusal and not at all in the
others. A player in a challenge league alone gets an empty Standard
from `acq price`.

A refresh to "nothing to do" took 5 calls by CLI and 23 by MCP in a
mock world, because the MCP's caller carries the plan and polls. That
count has sat at 18 to 21 through three rounds of fixes to the words.

**Why it matters.** An agent's ease is one of the three things not on
the table, and it has been measured by seats in worlds of one account
and one realm. The owner's world has three accounts and two realms.

**Direction.** A read that names no realm or league reads everything
held and says so, as `search --realm all` already does honestly. One
document says what a world holds and the words that name each part:
590 bytes for the owner's store (`trials/world.md`). The MCP's refresh
takes the CLI's shape.

**The dissent, and what it changed.** Choosing an account by which one
holds items is unsafe for a write: B has just logged in and holds
nothing, and a policy write meant for B lands on A. So the account
refusal stays on every verb, and gains the count beside each name. The
`world` trial left out never-fetched places and leagues the policy
names, which are what a first refresh needs.

**Whose.** The default for reads reverses C96's default, not its rule:
question 2. The rest is agent-facing design, a session's (C170).

## 3. The GUI: the server is fast, each act is a page load, and the road designs a feed for a surface ruled temporary

**What is true.** On 2026-11-19 the owner ruled, in his words, that
`acq web` "was never intended to be a high performance desktop app, and
it would be wrong to warp it into such" (C191). His standing direction
is that the C++ app's layout and speed are the reference. The road's
next GUI items, parts 2 and 3, design a live subscriber and a resident
for `acq web`. And C16 already rules that any GUI is web rendering over
loopback, so the separate GUI that C191 reserves would be another
client on the same kind of server.

On his 44,090 items the server answers a filter, a window or a sort of
every row in 30 to 120 ms. What a person meets is that opening a row,
sorting or picking a value is a redirect and a page of 87 to 150 KB,
with focus lost. The C++ app refilters in place after a 350 ms timer.

In a real browser on his store, replacing the answer region in place
took 33 to 167 ms to the next paint, 3 to 4 ms of it the browser's
(`trials/gui-in-place.md`). One item took 187 ms.

**Why it matters.** `acq web` is 12k lines, the fastest-growing and
most-reviewed part of the tree, and the README still says there is no
UI. Whatever the daily GUI is, nothing today says where it comes from.

**The dissent, and what it changed.** I first proposed striking C191.
The trial does not carry that: it ran while another agent compiled, no
person typed, and nothing refreshed underneath it. Replacing the
history entry on a new search would break Back. `cut` is not landing
machinery and stays. The page's basis sits in the head, outside the
region swapped. So this page is a question, not a proposal.

**Whose.** His: question 3. It is worth answering before parts 2 and 3
are designed.

## 4. The words are composed above the documents, twice

**What is true.** A search answer's JSON carries no sentence. The CLI
composes what a reader meets in 116 functions, 80 of them in one file;
the web page calls 86 of the CLI's symbols to say the same things; the
MCP gets none and words the same rules again in `describe`. `Way` has
20 variants, each spelled in two files and ten of them in three. About
150 remedies are written by hand outside any `Way`:
`acq price status --league Nowhere` prints what to do next, and its
`--json` does not, so an agent on the MCP never learns it.

Of 30 wording differences between the doors read by the investigator,
5 must differ, 10 are spelling, and 15 differ for no reason.

**Why it matters.** The web UI lives in the CLI crate because the words
do. A second GUI today rewrites 78 sentences or becomes another verb of
`acq`. And the frontends' tests hold the wording at each door (page 7).

**Direction.** A document carries its sentences and its ways on, made
where the document is made; a way on is a call; a door lays out and
spells. Most of the code moves; about 1 to 1.5k lines go.

**The dissent, and what it changed.** Refusals and applies already
carry sentences in their JSON. Some wording must differ by door.
Comparing documents cannot hold exit codes, escaping or an HTTP status.
And regrouping the MCP's thirty tools must lose no function:
`store_refused` cannot go to the CLI alone.

**Whose.** The shape of a document is the core API, his; it would come
as a design brief (C173): question 4.

## 5. A rename orphans the facts, and their history does not come back

**What is true.** Facts are filed by username; intent by uuid; the
keyring entry and the rails' dead-grant mark by username. On a rename
at GGG, `record_login` leaves the old facts file behind
(`index.rs:300`).

The registry's why (C35) and the remedy printed for a facts file below
its floor both treat facts as refetchable. A refetch brings back
current state only. `first_seen`, the item events (50,847 on his
store), the removed items and `seen_response` are products of the order
things were ingested in. The stored response has its item arrays
removed; `Store::rebuild` re-extracts nine item columns and nothing
else. The advisor's idea of re-deriving facts instead of migrating them
does not survive this.

**Why it matters.** It is his store, and a user's. The only thing the
tool tells a person to delete is the one file whose history cannot be
had again.

**Direction.** The uuid as the account's established identity.

**The dissent, and what it changed.** The list of what could go was too
wide. Logout needs the old keyring names until their cleanup has
succeeded; the first profile job runs before any uuid is known; an old
index entry can lack one; `filename_safe` also serves the intent files.
Those transition states stay until each is retired.

An inference not yet tested: a job queued under an old username may
escape a plan's spent check and be sent again
(`issues/a-rename-may-double-send.md`).

**Whose.** What the history is worth is his: question 5.

## 6. Four kinds of value where there is one arithmetic

**What is true.** The road's next search item is #316, a weighted sum
of totals, to be designed with #46. A total already prints as a
weighted sum of lines; 45 rows of the totals table carry weight −1,
which is #46's request shipped inside the table; three of the reasons
the translation refuses a site's weighted sum exist only because the
language has no weights. The code holds four result types for a value
and a 250-line parser for the table beside the language's own. A player
cannot ask for the best ring by life and resistance together:
`--sort 'pseudo.total_life + pseudo.total_ele_res'` is a parse error.

**Why it matters.** #316 built as it stands is a third special case.

**Direction.** Linear arithmetic over the values the language already
has, the table giving names to some of them.

**The dissent, and what it changed.** One occurrence, the largest and
the sum are three things on an item with two life lines, and C92 keeps
them apart; the count and least readings are not linear. The arithmetic
sits over those, and changes none. The `pseudo.` prefix cannot be
dropped: saved searches hold query text. A player's names can be added.

**Whose.** The road already puts this design to a session. No question.

## 7. The frontends' tests hold markup and spelling, and the estate shrinks by shape, not by deletion

**What is true.** An investigator read sixty tests whole. Forty-five
were load-bearing; every one could fail on the bug it is named for. So
the estate cannot be halved by retiring tests, and 407 retirement lines
have deleted one file.

The size is in the shape: 34 hand-built seed worlds, and `web.rs` at
160 lines a test. Goldens are 1.05 MB in 74 files, 795 KB of it web
pages whose named facts are raw markup; one 64 KB golden names no fact
at all. Sixteen of twenty web tests sampled would fail on a harmless
rewording. Over the last 300 commits, 0.9 lines of test and golden
changed for each line of source.

The strongest tests share one form, a table of inputs against an oracle
written from a ruling or a ground-truth claim. That is 5 to 7% of the
estate, nearly all in the daemon and the search.

**Why it matters.** The cost is not CI time; the test stage's median is
39 s. It is that any change to a word is three edits and a regenerated
megabyte.

**Direction.** The road's own t6 (one world, the door's corpus) is the
right next move, and page 4 is what would let it pay.

**Not checked.** The sixty classifications are the investigator's
judgment. The search, store, plan and verbs tests were not sampled.

**Whose.** A session's.

## 8. The registry is mostly not the owner's law, and his rulings went to its words

**What is true.** Two hundred entries, none ever withdrawn. Read by
C170's own classes, one investigator tags about 53 as law and the rest
as a session's design, mechanism, process or a restated check. C192
already rules that mechanism lives at the code; nothing has moved.

Of his 23 ruling issues, the investigator reads 12 as process or
registry wording, 8 as frontend matters and none as touching an
invariant or the live API. Before a first edit an agent is told to read
63 to 102 KB, 40 KB of it the same for every task, and the pack it is
told to run is a subset of the area file it is told to read whole.

**Why it matters.** His attention is the scarcest thing here, and by
this count about half of what reached him as a ruling was the wording
of the method, not the direction of the product.

**The dissent, and what it changed.** The 53-to-147 split is a judgment
of headlines and left out two of C170's classes. Saved reviews cite the
ids and are kept verbatim, so an id must stay resolvable. A promise
that binds every door needs a home with authority over all of them. So:
no renumbering, no purge.

**Whose.** His: question 6.

## 9. A typed read nobody calls, and eleven places per job kind

**What is true.** The store has a typed read of intent. Nothing in
production calls it. The planner, the verbs and the CLI parse the row
again above the store and each keeps its own "unreadable" state: about
ten types and variants. One condition gets two faults: a policy row
that is not JSON and one a newer build wrote are refused differently.

The daemon's own comment lists the places a new job kind is added by
hand; with the table, eleven. They are strings because the store links
no workspace crate and so cannot share the protocol's types.

On the owner's store none of the unreadable states has occurred.

Four dependency edges keep invariants 1, 3 and 5 structural. The
fourth, that only the daemon links the keyring, is checked by nothing.

**Direction.** A typed read that keeps the row's address and revision
when the value cannot be read. The store linking the protocol crate.

**The dissent, and what it changed.** The existing typed read drops the
row's metadata and cannot simply be substituted. An unknown realm in a
stored row must stay observable. Rarity in one store does not make the
recovery expendable.

**Whose.** The edge is C1's: question 7.

## 10. What the method cannot see: a premise, and a removal

**What is true.** Product commits per week went 277, 139, 66, 74, 45
from late October; commits that only save a review or edit the road
have matched or passed them since. There have been 460 cold reviews
since 2026-10-21, 123 of them of briefs. A first-round build review
finds a P1 or P2 56% of the time, and half of a sample of findings were
behaviour a user or GGG would notice. Review works.

What it cannot do is ask whether a concept should exist. The main
account drew five reviews and seventeen findings before it was removed;
the closed-for-good route drew six. Each made the concept heavier. Both
removals came from someone looking at a table.

Five improvements from the last audit that are not bugs have been open
since 2026-11-09 with no place on the road. This review found #206 in
127 ms; it had waited sixteen days as one of them.

**What died here.** I proposed two methods: cold diff review and proof
for the code that holds an invariant, and a blind agent's sitting for
the rest. The skeptic broke it with one case. Exact authorization, the
choice of account and the listener's security live in the frontends; a
brief review caught that a tab id held in two leagues could authorize
the wrong one, and no sitting would have been built to hold that. The
line cannot be drawn by crate. I do not have the rule that draws it.

**Whose.** Nobody's yet. It is the question I would bring to the later
session.

---

## What died, and what it saved

- **Two speeds by crate** (page 10).
- **The queue is memory.** C134's promise that a body which breaks the
  daemon is sent once rests on a marker that outlives the process. What
  stands is a defect: the ledger prunes only at a daemon's start
  (`issues/ledger-prunes-only-at-restore.md`).
- **Stop sitting for exactness.** A zero stake on one store is not an
  exact translation; the warning is for the inventory that would show
  the difference. What stands is a fact about proportion: in a sample
  of 467 of his sitting searches, on his store, no answer differed. I
  did not re-run that tally.
- **Facts re-derived instead of migrated** (the advisor's). Page 5.
- **Admission rewritten as one machine** (two scouts'). Page 1.
- **The limiter's prediction persisted** (a scout's). The first probe
  of a lifetime reads GGG's own counters.
- **A read-only SQL view for agents** (mine). It would re-decide the
  three-valued logic in every query. The agent's cost is vocabulary.
- **A `take` verb for any offer** (the advisor's). Unneeded if a way on
  is a call.

## If this project began today

I would start from use, as a number. Before the first verb, a world
shaped like a real player's (several accounts, two games, a challenge
league) and ten things a player wants, counted in calls and bytes. This
review's first hour found that the plainest question takes six tries.
Five months of careful work had not, because the question was never
asked of a world like the owner's.

I would search the small machines from the first day. The gate, the
limiter, a job's states, a session's: each is small enough to
enumerate, and an agent writes the search in an afternoon. A scenario
holds what someone thought of. The two faults on page 1 were each in
plain sight of 4,715 lines of excellent tests.

I would let documents carry their words, and generate the doors. Then a
third frontend is a speller, and the tests are one corpus through every
door.

I would decide what is irreplaceable before writing the first file.
Identity by uuid. History named as kept or as disposable. The words
"facts are refetchable" were true of the first design and stopped being
true quietly.

I would number what the owner rules and keep the rest where it binds.
An agent forgets between sessions, and the registry was the honest
answer to that. But a sentence at the code it governs is met exactly
when it is needed, and a type is read by the compiler on the agent's
behalf.

Where a design is open I would build it twice, by two agents who do not
see each other's work, and compare. Rewriting is cheap now. A review of
a diff can only add; two builds side by side show the premise. I have
not tried this, and it may not pay.

And I would bring the owner fewer, larger questions, each with
something to sit at.

Where the old way was right, and I would keep it exactly: the
dependency edges that make three invariants a fact of the build; the
send journal with an oracle that is not the code; the register of
numbered claims about GGG; a gate whose exit status decides; a plan as
an envelope that applying can only shrink; refusals that say what to
do; and nothing remembering a choice the user did not name. The tool's
answers are the best thing in it.

What I do not know: whether any of this would have been faster. The
method that is here built this.
