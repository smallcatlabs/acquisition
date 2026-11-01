<!-- written during the run; call numbers are rows of calls.csv -->

## 1. Before touching anything

**What the brief told me (the brief):** the tool is `acq`, an inventory tool for Path of Exile; the store holds `GERWARIC#7694` (a copy of a real collection, ~50 MB, no session) and I log in a second account through the door; every command reaches a mock API; there is no keyring.

**What the README told me:**
- *Can do:* download stash tabs and characters to the machine; search them offline (own query language, trade-site URL translation); set prices; render forum shop pages to paste; keep a league fresh with a sync policy plus plan/apply; an MCP server with 25 tools mirroring the CLI.
- *Needs:* a login (browser, `acq auth`) for anything that talks to GGG; the daemon `acqd` for job verbs (profile, leagues, characters, character, stashes, stash, refresh, submit) and for `auth`, `jobs`, `status`, `cancel`, `daemon`; it starts on demand. The store, search, pricing, policy and shop verbs need no daemon and no network. "Without a keyring, a login lasts as long as the daemon that holds it" (Log in) — so here `daemon stop` ends the session.
- *Before the first command* ("Before the first command"): the rate limit is the clock (30 tab requests / 5 min seen so far; a first sync of a few hundred tabs takes most of an hour); Standard is the default league unless `--league` is named; `acq leagues` lists leagues; a second login makes every verb need `--account`.
- *When a new league opens* ("A new league"): `acq leagues` names it; add it to the policy *by writing the realm back with every league it keeps* (naming only the new league drops Standard); then `refresh --plan --league <name>` and `--apply --league <name>` until the plan says nothing to do; first plan = listings, later plans = every tab; read each plan's work and ceiling before applying.
- *Pointers I cannot follow:* `CLI-REFERENCE.md`, `AGENTS.md`, the `decisions/<area>.md` files, `SECURITY.md`, `CONTRIBUTING.md`, and the source files named in the Knobs table. None turned out to matter before T1.
- *What I would not run without asking the person:* `acq auth logout`; `acq daemon stop` (it ends a keyring-less login, per Log in); `acq daemon reset-tripwire`; `acq policy set --replace` or any policy write that names a realm without every league it keeps; `acq refresh --all` / `--apply` against real GGG on a big account (it spends an hour of rate limit); `acq store import` / `store rebuild` (rewrite the store); `acq cancel` of a job I did not submit; `price set`/`price clear` on the real store; posting `diagnose` output anywhere.

**What the tool told me (world as found, no changes):**
- `acq --help` (call 1): 30 verbs, including `dash` (a TUI) and `web` (loopback page), neither of which I can use.
- One account on disk: `GERWARIC#7694`, last login 16d ago, "not persisted", store 54.4 MB (call 2).
- No daemon, so no live session; GERWARIC "not persisted — `acq auth` logs in again" (call 3). "daemon is not running; no queue on disk" (call 4).
- Store (call 5): `/…/store/mock/GERWARIC_7694.db (54.4 MB)`; responses 1126, league list never fetched, characters 65, tabs 3444 (every realm and league), items 22721 (+6 removed), events 22751; 8 granted skill entries not counted as items; **refused: 4 bodies**. Note the path is under `store/mock`, i.e. this is the stand-in provider's store.
- No sync policy set; the tool suggests a Standard-only policy (call 6).
- `acq 1.0.0-alpha.1 (contract 0eefc6bb2a3a), provider mock`; acqd at `W/bin/acqd`, 9302080 bytes (call 7).

## 2. The tasks

### T1 — log in and look around

*To the person:* You are logged in as **seat** (realm pc, guild "Playground"; the access token is valid ~55 s and renewed on use; no keyring, so the login lives only as long as the daemon) (calls 8, 9, 10). Your leagues are Standard, Hardcore and **Rehearsal** — Rehearsal is the new one, the only one whose category is not "Standard" (call 11). You have **no characters in Rehearsal**: both of yours, StashHoarder (Scion 97) and MuleQuadTab (Witch 12), are in Standard (call 12). Rehearsal has 8 stash tabs: Currency, Dump, a folder "Old leagues" holding Old 1 (public) and Old 2 (Quad), Uniques, Maps, and Maps (empty) (calls 13, 20). I fetched the Dump tab and the Old 1 tab: **both are empty** (calls 16, 18). Since no Rehearsal character exists, I fetched StashHoarder from Standard instead: an Alpha's Howl (Sinner Tricorne, 4 sockets) on the helm, Seven-League Step boots and three "Fake Item" placeholders in the inventory, and two guardian items; the store counts 7 items for him (calls 17, 21).

*Commands:* 8–21 (14 calls; 14, 15 were help; 19 a refusal).
*Right?* Yes, I hold it right: `tabs` (call 20) agrees with the listing (call 13) and shows the two fetches landed with 0 items; `store characters` (call 21) agrees with call 17. What would tell me otherwise: a later `refresh --plan` that still lists rh-dump/rh-old1 as never fetched. "Which is new" is my inference from category, not something the tool states — the tool never says "new".
*First right call:* 9 (who), 11 (leagues), 12/13 (characters, tabs).
*Where it stopped or misled me:*
- After the second login every store verb refuses without `--account` (call 19) — the README says so in one line under Log in, but job verbs still worked without it (calls 10–18) because only one account is *logged in*. That asymmetry is in `--help` but cost a call.
- `acq characters` has no `--league` filter, so "what characters do I have in it" needs reading every row.
- The fetched tab says `"index": 0` for both Dump and Old 1 (calls 16, 18) while the listing gives them indexes 1 and 3 (call 13).
- The README's tour line for `stash` shows no `--league`, though the "Standard unless you name a league" bullet lists `stash`; `stash --help` (call 14) has it, with an empty description for `<ID>`, `--sub`.

### T2 — keep the new league fresh (part 1; the ceiling run follows below)

*To the person:* There was no policy for your account `seat` (call 23), so I wrote one that only writes where none exists (`--if-revision 0`): Rehearsal, all tabs and all characters, refetched after an hour (call 24); stored as revision 1 (call 25). I named only Rehearsal because you asked for the new league; Standard and Hardcore are therefore *not* kept fresh — add them by writing the `pc` realm back with all three if you want that. The first plan (call 27) was 5 requests, all tab fetches (Currency, Old 2, Uniques, Maps, Maps (empty)), "5–15 sends", the quote said "ready" — no time estimate beyond that; Dump and Old 1 were skipped as fresh from T1, the folder as container-only, and no characters exist in Rehearsal. Applying it (call 29) ran 5 requests as job 13 in about half a second; "changed: no items … left: nothing". The tab list then shows all 7 tabs fetched with 0 items (call 30) — Rehearsal is empty, which a just-opened league would be. The next plan said "nothing to do" (call 31). The plan-as-file run: saved the JSON plan (call 33: 0 actions, `logical_requests` 0), applied it with `--max-requests 3` (call 34) and again (call 35); both said "nothing to do in Rehearsal" and exit 0 — the second application of a spent file is indistinguishable from the first in text.

### T2 — part 2: the careful way, with real work

Because nothing was left to plan, I made work: I temporarily set the Rehearsal max age to 60 s, revision-guarded (call 36 → revision 2), let time pass during T3, and planned to a file (call 46): 4 logical requests — the Rehearsal tab list, Dump, Old 1, and the character list — "4–12 sends" plus a HEAD probe per route and a token refresh if needed; every scope's `eta_seconds` 0. Applying that file with `--max-requests 2` was refused before anything ran: "plan exceeds the budget: 4 logical request(s) against max_requests 2 — nothing was submitted" (call 47; warranted, and it said what to change). With `--max-requests 4` it ran as job 19, 4/4 done, no items changed, and said "5 requests more are planned now" (call 48) — those 5 are the other tabs going stale under the 60 s age, not anything the apply revealed, though the note offers "a map or unique tab's substashes, or left undone" as the reasons. Handing it the same file once more (call 49): exit 0, "nothing to do: every request of this plan is spent — 4 fetched after it was compiled". I restored the 1 h age with `--if-revision 2` (call 50 → revision 3); the plan then said nothing to do (call 51); and the old file was now refused: "the sync policy moved: the plan cites revision 2, but revision 3 is stored" (call 52).

*Commands:* 22–36, 46–52 (22 calls). *Right?* Yes: the tab table (call 30) shows every Rehearsal tab fetched, matching the plan; what would tell me otherwise is a plan listing a never-fetched tab. *First right call:* 24 (the write), 27 (the plan). The final stored policy is revision 3, Rehearsal only, 1 h.
*Where the tool stopped or misled me:*
- `policy set`'s own next step, "preview the work with `acq refresh --plan`" (call 24), fails as printed: with two accounts it needs `--account`, and with no `--league` it plans Standard, which the policy does not cover: "the sync policy does not cover league Standard on realm pc" (call 26) — no hint that `--league Rehearsal` is what was meant, though the tool had just written that policy.
- The plan's `next: acq refresh --apply --league Rehearsal` (call 27), run as printed, was refused: "2 accounts are known and one is to be picked" (call 28). The `--help` says a job verb takes "the sole account logged in" — only `seat` was logged in — but `refresh --apply` behaved like a store verb. Same omission in calls 48, 49, 52's printed next steps.
- The plan text gives no time estimate: "quote … ready" (call 27); the JSON has `eta_seconds` per scope (call 46).
- Re-applying a spent file and applying a file with no actions are both exit 0 with different words (calls 35, 49): a program has to read text or `applied:false` in JSON.

### T3 — what I have (GERWARIC#7694, no daemon)

*To the person:* You have **22,721 live items** (+6 removed), in 3,444 tabs and 65 characters (calls 38, 42). By league: Standard (pc) 21,311, Allflame 1,312, Standard on PoE2 98 (call 42). Standard has 3,347 tabs holding 20,231 items, but only 405 tabs have ever been fetched — 2,926 never were (mostly map and unique substashes: 2,489 of 2,582 map tabs and 437 of 521 unique tabs never fetched), plus 16 folders (calls 43, 42). Allflame has 97 tabs, 96 fetched, 1,101 items (call 44). Characters: 65, 50 fetched (10 of them with empty bodies), 15 never fetched, 1,389 items on them (call 39). **How old:** everything is 50–59 days old: the oldest fetch is 59 days ago, the character *gerwarshot* in Standard; the newest 50 days ago, the *Dump Tab* in Standard (call 42). The character listing was last seen 50 days ago, and the league list was never fetched (calls 38, 39). **Refused:** 4 PoE2 character bodies were not ingested — gerwarsmash, boomsplam, flamdomrando, pilatesinstructor — each "missing an `id` on an item at `equipment[…].socketedItems[0]`", 59 days ago (call 45). 8 PoE2 granted-skill entries are kept but not counted as items (call 38).

*Commands:* 38–45 (8; 40 and 41 refusals). *Right?* Yes — the numbers reconcile: 3,347 + 97 = 3,444 tabs; Allflame 1,101 tab items + 211 on its four characters = 1,312 (calls 39, 42, 44). What would say otherwise: `tabs --expand` totals differing. *First right call:* 38.
*Where the tool stopped or misled me:*
- `search` with no realm refuses on this store (call 40) — the README's tour does say "a store of two realms needs --realm". `realm` is not a count key (call 41): the error lists 70+ keys but not that `--realm all --count league` already splits by realm (call 42 shows `Standard · pc` / `Standard · poe2`).
- `tabs` has no all-leagues view; I had to learn the leagues from `search --count league` (call 42).
- The `next:` lines in calls 43, 44 omit `--account`, so run as printed they would be refused; `store characters` (call 39) did include it.
- The store file grew from 54.4 MB (calls 2, 5) to 56.3 MB (call 38) with no write to this account by me — only reads and the other account's login; the tool doesn't say why.

### T4 — find things (GERWARIC#7694)

*To the person:*
- **(a) Six-links: yes, 187 items** with a six-link group in the PoE1 realm (call 56) — e.g. Skin of the Loyal and Cloak of Flame robes, a Tabula Rasa, several rare body armours and a rare Citadel Bow, almost all equipped on Standard characters, one (Foulborn Skin of the Lords) on I_EXIST in Allflame. The first ten are listed; the rest is paged with a `--next` token.
- **(b) Rare rings with "+# to maximum Life" and at least two resistances (fire/cold/lightning/chaos, as the trade site counts them): 195** (call 60). Twenty shown, e.g. Kraken Circle Bone Ring (+111 life, 12/12/12 elemental) on undxot, Storm Knuckle Two-Stone Ring (+53 life, four resistances) on I_EXIST, Dusk Hold Two-Stone Ring (+68 life, 75 cold / 67 lightning) on VVickedWitch.
- **(c) Per league and rarity** (call 62): Standard PoE1 — 7,693 without rarity (gems, currency, cards), 5,615 rare, 2,850 normal, 2,819 unique, 2,334 magic; Allflame — 561 none, 276 normal, 257 unique, 154 rare, 64 magic; Standard PoE2 — 43 none, 25 magic, 16 rare, 13 normal, 1 unique. Total 22,721.
- **(d) One item** (call 63): Storm Knuckle Two-Stone Ring, rare, item level 83, requires level 64, on I_EXIST's equipment in Allflame, fetched 54 days ago; implicit +12% fire and lightning resistances; explicit +33 evasion, +35% fire, +13% cold, +32% chaos resistance, crafted +53 maximum life; no price; nothing unread.
- **How old:** every answer carries its scope line: in PoE1, fetched 54 days ago at the oldest (Allflame / TheAbsenceOfPatience) to 50 days ago (Standard / Dump Tab), with 2,940 locations never fetched, "nothing here says what they hold" (calls 56, 60); over both realms 59 to 50 days (call 62). Each row carries its own "fetched 53d ago". So: these answers are about seven weeks stale, and blind to 2,489 map tabs and 437 unique tabs never fetched.

*Commands:* 53–63 (11; 57, 58 refusals; 53 overflowed my display, see Left out). *Right?* I hold (a)–(d) right as answers *about what the store holds*; (b) depends on my reading "life" as the `+# to maximum Life` line rather than `pseudo.total_life` (which also counts strength). The cross table sums to 22,721 (call 62), matching T3. What would say otherwise: a ring listed whose `show` lacks a life line. *First right call:* 56 (a), 60 (b), 62 (c; 61 cut 5 cells), 63 (d).
*Where the tool stopped or misled me:* the full `--describe` (call 53) is ~49 KB — too big for my display, so I learned the language entry by entry. "At least two resistances" needed a construct that `count(` is not (call 57); the refusal listed `sum(` and `holds(`, but `--describe sum(` failed ("is nothing describe knows", call 58) while its long list contained the answer, `pseudo.count_res`. `--cross` shows 10 cells by default and the hint "a larger --limit returns the rest" (call 61) is right but easy to miss.

### T5 — prices (GERWARIC#7694)

*To the person:* You have **no prices set by hand** in this store — every price is in-game text. In Standard, 492 items have a price the game decides (489 priced in game), 20,819 have none; in Allflame, 40 priced in game (calls 65, 67). **Price text the tool cannot use — fix these in the game (4 items):**
- Standard, tab "ACQUISITION-PRICE-TEST": *Thaumaturge's Incubator x3* — note `~price  chaos`, "the amount is empty" (calls 66, 69).
- Allflame, tab "~price 222 divine": *Karui Ward Jade Amulet* — `~price 5 exal`, "exal" is not a currency word; *Careful Planning Viridian Jewel* — `~price 5`, no currency; *Self-Flagellation Viridian Jewel* — `~c/o 5 chaos`, not a prefix the game writes (call 68). For these three the tab's 222 divine applies instead.
Also: **1,395 Standard items carry price text the trade site cannot see** (a note on a character's item, or a note/tab name in a non-public tab), and 3 items are `~skip` (call 65) — not errors in the text, but they list nothing.

*Writes:*
- Tab **ACQUISITION-PRICE-TEST** (`tab/pc/03cb479c65`, 81 items, public) at **5 chaos**, `--if-revision 0` so it could not overwrite: "5 chaos (revision 1), was unset"; receipt undo: `acq price clear --account 'GERWARIC#7694' tab/pc/03cb479c65 --if-revision 1` (call 75). Read back: "by hand: 5 chaos (own row, revision 1)"; its items "31 by hand only, 50 conflict" with their own in-game notes (call 80).
- Item **Essentia Sanguis Vaal Claw** (`item/081db69f…`, in a public "~price 4554 chaos" tab) at **2 divine**, `--if-revision 0`: revision 1, "was unset"; receipt undo: `price clear … --if-revision 1` (call 76). I read it (call 77: "own row, revision 1"), then replaced it naming that revision: **3 divine (revision 2), was 2 divine (revision 1)**; receipt: `acq price set … exact 2 divine --if-revision 2` "puts the prior back" (call 78). Read back: effective 3 divine by hand, relation **conflict** with the in-game 4554 chaos tab name — the hand row wins as more specific (call 79).
- *How I would know another program changed a price between read and write:* the revision. `price show` prints "own row, revision N"; a write with `--if-revision N` is refused "naming the current one" if it moved (per `price set --help`, call 70; I test that refusal in T7).
- *What each receipt lets me undo:* the tab's clears its row (back to unset); the first item receipt (call 76) would clear the item, but is stale once the 3-divine write made revision 2; the second (call 78) puts 2 divine back — it does **not** remove the price. To remove it entirely from here, a person needs `price clear … --if-revision 2`, which no receipt printed.

*Commands:* 64–80 (17). *Right?* Yes; the read-backs (calls 79, 80) show both rows at the revisions the receipts named. *First right call:* 65 (status), 66/68 (unusable texts), 75, 78 (writes).
*Where the tool stopped or misled me:* `price show` before any hand row prints no revision at all (call 74), so the "careful" first write has to know `--if-revision 0` from `--help`. The receipts' `next:` lines omit `--realm pc --league Standard` that `price status` printed, but they worked without them.

### T6 — the shop

*To the person:* Your Standard shop is **1 page** (2,288 characters, 31 items), all under one spoiler `~price 5 chaos`, every link in `Stash57`, which the tool names as the tab "ACQUISITION-PRICE-TEST" (call 82). That is the 5-chaos tab price from T5 — but only for the 31 items of that tab with no note of their own; its other 50 items carry their own in-game notes, so the game already lists them and they are omitted. **The 3-divine price does not appear at all:** the Essentia Sanguis Vaal Claw sits in a substash, and the tool *blocks* it — "1 blocked (1 in a substash)" — since the forum has no link form for a substash item (call 82; `shop render --help`, call 81). Left out overall: 491 items the game already lists (488) or skips (3), 1 blocked, 20,788 with nothing applying; no sync policy covers this account, so the page's facts are 50+ days old and nothing says otherwise beyond "policy: none set". *Next:* copy the text from `[spoiler="Shop Post 1 of 1 (31 items)"]` to the closing `[/spoiler]`, paste it into your shop thread in the league's shop subforum on pathofexile.com, preview, post — and do not move the items afterwards (README, What it does not do). `--page 1` prints the page alone for the clipboard.

*Cleanup:*
- Item: the receipt that removes it (call 76's `price clear … --if-revision 1`) run exactly as printed was refused: "has a price at revision 2 (re-read and retry)" (call 83) — warranted, since I had replaced it, and it said what to do. The later receipt (call 78) only "puts the prior back" (2 divine). **No printed command removed the price at its current revision**, so I composed `price clear … --if-revision 2` from the refusal and the read-back (call 84): "cleared, was 3 divine (revision 2)".
- Tab: call 75's undo run exactly as printed: "cleared, was 5 chaos (revision 1)" (call 85).
- Read back: item "by hand: nothing … effective: 4554 chaos in game" (call 86); tab "by hand: nothing … 31 unlisted, 50 in game only" (call 87) — as before (call 71); league status identical to call 65 (call 88).

*Commands:* 81–88 (8). *Right?* Yes; calls 86–88 match calls 71, 74, 65. *First right call:* 82.
*Where the tool stopped or misled me:* T5 had no warning that the item I priced could never reach a shop page — `price set` accepted a substash item silently and only `shop render` said "blocked". The undo chain for a replaced price never offers "remove" again (calls 78, 83).

### T7 — for a program

*To the person:* Every `--json` answer I asked for was **one JSON document on stdout**, nothing on stderr, success or failure (calls 89–99). Successes: `leagues` (call 89; T1's call 11) wraps GGG's answer as `{job_id, account, outcome, payload}`; `store status` (call 90; T3's call 38) gives flat counts — but drops the text's explanations of the 4 refused bodies and 8 granted skills to bare numbers, and adds fields the text never mentions (`unlifted_items`, `withheld_responses`, `withheld_items`); `search links=6 --limit 1` (call 91; T4's call 56) is the answer whole, with ages as both epoch and "54d ago", the continuation as a structured `request`, and the account's seven league names, which no text output had shown me; `price show` (call 92; T5's call 69) and `shop render` (call 93; T6's call 82, now with the T5 prices cleared: 0 posted, 0 pages, 492 left out with only 10 listed and `left_out_total`).

Failures, each once as text and once as JSON:

| failure | text (stream, words) | JSON (stream, doc) | told what to do next? |
|---|---|---|---|
| wrong account `NOBODY#0000` on `tabs` | stderr, exit 1: "no login made a store for `NOBODY#0000`; the names known: GERWARIC#7694, seat (see `acq accounts`)" (call 94) | stdout, exit 1, one doc: `kind:"not_logged_in"`, `accounts`, and two `offers` with `argv` — "the same request as GERWARIC#7694 / seat" (call 95) | yes, both. But `not_logged_in` is the wrong word: the name is unknown, and GERWARIC is not logged in either. |
| `stash rh-nope` through the daemon | stderr, exit 1: progress lines, then "job 25 failed: GET /stash/Rehearsal/rh-nope returned 404 Not Found; no rate headers; a body of 25 bytes" (call 96) | stdout, exit 1, one doc: `{job_id, account, outcome:"failure", error}` — **no `kind`, no `offers`** (call 97) | no: neither says "check the id with `acq stashes --league Rehearsal`", nor where the 25-byte body is. |
| `price set … --if-revision 7` on a cleared item | stderr, exit 1: "…'s price was cleared (revision 3); a new price names no revision" (call 98) | stdout, exit 1, one doc: `kind:"conflict"`, `revision:3`, no `offers` (call 99) | partly: it gives the current revision; "a new price names no revision" contradicts the clear's own receipt, which put it back with `--if-revision 0` (call 84). |

*Commands:* 89–99 (11). *Right?* Yes as observations: each output parsed as one document by eye; I did not run a JSON parser (no pipe allowed). *First right call:* 89.
*Where the tool misled me:* the README says every failure is `{"error":…,"kind":…}` with `offers` (The tour); a failed job verb's JSON has neither (call 97). The `--help` text does document the job form, so the README is the one that overstates.

### T8 — when things go wrong

*To the person:* Before stopping, the daemon was 1.0.0-alpha.1, contract 0eefc6bb2a3a, pid 3033, idle exit after 1800 s, 28 sends, log at `W/logs/184c905d80a6/mock/daemon.log`, send journal beside it (call 100); `acq version` says the client is 1.0.0-alpha.1, contract 0eefc6bb2a3a, and names the `acqd` it would start (`W/bin/acqd`, 9,302,080 bytes) (calls 7, 101) — the two agree. `acq daemon stop`: "daemon stopped (pid 3033) — logged out with it: seat — not in the keyring" (call 102). With no daemon:
- **Store-only read** (`tabs --league Rehearsal`): works unchanged, ages now "4m/5m ago" (call 104).
- **Observers** (`daemon status`, `jobs`, `auth status`): answer from disk and start nothing — "daemon is not running; on disk: 0 waiting … last log: …" (calls 103, 105, 106); `auth status` lists both accounts "not persisted — `acq auth` logs in again".
- **Needs the API** (`leagues`): refused, exit 1: "account \"seat\": not logged in — run `acq auth`; a daemon was started for this command and runs until idle — `acq daemon status` shows it, `acq daemon stop` ends it" (call 107). So a refused API command *still leaves a new daemon running* (pid 5082, call 108) — it said so, which is honest, but it started a daemon it knew could do nothing.
- **Logs:** `daemon status` and `diagnose` print the log path (calls 100, 103, 109); `diagnose` shows the last N journal and log lines, including both 404s of T7 and "submit refused: account … not logged in" (call 109).
- I stopped the second daemon (call 110); **the daemon is stopped** at the end (call 111). Session `seat` is gone with it; I did not log in again.

*Commands:* 100–111 (12). *Right?* Yes; call 111 confirms the end state. *First right call:* 100.
*Where the tool misled me:* `diagnose` replaced the substring `seat` inside the path `…/seat-worlds/…` with `<account 2>` ("~/<account 2>-worlds/…", call 109) — redaction by substring, which both mangles paths and, for an account named like a common word, would garble the report; meanwhile the socket path under `/var/folders/…` is printed unredacted (call 109). The README's log locations (`~/Library/Logs/acq`) are not where this world's log is (`W/logs/…`), presumably because the door sets `ACQ_LOG_DIR` — `daemon status` is the reliable source.

## 3. Faults

Worst first.

1. **Printed next-step commands that fail as printed when two accounts exist.** `policy set` says "preview the work with `acq refresh --plan`" (call 24) → refused for Standard not covered (call 26); the plan's `next: acq refresh --apply --league Rehearsal` (call 27) → "2 accounts are known and one is to be picked" (call 28); same omission in calls 43, 44, 48, 49, 52. *Should:* carry the `--account` and `--league` the command was run with (as `price status`, call 65, and `store characters`, call 39, already do). Refusals warranted? Call 28's was **not**: only `seat` was logged in, and `--help` says a job verb takes the sole logged-in account. Enough to recover? Call 28 yes (names both accounts); call 26 no (doesn't mention `--league`). A person hits this too — anyone with a second login.
2. **A price can be set on an item the shop can never post, silently.** `price set` on a substash item succeeded (call 76) with no note; only `shop render` reports it "blocked (1 in a substash)" (call 82). *Should:* say at write time that the target is unpostable. Person too.
3. **The undo chain loses "remove".** After a replace, the receipt only offers "put the prior back" (call 78); the earlier clear receipt goes stale and is refused (call 83, warranted, with "re-read and retry" — enough to recover only if you know `price clear --if-revision 2` is what you want). *Should:* a replace receipt also prints how to remove the row. Person too.
4. **A failed job's `--json` has no `kind` and no `offers`** (call 97), against the README's "a failure is `{"error":…,"kind":…}` … its `offers`"; and neither text nor JSON suggests checking the id with `acq stashes` (calls 96, 97). Agent mainly.
5. **A refused API command spawns a daemon it knows can't serve it** (call 107 → call 108), defeating "leave the daemon stopped" until `daemon stop` again. It does say so. Person too.
6. **`diagnose` redacts by substring**, rewriting `seat-worlds` as `<account 2>-worlds` (call 109). Person too (with a short or common account name).
7. **Wrong-account JSON `kind` is `not_logged_in`** for a name no store has (call 95). Agent.
8. **`price set` conflict says "a new price names no revision"** (call 98) while the clear receipt says re-create with `--if-revision 0` (call 84). Person too.
9. **Tab `index` disagrees** between `stash` (0 for both tabs, calls 16, 18) and `stashes` (1 and 3, call 13). Person too (only visible in JSON-ish output).
10. **Store file grew 54.4 → 56.3 MB** without a write to that account (calls 5, 38) — unexplained. Person too.
11. **`--describe sum(`** refused although `sum(` is a call the language offers (calls 57, 58). Person too.

## 4. What the README and the help did not say

- **Job verbs and store verbs pick an account differently, and `refresh --apply` behaves like a store verb**: with one session and two stores, `profile`/`stash` worked bare (calls 10–18), `tabs` and `refresh --apply` refused (calls 19, 28). Expected in README "Log in" ("a second login makes every verb need `--account`" — which is closer to the truth than `--help`'s "a job verb [takes] the sole account logged in").
- **How to write a policy only if none exists: `--if-revision 0`** — in `policy set --help` (call 22), not the README's "A new league", which only warns about dropping Standard.
- **`refresh --plan` without `--league` plans Standard and fails if the policy doesn't cover it** (call 26). Expected in "A new league" / `policy set`'s receipt.
- **A plan's time estimate** is only per-scope `eta_seconds` in `--json` (call 46); the text says "ready" (call 27). Expected in `refresh --help`.
- **How to count lines, e.g. "at least two resistances"**: `pseudo.count_res` (call 59), found only in the error list of call 58. Expected in `search --describe` index's first screen or the README tour.
- **A store with two realms needs `--realm` for search, and `realm` is not a count key; `--realm all --count league` splits by realm** (calls 40–42). Partly in the README tour.
- **`tabs` is per league with no all-leagues view**; the league list for a store comes from `search --json` scope (call 91) or `--count league` (call 42). Expected in `tabs --help`.
- **`--cross` shows ten cells by default** (call 61). In the output, not the README.
- **A first hand price on a target is written with `--if-revision 0`**; `price show` prints no revision until a row exists (call 74). In `price set --help` (call 70).
- **Substash items can be priced but never posted** (call 82). Expected in README "What it does not do" or `price set --help`.
- **Stopping the daemon logs you out without a keyring** — the README does say it (Log in, and the `daemon` tour line); `daemon stop` confirms (call 102). Said and needed.
- **Pointers I could not follow:** `CLI-REFERENCE.md` (the tour defers flags to it), `AGENTS.md`, `decisions/<area>.md`, `SECURITY.md`, `CONTRIBUTING.md`, the source files in Knobs. The help covered what I needed except the query language overview.

*Said and never needed:* Install (archives, checksums, quarantine, Windows shells), the MCP section and its 25 tools, `ACQ_SHELL`, `ACQ_NO_SPAWN`, `ACQ_HELLO_WITHIN`, `ACQ_STARTUP_WITHIN`, `ACQ_ANNOTATION_WAIT`, retention knobs, the rails (`ACQ_TRIPWIRE`, `ACQ_MAX_SENDS`, `ACQ_JOURNAL`), `ACQ_MOCK_DEGRADED_HEAD`, `store import/rebuild`, `submit`, `demo`, `web`, `dash`, `cancel`/`set-priority`, `--trade`, the relicensing audit.

## 5. The tool's own words

- *Right and useful:* the search's scope line ("2940 locations were never fetched: nothing here says what they hold", call 56) — exactly the staleness a person needs; the budget refusal "plan exceeds the budget … nothing was submitted" (call 47); "every request of this plan is spent" (call 49); "the sync policy moved: the plan cites revision 2" (call 52); price receipts with `was …` and an undo (calls 75, 76, 78, 84, 85); invalid-note explanations ("\"exal\" is not a word of currency table v1", call 68); `daemon stop`'s "logged out with it" (call 102); call 107 admitting it left a daemon running.
- *Wrong or misleading:* `next:` lines without `--account`/`--league` (calls 24, 27, 43, 44, 48, 52); "5 requests more are planned now (what this apply revealed, such as a map or unique tab's substashes, or left undone)" (call 48) when they were simply stale; `kind:"not_logged_in"` for an unknown name (call 95); "a new price names no revision" (call 98); `price status`'s "1 note … (no effect: the tab's name applies)" (call 65) for a tab whose name "reads none" (call 69); `diagnose`'s `<account 2>-worlds` (call 109).
- *Missing:* a time estimate in plain text (call 27); "check the id" on a 404 (calls 96, 97); a warning on pricing a substash item (call 76).
- *Read and never used:* `--routes`, the `--next acq1.…` page tokens (calls 56, 60), the plan's `jq` recipes (call 32), `wire: 5–15 sends` (call 27), the long `basis` lines, `diagnose`'s redaction counts.
- *Words I did not understand:* "unlifted_items", "withheld_responses/items" (call 90); "contract" (calls 7, 100) beyond "build digest"; "rails" and "tripwire" (call 100) — the README explains them only by source-file pointers; "facts v9 · derivation 12 · intent 5" in `basis`; "lacked" vs "failed" in term counts (call 56) — I inferred lacked = has no such field.

## 6. What you would ask of the tool

- Every printed `next:` carries the `--account`, `--league`, `--realm` the command ran with (calls 27, 28).
- `refresh --plan` with no `--league` and a single-league policy plans that league, or says "the policy covers Rehearsal: add `--league Rehearsal`" (call 26).
- A plain-text "≈ N min" on every plan (call 27).
- `price set` warns when the target cannot be posted (substash) (calls 76, 82).
- Every replace receipt also prints the clear that removes the row (call 78).
- A failed job's JSON with `kind` and `offers`, and a 404 that suggests `acq stashes` (call 97).
- A refused API command that does not leave a fresh daemon behind (calls 107, 108).
- `characters --league L` (call 12) and `tabs` across all leagues (call 43).
- `diagnose` redacting whole tokens only (call 109).

## 7. Left out

- *Not tried:* `acq web` and `acq dash` (full-screen/browser — I have neither); `--trade` URLs; `store import`/`rebuild`; `submit`, `demo`, `cancel`, `set-priority`, `result`; `refresh --tabs/--all/--characters`; `--deep` on map tabs (Rehearsal's map tabs had no substashes); a Standard policy for `seat`; the `acq-mcp` server; `daemon reset-tripwire`; logging in again after T8.
- *Files opened that are not one of the two:* none. One note: my harness saved call 53's 48.8 KB output to a file of its own because it exceeded my display; I did **not** open that file — I re-asked the language entry by entry (calls 54, 55, 58, 59) instead.
- *Files written:* `W/work/plan-rh.json` (call 33), `W/work/plan-rh2.json` (call 46), both by `--out`; the report and debrief through the door. No drafts.
- *State changed and left:* `seat`'s policy is revision 3, Rehearsal only, 1 h (call 50) — revisions 2 was my temporary 60 s policy to exercise the ceiling; GERWARIC's two prices were cleared (calls 84, 85) — the item row's revision counter now stands at 3 (call 98); the daemon is stopped (call 111).
- *The one thing cut I would most want back:* a real multi-round first sync — Rehearsal's tabs were all empty and its map tabs had no substashes, so I never saw a plan with substash fetches, a limiter hold, or a time estimate that mattered.
