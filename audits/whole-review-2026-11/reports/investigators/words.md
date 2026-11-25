# Investigator's report: words

As returned by the Opus investigator under `INVESTIGATOR.md`, local paths shortened. Its claims are its own; what the orchestrator re-ran is marked in `findings/words.md`.

## Track: one document, three projections — where the words live

Scratch: `<scratch>/words/` (`life.txt`, `life.json`, `ps.json`, `help.txt`, `web-syms.txt`, `way.sh`, `fns.sh`). I ran no cargo and used only the real-store copy, for reads.

### 1. The picture

The claim holds for two of the three doors. For the MCP it is wrong in an instructive way.

| Noun | Where it is composed | What pays for it |
|---|---|---|
| Search answer data (`Answer`) | search crate | C53/C149 contract; the MCP serializes it unchanged (`server.rs:222-249`) |
| The answer's sentences | **CLI** `search_cmd.rs`, 80 `*_words/_text` fns | the terminal reader. The web borrows them; the MCP never sees them |
| Glosses on fields | CLI consts **and** `describe.rs`, worded twice | C146 |
| Refusal ways (`Way`, 20 variants) and their `says` | verbs `refusal.rs`, once | C11, C143 |
| Spelling of a way | `terminal.rs` argv, `told.rs` request, `web/mod.rs` `Paged` | three doors with three vocabularies |
| A fault's remedy (21 `Fault` variants) | **prose**, twice: `terminal.rs:78-150`, `told.rs:35-110`; web overrides 2 arms | nothing: the remedy is never a `Way` |
| Shell quoting (4 shells) | search crate `answer.rs:3938-4222`, reads `ACQ_SHELL`/`MSYSTEM`; daemon twin `login.rs:790` | C156 |

**The search answer, traced (V).** Command:
`… acq --account 'GERWARIC#7694' [--json] search --realm pc 'pseudo.total_life>=90'`
- The text is 4.6 KB. The JSON is 18.2 KB, and a walk of every string in it found **no sentences**: only names, line texts and `*_age`.
- Each sentence a reader meets is composed in the CLI:
  - "(counted over every location in the realm…)" is `search_cmd.rs:837`.
  - "104 locations were never fetched: nothing here says what they hold" is `:1297`.
  - "each term counted alone … never over the matches" is `:2146`.
  - "each returns exactly the members counted" is `:1976`.
- Only "as they lie in their place…" comes from the search crate (`PLACE_ORDER_WORDS`, `answer.rs:1140`).
- The web prints the same CLI words: `page.rs:2191-2244` calls `search_cmd::scope_words`, `total_words`, `undecided_words` and others.
- An MCP agent reads none of these. It gets field names, plus `describe`, which words the same rules again in other words:
  - `describe.rs:291` against `search_cmd.rs:2146`;
  - `describe.rs:241` against `:837`;
  - `describe.rs:328` against `TRADE_OVER` (`:1043`).

**The refusal, traced (V).** `--account nobody --json search …`:
- `kind`, `accounts` and each offer's `says` ("the same request as X") are composed in verbs.
- The `error` tail "(see `acq accounts`)" is the CLI's (`terminal.rs:157`).
- The MCP's tail for the same refusal is different content, not just different spelling (`told.rs:121`, the username/uuid rule).
- A second, sharper case: `price status --league Nowhere` prints `next: acq refresh --plan …` in text, but `ps.json` has **no** way on at all. So an MCP agent never gets that remedy.

**Counts (V; commands in `fns.sh`, `way.sh`)**
- Sentence fns, by the given rule:
  - CLI 116 (80 of them in `search_cmd.rs`);
  - search 11, daemon 6, MCP 2, verbs 1, plan 0, store 0.
  - The wider suffix rule gives CLI 170.
- Symbols `web/` takes from `*_cmd`: 89 distinct (`web-syms.txt`), plus 5 from `terminal`/`run`.
  - 78 are sentences (`*_words`, `*_text`, `*_said`, and consts like `EXPLAIN_OFFERS`, `TRADE_OVER`).
  - 4 are layout (`Line`, `row_shows`, `items_cell`, `BLOCKS`/`describe_list`).
  - 7 are logic (`trade_realm`, `counted`, `clauses_listed`, `asks_totals`, `trade_word`, `open_browser`, `example_name`).
- `Way` has 20 variants, not 21.
  - Every one is spelled in `terminal.rs` and in `told.rs`.
  - 10 are also spelled in `web/`.
  - So each variant has 2–3 spellings, about 520 lines in all.
- Printed commands:
  - `printed.rs` composes them (242 lines; 12 uses of quote/print helpers).
  - About 150 string-literal lines outside it spell `acq <verb>` by hand: `price_cmd` 40, `store_cmd` 15, `shop_cmd` 13, `plan_cmd` 10, `jobs_cmd` 9, `daemon_cmd` 8, plus the daemon, the client and the MCP.
  - These are remedies no `Way` holds.

**C139 (V).** `acquisition-cli` has no `[lib]` target (`Cargo.toml:12`), and the 89 symbols are `pub(crate)`.
- C139's stated reasons are about distribution, not words, but the words make any other placement expensive.
- A second GUI (C191) can link verbs, search, plan and client today. It gets none of the 78 sentences: it would rewrite them or become another `acq` verb.

### 2. Theses

**T1. One home for the words: a document carries its sentences and its ways on, made in the crate that makes the document.**

```rust
pub struct Doc<T> { #[serde(flatten)] pub body: T, pub says: Vec<Said>, pub ways: Vec<Offer> }
pub struct Said  { pub at: &'static str, pub role: Role, pub text: String } // at: "total", "scope.never_fetched"
pub enum   Role  { Answer, Warning, Gloss /* fixed: by describe name, not inline */ }
pub struct Offer { pub at: &'static str, pub says: String, pub count: Option<usize>, pub call: Call }
pub struct Call  { pub verb: &'static [&'static str], pub args: BTreeMap<&'static str, Json>, pub person: bool }
// a frontend: fn lay(&Doc) -> text | html;  fn spell(&Call) -> argv | (tool, args) | url
```

- **What goes or moves** (estimates from lines read, I):
  - About 1.5–2k lines of `search_cmd.rs` sentence code move into search, and the web's 89 imports go to 0.
  - The `Way` spellers (about 520 lines) shrink to 3 generic `spell(Call)` fns of about 30 lines each.
  - `fault_words` ×2 (about 175 lines) go: a fault carries an `Offer`.
  - The FetchReason pair (`plan_cmd.rs:1610`, `page.rs:4477`) becomes one.
  - The roughly 150 hand-spelled `acq …` lines become `Call`s, so the MCP gains about 40 remedies it lacks today.
  - Net deletion is about 1–1.5k src lines and about 0.3k test lines (the five printed-command tests at `seat_faults.rs:3667-3922` become one property over every `Call`).
  - It is **not** "thousands" deleted. Most of the code moves; it does not vanish.
- **Why the edge cases go:**
  - A sentence made below the frontends cannot name a flag, so C143 becomes a dependency edge rather than the harness scan (`c143_a_text_speaks_no_frontend`).
  - A remedy that is a `Call` cannot be spelled wrong per door.
- **Smallest changed input:** `acq --json price status --league Nowhere`, and MCP `price_status`, gain an `offers` entry for `refresh plan`. The text is unchanged.
- **Rulings:**
  - C53 strengthens: text becomes layout over `says`.
  - C139's reasons survive, but `web` can become its own lib crate that `acq web` calls.
  - C146 gets one home.
  - C156's quoting moves up to the CLI. The daemon twin stays under C1.
  - C11 and C149: `offers` gain `call`, which is additive.
  - No invariant is touched.
- **Costs and gains:**
  - Security: none.
  - Performance: JSON grows by the `Answer`/`Warning` sentences, about 1 KB here (I). Fixed glosses stay by name.
  - Function: the MCP gains remedies.
  - An agent's ease: higher.
- **Strongest objection:** a sentence's layout is not separable from its words. "rows 1–10 of 586 shown, … (no sort asked): `<command>`" interleaves a count, a gloss and a printed command. `Said` would need slots for an embedded `Call`, and that is a template language. My answer: allow `{call}` as the one slot.

**T2. An MCP tool is the CLI verb path, generated from one verb table. Then `Way` is a `Call`, and T4's `take` is unnecessary.**
- **What goes:**
  - The name map (`characters` vs `store characters`, `sync_policy` vs `policy show`, `leagues stored:true`).
  - The 20-arm spellers.
  - Two hand-kept references (35 KB and 76 KB) become generated text from one table.
  - The `account` paragraph: 296 B × 19 copies = 5.6 KB of the 42 KB first turn (13%, V). The table writes it once, in the instructions.
- **Edge cases go** because an offer then *is* a call of a tool or a verb, in the same argument names. A door spells the verb path and nothing else.
- **Smallest changed input:** `tools/list`.
- **Touches:**
  - C13: the table marks which verbs each door has, since the MCP must not have `auth` or `daemon stop`.
  - `surface_size.rs:22`: the owner's rule declining trims under 10%, and the test requiring identical `account` text, are his to revisit.
- **Strongest objection:** CLI ergonomics (positional args, `--plan`/`--apply` modes) are not JSON's. A generated MCP drags those modes into tool schemas.

**T3 (a concept that must stay). A frontend owns a small, named residue of wording.** In a sample of 30 differences (V, read from the lines cited):

| Kind | Count | Examples |
|---|---|---|
| Must differ | 5 | NoDaemon in real mode (MCP never spawns, C13); AnotherDaemon; FetchReason "ticked" (the page's checkbox); the web's Replan and NotCovered (cause alone, link beside) |
| Spelling only (one table erases it) | 10 | 6 fault arms (`--league` vs `league`; `acq policy set` vs `set_sync_policy`); 4 `Way` spellings |
| Different for no reason | 15 | 5 fault arms (UnknownAccount names different remedies; NoUuid; NoDaemon; NoJobOnDisk; Unfinished, where the CLI ignores the `next` the fault carries); 5 FetchReason words; 3 gloss pairs (CLI vs `describe`); price status's missing way on; `--account` help (837 B) vs the MCP param (296 B) |

What a frontend must still own:
- its own controls' words;
- a person's act addressed to *its* reader (on the MCP, "a human, from a terminal");
- layout;
- quoting for its shell, or a URL encoding.

That is about a sixth of today's differences.

### 3. One thing I would defend

`refusal::Surface` with `Way`. T4 (`take`) would cut it, but it is already the right shape, half-applied. The fault is that faults and the `_cmd` files bypass it. A `take` verb is one more mechanism to fix a naming gap that T2 removes.

### 4. Where the old way is right

A terminal's decision view should differ from the JSON in density (C53). Printing the "never over the matches" gloss on every text answer serves a person.

For an agent the same gloss is schema. It belongs in `describe`, which the MCP's choice already reflects. Keep the asymmetry; drop the second wording.

### 5. Parting from the scouts

- **T1** says the MCP "patches CLI spellings". It does not: it gets no answer sentences at all and writes its own fault prose. On counts I agree: 116 fns, 89+5 symbols (T1 says 93).
- **T4** says 21 variants, each spelled three times. It is 20 variants: 2 full spellings plus the web's 10. And `take` is unneeded under T2.
- **T8**: CLI tests hold 1,183 JSON-index lines against 508 `.contains("` lines. "Mostly re-pin spelling" is too strong for test code. It is true of the goldens: 74 files, 1 MB. One gloss stands in 27 golden files across the CLI and the web, and under T1 those files would still carry it, because each renders it. What T1 removes is the second *source* edit, not the regenerations.
- **history-hindsight item 2**: I agree. The rendering function per surface exists already; the gap is the roughly 150 remedies that never become a `Way`.

**The agent question.** Words help an agent in a remedy's `says`: they tell it which offer to take. They are noise as fixed glosses repeated on each answer.

`acq --help` is 13.2 KB. 14 entries over 300 B make up 78% of it, and they carry 52 backticked field names. Two examples:
- `version` (638 B) explains how the contract digest is made;
- `jobs` (1.5 KB) enumerates the fields of its JSON.

Their reader is a past reviewer or seat. The reader of `--help` needs one line per verb.

### 6. What I did not check

- The web's rendering in a browser. I read `page.rs` only.
- Whether MCP clients accept JSON-Schema `$ref`, which would let the `account` doc be written once in the schema.
- Exact line counts for the moves (estimated).
- The `plan`/`jobs` documents end to end.
- The CLI's token cost per seat.

### 7. Small finds

- `plan_cmd.rs:1620` says "— played since", but the planner reports experience unequal in either direction (`page.rs`'s comment says so).
- `--json` offers on the CLI put a shell string in `request`, where the MCP and `--request` put an object: one key, two types.
- `price status --json` carries no way on; the text's `next:` line is CLI-only.
- "location lists seen" is composed twice: `search_cmd.rs:1233`, `store_cmd.rs:816`.
- The CLI ignores `Fault::NoJobOnDisk.next` and `Fault::Unfinished.next` and writes its own (`terminal.rs:138-143`).
- Shell detection reads the environment inside the search crate (`answer.rs:3979-4003`), a library below the frontends.
- MCP `leagues` takes `stored`, whose only legal value is `true` (the scout's find, confirmed in `params.rs`).
