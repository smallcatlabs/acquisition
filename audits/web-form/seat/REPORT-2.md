# Report — the page seat after the search form's series

Run on 2026-11-23 from `/Users/tom/seat-worlds/web-form/page-opus` (W).
Pane tools are named in each quote: `get_page_text` (GPT), `find`,
`read_page` (RP), `javascript_tool` (JS), `screenshot` (SS).

## Before touching anything

I read `W/BRIEF.md` and the section of `W/README.md` headed "The tour"
(printed with an `awk` range in a shell, which printed only that section),
and the two one-line address files. I listed `W` itself once (`ls`).

From the tour I expected: `acq web` gives "the search as a page shaped like
the trade site's", a trade-site URL run "as --trade runs it", each answer
under the form "filled with what was asked, to edit and run again"; "the
number a form would match beside its button as it changes"; saved searches
"saved from any answer, opened, renamed and cleared"; stash tabs and
characters as pages "to list, tick, plan and apply"; and "the language's
index, linked from the query box, each entry and block a page of its own".

What I meant to do first: open the collection address, read the nav, and
read the Search page's controls as text before clicking anything.

## The tasks

### T0 — Orient

Opened `http://127.0.0.1:56965/26d4b9b08cfc2af25e6cff5ca026d4d1/` (from
`W/web-collection.out`). The header (SS) shows links **Search · Saved ·
Stash tabs · Characters · Jobs** and, at right, "account GERWARIC#7694".
`find "link"` listed those five hrefs plus "Clear" (`.?realm=pc`) and "The
language's index: every entry by its name and one line" (`describe?from=.`).

What each page offers (GPT on each, nothing clicked but the nav links):

- **Search** — a top row "Search Items" box, a box for "A trade site
  search: its URL, its id or its text" with a button "Run the site's
  search" and "Matches: –" beside it, and the note "translated here, sent
  nowhere; a URL's realm is the search's". Then Realm (pc/xbox/sony/poe2),
  "Show this realm's leagues", League ("every league", Allflame (tabs on
  disk), …, Standard (tabs on disk), …), Membership (live/all/removed).
  Collapsible filter groups as on the trade site: Type Filters (Item
  Category, Item Rarity, "group on" tick), Weapon, Armour, Socket,
  Requirements, Map, Heist, Sanctum, Miscellaneous; Stat Filters with a
  Group select (And, Not, If, Count, Weighted Sum, Weighted Sum v2,
  "Crucible Passive Tree Path (unsupported)", "Mercenary Skill Group
  (unsupported)"), "Remove group", "+ Add Stat Filter", "Add Stat Group".
  Then Show 10…200, "Results as cards / a table", "Or count by" with the
  hint "keys as the CLI's count takes them: rarity, league, tab, class,
  category, line:resist,life", a **Search** button with "Matches: –" and
  "Clear". Below: "The query, in the language" (a second box with its own
  Show / Results as / "Run this query" / "Matches: –"), and at the foot
  "Show the fields this search does not run" with "A field with no mark
  runs as the trade site's; a mark says it is read differently here, or
  not run."
- **Saved** — "No saved searches: save one from any search's answer, under
  its query."
- **Stash tabs** — realm/league pickers, "Fetch the league list", "List
  this league's stash tabs", then "3347 tabs in Standard: 405 fetched, 2926
  never fetched, 16 folders (never fetched); 20231 items in tabs
  (characters apart); …; stash listing 73d ago", an "Items matching" box
  with Show, "Refresh ticked", and a table Tab / Type / Items / Fetched with
  an "Open" link per tab and a type filter.
- **Characters** — "41 characters: 41 fetched (10 with empty bodies); 1080
  items", same "Items matching" / "Refresh ticked", a table Character /
  Level / Items / Fetched.
- **Jobs** — "1 job: 0 running, 0 waiting, 1 done." with one row "The
  account's profile, for seat · done". (On GERWARIC's server this names
  the other account, `seat`; noted below.)

What I would do first: the Search page's form, Type Filters → Item
Category = Ring, Rarity = Rare, League = Standard, then a stat filter.

Coming back: each nav link reopened its page as it was (nothing was
changed yet).

### T1 — A search from the form

How I found each control: `find "combobox"` listed the selects by their
current value (League "every league", two "Any" under Type Filters, and a
hidden-looking "Instant Buyout and In Person" I never saw on screen). I set
**League** = "Standard (tabs on disk)", **Item Category** = "Ring", **Item
Rarity** = "Rare" (form_input, then SS to see them shown). For life I
clicked "+ Add Stat Filter" (SS) and typed `maximum life`; a dropdown
listed, in order (SS): "+# total maximum Life pseudo", "+# to maximum Life
explicit", "#% increased maximum Life explicit · alone exact · with a bound
ambiguous", "Minions have #% increased maximum Life …", and more. I picked
**"+# to maximum Life explicit"** (the line the game prints on a ring, and the
id the T4 text uses). A row appeared: a tick "on", the label, `min` / `max`
boxes and "explicit"; the min box had focus and I typed `70`.

Beside Search before pressing (SS, after scrolling down 3 ticks): **"Matches:
101"**, with "Clear" and a new box "Add a column" at right. The query box
under "The query, in the language" was still empty.

Pressed Search. The page became "Answer · Acquisition" at
`/search?query.term=&realm=pc&league=Standard&…` — a URL thousands of
characters long carrying every empty field of every filter group (tab note).
What stood with the answer (GPT, SS):

- "**101 matches**"
- "from the query JSON: exact — every clause with a stash meaning ran as the
  site reads it"
- "Over the items the store has fetched, priced or not — what it never
  fetched is the scope's: the site shows public listings, a tab's being
  tab.public, so the search is the site's and the items are yours"
- "Link: the search on the trade site" (a link; see T4)
- "Note: the search names no status, and the site fills what a search leaves
  unsaid with its own, its status among them"
- Query: `league=Standard category="accessory.ring" rarity=rare
  sum(line("# to maximum Life" source=explicit).arg1)>=70`
- Scope: "account GERWARIC#7694 · pc · live · 22623 items · 546 locations
  fetched / 2940 locations never fetched: MapStash 2489 of 2582, UniqueStash
  437 of 521, character 14 of 59 …"
- Total: "101 matches (the oldest fetch 76d ago, … the newest 73d ago, …)",
  "2940 locations were never fetched: nothing here says what they hold",
  "these tables read release 3.30 and no character here was fetched from it,
  the newest from 3.29 (73d ago): items may show earlier wording the tables
  do not read; a fetch cures it"
- "as a table · by stash tab · by character", then 10 cards (first:
  "Damnation Circle Ruby Ring · rare · frame rare", +78 to maximum Life …,
  "pc · Standard / TheFireworksCompany (equipment)"), each card ending with
  the matched value, e.g. `sum(line("# to maximum Life"
  source=explicit).arg1) 78`.
- "rows 1–10 of 101 shown, as they lie in their place, top row first and
  left to right (no sort asked) · press a number on a card to order the rows
  by it · more"
- A Terms table: "each term counted alone over all live pc items in all
  leagues, never over the matches": league=Standard 21311 matched; category
  1133 matched, 1 lacked, 5 undecided; rarity=rare 5769 matched / 8600 failed
  / **8254 lacked**; the life term 762.
- A basis line, then "Saved, this keeps the site's search as given,
  translated again each time it opens." with "Name" and "Save this search".

The form above the answer came back filled (JS over the inputs): league
Standard, category accessory.ring, rarity rare, stat
`explicit.stat_3299347043` min 70; and the language box was now filled with
the query above. The hidden save field holds the search as trade-site JSON
(`trade={"league":"Standard","query":{…"explicit.stat_3299347043","value":{"min":70}…}}`).

**Rings: 101** (from the "101 matches" heading, GPT; the same number the
button showed before pressing). Expected: a number in that range; I can't
check it except against T4/T9.

### T2 — Change the search on the page

On the answer page the form stood filled as asked; beside its Search button
the number read "Matches: –" (not 101) until I changed something (SS).

**First attempt — as a player would build it (pseudo totals).**
"Add Stat Group" opened a row of buttons "And · Not · If · Count · Weighted
Sum · Weighted Sum v2" (SS); I pressed **Weighted Sum**. A second group
appeared with group min/max, "Remove group", "+ Add Stat Filter". I typed
`to fire resistance`; first offer "+#% total to Fire Resistance pseudo",
then "+#% to Fire Resistance explicit", "+#% to Fire and Cold Resistances
explicit", … (SS). I picked the **pseudo total** for fire, then for cold
("+#% total to Cold Resistance pseudo" was the only offer for `total to cold
resistance`). Each line got min/max and a **weight** box. I set weight 1 and
1 and group min 60.

The number beside Search followed, but oddly: after adding the empty fire
line it read **"Matches: 50"**; after weights and min 60 it read (JS on the
count element, then zoom SS) **"Matches: 101 · not the site's search: 3 of
its clauses not run"**. The dropdown had given no mark on the pseudo lines.

My first Search press missed: the button had slid left as the count text
grew ("Matches: 101 · not the site's search: …"), so a click where it had
stood landed on the text. Second press ran. Answer (JS innerText of
`main`):

- "101 matches", "not the site's search: 3 clauses not run"
- "from the query JSON: PARTIAL — 3 clauses not run, so this is not the
  site's search"
- "Note: the link carries the query as asked: the clauses not run here run
  there"
- Status / Clause / Says: "NOT RUN stats.1 unsupported whole: its filter
  `stats.1.0` is no line"; "NOT RUN stats.1.0 its group `stats.1`
  translates whole or not at all, and is unsupported"; same for `stats.1.1`.
- Query: unchanged from T1 — the resistance group was dropped.

Switching the group to **Weighted Sum v2** (form_input) left the count at
"Matches: 101 · not the site's search: 3 of its clauses not run".

**Second attempt — explicit lines.** I set the group back to Weighted Sum,
pressed Remove on both pseudo lines (the count went to "–" each time), and
added "+#% to Fire Resistance explicit" and "+#% to Cold Resistance
explicit", weight 1 each, group min 60. Count: **"Matches: 4"**, no
not-run note. Search (third press needed: my coordinate press missed again
because the page coordinates I computed with JS were in CSS pixels, not the
screenshot's — my error, not the tool's). Answer (JS innerText):

- "4 matches", "from the query JSON: exact — every clause with a stash
  meaning ran as the site reads it"
- Query: `… sum(line("# to maximum Life" source=explicit).arg1)>=70
  sum(line(("#% to Fire Resistance" source=explicit) or ("#% to Cold
  Resistance" source=explicit)).arg1)>=60`
- Terms row 4: 193 matched, "→ #% to Cold Resistance (951) · #% to Fire
  Resistance (917)".

So with explicit lines it ran, but this is **not** what the person asked:
an implicit fire resistance (a Ruby Ring's, e.g. the first T1 card's "+33%
to Fire Resistance implicit") and hybrid "+#% to Fire and Cold Resistances"
lines are not in the sum. The form's way to ask the total — the pseudo line
it offers first — is not run in a weighted sum.

**Take one line out.** Pressed Remove on the cold line. Count beside
Search: **"Matches: 0"** while the answer below still said "4 matches" (no
mark that the answer no longer matched the form). Search →
"**0 matches**", "exact", Query `… sum(line("#% to Fire Resistance"
source=explicit).arg1)>=60`.

Did the number follow me: yes after each typed value or line chosen, but it
showed "–" after a group-type change or a Remove until the next edit, and
showed 50 for a weighted group with no weights or min (a group that, once
filled, was not run at all). Surprises: the pseudo weighted sum refused
whole; the Search button moving under the pointer as the count grows; the
stale answer under a changed form.

Coming back: each answer page came back with the form exactly as I left it
(SS of the top of the page after each run).

### T3 — What the form says of itself

Read without running anything (JS over `input[type=checkbox]`, every
`[title]`, and the `textContent` of each filter group, since collapsed
groups give no `innerText`):

- **The foot note** (GPT): "Show the fields this search does not run" (a
  tick, off) beside "A field with no mark runs as the trade site's; a mark
  says it is read differently here, or not run."
- **Visible marks** after a field's name: "**ambiguous**" (Armour, Evasion,
  Energy Shield, Ward, Quality, Map Tier), "**unsupported**" (Ultimatum
  challenge/reward/required item/reward unique; Transfigured Gem; Imbued
  Gem; Unidentified Tier; Fated; Foreseeing; Intangibility; Alternate Art;
  Map Completion Reward; Map Series, …), "**listing only**" (Status
  "Instant Buyout and In Person", Seller Account, Collapse Listings, Listed,
  Sale Type, Gold Fee, Buyout Price). In the Stat group select, two options
  are named "(unsupported)": "Crucible Passive Tree Path (unsupported)",
  "Mercenary Skill Group (unsupported)". In the stat dropdown, lines carry
  notes such as "explicit · alone exact · with a bound ambiguous".
- **Tooltips** (title attributes, read by JS; I saw no other way to read
  them in the pane), e.g. Status: "Online, available or securable is the
  seller's state, not the item's, so it means nothing in your stash."; DPS:
  "Damage per second at 20% quality, as the site figures it: within one of
  the site's own figure, which the display's rounding can move."; Class:
  "No item of your account carries a class requirement, so this is not
  run."; Map Tier: "The tier read from the map's base name; a map whose base
  names no tier has none here, where the site gives it one."; several "No
  item shows it; not run."; Crucible: "… whether the site means its tree or
  a chosen node has not been settled."; Buyout Price: "The market's price;
  ask for your own prices directly."
- **A tick box that changes reading**: under Armour Filters, "**assume 20%
  quality**" (on by default; hidden fields `defences=displayed` and
  `defences=q20`). Every filter group has a "group on" tick; each stat line
  an "on" tick.
- **Nothing on the form marks pseudo stat lines** as not run inside a
  weighted sum (T2) — the not-run only showed beside the count once the
  group was filled.

**Hidden fields**: ticking "Show the fields this search does not run"
(find → click by ref) did not reload; two groups that were not shown before
appeared: "**Ultimatum Filters**" (all four fields "unsupported") and
"**Trade Filters**" (every field "listing only"). Before the tick the GPT
listing of groups had no Ultimatum or Trade group. The Status select
("Instant Buyout and In Person") is in the DOM even with the tick off
(`find "combobox"` saw it in T1) but I did not see it on screen.

**Emptying the form**: the "Clear" link beside the count (`href=".?realm=pc"`).
It went to `/?realm=pc`, "Search · Acquisition", and everything was reset
(JS): league "" (every league), category "", no stat lines, "Show the
fields…" unticked, count "Matches: –". So Clear also drops the league and
the show-fields tick; the answer went with it.

Coming back: after Clear there is no "back to my last search" on the page;
the browser history (or the saved searches, T5) is the only way.

### T4 — A search from the trade site

Where it goes: the box at the top of the Search page, placeholder "A trade
site search: its URL, its id or its query JSON" (find), with "Run the site's
search" beside it. I clicked it (by ref) and typed the JSON exactly as given.
Before running, beside its button (SS): **"Matches: 101"** and "translated
here, sent nowhere; a URL's realm is the search's". Pressed "Run the site's
search".

The page became "Answer · Acquisition" at
`/search?realm=pc&trade=<the JSON, URL-encoded>#answer` (tab note). It said
(JS innerText of `main`):

- "**101 matches**"
- "from the query JSON: exact — every clause with a stash meaning ran as the
  site reads it"
- "Link: the search on the trade site"
- Status / Clause / Says: "**reported** · status · Online, available or
  securable is the seller's state, not the item's, so it means nothing in
  your stash."
- Query: `league=Standard category="accessory.ring" rarity=rare
  sum(line("# to maximum Life" source=explicit).arg1)>=70` — word for word
  T1's query; Scope and Total the same as T1 ("101 matches (the oldest fetch
  76d ago, … VVickedWitch (character); the newest 73d ago, … Dump Tab)"),
  same Terms table, same first cards.

**Compared with T1**: same 101, same query, same items in the first ten
cards. What it did not run: only `status` ("reported", not run). What it
ran: league, category, rarity, the life line.

**What happened to the form** (JS): it was filled from the JSON — League
"Standard", Item Category "Ring", Item Rarity "Rare", one stat line
`explicit.stat_3299347043` min 70, and the Status select set to "In Person
(Online)" (`online`); the trade box itself was emptied; the language box
held the translated query. So the trade text becomes an ordinary editable
form, as T1's form did.

**The way back to the trade site**: a link "the search on the trade site"
(find/JS): `href="https://www.pathofexile.com/trade/search/Standard/H4sIA…"`,
`target="_blank" rel="noopener"`. I did **not** open it. Its tail is
gzip+base64; I decoded it in the page with JS (no fetch) to
`{"status":{"option":"online"},"stats":[{"type":"and","filters":[{"id":"explicit.stat_3299347043","value":{"min":70}}]}],"filters":{"type_filters":{"filters":{"category":{"option":"accessory.ring"},"rarity":{"option":"rare"}}}}}`
with the league in the path. So it carries everything asked, status
included. What the page says goes with it and is left out: on this answer
only "Link: the search on the trade site" — no note. On T2's partial answer
the page said "Note: the link carries the query as asked: the clauses not
run here run there"; on T1's form answer, "Note: the search names no status,
and the site fills what a search leaves unsaid with its own, its status
among them". Nothing on any page says what the link leaves out (sort,
display, my "Show"/table choices); I would have expected one line on it.

Coming back: Back in history would return to the empty form; the answer's
own URL re-runs it (it is the trade text in the address).

### T5 — Keep a search

**Save.** At the foot of the T4 answer (SS): "Saved, this keeps the site's
search as given, translated again each time it opens." then "Name [ ] Save
this search". I typed `rare rings 70 life (trade text)` and pressed Save.
The page went to Saved (GPT):

> Saved searches · Back to the search · 1 saved search · the file at
> revision 1 · Name | Scope | Query | Revision | Last written | Id —
> "rare rings 70 life (trade text) | pc · live | league=Standard
> category="accessory.ring" rarity=rare sum(line("# to maximum Life"
> source=explicit).arg1)>=70 (from a site search: exact) | 1 | 0s ago via
> web | cd7f4ce903fd" · Rename · Clear

**Find and open.** Saved nav link → the name is a link
(`saved/open?id=cd7f4ce903fd&from=saved`; find). Opening it ran it at once
(URL `search?realm=pc&trade=…&saved=cd7f4ce903fd&saved_revision=1`). At the
top (GPT/SS): "Back to the saved searches" and **"This search holds what
the form below does not send: a search from the form leaves each of these
out. · query.status.option "online""**. The form was filled (Standard, Ring,
Rare, life min 70) and a **Status** row now showed "In Person (Online)" with
"listing only" beside it. Above the answer: 'Saved search "rare rings 70
life (trade text)", opened at revision 1 · the saved searches', then 101
matches, the same as T4. At the foot two save rows: 'Save over "rare rings
70 life (trade text)"' and "Save as a new search".

**Change one thing and run.** Life min 70 → 80; count beside Search
"Matches: 49"; Search → URL is now the long form URL with
`&saved=cd7f4ce903fd&saved_revision=1` and **no status** param. The answer
said (JS innerText): 'This answer's query is not the saved one's: the saved
query is league=Standard … >=70 over pc · live.', "**49 matches**", "exact",
and "Note: the search names no status, and the site fills what a search
leaves unsaid with its own, its status among them". So the status the
saved search held was dropped, as the top note had warned.

I saved that as a new search "rare rings 80 life (form)". Saved page (GPT):
"2 saved searches · the file at revision 2", the new row "rare rings 80 life
(form) | pc · live | … >=80 **(from a site search: exact)** | 1 | 0s ago via
web | 9dcbe9c5f0d2".

**What a saved search holds**: a name, a scope (realm · membership), the
query shown in the language, a revision, when and how it was last written
("via web"), an id — and underneath, the trade-site JSON (the save form's
hidden `trade` field in T1/T4, and the open-page note about `status`).

**Could I tell form from trade text?** No. Both rows say "(from a site
search: exact)"; the page saves a form search as site JSON too ("Saved,
this keeps the site's search as given" appears under a form answer). The
only difference I found is the status line, which shows only after opening
the trade-text one. The list does not show it.

Coming back: "Back to the saved searches" and the Saved nav link returned
to the list as left; reopening ran the saved query again.

### T6 — A tab as it lies, and in an order you name

**A currency tab.** On Stash tabs (league Standard already chosen) each row
has a tick box, the tab's name as a link, an "Open" button, type, items,
fetched (SS). The name links to a search
(`search?realm=pc&text=id%3A0ce277027d%20league%3DStandard&view.fields=icon`);
"Open" unfolds the tab in place (`tabs?…&open=0ce277027d#p-0ce277027d`). I
pressed **Open** beside "Currency 1" (CurrencyStash, 115 items, 73d ago).
Above the list (JS innerText): "Each place's items stand as they lie in their
place, top row first and left to right: open one by its name and press a
number on its card to order them by it." Columns: **Item · Base · Price ·
Location · Fetched · stack**. Order (SS, then JS for the far right): by
position, "x 0, y 0", "x 1, y 0", … to "x 110, y 0" (every currency slot is
y 0): Blacksmith's Whetstone, Scroll of Wisdom, Chaos Orb, Armourer's Scrap,
Mirror of Kalandra, … A line reads e.g. "Chaos Orb · Chaos Orb · none
(nothing applies) · Standard / Currency 1 · x 2, y 0 · 73d ago · 9548". The
**stack** column (9836, 7712, 9548, 8924, 1 …) lies past the right edge of
the 800-px pane (document 1328 px wide); I saw it only through JS, not on
screen without scrolling sideways. The first row's price says "none
(nothing applies: the price text in game is not where the index can see
it)".

**A gear tab, ordered by a value I named.** I followed the name link of
"Decent Gear" (PremiumStash, 37). Answer: "37 matches", Query
`id:1a4f77ce3c league=Standard`, "37 matching items in the table, as they lie
in their place, top row first and left to right (no sort asked) · open an
item by its name and press a number on its card to order the rows by it";
rows from "x 0, y 0" row by row (SS).

Where the pages let me name a value: an **"Add a column"** box. There are
two: one at the right of the form's Search button, one under the language
box's "Run this query". Typing `maximum life` offered the same list as the
stat filter: "+# total maximum Life pseudo", "+# to maximum Life explicit",
"#% increased maximum Life explicit", … (SS).

- **First try — the box by the form's Search button.** I picked "+# total
  maximum Life". It submitted the **form**, which was empty (the tab came
  in through the language box), so the tab was lost: "22623 matches",
  "Query (every item in scope)", "Link: none — the search names no league,
  …", "22623 matching items in the table, by pseudo.total_life, largest
  first, as asked" — Kaom's Hearts first, from every tab and league (SS).
  Nothing warned me that this box belongs to the form and not to the query
  that made the answer.
- **Second try — the box under the language box.** Reopened Decent Gear,
  typed `maximum life` there, picked the same line. URL gained
  `view.sort=pseudo.total_life&view.desc=true&view.value=pseudo.total_life`.
  Answer: "37 matches", same query, "**37 matching items in the table, by
  pseudo.total_life, largest first, as asked**", a new last column header
  "▼pseudo.total_life , ordered largest first" (JS; off-screen right), first
  rows Bramble Vise Titan Gauntlets (109), Empyrean Knot Coral Ring, Grim
  Crest Samnite Helmet, … (SS + JS).

So: the order is chosen by adding a column, and largest-first is assumed; I
saw no control for smallest-first on the page. Picking the column is the
only way I found to name the order without typing a query (besides
"press a number on a card", which needs cards and an item to hand).

Coming back: the Stash tabs page reopened with Currency 1 unfolded when I
used its `open=` URL; the "by pseudo.total_life" answer is in its URL and
came back the same.

### T7 — Abyssal sockets

All three over every league (League "every league"), pc, live.

1. **Form socket controls.** "Socket Filters" unfolds to "Sockets: min ·
   max · R · G · B · W · A" and "Link Groups: min · max · R · G · B · W · A"
   (SS; no tooltip on "A" — I read A as abyssal from the game). I typed 1
   in Sockets **A**. Count beside Search: "Matches: 73". Search → "**73
   matches**", "exact", Query `sockets.abyssal>=1`, "Link: none — the search
   names no league, and a site link is one league's: …", Terms
   `sockets.abyssal>=1 · 73 matched · 3355 failed · 19195 lacked`.
2. **A line the items display.** Stat filter, typed `abyssal socket`;
   offers (SS) "Has # Abyssal Sockets explicit", "Abyssal Troves in your
   Maps have …", "Socketed Rare Abyssal Jewels will be Consumed …", "Cannot
   have non-Abyssal sockets explicit", "Has # Abyssal Sockets implicit",
   "Has # Abyssal Sockets fractured". I added all three "Has # Abyssal
   Sockets" lines and set the group to **Count** with min 1 (an And group
   would have asked for all three at once). Count beside Search: 73. Search
   → "**73 matches**", "exact", Query `holds(line(("Has # Abyssal Sockets"
   or "Has # Abyssal Socket") source=explicit), line((…) source=implicit),
   line((…) is:fractured))>=1`. Terms: explicit 30 ("→ Has # Abyssal Socket
   (25) · Has # Abyssal Sockets (5)"), implicit 56, fractured 1.
3. **A query in the tool's language.** I learned the word from the
   language's index (link "The language's index: every entry by its name
   and one line" under the query box): the row "sockets.<word> · number ·
   how many sockets of one kind the item has, over every link group; GGG's
   letter for the word (red R, green G, blue B, white W, abyssal A) …" (JS
   over the index text). The form had also shown me `sockets.abyssal` as
   its translation in way 1. I typed `sockets.abyssal>=1` in the language
   box; "Matches: 73" under "Run this query" before running; Run → "**73
   matches**", Query `sockets.abyssal>=1`.

**Do they agree?** All three say 73. To see whether they are the same 73 I
edited the answer's address to `sockets.abyssal>=1 -line(("Has # Abyssal
Sockets" or "Has # Abyssal Socket"))` → "**0 matches**", with Terms
`sockets.abyssal>=1` 73 and the any-source line 73 ("→ Has # Abyssal Socket
(68) · Has # Abyssal Sockets (5)"). So every socketed item carries the line
and both sets are 73: the same items. Note the per-source counts (30 + 56 +
1 = 87) exceed 73, so some items hold the line from more than one source;
I did not look at which.

Coming back: each answer is in its URL; the index's "Back to the search"
link goes to the empty Search page (`/`) when the index is opened from the
empty Search page; from an answer it goes back to that answer (T8).

### T8 — The language's words

Where the pages told me: the link "The language's index: every entry by its
name and one line", under "The query, in the language" (on an answer page
that section is folded; I unfolded it, then followed the link). The index
page, "The language · Acquisition", opens (GPT): "This is the index, each
entry by its name and one line: name an entry for it whole (league,
pseudo.total_res, accessory.ring), a block for every entry in it (computed,
categories, trade), ~ and a pattern for the entries whose name or
definition it matches (~resist), or all for everything whole." with a
"Names: …  Describe" box. Then blocks as tables (Name · Kind · What it is):
fields, inside line( … ), inside linked( … ), values, composition,
operators, slots, counts, why an answer is what it is, an answer key by
key, categories, "trade — each clause a site search can carry, 151 rows",
"not built, refused by name", "limits, as the search states them". About
28,000 characters, 316 links (JS); each name is a link to its own page
(`describe?names=<name>&from=…`).

**A ring's total life**: `pseudo.total_life` — index row "pseudo.total_life
· total · the trade site's pseudo_total_life (`+# total maximum Life`)".
Its own page (GPT) spells it out: "the sum over the item's lines of 1 × arg1
of line("# to maximum Life") + 0.5 × arg1 of line("# to Strength") + 0.5 ×
… "# to Strength and Dexterity" … "# to Strength and Intelligence" … "# to
all Attributes"; …" and gives "e.g. pseudo.total_life>=60 -has:pseudo.total_life
undecided(pseudo.total_life)". (The explicit line alone, as the form wrote
it in T1, is `sum(line("# to maximum Life" source=explicit).arg1)`.) I ran
`league=Standard category="accessory.ring" rarity=rare pseudo.total_life>=70`
by editing the answer address: **130 matches** (T1's explicit-only 101, the
total counts strength and implicit life too).

**One thing the language can ask that the form cannot**: my own priced
items. The form's price fields are all "listing only" ("The market's price;
ask for your own prices directly."). The language has `priced` — "flag ·
whether the item carries an effective price: the game's note or public tab
name, or the owner's row, …" with "e.g. has:priced -has:priced
undecided(priced)". `has:priced league=Standard category="accessory.ring"`
→ **29 matches**, first price line "price 2222 chaos · game · from
tab/pc/72c292c282". (Also `note`, the item's own note text, and
`linked(red>=3 green>=1)` socket colours on one link group.)

What the page is like to read: complete, and every row is careful, but it
is dense and written for someone who already knows the tool. Sample: the
"computed values" header cites "(search/pseudo-stats/data/table-changes.csv),
over the C++ app's pseudomod tables, src/pseudomods.cpp at
acquisition-qt@946a4f51". There is no "how do I write a query" walk-through;
the examples ("e.g.") are on the entry pages, not in the index. A player
can find `pseudo.total_life` by searching the page for "life", but would
need the form's own translation to learn `line("…")` syntax.

**Did I come back to my search?** Yes. Opened from the saved ring search's
answer, the index's "Back to the search" pointed at that answer (with
`saved=cd7f4ce903fd`), and so did "Back to the search" on the entry page
for pseudo.total_life (JS on the hrefs). Opened from the empty Search page,
it points at `/`.

### T9 — The terminal and the page

How I found the command: the tour's line `acq search --trade <URL|ID> |
--trade-file <FILE|->`, then `acq search --help` through the door **[call
1]**, whose `--trade-file` entry reads "`--trade`, read from a file or from
stdin with `-`: the query JSON as the site posts it, … A `"league"` beside
the query is the search's league …". I wrote the JSON to
`W/drafts/t4-trade.json` and ran `search --realm pc --trade-file …` **[call
2]**: exit 1, "Error: 2 accounts are known and one is to be picked:
GERWARIC#7694, seat; name one with --account", with both full command lines
offered. I ran the first offer **[call 3]** (`--account 'GERWARIC#7694'`,
stdout saved as `W/work/t9-trade.txt`): exit 0, 526 ms.

What it printed:
- "trade   from the query JSON: exact — every clause with a stash meaning
  ran as the site reads it"
- "link      https://www.pathofexile.com/trade/search/Standard/H4sIAAAAAAAA_1WP0QqDMAxF_yXPIjIF0V8ZIqVmEuja0sYxKf33pfOl5im5NzncJIis-…"
- "reported  status  Online, available or securable is the seller's state,
  not the item's, so it means nothing in your stash."
- query `league=Standard category="accessory.ring" rarity=rare
  sum(line("# to maximum Life" source=explicit).arg1)>=70`
- "total   **101 matches** (the oldest fetch 76d ago, pc · Standard /
  VVickedWitch (character); the newest 73d ago, pc · Standard / Dump Tab)"
- the same ten first rows as the page, in the same order; the same terms
  (21311 / 1133+1 lacked+5 undecided / 5769 + 8254 lacked / 762).

**Agreement**: yes — same count, same query, same first ten rows, same
status note. **Same way back**: yes — the link is character for character
the href the page's "the search on the trade site" carries in T4 (compared
by eye against the JS read of the href). The CLI prints it as a bare URL;
the page as a link that opens in a new tab.

### T10 — As a player

Three things a player would want that I could not do, or not find:

1. **Ask "fire + cold resistance ≥ 60" the way the trade site does** — a
   weighted sum over the pseudo totals. The form offers exactly those lines
   first and then does not run the group ("unsupported whole: its filter
   `stats.1.0` is no line"), counting 101 instead. With explicit lines it
   runs but misses implicit and hybrid resistances (T2). (The language has
   `pseudo.total_fire_res` and `pseudo.total_cold_res`, so I suspect a sum of
   the two could be typed, but the form will not build it and I did not find
   an example of adding two totals on the pages.)
2. **See what a currency tab holds at a glance** — counts and worth. The
   tab view lists 115 slots by position with the stack count in a column
   past the right edge of the pane, and every price "none (nothing
   applies)". I found no way on the page to sort that tab by stack or show
   a chaos value per stack or a total for the tab (T6).
3. **Order what I see without leaving where I am.** On a tab opened by its
   name, the "Add a column" box I reached first belongs to the empty form
   and threw away the tab (22623 matches, T6). There was no smallest-first
   option, and no "clear back to my last search" after Clear (T3). Most
   of all, a player wants to price-check one item from its card. The only
   trade link is per search, and only when the search names a league.

## Faults, worst first

1. **"Add a column" next to the form's Search throws away the answer's
   scope.** (T6) I opened the Decent Gear tab by its name (an answer made
   from the language box, 37 matches). I typed `maximum life` in the "Add a
   column" box beside the form's Search button and picked "+# total maximum
   Life". The page ran the empty form: "22623 matches", "Query (every item
   in scope)", sorted by pseudo.total_life. It gave no warning that the tab
   was gone. It should have added the column to the 37 items shown, or the
   box should not stand beside an answer it does not belong to. The second
   box, under the language box, did it right.
2. **A weighted sum over the pseudo totals is not run, though the form
   offers those lines first.** (T2) Weighted Sum (and v2) of "+#% total to
   Fire Resistance pseudo" + "+#% total to Cold Resistance pseudo", min 60.
   The answer: "PARTIAL — 3 clauses not run", "unsupported whole: its filter
   `stats.1.0` is no line", 101 rings (the group was dropped). The page is
   honest about it, but it is the trade site's commonest use. Nothing on the
   stat dropdown marks the pseudo lines as unusable in a weighted group. The
   explicit-line substitute runs, but it misses implicit and hybrid
   resistances.
3. **The live count says something different from what the group does.**
   (T2) A Weighted Sum group with one pseudo line and no weight or min
   counted "Matches: 50". Once weights and min were filled the group was not
   run at all (101). After a Remove or a group-type change the count showed
   "–" until the next keystroke.
4. **The answer under a changed form is not marked stale.** (T2) After I
   removed the cold line, the count beside Search read "Matches: 0" while
   the answer below still read "4 matches", with nothing tying the two.
5. **The Search button moves under the pointer.** (T2) As "Matches: 101 ·
   not the site's search: 3 of its clauses not run" grows, the centred
   button slides left. A click where it had stood lands on the text and
   nothing runs.
6. **A tab's stack column is off-screen.** (T6) Opened in place, Currency
   1's table is about 1328 px wide in an 800 px pane. "stack", the number
   that matters for currency, is past the right edge. The sorted-by column
   ("▼pseudo.total_life") is off-screen too.
7. **A saved search does not say what it came from or what it holds.**
   (T5) The trade-text save and the form save both read "(from a site
   search: exact)". That the first holds `status "online"` shows only after
   opening it, and running it from the form then drops the status.
8. **The Status select shows a value the form will not send.** (T5) On an
   opened saved search, Status reads "In Person (Online)", while the banner
   says "a search from the form leaves each of these out.
   query.status.option "online"".
9. **Clear empties more than the search.** (T3) It also resets the league
   to "every league" and unticks "Show the fields this search does not
   run". There is no way back to the last search except browser history.
10. **Form answer URLs are thousands of characters long.** (T1) Every empty
    field of every filter group goes into the address
    (`…&query.filters.weapon_filters.filters.damage.min=&…`). They are hard
    to share and long in the pane's tab note.
11. **The collection's Jobs page lists the other account's job.** (T0) On
    GERWARIC's server: "The account's profile, for seat · done". Nothing
    says why one account's pages list another's job.

## What the tour and the pages did not say

- What the socket "A" box means: there is no tooltip or label on the page.
  The language index says "abyssal A".
- Which of the two "Add a column" boxes belongs to which query. They look
  identical; one runs the form and one runs the language box.
- That a weighted sum needs lines and not pseudo totals. This shows only
  after the group is filled, as "not run".
- What the trade-site link leaves out (sort, view, table/cards). The pages
  say what the link carries only when clauses were not run ("the clauses not
  run here run there").
- That three "Has # Abyssal Sockets" lines (explicit / implicit / fractured)
  need a Count group to mean "any of them". An And group asks for all three.
- The tour says saved searches are "renamed and cleared". On the list,
  "Clear" is a plain link (`saved/clear?id=…`) beside the name. Its text does
  not say whether it asks before clearing. I did not press it.
- The tour does not say that the CLI refuses without `--account` while this
  world holds two accounts. The error says so at once and offers both
  commands.

## The tool's own words

Words or phrases I did not understand, quoted:

- "Over the items the store has fetched, priced or not — what it never
  fetched is the scope's: the site shows public listings, a tab's being
  tab.public, so the search is the site's and the items are yours"
- "translated here, sent nowhere; a URL's realm is the search's"
- "none (nothing applies: the price text in game is not where the index can
  see it)"
- "10 counts have no link: a count's route is listed only when every route
  is asked for, which a page never does"
- "explicit · alone exact · with a bound ambiguous"
- "unsupported whole: its filter `stats.1.0` is no line"
- "reported" (as a clause status)
- "· frame rare" on every card
- The Terms columns "Lacked" and "Only together" (rarity=rare: 8254
  lacked)
- "basis store 861010a882b9 · build 9359c94733d4 · snapshot 1126 · facts v9
  · … derivation 12 · … totals v6 · defences v1 · currency v1 · notes v2"
- "Link: none — the search names no league, and a site link is one
  league's: a search id names none, and the query JSON names one as
  `league` beside its `query`"
- In the index: "(search/pseudo-stats/data/table-changes.csv), over the C++
  app's pseudomod tables, src/pseudomods.cpp at acquisition-qt@946a4f51"

## What I would ask of the tool

- Run a weighted sum (and count) over pseudo totals, or mark pseudo lines in
  the dropdown when the group they would join cannot run them.
- One "Add a column" per answer, acting on the answer shown.
- Keep the stack and sort columns in view, or let the table wrap.
- A smallest-first choice beside "largest first".
- A mark on the answer when the form above has changed since it ran.
- In the saved list, show whether a search came from the form or from site
  text, and any clause it holds that the form cannot show.
- Short form URLs (only the fields that have values).
- Show tooltips' "not run" reasons on the page, not only on hover.
- Say beside the trade link what it does not carry.

## Where I left the page for the terminal, and why

Only for T9, which asks for it: [call 1] `search --help` to find the
trade-file option, [call 2] refused (no `--account`), [call 3] the run. I
never needed the terminal to get past a page.

## What I read or ran outside the world

Nothing outside `W`. Inside `W`: `BRIEF.md`; the "The tour" section of
`README.md` (an `awk` range in a shell printed only that section, but the
command read the file to find it); the two address files; one `ls` of `W`.
I wrote `drafts/t4-trade.json`; `work/t9-trade.txt` was written by the
door's `--out`. The pages were reached only at the collection address and
by its own links, plus address-bar query edits on answer pages (T7, T8).
The trade link was never opened; I decoded its tail with JS in the page,
without fetching anything. The session that started me named a shared
notes folder. The brief forbids opening other files, so I did not read it.
One slip against the brief's "with your own file tool": I appended the
closing "Message to the person" with a shell heredoc. Every other part of
this report was written with the file tool, a task at a time.

## Left out

- The rehearsal account's pages (`W/web-rehearsal.out`): no task asked for
  fetching, so I never opened that address.
- Characters page items; "Refresh ticked", ticking, plans and applies.
- "Or count by", "as cards / a table" switches, "by stash tab · by
  character" views, the "Rename" and "Clear" of saved searches.
- Whether the abyssal per-source overlap (30 + 56 + 1 against 73) is right:
  I did not open those items.

## Which answers I would not bet on

- **101 rings** (T1/T4/T9) are what the store has fetched. The page itself
  says 2940 locations were never fetched (most map and unique tabs, 14
  characters), so the true number of rings owned may be higher.
- **4** rings for life + fire/cold ≥ 60 (T2): explicit lines only. The
  person's question with implicit and hybrid resistances would give more;
  the page would not run that.
- **73** abyssal-socket items (T7): the three ways agree and the set check
  gave 0, but the per-source counts summing to 87 is something I can't
  explain.
- **130** rings by `pseudo.total_life>=70` (T8): a different question from
  T1. It counts strength as half life, as the index says.

## Which claims rest on my own knowledge of the game rather than on the tool

- That the socket box "A" means abyssal (the index later confirmed it).
- That a player asks "fire + cold ≥ 60" as a weighted sum of the pseudo
  totals on the trade site, and that a Ruby Ring's implicit is fire
  resistance and should count.
- That stack size is the number a player wants from a currency tab, and
  that price-checking one item is a common need.
- That abyssal sockets come both as implicits (e.g. on some belts) and as
  explicits/uniques.

## Did I change the store in any way I did not undo

Yes, two saved searches, left in place: "rare rings 70 life (trade text)"
(id cd7f4ce903fd) and "rare rings 80 life (form)" (id 9dcbe9c5f0d2). The
saved-search file went from empty to revision 2. Nothing else was written:
no refresh, no price, no tick or plan applied. The CLI call was a read.

## Message to the person

The search pages work and are honest. The form, the trade-site text and
the command line all give the same answer to the same ring search: 101
rare Standard rings with at least 70 explicit life. They give the same
query and the same link back to the trade site. Each answer says plainly
what it ran, what it did not, and that 2940 never-fetched tabs and
characters are outside it.

Three things will trip a player:

1. "Fire + cold resistance ≥ 60" built the trade site's way (a weighted sum
   of the pseudo totals) is not run. The form offers those lines first, then
   drops the group ("PARTIAL — 3 clauses not run"). With explicit lines it
   runs, but it misses implicit and hybrid resistances.
2. On a tab opened by its name, the "Add a column" box beside the form's
   Search button runs the empty form. The tab is replaced by all 22,623
   items with no warning. The second box, under the query box, works.
3. Tab tables are wider than the pane. A currency tab's stack counts, and
   the column you sorted by, sit off-screen to the right.

Smaller ones: the Search button slides away as the count text grows; an
answer is not marked when the form above it changes; saved searches from
the form and from trade text look the same.

Abyssal sockets: 73 items, the same set three ways (form socket "A", the
"Has # Abyssal Sockets" lines, and `sockets.abyssal>=1`). A ring's total
life in the language is `pseudo.total_life`.

I left two saved searches in the store and changed nothing else. I never
opened the trade link or the rehearsal pages. Details, quotes and the
fault list are in REPORT.md.
