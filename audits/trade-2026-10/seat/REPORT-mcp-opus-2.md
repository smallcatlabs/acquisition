<!-- written during the run; call numbers are rows of calls.csv -->

## 1. Before touching anything

From what `open` printed (call 1) alone:

**What it can do.** Search every item the local store holds with a query language (`search`, language printed by `describe`), show one item whole (`show`), and translate a pasted trade-site search (URL, id or query JSON) into that language with no network (`search` with `{"query":{"trade":…}}`), reporting per clause whether it was exact, ambiguous, unsupported, listing_only or disabled and a `fidelity`. It lists accounts, tabs, characters, store size (`store_status`) and recent item events. It never prices: "a price is the owner's own listing". Separately it can declare a sync policy, compile it into a refresh plan, and apply that plan through a daemon; and submit/poll/cancel raw jobs.

**What it needs.**
- No login, daemon or network: `search`, `describe`, `show`, `accounts`, `tabs`, `characters`, `store_status`, `item_events`, `sync_policy`, `refresh_plan` (offline; a running daemon only adds a quote), `daemon_status`/`list_jobs` (observe only, fall back to the ledger on disk).
- Local write, no network: `set_sync_policy` (compare-and-swap on revision).
- Daemon (and through it the game's API, i.e. network and a login done by the human with the `acq` CLI): `submit_job`, `job_status`, `job_result`, `cancel_job`, `apply_plan`. In real-GGG mode the server never spawns a daemon. This world is `provider: mock`.

**What I would not run without asking the person.** `set_sync_policy` (it replaces their declared intent), `apply_plan` and `submit_job` (spend their API rate limit and rewrite the store from the game), `cancel_job` (kills their work). None is needed for these tasks, which are all reads; I will run none of them.

Then the world as found — see below (calls after 1).

**The world as found (calls 2–10, nothing changed).** One account, `GERWARIC#7694`, provider `mock` (call 7). Store file `store/mock/GERWARIC_7694.db` under W, 54,382,592 bytes, 22,721 items (6 removed), 65 characters, 3,444 tabs (call 6). It holds two realms, pc and poe2: a bare `search {}` is refused with `realm_needed` and two offers (call 2), as the instructions said it would be. In realm pc: 22,623 live items over 7 leagues, 546 fetches, oldest fetch 2026-09-07 19:41 UTC, newest 2026-10-15 14:20 UTC, and `never_fetched: 2940` — listed tabs/pages/characters no fetch has filled (call 3). Standard holds 21,311 of them (call 9): 20,231 in stash tabs and 814 equipped, 125 in inventories, 123 socketed jewels, 18 "guardian" on characters (call 5). By tab type the biggest are Premium 6,032, Quad 4,814, Gem 2,559, Normal 2,415, Flask 2,169 (call 9). 41 Standard characters are listed, many fetched 2026-09-08; several show `fetched_items` well below `item_count` (e.g. OnlyCircles 33 of 66) and some `fetched_items: 0` (call 8). The `tabs` listing for Standard was 745,672 bytes and the door cut it to its head (call 10), so I could not see which tabs are the never-fetched ones — that cut cost me knowing where the store is blind. The poe2 realm I did not search (only its 6 characters in call 8).

What this means for every answer below: the store sees only what was fetched, as of mid-September to mid-October 2026; 2,940 listed containers have never been read, so "you have none" can only ever mean "none in what was fetched".

## 2. The tasks

### T1 — a ring

**What I'd tell the person.** "Your friend's ring search asks for a Standard ring with at least 50 life, at least 20 dexterity, and either 20% chaos or 40% fire resistance. You have 22 rings that pass. The best for life: *Fate Twirl* (Ruby Ring, 110 life, 35 dex, 84% fire res) in tab '3.25 RF Chieftain (Remove-only)' at x 6, y 7; *Doom Circle* (Amethyst, 95 life, 54 dex, 22% chaos) in '3.25 Siege Ballista (Remove-only)' x 11, y 10; *Grim Grasp* (Diamond, 87.5 life counting its 23 strength, 54 dex, 20% chaos) in '>> 2 (Remove-only)' x 8, y 3. The other 19 run from 75 life down to 50, spread over 17 tabs, and one, *Death Circle*, is on your character VVickedBossingServices. The tab with the most is '3.25 Siege Ballista (Remove-only)', with 3.
If you don't care about dexterity, you get 69 more rings, 91 in all. Several of them have much more life: *Honour Hold* (Sapphire, 140 counting 54 strength, 42% fire) in 'Cem Stuff 1 (Remove-only)' x 6, y 2; *Empyrean Knot* (Amethyst, 115, 21% chaos) in 'Impending Doom (Remove-only)' x 6, y 6; *Corpse Band* (Two-Stone, 112, 30% chaos) in 'Winter Orb (Remove-only)' x 0, y 7; *Woe Hold* (Bone Ring, 110, 30% chaos), which mrs_flowers is wearing.
Would the site have shown you these? The translation of your friend's search is exact: every clause ran, nothing was left out. But the site lists only items you have put up for sale, and none of these 22 has a price the tool can see. So the site would most likely show none of them. The ring that wearing character has on is never on the site. 'Life' here is the site's own total, which adds half your strength, so the numbers should match what the site would print."

**Calls.** 11 (the URL as pasted; my harness cut its 30 KB print, so I could not read it), 12 (the same as counts: translation, `fidelity: exact`, `remainder: []`, total 22, by tab and character), 13–14 (the 22 rows over two pages, sorted by life), 15 (none of the 22 has `price.currency`; none has a note), 16 (`describe` of price fields), 17 (525 Standard items do carry a price), 18 (`show` of Dire Twirl), 19 (without dex: 69 more, first 15 rows). Total for T1: 9 calls.

**Do I hold it right?** The 22 are right as far as the store sees them. Each row shows the lines behind every term, and the totals add up by hand (Grim Grasp 76 + 23/2 = 87.5, call 13). The 69 come from my own edited query: the trade clauses minus dex, with `-pseudo.total_dex>=20` added so that only the new rings count. It ran over the same store (call 19). Two things would show it is wrong. First, rings in the 2,940 never-fetched containers (call 3). Second, the site's pseudo life could be built differently from this one, which follows "the C++ app's pseudomod tables" (call 4).

**Where the tool stopped or misled me.** On "would the site show the same", the tool's own words stop at "the site shows public listings" (call 1, `search.query`). It has no field for whether a tab is public, and none for whether a Remove-only tab can be listed. Most of these tabs are Remove-only. One ring sits in a tab named "~price 50 chaos (G) (Remove-only)", and another tab is named "1 divine (Remove-only)" (call 19). PoE prices a tab's contents by a name like that, yet `show` says price `none`: "nothing applies: the price text in game is not where the index can see it (C81)" (call 18). That sentence does not tell me whether the tab name was read and rejected (because the tab is Remove-only?) or never read. So "the site would show none" is my inference, not something the tool stated.

### T2 — boots

**What I'd tell the person.** "This search asks for Standard boots with at least 300 armour (as it would be at 20% quality), 25% movement speed, 100% total elemental resistance and 75 life. You own no boots that pass, in anything the tool has read. The closest are three pairs that have everything except the resistance. *Entropy League* (Vaal Greaves, 353 armour, 30% speed, 128 life, but only 32% resistance) is worn by mrs_flowers. *Doom Sole* (Titan Greaves, 322 armour at 20% quality, 28% speed, 83 life, 28% resistance) is in tab '30 chaos (Remove-only)' at x 6, y 10. *Anarchy Hoof* (Titan Greaves, 414 armour, 30% speed, 107.5 life, 20% resistance) is worn by BoomBoomBored. Two other pairs have the speed, the resistance and the life but no armour at all, because they are evasion bases: a pair of Harpyskin Boots in tab '3.19 Death's Oath Occ' and a pair of Stealth Boots in 'Soulrend PF (Remove-only)'. The site would find nothing of yours either. One part of the search, the armour floor, the tool can match only to within one point of the site's figure, but no pair of yours is near 300, so that cannot change the answer."

**Calls.** 20 (URL as pasted, as counts: total 0, `fidelity: ambiguous` on `armour_filters.ar` only, `remainder: []`), 21 (explain offer for the resistance term, sent as printed: forced true gives 3), 22 (those 3, sorted by total elemental resistance), 23 (my own text without the armour term: 2 pairs). 4 calls.

**Do I hold it right?** Yes, for the store as fetched. The zero is the product of terms whose counts each look sane (400 boots, 137 items with 100% resistance; call 20). The explain answer and my own query agree on which pairs miss by one term. It would be wrong if boots sit in the 2,940 never-fetched containers (call 3).

**Where the tool stopped or misled me.** Nowhere that mattered. The zero came with ready explain offers, and they were the fastest route to "what nearly qualifies" (calls 20–21). One quirk: the `sort` block on a row shows a single line, e.g. "+32% to Fire Resistance" with value 32 (call 22). I cannot tell from that whether 32 is the whole total or only one line of it.

### T3 — a shield

**What I'd tell the person.** "The search asks for a Standard shield with at least 100 armour (as it would be at 20% quality), +1 to maximum number of Summoned Totems, and 100 life. You have exactly one: **Tukohama's Fortress**, a unique Ebony Tower Shield with 473 armour at 20% quality, +1 totem and 117 life (91 + 26 implicit). It is in tab 'Facebreaker Totems (Remove-only)' at x 7, y 3. You own four other shields with the extra totem, but none has the life. *Rift Aegis* (Shagreen Tower Shield, 88 life) is in '3.20 Shockwave Totem (Remove-only)' x 8, y 3. *Foe Mark* (Ironwood Buckler, 3 life) and *Entropy Star* (Titanium Spirit Shield, no life) are in '>> 1 (Remove-only)'. *Sol Watch* (Angelic Kite Shield, no life) is in '3.23 Power Siphon (Remove-only)'. The site's version of this search would also turn up only the Fortress, and only if it were listed for sale. The armour floor is matched to within one point of the site's figure, and the Fortress is far above it."

**Calls.** 24 (URL as pasted, as counts: total 1, `fidelity: ambiguous` on the armour clause only, `remainder: []`), 25 (the tab bucket's request sent back: the row), 26 (my own text without the life and armour terms: 5 totem shields). 3 calls.

**Do I hold it right?** Yes. Every term shows the line or number behind it (call 25). Only 26 live items carry the totem line as explicit, and 5 of them are Standard shields (calls 24, 26). It would be wrong if the site's stat `explicit.stat_429867172` also matched a wording this build files under another template. The tool says the line term counts "explicit modifiers, crafted and fractured ones among them, as the site counts them" (call 24), and I have no way to check that.

**Where the tool stopped or misled me.** It did not stop me. In call 26, rows with no life line show `sort: {"status":"no satisfying occurrence"}`. That reads oddly, but the meaning is clear enough: the item has no life to sort by.

### T4 — magic find

**What I'd tell the person.** "Your friend's search is simply 'at least 20% increased Rarity of Items found as an explicit mod', on anything in Standard. You have 160 such items: 69 rings, 49 helmets, 34 amulets, and a few chests, gloves, shields, one pair of boots and one axe. 10 of them are on characters and the rest are spread over 66 tabs.
At 30% or more there are 27, best first:
- **Andvarius** (Gold Ring) 70%: in the Unique tab '~price 4444 chaos (Remove-only)', x 2, y 0
- **Andvarius** 53%: Unique tab '~price 4554 chaos', x 2, y 0
- *Empyrean Gyre* (Sapphire Ring) 51%: 'Small Jewelry (Remove-only)', x 4, y 2
- *Empyrean Eye* (Bone Ring) 51%: '|||||| MARKET |||||| (Remove-only)', x 7, y 4
- **Aurseize** 50%: worn by VVickedWilliam
- **Greed's Embrace** 48%: '8 chaos (Remove-only)', x 18, y 5
- *Dire Torc* (Gold Amulet) 41%: 'Small Jewelry (Remove-only)', x 11, y 11
- *Tempest Ward* (Nightmare Bascinet) 40%: 'Items 4 (Remove-only)', x 8, y 9
- **Ventor's Gamble** 39%: 'Special Gear (Remove-only)', x 0, y 1
- *Fate Halo* (Royal Burgonet) 39%: 'Lab Bases #1 (Remove-only)', x 8, y 12
- **Reaper's Pursuit** 38%: '~price 4554 chaos'
- *Mind Locket* 33%: '~price 40 chaos (F) (Remove-only)'
- *Doom Grip* 32%: worn by LordHelpTheMistress
- **Ventor's Gamble** 32%: 'Mercenary Items (Remove-only)'
- 31%: Ventor's Gamble in 'PConc (Remove-only)', Wheel of the Stormsail in '~price 4554 chaos', Greed's Embrace in 'Gear #2 (3.16)'
- nine more at exactly 30%: Presence/Eye of Chayula ×7, a Wheel of the Stormsail and Le Heup of All

By tab: '~price 4554 chaos' holds 5 of them. '3.13 Golems, BL, Misc (Remove-only)', 'Small Jewelry (Remove-only)' and '~price 4444 chaos (Remove-only)' hold 2 each. 13 other tabs hold 1 each, and 3 are on characters: VVickedWilliam, LordHelpTheMistress and BatteryStaple.
On the site, only the 5 in '~price 4554 chaos' are priced, at 4554 chaos each, and the tool can read that price. The other 22 have no price it can see, and most sit in Remove-only tabs, which the tool does not price even when the tab name says '~price'. The tool also counts only the explicit mod. Both Andvarius rings have a few percent more as an implicit (8% and 6%) that this search ignores, and the site ignores it too. One more difference: the site counts 'reduced Rarity' against the total, and the tool does not."

**Calls.** 27 (URL as pasted, as counts: total 160, `fidelity: ambiguous`: "the bound counts the wording given alone, where the site counts every form… not its forms '#% reduced Rarity of Items found'"), 28 (bound raised to 30: 27, by tab), 29 (the 27 sorted by the same `sum(...)` the term reads), 30 (price × tab: 5 priced at 4554, 22 with none), 31–32 (`show` of both Andvarius rings: Remove-only parent "price text in game is not where the index can see it (C81)", versus `"from":"tab/pc/52848d23bf"`, "4554 chaos in game"). 6 calls.

**Do I hold it right?** Yes for the 27, and the order checks out row by row (call 29). For an item with both increased and reduced rarity lines, the tool's number would be higher than the site's (call 27). I did not search for such items.

**Where the tool stopped or misled me.** First, the tab counts in call 28 name the *parent* tab ('~price 4554 chaos', 5). The rows in call 29 have `place.name: null` with a `parent`, because Unique-tab items sit in sub-tabs. A reader of the rows alone gets no tab name. Second, the reason a Remove-only `~price` tab gives no price is "not where the index can see it (C81)" (call 31). I only worked out that this means 'Remove-only' by comparing it with call 32.

### T5 — fire resistance

**What I'd tell the person.** "This search is a 'weighted sum' of at least 30% from the explicit (or scourge) '+#% to Fire Resistance' mod, on anything. That's broad: you have 510 Standard items that pass. By type: 94 rings, 85 helmets, 60 chests, 59 boots, 54 amulets, 54 belts, 47 gloves, 26 shields, 13 quivers, and a handful of weapons. The highest are four Cloak of Flame chests (73% in 'Foulborn Icicle Mine (Remove-only)' x 5, y 3; 72% worn by BoomBoomBored; 70% in 'DD Chieftain (Remove-only)'; 66% in '3.25 RF Chieftain (Remove-only)'). After them come *Gloom Spiral* (Sapphire Ring, 57%) and *Blight Locket* (Stygian Vise, 52%), both in 'Build Gear 2', then *Vortex Knuckle* (Two-Stone, 52%) in 'Small Jewelry (Remove-only)'. Note that the search counts only the plain fire resistance mod. A ring with '+40% to all Elemental Resistances' or '+20% to Fire and Cold' does not count towards the 30, the same as on the site. Only 13 of the 510 carry a price, so on the site you would see at most those 13. The scourge part, which the site may not count, changes nothing for you: no item gets over 30 only because of a scourge line."

**Calls.** 33 (URL as pasted, as counts: 510, `fidelity: ambiguous` on `stats.1.1` "A scourge modifier in a weighted sum is counted here; the site may leave it out", 13 priced in chaos), 34 (matches that are under 30 on the explicit line alone: 0, so the ambiguity costs nothing here), 35 (top 10 by the sum). 3 calls.

**Do I hold it right?** Yes. The two counts agree, the scourge test is 0 (call 34), and every row shows its line (call 35). I did not list all 510 or the 13 priced. The 510 seemed better given as types plus the top of the list.

**Where the tool stopped or misled me.** Nowhere. The ambiguity was stated where it lived, and I could test it with one query. The sum's name in `matched[].shows[].value.name` is the whole expression text, which is long but exact.

### T6 — maps

**What I'd tell the person.** "The search is blighted maps of tier 14 or higher in Standard. You have 83: **44 at tier 14, 22 at tier 15, 17 at tier 16**. 69 are in your quad tab 'Blight Maps' and 14 in your Blight tab 'Blight'. Each is a single map, not a stack. You also have **13 Blight-ravaged maps** (2 at tier 14, 7 at 15, 4 at 16). The site's 'Blighted' filter may include those, but this search leaves them out, so on the site the count could be as high as 96. Below the cut you have 63 more blighted maps at tier 13. None of these maps has a price the tool can see in the tier 14–16 set, so the site would most likely list none of them."

That last sentence is weaker than the others. I counted prices for T1, T4 and T5 but not for the maps. What I actually know is that call 36 does not show a price table, so "none priced" is unverified, and I would not tell the person that part.

**Calls.** 36 (URL as pasted, as counts by tier, tab and base: 83, `fidelity: ambiguous` on both the tier clause, "a map whose base names no tier has none here, where the site gives it one", and the blighted clause, "a Blight-ravaged map is outside it, and the site may count one"), 37 (`--sum stack`: `lacking` for all 83, so each item is one map), 38 (Blight-ravaged: 13, by tier), 39 (the Map tab: 488 maps, 47 with no tier), 40 (`^Blighted` items lacking a tier: 4, none a map: a Delirium Orb, an Incubator, an Idol, a Scouting Report). 5 calls.

**Do I hold it right?** The 44/22/17 are right for the store. I tested both stated ambiguities (calls 38, 40). Only the Blight-ravaged one can move the site's count. What would show the count wrong: 2,940 containers were never fetched (call 3), and a Map tab's sub-pages could be among them. The tool offers no view that says which of them are map pages, and I could not read the 745 KB tab listing (call 10).

**Where the tool stopped or misled me.** First, `--sum stack` answered `value: 0, lacking: 83` (call 37). I read that as 'no item has a stack size, so each is one', which is not quite the same as 'each is a stack of 1'. Second, the blighted term `typeline~"^Blighted "` also matches non-maps (call 40). Only the tier term keeps a 'Blighted Incubator' out, so if someone drops the tier clause the result fills with non-maps.

*T6, correction:* I checked the price claim in call 41. None of the 83 has a price, so "the site would most likely list none of them" now rests on that call and is no longer unverified. T6 took 6 calls in total (36–41).

### T7 — what I'd take

**What I'd tell the person.** "Here's what I'd hand you. All three are in your stash, so you don't have to take anything off a character.
- **Ring: Fate Twirl**, Ruby Ring, in tab '3.25 RF Chieftain (Remove-only)' at x 6, y 7. It is the best ring for life that meets your friend's search in full (calls 13, 43).
- **Boots: Gale Spark**, Harpyskin Boots, in tab 'Soulrend PF (Remove-only)' at x 7, y 6: 30% movement speed, 124 life, and 140% elemental resistance (48 fire, 44 cold, 48 lightning). It misses your friend's search only on armour, because it is an evasion base. If you want armour, the best pair that has it is *Entropy League* (Vaal Greaves, 353 armour, 128 life, 30% speed, but only 32% resistance), which mrs_flowers is wearing (calls 22, 42).
- **Shield: Tukohama's Fortress**, Ebony Tower Shield, in tab 'Facebreaker Totems (Remove-only)' at x 7, y 3: +1 totem, 117 life, 473 armour at 20% quality. It is the only shield that meets the totem search (call 25).

**Everything the tool knows about the ring:** a Rare *Ruby Ring* named Fate Twirl, item level 75, requires level 64, identified, no note and no price. Its mods:
- implicit +26% to Fire Resistance
- explicit +35 to Dexterity, +110 to maximum Life, +44 to maximum Mana, +43% to Fire Resistance
- crafted +15% to Fire and Chaos Resistances
- enchant: 'Your Meteor Towers create Burning Ground for 3 seconds on Hit'

That gives totals of 110 life, 35 dex, 84% fire resistance and 15% chaos. It passes your friend's search on the fire side; the chaos side (15 of 20) fails, which is allowed because the search needs only one of the two. It sits in a 12-wide premium tab ('Stash1' in the game's own data) at x 6, y 7. The tool first and last saw it in the same fetch, on 2026-09-08 17:15 UTC, so if you've moved it since then, it won't know."

**Calls.** 42 (the two boots that meet all but armour), 43 (`show` with `body` and `against` the T1 URL: `outcome: match`, each node, the stored body verbatim). Also reused from earlier tasks: 13, 22, 25. 2 new calls.

**Do I hold it right?** The three locations and the ring's mods are as the store holds them, from a fetch more than a month old (call 43: `last_seen` 1788887707). The choice is mine, made on life, resistance and speed. The tool states no worth (call 1, "No tool says what an item is worth"). If the person has rearranged their stash since September, the cells are wrong. A refresh would need a login and a daemon, and I did not offer to run one.

**Where the tool stopped or misled me.** It didn't. One small thing: the ring's `price.why` is "nothing applies" (call 43), while Dire Twirl in a '~price…(Remove-only)' tab gets "nothing applies: the price text in game is not where the index can see it (C81)" (call 18). The difference is the useful part, but I had to work it out myself.

*T7, correction:* "a 12-wide premium tab" in the ring paragraph is my own game knowledge. The tool did not say it. Call 43 gives only `tab_type: PremiumStash`, `inventoryId: "Stash1"` and x 6, y 7.

## 3. Faults

Worst first.

1. **The person's own question, "would the site show me the same", has no direct answer, and the price reason that bears on it is opaque.** The tool says only that "the site shows public listings" (call 1, `search.query`). It exposes no field for whether a tab is public or Remove-only. A '~price' tab name is read as a price in a normal tab (call 32: `"from":"tab/pc/52848d23bf"`, "4554 chaos in game"). In a Remove-only tab it is ignored, and the only reason given is "nothing applies: the price text in game is not where the index can see it (C81)" (calls 18, 31). I learned what that means only by comparing two Andvarius rings (calls 30–32). Before that, my T1 answer on site visibility was a guess. What it should say: "in a Remove-only tab, which cannot be public", plus a `tab.public` / `tab.remove_only` field to count by. A person would hit this too.
2. **The store's blind spot cannot be listed.** `scope.never_fetched: 2940` is in every answer (call 3), but nothing says which tabs, pages or characters those are. The one tool that might show them, `tabs`, answers 745,672 bytes for Standard with no filter or limit, and the door cut it to its head (call 10). So every "you have none" (T2 boots, the T6 count) carries an uncertainty I could not size. A person would hit this too; a host UI might render 745 KB, but no one reads it.
3. **Every search answer is heavy, even a counts view.** `basis`, `scope`, and for each term four counts each carrying a full request are repeated on every call. With `trade`, every clause comes back as well. Call 11 (20 rows of rings) was 30,696 bytes, and my harness cut its display at about 30 KB, so I had to repeat it as counts (call 12) and as text (calls 13–14). A count-only answer still ran 16,742 bytes (call 12). This mostly hurts an agent; a person behind a host UI is less affected.
4. **Rows in a Unique tab do not say what tab they are in.** Rows show `place.name: null` with a `parent` (call 29). The tab counts name the parent ('~price 4554 chaos', call 28). A reader of rows alone gets "null". It hits both.
5. **`--sum stack` reports single items as `lacking`** (call 37: `value 0, lacking 83`). "How many maps" from a sum is then 0 unless you know that an absent stack means 1. It hits both.
6. **The blighted clause's translation `typeline~"^Blighted "` matches non-maps**: a Blighted Delirium Orb, an Incubator, an Idol and a Scouting Report (call 40). The tier term saves the friend's search, but an edit that drops it would not be saved. It hits both.
7. **The `sort` block on a row can show one line for a total**: "+32% to Fire Resistance", value 32, sorted by `pseudo.total_ele_res` (call 22). That leaves open whether 32 is the whole total. Minor; it hits both.

Not the tool's fault: my harness refused the door's heredoc `-` form for `call` (at what would have been call 13), and I passed the JSON inline instead. No refusal from the tool cost me anything. The one refusal, `realm_needed` (call 2), came with offers that worked as printed (call 3).

## 4. What the instructions and the tool texts did not say

- **A '~price' tab name is a price, except in a Remove-only tab.** I expected it under `search.query` ("a price is the owner's own listing", call 1) or in `describe price.amount` ("the number the effective price states", call 16). I learned it in calls 30–32.
- **`sort` takes a `sum(line(...).arg1)` expression.** `search.view` lists "a field…, a computed value… or a line's number". I tried the sum and it worked in call 29.
- **Unique-tab items sit in sub-tabs with `name: null`, and the counts name the parent.** I expected this under `search` `rows… place by name and id`. I learned it in calls 28–29.
- **An item with no `stack` is one item, and `--sum` counts it as lacking.** I expected it under `describe counts` `--sum` ("an item lacking it adds nothing"), which is literally true but does not warn about this. I learned it in call 37.
- **What `item_count` versus `fetched_items` means on a character** (e.g. 66 vs 33, call 8). It is never stated, and I never learned it.
- **What `folders: 17` in `scope` is** (call 3). Never stated, never learned.
- **What the codes C10, C14, C45, C68, C81, C83, C84 and C96 refer to** (calls 1, 2, 16, 18). Never stated, never learned.

## 5. The tool's own words

**Right, and it carried me:** the instructions on beginning with `search {}` and the realm refusal with offers (calls 2–3). The `search.query` text on `trade` and `fidelity`, and above all each clause's `notes`. Every ambiguity the tool claimed, I could test in one query (scourge: call 34; tierless maps: call 40; Blight-ravaged: call 38). "No tool says what an item is worth" kept me honest in T7. The zero answer's `explain` offers (calls 20–21) were exactly what I needed.

**Wrong, missing or misleading:** `price.why` "not where the index can see it (C81)" (Fault 1). `tabs` says nothing about its size (Fault 2). `search` says "Rows past the limit are counted", but nothing warns that the answer around the rows is 10–20 KB (Fault 3).

**Read and never used:** `accounts` beyond one look, `item_events`, `daemon_status`, `list_jobs`, `submit_job`, `job_status`, `job_result`, `cancel_job`, `sync_policy`, `set_sync_policy`, `refresh_plan`, `apply_plan`, and `search`'s `count`, `counted_at`, `context` and `scope.membership`.

**Words I did not understand:** the C-numbers above; "the refresh slice"; "rails state (with the journal path)"; `basis` fields `derivation`, `intent`, `facts_version`, `totals`; "artifact_relation"; `denominator: "scope"` (I took it to mean "counted over all of scope"); "forms" in `query.trade.forms: 1`.

## 6. What you would ask of the tool

- A `tab.public` / `tab.remove_only` field and a plain `price.why` for Remove-only tabs — calls 18, 31.
- `tabs` with `limit`/`name`/`unfetched` filters, or `never_fetched` listed by name — calls 3, 10.
- A lean view: rows or counts without the per-term requests and the repeated `trade` clauses — calls 11, 12.
- `place.name` for a sub-tab item that names the visible tab (parent / child) — call 29.
- `--sum stack` that counts an absent stack as 1, or a `count_units` key — call 37.
- The blighted translation scoped to maps, e.g. `category=map typeline~"^Blighted "` — call 40.
- A short legend for the C-numbers in the instructions — calls 1, 18.

## 7. Left out

- **Not tried:** the poe2 realm. The remaining 54 rows of T1 without dex (only the first 15 are named). Listing all 510 T5 items or the 13 priced ones. `item_events` (e.g. whether the chosen ring moved). Any job, sync or refresh tool: none was needed, and each would have needed the person's say.
- **Files opened besides BRIEF.md and TASKS.md:** none. My harness saved call 11's full answer to a file of its own, and I did not open it. I also ran `date -u -r` three times to turn store timestamps into dates; that touched neither the tool nor its store.
- **The one cut I'd most want back:** the `tabs` listing for Standard (call 10, cut to 3,000 characters by the door). With it I could have said which of the 2,940 never-fetched containers are map or ring tabs, and so how far T2's zero and T6's count can be trusted.
