## Verdict: BROKEN on compatibility, NARROWED to wording and tools

1. **Compatibility is not where the weight is.** By keyword count, about 0.3% of production lines (349 of about 119.5k) and about 3% of tests (58 of 1,723) serve a state another build wrote. The project already made the pre-user cut on 2026-11-20 (`c140f64f`): the release is the floor, and 816 lines went.
2. **The weight is in words and in the project's own tools.** Goldens are 1.05 MB, and 76% of those bytes are the web pages' HTML. In the last 300 commits, 50 of the 102 that touch `crates/` regenerate a golden. In a sample of 62 review findings, 29% are about the method's own tooling.
3. **The deciding number:** 16% (10 of 62) of sampled P1/P2 findings protect compatibility. Meanwhile the irreplaceable facts history has no backup at all, and the intent backup has no production caller.

## Evidence

### 1. Compatibility and migration in the code
- `git grep -n -i -E 'migrat|legacy|compat|…' -- crates` finds 420 lines, mostly comments and goldens. By file, the version-state identifiers sit in the store crate: `SchemaTooNew`, `SchemaTooOld`, floors and `user_version`.
  - Grep of those identifiers: 349 production lines and 220 test lines.
  - Test names: 58 of 1,723 `#[test]` functions name a version, a floor, "newer", "unreadable" or "stamp".
- **The pre-user cut already happened.** Commit `c140f64f` (#293 cut 1) set "a stored shape older than the release is refused, never migrated". It deleted the facts ladder from v1 to v7, sync policy v1 and v2, and `acq store import` (299 lines added, 816 removed).
  - What stays, each for a shape the released build actually wrote: facts 8 to 9 (`crates/acquisition-store/src/lib.rs:250-261`, `:1470-1496`, about 30 lines), the jobs file's 0/1 to 2 step (alpha.1 wrote no stamp), and `HoldsAccountFacts`.
- **The protocol has no compatibility machinery.** C10 and C84 make the handshake single-version: a mismatched daemon is replaced, never negotiated with.
  - The only "every version parses" plane is `hello` and `daemon_stop` (C85). That pays every day: each rebuild of main leaves an older daemon to be told to stop.
- **Where it paid (inference, not checked):** the owner's facts file holds history going back to August–September and is now at v9. So it was upgraded in place, not rebuilt, through development versions. The 8 to 9 step postdates the release tag (`7ca3ad63` is 2026-10-20 and is not in `v1.0.0-alpha.1`). I did not read the store to confirm.
- **Where it has not paid:** `audits/whole-review-2026-11/findings/stored-row.md` reports 0 unreadable intent rows of 21 and 0 withheld responses of 4,946. No schema stamp has ever been refused.
  - `RUN-LEDGER.md` and `HISTORY.md` record no migration event: grep for `migrat|schema|newer build` finds only one "no migration line" note.

### 2. Rulings
There are 200 entries (`python3 -I tools/bothy.py list`). About 10 promise stability to a consumer (judged): C53 ("JSON changes are additive", `decisions/frontends.md:11`), C149, C47 (stable kind), C66, C65, C121, C124, C85, C194 and C165's closed list.

The most expensive by citations in test files (my regex count of `C<id>` in `tests/`, `unit/` and goldens):

| Ruling | Test citations |
| --- | --- |
| C53 | 107 |
| C165 | 37 |
| C143 | 33 |
| C100 | 30 |
| C85 | 29 |
| C149 | 26 |
| C47 | 25 |
| C194 | 22 |
| C121 | 19 |
| C124 | 11 |

Two of those cut the other way. C100's continuation-refused-across-a-basis is correctness, not compatibility. C10 and C84, the costliest identity rulings, are explicitly anti-compatibility.

The most-cited rulings overall are not stability promises at all: C122 (112, test method), C144 (87, earned success), C137 (71), C93 (71).

### 3. Reviews
I took every third P1/P2 line from the 185 in reviews added since 2026-11-01 (62 findings; `grep -E '…P[12]\b'`). Classification is judgment:

| Class | Count | Share |
| --- | --- | --- |
| (i) invariant or publisher | 2 | 3% |
| (ii) irreplaceable state or secret | 3 | 5% |
| (iii) correctness a user would notice today | 22 | 35% |
| (iv) compatibility or another build | 10 | 16% |
| (v) wording or a document's claim | 7 | 11% |
| (vi) the method's own tools and records | 18 | 29% |

This agrees with `reports/design-theses.md` T5: "a row this build would not have written" drew 24 findings, 7 of the 20 P1s. The heavy share is (vi), not (iv).

### 4. Goldens and wording
- 74 golden files, 1,049,903 bytes (`git ls-files | grep /goldens/ | xargs cat | wc -c`).
- 795,121 bytes are `goldens/web/`. The largest is `search_pages.txt` at 278 KB.
- 76 of 80 files that read `ACQ_UPDATE_FIXTURES` are in the CLI.
- 2,058 string assertions (`contains("` or `assert_eq!(…, "`) sit in 133k test lines.
- Churn: 50 of the last 300 commits regenerate a golden, 50 of the 102 that touch crates. Those 300 commits span only 2026-11-18 to 11-25; 9 regenerate a REFERENCE.md.

Who the words are frozen for: the owner and the seats. That is a real reader today, since agents read words, so this is not "users who don't exist". But the HTML goldens freeze a surface C191 calls a bootstrap, which the owner has said should move toward the C++ app's layout (memory, 2026-11-19). That 0.8 MB is the clearest case of freezing something nobody wants frozen.

### 5. Unreached states
Taken from the audits rather than re-derived:
- `stored-row.md`: about 15 `check_value` sites sit above the store, plus "about ten types and variants" that one typed read would remove.
- `persistence.md`: the restore machinery is about 420 lines plus about 2,000 test lines. The skeptic kept it for C134's crash loop, a real failure mode rather than a compatibility one.
- Judged total: about 1.5–3k production lines and about 2–3k test lines serve states that have never occurred. That is under 2% of source and about 3% of tests.

### 6. What is irreversible, and how thinly it is protected
- **Facts history** (`first_seen`, 50,847 item events, 223 removed items) can be lost three ways, and nothing backs it up:
  - A rename at GGG orphans it (`index.rs:300`, per `persistence.md`).
  - The remedy text tells the user to delete it: `FACTS_BELOW_FLOOR_REMEDY` (`lib.rs:257-260`) says "facts are refetchable … delete it". `HoldsAccountFacts` prints the same remedy.
  - There is no facts export.
  - C35's stated reason ("facts are refetchable") is false for history and still stands in `AGENTS.md`.
- **Intent and saved searches:** C35 promises "backup is a store-managed snapshot". `Annotations::export` (`annotations.rs:1384`) and `UserStore::export` (`user.rs:191`) are tested nine times, including two concurrent exports racing to one destination (`unit/tests/annotations.rs:1087`). They have no production caller: `git grep '\.export('` outside tests finds only `user.rs:192` delegating. The owner cannot make the backup the ruling promises.
- **The secret gets better rename handling than the history.** `persist_credentials` (`daemon/refresh.rs:105-130`) chases every old keyring name across renames, while the facts file under the same rename is orphaned. Invariant 5 also has no edge check (`stored-row.md`).
- **The release:** its notes say "Expect it to be broken and confusing" (`gh release view v1.0.0-alpha.1`). It promised nothing, yet C53 promises additive JSON and C149 records "the one change that is not additive, taken at the alpha". The real promise to the 23 downloads is the on-disk formats the release wrote, and `c140f64f` already keeps exactly those.

## Answer
How much of the estate serves users who do not exist:

| Part of the estate | Share | How |
| --- | --- | --- |
| Production code | under 2% | judged; 0.3% by keyword, measured |
| Tests | about 3% by name | measured |
| Goldens | 76% of bytes freeze a reshapable page | measured; the reader is the owner, not future users |
| Rulings | about 10 of 200, 5% | judged |
| Review findings | 16% compatibility plus 11% wording | judged |

**Would a pre-user ruling remove work?** Not much from compatibility: the floor already sits at the release, and the protocol is already single-version. What it would remove:
- C53's "JSON changes are additive" clause.
- The (iv) review class for rows "a newer build wrote". That class needs a newer build than the one the owner runs, and he runs main.
- The web HTML goldens, which could become structural assertions.

My judgment is about 1–2k test and golden lines plus a sixth of review findings. Not more. It would not license sloppiness, provided it names what stays frozen.

**Promises that should stay even now:**
- The five invariants.
- C10, C84 and C85's stable plane, because version skew between daemon and client happens daily.
- The release-floor formats, because the owner's own files went through them.
- C35's revision compare-and-swap on intent.
- C47, because a persisted queue turns a panic into a crash loop.
- The refusal of an unknown schema stamp. It is cheap, and the alternative is silently misreading irreplaceable rows.

## What nobody named
The project's stability machinery protects the shapes of its data. It does not protect the data itself.

The only irreplaceable things the owner has are 21 intent rows, 4 saved searches and a facts history no refetch can bring back. The project spent its rigor on refusing every foreign stamp in words held by goldens, while two things went undelivered:
- **No backup is reachable.** The intent backup is built and concurrency-tested but has no caller, and the facts history has none at all.
- **The refusal text points at deletion.** Two remedies tell the user to delete the history.

The useful ruling is not "nothing is frozen". It is the reverse: name the three irreplaceable stores, require a reachable backup verb and a rename path for each, and forbid any remedy that deletes one. That is a smaller and more urgent change than retiring compatibility code, of which little is left.

**Not checked:** the owner's store (forbidden), so whether its v8 to v9 upgrade actually ran is inferred; findings outside the sample; production lines serving unreached states beyond the keyword grep; the mock world.

## Appendix: the 62 sampled findings (judged)
- **i:** seat-issues-e (403/503 read as blocked), transport-seam-1.
- **ii:** 4-full-1-saved (P1, `user.db` collision), cut-293-1 (deletion remedy given to a valid store), process-286-fixes-a387 (P1, intent loss).
- **iii:** 148-8de5, form-editor, form-landing, form-place, saved (account), step6-count, plan-jobs, step6-sort (P1), sort-fixes (P1), 56-list-paging, daemon-208, daemon-209-fixes, frontends-246-fixes (P1 race), network-288, network-291-fixes, seat-issues-a, seat-issues-b-fixes, seat-issues-c, seat-issues-c-fixes, seat-issues-e-fixes, store-279, web-step6-seat.
- **iv:** 157-leagues-held, form-source, saved-fixes, step6-grouped, daemon-209, daemon-261 (P1), frontends-246 (P1, `version:2`), frontends-246-fixes (main-account), seat-issues-b, seat-issues-e (v1 apply).
- **v:** step6-values, 97-c143-reach-fixes, answer-facts-seat-fixes-2, every-epoch-words, frontends-211, seat-issues-c (remedy), trade-150.
- **vi:** 151-stuck-verdict, 175-test-process-pin, step6-check, step6-spike (×3), 246-backout, governance-docs, governance-seat, network-288-fixes, process-238, process-240, process-286, process-286-fixes-aef8, sheet-zd, sitting-282, time-residue, transport-seam-3-brief.
