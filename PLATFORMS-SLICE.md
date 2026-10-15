# Three platforms — open record

Step 2 of `brainstorming-notes/36-the-road-to-the-agent-release.md`:
the CLI, the MCP server and the daemon built, tested and released from
GitHub on Windows, macOS and Linux. Opened 2026-10-14 at `4f49e285`;
closes by 2026-11-01, or slips (note 36, "The sequence"). In the mold of
the earlier records: while the slice is open its findings and
observations land here, and at closure it is cut to the permanent short
record.

The scope is the note's step 2 with the owner's narrowing of
2026-10-14 beside V5 — direct downloads from GitHub only; Windows
signing, notarization, installers and other channels tabled; Developer
ID signing of the macOS binaries kept for the keychain. This file never
restates it. The parks it fires are in `decisions/daemon.md`
("Parked"); each is deleted in the commit that builds it.

## Before building — for the owner

Rulings first (`CONTEXT.md`, "Working style"). One boundary is open;
the choices were put with a recommendation and answered 2026-10-14.

1. **C83 on Windows — ruled 2026-10-14 as C112** (owner: "yes"; the
   endpoint split from C83 to keep each entry under the limit). C83 derives the socket "into a
   private per-user runtime directory"; a named pipe (C9) lives in one
   machine-wide namespace, so privacy cannot come from a directory. The
   session's first proposal: *on Windows the endpoint is a named pipe
   whose name derives from the user's SID and the world id; the daemon
   creates it as the first instance with an ACL admitting only this
   user and refuses to start if the name is taken; a client verifies
   the pipe server's process runs as this user before `hello`.* Not
   weighed in it: AF_UNIX sockets on Windows, which would keep C83's
   wording. The owner, verbatim: "should we have a subagent investigate
   this questions, or are you confident in your recommendation?", then
   "start the subagent" — a study of both, its recommendation brought
   here before step 0's transport is written. The study (2026-10-14):
   named pipes. tokio 1.53's `ServerOptions` has `first_pipe_instance`,
   `reject_remote_clients` (on by default) and an unsafe
   security-attributes hook (source read); a user-only DACL, the SID
   and the client's check of the server's process token type-check
   against windows-sys 0.61 (a probe, compiled for
   `x86_64-pc-windows-msvc`, not yet run). AF_UNIX has no async support
   on Windows — tokio#2201 open since 2020, three mio PRs closed
   unmerged, `uds_windows` blocking and its listener unusable with
   `async-io` — and gains no privacy (no peer credentials). A pipe
   vanishes with its server, so the stale-socket probe and
   `BoundSocket` are Unix-only. A squatted name becomes a refusal to
   start, not an impersonation. Proposed in place of C83's "the socket
   is derived from the root into a private per-user runtime directory",
   a property both arms meet: *the daemon's endpoint is derived from the
   world and the OS user, never chosen by hand; only that user can serve
   or reach it, and a client speaks to it only after verifying the
   server runs as that user.* The real-mode lock keeps its private
   per-user directory. To run on the VM: the same-user token check, a
   second user's squat and access refused, a daemon and a client at
   different elevations. Estimated 500–650 lines, windows-sys a direct
   `cfg(windows)` dependency of the store and the daemon or client.

Answered (the owner's words verbatim):

2. **The release matrix** — "i agree with your suggested builds to
   ship": macOS aarch64 and x86_64 (both built on one Apple-silicon
   runner); Linux x86_64 and aarch64 (the V4 VM is aarch64, so the row
   tests a shipped archive); Windows x86_64 only, its row run under the
   ARM64 VM's x64 emulation. A Windows aarch64 archive when asked.
3. **The tag** — "tag on step 4". Step 2 proves the release workflow by
   a manual dispatch whose archives are the run's artifacts (a browser
   download still carries Mark-of-the-Web, which the SmartScreen
   measurement needs).
4. **cargo-deny's licence allow-list** — "ok", to: allow the licences
   the graph carries (`cargo deny list`, 2026-10-14: Apache-2.0, MIT,
   ISC, Unicode-3.0, Unlicense, Apache-2.0 WITH LLVM-exception, BSD-2,
   BSD-3, BSL-1.0, Zlib), and two by exception for one crate each —
   MPL-2.0 for `option-ext` (under `directories`; file-level copyleft,
   met by shipping it unmodified with its notice) and
   CDLA-Permissive-2.0 for `webpki-roots` (the Mozilla root store
   reqwest's rustls carries). Should either matter later: `directories`
   can be replaced (`etcetera`, or the paths computed in `world.rs`),
   and the roots can come from the OS store (a reqwest feature, which
   changes what TLS trusts and is its own decision).
5. **Third-party notices** — "use cargo-about": the release workflow
   generates one notices file into every archive.
6. **Secrets for the release workflow** — "ok" (owner, 2026-10-14:
   "I do every portal action and handle every credential myself. Never
   ask me for a secret's value."). A GitHub Environment `release`, the
   owner as required reviewer, `v*` tags and dispatch only; secrets
   `APPLE_DEVELOPER_ID_P12_BASE64` and `APPLE_DEVELOPER_ID_P12_PASSWORD`,
   variable `APPLE_TEAM_ID`. The certificate is a Developer ID
   Application one, which in an Organization team only the Account
   Holder creates; new ones are issued under the G2 sub-CA (the original
   Developer ID CA expires 2027-02-01).

## Step ledger

One row per commit of the slice; a row is filled when it lands.

| Step | Commits | What lands |
| --- | --- | --- |
| 0 Windows builds | `d3689894` | the client's and daemon's transport behind one seam, named pipes on Windows (ruling 1); `BoundSocket` and the stale-socket probe Unix-only; the sibling as `acqd.exe` (`locator::ACQD`, C82); the spawn detached with no console window; the world's Windows arm (`%TEMP%\acq` for the real-mode lock, the holder's pid in `<lock>.pid`, the root without its `\\?\` prefix); `world_windows` (the SID, the user-only DACL, the server check) and `acquisition_client::endpoint` (connect, exists, a test listener); the tests' harnesses on both endpoints; README's gap says built, not run; the Windows park deleted |
| 1 Windows CI | `d831fc86`, `1ec073de`, `36559593` | the suite green on the VM (799 tests, Windows 11 ARM64): `acq` names itself `acq` in usage, the annotations export fsyncs through a write handle, a file's id is its volume serial and index (`world::file_id`, C84), every reported path is spelled plain (`world::canonical`), a spawned daemon inherits none of its caller's std handles; Unix mechanics gated or spelled per platform in the tests, the harnesses' sibling a hard link on Windows; `gate-windows.yml` beside the Linux workflow (build, the suite with the keyring round trip against Credential Manager, clippy); its first run found `daemon stop` returning while the daemon still exited, so the stop now waits for it; first green run 38665185349, 800 tests |
| 2 credential store | `82d32a55` | keyring 3's feature-selected backends kept, decided and written at `auth.rs` ("keyring"): the protected keychain is out of a bare CLI's reach and no sandboxed package ships; each backend held by a live run or a CI round trip; the park narrowed to the alternative, triggered by a sandboxed package |
| 3 cargo-deny | (this commit) | licences (question 4: the ten, MPL-2.0 for `option-ext` and CDLA-Permissive-2.0 for `webpki-roots` by exception), bans (wildcards; the workspace's crates `publish = false`, so their path dependencies pass) and sources (crates.io only) in `deny.toml`, the Linux workflow's step running all four; rustls 0.23.46 first (RUSTSEC-2026-0346, -0347, found by the same run); the park deleted |
| 4 release workflow | `002ed057`, `a0041308`, `9dbd05aa`, (this commit) | `release.yml`: the five archives (ruling 2), each `acq`, `acqd`, `acq-mcp`, both licences and the cargo-about notices (`about.toml`: over every target, `tools/notices-check.sh` holding the five targets' crates named); `tools/release-package.sh`; `tools/release-smoke.sh` per archive where it was built (extracted, the sibling, a symlinked `acq` on macOS and Linux, a mock login → status → stop); `sign-macos` in the `release` environment (Developer ID, hardened runtime, timestamp, the requirement checked to name the team); run 38727404964 signed both macOS archives (Developer ID Application, team `FPWFCYMZ2F`, identifiers `com.smallcatlabs.<bin>`, runtime flag, timestamp) and smoked them signed; a tag drafts the release with SHA256SUMS; the first run found the Windows archive's other contract revision (CRLF in `Cargo.lock` and `*.sql`, now LF everywhere) and its lost upload; run 38669893318 green, the five archives one revision (`b380f843e6c1`); the packaging park narrowed to what the narrowing tabled |
| 5 keychain prompts | (this commit) | the owner's first start of the signed release `acqd` (`RUN-LEDGER.md`, 2026-10-15): three prompts, one per persisted account, the start held 34 s and the command lost to C86's deadline; none after, for that identity. A one-time cost per identity the items do not yet trust, met by someone moving between builds, never by a fresh install (its own `acqd` creates the items). The park's second remedy built: on macOS the unresponsive report says to answer Keychain and run the command again before it names the pid to kill; the park deleted |
| 6 live rows | — | macOS: the tracer rung on HEAD, then the release archive's first start; Windows: a first-contact row at a ceiling of two, then the tracer rung, SmartScreen measured on a downloaded archive; Linux (V4): a first-contact row; one `RUN-LEDGER.md` row each, human-run from a terminal (C88) |

## The parks it fires (`decisions/daemon.md`)

| Park | Step | Fate in this slice |
| --- | --- | --- |
| Windows: named pipes, `acqd.exe`, the runtime directory | 0, 1 | built |
| Packaging: bundle layout, signed binaries, an installer, the CLI symlink | 4 | the archives and the macOS signature built; the park narrowed to the installer, Windows signing, notarization and other channels, behind the narrowing |
| The credential store as an explicit choice | 2 | decided (keyring 3 kept); the alternative stays parked behind a sandboxed package |
| cargo-deny's licences, bans and sources | 3 | built |
| The first start under a new code identity waits at the keychain prompts | 5 | met (three prompts, 34 s); the report names the prompt; deleted |

## Findings

One row per review round; the finding, then the property or test that
holds it now.

| Round | Commit | Findings | Held by |
| --- | --- | --- | --- |

## Observations still open

Data for the commit that touches each; none is a ruling.

- **The Windows gap, measured 2026-10-14 at `4f49e285`** with
  `cargo check --target x86_64-pc-windows-msvc`, the C compiler and
  archiver stubbed: the client and daemon failed on the Unix socket
  types; step 0 closed it at the type level. The first run on the VM
  (2026-10-14, `d3689894`) failed 53 of 800 tests; step 1's commit
  holds the fixes, and the suite passes there.
- **A second user, measured** (2026-10-15 on the VM, two standard users
  at the desktop, `tools/windows-checks.ps1`): the other user's open of
  a serving pipe is denied; with the name squatted by the other user,
  the client refuses it (os error 5) and the daemon refuses to start,
  naming the pipe (C112). The same-user check passes on every
  connection the suite makes, and the holder's pid is read from
  `<lock>.pid` by the lock tests.
- **Mixed elevation, measured** (2026-10-15 on the VM, one
  administrator, `windows-checks.ps1 elevation` each way round): both
  shells derive one world and one pipe, and each reaches the daemon the
  other started — status compatible, a request answered, `stop` from
  the other side stops it. The pipe's DACL names the user, not the
  token's integrity level, so an elevated daemon serves the same
  user's unelevated clients for its life (until stop or idle exit);
  expected otherwise before the sitting, and kept as found.
- **The study's check still open on Windows**: `canonicalize` on
  `subst` and mapped drives.
- **A host's job object** (measured 2026-10-15 on the VM,
  `windows-checks.ps1 job`; an ssh session is itself in one): a lazily
  spawned `acqd` under a kill-on-close job died with it, even where the
  job allowed breakaway, and outlived a silent-breakaway job (libuv's,
  under Node-based hosts). The spawn now asks to break away and falls
  back when the job forbids it: the breakaway-allowed job no longer
  takes the daemon. A host whose job forbids breakaway still ends the
  daemon when it exits — its jobs in flight with it, and the limiter's
  recent history, which a respawned daemon replaces by assuming the
  worst (slower, never over a limit; the idle verdict keeps a daemon
  alive for that history). Windows' sshd is one: a daemon started over
  ssh ended with the session, the breakaway asked for. Which other hosts
  do that is unmeasured.
- **Symlinks on Windows**: a standard user needs Developer Mode to make
  one, and one made so is an untrusted mount point to its readers
  (`canonicalize` fails, os error 448), so the test harnesses place a
  hard link instead. Step 4's "symlinked `acq`" smoke is a macOS and
  Linux check; a user who symlinks `acq` on Windows meets the same 448.
- **Printed commands are POSIX-quoted** (single quotes, `'\''` for an
  apostrophe): the tests run them through `sh`, Git for Windows' on the
  VM (`%LOCALAPPDATA%\Programs\Git\bin`) and on CI's runner. In
  PowerShell `'\''` does not read back; which shell a Windows agent runs
  printed commands in decides whether that matters.
- **The curl install carries no quarantine**, so an unnotarized,
  Developer ID–signed binary runs without Gatekeeper's check; an
  archive fetched by a browser is quarantined, and Gatekeeper is
  expected to refuse it (unmeasured) — the README's install line is the `curl`
  one (note 36). Notarization would staple only to `.app`, `.dmg` or
  `.pkg`; a bare Mach-O is checked online at first run.
- **The Linux archives' glibc floor is Ubuntu 22.04's (2.35)**: built on
  the oldest hosted runner, so a distribution older than that cannot run
  them. Musl static builds would lift the floor; nothing has asked.
- **Signed, not notarized** (run 38727404964): `spctl -a -t execute`
  rejects each binary as "Unnotarized Developer ID" — Gatekeeper's
  verdict, which applies to a quarantined copy; the archive fetched with
  `gh` carried `com.apple.provenance` only. Step 5 reads the signed
  `acqd`'s first keychain access. The `release` environment ran with no
  protection rule, though ruling 6 names the owner its required
  reviewer.
- **`ring` 0.17 needs clang** to build for `aarch64-pc-windows-msvc`
  (measured on the VM, VS Build Tools' LLVM component on `PATH`).
- **The machines for the rows** (provisioned 2026-10-14, each with a
  snapshot "provisioned, never logged in to GGG"): Ubuntu 26.04.1
  Desktop arm64 (GNOME on Wayland; `/run/user/1000` private; the login
  keyring unlocked; the keyring round trip passes against it; a mock
  login → status → stop resolved `acqd` as the sibling) and Windows 11
  Pro ARM64, standard user `acqtest`, VS 2022 Build Tools, toolchain
  1.99.0 for both MSVC targets. One guest on the GGG account at a
  time, the Mac's daemon stopped (per-account counters, one public
  address). A snapshot taken after a real `acq auth` is never reverted:
  it brings back a rotated refresh token. Builds are driven over ssh
  (key-only, the NAT subnet only, `acq-vm-setup/ssh-windows.ps1`);
  keyring checks and live rows run at the guest's desktop, since
  key-auth ssh reaches neither Credential Manager nor an unlocked
  keyring, and C88 refuses a real daemon without a terminal.
- **Tabled with the Microsoft Store** (asked 2026-10-14, verbatim: "how
  to get acquisition into the windows store so we can deliver a signed
  version of the app there (in addition to the traditional direct
  downloads from github) via  Small Cat Labs LLC"; tabled by the
  narrowing the same day). What was read, so a reopening starts here:
  company accounts are free (`storedeveloper.microsoft.com`, D-U-N-S or
  business documents, a work email on the company's domain); the Store
  re-signs an MSIX and not an MSI/EXE; a packaged app's new files under
  `AppData\Local` and `AppData\Roaming` are redirected per package, so
  a Store install and an archive install on one machine would see two
  worlds and two real-mode locks — C82's second installation and a hole
  in C31's per-user bound, to be ruled before any package; MSIX
  versions are four numeric parts, which `1.0.0-beta.1` does not map
  to; Artifact Signing is about $9.99 a month, and its quickstart
  (2026-05-21) does not state the three-year bar note 36 cites; EV
  certificates no longer bypass SmartScreen (since 2024).
