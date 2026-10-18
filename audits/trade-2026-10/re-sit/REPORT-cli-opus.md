<!-- written during the run; call numbers are rows of calls.csv -->

## Before touching anything

### T0 — first account, before any command

**From the brief:** `acq` is the CLI of Acquisition, an inventory tool for Path of Exile; it fetches stash tabs and characters from the game's API through a local daemon, keeps them in a local store, and lets the person search, price and list what they have, including by a pasted trade-site search (the brief). One account, `GERWARIC`, a copy of the person's real collection with their prices; no login, no daemon, a mock API; commands needing the API will say they cannot reach it (the brief).

**From TOUR.md:**
- Verb groups: session (`auth`, `accounts`); API jobs that lazy-spawn a daemon (`profile`, `leagues`, `characters`, `character`, `stashes`, `stash`, `refresh`, `submit`, `demo`, `jobs`, `cancel`, `dash`); policy/plans (`policy`, `refresh --plan` offline, `refresh --apply` executes); pricing and shop "offline, no daemon" (`reference currency`, `price status|list|show|set|clear`, `shop render`); the store "no daemon, no network" (`tabs`, `store characters`, `search` incl. `--trade <URL|ID>`, `--count`, `--cross`, `--describe`, `show <id> [--body]`, `store status|events|refused|import|rebuild`); `daemon status|stop|reset-tripwire`, `version`.
- Needs: a login (OAuth, `acq auth`) and a daemon (lazy-spawned) + network for job verbs; nothing but the store for `tabs`, `search`, `show`, `store …`, `price …`, `shop render`.
- Every command takes `--json`; failure is `{"error":…}` exit 1, usage refusal exit 2.
- The tour's first line is `cargo build …`, which this run forbids (the brief); the tour points at CLI-REFERENCE.md, AGENTS.md, LIVE-TESTING.md etc., which I may not read.

**What I would not run without asking the person:** `auth`/`auth logout`, any job verb that spawns a daemon and spends API requests (`refresh --apply`, `refresh --all`, `submit`, `stash`, `demo`), `price set/clear` (changes their listings, though "with undo"), `policy set`, `store import`/`rebuild` (rewrites the store), `daemon stop/reset-tripwire`, `cancel`. In this world the brief permits pricing freely, but no task asks me to change anything, so I will not.

### T0 — the world as found (calls 1–6)

Store: 54.4 MB, 65 characters, 3444 tabs (every realm and league), 22721 items (+6 removed), 4 refused bodies, "league list never fetched" (call 2). One account, `GERWARIC#7694`, "last login 16m ago · not persisted" (call 3). Standard holds 3347 tabs: 405 fetched, 2926 never fetched, 16 folders; 20231 items. Most of the never-fetched are 2550 MapStash (61 fetched) and 502 UniqueStash (65 fetched) entries (call 4). Tabs were fetched 40d ago (call 4). Characters were fetched 37–46d ago; 50 of 65 were fetched, 10 of those with empty bodies (call 5). Pricing in Standard: 21311 items, 492 priced by the game, 39 by hand, 50 conflicts (call 6). Nothing changed: every call was a read.

## The tasks

### T1 — a ring

**To the person:** "Your friend's ring search finds **22 rings** of yours in Standard (call 8, rows 21–22 in call 11). All are rares. Examples: Pain Nail Diamond Ring in tab *MARKET* at x2,y1 (56 life, 45 dex, 25% chaos); Doom Circle Amethyst Ring in *3.25 Siege Ballista (Remove-only)* x11,y10 (95 life, 54 dex, 22% chaos); Fate Twirl Ruby Ring in *3.25 RF Chieftain (Remove-only)* x6,y7 (110 life, 35 dex, 84% fire); Death Circle Unset Ring worn by VVickedBossingServices. The full list with tab and x/y is calls 8 and 11.
The tool says it ran the site's search **exactly**. It reports one clause as not meaningful for your stash: the seller's online status (call 8).
**A buyer can see only 1 of the 22**: the Pain Nail Diamond Ring in the public tab *MARKET*. It has no price set, so it shows up unpriced. 20 sit in private tabs and 1 is on a character (call 14, call 15).
Without the dexterity requirement you have **91** matching rings, so dropping it adds **69** (call 16 lists the 69, call 17 counts 91). Of the 91, 6 are in public tabs. 3 of those 6 are priced by the game, in tabs named *~price 2222 chaos* and *~price 8888 chaos* (call 17, call 18).
Everything here is as of the last fetch, 40 days ago for most tabs and 3 days ago for some. 2940 locations were never fetched, and the tool says nothing about what they hold (call 8)."

**Commands:** calls 8, 9 (it got mixed in with call 10, see Left out), 11–18. 10 calls for T1.
**Right?** I believe so. 22 + 69 = 91, which is consistent (calls 8, 16, 17). The life pseudo counts Strength/2: Grim Grasp shows 87.5 from 76 life + 23 Str (call 8). As far as I know the site's pseudo life does the same. It would be wrong if the site rounds the half or leaves Strength out. "Visible to a buyer" is my own reading of `tab.public=true`. The tool never says "a buyer sees this". It does not say whether unpriced items in a public tab are listed on the site. And "right now" cannot be answered from 40-day-old data.
**Where the tool stopped or misled me:** it has no single answer to "can a buyer see this?" I had to put it together from `tab.public` and `price.side`, and I found those field names only through a refusal (call 12). The rows of a cross-filtered search do not print the price (call 18).

### T2 — boots

**To the person:** "None of your boots meet the whole search: **0 matches** (call 19). It asks for armour ≥300 at 20% quality, ≥25% movement speed, ≥100% total elemental resistance and ≥75 life. No pair you own has both the armour and the resistances.
Pairs that drop the resistance requirement: Anarchy Hoof Titan Greaves, worn by BoomBoomBored (414 armour, 30% MS, 107.5 life). Entropy League Vaal Greaves, worn by mrs_flowers (353 armour, 30% MS, 128 life). Doom Sole Titan Greaves in *30 chaos (Remove-only)* x6,y10 (322 at q20, 28% MS, 83 life).
Pairs that drop the armour requirement: Gale Spark Harpyskin Boots in *Soulrend PF (Remove-only)* x7,y6 (140% ele res, 30% MS, 124 life). Ambush Spur Stealth Boots in *3.19 Death's Oath Occ* x7,y6 (101% res, 30% MS, 75 life) (call 22; counts 3 and 2 in calls 20 and 21).
On fidelity, the tool calls its translation **'ambiguous'** on one clause. Armour at 20% quality is figured 'within one of the site's own figure'; every other clause ran (call 19). I loosened armour to ≥299 and it added nobody who also has the resistances (call 22). So the ambiguity does not change the answer.
**A buyer sees nothing**, because nothing matched."

**Commands:** calls 19–22, 4 calls.
**Right?** I believe the 0 is right. The explain counts (calls 20, 21) and my loosened query (call 22) agree: two disjoint groups and no overlap. It would be wrong if `pseudo.total_ele_res` left out a kind of resistance the site counts, for example all-elemental mods on the armour boots. Those rows printed no resistance line at all (call 22), so I could not check.
**Where the tool stopped or misled me:** nowhere. On a 0 result the `offers` block (call 19) gave a direct way on. The near-miss rows still don't show what the boots *lack*: Titan Greaves rows print no resistance figure, so I can't tell how far off they are.

### T3 — a shield

**To the person:** "One shield of yours fits: **Tukohama's Fortress** (unique Ebony Tower Shield) in *Facebreaker Totems (Remove-only)* at x7,y3. It has 473 armour, +1 to maximum number of Summoned Totems, and 117 life (91 explicit + 26 implicit) (call 24). The tool says the translation is **'ambiguous'**, on the same armour-at-20%-quality rounding as the boots. Every clause ran (call 23). This shield's armour is 473 against a 100 minimum, so the rounding cannot matter here. **A buyer cannot see it**: it is in a private tab with no price (call 23)."

**Commands:** calls 23–24. Call 24 repeats call 23 without `--cross`, because a cross prints no rows. 2 calls.
**Right?** Yes, I believe so. It would be wrong if the site's totem filter wanted a value range the tool left out, but the clause printed no number. 26 items in the store carry the totem line (call 23), and only this one passes the other terms.
**Where the tool stopped or misled me:** `--cross` drops the rows, so I needed two calls to get both the location and the visibility.

### T4 — magic find

**To the person:** "His search is any item in Standard with at least 20% increased Rarity from explicit mods. It isn't limited to one slot. It finds **160** of your items (call 25).
**At 30% or more you have 27.** Best first, by the explicit rarity the search filters on (call 28, rest in call 27):
1. Andvarius Gold Ring, 70%: *~price 4444 chaos (Remove-only)*, unique-tab page b8286134ce, x2,y0
2. Andvarius Gold Ring, 53%: *~price 4554 chaos*, page e63d0f7d9d
3. Empyrean Gyre Sapphire Ring, 51%: *Small Jewelry (Remove-only)* x4,y2
4. Empyrean Eye Bone Ring, 51%: *|||||| MARKET |||||| (Remove-only)* x7,y4
5. Aurseize Steelscale Gauntlets, 50%: worn by VVickedWilliam
6. Greed's Embrace, 48%: *8 chaos (Remove-only)* x18,y5
7. Dire Torc Gold Amulet, 41% (57% with its implicit): *Small Jewelry*
8. Tempest Ward Nightmare Bascinet, 40%: *Items 4*
9. Ventor's Gamble, 39%: *Special Gear*
10. Fate Halo Royal Burgonet, 39%: *Lab Bases #1*
11. Reaper's Pursuit, 38%: *~price 4554 chaos*
12. Mind Locket talisman, 33%: *~price 40 chaos (F)*
…then 15 more at 30–32%, mostly Chayula amulets.
**Tabs** (call 26): *~price 4554 chaos* holds 5. *3.13 Golems, BL, Misc*, *Small Jewelry* and *~price 4444 chaos* hold 2 each. 12 other tabs hold 1 each. 3 are on characters.
Translation: **'ambiguous'** on one clause. The bound sums the mod and its plurals but not the 'reduced Rarity' form, and 'read the other way: adds 0, drops 0' (call 25). So in your stash the difference changes nothing.
**A buyer can see 5 of the 27**: the five in the public tab *~price 4554 chaos*, priced in game (calls 26, 29). 19 are in private tabs and 3 are on characters."

**Commands:** calls 25–29, 5 calls.
**Right?** I believe so. One caution on 'best': my first sort, `sum("…")`, also counted implicit rarity (call 27). Sorted that way, a 32%+11% Ventor's ranks above the 38% Reaper's Pursuit. Call 28 sorts by the explicit sum the search filters on. Which order is 'best' for mapping is a judgment call: total rarity is what a player gets. I gave the explicit order and named the implicit where it matters.
**Where the tool stopped or misled me:** the `--sort` help shows `'sum("T")'` (call 7) but doesn't say that it counts every source. I learned that only from the row's 'sorts by …' label (call 27). The trade line says 'The site's sort is reported, never applied' (help, call 7), but no sort was reported for this search. I could not tell whether the site's URL carried one.

### T5 — fire resistance

**To the person:** "The search is a weighted sum: explicit '+#% to Fire Resistance' plus the scourge version of the same mod, ≥30 in total, in any slot. It finds **510** of your items in Standard (call 30). By kind: 94 rings, 85 helmets, 60 chests, 59 boots, 54 amulets, 54 belts, 47 gloves, 26 shields, 13 quivers, a few weapons (call 31). They are spread over 141 tabs, plus 35 on characters. The biggest piles are *~price 40 chaos (F) (Remove-only)* with 37, then *Dump Tab*, *Gear (Remove-only)* and *Rings (Breach)* with 12–13 each (call 31). The first 40 rows with places are in call 30.
Note what it does **not** count: 'Fire and Cold Resistances' and 'all Elemental Resistances' lines don't add to this sum. Only the plain fire-res mod does (rows in call 30, clauses in call 32).
Translation: **'ambiguous'** on the scourge half. The tool counts scourge fire res 'the site may leave it out', and reading it the other way 'adds 0, drops 0' (call 30). So for your items the result is the same either way.
**Buyers can see 51**: those are in public tabs. 13 of them carry an in-game price, and the other 38 are public but unpriced. 424 are in private tabs and 35 are on characters (call 31)."

**Commands:** calls 30–32, 3 calls.
**Right?** The count is consistent across calls 30–32 (510). What could make it wrong for the person: they may expect hybrid and all-res mods to count, and the site's stat would not count them either, so the tool is faithful there. The JSON lists clauses `stats.1`, `stats.1.0`, `stats.1.1` but no `stats.0` (call 32). Either the URL's first stat group was empty and silently passed over, or it is missing; the tool does not say which.
**Where the tool stopped or misled me:** 510 rows can't be read as an answer, so the useful summary had to come from `--count` (call 31). The tool did that well. To see the clause list I needed `--json` (call 32), because the human form prints only the non-exact clauses.

### T6 — maps

**To the person:** "His search is Blighted maps of tier 14 or higher. You have **83** of them, all normal (white) and all in two tabs: **69 in *Blight Maps*** and **14 in *Blight***. **By tier: 44 × T14, 22 × T15, 17 × T16** (calls 33, 35). Each is a separate map, not a stack (call 34: no item has a stack size). You also have 63 Blighted T13s just under his cut (call 33, the term-2.1 vocabulary).
**None are visible to a buyer**: both tabs are private and nothing is priced (call 33).
Translation: **'ambiguous'** on tier. The tool reads the tier from the map's base name ('Blighted Map (Tier 14)'). A map whose base name has no tier would have none here, where the site would give it one (call 33).
One more caution: 2489 of your 2582 map-stash sections were never fetched (call 33 scope line). Blighted maps kept in the map stash would not be counted."

**Commands:** calls 33–35, 3 calls.
**Right?** For what is fetched, I believe so. 44 + 22 + 17 = 83, consistent over calls 33–35. It would be wrong if blighted maps sit in the unfetched map-stash sections. The tool cannot see those offline, and filling them needs the API this world does not have (the brief).
**Where the tool stopped or misled me:** nowhere. But the ambiguity note doesn't say whether any of *these* maps were affected. Every base here names its tier (call 35), so I take it none were.

### T7 — a unique

**To the person:** "I couldn't find your Loreweave. Nothing the tool has fetched carries that name: not live, not removed, in no realm or league (call 36 searched names, call 37 searched any text across every realm, live and removed: 0).
Your two unique tabs in Standard are *~price 4554 chaos* (370 items fetched) and *~price 4444 chaos (Remove-only)* (160 items fetched) (call 38). Both have body armours in them, 27 and 12 (call 39). Loreweave is an Elegant Ringmail, and the only uniques on that base I found are two Geofri's Sanctuary: one in *~price 4554 chaos* at x76,y0 and one in Allflame (call 37).
But 437 of your 521 unique-stash sections were **never fetched** (call 36 scope line), and the store is 40 days old (call 4). So it may well be in a section the tool never read, or it was moved there after the last fetch. Finding it needs a refresh of the unique tabs against the game's API. That can't be done in this world (the brief), and outside it I'd ask you before spending requests."

**Commands:** calls 36–39, 4 calls.
**Right?** The 'not in the store' half I hold to be right: three searches agree. I can't say where it is.
**Where the tool stopped:** two places.
- It told me what share of unique-stash sections was unfetched, but not *which* ones (by page name, e.g. 'Body Armours').
- `acq tabs --expand` would list all 3347 tabs (call 4), far past the door's 60,000-byte cut, and `tabs` has no filter by type.
So I couldn't say 'the Body Armours page of tab X was never fetched'. The 0-match answer explained itself well: 'never fuzzy-matched', offers to explain (call 36).

### T8 — what I'd take

**To the person:** "One thing first: all of this is in **Standard** (calls 8, 22, 24), and Standard items can't go into a new league. If 'mapping tomorrow' means the new league, none of it comes with you. For mapping in Standard, I'd hand you these:
- **Ring: Fate Twirl Ruby Ring**, in tab *3.25 RF Chieftain (Remove-only)* at x6,y7. It's the highest-life ring that meets the whole of your friend's ring search (calls 8, 41).
- **Boots: Gale Spark Harpyskin Boots**, in *Soulrend PF (Remove-only)* at x7,y6. 30% movement speed, 124 life, 48/44/48 fire/cold/lightning resistance, 586 evasion at 20% quality, 4-link G-G-G-G. They need level 78 and 153 Dex (call 44). They meet every part of the boots search except the armour ≥300: they are evasion boots (calls 21, 22). No pair of yours meets all of it (call 19).
- **Shield: Tukohama's Fortress**, in *Facebreaker Totems (Remove-only)* at x7,y3. It's the only shield that meets the totem-shield search (call 24). It suits only a totem build.
'Remove-only' tabs still let you take items out.

**Everything known about the ring** (calls 41–43): Fate Twirl, Ruby Ring, rare, identified, item level 75, requires level 64. Its mods:
- enchant: 'Your Meteor Towers create Burning Ground for 3 seconds on Hit'
- explicit: +35 Dexterity, +110 maximum Life, +44 maximum Mana, +43% Fire Resistance
- crafted: +15% Fire and Chaos Resistances
- implicit: +26% Fire Resistance
That totals 84% fire res (call 8) and 15% chaos res. Nothing in the item went unread (call 41).
It has no price, by hand or in game. Its tab reads no price and is **not public**, so no buyer sees it (call 42). The stored body says the same thing verbatim (call 43). It was last seen there **40 days ago** (call 41). If you moved it since, the tool would not know."

**Commands:** calls 40–44, 5 calls (plus the T1–T3 searches the picks rest on).
**Right?** The places and facts are as the tool printed them. The picks are my judgment: highest life among full matches for the ring; best resistances and life with full movement speed for the boots. Someone who weighs armour over resistances, or Dex over life, would pick differently. What would show me wrong: a refresh finding the items moved.
**Where the tool stopped or misled me:** the `price` row in `show` reads 'none (none)'. The doubled word didn't tell me what the second 'none' is (call 41). `price show` explained it (call 42).

## Faults

1. **No answer to "can a buyer see this?"** The trade header says 'the site shows public listings, so the search is the site's and the items are yours' (call 8). But the answer never splits the matches by whether a buyer could see them, and no row shows `tab.public` or a price unless the query names it. I had to know the field names, which I found only through a refusal (call 12). Then I had to put the answer together from `--cross tab.public,price.side` (calls 14, 17, 29, 31) and a second filtered search to get the rows (calls 15, 18). It should give, beside a trade search, the count and rows a buyer would see, and say whether the site lists unpriced public items. *A person would hit this too.* It's the second half of every T1–T6 question.
2. **Unfetched sections can't be named in bounded output.** Scope lines say '437 of 521' unique sections were never fetched (call 36), but not which ones. `tabs` lists everything with `--expand` (3347 tabs, call 4), has no type or fetched filter, and would pass the door's 60,000-byte cut. So for T7 I couldn't tell the person *which* page of the unique stash to refresh. *Person and agent.*
3. **One unknown name in `--describe` refuses the whole request** (call 12, exit 1). The refusal lists every known name, which was enough to go on and is in fact how I found `tab.public`, `price.side` and `priced`. But the two good names I'd asked for were not answered. *Person and agent.*
4. **`--sort 'sum("T")'` sums every source** (call 27: 'sorts by sum(line(…).arg1) 47' for 39 explicit + 8 implicit). The trade term it seemed to mirror sums only explicit. The help gives `'sum("T")'` as the example (call 7) and says nothing about sources. A 'best first' list came out in a different order from the search's own figure until I sorted by the full `line(… source=explicit)` form (call 28). *Person and agent.*
5. **The trade clause list may skip a clause silently.** In T5 the clauses run `stats.1`, `stats.1.0`, `stats.1.1`, with no `stats.0` (call 32). Either the first stat group was empty or it is missing, and the tool doesn't say which. Low confidence. *Person and agent.*
6. **The tour assumes a source checkout.** Its first line is `cargo build …`, and it points to CLI-REFERENCE.md, AGENTS.md, LIVE-TESTING.md and skills (TOUR.md). Someone handed only binaries has none of these. *Mostly agents, and anyone without the repository.*
7. **`price  none (none)`** in `show` (call 41): the second word goes unexplained until `price show` (call 42). *Both.*

## What the tour and the help did not say

- That `tab.public`, `price.side` and `priced` exist and are how to ask about visibility. Expected in `search --help` (call 7) or the tour's search line. Learned from the refusal in call 12; the definitions came in call 13.
- That `--trade` combines with `--count`/`--cross`. Expected in the `--trade` help (call 7). Learned by trying, call 14.
- That `-term` matches items that *lack* the value as well as those that fail it. Expected in the `--help` QUERY line (call 7). Inferred in calls 16–17 (22 + 69 = 91).
- That `sum("T")` takes every source. Expected beside `--sort` (call 7). Learned from the row label, call 27.
- Whether the trade site lists unpriced items in public tabs. Expected in the trade header (call 8). Never learned.
- How unique-stash sections are named and how to list the unfetched ones. Expected in `tabs` (call 4) or the scope line. Never learned (T7).

## The tool's own words

**Right and useful:**
- The trade header: fidelity word, 'over …' line, each ambiguous clause with 'read the other way: adds 0, drops 0' (calls 19, 25, 30, 33). That last phrase settled every fidelity question in one line.
- 'nothing here says what they hold' under the never-fetched count (every search).
- 'never fuzzy-matched' and the offers on a 0 result (calls 19, 36).
- `price show`'s 'reads none, not public' (call 42).
- The tabs footer explaining '(Remove-only)' (call 4).

**Misleading or unclear:**
- 'terms — each term counted alone over all live pc items in all leagues, never over the matches'. It's accurate, but the term counts (e.g. 30 for rarity ≥30, call 29) sit next to a total of 27 and invite a misreading.
- 'none (none)' (call 41).
- 'exact — every clause with a stash meaning ran' (call 8) reads as a promise of the same result as the site. It is the same *search*, over a 40-day-old store.

**Words I did not understand:** in the basis line, 'intent 33', 'derivation 12', 'facts v8' (every search). `listing_only` (call 32). 'granted … item-granted skill(s)' (call 2). The tour's C-numbers (C11, C20, C38, C64, C84, C88) and 'invariant 4'.

**Read and never used:** `--explain` beyond calls 20–21; `--context corpus`; `--print-request`/`--request`; `--next` beyond call 11; `--membership removed`; `show --against`; every job, policy, refresh, shop and price-writing verb.

## What you would ask of the tool

- A `visible`/`listed` column and count beside every `--trade` answer: public tab + game price, as the site would show it (calls 14, 18).
- Rows beside a count: `--count` with `--limit` rows, so location and tally come in one call (calls 23–24).
- The effective price in every row (call 18).
- `--describe a,b,c` answering the known names and naming the unknown one (call 12).
- `tabs --type UniqueStash --unfetched`, naming the sections never read (calls 4, 36).
- Say in the `--sort` help that `sum("T")` spans sources, and show the `source=` form (call 27).
- In a trade answer, every clause the URL held, including an empty one (call 32).

## Left out

- **Not tried:**
  - `price list`, `shop render`, `store events`/`refused`, `tabs --expand` (too big for the door), the bare `--describe` index (~47 KB)
  - any job verb, `auth`, `refresh`, `policy`: no API here (the brief), and nothing asked for them
  - checking T8's picks with `show --against`.
- **Call 10** (`search --describe trade`) ran, but I never read its output. My harness put the combined output of calls 9–10 in a file outside W instead of showing it. I did not open that file and reran call 9 alone as call 11. So the per-clause translation reference went unread.
- **Files opened** other than BRIEF.md, TASKS.md, TOUR.md: none.
- **Files written:** none besides this report (no drafts, no work files).
- **The one thing I'd put back:** reading `--describe trade` (call 10), to know from the tool rather than by inference how each site clause, including a missing `stats.0`, becomes a term.
