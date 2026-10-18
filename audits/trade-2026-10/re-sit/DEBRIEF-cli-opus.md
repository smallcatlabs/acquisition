# Debrief

## 1. Where did the tool stop you, and what did you do instead?

- **"Can a buyer see it?" (T1–T6).** No verb or field answers this directly. I asked `--describe price,listed,public`, was refused for `listed` (call 12), took `tab.public`/`price.side`/`priced` from the list the refusal printed (call 13), and then used `--cross tab.public,price.side` plus a filtered search for each task (calls 14–15, 17–18, 29, 31, 33).
- **T7, which unique-stash sections were never fetched.** The tool gives the count (437 of 521, call 36) but not their names. `tabs` has no type filter, and `--expand` would print 3347 tabs, past the door's cut. Instead I checked that both Standard unique tabs have fetched body armours (calls 38–39) and told the person what I could not see.
- **T2, nothing matched (call 19).** The tool didn't stop me here: its offers took me to `--explain` (calls 20–21), and I wrote my own loosened query (call 22).
- **Call 10.** My harness, not the tool, stopped me: it spilled the combined output of calls 9–10 to a file outside W. I didn't open it and reran call 9 alone as call 11. `--describe trade` stayed unread.

## 2. Which outputs did you have to read twice, and what made them hard?

- **The terms block** in every search (e.g. call 26: term 1 "30 matched" against "total 27 matches"). Its counts run over all leagues, not over the matches. The block says so in one line, but the two numbers side by side read like a contradiction.
- **Call 27's sort labels.** "sorts by sum(line(…).arg1) 47" next to "sum(… source=explicit …) 39" on the same row. I had to work out that the sort used a different figure from the filter.
- **Call 32 (`--json`).** Clauses `stats.1`, `stats.1.0`, `stats.1.1` and no `stats.0`. I read it twice looking for a clause I might have missed.
- **Call 8's header.** "exact — every clause with a stash meaning ran as the site reads it", followed by "over the items the store has fetched … the search is the site's and the items are yours". The sentence is dense; I read it as "same search, your stale items".
- **Call 41's `price none (none)`.**

## 3. What did you want to ask a person, and when?

- **T1, before call 14:** "By 'a buyer could see', do you mean in a public tab, or public *and* priced?" I chose public tab, and reported priced apart.
- **T4, after call 27:** "'Best' by the explicit rarity his search filters on, or by your total rarity including implicits?" I gave explicit order and named the implicit where it mattered.
- **T8, before calls 41–44:** "Are you mapping in Standard or in the new league?" Standard items can't move into a new league. I answered for Standard and put the caveat first. I would most want this one asked.
- **T8:** "For boots, do you value armour or resistances more?" Nothing met the whole search, so the pick was a judgment.
- **T7, after call 37:** "May I refresh your unique tabs against the API?" Not possible in this world (the brief). In a real run I would have asked before spending requests.

## 4. Which of the tool's own words misled you, and what did you take them to mean?

- **"exact"** (call 8). I almost took it to mean "you'd get the same result on the site". It means the same *search*, over this store's items. The "over" line corrects that, but the first word is the strong one.
- **"the site shows public listings, so the search is the site's and the items are yours"** (call 8). I took it as a hint that visibility is about public tabs. That led me to `tab.public`. But I'm not sure the tool means "public tab = listed". That's my reading, not its statement.
- **`'sum("T")'`** in the `--sort` help (call 7). I took it to mean the same sum as the trade term. It isn't: it spans sources (call 27).
- **"none (none)"** (call 41). At first I took it as an empty price and an empty side, a guess. `price show` (call 42) explained it.
- **"Remove-only"** (call 4 footer: "items can only be taken out"). I took it to mean the person can still pick items up there, and told them so in T8. I believe that's right.

## 5. Which answer would you retract, or not bet on, and why?

- **"A buyer can see N" in T1, T4 and T5.** I wouldn't bet on these as stated. I equated "visible" with `tab.public=true`, unpriced items included. Whether the site lists unpriced public items is my game knowledge (impression), not anything the tool said. And the data is 37–46 days old (calls 4, 5), so "right now" is not known.
- **T5's line that hybrid and all-res mods "don't add" and "the site's stat would not either".** The second half is my game knowledge (impression), not the tool's. The first half I inferred from the rows (call 30). I never checked an item with a hybrid mod against the query, e.g. with `show --against`.
- **T1's "as far as I know the site's pseudo life does the same"** (Strength/2 counted as life). Impression.
- **T8's boots pick.** It fails the armour clause (calls 21, 22). It's a judgment, not a match. I said so, but I'd retract any reading of it as "the boots your friend's search finds".
- **The T2 zero.** I'd bet on it for the fetched store (calls 19–22 agree), but not beyond the 40-day-old snapshot.

## 6. What would you change first, and why that first?

A visibility answer beside every `--trade` result: how many of the matches a buyer on the site would see and which ones, defined in the tool's own words (public tab; priced or not; as of when). The person asked it for every search. The tool doesn't name it, so it cost me a refusal and two extra calls per task (calls 12–15, 17–18, 29, 31). It is also where my answers are least sure (question 5). Everything else on my list costs a call; this one costs correctness.

## 7. What did you never find out?

- Where the Loreweave is, and which unique-stash sections were never fetched (calls 36–39).
- Whether the trade site lists unpriced items from public tabs. The tool never says (call 8).
- What the T5 URL's `stats.0` was, if anything (call 32).
- What `--describe trade` says about each clause (call 10, unread).
- What "intent 33", "derivation 12", "facts v8" and `listing_only` mean (every search, call 32).
- Whether the URLs carried a site sort. "The site's sort is reported" (call 7), but none was reported in any trade answer (calls 8, 19, 23, 25, 30, 33). I don't know whether that means no sort or no report.
- Whether blighted maps sit in the 2489 unfetched map-stash sections (call 33).

## 8. What should we have asked you that we did not, and what would you add to any answer above?

**A question to add:** "Which claims in your report rest on your own knowledge of the game rather than on the tool?" Several do:
- that unpriced public items are listed on the site
- that site pseudo life counts Strength/2
- that the site's fire-res stat excludes hybrid mods
- that Standard items can't move to a new league
- that Loreweave is an Elegant Ringmail (call 37 searched that base on my word)
In the report these sit beside tool-cited facts and are not all marked as mine. A reader should treat them as impressions.

**To add to question 1:** I never used `show --against` to check a pick or a borderline item against a query. It was there (call 40) and would have turned two impressions from question 5 into facts.

**To add to question 5:** the T1 report says call 9 "got mixed in with call 10". More exactly, call 9 ran and its output was shown only in part, because my harness spilled the combined output. Call 11 is the same command, rerun.
