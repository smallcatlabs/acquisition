# Seat ledger

Every seat, one row per run, in order: the time series of the owner's
first goal, a surface an agent can use from its own words
(`roadmap/STATE-2026-10-26.md`, R3.1). The procedure is the seat
skill; its last step writes the row. The run's record (the brief, each
run's `calls.csv`, `REPORT.md` and `DEBRIEF.md`, and the README with
the fate table) is under `audits/<slice>/seat/` and cited by the row,
so the row is the committed summary of the run. The live runs have
their own ledger, `RUN-LEDGER.md`. Rows stay as written: a fate
decided later, or a rerun, is a new row, never an edit to an old one.

A row is: the date; the build (the commit and the sha256 of the binary
the door ran); the door (CLI or MCP), the model, and the world whose
pristine store a rerun reads; the calls, as `calls.csv` counts them;
the calls to the first answer the runner held right, per task, as its
report names them (a dash where it named none), beside the last seat's
calls per task on the same door; the stops (a task left unfinished, or
a door command the rails refused); and the faults by fate, as the
record's fate table holds them on the day the row is written.

Seats before this ledger, recorded under `audits/`: the whole CLI
(`audits/rehome-2026-10/seat/`), the trade seat and its re-sit
(`audits/trade-2026-10/`), and the release seat
(`audits/release-2026-10/seat/`), whose MCP runs took T0 to T7 in 3,
1, 6, 8, 12, 6, 6, 9 calls (mcp-opus, 51), 2, 1, 8, 8, 13, 7, 8, 8
(mcp-sonnet, 55) and 3, 2, 6, 9, 16, 6, 7, 7 (mcp-opus-2, 56).

| Date | Build | Door | Calls | Calls to first correct answer per task | Stops | Faults by fate |
| --- | --- | --- | --- | --- | --- | --- |
| 2026-10-27 | `799c39d9` (the crates as at `c2095319`); `acq-mcp` `eaf0a961…` | MCP, Opus (`audits/readiness-2026-10/seat/`); pristine `~/seat-worlds/readiness-2026-10/pristine/` | 51; `open` 31,472 bytes (42,082 at the release seat) | T0 –, T1 1, T2 1 (totals) and 5 (by league), T3 3, 5, 6, 7, T4 2 (state) and 10 (writes), T5 1, T6 1, T7 1; per task 2, 2, 7, 7, 16, 5, 6, 6 against mcp-opus-2's 3, 2, 6, 9, 16, 6, 7, 7 | none: all eight tasks finished, T4 with three invalid notes unnamed; one heredoc refused by the rails, sent as a draft | proposed (a draft): the fix round 3 rules (the MCP's words, the pricing precedence sentence, the instructions' job-tool sentence); issues 4 (#9, #10, #11, #18); kept 1 (C121); not faults 3 |
