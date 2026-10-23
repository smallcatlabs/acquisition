# Codex review, round A (3e53a45e..04eb4ff1), codex-cli 0.163.0, read-only, 166,227 tokens

3e53a45e..04eb4ff1 — done-when not verified: no 2f brief exists in the committed tree, and the range has unresolved findings.

- **[P1] [choke.rs:836](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/ratelimit/choke.rs:836), shapes 1, 2:** The origin check covers only the initial URL; reqwest still follows redirects automatically. An allowed HTTPS request receiving `302 Location: http://api.pathofexile.com:443/x` retains its bearer because the locked reqwest implementation removes authorization only when the host or effective port changes. Redirected exchanges also lack separate limiter admission and journal entries. Smallest fix: disable automatic redirects on the shared HTTP client and add GET/HEAD redirect refusal cases.

- **[P2] [kinds.rs:369](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/kinds.rs:369), shape 2:** Invalid restored jobs authenticate before the new URL refusal, so they can send a refresh POST or report an authentication error instead of the promised parameter failure. Concrete case: a restored stash job with `{"id":"t1","sub":".."}` and an expired access token. The new test conceals this by installing a fresh token. Smallest fix: validate/render the URL before authentication in every affected network arm, and test an expired session.

- **[P2] [queue.rs:600](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/queue.rs:600), shape 5:** Wrong-type refusals echo an unbounded JSON value despite bounding invalid strings. A `league` array containing a string of 20 MiB of backslashes fits below the 64 MiB request limit, but its twice-escaped refusal exceeds the response limit, losing the structured refusal. The sleep check has the same cause. Smallest fix: describe invalid values by type and size, with a bounded preview.

- **[P3] [queue.rs:533](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/queue.rs:533), shapes 1, 4:** The unknown-kind refusal recommends kinds unavailable in the active provider mode. Submitting `bogus` in real mode says a client may submit `sleep`, `fetch`, and `whoami`, each immediately refused there. Smallest fix: filter the suggested kinds by provider mode or explicitly mark mock-only entries.

- **Not checked:** Builds, tests, quality gates, signal delivery, checkpoint fault races, and live API behavior were not verified, as instructed. The findings follow from static inspection, including the locally available source matching the locked reqwest version.
- **Nitpicks:** The provider-mode suggestion is minor. The oversized refusal requires unusual malformed input, although the response-limit failure is concrete.
- **Brief:** Supply a committed 2f round-A brief with explicit done-when criteria.
- **Plan:** No further change suggested.
- **Prompt:** For review ranges, permit a round brief to be named once instead of requiring every commit to name it.
