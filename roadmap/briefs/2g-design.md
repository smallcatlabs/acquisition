# 2g — The read economy and the refusal and receipt bundle: the design brief

Pull-forward item 1 (`roadmap/README.md`, "Pull-forward"; the phase 2
row's 2g sentence: "one bound at the shared layer under C53's three
output levels"), issues #9 and #10 with their comments of 2026-10-27.
A design brief: its output is the questions below, each with its
evidence, its options and a recommendation, the ruling questions with
their text drafted for the owner, and the named pins. The build brief
is written after he rules; nothing is built here. Both issues are one
question seen twice: what an agent is handed per call (the bytes of an
answer, a receipt or a refusal) and what shape lets it act without a
second read.

## Goal

An agent's call pays for what the next decision needs and nothing
else, and what it is handed back, an answer, a receipt or a refusal,
carries what it acts on next: the counts, the rows it asked for, the
request that continues, the revision the next write names, the undo,
the offer it sends back as printed. One rule for a list on every
surface, one shape for a refusal, one for a receipt, each held by one
test at the lowest crate (C122, a behaviour pinned once), so the next
seat meets no answer it must read twice and no count it must make by
eye.

## Read

- #9 and #10, bodies and comments: the release seat's numbers and the
  readiness seat's, by call.
- `roadmap/README.md`: "A step"; "Pull-forward"; the phase 2 row's 2g
  sentence; the ledger rows for #82 (the held corpus), #83 (the
  surface trimmed; its "Open" item: both copies of a result kept, the
  schemas the larger half) and #84 (the seat; rule 6, a typo'd account
  as `not_logged_in`, kept under C121 and the owner's to reopen; rules
  7 and 8 on #9 and #10; the `readings` prose-or-payload line, the one
  place the spelling depends on the error type). Under "Open", "For
  4-full, from 4b": the cards over rows carrying fields, and a web
  phrasing for the next actions the pages print as command lines.
- `roadmap/briefs/TEMPLATE.md` (the form), `roadmap/briefs/r2-keyring-design.md`
  (a design brief's rules), `roadmap/briefs/p83-surface-trimmed.md`
  (the measurement driver and the sizes it states).
- `decisions/frontends.md`: C53 (three output levels; "ten or fewer
  entities are listed, more are counted"; "ages are text, epochs are
  JSON"; "JSON changes are additive"), C11 (the failure document;
  `kind: "conflict"` with `revision`), C52 (no blind replace), C115
  (the answer leads with what bears; `tabs` counts-first with a
  limit), C116 (no id, no store name in a refusal), C121 (an account
  that cannot be picked), C143 (a shared refusal speaks the request's
  words; the adapter adds its spelling), C146 (a count says what it
  counts; a JSON field's name is the contract).
- `decisions/plans.md`: C44 and C76 (one door: a selection compiles
  to a plan; a plan's actions are the authorization, C38), C126 (a
  policy receipt's `league_notes`).
- `decisions/pricing.md`: C69 (a listing's two sides with revisions),
  C74 (every omitted item counted with a reason), C78 (a write returns
  the prior so a hand can undo it), C81.
- `decisions/search.md`: C98 (one basis, the held corpus), C100 (one
  answer shape; a continuation names its basis; "an answer that
  carries its evidence needs no second call"), C115, C118 (a row's age
  beside `place`: text in the CLI, an epoch in JSON).
- `decisions/daemon.md`: C85 (the frame bound and the closed error
  kinds), C86.
- `roadmap/GUI-NOTES.md` sections 6.2 (server-windowed rows keyed by
  item id; bucket headers are a counts view, an expanded bucket a rows
  view scoped to the tab) and 7 (the card and dense-table gap: rows
  carrying fields from the one maker `show`), dated evidence.
- `audits/readiness-2026-10/seat/REPORT.md` (sections 2, 3, 5, 6),
  `DEBRIEF.md` (answers 2, 5, 10), `README.md` (the fate table) and
  `calls.csv` (the text and whole sizes per call);
  `audits/release-2026-10/seat/README.md` ("What this seat counted",
  item 1; "The faults", the first two rows and the parked rows).
- `crates/acquisition-mcp/src/main.rs`: `told` and `told_as` (the
  document as the text block, read back as the structured copy),
  `spoken` and `PAYLOAD` (what is never respelled), `Prose` (the
  `readings` line), `failed`, `refusal`, `refused`, `refusing`,
  `priced` (the three places a refusal is sorted), `Args` (arguments
  refused as a document).
- `crates/acquisition-verbs/src/fault.rs`: `Fault::kind` (the kind
  words on every surface) and `Fault::fields` (`accounts`, `next`).
- `crates/acquisition-plan/src/pricing.rs`: `Refused`, `RefusedKind`
  (`conflict` with `revision`, `not_found`), `refused_write` (the live
  row is read before the refusal is made), `SetReceipt` (`write`,
  `in_facts`); `crates/acquisition-plan/src/price.rs` (`PriceWrite`:
  `written`, `prior`); `crates/acquisition-plan/src/listing.rs`
  (`items_here`, `items_covered_below`, each a whole `Vec<Listing>`);
  `crates/acquisition-plan/src/shop.rs` (`LEFT_OUT_LISTED` 50,
  `left_out_total`, `expand`, `posted` and `freshness` unbounded).
- `crates/acquisition-search/src/answer.rs`: `DEFAULT_LIMIT` 20,
  `RowsOut` (`limit`, `returned`, `after`, `left_out`, `next`), `Next`
  (a request, never a second token), `RowField` (`fields`, the icon
  for 4b); `crates/acquisition-search/src/tabs.rs` (`Tabs`: counts,
  `listed`, `rows`); `crates/acquisition-search/src/error.rs`
  (`to_json`: `readings`, `offers` as `{request}`).
- `crates/acquisition-store/src/lib.rs`: `CharacterRow` (two epochs,
  two counts), `Store::characters` (a bare `Vec`);
  `crates/acquisition-verbs/src/jobs.rs` (`Jobs::Live` a bare array,
  `Absent` an object); `crates/acquisition-cli/src/store_cmd.rs`
  (`ago`, the text's age; the characters footer the JSON lacks);
  `crates/acquisition-cli/src/price_cmd.rs` (`LIST_UP_TO` 10, the
  text's bound over a container's items; `render_write`, the printed
  undo).
- `crates/acquisition-protocol/src/protocol.rs`: `MAX_FRAME_BYTES`
  (64 MiB), the only bound today on a daemon answer (#56 holds the
  daemon's lists; not this step).
- `MCP-REFERENCE.md` as it stands: the sizes the descriptions state
  (`price_list` "some 1.3 KB an item", `shop_render` "some 200 bytes
  an item", `tabs` "thirty or fewer, else the ten").
- The tests under "The tests that pin today's shapes" below.

## Input

The evidence, by call. Text is the result's text block, what the
seat's door prints and the runner reads; whole is the result on the
wire, the structured copy included.

| Seat, call | Tool | Text | Whole | What it shows |
| --- | --- | --- | --- | --- |
| readiness 11 | `characters` {} | 15,805 | 33,671 | 65 rows, a bare array, no counts; the debrief's two retracted counts were made by eye from it |
| readiness 27 | `price_show` of a tab | 28,857 | 62,615 | 39 items under `items_here`, each whole, to find the one with a note |
| readiness 35 | `shop_render` {} | 34,779 | 75,401 | a 2,800-character page; 50 `left_out` entries and 38 ids under `freshness` before the page's `text`; the run's largest answer |
| readiness 2, 8, 18 | `accounts`, `search`, `show` | 237, 1,465, 3,547 | | epoch seconds for `last_login`, `oldest_fetch`, `first_seen`; the runner ran `date -u -r` outside the door |
| readiness 46 | `tabs`, a typo'd account | 175 | 476 | `not_logged_in` with `accounts`; the runner asked for `unknown_account` |
| readiness 47 | `price_show`, no such tab | 184 | 486 | `not_found` naming `acq search` and `acq tabs` (since respelled under C143), no `offers`, no `next` |
| readiness 49 | `price_set`, a stale revision | 158 | 436 | `conflict` with `revision: 4`, no current value, so "whether to overwrite" needs a `price_show` |
| readiness 28 to 31, 36, 37 | `price_set`, `price_clear` | | | receipts `target`, `written`, `prior`, `in_facts`; the undo rule stands only in `describe` pricing; a clear's `written` is null and its tombstone's revision is nowhere |
| release, four runs | `shop_render` | 5.1 MB | | `left_out` every item nothing applies to; fixed to 50 and counted (`d79c4ca7`); `posted` and `freshness` still unbounded; the rerun cut at 160 KB (mcp-opus-2, call 37) |
| release, three runs | `price_list` | 49 to 67 KB, cut | | no limit; fixed with `limit`, `total`, `listed` (`27cfd3c7`) |
| release, cli-sonnet, mcp-sonnet | a tab's `price show` | 22 to 174 KB | | every item of the container; parked, this step's |
| release, mcp-opus-2 31, 39 to 53 | `price_show`, the receipts | | | `manual.revision` a container's when inherited; a clear's receipt without the revision the next write names |
| #83's measurement | `search` {} | 7,431 | 16,126 | both copies sent: a result is a little over twice its document |
| #83's measurement | `describe` ["trade"] | 24,359 | 51,396 | the same, on the largest named section |
| readiness, the first turn | `open` | 31,472 | | the surface after #83; not this step's |

Fixed before the build starts: the readiness seat's pristine world
(`~/seat-worlds/readiness-2026-10/pristine/`, the owner's items as the
mock serves them) for the before-and-after measurement of the three
calls above (11, 27, 35), by the same driver #83 used; the tests named
below, each read before its shape moves.

## The pattern underneath

Every answer today is made at one of four makers (the search crate's
answer, the planner's pricing and shop documents, the store's rows
through the verbs, the verbs' job documents), each with its own idea
of a list: the search bounds rows at 20 and continues with `next`;
`price_list` takes a `limit` and says `total` and `listed`; the shop
lists 50 of `left_out` with `left_out_total` and `expand`; `tabs`
lists the whole under thirty, else ten, with `expand`; `item_events`
takes a `limit` of 200 and counts nothing; `characters`, a container's
`price_show` and a live `list_jobs` list everything and count nothing.
Four makers, six rules. The text level already obeys one rule, C53's
ten or fewer, which is why no CLI seat met the sizes the MCP seats
met: the CLI prints the decision view and the MCP sends the JSON,
and the JSON has no decision view. The read economy is C53's one
sentence reaching the JSON.

A refusal is likewise sorted in three helpers (the roadmap's review
checklist, "A sibling sentence left standing") and a receipt is made
per verb. What an agent does with each is one of four things: branch
on `kind`, send an offer back as printed, re-type from a reading, or
hand `next` to a person; and after a write, name `revision` or send
the undo. One shape per document kind, carrying exactly those, is the
bundle.

## The questions

Each with the options, the evidence for each, and a recommendation;
"ruling" marks a question the owner answers (a C53 level, a JSON
contract change, a kind in the protocol's closed set, C121),
"delegated" one the build decides under p0's ruling (b), recorded in
the commit that builds it and listed at the close.

### Q1. One bound: what is bounded, by what number, where (ruling)

What a list in a JSON answer holds by default.

- *Per-tool limits*, as today: each maker keeps its number (20, 50,
  ten or thirty, 200, none). Evidence against: six rules for one
  need; the three calls above are the tools with no rule; every new
  document invents one.
- *One bound in bytes*: no default answer over N KB. Evidence against:
  bytes are a presentation measure over rows of unlike size (1.3 KB a
  priced item, 200 bytes a `left_out` entry, 9.5 KB a row carrying
  every field for 4-full), so the window an agent or a page gets
  would vary with what a row carries, and a test on bytes pins a
  mechanism. The frame bound (64 MiB) stays the wire's bound.
- *One bound in rows, C53's own number*: a list of like entities in a
  JSON answer holds ten by default (the text's rule reaching the
  JSON), more on `limit`, all on `expand` where the tool has it, with
  its count beside it, how many it left out, and, where the maker
  continues (the search's `next`), the request. What an answer is for
  is never a list and never cut: a page's `text`, a plan's actions
  (C38, binding; a cut plan would authorize less than reviewed), a
  body asked for with `show`, a `describe` section. Evidence for: the
  search's shape already has every part (`limit`, `returned`,
  `left_out`, `next`); the CLI's text already lists ten; `tabs` and
  `price_list` already say the count first; the seat's three largest
  answers fall to a few KB each with nothing an agent then lacks (it
  asks `limit` for more, as call 14 asked 30 of 187). The search's
  default moves from 20 to ten, a default and not a shape.

Recommended: the third. The number lives once (the lowest crate every
maker links, the build's choice under C1), and each maker applies it;
`limit` is an argument on every tool that lists, `expand` stays where
C53 put it (the audit view) and means the list whole. Field names are
not renamed (C53, additive): a field added takes the search answer's
words (`total`, `returned`, `left_out`, `next`), and the names that
already stand (`listed`, `left_out_total`, `by_type`) stand.

C53's text, amended (the sentence after "Ten or fewer entities are
listed, more are counted."), recommended, 420 bytes:

> In JSON the same number is the default: a list of like entities stands beside its counts, holds ten unless `limit` names more or `expand` asks for all, says how many it left out and, where the maker continues, the request that continues it; what an answer is for (a page's text, a plan's actions, a body asked for, a `describe` section) is never a list and never cut. The number is held once, and every maker applies it.

### Q2. C53's three levels on the MCP: the text block and the structured copy (ruling on the levels; delegated on the copy)

Today `told` sends the document twice: as the text block, which the
seat's door and a host hand the agent, and as structured content,
which the tests read; a result is a little over twice its document.

- *The text block becomes the decision view, a rendering as the CLI
  prints, and the structured copy the contract.* Evidence against:
  "any request an answer holds is sent back as printed" needs the
  payload (offers, `next`, requests) in what the agent reads, so a
  prose text must carry the JSON too, and the agent read JSON well at
  every seat; what hurt was size and missing counts, not the form.
- *Send the structured copy alone.* Evidence against: a host that
  reads no structured content shows the agent nothing; the MCP
  specification's shape is a text block beside structured content.
- *Send the text block alone.* Evidence for: halves every result on
  the wire; structured content is optional where no output schema is
  declared, and this server declares none. Against: fifteen lines in
  six test files and the harness read the structured copy; #50 asks
  that every answer be an object for a host that reads it strictly.
- *Keep both; make the document's default the decision view (Q1).*
  Evidence for: the agent's read is the text block, so the bytes the
  agent pays fall with the document, not with the copy; the copy is
  the host's. Against: nothing measured; whether a host hands the
  model both copies is unmeasured here, and if one did, every answer
  would cost twice.

Recommended: on the levels, the fourth: on the MCP the agent reads the
JSON itself, so the decision view and the contract are one document
whose default shape is the decision view (Q1) and whose audit view is
`limit` and `expand`; no second rendering. On the copy, delegated to
the build: keep both, and measure once, in the build's message, what
the host at hand (Claude Code, the one the seats run under) hands the
model of a result that carries both; if it hands both, the structured
copy goes and the tests read the text, one line each.

C53's text, amended (a sentence added at the end), recommended, 246
bytes:

> On a surface whose reader is a program (the MCP) the JSON is the text the reader gets: the decision view is the document's default shape, the audit view its `limit` and `expand`, and no second rendering stands between the reader and the contract.

### Q3. Every answer an object, its counts beside its rows (ruling: one non-additive change)

`characters`, `item_events` and a live `list_jobs` answer a bare
array; every other tool answers an object. The debrief's answer 5
(two counts wrong, made by eye from call 11) and answer 10 ("which
numbers did you derive yourself?") rest on the array; #50 rests on
the MCP's definition of structured content as an object.

- *Leave the arrays; add the counts elsewhere.* Nowhere to put them.
- *Wrap each array in an object with the counts the CLI's footer
  already computes* (`characters`: the total, fetched, never fetched,
  fetched empty, items; `item_events`: the total in the window, the
  rows returned; `list_jobs`: open, finished, listed), the rows under
  one key. Evidence for: the CLI's text already prints those counts
  (`store_cmd.rs`, the characters footer), so the JSON gains what the
  text has and text stays a function of JSON (C53); the one rule of
  Q1 then holds on every answer. Against: a bare array becoming an
  object is the one change here that is not additive, so it is the
  owner's.

Recommended: the second, now, before `alpha.2`: the alpha's consumers
are the seats, the pages (`acq web` renders the verbs' documents and
is rebuilt with them) and no program anyone has reported. Delegated
beside it: the counts' words follow C146 (each says what it counts,
on every surface, the `describe` entry named).

Recommended ruling text, a new entry in `decisions/frontends.md`, 462
bytes (the id is the next free one; not written until he rules):

> **Every answer and every refusal is one JSON object, its lists counted beside their rows.** A tool answers no bare array; the counts an answer's text prints stand in its JSON under the words C146 asks, so text is a function of JSON (C53). The one change that is not additive, taken at the alpha, whose consumers are the seats and the pages. *Why:* a seat made two counts by eye from a bare array of 65 rows and retracted both. *Pinned:* the build names the test.

### Q4. Ages in words beside epochs, or instead, or neither (ruling: C53's sentence)

C53: "Ages are text, epochs are JSON"; C118 the same for a row's
place. On the MCP the agent reads JSON, so it reads epochs; the
readiness runner ran `date` outside the door and asked for words
(its answer 6); the release seat's mcp-opus marked its dates as hand
arithmetic, and both MCP Opus runs retracted arithmetic made from
positions and ages.

- *Words instead of epochs.* A contract change on every epoch field;
  a program loses the number; no.
- *An `at` epoch at every document's root so a program subtracts.*
  Evidence against: the agent is the program here, and it subtracts
  by eye; the search answer already carries `basis.snapshot.at` and
  the runner still left the door for `date`.
- *Words beside the epoch, for the epochs the text turns into ages.*
  Evidence for: additive; the CLI already decides which ages bear on
  a decision (`ago` in `store_cmd.rs`, `age` in the verbs' refresh)
  and prints them in one vocabulary (`6d ago`, `never`), so the rule
  is one sentence: an age the text prints, the JSON carries in words
  beside the epoch it is made from, made at the answer's own time. A
  row of `characters` gains two short strings; a search row's `place`
  gains one. Against: an age is relative to when the answer was made,
  so a program that caches the document reads a stale word; the
  epoch beside it is the fact, and the words say "at this answer".

Recommended: the third, with the vocabulary the CLI prints today and
one home for it (the function the text and the JSON share), so a
golden that changes changes once.

C53's text, amended ("Ages are text, epochs are JSON." becomes),
recommended, 210 bytes:

> Ages are text, epochs are JSON, and an age the text prints stands in words beside its epoch in the JSON, made at the answer's own time, so a program keeps the fact and a reader keeps the words on every surface.

### Q5. One refusal shape (delegated), and a typo'd account's kind (ruling, C121)

Today a refusal is `error`, `kind`, and per kind `accounts`, `next`
(a command for a person), `offers` (requests, `{request}`),
`readings` (prose on a scope refusal, payload on a language one),
`revision`, `span`. The seat asked for three things: an offer on
`not_found` (call 47), the current value in a conflict (call 49) and
a kind for a name that matches nothing (call 46).

*The shape, delegated.* One document on every surface, in the CLI's
`--json` as on the MCP: `error`, the words (C116, C143); `kind`, one
word from one closed list across the surfaces (the protocol's, the
verbs', the planner's and the search's words as they stand; no new
word without a ruling); and at most four optional keys, each meaning
one thing wherever it stands: `offers`, requests the agent sends back
as printed, each `{request, says}` and naming `tool` when it is
another tool's (an offer on `not_found` is a `search` or `tabs`
request; an offer on `not_logged_in` with names known is the same
call with each name, so a program recovers by sending an offer, not
by reading the kind); `readings`, how what was typed could be read,
payload on every refusal (the scope refusal's `--realm pc` becomes the
request's words in the search crate under C143, the CLI adding its
flag, which retires the prose-or-payload line); `next`, one sentence
or one command that is a person's act, never an agent's; and the
kind's own fields (`accounts`; `revision` with `current`, the row as
`price_show`'s `manual` prints it, which `refused_write` has in hand
when it makes the refusal). A key with nothing to say is absent, never
null. The search's own refusal (`SearchError::to_json`) is already
this shape but for `readings`; the three helpers in `main.rs` sort
into it and no tool decides alone.

*The kind, ruling.* The runner asked for `unknown_account` for a name
the index does not hold; C121 answers it `not_logged_in` with
`accounts`, because the daemon answers a named account it has no
session for the same way (C85's closed set), and one word for one
mistake lets an agent branch alike whether the verb went to the store
or the daemon.

- *Add `unknown_account`.* Evidence for: the runner's reading, "a
  program branching on `not_logged_in` would send the person to log
  in". Against: the daemon cannot tell a typo from a name with no
  login, so the new word would stand on the store's side only, two
  words for one mistake across the surfaces, which is what C121 was
  ruled against; the refusal already carries `accounts`, and under
  the shape above it carries the offer that fixes the typo.
- *Keep C121; the refusal's words say the two readings and the offer
  does the work.* The `error` says "no login made a store for
  `GERWARIK#7694`; the names known: GERWARIC#7694", and `offers`
  carries the call with the known name.

Recommended: keep C121 as it stands, no new kind; the owner's word
either way, since he kept it at the seat's close and marked it his to
reopen. No text to draft unless he reopens it; if he does, the
amendment is one clause in C121 naming `unknown_account` "when the
selector names none the index holds and the index holds some", and
the daemon's side stays `not_logged_in`.

### Q6. One receipt shape for every write (delegated)

Three tools write (`price_set`, `price_clear`, `set_sync_policy`);
each answers its own receipt. #10 names a clear's `written: null`
without the tombstone's revision, `price_clear`'s undo text, and
`manual.revision` being a container's when inherited; the seat
found the undo rule in `describe` pricing alone and a clear moving
the revision on (calls 36, 37, 48).

Recommended, one shape: `target` (the policy's account and realms
for a policy write); `written`, the row now standing, null after a
clear; `prior`, the row before, null on a create; `revision`, the
revision now current, which the next write on this target names (a
clear's is its tombstone's, so `written: null` stands and the number
is there); `undo`, the request that restores `prior`, an offer in the
shape of Q5 (the CLI prints it as the command it prints today, which
runs as printed; the MCP sends it as a request); and the write's own
facts (`in_facts` on a price write; `league_notes` on a policy
write, C126). Additive on both surfaces. For a policy write the build
verifies the prior is at hand (the compare-and-swap reads the stored
row to merge into it) before promising `prior`; if it is not, `prior`
is a price receipt's alone and the report says so. The inherited
`manual.revision` is already answered in words (`price_set`'s
`if_revision` doc: the revision when `listing.manual.from` is the
target itself); under this shape `listing.manual` gains nothing, and
a write naming a container's revision is refused as a conflict naming
the target's own revision or null, which is the teaching refusal.

### Q7. What 4-full needs that this must not foreclose (no question; constraints)

- The dense table is server-windowed rows keyed by item id with a
  client cache (GUI-NOTES 6.2); Q1's bound is that window: rows, an
  argument, no ceiling below the frame; `next` stays a request and
  `after` stays an offset, both in the answer, never an opaque token.
- Cards need rows carrying fields (GUI-NOTES 7; `view.rows.fields`,
  the icon today): a row then weighs KB, which is why the bound is
  rows and not bytes; nothing here touches the row's fields.
- The tree's bucket headers are a counts view and an expanded bucket
  a rows view scoped to it: Q3's counts beside rows are those
  headers, one shape the page reads as the agent does.
- The pages print next actions as CLI command lines beside their
  buttons ("Open", from 4b, "a web phrasing"); Q5's offers and Q6's
  `undo` as requests are that phrasing, one shape every renderer
  reads, so this step takes the verb change 4b left open rather than
  adding a page-side one later.
- The web UI with no account on disk shows the refusal as a page
  (from 4b): one refusal shape is one page template.
- The MCP's `search` description naming `view.rows.fields` stays for
  the web UI; not this step's.

### Q8. What the one rule closes per tool, and what the issues named that is closed already or stays elsewhere

Closed by Q1 alone: `shop_render` {} answers the pages, the counts,
ten of `posted`, ten of `left_out` with `left_out_total`, `freshness`
with its lists bounded and counted, `expand` for the rest; a
container's `price_show` answers its counts (items here, covered
below) and ten of each, `price_list` with `in` the route to the rest,
named in the description; `characters`, `item_events`, `list_jobs` by
Q3; `tabs` keeps counts first and lists ten with `limit`, its "thirty
or fewer whole" folded into the one rule (the C115 sentence stays
true: counts first and a limit). `describe` is by name since #83 and
each section is what an answer is for; not cut.

Closed before this step, named in #10 for the record: refusals naming
the CLI's flags (C143 at #84's fix round, the `spoken.rs` sweep); job
tools' malformed arguments as JSON-RPC errors (`Args` reads arguments
and refuses them as `arguments`, pinned in `tool_refusals.rs`);
`set_sync_policy`'s conflict without its revision (C11 amended at the
release, `kind: "conflict"` with `revision` on both surfaces).

Stays elsewhere: the refused bodies listed (#18); the invalid notes
named and `basis.intent` explained (#11); the daemon's `List` and
`Quote` over the frame (#56, the daemon's, a ruling); the first turn's
schemas (#83's "Open"); `realm` as a counts key (the seat's fault 10,
no issue; the search's, if a second runner asks).

## Rulings and delegations

| Question | Who | What he reads |
| --- | --- | --- |
| Q1 the bound: rows, ten, one home | owner | C53's amendment, 420 bytes |
| Q2 the levels on the MCP | owner; the copy delegated | C53's second amendment, 246 bytes |
| Q3 every answer an object | owner (not additive) | the new entry, 462 bytes |
| Q4 ages in words beside epochs | owner | C53's third amendment, 210 bytes |
| Q5 the refusal shape | delegated | listed at the close |
| Q5 a typo'd account's kind | owner (C121) | keep; one clause if reopened |
| Q6 the receipt shape | delegated | listed at the close |
| Q7 4-full's constraints | neither; held by the build | |

The three C53 amendments are one entry edited once; together they add
876 bytes to an entry of 713, over the 800-byte line that prints a
note and never fails (p0's ruling (c)); if he would rather, Q1 and Q2
are the amendment and Q3 and Q4 one new entry each.

## The tests that pin today's shapes

Found by grepping the test trees for the refusal, receipt, limit and
age words; each keeps its name and its fact words and changes only
where it reads the shape.

- The bounds and the counts: `crates/acquisition-search/tests/next.rs`
  (the continuation), `surface_faults.rs`
  (`c100_a_rows_evidence_is_bounded_and_says_what_it_left_out`),
  `answer.rs` (`c115_an_answer_unasked_carries_only_the_routes_a_next_step_hangs_on`),
  `rows.rs` and `explain.rs` (requests with `limit: 20`);
  `crates/acquisition-cli/tests/search_json.rs`
  (`c53_search_json_is_the_answer_whole_and_the_text_is_a_function_of_it`),
  `price_json.rs` (`price_list_is_bounded_by_a_limit_and_says_how_many_of_how_many`,
  `price_list_with_a_limit_of_zero_says_none_of_the_total`,
  `a_named_limit_lists_the_items_it_bounds`, `price_json_documents_are_the_views_the_text_reads`),
  `shop_json.rs` (`shop_render_json_is_the_document_the_text_and_the_page_read`),
  `seat_faults.rs` (`tabs_says_its_counts_first_and_counts_a_large_league`,
  `tabs_lists_the_never_fetched_of_one_type`,
  `jobs_lists_what_is_open_and_counts_the_finished_it_leaves_out`);
  `crates/acquisition-cli/src/price_cmd.rs`
  (`c53_price_list_groups_by_container_and_lists_ten_or_fewer`,
  `c53_the_expanded_list_show_and_status_are_linear_over_the_owners_league_tiled`),
  `shop_cmd.rs` (`c53_shop_render_opens_with_one_line_and_labels_every_page`,
  `c53_the_expanded_render_is_linear_over_the_owners_league_tiled`);
  `crates/acquisition-plan/src/shop.rs`
  (`c74_every_item_is_posted_omitted_blocked_or_off_the_page_and_counted`);
  `crates/acquisition-plan/src/listing.rs`
  (`c53_at_real_scale_the_report_round_trips_and_every_container_shows_its_two_sets`);
  `crates/acquisition-mcp/tests/pricing_tools.rs`
  (`the_shop_document_bounds_what_it_leaves_out_alike_on_both_surfaces`,
  `each_pricing_read_is_the_document_the_cli_prints`),
  `search_tools.rs` (`tabs_counts_first_and_lists_the_ten_with_the_most_items`,
  `tabs_lists_the_never_fetched_of_one_type`,
  `item_events_reads_back_its_hours_and_defaults_to_a_day`,
  `the_answers_text_leads_with_its_total_never_with_its_terms`),
  `surface_size.rs` (the descriptions that state a size or "thirty or
  fewer"), `mcp_reference.rs` (the reference regenerated);
  `crates/acquisition-store/src/lib.rs`
  (`characters_query_lists_live_rows_with_item_counts`);
  `crates/acquisition-cli/tests/web.rs` (seven page goldens; the jobs
  page `the_jobs_page_opens_with_the_halt`) and `readme_tour.rs`.
- The refusals: `crates/acquisition-mcp/tests/tool_refusals.rs`
  (`a_tool_refusal_is_a_result_marked_is_error_with_its_kind`),
  `search_tools.rs` (`an_authoring_error_is_a_result_with_its_kind_and_readings`,
  `a_query_nested_too_deep_is_refused_and_the_server_answers_the_next_call`,
  `arguments_a_tool_does_not_take_are_refused_as_a_document`,
  `the_requests_account_selects_the_store`),
  `pricing_tools.rs` (`a_blind_replace_and_a_blind_clear_are_refused_naming_the_current_revision`,
  `a_pricing_refusal_names_the_argument_after_each_flag_it_speaks`),
  `daemon_status.rs` (`a_job_tool_with_no_daemon_names_the_command_that_starts_one`,
  `the_no_daemon_remedy_names_the_provider_and_store_the_server_runs_with`),
  `spoken.rs` (the sweep), `ggg_refusal.rs`;
  `crates/acquisition-cli/tests/price_json.rs`
  (`a_write_refused_for_its_revision_is_a_conflict_naming_it_in_the_verbs_words`,
  `a_rows_refusal_names_the_account_it_ran_as_so_its_command_runs_with_two_known`,
  `a_rows_refusal_in_another_league_prints_the_show_that_displays_it`),
  `seat_faults.rs` (`an_account_that_cannot_be_picked_says_which_failure_and_lists_the_accounts`,
  `a_command_line_the_parser_refuses_is_one_json_document_under_json`),
  `error_kind.rs`, `no_ids.rs`;
  `crates/acquisition-verbs/tests/documents.rs`
  (`a_faults_kind_and_fields_are_the_protocols_words` and the four
  account tests after it);
  `crates/acquisition-search/tests/refusal.rs`, `offers.rs`,
  `explain.rs` (C143), `seat_faults.rs`.
- The receipts: `crates/acquisition-mcp/tests/pricing_tools.rs`
  (`each_pricing_write_is_the_receipt_the_cli_prints`,
  `a_clear_lands_alike_on_both_surfaces_without_the_facts`,
  `the_price_write_tools_say_a_clear_takes_a_revision`),
  `plan_loop.rs` (C52);
  `crates/acquisition-cli/tests/price_json.rs`
  (`price_set_and_clear_print_the_receipt_and_show_reads_it_back`,
  `a_prior_naming_a_retired_currency_promises_no_undo`),
  `seat_faults.rs` (`every_undo_a_price_receipt_prints_runs_as_printed_with_a_second_account_known`,
  `an_undo_quotes_a_target_a_shell_would_split`), `new_league.rs`;
  `crates/acquisition-plan/src/price.rs`
  (`c78_a_write_returns_the_prior_value_so_a_hand_can_undo_it`);
  `crates/acquisition-verbs/tests/documents.rs`
  (`a_policy_write_is_validated_and_compares_its_revision`).
- The ages: `crates/acquisition-mcp/tests/search_tools.rs`
  (`a_rows_place_carries_when_its_location_was_fetched`), the C118
  tests in `crates/acquisition-search/tests/seat_faults.rs` and
  `explain.rs`, and the goldens under
  `crates/acquisition-cli/tests/goldens/` that print "ago" (203 lines
  across them; a golden regenerates under `ACQ_UPDATE_FIXTURES=1` and
  names its facts).

## What the build brief would then draw

From `TEMPLATE.md`, after the rulings: the bound's number and its one
home, applied at the four makers with `limit` on every listing tool
and the counts of Q3, each maker's change pinned at its own crate and
the frontends' tests reading the shape alone (C122); the three
helpers in `main.rs` and the CLI's failure path sorting every refusal
into Q5's shape, with the scope refusal's readings reworded in the
search crate under C143 and `current` beside `revision`; the three
receipts in Q6's shape with `undo` as an offer, the CLI printing it
as the command it prints today; the ages' words beside their epochs
from the one function the text uses; the descriptions and `describe`
entries that state a size or a rule re-read against the new shapes
and the references regenerated; and the measurement, by #83's driver
over the readiness seat's pristine world, of calls 11, 27 and 35
before and after, stated in the build's message, with the one host
measurement Q2 asks. Its "Done when" is the three sizes, the tests
named above green or moved, the gate, and the next seat's brief
pointed at the shapes.

## Not in scope

- A new kind in the closed set, or a kind that stands on one surface
  only (Q5: the owner's).
- Renaming a JSON field that stands today (C53, additive): `listed`,
  `left_out_total`, `by_type`, `items_here` stay; a field added takes
  the search answer's words.
- A filter per tool for what a count names (`characters` by
  `never_fetched`, `price_list` by `invalid_notes` or `residue`):
  `limit` reaches the whole list deliberately, and a filter waits for
  the runner who asks twice (#11 holds the pricing ones).
- `store_refused` (#18); the words of #11; the first turn's schemas
  (#83's "Open"); the daemon's answers over the frame (#56); `realm`
  as a counts key.
- Rows carrying fields, the cards, the dense table (4-full); the time
  seam (4-0).
- A second rendering of a result on the MCP (Q2).
- The seat skill's wording and `SEAT-LEDGER.md` (the owner's).

## Rules

`TEMPLATE.md`, with these differences: a design, so no code and no
gate beyond `git diff --check`, `tools/docs-check.sh` and
`python3 -I tools/bothy.py check` on the commit that adds this
document; nothing under `crates/` changes; alone in the main tree on
`main`, committed alone, never pushed; the `Co-Authored-By` line for
the model last. The owner reads and rules; the build brief is written
after, by the session that starts the build, and names the rulings by
their ids and the delegated shapes by this file.

## Report

`TEMPLATE.md`, as text; item 1 is this document and its sha.
