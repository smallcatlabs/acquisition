9cfe13a2^..17ab853c — not fully met: the added documentation preservation and accuracy properties fail.

Findings, most severe first:

1. **P2 — `crates/acquisition-store/src/lib.rs:138`, shapes 3, 7:** The retained restart summary still says running jobs replay apart from its listed exceptions, contradicting the corrected daemon documentation. **Persisted input:** a running stash job named by a fault marker fails without replay under `restore_jobs`. Replace this second replay specification with a pointer to `restore_jobs`’s authoritative description.

2. **P2 — `crates/acquisition-daemon/src/daemon/queue.rs:38`:** The cuts remove unique rationales without giving them a committed home, violating added property 1. **Concrete reading:** the explanation for leaving outcomes uncompressed—dependency cost, readability in `sqlite3`, and compression remaining a local change—is gone; CLI rejected alternatives disappear similarly at `crates/acquisition-cli/src/plan_cmd.rs:20`. Restore these explanations compactly or move them into a committed document; the unprovided report cannot satisfy the tree-preservation requirement.

3. **P2 — `crates/acquisition-plan/src/lib.rs:162`, shape 1:** The rewritten description incorrectly makes a fresh policy-row read and refusal on a missing row universal. **Client input:** a valid selection plan with `basis.policy_revision = None` succeeds without reading a policy row (`check_spendable`, line 1512). Qualify policy checks as applying only to policy-derived plans.

4. **P2 — `crates/acquisition-daemon/src/daemon/kinds.rs:11`, shape 3:** The rewritten apply vocabulary still lists only stash actions. **Client input:** a valid apply containing `{"kind":"characters","params":{"realm":"pc"}}` passes `validate_apply`; named `character` actions also pass. Include both character kinds or point to the validator’s complete vocabulary.

5. **P2 — `crates/acquisition-plan/src/lib.rs:149`, shape 1:** The rewritten empty-stub description omits the requirement that the store also holds no items there. **Persisted input:** a stale map substash with `metadata.items = 0` and `item_count = 1` is fetched, because `is_empty_substash` requires both counts to be zero. Add that condition.

6. **P2 — `crates/acquisition-cli/src/plan_cmd.rs:32`, shape 3:** The rewritten failure description retains the obsolete `acq jobs` remedy. **Concrete reading:** after eleven children fail, `job_report` prints `acq result <parent-id> --json` to locate every failed id. Update the description to name that result command.

7. **P2 — `crates/acquisition-plan/src/shop.rs:29`, shape 3:** The retained mechanism paragraph says manual `no_price` is blocked, contradicting both `cell` and the later module documentation. **Persisted input:** an otherwise eligible stash item whose effective manual statement is `no_price` posts under an empty spoiler title. Describe that branch and exempt it from the following “other than exact or negotiable” sentence.

8. **P3 — `crates/acquisition-daemon/src/daemon/routes.rs:135`, shapes 3, 7:** The function documentation still says route knowledge is kept “in one place,” immediately below the corrected two-declaration map. **Concrete reading:** this function declares only policylessness; `NO_PROBE_ROUTES` separately controls probing. Update the sibling sentence to name both locations.

Findings 3–8 share a cause: older descriptions were retained or rewritten without fully reconciling their conditions with current code.

- **Not checked:** execution evidence—gate results, script imports/checks and test counts—is **not verified**, as requested. Static inspection found the eight pin substitutions consistent with `PINS` and the committed headers; no goldens or references changed.
- **Nitpicks:** finding 8 is minor because the module’s opening paragraph supplies the correct map.
- **Brief:** require a committed home for unique rationales; sending them only to the worker report conflicts with added property 1.
- **Brief:** explicitly permit the required ordinary `//` edit at `ratelimit/choke.rs:226`; it contradicts the literal `//!`/`///`-only done-when.
- **Plan:** no further change suggested.
- **Prompt:** no further change suggested.
