# p83 — The MCP surface trimmed: what a call needs in the tool list, the rest read once by name

Pull-forward item 1 of what remains (`roadmap/README.md`,
"Pull-forward"), issue #83, with its comment of 2026-10-26. The step
is in the delegated class (p0's ruling (b): a tool's words are the
step's own, recorded in the commits and listed at the close), except
Do 1, one ruling for the owner. The seat that follows it, #84, is a
separate step under the seat skill; this brief ends where the seat
begins and says what the seat will read. About a day.

## Goal

An agent's first turn with `acq-mcp` pays for what a call needs and
nothing else. Today `initialize` and `tools/list` together are 44,199
bytes, about 11,000 tokens at four bytes a token, on every session
before any call, and 3,864 of them are one paragraph on `search`. The
words that teach — the query language, the anatomy of an answer, the
trade translation's report, the pricing model, how the job tools
behave with no daemon — are read once, on demand, through the one
door the tool already has for reading on demand: `describe` by name.
A tool's description says what the tool answers, what it costs, and
where the rest is; the schema says what an argument is; a test holds
every description to a length so the surface cannot grow back. The
agent-facing goal is the owner's first (`roadmap/STATE-2026-10-26.md`,
R3.3); the seat ledger's first row (#84) measures the result.

## Read

- #83 (the sizes, the "also found" list, and the comment: every
  result is sent twice, as text and as structured content).
  `roadmap/STATE-2026-10-26.md` R3.3 and its section B (the register
  leaks into the product). #84, for what the seat will measure.
- `roadmap/README.md`: "A step", "Pull-forward", and r5's ledger row
  (the kind list from one source, `describe`'s sections as the
  answer's keys, `refresh_plan`'s `selection`): the sentences r5 made
  true are trimmed here, never made false.
- `decisions/frontends.md`: C146 (a count says what it counts: the
  same words on every surface the count stands, "the tool's
  description" among them, the sentence Do 1 is about); C53 (three
  output levels: a JSON field's name is the contract and does not
  change); C116 (no id in user-facing text); C143 (a shared refusal
  speaks the request's words; an adapter may add its own spelling
  after them); C121 (an account that cannot be picked is refused in
  the protocol's words); C11 (the failure document). `AGENTS.md`:
  C12 (a frontend consumes two surfaces: the protocol and the
  store's read API), C122 (a behaviour pinned once, at the lowest
  crate; wording is a golden naming the facts a reader must find),
  C46 (shared semantics live in Rust), C1 (the edges: the search
  crate links no plan, the plan no search).
- `crates/acquisition-mcp/src/main.rs`: the tool attributes (one
  `description` string each), the parameter structs and their doc
  comments (the schema's text), `RequestParams` (the hand-written
  schema of `search`), `describe_names_doc` and `submit_kind_doc`
  (text built from a source, the shape a pinned list takes),
  `get_info` (the instructions, one paragraph), `told` (the text
  block and the structured content, both sent), `priced` and
  `refusal` (where a refusal's words are made), `fault_words`.
- `crates/acquisition-mcp/tests/mcp_reference.rs`: how
  `MCP-REFERENCE.md` is rendered from the live server and regenerated
  (`ACQ_UPDATE_FIXTURES=1 cargo test -p acquisition-mcp --test main
  mcp_reference`), and `the_old_word_intent_stands_only_as_a_json_name`,
  the one test that reads the whole surface for a word.
- Every test that reads a tool's text, found by grepping the MCP
  tests for `"description"` and `instructions`; each keeps its name
  and its fact words and changes only where it reads them:
  `search_tools.rs`: `describe_is_as_large_as_its_text_says` (the
  sizes `describe`'s text states, measured against the sections),
  `a_trade_fidelity_is_never_said_to_be_the_sites_items`,
  `the_never_fetched_count_says_what_it_counts_and_what_no_answer_sees`,
  `a_terms_counts_and_resolved_values_are_said_to_be_over_the_scope`,
  `show_names_a_terms_outcomes_and_a_groups`,
  `the_search_tools_words_name_tab_public_every_fidelity_and_a_sums_sources`,
  `a_rows_place_carries_when_its_location_was_fetched`,
  `the_scope_counts_the_never_fetched_by_tab_type_and_names_the_list`;
  `pricing_tools.rs`: `the_price_write_tools_say_a_clear_takes_a_revision`;
  `tool_refusals.rs`:
  `submit_jobs_kind_names_the_kinds_the_daemons_refusal_lists` (r5:
  the kind list equals the daemon's refusal, word for word; the
  sentence stays whole). No test reads the instructions' words; the
  harness only keeps them (`harness/mod.rs`, `Mcp::start`).
- `crates/acquisition-search/src/describe.rs`: `Describe`, `SECTIONS`
  (the sections by the answer's keys, pinned against the answer by
  the search crate's `tests/surface_faults.rs`), the `describe`
  function; `crates/acquisition-cli/src/search_cmd.rs`,
  `describe_text` (the CLI's rendering of the same document by block:
  a section added in the crate is rendered there or it is JSON-only).
- `crates/acquisition-plan/src/pricing.rs`: the conflict refusal
  (`Refused`, its `revision`), whose words name `--if-revision`, the
  CLI's flag, on the MCP too (the release seat's finding 9: refusals
  speak the CLI's vocabulary; C143 lets the adapter add `if_revision`
  after it).
- The seat skill (`.claude/skills/seat/SKILL.md`) and its MCP door
  `seat-mcp.py`: `open` prints the instructions and the tools whole,
  "what a host shows"; `call` prints the result's text block alone
  and logs both sizes; `launch.sh` allows the runner the door and the
  world's files and nothing else. The release seat's record,
  `audits/release-2026-10/seat/README.md` ("What this seat counted",
  item 1: the tool texts' length named as a cause of re-reads by three
  runs; `price_status`'s counts by four, with the glosses in the
  description) and `REPORT-mcp-sonnet.md` ("Tool texts": `search` "a
  very long text"; "Instructions": "the best text I got").
- `rmcp` 3.1.4 as pinned (`Cargo.lock`): the handler has resource
  methods and a resources capability; the server enables tools alone.
  Read once, to see that resources are available; Do 3 says why they
  are not the shape.
- The mock-session skill, for every `acq` or `acq-mcp` you run.

## Input

- The surface today, from HEAD's binary (`target/debug/acq-mcp`,
  built at `6750b4ef`) by a driver script over stdio (`initialize`,
  `tools/list`; bytes of the compact JSON; tokens at four bytes each):
  `tools/list` 41,652 bytes (10,400 tokens), 25 tools; descriptions
  21,844 and schemas 18,240 of it; the instructions 2,547 (640
  tokens); together 44,199 (11,000 tokens). Per tool, whole
  (description + schema): `search` 8,479 (3,864 + 4,521), `price_set`
  2,738, `price_list` 2,626, `shop_render` 2,513, `set_sync_policy`
  2,341, `price_show` 2,116, `refresh_plan` 2,081, `price_status`
  2,024, `tabs` 1,754, `daemon_status` 1,648, `apply_plan` 1,620,
  `price_clear` 1,345, `store_status` 1,275, `show` 1,274, `describe`
  1,242, `submit_job` 1,218, `reference_currency` 843, `job_result`
  832, `job_status` 782, `characters` 762, `list_jobs` 644,
  `item_events` 552, `sync_policy` 382, `cancel_job` 330, `accounts`
  195. The same text repeated: "(no daemon, no network)" 14 times,
  the account selector's sentence 15 times at some 115 bytes, and
  `account` undescribed on seven tools.
- What the seat's runners read of it: `open` was 42,082 bytes of
  text at the release seat (mcp-sonnet call 1), saved by the harness
  to a file in three runs of six; the instructions' "a search over
  several realms is refused" misled mcp-opus-2 (`realm: all` is
  accepted, call 7) while the refusal's offers taught the right call.
- The words the tests above pin, each a seat's fault: they are facts
  a reader must find, and they move with the text to the surface the
  test reads after Do 4; none is dropped.
- The pricing conflict refusal's words as the plan crate makes them,
  `--if-revision N` (the CLI's spelling), read through `priced`
  unchanged.

## Do

1. **The owner's ruling, before the glosses move.** C146 says a
   count's words stand "on every surface the count stands, the text,
   the tool's description and the field's doc". On the MCP the count
   stands in a JSON answer, its only text the tool's description, so
   `price_status`'s 1,485 bytes and `search`'s scope sentences are
   C146's glosses and the trim cannot touch them without his word.
   Bring him this, recommended text under 800 bytes: C146 amended so
   that on the MCP the gloss stands in the `describe` entry the
   tool's description names in one line ("the counts are explained by
   `describe` with names [\"pricing\"]"), read once on demand rather
   than in every session's tool list, the field's name unchanged
   (C53). The evidence for moving: with the glosses in the
   description, the release seat still named `price_status`'s counts
   a cause of re-reads in four runs of six; the description did not
   prevent what C146 was written against, and the seat after this
   step measures whether the pointer does. If he keeps C146 as it
   stands, the count glosses stay in `price_status`, `search`,
   `store_status` and `tabs`, at their shortest, and the length test
   of Do 6 names those four with the bound they are held to instead.
   Build Do 2 to 5 on the sentences that are not count glosses while
   the answer is pending; the glosses move last, by his answer.
2. **The rule that decides each sentence.** A sentence stays in a
   tool's description only if an agent that has not read it would
   send a wrong call or misread a right answer and nothing the tool
   sends would tell it: no refusal, no offer, no key's own value. A
   rule the tool enforces is taught by its refusal: where the refusal
   says the remedy in the MCP's spelling the sentence goes; where it
   speaks a flag, `priced` adds the argument's name after it as
   `failed` does (C143), one function, then the sentence goes. What
   explains an answer's keys moves to the on-demand text of Do 4.
   What names another tool as the next step stays, in a clause. What
   a parameter's doc already says is not said again in the
   description. Prefer removing a sentence to rewriting it: the
   words that remain are today's words, cut, so the goldens' diffs
   read as deletions.
3. **The shape: `describe` by name, one door for everything read on
   demand.** Not MCP resources, though `rmcp` 3.1.4 has them, for
   three reasons recorded in the commit: a host decides whether a
   model may read a resource at all, where a tool call reaches every
   host; the seat's runner reaches the server only through the door,
   which has `open` and `call` and no resource read, so a seat could
   not measure what resources taught; and the language is already
   behind `describe`, so a resource would be a second door to the
   same text, not one. The tool's name and its `names` argument stay;
   its scope widens from "the query language" to "what the tools
   answer, by name": the search crate's sections as today, and two
   entries that are not the search's, `pricing` (the listing model:
   the hand row and what inherits it, the game's reading, relation
   and effective, the revision rule and the undo, the counts
   `price_status` reports) from a function in the plan crate's
   `pricing` module, and `daemon` (what the job tools answer with no
   daemon running, the ledger on disk, a daemon of another world or
   contract, a wedged one) as text of the MCP's own, since the
   document is the verbs crate's and the words are this adapter's.
   The MCP's `describe` dispatches by name over the three sources and
   its refusal of an unknown name lists every name it knows, as
   today; `describe_names_doc` lists the union. `describe` with no
   names stays the whole.
4. **The search crate's `answer` section.** The second half of
   `search`'s paragraph is the answer's anatomy — `query`, `scope`
   and what each of its counts counts (the never-fetched words, by
   type and by league), `rows` and a row's `place`, `view` and
   `view.rows.next`, `terms` over the scope and a pattern term's
   `resolved`, `basis`, `zero.offers`, which counts carry requests and
   `view.routes` — and belongs beside the language, in the search
   crate, as a section `answer` of `Describe` and `SECTIONS`, rendered
   by the CLI's `describe_text` too, pinned at the crate as the other
   sections are. The trade translation's report (`fidelity` and its
   three values, `other`, `remainder`, "never how far the items are
   the site's", `tab.public`) opens the `trade` section, before the
   catalogue's rows. The eight `search_tools.rs` tests named under
   Read then read `describe` with names [`answer`] or [`trade`] in
   place of the tool text, their names and fact words unchanged;
   `the_price_write_tools_say_a_clear_takes_a_revision` reads
   `pricing` the same way. A `search` call's `query`, `scope` and
   `view` docs keep the shape and the default in one sentence each
   and name the section for the rest.
5. **The instructions as a short list**, under 1,000 bytes: the
   provider; the three groups and what each costs (the store tools
   read the store, no daemon; the pricing tools write locally and
   send nothing; the job tools talk to the daemon a human started,
   never spawned in real mode); where to begin (`search` with no
   arguments; `scope.realm` where the store holds several, the
   refusal offers each); where the language and the answers are
   explained (`describe` with names, `fields` first, the whole over
   100 KB); a refusal is a result marked `isError` with `readings` and
   `offers`, and a request an answer holds is sent back as printed;
   no tool says what an item is worth. The nesting limit goes (the
   refusal names it); "a search over several realms is refused" goes
   (it misled, and the refusal teaches). A tool's own description
   keeps its cost in two words, since a host may show descriptions
   without the instructions.
6. **The length, pinned.** One test in the MCP crate over
   `tools/list` and the instructions: every tool description at most
   600 bytes, every parameter description at most 300, the
   instructions at most 1,000; the failure names the tool, the
   bytes and the bound. The `account` doc is one short sentence on
   every tool that takes it (the seven undescribed included); the
   `realm` doc one sentence; the kind list of `submit_job` stays
   whole, as the daemon's refusal lists it (r5).
7. **The second copy, weighed and measured.** `told` sends the
   document as the text block and as structured content. Measure one
   `search` and one `describe` with names [`trade`]: the text's bytes
   and the whole result's, in the commit message. Keep both unless
   the measurement says otherwise, and say why in the message: the
   seat's door and a host read the text block (`told`'s own doc says
   an agent reads the text), thirteen places in the tests read the
   structured content, and the specification asks for the text beside it. The
   measurement is the result's bytes, as the issue's comment asks,
   never the document's.
8. **The measurement, before and after**, by one driver script under
   your scratch over stdio, HEAD's binary before the first edit and
   the step's last binary after, alone on the machine: `tools/list`
   bytes and the instructions' bytes, per tool (description, schema,
   whole) and whole, tokens at four bytes each (a tokenizer's count
   beside it if you have one, said as such). The two tables go in the
   last build commit's message with the first-turn cost stated in
   one line: what an agent paid before any call, and what it pays
   now. The brief expects the whole under 20 KB; the build measures
   it.
9. **The references and the goldens.** `MCP-REFERENCE.md` regenerated
   in the commit that changes its source, every changed line read as
   an agent would; `the_old_word_intent_stands_only_as_a_json_name`
   extended to read the on-demand text too, since the rule is every
   word the server sends; `describe_is_as_large_as_its_text_says`
   holds the sizes the trimmed `describe` text states, remeasured.
10. One commit per concern: the answer section and the trade preamble
   in the search crate (with the CLI's rendering and the re-pointed
   tests); the pricing and daemon entries and `describe`'s dispatch;
   the descriptions and the instructions cut, with the length test
   and the reference; the measurement in the last message. Each
   agent-facing decision (the section names, the instructions' lines,
   what each description keeps) recorded in its commit and listed in
   the report for the close.

## Done when

- A test in `crates/acquisition-mcp/tests/` holds every tool
  description to 600 bytes, every parameter description to 300 and
  the instructions to 1,000, each failure naming the tool, the bytes
  and the bound; where Do 1 keeps C146 as it stands, the test names
  the four count-carrying tools and the bound they are held to.
- The last build commit's message carries the before and after tables
  of Do 8 (per tool and whole, bytes and approximate tokens, the
  instructions beside them) and one line on the first turn's cost.
- `describe` with names [`answer`], [`pricing`] and [`daemon`] each
  answer; an unknown name is refused listing every name; `SECTIONS`
  holds `answer`, pinned in the search crate; `acq search --describe
  answer` renders it.
- Every fact word a test under Read pins stands on the surface the
  test now reads; no test was deleted and none renamed.
- `MCP-REFERENCE.md` regenerated and committed with the source that
  changed; `CLI-REFERENCE.md` unchanged unless a shared sentence's
  source moved, each line explained.
- `account` is described on every tool that takes it, in one shared
  sentence.
- The pricing conflict refusal through the MCP names `if_revision`
  after the flag, pinned in `pricing_tools.rs`.
- The second copy's measurement and the decision are in a commit
  message.
- The owner's answer to Do 1 is recorded in the commit that moves or
  keeps the glosses, quoted.
- The gate green on the step's own tree (`tools/gate.sh`'s exit
  status), the report carrying the stage table.

## The seat after this step

#84's brief is the seat skill's, written by the session that starts
it from the release seat's `BRIEF-mcp.md` (`git show
10902dbb:audits/release-2026-10/seat/BRIEF-mcp.md`). What this step
leaves for it: the runner reads `open` (the trimmed instructions and
tools, what a host shows) and the README's tour, nothing else, and
the tool's own words must lead it to `describe` by name — the seat's
brief names no section, or the trim is not measured. The ledger's
"calls to first correct answer per task" compares against the release
seat's MCP runs (`calls-mcp-opus-2.csv` and its two siblings; the
README's per-task counts), and `open`'s bytes against 42,082. The
seat's door is unchanged by this step; a door that reads resources
is not needed because none exist.

## Not in scope

- #10, one refusal and receipt shape (a daemon refusal as a JSON-RPC
  error against the instructions' promise): the shape stays; only the
  spelling after a flag changes here (Do 2).
- The MCP's `selection` semantics (r5) and the plan tools' behaviour:
  their sentences are cut, their meaning is not.
- New tools (a priority setter, an events summary), a tool renamed
  (`characters` beside the job kind; an issue if the seat stops on
  it), an output schema on any tool (it would ride in `tools/list`
  and cost what the descriptions cost).
- An index shape for `describe` with no names, as the CLI's bare
  `--describe` prints one (47 KB against the whole's 100 KB): the
  CLI renders its index in `search_cmd.rs` from the whole document,
  so the MCP would need a document shape the crate does not have; an
  issue if the seat asks for it.
- MCP resources, for Do 3's reasons; the dependency is not added
  and no capability beyond tools is enabled.
- The CLI's help texts, except a rendering of the new section.

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree; scratch under your session's scratchpad in a
subdirectory named `p83`, every `ACQ_STORE_DIR` under it; commit on
`main`, never push; the `Co-Authored-By` line for your model last;
long commands as tracked background tasks; `tools/gate.sh`, and
`cargo build --workspace` before any daemon and after any test run
(C84, the artifact mismatch); a measurement runs alone on the machine,
with no gate, build or other driver beside it, and the before-run is
taken from HEAD's binary before the first edit; a fix round's
done-when is the template's. Regenerate a golden only with
`ACQ_UPDATE_FIXTURES=1` on the one test named, in a shell where the
variable is otherwise unset.

## Report

`TEMPLATE.md`, with the per-tool tables of Do 8 under measurements
and the owner's answer to Do 1 quoted under what landed.
