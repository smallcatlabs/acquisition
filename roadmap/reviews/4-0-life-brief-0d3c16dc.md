`roadmap/briefs/4-0-life.md` at `0d3c16dc` is **not yet buildable as drawn**: the admission fix introduces a blocking path in the signal watcher.

1. **Answered:** Do 1/3 establish the choke-visible admission fence before event handoff at every ingress; the new watcher defect is below.
2. **Answered:** saturating remainder, bounded rechecks and checked addition handle unrestricted knob values; the one-day cap changes wake frequency, not the idle exit threshold, because `Judge` still compares elapsed time against the full knob.
3. **Answered:** Do 4 now observes unit `Leave`, matching Do 1.
4. **Answered:** the dated design paragraph now assigns second-signal termination to the watcher’s independent backstop.

Findings, most severe first:

1. **Do 1 and Do 3 — Calling whole `begin_exit` on the watcher can block second-signal reception behind the rails-state or log mutex, contradicting the preserved independent escape.** `stop_admitting` calls both operations at [server.rs:564](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/server.rs:564); `begin_stopping` locks rails state at [rails.rs:551](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/rails.rs:551), which another sender holds while persisting state at [rails.rs:587](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/rails.rs:587); logging takes its mutex at [daemon.rs:1161](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon.rs:1161). The watcher cannot reach its second receive at [start.rs:230](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/start.rs:230) until these return. Its own runtime supplies the required runtime context; blocking is the problem. **Smallest sufficient change:** specify a lock-free stopping fence consulted by the choke point before ingress handoff, defer lock-taking/logging work off the watcher, and require a held-lock pin proving second-signal termination remains available.

What I did not check and why: no builds, tests, daemon runs, gate or fresh CI checks, as requested; unchanged builder and retirement details were not re-audited in full during this confirming pass. No new citation mismatch was found in the reviewed changes.

Nitpicks: none of the remaining finding; it affects termination behavior.

**Step 2 should not start as drawn until the admission fence also preserves the watcher’s independence from locks.**
