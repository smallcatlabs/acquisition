# 4-full-1-saved — the saved search: `user.db`, the verbs, `acq saved`

Step 1 of 4-full part 1 (`roadmap/briefs/4-full-1-design.md` §3 and
§7, ruled 2026-11-06, "yes, go ahead"), for an Opus worker that has
not seen the design session. Written by the design session against
the tree at `bb81729e`; cut 2026-11-07 under the cold review at
`57d9361c` (`roadmap/reviews/4-full-1-saved-brief-57d9361c.md`), the
foot saying what each finding changed. Closes #42 by its "Fire" line,
the first user-scoped kind actually written; the close is the owner's.

## Goal

When this step is done a saved search exists: a row in a file of its
own, `user.db`, beside the account files, keyed on an id the verbs
mint, holding a name, a canonical query text and a scope (C167,
user-scoped intent in a per-world file); the verbs list, show, set and
clear it under revision compare-and-swap through the store's one door
(C35, intent written only through the store crate); `acq saved` is the
first consumer, and `acq search --saved <id|name>` runs one. Nothing
reaches the daemon. This is the substrate morning 1's handoff runs on
(the design, §3): the MCP's tools are step 2 and the pages step 3, and
both inherit every rule this step states once in the verbs (C46,
shared semantics in Rust).

## Read

- `AGENTS.md`; `decisions/store.md` C167 (the file and its first kind),
  C35, C151 (an unreadable row is named, never dropped); the store's
  "Not doing" line on the annotation wait; `decisions/frontends.md`
  C46, C11 (`--json` on every command, `conflict` with `revision`),
  C53, C116 (no ruling id in user text), C121 (one word per mistake,
  `accounts` beside it), C143 (a shared text speaks the request's
  words), C149 (one JSON object), C163 (every time has its words),
  C168 (a page writes as the verbs write: this step builds the verbs
  it names); `decisions/search.md` C96 (the realm is scope), C104 (the
  canonical text); `AGENTS.md` C122, C137, C153, C154.
- `roadmap/briefs/4-full-1-design.md` §3 whole (the design this step
  builds: the kind, the store, the verbs, the CLI; the MCP and the
  pages are later steps), §6 and §7 step 1; the foot's re-cut line.
- `crates/acquisition-store/src/annotations.rs`: the module doc;
  `Annotations::open` (a raw path, the uuid optional metadata),
  `open_for` (names the path and binds the uuid: stamps a file that
  has none, verifies a stored one), `open_read_only_for` (requires a
  uuid, verifies the stored one, stamps nothing, writes nothing: the
  unbound read-only opener this step factors out of it keeps every
  one of those), `open_memory`; `IntentValue` and `check_value` (C66);
  `put` (`expected_revision` `None` creates, and recreates over a
  tombstone with its revision carrying on; `Some(n)` replaces a live
  row at `n` and nothing else: a tombstone's revision is never named
  to it), `delete`,
  `get`, `list`, `revision` (one statement), `export`; the busy
  timeout and `ACQ_ANNOTATION_WAIT`, which the user file inherits.
  `crates/acquisition-store/unit/tests/annotations.rs` and
  `annotations_kinds.rs`, included by `#[path]` from `annotations.rs`
  (the `c35_`, `c65_`, `c66_`, `c151_` tests, and `test_kinds`).
- `crates/acquisition-store/src/world.rs`, "Durable state in the
  world": `<root>/<provider>/` holds `daemon.db` and the rails beside
  the account files; `user.db` joins them.
- `crates/acquisition-verbs/src/frontend.rs`: `Frontend` (`via`, `dir`
  = `<root>/<provider>`), `Account::intent` and `open` (the shape
  `Frontend::user` copies); `refusal.rs`: `Way`, `Surface`, `document`,
  `offer`, the module doc's sentence "no new word (C121, kept)" and how
  `refused.rs` added `refused_body_not_found` beside it (#18: C121
  rules the account words alone); `receipt.rs`: `Receipt`, `shaped`,
  `price_set`, `price_clear`, `price_undo`; `fault.rs`: `Fault` and
  `kind`. `crates/acquisition-plan/src/pricing.rs`: `Expect`
  (`Exactly`, `Current`, `Create`) and `Expect::or` (a frontend's `0`
  is `Create`, none the default, else `Exactly`), the one write
  expectation every surface already speaks.
  `crates/acquisition-verbs/tests/documents.rs`:
  `c149_every_document_a_verb_returns_is_one_json_object` and the C163
  walk, which the new documents join; `a_faults_kind_and_fields_are_the_protocols_words`,
  the kinds-and-fields pin the new words join; its `Plain` surface,
  which matches `Way` exhaustively, as the MCP's `Spoken` does
  (`acquisition-mcp/src/main.rs`, `request`).
- `crates/acquisition-cli/src/main.rs`: `Cmd` (`Price`, `Policy`,
  `Search`, `Show`: where `Saved` goes), `Terminal` and its `Surface`
  (`argv` spells a `Way` as `acq …` words, C156); `price_cmd.rs`
  `set` and `clear` (the receipt printed, `render_write`); `search_cmd.rs`
  `SearchArgs`, the `typed` list that refuses a word of scope, query
  or view beside `--next` (the rule `--saved` joins), the `unbuilt`
  list (untouched: `--fields` is step 5's); `tools/arch-check.sh`, the
  `doors` regex at its C46 check (`Annotations::open_for` is in it;
  `User::open_for` joins it).
- `crates/acquisition-cli/tests/main.rs` (one binary, C122),
  `harness/mod.rs` (`offline_acq`, `golden`, `Transcript`, `sole_json`),
  `price_json.rs` (the shape: seeded through the store crate, no
  daemon, `ACQ_NO_SPAWN=1`; a transcript golden; a conflict pinned in
  the verbs' words), `no_ids.rs` (reads the help texts and the web's
  goldens, not every golden), `cli_reference.rs` (regenerates the
  reference under `ACQ_UPDATE_FIXTURES=1`) and `readme_tour.rs`
  (checks that every tour line resolves under clap; it runs nothing);
  `README.md`, the `acq price` and `acq policy` tour lines (the form
  `acq saved`'s copies), and its three account sentences this step
  must keep true: "a second login makes every verb need `--account`"
  (the mock paragraph), "every command takes `--account`" and "a verb
  that reads the store takes the one account known" (the account
  paragraph), "its 27 tools do what the CLI does" (the MCP paragraph).
- `crates/acquisition-cli/src/web/mod.rs` `mint`: sixteen random bytes
  as hex through `getrandom`, the minting this step copies at twelve
  hex characters; `crates/acquisition-cli/Cargo.toml`, the `getrandom`
  line the verbs crate copies.

## Input

Fixed: the design's §3 as re-cut (the kind, the id as key, the name a
field, no uniqueness over names, `name_taken` and
`ambiguous_saved_search`), C167 and C168 as entered. The last push's
CI: `Gate (Linux)` run 42339589586 and `Gate (Windows)` run
42339589510, both on `bb81729e`, in progress when this brief was
written; the worker reads their conclusions first, and a red run is
named in the report. No measurement. The failing inputs the tests
stage: a set naming a revision that moved; a create-only put meeting a
standing id, once and twice; two rows of one name written through the
store door; a name no row holds; an id that is a tombstone; a text the
language refuses; a scope naming no realm over a store of two; a row
of a later version; a prior a clear cannot write back; eleven rows of
one name; a world whose provider directory does not exist.

## Do

1. **The store: `user.rs`** (`acquisition-store/src/user.rs`), a `User`
   newtype over `Annotations::open` at `<dir>/user.db`, no uuid bound:
   `open_for(dir)`, `open_read_only_for(dir)` over an unbound read-only
   opener factored out of `Annotations::open_read_only_for` (the
   account one is that opener, then the stored uuid verified against
   the requested, refused where it differs, accepted where the file
   holds none, nothing stamped, nothing written: its contract as it
   stands, its tests kept, an absent-uuid case added), `open_memory()`;
   `put`, `get`, `list`, `delete`, `revision`, `export` delegated,
   and `list` answering the rows and the file's revision from one
   transaction. `SavedSearch`, the kind: `KIND` `saved-search`,
   `VERSION` 1, value `{version, name, text, scope: {realm,
   membership}}`, its `parse` strict (every field present, `name` one
   to sixty-four characters after trimming with no line break, `realm`
   one of the four, `membership` one of the three) and nothing of the
   language: the store links none. The scope word `search` as a
   constant the verbs use; the key is the id. A newtype, so a
   signature taking `&Annotations` cannot be handed the user file.
2. **The store's tests, one table.** The API's properties the `c35_`,
   `c65_`, `c66_` and `c151_` tests hold over the account file
   (compare-and-swap, the tombstone's ABA guard, the revision sum,
   provenance, a value refused at the door, an unreadable row named
   and cleared at its revision, the export) become one table in
   `unit/tests/annotations.rs` run over an opener for each file, so a
   property is pinned once and holds for both (C153); the file's
   mechanics (the opening race, a foreign writer, synchronous mode)
   stay pinned once, since `User` opens through the one `open`.
   `c167_` tests pin only what the newtype adds: the path, the kind's
   parse, the scope word, the one-transaction list, that the account
   opener still binds its uuid and the user opener binds none.
   Retirements, if the table replaces a test outright, in C154's form
   in the commit message, `retired X; held by Y`.
3. **The verbs: `saved.rs`.** `Frontend::user` opens `User::open_for(dir)`
   as `Account::intent` opens its file; a provider directory that does
   not exist is `not_logged_in` naming the directory, as a store read
   answers (C121). The documents, each one JSON object (C149) joining
   `documents.rs`'s walks (C149, C163):
   - `list`: `searches` counted (ten by default, `limit` and `expand`
     as every listing: C53), each row `id`, `name`, `text`, `scope`,
     `revision`, `written_via`, `actor`, its times with their words,
     and `request`, the search request the row is (`scope` and
     `query.text`, no account); the file's `revision` beside them.
     The id is printed beside the name as an item's and a tab's ids
     are (C116 forbids a ruling's id, never a record's).
   - `show(id or name)`: one row, the same shape.
   - `set`, **one write shape on every surface**, the verbs' rule
     stated once here and inherited by the CLI now and the MCP and
     the pages later: a `Set` value of `id: Option`, `expect: Expect`,
     and the fields `name`, `text`, `realm`, `membership`, each
     optional. The combinations, every other refused as `arguments`
     naming the pair: no id and `Create`, a create needing `name` and
     `text`; an id and `Exactly(n)`, the row read, the fields not
     named keeping its values, the write landing at `n`; an id and
     `Current`, the same at the revision read, so a write racing in
     between conflicts rather than clobbers (C52 holds: the row read
     is the row written over); an id and `Create`, the whole value
     required, landing only where no live row stands (the store's
     `None`: over a tombstone of that id the revision carries on; a
     live row refuses naming its revision), which is a clear's undo.
     A rename is a `Set` of the id and a name. *Why one rule:* a
     rename and a re-scope are the common edits, and a shape that
     takes the value whole would have an agent resend a text it did
     not mean to touch, C52's clobber in a smaller form; two shapes
     for one kind across two surfaces is the fault C46 and C143 exist
     to prevent. The text is read with the language's reader in the
     row's realm and stored canonical (C104); an authoring error
     refuses before the file is touched; the realm of a create is
     resolved as `acq search` resolves one (C96: over a store of one
     realm it is that realm, over several it is required). Answers the
     receipt (`receipt.rs`, shaped as `price_set` is): `written`,
     `prior`, `revision`, and `undo`: a replace's is a `Set` of the
     prior value whole at the written revision; a create's is its
     clear; spelled by each surface as a `Way`, or `no_undo` with its
     reason where the prior cannot be written back (a prior this build
     could not read).
   - `clear(id, expect)`: the receipt with `prior`, its `undo` a `Set`
     of the id, the prior value whole and `Create`, which recreates
     over the tombstone as the price undo recreates after a clear
     (`receipt.rs`, `price_undo`'s `Set` with no revision); the
     printed undo is replayed in a pin and lands.
   - `request_of(id or name)`: the search request a row is, what
     every `--saved` resolves through; an id is accepted wherever a
     name is.
   - **The id.** Minted by a function the create path takes
     (`&mut dyn FnMut() -> String`), the production one twelve hex
     characters from `getrandom` (the verbs crate gains the
     dependency, pinned as the CLI's `Cargo.toml` pins it); a create
     that meets a standing id mints once more, and a second collision
     is refused as `refused` in words saying two minted ids were taken
     and to try again: the mint is called at most twice, so it cannot
     loop.
   - **The name's courtesy check.** A set whose name another live row
     holds is declined before the write as `name_taken`: with one such
     row, offering the set re-aimed at it (the `Set` with its id and
     revision); with several, offering each row's show and run by id
     and saying a distinct name makes a new row, since a re-aim at any
     one would meet the others. The check is a read before the write
     and holds no uniqueness: a race may leave two rows of one name.
   - **The refusals**, three words joining the closed list as
     `refused_body_not_found` did (`refused.rs`), each with its fields
     and each bounded as every list is (C53, C149: ten rows beside
     `total` and `left_out`, the offers for those ten, `expand` the
     way to the rest): `saved_search_not_found` (a name or id none
     holds, offering the list), `ambiguous_saved_search` (a name
     several rows share, `searches` naming each id and name, the way
     on a run or a show by id: one word per mistake as C121 gives
     accounts), `name_taken` (above). A stale revision is the store's
     `conflict` with `current` (C11). `Way` gains `SavedSearches` (the
     list), `SavedSearchSet` (the `Set` whole, with `account` where a
     run needs one) and `SavedSearchClear`; the two surfaces that
     match `Way` exhaustively gain an arm each, the MCP's `Spoken`
     answering `None` for the three until step 2 and the tests'
     `Plain` naming them; the three words join
     `a_faults_kind_and_fields_are_the_protocols_words` with their
     fields; `refusal.rs`'s "no new word" sentence gains the precedent
     in a clause. The door grep in `tools/arch-check.sh` gains
     `User::open_for`.
4. **The CLI.** `acq saved` beside `acq price` in `main.rs`: `list`
   (the default; `--limit`, `--expand`), `show <ID|NAME>`, `set [<ID>
   --if-revision N] --name NAME ['<query>'] [--realm] [--membership]`
   (`--if-revision N` through `Expect::or`: `N` is `Exactly(N)`, `0`
   is `Create`, none is `Current`; without an id the expectation is
   `Create` and `--if-revision` is refused as `arguments`), `clear <ID>
   [--if-revision N]`;
   `--json` the document on each (C11), the text as `price set`'s
   `render_write` prints a receipt and `price list` a listing (C53).
   `acq search --saved <ID|NAME>` runs the row's request: it joins the
   `typed` list, so a query, `--query-file`, `--trade`, `--realm` or
   `--membership` beside it is the refusal `--next` gives, while
   `--account` and every view flag stand beside it; `--next` and
   `--request` conflict with it as with a query. `Terminal::argv`
   spells the three new `Way`s as `acq saved list`, `acq saved set …`
   and `acq saved clear …` words (C156). `CLI-REFERENCE.md` regenerated; the README
   tour gains one `acq saved` line and one `acq search --saved` line,
   in the `acq price` lines' form, which `readme_tour.rs` resolves
   under clap; the README's three account sentences (Read) gain their
   clause: the saved searches are the world's, so `acq saved` takes no
   `--account` and needs no login, while `acq search --saved` takes
   one as `acq search` does; and the MCP paragraph says the saved
   verbs have no tool until step 2 (shape 3, a sibling sentence).
5. **Pins, at the lowest crate that holds each** (C122). The store:
   Do 2. The verbs, in `documents.rs`: the documents join the C149 and
   C163 walks; `set` as a create, a replace at `Exactly`, a replace at
   `Current`, a rename keeping the text, a conflict at a moved
   revision in the store's words; the canonical text stored and a
   refused text touching no file; a clear's printed undo replayed and
   landing over the tombstone; the id's collision forced through the
   mint argument in two cases, one collision then a landing with the
   mint called twice, and two collisions then the refusal with the
   mint called exactly twice and never a third time; **the harmlessness
   of two rows of one name as a test**: two rows written through
   `User::put` with one name, as the race would leave them, both
   listed, `request_of(name)` refusing `ambiguous_saved_search` naming
   both ids, `request_of(id)` opening each, a third set of that name
   `name_taken` naming both with the run-by-id offers, and each offer
   the refusal prints replayed and answering; eleven rows of one name
   answering ten with `total` and `left_out`;
   `not_found` on a name and on a tombstoned id; a later version's
   row listed and named (C151); a directory that does not exist. The
   CLI, a module `saved_json.rs` in `tests/main.rs` in `price_json.rs`'s
   shape: a transcript golden over list, show, set, a rename, clear
   and each refusal, naming the facts a reader must find; `--json`
   equal to the verbs' document; `--saved` running a row equal to
   `acq search` with its text and scope; the beside-rule's refusal.
   Every test under C137: no wall-clock bound; the collision forced,
   never drawn.
6. **The gate and the commit.** `tools/gate.sh` green on the step's
   tree; one commit per concern or one for the step, the last
   naming this brief; the gate run and the push are the orchestrating
   session's, on the owner's word.

## Done when

- `crates/acquisition-store/src/user.rs` exists with `User` and
  `SavedSearch` as Do 1 states; `Annotations::open_read_only_for` is
  built over the unbound opener; `unit/tests/annotations.rs` runs its
  API table over both files, and `c167_` tests pin what the newtype
  adds.
- `crates/acquisition-verbs/src/saved.rs` exists with `list`, `show`,
  `set` (the one `Set` shape, Do 3), `clear` and `request_of`;
  `Frontend::user` opens the file; the three refusal words and the
  three `Way`s exist, every exhaustive match over `Way` compiling;
  doors and is green.
- `acq saved list|show|set|clear` and `acq search --saved` exist,
  their help in `CLI-REFERENCE.md` and their lines in the README tour,
  `cli_reference.rs` and `readme_tour.rs` green.
- The pins of Do 5 exist, each naming the behaviour it holds; the two
  collision pins count the mint's calls; the two-rows pin exists as a
  test, not a sentence, and replays the offers it checks.
- The README's three account sentences carry the saved verbs' clause,
  and its MCP paragraph says the saved verbs have no tool yet.
- The verbs crate links `getrandom` and nothing else new;
  `tools/arch-check.sh` reports no new edge.
- Every retirement in a commit message is in C154's form.
- No test asserts a wall-clock bound a slow machine can miss (C137).
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- The MCP's tools and `search.saved` (step 2); the pages, the save
  form and C168's pin (step 3); the answer page's renderer (step 4);
  cards and `--fields` (step 5): each a later step of the design's §7.
- A rename operation, a uniqueness constraint over names, a store-side
  transaction across two rows: the design's §6 declines them.
- A saved view (sort, limit, fields): the URL's; raised in the design's
  §9.
- A seed function of its own (#142's rule): the tests write rows
  through the store door or the verb, over the harness's worlds.
- Any change to the annotation file's schema or to `SyncPolicy` and
  `Buyout`: the kinds' owners.

## Rules

`TEMPLATE.md`. What differs:
- Work in the main checkout on `main`; no worktree, no branch. Commit
  locally, do not push; the orchestrating session gates, pushes and
  reviews.
- Every `acq` you run is in a mock world (the mock-session skill):
  `ACQ_PROVIDER=mock`, `ACQ_STORE_DIR` under your scratch. Never a bare
  `acq`, never a real-provider daemon: this is the owner's machine
  with his real store and keyring. Nothing in this step needs a daemon.
- A kind word, a verb's flag or a document's key this brief leaves
  open is the step's own agent-facing design, named in the report.
- A sentence of the design's §3 the tree disproves is corrected in the
  report, not silently built around.

## Report

`TEMPLATE.md`.

## Taken from the review (`57d9361c`, cut 2026-11-07)

1 a clear's undo recreates at the id with the prior whole under
`Create`, as the price undo does, replayed in a pin, `no_undo` where
the prior cannot be written (Do 3, Do 5); 2 the read-only opener keeps
its contract, verifying and never binding (Read, Do 1); 3 the store
creates with `None`, `0` is the frontend's sentinel, every `(id,
expect)` pair stated (Read, Do 3, Do 4); 4 `Way` gains the list, the
two exhaustive matches named, the kinds-and-fields pin joined (Read,
Do 3, Done when); 5 `name_taken` re-aims only at one row, offers the
rows by id for several, the offers replayed (Do 3, Do 5); 6 every
list of rows bounded with its counts, an eleven-row case (Do 3, Do 5);
7 two collision cases counting the mint's calls (Do 3, Do 5, Done
when); 8 the README's account sentences and its MCP paragraph named
(Read, Do 4, Done when); 9 `readme_tour.rs` and `no_ids.rs` described
as they are (Read, Do 4).
