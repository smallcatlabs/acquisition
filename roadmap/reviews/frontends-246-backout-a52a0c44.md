69602ff1^..a52a0c44 — #246’s done-when is not fully met: the normal submission paths enforce the rule, but the error path and supporting guidance have gaps.

- **P2 — tools/persist-check.sh:156:** The driver checks persisted accounts although refresh requires an account among all known accounts. With persisted Alice and logged-out Bob in the persisted index, running without `--account` passes preflight, starts a daemon, then aborts when refresh refuses `ambiguous_account`, leaving the daemon running. Count all known accounts and require the selector before starting the daemon.

- **P3 — crates/acquisition-verbs/src/jobs.rs:75:** An unreadable persisted account index silently permits implicit session selection. If Alice is the daemon’s sole live session and the Alice/Bob index becomes malformed or unreadable, an accountless `stashes` request falls through to `account: None` and runs as Alice despite being unable to establish that only one account is known. Preserve the `NoAccounts` fallback, propagate index-read failures, and pin that distinction.

- **P3 — README.md:219, shapes 1 and 3:** The new account-selection rule is stated more broadly than implemented. With two known accounts and one live session, `acq auth logout` still ends that session without an account; accountless `sleep` also remains allowed. Qualify the paragraph to account-requiring verbs and state the logout exception, as the CLI help does.

Execution checks, gate statuses, test counts, and live/browser behavior: **not verified**, because execution was prohibited.

Nitpicks: the README wording finding; the unreadable-index case is unlikely but affects dispatch behavior.

Prompt: the supplied last comment establishes the job requirement but omits the reversal of main accounts and `ACQ_ACCOUNT` cited by the commits; include that ruling to make authorization independently reviewable.
