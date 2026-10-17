# The trade import's seat — the record

Step 8 of `search/TRADE-PLAN.md`, under `PLAN.md` beside this file. The
close (the plan's step 10) adds the runs, their call counts and trees,
the fate of every fault and the debriefs' answers routed.

## The worlds

Made 2026-10-17 at `538022d6`, outside the repository under
`~/seat-worlds/trade-2026-10/`. The store is the owner's `GERWARIC`
facts file and its intent file, copied once with the daemon stopped
into `pristine/`, which nobody runs in; every run's world is copied
from it (`seat.py setup --store`). A world holds its brief (as
`BRIEF.md`), `TASKS.md`, the tour (CLI: README's "The tour", as
`TOUR.md`), its door, its binary under `bin/` and the store.

| Binary | sha256 |
| --- | --- |
| `acq` (release) | `eb258f2e849dbe64c80bf68502f8a87f9298dd0a1ad83ebf8c628d8541122041` |
| `acq-mcp` (release) | `02cbf760dff336bcebf1db7381a8434860f38c89675760c51a70f170d1ae7b79` |
| `copy-as-store` (release; made `pristine/`) | `6878430c6d35a5fc290f0f16b34a747099b5017a820a8a97ad57d2ebe3ed8012` |

## The door's cut

Both doors print a result whole up to 60,000 bytes and past that its
first 3,000 characters, saying so, and mark the row `head` in
`calls.csv`. The cut is the door's, never the tool's: a fault that rests
on a `head` row is read against the whole output in the world's
`calls/` before it is the tool's.
