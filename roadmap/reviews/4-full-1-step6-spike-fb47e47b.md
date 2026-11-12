`fb47e47b` — Decision 4’s record and patch landed; the brief’s full done-when is not yet met, and the rig needs fixes before judged runs.

- **P2 — `audits/web-2026-11/spike/rig-1.patch:57`, shape 2.** The measure filters rows by their declared index before checking their physical position, so it can miss a visibly misplaced row. Concrete local-fragment input: row 0 appears at visible slot 100 for one frame; its index falls outside the expected range, so it counts only as missing and can fit within the permitted 1% instead of voiding the run. **Smallest fix:** check every physically visible row’s position against its declared index before excluding indices outside the expected range.

- **P2 — `audits/web-2026-11/spike/rig-1.patch:949`.** The runner’s 180-second deadline cannot interrupt a blocking debugging-protocol call because its socket has no timeout. Concrete failure: Chrome remains connected but stops answering `Runtime.evaluate`; `receive()` blocks indefinitely and the runner never records its timeout. **Smallest fix:** propagate a monotonic deadline into socket operations and record expiration as a failed run.

- **P2 — `audits/web-2026-11/spike/README.md:30`, shape 6.** The reconstruction commands resolve the patch inside the newly created worktree, where it does not exist. Concrete reading: substitute a fresh directory and execute the displayed commands; `f3e663db` contains no `audits/web-2026-11/spike/rig-1.patch`. **Smallest fix:** capture the patch’s absolute path from the reviewing checkout and pass that path to both `git -C … apply` commands.

- **P2 — `audits/web-2026-11/spike/rig-1.patch:225`, shape 1.** The recorded timing output cannot support the README’s promise to report every window’s time and each server step. Concrete reading: a slow window caused by corpus checking produces only aggregate fetch, total-server and answer/listing statistics; its request, opening, corpus and rendering times are discarded. **Smallest fix:** retain each request’s URL, elapsed time and complete `serverTiming` entries alongside the aggregates.

- **P3 — `audits/web-2026-11/spike/rig-1.patch:623`.** The grouped page interpolates client-supplied league text into HTML without escaping it. Concrete client URL input: `group=tabs&league=%3Cb%3Ex%3C%2Fb%3E` renders markup in the explanatory paragraph. **Smallest fix:** escape the league in both constructions of `said`, preserving the deliberately constructed surrounding markup.

Execution-dependent checks—including patch application, measurements, input hashes and gate results—are **not verified**, as instructed.

I checked the saved patch and affected callers statically. Browser behavior and performance were not exercised; the later headless and owner runs remain intentionally pending. No goldens, references or retirement lines changed.

I would call the HTML-injection finding a nitpick for this isolated rig’s fixed Standard case; the measurement and reproduction findings affect its intended use.

The brief’s done-when spans several planned commits, so I distinguished this rig commit’s obligations from the later runs and close. Nothing else in the plan or prompt obstructed review.
