98dd155f — Decision 5’s record and patch are present, but this round needs fixes before judged runs; the brief’s full done-when remains incomplete.

Findings, most severe first. Patch locations refer to `audits/desktop-2026-11/spike/rig-1.patch`.

- **P2 — rig-1.patch:236, 298, 439 — Shape 2:** Acts can be logged as answered although their result never painted. For client clicks A then B, A’s sender returns before its paint callback, allowing B’s answer to replace A’s card before that frame; both callbacks still report success. An answered act must correspond to its own result actually drawn, with displaced results recorded as overtaken.

- **P2 — rig-1.patch:1185 — Shape 1:** The claimed read-only opener calls `Account::intent()`, which reaches the writable `Annotations::open_for`. For persisted input with a missing annotation file or missing UUID binding, this creates or stamps the file; it also takes the writing opener’s transaction and journal path for existing files. Both facts and intent must use read-only openers, as decision 1 and C124 require.

- **P2 — rig-1.patch:197 — Shape 1:** The sitting can lose its final acts because logging happens only on a one-second interval. A client click followed by closing the window before the next flush leaves no record, and the start script then kills the server; server write failures are also discarded at lines 1305–1307. The promise to retain every act needs acknowledged delivery through shutdown and visible persistence failures.

- **P2 — rig-1.patch:820, 973 — Shapes 1 and 2:** The late-frame judge measures the steady wheel together with subsequent jumps and pauses, despite the bar specifying the steady wheel alone. For persisted measurement input containing eight late intervals among 600 steady intervals, adding 350 smooth pause intervals changes a miss into a pass. Record and judge phase boundaries separately, with the display interval established outside the measured workload.

- **P2 — rig-1.patch:339, 1038:** The content policy blocks the inline styles used to indent the tree. Opening a folder from the persisted listing produces nested rows whose `style="padding-left:…"` attributes are refused by `style-src 'self'`, leaving the hierarchy visually flat. Apply indentation through permitted styling while preserving the policy.

- **P2 — rig-1.patch:419, 1641, 1958 — Shape 1:** Answer provenance is incomplete: server responses retain only `basis.snapshot`, and locally answered scrolls log `basis: null`. A client scroll within held rows therefore has no recorded basis, while responses from different intent revisions cannot be distinguished by their logged identity. Carry the complete search basis through responses, held rows and every answered act.

- **P2 — rig-1.patch:157, 271:** The page’s identity does not satisfy C16’s URL requirement. After client input selects a league, query and open places, reloading `/` loses them all, and Back cannot restore an earlier search; the server receives that state only through POST bodies. Preserve reconstructible URL state and navigation semantics, or obtain an explicit exception before relying on this rig.

- **P3 — audits/desktop-2026-11/spike/README.md:229 — Shape 1:** The C++ comparison describes 422.5 ms as refiltering, but the cited fixed source measures the whole `OnItemsRefreshed` operation; its `FilterItems` component is 35.2 ms. Name the measured operation precisely so the reference does not imply a filtering cost twelve times larger.

Not verified: patch application, builds, tests, gate status, timings and the owner’s sitting; execution was prohibited.

Nitpicks: the P3 comparison wording; the other findings affect ordinary interactions or the evidence used to judge them.

Brief: give this deliberately staged rig review its own done-when, separate from later runs and the sitting. Plan: no separate plan was supplied. Prompt: no change needed.
