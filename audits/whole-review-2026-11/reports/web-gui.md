## GUI slice (`acq web`): scout report

### 1. Model

`acq web` is an axum server inside `acq`. It binds `127.0.0.1:0`, and its routes sit under a 128-bit per-launch token path segment (`web/mod.rs:587-668`). A `guard` layer checks Host against the bound address and Origin (where one is sent) against its own, then sets the CSP, `nosniff`, `no-store` and `Referrer-Policy` headers (`mod.rs:683-733`). A `with_account` layer pins one account in a `OnceLock`. There are 16 routes, plus 5 static assets that are `include_str!`'d.

How a request becomes a page:
- A handler takes URL fields, strips the page-only fields (`from`, switch, picked, opening), and folds the form's dotted-path names into trade JSON (`form.rs`, 1,203 lines).
- It builds a `Request` and gets an `Arc<Corpus>` from the search crate's `Holder`. The store is opened and checked for currency on every request (`mod.rs:1246-1250`). It calls `acquisition_search::answer`, then composes extra reads: the window `cut`, `place_links`, the saved row, `held_leagues` (`mod.rs:1756-2070`).
- It hands documents to `page.rs`, which builds HTML with `format!` and `esc()` (452 `format!`, 298 `esc(` calls) and fills one template by chained `.replace` (`page.rs:184-230`). There is no template type, typed builder or view-model layer.

State lives in the URL alone (C16). A small amount of transient DOM state is held by four scripts (17 KB JS, 11 KB CSS):
- `count.js`: the debounced count.
- `lists.js`: stat rows from server `<template>`s, plus the finders.
- `window.js`: a virtual-scroll table fetching 50-row HTML blocks from `/rows`.
- `link.js`: swaps the site link's league.

The words are the CLI's (`search_cmd`, `store_cmd` helpers), so it is a projection by intent. In practice the handlers compose several documents per page, and the form's field grammar is a fourth request encoding, after the language, trade JSON and the `acq1.` token, that only the web speaks.

### 2. Measurements

| What | Figure | Command |
|---|---|---|
| Source | mod 4,550, page 5,630, form 1,203, fill 581, from 346 | `wc -l crates/acquisition-cli/src/web/*.rs` |
| Prose in code | mod.rs: 480 lines of `//!` module doc, 1,070 comment lines (~24%) | `grep -c '^//!'`, `grep -cE '^\s*///?'` |
| Tests | web.rs: 52 tests in 8,339 lines, the largest single test 751 lines. browser.rs: 16 tests | `grep -c '#[test]'`, an awk span over the fns |
| HTML-substring assertions | 458 `# holds` lines in web goldens, the same strings again as literals in web.rs (~280 `contains(`) | `grep -c '# holds' goldens/web/*` |
| Web goldens | 795 KB in 14 files (`search_pages.txt` 278 KB) | `wc -c goldens/web/*` |
| Page weight | Every search answer page re-sends the whole search form (~64 KB of `search_form.txt`) plus 50 table rows at ~313 B each. A stat-finder answer is 44 KB because each option carries its row template | golden reads, `awk` lengths of the `data-i=` rows |
| Churn since 11-01 | 102 commits on `src/web`. 48 touch both mod.rs and page.rs, 27 mod.rs only, 23 page.rs only. 86 of 102 also touch web.rs or its goldens. Src+tests+goldens: +34.8k −9.9k | `git log --name-only` / `--numstat` |
| Per step (approximate, by commit-message grep) | form-rows +1,417/−396; form-landing +1,887/−454; sitting-round +2,325/−1,878 | `git log --numstat --grep=4-full-1-<step> -- crates` |
| Process | 115 review answers and 31 briefs (7,562 lines) under `4-full-1*` | `ls roadmap/reviews`, `roadmap/briefs` |
| Browser test time | Linux CI: 0.9–5.4 s each. Windows CI: 5–33 s each | `gh run view <id> --log` |
| Timing figures | Form page 25 ms in the mock (Chrome); a count wait ~311 ms, mostly the 300 ms debounce. Nothing measured on the owner's 22k-item store at the page | `audits/web-2026-11/driver/README.md:204` |

### 3. Where a deep review would pay

**H1. C191 versus the direction actually being built (highest value; confidence high; verified).**
- C191 (`decisions/frontends.md:18`, ruled 11-19) says `acq web` "bootstraps development… never reshaped into a high-performance desktop app". It says a GUI laid out like the trade site or the C++ app is "a separate frontend".
- Yet the owner's 11-24 sitting (`audits/web-form/sitting/README.md`, stops 6, 8, 10 and 14) and #285 steer `acq web` toward the trade-site and C++ look. The fix round built resizable columns, sortable heads and league dropdowns into it.
- So the brief's premise ("no ruling states it as a goal") is half right. A ruling exists, and it points the other way from the work.
- Question for the reviewer: is part 2/3's live loop designed for `acq web` (server-rendered, URL-as-state, a full re-render per interaction) or for C191's separate frontend? Every part 2/3 wire decision depends on the answer.
- Payoff: avoids building a subscriber/feed into an architecture the owner has ruled is not the performance GUI.

**H2. The interaction model cannot give C++ interactivity (confidence high on the mechanism; performance not measured).**
- Sort, opening a row (`toggle_item` gives a 303 and a full GET, `mod.rs:1775-1787`), opening a place, and filtering are each a full round trip. Each re-folds the form, reopens the store, re-evaluates the whole query, and re-renders ~64 KB of form plus the answer.
- Each `/rows` block re-evaluates and re-sorts the whole answer to slice 50 rows (`mod.rs:2738-2772`, `Web::window`). C48 forbids a cached ordered result.
- #327 shows the hard limit: name, base, price, location and fetched cannot sort because C46 says only the language may order, and it orders no text.
- Keyboard support is the finder's arrows, Enter and Escape alone (`lists.js`). There are 20 `aria-` attributes in page.rs. Nothing is measured on accessibility.
- `GUI-NOTES.md` §6 already says a C++-grade table on the web costs 1–2k lines of interaction code, or TanStack, plus server windowing keyed by id.
- Question: measure on a 22k store what a sort, a row open and a fast scroll cost, and decide whether part 3's resident (a held ordered result per basis) is the answer.
- Files: `mod.rs` `search`/`rows`/`cut`, `window.js`, `GUI-NOTES.md` §5–6.

**H3. Layer-split files and prose-as-spec make every step expensive (confidence medium-high; judgment).**
- The files are split by layer (handlers in mod.rs, HTML in page.rs), not by page family. So about half the commits touch both 5k-line files, and 84% touch the tests too.
- The natural seams are page families, each with its handler and render together: search/editor (page.rs ~610-1830 plus form/fill), answer/cards/table (~1830-3460), places (`mod.rs:2963-3676`, `page.rs:3460-4270`), plan/jobs (~4270-5327), saved, describe. A shared kit would hold `esc`/`enc`, links, `post_form`, `Paged` and `from`.
- mod.rs's 480-line module doc restates each route's behaviour in prose. That is a second home for the facts, and the assessment counts ~19 findings against comments and docs ("words wider than code").
- Tests assert raw HTML substrings twice (the golden's `# holds` lines and the web.rs literals). So a class rename or attribute reorder touches code, test and golden.
- A view-model layer would make most page tests document comparisons and shrink both the prose and the substring assertions: one serializable struct per page, rendered by a thin template, asserted as JSON.
- Question: would that have prevented the recorded findings? The assessment says mod.rs's 12 findings "share no cause". A reviewer should re-read them against this hypothesis.

**H4. A flaky browser test came back after "closed", and nobody filed it (confidence high; verified).**
- `browser::two_stat_groups_one_weighted_are_made_edited_and_run_from_the_page` failed on Windows CI 5 times before #312's fix `51d99a3d` (11-24 04:50Z).
- It then hung to a 240 s TIMEOUT on the Windows gate at `178d51bb` (run 45547968713, 11-25 04:36Z), a new failure mode. The fix added `Fetch`-domain interception, a plausible hang source (judgment).
- No issue is open on it (`gh issue list --search two_stat_groups`). This is the memory's "flaky gates are fixed when found" rule unmet.
- Every recorded failure is on Windows, where browser tests run 5–33 s against 1–5 s on Linux.

**H5. Second semantics creeping into the page (confidence medium).**
- `page::fullest` gives a fresh form the league holding the most tabs (`page.rs:3487`), while the CLI defaults to Standard (#19). The same empty search answers different leagues at the two doors. C96 permits adapter defaults, but no test or ruling names this one.
- Handlers compose several documents per page (window, place links, held leagues). The semantic-model audit already flagged that pattern (4c/8), and it has since grown.

### 4. Low yield / already covered

- **The security posture is sound.** Loopback bind, a 128-bit token in the path, a Host check (blocks DNS rebinding), Origin refused when foreign, POST-only mutations on POST-only routes, a strict CSP, `Referrer-Policy`, `no-store`.
- **What a page can trigger.** `apply`, the listings and `leagues` reach GGG only through the daemon's door and rails. The apply envelope is a hidden field, trusted as `--apply=FILE` is (the earlier audit says so).
- **One residual worth a reviewer's minute.** POST with no Origin is accepted, so CSRF rests on the token alone. That is acceptable.
- **The template's chained `.replace`.** I checked injection: nav URLs are percent-encoded, so braces cannot inject.
- **Not worth redoing.** The semantic-model audit's cli-web findings (picker.js since replaced) and the C16 lexical scan (already a checklist row, `REVIEW-CHECKLIST.md:119`).

### 5. Blind spots

- **No page-level timing on a real-sized store.** Nothing measures a sort, a row open or a scroll on 22k items. There is no budget and no measured TTFB or weight per page; the goldens record only `<main>`.
- **Two held corpora at once.** `acq web` and `acq-mcp` each hold one (~214 MB each, M7) when both run. Part 3's "resident" is the place to decide this.
- **Accessibility and keyboard.** Nobody has assessed them; there is no ruling, test or audit.
- **What I could not determine.** Whether the 240 s timeout is a test-side Fetch hold left unreleased or a real page hang. I did not read browser.rs around line 923 deeply.

### 6. Surprises

- C191 exists and already routes "C++ layout and performance" to a separate frontend. That contradicts the framing that no ruling speaks to it, and the sitting round builds toward that layout in `acq web` anyway.
- `#312` is closed, but its test failed again (a new timeout mode) on the next Windows close, unfiled.
- The 50-row window re-runs the full query for every block. "No answer loads per request" (`mod.rs` doc) is true for the corpus, but not for the evaluation and sort.

### The question for the GUI's reviewer before parts 2 and 3

Under C191, is the live loop and the resident being designed for `acq web`, or for the separate high-performance frontend? If for `acq web`, which of C16's commitments (URL as the only state, a full server re-render per interaction, no client-side order) is the owner willing to relax to get the C++ interaction he asked for, measured first on his 22k-item store?

Top three claims verified by re-reading: C191's text (`decisions/frontends.md:18`); the post-fix timeout (CI run 45547968713 against the time of `51d99a3d`); the `/rows` per-block full answer (`mod.rs:2738-2772` and `Web::window`).
