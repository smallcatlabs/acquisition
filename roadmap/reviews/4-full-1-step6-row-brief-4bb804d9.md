`roadmap/briefs/4-full-1-step6-row.md` at `4bb804d9`: a builder cannot start as it stands; the closing behavior explicitly awaits the designer.

- **Changes what gets built — Open, for the designer.** The brief leaves closing undecided while requiring it to keep the row visible. The section offers server-rendered positioning or script-driven seeking and explicitly says “The build waits.” **Smallest change:** record the designer’s choice before handing the brief to a builder.

- **Changes what gets built — Decisions 3–4.** The toggle solves submission context but leaves an opened row’s appearance undefined after its block is evicted and fetched again. `window.js` drops even server-rendered blocks outside its window; `rows` reconstructs HTML through `page::window_rows` from the answer alone, which carries no opened-item state. Opening, scrolling away and returning can therefore lose the row’s mark and “closes” label while its card remains open. **Smallest change:** specify how every fetched block reconstructs that presentation—for example, carry opened ids on the window URL outside the search token—and require the browser test to scroll away and back before closing.

- **Changes what gets built — Done when, refused query.** The required refusal golden can pass while losing the query box’s carried view. For `search?realm=pc&text=%28&view.fields=icon&view.sort=ilvl&view.value=ilvl&view.desc=true`, parsing fails before an answer exists; existing `query_box` takes those controls from `RowsOut`, and passing `None` restores the text while resetting presentation, order and values. A refusal golden using default controls would miss this. **Smallest change:** require a refused query with explicitly named presentation, order and values, checking their preservation and a corrected resubmission.

- **Wording — Find first.** The page-test discovery command omits two goldens directly affected by this step. Running `git grep -l -E 'GET (item|search)|save-note' -- crates/acquisition-cli/tests` misses `goldens/web/table_rows.txt`, whose rows change from links to buttons, and `goldens/web/refresh_loop.txt`, which holds the place pages. **Smallest change:** expand the route alternatives to `GET (item|search|tabs|characters|rows)`.

Not checked: builds, tests, application execution, browser behavior, live issue contents or CI results. This was a static review of the supplied commit. The grouped implementation could not be inspected: its close lookup returns nothing, and this tree still contains the earlier place-page implementation.

Form obstacle: “6.5b as built” points into an implementation absent from the supplied tree, limiting integration review to its brief.

Prompt obstacle: none.
