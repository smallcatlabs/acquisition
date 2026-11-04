# seat-issues-d — a stash job for a tab the stored listing does not name

Round D of the answer-facts seat's issues (`audits/answer-facts-2026-11/seat/`,
filed 2026-11-04), at the owner's word of 2026-11-04 ("Yes, let's
continue"), after rounds A to C (`seat-issues-a.md` to `seat-issues-c.md`)
closed. Written by the orchestrating session against the tree at
`e9e0f9ec`. One new ruling, C164 (`decisions/daemon.md`, beside C127),
recorded before this brief; C127 (every param that shapes a send judged
at submit, no client value quoted whole), C14 (the daemon is the single
gate) and C143 (a refusal speaks in the request's terms) kept. One
commit, a test named for the rule; reviewed cold after the build.

The seat's calls are in `~/seat-worlds/answer-facts-2026-11/mcp-opus-1/calls/`
(MCP run 1): call 21 (`tabs` for Rehearsal, eight tabs, none
`rh-nope`), 57 (`submit_job` `stash` `rh-nope`, admitted as job 17),
59 and 60 (job 17 failed: "GET /stash/Rehearsal/rh-nope returned 404
Not Found"). Read them before the rule.

## The item

**#166, a stash job for a tab the stored listing does not name is
sent.** The client door (`admit_client`,
`crates/acquisition-daemon/src/daemon/queue.rs`, about line 1204;
`Request::Submit` in `daemon/server.rs`, about line 260) judges a
stash job's params by their shape alone, and the daemon spends a
request on the 404 the store's listing predicted. `refresh_plan`
already reports `unknown_tabs` for a plan. Rule, C164:

- After the account resolves and before an id exists, a client's
  `stash` job is judged against the account's stored listing of its
  realm and league (the store the daemon already opens for that
  account, `daemon/kinds.rs` about line 248; read through the store's
  read API, never raw SQL in the daemon). A tab the listing does not
  name, or names as gone (the one live-tab test, `TAB_GONE`), is
  refused, kind `refused`. A `sub` is judged only where the stored
  rows name that tab's substashes; otherwise it is admitted.
- No listing of that realm and league stored: admitted, as now.
- The refusal's words name the listing (realm and league), its age
  (the store's clock for the listing, as `tabs` prints it, C53 one
  clock), that it names no such tab, and that a `stashes` job fetches
  the listing again; the tab's id quoted bounded as C127's refusals
  quote (`describe_str`). Each frontend's text and JSON carry that
  refusal as they carry any submit refusal; where the CLI's or the
  MCP's refusal document offers a way on for a refused submit, the
  listing's fetch is offered in each surface's terms (C143): `acq
  stashes --league … --realm …` and `submit_job` kind `stashes`.
- The daemon's own submissions (an apply's children, a deep fetch's
  substashes) never pass the client door and are unchanged.
- `describe` and the MCP's `submit_job` text say a stash job for a
  tab the stored listing does not name is refused (mind the
  description bound; measure, never guess).

Pin: the daemon's queue tests beside the `c127_` ones: a world whose
stored listing names tabs A and B, a stash job for C refused and
nothing sent (the send journal empty for it), for A admitted, a gone
tab refused, a league with no stored listing admitted; the refusal's
wording in its golden on the CLI (`acq stash C` run in a mock world)
and the MCP's refusal test, each offer run as printed. Seen failing
first.

## Done when

The item has its commit, its message naming this brief, C164 and #166
(never `Closes #N`: the orchestrator closes it), and its pin; every
regenerated line is the item's words or a `# holds` line; both
references regenerated where help or a tool's text changed; C164's
`Pinned:` names the test; `tools/gate.sh` green, run bare at the end,
and `tools/gate.sh --only doc` green where a doc comment changed.

## Not in scope

#165 (a job's times, the protocol); #168's kind (C121); a fresh listing
fetched for the client; a new error kind; any other wording the reads
turn up (report it).

## Rules

`TEMPLATE.md`; the main tree, no other worker building; scratch
`/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/seat-issues-d/`;
`ACQ_UPDATE_FIXTURES=1` once after the holders are moved; no JSON key
renamed or removed; no push; no issue opened, closed or commented.
The test is seen failing before the fix, said in the report.

## Report

`TEMPLATE.md`'s six sections, then one paragraph: the rule as built,
the pin, the failing-before evidence, one line of what a reader now
meets.
