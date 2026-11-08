# The page seat, probe run — `acq web` driven through the browser pane

Written 2026-11-07 at the owner's word ("yes, setup the page seat").
The seat skill (`.claude/skills/seat/SKILL.md`) sits a blind agent at
the CLI or the MCP through a logging door. This run asks whether the
same can be done at the pages: a started session, in a mock world over
a copy of the owner's store, driving `acq web` through the desktop
app's built-in browser pane. It is a probe of the mechanics first and a
walk through the saved pages second. It is not blind: the runner may
read the repository. A blind run follows once the mechanics are known.

## What the run must answer (the probe)

1. Does a started session have the browser pane's tools
   (`mcp__Claude_Browser__*`)? If not, say so and stop; nothing else
   can be answered.
2. Does the page work in the pane: the key as the URL's first path
   segment kept across the picker's own fetches (`stats?q=`), the CSP
   letting the picker script run, the item art loading or refused?
   Read `read_network_requests` for the `stats` calls and say what
   path they used.
3. Are screenshots worth their tokens here, or does `read_page` /
   `get_page_text` answer every question? Say which you used for each
   stop and why.
4. Did the pane refuse anything (a site, a form, a download)?

## The world

Everything runs in a mock world. Never unset `ACQ_PROVIDER`; never run
a bare `acq`; never touch `~/Library/Application Support/acq`,
`~/Library/Logs/acq` or any path outside your scratch directory and the
repository. The real provider is the default (C88): a shell without
`ACQ_PROVIDER=mock` opens the owner's store.

```sh
unset ACQ_GGG ACQ_TRIPWIRE ACQ_MAX_SENDS ACQ_IDLE_SHUTDOWN
export ACQ_PROVIDER=mock ACQ_NO_KEYRING=1 ACQ_NO_SPAWN=1
export ACQ_STORE_DIR=<scratch>/world/store ACQ_LOG_DIR=<scratch>/world/logs
mkdir -p "$ACQ_STORE_DIR" "$ACQ_LOG_DIR"
```

The store is a copy of the owner's, 22,721 items, read-only for the
measurements (`search/MEASUREMENTS.md`, M3): make a fresh sqlite
`.backup` of `search/item-facts/raw/spike-GERWARIC_7694-2026-09-13.db`
under your scratch (never move or write the original), then wrap it:

```sh
sqlite3 search/item-facts/raw/spike-GERWARIC_7694-2026-09-13.db ".backup <scratch>/world/copy.db"
cargo build --workspace
cargo run -p acquisition-search --example copy-as-store -- <scratch>/world/copy.db "$ACQ_STORE_DIR"
```

The example records the copy's account in the world's index and moves
the copy into place, so `acq web` serves that account with no login.
`ACQ_NO_SPAWN=1` means a refresh or an apply from a page is refused
rather than starting a mock daemon: both are out of this run's scope.
Saved searches land in the world's own `user.db`, never the owner's.

Start the pages in the background and keep their output:

```sh
./target/debug/acq web --no-browser --json > <scratch>/world/web.out 2> <scratch>/world/web.err &
```

The URL is `web.out`'s one line. Open it in the pane (`preview_start`
with `url`, or `navigate`). Read `web.err` at the end for anything the
server said.

## The walk

Before the walk read `README.md`'s tour lines for `acq web`, `acq
search` and `acq saved`, and `CLI-REFERENCE.md`'s `acq saved` entries.
Then, in order, each a numbered stop in the report whether or not it
went as you expected:

1. **The empty list.** Open Saved before anything is written. Quote
   the words the page shows with no rows.
2. **Save from an answer.** Run a search (any class and a rarity), save
   it from the form, and record where you land and what the row shows.
   Reload the list: say whether the browser re-posted or asked to.
3. **Open.** Open the row. Say whether the query box holds the text and
   the realm is the row's.
4. **Rename and clear** on the page. After the clear, put the row back
   from the shell with the undo command `acq saved clear`'s receipt
   prints (do the clear a second time from the shell with `--json` to
   see the receipt, after saving again on the page), and confirm the
   row returns at its id.
5. **The conflict path.** Open a row's rename form in the pane, change
   the same row from the shell with `acq saved set --id … --name …`,
   then submit the form in the pane. Quote the notice.
6. **The handoff.** Save one from the shell with `acq saved set --name
   … 'class:ring rarity=rare'`, reload the list in the pane. Save one
   on the page, list it from the shell with `acq saved list`, run it
   with `acq search --saved <name>`.
7. **The picker.** On the search page, in "Find a stat", type `maximum
   life`. List what it offers. Then type `pseudo.pseudo_total_life`
   straight into a stat row's id with a min of 80 and search: quote
   the fate text beside the row and say how many rows the answer has.
8. **The query box.** Run `pseudo.total_life>=80` from "The query, in
   the language". Compare its row count with stop 7's. Then set Sort
   by to `pseudo.total_life` with highest first.
9. **The back button** after a save's redirect: where does it land, and
   does the browser ask to re-post?
10. **Anything else** that stopped you: a word you did not understand,
    a control that did nothing, a page that did not say what you
    expected. The tool is under test, never you.

Two things already on the ledger, not stops: the " · " after a full
stop in the written-since notice, and an empty query box running the
empty query.

## The report

Write `audits/web-2026-11/seat/REPORT-probe.md`: the four probe
answers first, then the stops by number, each with what you did, what
the page said (quoted from the page's text, not paraphrased), what you
expected, and which pane tool showed it. Then: what you changed in the
world and anything outside it (there should be nothing outside). End
with "which of these stops would you not bet on?" answered.

Commit nothing and push nothing; the orchestrating session reviews the
report and commits it. Do not edit source, do not run the gate, do not
fix anything you find: record it. Stop `acq web` (`kill %1`) at the
end; there is no daemon to stop.
