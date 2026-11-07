`roadmap/briefs/4-full-1-saved.md` at `d6fbc561`, unchanged in the working tree, is **not buildable as drawn**: the CLI’s positional grammar needs disambiguation.

Fourth review’s three findings:

1. **Answered:** replacement undo now passes the courtesy check, with the duplicate-name rename returning `no_undo` pinned.
2. **Answered:** re-aim is conditional on its own write passing that check; the duplicate-name edit and offer replay are pinned.
3. **Answered:** Do 5 requires every excluded input, removing the duplicated count.

Findings, most severe first:

1. **Do 4/5; done-when 3 — An optional id before an optional query leaves the create spelling without a defined parse.** The prescribed syntax is `set [<ID> --if-revision N] --name NAME ['<query>']`, while an id without a revision must also support `Current` ([brief:269](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:269)). With two optional clap positionals, `acq saved set --name rings 'class:rings'` assigns the query to the first positional, the id, leaving the create’s required text absent ([brief:177](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:177)). Quoting does not distinguish those positions, and revision presence cannot distinguish them while preserving `Current`. **Smallest change:** make the id a named option, such as `--id ID`, or give the query a named option; update the command spelling and require CLI pins for a query-bearing create and a query-omitting rename without `--if-revision`.

Not checked: builds, tests, gate execution, current CI conclusions or issue #42’s live body; this was the requested static repository review. Execution-dependent acceptance claims are **not verified**.

Nitpicks: none. The finding affects the ordinary create path.

The step should **not start as drawn**.
