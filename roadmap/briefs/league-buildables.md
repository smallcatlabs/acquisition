# league-buildables — the league milestone's three small fixes

Issues #74, #107 and #112, the step after daemon-safety
(`roadmap/README.md`, "Current step"): the owner's pre-league plan, its
third item. Three independent concerns in three areas; one commit per
concern. The rhythm is "A step".

## Goal

Three things the owner's league run or phase 3b could meet are closed.
A logout retry command the daemon prints runs as printed after a rename
whose index write failed (#74): the account selector also matches a
name the index holds for an entry's keyring, so the old or new name
reaches the one account. The price refusal for an item asked about in
the wrong realm names the realm and league the item lives in, and the
read it names shows the item (#107; the tree suggests `bf4d2586`
already fixed this, so the step proves it and pins it). The two repoe
generators read the poe1 pin for their headers from `tools/sources.py`
instead of typing it (#112), so phase 3b's regeneration names the
export it was drawn from. With these, `decisions/daemon.md` records
three of the owner's rulings as "Not doing" lines.

## Read

- `AGENTS.md`: invariant 5 (refresh tokens never in plaintext on disk),
  C51 below, C122 (a behaviour pinned once, at the lowest crate that
  holds it; wording is a golden), C116 (no id in user-facing text).
- `decisions/daemon.md`: C51 (selection is stateless; the CLI resolves
  `--account` against `accounts.json`, exact, never by prefix), C144
  (no keyring entry out of the index's reach; the held names), C127 (no
  refusal quotes a value whole); "Not doing", where Do 1's lines go.
- `decisions/frontends.md`: C121 (an account that cannot be picked is
  refused as `not_logged_in` or `ambiguous_account` on every surface).
- `decisions/frontends.md`: C146 (a count says what it counts), the
  ruling #107 was found under.
- `search/README.md` and C125 in `decisions/search.md` (a table names
  the export it was drawn from, read from the clone, never typed).
- Issues #74, #107 and #112 (no comments): each "Closes when" is quoted
  in "Done when".
- The code:
  - `crates/acquisition-store/src/index.rs`: `account_matches` (167),
    `AccountEntry::keyring_reach` (89), `Index::resolve` (407) and its
    tests (497 to 550), `settle_keyring_names` (364).
  - `crates/acquisition-daemon/src/daemon/login.rs`: `logout` (508),
    whose not-live branch resolves through `known_account` (606, which
    calls `Index::resolve`); `settle` (the retry command at 695).
  - `crates/acquisition-daemon/src/daemon/tests/refresh.rs:892`,
    `c144_renames_under_an_unwritable_index_leave_no_credential_behind`,
    the frozen-index rename scenario #74's pin builds on (`freeze`,
    `indexed_test_user`, `token_daemon_with_store`).
  - `crates/acquisition-daemon/src/daemon/tests/login.rs:1026`, the
    test that asserts the retry command's words.
  - `crates/acquisition-verbs/src/frontend.rs:214`, `Frontend::account`,
    the client-side door, which also calls `Index::resolve`.
  - `crates/acquisition-plan/src/pricing.rs`: `league_of` (219, reads
    `leagues_of` without the realm), `not_on_record` (296, reads
    `places_of`, which carries the realm, since `bf4d2586`), and
    `a_not_found_carries_its_target_and_the_scope_it_was_looked_for_in`
    (the xbox case near 1650).
  - `crates/acquisition-cli/tests/price_json.rs` and its golden
    `goldens/price_json/list_show_and_refusals.txt`; `seat_faults.rs`.
  - `search/repoe/scripts/trade-stat-map.py:35` and
    `search/repoe/scripts/base-taxonomy.py:40`, the pin typed as a literal (poe1@e2bd511a);
    `search/repoe/scripts/base-defences.py:44`, the shape to copy
    (`PINS['poe1'][:8]`); `tools/sources.py` (`PINS`, `POE1`, `CORPUS`,
    `require`).
- `python3 -I tools/bothy.py pack --path` for each file above names
  what else to read before touching it.

## Input

The last push's CI, read before this brief: `Gate (Linux)` run
40959800831 and `Gate (Windows)` run 40959800931, both on `380ef102`,
both completed `success`. Nothing to fix or file.

#74's failing input: a session renamed from test-user to B while the
account index cannot be written, so the index keeps test-user as the
entry's username with B among its held keyring names (C144's order: the
index holds a name before a credential is saved under it). The worker
confirms this state against `c144_renames_under_an_unwritable_index_…`
before building on it. Then a logout of B whose keyring clear fails,
with the index writable again, and the printed retry
`acq auth logout --account B` run with the session gone. Today the
retry reaches `known_account`, where `Index::resolve("B")` matches
neither the username test-user nor the uuid, and refuses as "no account
matches". The worker sees it fail first and says so in the report.

A correction to #74's text, measured on `380ef102`: `--account`
already accepts an exact uuid (`account_matches`, line 170; the CLI's
help at `acquisition-cli/src/main.rs:197`; MCP's `ACCOUNT` text). The
issue's reason for not printing the uuid, "a surface change", no longer
holds. The owner chose the smaller shape on 2026-10-29 anyway: the
printed command keeps the name a user knows.

#107's input: `bf4d2586` (2026-10-27) moved `not_on_record` to
`places_of`, which reads the realm, after h2 wrote #107's row
(2026-10-26). The plan crate's test already asserts that `price show
--realm xbox item/i1`, for an item held in pc/Standard, says "the facts
hold it in pc/Standard: `--realm` and `--league` read it there". What is
not pinned is that the read it names, run with those flags, shows the
item. `league_of` still reads `leagues_of` without the realm. That is
harmless while an item's id lives in one realm, and the step leaves it
alone unless the pin fails.

#112's input: the pinned poe1 clone is at poe1@e2bd511a, the pin in
`PINS`. Both raw stores in `CORPUS` and the three
`search/trade-query/data/*-2026-09-12.json` captures the scripts read
are present in this checkout. So both scripts run in place. Their
extracts are the files each writes under `search/repoe/data/`.

## Do

1. **The owner's rulings first, in their own docs commit.** Add three
   lines to `decisions/daemon.md`, "Not doing", in the form the
   existing lines take:
   - An allowlist of GGG's policy and rule names in the daemon log: not
     until there is a demonstrated need (the owner, 2026-10-29,
     daemon-safety's close).
   - Keeping GGG's text out of the send journal: declined. The journal
     keeps GGG's rate and header text verbatim, and invariant 5 is not
     touched, because no token reaches it (the owner, 2026-10-29).
   - Typing GGG's text where it enters the daemon (#132): held until a
     need or value is identified (the owner, 2026-10-29).

   Then add a comment on #132 citing the commit.
2. **#74.** A selector also matches an index entry by any name the entry
   holds for its keyring, by `account_matches`' rule for a username.
   One rule in `Index::resolve`, so the daemon's `known_account` and
   the frontends' `Frontend::account` both inherit it. A name that two
   entries match is `ambiguous_account` as now (C121). The doc comments
   that list what a selector matches change with it: `index.rs` 165 and
   406, `frontend.rs` 212. Whether the CLI's `--account` help and MCP's
   `ACCOUNT` text name the old name is the step's own agent-facing
   decision. The brief's lean is no, because only a printed retry
   leads a user there. Record it in the commit and list it at the
   close. The pin is a daemon test over #74's failing input: the
   printed retry, run as printed after the session is gone, clears the
   held name, and the index releases it. The unit tests in `index.rs`
   gain the held-name match and its ambiguity.
3. **#107.** A CLI process test in a mock world: an item held in one
   realm, asked for with `acq price show --realm <the other>`. The test
   reads the refusal's named realm and league, runs the read with those
   flags, and finds the item shown. If the test passes at once, the
   commit says that `bf4d2586` fixed #107 and that this pins it. If it
   fails, fix by choosing the remedy with the item's realm, one rule.
   Add the test where the harness's price tests already live
   (`price_json.rs`); no new binary (C122).
4. **#112.** Both headers take `PINS['poe1'][:8]` as `base-defences.py`
   does, imported from `sources`. Run each script in place. The pin is
   the run: `git diff --exit-code` over `search/repoe/data/` is empty.
5. The full gate, then the close per "A step".

## Done when

- `decisions/daemon.md`, "Not doing", holds the three lines of Do 1 in
  a commit of its own that touches no code.
- #74: `Index::resolve` matches a held keyring name, and a daemon test
  named for #74's case shows the printed retry command running as
  printed after a rename whose index write failed, with the held name
  cleared and released. #74's "Closes when" is "the printed retry
  command runs as printed after a rename whose index write failed,
  pinned by that input."
- #107: a CLI test asks for an item in the wrong realm and runs the read
  the refusal names. #107's "Closes when" is "the remedy is chosen with
  the item's realm, and a test asks for an item in the wrong realm and
  runs the printed remedy." The commit says whether the code changed or
  `bf4d2586` already held it.
- #112: neither script types the poe1 pin (poe1@e2bd511a) as a literal; each header is
  built from `PINS['poe1'][:8]`, and the extracts are byte-identical
  after a run in place (the commit message says it was run).
- `tools/gate.sh` green on the step's tree; `CLI-REFERENCE.md` and
  `MCP-REFERENCE.md` regenerated only if Do 2 changed help text.

## Not in scope

- Printing the uuid in the retry command (the reviewer's shape). The
  owner chose the name.
- `league_of` reading the realm, unless Do 3's test fails without it.
- #132, the log allowlist or any journal change beyond the three lines.
- The other repoe or trade-forms tools: `tools/trade-forms-table.py`'s
  trap was fixed in 2e. A third typed pin found in passing is named in
  the report, not fixed.
- Tools hygiene (#117, #118, #126, #114, #106), the next step.

## Rules

`TEMPLATE.md`'s. What differs:
- The shared checkout, no worktree; scratch under
  `$TMPDIR/league-buildables/`. Every `acq` runs in a mock world
  (the mock-session skill).
- Do 4 runs the scripts against the owner's local raw stores, read-only
  (`mode=ro`, as the script opens them). Nothing else reads them.
- A finding in a review may be taken stricter than proposed. Say
  "taken, but stricter" and why.

## Report

`TEMPLATE.md`'s.
