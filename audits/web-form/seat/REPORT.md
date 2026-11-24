# REPORT — the page seat, run in `/Users/tom/seat-worlds/web-form/page-opus`

Run on 2026-11-23. Pane tools are named by their short names (`get_page_text`,
`javascript_tool`, `find`, `computer` screenshot, `navigate`, `read_page`,
`form_input`). Door calls are `[call N]`. `C/` is the collection address
`http://127.0.0.1:59930/0947a8d0fe18dbbae97d65c7f84c271c/` (from
`web-collection.out`); `R/` is the rehearsal address
`http://127.0.0.1:59931/427f0c1b385604703ca15e2e6b7d011b/` (from
`web-rehearsal.out`).

## Before touching anything

From the tour (README "The tour"): `acq web` is described as "the search as a
page shaped like the trade site's, or a trade site search's URL run as --trade
runs it, each answer under the form filled with what was asked, to edit and run
again; its results as cards with item art, a table on asking, and the number a
form would match beside its button as it changes; the world's saved searches
…; the stash tabs and characters as pages, each a place opening its items, to
list, tick, plan and apply; and the language's index, linked from the query
box, each entry and block a page of its own". The CLI has `acq search --trade`,
`acq search --describe`, `acq saved …`.

What I opened first: `C/` in the pane (`preview_start`, then `get_page_text`,
`javascript_tool` listing links and form fields, one `computer` screenshot at
half scale). Before touching a control:

- A top bar (`javascript_tool`, links): **Search** `C/`, **Saved** `C/saved`,
  **Stash tabs** `C/tabs`, **Characters** `C/characters`, **Jobs** `C/jobs`;
  at the right "account GERWARIC#7694" (screenshot).
- **Search** page (`get_page_text`), four forms stacked:
  1. a realm picker (pc / xbox / sony / poe2) with "Show this realm's leagues";
  2. a textarea, placeholder "A trade site search: its URL, its id or its
     query JSON", a button "Run the site's search", "Matches: –" and the note
     "translated here, sent nowhere; a URL's realm is the search's";
  3. the trade-site-shaped form: "Search items" box, League ("every league",
     Allflame (tabs on disk), Ancestors (no tabs on disk), Hardcore …,
     Standard (tabs on disk) …), a status select (hidden name
     `query.status.option`, value `available`), Membership (live / all /
     removed), collapsible groups "Type Filters" (Item Category, Item Rarity),
     "Weapon Filters", "Armour Filters", "Socket Filters", "Requirements",
     "Map Filters", "Heist Filters", "Sanctum Filters", "Miscellaneous", and
     "Stat Filters" with a group type select (And, Not, If, Count, Weighted
     Sum, Weighted Sum v2, "Crucible Passive Tree Path (unsupported)",
     "Mercenary Skill Group (unsupported)"), "+ Add Stat Filter", "Add Stat
     Group"; then Show (20/50/100/200), "Results as cards / a table", "Or
     count by" (placeholder "rarity", note "keys as the CLI's count takes
     them: rarity, league, tab, class, category, line:resist,life"), a
     **Search** button with "Matches: –" beside it, and **Clear**
     (a link to `.?realm=pc`);
  4. "The query, in the language" — a textarea, a link "The language's index:
     every entry by its name and one line" (`C/describe?from=.`), Show,
     Results as, "Run this query", "Matches: –";
  5. a "Fetch the league list" button (POST `leagues`);
  6. at the foot, a tick box "Show the fields this search does not run" with
     "A field with no mark runs as the trade site's; a mark says it is read
     differently here, or not run."
  The field dump (`javascript_tool`) also showed form fields for Ultimatum
  filters and Trade filters (account, collapse, indexed, sale_type, fee,
  price) that have no visible heading in the page text — presumably the
  hidden ones that tick box shows.
- **Saved** (`get_page_text`): "No saved searches: save one from any search's
  answer, under its query."
- **Stash tabs** (`get_page_text`, truncated at 8000 chars, not scrolled): Realm,
  League, "Show", "Fetch the league list", "List this league's stash tabs";
  "3347 tabs in Standard: 405 fetched, 2926 never fetched, 16 folders (never
  fetched); 20231 items in tabs (characters apart); facts: 3331 tabs, 405
  fetched (the oldest fetch 76d ago, pc · Standard / Keepers (Remove-only);
  the newest 73d ago, pc · Standard / Dump Tab); stash listing 73d ago";
  "Items matching … Show Refresh ticked"; "the substashes box: also fetch a
  ticked map or unique tab's pages"; then a table Tab / Type / Items / Fetched
  with an "Open" link per tab (Dump Tab, QuadStash, 92, 73d ago; Currency 1,
  CurrencyStash, 115, 73d ago; …).
- **Characters** (`get_page_text`): "41 characters: 41 fetched (10 with empty
  bodies); 1080 items", a table Character / Level / Items / Fetched, "Open"
  links on those with items.
- **Jobs** (`get_page_text`): "1 job: 0 running, 0 waiting, 1 done." and one
  row "The account's profile, for seat — done — finished 6m ago". (On the
  collection's server, a job of the *other* account, `seat`. The daemon is
  shared; worth knowing, not necessarily wrong.)

What each page seems for: Search = ask the store like the trade site (form),
paste a trade search, or type the tool's own language; Saved = kept searches;
Stash tabs / Characters = places, opened to their items, ticked for refresh;
Jobs = the daemon's fetches.

What I would do first: fill the Type Filters (Ring, Rare), League Standard,
add a stat filter for maximum life with min 70, watch the "Matches" beside
Search, then press Search.

## The tasks

### T0 — Orient

Done as above. Tools: `preview_start`, `get_page_text` on each of the five
pages, `javascript_tool` for links and form fields, one screenshot. Going back:
each top-bar link reopened its page; the Search page came back empty, as it
was (I had touched nothing). Surprises noted: the two tab counts on Stash tabs
("3347 tabs" vs "facts: 3331 tabs" — 3347 − 3331 = 16, the folders, I guess);
a `seat` job on the collection's Jobs page; the 1080 character items plus
20231 tab items = 21311, not the brief's 22,721 (the rest presumably in
Allflame; not checked here).

### T1 — A search from the form

What I did (all on `C/`): League select → "Standard (tabs on disk)"; Type
Filters → Item Category "Ring", Item Rarity "Rare" (`find` "combobox" gave the
selects; set with `form_input`). The type filters group was open and "group
on" was already ticked. After those three, the number beside **Search** read
"Matches: 719 · 0 undecided" (`javascript_tool` reading the form's `<output>`s).

The life line: I clicked the box "+ Add Stat Filter" under Stat Filters and
typed `maximum life`. A list dropped down (screenshot, then `javascript_tool`
for its text). It began:
"+# total maximum Life pseudo / +# to maximum Life explicit / #% increased
maximum Life explicit · alone exact · with a bound ambiguous / …" — about 40
lines. Every life line offered was `explicit` or `pseudo`; no implicit,
crafted or fractured variant was offered for "+# to maximum Life" (the trade
site offers those, from my own knowledge). I took "+# to maximum Life
explicit" (clicked by coordinate). The row appeared as
"[x] on +# to maximum Life [min] [max] explicit" with a Remove button, the
min box focused; I typed 70. The hidden field became
`stat.id=explicit.stat_3299347043`, `stat.min=70` (`javascript_tool`).

Beside Search before pressing it: **"Matches: 101 · 0 undecided"**
(`javascript_tool` on the form's output).

Pressed **Search** (`find` → button "Search", click). The page became "Answer
· Acquisition" at a `C/search?…#answer` URL carrying every form field (about
6 KB of query string). The form above was refilled with what I asked (Ring,
Rare, Standard, the life row with 70 — screenshot). The answer
(`javascript_tool`, the `#answer` element's text):

> 101 matches
> from the query JSON: exact — every clause with a stash meaning ran as the site reads it
> Over the items the store has fetched, priced or not — what it never fetched is the scope's: the site shows public listings, a tab's being tab.public, so the search is the site's and the items are yours
> Link: the search on the trade site
> Note: the search names no status, and the site fills what a search leaves unsaid with its own, its status among them
> Query	league=Standard category="accessory.ring" rarity=rare sum(line("# to maximum Life" source=explicit).arg1)>=70
> Scope	account GERWARIC#7694 · pc · live · 22623 items · 546 locations fetched
> 2940 locations never fetched: MapStash 2489 of 2582, UniqueStash 437 of 521, character 14 of 59 (…) · location lists seen, the oldest 73d ago · each tab of Standard never fetched
> Total	101 matches (the oldest fetch 76d ago, pc · Standard / VVickedWitch (character); the newest 73d ago, pc · Standard / Dump Tab)
> 2940 locations were never fetched: nothing here says what they hold
> these tables read release 3.30 and no character here was fetched from it, the newest from 3.29 (73d ago): items may show earlier wording the tables do not read; a fetch cures it
> as a table · by stash tab · by character

Then item cards, the first "Damnation Circle Ruby Ring · rare · frame rare …
+78 to maximum Life explicit … pc · Standard / TheFireworksCompany (equipment)
· fetched 76d ago … sum(line("# to maximum Life" source=explicit).arg1) 78".
Each card's values (quality, item level, the life sum) are links that re-run
the search sorted by that value (`javascript_tool` on the answer's links:
`…&view.sort=ilvl&view.desc=true&view.value=…`). Under the answer, a form
"Name [ ] Save this search" (hidden `trade=` holds the query JSON).

**Count: 101 rare rings in Standard with explicit +# to maximum Life ≥ 70**
(from the answer heading and the number beside the button; the two agree).
I did not scroll through the 101; only the first 20 are shown ("Show 20").

What else stood on the page: the "Link: the search on the trade site" points
to `https://www.pathofexile.com/trade/search/Standard/H4sIAAAA…` (not opened);
the language box ("The query, in the language") was filled with the
translated query; "by stash tab" / "by character" links to the places pages
with `text=` the query.

Surprises: the Status select ("Instant Buyout and In Person", options
Instant Buyout / In Person (Online in League) / In Person (Online) / Any,
labelled "listing only") is disabled, so the search carries no status, which
the Note then remarks on. Scope says 22623 items, the brief 22,721.
On the answer page the number beside Search went back to "Matches: – · –
undecided" until something is changed. Coming back: the answer's URL is the
whole search, so reopening it (or Back) returns to the same answer; the form
is refilled each time.

(Process note: I appended T1 to this file with a shell heredoc rather than my
file tool; from T2 on I use the file tool as the brief asks.)

### T2 — Change the search on the page

On T1's answer page (form refilled), all by `computer` clicks/typing, numbers
beside the button read with `javascript_tool` from the form's `<output>`.

1. **"Add Stat Group"** (a link-like toggle under Stat Filters) opened a row of
   buttons: "And · Not · If · Count · Weighted Sum · Weighted Sum v2". I
   pressed **Weighted Sum**: a new "Group [Weighted Sum ▾] [x] group on",
   "[min] [max] [Remove group]", "+ Add Stat Filter" appeared.
2. First attempt, as a player usually does it: typed `fire resistance`; the
   list began "+#% total to Fire Resistance pseudo / +#% to Fire Resistance
   explicit / +#% to Fire and Lightning Resistances explicit / …". I took the
   **pseudo** "+#% total to Fire Resistance", weight 1, then "+#% total to
   Cold Resistance" pseudo, weight 1, then group min 60. Rows showed
   "[x] on +#% total to Fire Resistance [min] [max] / [1] pseudo [Remove]".
   Number beside Search: 101 before the group → **69** once the two pseudo
   rows were in with no group min → stayed 69 after the weights → **101**
   after typing 60 as the group min. A min that *raises* the count from 69 to
   101 made no sense to me.
   Pressed Search. The answer:
   > 101 matches
   > not the site's search: 3 clauses not run
   > from the query JSON: PARTIAL — 3 clauses not run, so this is not the site's search
   > Note: the link carries the query as asked: the clauses not run here run there
   > Status Clause Says
   > NOT RUN stats.1 unsupported whole: its filter `stats.1.0` is no line
   > NOT RUN stats.1.0 its group `stats.1` translates whole or not at all, and is unsupported
   > NOT RUN stats.1.1 its group `stats.1` translates whole or not at all, and is unsupported
   > Query league=Standard category="accessory.ring" rarity=rare sum(line("# to maximum Life" source=explicit).arg1)>=70
   So the pseudo "total" resistance lines the form offered (without any
   mark in the dropdown or on the row) are not run; the whole weighted group
   was dropped and the headline still says "101 matches". The refilled form
   kept the pseudo rows with no mark on them (screenshot). The badge and table
   are honest; the form and the live count were not (the 101 beside the
   button already reflected the dropped group, the 69 before it I cannot
   explain).
3. Second attempt: removed both pseudo rows (each row's **Remove**; count
   stayed 101), typed `to fire resistance`, took "+#% to Fire Resistance
   explicit", weight 1 → count **0** (min 60 still set; one fire roll cannot
   reach 60); then "+#% to Cold Resistance explicit", weight 1 → count **4**.
   Neither dropdown offered an *implicit* fire/cold line (a Ruby Ring's fire
   resistance is an implicit — the first ring of T1 has "+33% to Fire
   Resistance implicit"), nor did the life dropdown in T1.
   Pressed Search:
   > 4 matches
   > from the query JSON: exact — every clause with a stash meaning ran as the site reads it
   > Query … sum(line("# to maximum Life" source=explicit).arg1)>=70 sum(line(("#% to Fire Resistance" source=explicit) or ("#% to Cold Resistance" source=explicit)).arg1)>=60
   > Total 4 matches (the oldest fetch 76d ago, pc · Standard / Cem Stuff 1 (Remove-only); the newest 76d ago, pc · Standard / 3.20 Seismic Trap (Remove-only))
   **Count: 4** (answer heading; equals the number beside the button).
4. **Took one line out**: removed the Cold row (Remove). Number beside the
   button went to **0**. My first click on Search by coordinate missed (the
   page had shifted after the removal; nothing happened — I checked the URL
   still held the cold stat id); clicking by `find` reference worked:
   > 0 matches … from the query JSON: exact … Query … sum(line("#% to Fire Resistance" source=explicit).arg1)>=60

Did the number follow me? Yes, at every change after a short wait — with the
one oddity in step 2 (69 then 101 with the pseudo group). It goes back to
"–" when the answer page loads.

Surprises: the pseudo lines offered and then not run; no implicit variant of
any line offered; "Weighted Sum" and "Weighted Sum v2" both offered with no
word on the difference; the weight box sits under the row, after min/max, and
is blank by default (I typed 1 each time; I did not test what blank means).

Back and forth: each Search makes a new URL; Back returns to the previous
answer with its form.

### T3 — What the form says of itself

Nothing run. On T2's last answer page:

- The foot of the form (`get_page_text` at T0): a tick box **"Show the
  fields this search does not run"** with the sentence "A field with no mark
  runs as the trade site's; a mark says it is read differently here, or not
  run."
- **Marks** (`javascript_tool` listing every element with a `title` inside
  the form — 48 of them — and screenshots of opened sections): small grey
  words after a field, each with a tooltip:
  - "ambiguous" — e.g. Armour: "Armour at 20% quality, as the site figures
    it: within one of the site's own figure, which the display's rounding can
    move."; DPS/pDPS likewise; Map tier: "The tier read from the map's base
    name; a map whose base names no tier has none here, where the site gives
    it one."; Quality: "Quality as displayed, a catalysed ring's included as
    on the site; a map's quantity quality is not read."; crucible item.
  - "unsupported" — weapon Damage ("A weapon's damage as the site totals it
    is not read; ask for its damage values directly."), Class requirement
    ("No item of your account carries a class requirement, so this is not
    run."), map series, heist objective value, the four Ultimatum fields,
    transfigured gem, unidentified tier, fated, foil variation, and several
    "No item shows it; not run."
  - "listing only", drawn struck through in the Trade Filters (screenshot) —
    Status ("Online, available or securable is the seller's state, not the
    item's, so it means nothing in your stash."), Seller Account, Collapse
    Listings by Account, Listed, Sale Type, Gold Fee, Buyout Price ("The
    market's price; ask for your own prices directly.").
  - In the stat dropdown (T1/T2), lines carry "alone exact · with a bound
    ambiguous" or "alone ambiguous · with a bound ambiguous" (e.g. "#%
    increased maximum Life explicit · alone exact · with a bound
    ambiguous"). I did not find what "alone" and "with a bound" mean.
  - Armour Filters has a tick box **"assume 20% quality"**, ticked by
    default (the field behind it is `defences=q20`; untick → `displayed`,
    from the URL).
- **Hidden fields**: with the box unticked, the "Ultimatum Filters" and
  "Trade Filters" sections are not shown (`javascript_tool`: their
  `<summary>`s had no layout). Ticking the box (click by `find` ref) showed
  both at once, without reloading the page; the URL did not change.
- **No mark anywhere told me that a pseudo stat line is not run.** The
  dropdown offers "+#% total to Fire Resistance pseudo" exactly like the
  explicit lines; only the answer (T2) says "is no line".
- Emptying: the **Clear** link beside the Matches number (href `.?realm=pc`).
  It went to `C/?realm=pc` ("Search · Acquisition"): no stat rows, League back
  to "every league", category and rarity empty, the show-hidden box unticked,
  no answer (`javascript_tool`). Only defaults remained (status `available`,
  membership `live`, view 20).

### T4 — A search from the trade site

Where it goes: the box at the very top of the Search page, placeholder "A
trade site search: its URL, its id or its query JSON", with the button "Run
the site's search" and the note "translated here, sent nowhere; a URL's realm
is the search's" (found at T0 with `get_page_text`; located with `find`).
I put the JSON in with `form_input`. Before I pressed anything, the number
beside that button read **"Matches: 101"** (screenshot). I pressed "Run the
site's search" (click by coordinate). The URL became
`C/search?realm=pc&trade=<the JSON, URL-encoded>#answer`. The answer
(`javascript_tool`, `#answer` text):

> 101 matches
> from the query JSON: exact — every clause with a stash meaning ran as the site reads it
> Over the items the store has fetched, priced or not — …
> Link: the search on the trade site
> Status	Clause	Says
> reported	status	Online, available or securable is the seller's state, not the item's, so it means nothing in your stash.
> Query	league=Standard category="accessory.ring" rarity=rare sum(line("# to maximum Life" source=explicit).arg1)>=70
> Total	101 matches (the oldest fetch 76d ago, pc · Standard / VVickedWitch (character); the newest 73d ago, pc · Standard / Dump Tab)

- **Compared with T1:** same count (101), same Query line, same first cards
  (Damnation Circle Ruby Ring, Death Grasp Amethyst Ring …), same Total line.
  The difference is the status: T1 said "Note: the search names no status, and
  the site fills what a search leaves unsaid with its own"; T4 lists the
  status clause as **"reported"** with the reason above — read, not run.
- **What it ran and did not run:** everything but `status`, which it
  "reported". The T1 Note line is absent here.
- **The form:** refilled from the JSON (`javascript_tool`): League
  "Standard", Category "accessory.ring", Rarity "rare", one stat row
  `explicit.stat_3299347043` min 70, the Status select now "online"
  (still disabled), the language box filled with the same Query. The trade
  box at the top was **empty** again — the JSON lives only in the URL.
- **The way back to the trade site:** "Link: the search on the trade site",
  `target="_blank" rel="noopener"`, pointing to
  `https://www.pathofexile.com/trade/search/Standard/H4sIAAAAAAAA_1WP0QqDMAxF_yXPIjIF0V8ZIqVmEuja0sYxKf33pfOl5im5NzncJIis-IgwJ3CeyVmYwVlDFiE3f1O8ZwI-PYql7AYNvMgwhsugTWT8ekOauC0Ha_-Ypn4Yu6GX3Y8yBxb8m4Q9djkveakQF3qt5qrVinF34bzFU1pjjKK2gexeYgYViO9LIskHpX7fGVzJ4wAAAA`
  (`javascript_tool`, the anchor's `href`). **Not opened.** What goes with
  it: on this answer the page says nothing beyond the link text. (On T2's
  partial answer it said "Note: the link carries the query as asked: the
  clauses not run here run there".) To see what it carries I decoded the last
  path segment in the page with `javascript_tool` (base64url + gzip,
  computed in the page, nothing fetched): it is
  `{"status":{"option":"online"},"stats":[…"explicit.stat_3299347043"…{"min":70}…],"filters":{"type_filters":…"accessory.ring"…"rare"…}}`
  — the query as asked, status included, league in the path. T1's link
  decodes to the same without a status. Nothing the page says is left out of
  it; the realm (pc) is not in it, nor the tool's own scope (membership live).
  Whether the real trade site opens a search from a path segment like this
  I cannot tell from the tool (see "Which answers I would not bet on").

Back: the answer URL holds the whole trade JSON, so it is reopenable as is.

### T5 — Keep a search

- **Saving.** At the foot of T4's answer (screenshot, after the item cards,
  a "rows 1–10 of 101 shown, as they lie in their place, top row first and
  left to right (no sort asked) · press a number on a card to order the rows
  by it · more" line, a "Terms" table and a build line) stood: "Saved, this
  keeps the site's search as given, translated again each time it opens." and
  "Name [ ] [Save this search]". I typed `rings life70 (trade text)` and
  pressed it (clicks by coordinate). It took me to **Saved** (read
  with `javascript_tool` on `main`):
  > 1 saved search · the file at revision 1
  > Name Scope Query Revision Last written Id
  > rings life70 (trade text) pc · live league=Standard category="accessory.ring" rarity=rare sum(line("# to maximum Life" source=explicit).arg1)>=70 (from a site search: exact) 1 0s ago via web 1f0e75abf2fe [Rename] Clear
- **Finding it again.** Top bar → Saved (`navigate` to the link's address):
  the same row. The name is a link to `saved/open?id=1f0e75abf2fe&from=saved`.
  My click on it by `find` reference landed beside it and did nothing; a
  click by coordinate opened it.
- **How it showed when opened** (`javascript_tool`): URL
  `C/search?realm=pc&trade=<the JSON>&saved=1f0e75abf2fe&saved_revision=1&from=saved`;
  at the top "Back to the saved searches" and
  > This search holds what the form below does not send: a search from the form leaves each of these out.
  > • query.status.option "online"
  and above the answer
  > Saved search "rings life70 (trade text)", opened at revision 1 · the saved searches
  then the same 101-match answer as T4 with status "reported". The form was
  filled (Standard, Ring, Rare, life min 70) and a **Status** row "In Person
  (Online) — listing only" was now visible in the form header (screenshot);
  "Show" read **10**, where T1's form answer had shown 20.
- **Change one thing and run.** I changed the life min to 80 (triple-click,
  type). Beside Search: **49**. Pressed Search (by `find` ref). The URL became
  a form URL (`query.term=&…&saved=…`, no `trade=`). Answer:
  > Saved search "rings life70 (trade text)", opened at revision 1 · the saved searches
  > This answer's query is not the saved one's: the saved query is league=Standard category="accessory.ring" rarity=rare sum(line("# to maximum Life" source=explicit).arg1)>=70 over pc · live.
  > 49 matches
  > from the query JSON: exact — …
  > Note: the search names no status, and the site fills what a search leaves unsaid with its own, its status among them
  **Count: 49** (rings with ≥80 explicit life). As the banner had warned, the
  status `online` was dropped the moment the form was sent. At the foot: two
  save forms, "Name [rings life70 (trade text)] [Save over "rings life70
  (trade text)"]" and "Name [ ] [Save as a new search]", under the same line
  "Saved, this keeps the site's search as given, translated again each time
  it opens."
- I saved it as new, `rings life80 (form)`. Saved now:
  > 2 saved searches · the file at revision 2
  > rings life70 (trade text) pc · live …>=70 (from a site search: exact) 1 57s ago via web 1f0e75abf2fe
  > rings life80 (form) pc · live …>=80 (from a site search: exact) 1 0s ago via web 5ae036cd5b00

What a saved search holds, as far as the pages show: a name, a scope (realm
and membership: "pc · live"), the query (shown in the language), a revision,
when and how it was last written ("via web"), an id. From the "opened" banner
it also keeps the trade JSON as given (status included) for the one saved
from the trade text.

**Could I tell a form search from a trade-text search?** Not from the list:
both say "(from a site search: exact)" and show the same kind of Query. Only
by opening them, and even then only indirectly. I opened the form one too
(`navigate` to its `saved/open?id=5ae036cd5b00` link): it **also** opens as a
`trade=` URL — the form search was kept as trade JSON (`{"league":"Standard",
"query":{"filters":…,"stats":[…{"min":80}…]}}`, no status) — showing
"Saved search "rings life80 (form)", opened at revision 1", 49 matches, and
the "Note: the search names no status …" line, without the "holds what the
form below does not send" box. So what tells them apart is the status, not
where they came from; a form search *becomes* a site search when saved. The
line "Saved, this keeps the site's search as given" is printed under a form
search too.

Back and forth: "Back to the search" and "Back to the saved searches" links
both worked and returned the page as I left it.

### T6 — A tab as it lies, and in an order you name

**Currency tab.** Stash tabs page (top bar). Each tab's name is a link (the
"Open" I saw in the T0 text is part of the row, not a separate link — `find`
showed `link "Currency 1"` and a `checkbox "Currency 1"`). "Currency 1"
(CurrencyStash, 115 items) points to
`C/search?realm=pc&text=id:0ce277027d league=Standard&view.fields=icon` — a
tab opens as a **search in the tool's language** (`id:<tab> league=…`), as a
table. I opened that address (`navigate` to the link's href).

> 115 matches
> Query	id:0ce277027d league=Standard
> Total	115 matches (fetched 73d ago, pc · Standard / Currency 1)
> 115 matching items in the table, as they lie in their place, top row first and left to right (no sort asked) · open an item by its name and press a number on its card to order the rows by it

Columns: Item, Base, Price, Location, Fetched, and a linked "stack" (a sort
link). At first the table showed **only its header** (screenshot after
`scroll_to`): the rows are a scrolling window that fetches its rows as you
scroll (`javascript_tool` on the HTML: `data-windowed`, a `noscript` "The
first 50 items of 115 are shown here: the rest come in as the list scrolls,
which needs the page's script."). After I scrolled down 3 ticks 100 rows were
present (I did not scroll to the last 15). Order: the stash grid, top row
left to right — the first lines (`javascript_tool` on the rows):

> Blacksmith's Whetstone | Blacksmith's Whetstone | none (nothing applies: the price text in game is not where the index can see it) | Standard / Currency 1 · x 0, y 0 | 73d ago | 9836
> Scroll of Wisdom | Scroll of Wisdom | none (nothing applies) | Standard / Currency 1 · x 1, y 0 | 73d ago | 7712
> Chaos Orb | … | x 2, y 0 | 73d ago | 9548
> … Mirror of Kalandra … x 4, y 0 … 1 … Exalted Orb … x 11, y 0 … 436 … Orb of Augmentation … x 13, y 0 … 10000

Each line: name, base, price (with a reason when none), location with grid
x/y, how long ago fetched, stack size. Note the "stack" count for
Blacksmith's Whetstone is 9836 and Orb of Augmentation 10000 — larger than a
single in-game stack; in a currency tab that is what the slot holds, I think.

**Gear tab in an order I name.** "Decent Gear" (PremiumStash, 37) opened the
same way. Where to name a value: a box **"Add a column"** — there are two,
one right of the form's Search button and one under the language box (I used
the latter). Typing `life` offered (screenshot): "+# total maximum Life
pseudo / # Life Regenerated per Second pseudo / #% of Life Regenerated per
Second pseudo / #% of Physical Attack Damage Leeched as Life pseudo / +# to
maximum Life explicit / Regenerate # Life per second explicit / #% increased
maximum Life explicit / …". I clicked "+# total maximum Life pseudo". The page
re-ran at once with `view.sort=pseudo.total_life&view.desc=true&view.value=pseudo.total_life`:

> 37 matching items in the table, by pseudo.total_life, largest first, as asked
> Item Base Price Location Fetched ▼pseudo.total_life , ordered largest first
> Bramble Vise Titan Gauntlets · rare | Titan Gauntlets | none (nothing applies) | Standard / Decent Gear · x 6, y 5 | 76d ago | 109
> Empyrean Knot Coral Ring · rare | … | x 2, y 2 | … | 108
> Grim Crest Samnite Helmet · rare | … | 94
> Viper Harness Stygian Vise · rare | … | 93

(`javascript_tool`, `#answer` text.) The column head names the value by its
internal name `pseudo.total_life`, not "+# total maximum Life". Choosing a
column *also* sorted by it, largest first; I did not try a column without a
sort. Worth noting against T2: here a **pseudo** value works (sort and
column), while in the form's stat filter a pseudo line was "no line" and not
run.

Back: the Stash tabs page came back as before (`navigate`); each opened tab
is a URL and reopened the same.

### T7 — Abyssal sockets

All over every league (League "every league"), membership live.

1. **Form's socket controls.** Socket Filters (opened by `find` ref) shows
   "Sockets [min] [max] [R] [G] [B] [W] [A]" and "Link Groups [min] [max]
   [R] [G] [B] [W] [A]" — no marks. I typed 1 in Sockets **A**. Beside
   Search: **73**. Search:
   > 73 matches
   > from the query JSON: exact — every clause with a stash meaning ran as the site reads it
   > Link: none — the search names no league, and a site link is one league's: a search id names none, and the query JSON names one as `league` beside its `query`
   > Query	sockets.abyssal>=1
   > Total	73 matches (the oldest fetch 77d ago, pc · Allflame / TheAbsenceOfPatience (character); the newest 73d ago, pc · Standard / OnlyCircles (character))
   First card: "Brimstone Knuckle Hydrascale Gauntlets · rare … Has 1 Abyssal Socket explicit …".
   **Count: 73.**
2. **From a line the items display.** The card shows "Has 1 Abyssal Socket
   explicit". Cleared the form (Clear), typed `abyssal socket` in "+ Add Stat
   Filter"; the list offered (screenshot) "Has # Abyssal Sockets explicit ·
   alone ambiguous · with a bound ambiguous", "Abyssal Troves in your Maps …",
   "Socketed Rare Abyssal Jewels will be Consumed …", "Cannot have non-Abyssal
   sockets explicit", "Has # Abyssal Sockets implicit · …", "Has # Abyssal
   Sockets fractured · …" (so implicit/fractured variants *are* offered for
   this line). Took the explicit one, min 1. Beside Search: **5**. Search:
   > 5 matches
   > ambiguous: 1 clause as the site may not read
   > from the query JSON: ambiguous — every clause ran, 1 as the site may not read it
   > Status Clause Says
   > ambiguous stats.0.0 the bound sums the text and its plurals; not its forms "Has 1 Abyssal Socket" (other)
   > Query sum(line("Has # Abyssal Sockets" source=explicit).arg1)>=1
   The five are all uniques (Bubonic Trail Murder Boots, Wraithlord Bone
   Circlet, Shroud of the Lightless ×2, Lightpoacher Great Crown). **Count: 5.**
   The singular wording "Has 1 Abyssal Socket" — exactly what the gauntlets
   above display — is left out. I did not add the implicit/fractured rows.
3. **Typing a query in the language.** I learned the word from way 1: the
   form's translation printed `Query sockets.abyssal>=1`. (The index, read
   afterwards, agrees: "sockets.<word> number how many sockets of one kind the
   item has, over every link group; GGG's letter for the word (red R, green G,
   blue B, white W, abyssal A) …", and inside `linked( … )` "red green blue
   white abyssal".) Opened "The query, in the language", typed
   `sockets.abyssal>=1` (`form_input`) — "Matches: 73" under it — and pressed
   "Run this query": URL `C/search?realm=pc&text=sockets.abyssal>=1…`,
   > 73 matches · Query sockets.abyssal>=1 · Total 73 matches (…)
   **Count: 73.**

Do they agree? Ways 1 and 3 agree (73; the same query). Way 2, the one a
trade-site player reaches by reading the line on the item, finds **5**, and
the page's only warning is an "ambiguous" badge whose explanation I could
only half read. Back and forth: every answer is a URL; Clear and the top bar
brought me back to an empty form each time.

### T8 — The language's words

Where: the link "The language's index: every entry by its name and one line"
sits inside "The query, in the language", which is **collapsed** on answer
pages (I had to open it: the link had no layout until then). From the plain
Search page it goes to `C/describe?from=.`; from an answer it goes to
`describe?from=<that answer>` and the index's "Back to the search" returns to
that very answer (checked: its href was the answer's URL; entry links carry
the same `from=`).

What the page is like (`javascript_tool` on `main`, 28,473 characters, 316
links; not all read): "The language" / "Back to the search" / "This is the
index, each entry by its name and one line: name an entry for it whole
(league, pseudo.total_res, accessory.ring), a block for every entry in it
(computed, categories, trade), ~ and a pattern for the entries whose name or
definition it matches (~resist), or all for everything whole." / a "Names"
box with a **Describe** button / then blocks: "fields", "inside line( … )",
"inside linked( … )", "values", "composition", "operators", "slots",
"counts", "why an answer is what it is", "an answer, key by key", "not built,
refused by name", "limits, as the search states them", plus the long lists
of pseudo totals and categories. Each row "Name · Kind · What it is"; each
name a link to its own page. It is dense, precise and long; written for
someone who already knows GGG's JSON ("GGG's frameTypeId", "the arrays
`crucibleMods` and `utilityMods`"). No search box over it other than the
Names box (patterns with `~`), which I did not try.

**A ring's total life:** `pseudo.total_life`. Its own page
(`describe?names=pseudo.total_life`):
> pseudo.total_life total the trade site's pseudo_total_life (`+# total maximum Life`): the sum over the item's lines of 1 × arg1 of line("# to maximum Life") + 0.5 × arg1 of line("# to Strength") + 0.5 × arg1 of line("# to Strength and Dexterity") + 0.5 × arg1 of line("# to Strength and Intelligence") + 0.5 × arg1 of line("# to all Attributes"); a total leaves out a crucible node's lines and a flask's own lines …
> e.g. pseudo.total_life>=60 -has:pseudo.total_life undecided(pseudo.total_life)

**One thing it can ask that the form cannot:** `tab.remove_only` — "whether
the stash tab the item is in is remove-only, which GGG marks by ending its
name with ` (Remove-only)`: `true` or `false` … e.g. tab.remove_only=false"
(its own page). The form has no field for the tab an item is in at all. (Also
`note`, `price.amount`, `holds( )`, `undecided( )`, `linked( … )` by colour
on one group.)

Side checks run here, because the index lists `pseudo.total_fire_res` ("the
trade site's pseudo_total_fire_resistance (`+#% total to Fire
Resistance`)") which T2's weighted group called "no line": through the trade
box I ran `pseudo.pseudo_total_life` ≥70 on rare rings — **130 matches,
exact** — and `pseudo.pseudo_total_fire_resistance` ≥30 in an "and" group —
**116 matches, exact**, Query `… pseudo.total_fire_res>=30`. So pseudo lines
run in an And group; T2's failure is that a **Weighted Sum** group cannot hold
a pseudo total, and nothing on the form says so before running.

Did I come back to my search? Yes — "Back to the search" from the index
returned to the answer I came from.

### T9 — The terminal and the page

How I found the command: the tour lists `acq search --trade <URL|ID> |
--trade-file <FILE|->`; `[call 1] acq search --help` (exit 0) explains
`--trade-file`: "`--trade`, read from a file or from stdin with `-`: the
query JSON as the site posts it, `{"query": …, "sort": …}` or the query
object alone, needs no shell quoting this way. A `"league"` beside the query
is the search's league …". I wrote T4's JSON to `drafts/t4-trade.json`.

- `[call 2] acq search --realm pc --trade-file …/drafts/t4-trade.json` —
  **exit 1**: "Error: 2 accounts are known and one is to be picked:
  GERWARIC#7694, seat; name one with --account", offering both command lines.
  (The page never asks: each page server is one account's.)
- `[call 3] acq --account GERWARIC#7694 search --realm pc --trade-file …`
  — exit 0, 531 ms, saved as `work/t9-trade.txt`:
  > trade   from the query JSON: exact — every clause with a stash meaning ran as the site reads it
  >         link      https://www.pathofexile.com/trade/search/Standard/H4sIAAAAAAAA_1WP0QqDMAxF_yXPIjIF0V8ZIqVmEuja0sYxKf33pfOl5im5NzncJIis-IgwJ3CeyVmYwVlDFiE3f1O8ZwI-PYql7AYNvMgwhsugTWT8ekOauC0Ha_-Ypn4Yu6GX3Y8yBxb8m4Q9djkveakQF3qt5qrVinF34bzFU1pjjKK2gexeYgYViO9LIskHpX7fGVzJ4wAAAA
  >         reported  status  Online, available or securable is the seller's state, not the item's, so it means nothing in your stash.
  > query   league=Standard category="accessory.ring" rarity=rare sum(line("# to maximum Life" source=explicit).arg1)>=70
  > total   101 matches (the oldest fetch 76d ago, pc · Standard / VVickedWitch (character); the newest 73d ago, pc · Standard / Dump Tab)
  > rows    Damnation Circle Ruby Ring … Death Grasp Amethyst Ring … Kraken Circle Bone Ring … (10 of 101)

Agreement: **yes** — 101 (door's `total` line) against the page's 101; the
same query, the same faithfulness line, the same status "reported", the same
first rings in the same order, the same terms table (league 21311 matched,
category 1133 · 5 undecided, rarity 5769, life 762). **The way back is the
same**: the `link` line is character for character the page's link. The CLI
shows the URL in the open; the page hides it behind "the search on the trade
site". Neither opens it.

### T10 — As a player

Three things I would want from these pages and could not do, or could not
find how to do:

1. **"Fire + cold resistance total ≥ 60, counting implicits and dual-res
   lines" — the everyday weighted-sum search.** The form offers the pseudo
   "total" lines but drops a Weighted Sum group that holds them ("is no
   line", T2); with explicit lines it runs but leaves out a Ruby Ring's
   implicit fire resistance and "to Fire and Cold Resistances" lines. The
   language has `pseudo.total_fire_res` and `pseudo.total_cold_res`, but I
   found no way in the index to add two values together (only `sum` over
   lines a pattern selects, "your arithmetic, never a reviewed total").
2. **Price or list an item from the pages.** Every item says "price none
   (nothing applies)"; the item page (`C/item/<id>`, `javascript_tool`) shows
   its lines, sources, class, category and place, and has no control but the
   link to its place. The CLI has `acq price set` (tour); the pages offer no
   way I found to set, see or compare a price, nor to see what the market
   would pay (the trade link is the only bridge, and it is to be opened by
   hand).
3. **Use an item: copy its text (for the game, Path of Building or a trade
   whisper), or find upgrades for what a character wears.** No "copy item"
   on a card or the item page; a character opens as a list of items, with no
   way I found to compare a search's rings with the rings a character has on.
   (Also: from an item card I could not jump to "search for items like this"
   on the form.)

## Faults, worst first

1. **A Weighted Sum with the form's own pseudo lines is silently dropped,
   and the headline count still reads as an answer.** — T2. I built
   rings + life ≥70 + Weighted Sum(pseudo total fire res ×1, pseudo total
   cold res ×1) ≥60, all from the form's own dropdowns. The answer: "101
   matches", with "PARTIAL — 3 clauses not run", "NOT RUN stats.1 unsupported
   whole: its filter `stats.1.0` is no line". Should: either run it (the
   language has `pseudo.total_fire_res`, and the same pseudo runs fine in an
   And group: 116 exact, T8) or refuse before running — mark the row in the
   form, or not offer pseudo lines inside a Weighted Sum. The red badge is
   there, but "101 matches" in big type is the answer to a different
   question, and "is no line" does not tell a player what to do.
2. **The displayed line "Has 1 Abyssal Socket" is not found by the form's
   "Has # Abyssal Sockets" stat.** — T7. Stat filter → 5 matches (only
   uniques with 2 sockets, I believe); socket control and language → 73. The
   page says "ambiguous … not its forms "Has 1 Abyssal Socket" (other)". A
   singular/plural wording difference makes the obvious way miss 68 of 73
   items. Should: count the singular form as the trade site does, or at least
   say plainly "68 items that show 'Has 1 Abyssal Socket' are not counted;
   use Sockets A".
3. **The live count beside Search went 69 → 101 when I *added* a group
   minimum.** — T2. Pseudo group with no min: 69; typing 60 as min: 101.
   Adding a constraint should never raise the count; at that point the
   number already reflected the dropped group without saying so. Should:
   show the same "N clauses not run" next to the live number.
4. **No implicit variant offered for the life and resistance lines I
   typed.** — T1/T2. "maximum life" and "to fire resistance" offered only
   explicit and pseudo; "abyssal socket" offered explicit, implicit and
   fractured. So the form's resistance sum can't count a Ruby Ring's
   implicit. (I read the list text by script; it may have been longer than I
   scrolled — see "would not bet".)
5. **A trade-text search loses its status the moment it is edited in the
   form** — T5. Opened saved trade search (status online, "reported"), changed
   life to 80, Search → "the search names no status". The page warns ("This
   search holds what the form below does not send … query.status.option
   "online"") — honest, but the form then cannot keep it, and the status
   select is disabled.
6. **The saved list cannot tell a form search from a trade-text search** —
   T5. Both rows end "(from a site search: exact)"; a form search is stored
   as trade JSON. And "Saved, this keeps the site's search as given" is
   printed under a form answer too.
7. **A stash tab's table first shows only its header** — T6. Rows arrive
   only after scrolling (windowed list). A first glance reads as an empty tab
   of "115 matching items".
8. **"Show" changes between 20 and 10** — T1 vs T4/T5: a form search showed
   20, a trade-text or saved search opened with "Show 10" ("rows 1–10 of 101").
9. **Column names are internal** — T6: choosing "+# total maximum Life"
   gives the column head "▼pseudo.total_life".
10. **Clicks by reference land beside links** (saved search name, Search
   after a row removal) — T2, T5. Partly the pane; a click by coordinate
   worked. Noted because the page re-flows after removing a row, which made
   my first Search click miss.

## What the tour and the pages did not say

- That a stash tab "opens" as a language search (`id:<tab> league=…`) as a
  table, and that its rows load only as you scroll.
- That a Weighted Sum group cannot hold pseudo totals (it seems), and what
  "is no line" means.
- What "alone exact · with a bound ambiguous" on the stat list means.
- What "Weighted Sum v2" is, against "Weighted Sum".
- What a blank weight means in a Weighted Sum (I always typed 1).
- That the status select is disabled and why, until you hover ("listing
  only"); the T3 marks are tooltips.
- That a saved form search is stored as trade JSON.
- That the index link lives inside the collapsed "The query, in the
  language" on answer pages.
- Why the scope says 22623 items where the brief says 22,721, and why the
  Stash tabs page says "3347 tabs … facts: 3331 tabs".
- That the CLI needs `--account` here (two accounts known) when the page
  does not.

## The tool's own words

- "from the query JSON: exact — every clause with a stash meaning ran as the
  site reads it" (what is a clause *without* a stash meaning?)
- "Over the items the store has fetched, priced or not — what it never
  fetched is the scope's: the site shows public listings, a tab's being
  tab.public, so the search is the site's and the items are yours"
- "unsupported whole: its filter `stats.1.0` is no line"
- "its group `stats.1` translates whole or not at all, and is unsupported"
- "the bound sums the text and its plurals; not its forms "Has 1 Abyssal
  Socket" (other)"
- "alone exact · with a bound ambiguous", "alone ambiguous · with a bound
  ambiguous"
- "0 undecided" beside Matches
- "none (nothing applies: the price text in game is not where the index can
  see it)"
- "frame rare"
- "10 counts have no link: a count's route is listed only when every route
  is asked for, which a page never does"
- "these tables read release 3.30 and no character here was fetched from it
  … a fetch cures it"
- "Lacked", "Undecided", "Only together" (terms table heads)
- "Saved, this keeps the site's search as given, translated again each time
  it opens."
- "basis store 861010a882b9 · build … · snapshot 1126 · facts v9 · …
  derivation 12 …"

## What I would ask of the tool

- Mark, in the stat dropdown and on the form row, any line that will not run
  where it is placed (pseudo in a Weighted Sum), and show "N not run" beside
  the live count, not only after Search.
- Run pseudo totals inside a Weighted Sum, or say how to ask "fire + cold
  total ≥ 60" in the language.
- Match "Has 1 Abyssal Socket" with "Has # Abyssal Sockets" (and similar
  singular forms), or point to Sockets A in the warning.
- Offer implicit/crafted/fractured variants for every line that has them.
- Show the stat's human name in table column heads.
- Show in the Saved list whether a search came from the form or from trade
  text, and whether it carries anything the form cannot (status).
- Keep a trade search's status when edited in the form (or let the status
  select be set and say it is reported, not run).
- Render the first block of a tab's rows without a scroll.
- A "copy item text" and a price control on the item page.

## Where I left the page for the terminal, and why

Only for T9, as the task asks (`[call 1]` help, `[call 2]` refused for want
of `--account`, `[call 3]` the search). No page left me stuck enough to need
the terminal.

## What I read or ran outside the world (there should be nothing)

Nothing outside `W`. Inside `W` I read `BRIEF.md` and the "The tour" section
of `README.md` (printed with a shell `awk` over README.md that stops at the
next `##`), and at the start one `ls -la` of `W` itself and `cat` of the two
`.out` files. I did not open `PROMPT.md`, `seat.py`, `world.json`, `web.json`,
`setup-login.csv`, or anything under `bin/`, `logs/`, `store/`,
`setup-login-calls/`. In the page I decoded the trade link's gzip/base64 path
segment with the browser's own `DecompressionStream` (`javascript_tool`):
computed in the page, nothing fetched. No web search, no other site; the
trade-site link was never opened. One shell use I should name: T1 was
appended to REPORT.md with a heredoc rather than my file tool.

## Left out

- The rehearsal account's pages (`R/`) — no task in T0–T10 asked for a
  fetch, so I did not open them, refresh, tick, plan or apply.
- I did not try "Results as a table" on the form, "Or count by", "by stash
  tab"/"by character" views, Rename or Clear on saved searches, the
  "Describe" box or `~` patterns in the index, "Weighted Sum v2", Count/If/Not
  groups, or "Fetch the league list".
- I did not scroll through all 101 rings or all 115 currency rows.
- I did not test a blank weight, or the "assume 20% quality" tick.

## Which answers I would not bet on

- **No implicit variants offered for life/fire/cold** (Fault 4): I read the
  dropdowns by screenshot and a script over the list; the life list I read
  was cut at ~40 entries, so an implicit "+# to maximum Life" may sit lower.
  For "to fire resistance" the list looked complete (11 rows) but I did not
  scroll it.
- **Why 69 then 101** in T2: I report what the number did; the cause is my
  guess.
- **That the 5 abyssal matches are all 2-socket uniques**: I saw their
  names, not their socket lines.
- **Whether the trade link opens on the real trade site**: from my memory of
  the site, a search is normally a short id or `?q=` JSON; a gzip blob as a
  path segment is not something I recognise. I did not open it.
- The T6 currency stack sizes above a normal stack (9836, 10000): I read
  them as printed; whether they are right I cannot tell.

## Which claims rest on my own knowledge of the game rather than on the tool

- That players use a Weighted Sum of *pseudo total* resistances on the trade
  site, and that a single explicit fire roll cannot reach 60.
- That the trade site offers implicit/crafted/fractured variants of "+# to
  maximum Life" and resistances.
- That a Ruby Ring's fire resistance is an implicit (also shown by the tool
  on the first card).
- That the trade site would count "Has 1 Abyssal Socket" under "Has #
  Abyssal Sockets".
- That trade-site search links are usually short ids or `?q=` JSON.
- What players want in T10 (copy item text, Path of Building, prices).

## Did I change the store in any way I did not undo

Yes, in the saved searches only (the world's, not an account's): I saved
**`rings life70 (trade text)`** (id `1f0e75abf2fe`) and **`rings life80
(form)`** (id `5ae036cd5b00`); the Saved page reads "2 saved searches · the
file at revision 2". I left both in place for the person to see; I did not
clear them. No refresh, tick, plan, apply or price was done; the CLI calls
were a help, a refused search and a search (read-only), and `work/t9-trade.txt`
and `drafts/t4-trade.json` are mine.

## Message to the person

The search pages mostly work, and they are honest about what they ran.
Your rare rings in Standard with at least 70 explicit life come to **101**,
whether you ask from the form, from the trade site's JSON, or from the
terminal. All three agree, and the link back to the trade site is the same
in each. Saving, reopening and changing a search works. A stash tab opens
in the order it lies in, and you can sort it by a value you pick by name.

Two traps will catch a trade-site player:

1. **The resistance weighted sum.** If you build "fire + cold total ≥ 60"
   from the form's own "+#% total to … Resistance" lines, the page silently
   drops the whole group. It still shows "101 matches" in large type, with a
   small "3 clauses not run" badge. The live number beside Search even went
   *up* (69 → 101) when I added the minimum. With the explicit lines it runs
   (4 rings), but then implicits and dual-resistance lines are not counted.
2. **Abyssal sockets.** The stat line "Has # Abyssal Sockets" finds **5**
   items, because it skips the singular wording "Has 1 Abyssal Socket".
   The Sockets "A" box, or `sockets.abyssal>=1` typed in the query
   language, finds **73**.

Smaller things: a stash tab's table shows only its header until you scroll.
A saved form search and a saved trade-site search look the same in the
Saved list. A trade search loses its "online" status the moment you edit it
in the form. Column heads show internal names such as `pseudo.total_life`.

I could not find a way to price an item, copy an item's text, or compare
items with what a character wears.

I left two saved searches in place: "rings life70 (trade text)" and "rings
life80 (form)". Nothing else changed. I did not open the trade link or the
rehearsal account's pages.
