b3ab68f0^..380fccb1 — Do 2–4’s done-when does not fully hold; execution-dependent acceptance is **not verified**.

- **[P2] [refresh.rs:298](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/refresh.rs:298): An account-index read failure still lets a restart resurrect the rejected bearer.** Restore `at-kept` with an hour remaining, make `accounts.json` temporarily unreadable, receive its 401, stop, restore readability, and restart. `persist_credentials` returns at line 162 before attempting any keyring write or clear; the persisted entry still contains `at-kept`, which restore installs again. Make invalidation of an existing credential independent of successful index bookkeeping, preserving credential ordering, and pin this failure-and-restart sequence.

- **[P3] [TESTING-NOTES.md:102](/Users/tom/Development/GitHub/smallcatlabs/acquisition/TESTING-NOTES.md:102), shapes 1 and 3: The expiry claim remains broader than the behavior, with a literal-expiry sibling left standing.** A restored token with 3600 seconds remaining still causes a POST when `auth check` forces refresh or after rejection; the revised table and module comments omit these exceptions. Meanwhile, [tests/refresh.rs:1096](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/refresh.rs:1096) still says “refreshes only at expiry,” contradicted by the new five-second test. Qualify the claims as ordinary reuse without rejection or forced refresh, and update the remaining test prose.

- **Not checked:** Builds, tests, baseline failure reproduction, OS keyring behavior, live GGG behavior, and the gate, as instructed. Static inspection confirms the required test declarations, the two production serialization sites, the pins-table row, and C135’s built citation. Wire fixtures, goldens, and references are unchanged. Test success and gate exit statuses remain **not verified**; Do 5–7 were excluded.

- **Nitpicks:** Finding 2 is wording-only and could be treated as a nitpick operationally; finding 1 is not, despite requiring an uncommon index failure.

- **Brief:** Reconcile literal-expiry acceptance with the retained refresh margin and explicit refresh exceptions.
- **Plan:** Include restart after failed invalidation alongside successful persistence.
- **Prompt:** Give fix rounds their own acceptance lines, separate from the original brief’s execution-dependent requirements.
