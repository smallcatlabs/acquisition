# The skeptic on theses 6 to 10

Codex (`gpt-6.1-sol`, read-only sandbox), a model outside the orchestrator's family (C177), asked to break `THESES.md` as it stood, with this charge and nothing of the orchestrator's reasoning: check every claim, give the smallest input that gets worse, name what the thesis did not see and what would be missed, and end SURVIVES, WEAKENED or DIES. The answer is whole; local paths are shortened and its links written as `path:line`. The orchestrator's check of its counter-claims is in `PORTFOLIO.md`.

## Thesis 6

Restore-only pruning is real (`crates/acquisition-daemon/src/daemon/queue.rs:326`). The complete-body duplication claim is wrong: fact responses lose their item arrays, and `items` holds one current row per item (`crates/acquisition-store/src/schema.sql:16`). I did not independently remeasure the snapshot’s database sizes.

C111 suppresses requests using landed facts and live jobs; it supplies no durable record of failed attempts (`crates/acquisition-plan/src/lib.rs:1630`).

Smallest break: a route’s probe panics; restart and reapply. Today the fault marker closes that route even to newly submitted work (`crates/acquisition-daemon/src/daemon/tests/exit.rs:1656`). Remove the marker and persisted queue, and the prescribed rerun recreates the probe. C134’s quarantine becomes a repeatable crash/send loop.

Another loss: fetch a stash, change an item, fetch again, then request the first job’s result. A response pointer cannot reconstruct its original items. Users lose historical response evidence alongside C27’s durable reattachment.

The thesis also missed a production payload consumer: login extracts `payload.body.uuid` (`crates/acquisition-daemon/src/daemon/login.rs:244`).

The removable bytes do not establish that durable work and fault quarantine are removable.

DIES

## Thesis 7

The 45 negative-weight rows and rejected arithmetic sort check out (`crates/acquisition-search/reference/totals-v6.toml:222`, `crates/acquisition-search/src/parse.rs:95`). The claimed equivalence does not.

Smallest break: one item has two life occurrences, 40 and 25. Its line comparison against 60 fails; its sum passes; its projection sorts by 40. The existing boundary test already demonstrates these distinctions (`crates/acquisition-search/tests/rows.rs:300`). Replacing them with line sums destroys C92’s occurrence semantics and changes users’ matches and ordering.

The deletion estimate also includes nonlinear behavior: `count_of` tracks an uncertainty interval, and `least_of` computes a minimum with absence propagation (`crates/acquisition-search/src/pseudo.rs:775`). One “nothing” flag does not express those operations.

“Five weighted refusals” undercounts the function: it additionally rejects empty filters, nonpositive weights, missing bounds and inexact division (`crates/acquisition-search/src/trade/translate.rs:2670`). Arithmetic does not automatically settle the site’s semantics.

Dropping `pseudo.` also breaks existing saved query text. C104 and C194 make that text a durable surface.

WEAKENED — limit this to linear arithmetic over existing value semantics; preserve occurrence operations, nonlinear readings, uncertainty and existing names.

## Thesis 8

The arithmetic totals are consistent, but the sample excluded beast rounds and sparsely sampled later rounds (`audits/whole-review-2026-11/reports/investigators/search-kernel.md:138`). The orchestrator explicitly did not rerun that tally (`audits/whole-review-2026-11/findings/search-kernel.md:27`).

The fatal inference is equating measured agreement with exact translation. C119 counts differences between specified readings. A missing reading supplies no measurement; the implementation also excludes undecided outcomes from its add/drop counts (`crates/acquisition-search/src/answer.rs:3013`).

Smallest break: an empty corpus and a local Attack Speed filter. Every measured stake is zero. The thesis calls it exact although the catalogue explicitly records that the translation also matches the global twin (`crates/acquisition-search/reference/trade-catalogue-v1.toml:1310`). C99’s semantic promise becomes dependent on which items happen to be present.

What disappears is the warning that a pasted query means something different, even when today’s inventory cannot expose it. Another account, scope or future refresh can.

Keeping the oracle does not rescue this: it tests returned items and cannot detect excess matches (`crates/acquisition-search/tests/trade_oracle.rs:1`). Stopping sittings also freezes the evidence that could settle those excesses.

DIES

## Thesis 9

The central diagnosis is correct: `record_login` explicitly orphans the old username-named facts file (`crates/acquisition-store/src/index.rs:300`). Refetching cannot recreate its observation history. The 50,847-event figure remains supplied snapshot evidence.

The deletion list is wider than the code permits. `filename_safe` already serves UUID-named annotation files (`crates/acquisition-store/src/annotations.rs:178`). Changing the facts key does not remove that caller.

Smallest regression: an existing username-keyed credential is migrated; saving under UUID succeeds, but clearing the old entry fails. Delete the keyring-name chase and logout loses its route to that remaining secret. Current code deliberately retains names until cleanup succeeds (`crates/acquisition-daemon/src/daemon/refresh.rs:119`). That hurts C144 and users who expect logout to reach every saved credential.

The thesis also missed bootstrap: the first profile job is submitted under username before UUID exists (`crates/acquisition-daemon/src/daemon/login.rs:225`). Legacy index entries can lack UUID altogether (`crates/acquisition-store/src/index.rs:26`).

Users would miss recoverable sessions, complete logout and preserved WAL-backed history if those transition states were erased.

WEAKENED — UUID becomes the established identity; bootstrap, legacy cleanup reachability and safe path handling remain until their states are retired.

## Thesis 10

Realm already is a protocol type (`crates/acquisition-protocol/src/realm.rs:75`). The eleven-place maintenance comment exists, `get_as` has no production caller, and the quoted search reproduces 129 matching lines across 18 files. Matching lines are not proof of redundant behavior.

Smallest break: one future-version buyout row at revision 7 beside readable rows. Existing reads preserve its address and revision; pricing stops inheritance at that row (`crates/acquisition-plan/src/listing.rs:1452`). `get_as` instead returns `Invalid(ValueError)` without the row metadata (`crates/acquisition-store/src/annotations.rs:1277`). Substituting it wholesale loses the information required to name, clear or safely replace the row, or turns one bad row into a failed listing. C151, C35 and C81 pay.

The realm proposal misses another deliberate state: the store records realm strings without validating them (`crates/acquisition-protocol/src/realm.rs:47`). A closed enum makes an unknown realm unreadable where today it remains observable.

Zero withheld responses and four refused bodies concern different boundaries; they say nothing about whether intent recovery states are expendable. Users would miss partial readability and repair by revision.

WEAKENED — share vocabulary and row decoding only; preserve unknown stored values, row fault metadata and domain-specific unresolved states.
