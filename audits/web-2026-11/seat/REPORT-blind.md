# Report — page seat, blind run (`acq web` through the browser pane)

Runner: Claude (Opus 5.5), 2026-11-08. Addresses used, from the two `.out`
files: collection `http://127.0.0.1:50978/d74c6fa2dbddc2a23265b741b0bda3f1/`,
rehearsal `http://127.0.0.1:50979/10dad76ba5c764938a11bd2faf895e59/`.
Pane tools are cited as `get_page_text`, `read_page`, `find`, `js`
(the JavaScript tool), `screenshot`. Door calls as `[call N]`.

## Before touching anything

Read: `BRIEF.md` whole, and `README.md` from "## The tour" to the next
`##` (extracted with one `awk` over that range, nothing else of the file
printed). Opened the collection address, then followed its four nav
links (Search, Saved, Refresh, Jobs) to read each page, touching no
control on any of them.

What the pages offer (seen with `get_page_text`, `read_page`
interactive, and `js` over `document.forms` and `[title]` tooltips):

- **Search** (`/`): a form shaped like the trade site's: a free-text box
  ("Search items"), Realm (pc, xbox, sony, poe2) with a "Show this realm's
  leagues" button, Status (Instant Buyout and In Person / … / Any) marked
  "listing only", Membership (live, all, removed), and the trade site's
  collapsible filter groups (Type, Weapon, Armour, Socket, Requirements,
  Map/Chart, Heist, Sanctum, Ultimatum, Miscellaneous, Trade, Stat
  Filters with Group = And/Not/If/Count/Weighted Sum…, four stat rows and
  "Find a stat"). Then Show (20/50/100/200), "Sort by … highest first",
  "Or count by" ("keys as the CLI's count takes them: rarity, league,
  tab, class, category, line:resist,life"), a Search button, and a second
  form: a textarea "The query, in the language" with "Run this query".
  Last, a POST form "Fetch the league list". Many controls carry
  tooltips (read with `js`), e.g. "The free-text box matches a whole
  name or a whole base, as the site's does, never a word of one." and
  "Online, available or securable is the seller's state, not the
  item's, so it means nothing in your stash." Some options are suffixed
  "(exact)" or "(unsupported)" and some fields marked "exact" — I took
  these to say how faithfully the tool honours that trade-site filter.
- **Saved** (`/saved`): "No saved searches: save one from any search's
  answer, under its query."
- **Refresh** (`/refresh`): a realm picker, "Fetch the league list", a
  summary "3347 tabs in Standard: 405 fetched, 2926 never fetched, 16
  folders (never fetched); 20231 items in tabs (characters apart)" and
  "facts: 3331 tabs, 405 fetched (the oldest fetch 60d ago, pc ·
  Standard / Keepers (Remove-only); the newest 57d ago, pc · Standard /
  Dump Tab); stash listing 57d ago", buttons "Refresh selected" and
  "Plan under the policy", then a table "Stash tabs (3347)" (Tab, Type,
  Items, Fetched; map/unique tabs show "N pages of …"; folders shown as
  "Gems · Folder · 9 tabs") and "Characters (41)" (Character, Level,
  Items, Fetched). This is the collection's account, which the brief says
  has no session, but the page does not say so up front.
- **Jobs** (`/jobs`): "rails:  tripwire off · sends 2 since this daemon
  started · journal …/logs/2c938a0a23d8/mock/sends.jsonl", one row
  (`1 profile done 254 seat daemon finished 16m ago`) and "1 job this
  daemon holds (its own and the restored): 0 running, 0 waiting, 1 done".
  Note: this is the *collection* server's Jobs page, and the one job
  shown is the `seat` account's — the daemon is shared.

What I would do first: for T1, set Item Category = Ring, Item Rarity =
Rare, and add a stat filter for maximum life with min 70, then Search.

## The tasks

### T0 — Orient

Done as written above under "Before touching anything". Expected a
search page and some way to look after the account; got four pages
(Search, Saved, Refresh, Jobs) linked from a nav bar that `get_page_text`
does not show (it reads `<main>` only; the links came from `read_page`).
Surprise: the collection's own Refresh page lists all 3347 tabs and 41
characters with fetch ages and a "Refresh selected" button, and nothing
on it says this account has no session (the brief says so; the page
does not). The Jobs page on the collection server shows the `seat`
account's job.

### T1 — Rings for life

What I did (all `computer` clicks/typing and `form_input`, results read
with `get_page_text` and `js`):

1. Item Category = Ring, Item Rarity = Rare (`form_input`).
2. Typed "maximum Life" into "Find a stat". First try: no suggestions
   appeared at all (`screenshot`). Typing "life" a moment later gave a
   list; typing "maximum Life" again then worked. I cannot tell whether
   the first miss was the picker still loading; I note it, I do not
   count it as a fault.
3. The picker's list for "to maximum Life" (`js`) offers
   "+# to maximum Life explicit · alone exact · with a bound exact",
   the same line "implicit" and "fractured", and nothing else of that
   stat: **no crafted variant, no pseudo/total life**. "pseudo life" and
   "crafted" found nothing; "total" found only unrelated lines.
4. Picked the explicit one: the stat row filled with the raw id
   `explicit.stat_3299347043` and a small caption "+# to maximum Life".
   Typed 70 in its min, pressed Search.

The answer (`get_page_text`), quoted:

> 103 matches
>
> trade   from the query JSON: exact — every clause with a stash meaning ran as the site reads it
> query   category="accessory.ring" rarity=rare sum(line("# to maximum Life" source=explicit).arg1)>=70
> scope   account GERWARIC#7694 · pc · live · 22623 items · 546 locations fetched
>         2940 locations never fetched: MapStash 2489 of 2582, UniqueStash 437 of 521, character 14 of 59 …
> total   103 matches (the oldest fetch 61d ago, …; the newest 57d ago, pc · Standard / Dump Tab)
>             2940 locations were never fetched: nothing here says what they hold
> rows 1–20 of 103 shown · more
> terms   … 2    sum(line("# to maximum Life" source=explicit).arg1)>=70  762 matched · 21861 failed

Each row shows the item name and base, "rare", the matched lines (e.g.
"+78 to maximum Life (explicit)") and its place
("Standard / TheFireworksCompany (equipment) · fetched 60d ago", or a tab
with "x 2, y 9"). Below the answer the page fills "The query, in the
language" with the query it ran, and offers "Name  Save this search".

What did not work: I tried to count implicit and fractured life too,
with Group = "Weighted Sum (exact)", group min 70, and the three "+# to
maximum Life" rows (explicit, implicit, fractured) with no row bounds.
The answer was **730 matches**, headed:

> not the site's search: 4 clauses not run
> trade   from the query JSON: PARTIAL — 4 clauses not run, so this is not the site's search
>         NOT RUN   stats.0    unsupported whole: its filter `stats.0.0` has no weight

The form has **no weight field anywhere** (`js`: no input whose name or
placeholder mentions weight; each row has only stat/min/max), so
"Weighted Sum (exact)" is offered but cannot be completed from the page.
The page was honest that it dropped the clause, which is good; but it
still printed a 730-row answer to a question I did not ask.

**T1's answer: 103 rare rings with at least +70 explicit life**, from
the page's own count. Not what I meant by "at least 70 to maximum life"
for a player: life from an implicit (e.g. a Coral Ring's) or a crafted
mod is not added in, and the form gave me no way to add it.
Expected: a "total maximum Life" (pseudo) stat as the trade site has.

### T2 — The same, typed

How I learned the words: from the page itself. The T1 answer page
fills "The query, in the language" with the query the form became:
`category="accessory.ring" rarity=rare sum(line("# to maximum Life" source=explicit).arg1)>=70`.
I dropped `source=explicit` to take every source. The door was not
available to me at this point (see "Where I left the page"), so I never
read `--describe`.

Sorting is not a word of the language. I guessed `sort:life` at the end
of the typed query and pressed "Run this query"; the page (HTTP 400,
per the tab's note; text by `get_page_text`) said only:

> `sort` is no field the language knows — describe `fields` lists all 74; near it:
>     str:life

That page held nothing else: no query box to correct, no link to
"describe" (which is a command-line word; the page has no describe
view). The language's own form (textarea + "Run this query") has no
sort, limit or desc fields (`js` over `document.forms`); only the big
trade-style form has "Sort by" (`view.sort`, placeholder "ilvl"),
"highest first" (`view.desc`) and "Show" (`view.limit`). So I added
those three to the address bar query of the typed query's own answer
URL (the brief allows editing a page's own query):
`search?realm=pc&text=<query>&view.sort=sum(line("# to maximum Life").arg1)&view.desc=on&view.limit=200`.

Typed query: `category="accessory.ring" rarity=rare sum(line("# to maximum Life").arg1)>=70`

Answer (`get_page_text` + `js` over the rows):

> 111 matches
> query   category="accessory.ring" rarity=rare sum(line("# to maximum Life").arg1)>=70
> terms … 2    sum(line("# to maximum Life").arg1)>=70  797 matched · 21826 failed

Each row then carried "sorts by sum(line("# to maximum Life").arg1) N".
All 111 rows on one page, N non-increasing from 248 down to 70 (`js`
read every row's N in order: 248, 142, 136, 133, 128, 114, 113, 113, …,
70). The sort did what was asked.

Count vs T1: **111, not 103**. The difference is the source: the
typed query sums every "+# to maximum Life" line, which the rows show
tagged "(explicit)", "(implicit)", "(explicit, crafted)",
"(explicit, fractured)" and "(scourge)". I counted from the row list
the rings whose explicit line is under 70 but whose sum is 70+: Spirit
Whorl Coral (59+28 implicit), Pandemonium Finger Coral (56+30), Spirit
Eye Coral (57+29), Gloom Spiral Coral (64+21), Gale Hold Coral (53
crafted + 30 implicit), Rift Band Two-Stone (57 + 24 scourge), Fate
Whorl Coral (59+21), Tempest Turn Coral (67+24) — 8, and 103 + 8 = 111.
So the two answers agree once you know the form's picker only sums one
source. **The typed answer (111) is the one a player means.**

What the sort shows that I would not bet on: the top row is "Damnation
Whorl Two-Stone Ring … +248 to maximum Life (explicit)", then single
explicit lines of 136, 133, 128, 114 … From my own knowledge of the game
a single ring life mod does not roll near 248; I take these to be
several life mods the game shows merged into one line, but the page
does not say so (looked at in T5).

Also seen: the typed query's answer shows 10 rows by default, the
form's answer 20.

**Later, once the door worked** (the person added `W/bin` after my
first pass; my first door attempt had failed, see "Where I left the
page"): `search --describe` `[call 1]` printed the language's index. It
has a block "computed values — totals v6 … the trade site's pseudo
stats", with `pseudo.total_life  total  the trade site's pseudo_total_life
(`+# total maximum Life`)`. So the language **does** have the total I
looked for in T1 — the page's stat picker just never offers it, and no
page lists the language's words.

`[call 2]` refused: "2 accounts are known and one is to be picked:
GERWARIC#7694, seat"; `[call 3]` refused: "this store holds 2 realms and
a search names one in its scope: pc | poe2, or all". Both offered the
exact fix. `[call 4]`:
`acq --account GERWARIC#7694 search --realm pc --sort=pseudo.total_life --desc --limit=15 'category="accessory.ring" rarity=rare pseudo.total_life>=70'`

> total   132 matches …
> rows    Damnation Whorl Two-Stone Ring … sorts by pseudo.total_life 248 …
>         Honour Hold Sapphire Ring … sorts by pseudo.total_life 140 · +54 to Strength (explicit) · +113 to maximum Life (explicit, fractured) …
>         Victory Grip Breach Ring … sorts by pseudo.total_life 117.5 · +39 to Strength (explicit) · +98 to maximum Life (explicit) …

So the trade site's total counts half of Strength as life (113 + 54/2 =
140), which is why it gives **132**, not 111. The same text typed on the
page (with `view.sort=pseudo.total_life` in the address) also answered
"132 matches", top rows identical (`get_page_text`). Three counts for
"rare rings with at least 70 life", each right for its question: **103**
(explicit only, the form), **111** (every life line summed, my typed
query), **132** (the trade site's total life, Strength included). The
sort is descending in every case.

### T3 — Keep it

All by `computer` clicks/typing, read by `get_page_text`, `read_page`
and `js`.

1. **Save.** On T2's sorted answer page I typed the name "Rare rings 70+
   life" in "Name" and pressed "Save this search". `js` showed beforehand
   that the save form posts only `text`, `realm`, `membership`, `name`:
   **no sort, no desc, no limit**. I landed on the Saved page, which said
   "1 saved search · the file at revision 1" and showed the row
   (Name, Scope "pc · live", Query, Revision 1, "0s ago via web", Id
   `0f3f4efab470`, a "the new name" box with "Rename", and "Clear").
   No line said "saved"; the new row is the only sign.
2. **Find and open.** The name is a link to `saved/open?id=0f3f4efab470`
   (`read_page`). (My first click by `ref` landed beside it and did
   nothing; a click by coordinate worked — the pane, not the tool.)
   It opened the answer page headed:
   > Saved search "Rare rings 70+ life", opened at revision 1 · the saved searches
   >
   > 111 matches
   > query   category="accessory.ring" rarity=rare sum(line("# to maximum Life").arg1)>=70
   Same query, same count: **what opened is what was saved — but not
   what I had on screen when I saved it.** The sort (most life first)
   and the 200-row view were gone: it opened unsorted, "rows 1–10 of 111
   shown". The page never told me the sort would not be kept. Under the
   opened answer there are now two save forms: "Save over "Rare rings
   70+ life"" and "Save as a new search".
3. **Rename.** On the Saved page I replaced the text in the row's "the
   new name" box (pre-filled with the old name) with "Life rings (total
   70+)" and pressed Return. The page came back with "1 saved search ·
   the file at revision 2" and the row now named "Life rings (total
   70+)", its Revision 2. No message said "renamed from …".
4. **Second search, then clear.** Typed `category="accessory.amulet"
   rarity=rare` (441 matches) and pressed "Save this search" with the
   name empty, to see: a bare page (HTTP 400 per the tab's note) saying
   only `a name is one character at least once trimmed: ""` — no form
   to try again, I had to go back. Then saved it as "Rare amulets (to
   clear)": "2 saved searches · the file at revision 3", Id
   `1dd1a2bb977e`. Its "Clear" link opened a confirm page:
   > Clear the saved search "Rare amulets (to clear)"? It is this store's saved search, whoever saved it: clearing it takes it off the list for every page, agent and terminal that reads this store.
   with "Clear" and "Keep it". "Clear" returned to the list: "1 saved
   search · the file at revision 4", the amulet row gone.

What I would have wanted told: after each step, one line saying what
happened ("Saved "Rare rings 70+ life" (id 0f3f4efab470)", "Renamed
"…" to "…"", "Cleared "…" — undo"); the tour says `acq saved set` and
`clear` have undo, but no page offered an undo after the clear. At save
time: that the sort and row count are not part of a saved search. The
confirm-before-clear page was good and said clearly who else it affects.

### T4 — The terminal and the page

My first door attempt (during T2) failed before running `acq`:
"seat: /Users/tom/seat-worlds/web-2026-11/page-opus/bin is not a
directory — it is the directory of the binaries". The person later told
me `W/bin` had been missing (the brief's fault) and now exists. T4 was
done after that, door calls 5–9.

- `[call 5]` `acq saved list`:
  > 1 saved search (the file at revision 4):
  >   Life rings (total 70+)  pc live      category="accessory.ring" rarity=rare sum(line("# to maximum Life").arg1)>=70  (revision 2)  0f3f4efab470
  > next: `acq search --saved ID` runs one; `acq saved show ID` shows one
  Agrees with the page row for row: name, scope, query, revision, id,
  file revision 4.
- `[call 6]` `acq search --saved 0f3f4efab470 --limit 5`, exactly as the
  "next:" line said, **refused**: "2 accounts are known and one is to be
  picked: GERWARIC#7694, seat" (it offered both full commands). The
  hint it had just printed does not run on this store.
- `[call 7]` `acq --account GERWARIC#7694 search --limit=5 --saved='Life
  rings (total 70+)'` (by name): "total   111 matches", the same query,
  the same first rows as the page. Agrees with the page's 111.
- `[call 8]` `acq saved show 0f3f4efab470`: "revision 2, written 5m ago
  via web, created 6m ago", and "next: … `acq saved clear 0f3f4efab470
  --if-revision 2` clears it".
- `[call 9]` saved one from the door: `acq saved set --name 'Rings by
  total life (door)' --realm pc 'category="accessory.ring" rarity=rare
  pseudo.total_life>=70'`:
  > saved search 37d1e493550a: Rings by total life (door) over pc live: … (revision 1), was unset
  > next: `acq search --saved 37d1e493550a` runs it; `acq saved clear 37d1e493550a --if-revision 1` undoes it
  The collection's Saved page (`get_page_text`) then showed "2 saved
  searches · the file at revision 5" and the row "Rings by total life
  (door) · pc · live · … · 1 · 4s ago via cli · 37d1e493550a".

Before the door worked I had also opened the page-saved search on the
rehearsal server: same row there; it ran over the `seat` account and
answered "0 matches … scope   account seat · pc · live · 0 items",
saying plainly whose items it ran over.

**Agree:** the list, names, ids, queries, scopes, revisions and the
111; a search saved on either side shows on the other at once, marked
"via web" / "via cli".
**Disagree:** the door tells you what it did and how to undo it ("was
unset", "… undoes it"); the page says nothing after a save and offers no
undo. The page picks the account by which server you are on; the door
needs `--account` whenever two accounts are known, and its own "next:"
hint leaves it out. `saved list` at the door has no "Last written / via"
column; `saved show` does. Neither side can save a sort.

### T5 — One item

From T2's sorted answer I opened the top row, "Damnation Whorl Two-Stone
Ring" (the "+248 to maximum Life" one), by its name link
(`item/8e265a30…`). (Again a click by `ref` did nothing and a click by
coordinate worked: the pane.) The page (`get_page_text`, `screenshot`):

> Damnation Whorl Two-Stone Ring · rare · frame rare
> id      8e265a30168be9e9742313e2bfb8265e607ac2a2c6c51a761dd457e6a85e5401
> place   pc · Standard / Items 4 (Remove-only) · x 8, y 6 · fetched 60d ago
> base    Two-Stone Ring
> class   Rings
> category accessory.ring
> price   none (nothing applies) · why: acq price show --account cac319d8-e65f-4afa-92be-1de85d620033 --realm pc item/8e265a30…
> is      duplicated, identified
> shown   Item Level: 83
>         Requires Level 35
> lines   explicit   Adds 19 to 39 Cold Damage to Attacks against you
>                    "Adds # to # Cold Damage to Attacks against you"   low 19 · high 39 · avg 29 · arg1 19 · arg2 39
>         explicit   -41 to Evasion Rating          "# to Evasion Rating"   arg1 -41
>         explicit   +248 to maximum Life           "# to maximum Life"   arg1 248
>         explicit   +17% to Cold Resistance        "#% to Cold Resistance"   arg1 17
>         implicit   +14% to Cold and Lightning Resistances
> unread  nothing

plus a small item picture and a "basis" line. The only links on the
page are the four nav links (`read_page` interactive).

What it tells me: name, base, rarity, where it is down to the cell,
how old that knowledge is, item level, level requirement, every line
with its source and the numbers the search reads (`arg1`, `low/high/avg`),
that nothing of the body went unread, and that it is "duplicated".
So the 248 is one explicit line as the store holds it — not a merge by
the tool.

What I wanted and did not get:
- **Why** this ring has +248 life and "Adds … Cold Damage to Attacks
  against you" / "-41 to Evasion Rating". From my own knowledge these
  look like a reflected (mirrored, Kalandra) item's mods; the page's
  word is "duplicated", which I read as the API's flag for a mirrored
  copy, but it does not say "mirrored" and does not explain the odd
  values.
- A way back to the answer I came from, or to the other items in
  "Items 4", or to the tab on the Refresh page.
- Stat requirements (Str/Dex/Int), sockets/influence/corruption if any
  (I cannot tell "none" from "not shown"), mod tiers or prefix/suffix.
- Something to take the item elsewhere: the item text as the game
  copies it (for Path of Building or pasting), or a trade-site search
  for similar items.
- The price line points at a command line with an account **uuid**
  (`cac319d8-…`), not the account name the rest of the page uses
  (`GERWARIC#7694`), and does not say on the page what "nothing
  applies" means.
- A larger picture; the art is a small icon.

### T6 — The refresh loop (rehearsal account)

All in a second pane tab on the rehearsal address; read with
`get_page_text`, `read_page`, `js`.

1. **What the account holds.** Refresh page, first view:
   > no tabs known for Standard in /Users/tom/seat-worlds/web-2026-11/page-opus/store/mock/seat.db (run `acq stashes --account seat --realm pc --league Standard` or `acq refresh --all --account seat --realm pc --league Standard`)
   > facts: no tabs; stash never listed
   > no characters known in Standard (run `acq characters` or cover them in the sync policy)
   Only a Realm picker; no League picker; the page assumed Standard.
2. **"Fetch the league list"** (a POST button). It took me to the Jobs
   page, which showed `2  leagues  running  0  seat  web:84980  started
   0s ago`; four seconds later, without my reloading, the same page read
   `done … finished 3s ago` (the page updates itself). No link from the
   job to what it fetched.
3. Back on Refresh, a **League** picker had appeared: "Rehearsal (no
   tabs on disk)", "Standard (no tabs on disk)", "Hardcore (no tabs on
   disk)". Chose Rehearsal, "Show": the same three messages as step 1
   with "Rehearsal" in place of "Standard". **No page control lists the
   league's stash tabs or characters**; the page sends me to
   `acq stashes …`, `acq refresh --all …`, `acq characters`.
4. **"Plan under the policy"** → a bare Plan page (HTTP 400 per the
   tab's note):
   > no sync policy is set for seat — declare one first, e.g. `acq policy set --account seat --if-revision 0 '{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":"all","characters":"all","max_age_seconds":3600}}}}}'`
   No page can set a policy.
5. **"Refresh selected"** with nothing to tick → bare Plan page (400):
   > nothing is selected: tick a tab or a character to refresh it
   There is nothing to tick until the tabs are listed, and listing them
   is not offered on the page.

Here the pages left me no way on, which is when the brief sends me to
the door. On my first pass the door did not run (see T4); I finished T6
once it did (steps 6–11 below).

6. **Door, to list what the page could not.** `[call 10]` `acq --account
   seat stashes --realm pc --league Rehearsal` (the page's own
   suggestion): JSON with six top-level entries — Currency, Dump, a
   Folder "Old leagues" holding Old 1 and Old 2, Uniques, Maps, "Maps
   (empty)" — and on stderr "job 3 (seat): waiting / job 3 (seat): done".
   `[call 11]` `acq --account seat characters --realm pc`: two
   characters, StashHoarder (Scion 97) and MuleQuadTab (Witch 12),
   **both in league Standard**. So the Rehearsal league has no
   characters to choose; T6's "characters of the Rehearsal league" is
   none.
7. **Back on the Refresh page** (Rehearsal): "8 tabs in Rehearsal: 0
   fetched, 7 never fetched, 1 folder (never fetched); 0 items in tabs",
   "facts: 7 tabs, none fetched; stash listing 7s ago", and a table of
   the tabs with ticks, all "never". It still said "no characters known
   in Rehearsal (run `acq characters` or cover them in the sync
   policy)" — after I had run exactly that; the characters are in
   another league, and the line does not say so.
8. **Choose and plan.** Ticked Currency, Dump, Old 1, Uniques and "the
   substashes box: also fetch a ticked map or unique tab's pages", then
   "Refresh selected". The Plan page:
   > refresh Rehearsal on mock as seat — a selection, with its map and unique tabs' pages
   > the listing is fetched beside the selection: what it newly lists takes the next plan
   > pages of "Uniques": none to fetch on record — pages not known yet arrive with its fetch; apply, then select it again for them
   > 5 requests:
   >   list stashes         the selection is read from it
   >   fetch 4 tabs         4 selected
   >     rh-cur1  Currency  selected … rh-uniq  Uniques  selected
   > quote (0s ago):
   >   stash-list-request-limit  1 request — ready
   >   stash                     4 requests — ETA unavailable (unlearned)
   > wire: 5–15 sends; plus first-contact probes and token refresh if needed
   > Apply
   I knew what would happen: five requests, named.
9. **Apply.** It took me to the Jobs page, which showed (`get_page_text`)
   "7  apply  plan (5 requests)  running", children 8 (`stashes
   Rehearsal`, done) and 9–12 (`stash Rehearsal/rh-cur1` … `rh-uniq`,
   waiting, eta "~0s"), and a daemon "probe stash@seat"; five seconds
   later, unreloaded, all done, "sends 15". Nothing on the Plan or Jobs
   page said "applied: 4 tabs, N items".
10. **What landed.** Refresh page: "8 tabs in Rehearsal: 4 fetched, 3
    never fetched, 1 folder (never fetched); 0 items in tabs",
    "facts: 7 tabs, 4 fetched (fetched 10s ago, pc · Rehearsal /
    Currency)", the four rows "0 · 10s ago". A search `true()` on the
    rehearsal pages: "0 matches … account seat · pc · live · 0 items · 4
    locations fetched · 5 locations never fetched". Zero items looked
    like a fault, so I checked: `[call 12]` `acq --account seat jobs
    result 10` was refused ("unexpected argument 'result'" — I misread
    the tour's `acq jobs … | result <id>` line as a sub-verb of `jobs`);
    `[call 13]` `acq --account seat result 10` showed the mock answered
    Dump with `"items": []`. So 0 is true: the mock's tabs are empty.
11. **The Uniques pages.** As the plan said, I selected Uniques again
    with the substashes box. The new plan said **the same thing word
    for word**: "pages of "Uniques": none to fetch on record — pages not
    known yet arrive with its fetch; apply, then select it again for
    them". `[call 14]` `acq --account seat result 12` showed the first
    fetch had `"substashes_listed": 0`: the tab has no pages. The plan
    cannot tell "no pages" from "pages not yet known", so following its
    advice would loop forever. I did not apply the second plan.

Did I know what had happened / would happen next? On the pages alone, after "Fetch the
league list" I did not know where the result would show until the
League picker appeared on Refresh; nothing said "3 leagues fetched".
After "Plan under the policy" I knew exactly what was wrong (a policy
is missing) and exactly what to type — in a terminal, not on the page.
From a page user's seat the loop is a dead end at its first step on a
fresh account: the page needs the stash listed, and only the command
line lists it.
With the door: the plan page told me exactly what would be sent;
after Apply I watched it happen on Jobs; but "what landed" took three
pages and two door calls to establish (0 items true, Uniques has no
pages), and the Uniques advice was wrong.

Aside, on the collection's pages (to see the plan page the rehearsal
account never reached): on its Refresh page I ticked "Dump Tab" and
pressed "Refresh selected". The Plan page (`get_page_text`):

> refresh Standard on mock as GERWARIC#7694 — a selection
> facts: stash listed 57d before the plan; plan made 0s ago
> the listing is fetched beside the selection: what it newly lists takes the next plan
> 2 requests:
>   list stashes            the selection is read from it
>   fetch 1 tab             1 selected
>     5ba2e1880a  Dump Tab  selected
> quote: unavailable — no session for "GERWARIC#7694" — run `acq auth`
> wire: 2–6 sends; plus first-contact probes and token refresh if needed
> Apply

It already knew there was no session but still offered "Apply";
pressing it gave a bare Apply page (HTTP 400):
> account "GERWARIC#7694" is not logged in (live: seat); log in as it first
Clear, and nothing ran (no new job). But "log in as it first" has no
page to do it on, and the Refresh page had said nothing of the missing
session before I made the plan.

### T7 — The jobs

The Jobs page (nav link "Jobs", on either server; read with
`get_page_text` and `js` over `main`). It is one preformatted block:

> rails:  tripwire off · sends 4 since this daemon started · journal /Users/tom/seat-worlds/web-2026-11/page-opus/logs/2c938a0a23d8/mock/sends.jsonl
> id  parent  kind     target  state  prio  account  by         when              eta
>  1          profile          done    254  seat     daemon     finished 24m ago
>  2          leagues          done      0  seat     web:84980  finished 66s ago
> 2 jobs this daemon holds (its own and the restored): 0 running, 0 waiting, 2 done

During T6 I saw job 2 as `running … started 0s ago` with "sends 3",
then, without reloading, `done … finished 3s ago` with "sends 4": the
page updates itself.

What it tells a person: that the daemon is working or idle, each job's
kind, state, which account it is for, who asked (`daemon`, or
`web:84980` — I take the number to be the asking page server's process
id; the page does not say), when, and a count of "sends" (requests to
the API, I assume) with the path of a journal of them.

What it does not: what a job fetched or produced (no link from a job
to its result, to the leagues it found, or to the Refresh page); why a
job failed, if one did (none did here, so I could not see a failure);
what "rails", "tripwire", "prio", "eta" and "the restored" mean; which
server's page the job came from in words. Both servers show the same
daemon's jobs, for both accounts, so on the collection's pages a person
sees the rehearsal account's jobs mixed in (account column helps). There
are no controls: no cancel, no priority, no filter. (Written on my
first pass, before the plan ran; what the page showed during the apply
follows.)

After the apply (T6 step 9) the page showed the jobs' tree through a
`parent` column (7 → 8–12), the target of each (`Rehearsal/rh-cur1`),
`eta  ~0s` while waiting, daemon "probe" jobs (`stash-list@seat`,
`stash@seat`, prio 255) nobody asked for, door jobs marked `cli:25060`
beside page jobs `web:84980`, and at the end "3 earlier finished jobs
not listed (`acq jobs --all`)" — the older jobs drop off the page and
only the door can show them. What it still does not tell: how many
items each fetch brought (I needed `acq result <id>` at the door to
learn the tab was empty), and what "probe" is for.

### T8 — As a player

Three things a player would want from these pages that I could not do,
or could not find how to do:

1. **Ask for totals the way the trade site lets you**: "total maximum
   Life", "total elemental resistance", "total attributes". The form's
   stat picker has no total/pseudo stats and no crafted source, and its
   Weighted Sum has no weight box (T1). The language can sum sources
   (T2), but only if you already know to drop `source=`.
2. **Get a fresh league's stash onto the pages without a terminal.**
   On a new account the Refresh page can neither list the tabs nor set
   a sync policy, so nothing can be ticked or planned (T6). A player
   who starts a new league would be stuck on day one.
3. **Do something with an item once found**: price it or list it for
   sale (the tour has `acq price set` and `acq shop render`; the item
   page shows "price none (nothing applies)" and a command line), copy
   it as game text for Path of Building, or jump to the tab it sits in.
   From the item page the only way on is the nav bar (T5).

(Also wanted, smaller: see a stash tab as the game's grid; save a
search with its sort.)



## Faults, worst first

0. **(The world's, not the tool's) The door in the brief did not run at
   first.** `W/bin` was missing; seat.py said "…/bin is not a directory
   — it is the directory of the binaries" before any `acq` ran. The
   person added it partway through; T4, T6's door steps and `--describe`
   were done afterwards (calls 1–14).
1. **A fresh account's refresh loop dead-ends on the pages.** Did: Fetch
   the league list, chose Rehearsal, Show, then "Plan under the policy"
   and "Refresh selected". Happened: "no tabs known for Rehearsal … (run
   `acq stashes …`)", "no sync policy is set for seat — declare one
   first, e.g. `acq policy set …`", "nothing is selected: tick a tab or
   a character". Only `acq stashes` at the door `[call 10]` unblocked it.
   Should have: a button to list the league's stash and characters, or a
   way to set a policy. Task T6.
2. **The form's stat picker hides the language's own totals.** Did:
   searched "Find a stat" for life, pseudo, total, crafted. Happened:
   only explicit / implicit / fractured "+# to maximum Life"; the form
   answer (103, explicit only) is labelled "exact". The language has
   `pseudo.total_life` (`--describe`, `[call 1]`), which gives 132 on
   the page and at the door — but no page offers or lists it. Should
   have: the pseudo stats in the picker. Tasks T1, T2.
3. **The Uniques plan advises a loop.** Did: planned Uniques with the
   substashes box, applied, planned it again as told. Happened: the same
   line both times, "pages of "Uniques": none to fetch on record — pages
   not known yet arrive with its fetch; apply, then select it again for
   them", though the first fetch had `"substashes_listed": 0`
   `[call 14]`. Should have: "Uniques has no pages". Task T6.
4. **"Weighted Sum (exact)" is offered with no weight field.** Did:
   Group = Weighted Sum, min 70, three life rows. Happened: "PARTIAL —
   4 clauses not run … `stats.0.0` has no weight", and a 730-row answer.
   Should have: a weight box per row, or not offer the group. Task T1.
5. **Saving drops the sort and row count, silently.** Did: saved a
   sorted, 200-row answer. Happened: it opens unsorted, 10 rows; the
   save form carries no `view.*` fields; `acq saved set` has no sort
   either. Should have: kept them, or said they are not kept. Task T3.
6. **The typed query cannot be sorted from the page.** Did: looked for
   sort on the language form; tried `sort:life`. Happened: "`sort` is no
   field the language knows"; the language form has no sort/limit; I had
   to add `view.sort`/`view.desc` to the address by hand. Task T2.
7. **Error pages are dead ends.** Bare HTTP 400 pages with one line and
   no form to fix the input (bad query, empty save name, missing policy,
   nothing selected, not logged in), often pointing at command-line words
   ("describe `fields`", `acq policy set`, `acq auth`) the pages do not
   offer. Tasks T2, T3, T6.
8. **What a fetch brought is not shown.** After Apply, neither the Jobs
   page nor Refresh says "4 tabs, 0 items fetched"; "0" in the Items
   column looked like a failure until `acq result 10` `[call 13]` showed
   the mock sent `"items": []`. Tasks T6, T7.
9. **No confirmation or undo on save, rename, clear on the page.** Each
   just returns the list with a new revision number; the door's `saved
   set` says "was unset" and how to undo it `[call 9]`. Tasks T3, T4.
10. **The door's own "next:" hint does not run here.** `acq saved list`
    says "`acq search --saved ID` runs one"; run as written it refuses
    "2 accounts are known and one is to be picked" `[call 6]`. Task T4.
11. **"no characters known in Rehearsal (run `acq characters` …)"**
    stays after `acq characters` ran: both characters are in Standard,
    and the line does not say so. Task T6.
12. **The collection's Refresh page hides that it has no session.** It
    builds a plan with an "Apply" button, which then refuses: "account
    "GERWARIC#7694" is not logged in". Task T6 aside / T0.
13. **Item page: no way on, and an account uuid where a name is used
    everywhere else** ("why: acq price show --account cac319d8-…").
    Task T5.

## What the tour and the pages did not say

- That the form's stat picker never offers the language's pseudo totals
  (`pseudo.total_life` …) or a crafted source, and that the form, a
  summed `line(…)` and the pseudo total give three different counts
  (103 / 111 / 132) (T1/T2).
- That the language's words are listed only at the door (`--describe`);
  no page lists them.
- That sorting is not part of the language, and that a saved search
  keeps no sort (T2/T3).
- How to list a new league's stash from the pages — you can't (T6). The
  tour's `acq web` line promises "the refresh loop to tick, plan and
  apply", but ticking needs a listing only the door makes.
- That the rehearsal account's characters are all in Standard (T6).
- That the collection account has no session, until Apply refused (T6).
- What "duplicated" means on an item (T5).
- That the Jobs page shows every account's jobs on every server, and
  drops older ones to "`acq jobs --all`" (T7).
- That the Refresh page's League picker only appears after "Fetch the
  league list" (T6).
- The tour's `acq jobs … | status <id> | result <id>` line: I read
  `result` as a sub-verb of `jobs` and was refused `[call 12]`; it is
  `acq result <id>` `[call 13]`.

## The tool's own words

Words or phrases I did not understand, quoted:

- "alone exact · with a bound exact" / "with a bound ambiguous" (stat
  picker, T1).
- "trade   from the query JSON: exact — every clause with a stash
  meaning ran as the site reads it" (T1).
- "over the items the store has fetched, priced or not — what it never
  fetched is the scope's: the site shows public listings, a tab's being
  tab.public, so the search is the site's and the items are yours" (T1).
- "routes  each returns exactly the members counted (8 more: --routes)"
  (every answer).
- "basis   store 8876083d3e5e · build e807ff4a08f7 · snapshot 1126 ·
  facts v9 · … derivation 12 · … totals v6 …" (every answer).
- "1 lacked" / "8254 lacked" / "20055 lacked" in the terms table.
- "frame rare" and "is      duplicated, identified" (T5).
- "unread  nothing" (T5).
- "rails:  tripwire off · sends 15", "prio", "by web:84980",
  "cli:25060", "probe stash@seat", "(its own and the restored)" (T7).
- "quote: unavailable", "ETA unavailable (unlearned)", "wire: 5–15
  sends; plus first-contact probes" (plan page, T6).
- "the listing is fetched beside the selection: what it newly lists
  takes the next plan" (plan page, T6).
- "8 tabs in Rehearsal … 1 folder" beside "facts: 7 tabs" — two counts
  of the same tabs on one page (T6).
- "the file at revision 4" (Saved; the file is never named).
- "listing only" and "exact" markers beside form controls (T0).

## What I would ask of the tool

- A "List this league's tabs and characters" button on Refresh, and a
  way to start a sync policy there (or to plan without one).
- The pseudo totals in the form's stat picker, crafted included, weight
  boxes for Weighted Sum, and a page listing the language's words.
- Sort, desc and limit on the language form, and kept in saved searches.
- Error pages that keep the form and the input so one can fix and retry.
- A one-line result after save/rename/clear, with undo, as the door has.
- After Apply: "fetched 4 tabs, 0 items" in words; and "Uniques has no
  pages" rather than "select it again".
- Say "no session — refresh cannot run" on Refresh before a plan is made.
- On the item page: a link back to the answer and to the tab, the game's
  copy text, and plain words for "duplicated".
- On Jobs: a link from each job to its result or error.
- At the door: "next:" hints that run on the store they were printed on.

## Where I left the page for the terminal, and why

- T2, to learn the language's words: the pages have no list of them.
  First attempt failed (no `W/bin`); later `[call 1]` `search
  --describe`, then `[calls 2–4]` the total-life search.
- T4, by design: `[calls 5–9]`.
- T6, because the Refresh page could not list the league's stash or
  characters and named the commands to do it: `[calls 10–11]`. Then to
  learn whether "0 items" was true and why the Uniques plan repeated:
  `[calls 12–14]`.

## What I read or ran outside the world

Nothing outside `W`. Inside `W`: I read `BRIEF.md` whole and the "The
tour" section of `README.md` (one `awk` that printed only that range),
read the two `.out` address files, ran `ls -la W` once to see why
`W/bin` failed (a listing, no file opened), and ran the door: one
failed attempt before `W/bin` existed, then calls 1–14, each as the
brief writes it. Long door output (`[call 1]`, 33 KB) was saved by my
harness to a file outside the world, which I read back with `grep`/`sed`
— the door's own output, not another file. Pane: only the two
addresses, links from their pages, and edits to a page's own
address-bar query (adding `view.sort`, `view.desc`, `view.limit`;
changing `text=` to `true()` and to the `pseudo.total_life` query;
going straight to the rehearsal `refresh`, `saved/open` and `jobs`
links). I used the JavaScript tool to read forms, hidden fields and
tooltips, and once to click a stat button (T1). I did not read the
notes folder the starting session named, since the brief forbids
other files. The person told me, by a message from the session that
started me, that `W/bin` now existed; nothing else changed.

## Left out

- In T6: re-applying the Uniques plan (its advice would loop), and any
  sync policy (`acq policy set`): "Plan under the policy" was never
  tried with one. No characters in Rehearsal to fetch.
- I did not try every filter group or the "Or count by" box, nor
  "Show this realm's leagues" on Search, nor xbox/sony/poe2 realms.
- I opened one item only.

## Which answers I would not bet on

- T1 = 103, T2 = 111 and 132 are the tool's own counts, and I believe
  them as counts of what the store holds; whether "explicit" in the
  form's picker includes crafted lines I inferred from row tags
  ("explicit, crafted"), not from anything the page said.
- My 103 + 8 = 111 reconciliation is mine, by reading the 111 rows; I
  did not run the difference as a query. That 132 adds half of Strength
  I read from three rows' numbers, not from a definition.
- That `web:84980` / `cli:25060` are process ids, and "sends" are API
  requests: my reading of the words.

## Which claims rest on my own knowledge of the game rather than on the tool

- That a player asking "at least 70 to maximum life" means the total
  across implicit, explicit and crafted lines, and that the trade site
  offers a pseudo "total maximum Life" stat. (The tool's `--describe`
  later confirmed it carries `pseudo_total_life`.)
- That a single ring life mod does not roll near +248, and that the
  "Damnation Whorl Two-Stone Ring"'s odd mods look like a reflected
  (Kalandra-mirrored) item; "duplicated" being the API's mirrored flag.
- That Coral Rings carry an implicit life line, and that Strength gives
  life (1 per 2).
- That a new-league player would want to fetch their stash on day one.

## Did I change the store in any way I did not undo

Yes, deliberately and within the brief's leave:
- Saved searches (the world's): two remain — "Life rings (total 70+)"
  (`0f3f4efab470`, revision 2, from the page) and "Rings by total life
  (door)" (`37d1e493550a`, revision 1, from the door). "Rare amulets (to
  clear)" was saved and cleared. The saved-search file is at revision 5.
  The failed empty-name save changed nothing.
- Rehearsal account `seat`, at the mock: the league list (job 2), the
  Rehearsal stash listing (job 3) and character list (job 5), and the
  applied plan (jobs 7–12) fetching Currency, Dump, Old 1 and Uniques
  (0 items each). A second Uniques plan was made but not applied.
- Collection account: one plan made for "Dump Tab" and an Apply that was
  refused before anything ran.
No policies or prices were written; nothing on the collection's items.
