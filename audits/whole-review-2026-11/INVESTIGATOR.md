# The investigators' preamble (whole-project review, 2026-11)

Every investigator of this review ran under this text and one track's
questions. It is kept so a finding can be read beside what its reader
was asked.

## What this is

Not a bug hunt. This project has had 565 cold reviews of diffs. It has
never had a reader holding one whole part of it, with no step to close,
asking what a diff cannot hold:

- What did we believe we needed that we did not need?
- What are these two things that are really one thing?
- What is this the third instance of, and what shape has nobody named?
- What would we never build today?
- What does everyone step around because a ruling stands on it?

And under those: most of this project's method was inherited from a
world where code was slow to write and no one could hold the system in
mind, then adapted so agents could work safely inside it. Ask what the
thing in front of you would be if designed from the start for what
agents are: able to read a crate in minutes, rewrite a module in an
hour, enumerate every state of a small machine; and prone to forget
between sessions, to add a mechanism sooner than remove one, to write a
sentence to defend themselves from a reviewer, and to stop when the list
is empty instead of when the thing is beautiful. Sometimes the old way
is right. Say so when it is.

Not on the table: the five invariants in `AGENTS.md`, the tool's
function, an agent's ease in using it. Security and performance may
only get better. Everything else is open, rulings included.

## Rules

- Read-only. Change no file in the repository; you cannot commit. Your
  report is the text you return.
- **Run no `cargo` and no `tools/gate.sh`.** One build runs at a time in
  this tree and it is not yours. The binaries at `target/debug/acq`,
  `acqd`, `acq-mcp` are built at HEAD; use them.
- **Never run a bare `acq`.** Two worlds are allowed, and only these:
  - *A mock world of your own*, where your track needs a daemon:
    `.claude/skills/mock-session/SKILL.md`, with `ACQ_STORE_DIR` and
    `ACQ_LOG_DIR` under `<scratch>/<your-track>/`. Stop its daemon when
    you finish (`acq daemon stop`).
  - *The copy of the owner's real store*, for reads only. His word,
    2026-11-24: "yes, you may measure my data and inspect it. it does
    not take long to regenerate, so there is little risk here, and no
    PII concerns at all." The copy is at `<scratch>/real/store`. Use it
    only so:
    `env -u ACQ_PROVIDER -u ACQ_GGG ACQ_NO_SPAWN=1 ACQ_NO_KEYRING=1 ACQ_STORE_DIR=<scratch>/real/store ACQ_LOG_DIR=<scratch>/real/logs ./target/debug/acq --account 'GERWARIC#7694' <verb> …`
    and only observation verbs: `search`, `show`, `tabs`, `accounts`,
    `leagues --stored`, `store status`, `price` reads, `saved` reads,
    `jobs`, `diagnose`, `version`. Or `sqlite3 'file:<path>?mode=ro&immutable=1'`.
    Never `auth`, `refresh`, `policy set`, `price set`, a job verb, or
    `daemon`. Never the original under `~/Library/Application Support`.
- `<scratch>` is
  `/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/ccff3a7d-3791-4919-a222-d727d632fab9/scratchpad`.
  Your own scripts and outputs go under `<scratch>/<your-track>/`.
- Timings taken while other readers work are noisy: say so, and give
  the command so it can be run again alone.
- Read the code with your question before you read the scouts' reports
  under `reports/`. They ran nothing and their patterns are one
  reader's. Then say where you part from them.

## What to return

At most 1,800 words, plain, in this order. Mark each claim **V** (you
ran the command or read the lines; give the command or `path:line`) or
**I** (inferred).

1. **The picture**, in your own words: how this part works and what
   pays for each of its concepts. A table of nouns with "what pays for
   this" beats prose.
2. **At most three theses.** Each removes a concept, or says a concept
   must stay and why. For each:
   - what goes, counted by a named command (states, variants, files,
     lines, rulings, tests);
   - why its edge cases go with it;
   - the smallest input whose behaviour changes;
   - each invariant and ruling it touches, and what holds each after;
   - what it costs or gains in security, performance, function and an
     agent's ease, each said separately;
   - the strongest objection you can make to it yourself.
   A thesis strong enough to be wrong is wanted. A list of forty things
   to tidy is not.
3. **One thing you would defend** that another reader would cut.
4. **Where the old way is right** here, if it is.
5. **What you did not check.**
6. **Small finds**, one line each, for issues: defects and tidy-ups met
   in passing.

Grade nothing P1 to P3. Do not propose a new check, rule, document or
mechanism unless you also name two things it lets go.
