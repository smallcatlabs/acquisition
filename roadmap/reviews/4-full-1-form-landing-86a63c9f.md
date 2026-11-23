- `86a63c9f`: the brief’s done-when does not fully hold.

- **P2 — [web/mod.rs:1617](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/mod.rs:1617), shapes 3 and 9:** Search constructs `from` after `opening()` removes its opened item rows, so leaving and returning closes them. Concrete client input: `search?realm=pc&text=&view.fields=icon&item=r1`, with a matching `r1`; Save or Fetch the league list carries no `item=r1`. The changed `table_rows.txt` golden records this omission, while place pages preserve opened items. Build the search’s `FromPage` with its retained item fields and pin the return through a leaving form.

- **P2 — [web/mod.rs:3958](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/mod.rs:3958), shape 2:** A save conflict suppresses the editable save form required by the brief. Concrete case, already staged in `landing.txt`: the client submits name `mine`, but persisted input has advanced to a row named `elsewhere`; the refusal displays `elsewhere`, while `mine` survives only in a hidden retry field and cannot be reviewed or edited. Render the offered saved write through `save_form`, retaining the submitted name and source with the offered revision; assert the visible name input in the conflict case.

- Not verified: builds, tests, gate status, browser execution and fixture regeneration; execution was prohibited.

- Neither finding is a nitpick or requires an unlikely crafted input.

- Brief: sufficient to identify both gaps. Plan: no separate obstacle. Prompt: no change proposed.
