# seat-web-form-stat-bound — The abyssal stat under a bound

## Goal
A bound on a stat sums its text's plurals whether a form writes a `#`
or a digit where the text has a `#`. So `stat_3527617737`, "Has #
Abyssal Sockets", at `min` 1 finds an item displaying `Has 1 Abyssal
Socket`, as the site does (round ze), where today it misses each (the
page seat, fault 2). The rule is the forms table's, for every text; the
sitting lands. Closes #315.

## Inputs
- #315 and its comment (`gh issue view 315 --comments`): the seat's
  counts, round ze's answers, the fix. `audits/web-form/seat/REPORT.md`,
  fault 2. The owner's choice on #185 (presence over every form, a bound
  the text and its plurals) and his word of 2026-11-23 to build it.
- C106 (game data), C113 (the oracle of missed matches), C122 (pin
  once, lowest crate), C153 (a finding joins its surface), C154 (a
  retirement names its holder), C116 (no id in user-facing text).
- Code: `tools/trade-forms-table.py` (rule 3, `kind_of`, `word`); the
  search crate's `src/trade/forms.rs` (`open`, the loader's `plural`
  check), `src/trade/translate.rs` (the forms loop and its notes), and
  `reference/` (the game-forms table, the catalogue's header).
- The sitting: the recorder's files in `~/Downloads/acq-sitting`, copied
  to `search/pseudo-stats/raw/searches/ze-fragments/`; in that folder
  `ls | wc -l` gives 7, `cat * | wc -c` 136429, and
  `shasum -a 256 * | awk '{print $1}' | sort | shasum -a 256`
  `ec88a019b4b42911ecec72514ba39b0852f8ed4f21c2d90f08f81357494770af`;
  the sheet's `round_ze` at `a681ca7d`; the site-sitting skill's steps 6
  to 10; the last landing, `5c04b657`, as the model.
- The export's clone at its pin (`tools/sources.py`), checked by
  `python3 -I tools/trade-forms-table.py --check` before any edit.
- CI: the last push's run of each gate (`gh run list --branch main
  --workflow "Gate (Linux)" --limit 1`, and Windows), read first; a
  red run is this step's first concern.

## Decisions
The design session's (C172, who designs), settled with the owner:
1. **The owner's choice stands.** `Has 1 Abyssal Socket` is the text's
   singular, `other` only because rule 3 keeps out of `plural` a digit
   in place of a `#`, and the loader refuses it as one.
2. **A displayed line's digit is its number,** for every text: the line
   `Has 1 Abyssal Socket` templates to `Has # Abyssal Socket` with 1,
   which is how the unbounded translation already finds it. A form that
   is the text word for word but for a noun's or a verb's number is
   `plural` whether it writes a `#` or a digit where the text has a
   `#`, summed with the text under a bound. Rule 3 (docstring and
   `kind_of`) and the loader's check change together; nothing is keyed
   on this text or this stat. A word's number is what rule 3's `word`
   already reads; a plural it does not read (`Prefix`/`Prefixes`) stays
   `other` here and is listed in the report (Find first), not moved.
3. **The table is regenerated** by its generator at its pin, by the
   convention the reference tables follow for a change of content
   (its version and name; found below). How many forms
   move from `other` to `plural` is measured and said; each moved form
   is read once, and one that is not its text's singular or plural is a
   finding for the design session, not a silent move.
4. **A clause's status and notes are its row's**: no new status. A
   note that named a form left out of the sum no longer does.
5. **The sitting lands here**: ze01 to ze04's captures, the manifest's
   paragraph and row, the site's answer as the next numbered claim in
   `docs/design/trade-ground-truth.md`, and every count pin over the
   captures holding what it then measures. The oracle runs the four
   with every fetched item matched.
6. **Pinned at the lowest crate**, one row at the page (Done when).

## Find first
- `git grep -n -i 'digit\|plural\|kind_of\|open(' -- tools/trade-forms-table.py crates/acquisition-search/src/trade/forms.rs`:
  rule 3 and the loader's check.
- `git grep -n -e '\.kind\b' -e 'Kind::' -e 'summed' -e 'Shown::of' -e 'fn of(text' -e 'template::fixed' -- crates/acquisition-search/src/trade crates/acquisition-search/tests`:
  every reader of a form's kind, and how a form's text becomes a line
  where it writes a digit.
- After the regeneration, `grep -n 'kind = "other"' crates/acquisition-search/reference/trade-game-forms-v1.toml`,
  each read beside its text: an `other` that is its text but for a
  word's number `word` does not read is listed in the report.
- `git grep -n 'Has 1 Abyssal Socket\|is plural and does not' -- crates`:
  the refusal and the test that holds it.
- `git grep -n 'not its forms\|stat.text.forms\|stat.text.wording' -- crates ':!*/goldens/*'`:
  every note naming a form left out, and every count of the rules'
  clauses (the catalogue's header and its pin among them).
- `git log -p --format='== %h %s' -- crates/acquisition-search/reference/trade-game-forms-v1.toml | grep -E '^== |^[-+]version'`
  and `git grep -n 'VERSION' -- tools/trade-forms-table.py crates/acquisition-search/src/trade/forms.rs crates/acquisition-search/build.rs`:
  how a change of a table's content carries its version.
- `grep -o '^\*\*T[0-9]*' docs/design/trade-ground-truth.md | tr -d '*T' | sort -n | tail -1`:
  the highest claim number; the claim takes the next.
- With the captures written, `git grep -l -e 'captures.json' -e 'bodies.json' -- crates`
  and `cargo nextest run -p acquisition-search --no-fail-fast 2>&1 | grep -E '^ +FAIL'`:
  every test the landing and the rule move.

## Done when
- `tools/trade-forms-table.py`: rule 3's docstring and `kind_of` say a
  digit where the text has a `#` is the form's number;
  `trade-game-forms-v1.toml` is its output (`--check` prints `ok`), its
  header's counts as measured, its version by the convention found; the
  commit names how many forms moved and that each was read.
- `forms.rs`'s loader accepts such a form as `plural`, pinned in
  `trade_forms.rs`; the test holding the refusal is changed, or retired
  by a `retired X; held by Y` line.
- `trade-translate.toml`: the implicit and the explicit at `min` 1 each
  match an item displaying `Has 1 Abyssal Socket` of its kind and fail
  one without; at `min` 2 each fails `Has 1` and matches `Has 2 Abyssal
  Sockets`; no case's note names the singular as left out.
- `trade_translate.rs`'s counts, alone and with a bound, and the
  catalogue header's hold what `ACQ_CASES=full cargo nextest run -p
  acquisition-search --test main c113_every_stat_id_of_the_table_translates`
  measures (the gate runs a share; CI all), each move said in the commit.
- The sitting: ze01 to ze04's captures written by `trade-split.py
  --write`, their inventory, and what steps 7 to 10 write; a `MANIFEST.md`
  paragraph and row as round zd's; the claim in `trade-ground-truth.md`
  as `5c04b657`'s was. A case in `trade_oracle.rs` translates each of the
  four, none with a remainder, every fetched item matched; ze02's none
  held as none; every count pin the last Find-first command finds holds
  what the tests measure.
- `web.rs`'s `an_abyssal_sockets_stat_is_found_and_its_form_row_runs`
  gains the row with `min` 1, finding the abyssal belt alone.
- `CLI-REFERENCE.md`, `MCP-REFERENCE.md` and the goldens regenerated
  under `ACQ_UPDATE_FIXTURES=1`, each diff read.
- `tools/gate.sh` exits 0 on the close's tree.

## Not in scope
- #316 (a weighted sum of pseudo totals), #309, #317.
- Any other kind of form (`opposite`, `numberless`, an `other` that is
  not a plural): their status and notes stay.
- The pages beyond the one `web.rs` case; the look (#285).

## Rules
`TEMPLATE.md`. Every `acq` is in a mock world (the mock-session
skill); tooling sends nothing to the trade site. Land the captures as
the site-sitting skill says; the recording stays under `raw/`, never
committed. A choice of shape with two defensible answers goes to the
design session; the rest carries on.

## Report
`TEMPLATE.md`.
