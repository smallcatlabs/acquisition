# Acquisition — agent entry point

You are at the root of `smallcatlabs/acquisition`, branch `main`: a
Cargo workspace implementing Acquisition. `README.md` is the users'
document. The C++ app is its own repository, `acquisition-qt`
(GPL-3.0); nothing here derives from it (C110). History before the
root swap is under `spikes/rust-playground/` (tag `playground-end`).

This is the one file every session loads. The owner (Tom) holds the
boundaries: the invariants, the protocol, the core API surface and
every ruling. Agents own internals.

## What Acquisition is

Acquisition is a Path of Exile inventory tool, rewritten from C++/Qt to
Rust under the same GGG OAuth registration as the existing app. A local
daemon owns all GGG API traffic; thin frontends share it: a CLI, an
MCP server, and a GUI to come. The GGG relationship (the OAuth
registration, the rate-limit standing) is the project's most valuable
asset, and every design decision defers to protecting it.

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
  starts a daemon. A `--workspace` test run writes the same `acqd`,
  `acq` and `acq-mcp` as that build, held by the gate's last stage, so
  no build is owed after a test run; a `-p` run that leaves out the
  daemon's package writes no `acqd`. Never run the live drivers, a bare
  `acq` or a second gate while the gate runs.
- **The gate decides.** Every change keeps `tools/gate.sh` green, and a
  commit is gated on its exit status, never on a filtered grep.
- **No id in user-facing text** (C116): help, refusals, reasons and
  notes say the rule in words.
- **Tests pin behaviour at boundaries, never mechanisms.** The GGG-side
  contract surface is the send journal (`TESTING-NOTES.md`); the
  frontend-side surface is the protocol (C85).

## The workspace

| Crate | Holds | Rulings |
| --- | --- | --- |
| `acquisition-protocol` | the daemon contract shared by frontends | `decisions/daemon.md` |
| `acquisition-daemon` (`acqd`) | the only GGG sender: daemon, limiter, gate and providers | `decisions/daemon.md`, `decisions/network.md` |
| `acquisition-client` | the IPC client, daemon discovery and connection policies | `decisions/daemon.md` |
| `acquisition-store` | facts, intent and world storage, neutral read snapshots, the search read (`corpus.rs`) and the world (`world.rs`) | `decisions/store.md`, `decisions/daemon.md` |
| `acquisition-plan` | offline refresh planning and pricing | `decisions/plans.md`, `decisions/pricing.md` |
| `acquisition-search` | item search over the store's read API: the query language, the item derived from its stored body, the answer; its working directory is `search/` (read its README first) | `decisions/search.md`, `decisions/trade.md` |
| `acquisition-verbs` | the verbs that need the daemon and the store together, and every frontend's account, world and door | `decisions/frontends.md` |
| `acquisition-cli` (`acq`) | clap, rendering and `acq web`; generated help `CLI-REFERENCE.md` | `decisions/frontends.md` |
| `acquisition-mcp` (`acq-mcp`) | the stdio MCP frontend, sharing semantics through plan and search; generated tools `MCP-REFERENCE.md` | `decisions/frontends.md` |

Before touching an area, read its rulings file, its section of
`REVIEW-CHECKLIST.md`, and the module doc you will change; a crate's
`src/lib.rs` doc (a frontend's `src/main.rs`) names its rulings file at
the top, and `python3 -I tools/bothy.py pack --path <file>` prints what
to read before touching a file. Before running `acq` or driving
`acq-mcp`: `CLI-REFERENCE.md`, `MCP-REFERENCE.md`, or `--help`.

## Cross-cutting rulings

The rulings every area must know. Each area's own are in its file, one
bullet per ruling in the form `tools/bothy.py`'s docstring states;
tests and docs cite rulings by id.

- **C1 — The workspace is library-centric and the daemon is its own artifact.** `acqd` is the only GGG sender and links the protocol crate and the store, nothing else; the store links none of the workspace crates; the protocol crate links serde alone; the verbs crate holds what needs the daemon and the store together and links client, protocol, store, planner and search; a frontend links those and the verbs crate as it needs them, never the daemon; nothing below the verbs crate links it, and neither the daemon nor the planner links the search. *Why:* shared logic is written and tested once, and the edges are refused by a check, not held by discipline. *Pinned:* `tools/arch-check.sh` (the edges).
- **C2 — The daemon owns shared state; clients talk to it over local IPC.** *Why:* invariant 1 becomes structural. *Pinned:* nothing yet.
- **C4 — API requests are jobs, not calls.** A job has an id, a state and a priority. *Why:* a rate-limit wait can reach 300 s, which a blocking call cannot represent across three frontends. *Pinned:* nothing yet.
- **C12 — A frontend consumes exactly two surfaces: the daemon protocol and the store's read API.** A third channel is a protocol or store change, ruled first. *Why:* the contract stays in two places, enforced by what a frontend links. *Pinned:* `acquisition-protocol/tests/wire.rs`.
- **C14 — Agent traffic against GGG is allowed; the daemon is the single gate.** *Why:* GGG permits agent use that respects the API rules, and one gate every client passes through is the only enforceable line. *Pinned:* `acquisition-mcp/tests/ggg_refusal.rs`.
- **C15 — Tokio and reqwest.** Core exposes `async fn`; frontends provide the runtime. *Why:* the ecosystem default; a core with no runtime lets each frontend own its own. *Pinned:* nothing yet.
- **C34 — Four layers, each with one authoritative mutation path: facts through the store's ingest, intent through its annotation API, derivations reproducible from declared inputs, effects through the daemon.** The daemon is blind to intent and creates work only in service of client-submitted work: no schedules, no policy execution. *Why:* a sync can never clobber intent, structurally. *Pinned:* `tools/arch-check.sh` (the daemon, the protocol and the client never name the intent API).
- **C35 — Annotations are the only irreplaceable local state.** One per-account file named by the account uuid, keyed on stable GGG ids, written only through the store crate under revision compare-and-swap; no fact-side event deletes intent; backup is a store-managed snapshot. *Why:* facts are refetchable; intent has no server to refetch from. *Pinned:* the `c35_` tests in `annotations.rs`.
- **C38 — A plan is a serializable, immutable authorization envelope, and plans are binding.** Applying executes exactly the listed actions or a subset, never an addition; new facts produce a new plan. *Why:* reviewed work stays exact. *Pinned:* `acquisition-cli/tests/apply_loop.rs`, `acquisition-cli/tests/plan_json.rs`.
- **C46 — Shared semantics live in Rust, and every frontend is a Rust adapter** (clap, `rmcp`, a webview whose Rust backend is the adapter). A surface that evaluates for itself is a second semantics, ruled first. *Why:* this is what makes "built once, inherited by every frontend" true. *Pinned:* `tools/arch-check.sh` (the frontend door grep: no frontend opens a store, picks an account or compiles a plan itself), `acquisition-verbs/tests/`, `acquisition-mcp/tests/pricing_tools.rs` (the tools call the functions the CLI calls).
- **C47 — Panics are for broken internal invariants only; malformed external input is a structured error with a stable kind.** The store, plan and protocol crates deny `unwrap` and `expect` in production code. *Why:* a persisted queue turns a reproducible panic on bad input into a crash loop. *Pinned:* the `unwrap_used` lints in the store, plan, protocol and search crates' `lib.rs`.
- **C48 — The SQLite schema is internal; raw SQL is not a surface, and there is no cached search service.** *Why:* a stale result taken for current truth is the failure a cache reintroduces. *Pinned:* nothing yet.
- **C79 — Surfaces GGG does not sanction (the trade site, the forums, third-party feeds) are governed inputs, never runtime dependencies, and permission attaches to the access method.** The daemon never touches them; nothing at runtime depends on one; `SURFACES.md` registers each; a human may read one, and tooling fetches one only with the permission its row records. *Why:* the relationship is protected by keeping such surfaces deliberate. *Pinned:* nothing yet.
- **C110 — The Rust workspace is MIT OR Apache-2.0, and nothing in it derives from the expression in a C++ file whose copyright header names anyone but the owner.** The C++ app's behaviour and GGG's facts may be read and distilled, cited by claim or `acquisition-qt@<hash>`, never transcribed; no lint compares the trees until the next Rust table is drafted from a C++ one. *Why:* the C++ tree has many authors and the Rust tree must stay relicensable by one. *Pinned:* nothing yet.
- **C120 — Acquisition never posts to the forum or the shop.** `shop render` writes pages to stdout and a human pastes them; outward credentialed traffic (POESESSID) is refused, not deferred. *Why:* the owner's ruling; a user's agent can drive the user's own browser. *Pinned:* nothing yet.
- **C122 — A behaviour is pinned once, at the lowest crate that holds it, and the process tests share one harness.** A frontend's tests pin what its adapter adds: parsing, text rendering, refusal shape, streams and exit code, size bounds, the door. Wording is a golden, regenerated under `ACQ_UPDATE_FIXTURES=1` and naming the facts a reader must find, so a blind regeneration fails. Every test process takes the harness's one isolation list. A crate's integration tests are one binary, except a file that sets the process environment; the gate runs them under nextest, a process per test, never retried. *Why:* one wording was three edits in three crates, and 72 binaries linked the same crates. *Pinned:* `tools/arch-check.sh`, `.config/nextest.toml`, `acquisition-client/tests/harness/mod.rs`.
- **C153 — A finding joins the surface that holds it before it becomes a file, and a close retires as it pins.** A finding becomes a row in an existing table, corpus or transcript, or a case in an existing test, before it becomes a test file of its own. A surface's wording is held by one golden; a sentence literal elsewhere asserts only what no golden holds. A step's close lists what it retired beside what it pinned. *Why:* findings pinned as files of their own grew a ledger of incidents, one behaviour pinned at three layers. *Pinned:* nothing yet.
- **C154 — A test or a check is retired by a commit that names what now holds its property.** The message carries one line per retirement, `retired X; held by Y`, where Y is a test, a golden, a check, a type or a dependency edge that runs in the gate or in CI at that commit. A worker that cannot name Y keeps X and files an issue. *Why:* P6 (retire what is proven) had no procedure and no evidence home, so nothing was retired. *Pinned:* nothing yet.

## Principles

- **Evidence before ceremony.** A ruling lands in the registry before the code; a procedure becomes a skill after it has run twice and repeated a trap; a check or a rule is retired when what it guarded is proven (P1, P6).
- **Real use validates.** Each frontend contract needs its own consumer; generalise after two materially different consumers reveal the shared property, except where an early choice controls identity, durability, safety or compatibility (P2, P3).
- **Structure over discipline.** A rule that can be broken silently becomes a dependency edge, a lint or a test; knowledge sits at the lowest rung that holds it, and a boundary never sits below this file (P4, P5).

## Where things live

- **A ruling**: `decisions/<area>.md`, in the entry rule's form, the owner's word first. Its mechanism is a doc comment on the code, headed by the id; its property is a test that cites the id.
- **A finding**: the test named for the fault, or the commit message. A fault that came back is a row in `REVIEW-CHECKLIST.md`, the review checklist by area, read before reviewing a change there.
- **Work not done yet**: a GitHub issue, labelled `area:*` (its area file) and `trigger:*` (`user`, a user's report; `seat`, a seat run; `consumer`, a second consumer; `gui`, phase 4; `live`, a live run, a sitting or a game event; `ci`, a red CI run). A feature waiting to be asked for is an issue. What the project declines until a named trigger is a "Not doing" line at the foot of the area's file.
- **A fact about GGG**: a numbered claim in `docs/design/network-ground-truth.md` (the trade appendix beside it); `acquisition-qt` takes copies, never the reverse.
- **A live run**: one `RUN-LEDGER.md` row, read by its tail; evidence under gitignored `runs/`. A seat: one `SEAT-LEDGER.md` row.
- **What a user meets** (a verb, a flag, a knob, an install step): clap help or the MCP tool text, regenerated into the references with `ACQ_UPDATE_FIXTURES=1`, and one README tour line or knob row; `readme_tour.rs` runs the tour.
- **History**: `HISTORY.md`, one line per slice; the rest is `git log`. A deleted text is cited as `<path>` at `<commit>`.
- **The road ahead**: `roadmap/README.md`, read first for any step of it.
- **A working noun** (door, world, seat): `GLOSSARY.md`.

## Procedures

Skills under `.claude/skills/`, each saying when it applies: live-run
(anything that talks to the real GGG API), mock-session (every other
`acq`), session-close (before the last commit of a session), seat (a
blind agent at the tool), research-track, site-sitting (asking the
trade site).

## Quality gate

```sh
cargo build --workspace
cargo nextest run --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
git diff --check
tools/docs-check.sh
tools/arch-check.sh
tools/trade-check.sh
python3 -I tools/bothy.py check
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```

`tools/gate.sh` runs these, then the build again, with a table of exit
statuses; `--docs` runs the five that build no Rust.

Once per clone: `git config core.hooksPath .githooks` (the commit-msg
hook writes Claude's trailer as `Assisted-by:`) and `cargo install
cargo-nextest --locked` (the gate's runner).
