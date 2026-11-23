# 4-full-1-form-abyssal — Abyssal sockets

## Goal
An abyssal socket is asked for in the language, from the form's boxes
and by its stat, and each runs as the site reads it
(`4-full-1-form-design.md` §8 step 8, its first part; #185). The
language gains `abyssal`, GGG's `A`; the site's Sockets and Link
Groups filters read their key `a`; a Sockets `max` of 0 is no bound;
the sitting of 2026-11-23 lands. Closes #185.

## Inputs
- #185 and its comment of 2026-11-23 (the owner's word, rounds zb and
  zc, what the site answered); #309 (what is not settled).
  `4-full-1-form-design.md` §4's abyssal bullet, which this step
  supersedes where the decisions below differ.
- C101 (colours, and abyssal sockets beside them, over the item and
  within one link group), C113 (the oracle of missed matches), K6 (a
  clause's status), C116 (no id in user-facing text), C122 (pin once,
  lowest crate), C154 (a retirement names its holder).
- Code: the search crate's `sockets.rs` (`COLOURS`, `colour_words`,
  `letter`, `Groups::each`), `group.rs`, `bind.rs`, `describe.rs`,
  `trade/translate.rs` (`filter_keys`, the number loop, `read`, the
  `lacked_meets_maximum` branch), `trade/catalogue.rs` (`PLACEHOLDERS`,
  `lacked`), `reference/trade-catalogue-v1.toml` (its header, the two
  socket rows); the CLI's `web/form.rs`, `web/page.rs`, `web/fill.rs`.
- The sitting: the recorder's files in `~/Downloads/acq-sitting`,
  copied to `search/pseudo-stats/raw/searches/zb-fragments/` and
  `zc-fragments/`, each folder's
  `shasum -a 256 * | awk '{print $1}' | sort | shasum -a 256`
  `c35e62ff61ca50cab596a78d579eaf3e53e7ecf835cf3191b6a1a754aaf78ff6`
  and `c507299b909f45f61511e719100f7a9bd7fac276fa42f48321c13c4bd88df53c`; the sheet at
  `9594b66a` (zb) and `48bcfa0e` (zc); the site-sitting skill's steps
  6 to 10; the last landing, `393024b8`, as the model.
- CI at `48bcfa0e`: Gate (Linux) 45277288828 success, Gate (Windows)
  45277289166 success.

## Decisions
The design session's (C172, who designs), each settled with the owner:
1. **One table of socket words.** `COLOURS` becomes a table named for
   what it holds, socket words, of three columns: the word, GGG's
   letter, the site's key; `abyssal`, `A`, `a` its fifth row.
   `sockets.abyssal` and `linked(abyssal…)` follow as the four do,
   counted as they are. Every list of the words, letters or keys reads
   it: the binder, `group.rs`, `describe.rs`, `Groups::each`,
   `filter_keys`'s sockets entry, the number loop, `PLACEHOLDERS`, the
   form and the page. Words a user reads call it a socket, never a
   colour.
2. **The site's key `a`.** `socket_filters.sockets` lands
   `sockets.abyssal>={a}` and `socket_filters.links`, inside its
   `linked( … )`, `abyssal>={a}`; both stay exact (zb01 to zb10). `d`
   stays a key no row reads: non-null, its clause is unsupported by
   name, as today. Not taken: the design's envelope key `abyssal`, and
   Link Groups without a box.
3. **A Sockets `max` of 0 is no bound** (zb14, zc02): an attribute on
   the catalogue row beside `lacked`, on `socket_filters.sockets` alone,
   declared in the header and checked by the loader as `lacked` is. The
   translation drops that bound before any other reading of the bounds,
   and the clause's report says so in words. No rule keyed on a row's
   name in code; Link Groups and the ranges untouched (#309).
4. **The stat**, `stat_3527617737`, is in `trade-stats-v1.toml` and its
   form in `trade-game-forms-v1.toml`: no table changes. Shown end to
   end: zb11 and zb12 run in the oracle; the page's finder offers it and
   a form row of it runs.
5. **The form.** Each socket row gains an A box after W, made from the
   table's keys by the row's shape; exact, so no mark. No script change.
6. **The sitting lands here.** Once `a` is read and the `max` dropped,
   zb01, zb05, zb06, zb08 run and zb14's and zc02's items match; zc03's
   ten stand as a `sitting` finding under `misc_filters.quality`, its
   `asks` #309's first two questions. Each test that pins a count over
   the captures holds what it then measures.

## Find first
- `git grep -n -E 'COLOURS|colour_words|sockets::letter|SocketColour|sockets\.(red|green|blue|white)|"[rgbw]", "[rgbw]"|"R", "G"|red,? green,? blue' -- crates ':!*/goldens/*'`:
  each list of the words, letters or keys (a dotted field, a word list
  in a string), each to read the table.
- `git grep -n -i 'colour' -- crates/acquisition-search/src crates/acquisition-search/reference/trade-catalogue-v1.toml crates/acquisition-cli/src crates/acquisition-mcp/src`:
  each text a user reads that calls a socket word a colour, a row's
  `says` among them.
- `git grep -n -i 'abyssal\|no colour word\|which no word names' -- crates ':!*/reference/*'`:
  the tests and texts holding that an abyssal socket has no word (the
  generated model's `LETTERS` among them).
- `git grep -n 'lacked' -- crates/acquisition-search`: how a row's
  attribute is declared, loaded, refused and pinned.
- With the captures written, `cargo nextest run -p acquisition-search --no-fail-fast 2>&1 | grep -E '^ +FAIL'`:
  every test the landing moves.

## Done when
- Search crate: `sockets.abyssal` and `linked(abyssal>=1)` on a belt
  with one `A`, a Darkness Enthroned with two and a six-link with none;
  an unread colour widens `abyssal` as it does `red`; `describe` names
  five words; a misspelt word's refusal offers five. The generated
  tests name `A` by its word. Each test whose claim the word replaces
  is changed, or retired by a `retired X; held by Y` line.
- `trade-translate.toml`: a Sockets and a Link Groups case with `a`,
  each with matches and fails; Sockets `max` 0 alone, `min` 2 with
  `max` 0, and `max` 0 with `a` 1, each text and report as built; a `d`
  of 1 unsupported by name; Link Groups `max` 0 as today.
- The loader refuses the attribute on a row whose shape is not
  `sockets`, pinned in `trade_catalogue.rs`.
- The sitting: 17 captures written by `trade-split.py --write` and their
  inventory; `captures.json` and what steps 7 to 10 write; a
  `MANIFEST.md` paragraph and two rows as rounds y, z, za have; the
  site's answers in `docs/design/trade-ground-truth.md` as
  `393024b8`'s were. A case in `trade_oracle.rs` translates each of
  zb01 to zb14, zc01 and zc02, those with no items among them, and
  holds that none has a remainder; their fetched items match; zc03's
  are the finding's rows; every count pin the last Find-first command
  finds, `trade-oracle.toml`'s header among them, holds what the tests
  measure, each move said in the commit.
- Web: `search_form.txt` holds both rows' A boxes empty and filled; a
  `web.rs` case finds `stat_3527617737` by `stats?q=` and runs a form
  row of it over a mock world with an abyssal belt.
- C101's Pinned names the tests holding the word and drops "Unbuilt".
- `CLI-REFERENCE.md`, `MCP-REFERENCE.md` and the goldens regenerated
  under `ACQ_UPDATE_FIXTURES=1`, each diff read.
- `tools/gate.sh` exits 0 on the close's tree.

## Not in scope
- The index (`describe` page, #183's rest): step 8's second build.
- A word for a resonator's socket (`d`); #309 and any range's or Link
  Groups' `max` of 0; PoE2's sockets.
- The CLI's or MCP's own words beyond what the table and the goldens
  move; the look (#285).

## Rules
`TEMPLATE.md`. Every `acq` is in a mock world (the mock-session
skill). Land the captures as the site-sitting skill says; the
recording stays under `raw/`, never committed. A choice of shape with
two defensible answers goes to the design session (C172, who
designs); the rest carries on.

## Report
`TEMPLATE.md`.
