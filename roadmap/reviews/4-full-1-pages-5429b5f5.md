5429b5f5 — the brief’s done-when does not fully hold: the no-state pin omits pages Do 9 requires it to check.

- **P2 — [web.rs:1384](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/tests/web.rs:1384), shapes 1/2:** The scan checks a separate subset of responses rather than every recorded page. Concrete gap: plan, jobs, saved-conflict and clear responses are absent, so a `Set-Cookie` introduced on one of those paths would pass this pin. Scan responses through the common recording path, including their referenced scripts, and add a breaker on an omitted page.

- **P3 — [mod.rs:1425](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/mod.rs:1425), shapes 7/4:** Opening an unreadable saved search reads the persisted row twice, allowing the second read to contradict the first and produce a refusal without a remedy. Concrete interleaving: `saved_at` reads an unreadable row; another writer replaces it with a readable row; `request_of` succeeds; the fallback nevertheless says this build cannot read it and offers no list. Construct the refusal and list offer from the `UnreadRow` already read, preferably through a verbs helper.

- **P3 — [mod.rs:70](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/mod.rs:70), shape 3:** The existing refusal documentation still says every offer is a link and only search/plan/tabs ways belong to pages. A saved-write conflict now renders a POST form, directly contradicting that reading. Update this paragraph to describe saved links and write forms alongside the new section.

- Execution evidence—gate results, test counts and browser behavior—is **not verified**, as requested; this was a read-only review.

- Nitpicks: the unreadable-row race is unlikely in ordinary use; the stale documentation is also a nitpick. The scan coverage gap is an explicit missing brief requirement.

- Brief: I would require the scan at the page-recording seam to make “every recorded page” enforceable.
- Plan: no issue impeded review.
- Prompt: no issue impeded review.
tokens used
155,258
5429b5f5 — the brief’s done-when does not fully hold: the no-state pin omits pages Do 9 requires it to check.

- **P2 — [web.rs:1384](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/tests/web.rs:1384), shapes 1/2:** The scan checks a separate subset of responses rather than every recorded page. Concrete gap: plan, jobs, saved-conflict and clear responses are absent, so a `Set-Cookie` introduced on one of those paths would pass this pin. Scan responses through the common recording path, including their referenced scripts, and add a breaker on an omitted page.

- **P3 — [mod.rs:1425](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/mod.rs:1425), shapes 7/4:** Opening an unreadable saved search reads the persisted row twice, allowing the second read to contradict the first and produce a refusal without a remedy. Concrete interleaving: `saved_at` reads an unreadable row; another writer replaces it with a readable row; `request_of` succeeds; the fallback nevertheless says this build cannot read it and offers no list. Construct the refusal and list offer from the `UnreadRow` already read, preferably through a verbs helper.

- **P3 — [mod.rs:70](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/mod.rs:70), shape 3:** The existing refusal documentation still says every offer is a link and only search/plan/tabs ways belong to pages. A saved-write conflict now renders a POST form, directly contradicting that reading. Update this paragraph to describe saved links and write forms alongside the new section.

- Execution evidence—gate results, test counts and browser behavior—is **not verified**, as requested; this was a read-only review.

- Nitpicks: the unreadable-row race is unlikely in ordinary use; the stale documentation is also a nitpick. The scan coverage gap is an explicit missing brief requirement.

- Brief: I would require the scan at the page-recording seam to make “every recorded page” enforceable.
- Plan: no issue impeded review.
- Prompt: no issue impeded review.
