# Cold review of 4-full-1-cards' build, `6bd852fe..f562ca83`

Read cold on 2026-11-08 by an Opus subagent of the orchestrating
session against `roadmap/briefs/4-full-1-cards.md` (cut 4) and the
tree at `f562ca83`. The reviewer ran the search crate's tests (423),
the CLI's web, search JSON and reference tests (58 and 9), the form's
unit tests, the MCP's surface-size and describe tests, `tools/arch-check.sh`,
`cargo fmt --check`, `git diff --check`, `tools/docs-check.sh` and
`bothy.py check`, each green; it ran no gate and edited nothing. Its
report follows, verbatim but for this head; the fix round and the
fates are the roadmap's ledger row.

### Findings, most severe first

**1. The CLI's `search --rebind` refusal is no longer tested anywhere (C154).**
- **Where:** `crates/acquisition-cli/src/search_cmd.rs:341-343` (the `rebind` branch of what was `unbuilt`), and `crates/acquisition-cli/tests/search_json.rs:886-905`.
- **What's wrong:** the retired row `(["search","--fields","name"], "not_built")` was the only CLI test that sent `search` down its not-built branch. That row now pins `unknown_name`. Grepping `crates/acquisition-cli/tests` for `rebind` finds nothing. So "a not-built flag on `search` is refused by name, kind `not_built`" has no holder at the adapter.
- **What the retirement line covers:** it names the `--print-request` row, which proves `--fields` works, not this property. The wording is still held lower down (`acquisition-search/tests/refusal.rs:44`), and `show --basis` still pins the kind, but for `show`, not `search`.
- **How it shows:** change the `args.rebind` branch to return any other kind (or delete it so clap's flag becomes a no-op) and no test fails.
- **Fix:** add `(vec!["search", "--rebind"], "not_built")` to the `c11_` table (C153: joins the existing table), or name the real holder in the retirement line.

**2. An empty field list is accepted in some places and counted as "typed" in others.**
- **Where:** `answer.rs` `RowField::list` (lines 676-688; empty pieces are filtered out); `search_cmd.rs:345-348` (the list is read) and `:359` (`("--fields", args.fields.is_some())`).
- **How it shows:**
  - `acq search --count tab --fields ''` (or `--fields ,`) runs with no refusal, while `--fields item` there is refused with kind `view`.
  - `acq search --next T --fields ''` is refused as a typed flag that has "nowhere to stand".
  - On the page, `search?…&view.fields=,` folds to `Some([])`. That renders the compact table with no art and no presentation link; `asked_by_the_form` maps `[]` to the default, so the query box and the routes send the user back to cards.
- **Fix:** pick one rule. Either refuse an empty list as a usage error ("`--fields` names no field"), or treat it as absent everywhere, including the `typed` check. Add the case to the `c140_` reader assertions and the fold's unit test.

**3. The card writes the "is" line's join itself instead of calling a word function (C46, Do 7).**
- **Where:** `crates/acquisition-cli/src/web/page.rs` `card`: `<p class="flags">is {it.flags.join(", ")}`. The CLI's text does the same join separately at `search_cmd.rs:2505-2507`.
- **Why it matters:** Do 7 asks for one maker per wording. This is a second spelling of the `is` words, carried over from step 4's item page (`fact_row("Is", …)`) rather than introduced here. If one join changes, the card and the text will drift apart.
- **Fix:** add a `flags_words(item)` in `search_cmd.rs` and call it from both.

**4. The 1,000-row figure in M9 can no longer be re-measured at HEAD.**
- **Where:** `crates/acquisition-search/examples/rows-item.rs:1-8` and `search/MEASUREMENTS.md` M9.
- **How it shows:** since the cap landed in `60c3a1d2`, the example's N=1000 ask returns 200 rows (its own `rows` column will print 200). Its doc still says it measures 20, 200 and 1,000.
- **Fix:** one sentence in the example's doc and in M9 saying the 1,000 row was taken before the cap, at `b90102be`, and is re-taken only at that commit.

**5. Four commits in the range fail CI's doc stage (for the record, not a gate failure).**
- `b90102be` linked the crate-private `ShownItem::of` from public docs, so `b90102be`, `720a68cf`, `60c3a1d2` and `634cfac7` each fail `cargo doc`. `e26e18aa` fixes it.
- The gate doesn't run the doc stage and CI runs only on the pushed head, so nothing breaks. I did not run `--only doc` myself.

**6. Raised for the owner (behaves as the brief designed, but worth his eye):**
- **The cap is silent outside JSON.** `acq search --limit 500 --fields item` prints "rows 1–200 of N shown · more …". Only `view.rows.limit` in the JSON says 200 was applied, and neither the text nor the page says 500 was lowered. This meets C53's new line as written.
- **The text layout under `item`.** The item block is unindented, as the brief said, so its `base`/`class`/`price`/`lines`/`unread` labels sit at column 0 beside the answer's `total`/`terms`/`basis`. The only boundary between rows is a blank line.
- **Redundant "more" pointers.** Under `item`, a row's "N more" pointers (a `show` command at the terminal, an `item/<id>` link on a card) still appear even though the whole item is printed right there.
- **MCP.** `fields: ["item"]` reaches the MCP's `search` through the search crate's reader (`RequestParams`, `acquisition-mcp/src/main.rs:773`). Up to 200 rows, about 0.57 MB compact, per call, with no byte bound. No tool text and no `describe` key (`rows`, `view`) names `fields`, `item` or the cap, so an agent only meets them inside a continuation. The brief already raised this.

### Checked and found correct
- **One maker, `ShownItem::of`.** Called in `read_one` (`show.rs:438`) and in the row literal (`answer.rs:2185`). `grep -rn "ShownLine {\|categories.shown(" crates/*/src` finds only `show.rs`: the definition and the maker's lines.
- **No second evaluator in the page.** The card reads only `ItemParts`, `row.place` and `row.icon`. Card or table is chosen from `rows.fields` in the document, never from the URL. `show`'s goldens are unchanged.
- **The cap.** `.min(ITEM_ROWS)` applies only where `fields` contains `Item`. A smaller limit stands. The continuation carries the applied limit and the same fields. At exactly 200 rows `left_out` is 0 and there is no `next`; at 201 `left_out` is 1 and `next` returns the last row with its item (the cap's pin).
- **Escaping.** Applied once (`enc`, then `esc` on the href). The presentation link is a plain `<a class="presentation">` with no `way` class or `data-way`. The way check and the no-command test pass on every page.
- **MCP surface.** No ruling id in any user-facing text (`no_ids` green). No new route, no virtualiser. `MCP-REFERENCE.md` and `surface_size.rs` are unchanged and their tests pass.
- **CLI goldens, read for meaning.** `describe_at_a_terminal` loses only the `fields` not-built row. `an_items_detail` gains the `--fields icon,item --limit 2` case: head titled with its frame, the `icon none` line, then `base`…`unread` with the why command naming the account.
- **Web goldens.** Exactly the four named moved, plus `search_form.txt` by one control. `console_realm`, `jobs_halted` and `refresh_loop` are untouched. No `remove("icon")` is left in `web.rs`.
- **The retirement line.** Present in `634cfac7`. Its holder runs in the gate (`search_json` is in the `main` binary) — but see finding 1.

### Done-when: 11 items
- **Met (10), each checked against the tree:**
  1. `NOT_BUILT` holds `rebind` and `show basis` alone, `RowField` has `Icon` and `Item`, `ITEM_ROWS` is 200 beside `LISTED` citing M9, `answer.rs`'s comment names both numbers, the help number is under a test, and the owner's word is cited in `60c3a1d2` with the line itself in `6bd852fe`.
  2. The maker grep and both calls.
  3. The `rows.rs` pins, `explain.rs`'s table and the `c11_` table hold the new refusals.
  4. M9 is recorded and the example builds.
  5. `show`'s JSON is unchanged; only the two CLI goldens moved.
  6. The `--print-request` row's equality passes and the retirement line is in a commit message.
  7. The web goldens moved and held as listed.
  8. Both README tour lines say what Do 6 says; `readme_tour` green.
  9. The card in `page.rs`: one `card` function for rows and the item page, no whole-`Shown` word call, the results table only under `view.fields=icon`.
  10. `MCP-REFERENCE.md` and `surface_size.rs` unchanged.
- **Not verified (1):** "`tools/gate.sh` exits 0". I was told not to run it. Every gate stage except clippy and the whole-workspace nextest ran clean in the subsets above.

### Verdict
Nearly clean: the step builds what the brief asks, and the one-maker, cap, presentation and golden properties hold. A short fix round should:
- add the `search --rebind` → `not_built` row to the `c11_` table (finding 1);
- settle one rule for an empty field list on the CLI and the page, and pin it (finding 2);
- move the `is` join into a `search_cmd.rs` word function both the text and the card call (finding 3);
- add the M9 / example note on the pre-cap 1,000-row figure (finding 4);
- run `tools/gate.sh` and `--only doc` before the push.

Finding 6 goes to the owner as raised; no code change is owed for it.
