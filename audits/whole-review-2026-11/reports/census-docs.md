All ten tables below. The scripts are in the scratchpad: scratch/ (t1.py, t4.py, t7.py, t10.py). Tokens are bytes/4, and "last" is the date of the last commit touching the path.

**1. Tracked .md/.txt outside search/, roadmap/reviews, roadmap/briefs, audits, crates** (git ls-files, wc, `git log -1`; 41 files, 990,255 bytes total)

| path | bytes | tok | lines | last | commits |
|---|---|---|---|---|---|
| roadmap/LEDGER.md | 139162 | 34790 | 113 | 2026-11-09 | 81 |
| roadmap/STATE-2026-10-19.md | 94285 | 23571 | 966 | 2026-10-20 | 4 |
| CLI-REFERENCE.md | 76043 | 19010 | 904 | 2026-11-24 | 125 |
| docs/design/network-ground-truth.md | 74166 | 18541 | 1342 | 2026-11-18 | 31 |
| RUN-LEDGER.md | 69261 | 17315 | 92 | 2026-11-18 | 18 |
| docs/design/trade-ground-truth.md | 56733 | 14183 | 1001 | 2026-11-24 | 21 |
| roadmap/LANGUAGE-RETROSPECTIVE-2026-11-09.md | 41279 | 10319 | 702 | 2026-11-09 | 1 |
| roadmap/GUI-NOTES.md | 39652 | 9913 | 729 | 2026-10-21 | 4 |
| MCP-REFERENCE.md | 35258 | 8814 | 272 | 2026-11-23 | 127 |
| decisions/daemon.md | 30753 | 7688 | 67 | 2026-11-11 | 95 |
| REVIEW-CHECKLIST.md | 26836 | 6709 | 120 | 2026-11-23 | 38 |
| README.md | 23274 | 5818 | 358 | 2026-11-24 | 140 |
| AGENTS.md | 20802 | 5200 | 213 | 2026-11-23 | 66 |
| decisions/frontends.md | 20145 | 5036 | 42 | 2026-11-23 | 76 |
| decisions/search.md | 17766 | 4441 | 47 | 2026-11-23 | 43 |
| decisions/network.md | 17340 | 4335 | 35 | 2026-11-18 | 41 |
| roadmap/STATE-2026-10-26.md | 16097 | 4024 | 86 | 2026-10-26 | 1 |
| decisions/store.md | 15586 | 3896 | 40 | 2026-11-21 | 32 |
| decisions/process.md | 14760 | 3690 | 52 | 2026-11-24 | 15 |
| roadmap/TEST-SHAPE.md | 14277 | 3569 | 246 | 2026-10-30 | 4 |
| SEAT-LEDGER.md | 14190 | 3547 | 46 | 2026-11-23 | 9 |
| TESTING-NOTES.md | 13936 | 3484 | 239 | 2026-11-20 | 19 |
| .claude/skills/seat/SKILL.md | 12644 | 3161 | 212 | 2026-11-20 | 14 |
| .claude/skills/live-run/SKILL.md | 12607 | 3151 | 229 | 2026-11-20 | 19 |
| .claude/skills/site-sitting/SKILL.md | 11039 | 2759 | 192 | 2026-11-20 | 6 |
| decisions/pricing.md | 10926 | 2731 | 30 | 2026-11-20 | 13 |
| SURFACES.md | 10498 | 2624 | 32 | 2026-11-23 | 11 |
| decisions/plans.md | 8692 | 2173 | 27 | 2026-11-09 | 24 |
| roadmap/TIME-SEAM.md | 8684 | 2171 | 165 | 2026-10-31 | 3 |
| GLOSSARY.md | 8136 | 2034 | 146 | 2026-11-20 | 8 |
| decisions/trade.md | 7484 | 1871 | 38 | 2026-11-24 | 24 |
| HISTORY.md | 4865 | 1216 | 28 | 2026-11-10 | 7 |
| .claude/skills/session-close/SKILL.md | 4655 | 1163 | 99 | 2026-11-23 | 27 |
| roadmap/README.md | 4376 | 1094 | 91 | 2026-11-24 | 402 |
| .claude/skills/cold-review/SKILL.md | 3950 | 987 | 86 | 2026-11-12 | 5 |
| .claude/skills/mock-session/SKILL.md | 3426 | 856 | 63 | 2026-11-18 | 9 |
| .claude/skills/research-track/SKILL.md | 3237 | 809 | 62 | 2026-11-20 | 5 |
| CONTRIBUTING.md | 1498 | 374 | 38 | 2026-11-09 | 4 |
| decisions/ci.md | 1093 | 273 | 7 | 2026-11-24 | 5 |
| SECURITY.md | 833 | 208 | 25 | 2026-10-18 | 1 |
| CLAUDE.md | 11 | 2 | 1 | 2026-06-14 | 1 |

Caveat: the file set is `.md` and `.txt` only, so decisions/shape-baseline.tsv is excluded. The skills' SKILL.md files are listed here and again in table 3.

**2. decisions/*.md** (entries are bullets starting `- **C<n>`)

| file | entries | min B | median B | max B | "Pinned: nothing yet" | >800 B |
|---|---|---|---|---|---|---|
| ci.md | 1 | 672 | 672 | 672 | 1 | 0 |
| daemon.md | 38 | 169 | 767 | 3533 | 1 | 14 |
| frontends.md | 20 | 431 | 764 | 3293 | 1 | 9 |
| network.md | 22 | 270 | 726 | 1694 | 2 | 5 |
| plans.md | 12 | 398 | 608 | 1493 | 0 | 2 |
| pricing.md | 15 | 443 | 653 | 746 | 3 | 0 |
| process.md | 24 | 279 | 550 | 1294 | 16 | 2 |
| search.md | 22 | 474 | 733 | 2072 | 0 | 3 |
| store.md | 21 | 322 | 638 | 1768 | 2 | 4 |
| trade.md | 7 | 468 | 726 | 2543 | 0 | 1 |

- Total: 182 entries, with 26 "Pinned: nothing yet" and 40 over 800 B. The highest C id is C200.
- The 18 C ids in AGENTS.md are: 1, 2, 4, 12, 14, 15, 34, 35, 38, 46, 47, 48, 79, 110, 120, 122, 153, 154.
- Caveat: AGENTS.md's own bullets (as opposed to the decisions files') are not counted in the entry totals. The "nothing yet" count is a substring match anywhere in an entry.

**3. .claude/skills**

| skill | files | bytes | tok | last | commits |
|---|---|---|---|---|---|
| cold-review | 1 | 3950 | 987 | 2026-11-12 | 5 |
| live-run | 1 | 12607 | 3151 | 2026-11-20 | 19 |
| mock-session | 1 | 3426 | 856 | 2026-11-18 | 9 |
| research-track | 1 | 3237 | 809 | 2026-11-20 | 5 |
| seat | 5 | 58535 | 14633 | 2026-11-20 | 25 |
| session-close | 1 | 4655 | 1163 | 2026-11-23 | 27 |
| site-sitting | 1 | 11039 | 2759 | 2026-11-20 | 6 |

Caveat: seat's 5 files are SKILL.md plus 4 others (about 46 KB), and the seat directory is nearly half of all skill bytes.

**4. roadmap/briefs and roadmap/reviews, by month of first commit** (`git log --diff-filter=A --name-only`)

| dir | month | files added | bytes (current on disk) |
|---|---|---|---|
| briefs | 2026-10 | 93 | 1,329,647 |
| briefs | 2026-11 | 77 | 1,108,762 |
| reviews | 2026-10 | 228 | 575,600 |
| reviews | 2026-11 | 386 | 1,311,174 |

- Reviews: 565 tracked now, of which 105 end in `-prompt.md`.
- Caveat: "files added" counts every file ever added, so it exceeds the tracked count (614 vs 565 for reviews). Bytes are for files still on disk, and deleted files count as 0.

**5. audits/**

| subdir | files | bytes | first | last |
|---|---|---|---|---|
| answer-facts-2026-11 | 18 | 279079 | 2026-11-04 | 2026-11-04 |
| assessment-6-5 | 2 | 33457 | 2026-11-20 | 2026-11-20 |
| governance-2026-11 | 7 | 57757 | 2026-11-09 | 2026-11-10 |
| league-2026-11 | 16 | 262225 | 2026-11-01 | 2026-11-01 |
| readiness-2026-10 | 7 | 72917 | 2026-10-27 | 2026-10-27 |
| rehome-2026-10 | 29 | 562472 | 2026-10-12 | 2026-10-13 |
| release-2026-10 | 22 | 346893 | 2026-10-19 | 2026-10-19 |
| relicense-2026-10 | 17 | 107846 | 2026-10-12 | 2026-10-12 |
| search-stage-5-audit-results.md (a file, not a directory) | 1 | 10968 | 2026-10-12 | 2026-10-12 |
| semantic-model-2026-11 | 8 | 176194 | 2026-11-09 | 2026-11-09 |
| trade-2026-10 | 42 | 499437 | 2026-10-17 | 2026-10-18 |
| web-2026-11 | 20 | 671405 | 2026-11-07 | 2026-11-12 |
| web-form | 8 | 115141 | 2026-11-23 | 2026-11-24 |
| web-step6 | 5 | 60533 | 2026-11-19 | 2026-11-19 |

Caveat: dates come from `git log` over each path.

**6. Generated references**

| file | bytes | tok | `## ` headings | `### ` headings |
|---|---|---|---|---|
| CLI-REFERENCE.md | 76043 | 19010 | 55 | 0 |
| MCP-REFERENCE.md | 35258 | 8814 | 2 | 30 |

Caveat: the CLI's `##` headings are its commands (a proxy for verbs). The MCP's `###` headings are its tools, 30 of them.

**7. Issue tracker** (305 issues total: 86 open, 219 closed)

Open issues by area label (every open issue has an `area:` label, so none lack one):

| area | open |
|---|---|
| frontends | 29 |
| search | 19 |
| daemon | 17 |
| pricing | 9 |
| store | 8 |
| network | 7 |
| process | 6 |
| trade | 5 |
| plans | 2 |

Open issues by trigger label (every open issue has exactly one `trigger:` label):

| trigger | open |
|---|---|
| step | 36 |
| user | 18 |
| gui | 12 |
| live | 10 |
| consumer | 5 |
| seat | 4 |
| ci | 1 |

Open issue age: median 15 days. The oldest 10 are all 34 days old: #48 "The price plan and its receipt (C71, C78)", #47 "An other reading for a bound within one, and for map_tier", #46 "A named stat value", #45 "Realm segment semantics per endpoint...", #43 "Retention: acq store prune", #41 "Alternatives for a node", #40 "One explain over every term of a zero answer", #39 "The never fetched beside the total...", #37 "Whether the site can list an item...", #33 "What no sitting has asked the site".

Issues opened and closed per week, last 12 weeks (week starting Monday; the earliest issue is from the week of 2026-10-19):

| week | opened | closed |
|---|---|---|
| 2026-09-07 to 2026-10-12 (6 weeks) | 0 | 0 |
| 2026-10-19 | 68 | 1 |
| 2026-10-26 | 78 | 80 |
| 2026-11-02 | 32 | 37 |
| 2026-11-09 | 77 | 46 |
| 2026-11-16 | 28 | 30 |
| 2026-11-23 (partial) | 22 | 25 |

Open issues per milestone:

| milestone | open |
|---|---|
| (none) | 48 |
| 4-full, the GUI | 33 |
| League 3.30 and alpha.2 | 3 |
| PoE2 launch | 2 |

**8. Commits per ISO week since 2026-07-01** (`git log --no-merges --name-only`; `git log --format=%ad` lists the plain commit counts per week)

| week | commits (no-merges) | only crates/ | crates/ + other | no crates/ |
|---|---|---|---|---|
| W28 | 70 | 0 | 0 | 70 |
| W29 | 69 | 0 | 0 | 69 |
| W30 | 87 | 0 | 0 | 87 |
| W31 | 102 | 0 | 0 | 102 |
| W32 | 83 | 0 | 0 | 83 |
| W33 | 203 | 0 | 0 | 203 |
| W34 | 65 | 0 | 0 | 65 |
| W35 | 97 | 0 | 0 | 97 |
| W36 | 153 | 0 | 0 | 153 |
| W37 | 250 | 0 | 0 | 250 |
| W38 | 137 | 0 | 0 | 137 |
| W39 | 163 | 0 | 0 | 163 |
| W40 | 36 | 0 | 0 | 36 |
| W41 | 103 | 0 | 0 | 103 |
| W42 | 235 | 39 | 82 | 114 |
| W43 | 369 | 77 | 103 | 189 |
| W44 | 872 | 203 | 146 | 523 |
| W45 | 418 | 110 | 61 | 247 |
| W46 | 257 | 46 | 31 | 180 |
| W47 | 222 | 61 | 20 | 141 |
| W48 | 180 | 37 | 19 | 124 |

Caveats:
- The `crates/` directory exists only from W42, and earlier work lived under spikes/. Before W42 the "no crates/" column is simply everything.
- The `git log` totals including merges are slightly higher in some weeks: W28 76, W29 77, W30 94, W31 114, W32 86, W33 207, W34 70, W35 106, W36 153, W37 250, W38 139, W39 163, W40 36, W41 103, W42 235, W43 371, W44 872, W45 418, W46 257, W47 222, W48 180.
- Commits with no changed paths (empty commits) fall under "no crates/".
- W44 had 872 commits.

**9. Commit subject prefixes since 2026-08-01** (first word before the first colon)

| prefix | count |
|---|---|
| roadmap | 801 |
| search | 262 |
| spike(rate-limit-core) | 217 |
| `search,` | 187 |
| reviews | 178 |
| daemon | 134 |
| pricing | 106 |
| trade | 70 |
| docs | 68 |
| cli | 67 |
| web | 66 |
| store | 56 |
| docs(spike) | 50 |
| mcp | 48 |
| close | 41 |
| release | 39 |
| Daemon | 39 |
| tools | 38 |
| briefs | 37 |
| Commit | 36 |
| tests | 32 |
| decisions | 32 |
| brief | 31 |
| Search | 24 |
| platforms | 23 |
| seat | 22 |
| audits | 21 |
| acq | 19 |
| plan | 18 |
| spike(rust-playground) | 16 |

Caveat: the prefix is only the first whitespace-delimited word before the colon, and only for subjects containing a colon. Case variants (Daemon/daemon, Search/search, brief/briefs) and multi-scope forms such as `search,` are not merged, so merged counts would be higher (search combined is about 473, daemon 173).

**10. CI runs** (`gh run list --limit 200`; the 200 runs span only 2026-11-24T00:03Z to 2026-11-25T05:05Z, about 29 hours)

| workflow | runs | success | failure | cancelled | median min | p90 min |
|---|---|---|---|---|---|---|
| Gate (Windows) | 66 | 13 | 3 | 50 | 6.0 | 14.9 |
| Gate (Linux) | 66 | 16 | 1 | 49 | 5.9 | 12.4 |
| Committer | 66 | 66 | 0 | 0 | 0.2 | 0.3 |
| Keychain round trip | 1 | 1 | 0 | 0 | 1.3 | 1.3 |
| Release | 1 | 1 | 0 | 0 | 7.7 | 7.7 |

Caveat: duration is `updatedAt - createdAt`, so it includes queue time, and it counts cancelled runs (about 75% of the Gate runs, probably superseded pushes). The window is far too short to show a trend.
