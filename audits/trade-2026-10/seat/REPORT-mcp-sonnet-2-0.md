<!-- written during the run; call numbers are rows of calls.csv -->

## 1. Before touching anything

First account, from what `open` printed (call 1) only:

- What it can do: search/describe/show items in a local SQLite store (search by text query or pasted trade-site search, translated offline), list accounts, tabs, characters, store status and item events; declare a sync policy, compile a refresh plan and apply it; submit jobs to a daemon that fetch from the game's API (call 1).
- Needs: the store tools (search, show, accounts, tabs, characters, store_status, item_events, sync_policy, refresh_plan, describe) need no login, daemon or network. Job tools (submit_job, job_status, job_result, list_jobs, cancel_job, apply_plan, daemon_status) need a running daemon, and fresh data from the game needs a human login via the `acq` CLI (call 1).
- Would not run without asking the person: apply_plan and submit_job (they spend rate-limited requests against the real API in real mode), set_sync_policy (writes their stored intent), cancel_job. In this rehearsal world they are safe but I would still not use them for reading tasks.
- Note from the instructions: a price is only the owner's own listing; the trade-search translation may be partial, and the store holds items in every place, not just what the site would show (call 1).

Then the world as found (no change made):
- One account, GERWARIC#7694, two realms, pc and poe2; a bare `search {}` is refused with `realm_needed` and offers per realm (call 2). store_status: 22721 items, 6 removed, 65 characters, 3444 tabs (call 3).
- pc: 22623 live items, 21311 in Standard and 1312 in Allflame by the league table; but scope lists 7 leagues (call 4). 546 fetched, 2940 never fetched (tabs/pages/characters that a listing names and no fetch has filled); oldest fetch 1788810111, newest 1792074048, seconds since 1970 (call 4). Many Standard tabs are "Remove-only" folders/tabs (call 4).
- poe2: 98 items, all Standard, 1 never fetched (call 5) — not relevant to the person's Standard pc asks.
- Characters in pc Standard: 40 listed; several have fetched_items 0 or fewer fetched items than item_count (e.g. DLVR 0 of 0, OnlyCircles 33 of 66), so character gear is partly unseen (call 6). Fetch times are mostly 1788884xxx, older than the newest stash fetch (calls 4, 6).
