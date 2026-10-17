# The trade import's seat — the plan

Step 8 of `search/TRADE-PLAN.md`, run by the seat skill
(`.claude/skills/seat/SKILL.md`), the third seat of the search and the
first on two surfaces and two models. This file is the brief the writing and
orchestrating session runs under; it is deleted at the audit's close
and cited at its commit. The record is this directory's `README.md`
(the runs, their call counts, one row per fault with its fate) and
`TRADE-SLICE.md`'s step 8 row.

The owner's decisions, 2026-10-17, verbatim: the brief by "a dedicated
session to write the seat brief after its own research and assessment";
"I do like the idea of exercising both the mcp and cli"; reviews "in
whatever order is going to be most effective at getting actionable
feedback"; "i want to give the agents one open-ended question at the
end so they can provide answers to questions we haven't thought to ask
and/or expand on their existing answers, too"; "I'm also ok doing all
of this in serial so we don't have to worry about concurrency"; the store: "Let's use GERWARIC since I have the most equipment there."; the models: Opus 5.5 for the session and the first round, then "i like the idea of a sonnet round after an opus round. let's do that regardless."

## Roles, and what each reads

- **The writer, then orchestrator** — one session, in the repository,
  reading as any session does (`AGENTS.md`), and for this work:
  `search/TRADE-PLAN.md` ("Open for the seat"), `TRADE-SLICE.md`, the
  seat skill, the whole-CLI brief at `d1a10dd4`
  (`audits/rehome-2026-10/seat/BRIEF.md`), the MCP seat's brief and
  door at `8da019ad` (`spikes/rust-playground/search/agent-seat/`).
  It knows the open questions so the tasks reach them; no brief it
  writes says what it expects to be found. It never runs a task
  itself. Its last message is the handoff below.
- **The runners** — four fresh `general-purpose` subagents, never a
  fork (a fork inherits the session's context and is not blind), the
  model set on the call: the Opus round, CLI then MCP, then the Sonnet
  round, CLI then MCP, each launched only after the previous debrief is
  written and its files committed. The Sonnet round asks whether the
  tool's words carry a smaller model, the release's audience; a fault
  one model met and the other did not is a finding of its own kind.
  Each
  reads its brief and, for the CLI, the copy of README's tour in its
  world; the MCP runner's tour is the server's own instructions and
  tool texts, delivered over the protocol as at 11c (`search/LEDGER.md`),
  and nothing else — that difference is deliberate and is the only one
  between the two briefs beyond the door's commands. Neither sees the
  repository, the other's run, or this plan. A runner is never
  continued into a rerun: it has learned the tool.

## Inputs

- The store: the owner's `GERWARIC` account, one facts file and its
  intent file, copied through sqlite's online backup (C35) with the
  daemon stopped. The owner stops it; the session asks once.
- Three site searches the owner would hand the tool near a league
  start, "basic mapping gear", pasted as given:
  1. `https://www.pathofexile.com/trade/search/Standard/H4sIAAAAAAAAE42R3WrDMAyF3-Vcm9KO7cavUkrwbKUTeHawlFET_O4jDR0tCWG-kfX3cSRNEHU6CuyEPCjnBAuXKpq5ZwT2PEHrQPd4gEHPUaksCQ6wGITGkA-L6TSri13knmDw4-JIM_ubE-zHsRkEFvcZKcD2Lgo1s0MJdFMqrHWFettCXWbYv-T5L5elKyQs6pJfS93k70ntudAe8H1TsHms1ucx6arp1OaSv4mWQ3RP_tPXO6VrLvX1kt6TSC71UDhd0eb3C2hxhVrzAQAA`
  2. `https://www.pathofexile.com/trade/search/Standard/H4sIAAAAAAAAE5WQ2wrCMAyG3yXXRaYiQl9FpMQ1k0LXjCYTxui7yzaRTb0xNzmR708ygihqL2BH4E4DJ7CAaYBi5o6AvYygQ0dz3YOBJkSlvDSCBwudUO95tzgXUp0Jhbxr-UEtJXXSEU2TD4w9TUptSGAPp2LAB8FbJA-2wShUzE-msmJ0FGccRpdJgiimmr6o-6r6ExtD8405_1ruWq6r85e3uFW-CmtUunMetn_NLfd5d2NWgVKKeVXWjA_NjR7m937HqiqTPQEhnjSNwAEAAA`
  3. `https://www.pathofexile.com/trade/search/Standard/H4sIAAAAAAAAE3VPXQvCMAz8L_dchhPxo39FpFSbaaBbS5uJY_S_y9wetgfzkuQuyV1GZLHSZ-gRIQqHDhq2G1DUj8nQ1xEyRPrhDgoNe6E0E-ygQZ_o-cFSTQvmsL-cj6f6tEdRy0DM1LtQzclIEOuN54ag8La-p0m85Q663u2KguNs754cdGN9pnIrt5Xq7Mas-lX5sELPkIbtO6kNfaryi8k7lFLUAv05YtPG0BRfIcEDZicBAAA`
  Before the briefs are committed the session decodes each in the
  world (`--print-request`) and records what each translates to and
  with what remainder, in its handoff and nowhere the runner reads; a
  URL the build refuses is reported to the owner, never swapped.
- The tasks step 8 names (`search/TRADE-PLAN.md`): a URL from a
  friend, what in my stash matches, in a person's words; one of them
  bounding an `increased` stat; a map searched by its Base; a weighted
  sum over a scourge stat. The three URLs are the material for the
  first; the other three are composed by the writer in a person's
  words, each confirmed to run on the build.

## Steps, each with its acceptance

0. **The doors.** The CLI door (`.claude/skills/seat/seat.py`) takes
   its world root and the directory of the binaries it runs as
   arguments instead of deriving them from the repository, and gains a
   `debrief` command beside `report`, appending to `DEBRIEF.md`. The
   MCP door comes back from `8da019ad` into the skill's directory beside
   it, with the same arguments, the same `calls.csv` row per call and
   the same `report` and `debrief`, driving the release `acq-mcp` over
   stdio; the skill's text names both. Nothing a runner can reach spawns
   a daemon or touches the real provider: every command carries the
   mock provider, the world's store directory and no-spawn (C88).
   *Acceptance:* each door, in a scratch world outside the repository:
   setup, one call, `report`, `debrief`, teardown, with the row and the
   files as described; the gate green.
1. **The briefs.** `TASKS.md` (shared: `T0` orient before touching
   anything, then the tasks in a person's words), `BRIEF-cli.md` and
   `BRIEF-mcp.md` (each from the whole-CLI brief's shape: what the tool
   is, the two things the runner may read, the door's commands and that
   nothing else touches the tool, outputs read as printed, the world,
   the report's headings, the acceptance sentence, the last message of
   at most 300 words), and `DEBRIEF.md` (the questions below, verbatim,
   identical for both). *Acceptance:* committed; **review 1**, outside,
   reads the briefs for an expectation that leaks, a task the build
   refuses, and a world the door cannot make; its findings fixed and
   committed before any build or run.
2. **The build and the worlds.** `cargo build --release -p
   acquisition-cli -p acquisition-mcp` and the store copier, once; the
   binaries copied into each world, their sha256 in the handoff and the
   record; the store copy made once, the daemon stopped, and the same
   copy given to both worlds. Worlds live outside the repository, under
   the owner's home, one directory per surface holding the brief, the
   tour copy (CLI), the door, the binaries and the store; nothing else.
   *Acceptance:* the door's setup reports the world; no build runs from
   here until step 7 (C84: a build replaces nothing a runner holds,
   because the runner holds copies — the rule stands anyway).
3. **The Opus CLI run.** One subagent, model Opus 5.5, its prompt the
   brief's path and the prohibitions, nothing of what is expected. When its
   report is written, a second message carries the debrief, answered
   through the door. `calls.csv`, `REPORT.md`, `DEBRIEF.md` copied into
   this directory and committed as the run's record. *Acceptance:* the
   report cites calls by number; the debrief's last answer is to the
   open question.
4. **The Opus MCP run.** The same, a fresh subagent, launched after
   step 3's files are committed; its prompt carries nothing of step 3.
5. **The Sonnet round.** The CLI run and then the MCP run again, model
   Sonnet 5.5, the same briefs unchanged, each in a fresh world made
   from the same store copy, each committed as its own run (`-opus`,
   `-sonnet` in the file names); their prompts carry nothing of the
   Opus round.
6. **Reproduction.** Every claimed fault reproduced in a scratch mock
   world (the mock-session skill) before it is believed; the record
   says how many calls were read against each report. A fault that is
   a ruling's consequence — the stat-value park's trigger meeting its
   evidence, `blighted-base`, `scourge-in-sum` — goes to the owner
   before the round, not to the reviewer. A difference between the two
   surfaces' answers to one task is a finding of its own.
7. **The fix round.** One commit per fault, each held by a test in
   `crates/acquisition-cli/tests/seat_faults.rs` or the MCP's tests,
   named for the fault; what waits is a park with its trigger marked
   fired; references regenerated after help changes; the gate run bare.
8. **The reruns.** For each surface whose words the round changed: the
   first run's files renamed (`-1`), its world moved aside, the release
   built again and copied, a fresh world, a fresh runner with the model
   whose run met the fault (both models where both met it), the debrief
   again.
9. **Review 2**, outside: the round against the reports and debriefs,
   `calls.csv` beside them; its findings fixed by one rule, never per
   case.
10. **Close.** This directory's `README.md` — the runs with their
   model, call counts and trees, one row per fault with the runs that
   met it and its fate (a commit, a park, or why it
   is no fault), the debriefs' answers routed (a finding, a park, a
   README gap, or dropped, each named); `TRADE-SLICE.md`'s step 8 row;
   the briefs and this plan `git rm`'d, cited at their commits;
   teardown of both worlds; the session-close skill.

## The debrief

Asked after the report, through the door, each answer citing call
numbers where it can and marked "impression" where it cannot:

1. Where did the tool stop you, and what did you do instead?
2. Which outputs did you have to read twice, and what made them hard?
3. What did you want to ask a person, and when?
4. Which of the tool's own words misled you, and what did you take
   them to mean?
5. What would you change first, and why that first?
6. What did you never find out?
7. What should we have asked you that we did not, and what would you
   add to any answer above?

## Rules

- Serial throughout: one runner at a time, no build while one runs;
  the order is Opus CLI, Opus MCP, Sonnet CLI, Sonnet MCP.
- The runner reads its brief and its tour and nothing else; it is
  never told what to find; it is never continued.
- A report's counts are the runner's; the record says so.
- Binaries in a world are copies named by sha256; the store is a copy
  made once, the owner's files never written (C35).
- Nothing reaches GGG: the doors set the mock provider, the world's
  store and no-spawn on every command; this seat needs no daemon.
- A brief's expectation leak found at review 1 is fixed before the run,
  not noted for after.

## Handoff

The session's last message to the owner: the sha256 of each binary
copied; what each URL translated to, with its remainder; per run and its
model, the calls logged and the calls read, the faults claimed, reproduced and
their fates; the debrief answers that became a finding or a park; and
what `search/TRADE-PLAN.md` and `TRADE-SLICE.md` failed to tell it —
the reader test of the brief's cut.
