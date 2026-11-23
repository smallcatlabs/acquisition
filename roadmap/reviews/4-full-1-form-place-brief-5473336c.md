`roadmap/briefs/4-full-1-form-place.md` at `5473336c`: a builder could begin, but the acceptance criteria and shared-row contract need the corrections below.

Findings, most severe first:

- **Done when, first bullet — changes what gets built.** The acceptance fixture does not distinguish a body’s coordinates from the place’s usable cell. `corpus.rs::celled` removes the cell from worn and socketed items even when their bodies carry `x`/`y`; sorting on `item.cell` could pass the stated tab fixture while misordering those items. Smallest change: include worn and socketed items with body coordinates that would otherwise sort first, and require them after items with usable cells, ordered by id.

- **Decisions 4 / Done when, maker — changes what gets built.** The proposed parts omit frame text while requiring the CLI head to retain its current output. `search_cmd.rs::row_text` uses `shown_title` for expanded rows, and `shown_title` appends both rarity and frame; the listed name, rarity and base parts do not settle where that frame belongs. This is the first implementation choice left to guess. Smallest change: name an optional frame part, or explicitly retain the expanded `shown_title` branch, and require an expanded-row golden to preserve it.

- **Find first, last command / Done when, first bullet — wording.** The brief names an existing generated order oracle that I could not find. `tests/common/generated.rs` contains generators, fixtures, answer comparisons and `sort_of`, but no independent expected-order implementation; its generated properties generally compare answers obtained through the search API. Smallest change: identify the intended function by name, or replace the oracle requirement with preserving those existing generated properties and let the seeded boundary test establish the new order.

- **Find first, first command — wording.** The order-text scan misses an existing spelling of the old guarantee. It finds “store’s order” but misses “store’s stable order” in `answer.rs`’s Rows and continuation documentation, at lines 215 and 257. Smallest change: extend the expression to include that spelling, for example `store's \(stable \)\?order`, and use the expanded scan in the corresponding done-when line.

- **Inputs, code bullet — wording.** “The store’s two `ORDER BY`s” carries an unexplained count and does not accurately identify the intended statements. `git grep -n 'ORDER BY' 5473336c -- crates/acquisition-store/src/corpus.rs` also finds ordering for revisions, realms, listings, locations and gone rows. Smallest change: name `ITEMS_SQL` and `ALL_ITEMS_SQL`, with that discovery command beside them, instead of describing the file as having two orderings.

What I did not check and why: I did not build, run tests, execute the nextest discovery command, query CI, or access the real API or store, as instructed. I could not independently verify #296’s issue text; I checked the committed design and its recorded approval. I found no invariant or dependency-edge contradiction in the proposed implementation.

What got in the way: the brief’s numbered GitHub inputs are not available from the commit alone. The prompt otherwise allowed the review to proceed without difficulty.
