084ec530^..ddf055ab: the fix round’s done-when does not fully hold; the apostrophe case is pinned, but the token-free logging finding remains incomplete.

Findings, most severe first. Both share one cause: arbitrary upstream header strings still reach log formatting.

1. **crates/acquisition-daemon/src/daemon/kinds.rs:717, shape 3:** A successful HEAD probe still logs its raw rate-header snapshot. Concrete upstream input: a HEAD 204 with valid policy headers plus `X-Rate-Limit-Debug: rt-live-w4Qe` writes that token to `daemon.log`; the new API test puts its token header only on the GET response. Smallest fix: render this line through the shared safe rate renderer, retaining raw headers only in the payload, and pin the successful-HEAD echo case.

2. **crates/acquisition-daemon/src/ratelimit/policy.rs:529, shape 1:** Parsing policy and rule names into unrestricted `String`s does not make them safe to log. Concrete upstream input: a token-endpoint 400 with `X-Rate-Limit-Policy: rt-live-w4Qe`, otherwise valid Ip windows, and `{"error":"invalid_grant"}` logs the token through `rate_words`; malformed rules also escape through parser diagnostics—for example, `X-Rate-Limit-Rules: rt_echoed` without its windows produces `missing x-rate-limit-rt_echoed`. Smallest fix: render upstream identifiers through fixed known labels or placeholders throughout rate rendering and parser/mismatch diagnostics, without narrowing what the limiter accepts, and pin both accepted-name and malformed-name cases.

What I did not check and why:

- Execution evidence, gate statuses and test counts: **not verified**, as requested.
- External CI and GitHub state were outside the committed evidence.

Nitpicks: neither finding; both require unusual upstream responses, but directly contradict the credential guarantee.

Brief: distinguish unrestricted upstream identifiers from values whose parsing actually constrains their content.

Plan: the previous review’s opening mentions “finding 4,” but enumerates only findings 1 and 2; its summary should match that list.

Prompt: nothing impeded the review.
