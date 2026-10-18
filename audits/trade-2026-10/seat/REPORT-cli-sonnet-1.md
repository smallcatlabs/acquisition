<!-- written during the run; call numbers are rows of calls.csv -->

## 1. Before touching anything

From TOUR.md only (no command yet):
- acq fetches stash tabs/characters from the game API via a local daemon (`acq stashes`, `stash`, `refresh`, `character(s)`, `submit`), keeps a local store, and searches/prices/lists it offline (`search`, `tabs`, `show`, `price`, `shop render`, `store ...`). `search --trade <URL|ID>` runs a trade-site search against the store.
- Needs a login (`acq auth`, OAuth) and the daemon (lazily spawned) plus network for the job verbs (profile, leagues, characters, stashes, stash, refresh, submit). The store verbs (tabs, store, search, show, price, reference, shop) are described as offline, no daemon, no network.
- I would not run without asking the person: `auth`/`auth logout`, `refresh --apply`/`--all`, `submit`, `price set`/`price clear` (change their prices; the brief says safe here, but in real life they edit their data), `policy set`, `store import`/`rebuild`, `daemon stop/reset-tripwire`, `cancel`. TOUR.md's first line (cargo build) is something I am forbidden to run.
Then the world (calls 1–5), changing nothing: `acq store status` (call 2): GERWARIC#7694 store, 22721 items, 65 characters, 3444 tabs, no leagues rows, "refused: 4 bodies". `acq tabs` (call 3): 3347 Standard tabs, 405 fetched, 2926 never fetched, 20231 items; tabs last fetched 39d ago. `store characters` (call 4): 65 characters, 50 fetched, 15 never; Standard characters fetched 36–39d ago. Every search later says "546 locations fetched (oldest 40d ago, newest 2d ago); 2940 never fetched" (call 6). So the store is a stale, partial copy: anything in the 2940 unfetched locations is invisible to every answer below. `acq search --help` read (call 5).

## 2. The tasks

**T1 ring.** Told the person: 22 of your Standard rings match (call 6): 20 shown, the other 2 are in "Mercenary Items (Remove-only)" and "~price 50 chaos (G) (Remove-only)" (call 10, counted by tab; rows themselves I did not print); 1 of the 22 is a worn ring (VVickedBossingServices equipment, "Death Circle"), 17 tabs hold the rest, 8 of the first 20 sit in tabs 3.25 Siege Ballista / RF Chieftain / KB-CF Champion etc. all "(Remove-only)". Dropping the dexterity clause (call 9): 91 matches, so dex>=20 removes 69 rings (91 vs 22; both from the tool). Commands: 6, 7, 8, 9, 10. Right? The translation is "exact" (call 6) and the counts per term are printed, but I did not verify each row by hand; what would tell me it is wrong: a ring in a tab that is stale (tabs 39d old), or the site's "online/any" status clause being ignored (reported, "listing-only"). Site vs store: the site only shows items LISTED for sale; this finds anything stored, including worn items, "Remove-only" tabs, unpriced stuff. Where it stopped me: call 6 says "reported 1 with no stash meaning ... status" but the human output only names "status" – I needed `--json` (call 8, 19 KB) to learn which; call 7 (`--describe trade`, 39716 bytes) was too big for the harness and I did not read it.

**T2 boots.** Told: none. 0 of your boots match (call 11). The reason is the armour clause: dropping it gives 2 boots (call 12: "Ambush Spur Stealth Boots" in 3.19 Death's Oath Occ, "Gale Spark Harpyskin Boots" in Soulrend PF (Remove-only)), but those are evasion bases, so the site's Armour>=300 clause would have excluded them too. The tool flagged that clause "ambiguous" (the field is armour with quality taken to 20%, within one of the site's figure, call 11). 2 commands (+ reading). Right? Probably, with that caveat; a boot with armour ~299 vs 300 is where the site and the tool could differ. The 0 result also says "2940 locations were never fetched: nothing here says what they hold" (call 11) – good honest line.

**T3 shield.** Told: 1 match: Tukohama's Fortress (unique Ebony Tower Shield) in "Facebreaker Totems (Remove-only)", life 117, +1 totem (call 13). One command. Same ambiguous armour caveat (call 13). Right? Plausible; the armour clause was in the tool's ambiguity list; relaxing it I did not try.

**T4 magic find.** Told: 160 Standard items have at least 20% (explicit) increased rarity (call 14, 30 rows read; 168 across leagues); at least 30%: 27 items (call 15), best first: Andvarius 70% explicit in "~price 4444 chaos (Remove-only)" tab, Andvarius 53% in "~price 4554 chaos", Dire Torc / Empyrean rings in Small Jewelry / MARKET (Remove-only), Aurseize on character VVickedWilliam, etc. (all 27 rows are in call 15). Commands 14, 15. Right? The filter counts explicit lines only, but `--sort 'sum("…")'` sums explicit+implicit (Andvarius "sorts by 78" while its explicit sum is 70, call 15) so the order is by a different number than the filter: order is plausible, not strict by the filtered value. The tool also said ambiguous: "the bound sums the text and its plurals; not its forms 'reduced Rarity' (opposite)" (call 14). Worn items and equipment on characters (not tabs) are included; the site would not show those. Which tabs: call 15 rows name each; I did not run a --count tab on this query.

**T5 fire resistance.** Told: 510 Standard items have at least 30% fire resistance (explicit or scourge) (call 16, 526 across leagues), spread over 141 tabs; biggest: "~price 40 chaos (F) (Remove-only)" 37, 35 on no tab (worn/inventory on characters), Dump Tab 13, Gear (Remove-only) 13 (call 17). Only 30 rows read of 510. 2 commands. Right? The count is what the tool says; the clause is ambiguous (scourge line summed in, the site may leave it out, call 16), so the person should not trust the exact number. Too broad to act on; I did not narrow it as the person gave no gear slot.

**T6 maps.** Told: 83 Blighted maps of tier >= 14: 44 at tier 14, 22 at tier 15, 17 at tier 16 (call 19, 44+22+17=83). Tier 13 (63 maps) is outside the search. Commands 18, 19. Right? Count adds up and matches the typeline vocabulary in call 18 (T14 44, T15 22, T16 17). Caveat the tool printed: no map has a tier property in the store, the tier is read from the name, and a base without a tier in the name "lacks it, where the site gives such a map a tier" (call 18) – so the site might show more.

**T7 pick.** Ring: Fate Twirl Ruby Ring (110 life, 35 dex, 43%+26% fire res, +15% fire & chaos res crafted), in tab "3.25 RF Chieftain (Remove-only)" (calls 6, 20). Everything about it: ilvl 75, req level 64, an enchant, no price (call 20), and the stored body (call 23) which shows the slot x 6, y 7 and `verified:false`. Boots: Gale Spark Harpyskin Boots (30% MS, 124 life, 140 res, 4L) in "Soulrend PF (Remove-only)" (calls 12, 21). Shield: Tukohama's Fortress in "Facebreaker Totems (Remove-only)" (calls 13, 22). These are my picks from what the tools returned, not checked against any build. Where it stopped me: `show` names the tab but not the cell; the x/y exist only in `--body` JSON (call 23); and it did not say how old that tab's copy is.

## 3. Faults

1. **The sort number is not the filter number (call 15).** The query sums explicit rarity lines; `--sort 'sum("T")'` sums every source. Andvarius, 70 explicit + 8 implicit, shows `sorts by 78` and the row also prints a "70" – two numbers for one item. Keyboard person: hits it too.
2. **`show` gives the tab, not the position (calls 20–23).** "Where do I pick it up" needs the cell; x 6, y 7 are only in the `--body` JSON (call 23). Both.
3. **Which clause went unrun is not named in the human output (calls 6, 8).** "reported 1 with no stash meaning … status" – I had to read `--json` (19 KB, call 8) to see it is the seller-status filter. A person might guess; an agent needs another call.
4. **Stale and partial store is a header line, not a caveat on results (calls 3, 6).** 2940 locations never fetched, tabs 39d old; the answer says "total 22 matches". Only a 0-match answer repeats it (call 11). The site would show what is listed now. Both.
5. **Scope line says terms are "evaluated … in all leagues" (call 6) while the total counts Standard only** (term 4 "rarity >=20" 168 vs total 160, call 14); confusing. Both.
6. **The trade tour/describe is huge.** `--describe trade` printed 39716 bytes (call 7) and the harness persisted it; I did not read it. `--help` of search is 7.8 KB (call 5). Mostly agent.
7. **A refusal I met:** none; I was not refused by a command in this run.

## 4. What the tour and the help did not say

- That `search` with a bare query needs `--realm pc` because the store holds pc and poe2 (TOUR says "a store of two realms needs --realm"; I learned the exact form in call 9 by taking it from the printed route lines of call 6). A `--trade` URL carries it.
- That a trade URL translates to a query text you can edit (call 6 prints it; no help line says "copy the query line to change a clause").
- That `show` omits x/y (call 23 found them in `--body`).
- That worn/inventory items on characters are included in searches (call 14 rows "(equipment)", "(inventory)", "(guardian)").
- That the tab name "Remove-only" is a tab state and not a status of the item (no help said so; I assumed it from names in call 6).
- That `--count tab` gives (none) for character items (call 10).

## 5. The tool's own words

- TOUR.md: one line per verb, accurate for what I used (search, tabs, store characters, show). It names `CLI-REFERENCE.md`, `AGENTS.md`, `LIVE-TESTING.md`, a "mock-session skill", C-numbers (C20, C64…): I did not read or use them; the C-numbers also appear in help ("C93", "C105") and mean nothing to me. The first tour line ("cargo build … rebuild before a run") is not something I may run.
- `acq --help` (call 1): good one-liners; the `search` and `jobs` entries are paragraph-long. "ambiguous", "listing_only", "undecided", "lacked" are explained at use.
- Words I did not understand: "facts v8", "derivation 11", "totals v6", "a sitting" (call 16), "the park" (call 18), "the owner on ambiguous … verbatim: 'confirmed'" (call 11) – internal process text leaking into a user answer.
- `search --help` (call 5): right; very dense. Not used: --explain (offered in call 11), --next, --request, --print-request, --context.

## 6. What you would ask of the tool

- Print x/y (cell) in `show`'s default output (call 23).
- Name the unrun trade clause in the human summary, with its meaning (call 6, 8).
- Make `--sort` use the same sum as the filter or print both labelled (call 15).
- Say per result how many of its rows sit in tabs older than N days, and mark worn/inventory rows against "the site would not show this" (call 14).
- A `--count tab` option that also prints the tab's age (call 17).
- A shorter `--describe trade` index (call 7).

## 7. Left out

- Not tried: `--explain`, `--next` paging (so rows 21–22 of T1 and rows past 30 in T4–T6 were not printed), `price`, `shop render`, `reference`, `store events/refused`, `--membership removed/all` (so I do not know what the "removed" 6 items would add), `tabs --expand`, `--json` counts beyond call 8; all job verbs, as they need the daemon.
- Files opened that are not the three: none. The harness wrote `--describe trade` output (call 7) to a file in its own tool-results area; I did not open it. Work files written: none.
- Cut I would most want back: the full `--describe trade` text (call 7), which says what becomes of each site clause – the very thing the person asked ("where it might not show the same").
