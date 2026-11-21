# 4-full-1, the search form — One page: a search's source, its editor filled, above its answer

A Fable design, 2026-11-21, for the road's first item
(`roadmap/README.md`, "Next", 1): the search form's reshaping, with the
owner's rulings of 2026-11-20 on #193 (the source is the row) and #296
(a place's line) built in, and the landing rule noted on #194. Nothing
here is built or ruled: §8 is drafts for the owner, and the build
briefs follow his word. Read-only at `84d71d21`; nothing was run.

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
`saved.rs` doc.

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

## 2. The form is the JSON's editor

- **The fill.** `search_form` takes the envelope (`{league, defences,
  query}`) and the scope. Each control reads its value at its path;
  the stat groups and rows are rendered from `query.stats`, a row's
  text and fate found by its id (a lookup by id beside `vocabulary`,
  in the search crate, C46, shared semantics in Rust). A panel that
  holds a value renders open. What the envelope holds and no control
  reads is named above the form by the site's own path, with one
  sentence saying a search from the form leaves it out: the fill never
  drops silently.
- **The fill is the fold's inverse, and that is its pin.** The form
  rendered from an envelope, read as a browser submits it, folds to
  that envelope: over every captured site search the trade tests hold
  and one envelope that fills every control.
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
  CSS over the type's select, not script. The finder is driven from
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
  JSON unchanged. "Clear" is a link to the page with its scope alone.
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
  as the four are. The form's Sockets row gains an "A" box, the
  envelope key `a` on that filter, read by the translation; the Link
  Groups row gains none, an abyssal socket being a group of its own.
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
  to the view. No script composes a value or a URL.
- **#296, as ruled.** With no sort asked, a place's items stand by
  position, top row first and left to right, then by id: one default
  order for every door, made where the corpus is loaded. A line
  carries its rarity in words from one row maker the CLI's row shares.
  The Stack column is the value `stack`, which the default list holds
  wherever a match carries a stack: its head sorts as any value's
  does, and its cell is empty on a gear row (§8 asks whether this
  reaches every door).
- **The language's index (#183's rest).** `GET describe` renders the
  `describe` document, the index and an entry by name, linked from the
  query box.

## 7. The build

In series on the shared checkout, each one brief from `TEMPLATE.md`,
a cold review per round, the page seat after the last and before the
owner sits.

1. **The source in the row** (§3): store, verbs, CLI, MCP, the save
   form and the opening. Consumer: an agent saving a site search.
2. **The editor, served** (§2 less its script): the positional fold,
   the fill and its inverse pin, the one page, the refused search.
   The stat rows render from the envelope; an empty form offers one
   group and typed rows as today until step 3.
3. **Rows and groups** (§2's script): the templates, the finder as
   the adder, remove, groups, weights, the keyboard; held by the
   browser-driven check (C187, a script's rule), which retires the
   pick-fills-the-first-empty-row case by name.
4. **What rides** (§4 less the sockets): the quality box, the switch,
   the fates, the site box in the bar, Clear.
5. **The landing rule** (§5).
6. **The value finder** (§6).
7. **A place's line** (#296).
8. **Abyssal sockets** (#185), then **the index** (#183's rest).

Closes #180, #181, #183, #184, #185, #188, #190, #193, #296. #194
stays open for its rest.

## 8. Drafts for the owner

Each is his; nothing is entered until he rules. The two registry
lines pass `python3 -I tools/bothy.py check` on a copy of the registry
holding them.

- **The shape**: §1's sentence.
- **C<next> — A saved search keeps what was asked, and the text
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
  sentence naming the kind reads "a saved search (C<next>, the
  row keeps its source)".
- **C101, one clause** (a change to what a user types): "Socket
  colours, and abyssal sockets beside them, are asked over the whole
  item and within one link group."
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
key `a`; the finder's value ask.

## 9. What not to do

- Fill, add or order anything in a script: a script copies the
  server's template and sends the server's fields (C46).
- A second envelope for the form: the site's JSON is the form's, and
  what the site lacks stands beside or inside it by a named key.
- Storing the translated text beside the source: declined on #193.
- An option list on a stat row: the stat table holds none.
- The look (#285), the jobs page's words (#182), the other league's
  never-fetched tabs (#194), a saved view, live results as he types,
  a remembered switch: each has its issue or its "Not doing" line.
