# daemon-safety — the daemon paths the league run will hit

Issues #113, #116 and #110, the step after the clock-dependent tests
(`roadmap/README.md`, "Current step"), opened by the owner's ruling of
2026-10-29 that added the `trigger:ci` label. The rhythm is "A step".

## Goal

Three daemon paths the owner's league run can meet are made boring.
The daemon log holds no token by construction (#113): a token is a
type whose text no line can format, from where it enters the daemon to
where it leaves, and every line reaches the log through one function.
A use door that respawned a daemon and still found another contract
stops the daemon it started before it refuses (#116), so a partial
install leaves nothing holding the world for the hour. And a 401 that
lands while the account index is unreadable still narrows the keyring
entry (#110), so a restart never restores a bearer GGG rejected.

## Read

- `AGENTS.md`: invariant 5 (refresh tokens never in plaintext on disk;
  the log is on disk), C47 (no `unwrap` on external input), C122 (one
  harness; a behaviour pinned once, at the lowest crate that holds it).
- `decisions/daemon.md`: C7 (params never carry a secret; tokens are
  obtained inside the daemon), C10 (single-version handshake: a use
  verb may replace a mismatch), C82 (one installation, the sibling
  `acqd`), C84 (the runtime identity: contract and artifact), C134 (the
  panic hook writes one line through a clone of the log's file), C135
  (the keyring entry holds the access token beside the refresh token),
  C144 (no success the daemon did not earn), C148 (credential and index
  I/O under the credentials lock, never `shared`).
- `decisions/network.md`: C87 (a 401 bearer is dead), C88 (the real
  provider is the default; a real-mode daemon is started by a human).
- `REVIEW-CHECKLIST.md`, "Protocol, client, daemon lifecycle, the
  world": "A test that owns what it does not clean", "A stop that
  returned early" (the accounting #116's pin reuses), "A Unix mechanism
  assumed in product code".
- Issue #113 (no comments): its done-when is three clauses, quoted in
  "Done when" below. Issue #116 (no comments): "Done when that is ruled
  under C10 and pinned by the partial-rebuild case leaving no daemon of
  the world bound, counted by the harness's endpoint accounting."
  Issue #110 (no comments): "Closes when: a live run or a sitting meets
  the case, or the index-first rule is relaxed; then the restart test
  under a frozen index rides with that change." This step relaxes the
  rule for a narrowing rewrite, the issue's second trigger.
- The code:
  - `crates/acquisition-daemon/src/auth.rs`: `TokenResponse` (98),
    `KeyringEntry` (354), `exchange_code` (128), `refresh` (157),
    `token_request` (176), `keyring_save`/`keyring_load` (456, 464).
  - `crates/acquisition-daemon/src/daemon.rs`: the staged login's and
    `AuthSession`'s token fields (531 to 543), `CredentialStore` (1011), `Daemon::log` (1128),
    `note_error` (1140).
  - `crates/acquisition-daemon/src/ratelimit/choke.rs:20`, the `Bearer`
    trait; the two `bearer_auth` calls (552, 1016).
  - `crates/acquisition-daemon/src/daemon/start.rs`: the panic hook
    (`install_panic_hook`, 269), `end_at_once` (243), the startup
    writes in `run` (310 to 388), `restore_sessions` (807).
  - `crates/acquisition-daemon/src/daemon/refresh.rs`:
    `persist_credentials` (130), `reject_access_token` (243),
    `land_rejection` and `persist_rejection` (268 to 324).
  - `crates/acquisition-daemon/src/daemon/tests/bearer.rs:762`,
    `rejected_access_token_across_a_restart`, the scenario #110's pin
    extends.
  - `crates/acquisition-client/src/client.rs`: `NotReplaced` (572), its
    words (616), `open_door`'s respawn branch (1364 to 1371), `stop`
    (1578); the module doc's "still, after a respawn" (170 to 177).
  - `crates/acquisition-cli/tests/daemon_observe.rs`: `stage` (152) and
    the `c84_` tests, the C10 pin file; the harness's `sockets_under`
    and `scratch_tmp` (`acquisition-client/tests/harness/mod.rs`, 464
    and 138).
  - `crates/acquisition-verbs/src/diagnose.rs:15` to 17, the module
    doc's sentence on what the diagnosis never prints.
- `python3 -I tools/bothy.py pack --path` for `auth.rs`, `daemon.rs`,
  `choke.rs`, `refresh.rs` and `client.rs` routes to `daemon` and
  `network`; nothing there beyond what is cited above bears on the
  three concerns. `diagnose.rs` names no area (`--area frontends`).

## Input

The last push's CI, read before this brief: `Gate (Linux)` run
40940013037 and `Gate (Windows)` run 40940013132, both on `49688ee0`
(the last pushed commit; `main` here is at `884e7ffd`, the clock-tests
close, not yet pushed), both completed `success`. Nothing to fix or
file.

No sweep: #113's class is the log's writers, listed here in full,
measured on `884e7ffd` with
`grep -rnE '[.]log[(]|[^a-z_]log[(]|note_error[(]|writeln![(]'` over
`crates/acquisition-daemon/src`, leaving out tests, the mock server,
the journal's and rails' own files (`rails.rs` 488, 841), the IPC write
(`server.rs:863`) and stderr (`main.rs`, `start.rs:213`). 84 sites:

| File | Lines |
| --- | --- |
| `daemon.rs` | 1131 (`Daemon::log`'s own write), 1135, 1149, 1160, 1164, 1175 |
| `daemon/dispatch.rs` | 168, 188, 194, 274, 307, 318 |
| `daemon/kinds.rs` | 105, 175, 213, 239, 258, 711, 856 |
| `daemon/login.rs` | 44, 85, 373, 377, 388, 390, 419, 484, 670, 674, 711 |
| `daemon/queue.rs` | 51, 77, 88, 130, 160, 288, 297, 304 |
| `daemon/refresh.rs` | 250, 322, 472, 630, 633, 638 |
| `daemon/routes.rs` | 166, 179 |
| `daemon/server.rs` | 154, 167, 281, 294, 316, 336, 369, 436, 490, 493, 565, 588, 596, 629, 639, 646, 656 |
| `daemon/start.rs` | 227, 248, 298, 347, 369, 380, 384, 549, 558, 570, 580, 582, 592, 596, 663, 669, 713, 773, 840, 854, 868 |

Of these, eight write the log's file themselves rather than through
`Daemon::log`: `daemon.rs:1131` (its body), and `start.rs` 248
(`end_at_once`), 298 (the panic hook), 347, 369, 380, 384 (the startup
lines); `start.rs` 773, 840, 854 and 868 write `restore_sessions`'
buffer, which 663 relays line by line through `Daemon::log`.

A token in the daemon today is a `String`: `AuthSession`'s
`access_token` and `refresh_token`, the login's staged tokens,
`TokenResponse`, `KeyringEntry`, the keyring secret through
`CredentialStore`, `Bearer::token`, and the login's code and PKCE
verifier. Their `Debug` is redacted on the two structs that hold both
(`TokenResponse`, `KeyringEntry`); nothing stops a `format!` of the
field.

#116's failing input: a use verb whose sibling `acqd` is not the
artifact that runs, so the daemon it spawns is judged another identity
both times. The partial rebuild (`acq` rebuilt, `acqd` not) is staged
without a rebuild: a copy of `acq` whose sibling `acqd` is a shell
script that `exec`s the built `acqd`, so the client hashes the script
and the daemon its own executable (C84's artifact dimension, the same
refusal the issue quotes). Unix only: a script cannot stand as
`acqd.exe`; the code it pins is platform-neutral.

#110's failing input: `rejected_access_token_across_a_restart`'s first
lifetime with `accounts.json` replaced by bytes that are not JSON after
the restore and before the job meets its 401, then put back before the
second lifetime. Today the landing's persist fails at the index-first
hold, the entry keeps `at-kept`, and the second lifetime's first send
carries it and meets one more 401.

## Do

0. Concern 0, its own commit before this brief: the owner's word "Yes,
   add a CI trigger label." `AGENTS.md`, "Where things live", the line
   for work not done yet, gains "`ci`, a red CI run" in its trigger
   list, the line shortened to stay inside the 15,277-byte budget; a
   dated bullet in `roadmap/README.md`'s rulings.
1. **#113.** A token is `auth::Secret`, from where it enters the
   daemon (the token response's body, the keyring's read, the login's
   callback, the PKCE pair) to where it leaves (the bearer header, the
   token POST's form, the keyring's write). `Secret`'s `Debug` and
   `Display` print a placeholder, it has no `Serialize`, `Deref` or
   `AsRef<str>`, and its text is read by one private method, visible in
   `auth.rs` alone: the header, the form and the keyring's write are
   built there. `Bearer::token` returns a `Secret`; `CredentialStore`
   carries one. Every line the daemon writes to its log goes through
   one function, which `Daemon::log`, the panic hook, `end_at_once` and
   the startup writes call; `restore_sessions` returns its lines rather
   than writing a buffer. A test runs a login, a send with its bearer,
   a 401, and a refresh against the mock provider, with tokens the test
   can recognise, and reads the whole log: no token, code or verifier
   in it. `diagnose.rs`'s module doc names its token redaction a belt
   over a log that holds none by construction.
2. **#116.** The design, the step's own (agent-facing): a door that
   spawned a daemon, judged it another identity after its respawn, and
   finds it is the child this door started (its pid), stops it before
   refusing; a daemon the door did not start is left as now. The
   refusal keeps its words and adds what happened to the daemon and
   what to do: rebuild or reinstall so `acqd` and the frontend are one
   build. A stop that fails is said in the refusal. C10, C82, C84 and
   C88 keep their words (a use verb already may replace a mismatch; this
   stops what the same door started). Pinned in `daemon_observe.rs` by
   the staged partial install: the use verb refuses with the words, and
   `sockets_under(&scratch_tmp(base))` is empty after it.
3. **#110.** Pinned first by its input (above): the restart test under
   a frozen index, which fails on this tree. Then one rule: the
   index-first hold guards a name; a rewrite that only narrows an entry
   the session already holds in the keyring under its own name (the
   401's landing: the same name, the refresh token alone) adds no name,
   so it saves without the index. A narrowing save that fails leaves
   the entry and warns: the entry's refresh token is still the live
   grant, and clearing it would cost a login to save one 401.

## Done when

- `AGENTS.md`'s "Where things live" names `ci`, a red CI run, among
  the triggers; `tools/docs-check.sh` holds its budget; the README's
  rulings carry the owner's words of 2026-10-29.
- #113: "the daemon writes every log line through one function whose
  inputs cannot carry a token (a type, not a convention), a test pins
  that no token reaches the log on the send, refresh and login paths,
  and the diagnosis's redaction is named a belt in its module doc."
  Read on the tree: no `access_token` or `refresh_token` field of type
  `String` in the daemon's product code; one `fn` writes the log's file
  and the eight direct writers above call it; `Secret`'s text is read
  only in `auth.rs`.
- #116: the partial-install case refuses with words that say the
  daemon this door started was stopped and what to do, and leaves no
  daemon of the world bound by the harness's endpoint accounting; the
  design is recorded in the commit's message.
- #110: the restart test under a frozen index is in `bearer.rs`, its
  second lifetime sends no 401 and refreshes first; the narrowing rule
  is in `persist_rejection`'s doc, headed by C135 and C87.
- The gate green on the step's tree.

## Not in scope

- The journal's redaction (`rails.rs`): it holds no token by its own
  construction, and C152's fields are not the log.
- The diagnosis's redactor itself: it stays the belt; its needles and
  shapes are untouched.
- The mock server's tokens (`mockggg.rs`): the server side of the
  wire, not the daemon's log.
- A daemon on another world, or one this door did not start: C83 and
  C10 leave them standing, and #116 changes neither.
- The other three of #110's four conditions (a store failing under the
  daemon in general): the family stays C144's.

## Rules

`TEMPLATE.md`'s, and the orchestrating session's: work on `main` in the
shared checkout, never switch branches or push; `gh` writes one call at
a time; any `acq` in a mock world; kill only your own pids; restore an
experiment from a scratch copy, never `git checkout -- <file>`. The
status note is the orchestrator's
`side-session-notes/local_5e6841d2-…/daemon-safety-status.md`.

## Report

`TEMPLATE.md`'s.
