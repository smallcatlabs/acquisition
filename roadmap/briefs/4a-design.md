# 4a — the 4-lite design: the trade-view GUI's rulings

## Goal

The rulings 4-lite needs are in the registry and the 4-lite build
brief is written, so the build starts the day the owner rules. 4-lite
is the owner's call of 2026-10-24: a simple, read-only loopback web UI
shaped like the trade site's search — the form, the results table, the
counts — in users' hands for league 3.30, with the live-update loop
(SSE, the subscriber, reconnect) deferred to 4-full after the time
seam (phase 4-0, `roadmap/TIME-SEAM.md`). This is a design brief: it
ends in rulings and a build brief, not code.

## Read

- `roadmap/GUI-NOTES.md`: sections 2 to 9, 11 and 12 (the ruling
  packet is section 9; the build sequence it proposed is section 12,
  which this design re-cuts for the 4-lite/4-full split).
- `roadmap/TIME-SEAM.md`: section 4's "not in 4-0" line and C137's
  template rule, which bind 4-lite's tests from its first commit.
- `decisions/frontends.md` whole; `AGENTS.md` C12 (two surfaces), C46
  (shared semantics in Rust), C88 (the real provider and the human),
  C98 via `decisions/search.md` (the consumer holds the corpus), C85
  (the subscription contract 4-lite does not use), C120 (never posts).
- Issue #1, the keeper bundle, far enough to judge whether a
  terminal-launched loopback server needs any of it.

## Input

Fixed: the owner's rulings of 2026-10-19 (loopback web UI first,
Tauri a later wrapper) and 2026-10-24 (4-lite before the league;
2g and 2h after phase 3; the time seam before 4-full). The ruling
packet of `GUI-NOTES.md` section 9, recommendations included. The
league date: phase 3's window, which 4-lite must beat.

## Do

1. Scope the ruling packet to 4-lite and put it to the owner, question
   by question, his answer verbatim into the registry under the entry
   rule. Needed for 4-lite: 1 (C16's amendment), 2 (the listener's
   security — the one ruling that must not be rushed), 3 (the spawn
   policy), 5 (images: the recommendation is out, text-only), 6 (the
   table's shape), 7 (the toolchain), 8 (sort), 9 (the listing-only
   filters, disabled). Question 4 (the outbound encoder for "open on
   the trade site") is the design's to take or defer on the clock's
   evidence; question 10 is overtaken by the reorder.
2. Decide where the server holds its corpus (C98: the consumer holds
   it and checks currency before every answer; the server is the first
   long-lived consumer). The `acq-mcp` corpus move is 4-full's unless
   this design finds it load-bearing now.
3. Decide the keeper bundle in or out of 4-lite; the lean is out for a
   terminal-launched server, named explicitly either way.
4. Re-cut `GUI-NOTES.md` section 12 into the 4-lite build sequence:
   the web crate (listener, security, lifecycle, rows and counts
   routes as thin wrappers over the shared layer), the form from the
   committed filter definitions, the card and table. No SSE, no
   subscriber, no `Searcher` loop, no revision push — refresh is the
   user's, or a dumb poll if the design rules one.
5. Write the 4-lite build brief from `TEMPLATE.md`, with two done-when
   lines this file fixes in advance: every test obeys C137's rule (no
   wall-clock bound a slow machine can miss), and the moved-world
   review passes — every unknown-game-content path (a new item class,
   an untranslated mod, the league mechanic's items, exercised with
   2e's game-diff and rehearsal machinery) degrades to display with a
   badge, never a refusal or an empty answer, because launch users
   meet exactly those items first.

## Done when

- Each packet question 4-lite needs has the owner's answer in the
  registry, under 800 bytes each, bothy green.
- The 4-lite build brief exists, its Read checked against the tree,
  its done-when carrying the C137 line and the moved-world review.
- The corpus and keeper decisions are written down with their reasons,
  in the brief or the registry as their weight earns.
- This brief's open items are answered in the design session's close,
  each taken, filed or declined by name.

## Not in scope

- Code, including spikes: a timeboxed spike the design wants is named
  in the build brief it writes.
- The subscriber, SSE, reconnect, live invalidation: 4-full, after
  4-0 (`roadmap/TIME-SEAM.md`).
- Binding beyond loopback, images, posting anywhere (C120 — never):
  each parked with its trigger in the rulings this design writes.

## Rules

`TEMPLATE.md`.

## Report

`TEMPLATE.md`; the design session reports to the owner directly.
