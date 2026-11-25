## Code craft scout (comments, style, idiom, file shape)

**What I sampled.** I read the module doc plus a large part of the code in these files:
- `search/src/answer.rs`: the 332-line doc, the request types and the head of `unaged`.
- `cli/src/web/page.rs`: the doc and the full function index.
- `cli/src/web/mod.rs`: the head.
- `daemon/src/daemon/queue.rs`: the doc and `restore_jobs` (`queue.rs:322`).
- `store/src/lib.rs`: the doc, `record_in` and `ingest_item`.
- `client/src/client.rs`: the doc and the error and option types.
- `client/src/door.rs`: the first 140 lines.
- Smaller files: `store/src/age.rs`, `verbs/src/fault.rs`, the plan and search `lib.rs` docs, `cli/src/run.rs::run`, the table loaders in `category.rs` and `class.rs`, `protocol/src/job.rs` and `realm.rs`, and the head of `rails.rs`.

From `mcp` and `transport` I took comment ratios only. All crate paths are under `crates/acquisition-*`.

### 1. Model
The code is a dense, uniform house style. Almost every item has a doc comment written in registry prose: present tense, built on "never" and "the one X", and citing C-ids. The module docs hold the mechanism, as C192 intends: a short head, then `# Rulings held here` and `# As built` sections. Those sections are long run-on bullets that restate the behaviour the tests pin.

The history cut (#293 cut 5) worked:
- Over all non-test source, comment lines that name a date number 18, and those that name a round, seat or reviewer number 68 (out of 23k comment lines).
- The cut was mostly a rewrite, not a removal. The five commits show about +5.9k / −6.6k lines, a net of only about −700.

So the volume problem is no longer history. It is mechanism retold in prose.

Code idiom is mostly sound: one copy of the age vocabulary (`store::age`), no TODOs, few `#[allow]`s, and a deny on unwrap/expect in five crates. The structural debts come from shapes that C1's dependency edges forbid. The store links nothing, so realm, endpoint and location cannot be shared types. Each crate then re-strings them.

### 2. Measurements
Comment and file counts are over non-test source (counting stops at the first `#[cfg(test)]`). The scripts are in the scratchpad (`cm.py`, `fns.py`, `idi.py`, `cc.py`).

| Measure | Value | Command |
|---|---|---|
| Source lines / comment lines | 97,596 / 23,341 (23.9%); module docs 7,477 | `cm.py` |
| Comment share by crate | protocol 43%, client 32%, store 30%, cli 19% | `cm.py` |
| Module-doc lines under `# As built` / `# Rulings held here` | 3,230 / 1,485 | inline python |
| Module docs over 40 lines | 50 files, 3,906 lines past line 40 | same |
| Comment lines citing an id / saying "never" / saying "the one…" | 11.1% / 4.8% / 1.8% | `cc.py` |
| Functions over 100 / 200 lines; nesting ≥7 | 80 / 27; 28 | `fns.py` (brace depth) |
| Longest functions | `run.rs:78 run` 745, `store lib.rs:1575 record_in` 589, `describe.rs:353` 577, `kinds.rs:450 execute_inner` 476 | `fns.py` |
| `#[allow]` | 26: 13 `dead_code` (7 are `generated_by` headers), 10 `too_many_arguments` | `git grep '#\[allow'` |
| Hand-written error types (no `thiserror`) | 45 `impl Error`, 10 of them table errors | `git grep 'impl .*Error for'` |
| `.clone()` per 1,000 lines | plan 21, verbs 16, search 14, daemon 9, protocol 0 | `idi.py` |
| `"pc"` literals in source | 72 | `git grep '"pc"'` |
| "door" in crates vs "connect policy" | 1,822 vs 2 | `git grep -i -w -c` |

### 3. Where a deep review would pay (ranked)

**1. Shared domain types that C1 forbids (verified).**
- `store/src/lib.rs:542` `Endpoint` is the typed job-to-endpoint shape. The daemon re-parses it from `kind: String` + `params: Value` (`Endpoint::from_job`, `kinds.rs:265`). It also dispatches the same kinds again as string arms in `execute_inner` (`kinds.rs:465–761`). The protocol lists them a third time as `&[&str]` (`job.rs`).
- Realm has three shapes: `protocol::Realm` (an enum), `search::corpus::Realm { All, One(String) }` with `REALMS: [&str; 4]` (`corpus.rs:121`), and `realm: &str` 21 times in the store.
- A location is passed as loose parameters or `(String, String, String)` tuples (`ingest_item`, 11 parameters, `lib.rs:2979`).
- **Question for a reviewer:** should there be one leaf "vocabulary" crate under the store, or should the store link the protocol? This is an edge, so it is the owner's call.
- **Status:** #199 and #100 name the job-kind half and park it "for the GUI's wire design". The realm and location half is unfiled.
- **Payoff:** concepts removed, plus every `== "probe"`-style match (50 such comparisons).
- **Confidence:** high.

**2. Ten embedded reference tables, each with its own loader, header struct and error enum (verified).**
- Files: `class.rs`, `category.rs`, `defences.rs`, `properties.rs`, `totals.rs`, `trade/stats.rs`, `trade/forms.rs`, `trade/catalogue.rs`, `plan/currency.rs` and `vocabulary.rs`.
- Each is an `include_str!` parsed at runtime into `LazyLock<Result<_, XTableError>>`. That failure is threaded through the API (260 references to `*TableError`) and through fallbacks such as `class.rs:299` (empty names when the table does not load).
- A shipped table cannot fail at runtime if the gate parses it.
- **Question:** validate in `build.rs` or a test, then use one generic `Table<T>` loader with a shared `Header`, and panic as a broken internal invariant (C47 allows this).
- **Payoff:** about 9 error enums (around 400 lines), 7 `dead_code` allows, and error arms in callers.
- **Confidence:** high.

**3. The three worst grab-bag files have clear seams.**
- `cli/src/web/page.rs` (5.6k lines): it already has section markers for saved, forms/editor, answer/cards, item, places (`:3574`), plan/apply (`:4243`) and jobs (`:4780`). Split it into one module per page plus `card.rs`. It also hand-walks `&Value` documents (`refused`, `saved_cells`) where typed documents exist.
- `search/src/answer.rs` (4.2k lines, 60 types): split into request (types, parse, token), answer-out types, answering (`prepare`/`unaged`), and `Router`/routes (`:3213`).
- `store/src/lib.rs`:
  - `record_in` is one 589-line match over six endpoints, with a stash arm of about 300 lines. Make it one function per endpoint, sharing an `Ingest` context struct that also ends `ingest_item`'s 11 parameters.
  - `cli/src/run.rs::run` (745 lines, nesting 9) holds verb bodies inline. Move them into their `*_cmd` modules.
- **Payoff:** readability and review cost. Little code is removed.
- **Confidence:** high on the seams.

**4. Module docs as a second spec.**
- `# As built` bullets run 20 to 60 lines as single sentences. Examples: the site-search bullet at `answer.rs:94–121`, and `queue.rs:8–55`, which is one bullet.
- Some paste registry entries word for word: `web/mod.rs:11` ("C138 … (verbatim)"), `trade.rs:42,48`.
- `plan/lib.rs:9–35` restates C39, C38 and C37.
- Each such paragraph is one more sentence to keep true; the assessment found about 19 findings against comments.
- **Proposed rule (cuts about a third):** a module doc is at most about 40 lines and says what the module owns, its seams and the ids it holds. A behaviour that a named test pins is cited by the test's name, not retold. A registry sentence is cited, never copied.
- By arithmetic that rule removes about 3.9k lines (the lines past 40) plus the verbatim copies, with no "why" lost: the whys live in the item docs and the registry.
- **Confidence:** medium. This is judgment.

**5. Rules that force duplication.**
- The proptest config `generated.rs` is copied three times by design: the client, daemon and search copies all say "a change here is made in both" because "a shared test-support crate would be a new dependency edge".
- `cli/store_cmd.rs:239 plural_of` duplicates `terminal.rs:21 plural`, whose own doc says it is "from one copy" (verified).
- Pass-through wrappers: `store_cmd.rs:140 ago` and `page.rs:4246 span` only forward to `store::age`.
- **Payoff:** small, but it shows the "a rule makes a copy" pattern.

### 4. Agent-written smell, with prevalence (judgment)
- **Defensive and "one maker" prose:** 1,106 comment lines say "never" and 408 say "the one…". Many answer an imagined reviewer, for example "so no second maker of any of them stands here" (`answer.rs:242`). This is pervasive: about 6% of comment lines.
- **Speculative scaffolding:** `daemon/tests/door.rs:188` `#[allow(dead_code)] // the fields the later rounds' scenarios read`. This is rare.
- **Counters and flags in a single tuple:** `queue.rs` `(mut requeued, mut held, …six…) = (0,0,0,0,0,0)`; 14 functions take two or more `bool` parameters; there are 26 `Option<bool>`. This is moderate.
- **Owner and history residue:** "the owner's choice, over …" (`answer.rs:103`), "(the owner: "i accept your recommendation")" (`search/build.rs`), "(the owner: "the full list")" (`corpus.rs:120`). There are 72 lines like this. They are low value, and the cut kept them by rule.

### 5. Quotes
- **Best:**
  - `age.rs`: "An epoch after `now` (a clock set back) reads `0s ago`; the difference saturates".
  - `client.rs`: HELLO_WITHIN, "its shared lock never spans an await — so five seconds … is generous".
  - `queue.rs`: "// Only a concluded tree leaves memory."
- **Worst:**
  - `queue.rs` restore: "the marker's job is never a parent (C134's arm excludes one), so its failure is in the fact".
  - `answer.rs:242`: "so no second maker of any of them stands here".
  - `door.rs:188`: "the fields the later rounds' scenarios read".

### 6. House standard
`client/src/door.rs` is the best file I read: a sans-IO state machine with `step` as a pure function, one-line docs on each variant, and a stated criterion for where its edge is drawn. `store/src/age.rs` (one vocabulary, `SAID` reused by the MCP) and `verbs/src/fault.rs` (exhaustive `kind()`, a remedy per frontend that the compiler forces) are also good. The rule to take from them: put state in data, keep each doc a line long, and let the compiler enforce completeness.

### 7. For the review plan
- **The three to read line by line:** `store/src/lib.rs` (`record_in` and the ingest), `search/src/answer.rs`, and `daemon/src/daemon/kinds.rs` together with `queue.rs` (stringly kinds, `restore_jobs` at 356 lines).
- **Reviewer brief:** "Find shapes, not nits: a type that exists in one crate and is re-stringed in another; a match over strings that an enum would close; a 200+ line function and the struct its parameters want to become; a paragraph that a named test already says. For each, the deletion it enables." Give the reviewer the code without the module docs first, then the docs. That shows which sentences carry a why no code shows.

### 8. Low yield / already covered
- History in comments (the cut worked).
- Unwrap/expect: the daemon's 82 are mostly `lock().unwrap()`, and the 8 in search are a parser method named `expect`.
- TODOs: there are none.
- The age and time helpers are already unified.
- Job-kind typing is filed on #100.

### 9. Blind spots
- I did not read `mcp` or `transport` in depth, or `search/describe.rs` (577 lines) and `pseudo.rs`.
- I could not tell how much of the `pub` surface could be `pub(crate)`: search has 603 `pub` items against 227 `pub(crate)`, and it needs a cross-crate use check.
- The brace-depth script is approximate (strings stripped, nested functions counted separately).

### 10. Surprises
- `GLOSSARY.md` says "door" is ambiguous and that new text should use the specific word. The code uses "door" 1,822 times and "connect policy" twice.
- The glossary itself lists eight words with two or three senses (route ×3, basis, scope, pin…).
- "Envelope" has three senses in code: the plan (C38), the store's response body, and the trade query JSON. It has no glossary line.
- The workspace has no `[workspace.lints]`: the unwrap deny is repeated as a `cfg_attr` in five `lib.rs` files, and it sits after module docs of 200 to 350 lines.

**Verified myself:** claim 1 (`Endpoint`, `Realm`, the `kind` strings), claim 2 (the table loaders and their error counts), the duplicated `plural`, and the extent of `record_in` (`lib.rs:1575–2168`).
