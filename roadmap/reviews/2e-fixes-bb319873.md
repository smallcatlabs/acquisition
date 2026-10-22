252e8c9e^..bb319873 — the brief’s done-when does not fully hold; execution-dependent criteria are not verified.

Findings, most severe first:

- **[P2] tools/tracer-rung.sh:470 — shape 2: the concurrency fix leaves the absent-policy case unprotected.** If `STORED` is `null`, another client can create a pc policy containing Standard before this write; omitting `--if-revision` selects `Expect::Current`, allowing the driver’s stale realm to overwrite it. Add an atomic create-only write option backed by `Expect::Create` for this branch, retaining exact-revision writes for existing policies.

- **[P2] crates/acquisition-cli/tests/harness/mod.rs:491 — the log normaliser preserves Windows separators, so the new transcript cannot match its golden there.** Input `daemon log: <scratch>\logs\f323bd59fa67\mock\daemon.log` becomes `<scratch>\logs\<world>\mock\daemon.log`, whereas the golden requires forward slashes; comparison is byte-for-byte. Canonicalise separators within the recognised log path and add Windows-path cases.

- **[P3] .claude/skills/live-run/SKILL.md:63 — shapes 1 and 3: the revised probe rule still overstates which routes receive HEAD.** A fresh daemon running `acq leagues` skips HEAD because `NO_PROBE_ROUTES` contains `league` and `profile`, contradicting both “probes each route” and the adjacent HEAD allowance. Qualify the rule to probe-enabled routes and describe the leagues ceiling using its GET and any required token refresh.

What I did not check and why: execution was prohibited, so self-tests, historical export counts, both trade-check configurations, runtime tests, ten-run stability, registry checks, per-commit gates and Windows execution are **not verified**. Commit-message reports were read, not independently confirmed.

Nitpicks: none. The first finding requires a narrow interleaving, but it can overwrite irreplaceable intent.

Brief: record the accepted normaliser exception and seeded-driver measurement substitution directly in the brief.

Plan: include the create-after-null interleaving and Windows path spellings in fix validation.

Prompt: no change needed.
