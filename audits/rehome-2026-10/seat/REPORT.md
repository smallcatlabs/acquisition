<!-- the seat's report for BRIEF.md, 2026-10-13; call numbers are rows of calls.csv. The runner's harness refused its write, so it assembled the text at the end of the run and handed it back; the session saved it here unedited. -->

## 1. Before touching anything

Read: the brief, the README's "The tour" (lines 64-136), and `--help` for
the top level, `auth`, `accounts`, `store`, `daemon`, `refresh`, `policy`,
`submit` (calls 1-8).

**What the tool can do** (call 1): log in by OAuth; ask the game API,
through a daemon that queues and rate-limits jobs, for profile, leagues,
characters, tab lists and single tabs; keep what it fetched in a store on
disk; compile a declared "sync policy" into a plan and apply it; search
stored items with a query language; record prices on items, tabs and
characters and render a forum shop; inspect and manage the job queue; show
a TUI dashboard.

**Which commands need what** (from the help; corrected by later calls):

- Store or binary only: `accounts`, `tabs`, `search`, `show`, `store …`,
  `price …`, `shop render`, `reference`, `version`, `policy show|set`,
  `refresh --plan` (confirmed with the daemon stopped, calls 159, 164).
- Observe the daemon, never start it: `jobs`, `daemon status` (calls 158, 160).
- Need a running daemon and do not start one: `auth status`, `status <id>`,
  `result <id>` (calls 10, 161-163). The tour glosses `auth status` as
  "local session", and `result` help says "answered across daemon restarts".
- Spawn the daemon and need a login: `profile`, `leagues`, `characters`,
  `character`, `stashes`, `stash`, `refresh --apply/--tabs/--all`, `submit`,
  `demo`.

**What I would not run without asking the person:**

- Any job command outside a rehearsal: the help's first line says the real
  GGG API is the default.
- `refresh --all`, `refresh --apply`, `demo`, `submit` (they spend requests).
- `auth logout`; `policy set` (replaces the whole policy); `price set` or
  `clear` on a tab or character (covers many items).
- `store rebuild`, `store import`.
- `daemon stop` (it lost the session here, call 170) and `daemon
  reset-tripwire` (its help cites a "post-violation rule" it does not state).

**The world as found:**

- One account, `GERWARIC#7694`, "not persisted", 54.4 MB (call 9).
- Store: 1127 responses, 0 leagues, 65 characters, 3444 tabs, 22721 items
  (+6 removed), 22751 events, 8 granted skills, 4 refused bodies (call 12).
- No daemon, "no queue on disk" (calls 11, 14).
- `acq 1.0.0-beta.1 (contract bd361a2f41bf)`, `acqd` in `target/release/`
  (call 13).

## 2. The tasks

### T1 — log in and look around (login + calls 15-22)

Told the person: you are `seat` (uuid 00000000-0000-4000-8000-00000035ce05,
guild Playground), token valid about 53 s, keyring disabled (calls 16, 17).
Leagues: Standard and Hardcore (call 18). Characters: StashHoarder (Scion
97) and MuleQuadTab (Witch 12), both Standard (call 19). Tabs in Standard:
Currency, Dump, folder "Old leagues" (Old 1, Old 2), Uniques, Maps, Maps
(empty) (call 20). MuleQuadTab holds a Starkonja's Head helm with a
Determination gem and 4 inventory items (call 21). Tab `dump` holds 5
"Fake Item" entries (call 22).

Right? Yes as the mock gives it. One disagreement: the list says
MuleQuadTab is Witch 12 (call 19), the fetch says Scion 97 (call 21). I
hold that to be the mock.

Stops: none. Job verbs print raw JSON in text mode with `job N: waiting /
done` lines above it on stdout. `stashes` does not say it defaulted to
Standard except in the payload's `league` field.

### T2 — a first sync (calls 23-42)

Told the person: the tool wants a sync policy. I set
`{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":"all","characters":"all","max_age_seconds":3600}}}}}`
(revision 1, call 29). The plan was 7 requests (6 tabs, 1 character), 7–21
wire sends, "ready"; 2 tabs skipped (1 fresh, 1 folder), 1 character fresh
(calls 31, 32). Apply ran as job 12: 7 done, +15 added at 4 locations
(call 33). The next plan was not empty: 7 more requests for substashes
under Uniques and Maps, "never fetched" (call 37). A second apply (job 20)
added +19 at 7 locations (call 39). The third plan: "nothing to do:
covered entries are fresh (under 1h)" (call 40). Landed: 15 tabs, 14
fetched, 31 items (call 41); 2 characters, 14 items (call 36); search
total 45 (call 42).

Right? Yes; 31 + 14 = 45 agrees across calls 36, 41 and 42.

Stops and misleads:
- `policy show` and `refresh --plan` refused without `--account`, although
  the job commands just before had run without it (calls 23, 25).
- The policy's fields are not in any help. I learned them from the empty
  `policy show` (call 26) and a refusal (call 28).
- The first plan gives no hint that substashes will follow. "Everything"
  takes two rounds.
- The plan says "ready" but never a duration, so "how long" has no number.

### T3 — the queue (calls 43-81, 152)

Told the person: six `sleep` jobs all ran at once and none waited (call
49). Thirteen `fetch` jobs did queue under the mock's 5-per-10 s limit:
"6 running, 8 waiting (next in ~11s)", with a per-job ETA of ~11s or ~27s
(call 64). `cancel 46` answered "job 46 cancel requested" (call 66) and the
job showed `cancelled` (calls 68, 70). `set-priority 45 9` answered "job 45
priority -> 9" and its ETA moved from ~27s to ~10s (calls 67-69). `result
34` gave the payload (call 72). `result 45` while waiting: exit 1, "job 45
is still waiting" (call 73); later the payload (call 77). `result 46`:
"job 46 was cancelled", exit 0 (call 71). Detached job 47 was submitted at
call 63 and its result read at call 152. `dash`: "stdout is not a terminal
— use `acq dash --json` for a one-shot snapshot" (call 81).

Right? Yes.

Stops:
- No verb waits for a job. I blocked on a foreground `sleep` job to pass
  time (call 76).
- "When will it finish" is per job only; the summary line gives "next in",
  not when the queue drains.
- `jobs` lists every finished job of the daemon's lifetime, with no filter
  (48 rows by call 68).
- Nothing says which kinds queue. I learned by trying that `sleep` does not
  and `fetch` does.

### T4 — what I have (calls 82-91, 102)

Told the person: 22721 live items (+6 removed), 3444 tabs, 65 characters,
in a 54.4 MB store (call 82).

- By league: 21311 Standard (pc), 1312 Allflame (pc), 98 Standard (poe2)
  (call 90).
- By container: 21332 in stash tabs, 1389 on characters (call 102).
- Largest tab types: PremiumStash 6197, QuadStash 4814, GemStash 2611
  (call 102).
- Largest tab: "ULTIMATUMS (Remove-only)", 499 items (call 102).
- Characters: 50 fetched (10 with empty bodies), 15 never fetched (call 83).
- Age: pc data 32–36 days old, poe2 41 days; 551 locations fetched, 2941
  never fetched (calls 88-90).
- Refused: 4 poe2 character bodies, 41 days ago, each "missing an `id` on
  an item at `equipment[n].socketedItems[0]`" (call 84).
- 8 item-granted skills left in place (call 82).
- No events in the last 24 h (call 85).

Right? The counts agree (22623 + 98 = 22721). Not explained: the four
refused characters still show 11–30 items "fetched 41d ago" (call 83).

Stops: `search` refused without `--realm` on a two-realm store (call 86);
the error named the fix. I did not run `tabs` for Standard, because about
3,300 rows would exceed the door's 60,000 bytes and the summary line is
last. `tabs` has no summary-only form.

### T5 — find things (calls 92-101)

- (a) Yes: 187 items with six linked sockets, all pc (181 Standard, 6
  Allflame); 132 body armours, 30 bows; 101 rare, 76 unique (calls 97, 98).
- (b) 224 rare rings with life and two or more resistances, by
  `rarity=rare class=Rings has:pseudo.total_life pseudo.count_res>=2`
  (call 99). "Life" here is the trade site's total-life pseudo, which also
  counts strength.
- (c) League by rarity, one table, by `--cross league,rarity` (call 100):
  Standard pc has 5615 rare, 2850 normal, 2819 unique, 2334 magic and 7693
  with no rarity. The Allflame and poe2 rows are in the same call.
- (d) `show` on Kraken Circle Bone Ring printed place, base, class,
  category, price, every line with its template and number, and "unread
  nothing" (call 101).

Right? I hold these right. The per-term counts and the printed routes
would show a wrong term.

Stops: `search --describe` is 117,523 bytes (call 92) and the door showed
3,000 characters. I learned the language through named entries (calls 94,
96) and through the error of call 95, which lists every name `--describe`
knows. There is no index mode.

### T6 — prices (calls 103-129)

Told the person, Standard (calls 105-108): 531 items listed; 39 decided by
hand, 492 by the game; 50 conflicts, all in tab ACQUISITION-PRICE-TEST
(hand row of 5 chaos on the tab); 1 invalid note; 3 skipped; 1395 items
carry price text the index cannot see. Allflame (calls 112, 128): 2
conflicts in tab "~price 222 divine" (99 chaos by hand against the tab's
222 divine; 800 chaos by hand against a "~price 5 exa" note), and 3
invalid notes.

Writes, on Allflame tab "Gear" `4ffd40ba0f` (16 items):
- `price set tab/pc/4ffd40ba0f exact 5 chaos` → "5 chaos (revision 1), was
  unset"; the tab then showed "16 by hand only" (calls 116, 117).
- `price set item/a79c… exact 2 divine` → "(revision 1), was unset" (calls
  118, 119).
- `price clear item/…` → "cleared, was 2 divine (revision 1)" plus the
  command that puts it back; the item then inherited the tab's 5 chaos
  (calls 121, 122).
- `price clear tab/… --if-revision 1` → cleared; the tab is back to "13
  unlisted, 3 by hand only" and status is identical to before (calls
  123-125, 129, against 112, 113).
- A second set and clear of the item was my own revision test (calls 126,
  127).

Noticing another program's change:
- Every row has a revision; `--if-revision N` refuses a stale write: "is
  at revision 1 (re-read and retry)" (call 120).
- Revisions do not restart after a clear: the re-set item came back at
  revision 3 (call 126).
- The search basis prints `intent`, which went from 32 (call 88) to 38
  (call 138) over my six accepted writes. No help says that `intent` is
  the price state; I inferred it.

Right? Yes; the before and after reads match.

Stops and misleads:
- `price show tab/pc/4ffd40ba0f` failed with "not in the facts for
  Standard" (call 111). Price verbs are per league, default Standard,
  though the target names only a realm.
- `price set … --league Allflame` is a usage error, exit 2 (call 114):
  `show` takes `--league`, `set` does not.
- The `next:` line after a set gives a `price show` without `--league`,
  which fails for a non-Standard target (call 116 against 111).
- "11 name nothing in these facts" (call 105) reads as orphan rows. They
  are the Allflame rows; Allflame reports the other 5 the same way (call
  112).

### T7 — the shop (calls 130-134)

Told the person: Standard is 1 page, 2365 characters, 31 items (call 131).
Left out: 495 omitted (489 the game already lists, 3 the game skips, 3
skipped by hand) and 5 blocked (maps in a substash, "Q3"); 20780 have no
price. Warning: all 31 were last seen 32 days ago, past the policy's 300 s
window; "`acq refresh --plan` would send 9 requests". Allflame is 1 page,
2207 characters, 27 items; 40 omitted, 1 blocked (socketed) (call 132).
Next, in the tool's words: paste the page into the shop thread and preview
before posting; `--page N` prints one page alone (call 133, which at
`--size 1000` became page 1 of 3).

Right? The counts add up (31 + 495 + 5 + 20780 = 21311).

Oddities: the Allflame page has a spoiler with an empty title,
`[spoiler=""]`, holding one item (call 132), unexplained. "Q3" is not
explained. The summary lines and the page share stdout unless `--page` is
used.

### T8 — for a program (calls 135-151)

Five re-runs, each one JSON document on stdout, exit 0: `characters`
(135), `refresh --plan` (136), `store status` (137), `search` (138),
`price status` (139). Job verbs change shape under `--json`: text mode
prints the bare payload under progress lines, `--json` wraps it in
`{"outcome","payload"}` (calls 19, 135).

| Failure | Text | JSON | Says what to do next? |
| --- | --- | --- | --- |
| wrong account | exit 1, stderr (140) | `{"error"}`, no `kind` (141) | yes: "see `acq accounts`" |
| unknown item id | exit 1 (142) | `{"error","kind":"item_not_found"}` (143) | yes: ids are given whole |
| unknown job | "no job 9999" (144) | `kind":"unknown_job"` (145) | no |
| malformed query | (146) | `kind`, `span:[6,7]` (147) | partly: what was expected |
| malformed policy | (148) | `{"error"}`, no `kind` (149) | yes: lists the legal fields |
| missing realm | (86) | `kind`, `readings`, `offers` (151) | yes, the best of them |
| usage error under `--json` | — | exit 2, text on stderr, empty stdout (150) | not JSON at all |

### T9 — when things go wrong (calls 153-172)

`daemon stop` → "daemon stopped (pid 85423)" (call 157). Then:
- Store-only: `tabs` and `refresh --plan` work (calls 159, 164).
- Observers: `daemon status` → "not running; on disk: 0 waiting, 0
  recorded running, written 16s ago" (call 158); `jobs` → "nothing waits
  on disk" (call 160).
- `status 47`, `result 47`, `auth status` → exit 1, "daemon is not running
  (it spawns on demand for job commands)" (calls 161-163).
- Needs the API: `leagues` spawned a daemon (pid 89365, call 168) and
  failed with "not logged in — run `acq auth`" (calls 166, 167). The
  session did not survive the stop; this world disables the keyring. I did
  not log in again.
- With the new daemon up, `result 47` answered again (call 169).

Version: `acq 1.0.0-beta.1 (contract bd361a2f41bf)` and the sibling `acqd`
path (calls 13, 154). The running daemon's version, pid, contract and
sha256 are in `daemon status` (call 153), which also names the log,
`runs/seat-2026-10-13/logs/bb379c5c2d9c/mock/daemon.log`, and the send
journal `sends.jsonl` beside it. Daemon left stopped (calls 171, 172).

Right? Yes.

Stops: with no daemon running nothing prints the log path; I had it only
because I asked before stopping. `auth status` said "keyring: unavailable:
disabled by ACQ_NO_KEYRING" before the stop (call 16) and "keyring: ok"
after the restart (call 170).

## 3. Faults

Worst first.

1. **Account defaulting differs by command family.** Job commands took the
   logged-in account silently (calls 17-22); offline commands refused with
   "several accounts are known" (calls 23, 25). The top-level help says
   "the sole known/logged-in account", which reads either way. Person and
   agent.
2. **Price verbs are league-scoped and inconsistent about it.** `show`,
   `status` and `list` need `--league` for a non-Standard target (call
   111); `set` and `clear` refuse the flag (call 114); the `next:` hint
   omits it (call 116). Person and agent.
3. **`--json` is not total.** A usage error is exit 2 with clap text on
   stderr and nothing on stdout (call 150), against the tour's "a failure
   is `{"error":…}` on stdout with exit 1". Some errors carry no `kind`
   (calls 141, 149). Agent or program.
4. **One apply does not sync "all".** Substashes are planned only after
   their parent is fetched, and the first plan and apply say nothing of
   the second round (calls 31-37). Person and agent.
5. **`result`, `status <id>` and `auth status` need a running daemon and
   do not start one** (calls 10, 161-163), against "local session" and
   "answered across daemon restarts". Person and agent.
6. **`search --describe` is 117,523 bytes with no index** (call 92). The
   list of names appears only in an error (call 95). Mostly an agent.
7. **"rows … name nothing in these facts"** counts rows of another league
   as if orphaned (calls 105, 108, 112). Person and agent.
8. **No way to wait for a job, no filter on `jobs`, no queue-drain time**
   (calls 64, 73, 76). Mostly an agent.
9. **The policy schema is nowhere in the help** (calls 24, 26, 28). Person
   and agent.
10. **The keyring line disagrees with itself across a restart** (calls 16,
    170). Person and agent.
11. **Empty-titled spoiler in the Allflame shop page** (call 132). Person.
12. **`result` of a cancelled job exits 0** (call 71), while a waiting one
    exits 1 (call 73). Program.
13. **Text-mode job verbs print progress lines and JSON on the same
    stream** (call 17). Agent or program.

## 4. What the tour and the help did not say

| Fact | Where I expected it | Learned at |
| --- | --- | --- |
| With two accounts known, offline verbs need `--account`; job verbs use the logged-in one | tour's `--account` line | call 23 |
| The policy's fields (`tabs`, `characters`, `max_age_seconds`, all required) | `policy set --help` | calls 26, 28, 148 |
| Substashes are planned only after the parent tab is fetched; apply until "nothing to do" | `refresh --help`, tour | call 37 |
| `search` needs `--realm` on a multi-realm store, and `all` exists | tour's search line (shows `[--realm R]` as optional) | call 86 |
| `--describe` is about 117 KB; name entries, and a wrong name lists them all | tour's `--describe [name,…]` line | calls 92, 95 |
| Life and resistances are `pseudo.total_life` and `pseudo.count_res`; class values are plural (`Rings`) | `search --help` | calls 95, 96 |
| `price` and `shop` are per league, default Standard; `set` and `clear` take no `--league` | tour's price lines | calls 109, 111, 114 |
| Price targets are `item/<id>`, `tab/<realm>/<id>`, `substash/…`, `character/<id>`; types are `exact`, `negotiable`, `no_price`, `skip` | tour (`<target> <type>`) | call 104 |
| `intent` in the search basis moves with every price write | `price --help` | calls 88, 138 |
| Revisions keep counting after a clear | `price set --help` | call 126 |
| `sleep` jobs do not queue; `fetch` jobs do | `submit --help` | calls 49, 64 |
| `auth status`, `status <id>` and `result <id>` need a running daemon | tour | calls 10, 161, 162 |
| Stopping the daemon drops a session the keyring did not hold | `daemon stop` help | call 166 |
| The log and journal paths are printed only by `daemon status` while a daemon runs | tour's daemon line | calls 153, 158 |
| `dash --json` gives a one-shot snapshot | tour (`acq dash   # live TUI`) | call 81 |
| A usage error under `--json` is exit 2, not JSON | tour's `--json` paragraph | call 150 |
| Job verbs under `--json` wrap the payload in `{"outcome","payload"}` | tour | call 135 |
| `store status` reports only the selected account's store | `store --help` | calls 12, 35 |

## 5. The tool's own words

**Right and useful:**
- The `next:` lines (calls 29, 31, 116, 121, 131).
- The undo command printed by `price clear` (call 121).
- The empty `policy show` printing a full example (call 26).
- The search answer's scope, basis, per-term counts and routes (calls 97,
  99).
- The realm error with its readings and offers (calls 86, 151).
- "an id is given whole, as an answer printed it" (call 142).

**Wrong or misleading:**
- Tour: "`acq auth status` … local session" (it needs the daemon).
- Tour: "`--json` … is total".
- `result` help: "answered across daemon restarts".
- The `price show` hint without `--league`.
- "name nothing in these facts".

**Read and never used:**
- The last three paragraphs of the tour (provider replacement, C84, MCP,
  `LIVE-TESTING.md`, `tools/tracer-rung.sh`).
- The `jq` recipes in `refresh --help`, which the door forbids.
- Most of the `daemon status` and `version` help, each one very long
  sentence.

**Not understood:**
- "Q3" (call 131), "T18" (call 105), "T23" (call 130), "N16" and "N33"
  (call 32).
- "the index cannot see" (call 105); "intent", "facts v8" and "derivation
  11" in the basis line.
- "probe" until I saw one in `jobs`; "tripwire" and "rails"; "the
  post-violation rule".

**The `C` numbers:** they never helped, since I could not look one up, and
the long help sentences are harder to read for them (calls 1, 5). Where
the sentence itself states the rule (C10 "observes only", C96 in call 86)
the number is harmless. Where the number stands in for the rule (C81 in
call 128, C74 and C72 in call 130) it leaves a gap.

## 6. What I would ask of the tool

- One account-selection rule for every verb, or name the chosen account on
  job output (call 23).
- `--league` accepted on `price set` and `clear`, or league taken from the
  target; `next:` hints that run as printed (calls 111, 114, 116).
- Usage errors as `{"error","kind":"usage"}` under `--json`; `kind` on
  every error (calls 141, 149, 150).
- A plan line saying "substashes of N tabs will be planned after this
  apply", or an apply-until-fresh option (call 37).
- `acq wait <id>`, or `result --wait` (call 73).
- `jobs` hiding finished rows by default, with a queue-drain estimate
  (call 68).
- `search --describe` printing an index by default (call 92).
- `result` and `status <id>` answering from disk with no daemon, as `jobs`
  does (call 162).
- `daemon status` printing the log path when no daemon runs (call 158).
- A duration in the plan's quote when work is not "ready" (call 31).
- `tabs --summary`, the per-league totals without the tree (call 91).
- The policy schema in `policy set --help` (call 24).
- `price status` separating "rows for another league" from "rows naming
  nothing" (call 108).
- A title for the empty spoiler, or a reason for it (call 132).

## 7. Left out

**Not tried:** `store import`, `store rebuild`, `daemon reset-tripwire`,
`auth check`, `auth logout`, `refresh --tabs`/`--all`/`--deep`,
`refresh --apply=FILE` and `--max-requests`, `policy set --if-revision`
and `@FILE`, `jobs --watch`, `demo`, `stash --sub`/`--deep`, search
`--next`, `--request`, `--print-request`, `--explain`, `--sum`, `--sort`,
`--membership`, `show --body`, `store refused <id>`, `store events
--expand`, `price list --in`/`--covered-by`/`--effective`, `shop render
--template`/`--expand`, `reference currency`, `tabs` for Standard, the MCP
server, a re-login after the session was lost.

**Files opened besides the brief and the tour:** the heading lines of
`README.md` (a `grep '^## '` to find the tour's line range), and one file
outside the repository where the harness spilled the oversized output of
calls 152-165 (`…/tool-results/bejzvoys7.txt`); it held only what the door
printed.

**Work files written:** none under `runs/seat-2026-10-13/work/`; no verb
needed its own output back as a file. This report itself was not written
to disk by the seat: the harness refused the write, and the text was
handed back to the caller.

**The one cut I would most want put back:** `refresh --apply=FILE` with
`--max-requests` — the reviewed-plan path an agent should use before
spending a person's rate limit, and the one place the door's no-redirect
rule would have bitten.
