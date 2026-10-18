<!-- written during the run; call numbers are rows of calls.csv -->

## Before touching anything

### T0, from TOUR.md alone (no command run yet)

What the tool can do, per the tour: log in to the game's API by OAuth (`acq auth`), list accounts; run jobs against the API through a daemon that "lazy-spawns" (`profile`, `leagues`, `characters`, `character`, `stashes`, `stash`, `refresh`, `submit`, `demo`); manage a sync policy and compile/apply refresh plans; price items and render a forum shop ("offline, no daemon (C64)"); and read the local store with no daemon and no network (`tabs`, `store characters`, `search` including `--trade <URL|ID>` which runs "a trade site search (PoE1) as the query a person would type, run and labelled clause by clause", `show`, `store status|events|refused`, `store import|rebuild`).

What it needs, per the tour:
- Login + daemon + network: the "jobs against the API" block (the daemon lazy-spawns; the real provider is the default unless `ACQ_PROVIDER=mock`), `refresh --apply`, `auth`.
- Offline/no daemon: `policy`, `refresh --plan` ("compilation and reads are offline"), `reference`, `price *`, `shop render`, and the whole "store" block.
- `accounts` and `auth status` say they work with no daemon.

What I would not run without asking the person: anything that spawns a daemon against the real provider or spends API requests (`refresh --apply`, `refresh --tabs/--all`, `submit`, `stash`, `character(s)`, `stashes`, `auth`, `auth logout`); anything that changes their store or their prices/listings (`price set`, `price clear`, `policy set`, `store import`, `store rebuild`); `daemon stop`/`reset-tripwire`. In this world the brief says writes and prices are safe, but my tasks are all reads, so I don't need any of these.

Tour lines that don't apply to me: the first line says to `cargo build` and alias to `./target/debug/acq` and refers to AGENTS.md — I'm forbidden to build, and reach the binary only through the door. It also cites `CLI-REFERENCE.md`, "the mock-session skill", `LIVE-TESTING.md`, `RUN-LEDGER.md` and `C`-numbers (C11, C20, C38…) that I cannot read.

### T0, from the tool (calls 1–6)

- `acq --help` (call 1) agrees with the tour on what needs the API: `auth`, `profile`, `characters`, `character`, `leagues`, `stashes`, `stash`, `refresh`, `submit`, `demo`; and marks `accounts`, `tabs`, `search`, `show`, `store`, `price`, `reference`, `shop` as "no daemon". Note: help line 1 says "the real GGG API by default" — a job verb run carelessly would go to the real API outside this world.
- Whose: one account, `GERWARIC#7694`, "last login 53m ago · not persisted · store 54.4 MB" (call 2).
- How many: store holds "responses 1133 leagues 0 characters 65 tabs 3444 items 22721 (+6 removed) events 22751", plus 4 refused bodies and 8 granted PoE2 skills (call 3).
- From where: Standard has "3347 tabs … 405 fetched, 2926 never fetched, 16 folders (never fetched); 20231 items" (call 5). The never-fetched are almost all MapStash (2550 tabs, 61 fetched) and UniqueStash (502, 65 fetched) — substashes. 65 characters across pc and poe2, "50 fetched (10 with empty bodies), 15 never fetched; 1389 items" (call 6); the Standard pc ones were fetched 36–39d ago.
- How old: tabs "39d ago" (call 5); characters 36–40d ago, poe2 ones 45d (call 6); the search scope later says "oldest 40d ago, newest 2d ago" (call 8).
- Oddities already: `store status` says "leagues 0" while `tabs` finds a Standard league with 3347 tabs (calls 3, 5); `store status` counts 3444 tabs vs 3347 in Standard and doesn't say where the rest are (calls 3, 5).
- I changed nothing.

## The tasks

### T1 — a ring

**What I'd tell the person:** "Your friend's ring search (life ≥ 50, dexterity ≥ 20, and either chaos res ≥ 20 or fire res ≥ 40) finds **22 rings** of yours in Standard. One is worn: *Death Circle Unset Ring* on VVickedBossingServices. The rest are in stash tabs — e.g. *Pain Nail Diamond Ring* in MARKET, *Hate Turn Amethyst Ring* in Lab Runners, *Storm Band Two-Stone Ring* in Dump Tab, three in '3.25 Siege Ballista', two each in 'KB-CF Champion' and '3.25 RF Chieftain' (the strongest is *Fate Twirl Ruby Ring*, 110 life / 35 dex / 84 fire res), and others in single tabs (full list in calls 8 and 9). If you don't care about dexterity, it finds **91** — **69 more**, spread over 47 tabs (at most 3 per tab: '30 chaos', 'Gear', 'Lab Runners') plus 9 being worn by 8 characters (2 on mrs_flowers). Would the site have shown the same? The tool says the translation is exact: every clause that means something for your stash ran as the site reads it, and it ignored only the 'online' status filter, which is about sellers. But this is your stash as it was fetched 36–40 days ago, and 2,940 places (mostly map- and unique-tab sub-pages) were never fetched at all, so anything that's moved since, or that sits in those, isn't counted."

**Commands:** 8 (search, 20 rows), 9 (last 2 rows), 10 (`--describe trade`, to check what "exact" covers), 11 (`--explain 3`, dex forced true: "as given 22 · forced true 91"), 12 (the 69 added, by tab), 13 (the 9 tab-less, by character).

**Right?** I hold it right as far as the store goes: 91 − 22 = 69, which matches call 12's total of 69 with `-pseudo.total_dex>=20`. One row I checked by hand: *Grim Grasp Diamond Ring* is shown as "pseudo.total_life 87.5" from "+23 to Strength" and "+76 to maximum Life" (call 8) — strength / 2 counted as life, which is how the site's pseudo total works too. It would be wrong if `-term` let through rings that *lack* dexterity in a way the site wouldn't; the counts agree, so I don't think so. "Where it is" is only tab name and id here — no position in the tab.

**Where the tool stopped or misled me:** the `--next` page repeats the whole header and terms block (call 9). `--describe trade` (call 10) is 39,716 bytes and talks about "sittings", "the census", "t55", "C96", "the owner, 2026-10-15, verbatim" — insider records, not user explanation. In call 13 the term path column runs into the term ("0.3.0pseudo.total_dex>=20"). The "terms" header says "evaluated independently over live pc items in all leagues" even with `league=Standard` as a term — correct but it means the per-term counts are not what the person asked about.

### T2 — boots

**What I'd tell the person:** "None of your boots meet that search (armour ≥ 300, movement speed ≥ 25%, total elemental resistance ≥ 100, life ≥ 75). The resistance is what rules them out. Three pairs meet everything else but have only 20–32% resistance: *Entropy League Vaal Greaves* (worn by mrs_flowers), *Doom Sole Titan Greaves* (tab '30 chaos') and *Anarchy Hoof Titan Greaves* (worn by BoomBoomBored). Two pairs meet everything except armour, because they're evasion boots: *Gale Spark Harpyskin Boots* (140 res, 124 life, tab 'Soulrend PF') and *Ambush Spur Stealth Boots* (101 res, 75 life, tab "3.19 Death's Oath Occ"). Would the site agree? The tool flags one thing it may read differently: the site counts armour as if the boots had 20% quality, and the tool's figure can be one point off the site's. That can't matter here, because nothing failed on armour alone by one point. And the same 36–40-day age and 2,940 never-fetched places apply as for the ring."

**Commands:** 14 (search: "total 0 matches"), 15 (`--explain 4`: res forced true → 3), 16 (those 3, sorted by res), 17 (armour term dropped → 2, neither has armour).

**Right?** The zero I hold right. Each fact is in the rows: res 32/28/20 (call 16); "sorts last: no satisfying occurrence (armour.q20)" on both evasion pairs (call 17). Ambush Spur's 101 counts "+19% to Fire and Cold Resistances" twice (31+32+2×19), which is how the site counts it. I didn't run `--explain` on the life or movement-speed terms, so there could be a pair that misses only on life. Call 14 offered these, ordered "the part fewest items match first", and I ran just the first.

**Where the tool stopped or misled me:** nowhere badly. The zero answer came with ready-made `--explain` offers (call 14), which was the right thing. The one "ambiguous" note is a paragraph of internal record ("S26", "step 3, tests/properties.rs", "the owner on ambiguous, 2026-10-15, verbatim: \"confirmed\"") that a person can't do anything with (call 14). In call 16 the sorted value is printed as "sorts by 32" in front of a single line ("+32% to Fire Resistance"), which looks like the total but is actually `pseudo.total_ele_res`; the row doesn't name the field it sorted by.

### T3 — a shield

**What I'd tell the person:** "One shield matches: **Tukohama's Fortress** (unique Ebony Tower Shield): +1 to maximum number of totems, 117 life (91 + 26 implicit), 473 armour at 20% quality. It's in the tab 'Facebreaker Totems (Remove-only)'. You have 4 other shields with +1 totem, and none comes close on life. The nearest is a rare *Rift Aegis Shagreen Tower Shield* with 88 life (tab '3.20 Shockwave Totem'), then a buckler with 3 life and two shields with no life at all. Would the site agree? Same caveat as the boots — armour is counted the site's way at 20% quality, give or take a point, and this shield is far over the 100 asked for — plus the usual 36–40-day age and the unfetched places."

**Commands:** 18 (search: "total 1 match"), 19 (all totem shields, sorted by life: "total 5 matches").

**Right?** I hold it right. Call 18's single row shows every asked value. Call 19 shows the other four totem shields failing on life. If the site filed some shields' totem line under a different stat id, the tool would miss them. It flagged no collision for this stat (call 18 lists only the armour ambiguity).

**Where the tool stopped or misled me:** it didn't. Small thing: term 3's count ("26 matched") is over every category in every league, so it looks like 26 totem shields when there are 5 (calls 18, 19). That's the "evaluated independently" rule again.

### T4 — magic find

**What I'd tell the person:** "His search (at least 20% increased item rarity as an explicit mod) finds **160** of your items in Standard. The ones with **at least 30%: 27 items**. Best first: two *Andvarius* rings (70% and 53%), *Empyrean Gyre Sapphire Ring* and *Empyrean Eye Bone Ring* (51% each), *Aurseize* gauntlets (50%, worn by VVickedWilliam), *Greed's Embrace* (48%), *Dire Torc Gold Amulet* (41%), then a run down to 30%: Ventor's Gambles, two rare helmets (40%, 39%), Reaper's Pursuit, several Presence/Eye of Chayula, Wheel of the Stormsail, Le Heup. Where they are: 5 in your unique tab '~price 4554 chaos'; 2 each in '3.13 Golems, BL, Misc', 'Small Jewelry' and the unique tab '~price 4444 chaos'; one each in 12 other tabs ('3.12 Scion', '8 chaos', 'Corrupted Uniques', 'Gear #2 (3.16)', 'Items 4', 'Lab Bases #1', 'Mercenary Items', 'PConc', 'Special Gear', '|||||| MARKET ||||||', '~price 20 chaos (A)', '~price 30 chaos', '~price 40 chaos (F)'); and 3 worn — by BatteryStaple, LordHelpTheMistress and VVickedWilliam. 'Best' here is the explicit rarity his search counts. Counting the rarity some rings have built in, Dire Torc (57 total) and the Ventor's (43–47) would rank higher. Would the site agree? Mostly. The tool warns that the site also folds 'reduced rarity' lines into this stat and the tool doesn't, which could only matter for items with a negative roll. Plus the usual: 36–40 days old, and 2,940 unique/map sub-pages never fetched — which matters more here, because many of these are uniques."

**Commands:** 20 (search: 160), 21 (≥30, sorted with `'sum("T")'`: 27 — but the order was wrong, see below), 22 (≥30, sorted by the explicit sum: right order), 23 (count by tab and character).

**Right?** The 27 and the tab counts I hold right: 17 tab values + 3 "(none)" = 27 and 3 characters + 24 "(none)" = 27 (call 23). The order: call 22's is by the value filtered on. I only printed 3 rows of it and wrote the rest of the order from call 21's explicit values, reordered by hand. A reader should check against a full `--limit 30` run of call 22's command, which I didn't make.

**Where the tool stopped or misled me:** `--sort 'sum("T")'`, the form `search --help` gives, sums every source — implicit too — while the trade translation's filter is explicit-only. So "best first" came out in an order that isn't the filter's value: *Dire Torc* (41 explicit, "sorts by 57") above *Empyrean Gyre* (51) (call 21). Nothing warned me. I caught it only because the row prints both numbers. Unique-tab rows print an empty sub-page name: "~price 4554 chaos /" and "~price 4444 chaos (Remove-only) /  (Remove-only)" (calls 20, 21), so 'where is it' ends in a blank. Ambiguity note in call 20 is short and clear this time.

*Correction to T4 above:* "one each in 12 other tabs" should read **13**. Call 23 lists 17 tab values: 4 with more than one item, 13 with one. The 13 names given are right.

### T5 — fire resistance

**What I'd tell the person:** "This search is broad — at least 30% fire resistance from mods — and finds **510** of your items in Standard: 94 rings, 85 helmets, 60 body armours, 59 boots, 54 amulets, 54 belts, 47 gloves, 26 shields, 13 quivers and 18 weapons. 35 are being worn, across 20 characters (BoomBoomBored and LordHelpTheMister 3 each). The other 475 are spread over 141 tabs. The biggest piles: '~price 40 chaos (F)' (37), 'Dump Tab' and 'Gear' (13 each), 'Rings (Breach)' (12), 'Items 4' (11), then nine each in 'Breach Rings', 'Decent Gear', '~price 30 chaos (B)' and your '~price 4554 chaos' unique tab. Note that it counts only plain '+% to Fire Resistance' lines: a ring with '+% to Fire and Cold Resistances' or '+% to all Elemental Resistances' isn't counted, on the site either — that's how his search is built. Would the site agree? The tool flags one doubt: when an item has a scourge (Scourge league) fire-res line, the site seems to leave it out of the sum and the tool adds it in. You have 6 items with a scourge fire-res line. The 2 with a negative one (Viper Sanctuary Zodiac Leather, a Kaom's Heart) have no other fire res, so they don't change. The other four have positive ones (+20 to +29), which can only move an item if it sits right at the 30 line. Plus the usual 36–40 days and the unfetched sub-pages."

**Commands:** 24 (search with counts by tab, character, category: "total 510"), 25 (the 6 items with a scourge fire-res line), 26 (`show` one negative-scourge item: no explicit fire res).

**Right?** The 510 and the counts are consistent: 475 + 35 = 510 (call 24), and the category counts add to 510. I didn't fetch the 81 tabs past `--limit 60` (call 24 says "141 values · 60 shown"). I didn't check the four positive-scourge items (Redblade Banner, Kaom's Heart, Starkonja's Head, Cherrubim's Maleficence — call 25), and if any has explicit fire res below 30 that reaches 30 only with the scourge line, the tool would list it and the site might not. That's a gap I left.

**Where the tool stopped or misled me:** it didn't stop me. The ambiguity note is again written for the builder ("e043 and f005", "for a sitting", "tests/trade-oracle.toml") (call 24), but this time it was concrete enough to act on — I could check my own scourge items (calls 25, 26). The `show` "place" line gives tab name only, no position in the tab (call 26).

### T6 — maps

**What I'd tell the person:** "His search (blighted maps, tier 14 and up) finds **83** of yours: **44 tier 14, 22 tier 15, 17 tier 16**. 69 are in the tab 'Blight Maps' (37 T14, 20 T15, 12 T16) and 14 in 'Blight' (7 T14, 2 T15, 5 T16). You also have **13 Blight-ravaged maps** at those tiers (2 T14, 7 T15, 4 T16; all but one T15 in 'Blight') that his search doesn't count. If you'd run those too, that's T14 46 / T15 29 / T16 21. Below tier 14 there are 63 more T13s and a scatter of lower tiers. Would the site agree? The tool reads the tier from the map's name ('Blighted Map (Tier 14)'), and says it's not certain the site does the same for every map. For plain blighted maps I don't see how it could differ. I'm *not* sure whether the site's 'Blighted' filter takes in Blight-ravaged maps — the tool doesn't say. Bigger caveat: your map stash has 2,550 sub-pages and only 61 were ever fetched, so any blighted maps sitting there aren't counted at all."

**Commands:** 27 (search with counts by base and tab: "total 83", 44/22/17, 69/14), 28 (every Standard item whose type starts "Blight", crossed base × tab: found the Blight-ravaged ones), 29 (all Standard maps by tab: 769 in 10 tabs + 9 not in a tab).

**Right?** I hold 83 = 44 + 22 + 17 right (call 27), and it agrees with call 28's cells: T14 37 + 7, T15 20 + 2, T16 12 + 5. The Blight-ravaged counts are from call 28's cells (T14 2, T15 6 + 1, T16 4). It would be wrong if blighted maps are hiding in the unfetched map-stash pages (tab counts, call 5), and nothing here can say.

**Where the tool stopped or misled me:** the translation turns "Blighted Map: yes" into `typeline~"^Blighted "`, which silently leaves out 13 Blight-ravaged maps at the asked tiers (calls 27, 28). The catalogue marks the separate `map_uberblighted` filter unsupported, but the answer never mentions that Blight-ravaged maps exist and aren't counted. The tier ambiguity note again cites "C101", "the census's copy", "u065", "decisions/trade.md" and quotes the owner (call 27). `--count tab` doesn't show the tab's type, so I couldn't tell whether 'Maps (Remove-only)', with 441 maps (call 29), is the map stash.

### T7 — what I'd take

**What I'd tell the person:**
- **Ring — Fate Twirl Ruby Ring.** Tab '3.25 RF Chieftain (Remove-only)', column 7, row 8 counting from 1 (the store records x 6, y 7, counting from 0). Of the 22 rings from T1 it has the most life. Everything the tool knows: rare Ruby Ring, item level 75, requires level 64, identified, not corrupted. Implicit +26% fire res. Explicit +35 Dexterity, +110 maximum Life, +44 maximum Mana, +43% Fire Resistance, and crafted +15% Fire and Chaos Resistances. Enchant 'Your Meteor Towers create Burning Ground for 3 seconds on Hit'. That's 84% fire res in all, but only 15% chaos. No price on it — not by hand, not in game — and its tab isn't public, so you aren't selling it. As the tool last saw it ~39 days ago.
- **Boots — Gale Spark Harpyskin Boots**, tab 'Soulrend PF (Remove-only)', column 8, row 7 (x 7, y 6). *None* of your boots fully met his search. These miss only on armour, because they're evasion boots: 30% movement speed, 124 life, 48/44/48 fire/cold/lightning (140 total), 586 evasion at 20% quality, four green sockets linked. Needs level 78 and 153 Dex. If you want armour as he asked, the closest is *Entropy League Vaal Greaves*, worn by mrs_flowers (128 life, 30% MS, 353 armour) — but only 32% resistance.
- **Shield — Tukohama's Fortress**, tab 'Facebreaker Totems (Remove-only)', column 8, row 4 (x 7, y 3). It's the only shield that met the totem search. Before you take it, know that it carries **Blood Magic** (your skills cost life instead of mana), along with +1 totem, 40% increased totem damage, +300 armour per totem and 117 life. Needs level 70 and 159 Str; three red sockets, linked. If Blood Magic doesn't suit the build, the runner-up is the *Rift Aegis Shagreen Tower Shield* with 88 life, in '3.20 Shockwave Totem' (call 19).

**Commands:** 30 (`show --body` ring), 31 (`price show <id>`, refused), 32 (`price show item/<id>`), 33 (`show --body` boots), 34 (`show --body` shield). The choices rest on calls 8, 16, 17, 18, 19.

**Right?** The facts are as the store holds them (calls 30, 32–34). The choice is my judgement: "best ring" by life, with the asked res. The positions come from the raw body's `x`/`y` (calls 30, 33, 34), not from anything the tool derived, and from a fetch ~39 days old. If the person has moved the item since, the tool can't know.

**Where the tool stopped or misled me:** the tool never says *where in the tab* an item sits. `show`'s "place" is tab name only (calls 30, 33, 34); I had to dig `x`/`y` out of `--body` and turn them into a column and row myself. `price show` with the bare item id — the id `search` and `show` print — was refused with "is not a price scope … a target is one word: item/<id>…" (call 31). The refusal told me enough to recover in one try, but no other verb uses the `item/` prefix.

## Faults

Worst first. "Both" means a person at a keyboard would hit it too.

1. **The blighted-map translation silently drops Blight-ravaged maps.** "Blighted Map: yes" becomes `typeline~"^Blighted "` (call 27). The person owns 13 Blight-ravaged maps at T14–16 (call 28), and none is counted or mentioned. The answer's ambiguity block names only the tier reading (call 27). It should at least say "Blight-ravaged maps are not matched (N in scope)", because the catalogue already knows the site has a separate, unsupported filter for them (call 10, `map_filters.map_uberblighted`). Both.
2. **`--sort 'sum("T")'` doesn't sort by what the trade filter measured.** The filter is explicit-only. `sum("T")`, the form `search --help` gives, also adds the implicit line, so "best first" ranked *Dire Torc* (41 explicit, "sorts by 57") above *Empyrean Gyre* (51) (call 21). No warning. Spelling the whole `sum(line("T" source=explicit).arg1)` sorted right (call 22). The sort should take the query's own value by default, or say which lines it summed. Both.
3. **No position in the tab.** `search` rows and `show`'s "place" give tab name only (calls 8, 26, 30). The `x`/`y` exist only in the raw `--body` JSON (calls 30, 33, 34). "Where is it" is the person's first question. Both.
4. **Unique-tab locations end in a blank sub-page name:** "~price 4554 chaos /" and "~price 4444 chaos (Remove-only) /  (Remove-only)" (calls 20, 21). For a unique tab that's 502 pages (call 5), the blank is the part that matters. Both.
5. **The faithfulness notes are written in the builder's private records:** "S26", "step 3, tests/properties.rs", "the owner on ambiguous, 2026-10-15, verbatim: \"confirmed\"" (call 14); "e043 and f005 … for a sitting" (call 24); "C101 … u065 … decisions/trade.md" (call 27). `--describe trade` is the same at 39,716 bytes (call 10). A person can't judge "would the site have shown me the same?" from these without translating them. Both.
6. **`price show` refuses the id every other verb prints.** "is not a price scope … a target is one word: item/<id>, …" (call 31). The refusal was enough to recover (call 32), but the bare 64-hex id is unambiguous and could have been accepted. Both.
7. **`store status` disagrees with `tabs`:** "leagues 0" and "tabs 3444" (call 3) versus "3347 tabs in Standard" (call 5). Neither says where the other 97 tabs are, or why no league is counted. Both.
8. **Per-term counts are over every league and category.** "26 matched" for the totem line when there are 5 totem shields in Standard (calls 18, 19). The header says so ("evaluated independently over live pc items in all leagues"), but the numbers read as answers. Both.
9. **Layout:** a nested term's path runs into its text, "0.3.0pseudo.total_dex>=20" (call 13); a `--next` page repeats the whole header and terms block (call 9). Both.
10. **`--count tab` doesn't show the tab's type,** so I couldn't tell whether 'Maps (Remove-only)' (441 maps) is the map stash (call 29). Both.
11. **The tour opens with `cargo build … alias acq=./target/debug/acq`** and points at AGENTS.md, CLI-REFERENCE.md, LIVE-TESTING.md and "skills" (TOUR.md). It's a developer's README, not a user's. Agent (told not to build) and person alike.

## What the tour and the help did not say

- `-term` excludes items that *lack* the value as well as those that fail it — inferred when call 12's 69 equalled call 11's 91 − 22. Expected in `search --help` under QUERY, which only says "the language's not".
- `character` is a `--count` key, and `-has:tab` means "worn or carried by a character" — I tried `character` in call 13; `-has:tab` I copied from the route printed in call 12. Expected in `--count`'s help, which names only "`tab`, `league`, `rarity`, `base`, …".
- `--sort` takes a full `sum(line(... source=explicit).arg1)` expression (call 22), and `sum("T")` includes implicit lines (call 21). Expected at `--sort` in `search --help`.
- `has:map_tier` is a valid term (call 29) — a guess from the trade translation's `map_tier`.
- A price target needs the `item/` prefix (call 31). The tour says only `price show <target>`.
- An item's position in its tab is only in `show --body` (call 30). Expected in `show --help` or the tour's `show` line ("one item as the search derives it").

## The tool's own words

- **Right and useful:** `search --help` is thorough. On a zero answer the tool offers `--explain` per term, ordered by selectivity (call 14), and `--explain` (call 11) answered "what does dropping dex add?" in one command. Every count carries a route that reproduces it (call 12 → 13). The scope line states plainly "2940 locations never fetched", and on the zero answer adds "nothing here says what they hold" (call 14) — exactly the honesty the person asked for.
- **Misleading:** "exact — every clause with a stash meaning ran as the site reads it" (call 8) is about the *translation* and says nothing of coverage. A person reads "exact" as "the same results". The `--sort` example `'sum("T")'` (call 7) — fault 2.
- **Read, never used:** `--membership`, `--query-file`, `--trade-file`, `--routes`, `--context`, `--print-request`, `--request`, `--json`, the bare `--describe` index (~47 KB), and the whole jobs, policy, refresh and shop surface (call 1).
- **Words I didn't understand or had to guess:** "basis" and its parts ("snapshot 1133 · facts v8 · intent 33 · derivation 11 · totals v6"); "sitting", "census", "the park", "the owner"; the codes C11, C93, C96, K6, S26, S56, t55, u065; "granted: 8 item-granted skill(s)" (call 3); "relation none" (call 32); "undecided" vs "lacked" (both in call 7's help; I worked out "lacked" from the counts).

## What you would ask of the tool

- Put the tab position (column, row) and the sub-page name in every row and in `show`'s place. (calls 20, 30)
- Make the sort default to the query's own measured value, or print which lines a `sum` took. (call 21)
- Name the item kinds a translated clause leaves out, with a count — e.g. Blight-ravaged maps. (calls 27, 28)
- Write ambiguity notes as one plain sentence about the person's items, with the internal record under `--json`. (calls 14, 24, 27)
- Accept a bare item id in `price show`. (call 31)
- Show the tab type in `--count tab`. (call 29)
- Make `store status` and `tabs` agree on leagues and tab counts, or say why. (calls 3, 5)

## Left out

- **Not tried:** `--json` anywhere; `--routes`; `tabs --expand` (3347 tabs, likely over the 60,000-byte cut); `price list`/`status`; `store events` (to see whether my picks moved); `--explain` on the life and movement-speed terms of the boots search; a full `--limit 30` run of call 22; the 81 tabs past `--limit 60` in call 24; `show` on the four positive-scourge items in call 25.
- **Files opened besides the three:** one. Calls 9 and 10 ran in one shell invocation, and their combined output (42.4 KB) was too big for my tool display, so the harness saved it to `~/.claude/projects/-Users-tom-seat-worlds-trade-2026-10-cli-opus/b12c3dbb-…/tool-results/bfjha30s4.txt`, which I read with the Read tool. It holds only the door's own output of those two calls — nothing else. No file under W other than BRIEF.md, TASKS.md and TOUR.md was opened.
- **Work files written:** none.
- **One cut I'd most want back:** call 22 at `--limit 3`. The T4 "best first" order past the top three is my hand reorder of call 21's numbers, not a list the tool printed.
- **Door cuts:** no output hit the door's 60,000-byte head cut.
