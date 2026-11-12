# The check's driver: what drives the browser-driven check

Step 6.3 of the GUI's first slice, to
`roadmap/briefs/4-full-1-step6-driver.md`: what drives the
browser-driven check, and what it adds to the gate, asked of a page
`main` already serves.

## Verdict

**A hand-written driver works. It adds no compilation cost this Mac
resolves under the tolerance named before the timings, and its check
runs in about 1.4 s.** Candidate (b) is the test starting a headless
Chromium-family browser through the harness and speaking its debugging
protocol over a loopback WebSocket client written in the test crate:
141 code lines in `tests/browser.rs` and 52 in the CLI's harness, no
crate added. The tree's own checks admitted it unchanged. Its check
passed 20 of 20 runs on Chrome and 20 of 20 on Brave, and failed when
the count's script was broken so the count stopped following the
form. Its difference from the control in every timed stage is within
the tolerance named before the first run.

The crate candidates are refused by the architecture check:
`headless_chrome` reaches `ureq`, and `chromiumoxide` reaches
`reqwest`. A page that drives itself (c) would need `main` to serve a
test seam. Two decisions are drafted for the owner, the driver and the
check's home: #266 (the driver) and #267 (the home), both in the
owner's queue.

The machine: an Apple M4 Mac with 10 cores and 32 GB, macOS 27.2.1,
rustc 1.99.0, cargo-nextest 0.9.149, Google Chrome 157.0.8103.51,
Brave 157.1.99.55. The date is 2026-11-12. The copy was a detached
worktree of `main` at `8f81a4ac`; every number below is this Mac's.

## The check

`the_count_beside_the_button_follows_the_query_typed`, a module of the
CLI's one test binary (`tests/main.rs`). It starts the pages over the
seeded offline world as `web.rs` does (`offline_web`, made visible to
the crate), opens the form page in the browser and focuses the query
box. Then it types `class:rings` as key events (a key down carrying
each character's text, then a key up) and, after backspaces as keys,
`rarity=unique` in its place. After each, it reads the form as it
stands, exactly as `count.js` sends it (`new URLSearchParams(new
FormData(form))`), asks the `count` route for it itself, and waits
until the "Matches" output shows that route's `matched`: 3, then 1.
It asserts that the two totals differ. It waits under the runner's
hang ceiling alone (C137, no wall-clock bound of its own), polling
every 10 ms, and prints each wait's time.

Does it catch what it is for? With `count.js` changed so the count
stops following after its first answer, the check saw 3, never saw 1,
and the hang ceiling ended it at 120 s. With the script restored, it
passed.

## The candidates

**(c) No protocol, on paper.** A browser launched headless at a URL
can only load the page. To drive itself, the page would have to carry
a script that types into its own query box, waits for the count, and
reports what it saw, either by posting it somewhere or by writing it
where `--dump-dom` shows it. `main` would have to serve that script,
and it is the test seam into `acq web`'s scripts that
`decisions/frontends.md` declines in "Not doing". A script's own
dispatched events are also not a user's keys. The window spike's rig
did exactly this (`audits/web-2026-11/spike/`, rig-3: the measure
posts its result to a route of its copy), and it worked there because
the copy was never `main`. Not built.

**(a) A CDP client crate.** Of the maintained ones, `headless_chrome`
1.0.22 (MIT, synchronous, which fits a `#[test]`) was taken, with its
default features, which fetch no browser. It was added as a
dev-dependency of the CLI crate in the copy (`candidate-a.patch`: the
manifest's line and the lock, 473 crates against the control's 454).
The tree's checks, unchanged:

- `tools/arch-check.sh`: exit 1, on two lines.
  `EDGE acquisition-cli reaches ureq outside the transport crate`,
  by `acquisition-cli → headless_chrome → auto_generate_cdp → ureq`
  (the protocol's code generator, a build dependency, carries an HTTP
  client). And `EDGE the TLS stack is not the one the Rust daemon has
  presented`: ureq's rustls adds the `log` and `logging` features to
  the one rustls.
- `tools/gate.sh --only features`: exit 1 on all five release targets.
  A test build compiles `serde` and `serde_core` with other features
  than the build (C84, the artifact mismatch).
- `cargo deny check`: exit 0.

Measured no further. The other maintained crate, `chromiumoxide` 0.9.1
(MIT OR Apache-2.0, async), names `reqwest` as a direct dependency in
its default build (`cargo tree -i reqwest` with it added: `reqwest
v0.13.5 └── chromiumoxide v0.9.1`), which the same check refuses. It
was not taken.

**(b) Hand-written.** The browser is launched headless with a
debugging port, on a profile under the test's scratch, by
`harness::start_browser`. The browser is `ACQ_TEST_BROWSER`, else the
first installed of the usual macOS and Linux paths; a machine without
one fails the check, never skips. The protocol goes over `Cdp`, a
loopback WebSocket client in `tests/browser.rs`: masked text frames
out, frames read by their
length, events between a call and its answer passed over. No crate
was added (`candidate-b.patch`, the lock unchanged at 454 crates). Two
things the build taught:

- The first form spawned the browser from the test file. The
  architecture check refused it (`SPAWN … spawns browser(, which is
  not an allowed system tool`), since a process is started by a
  harness function under the one isolation list (C122). The launch
  moved into the harness, where that list applies to it as to every
  child. The check was not changed.
- The first run hung: the browser's page list (`GET /json/list`) was
  read to the stream's end, and the browser keeps the connection open
  whatever the request says. It is now read by its `Content-Length`.

Admitted: `tools/arch-check.sh`, `tools/gate.sh --only features` and
`cargo deny check` each exit 0, and `cargo clippy --workspace
--all-targets --all-features -- -D warnings` is clean on it.

**What the window spike's runner taught.** Its headless runner spoke
the same protocol by hand in Python (`rig-run.py` in
`audits/web-2026-11/spike/rig-3.patch`, 254 lines). The rig's three
cold reviews found three faults in it, all of one kind: a call that
could wait without end (no deadline on the socket; a per-read limit a
slow answer kept renewing; finding the browser's page with no
deadline). One more was found running it: a stale port file left on
the profile. The Rust check meets none of these the same way. It
keeps no deadline of its own, by C137 (no wall-clock bound a slow
machine can miss): each of its waits ends only at the runner's hang
ceiling (120 s here). Each test's profile is new under its own
scratch, so no port file is stale. The one fault it met, the page list
read to the stream's end, is the same kind of trap the Python runner's
discovery had: the browser holds its debugging HTTP connections open.

**Deadlines.** The candidate as measured carries none. A check that
hangs (a browser that stops answering, a socket read that never ends)
is ended by the runner's hang ceiling, and only there. The window
runner drew three review findings on exactly this. So the real check
must decide its deadlines from the start, as #266 notes: an absolute
deadline on each call to the browser and on finding its page, and
their expiry a failure that says which wait it was. That is a bound
C137 speaks to, so the step that builds the check squares the two. The
candidate was not changed after its runs.

## What it adds to the gate

Every stage of `tools/gate.sh` that compiles the CLI crate's tests,
each by the stage's own command (`run_stage`), timed apart:
- `build`: `cargo build --workspace`.
- `test`: its build, `cargo nextest run --workspace --all-targets
  --no-run`.
- `clippy`: `cargo clippy --workspace --all-targets --all-features --
  -D warnings`.
- `winclippy`: the stage's function, the same clippy for
  `x86_64-pc-windows-msvc` with the C compiler stubs, into
  `target/winclippy`.

No other stage compiles them. `features` reads `cargo tree`,
`artifacts` reads what the build wrote, and `doc` runs only on its
own, with no tests.

Clean means after `cargo clean`, twice each; incremental means after
touching `crates/acquisition-cli/tests/web.rs`, three times, the
median. The control is the copy untouched (candidate (b) reversed by
its own patch); then (b) the same way, in one sequence. Before each
sample the machine was waited on until its one-minute load was under 4
and no process but Spotlight's indexer used more than half a core.
Every sample started at a load of at most 3.99, after at most 191 s of
waiting. The owner did not sit during the timings: his sitting's
results file had no line.

The tolerance, named before the first run (Decision 6): a difference
is a cost only where it is over a tenth of the control's figure and
over the gap between the control's own samples; otherwise it is
unresolved.

| Stage | Kind | Control, samples (s) | (b), samples (s) | Difference of medians | Verdict |
| --- | --- | --- | --- | --- | --- |
| build | clean | 76.25, 77.64 | 76.58, 76.94 | −0.19 | unresolved |
| build | incremental | 0.13, 0.12, 0.12 | 0.14, 0.12, 0.12 | 0.00 | unresolved |
| test | clean | 147.33, 143.96 | 147.95, 147.00 | +1.83 | unresolved |
| test | incremental | 2.92, 2.72, 2.71 | 3.17, 2.71, 2.69 | −0.01 | unresolved |
| clippy | clean | 40.19, 40.46 | 40.39, 40.73 | +0.23 | unresolved |
| clippy | incremental | 0.56, 0.56, 0.56 | 0.57, 0.57, 0.56 | +0.01 | unresolved |
| winclippy | clean | 38.51, 38.04 | 39.32, 38.90 | +0.84 | unresolved |
| winclippy | incremental | 0.57, 0.56, 0.56 | 0.58, 0.57, 0.57 | +0.01 | unresolved |

Every difference is unresolved: candidate (b) adds no crate, and its
300 lines of test code are lost in the stages' own spread. A touch
with no edit lets rustc's incremental cache reuse nearly everything;
`-v` shows the CLI crate dirty and re-checked, in 0.5 s for clippy and
2.5 s for the test build. An edit to the check's own file would cost
more than these incremental figures. No total is projected from them.

What else (Decision 7), as this Mac's:
- Crates in `Cargo.lock` (`grep -c '^name = ' Cargo.lock`): control
  454, (b) 454, (a) 473.
- The check alone, 20 runs each, each run its own `cargo nextest run
  -p acquisition-cli --test main browser`, the time as nextest
  reports it:
  - Chrome 157: 20 of 20 passed, no failure, median 1.42 s (1.37 to
    1.58).
  - Brave 157: 20 of 20 passed, no failure, median 1.40 s (1.37 to
    1.79).
  - Each wait for a count took about 311 ms median on both (at most
    321): `count.js`'s 300 ms pause, then the route. Loading the form
    page took 25 ms on Chrome and 37 ms on Brave.
- The browsers here it drives: Chrome and Brave, through
  `ACQ_TEST_BROWSER` for Brave, which is installed under
  `~/Applications`. Edge and Chromium were not tried.
- What a machine must have: a Chromium-family browser; one without
  fails the check, never skips. The locator
  knows the macOS paths of Chrome and Brave and `/usr/bin/google-chrome`
  and `/usr/bin/chromium`; on Windows it needs `ACQ_TEST_BROWSER` or a
  Windows path added.

## The check's home: two shapes

**A gate stage.** The check runs in the gate's `test` stage, a module
of the one test binary, so every gate machine needs a Chromium-family
browser, and a missing one fails, never skips. This Mac has two. The
self-hosted Linux and Windows runners' software isn't known here.
GitHub's hosted images, read from their published lists (image
`20261101`), list Google Chrome 156 on Ubuntu 24.04 (with Chromium
156) and on Windows 2025, as read and not tried. Its cost here: about
1.4 s of test time, with no compile cost resolved above.

**A CI-only job.** The check runs as its own job on hosted runners
only, and the gate on the Mac and on the self-hosted runners skips it.
Its cost in CI is projected, never measured: one more job, a
toolchain and the CLI's test build on a hosted runner, the check's
1.4 s. A fault it catches is seen only after a push.

No workflow or gate was changed, in the copy or on `main`.

## What a check could take over from the review checklist

Two rows of `REVIEW-CHECKLIST.md` wait on this check:
- **"A script rule no test reaches."** The count's rule (the number
  shown is the route's for the form as it stands) is what this check
  holds. The picker's (a pick fills the first empty stat row, never a
  written one, and announces itself with a bubbling `input`) is a
  page's behaviour, and the same driver could hold it. What stays
  review: that a script names no other script, and keeps only a timer
  and its ask in flight. Those are properties of the code, and a
  browser sees only their effects.
- **"Page state the scan cannot see."** A browser check could hold a
  page's state across a reload: drive the page, reload, and assert
  that nothing it showed survives that the URL does not name. Through
  the protocol it could also read the page's storage, cookies and
  IndexedDB whatever spelling reached them. A cache held in a closure,
  or a generation a script keeps for itself inside one life, stays
  review: the browser sees only what it changes.

## Reproducing

From the repository's root, with `<dir>` a new directory:

```sh
patch="$PWD/audits/web-2026-11/driver/candidate-b.patch"
git worktree add --detach <dir> 8f81a4ac
git -C <dir> apply --check "$patch"   # exit 0; the same for candidate-a.patch
git -C <dir> apply "$patch"
cd <dir>
tools/arch-check.sh; tools/gate.sh --only features; cargo deny check
cargo nextest run -p acquisition-cli --test main --no-capture browser
ACQ_TEST_BROWSER="$HOME/Applications/Brave Browser.app/Contents/MacOS/Brave Browser" \
  cargo nextest run -p acquisition-cli --test main --no-capture browser
```

The tree that `candidate-b.patch` makes equals the copy the numbers
were taken in (`diff -r`, leaving out `target`, `.git` and the
gitignored `runs`).
