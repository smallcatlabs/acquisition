## 1. Model

An agent meets two doors over one semantics. `acq` (clap; `crates/acquisition-cli`) and `acq-mcp` (rmcp; `crates/acquisition-mcp`, about 2.9k src lines) both call `acquisition-verbs`, `-search` and `-plan` (C46). Each answer is a typed document whose JSON is the contract (C53, C149). The MCP returns that JSON directly. The CLI renders it as text through roughly 6k production lines (`search_cmd`, `plan_cmd`, `price_cmd`, `store_cmd`; the files are mostly renderers, with `#[cfg(test)]` making up the tail). `acq web` renders the same documents a third time, as HTML.

Next steps are typed: `refusal::Way`, spelled per surface by `refusal::Surface` (`Terminal`, `Paged`, `Spoken`). This replaced the MCP's hand-written `spoken()` tokenizer (#178, `9ce48bb3`).

Help is very dense prose. Every flag explains its edge cases inline, and the MCP pushes count glosses into a `describe` tool held by `acquisition-mcp/src/describe.rs`.

The surface has been shaped almost entirely by seats: blind agents with a 9-task brief, each followed by fix rounds. Each fault the seats found is pinned as a named test in `tests/seat_faults.rs`. The last CLI/MCP seat ran on 2026-11-04. Every seat since has been at the web pages.

## 2. Measurements

| What | Value | Command |
|---|---|---|
| CLI commands | 30 top-level + 24 sub (54 help sections); 64 distinct long flags, 168 flag occurrences; 2 flags that exist but are "Not built" (`--rebind`, `show --basis`) | python split of `CLI-REFERENCE.md` on `^## `, regex on flag lines |
| Env knobs | 21 named in README; 29 `ACQ_*` read in `crates/*/src` | `git grep -ho 'ACQ_[A-Z_]*' crates/*/src \| sort -u` |
| `acq --help` / `search --help` / whole reference | 13.2 KB (~3.3k tok) / 9.8 KB (~2.5k) / 76 KB (~19k) | same split |
| MCP | 30 tools, 111 parameters | `grep -c '^### '`, `grep -c '^- \`'` on `MCP-REFERENCE.md` |
| MCP first turn (every session) | 42,089 B (~10.5k tok), bound 43,000 | `acquisition-mcp/tests/surface_size.rs:20-26` |
| …of which the `account` param text | 19 copies × ~329 B ≈ 6.2 KB (15%); a test *requires* them identical | `grep -c '^- \`account\`'`; `surface_size.rs` `account_is_one_sentence…` |
| `describe` index | ~38 KB MCP, ~33 KB CLI (~9k tok); the instructions point there first | tool text |
| MCP seat total answer bytes | 219–249 KB per run (~60k tok); `tools/list` 72–76 KB of that (loaded twice) | python over `audits/{league,answer-facts}-2026-11/seat/calls-mcp-*.csv` |
| `apply_plan` args | the agent echoes a 1.9–2.0 KB plan envelope on each apply | same csv |
| Refresh task (T2) calls, MCP | 18, 18, 19, 21 across four runs and three fix rounds: flat | `SEAT-LEDGER.md` rows 33–39 |
| Since the last CLI/MCP seat (`f87e645d`) | 116 commits to cli(non-web)/mcp/search/verbs src; CLI-REFERENCE 64→76 KB (+19%), MCP-REFERENCE 26→35 KB (+34%) | `git log --since=2026-11-05 --oneline -- …`; `git show f87e645d:…` |
| `seat_faults.rs` | CLI 141 tests / 4,393 lines, still growing (latest 2026-11-23, after C153 on 2026-10-29); search 57 | `grep -c '#\[test\]'`; `git log` |

## 3. Where a deep review would pay

**1. The fixed cost of reading before acting, and help written as edge-case prose.** (High confidence; I verified the measurements.)
- An MCP session pays about 10.5k tokens before its first call and another ~9k for the `describe` index the instructions send it to. About 15% of the first turn is one `account` sentence repeated 19 times, and a test locks that duplication in.
- CLI help puts every edge case into the flag text: `--describe` alone is ~1.3 KB, `--account` ~1 KB.
- **Reviewer question:** what is the smallest text an agent must read to act correctly, with everything else one call away? Would a shared definitions block, short parameter docs and `describe` for the rules halve the first turn? The "decline a trim under 10%" rule in `surface_size.rs:22` blocks exactly this kind of change.
- **Files:** `acquisition-mcp/src/params.rs`, `server.rs`, `acquisition-cli/src/cli.rs`.
- **Payoff:** 5–20k tokens per session.

**2. The two doors share no naming scheme.** (High confidence.)
- `characters` is a GGG job on the CLI but a store read on the MCP (the CLI's `store characters`).
- The MCP `leagues` takes a required `stored: bool` that must be true (`params.rs:229-232`, refused at `server.rs:451`). It is a vestige parameter.
- The CLI has six per-kind job verbs where the MCP has one `submit_job`. `policy show` maps to `sync_policy`, `saved list/show` to `saved_searches`, `refresh --plan` to `refresh_plan`.
- #256 (open): `refresh --plan --json` prints a different document from the MCP's.
- The `Surface`/`Way` spelling layer exists partly because the names differ.
- **Reviewer question:** if each MCP tool were the CLI verb path (`price_set` ≡ `price set`) with the same JSON, what would the spelling layer, the seat briefs and the references lose?
- **Payoff:** concepts removed; one reference instead of two.

**3. Undo and receipts keep failing at the seats.** (Medium-high confidence.)
- "Undo" comes up in 14 of 23 debriefs. A stale undo was "recovered through the conflict's offer" in every league and answer-facts MCP run (ledger rows 35, 39). Before that, an undo was missing `--account` (release). A first policy has `no_undo`, and seats wrote irreversible policies knowing it.
- Each fix round pinned one more case (`seat_faults.rs:3667-3922`, five tests on printed undos).
- The missing shape: undo is a single-step compare-and-swap request, not "restore to the state before me".
- **Reviewer question:** should a write return a restorable prior state, or should there be a `clear` for every write kind, instead of a revision-guarded request that goes stale on the next write?

**4. Counts whose set is unnamed.** (Medium confidence.)
- Debriefs across seats (release, trade, answer-facts) stumble on `fetched` / `never_fetched` / `places` / `tabs` being over different sets, and on "location list seen" and the price counts overlapping.
- The debrief prompt grew a question 10 ("which number did you take for a whole it was not") just to catch this.
- Fixes are per-sentence (C146 tests, `seat_faults.rs:1525-1751, 2173-2431, 2811`).
- **Reviewer question:** does every count in a document carry its set as data (a `of:` key), so that the glosses in CLI text, MCP `describe.rs` and web HTML stop being three hand-kept homes?

**5. Three renderers of one document, with their own words.** (High confidence; verified.)
- `FetchReason` is worded twice: `plan_cmd.rs:1610` `fetch_why` ("selected") and `web/page.rs:4477` `fetch_words` ("ticked"). The MCP shows the enum raw and glosses in `describe.rs`.
- CLI text alone has ~650 prose literal lines across six `_cmd` files (grep of 3-word string literals); `page.rs` has another 350.
- **Reviewer question:** should a document carry its own `says` sentences (or a phrase table in verbs) that every renderer prints?
- **Payoff:** thousands of lines, and one wording golden per phrase (C153).

**6. Paging and envelope vocabulary differ by verb.** (Medium confidence.)
- Lists use `total`/`left_out` (accounts, tabs), `returned`/`limit`/reached (store events, refused), `open_left_out`/`finished_left_out` (jobs), `leagues_total` (leagues) and a `next` token (search only).
- `accounts` has no `--limit`.
- Job verbs wrap output in `{outcome,payload,job_id,account}`. `leagues --json` without `--stored` is GGG's raw answer. `auth --no-browser --json` prints two documents. `dash` needs `--json`.
- **Reviewer question:** is there one list envelope (`rows`, `total`, `left_out`, `next`) that every listing could adopt?

**7. The refresh loop's call count.** (Medium confidence.)
- The CLI or MCP agent loops policy → plan → apply → poll `job_status` → `job_result` → plan again, and echoes the plan envelope back each time.
- The MCP's T2 call count never moved across fix rounds.
- **Reviewer question:** would a blocking `apply` (wait plus result in one call) and a plan handle in place of the envelope stay inside C34/C38 (client-side, still binding)?

**8. `seat_faults.rs` is organised by where a fault came from, not by surface.** (High confidence.)
- 141 incident-named tests (`cli4_…`, `cli172_…`) in one file, still growing after C153 said findings join an existing surface.
- **Reviewer question:** which of these are the same property at different nouns, for example the five "every printed command runs as printed" tests at `:3667-3922`? Could they be one table-driven test per surface?

## 4. Low yield / already covered

- The age vocabulary is unified: `store::age::{span,ago}`, which `store_cmd.rs:140` and `page.rs:4246` delegate to.
- Key order is fixed by #200 (preserve_order).
- Plan ages are fixed by #211.
- The `spoken()` tokenizer is retired (#178).
- Dead work in this slice is already listed in #214. Frontend fact writes are #212.

Fate of the semantic-model findings for this slice:

| Finding | State | Where |
|---|---|---|
| Non-UTF-8 argv panic | Standing, unfiled | `terminal.rs:46`, `:678` still use `std::env::args()` |
| `print_result` renders by retention | Standing, unfiled; now commented as intended | `jobs_cmd.rs:404-410` |
| `version` two makers | Standing, unfiled | `run.rs:562-580` |
| Account list two shapes | Standing, unfiled | `store_cmd.rs:57-67`, `json!` |
| MCP writer `select!` reorder | Standing, unfiled | `transport.rs:99` |
| Key order | Fixed | #200 |
| CLI plan ages | Fixed; document divergence remains | #211; divergence in #256 |
| `spoken()` | Fixed | #178 |

## 5. Blind spots

- Nobody has measured the CLI side's token cost per seat. `calls.csv` for the CLI has no byte column that I checked.
- No seat has used a non-Opus model since the release seat (Sonnet, October).
- I could not confirm the raw `tools/list` JSON overhead of the `anyOf [T, null]` schemas without running the binary.
- No seat has driven the MCP with a host's real tool-selection behaviour across 30 tools, for example whether it picks `characters` meaning the store read.
- Whether the 29 vs 21 env-knob gap is all test-only knobs: likely, but not checked.

## 6. Surprises

- The seat skill says to sit "after any change to the surface an agent meets". Yet 116 commits and 30% growth in the references have landed since the last CLI/MCP seat.
- `surface_size.rs:22` codifies the owner's rule declining trims under 10% of the first turn, and requires the most repeated text to stay repeated.
- The MCP `leagues` tool has a parameter whose only legal value is `true`.
- C153 (2026-10-29) arrived after most of `seat_faults.rs` existed, and the file kept growing after it.

I verified the top claims in code myself: 1 (`surface_size.rs`, the reference counts), 2 (`params.rs:229`, `server.rs:451`) and 5 (`plan_cmd.rs:1610` against `page.rs:4477`). Debrief sampling covered: release cli-opus, trade mcp-opus-1, answer-facts cli-2 and mcp-1, plus grep across all 23 debriefs and the ledger rows.
