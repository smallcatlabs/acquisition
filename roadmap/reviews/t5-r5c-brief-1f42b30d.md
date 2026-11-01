`roadmap/briefs/t5-r5c.md` at `1f42b30d` has a buildable implementation plan, but contradictory ruling text prevents starting exactly as drawn.

1. **Narrower (sound):** C159 now matches the daemon-oracle assertion plus checked external copies; its entry form is sound and it is 714 bytes excluding newline, but Input retains the superseded wording.
2. **Answered:** `exit.rs:1695–1697` checks removal after a successful restore; the scenarios relinquish marker reads and retain protocol, journal/wire and permitted ledger observations.
3. **Answered:** directory-derived Windows pipe naming is feasible inside the runtime before daemon construction (`door.rs:475–477`, `endpoint.rs:56`, `world.rs:403`).
4. **Answered:** the direct `pick_runnable` assertion holds the component property; the already-Waiting job needs no replacement wait (`exit.rs:383–391`).
5. **Answered:** seven required scenario conversions, the admission-table folding and retained stress component are distinguished; the optional eighth conversion is explicit, and C42’s Pinned follows the rows.
6. **Answered:** attribute inspection includes declarations, the thread sleep is allowed, and `harness.rs:74` is included.
7. **Answered:** the plans rulings are correctly located and the three external copies correctly counted.

Findings, most severe first:

- **Do 6 / Input — The brief still presents the superseded, wider C159 as its fixed registry input.** Concrete reading: [t5-r5c.md:183](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/t5-r5c.md:183) says “every reader,” contradicting [network.md:25](/Users/tom/Development/GitHub/smallcatlabs/acquisition/decisions/network.md:25) and Do 6. **Smallest change:** replace the quoted draft with a reference to the current C159 entry.
- **Do 4 / Read — The inventory still equates the 30-second credential guard with the local runner kill.** Concrete reading: [t5-r5c.md:145](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/t5-r5c.md:145) conflicts with Do 4 and [nextest.toml:15](/Users/tom/Development/GitHub/smallcatlabs/acquisition/.config/nextest.toml:15), which terminates after four 30-second periods. **Smallest change:** state that the guard is 30 seconds and the local kill is 120 seconds, equal to `HANG`.

Not checked: builds, tests, gates, breakers, execution times or CI artifacts, because execution was prohibited. This was a review of the re-cut and its supporting citations, not a fresh exhaustive implementation audit.

Nitpicks: original findings 6 and 7, and the remaining timeout-inventory correction.

Round 5c may start after the contradictory C159 quotation is removed or corrected; not exactly as drawn.
