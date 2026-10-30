`4af4113c^..68ae20de` — the brief’s done-when does not fully hold as written.

- **P2 — [tools/gate.sh:232](/Users/tom/Development/GitHub/smallcatlabs/acquisition/tools/gate.sh:232), shapes 1 and 6:** The Windows stage supplies a Bash script as the compiler executable even on native Windows, where `cc` invokes it through Rust’s `Command` without a shell. Concrete build-tool input: `tools/gate.sh --only winclippy` on Windows with a fresh target directory reaches bundled SQLite’s compiler invocation and cannot execute the `.sh` file. Use the native toolchain on Windows and reserve the stub for other hosts; pin that selection.
- **P3 — [tools/gate.sh:232](/Users/tom/Development/GitHub/smallcatlabs/acquisition/tools/gate.sh:232):** The stage hard-codes `CARGO_TARGET_DIR=target/winclippy`, contrary to the brief’s explicit “scratch … never `target/`” rule. Every ordinary invocation takes this path; the commit’s reinterpretation does not satisfy that wording. Use a stable directory under the prescribed scratch location, which can retain the warm cache.

Execution evidence—gate statuses, breaker results, routing runs and measured counts—**not verified**; no builds or tests were run, as requested.

I did not regenerate the extracts or independently compare every taxonomy row with the pinned clone and private stores. No runtime client-input, persisted-input or upstream-input handling changed.

Nitpicks: finding 2 is a process mismatch without a demonstrated behavioural failure; finding 1 is substantive.

Brief: the extract done-when says “every extract,” but the README still lists explicitly deleted historical outputs such as `hash-check.csv` and `mod-stat-index.csv`; exempt those historical entries or require their citations to be updated.

Plan: no additional impediment. Prompt: no change suggested.
