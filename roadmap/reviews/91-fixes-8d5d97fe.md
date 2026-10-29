`9041d562^..8d5d97fe` — the fix-round done-when does not fully hold: target discovery can still silently omit release targets.

- **[P2] [tools/gate.sh:156](/Users/tom/Development/GitHub/smallcatlabs/acquisition/tools/gate.sh:156), shape 1:** The new reader checks `include` without establishing that it describes the complete release matrix. Configuration input adding `os: [windows-latest]` and `target: [aarch64-pc-windows-msvc]` beside the existing `include` creates a sixth release job, while this stage still checks only the original five targets and can pass. Reject matrix axes or other unsupported matrix structure before extracting targets, and add a breaker for this mixed matrix.

Not verified: builds, tests, gate statuses, and artifact equality; execution was prohibited and the worker’s report was unavailable. No live API traffic or owner-store access was performed.

Nitpicks: The finding is dormant under today’s matrix, but concerns the coverage guarantee this fix addresses.

Brief: No impediment.
Plan: No additional changes proposed.
Prompt: No changes proposed.
