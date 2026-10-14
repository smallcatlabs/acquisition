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

1. **C83 on Windows (a ruling, open).** C83 derives the socket "into a
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
   here before step 0's transport is written.

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
| 0 Windows builds | — | the client's and daemon's transport behind one seam, named pipes on Windows (ruling 1); `BoundSocket` and the stale-socket probe Unix-only; the sibling as `acqd.exe` (`std::env::consts::EXE_SUFFIX`, C82); the spawn detached with no console window; the world's Windows arm (the runtime directory, the lock holder's pid readable, the canonical root's form); README's "Unix only" gap goes; the Windows park deleted |
| 1 Windows CI | — | a Windows job beside the Linux one, the gate on the mock harness, the keyring round trip against Credential Manager |
| 2 credential store | — | the choice decided and written at `auth.rs`; the park deleted |
| 3 cargo-deny | — | licences (question 4), bans and sources in `deny.toml` and the workflow; the park deleted |
| 4 release workflow | — | per-target archives of `acq`, `acqd`, `acq-mcp`, the licences and the notices; macOS binaries Developer ID–signed with the hardened runtime and a timestamp, not notarized; C82's smoke per archive (extracted, `acq version --json` names the sibling, a symlinked `acq` resolves the same `acqd`, a mock login → status → stop); the packaging park deleted |
| 5 keychain prompts | — | the park's remedy, or its record as a one-time cost, by what the macOS row shows |
| 6 live rows | — | macOS: the tracer rung on HEAD, then the release archive's first start; Windows: a first-contact row at a ceiling of two, then the tracer rung, SmartScreen measured on a downloaded archive; Linux (V4): a first-contact row; one `RUN-LEDGER.md` row each, human-run from a terminal (C88) |

## The parks it fires (`decisions/daemon.md`)

| Park | Step | Fate in this slice |
| --- | --- | --- |
| Windows: named pipes, `acqd.exe`, the runtime directory | 0, 1 | built |
| Packaging: bundle layout, signed binaries, an installer, the CLI symlink | 4 | built as archives; the installer tabled (the narrowing) — the park's remainder stays with that trigger |
| The credential store as an explicit choice | 2 | built; see the observation on the protected keychain |
| cargo-deny's licences, bans and sources | 3 | built |
| The first start under a new code identity waits at the keychain prompts | 5 | the owner's first Developer ID start is a new identity with three accounts: it meets the trigger |

## Findings

One row per review round; the finding, then the property or test that
holds it now.

| Round | Commit | Findings | Held by |
| --- | --- | --- | --- |

## Observations still open

Data for the commit that touches each; none is a ruling.

- **The Windows gap, measured 2026-10-14 at `4f49e285`**:
  `cargo check --workspace --all-targets --keep-going --target
  x86_64-pc-windows-msvc`, the C compiler and archiver stubbed (a check
  never links; this Mac has no Windows SDK, and the VM showed every C
  dependency compiles there). The protocol, store, plan and search
  crates check clean; the store warns once (`world.rs`, an unused `mut`
  off Unix — a `-D warnings` failure on Windows). `acquisition-client`
  fails at the `tokio::net::UnixStream` imports (2 errors; 3 with its
  tests), `acquisition-daemon` at `UnixListener`/`UnixStream` and
  `BoundSocket`'s `dev`/`ino` (8); the CLI and MCP are never reached.
  By reading, past the wall: `plan_cmd.rs`'s test binds a
  `UnixListener`; `client/tests/contract.rs` (14 Unix sites), the CLI's
  and MCP's harnesses (symlinks, `FileTypeExt`) and `cli/tests/world.rs`
  (`OsStrExt`, already `cfg`'d).
- **The lock holder on Windows.** `Lock::acquire` reads the holder's pid
  from the locked file; `File::try_lock` is `LockFileEx`, a mandatory
  lock, so the read fails and the refusal (and C86's report) loses its
  pid. To verify on the VM before step 0's fix.
- **The runtime directory off Unix.** `current_uid` is 0, so the
  fallback is `%TEMP%\acq-0`, and `private_dir`'s owner and mode checks
  are skipped; with ruling 1 only the real-mode lock lives there.
- **The canonical root on Windows** is a verbatim `\\?\C:\…` path; the
  world id hashes it and every report prints it. Its form is decided
  before the first Windows world exists, since the socket name and the
  log directory derive from the id.
- **The spawn** (`spawn_daemon`) sets no creation flags: on Windows
  `acqd` would share the caller's console and die with it, and an MCP
  host with no console would open a window for it. `client.rs`'s
  unresponsive report says `kill {pid}`.
- **Symlinks in the test harnesses** need Developer Mode or elevation on
  Windows: CI's runner is elevated, the VM's `acqtest` is a standard
  user.
- **The protected keychain is out of reach** (Apple DTS, read
  2026-10-14): it needs `keychain-access-groups`, a restricted
  entitlement only a provisioning profile authorizes, which a bare
  command-line tool cannot carry. keyring 3's `apple-native` (today's
  build) uses the file-based keychain and needs none. The credential
  park's keyring-core alternative names the protected keychain; for
  this release the file keychain is the only macOS store either way.
- **The curl install carries no quarantine**, so an unnotarized,
  Developer ID–signed binary runs without Gatekeeper's check; an
  archive fetched by a browser is quarantined, and Gatekeeper is
  expected to refuse it (unmeasured) — the README's install line is the `curl`
  one (note 36). Notarization would staple only to `.app`, `.dmg` or
  `.pkg`; a bare Mach-O is checked online at first run.
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
  it brings back a rotated refresh token. Builds may be driven over
  ssh; keyring checks and live rows run at the guest's desktop, since
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
