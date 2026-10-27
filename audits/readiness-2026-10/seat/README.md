# The readiness seat — the record

Pull-forward step 3, #84: the seat over the MCP surface #83 trimmed,
before `alpha.2`, and the first row of `SEAT-LEDGER.md`. Run under
`BRIEF.md`, `PROMPT.md` and `PROMPT-debrief.md`, committed at
`799c39d9`. One run, Opus, on the MCP door, 2026-10-27. The fates
below were decided by the orchestrating session under the owner's
standing word for the day; the fix round is `64edb5d7`, `45ea27f7` and
the commit that writes this table. The close removes `BRIEF.md` (the
skill's step 10) and writes the ledger's row for the fates.

## The world

Made 2026-10-27 under `~/seat-worlds/readiness-2026-10/`, outside the
repository. `pristine/` is `setup --store` over the gitignored copy of
the owner's store at `search/item-facts/raw/m3/store` (one account,
`GERWARIC#7694`, provider `mock`, facts of 2026-09-02 to 09-11, 22,721
live items), so the owner's store was not read. `mcp-opus/` is `setup
--store pristine/store` with no daemon (`ACQ_NO_SPAWN=1`), its brief as
`BRIEF.md`, `README.md` as `799c39d9` carries it, the door
`seat-mcp.py` and `bin/`. **Both stores are kept** (the issue: a rerun
reads the same facts); a rerun is `setup --store pristine/store` into a
fresh world.

| Build | Tree | `acq-mcp` sha256 | `acq` sha256 |
| --- | --- | --- | --- |
| `cargo build --release -p acquisition-cli -p acquisition-mcp`, aarch64-apple-darwin | `799c39d9` (the crates as at `c2095319`) | `eaf0a961eeda4900403c2728b654eccfb65ac1814755325ef32bdb06fd52b8e0` | `cd04c585646dcc5b71c807c3999f4bc8ec820e288981615cd5bd34c345c16f3a` |

The runner's startup context is the owner's global `CLAUDE.md` alone,
sha256 `e9c1329f…`, as at the release seat.

## The run

| Run | Model | Session | Calls | Cut | Turns | Cost | Files |
| --- | --- | --- | --- | --- | --- | --- | --- |
| mcp-opus | `claude-opus-5-5` | `0f995149` | 51 | 0 | 72 + 2 | $3.64 + $0.20 | `calls.csv`, `REPORT.md`, `DEBRIEF.md` |

All eight tasks finished; T4's "is anything wrong" with a gap (three
invalid notes counted and not named). One door command denied by the
rails (a `call` heredoc, sent again as `drafts/t3b.json`); the harness
saved calls 1, 3 and 35 to files of its own, and the runner read 1 and
35 whole there; it ran `date -u -r` outside the door to turn epochs
into dates. Every price it wrote was cleared and read back (call 51
against call 20). Its debrief retracts two T2 counts made by eye from
call 11 (nine Solo Self-Found characters, not ten; ten empty bodies,
not fourteen) and T3(a)'s "most on characters", and marks six claims
as its own knowledge of the game.

## What this seat measured

1. **The first turn.** `open` printed 31,472 bytes of text (calls 1
   and 3), against 42,082 at the release seat's MCP runs.
2. **The trim's door.** The runner reached `describe` by name from the
   tools' own words, five times (calls 5, 12, 13, 19, 41), never whole;
   the brief named no section.
3. **Calls per task**, T0 to T7: 2, 2, 7, 7, 16, 5, 6, 6 (51), against
   the release seat's 3, 1, 6, 8, 12, 6, 6, 9 (mcp-opus, 51), 2, 1, 8,
   8, 13, 7, 8, 8 (mcp-sonnet, 55) and 3, 2, 6, 9, 16, 6, 7, 7
   (mcp-opus-2, 56). **Calls to the first answer the runner held
   right**, counted within the task from the call its report names:
   T0 none named; T1 1 (call 3); T2 1 for the totals (call 4), 5 for
   the split by league (call 8); T3 3, 5, 6, 7 for (a) to (d) (calls
   14, 16, 17, 18; (b)'s first query, call 15, was wrong and the
   runner caught it from `terms[].resolved`); T4 2 for the state (call
   20), 10 for the writes (call 28); T5 1 (call 35); T6 1 (call 40);
   T7 1 (call 46). The release seat named no first-right call, so this
   column starts here.
4. **The shop render**: 34,779 bytes with no arguments (call 35),
   against 160,473 at mcp-opus-2; still the largest answer of the run
   for a 2,800-character page.

The counts in the report's task sections are the runner's, not
recomputed here.

## The faults, by the rule each expresses

Read against the calls they cite: calls 1, 5, 6, 7, 12, 13, 19, 33,
35, 45, 46, 47, 49 (thirteen), and calls 6, 46 and 47 reproduced in a
scratch world made from the same pristine, on the same build.

| Rule | Faults (calls) | Whose | Fate |
| --- | --- | --- | --- |
| An MCP answer speaks the MCP's words: tools and argument paths, never the CLI's verbs and flags | `realm_needed`'s `readings` `--realm pc` (6); `not_found` names `acq search`, `acq tabs` (47); the shop's `unresolved` rule names `acq price show` (35); `describe` "store" names `acq store refused` (5), "counts" documents `--count`, `--cross`, `--sum` (13), "values" `--sort` (12) | the tool's; the runner's worst and its first change | fixed by one rule, `64edb5d7`: every text the MCP serves passes through the spelling (C143's form, one table of commands and flags), pinned by the sweep in `crates/acquisition-mcp/tests/spoken.rs` |
| What a tool counts, a tool lists | `refused_bodies: 4` with no MCP listing (5); `invalid_notes` counted, not named (20, 21) | the tool's | not fixed here: #18 and #11, this seat's calls added as comments |
| Every key an answer carries is explained where `describe` says | `freshness.position_before_listing` (35); `persisted` on `accounts` (2); `basis.intent` moving 0 to 3 (18, 34) | the tool's | not fixed here: #11, this seat's calls added as a comment |
| The pricing model states who decides | an item's game note beat the hand row on its tab (33, its `why` says so per item); `describe` "pricing" names `effective` and not the rule (19) | the tool's, a gap in words (C69 and C81, the most specific statement, hold the rule) | fixed, `45ea27f7`: one sentence in the `effective` entry, pinned in `pricing_tools.rs` |
| The instructions' sentences are true of every tool they name | "The job tools talk to the daemon (… list_jobs, job_status, job_result, … daemon_status)" (1), four of which answered without one (40, 42, 45) | the tool's | fixed by the commit that writes this table: the daemon line names `submit_job`, `apply_plan`, `cancel_job`, and a line says the four observers answer without one; pinned in `surface_size.rs`'s group-sentence test |
| A kind names the mistake | a typo'd account refused as `not_logged_in` (46) | C121's consequence (one word for an account that cannot be picked, on purpose) | kept as designed (C121); raised for the owner at the close |
| The read economy | `shop_render` 34,779 bytes, mostly `left_out` and `freshness` (35); a tab's `price_show` 28,857 bytes (27); `characters` 65 rows and no counts (11), which the debrief names as the cause of its two retracted counts | the tool's | not fixed here: #9 (2g), this seat's calls added as a comment |
| Ages are words where a person reads them | epoch seconds in every answer (2, 8, 18) | the tool's (C53: epochs are JSON) | not fixed here: #10 (2g), this seat's calls added as a comment |
| Not faults | "Begin with search and no arguments" fails (1, 6): the same sentence goes on "where the store holds several realms, name scope.realm (the refusal offers each)"; `realm` as a count key (7): realm is the scope, every bucket carries it (8), and the refusal listed the keys; `job_status` on an unknown id advising a daemon (45): it says both, "no job 1 on disk, and the daemon is not running" | the runner's reading | not taken |

## What the README and the tools did not say (input, not faults)

Where Claude Code's `.mcp.json` or Claude Desktop's configuration
lives; whether the host's `env` needs `ACQ_STORE_DIR` beside
`ACQ_PROVIDER`; whether `acq auth check` restores a login the index
shows `persisted: false`. The runner never needed the install, the
knobs, the rails, the rate-limit clock or the tour beyond `auth
check`: "Your agent" was the one section it used. The door sets the
provider and the store on every call, so the README's bare
configuration was not measured against this store (the report, T1).

## The debriefs, routed

To be routed at the close. The debrief's answer 5 (counts by eye from
a bare array) and 10 ("which numbers did you derive yourself?" as a
question to add) are the two the runner most wants read.
