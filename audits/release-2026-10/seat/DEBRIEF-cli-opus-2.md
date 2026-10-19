# Debrief

## 1. Where did the tool stop you, and what did you do instead?

- Call 25: `store status` refused once two accounts were on disk. I added `--account GERWARIC` to every store command after that.
- Call 33: `--count realm` refused. I used `--realm all` with `--count league`, whose values carry the realm (call 34).
- Calls 51, 53: `price list` printed no items. I went to `price show` on the tab (call 52), then found `--expand` (call 54).
- Call 79: the door cut the shop's JSON at its head. I recorded it and did not work around it.
- Calls 89, 90: the mock invents a character for any name, so I could not make an API not-found. I left that untested.
- Call 101: a failed `profile` left a daemon running. I stopped it (call 103).
- Call 106: `dash` refused on the missing daemon. Starting a daemon only to watch dash fail on the missing terminal seemed pointless, so I moved on as the brief says.
- Not the tool: my harness cut the combined output of calls 77 and 78. Rather than open its saved file, I re-ran the shop command (call 79).

## 2. Which outputs did you have to read twice, and what made them hard?

- Call 55, `price status --expand`: "39 decided by hand", "89 by hand" and "87 items inherit a container's row" sit three lines apart. Nothing says that 89 = 39 + the 50 that lose to the game.
- Call 68, the shop page: I had to count links per `location="StashN"` to work out which tab was mine. I reconciled 57 = 29 + 1 + 1 + 25 + 1 by hand.
- Call 34 and the search scope lines: "551 locations fetched (oldest 47d ago, newest 4d ago)" against the "41d ago" on every tab I listed.
- Call 39, the vocabulary count: the 9 "computed" pseudo rows below the templates. I needed `--describe` (call 40) to know what `pseudo.count_res` counted.
- Calls 1, 32, 92, the help texts: each is a paragraph per flag and hard to scan for the one fact needed.
- Calls 69/70 against call 45: what a clear does to the revision. Call 84 (revision 4) settled it.

## 3. What did you want to ask a person, and when?

- T4(b), call 39: whether "life" meant the flat `+# to maximum Life` line or any life line. I assumed the flat line and said so in the report.
- T5, call 52: the existing price-test tab was already at 5 chaos. I wanted to ask which tab they wanted at 5 chaos. I picked an unpriced one (call 59) so the write would be visible and reversible.
- T6, call 68: the tool suggests `acq policy set` to cover my two tabs. I would ask before running it (README Known gaps: it replaces the whole policy). I did not run it.
- T6 cleanup: whether "put the tab back as you found it" meant unset. That is how I found it (call 59), so I cleared it.
- T5, calls 51 and 54: whether the 999-chaos notes are deliberate test notes. I flagged them rather than act.

## 4. Which of the README's or the tool's own words misled you, and what did you take them to mean?

- `price list --help`, "ten or fewer listed per group" (call 49). I took it to mean the items would be shown. They were not (calls 51, 53).
- The clear receipt, "a cleared price names no revision" (calls 69, 70). For a while I took it to mean revisions restart. Call 84 showed revision 4.
- `daemon --help`, "Debugging only — normal use never needs manual lifecycle" (call 92). I took it to mean an agent should not touch the daemon. The README's agent recipe needs `daemon stop`.
- The shop's "oldest 4d ago" (call 68). I first read it as the items' age; I now think it is the listing's (impression, from call 55's basis).
- README, "a login lasts as long as the daemon that holds it". I took it to mean `daemon stop` alone would cost the login. It did (call 20). It did not mislead.
- `acq accounts`, "last login 4h ago" (call 2), for an account the brief says has no login. I took it as the copy's history. Unresolved.

## 5. Which answer would you retract, or not bet on, and why?

- T6, which `StashN` is which tab: I matched Stash267 to '3.13 Random Build Gear' and Stash61 to 'Build Gear 2' by count and x/y. The tool never said so in text. Its JSON for the tab says `index 281` (call 77), not 267. The policy text says n is a rank that leaves out folders and substashes (call 79), which could explain the gap, but I did not verify that. Moderate bet.
- Fault 3, the size of the shop JSON: I could not see what fills its 207 KB, so it may be a fault of the help's wording rather than of the output. Plausible only.
- T2, "the jobs between are probe jobs": inferred from `characters --help` (call 1). The missing job numbers 5, 7, 10 and so on were never shown to me.
- T3, the four refused PoE2 characters "a later or partial fetch landed": impression.
- T2, "policies learned 6 → 1 loses the limiter history the idle exit relies on": an inference from the Knobs table, not shown.

## 6. Which claims in your report rest on your own knowledge of the game rather than on what the tool printed?

- T4(b): that "Dusk Hold" is right to count as 2 resistances. That a Two-Stone ring's dual-res implicit is ordinary is game knowledge. The definition itself came from call 40.
- T2: calling "Starkonja's Head" with a rare frame mock data. That it is a unique in the game is my knowledge (the brief says the mock's items are invented).
- T4(a): calling Tabula Rasa and Shavronne's Wrappings notable worn items. That is my judgement; the tool printed only the names.
- T5: that the 999-currency notes look like test notes. Impression from the tab name 'ACQUISITION-PRICE-TEST' and the values.
- T6: that a forum page's links would break if the tab index moved. The tool said this (call 68); not my knowledge.
- Otherwise none: counts, prices and places are the tool's.

## 7. What would you change first, and why that first?

`price list` printing items it claims to list (calls 51, 53). It is a read, so it looks harmless. But it hides the answer to "is anything wrong with my prices", and a person or agent who trusts it would conclude there is nothing to see. Every other fault I found either announces itself (the extra daemon, the JSON size) or costs one extra command. This one produces a confident wrong picture.

## 8. What did you never find out?

- Which location is "newest 4d ago" (call 34). I take it to be the stash listing, but did not confirm.
- Which items carry the 1 invalid note in Standard and the 3 in Allflame (calls 47, 48).
- What fills the 207 KB of shop JSON, and whether it is one document (call 79).
- Whether `acq-mcp`, the licences and `SHA256SUMS` are in the archive (T1).
- How `dash` behaves with a daemon but no terminal (call 106).
- What an API-side not-found looks like as text and as JSON (calls 89, 90).
- What "unlifted", "withheld" and "intent" mean (calls 75, 34).
- The two Allflame conflicts (call 48), which I never listed.

## 9. What should we have asked you that we did not, and what would you add to any answer above?

- You might have asked: "Did you change the person's store in any way you did not undo?" Yes, once. My T7 test row moved the ring's revision counter to 5 (set at 4, cleared). Status matches its first reading (call 73 before T7). I did not re-read status after call 87, but the item reads none (call 88). The search basis's "intent" rose from 33 to 40 (calls 34, 96), so the store records my writes even though the prices are gone. A person who audits intent history would see them.
- Add to 1: the "Your agent" recipe logs the person out without asking. As an agent, I ran `daemon stop` (call 20) on the brief's instruction. With a real person, it is something I would ask about first, as I said in T0.
- Add to 5: in T3 the per-league tab total (3,347 + 97 = 3,444) reconciles. I did not reconcile the 551 fetched locations against the 405 + 96 + 50 = 551. They do add up, but I only noticed this while writing here.
