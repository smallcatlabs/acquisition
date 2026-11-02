# transport-seam-1b — the transport crate (#160; step 1's last round)

The first cut, written by the orchestrating session (Opus) on 2026-11-01
at `baf625c9`, every line number below from it. Step 1
(`roadmap/briefs/transport-seam-1.md`) was built as `6ca5b9a3..42c9a7b7`
and its fix round as `3365183b..7c1c9626`; the fix round's cold review
(`roadmap/reviews/transport-seam-1-fixes-7c1c9626.md`) found check 13
missing `pub(crate) extern crate r#reqwest as web;`, the first review's
fault (a spelling matched, not the name resolved) in a smaller form. The
orchestration's rule sent it to the owner rather than to another round
(#160, its packet in the issue's comment); he ruled "(b) Own crate",
entered at `baf625c9` before any code: C1 amended (the daemon links the
protocol crate, the store and the transport crate; the transport crate
alone links an HTTP client, links no workspace crate, and only the
daemon links it), the crate's row in `AGENTS.md`, the file's budget
16,600 bytes, and the README's rulings list. This round builds it, and
with it step 1 is ready to close. To be reviewed cold before the worker
reads it. Opus builds it: the worker that built step 1, which holds its
context. Reviewed cold once (`roadmap/reviews/transport-seam-1b-brief-77240c34.md`:
buildable as drawn, one citation nit, fixed in a documents-only cut with
no further review).

## Goal

When this round is done, the one production transport and its one
private `reqwest::Client` live in a crate of their own,
`crates/acquisition-transport`, beside the neutral types the choke
point and the test transports share (`Prepared`, `Answer`, `Transport`,
`Pending`); the daemon links that crate and has no normal or build
dependency on reqwest, so no source in the daemon's production build can
name reqwest under any spelling, which the compiler refuses rather than
a reader of text; and check 5 of `tools/arch-check.sh`, which reads
cargo's resolved graph, holds the edges: reqwest enters the workspace's
production graph through the transport crate alone, the transport crate
links no workspace crate, and no workspace crate but the daemon links
it. Check 13 retires onto those edges. Nothing a user, a journal line,
the wire or a test meets changes from step 1 as built.

What the ruling took, said again so the worker does not try to undo it:
the production transport's constructor is the crate's public API, so
"only the choke point builds it" is held by review (and by `choke.rs`
being its one caller), no longer by the module tree. No check of a
spelling is to be added for it.

## Read

- `roadmap/README.md`, the rulings list's last entry (#160);
  `AGENTS.md` C1 and the crate table (the transport crate's row);
  `decisions/network.md` C161 (26: its *Pinned* names check 13 today),
  C130, C150; `AGENTS.md` C122 (one test binary per crate, the layout
  check 8 holds), `decisions/daemon.md` C84 and the `features` stage (`tools/gate.sh` 33–37,
  164–227: the plain build's and the test build's feature graphs
  compared package by package).
- #160 and its comment: the three spellings, the two shapes, what (b)
  does not hold.
- `crates/acquisition-daemon/src/ratelimit/transport.rs` whole (192
  lines at `42c9a7b7`, unchanged since): the module doc (1–38), the
  imports and `use super::{CONNECT_TIMEOUT, REQUEST_TIMEOUT}` (40–45),
  `Pending` (49), `Prepared` (54–80, `Prepared::new` writing
  `crate::provider::USER_AGENT`), `Answer` (84–88), `Transport` (92–95),
  `Production` (98–108) and its `send` (110–133), `build_client`
  (135–151, `crate::provider::USER_AGENT` on the client), the tail test
  `the_production_transport_sends_the_prepared_user_agent_not_its_default`
  (153–192, reading the request with `crate::mockggg::read_request`).
- `crates/acquisition-daemon/src/ratelimit/choke.rs`: the module doc
  (1–8), the child declaration and re-exports (9–15, `#[path =
  "transport.rs"] mod transport;`, `pub(crate) use transport::Pending;`,
  `pub(crate) use transport::{Answer, Prepared, Transport};`), the field
  (206–209), the constructors naming `transport::Production::new()`
  (255, 273), `with_transport` under `#[cfg(test)]` (283–294),
  `CONNECT_TIMEOUT` and `REQUEST_TIMEOUT` (their definitions, read by
  `build_client` through `super::`).
- `crates/acquisition-daemon/Cargo.toml` 30–40: `reqwest` at 35
  (`default-features = false`, `json`, `rustls-tls`), `http` beside it;
  every test under the daemon's `src/` that names reqwest and so needs
  it as a dev-dependency once the normal one goes (`grep -rn reqwest
  crates/acquisition-daemon/src --include='*.rs'`, outside
  `transport.rs`: `mockggg.rs:1165` and `rails.rs:1423` in tail test
  modules, the door's browser, the callback GETs in `log.rs`, `exit.rs`
  and `login.rs`, the account bait in `queue.rs`, `responses.rs`'s status
  type, and a comment in `queue.rs`).
- `tools/arch-check.sh`: the header (2–60; item 13 at 49–54), check 5's
  helpers (`forbid` 262, `allow` 281, `only_self` 295, the `table` they
  read, built from `cargo metadata`) and its rule lines (310–323), check
  13 (its section from the line headed `---- 13.` to the check's end),
  the shared `tail_reader` check 12 now runs. `tools/arch-check-breakers.sh`:
  the dependency cases (the `adddep`, `forbid` and `only_self` cases
  under "the code"), check 13's cases (from 319, `TRANSPORT=`), the
  `files` list that every restore compares (its line naming
  `ratelimit/transport.rs` at 56).
- `.cargo/mutants.toml` 1–20: `examine_globs` (the daemon's
  `ratelimit.rs`, `ratelimit/*.rs`, `gate.rs`), "each mutant is judged
  by the daemon package's own tests", `test_tool = "nextest"`;
  `.github/workflows/weekly.yml` 30–70 (cargo-mutants 27.1.0, four
  shards). The production transport's real-wiring tests that judge its
  mutants live in the daemon's spec (the table's wire run, the 3xx, the
  truncated bodies) and in the moved user-agent test.
- `crates/acquisition-protocol/Cargo.toml` (a library crate's manifest
  in the workspace's form: the workspace's package keys, `autotests`,
  its doc comment), `Cargo.toml` (the workspace's `members`), and a
  crate's `src/lib.rs` doc naming its rulings file at the top (`AGENTS.md`,
  "The workspace").
- The words that name `transport.rs` or check 13 today (`git grep -n
  "transport\.rs\|check 13"` outside `roadmap/reviews/` and the two
  dated briefs): `choke.rs` 9–12, `transport.rs:30`, `decisions/network.md:26`,
  `roadmap/README.md` (the rulings entry, which stays, and "Current
  step"), `tools/arch-check.sh` and its breakers.

## Input

- The step's tree at `baf625c9`, its gate green on `7c1c9626` by the
  worker's run and the orchestrating session's (1415 tests), the
  breakers 107 ok, 0 failed by the worker's run.
- No failing input: the round moves code and changes no behaviour; its
  evidence is the gate, the `features` stage, the moved tests and the
  breakers.
- The last push's CI: re-read at the round's start (`gh run list
  --branch main --workflow "Gate (Linux)" --limit 1`, the same for
  Windows), named in the report.

## Do

Two commits, each gated on its own tree.

1. **The crate, the edges, check 13 retired** (one commit: the check
   cannot retire before the edges hold it, and the edges cannot hold
   before the crate exists).
   - `crates/acquisition-transport`, a library in the workspace's form,
     a member of the workspace: `src/lib.rs` holding what
     `ratelimit/transport.rs` holds, moved whole, its module doc headed
     by C161 and naming its rulings file (`decisions/network.md`) at the
     top. It reads nothing of the daemon's: the user-agent and the two
     timeouts are given to it (`Production::new` takes the user-agent
     and the two durations, or a small config; `Prepared::new` takes the
     user-agent), so the one constant stays `provider::USER_AGENT` and
     the two durations stay `choke.rs`'s, each read where it is today.
     Its dependencies: `http` and `reqwest` with reqwest's features as
     the daemon declares them today, so the graph builds nothing new; a
     dev-dependency on tokio for its own test. Its one test, the
     distinct user-agent case, moves to its tail test module, reading
     the request off the socket with a few lines of its own (the crate
     links no workspace crate, so not `mockggg::read_request`).
   - The daemon: depends on `acquisition-transport`; its normal
     `reqwest` dependency goes, and reqwest is a dev-dependency for the
     tests that drive a real socket (Read), declared with the same
     features so the `features` stage stays green (a dev-dependency with
     other features splits the test build's graph, C84);
     `ratelimit/transport.rs` and the `#[path]` child go; `choke.rs`
     names `acquisition_transport::{…}` and is the one production file
     that names `Production` (`grep -rn "Production" crates/acquisition-daemon/src`
     outside tests prints `choke.rs` alone, said in the message, not a
     check); the re-exports the crate's test support uses
     (`Pending`, `Prepared`, `Answer`, `Transport`) stay where the spec
     and step 2's transports find them.
   - Check 5's edges, in its helpers' form (a helper added only if none
     of `forbid`, `allow`, `only_self` says it): reqwest is a direct
     normal or build dependency of `acquisition-transport` alone, and no
     other member reaches it in a normal or build edge except through
     that crate (the daemon's dev-dependency the one dev edge, named in
     the rule's words); `acquisition-transport` links no workspace crate;
     no member but the daemon links `acquisition-transport`, at any
     depth. Each refusal says the rule and what to do (C161, C1). The
     header's list says check 5 holds these edges.
   - Check 13 removed whole: its section, its header item (the numbers
     of 10 to 12 kept, 13 named retired in one line so a citation of it
     still finds its fate), its breakers and the `files` entry for
     `transport.rs`. Check 12's shared `tail_reader` stays as check 12
     runs it. The message carries `retired check 13 of
     tools/arch-check.sh and its breaker cases; held by check 5's
     transport edges and by the daemon having no normal dependency on
     reqwest, which the compiler holds`.
   - Breakers for the new edges, in the dependency cases' form: a
     second member declaring reqwest as a normal dependency; the daemon
     declaring reqwest as a normal dependency again; a frontend or the
     client linking `acquisition-transport`; the transport crate linking
     a workspace crate; the tree as it stands passing. Each seen refused
     by the new rule (the report names each). The three spellings #160
     names (the workspace alias, the root-group alias, the raw
     identifier) staged once each against this tree, by hand or as
     breakers: each now fails to compile or is refused by check 5,
     never passes; the report says which.
   - `.cargo/mutants.toml`: the crate's source among `examine_globs`,
     judged by the daemon's tests as well as its own (the spec holds the
     production transport's real-wiring tests), by cargo-mutants
     27.1.0's own option for testing another package, read from its
     documentation and named in the message; `cargo mutants --list`
     showing the crate's mutants is the evidence (no run).
   - C161's *Pinned* names check 5's transport edges and their breakers
     in place of check 13, keeping the wire table's expiry rows for step
     3. `choke.rs`'s module doc and field comment say the transport is
     the transport crate's.

2. **The words and the step's state.** `transport-seam-1.md` gains one
   dated line under its provenance: Do 2's module-tree privacy and Do 4's
   check 13 settled by this round (#160), so a reader of that brief
   finds where they went. `GLOSSARY.md`'s "transport" line names the
   crate if it names a file. "Current step" says step 1's last round
   has landed and its review is pending.

3. `tools/gate.sh` on each commit's tree, its status read on its own
   line; `tools/arch-check-breakers.sh` once on Do 1's tree, its last
   line in the report; every golden, `CLI-REFERENCE.md`,
   `MCP-REFERENCE.md` and `README.md` byte-identical to `baf625c9`.

## Done when

- `crates/acquisition-transport` is a workspace member whose
  `Cargo.toml` names `http` and `reqwest` (reqwest's features as the
  daemon's were) and no workspace crate; its `src/lib.rs` holds
  `Prepared`, `Answer`, `Transport`, `Pending`, the production transport
  and `build_client`, and the distinct user-agent test.
- `crates/acquisition-daemon/Cargo.toml` has no `reqwest` under
  `[dependencies]` and one under `[dev-dependencies]` with the same
  features; `crates/acquisition-daemon/src/ratelimit/transport.rs` does
  not exist; `choke.rs` declares no `#[path]` child.
- `tools/arch-check.sh` has no check 13 section and its check 5 holds the
  three transport edges; `tools/arch-check-breakers.sh` has no check 13
  case and has the new edge cases; the retirement line is in Do 1's
  message, with the three spellings' fate.
- `.cargo/mutants.toml` examines the crate's source and names how the
  daemon's tests judge it.
- C161's *Pinned* names check 5's transport edges; `AGENTS.md` is as
  `baf625c9` left it.
- No golden, reference or README line changed over the round.
- The gate's exit status per stage on each commit's tree, the breakers'
  last line and `cargo mutants --list`'s count for the crate are
  execution evidence, in the worker's report; the orchestrating session
  re-runs the gate on the round's tree.

## Not in scope

- Anything of step 2 or 3 (the brief of step 1 lists them), the
  builder's transport option among them.
- A check that `Production` is constructed only in `choke.rs`: the
  owner's ruling leaves that to review (Goal).
- Moving the daemon's own tests off reqwest (the browser's GET, the
  callback GETs): they keep a real socket and the dev-dependency.
- `AGENTS.md`, C1 and the budget: entered at `baf625c9`; the round
  changes neither.

## Rules

`TEMPLATE.md` and step 1's brief's Rules, which this round keeps: the
main tree on `main`, shared; never push; signed commits naming this
brief and #160; no daemon against the real provider; the gate's status
on its own line; kill only this session's pids; an experiment undone
from a scratch copy compared with `cmp`. The round changes non-test
code, `tools/` and a manifest, so its commits are reviewed cold as one
range.

## Report

`TEMPLATE.md`, and: the gate per stage for each commit; the breakers'
last line; each new edge case and each of the three spellings seen
refused or not compiling, and how; `cargo mutants --list`'s count for
the crate and the option named; the last push's CI. Raised for review,
at least: anything in the daemon that had to name `Production` outside
`choke.rs`; any feature the `features` stage found split.
