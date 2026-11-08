Review of `roadmap/briefs/4-full-1-cards.md` (cut 2) at commit `117a72c8`, read cold against the tree as it stands.

**What checked out.** I checked these against the tree and found them correct:
- **Line citations.** Every one I checked is right: `Shown` 69, `Shown::aged` 238, `read_one` 252, `Held` 357, `name_sockets` 977, `Rows` 599, `RowField` 621, `RowsOut`/`Row` 1485–1529, `Answer::aged` 1642, `SearchArgs::fields` 197, the `unbuilt` check 339, `Rows { … fields: Vec::new() }` 552–557, `shown_text` 2403, `LIMITS` 68, form test 717, `asked_by_the_form` 548, `search` 884.
- **web.rs.** Lines 373/374, 392/393, 578, 2916 (`remove("icon")`), 638/639/642, 763, 843/845/851 and the way check at 1175.
- **Other test files.** `explain.rs` 736–740 and the c143 table at 823; `surface_faults.rs` 724; `search_json.rs` 889 and 952–958; the describe golden at line 318; README 195/203.
- **The copy.** Its SHA-256 matches and the file is on disk.
- **CI.** Run ids and conclusions are right. 42496970974 failed the same test. Open `trigger:ci` is #179 and #150.
- **MCP.** `FIRST_TURN` is 43,000 with 42,089 measured. The describe `answer` section is held under 7,500 with 7,401 measured. The `view` schema text names no `fields`.
- **Spike arithmetic.** 128,387 + 438,085 = 566,472 and 635,140 + 2,266,863 ≈ 2.9 MB. The run output matches the table.
- **The disagreements with the design.** C166 is daemon frames only. `Held.category` is the ids. `show` names the parent by id. The deriver's line order is real (`derive.rs:631`).
- **Not-built list.** `not_built`'s fallback branch is as described. No other test or golden names `fields` as not built.
- **The one-maker grep.** It finds exactly three lines today, all in `show.rs`.

**Findings**

1. **The commit order leaves the web equality tests red from Do 3 to Do 8, and the brief never says where their edits land.**
   - **What breaks.** Do 3 adds `RowsOut.fields`, which is serialized whenever it is non-empty. Today the fold asks `fields: vec![RowField::Icon]` for every rows view (`form.rs:305–315`). So from Do 3's commit on, the page's JSON carries `"view":{"rows":{…,"fields":["icon"]}}`, while the CLI's carries none: the CLI cannot ask fields until Do 6, since `--fields` is refused as not built until then.
   - **Which tests.** Four tests compare the view whole and fail: `web.rs:376` and `:400` (`assert_eq!(page, cli)`), `:581` (`page["view"] == cli["view"]`) and `:2918` (`assert_eq!(page, cli, "{id}")`).
   - **The second window.** Do 8 then turns the page's default to `[icon, item]`. So the tests' CLI side needs `--fields icon` from Do 6 to Do 8, and `--fields icon,item` only from Do 8 on.
   - **What the brief says now.** Input and Do 9 describe only the end state ("each CLI side gains `--fields icon,item`"). Do 10's order list covers the count refusal and Do 4/5, but not this.
   - **Why it matters.** A worker committing per concern, as `TEMPLATE.md` allows, commits Do 3, Do 5 and Do 6 on a red web suite without knowing it. Or they discover it at the gate and reorder by guesswork.
   - **What the brief should say.** Either land the `RowsOut.fields` echo in Do 6's commit, with the four tests gaining `--fields icon` there and `icon,item` in Do 8's commit. Or state that each of those commits edits the four comparisons to what the page then asks. Add the rule to Do 10's order list.

2. **The presentation link would break the way check unless it is kept off the `way` class, and the brief says nothing about that.**
   - **What the check does.** `ways_on` (`web.rs:1079–1090`) panics on any element with `class="way"` that has no `data-way`. It also asserts that every way names a request the document holds (`web.rs:1196–1201`).
   - **Why the link fails it.** The "as a table"/"as cards" link is built from the page's URL by the handler, not from a request in the answer. Do 9 says "The way check … stand[s] as [it is] and run[s] on every page [it runs] on today", and adds a page that holds the link.
   - **Why it matters.** A worker who styles the link like the other ways, or renders it through `way_on`, fails `every_request_an_answer_carries_is_a_way_on_its_page` on every rows page with rows.
   - **What the brief should say.** The presentation link is a plain `<a>` with its own class, never `class="way"` and never `data-way`, because it carries no request of the document. That is why the way check stands unchanged.

3. **The brief's per-golden counts of answer pages that move are too high.**
   - **search_pages.txt.** The brief says nine of its twelve `GET search` entries move. Only five render rows today: `grep -n 'class="results"' tests/goldens/web/search_pages.txt` finds lines 64, 101, 148, 243 and 335. Of the other four answers, two are counts ("the league counted by rarity", "every item counted by class"). The other two are zero answers ("an xbox URL from the pc page", "rings named as the undecided chalice is").
   - **answer_ways.txt.** The brief says six move; four have rows (lines 17, 133, 186, 221). "Every ring outside Standard" and "the id that left" have none.
   - **Why those pages cannot move.** Do 8 gives counts and zero answers neither the link nor the hidden field. A count's request carries no fields (`Shape::Counts` has no `rows`). So nothing on those pages changes.
   - **Why it matters.** The Report asks for each golden's diff summarised. A worker expecting nine will hunt for four changes that should not exist.
   - **What the brief should say.** search_pages.txt: five answer pages and `GET item/r1`. answer_ways.txt: four. Each count and zero page is unchanged. saved_pages.txt (eight) and moved_world.txt (two and two) are right as written.

4. **The one-maker line in Done when is already true today.**
   - **The gap.** `grep -rn "ShownLine {\|categories.shown(" crates/*/src` finds `show.rs:99`, `:355` and `:368` today, before any change. So it cannot show that "`read_one` and the rows view both call" the maker. A build where `answer.rs` makes the five keys some other way passes it, as long as it avoids those two spellings: for example, by calling `show` per row, or by building the lines without a `ShownLine {` literal.
   - **What the brief should add.** Two positive checks: the maker's name appears in `answer.rs` (the `Row` making), and `read_one` calls it. The equality pin in `rows.rs` remains the behavioural holder.

5. **The spike's bias statement leaves out three things that move its numbers** (a judgment on how much each matters).
   - **Pretty bytes.** The pretty column serializes each five-key object at depth zero (`spike-main.rs`, `to_vec_pretty(p)`). In an answer each object sits two levels deep, inside `rows` and the row. `acq search --json` prints pretty (`search_cmd.rs` `emit`, `to_string_pretty`), so the pretty figures undercount what a terminal or agent receives.
   - **Serialization time.** The "five keys' JSON" time serializes a prebuilt `serde_json::Value`. The built row serializes `item` through `without_lines` (`show.rs:60–66`), which makes a `Value` per item first, then serializes it. That cost is neither timed nor named among the omissions; the brief names only the clone, the slots and the category words.
   - **Prices.** The intent file is empty (`Annotations::open_memory_for`). So prices come from item notes alone, with none from the owner's hand-written prices.
   - **Effect and fix.** None of these changes the cap, which is chosen by count. They do belong in the bias sentence the brief calls honest. Separately, "86 ms … what `GUI-NOTES.md` §7 said a card per `show` would cost" overstates the match: §7 says "fifty cards would cost about two seconds", about 40 ms each.

6. **Do 7 only covers documents asking `[icon, item]` or `[icon]`.**
   - **The gap.** It says cards render "under `[icon, item]`" and the table "under `view.fields=icon`", with the choice made "by the document's `view.rows.fields`". But `search?next=<token>` renders whatever view the token holds (`mod.rs:893–894`). A token minted by the CLI without `--fields`, or by an MCP agent, carries `[]` or `[item]`.
   - **What the brief should say.** Make the rule total: a card wherever the rows carry `item`, the table otherwise, art only where `icon` was asked.
   - **The matching fold rule.** Do 8 (`asked_by_the_form`) already returns no link for "any other list". That is consistent, but should be said beside the rule.

7. **Input states the helper's planned change as if it were already in the tree.**
   - **The sentence.** "The helper appends `--fields icon,item` to the CLI's arguments itself".
   - **The tree.** Today `trade_answers_as_the_cli` pushes only `--json` (`web.rs:386–387`) and strips `icon` from the page's rows (392–393).
   - **What the brief should say.** "The helper will append …, so all three compare whole." The timing should follow finding 1.

8. **The hidden `view.fields=icon` is limited to answers with rows** (judgment, minor).
   - **The inconsistency.** The query box already carries limit, sort and desc on every rows view, a zero answer included (`page.rs`, `answer`, `query_box(… Some(rows))`). Do 8 withholds the hidden field on a zero answer. So editing a zero answer's query, reached in table mode, switches back to cards while keeping the limit and sort.
   - **What the brief should say.** Keep the rows-only condition for the link, but carry the hidden field wherever the query box carries the rows view's other fields.
   - **A wording slip in the same bullet.** "An empty value the default as the Show control's empty option is": the big form's Show select has no empty option (`search_form`, `LIMITS` only); only the query box's does.

9. **"The `Held` it already builds for `against`" is only true when a query is given** (minor wording).
   - **The tree.** `read_one` builds a `Held` only inside `against.map(…)` (`show.rs:333–343`), cloning the item, place and price.
   - **What the brief should say.** `read_one` builds the `Held` unconditionally and hands it to the maker and, where given, to `against_held`.
   - **What the worker must not miss.** The maker takes the lines out of the item, as `read_one` does today (`show.rs:353–365`). The rows path borrows `Held` from the corpus, so it must clone the item first.
