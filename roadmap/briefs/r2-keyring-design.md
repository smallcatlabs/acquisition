# r2 — The credential I/O leaves the shared lock: the design brief

Readiness step 2 (`roadmap/README.md`, row r), after r1. A design
brief: its output is a build brief (`r2-build.md`), a drafted ruling
for the owner, and the named pins. The build follows the owner's
ruling. #81; #55 (C135) is decided here, with it or after it.

## Goal

A design under which no keyring or account-index call runs while the
daemon's `shared` lock is held, `hello` is answered under no lock that
credential I/O can hold, and every credential-store result still
reaches the caller and the words the user reads (C144, no success the
daemon did not earn). Why now: with C135 unbuilt, every lifetime
refreshes at its first token use; a macOS ad-hoc-signed build prompts
on keychain modify; the owner rebuilds often during the league, so the
first job after each rebuild meets the keychain dialog with every
client reporting the daemon unresponsive (C86's "answers hello in
milliseconds" false in that state), and jobs that hold a gate permit
blocked behind their token read.

## Read

- #81 (the sites and the schedule), #55 (C135, the owner's yes of
  2026-10-22), `roadmap/STATE-2026-10-26.md` section 2D and R1.2.
- `decisions/daemon.md`: C3, C6, C86, C133 (one exit, a bounded
  drain), C134 (a panic aborts; the hook takes no lock), C144, C135,
  C50 (a login whose profile fetch fails fails whole), C24 and C87
  (`decisions/network.md`: a dead grant, a dead bearer).
- `crates/acquisition-daemon/src/daemon.rs`: `Daemon`, the lock order
  stated in one place (search for "lock order"), `Sessions`,
  `AuthSession`, `Staging`, `CredentialStore`.
- `crates/acquisition-daemon/src/daemon/refresh.rs`: `finish_refresh`
  (around line 435), `install_tokens_locked` (71),
  `persist_credentials_locked` (126), `RefreshOwnerGuard`, the
  single-flight; `login.rs`: `complete_login` (233), `logout` (418, which
  also serves `auth forget`); `server.rs`: `handle_conn` (85 to 92, the lock taken before
  `hello` is read), `is_idle` (589).
- `crates/acquisition-store/src/index.rs`: `Index::load` and the
  atomic, synced write (#63's second part, landed in h1).
- The tests that pin what must not move: `daemon/tests/refresh.rs`,
  `login.rs`, `sessions.rs`, `bearer.rs` (the `rejected_access_token_*`
  family, C87), `exit.rs`; the harness's scripted credential store in
  `daemon/tests/harness.rs` (its default since `2a9d5aa4`).
- `roadmap/TIME-SEAM.md` section 4: the daemon lifecycle as a machine
  over an injected clock, 4-0's design. This step is a candidate for
  that seam and must not wait for it; say in the design where the two
  meet.
- `REVIEW-CHECKLIST.md`, "Network" ("An owner that leaves without
  publishing") and "Protocol, client, daemon lifecycle".

## Input

The three schedules the build's tests stage, each with a credential
store that blocks on `save` (and on `clear`) until the test releases
it, never a clock:

1. A refresh is saving; a new connection sends `hello`. Today it waits
   on `shared`; after, it is answered.
2. A refresh is saving; a job past admission holds a gate permit and
   reads its token. Today it blocks; after, it reads the token the
   refresh installed or waits only for the install, never for the
   save.
3. A login is completing (`complete_login`); `daemon status` is asked
   and a second login starts. Today both wait; after, the status
   answers and the second login is refused or queued in the words
   C133 and C50 already use.

And one property that must survive each: the save's failure reaches
the caller (the refresh's job fails naming it; logout and forget say
what the keyring did), as the `c144_` tests hold today.

## Do

1. **Trace what each path needs the lock for.** For refresh, login,
   logout and forget: separate the session-table write (install the
   tokens, mark the session) from the I/O (index load and write,
   keyring save or clear). Write the trace as a table in the design:
   path, what runs under `shared` today, which of it needs the lock.
2. **The shape.** Recommended: compute outside, publish inside. The
   refresh computes the new tokens, releases `shared`, performs the
   index write and the keyring save, then retakes `shared` to install
   and to mark the session saved or unsaved. The single-flight guard
   (`RefreshOwnerGuard`) still serialises refreshes per account and
   still publishes on drop. A reader that needs the token (the bearer
   read under a permit) waits on the install, which is now a lock-free
   wait on the refresh's publication, never on the save. A save that
   fails marks the session, and the job that needed it fails naming
   the mark (C144). Say what the exit's drain (C133) waits for when a
   save is in flight at the bound.
3. **`hello` ahead of the lock.** The connection count and
   `last_activity` move to atomics or to a lock no I/O path holds, or
   the bookkeeping moves after the `hello` exchange; `is_idle` reads
   them the same way. Say which and why.
4. **C135 (#55).** Decide whether the access token joins the keyring
   entry in this step's build or the next: it reduces how often the
   refresh runs but does not remove the schedule. Recommended: after,
   as its own round, so this step's review reads one change.
5. **The ruling.** Draft one new entry for the owner, under 800 bytes,
   in the entry rule's form: no credential-store or index I/O runs
   under the daemon's shared lock, and `hello` is answered under no
   lock such I/O can hold; its *Why:* the schedule above; its
   *Pinned:* the three tests by name. If an amendment to C86 or C144
   serves better than a new entry, say so. The id is the next free one;
   do not write it until the owner rules.
6. **The build brief**, `r2-build.md`, from `TEMPLATE.md`: the three
   tests named for their schedules, each forced with the blocking
   store (C137: no wall-clock bound), the `c144_` and `rejected_access_token_*`
   families named as the tests that must stay green, the lock-order
   comment updated, and a measurement: the handshake's latency under a
   blocked save before and after, on a mock session, stated as a
   number in the build's message.

## Done when

- The design is one document the owner can read in ten minutes: the
  trace table, the shape, the `hello` change, the C135 decision, the
  drafted ruling measured under 800 bytes, the three tests named with
  their expected outcomes.
- `r2-build.md` is written from `TEMPLATE.md` and every citation in it
  is checked against the tree.
- Nothing under `crates/` changed.

## Not in scope

- The lock merge whole, the typed `JobKind`, `frame.rs` shared (the
  architecture rows under "Open"); the time seam (4-0).
- The quote's simulation under the limiter lock and the outranked
  check (`STATE-2026-10-26.md` names them; neither is this schedule).
- Keyring 4 or any change to which store holds the token.

## Rules

`TEMPLATE.md`, with these differences: a design, so no build and no
gate beyond `git diff --check` and `tools/docs-check.sh` on the
commit that adds the two documents; alone in the main tree on `main`;
scratch under your session's scratchpad in a subdirectory named `r2`;
commit on `main`, never push; the `Co-Authored-By` line for your
model last. A Fable session, or an Opus session with the owner
reading: the roadmap's roles say a step that needs a design has its
design ruled before the build brief is written.

## Report

`TEMPLATE.md`, as text; item 1 is the two documents.
