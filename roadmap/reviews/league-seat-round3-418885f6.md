98a1a741^..418885f6 — the three rerun faults are fixed and pinned, but the single wording-pin requirement is not met.

- **P3 — [price_cmd.rs:1625](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/price_cmd.rs:1625), shape 10:** Wording has redundant pins: the no-price sentence remains a unit-test literal beside its new golden, and the shared `Standing` sentence is asserted in planner, CLI and MCP tests. Concrete failure: rewording either sentence requires updating several independent expectations despite unchanged behavior. Keep the price golden, give `Standing` one wording golden, and retain variant, field and adapter assertions elsewhere. This is one shared cause; no runtime defect was found in the changed paths.

- **Not checked:** execution, gate statuses and test counts — **not verified**, as requested; no builds, tests or daemons were run.
- **Nitpicks:** the duplicate wording pins are a maintenance nitpick, with no failing client, persisted or upstream input identified.
- **Brief:** add the rerun’s three faults to the brief so its acceptance criteria match this round.
- **Plan:** no further change suggested.
- **Prompt:** no impediment.
