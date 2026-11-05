215ef295^..c8a3ed0f — the brief’s done-when does not fully hold: web refusal recovery remains incomplete.

- **[P2] [web/mod.rs:524](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/mod.rs:524), shape 3:** `Paged::words` retains CLI remedies even where an existing page can perform the recovery. Concrete input: POST an earlier plan envelope after the persisted policy revision changes; `Fault::Replan` prints `acq refresh --plan …`, with no recovery link, although `/plan` already handles that account, realm and league. Smallest fix: emit a `Way::RefreshPlan` for `Replan`, render page wording for that remedy, and pin the stale-policy Apply refusal and its followed link.

- **[P3] [web/mod.rs:435](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/mod.rs:435), shapes 1 and 4:** The reverse fold drops counts views that the existing search form supports, silently removing their realm offers. Concrete client input: `/search?text=id%3At1&view.count=rarity` against a persisted store holding multiple realms; each offered counts request is reconstructed as rows, fails the equality check, and disappears. Smallest fix: preserve `view.count` and its limit when reconstructing form fields, and pin realm links that retain the counts view.

- **Not checked:** builds, tests, gate exit status and fixture regeneration are **not verified**, as execution was prohibited; report-only inventories and stop reasons were unavailable by design.
- **Nitpicks:** finding 2 is a practical edge case because the normal form supplies a realm; finding 1 is ordinary stale-plan recovery.
- **Brief:** its report-only done-when criteria cannot be assessed from commits alone.
- **Plan:** no separate plan was supplied.
- **Prompt:** no other impediment or suggested change.
