# Debrief

## 1. Where did the tool stop you, and what did you do instead?

- **`tabs` (call 9).** 745,672 bytes for one league, with no limit or filter, and the door cut it to the head. I used the `tab` key in search counts instead (calls 12, 24, 33, 34). That told me *where* items are, but never *when* those tabs were last read. That loss stuck: in T6 and T7 I could not date 'Blight Maps' or 'Facebreaker Totems'.
- **Call 10.** The default 20 rows of a trade search came to 31 KB. My harness, not the door, moved it into a file I may not open. After that I asked for counts first and rows in small pages (calls 11, 12, 25, 26). That is a workable habit, but I had to invent it.
- **"Would the site show me the same?"** The tool has no notion of a public tab. I fell back on `price.currency` counts (calls 14, 27) and on my own knowledge of the game ('Remove-only' tabs, worn items). In the report I said which part came from the tool and which from me.
- **Total rarity (T4).** No name was offered, and `describe ["computed"]` is said to be over 100 KB. I guessed `pseudo.increased_rarity` (call 29). The guess worked; a wrong guess would have been refused with a list of names, which I did not test.
- **Two refusals I recovered from at once:** `realm_needed` (call 2, whose offer I sent as call 3) and `realm` as a counts key (call 6, then call 7).

## 2. Which outputs did you have to read twice, and what made them hard?

- **The trade clause notes (calls 11, 13, 17, 31, 34).** Each note packs a rule, its evidence and a citation into one sentence. For example: "within one of the site's own figure, which works from the roll the display has rounded: equal on 1,148 of the captures' 1,313 defence rows … the owner on ambiguous, 2026-10-15, verbatim: \"confirmed\"". To get "armour may be off by one near the bound" I had to read it twice.
- **Call 32's `terms`.** For the term `-sum(...)>=30`, the block reports matched 526: the count of the *positive* inner term, not of the negated term I wrote. I had to check `path: "2.0"` to see it was the inner node.
- **Call 36's sort value.** `total_res: 105` for Storm Band, while the shown lines add to 92 (43 + 36 + 13). I worked out that a hybrid 'Fire and Cold' line counts twice. That is the site's rule, I believe, but the row does not say so. (Impression on the site's rule.)
- **Place names (calls 25, 26).** `"name":""` sat next to `"parent":{"name":"~price 4554 chaos"}`. At first I read the item as being in a tab with no name.
- **The scope block on every answer (call 3).** `fetched: 546, never_fetched: 2940, folders: 17`. I still don't know what the unit is.

## 3. What did you want to ask a person, and when?

- **T1, after call 14:** "Which of your tabs are public?" The answer to 'would the site show the same' turns on it, and neither the tool nor I can know it.
- **T4, after call 28:** "When you say 'at least 30', do you mean as the site counts it (explicit only) or what you'd actually have (with implicits)?" I answered both instead (calls 24, 29).
- **T7, before call 36:** "What build or which character are you mapping with?" Picking 'the one ring' depends on it. I chose by total resistance and named the alternative (Fate Twirl).
- **T0:** "May I run a refresh plan?" `refresh_plan` is offline and sends nothing (call 1), so it was probably safe. I left it alone because no task needed it. I now think T7 needed it, to tell whether the September-old boots and shield are still where the store says.

## 4. Which of the tool's own words misled you, and what did you take them to mean?

- **`fidelity: "exact"` (call 11), read with `search`'s text "partial where the remainder is not empty, the answer then not the site's" (call 1).** I first took 'exact' to mean 'this is what the trade site would return for you'. It means the clauses translated without loss. I caught it only because the rows held a worn ring (call 11) and no prices (call 14).
- **`status: "listing_only"` (calls 11, 13).** I took it to mean 'this clause concerns listings, so it is dropped'. That is right, but I also took it to mean the tool had dealt with the listing difference. It hadn't; it only set the clause aside.
- **`place.name` (calls 25, 26).** I took it as the tab's name. For a substash it is the child's often-blank name.
- **"Begin with search with no arguments" (call 1).** It does say a store of several realms refuses. Not misleading, but the first instruction leads straight into a refusal (call 2).

## 5. What would you change first, and why that first?

Make every trade answer say what the site could and could not show: a count of matches that are worn, in inventories, unpriced, or in tabs the tool doesn't know to be public. Reword `fidelity` so it cannot be read as 'the site's result'.

Why first: it is the only fault that produces a **wrong answer to the person's actual question**, stated confidently. The person said trust depends on knowing "where it might not" match the site. Every other fault (size, blank names, jargon) costs effort or calls. This one costs correctness, and a person driving the server would be misled exactly as I nearly was.

## 6. What did you never find out?

- Which tabs are public, and whether the store holds that fact at all.
- What `fetched` / `never_fetched` / `folders` count (call 3), and why 546 + 2940 ≠ 3444 tabs (call 5).
- What `fetched_items` vs `item_count` means for a character (call 8).
- What `container=guardian` is (calls 7, 31).
- When 'Blight Maps', 'Facebreaker Totems (Remove-only)', 'Lab Runners' and 'MARKET' were last fetched (call 9 cut).
- What the one item with no `category` is (call 11, `lacked: 1`).
- Whether `reqlevel` searches treat a '(gem)' requirement as the item's own (call 39).
- What C81 means, beyond its sentence (call 28).
- Whether the site's Blighted filter includes Blight-ravaged maps (call 13 says no sitting has asked).
- What call 10 said.

## 7. What should we have asked you that we did not, and what would you add to any answer above?

**A question you could add:** "Which of your answers to the person would you now retract or soften?" My answer:
- T2's "you'd see nothing on the site either" holds.
- T7's boots and shield pickups ("log into mrs_flowers", "column 7, row 3") rest on data last seen 2026-09-08 (calls 38, 39). I should have led with that, not tucked it at the end.
- T1's "the site would show you none or almost none" is my inference from game knowledge, not from the tool. It is labelled so in the report, but a reader might take it as measured.

**What I'd add to 1:** my own counts-first, small-pages habit cost calls (T1 took seven where three would do). That cost belongs partly to me, not only to the tool.

**What I'd add to 5:** second would be making `tabs` pageable. It is the only door to freshness, and freshness is what every 'where is it' answer leans on.
