`roadmap/briefs/t5-r1.md` at `61052e5a`, unchanged in the working file: **buildable as drawn.**

1. **Answered** — Do 3 includes both bearer-rejection publications and completes the inventory of runtime publications affecting `AuthStatus`. Notification outside `shared` is buildable throughout; the rename is covered after its caller’s hold, and restore notification follows the landed flag.
2. **Answered** — Done-when 4 names `DoorBuilder::run` and requires plain `#[test]` callers, matching Do 4’s runtime ownership.

Findings: none. The added citations match the tree; no contradiction introduced by this cut was found.

Not checked: compilation, tests, gates, timing, historical CI outcomes or an exhaustive re-review of unchanged sections. This was the requested read-only confirming review; no project code was executed.

Nitpicks: the prior done-when API mismatch was editorial and is resolved; none newly raised.

Round 1 may start as drawn: **yes.**
