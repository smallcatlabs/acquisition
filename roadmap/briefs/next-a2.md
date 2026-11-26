# next-a2 — tasks counted through both doors

Step A2 of `audits/whole-review-2026-11/NEXT.md`, cut from its "A2.
The counted tasks" and from `next/reports/world.md` §3 beside it (a
reading: where it and the tree differ, the tree holds). Written by the
orchestrating session on 2026-11-25, after next-a1. No new ruling.

## Goal

One table says what ten things a player wants cost a reader today:
for each task and each door (the CLI, the MCP), the calls it took, how
many of them were refused, a bound on the bytes read, and the answer
reached, beside the answer the task must reach. It is the reader's
journal, as the send journal is GGG's: a later step that claims the
tool got easier shows this table's diff. It pins today's numbers,
bad ones included, and changes nothing a reader meets.

## Inputs

- C122 (a behaviour pinned once; one harness) and C153 (one golden per
  wording; a finding joins a surface that exists).
- C14 (agent traffic passes the daemon's one gate): the refresh task
  runs over the mock, under the harness's isolation.
- `crates/acquisition-client/tests/harness/seeded.rs`: the world and
  its variations; its constants are where an answer comes from.
- `crates/acquisition-cli/tests/journey.rs` and
  `crates/acquisition-mcp/tests/journey.rs`: a session through each
  door over the mock, the login outside the record.
- `crates/acquisition-mcp/tests/harness/mod.rs`: `Mcp` (its `rpc`
  drops the answer's length), `acq` (the CLI beside the server, under
  the same isolation), `finish_job` (the polls).
- `crates/acquisition-mcp/tests/surface_size.rs`: the first turn's
  bytes, held as a bound.
- `audits/whole-review-2026-11/findings/first-answer.md`: what was
  counted by hand, to compare with.

## The tasks

In a player's words. The first six are `NEXT.md`'s; the last four are
from the owner's own store. He has seen the list.

1. What do I hold, and how old is it?
2. Which of my items have at least 90 life?
3. What is in my tab called "Rings"?
4. Is this item priced, and at what?
5. Show me everything about this one item.
6. Bring my stash up to date, until there is nothing left to fetch.
7. I only play one league, not Standard: what are my things priced
   at? (No league named.)
8. Task 2 when I hold items in both games.
9. Put a price on this item.
10. Run the search I saved.

## Decisions

1. **One test drives both doors, in the MCP's tests.** That crate's
   harness already runs the `acq` built beside the server, so one test
   file and one golden hold both columns side by side. Rules out a
   table split across two crates, which nobody would read together.
2. **The golden is the table and nothing else:** a row for each task
   and door with calls, refusals, a byte bound, `polled` or not, and
   `reached` or the wrong answer in a few words; under each row, its
   calls, one line each. No output text: the wording has its goldens.
3. **A script is what a reader holding only the help would try.**
   The reader has the task's words and what the door has shown so far.
   Each row starts with nothing read but the door's own first turn
   (`acq --help`; `initialize` and `tools/list`), which is a row of
   its own and is counted once. Reading help is a call. The test
   holds the rule: each verb, flag, tool name and argument name a call
   uses occurs in what its row, or the first turn, has read before
   that call. A value (an account, a tab's name, an id) comes from the
   task's words or from earlier output. A script that goes straight to
   the right call because its author knew it fails that check.
4. **Where a reader would guess, the script takes the plainest
   guess,** and a comment on the row says what it guessed and why.
   When a refusal offers a way on, the next call is the offer as
   printed.
5. **The answer is in the row.** Each task names the facts its last
   output must hold, from the world's constants. A row whose answer is
   wrong says what it got. Task 7 runs over the world cut to one
   league that is not Standard; if today's answer is an empty
   Standard, the row says so and stays.
6. **Counts.** A call is one command or one tool call. A refusal is a
   non-zero exit, or a tool result marked as an error. Polls of a job
   are counted apart and the golden holds only whether there were
   any: they are wall-clock. Bytes are what the reader was sent
   (stdout and stderr; the result as the harness receives it), rounded
   up to two significant figures, so a rewording does not move the
   golden and a tenth saved does.
7. **Worlds.** Tasks 1 to 5 and 7 to 10 run over the seeded world or a
   variation, with no daemon. Task 6 runs over the mock, logged in as
   the journeys log in, the login outside the count.

## Find first

- `grep -n 'pub fn ' crates/acquisition-client/tests/harness/seeded.rs`:
  the variations that exist; add one only if a task needs it.
- `git grep -n -W 'fn rpc\|pub fn call\|pub fn finish_job' -- crates/acquisition-mcp/tests/harness/mod.rs`:
  where the counter goes.
- `git grep -n 'ACQ_UPDATE_FIXTURES' -- crates/acquisition-mcp/tests crates/acquisition-client/tests/harness crates/acquisition-cli/tests/harness`:
  how each crate regenerates a golden; reuse one, and say which.
- `./target/debug/acq --help`, and the `tools/list` answer: the first
  turn each row starts from.
- For each task, before scripting it: do it by hand in a mock world
  written by `cargo run -q -p acquisition-verbs --example seeded-world`,
  from the help alone, and keep the calls you made, wrong ones
  included. The script is that record.

## Done when

- One golden, `crates/acquisition-mcp/tests/goldens/counted_tasks.txt`,
  in decision 2's form, holds the ten tasks through both doors and the
  two first-turn rows, and is regenerated by one command the test's
  doc names.
- The test fails when a call uses a verb, flag, tool or argument its
  row has not read (decision 3), shown by one case that stages it.
- The seeded tasks hold `no_daemon_appeared`; task 6 holds the
  journal's invariants as the journeys do.
- The commit message gives each row's numbers beside
  `first-answer.md`'s where that file counted the same task, and says
  where they differ and why.
- `git diff --stat <the step's base> -- 'crates/*/src'` prints
  nothing, the base named in the commit message.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- Making any row better. A wrong answer or a long path found here is
  a row, and an issue only if no step of `NEXT.md` holds it.
- A seat: a script measures the script, and whether new words teach a
  blind reader is the seat's to say.
- The mock serving the seeded world. If task 6 cannot be made honest
  over the mock's own world, stop and say so.
- The web pages as a third door.
- Converting more seeds to the world.

## Rules

`TEMPLATE.md`. What differs:

- Commit to `main`; push once, after the gate, alone in its command.
  A refused push is left and named in the report.
- Order no cold review and read nothing under `roadmap/reviews/`.
- No change to `decisions/`, `AGENTS.md` or the road.

## Report

`TEMPLATE.md`, and with it: the golden's table whole; each guess a
script took (decision 4); each task you could not script honestly.
