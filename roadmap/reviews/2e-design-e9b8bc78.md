08c0907a^..e9b8bc78 — done-when cannot be assessed: the design brief contains no acceptance lines; implementation is explicitly deferred.

Findings, most severe first:

1. **roadmap/briefs/2e-design.md:351 — P2, shape 8.** The proposed league-table read cannot establish membership in the latest fetched list: ingest only upserts rows (`crates/acquisition-store/src/lib.rs:1176–1185`). Fetch `[Standard, Rehearsal]`, then `[Standard]`, and Rehearsal remains present; a first fetch of `[]` also leaves no row distinguishing fetched-empty from never-fetched. Smallest fix: expose the latest successful leagues response’s membership and fetch time through one store read, and test both cases.

2. **roadmap/briefs/2e-design.md:218 — P2, shape 6.** The prescribed recovery orders regeneration before the pin update, but generators call `sources.require`, which refuses the newer checkout while `PINS` still names the old commit. With the clone at `ee2084de` and the pin at `e2bd511a`, following that order stops at regeneration. Smallest fix: say to update the pin and manifest before running generators, then commit those changes and regenerated outputs together.

3. **roadmap/briefs/2e-design.md:277 — P2, shape 1.** “Every tab fetched” conflicts with the proposed empty copy of the existing tree: folders are never fetched, and substash stubs reporting zero items are skipped by the planner. A zero-count `rh-m001` cannot satisfy that assertion without changing existing behaviour. Smallest fix: specify that eligible tabs are fetched, folders remain unfetched, and empty substashes are skipped—or absent from the returned tree—and pin those outcomes separately.

4. **roadmap/briefs/2e-design.md:325 — P3, shape 1.** The proposed README sentence says a new league’s first plan lists every owned tab, whereas the brief itself specifies one listing request at line 273, and policy coverage may select only some tabs. Smallest fix: describe discovery and fetching over successive plans, and instruct readers to inspect each plan’s work and ceiling.

What I did not check and why: I ran no builds, tests, generators, daemons, or gate stages, as requested. Reported timings, execution results, and rehearsal counts remain **not verified**. No executable or golden files changed in this range.

Nitpicks for unlikely cases: none; the findings concern ordinary pin updates and league snapshots or explicit rehearsal expectations.

Brief: add checkable acceptance lines for the design stage, distinct from future build acceptance.

Plan: update the roadmap’s “no brief yet” statement when recording this design.

Prompt: distinguish design acceptance from implementation done-when when requesting a design-only review.
