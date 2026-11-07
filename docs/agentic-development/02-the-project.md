# 2. The project

What Acquisition is, the one constraint that governs every decision,
how the Rust rewrite is built, and why a game inventory tool turned
out to be a good place to learn agentic development.

## 2.1 What it does

Path of Exile is an online action role-playing game whose economy
runs on player-to-player trade. A serious player accumulates
thousands of items across hundreds of "stash tabs" on the game
company's servers. Grinding Gear Games (GGG) exposes an HTTP API,
under OAuth, that lets a tool download those tabs and the player's
characters.

Acquisition downloads your stash tabs and characters to your own
machine, lets you search them offline, lets you set prices on what
you want to sell, and renders the forum "shop" pages that list it.
That is the whole product, and it has been the whole product since
2014, when Ilya Zhuravlev wrote the original C++/Qt desktop app. The
rewrite described in this series keeps that scope and changes the
shape: instead of one desktop window, it ships three executables.

| Binary | What it is | Who uses it |
| --- | --- | --- |
| `acqd` | a local daemon, the only process that ever talks to GGG | nobody directly; the two frontends start and share it |
| `acq` | a command-line tool with about thirty verbs | a person at a terminal |
| `acq-mcp` | a Model Context Protocol server over stdio, 25 tools | an AI agent such as Claude |

A browser view (`acq web`) exists as a verb of `acq` and renders the
same documents on a loopback port. A full GUI is planned, not built.
The original C++ app lives on in its own repository and the owner
maintains both.

For the reader with a frontend background: the interesting
architectural claim is that the three surfaces share one semantics,
and the project enforces that structurally rather than by review.
Section 2.4 says how.

## 2.2 The constraint that governs everything

GGG's API is rate limited, and the limits are policed per OAuth
client registration. A tool that floods the API, retries through a
block, or looks like a bot can lose its registration, and with it
every user. The original Acquisition has held a registration since
OAuth arrived, and the rewrite runs under that same registration with
the same user-agent string.

The project treats this relationship as its most valuable asset, and
you can see the consequences everywhere:

- **One sender.** Every request to GGG passes through one process
  (`acqd`), one rate-limit choke point and one global gate. A code
  path that sends any other way is defined as a critical bug, however
  reasonable it looks locally. This is invariant 1 in the agent entry
  file, and a shell script checks the dependency edges that make it
  structural.
- **Headers are truth.** GGG returns rate-limit headers on every
  response. Local rate-limit state is treated as a prediction that the
  headers correct, never the reverse.
- **Never retry through a Cloudflare block.** The one known incident
  is recorded as a numbered claim in a ground-truth document, and the
  recognition signatures are pinned.
- **Agents are allowed, through the gate.** GGG permits agent use that
  respects the rules. The project's answer is not to forbid AI agents
  but to make the daemon the only door, so that an agent, a CLI user
  and a GUI are indistinguishable on the wire.
- **The default is dangerous, so the default is guarded.** The real
  provider is the default build. Every agent session is required to
  run in a "mock world": a stand-in GGG that serves made-up stashes,
  under an isolated store directory. A bare `acq` opens the owner's
  real store, so an agent never runs one.

A reader from a large organisation will recognise the shape: a
third-party dependency whose terms of service are a business risk,
turned into a set of non-negotiable engineering properties with
checks behind them. What is unusual is how early and how explicitly
it was written down, and that the people being constrained are mostly
AI sessions rather than engineers.

## 2.3 The data model in one paragraph

The store holds four layers, each with exactly one authoritative way
to change it. **Facts** are what GGG sent, written only by the
store's ingest path. **Intent** is what the user decided (prices,
notes), written only through an annotation API under revision
compare-and-swap, in one file per account, and never deleted by any
fact-side event. **Derivations** (search indexes, plans) are
reproducible from declared inputs. **Effects** (requests to GGG) go
only through the daemon, which is deliberately blind to intent and
runs no schedules of its own. The practical consequence is the one
that matters to a user: a sync can never clobber a price you set, and
the only irreplaceable local state is the small annotation file. The
rest is refetchable.

## 2.4 How the workspace is built

The workspace is nine Rust crates with a strict dependency order. The
rule is written as a ruling and enforced by a script that fails the
gate if an edge appears that should not.

```
                 acquisition-cli (acq)     acquisition-mcp (acq-mcp)
                          \                     /
                         acquisition-verbs  (account, world, "door")
                        /       |        \          \
         acquisition-client  acquisition-plan  acquisition-search
                   |                 \            /
         acquisition-protocol      acquisition-store
                   |                      |
                acquisition-daemon (acqd) — links protocol and store only
```

The properties the shape buys:

- **Shared semantics live in Rust once.** A CLI verb and an MCP tool
  call the same function in the verbs, plan or search crate. A
  frontend is an adapter: argument parsing, rendering, exit codes. A
  frontend that evaluates anything for itself is defined as "a second
  semantics" and refused by the gate's grep. For anyone who has
  watched a web client and a mobile client drift apart on validation
  rules, this is the familiar problem solved at the dependency graph.
- **Two surfaces, no third.** A frontend may consume exactly two
  things: the daemon protocol and the store's read API. Anything else
  is a protocol or store change, ruled first.
- **Jobs, not calls.** API work is submitted as a job with an id, a
  state and a priority, because a rate-limit wait can reach five
  minutes and a blocking call cannot represent that across three
  frontends.
- **Plans are binding.** Refreshing a league compiles a plan, a
  serialisable list of the exact requests to make, which the user
  reads before it runs. Applying executes exactly that list or a
  subset, never an addition. The agent-facing version of this is the
  same: an agent asks for a plan, shows it, and applies it.
- **The daemon is the only GGG sender and links almost nothing.** It
  never sees intent, never links the search, never schedules work.
  The blast radius of a bug anywhere else cannot reach the wire.

The test strategy follows the same logic. Tests pin behaviour at the
two boundaries: the GGG-side contract is a "send journal", one JSON
line per real send, and tests assert against it with "oracles" written
from the rules rather than the code; the frontend-side contract is the
protocol. Wording a user sees is a "golden" that names the facts a
reader must find, so a blind regeneration fails. A behaviour is pinned
once, at the lowest crate that holds it.

## 2.5 Where the documentation went

A frontend engineer opening this repository will notice that the
documentation is mainly for agents, and the README says so. The
entry file `AGENTS.md` is the one document every session loads, held
to a byte budget. Under it:

| Document | Holds | Analogue in a large organisation |
| --- | --- | --- |
| `decisions/<area>.md` | numbered rulings (131 at the time of writing), each with a *Why* and what test *pins* it | architecture decision records, but mandatory and cited by id |
| `REVIEW-CHECKLIST.md` | faults that came back, by area, read before reviewing | postmortem action items that survived |
| `roadmap/README.md`, `roadmap/briefs/` | the plan, and one brief per step a session runs from | the planning doc and the ticket, merged |
| `roadmap/reviews/` | the external reviewer's findings per commit | code review threads, kept |
| `docs/design/*-ground-truth.md` | numbered facts about GGG's behaviour, observed | the vendor integration wiki page, but with evidence |
| `RUN-LEDGER.md` | one row per run against the real API | the deploy log |
| `GLOSSARY.md` | the working nouns | ubiquitous language |
| `.claude/skills/` | procedures that have run twice and repeated a trap | runbooks |
| `CLI-REFERENCE.md`, `MCP-REFERENCE.md` | generated from the binaries' own help text | API reference, but a test fails if it drifts |

Report 5 explains how these are used; report 6 argues about whether
the weight is worth it. For now the point is that the project chose
to make its knowledge legible to a stateless worker, and that this
choice shaped everything downstream.

## 2.6 Why this was a good laboratory

Three features made this project a better place to learn agentic
development than most:

- **A hard external constraint with real consequences.** The GGG
  relationship meant "move fast and break things" was never an
  option, so the question became how to move fast without breaking
  the one thing that mattered. Most of the process in report 5 is an
  answer to that question.
- **A twelve-year reference implementation.** The C++ app is the
  specification: every feature has a known-good behaviour to compare
  against. That is also a trap, since the C++ tree is GPL with many
  authors and the Rust tree is MIT or Apache; the relicense audit in
  report 4 is what the trap cost.
- **One owner.** There was no team to coordinate, which meant every
  process mechanism in the repository exists because the owner or an
  agent found it necessary, not because an organisation required it.
  That makes the record unusually clean evidence of what agentic work
  actually needs.

## 2.7 By the numbers

All figures are at commit 10c40dc2 (2026-10-26); report 3 gives the
derivations and the commands that reproduce them.

| Measure | Value |
| --- | --- |
| Commits in the repository, 2014 to 2026-10-26 | 3,840 |
| Commits in the four AI months, July to October 2026 | 2,351 (the twelve years before: 1,416) |
| Commits since June 2026 by the owner | 2,350 of 2,358 |
| Commits since April 2026 carrying a Claude trailer | 2,076 of 2,371 (88%) |
| Days from the first Rust file to the first release | 71 (2026-08-09 to 2026-10-19) |
| Rust crates / lines of Rust / share that is test code | 9 / 165,105 / about 58% |
| Markdown in the tree | 153 files, 2.24 MB, against 6.58 MB of Rust |
| Rulings in the area registries (highest id C147) | 131 |
| Briefs / review files / skills | 35 / 10 / 6 |
| CLI verbs / MCP tools | about 30 / 25 |
| Open issues / closed | 73 / 13 |
| Known external users at alpha.1 | none |
