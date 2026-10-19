<!-- written during the run; call numbers are rows of calls.csv -->

## Before touching anything

**What the brief told me (the brief):** acq-mcp is the MCP server of Acquisition, a PoE inventory tool; one account, `GERWARIC`, is in the store (~50 MB, a copy of a real collection with set prices); there is no login and no daemon; calls needing GGG's API will say they cannot reach it; everything points at a mock and the store under W.

**What the README told me, before any call:**
- Can do: download stash tabs and characters to the machine (through the daemon `acqd`), search them offline, price items, render forum shop pages to paste. MCP tools named: `search`, `describe`, `show`, `tabs`, `price_status`, `price_set`, `shop_render` "and four more", `refresh_plan`, `apply_plan`, `submit_job`. The full list is in `MCP-REFERENCE.md` — a pointer I cannot follow here.
- Needs: store, search, pricing and shop tools need no daemon and no network. Fetching tools need a daemon, and against GGG `acq-mcp` never starts one; it uses one the person started, which idles out after 60 s unless `ACQ_IDLE_SHUTDOWN` is set. A daemon against GGG needs a login (`acq auth`), a person's act at a desktop terminal, keyring-backed.
- Before an agent can use it: install the archive, put it on PATH, `acq auth`, add the `mcpServers` entry to `.mcp.json`, and for fetching, start a daemon with `ACQ_IDLE_SHUTDOWN=3600 acq auth check` (or `acq auth` without a keyring).
- What I would not call without asking: `apply_plan` and `submit_job` (they spend the person's GGG rate limit — "a first sync of a few hundred tabs takes most of an hour"), and any price write or clear (they change the person's shop). The README says writes come "with undo" and take `--if-revision`, which I'd use. The policy write (CLI `policy set`) replaces the whole policy across realms (Known gaps) — I would not touch it at all without the person.
- The README does not say whether the MCP server exposes `--account`, nor the default-league rule for MCP (the CLI reads Standard without `--league`).

**What `open` told me (call 1) that the README had not:**
- The full tool list — 26 tools, not the README's 11 named plus "four more": `accounts`, `apply_plan`, `cancel_job`, `characters`, `daemon_status`, `describe`, `item_events`, `job_result`, `job_status`, `list_jobs`, `price_clear`, `price_list`, `price_set`, `price_show`, `price_status`, `reference_currency`, `refresh_plan`, `search`, `set_sync_policy`, `shop_render`, `show`, `store_status`, `submit_job`, `sync_policy`, `tabs`. (I count 25 in the printed list; the README's "and four more" after `price_status, price_set, shop_render` matches `price_list`, `price_show`, `price_clear`, `reference_currency`.) Job-control tools (`job_status`, `job_result`, `list_jobs`, `cancel_job`, `daemon_status`) and the policy tools are not named in the README's agent section at all.
- The provider in this world is `mock` ("Acquisition (provider: mock)", call 1).
- How to start: "Begin with search with no arguments … and describe with names [\"fields\"]"; `describe` whole is over 100 KB.
- MCP writes are stricter than the CLI: `price_set`/`price_clear` require `if_revision` to replace/remove an existing row (the README shows `--if-revision` as optional on the CLI); `set_sync_policy` is compare-and-swap. This removes much of my worry about clobbering.
- Refusals are `isError` results with `readings` and `offers`; counts carry their own follow-up requests.
- Each pricing/store tool takes an optional `account`; several (search's `scope`, describe) are documented in CLI flag words.
- In mock mode the server's instructions imply it may spawn a daemon ("never spawns … in real-GGG mode") — the README says only "against GGG `acq-mcp` never starts one".

**What the README said that `open` did not:** where the store lives per platform; the 60 s idle exit and `ACQ_IDLE_SHUTDOWN`; that logging in is `acq auth` at a desktop (the instructions only say "Login is done by the human via the `acq` CLI"); the rate-limit scale (30 tabs / 5 min); Known gaps (PoE2 partial, empty-body characters, policy-set replaces whole policy — though the MCP tool guards it with a revision).

**Whose items, from where (calls 2, 3):** one account, `GERWARIC#7694` (uuid `cac319d8-…`), last login 1792424891, `persisted: false` (call 2). The store is `W/store/mock/GERWARIC_7694.db`, 54,382,592 bytes, 22,721 items, 6 removed, 3,444 tabs, 65 characters, 4 refused bodies, league list "never fetched" (call 3). Both answers say `provider: mock` and the path is under `mock/` — so by the tool's own labels this is mock-provider data, while the brief says it is a copy of a real collection (the brief). The README says mock data is "made-up stashes … kept apart from the real"; a person reading these answers alone would conclude their items are fake. That's how this world is built, but the tool gives no way to tell the two apart.

(Correction to the T0 paragraph above: the list call 1 printed has 25 tools, not 26; I miscounted before listing them.)

## The tasks

### T1 — the host

**To the person:** "I've written the configuration the README shows, `W/work/acq.json`, with one server `acquisition` whose `command` is `/Users/tom/seat-worlds/release-2026-10/mcp-opus/bin/acq-mcp`. The server started through it and listed the same instructions and tools (call 4, 42,082 bytes, the same size as call 1)."

**Calls:** 4. **Right?** Yes, in that the host started it and it answered identically to call 1. What would say otherwise: a real host that needs `args`/`env` or a `"type": "stdio"` key, which the door accepted without.

**What the README told me / did not:** it gave the exact JSON shape, said "with the path where you unpacked it", and that it fits Claude Code's `.mcp.json` or Claude Desktop's configuration. It did not say: where Claude Desktop's configuration file lives on macOS; whether an `env` block is needed (e.g. `ACQ_ACCOUNT`, `ACQ_STORE_DIR`, `ACQ_PROVIDER=mock` for trying it) — the Knobs table lists them but never says the MCP server reads them; how the server picks the store (it found `W/store/mock/…` here, call 3, which the world's door arranges and the README does not explain for MCP); that the server's first answer is about 42 KB of tool text (call 4), which a host loads into every session.

### T2 — what I have

**To the person:** "You have 22,721 live items (call 3): 22,623 in Path of Exile 1 (call 6) and 98 in Path of Exile 2, all on PoE2 characters (call 7). PoE1 spans seven leagues — Allflame, Ancestors, Hardcore, Phrecia 2.0, Solo Self-Found, Standard, Ziz Rapture HCSSF Class Gauntlet (call 6) — but almost all of it is Standard: 3,347 Standard tabs, of which 405 have been fetched, 2,926 never fetched and 16 are folders, holding 20,231 items (call 8). The biggest are your premium tabs (163, 6,032 items), quad tabs (44, 4,814), gem tabs (12, 2,559), normal (36, 2,415) and flask tabs (11, 2,169); of 2,550 map tabs only 61 are fetched and of 502 unique tabs only 65 (call 8) — so I can't tell you what's in about 2,490 map tabs and 437 unique tabs. You have 65 characters (calls 3, 9): 59 on PoE1 (45 fetched) and 6 on PoE2 (5 fetched) (calls 6, 7); 15 characters, mostly in old leagues, were never fetched (call 9). Several fetched characters show 0 items (e.g. DLVR, gerwaric, VerySillyPerson — call 9); the README warns that 0 can mean an empty body from GGG rather than an empty character.
How old: the newest fetch is 1792074048 and the oldest 1788810111 (call 6) — by my own hand conversion about 15 Oct 2026 and about 7 Sep 2026, so the freshest data is about four days old and most tab contents (fetched_at around 1788884xxx, call 8) are about six weeks old. PoE2 was fetched around 2 Sep 2026 (calls 7, 9). The league list itself was never fetched (call 3).
Refused: the store says it holds 4 refused bodies and 6 removed items (call 3); I could not see what the 4 were — no MCP tool shows refused bodies."

**Calls:** 5–10 (call 5 refused, asking for a realm). **Right?** The counts agree across calls: 22,623 + 98 = 22,721 (calls 3, 6, 7); never_fetched 2,940 in pc = 2,926 Standard tabs + 14 characters (calls 6, 8, 9). The dates are my hand arithmetic (my one attempt to convert them with a script was refused by my harness), not the tool's: every time is epoch seconds and nothing prints an age. What would say I'm wrong: the gap between `item_count` and `fetched_items` on characters (e.g. TheAbsenceOfPatience 69 vs 37, call 9), which no text explains — I took `item_count` as the count without knowing what the difference is.

**Where it stopped or misled me:** the instructions' "Begin with search with no arguments" is refused on this store (call 5, `realm_needed`); the refusal's offer worked when sent back (call 6), but `{}` is never a beginning here. No tool exposes the 4 refused bodies (the README's tour has `acq store refused [id]`; no MCP twin). `item_events` "from recent ingests" with `hours: 2400, limit: 5` returned the five *oldest*, all `added` at 1788128184 (call 10) — no sort is offered, and its text says nothing of order. `characters` has no "with empty bodies" count, which the README's Known gaps promises for `acq store characters`.

### T3 — find things

**(a) To the person:** "Yes — 187 items with six linked sockets (call 12), all in Path of Exile 1. The first 30 are mostly body armours and bows on your characters' equipment — e.g. Skin of the Loyal on TheFireworksCompany, Tabula Rasa on BatteryStaple, Shavronne's Wrappings on gerwarwitch, Terminus Est in gerwaranger's inventory — then stash pieces in `PConc (Remove-only)` and `3.17 Skeleton Mage / 3.18 RF (Remove-only)` (call 12). The other 157 are one `next` request away. This counts only what has been fetched: 2,941 tabs and characters never were (call 12)."

**(b) To the person:** "195 of your 730 rare PoE1 rings have a `+# to maximum Life` line and at least two kinds of resistance (call 16; 730 from call 14). The best by total life is Damnation Whorl, a Two-Stone Ring with +248 life, +17% cold and +14% cold & lightning, in `Items 4 (Remove-only)` at x 8, y 6; then Honour Hold (+113 life fractured, +54 Str, fire 42 / cold 68) and Maelström Knot (+136 life, all four resistances) (call 16)."
How I read the request: life = the line `# to maximum Life`; "two resistances" = `pseudo.count_res>=2`, which counts *kinds* (fire, cold, lightning, chaos), so a single `+#% to Cold and Lightning Resistances` line counts as two (call 15). The query: `class:ring rarity=rare line("# to maximum Life") pseudo.count_res>=2` (call 16). Caveat: the life line spans -109 to 248 on these rings (call 14), so a ring with *negative* life would also match; `line("# to maximum Life" arg1>=1)` would exclude it — I did not run that.

**(c) To the person:** "Only two PoE1 leagues hold items: Standard 21,311 (rare 5,615; normal 2,850; unique 2,819; magic 2,334; no rarity — gems, currency, cards — 7,693) and Allflame 1,312 (no rarity 561; normal 276; unique 257; rare 154; magic 64). PoE2 Standard holds 98 (no rarity 43; magic 25; rare 16; normal 13; unique 1)." (call 17; sums mine: they total 22,721, matching call 3.) The other five PoE1 leagues have characters that were never fetched (call 9), so they show nothing, which is not the same as empty.

**(d) To the person:** Damnation Whorl, from call 16 (call 18): Two-Stone Ring, rare, item level 83, requires level 35, flags duplicated and identified; implicit +14% to Cold and Lightning Resistances; explicit "Adds 19 to 39 Cold Damage to Attacks against you", "-41 to Evasion Rating", "+248 to maximum Life", "+17% to Cold Resistance"; class Rings, category accessory.ring; no price ("nothing applies"); first and last seen 1788886607; and the stored body verbatim.

**Calls:** 11–18 (describe 11, 13, 15; search 12, 14, 16, 17; show 18). **Right?** (a), (c), (d) I hold right as far as the store goes; (c)'s cells sum to call 3's total. (b) depends on my reading of "life" and "two resistances", stated above. What would tell me it's wrong: a ring in the 195 whose life line is negative, or a person who meant two separate resistance *lines*.

**Where the tool stopped or misled me:** nothing stopped me. The language was learnable from `describe` in three calls, and `line:resist`/`line:life` counts (call 14) were the step that made (b) possible — it named both the templates and the `pseudo.*` values. But no tool text says that a search over this store needs a `scope.realm` (I learned it from call 5), and the `describe` answers cite repository files I cannot open (`SURFACES.md`, `search/category/data/…`, `src/pseudomods.cpp`, call 11) — noise for an agent. The ring's +248 life and "Adds … to Attacks against you" look odd for a Two-Stone Ring; the tool shows the stored body (call 18), so this is the data, not the tool.

### T4 — prices

**To the person — what you've priced (Standard, PoE1):** "Of 21,311 Standard items, 531 have a price: 492 decided by the game (a note, or a public tab's name) and 39 by your own rows; 20,780 have none (call 19). You have 16 hand-written rows: 5 apply in Standard, 11 belong to other leagues (10 items and one tab, `439cdc05be`), none are unmatched or unreadable (call 19). Priced items sit mostly in three tabs whose *names* are prices — `~price 4554 chaos` (370 items), `~price 8888 chaos` (58), `~price 2222 chaos` (9) — plus `ACQUISITION-PRICE-TEST` (49 by game, 31 by hand), `Maps (Remove-only)` (5 by hand) and three single items (call 21)."

**What looks wrong:**
- "50 items disagree between your row and the game (call 19). The ones I could read are in `ACQUISITION-PRICE-TEST`: your tab row says 5 chaos, but item notes like `~price 999 chaos` win, because a note is more specific than a tab row (call 20, e.g. an Obscured Incubator: 'by hand: 5 chaos; in game: 999 chaos')."
- "119 items carry a price note — `~price 1 divine`, `~price 10 exalted`, `~price 50 chaos` … — in tabs that are **not public**, so they are not priced at all (calls 22, 23). If you meant to sell them, those tabs need to be public."
- "One note can't be read as a price (`invalid_notes: 1`, call 19); I could not find which — no tool names it."
- "Three tab names price whole tabs at 4,554 / 8,888 / 2,222 chaos per item (call 21) — probably test values; worth a look."
- "11 rows belong to other leagues (call 19) — not wrong, but they don't show here."

**The writes:**
- Tab `Items 1` (`tab/pc/c2d7e8195f`, public, 41 items, no row of its own — call 25) at 5 chaos: created, revision 1, `prior: null`, `in_facts: true`, `written_via: "mcp"` (call 26).
- Item The Perfect Form (Zodiac Leather, in `Items 1` at x 2, y 9) at 2 divine: created, revision 1, `prior: null` (call 27).
- Read back (call 28: item row revision 1, 2 divine; its tab row 5 chaos), then replaced with 3 divine naming `if_revision: 1`: landed as revision 2, `prior` the 2-divine row of revision 1 (call 29).
- Read back (call 30): item `manual` 3 divine, revision 2, effective 3 divine "by hand (row on item/…)"; container tab `manual` 5 chaos revision 1, effective 5 chaos. Both new prices where the tool reads them back.

**What each receipt lets me undo** (by `price_set`'s own text, call 1): the two creates (calls 26, 27) — `price_clear` with `if_revision` the written revision (1). The replace (call 29) — `price_set` with the prior's value (2 divine) and `if_revision: 2`. Every receipt carries both rows whole, so the undo needs no other read.

**Knowing another program changed a price between read and write:** the revision. I read revision 1 in call 28 and named it in call 29; if anything (the CLI, another agent) had written in between, the row's revision would have moved and — per the tool text — the write would be refused naming the current revision. I test that refusal on purpose in T7. `updated_at` and `written_via` (`mcp` vs the old rows' `cli`, call 20) would also show who wrote last.

**Calls:** 19–30 (reads 19–25, 28, 30; writes 26, 27, 29). **Right?** Yes for the writes — each receipt and readback agrees. The status counts I hold right but not reconciled: `has:priced` in Standard is 525 (call 21) against 531 effective in call 19; I believe the 6 are the `skip`/no-price statements (`has:priced` says those lack it), but no answer says so. Call 19's `residue: 1395` I could not interpret.

**Where it stopped or misled me:** `price_list` for the 50 conflicts was 66,806 bytes and the door cut it to its first 3,000 characters (call 20): I saw one conflict whole and could not list the 50 — there is no `limit` or count-only form on `price_list`. I worked around it with a search cross (call 21), which cannot filter by `relation`. Nothing names the one invalid note. `price_show` on a 41-item tab was 28,857 bytes (call 25) — every item's full listing, no way to ask for the tab alone.

(Correction to T4: `Items 1` holds 39 items, not 41 — I counted the `items_here` of call 25 again against the page of call 31.)

### T5 — the shop

**To the person:** "Your Standard shop is one forum page, 'Shop Post 1 of 1 (69 items)', 5,076 characters (call 31). It lists 69 items you priced by hand: 31 in `ACQUISITION-PRICE-TEST` (shown as `Stash57`: 29 at 5 chaos, one at 2 chaos, one at b/o 1.5 divine) and 38 in `Items 1` (`Stash64`) — 37 under the `~price 5 chaos` spoiler from the tab price, and The Perfect Form alone under `~price 3 divine` as `[linkItem … location="Stash64" … x="2" y="9"]` (call 31).
Left out (call 32's `policy`): 489 items the game already lists at its own price ('a page must not contradict what the site shows') — including Ghoul Joy in `Items 1`, whose own note says 10 chaos; 3 the game marks `~skip`; 3 you marked skip by hand; 5 blocked because they sit in a map or unique substash, which the forum cannot link; and 20,742 with no price at all. Those add up to 21,311, every Standard item (calls 19, 32).
Next: copy the page's text and paste it into a forum shop thread yourself — the tool posts nothing. Keep the items where they are: each link names a tab and a cell."

**Cleanup:** removed the item's row naming revision 2 (call 33; receipt `written: null`, `prior` the 3-divine row) and the tab's naming revision 1 (call 34; `prior` the 5-chaos row). Read back: The Perfect Form and `Items 1` both "no row applies and the game states nothing" (call 35); `price_status` identical to before I started — 16 rows, 39 manual, 50 conflicts, 20,780 none (call 36 vs call 19).

**Calls:** 31–36. **Right?** The page and its placement I hold right (counted cell by cell against call 25). The left-out counts are from the part of call 32 the door printed; the tail of `policy` (from `tab_unliste…` on), all of `left_out`, `counts` and `freshness` were cut — I infer the unseen policy cells are 0 only because the seen ones already sum to 21,311. What would say I'm wrong: a non-zero cell after the cut, which would mean double counting.

**Where the tool stopped me:** `shop_render` with no `page` answered **5,057,320 bytes** (call 32) — it lists every item not posted, 20,742 of them unpriced, in `left_out`. The door cut it to 3,000 characters; a host would put 5 MB into an agent's context or refuse it. There is no argument to leave `left_out` out or to ask only counts and `freshness`. `page: 1` (call 31) is the usable form, but it drops `policy`, `left_out` and `freshness` entirely, so I could not report freshness at all. The README did not say where to paste, or that items must stay put and tabs stay public for links to work.

### T6 — the daemon

**Which tools say they need a daemon** (their texts, call 1): `submit_job` and `apply_plan` ("In real-GGG mode the daemon must already be running (a human starts it; this server never spawns one there)"), `job_status`, `job_result`, `cancel_job` (job ids from the daemon), and the server's instructions: "Job tools talk to the local daemon". `daemon_status` and `list_jobs` observe only and answer without one; `refresh_plan` is offline and only *adds* a quote from a running daemon.

**What they told me with none running:**
- `daemon_status`: `{"persisted":null,"running":false}` (call 37); `list_jobs` the same (call 38). Nothing about how to start one.
- `refresh_plan`: a 5-action plan (list stashes + 4 stale tabs, `logical_requests: 5`, 5–15 wire sends) and `quote_note: "no quote: no daemon running — plan compiled offline"` (call 39). Its `age_seconds: 361286` (≈ 4.2 days) confirms my hand-computed age in T2.
- `submit_job {"kind":"leagues"}`: a JSON-RPC protocol error `-32603`, "daemon is not running and ACQ_NO_SPAWN forbids starting one from here", `data.connect: "absent"` (call 40).
- `apply_plan` with call 39's plan sent back whole and `max_requests: 5`: the same `-32603` error (call 42).
- `job_status {"id":1}`: `-32603`, "no daemon running" (call 41).
None of them said what a person should do. `ACQ_NO_SPAWN` is the door's (or the host's) environment, not something the person set; the message names an internal knob instead of the remedy.

**What the README tells the person to do:** "Your agent": the fetching tools need a daemon, `acq-mcp` never starts one against GGG, the daemon exits after a minute idle, so start one with a longer idle time while none is running:
`ACQ_IDLE_SHUTDOWN=3600 acq auth check` (login back from the keyring) or `ACQ_IDLE_SHUTDOWN=3600 acq auth` (without a keyring). PowerShell: `$env:ACQ_IDLE_SHUTDOWN = 3600` first.

**What I would tell the person to type:** "Everything you asked so far worked without a daemon. If you want me to refresh — the plan I have would fetch your stash list and 4 stale tabs, 5 requests (call 39) — open a terminal at your desktop (not over ssh) and type:
`ACQ_IDLE_SHUTDOWN=3600 acq auth check`
If it says you're not logged in, type `ACQ_IDLE_SHUTDOWN=3600 acq auth` and log in on GGG's page. Then tell me, and I'll check `daemon_status` before I apply the plan. It stays up an hour after its last job." (In this world there is no login and no daemon — the brief — so I could not verify any of it.)

**Calls:** 37–42. **Right?** The reading of what the tools say, yes. Whether `acq auth check` actually leaves a daemon running that this MCP server then finds I cannot check here, and the README's line ("a daemon that idles out after an hour, your login back from the keyring") is all I have.

**Where it misled me:** the server's instructions say "a refusal is a result marked isError with readings and offers"; the three daemon refusals are protocol errors instead, with no `readings`, `offers` or `kind` (calls 40–42). An MCP host often shows a protocol error as a tool failure rather than handing the agent the text. `apply_plan` reached the daemon check without first checking the plan's validity offline — so I can't tell from call 42 whether my envelope would have been accepted.

### T7 — for a program

1. **Wrong account** (`GERWARIK`): `search` → `{"error":"no account matches `GERWARIK`: `scope.account` takes a username, with or without its #discriminator, or a uuid","kind":"account_unknown","readings":["GERWARIC#7694"]}` (call 43); `price_status` → the same shape, naming `account` (call 44). **One document a program can read:** yes — `kind` is a stable key and `readings` holds the right name. **Told me what to do next:** yes, it names the only account. No `offers` (a ready request) on search, though search's own text promises them "where a way on is a search".
2. **Target that does not exist** (`item/00000000deadbeef`): `price_show` → `{"error":"item/00000000deadbeef is not in the facts for Standard (21311 items, 3388 tabs and characters on record); `acq search` finds an item's id, `acq tabs` a tab's"}` (call 45) — **no `kind`, no `readings`**: a program can only string-match the prose, and the way on is given as CLI commands (`acq search`, `acq tabs`), not as the MCP tools `search`/`tabs`. `show` → `{"error":"no item … in this store: an id is given whole, as an answer printed it (22721 live items, every realm)","kind":"item_not_found"}` (call 46) — has a `kind`, tells me ids must be whole. Partly readable, partly told what to do.
   And the one that was *not* refused: `price_set` on the same non-existent item **landed** — revision 1, `in_facts: false` (call 47). Its text says so ("a typo: the row stands"), but a write a program makes to a mistyped id succeeds; only `in_facts` warns. I removed it (call 50) and `price_status` is back to its starting state (call 51 = call 19).
3. **Price write naming the wrong revision** (7, where the row is 1): on my stray row → `{"error":"annotation item/00000000deadbeef/buyout is at revision 1 (re-read and retry)"}` (call 48); on the real `ACQUISITION-PRICE-TEST` tab row → `{"error":"annotation tab/pc/03cb479c65/buyout is at revision 1 (re-read and retry)"}` (call 49). Nothing was written. **One document a program can read:** barely — no `kind` (so a conflict can't be told from any other refusal), and the current revision is only inside the sentence, not a field, though `price_set`'s text promises the refusal is "naming the current revision". **Told me what to do:** yes, "re-read and retry", and the number to name. "annotation" and "/buyout" are internal words that appear nowhere else in the tool texts.

**To the person / a program's author:** "Account mistakes are cleanly typed and say the right name. Missing targets and revision conflicts come back as prose with no `kind`, and the daemon refusals (T6, calls 40–42) aren't tool results at all but protocol errors. A write to a target that doesn't exist is accepted."

**Calls:** 43–51 (refusals 43, 44, 45, 46, 48, 49; the accepted typo write 47; cleanup 50, 51). **Right?** Yes; each refusal is quoted whole as printed.

## Faults

Worst first.

1. **`shop_render` with no `page` answers 5,057,320 bytes** (call 32). It lists every item not posted, 20,742 of them with no price at all, in `left_out`, and has no argument to leave that out or ask only for `counts`/`policy`/`freshness`. It should answer with counts by default and list members on request, as `search` does. Not a refusal. *Who hits it:* an agent. A host either fills its context with 5 MB or fails the call, and the useful parts (freshness, the tail of `policy`) come after the list. The door cut it to 3,000 characters, which is how I lost `freshness`. A person at the CLI likely has `--expand` to guard this (README tour), so probably not them.
2. **A price write to a target that doesn't exist lands** (call 47: `item/00000000deadbeef`, revision 1, `in_facts: false`). The text says so ("a typo: the row stands"), but it means a mistyped id silently creates a row that no listing shows. It turns up only as `unmatched` in `price_status`. Refusing unless the caller opts in would be safer, or at least a refusal-shaped warning. *Who:* both. The CLI's `price set` says it behaves the same way (call 1, "Stricter than `acq price set`" applies only to revisions).
3. **Daemon refusals are JSON-RPC protocol errors and don't give the remedy** (calls 40, 41, 42). *Warranted:* yes, there's no daemon (the brief). *Enough to recover:* no. "ACQ_NO_SPAWN forbids starting one from here" names an internal knob the person never set. It doesn't say "ask the person to run `ACQ_IDLE_SHUTDOWN=3600 acq auth check`", which is the README's answer. It also breaks the instructions' promise that "a refusal is a result marked isError with readings and offers". *Who:* an agent. A person at the CLI gets a daemon started on demand.
4. **`price_list` has no limit or count form** (call 20: 66,806 bytes for the 50 conflicts, cut to 3,000 characters). I could read one conflict of 50. Similarly `price_show` on a 39-item tab is 28,857 bytes (call 25). *Who:* an agent.
5. **Missing-target and revision-conflict refusals have no `kind` and no fields** (calls 45, 48, 49). *Warranted:* yes, my requests were wrong on purpose. *Enough to recover:* for a reader yes ("re-read and retry", "is at revision 1"; `acq search` finds ids). For a program no: the current revision is only in prose, though `price_set`'s text promises the refusal "naming the current revision". Call 45 points at CLI commands instead of the MCP tools. *Who:* an agent or program.
6. **The instructions' first step is refused on this store** (call 5). "Begin with search with no arguments" gets `realm_needed` on any store with PoE2 characters. *Warranted:* by design. *Enough to recover:* yes, the offers worked when sent back (call 6). The fault is that the instructions open with a step that fails for anyone who has played PoE2. *Who:* an agent.
7. **No MCP way to see the 4 refused bodies or the 1 invalid note** (calls 3, 19). The CLI has `acq store refused [id]` (README tour). *Who:* both; a person can use the CLI.
8. **Ages are epoch seconds only** (calls 6, 8, 9). No answer says "4 days old", except `refresh_plan`'s `age_seconds` (call 39). I had to work the dates out by hand. *Who:* an agent; a person at the CLI probably sees dates.
9. **`item_events` returns the oldest rows first** (call 10, `hours: 2400, limit: 5` → five events from 1788128184). Its text says "recent ingests" and offers no sort. *Who:* both.
10. **`characters` shows `item_count` and `fetched_items` disagreeing** (e.g. 69 vs 37, call 9) with no text saying what each counts. *Who:* both.

## What the README and the tool texts did not say

- **A search over this store needs `scope.realm`.** I expected it in the server's instructions, which say the opposite ("Begin with search with no arguments"), or in `search`'s text, which only says "needed only when the store holds several". I learned it at call 5.
- **The full tool list.** The README's "Your agent" section names 11 tools and "four more", and points at `MCP-REFERENCE.md`, which I could not open. The job, daemon and policy tools appear only in `open` (call 1).
- **Whether the host configuration needs `env`** (`ACQ_ACCOUNT`, `ACQ_STORE_DIR`, `ACQ_PROVIDER`), and where Claude Desktop's file is. I expected both in "Your agent". Not learned; the bare `command` worked here (call 4).
- **That a note in a private tab is no price.** `price_status`'s text says "in game … read only where the tab is public" (call 1), but the README never says it, and I only saw what it means at calls 22–23: 119 noted items unpriced.
- **`shop_render`'s size** and that `page` drops `policy`/`left_out`/`freshness`. I expected this in `shop_render`'s text. Learned at calls 31–32.
- **What to tell the person when a job tool finds no daemon.** The README says it ("Your agent"); no tool text or refusal does (calls 40–42).
- **What a person does with the shop page** (which forum, that items must stay put and tabs public). I expected it in the README's opening or "What it does not do". Never stated.
- **`price_set` on a missing target creates a row.** This *is* in `price_set`'s text (`in_facts`). I confirmed it at call 47.
- **Where the README pointed and I could not follow:** `MCP-REFERENCE.md` (mattered: the tool list), `CLI-REFERENCE.md`, `AGENTS.md`, `LIVE-TESTING.md`, `decisions/*.md`, `CONTEXT.md`, the source links in Knobs (didn't matter).
- **What the README said that I never needed:** Install (archive table, curl, quarantine, Windows, Linux glibc); Log in; most of the CLI tour (used once, to learn `store refused` exists); Knobs (none applied to an MCP host as far as the README says); the rate-limit clock (no daemon); Known gaps on policy set and the mock; Licence, Reporting.

## The tool's own words

- **Right and useful:** the account refusal (calls 43, 44: `kind` plus `readings` with the right name); `refresh_plan`'s `quote_note` "no quote: no daemon running — plan compiled offline" (call 39); the price receipts with `written`/`prior` whole (calls 26, 27, 29, 33, 34); `price_show`'s `why` lines ("999 chaos in game: the note is more specific than the row on the tab", call 20); `shop_render`'s policy `why` per cell (call 32); `describe`'s `line:` vocabulary (call 14).
- **Wrong or misleading:** "Begin with search with no arguments" (call 1 vs call 5). "a refusal is a result marked isError with readings and offers" (call 1 vs calls 40–42, 45, 48, 49). `price_set`'s "refused naming the current revision": the revision is only in the sentence (call 48). `item_events` "recent" (call 10). Call 45 names `acq search`/`acq tabs`, not the MCP tools. `provider: mock` and the store path `store/mock/…` (calls 1, 2, 3) on what the brief says is a real collection copy: the tool is truthful about how it runs, but by the README's words ("made-up stashes … kept apart from the real") a person would think their items are fake. That comes from how this world is built.
- **Missing:** a remedy in the daemon refusals; human ages; counts-only forms of `shop_render` and `price_list`.
- **Words I did not understand:** `residue: 1395` (calls 19, 36); `unlifted_items`, `withheld_items`, `withheld_responses` (call 3); `derivation`, `intent: 33` in `basis` (every search); "annotation … /buyout" (calls 48, 49); `kind: "buyout"` on an `exact` row (call 26).
- **Noise:** `describe` cites repository files and commits (`SURFACES.md`, `search/category/data/…`, `src/pseudomods.cpp at acquisition-qt@946a4f51`, call 11) that an agent can't open.
- **Read and never used:** `trade` queries, `explain`, `context: corpus`, `membership: removed`, `holds()`, `view.routes`, `sync_policy`/`set_sync_policy`, `reference_currency`, `job_result`, `cancel_job`, `template` and `size` on `shop_render`.

## What you would ask of the tool

- `shop_render` returning counts, policy and freshness by default, members of `left_out` on request (call 32).
- A `limit`/`next` on `price_list`, and a tab-only form of `price_show` (calls 20, 25).
- Daemon refusals as `isError` results with `kind: "daemon_absent"` and the command a person should type (calls 40–42).
- `kind` and `current_revision` fields on a revision conflict, and `kind` on a missing target (calls 45, 48, 49).
- `price_set` refusing a target not in the facts unless asked to keep it (call 47).
- An MCP twin of `acq store refused` and a way to find the invalid note (calls 3, 19).
- An `age_seconds` beside every `fetched_at`, or ISO dates (calls 6, 8, 9).
- Instructions that begin with a request that works on a two-realm store (call 5).
- Newest-first, or a sort, on `item_events` (call 10).

## Left out

- **Not tried:** a trade-site query; `sync_policy`/`set_sync_policy`; `reference_currency`; `job_result`, `cancel_job`; `membership: removed`; `explain`; paging the other 157 six-links and 185 rings; the refined ring query `line("# to maximum Life" arg1>=1)`; `shop_render` with `size`/`template`; `price_list` filtered by `in`, to list the 50 conflicts a tab at a time.
- **Files opened that are not the two:** one. My harness saved call 1's printed output (42 KB) to its own file, `~/.claude/projects/-Users-tom-seat-worlds-release-2026-10-mcp-opus/11891c2e-…/tool-results/bve6d66dq.txt`, instead of showing it inline, and I read that file to see call 1 whole. It is the door's output as printed, but it is a file outside the two. It did the same for call 4 (`bydps0juj.txt`), which I did not open; I relied on call 4's identical size and its preview.
- **Files written:** `W/work/acq.json`; `W/drafts/t1t2.md`, `t3.md`, `t4.md`, `t5.md`, `t6.md`, `t7.md`, `final.md` (report parts, because my harness refused a heredoc), and `W/drafts/apply.json` (call 42's arguments).
- **Also:** one Bash attempt to convert epoch times with a Python one-liner was refused by my harness. It didn't touch the tool or store. The dates in T2 are hand arithmetic, checked against `age_seconds` in call 39.
- **Store state:** every price I wrote is removed; `price_status` at call 51 equals call 19.
- **The cut I most want back:** call 32's tail, meaning `freshness`, the rest of `policy`, and `counts`, so I could tell the person whether the shop's items are older than their sync window.
