# seat-issues-e — a job's times and failure kind, and a name never held

Round E of the answer-facts seat's issues (`audits/answer-facts-2026-11/seat/`,
filed 2026-11-04): the two the owner ruled on 2026-11-04 ("agreed. yes
to both"), after rounds A to D (`seat-issues-a.md` to `seat-issues-d.md`)
closed. Written by the orchestrating session against the tree at
`9cf967f4`. Two rulings recorded before this brief: C165 new
(`decisions/daemon.md`: a job's record says when, a failed job says
what kind of failure it is) and C121 amended (`decisions/frontends.md`:
`unknown_account` for a name the index never held). Kept: C10 (the
handshake is single-version, so an additive field needs no
negotiation), C45 (with no daemon the ledger answers, in the same
keys), C53 and C163 (every time a document carries has its words
beside it, one clock per age), C143 (a refusal and an offer speak the
request's terms per surface), C144 (a fetched body that could not be
recorded is kept in the failure), invariant 3 (never retry through a
Cloudflare block). One commit per item, a test named for each;
reviewed cold after the build.

The seat's calls: `~/seat-worlds/answer-facts-2026-11/mcp-opus-1/calls/`
(MCP run 1) 8, 14, 19, 20, 23, 56, 59, 60; `cli-opus-1/calls/` (CLI run
1) 103, 104, 105. Read them before each rule. Calls 104 and 105 (a 404
on `stash no-such-tab`) are now refused before the send by C164; the
failure's kind still matters for every failure C164 cannot foresee.

## The items, in this order

1. **#168's kind, a name never held is refused as `not_logged_in`**
   (MCP call 56, CLI call 103). Rule, C121 as amended: a selector that
   names an account the account index never held is refused with the
   new kind `unknown_account`, on every surface and by the daemon alike
   (`resolve_account` and `Sessions::matching`,
   `crates/acquisition-daemon/src/daemon.rs` and `daemon/login.rs`; the
   verbs' account picking, `crates/acquisition-verbs/src/fault.rs`);
   `not_logged_in` stays for no account known and for a held account
   with no session; `ambiguous_account` unchanged. A new member of
   `ErrorKind` (`crates/acquisition-protocol/src/protocol.rs`, `ALL`
   and `as_str`), its wire test; each surface's text and offers
   unchanged but the kind (the ready commands of round C stand).
   `describe`, the tools' text and both references name the kind. Pin:
   C121's pins (`seat_faults.rs` `an_account_that_cannot_be_picked_`,
   `tool_refusals.rs` `a_tool_refusal_is_`, `search_tools.rs`
   `the_requests_account_selects_the_store`), each case of the three
   kinds told apart, the daemon's own sessions test beside them. C121's
   `Pinned:` kept current.
2. **#165's times, a job's record carries no time** (MCP calls 8, 14,
   19, 20, 23, 59, 60). Rule, C165: `JobInfo`
   (`crates/acquisition-protocol/src/job.rs`, about line 80), a job's
   result (`Response::Result`, and each frontend's result document) and
   its ledger row (`crates/acquisition-store/src/jobs.rs`, which has
   `submitted_at` and `updated_at` today) carry `submitted_at`,
   `started_at` and `finished_at`, unix seconds on the daemon's clock,
   each absent until it happens (a waiting job has no `started_at`; a
   job re-queued after a 429 keeps its first start, say so in the doc).
   The ledger gains the columns its schema needs (a migration the
   store's schema rules allow; an old row reads with them absent). Each
   frontend's document carries `<time>_age` beside each, from the one
   formatter `PersistedJob::aged` uses (C163); text gives them in words
   on `acq jobs`, `status`, `result` and a waited job's last line, and
   the MCP's `list_jobs`, `job_status`, `job_result`; with no daemon,
   the ledger's answer carries the same keys (C45). Pin: the protocol's
   wire test (the fields, absent and present), the daemon's queue test
   (each time set at its transition, on a fixed clock), a golden on
   each surface, live and from the ledger.
3. **#165's failure kind, a failed fetch offers no way on** (CLI calls
   104, 105; MCP calls 59, 60). Rule, C165: `Outcome::Failure` carries
   `kind`, from a closed list the protocol names (`job.rs`, beside
   `Outcome`), read from what the daemon already knows at each failure
   path, never from the sentence: one kind per distinct way on. Derive
   the list from the daemon's failure paths and name it in the report;
   expect at least a GGG answer that the resource is not there (404), a
   GGG refusal or server fault, a Cloudflare block, a rate-limit
   exhaustion if one ends a job, a fetch that could not be recorded
   (C144, its body kept), a parent whose children failed (C23), and an
   internal fault. Each frontend's failure document carries the `kind`
   and its `offers` in its own terms (C143), the CLI's text one line of
   the way on: a stash's 404 the fetch C164's refusal names (the
   league's `stashes` job, or a substash's parent); a not-recorded body
   where it is kept; a parent its failed children's listing; a
   Cloudflare block no retry at all, in words that say not to (invariant
   3), never an offer that sends. A failure from before the field reads
   with no kind and no offers. `describe` and the tools' text name the
   kinds. Pin: the daemon's tests, each kind at its path; the CLI's
   golden of a failed fetch with its offer run as printed; the MCP's
   `job_result` of the same; a Cloudflare failure offering nothing that
   sends.

## Done when

Each item has its commit, its message naming this brief, its ruling and
its issue (never `Closes #N`: the orchestrator closes them), and its
pin; C121's and C165's `Pinned:` name the tests; every regenerated line
is one of the items' words or a `# holds` line; both references
regenerated where help or a tool's text changed; `tools/gate.sh` green,
run bare at the end and before each commit, and `tools/gate.sh --only
doc` green where a doc comment changed.

## Not in scope

#7 beyond what item 3's kinds give (a GGG error's verb-level hint);
any other wording the reads turn up (report it); a job's history beyond
its three times; a retry policy.

## Rules

`TEMPLATE.md`; the main tree, no other worker building; scratch
`/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/seat-issues-e/`;
`ACQ_UPDATE_FIXTURES=1` once after the holders are moved; no JSON key
renamed or removed; no push; no issue opened, closed or commented.
Each fix's test is seen failing before the fix, said in the report.

## Report

`TEMPLATE.md`'s six sections, then one paragraph per item: the rule as
built, the pin, the failing-before evidence, one line of what a reader
now meets; for item 3, the kinds and the path each is read at.
