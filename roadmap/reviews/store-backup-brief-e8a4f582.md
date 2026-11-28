`roadmap/briefs/store-backup.md` at `e8a4f582` — a builder should not start from it unchanged.

Findings, most severe first:

1. **Decisions 1 — changes what gets built.** The existing export does not provide the unconditional no-overwrite guarantee the brief assigns it. `Annotations::export` explicitly documents that its fallback checks existence and then renames, allowing a simultaneous export to replace another on filesystems without hard links (`annotations.rs:1366–1441`). Smallest change: require publication to refuse when it cannot guarantee no replacement, and pin that fallback’s behavior.

2. **Decisions 2 — changes what gets built.** The first thing a builder must guess is how to back up a legitimate world lacking annotations or saved searches. `Seeded::write` deliberately creates neither an annotation file for an account without intent nor `user.db` without saved searches; the read-only openers refuse missing files. Smallest change: record absent sources explicitly in the manifest, leave them uncreated, and add a rehearsal using `without_intent().without_saved()`.

3. **Decisions 3 and 6 / Done when — changes what gets built.** The promised whole-store recovery sequence omits the files and running daemon that login itself produces. `login_with_profile` submits a profile job, and `daemon/kinds.rs` records that profile through `Store::open`, creating the facts file; restore must therefore first refuse the daemon and then skip that file. Smallest change: spell out login, stop, move the newly created facts file and companions aside, restore; rehearse that sequence instead of relying exclusively on deletion within an already indexed world.

4. **Decisions 1–3 / Not in scope — changes what gets built.** The brief defines atomic publication of individual files but leaves interrupted bundle creation and partial restore outcomes undecided. Annotations can export successfully before the facts export fails, leaving an existing destination directory that another backup must refuse; the existing export only cleans its own partial file. Smallest change: define how an incomplete directory is identified and handled, whether restore validates all required sources before publishing any, and explicitly exclude resume or repair if those are deferred.

5. **Done when — changes what gets built.** The rehearsal can pass while losing history named in the goal. It never explicitly compares `first_seen`, and `store events` defaults to ten events within a day (`acquisition-verbs/src/events.rs`), so older events can disappear without changing the compared document. Smallest change: require a fixed clock, an untruncated history comparison covering events outside the default window, and preservation of item `first_seen`; require the concurrently produced backup to restore those contents, rather than merely “open whole.”

6. **Decisions 4 — wording.** The help is instructed to exclude data that the chosen backup necessarily contains. A snapshot of the entire facts file includes current items and responses, yet the help must say the backup does not hold “what a refresh fetches again.” Smallest change: say it includes current facts alongside history, and excludes login credentials and the job queue.

7. **Find first — wording.** Three discovery commands miss their intended holders, and the export command’s accompanying tree claim is false. Executed against `e8a4f582`:
   - The export search under `'crates/*/src'` returns nothing; searching `crates` finds the production delegate in `user.rs:192` and export tests under `unit/tests/annotations.rs`.
   - The refusal search under `'crates/*/tests'` returns nothing; it misses `tests/saved_json.rs`, its golden, and `unit/tests/index.rs`; searching for `FACTS_BELOW_FLOOR_REMEDY` also finds the below-floor tests.
   - The daemon search finds only two `world.rs` documentation lines; it misses `Frontend::observe`, `Client::observe_at`, and `Lock::acquire`.

   Smallest change: search exports under `crates`; search refusal holders with `FACTS_BELOW_FLOOR_REMEDY|holds the facts of an account|restores the account` under both test trees; add `observe_at|pub async fn observe|Lock::acquire|pub fn acquire|lock_path` to the daemon discovery pattern; remove “none outside tests today.”

8. **Done when — wording.** “Tests pin each refusal” contradicts decision 3’s successful skip behavior for an existing file. Smallest change: require a mixed case that preserves an existing file, restores missing files, and reports both outcomes successfully.

9. **Done when / Report — wording.** Reading golden diffs “line by line in the report” cannot be checked from the commit alone, contrary to the template. Smallest change: keep that inspection instruction in Report, and make acceptance require committed golden assertions naming preserved history, moving files aside, and what refresh actually recovers.

What I did not check and why: I did not build, execute software, run the gate, invoke `bothy.py`, or query GitHub issues and CI, because this review was restricted to reading the committed tree. I ran the textual discovery commands against the named commit. I found no explicit contradiction of the five invariants or forbidden dependency edges.

Brief form: the fixed line slice for the architecture check found its intended door grep, but a symbol-based search would survive movement.

Prompt: nothing impeded the review.
