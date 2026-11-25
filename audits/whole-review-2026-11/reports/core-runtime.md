## Core runtime scout: daemon, transport, protocol, client (HEAD 122c44fa)

### 1. Model
`acqd` owns all GGG traffic. A request is a job: `Shared.jobs` in memory is the truth, mirrored to `daemon.db` at every change (C6). The dispatcher (`daemon/dispatch.rs`) picks one job per "serial key". `process` probes unknown routes with HEAD, fails degraded routes, and polls `choke.check` in ≤1 s slices. Then `ChokePoint::acquire_send` (`ratelimit/choke.rs:465`) re-checks everything: rails halt, `wanted`, limiter wait plus C190 closure, a gate permit (≤2 in flight, one per policy, HEAD exclusive), then the same checks again. Only after that does it read the bearer, send, observe headers, classify the answer (block first) and record it to the journal, rails and ring.

"May this route send now?" is answered by five holders under separate mutexes:
- limiter policy windows
- limiter `retry_holds`
- limiter `Degraded`, plus the daemon's `closed_with` mirror
- the rails' persisted C190 route closure
- the rails' halts (tripwire, block, unreachable, ceiling, stopping)

`rails.rs` is a grab-bag. It holds block recognition, the halts, the C190 closure ladder, the dead-grant mark, the journal writer and the env config, and persists four kinds of mark in `rails.json`.

The client is 2,264 production lines (`client.rs`) plus 483 (`door.rs`) for connect, lazy-spawn, the exact-match handshake (C10/C84) and stop. Lifecycle is a pure machine (`life.rs`, C157), and the idle exit is real behaviour.

### 2. Measurements
| What | Value | Command |
|---|---|---|
| Production / test lines | daemon 17,231 / 30,772; client 3,320 / 1,929; protocol 1,659 / 162 (+1,024 `tests/wire.rs`); transport 165 / 72 | python script over `git ls-files`, cut at `#[cfg(test)] mod tests {` |
| Comment lines : code lines (slice production) | 6,270 : 14,948 (0.42) | same script, `//`-prefixed lines |
| `Mutex<…>` fields in daemon production (mock excluded) | 16 | `git grep 'Mutex<' … \| grep -v //` |
| `#[cfg(test)]` items inside production code | 45 (18 in `daemon.rs`) | python script |
| Halt / closure check sites outside `rails.rs` | 12 `.halted()` + 3 `closed_for` | `git grep '\.halted()\|closed_for'` |
| `Refusal::new` / `Outcome::failure` sites | 59 / 23 | `git grep -c` |
| Protocol variants | Request 15, Response 14, Bootstrap 2+2 | `grep` of `protocol.rs:507-779` |
| Job kinds (string `match` arms in `kinds.rs`) | 11: 3 mock-only (`sleep`, `fetch`, `whoami`), 1 internal (`probe`) | `grep '"…" =>' kinds.rs` |
| Mock lines shipped in release `acqd` | 1,156, plus `mock-items.json` | `lib.rs:37` is `pub mod mockggg` (not cfg(test)) |
| Slice commits since the audit (`5f3ad0d`) | 44 of 658 | `git log 5f3ad0d..HEAD -- <crates>` |

### Fate of the semantic-model audit's findings
| Finding | State at HEAD |
|---|---|
| Gate occupancy `debug_assert!` ×4 (set, not multiset) | **Standing**, verified at `gate.rs:113,206,256,259`; filed #206 and #66, open |
| `BlockShape::of` substring match sets a persisted mark | **Standing**, verified at `rails.rs:291-311`. "Ray ID" now needs the whole phrase, and the `Verdict::of` narrowing stays. Filed #207 (ruling wanted), open |
| `PolicyMismatch` arm skips `assume_counted` | **Standing, unfiled**. Verified at `limiter.rs:588-638`: history is pushed but hits are not advanced |
| `persist` needs `shared` by comment only | **Standing, unfiled** (`queue.rs:214-219`). Its sibling `end(&self, s: &mut Shared, …)` (`entry.rs:221`) was typed by #261 |
| Dashboard unpaged | **Standing**, #177 open (`server.rs:459`) |
| Idle hour spent during a long job | **Fixed** (#208; `entry.rs:223`, `queue.rs:1189`) |
| `frame.rs` duplicated | **Standing**: the diff is comment-only, and the doc now claims "byte for byte", held by nothing. Row in #100 |
| Unknown job-state word dropped silently | **Fixed**: logged and refused (#209, `queue.rs:387`). `job_state()` is still a second parser (`daemon.rs:330`) |
| Outcome that no longer reads becomes "no job" | **Fixed** (#209) |
| Submit racing stop | **Fixed** (#209, `44139af6`) |
| Ignored `persist` results | **Named** in C6's residual (#209) |
| Spawn race (late pid) | **Standing, unfiled**: `door.rs:330-341` reads the holder on child exit, and `world.rs:955-957` locks, then truncates, then writes the pid |
| Response plane lenient vs C85; C147 env reads | Filed #210, open |
| Dead-grant mark keyed by username | **Standing, unfiled** (`refresh.rs:482`) |
| Keyring rewrite on a 401 skipped while `saving` | **Standing, unfiled** (`refresh.rs:306`) |
| `retry_holds` vs `retry_hold_until` | **Standing**. Only the dead `secs` field is in #214 |
| Life step's wildcard arm | **Fixed** (#198) |

So: 6 fixed, 5 filed and open, 6 unfiled and standing, one of them latent-dangerous (`PolicyMismatch`).

### 3. Where a deep review would pay

**H1. One admission authority (highest payoff; confidence medium).** One question is answered in three tiers:
- `pick_runnable` (`dispatch.rs:85-120`)
- `process` (`dispatch.rs:156-226`)
- `acquire_send` and `acquire_head` (`choke.rs:465-534`)

Two of the tiers serialize independently: the dispatcher by serial key, the gate by policy. Pacing is a 50 ms–1 s polling loop across three locks. Route closure has three representations:
- limiter `Degraded`, plus its `closed_with` mirror (`daemon.rs:1141,1750`)
- the rails' C190 closure
- `retry_holds`

`quote.rs` re-derives the same answer for its ETA. One lock-held `admit(route, now) -> Go(permit) | Wait{until, why} | Refuse(why)`, which `quote` also simulates, would:
- fix #206 and #66 (the multiset bug, and "final admission ignores Degraded") by construction
- put `PolicyMismatch` beside the parse-error arm
- turn the "Not doing" row about the admission loop as a machine (C157's one exception) into a computed wake-up

Reviewer question: which of the five holders are one concept? Read `choke.rs:395-540`, `dispatch.rs:80-240`, `gate.rs`, `limiter.rs` (`wait_for`, `observe_impl`), `rails.rs:532-690`, `quote.rs`, rulings C19, C21, C25, C133 and C190, and N6 and P-B. Payoff: three closure concepts become one, an estimated 800–1,500 lines go, and the latent N6 hazard retires.

**H2. Split `rails.rs` by duty, and give each persisted mark one home (confidence medium).**
- `Verdict` and `BlockShape` are a pure classifier and belong with `classify_response`.
- The dead-grant mark is session state. It is keyed by username where C49 wants the uuid.
- The C190 closure is admission state (H1).
- The journal is a recorder.

Daemon state persists across `daemon.db`, `rails.json` (four mark kinds plus preserved unknown fields), `fault.json`, the keyring, `accounts.json` and `sends.jsonl`. Ask: what does each file hold that another does not? Read `rails.rs:100-980`, `decisions/network.md` C24, C83, C129 and C190, and `store/src/world.rs:1000-1050`.

**H3. Make the daemon's lifetime not load-bearing (owner's ruling; confidence low-medium).** C20 probes once per lifetime, the limiter is never persisted, and the idle verdict waits on "limiter cold". So the idle hour, the Life machine's judge, and much of the door's spawn and replace machinery exist partly to keep a warm prediction alive. Persisting the prediction (policy history and holds) beside `rails.json` would let headers keep correcting it (invariant 2). It would also make re-probes, the idle hour's cost (#208's motive) and the spawn race matter less. Read C3, C20, C157, `life.rs`, `server.rs` driver (around lines 650-1000), `client.rs:1245-1600` and #208.

**H4. Protocol surface (confidence high, small payoff).**
- `Dashboard` duplicates `DaemonStatus`, `AuthStatus` and `List`, is unpaged, and has one consumer (`cli/dash.rs:19`).
- Under C10's exact-match handshake, only `hello` and `daemon_stop` ever cross versions. C85's strict-request / lenient-response distinction for the rest is mostly ceremony; say so rather than pin it (#210).
- `SetPriority` and `priority`: one level is in use ("Not doing"), but a verb, an event gap and tests are carried.
- The twin `frame.rs` copies.

**H5. The mock as a second GGG (confidence medium).** 1,156 production lines ship in release `acqd`. The kind table and routes branch on `is_real()` at 8 sites. The policy table (`mockggg.rs:203-226`) is hand-copied from `network-ground-truth.md:409-411`, N39 and N40, and no test reads the ground truth. It does not model Cloudflare blocks (those live in `tests/transport.rs`, a second fake of 1,269 lines). Its windows reset at every restart, so the restore's "requeued" premise is never exercised. Ask: should the table be generated from, or checked against, the ground-truth table, and can the mock-only kinds leave the production kind table? The typed `JobKind` is #100's deferred row.

### 4. Low yield / already covered
- Invariant 5 (`Secret`, keyring): held by types and tests (#217).
- Invariant 1 edges: held by `arch-check` checks 5 and 15 (#216).
- The job "one ending" (#261), restore edges (#209) and the Life machine (#198): freshly reworked.
- Event-order nondeterminism: by design (C85).
- Env knobs: each read at about one site; not the sprawl suspected.

### 5. Blind spots
- Nobody measured the pacing loop's cost. It runs `queue_order()` (a full sort under `shared`) per slice per key (`dispatch.rs:210`). No measurement exists of CPU at idle with parked jobs, or of release binary size including the mock.
- 59 free-form `Refusal::new` strings: whether the words should be typed, as #178 began for "way on", is unexamined.
- Test weight is 1.8 test lines per production line in the daemon. Whether the 4,715-line `ratelimit/tests/spec.rs` duplicates the wire table is unexamined.
- I could not verify Windows pipe paths or run anything; all is static reading.

### 6. Surprises
- `frame.rs`'s doc was edited by a review commit (`4b4abc93`) to assert "byte for byte", a property held by nothing.
- `end()` was typed to take `&mut Shared` while `persist`, ten lines from its call, still relies on a comment.
- C157 says every waiting lifecycle is a machine, yet the choke point's admission loop is explicitly exempted in "Not doing". It is the one most tied to invariant 1.
- `PolicyMismatch`'s asymmetry was called "C19-consistent to fix" by the audit and appears in no issue.

I re-read the code behind the top three claims myself: the gate's `debug_assert!`s (`gate.rs:100-120, 195-262`), `BlockShape` and `Verdict` (`rails.rs:291-384`), and the `PolicyMismatch` arm (`limiter.rs:584-638`). I also re-read the three admission tiers (`choke.rs:395-534`, `dispatch.rs:80-240`) and the spawn race (`door.rs:330-341`, `world.rs:924-968`).
