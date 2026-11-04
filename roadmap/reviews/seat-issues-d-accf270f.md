`accf270f` (`a50ac14c..accf270f`): the brief’s done-when does not fully hold.

1. **[P2]** [daemon/queue.rs:1276](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/queue.rs:1276), **shape 4**: a substash refusal recommends refreshing the wrong listing. With persisted parent `M` naming `S1`, a client requesting newly available `S9` is refused; following the suggested `stashes` job preserves those substash rows, so the retry remains refused. Recommend fetching `M` itself to refresh its substashes, and pin that recovery sequence.

2. **[P2]** [daemon/kinds.rs:329](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/kinds.rs:329): `Path::exists()` treats filesystem inspection errors as absence and admits the job without checking its listing. For example, an existing account store behind a directory lacking search permission bypasses admission and permits a send instead of the promised unreadable-store refusal. Use `try_exists()` and refuse its error; pin that nothing is submitted or sent.

3. **[P3]** [daemon/queue.rs:1255](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/queue.rs:1255): the new refusals render client-supplied `league` whole instead of using the shared bounded renderer. A valid 200-byte league is echoed whole in the listing refusal and the unreadable-store refusal at `kinds.rs:334`; both share this cause. Apply `describe_str` to league values in both paths and pin the longer-than-preview case.

4. **[P3]** [tests/tool_refusals.rs:232](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-mcp/tests/tool_refusals.rs:232), **shape 10**: the MCP test duplicates wording already held by the CLI golden. Rewording “names no tab … that stands” and regenerating its golden still leaves this independent prose pin failing. Keep the MCP assertions about JSON-RPC refusal, kind, message presence and admission; leave exact wording with the golden.

Execution evidence—including both gates, failing-before runs, runtime sizes and test counts—was **not verified**, as requested; seat outputs were unavailable, so their evidence was judged against the brief’s quotations.

Findings 2 and 3 are the unlikely-in-practice cases I would call nitpicks: inaccessible store directories and unusually long league values.

The brief should distinguish recovery of the league listing from recovery of a parent’s substash listing. The registry should also reconcile C164’s exception with C28’s categorical “never reads the store.” Nothing else in the plan or prompt impeded review.
