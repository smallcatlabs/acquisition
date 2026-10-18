# Debrief

## 1. Where did the tool stop you, and what did you do instead?

- **"Would the site show this?"** The tool couldn't answer it in one search (calls 7, 13). Instead I added `has:priced` to each query (calls 13, 17, 30, 35, 43), cross-checked a few items with `price show` (calls 14, 26, 47), and read `price.side game` off the rows (calls 17, 35). That left one gap I can't close: an *unpriced* item in a *public* tab, which the site can show with no price. I said so in the report.
- **`--count` with `--cross`** was refused (call 33). I split it and ran `--count` alone (call 34). For T6 I ran `--cross` alone (call 41).
- **Item age.** `show` has none (call 46). I found no other way and gave the person the store-wide "about 40 days" (call 7) as a caveat.
- **Harness limit, not the tool's:** calls 9 and 10 were too big for my display, so I read the files my harness saved them to.
- **My own permission denial:** a shell loop over four `--explain` calls was refused by my harness's permission mode (not a call of the door). I ran them one by one (calls 19–22).

## 2. Which outputs did you have to read twice, and what made them hard?

- **The `terms` block** (every search, e.g. call 7). The counts are "each term counted alone over all live pc items in all leagues, never over the matches". On first read I took "1133 matched" for rings in Standard. It is over all leagues, and the per-term numbers say nothing about the query's matches.
- **Call 30 against call 31.** Tab ids in the count didn't match the ids in the rows (unique-tab sub-tabs). I had to reason out that `52848d23bf` is the parent of `e63d0f7d9d` etc. (confirmed only indirectly by `price.from tab/pc/52848d23bf`, call 35).
- **Call 42.** I asked for untiered maps and got 977 items, mostly ultimatums and invitations. I had to work out that `category=map` is a parent.
- **Call 43.** I needed arithmetic (26 = 14 + 12, 70 = 69 + 1) to place the ravaged maps by tab, because a `--cross` and a `--count` couldn't be had together.
- **The trade header's "over" line** (call 7). I read "the search is the site's and the items are yours" three times before deciding it meant "same filter, your items", not "these are what the site would show".

## 3. What did you want to ask a person, and when?

- **At T7 (before call 44):** "Will you map in Standard or in the new league?" Standard items can't go to a new league, and the person's opening line mentions both. I answered for Standard and said so.
- **At T7 (after call 48):** "Is your totem build fine with Blood Magic?" I warned instead and named an alternative.
- **At T7:** "What does 'best' mean for you: life, resistances, rarity?" I chose total resistance and said it was my judgement.
- **At T6:** "Do you want your map tab downloaded?" Only 61 of 2550 sections are in the store (call 4). In the real world that needs the API and a login, which I would not start without asking.
- **At T0:** nothing. The world was read-only and safe.

## 4. Which of the tool's own words misled you, and what did you take them to mean?

- **"the search is the site's and the items are yours"** (call 7). At first I took it to mean the rows are what the site would show of mine. It means only that the filter is translated faithfully.
- **"exact"** (call 7). I briefly took it to cover the site's *visibility*, not just the clause translation.
- **"newest 2d ago"** (call 7). I took it to mean something was refreshed two days ago, and then couldn't find what (calls 4, 5). I still don't know what it refers to.
- **"priced" / `has:priced`** (call 10). I took it as "the site sees a price" until I re-read the definition: it includes the owner's hand rows. In these tasks every priced match had `price.side game`, so it didn't change an answer, but it could.
- **`listing_only`** (call 9). I took it to mean "a clause about the seller's listing, so not run on your stash". I believe that's right, but it's my reading. *Impression.*

## 5. What would you change first, and why that first?

**A "site-visible" field and row marker** (priced in game on a public tab, or in a public tab at all). The person asked "would the site have shown me the same?" for every search. The tool's whole `--trade` feature exists to bridge the site and the stash, and that bridge is exactly what's missing. Everything else (age, ids, sort shorthand) costs extra commands. This one costs a right answer: I could only infer it (calls 13, 14, 30, 35, 43).

## 6. What did you never find out?

- What was fetched "2d ago" (call 7).
- When Dump Tab, Soulrend PF and Facebreaker Totems were last fetched, so how stale my T7 directions are.
- Whether any of the 22 rings, or the other matches, sit in a public tab with no price, which the site *would* show (calls 13, 14).
- Whether the site rounds the half-life from Strength in `pseudo.total_life` (87.5, call 7; 77.5, call 17). That decides near-threshold matches like the 50-life floor in T1.
- What `"verified": false` and "not persisted" mean (calls 46, 2).
- What the 2489 unfetched map-tab sections hold (call 4).

## 7. What should we have asked you that we did not, and what would you add to any answer above?

- **You could have asked: "Which of your answers to the person would you bet on, and which not?"** I'd bet on the match counts (T1 22/91, T3 1, T4 160/27, T5 510, T6 83), because the tool marked each clause and I tested the flagged ambiguities (calls 36–38, 42). I would not bet on the "site would show N" numbers, which are inferences (finding 1), or on T6's count being complete (finding 3).
- **Added to 1:** the most useful thing the tool did when it stopped me was call 18's ready-made `--explain` list. A zero-match answer that tells you which filter to relax is the right design, and I'd keep it.
- **Added to 2:** printing the per-term counts over the *whole* corpus every time (calls 7–43) made every answer long. Counts over the matches, or none, would have read faster. *Impression.*
- **A process note:** two report claims rest on reading harness-saved files (calls 9, 10). If that breaks the brief's rule, then the T1 use of `has:priced` came from a file I shouldn't have opened. The same field appears in `price status` (call 12), though not under that name.
