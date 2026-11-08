# Cold review of 4-full-1-answer's build, `7c1ce05a`

Read cold on 2026-11-08 by an Opus subagent of the orchestrating
session against `roadmap/briefs/4-full-1-answer.md` (cut 4) and the
tree at `7c1ce05a`. The reviewer ran the web module's tests (26 of 26),
the CLI reference, no-ids, normaliser, search JSON and seat-fault tests
(108 of 108) and `tools/arch-check.sh` (0), ran no gate and edited
nothing. Its report follows, verbatim but for this head; the fix round
and the fates are the roadmap's ledger row.

## Findings, most severe first

**1. Medium: the row "more" links that Do 4 adds are pinned nowhere.**
- Where: `page.rs` `rows_table`, lines ~1063–1075 (`RowPiece::MoreSorted` and `more_of_term_words` become `<a href="item/<id>">`).
- How it shows: no golden under `tests/goldens/web/` has "more sorted" or "more of term", and no web.rs assertion does either (`grep -rn "more sorted\|more of term" crates/acquisition-cli/tests/` finds only `search_json.rs:171`).
- The text side of the same split has no golden either. That is `row_text`'s rewritten `RowPiece::MoreSorted(n) => "{n} more sorted: {show}"`. Only "more of term" is held, by one `search_json.rs` assertion.
- So a fault in the new `RowPiece` split, its dedupe keyed on `RowPiece`, or the link itself would pass the gate.
- Fix: stage a row that leaves lines out in `the_ways_no_seeded_page_reaches_are_a_wording_golden`, for example an item where a term touches more lines than it shows, or a sort with `left_out`. Record its link in `answer_ways`, and add the text case for "more sorted" to a CLI golden.

**2. Medium-low: "every CLI text golden is unchanged, byte for byte" covers only the paths a golden holds.**
- The refactor rewrote these text paths, and none of them has a golden outside `web/`. I checked by grepping every golden outside `web/`, at both the parent commit and this one:
  - "N more: --explain <path>…" (`more_parts_words`)
  - `then    … where this route was printed` (`before_words`)
  - the trade "… and N more: the JSON's query.trade.clauses" line (now `format!("{word:<9}")` over `clauses_listed`)
  - "and held no item" (`REMOVED_HELD_NONE`)
  - "N more sorted: acq show …"
  - price "· from item/…" and "price   undecided:" (`price_words`)
  - "N reasons more: the undecided route below …"
- I read each one before and after, and each is equivalent. That includes the 9-wide status padding: "NOT RUN" padded to "NOT RUN  ", "reported" to "reported ".
- Nothing is broken today, but the commit's evidence does not reach these paths. Before the commit, the web goldens' `<pre>` was not holding them either.
- Fix: narrow the claim in the close, or add the cases. This is cheapest folded into the CLI goldens per C153.

**3. Low: Do 2's "one maker per wording" is not complete.**
These facts are said by both renderers but composed twice, as inline `format!` in `page.rs` mirroring `search_cmd.rs`:

| `page.rs` line | Wording | Composed again in |
| --- | --- | --- |
| 1150 | a kind's `"{kind} {count}"` | `bucket_text` |
| 1157, 1207 | `"{kind}: {needs}"` / `"{label}: {needs}"` | `bucket_text`, `cross_text` |
| 1235 | the margin's `"{n} none"` | `cross_text` |
| 1247 | `"undecided: {tally}"` | `cross_text` |
| 1325 | a suggestion's `"{term} ({n items})"` | `answer_tail` |
| 1118, 1200 | `"sum {summed}"` | the text's |
| 1261 | explain's `"{path}  {node}"` | the text's |
| 1519 | the item's `"{realm} · {place}"` | `shown_text` |
| 1535 | `"open {…}"` | `shown_text` |

`page.rs`'s module doc ("each fact in the words the CLI's text says it in, made once in `search_cmd.rs`") claims more than this delivers. Fix: factor these into word functions, or soften the doc.

**4. Low: the commit message carries "Closes #176".**
- A push to main auto-closes #176 before the orchestrating session adds the comment the brief's Done-when asks for.
- The message also names only #182 for the remainders. It does not name #190 (a refusal keeping its input), `failure_words`, or `console_realm.txt`'s "run `acq stashes …`". That naming is the close's job, and I cannot see the report.

**5. Low: the way check skips some 200 answer pages, and one inventory part is not recorded.**
- `every_request_an_answer_carries_is_a_way_on_its_page` runs beside every `pages.page` answer of the two golden tests, which is what the brief asks.
- It does not run on two kinds of 200 answer pages the same test fetches:
  - the pages reached through refusal links in the realm loop (`answered`, web.rs ~868);
  - the `listed` page reached from `item/t1`'s offer.
- Nor does it run on the answer pages of `moved_world` or `saved_pages`.
- Inventory row 7's kinds (`…/buckets/<b>/sources/<k>`, `/flags/<k>`) appear on no recorded page; only a bucket's own way does.
- Fix: call the check beside those followed answers; it costs nothing.

**6. Low: `RESTS_NO_FORM_READS = []` rests on one key's evidence.**
- The only rest exercised is a `name` key.
- `rest_of` also keys `line:{text}` and `line~{text}`. `asked_by_the_form` writes `view.count` as `counts.lists().join(",")`, and the fold splits on commas (the test's `line:resist,life` case).
- So a template containing a comma (e.g. "#% increased Attack, Cast and Movement Speed") would not fold back. It would render as a span, which the empty allowlist then fails.
- Fix: raise this, or stage a line-group rest.

**7. Low: the second retirement line could name a holder.**
- It says the property is gone, which the brief permits.
- But `an_answer_or_item_page_prints_no_command_line_and_no_flag` would catch a returning `--account u-web` on the item page, since " --a" trips it. That test could be named as the holder instead.

**8. Low: facts the page states differently, all in the brief's scope.**
- Pre-existing, kept by "the rows table as today": under realm `all`, `row_text` prints each row's realm and the page's rows table does not.
- The set-aside row drops "under --context corpus", so the page no longer says which context set the query aside.
- The text's "each returns exactly the members counted" has no page counterpart.
- None of the views involved is reachable from a page today.

**9. Cosmetic.**
- The unrouted sentence ("N more counts carry no link: …") has "more" with nothing before it to count from.
- A line template's three-space padding (`"# to maximum Life"   arg1 95`) collapses in HTML.

## Checked and clean
- **Escaping:** HTML is escaped once throughout.
  - `way_on` escapes the words, the pointer and `{link}{extra}`; `fact_row` escapes its label and is handed escaped cells.
  - The stake's `stake_words` wraps already-HTML ways in plain literals.
  - Queries in the textarea and hidden fields are `&quot;`/`&gt;`-escaped once (seen in `answer_ways.txt`).
- **No mismatched hrefs:** in every recorded way the pointer matches its holder, which the check enforces against the JSON. The hrefs I read in the goldens are the holder's request (total and term undecided `undecided(…)`, the removed `membership=all`, the rest `view.count=name`, the stake's drops query).
- **No unspoken request:** every request the document carries is a link or a span.
- **No ruling id** in page text.
- **No command or flag** on any answer or item page (the sweep reads 28 such pages).
- **Continuations:** `basis_changed`'s offer strips basis and after (`answer.rs` ~1790), so no refusal meets the new continuation arm.
- **Scope:** "Not in scope" is respected; the stat shows no search, verbs or MCP crate touched.
- **arch-check** passes, and the `search_cmd::` items `page.rs` calls are exactly the commit's list.

## The brief's "Done when": 9 items
- **Met, 7:**
  - The grep: only word makers and `trade_realm`, all listed; no `answer_head`, `answer_tail`, `answer_more`, `shown_text`, `printed::` or `command()` in `web/` (ran the grep).
  - Goldens outside `web/` unchanged (`git diff --stat` over the commit).
  - The `Paged` continuation arm, every way through `spell`/`Paged`, and the tabs way as `Way::Tabs` (read the code).
  - The no-command property, and the four other goldens unchanged (the test passes; numstat lists only the four web goldens, matching the message's +/- counts).
  - The tests in `web.rs` under the commit's names, no new seed function, and the Input's tests changed as it says (read the diff and the tests).
  - Docs, CLI-REFERENCE and `cli_reference` green (read them; the test passes). The doc's "made once" claim is the note in finding 3.
  - The C154 lines: each Y exists, runs under nextest and passes.
- **Partly met / not verifiable from the tree, 1:** #176's close naming its remainders (finding 4).
- **Not verified, 1:** `tools/gate.sh` exits 0. I was told not to run it; the subsets above pass.

## Verdict
Not clean, but close. Nothing is broken on any page the goldens hold, and the text renderer is equivalent by reading. A fix round should:
- record a row-more link on a staged page and add a CLI case for "more sorted" (finding 1);
- add, or narrow the claim over, the text paths with no golden (finding 2);
- fold the duplicated word formats into `search_cmd` functions, or soften `page.rs`'s doc (finding 3);
- call the way check beside the followed answers in `every_search_page_is_a_wording_golden` (finding 5);
- decide whether "Closes #176" stays in the message, and make sure the close names #190 and #182's remainders (finding 4).

Findings 6–9 are for the owner's raised list.
