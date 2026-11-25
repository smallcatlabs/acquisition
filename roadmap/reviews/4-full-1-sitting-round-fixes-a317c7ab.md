a317c7ab — the fix-round’s done-when does not fully hold: one previously reported history comment remains.

- **P3 — [web/page.rs:1585](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/page.rs:1585), shapes 3 and 11:** The comment cleanup leaves a sibling production comment containing sitting history. **Failing reading:** “stop 6 of the owner’s sitting” remains in `results_as`’s documentation, despite the prior finding covering these narratives throughout the affected modules. Remove that parenthetical and retain the behavioral explanation. No behavioral defects found in the fixes.

- **Not checked:** builds, tests, browser behavior, and gate execution are **not verified**, as requested; no real-store or upstream interaction was performed.
- **Nitpicks:** the remaining finding is editorial and has no runtime effect.
- **Brief:** include #317’s body and comments as fixed local inputs.
- **Plan:** no further change suggested.
- **Prompt:** no change suggested.
