a17bec8a — the brief’s done-when does not fully hold: the replacement test does not isolate every required count refresh.

- **P2 — [page.css:68](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/assets/page.css:68):** CSS-hidden numeric fields can still block Search through native form validation. Concrete client input: leave a weighted group’s bound or weight as an unfinished number such as `1e`, switch the group to `and`, then press Search. The irrelevant field remains invalid but cannot receive focus; the fold never gets to discard it. Smallest fix: prevent irrelevant numeric controls from blocking submission, and pin this sequence in the browser test.

- **P2 — [browser.rs:625](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/tests/browser.rs:625), shapes 2 and 10:** The replacement test lets subsequent typing or an unchanged total satisfy the promised count refresh after list operations. Concrete reading: the first pick is followed by typing `70` before checking the count, weighted picks are followed by typing weights, and adding the empty group preserves the previous total; the final group removal also preserves the preceding total. Consequently, the retirement loses the old test’s isolated pick-announcement coverage. Smallest fix: check list operations independently after earlier requests settle, using distinguishable totals—or observing the exact count request where totals legitimately stay equal—and ensure missing notifications fail the test.

Execution-dependent done-when lines, browser behaviour, tests and gate status: **not verified**, because execution was prohibited. Issue #184’s comments were not supplied; the committed brief and design were reviewed.

Nitpicks for unlikely cases: none.

Brief: add a browser case for invalid numeric input hidden by a type change.  
Plan: no other changes suggested.  
Prompt: no changes suggested.
