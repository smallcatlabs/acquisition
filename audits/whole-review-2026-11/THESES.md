# The ten theses, as put to the skeptic

The orchestrator's theses after nine investigators reported
(`reports/investigators/`) and three trials ran (`trials/`), written
before any dissent. Each is stated strongly enough to be wrong. The
skeptic's answers are `dissent/`; what survived is `PORTFOLIO.md`.

Evidence is by path. **V** means the orchestrator ran the command or
read the lines; everything else is the named investigator's.

## 1. One document, doors that spell

Every verb answers one typed document that carries its data, its
sentences and its ways on as calls (`Doc { body, says, ways }`, a way
on being `Call { verb, args }`), made in the crate that makes the
document. A door lays out and spells a call for its reader (argv, a
tool call, a URL) and words nothing.

- Today the JSON answer carries no sentence; the CLI composes them in
  116 functions, 80 in `search_cmd.rs` (**V**: `git grep -hoE 'fn
  [a-z_]+_(words|text|sentence|says)[(<]' -- crates/acquisition-cli/src`),
  the web borrows 86 of the CLI's symbols (**V**), and the MCP words
  the same rules again in `describe.rs` (**V**: `search_cmd.rs:2146`
  beside `describe.rs:291`).
- `acq price status --league Nowhere` prints a `next:` remedy; its
  `--json` has none (**V**), so an MCP agent never gets it. About 150
  hand-spelled `acq …` remedies are in no `Way` (`reports/investigators/words.md`).
- `Way` has 20 variants (**V**), each spelled in `terminal.rs`,
  `told.rs` and for 10 of them `web/`.
- Goes: the web's imports from `*_cmd`, the three `Way` spellers, the
  two `fault_words`, the second wording in `describe`, quoting for four
  shells inside the search library (`answer.rs:3938-4222`). Net about
  1 to 1.5k source lines; most code moves.
- Follows: an MCP tool is a verb path from one table (30 tools to
  about 14, a 42.5 KB first turn to about 15 KB,
  `reports/investigators/first-answer.md`); the frontends' tests become
  one request corpus run through three doors and compared as documents;
  the 795 KB of web goldens (**V**) shrink to structure; a second GUI
  can link the words.

## 2. The daily GUI is in-place updates over the documents `acq web` already serves

C191 (the owner's ruling of 2026-11-19) keeps `acq web` a development
surface and reserves a separate high-performance GUI. C16 already rules
that any GUI is web rendering served over loopback. So the separate GUI
differs from `acq web` only in how the client behaves, and the server
is already fast enough.

- In a real browser on the owner's 44,090 items, fetching the page for
  a changed query and replacing the answer region took 33 to 167 ms to
  the next paint over nine queries; the DOM work was 3 to 4 ms; sorting
  all rows by a value took 117 ms (**V**, `trials/gui-in-place.md`).
  The C++ app refilters after a 350 ms timer.
- What is slow is that each act is a navigation: a 303 and an 87 to
  123 KB page, focus and scroll lost. One item's page takes 187 ms
  (**V**) because `show` scans the store.
- Thesis: strike C191; `acq web`'s in-page acts (open a row, sort, pick
  a value, filter as you type) become swaps of a region fetched from
  the same URL, with `history.replaceState` keeping the URL the whole
  state (C16 kept). The 303 landing machinery for in-page acts goes
  (`toggle_item`, `at`, `cut`, `picked`). Parts 2 and 3 design the live
  loop for that.
- C16's lexical scan refuses any script containing the word `history`
  (**V**: `web/mod.rs:231-233`), which forbids the one API that keeps
  URL-as-state honest under updates.

## 3. Scope is everything held, shown whole

A read with no scope named reads every realm and league the account
holds and says so; nothing reads a fixed name, and nothing refuses for
want of a realm. One document, `world`, says what is held and the exact
words that name each part.

- Scope resolves three ways today: `search` refuses when two realms are
  held; `tabs`, `price`, `refresh`, the web and the MCP silently read
  `Realm::DEFAULT` (pc) and `DEFAULT_LEAGUE` ("Standard"), 36 lines in
  10 files (**V**); the account refuses when more than one is known.
- `search --realm all 'pseudo.total_life>=90'` already answers 586
  matches and marks the 98 PoE2 items undecided (**V**).
- A player in a challenge league alone gets an empty Standard from
  `acq price`.
- `world` for the owner's store is 590 bytes as text, 973 as JSON
  (**V**, `trials/world.md`). It replaces `accounts`, `store status`,
  `leagues --stored` and the head of `tabs` as the first call.
- The account refusal stays only where two accounts hold items, and
  carries each one's item count.
- About a quarter of P1 and P2 review findings in every period name
  account, realm or league (`reports/history-hindsight.md`).

## 4. Two speeds: prove the law, sit the shell

One method (brief, build, cold diff review, fix rounds, close) is
applied alike to the limiter and to a page of HTML. The code that holds
the five invariants (transport, the daemon's `ratelimit/`, `gate.rs`,
`rails.rs`, `auth.rs`, the store's ingest and intent API, the hooks)
should get more than review; everything else should get less.

- Law gets proof: a bounded exhaustive explorer over the gate and the
  limiter as they are today (`trials/admission-explorer.md`), the
  decision made one function with an exhaustive match, the gate's
  duplicated `active_policies` set deleted (#206), and the weekly
  mutation run's logs read as a test-by-mutant matrix.
- The shell gets a seat: an agent-facing step is checked by a blind
  agent doing fixed tasks in a world shaped like the owner's (three
  accounts, two realms), counted in calls and bytes, in place of brief
  reviews and fix-round diff reviews. The MCP's refresh loop has stayed
  at 18 to 21 calls through every fix round; this sitting measured 23
  against the CLI's 5.
- Brief reviews: 123 reviews, 659 KB, 85% of the build reviews' bytes.
  First-round build reviews find a P1 or P2 56% of the time; of 40
  findings sampled, 19 were behaviour a user or GGG would notice
  (`reports/investigators/process.md`).
- Improvements that are not bugs do not land: #196, #206, #210, #212,
  #213, #214 have been open under `trigger:step` since 2026-11-09
  (**V**), because "Next" holds only features. One slot in "Next" is
  always a removal.

## 5. Number only law

Of 200 rulings, about 53 are law by C170's own classes (an invariant, a
boundary, his store, the live API). The other 147 are agent-facing
design, mechanism, process and restated checks, about 98 of 133 KB.
They move to where they bind (help text and its golden, the module doc,
the two skills, the check's header) and lose their numbers.

- No ruling has ever been withdrawn. C192 already says mechanism lives
  at the code; nothing moved.
- Of the owner's 23 ruling issues (**V**: all closed), 12 were process
  or registry wording and none touched an invariant or the live API.
- The reading path before a first edit is 63 to 102 KB, and each pack
  is a subset of the area file the agent is also told to read whole.
- Goes with it: `bothy.py`'s shape lint, the registry clauses of
  `docs-check.sh`, about 4,200 citations of non-law ids.

## 6. The queue is memory

Jobs live in the daemon's memory. A ledger keeps terminal results, a
done fetch's result being a summary and the id of the response it
landed in. A plan is re-appliable and spends each request once (C111),
so re-running the apply sends exactly what did not land.

- The owner's `daemon.db` is 69.6 MB; 60 MB is the bodies of 3,498 done
  `stash` jobs, each also in the facts store (**V**). Retention prunes
  only at restore (**V**: `queue.rs:326`), so a daemon that never
  restarts keeps every body.
- Goes: `restore_jobs` and the fault marker (about 420 lines), most of
  `store/src/jobs.rs` (789), the jobs migration ladder, the tainted
  tree, held mid-fan-out parents, `Interrupted`, about 2,000 test
  lines; C6, C25, C45, C133, C134, C136 shrink.
- Invariant 3 gets stronger: today, after a block's reset, a successor
  daemon sends the old queue unprompted.
- Loses: resume of a long refresh across `daemon stop`, a replace or a
  crash; a person or agent re-runs the apply.

## 7. One value expression, before #316

A field, a `line(…).slot`, a `sum`, a named total and the trade site's
weighted sum are one thing: a linear combination of line sums, with one
flag for what nothing is worth, and a table that gives some of them
names. #316 and #46 are rows of it.

- A total already prints as `1 × arg1 of line("# to maximum Life") +
  0.5 × arg1 of line("# to Strength") …` (**V**); 45 table rows carry
  weight −1 (**V**).
- `--sort 'pseudo.total_life + pseudo.total_ele_res'` is a parse error
  (**V**), so "best life and resistance ring" cannot be asked.
- Collapses four value-result types, the totals table's own 250-line
  parser, `pseudo.rs` 485–855 and three of the five weighted refusals
  in `translate.rs`: 700 to 1,000 lines.
- Fields and totals become one namespace and the `pseudo.` prefix goes,
  so `life>=90` binds.

## 8. Stop sitting for exactness

The translation's ambiguity is measured by how many clauses are marked,
not by what an answer would lose. Of 467 sitting searches run against
the owner's store, 229 were exact, 217 ambiguous and 21 partial; of
1,236 ambiguous clauses, 1,114 carry no other reading and the other 122
each change the answer by nothing
(`reports/investigators/search-kernel.md`).

- The sittings are the owner's hours. Further rounds for ambiguity rows
  buy no user-visible change on his store.
- Instead: print the stake (C119) for the two commonest ambiguities,
  and call a clause ambiguous only where the stake is not zero.
- The site's own answers stay the project's best oracle; the oracle
  test stays.

## 9. The account is its uuid

Facts are filed by username, intent by uuid, the keyring entry and the
rails' dead-grant mark by username. A rename at GGG orphans the facts
file, and with it the history no refetch brings back (`first_seen`,
50,847 item events, removed items).

- Goes: `filename_safe` and its collisions, the reserved-stem escape,
  the keyring-name chase (26 references in `index.rs`, 25 in
  `refresh.rs`), the rename-follow.
- The store's own remedy for a facts file below its floor is to delete
  it, and the registry says facts are refetchable; the history is not.

## 10. One home for shared words

The store links the protocol crate and only it. Job kind and realm
become types there; the store reads intent through its one typed read.

- The daemon's own comment lists eleven places a new job kind is added
  by hand (**V**: `daemon/queue.rs`, above `KINDS`).
- The store's typed intent read `get_as` has no production caller
  (**V**); `check_value` is called above the store instead, and each
  caller keeps its own "unreadable" state: about 10 types and variants,
  129 match lines in 18 files.
- On the owner's store none of those states has occurred: 0 unreadable
  intent rows of 21, 0 withheld of 4,946, 4 refused bodies from one day
  (`reports/investigators/stored-row.md`).
- C1 changes by one clause; `arch-check.sh:488` drops one name.
