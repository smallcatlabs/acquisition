# Debrief

## 1. Where did the tool stop you, and what did you do instead?

- `search {}` refused `realm_needed` (call 24); I sent its `realm all` offer as printed (call 25).
- A counts key I guessed (`realm`, `kind`) was refused `unknown_name` (call 26); I picked `league`/`container` from the list the refusal printed (call 27).
- `line:resist,life` as one key gave 0 values with no warning (call 34); I split it into two keys (call 35).
- No MCP tool lists leagues; I guessed `submit_job kind:"leagues"` from `submit_job`'s text (calls 7–9).
- The refused bodies cannot be named through the MCP (call 30); I told the person to run `acq store refused` and stopped there.
- The item's "remove the price" undo was stale (call 54); I sent the conflict's offer (call 55).
- Not the tool: my harness refused the door's `call … - <<'J'` heredoc for `apply_plan` (and an earlier command that used a shell variable for W). I wrote the arguments to `W/drafts/apply1.json` and `apply2.json` and used `--from` (calls 12, 17). The `report` heredoc worked throughout.

## 2. Which outputs did you have to read twice, and what made them hard?

- Call 25 (and every search after it): the `scope` block. `never_fetched_by_type` holds `fetched` counts and a `character` "type". The oldest/newest ages only made sense once I saw they were the same for every query (calls 32–37).
- Call 52 (`shop_render`, 17,676 bytes): to find where the two prices sat I had to go from the `policy` table to `posted` to the page `text`. The answer was at the end of the page text.
- Call 54: the conflict's `error` is CLI wording with an MCP translation tacked on. The `offers` were clearer than the text.
- Call 1: the tool list, 36 KB, which I read from the harness's saved copy.
- Call 29: `item_count` was larger than `fetched_items`, and I only understood why from `describe store` (call 30).

## 3. What did you want to ask a person, and when?

- T2, before call 10: whether the new-league policy should also keep Standard, and what `max_age_seconds` they want. I took "the new league" literally and chose 3600 from the schema's example. I said both in the report rather than asking.
- T4(b): whether "two resistances" counts chaos, and whether "+% to all Elemental" counts as one or three. I used the tool's `pseudo.count_res` and stated its reading.
- T5, after call 39: the brief led me to expect hand-set prices and there were none. I would have asked whether the person meant their in-game notes.
- T5: which tab and which item to price. I picked Dump Tab (private) and a ring worn by a character, without asking.
- T8: whether I could let the daemon go, to see the no-daemon answers. I didn't, because `seat`'s login would not come back without the person.

## 4. Which of the README's or the tool's own words misled you, and what did you take them to mean?

- The server instructions' "Begin with search and no arguments" (call 1). I took it as a first call that works; on this store it errs (call 24).
- `scope.oldest_fetch_age`/`newest_fetch_age` on every search answer (calls 32–37). At first I took them as the age of the answer's facts. They are the scope's.
- `never_fetched_by_type` (call 25). I took it as "never-fetched tabs by type"; it carries fetched counts too.
- `kind: "not_logged_in"` (call 58). From the kind alone I would have read it as a login problem; the text shows it was a name problem.
- `policies_known` (calls 5, 63). At call 5 it read to me like "a sync policy exists", and I briefly thought T2 might find one. Call 6 said `null`. I now take it as rate-limit policies learned (call 64's wording), but that is inference.
- README "against GGG `acq-mcp` never starts one". I read it as "never starts one", full stop, until call 64 said it does in mock mode.
- The brief's "with the prices they have set", which is the brief's wording, not the tool's. I expected hand rows; call 39 shows none.

## 5. Which answer would you retract, or not bet on, and why?

- **Correction:** in T4(d) I wrote that Kraken Circle was "first seen 59 days ago". The tool printed only `first_seen: 1788361880` with no age words (call 38). I worked out the 59 days myself, which breaks the rule that a time must be one the tool printed. The tool's own fact is the timestamp. Likewise in T3 I wrote "6 more marked removed" from `items_removed: 6` (call 3); that one is printed, so it stands.
- **T8, what to type:** I would not bet on "`acq auth` with `ACQ_PROVIDER=mock` and `ACQ_STORE_DIR=<W>/store`" being complete. The running daemon also has `ACQ_NO_KEYRING` and a log dir under `W/logs` (call 63), and it may need `ACQ_IDLE_SHUTDOWN` to match. Untested; the source is the README.
- **T4(b), 224 rings:** right by `pseudo.count_res`, but I did not confirm how that total counts the all-elemental line. Impression: it counts it as three (call 36, Kraken Circle shows `count_res: 3`).
- **T3:** "two smaller cells I did not list" (call 27, `left_out: 2`). I never looked at what they are.
- **T7, "one request out of 30 per 5 min":** that limit is the stash route's header in call 21. Applying it to GGG is the README's claim plus my inference.
- **T8, `policies_known`:** my reading of it as rate-limit policies is inference (see 4).

## 6. Which claims in your report rest on your own knowledge of the game rather than on what the tool printed?

- "Six-link" means a 6-socket link group, and `linked(size>=6)` is that query (T4a). The query syntax is the tool's; the equivalence is mine.
- The 8,297 items with no rarity are "gems, currency, cards and the like" (T4c). That is my game knowledge; the tool printed only `-has:rarity` (call 37).
- `Ring2` is "the second ring slot" (T4d, T6). The tool printed `inventory_id: "Ring2"` only.
- Telling the person to "fix the note in game", "make the tab public, or move the items" (T5). That is game knowledge; the tool said only that the text "is not where the index can see it" (call 41).
- That counting chaos resistance as a resistance is a judgement call (T4b).
- The guess that the leading space in `" ~price 5 chaos"` is a forum convention (T6). I marked it as an assumption.
- "Post it in the Standard shop subforum" is the README's ("What it does not do"), not game knowledge and not the tool's.

## 7. What would you change first, and why that first?

The search answer's ages: report the oldest and newest fetch of the *matched items*, beside or instead of the scope's (calls 32–37). Every "how old is this" answer a person gets from an agent goes through it. It is silently wrong rather than refused, so nobody would notice. And the brief's T4 asks exactly that question of every answer. The other faults either refuse with an offer, or cost one call.

## 8. What did you never find out?

- What the server and the jobs do with no daemon running (`job_status`'s `next`, `list_jobs`'s `persisted`), and whether the server really spawns a daemon in mock mode (call 64) and what that daemon would do without `seat`'s login.
- When the running daemon will exit (call 63 gives no idle-clock start).
- Which 4 bodies the store refused, and why (calls 3, 30).
- How long a real plan takes before the limits are learned (`eta_seconds: null`, calls 11, 16).
- What the two cross cells left out in call 27 were.
- Why 8 league names hold items in only 3 league×realm buckets (calls 25, 37). Impression: characters listed but never fetched.
- Whether `price_status` read back `manual_only: 0` after the cleanup (not re-run).

## 9. Did you change the store in any way you did not undo?

Yes:
- **`seat`:** the sync policy (revision 1, Rehearsal; calls 10, 23) stays. There is no verb to clear a policy (call 10 `no_undo`). The Rehearsal tab and character listings and 7 tab fetches (calls 12, 17) are in `seat`'s store, and the job ledger holds jobs 2–17, including failed job 17 (call 61). All were asked for by T2 and T7.
- **`GERWARIC#7694`:** both price rows are cleared (calls 53, 55; read back in calls 56, 57). But the row history is not as I found it: the tab's revision count now stands at 2 and the item's at 3. The clear receipts say "the row's count continues past the clear" (calls 53, 55), and call 62 shows the tab's cleared revision 2 is remembered. Call 62 was refused, so it wrote nothing.
- **Outside the store:** `W/work/acq.json`, `W/drafts/apply1.json` and `apply2.json`. The daemon's send journal grew to 18 sends (call 63).

## 10. What should we have asked you that we did not, and what would you add to any answer above?

- You could have asked: "Which answers would a person have got differently from the CLI?" Several of my stumbles were MCP-only: `line:a,b` in a JSON key (call 34), no leagues tool, no `store refused`, the CLI text inside MCP errors (call 54), and `apply_plan` needing the plan echoed back verbatim (calls 12, 17). The README presents the MCP as doing "what the CLI does", and in these places it does not.
- Added to 5: the T3 "where" answer leans on `container` values (`items`, `equipment`, `jewels`, `guardian`). I read `items` as "in stash tabs" because the counts match the `tabs` total (calls 27, 28); the tool never says so.
- Added to 9: I priced a tab and an item in the person's copy of their real collection without asking. The brief made that safe here; on the real store I would ask first, as I said in T0.
