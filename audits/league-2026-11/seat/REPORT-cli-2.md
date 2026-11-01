<!-- written during the run; call numbers are rows of calls.csv -->

## 1. Before touching anything

**What the brief told me (the brief):** the tool is `acq`, the CLI of Acquisition for Path of Exile; the store holds `GERWARIC#7694` (a copy of a real ~50 MB collection with prices, no login) and I log in a second account through the door; the world points at a mock API; there is no keyring; the door starts the daemon with a 30-minute idle exit.

**What the README told me:**
- Can do: download stash tabs and characters to the machine, search them offline (incl. trade-site URLs), price items, render forum shop pages to paste; an MCP server with 25 tools doing the same.
- Needs: a login (browser, `acq auth`) for anything that fetches; jobs go through the daemon `acqd`, which `acq` starts on demand beside itself and which needs a login; the store, search, pricing, shop, `policy set` and `price set` need no daemon and no network. Without a keyring a login lasts as long as the daemon that holds it. A second login makes every verb need `--account`.
- Before a first command ("Before the first command"): the rate limit is the clock (30 tab requests per 5 minutes, first sync of a few hundred tabs most of an hour); know which verbs need the daemon; everything defaults to Standard unless `--league` is named.
- New league: `acq leagues` names it; write the first policy with `--if-revision 0` (lands only where none is stored); write a realm back with every league it keeps (naming only the new league drops Standard); then `refresh --plan --league X` and `--apply --league X` until the plan says nothing to do; read each plan's work and ceiling before applying.
- Pointers I cannot follow here: `AGENTS.md`, `CLI-REFERENCE.md`, the `*.rs` sources the Knobs table links, `decisions/<area>.md`, `SECURITY.md`, `CONTRIBUTING.md`.

**What I would not run without asking the person:** `acq policy set --replace` (or any policy write without `--if-revision`) on their real account; `acq refresh --all`/`--apply` without a `--max-requests` ceiling against GGG (hour-long, rate-limited); `acq daemon stop` while a login is not in a keyring (it logs them out) or while jobs run; `acq daemon reset-tripwire`; `acq auth logout`; `acq store rebuild`/`store import` on their store; `price clear` on their prices; `cancel` of jobs I did not submit.

**What the tool told me (the world as found):** `acq --help` lists 27 verbs incl. a TUI `dash` and a loopback `web` (call 1). One account on disk, `GERWARIC#7694`, last login 16d ago, "not persisted", store 54.4 MB (call 2). No daemon running, no queue on disk (call 3) — although the brief said the door starts one, it does so at login. `auth status`: no live session; GERWARIC "not persisted — `acq auth` logs in again" (call 4). Store at `W/store/mock/GERWARIC_7694.db` (54.4 MB): 1126 responses, league list never fetched, 65 characters, 3444 tabs, 22721 items (+6 removed), 22751 events; 8 granted skill entries; 4 refused bodies (call 5). Note the store lives under `store/mock` — i.e. this world runs `ACQ_PROVIDER=mock`.

## 2. The tasks

### T1 — log in and look around

**To the person:** "You are logged in as `seat` (mock provider, no keyring — the login lives only as long as the daemon). You have three leagues: Standard and Hardcore (category Standard) and **Rehearsal**, the new one (its category is Rehearsal, not Standard). You have two characters, StashHoarder and MuleQuadTab, both in Standard — **none in Rehearsal**, so I could not fetch a character of the new league; I fetched MuleQuadTab instead. Rehearsal has eight stash tabs: Currency, Dump, a folder 'Old leagues' holding Old 1 (public) and Old 2 (quad), Uniques, Maps and 'Maps (empty)'. I fetched Dump and Currency: both are empty. MuleQuadTab carries one helmet and four inventory items, all 'Fake Item n' normals."

**Commands:** login (call 6), `auth status` (call 7), `leagues` (call 8), `profile` (call 9), `characters` (call 10), `stashes --league Rehearsal` (call 11), help for `stash` and `characters` (calls 12, 13), `stash rh-dump --league Rehearsal` (call 14), `character MuleQuadTab` (call 15), `stash rh-cur1 --league Rehearsal` (call 16), `tabs --league Rehearsal` refused without `--account` (call 17), with it (call 18), `store characters --account seat` (call 19).

**Right?** I hold it right; first right answer call 7 (who), call 8 (leagues). What would say otherwise: "new" is my inference from the category column and the brief — the tool never marks a league as new or current; `leagues` prints no start date. The stored tab list (call 18) agrees with the fetched one (call 11): 8 tabs, 2 fetched, 0 items.

**Where it stopped or misled me:**
- No character is in the new league, and `characters` has no `--league` filter (call 13), so "what characters in it" needed reading the league field by hand.
- The mock's character fetch (call 15) says MuleQuadTab is a level-97 Scion; the list (call 10) and the store (call 19) say level-12 Witch. Mock data, but the store kept the list's level without noting the disagreement.
- `stash` index in the fetched tab is 0 for both tabs (calls 14, 16) while the listing gives 1 and 0 (call 11) — mock data.
- `stash --help` gives the `<ID>` argument and `--sub` no words at all (call 12).
- After the second login every store verb needs `--account` (call 17), as the README's "Log in" says; but job verbs (`leagues`, `stash`) did not — they took the one *logged in* account, which the global `--account` help explains. The refusal on stderr (call 17) names both accounts and both ways to pick — good.
- `profile`, `characters`, `stash`, `character` print GGG's JSON with no `--json` flag; `leagues` prints a table (call 8). The README's "a job verb … prints GGG's JSON" is true of all but `leagues`.
- `auth --no-browser --json` (the door's login, call 6) printed two JSON documents on stdout (the `authorize_url` line, then the result) — the README says `--json` is "one document"; I note it but the brief asks me not to fault the login's mechanics.

### T2 — keep the new league fresh

**To the person:** "There was no sync policy on `seat` (call 22). I wrote one that covers only Rehearsal — every tab, every character, re-fetch anything older than an hour — with `--if-revision 0`, which the tool lands only where no policy exists; it stored it as revision 1 (call 23). Sending the same write again was refused with 'the sync policy is at revision 1' (call 24), so had you had a policy it would not have been overwritten. Stored policy: call 25. The first plan was 5 requests — the five tabs never fetched (Old 1, Old 2, Uniques, Maps, Maps (empty)) — Currency and Dump fresh from my T1 fetches and the folder skipped; quote 'ready', 5–15 sends on the wire (call 27). Applied as job 13 → jobs 14–18, all 5 done, no items changed, 'left: nothing' (call 28). The stored tab list now shows 7 of 8 fetched (folder never — it is only a container), 0 items: every Rehearsal tab is empty (call 29). A second plan: 'nothing to do' (call 30). The careful round: I saved the plan as a file (call 31, `W/work/plan-rehearsal.json`): 0 actions, 0 logical requests, 0 wire sends; applied it under `--max-requests 5` (call 32) and again (call 33): both said 'nothing to do', exit 0. Your policy does not cover Standard or Hardcore — say if you want them added."

**Commands:** 20–33 (help 20, 21, 26).

**Right?** Right as far as it goes: first right answer call 23 (the write), call 28 (apply). What would show it wrong: a non-empty Rehearsal tab — the mock serves empty tabs, so "landed" is shown by fetch times (call 29), not by items.

**Where the tool or README stopped or misled me:**
- **The careful round could not be exercised.** By call 31 nothing was left, so the ceiling (`--max-requests`) and the second hand-over of a plan file (`applied: false`/`spent` per the help, call 26) were never tested: with an empty plan the tool printed the plain "nothing to do" both times (calls 32, 33), the same as a fresh plan — it does not say whether the file was spent, refused or simply empty. How I would have done it with work left: `refresh --plan --json --league Rehearsal --out plan.json`, read `.logical_requests` and `.actions`, then `refresh --apply=W/work/plan.json --max-requests N`; then the same command again, expecting the help's "sends nothing (`applied: false` and `spent`)". I did not lower `max_age_seconds` or add Standard just to manufacture work against the person's stated wish.
- **`policy show` offers an unsafe and unrunnable command** (call 22): with no policy it says "write one with `acq policy set '{…Standard…}'`" — no `--if-revision 0` (the README's own rule for a first policy), no `--account` (which two accounts on disk require, call 17), and it names Standard, not the league just listed.
- **The overwrite refusal** (call 24) says "re-read and retry". Warranted (I asked it to refuse). Recoverable: it gives the revision; but "retry" is the wrong advice for someone who said "stop and tell me", and it does not say a policy *exists* in words.
- The README's "A new league" says the first plan is "the listings, one or two"; here it was the tabs at once (call 27) because the listings were fresh from T1 — consistent, but the README reads as if always two rounds.
- The help (call 26) says a first sync takes two rounds because of map/unique substashes; Maps and Uniques gave none here (call 28 "left: nothing").
- Plan text and JSON never give a duration (the README promised "how long"): only "quote … ready" and "wire: 5–15 sends" (call 27).
- `refresh --help` (call 26) teaches agents to pipe to `jq` — impossible under this seat's rules and not mentioned in the README.

### T3 — what I have

**To the person:** "You have **22,721 live items** (plus 6 removed) (calls 34, 42). By league: 21,311 in Standard (PC), 1,312 in Allflame (PC), 98 in Standard on PoE2 (call 42). By place: 21,332 in stash tabs, the rest on characters — 966 equipped, 165 in inventories, 137 jewels, 23 'guardian', and on PoE2 characters 56 equipped and 42 skills (call 42); `store characters` totals 1,389 items on 65 characters (call 36). Tabs: 3,444 in all (call 34) — 3,347 in Standard, of which only 405 were ever fetched and 2,926 never (mostly map and unique substashes: 2,489 of 2,582 map tabs and 437 of 521 unique tabs), plus 16 folders (call 38); 97 in Allflame, all fetched but the folder, 1,101 items (call 44). Characters: 65 — 50 fetched (10 of them with empty bodies), 15 never fetched (call 36). **How old:** everything is old — the newest fetch is 50 days ago (Standard / 'Dump Tab'), the oldest 59 days ago (Standard / gerwarshot, a PoE2 character) (call 42); the character list was seen 50–59 days ago; the account's league list was never fetched (call 34). **Refused:** 4 bodies, all PoE2 characters (gerwarsmash, boomsplam, flamdomrando, pilatesinstructor), 59 days ago, each 'missing an `id` on an item at equipment[n].socketedItems[0]' — kept verbatim (call 43). Also 8 PoE2 granted-skill entries kept with their characters and not counted as items (call 34). What sits in the 2,941 never-fetched places is unknown."

**Commands:** 34–44 (help 35, 37; `search --describe` 39 belongs to T4's learning).

**Right?** Yes; the numbers cross-check: 3,347 + 97 = 3,444 tabs; 20,231 + 1,101 = 21,332 tab items; 1,101 + 211 (four Allflame characters) = 1,312 Allflame items; 2,926 tabs + 15 characters = 2,941 never-fetched places (calls 36, 38, 42, 44). First right answer: call 34 (count), call 42 (ages and their sources). What would make it wrong: none of these were fetched for 50 days — the store is a picture of 50–59 days ago, not now.

**Where the tool or README stopped or misled me:**
- No single verb answers "how old and from where" — `store status` gives counts but no ages (call 34); `tabs` is one league at a time and defaults to Standard (call 38); the oldest/newest location appeared only in a `search` scope line (call 42). I found it by luck.
- The PoE2 character `pilatesinstructor` has its body refused (call 43) yet `store characters` lists it fetched 59d ago with 30 items (call 36) — the two do not say how they relate (an earlier good fetch? the listing's items?).
- **`store status` size changed from 54.4 MB (call 5) to 56.3 MB (call 34)** though I had written nothing to `GERWARIC#7694` — only read it and written `seat`'s store. The tool does not say why (journal/WAL growth, a read that writes?). For a person told the store verbs only read, unexplained.
- `store status` says "league list never fetched (`acq leagues`)" for an account with no login — the offered command cannot run for it.

### T4 — find things

**To the person:**
- (a) "Yes: **187 items with six linked sockets** — 132 body armours, 30 bows, 9 two-handed axes, 7 staves, 5 two-handed swords, 3 warstaves, 1 two-handed mace; 181 in Standard, 6 in Allflame; 161 in stash tabs, 24 worn, 2 in inventories (calls 45, 46). E.g. a Tabula Rasa on BatteryStaple, a 6-link Astral Plate on VVickedWilliam."
- (b) "**195 rare rings** have a '+# to maximum Life' line and at least two resistances (PC; 188 Standard, 7 Allflame; 172 in tabs, 21 worn, 2 in inventories) (call 49). If 'life' also counts Strength (the trade site's total life pseudo), it is 224, plus 16 PoE2 items the tool could not decide on because it has no PoE2 class or totals table (call 47)."
- (c) "Per league and rarity (call 51): Standard PC — 5,615 rare, 2,850 normal, 2,819 unique, 2,334 magic, 7,693 with no rarity (gems, currency, cards…); Allflame — 154 rare, 276 normal, 257 unique, 64 magic, 561 none; Standard PoE2 — 16 rare, 13 normal, 1 unique, 25 magic, 43 none. 8,297 have no rarity in all."
- (d) "Storm Knuckle Two-Stone Ring, rare, item level 83, requires level 64, worn by I_EXIST in Allflame, fetched 54 days ago: +33 evasion, +35% fire, +13% cold, +32% chaos res, crafted +53 life, implicit +12% fire and lightning res; not priced; nothing unread (call 52)."
- **Age of facts:** every answer carries the same scope line: the newest fetch 50 days ago (Standard / Dump Tab), the oldest 59 days ago (54 on PC only), and "2,941 locations were never fetched: nothing here says what they hold" — 2,489 map tabs, 437 unique tabs, 15 characters (calls 45–51). Each row prints its own "fetched Nd ago" (calls 45, 47, 52). So every count is a floor, 50+ days old.

**Commands:** 39–41, 48 (learning the language), 45–47, 49–52.

**Right?** I hold (a), (c), (d) right: first right answers call 45, call 51, call 52. (b) depends on what "life" means: the tool's `pseudo.total_life` counts half of Strength as life (call 41) and so lets a Strength-only ring in (Viper Band, call 47); I took the stricter call 49 as the answer and named both. Cross-check: the 15 cells of call 51 sum to 22,721.

**Where the tool or README stopped or misled me:**
- **`search --describe` with no argument printed 49,833 bytes** (call 39) — under the door's 60 KB cut, but too big for my harness, which kept only the first 2 KB in front of me; I did not open the saved copy (it is a file outside the two). I recovered because a deliberately wrong name (`life`, call 40) made the refusal print every entry name — the most useful page of help I got, by error.
- `--cross` silently showed 10 of 15 cells until `--limit` (call 50 → 51) — it said so ("a larger --limit returns the rest"), good, but `--limit` meaning "cells" in a cross and "rows" elsewhere is only explained once you hit it.
- A store with two realms: the README says `search` needs `--realm`; I always passed it. With `--realm all`, terms on PoE2 items are "undecided" (call 47) — explained well, with "a refresh will not help; a reference update may".
- `describe` lists `is` twice in the index (call 40).
- The `more` line carries a ~900-character opaque `--next` token (calls 45, 47) — workable for a program, unreadable for a person.
- The README's tour never says how to ask "how old" — I learned it is in every search's `scope` line by reading output (call 42).

### T5 — prices

**To the person:** "You have **no prices set by hand** in this store — 0 in Standard and 0 in Allflame (calls 54, 56); the brief said the copy carries prices you set, and those turn out to be in-game prices: 489 items priced in game in Standard (492 decided by the game), 40 in Allflame. **Price text the tool cannot use — fix these in the game:**
- Standard, tab 'ACQUISITION-PRICE-TEST': *Thaumaturge's Incubator x3* — note `~price  chaos`, the amount is empty (call 55).
- Allflame, tab '~price 222 divine': *Karui Ward Jade Amulet* — note `~price 5 exal`, 'exal' is not a currency word; *Careful Planning Viridian Jewel* — `~price 5`, no currency; *Self-Flagellation Viridian Jewel* — `~c/o 5 chaos`, not a prefix the game writes (call 58). These three still sell at the tab's 222 divine.
- Also 1,395 items in Standard carry price text the trade site cannot see (notes on character items or in private tabs), and 3 are `~skip` (call 54); I did not list them one by one.
I priced the tab 'Delirium (Remove-only)' (tab/pc/687a520a63, 16 items, private) at **5 chaos each** (call 63), and the *Dread Coil Topaz Ring* in your public MARKET tab at **2 divine** (call 64), then — after reading it at revision 1 (call 65) — replaced it with **3 divine** naming revision 1; it landed as revision 2 (call 66). Read back: the ring 3 divine by hand, revision 2 (call 67); the tab 5 chaos by hand, revision 1, its 16 items 'by hand only' (call 68)."

**What each write said and what its receipt undoes:**
- Tab (call 63): "5 chaos (revision 1), was unset"; undo `acq price clear --account 'GERWARIC#7694' tab/pc/687a520a63 --if-revision 1` — removes the row, back to unset.
- Ring at 2 (call 64): "was unset"; undo `price clear … --if-revision 1` — superseded by the next write.
- Ring at 3 (call 66): "3 divine (revision 2), was 2 divine (revision 1)"; undo `acq price set … exact 2 divine --if-revision 2` — puts **the 2 divine** back, not the unset state; to get back to "never priced" one needs a `price clear`, which no receipt offered after the first.
- **Knowing another program wrote in between:** every write named a revision (`--if-revision 0` for the first, `1` for the replacement); per the help (call 62) a write at a revision that is no longer the row's is refused naming the current one. I test that refusal in T7.

**Commands:** 53–68 (help 53, 59, 62; tab choice 60).

**Right?** Yes; first right answer call 54 (priced state), calls 55 + 58 (the bad notes), calls 67/68 (read-backs).

**Where the tool or README stopped or misled me:**
- **`price show` of an unpriced target prints no revision** (call 61), while `price set --help` says to pass "what `acq price show` printed when you reviewed it" (call 62). For a never-priced target you must know that 0 means "none" — `price set --help` does say it, `price show` does not.
- `price status` defaults to Standard and says nothing about other leagues (call 54); Allflame's three bad notes appeared only when I asked by league (call 56). A person asking "is anything wrong with my prices" gets half the answer.
- The README never says what a "price" in this store is made of (hand rows vs. in-game note/tab text, public vs. private) beyond "What it does not do"; `price status` (call 54) explains it well.
- Pricing a private tab (call 63) was accepted silently; whether those 16 items reach the shop is only learned in T6.
- `price show` without `--league` worked for an item (README: "reads the league the facts hold the target in") — correct (calls 65, 67).

### T6 — the shop

**To the person:** "Your Standard shop is **one page** of 1,318 characters with **17 items** (call 70). Inside the outer spoiler 'Shop Post 1 of 1 (17 items)' there are two price spoilers: '~price 5 chaos' holding 16 links to `Stash9` = the tab 'Delirium (Remove-only)' (x 28–43, y 0), and '~price 3 divine' holding 1 link to `Stash39` = 'MARKET', x 4 y 0 — the Dread Coil Topaz Ring. **Left out:** 492 items the game already lists (489 by their in-game note or tab price, 3 marked ~skip); 0 blocked (nothing in a substash or socketed); 20,802 items nothing applies to. It also warned that no sync policy covers these facts, and that the 17 positions were seen before the current stash listing — if a tab was reordered since, a link would point elsewhere; a refresh re-reads them. **Next:** paste page 1 into your shop thread in the Standard shop subforum on pathofexile.com, preview before posting, and leave the items where they are (a moved item may lose its link). But this store is 50+ days old — refresh before you post. I then removed both prices: the ring's and the tab's; both read back as 'nothing applies', and `price status` is back to 0 by hand, 489 in game (calls 74–76), as it was in call 54."

**Commands:** 69–76.

**The undo, honestly:** the tab's undo printed by call 63 (`price clear … tab/pc/687a520a63 --if-revision 1`) ran exactly as printed and worked (call 73). **For the item no printed undo removed the price as it stood:** the last receipt (call 66) offered only "put 2 divine back"; the earlier one (call 64) offered `price clear … --if-revision 1`, which I ran exactly as printed and which was refused (call 71: "has a price at revision 2, not the revision named: `--if-revision 2` clears it as it stands"). I then ran the command that refusal named (call 72). So the item was cleaned with a command the tool printed — in a refusal, not in a receipt.

**Right?** Yes; first right answer call 70. What would make it wrong: `Stash9`/`Stash39` are ranks computed from a 50-day-old listing (the tool says so, call 70).

**Where the tool or README stopped or misled me:**
- Receipts are one step deep: a replacement's receipt undoes to the previous price, never to "unset"; the README's "set a price, with undo" doesn't say that (calls 64, 66).
- The 16 items of a **private** tab went on the page (call 70) without a word that the tab is not public — `price show` had said "not public" (call 68). README's "What it does not do" says the trade site lists a private tab's items via the shop, so it is likely right; the shop page itself does not say so.
- `shop render --help`: `--realm` has no description (call 69).

### T7 — for a program

**To the person:** "All five commands I re-ran with `--json` gave one JSON document on stdout, exit 0, nothing on stderr: `leagues` (call 77, the job envelope `{job_id, account, outcome, payload}` with GGG's body whole), `store status` (call 78), `show <ring>` (call 79), `price list --game-text invalid_note` (call 80), `shop render` (call 81 — after my clean-up, so 0 pages; its `policy` table names every cell and why). The three failures:"

| failure | text (stream, exit) | JSON (stream, exit) | one document? | says what to do next? |
| --- | --- | --- | --- | --- |
| wrong account `NOBODY#0000` on `tabs` | stderr, exit 1 (call 82) | stdout, exit 1, `kind: "not_logged_in"` (call 83) | yes | yes — text names both known accounts and `acq accounts`; JSON adds `offers` with ready `argv` for each account |
| fetch through the daemon of tab `rh-nope` | stderr (progress lines + `Error: job 20 failed: GET /stash/Rehearsal/rh-nope returned 404 Not Found; no rate headers; a body of 25 bytes`), exit 1 (call 84) | stdout, exit 1, `{"job_id":21,"outcome":"failure","error":…}` (call 85) | yes | **no** — neither form says "this tab id does not exist; `acq stashes --league Rehearsal` lists the ids" |
| price write at `--if-revision 7` on a cleared row | stderr, exit 1: "price was cleared (revision 3); a new price names revision 0, or none" (call 86) | stdout, exit 1, `kind: "conflict"`, `revision: 3`, one offer: the same price with `--if-revision 0` (call 87) | yes | yes |

**Commands:** 77–87.

**Right?** Yes, first right answer call 77. The README's claim "Every command takes `--json`, and it is total" held for everything I tried in T7.

**Where the tool or README stopped or misled me:**
- **Wrong `kind` for an unknown account**: `"not_logged_in"` (call 83) — the account does not exist at all; `seat` *is* logged in. A program branching on `kind` would tell the user to log in.
- **404 on a nonexistent tab gives no way on** (calls 84, 85): HTTP wording ("no rate headers; a body of 25 bytes") instead of "no such tab", no `offers`, no `next`.
- The 404 JSON (call 85) exits 1 — the README says a failed job is "its outcome record … not a refusal", which I read as possibly exit 0; `result --help` says "A failed or cancelled job exits 1" — consistent, but the README doesn't say the exit code.
- A wrong-revision write on a *cleared* row (call 86) is a different message from the live-row case (call 71) — both helpful; the JSON `offers` re-creates the price, which a careful program must not run blindly after a conflict.
- The `basis.intent` in call 79 is 5 while the text `show` in call 52 printed "prices and policy revision 0" — my five price writes moved it, as the `basis` describe entry (call 41) says; useful for detecting another writer, but the README never mentions it.

### T8 — when things go wrong

**To the person:** "The tool is `acq 1.0.0-alpha.1`, contract `0eefc6bb2a3a`, provider mock; the `acqd` beside it (9,302,080 bytes) is what a job command would start (call 89). The running daemon was the same version and contract, pid 89859, up 349 s, idle exit after 1800 s, 23 sends so far (call 88). Its log is `W/logs/d809ff563606/mock/daemon.log` and its send journal `W/logs/d809ff563606/mock/sends.jsonl` (call 88; `diagnose` prints their tails, call 97). I stopped it (call 90) — and **that logged `seat` out**: 'not in the keyring, so the session lived in that daemon alone'. With the daemon down:
- a store-only read (`tabs --league Rehearsal`) works exactly as before (call 91);
- the observers answer from disk and start nothing: `jobs` — 'daemon is not running; nothing waits on disk' (call 92); `daemon status` — 'not running; on disk: 0 waiting, 0 recorded running', and the last log path (call 93);
- a command that needs the API (`leagues`) **started a new daemon and then refused**: 'account "seat": not logged in — run `acq auth`; a daemon was started for this command and runs until idle' (call 94). That daemon (pid 92787, 0 sends) would have sat idle for 30 minutes with no login; I stopped it (calls 95, 96). `diagnose` confirms nothing running at the end (call 97)."

**Commands:** 88–97.

**Right?** Yes; first right answer call 88 (version and log), call 91–94 (the three behaviours). The daemon is left stopped (calls 96, 97).

**Where the tool or README stopped or misled me:**
- **A job command with no session starts a daemon it cannot use** (call 94) and leaves it running "until idle" (here 30 minutes; 1 hour by the README's default). It says so honestly, but it should check the login before spawning — the README's "Log in" claims "`acq` refuses to start a daemon against GGG without one"; under the mock it did start one.
- `daemon stop` silently ending the login (call 90) is said *after* the fact; the README's tour does say "(a login not in the keyring goes too)", the verb does not warn before.
- The README says the log is under `~/Library/Logs/acq` on macOS; here it is under `W/logs/…` because the world sets `ACQ_LOG_DIR` — `daemon status` printed the real one, as the README said it would.
- `version` reports `acqd` by length (call 89) while `daemon status` reports it by sha256 prefix (call 88): two different identities for the same file, so a person cannot compare them by eye.

## 3. Faults

Worst first.

1. **The tool's own offered first-policy command can overwrite.** `policy show` with no policy offers `acq policy set '{…"Standard"…}'` without `--if-revision 0` and without `--account` (call 22). The README's own rule for a new league is `--if-revision 0`. Copying the offer is the unsafe form, names the wrong league for a new-league user, and cannot run where two accounts are on disk (call 17). This is not a refusal. A person at a keyboard would hit it, and an agent copying `next` lines even more so.
2. **A job command with no session spawns a daemon, then refuses** (call 94). The refusal is warranted: I was logged out. It says enough to recover ("run `acq auth`") and admits the stray daemon. But the spawn itself is the fault: it leaves a useless process running for the idle period, which is 30 min here (call 95). A person would hit it too.
3. **The wrong `kind` for an unknown account.** The JSON says `kind: "not_logged_in"` for an account that does not exist (call 83). The refusal is warranted, and the text and offers are good enough to recover. The `kind` misdirects a program, so this one is mostly an agent's problem.
4. **A 404 on a nonexistent tab gives no way on** (calls 84, 85). The refusal is warranted, but recovery is poor: the message is HTTP wording ("no rate headers; a body of 25 bytes") with no "no such tab" and no `acq stashes` offer. Both a person and an agent would hit it.
5. **Undo receipts are one step deep.** After a replacement, no receipt removes the price (call 66). The first receipt's clear is refused once the row has moved on (call 71). That refusal was warranted and recoverable, because it named `--if-revision 2`. A person would hit this too.
6. **`price status` and `shop render` default to Standard and say nothing about other leagues** (calls 54, 70). Allflame's three bad notes appeared only when I asked for that league by name (call 56). A person would hit it.
7. **`price show` prints no revision for a target with no row** (call 61), while `price set --help` says to name what `price show` printed (call 62). A person would hit it.
8. **Store size changed under a read-only account.** The size went from 54.4 MB (call 5) to 56.3 MB (call 34) with no explanation, before I had written anything to that account. I am not sure this is a fault, so I flag it. It shows up for both a person and an agent.
9. **An empty plan file applied twice gives the same output both times** (calls 32, 33). The tool says nothing about whether the plan was spent or empty. Minor.
10. **Mock data disagrees with itself.** The character level and class differ between the list (call 10) and the fetch (call 15), and the tab index differs between the listing (call 11) and the fetch (calls 14, 16). The store keeps the list's value without comment (call 19). Only the mock is affected, but the store does not flag the disagreement.

Refusals that were warranted and recoverable, with no fault: call 17 (pick an account), call 24 (policy exists), call 71 (stale revision), call 86 (cleared row). Call 40 (unknown describe entry) is a good refusal: it listed every entry.

## 4. What the README and the help did not say

- **How old the facts are, and where the oldest and newest came from.** I expected this in the README's store tour lines or in `store status`. It is only in every `search` scope line, which I found by luck at call 42.
- **`characters` has no `--league`.** I needed to know which characters are in the new league. The README's tour shows `[--realm R]` only and the verb's help confirms it (call 13). I sorted by the league field by hand (call 10).
- **The README says a job verb prints GGG's JSON, but `leagues` prints a table** (call 8). The verb's help says so; the README's "What needs the daemon" does not.
- **The query language.** The README gives one line, "`--describe` is the language". The full index is 49.8 KB (call 39). The compact list of entry names came only from an error (call 40). The README's tour never names `links`, `category`, `pseudo.*`, `line(…)` or `holds`.
- **`pseudo.total_life` counts Strength.** I needed this for "rings with life". It is in the `describe` entry (call 41) and was confirmed by a Strength-only ring in the results (call 47).
- **`--limit` caps a `--cross` table's cells.** I expected it in the README's `search … --cross` line. I learned it at call 50.
- **There is no revision to read before a first price write.** The 0 convention is in `price set --help` only, not in `price show` (call 62 vs call 61).
- **The `basis.intent` revision moves with every price and policy write.** It is useful for detecting another writer. The README is silent; I found it in `describe` (call 41) and in call 79.
- **Stopping the daemon without a keyring logs you out.** The README says so only in a parenthesis in the tour. The verb says it only afterwards (call 90).
- **A job command with no login still spawns a daemon.** This contradicts the README's "Log in" ("refuses to start a daemon … without one"); see call 94.
- **The plan gives no duration.** The README's "A new league" says to read "its work and its ceiling". The plan gives "wire: 5–15 sends" and "quote … ready", and no time (call 27).
- **Pointers I could not follow, and whether they mattered:** `CLI-REFERENCE.md` might have held the query language whole, which mattered for T4. The Knobs table's `*.rs` links, `decisions/<area>.md` and `AGENTS.md` did not matter.

**Said, but never needed:** install, the checksums, macOS quarantine, Windows shells and `ACQ_SHELL`; the MCP section; most Knobs (rails, `ACQ_MOCK_DEGRADED_HEAD`, retention, `ACQ_HELLO_WITHIN`, `ACQ_STARTUP_WITHIN`, `ACQ_ANNOTATION_WAIT`); `store import`/`rebuild`, `demo`, `submit`, `set-priority`, `web`, `dash`, `--trade`. The README's "the first plan is the listings" did not happen here, because the listings were fresh from T1 (call 27).

## 5. The tool's own words

- **Right and useful:**
  - The `search` scope line, including "2941 locations were never fetched: nothing here says what they hold" (calls 42, 45).
  - The undecided explanations: "no class table for realm poe2 … a refresh will not help; a reference update may" (call 47).
  - The bad-note reasons (calls 55, 58).
  - The shop's position warning (call 70).
  - Every receipt naming its undo (calls 63, 64, 66, 72, 73).
  - `daemon stop` saying who was logged out (call 90).
  - The account-pick refusal (call 17).
- **Wrong or misleading:**
  - The offer from `policy show` (call 22).
  - `kind: "not_logged_in"` for an unknown account (call 83).
  - "re-read and retry" (call 24), which is the wrong advice when the person wanted the write refused.
  - `store status` offering `acq leagues` for an account with no login (call 34).
  - The README's "refuses to start a daemon" against what happened in call 94.
- **Missing:**
  - No words in the help for `stash`'s `<ID>` and `--sub` (call 12), or for `shop render --realm` (call 69).
  - No duration in plans (call 27).
  - A 404 not phrased as "no such tab" (call 84).
- **Read and never used:** the `jq` recipes in `refresh --help` (call 26), which I could not run under this seat. Also the `--next acq1.…` cursors (calls 45, 47), the `--routes` hints and the `granted` line (call 5).
- **Words I did not understand, or understood only late:**
  - "contract", a digest explained in `--help` (call 1).
  - "rails" and "tripwire".
  - "unlifted_items" and "withheld_responses" (call 78), never explained.
  - "intent" in `basis`, understood only via call 41.
  - "guardian" as a container (call 42).
  - "residue", explained by `price list --help` (call 59).
  - "wire sends" versus "logical requests" (call 31).

## 6. What I would ask of the tool

- `policy show` with no policy: offer `policy set … --if-revision 0 --account X`, naming the leagues from `acq leagues` (call 22).
- Check for a session before spawning a daemon for a job command (call 94).
- `kind: "unknown_account"` for a name no store has (call 83).
- A 404 on `stash <id>`: "no such tab in <league>", with an offer of `acq stashes --league <league>` (call 84).
- `price status` with no `--league`: one line per league that has any price state (calls 54, 56).
- `price show` prints "revision 0 (no row)" for an unpriced target (call 61).
- A replacement's receipt also prints the clear that returns the target to unset (call 66).
- `store status` prints the newest and oldest fetch, as `search`'s scope line does (call 34 vs call 42).
- `characters --league L` (call 13).
- The plan prints an estimated duration next to its request count (call 27).
- `refresh --apply=FILE` says "already spent" or "empty plan" distinctly (calls 32, 33).

## 7. Left out

- **Not tried:**
  - `--max-requests` actually refusing, and the second apply of a non-empty plan file (T2: nothing was left to plan).
  - `acq web` and `acq dash`, the interfaces the brief said to try once.
  - `store events`, `price list --game-text residue` (1,395 items), `--trade` and `show --body`.
  - `jobs --all`, `result` and `cancel`.
  - `auth check`, `auth logout` and `version --json`.
  - A usage-error case (`kind: "usage"`, exit 2).
  - Logging in again after T8.
- **Files I opened beyond the two:** none. One output, `search --describe` (call 39, 49,833 bytes), was too large for my harness to show in full. The harness kept a copy outside W, and I did **not** open it.
- **Files I wrote:**
  - `W/work/plan-rehearsal.json`, via `--out` (call 31).
  - `W/drafts/report-tail.md`, this part: the harness refused this heredoc.
  - The report through the door.
- **The cut I would most want back:** a real, non-empty plan file applied under a ceiling and then handed over a second time. That is the "careful way" the person asked for, and this world's empty Rehearsal league left it untested.
