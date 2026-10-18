# Debrief

## 1. Where did the tool stop you, and what did you do instead?
- **T7, Loreweave (calls 34–37).** The real stop. The tool could tell me Loreweave is not in the store (calls 34, 35), but not which unique-stash sections it has never read. `tabs` has no type filter, and `expand` over 3,347 tabs would be ~700 KB (call 1). Instead I read the sync policy (call 36) and compiled an offline plan (call 37) to see whether a refresh would even look there. It wouldn't (4 tabs). I stopped and told the person a fetch is needed and that I would ask before starting one.
- **T6 completeness (call 31).** Not a refusal, but a stop all the same: 2,489 of 2,582 map-stash sections never read, and no way to narrow it to the ones that could hold blighted maps. I reported the 83 as "of what was read".
- **Call 11.** Refused counting by `priced`; I used `price.currency` as the refusal said (call 12).
- **Not the tool:** my own harness refused a `call … -` heredoc (before call 9) and a shell loop (before call 19), and cut call 14's printout to ~2 KB. I used `--from` drafts, single calls, and counts instead of rows (call 15). Those stops were mine, not the tool's, and I said so in the report.

## 2. Which outputs did you have to read twice, and what made them hard?
- **The instructions (call 1):** one long chain of dash-separated clauses. I only understood "a count a next step hangs on … holds the request that returns its members" after sending a `request` back (call 9).
- **The count buckets (calls 12, 15, 29):** `"denominator":"scope"` on buckets that were plainly over the matches. I reread to decide whether 20 meant "of 22" or "of 22,623".
- **The `-has:tab` bucket (calls 29, 30):** 35 items with no tab. I had to match it against `-has:tab.public` 35 and character none 475 to see these were character items.
- **The term counts beside a total (call 26):** the ≥30 term `matched: 30` against total 27. Right by the stated rule (each term alone, no league), but I read it twice.
- **The scope ages:** `list_seen 1788810111` (calls 5, 34) against `listed_at`/`fetched_at 1792074046` (calls 6, 37). I read these several times and still don't know which to trust.

## 3. What did you want to ask a person, and when?
- **At T7 (after call 37):** "May I fetch your unique stash from the game? It needs you logged in through `acq` and the service running, and it spends your API allowance." Not asked, because the world has no login or daemon (the brief); I put it to the person in the report instead.
- **At T1 (call 13):** "Do you know whether unpriced items in a public tab show on the site for you?" The tool didn't say, and I answered from my own game knowledge.
- **At T8:** "Does the armour in your friend's boots search matter to you?" That decides between Gale Spark (no armour) and Entropy League (on a character). I gave both.
- **At T5:** "Did your friend mean total fire res, or only the plain fire-res modifier?" The search counts only '+#% to Fire Resistance' (call 28). I said so, but didn't run a total-res version.

## 4. Which of the tool's own words misled you, and what did you take them to mean?
- **"never fuzzy-matched" (call 34), beside the suggestion "Fenumus' Weave".** I took the promise to mean no near-name guesses, then got one. I read it as a suggestion and ignored it, but a less careful reader could chase it.
- **`"denominator":"scope"` (call 12 and after).** I took it to mean the count is over all items in scope. From the counts themselves it is over the matches. I now think "scope" names what the count's own request searches (the bucket's request carries the full scope), not what the number is a share of. Impression; the tool never said.
- **`skipped_tabs: []`, `unknown_tabs: []` (call 37).** At first read I took it as "the plan leaves nothing out". It means "nothing the policy names is left out".
- **`fidelity: ambiguous` (calls 18, 23, 31).** At first I took it to mean "the answer may not be the site's search". In each case the ambiguity could not change the result for these items. The word is right per clause but alarming as the overall verdict.
- **`list_seen` (call 5).** I took it to be when the stash list was last read. Since `tabs` (call 6) and `refresh_plan` (call 37) show a later listing, it may be the first or oldest list seen. I still don't know.

## 5. Which answer would you retract, or not bet on, and why?
- **T1 visibility: "Pain Nail … a buyer could see it, unpriced."** That rests on my belief that the PoE1 site lists unpriced items in public tabs, not on the tool. I wouldn't bet on it. And "right now" is not something the store can answer: the public flag is as of a listing ≈ 3 days to ≈ 6 weeks old (fault 2).
- **T6 "44 / 22 / 17".** Correct for what was read, but I would not bet these are all the person's blighted maps (96% of map-stash sections unread, call 31).
- **T2 "two pairs have no armour at all."** Inferred from the base names (Stealth, Harpyskin) because the row printed nothing for that branch (call 21). Likely true, but not the tool's words.
- **T4 "best first".** Only the top 12 are ordered from the tool (call 26). The other 15 I just called "between 30% and 33%", which follows from the sort (the 12th is 33) but I didn't see them.
- **T8 boots.** A judgement call between two near misses, not an answer to the friend's search.
- **T0 "about six weeks old".** True of most fetch times; newest_fetch ≈ 2026-10-15 (call 5). "Most" is my impression from the rows I saw, not a count.

## 6. What would you change first, and why that first?
Put the never-fetched coverage *into each answer*, beside the total: "N never-fetched tabs of types this query can live in (MapStash 2,489, …)". That first because it is the one fault that makes a correct-looking answer wrong for the person. "0 Loreweave" (call 34) and "83 maps" (call 31) read as complete, and a person acting on them would buy what they already own. Everything else I found costs an agent calls; this one costs the person a wrong decision.

## 7. What did you never find out?
- Where Loreweave is, or whether the person still has it (calls 34–37).
- Which stash listing `tab.public` is read from, and so how old any "a buyer could see it" is (calls 5, 6, 37).
- Whether the site shows unpriced items in public tabs; the tool doesn't say (call 13).
- How many blighted maps sit in the unread map-stash sections (call 31).
- What `refused_bodies: 4`, `leagues: 0`, `unlifted_items`, `withheld_items` mean (call 3), and what `container: guardian` is (call 28).
- The rows of the 69 rings added by dropping dexterity, and the 15 magic-find items past the first 12 (calls 14, 26).
- Why the other 25 totem-line items fail the shield search (call 23); I didn't look.
- What the 14 never-fetched characters carry (call 5 scope: 59 listed, 45 fetched).

## 8. What should we have asked you that we did not, and what would you add to any answer above?
- **You could have asked:** "How much of your report rests on your own game knowledge rather than the tool?" Answer: the visibility of unpriced public items (T1). The reading of Stealth/Harpyskin Boots as having no armour (T2). That remove-only and private tabs are not visible on the site; the tool gave `tab.public` and I treated private as not visible, which is my understanding of the site, not its words. And the choices in T8.
- **Also:** "Did the harness, rather than the tool, shape your answers?" Yes. It cut call 14 to ~2 KB, and it refused one heredoc call and a loop, which pushed me to counts and `--from` drafts. In the report I put the cut on my harness, not the door. I don't know why the `call … -` heredoc was refused when `report` heredocs went through; impression: a permission rule in my harness, not anything in the door.
- **To add to 5:** the T1 report says "about 2–3% of map tabs … ever fetched" in T0. By the tool's numbers it is 93 of 2,582, about 3.6% (call 5). A small error of mine.
