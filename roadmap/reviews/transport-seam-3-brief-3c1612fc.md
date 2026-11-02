`roadmap/briefs/transport-seam-3.md` at `3c1612fc` is buildable as drawn; all four findings from the first review are answered.

Findings, most severe first:

1. **Do 1 — cancellation wording is ambiguous.** Input’s “as a cancel drops it” suggests that cancelling a running job drops its transport future, whereas [queue.rs:633](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/queue.rs:633) sets `cancel_requested`, and [choke.rs:74](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/ratelimit/choke.rs:74) limits cancellation to admission. **Smallest change:** replace the parenthetical with “the component test directly drops the polled future.”

2. **Do 2 — the inventory’s history citation names the wrong unit.** [The brief:256](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/transport-seam-3.md:256) says `b79e36f8` names six kept tests; that message actually lists six bind sites, including helpers. Six production-wiring tests do exist. **Smallest change:** cite `c27557ea`’s test inventory or list the six test names directly.

What I did not check and why: compilation, clippy, tests, gates, breakers, runtime cancellation/EOF behaviour or refreshed CI status; this was the requested static, read-only review.

Which findings I would call nitpicks: both; neither blocks implementation.

The step may start as drawn, within the owner’s named window.
