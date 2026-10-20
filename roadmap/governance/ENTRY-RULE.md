# The entry rule, and ten entries rewritten under it

The rule goes into `tools/bothy.py`'s docstring (step 1c) and
`tools/docs-check.sh` refuses its history markers (step 1b). The
samples below are the rule applied; the migration applies it to all
119 entries and the one new one (C120).

## The rule

An entry is one bullet:

`- **C<n> — title.** ruling. *Why:* one sentence. *Pinned:* the test or check that holds it.`

1. **Plain declaratives.** A subject, a verb, one idea per sentence.
   The five-word maxims stay where they exist ("Reads observe,
   assertions plan, apply spends").
2. **One *Why*,** the reason and nothing else.
3. ***Pinned* names the file** and the test name or prefix that holds
   the property; a ruling nothing holds says `*Pinned:* nothing yet`.
   There is no *Details:* (`bothy show C<n>` lists where the code
   names the id) and no *Evidence:* (a claim is cited in the ruling or
   the *Why* with its gloss; a ledger row or an audit is history).
4. **A gloss on every cited id,** two to five words, the first time
   the id appears in the entry: `C53 (three output levels)`, `N33
   (the token endpoint's policy)`, `S131 (one reading of place)`. The
   gloss helps the reader and binds nothing.
5. **No history.** No date; no "Decided", "Ruled", "Amended", "Kept";
   no commit hash; no owner quote; no seat call number; no
   measurement; no note citation. `git log -S'C<n>'` and the commit
   messages hold all of it. A ruling whose build waits says `Unbuilt`
   and names its issue.
6. **The limit stays at 800 bytes.** With history gone the ten samples land
   between 386 and 727 bytes (the table at the foot); the limit is a
   prompt to route a mechanism to the code, never a budget to fill. `docs-check.sh` refuses an entry
   that carries a history marker (a date, a seven-hex commit, "owner:",
   "verbatim", "Amended", "Decided", "Ruled", "call N").
7. **The owner's words stay the authority.** An entry changes only by
   the owner's ruling, and the commit that changes it quotes him.

What each sample lost is said under it. Sizes are measured at the foot.

## The ten

### C1 (cross-cutting; was: edge list with two amendment dates and "C12's two surfaces as edges the check refuses")

- **C1 — The workspace is library-centric and the daemon is its own artifact.** `acqd` is the only GGG sender and links the protocol crate and the store, nothing else; the store links none of the workspace crates; the protocol crate links serde alone; a frontend links client, protocol, store, planner and search as it needs them, never the daemon; neither the daemon nor the planner links the search. *Why:* shared logic is written and tested once, and the edges are refused by a check, not held by discipline. *Pinned:* `tools/docs-check.sh` (the edges; `tools/arch-check.sh` once step 1e moves them).

Lost: "Amended 2026-09-09, 2026-09-17"; the parenthetical history of C89.

### C3 (daemon; was: an amendment note and the release seat's "Kept 2026-10-19 … (owner: 'yes, go ahead')")

- **C3 — The daemon spawns lazily and exits when idle.** A use verb spawns a daemon when none runs; an observation verb never spawns or replaces (C10, single-version handshake); a frontend spawns only where its own policy allows (C13, the MCP never in real mode). The daemon exits after a short period with no connections and no queued work. A job verb with no login still spawns, and its refusal says that a daemon started and when it will exit. `daemon stop` and `daemon status` exist for debugging. *Why:* no stray processes and no start-up ceremony. *Pinned:* `daemon.rs`, `idle_verdict_*`.

Lost: the gpg-agent simile, "Amended 2026-09-09", "Kept 2026-10-19 after the release seat", the owner quote; the release seat's outcome is now the ruling's own sentence.

### C36 (plans; was: two dates, a *Pinned* list, and the 0b amendment with the owner's "My answer is: yes.")

- **C36 — The sync policy is the first annotation: a per-account declaration of desired coverage and freshness, compiled by the frontend's planner into minimal requests.** It is not a scheduler. A `metadata.items` count can prove a tab changed, never that it did not. A `policy set` replaces only the realms it names, and a realm named with no leagues is dropped; `--replace` (MCP `replace`) replaces the whole; the MCP names the revision it replaces (C52, no blind replace). *Why:* the C++ tracked-set and clean-refresh semantics describe this one object. *Pinned:* `acquisition-plan/src/lib.rs` (the count heuristic, the strict parse, `a_first_merge_drops_a_realm_named_with_no_leagues`), `plan_json.rs`, `plan_loop.rs`.

Lost: "Decided 2026-08-31. Amended 2026-10-19 (owner: …)"; the sentence naming the C++ topics and essays.

### C53 (frontends; was: "Ruled 2026-09-02; amended 2026-09-03 after the live density reading", *Details*, *Evidence: ledger row*)

- **C53 — Output has three levels: default text is the decision view, `--expand` the audit view, JSON the contract.** Text is a function of JSON; grouping is presentation, never authorization. Default text states repeated context once, leaves out prose and provenance that do not change the next decision, opens with one answer and ends with the next action. Ten or fewer entities are listed, more are counted. A failure names target, cause and evidence; every nothing says which nothing. Ages are text, epochs are JSON. Escapes only on a tty. JSON changes are additive. Stash tab ids sit in the last column. *Why:* a reader decides from the first screen and audits only on request. *Pinned:* the `c53_` tests.

Lost: the two dates, the *Details* pointer (`plan_cmd.rs` names C53 itself), the ledger row; the review's catch, the live reading's measurement in the *Why*.

### C80 (pricing; was: "Confirmed on the site 2026-09-09, all three ways (PRICING-SLICE.md …)", owner quote, the owner's counts)

- **C80 — The game side's tab for a nested item is the tab a user can name and publish.** A substash reads its parent's name and `public` and reports its own name beside them; a folder child reads itself; a folder's name is never a price. *Why:* a substash has no settable name or `public` of its own, and a folder is a grouping, not a tab. *Pinned:* the `c80_` tests in `listing.rs`.

Lost: the site confirmation (the commit that closed the provisional clause holds it), "64 substashes … 77 folder children", the quote, "Ruled 2026-09-06".

### C87 (network; was: the owner quote "we need to fix this rather than just record it", "which the rails-off default did until this ruling", a date)

- **C87 — An access token the provider rejects with a 401 is dead.** No send leaves the gate carrying it once the rejection has landed, and the account's next send is the refresh. The choke point reads the token after admission and reports a 401 before the rejecting send's permit drops; a waiter whose token is gone refreshes outside the permit and retries once; sends already on the wire may land 401 too; the job that met the 401 fails and says so. *Why:* a rejected bearer re-sent per job is a timed stream of 401s spending the invalid-request budget (N34, the dead bearer re-sent; N27, the 4xx budget). *Pinned:* `daemon.rs`, `rejected_access_token_*`.

Lost: the quote and the date; "Details: ratelimit.rs, Bearer" (the module names C87).

### C110 (cross-cutting; was: the audit's numbers, the 51a33751 story and "owner: 'accepted'", an *Evidence* directory)

- **C110 — The Rust workspace is MIT OR Apache-2.0, and nothing in it derives from the expression in a C++ file whose copyright header names anyone but the owner.** The C++ app's behaviour and GGG's facts may be read and distilled, cited by claim or `acquisition-qt@<hash>`, never transcribed; no lint compares the trees until the next Rust table is drafted from a C++ one. *Why:* the C++ tree has many authors and the Rust tree must stay relicensable by one. *Pinned:* nothing yet.

Lost: "Audited 2026-10-11 at 6ce6500f: 0 shared comment runs, 20 shared literals …" (the audit directory `audits/relicense-2026-10/` stays in the tree and the commit message cites it); the cross-cutting park on the transcription lint, now the entry's last clause.

### C111 (plans; was: "the seat's replay spent 7 requests", "Decided 2026-10-13 (the session's, under the owner's delegation, on a design study)")

- **C111 — A plan spends each request once.** An action is spent when its target was fetched at or after the plan's snapshot, or when the daemon already holds the same request for the account. Apply sends the unspent actions and names the spent ones; it never refuses, so a replayed plan sends nothing. The check is frontend-side before submit, as C44's (the revision check); two applies racing both send. A fetch of what is fresh is asked for by name, never by replay. *Why:* binding authorizes a fetch, not a fetch per application. *Pinned:* `acquisition-cli/tests/seat_faults.rs`, `acquisition-mcp/tests/plan_loop.rs`.

Lost: the seat measurement and the delegation note; "Details: acquisition-plan, unspent".

### C115 (frontends; the report's own example of the style; was: "measured at the re-sit: no call spent, one re-read a run (parked)", "Owner, 2026-10-18 … 'I accept all 6. Go ahead.'", *Details: note 38*, *Evidence: audits/*)

- **C115 — An answer leads with what bears on the question: the total and the view come before the terms block.** The terms block stays, without per-term request counts, and C93's meaning (absent is false, undecided is counted apart) stays. JSON carries a count's request only when routes are asked for. `tabs` in JSON takes the CLI's counts-first shape and a limit. *Why:* the MCP's default answer was larger than every seat harness's cut, and runs re-read the terms block instead of the answer. *Pinned:* `acquisition-search/tests/answer.rs`, `acquisition-cli/tests/search_json.rs`, `acquisition-mcp/tests/search_tools.rs`.

Lost: the measurement, the quote, the note citation, the audit path.

### C117 (search; was: "both second-round Opus runs made 'would a buyer see it' their worst fault … at four calls a task", the owner quote, *Details: note 38*)

- **C117 — GGG's facts about a tab are fields, and the tool says nothing of what the site lists.** `tab.public` (the listing's `metadata.public`; a substash takes its tab's; an item on a character lacks it), `tab.remove_only` (the name suffix GGG adds, N47) and `price.side` are searchable. Whether the site would list an item is the agent's inference, labelled as such. *Why:* seat runs inferred "would a buyer see it" from `has:priced` and `price show` while the store held the flag. *Pinned:* `acquisition-search/tests/seat_faults.rs`, `acquisition-cli/tests/seat_faults.rs`.

Lost: the call counts, the quote, the note citation; the park on the site's half is now an issue.

## Sizes

Bytes per entry, measured by a script over this file and the registry at `c5535d52`:

| Entry | Before | After |
| --- | --- | --- |
| C1 | 669 | 546 |
| C3 | 701 | 596 |
| C36 | 798 | 724 |
| C53 | 789 | 727 |
| C80 | 786 | 386 |
| C87 | 798 | 659 |
| C110 | 797 | 484 |
| C111 | 797 | 623 |
| C115 | 791 | 627 |
| C117 | 739 | 580 |
| total | 7665 | 5952 |
