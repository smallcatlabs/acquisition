# Codex review: the h1 fix round, `bd768467^..afeab845`

The outside review of the five fix-round commits (`roadmap/briefs/h1-fixes.md`), run 2026-10-26 by the committing session
(`codex exec --sandbox read-only`, the prompt `roadmap/briefs/REVIEW-PROMPT.md`). The findings verbatim; the owner reads them before any fix is built.

bd768467^..afeab845 — the brief’s done-when does not fully hold; execution-dependent test and gate conditions are not verified.

1. **[P1] [daemon/login.rs:424](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/login.rs:424), shape 1:** Logout can acknowledge success while leaving the account restorable under its old name. Input: refresh A→B saves B but cannot write the index, leaving A persisted and uncleared; restore directory writability, then logout B. `set_persisted(B, false)` silently succeeds without finding B, and restart restores A. Resolve the indexed account by the session’s uuid, clear its indexed names, and mark that actual entry not persisted.

2. **[P2] [daemon/refresh.rs:173](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/refresh.rs:173):** The rename and its cleanup reference are separate durable writes, so interruption can orphan the old credential. Input: rename A→B; `record_login` saves B with no held name, then the process exits—or `hold_keyring_name` fails—before A is recorded. Subsequent logout reaches B alone. Add one index operation that saves the new username and held name together, and pin interruption between the current writes.

3. **[P2] [daemon/login.rs:486](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/login.rs:486), shape 3:** Forget still clears credentials outside the lock that orders login persistence. Schedule: forget clears non-live A; login A then saves its new credential, marks persisted and installs its session; forget finally acquires `shared`, marks A not persisted and acknowledges success. A live session and credential remain after the acknowledged logout. Perform the live/non-live decision, index read, clears and mark under one hold of `shared`, rechecking session state there.

4. **[P2] [daemon/refresh.rs:172](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/refresh.rs:172), shape 1:** A second rename can discard an uncleared name despite the promise that none becomes unreachable. Input: A→B fails clear(A), leaving `keyring_name=A`; B→C then fails both old-name clears. The code replaces the held name with B, and logout C clears C and B while leaving A permanently unaddressable. Preserve every uncleared name durably; the single-name design needs an explicit policy for this case.

5. **[P3] [acquisition-cli/src/main.rs:1083](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/main.rs:1083), shapes 1 and 7:** JSON infers keyring success from arbitrary text inside the daemon’s sentence. Input: the supported mock username `keyring entry cleared`, followed by logout with the keyring off; the username satisfies `message.contains(...)`, producing `"keyring":"cleared"` although nothing was cleared. Carry the keyring outcome as structured protocol data instead of searching prose.

- **Not checked:** Builds, test results, historical red runs, gate stages, actual crash behavior and platform behavior: execution was prohibited. The commits describe red runs and exceptions but contain no stage-by-stage gate exit evidence. Pins covering the seven original faults are present; the review deletion and historical citation are present; no goldens changed, and the reference changes follow the help edit.
- **Nitpicks:** Finding 5’s concrete username is unlikely in ordinary use. Findings 1–4 affect the stated credential lifecycle guarantees, including when their triggering failures are uncommon.
- **Brief:** Define how the single held-name field must handle another rename before cleanup succeeds.
- **Plan:** Add failed-rename persistence followed by logout, interruption between index mutations, and concurrent forget/login to the recovery scenarios.
- **Prompt:** No change needed.
