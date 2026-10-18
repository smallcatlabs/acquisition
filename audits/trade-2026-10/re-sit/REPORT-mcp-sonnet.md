<!-- written during the run; call numbers are rows of calls.csv -->

## 1. Before touching anything (T0)

**What the brief told me (the brief, not the tool):** the tool is acq-mcp, MCP server of Acquisition, an inventory tool for Path of Exile; one account, GERWARIC, a copy of the person's real collection with their prices; no login and no daemon exist in this world, so anything needing the game's API will say it cannot reach it; the world is a mock, safe to write to.

**What `open` told me (call 1):**
- Instructions: search answers a query over every item in the store; describe prints the query language; show is one item whole. A trade-site search (URL, id or JSON) is `{"trade": …}` translated to text. "No tool says what an item is worth: a price is the owner's own listing."
- Store tools (search, show, accounts, tabs, characters, store_status, item_events) read the SQLite store directly: no daemon, no network.
- Refresh slice: sync_policy / set_sync_policy (local intent), refresh_plan (offline), apply_plan (spends the plan as a parent job; needs the daemon).
- Job tools (submit_job, job_status, job_result, cancel_job, list_jobs, daemon_status) talk to the local daemon; API requests are jobs. Login is done by the human via the `acq` CLI.
- It says to begin with `search` with no arguments and `describe` with names ["fields"].

**What I would not run without asking the person:** submit_job, apply_plan, set_sync_policy, cancel_job (these change state or send requests to the game's API through a daemon; in a real setting they spend rate limit and may hit the real account). refresh_plan and the store readers are read-only.

**The world as found:**
- call 2: `search {}` was refused (`realm_needed`): the store holds 2 realms (pc, poe2); the refusal offered the request for each.
- call 3 (store_status): 22721 items, 6 removed, 3444 tabs, 65 characters, 22751 events, 1133 responses, 54,382,592 bytes; `leagues: 0` (odd beside the data); path under W/store/mock/GERWARIC_7694.db.
- call 4 (pc realm): 22623 live items, 21311 in Standard and 1312 in Allflame; rarity rare 5769, normal 3126, unique 3076, magic 2398, no rarity 8254. Scope lists 7 leagues. `never_fetched` 2940 (MapStash 2582 tabs of which 93 fetched; UniqueStash 521 of which 84 fetched; character 59 of which 45 fetched); `never_fetched_leagues` is ["Standard"], so Standard has tabs never fetched and answers there say nothing about what those hold.
- Age: oldest_fetch 1788810111, newest_fetch 1792074048, snapshot at 1792074278 (seconds since 1970; roughly 7 Sept to 12 Oct 2026 by my arithmetic, not printed by the tool).
- call 5: one account, GERWARIC#7694, provider mock.

## 2. The tasks

### T1 — ring (calls 6, 7, 8)
Note: the harness refused a heredoc call once (no number printed), so I passed arguments with `--from` drafts (calls 6–8).
- **Told the person:** the tool translated the site search with fidelity "exact", remainder empty (call 6): Standard, ring, life>=50, dex>=20, and (chaos res>=20 or fire res>=40). It finds 22 of their rings (call 6 total.matched 22; rows 1–20 in call 6, the last 2 via view.rows.next in call 7: Dire Twirl in "~price 50 chaos (G) (Remove-only)" cell 4,0 and Plague Grasp in "Mercenary Items (Remove-only)" cell 11,2). Examples: Death Circle is on character VVickedBossingServices (equipment); Pain Nail in tab MARKET (2,1); Hate Turn in Lab Runners (7,4); Storm Band in Dump Tab (20,19, the newest fetch); most others sit in "(Remove-only)" Premium tabs. Each row gives tab name, tab id, cell and fetched_at.
- **Without dexterity (call 8):** 91 match instead of 22, so dropping dex adds 69; 10 of the 91 have no tab (characters or similar; `-has:tab` 10). The by-tab table shows 20 of 56 tabs (36 left out, would need view.counts.limit). I did not list the 69 individually.
- **Right?** I hold the 22 to be right as a translation of the site's search over what is stored. It is not proof of what they own: Standard has never-fetched tabs (call 6 scope `never_fetched_leagues` ["Standard"]), so rings in unfetched tabs are not counted. Fetch times in call 6 are mostly 1788885xxx–1788888xxx (early Sept) except Dump Tab (1792074047). What would tell me it's wrong: a ring in a never-fetched tab, or a fresher fetch changing stats.
- **Stopped/misled:** 20-row default page, 49,261 bytes on the wire for call 6 (rows repeat the whole scope block). Pseudo-stat sums like Grim Grasp's total_life 87.5 (76 life + 23 str/2) are computed, not shown on the item. The `status` clause (online/available) is `listing_only` and not run, so "what a buyer sees" is not answered by this search. The row has no price.

### T2 — boots (calls 9, 10, 11)
- **Told the person:** the search as pasted finds nothing of theirs (call 9, total.matched 0). It reads as: Standard, boots, armour at 20% quality >= 300, movement speed >= 25, total elemental res >= 100, life >= 75. Fidelity is "ambiguous" (call 9): the armour clause is `ambiguous` ("within one of the site's own figure, which the display's rounding can move"), no remainder. Dropping only the armour clause gives 2 boots (call 10): Ambush Spur (Stealth Boots) in "3.19 Death's Oath Occ" cell 7,6, and Gale Spark (Harpyskin Boots) in "Soulrend PF (Remove-only)" cell 7,6. `show` with `against` armour.q20>=300 (call 11) says Gale Spark `lacked` it: its only defence is Evasion Rating 586, no armour. So those two pass everything except the armour the friend asked for; I'd say so, and not call them matches of the search.
- **Right?** The zero looks right for the search as the site would run it. Two limits: Standard has never-fetched tabs (call 9 scope), and the tool names the armour clause ambiguous but gave no `other` reading or adds/drops for it, so I cannot say from the answer whether the ambiguity could change the zero. What would tell me it is wrong: a boot with armour in an unfetched tab. I did not check Ambush Spur's armour.
- **Stopped/misled:** the zero page offers 6 `explain` requests (one per term) but not the one thing I wanted, which term(s) alone empty it; I found that by dropping a term by hand (call 10) and by show (call 11). Call 11's price block: `status none, "nothing applies"`.

### T3 — shield (call 12)
- **Told the person:** one match (call 12, total 1): the unique Tukohama's Fortress (Ebony Tower Shield), in tab "Facebreaker Totems (Remove-only)" (id 8894d5b692), cell 7,3, fetched 1788886624. It has armour 473 (at 20% quality), +1 to maximum Summoned Totems, total life 117 (91 + 26 implicit). The search is Standard, shield, armour.q20>=100, +# maximum Summoned Totems (explicit), life>=100.
- **Right?** Yes, I hold it right. Fidelity "ambiguous" only because of the armour clause, whose 473 is far over 100, so that ambiguity cannot change the result (my reading; the tool gave no `other` for it). Would be wrong if an unfetched Standard tab holds another shield.
- **Calls:** 1. Nothing stopped me.

### T4 — magic find (calls 13, 14)
- **Told the person:** the site search is Standard + total explicit "#% increased Rarity of Items found" >= 20. 160 items of theirs match (call 13, total 160; 20 rows shown, 140 left out; I did not page through all 160). The tool flags the clause `ambiguous` (the game also shows reduced Rarity) but its `other` reading adds 0 and drops 0 (call 13), so it makes no difference here. Only the ones with at least 30 (call 14, query = old text in parentheses plus a new term): 27 items. In 16 tabs plus 3 in no tab (characters): "~price 4554 chaos" 5 (a UniqueStash folder), "3.13 Golems, BL, Misc (Remove-only)" 2, "Small Jewelry (Remove-only)" 2, "~price 4444 chaos (Remove-only)" 2, and one each in 3.12 Scion, 8 chaos, Corrupted Uniques, Gear #2 (3.16), Items 4, Lab Bases #1, Mercenary Items, PConc, Special Gear, |||||| MARKET |||||| , ~price 20 chaos (A), ~price 30 chaos, ~price 40 chaos (F) (all Remove-only except Gear #2 and ~price 4554). From call 13 the 30+ visible on page 1 include Presence of Chayula (character BatteryStaple, and another in the 4554 folder), Aurseize (50, character VVickedWilliam), Doom Grip (32, LordHelpTheMistress), Ventor's Gamble (31, PConc).
- **Right?** Mostly. One inconsistency: the count table totals 24 in tabs + 3 with no tab = 27, matching total 27, but term 1 alone says 30 items match >=30 over the whole scope (call 14 terms; that includes Allflame, since the league term is separate). Consistent, per the definition "each term alone over every item in scope". I did not list which items the 27 are; only the tabs. If the person wants "where", cell and item names for those 27 I would page rows (not done).
- **Stopped/misled:** counting by tab names the UniqueStash folder (parent) rather than the sub-tab (call 13 rows show `name: null` with `parent`); the tab count's name for those is the parent's, not a tab with its own name. The word "rarity" in the person's ask means item-rarity stat; the tool's `rarity` field is something else (item rarity class). I took the stat reading.

### T5 — fire resistance (call 15)
- **Told the person:** the search is not a gear filter: it is Standard plus the sum of "#% to Fire Resistance" lines (explicit, and scourge) >= 30, no item-type restriction. 510 of their items match (call 15 total 510): rings 94, helmets 85, chests 60, boots 59, amulets 54, belts 54, gloves 47, shields 26, quivers 13, and 22 weapons (wand 5, mace 3, bow 2, rune dagger 2, sceptre 2, one each of sword, claw, one-hand axe, two-hand axe). I counted by category rather than listing 510 rows; I did not list where each is. If they want "where", I would narrow by category and page rows (not done).
- **Right?** The counts add up to 510 (my sum). Fidelity "ambiguous": the scourge part of the sum is ambiguous ("the site may leave it out"); its `other` reading (explicit only) adds 0 and drops 0 (call 15), so no difference here. A caveat I could not settle: the line counts "#% to Fire Resistance" only, so an item whose fire res comes from "Fire and Cold Resistances" or "all Elemental Resistances" is not in these 510, whereas the pseudo total in T1 counts those. The tool called the clause exact, so I take it that the site does the same.
- **Calls:** 1. Stopped by nothing; the term list says 526 for the term alone (includes Allflame).

### T6 — maps (calls 16, 17)
- **Told the person:** the search is Standard, map tier >= 14, and a blighted map (category map and name starting "Blighted "). They have 83 (call 16 total 83; call 17). By tier (call 17): tier 14 = 44, tier 15 = 22, tier 16 = 17. By tab (call 17): "Blight Maps" (QuadStash abaabd4b60) 69, "Blight" (BlightStash 0d066681f3) 14. The 20 rows of call 16 show cells (e.g. Blight tab x 21–59, y 0; Blight Maps e.g. 0,14 and 3,5), fetched 1788884441 and 1788885901.
- **Right?** The tier counts agree with the typeline breakdown in call 16 (44, 22, 17; they sum to 83). Caveats: (a) fidelity is "ambiguous" because the tier is read from the base name, "a map whose base names no tier has none here, where the site gives it one" (call 16/17 clause note) — a blighted map without "(Tier N)" in its name would be missed and the tool did not say whether there are any; (b) Standard has never-fetched map tabs (call 16 scope: MapStash 2582 tabs, 93 fetched), so the person may hold more blighted maps than 83 in tabs the tool has never read; (c) "Blighted" is matched by name only.
- **Stopped/misled:** the `tab`-count in call 17 worked; I did not need rows beyond the first 20 of call 16.

### T7 — Loreweave (calls 18–23)
- **Told the person:** I did not find a Loreweave in what the tool holds. `name:loreweave` over the pc realm, live and removed items, matched 0 (call 21); the zero page's only suggestion was "Fenumus' Weave" (a unique Carnal Mitts, in "3.12 Pathfinder (Remove-only)" cell 8,0 — not what they asked for). Over all realms `name:weave` matched 32 (call 22), none named Loreweave among the 10 names it resolved (more:10 not shown); the only `Sacrificial Garb` unique, the base I know Loreweave to be (my game knowledge, not the tool's), is Atziri's Splendour in "~price 4444 chaos (Remove-only)" (call 23). The honest answer is "not in the stored data", and the stash is incomplete: unique-stash tabs are `never_fetched` 521 tabs of which 84 fetched (calls 21–23 scope), so the Loreweave may sit in a tab the tool has never read, and the tool can say no more than that.
- **Right?** I hold "not found" to be right for the fetched data and no more. Would be wrong if a never-fetched unique tab holds it, or if it was stored under a different name.
- **Where the tool stopped/misled:** call 18, bare `Loreweave` refused with `bare_word` ("never guessed: say what it is") and readings `"Loreweave"` and `line(template:Loreweave)`; neither is a name search. Call 19, the first reading, ran as `text:` and matched 0 with no hint that `text` is not the item name. I needed `describe` (call 20) to learn `name:` and found the search by `name:`/`name=` there. 5 calls to reach the right query. The refusal did not offer `name:`.

### T8 — what I'd take (calls 24–28, using picks from calls 6, 7, 9–12)
- **Told the person:**
  - **Ring: Fate Twirl** (Ruby Ring, rare, ilvl 75, req level 64). Highest life of the 22 matches (110; next is Doom Circle with 95, call 6). Tab "3.25 RF Chieftain (Remove-only)" (id e50887031b), cell x 6, y 7 (call 24; stored fetch 1788887707). Everything the tool knows (call 24, `show` with body): enchant "Your Meteor Towers create Burning Ground for 3 seconds on Hit"; explicit +35 Dexterity, +110 maximum Life, +44 maximum Mana, +43% Fire Resistance, +15% to Fire and Chaos Resistances (crafted); implicit +26% Fire Resistance; no properties, no note, price `none` ("nothing applies"); item id 176c179e…95d5fb1a; stash, inventoryId Stash1, 1x1, first_seen = last_seen = 1788887707, not removed, identified, verified false. The pseudo fire res sum 84 = 43 + 15 + 26; its chaos res is 15, under the 20 the search asked for, so it matched on the fire side only (call 6, term 4.1). I did not weigh price or what they want on a build.
  - **Boots: Gale Spark** (Harpyskin Boots, rare): "Soulrend PF (Remove-only)" (id b277a2a094), cell 7,6 (calls 10, 11): +124 life, 140 total elemental res, 30% movement speed, Evasion 586. It does NOT meet the friend's search (no armour, call 11); I'd say so. Alternative: Ambush Spur in "3.19 Death's Oath Occ" cell 7,6 (101 res, life 75, 30% MS; call 10).
  - **Shield: Tukohama's Fortress** (unique): "Facebreaker Totems (Remove-only)" (id 8894d5b692), cell 7,3 (calls 12, 28): +1 totems, 40% increased Totem Damage, +91 life (+26 implicit), +300 Armour per Summoned Totem, Blood Magic, 25% block, 473 armour, 3 red sockets, req level 70, 159 Str; price none.
- **Which a buyer on the site could see (the person's last question):** the tool cannot answer. Of the 22 rings, `has:priced` matched 0 (call 26; 591 items in the whole pc scope are priced). Gale Spark and Tukohama's Fortress also show price `none` (calls 11, 28). Dire Twirl, whose tab is named "~price 50 chaos (G)", also shows `none` with the reason "the price text in game is not where the index can see it" (call 27), so the tool cannot see a price set through a tab name. Nothing in the tool says whether a tab is public or an item is listed (the site-side `status` clause is `listing_only`, "means nothing in your stash", calls 6, 9). What I would tell the person: none of the three I picked carries a price the tool can see; whether any is listed on the site I cannot say.
- **Whether the tool ran the same search the site would have:** T1 and T5 translations were "exact" or flagged only a no-difference ambiguity (calls 6, 15); T2, T3, T6 had one `ambiguous` clause (armour, map tier) with no alternative reading given (calls 9, 12, 16); T4's ambiguity changed nothing (call 13). All run over stored fetches, not over the site's listings, so "same search" holds for the filter only.
- **Right?** The picks are my judgement over a store with old fetches (the picks' `fetched_at` are 1788885886–1788887707) and unfetched tabs. What would tell me they are wrong: the item has moved (item_events or a fresh fetch, which needs the daemon; I tried neither).
- **Calls:** 5 (24–28); the price lookup took 3 of them (25 refused `price`, 26 `has:priced`, 27 show).

## 3. Faults (worst first; P = a person would hit it too, A = only an agent)
1. **A buyer's view cannot be read, and the tool says nothing about it** (calls 26, 27, 11, 28; P and A). A price set through a tab name shows as `none` with a reason (call 27), but "which would a buyer see" needs tab publicity and listing state, which no tool or search field offers. The instructions say "a price is the owner's own listing" but not that tab-name prices are invisible, nor that nothing says "listed". Worst, because the person's central question hangs on it.
2. **Completeness of the data is told in totals, not per question** (calls 4, 6, 21–23; P and A). `never_fetched` covers all tabs (UniqueStash 521, 84 fetched; MapStash 2582, 93 fetched); `never_fetched_leagues` names only Standard. So "no Loreweave" (call 21) and "83 blighted maps" (call 16) come with no way to say how much is unread; a zero cannot be told from a gap.
3. **A bare name is refused and the refusal sends you to the wrong field** (calls 18, 19; P and A). Readings offered: `"Loreweave"` (becomes `text:`, matched 0, call 19) and `line(template:…)`. `name:` was never offered; I got it from `describe` (call 20). 5 calls to ask for a unique by name.
4. **A zero page does not say which clause empties it** (call 9; P and A). It offers six per-term explains; I found the culprit by hand (calls 10, 11).
5. **Ambiguity sometimes has no alternative reading** (calls 9, 12, 16; P and A). Calls 13 and 15 gave `other` with adds/drops; the armour and map-tier ambiguities did not, so I could not say whether they matter (I reasoned from the values that they did not).
6. **Answers repeat a large scope block** (call 6: 22,692 bytes of text for 20 rows; 49,261 bytes on the wire; mostly A). Each call also repeats `basis`.
7. **`store_status` says `leagues: 0`** while search lists 7 leagues in scope (calls 3, 4; P and A).
8. **Counting by tab on a UniqueStash names the parent folder** (calls 13, 14): rows show `name` null with a `parent`; the tab count shows the parent's name.
9. **Small:** `describe` for `name` returns a `totals` text that names a build-internal source path (call 20: "src/pseudomods.cpp at acquisition-qt@946a4f51").
(Not the tool's: the harness refused two of my heredoc calls, with no call number; I used `--from` drafts for the calls after the first and for this report part.)

## 4. What the instructions and the tool texts did not say
- A search needs a realm when the store holds several: the `open` text half-says it ("over a store of several realms that is refused") but not that this store is one; learned at call 2.
- The item-name field is `name`, and a bare word is not guessed: learned at calls 18 and 20; I expected it in the search tool's description.
- `price` is not a search field; `has:priced` is (call 25 refused with readings, call 26 ran). I expected it in the instructions.
- That prices set in tab names (`~price …`) are not seen (call 27): expected next to "a price is the owner's own listing".
- That `"…"` (what the bare-word refusal offered) searches line text, not names (call 19).
- That a count by `tab` names the parent for UniqueStash sub-tabs (call 14).
- That counts default to 20 buckets and say "left_out_needs view.counts.limit" (call 8).
- That page 2 of a trade search is requested by sending `next` whole: the search description says so; it worked (call 7).

## 5. The tool's own words
- **Instructions (call 1):** accurate and dense. Right: store tools need no daemon; "No tool says what an item is worth: a price is the owner's own listing." Missing: no word on tab-name prices or on listing state. "Begin with search with no arguments" is refused here (call 2), which the same sentence half-explains. Read and not used: the refresh slice (sync_policy, set_sync_policy, refresh_plan, apply_plan), the job tools, daemon_status, item_events, characters, tabs.
- **search description:** long but exact about `next`, `routes` and zero offers; "A zero total offers requests under zero.offers" held (call 9). Nothing in it was wrong; it says plainly that the answer is "over the items the store has fetched, priced or not", which was true.
- **describe:** one-name entries are small and useful (call 20).
- **Refusals:** call 2 `realm_needed` with a ready request per realm: good, enough to go on. Call 18 `bare_word`: clear, but its readings did not include the field I needed. Call 25 `unknown_name` ("--describe fields lists all 70; near it:" with `has:priced`): good and enough, though it speaks in terminal flags (`--describe`) in an MCP tool.
- **Zero page `said`:** "a line the search cannot name is shown as unknown, with what it knows of it, and never fuzzy-matched": right for call 21 (it suggested Fenumus' Weave), not relevant to call 9.
- **Words I did not understand or did not use:** `facts_version`, `derivation`, `unlifted_items`, `withheld_items`, `refused_bodies`, `beast_colours`, `list_seen`, `holds(...)>=1` (I took it as "at least one of the listed terms"), `q20`, `resolved … more`, `denominator`.
- **Price block:** `status none / kind none / why`: call 27's `why` explains itself; calls 11, 24 and 28 say only "nothing applies".

## 6. What you would ask of the tool
- Say per question how much of the asked scope is unfetched (call 21: unfetched UniqueStash tabs in the league).
- A field or note for "listed on the site" and "tab public", or say outright that it cannot know (call 26).
- Read tab-name prices (`~price N chaos`) as the tab's price, or say they are not read (call 27).
- On a refused bare word, offer `name:"…"` (call 18).
- On a zero, say which single term (or pair) empties it (call 9).
- Give `other` for every ambiguous clause, armour and map tier too (calls 9, 12, 16).
- A compact row mode (id, name, place, cell) without the repeated scope block (call 6).
- Make `store_status` agree with the league list (call 3).

## 7. Left out
- Not tried: tabs, characters, item_events, sync_policy, set_sync_policy, refresh_plan, apply_plan, submit_job and the other job tools, daemon_status, describe whole/fields/trade. I ran nothing that needs the daemon or changes state.
- Not read: the 69 extra rings the dex-less search adds in T1 (call 8 counted them by tab, 20 of 56 tabs shown); T4 rows 21–160 and which 27 items have 30+ (I gave tabs); T5's 510 items by place (counted by category); T6 rows 21–83 (counted by tier and tab); `name:weave` rows 21–32 (call 22).
- Files opened: only W/BRIEF.md and W/TASKS.md.
- Files written: the report through the door; drafts under W/drafts/: t1ring.json, t1next.json, t1nodex.json, t2.json, t2b.json, t2c.json, t3.json, t4.json, t4b.json, t5.json, t6.json, t6b.json, t7.json, t7b.json, t7c.json, t7d.json, t7e.json, t8a.json, t8b.json, t8c.json, t8d.json, rep8.md.
- The one cut I would most want back: the full list of the 69 extra rings for T1's "I don't care about the dexterity" (call 8 gave only a tab table), since the person asked what it adds.
