# 56-list-paging — the job list paged, every answer within its frame

Issue #56's wire items, built at the owner's word of 2026-11-05 ("Yes,
build #56"), on the rulings he entered that day: C166 and C85's
unknown-field sentence (`decisions/daemon.md`, at `ddef4980`). Written
by the orchestrating session against the tree at `ddef4980`. Daemon and
protocol: an Opus worker.

## Goal

No answer the daemon writes can pass the frame bound. Today `List`
answers every job this daemon holds and every finished row of its
lifetime in one frame. One apply admits up to `APPLY_MAX_ACTIONS`
(50,000) jobs, and a job's params may run to `PARAMS_MAX_BYTES` (8 MiB),
so the list can pass `MAX_FRAME_BYTES` (64 MiB). The client then
discards it as oversize, and no single request is to blame. When this
step is done:

- `List` answers one page of the state asked for, `open` by default.
- Every surface that shows the queue reads it in pages.
- A `Quote` is bounded at the door.
- A response that would still pass the bound is answered as `internal`.
- Every versioned request refuses a field it does not know.

## Read

- `AGENTS.md`.
- `decisions/daemon.md`:
  - C166, the ruling this step builds.
  - C85: the frame bound; `hello` and `daemon_stop` are the stable
    plane that ignores unknown fields, and every other request refuses
    one.
  - C6: the list is this daemon's own jobs, its memory and its
    lifetime's finished rows.
  - C10: a single-version handshake, so no other variant meets a peer
    of another version.
  - C47, C165.
- `decisions/frontends.md`:
  - C149: every answer is one object.
  - C53: the read economy, ten rows and `expand`.
  - C146: a count says what it counts.
  - C143: a shared text speaks the request's words, never a flag.
- `REVIEW-CHECKLIST.md`: the protocol/daemon section and the frontends
  section ("A document unbounded by default"; "A document's producer
  moved without its readers listed").
- The protocol:
  - `crates/acquisition-protocol/src/protocol.rs`: `Request` (line 430,
    internally tagged `req`), `Request::List` (line 455) and
    `Response::Jobs` (line 559).
  - `Bootstrap` (line 94): `hello` and `daemon_stop`, parsed before
    `Request`.
  - `MAX_FRAME_BYTES` (line 80).
  - The `deny_unknown_fields` already on the `QuoteJob` and `Quote`
    structs (lines 333, 350, 381).
  - The fixture `crates/acquisition-protocol/tests/fixtures/wire/request/list.json`
    and `tests/wire.rs`'s `c85_` tests.
- The daemon:
  - `crates/acquisition-daemon/src/daemon/queue.rs`: `job_list` (line
    427), whose doc already says how memory and the table's pages are
    read without loss; `LIST_PAGE`, `PARAMS_MAX_BYTES` and
    `APPLY_MAX_ACTIONS` (lines 75, 1152, 1169).
  - `daemon/server.rs`: the `Request::List` and `Request::Quote` arms
    (lines 349 and 405), and `write_line` with `UNSERIALIZABLE` (line
    1190 on).
  - `daemon/quote.rs`: `quote`.
  - `daemon/kinds.rs` near line 1105: how an apply's count bound
    refuses.
  - The daemon tests that call `Request::List`: `daemon/tests/queue.rs`
    (its `list` helper, line 88), `exit.rs`, `dispatch.rs`,
    `parents.rs`, `quote.rs`.
- The readers of the list. Each must keep its answer; name each in the
  map.
  - `crates/acquisition-verbs/src/jobs.rs`:
    - `Jobs` / `Listed` / `JobCounts` (lines 151–260): `JobCounts::of`
      counts every job before the cut, and the unexpanded document
      shows ten open and ten finished.
    - `Frontend::jobs` (line 767).
    - `tally`, a parent's children counted from the whole list (line
      102).
  - `crates/acquisition-verbs/src/refresh.rs`:
    - `list` (line 709).
    - Its use at line 635: the queued fetches a plan's spend counts,
      open jobs.
  - `crates/acquisition-cli/src/main.rs`:
    - `Cmd::Jobs` with `--watch` and `--all` (line 555).
    - `list` (line 2762), the watch's snapshot.
  - `crates/acquisition-mcp/src/main.rs`: `list_jobs` (line 2209), its
    `expand`, and its tool text (lines 717–729).
  - `crates/acquisition-cli/src/web/mod.rs`: `jobs` (line 1052).
  - The client harness's fake daemon
    (`crates/acquisition-client/tests/harness/mod.rs`),
    `acquisition-client/tests/contract.rs`, and the CLI and MCP tests
    that list jobs (`grep -rln 'Request::List\|"list"' crates/*/tests`:
    `price_json.rs`, `seat_faults.rs`, `pricing_tools.rs`, the web
    golden `refresh_loop.txt`).

## Input

The bounds on the tree at `ddef4980`:

| Bound | Value | Where |
| --- | --- | --- |
| `MAX_FRAME_BYTES` | 64 MiB (`64 << 20`) | the protocol |
| `PARAMS_MAX_BYTES` | 8 MiB (`8 << 20`) | the daemon's queue |
| `APPLY_MAX_ACTIONS` | 50,000 | the daemon's queue |

A page capped by a row count alone does not fit by construction: 100
rows of 8 MiB params is 800 MiB. So the page's cap is in bytes as well
as rows (Do 2).

The last push's CI: `Gate (Linux)` run 42033006254 and `Gate (Windows)`
run 42033006320 on `ddef4980`, in progress when this brief was written.
Read both again at the start and name their conclusions in the report;
a red one is reported, not fixed here.

## Do

1. **The map**, in the first commit's message: every reader of
   `Request::List` from the Read list, and what each needs from the
   list (a page, every open job, a parent's children, counts).

2. **The request and the answer.**
   - `Request::List` takes three fields, each defaulted so that
     `{"req":"list"}` still parses as the first page of open jobs:
     - `state`: `open` (default), `finished` or `all`;
     - `limit`: rows per page, default 100, at most a named maximum;
     - `after`: the job id after which to resume.
   - Pages run newest first by job id; `after` resumes below that id.
   - `finished` keeps C6's scope: this daemon's finished rows and what
     its restore brought back, as `job_list` reads them now.
   - A page holds at most `limit` rows and at most the bytes that keep
     its serialized answer within `MAX_FRAME_BYTES`, with headroom for
     the envelope. The byte budget is a named constant, its reason in
     its doc. A page always carries at least one row when any remain;
     one row fits, since `PARAMS_MAX_BYTES` is under the frame.
   - `Response::Jobs` carries `jobs` (the page), `next` (the id to pass
     as `after`, absent on the last page) and the counts the frontends
     print today, made by the daemon over every job of this daemon's
     list whatever the state asked:
     - `total`;
     - by state: running, waiting, done, failed, cancelled;
     - the soonest and the latest predicted start among the waiting.

     This is the session's design inside C166, which says nothing of
     counts: a page cannot count what it leaves out. List it in the
     close for the owner. The finished counts come from the table by a
     count query under the per-page lock rule, never by reading the
     rows.
   - `job_list`'s doc keeps its no-loss argument and adds the paging
     argument: ids are stable and never reused, so `all` paged by
     `after` lists no job twice and skips none that existed at the
     first page. Under `open`, a job that finishes between pages leaves
     the list.
   - Pin in the daemon's tests (C153: the existing list cases gain
     these, no new file):
     - each state;
     - newest first;
     - `after` resuming;
     - a page cut by its byte budget before its `limit`, with stated
       params sizes;
     - the counts over jobs the page leaves out;
     - `{"req":"list"}` answering open jobs.

     The wire shape is pinned in `acquisition-protocol/tests/wire.rs`,
     with the request fixture and a response fixture.

3. **Every reader pages.**
   - The verbs crate owns a page loop: one function that reads every
     page of a state, used by every reader that needs the whole list.
     These are `tally`, refresh's queued fetches (state `open`) and
     `expand`.
   - The unexpanded document asks for two first pages: `open` and
     `finished`, ten rows each. `JobCounts` then comes from the
     answer's counts, not from counting rows.
   - The ten shown are the newest. Change the words that say "the first
     ten open jobs by id" (the clap help, the MCP tool text) to say what
     is now shown. This is agent-facing design and the session's call;
     list it in the close.
   - `tally` may read only what it needs if a narrower read stands
     without a new request field. No new request field beyond C166's
     three.

4. **A state on each surface.**
   - `acq jobs` gains `--state open|finished|all`.
   - The MCP's `list_jobs` gains `state`.
   - Each surface's default view stays what it shows today, open and
     finished both; `state` narrows it. `--all` and `expand` keep
     meaning every row, of the chosen state, read page by page.
   - The web jobs page shows its view unchanged (no new route or
     control: C138, the owner's).
   - The MCP's `list_jobs` with `expand` over a very large queue
     answers whatever size it reaches today. Say it in the report under
     "Raised", with a number: the bytes a 50,000-row expand answer
     would take at a stated row size. Do not bound it here.

5. **A `Quote` bounded at the door.**
   - `Request::Quote` with more jobs than `APPLY_MAX_ACTIONS` is
     refused before any work, in the words and kind an apply over its
     bound gets (`kinds.rs`'s refusal; one function if the two can
     share it, C153).
   - Pin it in `daemon/tests/quote.rs` beside the existing quote
     cases, at the bound and one over.

6. **No answer written past the bound.**
   - `write_line` writes a response whose serialized line passes
     `MAX_FRAME_BYTES` as an `internal` error in its place. The
     constant beside `UNSERIALIZABLE` says the answer was too large to
     send and that the connection stays open.
   - The connection is kept.
   - Pin it as a unit test of the writer beside the unserializable
     test, with a bound the test can reach. Pass the bound in, or pin
     the decision function. Never write 64 MiB in a test.

7. **Unknown fields refused on every versioned request** (C85).
   - Every `Request` variant refuses a field it does not know, as
     `bad_request`, in words that name the field, through the path a
     malformed frame takes today.
   - `Bootstrap` (`hello`, `daemon_stop`) keeps ignoring one.
   - Check that serde's container-level `deny_unknown_fields` on an
     internally tagged enum refuses an unknown field and still accepts
     the `req` tag. If it does not, say what you used instead.
   - Pin it in `wire.rs`'s `c85_` tests: a list with a misspelled
     field refused, a hello with an extra field accepted.

8. **References and goldens.** Regenerate `CLI-REFERENCE.md`,
   `MCP-REFERENCE.md` and every golden that moves under
   `ACQ_UPDATE_FIXTURES=1`, and read every changed line (shape 9). The
   README gains nothing unless the tour runs `acq jobs`; if it does,
   the tour line stays true.

## Done when

- `{"req":"list"}` answers the newest page of open jobs.
- `state`, `limit` and `after` page every job of a state once, newest
  first.
- A page stops at its byte budget before its row limit.
- The answer carries `next` and the counts over all of this daemon's
  jobs.
- All of the above are pinned in the daemon's tests and the wire's.
- Every reader of `List` reads through the verbs crate's page loop or
  one first page, as the map names it. `acq jobs`, `list_jobs` and the
  web jobs page show what they showed, with the newest ten in place of
  the first ten. `--state` and `state` narrow them.
- A `Quote` over `APPLY_MAX_ACTIONS` jobs is refused before any work;
  pinned at the bound and one over.
- `write_line` answers a line past the bound as `internal` and keeps
  the connection; pinned.
- A `Request` with an unknown field is refused as `bad_request`, and a
  `hello` with one is accepted; pinned in `wire.rs`.
- Both references and every moved golden are regenerated and read.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- `Bootstrap`'s shape.
- The frame bound's value.
- `PARAMS_MAX_BYTES` and `APPLY_MAX_ACTIONS`.
- A request field beyond C166's three.
- Retention of finished rows.
- A new web route or control.
- Bounding the MCP's `expand` answer.
- `read_frame` (#56's internal items, built 2026-11-04).

## Rules

`TEMPLATE.md`, plus:

- Work in the main checkout on `main`. Commit locally; do not push (the
  orchestrator pushes).
- **This is the owner's machine, with his real store and keyring.
  Never start a real-provider daemon, and never run a bare `acq`.**
  Every `acq` and `acq-mcp` runs under the mock-session skill, with
  `ACQ_PROVIDER=mock` and `ACQ_STORE_DIR` under your scratch.
- C153, C154.
- A wait in a daemon test is forced (C137, C160), never a sleep.
- One commit per Do item or a logical pair, each naming this brief.
- Run the gate in the foreground, bare. Read its exit status alone,
  never through a pipe or `tail`.

## Report

`TEMPLATE.md`, plus:

- The map.
- The session's design calls: the counts, the newest-ten wording, the
  surfaces' `state`.
- The byte budget and its reason.
- The MCP `expand` number under "Raised".
