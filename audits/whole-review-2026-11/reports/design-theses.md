# Design theses for the full-stop review

Read-only; nothing built or run. **V** = verified by me in code or by a command; **I** = inferred. Paths are under ``.

## 1. Theses, ranked by expected value

**T1. The words a person reads are shared semantics held in a frontend; move them below the frontends.**
- Evidence (V): the CLI's command modules hold 114 `*_words/_text/_sentence` functions against 12 in search and 0 in plan/store (`grep -rhoE 'fn [a-z_]+_(words|text|sentence)\b'`). `web/` reaches 93 distinct `*_cmd::` symbols, which is why C139 puts the GUI inside the CLI crate. The MCP gets none and patches CLI spellings (`crates/acquisition-mcp/src/told.rs`; semantic audit 4b.18).
- Removes: C139's forced placement, `spoken()`, two-makers drift (plan ages, `version`, `auth status`), most of shapes 3 and 7 across doors. A second GUI (C191) becomes possible without linking `acq`.
- Test: list every symbol `web/` and `acq-mcp` take from `*_cmd`; classify each as sentence, layout or logic.
- Judge: designer, then owner (C46 boundary). Confidence: high.

**T2. The queue need not persist: a plan is already re-appliable and the store already records what landed.**
- Evidence (V): `plan.unspent(&facts, &jobs, …)` at `crates/acquisition-verbs/src/refresh.rs:712` re-derives the remainder. C6's own why is "the one thing a restart lost". C47's why (crash loop) and C134's fault marker exist only because rows replay.
- Removes: `store/src/jobs.rs` (791 lines), restore and eviction, C45's offline ledger reads on eight verbs and tools, `Interrupted`, `QueueFailed`, tainted trees. `queue.rs` drew 10 findings, all in restore.
- Loses: resume across a daemon replacement (mostly a developer's rebuild loop) and `acq result <id>` after restart.
- Test: write the smallest breaking inputs; spike-delete in a worktree and read the shortstat.
- Judge: owner (C6, C27, C45). Confidence: medium.

**T3. There are two query languages of near-equal weight; choose one as the model.**
- Evidence (V): the GUI form is generated from the trade site's filter dump and folds into the site's query JSON (`crates/acquisition-cli/src/web/form.rs:1-14`). CLI and MCP speak the own text language. `trade/` is about 5.6k lines. Outbound translation is under "Not doing" in `decisions/trade.md`, and `Paged` omits "a search the form cannot ask" (`web/mod.rs:1434`).
- Consequence (I): an agent's query cannot be edited in the owner's form, so morning 1's handoff is one-way.
- Removes: either the translation's ambiguity machinery or the form's separate model; reorders roadmap item 4.
- Test: run the language corpus through a paper inverse translation and count what the form could hold.
- Judge: owner. Confidence: medium-high that the question is real; low on which side wins.

**T4. An offer is a value; every door should take it back unchanged.**
- Evidence (V): `Way` has 21 variants (`crates/acquisition-verbs/src/refusal.rs:57`), each spelled three times. `answer.rs:3938-4222` is four shells' quoting. The MCP has 30 tools and a 35 KB reference, and the `account` paragraph repeats 19 times. `apply_plan` requires the agent to echo the whole envelope back.
- Shape: one `take` verb or tool accepting any offer or continuation; plans held by handle.
- Removes: the shape-6 class for agents, most per-door "every printed command runs" sweeps, and roughly half the tool list.
- Test: replay one seat transcript by hand under `take`; count calls and bytes.
- Judge: designer (C170 delegates this), with a seat. Confidence: medium.

**T5. Facts never migrate: on a schema mismatch they are re-derived from the verbatim bodies.**
- Evidence (V): the store doc says a wrong extraction "is repaired by re-extracting, never by refetching"; `CONTRACT_REVISION` already digests the store sources; C10 is single-version for the daemon.
- Removes: the migration ladder as a concept (assessment candidate 1 only trims its old steps), `user_version` branching, and the facts half of "a row this build would not have written" (24 findings, 7 of the 20 P1s). Intent keeps strict versions (C35).
- Test: confirm `Store::rebuild` reproduces every derived column from `responses` and `items.json`.
- Judge: line-by-line reader, then owner. Confidence: medium.

**T6. An account is its uuid, everywhere.**
- Evidence (V): facts files are named by username, intent by uuid, the rails dead-grant mark by username (C50's text; semantic audit 4c and 4b.9). "Which account" drew 20 findings.
- Removes: rename orphans, the `filename_safe` collision, and the index as a mirror (it becomes a directory listing plus display names).
- Test: enumerate every `username` used as a key.
- Judge: reader. Confidence: medium-high.

**T7. The read side of the protocol is one snapshot plus events.**
- Evidence (V): `Status`, `List`, `Dashboard`, `DaemonStatus` and `AuthStatus` overlap field for field (`crates/acquisition-protocol/src/protocol.rs:629-780`). `Dashboard` is unpaged (4b.7). Jobs are `kind: String, params: Value` (#100 already open).
- Removes: about five variants and the two-read races (4a.12).
- Test: a field-by-field table of the five responses.
- Judge: designer; owner for the wire. Confidence: medium.

**T8. The frontends' suites re-pin spelling; T1 and T4 must land before t6 can pay.**
- Evidence (V): `crates/acquisition-cli/tests/web.rs` is 8,339 lines for 52 tests, with 280 `contains(` and 95 commits. Frontend tests run to about 39k lines. Over two weeks the tree gained 15k test lines for 14k source lines, with no test file deleted (assessment).
- Shape: one request corpus run through three doors, compared as JSON, plus one rendering golden per block.
- Test: sample 20 `web.rs` tests; count those that assert a document fact a lower crate already pins.
- Judge: reader. Confidence: medium-high.

**T9. The registry is law and design notes under one id space; only law should be numbered.**
- Evidence (V): C1 to C200 are all live and none has been withdrawn (`bothy.py list`). Median entry is 699 bytes. C170 already says agent-facing design is recorded in its commit, yet C63, C152 and C156 are that.
- Evidence (V): since 2026-11-09, 422 of 635 commits touch no file under `crates/`, and 163 are review saves.
- Removes: perhaps 80 to 100 entries, demoted to doc comments at their code.
- Test: tag each entry as owner-class or session-class by C170's own words.
- Judge: owner. Confidence: medium.

**T10. Finding heat follows file size, and "As built" module docs are a second implementation.**
- Evidence (V): 13 source files are 2,000 lines or more. The findings-by-file leaders are `web/mod.rs` (4,550 lines, 480 of them module doc), `queue.rs`, `page.rs` (5,630) and `answer.rs` (4,222). Comments are 27,071 of 119,553 source lines. 71 of 155 findings were words wider than the code.
- Shape: a module doc states ownership and pointers only; behaviour lives in test names.
- Test: cut one "As built" block to ten lines and give a cold agent the same edit task in both versions.
- Judge: a seat. Confidence: medium.

**T11. `acq web` is ruled a development surface (C191) but is the largest frontend and holds semantics.**
- Evidence (V): about 12k source lines; `form.rs`, `fill.rs` and `from.rs` (2.1k lines) hold query construction.
- Shape: before parts 2 and 3, rule what the product GUI consumes. The JSON each route already serves is the third surface, which C138 denies.
- Judge: owner. Confidence: high that it needs ruling.

## 2. Patterns in hindsight

1. **A finding becomes a pin and is never unpinned.** Tests, rulings and checks only grow, and retirement moves pins between layers. This is a strength for the five invariants and a cost elsewhere. Ask: which pins guard something a user or GGG would notice?
2. **A convenience becomes a state family** (main account, C190, the query's own order; candidates: persisted queue, lazy spawn). Ask of every enum variant and file on disk: which decision does it save, and would a refusal say the same?
3. **A guard gets a guard**: check, breaker, census (11 breaker scripts; `arch-check.sh` is 1,183 lines of grep). Structure-over-discipline is right, but the instrument is spelling. Ask: which checks could be a dependency edge or a type?
4. **Prose as implementation**: registry, module doc, help, reference and README each restate behaviour. Ask: which sentence has exactly one reader?
5. **Research precision outruns the product question** (sittings, 2.4 GB of captures, about 8k lines of trade Python). The oracle is a real asset. Ask what user decision each open trade issue changes.

## 3. Running a review that finds these

**Phases, in order:**
1. **Census, no code review.** Every noun: state variants, files on disk, revision kinds (three `Basis` types, V), protocol variants, knobs, tools, checks. Each gets "what pays for this".
2. **Heat maps.** Findings by file, churn by file, lines per concept by grep.
3. **Thesis trials.** Each thesis gets an advocate, who must produce the deletion list by command, and a skeptic from another model family, who must produce the smallest breaking input.
4. **Worktree deletion spikes** for the top three, measured by `git diff --shortstat`, never landed.
5. **Directed line-reading** only in the hot files.
6. **Five one-page proposals at most** reach the owner.

**The brief** bans P1 to P3 grades in phases 1 to 3. It asks "what are these two things that are one", "what is this the third instance of" and "what would you delete", and it requires each reviewer to defend one thing others would cut.

**What `REVIEW-PROMPT.md` cannot find.** It is scoped to a diff against a brief, so the brief's premise, absence, aggregate and cost are out of frame. Its shapes are a fault taxonomy. C173 turns every finding into a rule plus a pin, so it can only add. It never sees two steps at once, and it has no grade for "unnecessary". Both worked removals came from tables, not reviews.

**Evidence standard for a radical finding:**
- its carriers enumerated by a named command;
- the loss stated as the smallest input whose behaviour changes;
- each ruling and invariant touched, with what holds it after;
- a spike's shortstat;
- a written dissent;
- one "morning" walked through after the change;
- its reversibility window (before `alpha.2`, protocol keys are free).

## 4. What I would not review

- **The limiter, gate and choke point**: a 4.7k-line spec suite, weekly mutants, journal oracles. Just close the semantic audit's two latent items (the `debug_assert!`s and `BlockShape`).
- **Transport** (236 lines), **`Secret`/auth**, and the protocol's wire fixtures.
- **Search's parse, bind and eval core**: pure, property-tested, oracle-backed.
- **Ingest semantics**, beyond the audit's tab-name fix.
- **Mock-as-mode**: it is the safer design for agent shells.
- **Language choice**: the retrospective settled it.
- **Corpus load** (268 ms at 22.7k items), until a GUI performance target is ruled.
- **Release, licensing and packaging.**
- **The per-step cold review itself**: keep it for what it does.

One sizing I did not verify beyond `wc -l` and my own grouping: the files for process lifecycle and queue durability (client crate, `world*.rs`, `store/jobs.rs`, `start.rs`, `life.rs`, `queue.rs`, `entry.rs`) total about 10.6k source lines, against about 5.7k for `ratelimit/`, `gate.rs` and `rails.rs`. The review should measure that properly before leaning on it.
