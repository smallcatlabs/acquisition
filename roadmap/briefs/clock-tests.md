# clock-tests — the tests that hang on the clock (#127, #76, #111)

Issues #127, #76 and #111, opened by landing #128's ruling, which is
this step's concern 0. Chosen by the orchestrating session as the
first of the pre-league work: on 2026-10-29 #109's age flake, the same
class, turned CI red three times and cost the most time of the day.
The rhythm is "A step" (`roadmap/README.md`). No ruling beyond #128's
is needed: nothing here changes a boundary, the real store or the
real API.

## Goal

No golden and no assertion in the CLI, MCP, verbs and store tests
depends on which second a step of the test lands in: an age's width,
a fold on two equal ages, or a sleep that proves a wait. The stated
exception is what the sweep (Do 4) files rather than fixes, named in
#131 with its evidence. (Settled in the round: the goal said "no
assertion" while Do 4 allowed filing; the review's last note.) Each issue's
"Closes when" is met by the fix that removes the clock at its source,
the renderer's output or the test's assertion, so that a red check on
a slow runner says something about the change. The class is named, so
a review and a later test can find it by one rule. An injected clock is
not this step's: it is phase 4-0's (`roadmap/TIME-SEAM.md`, section
4).

## Read

- `AGENTS.md`: "Tests pin behaviour at boundaries, never mechanisms";
  C122 (a behaviour pinned once, wording a golden read as a diff).
- C137 in `decisions/daemon.md` (a deadline's contract is its expiry
  behaviour; no test asserts a wall-clock bound a slow machine can
  miss): the rule each fix applies, in a golden and a store test
  rather than a deadline.
- `REVIEW-CHECKLIST.md`, "A count bounded by a whole-second clock":
  the nearest named shape, the apply's `changed:` line.
- #109's fix, `e5770ac5` and `02635d56`: the same class in a
  comparison, answered by one harness rule (`unaged`,
  `crates/acquisition-client/tests/harness/mod.rs`) rather than per
  test.
- #127: `crates/acquisition-cli/src/store_cmd.rs`,
  `render_tabs_table`, which pads the `fetched` cell to a fixed twelve
  (`{:<12}`); `crates/acquisition-cli/tests/harness/mod.rs`, `age_rule`;
  `crates/acquisition-cli/tests/normaliser.rs`, whose
  `normalising_a_committed_golden_changes_nothing` holds the rules
  idempotent.
- #76: `crates/acquisition-cli/src/plan_cmd.rs`, `render_header`'s
  `facts:` line; `crates/acquisition-store/src/snapshot.rs`,
  `ListingBasis` and `stash_basis`/`character_basis`.
- #111: `crates/acquisition-store/src/lib.rs`, `Store::open` and
  `init`, `a_current_file_opens_while_a_writer_holds_the_lock` and
  #63's `a_writer_beside_the_ingest_makes_it_wait_never_fail_its_upgrade`
  (the busy-handler signal).
- The goldens that hold a padded age, by `grep -rlE "<age> ago {2,}"
  crates/*/tests/goldens`: `journey/a_first_session.txt`,
  `new_league/launch_morning.txt`, `seat_faults/tabs_counts_first.txt`
  (29 lines); the golden holding the fold,
  `journey/a_first_session.txt:39`.

## Input

The last push's CI, read at the step's start: `Gate (Linux)` run
40940013037 and `Gate (Windows)` run 40940013132, both on `49688ee0`,
both in progress. The step goes on.

Measured on `49688ee0`:

- **#127.** The issue says the cell is padded to the widest raw age;
  the tree says a fixed twelve. Either way the cell's width (the age
  and the spaces after it) is constant while the age fits, and the
  `age` rule replaces the age but keeps the spaces, so the normalised
  line moves when the age's digits do. The committed
  `new_league/launch_morning.txt` already shows it: two rows of one
  table, `Currency` with 19 spaces after `<age> ago` and `Dump` with
  20.
- **#76.** The store holds a `response_id` per listing
  (`ListingBasis`), but the stash and character listings are always
  two responses of two endpoints (`stash_basis` reads `endpoint =
  'stashes'`, `character_basis` `'characters'`), and `responses` has
  no job or apply column (`schema.sql`). "Both listed by the same
  response" is never true, and no other stored fact ties the two. The
  issue's second fix, two listings at one logical instant, needs the
  daemon's clock injected: phase 4-0's.
- **#111.** `Store::init` sets the busy timeout itself, which replaces
  any busy handler a caller installs, so a test cannot observe the
  opener's wait through `Store::open`. The same test's first half
  bounds the non-waiting open by `elapsed() < 2s`, a second wall-clock
  bound of the same shape.

## Do

1. **#127, the padded age.** Choose between the issue's two fixes and
   say why in the commit. A renderer fix cannot take the clock out of
   this cell: the age is the clock, and a fixed-width or right-aligned
   age moves its own padding the same way. So the harness's `age` rule
   keeps the width of a padded cell, the age and the spaces after it,
   never fewer than one space; no word a user meets changes. Pinned in
   `normaliser.rs` by a row of the tabs table at `9s ago`, `10s ago`
   and `100d ago` reading one text. The three goldens are regenerated
   and each changed line read: only spaces after `<age> ago` move.
2. **#76, the fold.** The renderer stops folding: the facts line names
   each listing's age (`stash listed <age> ago; characters listed
   <age> ago`), since the store holds no fact to fold on and a logical
   instant is 4-0's. This changes words a user meets in the one case
   the fold printed; it is an agent-facing design, recorded in the
   commit and listed at the close. Pinned by a unit test in
   `plan_cmd.rs` that stages the two listings in one second and a
   second apart and reads one shape.
3. **#111, the sleep.** `Store::open` sets the busy timeout and `init`
   takes a connection whose busy policy its caller has set. The test
   opens through `init` with a counting busy handler: the second half
   commits the writer only after the opener's handler has run (#63's
   shape) and asserts it ran; the first half's handler counts and
   refuses, so an open that wanted the lock fails at once, and the
   `elapsed()` bound goes. No sleep, no wall-clock bound.
4. **The class, by one rule.** Search the CLI, MCP, verbs and store
   tests (and the renderers their goldens read) for any other output
   or assertion that depends on wall-clock coincidence: an age's
   width, a fold or branch on equal ages, a sleep proving a wait or an
   elapsed-time bound. Fix what is small by the same rules; file the
   rest as an issue with its evidence (one `gh` call each). The close
   names the pattern; a `REVIEW-CHECKLIST.md` row if it came back.

## Done when

- `roadmap/README.md` holds the owner's dated "yes to 128" bullet and
  "A step" item 1 names the CI read; `TEMPLATE.md`'s Input names the
  last push's CI.
- `age_rule` keeps a padded cell's width, and
  `the_age_rule_keeps_a_padded_cells_width_whatever_the_ages_digits`
  pins it; every changed golden line holds `<age> ago` and differs
  only in the spaces after it.
- `render_header` has no branch on equal ages, and
  `the_facts_line_reads_one_shape_whether_the_listings_share_a_second`
  pins it; `journey/a_first_session.txt:39` reads the two ages.
- `a_current_file_opens_while_a_writer_holds_the_lock` holds no
  `sleep` and no `elapsed()`, and fails when the migration's lock is
  made deferred or the version gate's read immediate (the report
  shows both).
- Each other instance the sweep found is fixed in this step's commits
  or named in an issue the close cites.
- `tools/gate.sh` is green on the step's tree.

## Not in scope

- An injected clock, in the daemon or the store: phase 4-0
  (`roadmap/TIME-SEAM.md`).
- #128's candidate Windows-target clippy stage: moved to #106 as one
  comment, measured cold there.
- Polling loops that sleep until a condition holds, under a deadline
  that only a broken run reaches: not this class.
- Any other wording of the plan header, or of the tabs table.

## Rules

`TEMPLATE.md`. What differs: this step runs in the main tree on
`main`, in a checkout other sessions share; never push, `gh` writes
one at a time, `acq` only in a mock world, kill only this session's
pids, and an experiment is undone from a scratch copy, never `git
checkout -- <file>`.

## Report

`TEMPLATE.md`.
