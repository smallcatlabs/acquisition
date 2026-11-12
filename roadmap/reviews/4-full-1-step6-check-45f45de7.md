45f45de7 — the brief’s done-when does not fully hold: page readiness has a race, and the checklist retirement exceeds the tests’ coverage.

- **P2 — [crates/acquisition-cli/tests/browser.rs:230](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/tests/browser.rs:230):** Clearing every event received before the `Page.navigate` reply can discard the requested page’s load event and hang a healthy test until the runner kills it. The concrete input is local browser protocol traffic ordered as `Page.loadEventFired`, then the navigation reply; the comment assumes that load belongs to `about:blank` without identifying its navigation. Smallest fix: wait for the requested document’s URL, readiness and expected control, or correlate lifecycle events with the navigation’s loader.

- **P2 — [REVIEW-CHECKLIST.md:120](/Users/tom/Development/GitHub/smallcatlabs/acquisition/REVIEW-CHECKLIST.md:120), shapes 1 and 10:** The retirement claims that the tests hold the count’s writes to its own outputs, but both tests inspect only `matched`. A regression selecting `undecided` through `document.querySelector(...)` instead of `form.querySelector(...)` would write another form’s undecided count and visibility while satisfying every new assertion. This concerns presentation of the seeded persisted facts, not malformed client or upstream input. Smallest fix: retain the undecided-output isolation clause in the checklist; alternatively, test distinct undecided counts and visibility across forms.

- **Not checked:** gate/doc results, mutation failures, timeout cleanup and hosted runs are **not verified**; this was a read-only review with no execution.

- **Nitpicks:** neither finding concerns only a case unlikely in practice.

- **Brief:** distinguish the build’s acceptance criteria from the later close’s hosted-run record.
- **Plan:** no other change suggested.
- **Prompt:** nothing obstructed the review.
