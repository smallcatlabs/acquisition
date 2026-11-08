# Seat report: the cards and the ways on (cards-opus, 2026-11-08)

Runner: Claude Opus 5.5, blind. I read `BRIEF.md` and the "The tour"
section of `README.md` (with `awk`, from its heading to the next `##`),
the two `.out` address files, and nothing else.

Collection pages: `http://127.0.0.1:64075/91e91bdee5626101ff1632deaf98fdb4/`
Rehearsal pages: `http://127.0.0.1:64074/272a178059cadfa04c003d854bec0d4c/`

## Before touching anything

Written after opening the collection's front page and following the
four menu links, before using any control (all via `get_page_text`,
plus one `javascript_tool` read of the links and form fields, and one
screenshot).

The menu (read with `javascript_tool`, because `get_page_text` reads
only `<main>` and so leaves the menu out): **Search**, **Saved**,
**Refresh**, **Jobs**.

- **Search** (`/`): a form shaped like the trade site's. A free text
  box "Search items" (field `query.term`), Realm (pc, xbox, sony,
  poe2) with "Show this realm's leagues", Status ("Instant Buyout and In
  Person" … "Any", marked "listing only"), Membership (live / all /
  removed), then collapsible filter groups: Type Filters (Item Category,
  Item Rarity), Weapon, Armour, Socket, Requirements, Map/Chart, Heist,
  Sanctum, Ultimatum, Miscellaneous, Trade, and Stat Filters (Group: And,
  Not, If, Count, Weighted Sum, Weighted Sum v2, and two marked
  "(unsupported)": "Crucible Passive Tree Path", "Mercenary Skill
  Group"; four stat rows with min/max; "Find a stat"). Some options carry
  the word "exact" beside them. Then "Show 20/50/100/200", "Sort by",
  "highest first", "Results as cards / a table", "Or count by — keys as
  the CLI's count takes them: rarity, league, tab, class, category,
  line:resist,life", and a **Search** button.
  Below that, two more forms: "The query, in the language" (a textarea,
  Show 10–200, Sort by, highest first, "Run this query"), and "A trade
  site search: its URL, its id or its query JSON" ("Run the site's
  search", "translated here, sent nowhere; a URL's realm is the
  search's"). Last, a "Fetch the league list" button (a POST form).
- **Saved** (`/saved`): "No saved searches: save one from any search's
  answer, under its query."
- **Refresh** (`/refresh`): Realm select and "Show", "Fetch the league
  list", then "3347 tabs in Standard: 405 fetched, 2926 never fetched,
  16 folders (never fetched); 20231 items in tabs (characters apart)"
  and "facts: 3331 tabs, 405 fetched (the oldest fetch 61d ago, pc ·
  Standard / Keepers (Remove-only); the newest 58d ago, pc · Standard /
  Dump Tab); stash listing 58d ago". Buttons "Refresh selected" and
  "Plan under the policy". A table "Stash tabs (3347)" (Tab, Type,
  Items, Fetched) with folders and lines like "246 pages of >>>>> MAPS
  <<<<< (Remove-only)" under map and unique tabs, and "the substashes
  box: also fetch a ticked map or unique tab's pages"; then
  "Characters (41)" with level, items and age.
- **Jobs** (`/jobs`): "rails: tripwire off · sends 2 since this daemon
  started · journal /Users/tom/seat-worlds/web-2026-11/cards-opus/logs/…/mock/sends.jsonl",
  a table with one row (`1 … profile … done … seat … daemon … finished
  5m ago`) and "1 job this daemon holds (its own and the restored): 0
  running, 0 waiting, 1 done".

What each seems to be for: Search is the main thing, three ways in
(form, query language, trade URL). Saved keeps searches. Refresh
chooses tabs to fetch from the game. Jobs shows what the daemon is
doing.

First surprises, before any control: the collection's Jobs page shows
a job of the account `seat`, so the daemon looks shared between both
page servers. Refresh says "3347 tabs" in one line and "facts: 3331
tabs" in the next (3331 + 16 folders = 3347, so probably folders
counted or not, but the page doesn't say so). "20231 items in tabs"
is not the 22,721 the brief gives the collection; "(characters apart)"
probably covers the difference. The Refresh page is on the collection's
pages even though the brief says this account cannot refresh: nothing
on the page says so.

What I would do first: Item Category = Ring, Rarity = Rare, a stat
row for "maximum Life" with min 70, Search, cards.

## The tasks

### T0 — Orient

See "Before touching anything" above. Pane tools: `preview_start`,
`get_page_text` on each of the four pages, `javascript_tool` for the
menu links and form field names, one `computer` screenshot.

### T1 — Rings for life, as cards

**What I did.** On the Search form: typed "maximum Life" into "Find a
stat" (a live list appears under it; each entry like "+# to maximum
Life explicit · alone exact · with a bound exact"). There is no pseudo
or "total" life stat: typing "pseudo" or "pseudo total maximum Life"
listed nothing and said nothing. "+# to maximum Life" exists three
times, as `explicit`, `implicit` and `fractured` (read with
`javascript_tool` from the list); no `crafted` entry in the first 50
("11 more match: add a word"). I chose the explicit one; the stat box
filled with `explicit.stat_3299347043` and the words "+# to maximum
Life" under it (screenshot). Min 70; Item Category = Ring; Item Rarity
= Rare (`form_input`); Search (`computer` click by ref). The form sent
(read with `javascript_tool` before submitting):
`category.option=accessory.ring`, `rarity.option=rare`,
`stats[0].type=and`, `filters[0].id=explicit.stat_3299347043`,
`value.min=70`, `view.limit=20`.

**What the page said** (`get_page_text`):

> Answer / 103 matches / from the query JSON: exact — every clause with
> a stash meaning ran as the site reads it
>
> Query category="accessory.ring" rarity=rare sum(line("# to maximum
> Life" source=explicit).arg1)>=70
>
> Scope account GERWARIC#7694 · pc · live · 22623 items · 546
> locations fetched / 2940 locations never fetched: MapStash 2489 of
> 2582, UniqueStash 437 of 521, character 14 of 59 (counted over every
> location in the realm, in every league, whatever the query asks,
> since what a never-fetched location holds is unknown) · location lists
> seen, the oldest 61d ago · each tab of Standard
>
> Total 103 matches (the oldest fetch 61d ago, pc · Allflame /
> TheAbsenceOfPatience (character); the newest 58d ago, pc · Standard /
> Dump Tab) / 2940 locations were never fetched: nothing here says what
> they hold

**How many:** 103, from the page's "103 matches"; 20 shown, "rows 1–20
of 103 shown · more".

**How each item shows (cards).** A card per item, with item art
(an `<img>` per card; the screenshot shows a ring picture), the name
line as a link ("Damnation Circle Ruby Ring · rare · frame rare"), the
base, quality, item level, requirement, every mod line with its source
after it in small grey text ("+78 to maximum Life explicit", "+1 to
Minimum Endurance Charges explicit, crafted", "+33% to Fire Resistance
implicit", enchants marked "enchant"), "is identified" / "is corrupted,
identified" / "is elder, identified", sockets ("sockets G · links 1"),
"price none (nothing applies)", the place ("pc · Standard /
TheFireworksCompany (equipment) · fetched 61d ago", or for a tab
"pc · Standard / MARKET · x 2, y 9 · fetched 61d ago"), and then why it
matched: "category accessory.ring / sum(line("# to maximum Life"
source=explicit).arg1) 78 / +78 to maximum Life (explicit)".

What one entry tells me: everything a player sees on the item in game,
plus where it is and how old that knowledge is, and the value that
made it match. What it does not tell me: the mods' tiers or ranges, the
total life counting implicits (the Coral Ring "Empyrean Knot" shows
"+78 … explicit" and "+22 to maximum Life implicit", and the answer
counts 78), whether any other ring was missed because its life is split
between implicit and explicit, and a price beyond "none". One price
line read "price none (nothing applies: the price text in game is not
where the index can see it)" on an item in the tab "~price 30 chaos
(C)" — the tab's own name is a price note, and the page says it cannot
see it.

**The order** is not by life: 78, 71, 111, 74, 110, … The page does
not say what order it used. Equipped items come first, then tabs.

**The other way: a table.** The link "as a table" just above the cards
(the link adds `view.fields=icon`; followed by coordinate click). The
table (`get_page_text`) has columns "Item / Lines / Place": the name and
base and rarity; only the matching lines ("category accessory.ring",
"sum(...) 78", "+78 to maximum Life (explicit)"); and the place without
the realm. The table's link back is "as cards". Same header, same 103,
same order.

What each is good for: cards to judge an item whole without opening it
(every mod, sockets, corruption); the table to scan many matches for
the one number asked and where each sits. The table has no art and
no other mod.

**Under the answer, both ways** (`get_page_text`):

> Terms: each term counted alone over all live pc items in all
> leagues, never over the matches
>
> Path Term Matched Failed Lacked Undecided Only together
> 0 category="accessory.ring" 1133 21489 1 0
> 1 rarity=rare 5769 8600 8254 0
> 2 sum(line("# to maximum Life" source=explicit).arg1)>=70 762 21861 0 0
>
> 8 counts carry no link: an answer carries those routes only where
> every route is asked, which no page asks

and a footer "basis store 3e5e1dec9df9 · build 8ee723165fba · snapshot
1126 · facts v9 · …", then the query-language form and "Name  Save this
search".

**What I expected.** A ring search shaped like the trade site, with
"pseudo total maximum Life" offered. It isn't, so "at least 70 to
maximum life" became "at least 70 explicit". I did not get an answer
counting implicit life from the form; T2 tries the language.

Noticed in passing: every card's art is an image from
`https://web.poecdn.com/gen/image/…` (read with `javascript_tool`, each
`naturalWidth` 47, so they loaded). The pages fetch from the game's
CDN while the trade form says "translated here, sent nowhere".

### T2 — The same, typed and sorted

**How I learned the words.** Not from the pages. The answer to T1
printed the query in the language ("Query category="accessory.ring"
rarity=rare sum(line("# to maximum Life" source=explicit).arg1)>=70")
and prefilled it into the box "The query, in the language" under the
answer, so I could copy it. But no page names a reference: the box has
no placeholder, no title, no help link (read with `javascript_tool`).
The "Sort by" box shows only the placeholder "ilvl". For the words I
went to the door: `acq search --describe` **[call 1]**, 33 KB: fields,
`line( … )`, `sum`, operators, slots, 82 "computed values" including
`pseudo.total_life` "the trade site's pseudo_total_life (`+# total
maximum Life`)", categories, counts, and "limits".

**First try, a refusal.** On the front page's language form I typed
`category=accessory.ring rarity=rare sum(line("# to maximum Life"
source=explicit).arg1)>=70`, Sort by `sum(line("# to maximum Life"
source=explicit).arg1)`, ticked "highest first", Show 200, "Run this
query". The pane's tab note said "the page in this tab was served with
HTTP status 400", and the page (`get_page_text`) said only:

> Answer / a value that is more than letters, digits and underscores is
> quoted / "accessory.ring"

Clear about what to do, but the refusal page has no form, no link and
no menu inside `<main>` (`javascript_tool`: 0 forms, 0 links): to fix it
I edited the address bar (`%22accessory.ring%22`). The form's own
Category dropdown had produced `category="accessory.ring"` quoted, so
copying the printed query works; typing it as the trade site spells it
does not.

**Second try, the answer** (`navigate` to the edited address,
`javascript_tool` reading `<main>`'s text): "103 matches", the same
Query line, Scope and Total as T1. **The count agrees with T1: 103 and
103.** All 103 cards are on the page (103 item links counted with
`javascript_tool`), sorted by explicit life from 248 down to 70; each
card now starts its "why" block with "sorts by sum(line("# to maximum
Life" source=explicit).arg1) 248".

**What the page says about rows.** With all 103 shown, nothing: no
"rows … shown" line at all (searched the text with `javascript_tool`).
With fewer shown than matched it says "rows 1–20 of 103 shown · more".
The Show menu offers 10, 20, 50, 100, 200 and nowhere says why 200 is
the top. The address takes more: `view.limit=500` on `category=
"accessory.ring" rarity=rare` gave "730 matches" and "rows 1–500 of 730
shown · more" (500 item links), and `view.limit=5000` gave all 730 with
no rows line. So the 200 is the menu's, not the tool's, and the page
does not say so.

The language form has no "Results as cards / a table" control; its
answer comes as cards unless `view.fields=icon` is in the address (the
"as a table" link adds it).

**The total life, which the form could not ask** (address edited):
`category="accessory.ring" rarity=rare pseudo.total_life>=70`, sort
`pseudo.total_life`, highest first, 200, as a table: "132 matches",
all shown. Rows show how the total is made: "Rage Finger Coral Ring …
sorts by pseudo.total_life 142 / +117 to maximum Life (explicit) / +25
to maximum Life (implicit)"; "Fate Band Two-Stone Ring … pseudo.
total_life 131 / +48 to Strength (explicit) / +107 to maximum Life
(explicit)"; "Victory Grip Breach Ring … sorts by pseudo.total_life
117.5 / +39 to Strength (explicit) / +98 to maximum Life (explicit)".
So the total counts Strength (half of it), as the trade site's pseudo
does (my knowledge, not the tool's words; the tool shows the Strength
line beside the total but never says it adds half of it). 132 is the
answer to "at least 70 total life" counting Strength; 103 is "at least
70 on the explicit life line". Neither page says which a player meant.

Terms for this one: "2 pseudo.total_life>=70 879 1689 20055 0" — 20055
"Lacked", and "9 counts carry no link".

Top of both lists: "Damnation Whorl Two-Stone Ring … +248 to maximum
Life explicit … Adds 19 to 39 Cold Damage to Attacks against you
explicit … -41 to Evasion Rating explicit … is duplicated, identified"
in "Items 4 (Remove-only)". +248 on a ring and a "against you" line
look like no ring the game rolls (my knowledge); the page shows it
plainly, with no remark.

### T3 — Follow the links

On the T2 answer at Show 20 (explicit life, highest first), I listed
every link inside `<main>` that is not an item (`javascript_tool`).
There are exactly three:

1. **"each tab of Standard"**, at the end of the Scope's never-fetched
   sentence ("… · location lists seen, the oldest 61d ago · each tab of
   Standard"). Lands on `/refresh?realm=pc&league=Standard` (`navigate`
   to its href, then `javascript_tool`): the same Refresh page the menu
   opens, "3347 tabs in Standard: 405 fetched, 2926 never fetched, 16
   folders (never fetched) …", with every tab and its "Fetched" age.
   It matches its words: every tab of Standard is there. But the
   sentence it ends is about the 2940 never-fetched locations, and the
   page it opens does not pick those out (no filter for never fetched;
   the visible list shows mostly fetched tabs, the never-fetched map
   and unique pages folded as "246 pages of …"), and the answer also
   holds an item in Allflame ("Blood Spiral Bone Ring … pc · Allflame /
   TheAbsenceOfPatience"), which a Standard page does not cover.
2. **"as a table"** (and on the table, "as cards"), above the first
   card. Lands on the same answer with `view.fields=icon` added: the
   same 103, the same order, the table described in T1. Matches.
3. **"more"**, in "rows 1–20 of 103 shown · more". Lands on
   `search?next=acq1.…` (a long opaque token; followed by `computer`
   click by ref, then read with `javascript_tool`): "rows 21–40 of 103
   shown · more", the next twenty in the same order (106 down to 94).
   Matches. But the page reached by "more" has **no "as a table" link,
   no way back to rows 1–20 and no "first"**; only "more" again. Its
   query box and its save form still carry the query.

**Numbers and words I expected to lead somewhere and did not:**
"103 matches"; "22623 items"; "546 locations fetched"; "2940 locations
never fetched" and each of "MapStash 2489 of 2582", "UniqueStash 437 of
521", "character 14 of 59" (I expected each to list those locations);
the oldest and newest fetch places ("pc · Allflame /
TheAbsenceOfPatience (character)", "pc · Standard / Dump Tab"); every
card's place ("Standard / MARKET · x 2, y 9") — I expected a tab name
to open that tab's items; and every number in the Terms table
(Matched 1133, Failed 21489, Lacked 1, …). For the Terms numbers the
page says why, in its own words: "8 counts carry no link: an answer
carries those routes only where every route is asked, which no page
asks". I do not understand that sentence well enough to say what
would make a page "ask every route"; what it plainly says is that no
page ever will, so on the pages those counts never link.

### T4 — A friend's search

**What I did.** Front page, the box "A trade site search: its URL, its
id or its query JSON", pasted the friend's URL whole (typed with
`computer`, value checked with `javascript_tool`), "Run the site's
search". The address became `search?realm=pc&trade=…`.

**What the page said** (`get_page_text`):

> Answer / 732 matches / not the site's search: 2 clauses not run /
> from a URL: PARTIAL — 2 clauses not run, so this is not the site's
> search
>
> Status Clause Says
> NOT RUN stats.1 A crucible tree path is not read; not run.
> NOT RUN stats.1.0 its group `stats.1` translates whole or not at all,
> and is unsupported
> reported status Online, available or securable is the seller's
> state, not the item's, so it means nothing in your stash.
>
> Query league=Standard sum(line("# to maximum Life"
> source=explicit).arg1)>=70

**How many:** "732 matches", "rows 1–10 of 732 shown · more" — from the
page. The answer is any item at all (gloves, helmets, body armour, a
unique amulet "Defiance of Destiny") in Standard with an explicit life
line of 70 or more.

**How faithfully, by the page's own words:** it says plainly it is not
the friend's search ("PARTIAL"), names the clause it could not run (a
crucible tree path, `stats.1`, and its one filter `stats.1.0`), and
says what it did with the status ("reported", and why). It counts that
as "2 clauses", though `stats.1.0` is the inside of `stats.1`; to me it
is one thing not run. The page does not show the friend's search as
the site would show it (no "Category", "Status: Online" etc. in the
site's words), only the translated query; to check the translation I
decoded the URL's own string myself (a `python3 -I -c` one-liner over
the string in the brief, nothing else read): `status online`; stats
group 0 `and`, `explicit.stat_3299347043` min 70; stats group 1 type
`crucible` with the same stat id. So the translation is right: the
friend asked no category and no rarity, the league `Standard` came from
the URL (as the front page's note says: "The league the URL names … becomes a term of the search").

Noticed: the scope line now says "location lists seen, the oldest 58d
ago", where every T1–T3 answer said "the oldest 61d ago", over the same
store and the same "2940 locations never fetched". Nothing on the page
says why one answer's lists are three days younger.

**Saving.** Under the answer: "Name [ ] Save this search". Its form
(`javascript_tool`) carries hidden `text` = the translated query,
`realm=pc`, `membership=live` and the name — not the trade URL and not
the "PARTIAL" report. Named it `friend life 70`, Return. Landed on
Saved (`get_page_text`), with no word that anything was saved beyond
the list itself:

> 1 saved search · the file at revision 1
> Name Scope Query Revision Last written Id
> friend life 70 pc · live league=Standard sum(line("# to maximum Life"
> source=explicit).arg1)>=70 1 0s ago via web 5bd947c89986
> Rename / Clear

**Opening it from Saved.** The name link (`saved/open?id=5bd947c89986`;
a click by ref did nothing, a click by coordinate worked, as the brief
warned). Landed on (`javascript_tool`):

> Answer / Saved search "friend life 70", opened at revision 1 · the
> saved searches / 732 matches / Query league=Standard sum(line("# to
> maximum Life" source=explicit).arg1)>=70

with the same first items and "rows 1–10 of 732 shown", and at the
foot "Save over "friend life 70"".

**Is what opens what I ran?** The query and the 732 are the same. But
the opened search has lost everything that said it was not the
friend's search: no "PARTIAL", no "not the site's search: 2 clauses
not run", no Status/Clause table. Someone opening "friend life 70" next
week would see a plain answer and nothing saying the crucible part was
dropped. So: the same query, not the same answer page, and the part
that mattered for trusting it is gone.

### T5 — One item

**What I did.** Opened "Empyrean Knot Coral Ring" — the ring from T1
whose card shows both an explicit and an implicit life line — through
its card's name link (`item/51e6c1055b1d…c682`; I opened the href the
T1 answer gave, with `navigate`). Read with `get_page_text` and
`javascript_tool`.

**What the item page says** (`get_page_text`): the same card as in the
answer (art, name "Empyrean Knot Coral Ring · rare · frame rare",
"Coral Ring", "Item Level: 83", "Requires Level 64", the six lines with
their sources, "is identified", "price none (nothing applies)", "pc ·
Standard / Decent Gear · x 2, y 2 · fetched 61d ago"), then a block the
answer did not have:

> Id 51e6c1055b1d94f8c53921012d459ec5d2f4e353f674f4b03192c79375c4c682
> Class Rings
> Category accessory.ring
> Unread nothing
> Source Line Template Slots
> explicit +16 to Strength "# to Strength" arg1 16
> explicit +78 to maximum Life "# to maximum Life" arg1 78
> …
> explicit, crafted +15% to Cold and Chaos Resistances "#% to Cold and
> Chaos Resistances" arg1 15
> implicit +22 to maximum Life "# to maximum Life" arg1 22

and the basis footer.

**What the answer's entry told me.** Everything above the Id: the full
card, plus why it matched ("category accessory.ring / sum(line("# to
maximum Life" source=explicit).arg1) 78 / +78 to maximum Life
(explicit)"). The item page adds the id, the class, "Unread nothing",
and each line's template and slots — which is the language's
vocabulary for this item: useful for writing the next query (now I know
the implicit life is `line("# to maximum Life" source=implicit)`), not
for playing.

**What I wanted from either and did not get:**
- The totals. Neither the card nor the item page says this ring's total
  life (the language has `pseudo.total_life`; by its T2 behaviour this
  ring would be 78 + 22 + 16/2 = 108 — my arithmetic, not the page's).
  No total resistance either, the number a player reads a ring for.
- Why it matched, on the item page: the item page forgets which answer
  I came from. It has no link back to the answer, no link at all
  (`javascript_tool`: 0 links, 0 forms in `<main>`), and the menu's
  "Search" opens a blank form. The tour's `acq show --against` (a
  query's outcome on this item, term by term) has no page.
- Mod tiers or roll ranges, so I cannot tell a good 78 from a bad one.
- The tab "Decent Gear" as a link to the rest of that tab.
- A price: "price none (nothing applies)", and nowhere to set one.

### T6 — The terminal and the page

**List** **[call 2]** `acq saved list`:

> 1 saved search (the file at revision 1):
>   friend life 70  pc live  league=Standard sum(line("# to maximum
>   Life" source=explicit).arg1)>=70  (revision 1)  5bd947c89986
> next: `acq search --saved ID` runs one; `acq saved show ID` shows one

The search saved on the page is there, same id, same query. Like the
page, nothing in it says the friend's crucible clause was dropped.

**Run it** **[call 3]** `acq search --saved 'friend life 70' --limit 5`
→ exit 1: "Error: 2 accounts are known and one is to be picked:
GERWARIC#7694, seat; pick one with --account or ACQ_ACCOUNT", offering
two command lines. The saved row's scope is "pc · live", no account;
the page had run it as GERWARIC#7694 without asking. **[call 4]** the
first offered line, `acq --account 'GERWARIC#7694' search --limit=5
'--saved=friend life 70'` → exit 0: "total 732 matches", the same first
five items as the page in the same order (Damnation Circle Ruby Ring
78, Eagle Clutches Goliath Gauntlets 70, Death Grasp Amethyst Ring 71,
…). Same answer as the page.

Two differences from the page in the same answer: the scope's
never-fetched sentence ends "each tab: acq tabs --account
'GERWARIC#7694' --realm pc --league Standard --never-fetched" — the
terminal gives the never-fetched list I wanted from the page's "each
tab of Standard" link in T3; and "routes each returns exactly the
members counted (4 more: --routes)" where the page said "4 counts carry
no link … which no page asks".

**The rings with each row carrying the whole item.** How I found out:
`acq search --help` **[call 5]**, under `--fields`: "What each row
carries beside its compact fields, by name: `icon`, the URL of the
item's art; `item`, the item as `show` gives it — its lines, class,
category and price. Under `item` an answer holds at most 200 rows; the
rest are its continuation". The same help says of `--sort`: "A sum
spans every source the line has — implicit and crafted with explicit
… one source's is its form, `'sum(line("T" source=explicit).arg1)'`"
— a third reading of "total life", not offered on any page.

**[call 6]** the rings query, sorted, `--limit 2 --fields icon,item`,
without `--realm` → exit 1: "Error: this store holds 2 realms and a
search names one in its scope: pc | poe2, or all" (the pages never
asked; they default to pc). **[call 7]** with `--realm pc` → exit 0,
"103 matches" (agrees with T1 and T2), sorted 248, 136, … Each row:

> Damnation Whorl Two-Stone Ring · rare · frame rare · Standard / Items
> 4 (Remove-only) · x 8, y 6 · fetched 61d ago
>     sorts by sum(...) 248 · +248 to maximum Life (explicit) · category
>     accessory.ring · sum(...) 248
>     id 8e265a30…5401
>     icon https://web.poecdn.com/gen/image/…/TopazSapphire.png
> base    Two-Stone Ring
> class   Rings
> category accessory.ring
> price   none (nothing applies) · why: acq price show --account
>         'GERWARIC#7694' --realm pc item/8e265a30…5401
> is      duplicated, identified
> shown   Item Level: 83 / Requires Level 35
> lines   explicit  Adds 19 to 39 Cold Damage to Attacks against you
>                   "Adds # to # Cold Damage to Attacks against you"
>                   low 19 · high 39 · avg 29 · arg1 19 · arg2 39
>         …
> unread  nothing

So **yes, the command line can**: everything the item page shows (name,
base, class, category, place, id, art URL, price, flags, the shown
properties, each line with source, template and slots, "unread"), and a
bit more (a `why` command for the price; `low/high/avg` slots on ranged
lines). The page cannot: its cards show the lines but not the
templates/slots/class/id; its table shows less. What it looks like: a
block per row, but the item's blocks (`base`, `class`, `category`,
`price`, `is`, `shown`, `lines`, `unread`) start at column 0, at the
same indent as the answer's own `query`, `scope`, `rows`, `more`,
`terms`; reading it, the second item's header looks like a
continuation of the first's `unread`, and only the blank line tells the
rows apart. At most 200 rows per answer with `item`.

### T7 — The refresh loop

All on the rehearsal pages (`…:64074/272a…/`) unless a door call is
named.

1. **Front page.** The same Search form as the collection's. Nothing on
   it, or in the menu, says which account these pages are over
   (`javascript_tool` over the menu and `<main>`); I learned it was
   `seat` only from the Refresh page's messages.
2. **Refresh** (`get_page_text`): "no tabs known for Standard in
   /Users/tom/seat-worlds/web-2026-11/cards-opus/store/mock/seat.db (run
   `acq stashes --account seat --realm pc --league Standard` or `acq
   refresh --all --account seat --realm pc --league Standard`) / facts:
   no tabs; stash never listed / … no characters known in Standard (run
   `acq characters` or cover them in the sync policy)". There was no
   league control at all, only Realm and "Show"; the page assumed
   Standard.
3. **"Fetch the league list"** (a POST; `computer` click). Landed on
   **Jobs**, not back on Refresh: "2 leagues done 0 seat web:824
   finished 3s ago". No list of leagues there, no link back. Reopening
   Refresh: a **League** select had appeared: "Rehearsal (no tabs on
   disk) / Standard (no tabs on disk) / Hardcore (no tabs on disk)".
4. **League = Rehearsal, Show** → "no tabs known for Rehearsal in … (run
   `acq stashes --account seat --realm pc --league Rehearsal` or `acq
   refresh --all …`)". **The pages have no control that lists a
   league's tabs.** I tried "Plan under the policy" → HTTP 400 (tab
   note) and "no sync policy is set for seat — declare one first, e.g.
   `acq policy set --account seat --if-revision 0 '{"version":3,…}'`".
   So the page left me no way on: to the door.
5. **[call 8]** `acq stashes --account seat --realm pc --league
   Rehearsal` → exit 0, raw JSON (no `--json` asked): 6 entries,
   Currency (rh-cur1), Dump (rh-dump), a folder "Old leagues" (rh-fold)
   holding "Old 1" (rh-old1, `public: true`) and "Old 2" (rh-old2),
   Uniques (rh-uniq), Maps (rh-maps), "Maps (empty)" (rh-mape); stderr
   "job 3 (seat): waiting / job 3 (seat): done; …".
6. **Refresh, Rehearsal again**: "8 tabs in Rehearsal: 0 fetched, 7
   never fetched, 1 folder (never fetched); 0 items in tabs (characters
   apart) / facts: 7 tabs, none fetched; stash listing 6s ago", a table
   of seven tabs each with a checkbox, Items "-", Fetched "never", per-
   column filter boxes, and "the substashes box: also fetch a ticked
   map or unique tab's pages". In the screenshot, Uniques, Maps and
   Maps (empty) sit at the same indent as the folder's Old 1 and Old 2,
   so they look like they are in "Old leagues"; the JSON says they are
   not.
7. **Chose** Dump, Old 1, Uniques and ticked the substashes box
   (`form_input`; form checked with `javascript_tool`: `deep=true`,
   `tab=rh-dump`, `tab=rh-old1`, `tab=rh-uniq`). **"Refresh selected"**
   → the **Plan** page (`get_page_text`):

   > refresh Rehearsal on mock as seat — a selection, with its map and
   > unique tabs' pages / facts: stash listed 19s before the plan; plan
   > made 0s ago / the listing is fetched beside the selection: what it
   > newly lists takes the next plan / pages of "Uniques": none to fetch
   > on record — pages not known yet arrive with its fetch; apply, then
   > select it again for them / 4 requests: list stashes … fetch 3 tabs
   > … / quote (0s ago): stash-list-request-limit 1 request — ready;
   > stash 3 requests — ETA unavailable (unlearned) / wire: 4–12 sends;
   > plus first-contact probes and token refresh if needed / Apply

   Clear about what it will do. The Apply form carries the whole plan
   as JSON in a hidden field.
8. **Apply** → Jobs: "5 apply plan (4 requests) running … 6 5 stashes
   Rehearsal running … 8 5 stash Rehearsal/rh-old1 waiting … ~0s"; the
   page does not update itself; reopening it 5 s later: all "done",
   "10 jobs this daemon holds …: 0 running, 0 waiting, 10 done". The
   Jobs page has no link anywhere (`javascript_tool`: 0 links in
   `<main>`), no outcome or item count per job, and no way back to the
   Refresh or the plan.
9. **What landed** (Refresh, reopened): "8 tabs in Rehearsal: 3
   fetched, 4 never fetched, 1 folder (never fetched); **0 items in
   tabs**", Dump 0, Old 1 0, Uniques 0, each "15s ago". A search on
   these pages, `league=Rehearsal` (address edited): "0 matches … Scope
   account seat · pc · live · 0 items · 3 locations fetched … nothing in
   scope carries the league of term 0: league=Rehearsal — a line the
   search cannot name is shown as unknown, with what it knows of it,
   and never fuzzy-matched", then "Offers: each part of the query forced
   true and forced false" with links "part 0: league=Rehearsal" and
   "every item in scope, this query set aside".
10. **"Apply, then select it again"** for the Uniques pages: I planned
    Uniques and Maps with the substashes box (address of the plan form
    with `tab=rh-uniq&tab=rh-maps&deep=true`). The plan said the same
    "pages of "Uniques": none to fetch on record — … apply, then select
    it again for them" and the same for Maps; applied; Refresh: "4
    fetched … 0 items", no page lines under Uniques or Maps.
11. **What came back, from the door** — the pages give no way to see a
    job's result. **[call 9]** `acq --account seat jobs result 7` → exit
    2, "error: unexpected argument 'result' found" (I read the tour's
    "acq jobs [--watch] [--all] | status <id> | result <id> [--wait]" as
    sub-verbs of `jobs`; they are verbs of their own). **[call 10]**
    `acq --account seat result 7` → the Dump fetch: `"items": []`,
    `"substash_jobs": []`, `"substashes_listed": 0`. **[call 11]**
    `result 14` (Maps, second apply): the same, `"items": []`,
    `"substashes_listed": 0`. So the mock returned empty tabs and no
    pages; "0 items" is the truth, not a lost fetch.
12. **Characters**: the Refresh page only says "no characters known in
    Rehearsal (run `acq characters` …)". **[call 12]** `acq characters
    --account seat` → two characters, both in **Standard**
    ("StashHoarder" Scion 97, "MuleQuadTab" Witch 12): the account holds
    no characters in Rehearsal.

**Where I did not know what had happened.** (a) After "Fetch the league
list" I was dropped on Jobs with no league list and had to guess that
Refresh had changed. (b) After Apply, Jobs said "done" for every job and
the Refresh page said "0" items: I could not tell from the pages whether
the fetch brought back empty tabs or failed to store what it got, until
the door's `result` showed `"items": []`. (c) The plan's "apply, then
select it again for them" for the Uniques pages never resolved: after
the second apply the page still showed no pages and said nothing about
why (the door shows the mock listed none). (d) The Jobs page also
showed, on the collection's pages before I did anything, jobs of `seat`
— the one daemon serves both page servers, and its list is of all
accounts.

What the account holds, in sum (pages + door): leagues Rehearsal,
Standard, Hardcore; in Rehearsal 7 tabs in one folder and six top-level
entries, all empty at the mock; two characters, both in Standard.

### T8 — As a player

Three things a player would want from these pages that I could not do,
or could not find how to:

1. **Search by the totals a player thinks in.** "Rings with 70+ total
   life and 80+ total resistance" is the first search any player
   makes. The form's "Find a stat" offers no pseudo/total stat at all
   (T1), so the form can only ask one source of one line; the language
   has `pseudo.total_life` and `pseudo.total_res` (T2, [call 1]), but
   only someone who went to the command line for `--describe` would
   know. And no card or item page shows an item's totals (T5).
2. **Go from an item to where it sits, and see that tab.** Every card
   says "Standard / Decent Gear · x 2, y 2", but neither the place nor
   the tab is a link, and no page shows a tab's contents (T3, T5). To
   pull an item out of 3347 tabs a player wants the tab and its
   neighbours.
3. **Price things and see what they're worth.** Every item read "price
   none (nothing applies)"; one even said "the price text in game is
   not where the index can see it" while sitting in a tab named
   "~price 30 chaos (C)". The tour lists `acq price set` and `acq shop
   render`, but the pages have no price control and no shop view (the
   form's Trade Filters has a price filter, which I did not try). Selling is half of why a player
   searches their own stash.

## Faults, worst first

1. **A saved trade search forgets that it was partial.** (T4)
   I ran the friend's URL: the answer said "PARTIAL — 2 clauses not run,
   so this is not the site's search" and named the crucible clause.
   Saved it; opened it from Saved: the same 732 and query, but no
   "PARTIAL", no "not run", no clause table. The saved row ([call 2]
   too) keeps only the translated text. Should: keep the trade source
   and its fidelity report with the saved search and show them on
   opening.
2. **The form's stat finder has no total/pseudo stats.** (T1)
   Typed "maximum Life" and "pseudo": only explicit/implicit/fractured
   lines. The language has 82 computed totals including
   `pseudo.total_life` ([call 1]); a ring with life split across
   implicit and explicit is silently missed by the form (103 vs 132).
   Should: offer the totals in "Find a stat", as the trade site does.
3. **The pages cannot list a league's tabs or plan without a policy;
   they send you to the command line.** (T7)
   Refresh for Rehearsal: "no tabs known … (run `acq stashes …`)";
   "Plan under the policy": HTTP 400, "no sync policy is set for seat —
   declare one first, e.g. `acq policy set …`". Should: a button that
   lists the league's tabs (the page already has "Fetch the league
   list"), and a way to set or show the policy.
4. **No page tells you what a fetch brought back.** (T7)
   After Apply, Jobs says "done" with no outcome, count or link, and
   Refresh says "0" items. Only the door's `acq result 7` [call 10]
   showed `"items": []`. Should: Jobs (or the Refresh page) shows each
   fetch's outcome and item count, and empty-vs-failed plainly.
5. **The trade site's own category spelling is refused, on a page with
   no way back.** (T2)
   `category=accessory.ring` → HTTP 400 "a value that is more than
   letters, digits and underscores is quoted / "accessory.ring"", on a
   page with no form and no link. Should: show the query box again with
   the error beside it (or accept the dotted id unquoted).
6. **"more" loses the table link and has no way back.** (T3)
   Page 2 ("rows 21–40 of 103") has only "more"; no "as a table", no
   previous/first. Should: keep the view links and add previous.
7. **The item page is a dead end.** (T5)
   No links at all: not back to the answer, not to the tab, not "why
   this matched". Should: link back, link the place, show `show
   --against` for the answer it came from.
8. **"each tab of Standard" does not show what its sentence is about.**
   (T3) It ends the "2940 locations never fetched" sentence but opens
   the whole Refresh list for Standard, with no never-fetched filter and
   not the other league the answer holds (Allflame). The terminal's
   version is `acq tabs … --never-fetched` ([call 4]). Should: link to
   the never-fetched list, by league.
9. **"Fetch the league list" lands on Jobs and shows no leagues.** (T7)
   Should: return to Refresh with the league select filled and say so.
10. **The Show menu stops at 200; the address takes 5000; nothing says
    which is the limit.** (T2) Should: say on the page why 200, or
    offer "all".
11. **The rehearsal pages never name their account.** (T7) Only the
    Refresh page's error text ("--account seat") revealed it. Should:
    the account in the header.
12. **The pages fetch item art from `web.poecdn.com`.** (T1) A tool
    whose trade form promises "translated here, sent nowhere" makes a
    request to the game's CDN per card. Should: say so, or serve the art
    locally / on request.
13. **The answer's default order is unexplained.** (T1) 78, 71, 111,
    74, 110 … with nothing saying what it is sorted by.
14. **CLI rows with `--fields item` are hard to read.** (T6) The item's
    `base`/`class`/`lines`… start at column 0 like the answer's own
    `scope`/`more`/`terms`.
15. **Refresh's table indents top-level tabs as if in the folder above.**
    (T7, screenshot) Uniques, Maps, Maps (empty) look like members of
    "Old leagues".
16. **Small unexplained numbers.** (T0, T4) Refresh says "3347 tabs" and
    "facts: 3331 tabs" on adjacent lines; answers say "the oldest 61d
    ago" for location lists, but "58d ago" when `league=Standard` is a
    term — neither explained.

## What the tour and the pages did not say

- Where to learn the query language. The tour has `acq search
  --describe`; the pages, which show the language and invite you to
  type it, never point to it.
- That the form's Category dropdown quotes the value and typing it bare
  is an error.
- That `pseudo.total_life` counts half of Strength (seen on rows; never
  said), and that `--sort 'sum("T")'` without `source=` spans every
  source ([call 5]) — three readings of "life", none named on a page.
- What the default order of an answer is.
- That the "Show" limit is the menu's, not the tool's.
- That the Refresh page needs a tab listing first, and that the only
  way to get one is the command line; that planning needs a policy.
- That one daemon and one Jobs list serve both accounts' pages.
- That a saved search keeps no account (the CLI then refuses, [call 3])
  and no trade source.
- In the tour: `acq jobs … | status <id> | result <id>` reads as
  sub-verbs of `jobs`; they are top-level verbs ([call 9] refused,
  [call 10] worked).
- That a store of two realms makes every CLI search need `--realm`
  ([call 6]); the pages default to pc silently.

## The tool's own words

Quoted, words or phrases I did not understand or could not act on:

- "8 counts carry no link: an answer carries those routes only where
  every route is asked, which no page asks" (every answer).
- "frame rare" (on every card, after "rare").
- "from the query JSON: exact — every clause with a stash meaning ran as
  the site reads it" (the form's answer): which clauses have no "stash
  meaning"?
- "Over the items the store has fetched, priced or not — what it never
  fetched is the scope's: the site shows public listings, a tab's being
  tab.public, so the search is the site's and the items are yours".
- "Only together" (a Terms column, always empty).
- "Unread nothing" (item page).
- "alone exact · with a bound ambiguous" (each stat in the finder).
- "listing only" (struck through beside Status on the form).
- "wire: 4–12 sends; plus first-contact probes and token refresh if
  needed" and "ETA unavailable (unlearned)" (Plan).
- "rails: tripwire off" (Jobs).
- "nothing in scope carries the league of term 0: league=Rehearsal — a
  line the search cannot name is shown as unknown, with what it knows of
  it, and never fuzzy-matched" (the second half is about lines, not
  leagues).
- "basis store 3e5e1dec9df9 · build … · derivation 12 · classes v1 · …"
  (every footer).
- "the file at revision 1" (Saved).

## What I would ask of the tool

- Put the totals (`pseudo.*`) in "Find a stat", and show an item's
  totals on its card and its page.
- Keep a trade search's fidelity report with the saved search.
- On Refresh: a "list this league's tabs" button, and a way to see and
  set the policy, so the loop never needs the terminal.
- On Jobs: each job's outcome and item count, linked to what landed.
- Make the place on a card a link to the tab; give the item page a way
  back and a "why it matched".
- On a refused query, show the query box again with the error.
- A link to the language reference beside "The query, in the language".
- Name the account in the header of every page.

## Where I left the page for the terminal, and why

- T2, [call 1] `acq search --describe`: the pages show the language but
  name no reference for its words.
- T6, [calls 2–7]: the task asked for the door.
- T7, [call 8] `acq stashes …`: Refresh had no way to list Rehearsal's
  tabs and named this command; "Plan under the policy" refused without a
  policy.
- T7, [calls 9–11] `acq result …`: no page shows what a fetch returned,
  and the pages said "0 items" with no way to tell empty from failed.
- T7, [call 12] `acq characters`: the Refresh page has no control to
  fetch characters and named this command.

## What I read or ran outside the world

- To decode the friend's URL I ran one `python3 -I -c` one-liner over
  the string printed in the brief (base64 + gzip + JSON); it read no
  file and touched nothing of the tool.
- The door's output for [call 1] was 33 KB; the harness I run in saved
  it to a file in its own folder
  (`~/.claude/projects/…/tool-results/bqi8c09v9.txt`) and I read that
  file to see the rest of the output. It holds exactly the door's
  printout, nothing else; but it is a file outside `W`.
- I loaded my own instructions for the browser pane (a skill of the
  harness, about the pane, not the tool).
- Nothing else: no source, no other document, no web, no other address
  than the two pages and the links they offered (and their own address
  queries edited).

## Left out

- Rename and Clear on Saved (Clear is a GET link; I did not press it).
- "Or count by" and `--count`, membership `all`/`removed`, the other
  realms, the Weighted Sum / Count stat groups.
- The CLI's `acq tabs --never-fetched` that the terminal offered in
  [call 4].
- Fetching Currency, Old 2, Maps (empty) on the rehearsal account, and
  setting a sync policy (`acq policy set`) to try "Plan under the
  policy".
- Any price command.

## Which answers I would not bet on

- Whether 103 or 132 is "the" answer to "rings with at least 70 to
  maximum life": both are right for their reading; the page never asked
  which I meant.
- Whether the rehearsal tabs being empty is the mock's design or a fault
  of the fetch: the door shows the mock returned `"items": []`, but I
  cannot see the mock's side.
- My reading that "2 clauses not run" is really one clause (`stats.1`
  and its own filter `stats.1.0`).
- Why the location lists were "58d" old in league=Standard answers and
  "61d" in others.

## Which claims rest on my own knowledge of the game rather than on the tool

- That a player would read "life" on a ring as total life including
  implicits (e.g. Coral Ring's implicit life).
- That the trade site's pseudo total life counts half of Strength (the
  tool's rows show Strength beside the total and the numbers add up that
  way; the tool never says it).
- That +248 maximum Life on a ring, and "Adds … Cold Damage to Attacks
  against you" on it, are no ring the game rolls.
- That a "crucible" stat group is the trade site's Crucible passive tree
  filter.
- That a player wants tab links, totals and prices (T8).

## Did I change the store in any way I did not undo

Yes, and I did not undo it:
- **Saved searches (the world's, both pages):** I saved "friend life
  70" (id `5bd947c89986`, revision 1). It is still there.
- **The rehearsal account `seat`'s store** (`store/mock/seat.db` by the
  page's words): the league list (job 2), the Rehearsal tab listing
  ([call 8], job 3, and again with each apply), fetches of Dump, Old 1,
  Uniques (apply, jobs 5–9) and Uniques, Maps (apply, jobs 11–14), all
  empty; the character list ([call 12], job 15). The daemon holds those
  15 jobs.
- **The collection `GERWARIC#7694`:** nothing written to its items; only
  read.

## Last message to the person

Every task was attempted (T0–T8); the searching works and its counts
agree across form, language and terminal (103 rings by explicit life,
every way). What you most need to know:

1. A saved trade search forgets it was partial. The friend's search ran
   honestly ("PARTIAL — 2 clauses not run", the crucible group), but
   once saved and reopened, every word of that is gone; 732 items look
   like the friend's answer.
2. The form can't ask for total life or resistance. The language can
   (`pseudo.total_life`: 132 rings vs the form's 103), but no page
   names the language's reference; I found it only through `acq search
   --describe`.
3. The refresh loop can't be done on the pages alone. Listing a
   league's tabs and planning under a policy both send you to the
   terminal; after Apply, the pages say "done" and "0 items" with no
   way to tell empty from failed (the door showed the mock's tabs were
   empty).
4. Dead ends: the item page has no links, "more" loses the table link
   and has no way back, a refused query leaves no form, "Fetch the
   league list" drops you on Jobs.
5. The cards load item art from web.poecdn.com.

I changed the world: one saved search ("friend life 70") and the
rehearsal store's league list, tab listing, five empty tab fetches and
character list; none undone. Outside the world I read only the
harness's saved copy of one door output and decoded the friend's URL
string myself.
