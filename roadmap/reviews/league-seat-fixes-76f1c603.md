847ad3f4^..76f1c603 — the ten fixes and their pins are present, but the printed-command requirement still has edge-case failures.

Findings, most severe first:

- **P2 — [plan_cmd.rs:540](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/plan_cmd.rs:540), shape 6:** Plan-derived commands prefer a historical account name over its stable UUID. **Input:** a saved plan for `Old#1`, followed by an account rename where the index retains only `New#1` with the same UUID. Rendering that valid plan prints `--account 'Old#1'`, which cannot resolve; applying it explicitly by UUID can succeed but print the same broken continuation. **Smallest fix:** use `account_uuid` for plan-derived commands and pin a saved-plan rename case.

- **P3 — [refusal.rs:342](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-verbs/src/refusal.rs:342), shape 5:** Covered-league output is unbounded in both refusal prose and offers; `preview_line` shares this cause. **Input:** a persisted policy covering eleven or more leagues prints every league and every offer despite the ten-entry default rule. **Smallest fix:** bound the names and offers, count omissions, and provide an account-qualified policy read for the remainder; pin an eleven-league case.

- **P3 — [main.rs:1225](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/main.rs:1225), shape 6:** The new refresh commands inherit scope arguments that fail when a value starts with `-`. **Input:** client policy JSON naming league `-L` is accepted, but its preview prints `--league -L`, which clap treats as an option; the uncovered-league offer fails likewise. **Smallest fix:** spell scope options as single `--name=value` arguments before shell quoting, and pin the printed command.

Not checked: execution, gate statuses and test counts — **not verified**, because this review was read-only.

Nitpicks: the leading-hyphen league and large-policy findings are unlikely in ordinary use; the rename finding affects a supported persisted-plan workflow.

Brief change: reconcile “offer each covered league” with the default list bound.

Plan change: use stable account identity in commands derived from saved plans.

Prompt: no impediment.
