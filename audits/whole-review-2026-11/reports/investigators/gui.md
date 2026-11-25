# Investigator's report: gui

As returned by the Opus investigator under `INVESTIGATOR.md`, local paths shortened. Its claims are its own; what the orchestrator re-ran is marked in `findings/gui.md`.

**Track: the GUI's direction.** Read-only. No cargo was run. I timed the release `acq` (built 19:29, same contract) on the copy of the real store, with GETs only, and stopped each server afterwards (`pgrep -fl "acq --account"` prints none). Other readers were working at the same time, so the timings are noisy. Each figure is three runs; rerun with `curl -s -o /dev/null -w '%{time_total} %{size_download}\n' "$URL<route>"`.

## 1. The picture

`acq web` is axum, with 16 routes under a per-launch token (`web/mod.rs:638-661`). `page.rs` builds HTML with `format!`/`esc` (601 `format!|write!|push_str` hits). There are four scripts (579 lines with the CSS). **V**

| Concept | What it is | What pays for it |
|---|---|---|
| URL as state (C16) | A page is rebuilt from its URL. No storage, cookie or `history` (a lexical scan) | Reload or a link always shows the truth at one basis (C98). Pages can be bookmarked |
| Routes + `Accept: json` | Each route serves the verb's document as JSON | Tests compare a page with `--json`. This is already a thin-client contract |
| `form.rs` fold / `fill.rs` | Controls generated from the site's filter dump. Names are JSON paths. Fill is the exact inverse, held by a partition pin | Trade-site familiarity (the owner's stop 1). Site searches can be pasted |
| `from` / landing rule (`from.rs`, 346 lines) | Every leaving request carries its page back | A navigation-per-act model needs a way back |
| toggles, `toggle_item`, `at`, `cut` | Opening a row or a place is a 303 to a full page with an anchor | Keeps "the page is its URL" while the page cannot change itself |
| `Paged` / ways on | Each offer in a refusal becomes a link or a form | C143, no command lines on pages |
| window (`rows`, `window.js` 152 lines) | Server-windowed 50-row blocks over the held corpus | 44k rows with no DOM blowup (spike verdict) |
| `count.js` | A debounced count per keystroke | The only as-you-type behaviour |
| words | 80 distinct `search_cmd::*` items plus `store_cmd` helpers, imported from the CLI | One wording on every surface (C53) |

**Measured on 44,090 pc items** (the scouts' "22k" is stale). Release build. **V**

| Route | Server time | Bytes |
|---|---|---|
| `search?realm=pc&text=` (cards) | 38–42 ms | 87,018 |
| same as a table | 55–57 ms | 97,868 |
| `pseudo.total_life>=90` table (586 matches) | 65–68 ms | 98,308 |
| same, first request (corpus load) | 0.89–1.0 s | — |
| JSON of that answer | 60 ms | 20,135 |
| `rows` window at offset 50, 20000 or 44000 | 30–33 ms, flat | ~26,000 |
| all 44k rows sorted by a value | 110–123 ms | 99,825 |
| `count` | 35 ms | 3,529 |
| page with one row opened | 111 ms | 104,078 |
| `item/<id>` (HTML or JSON) | 193–257 ms | 3,905 |
| `tabs` Standard, closed (280 toggles) | 67–73 ms | 138,202 |
| `tabs` with a 32-page map stash open | 90–101 ms | 150,804 |
| `tabs` under a query | 151–153 ms | 66,424 |

- The debug build is only 20–25% slower (the life table at 80 ms against 65 ms). **V**
- The home page alone is 67,207 bytes, so about 67 KB of every 98 KB answer page is the form sent again. **V**

**The HTML (V, screenshot).** It is legible, dark, trade-like and dense, with coloured rarity names, a sticky head and sortable value heads. Someone could live in it as a reader. They would not live in it as a tool. Each act (open a row, sort, pick a value, open a tab) is a 303 plus a 100–150 KB page load, after which focus is lost and only the anchor restores scroll. There is no selection, no arrow-key movement through rows, and no filtering as you type. The count is the only live part.

**The C++ app (V, read for behaviour only):**
- It is one process with an in-memory model.
- A form edit restarts a 350 ms single-shot timer, then refilters in place (`acquisition-qt/src/ui/mainwindow.cpp:81,222-224,1632`). Expansion and selection are saved and restored around the refilter (`:1642-1690`).
- A selection change, after 100 ms, fills the item pane and the buyout panel (`:80,218`).
- There are search tabs (`QTabBar`, `:325`), a By Tab / By Item switch (`:339`) and header-click sorting (`:392`).
- Refresh deltas land in visible buckets in under 1 ms (GUI-NOTES §6.1).
- There are no page loads. Everything happens within one debounce.

**The gap is in the client, not the server.** At twice the scouts' size, filter, window, count and sort all sit within the C++ app's own 100k budgets (77 ms for a broad filter; GUI-NOTES §6.1). **V+I**

## 2. Theses

**T1. Strike C191: `acq web` is the product GUI, and C16's semantics clause stands. Position (b).**
- What goes: 1 ruling (C191), and the separate frontend it reserves, before any line of it exists.
- Edge cases that go with it: the double plan (dev surface now, product later), and parts 2 and 3 designing a feed for a surface ruled temporary.
- Smallest input that changes: none at runtime. What changes is what parts 2 and 3 design for.
- Rulings touched:
  - C16 holds: semantics stay server-side, the URL is identity, and Tauri is still the shell.
  - C138 holds: the client ships in the same binary as its routes, so the routes stay private and unversioned. This answers T11's "third surface": only option (c) would create one.
  - C139 holds.
- Security: unchanged.
- Performance: the target becomes the C++ budgets measured at the page.
- Function: the owner gets the trade-site look (which is native to a browser) and a path to C++ interaction.
- Agent's ease: one GUI to know, not two.
- Why not (a): it leaves morning 2 on pages ruled temporary, and the owner with no daily GUI until a rewrite.
- Why not (c):
  - A native frontend must rebuild the generated trade form, which egui handles worst (GUI-NOTES §2.4).
  - It must move about 80 word functions out of the CLI crate (C1: a frontend cannot link `acquisition-cli`), plus the fold.
  - It would arrive months later.
- Strongest objection: C191's own reason, that speed work warps a simple surface. T2 is my answer: the change is smaller than the machinery it removes.
- What would change my mind:
  - a swap prototype on his store that cannot keep as-you-type filtering under about 150 ms perceived;
  - "C++ performance" meaning a million items. That is a corpus-memory question (9.5 KB per item, GUI-NOTES §6.3) and no GUI choice fixes it;
  - his wanting a window with no browser at all, which Tauri already covers.

**T2. In-page acts become in-place swaps over the same routes; the 303-landing machinery for them goes.**
- What goes:
  - `toggle_item`, `at` and `cut`'s landing;
  - `picked`'s 303;
  - the toggle 303s for places (`grep -c toggle src/web/*.rs` = 45; `opening` 33; tests: `toggle` 38, `picked` 31).
- What stays: `from` for real steps (plan, saved, apply).
- The script fetches the same URL's region and calls `history.replaceState`, so the URL stays the whole state (C16 kept). The lexical ban on the word `history` narrows to `pushState`/storage. Today that ban forbids the very API that keeps URL-as-state honest under updates. **V** (`mod.rs:231-233`)
- Filtering as you type becomes `count.js` asking the rows region, not just the count. The server already answers that in 30–65 ms.
- Edge cases that go with it: anchor seeking after reload, scroll lost on sort, the "kept rows" rules across view changes. They exist only because each act reloads the page. **I**
- Smallest input: clicking a row name.
  - Today: a 303, then 104 KB and 111 ms, with scroll restored by anchor.
  - After: a card fragment of about 4 KB inserted under the row. With perf H3's keyed `show`, that should be well under 50 ms. **I**
- Rulings touched:
  - C16's basis line still has to hold: a swapped region carries its basis, and the page reloads on a moved basis, as `rows`' 409 already does.
  - C187: the browser check grows to cover the swaps.
- Security: unchanged (GET fragments under the token).
- Performance: about 67 KB less per act, and no lost focus.
- Function: selection and keyboard navigation become possible as transient state, which C16 already permits.
- Agent's ease: fewer concepts (no landing for in-page acts).
- Strongest objection: no-script and pages a screen reader can address degrade. Keep the plain GET forms as the no-script path; they already work.

**T3. Pin the durable part, the routes, their JSON and the fold, and let the markup go.**
- Durable:
  - each route's document is the verb's (the ~10 "answers as the CLI" tests);
  - C138's listener tests;
  - C168's writes;
  - the fold∘fill partition.
- Disposable:
  - 795,121 bytes of web goldens in 14 files (`cat goldens/web/*.txt | wc -c`), including 458 `# holds` lines;
  - 174 HTML-shaped literals in `web.rs` (`grep -c '"<div\|"<a \|...'`);
  - the landing and toggle tests.
- 86 of 102 web commits also touched tests or goldens (scout). **V/I**
- What goes: most of the markup goldens. Keep one smoke render per page and an escaping test.
- Edge cases that go with it: a class rename stops being a three-file edit.
- Smallest input: renaming a CSS class. Red today, green after.
- Rulings touched: C153/C122 want wording held once. The words are the CLI's (`search_cmd`) and already golden there, so the page golden double-holds them. The page's own "player's words" (#182) need a small golden of their own.
- Security: the escaping test must stay.
- Agent's ease: a page module can be rewritten in an hour without regenerating 800 KB.
- Strongest objection: a markup golden caught real regressions a JSON test cannot (an offer drawn as the wrong control). Keep the "every offer is a link or a form" walk as a structural test, not a golden.

## 3. What I would defend

The form generated from the site's dump, with its exact fill/fold partition. Another reader cuts it as a second query model. It is the familiarity the owner asked for, it is the only part tied to a sanctioned reference, and its pin is a clean shape.

**The two query models (Q5).** The translation is one-way, site to language (`acquisition-search/src/trade/translate.rs:452`; outbound is under "Not doing"). So:
- an agent's text opens only in the query box;
- a form search opens in both, the box holding its canonical text.

The language is the model, because it is what evaluates and it is a superset (sums, `holds`, weights). The form should be a lens on the image of `translate`: a partial inverse that fills the form where a text came from a site search, or is shaped like one, and says "not in the form's terms" otherwise. That is one function in the search crate, pinned by the existing partition. Nothing goes. T3 of `design-theses.md` was "low on which side wins"; I am not. **I**

## 4. Where the old way is right

- Server-side semantics, server windowing, and no cached ordered result (C46/C48).
- On 44k items a window costs 30 ms at any offset (**V**). The C++ app needed weeks of row-operation protocol for what a keyed re-render does here.
- The URL as identity is also right. Only the navigation-per-act reading of it is wrong.

## 5. What I did not check

- Browser paint and parse times (server time only).
- Windows.
- Accessibility.
- The part 2 subscriber code.
- A swap prototype (T2 is inferred).
- The C++ app running.
- How the currency check (perf H1) splits inside the 30 ms window.

## 6. Where I part from the scouts

- **web-gui H2**, that the model "cannot give C++ interactivity". The server can; I measured it at twice their store size. The navigation model is what cannot.
- **web-gui H1.** Agreed, and sharpened into the question below.
- **perf H3.** Confirmed: the item page takes 200 ms whether HTML or JSON, against 111 ms for a whole page with the row opened from the held corpus.
- **design T11.** The route JSON need not become a third surface under (b). It would under (c).

## 7. Small finds

- `item/<id>` costs 193–257 ms for 3.9 KB because `show` scans the store. The opened-row card is cheaper (perf H3, unfiled).
- Every answer page sends the full ~67 KB form again.
- The first search after start is ~1.0 s (corpus load). Nothing on the page says it is loading.
- C16's script scan bans the word `history`, which bans `replaceState`, the API that would keep the URL as state under in-place updates.
- The debug build is within 25% of release on the web paths, so debug timings are usable for the GUI.
- `tabs` under a query (151 ms) is twice the closed page (70 ms).

**The question for the owner (yes/no):** Is `acq web` the GUI you will use every day, so that C191 is struck and parts 2 and 3 design its live loop as in-place updates over its existing routes?
