770403d4 — fix-round done-when partially holds: both search gaps are addressed; the history finding remains.

- **P3 — [tools/trade-sheet.py:3007](/Users/tom/Development/GitHub/smallcatlabs/acquisition/tools/trade-sheet.py:3007), shapes 3 and 11:** The history cleanup removes the sitting narrative from the docstring but adds a review narrative in a comment. Reading “Appended after the round’s cold review” encounters history outside the records again. Delete that recap and its review link; retain the explanation that committed searches stay unchanged.
- **Not verified:** gate status, test counts, composer checks, link acceptance and live results; execution was forbidden. Static review covered the diff, generated row text, affected helpers and readers, and cited committed evidence.
- **Nitpicks:** the sole finding concerns words alone and has no runtime effect.
- **Brief:** the prior review supplies a clear fix-round scope.
- **Plan:** no separate plan was supplied.
- **Prompt:** the supplied #33 text has six entries while the commit cites items 9–11; supplying the matching revision would clarify those references.
