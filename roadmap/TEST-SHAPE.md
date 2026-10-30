# Test-shape notes: the 2026-10-29 read of the build and test system

Provenance: Fable (`claude-fable-5-1`), one session with the owner on
2026-10-29, read-only; five Opus readers over the daemon, the search,
the doors, the tooling and the store, plan and protocol; the full suite
and the gate run twice on the 10-core M4; the readers' load-bearing
claims checked against the tree. In the manner of `TIME-SEAM.md`:
dated evidence and a proposal, never a ruling. Where a count below
came from a reader's regex it says so, and the step that acts on it
re-reads its sites first (`TESTING-NOTES.md`, "Read the sites before
deciding what to do with them": sixteen literals were seven redundant
and nine already right, and the plan changed from convert to
cross-check).

The owner's question: the build and test take a long time, but is
that the root? Much of the system may be scaffolding from the
build-out, written piecemeal; some tests may pin minutiae and block a
future rewrite of the internals. What is worth keeping, tweaking,
removing, and what shape would we want for ourselves?

## 1. What it costs, measured

| Measurement | Result |
| --- | --- |
| Full test run, warm | 1391 tests, 26 s wall, 237 test-seconds |
| Tests under 0.1 s | 1059 of 1391 |
| Five tests over 5 s | 52 of the 237 test-seconds |
| Gate, nothing edited | 61 s (test 31, clippy 8, winclippy 9, scripts 11) |
| Gate after touching one store file | 247 s (build 30, test 108, doc 79, the rest 30) |
| Compile after that touch | 607 CPU-s, 433 of them test units |
| Daemon library, plain vs with its inline tests | 24 s vs 74 s |
| Three one-test environment binaries | 37 CPU-s of compile for 3 tests |
| Linux CI gate | 7 to 10 min; the test step 355 s (3 min 19 s compile, 155 s run under `ACQ_CASES=full`) |
| Linux CI breakers job | 339 s a push; never red except for its own rot (`fb85d749`) |
| Windows CI gate | 9 to 34 min |
| Test lines against production lines | about 95k against 73k |
| Commits since the root swap touching only markdown | 43% |

The suite is not slow. Three mechanical facts explain most of the
time from an edit to green:

- **Inline tests triple a library's compile.** The daemon, the store
  and the plan keep 7k to 18k lines of tests inside `src/`, so every
  library compile pays for them, and so does every crate above.
- **A store test edit moves the contract revision.** The protocol
  crate's build script hashes the store's whole `src/`, tests
  included, so an edit to a store unit test recompiles the protocol
  crate and everything that links it.
- **The doc stage is 80 s after any edit, serial, and CI never runs
  it.** The one check that can pass locally and go unchecked upstream
  is also the largest fixed cost after the test compile.

The five slow tests are real-time waits, not daemon starts. A mock
daemon starts and logs in within 0.2 to 0.6 s. The slow ones: the
new-league plan waiting out the mock's real stash limiter (18 s), the
web refresh loop (17 s), two trade tests each parsing a 17 MB capture
in its own process (12 s), and four provider tests each waiting out
the sleep job's three-second default (12 s).

## 2. The patterns

1. **A ledger of incidents, not a description of the contract.**
   Files are named for the audit that found the fault: `seat_faults.rs`
   in two crates, `surface_faults.rs`, tests named for a step, a rule,
   a seat or a review. Of 1354 test functions, 429 are named by a
   ruling or claim id. "A finding is pinned as a test" has no
   counterpart that consolidates, so one behaviour ends up pinned at
   three layers; `REVIEW-CHECKLIST.md` names the shape, and `cafd1e46`
   cut 1,500 lines of it on 2026-10-21, but the generator still runs:
   the CLI's `seat_faults.rs` has 65 commits, 25 of them edit-only.
2. **Wording pinned twice.** The goldens with their `# holds` facts
   are a good design. Beside them sit about 600 `.contains("…")`
   sentence literals (231 in the CLI's tests, 176 in the daemon's),
   many repeating a fact a golden holds. A rewording is three edits in
   three crates.
3. **Twenty-five fixture worlds.** The mock serves one. Beside it: 18
   seed functions across the CLI's and the MCP's tests, three private
   mini-worlds in the plan crate, 131 inline tab fetches in the
   search's tests (a reader's count), and every store row built
   inline. The journey's facts table is read by three files, and
   `apply_loop.rs` hard-codes its numbers without reading it.
4. **The daemon's tests drive internals and carry the suite's best
   idea.** The harness builds the daemon by a 25-field struct literal
   duplicated from `start.rs` (a second copy in `sessions.rs`), submits
   through the internal `submit` 111 times against 25 uses of
   `handle_request`, and synchronises through the shared state at 138
   sites; `sessions.rs` asserts generation counters and receiver counts
   with no oracle. Yet 157 of the 215 scenarios end through a teardown
   that runs the journal oracles, and those, with the scripted-server
   scenarios for a 401, a 429, a Cloudflare block and a laptop sleep,
   are what a rewrite must keep. The contract is right; the door is
   wrong.
5. **The search is already close to the shape wanted.** About 276 of
   its 381 tests are query text in, answer JSON out, over items entered
   through the store's ingest; its 21 metamorphic properties touch no
   internals; the expected-answer tables and the trade oracle over 760
   captured searches are corpus-shaped. What pins mechanism is small:
   `derive.rs` entering below `show`, a `facts_version` literal, table
   version literals, about 20 scattered describe-wording checks with no
   describe golden.
6. **Guards of guards past their use.** `fixtures`, `artifacts`,
   `features` and `tree` each came from one incident (#87, #88, #124,
   #125). `features` now holds the property at its cause on every
   release target; `artifacts` re-holds it on one host after the fact
   (corrected 2026-10-29: `features` holds feature-graph equality,
   `artifacts` executable equality, which also sees a profile split and
   a binary crate with no integration-test target; t2's build report
   and its reviews, `reviews/t2-8b02f38b.md` and
   `reviews/t2-fixes-79658fc2.md`);
   `fixtures` guards a variable the script already unsets. The six
   breaker suites are 969 lines and 339 s a push. cargo-deny is the
   only CI check that has caught a real fault. Clippy's `--all-features`
   lints nothing: no crate declares a feature.
7. **Where a rewrite is actually blocked** is narrower than feared:
   about 45 daemon tests asserting internal state and 75 synchronising
   through it (a reader's regex); 19 search tests entering at the
   deriver; about 12 store tests naming index names, an envelope key or
   a private helper; the version literals. The rest sits at a boundary.
   The problem is mass and organisation, not mechanism.

## 3. The reframe

Organise the suite by contract surface, one oracle and one corpus per
surface, and let a finding become a row in a corpus rather than a file
of its own. The four surfaces, each with the seed of its oracle:

- **The GGG side.** The journal oracles over scripted-server
  scenarios, driven through the five-call door `TESTING-NOTES.md`
  already names (construct with a base URL, a clock and a credential
  store; submit through the protocol; await terminal through the
  protocol; read the journal). One constructor shared with `start.rs`.
  The permutation families become tables; a failure carries a
  structured kind so a test asserts the classification once;
  `sessions.rs` rewritten as journaling scenarios ("one owner" becomes
  "one token POST on the wire"). Estimate: 215 scenarios to about 120
  with no loss on the GGG side.
- **Facts and plans.** Keep the C35 tests, the migrations, the world
  and socket derivation, the C38 envelope tests, the price grammars
  and the two committed JSON schemas. Move the public-API tests out of
  `src/` and let the digest skip tests. Cut the dozen store tests that
  name SQL internals.
- **The search.** Keep the properties and the oracle unchanged. Fold
  the per-feature example tests into a case table (items, query,
  expected counts, ids, refusal kind) over a few named fixtures; enter
  the deriver's expectations through `show`; one describe golden; drop
  the version literals. Both fault files become tagged rows.
- **The doors.** One canonical mock world extended into one facts
  table that the mock, the store, the plan and the search all load. A
  scenario corpus run through both doors: a few long CLI transcripts in
  the shape of the journey's (the 22 goldens under 30 lines merge into
  them), the MCP compared field for field to the CLI's JSON, each
  door's refusal shape, one sampled "every printed command runs" sweep
  in place of five exhaustive ones. The protocol's wire fixtures stay
  as they are.

Around that core, the checks that guard a product invariant stay in
every gate: the dependency edges, the harness-only spawn rule, the
no-id sweep, the surface-size bounds, the README tour, `features`,
`tree`, winclippy while Windows keeps going red, and the shipped-table
checks. The working-memory checks (stale identifiers, document paths,
unknown ids) cost six seconds and earn them. The rest moves to CI or
goes: the doc stage to a parallel CI job, `fixtures` and `artifacts`
retired, the breakers replaced by a positive control inside each check
(the dependency guard must find a known edge; the identifier scan must
resolve a known token), the sitting tools' self-tests to the
site-sitting skill.

## 4. The time seam

`TIME-SEAM.md` (phase 4-0) and this reframe want the same thing from
the same place. The door collapse needs one constructor shared with
`start.rs`; the seam needs the same constructor, since `start.rs`
hard-wires `SystemClock` and the lifecycle polls must become
transitions over one injected clock. The seam's fourth item, a runner
that drives a machine through injected ticks, is the scenario-row
shape above: a script, requests and clock moves in, a journal out. Two
steps would rewrite the same 215 scenarios twice, so the daemon's
test reorganisation is 4-0's first move, or the driving-surface
decision is 4-0's design session's. Two corrections the seam makes to
this read: the client's handshake deadline runs in logical time
in-process, with the cross-process twin a smoke test asserting no
numbers; and a fast-window knob on the mock buys back about 45 s now
but is a stop-gap, the seam being the cure. Issue #131's small items
proceed on their own, except the exit-machine ones the issue already
assigns to the seam.

## 5. The governance applied

- **P6 is the lever never pulled.** "A check or a rule is retired when
  what it guarded is proven" has no procedure and no evidence home.
  Applied literally: every deletion is a finding whose evidence is the
  structural guard that now holds the property (the feature pins and
  `features` for `artifacts`; a positive control for a breaker; a
  boundary test or a compiler edge for a mechanism test), recorded in
  the commit message. The cleanup is a series of commits each naming
  what supersedes what.
- **Rulings first, findings after.** The stopping rule (a row before a
  file; wording in one golden per surface; the close asks what can be
  retired) is process and the owner's, recorded in the registry before
  any deletion, as C122's complement. Gate changes are the owner's.
  How a crate's own tests are organised is the session's, recorded in
  the commit and listed at the close.
- **Evidence before ceremony.** The gate's cost after a store edit was
  never measured with incremental compilation on, and moving inline
  tests out of `src/` is unmeasured; each is a brief that measures
  with the same command before and after. A gate timing line per run
  is ceremony until a third measurement is needed.
- **Real use validates.** The CLI and the MCP are the two consumers,
  the journey runs at both, and the shared facts table is the property
  they revealed; the GUI's door is the third consumer coming, so the
  corpus is due before 4-full.
- **Structure over discipline.** The stopping rule stays discipline,
  applied at the close and in the review prompt. The structural pieces
  are the compile cost and the positive controls.
- **A procedure becomes a skill after it has run twice.** Dissolving a
  fault file into rows is done by hand for the CLI's `seat_faults.rs`
  and then the search's before it is written down.

## 6. Order of value

Now: the stopping rule; the gate trims (the doc stage to CI, the two
dead stages, `--all-features`, the four real-time waits); the daemon's
door collapse with the seam, since it buys the most freedom per line.
After the league: the canonical world and the door scenario corpus,
the largest size reduction; the search's case table last, being
already well shaped. Measure incremental compilation and tests out of
`src/` before adopting either.

## 7. Open questions for the step that acts

- Does the five-call door take the clock trait the seam settles on, or
  the daemon's `ManualClock` as it stands?
- Where does the canonical world's facts table live so the store, the
  plan, the search and the mock can read it under C1's edges? The
  journey's table is standard-library only for that reason.
- Which of the 600 wording literals are facts a golden already holds,
  and which are the only pin of their sentence? Counted by regex here;
  read before cut.
- Whether the three environment binaries fold once nextest is the
  only runner the gate knows, or stay for the `cargo test` fallback.
