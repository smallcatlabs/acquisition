# r5 — The five surface sentences

Readiness step 5 (`roadmap/README.md`, row r). Closes #78. Half a day
for the first three; the selection (items 4 and 5) is a design in the
delegated class, recorded in its commit under the owner's delegation.

## Goal

Every sentence the MCP and the CLI print about their own surface is
one the build honours: the job-kind list, the `describe` sections, the
logout receipt's words, and the selection flags C76 names. The
generated references are goldens of the binary's words, so a wrong
sentence is pinned, not caught; this step adds the one test that could
catch the first.

## Read

- #78 (the five, each with its file); `roadmap/STATE-2026-10-26.md`
  section 2F.
- `decisions/plans.md` C76 (one door: `--tabs`, `--all`,
  `--characters`, a page's ticks; the `refresh` kind is gone), C44 (a
  selection plan carries no policy revision), C111 (each request spent
  once); `decisions/frontends.md` C116, C122 (a behaviour pinned once,
  at the lowest crate), C143 (a shared refusal speaks the request's
  words), C146; `decisions/store.md` C55 (a character's id is its
  identity, its name its address).
- `crates/acquisition-mcp/src/main.rs`: `SubmitParams` (around line
  535, the kind list), `DescribeParams` (496, the sections list), the
  `refresh_plan` tool and its params.
- `crates/acquisition-cli/src/main.rs`: `acq submit`'s help (where its
  kind list comes from), the `Logout` help (around 780) and the logout
  arm's `keyring` values (around 1074).
- `crates/acquisition-daemon/src/daemon/queue.rs`: the `KINDS` table
  and the refusal that lists the client kinds (the words the MCP's
  list must equal).
- `crates/acquisition-search/src/describe.rs`: the sections as the
  answer names them (the keys the drive of 2026-10-26 saw:
  composition, computed, counts, explain, fields, limits, line,
  linked, not_built, operators, slots, totals, trade).
- `crates/acquisition-cli/src/plan_cmd.rs`: the selection (`--tabs`,
  `--all`, `--deep`; `characters: Vec::new()` around line 375);
  `crates/acquisition-verbs/src/refresh.rs`: `Opened::plan_selection`
  (what a selection carries); `crates/acquisition-cli/src/web/mod.rs`:
  the refresh page's ticks (how the page selects characters today).
- Tests: `crates/acquisition-mcp/tests/journey.rs` and
  `tool_refusals.rs`; `crates/acquisition-cli/tests/plan_json.rs`,
  `apply_loop.rs`, `web.rs` (the refresh loop), `seat_faults.rs`;
  `crates/acquisition-plan/src/lib.rs`, the `c44_` tests (the
  selection plan).
- The reference regeneration: `ACQ_UPDATE_FIXTURES=1 cargo test -p
  acquisition-mcp --test main mcp_reference` and the CLI's twin; read
  each diff as a user would (review shape 9).

## Input

- The daemon's own refusal for an unknown kind, as the drive recorded
  it: "unknown job kind "refresh"; a client may submit: profile,
  characters, character, leagues, stashes, stash, apply, sleep, fetch,
  whoami".
- The `describe` answer's keys, from a run of `acq search --describe
  --json`.

## Do

1. **The kind list, one source.** The words the daemon's refusal lists
   are the words `acq submit --help` and the MCP's `submit_job`
   describe; hold them in one place both frontends link (the protocol
   crate names the kinds a client may submit, or the verbs crate
   carries the sentence), and pin it: an MCP test submits an unknown
   kind to a mock daemon and asserts the refusal's list equals the
   description's list, so the two cannot drift again. `apply` joins the
   list with one line on what it takes.
2. **`describe.names`**: the sections list is the answer's own key
   list, generated or asserted equal in a test; `trade` and `totals`
   appear.
3. **The logout receipt**: the help names `failed` beside the other
   three, with what it means.
4. **`acq refresh --characters <ID,…>`**: character ids (C55), carried
   into `plan_selection` as the page carries them; `--all` says in its
   help whether it covers characters, and does what it says. Pinned at
   the verbs crate (a selection naming characters compiles a plan
   whose actions name them), with the CLI's adapter part in
   `plan_json.rs`.
5. **The MCP's selection**: `refresh_plan` gains `selection` ({`tabs`:
   [ids], `all`: bool, `characters`: [ids], `deep`: bool}), exclusive
   with the policy compile, answering the same envelope `apply_plan`
   takes (C76, one door; C44, no revision on a selection plan). The
   tool's description says both forms in two sentences. Recorded under
   delegation in the commit; `MCP-REFERENCE.md` regenerated.
6. One commit per sentence; the references regenerated in the commit
   that changes their source, every changed line explained.

## Done when

- The MCP's kind description equals the daemon's refusal list, held by
  a test that drives a mock daemon.
- `describe` with `["trade"]` and `["totals"]` is documented and
  answers; the sections list equals the answer's keys, pinned.
- `acq auth logout --help` names four keyring values.
- `acq refresh --characters` compiles and applies a selection plan in a
  mock world (the apply loop test extended by one case).
- The MCP's `refresh_plan` with `selection` returns a plan `apply_plan`
  accepts; pinned in `plan_loop.rs`.
- Both references regenerated; the commit messages explain every
  changed line.
- The gate green on the step's own tree, every stage's exit status
  shown.

## Not in scope

- #10 (one refusal and receipt shape) and #83 (the trimmed surface):
  the sentences change, their length and register do not, here.
- A `set_priority` tool and an events summary on the MCP (noticed by
  the same read; an issue if a consumer asks).
- Character names as selection addresses (C55: the id is the
  identity; the name is a request's address, not a flag's).

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree; scratch under your session's scratchpad in a
subdirectory named `r5`, every `ACQ_STORE_DIR` under it; commit on
`main`, never push; the `Co-Authored-By` line for your model last;
long commands in the background; `tools/gate.sh` if p0 has landed,
else the gate bare by each stage's own exit status, and `cargo build
--workspace` before any daemon and after any test run (C84).

## Report

`TEMPLATE.md`, as text.
