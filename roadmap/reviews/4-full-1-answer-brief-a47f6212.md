Review of `roadmap/briefs/4-full-1-answer.md` (cut 2) at commit `a47f6212`, read cold against the tree as it stands.

What checks out. The eleven `acq …` lines in `search_pages.txt` (75, 111, 113, 185, 186, 188, 194, 234, 236, 306, 319) are where the brief says. So are `saved_pages.txt:64` and `moved_world.txt:19`, `:45`, `:121`, `:123`, and the four lines at `7a72de85`. The brief's grep finds every `web.rs` line it cites: 419-421, 633-634, 655, 660-661, 680-682, 714-715, 742, 746, 757-760, 766/768 and 781-786. The continuation lines 1812-1817 and 2395-2403 match. Other confirmed facts:
- `saved_pages.txt` holds eight `GET search?` pages.
- `grep -rn "Way::Search" crates/acquisition-verbs/src` finds two sites (`refusal.rs:375`, `:589`).
- CI runs 42490674447 and 42490674446 are both `success` on `35cf33a7`, which is `origin/main`.
- The `Answer` pointers in the inventory match the serde shapes: `ViewOut` is externally tagged snake_case; `Count` is flattened into `Bucket`, `OfKind`, `Cell`, `Together` and `Removed`; `Total.matched` is a bare `usize`.

The findings below are what would still stop or mislead a worker, most severe first.

1. **Do 8, "Every way, where it stands": as worded, the test cannot be built as a separate test, and its follow checks fail on the tabs link.**
   - *Wrong: the pages cannot be "served again".* The test runs "over every answer page the two golden tests of Do 7 record … served again". A golden records labels, not URLs: `GET search?<the ring search>`, `GET search?<an xbox URL from the pc page>` (`search_pages.txt:41`, `:164`). The staged world's pages exist only after its stagings, done inline and in order (Input; `web.rs:847-920` is the model). A separate test can reach them only by repeating every staging inline, or by factoring the staging into a function. Such a function is close to the "new seed function" #142's rule and Done-when forbid.
   - *Wrong: the tabs link breaks the follow checks.* The test says "For each `<a class="way">`, followed with JSON asked: … its `query.text`, `scope.realm` and `scope.membership` equal the request's". That includes `data-way="tabs"`. That link goes to `refresh?realm=…&league=…` (`mod.rs:703-707`, `:287`), whose document is the tabs listing (`mod.rs:1017-1060`). It has no `query`, no `total` and no holder request.
   - *Should say:*
     - The check is one function in `web.rs`, given the server, the page's URL and its reply. The two golden tests call it beside each `pages.page` of a 200 answer, so the pin is those tests plus the function, named in the commit. Alternatively, name the shared-staging exception explicitly.
     - The tabs `<a>` is followed for status alone (200, the refresh page), and its pointer equality is the only other check.

2. **Input, "The facts the tests stand on" / Do 7 / foot: row 10 is pointed at filters that can never stake, and a working recipe already exists.**
   - *Wrong: the brief points at the wrong filters.* It says to record the ambiguous clause "where the build finds a form filter whose fate is `ambiguous` (`search_form.txt` marks them)". The nine `fate-ambiguous` controls in `search_form.txt` are DPS, pDPS, armour, evasion, ES, ward, map tier, quality and crucible. Only two catalogue rows carry an other reading (`other =`): `stat.text.forms` and `stat.scourge.summed` (`crates/acquisition-search/reference/trade-catalogue-v1.toml:1321`, `:1334`). Both are stat clauses. `Stake` exists only where a clause's row carries `other` (`answer.rs:1250-1256`, module doc lines 165-178). No form filter stakes, so a worker following the brief finds none. They would then report row 10 unreachable and raise it to the owner for nothing.
   - *The recipe that works.* `seat_faults.rs:3159-3197` (`an_ambiguous_clauses_line_says_what_its_other_reading_adds_and_drops`, golden `seat_faults/an_ambiguous_clause_adds_and_drops.txt`) stages it:
     - two Iron Rings: one with "25% increased Rarity of Items found", one with "30% increased …" and "10% reduced …";
     - the query JSON `{"query": {"stats": [{"type": "and", "filters": [{"id": "explicit.stat_3917489142", "value": {"min": 20}}, {"id": "explicit.stat_1030153674"}]}]}}`;
     - the result is `stats.0.0` read the other way, drops 1.

     The page's trade box takes a query JSON (`web.rs:697-717`). The drop's route is a query of its own with no `trade` (`answer.rs` module doc lines 119-120), so it folds back through `asked_by_the_form`. Row 10 is reachable in the staged world with two staged items and no new seed function.
   - *Wrong: which world holds row 10 is unclear.* Input describes the staged world's order (tab, removed id, cut list), then puts row 10 apart.
   - *Should say:*
     - Row 10 is staged from that recipe as the staged world's fourth step, after the cut list, its page read in the light of the earlier stagings.
     - Drop "(`search_form.txt` marks them)".
     - Drop the foot's "may reach no golden" raise, or keep it only for a failure the build reports.

3. **Inventory row 11 and Do 4 / Do 8: the zero answer's undecided is reached by no page the brief lists, has no place on the page, and the text prints it nowhere.**
   - *No listed page reaches it.* The one zero answer recorded is the xbox URL page. Its scope holds one item, Iron Grip, which is decided (`web.rs:118-127`; golden `search_pages.txt:181-196` shows no undecided). So `zero.undecided.count` is 0 and carries no request (`Router::count`, `answer.rs:2520-2525`). Yet Do 8 asserts "a recorded page per inventory row a page reaches: rows 1 to 7, 11 to 13".
   - *Do 4 does not place it.* Do 4's zero-answer bullet lists selectors, explain offers and the corpus offer, not the zero's undecided.
   - *The text prints it nowhere, and it duplicates the root's.* `zero.undecided` is built from the same `n_undecided` and `root_undecided` as `total.undecided` (`answer.rs:1994`, `:2112`), so it is the root's count and route again. The text never prints it: `answer_tail`'s zero block prints `resolved_to_nothing` and the offers (`search_cmd.rs:1400-1446`), and the walk takes "undecided at the root" from `total` (`:1577`). Do 2's "one enumeration of the counts", factored from that walk, therefore omits it. Do 8 still demands exactly one `class="way"` element for `/zero/undecided`.
   - *Should say:*
     - Where the zero's undecided renders: say, beside "nothing in scope carries …" as the zero block's own way.
     - That it is outside Do 2's walk.
     - A page that reaches it over `seed`. For example, a pc query whose only candidate is `new`, whose class is undecided, such as `class:rings name="Brood Lure"`; the build confirms it answers 0 matched and 1 undecided. Otherwise, take row 11 out of Do 8's list and raise it.

4. **Input / Do 7: the staged never-fetched tab cannot show "its other leagues".**
   - *Wrong.* Do 7 asks the new golden to record "the never-fetched tab with the tabs link and its other leagues". Input stages "a pc stash listing naming a tab no fetch has read", which is one league. `other_leagues` counts the other leagues in `scope.never_fetched_leagues` that hold a never-fetched tab (`answer.rs:2992-2999`). With one league it is 0, and the words "N other leagues hold never-fetched tabs" (`search_cmd.rs:1075-1084`) are never made.
   - *Should say.* Stage never-fetched tabs in two pc leagues: for example, a `Standard` listing that adds a third tab, and a `Hardcore` pc listing with one tab. Record one page with no league term, where the pointer names the league holding the most and counts the other. Optionally add a second page whose query excludes the pointed league, for the "never a league the query excludes" rule Input states.

5. **Do 2: three phrases the text makes carry a flag, a command, or a place that is false on a page, and Do 2's list of splits misses each. No golden page reaches any of them, so the no-command test cannot catch them.**
   - *The set-aside line.* `answer_head` prints "aside   … — set aside under --context corpus, counted nowhere" (`search_cmd.rs:1026-1030`). Do 4 asks the page for "the set-aside query where there is one" in the text's words. Do 2's list of facts whose words and command are made together omits this line, so the HTML would inherit the flag.
   - *"N more sorted".* This line is pushed into the sort's part, which becomes `shows`, not `more` (`search_cmd.rs:1268-1275`). It goes through the dedupe, and the page prints `shows` and `more` alike (`page.rs:783`, `:801-806`). Do 2 says "`row_shows`'s `more`", so a worker who splits only `more` leaves `acq show …` in a sorted row's cell.
   - *The reasons' remainder.* The text says "N more: the undecided route below lists every item with its reasons" (`search_cmd.rs:1462-1466`). Do 4 puts the undecided way in the total line above the reasons. "Below" is then false on the page if the words are "made once".
   - *Should say.* Name all three in Do 2's list: the aside's flag, the sort's "more sorted" inside `shows`, and the reasons' "route below". Each is split so the text adds its flag, command or place and the page adds its own.

6. **Do 7 / Input: where the basis masking rule goes, and the second test that reads the web goldens.**
   - *Wrong: "the one" reader.* Input says `no_ids.rs` is "one test" that reads the web pages. `normaliser.rs`'s `normalising_a_committed_golden_changes_nothing` also normalises every golden under `tests/goldens/`, `web/` included (`normaliser.rs:292-320`). Its module doc asks for one case per rule (`normaliser.rs:1-8`).
   - *Ambiguous: which `harness/mod.rs`.* Do 7 says to put a masking rule "in `harness/mod.rs` beside `basis_rule`". `basis_rule` runs inside `normalise` over every transcript golden (`harness/mod.rs:675-693`). The page-only rules, such as `basis_meta_rule`, are applied in `Pages::page` (`:1600-1606`) or `Pages::rules` (`:1535-1546`).
     - A rule added to `normalise` must leave every CLI golden unchanged (Do 2 forbids any CLI golden change) and needs a case in `normaliser.rs`.
     - A rule added in `Pages` touches page goldens alone.
   - *Should say.*
     - The basis at the foot is masked either by a page-only rule in `Pages` (as `basis_meta_rule` is), or by rendering the basis so that a line begins `basis   store `, which `basis_rule` already reads.
     - Name `normaliser.rs` beside `no_ids.rs` as a test that must stay green over the new and regenerated goldens.

7. **Do 3: how `page.rs` reaches `Paged`, and the continuation arm's account.**
   - *The signatures do not reach `Paged`.* `page::answer(a, opened, links)` holds no `Web` (`page.rs:760`). `Paged` is `mod.rs`'s private struct over `&Web` (`mod.rs:662-664`). It is reachable from the child module as `super::Paged`. `Surface::request` answers `Option<(Value, Option<&str>)>`, a JSON string or a `{post, fields}` object, not a link (`refusal.rs:167`).
   - *The account check is lost.* The new arm "takes a continuation first" and links it whatever its `scope.account`. `asked_by_the_form` refuses a request for another account first (`mod.rs:542-549`). An answer's own `next` names the served account, so the gap is a refusal's.
   - *Should say.*
     - `page::answer` takes the way-speller as a parameter. Either `&dyn Surface`, or a `&dyn Fn(&Way) -> Option<String>` built in `search` from `Paged`.
     - The continuation arm keeps the same `serves` check `asked_by_the_form` makes.

8. **Input, inventory row 10: "(`Stake`; under `set_aside` the same)" names a pointer that never exists.** The module doc says that under the corpus context the site search "is set aside, and nothing is staked" (`answer.rs` module doc, lines 177-178). So `/set_aside/trade/clauses/<c>/other` never stands. *Should say:* row 10 is `/query/trade/clauses/<c>/other/adds` and `/drops` alone. The way test's walk needs no `set_aside` case beyond the `site` skip.

9. **Do 2: the label example is in the wrong form.** "clause 0 read the other way, adds" should be `clause {at} read the other way, {what}`, with `at` the clause's site path (`search_cmd.rs:1574`). An instance is "clause stats.0.0 read the other way, drops" (`seat_faults/an_ambiguous_clause_adds_and_drops.txt:7`). A worker writing the factored function's test from the brief's example would miss.

10. **Input: "`seed`'s seven items" should be six.** The seed's items are `r1`, `r2`, `kaom` (t1), `new`, `mech` (t2) and `x` (x1) (`web.rs:84-127`). The conclusion stands: no seeded term resolves past ten values.

11. **Read: the review checklist's frontends section is missing.** AGENTS.md says to read an area's section of `REVIEW-CHECKLIST.md` before touching it. "Frontends and the surface an agent meets" (`REVIEW-CHECKLIST.md:103` on) holds two rows this step meets:
    - "a behaviour pinned at three layers" (line 118), which bears on Do 8's "no new test compares the two layouts";
    - "page state the scan cannot see" (line 120), which bears on the new `data-way` markup and anything put in a URL.

    Add the section to the Read list.
