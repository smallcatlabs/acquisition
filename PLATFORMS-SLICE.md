# Three platforms — closed record

Step 2 of `brainstorming-notes/36-the-road-to-the-agent-release.md`:
the CLI, the MCP server and the daemon built, tested and released from
GitHub on Windows, macOS and Linux. Opened 2026-10-14 at `4f49e285`,
built as seven steps (0–6) in 26 commits, run live on all three
platforms on 2026-10-15 and closed the same day. This file is its
permanent short record in the mold of `DAEMON-SPLIT-SLICE.md`; the open
record's full text — the study behind ruling 1, every observation as it
stood — is the file at `eccd22c8`.

The scope was the note's step 2 with the owner's narrowing of
2026-10-14 beside V5, verbatim: "I want to table the other distribution
methods and stick to direct downloads from github so keep that stuff
from slowing down the critical path." Direct downloads from GitHub only;
Windows signing, notarization, installers and other channels tabled;
Developer ID signing of the macOS binaries kept for the keychain. The
rulings live in the registry (`decisions/daemon.md`, C112 new), the
mechanisms in the module docs and scripts named below, the properties
in the tests, the live runs in `RUN-LEDGER.md` (2026-10-15). Nothing
here is a second authority.

## Final state

- Tip at closure: the commit that cut this record. The release
  workflow's proof is run 38727404964 (dispatched on `68a93d66`,
  `sign=true`): five archives, one contract revision (`b380f843e6c1`),
  each smoked where it was built, the two macOS ones signed and smoked
  again. No tag: the tag is note 36's step 4 (ruling 3).
- The gate runs on Linux and on Windows (`gate-windows.yml`, the suite
  with the keyring round trip against Credential Manager); 808 tests on
  macOS at closure.
- Every live row passed, human-run from a terminal: macOS (the tracer
  driver on HEAD; the signed archive's first real start), Windows (first
  contact; the tracer by hand; SmartScreen), Linux (first contact).

## The rulings (2026-10-14, the owner's words verbatim)

1. **C83 on Windows** — ruled as C112 (owner: "yes"), after a study the
   owner asked for ("start the subagent"): a named pipe whose name
   derives from the user's SID and the world, created first-instance
   with a user-only DACL, a squatted name a refusal to start, the
   client verifying the server runs as this user before `hello`.
   AF_UNIX on Windows lost on async support and gained no privacy.
2. **The release matrix** — "i agree with your suggested builds to
   ship": macOS aarch64 and x86_64 (one Apple-silicon runner); Linux
   x86_64 and aarch64; Windows x86_64 only, its row run under the ARM64
   VM's emulation. A Windows aarch64 archive when asked.
3. **The tag** — "tag on step 4": note 36's step 4. Step 2 proved the
   workflow by manual dispatch.
4. **cargo-deny's licence allow-list** — "ok": the ten licences the
   graph carries, MPL-2.0 for `option-ext` and CDLA-Permissive-2.0 for
   `webpki-roots` by exception (`deny.toml` says why each is met, and
   what replaces either).
5. **Third-party notices** — "use cargo-about" (`about.toml`).
6. **Secrets** — "ok", with "I do every portal action and handle every
   credential myself. Never ask me for a secret's value.": the GitHub
   Environment `release`, the owner its required reviewer (set after
   the first signed run had gone through without one); secrets
   `APPLE_DEVELOPER_ID_P12_BASE64`, `APPLE_DEVELOPER_ID_P12_PASSWORD`,
   variable `APPLE_TEAM_ID`.

## Step ledger

| Step | Commits | What landed |
| --- | --- | --- |
| 0 Windows builds | `d3689894` | the transport behind one seam, named pipes on Windows (C112); `acqd.exe` as the sibling; the detached spawn; the world's Windows arm and `world_windows` (SID, DACL, server check) |
| 1 Windows CI | `d831fc86`, `1ec073de`, `36559593`, `56a2d38d` | the suite green on the VM (the first run failed 53 of 800); `gate-windows.yml`; `daemon stop` waits for the exit |
| 2 credential store | `82d32a55` | keyring 3's backends kept, decided at `auth.rs` ("keyring") |
| 3 cargo-deny | `9490dd7c`, `40270d98` | licences, bans and sources in `deny.toml`; rustls past two advisories the first run found |
| 4 release workflow | `002ed057`, `a0041308`, `9dbd05aa`, `f9c8019a` | `release.yml`, `tools/release-package.sh`, `tools/release-smoke.sh`, the notices; one contract revision everywhere; the macOS archives signed (Developer ID Application, team `FPWFCYMZ2F`, hardened runtime, timestamp) |
| 5 keychain prompts | `a3f76a99` | the first start under a new identity measured; on macOS the unresponsive report says to answer Keychain before it names the pid |
| 6 live rows | `a5d94118`, `cac223f4`, `678b574b`, `9012c0cc`, `4237b62c`, `eccd22c8` | the five rows of 2026-10-15 |

Beside them: `d153ce69` (the spawn breaks away from a host's job where
it may; `tools/windows-checks.ps1`), `68a93d66`, `0ed45690`, `30ec2da2`
(the second-user and elevation sittings).

## The parks it fired (`decisions/daemon.md`)

| Park | Fate |
| --- | --- |
| Windows: named pipes, `acqd.exe`, the runtime directory | built, deleted |
| Packaging: bundle layout, signed binaries, an installer, the CLI symlink | the archives and the macOS signature built; narrowed to what the narrowing tabled |
| The credential store as an explicit choice | decided; the alternative parked behind a sandboxed package |
| cargo-deny's licences, bans and sources | built, deleted |
| The first start under a new code identity waits at the keychain prompts | measured, the report names the prompt; deleted |

## Shapes of fault — the review checklist for platform work

No review round ran; the faults were found by the first run on each new
surface. Each shape, then what holds it.

- **A Unix mechanism assumed in product code.** The daemon inherited
  its caller's stdout pipe on Windows (the caller never saw EOF); a
  file's identity was its inode; reported paths kept `\\?\`; the
  annotations export fsynced through a read handle. Held by the suite on
  Windows in CI and
  `a_lazily_started_daemon_keeps_no_pipe_of_the_command_that_started_it`.
- **Bytes that differ by checkout.** CRLF in `Cargo.lock` and `*.sql`
  gave the Windows archive another contract revision (`.gitattributes`;
  `crates/acquisition-protocol/tests/line_endings.rs`).
- **A process tree the host owns.** A kill-on-close job ended a lazily
  spawned daemon; the spawn now asks to break away (`world_windows`).
  A host that forbids breakaway still ends it (parked).
- **A check that reads prose.** The tracer driver grepped `acq tabs`'s
  text for tab ids; the third layout change to abort a passing run
  (`76ef839b` summarizes a league over thirty tabs). It reads the JSON
  by id now; a script checks structured output, never the human form.
- **A rail that a hint undid.** A met ceiling was a flag that
  `reset-tripwire` cleared, one send out per reset, and both the daemon
  log and `daemon status` told the reader to reset. The ceiling holds
  on the count for the lifetime (`rails.rs`, `ceiling_met`;
  `the_ceiling_counts_every_method_and_halts_at_its_number`;
  `a_ceiling_halt_is_advised_as_the_lifetimes_and_a_trip_as_a_reset`).
- **A stop that returned early.** `daemon stop` returned while the
  daemon still exited (`a_stopped_daemon_is_gone_when_the_stop_returns`).

## What the runs taught

- **Windows users and elevation** (`tools/windows-checks.ps1`, mock
  only, run at the guest's desktop): another user's open of a serving
  pipe is denied; with the name squatted by another user the client
  refuses it (os error 5) and the daemon refuses to start, naming the
  pipe. One administrator's elevated and unelevated shells share one
  world and one pipe and reach each other's daemon (parked: whether
  elevation is a boundary).
- **ssh is not a desktop.** On Windows a daemon started over ssh ends
  with the session (sshd's job forbids breakaway), and key-auth ssh
  reaches neither Credential Manager nor an unlocked Linux keyring:
  keyring checks and live rows run at the guest's desktop. Another
  account's profile is unreadable to the ssh user, so evidence is
  copied out by the account that ran.
- **The keychain trusts identities, not files.** Each identity the
  items do not yet trust — a Developer ID release, an Apple Development
  build, an ad-hoc rebuild — costs one prompt per persisted account
  once, asking for the login keychain password; "Always Allow" adds it.
  A fresh install never meets it (its own `acqd` writes the items);
  items rewritten by a login under one identity are new to every other.
- **Gatekeeper and SmartScreen.** The signed, unnotarized binaries are
  rejected by `spctl` as "Unnotarized Developer ID", which applies to a
  quarantined copy; fetched with `gh` or `curl` they carry none and run.
  On Windows, a browser download unpacked with Explorer kept `ZoneId=3`
  and ran with no SmartScreen dialog ("Check apps and files" on) — one
  machine, one download (the packaging park's trigger names both).
- **Symlinks on Windows**: a standard user needs Developer Mode, and the
  link is then an untrusted mount point (os error 448), so the harnesses
  place a hard link and the smoke's symlinked `acq` is macOS and Linux
  only.
- **Building for Windows ARM64**: `ring` 0.17 needs clang (VS Build
  Tools' LLVM component on `PATH`).
- **The guests**: Windows 11 Pro ARM64 and Ubuntu 26.04.1 arm64 under
  VMware Fusion, each with a snapshot "provisioned, never logged in to
  GGG". A snapshot taken after a real `acq auth` is never reverted (it
  brings back a rotated refresh token). DHCP moves the guests'
  addresses; ssh is key-only from the NAT subnet.
- **The Microsoft Store** (asked 2026-10-14, tabled the same day): a
  company account is free; the Store re-signs an MSIX, not an MSI/EXE;
  a packaged app's `AppData` writes are redirected per package, so a
  Store install beside an archive install would see two worlds and two
  real-mode locks — C82's second installation and a hole in C31's
  per-user bound, to be ruled before any package; MSIX versions are four
  numeric parts; EV certificates no longer bypass SmartScreen.

## Owner verdicts, verbatim

- On the order of work: "yes, let's go ahead. use subagent builders
  where it makes sense, or let me know if you'd like to work in a fresh
  session, or just do the work yourself if that's close to optimal".
- Step 5: "i got three prompts at step 3, and then some 3 more prompts
  at step 6. i have no idea if sign-acqd has been run since the last
  build."
- macOS rows: "i ran them, but i had 3 keychain approvals and i think
  something timed out." (the driver's abort); "It went smoothly".
- Windows: "yes, i got zoneid=3. when i double-clicked on acq.exe the
  console window flashed but there was no smartscreen dialog." "Here is
  the output from steps 2 and 3, with no surprises".
- The close: "don't worry too much about dates. let's work logically in
  a way that will keep things simple and moving forward. go ahead and
  close and do whatever else is needed to wrap this up."

## Where the open observations went

| Observation | Went to |
| --- | --- |
| The Windows gap, second user, mixed elevation, keychain, SmartScreen, symlinks, `ring`, the machines, the Store | this record, above |
| An elevated daemon serves unelevated clients | `decisions/daemon.md`, "Parked" |
| A host whose job forbids breakaway ends the daemon | `decisions/daemon.md`, "Parked" |
| `subst` and mapped drives | `decisions/daemon.md`, "Parked" |
| The glibc floor | `decisions/daemon.md`, "Parked" |
| Gatekeeper on a browser download, SmartScreen on a release | the packaging park's trigger |
| Printed commands are POSIX-quoted | `decisions/frontends.md`, "Parked" |
| The cargo-about version shown for a two-target crate | `about.toml`, where it is generated |
| The `release` environment without its reviewer | dropped: set by the owner |
