---
name: seat
description: Sit a blind agent at the tool as its user — a headless session that reads only a committed brief and the tool's own words, drives the release `acq` or `acq-mcp` through one logging door in a mock world, and reports where the tool stopped it — then fix what it found and sit it again. Use before a release, and after any change to the surface an agent meets.
---

# Seat

A seat asks what the tool's own words are worth to someone who has
nothing else. The runner is a headless session started in its world,
told to read its brief and the tour and no other file; every command goes through one door that logs
it; the report cites calls by number. The tool is under test, never
the runner. Ran on the MCP's search tools (`search/LEDGER.md`, 11c) and
on the whole CLI twice (`audits/rehome-2026-10/seat/`, 2026-10-13: 13
faults, a fix round, a rerun), and repeated the traps below, so it is a
skill (P6).

## Before the run

1. Pick the record directory, `<AT>` (`audits/<slice>/seat/`). It
   holds the brief and, after each run, its `calls.csv`, `REPORT.md`
   and `DEBRIEF.md` copied out of the world; at close, the README with
   the fate table. A world is a directory outside the repository (a
   runner in the repository can read it), short-pathed under the
   owner's home, holding the brief, the door, `bin/` and what the door
   makes: `store/`, `world.json`, `calls.csv`, `calls/`, `work/`.
2. Write `<AT>/BRIEF.md`. The one to start from is the whole-CLI
   brief at `d1a10dd4`
   (`git show d1a10dd4:audits/rehome-2026-10/seat/BRIEF.md`). It says:
   - what the tool is, in the words a person handing it over would use;
   - the two things the runner may read (the brief, README's tour) and
     that `AGENTS.md`'s reading list does not apply to this run;
   - the door's commands, and that nothing else touches the tool or its
     store: no `acq` by path, no `sqlite3`, `cargo`, `curl`;
   - that every output is read as printed (no pipe, no filter), and
     where a file the tool reads may live (the world's `work/`);
   - the world: which accounts exist, which has no login and why;
   - tasks in a person's words, `T0` (orient before touching anything)
     to the last, later ones building on earlier;
   - the report's headings: before touching anything; the tasks;
     faults, worst first; what the tour and help did not say; the
     tool's own words; what it would ask of the tool; left out;
   - an acceptance sentence, and a last message of at most 300 words.
3. Commit the brief before the run.
4. Run `python3 -I .claude/skills/seat/doors_test.py`: both doors
   against a tool that stalls, answers half a line or not JSON, exits
   or lingers, and a login that prints no URL — each must end within
   its deadline, leave no process and leave a row (review 6 of the
   release seat found a reader that could block past its deadline).
5. Build the release binaries and the store copier, then the world
   (never while the gate runs, C84):

   ```sh
   cargo build --release -p acquisition-cli -p acquisition-mcp   # -p acquisition-daemon for a world with a login
   cargo build --release -p acquisition-search --example copy-as-store
   python3 -I .claude/skills/seat/seat.py <PRISTINE> <BIN> setup --facts <ACCOUNT>.db   # once, never run in
   python3 -I .claude/skills/seat/seat.py <WORLD> <BIN> setup --store <PRISTINE>/store   # or bare: an empty world
   ```

   Every door command takes the world's root and the directory of the
   binaries it runs; copy the binaries into the world's `bin/` and the
   door beside them, so the runner reaches nothing of the repository.
   `--facts` copies one facts file of the owner's store, and the intent
   file paired with it, through sqlite's online backup: the owner's
   files are read once and never written (C35). It refuses while a
   daemon's WAL holds pages; the owner stops the daemon first. Every
   command carries `ACQ_NO_SPAWN=1` unless setup said `--daemon` (then
   `bin/` needs `acqd`, and `login` works); `world.json` records the
   source and each binary's sha256. The CLI door is `seat.py` (`run`,
   `login`); the MCP door is `seat-mcp.py` (`open`, `call`), one
   `acq-mcp` process per call over stdio, in a world `seat.py` made.
   Both write `report` and `debrief` to the world.
6. Launch the runner with `launch.sh WORLD MODEL DOOR PROMPT-FILE`,
   in the background, never as a subagent: a subagent starts in the
   repository with its instructions, the memory index and the git log,
   and is not blind (the trade seat's probe, 2026-10-17). The script's
   rails are its header. The prompt names the brief's path and repeats
   the prohibitions; it says nothing of what you expect to be found.
   The debrief resumes the same session (`… PROMPT-FILE SESSION-ID`):
   the questions of the plan, the last an open one, and among them
   "which answer would you retract, or not bet on?", which three of the
   trade seat's eight runs proposed and two more answered unasked, and
   "which claims in your report rest on your own knowledge of the game
   rather than on the tool?", which both Opus runs of the re-sit
   proposed and answered unasked (`audits/trade-2026-10/re-sit/`), and
   "did you change the store in any way you did not undo?", which both
   reruns of the release seat proposed and answered unasked
   (`audits/release-2026-10/seat/`).

## After the run

7. Check the report against `calls.csv` and the outputs under the
   world's `calls/` before believing a fault: reproduce it in a scratch
   mock world (the mock-session skill). Of the rerun's faults, one
   was false as stated and one was not shown by the calls it cited.
   Say in the record how many calls you read.
8. The fix round. One commit per fault; each fix is held by a test in
   `crates/acquisition-cli/tests/seat_faults.rs` named for the fault. A
   fault that is a ruling's consequence is kept and made visible, or
   the ruling is brought to the owner; one that waits is a park with
   its trigger marked fired. Regenerate the references after help
   changes (`ACQ_UPDATE_FIXTURES=1`).
9. The rerun, when the round changed what an agent meets: rename the
   first run's files (`REPORT-1.md`, `calls-1.csv`), move its world
   aside, tell the brief not to open them, build the release again,
   set up a fresh world, and launch a fresh runner. Never continue
   the first: it has learned the tool.
10. Close: write `<AT>/README.md` — the runs, their call counts and
   trees, and one row per fault with its fate (a commit, a park, or
   why it is no fault); `git rm <AT>/BRIEF.md`, citing its commit;
   `seat.py <WORLD> <BIN> teardown` (stops its daemon, removes its
   store). The report's "what the tour and the help did not say" is
   input for the README, not a list of faults.

## Traps

- **The runner's harness may refuse it the report file.** The first
  run's `REPORT.md` had to be lifted from the task's output; the door's
  `report` command exists so the runner writes through the door, a
  part at a time as each task ends.
- **No redirect means no plan file.** A verb's output becomes a file
  another verb reads only through `run --out NAME`; without it the
  first run could not try `--apply=FILE` at all.
- **The real provider is the default (C88).** The door sets
  `ACQ_PROVIDER=mock` and the store directory on every command and
  drops a leftover `ACQ_GGG`; a runner that reaches `acq` any other way
  has left the rehearsal. The brief forbids it and says why it is safe
  to write, price and cancel freely.
- **A world's path is its socket's.** A store directory deep under a
  temporary directory makes a socket path too long to bind; keep a
  world with a daemon short-pathed.
- **The harness reads a heredoc as shell.** Under the launch's rails it
  refused a door command whose heredoc held prose it could not parse
  (`?` before an unclosed apostrophe) and one holding JSON with escaped
  quotes. An Opus runner rewrote a refused call inline; a Sonnet runner
  stopped and asked, as its brief bound it, twice (the trade seat's
  Sonnet MCP runs, at T2 and at T1). Any door command's heredoc may be
  refused, so every one has a file form: `W/drafts/NAME`, written by
  the runner's file tool, read by `report`, `debrief` or `call --from
  NAME`, or by the CLI door's `run --stdin`.
- **The rails deny what is not allowed, not what the harness calls
  safe.** A runner ran `date` outside the door three times (the trade
  seat, mcp-opus-2); it reads nothing of the repository, but the record
  names it, as it names a harness's spill of a long output read
  outside W.
- **A long output is the runner's context.** The door prints the head
  of anything over 60,000 bytes and says so; a verb that needs that is
  a finding (`acq tabs` at 355 KB, `search --describe` at 117 KB).
- **A report's counts are the runner's.** Numbers in the task sections
  were not recomputed by the session in either run; say so.
- **A seat's close tears down every world's store, the pristine's too.**
  The re-sit found the plan's pristine copy gone (`teardown` had
  removed it at the trade seat's close) and made it again from the
  owner's unchanged files; make the copy before the briefs, and check
  it is there before a plan names it.
- **The seat's fix round ends with the gate run bare.** A commit went
  through with `git diff --check` failing behind a `;`.
