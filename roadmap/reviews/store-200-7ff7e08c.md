7ff7e08c — #200’s source-level done-when holds; no behavioral defects found.

- **P3 — crates/acquisition-daemon/src/mockggg.rs:1445, shape 1:** The commit message miscounts the replacements as 17 production calls and 31 test calls; the committed tree has 13 production calls and 35 test calls, including this call inside `#[cfg(test)]`. Correct the census in a follow-up commit message.
- **P3 — crates/acquisition-verbs/tests/documents.rs:2790, shape 3:** The comment still says a JSON value would sort the document’s keys, which is false with the newly enabled feature. Describe that behavior explicitly as historical.

- **Not verified:** Builds, gate stages, fixture regeneration execution, and the unchanged receipt test’s passing result; execution was prohibited.
- **Nitpicks:** Both findings concern words alone and do not block the implementation.
- **Brief:** The supplied issue and owner’s ruling adequately define completion.
- **Plan:** No separate plan was supplied or needed for this review.
- **Prompt:** Nothing obstructed the review.
