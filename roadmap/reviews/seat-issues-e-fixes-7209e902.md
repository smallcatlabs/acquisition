9009af8f..7209e902: the fix round’s done-when does not hold; failure classification remains incomplete across sibling paths.

- **P2 — [auth.rs:181](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/auth.rs:181), [kinds.rs:882](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/kinds.rs:882), shape 3:** `not_retried` is applied to GET failures, while a refresh’s ordinary 503 and a HEAD’s ordinary 403/503 still become `upstream`. Concrete upstream input: an expired session’s `whoami` refresh receives an origin 503 without Cloudflare signatures; its result offers another `submit_job`/`acq submit`, repeating the POST the new classification should leave for human inspection. Classify these sibling statuses as `not_retried`, preserve the rejected-grant mark where applicable, and pin their resulting offers alongside their kinds.

- **P2 — [dispatch.rs:165](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/dispatch.rs:165), shapes 3, 7:** The degraded-route adapter still replaces the prerequisite’s typed failure with `upstream`, so the client’s job receives different recovery advice from its probe. Concrete input: a client submits `characters` with an expired session, and the probe’s refresh receives GGG’s `400 invalid_grant`; the probe fails `not_logged_in`, but `characters` fails `upstream` and offers resubmission although login is required. The HEAD-429 table likewise explicitly retains `upstream` for the waiting fetch while changing its probe to `rate_limited`. Preserve the typed cause through route closure and pin the original client job’s kind and offers.

- **Not verified:** gate/tests, failing-before claims, and printed-command execution; execution was prohibited.
- **Nitpicks:** none.
- **Brief:** no changes suggested.
- **Plan:** no separate plan was supplied.
- **Prompt:** no changes suggested.
