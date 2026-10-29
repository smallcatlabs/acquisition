# 85 — generative input for the rate-limit header parse and the IPC frame decode

Issue #85. The first item of "Pull-forward" (`roadmap/README.md`), after
the league group's close at `c308cf0d`. The rhythm is "A step". This
brief stops at Do 1: the step needs a ruling it does not have, and the
build waits for the owner's word.

## Goal

The two parsers that read input nobody in the workspace controls are
driven by generated input, not only by the tables someone wrote: the
`X-Rate-Limit-*` and `Retry-After` parse (upstream input, a GGG
response; `ratelimit/policy.rs`), and the frame reader with the decode
after it (client input, any local process that reaches the socket;
`frame.rs`, then the protocol's `Bootstrap::read` and `Request`). What a
generated input may not do is the property, stated once: no panic, in
the parse or in the limiter's arithmetic that consumes what the parse
accepted (`REVIEW-CHECKLIST.md`, "A parsed value that overflows"); no
policy accepted outside the caps its constants name; no frame longer
than its bound, and the two copies of the reader reading every stream
alike. What a run finds is fixed by one rule and pinned as an ordinary
test in the daemon or the client crate (C122, a behaviour pinned once at
the lowest crate that holds it), or filed.

## Read

- `AGENTS.md`: invariant 2 (headers are the truth), C1 (the workspace's
  edges; `tools/arch-check.sh:289`, `only_self acquisition-daemon`, "no
  package but acquisition-daemon names the daemon", and the same words
  in `crates/acquisition-daemon/Cargo.toml:17`), C47 (malformed external
  input is a structured error, never a panic; the daemon crate has no
  `unwrap_used` lint), C122.
- `roadmap/STATE-2026-10-26.md:65`, R5: the evidence ("`ratelimit/policy.rs`,
  which has no generative test"). Issue #85 (`gh issue view 85 --json
  body,comments`; no comments): "`cargo-fuzz` targets for both, run on a
  schedule, not in the gate. The query language already has proptest
  generators."
- `decisions/network.md`: C17 (the custom policy layer over GGG's
  headers), C18 (state learned only from headers), C19 (a counted send
  with unusable headers paced as counted), C33 (the limiter's spec is
  test tables). `decisions/daemon.md`: C85 (one request in flight per
  connection; the frame is its unit), C10 (the hello before anything).
- `REVIEW-CHECKLIST.md:15`, "A parsed value that overflows": every
  header field that enters arithmetic bounded where it is read; the
  `spec.rs` tables hold it today.
- `TESTING-NOTES.md`: the journal is the GGG-side contract surface;
  nothing here sends, so no journal is involved.
- The code, all of it already `pub` (nothing needs exposing):
  - `crates/acquisition-daemon/src/ratelimit/policy.rs`: `Policy::parse`
    (59), `parse_integer` (207), `parse_retry_after` (271),
    `parse_triplet` (299), `parse_limits` (318), `parse_state` (346);
    the caps `RETRY_AFTER_CAP_SECS`, `MAX_POLICY_PERIOD_SECS`,
    `MAX_POLICY_RESTRICTION_SECS`. Re-exported by `ratelimit.rs:52`.
  - `crates/acquisition-daemon/src/ratelimit/limiter.rs`: `Limiter::observe`
    (498), `observe_probe_429` (412), `wait_for` (211), `eta_for` (227),
    `statuses` (737), `project` (799): where an accepted value meets
    `Instant` and `Duration` arithmetic.
  - `crates/acquisition-daemon/src/ratelimit/tests/spec.rs`: the tables
    the seeds come from, `n5_header_contract_parses` (141),
    `retry_after_is_independent_of_full_policy_validity` (180),
    `d8_full_header_grammar_rejects_every_partial_or_malformed_shape`
    (217), and the pacing rows.
  - `crates/acquisition-daemon/src/frame.rs` and
    `crates/acquisition-client/src/frame.rs`: identical below their
    module doc's second paragraph (`diff` shows only that paragraph);
    their tests `a_line_over_the_bound_is_discarded_through_its_newline`
    (79) and `an_oversize_line_cut_off_by_the_close_is_a_close` (107).
    Readers: the daemon's server (`daemon/server.rs:118`, then
    `Bootstrap::read`, then `serde_json::from_slice::<Request>` at 208),
    the client (`client.rs:1122` and 1817, then `Response`).
  - `crates/acquisition-protocol/src/protocol.rs`: `MAX_FRAME_BYTES`
    (94, 64 MiB), `Bootstrap::read` (134), `Request` (437), `Response`
    (543).
  - `crates/acquisition-search/tests/common/generated.rs:1100`, `cases`
    and `config`: the workspace's one shape for a generated property, a
    local count and a full count under `ACQ_CASES=full`, a fresh printed
    seed each run; `.github/workflows/gate-linux.yml` sets
    `ACQ_CASES=full` on CI. proptest 1.11.0 is a dev-dependency of the
    plan and the search.
  - `.github/workflows/`: `gate-linux.yml`, `gate-windows.yml`,
    `release.yml`; no scheduled workflow exists. `rust-toolchain.toml`
    pins 1.99.0 stable, "nothing floats"; no nightly is installed here
    (`rustup toolchain list`), nor `cargo-fuzz`.

## Input

No measurement is fixed before the work: the step's measurement is its
own run (Do 6). The seeds are fixed: the header sets of `spec.rs`'s
tables at this brief's commit, and the two frame tests' streams.

## Do

1. **The ruling** (the owner's; recorded in `decisions/daemon.md` in its
   own docs commit, quoting him, before any code). `cargo-fuzz` needs a
   package of its own that names the code it drives: a `fuzz/` package
   naming `acquisition-daemon` for the header parse and the limiter, and
   `acquisition-client` for the client's copy of the reader. Today no
   package but the daemon names the daemon (C1's edge, held by
   `only_self` and the daemon's manifest), and the gate's toolchain is
   one pinned stable. Two shapes meet the goal:
   - **(a) Generated properties in the crates' own tests**
     (recommended). proptest, the shape the plan and the search already
     use, as a dev-dependency of the daemon and the client; the
     properties live beside the tables they extend (`spec.rs`,
     `frame.rs`'s tests) and run in the gate at a small count and on CI
     at the full one through `ACQ_CASES`, with a printed seed. No new
     package, no edge, no second toolchain, no new workflow: the
     smallest change that holds the property, and a case that fails
     is already an ordinary test at the lowest crate. It departs from
     the issue's words twice: proptest rather than `cargo-fuzz`, and in
     the gate at a small count (its cost the build measures) rather
     than only on a schedule. What it gives up is coverage-guided
     search; the grammars are a colon triplet and a newline, small
     enough that structured generation aimed at the boundaries (the
     caps, `u32::MAX`, `u64::MAX`, the bound plus one) reaches what a
     coverage-guided byte search would.
   - **(b) `cargo-fuzz`, as the issue says.** A `fuzz/` package outside
     the workspace (its own `[workspace]`, its own `Cargo.lock`), built
     by a weekly scheduled workflow on a pinned dated nightly and never
     by the gate or a release. The ruling it needs, as a draft entry:
     "**C<next> — The fuzz package stands outside the workspace and may
     name the daemon and the client as libraries.** It is built only
     by its scheduled workflow, never by the gate, a release or a
     frontend, on a pinned nightly of its own. *Why:* a fuzz target
     must link the parser it drives, and C13 (never in-process with the
     daemon) is about frontends, which this package is not. *Pinned:*
     `tools/arch-check.sh` (`only_self` reads workspace members)."
     Beside it, the daemon manifest's sentence and `only_self`'s reason
     say "no workspace member". What it costs: the nightly pin to keep,
     a lockfile `cargo-deny` and #86's Dependabot do not see unless
     named, a workflow, and a finding that still has to be rewritten as
     an ordinary test.
   - Not offered: moving the header grammar into the protocol crate, or
     deduplicating the two `frame.rs` copies. Each is a C1 change for a
     test's convenience.

   The rest of this list is written for (a). Under (b), Do 2 to 4 become
   two `fuzz_targets/` with the same assertions, Do 5 the scheduled
   workflow, and the seeds are files under `fuzz/seeds/<target>/`.

2. **The header property** (`ratelimit/tests/spec.rs`, the daemon's
   dev-dependencies). A generator of header sets built from the tables'
   rows and mutated at the places the grammar branches: a rule list
   with empty, repeated, mixed-case and whitespace-padded names; a
   triplet field drawn from `0`, `1`, each cap and the cap plus one,
   `u32::MAX` and one past it, `u64::MAX` and a twenty-digit run, a sign,
   a non-digit, an empty field; a field count of two to four; a missing
   or extra state header; a `Retry-After` drawn the same way. For each:
   `Policy::parse` and `parse_retry_after` return without panic; an
   `Ok` policy holds every window inside the caps the parse claims
   (`max_hits` and `period_secs` above zero, period and restriction at
   most their maxima, limits and state of one length with equal periods);
   then a fresh `Limiter` observes it (`observe`, counted and not, and
   `observe_probe_429` with the drawn `Retry-After`) at an `Instant` and
   is read by `wait_for`, `eta_for` at a drawn `ahead`, `statuses` and
   `project` at that instant and at one a drawn duration later, none
   panicking. The count follows `generated.rs`'s shape: a local count,
   a full one under `ACQ_CASES=full`, the seed printed. Where the search
   crate's `config` is not reachable from the daemon (it is that
   crate's test module), the property carries the same few lines and
   says whose shape they copy.
3. **The frame property** (`frame.rs`'s tests, both copies, which stay
   identical). A generator of streams: chunks of a JSON request,
   arbitrary bytes, newlines, runs of the bound's length and one more,
   no final newline, empty lines; a read capacity of 1 to 64 and a
   bound of 0 to 64. Read to the close: every `Line` is at most the
   bound and holds no newline, the reading ends, and the frames match a
   reference split of the same bytes (split on newline, a piece over the
   bound is `Oversize`, an unterminated last piece is a `Line` if within
   the bound and not empty, else the close). The reference is a few
   lines in the test, not a third reader. Each `Line` then goes through
   `Bootstrap::read` and `serde_json::from_slice::<Request>` (the
   daemon's copy) or `::<Response>` (the client's) without panic. The
   two copies' agreement holds because the property is the same text in
   both and each is held to the same reference.
4. **The seeds.** The generator of Do 2 starts from the header sets of
   `n5_header_contract_parses`, `d8_…`'s rejected shapes and the pacing
   rows, taken as constants the tables already declare where they are
   shared (`CHAR_LIST`) and copied where they are inline; Do 3's from
   the two frame tests' streams.
5. **What a run found.** Each failing case shrunk by proptest becomes a
   named ordinary test beside the table it extends, in the commit of
   its fix, fixed by one rule (`roadmap/README.md`, "A step", 4); one
   that is not this step's to fix is filed as an issue with the case.
6. **The measurement.** Each property run alone at a long count by hand
   (`PROPTEST_CASES`, e.g. 1,000,000, or a bounded wall time), on this
   machine, after `cargo build --workspace`: the count, the time and what
   it found go in the commit message of the property and in the report.
7. The full gate (`tools/gate.sh`) after `cargo build --workspace`, in
   the background, output under the session's scratch; one commit per
   concern (the header property, the frame property, each finding's
   fix); then "Current step" as built, review pending, and the Codex
   review over the range.

## Done when

- `decisions/daemon.md` holds the owner's ruling on Do 1, quoted in the
  commit that lands it, before any code commit.
- Under (a): a generated property over the header parse and the
  limiter reads in `spec.rs`, and one over the frame reader in both
  `frame.rs` copies, each with a local count, a full count under
  `ACQ_CASES=full`, and a printed seed; the two copies of `frame.rs`
  differ only in their module doc's paragraph (`diff`).
- Each property's commit message states its long run: the cases, the
  time, and what it found.
- Every case a run found is a named test in the daemon or the client
  crate, green, in the commit of its fix, or an issue named in the
  close.
- `tools/arch-check.sh` is unchanged under (a); the gate is green on the
  step's tree, each stage by its own exit status.

## Not in scope

- `cargo-mutants` over `ratelimit/` and `gate.rs`, Dependabot, the
  scheduled gate run and the macOS keyring round trip: #86.
- The token response's fields (`expires_in`; checklist row 15's second
  half): they have their table test and are not the issue's two parsers.
- A change to the parse's grammar or to what the limiter does with an
  accepted value, except as one rule a finding needs.
- Deduplicating the two `frame.rs` copies, or moving either parser into
  the protocol crate: C1, a ruling of its own.

## Rules

`TEMPLATE.md`. The step runs in the main tree on `main`, one session;
scratch under this session's scratchpad in a subdirectory named `85`.
Kill only the processes you started. No `acq` runs; nothing sends to
GGG. Never push.

## Report

`TEMPLATE.md`.
