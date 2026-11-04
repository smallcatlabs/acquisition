<!-- written during the run; call numbers are rows of calls.csv -->
# 1. Before touching anything

**From the brief.** acq-mcp is the MCP server of Acquisition, a PoE inventory tool; the store holds `GERWARIC#7694` (a ~50 MB copy, no login) and `seat` (logged in by the person, daemon running, exits four hours after its last connection, no keyring); everything points at a mock API (the brief).

**From the README alone.**
- Can do: download stash tabs and characters to the machine, search them offline (including trade-site URLs translated with no network), set prices, render forum shop pages to paste. Never reads the trade site, never posts.
- Needs: store, search, pricing and shop tools need no daemon and no network; fetching tools (jobs, apply) need the daemon the person started with `acq auth` at a terminal; `acq-mcp` "never starts one" against GGG. Without a keyring "a login lasts as long as the daemon that holds it".
- Before an agent can use it: unpack, put the MCP config (`"command": "/path/to/acquisition/acq-mcp"`) in `.mcp.json`; log in with `acq auth`; for the mock, `"env": {"ACQ_PROVIDER":"mock"}` after `ACQ_PROVIDER=mock acq auth`, and `ACQ_STORE_DIR` "must match your acq's".
- New league: read `acq leagues --stored`; write the first policy with `if_revision: 0` (lands only where none is stored); naming only the new league in a realm drops Standard; then plan and apply `--league <name>` until the plan says nothing to do; first plan is the listings, later plans every tab — read each plan's work and ceiling first.
- I would not, without asking: write over an existing sync policy (`replace`, or any `if_revision` other than 0 the first time), `apply_plan` a plan with a large request count (it spends the person's real rate limit against GGG), `cancel_job` on jobs I did not submit, and anything that needs the person to re-login. Here the brief makes all writes safe; I still treat the policy overwrite as needing a stop.
- Pointers I cannot follow here: `AGENTS.md`, `CLI-REFERENCE.md`, every `crates/…` link in Knobs, `decisions/<area>.md`, `SECURITY.md`, `CONTRIBUTING.md`.

**What `open` (call 1) told me that the README did not.**
- The server is in `provider: mock` (instructions' first line).
- It lists **26** tools; the README says "its 25 tools" and its four groups name 25 — `leagues` (stored league list, `stored` must be `true`) is missing from the README's list.
- How to start: "Begin with search and no arguments"; `describe` with no names is a ~38 KB index; refusals carry `error`, `kind`, `readings`, `next`, `offers`, sent back "as printed".
- `price_clear` requires `if_revision` (refused without, naming the current one); `apply_plan` refuses if the policy revision moved; `max_requests` bounds an apply at admission.
- `submit_job` kinds: profile, characters, character, leagues, stashes, stash, apply, sleep, fetch, whoami.
- Ages are given "in words beside its time" everywhere (`describe ["ages"]`).
- `list_jobs`/`job_status`/`job_result`/`daemon_status` answer from the ledger on disk with no daemon.

**What the README said that `open` did not.** The daemon's idle exit (an hour by default; `ACQ_IDLE_SHUTDOWN`) and what the person types when it is gone (`acq auth check` / `acq auth`); rate limit 30 tabs per 5 minutes and "a first sync of a few hundred tabs takes most of an hour"; Standard is the default league; the store's paths; the `--if-revision 0` rule for a first policy is stated in the README as a recipe, while `set_sync_policy` only says "omitted, or 0, only to create the first" — it does not say an omitted `if_revision` with a policy already stored is refused (it says "required when a policy exists", which I read as: refused).

**Whose items are here (call 2).** Two accounts, provider `mock`: `GERWARIC#7694`, last login 19d ago, `persisted: false`, store `W/store/mock/GERWARIC_7694.db`; and `seat`, last login 35s ago, `persisted: false`, store `W/store/mock/seat.db`. Neither login is in a keyring. The tool does not say where GERWARIC's items originally came from beyond `provider: mock` (the brief says it is a copy of the person's real collection — that is the brief's, not the tool's).

# 2. The tasks

## T1 — the host

**To the person.** "I wrote `W/work/acq.json` in the README's shape: server `acquisition`, `command` = `W/bin/acq-mcp`, and `env` `ACQ_PROVIDER=mock` and `ACQ_STORE_DIR=W`. Opened through it, the server answers the same instructions and 26 tools as before (call 3, 37872 bytes, identical size to call 1), and its `daemon_status` says it found your running daemon and that provider, contract, artifact and world all match (call 4: `world` `W/store`, `world_matches: true`)."

- Calls: 3, 4. Right: yes; the first call I hold right is 3, confirmed by 4. What would say it is not: `world_matches: false` or `provider: ggg` in call 4.
- What the README told me: the JSON shape; that knobs go through `env`; `ACQ_PROVIDER=mock` for the stand-in; that `ACQ_STORE_DIR` "must match your acq's".
- What it did not: what value `ACQ_STORE_DIR` takes — the data dir (which holds `store/mock/…`) or the `store` dir itself. I reasoned from "Your data stays … under `~/Library/Application Support/com.smallcatlabs.acq` … one database per account in `store/ggg`" and the Knobs row (default "the platform data dir") that it is the parent of `store/`, i.e. `W`. Call 4's `world` printed `W/store`, so the daemon calls the `store` dir the world while the knob takes its parent — a naming a reader can trip on. The README also does not say whether the mock needs `ACQ_NO_KEYRING` in the env (the daemon said `keyring: unavailable: disabled by ACQ_NO_KEYRING`, call 4) — it did not matter for an MCP server that never logs in. The README's server name `acquisition` I kept.

## T2 — keep the new league fresh

**To the person.**
"Before I went to the API the tool knew **no leagues** for your `seat` account: the stored list was never fetched (`fetched_at: null`, `"never"`), and you had **no sync policy** (`policy: "none"`, call 5; `policy: null`, call 6). So I fetched the list once (a `leagues` job, calls 7–8). It holds **Standard, Hardcore and Rehearsal** on pc, fetched '4s ago' (call 9). The tool does **not** mark which is new: the stored list carries no start date and no 'new' flag (call 9); I read it from the job's payload, where Rehearsal's description is 'The mock's challenge league, empty on its first morning' (call 10). So the new league is **Rehearsal**.

No policy was there, so I wrote the first one with `if_revision: 0`, which can only create (call 11): pc → Rehearsal, `tabs: all`, `characters: all`, `max_age_seconds: 3600`. It stored revision 1, `written_via: mcp`, `prior: null`, and says there is no undo: 'a first policy has nothing before it to put back, and no verb clears a policy' (call 11). It names only Rehearsal; your Standard is not covered (you had no policy, so nothing was dropped).

First plan (call 12): two listings — the Rehearsal stash list and the character list, both `never_listed`; 2 logical requests, 2–6 wire sends (HEAD probes, a token refresh, up to 2 re-sends per 429 outside every estimate); no ETA — the limiter had not learned these routes yet (`eta_seconds: null`, 'policy not yet learned'); leaves out 0 tabs (`uncovered_tabs: 0 of 0`); rests on nothing — stash and character listings 'never'. I applied it bounded at `max_requests: 2` (call 13 → job 3), it was done at once (call 14), 2 children done, 0 failed (call 15). It landed: 8 Rehearsal tabs listed '7s ago', none fetched, one of them a folder (call 16).

Second plan (call 17): 7 tab fetches, all `never_fetched`, 7 logical requests, 7–21 wire sends, no ETA again; it leaves out 1 tab, `Old leagues`, because it is a folder (`skipped_tabs`), and 0 tabs fall outside the policy (`0 of 7`); it rests on the stash and character listings '9s before generated_at'; it notes `no_characters_listed: true` — you have no character in Rehearsal. Applied with `max_requests: 7` (call 18 → job 8, 1 tab skipped), done (call 19), 7 children done, 0 failed (call 20). It landed: all 7 tabs fetched '7s ago', each with 0 items — the league is empty (call 21).

Third plan (call 22): **nothing left** — 0 actions, 0 requests; every tab skipped as `fresh` (age 9 s), the folder as `folder`. The tool will want them again once they pass 3600 s; it does not run on its own — someone has to plan and apply again."

- Calls: 5–23. Right: yes. First call held right: 5. What would say it is not: a tab in `tabs` with `fetched_at` older than the apply, or a child in `failed_ids`.
- Job records afterwards: `job_status` shows the state, the params it was sent and `submitted_by` but **no time** — no submitted, started or finished time (calls 8, 14, 19). `job_result` of the parent gives child ids and done/failed counts (calls 15, 20); a child's gives the raw stash body and GGG's rate-limit headers, again with no time (call 23). To learn *when* anything landed I had to go to `tabs` (call 21).
- "How long" the plan would take: the tool could not say — every scope had `eta_seconds: null` before the routes were learned (calls 12, 17). The README's "thirty every five minutes" is the only clock I was given; the mock answered in seconds.
- Where the tool stopped/misled me: the stored league list cannot tell me which league is new (call 9); the README says `acq leagues` "names a new league the day it opens" — the MCP `leagues` tool reads only the stored list and names nothing new. The policy write's `no_undo` says no verb clears a policy — a person should know that before writing one.

## T3 — what I have

**To the person.**
"**The whole store** (`GERWARIC#7694`): **22,721 live items** (6 removed), in 3,444 tabs and 65 characters, 54,382,592 bytes on disk (call 24). Across both realms, pc and poe2, the items sit in 463 places in 8 leagues: Allflame, Ancestors, Hardcore, Phrecia 2.0, Runes of Aldur, Solo Self-Found, Standard, Ziz Rapture HCSSF Class Gauntlet (call 26). Of the places, 551 were fetched and 2,941 never were — 2,489 map tabs, 437 unique tabs and 15 characters, all in Standard (call 26). The oldest thing I know was fetched **62 days ago**, from the character `gerwarshot` in Standard; the newest **53 days ago**; the tab list was last seen 62 days ago (call 26). Nothing is from this week: everything I tell you about these items is at least 53 days old. Your league list was never fetched on this account (call 24). The store also says it **refused 4 bodies** it would not take (call 24) — I cannot show you which: the tool says `acq store refused` 'has no tool here: a person runs it' (call 29).

**Standard's tabs**: 3,347 tabs, 405 fetched, 2,926 never fetched (2,489 map, 437 unique), 16 folders, 20,231 items (call 27). The ten largest tabs were fetched 56 days ago and listed 53 days ago (call 27). The tool gives no oldest/newest for the league's tabs as a whole here — only per row, ten of 3,347 rows. Other pc leagues it holds: Allflame (97 tabs, 4 characters), Ancestors, Hardcore, Phrecia 2.0, Solo Self-Found, Ziz Rapture (characters only) (call 27).

**Prices in Standard**: 21,311 items in 3,388 containers; **no hand-set prices** (`rows.total: 0`, `manual: 0`); 492 items priced by the game's notes or tab names (487 exact, 2 negotiable, 3 skip); 1 invalid note; 1,395 'residue' (price text the trade site cannot see); 17 priced tabs, 3 of them public (call 28). Its facts: 3,372 places, 446 fetched, oldest fetch 56 days ago (tab `Keepers (Remove-only)`), newest 53 days ago (`Dump Tab`), stash and character listings 53 days ago (call 28). Same seven other pc leagues listed (call 28)."

- Calls: 24–29 (25 refused: two realms need `scope.realm`; I sent its `realm all` offer as printed, call 26). Right: yes as the tool's account of itself. First call held right: 24.
- What would say it is not: the store-wide answer (call 26) gives two different places for the same newest time 1789147353: `scope.newest_fetch_location` = stash `Dump Tab`, `total.newest_fetch_location` = character `gerwarwitch`. Both cannot be "the" newest; it is a tie broken two ways — see Faults.
- Ages: all three answers give an age in words beside each time (calls 26, 27, 28). The store-wide answer's oldest (62d, a character) is older than the Standard price facts' oldest (56d, a tab) — they count different place sets (463 vs 3,372 places; the first counts only places holding live items, as far as I can tell — the tool does not say).
- Refused/unreadable: 4 refused bodies counted (call 24), 0 shown; `unlifted_items` 0, `withheld` 0 (call 24); price rows `unreadable` 0 (call 28).
- Where it stopped me: no MCP way to list the refused bodies (call 29). `store_status` (call 24) carries no age at all — not the newest fetch, nor the snapshot; the instructions say "Begin with search", and only there did I get ages.
- Brief said the person has "prices they have set"; the tool shows 0 hand rows in Standard (call 28) — the prices are notes in game.

## T4 — find things

I learned the language from `describe` patterns (`~link`, call 30; `~resist`, call 32). Every search names `realm: all`, because the store holds pc and poe2 and a search without a realm is refused (call 25).

**(a) "Do I have any gear with six linked sockets?"** — "Yes: **187 items** with a six-link, in 117 places (call 31). The first ten are all on characters — e.g. *Skin of the Loyal* Simple Robe on TheFireworksCompany, *Tabula Rasa* on BatteryStaple, *Damnation Shroud* Astral Plate on mOtOrIZedBiCYclE (call 31). These matches were fetched between **57 days ago** (Allflame character `I_EXIST`, the oldest) and **53 days ago** (`gerwarwitch`, the newest) (call 31, `total`), while the store as a whole reaches back 62 days (call 31, `scope`)." Query `links=6`; 0 undecided. 177 rows left out, pageable through `view.rows.next`.

**(b) "Which of my rare rings have life and at least two resistances?"** — "**224 rare rings** with some life and two or more resistances, in 106 places (call 33); e.g. *Kraken Circle* Bone Ring (+111 life, 12% to all elemental) and *Damnation Circle* Ruby Ring (+78 life, fire/cold/chaos) on characters. Matches are 57 to 53 days old (call 33, `total`). **16 more items could not be decided** — PoE2 items, for which the tool has no class or totals table ('a refresh will not help; a reference update may'), and it gave me the request that lists them (call 33, `undecided`)." Query `class:ring rarity=rare pseudo.total_life>0 pseudo.count_res>=2`. Caveat I would tell the person: `pseudo.count_res` is the trade site's count and includes chaos (call 32; the matches show chaos counted, e.g. Woe Grasp: fire + chaos, call 33); `pseudo.total_life` is a name I guessed by analogy — `~resist` does not list it, and the query parsed and matched (call 33), so the guess held.

**(c) "How many items per league and per rarity?"** — the first cross (call 34) printed 10 of 15 cells, `left_out: 5`, `left_out_needs: view.cross.limit`; with a limit of 50 (call 35):

| realm / league | no rarity | normal | magic | rare | unique | sum |
|---|---|---|---|---|---|---|
| pc Standard | 7,693 | 2,850 | 2,334 | 5,615 | 2,819 | 21,311 |
| pc Allflame | 561 | 276 | 64 | 154 | 257 | 1,312 |
| poe2 Standard | 43 | 13 | 25 | 16 | 1 | 98 |

Sum 22,721 = the store's live items (calls 24, 35). The other six leagues the scope names (Ancestors, Hardcore, Phrecia 2.0, Runes of Aldur, Solo Self-Found, Ziz Rapture…) hold **no live items** — they appear in `scope.leagues` (call 35) and `leagues_held` lists them with characters (call 27) but no cell counts them. "No rarity" (8,297) is currency, gems, cards and the like (margin, call 35). Ages: the counts carry only the whole scope's ages (62d oldest, 53d newest, call 35) — a cross gives no age per cell.
Agreement: pc Standard 21,311 = `price_status` Standard `counts.items` 21,311 (call 28). `tabs` Standard says `items: 20,231` (call 27) — 1,080 fewer, because `tabs` counts tab items only and the rest sit on Standard's 41 characters; the `tabs` answer does not say that it leaves characters out. I did not read Allflame elsewhere to cross-check its 1,312.

**(d) One item, everything** — *Kraken Circle*, Bone Ring, rare, ilvl 78, requires level 64, identified; equipped on character `undxot` (pc, Standard); first seen 62 days ago, last seen 56 days ago, that character fetched 56 days ago; six lines with template and numbers (+110 accuracy, +111 life, +12% all elemental res, minion attack/cast speed, crafted −6 mana cost, implicit minion res); class Rings, category `accessory.ring`; price `none` — 'nothing applies'; snapshot 52 days old (call 36).

- Calls: 30–36. Right: yes. First call held right: 31. What would say it is not: a `show … against` on a ring outside the 224 explaining a miss I did not expect; I did not run one.
- Where the tool stopped me: none outright; (c)'s first answer was cut at ten cells (call 34). `scope.newest_fetch_location` and `total.newest_fetch_location` name two different places for one time (calls 26, 31, 33, 34, 35).

## T5 — prices

**To the person.**
"**What you have priced** (Standard, the only league I asked): **nothing by hand** — no hand rows at all (`rows.total: 0`) — and **492 items priced by the game**, from notes or public tab names: 487 exact, 2 negotiable, 3 skip; 17 tabs carry a price, 3 of them public (call 28).

**What is wrong:**
- **1 note the tool cannot use**: *Thaumaturge's Incubator* (stack of 3) in tab `ACQUISITION-PRICE-TEST` at x 9, y 5 — note `~price  chaos`, 'the amount is empty — what the dialog leaves after an invalid entry' (call 38). Fix it in game.
- **1,395 items whose price text the trade site cannot see** ('residue'): a note on a character's item, or a note or tab name in a tab that is not public (call 37). E.g. the tab `~price 4444 chaos (Remove-only)` is private, so its 'price' prices nothing — *Fairgraves' Tricorne*, *Honourhome*, *Devoto's Devotion* and the rest of its substash (call 39). Make that tab public in game, or move the items. I listed 3 of 1,395 (call 39); `price_list` with `game_text: residue` and `expand` names them all.

**What 'an item the game skips' means:** 3 items' game side says `skip` (call 28) — the game's `~skip` note or tab name (the `type` field of `price_set`, call 1). The tool's only further word is that 'a hand row nearer the item still applies over a skip' (call 37). It does not say in plain words that a skip keeps the item off the shop, and I did not list the three — `price_list` has no filter for the game's statement, only for `relation`, `effective` and `game_text` (call 1).

**My writes:**
1. Tab `ACQUISITION-PRICE-TEST` (`tab/pc/03cb479c65`, public, 81 items, 50 of them priced by their own notes, call 40) at **5 chaos**: created, revision 1, `prior: null`, `in_facts: true`; undo = `price_clear` naming revision 1 (call 42). It now decides 30 items (the 50 with notes keep the note's price — the note beats a tab row, call 37).
2. Item *Thaumaturge's Incubator* (`item/4aefe3b5…f9f`, the invalid-note one) at **2 divine**: created, revision 1, `in_facts: true`; undo = `price_clear` revision 1 (call 43).
3. I read it back: `manual.revision: 1`, `from` the item itself (call 44), then **replaced it with 3 divine naming `if_revision: 1`**: it landed, revision 2, `prior` the 2-divine row; undo = `price_set` 2 divine naming revision 2, 'puts the prior back' (call 45).

**Read back:** the item: 3 divine by hand, revision 2, effective (call 47); the tab: 5 chaos, revision 1 (call 47's `container`); hand-priced items in Standard now 31 (`effective: manual`, call 46 — the 30 tab items and the one item).

**If another program changed a price between my read and my write**, the write naming the revision I read would be refused as a `conflict` naming the row's current revision and the row as it stands (call 37's revision rule; I show this refusal in T7). Without that I would not know: a create omitting `if_revision` lands where no row exists."

- Calls: 37–47. Right: yes. First call held right: 38 (the invalid note); writes from 42.
- What would say it is not: a `price_show` of the tab or item not showing my row; none.
- `in_facts` on every receipt was `true` (calls 42, 43, 45) — it says the address exists in the facts in some league, not that it is fresh (call 37).
- Where the tool stopped/misled me: "skip" is never defined in words a person can act on (calls 1, 37); `price_list` has no `game` statement filter to name the three skip items. The README's line 'a note in a private tab is no price' was the clearest sentence on residue I read.

## T6 — the shop

**To the person (no policy, call 48).**
"Your Standard shop is **1 page**, 2,327 characters of a 50,000 limit, 31 items. Paste the page's `text` into a post in Standard's shop subforum on pathofexile.com. On it: one spoiler `~price 5 chaos` holding 30 links into `Stash57` — that is tab `ACQUISITION-PRICE-TEST` (index 56, so its rank 57 among the tabs the website lists) — and after it one spoiler `~price 3 divine` holding the single link `Stash57 x=9 y=5`, the Thaumaturge's Incubator (call 48). Left out: **492** items — 489 the game already lists at its own price ('a page must not contradict what the site shows') and 3 the game marks `~skip`; none blocked; 20,788 'off page', nothing prices them (call 48). The 50 other items in that tab with their own notes are among the 489 — your notes beat the tab price.
Freshness: with no policy, `freshness.policy: not_set`, nothing called stale or uncovered; the posted items' positions were fetched **53 days ago** and the stash listing their `Stash57` rank comes from was read **53 days ago** (`stash_listing_fetched_at_age`, `freshness.facts`, call 48); `position_before_listing: 0` — the positions are not older than the listing. If you have moved or reordered tabs in those 53 days the links can point at the wrong place; the tool tells me nothing about that beyond the dates."

**With a policy (calls 49, 51).** I wrote GERWARIC's first policy, if_revision 0, pc → Standard, tabs all, characters all, 3600 s (call 49: revision 1; `league_notes`: the league list was never fetched on this account; `no_undo`, 'no verb clears a policy'). The pages were identical (call 51; call 50 is the page alone, which carries no freshness at all — I had to render whole again). What changed is `freshness`: `policy: covers`, revision 1, `window_seconds: 3600`, **`stale_total: 31` — every posted item is stale**, `oldest_stale_seconds: 4651202` with the words '53d ago', `uncovered_total: 0`, and `refresh_requests: 3374` — what bringing the league fresh would cost (call 51). "Reported never enforced" (call 1): it renders anyway.

**Clean-up.**
- Item: the only removal the receipts offered was call 43's `price_clear` naming revision 1; sent exactly as printed it was **refused as `conflict`**: 'has a price at revision 2, not the revision named', with the row as it stands and one offer, the same clear at revision 2 (call 52). Refusal warranted — my replacement (call 45) moved the row on, and call 45's own undo puts 2 divine back rather than removing. I sent the offer as printed (call 53): removed, revision 3, undo a create of 3 divine.
- Tab: call 42's undo, `price_clear` revision 1, sent exactly as printed: removed, revision 2 (call 54).
- Read back: the item 'no row applies and the game states nothing', its tab the same (call 55) — as found in call 40.
- **Not put back: the sync policy I wrote on `GERWARIC#7694` (call 49) stays** — no verb clears a policy (call 49). It names Standard only.

- Calls: 48–55. Right: yes. First held right: 48.
- What would say it is not: a link whose `Stash<n>` disagrees with the tab's place in the game's list; I cannot check that from here.
- Where it stopped/misled me: a removal undo goes stale after a replace and only the refusal offers the current one (calls 45, 52); `oldest_stale_seconds` is a duration but is given an age 'ago' as if it were a time (call 51); `shop_render` with `page` drops `freshness` and `left_out` (call 50).
- What a person does next: copy the `text` of each page into a forum post in the league's shop subforum, and leave the items where they are (the README: 'a moved item may lose its link').

## T7 — for a program

**Times read again, one answer per task, from the printed document alone:**

| Answer | Time it carries | Age without arithmetic? | Age of what? |
|---|---|---|---|
| T2 plan (call 17) | `generated_at` | yes, `generated_at_age` '0s ago' | the plan |
| | `basis.stash_listing.fetched_at`, `character_listing.fetched_at` | **only relative**: '9s before generated_at' — to get 'ago' a program adds `generated_at_age`; the words sit outside the `plan` envelope, the envelope itself holds raw times only | the listings the plan rests on, not the tabs |
| | `quote.observed_at` | yes, '0s ago' | the daemon's quote |
| T2 job records (calls 8, 14, 19, 20, 23) | **none** — no submitted, started or finished time | no | — |
| T3 store (call 26) | `scope.oldest_fetch`/`newest_fetch`/`list_seen`, `total.…` the same, each row's `place.fetched_at`, `basis.snapshot.at` | yes, each has `_age` ('62d ago', '53d ago'; `snapshot_at_age` '52d ago' at top level, away from `basis`) | `scope` = everything in scope; `total` = the matches; a row's = its place. For `{}` the two coincide — except the newest *location*, which differs (Faults) |
| T4 six-links (call 31) | same shape | yes | `total` is the matches' own (57d–53d), `scope` the store's (62d–53d) — clearly kept apart |
| T5 receipt (call 45) | `written.created_at`/`updated_at`, `prior.…` | yes, `_age` each | the row written |
| T6 shop (call 51) | `rendered_at`, `listing.stash_listing.fetched_at`, `freshness.facts.*`, `oldest_stale_seconds` | yes for the times; `oldest_stale_seconds` (4651202) is a *duration* labelled `_age` '53d ago' — read as a time it is nonsense | `freshness.facts` = the posted items' locations only (1 place); `stash_listing` = the whole league's listing. No per-posted-item time |

**Three failures on purpose:**
1. **Wrong account** (`GERWARIK#7694` to `tabs`, call 56): refused, `kind: not_logged_in`, one JSON document with `error`, `accounts` (the names known) and two `offers`, the same request as each real account. Warranted: yes. Recoverable: yes. But the kind is misleading — 'no login made a store for …' — the account does not exist; it is not a login problem, and GERWARIC itself has no login either and reads fine.
2. **A job fetching a stash tab that does not exist** (`stash`, `rh-nope`, call 57): **accepted** (job 17) — the tool did not check the id against the listing it holds (`tabs`, call 21). Its record: `state: failed` (call 59), `outcome: failure`, `error: "GET /stash/Rehearsal/rh-nope returned 404 Not Found; no rate headers; a body of 25 bytes"` (call 60). One document a program can read, yes; what to do next: **nothing** — no `offers`, no hint that `rh-nope` is not among the 8 tabs on record, no time it failed.
3. **Price write naming a wrong revision** (`if_revision: 7` on the tab whose row I cleared, call 58): refused, `kind: conflict`, 'price was cleared (revision 2); a new price names revision 0, or none', `revision: 2`, one offer: the same price as a create with `if_revision: 0`. Warranted: yes. Recoverable: yes. (Also call 52: a stale undo refused the same way, with `current` — the row as it stands — and its offer.) I did not send call 58's offer — it would re-price the tab.

- Calls: 56–60 (and 17, 21, 26, 31, 45, 51 re-read). Right: yes. First held right: 56.
- What would say it is not: the README says a failed job is 'its outcome record, `outcome: "failure"` with `error`, not a refusal' — call 60 matches it exactly.

## T8 — the daemon

**To the person.**
"I have been talking to **acqd 1.0.0-alpha.1, pid 39659**, the one beside the server in `W/bin` (same file, contract `a0870ab4ec7f`), serving the store at `W/store` in **mock** mode, compatible with this server on all four counts (call 61). It had been up **384 seconds** when I asked (call 61; 65 s at call 4), has **1 connection**, nothing running or waiting, max 2 in flight; it has made **18 real sends** since it started (2 at call 4), knows 5 rate-limit policies, no rail halted, tripwire off, no send cap; its send journal and log are under `W/logs/032cd28786ee/mock/`; the keyring is 'unavailable: disabled by ACQ_NO_KEYRING' (call 61).

**When it goes away:** it says `idle_shutdown_seconds: 14400` — four hours — (call 61), counted, by the README, from its last connection or job submitted once nothing is sending or waiting and the limiter's recent history is gone. It does **not** say when its last connection was, so I cannot give you a clock time — only 'four hours after the last time anything used it'. The README's default is an hour; this one was started with four.

**Which tools need it:** `submit_job`, `apply_plan`, `cancel_job` talk to it; `refresh_plan` adds its quote if one runs; `list_jobs`, `job_status`, `job_result`, `daemon_status` answer from the ledger without one; everything else reads the store (call 1, instructions). In mock mode this server will itself start a daemon 'into an empty socket'; against GGG it never does (call 62).

**When it is gone, type:** because there is no keyring here, your `seat` login lives only in that daemon and goes with it (the README: 'Without a keyring, a login lasts as long as the daemon that holds it'; `persisted: false`, call 2). So `acq auth check` will not bring it back; at your desktop terminal, in this world's settings, run
`ACQ_PROVIDER=mock ACQ_STORE_DIR=W ACQ_NO_KEYRING=1 acq auth`
and log in as `seat` again, then `acq daemon status` to see it and its idle exit. (`ACQ_NO_KEYRING=1` I take from the daemon's own keyring line; the README's 'Your agent' section names only `ACQ_PROVIDER` and `ACQ_STORE_DIR`.) After that I can carry on."

- Calls: 61, 62 (and 4, 1, 2). Right: yes, for what it says of itself. First held right: 61 (4 earlier).
- What would say it is not: `running: false` or `world_matches: false`.
- Where it stopped me: no start time, no last-connection time, no predicted exit time — only `uptime_seconds` (no words) and `idle_shutdown_seconds` (call 61). `policies_known: 5` is explained only as 'rate-limit policies learned' (call 62). I could not test what the tool says once the daemon is gone without stopping it, which no tool here does.

# 3. Faults

Worst first.

1. **Job records carry no time.** `job_status` and `job_result` give state, params, payload or error, but no submitted, started or finished time (calls 8, 14, 19, 20, 23, 59, 60). Should: each job's times with their ages, as every store answer has. A program cannot say when a fetch landed or failed from the job; it must go to `tabs` (call 21). Person and agent both (the CLI's `acq jobs` likely shares the ledger; I could not check).
2. **A job for a tab the store knows does not exist is accepted and sent.** `submit_job` `stash` `rh-nope` was admitted and spent a real request on a 404 (calls 57, 59, 60), though the listing on record names 8 tabs and not that one (call 21). Its failure record says only the HTTP 404 — no offer, no 'this id is not in the listing fetched N ago'. Should: at least note the id is unknown to the listing, as `refresh_plan` does with `unknown_tabs` (call 1). The failure itself was warranted; it did not say enough to recover. Agent mostly (a person uses `refresh --tabs`).
3. **Nothing names a new league.** The stored league list (calls 5, 9) holds ids, realms, `named` and `held` — no start date, no 'new' mark. The README says `acq leagues` 'names a new league the day it opens'; the MCP's `leagues` (`stored: true` required) does not, and the `leagues` job's payload has only GGG's description (call 10). I found Rehearsal from a mock description. Person and agent.
4. **Two different 'newest' places for one time.** In every store-wide search, `scope.newest_fetch_location` is stash `Dump Tab` and `total.newest_fetch_location` is character `gerwarwitch`, both at 1789147353 (calls 26, 31, 33, 34, 35). A tie broken two ways; a reader takes them as two facts. Should: one rule, or say it is a tie. Both.
5. **`oldest_stale_seconds` is a duration printed as a time's age** — `4651202` beside `oldest_stale_seconds_age: "53d ago"` (call 51); the `_age` rule (call 29) is for times. Both, a program more.
6. **A removal undo goes stale after a replace.** The create's receipt offers `price_clear` at revision 1 (call 43); after the replace (call 45) that undo is refused as `conflict` (call 52) and the replace's own undo restores 2 divine, not 'no price'. Refusal warranted (my request named a stale revision); it said enough — `current` and one offer at revision 2 — to recover (call 53). The fault is only that no receipt the person holds says how to get back to *before the first write*. Both.
7. **Wrong account refused as `not_logged_in`** (call 56). Warranted, recoverable (it lists the names and offers both), but the kind is wrong: GERWARIC has no login and reads fine; the name simply does not exist. Should be e.g. `unknown_account`. Both.
8. **README says 25 tools; the server has 26** — `leagues` is missing from its list (call 1). Agent mostly.
9. **`shop_render` with `page` drops `freshness`, `left_out` and `policy`** (call 50 vs 51) — the record says 'that page's record in place of the whole render' (call 1), but a person pasting page by page loses the warning that every item is stale. Both.
10. **A sync policy can never be removed** — `no_undo`: 'no verb clears a policy' (calls 11, 49). I left one on GERWARIC#7694 that I cannot take back. The README does not warn before the first write. Both.
11. **No MCP way to list refused bodies** — `store_status` counts 4 (call 24), `describe` says 'acq store refused has no tool here: a person runs it' (call 29). Agent only.
12. **The tool cannot say how long a first plan takes**: `eta_seconds: null` on every scope until a route is learned (calls 12, 17). Honest, but a person asking 'how long' gets nothing but the README's 'thirty every five minutes'. Both.

# 4. What the README and the tool texts did not say

- **What value `ACQ_STORE_DIR` takes** — the data dir, parent of `store/` — expected in 'Your agent' or the Knobs row; learned by inference from the paths section and confirmed by `world_matches: true` in call 4. And that the daemon then calls `W/store` its `world` (call 4).
- **Which league is new** — expected in the `leagues` tool text or the README's 'A new league'; learned from a mock league description in a job payload (call 10).
- **That `pseudo.total_life` exists** — expected in `describe`'s `~resist`-style index; guessed, and it parsed (call 33).
- **That `pseudo.count_res` counts chaos** — implied by 'the trade site's pseudo_count_resistances' (call 32), seen in matches (call 33).
- **That `tabs.items` counts tab items only, not characters'** — expected in the `tabs` text; learned by comparing calls 27 and 28/35.
- **What 'skip' means for the shop** — expected in `describe ["pricing"]`; learned from the shop's policy table, `game_skips`: 'the game marks it do-not-index (~skip)' (call 48).
- **That the mock server spawns a daemon itself** — expected in the instructions, which speak only of real mode; learned from `describe ["daemon"]` (call 62).
- **That the daemon's exit cannot be timed** — no last-connection time anywhere (calls 4, 61).
- **That an apply finishes in seconds against the mock** — I polled once and it was done (calls 14, 19); nothing said to expect that.
- **The README said and I never needed:** install and checksums, Windows shells and `ACQ_SHELL`, the macOS quarantine note, `acq web`, `acq store import`, `acq diagnose`, the rails knobs, `ACQ_MOCK_*`, licence and audit, every link under `crates/…`, `AGENTS.md`, `CLI-REFERENCE.md`, `decisions/`, `SECURITY.md`, `CONTRIBUTING.md` (pointers I could not follow and did not need).

# 5. The tool's own words

- **Right and useful:** the instructions' rule 'any request an offer or an answer holds is sent back as printed' held every time (calls 26, 53); refusals are one JSON document with `kind` and `offers` (calls 25, 52, 56, 58); every store time has an `_age` in words; `scope` vs `total` in search keeps the store's age apart from the matches' (call 31); the price receipts (`written`, `prior`, `revision`, `undo`, `in_facts`) are exact and the undo's `says` is plain (calls 42–45, 53, 54); the shop's `policy` table explains every omission in one line (call 48); `refresh_plan`'s `not_covered` is candid about what the estimate leaves out (call 12).
- **Wrong:** `not_logged_in` for a name that does not exist (call 56); `oldest_stale_seconds_age` '53d ago' (call 51); the README's '25 tools' (call 1); README 'an hour after its last connection' vs this daemon's 14400 (call 61) — the README covers it with 'or as long as the idle knob sets'.
- **Missing:** times on job records; a 'new' mark or start date on leagues; a time for the daemon's start and last connection; a summary oldest/newest in `tabs`; a filter for the game's statement (skip) in `price_list`.
- **Misleading:** the instructions' 'in real mode a human starts it and this server never spawns one' says nothing of mock mode, where it does (call 62); `refresh_plan`'s listing ages are 'N before generated_at', relative, and sit outside the `plan` envelope (call 17).
- **Read and never used:** `item_events`, `characters`, `cancel_job`, `list_jobs`, `quote.scopes` rules, `basis` fields (`derivation`, `classes`, `beast_colours` …), `reference_currency`'s `evidence`, `view.explain`, `trade`.
- **Words I did not understand:** 'unlifted_items' (call 24, explained only in call 29); 'residue' until `describe ["pricing"]` (call 37); 'rails' and 'tripwire' (README Knobs, call 61) — I understood them only as test-run brakes; 'the limiter's recent history is gone' (README) — when that is, nothing says; `snapshot_response`, `facts_version`, `derivation` in `basis`.

# 6. What you would ask of the tool

- Put `submitted_at`, `started_at`, `finished_at` with ages on every job record (calls 8, 59, 60).
- Check a `stash` job's id against the stored listing and refuse or note an unknown one (call 57).
- Mark a league as new, or carry its start date, in `leagues` (call 9).
- Make `scope.newest_fetch_location` and `total.newest_fetch_location` agree on a tie (call 26).
- Rename `oldest_stale_seconds_age` or give it as a duration in words (call 51).
- Give a receipt's `undo` a way back to before the first write, or say on a replace that the create's undo is now stale (calls 45, 52).
- Use a `kind` like `unknown_account` for a name never seen (call 56).
- Give `daemon_status` a start time, last-connection time and predicted exit time (call 61).
- Add an oldest/newest summary to `tabs` (call 27).
- Keep `freshness` on a `shop_render` of one page (call 50).
- Expose `store refused` as a tool (call 29).
- Let `price_list` filter by the game's statement (skip) (call 38).
- A verb to clear a sync policy, or a warning before the first write that none exists (call 11).

# 7. Left out

- **Not tried:** `cancel_job`; `list_jobs`; `item_events`; `characters`; a `trade` URL search; `show` with `against` or `body`; `describe` with no names (the 38 KB index) or `all`; what the tool says once the daemon is gone (no tool stops it); a `refresh_plan` for GERWARIC's Standard (it would have priced 3,374 requests, call 51); Allflame cross-checks; the three skip items by name; pages of the 6-link and ring results past ten; T6's re-render after `apply` (GERWARIC has no login, so its policy was never applied).
- **Files opened that are not one of the two:** the harness saved the over-size output of `open` (calls 1 and 3, 37 KB each, under the door's 60,000-byte cut) to a file in my session's tool-results folder and I read call 1's copy with my file tool to see what `open` printed; nothing else.
- **Files written:** `W/work/acq.json`; `W/drafts/apply1.json`, `W/drafts/apply2.json` (the plans for `apply_plan --from`, because the harness refused the heredoc); `W/drafts/r2.md` … `r9.md` (report parts, `report --from`). One write I attempted to a path outside W by mistake (`/Users/tom/seat-worlds/antml/placeholder`) was denied by the harness and wrote nothing. Not put back: GERWARIC's sync policy (call 49) and seat's (call 11) — no verb clears one; the item and tab prices are cleared (call 55).
- **The one thing I cut that I would most want back:** checking that the shop's `Stash57` link really is the tab's place in the game's list — the one thing on the page a person cannot correct later, and the one thing I had no way to verify.
