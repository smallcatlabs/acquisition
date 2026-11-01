# describe-slices — the index on both surfaces, then a slice by pattern (#153, #154)

Written 2026-11-01 by the orchestrating session, from #153 (filed by
the league seat, `audits/league-2026-11/seat/`), #154 (the owner's,
with his comment on derived groups and the seat session's comment
with its measurements and two settlements) and the tree at `fdbab0ce`.
Cut four times: the first cut (`4f879174`) was reviewed cold
(`roadmap/reviews/describe-slices-brief-4f879174.md`, the prompt
beside) and the second (`9dfa89f5`) answered its ten findings; the
second was reviewed cold too (`describe-slices-brief-9dfa89f5.md`)
and the third (`38073d13`) answered its five; the third's confirming
review (`describe-slices-brief-38073d13.md`) found one test sentence
contradicting settled 6, corrected in this fourth cut. Each is named
at the foot, "Taken from the reviews". One step, two issues in order, both
surfaces (`acq search --describe` and the MCP's `describe`): how the
search language discloses itself to an agent, progressively — the
index, a slice the size of a question, an entry whole.

## Goal

When this step is done, `describe` takes the same list on both
surfaces and answers the same document for it: nothing named is the
index (every entry's name, kind and one line; a name and a line, the
examples under the entry), a name is the entries whole as today, `all`
is everything whole, and `~pattern` is a slice: the entries whose name
or definition the pattern matches, whole when they are ten or fewer,
their index lines otherwise, counted; a pattern that matches nothing is
refused in words that say so and offer the index. The index and a
pattern's slice are density-controlled (one line an entry, the whole
only under the store's count), measured on both surfaces against the
seat door's 60,000-byte cut for the cases the seat named; a named block
stays whole and an ordinary unknown name keeps its word list, as today.
The whole's content is unchanged under `all`, the step's keys standing
only where they say something. The MCP's `describe` calls the search
crate's function as the CLI does (C46, shared semantics in Rust), and
the one-line rule and the pattern's reading live there once.

## Read

- `AGENTS.md`: the hard rules (every `acq` in a mock world; C116, no
  id in user-facing text; C122, one golden per wording, regenerated
  under `ACQ_UPDATE_FIXTURES=1`; C153, a finding joins the surface that
  holds it); C46 (shared semantics live in Rust, every frontend an
  adapter); C53 (three output levels; on the MCP the JSON is the text
  the reader gets and the decision view is the document's default
  shape; a `describe` section is never cut; JSON changes are
  additive; and, amended 2026-11-01 for this step, a document read at
  two densities is the decision view where nothing is named, on every
  surface and in every format, and the whole by name); C97 (the closed
  lists are printed in the help); C143 (a shared refusal speaks the
  request's own words); C156 (a printed command is quoted for the
  shell it is pasted into, `argv` beside it in JSON).
- `decisions/search.md` (C97, C101, C102) and `decisions/frontends.md`
  (C11, C53 as amended, C143, C146, C149); `REVIEW-CHECKLIST.md`,
  "Search" and "Frontends and the surface an agent meets" (a document
  unbounded by default; a printed command that does not run as
  printed; a sibling sentence left standing; a fixture regenerated
  blind).
- `search/README.md`, the crate's working directory, its rules in
  force.
- `crates/acquisition-search/src/describe.rs`, whole: the module doc
  ("every word an entry is named by can be asked for alone"),
  `Describe` and `SECTIONS` (the sixteen keys; `pseudo` names the
  computed block too), `Named` (`name`, `kind`, `what`, `values`,
  `examples`; the last two skipped when empty, line 99),
  `describe(names)` (line 304: the whole built, then narrowed by
  `answers` — an entry answers to its name, each word of it, a
  computed value's name without `pseudo.`, a call's name without its
  `(`; the known list at 730 holds the eleven entry sections and the
  blocks, no category id; the categories survive only where `category`
  or `categories` is named, 798 — and an unknown name refused as
  `unknown_name` listing every word, 772, with `near` readings),
  `trade_catalogue()`, `answer_keys()`, `LIMITS`; it returns
  `Result<Describe, LanguageError>`.
- `crates/acquisition-search/src/category.rs:1108` `Described` (`id`,
  `text`, `holds`); `crates/acquisition-search/src/error.rs`:
  `LanguageError` (`kind`, `message`, `span`, `readings`), `SearchError::Scope`
  (`kind: &'static str`, `message`, `offers: Vec<Offered>`), `Offered`
  (162: `reading`, an optional search `Request`, a realm) and
  `offering_realms` (210), the sibling: a refusal offering each realm
  as a request; `crates/acquisition-verbs/src/refusal.rs`: `Way` (50–)
  and `Surface::request` (the CLI's at `acquisition-cli/src/main.rs:1210`,
  the MCP's at `acquisition-mcp/src/main.rs:327`), the one maker of
  every printed request; `of_search` (222), `searched` (299: every
  scope offer becomes a reading, and only a search request becomes a
  `Way`) and `offered` (231).
- `crates/acquisition-cli/src/search_cmd.rs`: the `describe` arg and
  its help (lines 144–157: "about 47 KB", "`--describe all` is every
  entry whole, over 150 KB", "`--json` is whole unless entries are
  named"); the dispatch (407–419: bare is `index`, `all` is
  `describe(&[])`, the CLI resolving `all` itself); `definition_head`
  (1783: the text before the first `: `, the one-line rule) and
  `describe_text` (1791–1963: the index renders the computed values and
  the categories one line each, counts the answer's keys and the trade
  rows, and ends on the `more:` line); `crates/acquisition-cli/src/web/form.rs:144`,
  the web UI's one reader of `describe` (`["trade"]`, the catalogue
  for its form), which this step leaves as it is.
- `crates/acquisition-mcp/src/main.rs`: `OWN_SECTIONS` and
  `own_sections()` (732–766: pricing, daemon, store, each entry a
  `name` and a `what`), `Described` (768–783, the language flattened
  beside the own sections), `DescribeParams` and `describe_names_doc`
  (786–799: "Omitted: everything, over 150 KB"), the `describe` tool
  (1557–1619: its description's sizes, "An unknown name is refused,
  listing every name it knows"; an own name answered by the MCP, the
  rest handed to `acquisition_search::describe`), the instructions'
  describe line (2083–2084), `told_as` and `Prose { all: true }`
  (499–520, 1233–1250: every string of `describe` is spelled for the
  MCP, the CLI's flags given this server's spelling after them, except
  the keys `PAYLOAD` names, 1226, which a spelling never touches).
- `crates/acquisition-store/src/bound.rs`: `LISTED` (16), the one
  bound every maker applies (C53) — a count of like entities, never a
  byte size — and `bounded`.
- `crates/acquisition-search/src/bind.rs` 1380–1404: the one compiler
  of a `~` pattern (`RegexBuilder`, case-insensitive) and the
  `bad_pattern` words, "`{name}~` takes a pattern in Rust regex syntax,
  and this is not one: {e}"; the `~` entry of `describe` ("Rust regex
  syntax, unanchored, any case unless it says (?-i)").
- The tests that assert on `describe`'s shape or words today, each
  found by `git grep -n describe crates/*/tests` and read before an
  edit:
  - search crate: `tests/surface_faults.rs`
    `c97_describe_says_how_terms_compose_and_knows_the_words_it_prints`
    (192: reads `describe(&[])` as the whole at 193 and checks every
    example parses, flattening absent examples at 216 — under the
    index it would check nothing; a call's `(` form),
    `c97_the_sections_listed_are_the_answers_keys_and_each_answers_alone`
    (555: the whole's keys equal `SECTIONS`; a narrowed answer keeps
    `totals`), `the_answer_section_names_every_key_an_answer_prints`
    (592); `describe(&[])` read as the whole in `acceptance.rs:331`,
    `category.rs:1013`, `counts.rs:865`, `explain.rs:702`,
    `percentile.rs:1404`, `price.rs:593`, `pseudo.rs:664`,
    `ranged.rs:1073`, `refusal.rs:103` and `:125`.
  - CLI: `tests/seat_faults.rs`
    `describe_is_an_index_that_names_every_entry_and_says_how_to_read_one`
    (498: the release seat's pin — the index under 50,000 bytes, a
    computed line, a category line, the `more:` line naming
    `--describe all`; an entry, `all` and bare `--json` whole, 528);
    `tests/search_json.rs` `describe_and_show_print_json_whole_and_text_from_it`
    (1193: bare `--json`'s `fields[].name` in order; `league,line`
    narrowed), the trade line of the index (2770), `--describe clas`
    refused `unknown_name` (856), three `--describe` beside a view
    refused `view` (879–881), named entries read whole (1500, 1611,
    1738, 1798, 2750); `tests/no_ids.rs` (229: `describe` whole and
    `trade,category` swept for ids; keep it sweeping the whole);
    `tests/readme_tour.rs` (the tour line resolves under clap: the
    verb and its flags, never the argument's value);
    `tests/cli_reference.rs` (`CLI-REFERENCE.md` equals the help);
    `tests/web.rs` (the form's page, which reads the catalogue).
  - MCP: `tests/search_tools.rs` `describe_is_the_clis_and_reads_no_store`
    (371: `describe {}` spoken as the CLI's bare `--describe` after
    its own sections are removed, two names as the CLI's, `leauge`
    refused listing every name, the own names after the language's;
    `assert_spoken_as`, `tests/harness/mod.rs:368`, equates keys,
    array lengths and numbers); `tests/spoken.rs` (208, 215:
    `describe {}` and `nonsense` in the spelling sweep);
    `tests/surface_size.rs` (155–180: `describe` by section);
    `tests/pricing_tools.rs` (795, 814); `tests/mcp_reference.rs`
    (109: `describe {}` appended to the intent sweep's text as "the
    whole, every section the tools point to" — after this step that
    sweep asks `all`, so it keeps sweeping the whole);
    `MCP-REFERENCE.md` (the tool's description and `names` doc).
- `README.md` line 191, the tour line
  (`acq search --describe [name,…|all]   # the query language: an index, or entries whole`)
  and 124 (`describe` among the MCP's tools); `CLI-REFERENCE.md` 212.

## Input

The last push's CI on `main` at `fdbab0ce`: `Gate (Linux)` run
41321381891 and `Gate (Windows)` run 41321381888, both success (`gh run
list --branch main --workflow … --limit 1`, 2026-11-01).

**The ruling.** The one question the first cut brought to the owner —
whether nothing named is the index on both surfaces and in both
formats — he ruled on 2026-11-01: "Yes, the index." Recorded before
this cut as C53's amendment in `decisions/frontends.md` (the commit
before this one): bare `describe` is the index in text and in JSON on
the CLI and on the MCP, `all` everything whole, the additive clause
read as protecting shapes a program can use, the whole reachable by
name. The build rests on it; nothing else in this brief is a ruling.

**The world.** Every measurement in this brief and in the step runs in
a mock world over a scratch copy of the league seat's pristine
(`~/seat-worlds/league-2026-11/pristine/store/mock`, 22,721 live items,
one account `GERWARIC#7694`; copied with `cp -R` into the session's
scratch; `ACQ_PROVIDER=mock`, `ACQ_NO_KEYRING=1`, `ACQ_NO_SPAWN=1`,
`ACQ_STORE_DIR` and `ACQ_LOG_DIR` under the scratch), on a debug build
(`cargo build --workspace`): `describe` reads no store and a byte count
does not depend on the profile. Never the owner's store, never a
real-mode daemon. The MCP is driven over stdio by a twelve-line script
in the session's scratch (`initialize`, `notifications/initialized`,
one `tools/call`; the size is `content[0].text`'s bytes); the CLI by
`./target/debug/acq`, the text's bytes by `wc -c`.

**Before, at `fdbab0ce`** (the seat session's numbers on #154
reproduced to the byte):

| Asked | CLI text | CLI `--json` | MCP (compact JSON) |
| --- | --- | --- | --- |
| nothing named (the CLI's index; the MCP's whole) | 49,833 (398 lines) | 182,918 | 169,215 |
| `all` | 171,824 | 182,918 | refused, `unknown_name` (4,592) |
| `computed` | 86,354 | 90,702 | 83,460 |
| `pseudo.count_res,pseudo.total_life` | 1,436 | 1,812 | 1,599 |
| `~resist` | refused, `unknown_name`, listing every word (4,078) | — | refused, the same (4,613) |

The index's lines holding a word (`grep -ci` over the CLI's index
text, the seat session's measure): `resist` 14 lines, 1,967 bytes;
`life` 16, 2,702; `weapon` 28, 2,749. Of `resist`'s 14, four are
example lines (`e.g. …`) and ten are entries.

**What a pattern matches, by rule** (a Python read of the whole
`--json` document, case-insensitive, over the eleven entry sections —
fields, line, linked, values, composition, operators, slots, computed,
counts, explain, answer — the categories, and the MCP's own entries):

| Pattern | name + one line (the text before the first `: `) | name + definition whole | categories, id + text / + holds | MCP own, name + what |
| --- | --- | --- | --- | --- |
| `resist` | 10 (10,123 bytes whole, compact JSON) | 11 | 0 / 0 | 0 |
| `life` | 4 (2,233) | 5 | 0 / 1 | 0 |
| `weapon` | 1 (230) | 24 | 25 / 26 | 0 |
| `a` | 199 | 204 | 75 / 82 | 23 |

`weapon` is the case that decides the reading: the weapon fields and
the `adds_*` computed values say "weapon" after their first `: `
(`critical_strike_chance`: "the displayed `Critical Strike Chance`, in
percent: a weapon's, and a spell gem's, …"; `pseudo.adds_physical`'s
"where -category=weapon" is deep in its definition), so a pattern over
the one line alone finds one of the twenty-four. `resist` by definition
is eleven: the ten plus `answer`'s `view`, whose definition names
`line:resist`. The one line is what the index prints; the definition is
what a pattern reads (settled 3).

**The whole, by section** (compact JSON bytes of the bare `--json`
document; "heads" is the sum of each entry's text before its first
`: `): fields 73 entries, 21,914, heads 5,173; computed 92, 85,699,
heads 8,059; answer 15, 5,364, heads 1,957; categories 82, 11,067, id
and text alone 1,987; trade 149 rows, 25,059; counts 4, 2,594, heads
150; explain 3, 1,604, heads 414; line 3, 1,229, heads 353;
composition 7, 1,269, heads 504; linked, values, operators, slots each
under 800; not_built 3, 269; limits 6, 816; `totals` 403. The eleven
entry sections' names, kinds and heads together: 22,131 bytes. The
MCP's own: pricing 10 entries, 7,256, heads 1,442; daemon 8, 3,933,
heads 857; store 5, 4,011, heads 408. So the index after this step is
near 30 KB of text and near 35 KB of compact JSON (an estimate; the
build measures it), the computed block's lines near a third of it.

**The shell trap.** A pattern at the start of a word is a tilde
expansion: `zsh -c 'echo ~resist'` fails with "no such user or named
directory: resist" before `acq` runs; bash leaves it. Quoted
(`'~resist'`) or after `=` (`--describe=~resist`) both shells pass it
whole. Every printed or documented `--describe` pattern is quoted, as
a query starting with `-` is (`CLI-REFERENCE.md` 200), and a printed
command's `argv` stands beside it in JSON (C156). The MCP has no
shell.

**Fixed inputs the tests stage:** a pattern matching exactly ten
entries and one matching eleven (the bound seen from both sides; the
worker finds two such patterns over the shipped tables and names them,
or stages the count another way the report names); `~a` (the wide
pattern, lines only); a pattern matching nothing (`~qqqqq`); a pattern
that does not compile (`~(`); a pattern beside a name (`league,~life`);
`all`; a category line asked by its id (`accessory.ring`); a pattern
holding a CLI flag's spelling (`~--count`), echoed unchanged by the
MCP; a pattern holding a comma (`~^.{1,20}$`) on both surfaces; the
bare `~`; a not-built construct asked by name; `web/form.rs`'s
`describe(["trade"])` unchanged.

## Settled in this brief, under the owner's delegation

Agent-facing (a flag's words, a tool's shape, the words of an answer),
recorded here and in the commits, listed at the close for his read;
the one ruling is Input's.

1. **One list, one document, both surfaces.** `describe` takes the
   list as typed and resolves it in the search crate: empty is the
   index, `all` is the whole, a name is its entries whole, `~pattern`
   a slice. Each frontend passes its list through and resolves nothing
   itself (today the CLI resolves `all` and `index` on its own:
   `search_cmd.rs` 407–419, two places for one rule). The CLI's list
   is read as `--count`'s is (`search_cmd.rs:172`, no clap delimiter,
   split in code): commas part the names, and an item starting `~`
   takes the rest of that `--describe` value whole, so a pattern's own
   commas (`'~^.{1,20}$'`) reach the search crate intact; a name after
   a pattern is given by `--describe` again, which adds a list of its
   own. The MCP's `names` is a JSON array, one pattern an item.
2. **The one-line rule moves into the search crate**: an entry's line
   is its definition up to its first `: ` (`definition_head`, the rule
   the CLI's index applies to a computed value today), its name and
   its kind; no values, no examples; a category's id and site text
   without `holds`; the answer's keys one line each (no longer a
   count); the trade catalogue counted by rows and named; not_built
   and limits as today, one line each. The index says how to go on in
   words that name entries and lists, never a frontend's flag (C143;
   the MCP spells flags after them). Every entry line the index
   prints is askable by its name — the eleven sections' entries, the
   section names, and the categories: a category's id becomes a name
   describe knows (exact, case-insensitive), answering its row whole,
   which the resolver does not do today (`describe.rs` 730, 798). A
   not-built construct's line stays what it is: asked by name it is
   refused `not_built`, as today (`bind.rs:138`, `describe.rs:754`),
   and a limit's line has no name.
3. **A pattern reads the language's entries**: an entry's name and
   definition whole (`what`), a category's id, text and holds; never
   values, examples or the trade catalogue's rows (`trade` is a block,
   reached by its name). The MCP's own sections (pricing, daemon,
   store: three blocks of five to ten entries each, which every tool
   description already points at by name) are not read by a pattern:
   they stand as lines in the MCP's index and whole by their names, as
   today; a pattern that matches nothing says the language's entries
   are what it read and that those three are read by name. So the
   language's slice is one document on both surfaces, exactly, and the
   search crate knows no frontend (C1). The case for the definition
   over the one line is the `weapon` row above; the precedent is
   `--count line:`, whose words read "the computed values whose name or
   own definition holds the words".
4. **Two tiers at the store's count.** A pattern's matches, ten or
   fewer (`acquisition_store::bound::LISTED`, a count of entries, never
   a byte size), are answered whole; more are answered as their index
   lines, counted. An entry a name reaches is whole whatever a pattern
   says of it; several items in one list resolve each on its own, and
   the denser form wins where two reach one entry (so a list of many
   small items may answer more than ten whole entries: the bound is
   per pattern, and the help says so). The answer carries, per
   pattern, the pattern, how many it matched and which tier, and one
   sentence on how to go on (name one for its whole).
5. **Nothing matched is a refusal** of kind `unknown_name` (the kind
   list stays closed: the pattern names nothing describe holds, and
   the reader's move is the same, re-ask), with no readings and one
   offer, the index, as a request the surface spells (`acq search
   --describe` at a terminal with its `argv`; `describe` with `{}` on
   the MCP), through `Way` and `Surface::request` as every offer is.
   A pattern that does not compile is `bad_pattern` in the compiler's
   own words. An ordinary unknown name keeps today's refusal, its word
   list whole (the way an agent finds a name it half knows).
6. **Pattern matching is the operator's**: Rust regex syntax,
   unanchored, any case unless `(?-i)`, compiled by the one compiler
   `bind.rs` holds; the pattern is tried against the name and against
   the definition each as its own string, so `^` and `$` are the ends
   of a name or of a definition and `~^pseudo\.total` (one backslash,
   the regex's escaped dot) reads as it says. A pattern is `~` followed
   by at least one character: the bare item `~` is the operator entry's
   own name (`describe.rs:575`) and answers it whole, as today.
7. **The step's keys stand only where they say something.** The index
   carries `trade_rows` and `next`; a pattern's answer carries
   `patterns` and `next`; the whole under `all` and an answer by name
   carry none of them, so they serialize as today (the sections pin
   keeps holding on `all`; `no_ids.rs` and the ten whole readers ask
   `all`). The echoed `patterns[].pattern` is payload (`PAYLOAD`), never
   spelled by the MCP.

## Do

In order; #153 is one commit, #154 one or two, no `Closes` keyword.
The ruling's commit precedes the build (Input).

1. **The index as a document of the search crate (#153).** In
   `describe.rs`, the index is made by one function from the same
   tables the whole is made from: every section of `Describe`, each
   entry a `Named` with its `what` cut to its line and `values` and
   `examples` empty (serialized without them today, line 99), the
   categories as `Described` with `holds` empty (add
   `skip_serializing_if` so the index carries id and text), `trade`
   none beside `trade_rows: Option<usize>` (the catalogue's row count,
   some in the index alone) and `next: Option<String>`, the index's
   sentence on how to go on (some in the index and in a pattern's
   answer alone). `describe(names)` resolves the list: empty → the
   index; `["all"]` → the whole, serialized as today; a category's id
   → its row whole (settled 2); otherwise as today, with `~` items by
   Do 4. The CLI passes its list through, bare `--describe` and bare
   `--json --describe` both the index, `all` the whole; `describe_text`
   renders what it is given (an entry with no values and no examples
   prints one line; the trade line prints from `trade_rows` and the
   next sentence from `next`), its `index` flag gone. The MCP's
   `describe` passes its list through too: `names` omitted → the
   language's index beside its own sections as lines (each own entry's
   `what` cut by the search crate's one-line rule, exported as a
   function on a string, naming no frontend); `all` → everything whole
   (the own sections whole as today); an own name or a language name
   as today. `web/form.rs:144`'s `describe(["trade"])` answers as
   today.
2. **The words (#153).** The CLI's `--describe` help: bare is the
   index, "a name and one line each, about N KB" (N measured by the
   build on its tree, rounded as the help rounds today), a name an
   entry whole, `all` everything (over 150 KB), `--json` the same
   document; the index's `next` sentence names an entry, a block,
   `all` and (after Do 4) a pattern, in words without a flag. The
   MCP's `names` doc and tool description: omitted is the index (its
   size), `all` everything with the sizes the description gives today,
   re-measured; the instructions' describe line the same in one
   sentence; every other tool description that points at `describe`
   with names is read and left as it is (a name still answers whole).
   `CLI-REFERENCE.md` and `MCP-REFERENCE.md` regenerated under
   `ACQ_UPDATE_FIXTURES=1` and read as a diff of what a user now reads
   (shape 9).
3. **The pins (#153).** At the lowest crate (C122): in
   `tests/surface_faults.rs`, the index's density — over every
   section, every entry's `what` holds no `: `, its `values` and
   `examples` are empty, every category's `holds` is empty, `trade` is
   none and `trade_rows` is the catalogue's count, `next` is some; the
   index names every entry the whole names, in the whole's order, and
   every entry name it prints (the eleven sections' entries, the
   section names, a category's id) answers whole when asked alone,
   while a not-built construct asked by name is still refused
   `not_built`; `all` serializes as the whole did (the sections
   pin, `c97_the_sections_listed…`, reads `all`, and a new case holds
   that the index's keys are the sections less `trade`, which the
   index leaves out, plus `trade_rows` and `next`);
   `c97_describe_says_how_terms…` reads `all` and asserts at least one
   example was checked (a nonempty-example witness), and the ten
   `describe(&[])` readers ask `all` where they mean the whole (one
   edit each, named in the commit). The CLI: `seat_faults.rs:498`
   re-pinned to the ruling (bare `--json` the index; the `more:`
   line's successor), the wording of the index's foot one golden under
   `goldens/search_json/` with its `# holds` facts; `search_json.rs`
   1193 and 2770 updated; `no_ids.rs` asks `all`. The MCP:
   `search_tools.rs:371` pins `describe {}` (its own sections removed,
   as today) as the CLI's bare `--json --describe` and `all` as
   `--describe all` (`assert_spoken_as`); `mcp_reference.rs:109` asks
   `all`; `spoken.rs` gains `all` beside `{}` (both swept). No wording
   pinned twice.
4. **The pattern (#154).** In `describe.rs`: a list item that is `~`
   and at least one character more is a pattern over the language's
   entries (settled 3 and 6; the bare `~` stays the operator's name);
   the CLI's `describe` arg loses its clap delimiter and the CLI
   splits the value as settled 1 says, a pattern taking the rest of
   its value (`--count`'s shape);
   compiled by `bind.rs`'s compiler (the function made `pub(crate)`,
   its words shared, never a second `RegexBuilder`); its matches
   resolved by the tier rule (settled 4) over `LISTED`; the answer
   gains `patterns: Vec<Matched>` (`pattern`, `matched`, `whole:
   bool`; present where a pattern was asked) and `next`. Nothing
   matched is the refusal of settled 5, built as a `SearchError::Scope`
   with kind `unknown_name`, no readings and one `Offered` that is a
   describe request: `Offered` gains that form (a describe with a list
   of names, empty for the index), `searched` in `refusal.rs` turns it
   into a `Way::Describe { names }` and not into a reading, and each
   `Surface::request` spells it (the CLI as `acq search --describe`
   with `argv`; the MCP as the `describe` tool with `{}`); existing
   realm offers unchanged. `describe`'s return type widens to
   `SearchError` where that is the smallest change (its callers named
   in the commit), or the describe refusal is lifted at the one place
   the CLI and the MCP already convert it. `patterns[].pattern` joins
   `PAYLOAD` on the MCP. The CLI's text: a pattern's answer opens with
   one line per pattern (the pattern, the count, whole or lines) and
   ends on `next`; a refused pattern prints as every search refusal
   does, its offer the printed command.
5. **The pins (#154).** Search crate: the fixed inputs of Input, each
   a property — the tier seen from both sides at ten and eleven; a
   match on the definition that the line alone would miss
   (`critical_strike_chance` under `~weapon`); case; `(?-i)`; `^` and
   `$` against a name alone and against a definition alone (a pattern
   anchored at a definition's start matches the entry whose definition
   starts so and not one whose definition holds the text later and
   whose name does not match); the bare `~` answering the
   operator entry; a name beside a pattern; nothing matched
   refused `unknown_name` with no readings and one describe offer;
   `~(` refused `bad_pattern` with the compiler's words; the trade
   rows not read (`~pseudo_total` finds no trade row by its key); the
   wide pattern answers lines only and its count is the number of
   entries matched; a category line asked by its id answers its row
   whole. CLI: one golden for the slice's words (a pattern answered
   whole, one answered as lines, the refusal with its printed offer),
   quoted for the shell as printed, the offer run as printed by the
   test (shape 6); a pattern with a comma (`'~^.{1,20}$'`) reaching the
   search crate whole, and `--describe` given twice adding its names.
   MCP: `~resist`, `~a` and `~^.{1,20}$` spoken as the CLI's `--json`
   answers, the documents one (no own section stands in a pattern's
   answer); `~--count` echoed unchanged in `patterns[].pattern`; the
   refusal's offer `describe` with `{}` sent back and answered as the
   index.
6. **The words (#154).** The `--describe` help and the MCP's `names`
   doc and description name the pattern form with a quoted example
   (`--describe '~resist'`; `names: ["~resist"]`), the tier in one
   sentence (whole when ten or fewer, their lines otherwise, per
   pattern), that it reads names and definitions, and on the MCP that
   pricing, daemon and store are read by name. The index's `next`
   sentence names it. References regenerated and read as a diff.
7. **The README.** The tour line becomes
   `acq search --describe [name,…|'~pattern'|all]   # the query language: an index, a slice by pattern, or entries whole`
   (one line; `readme_tour.rs` resolves its verb and flags); no other
   README change unless a sentence there contradicts the new help,
   named in the commit.
8. **The measurements, after**, in the report, each by the commands of
   Input on the step's tree: the index on both surfaces (CLI text and
   `--json`, MCP); `all` on both; `~resist`, `~life`, `~weapon` on
   both (text and `--json` on the CLI), each with its count and tier;
   `~a` on both; the refusal on both. Then `tools/gate.sh` on the
   step's tree before its last commit, nothing tracked edited while it
   runs.

## Done when

- From the commits: `describe.rs` makes the index and resolves `all`,
  a category's id and `~` (Do 1, 4); `search_cmd.rs` passes the list
  through and `describe_text` has no index flag; the MCP's `describe`
  passes its list through and cuts its own entries by the search
  crate's one-line function (Do 1, 4); `Offered`, `searched` and `Way`
  carry the describe request (Do 4); `web/form.rs:144` unchanged.
- From the commits: the pins of Do 3 and Do 5 exist by name; every
  golden, `CLI-REFERENCE.md` and `MCP-REFERENCE.md` changed are read
  as a diff of what a user now reads and each change is explained by a
  source change in the same commit (shape 9); no wording pinned twice
  (C153); the README tour line (Do 7).
- From the commits: no `C`, `S`, `N`, `T` or `Q` id in the new words
  (`no_ids.rs` green); the help and the tool text carry sizes the
  commit message says it measured, by command.
- Report-only (execution): `tools/gate.sh` exit 0 per stage on the
  step's tree; the table of Do 8 on both surfaces, every slice of the
  seat's three under 60,000 bytes on each, and the index on each under
  the CLI index's 49,833 of today. These are measured cases, not a
  guarantee over every list: a named block stays whole, an ordinary
  unknown name keeps its word list, and a list of many items may hold
  more than ten whole entries (settled 4).
- #153's "Closes when" met: `describe` on the MCP has an index form,
  the same index the CLI's bare `--describe` prints, bounded to a name
  and a line an entry; measured on both surfaces before (Input) and
  after (Do 8). #154's: both surfaces answer a pattern with the
  matching entries, the same set on each (whole at ten or fewer, their
  lines otherwise: the brief's settlement of the issue's "whole", as
  the seat session's comment settled it); nothing matched says so and
  offers the index; the describe golden holds the wording; the three
  slices measured on both surfaces; the help and the tour say how.

## Not in scope

- **Derived groups** (the owner's comment on #154): a trade-site
  filter group as the set of entries its clauses become, through the
  shipped catalogue (`reference/trade-filters-2026-09-12.json`,
  `trade/catalogue.rs`) and the translation table, never a hand list.
  The right second step, in that form; it waits for a seat that types a
  group name (the league seat's two needs were pseudo stats no site
  group names). At the close it gets an issue, `trigger:seat`, citing
  the comment. Nothing here builds a group or a tag.
- A pattern over the MCP's own sections (pricing, daemon, store):
  read by name, as every tool description points at them; a seat that
  patterns for one of their entries is the trigger (settled 3).
- A bound on a named block: `--describe computed` stays whole (86 KB;
  C53, a `describe` section is never cut); the index and the pattern
  are the way in. A bound on the ordinary unknown name's word list.
- A pattern over the trade catalogue's rows or the categories'
  `holds` lines as the index's content; a pattern over values or
  examples.
- `limit` or `expand` on `describe`: C53's knobs belong to lists of
  like entities; the whole is `all`.
- Any change to what an entry says (its definition) or to the one-line
  rule beyond moving it; a `kind` added to `ErrorKind`.
- The web UI: no describe page; its one reader, `web/form.rs:144`,
  asks `trade` by name and is unchanged.

## Rules

`TEMPLATE.md`. What differs: the build runs as an Opus subagent in the
main checkout on `main` (one writer per tree; the orchestrating
session's own gate runs on the range after); its scratch is the
session's scratch subdirectory named in its launch; every `acq` and
`acq-mcp` it runs is in that mock world over the scratch copy of the
pristine, never the owner's store, never a real-mode daemon; never
push; no `Closes` keyword; every number in a commit message (a size, a
count) from a command the message names, run after the last edit; a
commit ends with the Co-Authored-By line the launch gives. The
attribution line names the model the worker runs on.

## Report

`TEMPLATE.md`, and: the table of Do 8; the two patterns that stage ten
and eleven matches; the `Offered` form, the `Way` variant and the
return type chosen for the offer; the size literals written into the
help and the tool text, each with the command that measured it; the
agent-facing decisions the build made beyond those settled here, each
a sentence.

## Taken from the reviews

The first review's ten (`roadmap/reviews/describe-slices-brief-4f879174.md`):

1. The ruling recorded in `decisions/frontends.md` (C53 amended) in
   the commit before this cut, and the Do items rest on it (Input).
2. The MCP's own sections are not read by a pattern: the language's
   slice is one document on both surfaces, exactly, and the search
   crate takes no extra lines (settled 3, Do 1, Do 5, Not in scope).
3. The offer path drawn through `Offered`, `searched` and
   `Way::Describe`, apart from readings, with `argv` and `{}` (settled
   5, Do 4).
4. `trade_rows` and `next` present only where they say something, so
   `all` serializes as today and the sections pin holds on `all`
   (settled 7, Do 1, Do 3).
5. A category's id is a name describe knows, answering its row whole,
   with a round-trip pin (settled 2, Do 1, Do 5).
6. `patterns[].pattern` is payload on the MCP, with a case (settled 7,
   Do 4, Do 5).
7. `c97_describe_says_how_terms…` reads `all` with a nonempty-example
   witness (Read, Do 3).
8. The owner's question is ruled, and the alternative is gone (Input).
9. Density control, not a byte guarantee: the measured cases named,
   the exceptions kept (Goal, settled 4 and 5, Done when).
10. `web/form.rs:144` named and left unchanged (Read, Do 1, Not in
    scope).

The second review's five (`roadmap/reviews/describe-slices-brief-9dfa89f5.md`):

1. The CLI's list read as `--count`'s, a pattern taking the rest of
   its value, so a comma inside a pattern survives; the comma pattern
   a parity case (settled 1, Do 4, Do 5, Input).
2. A pattern is `~` and at least one character; the bare `~` stays the
   operator entry's name, both pinned (settled 6, Do 4, Do 5).
3. The index's keys are the sections less `trade` plus `trade_rows`
   and `next` (Do 3).
4. "Every name answers whole" scoped to entries, section names and
   categories; a not-built construct stays a refusal (settled 2, Do 3).
5. The escaped dot written once, with an anchored-definition case
   (settled 6, Do 5).

The third review's one (`roadmap/reviews/describe-slices-brief-38073d13.md`):

1. The anchored-definition case's negative is an entry whose
   definition holds the text later and whose name does not match, so
   the case contradicts nothing in settled 6 (Do 5).
