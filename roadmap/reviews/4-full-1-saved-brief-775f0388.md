[4-full-1-saved.md](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md) at `775f0388` is implementable against the tree, but not buildable as one consistent specification with its named design.

Fifth review’s finding: **answered**—`--id` leaves the query as the sole positional; Do 5 pins a query-bearing create and a query-omitting rename.

Owner’s rule: **carried through** the active brief and the design’s duplicate-name rule; no active courtesy refusal, duplicate re-aim or duplicate-only `no_undo` remains.

Findings, most severe first:

1. **Do 3/4; done-when 2/3 — The governing design still specifies the superseded write contract and command grammar.** The design says `set` takes the whole value ([design:183](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-design.md:183)), prints a positional-id grammar and rename ([design:198](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-design.md:198)), and assigns the future MCP a whole-value write ([design:205](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-design.md:205)). The brief instead requires one partial-field `Set` with revision-bound merging and named `--id` ([brief:173](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:173), [brief:261](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:261)). The design also retains `not_found` where the brief settles `saved_search_not_found` ([design:191](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-design.md:191)). These share one cause: earlier settlements were not propagated to the sibling specification. **Smallest change:** explicitly supersede those §3 sentences in the brief with Do 3/4’s contract, and make §3 point to that settled contract.

2. **Done-when 7 / retirement accounting — The four commit-message retirement lines do not establish executable holders under C154.** The ambiguity pin is still a future requirement, and “the rule” is prose, not a test, golden, check, type or dependency edge running at this commit ([AGENTS.md:119](/Users/tom/Development/GitHub/smallcatlabs/acquisition/AGENTS.md:119)). Moreover, ambiguity-on-open does not hold undo or write-acceptance behavior; the brief’s foot overstates that transfer ([brief:467](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:467)). **Smallest change:** describe these as dropped planned cases under the owner’s ruling, rather than executable test retirements; reserve C154 accounting for actual tests replaced during the build.

Not checked: builds, application runs, tests, gate execution or current CI conclusions, as requested; execution-dependent acceptance claims are **not verified**. Issue #42 was assessed from its quoted design text, without a live GitHub lookup.

Nitpicks: finding 2 is documentation/accounting only; no executable test was removed.

The step should **not start as drawn** until the conflicting design instructions are reconciled.
