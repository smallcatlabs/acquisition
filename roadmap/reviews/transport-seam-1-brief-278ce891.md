[transport-seam-1.md](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/transport-seam-1.md) at `278ce891` matches the current file and is buildable as drawn.

1. **Answered.** A private child module with the production type and constructor restricted to `choke`, a private client field, and only neutral re-exports leaves no production access path outside the choke point. Directly in `choke.rs`, `#[path = "transport.rs"] mod transport;` resolves to `ratelimit/transport.rs`, inside `.cargo/mutants.toml:16`’s glob.
2. **Answered.** The HEAD reading is correct: authorization/send at `choke.rs:564`, probe outcome at `:618`, degradation through `:664`, uncounted journal entry at `:679`, and `ProbeError::Failed` with no status at `:720`. It never enters GET’s lost-send path.
3. **Answered.** The distinct prepared user-agent exercises production forwarding without the client default masking an omission; the in-process run separately checks preparation and bearer sensitivity.
4. **Answered.** Do 1 includes `policy.rs:590`; the twenty-before/eight-after counts agree with the source.
5. **Answered.** The brief now cites `auth.rs`’s credential discipline rather than attributing it to the unrelated ruling.

Findings: none. I found no new contradiction or blocking citation mismatch. The builder-option deferral is sound, and Do 2’s invalid-header case supplies an injection caller before Do 3; the commit order requires no later step.

What I did not check and why: compilation, runtime wire behavior, pacing, gate/breaker results, or current CI status; this was the requested read-only review. I ran no builds, tests, probes, daemons, or live requests.

Nitpicks: original findings 4 and 5; no new findings.

The step may start as drawn, in the owner’s designated window.
