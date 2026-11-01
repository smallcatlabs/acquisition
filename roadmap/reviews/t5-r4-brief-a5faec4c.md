`roadmap/briefs/t5-r4.md` at `a5faec4c`, unchanged in the tracked tree, is buildable as drawn.

1. **Narrower, sound** — 799 retains its component holder; the same-account exchange cannot overtake the held refresh’s token-policy permit.
2. **Answered** — both sessions start refresh-due; advancing past the warmed token’s expiry produces `[other POST, other GET, other POST held]`. `advance` moves both clock faces; no lifecycle timer runs by default. After Running, the specified job reaches the gate without another pending wait: authentication precedes HTTP, and token pacing is ready in this setup. `quiet` therefore establishes the intended gate wait.
3. **Answered** — Do 1 requires the account fixture to answer code grants before 458 uses it; its unmatched hold records an erroneous POST, and `finish` awaits no account-server completion.
4. **Answered** — the three compared fields remain equal on the frozen parking clock; credential persistence publishes the final session keyring word and notifies `await_auth`.
5. **Answered** — restoration fits between daemon construction and dispatcher spawn; finishing the first door leaves the separately owned database available to the second connection.
6. **Narrower, sound** — 893 retains the existing same-account component holder instead of substituting an interleaving that tests less.
7. **Answered** — the obstruction is now precisely a directory at the index-file path, leaving the facts database writable.
8. **Narrower, sound** — 203 observes the surviving session through its distinct seeded expiry, which `abort_login` preserves and `AuthStatus` exposes; 826 requires a leave strictly before the configured drain bound, so a leaked exchange count cannot pass by timing out.
9. **Answered** — the held response plus `quiet` puts the second caller on the flight before rejection; the later caller encounters the dead-grant mark and sends no POST.
10. **Answered** — `settle` waits for the counted blocking landing to finish, including clearing the mark after publishing `login_ok`.
11. **Answered** — the builder accepts an empty session table directly, and `AuthStatus.logged_in` then evaluates to false.
12. **Answered** — the revised searches distinguish scenario attributes and exempt the retained component yield and parking-clock datum.

Findings, most severe first:

- **Do 4 / Goal — non-blocking count inconsistency:** the Goal says “two of its scenarios,” while Do 4 explicitly migrates four cancellation scenarios. The existing functions are at `crates/acquisition-daemon/src/daemon/tests/dispatch.rs:763`, `:829`, `:862`, and `:897`; Do 4 agrees with that inventory. **Smallest brief change:** remove “two of its scenarios and” from the Goal’s parenthesis.

What I did not check: builds, tests, gate results, CI, timings, or executable implementations of the proposed additions; this was the requested read-only brief review. I did not repeat the first review’s complete unchanged citation inventory.

Nitpicks: only the Goal’s scenario-count wording above.

**Round 4 may start as drawn.**
