# 2d — The daemon and limiter file splits (design)

A Fable design for step 2d of `roadmap/README.md`, drawn from three
surveys of the tree at `446223ff` (two Opus: the production code's
coupling map and seams; the tests, their helpers and the journal
oracles; one Sonnet: every citation of the two files and what each
check would say). Every `file:line` is at `446223ff`. The build brief
is `2d-build.md`. Nothing here is built.

## 1. The problem, measured against the tree

**Two files hold a third of the daemon crate's weight in one piece
each.** `crates/acquisition-daemon/src/daemon.rs` is 9,894 lines: 4,857
of production code and 5,037 of tests, 51 per cent. Its one
`impl Daemon` block runs 3,124 lines (`daemon.rs:1148-4271`) and holds
75 methods: logging and persistence, routes and probes, the queue and
the dispatcher, the eleven job kinds, login and the single-flight
refresh, the connection handler, the quote, the idle watchdog. Its
tests sit in three modules whose names say less than they hold:
`dispatcher_tests` (`:5992-9894`, 48 tests) is where the login,
persistence, quote, idle, rails and dead-bearer scenarios live too.
`ratelimit.rs` is 4,509 lines: the header contract, the limiter and
the choke point as production (`:1-2203`), then the spec tables
(`:2204-4509`, 47 tests). The assessment named both as the three
un-split giant units' first two (`roadmap/STATE-2026-10-19.md:18`),
and the 2f ultra review of daemon and network will read them.

**The reading list a session gets for `daemon.rs` misses the network
rulings its own tests pin.** `python3 -I tools/bothy.py pack --path
crates/acquisition-daemon/src/daemon.rs` prints the daemon area's
rulings and not the network area's, because the file's module doc
names `decisions/daemon.md` (`:12`) and never `decisions/network.md`.
Six network rulings are pinned by tests inside that file: C19
(`headerless_503_fails_its_job_and_paces_the_next_send`), C24 and C87
(the dead grant, the dead bearer), C25 (the halted daemon), C26 (the
429 retries), C32 (declared route knowledge). A session about to touch
`valid_access_token` is told to read the daemon's rulings and not the
file where C87 stands.

**The module docs carry 16 KB of registry copies that have all
drifted.** `daemon.rs:10-110` ("Decisions as recorded": C6, C23, C31,
C32, C43 and a 73-line C49–C51 design note), `ratelimit.rs:25-39` (C18,
C19) and `gate.rs:7-17` (C21). Every copy differs from the registry's
entry: C23's copy lacks the children's tally clause 2a added; C32's
says "only `/profile` is declared" where the registry and
`NO_PROBE_ROUTES` (`daemon.rs:1601`) say two routes; C6's and C31's
carry mechanism (restore rules, retention, the lock's path) that no
code comment holds. No production item in either file carries a doc
comment headed by one of those ids; the mechanism lives only in the
copies, and C18, C19 and C32 are cited by nothing else in the code.

**The registry's pins name the file by a bare name that no check
resolves.** Thirteen `*Pinned:*` fields say `daemon.rs` and eight say
`ratelimit.rs` (`decisions/daemon.md:7,9,20,29,31`,
`decisions/network.md:7-11,13-18`, `decisions/plans.md:10,12,13`).
`tools/bothy.py` resolves only a token with a slash (`bothy.py:261-267`),
so a test that moves to another file leaves its pin pointing at the
wrong one, unflagged. The two area intros name the files with slashes
(`daemon.md:3`, `network.md:3`), and those are checked: a layout that
replaced `daemon.rs` with `daemon/mod.rs` would fail `bothy check`.

**The journal oracles are private and not what the notes say.**
`TESTING-NOTES.md`, "The journal is the contract surface", says the
first four invariants "run over every harness journal". The survey
counted: of the 42 daemon tests whose daemon writes a journal, 12 run
all three oracles (`finish_harness_wire`, `daemon.rs:7130`), 10 run one
or two through their own helpers, and 20 run none; six of those twenty
put sends on the wire (`:7279`, `:7340`, `:7874`, `:8064`, `:9679`,
`:9779`). The daemon's probe-first rule (`assert_wire_contract`,
`:7183-7211`) exempts only `oauth-token`, where the notes (`:87`), the
client harness (`acquisition-client/tests/harness/mod.rs:660-663`) and
`NO_PROBE_ROUTES` exempt `profile` and `league` too; it survives because
no checked journal carries a send on either route. The four oracles sit
at `:7110-7278` and `:7642-7679` inside a test module no document names
by file. None of this is a move's to fix (section 7); the split gives
the oracles a file a note can name.

**What the surveys found that a split must not touch.** A lock-order
inversion: `exit_process` holds the `jobs_db` guard as a temporary in
its `match` scrutinee while the error arm calls `note_error`, which
takes `shared` (`daemon.rs:4218-4229`), the reverse of `persist`'s
`shared` then `jobs_db` (`:1198-1218`). A stale comment at `:1168`
("takes the shared lock") contradicting `:1031-1032`. Six doc
paragraphs attached to the wrong item (section 2.4). Two near-twin
test harnesses (`test_daemon` at `:5058` and `:6313`,
`wait_for_refresh_waiters` at `:5256` and `:6289`). All raised in
section 7.

## 2. The shape

A pure move: every statement of the two files lands in one of the new
files unchanged, and a script holds that property at each commit
(section 2.5). What a split may change is exactly four things: the
`use` and `mod` declarations, a module doc's first lines, a visibility
token (`pub(super)`) on an item a sibling file needs, and which item a
misplaced doc paragraph sits on. Nothing is renamed, reordered within
an item, extracted or rewritten. The layout follows the one directory
module the tree has, `acquisition-search/src/trade.rs` beside
`trade/`: the root file keeps its name and path, and its children sit
in a directory of that name. So `daemon.rs` and `ratelimit.rs` stay,
every bare citation of them stays true, and the module's `pub` items
keep their paths (`daemon::run` for `main.rs:39`;
`crate::ratelimit::{Bearer, ChokePoint, Clock, …}` for `auth.rs`,
`mockggg.rs`, `rails.rs` and the daemon) through `pub use` in the root.

The rule that places a type: **a type whose fields more than one child
reads lives in the root; a type one child uses lives with it; a type
one child makes and another consumes lives with its maker,
`pub(super)`.** In Rust a child sees its ancestors' private items and
not its siblings', so with `Daemon`, `Shared`, `Entry`, `Refusal`, the
session types and `CredentialStore` in the root, no field widens
anywhere: the startup that builds the `Daemon` literal and the two test
harnesses that build it field by field (`daemon.rs:4757-4781`, `:5073`,
`:6380`) are children too. What widens is a method or free function
defined in one child and called from another or from a test file,
about forty tokens in all (appendix A names each).

### 2.1 The daemon: a root and ten children

| File | Holds (the sections of appendix A) | Lines, about | Rulings it names |
| --- | --- | --- | --- |
| `daemon.rs` | the module doc with this table in words; the imports; the knobs and constants; `Entry`; `Refusal`; the session layer (`AuthSession`, `Sessions`, the flight types, C49's one interpreter of accounts: `resolve_account`, `canonical_account`, `kind_needs_account`); `Shared`; the `Daemon` struct; `CredentialStore`; the basics every child calls (`log`, `emit`, `note_error`, `unix_now`, `uptime_seconds`, `rails`, `announce_trip`); the `mod` lines and the `pub use` | 850 | `decisions/daemon.md` |
| `daemon/endpoint.rs` | `Stream`, `Listener`, `BoundSocket`: the two platforms' arms, every `cfg(unix)` and `cfg(windows)` in the file (`:889-1027`) | 140 | daemon |
| `daemon/routes.rs` | `route_for` to `ensure_probe` (`:1477-1631`): the routes, `declare_route_knowledge` and `NO_PROBE_ROUTES`, the probes | 160 | network |
| `daemon/queue.rs` | the persisted queue and its admission: `persist`, `queue_failed`, `restore_jobs`, `stored_outcome`, `submit`, `submit_child`, `submit_with_parent`, `cancel`, `set_priority`, `cancelled`, `admit_realm`, `parent_gone` | 480 | daemon |
| `daemon/dispatch.rs` | the dispatcher and a job's life: `dispatcher`, `pick_runnable`, `run_active`, `process`, `conclude`, `maybe_finish_parent`, `finish`, `start_and_finish`, `execute`, `Exec`, `may_requeue_429` | 600 | daemon, network |
| `daemon/kinds.rs` | what each job kind does and what it records: `execute_inner`, `api_get`, `ApiError`, `Auth`, `SessionBearer`, `StagedBearer`, `classify_api_body`, `validate_apply`, `fan_out_stopped`, `record`, `with_index`, `index_apply` | 900 | daemon, plans |
| `daemon/login.rs` | the login flow and the account's end: `auth_start` to `finish_auth_flow_error`, `logout`, `forget_account`, `wait_callback` | 470 | daemon |
| `daemon/refresh.rs` | the single-flight refresh and the dead bearer: `install_tokens_locked`, `reject_access_token`, `valid_access_token`, `finish_refresh`, `RefreshOwnerGuard`, `AccessTokenResult`, its two constants, `session_changed`, `finish_stale`, `wait_for_refresh` | 330 | network |
| `daemon/quote.rs` | `quote` (`:3813-3985`) | 180 | plans |
| `daemon/server.rs` | the connection and the lifecycle: `handle_conn`, `handle_request`, the status renderers (`session_statuses`, `headline_session`, `keyring_summary`, `auth_status`), `reset_rails`, `exit_process`, `is_idle`, `has_live_jobs`, `idle_watchdog`, `Signal`, `recv_event`, `write_line` | 560 | daemon |
| `daemon/start.rs` | `run`, `take_locks`, `run_with_log`, the knob readers (`idle_shutdown_from_env`, `retention_from_env`), `DIAGNOSTIC_CAP_BYTES`, `rotate_if_over` | 450 | daemon |

The largest child is the kinds file, and 560 of its lines are
`execute_inner`'s one `match`; the typed `JobKind` the roadmap defers
past the ultra review is what shrinks it, not this step (section 5).
The "Decisions as recorded" blocks move verbatim to the file that holds
their mechanism: C6 to the queue, C23 to the dispatch, C31 and the
C49–C51 note to the root beside the session layer, C32 to the routes,
C43 to the kinds; C18 and C19 to the limiter (2.3). Each new file's
module doc opens with one sentence saying what the file holds and a
`Rulings:` line naming its area file, as `lib.rs:13` does, so
`bothy pack --path` routes it to that area alone; a file that names two
areas is read under both.

### 2.2 The tests: a parallel tree

The daemon's tests move to `daemon/tests.rs`, a `#[cfg(test)]` child
of the root declaring one file per theme under `daemon/tests/`. As
children of `daemon` they see the root's private types and fields, as
they do today; a method they call that now lives in a sibling child is
`pub(super)`. Tests inline at the foot of each child were weighed and
declined: the dispatcher's harness serves every theme, so each
production file would carry a `use` of it and the login file alone
would reach 2,300 lines (section 5).

| File | Tests (by line at `446223ff`) | Helpers that move with them |
| --- | --- | --- |
| `tests/harness.rs` | none | the dispatcher module's shared helpers: the clocks, the credential stores, `ScriptedResponse` and the servers, `test_daemon` and its scenario forms, the journal readers, `wait_terminal`, `finish_harness` and `finish_harness_wire`, the four oracles (`assert_journal_matches_wire`, `assert_wire_contract`, `assert_pacing_follows_responses`, `assert_never_over_the_limit`), `wire_posts`, `full_hold_ms`, `tripwire_config`, `token_server_answering`, `logged_in` |
| `tests/sessions.rs` | `auth_session_tests` whole (`:4860-5888`, 15) | its own harness as it stands: `MemoryCredentialStore`, `test_daemon`, the token responders, `wait_for_refresh_waiters` |
| `tests/responses.rs` | `response_tests` (`:5890-5991`, 5) | `rate` |
| `tests/diagnostics.rs` | `c83_a_diagnostic_over_the_cap_…` (`:232-286`, 1) | — |
| `tests/bearer.rs` | the `rejected_access_token_*` four and the laptop-sleep expiry (`:6595`, `:6610`, `:6916`, `:6929`, `:6964`) | `:6480-6593`, `:6626-6915`: the scenario builders, the bearer servers, the two assertions, `ts_seconds` |
| `tests/dispatch.rs` | the independence pair, priority, the headerless 503, malformed listings, declared route knowledge, the 429 family, the HEAD 429s, the real-mode whoami (`:7279`, `:7340`, `:7680`, `:7726`, `:7791`, `:8064`, `:8446`, `:8481`, `:8519`, `:8547`, `:8600`, `:8640`, `:9638`; 13) | `BlockingClock` and `saturated_probe_server` (`:6055-6116`, `:6224-6251`), `fetch_payload_marker`, `terminal_event_count`, `journal_waits` |
| `tests/stress.rs` | the N6 stress (`:7407`, 1) | `wait_for_refresh_waiters` (`:6289-6310`) |
| `tests/parents.rs` | apply's fan-out and admission, the parent and child cases (`:7874`, `:7972`, `:9109`, `:9200`, `:9230`, `:9274`; 6) | — |
| `tests/login.rs` | the six login tests (`:8191-8445`) | `login_tokens`, `login_daemon`, `run_login`, `login_result` (`:8129-8190`) |
| `tests/queue.rs` | the seven persistence tests (`:8856`, `:8969`, `:9078`, `:9128`, `:9140`, `:9175`, `:9254`) | `test_db_path`, `persisting_daemon`, `remove_db`, `break_queue` (`:8818-8855`) |
| `tests/rails.rs` | the tripwire, the ceiling, the halted refresh (`:8703`, `:8771`, `:9599`) | `wait_until` (`:8688`) |
| `tests/idle.rs` | the three idle verdicts (`:9305`, `:9342`, `:9412`) | — |
| `tests/refresh.rs` | the dead grant and the transient failure (`:9505`, `:9564`) | — |
| `tests/quote.rs` | the two quote tests (`:9679`, `:9779`) | — |

Sixty-nine tests before and after; the crate's count stays 147. The
two harnesses stay two: `sessions.rs` keeps its own `test_daemon` and
`wait_for_refresh_waiters`, so no name collides and nothing is renamed.
A helper used by tests of one file moves with them; one used by two
files goes to the harness. Each test file opens with
`use crate::daemon::*;` and, where it needs it, `use super::harness::*;`.

### 2.3 The limiter: a root, three children and the spec

| File | Holds | Lines, about | Rulings |
| --- | --- | --- | --- |
| `ratelimit.rs` | the module doc (`:1-24`); `Clock` and `SystemClock` (used by the choke, the daemon, the mock and the rails); `url_path`; the `mod` lines and the `pub use` and `pub(crate) use` that keep every `crate::ratelimit::` path | 80 | network |
| `ratelimit/policy.rs` | the header contract (`:103-438`): `Window`, `WindowState`, `Rule`, `Policy`, `PolicyParseError`, `RetryAfter`, the parsers; the two caps (`RETRY_AFTER_CAP_SECS`, `MAX_POLICY_*`); `response_headers_snapshot` and `rate_limit_snapshot` | 480 | network |
| `ratelimit/limiter.rs` | the buckets and their constants (`:58-102`), `EndpointState`, the keys, `PolicyState`, `Limiter`, the deadline arithmetic, `ScopeProjection` and `rule_statuses` (`:439-1297`); the C18 and C19 blocks | 920 | network |
| `ratelimit/choke.rs` | `Bearer`, `ProbeError`, `SendError`, `CompletedResponse`, the classification, `SentAt`, `ChokePoint` and its sends, `build_client`, the timeouts (`:1298-2161`) | 870 | network |
| `ratelimit/spec.rs` | the test module whole (`:2205-4509`) | 2,300 | — |

The spec stays one file by C33's own word: it is the limiter's spec,
"test tables, not prose", and `CHAR_LIST`, `with_state`, `parse` and
`retry` serve both its halves. What widens for it: `Limiter.policies`,
`PolicyState` and its `raw` and `history`, `observe_landed`,
`LandedObservation` and its fields, `classify_response`,
`ResponseClassification`, `Policy::has_same_shape`, `ChokePoint.limiter`,
twelve tokens the survey listed. The choke's one private reach into
the limiter, `observe_landed` with a `LandedObservation` literal
(`:1899-1901`), widens the same way.

### 2.4 What the move may carry and what it may not

Allowed, because the checker counts them as moves: a doc paragraph
reattached to the item it describes. The survey found six:
`fan_out_stopped`'s doc on `admit_realm` (`daemon.rs:4273-4276`),
`Sessions::get`'s on `find` (`:647-649`), the choke's "callers name the
route" paragraph on `url_path` (`ratelimit.rs:1414-1417`),
`rate_limit_snapshot`'s on `response_headers_snapshot` (`:2157-2158`),
`ChokePoint::check`'s on `declare_policyless` (`:1560-1562`),
`Limiter::wait_for`'s on `declare_policyless` (`:575-578`). The build
brief names each; the worker moves a paragraph only from that list.

Not allowed: a word of code changed, the stale comment at `:1168`
corrected, a helper deduplicated, an arm of `execute_inner` extracted
(each arm's tail is a bare `Outcome` the enclosing `Ok(match …)`
wraps, so an extraction edits every arm; and the `apply` arm shadows
`kind`), the oracle aligned, the lock inversion fixed, a copy's drift
corrected. Each is in section 7 with its home.

### 2.5 The checker

A pure move is a property the gate cannot see and a reviewer cannot
read off a 10,000-line diff, so the build brief carries a script
(`2d-build.md`, appendix A) that holds it: the statements of the old
file, read from `git show <sha>:<path>`, equal the statements of the
new files as a multiset. It sets aside what a split must change (blank
lines, `use` and `mod` lines, `//!` lines, `#[cfg(test)]` and `#[path]`
attributes, a leading visibility token), removes whitespace, cuts code
at `;`, `{` and `}`, drops the trailing comma rustfmt drops when it
joins lines, and keeps each comment line as one statement. Measured on
this tree: the dispatcher tests dedented into a file and formatted
reflow 58 lines and the checker reports 0 missing, 0 added; one
signature given an extra parameter reports 1 and 1; `daemon.rs` against
itself is 7,481 statements and `ratelimit.rs` 2,539. Each move commit
quotes the checker's line in its message. Module-doc lines are outside
the checker, so the message also states that every `## C<n>` block
appears once across the crate's module docs at its recorded size
(section 1's bytes).

## 3. What each reader meets afterwards

- **A session about to touch the refresh** runs `bothy pack --path
  crates/acquisition-daemon/src/daemon/refresh.rs` and reads the
  network area, where C24 and C87 are; the routes file reads network,
  the quote reads plans, the dispatch reads both daemon and network.
- **The registry** names each pinned test by a path with a slash
  (`daemon/tests/queue.rs`, `a_broken_queue_is_fatal_to_restore`), which
  `bothy check` resolves from then on; the nineteen fields (C19 and C32 name both files) and the three
  intros of appendix B are edited in the commit that moves each test.
- **`TESTING-NOTES.md`** names `daemon/tests/harness.rs` as the
  oracles' home and says what is true of who runs them (section 7's
  first item gives the sentence).
- **The 2f reviewer** reads a 160-line routes file under C32, a
  330-line refresh under C87, and a kinds file whose one `match` is the
  deferred `JobKind` item's evidence.
- **`git blame`** on a moved line finds the move commit; the commit's
  message names the source ranges of each new file, and
  `git blame -C -C` finds the line's origin. `git log --follow` on a
  child finds nothing before the move; the message is the pointer.

## 4. The rulings this needs

None. The layout within a crate is an internal, the agents' to decide
and build; C1's edges, C122's one binary per crate and the invariants
are untouched, and `tools/arch-check.sh` sees the daemon crate as a
package. The registry changes are *Pinned:* paths and the three area
intros, under the owner's delegation and listed in appendix B for his
review; no entry's ruling words change. One registry habit is raised,
not ruled (section 7): a *Pinned:* file written with its path from the
crate's `src/`, so the check that exists resolves it.

## 5. Alternatives weighed and declined

- **`daemon/mod.rs` and `ratelimit/mod.rs`.** `bothy check` resolves
  the slash paths in `decisions/daemon.md:3` and `network.md:3` and
  would fail; thirty bare citations would go false; the tree's one
  precedent is `trade.rs` beside `trade/`.
- **The types in their children, fields `pub(super)`.** About 55 field
  tokens (`Daemon` 17, `AuthSession` 11, `Sessions` 7, `Shared` 7,
  `Entry` 6, `RefreshOwnerGuard` 5) for no property gained; the root as
  the daemon's state is the shape a reader expects.
- **Tests inline at the foot of each child.** The shared harness would
  be a `use` in every child, the login file would reach 2,300 lines,
  and a test that pins two files' mechanism (the dead bearer: refresh
  and dispatch) has no one foot to sit at.
- **One `daemon/tests.rs` of 5,000 lines.** The file the step exists to
  split, moved.
- **The limiter's spec split in three.** Four helpers serve both halves
  and C33 calls it one spec; a split would widen helpers for a file
  boundary the ruling does not draw.
- **One harness for the daemon's tests.** `test_daemon` and
  `wait_for_refresh_waiters` exist twice with different shapes; merging
  them is an edit, and renaming one is a rename. Raised in section 7.
- **Extracting `execute_inner`'s arms into one method per kind.** Not
  a move (section 2.4); the typed `JobKind` the roadmap defers past the
  ultra review does it with a type, once.
- **Fixing the oracle, the lock inversion, the stale comment and the
  copies' drift in the same step.** Each changes behaviour or meaning,
  which the checker exists to refuse; each has a home in section 7.
- **A kept `tools/move-check.py`.** A one-off whose evidence is the
  three commit messages; the script lives in the brief, which the
  phase close deletes and history keeps.
- **A `bothy` rule that every *Pinned:* file carries a slash.** A new
  check for a habit met once here; raised, with the nineteen fields this step
  converts as the evidence a later ruling would cite.

## 6. Risks and what holds them

- **Windows.** The `cfg(windows)` arms move whole into `endpoint.rs`,
  and a macOS build cannot compile them; an import that the Windows
  arm needs and the Unix arm does not fails only there. The Windows
  gate runs on the owner's push (`.github/workflows/gate-windows.yml`,
  `cargo clippy` on the `cfg(windows)` code); the brief asks the worker
  to build and clippy the crate on the owner's Windows guest over ssh
  before the step closes, where the guest is up, and otherwise to say
  so and leave the Windows gate to the push.
- **`unused_imports` under `-D warnings`.** Each child imports with
  `use super::*;`, which carries the root's imports and private items
  down without a per-file list; a child that used nothing of the root
  would warn, and none does.
- **Four intra-doc links** (`daemon.rs:175`, `:480`, `:903`, `:3029`):
  each pair lands in one file; `cargo doc` under `-D warnings` is a
  gate stage.
- **Tests in a sibling of the method they call.** About thirty
  `pub(super)` tokens, each named in appendix A; a missed one is a
  compile error, not a silent change.
- **rustfmt's reflow** when a nested test module becomes a file: the
  checker is reflow-insensitive (section 2.5), and the diff shows the
  reflowed lines, which the reviewer reads under `--color-moved`.
- **The README's knob rows** link `ACQ_IDLE_SHUTDOWN` and the retention
  knobs to `daemon.rs`; their readers move to `start.rs`, and the link
  moves with them. `tools/docs-check.sh`'s knob scan reads the
  production tree and excludes `*/tests/*`, so the test tree needs no
  README row.
- **Scope pressure.** The surveys found eight things a reader would fix
  while there. Section 7 gives each a home; the worker's report names
  any ninth as raised, never fixed.

## 7. Raised for the owner, each with its home

1. **The oracles' coverage and `TESTING-NOTES.md`'s sentence.** The
   step corrects the sentence to what is true: the three oracles run
   through `finish_harness_wire` in twelve tests and through the
   scenario helpers in ten more, and twenty journaling daemons check
   nothing. Running the oracles over every journaling daemon, and
   aligning the probe-first rule's exemptions with `NO_PROBE_ROUTES`,
   is a test change with fallout the survey could not size (`:8064`
   sends on both exempt routes and checks nothing today); proposed as a
   step of its own after the move lands, `2d-oracle`, before 2f reads
   the tests, or as a line for 2f's fix list. The design recommends the
   step: it is small, the file now has a name, and 2f's reviewers
   should read oracles that run.
2. **The client harness's probe-first rule** matches the route name
   before `@` (`harness/mod.rs:660-663`) and would miss a realm-suffixed
   `league/xbox@…`; the daemon's uses `split_endpoint_key`. For
   `2d-oracle` or 2f.
3. **The lock inversion in `exit_process`** (`daemon.rs:4218-4229`,
   `jobs_db` then `shared`; `persist` is `shared` then `jobs_db`). For
   2f, with the lock order as practised (store, shared, jobs_db,
   queue_failure, log; shared before the choke's and the rails'
   internals; a gate permit before shared) as the sentence 2f writes
   where the locks are declared.
4. **The stale comment at `daemon.rs:1168`**: `log` takes no shared
   lock; the field doc at `:1031-1032` is right. A one-line fix for the
   first commit that touches the root for another reason.
5. **The copies' drift.** The "Decisions as recorded" blocks move
   verbatim and stay wrong where they were wrong. Their cut to the
   mechanism each file holds, headed by the id as `AGENTS.md`'s "Where
   things live" asks, is prose the owner's budget reassessment or 2f
   decides; the design notes that C18, C19 and C32 would then be cited
   by no code line unless the mechanism comment names them.
6. **Two harnesses.** `sessions.rs`'s `test_daemon` and
   `wait_for_refresh_waiters` beside the harness's; a merge is an edit
   for a test step, 2d-oracle or 2f.
7. **A *Pinned:* file with a slash.** This step writes nineteen fields that
   way; whether `bothy check` should require it everywhere is the
   owner's, as a line in its docstring's entry rule.
8. **`execute_inner`** stays one 560-line `match` in `kinds.rs`; the
   typed `JobKind` is deferred past 2f by the roadmap and this step
   does not reopen it.

## 8. The owner's word

Awaited.

## Appendix A — the daemon's items by destination (at `446223ff`)

Line ranges are the survey's, verified against the file. A `pub(super)`
column names the methods and functions that cross a file boundary
(called from another child, from the root, or from a test file); a type
listed in the root needs no change.

| Destination | From `daemon.rs` | `pub(super)` |
| --- | --- | --- |
| `daemon.rs` (root) | the module doc's first nine lines and the C31 and C49–C51 blocks (`:1-9`, `:26-29`, `:38-110`); the imports (`:112-147`); the constants (`:150`, `:160-172`); `Entry` and its impl (`:381-449`); `Refusal`, its impls, `AccessTokenResult` excepted (`:518-564`); the session types and impls (`:567-813`); `Shared` and its impl (`:815-888`); `Daemon` (`:1029-1083`); `CredentialStore`, `OsCredentialStore` (`:1131-1146`); the basics (`:1152-1191`, `:1459-1475`); the account selectors (`:1636-1686`) | none: the root's items are visible to every child |
| `endpoint.rs` | `:889-1027` | `Listener::bind`, `Listener::accept`, `BoundSocket::remove_if_still_ours` |
| `routes.rs` | the C32 block (`:30-33`); `:1477-1631` | `route_for`, `keyed_route_for`, `needs_auth`, `serial_key`, `probe_pending`, `declare_route_knowledge`, `route_probes`, `ensure_probe` (`NO_PROBE_ROUTES` with them) |
| `queue.rs` | the C6 block (`:18-21`); `:1193-1354` (`persist`, `queue_failed`, `restore_jobs`, `stored_outcome`); `:1687-1889` (`submit` to `cancelled`); `admit_realm` (`:4283-4293`), `parent_gone` (`:4495-4497`) | `persist`, `queue_failed`, `restore_jobs`, `stored_outcome`, `submit`, `submit_child`, `submit_with_parent`, `cancel`, `set_priority`, `cancelled` |
| `dispatch.rs` | the C23 block (`:22-25`); `Exec`, `may_requeue_429` (`:372-379`); `:1896-2287` | `dispatcher`, `pick_runnable`, `conclude`, `maybe_finish_parent`, `finish`, `start_and_finish`, `execute`, `Exec`, `may_requeue_429` |
| `kinds.rs` | the C43 block (`:34-37`); `ApiError`, `Auth`, the bearers (`:291-369`); `record`, `with_index`, `index_apply` (`:1359-1457`); `:2289-2941`; `validate_apply`, `fan_out_stopped`, `classify_api_body` (`:4307-4372`, `:4387-4424`) | `execute_inner`, `record`, `with_index`, `index_apply`, `validate_apply`, `classify_api_body`, `ApiError` |
| `login.rs` | `:2948-3279` (`auth_start` to `finish_auth_flow_error`); `logout`, `forget_account` (`:3546-3601`); `wait_callback` (`:4428-4461`) | `auth_start`, `begin_auth_flow`, `stage_auth_flow`, `staged_profile_token`, `login_with_profile`, `complete_login`, `abort_login`, `logout`, `forget_account` |
| `refresh.rs` | `AccessTokenResult` and the two constants (`:473-477`); `RefreshOwnerGuard` and its impls (`:1085-1129`); `:3287-3542`; `session_changed`, `finish_stale`, `wait_for_refresh` (`:4490-4516`) | `install_tokens_locked`, `reject_access_token`, `valid_access_token`, `finish_refresh`, the two constants |
| `quote.rs` | `:3813-3985` | `quote` |
| `server.rs` | `:3603-3663` (the status renderers); `:3678-3804`; `:3987-4270`; `has_live_jobs` (`:4374-4385`); `Signal`, `recv_event`, `write_line` (`:4465-4485`, `:4518-4525`) | `handle_conn`, `handle_request`, `keyring_summary`, `auth_status`, `reset_rails`, `is_idle`, `has_live_jobs`, `idle_watchdog` |
| `start.rs` | `idle_shutdown_from_env` (`:154-159`); `DIAGNOSTIC_CAP_BYTES`, `rotate_if_over` (`:174-229`); `retention_from_env` (`:454-471`); `:4530-4857` | `run`, `rotate_if_over`, `DIAGNOSTIC_CAP_BYTES` (re-exported `pub` from the root), `idle_shutdown_from_env` |

The worker confirms each range against the tree before moving it and
names in the report any line this table misplaces.

## Appendix B — the citations the step edits

| Site | Today | After |
| --- | --- | --- |
| `decisions/daemon.md:7` (C3) | `daemon.rs`, `idle_verdict_*` | `daemon/tests/idle.rs`, `idle_verdict_*` |
| `decisions/daemon.md:9` (C6) | `daemon.rs`, `a_broken_queue_is_fatal_to_restore` | `daemon/tests/queue.rs`, … |
| `decisions/daemon.md:20` (C23) | `daemon.rs`, the two parent tests | `daemon/tests/parents.rs`, … |
| `decisions/daemon.md:29` (C31) | `ratelimit.rs`, `account_rules_are_paced_per_account` | `ratelimit/spec.rs`, … |
| `decisions/daemon.md:31` (C50) | `daemon.rs`, `a_login_whose_profile_fetch_fails_fails_whole` | `daemon/tests/login.rs`, … |
| `decisions/network.md:7,8,18` (C17, C18, C33) | the `ratelimit.rs` test tables | the `ratelimit/spec.rs` tables |
| `decisions/network.md:9` (C19) | `ratelimit.rs`, …; `daemon.rs`, `headerless_503_…` | `ratelimit/spec.rs`, …; `daemon/tests/dispatch.rs`, … |
| `decisions/network.md:10` (C20) | `ratelimit.rs`, `probes_teach_…` | `ratelimit/spec.rs`, … |
| `decisions/network.md:11` (C21) | `ratelimit.rs`, `common_gate_caps_…`; `gate.rs`, … | `ratelimit/spec.rs`, …; `gate.rs` unchanged |
| `decisions/network.md:13` (C24) | `daemon.rs`, the dead-grant test and `a_completed_login_clears_the_dead_grant_mark` | `daemon/tests/refresh.rs`, …; `daemon/tests/login.rs`, … |
| `decisions/network.md:14` (C87) | `daemon.rs`, `rejected_access_token_*` | `daemon/tests/bearer.rs`, … |
| `decisions/network.md:15` (C25) | `daemon.rs`, the halted-idle and halted-refresh tests | `daemon/tests/idle.rs`, …; `daemon/tests/rails.rs`, … |
| `decisions/network.md:16` (C26) | `daemon.rs`, the two 429 tests | `daemon/tests/dispatch.rs`, … |
| `decisions/network.md:17` (C32) | `ratelimit.rs`, …; `daemon.rs`, `declared_route_knowledge_holds_against_the_mock` | `ratelimit/spec.rs`, …; `daemon/tests/dispatch.rs`, … |
| `decisions/plans.md:10,12,13` (C40, C42, C43) | `daemon.rs`, … | `daemon/tests/quote.rs`, …; `daemon/tests/parents.rs`, … (both) |
| `decisions/daemon.md:3`, `network.md:3`, `plans.md:3` | the intros' file lists | `acquisition-daemon/src/daemon.rs` and the files under `daemon/`; `acquisition-daemon/src/ratelimit.rs` and the files under `ratelimit/`; "the `apply` kind in `daemon/kinds.rs`" |
| `crates/acquisition-daemon/src/lib.rs:3,7` | the module list | the roots and their directories, one clause each |
| `README.md:234-235,246` | `[daemon.rs][daemon]`, the link to `daemon.rs` | `[start.rs][daemon]`, the link to `daemon/start.rs` |
| `REVIEW-CHECKLIST.md:15,16` | the `ratelimit.rs` test tables; `(ratelimit.rs)` | `ratelimit/spec.rs` |
| `TESTING-NOTES.md:81-82` and `:171` | "The first four run over every harness journal"; "The harness leaves `daemon.rs`" | section 7's sentence, naming `daemon/tests/harness.rs`; "leaves `daemon/tests/`" |
| `.claude/skills/live-run/SKILL.md:38,48` | `ratelimit.rs`'s test table; `route_probes` in `daemon.rs` | `ratelimit/spec.rs`; `daemon/routes.rs` |
| `crates/acquisition-protocol/src/status.rs:4` | `ratelimit.rs`'s `rule_statuses` | `ratelimit/limiter.rs`'s |
| `crates/acquisition-protocol/tests/wire.rs:225` | `daemon.rs`, the quote's `not_covered` | `daemon/quote.rs` |
| `crates/acquisition-store/src/world.rs:111`, `crates/acquisition-cli/tests/world.rs:586` | the cap is the daemon's, `daemon.rs` | `daemon/start.rs` |
| `tools/persist-verify.py:78`, `tools/tracer-verify.py:21,43,68` | comments naming the two files | `daemon/routes.rs`; `ratelimit/limiter.rs` |
| `daemon.rs:4243` (moves to `server.rs`) | `dispatcher_tests::idle_verdict_*` | `tests/idle.rs`'s `idle_verdict_*` |

Left as they are: `crates/acquisition-protocol/src/protocol.rs:75,256`
(`Refusal` stays in `daemon.rs`), every `tests/daemon.rs` of the verbs
crate, every history citation, `roadmap/STATE-2026-10-19.md` and the
audits (dated evidence), `docs/design/network-ground-truth.md:801` (the
spike's file).
