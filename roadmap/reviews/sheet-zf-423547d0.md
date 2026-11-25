423547d0 — partial: searches are prepared; the supplied issues’ done-when remains pending a sitting, with two gaps in the preparation.

- **P2 — [tools/trade-sheet.py:2954](/Users/tom/Development/GitHub/smallcatlabs/acquisition/tools/trade-sheet.py:2954), shapes 1 and 2:** zf08 requests only total life while promising to show all four totals. An upstream result where life counts the line but Intelligence still omits it produces the same observations as all four being fixed: zf07 finds none and zf08 finds jewels. Require the ordinary line independently and request all four totals in an `if` group, then compare each against the fetched item’s contributing lines; regenerate the sheet.

- **P2 — [tools/trade-sheet.py:2907](/Users/tom/Development/GitHub/smallcatlabs/acquisition/tools/trade-sheet.py:2907), shapes 1 and 2:** counting placeholders overlooks an available non-ranged three-number case, so it does not justify omitting that question. The upstream vocabulary includes `explicit.stat_723388324`: `#% chance … # Mana … 0.1 second Cooldown`; the catalogue explicitly includes two placeholders beside a written digit under `stat.text.many`, and `trade-translate.toml:911` already exercises this case. Add a controlled bounded search for this shape, or retain it explicitly as unasked with a reason that addresses it.

- **P3 — [tools/trade-sheet.py:2895](/Users/tom/Development/GitHub/smallcatlabs/acquisition/tools/trade-sheet.py:2895), shape 11:** the new docstring puts the dates and narrative of earlier sittings into code. Move that recap to the commit record and describe the function’s questions with pointers to their evidence and issues.

- **Not verified:** gate status, test counts, composer checks, link acceptance and live results; execution was forbidden.

- **Nitpicks:** only the historical docstring finding; the two search-design gaps affect what the owner can establish at the sitting.

- **Brief:** the supplied issues describe sitting outcomes, without a separate done-when for composing this round.
- **Plan:** no separate plan was supplied.
- **Prompt:** the supplied #33 text has six entries, whereas the commit cites items 9–11; supplying the matching issue revision would clarify scope.
