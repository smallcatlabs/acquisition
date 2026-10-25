# 4b, the sitting round — eleven stops, five rules

## Goal

The owner sat at `acq web` on 2026-10-25 (`step-4b` after its two fix
rounds, in the worktree beside this checkout) and
brought eleven stops. Read one at a time they are eleven patches; read
by the rule each expresses they are five, and two of the five remove
mechanism. This round builds the five, each by its rule and pinned at
the lowest crate that holds it (C122), so the page meets the next
league with one road for every refresh, a form that reads as the
site's, and an escape hatch for everything the site's form cannot
say. The rulings it rests on are in the registry first: C44 amended (a
selection plan carries no policy revision), C142 (the defences'
reading beside the trade JSON) and C143 (a shared refusal speaks the
request's words), all taken 2026-10-25.

## Read

- `roadmap/README.md`, then `roadmap/briefs/4b-build.md` (the build
  this round follows; its Rules bind here), then this file.
- The rulings: C44 as amended, C111 (each request spent once) and C38
  (a plan is a binding envelope) in `decisions/plans.md`; C141, C142,
  C114 (the property bridge: every site property a field by its
  displayed name) and C113 in `decisions/trade.md`; C143, C116, C53 in
  `decisions/frontends.md`; C93 (absent is false) and C106 (data and
  convention) in `decisions/search.md`; C122 and C46 in `AGENTS.md`.
- The planner: `crates/acquisition-plan/src/lib.rs`, `PlanBasis`
  (`policy_revision: i64`, whose doc says "there is no ad-hoc path",
  which this round makes true the other way round), `plan_refresh`,
  the spend check (`SpendError::PolicyGone`, `PolicyMoved`, near line
  1440) and the `c44_` tests; `crates/acquisition-store/src/snapshot.rs`,
  `RefreshSnapshot` and how `refresh_snapshot` reads the stored policy
  row; `crates/acquisition-verbs/src/refresh.rs`, `Opened::compile`,
  `plan`, `apply`.
- The ad-hoc kind: the `refresh` submit in `crates/acquisition-cli/src/main.rs`
  (`refresh_params`), the kind in the daemon, and the CLI help's
  parenthesis that marks it to retire; `acquisition-cli/tests/apply_loop.rs`
  and `plan_json.rs`, C38's pins the selection plan must also satisfy.
- The search crate: `derive.rs` (the `hybrid` read near line 588, the
  flags and `flags_unknown`), `reference/properties-v1.toml` (the
  `armour.q20` family, `reads = "q20"`), `reference/trade-catalogue-v1.toml`
  (`misc_filters.gem_vaal`, the four `armour_filters` rows),
  `trade/translate.rs`, the count keys' parse (today in the CLI crate:
  find it by grepping `line:` in `crates/acquisition-cli/src/search_cmd.rs`),
  the refusal of a count beside a sort (grep `orders rows` in
  `crates/acquisition-search/src`); `tests/properties.rs`,
  `trade_translate.rs`, `trade_decode.rs`, `unread.rs`.
- The store: `league_list` in `crates/acquisition-store/src/lib.rs`
  and the `leagues` job kind (one GET, no probe: the live-run skill,
  "A new league on a contacted route").
- The web module, `crates/acquisition-cli/src/web/`: `mod.rs` (the
  tabs page near line 570 to 690, the guard), `page.rs`, `form.rs`,
  the assets; `tests/web.rs` and its goldens.
- The owner's page save of the trade site, local beside
  `search/trade-query/data/` (the surfaces register: a human's
  recording; read for the form's organization and order only, copy
  nothing of GGG's, fetch nothing).
- `RUN-LEDGER.md`, its tail, for the row's form; the sitting's journal
  at `/Users/tom/Library/Logs/acq/b646a62da0d2/ggg/sends.jsonl`, which
  you have counted.

## Input

Fixed: the three rulings; the owner's eleven stops, each quoted in
the Do item that takes it; the sitting's journal, 202 sends in three
daemon lifetimes (191 GET, 6 POST, 5 HEAD), one non-2xx, a token POST
answered 400 at 16:26:48Z before the owner re-authenticated, no 429,
no trip; the ceiling of 10 that halted the first deep refresh after
one substash, and the 300 the owner restarted under. The failing
inputs the tests stage: a selection plan applied with no stored policy
row; a policy plan applied after the row moved (still refused); the
same selection envelope posted twice; a Vaal Regalia and a Vaal gem
under the site's Vaal filter; an armour piece at 0 %, 12 % and 20 %
quality under both readings; `view.count=rarity&view.sort=ilvl`; a
league in the store's list with no tabs on disk.

## Do

In this order; one commit per concern; the gate before each commit.

1. **The tables grow by a row** (stops 3 and 7; the owner: "we could
   also filter for class = skill gem and look for gems with a vaal
   tag"; "i want to be able to search for the value of the armour at
   the quality of the item as it stands as well in the cli").
   - `is:vaal`: the derivation lifts `hybrid.isVaalGem` into the flags
     as every other body mark is lifted; `vaal` joins the closed set
     of `is:` marks and the described fields; the catalogue row
     `misc_filters.gem_vaal` lands on `is:vaal`, disposition exact,
     its note and `says` rewritten, the type-line guess gone. Pin in
     `trade_translate.rs` with a Vaal Regalia that must not match and
     a Vaal gem that must.
   - The displayed defences: four rows in `properties-v1.toml` named
     `armour`, `evasion`, `energy_shield`, `ward` (or the family's
     existing stems: read the four `.q20` names and drop the suffix),
     `reads = "number"`, each defined as the displayed value as the
     item stands; the described fields regenerated. Pin in
     `properties.rs` at 0 %, 12 % and 20 % quality beside the `.q20`
     twins.
2. **The selection is a plan, and the ad-hoc kind retires** (stop 6;
   C44 as amended; the owner, verbatim: "a plan compiled from an
   explicit selection carries no policy revision and the apply checks
   none, its authorization being the selection itself; the ad-hoc
   refresh kind retires once the page and the CLI submit selections
   as plans").
   - The planner compiles a plan from an explicit selection (tabs by
     id, characters by id, deep or not) over the same snapshot, with a
     basis that says so in place of a policy revision: the shape is
     yours (an enum on the basis, or an option), the rule is C44's.
     The spend check skips the policy comparison for a selection plan
     and keeps it for a policy plan; C111 and C38 hold unchanged. Pin
     beside the `c44_` tests: a selection plan applies with no policy
     row, a policy plan after a moved row still refuses, a selection
     envelope posted twice sends nothing.
   - The verbs gain the selection compile beside `Opened::compile`;
     the plan page renders a selection plan as it renders a policy
     plan, the quote included, and its text says the plan is a
     selection. A never-fetched map or unique tab under deep yields
     the tab's fetch and says its pages await the fetch, as the policy
     road does today; the plan page says to apply again for them.
   - The tabs page's "Refresh selected" compiles the selection and
     lands on the plan page; apply is the one apply route. The CLI's
     `refresh --tabs` and `--all` compile a selection plan and apply
     it through the same road, printing the plan first; the help's
     parenthesis about two doors goes.
   - The ad-hoc `refresh` kind is removed from the daemon, the
     protocol's kinds, the CLI and the tests that submit it, in the
     round's last commit, once nothing submits it. If its removal
     reaches code this brief has not named, stop and name it in the
     report rather than widen the round.
3. **A page renders one more document that exists** (stops 5 and 11;
   the owner: "it looks like the daemon is stuck"; "there's no way to
   list leagues?").
   - The jobs page opens with the daemon document's rails line and
     its halt, as `acq daemon status` prints them, so a queue that
     stands still says why. Golden.
   - The league control on both pages is a select over the store's
     `league_list`, each league marked with whether tabs for it are on
     disk, with a "fetch the league list" button that submits the
     `leagues` job and lands on the jobs page; the free text box goes.
     With no list on disk the select is empty and the button is the
     only control. Golden, with the fixture's list and one league with
     no tabs.
4. **The page's organization is the site's, and every list is a
   collapsible group** (stops 4, 9 and 10; the owner: "make map and
   unique stashes collapsible under their parent"; "make the entire
   stash list collapsible and the characters list collapsible as
   well"; "i want the organization and layout to match more closely,
   so people feel like they are looking at the trade site"; "just for
   form layout for now").
   - The form takes the site's organization and order as the owner's
     local page save shows it: the dark page, the name search bar
     across the top, the filter groups as panels in the site's columns
     and order, each with its header, its enable toggle and its rows
     of label with min and max or a dropdown, the stat block with its
     picker, the search button, the result count at the head. No font,
     art or stylesheet of GGG's; the organization alone. The results
     keep their table, with the columns in the card's order and the
     rarity colour on the name. One template and stylesheet pass;
     the goldens are regenerated and read.
   - On the refresh page the stash list and the character list are
     each a `details` group with a summary carrying the count, open by
     default; inside the stash list each map or unique tab is a
     `details` group holding its substash rows, closed by default.
     No script. Golden.
5. **A shared answer speaks the request's words, and shared parsing
   lives in the shared crate** (stops 1 and 2; C143).
   - Every refusal in the search crate that names a flag is reworded
     to the view's field or the term (C143); the CLI adds its flag
     spelling after, where it wants to. Find them by grepping `--` in
     the search crate's error strings. The goldens that carry the old
     words are regenerated and read; `no_ids.rs` passes.
   - The count keys' list parse (`line:resist,life`, a comma inside an
     entry part of its text) moves from the CLI to the search crate
     as one function; the CLI, the MCP's array form and the page's
     count control all call it. Pin once, in the search crate.
6. **The escape hatch** (the owner's "is there a way to search for
   armour that is not at q20?", which the site's form cannot say):
   the translated query is shown in an editable box under the form,
   as the design notes drew it; editing it drops the trade input and
   runs the text as the user's own query, a rule the crate already
   has, and the answer page shows the text it ran. The box is the one
   place every field the language has is reachable from the page, so
   the next "the site cannot say X" lands there and not in a control.
   Golden.
7. **The one convenience** (stop 8; the owner: "a checkbox in the
   armour filters section to turn it on/off, although under the hood
   it may not be best implemented as a boolean flag"): the trade
   decoder reads `defences` beside `league` (C142); the translation
   lands the four armour-filter rows on the displayed or the
   normalised family by it and labels each clause with the field used;
   the page's armour panel gains the checkbox and the fold writes the
   option; `--trade` on the CLI takes the same envelope key. Pin in
   `trade_decode.rs` and `trade_translate.rs`.
8. **The ledger row and the close's inputs**: draft the sitting's
   `RUN-LEDGER.md` row by the tail's form from the journal counts in
   Input (the deep refresh of one map tab, 245 substash fetches, the
   ceiling halt at 10 and the restart at 300, the one 400); the
   round's last commit carries the step's ledger line with this round
   and sets "Current step" to say 4b's close awaits the owner.

## Done when

- Each of the eleven stops is answered by one of the Do items above,
  named in the commit that takes it.
- `is:vaal` and the four displayed defence fields exist, described
  and pinned; the Vaal catalogue row is exact.
- A selection plan compiles, renders, quotes and applies through the
  one apply route from the page and from `refresh --tabs` and
  `--all`; the `c44_` pins hold both halves of the amended rule; the
  ad-hoc `refresh` kind is gone from the daemon, the protocol, the CLI
  and the tests, or the report names exactly what stopped its
  removal.
- The jobs page carries the daemon's rails line and halt; the league
  control is a select over the store's list with its fetch button.
- The form's organization is the site's and every refresh-page list
  is a `details` group, both pinned by goldens that name a panel
  heading, a group summary and a substash row.
- No refusal string in the search crate names a flag; the count keys'
  list parse has one home and three callers.
- The editable query box exists and runs an edited text as the user's
  own query, pinned.
- `defences` is read beside the trade JSON and lands the four rows by
  it, pinned; the armour panel has its checkbox.
- The `RUN-LEDGER.md` row is drafted; the ledger line carries the
  round; the gate is green on the round's own tree; no test asserts
  a wall-clock bound a slow machine can miss (C137).

## Not in scope

- The results as site-style cards, rows carrying every field, the
  dense table: 4-full (the owner: "just for form layout for now").
- SSE, the subscriber, reconnect: 4-full after 4-0.
- A text query parsed on the page, a second picker, a client-side
  anything: the box sends text and the search crate reads it.
- Count keys taking more than the CLI takes: the page takes exactly
  the CLI's list, through the shared parse.
- Removing `armour.q20` or changing what the site's reading computes:
  the displayed family stands beside it.
- A third Codex review of this round before the owner asks: the
  round's review runs once, as "A step" says, over its commits as one
  range.

## Rules

`TEMPLATE.md`, and `4b-build.md`'s four: a page renders a document
and nothing else; every page is user-facing text under C116; the
server calls the verbs and opens nothing itself; nothing the build
runs touches the real provider.

## Report

`TEMPLATE.md`.
