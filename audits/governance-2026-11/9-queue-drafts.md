Below are five issues to file and eight items I dropped as settled. List A had 12 entries; two of them were already settled, so issue 1 has ten and its title says "ten entries", not "twelve". List B's lone store item went into the daemon issue, so that issue carries both area labels. I filed nothing and made no gh writes.

Line numbers for `roadmap/README.md` are at HEAD 73e7e77c. Someone edited that file in the working tree while I was reading it (it is now 79 lines, and `decisions/process.md` is new and untracked), so check every README line number again after the cut. LEDGER line numbers match the tree as it stands.

=== ISSUE 1 ===
Title: Registry words changed under delegation: ten entries for your read
Labels: trigger:ruling, area:process
Body:
Agent sessions changed or added these registry words (the numbered rulings in `AGENTS.md` and `decisions/*.md`) under your delegation. Each was recorded only in a commit message or the roadmap. Tick an item if its words stand; otherwise rule on it here. Two other harvested entries (C127, C129) turned out to be settled already and are not listed.

- [ ] **The cap on cross-cutting rulings**: `tools/docs-check.sh` sets `CROSS_LIMIT=18`, the most rulings the always-loaded list in `AGENTS.md` may hold, and it holds 18 now. One session raised the cap from 15 to 16 to admit C122 (behaviours pinned once). Another raised it from 16 to 18 for C153 and C154 (the stopping and retirement rules), reading your budget word ("raise the AGENTS.md budget as needed for the duration of this work") as covering the count, which that word never named. Source: a19dbce5, 12a04b24, 75c0bc44. Waits on it: the next cross-cutting entry, which fails the docs check.
- [ ] **C27 (closing a client cancels nothing)**: a session added "The daemon runs in a process group of its own on every OS", so a Ctrl-C or a closed terminal no longer kills a daemon that a command started on macOS or Linux. Source: 69c7eede. Nothing waits on it.
- [ ] **C128 (the token goes only to GGG's API)**: a new entry. The one place that attaches the access token refuses any GET or HEAD whose URL is not the provider's API origin, before any wait. It sends nothing and does not read the token, and a refused probe closes its route for a cooldown. Source: c429c2a8. Nothing waits on it.
- [ ] **C25 (a halt fails no job)**: the entry's words are yours, and only its test name changed. Beneath it, the session decided two behaviours and called them its own. While halted, the daemon picks no job that would send. A Cloudflare block on the token request parks its job instead of failing it. Source: 8d2d6991. Nothing waits on it.
- [ ] **C141 (a trade JSON carries its league)**: a new entry, made as agent-facing design. The site's query JSON may come as `{league, query, sort}`, with the league read as a term and any other key reported unsupported. The alternative, rewriting it as text with `league:` added, was declined because it loses the per-clause statuses. Source: eeb72437. Nothing waits on it.
- [ ] **C142 (how defence filters are read)**: a new entry. `defences: "displayed"` reads Armour, Evasion, Energy Shield and Ward as the item shows them; absent or `"q20"`, they are read on the site's normalised values. You asked for the search and said it "may not be best implemented as a boolean flag"; the session chose this option. Source: f2239aa6. Nothing waits on it.
- [ ] **C86 (the 5 s handshake deadline)**: the words are unchanged. A session judged that applying the same deadline knob to every read-only request does not widen C86, so it amended nothing. If you read it as widened, the entry is yours to amend; the code already matches. Source: 35de06b8. Nothing waits on it.
- [ ] **C137's knob list (deadlines tested at expiry)**: `ACQ_LOGIN_WITHIN`, the daemon's wait for the browser login (300 s), is not on the list. The session judged it a user's deadline, not a liveness one, and did not amend the entry. Question: should it join the list? Source: 28362ba6. Nothing waits on it.
- [ ] **C160 (daemon timers on one clock)**: you said "yes to all" to "every wait in the daemon's sources sleeps through `Clock`" (7287b42c). Under the grant of 2026-11-01, a session narrowed that to "Every tokio timer", and narrowed the second clause to "nothing the hook accounts for is pending". Your three later "as drafted" amendments changed other clauses; the narrowing itself was never put to you. Source: 132a43ef, 2e0fa403; roadmap/README.md:649. Nothing waits on it.
- [ ] **The store's "Not doing" line on a reserved-name facts file**: `decisions/store.md:37` reads "not until `user`, a report of a real world holding one". The rulings behind it are yours ("Recovery refusal", "Shrink and close"). The line's words were written under delegation, and the step closed without a review of them. Source: f073c494; roadmap/README.md:310. Nothing waits on it.

=== ISSUE 2 ===
Title: Parked daemon and store decisions: four for your ruling
Labels: trigger:ruling, area:daemon, area:store
Body:
Sessions parked these questions for you in commit messages and in `roadmap/LEDGER.md`, and nothing queued them. One question per item.

- [ ] **A stopping daemon is not shown as stopping**: once a stop begins, the daemon refuses new work with "the daemon is stopping". `acq daemon status` deliberately does not report it; the code comment says the halt's words name a remedy and a stop has none. Question: should status say the daemon is stopping? Source: 0c4a0829; `crates/acquisition-daemon/src/rails.rs:547`. Nothing waits on it.
- [ ] **`acq daemon stop` after an unconfirmed exit**: when the daemon acknowledges a stop but is not seen gone within the bound, `acq daemon stop` still prints "daemon stopped". The restart path already says the daemon "did not confirm its exit" (your "Yes, as proposed", c526a3c3). The session called changing this verb's words and exit code "a design of its own". Question: change them, or keep them? Source: 380ef102; roadmap/LEDGER.md:49. Nothing waits on it.
- [ ] **Should the dashboard request page?**: the job list now pages. `Request::Dashboard` still answers whole, and an answer past the size bound would come back as an internal error. Question: page it now, or leave it until a dashboard nears the bound? Source: roadmap/LEDGER.md:95 (4587d782). Nothing waits on it.
- [ ] **The annotation file's wait knob**: `ACQ_STORE_WAIT` sets the wait for the facts and queue files. The annotation file (prices, policy) waits on `ACQ_ANNOTATION_WAIT` throughout, including its journal-mode retry, which the brief had put under the store knob. The session chose this so that each file has one knob. Options: keep it, or switch the retry to the store knob, a one-argument change. Source: 8de554bf. Nothing waits on it.

=== ISSUE 3 ===
Title: Parked network decisions: three for your ruling
Labels: trigger:ruling, area:network
Body:
Sessions parked these questions for you in commit messages and in `roadmap/LEDGER.md`, and nothing queued them.

- [ ] **A blocked HEAD 429 holds its route**: when Cloudflare answers a HEAD probe with a 429, the session made it install the route's retry hold and count a violation, as a blocked GET 429 already does. That way a reset inside Cloudflare's Retry-After still waits it out. A HEAD 403 without `cf-mitigated` still degrades the route, by your narrow rule. Question: keep the hold? Source: 6598d873. Nothing waits on it.
- [ ] **The send journal keeps GGG's headers verbatim**: the journal, the daemon's record of every request to GGG, is the one file on disk that holds GGG's text as sent: its rate-limit and header snapshots. The contract tests rely on them. This was raised next to invariant 5 (no refresh token in plaintext on disk). Question: does keeping them stand? Source: 380ef102; roadmap/LEDGER.md:49. Nothing waits on it.
- [ ] **The HTTP crate's public API is held by review**: the crate that holds the one HTTP client must export no reqwest item. No check enforces this: a trial `pub use reqwest` built cleanly and the dependency check could not see it, and stable Rust has no lint for it. The session kept it as a review rule: a `REVIEW-CHECKLIST.md` row, plus the crate's doc listing its public items. Question: accept review, or ask for a check? Source: ac4642f5, 65aca586; roadmap/LEDGER.md:75. Nothing waits on it.

=== ISSUE 4 ===
Title: Parked frontend decisions: nine for your ruling
Labels: trigger:ruling, area:frontends
Body:
Sessions and seats (blind agent runs at the tool) parked these questions for you, and nothing queued them.

- [ ] **C52 (no blind replace of intent) beside a price conflict's offer**: a price write refused for a stale revision now shows the stored row and offers the same write at that revision. The session judged this within C52, since the agent has been shown what it writes over. Question: does that stand? Source: 4c8bc17d; audits/league-2026-11/seat/README.md:197. Nothing waits on it.
- [ ] **A policy conflict offers nothing**: `policy set` with a wrong revision names the stored revision but offers no write, unlike a price conflict. The round's brief left it out. Question: should it offer one? Source: audits/league-2026-11/seat/README.md:199. Nothing waits on it.
- [ ] **C3 (the daemon starts on demand) under the mock**: a job verb with no login still starts a daemon and says so. Four seats met this as a surprise under the mock provider, and each kept it. Question: keep it, or have a refused job verb start nothing? Source: audits/answer-facts-2026-11/seat/README.md:200, :204. Nothing waits on it.
- [ ] **C36 (the sync policy) has no clear**: no verb clears a policy. A seat asked for a clear or a warning before the first write; the session kept C36 and added a README warning (9ea28af9). Question: keep it with no clear verb? Source: audits/answer-facts-2026-11/seat/README.md:209. Nothing waits on it.
- [ ] **C53 (three output levels) on text receipts**: a text receipt omits `in_facts` under the default text view's rule, and the help says so; a seat read it as missing. Question: does that stand? Source: audits/answer-facts-2026-11/seat/README.md:210. Nothing waits on it.
- [ ] **Web pages print CLI remedies**: where no page can act, pages print a command instead. The cases are: a policy write, the policy past ten leagues, a login (`acq auth`), a job's result, and the daemon's commands. Question: which of these become page routes? Source: 3ce22d81. Nothing waits on it.
- [ ] **C168 (pages write as the verbs do) and "receipt"**: the entry says a page "answers the one receipt". Under your "Redirect to the list", the receipt is the JSON document and no page renders one. Question: should the entry say "receipt document"? Source: roadmap/README.md:333. Nothing waits on it.
- [ ] **Item lines at column 0 in the CLI**: with `--fields item`, the item's `base`, `class` and `lines` start at column 0, like the answer's own lines, and a seat found it hard to read. Question: indent them? Source: audits/web-2026-11/seat/README.md:103. Nothing waits on it.
- [ ] **The frontends' whole-code review**: a review like the one already done for the daemon, store and network is set for after the league. Question: keep that timing? Source: roadmap/README.md:502. Waits on it: the league run.

=== ISSUE 5 ===
Title: Parked process decisions: five for your ruling
Labels: trigger:ruling, area:process
Body:
Sessions parked these questions for you, and nothing queued them.

- [ ] **The mock-session skill's login line**: the old placeholder invited a shell substitution that zsh misreads, and the login timed out after 300 s. The skill now saves the authorize document to a file and approves with a literal `curl … sed` line that runs under zsh and bash. The session named it process wording an agent changed. Question: keep it? Source: 1f8053f0. Nothing waits on it.
- [ ] **The macOS Keychain test as a release job, option (a)**: the macOS Keychain round trip is a `keyring-macos` job in `release.yml`, and the release draft waits on it. The session took (a) as clear cut. Option (b) moves the job to `weekly.yml`. Question: (a) or (b)? Source: 9adad710; roadmap/README.md:65. Waits on it: the next release, the job's first run.
- [ ] **The gate's `artifacts` stage, kept against your ruling**: you retired the stage on 2026-10-29 (it checks that a test run leaves the same `acq`, `acqd` and `acq-mcp` a plain build writes) on the premise that the `features` stage held its property. The review showed `features` misses a profile split and a binary with no integration test, so the session reverted the retirement. roadmap/README.md:111 still says "retired". Question: keep the stage and correct that line, or retire it? Source: 45cb2946, d6a6a039. Waits on it: the next item.
- [ ] **The close skill's release-uplift sentences**: question: do the session-close skill's sentences on rebuilding release binaries stand now the gate keeps `artifacts`? Source: roadmap/README.md:514. Waits on it: the item above.
- [ ] **Table generators' `--check` in the local gate**: question: should the local gate run the class, defence and forms generators' `--check` where the game-data clone sits at its pin? It costs 9.1 s a gate, 8.8 s of it the forms table's, and 0.35 s without forms. Options: all three, all but forms, or neither. Source: roadmap/README.md:509. Nothing waits on it.

=== DROPPED AS SETTLED ===
- C127 (a client submits only listed kinds), entered and widened three times (718f1cde, 3bcbb8fa, 04eb4ff1, f3c70bde): the owner's "yes to all" of 2026-10-27 covered "C127's words" and landed the entry as it stands now (ea0b0255); no later commit touches it. Caveat: that walk-through described a rewording; the earlier widenings were not each named.
- C129's reading of "page marker" (99e2230b): the owner's "i accept your recommendation" (ed5e1cc0, the marker counts only without GGG's policy header), then his "yes for the cloudflare issue" (bced3c6c), which wrote "by a marker with no policy header" into the 5xx clause.
- The drafted C32 sentence (LEDGER.md:55): the owner's "yes" of 2026-10-30, 5c31632a.
- The `awakened-poe-trade` pin (LEDGER.md:55): the owner's "Yes, fix now", 75cf4cff.
- Whether a token POST should journal a line before it is sent (LEDGER.md:66): ruled by the owner on 2026-10-31, left until a live run shows it matters (c03a11c5, a "Not doing" line).
- The retention prune stays non-sticky (LEDGER.md:96): "I accept your recommendation for C6", 1130b231.
- The narrowings of the 4-0 design's section 4 (5a2c1725, 9736bdcf, e818ed51): the time-seam retrospective (roadmap/reviews/time-seam-retrospective.md) tables each narrowing in section 3, and its question 6(a) asks to close #147 "the narrowings recorded in §3". The owner answered "yes to all" (roadmap/README.md:645). The evidence is indirect, through the retrospective.
- "A stop loses a landed login", the first half of 0c4a0829: the owner's C50 ruling (26160d48) settles it. A lost login's profile fetch dies with its lifetime "and the user logs in again". The second half, status not showing the stopping halt, stays open in issue 2.

=== OWNER-ONLY ITEMS ===
- Whether GGG echoes `state` on a denied login: open. #68 closed 2026-10-28, but the fact was never read; README:504-508 at HEAD; `network-ground-truth.md` has no denial entry (grep for access_denied and denial finds nothing). It needs one denied live login.
- `tools/tracer-rung.sh` at the next live run: done. RUN-LEDGER.md 2026-11-03, the transport seam's rung, a pass on the second attempt (the first was stopped).
- `tools/persist-check.sh`: not run since 2026-08-30, the only RUN-LEDGER.md row that names it (line 54). README:500 says "edited at 2g, not run since".
- The `keyring-macos` release job's first run: never run. `gh run list --workflow release.yml` shows the last run on 2026-10-22 (39714828685), before the job landed on 2026-10-29 (04a5a908).
- The weekly `sign-acqd` job's first run: done. The scheduled Weekly run 42637884509 on 2026-11-09 shows `sign-acqd` success, all four mutants shards success. Of the four dispatched or scheduled weekly runs from 2026-11-01 to 2026-11-03, three failed; I did not read why.
- Dependabot closing #124 (ratatui 0.30.2): done. Closed 2026-10-29T19:27:55Z.
- The `runs/` copy of the 2026-10-25 sitting's journal (c3537ceb): not found. Listing `runs/` shows no entry dated 2026-10-25 and nothing named for 4b or a sitting.
- The owner's morning at `acq web` in his own store (LEDGER.md:102): pending. It is "Current step" at README HEAD; RUN-LEDGER.md has no row after 2026-11-03. The README records his evening sitting of 2026-11-07, but that sitting has no RUN-LEDGER row.
- Phase 3, the league run: not started. README: "Next, Phase 3, the owner's league run, at his word".

Outside your lists, I also saw these delegated items with no owner word found. I did not chase them:
- **C136 (the daemon's submitter is its own):** entered "under the owner's delegation" (26160d48).
- **C143 (a shared refusal speaks the request's words):** entered "under delegation, listed for review" (f2239aa6).
- **List paging's design re-read (LEDGER.md:95):** the answer's `counts`, the newest ten, `state` defaulting to `all`, `limit`'s range and the byte budget.
- **List paging's age flake:** seen in two tests in that row and in LEDGER.md:96.
