02b83466^..a3bc5c38 — #246’s requested behavior is only partially met; the done-when does not fully hold.

Findings, most severe first:

- **P1 — `crates/acquisition-verbs/src/frontend.rs:353`: Account selection fails open when persisted main-account intent cannot be read.** With Alice configured as main, an unsupported persisted value such as `{"version":2,"uuid":"alice-uuid"}`, and only Bob logged in, `daemon_selector` swallows the read failure and submits without a selector, allowing the daemon to run Bob’s job. Return and propagate storage/parse failures; preserve the existing fallback only where no main account is configured.

- **P1 — `crates/acquisition-mcp/src/server.rs:900`, shape 7: The selected account is read twice, so attribution can disagree with the operation.** A concurrent client changing the main account between `daemon_selector` here and the second resolution inside `jobs::submit` can produce a job for Bob with `main_account` naming Alice. The affected `price_clear` caller similarly resolves intent and facts separately at lines 748 and 756. Resolve one `Account` per operation and reuse it for the effect, reads, and attribution.

- **P1 — `crates/acquisition-verbs/src/main_account.rs:236`: Unreadable persisted intent is handled inconsistently, allowing blind replacement and false undo receipts.** For a valid JSON row with unsupported `version:2`, `Expect::Current` accepts the raw row and overwrites it without the explicit revision required for unreadable values; for malformed persisted JSON replaced at its named revision, line 242 discards the prior, causing the receipt to claim `prior:null` and offer a clear as an undo. Model readable, unreadable, and absent priors separately, as saved searches do; refuse an unreadable current value and retain its revision/fault on deliberate replacement.

The next two findings share one cause: chosen-account metadata is tracked in frontend adapters instead of travelling with every verb result and failure.

- **P2 — `crates/acquisition-cli/src/run.rs:71`, shape 3: Most CLI JSON answers omit the required `main_account` field.** With Alice main among several accounts, `acq tabs --json` writes attribution only to stderr; its JSON consumer cannot discover that the default was used. Apply the shared `main_account::mark` rule to affected JSON answers and refusals. The commit’s stated follow-up does not satisfy the ruling that landed in this range.

- **P2 — `crates/acquisition-mcp/src/server.rs:831`, shape 3: Saving a search without a realm silently takes the main account without disclosing it.** Client input `saved_search_set {"name":"Rare","text":"rarity=rare"}` resolves the realm through `saved::resolved_realm`, which calls `Frontend::account` directly and bypasses `took_main`; both its receipt and subsequent refusals lack attribution. Carry the chosen-account metadata out of realm resolution and through the receipt/refusal path on both frontends.

- **P3 — `crates/acquisition-cli/src/cli.rs:39`, shape 3: Global account help still describes the old selection rule.** With several known accounts and a main account set, `acq --help` still promises refusal and says jobs take the sole logged-in account; the same stale paragraph remains in `CLI-REFERENCE.md:47`. Update the source help to explain main-account precedence and regenerate the reference.

Execution, gate results, test counts, and live/browser behavior: **not verified**, as this review was restricted to reading.

Nitpicks: the stale-help finding is wording-only; the other findings affect selection, persistence, receipts, or the machine-readable contract.

Brief/plan: the supplied issue has no explicit done-when list, so I judged its requested behavior together with the ruling committed in this range.

Prompt: no change needed.
