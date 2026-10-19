1. Where the tool stopped me
- `price list --limit 1` was refused as an unknown option (call 44). I reran it with `--expand` and no limit, which gave 21 KB (call 45).
- Wrong revision on a cleared row: "annotation no longer exists" (call 77). I wrote a live row (call 79) and retried (call 80).
- `acq dash`: "stdout is not a terminal" (call 94). I moved on without running `dash --json`.
- Oversized JSON (calls 71, 72): the door cut them, not the tool. I could not check whether they end as one valid document, and I did not try `--page N --json`.
- Not logged in (call 92): I accepted it and stopped the daemon the failed call had started (call 96).
- Nothing else stopped me.

2. Outputs read twice
- Call 62, the shop page: the stash numbers (Stash57, 71, 76) are not tied to tab names. I worked out which was which from link counts. That is an inference, not something the tool printed.
- Call 20 against call 25: two different fetch ages for the same store.
- Call 25: the tab-item, character-item and league figures do not form a clean partition of 22721. I could not reconcile them.
- Call 31: the terms block counts each term over the whole store, not over the matches.
- Call 45: 50 near-identical conflict blocks.

3. What I wanted to ask a person
- Before T2: was stopping the login's daemon allowed, since it logs the session out? I decided it was within the task (call 16).
- T4(b): does "resistances" include chaos? I reported both answers (calls 31, 32).
- T5: should I use the test tab that already had a 5-chaos row? I avoided it.
- Whether the door's cut of the oversized JSON was intended (calls 71, 72).

4. Words that misled me
- README: "the same without a keyring" read as if an agent could run `acq auth`. It needs a browser.
- README: "while none is running" implied nothing needs doing. A daemon was already up from the login.
- Help for `price set`: "Prints ... how to put that back". The first-time receipts print no undo (calls 55, 56).
- "(re-read and retry)" read as if retrying would work. A retry with the same revision fails again.
- `access_expires_in_seconds: 59` read as a login about to lapse. It renews on use.
- "1 invalid notes": I took it to mean something was wrong, but I could not find which note.

5. What I would not bet on
- T1: I confirmed only `acq` and `acqd`. I would not say "the archive is complete".
- T3: the 1101 left-over items are my arithmetic, not a tool figure.
- T6: that Stash71 is my tab and Stash76 my item. Impression, from link counts.
- T7: whether the two big JSON answers are valid documents.
- T4(b): 195 against 170 depends on which resistances the person means.

6. Claims resting on my own knowledge of the game
- Mostly none; the counts are the tool's.
- Impression: I called "Fake Item" data mock data from the brief, not from the game.
- I said the ACQUISITION-PRICE-TEST tab is price-test data from its name and its notes (calls 40, 45). That is my reading.

7. What I would change first
The README's longer-daemon recipe: say how to stop the running daemon and that stopping logs you out without a keyring. Every agent doing T2 meets this first (calls 15–18). After that, `kind` on every JSON failure and a smaller shop JSON.

8. What I never found out
- Whether the shop JSON is valid.
- What `--page N --json` returns.
- Which note is the one invalid note.
- What happens when `--account` is omitted with two accounts.
- Whether the real provider refuses to spawn a daemon for a failed job.
- Whether `acq-mcp`, the licences and `SHA256SUMS` are in `bin`.
- Whether the forum accepts a leading space in the spoiler label.

9. What we should have asked, and additions
- Ask: would you hand this tool to a person, and what would you tell them first? My answer: yes for reading, searching and pricing. For fetching, tell them first about the daemon and the login being lost on stop.
- Addition to 1: apart from the first-attempt refusal of my own report append, which was the harness's and not the tool's, nothing the tool did was unrecoverable.
