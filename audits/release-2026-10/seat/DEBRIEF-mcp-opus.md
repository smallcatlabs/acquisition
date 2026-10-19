# Debrief

## 1. Where did the tool stop you, and what did you do instead?

- **Call 5:** search with no arguments was refused (`realm_needed`). I sent back its offer (call 6) and named a realm from then on.
- **Call 20:** `price_list` of the conflicts was 66,806 bytes and the door cut it. I switched to a search cross of tab × price.side (call 21), which cannot filter by relation. I never listed the 50 conflicts.
- **Call 32:** the whole `shop_render` was 5 MB and cut. I used `page: 1` (call 31) for the page itself, and inferred the unseen policy cells were 0 from the arithmetic (the seen cells already sum to 21,311). Freshness is lost.
- **Calls 40–42:** with no daemon, the job tools all failed. I stopped there, as the brief intends, and wrote down what I would tell the person instead.
- **Calls 3 and 19:** I found no tool for the 4 refused bodies or the 1 invalid note. I tried `item_events` (call 10), which didn't help, and then reported it as not found.

## 2. Which outputs did you have to read twice, and what made them hard?

- **Call 1 / tool texts:** long sentences in which a clause about one field runs straight into the next (search's text is about 3 KB in one paragraph). I reread `price_set`'s undo rules before T4.
- **Call 19 (`price_status`):** `by_relation`, `by_effective`, `by_game_statement`, `game_priced` (489) and `residue` overlap. I reread it to reconcile 531 effective against 525 `has:priced` (call 21) and never fully did.
- **Call 32's `policy`:** to check that the counts summed to 21,311.
- **Call 25:** 28 KB to find the tab's own listing at the top, then counting `items_here` cell by cell. I got 41 the first time and 39 the second, and corrected it in T5.
- **Call 14:** two big count tables, read to find the `pseudo.*` names.

## 3. What did you want to ask a person, and when?

- **T3(b):** whether "life" meant flat `+# to maximum Life` or total life including Strength, and whether "two resistances" meant two kinds or two lines. I chose and stated my reading.
- **T4:** which tab and which item they wanted priced. I picked `Items 1` and The Perfect Form myself, because the task leaves it to me.
- **T6, before `submit_job` (call 40) and `apply_plan` (call 42):** against GGG I would have asked first. Here the brief says the world is safe, so I didn't.
- **T7, before call 47:** whether a deliberate stray price row was acceptable. I did it and cleared it at once (call 50).
- **T4:** whether the `~price 4554 chaos`-style tab names were intended (call 21).

## 4. Which of the README's or the tool's own words misled you, and what did you take them to mean?

- **"Begin with search with no arguments" (instructions, call 1):** I took it to work on any store. It was refused (call 5).
- **"a refusal is a result marked isError with readings and offers" (call 1):** I expected every refusal in that shape. Calls 40–42 are protocol errors; calls 45, 48 and 49 have no `kind` or `readings`.
- **`price_set` "refused naming the current revision":** I expected a field. It is prose (call 48).
- **`item_events` "from recent ingests":** I expected newest first. It returned the oldest (call 10).
- **`provider: mock` (calls 1–3) against the README's "made-up stashes":** for a moment I took the data to be fake. The brief corrected me.
- **The README's "and four more" tools:** I took the list to be nearly complete. There are 25 tools (call 1).
- **`fetched` in search's scope (546, call 6):** I first read it as tabs only. Impression: it counts tabs plus characters.

## 5. Which answer would you retract, or not bet on, and why?

- **The T2 dates:** hand arithmetic. Only the newest age is cross-checked, by `age_seconds` 361286 (call 39). "About 7 Sep" and "about six weeks" I would not bet on to the day.
- **T5's left-out counts:** the tail of call 32 was unseen. The zero-tail is an inference from the sum.
- **T4's "the 6 are skip/no-price":** a guess about 531 vs 525 (calls 19, 21).
- **T3(b)'s 195:** may include rings with negative life (call 14 shows a minimum of -109). I would bet on "about 195" but not the exact number.
- **Fault 1's claim that the CLI likely guards this with `--expand`:** impression, from the README tour line alone.
- **Fault 2's claim that the CLI behaves the same:** inferred from the tool text, not tested.
- **The `apply_plan` remark in T6:** I can't tell whether it validated the plan before the daemon check. I hedged it, but it reads as a criticism and should be treated as unproven.

## 6. Which claims in your report rest on your own knowledge of the game rather than on what the tool printed?

- That +248 life and "Adds … Cold Damage to Attacks against you" are odd for a Two-Stone Ring (T3). Game knowledge.
- That the six-link results are "mostly body armours and bows". I read that from the base names, using game knowledge of the bases.
- That `~price 4554 chaos` tab names are "probably test values". My judgment.
- That shop pages are pasted into a forum shop thread, and that items must stay put and tabs public for links to work. Partly game knowledge; the tool says only "for a person to paste" and that links name tab and cell (calls 31, 32).
- That gems, currency and cards make up the "no rarity" bucket (T3c). The field description in call 11 supports it ("a gem, a currency stack, a card has none").
- That a GGG fetch spends a rate limit the person cares about (T0). That comes from the README.

## 7. What would you change first, and why that first?

`shop_render`'s default answer (call 32). It is the one tool whose normal, argument-free use breaks an agent outright: 5 MB lands in context, and the parts a person needs (how many pages, what was left out and why, freshness) are either buried or cut. Every other fault I found has a workaround inside the tool. This one only has `page`, which throws away the explanations. Second would be the daemon refusals (calls 40–42), because they are the moment an agent must hand off to a person and they don't say what to hand off.

## 8. What did you never find out?

- Which note is the 1 invalid note (call 19), and what the 4 refused bodies are (call 3).
- The list of the 50 conflicts (call 20 was cut).
- `freshness` of the shop (call 32 was cut).
- What `residue: 1395`, `unlifted_items`, `withheld_*`, `derivation` and `intent` mean (calls 3, 19).
- Why a character's `item_count` differs from `fetched_items` (call 9).
- Whether `apply_plan` would accept my resent envelope (call 42).
- Whether a real host needs `env` or `type: stdio` in the configuration (call 4).
- Whether the MCP server finds a daemon started by `acq auth check` (T6). Untestable here.
- What the other 11 rows in other leagues price (call 19).

## 9. What should we have asked you that we did not, and what would you add to any answer above?

You might have asked **"what did you do that a careless agent would get wrong?"** The tab row I wrote at 5 chaos would have overridden nothing, but on `ACQUISITION-PRICE-TEST` notes beat tab rows (call 20). A person who prices a tab expecting every item to follow would be misled; `price_show`'s `why` is the only place that says so.

Additions:
- **To 1:** two harness refusals also changed my path: the date conversion, and the heredoc for the T1/T2 report part. I moved to drafts and `--from`, as the brief allows.
- **To 5:** in T0 I first wrote "26 tools" and corrected it to 25 in the next part. The report keeps both lines.
- **To 4:** I read call 1 from my harness's saved file, not inline. That file is outside the two allowed (reported in "Left out").
