<!-- written during the run; call numbers are rows of calls.csv -->

# Report — the release seat, command line

## 1. Before touching anything

(Order note: I ran `acq --help` as call 1 before writing this section; what follows under "the README told me" is from the README alone, and call 1 is kept under "the tool told me".)

### What the brief told me
- `acq` is the CLI of Acquisition, a PoE inventory tool; a local daemon fetches from the game's API into a store on this machine (the brief).
- The world: binaries unpacked under W/bin, nothing on PATH; the store holds `GERWARIC` (≈50 MB copy, with prices, no session) and a second account I log in through the door's `login`; no keyring; the door starts the daemon with a 30-minute idle exit; every command is pointed at a mock (the brief).

### What the README told me
- **Can do:** download stash tabs and characters; search them offline; price items you want to sell; render forum shop pages that you paste (it never posts). Three binaries: `acq` (CLI), `acq-mcp` (MCP server over stdio), `acqd` (daemon that sends every request under GGG's rate limits). Keeps a league fresh via a policy → plan → apply cycle. Translates a trade-site search to a query without network. It does not value items.
- **Needs, for what:**
  - *Login:* `acq auth` (browser; `--no-browser` prints the link). "acq refuses to start a daemon against GGG without one." Without a keyring "a login lasts as long as the daemon that holds it".
  - *Daemon:* job verbs (`profile`, `leagues`, `characters`, `character`, `stashes`, `stash`, `refresh`, `submit`, `demo`) and `auth`, `jobs`, `status`, `cancel`, `daemon` talk to it; job verbs start it on demand; `jobs`/`status`/`daemon status` only observe and never start one. Idle exit 60 s by default (`ACQ_IDLE_SHUTDOWN`).
  - *Network:* only the job verbs (through the daemon) — rate-limited, "thirty every five minutes" on tabs, holds up to five minutes.
  - *Nothing:* store, search, pricing (`price set` included), `policy set`, shop and `reference` verbs are offline and need no daemon.
- **Before the first command** the README asks me to know: the rate limit is the clock; what needs the daemon; commands default to Standard without `--league`; where the data lives (`acq store status` prints the path); after an update the next job command replaces the daemon. Also: `--json` is "total" (failure `{"error":…}` exit 1, usage `"kind":"usage"` exit 2); `--account` or `ACQ_ACCOUNT` selects the account, and with several accounts a verb refuses and names them.
- **What I would not run without asking the person:** `acq auth logout` (ends their login); `acq refresh --all` / `--apply` against GGG (hundreds of rate-limited requests, an hour of their budget); `acq policy set` (replaces the whole policy, every realm — Known gaps); `acq store import` / `store rebuild` (rewrites the store); `acq price set/clear` on their real store and `acq cancel` of jobs they submitted; `acq daemon stop` / `reset-tripwire` while they have work running. Here the brief makes the world a safe rehearsal, so the tasks' writes are authorised.
- Pointers I could not follow: `AGENTS.md`, `MCP-REFERENCE.md`, `CLI-REFERENCE.md`, `LIVE-TESTING.md`, `CONTEXT.md`, `decisions/*.md`, the source files in the Knobs table.

### What the tool told me
- `acq --help` (call 1): the verb list matches the README's tour, plus `dash` (TUI), `help`, and global `--json` / `--account`. It adds: `auth` completes only once its own profile job lands the account uuid; `characters`' first use queues a visible `probe` job (one HEAD); `jobs` with no daemon answers from the ledger on disk; `cancel`/`set-priority` never start a daemon; `version --json` shape.

### The world as I found it (read-only commands)
- Accounts: one, `GERWARIC#7694`, "last login 2h ago, not persisted, store 54.4 MB" (call 2). The second account does not exist yet.
- Store: `W/store/mock/GERWARIC_7694.db (54.4 MB)`; responses 1133, league list never fetched, characters 65, tabs 3444 (every realm and league), items 22721 (+6 removed), events 22751; 8 PoE2 item-granted skills left in place; 4 refused bodies (call 3). The path has a `mock/` segment — the store is split by provider, which the README's "Its data is kept apart from the real" hints at but does not name.
- Daemon: "daemon is not running; no queue on disk" (call 4). `auth status`: no live session; GERWARIC "not persisted — `acq auth` logs in again" (call 5).
- Tabs in Standard: 3347 (405 fetched, 2926 never fetched, 16 folders), 20231 items; largest tabs fetched 41d ago (call 6). Characters: 65 — 50 fetched (10 with empty bodies), 15 never fetched; 1389 items; pc and poe2 realms (call 7).
- Nothing was changed by calls 2–7, as far as their help and the README say (all store reads or observers).

## 2. The tasks

### T1 — the install
**What I would tell the person:** "`acq` runs and says it is `1.0.0-beta.1` (contract `97a12b04add0`), and it finds its daemon `acqd` right beside it in the same directory (8,326,400 bytes). That is all I can confirm from the tool. I can't see from here whether `acq-mcp`, the licences, the third-party notices or `SHA256SUMS` came out of the archive, and I didn't check the archive's checksum — nothing in `acq` reports those. On your Mac you still need to put the directory on your `PATH` (or symlink `acq` into one); if you downloaded the archive with a browser rather than `curl`, clear the quarantine with `xattr -dr com.apple.quarantine <directory>`. A first job command will also need a login (`acq auth`)."

**Commands:** 8 (`version`), 9 (`version --json`), 10 (`--version`), 11 (`version --help`); and calls 1–7 ran, so the binary executes on this Mac.

**Right?** I hold it right for what it covers: `acqd` path is `W/bin/acqd` (call 8), i.e. beside `acq`. What would show it wrong: a job command failing to start the daemon (T2 tests that). `version` only reports the *candidate* acqd by path/length/mtime — "no hash" (call 11) — so it cannot tell me the acqd is the one the archive shipped.

**Not checkable from here:** `SHA256SUMS` verification (no `shasum` allowed against the tool, and the README gives no verify command at all); whether `acq-mcp` exists (no `acq` verb mentions it; `version` names only acqd); licences/notices; code signature / quarantine state (the README says "signed with a Developer ID and not notarized" but gives no way to check); that the symlink is followed (README: "the link is followed") — I have no PATH setup to try.

**Where the tool or README misled/stopped me:** the README's Install section tells you to verify nothing — it lists `SHA256SUMS` in the archive but never says to run `shasum -a 256 -c SHA256SUMS`. Human `version` (call 8) omits the provider; only `--json` (call 9) shows `"provider":"mock"` — a person cannot see from the human line that they are on the mock.

### T2 — log in, and a daemon for an agent
**What I would tell the person:** "You're logged in as `seat` (realm pc, guild Playground, uuid `00000000-0000-4000-8000-00000035ce05`), on the mock. You have two leagues, Standard and Hardcore. Two characters, both in Standard: StashHoarder (Scion, 97) and MuleQuadTab (Witch, 12, says the list). Standard has six top-level tabs — Currency, Dump, a folder 'Old leagues' holding Old 1 and Old 2, Uniques, Maps, and Maps (empty). MuleQuadTab carries a helm, Starkonja's Head (Silken Hood) with a Determination gem in it, plus four placeholder items 'Fake Item 0–3'; the Dump tab holds five placeholder items 'Fake Item 0–4'. I then gave you a daemon that stays up an hour with nothing to do: since there's no keyring, stopping the old one logged you out, and logging in again with `ACQ_IDLE_SHUTDOWN=3600` started a new daemon (pid 96336, was 94693) that says 'idle exit after 3600s'."

**Commands:** login 12; 13 (`auth status`), 14 (`profile`), 15 (`leagues`), 16 (`characters`), 17 (`stashes`), 18 (`character MuleQuadTab`), 19 (`stash dump`); 20, 21 (`daemon status`, `daemon --help`); 22 (`daemon stop`), 23 (`auth status`); login 24 with `ACQ_IDLE_SHUTDOWN=3600`; 25 (`daemon status`), 26 (`auth status`).

**Right?** Yes for the daemon: call 20 shows pid 94693 "idle exit after 1800s" (the door's); call 22 stopped it and said "logged out with it: seat — not in the keyring…"; call 25 shows pid 96336, "up 0s", "idle exit after 3600s"; call 26 shows the session live in it. The character data I hold as what the mock sent, but it contradicts itself: the list (call 16) says MuleQuadTab is a **Witch, level 12**; its fetch (call 18) says **Scion, level 97**; and the helm is `frameType 3` (unique) but `frameTypeId "rare"` (call 18). That is the mock's data, not something the CLI made — but the CLI passes it on without comment.

**Where the tool or README stopped/misled me:**
- The README's recipe is right for a no-keyring world, but it leaves out the step that matters: "start it … while none is running" — with a daemon already up (the one the login started) you must `acq daemon stop` first, and the README files `daemon` under "debugging"/"normal use never needs manual lifecycle" (call 21). Stopping it logs you out (call 22), which the README does say ("the login went with the last daemon") — the tool says it plainly too.
- `auth --no-browser --json` (calls 12, 24) printed **two** JSON documents on stdout (the `authorize_url` line, then the result), against "one document on stdout" (call 1's `--json` help). Expected for a two-step flow, but a program parsing stdout as one document will fail.
- Job verbs print the API's JSON even without `--json` (calls 14–19): there is no human form of `profile`, `characters`, `stashes`, `stash`. The README says "prints GGG's JSON" — honest, but a person asking "what tabs do I have" gets 1957 bytes of JSON (call 17).
- Job ids skip (2, 3, 4, 6, 8, 10): the hidden ones are presumably the `probe` jobs call 1's help mentions; nothing in the output shows them.
- `access_expires_in_seconds: 59` (calls 12, 24) looks alarming; `auth status` explains "renewed on use; the login outlasts it" (call 13) — good.

### T3 — what I have (GERWARIC)
**What I would tell the person:** "You have **22,721** live items (plus 6 removed) across 3,444 tabs and 65 characters. By league: 21,311 in Standard (pc), 1,312 in Allflame (pc), 98 in Standard on PoE2. 21,332 are in stash tabs and 1,389 on characters (966 equipped, 165 in inventories, 137 jewels, 23 'guardian', and on PoE2 56 equipped plus 42 skill gems). Standard has 3,347 tabs, but only 405 of them have ever been fetched; the 2,926 that never were are nearly all map and unique substashes — 2,489 map tabs and 437 unique tabs — so I can't tell you what's in them. Allflame has 97 tabs, all fetched except a folder. Of the 65 characters, 50 are fetched (10 of those came back empty) and 15 never were. The data is old: the big tabs were fetched 41 days ago, characters 38–47 days ago, the tab list itself was last seen 47 days ago, and the newest thing anywhere is 4 days old. Nothing changed in the last 24 hours. The store refused 4 character bodies, all PoE2 (gerwarsmash, boomsplam, flamdomrando, pilatesinstructor), 47 days ago, each because an item socketed in their gear had no `id`; it keeps them as evidence. Your league list was never fetched."

**Commands:** 27 (refused: two accounts), 28 (`store status`), 29 (`store --help`), 30 (`store refused`), 31 (`store events`), 32 (`tabs --help`), 6 and 36 (`tabs` Standard / Allflame), 7 (`store characters`), 33 (`search --help`), 34 (failed: `realm` is not a count key), 35 (`search --realm all --count league,container`), 37 (`--count tab`).

**Right?** The counts agree with each other, which is why I trust them: tabs 20,231 (call 6) + 1,101 (call 36) = 21,332 = the `items · pc` container (call 35); character items 1,389 (call 7) = the six character containers (call 35) = `tab (none)` (call 37); never-fetched 2,941 = 2,489 + 437 + 15 (call 35) and matches call 6's per-type figures. What I could not pin down: **which location is "newest 4d ago"** (calls 35, 37) — no tab or character I listed is younger than 38d; finding it would need `tabs --expand` over 3,347 rows, over the door's 60 KB limit, or a sort by fetch age, which I did not find.

**Where the tool stopped/misled me:**
- After the second login every store verb needs `--account` (call 27): the refusal is right and names both accounts — good. But the brief's "on GERWARIC's store, no daemon involved" is the store's, while the error appears only because a *login* elsewhere added an account.
- `--realm all` is required for a two-realm store, yet `realm` is not a count key (call 34); the refusal lists the keys — recoverable, and `league` counts show the realm anyway (call 35).
- `tabs` defaults to `--league Standard` and `--realm pc`; nothing lists which leagues hold tabs — I learned Allflame from `search --count league` (call 35), not from `tabs`.

### T4 — find things (GERWARIC)
**What I would tell the person:**
- (a) "Yes — **187** items with a six-link, out of everything fetched: 181 in Standard and 6 in Allflame; 161 sit in stash tabs, 24 are worn and 2 are in a character's inventory; 101 rare, 76 unique, 7 magic, 3 normal. For example the Tabula Rasa on BatteryStaple and Shavronne's Wrappings on gerwarwitch. 2,941 map/unique substashes and characters were never fetched, so there may be more."
- (b) "**195** rare rings have `+# to maximum Life` and at least two resistances. The best by total resistance is *Rapture Eye* Sapphire Ring in the Standard tab '3.13 Random Build Gear' (+54 life, 146 % total res). 7 more PoE2 items could not be decided — the tool has no class or totals table for PoE2. Note 'two resistances' is counted the trade site's way: '+% to all Elemental Resistances' counts as fire, cold and lightning."
- (c) "Per league and rarity: Standard (pc) — 5,615 rare, 2,850 normal, 2,819 unique, 2,334 magic, 7,693 with no rarity (currency, gems, cards…); Allflame — 154 rare, 276 normal, 257 unique, 64 magic, 561 none; Standard (PoE2) — 16 rare, 13 normal, 1 unique, 25 magic, 43 none."
- (d) "Rapture Eye, Sapphire Ring, rare, item level 74, needs level 57; Standard, tab '3.13 Random Build Gear', at x 11, y 7, fetched 41 days ago; class Rings, trade category accessory.ring; identified; implicit +29 % cold res; explicits +5 ES, +43 % fire, +35 % cold, +39 % lightning, +54 life (crafted); not priced; nothing unreadable. It has its raw GGG body too, which adds the icon URL and `verified: false`."

**Commands:** 38 (`--describe fields,…`), 41 (bare `--describe`, 48,571 bytes — **cut by my harness at ~2 KB**, not by the door; I did not open the file it saved), 42 (unknown entry → the list of entry names), 43 (`--describe pseudo.count_res,pseudo.total_life,class`), 39 and 40 (a), 44 (b), 45 (c), 46 and 47 (d).

**Right?** (a) and (c) I hold right; their totals reconcile (calls 40, 45 sum to 187 and 22,721). (b) depends on reading "life" as the `# to maximum Life` line — `pseudo.total_life` would add half of Strength, a different answer — and on the trade site's notion of a resistance count (call 43). What would show it wrong: a ring with life only from Strength, which this query leaves out by design. Searching `--realm all` brought 7 PoE2 'undecided' along (call 44); `--realm pc` would have been the cleaner question.

**Where the tool stopped/misled me:** the query language is learnable from the tool alone, and well — but only because a wrong `--describe` name prints the full list of entry names (call 42); the bare index is 47 KB (call 41), too big for an agent's view. The `more` line's `--next` token is 600+ characters (calls 39, 44). The row form prints the lines the query *touched*, not the item — fine, `show` gives the rest.

### T5 — prices (GERWARIC)
**What I would tell the person:** "In Standard, 531 of your 21,311 items are priced: 489 by notes or tab names in the game, 89 by hand through Acquisition (39 decided only by hand, 50 where both sides speak). Most of the in-game prices come from public tabs named `~price 4554 chaos`, `~price 8888 chaos`, `~price 2222 chaos` and two bulk-ratio tabs. By hand you have 16 price rows, but only 5 apply to Standard; the other 11 are for other leagues. What might be wrong: (1) **50 conflicts**, all in your public tab ACQUISITION-PRICE-TEST, which you priced at 5 chaos by hand, but each item there has its own game note (`~price 999 chaos`, `~price 999 mirror`, `~skip`, …), and the note wins because it's more specific; (2) 1 invalid note, which has no effect, so the tab's price applies — the tool counts it but I didn't find which item it is; (3) 3 items are `~skip`; (4) 1,395 items carry price text the index can't use (notes on characters, names of non-public tabs), and nothing lists them for sale. Then I priced the tab **Delve** (30 items) at **5 chaos** each, priced the ring *Rapture Eye* at **2 divine**, and replaced that with **3 divine**, only if it was still revision 1. It was, so the change went through as revision 2."

**Commands:** 48 (`price --help`), 49–50 (`price status`, `--expand`), 51 (`price list --help`), 52 (conflicts), 53 (`price show` of the conflicting tab), 54 (`price list`), 55 (refused: `--limit` is not a `price list` flag, exit 2), 56 (conflicts `--expand`), 57 (`tabs --type DelveStash`), 58 (`price set --help`), 59–60 (read before write), 61 (tab → 5 chaos), 62 (item → 2 divine), 63 (read revision: 1), 64 (item → 3 divine `--if-revision 1`), 65–66 (read back both).

**What each write said, and what its receipt lets me undo:**
- call 61: `tab/pc/2f01a16e65: 5 chaos (revision 1), was unset` and a `next:` to read it. It gives **no undo command**, though `price set --help` (call 58) says it "Prints what it replaced, how to put that back". From "was unset" the undo is `price clear`, but I have to work that out myself.
- call 62: the same form, `2 divine (revision 1), was unset`; again no undo command printed.
- call 64: `3 divine (revision 2), was 2 divine (revision 1)` and the undo `acq price set item/… exact 2 divine --if-revision 2`, which is exact and guarded. **But the printed undo has no `--account`**, and with two accounts on disk every store verb refuses without one (call 27). Pasted as printed, it would fail (tested in T6).
- Read back: call 65 `3 divine (own row, revision 2, set 0s ago)`; call 66 `5 chaos (own row, revision 1…)`, `30 items — 30 by hand only`.

**Another program changing a price between my read and my write:** `--if-revision N` (call 58) refuses a write unless the row is still revision N, and the refusal names the current revision. Without the flag, a write "replaces whatever is stored", but "a write racing in between still conflicts rather than being clobbered" (call 58). I would see the change as a revision higher than the one I read (`price show` prints revision and age, call 63). T7 tests the refusal.

**Right?** Yes: both read back as written (calls 65, 66). What would show they're wrong: a shop render without them (T6).

**Where the tool stopped/misled me:** the conflict wording reads backwards to me: the list's `conflict  game  5 chaos  999 chaos` (call 56) puts the hand price in the column before the game price, with no column headers. `price list` has no `--limit`, unlike `search` (call 55). I couldn't find a way to list the "1 invalid notes" or the "1395 items carry price text the index cannot see" (call 49); no `next:` line points to them.

### T6 — the shop (GERWARIC)
**What I would tell the person:** "Your shop fits on **one page**, 4,596 characters holding 62 items, well under the forum's 50,000 limit. It's a single `[spoiler="Shop Post 1 of 1 (62 items)"]` block with one inner spoiler per price. The **Delve** tab's 30 items are under `~price 5 chaos`, as `location="Stash11"` links from x 0 to 35, y 0, and that spoiler also holds 29 items of ACQUISITION-PRICE-TEST (`Stash57`). The **Rapture Eye** ring is the only item under `~price 3 divine`, at `Stash267` x 11, y 7, which matches where the store has it. Two more hand prices make their own spoilers (`~price 2 chaos`, `~b/o 1.5 divine`). Left out: the 489 items the game already lists at its own price (a page mustn't contradict the site), the 3 the game marks `~skip`, and the 3 you skipped by hand (Test๏Character's). **Blocked:** 5 maps priced 2 divine in a map substash, because the website gives no link for a substash item. The tool also warns that the Delve tab and the ring's tab aren't in your sync policy, that 31 items on the page were last seen up to 4 days ago (past the policy's 300 s window), and that links use tab positions, so if a tab was reordered since, a link points at the wrong slot. A refresh (`acq refresh --plan`, 5 requests) would fix that. **Next:** paste page 1 into your shop thread on the forum and preview it before posting. The tool never posts. I've now taken off the ring's price and the Delve tab's price; both read back as unpriced, the same as before I started."

**Commands:** 67 (`shop render --help`), 68 (`shop render`), 69 (`--expand`: **3,554,399 bytes, cut by the door to its first 3,000 characters**. That cost me the rest of the policy table, from `tab_unlisted` on, and every item left off), 70 (call 64's printed undo pasted as printed: refused, "several accounts are known"), 71 (`price clear --help`), 72 (clear item `--if-revision 2`), 73 (clear tab `--if-revision 1`), 74–75 (read back), 76 (`price status`, same as call 49 to the character).

**Right?** I hold it right. The counts reconcile: 30 Delve + 29 + 1 + 1 + 1 = 62 links (call 68), and 31 hand-only items in ACQUISITION-PRICE-TEST (call 53) = 29 + `2 chaos` + `1.5 divine`. The ring's x 11, y 7 matches call 46. What would prove it wrong: `Stash11` / `Stash267` are the tab's *rank* "among the tabs the website lists" (call 69), computed from a stash listing 4 days old (call 50) and item positions 41 days old. Only the game can confirm them, and I can't check that here. Cleanup: calls 74 and 75 are the same as calls 59 and 60, and call 76 matches call 49.

**Where the tool stopped/misled me:**
- `shop render --expand` writes **3.5 MB** (call 69). The help says "every item left off the page with its cell" (call 67), and that includes 20,749 "nothing applies" items. Nobody can read that much, and the door cut it.
- The undo commands printed by `price set` (call 64) and `price clear` (calls 72, 73) leave out `--account`, so pasted as printed they fail when two accounts are known (call 70). The refusal names both accounts, so the fix is easy, but the receipt isn't the runnable command the help promises.
- The clear receipts' undo (`price set … exact 3 divine`, call 72) **drops the revision guard** that the set receipt had (call 64 printed `--if-revision 2`).

### T7 — for a program
**What I would tell the person:** "Mostly yes. Every command I re-ran with `--json` printed one JSON document on stdout and nothing on stderr. Job verbs wrap the API's answer as `{account, job_id, outcome, payload}`. Store, search and price verbs print their own structured document. Failures came as JSON on stdout with exit 1, and a usage error with `kind: usage` and exit 2, as the README says. Three caveats. (1) The shop as JSON is **7.4 MB** without `--expand`, because it carries every unpriced item; a program wants `--page N --json`, which is small. (2) Not every error carries a `kind`: the stale-revision refusal and the failed job don't. (3) The current revision appears only inside the error *sentence*, so a program has to parse English to retry. `auth --no-browser --json` printed two documents (T2)."

**Reruns (five, one per task):**
| task | text call | JSON call | one document? |
| --- | --- | --- | --- |
| T2 `characters` | 16 | 77 | yes: `{account, job_id: 13, outcome: "success", payload}`; progress lines on stderr vanish under `--json` |
| T3 `store status` | 28 | 78 | yes: flat object with `bytes: 54382592`, `refused_bodies: 4`, plus fields text never shows (`unlifted_items`, `withheld_responses`, `withheld_items`) |
| T4 `search links=6 --count league` | 40 | 79 | yes. It carries more than the text: `scope.leagues` lists 8 leagues, and `never_fetched_by_type` gives fetched counts too |
| T5 `price show item/…` | 75 | 80 | yes (`schema: 3`, chain item → tab → tab `0a2feafd9c`, `index: 281`) |
| T6 `shop render` | 68 | 81 / 82 | 81: **7,478,732 bytes, cut by the door at 3,000 characters**, so I cannot confirm it is one well-formed document. 82 (`--page 1 --json`): yes, `{number, of, chars, items, text}`, 2,763 bytes |

**Failures on purpose:**
| failure | text | JSON | told me what to do next? |
| --- | --- | --- | --- |
| wrong account `GERWARIK` | 83: exit 1, stderr `no account matches "GERWARIK" (see \`acq accounts\`)` | 84: exit 1, `{"error":…,"kind":"unknown_account"}` | yes: points to `acq accounts`. It doesn't suggest the near match `GERWARIC` |
| item id that does not exist (last two hex digits changed) | 85: exit 1, `no item … in this store: an id is given whole, as an answer printed it (22721 live items, every realm)` | 86: exit 1, `kind: item_not_found` | partly: it explains how to give an id but not where to find one (`search`) |
| price write naming the wrong revision (4; the row is 1) | 87: exit 1, `annotation tab/pc/03cb479c65/buyout is at revision 1 (re-read and retry)` | 88: exit 1, `{"error": …}`, **no `kind`**, no `current_revision` field | yes in words: "re-read and retry", and it names revision 1. But "annotation … /buyout" is an internal name the user never typed (they typed `tab/pc/03cb479c65`) |
| extra: stash id that does not exist (job verb) | 89: exit 1, `job 15 failed: GET /stash/Standard/nosuchtab returned 404 …; body: {"error":"no such stash"}` | 90: exit 1, `{account, error, job_id: 17, outcome: "failure"}`, no `kind` | no: it doesn't point to `acq stashes`. It does say which league it looked in (Standard) |
| extra: usage error | 55: exit 2, clap usage | 91: exit 2, `kind: usage` | yes |

**Right?** I hold it right for what I saw. What the door hid: whether call 81 is valid JSON to its end.

**Where the tool stopped/misled me:** the `--json` help (call 1) says a failure is `{"error", "kind"}`, but calls 88 and 90 have no `kind`. The JSON form of `shop render` is unusable whole (call 81).

### T8 — when things go wrong
**What I would tell the person:** "I stopped the daemon. Stopping it logged `seat` out, because without a keyring the session lives only in the daemon. With no daemon: **store reads work as before** (`tabs` answered normally). **Observers work and don't start one.** `jobs` says the daemon isn't running and nothing waits on disk; `status 13` shows job 13 from the ledger on disk; `result 13` still gives the finished job's payload from disk; `daemon status` says it isn't running and names the last log. **A command that needs the API** (`profile`) failed with `not logged in — run \`acq auth\``. It did not say so, but it **started a new daemon** anyway, which then sat idle with no session (pid 9679, idle exit 1800 s). I stopped that one too. Versions: `acq` is `1.0.0-beta.1`, contract `97a12b04add0`; the daemon reports the same version and contract, and in JSON that the file it runs is the same file `acq` would start (`artifact_relation: same_file`, sha256 `45610ff42e25…`). Logs: `W/logs/26ebc7acecb5/mock/daemon.log`, with a send journal `sends.jsonl` beside it. The daemon is stopped now."

**Commands:** 92 (`daemon status`: pid 96336, up 414 s, idle 3600 s), 93 (`daemon stop`: "logged out with it: seat"), 94 (`daemon status`: not running, last log), 95 (`tabs`, store read, exit 0), 96 (`jobs`), 97 (`status 13` from the ledger), 98 (`daemon status`: still none, so the observers spawned nothing), 99 (`result 13` from disk), 100 (`profile`: exit 1 "not logged in"), 101 (`daemon status`: **a new daemon, pid 9679, up 3 s**), 102 (`profile --json`: `kind: not_logged_in`), 103 (`daemon status --json`: version, contract, artifact, log), 104 (`daemon stop`), 105 (not running), 106 (`dash`: exit 1 "daemon is not running (it spawns on demand for job commands)"; it never got as far as needing a terminal), 107 (still not running). Versions also from T1 calls 8–10.

**Right?** I hold it right. The daemon is left stopped (calls 105, 107). One oddity I can't explain: the ledger's "written N s ago" is 26 s in call 94 and 30 s in call 98, but 76 s for job 13 in call 97, between those two calls.

**Where the tool stopped/misled me:**
- `profile` with no session **spawned a daemon and left it running** for 30 minutes while telling me only "not logged in" (calls 100, 101). The README's "acq refuses to start a daemon against GGG without one" says nothing about the mock, and here the daemon started anyway.
- `dash`'s refusal (call 106) says the daemon "spawns on demand for job commands", but `dash` doesn't spawn one, and it doesn't say what to run instead.
- The `logs/<hash>/mock/` path isn't in the README. Its Knobs table says only "the platform log dir", which I could only learn from `daemon status` (calls 20, 94). `ACQ_LOG_DIR` must be set in this world, but nothing printed says so.

## 3. Faults (worst first)

1. **Undo commands that can't be run as printed.** `price set` and `price clear` print the command that "puts it back", but without `--account` (calls 64, 72, 73). With two accounts on disk that command is refused (call 70). Also: the set on an unset target prints no undo at all, though the help promises "how to put that back" (calls 58, 61, 62), and the clear's undo drops the `--if-revision` guard that the set's undo had (calls 64 vs 72). *Should:* print the command whole, account and guard included, and for "was unset" print the `price clear … --if-revision 1`. *Refusal (call 70):* warranted (the account really is ambiguous); recoverable (it names both accounts); but the tool itself wrote the wrong command. **Both** a person and an agent hit this as soon as a second account is logged in. An agent pasting receipts hits it every time.
2. **A job verb with no session starts a daemon and doesn't say so.** `profile` exited 1 "not logged in — run `acq auth`" (call 100), and a fresh daemon was then running, idle for 1800 s with no session (call 101). *Should:* check for a session before spawning, or at least say a daemon was started. *Refusal:* warranted, and it says how to recover. The side effect is the fault. **Both**, though an agent is more likely to probe this way.
3. **`shop render --json` is 7.4 MB, and `--expand` is 3.5 MB** (calls 81, 69), because both carry every item that isn't priced (20,749/20,780). *Should:* leave the unpriced items out, or count them only, and keep "every item left off" behind a flag that pages. **Agent mostly**, since a person reading text uses the 6 KB default. Both outputs were cut by the door.
4. **JSON errors are not uniform.** The `--json` help promises `{"error", "kind"}` (call 1), but the stale-revision refusal has no `kind` (call 88) and neither does a failed job (call 90). The current revision appears only in the sentence (calls 87, 88). *Refusal (87/88):* warranted, and the text says "re-read and retry" and names revision 1, so a person can recover. A program has to parse prose. The wording uses an internal name, `annotation tab/pc/03cb479c65/buyout`. **Agent only.**
5. **`auth --no-browser --json` prints two JSON documents** (calls 12, 24), against "one document on stdout". **Agent only.**
6. **The README's long-idle recipe leaves out "stop the running daemon first"**, and stopping it logs you out (calls 20–25). The README files `daemon stop` under debugging, so a reader doesn't expect to use it. **Both.**
7. **A stash id that doesn't exist** returns the raw HTTP line, `GET /stash/Standard/nosuchtab returned 404 …` (calls 89, 90), with no pointer to `acq stashes` and no `kind`. *Refusal:* warranted; recovery not said. **Both.**
8. **Job verbs have no human output** (calls 14–19): `stashes` prints 1,957 bytes of JSON to answer "what tabs do I have". **Person.**
9. **The human `version` line hides the provider** (call 8; only call 9's JSON says `mock`). A person can't tell from it whether they're on the real API. **Person.**
10. **`price list` columns have no headers** (calls 54, 56): in `conflict game 5 chaos 999 chaos` you have to infer which column is the hand price and which the game price. **Both.**
11. Minor: job ids skip (2, 3, 4, 6, 8, 10; 15, 17) with nothing shown for the jobs in between (calls 14–19, 89–90). `dash` with no daemon says it "spawns on demand for job commands" but doesn't say what to run (call 106). The mock's character list and character body disagree (Witch 12 vs Scion 97, calls 16, 18); that is the mock's data, not the CLI's.

Refusals that were warranted and recoverable: call 27 (names both accounts), 34 (lists the keys), 42 (lists every entry name; this is how I learned the language), 55/91 (usage, `kind: usage`, exit 2), 83/84 (`see acq accounts`; no near-match suggestion), 85/86 (says how to give an id, but not where to find one).

## 4. What the README and the help did not say

- **That the long-idle recipe needs the running daemon stopped first, with `acq daemon stop`.** Expected in README "Your agent". Learned at calls 20–22.
- **That a job verb with no session spawns a daemon anyway** (on the mock at least). Expected in README "Before the first command" / "Log in". Learned at call 101.
- **That printed undo commands leave out the account.** Expected in `price set --help`. Learned at call 70.
- **How to verify the archive** (`SHA256SUMS`). README "Install" lists the file but gives no step for it.
- **Where the logs are**: `logs/<hash>/<provider>/daemon.log` plus `sends.jsonl`. README Knobs says "the platform log dir" but never names it. Learned at call 20.
- **That the store is split by provider** (`store/mock/<account>.db`). README "Your data stays" names only the directory. Learned at call 3.
- **Which leagues hold data.** `tabs --help` defaults to Standard and doesn't say how to list leagues offline (`acq leagues` needs the API). Learned at calls 35 and 79 (`scope.leagues`).
- **That there is a `pseudo.count_res`, and that "all Elemental Resistances" counts as three.** Found only by an unknown `--describe` name (calls 42, 43). The `search --help` examples don't lead there.
- **That `price list` has no `--limit`.** Learned at call 55.
- **That `shop render --json` carries every unpriced item.** Expected in `shop render --help`. Learned at call 81.
- **How to find "1 invalid notes" and the "1395 items carry price text the index cannot see"** (call 49). Nothing says how to list them. I never found out.
- **Which location is "newest 4d ago"** (call 35). Call 50 suggests it is the stash *listing* (response 1128, 4d ago), not any item location; the scope line doesn't say which.

The README said things I never needed: the Windows and Linux install notes, PowerShell, the MCP config block, `cargo build`, most of the Knobs table (`ACQ_CASES`, `ACQ_CODESIGN_IDENTITY`, `ACQ_KEYRING_ROUNDTRIP`, rails, `ACQ_MOCK_DEGRADED_HEAD`), the policy/plan cycle, `store import`, `--trade`, and "Known gaps" on `policy set` and the mock's 429s. "Known gaps" on PoE2 *was* useful: it explained the 7 undecided in call 44.

## 5. The tool's own words

**Right and useful:** "logged out with it: seat — not in the keyring, so the session lived in that daemon alone; `acq auth` logs in again" (calls 22, 93). "access token valid for ~54s (renewed on use; the login outlasts it)" (call 13). The `next:` lines on almost every read (calls 6, 49, 53, 61). "2941 locations were never fetched: nothing here says what they hold" (call 35). `price show`'s cause chain, "the note is more specific than the row on the tab" (call 56). The shop's counted omissions, each with a reason (calls 68, 69). The poe2 undecided line: "no class table for realm poe2 … a refresh will not help; a reference update may" (call 44).

**Wrong or misleading:** `price set --help` "Prints what it replaced, how to put that back" (call 58, vs calls 61, 62). `--json` "a failure included (`{"error", "kind"}`)" (call 1, vs calls 88, 90). `dash`: "it spawns on demand for job commands" (call 106) reads as if `dash` would spawn one. `profile`'s "not logged in" (call 100) leaves out the daemon it just started. "annotation tab/pc/…/buyout" (call 87): a name I never typed.

**Read and never used:** `--explain`, `--context`, `--request`/`--print-request`, `--membership`, `--trade`, the `dash` description, `set-priority`, `submit`, `demo`, the long `daemon status --help` paragraph on `artifact_relation` (I used only what it printed).

**Words I didn't understand:** "contract" (a digest of crates, call 11; I understand what it is but not why a user needs it), "unlifted_items", "withheld_responses/items" (call 78), "intent 33" → "intent 38" in `basis` (calls 35, 79; perhaps a count of price rows written), "derivation 12", "facts v8", "granted … `_granted` … never rows" (call 3), the `guardian` container (call 35), "lacked" vs "failed" in term counts (call 39), "rails" (call 20), "positions: … seen before the current stash listing" (call 68; I understood it only after the policy table's "rank among the tabs the website lists").

## 6. What I would ask of the tool

- Print every undo and `next:` command with the `--account` it ran as, and with its revision guard (calls 64, 70, 72).
- For a set on an unset target, print the `price clear` that undoes it (calls 61, 62).
- Don't spawn a daemon for a job verb when no session exists, or say that you did (calls 100, 101).
- `shop render --json`: leave the unpriced items out unless asked (call 81).
- Give every JSON error a `kind`, and give the revision refusal a `current_revision` (calls 88, 90).
- `auth --no-browser --json`: one document, or say in the help that it prints two lines (call 12).
- A human summary for `stashes`/`characters`/`profile` (call 17).
- Show the provider on the human `version` line (call 8).
- Point `stash <bad id>` to `acq stashes` (call 89).
- Headers on `price list` rows (call 56).
- A way to list the invalid notes and the "price text the index cannot see" (call 49).
- An offline list of the leagues the store holds (calls 35, 79).
- Suggest the near match for a mistyped account (call 83).

## 7. Left out

- **Not tried:** `refresh` (`--tabs`, `--plan`, `--apply`), `policy show`/`set`, `submit`, `demo`, `cancel`, `set-priority`, `store import`/`rebuild`, `store refused <id>`, `store events --expand`, `search --trade`, `--explain`, `--next`, `reference currency`, `auth check`, `auth logout`, `acq-mcp` (the brief allows `acq` only), `ACQ_NO_SPAWN`, `price list --covered-by`, a template for `shop render`.
- **Files I opened beyond `BRIEF.md` and `README.md`:** none. My harness saved call 41's 47.5 KB output to a file of its own (it showed me only the first ~2 KB); I did not open that file.
- **Files I wrote:** this report, through the door's `report`. Sections 3–7 went through the draft `W/drafts/report-3-7.md` and `report --from`, because my harness refused that part's heredoc. Nothing under `W/work/`.
- **The one cut I would most want back:** the rest of call 69 (`shop render --expand`), the remaining policy-table rows (`tab_unlisted` onward) and the five blocked maps' cells. That is where the shop explains itself, and the door's 3,000-character cut fell in the middle of it.
