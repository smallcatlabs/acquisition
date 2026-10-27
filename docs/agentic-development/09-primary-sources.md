# 9. Primary sources, annotated

The other reports describe the project's documents; this appendix shows
them, each reproduced exactly as it stands in the tree or in `git show`,
with numbered margin notes that translate the compressed wording and say
what to notice. Read each source first and its notes second, as you
would read a design doc with a senior engineer's comments beside it.

Conventions: *verbatim* means byte for byte, typos included. A note is
keyed to a quoted phrase, so you can find it by search. "GERWARIC" is
the owner's git identity; every commit is authored under it, including
those an agent wrote, and the trailer at the foot of a message
(`Co-Authored-By:` in the earlier history, `Assisted-by:` later) names
the model that wrote it. "Report 4, section 4.8" points at another
report of this series in the same directory.

---

## 9.1 A ruling: C88, C116, and the grammar they obey

**What it is.** A *ruling* is one of the project's numbered decisions,
kept in `decisions/<area>.md`, one file per area of the code. C88 says
the real Path of Exile API (run by Grinding Gear Games, "GGG") is the
default, and the mock (a fake API inside the daemon, for tests and
agent sessions) must be asked for. It was ruled on 2026-09-12 in
`515bf034` (trailer: Claude Fable 5.1) after the owner answered a design
note with "agree to flip the default". C116, ruled 2026-10-18 in
`c0be5c5a`, says no ruling id may appear in text a user reads. Each is
shown as it stands today and as first written, because the difference
is the grammar's work; the grammar itself is the docstring of
`tools/bothy.py`, the script that lints the registry. C88 is the ruling
the entry file's first hard rule cites, so it shapes every agent
session; C116 is a ruling about words, which most later rulings are.

C88 today (`decisions/network.md`, one line):

````text
- **C88 — The real provider is the default; the mock is opted into.** `ACQ_PROVIDER=mock` selects the in-process mock; unset or `ggg` is the real API; any other value, or `ACQ_GGG` at all, is refused at start. A human starts a real-mode daemon: the CLI spawns or replaces one in real mode only with a terminal on stderr; a mock spawn is free; `acq-mcp` never spawns in real mode (C13, the MCP's spawn policy). The rails and one daemon per provider stand. *Why:* one gate protects the relationship, not callers kept away (C14, agents through one gate), and a mock default put every offline read behind a flag. *Pinned:* the `c88_` tests in `provider_default.rs`, the CLI's and the MCP's.
````

C88 as first written (the line `515bf034` added to `spikes/rust-playground/decisions/network.md`):

````text
- **C88 — The real provider is the default; the mock is opted into.** `ACQ_PROVIDER=mock` selects the in-process mock; unset or `ggg` is the real API; any other value, or `ACQ_GGG` at all, is refused at start. A real-mode daemon is started by a human: the CLI spawns or replaces one in real mode only with a terminal on stderr; mock spawn is free; `acq-mcp` never spawns in real mode (C13). The standing rule, the rails and one daemon per provider stand. *Why:* the mock default was the ladder's first rung and C14 retired its premise — one gate protects the relationship, not callers kept away — while the owner's every offline read sat behind a flag (note 14 at `ee1f3985`). Owner 2026-09-12: "agree to flip the default". *Pinned:* `provider_default.rs`. Ruled 2026-09-12.
````

C116 today (`decisions/frontends.md`, one line):

````text
- **C116 — No id stands in user-facing text: no `C`, `S`, `N`, `T` or `Q` id in a help text, a reason, a note, a `says` or a refusal; the rule's words stand in its place.** Nor does a store name (`annotation`, `buyout`, a scope or key) stand in a refusal, which speaks the verb's words. *Why:* every seat met ids it could not look up, and a reason citing one read as a defect of the tool. *Pinned:* `acquisition-cli/tests/no_ids.rs` (any letter with digits).
````

C116 as first written (the line `c0be5c5a` added):

````text
- **C116 — No id in user-facing text: no `C`, `S`, `N`, `T` or `Q` id in a help text, a reason, a note, a `says` or a refusal; the rule's words stand in its place.** The trade report's rule (`the_reports_words_cite_no_record`) widened to every surface as one test. *Why:* every seat since the whole-CLI one met ids it could not look up ("they never helped"); the trade seat's price reason "(C81)" read as a defect of the tool to two runs. Owner, 2026-10-18, on the six as recommended: "I accept all 6. Go ahead." *Details:* `brainstorming-notes/38-the-answer-an-agent-reads.md`, step 4.
````

The entry grammar (`tools/bothy.py`, from its module docstring):

````text
The entry rule. An entry is one bullet,
`- **C<n> — title.** ruling. *Why:* one sentence. *Pinned:* the test or check that holds it.`

1. Plain declaratives: a subject, a verb, one idea per sentence.
2. One *Why:*, the reason and nothing else.
3. *Pinned:* names the file and the test name or prefix that holds the
   property; a ruling nothing holds says `nothing yet`. There is no
   *Details:* (`show` lists where the code names the id) and no
   *Evidence:* (a claim is cited in the ruling or the *Why:* with its
   gloss).
4. A gloss of two to five words on every cited id, the first time it
   appears in the entry: `C53 (three output levels)`. The gloss helps
   the reader and binds nothing.
5. No history: no date, no commit, no owner's quote, no "Decided",
   "Ruled", "Amended" or "Kept", no seat call number, no measurement,
   no note citation. `git log -S'C<n>'` and the commit messages hold
   it, and docs-check refuses the markers. A ruling whose build waits
   says `Unbuilt` and names its issue.
6. One bullet under docs-check's length limit, 800 bytes: a prompt to
   route a mechanism to the code, never a budget to fill.
7. The owner's words stay the authority: an entry changes only by his
   ruling, and the commit that changes it quotes him.

The not-doing rule. "Not doing" at the foot of an area file is one
bullet per thing someone would propose and the project declines:
`- <item>: not until <trigger>.`, or `- <item>: refused …` for what no
trigger reopens. The backlog is GitHub issues, never this list.
````

**Notes**

1. **"`- **C88 — title.**`".** One Markdown bullet: an id, a bold title
   that is a full declarative sentence, the body, then two labelled
   tails, *Why:* and *Pinned:*. The large-organisation analogue is an
   Architecture Decision Record compressed from a page to a paragraph.
   The difference: *Pinned:* names the test that fails if the ruling is
   broken, and tests carry the id in their names (`c88_...`), so a grep
   finds the proof. "nothing yet" is allowed and honest.
2. **"the mock is opted into".** An inversion. In August the real API
   needed an opt-in flag (`ACQ_GGG=1`, visible in the entry file in
   9.2). Report 4, section 4.8 tells the story; note 4 explains why
   agents did not become less safe.
3. **"`ACQ_GGG` at all, is refused at start".** A retired knob is not
   ignored; its presence stops the program, so a stale shell export
   cannot leave someone in the wrong mode unknowingly. Most
   organisations let stale feature-flag values be ignored.
4. **"only with a terminal on stderr".** The mechanism that keeps
   agents out of real mode: an agent's shell has no terminal attached,
   a human's does. "Agents never start a real daemon" is not a rule an
   agent must remember; the program refuses. The entry file's
   principle for this is "structure over discipline".
5. **"(C13, the MCP's spawn policy)".** The parenthesised words after a
   cited id are the *gloss* entry rule 4 requires, so a reader without
   the registry open knows what the cited ruling is about.
6. **The birth form's "Owner 2026-09-12: "agree to flip the default".
   ... Ruled 2026-09-12."** The first version carried a quote, a date
   and a pointer to a design note; today's carries none. Entry rule 5
   ("No history") moved history into git, where `git log -S'C88'` finds
   it. Every entry was rewritten to this rule in the governance reset
   of 2026-10-20 (report 5, section 5.5, stage 5). A ruling now reads as
   current law instead of accreting its past.
7. **C116, "no `C`, `S`, `N`, `T` or `Q` id in a help text".** Ids help
   maintainers and mean nothing to users. The *Why:* names the evidence:
   blind agent testers ("seats", 9.5) kept meeting ids they could not
   look up. The analogue is a lint forbidding ticket numbers in UI copy;
   here the lint came from watching an agent user stumble. Its birth
   form's *Details:* tail and the owner's "I accept all 6. Go ahead."
   are gone, by rules 3 and 5.
8. **Rule 6, "800 bytes: a prompt to route a mechanism to the code,
   never a budget to fill".** A length limit on a decision, with a
   warning against the failure that happened anyway: entries edited to
   fit to the byte. Ruling (c) in 9.6 made the check a printed note
   instead of a failure.
9. **Rule 7, "an entry changes only by his ruling, and the commit that
   changes it quotes him".** Agents draft; only the owner enacts, and
   the evidence of consent is a quotation in the commit. Section 9.8
   collects some.
10. **"The not-doing rule".** Each area file ends with what the project
    declines, each with the trigger that would reopen it: a "won't fix"
    with a reopening condition. Agents tend to re-propose the same
    improvement; a written decline stops the loop.
11. **"`tools/bothy-breakers.sh` proves each refusal on a fixture"** (the
    docstring's last paragraph, not reproduced). A *breaker* suite feeds each check a deliberately broken input and
    expects it to fail. A checker that silently stopped checking would
    otherwise look exactly like a clean tree.

---

## 9.2 The entry file, then and now

**What it is.** `AGENTS.md` is the one file every agent session loads
automatically (the root `CLAUDE.md` holds only `@AGENTS.md`, which
includes it). Its first Rust version was added on 2026-08-24 in
`25d28b14` (trailer: Claude Fable 5), when the Rust code was an
experiment on a branch of the C++ repository; it was 1,916 bytes. It is
reproduced whole, then the "Invariants" and "Hard rules" sections of
today's 15,231-byte file. Report 5, section 5.5 tabulates the six
stages between; this section shows the two ends' text. Everything a
session knows before it has read any other file is here, so the file's
growth is the record of what every session had to be told.

The first Rust entry file (`git show 25d28b14:spikes/rust-playground/AGENTS.md`), whole:

````text
# Rust playground — agent entry point

You are in `spikes/rust-playground` on branch `spikes/rust-playground`: a
Cargo workspace (`acquisition-core`, `acquisition-cli`) that is the
**reference implementation** of the Rust daemon and rate limiter. The
repository-level `AGENTS.md` describes the C++ app on `master`; its build
and Qt guidance does not apply here.

Read, in this order, before changing anything:

1. `README.md` — what exists, how to run it, knobs, known gaps.
2. `CONTEXT.md` — invariants, decisions with rationale, open topics,
   deferred work, and the working style. Owner (Tom) holds the boundaries
   (invariants, protocol, core API surface); agents own internals.
3. `LIVE-TESTING.md` — the control document for anything that touches the
   real GGG API. **Nothing here talks to GGG unless `ACQ_GGG=1` is set;
   never set it without the ladder rules in that file.** Live runs are
   human-run.
4. `TESTING-NOTES.md` — how the project checks its own work; the send
   journal is the contract surface.

Quality gate, kept green by every change (`NETWORK-CLEANUP.md`):
`cargo test --workspace --all-targets`, `cargo clippy --workspace
--all-targets --all-features -- -D warnings`, `cargo fmt --all -- --check`,
`git diff --check`.

Working against the mock daemon in a session: use `ACQ_SOCKET=<short
path>` and `ACQ_NO_KEYRING=1` to stay isolated, `ACQ_SNAPSHOTS=<scratch>`
so `acq pull` does not write under `~/.local/share`, and `acq daemon stop`
when done. A scripted mock login is `acq auth --no-browser` in the
background, then `curl` the printed URL with `/authorize?` replaced by
`/approve?`. macOS has no `timeout`; long holds (up to 300 s + 60 s) are
the limiter working, not a hang.

Facts about GGG live in `../../docs/design/network-ground-truth.md` and are
cited by claim number; new claims are authored on the master-side branch
and cherry-picked here, never the reverse.
````

Today's `AGENTS.md`, the "Invariants" and "Hard rules" sections:

````text
## Invariants

As few as possible, each a property, never a mechanism.

1. **Every daemon-originated request to GGG goes through one rate-limit
   choke point and one send-lifetime global gate**: API GET, HEAD
   probe, OAuth code exchange and OAuth refresh (the token endpoint has
   its own policy, ground truth N33). The browser-owned authorize
   navigation is outside this boundary. A daemon HTTP path that
   bypasses the choke point or the gate is a critical bug however
   reasonable it looks locally.
2. **Rate-limit headers from GGG (`X-Rate-Limit-*`) are the source of
   truth.** Local rate-limit state is a prediction; headers correct it.
3. **Never retry through a Cloudflare block.** Recognition signatures
   and the one known incident: ground truth N1–N4, N28.
4. **The user-agent string stays continuous with the existing
   registration** ("Acquisition 1.0, same registration, new
   capabilities").
5. **Refresh tokens are never stored in plaintext on disk.** The OS
   keyring is their default home.

## Hard rules

- **The real provider is the default (C88). Never start a real-mode
  daemon from an agent shell.** Every `acq` an agent runs is in a mock
  world: `ACQ_PROVIDER=mock` and `ACQ_STORE_DIR` under your scratch
  (the mock-session skill). A bare `acq` opens the owner's store. Live
  runs are the owner's, from a terminal, under the live-run skill.
- **Rulings first, findings after.** A ruling (an invariant, a
  boundary, process, scope, anything touching the owner's real store
  or the real API) is his, recorded in `decisions/<area>.md` before the
  code that depends on it; a commit or push waits for his word.
  Agent-facing design (a verb, a flag, an MCP tool, a query word, the
  words of an answer) is the session's: recorded in the commit that
  builds it, listed at the close for his review, and entered in the
  registry only when the close finds it a property another area must
  know. A finding (what building or running taught) is recorded
  after, at its home below.
- **Build order.** Run `cargo build --workspace` before anything that
  starts a daemon, and after any test run or edit to a contract input
  (the protocol and store sources, the manifests, the lock), since
  tests never write `acqd` (C84, the artifact mismatch); `tools/gate.sh`
  ends with that build. Never run the live drivers, a bare `acq` or a
  second gate while the gate runs.
- **The gate decides.** Every change keeps `tools/gate.sh` green, and a
  commit is gated on its exit status, never on a filtered grep.
- **No id in user-facing text** (C116): help, refusals, reasons and
  notes say the rule in words.
- **Tests pin behaviour at boundaries, never mechanisms.** The GGG-side
  contract surface is the send journal (`TESTING-NOTES.md`); the
  frontend-side surface is the protocol (C85).
````

**Notes**

1. **"the **reference implementation**" and "The repository-level
   `AGENTS.md` describes the C++ app".** In August the Rust work was a
   spike, and its entry file first had to say that the other entry file
   did not apply. The Rust code became the repository on 2026-10-12
   (report 3, section 3.6).
2. **"Read, in this order, before changing anything".** The first file
   is a router to four documents and holds almost no rules; today's
   holds the rules directly. Report 5 calls this the arc "from a
   landing page, to a router ... to a constitution".
3. **"Owner (Tom) holds the boundaries (invariants, protocol, core API
   surface); agents own internals."** Survives almost verbatim at the
   top of today's file. A decision-rights split of the kind a tech lead
   and a team agree on, written down because a session cannot remember
   last week's conversation about it.
4. **"Nothing here talks to GGG unless `ACQ_GGG=1` is set".** Safety by
   opt-in flag plus a rule. Today's first hard rule is the reverse, and
   the safety is the terminal check (9.1, note 4). The "ladder" was a
   graded sequence of live runs, each allowed only after the previous
   one behaved (report 4, section 4.5).
5. **"Live runs are human-run."** Survives as "Live runs are the
   owner's, from a terminal". The project's most valuable asset is its
   standing with GGG's API, and the one action that can damage it is
   never delegated.
6. **The four-command gate.** Today, ten stages in one script,
   `tools/gate.sh`, printing a table of exit statuses. The rule "a
   commit is gated on its exit status, never on a filtered grep" names
   the failure it forbids: a verdict read off output piped through a
   search, which can show the passing lines and hide the failing one.
   Section 9.4 shows the gate script itself shipping a false green.
7. **"Working against the mock daemon in a session: use `ACQ_SOCKET=`
   ..."** A recipe inside the entry file. Today it is a *skill*, a
   procedure file under `.claude/skills/` loaded when a task matches
   (`mock-session`). A procedure becomes a skill "after it has run twice
   and repeated a trap" (Principles, P6).
8. **"long holds (up to 300 s + 60 s) are the limiter working, not a
   hang."** Addressed to the agent's behaviour, not the code. An agent
   watching a command sit silent for five minutes may conclude it hung
   and kill it; the sentence says the wait is legitimate before the
   agent has to guess. Lines like this show the file is written for a
   reader whose habits are known.
9. **"As few as possible, each a property, never a mechanism."** An
   invariant says what must stay true, never how; the analogue is an
   SLO versus a runbook. Five in two and a half months.
10. **"a critical bug however reasonable it looks locally."** Written for
    a reader who reasons locally: an agent fixing one function can make
    a sensible shortcut that breaks a global property it cannot see.
    The sentence pre-empts that reasoning by name.
11. **"A bare `acq` opens the owner's store."** The concrete danger.
    Without the isolation variables the tool reads, and can write, the
    owner's real inventory data. Every agent run is therefore in a
    *mock world*: its own store directory and daemon (`GLOSSARY.md`).
12. **"Rulings first, findings after."** The owner's phrase of 2026-10-13
    (9.8, quotation 5), expanded into three classes: a *ruling*
    (boundaries, process, the real API or store) is the owner's, written
    before the code; *agent-facing design* (a flag, a verb, the words of
    an answer) is the session's, recorded in its commit and listed for
    his review; a *finding* is recorded after. The middle class came from
    the packet in 9.6. A RACI chart, scoped by kind of decision rather
    than by person.
13. **"since tests never write `acqd` (C84, the artifact mismatch)".** A
    Cargo trap: running tests does not rebuild the daemon binary, so a
    stale daemon would meet new client code and refuse the handshake.
    The rule names the trap so a session need not discover it.
14. **The size.** 1,916 bytes then, 15,231 now, against a budget of
    15,277 in `tools/docs-check.sh`, which since 2026-10-26 prints a note
    rather than failing (9.6, ruling (c)). Every byte is read by every
    session: the budget is a cost control on attention, not on disk.

---

## 9.3 A brief: r3, the hour idle default

**What it is.** A *brief* is the work order a fresh agent session
receives; the building session reads `roadmap/README.md`, the brief and
what the brief names, nothing else. `roadmap/briefs/r3-idle-and-eviction.md`
was written on 2026-10-26 in `69e37e48` (trailer: Claude Fable 5.1), one
of six briefs an assessing session committed together for later Opus
sessions; r3 has not yet run. It follows `roadmap/briefs/TEMPLATE.md`
exactly and holds all three kinds of work a brief can: a ruling just
given, a ruling still to ask for, and a build. Whole (121 lines):

````text
# r3 — The hour idle default, and finished trees out of memory

Readiness step 3 (`roadmap/README.md`, row r), after r2's design (it
does not wait for r2's build). Closes #80; builds C3 as the owner
ruled it on 2026-10-26 ("I approve the 1 hour idle default"). About a
day.

## Goal

A daemon idles out after an hour, not a minute, so an agent's session
and a person's sitting meet the daemon they started a while ago and the
README's "Your agent" section needs no `ACQ_IDLE_SHUTDOWN=3600`
recipe; and a daemon that lives an hour through many applies holds in
memory only the trees that are live, the mirror on disk answering for
the rest, so its memory is bounded by its work and not by its
lifetime. A long refresh is untouched by either: live jobs hold the
daemon however long the limiter makes them wait (`is_idle`), and disk
retention stays at 7 and 30 days.

## Read

- #80 (the evidence: finished jobs never leave `Shared.jobs`,
  `Entry.params` duplicated, the parent tally computed by scanning
  children); `roadmap/STATE-2026-10-26.md` R1.3.
- `decisions/daemon.md`: C3 (amended: an hour, `Unbuilt: issue #80`),
  C6 (memory the source of truth, the table its mirror: this step's
  amendment), C23 (a parent's tally), C45 (an observer reads the ledger
  on disk), C133 (the exit's drain), C7 (params are public).
- `crates/acquisition-daemon/src/daemon.rs`: `IDLE_SHUTDOWN` (around
  line 179), `Entry` (the two params fields), `Shared.jobs`;
  `daemon/server.rs`: `is_idle` (589), `has_live_jobs` (625), the
  `Request::Status`, `Request::Result` and the jobs-listing arms
  (around 260); `daemon/dispatch.rs`: `maybe_finish_parent` (347),
  `conclude`, `finish`; `daemon/queue.rs`: `persist`, `restore_jobs`
  (125, the prune at start), `describe`; `daemon/start.rs`:
  `idle_shutdown_from_env` (10), `retention_from_env` (78).
- `crates/acquisition-store/src/jobs.rs`: `JobDb`, `prune` (252), the
  read-only `Persisted` view and `observe_job` (what an observer reads
  today; the daemon's own fallback reads the same rows).
- Tests: `daemon/tests/idle.rs` (`idle_verdict_*`, on a manual clock),
  `parents.rs`, `queue.rs`, `persisted_queue.rs` (the `c45_` tests),
  the CLI's `apply_loop.rs` and `jobs` goldens; `acq jobs --all`'s help
  ("every finished job the daemon still holds") in
  `crates/acquisition-cli/src/main.rs`.
- `README.md`: the knob table's `ACQ_IDLE_SHUTDOWN` row (default 60)
  and "Your agent" (the recipe this step removes); `readme_tour.rs`
  runs the tour.

## Input

- The C6 amendment, ruled by the owner before Do 3 (Do 1).
- The schedules the tests stage: a parent with one slow child, an
  eviction attempted, the children still in memory; a tree concluded
  and persisted, evicted, `status` and `result` answered identically
  from the mirror; a thousand single jobs concluded, none in memory;
  a live job under a manual clock advanced past an hour, the daemon
  still up.

## Do

1. **The C6 amendment**, brought to the owner first: memory is the
   runtime source of truth for live trees; a tree whose parent has
   concluded and whose every row is persisted may leave memory, and
   the table answers for it. Under 800 bytes, the owner's words kept.
2. **The hour.** `IDLE_SHUTDOWN` to 3600 s; the knob's README row;
   "Your agent" loses the `ACQ_IDLE_SHUTDOWN=3600` lines and keeps
   `acq auth check` as the way a person starts a daemon for an agent
   (C13); every help text or tool description that says "a minute"
   regenerated (`CLI-REFERENCE.md`, `MCP-REFERENCE.md`); the
   no-daemon remedy's printed command (`Frontend::start_command`) loses
   its `ACQ_IDLE_SHUTDOWN=3600` once the default is the hour, with its
   test. The `idle_verdict_*` tests advance a manual clock past the
   knob, so they move with it; the harness's isolation list sets its
   own value and is unchanged.
3. **Eviction.** One function, `evict_tree`, called after a parent
   concludes (and after a childless job concludes) once its persist
   landed: the tree leaves `Shared.jobs`. `Status`, `Result` and the
   jobs listing for an id not in memory read the mirror (`JobDb`), the
   same rows `Persisted` reads, and answer in the same documents; the
   ten most recent finished jobs the listing shows come from the
   mirror; `--all` means every finished job within retention. Children
   stay until their parent concludes, since the tally scans them.
4. **One params copy.** `Entry.params` goes; `info.params` is the one.
5. **Pins**, each named for its schedule (Input), none on a clock.
6. **A measurement** in the message: the daemon's resident size after a
   thousand single mock jobs, before and after.

## Done when

- `acq daemon status` on a fresh mock daemon reports
  `idle_shutdown_seconds: 3600`; the knob row says 3600; "Your agent"
  names no `ACQ_IDLE_SHUTDOWN`; `readme_tour` green.
- The four pins green; `c45_` and `parents.rs` green unchanged.
- `acq jobs --all`'s help says what it now lists.
- The references regenerated, every changed line explained in the
  message (review shape 9).
- The gate green on the step's own tree, every stage's exit status
  shown.

## Not in scope

- C135 (#55) and r2's lock change.
- The parent scan's O(n²) and the outranked check, unless eviction
  removes them for free.
- The MCP spawning in real mode (a C13 question the owner has not
  taken up).
- Disk retention and `prune`'s schedule.

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree; scratch under your session's scratchpad in a
subdirectory named `r3`, every `ACQ_STORE_DIR` under it; commit on
`main`, never push; the `Co-Authored-By` line for your model last;
long commands in the background; `tools/gate.sh` if p0 has landed,
else the gate bare by each stage's own exit status, and `cargo build
--workspace` before any daemon and after any test run (C84).

## Report

`TEMPLATE.md`, as text.
````

**Notes**

The template's headings, in order: **Goal**, **Read**, **Input**,
**Do**, **Done when**, **Not in scope**, **Rules**, **Report**.

1. **Header: "Closes #80; builds C3 as the owner ruled it on 2026-10-26
   ("I approve the 1 hour idle default"). About a day."** Before any
   heading: the roadmap position, the GitHub issue, the owner's ruling
   quoted, and a size. A ticket header in prose.
2. **Goal: "A long refresh is untouched by either".** The template asks
   for one paragraph on "what is true when the step is done, and why it
   matters". This one also says what must not change, which a product
   requirements doc would put under non-goals; it protects the
   rate-limiter's legitimate waits from an idle-timeout change.
3. **Read: "`IDLE_SHUTDOWN` (around line 179) ... `is_idle` (589)".**
   Line-level pointers, because a fresh session has no memory of the
   code. The template requires the writer to check every citation
   against the tree, because phase 0's briefs "cited a ruling as built
   that was parked, named a test that did not exist", and each cost a
   review finding.
4. **Input: "The schedules the tests stage".** Four test scenarios named
   before any code exists: acceptance criteria in the ticket, written so
   a reviewer can check the tests against the brief.
5. **Do 1: "The C6 amendment, brought to the owner first ... Under 800
   bytes, the owner's words kept."** The template: a ruling the step
   needs "is the first item, brought to the owner before the code".
   Changing what the daemon holds in memory touches a boundary (C6), so
   the worker stops and asks. "800 bytes" is the entry limit (9.1).
6. **Do 2: every place "a minute" appears.** The README, the help, the
   generated references and the command the no-daemon refusal prints.
   This anticipates two numbered "shapes of fault" from the review
   prompt: 3, "a sibling sentence left standing", and 6, "a printed
   command that does not run as printed".
7. **Do 5: "none on a clock".** A *pin* is a test that holds a property;
   the template forbids tests that assert wall-clock bounds a slow
   machine can miss. Time is injected and advanced by hand.
8. **Do 6 and Done when: "A measurement in the message".** Done when
   must be checkable "from the commit alone", because "the reviewer
   reads the commit and not the report". The commit message is the
   durable record; the worker's report to its caller is not.
9. **"(review shape 9)".** "A fixture regenerated blind": generated help
   references nobody read are how unintended user-visible changes slip
   through.
10. **Not in scope: "a C13 question the owner has not taken up".** The
    template: "what a worker will be tempted to do and must not, with
    where it lives instead". Agents fix adjacent things they notice;
    this names them in advance.
11. **Rules: "`TEMPLATE.md`, with these differences".** The template
    forbids copying its rules, because "a copied rule drifts". The
    differences are operational: alone in the main tree, scratch under
    a subdirectory named `r3` so parallel workers cannot collide, never
    push, and "if p0 has landed", because this brief was written before
    the gate script it mentions existed.
12. **Report: "`TEMPLATE.md`, as text."** The template's six report
    items end with "Raised for review: anything about the brief, the
    plan, the tools or this way of working that got in your way", and
    the reading session answers each in its close commit. The worker is
    asked to criticise the process that directed it.

---

## 9.4 A review, and the commit that answered it

**What it is.** After a step's commits, the committing session launches
Codex (OpenAI's coding agent) read-only with a fixed prompt,
`roadmap/briefs/REVIEW-PROMPT.md`, naming the range. Codex's final
answer is saved verbatim under `roadmap/reviews/`; the owner reads it;
only then is anything fixed. This review covers p0, the process round
of 2026-10-26: six commits from `f1a29952` to `1523a776`, one of which
(`b3258038`) created `tools/gate.sh`. It was saved by `fa1e6d62`
(trailer: Claude Opus 5.5), and `16ef9210` is the fix. It is short, it
shows the reviewer's fixed format, and its first finding is a false
green in the gate itself, the script every other check depends on.

The review (`roadmap/reviews/p0-1523a776.md`), whole:

````text
# p0 review, f1a29952^..1523a776

Codex, read-only, launched by the committing session with `roadmap/briefs/REVIEW-PROMPT.md`; its final answer verbatim.

f1a29952^..1523a776 — brief’s done-when partially met: static conditions hold; green gate and breaker exits are not verified.

- **[P2] tools/gate.sh:50:** Substring validation accepts several adjacent stage names as one stage, allowing a false green. Input: `tools/gate.sh --only 'build test'` passes validation, matches no `run_stage` branch, and reports success without running either check. Fix: compare against individual array elements and make an unknown stage return nonzero.
- **[P2] .claude/skills/session-close/SKILL.md:12 — shapes 1, 3:** The exhaustive exclusion list incorrectly authorizes `--docs` for changes that tests consume; the same claim appears in `tools/gate.sh:5`. Input: remove `*.lock text eol=lf` from `.gitattributes`; the prescribed subset omits `every_kind_the_digest_reads_is_checked_out_with_lf_everywhere`, which checks that rule. Fix both sentences to require establishing that no test or build consumes the changed input, without presenting this list as exhaustive.
- **[P3] GLOSSARY.md:38 — shape 1:** “Every quote” incorrectly promises the search answer’s provenance fields. Reading a daemon quote, including one embedded in a refresh plan, finds `observed_at`, provider, account, work and scheduling scopes, but no store, snapshot, intent revision or build (`acquisition-protocol/src/protocol.rs:358`). Fix: restrict this definition to search answers and define the other basis meanings separately.
- **[P3] tools/docs-check.sh:67 — shape 3:** The comment still says saved reviews are deleted at phase close, contradicting the retention ruling applied here. Concrete reading: a maintainer explaining `SKIP_ROADMAP` encounters the superseded deletion policy. Fix: replace that clause with “retained permanently.”

Not checked: execution of the gate, breakers, builds or tests, as instructed; their reported successful exits remain unverified. The fixed tree’s 15,230-byte `AGENTS.md`, glossary coverage, scan inclusion, lore matches, ruling application and commit tables were checked statically. No goldens or reference files changed.

Nitpicks: the stale deletion comment is a documentation nitpick. The unusual multi-stage argument still warrants fixing because it produces a false gate verdict.

Brief: replace “stages that read no Rust” with “stages that do not compile or run Rust,” since these checks inspect Rust sources.

Plan: no separate plan was supplied; the brief and commits provided sufficient review scope.

Prompt: no change needed; the range and prohibition on execution were clear.
````

The fix commit (`git show -s --format=%B 16ef9210`), whole:

````text
p0's fix round: the review's four findings

p0, the process round (roadmap/briefs/p0-process.md). The owner read
the review (roadmap/reviews/p0-1523a776.md) and ruled: "Fix all four
findings as proposed, then close p0."

1. P2, gate.sh's --only matched a substring: "build test" passed the
   check, ran no stage and printed green. The rule: a stage name the
   script does not know fails. --only now compares against each name
   exactly, and run_stage's default branch refuses an unknown stage
   with status 2, so a name that slips the check still cannot pass.
   `tools/gate.sh --only 'build test'` and `--only ''` exit 2; `--only
   diff` exits 0. Not pinned as a test: the script has no harness, and
   the default branch removes the class rather than a case.
2. P2, --docs' guidance listed what tests read as if the list were
   whole (.gitattributes is read by a test too). The rule: the session
   establishes that no test or build reads its change, and runs the
   full gate when unsure; the list survives only as examples
   (session-close section 1, gate.sh's help). The review's brief note
   taken too: "the five that build no Rust" in AGENTS.md and gate.sh,
   since the checks read Rust sources.
3. P3, the glossary's "basis" said every quote carries the search
   answer's fields. Now two senses: a search answer's (store, account,
   snapshot, intent revision, build) and a quote's (the work it prices
   echoed back, with its observation time, provider and account;
   `Quote` in acquisition-protocol). 45 entries.
4. P3, docs-check's comment on SKIP_ROADMAP said a saved review is
   deleted at its phase's close; it is kept there for good.

AGENTS.md is 15,231 bytes (15,235 at the round's start).

Gate (tools/gate.sh, the full gate):

stage     rc seconds
build      0       1
test       0      41
clippy     0       1
fmt        0       1
diff       0       0
docs       0      10
arch       0       3
trade      0       6
bothy      0       0
doc        0       0
rebuild    0       1

Assisted-by: Claude Opus 5.5 <noreply@anthropic.com>
````

**Notes**

1. **"its final answer verbatim".** Stored unedited, never rewritten. A
   summarised finding can be softened by the summariser.
2. **"green gate and breaker exits are not verified".** The reviewer is
   told not to run anything, and says what it could not check rather
   than taking the committing session's word: "LGTM pending CI", made
   explicit and kept.
3. **"[P2]", "[P3]", "shapes 1, 3".** Severities, and the numbered
   "shapes of fault" in the review prompt: 1 is "a claim wider than its
   evidence", 3 "a sibling sentence left standing". Findings are
   classified against a vocabulary built from the project's own repeat
   mistakes (`REVIEW-CHECKLIST.md` holds the per-area version).
4. **"`tools/gate.sh --only 'build test'` passes validation, matches no
   `run_stage` branch, and reports success without running either
   check."** The gate script was written that day to make "the gate
   decides" structural, and its stage-name check was a substring match.
   Every finding comes with a concrete triggering input.
5. **"Brief: ...", "Plan: ...", "Prompt: ...".** The review ends by
   critiquing its own inputs: the same upward channel as the brief's
   Report item 6, applied to the reviewer.
6. **"The owner read the review ... and ruled: "Fix all four findings as
   proposed, then close p0.""** The fix opens with the owner's word.
   Because findings wait for his reading, he sees them while still open
   and can redirect.
7. **"The rule: a stage name the script does not know fails. ... so a
   name that slips the check still cannot pass."** Each finding is
   answered by the rule it broke, then the fix, and this fix removes the
   class (any unknown stage, however it arrives), not just the reported
   case.
8. **"Not pinned as a test: the script has no harness".** An admitted
   gap against the project's own rule, stated rather than left for the
   reader to notice.
9. **"AGENTS.md is 15,231 bytes" and the "stage rc seconds" table.** The
   commit carries its evidence: a size measured after the last edit,
   and every stage's exit status and time. A reader can see the test
   suite ran (41 seconds), not merely that someone said so.
10. **What happened next.** The fix was itself reviewed
    (`roadmap/reviews/p0-fixes-16ef9210.md`), and Codex found a second
    false green: a variable named `selected` inherited from the caller's
    shell could pre-select a valid stage, so `--only bogus` still
    printed green. Fixed in `c18eb602`. Two review rounds to close one
    class of false green in one shell script, both found by reading.

---

## 9.5 A seat record

**What it is.** A *seat* is a usability test with an agent as the user.
A headless session, started outside the repository with only a brief
and the README's tour, drives the released binary through a logging
wrapper (the *door*, `.claude/skills/seat/seat.py`) in an isolated
*world* (its own store and daemon over a copy of the owner's data,
mock API), and reports where the tool stopped it, citing logged calls
by number. Shown: the whole record of the whole-CLI seat of 2026-10-13,
the run table of the release seat of 2026-10-19, and the skill's
"Traps". Seats are the project's main evidence on whether the tool's
own words (help, errors, refusals) are enough.

The whole-CLI seat (`audits/rehome-2026-10/seat/README.md`), whole:

````text
# The whole-CLI seat (note 36, step 1) — 2026-10-13

Two runs of one brief (`BRIEF.md` at `d1a10dd4`; the first run's at
`2e1d8e51`) by a subagent blind to everything but the brief and the
README's tour, driving the release `acq` through the seat skill's door
(`.claude/skills/seat/seat.py`, which began here) in a
mock world over a copy of the owner's store. `REPORT-1.md` and
`calls-1.csv` are the first run (172 commands, the tree at `25332463`);
`REPORT.md` and `calls.csv` the rerun after the fix round (238 commands,
the tree at `c5f3b4a2`). The outputs are under gitignored `runs/`. Each
report's section 4, what the tour and the help did not say, is the
release README's input.

What each fault became. A fix is held by the test of
`crates/acquisition-cli/tests/seat_faults.rs` its commit names.

| Run | Fault | Fate |
| --- | --- | --- |
| 1 | account defaulting differs by verb family | kept as C51 has it, made visible: `73e49bac` |
| 1 | price verbs scoped to a league, inconsistently | fixed: `9cb2086d` |
| 1 | `--json` not total on a refused command line | fixed, C11 amended: `1c43a6f5` |
| 1 | one apply is not a sync | the apply says what is left: `23c5ac50`; the fan-out stays parked (`decisions/plans.md`) |
| 1 | `status`, `result`, `auth status` need a daemon | answered from disk: `73e49bac`; the MCP's parked (`decisions/frontends.md`) |
| 1 | `search --describe` is 117 KB | bare is an index: `54102b85` (touches how C97 is met) |
| 1 | another league's rows read as orphans | fixed: `9cb2086d` |
| 1 | nothing waits for a job; `jobs` unfiltered | `result --wait`: `73e49bac`; `jobs`: `c5f3b4a2` |
| 1 | the policy's fields in no help | fixed: `23c5ac50` |
| 1 | "keyring: ok" with the keyring disabled | fixed: `647d8dcf` |
| 1 | an empty-titled spoiler on a shop page | by design: the render's no-price row |
| 1 | a cancelled job's `result` exits 0 | fixed, C11 amended: `73e49bac` |
| 1 | progress and payload on one stream | fixed: `73e49bac` |
| 1, 2 | ids nobody outside the repository can look up | parked, fired; built with the release's text (`decisions/frontends.md`) |
| 1, 2 | not every failure carries a `kind` | usage and account kinds added: `1c43a6f5`; the taxonomy park stands |
| 2 | a plan file applied twice fetches twice | C111: a plan spends each request once; a replay sends nothing and says what spent it (after a design study the owner asked for): `188b0ae3` |
| 2 | `cancel` on a running job "does not stop it" | it does, at the job's next check; the words fixed: `76ef839b` |
| 2 | `daemon stop` ends an unpersisted login silently | fixed: `76ef839b` |
| 2 | a job verb's JSON differs with `--json` | by design (C53), now said in the help: `76ef839b` |
| 2 | `acq tabs` unbounded, its summary last | fixed: `76ef839b` |
| 2 | a league planned fresh after its items moved | the mock's: one item id in two leagues |
| 2 | `price set` on an unknown target exits 0 | by ruling (C64), with the note it prints |
| 2 | a row's revision restarts after a clear | not shown by the calls cited; run 1 saw it keep counting |
| 2 | a token's short life reads as a logout to come | the words fixed: `76ef839b` |

Not reviewed by the session: the task sections' counts, and all but
some twenty of the 410 calls. The rerun's rule breaches are its own
"Left out": it listed README's headings to find the tour, and read one
harness spill file of the tool's output.
````

The release seat's run table (`audits/release-2026-10/seat/README.md`, "The runs"):

````text
## The runs

Each launched by `.claude/skills/seat/launch.sh` in its world, the
debrief the same session resumed; the reruns on the second archive
after the fix round, in fresh worlds, never continued from the first.
Counts are the door's (`calls.csv`); a report's own counts are the
runner's, never recomputed here. Cost is the session's at list price.

| Run | Model | Session | Calls | Cut | Turns | Cost | Files |
| --- | --- | --- | --- | --- | --- | --- | --- |
| cli-opus | `claude-opus-5-5` | `4592effd` | 107 | 2 (`shop render --expand` 3.5 MB, `--json` 7.4 MB) | 76 + 3 | $3.55 | `calls-cli-opus.csv`, `REPORT-cli-opus.md`, `DEBRIEF-cli-opus.md` |
| mcp-opus | `claude-opus-5-5` | `11891c2e` | 51 | 2 (`price_list` 67 KB, `shop_render` 5.1 MB) | 66 + 3 | $3.46 | `calls-mcp-opus.csv`, `REPORT-mcp-opus.md`, `DEBRIEF-mcp-opus.md` |
| cli-sonnet | `claude-sonnet-5-5` | `becbb2ab` | 97 | 2 (`price show --json` 174 KB, `shop render --json` 7.5 MB) | 112 + 2 | $1.50 | `calls-cli-sonnet.csv`, `REPORT-cli-sonnet.md`, `DEBRIEF-cli-sonnet.md` |
| mcp-sonnet | `claude-sonnet-5-5` | `b8abe4b8` | 55 | 2 (`price_list` 67 KB, `shop_render` 5.1 MB) | 71 + 2 | $1.48 | `calls-mcp-sonnet.csv`, `REPORT-mcp-sonnet.md`, `DEBRIEF-mcp-sonnet.md` |
| cli-opus-2 | `claude-opus-5-5` | `cbf8834a` | 107 | 2 (`shop render --json` 208 KB, twice) | 77 + 2 | $3.24 | `calls-cli-opus-2.csv`, `REPORT-cli-opus-2.md`, `DEBRIEF-cli-opus-2.md` |
| mcp-opus-2 | `claude-opus-5-5` | `dd922bd9` | 56 | 1 (`shop_render` 160 KB) | 74 + 2 | $3.21 | `calls-mcp-opus-2.csv`, `REPORT-mcp-opus-2.md`, `DEBRIEF-mcp-opus-2.md` |
````

The seat skill's traps (`.claude/skills/seat/SKILL.md`, "Traps"):

````text
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
````

**Notes**

1. **"blind to everything but the brief and the README's tour".** No
   code, rulings or design notes, as a new user would have none. The
   analogue is a moderated usability study or a new hire's first day;
   the differences are that it repeats on demand, logs every action,
   and the subject writes its own report.
2. **"over a copy of the owner's store".** Realistic data without the
   real API, so the tester can write, price and cancel freely with no
   risk to the GGG relationship.
3. **The "Fate" column.** Each fault ends as *fixed*; *kept* (by design,
   often with the help changed so the design shows); *parked* (deferred
   with a trigger); *the mock's* (a fault in the test double); or *not
   shown by the calls cited* (rejected for evidence). Bug-tracker
   resolution codes, except every resolution names its commit.
4. **"ids nobody outside the repository can look up | parked, fired".**
   The origin of C116 (9.1). "Fired" means the deferred item's trigger
   had arrived.
5. **"a plan file applied twice fetches twice | C111".** The most
   consequential row: replaying a saved plan sent the same API requests
   twice, spending the project's rate-limit budget with GGG. A blind
   tester found it; it became a ruling after a design study the owner
   asked for.
6. **"Not reviewed by the session: ... all but some twenty of the 410
   calls."** The record states what was not checked.
7. **"Model" and "Cut".** Two models (Opus, Sonnet) on two frontends
   (the CLI and the MCP server, the interface an AI host uses), which
   separates "the tool's words fail" from "one model's habits fail".
   The door prints outputs over 60,000 bytes only as their head
   (explained in the source just above this table); a 7.5 MB document
   is unreadable to an agent, and each cut became a fault about
   unbounded output.
8. **"Turns 76 + 3".** The section says the debrief resumed the same
   session, so this is read here as the run's turns plus the
   debrief's; the table does not say so itself.
9. **"Cost $3.55 ... $1.48".** Per run at list price; the six total
   $16.44, roughly what an hour of one engineer's time costs. The `-2`
   reruns ran "in fresh worlds, never continued from the first", so the
   second testers carry nothing from the first.
10. **The traps.** Each is a way the runner, rather than the tool, ends
    up under test: the harness refusing the report file, a heredoc
    refused by the harness's safety rules, `date` run outside the door.
    The skill exists because these recurred (P6).
11. **"An Opus runner rewrote a refused call inline; a Sonnet runner
    stopped and asked, as its brief bound it, twice".** A recorded
    difference between models under one instruction. The skill's answer
    was a file form for every door command, so neither path is needed.
12. **"A commit went through with `git diff --check` failing behind a
    `;`."** In a shell, `a ; b` reports only `b`'s status. The filtered
    grep (9.2, note 6), the gate's false green (9.4) and this are one
    failure in three forms; the gate script captures each stage's
    status in its own variable.

---

## 9.6 A ruling packet, and the owner's word

**What it is.** When a step needs the owner's decisions, the session
sends a *packet*: several questions in one message, each with a
recommendation and its evidence. The p0 brief
(`roadmap/briefs/p0-process.md`, 2026-10-26, `69e37e48`, trailer Claude
Fable 5.1) posed four process questions as its first Do item; the
building session sent them; the owner answered in one sentence;
`f1a29952` recorded it. `git log --grep='approve all 4'` returns five
commits because each commit applying a ruling quoted it again; the
recording commit is the earliest. Question first, answer second.

The question (`roadmap/briefs/p0-process.md`, "Do", item 1):

````text
## Do

1. **The rulings packet**, one message to the owner before any edit,
   four questions with the recommendation under each:
   - (a) Reviews stay under `roadmap/reviews/` for good; a ledger row is
     a commit range and a close date; the save-alone, delete,
     cite-whole and hash-in-ledger steps of "A step" items 3 and 5 go,
     and a phase close deletes nothing. Recommended: yes. What it
     protects today is `tools/docs-check.sh`'s path scan, which already
     excludes `roadmap/reviews/`.
   - (b) Rulings after the fact for the delegated class: an agent-facing
     design decision (a verb, a flag, a tool, a query word, the words
     of an answer) is recorded in the commit that builds it and listed
     in the close; it enters the registry only when the close finds it
     a property another area must know. A boundary, an invariant and
     anything touching the real store or the real API stay rulings
     first, the owner's. Recommended: yes; the owner's reading drops to
     boundaries and findings.
   - (c) `AGENTS.md`'s budget and the 800-byte entry limit print a note
     and never fail the gate; the history-marker refusal and the
     cross-cutting count stay as failures. Recommended: yes; the
     evidence is 28 entries within 50 bytes of the limit and a wording
     the owner had to restore (`ae837d12`).
   - (d) The glossary's home: a "Glossary" section in `AGENTS.md` if
     (c) frees the room, else `GLOSSARY.md` named from "Where things
     live". Recommended: `GLOSSARY.md`, read on demand, named from the
     entry file in one line.
````

The answer (`git show -s --format=%B f1a29952`):

````text
roadmap: the owner's four process rulings from p0's packet

p0, the process round (roadmap/briefs/p0-process.md), Do 1: the
packet of four process questions went to the owner with a
recommendation under each. His answer, 2026-10-26: "I have reviewed
the questions and approve all 4 recommendations."

Recorded under "The owner's rulings for this road", each as the
packet drew it: (a) reviews stay under roadmap/reviews/ for good and
the review choreography's bookkeeping goes; (b) rulings after the
fact for the delegated class, an agent-facing design decision
recorded in its commit and listed at the close; (c) the AGENTS budget
and the 800-byte entry limit become notes; (d) the glossary is
GLOSSARY.md, named from the entry file. The commits after this apply
them; nothing else changes here.

Gate (tools/gate.sh, untracked at this commit, run over this tree):
build 0, test 0, clippy 0, fmt 0, diff 0, docs 0, arch 0, trade 0,
bothy 0, doc 0, rebuild 0.

Assisted-by: Claude Opus 5.5 <noreply@anthropic.com>
````

**Notes**

1. **"one message to the owner before any edit, four questions with the
   recommendation under each".** The owner's attention is the scarce
   resource, so questions are batched, each with a recommendation and
   evidence: a decision memo. What is unusual is that an agent wrote
   it, about the rules that govern agents.
2. **(a) "a phase close deletes nothing".** Saved reviews used to be
   deleted at a phase's close; the review cited in 9.7 went that way.
   This reversed the policy.
3. **(b) "Rulings after the fact for the delegated class".** The agents
   asking for more autonomy, with the boundary drawn in the question:
   "A boundary, an invariant and anything touching the real store or
   the real API stay rulings first, the owner's." The payoff is framed
   for the owner: "the owner's reading drops to boundaries and
   findings".
4. **(c) "28 entries within 50 bytes of the limit and a wording the
   owner had to restore (`ae837d12`)".** A check the agents built was
   being gamed by the agents: entries trimmed to fit, once losing the
   owner's wording. Keep the measurement, drop the failure: Goodhart's
   law, noticed and corrected in the agents' own proposal.
5. **"I have reviewed the questions and approve all 4
   recommendations."** The owner's whole answer. Report 4, section 4.16
   discusses what he did with agent-made decisions; this is the common
   case, a one-line approval of a well-argued packet.
6. **"Recorded ... each as the packet drew it".** Entered in the
   packet's own words: rewording after approval would change what was
   approved (9.1, rule 7).
7. **"The commits after this apply them; nothing else changes here."**
   The recording commit adds 15 lines to `roadmap/README.md` and applies
   nothing, so the decision stands alone and its effects follow as a
   diff.
8. **"tools/gate.sh, untracked at this commit, run over this tree".** The
   script existed in the working tree but was not yet committed, and
   the message says so.

---

## 9.7 A commit message as a document

**What it is.** `0ccafbbf`, 2026-10-19 (trailer: Claude Fable 5.1): the
first time Codex reviewed anything in the project, and what it reviewed
was the roadmap itself (`a987d02f`, written by a Fable session the same
day). The owner asked for the review to judge the review mechanism as
well as the plan.

**Why this one.** The brief offered `0ccafbbf`, or `fba1b03` (the
largest message, 140 lines, the `acq web` build of 2026-10-25) if
`0ccafbbf` ran under 30 lines. It runs 48, so the rule picks it, and it
is the more instructive: `fba1b03` is mostly an inventory of a build
and would need trimming, while `0ccafbbf` shows a process correcting
itself on first contact, in one readable message.

````text
roadmap: the plan reviewed from outside before any step runs — eleven findings, nine taken, the review mechanism itself corrected by its first use

The owner asked to test the outside review on the roadmap itself, and
to judge the mechanism as well as the plan. Codex (gpt-6.1-sol, high
reasoning) read both roadmap commits, the seven reports, the triage,
the briefs and the standing documents in 28 read-only commands and
192,173 tokens, and returned eleven findings. They are verbatim at
roadmap/reviews/roadmap-a987d02f.md, the first file in the directory
every later step's review will use.

The mechanism learned first: the review subcommand refuses a custom
prompt beside a commit, and the sandbox flag must precede any
subcommand, so the launcher is plain `codex exec` with the commit named
in the prompt (two failed launches before one that ran; the recorded
command is now the one that works). The prompt learned next, from the
review's own finding 2: it led with the project's shapes of fault and
put plain defects second, which would steer a reviewer toward prose
and away from the deadline and cancellation bugs the network record
holds. It now leads with behavioural review, then the area checklist,
then the shapes; asks for the smallest sufficient fix instead of "one
rule, never a patch per case" (the fix's shape is the fixer's and the
owner's call); allows "not verified" where a done-when needs execution;
and ends by asking the reviewer what about the brief, the plan or the
prompt it would change. The brief template gains the same closing
section for workers, and the ledger answers every raised item.

The plan's fixes: 0b had delegated an intent-write rule to the worker
against "rulings first", so the merge rule is now proposed in the brief
for the owner's yes before code, with C52 (no blind replace on the MCP)
named; alpha.2 is split from league day and gated on named inputs that
only the owner and RePoE's lag can supply; measurements run alone, as
the M3 method already said; 0a's goal was wider than its change (the
removed ids are still read and sorted), so it is narrowed, given a
bound of 1.2 times the base copy, and its fixture made reproducible by
script and manifest; ST-3's third trigger clause is met by the shipped
`--membership removed`, so six parks have fired, not five, and the
prune verb is scheduled at the league end; the parks the triage said to
build first each have a home or an explicit deferral; five sentences of
the synthesis had grown wider than their reports (dead code, the
invariants, the checker commits, what P6 has retired, the tracker
trigger) and are corrected in place, dated; two citations fixed.

Not taken: finding 10's wider suggestion to re-split the whole
synthesis into measurements, judgments and choices; the five sentences
were the ones that were wrong.

Assisted-by: Claude Fable 5.1 <noreply@anthropic.com>
````

**Notes**

1. **The subject line.** 146 characters with a verdict in it ("eleven
   findings, nine taken"), against the usual convention of about 50. The
   subject is what `git log --oneline` readers scan, so it carries the
   outcome. Report 3, section 3.8 covers how messages became the record.
2. **"in 28 read-only commands and 192,173 tokens".** The cost is
   measured (tokens are the unit of model compute), so a later reader
   can judge whether a review this deep is worth repeating.
3. **"two failed launches before one that ran; the recorded command is
   now the one that works".** Tooling friction fixed at its source; the
   working command is in `roadmap/README.md` today.
4. **"it led with the project's shapes of fault and put plain defects
   second, which would steer a reviewer toward prose".** A model
   reviewer follows the framing it is given; a prompt that leads with
   documentation faults gets documentation findings. Behaviour first.
5. **"instead of "one rule, never a patch per case" (the fix's shape is
   the fixer's and the owner's call)".** A house rule removed from the
   reviewer's prompt because it took a decision from the people who own
   it.
6. **"0b had delegated an intent-write rule to the worker against
   "rulings first"".** The reviewer caught a brief handing a worker a
   decision that was the owner's: a rule about writing user intent (the
   user's own annotations, the one state that cannot be refetched).
7. **"five sentences of the synthesis had grown wider than their
   reports".** Shape 1: a summary claiming more than its sources. The
   synthesis was a Claude session's; a model from another vendor,
   reading the same sources, caught it. That is the point of an
   outside review.
8. **"Not taken: finding 10's wider suggestion ..."** What was declined,
   and why, so a reader can judge the two findings not taken.
9. **"They are verbatim at roadmap/reviews/roadmap-a987d02f.md".** No
   longer in the tree: deleted at phase 0's close (`c5535d52`,
   2026-10-20) under the old policy that ruling (a) in 9.6 ended. It
   reads back with `git show c5535d52^:roadmap/reviews/roadmap-a987d02f.md`.

---

## 9.8 The owner's voice in the record

The owner writes few words into the record, and almost all of them are
quoted by agents, in commit messages or in `roadmap/README.md`'s "The
owner's rulings for this road". Ten, in date order, as each source has
them; typos are his.

1. **2026-09-07**, design note 14's opening line (the note's text is in
   the diff of `515bf034`, which deleted it): "I think we are ready to
   remove the ACQ_GGG training wheels." The request behind C88 (9.1).
2. **2026-09-09**, `e323aed4`, design note 16, "The owner's rulings on
   6.8 (2026-09-09, verbatim)": "omg yes--embedding the daemon sounds
   terrible compared to `acqd`. the path we took was an unplanned
   shortcut. time to grow up." The answer to "must every frontend be
   able to become the daemon"; it led to the separate daemon binary
   (report 4, section 4.9).
3. **2026-09-09**, the same note: "eliminate. there may be long-running
   tests in our future, be we will design that bridge appropriately when
   we come to it." On a long "frozen soak" test procedure, whose scripts
   the same commit deleted.
4. **2026-09-12**, `515bf034`: "agree to flip the default". The whole of
   the ruling that made the real API the default.
5. **2026-10-13**, `20a3f7f8`: "rulings first, findings after". His
   resolution of a contradiction an audit of the documents found
   between two sentences of the old "Working style" section; now a hard
   rule (9.2, note 12).
6. **2026-10-18**, `c0be5c5a`, in C116's first text: "I accept all 6. Go
   ahead." Six rulings drawn from the trade seat, C116 among them.
7. **2026-10-19**, `roadmap/README.md`: "I accept all your
   recommendations exactly as proposed with a loopback web UI." The
   post-alpha roadmap approved with one amendment: the GUI's first form
   is a local web page rather than a desktop app.
8. **2026-10-19**, `e01cd7f5`: "Yes, make this an alpha release." The
   commit's subject gives the reason: "no GUI yet and the feature sets
   untested in the wild, so not a beta".
9. **Under the 2026-10-19 rulings**, `roadmap/README.md`: "stick to Opus
   5.5 as much as possible, with Fable called in for complex analysis,
   synthesis, and design." The division of labour between models
   (report 5, section 5.2).
10. **2026-10-25**, `efc6e745`: "you are in the driver's seat … they are
    yours". On who owns the wording of two rulings; the ellipsis is the
    commit's, so the full sentence is not in the record. Report 4,
    section 4.14 takes its title from it.

Two more appear above: "I have reviewed the questions and approve all 4
recommendations." (9.6) and "Fix all four findings as proposed, then
close p0." (9.4), both 2026-10-26.

The pattern: the owner rarely writes the rule. Agents draft the
question and the recommendation; he answers in a line, sometimes with
one amendment ("with a loopback web UI"), sometimes with a reason
("time to grow up"); the agent quotes that line in the commit that
enacts it. The authority is his, the drafting theirs, and the quotation
is the proof of consent.

---

## How to read these

Aaron, these are the documents themselves, dense on purpose: each was
written for a reader, usually a model session, that would read every
word and nothing else. Read each source once for its shape (headings,
labelled tails, tables) before reading it for content; the shape is
what you would carry to another project. Use the notes as a glossary in
place, and follow a note's pointer to another report for the story
behind the artefact. Across all eight sections, look for three things:
every claim carries its evidence (a test name, a commit hash, a
measured number, a quotation); every document says what it did not
check; and every decision says whose it was. Where a source seems to
repeat itself, it is usually stating a rule where its reader will meet
it, because that reader cannot be assumed to remember.

## Reproduce

```sh
git show 515bf034 c0be5c5a e323aed4 20a3f7f8 e01cd7f5 efc6e745 c18eb602; git show 25d28b14:spikes/rust-playground/AGENTS.md; git show -s --format=%B 16ef9210 f1a29952 0ccafbbf; git log --grep='approve all 4' --format='%h %ad %s' --date=short; git show c5535d52^:roadmap/reviews/roadmap-a987d02f.md
```
