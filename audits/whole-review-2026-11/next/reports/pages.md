# Investigator's report: what the page goldens have caught

As returned by an Opus investigator on 2026-11-25, read-only, no cargo and no `acq`. The review proposed that the web's goldens hold a structural projection of a page instead of its markup. It was asked what the markup goldens have actually bought, and told that "they are earning their keep, with the commits that show it" was a welcome answer. Its claims are its own.

## 1. History: 40 of the 96 commits that touched `tests/goldens/web/` since 2026-10-26

Sample rule: rows ⌊k·2.4⌋+1, so n≈2.4, starting at 42dfe873 and ending at 3c566df7. Source and golden line counts come from `git show --numstat`.

| Class | Count | Examples |
|---|---|---|
| (a) the golden follows a page change that was meant | 28 | 04d13af6, 943576cd, f562ca83 (+954 lines), 7c1ce05a, 1ee23513, 27d5286b (a new plan_session case) |
| (b) the golden's diff showed a fault that was then fixed | 1 | c4550601 |
| (c) churn from a change made elsewhere | 11 | 847f0a53, 8ae1d597, 1ab468eb, 01cdf862, f1e06eec (adds 171 lines to answer_ways; the same fact is in CLI goldens), 64a23489, 593879fe, eecb6ef4, 077c80c8, 3c566df7, c0d4f107 (a harness reshaping with no source change) |

No commit message among the 96 says that the author's own regeneration caught a fault. Each says "regenerated, each diff read", and the faults were found by reviews, seats and sittings, then pinned by targeted tests. Three cases come close; none was found by the gate.

1. **86a63c9f → c4550601: a lost form field.** The landing-rule commit regenerated `table_rows.txt` with `from=search?…view.fields=icon`, which dropped `item=r1&item=kaom`. The author missed it. The cold reviewer found it by reading the golden: "The changed `table_rows.txt` golden records this omission" (`roadmap/reviews/4-full-1-form-landing-86a63c9f.md:3`). It was fixed in the next round.
2. **1ee23513 → e9c89426: a count or wording contradiction.** `refresh_loop`'s golden showed "Stash tabs (6)" beside "15 tabs". The reviewer read it in the golden (`roadmap/reviews/4-full-1-step6-places-42b18a92.md:3`), and e9c89426 fixed it (its body, point 1).
3. **943576cd → 7a93d1e6: a test-world fault, not a page fault.** The golden failed in compare mode because the league lists differed between test worlds. This was caught only in the following commit; the author had regenerated one test's goldens and committed without rerunning the module's tests.

Faults by kind: one lost form field, one text-fact contradiction, one harness fault. None was escaping, a wrong link target, a wrong control or layout. Both page faults are facts that a projection would show as plainly, and probably more legibly than a 4,550-character line.

## 2. Review findings: 25 findings about pages

Sources: the cold reviews of 7c1ce05a, 5429b5f5, 49c70542, 86a63c9f, 3d8d12ea, a17bec8a, 3675cc7b, 306e0abd and 0e3a1dc0, read whole; review files are under `roadmap/reviews/`.

| Would hold it | Count | Findings |
|---|---|---|
| A structural projection (fields, links, statuses, text facts) | 14 | Rows' "more" links (7c1ce05a #1). The way check skipping pages (#5). A comma rest not folding (#6). Facts said differently (#8, #9a). `from` lost on the tabs form (49c70542 P2). Omission notes missing (P2). Home ignoring `league=` (P3). Opened rows lost from `from` (86a63c9f P2). A conflict's name only in a hidden field (P2). A hidden sort the user cannot correct (3d8d12ea P2). A save note missing (P3). A continuation's sort links dropping `item` (306e0abd P3). A head not matching its columns (0e3a1dc0 P2). |
| The real markup or a browser | 6 | Padding collapsing in HTML (7c1ce05a #9b). CSS-hidden number fields blocking submit (a17bec8a P2). A browser test's count check too loose (P2). A failed `window.js` fetch never retried (3675cc7b P2). A browser bound of +2 that should be +1 (P3). A refused box loading no count script (306e0abd P2). |
| Neither: logic, race, overflow or headers, each needing its own test | 5 | Set-Cookie scan coverage (5429b5f5 P2). Reading a saved row twice (P3). A padded stat id (3d8d12ea P3). Offset overflow (3675cc7b P3). Location rank ignoring the league (0e3a1dc0 P2). |

Before review, the goldens caught none of the 25. One of them (86a63c9f P2) was sitting in a golden unnoticed.

## 3. What already exists

- `tests/harness/submitted.rs:72` `submitted()` reads a form the way a browser submits it. It handles a control's owning form, `<template>` content, disabled controls, unticked boxes, a select's chosen option and HTML number validity. This is the strong piece.
- `web.rs` has narrower readers: `heads` (1535, `a.head` links only), `box_fields` (1562, the query box's hidden inputs only, no textarea), `offered_links` (4030, `ul.offers` only), `posted_forms` (5511, POST forms only), `ways_on` (2972) and `main_of` (5739).
- `web.rs` keeps duplicate copies of `unescaped` and `attribute` (5495, 5504) beside the harness's.
- Missing pieces: a reader for every link on a page, a text-facts extractor, and a status line.
- `browser.rs` has 16 tests through CDP. They cover the scripts: counting, finders, the window, resizing, league swaps.

A sketch of the projection for `GET search?<the ring search>`:

```
GET search?<the ring search>  200 text/html  scripts: count window keep
basis store <store> · build <build> · snapshot 7 · facts v9 · …
form #search-form GET search#answer novalidate
  query.filters.type_filters.filters.category.option=accessory.ring
  stat.id=explicit.stat_3299347043  stat.min=70  view.limit=20  view.fields=
form .query-box GET search#answer
  realm=pc membership=live view.limit= view.fields=icon
  text=league=Standard category="accessory.ring" sum(…)>=70
form .table GET search  realm=pc league=Standard …
  button toggle_item=r1 "Doom Loop Two-Stone Ring · rare"
  button toggle_item=r2 "Rune Coil Two-Stone Ring · rare"
form POST leagues  from=search?realm=pc&league=Standard&… "Fetch the league list"
form POST /<token>/saved  from=… "Save"
links
  "as cards"   -> search?…&view.fields=
  head Fetched -> search?…&view.sort=fetched&view.desc=true
  head Item    (none: "Not sortable yet: …")
  "Link"       -> https://www.pathofexile.com/trade/search/Standard/H4s… (external)
  term 1 undecided -> search?realm=pc&text=undecided(category="accessory.ring")
  index        -> describe?from=search%3F…
facts
  h1 Answer · h2 The search that ran
  2 matching items in the table, as they lie in their place … (no sort asked)
  row r1 | Two-Stone Ring | none (nothing applies) | Standard / Rings · x 0, y 0 | 95
  Total 2 matches (fetched <age> ago, pc · Standard / Rings)
  Scope account Alice#1234 · pc · live · 5 items · 2 locations fetched …
  terms: 0 league=Standard 5/0/0/0 · 1 category=… 3/1/0/1 · 2 sum(…)>=70 3/2/0/0
  5 counts have no link: …
  from the query JSON: exact — …
```

Against 278 KB of `search_pages.txt`, a projection loses:

- **Escaping:** text is unescaped. See section 4.
- **Classes:** 156 of the 445 `# holds` lines name a class, a `data-` attribute, a title or a tag, such as `rarity-rare` or `fate fate-ambiguous`.
- **Script hooks:** `data-route`, `data-row-height` and the like. browser.rs covers these by behaviour.
- **Hover titles,** unless they are kept as facts.
- **Open state:** `<details open>`.
- **Nesting and order.**

## 4. The two properties that stay as their own tests

- **Escaping item text:** no test pins it. `escaped()` (`web.rs:8130`) checks only the language index's names. No seeded item text contains `<` (`grep -c '&lt;'` gives 0 for every page golden except `describe.txt`). The only thing holding it is the markup golden's incidental `Kaom&#39;s Heart` (102 `&#39;` in `search_pages.txt`). So today it depends entirely on the markup goldens, and only for the apostrophe.
- **C143, every way on is a link or a form:** held by `an_answer_item_or_place_page_prints_no_command_line_and_no_flag` (`web.rs:3179`), `every_request_an_answer_carries_is_a_way_on_its_page` (3069), `every_leaving_request_lands_with_the_way_back_to_its_page` (7693) and `every_refusal_a_leaving_request_meets_has_its_way_on_and_its_way_back` (7867). These scan the live page bodies, not the goldens, so they are independent of them.

## 5. Position

The markup goldens are not earning their keep as regression catchers. In six weeks, neither the gate nor an author's diff reading caught a page fault through them. They paid off twice as review evidence, and both times the fact was a field or a count that a projection holds. Their real cost is the 11 of 40 sampled commits that were churn, plus the 8 to 10 goldens a single wording moves.

Convert them **before** more GUI work, and especially before any in-place-swap redesign, which would rewrite the markup while a projection stays still.

The smallest first step has two parts:

1. Seed one item named `<img src=x onerror=1> & "Kaom's"` and assert the page holds the escaped form and no raw `<img`. This fixes the one property only the markup goldens hold today.
2. Convert `search_form.txt` (64 KB, no facts, and `submitted()` already reads exactly what it needs) to a projection.

**Smallest real bug the converted suite would miss** (inference): the query box rendered closed on a text answer, i.e. `query-details" open` lost. That attribute appears 28 times in the goldens, and no test names it. The rarity colour class (`rarity-rare`) is in the same position, except where `web.rs:2162` and `:2216` assert it. Either needs the projection to record the open state, or a test of its own.

**Is the conversion mechanical?** The projector itself is: one function, built mostly from the helpers above. The 445 `# holds` lines are not. 156 of them are markup-shaped, and each needs a decision: become a text fact, become a projection line, become a dedicated test, or be dropped. Have a cheaper model do the regeneration only after that translation table has been written with judgment, one golden at a time.

Commands used: `git log --since=2026-10-26 --format='%h %s' -- crates/acquisition-cli/tests/goldens/web`; `git show --numstat`; `grep -c '^# holds'`; `grep -c '&lt;'` and `grep -c "&#39;"` over `tests/goldens/web/*.txt`.
