<!-- written during the run; call numbers are rows of calls.csv -->

## 1. Before touching anything

**From TOUR.md alone, before any command.**

What it can do (TOUR.md): log in to the game's API by OAuth (`acq auth`); fetch profile, leagues, characters, stash tab lists and single tabs through a daemon that "lazy-spawns" on first job use; submit/cancel/watch jobs; compile and apply a sync "plan" from a policy; keep everything in a local store that can be read with no daemon and no network (`tabs`, `store characters`, `search`, `show`, `store status|events|refused`); price items and render forum shop pages "offline, no daemon"; and run a trade-site search URL as a local query (`search --trade <URL|ID>`).

What it needs, for what (TOUR.md): a login (OAuth, keyring tokens) and the daemon and network for the "jobs against the API" block — `profile`, `leagues`, `characters`, `character`, `stashes`, `stash`, `refresh`, `submit`, `demo`. Nothing for the "store" block and for pricing/shop ("no daemon, no network"). `refresh --plan` is said to compile offline; `refresh --apply` executes requests.

What I would not run without asking the person: `auth` / `auth logout` (touches their keyring/session); anything that spawns a daemon against the *real* provider, which TOUR.md says is the default (`ACQ_PROVIDER=mock` is opt-in) — so `refresh`, `stash`, `stashes`, `characters`, `submit`, `demo`, `refresh --apply`; `policy set` (changes what syncs); `price set/clear` (changes their shop prices — undo exists, but it's their shop); `store import` / `store rebuild` (rewrites the store); `daemon stop|reset-tripwire` (TOUR calls those rails). In this world the door makes all of this safe, but these are the ones I'd ask about in a real seat. I'd also not run the tour's first line (`cargo build`) — it is a developer's line, not a user's, and the brief forbids building.

Unclear from TOUR.md: what the C-numbers (C10, C20, C38…) and "invariant 4" refer to — they point at documents I may not read; the tour leans on `AGENTS.md`, `CLI-REFERENCE.md`, `LIVE-TESTING.md`, skills, which a user handed only the binary would not have.

**The world as found (read-only commands, calls 1–6).** One account, `GERWARIC#7694`, "last login 3h ago", "not persisted", store 54.4 MB (call 2). The store holds 1133 responses, 65 characters, 3444 tabs over every realm and league, 22721 items (+6 removed), 22751 events, 4 refused bodies, league list never fetched (call 3). It is a two-realm store: pc and poe2 characters both appear (call 5), so `search` needs `--realm` unless a trade URL names it (call 6). Standard (pc): 3347 tabs, 405 fetched, 2926 never fetched, 16 folders; 20231 items in tabs (call 4); the big tabs were fetched "39d ago" (call 4). Characters: 65, 50 fetched (10 with empty bodies), 15 never fetched, 1389 items; pc characters fetched 36–40d ago, poe2 ones 45d ago (call 5). The search's own scope line says 22623 live pc items, 546 locations fetched "oldest 40d ago, newest 2d ago", 2940 locations never fetched (call 7) — the "newest 2d ago" is not visible in `tabs` or `store characters`, which say 36d at the newest. Most never-fetched locations are UniqueStash (437 of 502 unfetched) and MapStash (2489 of 2550) substashes (call 4). I changed nothing.

## 2. The tasks

### T1 — a ring

**What I would tell the person.** "Your friend's ring search — a ring with at least 50 total life, 20 total dexterity, and either 20% chaos res or 40% fire res — finds **22 rings of yours in Standard**. One is worn: *Death Circle Unset Ring* on **VVickedBossingServices**. Two are in your **MARKET** and **Lab Runners** tabs (*Pain Nail Diamond Ring*, MARKET x2 y1; *Hate Turn Amethyst Ring*, Lab Runners x7 y4), one in **Dump Tab** (*Storm Band Two-Stone Ring*, x20 y19) and one in **3.19 Death's Oath Occ** (*Apocalypse Nail Ruby Ring*). The other 17 are in old **Remove-only** tabs — e.g. three in *3.25 Siege Ballista* (best of them *Doom Circle Amethyst Ring*, 95 life / 54 dex / 22% chaos, x11 y10), *Fate Twirl Ruby Ring* in *3.25 RF Chieftain* (110 life / 35 dex / 84% fire, x6 y7), *Gloom Knot Two-Stone Ring* in *Eviscerate* (75 life / 50 dex / 34% chaos). **Would the site show you the same? No — it would show none of them.** None of the 22 carries a price the trade index can see; the one in the tab named `~price 50 chaos (G) (Remove-only)` reads 50 chaos but the tab is not public, so the site never sees it. The site only shows items in public tabs; your worn ring and your Remove-only tabs are never listed. The match itself is the site's: the tool calls the translation exact. Two caveats: about 2,900 of your tabs (mostly unique and map sub-tabs) have never been downloaded, and everything else was downloaded about 40 days ago."

"**Without the dexterity:** 91 rings instead of 22 — 69 more, spread over 56 tabs and 9 characters (2 on mrs_flowers; one each on AllEarsRacer, CameraPotatoCat, TheFireworksCompany, VVickedBossingServices, VVickedRanger, VVickedWitch, mAcRAmEsWeATer, undxot). Of the 91, only **3 would show up on the site**, all in public priced tabs: *Plague Turn Amethyst Ring* (67 life, 22% chaos, x8 y7) and *Bramble Whorl Amethyst Ring* (77.5 life, 20% chaos, x5 y7), both in `~price 2222 chaos`, and *Dusk Eye Amethyst Ring* (71 life, 23% chaos, x3 y6) in `~price 8888 chaos`."

**Commands:** 7 (the URL), 8 (rows 21–22 via `--next`), 9–10 (`--describe trade`, `--describe`), 11–12 (`price --help`, `price status`), 13 (the query + `has:priced` → 0), 14 (`price show` on the `~price 50 chaos` ring: "not public"), 15 (`--explain 3`: dex forced true → 91), 16 (counts by tab/character/price), 17 (the 3 priced).

**Right?** I hold the 22 and the 91 right as the store has them; the translation said "exact" (call 7) and every row shows the lines that met each term. What would tell me it is wrong: (a) the 2940 never-fetched locations (call 7) — a ring could be sitting in one; (b) the store is ~40 days old; (c) `pseudo.total_life` counts Strength as half a life (e.g. 87.5 on *Grim Grasp Diamond Ring*, call 7; 77.5 on *Bramble Whorl*, call 17) — I believe that is the site's rule too, but whether the site rounds that half I could not check. My "site shows none of the 22" rests on `has:priced` (call 13) plus `price show` saying "not public" (call 14); the search has no field for "in a public tab", so an unpriced item in a public tab — which the site *can* show with no price — I cannot rule in or out from the search. That is a real gap in my answer.

**Where the tool stopped or misled me.** The `--trade` header (call 7) says "the site shows public listings, so the search is the site's and the items are yours" — it never says which of *these* items are public or listed; I had to find `has:priced` in the index (call 10) and `price show` (call 14). The first page cut at 20 of 22 and the way on was a ~1.5 KB `--next` token (calls 7–8). `--describe trade` (32 KB) and `--describe` (47 KB) were over my harness's own display limit (not the door's 60 KB), which saved each to a file I then read — see Left out.

### T2 — boots

**What I would tell the person.** "Your friend's boots search — boots with at least 300 armour (as the site counts it, at 20% quality), 25% movement speed, 100% total elemental resistance and 75 total life — finds **nothing of yours in Standard**. No single filter is the whole problem; it's the combination. The closest you have:
- dropping the resistance filter, three armour boots qualify, but none has more than 32% elemental resistance: *Entropy League Vaal Greaves* worn by **mrs_flowers** (353 armour, 30% MS, 128 life, 32% fire res); *Anarchy Hoof Titan Greaves* worn by **BoomBoomBored** (414 armour, 30% MS, 107.5 life, 20% fire res); *Doom Sole Titan Greaves* in the Remove-only tab **30 chaos** at x6 y10 (322 armour at 20% quality, 28% MS, 83 life, 28% cold res).
- dropping the armour filter, two boots have everything else, but no armour at all: *Gale Spark Harpyskin Boots* in **Soulrend PF (Remove-only)** x7 y6 (124 life, 140% ele res, 30% MS) and *Ambush Spur Stealth Boots* in **3.19 Death's Oath Occ** x7 y6 (75 life, 101% ele res, 30% MS).
The site would have shown you nothing either. The tool says one filter, armour, is 'ambiguous': its 20%-quality armour figure can be off by one from the site's. None of your boots sits near 300, so here that changes nothing."

**Commands:** 18 (the URL: 0 matches, armour flagged ambiguous), 19–22 (`--explain` on terms 4, 2, 3, 5: forcing ele res true → 3, armour true → 2, movement speed or life true → 0), 23–24 (the near misses, sorted).

**Right?** I hold "none" right for the store as it stands. What would tell me otherwise: the 2940 never-fetched locations and the ~40-day age (call 18); the armour off-by-one (call 18) can't flip anything here, since the nearest armour values are 322 and none (calls 23–24). I did not check whether any of the five near misses has a price the site can see.

**Where the tool stopped or misled me.** Nowhere. The zero-match answer handed me the `--explain` commands to run, ready to copy (call 18), and they showed which filters were blocking. To see *which* items were close I had to rewrite the query by hand without the term (calls 23–24). The `forced true` route printed in call 19 would have done it, but it can't be sorted the way I needed.

### T3 — a shield

**What I would tell the person.** "Your friend's totem shield search — a shield with at least 100 armour at 20% quality, +1 to maximum number of Summoned Totems, and 100 total life — finds **one shield of yours: Tukohama's Fortress (Ebony Tower Shield)**, in the tab **Facebreaker Totems (Remove-only)**, x7 y3. It has 473 armour, 117 life (91 + 26 implicit) and the +1 totem. The site would **not** show it to you: that tab isn't public and the shield has no price. You have four other shields with the +1 totem that fail on life: *Rift Aegis Shagreen Tower Shield* (88 life) in **3.20 Shockwave Totem (Remove-only)** x8 y3, *Foe Mark Ironwood Buckler* (3 life) and *Entropy Star Titanium Spirit Shield* (no life), both in **>> 1 (Remove-only)**, and *Sol Watch Angelic Kite Shield* (no life) in **3.23 Power Siphon (Remove-only)**."

**Commands:** 25 (the URL: 1 match; armour flagged ambiguous, as in T2), 26 (`price show`: "reads none, not public"), 27 (every totem shield, sorted by life: 5).

**Right?** Yes, as the store holds it. A 473-armour shield is far from the one-point ambiguity on armour (call 25). Same caveats as before: 2940 never-fetched locations, ~40 days old.

**Where the tool stopped or misled me.** Nowhere.

### T4 — magic find

**What I would tell the person.** "Your friend's search — anything with at least 20% increased Rarity of Items found as an explicit modifier — finds **160 items of yours in Standard** (call 28). Best first (call 29): two **Andvarius** rings, 70% (in the unique tab `~price 4444 chaos (Remove-only)`) and 53% (in the unique tab `~price 4554 chaos`); two rare rings at 51%, *Empyrean Gyre Sapphire Ring* (Small Jewelry (Remove-only), x4 y2) and *Empyrean Eye Bone Ring* (`|||||| MARKET |||||| (Remove-only)`, x7 y4); **Aurseize** gloves at 50%, worn by VVickedWilliam; **Greed's Embrace** at 48% (`8 chaos (Remove-only)`, x18 y5); *Dire Torc Gold Amulet* 41%; *Tempest Ward Nightmare Bascinet* 40%; then a run of 39% down to 30%.

**At least 30%: 27 items** (call 30). Three are worn, by **BatteryStaple** (Presence of Chayula, 30%), **LordHelpTheMistress** (Doom Grip Moonstone Ring, 32%) and **VVickedWilliam** (Aurseize, 50%). The other 24 are in 17 tabs:
- `~price 4554 chaos` — 5 (Andvarius 53%, Reaper's Pursuit 38%, Wheel of the Stormsail 31%, Presence of Chayula 30%, Eye of Chayula 30%)
- `~price 4444 chaos (Remove-only)` — 2 (Andvarius 70%, Eye of Chayula 30%)
- `3.13 Golems, BL, Misc (Remove-only)` — 2 (two Presence of Chayula)
- `Small Jewelry (Remove-only)` — 2 (Empyrean Gyre ring 51%, Dire Torc amulet 41%)
- one each in `|||||| MARKET |||||| (Remove-only)`, `8 chaos (Remove-only)`, `Items 4 (Remove-only)`, `Special Gear (Remove-only)`, `Lab Bases #1 (Remove-only)`, `~price 40 chaos (F) (Remove-only)`, `Mercenary Items (Remove-only)`, `PConc (Remove-only)`, `Gear #2 (3.16)`, `3.12 Scion (Remove-only)`, `~price 20 chaos (A) (Remove-only)`, `~price 30 chaos (Remove-only)`, `Corrupted Uniques (Remove-only)` (call 31 has each item's x/y).
**The site would show only 5 of the 27**: the five in `~price 4554 chaos`, all priced 4554 chaos by that tab's name. Everything else is worn or in a tab that isn't public. (`~price 4444 chaos (Remove-only)` has a price in its name but has no price the site can see, call 30.) One caveat from the tool: it doesn't net 'reduced Rarity' against 'increased'. An item with both could be counted here and not on the site, or the other way round."

**Commands:** 28 (URL: 160), 29 (same, `--sort … --desc`: best 20), 30 (`>=30`, counts by tab/character/price: 27; 5 chaos-priced), 31 (the 27, all rows, best first).

**Right?** The 160 and the 27 I hold right for the store, with the usual two caveats (2940 locations never fetched; ~40 days old) plus the 'reduced' form the tool flags as ambiguous (call 28). "Best" here means the most rarity, nothing else; I didn't weigh the slot or the rest of the item. The 5 that the site would show are the 5 with a price currency (call 30). That they're the 5 in `~price 4554 chaos` is my inference from the rows in call 31; the count's routes would confirm it, but I didn't run them.

**Where the tool stopped or misled me.** (1) To sort "best first" I had to type the query's sum expression back as `--sort 'sum(line("#% increased Rarity of Items found" source=explicit).arg1)'` (call 29). The tool knew the trade search's own sort and only *reported* it; it offered no short way to say "sort by the value the search filtered on". (2) In the tab count (call 30) a unique tab's sub-tabs are rolled into the parent under its id (`52848d23bf`), but the rows (calls 29, 31) print `~price 4554 chaos / e63d0f7d9d` etc., the *sub-tab* id — so the ids in the two views don't match, and nothing says the one is the other's parent. (3) `~price 4444 chaos (Remove-only)` reads as priced to a person; only the price column (none) shows it isn't.

### T5 — fire resistance

**What I would tell the person.** "Your friend's fire resistance search — anything whose own fire resistance modifiers ('+#% to Fire Resistance', explicit or scourge, added up) reach at least 30% — finds **510 items of yours in Standard** (call 32). That's far too many to list here, so here's how they break down (call 34):
- by slot: 94 rings, 85 helmets, 60 body armours, 59 boots, 54 amulets, 54 belts, 47 gloves, 26 shields, 13 quivers, 18 weapons
- by place: 475 are in stash tabs (304 premium, 157 quad, 13 unique, 1 normal); 35 are on characters (32 worn, 2 in bags, 1 on a guardian)
The highest (call 39): four **Cloak of Flame** robes at 66–73% (one worn by BoomBoomBored; the others in Foulborn Icicle Mine, DD Chieftain and 3.25 RF Chieftain, all Remove-only); *Gloom Spiral Sapphire Ring* 57% and *Blight Locket Stygian Vise* 52%, both in **Build Gear 2**; *Vortex Knuckle Two-Stone Ring* 52% in Small Jewelry (Remove-only); two **Arn's Anguish** belts at 51–52%.

**The site would show 13 of the 510** (call 35), all priced by their tab's name: four rares in `~price 2222 chaos` / `~price 8888 chaos` (*Cataclysm Curtain Triumphant Lamellar* 45%, *Onslaught Keep Ezomyte Burgonet* 43%, *Vortex Trap Leather Belt* 43%, *Vengeance Veil Zodiac Leather* 42%), and nine uniques in the unique tab `~price 4554 chaos` (Chernobog's Pillar 50%, Frostbreath 48%, Sundance 36%, Cloak of Flame 33%, The Deep One's Hide 33%, Kintsugi, Heatshiver, Memory Vault, Dyadian Dawn at 30%).

Worth knowing about the search itself: it counts only modifiers that say exactly '+#% to Fire Resistance'. 'Fire and Cold Resistances', 'all Elemental Resistances' and implicits don't count. The site does the same, so the list is the same, but it isn't 'everything that gives fire res'."

**Commands:** 32 (URL: 510; scourge-in-sum flagged ambiguous), 33 (refused, see Faults), 34 (counts by category/container/tab type/price), 35 (the 13 priced), 36–38 (the scourge ambiguity checked both ways: 6 items carry a scourge fire line, none of them is in the 510, and none would get in without its scourge line), 39 (best 10).

**Right?** I hold 510 and 13 right for the store. I checked the flagged ambiguity, and it moves nothing (calls 37–38). Usual caveats: 2940 never-fetched locations, ~40 days old.

**Where the tool stopped or misled me.** I asked for `--count` and `--cross` in one call and was refused (call 33): "an answer has one view". The refusal was clear enough to go on. Otherwise the tool was fine; the scourge ambiguity was named precisely enough to test.

### T6 — maps

**What I would tell the person.** "Your friend's search is for blighted maps of tier 14 and up. You have **83** in Standard (call 40), in two tabs (call 41):

| tier | Blight Maps tab | Blight tab | total |
|---|---|---|---|
| 14 | 37 | 7 | **44** |
| 15 | 20 | 2 | **22** |
| 16 | 12 | 5 | **17** |

You also have **13 Blight-ravaged maps** at those tiers (2 × T14, 7 × T15, 4 × T16); 12 are in the Blight tab and 1 in Blight Maps (calls 42–43). The tool leaves them out of the search, but the site may count them as blighted, so it could show you up to 96 where the tool says 83. Either way **none of these maps has a price**, so the site would show you **none** of them in a buyer's search (call 43). Big caveat: your **map tab** has never been fully downloaded. Of its 2550 sections, only 61 have been fetched (call 4). Blighted maps sitting there are not counted here."

**Commands:** 40 (URL: 83; map tier and blighted flagged ambiguous), 41 (`--cross map_tier,tab`), 42 (the ambiguity: ravaged and untiered maps), 43 (blighted + ravaged by tab type, typeline, price: 96, none priced, none in a MapStash).

**Right?** The 83 and the per-tier counts I hold right for what was fetched. That the 12 ravaged maps are in the Blight tab is my arithmetic: BlightStash 26 = 14 blighted + 12 ravaged, and QuadStash 70 = 69 + 1 (calls 41, 43). What would tell me the count is low: the map tab's 2489 unfetched sections (call 4). The tool's scope line says "2940 locations were never fetched: nothing here says what they hold" (call 40), but it doesn't tie that to *maps* for a map search.

**Where the tool stopped or misled me.** The flagged "map tier read from the base name" ambiguity was clear. My call 42 query, `category=map -has:map_tier`, swept in 977 items (ultimatums, invitations, fragments), because `category=map` is the parent of fragments, scarabs and invitations. That's my mistake, but it was easy to make: the index says "a parent holds everything under it by the dot" (call 10), and I didn't read it as meaning `map` covers `map.fragment`.

### T7 — what I'd take

**What I would tell the person.** "If you're mapping in **Standard** tomorrow, here's what I'd hand you. (These are Standard items; they can't come with you into the new league.)
- **Ring — Storm Band, Two-Stone Ring**, in **Dump Tab**, x20 y19. Of the 22 rings from your friend's search, it has the most total resistance: 105% (call 44). It has +68 life, +41 dex, +47 energy shield, **24% increased item rarity**, +43% fire, +36% lightning, and +13% fire & cold from the implicit. It's item level 80 and needs level 59. It's identified, has no price, and Dump Tab isn't public, so nobody can see it on the site (calls 46–47). If you'd rather have life than spread resistances, take *Fate Twirl Ruby Ring* instead (3.25 RF Chieftain (Remove-only), x6 y7): 110 life, 35 dex, but mostly fire resistance (84%) (call 44).
- **Boots — Gale Spark, Harpyskin Boots**, in **Soulrend PF (Remove-only)**, x7 y6. Nothing of yours matched your friend's boot search. These miss only the armour (they're evasion boots), and otherwise they beat it: 124 life, 48/44/48% fire/cold/lightning, 30% movement speed, 586 evasion, 4 green linked sockets. They need **level 78 and 153 Dex** (call 49). If you need armour boots, *Entropy League Vaal Greaves* are on mrs_flowers already (353 armour, 128 life, but only 32% resistance) (call 23).
- **Shield — Tukohama's Fortress, Ebony Tower Shield**, in **Facebreaker Totems (Remove-only)**, x7 y3. It's the only match for the totem search. 473 armour, 25% block, +117 life, +1 totem, 40% increased totem damage, +300 armour per totem, 3 red linked sockets; needs level 70 and 159 Str. **Warning: it has Blood Magic**, so your skills cost life instead of mana. Make sure your totem build is set up for that before you equip it (call 48). If it isn't, the next totem shield is *Rift Aegis Shagreen Tower Shield* (88 life) in 3.20 Shockwave Totem (Remove-only), x8 y3 (call 27).
Remember all of this is from a download about 40 days old. If you've moved things since, they may not be where I say."

**Everything I know about the ring** (call 46, verbatim from the store): name *Storm Band*, base Two-Stone Ring, class Rings, category accessory.ring, rarity rare, identified, item level 80, requires level 59, place pc · Standard / Dump Tab (`inventoryId` Stash1) x20 y19, `verified: false`, no price; explicit lines +41 to Dexterity, +47 to maximum Energy Shield, +68 to maximum Life, 24% increased Rarity of Items found, +43% to Fire Resistance, +36% to Lightning Resistance; implicit +13% to Fire and Cold Resistances; "unread nothing". Listing (call 47): no hand price, no price in game, Dump Tab `5ba2e1880a` "reads none, not public". Totals the search computed: total res 105, fire 56, life 68, dex 41 (call 44).

**Commands:** 44 (the T1 search sorted by total res), 45 (`show --help`), 46 (`show --body`), 47 (`price show`), 48–49 (`show` on the shield and boots).

**Right?** The facts about each item I hold right as stored. The *choices* are my judgement, not the tool's: "best" mapping ring = most total resistance among the 22. What would make them wrong: the items have moved since the download (~40 days, call 7; the tool doesn't print an age for this one item); the person's build (Dex/Str requirements, Blood Magic); a better item in the 2940 never-fetched places.

**Where the tool stopped or misled me.** `show` (call 46) gave every line but **no age**: it doesn't say when this ring was last seen in Dump Tab, though `tabs` prints a "fetched" age per tab (call 4). For "where do I pick it up" that's the fact I most wanted. Also, `show --body` printed `"verified": false`, which I don't understand and the tool doesn't explain.

## 3. Faults

Worst first.

1. **"Would the site show me this?" has no answer in the search itself.** The `--trade` header says "the site shows public listings, so the search is the site's and the items are yours" (calls 7, 18, 25, 28, 32, 40), but no row says whether *that* item is public or listed. The search language has no field for "in a public tab" or "priced in game". `has:priced` (call 10) is the nearest, but by its own definition it also takes "the owner's row", a hand price the site never sees. The rows do print `price.side game` (calls 17, 35), but `price.side` isn't in the index's field list (call 10). Only `price show`, one item at a time, says "not public" (calls 14, 26, 47). I answered the person's main trust question by inference (calls 13, 17, 30, 35, 43). *A person would hit this too.* It is the one question the person asked in every task.
2. **No age on an item, and inconsistent ages around it.** `show` (call 46) and every search row give a place but not when it was last seen there. The scope line says locations were fetched "oldest 40d ago, newest 2d ago" (call 7), but `tabs` (call 4) and `store characters` (call 5) show nothing newer than 36d. I could not find what was fetched 2 days ago. For T7's "where do I pick it up" this is the fact that matters. *Person too.*
3. **A map search isn't warned that the map tab is mostly unfetched.** `tabs` shows MapStash 2550 sections, 61 fetched (call 4). The map search (call 40) prints only the generic "2940 locations were never fetched: nothing here says what they hold", the same line as for rings. The per-tier counts (call 41) could be badly low and nothing in the answer points at the map tab. *Person too.*
4. **Two different ids for a unique tab's sub-tabs.** Rows print `~price 4554 chaos / e63d0f7d9d` etc., the sub-tab's id (calls 29, 31, 35). `--count tab` puts the same items under the parent `52848d23bf` (call 30), and `price.from tab/pc/52848d23bf` (call 35). Nothing says the one id belongs to the other. *Person too.*
5. **No way to sort by what the search filtered on.** For "best first" I had to type `--sort 'sum(line("#% increased Rarity of Items found" source=explicit).arg1)'` back in by hand (calls 29, 35, 39). The tool reports the site's own sort and doesn't apply it, and it offers no short form. *Person too.*
6. **Refusal for `--count` with `--cross`** (call 33): "an answer has one view", on stderr, **exit 1**. The tour says "a refused command line" is `"kind":"usage"` with **exit 2**. I didn't pass `--json`, so this may be the human form's rule, but the tour doesn't say exit codes differ by form. The refusal itself was clear enough to go on. *Person too.*
7. **Help and tour are written for the developers.** The tour opens with `cargo build --workspace` and "rebuild before a run (AGENTS.md…)". Help texts cite C-numbers and S-numbers throughout (C81, C93, C106, S177…) that a user cannot look up. *Person too.*
8. **Pagination tokens of about 1.5 KB** (calls 7, 28, 29, 32, 39, 40, 44) to carry on a list of 22. It works (call 8), but it's heavy to copy. *Mostly an agent problem.*
9. **`--describe trade` (32 KB) and `--describe` (47 KB)** are the only full description of what a trade search becomes (calls 9, 10). Both fit the door's 60 KB, but they went over my harness's own display limit. *Agent only.*

## 4. What the tour and the help did not say

- **That the site's view is "priced in game on a public tab", and how to ask for it.** I expected it in `search --help`'s `--trade` text or in the trade header. I learned `has:priced` from the index (call 10), the "index cannot see" rule from `price status` (call 12), and "not public" from `price show` (call 14).
- **That `--sort` works beside `--trade`.** The help says "the site's sort is reported, never applied". It doesn't say my own `--sort` is accepted with a trade URL. I learned it by trying (call 29).
- **That `--count` and `--cross` can't be combined.** I expected it in `search --help`. Learned at call 33.
- **That `category=map` takes in fragments, scarabs and invitations.** The index says "a parent holds everything under it by the dot" (call 10); the effect only became plain at call 42.
- **Where an item's "last seen" time is.** I expected it in `show` (call 45 help, call 46 output). I didn't find it anywhere.
- **What `"verified": false` means** in the stored body (call 46). Nothing says.

## 5. The tool's own words

- **Right and useful:** the trade header's fidelity labels ("exact" / "ambiguous", each clause named with why) were precise enough to test (calls 18, 32, 36–38, 40, 42). The zero-match answer offering ready `--explain` commands (call 18), and `--explain`'s forced-true/false counts (calls 15, 19–22), were the best part of the tool. "2940 locations were never fetched: nothing here says what they hold" is honest. `price show`'s "reads 50 chaos, not public" (call 14) is exactly the right sentence.
- **Wrong or misleading:** the tour's first line (`cargo build … alias acq=./target/debug/acq`) is a developer's line, not a user's. "newest 2d ago" (call 7) agrees with no other view (calls 4, 5). "the search is the site's and the items are yours" (call 7) reads as if the site would show these items; mostly it would not (calls 13, 30, 35, 43).
- **Missing:** a field for public/listed; an item's age; tying never-fetched places to the search's category.
- **Read and never used:** `auth`, `accounts` beyond call 2, `profile`/`leagues`/`characters`/`stash(es)`/`refresh`/`submit`/`demo`/`jobs`/`cancel`/`dash`, `policy`, `refresh --plan/--apply`, `reference currency`, `price set/clear/list`, `shop render`, `store events/refused/import/rebuild`, `daemon`, `version`; and in search `--json`, `--routes`, `--membership`, `--context`, `--request`/`--print-request`, `--query-file`, `--trade-file`, `--sum`, `show --against`.
- **Words I did not understand:** "basis" and its parts (`snapshot 1133 · facts v8 · intent 33 · derivation 12`); every C/S number; "rails", "tripwire" (tour); "probe" (help, call 1); `listing_only` as a status name (call 9) — I took it to mean "only about the seller's listing, so not run"; `"verified": false` (call 46); "not persisted" in `accounts` (call 2).

## 6. What I would ask of the tool

- A search field for "visible on the trade site" (priced in game on a public tab), and a column in `--trade` rows saying so — calls 7, 13, 14.
- An item's last-seen age in `show` and in rows — call 46.
- A map (or any) search that names the never-fetched places of the matching tab type, e.g. "MapStash: 2489 sections unfetched" — calls 4, 40.
- `--sort` shorthand for "the value this search filters on" — call 29.
- One id for a unique tab's sub-tab in both rows and counts, or rows that print the parent too — calls 30, 31.
- Allow `--count` beside `--cross` in one answer — call 33.
- A short user-facing tour without build steps and C-numbers — TOUR.md, call 1.

## 7. Left out

- **Not tried:** any `--json`; `--routes`; `--membership all`; `tabs --expand` (would have dated Dump Tab, but it lists 3347 tabs); `price list`; `store events` (might explain "newest 2d ago"); `show --against`; any job or API verb (the world has no login or daemon, and I wouldn't run them without asking); the rows past the first 50 of T5's 510 (call 32) and T6's 83 (call 40). I used counts instead and didn't page.
- **Files opened that are not one of the three:** two files written by *my harness*, not the tool or the door: the saved stdout of call 9 (`--describe trade`, 32 KB) and call 10 (`--describe`, 47 KB), under `~/.claude/projects/-Users-tom-seat-worlds-trade-2026-10-cli-opus-2/…/tool-results/`. They were over the harness's display limit, which saved them there; reading them was the only way to see those outputs. I opened nothing else.
- **Work files written:** none (`W/work/` and `W/drafts/` unused).
- **The one cut I'd most want back:** checking when each item handed over in T7 was last seen where I said it is. It was a `tabs --expand` / `store events` look that I left out.
