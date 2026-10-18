<!-- written during the run; call numbers are rows of calls.csv -->

## 1. Before touching anything

First account, from `open` only (call 1):

- It can search every item in the local store (search, show, describe), list accounts, tabs, characters, item events and store status, all read straight from a SQLite store with no daemon and no network (call 1). A trade-site search (URL, id or JSON) is accepted as `{"trade": …}` and translated to a text query (call 1).
- It does not price anything: "a price is the owner's own listing" (call 1).
- It needs the daemon and the game's API only for the job tools (submit_job, job_status, job_result, list_jobs, cancel_job, daemon_status) and apply_plan. Login is done by the human with the `acq` CLI (call 1).
- The sync-policy tools (sync_policy, set_sync_policy, refresh_plan) are local and send nothing (call 1).
- I would not run without asking the person: apply_plan, submit_job and cancel_job, because they spend API requests or change jobs. I would also ask before set_sync_policy, because it rewrites their stored intent (call 1).

The world as found (calls 2–6, nothing changed):
- Account `GERWARIC#7694`, mock provider. The store holds 22721 items, 6 removed, 3444 tabs and 65 characters (call 2).
- A bare `search` was refused because the store holds two realms, pc and poe2. The refusal's offers were the request for each realm (call 3). The pc realm has 22623 live items: 21311 in Standard and 1312 in Allflame (call 4).
- Of those, 546 were fetched and 2940 never were (call 4). The oldest fetch is 1788810111 and the newest 1792074048 (call 4). The tab list shows most tabs fetched at 1788884440–1788884505, and a few at 1792074047–8 (call 5).
- Many characters were never fetched (`fetched_at: null`), and several were fetched with 0 items (call 6). So "what I have" is only as complete as that fetch. I will say so with each answer.
- `tabs` answered 745672 bytes and the door printed only the first 3000 characters (call 5). I lost the tab list beyond tab idx 13. I did not need it for T0.
