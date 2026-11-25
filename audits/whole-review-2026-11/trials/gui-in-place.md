# Trial B: in-place updates, timed in a real browser

Thesis 2 says the server is fast enough for a daily GUI and the page
model is what is slow. This trial timed the page model's alternative on
the copy of the owner's store (44,090 pc items), with no product code
changed: `./target/release/acq --account 'GERWARIC#7694' web
--no-browser` under `ACQ_NO_SPAWN=1` (reads only, GETs only), opened in
the desktop app's browser pane, and a script run in the page that, for
each query, fetches the search page for that query, parses it, replaces
the `#answer` region, forces layout and waits two animation frames.

An Opus agent was compiling the daemon crate on the same machine at the
time, so these are upper bounds.

| Query | Fetch ms | Parse ms | Swap ms | To paint ms |
| --- | --- | --- | --- | --- |
| `pseudo.total_life>=9` | 57, 44 | 1.7 | 2.3 | 69, 66 |
| `pseudo.total_life>=90` | 43, 43 | 1.3 | 2.0 | 50, 67 |
| `pseudo.total_life>=90 class:rings` | 98, 101 | 1.3 | 2.2 | 133, 117 |
| `pseudo.total_life>=60 class:rings pseudo.total_ele_res>=40` | 159, 159 | 1.3 | 2.2 | 167, 167 |
| the empty query | 39 | 1.0 | 1.3 | 52 |
| `rarity=unique` | 30 | 1.1 | 1.8 | 48 |
| `rarity=unique -has:priced` | 32 | 1.2 | 1.6 | 50 |
| `class:boots` | 27 | 1.1 | 1.9 | 33 |
| `level>=20 quality>=20` | 41 | 1.3 | 2.1 | 50 |
| all rows sorted by `ilvl`, descending | 100 | 1.3 | 2.1 | 117 |

Each fetch returned the whole page, 87 to 123 KB, of which the trial
used only the answer region.

Single fetches from the same page:

| Route | ms | Bytes |
| --- | --- | --- |
| the answer as JSON, `pseudo.total_life>=90` | 34, 34 | 76,197 |
| the answer as JSON, `class:rings` | 18, 17 | 58,516 |
| the answer as JSON, `pseudo.total_ele_res>=40` | 78, 78 | 73,685 |
| `item/<id>` as HTML | 189, 186, 187 | 3,888 |
| `item/<id>` as JSON | 187, 186 | 3,887 |

## What it shows

- Replacing the answer in place costs the browser 3 to 4 ms. The time
  is the server's, and for every query tried it is inside the 350 ms
  the C++ app waits before it refilters.
- One item is the slowest thing on the page, 187 ms for 3.9 KB, because
  `show` scans the store for one id. A row opened from the held corpus
  would not.
- Two totals in one query cost 159 ms; one costs 43 to 78. That is the
  next thing to measure if filtering as you type is built.

## What it does not show

- A person's judgment of the result. Nothing was typed by hand; the
  queries were sent by script.
- Windows, or a browser other than the app's own.
- A live refresh arriving while the page is open.
