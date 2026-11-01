<!-- written during the run; call numbers are rows of calls.csv -->

## Before touching anything

**What the brief told me (the brief).** The tool is Acquisition's MCP server `acq-mcp`; the world points at a mock API; two accounts: `GERWARIC#7694` (a copy of the person's real collection, ~50 MB, no session) and `seat` (logged in by the person; a daemon runs for it, set to exit four hours after its last connection); no keyring in this world.

**What the README told me (before any call).**
- Can do: download stash tabs and characters to a local store; search offline (own query language, plus trade-site URLs translated with no network); price items/tabs/characters by hand with undo; render forum shop pages to paste; keep a league fresh via a sync policy -> plan -> apply. It does not value items, post to the forum, or read the trade site.
- Needs: store/search/pricing/shop tools need no daemon and no network. Fetching tools (`submit_job`, `apply_plan`, `cancel_job`) need a running daemon, and against GGG `acq-mcp` never starts one. A daemon needs a login, which is "a person's act at a terminal" (`acq auth`).
- Before an agent can use it: unpack, keep the three binaries together, `acq auth` at a desktop terminal, add the `mcpServers` JSON to the host, with `env` `ACQ_PROVIDER=mock` for the stand-in and `ACQ_STORE_DIR` matching `acq`'s.
- New league: `acq leagues` names it; write the realm back with *every* league it keeps (naming only the new one drops Standard); then `refresh --plan/--apply --league <name>` until the plan says nothing to do; first plan is listings, later plans fetch every tab; read each plan's work and ceiling first.
- What I would not call without asking: `set_sync_policy` with `replace: true` or over an existing policy; `apply_plan` / `submit_job` against the real API on a large plan (rate limit: "a first sync of a few hundred tabs takes most of an hour"); `cancel_job` on a job I did not start; `price_set`/`price_clear` on the person's real prices. (Here the brief makes all of these safe.)
- Contradiction already: README says the daemon exits "an hour after its last connection"; the brief says four hours for this one (likely `ACQ_IDLE_SHUTDOWN` was set — the knob table says default 3600).

**What `open` told me that the README had not (call 1).**
- Instructions: provider is `mock`; "Begin with search and no arguments"; `describe` explains the language and answers and "the whole is over 150 KB" (so ask by name); a refusal is `isError` with `error`, `kind`, `readings`, `next`, `offers`, and any offered request is "sent back as printed".
- `list_jobs`, `job_status`, `job_result`, `daemon_status` answer without a daemon from the on-disk ledger.
- `price_clear` *requires* `if_revision` ("a clear without it is refused naming the current revision"); `price_set` needs it when a row exists.
- `set_sync_policy` is compare-and-swap: `if_revision` "omitted, or 0, only to create the first" — the README's `policy set` mentions `--if-revision N` but never says omitting it is create-only.
- `apply_plan` takes the plan envelope verbatim and has `max_requests`; plans are refused if the policy revision moved.
- `submit_job` kinds: profile, characters, character, leagues, stashes, stash, apply, sleep, fetch, whoami.
- Tools all take `account` "needed only when several are known" — and two are known here (call 2).

**What the README had that `open` did not.** The new-league procedure and the warning that a realm is written whole; the rate limit (30 per 5 minutes on tabs); store locations; how to start a daemon (`acq auth check`, `acq auth`). The server's instructions do not mention leagues, the rate limit, or what to tell a person when no daemon runs. README points at `CLI-REFERENCE.md`, `AGENTS.md` and `decisions/*.md` — pointers I cannot follow here.

**Whose items are here (call 2).** `accounts` (provider `mock`) lists 2: `GERWARIC#7694` (last login "16d ago", `persisted:false`, store `W/store/mock/GERWARIC_7694.db`) and `seat` (last login "27s ago", `persisted:false`, store `W/store/mock/seat.db`). Both are under `store/mock`, i.e. the stand-in's store, as the README's data section says for mock. Neither login is in a keyring.

## The tasks

### T1 — the host

**To the person:** "I've set myself up with `W/work/acq.json`: one server, `acquisition`, command `W/bin/acq-mcp`, with `env` `ACQ_PROVIDER=mock` and `ACQ_STORE_DIR=W`. Through it I see the same two accounts and the daemon you started."

- Calls: 3 (open through the config), 4 (`accounts`), 5 (`daemon_status`). First call held right: 3.
- Right? I hold it right: call 3 printed the same 36104 bytes of instructions and tools as call 1; call 4 lists the same two store files as call 2; call 5 says `running:true`, `compatible:true`, `provider_matches`, `world_matches:true`, `world` = `W/store`, `pid` 2618. What would tell me it is wrong: `world_matches:false`, or `accounts` with a third/empty store.
- What the README told me: the `mcpServers` shape and `command`; `env` `ACQ_PROVIDER: mock` for the stand-in; "`ACQ_STORE_DIR` there must match your `acq`'s".
- What it did not: what value `ACQ_STORE_DIR` takes. The knob table says its default is "the platform data dir", and the data section says the store is "one database per account in `store/ggg` (`store/mock` …)" under it, so I set it to `W` (the parent of `W/store`). Call 5 then reports the daemon's `world` as `W/store` — a different directory from the one I set — and still `world_matches:true`. So the README's "store dir" and the daemon's "world" are not the same path, and nothing says so. Had I set `W/store` (the path `accounts` shows the dbs under), I do not know whether it would have looked for `W/store/store/mock`. I did not try it. The README also does not say whether the host config should carry `ACQ_NO_KEYRING` or `ACQ_IDLE_SHUTDOWN`; call 5 shows the running daemon has `keyring: "unavailable: disabled by ACQ_NO_KEYRING"` and `idle_shutdown_seconds: 14400`, set by whoever started it, not by the server.
- Where the README misled: it says the daemon exits "an hour after its last connection"; call 5 says 14400 s (four hours), matching the brief. A knob explains it, but the person's "Your agent" section states one hour as fact.

### T2 — keep the new league fresh

**To the person:** "On `seat` you have three leagues: Standard, Hardcore and **Rehearsal**. Rehearsal is the new one (the mock describes it as 'The mock's challenge league, empty on its first morning') (call 8). You had no sync policy (call 6), so I created one with the request that can only create: `if_revision: 0`. It now holds `pc` -> `Rehearsal`: all tabs, all characters, `max_age_seconds: 3600`, revision 1 (calls 9, 10). It covers only Rehearsal. Standard isn't in it because you never had a policy, but if you want Standard kept fresh too, I have to write the `pc` realm again naming both leagues. I applied two plans. The first listed your tabs and characters (2 requests). The second fetched all 7 Rehearsal tabs (7 requests), and the folder was skipped. Everything landed: 8 tabs, 7 fetched, 0 items, because the league is empty. You have no characters in Rehearsal yet (both of yours, StashHoarder and MuleQuadTab, are in Standard). A third plan says there is nothing left to do."

- Calls: 6 (`sync_policy` -> `{"policy":null}`), 7 (`submit_job` leagues -> job 2), 8 (`job_result` 2), 9 (`set_sync_policy`, create-only), 10 (read back), 11 (plan 1), 12 (`apply_plan` with `max_requests:2` -> job 3), 13 (`job_status` 3: `done`), 14 (`job_result` 3: children 4,5 done), 15 (`tabs` Rehearsal: 8 tabs, 7 never fetched), 16 (`characters`: 2, both Standard), 17 (plan 2), 18 (`apply_plan` with `max_requests:7` -> job 8), 19, 20 (job 8 done, children 9–15, 0 failed), 21 (`tabs`: 7 fetched, `fetched_items:0` each), 22 (`job_result` 14, the Maps tab: `items:[]`, `substashes_listed:0`, rate-limit headers `15:10:60,30:300:300`), 23 (plan 3: `actions:[]`, `logical_requests:0`, every tab `fresh` age 12 s).
- First call held right: 8 (the league list). The policy write (9) is right too.
- Right? Yes for this mock. What would tell me it was wrong: a plan 3 that still had actions, or `tabs` showing `fetched_items:null`.
- **What each step told me.** Plan 1 (call 11): `list_stashes` + `list_characters`, both `never_listed`; `logical_requests: 2`; `wire_sends` min 2 / max 6, with a HEAD probe and a token refresh listed as prerequisites; `quote.scopes[].eta_seconds: null` with "policy not yet learned". Plan 2 (call 17): 7 `fetch_tab` actions, skipped the folder `rh-fold`, `logical_requests: 7`, `wire_sends` 7–21, `eta_seconds: null`; `notes.no_characters_listed: true`. **How long: neither plan gave a time.** Both ETAs were null, because the limiter had not yet learned the route. The README's "thirty every five minutes" is the only duration I had. Job 14's record afterwards showed the real stash limit (`30:300:300`).
- **What the job records let me read.** A parent `apply` gives `child_jobs`, `children` counts and `failed_ids` (calls 14, 20). A child stash job gives the stash body (empty), substash count and the rate-limit headers (call 22). The parent's `job_status` (calls 13, 19) was already `done` on the first poll, so I never saw it waiting.
- Where the tool or README stopped or misled me: (1) `sync_policy` with no policy answers `{"policy":null}` (call 6) with no revision and no hint that `if_revision` 0 is the create-only form. The tool text for `set_sync_policy` says that, but the README's `--if-revision N` line does not. (2) `daemon_status` (call 5) said `policies_known: 1` before I wrote any policy, while `seat`'s was null (call 6). I take it to be GERWARIC's, but nothing says whose. (3) The README's "the first plan is the listings, one or two … and the plans after it fetch every tab" matched exactly. (4) The README's warning "a policy naming only the new league drops Standard" made me stop and think. With no prior policy, the receipt `prior: null` (call 9) confirms that nothing was dropped.

### T3 — what I have

**To the person:** "You have **22,721 live items** (plus 6 removed) in a 54 MB store (call 24). By realm and league (call 27): PoE1 Standard 21,311, PoE1 Allflame 1,312, PoE2 Standard 98. Your PoE1 Standard stash has 3,347 tabs (16 of them folders) holding 20,231 items. Only 405 of those tabs have ever been fetched; 2,926 never were, almost all map tabs (2,550, of which 61 were fetched) and unique tabs (502, 65 fetched) (call 28). You have 65 characters: 50 fetched, 10 of those with empty bodies, and 15 never fetched, holding 1,389 items (call 29). **Everything is old.** The oldest fetch was 59 days ago, the character `gerwarshot` in Standard. The newest was 50 days ago, the stash tab `Dump Tab` in Standard. The tab and character list was last seen 59 days ago (call 27). Your league list was never fetched (call 24). The store also says 4 item bodies were refused. I can't list them from here: the tool says `acq store refused` 'has no tool here: a person runs it' (call 30). This account has no session (the brief), so I can't refresh any of it."

- Calls: 24 (`store_status`), 25 (`search {}`: refused, `realm_needed`), 26 (counts by `realm`: refused, `unknown_name`), 27 (counts by league over `realm: all`), 28 (`tabs`), 29 (`characters`), 30 (`describe ["store"]`). First call held right: 24 (counts); 27 for the ages and their sources.
- Right? Yes. The counts agree with each other: `fetched` 551 + `never_fetched` 2941 + `folders` 17 = 3509 = 3444 tabs + 65 characters (calls 24, 27). Per league: 21,311 + 1,312 + 98 = 22,721. What would tell me it is wrong: a `tabs` sum per league that disagrees with call 27. I did not list the other leagues' tabs.
- Where the tool stopped me: following the instructions, `search {}` was refused (call 25) for two realms. The error text says "pc | poe2, or all", but its `offers` hold only `pc` and `poe2`, with no `all`. Counting by `realm` was refused (call 26) because `realm` is no query key. That refusal had no `offers` or `next`, just the key list. On a character row, `item_count` (69) exceeds `fetched_items` (37) (call 29). `describe ["store"]` explains it: socketed gems are not counted in `fetched_items` (call 30). The 4 refused bodies cannot be read over MCP (call 30).

### T4 — find things

**To the person:**
- (a) "Yes. **187** of your PoE1 items have six linked sockets (`links>=6`, call 32): 181 in Standard and 6 in Allflame. 161 are in stash tabs, 24 are worn by characters and 2 sit in character inventories (call 33). For example: Shavronne's Wrappings on gerwarwitch, Tabula Rasa on BatteryStaple, and a white-socketed Vaal Regalia on VVickedWitch (call 32). This is as of fetches 50–54 days ago, and 2,926 of your map and unique tabs were never fetched (call 28), so there may be more."
- (b) "**195** rare rings have a `+# to maximum Life` line and at least two resistances: 188 in Standard and 7 in Allflame, 172 of them in stash and 23 on characters (call 37). A looser reading using the trade site's pseudo 'total life' gives 224 (call 36), but that one also counts Strength. For example, Viper Band qualifies on '+58 to Strength' alone. I'd go with 195."
- (c) "PoE1 Standard has 21,311 items: 7,693 with no rarity (currency, cards, fragments and the like), 5,615 rare, 2,850 normal, 2,819 unique and 2,334 magic. PoE1 Allflame has 1,312: 561 with no rarity, 276 normal, 257 unique, 154 rare and 64 magic (call 38). PoE2 Standard has 98: 43 with no rarity, 25 magic, 16 rare, 13 normal and 1 unique (call 39). Other leagues you have characters in hold no items."
- (d) "Damnation Circle, a Ruby Ring, ilvl 78, needs level 60, 11% resistance-modifier quality. It's worn in ring slot 2 by TheFireworksCompany (Standard). Lines: +78 maximum Life, +59 maximum Mana, +31% Cold, +33% Chaos, 0.27% physical attack damage leeched as mana, crafted +1 minimum Endurance Charges, implicit +33% Fire. It isn't priced ('nothing applies'). First seen 1788362161, last seen 1788884802, i.e. fetched 53 days ago (call 40)."

- Calls: 31, 34, 35 (`describe` operators/linked/totals/line; `total_res`; `count_res`), 32, 33 (a), 36, 37 (b), 38, 39 (c), 40 (d). First held right: 32 for (a), 37 for (b), 38+39 for (c), 40 for (d).
- **Ages.** Every search answer carries a `scope` with `oldest_fetch`/`newest_fetch`, their ages in words and *where* they came from, `list_seen_age`, `never_fetched` and `never_fetched_by_type`. Each row carries its own `fetched_at_age`. For `realm: pc` (calls 32–37) the oldest is 54 days (character TheAbsenceOfPatience, Allflame) and the newest 50 days (stash 'Dump Tab'). For `realm: all` (call 38) the oldest is 59 days (gerwarshot, PoE2). The age words are made per answer (call 30, "ages").
- Right? (a) and (c) add up (call 38's cells sum to call 27's per-league totals). For (b), the trap is in reading "life": call 36's `pseudo.total_life` includes Strength (visible in its `matched` rows), which the tool did *show*. So I hold 195 right for "a life line", and would want the person to confirm that is what they meant.
- Where the tool stopped or misled me: the query language's index is not in the tool texts. `describe` with no names is "over 150 KB", above this door's 60,000-byte cut, and the `computed` section alone is "some 80 KB". So I found `pseudo.count_res` **by guessing a name** (call 35). Had the guess failed, the refusal would list every name, per `describe`'s text. `describe ["totals"]` returned only a provenance sentence (call 31), naming a CSV and a C++ file I cannot read.

### T5 — prices

**To the person:** "In Standard you have no prices set by hand. All 492 of your priced items are priced by the game itself: 487 exact, 2 negotiable and 3 `~skip`, from notes in 17 priced tabs, 3 of them public (call 41). Two kinds of problem:
1. **One note the tool can't read:** a Thaumaturge's Incubator (stack 3) in the public tab `ACQUISITION-PRICE-TEST`, cell x9 y5. Its note is `~price  chaos`, and the tool says 'the amount is empty — what the dialog leaves after an invalid entry' (call 42). Fix the note in game.
2. **1,395 items with 'residue':** price text the trade site can't see because it sits in a private tab (call 43). 160 of them are under your private unique tab named `~price 4444 chaos (Remove-only)`, e.g. Fairgraves' Tricorne, Honourhome, Devoto's Devotion, Goldrim, Abyssus (calls 43, 44). Make that tab public, or price those items another way. I could not group the other 1,235 by tab: search's `price.amount` sees only effective prices (call 46), and `price_list` has no per-container count.

Then I made the writes you asked for:
- **Tab `ACQUISITION-PRICE-TEST` at 5 chaos** (call 49: revision 1, `prior: null`). It applies to 31 of the tab's 81 items. The other 50 carry their own `~price 999 …` notes, and 'the note is more specific than the row on the tab', so the game note wins there (calls 47, 54, 56).
- **Goldrim (a Leather Cap in that private unique tab) at 2 divine** (call 51: revision 1), then **3 divine**, replacing revision 1 as read in call 52 (call 53: revision 2, `prior` = 2 divine rev 1).
- Read back: the tab row is 5 chaos rev 1, and the items inherit it with `inherited: true` (call 54). Goldrim is `manual` 3 divine rev 2, effective 3 divine (call 55)."

- Calls: 41–56 (41 status; 42–44 problems; 45, 46 a failed grouping attempt; 47 tab read; 48 currency; 49 tab write; 50–53 item read/write/read/replace; 54–56 read-backs). First held right: 41.
- **Receipts and undo.** Call 49's `undo` is `{"tool":"price_clear","request":{"account":…,"if_revision":1,"target":"tab/pc/03cb479c65"},"says":"undoes it"}`, i.e. remove the row I created. Call 51's is the same for the item. Call 53's `undo` is `{"request":{… "amount":"2","currency":"divine","if_revision":2 …},"says":"puts the prior back"}` with **no `tool` field**; per the server's instructions ("to the tool it names or the same one") that means `price_set` again. Undoing call 53 puts back 2 divine, not "no price"; to get back to nothing I need call 51's clear, or a clear naming the current revision.
- **Knowing another program changed a price between read and write:** each row has a `revision` (`listing.manual.revision` in `price_show`). A write names it in `if_revision` and is compare-and-swap, so a stale revision is refused (tested in T7). `show`/`search` also carry `basis.intent`, "the revision of your prices and sync policy" (tool text of `show`).
- Right? Yes: both read-backs show what I wrote. What would tell me it was wrong: an `effective` that is not 5 chaos on the 31 items, or Goldrim not at rev 2.
- Where it misled me: "price one stash tab at 5 chaos per item" really reaches only 31 of 81 items. The `price_set` receipt (call 49) did not say so. It took two `price_list` calls (54, 56) to find the 50 conflicts. Someone pricing a whole tab would want the receipt to count what the row now decides.

### T6 — the shop

**To the person:** "Your Standard shop is **one page** (2,288 characters, under the 50,000 page size), holding **31 items**, all from `ACQUISITION-PRICE-TEST`. That's `Stash57` on the forum: the tab's index is 56, ranked without folders and substashes. All 31 sit under one spoiler `~price 5 chaos` (call 57). One of them is the Thaumaturge's Incubator at x9 y5 whose in-game note is broken (T5). Because its note decides nothing, the 5-chaos tab price covers it on the forum page.
**Goldrim at 3 divine is not on the page.** It was blocked (`substash`): 'the website's stash view offers no link for an item in a map or unique substash … blocked until one is' (call 57).
Left out: 489 items the game already lists at its own price ('a page must not contradict what the site shows') and 3 `~skip` items, both omitted; 1 blocked (Goldrim); 20,787 off the page because nothing prices them. Freshness: `policy: not_set`. You have no sync policy on this account, so the tool can't say whether the posted items are current, and these facts are 50 days old.
**What you do next:** copy `pages[0].text` and paste it as a post in your league's shop thread on pathofexile.com. The tool posts nothing. Don't move the linked items in game, because a moved item may lose its link (README)."
Then I cleaned up: the tab's price is removed and Goldrim is back to no price (calls 58–63). Since nothing of mine stands, the shop above no longer reflects them.

- Calls: 57 (render), 58 (tab undo from call 49, sent exactly: landed, rev 2), 59 (item undo from call 51, sent exactly: **refused** `conflict`, "has a price at revision 2 (re-read and retry)"), 60 (`price_clear` with `if_revision: 2` from the refusal's `revision`: landed, rev 3), 61 (`price_status` identical in every count to call 41), 62, 63 (both targets: `relation: none`, `effective: none`).
- First held right: 57.
- **The undo was not fully the receipt's.** The tab went back with the exact request (call 58). The item could not: the only receipt request that *removes* its price is call 51's, which names revision 1, and after the replacement (call 53) the row is at 2. Call 53's receipt undoes to "2 divine", not to nothing. So I did what the refusal said ("re-read and retry") and used its `revision: 2` (call 60). That clear is a request I built from the refusal, not one a receipt printed. I hold the end state right (calls 61–63). What would tell me otherwise: a `rows.total` other than 0 in call 61.
- Where the tool stopped me: (1) Nothing at write time (call 51) warned that an item in a unique substash cannot go on the shop; I learned it only at render (call 57). (2) The clear receipts (58, 60) say the row's "count continues past the clear": a re-create starts at a later revision. That is fine but surprising.

### T7 — for a program

**To the person:** "Two of the three refusals are single JSON documents a program can act on. The failed job is a record with a sentence in it, not something a program can branch on."

1. **Wrong account** (`tabs` with `GERWARICK#7694`, call 64): `isError`; one JSON document with `error`, `kind: "not_logged_in"`, `accounts: [GERWARIC#7694, seat]` and `offers`, each a ready `{"request":{"account":…}}` saying "the same request as …". A program can recover by resending an offer. **But `kind` is misleading:** the problem is an unknown name, not a missing login. A program that maps `not_logged_in` to "ask the person to run `acq auth`" would do the wrong thing. The account `GERWARIC#7694` also has no login, yet reads fine (T3).
2. **Job for a nonexistent tab** (`submit_job` stash `rh-nope` on `seat`, call 65): accepted with `{"job_id":17}`. No check against the tab list the store holds (call 15 lists 8 Rehearsal tabs, none `rh-nope`). `job_status` (call 66) says `state: "failed"`, `retries: 0`. `job_result` (call 67) is *not* `isError`: `outcome: "failure"`, `error: "GET /stash/Rehearsal/rh-nope returned 404 Not Found; no rate headers; a body of 25 bytes"`. It is one document, but the failure is a free-text sentence with no `kind`, no `next`, no `offers`, and the 25-byte body is not shown. A program cannot tell "the tab does not exist" from any other 404 without parsing English. Nothing tells it what to do next (e.g. re-list stashes with `refresh_plan`). It did cost one real request against the API's rate limit.
3. **Price write naming a revision that is not the row's**: (a) `price_set` on the tab with `if_revision: 7` after its row was cleared (call 68): `isError`, `{"error":"tab/pc/03cb479c65's price was cleared (revision 2); a new price names no revision","kind":"conflict","revision":2}`. It is warranted, it is one document, and the `error` text says what to do: name no revision. But the machine-readable `revision: 2` invites a program to resend `if_revision: 2`, which the text says is wrong. There are no `offers`, though the tool knows the exact create request. Call 69 confirms nothing landed. (b) In T6, call 59 (`price_clear` naming 1 when the row was at 2): `kind: "conflict"`, `current` (the whole row), `revision: 2`, "re-read and retry". It is warranted, a program can retry with `revision`, and that worked (call 60). No `offers`.

- Calls: 64–69 (and 59 from T6). First held right: 64.

### T8 — the daemon

**To the person:** "I've been talking to `acqd` (pid 2618, version 1.0.0-alpha.1, provider `mock`), the copy at `W/bin/acqd`, over a socket in your temp folder. It serves the store at `W/store`. It's been up 419 s, has made 19 sends, has no jobs running or waiting, and has learned 5 rate-limit policies. It runs with no keyring ('disabled by ACQ_NO_KEYRING'), so your `seat` login lives only in it (call 70). It exits after `idle_shutdown_seconds: 14400`, i.e. **four hours** after its last connection or job submitted, once nothing is sending or waiting. It doesn't say *when* that will be. Its log is `W/logs/8787a97f5ae1/mock/daemon.log`. When it's gone, the `seat` login goes with it, because there's no keyring. In a terminal on this machine, type:
```
ACQ_PROVIDER=mock ACQ_STORE_DIR=<W> ACQ_NO_KEYRING=1 acq auth
```
to log in again and start a new daemon (the README's line for 'without a keyring'; with a keyring it would be `acq auth check`). Then `acq daemon status` to see it. Against the real game it's the same without `ACQ_PROVIDER=mock`."

- **Tools that need a daemon** (server instructions, call 1): `submit_job`, `apply_plan`, `cancel_job`. `refresh_plan` adds a quote if one runs. `list_jobs`, `job_status`, `job_result` and `daemon_status` answer from the ledger without one. `describe ["daemon"]` (call 71) adds: with none running, `job_status` carries `next`, "the command a person runs to start a daemon". I did not see that, because the daemon never went away. It also says "in mock mode it starts one into an empty socket", which neither the README nor the instructions say.
- Calls: 70, 71 (plus 5). First held right: 70.
- Right? The daemon facts are the tool's own (call 70). The command is the README's ("Your agent" and "Log in"). I put the env vars together myself from the knob table and call 70's keyring line: no text tells the person to repeat `ACQ_STORE_DIR` / `ACQ_NO_KEYRING` / `ACQ_PROVIDER=mock` on `acq auth`. If they type plain `acq auth`, I expect it would log in against GGG in the default store, which is the wrong world. I could not test that here.
- Where it misled me: the README's "exits an hour after its last connection" vs call 70's 14400 s. Also, in T2 I read `policies_known: 1` (call 5) as sync policies. Call 71 says it is "rate-limit policies learned", and call 70 shows 5 after my fetches. **My T2 remark (2) about it is wrong. The tool was not.** The bare key name misled me, but the description explains it.

## Faults

Worst first.

1. **A failed job is not a document a program can act on** (calls 65–67). `submit_job` stash for a tab the store's own listing lacks was accepted and spent a request; `job_result` gave `outcome: failure` with an English `error` sentence, no `kind`, no `next`, no `offers`, and the 404 body withheld. *Should:* check the id against the stored listing at submit, or at least give the failure a `kind` (`not_found`) and a `next` (re-list via `refresh_plan`). Refusal analysis: not a refusal; the failure was warranted (my request was wrong), and it did not say enough to recover. Hits a person (via `acq stash`) too, and an agent hardest.
2. **No receipt can undo a replace back to nothing** (calls 51, 53, 59, 60). After create (rev 1) then replace (rev 2), the create's undo names rev 1 and is refused. The replace's undo restores 2 divine. Getting back to "as I found it" needs a hand-built clear using the revision a refusal names. *Should:* a replace's receipt could also carry the "back to the original state" request, or the conflict refusal could offer the clear at the current revision. Refusal (call 59): warranted, because the receipt was stale. It said enough to retry by hand, but offered nothing. Both a person and an agent hit it.
3. **`kind: not_logged_in` for a misspelled account** (call 64). The kind says the wrong thing (the name is unknown; and accounts with no login read fine, T3), though `offers` recover. Warranted refusal; enough to recover through offers; misleading to a program that branches on kind. Agent mainly; a person reads the sentence.
4. **The `realm_needed` refusal names `all` but does not offer it** (call 25). The instructions say to begin with `search` and no arguments, which is refused on this store. Warranted given its rule; it recovered via offers, but for one realm only. Agent only.
5. **Pricing a tab gives no count of what the row decides** (call 49 vs 54/56). The receipt says `in_facts: true`, but not that 50 of 81 items keep their game price. Both.
6. **Shop exclusion is found only at render** (call 51 vs 57). A substash item can be priced but never posted; the `price_set` receipt doesn't say so. Both.
7. **`price_set` conflict's `revision` field contradicts its text** (call 68). The text says name no revision; the field `revision: 2` reads as the one to send. Program only.
8. **Unknown query key refusal has no offers** (call 26: `realm` as a count key). It lists keys, but does not offer `realm` in `scope`. Agent only.

## What the README and the tool texts did not say

- **What `ACQ_STORE_DIR` is set to**: the parent of `store/`, i.e. the platform data dir, while the daemon reports `world` as `<dir>/store` (call 5). Expected in README "Your agent" or the Knobs row. Learned at call 5, by inference only.
- **That the host config needs the same env as the daemon's starter** (`ACQ_NO_KEYRING`, `ACQ_IDLE_SHUTDOWN`) or not. README "Your agent". Never learned. It worked without them (calls 4, 5).
- **The idle exit actually in force**: README says an hour, call 5 says 14400 s. README "Your agent" should say "an hour by default (`ACQ_IDLE_SHUTDOWN`), `daemon_status` says". Learned at call 5.
- **Create-only form of `set_sync_policy`**: `if_revision` 0/omitted. Present in the tool's own text (call 1), absent from the README's `policy set` line. Needed at call 9.
- **That a plan gives no duration on a fresh route** (`eta_seconds: null`, calls 11, 17). README "Before the first command" gives 30 per 5 minutes; no text says a first plan cannot quote. Learned at call 11.
- **The query language's names**: `pseudo.count_res` and `pseudo.total_life` (and that the latter includes Strength). Expected in `search`'s text or a `describe` index small enough to read; `describe` is >150 KB and the door cuts at 60,000 bytes. Learned by guessing at calls 35, 36.
- **Refused bodies are not readable via MCP**. Not in the instructions or `store_status`'s text; said only inside `describe ["store"]` (call 30).
- **Substash items cannot be posted**. Expected in `price_set` or `shop_render` text; learned at call 57.
- **`policies_known` means rate-limit policies**. Learned at call 71 (I misread it at call 5).
- **What `acq auth` needs for the agent's world**: the env vars. README "Your agent" says `ACQ_STORE_DIR` "must match", but not that the re-login must carry it. Never confirmed.
- **What the README said that I never needed**: install, checksums, macOS quarantine, Windows quoting/`ACQ_SHELL`, `acq web`, `acq store import`, `diagnose`, the rails knobs, `ACQ_MOCK_DEGRADED_HEAD`, licence. Pointers I could not follow: `AGENTS.md`, `CLI-REFERENCE.md`, `decisions/<area>.md`, `SECURITY.md`, `CONTRIBUTING.md`, the source links in the Knobs table. None of them blocked me, except that `CLI-REFERENCE.md` might have held the query-language index I had to guess at.

## The tool's own words

- **Right and useful:** the instructions' rule that a refusal carries `offers` to resend as printed (call 64 worked exactly). Ages "in words beside the time" everywhere, and `oldest_fetch_location`/`newest_fetch_location` answered T3's "from where" outright (call 27). Plan `wire_sends` with its `prerequisites` and `not_covered` (calls 11, 17) is an honest ceiling. Price `why` strings ("the note is more specific than the row on the tab", call 56; "the amount is empty — what the dialog leaves after an invalid entry", call 42) are clear. `shop_render`'s policy table explains every cell with a reason (call 57). `describe ["store"]` (call 30) explains `fetched_items` vs `item_count`.
- **Wrong or misleading:** `not_logged_in` for a typo (call 64). `revision: 2` beside "names no revision" (call 68). README "an hour" vs 14400 s (call 5). `never_fetched_by_type` lists `character` with a `tabs` count (call 27), a tab word for characters. `tabs` rows lack `listed_at_age` while `characters` rows have it (calls 15, 16).
- **Missing:** a `kind` on a failed job (call 67). A `tool` field on call 53's undo (the first two receipts name it, this one does not; the instructions' "or the same one" covers it, but it is inconsistent).
- **Read and never used:** `search`'s `count`/`counted_at`/`basis` continuation fields; `trade` queries; `item_events`; `list_jobs`; `cancel_job`; `reference_currency`'s `evidence`; `shop_render`'s `template`.
- **Words I did not understand:** "rails halt", "a failed queue write parks the queue on disk, and nothing waiting then holds it" (README "Your agent"). "residue" was clear only after `price_list`'s `game_text` description. `undecided`/`lacked` in `terms` I could guess but did not look up. `derivation`, `facts_version`, `beast_colours` in `basis`.

## What you would ask of the tool

- Give a failed job's record a `kind` and a `next` (call 67).
- Refuse or warn at `submit_job` for a tab id the stored listing lacks (call 65).
- Offer the current-revision clear in a price `conflict` refusal, and give a replace's receipt the undo back to the original state (calls 59, 53).
- Use `unknown_account` rather than `not_logged_in` for a name no store has (call 64).
- Offer `realm: all` in `realm_needed` (call 25).
- Count in the `price_set` receipt how many covered items the new row decides, versus those a nearer note keeps (call 49).
- Warn at `price_set` that a substash item cannot be posted to the shop (call 51 vs 57).
- A small `describe` index of computed names (under ~10 KB) (calls 31, 35).
- Have `daemon_status` say when the idle exit would fall (`exits_at` or `idle_for_seconds`) (call 70).
- Expose `store refused` as a tool, or say so in `store_status`'s text (call 30).
- A per-container count in `price_list` for `game_text: residue` (call 43).

## Left out

- **Not tried:** `cancel_job`; `list_jobs`; `item_events`; trade-site URL queries; `shop_render` with `template`/`page`; the no-daemon path (I could not stop the daemon and would not); a host config pointing `ACQ_STORE_DIR` at `W/store`; Allflame's or other leagues' price status; PoE2 searches beyond counts; `describe` with no names (over the door's cut).
- **Files opened beyond the two:** the harness saved calls 1 and 3 (`open`, 36,104 bytes) to files in my session's tool-results folder because they exceeded its display limit, and I read call 1's file to see the whole `open` output. It is the door's own printed output, not a tool file. No other file.
- **Files written:** `W/work/acq.json` (host config); `W/drafts/apply1.json` and `W/drafts/apply2.json` (the `apply_plan` arguments, verbatim plans from calls 11 and 17, because the harness refused the call heredoc); the report.
- **The one thing I cut that I would most want back:** watching the session end. Letting the daemon go away (or stopping it) to see `job_status`'s `next` and whether `submit_job` refuses with a usable command for the person.
