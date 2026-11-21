# 4-full-1, the search form — One page: a search's source, its editor filled, above its answer

A Fable design, 2026-11-21, for the road's first item
(`roadmap/README.md`, "Next", 1): the search form's reshaping, with the
owner's rulings of 2026-11-20 on #193 (the source is the row) and #296
(a place's line) built in, and the landing rule noted on #194; and,
at his ask of 2026-11-21, a search's way back to the trade site (§7).
Ruled by him on 2026-11-21 ("Approve"), the four choices of §9 as
recommended; the registry lines are entered and nothing is built. The
build briefs follow. Read-only at `84d71d21`; nothing was run. Cut
the same day under the cold review at `22337d17`
(`roadmap/reviews/4-full-1-form-design-brief-22337d17.md`): the foot
says what each finding changed.

What was read: the road and `decisions/process.md`; issues #180, #181,
#183, #184, #185, #188, #190, #193, #194, #296, #243, #244, #285, #182
with their comments; the sitting of 2026-11-10 and the page seat after
step 6; `4-full-1-design.md` and `4-full-1-step6-design.md`; the
frontends, store, search and trade rulings named below; `web/form.rs`
whole, `web/mod.rs`'s doc, `search`, `asked_by_the_form`, `Paged`, the
POST handlers and `to_job`; `web/page.rs`'s `search_form`, `query_box`,
`trade_box`, `save_forms`; the two form scripts; the search crate's
`QueryInput`, `trade.rs`, `trade/vocabulary.rs`, the translation's key
lists, `show.rs`'s `trade_value`; the store's `user.rs`; the verbs'
`saved.rs` doc; for §7, the trade area's "Not doing" lines,
`SURFACES.md`'s trade rows and `tools/trade-sheet.py`'s composer.

His words that set the reference: "my muscle memory and search skills
are highly adapted to the trade site"; and of a saved search, "load a
saved search and modify it slight, then run or save it again."

## 1. The one shape

**A search is its source, and its page is that source's editor,
filled, above its answer.** A request's query already has the
spellings (`answer.rs`, `QueryInput`): a text in the language, or a
site search, which is a URL, an id, or the site's query JSON with the
league and the defences' reading beside it (C113, the third spelling;
C141 and C142, what stands beside the JSON). The form's fields fold
into that JSON today (`form.rs`, `fold`). So the form is that JSON's
editor and the query box is the text's. What is missing is the fold's
inverse, a form rendered from the JSON, and every ask below is that
one function met from another side.

| What was asked | What it is in this shape |
| --- | --- |
| Tweak a bound and search again, as on the site | The answer stands under the form that made it, still filled |
| Add and remove stat rows and groups; a weight per row (#184) | The form renders as many groups and rows as its JSON holds; adding one copies a template the server wrote |
| A saved search refills the form (#193, his ask) | The row keeps its source; a source the form can hold fills it |
| A saved site search says what its translation lost (#193) | The source is translated at every read, so the report is the answer's own |
| Open a site search in the form (#188) | A site search decodes to the same JSON; running it fills the form |
| A refused search keeps its input (#190) | The page is the editor filled from what was sent, the refusal where the answer would stand |
| A typed stat id has no fate (#183) | No id is typed: a row is a pick or the fill's, and both carry its text and fate |
| Sort by a value a player names (the seat, fault 2) | The stat finder, asked for values, beside the rows' order |
| Run a search on the trade site (his ask, 2026-11-21) | The site's JSON the search holds, written as the site's id; nothing is translated |

## 2. The form is the JSON's editor

- **The fill.** `search_form` takes the envelope (`{league, defences,
  query}`) and the scope. Each control reads its value at its path;
  the stat groups and rows are rendered from `query.stats`, a row's
  text and fate found by its id (a lookup by id beside `vocabulary`,
  in the search crate, C46, shared semantics in Rust). A panel that
  holds a value renders open. What the envelope holds and no control
  reads is named above the form by the site's own path, with one
  sentence saying a search from the form leaves it out: the fill never
  drops silently. A control that is the listing's shows the value it
  was filled with and sends nothing, as today, and is named the same
  way.
- **The fill parts an envelope in two, and that is its pin.** What
  the form holds, read as a browser submits it, folds back to it
  exactly; what the form names as left out (the listing's fields, and
  each path no control reads) is all of the rest. Both are compared in
  the fold's own normal form, empties and a `disabled` of false
  dropped. Held over every captured site search the trade tests hold,
  the translation's cases of a key it does not read among them, and
  one envelope that fills every control.
- **Stat fields are positional, as `view.value` is.** `group.type`
  begins a group and `stat.id` a row in it; `group.min`, `group.max`
  and `group.disabled` belong to the group last begun, and `stat.min`,
  `stat.max`, `stat.weight`, `stat.option` and `stat.disabled` to the
  row last begun. No name carries an index: the names the fold accepts
  stay a fixed set, a script that adds a row names nothing, and a
  removed row leaves no hole. A field before the thing it belongs to
  is a usage error. `view.value` already rests on a URL's fields
  keeping their order; the build's first command finds every maker of
  a link or a hidden field, and a test holds a two-group search
  through every link of its page. *Declined:* indexed paths
  (`query.stats[1].filters[2]…`), where the script counts and the fold
  bounds an index.
- **Rows and groups as the site's.** Each group ends with its finder
  ("Add Stat Filter"): a pick adds a row to that group, copied from a
  `<template>` the server rendered, carrying the entry's id, text, kind
  and fate. A row holds its on/off box, its text, min, max, a weight
  where its group's type sums, an option where its filter holds one,
  and a remove button. After the last group, "Add Stat Group" offers
  the types. What a type shows (a count's bound, a row's weight) is
  CSS over the type's select, not script. CSS hides and the browser
  still sends, so the fold drops a field its group's type does not
  take, asked of the translation's own key lists (`translate.rs`,
  `stat_value_keys`) and never restated in the page; on a filled form
  such a field is a path no control reads. A weighted group changed to
  `and` is one case of the browser-driven check. The finder is driven from
  the keyboard as the site's: arrows move, Enter picks, and Enter
  there never submits. A page loaded from its URL shows every row
  with no script; adding one needs the script.
- **One page.** `.` and `search` render one page: the bar, the form,
  the boxes, then the answer or the refusal at an anchor each search
  form's action names. The source says which editor is filled. The
  form's fields or a site search fill the form, a site search through
  its decode, and the query box holds the canonical text, closed. A
  text fills the query box, open, over an empty form. A continuation
  is read by its token's query the same way. A refused search is the
  same page with the editor filled from what was sent, its status and
  JSON unchanged: from the envelope where the fold or the decode gave
  one; the site box holding its text as sent where the decode refused
  it; the refusal over an empty form where the fold refused the
  fields, since such a URL was never the form's. "Clear" is a link to
  the page with its scope alone.
- **A site search is one box and one button**, in the bar: it runs as
  given, labelled with its report as today, and the form stands filled
  from it. Searching from the form then asks the form's fields. No
  second "open without running" button: the count beside Search
  already answers before a run.

## 3. The saved row is its source (#193, as ruled)

- **The row is a name over a request's scope and query**: `{version: 2,
  name, scope, query}`, `query` holding exactly one of `text`, stored
  canonical as today, or `trade`, the site search as given: a URL or
  an id as a string, the query JSON as an object, which is what the
  form's fields are. A JSON string is stored as its object, so the
  form's envelope and a pasted JSON are one spelling. A version 1 row
  reads as `query.text` and no read rewrites it (C66, intent values
  are typed).
- **The text is derived at every read.** `request_of` answers the
  row's scope and query as a request; the list's row keeps `text`, now
  the text the source reads as in this build, beside `query` and, for
  a site search, its translation's fidelity. A source this build
  cannot translate is listed with its fault and refused when run
  (C151, an unreadable row is named).
- **The door checks the source as it checks a text**: read in the
  row's realm before any file is opened; a URL's realm is the row's.
- **Every surface saves a source**: `acq saved set` takes `--trade` and
  `--trade-file` as `acq search` does; the MCP's `saved_search_set`
  takes `trade` beside `text`, its first turn measured against
  `surface_size.rs`; a page's save form carries the answer's source,
  `text` or `trade`. Opening a row redirects to `search?text=` or
  `search?trade=`, which §2 fills.

## 4. What rides on the form

- **The quality box (#180)** reads "assume 20% quality" and starts
  ticked; unticked, the envelope carries `defences: "displayed"`. C142
  stands as written: the default was and is the site's reading, and
  only the box's sense and words change. An unticked box sends
  nothing, so a hidden `defences=displayed` stands before the box's
  own `defences=q20` and the fold reads the last sent.
- **The switch (#181)** stands at the page's foot. Fields that are
  not run or are the listing's are hidden unless it is ticked; a
  field that holds a value always shows. It is a page field in the
  URL, read apart from the fold as `saved` is, and CSS does the
  hiding.
- **A fate is said where it is not exact.** A control that runs as the
  site's says nothing; one read differently, or not run, keeps its
  mark and its title. One line by the switch says so.
- **Abyssal sockets (#185).** The language gains a fifth socket word,
  `abyssal`, GGG's `A`, over the item and within a link group, counted
  as the four are. The form's Sockets row gains an "A" box, which the
  envelope carries as a count beside its query, `abyssal`, read by the
  translation (§7, nothing of ours inside the query); the Link Groups
  row gains none, an abyssal socket being a group of its own.
  The site's own `min` and `max` are untouched: what they count of an
  abyssal socket is a sitting's question.

## 5. Leaving a page: the landing rule (#194's comment, #190's rest)

**A request that leaves its page carries that page, and what answers
it leads back to the page holding what was sent.** One maker writes
`from`, the page's own route and fields, into every form whose action
is another route (each POST, and the place form's plan); one reader
takes it only as a route of this server. Then:

- a submit lands on its job with the way back made from `from`, the
  league list's included, and `to_job` decides nothing for itself;
- a refused write renders its refusal, the form as sent (a save's
  empty name, its source kept) and the way back;
- a refused plan (nothing ticked, no policy, no session) renders its
  refusal and the way back to the place page with every tick and open.

A refusal's words stay the shared ones (C143, a text speaks the
request's words). The build lists each refusal these forms can meet,
by the command its brief gives; each either has a page's way on or
names its command as a person's act at a terminal.

## 6. Naming a value, and a place's line

- **The value finder.** Wherever a page says its rows' order, "Add a
  column" stands beside it: the stat finder asked for values, listing
  stats and the form's number fields by the site's names. An entry
  carries the value text that reads it, made in the search crate as
  `show.rs`'s `trade_value` makes a line's. A pick sends the page's
  form with one field naming the entry, answered `303` to the page
  ordered by that value through `view_link`, the one maker of a change
  to the view. No script composes a value or a URL. It lists an
  entry only where the search crate makes a value for it today
  (`trade_value` answers one, or the field is a number's); no
  translation is widened for it.
- **#296, as ruled.** With no sort asked, a place's items stand by
  position, top row first and left to right, then by id: one default
  order for every door, made where the corpus is loaded. A line
  carries its rarity in words from one row maker the CLI's row shares.
  The Stack column is the value `stack`, which the default list holds
  wherever a match carries a stack: its head sorts as any value's
  does, and its cell is empty on a gear row (§9 asks whether this
  reaches every door).
- **The language's index (#183's rest).** `GET describe` renders the
  `describe` document, the index and an entry by name, linked from the
  query box.

## 7. A search goes back to the site

His choice among three ways, 2026-11-21 ("A, the ask goes back"): what
goes to the site is what was asked, the site's JSON a search holds as
its source, written as the site's id. Nothing is translated, so the
language, the answer and the fold gain no case.

- **The rule that makes it an encoding**: nothing of Acquisition's
  stands inside a site search's query object. What the site's form
  lacks stands beside it, as the league and the defences' reading do
  (C141, C142), and §4's abyssal count joins them.
- **The link.** One function in the search crate writes the id from
  the query object as held and the URL from the realm and the league:
  the decode's rule (`trade.rs`) run backwards. Its pin is the decode:
  a composed link decodes to its query, its realm and its league. The
  tool sends nothing and a person opens the link (C79, governed
  surfaces).
- **Where none is made**, each said in one sentence: a search typed in
  the language, whose translation outward stays declined; a search
  over no one league, or over every realm.
- **What stands beside the link**: each thing beside the query that
  the site cannot be asked (defences as the item stands, an abyssal
  count); and that the link carries what was asked, so a field not run
  here runs there.
- **One place.** The trade report of an answer whose source is a site
  search carries the link: the page shows it beside the answer, the
  CLI prints it and an agent can hand it on. It is the search that
  ran, never a form's unsent edits.
- **The listing's fields stay as they are** (§2): a site search opened
  and run as given goes back whole, and one edited in the form leaves
  them out and says so. Making them editable is a question about what
  the form is for, not taken here.
- **A sitting first** (the site-sitting skill): a few links from the
  Rust composer, one with no `status`, opened by the owner. The
  sitting tool's composer reproduces the site's ids
  (`tools/trade-sheet.py --self-test`); whether the site takes another
  encoder's bytes, or a query with no status, is not known. The step
  is built on the sitting's record. The link is the object as held, so
  nothing is added to it: if the site refuses a query with no status,
  the step stops and the question is the owner's (C179, what reaches
  him).

## 8. The build

In series on the shared checkout, each one brief from `TEMPLATE.md`,
a cold review per round, the page seat after the last and before the
owner sits. This document is the design each brief names as its
input; a brief carries its own find-first commands, and its done-when
starts from its step's "seen when" here.

1. **The source in the row** (§3): store, verbs, CLI, MCP, the save
   form and the opening. Consumer: an agent saving a site search.
   *Seen when:* a site search saved on each surface opens with its
   report, and a version 1 row still runs.
2. **The editor, served** (§2 less its script): the positional fold,
   the fill and its inverse pin, the one page, the refused search.
   The stat rows render from the envelope; an empty form offers one
   group and typed rows as today until step 3. *Seen when:* a form
   search's answer, a refused one, a site search's and a saved form
   search's each stand under the form filled with it.
3. **Rows and groups** (§2's script): the templates, the finder as
   the adder, remove, groups, weights, the keyboard; held by the
   browser-driven check (C187, a script's rule), which retires the
   pick-fills-the-first-empty-row case by name. *Seen when:* a search
   of two groups, one weighted, is made, edited and run from the page
   with no id typed.
4. **What rides** (§4 less the sockets): the quality box, the switch,
   the fates, the site box in the bar, Clear. *Seen when:* each of
   the five is in the form's golden, filled and empty.
5. **The landing rule** (§5). *Seen when:* each submit, and each
   refused write or plan, leads back to its page holding what was
   sent.
6. **The value finder** (§6). *Seen when:* a tab opened with no query
   is ordered by a value picked by name.
7. **A place's line** (#296). *Seen when:* a currency tab reads in its
   own order with its stacks and rarities, on its page and at `acq
   search`.
8. **Abyssal sockets** (#185), then **the index** (#183's rest).
   *Seen when:* the word answers in the language and from the form's
   box, and the query box links the index.
9. **The link to the site** (§7), after its sitting; last, so it can
   wait or fall away without touching a step before it. *Seen when:*
   a form search's answer carries a link that decodes to that search.

Closes #180, #181, #183, #184, #185, #188, #190, #193, #296. #194
stays open for its rest.

## 9. Drafts for the owner

Each was his. Ruled 2026-11-21: "Approve", of the design as it stood
at `c8d6cea6`, with the four choices below as recommended and, of the
three ways to §7, "A, the ask goes back". Entered as drafted: C194 and
C167's sentence (`decisions/store.md`), C195 and the "Not doing" line
(`decisions/trade.md`), C101's clause (`decisions/search.md`) and the
surfaces row. The way back's step and its sitting are #303.

As drafted:

- **The shape**: §1's sentence.
- **C<next>, now C194 — A saved search keeps what was asked, and the text
  it runs is derived at every read.** Drafted: "The row holds a name,
  a scope and the query as given: a text in the language, or a site
  search as a request spells one (C113, the third spelling), the
  search form's fields among them as the site's query JSON. The text a
  row runs is made when the row is read, so an opened row carries its
  translation's report, and a build with other tables may read a site
  search differently and says so. *Why:* a row that stored the
  translation dropped its source, so an opened search could not say
  what the site's search had lost and a form could not reopen what it
  made. *Pinned:* nothing yet; Unbuilt, #193." With it, C167's
  sentence naming the kind reads "a saved search (C194, the
  row keeps its source)".
- **C101, one clause** (a change to what a user types): "Socket
  colours, and abyssal sockets beside them, are asked over the whole
  item and within one link group."
- **C<next+1>, now C195 — A search held as the site's JSON goes back to the
  site as it was asked.** Drafted: "Nothing of Acquisition's stands
  inside a site search's query object: what the site's form lacks
  stands beside it (C141, the league beside the JSON). A search whose
  source is the site's JSON is written as a site link from that
  object, its realm and its league, and each thing beside it that the
  site cannot be asked is named as left out. Acquisition sends
  nothing, and a person opens the link (C79, governed surfaces). A
  query in the language has no link. *Why:* the link is then an
  encoding of what was asked, and no second translation taxes the
  language. *Pinned:* nothing yet." With it, the trade area's "Not
  doing" line reads "The outbound translation of a query in the
  language, and PoE2 on the trade site: not until a user needs a
  search they typed or an agent wrote sent to the site, or PoE2 trade
  is in use."; and `SURFACES.md`'s row for the site's data endpoints
  permits a link the tool writes for a user's own search, opened by
  that user.
- **The route** (C138, the listener): `describe`, GET, joins. Nothing
  else is a new route: the finder's two asks are one route's field.
- **Four choices with a second defensible answer**:
  1. The switch's default. Recommended: hidden unless ticked, so what
     the form shows runs. The other: shown, as the site lays them out.
  2. The Stack column's reach. Recommended: the value `stack` in the
     default list on every door, so `acq search` and the MCP show a
     currency tab's counts too. The other: the pages alone, which
     needs a column that is no value.
  3. A site search's one button (§2), in place of #188's two.
  4. Fates said only where not exact (§4).

Agent-facing design, the session's, for his review: the row's `query`
and each surface's `trade`; the positional names; `from`; the envelope
key `abyssal`; the finder's value ask; where the link stands in the
trade report.

## 10. What not to do

- Fill, add or order anything in a script: a script copies the
  server's template and sends the server's fields (C46).
- A second envelope for the form: the site's JSON is the form's, and
  what the site lacks stands beside it by a named key (§7).
- Translating a query outward, a second "open on the site" button on
  the form, or a link for unsent edits (§7).
- Storing the translated text beside the source: declined on #193.
- An option list on a stat row: the stat table holds none.
- The look (#285), the jobs page's words (#182), the other league's
  never-fetched tabs (#194), a saved view, live results as he types,
  a remembered switch: each has its issue or its "Not doing" line.

## Taken from the review (`22337d17`, cut 2026-11-21)

1 the status fallback is gone: the link is the object as held, and a
site that refuses one without a status is the owner's question (§7); 2
the fill's pin is a partition in the fold's normal form, the named
paths all of the rest (§2); 3 the fold drops a field its group's type
does not take, by the translation's own key lists (§2); 4 a refused
search fills from the nearest thing read, and a fold's refusal stands
over an empty form (§2); 5 each step says what is seen when it is
done, and its brief carries the commands (§8); 6 the finder lists only
what has a value today (§6). Not taken: 7, the size and the reading
list. This is the design nine briefs name as their input, as the two
designs before it are, and each brief is held to the template's
bound.
