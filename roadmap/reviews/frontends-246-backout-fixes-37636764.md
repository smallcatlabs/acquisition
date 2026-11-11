37636764 — The three prior cases are corrected in source, but the fix round still has a preflight error-path gap.

- **P3 — tools/persist-check.sh:156–158:** The retained `|| echo 0` treats a failed account-index read as permission to continue. With malformed persisted `accounts.json` and no `--account`, `acq accounts --json` fails; the numeric check does not refuse, the driver starts a daemon, and refresh then fails reading that same index, leaving the daemon running. The daemon’s restore path logs the index error and continues. Smallest fix: abort preflight when the account query fails, validate its count, and pin that failure alongside the persisted-plus-logged-out case.

Execution checks, gate status, test counts, and live/browser behavior: **not verified**, because execution was prohibited.

Nitpicks: none; the finding requires an uncommon persisted-input fault but has an operational consequence.

Brief: the supplied reversal and job ruling made authorization clear. Plan: no separate plan was supplied or needed for this fix round. Prompt: no changes suggested.
