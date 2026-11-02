`roadmap/briefs/transport-seam-2.md` at `aba6a164`, unchanged in the working tree, is buildable as drawn, with two documentation nits.

The second review’s finding: **answered**. Do 5 covers both no-listener sends through the refusing transport and requires every non-kept spec choke point to use an in-process transport. It also explicitly preserves production construction in the six kept tests, including the three currently using `parked_choke`. No further unassigned provider socket path emerged from static tracing.

Findings, most severe first:

1. **Do 2; done-when 2 — the socket inventory understates the explicitly retained exceptions.** Concrete reading: `roadmap/briefs/transport-seam-2.md:36` omits the mock’s TCP test, although `:109` and `:240` retain it; `crates/acquisition-daemon/src/mockggg.rs:1164–1168` starts its listener and sends through a direct reqwest client. The completion grep also finds the retained production callback bind at `crates/acquisition-daemon/src/daemon/login.rs:23`, absent from its explicit list. **Smallest change:** add the mock’s TCP test to the goal’s exceptions and the production callback bind to done-when 2; scope “only sockets” to provider/callback TCP, since IPC sockets also remain.

2. **Do 5’s Read inventory — “`parked_choke()` is called 14 times” counts its definition as a call.** Concrete reading: `roadmap/briefs/transport-seam-2.md:140`; `crates/acquisition-daemon/src/ratelimit/tests/spec.rs:28` is the definition, followed by thirteen call sites. **Smallest change:** say “fourteen occurrences, including the definition; thirteen calls.”

What I did not check: compilation, gates, tests, counters, census, mutants, breakers, or current CI, because execution was prohibited. Static reading found no commit-order blocker; actual intermediate compilation and scheduling behavior remain execution evidence for the worker.

Nitpicks: both findings. The mock exception is already intentional and supported by the brief; the count does not change the migration.

**The step may start as drawn.**
