Review of `roadmap/briefs/4-full-1-cards.md` (cut 3) at commit `3d226f73`, read cold against the tree as it stands.

**What checked out.** I checked these against the tree and found them correct:
- Every line number the brief cites: show.rs 69/238/252/~339; corpus.rs 357/977; answer.rs 599/621/1485–1530/1642; search_cmd.rs 197/339/557/2403; form.rs 68/717; mod.rs 548/884; web.rs 373/374/392/393/578/581/638/639/642/763/843/845/851/1175/2916; search_json.rs 889/952–958/2088; explain.rs 736–740/~825; surface_faults.rs 724; describe golden line 318; README 195/203.
- The golden counts. `class="results"` appears 5× in search_pages, 8× in saved_pages, 2× in moved_world and 4× in answer_ways, at the lines named, and the zero, count and refusal pages are as stated.
- The spike's numbers against its run file. 128,387 + 438,085 = 566,472; 635,140 + 2,266,863 ≈ 2.9 MB.
- The copy's SHA-256 (`fbae8786…`).
- The CI runs and their head commits. 4b28dad1 is still the last push.
- The MCP figures: 43,000, 42,089, 7,500 and 7,401.
- That no field the new row keys add is named `request`. derive.rs, price.rs, category.rs and class.rs have none.
- That `ways_on` panics without `data-way`.
- That `categories.of` and `categories.shown` never read `item.lines`, so the maker can take `Held.category` safely.
- That the explain-branch and the c143 table can take the new refusal.
- That serde's refusal of `["price"]` in `c140_` is untouched by adding `Item`.
- That the `--saved` and `typed` claims hold, and that `asked_by_the_form`'s last lines replace the offered fields.

**Findings**

1. **The order of Do 6 and Do 7 does not compile as written. `price_words` takes a whole `Shown`, and Do 6 needs it on a row.**
   - Do 6 splits `shown_text` "so the item's block is one function both call". That block runs from `base` to `unread` and includes the price line, which calls `price_words(s: &Shown)` (search_cmd.rs:2606; called at 2443).
   - A row carries the five keys, never a `Shown`.
   - The brief retypes `price_words` (with `shown_title` and `shown_place_words`) only in Do 7, which comes after Do 6 in Do 10's order.
   - A worker following the order hits a type error in Do 6's commit, or retypes ad hoc and leaves Do 7 describing work already done.
   - **The brief should say:** retype `price_words` to the parts it reads (the item's id and the `Priced`, or Do 2's struct) in Do 6's commit, or in Do 2's. Do 7 then names only `shown_title`, `shown_place_words` and `item_badges`.

2. **Do 10 places no commit for Do 7's golden moves, and Do 7 alone moves the item pages.**
   - Do 7 renders the item page as card plus facts table whatever the page asks. So the commit that lands Do 7 changes `GET item/r1` in `search_pages.txt` and both item pages in `moved_world.txt`.
   - It also breaks the holds at web.rs:845 (`"<th>Price</th><td>none ("`) and web.rs:851 (`find("<th>Price</th>")`). The price leaves the facts table.
   - Do 10 says only "the commit that changes the goldens" (one commit). It sets Do 8 against the JSON comparisons, but says nothing of where Do 7 falls.
   - A worker who commits Do 7 apart from Do 8 leaves web tests red, or regenerates two goldens twice without being told to.
   - **The brief should say:** either Do 7 and Do 8 land in one commit, or Do 7's commit regenerates the item-page entries and rewrites the 845 and 851 holds, and Do 8's commit regenerates the answer pages.

3. **(Judgment) The cap is a second row number in answer.rs, and the code says there will be none. It also lowers a `limit` the request named.**
   - answer.rs:342–346: the rows default "is the store's one number (C53, the read economy's bound: `acquisition_store::bound::LISTED`), never a second one here."
   - bound.rs:11–15 (C53's home): "A request's `limit` names more or fewer."
   - Do 5 puts `ITEM_ROWS` in answer.rs and silently lowers a named `limit`. Do 1 says "No document field changes meaning."
   - Under `item`, `limit` stops meaning what C53's text says: a named number of rows.
   - **The brief should say:** where `ITEM_ROWS` lives. Beside `LISTED` in `acquisition_store::bound` is C53's "held once" home. If it stays in answer.rs, Do 5 rewrites the comment at answer.rs:342–346 and says why a ceiling is not the default number.
   - Separately, the foot should ask the owner explicitly whether lowering a named `limit` needs a C53 line. Today Do 1 asserts it needs none.

4. **Do 2's suggested shape breaks tests that read `Shown`'s fields as Rust fields, which the brief says stay unchanged.**
   - crates/acquisition-search/tests/sockets.rs:453 (`.map(|s| s.item)`), :559 and :590 (`.unwrap().item`) read `Shown.item` directly.
   - With the five moved into a flattened struct they stop compiling.
   - Do 2 says "every test and golden reading `show`'s JSON or text unchanged is the proof". These three read neither, so the worker meets an edit the brief does not name. The same applies to every `s.item`, `s.lines`, `s.class`, `s.category` and `s.price` in search_cmd.rs and page.rs.
   - **The brief should say:** name sockets.rs:453/559/590 as edits the shape needs (a field path, not a behaviour). Or suggest keeping `Shown`'s five fields and building them from the maker's struct.

5. **The CLI text under `item` drops the frame.**
   - Do 6 says the title is "not printed again … (the row's head says them)".
   - `shown_title` adds `· frame <frame>` (search_cmd.rs:2537–2546; the golden shows `· rare · frame rare`).
   - `row_text`'s head prints the title, rarity, realm under `all`, place and removal, never the frame (search_cmd.rs:1278–1290).
   - Where frame differs from rarity (currency, gems, foils), the item's frame then appears nowhere in the text.
   - **The brief should say:** under `item` the head or the block says the frame (C53: text is a function of the JSON), or the drop is accepted and listed for the owner.

6. **The red Linux run is not filed, and the template asks that it be.**
   - TEMPLATE.md: "A red run is this step's first concern or is filed, and the brief says which and where."
   - The brief says the orchestrating session will file it "before the build starts".
   - At 3d226f73, `gh issue list --label trigger:ci` still shows only #179 and #150 open. Nothing holds `refresh_loop.txt`'s 4-versus-5.
   - **The brief should say:** the issue number, once filed. The foot's "to be filed" becomes that number.

7. **Do 4's hash check names the wrong file.**
   - "a fresh `sqlite3 -readonly … ".backup …"` … under your scratch (its SHA-256 checked against Input's before and after)" reads as the backup's hash.
   - Input's hash is the source file's. A `.backup` copy is written page by page and need not hash the same, so a worker who hashes the backup may stop for nothing.
   - Also: the spike's compact answer is unaged (the spike calls `acquisition_search::answer` without `.aged`), while its five keys come from `show`, which is aged. Do 4 should say whether M9's example ages its answer, so the check compares like with like. The difference is small, well inside the tenth.
   - **The brief should say:** "the source's SHA-256, before and after", and whether M9 ages the answer.

8. **Small citation errors in Read that would send a worker to the wrong place.**
   - C108 is in `decisions/store.md:15`, not `decisions/search.md`.
   - `view` is rows.rs's own helper (rows.rs:104), not one of `tests/common/mod.rs`'s.
   - common's `load` and `show` each open a fresh, empty in-memory intent (common/mod.rs:186–205).
   - So the equality pin's "with the same intent file" (an item priced by a written price) cannot use those helpers. It needs `load_with(&s, &intent, …)` and `acquisition_search::show(&s, &intent, …)` over one `Annotations`.
   - **The brief should say:** both facts, in Read and in Do 9's first pin.

Nothing else I could support from the tree would change the brief.
