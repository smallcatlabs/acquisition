<!-- debrief, written after the report; call numbers are rows of calls.csv -->

**1. Where did the tool stop you, and what did you do instead?**

- **T2, the careful round.** By the time I saved the plan file (call 31) nothing was left to plan, so the ceiling and the second hand-over could not be tested (calls 32, 33). I described how I would have done it. I decided not to lower `max_age_seconds` or add Standard just to create work, because that would have gone against what the person asked for.
- **T1.** `characters` has no `--league` (call 13), and the new league has no characters (call 10). I fetched a Standard character instead and said so.
- **T4.** `search --describe` printed 49.8 KB (call 39), and my harness showed me only the first 2 KB. I asked for an entry I knew would be refused (`life`, call 40), which printed every entry name, and then asked for the entries I needed by name (calls 41, 48).
- **T6.** The item undo printed in call 64 was refused once the price had moved to revision 2 (call 71). I ran the command that refusal named instead (call 72).
- **Not the tool:** my harness refused one long report heredoc. I used the brief's fallback, `W/drafts` with `report --from`.

**2. Which outputs did you have to read twice, and what made them hard?**

- **The `search` scope line** (calls 42, 45). It packs the oldest and newest fetch, the never-fetched counts by type and the listing age into two long lines. I re-read it to check the never-fetched sum (2,926 + 15 = 2,941) against `tabs` (call 38).
- **`price status`** (call 54). It is one dense sentence about residue, public versus private, and the game versus hand counts. "492 the game decides" next to "489 priced in game" took a second reading; the difference is the 3 skips.
- **The receipts after replacing a price** (calls 64, 66). I had to work out that "puts the prior back" means 2 divine, not unset.
- **`daemon status`** (call 88). "connections: 1" while nothing else was running turned out to be my own command, I think. That is an impression.

**3. What did you want to ask a person, and when?**

- **T2:** whether to add Standard and Hardcore to the policy, since the README warns that a policy naming only the new league drops Standard. Here there was no prior policy, so nothing was dropped. I asked in the report instead.
- **T4b:** whether "life" means a maximum-Life line or the trade site's total life, which counts Strength. I gave both answers (calls 47, 49).
- **T5:** which tab and which item to price. I chose a private Delirium tab and a ring in the public MARKET tab. A person might not have wanted a private tab's items on the shop page.
- **T8:** whether stopping the daemon was acceptable, given that it logs `seat` out (call 90). The brief told me to stop it.

**4. Which of the README's or the tool's own words misled you, and what did you take them to mean?**

- **The README's "Log in": "`acq` refuses to start a daemon against GGG without [a login]".** I took it to mean a job command with no login would refuse before spawning anything. It spawned a daemon (call 94). That may be because this world runs on the mock, but the README doesn't say so.
- **The README's "A new league": "the first plan is the listings".** I expected two rounds. The first plan went straight to the tabs (call 27).
- **`refresh --help`: "a first sync takes two rounds"** because of substashes. Here it took one (call 28).
- **The brief's "with the prices they have set".** I expected hand prices. There were none, only in-game ones (call 54). That is the brief's wording, not the tool's.
- **`kind: "not_logged_in"`** for an unknown account (call 83). I briefly read it as "log in", when the problem was a wrong name.
- **"re-read and retry"** (call 24). Read alone, it seemed to invite overwriting the policy.

**5. Which answer would you retract, or not bet on, and why?**

- **Fault 8, the store size changing between call 5 (54.4 MB) and call 34 (56.3 MB).** Something may simply have grown during reads, such as a cache or journal file. That is an impression, and it may not be a fault at all.
- **Calling Rehearsal "the new one" in T1** (call 8). It is an inference from the category column and the mock's description (call 77, "the mock's challenge league, empty on its first morning"). The tool never labels a league as new.
- **T4b's count of 195.** It depends on what "life" means.
- **In T6, "the private tab's items reach the trade site via the shop".** That rests on the README's "What it does not do", not on anything I observed.

**6. Which claims in your report rest on your own knowledge of the game rather than on what the tool printed?**

- That the 8,297 items with no rarity are "gems, currency, cards…". `describe` says a gem, a currency stack and a card have no rarity (call 39's visible head), but the "…" is mine.
- Calling six-linked bows and staves "gear". The tool just counted `links=6`.
- That the person should refresh before posting, because 50-day-old positions are risky. The tool hints at this (call 70); the advice is mine.
- That pasting goes into the Standard shop subforum. That comes from the README, not the tool.
- That "exal" is meant as Exalted Orb. I did not claim it, but I read the note that way.

**7. What would you change first, and why that first?**

I would change the offer `policy show` prints when no policy exists (call 22). It is the first thing a person sees on a new league's day. It is the one place the tool hands over a ready-to-run command that can overwrite, and it contradicts the README's own `--if-revision 0` rule. Fixing it is cheap, and every agent that copies `next` lines would benefit.

**8. What did you never find out?**

- Whether `--max-requests` refuses, and what a spent plan file says.
- What `unlifted_items` and `withheld_responses` mean (call 78).
- Why the store grew (calls 5, 34).
- How `pilatesinstructor` has both a refused body and 30 stored items (calls 36, 43).
- Whether the daemon in call 88 was the one the door started at login. "Up 349s" seemed short; that is an impression.
- What `web` and `dash` do with no terminal; I never tried them.

**9. Did you change the store in any way you did not undo?**

Yes, partly.

- **`seat`, kept on purpose:** the policy at revision 1 (call 23), which is what the person asked for. Its store also gained the listings and the 7 tab fetches from T1/T2, plus the failed `rh-nope` fetches (calls 84, 85), which are now in the send journal.
- **`GERWARIC#7694`:** both prices are cleared and read back as nothing applies (calls 74–76). But the rows' revision counters moved on: the item was cleared at revision 3 (call 86), and per call 72 "the row's count continues past the clear". The account's `basis.intent` moved from 0 to 5 or more (calls 52, 79). These cannot be undone through the tool.
- **The daemon is stopped**, and `seat` is logged out (call 90).

**10. What should we have asked you that we did not, and what would you add to any answer above?**

You might have asked whether the age warnings changed what I would tell the person. They did: every T4/T5/T6 answer rests on facts that are 50–59 days old. A shop page built on them (call 70) could link to items that have moved. The tool says so, but softly ("a refresh re-reads them").

To answer 1, I would add that I never re-logged in after T8. Any later job command would refuse, and spawn another idle daemon (call 94).
