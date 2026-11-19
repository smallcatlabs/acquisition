e9bcae07 — the fix round’s done-when does not fully hold.

- **P2 — `crates/acquisition-daemon/src/daemon/quote.rs:102`, shape 3:** The policyless branch still promises `eta_seconds: 0` for a permanently closed route. Concrete upstream/client sequence: `/profile` first answers a headerless 200, later accumulates invalid answers until permanently closed, then a client quotes a `profile` job. The choke point returns no ETA, but this branch substitutes zero beside the reset advice. Apply permanent-closure handling independently of the endpoint’s policy state and add this case to the quote table.

- **P2 — `crates/acquisition-cli/tests/apply_loop.rs:331`, shapes 2 and 10:** The replacement pin exercises only an apply with directly held children, leaving the original deep-stash failure unpinned and covering less than the retired test. Concrete regression: restrict `parent_held` to `apply`; this test still passes while client input `acq stash <map-tab> --deep` can again wait forever after upstream invalid substash answers permanently close its route. The retired test also covered nested descendants and exclusions for moving children. Extend the existing daemon parent tests with a deep parent and mixed moving/held descendants, and pin the direct deep-stash wait through the frontend.

- **Not checked:** Execution, mock scenarios and gate success are **not verified**, as requested; this was a source-only review.
- **Nitpicks:** None.
- **Brief:** The amended log requirement resolves the earlier journal ambiguity.
- **Plan:** No change suggested.
- **Prompt:** No change suggested.
