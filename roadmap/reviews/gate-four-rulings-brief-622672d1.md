`roadmap/briefs/gate-four-rulings.md` at `622672d1`: a builder cannot start as written; the required first commit explicitly awaits an unresolved answer.

1. **Decisions 2 and 5; Done when — changes what gets built.** The required registry commit contradicts the registry’s entry rules. **Evidence:** Decision 5 requires dated owner quotations inside entries; `tools/bothy.py`’s entry rule 5 forbids both, and `tools/docs-check.sh` rejects those markers. The parenthetical acknowledges the conflict without resolving it. **Smallest change:** require property-only entries, with quotations and dates in the commit message, and remove the pending referral before handoff.

2. **Decision 3; Done when — changes what gets built.** The brief leaves the clone-free generator breaker’s design undecided while requiring it to pass. **Evidence:** `tools/trade-check-breakers.sh` edits committed tables and invokes the real check; the proposed stage skips without a clone, while the generators’ `require(...)` calls refuse absent inputs. The builder must first guess how CI exercises an actual generator mismatch. **Smallest change:** settle the fixture or injection approach and state which refusal must execute without a clone, preserving production pin checks.

3. **Done when, Keychain workflow — changes what gets built.** A successful workflow dispatch does not by itself establish that the Keychain round trip occurred. **Evidence:** `auth.rs`’s `keyring_round_trip_survives_a_fresh_entry` returns successfully when `ACQ_KEYRING_ROUNDTRIP` is absent; `features` only checks target triples and Cargo feature graphs. A consolidated workflow with the opt-in accidentally omitted could produce the required green dispatch without meeting the goal. **Smallest change:** explicitly preserve the opt-in and exact test selector, and require the close message to identify the tested commit, run, and evidence that the named test exercised the OS store.

4. **Inputs — wording.** “The last comment” does not identify a fixed owner ruling. **Evidence:** `gh issue view 257 --comments` and its counterpart for #225 read mutable issue conversations; a later comment changes the designated input. **Smallest change:** identify the accepted recommendations and owner replies by comment permalink or comment id.

5. **Find first, stage selections — wording.** The command does not find every selection it claims to find. **Evidence:** running it against `622672d1` finds `names`, `ONLY`, and `DOCS`, but misses the selection logic at `tools/gate.sh:310–320`, including the loop combining `names` and `ONLY`. **Smallest change:** add `-e 'selected='` and describe the search as locating the declarations and selection logic.

6. **Decision 3; Find first — wording.** The documentation sweep misses a standing statement that the change makes false. **Evidence:** `tools/trade-forms-table.py`’s module doc says the gate has no clone and “never runs it”; neither relevant Find-first command selects that statement, and the decision only requires rewriting `trade-check.sh`’s header. **Smallest change:** include generator documentation in the sweep and require that statement to describe conditional execution.

7. **Not in scope — wording.** The Keychain work lacks an explicit limit on expanding the existing test’s guarantee. **Evidence:** `release.yml` expressly limits the current job to an unsigned test binary using the runner’s login keychain, excluding the signed archive’s item ACL. Extraction and first-run failures could draw the builder into signing or backend changes. **Smallest change:** preserve that limit explicitly and exclude changes to signing, item ACLs, and authentication behavior.

**What I did not check:** I ran the text searches against the named commit and read the relevant scripts, workflows, and rulings. I did not execute project scripts, builds, tests, or workflows. GitHub reads failed with `error connecting to api.github.com`, so I could not verify the owner comments or cited run results.

**Form impediment:** “Settled with the owner” introduces decisions that subsequently declare unresolved prerequisites.

**Prompt impediment:** None beyond the unavailable GitHub inputs.
