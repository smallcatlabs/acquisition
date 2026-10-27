# Debrief

## 1. Where did the tool stop you, and what did you do instead?
- `search {}` was refused with `realm_needed` (call 6). I used `scope.realm: "all"` instead.
- `realm` is not a count key (call 7). I counted by `league` only and read the realm off each bucket (call 8).
- Listing which 4 bodies were refused: the tool pointed me to `acq store refused` (call 5), and I had no MCP way to do that. I reported the count only.
- Naming the invalid notes: `price_status` only counts them (calls 20, 21). I used searches (calls 24–26). That named the Standard note. For Allflame I could only list candidates.
- `submit_job` was refused with `no_daemon` (call 43). I reported the command it gave and fetched nothing.
- Outside the tool, the harness refused a heredoc on `call`. I wrote the arguments to `W/drafts/` and used `--from` (calls 15, 16).

## 2. Which outputs did you have to read twice, and what made them hard?
- **`search`'s `scope` block** (call 8 and every search after it). `never_fetched_by_type` lists `MapStash tabs 2582, fetched 93`, inside a key named "never fetched". I couldn't tell whether 2,582 counted unfetched tabs or all tabs. And `fetched 551 + never_fetched 2941` does not equal 3,444 tabs + 65 characters. I settled it with the folders (17) but never confirmed that reading.
- **`shop_render`** (call 35): 34 KB in which the page `text` comes last, after 50 `left_out` entries.
- **`price_show` of a tab** (call 27): 28 KB of repeated per-item blocks. I had to scan it to find the one item with a note.
- **`characters`** (call 11): a bare array of 65 rows with no counts. See answer 5: my counts from it were wrong.

## 3. What did you want to ask a person, and when?
- **T3(b):** whether "at least two resistances" meant two elements (my reading), two resistance lines, or counting a dual-resistance line once.
- **T4:** which tab and which item to price. I picked "Items 1" and Apocalypse Skin myself. Also whether the `~price 4554 chaos` tab name was meant as a price per item.
- **T6:** whether to submit a job at all. I sent `whoami` only (call 43), judging it harmless in the mock. Against a real account I would have asked first.
- **T1:** where their host reads its configuration from.

## 4. Which of the README's or the tool's own words misled you, and what did you take them to mean?
- "Begin with search and no arguments" (call 1). I took it literally, and it was refused (call 6).
- "The job tools talk to the daemon (… list_jobs, job_status, … daemon_status)" (call 1). I took it that these would fail without a daemon. They answer from disk (calls 40, 42, 45).
- `kind: "not_logged_in"` (call 46). For a moment I read it as a login problem. It was my typo.
- The brief's "with the prices they have set" against the tool's `rows.total: 0` (call 20). I first expected hand rows. The tool showed the prices are in-game notes and tab names.
- `persisted: false` (call 2). I took it to mean "no keyring login". That is an impression, and my T6 advice rests on it.

## 5. Which answer would you retract, or not bet on, and why?
- **Retract (T2, character counts).** I wrote "15 never fetched … e.g. all ten Solo Self-Found characters" and "14 fetched with an empty body". I counted these by eye from call 11, not from a printed count. Recounting call 11's rows:
  - 15 have `fetched_at: null`. That number is consistent with call 8's `character … fetched 50` of 65.
  - Solo Self-Found has **nine** characters, not ten.
  - Rows with `fetched_items: 0` number **ten**, not 14: DLVR, VerySillyPerson, Overdramatic, GeorgiaOKeeffe, Jaozun, RotisseriePotato, gerwaric, CorrectHorseBattery, TheLateBloomer, mUrDEroUsCUsHionS.

  The report broke its own rule here, giving a count the tool never printed.
- **Retract "most" (T3a).** I wrote "Most [six-links] are worn or carried by characters". Only the first 30 of 187 were shown (call 14), and 26 of those were on characters. I never counted the rest. A `view.counts` on `container` would have settled it.
- **Not bet on:**
  - T6's fallback, "run `acq auth` and log in as GERWARIC". It is an inference from the README and from `persisted: false`.
  - T5's reading of `position_before_listing`. The tool never explains it.
  - The Allflame invalid-note candidates (call 26).
  - That `Stash64` opens "Items 1" on the website.
- The "1,080 on characters" in T2 is my own arithmetic (21,311 − 20,231 from calls 8 and 9), not a printed number.

## 6. Which claims in your report rest on your own knowledge of the game rather than on what the tool printed?
- That minion, aura and cluster-jewel resistance lines are not the player's resistances (T3b), and so my choice of pattern.
- That items with no rarity are "currency, gems, cards and the like" (T3c).
- That `~skip` or a private tab is how to say "don't sell", and that a per-item tab-name price of 4,554 chaos looks like a mistake (T4).
- That `~c/o 5 chaos`, `~price 5`, `~price 5 exal` and `~price 100 chaos A` are the likely invalid notes (T4).
- That posting goes in the Standard shop subforum. That one is the README's, not the tool's.
- The interpretation of "two resistances" itself.

## 7. What would you change first, and why that first?
Make every refusal and `describe` entry speak MCP: tool names and argument paths instead of `acq …` and `--flags` (calls 5, 6, 13, 35, 47). It is the fault an agent hits from its very first call (call 6), and every recovery runs through it. It also hides a real gap: refused bodies have no MCP tool at all.

## 8. What did you never find out?
- Which 4 bodies were refused (call 5).
- Which 3 Allflame notes are invalid (call 21).
- What `position_before_listing` means (call 35).
- Where Claude Code or Claude Desktop reads the configuration from.
- Whether the README's bare config would reach this store without the door's environment.
- Whether a create races safely.
- Whether `acq auth check` works with `persisted: false`.
- What `basis.intent` counts. It went from 0 to 3 after my writes (calls 18, 34).
- How the 187 six-links divide between characters and tabs.
- Whether the 1,395 residue includes tab names on private tabs. My 119 covers item notes only (calls 20, 24).

## 9. Did you change the store in any way you did not undo?
Every price row I wrote is cleared, and `price_status` matches the start (call 51 = call 20). What I could not undo:
- **Revision history:** the item's revision counter moved on. A clear "continues" the revision, so the next row would start above 4 (calls 48, 50; rule in call 19). The tool shows no tombstones in `rows`, but I can't say whether any remain on disk.
- **Possible derived state:** `basis.intent` changed from 0 to 3 (call 34), which suggests some derived state recorded my writes.

## 10. What should we have asked you that we did not, and what would you add to any answer above?
- **Missing question:** "Which numbers in the report did you derive yourself?" Answer 5 shows I miscounted from a long array. The tool's lack of counts in `characters` (call 11) invited that.
- **To add:** the most useful thing the tool did was `terms[].resolved` in `search` (call 15). It let me catch my own wrong query, and that is worth keeping and copying elsewhere, for example in a `price_list` filter of invalid notes.
