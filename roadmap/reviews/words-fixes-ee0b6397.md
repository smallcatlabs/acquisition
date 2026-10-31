5e630995^..ee0b6397 — fix-round done-when does not fully hold: the original findings are addressed under C156, but command and refusal gaps remain.

- **P2 — [login.rs:765](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/login.rs:765), shape 6:** Logout recovery commands use the daemon’s startup shell rather than the requesting frontend’s shell. With persisted account `O'Neil#1234`, start the daemon under PowerShell, then request logout under Git Bash and encounter a credential-clear failure: the printed `'O''Neil#1234'` becomes `ONeil#1234` when pasted. Smallest fix: print the known account UUID as the retry selector, with explicitly labelled shell variants when unavailable, and pin a daemon/client shell switch.

- **P3 — [main.rs:1373](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/main.rs:1373), shapes 2 and 4:** The shell hint runs only after successful argument parsing, so common cmd.exe leftovers exit without the promised remedy. Client input produced by pasting `acq search 'rarity=rare is:corrupted'` into cmd.exe supplies two positional arguments; `refuse_usage` exits before `shell_hint` runs. Smallest fix: invoke the same hint on text-mode parser failures and pin that path through the binary.

- **P3 — [answer.rs:2855](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/answer.rs:2855), shape 1:** The new rationale incorrectly says PowerShell swallows `--` when invoking `acq`. For the reading `acq search -- '-is:corrupted'`, PowerShell passes `--` to the native executable, as [Microsoft documents](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_parsing?view=powershell-7.5#the-end-of-parameters-token). Smallest fix: remove that explanation and correct the commit’s claim in a follow-up; the whitespace spelling can remain.

- Builds, tests, gate status and Windows execution: **not verified**; this was a read-only review.
- Nitpicks: finding 3 is documentation-only. Finding 1 requires an uncommon combination, but breaks credential recovery, so I would retain it.
- Brief: update #16’s acceptance text to reflect the superseding shell ruling.
- Plan: no separate plan was supplied.
- Prompt: no changes suggested.
