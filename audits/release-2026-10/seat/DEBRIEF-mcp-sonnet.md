1. **Where the tool stopped me.**
- The daemon tools (calls 42–45). I stopped there and never tried `apply_plan`. Nothing told me what to do, so the next call would have been a guess.
- `shop_render` (call 34). The door cut it, so I lost `left_out`, `counts` and `freshness`. I used `page: 1` (call 33) for the page text and the policy head of call 34 for the omissions.
- `search` without a realm (call 5). The offer was sendable, but I wrote my own request instead.
- My harness refused a heredoc (call 17). I used `W/drafts/rings.json` with `--from`, as the brief allows.
- `price_list` with a filter (calls 22, 23, 32). I had no way to limit it and read only the heads.

2. **Outputs I read twice.**
- `open` (call 1). The harness showed me only 2 KB, so I read the saved copy.
- `describe` (calls 12, 13). The operators are in one section, the fields in another, and `composition` came back empty.
- The call 17 `terms` list. My `or` expanded into a long tree, and I checked it against the rows.
- `price_status` (call 20). The row accounting is hard to follow: 16 rows, 5 applied, 11 in other leagues. `agree: 0` and `residue` are unexplained.
- Call 7, which is long and has no summary.

3. **What I wanted to ask a person.**
- Whether "gear" in T3(a) meant armour and weapons only. Armour and weapons are all that `links=6` turned up (call 15), so I did not ask.
- Whether "two resistances" meant two lines or two types (call 17). I picked one reading and said so.
- Whether to touch the existing "ACQUISITION-PRICE-TEST" tab (call 23). I priced the Delve tab instead.
- Before call 42: whether I may call a job tool that might start a daemon. The brief covered it.
- Which forum thread the pages go to (T5). The tool never says.

4. **Words that misled me.**
- "Begin with search with no arguments" (call 1). I took it to mean that works. It was refused (call 5).
- "annotation no longer exists" (call 54). I first read it as the target being gone, not the price being cleared.
- The README's "its tools do what the CLI does". I took that to be exact, but the price writes are stricter, per the tool text.
- `store_status` "leagues":0 (call 4). I took it to mean no leagues, which contradicts call 8.
- `tabs` "realm: pc, xbox, or sony". I took it that poe2 is unsupported; I did not test it.

5. **What I would retract or not bet on.**
- T3(b)'s 195 (call 17). My regex also admits "all maximum Resistances" and "Stone Golem" lines.
- The ages in T2, which I corrected in T4 once I found `taken_at`. I'd bet on the T4 figures, not the T2 ones. The newest-fetch age (~4.2 days) is my subtraction against that clock.
- My guess that the 1080 and 211 item gaps between `search` and `tabs` are character items. The tool never says (calls 8–10). Impression.
- "The ACQUISITION-PRICE-TEST tab is the person's leftover". Impression.
- The claim that the 5-chaos spoiler's Stash57 links are that tab (call 33). Unconfirmed.

6. **Claims resting on my own knowledge of the game.**
- That a "six-link" item is `links=6` and that "gear" means armour and weapons. The tool defined `links`; the gear reading is mine.
- That life and resistance lines are what the rings query should look at, and that dual-resistance lines count as two. The tool's `total_res` weights them the same way (call 13).
- That fossils, resonators and Delve are what the Delve tab holds. I read that off the item names.
- That `~price` and `~b/o` are the forum's notation. The tool printed them.

7. **What I would change first.** Make `shop_render` and the other large answers controllable, with a limit or counts only, because that cost me a task result (T5's omissions and freshness). Second, put `kind` and a machine-readable next step in every refusal.

8. **What I never found out.**
- Which 4 bodies were refused (call 4).
- Why `agree` is 0.
- What `residue` counts.
- What the 50 conflicts are beyond the first one (call 23).
- What `freshness` says.
- Whether `apply_plan` works with a daemon.
- Whether a real host would have spawned a mock daemon.
- Whether poe2 prices and shops work.

9. **What you should have asked, and what I would add.**
- You should have asked whether I trusted each number. I trust the counts that sum (calls 4, 8, 15, 18) and the price receipts and read-backs (calls 27–39, 53, 55). I trust the ages less.
- Add to answer 1: the cut hid the very part the task asked for, and the door said so only in a header line.
- Add to answer 7: the tool's own text was the best documentation. The README would help most by pointing to it.
