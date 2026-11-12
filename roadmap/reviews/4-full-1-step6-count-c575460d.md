c575460d — the brief’s done-when does not fully hold.

- **P2 — `crates/acquisition-cli/src/web/assets/count.js:45`:** A form change leaves the previous request eligible to publish until the next debounce timer fires. Concrete client-input sequence: request A starts, the user edits the query to B, then A completes within the next 300 ms; line 33 accepts it and displays A’s count beside B. Continued typing can prolong that misleading display. Smallest fix: abort the current controller and clear the outputs immediately in `changed`, then debounce only the next fetch.

- **P2 — `crates/acquisition-cli/src/web/mod.rs:1192`, shape 1:** The count still validates some view values before replacing the view, despite the promise that the view is not read. Concrete client input: `count?realm=pc&text=class%3Arings&view.count=line%3A%22life` is refused by `count_keys` for its unfinished quote; malformed `view.limit` and `view.fields` likewise fail before `counted()` runs. Smallest fix: skip value parsing for recognized view fields when folding a count, while retaining unknown-field validation, and extend the count test with malformed view values.

- **Not checked:** Execution-dependent requirements, including the gate and `--only doc`, are **not verified**; this was a read-only source review with no builds, tests, browser execution, or live traffic.

- **Nitpicks:** Neither finding is a nitpick; both arise from ordinary form editing.

- **Brief:** Its requirement that `count.js` appear in the head is also unmet: `assets/page.html` inserts scripts before `</body>`, and the new assertions check presence anywhere. Require inclusion on the page unless head placement serves a specific purpose.

- **Plan:** Nothing else impeded the review.

- **Prompt:** Nothing impeded the review.
