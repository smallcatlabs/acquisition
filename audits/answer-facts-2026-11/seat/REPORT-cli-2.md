<!-- written during the run; call numbers are rows of calls.csv -->

## 1. Before touching anything

**What the brief told me (not counted as the tool's or README's):** acq is the CLI of Acquisition, a PoE inventory tool; the store holds GERWARIC#7694 (a ~50 MB copy, no session) and I log in a second account through the door; the world has no keyring; the door's daemon idles out after 30 min; every command goes to a mock (the brief).

**What the README told me (before any command):**
- Can do: fetch stash tabs/characters through a daemon (`acqd`) under GGG's rate limits; search, show, price, render forum shop pages offline; keep a league fresh with a sync policy + plan/apply; an MCP server with 26 tools.
- Needs: a login (`acq auth`, browser; `--no-browser` prints the link) for anything that fetches; the daemon for job verbs, `auth`, `jobs`, `status`, `cancel`, `daemon`; store/search/price/shop/policy-write verbs need no daemon and no network. Without a keyring "a login lasts as long as the daemon that holds it".
- Before a first command ("Before the first command"): the rate limit is the clock (30 tabs / 5 min seen; a first sync of a few hundred tabs takes most of an hour); verbs default to Standard without `--league`; data location; after an update the next job command replaces the daemon.
- New league: `acq leagues --stored` (offline) then `acq leagues` to fetch; first policy with `--if-revision 0`; when adding a league write the realm back whole (naming only the new league drops Standard); then `refresh --plan --league` / `--apply --league` until nothing to do; read each plan's work and ceiling first.
- Pointers I could not follow here: `AGENTS.md`, `CLI-REFERENCE.md`, `decisions/<area>.md`, `SECURITY.md`, `CONTRIBUTING.md`, and the source files the Knobs table cites. CLI-REFERENCE.md is the one that would have mattered (the tour says it "holds every one").
- What I would not run without asking the person: `policy set --replace` or a policy write without `--if-revision` on a real account (a realm is written whole, so it can silently drop leagues); `refresh --all`/`--apply` on a large league against GGG (hour-long, spends their rate budget); `daemon stop` while a login is held without keyring (it drops the login); `daemon reset-tripwire`; `store rebuild`/`store import`; `auth logout`; `price clear`/`price set` on real items; `diagnose` output posted anywhere.

**What the tool told me, looking at the world unchanged:**
- `acq --help` (call 1): the verb list; notable that `--json` claims to be total, and `--account` defaults.
- One account on disk: `GERWARIC#7694`, "last sign-in or token refresh 19d ago", "not persisted", store 54.4 MB (call 2). No live session; daemon not running (call 3, call 4: "daemon is not running; no queue on disk").
- Store (call 5): path `W/store/mock/GERWARIC_7694.db` (54.4 MB) — note `store/mock`, i.e. this copy lives under the mock provider; responses 1126, characters 65, tabs 3444 "(every realm and league)", items 22721 (+6 removed), events 22751; 8 granted skill entries; "refused: 4 bodies". It gives no age for any of it.
- No league list on record (call 6: "no league list on record; `acq leagues` fetches it"); store status agrees ("league ids seen … 0", call 5).
- No sync policy (call 7), which offers the exact `--if-revision 0` command, and lists the leagues held on disk: pc Allflame, Ancestors, Hardcore, Phrecia 2.0, Solo Self-Found, Standard, Ziz Rapture HCSSF Class Gauntlet; poe2 Runes of Aldur, Standard — all "(no policy)".

## 2. The tasks

### T1 — log in and look around

**What I would tell the person:** You are logged in as `seat` (mock provider), alongside the stored, session-less `GERWARIC#7694`; no keyring, so the login lives only as long as the daemon (call 9). Before going to the API the tool knew **no** leagues for either account — "no league list on record" (calls 6, 12) — so there was nothing to age and nothing the policy named (there is no policy, calls 7, 13). Fetched afresh (call 14): Standard and Hardcore (started 5032d ago) and **Rehearsal** (pc, category Rehearsal, started 4d ago) — the new one. What changed: from nothing to three leagues; `--stored` now says "league list fetched 2s ago" and each league "no policy, nothing on disk" (call 15). Characters: two, both in **Standard** (StashHoarder, Scion 97; MuleQuadTab, Witch 12) — you have **no character in Rehearsal** (call 16). Rehearsal stash tabs: 8 rows — Currency, Dump, folder "Old leagues" (holding Old 1, public, and Old 2, Quad), Uniques, Maps, Maps (empty) (call 17; counted by call 21). One tab: Dump is empty (call 19), and Old 1 is empty too (call 20). One character (Standard, since none is in Rehearsal): MuleQuadTab holds one Helm and four main-inventory items, all "Fake Item N" normals (call 18) — but the character body says **Scion level 97** while the list said **Witch level 12** (call 16 vs call 18); I cannot tell whether that is the mock's or the tool's, and I would tell the person the two disagree.

**Commands:** 8 (login), 9–21 (13 runs; 10 was help, 11 a refusal).
**Right?** Yes for who/leagues/tabs; the character's class/level is contradictory between two answers. A stored tab listing with ages (call 21: Dump "7s ago", Old 1 "3s ago", the rest "never") agrees with what I fetched. First call held right: call 9.
**Where the tool stopped or misled me:** a store verb refused once two accounts existed (call 11) — warranted, recoverable (it named both and the flag). The fetched `leagues` text (call 14) gives no fetch time; only `--stored` does (call 15). Job verbs print raw JSON with no human form even without `--json` (calls 16–20), and their documents carry `rate_limit` but no time of fetch. `stash` (calls 19, 20) reports `"index": 0` for tabs the list placed at 1 and 3. The login's JSON (call 8) printed two documents on stdout (an `authorize_url` line then the result) — that is the door's login path, noted only.

### T2 — keep the new league fresh

**What I would tell the person:** There was no policy on `seat` (call 13), so I wrote one that covers Rehearsal (all tabs, all characters, refetch after 3600 s) with `--if-revision 0`, which only writes where no policy exists (call 23: "sync policy written (revision 1)"). Writing it a second time the same way was refused without overwriting: "a sync policy stands at revision 1; 0 writes only where none is stored: name 1 to replace it" (call 25). The stored policy is exactly that one league, "revision 1, updated 2s ago". It also notes that Standard is held but not named (call 24). The policy covers **only** Rehearsal, which is what you asked for. Standard is not kept fresh.
First plan (call 27): **5 requests, "5..15 wire sends"** (plus a HEAD probe per route per daemon lifetime and a possible token refresh). It fetches 5 never-fetched tabs: Currency, Old 2, Uniques, Maps and Maps (empty). It **leaves out 3 tabs**: 2 because they are "fresh" (Dump and Old 1, which I fetched in T1) and 1 because it is a folder ("container only"). It covers no characters, because none is in Rehearsal. Its facts: "stash listing 40s old (response 4), character listing 42s old (response 3)". The quote says "ready now", with the limiter's counters "seen 30s ago". It gives **no time estimate** beyond "ready now", and says it does not cover 429 re-sends or a token refresh. Applying it (call 28) ran job 12 with 5 requests, all done (jobs 13–17): "changed: no items — 5 responses recorded … left: nothing". It landed: `tabs` now shows 7 fetched and 1 folder, with the five new tabs "4s ago" and 0 items each (call 29). The plan asked for again says "nothing to do in Rehearsal" (call 30). The help warned that a first sync with map/unique tabs takes two rounds (call 26), but it took one here, I assume because the mock's map and unique tabs are empty.
The careful way: I saved the plan as a file (call 31 → `W/work/plan1.json`), but by then it had **0 actions** and `logical_requests: 0`. Applying it with `--max-requests 3` (call 32) and handing it over again (call 33) both printed "nothing to do in Rehearsal … facts: stash listed 53s ago". Neither the first nor the second apply said whether the file had been spent; the help says a re-apply prints `applied: false` / `spent` in JSON. How I would have done it with work left: wait until the tabs are older than 3600 s (or write a shorter `max_age_seconds` with `--if-revision 1`), save `refresh --plan --json` to a file, read `logical_requests` and `actions`, apply it with `--apply=FILE --max-requests N`, and apply it again to see whether "spent" is reported. I did not change the person's freshness setting just to make work.

**Commands:** 22–33 (12; 22 and 26 were help, 25 the deliberate refusal).
**Right?** Yes. The tabs listing (call 29) agrees with the plan and the apply. One thing would show it is wrong: a tab with items in the game showing 0 here. I cannot check that against a mock. First call held right: call 23.
**Where the tool misled me:** "facts: stash listed 53s ago" was printed identically at calls 32 and 33, seconds apart. It is the age at plan time, labelled as "ago" now. The `--json` plan gives its basis as epoch seconds (`fetched_at`), so finding the age takes arithmetic, and it also wrote a note on stderr (call 31). The README's new-league recipe says the first plan is "the listings, one or two". Here the first plan went straight to the tabs, because my T1 fetches had already made the listings.

### T3 — what I have

**What I would tell the person:**
- *The store as a whole* (call 36): 22721 items (+6 removed), 3444 tabs "(every realm and league)", 65 characters, 1126 responses, in a 56.3 MB file. In call 5 the same file was 54.4 MB, and I wrote nothing to this account in between. It says **nothing about age**. It names no leagues. It says "league ids seen across every league fetch: 0 (never fetched)". It refused 4 bodies and points at `acq store refused`, which shows all 4 (call 41). They are poe2 character bodies from 62d ago, each "missing an `id` on an item at `equipment[N].socketedItems[0]`". The tool's ages exist only per tab (call 37), per character (call 40) and per event (call 39). I could not get an oldest or newest for the whole store from any one command. Pieced together: characters were fetched as long ago as 57d (the Allflame four) and listed 53d ago (call 40). The newest events are 53d ago: "ACQUISITION-PRICE-TEST" changed and moved in, and AllEarsRacer moved out (call 39). `store events --summary` with the default window said only "0 events in the last 24 h" (call 38). I had to widen it with `--hours 100000` to see anything.
- *One league's tabs, Standard* (call 37): 3347 tabs, 405 fetched, 2926 never fetched, 16 folders, 20231 items. The ten largest tabs are each "56d ago", and the first line gives no league-wide age. It names the other leagues it holds: Allflame (97 tabs, 4 characters), Ancestors, Hardcore, Phrecia 2.0, Solo Self-Found and Ziz Rapture HCSSF Class Gauntlet (characters only), all "no policy". It does not mention the poe2 realm that call 7 listed. It says nothing about refusals. Of its tabs, 2926 have never been fetched, so the item count covers only 405 of the tabs.
- *Prices' standing, Standard* (call 42): 21311 items: 492 "the game decides", 0 by hand, 20819 "nothing applies". This is the one answer that gives oldest and newest: "3372 locations, 446 fetched (the oldest fetch 56d ago, pc · Standard / Keepers (Remove-only); the newest 53d ago, pc · Standard / Dump Tab); stash listing 53d ago, characters listed 53d ago". It names the same other leagues as `tabs`. It points at 1395 items carrying price text the trade site cannot see, 3 `~skip` items and 1 invalid note, each with a `price list` command. It says nothing of refused bodies.
- Item counts disagree across scopes, and no answer explains why. `tabs` says Standard holds 20231 items (call 37), `price status` says 21311 items in Standard (call 42), and the whole store holds 22721 (call 36). I take the difference to be characters' items, since `price status` counts 3372 locations against 3347 tabs, but no answer says so.

**Commands:** 34–42 (9; 34–35 help).
**Right?** I believe the counts. "How old" for the whole store I hold only partly: the oldest fact in the store is the 62d-old refused bodies or the 57d-old character fetches, depending on what counts, and no command says which. First call held right: call 37.
**Where the tool stopped me:** `store status`, the obvious answer to "how old is what you know", carries no time at all. The events window defaults to 24 h, which on a 53-day-old store looks like "nothing happened".

### T4 — find things

**What I would tell the person:**
- (a) Six-link gear: **yes, 187 items** in pc (call 48). The first ten are body armours and a bow, mostly on Standard characters (Skin of the Loyal, Tabula Rasa, a 6-link Astral Plate on VVickedWilliam…). The matches range from 57d old (Allflame / TheAbsenceOfPatience) to 53d old (Standard / Dump Tab). The scope line adds that the store's "location list [was] seen 57d ago" and that 2940 locations (2489 map tabs, 437 unique tabs, 14 characters) were never fetched, "nothing here says what they hold". There may be more 6-links in those.
- (b) Rare rings with life and at least two resistances: **224** (call 50). The ages are the same kind as in (a): oldest 57d (Allflame / TheAbsenceOfTotems), newest 53d (Dump Tab). The query was `class:ring rarity=rare pseudo.total_life>0 pseudo.count_res>=2`. Caveat for the person: the tool's "total life" (call 46) also counts half of any Strength, so a ring with Strength and no life line would match. And `count_res` counts "+12% to all Elemental Resistances" as three (call 53). My query is the trade site's notion, not a strict "has a life line". My first try, `has:line("# to maximum Life")`, was refused: "`line` is no field the language knows" (call 49). The language describes `line("…")` only as a sort value (call 46), and I found no way to say "has this line" in what I read.
- (c) Per league and rarity (call 51): Standard 21311 (7693 with no rarity, 5615 rare, 2850 normal, 2819 unique, 2334 magic). Allflame 1312 (561 / 154 / 276 / 257 / 64). Ages: oldest 57d, newest 53d. No other pc league holds a live item, and the poe2 realm was out of scope. **Agreement with elsewhere:** `tabs` says Standard holds 20231 items (call 37) and Allflame 1101 (call 52). Search says 21311 and 1312. `price status` agrees with search for Standard (21311, call 42). The gaps are exactly the characters' items (Allflame's four characters hold 69+44+53+45 = 211 in call 40, which is 1312 − 1101). `tabs` counts only tab items and does not say so.
- (d) One item, Kraken Circle Bone Ring (call 53): rare Bone Ring, ilvl 78, requires level 64. It is equipped on Standard / undxot, fetched 56d ago, and unpriced ("nothing applies"). It has six lines with their templates and numbers, and "unread nothing". The answer carries the age of this item's location (56d), not of the store.

**Commands:** 43–53 (11; 43–46 language, 47 and 49 refusals).
**Right?** I hold (a), (c) and (d) right, and (b) right for the trade site's meaning. Each search answer gives the age of **the matches** (oldest and newest location) separately from the scope's "location list seen 57d ago", which is what the person wanted. First call held right: call 48.
**Where the tool stopped or misled me:** the default `--describe` index (call 43) is 33.5 KB, more than my harness showed inline, so I asked for slices instead. A search without `--realm` was refused (call 47); the refusal was warranted and recoverable, since it names the three choices. The README tour does say a store of two realms needs `--realm`. "Location list seen 57d ago" (calls 48, 50, 51) does not match `price status`'s "stash listing 53d ago" (call 42) for the same store; I cannot tell which listing each means. `tabs --limit 0` printed a dangling header "the 0 with the most items:" (call 52).

### T5 — prices

**What I would tell the person:**
- **What you have priced** (Standard, calls 42, 57, 61): nothing by hand ("0 priced by hand"). 489 items are priced in game in public tabs, and 492 are "the game decides", which counts the 3 `~skip` items too. They come from 17 priced tab names (3 of them public) and notes (call 61). The first listed group is a substash named "~price 4554 chaos" holding 9 claws, plus an "ACQUISITION-PRICE-TEST" tab (call 57). 1395 items carry price text the trade site cannot see, either on characters or in non-public tabs (call 42).
- **Price text the tool cannot use:** one note, on **Thaumaturge's Incubator x3** (item 4aefe3b5…) in tab "ACQUISITION-PRICE-TEST": `"~price  chaos"` "reads invalid: the amount is empty" (calls 55, 58). Fix it in the game. There is a second kind that `price status` **does not count**: a tab name the tool reads as invalid. Tab "~b/o 5000/2 chaos" (d4a7b4ce8c) "reads invalid: a ratio (5000/2) in a tab name is invalid and lists nothing" (call 59). The tab "~price 1000/2 chaos" (ffaab719d0) is very likely the same, though I did not open it. I found these only because a `~skip` item happened to sit in such a tab. No `--game-text` choice lists them (call 60).
- **"An item the game skips"**: a `~skip` note on an item in a public tab, which states that the item is not for sale. It overrides its tab's name, and a hand price set nearer the item still applies (call 61). There are 3: Ornate Incubator x1, Horror Mark Titanium Spirit Shield and Springleaf Plank Kite Shield (call 56).
- **Writes:**
  - Tab "Delve" (tab/pc/2f01a16e65, 30 items, not public), set to 5 chaos with `--if-revision 0`. Receipt: "5 chaos (revision 1), was unset". It gives the undo `acq price clear … --if-revision 1` (call 65). Read back: "by hand: 5 chaos (own row, revision 1, set 32s ago) … 30 items — 30 by hand only" (call 74).
  - Item Vengeance Wing Imperial Bow (item/a2a93391…, in "Market (Remove-only)", not public), set to 2 divine with `--if-revision 0`. Receipt: "2 divine (revision 1), was unset", with a `price clear … --if-revision 1` undo (call 70). I read it at revision 1 (call 71). The replacement `--if-revision 1 … exact 3 divine` landed: "3 divine (revision 2), was 2 divine (revision 1)". Its undo is `price set … exact 2 divine --if-revision 2` (call 72). Read back: "3 divine (own row, revision 2, set 3s ago)". The game note "~price 50 chaos" is still there, but it "decides nothing" in a non-public tab (call 73).
  - **What each receipt lets you undo:** only that one write, back to the prior state. The second undo restores 2 divine, not "unset".
  - **Whether the thing priced is in the facts:** the receipts say nothing either way. The help says a note is printed only when the facts hold no such target (call 64), so silence means "found". I would rather it said so. The read-backs name the tab and its item count, which shows the targets exist.
  - **Another program changing a price between my read and my write:** `--if-revision N` with the revision I read refuses and names the current revision (help, call 64). I test that in T7. The README calls this "with undo" but never mentions the revision guard on prices.

**Commands:** 54–74 (21; help at 54, 60, 64, 68; 66 and 67 refused while I guessed the flag syntax).
**Right?** Yes: both prices read back as written (calls 73, 74). First call held right: call 55.
**Where the tool stopped or misled me:** `-priced` (call 66) and `-is:priced` (call 67) were refused. The first refusal offered `name:priced`, `"priced"` and `line(template:priced)`, none of which was what I meant; the right `-has:priced` showed up only in `--describe=priced` (call 68). The second refusal did list every `is:` word, but not the `has:` form. Search's "location list seen" was 57d with all leagues in scope (call 48) and 53d with `league=Standard` (call 69). That fits as the age of the scope's oldest listing, but no answer says so.

### T6 — the shop

**What I would tell the person:**
- Without a policy (call 76): **1 page**, 2319 characters, 31 items. Your 5-chaos price is the spoiler `~price 5 chaos`, holding 30 links to "Stash11", the tab "Delve". Your 3-divine price is the spoiler `~price 3 divine`, holding one link to "Stash133", the tab "Market (Remove-only)", at x 1, y 3. Left out: 492 items, of which "489 the game lists" (already priced in game, so the trade site lists them without the forum) and "3 the game skips". Blocked: 0. "Nothing applies": 20788. The rest of Standard's 21311 items have no price. Delve shows 30 items, but its links skip x = 17, 21, 28, 29, 30 and 34, which I take to be empty cells. Ages: the posted items were fetched 56d ago (both locations). "Stash listing 53d ago". The page also warned: "31 items on the page seen before the current stash listing, whose indices the links use; a tab reindexed between would move the link — a refresh re-reads them". "policy: none set, so no refresh covers the page's facts".
- **What to do next:** "paste page 1 of 1 into the shop thread and preview it before posting". The README says that is your league's shop subforum, and not to move the items afterwards.
- With a policy covering Standard (written with `--if-revision 0`, revision 1, call 77; it noted "the league list was never fetched, so Standard is not read against it"): the same page (call 78). The only change is in freshness. The "policy: none" line became "stale: 31 items on the page last seen past the policy's window of 3600s (oldest covered stale item 56d ago); `acq refresh --plan …` would send 3374 requests", and the other leagues changed from "no policy" to "not named". It still renders. Help says freshness is "reported, never enforced" (call 75).
- **Cleanup**, run exactly as printed:
  - The item's removal undo printed by call 70 (`price clear … --if-revision 1`) was **refused**: "has a price at revision 2, not the revision named: `--if-revision 2` clears it as it stands" (call 79). The refusal was warranted, since my own call 72 had moved the row on, and it was recoverable. I then ran its offered `--if-revision 2`: "cleared, was 3 divine (revision 2)" (call 80).
  - The tab's undo printed by call 65 landed: "cleared, was 5 chaos (revision 1)" (call 81).
  - Read back: the bow is "by hand: nothing" (call 82), and Delve is "by hand: nothing … 30 unlisted" (call 83), as found in call 63.
  - **Not put back:** the Standard policy on GERWARIC#7694 (revision 1, call 77) is still there. The brief asked me to clean up the prices only.

**Commands:** 75–83 (9; one door invocation of mine was malformed, `--page` before `--`, and refused by the door before running anything; it has no call number).
**Right?** Yes. First call held right: call 76.
**Where the tool misled me:** an undo printed by a receipt goes stale as soon as you write again (call 79). "The undo" for the item depends on which receipt you take: the latest (call 72) would have put 2 divine back rather than removing the price. The help says "every omission counted with its reason" and it did count them, but "nothing applies" (20788) is not something one would call an omission.

### T7 — for a program

**What I would tell the person:** All five reruns, and all three failures, produced one JSON document on stdout. The ages are uneven, though.
- T1 `leagues` (call 84, rerun of call 14) is one document, `{job_id, account, outcome, payload}`. It carries **no time at all**: no fetch time and no age. The only times in it are the leagues' `startAt`, given as ISO timestamps with no age.
- T3 `price status` (call 85, rerun of call 42) is one document. Every time in it has a ready-made age beside it: `taken_at_age` "0s ago", `stash_listing_fetched_at_age` and `character_listing_fetched_at_age` "53d ago", and `facts.oldest_fetch_age` "56d ago" and `newest_fetch_age` "53d ago" with their locations. These are the ages of the league's facts, which is what I asked about. `list_seen_age` is "53d ago" too.
- T4 `search links=6` (call 86, rerun of call 48) is one document. `total.oldest_fetch_age` "57d ago" and `newest_fetch_age` "53d ago" are the **matches'** ages. `scope.*_age` covers the whole realm, which is wider. `snapshot_at_age` is "52d ago". The `basis.snapshot.at` inside the `next` request has no age beside it, so a program would need arithmetic there. `list_seen` equals `oldest_fetch` exactly (1788810111, a character fetch), which makes me doubt that "location list seen 57d ago" is the age of a listing at all.
- T5 `price show tab/…` (call 87, rerun of call 63) is one document. `basis.at_age` gives "53d ago" for the tab (the stash listing) and "56d ago" for each item (that item's own response). Both are the thing's own ages. The document is 14 KB for a 30-item tab, with 10 items returned and `left_out: 20`.
- T6 `shop render` (call 88, rerun of call 78) is one document, with `rendered_at_age`, the listing ages and `freshness`. Because I ran it after the T6 cleanup, it has 0 pages and 0 posted, so `freshness.stale` is empty. **My ordering cost the comparison.** The write case was not rerun as JSON (see "Left out"). The plan JSON from T2 (call 31) carries epoch times only (`generated_at`, `fetched_at`) and no ages.
- Failures, text then JSON:
  1. **Wrong account.** Text: stderr, "no login made a store for `NOBODY#0000`; the names known: GERWARIC#7694, seat (see `acq accounts`)", exit 1 (call 89). JSON: stdout, one document with `kind: "not_logged_in"`, `accounts` and two `offers`, each with `argv`, exit 1 (call 90). It says what to do next. The kind `not_logged_in` is a misleading word for "no such account".
  2. **A stash tab that does not exist, fetched through the daemon.** Text: stderr, "job 19 failed: GET /stash/Rehearsal/rh-nosuch returned 404 Not Found; no rate headers; a body of 25 bytes", exit 1 (call 91). JSON: stdout, `{job_id, account, outcome: "failure", error}`, exit 1 (call 92). It is one document, but it has **no `kind` and no `offers`**. Nothing points to `acq stashes` or `acq tabs` to find the right id.
  3. **A price write naming the wrong revision.** Text: stderr, "price was cleared (revision 2); a new price names revision 0, or none", exit 1 (call 93). JSON: stdout, `kind: "conflict"`, `revision: 2`, with one offer that is the same write with `--if-revision 0` and its `argv` (call 94). It says what to do next.

**Commands:** 84–94 (11).
**Right?** Yes, these are the documents as printed. First call held right: call 84.
**Where the tool fell short:** the README says "Every command takes `--json`, and it is total". That held for every command I tried, but a job verb's failure has neither the `{error, kind, offers}` shape nor any offers. Age fields are applied inconsistently: the store answers have `_age` strings, but the job answers and the plan envelope carry none.

### T8 — when things go wrong

**What I would tell the person:**
- **What the daemon had done, before I stopped it.** It was 1.0.0-alpha.1 (contract 7cd97eef58c2), pid 44414, "up 370s", provider mock, "idle exit after 1800s" (the door's knob). It reported "sends 22 since this daemon started", "policies learned: 6", and gave its log path and journal path (call 95). `acq jobs --all` listed 20 jobs: 18 done and 2 failed (my deliberate 404s). Jobs 1, 4, 6, 8 and 10 were the daemon's own profile job and its four probes (call 97). "Since when" is given only as an uptime ("up 370s", `uptime_seconds` 378 in call 99), not as a clock time.
- **What it knows of the API's limits.** `acq dash` refused without a terminal: "stdout is not a terminal — use `acq dash --json`" (call 98). The JSON snapshot (call 99) gave six policies, each with windows (hits/max/period/restriction), `next_safe_in_seconds` and `last_observed_seconds_ago`. Examples: stash 15 per 10 s and 30 per 300 s; stash list 10 per 15 s and 30 per 60 s; character list 2 per 10 s and 5 per 300 s; token 60 per 30 s per IP. It also listed all 22 sends with `seconds_ago`, and errors including three "keyring save failed: disabled by ACQ_NO_KEYRING".
- **Version and logs.** `acq version`: "acq 1.0.0-alpha.1 (contract 7cd97eef58c2), provider mock", plus the sibling acqd path and size (call 96). The running daemon reported the same version and contract and the acqd sha256 prefix 42a487d3adf3 (call 95). Logs are at `W/logs/774371aaab7d/mock/daemon.log`, with the send journal `sends.jsonl` beside it (calls 95, 103). I did not open them.
- **Stopping.** "daemon stopped (pid 44414) / logged out with it: seat — not in the keyring, so the session lived in that daemon alone; `acq auth` logs in again" (call 100). With no daemon:
  - A **store-only** verb works unchanged: `tabs` printed the same listing, now "5m ago" (call 101).
  - **Observers** answered without starting one: `jobs` said "daemon is not running; nothing waits on disk" (call 102), and `daemon status` said "on disk: 0 waiting, 0 recorded running, written 45s ago" with the last log path (call 103).
  - An **API** verb, `leagues`, failed with exit 1: "not logged in — run `acq auth`; a daemon was started for this command and runs until idle — `acq daemon status` shows it, `acq daemon stop` ends it" (call 104). So it **started a fresh daemon even though it could not use it**. That daemon showed "up 2s … sends 0" (call 105). I stopped it (call 106) and confirmed it was stopped (call 107: "daemon is not running").
- I did not run `acq diagnose`.

**Commands:** 95–107 (13).
**Right?** Yes. First call held right: call 95.
**Where the tool stopped me:** `dash` needs a terminal (warranted, and it offered `--json`). The API verb without a login spawned a daemon it could not use, so a person who only wanted the error message is left with a background process running for 30 min (or an hour by default) unless they stop it.

## 3. Faults

Worst first. "Both" means a person at a keyboard would hit it as well as an agent.

1. **No age for the store as a whole.** `store status` gives counts, size and refusals but no time at all (calls 5, 36). The tool can compute the oldest and newest fetch, and does so for a league in `price status` (call 42) and for matches in `search` (call 48), but nowhere for the whole store. To answer "how old is what you know" I had to piece it together from three commands (calls 39, 40, 41). It should print the oldest and newest fetch with their locations, and the age of the refused bodies. Both.
2. **One league's item count differs between verbs, and none of them says why.** For Standard, `tabs` says 20231 items (call 37), while `price status` and `search --cross` say 21311 (calls 42, 51). For Allflame it is 1101 against 1312 (calls 52, 51). The difference is exactly the characters' items. Every verb is internally consistent, but `tabs`' first line, "20231 items" for "3347 tabs in Standard", reads as the league's total. It should say "in tabs" and name what it leaves out. Both.
3. **Invalid tab-name prices are not counted.** `price status` counts 1 invalid note (call 42), but a tab named "~b/o 5000/2 chaos" "reads invalid" (call 59) and appears in no count. `price list --game-text` has no value for it (call 60). A person asking "what price text can't you use" gets an incomplete answer. Both.
4. **The applied-plan age is frozen but labelled "ago".** `refresh --apply=FILE` printed "facts: stash listed 53s ago" at call 32 and again, identically, at call 33. The age is relative to plan time, not now. Text output also does not say whether the file was "spent" or what `--max-requests` meant when the plan had 0 requests. Both.
5. **A failed job's JSON has no `kind` and no `offers`.** The README calls `--json` "total", with failures as `{error, kind, offers}`. A 404 fetch gave only `{job_id, account, outcome: "failure", error}` (call 92), and its text form (call 91) has no next step either. Warranted? The request was wrong (no such tab). Enough to recover? No: nothing points to `acq stashes`/`acq tabs` for the right id. Both, though it matters most to a program.
6. **An API verb with no login starts a daemon it cannot use** (call 104), and leaves it running until idle (1800 s here, 3600 s by default). Warranted refusal: yes, I had no session. Enough to recover? Yes: it says `acq auth` and how to stop the daemon. The spawn itself is the fault. Both.
7. **Job verbs carry no time.** The JSON from `leagues`, `characters`, `stash` and `character` has `rate_limit` but no fetched-at and no age (calls 14, 16–20, 84). The plan envelope gives epoch seconds only (call 31). Only the store verbs give `_age` strings (calls 85–88). Mostly an agent.
8. **"Location list seen 57d ago" is likely mislabelled.** In call 86 `list_seen` equals `oldest_fetch` exactly (1788810111, a character fetch), while `price status` says the stash listing is 53d old (call 42). Both.
9. **The character list and the character body disagree.** The list says MuleQuadTab is a Witch, level 12 (call 16); the fetched body says Scion, level 97 (call 18). This may be the mock. The tool printed both without remark. Both.
10. **A search refusal suggests the wrong fix.** `-priced` was refused with offers `name:priced`, `"priced"` and `line(template:priced)` (call 66), none of which is the flag form `-has:priced` (call 68). Warranted: yes, my syntax was wrong. Enough to recover: no. Both.
11. **A receipt's undo goes stale after the next write.** The undo printed by call 70 was refused after call 72 (call 79). Warranted: yes. Enough to recover: yes, it named `--if-revision 2`. Listed only because "with undo" in the README suggests an undo you can run later. Both.
12. **An unknown account gets `kind: "not_logged_in"`** (call 90). That is the wrong word for "no such account"; the text itself is good. Agent.
13. **The store file grew from 54.4 MB to 56.3 MB** between calls 5 and 36, with no write of mine to that account in between (I had only read it, and written to `seat`). It may be WAL or derived columns. The tool does not say. Both, but unremarkable.
14. Minor:
    - `stash` reports `"index": 0` for tabs the list placed at 1 and 3 (calls 17, 19, 20), possibly the mock.
    - `tabs --limit 0` prints a dangling header "the 0 with the most items:" (call 52).
    - `store events --summary` defaults to 24 h and says "0 events" on a store whose newest event is 53 days old (call 38).

## 4. What the README and the help did not say

- `tabs` item counts leave out characters' items. Expected in `tabs --help` or the README tour's `tabs` line. Learned by comparing calls 37, 51 and 52.
- A flag field is written `has:priced` / `-has:priced`. Expected in the `search` tour line or `search --help`. Learned at call 68, after two refusals (66, 67).
- `line("…")` is not usable as a `has:` test. Expected in `--describe`. Learned at call 49. I never found how to say "has this line".
- `store status` carries no age. Expected, given the brief's question, under the README's "Your data stays on your machine" (which says `store status` prints the path). Learned at call 36.
- The default events window is 24 h. It is in the tour (`[--hours N]`) but without its default. Learned at call 38.
- An API verb with no login spawns a daemon anyway. Expected in "What needs the daemon". Learned at call 104.
- A job failure under `--json` is not the `{error, kind, offers}` shape. The README says "a job that fails is its outcome record", which is true, but not that it carries no offers. Learned at call 92.
- The first plan of a new league goes straight to tabs when the listings are already fresh. The README's "A new league" says "the first plan is the listings, one or two". Learned at call 27.
- What "location list seen" means. Expected in `search --help`. Never learned (calls 48, 86).
- `search --describe` with no argument is 33.5 KB (call 43). Neither `--help` nor the README says so; the README's "an index" sounds small.
- **README said, and I never needed:** Install and checksums, the macOS/Windows notes, PowerShell quoting and `ACQ_SHELL`, the MCP configuration and its 26 tools, most of the Knobs table (rails, `ACQ_MOCK_*`, `ACQ_*_WITHIN`, `ACQ_*_WAIT`, retention), `store import`/`rebuild`, `search --trade`, `web`, Known gaps, Licence, Reporting. Pointers I could not follow here: `AGENTS.md`, `CLI-REFERENCE.md` (the one that would have mattered, for the query syntax), `decisions/*.md`, `SECURITY.md`, `CONTRIBUTING.md`, and the source links in Knobs.

## 5. The tool's own words

- **Right and useful:**
  - The policy refusal (call 25) and the price conflict refusals (79, 93, 94), each naming the current revision and the exact next command.
  - The plan text (call 27): requests, wire-send range, basis ages, skips with reasons, a quote, and what it does not cover.
  - The shop's "positions … seen before the current stash listing" and "stale … would send 3374 requests" (calls 76, 78).
  - `daemon stop` saying the login went with it (call 100).
  - The search answer separating the matches' ages from the scope (call 48).
- **Wrong or misleading:**
  - "facts: stash listed 53s ago", frozen (calls 32, 33).
  - `kind: "not_logged_in"` for an unknown name (call 90).
  - "location list seen" (calls 48, 86).
  - "20231 items" in `tabs` (call 37), read as the league's total.
  - "492 the game decides" next to "489 priced in game" (call 42). The difference is the 3 skips, but nothing joins the two numbers.
  - "every omission counted" (call 75), which counts "nothing applies" (20788) apart from the omissions, though a reader might take those for omissions too.
- **Missing:**
  - Any time on job outputs (calls 14, 16–20, 84).
  - Any age in `store status` (call 36).
  - Any count of invalid tab names (call 61).
- **Read and never used:** `--routes`, the `--next` cursor tokens (calls 48, 50), `argv` in offers, `snapshot`/`basis` digests, the `--describe` "an answer, key by key" block (call 46), `explain` views.
- **Words I did not understand, or understood only by guessing:**
  - "residue" (calls 42, 60), "contract" (call 96), "rails" (call 95).
  - "membership live" (call 48), "derivation 12" and "intent 6" in basis (call 86), "wire sends" against "requests" (call 27).
  - "unruled_kind" (call 88), "the game decides" (call 42), "off_page" (call 88).

## 6. What I would ask of the tool

- `store status`: oldest and newest fetch with locations, the league list's age, and the refused bodies' ages (call 36).
- `tabs`: say "items in tabs" and name the characters' items it leaves out (calls 37, 51).
- `price status`/`price list`: count and list tab names that read invalid (calls 59, 61).
- `refresh --apply=FILE`: give the facts' age as of now, or say "as of the plan", and say "spent" in text (calls 32, 33).
- A failed job's JSON: `kind` and `offers`, e.g. `acq stashes --league L` for a 404 tab (call 92).
- Do not spawn a daemon for an API verb when no login exists or can be restored (call 104).
- Put a fetched-at age on every job verb's output (calls 16–20, 84).
- When a field is a flag, the search refusal should offer `has:<flag>` (call 66).
- Use `kind: "unknown_account"` for a name never seen (call 90).
- Point the events default window at the newest event, or say how old that is (call 38).
- Give `search --describe` with no argument a short index, with the full one behind `=all` (call 43).

## 7. Left out

- **Not tried:**
  - `web`, `refresh --tabs/--all/--deep`, `submit`, `demo`, `cancel`, `set-priority`, `result`, `status <id>`.
  - `store import`/`rebuild`, `store refused <id>`, `policy --replace`, `auth check`/`logout`, `diagnose`, `search --trade`.
  - The MCP server.
  - A successful price write with `--json`.
  - A careful apply that had real work, so `--max-requests` never refused or bounded anything (T2 had nothing left to do).
- **Files opened that are not one of the two:** none. My harness saved call 43's 33.5 KB output to a tool-results file of its own, and I did not open it.
- **Files I wrote:**
  - This report through the door: its first nine parts by heredoc, and this last part, refused as a heredoc by my harness, as `W/drafts/report-tail.md` with `report --from`.
  - `W/work/plan1.json` through `--out` (call 31).
  - Nothing else.
- **State I leave changed:**
  - `seat` has a Rehearsal policy at revision 1 (call 23).
  - `GERWARIC#7694` has a Standard policy at revision 1 (call 77).
  - Both price rows are cleared, and their revisions continue (calls 80, 81).
  - The daemon is stopped (call 107) and `seat` is logged out (call 100).
- **The one thing I cut and would most want put back:** the careful apply with work in it. That means shortening `max_age_seconds` or waiting out the hour, so that a saved plan with N > ceiling is refused, a plan with N ≤ ceiling runs, and re-handing the file shows "spent". As run, T2's last step proves nothing about the ceiling.
