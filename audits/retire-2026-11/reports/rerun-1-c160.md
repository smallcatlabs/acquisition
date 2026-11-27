## Rows
| id | verdict | clause | the new thing that breaks it | what fails, or nothing | conf |
| C160 | caught | 1 | a new tokio timer in the daemon (`sleep`, `timeout`, `interval` or another spelling) that does not sleep through the daemon's `Clock` | check 12 at tools/arch-check.sh:891 reads the daemon's tree and refuses any tokio timer outside `impl Clock for SystemClock` and tail test modules; tools/arch-check-breakers.sh:432-437 confirms it rejects the bad spellings. Clauses 2-5 are bound: the park hook's accounting and its verdict words are held by door.rs:1715-1760, :1773, :2069-2118 and dispatch.rs:198-210 | high |

## Noticed
- This row was missing from my first report, though I had already judged C160 when I wrote it. The first reader cites check 12 as starting at line 892; its header comment is at tools/arch-check.sh:891.
