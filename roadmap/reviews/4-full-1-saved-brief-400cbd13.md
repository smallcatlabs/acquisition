[roadmap/briefs/4-full-1-saved.md](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md) at `400cbd13`, unchanged in the working tree, is **not buildable as drawn**.

The second review’s eight findings:

1. **Answered:** exact-revision reads and the interleaving pin are explicit.
2. **Answered:** five recovery ways exist; semantic dispatch and printed-command replay belong to their respective surfaces.
3. **Answered:** an unreadable-prior clear succeeds at the named revision and returns `no_undo`.
4. **Answered:** the refusal-shape table, field whitelist and missing-row kind are corrected.
5. **Answered:** `Current` is qualified as the human default; agent writes name their revision.
6. **Answered:** global account acceptance and the sibling help/reference sentences are covered.
7. **Answered:** minting is fallible, its failure structured, and collision retry bounded to two calls.
8. **Answered:** the tour checker is named precisely and the deadline ruling’s location corrected.

Findings, most severe first:

1. **Do 3/5 — The duplicate-name state can make a readable clear’s promised undo immediately refuse.** The courtesy check rejects any set whose name another live row holds ([brief:225](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:225)), while clear promises recreation except where its prior cannot be read ([brief:204](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:204)). Stage the prescribed two rows named `rings`, clear one, then replay its complete-value `Set/Create`: the surviving row causes `name_taken`, without any intervening write. The two-rows pin stops before this interaction ([brief:301](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:301)). **Smallest change:** extend clear’s `no_undo` rule to a prior the courtesy check prevents restoring, and add this case to the existing duplicate-name pin.

2. **Do 1/3/5 — The kind excludes a scope that the prescribed search realm resolution can return.** Do 1 permits only four realm words ([brief:138](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:138)); Do 3 promises resolution as `acq search` does ([brief:196](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:196)). Search accepts explicit `all` ([search/corpus.rs:135](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/corpus.rs:135)) and resolves an account store holding zero realms to `All` ([search/corpus.rs:637](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/corpus.rs:637)). Either path produces a scope the saved kind refuses. **Smallest change:** permit the four realms **and `all`**, with pins for explicit `all` and the zero-realm resolution.

3. **Do 1/3 — The newtype’s prescribed API omits the exact-read operation Do 3 requires.** The delegated methods exclude `at_revision` ([brief:132](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:132)), but the verbs must read through it ([brief:175](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:175)); the existing operation belongs to `Annotations` ([annotations.rs:1196](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-store/src/annotations.rs:1196)). **Smallest change:** add `User::at_revision` to the delegation list, or explicitly authorize an equivalent exact-revision read through `User`.

4. **Do 4/5 — The saved-input exclusion list omits `--trade-file`, another query source.** The brief lists conflicting sources without it ([brief:268](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:268)); the existing parser and `typed` list distinguish it from `--trade` ([search_cmd.rs:123](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/search_cmd.rs:123), [search_cmd.rs:341](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/search_cmd.rs:341)). **Smallest change:** name `--trade-file` in the exclusion rule and its CLI cases.

5. **Done-when 2 — “doors and is green” is an incomplete acceptance condition.** It names neither an artifact nor a check ([brief:336](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:336)). **Smallest change:** replace it with “the doors regex includes `User::open_for`.”

Not checked: builds, tests, gate execution or current CI conclusions, as requested. Execution-dependent done-when claims are **not verified**. GitHub was unreachable for issue #42; its closure rationale was read from the design.

Nitpicks: finding 5; finding 3 is a small API-completeness omission. The duplicate-name case is uncommon but explicitly admitted by the design.

The step should **not start as drawn**.
