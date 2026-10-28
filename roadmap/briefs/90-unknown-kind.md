# 90 — a job kind the store cannot place fails its job

## Goal

Today the daemon's `record` (`crates/acquisition-daemon/src/daemon/kinds.rs`)
asks the store where a fetched body belongs through `Endpoint::from_job`
(`crates/acquisition-store/src/lib.rs`), and when the store answers
`None` the arm returns `Ok(())`: the job concludes as a success with
nothing stored. No kind a client can submit reaches that arm on this
tree, so it is a trap for the next kind rather than a live fault; the
path a PoE2 stash endpoint would take on launch day (phase 5) runs
straight into it, and it is the shape C144 names, the daemon reports no
success it did not earn. When this step is done a kind the store cannot
place fails the job in a sentence naming the kind, keeping the fetched
body as the failure's payload like every other store fault after a
send, and a test at the daemon pins it (C122, a behaviour is pinned
once, at the lowest crate that holds it). The issue is #90; the commits
say "closes #90" only on the last.

## Read

- Issue #90: the fault, the one-rule fix, and the wider cause left to
  the typed `JobKind` row (#65's second comment), which is not this
  step.
- `AGENTS.md`: C122 (one pin, at the lowest crate), C47 (malformed
  external input is a structured error), C116 (no id in user-facing
  text), "The gate decides".
- `decisions/daemon.md`: C144, the daemon reports no success it did not
  earn, whose entry names a store that cannot be opened or written after
  a send; its `Pinned` field is the `c144_` tests. C127, the kind table
  and the door: a kind outside `KINDS` is refused at submit, so the kind
  that reaches `record` is a row of the table or a restored row from a
  build with another table (`execute_inner`'s last arm, `kinds.rs`
  line 744).
- `decisions/store.md`: C28, the daemon's part of the store is
  `record(endpoint, params, status, body)` and it never looks inside a
  body; C30, a refused body is evidence, kept in `refused`. An unknown
  kind is neither: the store was never asked, so there is nothing to
  keep but the failure's payload.
- `crates/acquisition-daemon/src/daemon/kinds.rs`: `record` (lines 195
  to 270): the `store_dir` arm (a daemon with no store, line 202), the
  `from_job` arm (line 205, the fault), the `not_recorded` closure
  (line 208, the one shape of a store fault after a send: a log line
  and `Outcome::failure_keeping` with the params and the body), the
  account arm, the open, the write. Its four callers, each a network
  kind (lines 349, 371, 430 and 453: `characters`; `character`,
  `leagues` and `profile`; `stashes`; `stash`), every one a kind
  `from_job` knows. `fetch`, `whoami`, `sleep`, `probe` and `apply`
  never call `record`, so the mock's `fetch` the issue offers as a
  staging kind does not reach the arm either.
- `crates/acquisition-store/src/lib.rs`: `Endpoint::from_job` (lines
  618 to 642), six kinds and a `None` for the rest; its test
  `endpoint_from_job_kinds` (line 4061) pins the `None` for `probe`.
- `crates/acquisition-daemon/src/daemon/tests/dispatch.rs`: the C144
  section (line 1823 on): `schema_too_new`, `kept_failure`,
  `c144_a_store_that_cannot_record_fails_the_job_and_keeps_the_body`
  (a direct call on `record` with a `store_dir` set), and
  `c144_a_job_whose_store_cannot_open_ends_failed_with_its_body` (a
  submitted `profile` through the dispatcher). The new test is the
  first's sibling.
- `crates/acquisition-daemon/src/daemon/tests/harness.rs`: `test_daemon`
  (line 398), `TestPath::dir`, `finish_harness` (line 621).
- `REVIEW-CHECKLIST.md`, "Store and plans" and "Protocol, client, daemon
  lifecycle, the world": "A failure that names no cause" (the failure
  must name the kind and say what the store could not do), "A claim the
  code did not make" (the commit says what the test stages and no
  more).

## Input

No measurement. The failing input the test stages: a direct call on
`record` with a `store_dir` set, an account, a kind the daemon's table
does not hold and the store does not know (a test-only name), and a
body. No kind a client can submit reaches the arm on this tree (see
"Read", the four callers), so the pin is the direct call, as the C144
unit test beside it is; the submit-shaped sibling is not possible
without a test-only kind in `execute_inner`, which would spread the
vocabulary the issue leaves to #65.

## Do

1. **The rule.** In `record`, the `from_job` arm fails the job through
   the `not_recorded` shape: the log line `store: <kind> not recorded:
   ...` and `Outcome::failure_keeping`, the sentence "the response was
   fetched, but the store could not record it: the store knows no place
   for a <kind> response", the params and the body as the payload. The
   arm moves below the closure and runs before the account arm and the
   open, so no account file is created for a kind nothing can be
   recorded under. The `store_dir` arm stays: a daemon with no store
   records nothing by design (the login's and the restore's reads of
   `store_dir` share that reading).
2. **The doc comment** on `record` (the C144 paragraph) gains the
   sentence: a kind the store cannot place fails the job the same way.
   The registry's C144 entry is the owner's; whether its sentence should
   name the unknown kind is raised in the report, not edited.
3. **The pin**, in `dispatch.rs`'s C144 section:
   `c144_a_kind_the_store_cannot_place_fails_the_job_naming_the_kind`:
   a `test_daemon` with a `store_dir` under a `TestPath`, `record`
   called with an account, a test-only kind and a body; the result is a
   failure whose sentence names the kind and says the store could not
   record it, whose payload is the params and the body, and the
   account's store file does not exist after the call. The test rides
   in the commit of the fix.
4. `cargo build --workspace`, then the gate; one commit, its message
   naming this brief.

## Done when

- `record` returns a failure, never `Ok(())`, for a kind `from_job`
  returns `None` for, when a `store_dir` is set; the sentence names the
  kind; the payload holds the params and the body.
- A test in `dispatch.rs`'s C144 section stages a test-only kind through
  `record` and asserts the three facts above and that no store file was
  created.
- `record`'s doc comment states the rule under C144.
- The gate is green on the step's tree, each stage by its own exit
  status.

## Not in scope

- The typed `JobKind`, one vocabulary for the kinds spread over
  `KINDS`, `route_for`, `kind_needs_account`, `sends_unrouted`,
  `NO_PROBE_ROUTES`, `admit_realm`, the apply vocabulary, the store's
  `from_job` and the protocol's `target_of`: #65's second comment.
- A kind added to `execute_inner` or `KINDS` for the test's sake: it
  would widen the vocabulary the step leaves alone.
- The `store_dir` arm: a daemon without a store is a configuration, not
  a fault.
- The C144 registry sentence: the owner's, raised in the report.

## Rules

`TEMPLATE.md`. The step runs in the main tree, one session; kill only
the processes you started.

## Report

`TEMPLATE.md`.
