`roadmap/briefs/4-full-1-form-values.md` at `eb0b8e88`: a builder could start, but would have to resolve the placement ambiguity below.

1. **Done when — changes what gets built.** The completeness check compares the listing against its own new value function, so both can omit supported values and still pass. Concretely, returning no value for `misc_filters.ilvl` would satisfy that comparison and “every offered value is accepted,” while the browser cases could use total elemental resistance throughout; “Item Level” would remain undiscoverable despite the goal. Smallest change: independently compare the function’s result with the bounded translation’s compared value for every candidate, and explicitly pin `Item Level → ilvl`.

2. **Decisions 1, 3 and 7 — wording.** “On each place page” conflicts with placement beside item heads, leaving visibility without heads unspecified. In `web/page.rs`, `places` returns before rendering the list form for an empty league, and obtains item heads only from an open place’s window; Decision 3 also explicitly anticipates pages without heads. Smallest change: qualify Decision 7 as “on place pages where item heads are drawn,” consistent with the design’s “wherever a page says its rows’ order.”

3. **Find first — wording.** The golden-discovery command misses affected editors that currently contain neither `data-finder` nor `view.sort`. Running it excludes `console_realm.txt`, whose line 31 contains the query-box form that Decision 1 changes. Smallest change: add `class="query-box"` to that command’s alternatives, plus a selector for place-list forms if the finder will appear without item heads.

I found no contradiction with the named rulings or the invariants. I ran all seven read-only “Find first” commands; the other commands located their stated targets.

I did not build, run tests, drive a browser, or verify remote CI and issue state because this review was restricted to reading the committed tree. Runtime behavior remains unverified.

The brief’s form caused no additional difficulty. The prompt’s execution restriction was clear when read alongside its explicit permission to run discovery commands.
