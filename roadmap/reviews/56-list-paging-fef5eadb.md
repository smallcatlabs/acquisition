# Review of 56-list-paging at fef5eadb^..4587d782

Codex (`codex exec --sandbox read-only`, `roadmap/briefs/REVIEW-PROMPT.md`), its answer verbatim.

fef5eadb^..4587d782 — the brief’s done-when does not fully hold.

- **[P1] crates/acquisition-daemon/src/daemon/queue.rs:663:** The byte budget is applied after potentially gigabytes of params have been materialized, so a small list request can exhaust daemon memory. Concrete persisted input: 256 finished jobs with admitted params near 8 MiB each; `{"req":"list","state":"finished","limit":1}` reads all 256 rows—roughly 2 GiB of params—before cutting the page. The memory branch similarly clones `limit + 1` complete candidates before checking bytes. Apply one rule to both sources: select candidates by id and serialized size, then materialize only the bounded page plus bounded lookahead; pin that behavior with small injected budgets.

- **[P2] crates/acquisition-verbs/src/jobs.rs:326:** The default view concatenates two independently timed state reads without reconciling job identity, allowing one job to appear twice with conflicting states. Concrete input/interleaving: the open page returns running job 1; it completes before the finished request; that page returns job 1 again, producing two rows beside counts saying `total: 1`. Merge by job id, preferring the later observation, before computing omissions; pin a completion forced between these two requests.

- **Not checked:** Gate status, test counts and execution-dependent done-when lines are **not verified**, as requested; no builds, tests, daemons or live API calls were run. The worker report and external CI conclusions were not consulted.

- **Nitpicks:** Neither finding. Near-limit params are uncommon, but they are the brief’s stated reason for byte budgeting; completion between requests is ordinary concurrency.

- **Brief:** Add a working-memory bound alongside the serialized-page bound.
- **Plan:** No separate plan was supplied; the committed reader map was sufficient.
- **Prompt:** No change suggested.
