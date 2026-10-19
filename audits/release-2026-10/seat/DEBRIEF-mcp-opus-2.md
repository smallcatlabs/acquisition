# Debrief

## 1. Where did the tool stop you, and what did you do instead?
- Call 6: a plain `search` was refused (two realms). I used `scope.realm: "all"`, which the refusal's text named but its `offers` did not (call 7).
- Call 37: `shop_render` answered 160,473 bytes and the door cut it to 3,000 characters. I asked for `page: 1` (call 38), which gave the page but not `counts`, `left_out` or `freshness`. I then reported the left-out items as counts from the part of `policy` I could see, not as a list of items. No way round it that I found.
- Calls 21 and 22: `invalid_notes` was counted (1 and 3). I tried to find those notes through a search (call 27) and failed. I reported them as unfound.
- Call 3: 4 refused bodies, with no MCP tool to list them. Reported as unfound.
- Call 46: `refresh_plan` for Allflame came back as a protocol error. I read `sync_policy` (call 48) and planned Standard instead (call 49).
- Calls 45 and 47: no daemon. I did not try to start one; that is outside the door and the brief.
- `apply_plan` was denied by my harness, not by the tool, so it has no call number. I did not route around it.

## 2. Which outputs did you have to read twice, and what made them hard?
- Call 1 (43 KB of tool texts): long sentences full of clauses and em-dashes. The `price_set` text packs creating, replacing, undoing and an unreadable prior value into one sentence. I reread it before calls 32 and 34.
- Call 7: `never_fetched_by_type` lists `tabs` and `fetched`, but not never-fetched. I had to add numbers across calls 7, 8 and 9 to find what the fields meant.
- Call 21: the `price_status` counts. `by_relation`, `by_effective` and `by_game_statement` overlap, and I had to add them up (game 492 = game_only 442 + conflict 50) to see that in a conflict the game side wins. `residue` I never understood.
- Call 31: an item with no row of its own still showed `manual.revision: 1`. I read it twice to see that `from` was the tab and `inherited` was true.
- Call 38: the page text. I counted the `Stash216` links by hand (29), which is error-prone (see 5).

## 3. What did you want to ask a person, and when?
- Before call 17: what "life" means to them (a maximum-Life line, or the trade site's total life, which also counts half of Strength), and whether "two resistances" means two resistance mods or two resistance types. I chose and said so.
- Before call 30: which tab and which item to price. The brief let me pick, and I picked a private build tab. A real person would want to name these, and might not want a private tab on a shop page.
- Before `apply_plan` and `set_sync_policy`: these spend rate limit or replace the whole policy. I would have asked and did not run them.
- At T4 "is anything wrong": whether the 999-chaos notes on `ACQUISITION-PRICE-TEST` are deliberate test values. The tab name suggests they are (impression).

## 4. Which of the README's or the tool's own words misled you, and what did you take them to mean?
- The instructions' "a search over several realms is refused": I took it to mean `realm: all` was not allowed. It was allowed (call 7).
- The README's "leave it out for your own account" (about `ACQ_PROVIDER`): I took it to mean my host config decided the provider. In this world it did not; the server still said `provider: mock` (call 4).
- `never_fetched_by_type` (call 7): at first I took `tabs: 2582` to be the never-fetched Map tabs.
- `price_show`'s `manual.revision` on an inherited row (call 31): I nearly took it as the item's own revision.
- "A clear takes a revision too" (`price_clear` text): I expected the clear's revision in its receipt (call 39). It wasn't there.
- The `price_list` text says that under `limit` the `containers` are "theirs alone". With `limit: 0` (call 24) I expected to still see the containers, and got none.

## 5. Which answer would you retract, or not bet on, and why?
- T5: "the `~price 5 chaos` spoiler holds 29 `Stash216` links, plus the ring makes 30, and 51 − 30 = 21 = the 21 socketed blocks, so the socketed items are gems in that tab". The 29 is my hand count. The link to the 21 is inference from arithmetic; the tool never said which items were socketed. I would not bet on it.
- T5: "`Stash57` is `ACQUISITION-PRICE-TEST`". I inferred it from that tab's `index: 56` (call 23) and the rank rule in `policy`. The tool did not say so, and folders before it would shift the rank. I would not bet on it. Nor on "the other links are your earlier hand prices on Stash57".
- T5: "the 50 conflicting items are among the 489 `game_lists`". Inference from the counts; I never saw the `left_out` list.
- T1: "the door sets the provider and store". Inference from call 4, not something I read.
- T6/Fault 1: "typed as given, it would start a `ggg` daemon for another world". Reasoned from the README, never tested.
- T2: "most of what I know of your Standard stash is about six weeks old". This rests on the ten tabs `tabs` listed (call 8), not on every tab.
- T2: the dates beyond the six I converted with `date` (e.g. 7–8 Sep, 11 Sep) are my own mental arithmetic.

## 6. Which claims in your report rest on your own knowledge of the game rather than on what the tool printed?
- What "six linked sockets" means as a question (a six-socket link group), and treating "gear" as all items without filtering to gear classes. 187 includes anything with a six-link, which in practice is gear.
- That a socketed item in a tab means gems in gear, and that the 8,297 items with no rarity are ordinary for gems, currency and cards (the frame counts in call 19 support it).
- That "offer" in `~price 999 offer` is a real currency (an Offering to the Goddess); I did not flag it as a misread note. The tool's `price.currency=offer-gift` etc. (call 27) supports this, but I did not check `offer` with `reference_currency`.
- "Paste it into a thread of your own in the shop subforum": partly the README, but "a thread of your own" is mine.
- That a person usually means maximum Life by "life", in T3(b).

## 7. What would you change first, and why that first?
The `no_daemon` refusal's `next` (calls 45, 47). It is the only text the tool hands to the person to type, at the moment they are trying to get the agent working. If it names the wrong provider or store, the person ends up with a daemon the server will not use. Every other fault I found costs an agent extra calls; this one sends a person to the wrong place. Second: make every refusal an `isError` result (calls 46, 50).

## 8. What did you never find out?
- Which notes are invalid (calls 21, 22) and which 4 bodies were refused (call 3).
- What `persisted` means in `accounts` (call 2); what `unlifted_items` and `withheld_*` mean (call 3); what `residue: 1395` means (call 21).
- Which fetch `newest_fetch` (15 Oct) is (call 7).
- Why an Allflame tab has `item_count` 40 but `fetched_items` 19 (call 9).
- The rest of `shop_render`'s `policy` (after `tab_unlist…`), and its `freshness` (call 37).
- Whether undoing a clear needs an `if_revision`, and which one.
- What `apply_plan` says with no daemon.
- Whether a host config without `ACQ_PROVIDER` would, outside this world, have reached this store.

## 9. What should we have asked you that we did not, and what would you add to any answer above?
- You could have asked whether the cleanup was complete beyond the counts. The tab row and the item row were cleared, and `price_status` matches (calls 42, 56). But each clear left a revision behind (the next create came at revision 4, call 53), so the store is not byte-for-byte as I found it. That is what the tool intends, but the person might want to know.
- You could have asked whether my T4 writes ever appeared where a buyer would see them. They did not: they were intent rows only, and they reached only a rendered page that I never pasted.
- Added to 1: my harness also saved two over-long outputs (calls 1 and 28) to its own files outside W, and I read call 1's. The text is the same as the door printed, but it was not read "as printed" in the terminal.
- Added to 5: the report's T3(a) count of 187 is for pc only. PoE2 items were not searched for six-links.
