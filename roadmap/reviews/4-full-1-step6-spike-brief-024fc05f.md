`roadmap/briefs/4-full-1-step6-spike.md` at `024fc05f`: a builder needs the measurement and reproduction gaps below resolved before starting.

Findings, most severe first:

1. **Decisions 8–9; Done when — changes what gets built.** The bar permits a passing verdict while steady scrolling shows mostly blank space. Decision 9 explicitly records missing-row frames without judging them: smooth animation of empty blocks, followed by jumps populated within 100 ms, can satisfy every threshold and the done-when without demonstrating that the script holds the table while scrolling. **Smallest change:** name an acceptable missing-row-frame threshold before the first run and include it in the verdict.

2. **Decision 2; Done when — changes what gets built.** The saved artifacts need not reconstruct the prototype that produced the measurements. The brief requires two script files and the copy’s **Rust** patch, then deletes the copy; the existing renderer also consumes `web/assets/page.html` and `page.css` through `include_str!`, and edits to those files would escape that contract. The retry and virtualiser can also leave several measured implementations behind one final patch. **Smallest change:** preserve the complete applicable prototype diff, including new assets, for each implementation whose measurements support the verdict.

3. **Decisions 3–5 — changes what gets built.** The window URL cannot express the variable window size the script must request. Decision 3 leaves only a row number variable, while Decision 4 requests viewport-dependent `3v` rows; `Rows.limit` is serialized inside the request carried by `Request::token`, so changing the offset alone cannot change that limit. This is the first interface choice the builder must guess. **Smallest change:** explicitly allow offset and limit parameters, with Rust applying the count and `ITEM_ROWS` bounds.

4. **Decisions 7–9; Inputs — changes what gets built.** The measurement leaves workload choices open that can change both coverage and the verdict. Neither the viewport dimensions nor the twenty jump fractions are fixed, and “about 1,000” supplies no selection tolerance; in G1, fractions can land outside the opened tab’s item block. Recording the eventual commands does not establish these choices before measurement. **Smallest change:** fix the viewport and build profile, give an F1k selection criterion, and define jump targets that exercise the opened block’s beginning, middle, end and boundaries; record the selected requests and geometry with the results.

5. **Not in scope; Decision 6 — wording.** Excluding “the table’s renderer” leaves unclear which renderer the spike must nevertheless build. Decisions 5–6 require Rust-rendered rows containing name, base, price, location and query values, while step 6.4 owns the table renderer. **Smallest change:** explicitly permit the minimal fixed-height prototype renderer and reserve the production renderer, heads and sort links for step 6.4.

What I did not check and why: I did not execute Acquisition, builds, tests, gates, browsers or performance measurements, as instructed; I also did not query current CI. The listed read-only source lookups reached their intended targets, and the measurement file’s current SHA-256 matched the brief. I found no discovery-command coverage defect or contradiction with the named invariants and dependency rulings.

Form obstacle: “The same file” and “of the copy” require expanding several discovery commands before running them.

Prompt obstacle: None.
