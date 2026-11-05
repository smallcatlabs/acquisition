# 44-folder-once — the store says "folder" once

Issue #44, built at the owner's word of 2026-11-04 (the night run),
`trigger:consumer` waived. Written by the orchestrating session against
the tree at `f06f34f0`, #108's close.

## Goal

GGG's tab type `Folder` (a tab that groups tabs and holds no items) is
spelled and compared by each consumer for itself: the store's coverage
and two SQL statements, two private `FOLDER` constants in the plan
crate, the plan's skip reasons, the search's tab listing, the verbs'
refresh, the daemon's queue, the CLI's price and web code. A typo or a
second spelling in one place would silently count a folder as a tab
with no items, or the reverse. When this step is done the store says
what a folder is once, and every consumer asks it.

## Read

- `AGENTS.md`, C1 (the store links no workspace crate; everything
  that needs it links it, so the store is the lowest home), P4
  (structure over discipline).
- `decisions/store.md`; `REVIEW-CHECKLIST.md`, the store section.
- The sweep below, each site read in context.

## Input

The sweep, run on the tree this brief was written against
(`grep -rn '"Folder"\|'"'"'Folder'"'"'' crates/*/src`, then the
consts):

- store: `coverage.rs:182` `is_folder(location)`; `lib.rs:2027`,
  `:3144`, `:3150` (SQL `type = 'Folder'`).
- plan: `lib.rs:2099`, `:2304` (`tab.r#type == "Folder"`);
  `shop.rs:161` and `listing.rs:181` (`const FOLDER`), used at
  `listing.rs:1329`.
- search: `tabs.rs:120`, `:155`.
- verbs: `refresh.rs:77`.
- daemon: `daemon/queue.rs:1297`; `mockggg.rs:273`, `:598`, `:628`,
  `:1546` (the mock GGG's own world).
- cli: `web/mod.rs:629`, `price_cmd.rs:980`.

Run it again first; the measure of done is that it prints only the
store's one definition (and the mock's data table, if kept, see Do 3).

## Do

1. **One definition in the store.** A public constant for GGG's word
   and one predicate over a tab type, documented once (what a folder
   is: GGG's grouping tab, no items of its own; fetched never). Where
   the store's row types carry a type (`lib.rs:677`, `snapshot.rs:58`,
   `corpus.rs:215` and the coverage row), a method on each that calls
   the predicate is welcome if it reads better at the call sites; one
   predicate underneath.
2. **Every consumer asks it**: the plan (its two private constants
   gone), the search, the verbs, the daemon's queue, the CLI. The SQL
   uses the constant, not a second literal (a bound parameter, or a
   statement built from the constant once).
3. **The mock GGG** (`mockggg.rs`) speaks GGG's side: its tab table
   may keep the literal as data, since it is GGG's wire value, but its
   comparisons call the predicate. Say which you did.
4. No behaviour changes: every existing test passes unchanged. No new
   test is needed for a refactor that a type holds; if you add one,
   it pins the predicate at the store (C122).

## Done when

- The Input's grep prints only the store's definition (and the mock's
  data rows, if kept, said in the commit).
- No crate other than the store declares its own folder constant.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- Any other GGG tab type (`MapStash`, `UniqueStash`, …): this step is
  the one with four comparers.
- How folders are counted or skipped: unchanged.

## Rules

`TEMPLATE.md`, and as #18's brief: the main checkout on `main`, commit
locally, no push; every `acq`/`acq-mcp` in a mock world
(`ACQ_PROVIDER=mock`, `ACQ_STORE_DIR` under your scratch), never a bare
`acq` or a real-provider daemon.

## Report

`TEMPLATE.md`.
