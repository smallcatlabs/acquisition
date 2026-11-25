# The skeptic on theses 1 to 5

Codex (`gpt-6.1-sol`, read-only sandbox), a model outside the orchestrator's family (C177), asked to break `THESES.md` as it stood, with this charge and nothing of the orchestrator's reasoning: check every claim, give the smallest input that gets worse, name what the thesis did not see and what would be missed, and end SURVIVES, WEAKENED or DIES. The answer is whole; local paths are shortened and its links written as `path:line`. The orchestrator's check of its counter-claims is in `PORTFOLIO.md`.

Read-only inspection completed. I ran no Cargo, `acq`, or network requests. Reported timings, private-store counts, and GitHub issue states remain documentary evidence, not independently reproduced measurements.

## Thesis 1

The premise is overstated. The supplied grep returns **122 functions, not 116**; 80 are in `search_cmd.rs`. JSON already carries sentences: refusal construction inserts `error` at `crates/acquisition-verbs/src/refusal.rs:228`, and apply documents carry `note` at `crates/acquisition-verbs/src/refresh.rs:219`. The 20 `Way` variants and 795,121 bytes of web goldens check out. Neither establishes the proposed deletion savings.

Smallest counterexample to identical documents across doors: real provider, absent daemon, one submission. The MCP must explain that it cannot start the daemon; its wording deliberately differs at `crates/acquisition-mcp/src/told.rs:82`. Erasing that distinction loses an actionable explanation of C13.

The proposed 14-tool consequence also removes functionality. Its source proposal explicitly relegates `store_refused` to the CLI. An MCP-only agent investigating one malformed response loses the evidence read currently exposed at `crates/acquisition-mcp/src/server.rs:410`. C30 preserves that evidence precisely to avoid fetching again.

What this misses is capability-specific wording and frontend transport behaviour. Document equality cannot establish exit codes, escaping, HTTP refusal status, or permission differences.

WEAKENED — shared words and calls survive; universal wording, document-equivalence tests, and the claimed tool deletions do not follow.

## Thesis 2

The measurements support a limited latency claim. They do not establish a daily GUI: the trial excludes human interaction, Windows, and concurrent refresh explicitly at `audits/whole-review-2026-11/trials/gui-in-place.md:52`. Compilation occurring concurrently also violates C185’s measurement condition; it does not turn samples into guaranteed upper bounds.

Smallest regression: submit search A, submit search B, press Back. Replacing the current history entry loses A. Navigation currently preserves that return path. A reader comparing two searches would miss it.

The deletion list misidentifies mechanisms. `cut` reconstructs window segments and renders the blocks containing opened and landing rows; it is not merely 303 machinery (`crates/acquisition-cli/src/web/mod.rs:2603`). Delete it while retaining reloadable URL state and an opened item outside the first block loses its reconstruction path.

The proposed region swap also leaves basis metadata outside that region: the template places it in the head at `crates/acquisition-cli/src/web/assets/page.html:7`. Updating rows alone cannot satisfy C16.

The history scan claim is correct. Calling `replaceState` the *one* API keeping URLs honest is false; existing navigation already does that.

WEAKENED — the timings support experimenting with answer-region updates, not striking C191 or deleting reconstruction and navigation behaviour.

## Thesis 3

“Everything defaults to pc/Standard” is too broad. Pricing already infers a known target’s league and refuses multiple possibilities at `crates/acquisition-plan/src/pricing.rs:320`.

The 590/973-byte `world` is also incomplete evidence. Its SQL enumerates live items, while never-fetched places and policy-only leagues were explicitly omitted (`audits/whole-review-2026-11/trials/world.md:19`). Those are exactly the states a first-refresh document must explain.

Smallest damaging situation: account A holds one item; newly authenticated B holds none. An unqualified policy write intended for B currently refuses because both accounts are known (`crates/acquisition-store/src/index.rs:500`). Under the proposed item-count rule, A becomes the sole populated candidate. Population cannot establish authorization to write A’s intent.

Zero items does not mean zero valuable state. Orphaned annotations deliberately survive removed facts at `crates/acquisition-store/src/annotations.rs:8`. This hurts C121 and the owner’s protection against silently choosing an account.

The all-realms example demonstrates honest uncertainty for one total, not semantic equivalence across games. C96 expressly protects realm-specific meanings (`decisions/search.md:23`).

Lost: deliberate account selection, realm-specific interpretation, and visibility into coverage that has no items yet.

WEAKENED — aggregate discovery can survive; item-count-based account selection and blanket scope expansion cannot.

## Thesis 4

The opening claim is false. Small fixes need no brief (`decisions/process.md:18`); test-only and ordinary document-only rounds already escape cold review (`decisions/process.md:26`).

The evidence also contradicts “Next holds only features”: it includes test-harness consolidation and requires the facts-door work before release (`roadmap/README.md:37`).

Smallest counterexample to replacing design review with fixed tasks: two leagues contain tab id `same`; select it, change leagues, then plan the selection. A cold brief review identified that this could authorize the other league’s tab (`roadmap/reviews/4-full-1-step6-places-brief-92f0be8e.md:3`). Three accounts and two realms do not force a seat fixture to contain that collision. The lost protection is C38’s exact authorization.

Your “shell” includes authorization, world selection, and browser security—not merely HTML presentation.

The proposed proof is bounded exploration, not proof of the five invariants. The uncommitted gate explorer currently prints discovered violations and returns successfully (`crates/acquisition-daemon/src/gate.rs:743`). It cannot justify retiring enforcement.

Lost: independent examination of unexercised identity collisions and changes introduced by fixes. The investigator itself reports ten later-round P1s.

DIES.

## Thesis 5

The **200 entries and 133,105 entry bytes** check out. The **53/147 split does not follow C170**. Its actual classes include *the registry’s words and the gate’s stages*, both omitted from the thesis’s paraphrase (`decisions/process.md:7`). The investigator labels its classification judgment from headlines, not a verified partition (`audits/whole-review-2026-11/reports/investigators/process.md:59`).

Smallest concrete regression: open an existing review citing a removed non-law id, then ask `bothy show` for it. That direct lookup disappears. For example, `roadmap/reviews/words-fixes-ee0b6397.md:1` evaluates fixes under C156. Saved reviews remain verbatim under C177; rewriting current source citations does not repair those historical callers.

The claimed migration cost therefore omits immutable reviews and their prompts. Git archaeology remains possible, but is materially worse than a live property lookup.

More seriously, help goldens hold examples and wording. They do not independently preserve cross-frontend promises such as additive JSON compatibility. Moving a promise into one door’s help obscures its authority over another door.

Lost: stable references and a discoverable statement distinguishing a shared obligation from one implementation’s current behaviour.

Removing a number alone has no runtime counterexample. The proposed wholesale classification and relocation do.

WEAKENED — move mechanism prose; preserve shared property authority and identifiers still needed by immutable records.
