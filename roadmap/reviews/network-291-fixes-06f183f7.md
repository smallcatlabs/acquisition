06f183f7 — the fix round’s done-when does not fully hold.

- **P2 — `crates/acquisition-daemon/src/daemon/quote.rs:150`, shapes 1 and 3:** Flooring the ETA at the cooldown still omits pacing after a non-sampled route’s closure ends. Concrete upstream state and valid client input: `stash/xbox@A` and `stash@A` share the learned stash policy; Xbox is open, PC is closed for 3600 seconds, and the quote requests one Xbox fetch plus 31 PC fetches. Xbox sorts first and supplies the projection, so the floor quotes only the remaining hour, although 31 PC requests exceed the policy’s 30-per-300-second window and cannot all dispatch when that hour ends. Project the scope from its latest relevant closure end, rather than flooring a projection made from now, and pin a closed non-sampled route with enough requests to require further pacing.

- **P3 — `crates/acquisition-cli/tests/apply_loop.rs:357`:** The new timeout path leaves the apply child without cleanup. Concrete test failure: the reset succeeds but the apply hangs; the deadline assertion panics, dropping a raw `std::process::Child` without killing or reaping it. Wrap the spawned apply in the existing `harness::Proc` guard before waiting, as the previous review requested.

- **Not checked:** Execution, including builds, tests, the gate and documentation stage, is **not verified**, as requested.

- **Nitpicks:** The child-cleanup finding concerns an uncommon test failure path; I would call that a nitpick. The shared-policy estimate affects valid client requests.

- **Brief:** No change suggested for this fix round.
- **Plan:** No approved plan was supplied; I assessed the round against the findings its commit names.
- **Prompt:** No change suggested.
