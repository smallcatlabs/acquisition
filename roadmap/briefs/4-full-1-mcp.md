# 4-full-1-mcp — the saved search on the MCP: three tools and `search.saved`

Step 2 of 4-full part 1 (`roadmap/briefs/4-full-1-design.md` §3 "The
MCP" and §7 step 2), for an Opus worker that has not seen the design.
Written against the tree at `26ae36c1`, after step 1
(`roadmap/briefs/4-full-1-saved.md`) closed on 2026-11-07 (`roadmap/
LEDGER.md`, its last row). The owner asked for this brief on
2026-11-07. Where the design's §3 and step 1's brief differ, the brief
won and §3 says so; this brief builds over what step 1 built, not over
the design's earlier words. Cut 2026-11-07 under the cold review at
`6da0c34b` (`roadmap/reviews/4-full-1-mcp-brief-6da0c34b.md`), the foot
saying what each finding changed.

## Goal

When this step is done an agent at `acq-mcp` lists, shows, saves,
changes and clears saved searches and runs one: three tools,
`saved_searches`, `saved_search_set` and `saved_search_clear`, and
`search` taking `saved`, each an adapter of the verbs step 1 built in
`crates/acquisition-verbs/src/saved.rs`, so every answer and every
refusal is the document `acq saved … --json` or `acq search --saved …
--json` prints for the same request (C46, shared semantics in Rust:
a tool calls the function the CLI calls). The MCP's `Spoken` spells the
five saved ways as its own tools' arguments, so every offer a saved
receipt or refusal makes is sent back as printed and answers. This is
morning 1's handoff on the agent's side (the design, §3, "The
handoff"): the owner saves at a terminal or, at step 3, on a page; the
agent lists and runs; the agent saves; the owner lists, with
`written_via` saying the channel. Nothing reaches the daemon.

## Read

- `AGENTS.md`, `roadmap/README.md` "A step", and `TEMPLATE.md`.
- `roadmap/briefs/4-full-1-design.md` §3 whole, closely "The verbs",
  "The MCP" and "The handoff"; §6 (no `saved` spelling in the search
  crate's `QueryInput`: the name resolves in the verbs; never handing
  the agent the web URL) and §7 step 2.
- `roadmap/briefs/4-full-1-saved.md` Do 3 and Do 4: the verbs' and the
  CLI's contract as built, which this step adapts and does not change.
- `decisions/frontends.md`: C11 (`conflict` with `revision` on the CLI
  and the MCP), C13 (the MCP a thin client, never spawning in real
  mode), C52 (an agent never clobbers intent it has not read: a write
  names the revision it replaces, the create-only form), C53 (ten
  listed, `limit` and `expand`; on the MCP the JSON is the text), C116
  (no ruling id in user text), C121 (an account that cannot be picked,
  in the protocol's words), C143 (a shared text speaks the request's
  words; a way is a value each surface spells; an adapter may add its
  own spelling after them), C149 (one JSON object), C163 (every time
  has its words); `decisions/store.md` C167 (the user file, per world,
  not per account) and C35. `AGENTS.md` C46, C122, C153, C154.
- `crates/acquisition-verbs/src/saved.rs`: the module doc; `Set`
  (`id`, `expect`, `name`, `text`, `realm`, `membership`) and the
  `(id, expect)` pairs `set` accepts, every other refused as
  `arguments` before a file is opened; `list(frontend, limit, expand)`
  → `SavedSearches`; `show(frontend, id_or_name)` → `SavedRow`;
  `request_of(frontend, id_or_name, account)` → `Request`;
  `set(frontend, account, &Set, &mut mint)` and `clear(frontend, id,
  expect)` → `SavedWritten`; `SavedWritten::undo`; the refusals
  `saved_search_not_found`, `ambiguous_saved_search` (with `searches`,
  `total`, `left_out`), `realm_needed` (from `resolved_realm`, each
  realm offered as the same create), `conflict` (with `revision` and
  `current`, from `conflicted`); `check_id`. `crates/acquisition-verbs/
  src/refusal.rs`: `Way`'s five saved variants (`SavedSearches {
  expand }`, `SavedSearchShow { id }`, `SavedSearchRun { id, account
  }`, `SavedSearchSet(Box<Set>)`, `SavedSearchClear { id, if_revision
  }`), `Surface`, `again`; `receipt.rs` `saved`.
  `crates/acquisition-plan/src/pricing.rs` `Expect::or` (none is the
  default, `0` is `Create`, another `Exactly`).
- `crates/acquisition-cli/src/saved_cmd.rs` (how the CLI adapts the
  same functions: `Expect::Current` its default where an id is named,
  `receipt::saved(&w, &Terminal::writing())` under `--json`) and
  `search_cmd.rs`, the `--saved` branch (the beside list; the row's
  request folded into the arguments; its refusal's kind and words).
- `crates/acquisition-mcp/src/main.rs` (`python3 -I tools/bothy.py pack
  --path crates/acquisition-mcp/src/main.rs` prints the rulings to read
  first): the module doc, "As built: pricing" and its one stricter rule
  (a write naming no revision is a create); `Spoken` and its `request`,
  whose five saved arms answer `None` since step 1; `Way::Again`'s arm
  for `search` (`refusal::again`, which reads the arguments with the
  search crate's reader and answers none for what it cannot read);
  `Args` (`parse`, `read`: an argument a tool does not take is
  `arguments`); `RequestParams` and its schema; the `search` tool;
  `price_set` and `price_clear` (the pattern: `Expect::or(p.if_revision,
  Expect::Create)`, the receipt spelled by `Spoken { named: "account"
  }`); `ACCOUNT`; `WRITTEN_VIA` and `Frontend::via`; `get_info`'s
  instructions.
- `crates/acquisition-mcp/tests/`: `main.rs` (one binary, C122);
  `harness/mod.rs` (`Mcp`, `acq`, `scratch`, `document`,
  `c143_a_text_speaks_no_frontend` and its `DATA` keys,
  `assert_spoken_as`); `pricing_tools.rs`, the pattern C46 pins:
  `cli`, `tool`, `steady` with `RECEIPT`, `offers_alike`,
  `undo_alike`, `each_pricing_write_is_the_receipt_the_cli_prints`,
  `every_receipts_undo_sent_back_as_printed_puts_the_prior_back`;
  `search_tools.rs` `seed` and `seed_bob` (a second account) and
  `spoken.rs` `seed` (pc and poe2, two realms): the facts this step's
  tests stand on, made visible to the new module, never copied (#142's
  rule: no new seed function); `tool_refusals.rs`
  `every_tool_refuses_an_argument_it_does_not_take` (walks
  `tools/list`: a new tool joins it by existing); `surface_size.rs`
  (`DESCRIPTION` 600, `PARAMETER` 300, `INSTRUCTIONS` 1,000,
  `FIRST_TURN` 38,000, and
  `a_group_sentence_names_only_the_tools_it_is_true_of`, which reads
  the instructions' "write locally" line); `mcp_reference.rs`
  (`the_reference_matches_the_server`, regenerating `MCP-REFERENCE.md`
  under `ACQ_UPDATE_FIXTURES=1`, and
  `mcp8_the_readmes_tool_count_and_list_are_the_servers`, which reads
  the README paragraph holding " tools do what the CLI does" and takes
  every backticked word after that phrase as a tool name: a backticked
  word there that is no tool fails it, which bit step 1's worker);
  `ggg_refusal.rs` (the spending tools in real mode; the saved tools
  spend nothing and do not join it).
- `README.md`, the MCP paragraph ("its 27 tools do what the CLI does:
  …") and the paragraph after it, which opens "The saved searches (`acq
  saved`) have no tool yet."

## Input

Fixed: the verbs' contract as step 1 built it (`saved.rs`, closed);
C52's create-only form as the pricing tools speak it; the design's §3
"The MCP" as this brief reads it below. The tool names are step 1's
own already: the verbs' test surface `Plain` names `saved_searches`,
`saved_search_set` and `saved_search_clear`
(`crates/acquisition-verbs/tests/documents.rs`, its `request`).

**The first turn, measured.** `initialize` and `tools/list` from the
debug `acq-mcp` built at 08:49 on 2026-11-07 (after `0ca39d50`; the
next commit, `f073c494`, touched no file of the MCP crate), driven in
a mock world (`ACQ_PROVIDER=mock`, `ACQ_STORE_DIR` and
`ACQ_NO_SPAWN=1` under scratch) by a script sending what the harness
sends and counting each result as compact JSON in UTF-8 bytes:
`initialize` 1,127, `tools/list` 36,818, together **37,945 bytes
against `FIRST_TURN`'s 38,000**, 55 bytes free; the instructions
**994 bytes against `INSTRUCTIONS`' 1,000**, 6 free. The three
pricing tools nearest these in shape measure, each whole as
`tools/list` serves it, `price_show` 2,208, `price_set` 2,186 and
`price_clear` 1,350 bytes; three tools of that order add some 4,000 to
5,000 bytes, an estimate in bytes from those three, not a measurement.
The step measures its own in its first commit
(`the_whole_first_turn_is_held_to_its_bound`'s method).

The last push's CI: `Gate (Linux)` run 42383992629 and `Gate
(Windows)` run 42383992628, both on `26ae36c1`, in progress when this
brief was written; the worker reads their conclusions first, and a red
run is named in the report.

The failing inputs the tests stage: a set with an id and no
`if_revision` over a live row; a
clear with no `if_revision`; a clear at `if_revision: 0`; a write at a
revision that moved; a set naming a nonzero `if_revision` and no id
(`0` is the create sentinel, Do 3); `saved` beside each search field
it excludes, and a key the reader does not know beside it; `saved`
beside `limit` or `expand` on `saved_searches`; a name two rows share,
opened by `saved_searches` and run by `search`; a name and an id none
holds; a create naming no realm over a store of two realms; `search`
with `saved` and no `scope.account` over a world of two accounts.

## Do

1. **The bound, a ruling before the code** (the design's §7 step 2:
   "held under 38,000 bytes, the tools' texts cut to fit, or a
   measured ruling brought to the owner"). With 55 bytes free, no cut
   of the new tools' own texts fits them; fitting would cut some
   4,000 bytes from the other 27 tools' texts, which seats tuned. The
   draft for the owner, brought by the orchestrating session before
   the worker starts: *`FIRST_TURN` becomes the next 1,000 above the
   turn this step measures, its comment naming the measure, the three
   tools and this step, as #104 set it at 38,000 over 37,824.* Its
   alternative, his to choose: the bound stands and the step cuts the
   existing texts to fit, each cut named in the commit. The worker
   builds under whichever he rules and changes the constant only as
   ruled. `INSTRUCTIONS` is not part of the ruling: Do 6's line fits
   by cutting words inside the instructions.
2. **`saved_searches`** (no daemon, no account): arguments `saved`
   (an id or a name, as every `--saved` takes one), `limit`, `expand`.
   With `saved`, the tool answers `saved::show`'s document, the row;
   without it, `saved::list(frontend, limit, expand)`'s. `saved` beside
   `limit` or `expand` is refused as `arguments`, naming the pair (a
   row is never a list: C53). Its description says the saved searches
   are the world's, not an account's, that each readable row carries
   `request`, the search it is (a row this build cannot read is named
   by its id, revision and why), and that `search` runs one by
   `saved`.
3. **`saved_search_set`**: arguments `id`, `if_revision`, `name`,
   `text`, `realm`, `membership`, `account`, read into the verbs' one
   `Set` with `expect: Expect::or(if_revision, Expect::Create)`, as
   `price_set` reads its revision, and handed to `saved::set(frontend,
   account, &set, &mut saved::mint)`. So the MCP's forms are: no `id`,
   a create (`name` and `text` required, `realm` resolved over
   `account` as `acq search` resolves one where none is named); `id`
   and `if_revision`, a write over the row at exactly that revision,
   the fields not named keeping the row's values (a rename is `id`,
   `if_revision` and `name`); `id` and no `if_revision` (or `0`), a
   create at that id, the whole value required, landing only where no
   live row stands there, which is a clear's undo. `if_revision: 0`
   is the create sentinel with or without an id, as `price_set` reads
   it; a nonzero `if_revision` with no id, and every other pair, is
   the verbs' `arguments` refusal; the MCP never writes at
   `Expect::Current` (C52: no blind replace). `account` is read only
   where a create names no realm, to resolve it, and the tool's
   description says so. Its parameter description is `ACCOUNT`, the one
   sentence every tool that takes `account` shares
   (`account_is_one_sentence_on_every_tool_that_takes_it`), and a
   shared sentence is true of every tool that carries it: today it
   promises `unknown_account` for any name none holds and
   `not_logged_in` with none known, which this tool meets only on the
   create that reads it (with none known it saves over `all`). The
   worker words `ACCOUNT` so it is true of this tool too (a tool that
   reads no account where it needs none, say), its added bytes counted
   in Do 1's measure, and never adds an account lookup to make the old
   words true. The answer is
   `receipt::saved(&written, &Spoken { named: "account" })`.
4. **`saved_search_clear`** (no account): arguments `id` and
   `if_revision`, both required in the schema, read as
   `saved::clear(frontend, id, Expect::Exactly(if_revision))`: not
   through `Expect::or`, since a clear is no create and the MCP has no
   clear naming no revision. A missing `if_revision` is `arguments`
   naming it; `if_revision: 0` names a revision no row stands at, so
   the store answers `conflict` with the revision that stands and the
   clear offered at it. The answer is the receipt, as Do 3's.
5. **`search` takes `saved`**: an id or a name, beside which
   `scope.account`, `view` and `context` stand and nothing else of a
   request: `query`, `scope.realm`, `scope.membership`, `request`,
   `basis`, `count` and `counted_at` beside it are refused, each
   naming the word, in the kind the CLI's `--saved` refusal carries and
   its sentence with the request's words in place of the flags (as
   `search_cmd.rs` builds it: a `SearchError::Scope`, so it reaches the
   agent through `refused`). The adapter takes `saved` out of the
   arguments, calls `saved::request_of(frontend, saved,
   scope.account)`, and sets the row's `query.text`, `scope.realm` and
   `scope.membership` into the arguments as sent, so every other key
   the call sent, known or not, well formed or not (`scope.acount`, a
   numeric account), reaches `Asked::read`, the one reader every
   request passes, and is refused there as today; the search crate
   never sees the word `saved` (the design's §6). `scope.account` is
   read for `request_of` only where it is a string; anything else is
   left for the reader to refuse. A refusal of `request_of` is spelled
   with `named` `scope.account`. The schema's `saved` property says
   what it is and what stands beside it; the tool's description gains
   one clause; H1 in the module doc and `RequestParams`' doc, which say
   the arguments reach the reader whole and nothing here reads them,
   name the one exception, `saved`, resolved before the reader.
6. **`Spoken` speaks the five ways**, each a tool's arguments, the
   tool named where it is another than the one refused:
   `SavedSearches { expand }` → `saved_searches`, `{}` or `{"expand":
   true}`; `SavedSearchShow { id }` → `saved_searches`, `{"saved":
   id}`; `SavedSearchRun { id, account }` → `search`, `{"saved": id}`
   with `scope.account` where `account` names one;
   `SavedSearchSet(set)` → `saved_search_set`, its `id` where named,
   `if_revision` under `Exactly(n)`, none under `Create` (the MCP's
   create, at the id where one is named), and the fields named;
   `Expect::Current` answers `None`, a form the MCP does not send (no
   way the verbs make carries it today); `SavedSearchClear { id,
   if_revision }` → `saved_search_clear`, both. `Way::Again` on
   `search` with `saved` among the arguments: `refusal::again` reads
   none of it (it is no request the reader reads), so the arm sets
   `scope.account` in the arguments as sent, and nothing restarts. The
   arm's comment saying step 2 gives these tools is replaced. The
   module doc gains "As built: the saved search", stating Do 2 to Do 5
   and the stricter rule the pricing section states. The instructions'
   "write locally and send nothing" line names `saved_search_set` and
   `saved_search_clear` beside the three it names, and the
   instructions stay within `INSTRUCTIONS`' 1,000 bytes by cutting
   their own words, each cut named in the commit.
7. **The words outside the server.** `MCP-REFERENCE.md` regenerated
   under `ACQ_UPDATE_FIXTURES=1`. The README's MCP paragraph says 30
   tools and lists the three in a group of their own ("keep searches"
   or the like), with no other backticked word after " tools do what
   the CLI does"; the next paragraph's "The saved searches (`acq
   saved`) have no tool yet." goes, and its "The store and pricing
   tools need no daemon" names the saved tools among them.
8. **Pins, what the adapter adds** (C122), one module
   `saved_tools.rs` in `tests/main.rs`, in `pricing_tools.rs`'s shape,
   the CLI beside the server over one world each, no daemon:
   `pricing_tools.rs`'s helpers (`cli`, `tool`, `steady`,
   `offers_alike`, `undo_alike`) are made visible to the module and
   reused, `undo_alike` taking the command prefix it checks, never
   copied.
   - **The document the CLI prints** (C46, the pin that each tool calls
     the verbs' function the CLI calls): `saved_searches` bare, with
     `limit` and with `expand` over eleven rows, by id and by name,
     each equal to `acq saved list|show … --json`; `saved_search_set`
     as a create, as a write by `id` and `if_revision`, as a rename,
     and as a create naming `all`, and `saved_search_clear`, each
     receipt equal to the CLI's under `steady` with `RECEIPT` (the
     channel and the undo compared apart, the undo saying the same:
     `undo_alike`, its `acq saved ` prefix). A create with no id mints
     its id at random in each world, so the two receipts differ there
     by design: each receipt's minted id is first checked to be the
     same at every place it stands in that receipt (`target.id`,
     `written.id`, the undo's), then replaced by one placeholder in
     both before they are compared, and every later write and replay
     in a world uses that world's own id; `search` with `saved` (by
     id and by name, with a `view`) equal to `acq search --saved …
     --json` and to `search` with the row's text and scope.
   - **The refusal shape**, each equal to the CLI's for the same
     request where the CLI can make it (`offers_alike`):
     `saved_search_not_found` on a name and on a cleared id;
     `ambiguous_saved_search` from `saved_searches` and from `search`;
     `conflict` at a moved revision, with `current`; `realm_needed` on
     a create over two realms; and, the MCP's own, Do 3's set with an
     id, the whole value and no `if_revision` over a live row refused
     as `conflict` with nothing written (C52), and with a name alone
     refused as the verbs' `arguments`, Do 4's clear with no `if_revision`
     (`arguments`) and at `0` (`conflict`), Do 2's `saved` beside
     `limit`, Do 5's beside-rule for each word it excludes, and a key
     the reader does not know beside `saved` (`scope.acount`) refused
     by the reader as it is without `saved`.
   - **Every offer sent back as printed answers**: each undo (a
     create's clear, a replace's set at the revision written, a clear's
     create at the id) sent to the tool it names lands and puts the
     prior back; each `ambiguous_saved_search` offer, from either tool,
     answers, the eleven-row name's offer of every row (`expand`)
     among them; `saved_search_not_found`'s offer of the list answers;
     the conflict's offer lands; each `realm_needed` offer
     lands; `search` with `saved` and no account over two accounts
     refused `ambiguous_account`, each offer answering (Do 6's
     `Way::Again` arm).
   - **The handoff, both ways**: a row written by `acq saved set` is
     listed by `saved_searches` and run by `search`; a row written by
     `saved_search_set` is listed by `acq saved list --json` with
     `written_via` `mcp`.
   - **The size bounds**: `surface_size.rs` as ruled in Do 1, its
     per-text bounds walking the new tools by existing; no other new
     bound.
   - **The text speaks no frontend**: every result this module meets
     passes `c143_a_text_speaks_no_frontend`. A saved row's `name` and
     `text` are stored data; if the rule flags one, that is a finding
     for the report, not a key added to `DATA` silently.
   Every test under C137 (no wall-clock bound). Nothing the verbs'
   `documents.rs` already pins is pinned again here: the merge at a
   revision, the mint, the eleven-row bound, the unreadable row.
9. **The gate and the commit.** `tools/gate.sh` green on the step's
   tree; one commit per concern or one for the step, the last naming
   this brief; the first commit carries the measured first turn in its
   message. The gate run and the push are the orchestrating session's,
   on the owner's word.

## Done when

- `acq-mcp`'s `tools/list` serves `saved_searches`,
  `saved_search_set` and `saved_search_clear`, and `search`'s schema
  has a `saved` property; `MCP-REFERENCE.md` shows them, and
  `the_reference_matches_the_server` is green.
- In `crates/acquisition-mcp/src/main.rs`, the three tools call
  `saved::list`, `saved::show`, `saved::set` and `saved::clear`, and
  `search` calls `saved::request_of`; no tool reads `user.db` or
  evaluates a saved search itself (`tools/arch-check.sh`'s door grep
  green).
- `Spoken::request` has no arm answering `None` for a saved way but
  `SavedSearchSet` under `Expect::Current`, and no comment saying the
  saved search has no tool.
- `saved_search_set` reads its revision through `Expect::or(…,
  Expect::Create)` and `saved_search_clear` through `Expect::Exactly`;
  neither tool takes a form that writes at `Expect::Current`.
- `crates/acquisition-mcp/tests/saved_tools.rs` exists, is listed in
  `tests/main.rs`, and holds each pin of Do 8, each named for the
  behaviour it holds; it defines no seed function of its own.
- `surface_size.rs`'s `FIRST_TURN` is as the owner ruled in Do 1, its
  comment naming the measure; `INSTRUCTIONS` is 1,000.
- The README's MCP paragraph names 30 tools and the three by name, and
  no sentence says the saved searches have no tool;
  `mcp8_the_readmes_tool_count_and_list_are_the_servers` is green.
- The module doc of `main.rs` has a saved-search section.
- Every retirement in a commit message is in C154's form.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- The pages, the save form, `saved/open` and C168's pin: step 3.
- Any change to the verbs' contract (`saved.rs`, `refusal.rs`'s
  `Way`, `receipt.rs`): step 1's, closed. A verbs word that reads
  wrong on the MCP is raised in the report, not changed; an adapter
  adds its own spelling after the shared words where C143 allows it.
- Any change to the daemon, the protocol or the store.
- A `saved` spelling in the search crate (`QueryInput`, `Asked`): the
  design's §6.
- The CLI: `acq saved` and `acq search --saved` stand as built.
- The mock seat the design names as this step's first consumer: the
  orchestrating session's, under the seat skill, after the close.
- Cutting other tools' texts, unless the owner rules so in Do 1.

## Rules

`TEMPLATE.md`. What differs:
- Work in the main checkout on `main`; no worktree, no branch. Commit
  locally, do not push; the orchestrating session gates, pushes and
  reviews. (Binds your work.)
- Every `acq` and `acq-mcp` you run is in a mock world (the
  mock-session skill): `ACQ_PROVIDER=mock`, `ACQ_STORE_DIR` under your
  scratch. Never a bare `acq`; nothing in this step needs a daemon.
  (Binds your work.)
- Do 1's ruling comes before the code: a bound changed without it is
  a finding against the step. (Binds the thing you build.)
- A tool's argument name, a description's words or a refusal's
  sentence this brief leaves open is the step's own agent-facing
  design, named in the report and in the commit that builds it.
- A sentence of this brief the tree disproves is corrected in the
  report, not silently built around.

## Report

`TEMPLATE.md`. Add, under 3, the first turn and the instructions in
bytes before and after, measured as `surface_size.rs` measures them.

## For the owner's review

Agent-facing design decided here, the session's (the design names the
three tools and `saved`, and leaves these open or loose):
- `saved_searches` takes one `saved`, an id or a name, where the
  design wrote "`id` or `name` for one": the word `search` takes, one
  argument for one function (`show` takes either), and no pair to
  refuse.
- `saved_searches` and `saved_search_clear` take no `account`, where
  the design wrote that each tool takes one: the saved searches are
  the world's (C167), and neither verb reads an account; an `account`
  sent is refused as an argument the tool does not take, which says
  so. `saved_search_set` takes it, to resolve a create's realm.
- `saved_search_set` with an `id` and no `if_revision` is a create at
  the id, the clear's undo: the design's "neither to create" covered
  the create with no id alone.
- `saved_search_clear` requires `if_revision` and reads `0` as a
  revision, where `price_clear` reads it as a create the pricing verb
  then refuses naming the current revision: the saved verbs refuse a
  clear at `Create` as `arguments` in words naming a clear with no
  revision, which the MCP does not have.
- Beside `saved`, `search` takes `context` as well as `scope.account`
  and `view`, as the CLI takes `--context` beside `--saved`; `request`,
  `basis`, `count` and `counted_at` are refused, as `--request` and
  `--next` conflict with `--saved`.
- `if_revision: 0` with no id is a create, as `price_set` reads `0`,
  where the CLI refuses `--if-revision` without `--id`: the MCP's two
  set tools read `0` alike.

The ruling Do 1 drafts, the owner's: the first turn's bound.

## Taken from the review (`6da0c34b`, cut 2026-11-07)

1 the creates compared with each world's minted id checked consistent
and then masked, later writes using each world's own (Do 8); 2 `saved`
resolved by setting the row's text and scope into the arguments as
sent, so the reader still refuses every key it does not know, pinned
(Input, Do 5, Do 8); 3 a shared sentence true of every tool that
carries it: `ACCOUNT` worded to hold on `saved_search_set`, the tool's
limited use said in its description, no account lookup added (Do 3);
4 `0` the create sentinel with or without an id, the failing input a
nonzero revision, listed for the owner (Input, Do 3, the foot); 5 the
list offers of `saved_search_not_found` and of an eleven-row name
replayed (Do 8); 6 H1 and `RequestParams`' doc name the `saved`
exception, "each readable row" (Do 2, Do 5).
