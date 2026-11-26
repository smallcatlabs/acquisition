## Verdict: NARROWED

1. The core of the hypothesis holds. No signal from outside the project has ever reached the road. Every one of the 305 issues was filed under the owner's login, and 0 of the 58 `trigger:user` issues record anyone but him.
2. Two parts of it break. The cheapest outside signal is not "ten real hands". It is one named person who asked for this exact product and has never been asked to try it. Also, the owner's own daily use, which the project already named as "the fastest way to find out whether the tool is right", is not happening.
3. **The deciding number: 23 downloads of the alpha.1 archives, against 0 outside reports.** Outsiders already touch the tool, and nothing is set up to hear from them. (`gh release view v1.0.0-alpha.1 --json assets`: 3+1+1+10+8. `gh api repos/.../traffic/views`: 125 views, 18 unique visitors in 14 days.)

## Evidence

**Where the issues came from.** I classified all issues with a keyword script over title and body (`gh issue list --state all --limit 400 --json ...`). Per class: count, how many are open, and the median days a closed one waited.

| Class | n | open | median days to close |
|---|---|---|---|
| unclassified (mostly worker and build findings) | 103 | 36 | 1.1 |
| seat | 41 | 10 | 0.5 |
| audit | 38 | 10 | 1.0 |
| site sitting | 30 | 12 | 0.1 |
| owner at the pages (sittings in his store) | 25 | 8 | 1.4 |
| review | 24 | 2 | 1.0 |
| parks ("reopen on a user's report") | 15 | 0 | 13.2 |
| owner's own use | 12 | 5 | 0.1 |
| CI | 11 | 0 | 0.3 |
| live run | 6 | 3 | 0.2 |

- **What `trigger:user` holds.** I ran the same script over the 58 issues with that label: owner sittings 15, unclassified 15, Codex/cold review 7, parked ideas 5, seat 4, audit 4, site sitting 3, owner use 3.
  - The label means "a user would notice". It does not mean "a user reported it".
  - The nearest thing to a real report from use is the owner's own stops at `acq web`, for example #243, #248 and #327.
- **Which source changed the product most (judgment).** I judged by whether an issue changed a page's structure, a ruling, or behaviour GGG sees, rather than words.
  - The owner's use and sittings in his own store reshaped the GUI: #248 (pages of places replacing Refresh), #244 (the dense table), and #246 (the main-account ruling, later removed).
  - They also produced the one fix to behaviour GGG sees that came from use. On 2026-11-18, his own Standard refresh sent 90 consecutive requests that were all refused with 400, because "nothing counts answers across jobs". That became C190 and #288 (`RUN-LEDGER.md`, last row).
  - Seats are fast and plentiful, but their fates in `SEAT-LEDGER.md` are mostly fixes to wording.

**The owner's use for the tool's purpose is thin.** All from the review's measurements on a copy of his store (`findings/ground-truth.md`):
- 17 prices set, all in one sitting in September.
- 4 saved searches.
- Top-level jobs: 8 applies and 4 listings over four days, then none in the following five.
- In the review's words, "pricing and the shop have been used once".

`RUN-LEDGER.md` has 59 rows: 25 in August, 19 in September, 7 in October, 8 in November (`grep '^| 2026'`). The rows that are use rather than tests are 2026-10-25 and 2026-11-10 (evaluation sittings), 2026-11-17 (league-launch refreshes) and 2026-11-18.

The 4-full design prompt (`roadmap/briefs/4-full-design-prompt.md`, 2026-11-06) says the owner "does not have a solid feel for what users will want", and also that daily owner use is the fastest test. The measured use does not meet that intent.

**The first plain question.** "Which of my items have 90 or more life", asked on his store, took six tries: five refusals, then an answer (`findings/ground-truth.md`). The portfolio's reading was that seats "sit in pristine worlds of one realm and one account". So the inside instrument was blind to the realistic world; the review sees that as the method's blind spot, not a matter of how much effort went in. One real store found in an hour what months of seats did not.

**Where the effort went** (`git log --since=2026-10-12 --numstat`, lines changed and commits).

| Area | Lines changed | Commits |
|---|---|---|
| `search/` research data | 463k | |
| `acquisition-search` crate | 203k | 237 |
| `acquisition-cli` crate | 111k | 517 |
| `acquisition-daemon` crate | 99k | 292 |
| `roadmap/` | 89k | 1,208 |
| `audits/` | 44k | |

- 1,309 of 2,550 commits (51%) touch only `roadmap/` or `audits/`.
- By message keyword, and the themes overlap: sitting/trade/site 619, web/GUI/page 679, seat 450.
- Trade-named source and tooling, data files excluded, is about 22k lines (`git ls-files | grep -iE 'sitting|trade|capture|site' | xargs wc -l`).

**What the sittings cost and bought.** The owner has hand-run 1,255 trade-site searches (`reports/investigators/search-kernel.md`). In a sample of 467 of them, no answer differed on his store: 122 clauses carried a stake, each "adds 0, drops 0". I did not re-run that. What the sittings bought a user: links the tool composes now open correctly on the site (stop 9 of 2026-11-24), and #311.

**The GGG standing does not argue against ten users.**
- Under OAuth, every API rule is Account-scoped and there is no Client rule (`docs/design/network-ground-truth.md`, line 415). A stranger's traffic spends the stranger's own limits.
- The daemon is the single gate (invariants 1 to 3, pinned).
- The C++ app already runs under the same registration for real users: 25 to 73 downloads per release (`gh api .../acquisition-qt/releases`), and 9 issues from people other than the owner since June.
- The real risk is a defect that reaches every user at once, like the 400 run of 2026-11-18. Owner use found that one, and C190 now caps it.
- Judgment: ten alpha users add less exposure than the C++ app's existing users. I did not check the C++ app's limiter.

**What does argue against ten users now: freezing the surface.**
- The review proposes cutting about 30 MCP tools to about 14 and removing nine CLI verbs.
- Facts migrations are still routine (facts v7, `Store::open` migrates).
- Then there is support: the owner already answers C++ issues that run to 6 to 12 comments.
- Installing is curl, checksum and tar, for a CLI only. macOS builds are signed but not notarized; Windows builds are unsigned.
- A broad release now would cost more than it teaches. That is why the verdict is narrowed, not upheld.

**No evidence of agent use for a real purpose.** The only case is the 2026-11-10 morning, where the reviewing session itself was the agent at `acq-mcp`, at his word, in his store (`audits/web-2026-11/sitting/README.md`). Even that hit two refusals over scope (A1, A2). There is nothing weekly. Seeing it weekly would take a player's own agent with a call log they agree to share. `acq diagnose` already exists as the paste format for a report.

## The answer

The product frame is the blind spot, but not in the way the hypothesis says. The missing instrument is **one real world used for its purpose, with a channel back**. The owner's store is one such world, unused between evaluations. The person below is the second, and has never been asked.

**Smallest reversible change, within two weeks.** One direct ask to the person behind acquisition-qt #187, "Headless Support". This is the outside request that, in the owner's own words on that thread on 2026-10-29, inspired the rewrite.
- In July that person wrote that it was "on my radar". The owner later posted only a link, and there has been no reply since.
- The ask: point `acq-mcp` from alpha.1 at your own stash with your own agent. Send back the call count to a first answer and any `acq diagnose` paste.
- The same ask could go to the C++ app's recent outside reporters, for example on #234 (tabs inside folders not marked as such) or #197 (old prices retained or dropped at random).
- It is undone by not asking again. It needs no new release, no changed surface, and no GGG exposure beyond what the C++ app's users already carry.

**The number it moves: issues not authored by the owner's login** in this repository, 0 today. Because agents file through his account, the author field is the one field that cleanly separates outside reports from inside ones; no label can. Read it with `gh issue list --state all --json author --jq '[.[]|select(.author.login!="<owner>")]|length'`. A second number: calls to the first correct answer on a stranger's world, set beside the six on the owner's.

**What the inside signals have that outside use would not:** speed (a seat issue closes in a median of 0.5 days), reproducibility (pristine worlds, logged calls), and safety near the GGG standing. They keep that value as regression checks. They cannot choose what to build.

## What nobody named

- **The rewrite's origin customer is lost.** acquisition-qt #187 asked for an agent-driven headless client, and the rewrite was built in answer to it. The one outside agent-user the project has ever had was never asked to sit at the result.
- **A free stream of real users' trouble already exists, and is read once.**
  - The C++ app's outside issues (6 since August) are real trouble on real stores, read by the rewrite only once, on 2026-10-19 (`audits/release-2026-10/cpp-issues.md`).
  - None of them touches trade-site exactness, which took about a quarter of the commits by keyword. Judgment: the road is set by the owner's evaluation sessions, not by what players report.
- **`trigger:user` has drifted to mean "imagined user".** The project believes its user triggers are 18 open and 40 closed, while its count of real outside users is 0. The 15 parks marked "reopen on a user's report" can never fire as things stand.
- **23 archive downloads and 18 unique visitors produced 0 reports.**
  - The README's "Reporting" section asks for a GitHub issue, which is a high bar for a player.
  - Judgment: either the alpha failed silently in strangers' hands, or nobody asked them anything. Either way, the project cannot tell.

**Not checked:**
- Who the downloaders were. Some of the 10 Windows x64 downloads may be the owner's test VMs.
- Whether the note to GGG was sent.
- The C++ app's limiter.
- The owner's hours beyond the 1,255 searches.
- The traffic API covers 14 days only.

Scratch, not committed: `<scratchpad>/cls.py` (the classifier) and `issues.json`.
