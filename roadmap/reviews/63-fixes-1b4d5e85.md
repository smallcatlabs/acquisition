16a89a85^..1b4d5e85 — the five prior findings are addressed and pinned in source; one new repair-command defect remains.

- **[P3, shape 6] [price_cmd.rs:234](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/price_cmd.rs:234):** The new status text promises a clearing command even for malformed stored addresses, which `pricing::clear` rejects before reaching the store. **Failing persisted input:** a buyout row with scope `tab`, key `pc`, revision 9—the existing listing fixture already includes this address—prints `acq price clear --account a tab/pc --if-revision 9`, but a tab target requires `tab/<realm>/<id>`. Emit the clearing command only when the address parses as a `PriceTarget`; otherwise retain an accurate diagnostic, and pin the semantic validation rather than clap parsing alone.

- **What I did not check:** Execution, test results, and gate exit statuses: **not verified**, as requested. Unchanged code was read only where needed to trace affected callers; no send path changes in this range. The MCP reference change matches its source description.

- **Nitpicks:** Finding 1 qualifies: normal price writes cannot produce malformed addresses, although persisted malformed addresses are explicitly handled and tested.

- **Brief:** No change proposed.
- **Plan:** No change proposed.
- **Prompt:** No change proposed.
