1. Where the tool stopped me. T7: Loreweave not found by name or base (calls 20, 21); every search says 2940 locations were never fetched, UniqueStash 437 of 521. `refresh --plan` (call 22) would not fetch the unique tabs, and there is no login or daemon, so I stopped and reported it as not found. T8 boots: the search had no match (call 8), so I offered a near miss (call 28) and said so. Output cuts: call 26 (`--describe trade`, 33 KB) was saved to a file by my harness; I did not open it.

2. Read twice. The term tables ("counted over all live pc items in all leagues": 30 beside a total of 27, calls 14-15) and the "ambiguous" notices (calls 8, 12, 13, 16, 17), which don't say whether this answer changes. Also `price list` (call 29), grouped by container with item ids, and I matched tab ids to T4 rows by eye.

3. What I wanted to ask the person. At T4 (call 15): "best" by explicit rarity (the site's stat) or by total? At T7: should I fetch the unique tabs (which needs login and spends rate limit)? At T8: boots, strict match or the nearest miss? And is "buyer could see" meant as public and priced tab?

4. Misleading words. "exact" (call 5) read as "the same result as the site"; it only means every clause with a stash meaning ran. "Remove-only" I took as not public, from call 29's "(not public)", not from the tour. "ambiguous ... adds 0, drops 0" I took to mean no effect here. "all leagues" in the term header, see 2. Impression: "2940 locations never fetched" first read as a count of tabs, it is locations including characters.

5. Would not bet on: T7 (absence, not proof); the T8 listing claim (5 T4 items public at 4554 chaos), matched by eye and unchecked for T1/T5/T6; T4 ordering (call 15, total not explicit); T5 beyond counts (no rows, 81 tabs cut). T2's 0 and T3's single match I would bet on (calls 9-12).

6. Change first: tell the user which unfetched tabs could hold an absent item and what command would fetch them. It is the only fault that left a task undone, and "0 matches" is otherwise indistinguishable from "not fetched".

7. Never found out: whether the Loreweave exists in the store at all; what the site would return; the 50 price conflicts (call 25); the 1395 items with unseen price text; what `--json`, `shop render` and `store refused` give; the contents of the 4 refused bodies (call 4).

8. Should have asked: how much I trusted each answer, and which task I would redo with more time. Addition to 1: using counts instead of rows (T5, T6) was my choice to save output; a person wanting positions would need extra commands.
