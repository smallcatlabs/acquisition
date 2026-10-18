<!-- written during the run; call numbers are rows of calls.csv -->

# 1. Before touching anything

**T0, from what `open` printed (call 1) and nothing else.**

What the tool can do (call 1):
- Read a local SQLite store of a Path of Exile account's stash tabs and characters: `search` (a query language, also a pasted trade-site search via `{"trade": …}`), `show` (one item whole, optionally `against` a query and with its stored `body`), `describe` (the query language), and the inventory tools `accounts`, `tabs`, `characters`, `store_status`, `item_events`. The instructions say these "read the shared SQLite store directly — no daemon, no network".
- Plan a refresh: `sync_policy` (read), `set_sync_policy` (write intent, local, compare-and-swap on a revision), `refresh_plan` (offline compile of an action set + "coarse wire estimate"), `apply_plan` (spends it as one daemon job).
- Talk to a daemon: `submit_job`, `job_status`, `job_result`, `list_jobs`, `cancel_job`, `daemon_status`.
- It says outright it does not value items: "No tool says what an item is worth: a price is the owner's own listing."

What it needs, for what (call 1):
- Store tools: nothing but the store file. No login, no daemon, no network.
- `sync_policy`, `set_sync_policy`, `refresh_plan`: the store only ("offline", "sends nothing"); `refresh_plan` adds a quote if a daemon happens to run.
- `submit_job`, `apply_plan`, `cancel_job`, `job_result`, `job_status`: a daemon, and through it the game's API, so a login. Login is "done by the human via the `acq` CLI"; in "real-GGG mode" the server never starts a daemon. The instructions name the provider as `mock` here.
- `daemon_status`, `list_jobs`: say they observe only and fall back to the on-disk ledger.

What I would not run without asking the person first:
- `submit_job` and `apply_plan` — they spend the person's API rate limit (holds "can reach ~5 minutes") and change the store; `submit_job`'s params are "visible to every connected client".
- `set_sync_policy` — it overwrites the person's declared intent.
- `cancel_job` — it kills work someone else may have queued (it cascades).
- I would also ask before `show` with `body: true` on many items only out of size, not risk.
None of the tasks needs any of the first three; I plan to use only the store tools, `describe`, and the read-only `daemon_status`/`list_jobs`/`sync_policy` to look.

**T0, the world as found (calls 2–9, read-only).**
- One account: `GERWARIC#7694` (uuid cac319d8-…), provider `mock`, `persisted: false` (call 4).
- The store is `W/store/mock/GERWARIC_7694.db`, 54,382,592 bytes. It holds 22,721 items, 6 removed, 65 characters, 3,444 tab rows, 1,133 responses and 22,751 events, plus "refused_bodies: 4". It has `leagues: 0` (call 5).
- The store holds two realms, pc and poe2. A bare `search {}` is refused with `realm_needed`, and its offers are one request per realm (call 2). The pc realm has **22,623 live items** (call 3), so about 98 are poe2 (22,721 − 22,623; my arithmetic, not a printed figure).
- In pc, items exist in only two leagues: **Standard** has 20,231 in stash tabs (`container=items`), 814 equipped, 125 in inventories, 123 in jewel sockets and 18 under `guardian`. **Allflame** has 1,101 + 152 + 40 + 14 + 5 (call 7). The scope also lists Ancestors, Hardcore, Phrecia 2.0, SSF and a Ziz gauntlet league, which have no items (call 3).
- How old: stash fetches run from 1788810111 (2026-09-07) to 1792074048 (2026-10-15 14:20 UTC). The snapshot is at 1792074278 (2026-10-15 14:24 UTC) (call 3). The scope says `fetched: 546`, `never_fetched: 2940`, `folders: 17` (call 3). So most of the 3,444 tab rows were **never fetched**, and their contents are not in any answer. Standard has 41 characters (call 8). Their listing was seen on 2026-09-11, and most were fetched on 2026-09-08. Several show `fetched_items` well below `item_count` (e.g. OnlyCircles 33 of 66), and nothing told me why.
- The `tabs` answer for Standard alone was **745,672 bytes** (call 9). The door cut it to its head, so I did not see the whole tab list. The tool has no limit, filter or page for it. In the head, "Dump Tab" and "Currency 1" were fetched on 2026-10-15. The others I saw (Keepers, Currency 2–4, Blight, Delirium, Delve, Divination, Essence…) were fetched on 2026-09-08.
- Nothing was changed: no job, no policy write.

What I'd tell the person: "Your Standard collection here is about 20k stash items plus what 41 characters wear. Most tabs were last read in early September, two were read on Oct 15, and roughly 2,900 tab entries were never read at all. So 'you don't have it' only means 'not in what was fetched'."

# 2. The tasks

## T1 — a ring

**What I'd tell the person:**
"The search asks for a Standard ring with 50+ life, 20+ dex, and either 20%+ chaos res or 40%+ fire res. The tool translated it with nothing left over (`fidelity: exact`, empty `remainder`) (call 11). **You have 22 such rings** (call 11).
- 21 are in stash tabs and 1 is worn. The worn one is the rare 'Death Circle' Unset Ring on VVickedBossingServices (57 life, 28 dex, 40% fire) (calls 11, 12).
- The stash ones are in 17 tabs (call 12). '3.25 Siege Ballista (Remove-only)' has 3. '3.25 RF Chieftain (Remove-only)' and 'KB-CF Champion (Remove-only)' have 2 each. These have 1 each: '3.13 Self-curse (Remove-only)', '3.19 Death's Oath Occ', '3.21 RF Jugg (Remove-only)', '50 chaos (Remove-only)', '>> 0 (Remove-only)', '>> 2 (Remove-only)', 'Breach Rings (Remove-only)', 'Dump Tab', 'Eviscerate (Remove-only)', 'Lab Runners', 'MARKET', 'Mercenary Items (Remove-only)', 'Small Jewelry (Remove-only)' and '~price 50 chaos (G) (Remove-only)'.
- Two examples: 'Pain Nail' Diamond Ring in MARKET (56 life, 45 dex, 25% chaos), and 'Hate Turn' Amethyst Ring in Lab Runners (58 crafted life, 35 dex, 20% implicit chaos) (call 11).
- **Without the dexterity requirement, 91 rings match**, 69 more (call 15). 81 are in the stash and 10 are worn, across 9 characters (mrs_flowers wears 2) (call 16).
- **Would the site show you the same?** The search's terms match the same way: every clause is `exact` (call 11). But the site only shows items listed in public stash tabs. **None of these 22 has a price on it** (call 14: every one under `-has:price.currency`). One is on a character, which the site never shows. Most are in tabs named '(Remove-only)'; from what I know of the game, not from the tool, those cannot be public. So on the site you would most likely see none or almost none of these. The tool can't tell me which of your tabs are public."

**Calls:** 10 (its output was too large for my harness to show inline, so I did not read it; see Left out), then 11, 12, 13, 14, 15 and 16. That is seven calls; three would have done (11, 12 and 15).

**Right?** I hold the 22 and the 91 to be what the store says. The tool showed the matched lines on three rows, and they meet the bounds (call 11). It would be wrong if the store is stale or incomplete. 2,940 tab entries were never fetched (call 3), and most fetched tabs date from 2026-09-08. One item has no `category` at all (`lacked: 1`, call 11). If that is a ring, it is missed silently.

**Where the tool stopped or misled me:**
- `fidelity: exact` is about how the clauses translate. It says nothing about what the site would show. The tool never says that the site sees only public, listed stash items and the store sees everything, including worn items (calls 11, 13). `describe ["trade"]` (call 13) lists `status`, `trade_filters.*` and `sale_type` as `listing_only`, but it never states the difference in the item sets.
- Dropping a term was easy with `view.explain` on the term's path (call 15). Nothing pointed me there for "what if I drop X"; I inferred it from the `search` text.
- Every answer repeats the full `terms` block and the full `basis`, about 6–7 KB, even in a counts view (calls 12, 14, 16). Three rows plus the trade report came to 13.5 KB (call 11), and the default 20 rows came to 31 KB (call 10).

## T2 — boots

**What I'd tell the person:**
"The boots search asks for Standard boots with 300+ armour (counted at 20% quality), 25%+ movement speed, 100%+ total elemental res and 75+ life. **You have none** (call 17).
- The translation lost nothing (`remainder: []`). The tool marks it `fidelity: ambiguous` because of the armour clause. The site computes armour at 20% quality from rounded numbers, and the tool says its own figure is 'within one' of the site's (calls 17, 13). That only matters for boots right at 300.
- **Elemental resistance is what rules your boots out.** Ignore it and 3 pairs meet the rest (call 18). Ignore movement speed instead and you still get none (call 19). The three, best resistance first (call 20):
  - 'Entropy League' Vaal Greaves, worn by **mrs_flowers**: 353 armour, 30% move speed, 128 life, but only 32% elemental res.
  - 'Doom Sole' Titan Greaves in stash tab **'30 chaos (Remove-only)'**: 322 armour at 20% quality (268 as it sits), 28% move speed, 83 life, 28% res.
  - 'Anarchy Hoof' Titan Greaves, worn by **BoomBoomBored**: 414 armour, 30% move speed, about 107 life (97, plus 21 strength counted as 10.5 life), 20% res.
- On the site: you'd see nothing either, since nothing matches. Two of the three near-misses are worn, and the site never shows worn gear."

**Calls:** 17, 18, 19 and 20 (four). The zero answer offered an `explain` per term (call 17), and two of them found the blocker.

**Right?** "None" is what the fetched store says, with the same staleness and never-fetched caveat as T1 (call 3). The near-miss numbers come from the tool's own shown lines (call 20). One thing could make it wrong: the 100% bound is a sum over fire, cold and lightning res. The tool's `pseudo.total_ele_res` follows the site's totals table (call 13, `totals`), so I trust it. But I did not open any item to check that the sum includes 'all elemental resistances' lines.

**Where the tool stopped or misled me:** Nowhere, really. The `zero.offers.explain` list is ordered with the decisive term first (call 17), which was the right one to try. One small point: the sort row for 'Entropy League' shows `total_ele_res: 32` but names only the crafted fire line (call 20), so I could not see from the row whether anything else counted.

## T3 — a shield

**What I'd tell the person:**
"The shield search asks for a Standard shield with 100+ armour (counted at 20% quality), an explicit '+# to maximum number of Summoned Totems' line, and 100+ life. **You have exactly one: Tukohama's Fortress**, a unique Ebony Tower Shield, in stash tab **'Facebreaker Totems (Remove-only)'** (calls 21, 22).
- It has 473 armour at 20% quality, +1 maximum Summoned Totems, and 117 life (91 explicit plus 26 implicit) (call 22).
- 26 of your items carry the explicit totem line, but this is the only shield among them that also meets the armour and life bounds (call 21).
- On the site: the clauses translate fully (`remainder: []`). The answer is marked `ambiguous` only for the armour rounding, and with 473 against a 100 floor that can't matter (call 21). The site would show this shield only if that tab were public and the shield listed. Its tab is a '(Remove-only)' tab, and the tool says nothing about whether it is public."

**Calls:** 21 and 22 (two).

**Right?** Yes, as far as the fetched store goes. The matched lines are printed and meet every bound (call 22). The same never-fetched caveat as T0 applies (call 3).

**Where the tool stopped or misled me:** It didn't. The one thing I could not ask is whether the tab is public.

## T4 — magic find

**What I'd tell the person:**
"Your friend's search is any Standard item whose explicit 'increased Rarity of Items found' adds up to 20%+, with no item type set. **You have 160** (call 23): 69 rings, 49 helmets, 34 amulets, 2 chests, 2 gloves, 2 shields, 1 boots and 1 two-handed axe. 150 are in the stash and 10 are worn (call 23).

**At least 30% (explicit, as the site counts it): 27 items** (call 24), best first (calls 25, 26):
- 70% Andvarius (unique Gold Ring): sub-tab of '~price 4444 chaos (Remove-only)'
- 53% Andvarius: sub-tab of '~price 4554 chaos'
- 51% Empyrean Gyre (rare Sapphire Ring): 'Small Jewelry (Remove-only)'
- 51% Empyrean Eye (rare Bone Ring): '|||||| MARKET |||||| (Remove-only)'
- 50% Aurseize (unique gloves): **worn by VVickedWilliam**
- 48% Greed's Embrace: '8 chaos (Remove-only)'
- 41% Dire Torc (rare amulet): 'Small Jewelry (Remove-only)'
- 40% Tempest Ward (rare helmet): 'Items 4 (Remove-only)'
- 39% Ventor's Gamble: 'Special Gear (Remove-only)'
- 39% Fate Halo (rare helmet): 'Lab Bases #1 (Remove-only)'
- 38% Reaper's Pursuit (axe): sub-tab of '~price 4554 chaos'
- 33% Mind Locket (talisman): '~price 40 chaos (F) (Remove-only)'
- 32% Doom Grip (rare ring): **worn by LordHelpTheMistress**
- 32% Ventor's Gamble: 'Mercenary Items (Remove-only)'
- 31% Ventor's Gamble: 'PConc (Remove-only)'
- 31% Wheel of the Stormsail: sub-tab of '~price 4554 chaos'
- 31% Greed's Embrace: 'Gear #2 (3.16)'
- 30%, nine items:
  - Presence of Chayula: **worn by BatteryStaple**
  - Presence of Chayula: sub-tab of '~price 4554 chaos'
  - Presence of Chayula ×2: '3.13 Golems, BL, Misc (Remove-only)'
  - Presence of Chayula: '3.12 Scion (Remove-only)'
  - Eye of Chayula: sub-tab of '~price 4554 chaos'
  - Eye of Chayula: sub-tab of '~price 4444 chaos (Remove-only)'
  - Eye of Chayula: 'Corrupted Uniques (Remove-only)'
  - Wheel of the Stormsail: '~price 20 chaos (A) (Remove-only)'
  - Le Heup of All: '~price 30 chaos (Remove-only)'

By tab (call 24): '~price 4554 chaos' (with its sub-tabs) holds 5. '3.13 Golems, BL, Misc', 'Small Jewelry' and '~price 4444 chaos' hold 2 each. 13 other tabs hold 1 each. 3 are worn (call 24).

**If you count implicit rarity too** (what you'd actually get in a map), **33 items reach 30%**, 6 more (calls 29, 30):
- Ventor's Gamble at 26+13=39%: '>> 0 (Remove-only)'
- a magic Gold Ring of Excavation at 24+15=39%: '5 chaos (Remove-only)'
- The Highwayman at 25+12=37%: '3.22 Items (Remove-only)'
- The Highwayman at 37%: 'Unique Rings'
- Ghoul Medallion at 16+16=32%: '8 chaos (Remove-only)'
- The Highwayman at 30%: 'Unique Rings'

The best one also has more than it shows above: the 70% Andvarius carries another 8% implicit (call 28).

On the site: the search uses the site's explicit stat. The tool marks it `ambiguous` because the site also folds in 'reduced Rarity' lines, which the tool does not subtract (call 23). Of the 27, only the **5 in '~price 4554 chaos' carry a price** (4554 chaos each), so the site could list only those, if that tab is public (call 27). The tabs named '~price 4444 chaos (Remove-only)', '~price 30 chaos (Remove-only)' and so on carry no price in the tool. It says 'the price text in game is not where the index can see it (C81)' (call 28). The 3 worn items never show on the site."

**Calls:** 23 to 30 (eight). 23 to 26 answer the question as asked. 27 and 28 are for the site comparison, and 29 and 30 for the implicit reading.

**Right?** The 160 and the 27 follow the friend's search, explicit only. 'Best ones first' is by that explicit sum, and each row shows the line (calls 25, 26). The 33 relies on a name I guessed, `pseudo.increased_rarity`. Its rows add explicit and implicit as I expect (call 30). 'At least 30' meaning explicit-only is my reading of the person, so I gave both answers.

**Where the tool stopped or misled me:**
- A substash's place prints with a blank or near-blank `name` ("" or " (Remove-only)"), and its real label sits only in `parent.name` (calls 25, 26). A reader taking `place.name` would tell the person 'tab ""'.
- In counts, the `tab` key folds substashes into their parent (call 24 gives '~price 4554 chaos' 5). The rows give the substash (calls 25, 26). The two answers name places differently.
- The total-rarity pseudo is not named in any text I was shown. I found `pseudo.increased_rarity` by guessing (call 29).
- The heredoc form of `call` was blocked by my harness, not the tool, so I sent `next` inline (call 26).

## T5 — fire resistance

**What I'd tell the person:**
"This search is a weighted sum: explicit fire res plus scourge fire res, totalling 30%+, any item, Standard. **You have 510 items** (call 31).
- By type: 94 rings, 85 helmets, 60 chests, 59 boots, 54 amulets, 54 belts, 47 gloves, 26 shields, 13 quivers and a few weapons (call 31).
- 475 are in the stash, 32 are worn, 2 are in character inventories and 1 is in a `guardian` slot (call 31).
- They are spread over **141 tabs** (call 33). The biggest are '~price 40 chaos (F) (Remove-only)' with 37, 'Dump Tab' 13, 'Gear (Remove-only)' 13, 'Rings (Breach)' 12, 'Items 4 (Remove-only)' 11, and 9 each in 'Breach Rings (Remove-only)', 'Decent Gear', '~price 30 chaos (B) (Remove-only)' and '~price 4554 chaos'. 20 characters wear some; BoomBoomBored and LordHelpTheMister wear 3 each (call 33).
- **What the site would not count, and neither does this search:** fire res from '+% to all Elemental Resistances', from 'Fire and Cold'-type hybrid lines, or from implicits. If you count all of those, **83 more items** reach 30% fire (call 32), mostly rings (39), helmets (13) and shields (13).
- The tool marks this search `ambiguous` for one reason. It **adds** scourge fire res into the sum, while the site was seen to leave a scourge line out in one case (call 31). So an item with scourge fire res might be counted differently here than on the site."

**Calls:** 31, 32 and 33 (three).

**Right?** The 510 is the friend's search as the tool reads it. The 83 rests on `pseudo.total_fire_res`, which the tool says follows the site's own pseudo totals (call 13, `totals`). I did not list the 510 one by one. It would take about 26 pages of rows, and the person needs the shape more than the list.

**Where the tool stopped or misled me:**
- The counts table holds 10 tabs by default and says `left_out: 131`, `left_out_needs: "--limit"` (call 33). That names the terminal flag, not the MCP field (`view.counts.limit`). The `search` text does map the flag names, so I could follow it.
- `container=guardian` is never explained anywhere I could read (calls 7, 31).

## T6 — maps

**What I'd tell the person:**
"The map search asks for Standard maps of tier 14 or higher whose type starts with 'Blighted'. **You have 83** (call 34):
- **Tier 14: 44**
- **Tier 15: 22**
- **Tier 16: 17**

They are in two tabs: **'Blight Maps' holds 69 and 'Blight' (your Blight stash tab) holds 14** (call 34).

You also have **13 Blight-ravaged maps** at tier 14+, which this search does not include: 2 at T14, 7 at T15 and 4 at T16. 12 are in 'Blight' and 1 is in 'Blight Maps' (call 35). Whether the site's 'Blighted Map' filter takes those in, the tool doesn't know; it lists Blight-ravaged as its own unsupported filter (call 13).

The tool marks the tier part `ambiguous` (call 34). It reads a map's tier from the name, e.g. 'Blighted Map (Tier 14)', and unique or guardian maps whose name has no tier get none, where the site would give them one. For blighted maps every name carries the tier, so I don't expect that to matter here. Below tier 14 you hold more blighted maps, e.g. 63 at T13 (call 34, `resolved`)."

**Calls:** 34 and 35 (two).

**Right?** The counts are what the store holds, and tier per map is read straight from the type line (call 34). The 'Blight' tab was last fetched on 2026-09-08 (call 9's head). I don't know when 'Blight Maps' was fetched, because call 9's head was cut before it.

**Where the tool stopped or misled me:** Nowhere for the count. The `resolved` block (call 34) volunteered the tier spread below 14, which was helpful. The Blight-ravaged question I had to raise myself. The translation only hints at it through the unsupported row in `describe ["trade"]` (call 13). The answer to this search does not mention that Blight-ravaged maps exist.

## T7 — what I'd take

**What I'd tell the person:**
"For mapping in Standard tomorrow, I'd hand you these three.

- **Ring: 'Storm Band', a rare Two-Stone Ring, in your 'Dump Tab'** (a quad tab), column 20, row 19 counting from 0 (calls 36, 37). Among your 22 rings that fit your friend's search, it has the most total resistance (105 by the site's way of counting) (call 36). It's also the most freshly seen: last read 2026-10-15 (call 37).
- **Boots: 'Entropy League', rare Vaal Greaves, worn by your character mrs_flowers.** Log into mrs_flowers and take them off the Boots slot (calls 20, 39). None of your boots meet your friend's 100% resistance bar (T2). These are the best of the three that meet everything else: 353 armour, 30% move speed, 128 life, +32% fire (crafted) and +26% chaos. They also have Searing Exarch and Eater of Worlds implicits ('searing', 'tangled') and 3 green + 1 red linked sockets (call 39). They were last read 2026-09-08 (call 39, `last_seen` 1788884457), so check that mrs_flowers still wears them.
- **Shield: Tukohama's Fortress**, unique Ebony Tower Shield, in tab **'Facebreaker Totems (Remove-only)'**, column 7, row 3 (call 38). It is the only shield that matches (T3). Know what you're taking: it has **Blood Magic**, so your skills cost life, not mana. It gives +1 totem, 40% increased totem damage, +300 armour per summoned totem, 117 life, 473 armour at 20% quality and 25% block, with 3 red linked sockets. It needs level 70 and 159 strength (call 38). It was last read 2026-09-08 (call 38, `last_seen` 1788886624).

**Everything I know about the ring** (call 37):
- Rare Two-Stone Ring, 'Storm Band', item level 80, identified, not corrupted (its flags list only 'identified').
- Requires level 59.
- Implicit: +13% to Fire and Cold Resistances.
- Explicit: +41 Dexterity; +47 maximum Energy Shield; +68 maximum Life; 24% increased Rarity of Items found; +43% Fire Resistance; +36% Lightning Resistance.
- So: 68 life, 41 dex, 56% fire, 13% cold, 36% lightning, 0% chaos, 47 energy shield and 24% rarity (a nice bonus for mapping).
- Why it fits your friend's search: life 68 ≥ 50, dex 41 ≥ 20, and fire 56 ≥ 40. It has no chaos res, but the search needs only one of chaos or fire (call 37).
- It is in 'Dump Tab' at x 20, y 19. First seen by the tool on 2026-08-30 and last seen on 2026-10-15 at 14:20 UTC (call 37).
- **No price** is set on it, and no note (call 37). The tool never values items, so I can't tell you what it's worth.
- What the site would say: it would show this ring only if 'Dump Tab' were public and the ring listed. With no price, I doubt it is."

**Calls:** 36 to 39 (four), building on T1–T3.

**Right?** The ring, boots and shield are the store's facts. The choice is mine: most total res for the ring, and for the boots the best of the three near-misses (which also have the most life). Someone with a different build in mind might pick 'Fate Twirl' (110 life, 84% fire, in '3.25 RF Chieftain (Remove-only)') instead (call 36). The boots and shield were last seen on 2026-09-08, so they may have moved since. The tool cannot say without a refresh, which needs the daemon and a login. I did not try one.

**Where the tool stopped or misled me:**
- 'Where do I pick it up' needed the stored body (`body: true`) for x/y in the tab (calls 37, 38). The derived `place` has no position.
- In `against`, the group node's outcome is `"match"` while every other node's is `"matched"` (call 37). It's minor, but they are two words for one state.
- The boots' level requirement prints as 'Level 70' and `reqlevel: 70`. The body marks it `"suffix":"(gem)"`, meaning the level comes from a socketed gem, not the boots (call 39). The derived fields drop that suffix. I did not test whether a `reqlevel` search would treat it as the item's own requirement.

# 3. Faults

Worst first.

1. **"Would the site show me the same?" gets a confident wrong answer.**
   - What it said: `search` says of `fidelity` that it is "partial where the remainder is not empty, the answer then not the site's" (call 1). That implies an `exact` answer *is* the site's. T1 came back `fidelity: exact` (call 11) with 22 rings. One of them is worn by a character, which the site never shows. None is priced (call 14), and nearly all sit in '(Remove-only)' tabs.
   - What it should have done: say plainly, in the answer, that fidelity covers the translation of clauses, not the item set. The site sees only listed items in public stash tabs; the store holds worn, inventory and private items too. Ideally it would also split the total by what the site could and could not see.
   - Who hits it: both. A person reading 'exact' would believe it as readily as an agent. `describe ["trade"]` marks `status`, `trade_filters.*` and `sale_type` as `listing_only`, but never says what that does to the result (call 13).
   - Calls: 1, 11, 13, 14.

2. **Answers too big to read, with no way to shrink some of them.**
   - `tabs` for one league returned 745,672 bytes (call 9). The tool has no limit, filter or page for it, and the door cut it to the head, so I never saw the whole tab list. That is why I could not give fetch dates for 'Blight Maps' or 'Facebreaker Totems'.
   - Every `search` answer repeats the full `terms` block and `basis`, about 6 KB, even in counts and explain views (calls 12, 14, 15).
   - The default 20 rows of a trade search came to 31 KB (call 10). My harness diverted that into a file I may not open, so call 10 went unread.
   - Who hits it: the agent mostly. A person on a terminal scrolls, but would also face the 745 KB `tabs` answer.

3. **Substash places print a blank name.**
   - Rows give `place.name: ""` or `" (Remove-only)"`, with the real label only in `parent.name` (calls 25, 26, 28).
   - Counts by `tab` fold substashes into the parent instead (call 24 gives '~price 4554 chaos' 5). So rows and counts name the same item's place two ways.
   - Who hits it: both. A person sees an unnamed tab.

4. **"You have none" is never qualified by what was never fetched.**
   - The scope prints `never_fetched: 2940` beside `fetched: 546` on every answer (call 3). No answer turns that into a warning, a `zero` note or a caveat on the total.
   - The units are also unclear: 546 + 2940 = 3,486, but `store_status` says `tabs: 3444`, `characters: 65` (call 5).
   - Who hits it: both.

5. **Tool texts cite private references a reader can't follow.**
   - Examples: `C81`, `C96`, `C101`, `S26`, `t55`, `u065`, `K6`, 'a sitting', 'the park', `decisions/trade.md`, `tests/trade-oracle.toml`, and the owner's verbatim quotes.
   - They appear in the instructions (call 1: "C14"), in every trade clause note (calls 11, 13) and in a price refusal reason (call 28: "the price text in game is not where the index can see it (C81)").
   - Who hits it: both. They read as noise at best.

6. **The count-table overflow names a terminal flag:** `left_out_needs: "--limit"` (call 33). The MCP field is `view.counts.limit`. The `search` text does map flags to fields, so I could follow it. A person on the CLI would be fine; this one only hits an agent.

7. **The level requirement loses its '(gem)' suffix.** The boots' body says `Level 70` with `"suffix":"(gem)"`, but the derived `requirements` and `reqlevel` read a plain 70 (call 39). I did not test a `reqlevel` search, so this is possible, not shown. Who hits it: both.

8. **Two words for one outcome.** `against` gives `"match"` for the `holds` group and `"matched"` for every other node (call 37). Minor. Who hits it: both.

**Refusals:**
- Call 2 (`realm_needed`): its offers, sent back as printed, worked at once (call 3). Enough.
- Call 6 (`realm` is not a count key): it listed every key it knows. Enough to go on, though `realm` appears in `scope` and in every place, so it's a natural thing to try.

# 4. What the instructions and the tool texts did not say

- **That the site shows only listed items in public stash tabs.** I expected it in the instructions' trade sentence or in `describe ["trade"]`. I learned it from what I already know of the game; the tool never confirmed it (calls 1, 13, 14).
- **That `realm` is not a counts key.** I expected it in `search`'s view description. I learned it from call 6.
- **That a substash's label is only in `place.parent.name`.** I expected it in `search`'s row description ("place by name and id"). I learned it at calls 25 and 28.
- **That a total-rarity pseudo exists, and its name `pseudo.increased_rarity`.** I expected it among names `describe` lists, but the computed section is "over 100 KB", so I guessed instead. Learned at call 29.
- **That an item's position in its tab (x, y) is only in the raw body.** I expected it in `show`'s `place`. Learned at call 37.
- **What `fetched` / `never_fetched` / `folders` count, and what `fetched_items` vs `item_count` mean on a character** (OnlyCircles: 33 of 66). Expected in `search`'s `scope` text and in `characters`' text. Never learned (calls 3, 8).
- **What `container=guardian` is.** Never learned (calls 7, 31).
- **When a tab-name price ('~price X chaos') counts.** It counts for '~price 4554 chaos' (call 27). It does not for the '(Remove-only)' ones (calls 27, 28). The reason given was 'C81'.
- **That `view.explain` on a term's `path` is how to answer "what if I drop this term".** The `search` text does describe `explain`, so this one was there. I'm recording that it worked well (calls 15, 18).

# 5. The tool's own words

**Right:**
- The realm refusal and its offers behaved exactly as the instructions said (calls 2, 3).
- "No tool says what an item is worth: a price is the owner's own listing" was true and saved me from inventing values.
- Every count holding the request for its members worked as printed (call 26's `next`).
- The `zero.offers.explain` list on a zero answer was the most useful thing in the tool (call 17).
- The `search` text's map from terminal flags to `view` fields made `--limit` intelligible.

**Wrong or misleading:**
- `fidelity` in `search`'s text: "partial where the remainder is not empty, the answer then not the site's". Read with the instructions' "translated to the text a person would type", it suggests an exact answer is the site's set (Fault 1).

**Missing:**
- Site visibility (public, listed, worn).
- Never-fetched caveats.
- Substash naming.
- Any size warning on `tabs`.

**Read and never used:** `refresh_plan`, `apply_plan`, `sync_policy`, `set_sync_policy`, `submit_job`, `job_status`, `job_result`, `list_jobs`, `cancel_job`, `daemon_status`, `item_events`; and `search`'s `context`, `count`/`counted_at` and `membership` options.

**Words I did not understand:**
- References: C14, C45, C69, C81, C83, C84, C94, C96, C101, C106, S26, S47, S57, K5, K6, t37, u061.
- Terms of art: 'sitting', 'park', 'census', 'rails state', 'artifact_relation'.
- In `basis`: 'intent', 'facts_version', `derivation`.
- In the trade report: `forms`, `catalogue`.
- 'listing_only' is a status word whose meaning I could only infer.

# 6. What you would ask of the tool

- Say in every trade answer which matches the site could not show (worn, inventory, unlisted, private tab), and give the count (calls 11, 14).
- Rename or redefine `fidelity` so 'exact' cannot be read as 'the site's result' (call 11).
- Add `limit`, `offset` and a `name` filter to `tabs` (call 9).
- Add a `terse` switch that drops the repeated `terms` and `basis` from views that don't need them (calls 12, 14, 15).
- Print a substash's place as 'Parent / Child', never a blank name (calls 25, 28).
- Warn on a zero or low total when `never_fetched` is large (calls 3, 17).
- Put x/y in `place` (call 37).
- Name the overflow field in MCP terms, `view.counts.limit`, not `--limit` (call 33).
- Allow `describe` of one computed family, e.g. `rarity`, so a pseudo can be found without guessing (call 29).
- Drop the private reference codes from user-facing texts, or link them to text a reader can open (calls 1, 13, 28).

# 7. Left out

**What I did not try:**
- Any refresh: no `refresh_plan`, `sync_policy`, `daemon_status`, `list_jobs` or job tool. So I cannot say what bringing the never-fetched tabs or the September-old characters up to date would cost.
- `item_events`.
- `describe` with `fields`, `computed`, `counts` or a single name.
- Listing T5's 510 items or T4's 160 one by one.
- The single item with no `category` (`lacked: 1`, call 11), to see whether it is a ring, boots or shield that every search missed.
- A `reqlevel` search to test Fault 7.
- Any Allflame or poe2 item.

**Files opened:** none besides `BRIEF.md` and `W/TASKS.md`.
- Call 10's answer was diverted by my harness into a file outside W. I did not open it, so call 10 is unread.
- I ran `date -u -r` three times to turn the tool's epoch seconds into dates. That is not a call against the tool or the store.
- One breach of the rules: call 2 went out with W held in a shell variable (`$W`) rather than written in full. Every later command spelled it out.
- One `call` sent with a heredoc was refused by my harness before reaching the door. It is not a numbered call; I re-sent it inline as call 26.

**The one thing I cut that I'd most want put back:** the fetch dates of the tabs my answers point at ('Blight Maps', 'Facebreaker Totems (Remove-only)', 'Lab Runners', 'MARKET'). They were lost when `tabs` (call 9) was cut at its head. Without them I cannot tell the person how old 'it's in this tab' is.
