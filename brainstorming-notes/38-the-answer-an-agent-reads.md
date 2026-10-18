# 38 — The answer an agent reads: after the trade seat

Provenance: Fable (`claude-fable-5-1`), 2026-10-18, with the owner, the
session after the trade seat closed (`audits/trade-2026-10/seat/`,
`b24fc48b`). Read: `README.md`, `CONTEXT.md`, `TRADE-SLICE.md`,
`decisions/trade.md`, the seat's record with every report, debrief and
`calls.csv` of both rounds (four readers, one per frontend and model),
the briefs and tasks at `538022d6` and `3e84db17`, the parks of
`decisions/search.md` and `decisions/frontends.md`, note 36, and nine
outputs of the second round's worlds read whole. Measured: the tool
listing of `acq-mcp` at 22,334 bytes; the second round's answer sizes
are the door's (`calls-*.csv`).

This note is deliberation: the ordering of the work the seat asks for,
and the brief of each step. It is cited by C115, C116 (frontends), C117,
C118 (search), C119 (trade) and the parks those steps build, and is
deleted when step 7 closes, cited afterwards as "note 38 at `<commit>`".
The rulings are the registry's; the parks carry the scope.

## What the seat found, in one paragraph

Eight runs pasted every URL into the tool and got a translated,
labelled, running query first time, with no network; the translation
never failed. Every fault sits at the answer's edges. After the fix
round, every runner told the site's search from the store's items; what
remained, three of four runs placing it first or second: the never
fetched can be neither listed nor sized against the question; the MCP's
default answer (about 31 KB) is over the cut of every runner's harness
(about 30 KB), so two runs chose an item from one row; the terms block
over the scope cost a re-read in three runs after its words were fixed
twice; the person's "would a buyer see it" was inferred from four calls
because no field says a tab is public; and ids and jargon (C81, S26,
"a sitting", the Remove-only suffix) stayed unreadable.

## The test every step passes

A change earns its place only if it removes work from the agent at the
surface, and it reuses a shape the tool already has rather than adding
one. Owner, 2026-10-18: "keep simplicity, usability, and elegance as
guiding lights. Patching edge cases and adding mechanisms is likely to
be seductive, and may be necessary, but should only be done in service
to the goal of making the system better and easier for you to use."

## The rulings (owner, 2026-10-18: "I accept all 6. Go ahead.")

1. The answer's weight: total and rows before the terms block on both
   surfaces; the JSON carries a count's request only when routes are
   asked, as the CLI already does; the terms stay, without requests
   (C115). 2. C93's meaning stays; no cascade number; the demotion is
   the fix, measured at the re-sit (C115). 3. GGG's facts about a tab
   are fields — public, remove-only, a price's side — and the tool says
   nothing of what the site lists (C117). 4. An ambiguous catalogue row
   may carry its other reading; the answer counts the items in scope it
   touches, with a route (C119). 5. No id in user-facing text (C116).
   6. The item's age on its row, and the never fetched said by tab type
   (C118); why a tab was never fetched, by policy or not yet, is left
   out until the re-sit asks.

## The sequence

Steps 1 to 4 run one at a time — three touch the search crate's answer
— each an Opus worker under this note's step brief, reviewed against the
gate by the leading session and then by the external review (Astra's
Codex pass), one commit per fault or feature, the gate run bare before
each commit. A worker gets the finding and the brief, never the fix
expected; what it proposes beyond its brief is a park, not a commit.

| Step | Builds | Acceptance |
| --- | --- | --- |
| 1 | the answer's weight (C115); `tabs` limited and summarised in JSON as the CLI's text is; `left_out_needs` names a JSON field | on a copy of the owner's store: an MCP three-row answer under 4 KB, the default twenty rows under 20 KB, a counts-only answer under 4 KB, `tabs` under 10 KB by default. Built 2026-10-18, `e57db22b..2afc262c`: `tabs` met (3,360 bytes); the three others not — 7,888, 23,105 and 7,965 bytes on the ring URL — the weight left being the trade report (2.7 KB a call) and the rows' evidence; the report's share is step 3's |
| 2 | coverage (C118): the row's age; the never fetched by tab type on the scope line; the scope line points at the list that exists (`tabs`) instead of a flag not built | the tests; the re-sit. Built 2026-10-18, `d483a199..bb82f568`, the review's one fault (a `tabs` pointer with no league) fixed at `8b4446f4`: `place.fetched_at` and the by-type scope line, counted by `tabs`'s own maker, the pointer naming the query's league or the one holding the most never fetched; the ring URL's three rows 7,888 to 8,196 bytes, twenty rows 23,105 to 23,784 |
| 3 | the stake of an ambiguous clause (C119), over the catalogue's ambiguous rows that admit another reading; the trade report's weight | T6 says how many Blight-ravaged maps the site may add, with the route; T4 its `reduced` forms; T5 the items the scourge line decides; each a labelled oracle or translate row; the ring URL's three-row answer measured again on the store copy. Built 2026-10-18, `5dbbad12..2589b9de`: four rows carry another reading (T6 adds 13, T4 and T5 add and drop none, each with its route), a bound within one and the map tier parked; an exact clause with a term of its own keeps `at`, `status`, `term`, `path`, one without (a sum's filter, a group) its whole row (the review's one fault, `c74f837f`); the `tree` stays (the MCP takes it back as a request); three rows 8,196 to 7,067 bytes, twenty 23,784 to 22,587 |
| 4 | the tab facts as fields (C117); ids out of help, reasons, notes and refusals (C116), the Remove-only suffix said once where tabs are listed; the seat's small unrouted faults as `seat_faults` tests | the tests; the references regenerated. Built 2026-10-18, `6b9206fe..259cad75`, eight commits: `tab.public`, `tab.remove_only`, `price.side` as closed-set fields over the reads the store has, the mark said once under `tabs`; 139 ids swept from every surface by one test over the references, `describe` and every crate's string literals, the two frontends parks deleted; `--limit` beside a count says a count shows no rows; `show`'s price line ends with its `price show` command; a sorted total's `shows` already held every contributing line (test alone); an unknown name's refusal names the `describe` entry that lists them all, by the closed set's existing rule; the trade header's "in every place" is gone; `--view locations` went rather than gained a refusal. The tool listing 23,717 to 23,854 bytes on the wire, the ring URL's three rows 7,067 unchanged. The review's one fault: the `price show` pointer named no account or realm, so it failed where two accounts are logged in or the item is Xbox's; it now opens as every printed command does (`answer::opening`), with the account the item was shown for and its realm |
| 5 | the sitting the fate tables queue (Blight-ravaged under `map_blighted`, the `(gem)` level, the rarity forms), beside the build | the site-sitting skill |
| 6 | the re-sit (the seat skill): the person's question split in two — is this search the site's, and which of these could a buyer see — one task whose honest answer turns on a never-fetched tab, Opus and Sonnet on both surfaces | the fate table; the three re-reads of step 1 counted |
| 7 | step 9 of `TRADE-SLICE.md`, the close | the session-close skill |

## Step briefs

**Step 1 — the answer's weight.** Finding: mcp-opus-2 "I learned to
scroll for `total`"; a counts-only answer of 16,742 bytes (call 12),
13,693 bytes for 17 numbers (mcp-sonnet-1 call 8), 745,672 bytes of
`tabs` in three runs, `left_out_needs: "--limit"` in a JSON answer.
Build: the order on both surfaces (trade header, query, scope, total,
view, how to continue, terms, routes, basis — JSON keys in that order,
since an agent reads JSON as text); a count's or a term's `request`
only under `view.routes`, the CLI's `--routes`; `tabs` in JSON with the
CLI's counts-first shape and an `expand`; a row's `shows` as slim as the
decision view allows (C53, C100). Leave: `zero.offers` (the way on),
`view.rows.next`, `describe`'s sizes. Deletes the frontends park "The
search tools' second round" in its commit; the dropped-request park
stays. Tests in `search_tools.rs` and `seat_faults.rs`, named for the
fault; the acceptance measured on the store copy and written in the
commit message.

**Step 2 — coverage.** Finding: cli-opus-2 fault 3 (a map search not
told the map stash is 61 of 2,550 fetched), cli-sonnet-2 fault 1,
mcp-opus-2 fault 2, mcp-sonnet-2 reading 2,940 as items by T1; the age
asked by cli-opus-2 and mcp-sonnet-2 for "where do I pick it up". Build:
the location's `fetched_at` on the row beside `place` — text age in the
CLI, epoch in JSON (C53); the never fetched on the scope line by tab
type, the few largest named and the rest counted, so a map question
sees `MapStash` at once with no mapping from category to tab type; the
scope line's "the full list: --view locations, not built" replaced by
the list that exists. Whether `--view locations` keeps any purpose
beside `tabs` is the worker's proposal; the coverage-list park is
deleted or rewritten in the commit. Nothing reads the sync policy.

**Step 3 — the stake of an ambiguous clause.** Finding: both Opus
runs measured each ambiguity by hand at three or four calls (cli-opus-2
calls 36–38, 42–43; mcp-opus-2 34, 38, 40); mcp-opus-2: "the label
does not hint at how much is at stake"; mcp-sonnet-2 fault 6. Build: an
optional other reading on an ambiguous catalogue row, a fragment in the
language, with the clause's bound where it needs one — the Blight-ravaged
prefix at the tier bound, the `reduced` form, the sum with and without
the scourge line, a defence within one of its bound; the answer counts
the items in scope that reading would add or drop against the query as
run, prints the count on the clause's line with its route (the query
with the clause replaced), and prints nothing where a row carries no
other reading (C93: absent, never zero). The reading applies where the
clause has every property of the shape asked (`TRADE-SLICE.md`, shapes
of fault). A ruling in `decisions/trade.md` names `TRADE-SLICE.md`'s
ledger row 8b.
The report's weight, measured after step 1 (ring URL, three rows, 7,888
bytes): `query` is 2,732 of them — nine clauses each echoing the site's
filter (`site`), its catalogue row and the row's sentence (`notes`, the
same sentence four times for four pseudo stats), beside the `text` that
already states the whole translation and a `tree` that restates the
`text`. Two questions for the worker, answered by the shape, not a flag:
what the report says of an exact clause that the `text` does not (the
clauses that differ from the text — ambiguous, unsupported,
listing_only — are the report's reason), and whether an answer needs
the `tree` beside the `text` (`describe` and `explain` exist). Whatever
is cut, `describe trade` keeps the catalogue's sentences (C53: text a
function of JSON; C100: one shape).

**For step 6 to count, left as step 2 built them.** The CLI's total
carries its own caveat ("2940 locations were never fetched: nothing
here says what they hold", a first-round fix for cli-sonnet call 6) two
lines under the scope line that now says the same number by type; and
the scope's second line runs to about 190 characters over the owner's
store. Neither is patched before the re-sit says whether a reader
skips, misreads or is served by them. Step 4 adds two of its own: the
words "the site shows public listings" (the trade header, the search
tool's `query`) do not name `tab.public`, so a reader asking "would a
buyer see it" finds the field through `describe fields` or not at all;
and a row sorted by a total shows each contributing line's text, not its
weight, so a hybrid line counted twice (105 = 43 + 36 + 2 × 13) is
summed by hand. The re-sit's record says whether either cost a call.

**Step 4 — facts, words, and the small faults.** Build: `tab.public`
(GGG's `metadata.public`; a substash's is its tab's; a character's item
lacks it), `tab.remove_only` (the suffix GGG puts on the name, N47),
`price.side` as fields in the index; no claim about the site. No `C`,
`S`, `N`, `T` or `Q` id in any help text, reason, note, `says` or
refusal — the trade report's rule (`the_reports_words_cite_no_record`)
widened to every surface as one test; the refusal of `--view locations`
(still refused by name, nothing pointing at it since step 2) names
`tabs` or goes; the two frontends parks on ids
deleted in the commit. One sentence on the Remove-only suffix where
tabs are listed. The seat's unrouted faults, each a test: `--count`
with `--limit` says no rows are shown under a count (cli-sonnet both
rounds); `show` prints the `item/<id>` form `price show` takes
(cli-sonnet-2 fault 6); a row sorted by a pseudo total shows every
contributing line (mcp-opus both rounds); a refused count key's
readings fit (`kind`, mcp-sonnet-2 fault 7); the trade header's "in
every place" cannot read as full coverage (cli-sonnet-2 debrief 4).

**Step 6 — the re-sit.** The brief from `3e84db17` with the person's
question split, one task that turns on a never-fetched tab (a unique in
the unique stash, 65 of 502 fetched), the same world recipe, a fresh
record directory under `audits/`, the debrief's questions as the seat
skill now has them. Its record counts, for step 1, how many runs re-read
the terms block.
