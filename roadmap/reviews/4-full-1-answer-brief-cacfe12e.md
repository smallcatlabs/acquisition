Review of `roadmap/briefs/4-full-1-answer.md` (cut 1) at commit `cacfe12e`, read cold against the tree as it stands.

The brief's counts, line numbers and citations mostly check out against the tree. That covers the eleven `acq …` lines in `search_pages.txt`, saved_pages line 64, moved_world lines 19, 45, 121 and 123, and the four at `7a72de85`. It also covers the `web.rs` line numbers its grep names, the two `Way::Search` sites in the verbs, the eight `GET search?` pages in `saved_pages.txt`, CI runs 42490674447 and 42490674446 (both success on `35cf33a7`), and the #190 quote. The findings below are what would still stop or mislead a worker building from the brief alone. They run most severe first.

1. **Do 3 / Input "The tests the step changes": routing the continuation through `Paged` breaks two tests, and the brief names neither break.**
   - *What is wrong.* Every link `Paged::request` makes goes through `Web::link`, which returns an absolute path with the token first (`crates/acquisition-cli/src/web/mod.rs:489-498`). Today's continuation is the relative `search?next=…` (`crates/acquisition-cli/src/web/page.rs:818-823`). The tests that follow it add the token in front themselves:
     - `c16_every_page_s_basis_line_is_its_documents_own` follows `at(&more)`, which is `/{token}/{path}` (`crates/acquisition-cli/tests/web.rs:1795`, `:1812-1817`).
     - `saved_open_runs_the_rows_search_and_writes_nothing` follows `format!("/{token}/{more}")` twice (`web.rs:2395-2403`). The brief does not name this test at all.

     With the link made by `Web::link`, both tests request `/<token>//<token>/search?…`, which no route answers.
   - *Why the link cannot stay relative.* `Paged` also spells the ways on refusal pages served under other paths. A relative `search?next=` on `saved/open` resolves to `/<token>/saved/search?…`, so Do 3's "refusals included" needs the absolute form.
   - *What the brief should say.* The continuation is `Web::link("search", [("next", token)])`, absolute like every other `Paged` link. Both tests drop their own token prefix: `web.rs:1813`, `:1816`, `:2399` and `:2402`. The Input entry for line 1812 should say this, not only that `link_saying` still finds the words "more".

2. **Input "The tests the step changes": three assertions on these pages go unnamed. One will fail and two will silently stop pinning anything.**
   - `web.rs:742`, `assert!(!counted.body.contains("<table>"), "a count shows no rows")`. Do 4 asks for "an HTML table per key" on a counted answer. A bare `<table>` fails this test. A table with a class passes, but then the assertion no longer tests what its message says.
   - `web.rs:655`, `assert!(!edited.body.contains("trade   from"))`. This pins "an edited text runs as the user's own query, the trade input dropped". Once the opening is HTML, "trade   from" appears on no page, so the assertion passes whatever the page does.
   - `web.rs:757-760`, the item page's `!item.body.contains("--account u-web")` with its comment about the why line (the page seat's fault 13, `2fcd2cfc`). It becomes empty once the why line goes. The plumbing that served it is then dead:
     - `page::item`'s `account` parameter (`page.rs:850`), which is used only for `shown_text`'s why line;
     - the item handler's `account.entry.username.clone()` (`mod.rs:980-990`).
   - *What the brief should say.* Name all three:
     - rewrite 655 against the HTML's own words for the translation;
     - rewrite 742 to test that no results table is present (`class="results"`) and require the count tables to carry their own class;
     - delete 757-760 and the account plumbing together with the why line. Its property is gone rather than moved to another holder, so say so in the commit.

3. **Do 7 / Do 8: the list of pages to add misses rows a page can reach, and the two lists disagree with each other.**
   - Do 7 adds pages only for "a never-fetched tab, a cut value list with its rest, a removed id". No golden under `tests/goldens/web/` records any of these:
     - a continuation (row 1): `grep -rn "more</a>"` over the web goldens finds nothing;
     - a count bucket that carries a route (row 7): the counted page at `search_pages.txt:258-288` has rare and unique buckets, neither of which carries a route;
     - an ambiguous clause's adds and drops (row 10).
   - Each of these is reachable. `view.limit=1` gives a continuation, as `web.rs:1811` does. `view.count=class` over the seed puts `new` (Wombgift Chalice) in an `undecided` bucket, which carries its route by default. A form filter whose fate is `ambiguous`, such as DPS (`search_form.txt:18`), gives an ambiguous clause, if an item makes the other reading differ.
   - Do 8's way test lists "a page with rows left out" among "each answer page Do 7 records", but Do 7 records no such page. As written, the continuation's HTML, its `data-way="/view/rows/next"` and the appended `saved`/`saved_revision` are held by no golden.
   - *What the brief should say.* Do 7 records a page for rows 1 and 7, and for row 10 where the seed can make one or says why it cannot. Do 8's list should then name exactly the pages Do 7 records.

4. **Input "The facts the tests stand on" / Do 7: the worker cannot stage these worlds as written without changing every page recorded after them, and the cut list cannot come from the seed.**
   - *Staging changes the shared world.* The new pages are to be "recorded in `every_search_page_is_a_wording_golden`", which records pages in order over one world (`web.rs:599-845`). The server's corpus holder reloads once the store moves (`mod.rs` module doc, "One corpus, held"). Each staging therefore changes every later page in that test:
     - a re-recorded `pc/Standard` stash listing with a never-fetched tab turns the scope line of every later pc answer from "0 locations never fetched" to 1;
     - a later `t1` listing that drops an item changes the totals and term counts.

     That would break Do 7's rule that each golden diff holds "the answer and item pages' new markup and nothing else".
   - *The cut list needs staged items.* A value list carries `rest` only when `resolved.more > 0` (`crates/acquisition-search/src/answer.rs:1968-1972`), which needs more than `bound::LISTED` = 10 values (`crates/acquisition-store/src/bound.rs:16`). It also needs a term `rest_of` keys: a field test, or a line group with one template (`answer.rs:2722-2751`). The seed holds seven items (`web.rs:32-133`), so "or the build says which seed value makes one" cannot happen.
   - *What the brief should say.* Stage after the last existing page of that test, in a stated order, with each later staged page's counts read in the light of the earlier stagings. Or stage each in its own `offline_web` world, recorded into the same golden from a second test. For the cut list, stage eleven or more distinct values under a `class:` or `line~` term.

5. **Do 4 / the inventory: "its count as a way" over-promises for most counts, and the pointer the test keys on is described two ways.**
   - *Most counts carry no route.* By default only the counts a next step hangs on carry a request (C115; `answer.rs:148-160`; `search_cmd.rs:1568-1655`): undecided, together, matched where nothing matched, a key's `none` and `undecided`, the forced counts, and the adds and drops. A value bucket and a vocabulary row's kinds (`OfKind`) have none. Do 4 says "each bucket its label, its count as a way (row 7)" and "its kinds each with its count as a way". Do 8, meanwhile, forbids any `class="way"` element that names no request.
   - *The pointer is described two ways.* Do 3 says `data-way` is "the JSON pointer of the request", but its examples (`/terms/1/undecided`, `/view/rows/next`) and Do 8 ("that value's parent pointer") use the parent. The inventory's "Its field" column then names `.buckets[].count`, `.cells[].count` and `.sources[].count`. Those `Count`s are flattened (`crates/acquisition-search/src/counts.rs:350-356`, `:381-389`, `:398-403`), and so are `Together` and `Removed` (`answer.rs:1360-1368`). The `request` therefore sits on the bucket, cell, kind, together or removed object itself, and `count` there is a number.
   - *What the brief should say.*
     - A count is a way only where it carries a `request`; otherwise it is a plain number with no class.
     - `data-way` is the pointer of the object that holds `request`, for example `/view/counts/tables/0/buckets/2`, `/terms/1/together`, `/terms/0/removed`, `/terms/0/resolved/rest`, `/zero/offers/explain/0`.
     - Fix the column to match.

6. **Do 2: the number of routes left out is a count the renderer computes, and the brief does not ask for it to be factored.**
   - The "(5 more: --routes)" number does not come from the document. `answer_tail` computes it: it sorts every `Count` into `first` and `rest`, then counts the non-zero ones that carry no route (`search_cmd.rs:1568-1660`). Do 4 asks the page to say "their number in words", but Do 2's list of things to split names only functions that make words, and the module doc's "a page … composes nothing" (`page.rs:1-11`) holds.
   - *What the brief should say.* Factor the enumeration out of `answer_tail` (each count with its label and whether it carries a route) into one function that both renderers call. The same function also gives the page its list of ways.

7. **Do 3: the brief's test for a continuation ("a request with a `basis`") is not the test that `search?next=` accepts.**
   - `Request::from_token` accepts only a request with a place, meaning rows `after` and a `basis` together (`answer.rs:788-803`, `:833-836`). A `basis` with no `after` is refused as "no token". `place()` is private to the search crate (`answer.rs:789`), so `Paged` must read `request.basis` and `Rows::after` (both `pub`) itself.
   - *What the brief should say.* "A request with a place (rows `after` and a `basis`)". Any other request goes on to `asked_by_the_form` as today.

8. **Done when / Do 9: the ` --routes` lines are not counted among what is retired, and the grep in Done when is broken.**
   - *The `--routes` lines.* The three goldens hold 14 lines of `routes … (N more: --routes)` with no `acq ` in them:
     - `search_pages.txt:73`, `:109`, `:154`, `:192`, `:232`, `:280`;
     - `saved_pages.txt:62`, `:188`, `:223`, `:258`, `:293`, `:352`, `:528`;
     - `moved_world.txt:119`.

     Done when checks only for "no `acq `", and the retirement line names only the `acq …` lines and the why line. Done when should also say "no ` --` followed by a letter" on these pages (the no-command test's own rule), and the retirement line should include the flag lines.
   - *The grep.* `grep -n "search_cmd::\|printed::\|command()" crates/acquisition-cli/src/web/` has no `-r` and fails on a directory. Once fixed, it also matches `crate::search_cmd::trade_realm`, a URL reader rather than a word-maker, at `form.rs:296` and `mod.rs:869`. Write it as `grep -rn`, and list `trade_realm` as allowed.

9. **Input / Done when: the count of the other web goldens is wrong.** `tests/goldens/web/` holds seven goldens. Besides the three that regenerate, `search_form.txt` exists alongside `console_realm.txt`, `jobs_halted.txt` and `refresh_loop.txt`. "The other three web goldens are unchanged" should be "the other four", naming `search_form.txt`, which `panes_golden` writes (`harness/mod.rs:1674`).

10. **Do 7: a required fact cannot be recorded on several pages.** "Each answer page records at least … a term's undecided count as a link". No such count exists on the trade URL page (`search_pages.txt:123-162`), the xbox zero (`:164-202`) or the counted page (`:258-288`). It should read "where the answer has one".

11. **Input, "No test outside `web.rs` reads a web page": the claim is overstated, and the one other reader matters.**
    - The grep it quotes, `crates/*/tests/*.rs`, names `web.rs` alone. `harness/mod.rs` is not matched by that glob.
    - `no_ids.rs` reads every web golden in `c116_no_user_facing_text_cites_a_record_id` (`crates/acquisition-cli/tests/no_ids.rs:239-262`). Every new label and the markup this step adds to the goldens therefore passes through the C116 sweep.
    - Say so, as a boundary the new goldens must pass.

12. **Do 6: some sentences that become false are not listed.**
    - `answer_head`'s doc says it is "what the web UI's answer page states above its table, word for word" (`search_cmd.rs:1005-1007`).
    - `answer_tail`'s doc says it is "what the web UI's answer page states under its table" (`:1336-1338`).
    - Both functions are `pub(crate)` only for `page.rs`; after the step they can be private.
    - Add these to Do 6, along with `page.rs`'s docs at `:755-759` and `:848-849`, which the brief does cover.

13. **Do 3 / Do 8: the tabs way is underspecified.**
    - `TabsPointer.realm` is a `String` (`answer.rs:2963-2968`), while `Way::Tabs.realm` is the protocol's `Realm` (`crates/acquisition-verbs/src/refusal.rs:45`, `:66-70`). The pointer's fields must be parsed. `pointer.account` is already `account_named()` (`answer.rs:3001`).
    - The pointer also carries `other_leagues`, whose words ("N other leagues hold never-fetched tabs", `search_cmd.rs:1074-1084`) the brief does not mention.
    - Do 8's "absent where it has none" should mean where `tabs_pointer()` is `None`. That is not the same as a scope with no never-fetched tabs: the pointer is also `None` under `all` realms and for leagues the query excludes (`answer.rs:2980-2998`).

14. **Do 4: the value list is ambiguous.** The text shows five values out of the JSON's ten, and its "N more" is `more + (len − 5)`. The exception is a category term, which shows every pick (`search_cmd.rs` in `answer_tail`'s terms loop, the comment above `shown`).
    - The brief does not say whether the page shows five or all of the values listed, so the number on the rest way is undetermined.
    - The trade clauses' "… and N more: the JSON's query.trade.clauses" (`search_cmd.rs:963-966`) points a person at a JSON the page does not show.
    - Decide both, or list both as the step's own design.

15. **Do 8: the no-command test checks one property twice, and its file-reading half races regeneration.**
    - Reading the goldens and checking the pages "as served" pin the same property at the same layer. C122 asks for a behaviour pinned once, and the served pages are already the goldens.
    - Under `ACQ_UPDATE_FIXTURES=1`, nextest runs each test in its own process. The file-reading test can read a golden before `every_search_page_is_a_wording_golden` rewrites it, so the first regeneration run can fail on stale files.
    - Keep one half, or say that the regenerating run is followed by a plain run.

16. **Goal / Done when: closing #176.** The issue's title is "acq web's search and refusal pages render the CLI's text, commands included". Its body names the refresh page's `acq stashes` / `acq characters` lines. After this step those lines still stand on the refresh refusals (`console_realm.txt:49`, `:54`, `:70`, `:75`), and the refusal words stay `failure_words`. The brief reads "Closes when" narrowly, which is defensible. The close, and the comment that closes the issue, should name where each remainder now lives (#182, #190), so the issue is not read as having closed them.
