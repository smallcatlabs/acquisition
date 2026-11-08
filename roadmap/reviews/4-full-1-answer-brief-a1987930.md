Review of `roadmap/briefs/4-full-1-answer.md` (cut 3) at commit `a1987930`, read cold against the tree as it stands (no code has moved since `35cf33a7`; `git log 35cf33a7..a1987930` touches only the brief and its reviews).

What checks out. These claims match the tree:
- The eleven `acq …` lines in `search_pages.txt`, `saved_pages.txt:64`, and `moved_world.txt:19`, `:45`, `:121` and `:123`.
- The eight `GET search?` pages in `saved_pages.txt`.
- Every `web.rs` line the Input grep cites: 419-421, 633-634, 655, 660-661, 680-682, 714-715, 742, 746, 757-760, 766/768, 781-786, 962 and 977.
- The serde shapes behind every pointer in the inventory:
  - `Removed` flattens a `Route` and has no `count`.
  - `Resolved.rest` is an unflattened `Route`.
  - `ViewOut` is externally tagged.
  - `Next` holds `request`.
- `Request::from_token`, `place()` being private, and `basis` and `Rows.after` being public.
- `Paged`'s arms, the `serves` check in `asked_by_the_form`, and `Way::Search(Box<Request>)` at two sites (`refusal.rs:375`, `:589`).
- `tabs_pointer`, and `TabsPointer.realm` being a `String`.
- `shed_routes`.
- `trade_text`'s cut at ten.
- The two `other =` catalogue rows.
- `bound::LISTED` = 10.
- The C116 sweep, and `normaliser.rs`'s sweep over every golden.
- The `basis_rule` and `token_rule` masks (an `acq1.` token in a link is masked, since `enc` keeps `-`, `_` and `.`).
- `acq web`'s refresh route working offline.
- CI run 42490674447: `success` on `35cf33a7`.

The findings below are what would still mislead a worker, most severe first.

1. **Input, "The facts the tests stand on" (row 10's recipe): the rings as described cannot stake, so row 10 would have no way.**
   - *The brief's summary drops a line.* It says to stage "two Iron Rings, one with "25% increased Rarity of Items found", one with "30% increased …" and "10% reduced …"". The query JSON it quotes has two filters, and the second, `explicit.stat_1030153674`, is the Mana on Kill stat.
   - *The recipe it cites has that line.* In `seat_faults.rs:3167-3182` every ring also carries `"Recover 2% of Mana on Kill"` (`lines.push(...)` at `:3169`). Each ring is `"name": ""`, `"typeLine"`/`"baseType": "Iron Ring"`, `"frameTypeId": 2`, with ids `plain` and `both`. The golden's query is `sum(line("#% increased Rarity …").arg1)>=20 line(("Recover #% of Mana on Kill" or "Lose …") source=explicit)` (`goldens/seat_faults/an_ambiguous_clause_adds_and_drops.txt:12`).
   - *What happens if a worker follows the summary.* Rings without the mana line fail the second conjunct, so the answer is a zero. Both stake counts are 0 and carry no route (`Router::count` gives no route at 0, `answer.rs:2519-2525`). Row 10 then has no `<a class="way">`, Do 8's "a recorded page per inventory row … 10" fails, and the worker reports row 10 unreachable.
   - *Should say:* each ring also carries "Recover 2% of Mana on Kill", as `seat_faults.rs:3167-3182` stages them. Or say "copy that test's two items whole" and drop the paraphrase.

2. **Input, "The facts the tests stand on", and Do 7: the "three pages it gains" include one already recorded, and Do 7 adds a fourth.**
   - *The xbox page is not new.* Input lists the pages `every_search_page_is_a_wording_golden` "gains": rows left out, a routed bucket, and "the zero answer (the xbox URL page: two explain offers, the corpus offer, rows 12 and 13)". The xbox URL page is already recorded (`web.rs:675-694`, golden `search_pages.txt:164` "GET search?<an xbox URL from the pc page>", its offers at `:185-188`).
   - *Do 7 compounds it.* It speaks of "the three pages Input adds to `every_search_page_is_a_wording_golden`" and then "gains the zero answer with an undecided (row 11) beside Input's three". A worker will either record the xbox page a second time or hunt for a third new page that does not exist.
   - *Should say:* the test gains three pages: the rows left out (row 1), the routed bucket (row 7) and the zero answer with an undecided (row 11). The xbox page, already recorded, is where rows 12 and 13 are checked. Do 7's "the three pages Input adds" should be read the same way, with no separate "beside Input's three".

3. **Do 6: the corrected `page.rs` module doc as worded would be false for the plan and jobs pages.**
   - *The wording.* Do 6 asks the module doc to say "the pages render their documents as HTML, the words made once with the text's, and print no command line".
   - *What that doc covers.* `page.rs`'s module doc (lines 1-11) is every page's: it names "the plan, the jobs table" beside the answer and `show`.
   - *Those pages still print the CLI's text after this step.* `mod.rs` renders them through `page::pre` of the CLI's text: the refresh page at `:1105`, `:1112` and `:1131`, the plan at `:1346`, the apply at `:1398`, the jobs at `:1435` and `:1443`. Their command lines are #182's (`refresh_loop.txt`, `jobs_halted.txt`, `console_realm.txt`).
   - *Should say:* the answer and item pages render their documents as HTML and print no command line. The plan, refresh and jobs pages still print the CLI's text until #182. This is the same split Do 6 already asks of `mod.rs`'s bullet.

4. **Input, "The tests the step changes", line 742: the brief says `!counted.body.contains("<table>")` "fails once a count renders an HTML table". It cannot fail.**
   - *Why it passes now.* The rows table today is `<table class="results">` (`page.rs:776`), which does not contain the literal `<table>`. The assertion already passes on a page that shows rows.
   - *Why it would still pass.* Do 4 gives each count table "a class of its own", so a page with them still holds no bare `<table>`.
   - *The rewrite is still right, for the opposite reason.* The check is vacuous today and would stay vacuous.
   - *Should say:* the assertion is vacuous (no table on any page is a bare `<table>`), and it is rewritten to its property: no `class="results"` on the counted page, and the count's own class present.

5. **Input / Do 7: the staged never-fetched tabs tie, so "the pointer names the league holding the most" is false as staged.**
   - *How the order is decided.* `never_fetched_leagues` is sorted by count, then by name (`acquisition-store/src/coverage.rs`, `by_league.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)))`).
   - *What the staging gives.* One never-fetched tab in `Standard` and one in `Hardcore` is a tie. The pointer then names `Hardcore` by name, and counts `Standard` as the other league.
   - *Two consequences.*
     - The page with no league term shows "1 other league holds never-fetched tabs" for the reason Input does not give.
     - The second page's `-league=…` must exclude `Hardcore` to exercise "the pointer names the other". Excluding `Standard` changes nothing.
   - *Should say one of two things.*
     - Stage two never-fetched tabs in `Standard`, so it does hold the most, and the second page excludes `-league=Standard`.
     - Or keep one each, say the tie goes to name order (the pointer names `Hardcore`), and make the second page `-league=Hardcore`.

6. **Do 8, the tabs way: the stated reading of "where `tabs_pointer()` would be some" is not `tabs_pointer`'s rule.**
   - *The brief's reading.* "The JSON's scope under one realm holding a never-fetched tab in a league the query does not exclude."
   - *What `tabs_pointer` does.* Where the query names a league (`league=` fixed or or-ed), it points only among the named leagues (`answer.rs:2980-3007`, `!named || among(l, false)`).
   - *Where they diverge.* A query `league=Standard` over a scope whose only never-fetched tab is in `Hardcore` has no pointer, but the brief's reading expects a tabs way. None of the brief's pages meets this:
     - the seeded world has no never-fetched location (`search_pages.txt:59`, "0 locations never fetched");
     - the staged pages name no league or exclude one.

     A worker building the predicate to the letter writes a wrong one that the goldens do not catch.
   - *Should say:* present where `scope.realm` is one realm and `scope.never_fetched_leagues` holds a league that the query does not exclude, and, where the query names leagues, one of those named.

7. **Input, "The tests the step changes": three of the four line numbers the continuation fix names are off by one.**
   - The brief says "lines 1813, 1816, 2399 and 2402 drop their prefix".
   - The lines that carry it are 1813 (`follow(&web, &at(&more))`), 1817 (`json_at(&web, &at(&more))`), 2400 and 2403 (`format!("/{token}/{more}")`). `grep -n 'at(&more)\|"/{token}/{more}"' crates/acquisition-cli/tests/web.rs` finds them.
   - *Should say:* 1813, 1817, 2400 and 2403.

8. **Input, the cut list (row 5): the conditions for `rest` are stated incompletely.**
   - *What the brief says.* A value list carries `rest` "only for a term `rest_of` keys, a field test or a line group with one template".
   - *What is missing.* `resolved` exists only where `resolves(term)` holds (`answer.rs:2692-2716`):
     - a group that names a template;
     - a field test with `:` or `~` (`Op::Contains | Op::Match`), never a text or `id` atom.

     `rest_of` also excludes `tab` and `category` (`:2722-2752`).
   - *Why it matters.* A worker who stages eleven distinct values and writes an `=` term, or a `tab:`/`category` term, gets no value list or no `rest`.
   - *Should say:* the term is a `:` or `~` selector on a field other than `tab`, `category`, text and `id` (for example `name:` or `base:` over eleven distinct values), or a line group whose selector is one template test.

9. **Do 4, the zero answer: the block's own sentence is not listed.**
   - The text prints each selector nothing carries with "— {note, or "it left the scope, as its term says" for an id, or `zero.said`}" (`search_cmd.rs:1397-1412`). `said` is `S107` or `SUGGESTED_SAID` (`answer.rs:331-336`).
   - Do 4 lists "each selector nothing carries with its suggestions", but not `said` or `note`.
   - The staged removed-id page (`id:` of an item that left) is a zero answer whose only words there are "it left the scope, as its term says".
   - *Should say:* each selector nothing carries, with its note or the zero's `said` (or the id's "it left the scope" words), then its suggestions.

10. **Do 6: one more doc becomes false and is not named.**
    - *The doc.* `row_shows`'s doc says "the touched lines the web UI's table carries in a cell" (`search_cmd.rs:1222-1225`).
    - *Why it changes.* Do 2 splits `row_shows`'s "more" lines and its "N more sorted". If the page stops calling `row_shows`, or calls a split form, that sentence is false or partly false.
    - *Should say:* add `row_shows`'s doc to Do 6's list, corrected to whatever the split leaves.

11. **Do 5, the price line: "the price with its side and source" names a field `Shown` does not have.**
    - The text shows `p.text`, `p.side`, and `from` only where `from` is not the item's own target (`search_cmd.rs:2205-2214`, `.filter(|from| *from != target)`).
    - A worker looking for a "source" on `Priced::Is` finds `from`.
    - *Should say:* the price, its side, and the row it is priced from where that is not the item itself (`from`). For a removed item (`Priced::None` with kind `removed`), the text prints no why line already.

12. **Minor places a worker searches.**
    - **Where the help sentence lives.** Do 6 says `acq web`'s help "renders what these verbs print" changes, but not where that text is: it is the clap doc comment at `crates/acquisition-cli/src/main.rs:241`. It appears twice in `CLI-REFERENCE.md` (lines 14 and 106), both regenerated.
    - **Which `TEMPLATE.md`.** Read names `TEMPLATE.md`; the file is `roadmap/briefs/TEMPLATE.md`, and the root has none.
    - **Rules that do not say what they bind.** The template asks a brief to say whether each rule binds the session's work or the thing built (`roadmap/briefs/TEMPLATE.md:107-109`). The Rules' last two bullets ("A choice that would add a route …", "A sentence of this brief the tree disproves …") say neither.
