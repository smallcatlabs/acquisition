# Debrief

## 1. Where did the tool stop you, and what did you do instead?
- **Bare `search --describe`** (call 41) was 47.5 KB, and my harness cut it to about 2 KB. I gave `--describe` a name that doesn't exist (call 42) to get the list of entry names, then asked for the entries I needed (call 43).
- **`shop render --expand`** (call 69, 3.5 MB) and **`shop render --json`** (call 81, 7.4 MB) were cut by the door. Instead I used the default text render (call 68) and `--page 1 --json` (call 82). I never saw the policy table from `tab_unlisted` on, and never checked that call 81 is valid JSON.
- **The undo printed by call 64** was refused, "several accounts are known" (call 70). I added `--account` and used `price clear` instead (calls 72, 73).
- **`price list --limit`** was refused (call 55). I ran the whole list (call 56).
- **`--count realm`** was refused (call 34). I counted by `league`, which shows the realm too (call 35).
- **`dash`** refused with no daemon running (call 106). I recorded it and moved on.
- **The "newest 4d ago" location** (call 35). Listing every tab with `tabs --expand` would have gone past the door's 60 KB limit, so I stopped there.
- **The report heredoc for sections 3–7 was refused by my harness.** I wrote the draft `W/drafts/report-3-7.md` and appended it with `report --from`.

## 2. Which outputs did you have to read twice, and what made them hard?
- **The rows of `price list`** (calls 54, 56): the columns have no headers. I worked out that the third column is the hand price and the fourth the game price only from the `note … reads …` lines under each row.
- **The shop render's `positions:` line and the `Stash57`/`Stash267` names** (call 68). It only made sense once the policy table (call 69) said `n` is "the tab's rank among the tabs the website lists". I compared the ring's x 11, y 7 with call 46 to be sure.
- **The search `scope` block** (call 35): "551 locations fetched (oldest 47d ago, newest 4d ago)" against tabs fetched 41d ago (call 6). I'm still not sure what the 4d refers to.
- **`terms` counts: matched / failed / lacked / undecided** (call 39). I took "lacked" to mean the item has no such field. That's an impression; I never checked it.
- **`store status`** (call 3): "granted: 8 item-granted skill(s) left in place … never rows". I read it twice and still don't know what it means.

## 3. What did you want to ask a person, and when?
- **At T2:** whether "keep a daemon up for the agent" allowed me to stop the daemon that was already running, knowing that would log them out (call 22). The brief's recipe implied yes, so I went ahead.
- **At T5:** which tab and which item they wanted priced. I chose Delve and Rapture Eye myself.
- **At T4(b):** whether "life" meant the `+# to maximum Life` line or the trade site's total life, which adds half of Strength, and whether "+% to all Elemental Resistances" counts as one resistance or three.
- **At T5:** whether the 50 conflicts in ACQUISITION-PRICE-TEST are a deliberate test tab. Its name suggests so, and that changes "is anything wrong".

## 4. Which of the README's or the tool's own words misled you, and what did you take them to mean?
- **README:** "start it with a longer idle time while none is running". I took it to mean I had to make sure none was running. It never says how. `daemon stop` is filed under "Debugging only — normal use never needs manual lifecycle" (call 21), which made me hesitate.
- **README:** "acq refuses to start a daemon against GGG without one". I took it to mean a job verb with no login wouldn't spawn a daemon. On the mock, one did (call 101).
- **`price set --help`:** "Prints what it replaced, how to put that back" (call 58). I expected a runnable undo command on every write. Calls 61 and 62 printed none, and call 64's can't be run as printed (call 70).
- **The `--json` help:** "a failure included (`{"error", "kind"}`)" (call 1). I expected every error to have a `kind`. Calls 88 and 90 have none.
- **`dash`:** "daemon is not running (it spawns on demand for job commands)" (call 106). I read it as hinting that `dash` would start one. It didn't.

## 5. Which answer would you retract, or not bet on, and why?
- **Fault 3's reason**, "because both carry every item that isn't priced": for `--json` (call 81) I inferred this from the size and the help for `--expand`. I only saw the first 3,000 characters. Not proven.
- **Fault 1:** "an agent pasting receipts hits it every time" overstates it. It only happens when more than one account is known. I'd soften it.
- **Fault 2:** that `profile` started the daemon (call 101). The daemon's 1800 s idle exit matches the setting the door passes, so the door's environment was on that command. I'm fairly confident the spawn came from `acq profile`, but I didn't watch the process start.
- **T5, "nothing lists them for sale"** about the 1,395 items with unread price text: that's my inference from "shown, not a side" (call 49), not something the tool printed.
- **T3/section 4, "newest 4d" is the stash listing:** a guess from call 50.
- **T6:** that the forum links will hit the right slots. I can't check the tab ranks from here.

## 6. Which claims in your report rest on your own knowledge of the game rather than on what the tool printed?
- **T4(a):** that "six linked sockets" means `links=6`. The tool printed what `links` means (call 38); mapping the person's words onto it was mine.
- **T4(b):** that "life" means the `+# to maximum Life` line, and that "resistances" includes chaos. The tool defines `count_res` that way (call 43), but choosing it was mine.
- **T2:** that Starkonja's Head is a unique, so `frameType 3` with `frameTypeId "rare"` is inconsistent (call 18). That's game knowledge.
- **T4:** calling Tabula Rasa and Shavronne's Wrappings examples worth naming. The names came from call 39; why they're notable is mine.
- **T6:** "paste into your shop thread … preview" is the tool's (call 68). That the forum needs the shop thread to exist first is my knowledge, and I didn't state it.
- **T0:** that `refresh --all` against GGG would cost "an hour of their budget" mixes the README's rate figures with my own judgement.

## 7. What would you change first, and why that first?
I'd make every printed command (undo receipts and `next:` lines) complete: carry `--account` and the revision guard, and for a set on an unpriced target, print the `price clear … --if-revision 1` that undoes it (calls 61, 62, 64, 70, 72). These receipts are how a person or agent recovers from a mistake, which is the tool's own safety story. They break silently as soon as a second login happens, which the README's own agent recipe makes likely.

## 8. What did you never find out?
- Which item has the "1 invalid notes", and how to list the 1,395 items whose price text the index can't see (call 49).
- What location is "newest 4d ago" (call 35).
- Whether call 81 is one valid JSON document.
- The rest of the shop's policy table (call 69).
- Whether `acq-mcp`, the licences and `SHA256SUMS` are in `W/bin`.
- What "granted", "unlifted", "withheld" and "intent" mean (calls 3, 78, 35).
- What the hidden jobs between the visible job ids are (calls 14–19).
- Why the ledger's "written N s ago" ages disagree (calls 94, 97, 98).

## 9. What should we have asked you that we did not, and what would you add to any answer above?
- **"What did the world's setup do that a person's wouldn't?"** The door clearly sets `ACQ_NO_KEYRING`, `ACQ_LOG_DIR`, the store directory and a 1800 s idle exit on every command (calls 12, 20, 101), but nothing the tool prints says these knobs are set. A person can't ask the tool which knobs are in effect, and that matters when a daemon "unexpectedly" has a 1800 s idle exit (call 101).
- **Added to 5:** my T2 statement that MuleQuadTab is a Witch, level 12, is only what the list said (call 16). The fetched body says Scion, 97 (call 18). I wouldn't tell the person either one as fact.
