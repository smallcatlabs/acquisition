# Notes to the owner, one per phase

Short and plain: what was learned that surprised, and where the review
is going. The detail is under `findings/`.

## Phase 0, ground truth (2026-11-24)

I read the charge, then sat at the tool with a copy of your store
before reading anyone's conclusions.

What surprised me:

- **It took me six tries to ask which items have 90 life.** Every
  refusal was correct and helpful. Nothing is broken. But nobody had
  counted the trips, and the plan I was handed points at the opposite
  problem (too many defaults).
- **Your job ledger is a second copy of your stash.** `daemon.db` is
  70 MB, 60 MB of it the bodies of finished fetches that the facts
  store already holds.
- **A search costs 0.9 s on your store, twice October's number,**
  because your store has doubled and each search rebuilds everything.
- **You have set 17 prices, once, in September.** The tool's real use
  is refresh and search. I say this only so the review weighs what is
  used.
- The answers themselves are very good. That part I would not touch.

Where it is going: nine Opus investigators are reading one track each
(the first answer, what persists, admission, where the words live, the
process, the GUI's direction, the tests, the search's kernel, the
stored row and the crate edges). Each was asked for what it would
remove and the smallest input that would break the removal, not for a
list. I hold the question of what the method would be if designed for
agents.
