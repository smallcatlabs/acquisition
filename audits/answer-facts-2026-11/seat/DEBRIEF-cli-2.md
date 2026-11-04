# Debrief

## 1. Where did the tool stop you, and what did you do instead?

- **Call 11.** A store verb refused once two accounts existed. I added `--account` from then on.
- **Call 47.** A search refused without `--realm`. I added `--realm pc`.
- **Call 49.** `has:line("# to maximum Life")` was refused. I switched to the trade-site pseudo totals (`pseudo.total_life>0`), which changes the meaning: Strength counts toward that total.
- **Calls 66–67.** `-priced` and `-is:priced` were refused. I read `--describe=priced` (call 68) and used `-has:priced`.
- **Call 43.** The `--describe` index was 33.5 KB, more than my harness showed inline. I asked for slices instead (calls 44–46).
- **Calls 30–33.** These weren't a stop, but there was nothing left to plan, so the careful apply under a ceiling ran on an empty plan. I described how I would have done it instead of changing the person's `max_age_seconds`.
- **Call 79.** The undo printed by call 70 was refused as stale. I ran the revision the refusal offered (call 80).
- **Call 98.** `dash` needs a terminal. I used `dash --json` (call 99).
- **Call 104.** The API verb refused without a login and spawned a daemon. I stopped it (call 106).
- **My harness, not the tool:**
  - It refused one compound shell command early on, so I ran each door command alone.
  - It refused the heredoc for the report's last part, so I used `W/drafts/report-tail.md` with `report --from`.
  - My own malformed door call (`--page` before `--`) was refused by the door and has no call number.

## 2. Which outputs did you have to read twice, and what made them hard?

- **Call 27 (the plan).** "5 requests, 5..15 wire sends" next to "plus a HEAD probe … and an OAuth token refresh". I had to work out that "requests" are logical and "wire sends" include up to two 429 re-sends each.
- **Call 42 (price status).** "492 the game decides … 489 priced in game … 1395 items carry price text the trade site cannot see". There are three overlapping counts and nothing joins them. I reconciled 492 − 489 = 3 skips myself.
- **Call 48 (search).** The scope paragraph mixes a realm-wide never-fetched count, "location list seen 57d ago" and a per-league `tabs` route. I could not tell which set each part described.
- **Calls 32–33.** Two identical "53s ago" lines, seconds apart. I read them twice to be sure the age had not moved.
- **Call 86.** I spotted that `list_seen` equals `oldest_fetch` only on a second read.
- **Call 59.** "reads invalid: a ratio (5000/2) in a tab name is invalid and lists nothing, public". The trailing "public" belongs to the tab, not to the reading.

## 3. What did you want to ask a person, and when?

- **Before call 23.** Should the new-league policy for `seat` also keep Standard? I wrote Rehearsal only, which is the literal ask.
- **Before call 18.** There is no character in Rehearsal. Should I fetch a Standard one, or report none? I did both.
- **At call 30.** May I shorten `max_age_seconds` so that the careful apply has work to do? I did not, and said so.
- **Before call 77.** May I write a policy on the person's real-copy account GERWARIC#7694, and should I remove it afterwards? T6 required it. I left it in place (see 9).
- **Before calls 65 and 70.** Which tab and which item to price. I chose Delve and a 6-link bow myself.

## 4. Which of the README's or the tool's own words misled you, and what did you take them to mean?

- **README: "Every command takes `--json`, and it is total: a failure is `{"error":…,"kind":…}` … its `offers`".** I took that to mean every failure carries `kind` and `offers`. A failed job (call 92) carries neither. The README does go on to say a job failure is its outcome record, which I then read as the exception.
- **README "A new league": "the first plan is the listings, one or two".** I expected a listings-only first plan. Call 27 went straight to tabs.
- **`tabs` (call 37): "20231 items".** At first I took it as Standard's item total. It is the items in tabs (calls 42, 51).
- **"facts: stash listed 53s ago" (calls 32, 33).** I read it at first as the age now. It is the age at plan time.
- **"location list seen 57d ago" (call 48).** I took it to mean the stash listing's age. Call 86 suggests it is the oldest fetch. Impression: I never settled this.
- **`kind: "not_logged_in"` (call 90).** It suggested a login problem, but the account simply did not exist.
- **README "price set … with undo".** I took the undo to be durable. It is pinned to a revision and goes stale after the next write (call 79).

## 5. Which answer would you retract, or not bet on, and why?

- **T4(b)'s "224 rings".** It is the trade site's pseudo notion. It counts Strength toward life (call 46) and one all-res line as three resistances (call 53). A strict reading of "life and two resistances" could give fewer. I would not give 224 to the person without that caveat.
- **Fault 13 (the store grew 54.4 → 56.3 MB).** I do not know the cause. It may be the tool's normal behaviour, and I would drop it as a fault.
- **Fault 8 ("location list seen" is mislabelled).** It rests on one coincidence of timestamps (call 86). Impression.
- **Fault 9 (class and level disagree) and the `index: 0` point.** Both may be the mock's invention, not the tool's.
- **T3's "oldest fact in the store is 62d (refused) or 57d (characters)".** The 57d comes from the 10 of 65 characters that call 40 listed, plus the pc-only search scope (call 86). I never saw the poe2 characters' ages. I would not bet on it as the store's oldest.

## 6. Which claims in your report rest on your own knowledge of the game rather than on what the tool printed?

- That the six-links found are "body armours and a bow". I recognised the base types (Simple Robe, Astral Plate, Imperial Bow); the tool did not say "body armour".
- That Rehearsal is "the new one". I inferred it from "started 4d ago" and its category. The tool said a new league is listed "the day it opens" (call 10) but did not mark it as new.
- That the gaps in Delve's x positions are empty cells (T6). This is inference.
- That "~skip" means not for sale. The tool said "do-not-index (~skip)" (call 88). "Not for sale" is my gloss.
- That "the trade site lists them without the forum" for game-priced items. This paraphrases call 88's "the game already lists it", with my game knowledge added.
- That the Standard gap of 1080 items is characters' items. I verified it only for Allflame's 211 (calls 40, 52, 51). For Standard it is inference.

## 7. What would you change first, and why that first?

I would give `store status` the age of what the store holds: the oldest and newest fetch with their locations, the league list's age, and the refused bodies' ages. "How old is what you know" is the person's first question, and `store status` is the README's named answer for the store. It is also the one place where the tool says nothing about time, although it computes those ages elsewhere (calls 42, 48). It is cheap, and it fixes the most-asked question.

## 8. What did you never find out?

- What "location list seen" means (calls 48, 69, 86).
- Whether `--max-requests` refuses a plan over the ceiling, and what "spent" looks like on a re-apply with work. The plan was empty.
- How to search "has this line" (call 49).
- Whether a map or unique tab's second round happens (the help, call 26, says a first sync takes two). The mock's tabs were empty.
- Why the store file grew (calls 5, 36).
- What "residue" means beyond its one-line help, and what `derivation` and `intent` mean in the basis.
- Whether the class and level mismatch is the mock's or the tool's.
- How many items sit in tabs with invalid names. There is no count for them.
- The poe2 realm's ages and contents.

## 9. Did you change the store in any way you did not undo?

Yes:
- **A policy on `seat`** for Rehearsal, revision 1 (call 23). This was the task's goal.
- **A policy on GERWARIC#7694** for Standard, revision 1 (call 77), which I did not remove. The world as found had none (call 7).
- **The price rows' revision counters** continue past the clears (calls 80, 81). The prices are gone but the rows' history is not "as found": the next price write on those targets will see revision 2.
- **Fetched data for `seat`.** The league list was fetched twice (calls 14, 84; the second was my T7 rerun, a real fetch), plus the character, the stash tabs and the apply's 5 tabs.
- **Two failed 404 jobs** (19, 20) are in the job ledger.
- **`seat` is logged out** (call 100), and the daemon is stopped.

## 10. Which age did you quote for a set it was not the age of, and which number did you take for a whole it was not?

- **Ages:**
  - In T3 I gave "57d" as the store's oldest character fetch from call 40, which showed 10 of 65 characters. It is the age of those ten (corroborated for pc by call 86), not of all 65 or of poe2.
  - In T6 I wrote that the posted items were fetched 56d ago and that the "stash listing [was] 53d ago" (call 76). Those are right, but I let the listing age stand beside the positions, and the tool itself warns the positions predate it.
  - In T2 I quoted the quote's "seen 30s ago" as the limiter's state. It is the age of the last header for that route only.
- **Wholes:**
  - In T5, "What you have priced: nothing by hand … 489 in game" is **Standard only** (calls 42, 57). I did not check Allflame, which holds 97 tabs, so "what have I priced" was answered for one league as though for all.
  - In T3 I first treated `tabs`' 20231 as Standard's whole before correcting it.
  - In T4(c), "no other pc league holds a live item" rests on call 51 covering pc only. Poe2 was not counted.

## 11. What should we have asked you that we did not, and what would you add to any answer above?

- **You could have asked how long the run took and how my sense of time compared.** The daemon said "up 370s" at call 95. The whole API part of the run took about six minutes, and the "minutes" I felt between commands were not real time. Ages like "4s ago" and "5m ago" (calls 29, 101) were the honest clock.
- **To 9 and 10:** I reran the shop as JSON after the cleanup (call 88). The T7 comparison of a shop with posted items as JSON was lost through my own ordering, not the tool's.
- **To 1:** I never opened the harness's saved copy of call 43's output, so I read the query language only in slices. Some syntax I may simply have missed.
