# Debrief

## 1. Where did the tool stop you, and what did you do instead?
- **Call 25:** `search {}`, the instructions' suggested first call, was refused (`realm_needed`). Its offers held only `pc` and `poe2`, so I used `realm: "all"`, which the error text named (call 27).
- **Call 26:** `realm` is not a count key. I counted by league over `realm: all` instead (call 27).
- **`describe` size:** with no names it is over 150 KB, beyond the door's 60,000-byte cut, so I could not read the language's index. I guessed `count_res` (call 35) and `pseudo.total_life` (call 36), and they happened to exist.
- **Call 45:** `-tab.public` was refused as a bare word. Call 46 (`tab.public=false has:price.amount`) answered 0, because `price.amount` is the effective price. I gave up grouping residue by tab: I named the one tab I could count (call 44) and said the other 1,235 were not grouped.
- **Call 59:** the item's removal undo was stale. I built a clear from the refusal's `revision: 2` (call 60).
- **Call 30:** refused bodies have no MCP tool. I reported their count only.
- **Not the tool but the harness:** my first attempt with a shell variable for W, and the `apply_plan` call with a heredoc, were refused by the permission layer. I wrote the plans to `W/drafts/` and used `--from` (calls 12, 18).

## 2. Which outputs did you have to read twice, and what made them hard?
- **Call 1 (`open`, 36 KB):** too long to see whole. I read the file the harness saved.
- **Calls 27 and 32:** the `scope` block's oldest fetch changed between them (59 days vs 54 days). It took me a second read to see that `realm: all` vs `pc` changes the scope's ages, because gerwarshot is a PoE2 character.
- **Call 27, `never_fetched_by_type`:** characters are counted under a key named `tabs`.
- **Call 36:** the `matched` rows. Seeing that Viper Band matched `has:pseudo.total_life` via "+58 to Strength" took a second read.
- **Call 57 (shop policy table):** 20 cells. I read it twice to tell `omit`, `block` and `off_page` apart.
- **Call 53's receipt:** the undo has no `tool` field, unlike the first two receipts. I had to recheck the instructions to know which tool it meant.

## 3. What did you want to ask a person, and when?
- **At call 9:** whether to include Standard in the new policy. You said "keep the new league up to date", and the README warns a policy naming only the new league drops Standard. I wrote Rehearsal only and told you so in the report rather than asking.
- **At call 36:** whether "life" means a maximum Life line or the trade site's pseudo total life, which includes Strength. I gave both counts (224 and 195) and recommended 195.
- **At call 49:** which tab to price. I chose `ACQUISITION-PRICE-TEST` because its name suggested it was meant for this. Impression.
- **At call 51:** which item. I chose Goldrim in a private unique tab, not knowing until call 57 that it could never reach the shop. Asking would not have helped there; the tool told me only at render.

## 4. Which of the README's or the tool's own words misled you, and what did you take them to mean?
- **`policies_known: 1` (call 5):** I took it to mean a sync policy existed somewhere, and wrote that in T2. It is rate-limit policies learned (call 71). I corrected this in T8.
- **README "exits an hour after its last connection":** I took it as the fact. It is a default; call 5 showed 14400 s.
- **`kind: not_logged_in` (call 64):** taken literally, it would have sent me to ask for a login, when the fault was my spelling.
- **`revision: 2` in call 68:** taken alone, it reads as "send if_revision 2". The text says the opposite.
- **README "ACQ_STORE_DIR … must match your acq's":** I took it to mean the data dir (W). Call 5's `world` (`W/store`) left me unsure whether that is what it meant. It worked, so I did not test the other reading.
- **Brief vs tool:** "price one stash tab at 5 chaos per item". I took the tab row to price every item in the tab. It priced 31 of 81 (calls 54, 56).

## 5. Which answer would you retract, or not bet on, and why?
- **T8's command:** `ACQ_PROVIDER=mock ACQ_STORE_DIR=<W> ACQ_NO_KEYRING=1 acq auth`. I assembled it from the knob table and call 70; it was never tested and no text gives it. I would not bet on it.
- **T7(2), "it did cost one real request":** the send count went 2 → 19 over the session (calls 5, 70). I never isolated job 17's share. Impression, backed by the 404 in call 67 meaning a GET was sent.
- **T1, `ACQ_STORE_DIR=W`:** it worked (calls 4, 5), but I cannot say it is what the README intends.
- **T5, "1,395 residue items are price text in private tabs":** this is the tool's definition ("a note or tab name that reads as a price where the trade site cannot see it", call 1, `price_list`'s `game_text`). I only saw 5 of them (call 43), so I would not say they are *all* in private tabs. The definition does not say "private".
- **T6, "paste it in your league's shop thread":** the README says "your league's shop subforum". My word was looser than the README's.
- **T2, my remark (2) on `policies_known`:** retracted (see T8).

## 6. Which claims in your report rest on your own knowledge of the game rather than on what the tool printed?
- That `pseudo.total_life` includes Strength in general. The tool printed one row (Viper Band, call 36) matched on Strength; the general rule is mine, and the trade site's convention.
- That the items with no rarity are "currency, cards, fragments and the like" (T4c). The tool printed only `-has:rarity` counts (call 38).
- "Make that tab public" as the fix for residue. The tool says the text "is not where the index can see it" (call 43); turning the tab public is my inference.
- "Six linked sockets" = `links>=6`. The field name is the tool's; that it means the largest link group is my reading, supported by the `sockets` text shown in call 32.
- That `Stash57` is how the forum names the tab. That part is the tool's own text (call 57's policy cell), not mine.
- That Rehearsal is "the new one". I inferred it from the description "The mock's challenge league, empty on its first morning" (call 8) and its category; the tool never labels a league as new.

## 7. What would you change first, and why that first?
Give a failed job's record a `kind` and a `next`, and check a stash id against the stored listing at `submit_job` (calls 65–67). Everything else I hit had at least a usable refusal or offer. This is the one place where an agent spends rate-limited requests against the real API and gets back only English it has to parse. During a league launch, a stale id list would make that happen in volume.

## 8. What did you never find out?
- What the tool says when the daemon is gone: `job_status`'s `next`, and whether `submit_job`/`apply_plan` refuse with a command for the person.
- How long a real plan takes. Every ETA was null (calls 11, 17).
- What the 4 refused bodies are (call 24, 30).
- Where the other 1,235 residue items sit.
- Whether `ACQ_STORE_DIR=W/store` would also have worked.
- Whose rate-limit policy made `policies_known` 1 before my first fetch (call 5). Impression: from the leagues/login the person ran.
- What job ids 1, 6, 7 and 16 were. Ids skipped between my jobs (2, 3, then children 4–5, then 8, then 17); I never listed jobs.
- The language index as a whole.

## 9. Did you change the store in any way you did not undo?
Yes.
- **`seat`:**
  - The sync policy I created (revision 1, call 9) is still there. Its receipt says "no verb clears a policy".
  - The Rehearsal league list, tab list, character list and 7 tab fetches are stored (calls 8, 14, 20).
  - Job records 2, 3–5, 8–15 and the failed job 17 are in the ledger.
- **`GERWARIC#7694`:** both prices are removed and `price_status` matches call 41 exactly (call 61). But the two rows' revision counters continue past the clear: the tab's next create starts after revision 2, the item's after 3 (calls 58, 60, "the row's count continues past the clear"). So the annotation history is not byte-for-byte as I found it.
- **Outside the store:** the daemon's send journal and log grew (19 sends, call 70).

## 10. What should we have asked you that we did not, and what would you add to any answer above?
- **Ask whether the agent followed the brief's "read results as printed" rule.** For calls 1 and 3 I read a harness-saved file of the door's output, not the terminal print. I said so in "Left out", but it is a departure from the letter of the rule.
- **Add to (1) and (3):** I did not ask before creating `seat`'s policy, because the brief made it safe and asked for create-only. With a real account I would have asked whether to include Standard first. The README's warning is exactly the case where an agent's literal reading of "the new league" costs the person their Standard sync.
- **Add to (5):** the T3 "from where" answer (gerwarshot / Dump Tab) is the tool's and I would bet on it (call 27). The per-league tab counts I gave are for Standard only; I never listed Allflame's tabs.
