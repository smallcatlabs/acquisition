# 4-full-1, step 6.4a — The browser-driven check, in the gate

## Goal
The gate loads `acq web`'s pages in a real browser and holds the two
script rules that until now were held by reading: the count beside a
form's button follows the form, and the stat picker fills the first
empty row. It is the first part of step 6.4 and comes before the
table, so the table's window script lands with a check to join and
the driver's first run on every gate machine is of a small thing.
Closes #267.

## Inputs
- C187 (the browser-driven check: a gate stage, a client the tests
  hold, no crate; a machine without a Chromium-family browser fails,
  never skips), ruled on #266 and #267.
- `audits/web-2026-11/driver/README.md`: what was measured.
  `audits/web-2026-11/driver/candidate-b.patch`: the candidate that
  passed 20 of 20 on Chrome and on Brave. It adds `tests/browser.rs`
  (the client `Cdp`, `launch`, and the test
  `the_count_beside_the_button_follows_the_query_typed`), the
  harness's `browser_exe`, `Browser` and `start_browser`, `mod
  browser;` in `tests/main.rs`, and makes `web.rs`'s `offline_web`
  visible to the crate.
- C137 (a deadline's contract is its behaviour at expiry; no test
  asserts a wall-clock bound a slow machine can miss) and
  `.config/nextest.toml`'s `slow-timeout`: the runner's hang ceiling.
- C122 (one test binary a crate, one harness, a process per test),
  C154 (a retirement names what now holds the property).
- `REVIEW-CHECKLIST.md`, the row "A script rule no test reaches".
  `assets/count.js`, `assets/picker.js`: the rules.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`. Both run on GitHub's
  hosted runners while the owner's are offline; their images list
  Google Chrome as installed, which is read, not yet run.

## Decisions
1. **Start from the patch, as it is.** Apply `candidate-b.patch` to
   `main` and build from there; it is the code that was measured.
   What changes below is what a gate stage needs that a spike did not.
2. **The runner's ceiling is the only deadline.** The check waits for
   what it expects and adds no bound of its own (C137). A browser
   that stops answering is a test the runner ends at its hang
   ceiling. Prove it once by hand: with `count.js` broken the test
   ends there, and the report says how long that took.
3. **A missing browser fails with the words to fix it.** The locator
   reads `ACQ_TEST_BROWSER` first, then the paths it knows. Add
   Windows': Chrome and Edge under `Program Files` and `Program Files
   (x86)`; read the paths from the hosted image's published list and
   a Windows machine's own, not from memory. With none found the
   test fails, naming the variable and the paths it tried.
4. **The browser is the test's own.** Headless, on a profile
   directory under the test's scratch, started through the harness's
   isolation as every child is, and killed when the test ends
   however it ends. Nothing reaches the owner's profile or opens a
   window.
5. **Two rules, two tests.** The count's, as the patch has it. The
   picker's, new: a pick fills the first empty stat row and leaves a
   written row as it was, and the count follows the pick. Each types
   and clicks as a user's keys and pointer would, and compares what
   the page shows with what the server answers the test for the same
   form.
6. **The checklist row retires** (C154): the commit that adds the
   tests removes the row and carries `retired <the row>; held by
   <the two tests>`. If a clause of the row is held by neither test,
   keep that clause in the row and say which.
7. **C187 loses `Unbuilt`** and its *Pinned:* names `browser.rs` and
   the two tests; `python3 -I tools/bothy.py check` passes.
8. **What a user of the gate is told.** `AGENTS.md`, "Once per
   clone", gains: a Chromium-family browser (Chrome, Brave, Edge or
   Chromium), which the gate's browser tests drive headless;
   `ACQ_TEST_BROWSER` names one the tests cannot find.
9. **No workflow change.** The first push's hosted runs are the
   proof that the images carry a browser the locator finds. If a
   hosted run is red because no browser is found or it will not
   start, stop and send the failing lines: what CI runs on is the
   owner's, and a flag that weakens the browser's sandbox is not the
   builder's to add.

## Find first
- `git apply --numstat audits/web-2026-11/driver/candidate-b.patch`
  and `git apply --check` of it on `main`: what it touches, and that
  it still applies.
- `git grep -n "fn isolate_at\|fn offline_web\|pub const SCRIPTS" -- crates/acquisition-cli/tests crates/acquisition-client/tests`:
  the isolation a child takes, the seeded pages, the scripts served.
- `git grep -n "data-count\|class=\"stat\|picker" -- crates/acquisition-cli/src/web/page.rs`:
  the controls the two tests drive.
- `git grep -n "browser-driven" -- REVIEW-CHECKLIST.md decisions`:
  the row that retires and the lines that name the check.
- `grep -n "slow-timeout" .config/nextest.toml`: the ceiling.
- `git grep -n "nextest\|ACQ_TEST" -- .github/workflows tools/gate.sh`:
  how the gate and CI run the tests, for the report; nothing there
  changes.

## Done when
- `crates/acquisition-cli/tests/browser.rs` holds the two tests of
  Decision 5 as modules of the crate's one test binary, and no
  `Cargo.toml` or `Cargo.lock` changes.
- Neither test names a duration it waits against.
- The locator fails, never skips, with the words of Decision 3 (a
  test of the locator's refusal, which needs no browser).
- The checklist row is gone or cut to what no test holds, and the
  commit carries its retirement line (Decision 6).
- C187 reads as Decision 7 says; `AGENTS.md` as Decision 8.
- `tools/gate.sh` exits 0 on the step's tree, and `--only doc`.
- The close names the hosted runs of Gate (Linux) and Gate (Windows)
  on the build's push, both green with the two tests run.

## Not in scope
- The window script's check: it joins with the table, the next part.
- A crate, a workflow change, a gate stage of its own: the tests run
  in the `test` stage.
- A browser on the owner's runner VMs: his, before they come back
  (#267's note).
- Firefox or Safari.

## Rules
`TEMPLATE.md`. Every `acq` and `acq web` runs in a mock world, as the
harness starts it. This round changes the shared test harness, so it
takes a cold review (C175).

## Report
`TEMPLATE.md`.
