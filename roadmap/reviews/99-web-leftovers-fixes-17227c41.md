17227c41 — both prior findings are fixed and pinned; the fix-round’s full done-when remains unconfirmed.

- **P3 — [main.rs:246](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/main.rs:246), shapes 1 and 3:** The help still promises startup whenever no account exists, omitting the selector exception this fix restores. Concrete client input: `acq --account Typo web` against an empty persisted index exits before binding, despite “it starts all the same.” The same claim appears in `CLI-REFERENCE.md`; the README also omits the qualification. Smallest fix: say startup is deferred only when neither `--account` nor `ACQ_ACCOUNT` names an account, regenerate the reference, and qualify the README sentence.

- **Not checked:** Gate results and runtime behavior are **not verified** because execution was prohibited; no browser or upstream GGG behavior was exercised.
- **Nitpicks:** None; the wording affects supported selectors, including an inherited `ACQ_ACCOUNT`.
- **Brief:** Correct Do 1’s empty-index explicit-selector kind from `unknown_account` to `not_logged_in`, as the preceding review already explains.
- **Plan/prompt:** Nothing impeded the review.
