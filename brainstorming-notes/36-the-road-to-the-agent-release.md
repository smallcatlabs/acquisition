# 36 — The road to the agent release (after the search slice)

Read with this (2026-10-19): it is cited by `CONTEXT.md`, not
`README.md`; the release shipped as `1.0.0-alpha.1`, not `beta.1`; the
work after it is `roadmap/README.md`.

Provenance: Fable (`claude-fable-5-1`), the session after the search
slice closed, 2026-10-11, with the owner. Read: `AGENTS.md`,
`README.md`, `CONTEXT.md`, every "Parked" section under `decisions/`,
`TESTING-NOTES.md`, the tail of `RUN-LEDGER.md`, `SEARCH-SLICE.md`
("Observations still open"), notes 07 and 08 (the August direction
notes, the precedent for this one), the first seat's brief and report
(`runs/seat-2026-09-26/`), `tools/docs-check.sh`, the Linux workflow,
and the master side's `docs/adr/0003-rewrite-vs-evolve.md` header, the
C++ app's update checker and its links. Measured: the suite (779 pass,
~170 s of test binaries run back to back with nothing compiled, 83 s of
it the six generated-search binaries), the source (77,861 lines over
eight crates; `daemon.rs` 9,728 with its test module from line 5,905),
the always-loaded documents (34 KB), the last live run (2026-09-12).
The draft's seven verdict slots were reviewed by a second Fable session
the same day (`runs/note-36-review-2026-10-11.md`, gitignored, deleted
once folded); the owner read the review and agreed with it whole, and
its rulings are folded in below, each claim re-verified against the
code by the drafting session before it was written here.

This note is the ordering of the next slices and nothing else: the
parks carry the scope, the registry the rulings, the slice records the
builds. It is deliberation, cited by `README.md` from the root-swap commit
on, and deleted when the last step below closes, cited afterwards as
"note 36 at `<commit>`". A step that opens a slice gets that slice's
own plan; this note does not grow one.

## The rulings this note rests on (owner, 2026-10-11)

Seven questions were put to the owner before this draft; each line is
the question in one clause, his answer in quotes, and where the answer
lands. The quotes are his words; the rest is the session's.

1. Transfer the whole repository, or a fresh one for the Rust build —
   "One repo for Rust and c++" in the morning; reversed the same evening
   with the fork link, the issue tracker and the update checker's tag
   parse on the table: two repositories, the C++ app renamed ("I like
   acquisition-qt for the legacy"), the Rust build pushed with its whole
   history ("i'm with you on clone") → step 1, the repo.
2. The shipped Rust app takes the C++ app's place and data, or installs
   beside it — "Rust sits beside c++ now and replaces later" → step 1,
   the identity; the replace ruling is a later event, not dated.
3. The shipping line, for the ADR and the parks that wait on "a
   shipping decision" — "The Rust build ships as Acquisition's agent
   surface by PoE2 1.0: the CLI, the MCP server and the daemon on
   Windows, macOS and Linux; GUI and other features follow." → step 0,
   the ADR in `acquisition-qt`, quoted here and never restated; the parks in
   `decisions/daemon.md` cite it when they fire at step 2.
4. The distribution channel: archives, `cargo install`, a tap, winget,
   installers — "Let's continue with github releases for simplicity
   and we can add other channels later." → step 2, the channel.
5. The trade-URL import's edges: inbound only or both directions, PoE1
   only or PoE2's trade data too — "inbound, poe1 only. poe2 at launch
   or earlier if possible" → step 3, and the PoE2 sitting in steps 5
   and 6.
6. The whole-CLI seat's mode, since refresh needs a daemon and price
   writes touch the one irreplaceable state (C35) — "i'm not sure why
   we can't run against my real store, but a mock session and store
   copy is fine." → step 1, the seat: the mock session for the job and
   write verbs, a store copy for the writes, the real store read-only
   for search as the first seat did.
7. The date the draft is built on — "PoE2 1.0 launches on December 11,
   2026." → the sequence's dates and the slack.

Two more of his, from the review's questions, are quoted where they
land: the update checker (step 1) and code signing (step 2).

The calendar is ruling 7's: the release is calendar-bound because it
is a shipping decision; what ships stays evidence-driven (P1), and a
step that has not earned its close by its own evidence slips rather
than ships.

## The sequence

Eight weeks and five days from this note to the launch. Each step
closes by a date; the slack is the two weeks before the launch, kept
empty on purpose. The draft's verdict slots V1–V7 are ruled (the
review, the owner's agreement, 2026-10-11) and stand in the step
sections as decisions.

| Step | Closes by | Fires (park, file) | Decided at the draft |
| --- | --- | --- | --- |
| 0. The ruling written down | 2026-10-18 | the "shipping decision" triggers now have a dated event to cite | — |
| 1. Rehome, with the reads and the seat beside it | 2026-10-18 | the toolchain pin, fired at `cfd3dc9c` (`decisions/daemon.md`) | V1 identity `acq`; V2 version `1.0.0-beta.1`, tag `v…`, pre-release; V3 pin `1.99.0` |
| 2. Three platforms, released from GitHub | 2026-11-01 | Windows; packaging; the credential store as an explicit choice; cargo-deny's licenses, bans and sources; the first-start keychain prompts (all `decisions/daemon.md`) | V4 a Linux VM row; V5 macOS Developer ID now, Windows unsigned, procurement in parallel |
| 3. Trade-URL import, inbound, PoE1 | 2026-11-15 | the trade translation (`decisions/trade.md`, split from search 2026-10-17); "what no sitting has asked the site" in part, by the sitting it needs | V6 the fix round inside the slice |
| 4. The release | 2026-11-22 | pricing on the MCP, fired or discarded by the seat's record (`decisions/frontends.md`) | V7 the policies' positions fixed, wording drafted here |
| 5. Slack | 2026-12-10 | nothing planned | — |
| 6. Launch-day first contact | 2026-12-11 | PoE2 currencies (`decisions/pricing.md`); per-realm merge at `policy set` (`decisions/plans.md`) on the second realm's first day of daily use; a PoE2 route under C59 is one row plus a first-contact sample | — |

## Step 0 — the ruling written down

Owner, in `acquisition-qt`: ADR 0003 rewritten with ruling 3 verbatim. The
playground follows in the rehome commit: `README.md`'s charter sentence
"Whether this code ships is the owner's decision (ADR 0003) and nothing
here anticipates it" becomes a citation of the ruling and of this note;
`AGENTS.md`'s "ADR 0003 … ignore it" line became, at the
two-repository ruling, "lives there, is the owner's call and needs
nothing from you". Nothing else in the always-loaded set changes.

## Step 1 — rehome

**The repo.** Two repositories (ruling 1), both under `smallcatlabs`.
The C++ app's repository is renamed `acquisition-qt` and transferred,
rename first so no interim name is shadowed; it keeps its fork link,
issues, PRs, discussions, the Sentry secret, the Coverity project and
the redirect chain the installed update checker reads (Qt's default
redirect policy follows GitHub's 301). Its references — the update
checker's releases URL, the help and issues links, the legacy-buyout
discussion link, the README, the user guide, the Coverity badge, the
CMake homepage and publisher email — ride the next C++ release; a
pointer to this repository in its README rides no release. The Rust
build is this repository: the spike branch pushed with its whole
history as `main`, the pre-swap tip tagged `playground-end`, one
root-swap commit (the workspace to the root, the C++ tree gone, the two
ground-truth documents carried), the licence declared before the push
(C110, `audits/relicense-2026-10/`). Done 2026-10-11; the old branch is
deleted from `acquisition-qt` once this repository's gate is green.
Facts about GGG are authored here now and `acquisition-qt` takes copies.

**The hazard two repositories dissolve.** The C++ app's update checker
(`src/util/updatechecker.cpp`) reads every release of its own
repository, parses the tag as semver, and notifies on a release or a
pre-release above the running version; its build workflows run on
`push: tags: ['v*']`. In one repository a Rust tag spelled `v1.0.0…`
would have nagged every installed C++ client and started three failing
C++ builds, and the draft carried a prefixed tag for it. In its own
repository the Rust build tags plainly.

- **V2 — version `1.0.0-beta.1` at this step; tag `v1.0.0-beta.1`; the
  GitHub pre-release flag**, so this repository's `/releases/latest`
  stays empty until a stable release. Not `1.0.0`, because the frontends
  park expects the search tools' JSON to move; not `0.x`, which would
  sort below the C++ line's `v0.18.6` and contradict what the user-agent
  already told GGG.
- **The user-agent's version segment** derives from
  `CARGO_PKG_VERSION` instead of the frozen `1.0.0-alpha.1`
  (`provider.rs`): the C++ app rolls its version every release under
  this registration (`networkmanager.cpp`), so a moving version is the
  continuity invariant 4 means; the shape `acquisition/<v> (contact:
  …)` and the contact are what stay. The owner's reading, agreed with
  the review.

**The identity (P3: irreversible, first-consumer treatment).** The
Rust app sits beside the C++ app (ruling 2), so its data directory must
differ from the C++ app's `acquisition` on every platform; the C++ app
sets no organization, so Qt's local-data location is `…/acquisition`.

- **V1 — the identity is `acq`, with no owner prefix**, and it is a
  set: `APPLICATION` in `world.rs` (the data directory, and
  `~/Library/Logs/<app>` on macOS), the real provider's keyring service
  (`acquisition-rust-spike` today in `provider.rs`; the mock's
  `acquisition-playground` is test-only and can follow), the codesign
  identifier `com.gerwaric.acqd` in `tools/sign-acqd.sh`, and the
  Windows arms to come. An empty organization yields a one-segment path
  on all three platforms in the `directories` crate. Why `acq`: it is
  what the user types and sees (`acq`, `acqd`, `acq-mcp`; the runtime
  directory is already `acq-<uid>`); `acquisition-agent` turns false
  when the GUI lands; `acquisition2` reads as PoE2. Other sites that
  carry the old strings and move with them: `tools/notes-check.py`'s
  hardcoded path, the wire fixtures' example paths
  (`acquisition-protocol/tests/wire.rs`), the daemon's mock-service
  tests; `search/item-facts/MANIFEST.md` is history and stays. The
  owner's migration: one directory move with the daemon stopped, the
  socket re-derived from the new root, one `acq auth` per account (two
  today) under the new service name.
  Done in the tree 2026-10-13, and the prefix amended the same day by
  the owner: "can we make smallcatlabs.com the org for acquisition?" —
  the `directories` qualifier `com` and organization `smallcatlabs`
  (macOS `com.smallcatlabs.acq`, Windows `smallcatlabs\acq`, Linux
  `acq` as before), the codesign identifier `com.smallcatlabs.acqd`.
  The session's choices where no ruling named one: the real keyring
  service `acq`, the mock's service and client id `acq-mock` (the two
  providers' entries stay separate), the macOS log directory
  `~/Library/Logs/acq`, the workflow `gate-linux.yml`. The owner
  migrated the same day: both directories moved, and three accounts
  logged in under the new service (GERWARIC, _vagabond and _exiled,
  the third new).

**What does not move.** The user-agent's shape and contact (invariant
4); the OAuth client id.

- **V3 — the toolchain pin, at this step; the park deleted.** Its
  trigger fired at `cfd3dc9c`: CI's clippy step failed on every push
  from 2026-10-01 to 2026-10-10 after floating stable moved to 1.99.0
  on 2026-09-28 while the local gate stayed on 1.94.1 — "the first
  clippy or build break on a toolchain bump", verbatim. The registry's
  rule applies, a fired trigger deletes its entry in the build's
  commit. `rust-toolchain.toml` with channel `1.99.0` and the clippy
  and rustfmt components; the Linux workflow installs the file's
  toolchain in place of `@stable`; step 2's release workflow reads the
  same file; a bump is a deliberate commit that runs the gate. No
  scheduled canary (P6): the canary's record is one lint caught and
  ten red days unseen.

**Beside it, in the same week, three reads and one seat**, each a
brief with a fixed input and a table back with a verdict slot per
row, the owner deciding deletions:

- The contradiction read: the always-loaded set, the area files, the
  closed records, against each other; narrative a closed record still
  carries as "next" (`TESTING-NOTES.md`, "Next direction", describes
  August's next step as the next step).
- The test read: one question per test, "what change to the product
  makes this fail, and is that change a boundary?" (`TESTING-NOTES.md`,
  "what else here reads as a check but cannot fail"); the speed lever
  named separately — the generated-search binaries at smaller case
  counts locally and full in CI is a knob, not a redesign.
- The parks read: every park with a proposed fate — stays, fires in a
  step above, deleted as withdrawn — in one table.
- The whole-CLI seat: mock login, refresh, price, render, search, end
  to end, in a mock session (the skill) with a copy of the owner's
  store (`examples/copy-as-store.rs`); search read-only against the
  real store as the first seat did (ruling 6). The second run of the
  seat procedure, so under P6 the seat becomes a skill at this close.
  Its record is the release's README's input: what an agent needed
  that the tour did not say.

## Step 2 — three platforms, released from GitHub

**Windows** (fires the Windows park): named pipes in the client and the
daemon, `acqd.exe` beside `acq.exe` in the sibling lookup, the world's
Windows arms (the runtime directory, the lock, the data and log
directories), the keyring's Windows arm, and a Windows job in the
Linux workflow running the gate on the mock harness, modelled on
the C++ Windows workflow. The live half runs on a Windows terminal of
the owner's, under C88 as any first contact: a first-contact row at a
ceiling of two, then the tracer rung, in `RUN-LEDGER.md` as every run
is. That row also verifies the signing assumption below: whether a
CLI launched from a terminal, or `acq-mcp.exe` spawned by an MCP host,
meets SmartScreen at all.

**macOS**: builds and runs live today; the debug daemon is signed
under an `Apple Development` identity (`tools/sign-acqd.sh`;
`RUN-LEDGER.md`, 2026-09-12). The release half is the packaging park:
release binaries signed under a **Developer ID** from the owner's
Apple account, notarized, and C82's acceptance criterion as a smoke
test — the CLI and the MCP resolving one `acqd` from the archive's
layout. The tracer rung on current HEAD before the tag: the last live
run is 2026-09-12 and the search slice landed since.

**Linux**: the gate already runs in CI with a real Secret Service. A
release archive is the remaining work.

- **V4 — one first-contact row from a Linux VM;** the release-note
  caveat ("Linux is tested on CI's mock harness only") only if no VM
  exists by this step. The row proves what CI cannot: real TLS to GGG
  from a Linux build, the `/run/user/<uid>` runtime directory and the
  real-mode lock under a real login session, the keyring under a real
  desktop rather than CI's unlocked throwaway. Mechanics: `acq auth
  --no-browser` prints the URL and the callback is a loopback listener
  on the daemon's host, so a desktop VM needs nothing more; a headless
  box needs an ssh port forward after the URL prints and
  `ACQ_NO_KEYRING=1` for the row. The limiter is platform-independent;
  the row proves nothing there.

**The channel** (ruling 4): GitHub releases, one archive per platform
built by a release workflow from a `v` tag; other channels later,
each a packaging target of its own when asked. The parks that fire with
the shipping decision close here: the credential store as an explicit
choice (keyring-core with one store crate per platform, or the
feature-selected backends kept, decided and written at `auth.rs`);
cargo-deny's licenses, bans and sources joining advisories in the
workflow — the manifests declare `MIT OR Apache-2.0` and the
repository since the licence commit; the first-start keychain prompts, since a signed release on a
second machine is that park's trigger.

- **V5 — the first Windows archive ships unsigned, with release-note
  text; procurement starts at this step, in parallel.** Nothing in the
  C++ workflows signs: the Windows one checks only Microsoft's
  redistributable's signature, the installer is unsigned, and the
  macOS one neither signs nor notarizes. The owner, verbatim: "I have
  an apple developer account and google play account via Small Cat
  Labs LLC. If there's an equivalent for microsoft that allows us to
  sign things, we can do that. Or we can postpone signing until a
  future version as you suggest." Consequences: the Apple account makes
  the Developer ID above available now, no procurement; Microsoft's
  equivalent is Azure Trusted Signing, whose onboarding (its
  quickstart, read 2026-10-11) is limited to legal entities with three
  or more years of verifiable tax history and a non-consumer email
  domain — whether the LLC meets that bar is the owner's fact; if yes,
  start the validation here and sign the release workflow's output
  when it lands; if not, SignPath Foundation or an OV certificate on a
  hardware token, or postpone. Nothing of it is on the critical path
  to December 11. Unsigned is defensible for this release because the
  C++ app has shipped unsigned to the same audience for years, and the
  SmartScreen question is measured at the Windows row above.
  Narrowed by the owner, 2026-10-14, verbatim: "I want to table the
  other distribution methods and stick to direct downloads from github
  so keep that stuff from slowing down the critical path." The
  session's reading, unopposed: tabled are
  Windows signing and its procurement, macOS notarization, installers
  and every channel beside the archives. Kept: Developer ID signing of
  the macOS binaries in the release workflow, without notarization,
  because a stable code identity is what keeps the keychain from
  prompting per account on every update (the keychain-prompt park); an
  archive fetched with `curl` carries no quarantine, so the README's
  install line is a `curl … | tar -x`. PoE1's 3.30 league launches
  2026-11-13, between steps 3 and 4; the owner, verbatim: "I don't
  think that changes anything we are doing here".

## Step 3 — trade-URL import, inbound, PoE1

Fires the trade-translation park: C99 rules the boundary as a
translation with a report, and this step builds the inbound half — a
site URL to a query, each clause exact, ambiguous, unsupported or
listing-only, the remainder never run as equivalent. Day one verifies
S56 (a URL decodes with no network) against the saved captures in
`search/trade-query/raw/searches/`, because everything offline rests on
it. A sitting (the site-sitting skill) for the clauses the translation
meets that no sitting has asked, which takes rows off the "what no
sitting has asked the site" park. PoE2's trade data (ruling 5) at the
launch or when the PoE2 site publishes it, as its own sitting; the
realm the URL names is the scope (C96), never a term.

A seat after the build, the third of the search's, and a fix round
held by `tests/seat_faults.rs` as the first was.

- **V6 — the fix round is a step of the slice, closed before the
  release.** Both rounds so far closed the day they opened (9b, twelve
  commits on 2026-09-26; 11c's brief, run, round and rerun all on
  2026-10-10); the slack is for the first users, a different
  population; and this note's own rule settles it — a step that has not
  earned its close slips rather than ships. What keeps the round off
  the critical path is 11c's filing rule: fix faults of the shapes in
  the record's findings table, file anything new as a park with a
  trigger. The alternative is a release README that documents a known
  fault.

## Step 4 — the release

The policies, the user-facing README (install, `acq auth`, an MCP
host's configuration), the note to GGG under the registration's
framing before anything public, the tag, the reddit post (owner's).
The seat's record fires or discards "pricing on the MCP". The C++
issues the Rust design answers close with a note.

- **V7 — the policies' positions are fixed now; the wording is drafted
  here, in the owner's voice; the files land at this repository's
  root.** GitHub reads `SECURITY.md` and `CONTRIBUTING.md` from the
  default branch, `main` here. Private vulnerability reporting is
  enabled on `acquisition-qt` and is enabled here at step 4;
  `SECURITY.md` points at it. The
  positions: supported versions are the latest release only (trains,
  not backports); the asset named is the keyring-held refresh tokens
  and the GGG relationship; `CONTRIBUTING.md` is a pointer to
  `AGENTS.md` and the gate, never a second authority; the AI-assisted
  PR rule is the `.githooks/commit-msg` stance generalised — the
  submitter is the author and accountable, assistance disclosed as an
  `Assisted-by:` trailer, the gate decides.

## Step 5 — slack, and step 6 — the launch

Two weeks with nothing planned: the fix round from the first users, a
second channel if one is asked for, the PoE2 trade-data sitting if the
site is up. Launch day is a first-contact session by the live-run
skill: the PoE2 stash routes as C59 rows with samples, and the two
parks the second realm's daily use fires.

## Not in this note

- The GUI: the slice after the release, under C16 as it stands;
  whether Tauri is revisited is that slice's first question.
- An external tracker (GitHub issues as the home of work, or a
  git-native agent tracker): a park in `CONTEXT.md`. The owner split
  the question, 2026-10-12. The relation of an issue to a park is stated
  before the first issue arrives, a sentence in step 4's
  `CONTRIBUTING.md`: the parks are the graph, an issue is evidence that
  may fire a trigger, a bug is a finding. Whether the graph moves is
  the park, and its trigger is the first issue a park cannot absorb,
  never the release.
- The daemon harness extraction: `TESTING-NOTES.md` says the day a
  fresh build of the daemon starts, not before; nothing above is that.
