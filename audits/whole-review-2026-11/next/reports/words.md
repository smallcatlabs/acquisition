# Investigator's report: the words below the doors

As returned by an Opus investigator on 2026-11-25, read-only, no cargo and no `acq`. It was handed a shape to break, called (B): keep documents as data; move the functions that turn a document into words out of the CLI's binary crate into a library below the frontends, each taking the document and a door's speller; make every remedy a `Way`; let the web import the words from the library and become its own crate; let the MCP serve the same decision-view text. Its claims are its own.

## Verdict up front

(B) holds for the terminal and the MCP: both are plain text, so a word function that takes `&dyn Surface` can spell a way on as backticked text for either one. (B) does not work for the web, and it quietly contains half of the audit's design. A remedy that `--json` and the MCP are to carry has to sit in the **document** as an offer, not inside a word function. So (B) is really `Doc{body, ways}` with no `says`. That is a sound middle shape, and the repo already has it twice: refusal `offers` and receipt `undo`. But it is not "only moving functions". Three of the steps need the owner's ruling: C53 (MCP text), C139 (web crate) and C1 (a new crate edge).

## 1. How each file makes its text

Sampling method: an awk pass over the top-level fns before `#[cfg(test)]`. A fn counts as IO if its signature names `Ctx`, `Client`, `async` or `Store`. It counts as mixed if it prints, or is IO with more than 2 prose literals. Everything else counts as pure. This is a heuristic. Then read: search_cmd 1223–1726, store_cmd 280–440 and 555–600, price_cmd 395–410 and 1060–1200, and plan_cmd 1605–1630.

| file | orch/IO | pure | mixed | notes |
|---|---|---|---|---|
| search_cmd | 47 | 2546 | 376 | The "mixed" count is really orchestration: `search()` at 352–660 and `fail`. Words read the clock at :1108, :3006 and :3011 (easy to pass `now` in). |
| price_cmd | 64 | 1089 | 92 | `render_*(view) -> String` is pure, but it calls `Terminal::writing()` directly (:1177). |
| plan_cmd | 187 | 1397 | 213 | `apply_plan` (:471, async) prints progress as it goes. |
| store_cmd | 223 | 541 | 412 | **The worst file.** `tabs` prints line by line and builds commands with `push_str` (:405–418). `no_tabs_text(…, store: &Store, …)` (:157) and `no_characters` (:575) **query the store** from inside the words. |
| jobs_cmd | 168 | 389 | 137 | The watch loop depends on the tty and time (`is_terminal`, `Instant`). It stays in the CLI. |
| shop / saved / daemon / leagues | 82/90/10/9 | 303/221/218/146 | 0/14/0/0 | Clean. |

That comes to about 6.8k pure lines and about 1.2k mixed. Only store's mixed lines are real untangling: its words need fields the document lacks, which is already a C53 breach ("text is a function of JSON").

## 2. What `web/` takes from the CLI

From `git grep -ohE '(search_cmd|plan_cmd|price_cmd|store_cmd|jobs_cmd|shop_cmd|saved_cmd|daemon_cmd|leagues_cmd|terminal|printed|run)::[A-Za-z_]+' -- crates/acquisition-cli/src/web | sort | uniq -c | sort -rn`:

- `search_cmd`: 80 distinct symbols, 99 uses.
- `store_cmd`: 6 (`ago`, `items_cell`, `tabs_head`, `tabs_facts_line`, `no_characters_cause`, `characters_counts_line`).
- `saved_cmd`: 2.
- `terminal`: 3 (`plural`, `fault_words`, `failure_words_by`).
- `run`: 2 (`Ctx`, `connect`).
- `crate::auth_cmd::open_browser` (web/mod.rs:622).

There are no imports from `plan_cmd` or `price_cmd`. The web words plans itself instead: `page.rs:4477 fetch_words` and `plan_cmd.rs:1610 fetch_why` already differ ("ticked" against "selected", and other wording).

Once the words move, these ties remain:

- `Ctx`, which is only `{frontend, account}` (run.rs:38).
- `run::connect`, which sets the CLI's `SPAWNED` static (main.rs:45).
- `open_browser`, 12 lines.
- `fault_words`.

None of these involves clap. The real block is the rulings:

- **C139** says "the server, its pages and the opener live in `acquisition-cli`" and that the web "links nothing the CLI does not". Its reasons (distribution, the human door) survive a library crate that `acq web` calls, but its text has to be amended.
- **C1** lists the edges, and `tools/arch-check.sh:480–489` enforces them. A new crate means a new edge.

## 3. Printed commands

`printed.rs` (242 lines) spells search ways (`Next`, `Route`, `Request`, `TabsPointer`) through its own `Printed` trait, **not** through `Surface`/`Way`. The grep `git grep -n '"[^"]*acq [a-z]' -- crates/*/src` finds 152 lines, 119 of them outside `#[cfg(test)]`: price 19, store 15, terminal 9, daemon 9, shop 6, plan 6, jobs 6, then the daemon, client and MCP crates.

A sample of 15:

| site | existing `Way`? |
|---|---|
| price:247 `price clear … --if-revision` | yes, `PriceClear` |
| jobs:362 `acq result {id} --json` | yes, `Result` |
| price:406/562 `price status … --expand` | no: `PriceStatus` has no `expand` |
| price:474/480/488/557/560 `price list --game-text/--effective/--relation` | new `PriceList` with filters |
| price:769 `--limit N`, price:854 `item/<id>` | a **way with a hole**; `Way` models only complete requests |
| shop:428 `price set item/<id> exact <amount> <tag>` | `PriceSet` with three holes |
| shop:198 and plan:1008 bare `acq policy set` | a pointer to a verb; `PolicySet` requires a value and a revision |
| shop:418 `shop render --page N` | new variant plus a hole |
| store:129 `accounts --expand` | new |
| store:878 `store refused … --limit` | `RefusedBodies` has no limit |
| store:1090 `store events …` | new |
| store:556 `acq characters`, leagues:117 `acq leagues` | new (or `Submit`) |
| daemon:166 `daemon reset-tripwire` | new, and it is a person's act (`PERSONS`, mcp harness/mod.rs:353) |

So about 2 of the 15 fit as they stand. The rest need about 10 new variants, a hole concept, and a "this request with X" modifier (`--expand`/`--limit` in price:854 and 1065). Each new variant also costs an exhaustive arm in each speller: terminal.rs (24 `Way::` uses today), told.rs (20) and web (21).

Interleaving: sentences like price:1065 put two commands and a bare flag in one sentence. A function returning a `String` handles that for the CLI and the MCP (`{spelled}` in backticks). **It cannot do so for the web.** A link needs HTML, so a `String` with an embedded way forces the word function to know about escaping, or to return segments. Returning segments is the audit's `Said` with a `{call}` slot. Today the code avoids this by sharing only command-free fragments (`rows_shown_words` and the like) and assembling with commands per door (`answer_tail`, :1766–1990, is CLI only). Inference: keep that split.

## 4. MCP

`told.rs:463–488`: the text block alone, measured. When structured content was sent, Claude Code passed only the structured copy, so both forms cannot be sent. C53 (frontends.md:11) is explicit: on the MCP "the JSON is the text… no second rendering stands between the reader and the contract." **So MCP text is a C53 ruling.**

What pins JSON-as-text:

- `Mcp::call` in tests/harness/mod.rs:291 parses every result as JSON or panics. Every MCP test goes through it (search_tools alone has 55 calls).
- `c149_every_answer_the_step_wrapped_is_one_object` (search_tools:1469).
- `the_answers_text_leads_with_its_total…` (:946) reads key order.
- `c143_a_text_speaks_no_frontend` (harness:373). This one helps (B): it would catch any CLI spelling that leaks through.
- A per-tool `format` parameter runs into `surface_size.rs` (FIRST_TURN 43,000 against 42,089 measured).

What an agent would lose under text by default:

- `offers[].request` objects it sends back verbatim.
- Continuation tokens, unless the text embeds them whole.
- Item ids for `show_item` and `price_set`.
- The field paths that `sort` and `value` take.
- Epochs.

The MCP speller (`Spoken::request`, told.rs:212) returns JSON objects that depend on the tool being served (`CALLED`). It would need a text form added.

## 5. Tests and duplication

- **Moves:** about 37 `#[cfg(test)]` unit tests (search 3, price 9, plan 11, store 4, jobs 6, shop 2, daemon 2).
- **Stays and does not change:** the goldens. They are process-level (`tests/goldens/`: seat_faults 33, web 14, search_json 10, …) and output stays the same. MCP text would **add** goldens.
- **Ends:**
  - fault remedy prose: terminal.rs:78–150 against told.rs:35–110 plus the web's two overrides;
  - the gloss pairs, if `describe.rs` reads the moved consts (search_cmd:2146 against search describe.rs:291);
  - FetchReason, but only as a wording decision with goldens on two doors.
- **Does not end:** the per-door `Way` spellers. That is correct, but they grow.

What `Doc{says}` gives that (B) does not:

- Sentences reach a JSON reader, which matters for a non-Rust GUI. C46 makes every frontend a Rust adapter, so this mostly does not matter.
- The web can place a sentence by key. Under (B) the web stays its own assembler.

Tests: a recording `Surface` can collect every way a text spells, which gives `Doc{ways}`'s "every way parses" property. That is an inference.

## 6. Slices

1. **Smallest:** `price status` document gains `offers` with the existing `Way::RefreshPlan`, and the text reads it. Additive JSON, and the MCP gains the remedy. Small judgment.
2. Move `plural`, `ago` and `span` down a crate. Mechanical.
3. Move the 80 command-free search fragments into the search crate. No C1 change; thread `now`. Mechanical.
4. Fault remedies become `Way`s. Needs design: person's acts and new variants.
5. Per verb: hand-spelled remedies become `Way`s in the document, plus holes and the modifier. Needs design, and is large: about 119 sites.
6. Per verb: move the pure renderers with a speller. Mechanical for daemon, leagues, saved and shop; store first needs document fields.
7. Web crate. Needs the C139 and C1 rulings; then mechanical.
8. MCP text. Needs the C53 ruling, a harness rewrite and a check on agent capability. **The riskiest slice.**

Where (B) is too optimistic:

- Slice 5 is the size of the audit's design: new variants, holes, and three spellers each.
- Store's words read the store.
- The web cannot use `String` + `Surface` for anything that carries a link.
- The MCP text contradicts a ruling and a measured client behaviour.
