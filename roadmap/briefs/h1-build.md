# h1 — The daemon reports no success it did not earn

The first of three groups picked from the open-issue inventory of
2026-10-25, outside the roadmap's phase order at the owner's word
("Let's do this"). Not a roadmap phase step: no ledger line, but the
rhythm of "A step" (`roadmap/README.md`) holds whole: brief, build,
gate, commit, review, the owner's reading, close.

## Goal

Four issues share one fault: the daemon, or a frontend speaking for it,
reports a success the tree does not hold. A client's profile job can
run on a login's unproven staged token and write its uuid into the
session (#71). Logout and forget tell the user the keyring was cleared
when it was not, and a renamed account leaves an entry nothing can
reach (#61). A store that cannot be written after a send is logged and
the job reported done, and the account index is a torn-write risk on
four of five paths (#63). A present key of the wrong type reads as an
empty list and retires every item at the tab (#62). When this step is
done, each of those is refused or reported in its own words, under
C144 (the daemon reports no success it did not earn) and C145 (a
present key of the wrong type is malformed; an absent key is empty),
and a test pins each.

## Read

- `AGENTS.md`: the invariants, "Hard rules", C47 (structured errors,
  no panics on input), C84 (the artifact and contract identity: a
  protocol change moves the contract revision, which is expected here),
  C122 (one pin per behaviour, at the lowest crate that holds it).
- `decisions/daemon.md`: C144 (this step's ruling), C50 (the uuid at
  login, the lost login failed at restore), C133 (the one exit and its
  15 s for a pending login), C136 (the submitter `daemon` is
  unforgeable), C35 in `decisions/store.md` (annotations are the only
  irreplaceable state; nothing here touches intent).
- `decisions/store.md`: C145 (this step's ruling), C29 (items lifted
  at the seams), C30 (a refused body is evidence, kept in `refused`).
- The four issues, read whole with `gh issue view <n>`: #71, #61, #63,
  #62, and #59 for its first part. Their line numbers are from
  2026-10-22 and 2026-10-23; re-find each site in the tree before
  relying on it.
- `REVIEW-CHECKLIST.md`, the sections "Network: the limiter, the gate,
  OAuth", "Store and plans" and "Protocol, client, daemon lifecycle,
  the world".
- The code: `crates/acquisition-daemon/src/daemon/login.rs`
  (`staged_profile_token`, `complete_login`, `abort_login`, the
  `Staging` struct, `logout`, `forget_account`), `daemon/kinds.rs`
  (`render_then_authenticate`, `record`, the index writes),
  `daemon/refresh.rs` (the rename and the save under `shared`),
  `daemon/start.rs` (restore, `auth::keyring_load`), `daemon.rs` (the
  error table around line 387, the `CredentialStore` trait around line
  894, the doc comment around line 477), `daemon/server.rs` (the
  logout answer around line 342); `crates/acquisition-store/src/index.rs`
  (`save`, line 245), `lib.rs` (the seams the issue #62 cites);
  `crates/acquisition-protocol/src/job.rs` (`Outcome`, line 121).
- The tests that assert on the words this step changes, found by
  grepping for "keyring cleared": `crates/acquisition-cli/src/main.rs`
  line 1059 prints them, and three goldens carry them,
  `crates/acquisition-cli/tests/goldens/seat_faults/a_stop_names_the_logins_it_ended.txt`,
  `goldens/journey/a_first_session.txt`,
  `goldens/new_league/launch_morning.txt`, read by `seat_faults.rs`
  and their own test files. Grep again before the build for any the
  tree has added.
- The daemon's in-crate tests under
  `crates/acquisition-daemon/src/daemon/tests/` (`login.rs`,
  `sessions.rs`, `bearer.rs`, `refresh.rs`, `harness.rs`) for the
  harness that stages a credential store and a login.

## Input

- The two rulings, C144 and C145, in the registry at this brief's
  commit, with the owner's words in that commit's message: "Fail the
  job, keep the body" and "Absent is empty, wrong type is malformed".
- The failing inputs the tests stage, each named here:
  1. A client `profile` job for the staged username submitted during
     the window between the token exchange and `complete_login`.
  2. A credential store whose `clear` fails under logout, and under
     forget; a store whose `save` fails under a refresh that renames
     the account; a store whose `load` fails at restore.
  3. A facts file that cannot be opened at `record` (a schema newer
     than the daemon's, `SchemaTooNew`, is the shape the issue names),
     and a write that fails after the open.
  4. Two accounts whose index writes interleave, and a crash between
     the temp write and the rename.
  5. A 2xx stash body whose `items` is an object; one whose `children`
     is a string; a character body whose `socketedItems` is a number
     two levels down; a `leagues` body whose `leagues` is an object;
     a stash body whose `stash.id` is not the id the params name.
- No measurement: nothing here is a performance change.

## Do

1. **#71, the staged token.** Give `Staging` the login's own profile
   job id, set at the submit that names `DAEMON_SUBMITTER`, and make
   `staged_profile_token` answer only for that job. A client's
   `profile` job for the same account in the window runs on the
   account's session, or fails `not_logged_in` when there is none.
   Correct the two doc comments (`login.rs` around line 136,
   `daemon.rs` around line 477) to say what the code now does, headed
   by C144. Pin it in the daemon's in-crate tests.
2. **#59 part 1 on the same field.** With the profile job id on
   `Staging`, count a login as outstanding in the drain while its
   profile job is terminal and `staging` is not yet cleared, so a stop
   that lands in that gap waits for `complete_login` or `abort_login`
   under C133's 15 s. Pin by forcing the gap, never by timing (C137).
   **#59 part 2**: at restore, a `profile` job whose submitter is the
   daemon fails as a lost login's (C50 already says so) and sends
   nothing; pin it.
3. **#61, the credential store.** Add `load` to `CredentialStore` and
   route restore through it, so a test can stage a store that fails at
   any of the three. Every result reaches the caller: `logout` answers
   the clear's failure on the wire (the error-table row changes under
   C144; rewrite the row), and the CLI's words say what happened,
   "logged out (session dropped; keyring clear failed: …)" or the like,
   with `--json` carrying a `keyring` field either way; `forget` clears
   whether or not the entry is marked persisted and logs the result,
   not its hope; a refresh that renames clears the old name's entry,
   and a failed save clears the entry it would have replaced. Drop or
   redact the `Debug` derive on `TokenResponse`. Pin each with a staged
   store. Regenerate the goldens that carry the old words under
   `ACQ_UPDATE_FIXTURES=1` and read every changed line as the diff of
   what a user now reads; name the facts in the test.
4. **#63, the store failure and the index.** In `record`, an
   unopenable or unwritable store after the send is `Outcome::Failure`,
   not `Ok`; add an optional `payload` to `Outcome::Failure` in the
   protocol crate (`#[serde(default, skip_serializing_if)]`, so an old
   failure reads unchanged) carrying the fetched body, and have the
   failure's sentence name the store fault. Pin the wire shape in
   `acquisition-protocol/tests/wire.rs` (a failure with a payload
   round-trips; one without serialises as before) and the behaviour in
   the daemon's tests. The contract revision moves; that is C84
   working, not a finding. The CLI and MCP renderers show a failed
   job's payload where they show a done job's, or say in one line that
   the body was kept; check `acq result` and `job_result`. Make
   `Index::save` write to a unique temp name, `sync_all` the file and
   the directory, then rename; take `shared` on every index write
   (refresh, the profile job, logout, forget), as login does. Open the
   facts writers' transactions `IMMEDIATE` as the annotations writer
   does. Pin the torn write with a crash staged between write and
   rename, and the interleave with two accounts. Leave the hygiene
   list's remaining rows on the issue (the unparsable annotation row
   is C35's and the owner's; `daemon.db`'s `user_version` is a schema
   question): the close says "part of #63" with what stays.
5. **#62, the seams.** Under C145, at every seam the issue cites a
   present key of the wrong type is `MalformedBody`; the id check
   recurses into `socketedItems` as the ingest does; a failed `INSERT
   INTO refused` still returns `MalformedBody` naming that the evidence
   could not be kept; a stash body's `stash.id` is checked against the
   params' location. Before relying on "absent is empty", grep the
   committed captures and the mock fixture for a body with an absent
   `items` or `children` key, and say in the report what you found.
   Pin each input of the list above in the store's tests.
6. Fill *Pinned:* on C144 and C145 with the test files and names, and
   head each mechanism's doc comment with its id.
7. One commit per concern, in the order above or any that compiles at
   each step; the gate before each; the messages name this brief and
   close or part-close their issues ("Closes #71", "Closes #61",
   "Closes #62", "Closes #59", "part of #63").

## Done when

- `acquisition-daemon`'s tests pin: a client profile job in the staging
  window never receives the staged token; a stop in the gap waits for
  the login; a restored daemon-submitted profile job fails and sends
  nothing; a failing clear, save and load each reach the caller and the
  words; a rename and a failed save leave no orphaned entry.
- `acquisition-protocol/tests/wire.rs` pins the failure payload's shape.
- `acquisition-store`'s tests pin each of the five malformed inputs as
  `MalformedBody` with the body kept in `refused`, the recursive id
  check, the location cross-check, the atomic index write and the
  interleave.
- The CLI's logout words and `--json` carry the keyring's outcome; the
  changed goldens are read and named in the commit.
- The two doc comments at `login.rs` and `daemon.rs` state what the code
  does; the error table's logout row states the new rule.
- C144 and C145 carry their *Pinned:* lines.
- Every `acq` the build runs is in a mock world.
- The gate green on the step's own tree, every stage's exit status
  shown, and `git diff --check` clean.

## Not in scope

- Anything that adds a send to GGG. Nothing here does; a change that
  would is brought back, not built.
- The lock merge, the typed `JobKind`, or any architecture row from the
  roadmap's "Open" list; the keyring save under `shared` stays where it
  is.
- #55 (the access token in the keyring), #68 (the login callback),
  #69 (the client's doors): the next groups'.
- A `deny_unknown_fields` on `Request` or any protocol change beyond the
  optional failure payload (#56, the owner's).
- The unparsable annotation row's repair (#63's hygiene list; C35 and
  the owner's).
- The CLI wording outside logout and the failed job's payload line.

## Rules

`TEMPLATE.md`, with these differences:

- This step runs alone in the main tree, `/Users/tom/Development/GitHub/smallcatlabs/acquisition`,
  on branch `main`, never a worktree; the owner asked for the groups in
  series. Nothing else builds there while you work. Your scratch is
  `/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/760d4dea-22c5-4f55-b274-679afa47172f/scratchpad/h1/`;
  make it; every `ACQ_STORE_DIR` is under it.
- Commit on `main`; never push. End each message with the line
  `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`; the
  commit-msg hook rewrites it as the project's `Assisted-by:` trailer.
- Run commands likely to take more than two minutes in the background
  and wait on them; `cargo build --workspace` before any test that
  starts a daemon, and again after any test run before another (C84).

## Report

`TEMPLATE.md`. Return it as text; you cannot write a report file the
launching session reads.
