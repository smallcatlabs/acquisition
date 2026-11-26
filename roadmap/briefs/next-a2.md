# next-a2 — tasks counted through both doors

Step A2 of `audits/whole-review-2026-11/NEXT.md`, cut from its "A2.
The counted tasks" and from `next/reports/world.md` §3 beside it (a
reading: the tree holds). Written by the orchestrating session on
2026-11-25, after next-a1; re-cut once, answering its cold review
(`roadmap/reviews/next-a2-brief-e55a41db.md`). No new ruling.

## Goal

One table says what ten things a player wants cost a reader today:
for each task and each door (the CLI, the MCP), the calls it took, how
many were refused, the bytes read, and the answer reached beside the
answer wanted. It is the reader's journal, as the send journal is
GGG's: a later step that claims the tool got easier shows this
table's diff. It pins today's numbers, bad ones included, and changes
nothing a reader meets.

## Inputs

- C122 (a behaviour pinned once; one harness), C153 (a finding joins
  a surface that exists).
- `crates/acquisition-client/tests/harness/seeded.rs`: the world, its
  variations, and the constants an answer is written from.
- The two `journey.rs` (the CLI's and the MCP's tests): read how each
  logs in over the mock and what it leaves out of its record.
- `crates/acquisition-mcp/tests/harness/mod.rs`: `Mcp` (read what
  `rpc` and `call` return and drop), `acq` (the CLI beside the server,
  under one isolation), `finish_job` (the polls).
- `audits/whole-review-2026-11/findings/first-answer.md`: what was
  counted by hand, to compare with.

## The tasks

The player is Alice (`ALICE`) and knows her account's name. "Whole" is
`Seeded::owner()`; "pc" is it with her poe2 scope dropped; "Hardcore"
is it cut to her pc Hardcore.

| # | In her words | World | She also knows | Wanted |
| --- | --- | --- | --- | --- |
| 1 | What do I hold, and how old is it? | whole | | her three leagues, each with its realm; the tab Never as never fetched; an age |
| 2 | Which items have at least 90 life in all? | pc | | each item whose life lines sum to 90 or more, by name; the one it cannot decide, said so |
| 3 | What is in my tab Rings? | whole | the name | its two rings, by name |
| 4 | Are the plain Chaos Orbs in my tab "~price 3 chaos" priced? | whole | the tab's name | 3 chaos |
| 5 | Show me my Kaom's Heart whole | whole | the name | its life line and its sockets |
| 6 | Bring my stash up to date | the mock, logged in | | a plan that names no request |
| 7 | I play one league: what is priced? (none named) | Hardcore | | Hardcore's own count of items |
| 8 | Task 2, holding both games | whole | | task 2's items, and poe2's said to be answered or left out |
| 9 | Price my Doom Loop at 10 chaos | whole | name, price | a read after the write says 10 chaos |
| 10 | Run my saved search Life rings | whole | its name | the rows its query matches |

## Decisions

1. **One test drives both doors, in the MCP's tests,** whose harness
   already runs the `acq` built beside the server: one file, one
   golden, both columns side by side.
2. **The golden is the table:** for each task and door a row of calls,
   refusals, bytes, `polled` or not, the facts wanted and `reached`
   or what came instead; under it the row's calls, one line each. No
   output text: wording has its own goldens.
3. **A script is what a reader holding only the help would try,** and
   the test holds it. A row starts having read the door's first turn
   (`acq --help`; `initialize` and `tools/list`), a row of its own,
   counted once. Reading help is a call. Every word of a call (a verb,
   a flag, a tool, an argument's name, a value, each word of a query)
   occurs in the task's row above, the first turn, or what the row was
   sent before that call. A script that goes straight to the right
   call because its author knew it fails.
4. **Where a reader would guess, the script takes the plainest guess**
   and a comment says what and why. After a refusal that offers a way
   on, the next call is the offer as printed.
5. **The answer is checked, not judged:** `reached` is the wanted
   facts found in the row's last output, each written from the world's
   constants. A wrong answer stays a row: task 7 may reach an empty
   Standard today.
6. **Counts.** A call is one command or one tool call. A refusal is a
   non-zero exit, a JSON-RPC error, or a result marked as an error. A
   job's polls are left out of calls and bytes, and the golden holds
   only whether there were any: they are wall-clock. Bytes are what
   the reader was sent (stdout and stderr; the result as the harness
   receives it), rounded up to two significant figures: a measure the
   golden compares, coarse enough that most rewordings leave it.
7. **Worlds.** All but task 6 run over the seeded world with no
   daemon. Task 6 runs over the mock, logged in as the journeys log
   in, the login outside the count.

## Find first

- `grep -n 'pub fn \|pub const ' crates/acquisition-client/tests/harness/seeded.rs`:
  the variations and constants; add a variation only if a task's
  world needs it.
- `git grep -n -W 'fn rpc\|pub fn call\|pub fn finish_job' -- crates/acquisition-mcp/tests/harness/mod.rs`:
  where the counter goes.
- `git grep -n 'ACQ_UPDATE_FIXTURES' -- 'crates/*/tests'`: how a
  golden is regenerated; reuse one way and say which.
- For tasks over the whole world, before scripting: do each by hand
  from the help alone, under the mock-session skill's "read work"
  heading (`cargo run -q -p acquisition-verbs --example seeded-world -- <scratch>/store`),
  and keep the calls you made, wrong ones included. Task 6 by hand in
  a fresh mock world of its own, logged in as the skill says. The
  script is that record; a task over a variation takes the test's
  first run as its record.

## Done when

- One golden, `crates/acquisition-mcp/tests/goldens/counted_tasks.txt`,
  in decision 2's form, holds the ten tasks through both doors and the
  two first-turn rows, regenerated by one command the test's doc
  names.
- Two staged cases fail the test: a call using a flag its row has not
  read, and a call using an id its row has not been sent.
- The seeded tasks hold `no_daemon_appeared`; task 6 holds the
  journal's invariants as the journeys do.
- The commit message gives the rows' numbers beside
  `first-answer.md`'s where it counted the same task, and why they
  differ.
- `git diff --stat <base> -- 'crates/*/src'` prints nothing, the base
  named in the commit message.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- Making any row better: a wrong answer or a long path is a row.
- A seat: whether words teach a blind reader is the seat's to say.
- The mock serving the seeded world, and the example taking a
  variation. If task 6 cannot be honest over the mock's own world,
  stop and say so.
- The pages as a third door; converting more seeds.

## Rules

`TEMPLATE.md`. What differs:

- Commit to `main`; push once, after the gate, alone in its command.
  A refused push is left and named in the report.
- The orchestrator orders the cold review (C175, a harness change is
  reviewed) and writes the close with the road; you do neither, and
  read nothing under `roadmap/reviews/`.

## Report

`TEMPLATE.md`, and with it: the golden's table whole; each guess a
script took; each task you could not script honestly.
