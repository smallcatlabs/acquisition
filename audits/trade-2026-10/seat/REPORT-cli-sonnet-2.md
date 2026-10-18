<!-- written during the run; call numbers are rows of calls.csv -->

## 1. Before touching anything

From TOUR.md only (no command run yet):
- Can do: log in via OAuth (`acq auth`), fetch profile/leagues/characters/stash tabs through a daemon (`acq stashes`, `acq stash`, `acq refresh`), keep a local store, and search it offline (`acq search`, including `--trade <URL>` for a pasted trade-site search, `--count`, `--cross`, `--sort`), show one item (`acq show`), price items (`acq price ...`), render shop pages.
- Needs: login + daemon + network for the job verbs (auth, profile, leagues, characters, stashes, stash, refresh). Offline with no daemon: `tabs`, `store ...`, `search`, `show`, `price`, `reference`, `shop render`, `policy`, `refresh --plan`.
- Would not run without asking the person: `acq auth`/`logout` (login/session), `refresh --all/--apply`, `submit`, `store import/rebuild`, `price set/clear`, `policy set`, `daemon stop/reset-tripwire` — they spend API budget or change the person's data. (In this rehearsal world they are safe, but not in the real one.)
- Tour oddities: it tells me to `cargo build` and mentions AGENTS.md, CLAUDE-style files, skills and `CLI-REFERENCE.md`, none of which I may use; it cannot be followed as written.

The world as found (calls 2, 4, 5): store GERWARIC_7694.db, 54.4 MB, items 22721 (+6 removed), tabs 3444 across every realm and league, 65 characters (call 2). In Standard: 3347 tabs, only 405 fetched, 2926 never fetched, 20231 items (call 4); fetched tabs are about 39 days old (call 4). Characters: 50 fetched, 10 with empty bodies, 15 never fetched, 1389 items; PoE2 characters exist too, so the store holds two realms and search needs --realm (calls 3, 5). Caveat for the person: anything in the never-fetched tabs (mostly map/unique tabs) is invisible to every search, and data is ~39 days old.

## 2. The tasks

### T1 — ring
Told the person: the pasted search translates exactly ("exact — every clause with a stash meaning ran as the site reads it", call 6) to Standard rings with life>=50, dex>=20, and (chaos res>=20 or fire res>=40); 22 of their rings match (call 6). 20 shown with places (call 6): mostly "(Remove-only)" tabs, plus MARKET, Lab Runners, Dump Tab, one on character VVickedBossingServices (equipment). The tab count (call 7) accounts for the other 2: tabs "Mercenary Items (Remove-only)" and "~price 50 chaos (G) (Remove-only)". Without the dexterity clause: 91 rings (call 8), so dropping dex adds 69.
Commands: 6, 7, 8. I hold the count right for what the store has; it is not right as "all my rings": 2940 locations were never fetched and the data is 40d old at the oldest (call 6). Versus the site: the site shows other people's public listings, this shows only mine, and online/status is meaningless here (call 6). Terms are counted over all leagues (call 6 "each term counted alone over all live pc items in all leagues"), but the total is league-restricted.
Where the tool misled/stopped: `--limit` with `--count` shows no rows (call 7), so I needed a second call; the 2 unseen rows I only inferred from the tab counts, not read. `--next` token is a huge string I did not use.

### T2 — boots
Told the person: nothing of theirs matches this boots search: 0 matches (call 9). The translation is "ambiguous": the armour-at-20%-quality clause (armour.q20>=300) is figured within one of the site's own number, so a boot right at the edge could differ (call 9). I checked that this does not hide a match: forcing that clause true gives only 2 (call 10), forcing the elemental-resistance clause (>=100) true gives 3 (call 11), and the three that meet everything but resists are listed (call 12): Anarchy Hoof Titan Greaves on character BoomBoomBored, Entropy League Vaal Greaves on mrs_flowers (both equipped), and Doom Sole Titan Greaves in tab "30 chaos (Remove-only)" x6,y10. None has 100% ele res, so I would call them near misses, not matches. (The third has Armour 268 shown but q20 322 — derived value.)
Commands: 9, 10, 11, 12. I hold 0 to be right for the store (the explain counts agree with each other); it would be wrong if never-fetched locations (2940, call 9) hold boots. Tool strength here: --explain was useful. Weakness: the "offers" list prints the full query six times, noisy.

### T3 — shield
Told the person: 1 match (call 13): the unique Tukohama's Fortress Ebony Tower Shield, in tab "Facebreaker Totems (Remove-only)" at x 7, y 3, with +1 to max Summoned Totems, 117 life, armour 473 at 20% quality. Translation "ambiguous" only for the armour-at-20%-quality clause (rounding, call 13). Command: 13 only. I hold it right for the fetched tabs; the totem clause is `source=explicit` only (26 items match it in the whole store), so an item with the totem mod from another source would not show. Not checked: whether a shield just under 100 armour at q20 is excluded by rounding (the tool says within one).

### T4 — magic find
Told the person: the site's search (item rarity >= 20, explicit) finds 160 of their items (call 14); with their own "at least 30" it is 27 items (call 15, 16). Best first (call 15): Andvarius Gold Ring 70% explicit (78 with implicit) in "~price 4444 chaos (Remove-only)", another Andvarius 53% in "~price 4554 chaos", Dire Torc Gold Amulet 41 in Small Jewelry, Empyrean Gyre ring 51 (Small Jewelry), Empyrean Eye ring 51 (MARKET), Aurseize Steelscale Gauntlets 50 on character VVickedWilliam (equipment), Greed's Embrace 48 in "8 chaos"... 12 of 27 shown. Tabs of all 27 (call 16, 17 values): "~price 4554 chaos" 5; "3.13 Golems, BL, Misc" 2; "Small Jewelry" 2; "~price 4444 chaos" 2; 12 other tabs 1 each; 3 are on characters (shown as "(none)" tab).
Commands: 14, 15, 16. Call 14 printed 38144 bytes and the harness (not the door) saved it to a file and showed me 2 KB; I did not open that file and re-ran narrower (calls 15, 16).
Caveats: (a) the translation is "ambiguous": the site's stat bound "sums the text and its plurals; not its forms 'reduced Rarity'" (call 14). (b) The query counts explicit lines only (call 14's query: source=explicit), but rows are sorted by explicit+implicit total: Andvarius 78 sorts first though its explicit is 70, and a ring whose total >=30 only thanks to an implicit would be dropped (e.g. an item with 20 explicit + 15 implicit) — this matches how the site's filter reads, I assume, but I did not verify against the site. (c) --sort with the sum expression worked on my first guess only because call 14's output named the form. The 3 "(none)" items are characters' equipment, not a tab; the person has to be told that. Count mismatch to explain: "30 matched" in the term line vs "27 total" — the term line counts every league (call 15), the total is Standard only.

### T5 — fire resistance
Told the person: 510 of their items match (call 17), too many to list: by type rings 94, helmets 85, chests 60, boots 59, amulets 54, belts 54, gloves 47, shields 26 (the first 8 of 18 categories, call 18); in 141 tabs — biggest "~price 40 chaos (F) (Remove-only)" 37, Dump Tab 13, "Gear (Remove-only)" 13, "Rings (Breach)" 12, "Items 4" 11 — and 35 are on characters (call 18). The search is "fire res >= 30 summed over explicit and scourge lines"; the translation is "ambiguous": the site may leave out a scourge modifier in a weighted sum (call 17), so this may show a few more than the site. It does not count implicit fire res, nor "all elemental" or "fire and cold" lines (e.g. rows show only "+45% to Fire Resistance (explicit)"); whether the site does the same I could not tell from the tool. Commands 17, 18. A stray typo run of the door with a wrong argument preceded call 18 (disclosed in Left out). I only read 15 of 510 rows; I hold the count right, the 8-of-18 category list cut.

### T6 — maps
Told the person: 83 of their Blighted maps of tier 14 or higher (call 19, 20): 44 at tier 14, 22 at tier 15, 17 at tier 16 (call 20). Where: 69 in tab "Blight Maps", 14 in tab "Blight" (call 20); coordinates are in call 19 (40 of 83 rows shown, all "normal" rarity). Caveats the tool gave: translation "ambiguous" twice (call 19): the tier is read from the map's base name, so a map whose base names no tier has none here where the site gives it one; and "blighted" is a type line starting "Blighted", so a "Blight-ravaged" map is outside it where the site may count one. The search was tier >= 14 from the URL, so the person's tier 13 and lower maps (63 at Tier 13 in the whole store, call 19) are not counted, and tabs of maps not fetched (2940 locations, call 19) are not included. Commands 19, 20. I hold the 44/22/17 split right for fetched tabs; the sum 83 matches the total (call 19, 20).

### T7 — what I'd take
Told the person:
- Ring: Fate Twirl, Ruby Ring (rare, ilvl 75, req. level 64), in tab "3.25 RF Chieftain (Remove-only)" at x 6, y 7 (call 21). It is a T1-search match (call 6) with +110 life, +35 dex, +44 mana, +43% fire res, +15% fire and chaos res (crafted), +26% fire res (implicit), and an enchant "Your Meteor Towers create Burning Ground for 3 seconds on Hit" (call 21). Everything the tool knows is in `show` plus `--body` (call 26): the stored body adds the inventoryId "Stash1" and "verified": false; no price (call 25: "nothing applies"; the tab "reads none, not public").
- Boots: nothing meets the T2 search (calls 9–12). Best of the near misses: Entropy League Vaal Greaves, rare, 4-linked G-G-G-R, 353 armour at 20% quality, +128 life, 30% total movement speed, +32% fire res, +26% chaos res — but NO cold or lightning res, and it is equipped on character mrs_flowers (calls 12, 22). Stash alternative: Doom Sole Titan Greaves in "30 chaos (Remove-only)" x6,y10: 83 life, 28% move speed, only 28% cold res, 268 armour (call 23). I would say plainly that neither is a match and the Entropy pair is a pick only if they don't mind taking it off a character.
- Shield: Tukohama's Fortress Ebony Tower Shield, tab "Facebreaker Totems (Remove-only)" x 7, y 3 (call 13) — the only match.
Commands: 21–26 (call 24 was a refused `price show` with a bare id; the error told me to write item/<id>, enough to go on). Wrong if: items moved since the fetch (40d old, call 6) or equipped items were swapped; the "Remove-only" tabs may be tabs the person flagged as retired. I did not show the shield or boots in full beyond the above.

## 3. Faults
1. (Both person and agent) The scope line always says the search is over "22623 items", "546 locations fetched", "2940 locations never fetched … nothing here says what they hold" (calls 6, 9, 13, 15, 17, 19) — good — but "the full list: --view locations, not built": the help of `--view` says "Not built" (call 3), so I could not see which tabs were never fetched; I only know that Standard is 3347 tabs/2926 unfetched (call 4). That is the main limit on "what do I have".
2. (Both) Term lines count "over all live pc items in all leagues", so the per-term number (e.g. 30 matched, call 15, 16) does not equal the total (27) and I had to infer why (calls 15, 16, 19).
3. (Both) The trade-translation is applied but its detail is easy to miss: T4's query only counts explicit lines while the sort used explicit+implicit (call 15, Andvarius 78 vs 70) — the site's reading is not stated.
4. (Both, rare) Large output: call 14 printed 38144 bytes of rows because --limit 100 is mine; the harness cut it, not the door. My own doing, but the default of printing a `--next` token of ~1.5 KB at every "more" line is long noise (calls 6, 8, 15, 17, 19).
5. (Both) `--count` with `--limit` shows no rows without saying so; `--limit` then means table size (call 7).
6. (Both) `price show` needs `item/<id>` and the `show` output gives none of that (call 24, error), though the error is clear.
7. (Both) The `offers` for a zero result print the full query six times (call 9).
8. Items equipped on characters are reported with the tab blank — "(none)" in --count tab (calls 7, 16, 18) — while the row place says "(equipment)". Not a fault, but a person would need to know.

## 4. What the tour and the help did not say
- That a store with PoE1 and PoE2 characters needs `--realm` (help of search says so, call 3); but with a --trade URL the realm comes from the URL and `--realm` beside it is an error (call 3 help) — I only used one or the other, fine.
- That `--sort` takes a `sum("<template>")` expression for an aggregate: the --help gives the form `'sum("T")'` (call 3). I took the template text from the printed query in call 14.
- That `--explain N` numbers terms from 0 including league= (call 10, 11) — found from the offers list in call 9.
- That the tab column of a count has "(none)" for characters (call 7).
- That tour text names files I may not read (CLI-REFERENCE.md, AGENTS.md, skills) and a `cargo build` (TOUR.md).
- Not learnt: how to list the unfetched tabs; what "Remove-only" means; whether `price` would show a price for items in tabs.

## 5. The tool's own words
- Right and useful: the `--trade` header ("exact"/"ambiguous", each ambiguous clause named, calls 6, 9, 13, 17, 19), the "over" line ("the site shows public listings, so the search is the site's and the items are yours"), the scope lines about never-fetched locations, `--explain`, and the `price show` refusal (call 24).
- Words I did not understand: "lacked", "undecided", "basis", "scourge", "Remove-only" (a tab name, probably the person's), "forced true".
- Read and never used: `--context corpus`, `--print-request`, `--request`, `--next`, `--routes`, `--fields`, `--view`, all job verbs, `daemon`, `dash`, `shop`, `policy`.
- Tour: lists `acq search --trade` as "run and labelled clause by clause" — accurate. The tour's build step cannot be done here.

## 6. What you would ask of the tool
- List the never-fetched tabs (`--view locations`, call 6/3) so I can tell the person what is missing.
- `--count` should say that rows are suppressed (call 7).
- A shorter `more` line (the token) or print it only with --routes (call 6).
- Make `show` print the `price show` target (call 24).
- Say in the trade header whether implicit lines count in the site's stat filter (call 15).
- A total of matches in the term lines that is for the league and not all leagues (calls 15, 16).

## 7. Left out
- Not tried: the job verbs, daemon, auth, shop, policy, `--request`, `--next`, `--view`, `--json`, `--context corpus`, `--membership all/removed` (removed items), `--describe`, `store events/refused`, price listing. I did not look at the 4 refused bodies (call 2).
- Not verified against the real trade site.
- Files opened that are not the three: none. Call 14's output was cut by the harness and saved to a file I did not open.
- Work files written: none.
- A slip: before call 18 I ran the door once with a mistaken argument (python3 -I …/seat.py …/seat.py) with `2>/dev/null` appended — a redirect and not a call recorded in calls.csv as I recall; it did nothing to the tool.
- The one cut I'd most want back: `--view locations` (the list of never-fetched tabs).
