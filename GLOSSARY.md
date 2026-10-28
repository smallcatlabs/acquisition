# Glossary

The project's working nouns, one plain line each, as the tree uses
them. Read on demand: when a document, a ruling or a commit message
uses one of these words in a sense that ordinary English does not give
it. A word with two live senses has a line for each. A new working
noun gets a line here when a second document starts to use it.

## Door

"Door" has carried several meanings. Each has its own word now, and a
new text uses that word rather than "door".

- **connect policy** (the most common sense of "door"): how a frontend
  reaches the daemon. The *use* policy may start or replace a daemon,
  the *observe* policy never does; the human and the agent policies are
  the two a frontend is built with (`crates/acquisition-client/src/client.rs`).
- **shared opener**: a call that opens a store, picks an account,
  compiles a plan or connects. Only the verbs crate makes these calls;
  `tools/arch-check.sh` refuses a frontend that does (C46, every
  frontend a Rust adapter).
- **admission**: the daemon judging a client's job kind and parameters
  before the job exists; a refused kind is "refused at admission".
- **accessor**: a general, raw-path reach into an item's stored body.
  The search offers none: a missing reach becomes a named field (C101,
  fields not accessors).
- **the write gate** and **the read-only open**: the store's two ways
  in for intent, the one where an annotation is typed and refused, and
  the one that never migrates.
- **the seat's logger**: the one script a seat's blind agent issues
  every command through, which logs it and prints only the head of a
  long output.
- **path**, in the idiom "two doors to one task": two code paths to
  one task, which is one too many.

## The rest

- **basis** (of a search answer): the label every search answer
  carries, naming the store, the account, the snapshot, the intent
  revision and the build it was derived from.
- **basis** (of a quote): what a daemon quote names as its ground, the
  work it prices echoed back, with its observation time, provider and
  account.
- **breaker**: a deliberately broken copy of a document or a tool,
  run to prove a check refuses it by name; the breaker scripts under
  `tools/` hold them.
- **brief**: the committed document a worker or a seat reads for one
  step or one run: its goal, what to read, what done means.
- **close** (a step's): the commit that checks the brief's done-when
  against the tree, answers what the worker raised and writes the
  ledger line (`roadmap/README.md`, "A step").
- **close** (a phase's): the commit that folds the phase's ledger lines
  into one and adds the `HISTORY.md` line.
- **facet**: one of the two things a league's sync policy covers, its
  tabs or its characters; in the counts view, one field counted on its
  own rather than crossed with another.
- **fold**: what a phase close does to its ledger lines (many into
  one); in the web form, the turning of submitted fields into one
  trade request.
- **golden**: a recorded wording a frontend test compares against,
  regenerated under `ACQ_UPDATE_FIXTURES=1`, and naming the facts a
  reader must find (C122, one harness for every test).
- **home**: the one place a kind of record belongs: a ruling in its
  area file, a finding in its test or commit, a refresh token in the
  keyring (`AGENTS.md`, "Where things live").
- **journal**: the daemon's one JSON line per real send to GGG (method,
  route, status, rate-limit headers, never a token or a body); the
  surface tests read for what was sent.
- **knob**: an environment variable a user may set, such as
  `ACQ_STORE_DIR`; each has a row in the README's knob table.
- **last** (a round): the final fix round of a step, after the tail.
- **ledger**: a table of dated rows read from its newest end:
  `RUN-LEDGER.md` for live runs, `SEAT-LEDGER.md` for seats,
  `roadmap/LEDGER.md` for closed steps; `search/LEDGER.md`, the search
  slice's closed record, is looked up by its row instead.
- **ledger** (the job ledger): the daemon's persisted job queue on
  disk, which a frontend reads when no daemon answers.
- **mirror**: the jobs table in the world's database, written at every
  job state change so the in-memory queue survives a restart.
- **oracle**: a test assertion written from the rules rather than from
  the code, run over every test daemon's journal at teardown.
- **park**: a "Not doing" line at the foot of an area's rulings file,
  declined until a named trigger.
- **parked** (a job): one turned away before its send by a halt, an
  exit or a cancel.
- **pin** (a test): to hold a behaviour or a ruling with a test; every
  ruling names what pins it, or says nothing does yet.
- **pin** (an input): a fixed version of an outside source, such as a
  sibling clone at a commit, moved only on purpose.
- **place**: where an item sits (league, tab, character, container),
  read from the store's facts rather than the item's body, and printed
  with how old the fetch is.
- **rail**: a safety mechanism in the daemon for live runs (the
  tripwire, the send ceiling, the journal, the block mark) that can
  only refuse or record a send, never add one.
- **realm**: the level above the league (pc, xbox, sony, poe2); the
  search takes it as its scope rather than as a term.
- **round**: one pass of review and fixes over a step (its fixes, then
  the tail, then the last); in a sitting, one batch of site searches.
- **route** (in search): the runnable command that reaches the items
  behind a count.
- **route** (in the daemon): the endpoint key a send is paced and
  journaled under.
- **route** (as a verb): to move a text to its home.
- **rung**: one live run on the ladder that proved the rate limiter,
  rehearsed first against the mock; in the principles, a layer of the
  tree, knowledge sitting at the lowest one that holds it.
- **scope** (in search): the realm a query runs over, stated apart
  from the query's terms.
- **scope** (in an annotation): the first part of its address, saying
  what kind of thing its key names, an item or a tab.
- **seat**: a run in which a blind agent, reading only a brief and the
  tool's own words, drives the released tool through the seat's
  logger and reports where the tool stopped it.
- **sitting**: a session in which the owner opens prepared trade-site
  searches in a browser and the recorder captures what the site
  answers.
- **slice**: a closed body of work (the refresh, pricing, search and
  honesty slices) with one `HISTORY.md` line.
- **step**: one brief, build, gate, commit, review and close in the
  roadmap, with one ledger line; inside a slice's record, a numbered
  stage of its build.
- **tail** (a round): the fix round after a step's first fix round.
- **tail** (of a ledger): its newest rows, the ones read first.
- **world**: one store directory, which one daemon locks and serves and
  whose socket is derived from it; a mock world is one made under
  scratch for an agent's session.
