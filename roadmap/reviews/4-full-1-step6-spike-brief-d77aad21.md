`roadmap/briefs/4-full-1-step6-spike.md` at `d77aad21`: a builder can start; the wording gaps below should be closed before recording the result.

- **Decision 4 / Done when — wording.** The saved patch need not explicitly identify its base commit, weakening reproduction after the worktree is removed. The brief requires the “whole diff from its base commit” and proof that it applies, but neither the record’s contents nor the done-when requires saving that commit’s SHA. **Smallest change:** require the record to name the base SHA and the command used to check applicability.

- **Decision 4 — wording.** “The page’s size not growing with the row count” conflates bounded rendered content with the scroll extent that must grow. The approved design, §3–§4, uses fixed-height rows and counts to establish the document’s scroll height; that height necessarily depends on row count. **Smallest change:** say “the live DOM’s size,” and require the record to define what it measures.

- **Inputs / Done when — wording.** Two requested counts lack their producing commands, contrary to C174, the brief’s rules-and-commands requirement. Inputs asserts “three reviews” beside a filename wildcard, while Done when asks for the script’s size in lines without naming a measurement command. **Smallest change:** add `git ls-files 'roadmap/reviews/4-full-1-step6-spike-brief-*.md'` for the review set and `wc -l <script-path>` for the final script size.

**What I did not check and why:** I did not build, execute Acquisition, launch browsers, run gates or measurements, or modify files, as instructed. I did not query external CI. I ran the discovery searches against the pinned tree; they reached their intended code. The available measurement file’s checksum matches. I found no direct contradiction with the named rulings, invariants or dependency boundaries, and no material missing scope exclusion.

**Form:** Incorporating every fault from earlier reviews requires separating their enduring properties from superseded recipe details.

**Prompt:** No obstacle.
