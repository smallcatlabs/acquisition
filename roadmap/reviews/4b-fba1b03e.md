# Codex review: step 4b, the 4-lite build, `f29506bc^..fba1b03e`

Launched 2026-10-25 by the session that committed the build, at the
owner's word (`codex exec --sandbox read-only`, model gpt-6.1-sol,
`roadmap/briefs/REVIEW-PROMPT.md`). The findings verbatim; the owner
reads them before any fix is built.

---

`f29506bc^..fba1b03e`: the brief’s done-when does not hold; the owner’s sitting is explicitly pending, and execution-dependent criteria are **not verified**.

Findings, most severe first:

1. **[P1] [web/mod.rs:636](/Users/tom/Development/GitHub/smallcatlabs/acquisition-4b/crates/acquisition-cli/src/web/mod.rs:636), shape 7:** Selected refreshes resolve the account again through the daemon instead of using the account whose inventory the server displays. Start without `--account` while Alice is sole account, then log Alice out and Bob in: the page still displays Alice’s selections but submits them under Bob’s sole live session. Pass the server’s resolved username explicitly to both submission paths.

2. **[P2] [web/mod.rs:572](/Users/tom/Development/GitHub/smallcatlabs/acquisition-4b/crates/acquisition-cli/src/web/mod.rs:572):** Folder and substash checkboxes promise refreshes the ad-hoc job cannot perform. Tick the fixture’s `fold`, or a stored map substash: `refresh` searches only regular tabs from the stash listing, reports these IDs as unknown, and can finish successfully without fetching the selection; the jobs page never exposes that result payload. Disable unsupported selections with instructions to select the parent and use deep refresh, or submit substashes with their full parent coordinates.

3. **[P2] [assets/picker.js:32](/Users/tom/Development/GitHub/smallcatlabs/acquisition-4b/crates/acquisition-cli/src/web/assets/picker.js:32):** Picking a fifth stat silently replaces the first stat while retaining its previous bounds. Fill all four rows, then choose another picker result: the first condition changes without explicit replacement. Remove the fallback to `slots[0]` and explain that a row must first be cleared.

4. **[P2] [web/page.rs:111](/Users/tom/Development/GitHub/smallcatlabs/acquisition-4b/crates/acquisition-cli/src/web/page.rs:111):** Changing realm leaves the league selector populated from the previous realm. With PC Standard and Xbox Hardcore items, opening the default form and selecting Xbox still offers PC’s league choices and provides no way to load Xbox’s choices through the UI. Add a realm-selection submission that rerenders the form and its counts before selecting a league.

5. **[P2] [web/form.rs:331](/Users/tom/Development/GitHub/smallcatlabs/acquisition-4b/crates/acquisition-cli/src/web/form.rs:331):** An authenticated query can force an arbitrarily large allocation before search validation. For example, `query.stats[1000000000].type=and` expands an array to a billion elements; a maximum-sized index also makes `n + 1` overflow. Validate submitted paths against the form’s permitted names and bounded indices before constructing JSON.

6. **[P2] [web/form.rs:197](/Users/tom/Development/GitHub/smallcatlabs/acquisition-4b/crates/acquisition-cli/src/web/form.rs:197):** The counts page promised by the brief’s Goal is unreachable through the UI because every folded request selects rows. A user cannot request matching items counted by rarity or league; the internal league dropdown calculation does not provide that page. Add count-view controls feeding the existing search counts document and its renderer, with a wording golden.

What I did not check and why: no builds, tests, gate stages, cargo-deny, README-tour execution, browser interactions, or live traffic, as instructed. Their execution-dependent done-when claims remain **not verified**. The committed source contains the listener, equality, apply-loop, league, icon, stat-listing, and moved-world tests.

Nitpicks: none. Finding 5 requires a manually crafted authenticated request and is least likely during ordinary use, but its resource failure warrants correction.

Brief: make the counts page explicit in Do 6 and distinguish regular tabs, folders, and substashes in Do 7.

Plan: account resolution must remain consistent across reads and submissions throughout a long-lived frontend’s lifetime.

Prompt: explicitly distinguish assessing committed test coverage from verifying that those tests pass.
