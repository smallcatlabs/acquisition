`roadmap/briefs/4-full-1-step6-table.md` at `9ffa164d`: a builder needs the rendering and first-block integration decisions below before starting as drawn.

Findings, most severe first:

1. **Decisions 1 and 4 — changes what gets built.** Reusing the rig’s renderer can erase uncertainty that the production table must preserve. In `rig-3.patch`, the window renderer prints `Valued.value` without its accompanying `status` whenever a number exists, so `{value: 40, status: "incomplete"}` appears as an exact `40`; it also renders both `Priced::None` and `Priced::Open` as an empty price cell. This conflicts with C94’s requirement to mark incomplete subtotals and loses distinctions that `search_cmd::row_shows` and `price_words` already render. Smallest change: explicitly require preservation of value and price statuses, reuse the existing words, and require goldens covering incomplete values and unresolved prices.

2. **Decisions 2–3 — changes what gets built.** The brief does not explain how the initial table gets item data for prices while remaining a table. Today `page::answer` chooses cards whenever `rows.fields` contains `Item`; `rows_links` and `query_box` also use those fields to identify the presentation. Adding `Item` therefore changes the presentation, while leaving the initial request unchanged leaves its rows without prices. Smallest change: specify how the original presentation request is retained while a bounded item-bearing window at the same basis supplies the table, and name the renderer inputs that distinguish them.

3. **Decisions 1–2 and 8 — changes what gets built.** The measured script cannot adopt the server-rendered first block required by this brief. Its segment initialization always creates an empty `blocks` map, and `ask` appends a new block without recognizing existing markup. A literal transplant leaves the initial rows unmanaged and fetches another first block. The prescribed middle/end checks can miss the initial duplication. Smallest change: specify that initialization registers the server-rendered block as completed and subject to eviction; check the initial viewport and a return to the top as well.

4. **Decision 3 and Done when, second bullet — changes what gets built.** The JSON equality criterion omits the request normalization needed to make it true. For a valid continuation requesting only `icon`, `rows` must add `item`, whereas `search?next=<that-token>` does not; the documents therefore differ even at the same offset. Different limits also produce different windows. Smallest change: define the comparison request explicitly—decoded token with absolute `after`, requested `limit`, and item fields applied—and compare `rows` with `search?next=` carrying that normalized request’s token.

5. **Decision 8 — changes what gets built.** “A bound that does not grow with the table” has no specified criterion, and the proposed test uses only one table size. A builder can choose a bound equal to that fixture’s entire row count and retain every visited block while passing the check. Smallest change: require a bound derived from viewport height, row height and block size, independent of total rows, and exercise a table larger than that bound.

6. **Find first, table-test command — changes what gets built.** The command does not find every test it claims to find. It finds the `answer_ways.txt` golden, but misses the table-specific assertions in `the_ways_no_seeded_page_reaches_are_a_wording_golden`: the `view.fields=icon` case asserts “more sorted”, “more of term”, and `item/many`, none matching the supplied pattern. Smallest change: extend discovery to `view.fields` and trace the matching goldens to their producing tests before applying the retirement requirement.

7. **Find first, route command — wording.** `git grep -n "route(&at("` finds route registrations but no module documentation. The documentation instead contains prose and spellings such as `GET saved` and `GET count`; there is no module-doc route list matching that command. Smallest change: describe this command as finding registrations, and add a separate command that reads the module documentation.

What I did not check and why: I did not build, run tests, launch a browser or daemon, inspect live CI, or access GitHub issues, as this review was restricted to the committed tree and read-only discovery. Runtime correctness and hosted-run results remain unverified.

Form: the brief’s shorthand API inventories leave the builder to resolve the important distinction between presentation and row payload.

Prompt: no obstruction.
