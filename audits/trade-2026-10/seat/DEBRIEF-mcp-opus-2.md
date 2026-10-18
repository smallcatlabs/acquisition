## 1. Where did the tool stop you, and what did you do instead?

It stopped me in two places. Each time I went around it instead of through it.
- **`tabs` for Standard** (call 10): the answer was 745,672 bytes and the door cut it to its head. I wanted to know which tabs and pages are among the 2,940 `never_fetched`. Instead I counted by `tab.type` with search (calls 9, 39). That tells me what *was* read, never what was not. I left the question open and said so in T2 and T6.
- **Site visibility** (T1): no field says whether a tab is public. The one hint, `price.why` "…not where the index can see it (C81)" (call 18), I could not read. Back in T1 I guessed that the site would show none. In T4 I compared a priced and an unpriced Andvarius (calls 30–32) and inferred the rule: a "~price" tab name counts only outside Remove-only tabs. That is still an inference. The tool never says "Remove-only".

My harness also cut call 11's display at 30 KB (the tool answered in full). I repeated the query as counts and as text (calls 12–14). That was 3 calls for what should have been 1.

## 2. Which outputs did you have to read twice, and what made them hard?

- **Call 12, and every search answer like it.** The one fact I wanted, `total.matched: 22`, sits near the end, after `query.trade.clauses` and the per-term blocks. In every term block each count carries a full request, so the important numbers are buried under repeated boilerplate. I learned to scroll for `"total"`.
- **Call 37**: `"sum":{"lacking":83,"value":0}`. On first read it said "you have 0 maps". On second read it said no item carries a stack size. I settled on "each item is one map", but that is my reading (impression).
- **Call 29 against call 28.** The counts named the tab '~price 4554 chaos' (5 items). The rows said `name: null` with a `parent`. For a moment I thought the rows and the counts disagreed.
- **Call 22**: the `sort.shows` block gave one resistance line for `pseudo.total_ele_res`. I could not tell whether it was the whole total or only part of it.

## 3. What did you want to ask a person, and when?

- **In T1 and T4: "Are your Remove-only tabs public, and do you sell from them?"** The person's question was "would the site show me the same?", and the store cannot answer it (calls 15, 18, 31).
- **In T7: "What do you value: armour or resistance? And is a ring on a character fair game?"** I chose Gale Spark (evasion, 140% res) over armour boots (call 42) on my own judgment.
- **In T0, and again at T6: "Your data is from 2026-09-08 for most tabs. Should it be refreshed first?"** A refresh needs their login and daemon (call 1). I would not start one without asking, and in this world it could not have run anyway.
- **In T4: "Is 4554 chaos a real price, or a 'not for sale' marker?"** (call 32). That is impression; I did not raise it in the report.

## 4. Which of the tool's own words misled you, and what did you take them to mean?

- **"the price text in game is not where the index can see it (C81)"** (calls 18, 31). I first read this as a gap in the index: the tool fails to read tab names. Calls 30–32 showed it *does* read them, except in Remove-only tabs. The words point to a defect in the tool when the real reason is the tab's state in the game.
- **"the site shows public listings"** (call 1, `search.query`). I took it to mean the tool would help me tell which items are public. It cannot, and nothing says so.
- **`never_fetched` … "no answer says what they hold"** (call 3, and `search`'s text). This is honest, but I took it to mean the gap could be inspected somewhere. `tabs` was the only candidate, and it was too big to read (call 10).
- **"Every count that is not zero holds the request that returns exactly its members"** (`search` text). This is true. But I took the requests to be the way on, and resending them brings back the whole heavy answer. A request that names a tab id (call 25) worked well. The terms' own requests I never needed.
- **`fidelity: ambiguous`** (calls 20, 24). It led me to expect the answer might be wrong. In every case the ambiguity cost nothing for this store (calls 34, 40), and the label does not hint at how much is at stake.

## 5. What would you change first, and why that first?

I would answer the person's question about the site directly. That means a field such as `tab.public` / `tab.remove_only` per tab that search can count by, and a plain `price.why` ("in a Remove-only tab, which the site never lists"). It comes first because the person framed every task around it: "I'll trust it more if I know where it might not" (TASKS.md). In T1 the tool left me guessing, and the guess only became an inference two tasks later (calls 18, 30–32). Every other fault on my list costs calls or bytes. This one costs the person a wrong answer.

## 6. What did you never find out?

- Which of the 2,940 never-fetched containers are map, ring or boot tabs (calls 3, 10). So T2's zero and T6's 83 are unsized.
- Whether the Remove-only tabs really cannot be listed on the site. The rule is my inference from the price behaviour (calls 30–32) plus game knowledge; the tool never says it.
- What `item_count` versus `fetched_items` on a character means (call 8): items not yet fetched, or socketed gems counted apart?
- What `folders: 17`, the C-numbers, `basis.intent`/`derivation`, and `query.trade.forms` mean (calls 1, 3).
- Whether the site's pseudo life and resistance totals equal this build's. The tool cites "the C++ app's pseudomod tables" (call 4), and I had nothing to compare against.
- Whether the poe2 realm holds anything relevant. I never searched it.

## 7. What should we have asked you that we did not, and what would you add to any answer above?

**What you should have asked:** "Which of your answers to the person would you retract or soften now?" Mine:
- **T1's** "the site would most likely show none" was written before I understood the Remove-only rule, and it rested on less than it seemed. T4's evidence later supported it (calls 30–32), and I should have gone back and added that to T1.
- **T6's** first draft said the maps were unpriced before I had checked. I corrected it after call 41, but the report still carries both versions.
- **T7's** "12-wide" tab size was my own game knowledge, which I corrected in the report.

**To add to 1:** the heredoc form for `call` was refused by my harness, not the tool. Inline JSON with escaped quotes worked but is error-prone. Some of the cost of a long query text is in the shell, not in the tool.

**To add to 5:** second, I would add a lean answer mode. About 80% of the bytes I read were repeated `basis`, `scope` and per-term requests (calls 12–14, 19).

*Correction to 7:* "about 80% of the bytes" is an impression, not a measurement. The door printed only the totals (e.g. 16,742 bytes for call 12), and I did not measure the parts.
