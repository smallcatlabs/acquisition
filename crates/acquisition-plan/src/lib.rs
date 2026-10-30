//! The planner: policy compilation and Plan construction (C39). This crate
//! turns a neutral store snapshot ([`acquisition_store::RefreshSnapshot`] —
//! facts and intent named together, nothing derived) plus the sync policy
//! into a [`RefreshPlan`]: a serializable, immutable authorization
//! envelope, computable with the daemon down.
//!
//! Rulings: `decisions/plans.md`, `decisions/pricing.md`; the cross-cutting ones are `AGENTS.md`'s.
//!
//! Boundaries this crate holds (each a registry decision, cited by id):
//! - C39: it is linked by **frontends only**, never the daemon — the
//!   daemon crate never links this one (`tools/arch-check.sh` refuses the
//!   edge), so policy execution cannot reach the daemon by construction.
//! - C38: plans are **binding**: applying one executes exactly the listed
//!   actions or a strict subset, never an unreviewed addition. v1 plans
//!   therefore act only on facts already on record — a league that was
//!   never listed plans the listing alone, and newly discovered tabs wait
//!   for the next plan (honest eventual reconciliation). Dynamic `--deep`
//!   fan-out is excluded: substashes get actions only once their stubs are
//!   in the store.
//! - C37: a policy id covers the tab **and its children** (decided
//!   2026-09-01, tracer rung): a map/unique tab's substashes are planned
//!   once their stubs are on record — the cycle after the parent's first
//!   fetch — and a folder's children at once, since the listing carries
//!   them. One rule, no per-type logic; binding is untouched (every action
//!   is still an explicit tuple). A substash stub reporting 0 items with
//!   nothing held is skipped as `empty_stub` (GGG appears to list only
//!   non-empty substashes; a guard, not a saving).
//! - C36: `metadata.items` counts are heuristic evidence: they can prove a tab
//!   changed (a disagreeing count on a listing newer than our fetch forces
//!   a fetch), never that it didn't (an agreeing count never skips one —
//!   only the policy's own freshness window does that).
//! - Work has two dimensions: `logical_requests` is exact for a refresh;
//!   `wire_sends` is a coarse range plus named prerequisites, never a
//!   precise accounting (the wire-budget feature is deferred).
//!
//! # As built
//!
//! What the planner does:
//!
//! `plan_refresh(provider, &snapshot, now)` parses
//! the snapshot's sync-policy row (the planner owns that value's schema:
//! version-stamped and strict-parsed — a typo'd field is a structured
//! error, never intent half-honored; a newer version is refused as such;
//! **v3** covers per facet under `realms.<R>.leagues.<L>`: `tabs` and/or
//! `characters`, each `"all"` or an id list, absent meaning no coverage
//! of that facet — a v3 value must spell it absent: `[]` or `null`
//! there is refused as non-canonical by the store's exact round-trip
//! (C66, 2026-09-05), while a v1/v2 value's `[]` still upgrades to
//! absent — an entry covering neither refused
//! as "names no work"; `tabs` under `poe2` is a parse error because the
//! stash endpoints are PoE1 only while `characters` is taken under every
//! realm; a v1 `leagues.<L>` value still parses as realm pc, and v1/v2
//! as tab coverage only)
//! and compiles it, with the daemon down, into a serializable
//! `RefreshPlan`: the explicit action set (the league's stash listing
//! and/or per-tab fetches, the realm's character listing and/or
//! per-character fetches — each facet as the policy covers it, each
//! action carrying its reason — never fetched, stale, or a listing newer
//! than our fetch disagreeing: a tab's item count, a character's
//! `experience` or league), covered-but-skipped tabs and characters with
//! reasons (a listed `deleted`/`expired` character is never fetched; one
//! the listing gave no league is outside every league's coverage and
//! reported as such), policy ids the facts lack reported rather than
//! invented into actions, the bases it cites (both listing response ids,
//! policy revision or selection, account uuid, snapshot time and the
//! response the snapshot was read under), exact
//! `logical_requests`, and a coarse `wire_sends` range with named
//! prerequisites (probe, OAuth refresh) — never a precise wire
//! accounting. A plan is authorized on one of two bases, exactly one,
//! which its basis names: the stored policy at a revision (`plan_refresh`;
//! apply refuses it once the row moves, C44), or an explicit selection
//! (`plan_selection`; its authorization the selection itself, no revision
//! checked, C44 as amended, and its listing refreshed on every run,
//! C76). An action is spent once a fetch of its target lands in a
//! response after the snapshot's, or the daemon holds the same request
//! (C111, by response and never the clock). A serialized plan
//! re-validates on parse — unknown fields at any depth (the derived
//! envelope must re-serialize to exactly what was read, which closes the
//! one hole serde leaves: extra fields beside a unit reason's `kind`), a
//! newer schema stamp, a wrong operation, an action outside the
//! envelope's league (the realm-wide character listing carries none and
//! is in-envelope for any league of its realm), or derived quantities
//! that do not recompute (the wire projection, prerequisites included)
//! are refused whole — so apply can trust what it reads back. Plans are
//! binding and act only on facts on record: a never-listed league (or
//! realm, for characters) plans the listing alone (covered tabs and
//! characters are reported as awaiting the listing — without a basis the
//! plan has no membership authority), substash
//! fetches come only from stubs already in the store (no dynamic deep
//! fan-out; one whose recorded parent has been retired is skipped with
//! its reason, never fetched by a guessed path), and newly discovered
//! tabs and characters wait for the next plan. A listed
//! `metadata.items` count forces a fetch when a listing newer than our
//! fetch disagrees with what the store holds; it never skips one. A
//! character fetch is addressed by the *listed* name (identity is the
//! id; the name is the address, and a moved name fails its child
//! honestly or lands the id the server answers with). Each
//! action renders the daemon's own `(kind, params)` job tuple
//! (`RefreshAction::job`), pinned by decoding through
//! `Endpoint::from_job` — the store's production decoder of the job
//! vocabulary — and by a plan→record→replan loop that proves applied
//! actions satisfy the plan. A plan may carry the daemon's **quote** as
//! optional enrichment (C40; tracer step 5, built 2026-09-01): `quote` is its
//! own protocol request — a read-only, non-reserving projection of the
//! work's `(kind, params)` tuples over current limiter state, per
//! scheduling scope with per-window headroom (stamped with its own
//! observation age, read under one limiter lock with the ETA), the queue
//! counted ahead, and a forward-simulated ETA that is an estimate, never
//! a promise — seeded (as a count, safe against absurd headers) with
//! server-reported hits the local history never saw, so it over-waits
//! rather than floods; unlearned routes, probes, OAuth refresh, 429
//! re-sends, and a rails halt are named rather than silently omitted.
//! The quote echoes the job tuples it priced (`work`), and attaching one
//! (`with_quote`) validates it speaks about the plan's own provider,
//! exactly its account, and exactly its actions in order — never just a
//! matching count; carrying it bumped the plan schema (v3; the `empty_stub`
//! skip kind made v4 on 2026-09-01; realm beside league on the envelope
//! and on every action made v5 on 2026-09-02 — every tuple names its
//! realm explicitly, pc included; characters made **v6** the same day:
//! `list_characters` / `fetch_character` actions, `basis.stash_listing` +
//! `basis.character_listing`, `skipped_tabs` / `unknown_tabs` beside
//! `skipped_characters` / `unknown_characters`). Same
//! no-panic clippy ratchet as the store crate.
//!
//! The pricing module begins with reference data: the currency table
//! (C68) is `reference/currency-v1.toml`, compiled into the binary and
//! read through [`currency::table`] — see `currency.rs`, "As built". The
//! typed `buyout` value and its target are `price.rs` (C67); the game
//! side of a listing — what a note or a tab name says — is one pure
//! function, [`game_side::read`] (C69); the listing state — every
//! subject's manual side by specificity (C70), game side through the tab
//! C80 names, and their relation — is [`listing::resolve`] over the
//! store's [`acquisition_store::PricingSnapshot`], see `listing.rs`,
//! "As built". The one write path — a single-row compare-and-swap that
//! returns the row it replaced (C78) and refuses a retired tag for a new
//! price (C67) — is [`price::set_buyout`] / [`price::clear_buyout`],
//! built once for every frontend the way [`put_sync_policy`] is.
//!
//! # Decisions as recorded
//!
//! The rulings are the decision registry — `decisions/plans.md` for this
//! area, `AGENTS.md` for the cross-cutting ones (`C<n>`); what follows is each
//! entry's full text as recorded there, moved here on 2026-09-02 because
//! the mechanism it describes is this module's. The registry is current;
//! this is the mechanism as decided, kept beside the code that implements it.
//!
//! ## C37 — A tab id in the sync policy covers that tab and its children.
//!
//! **A tab id in the sync policy covers that tab and its children.** A tab is covered when its own id is listed or its parent's id is; one rule, no per-type logic, and it gives every case the owner meant (tracer rung, 2026-09-01): a map or unique tab's substashes are planned once their stubs are on record — the cycle after the parent's first fetch, since discovery still waits for facts (binding untouched: every action stays an explicit reviewed tuple, nothing is added at apply); a folder's children are planned at once, because the listing already carries them (folders themselves are never fetched); a child named directly still works. A substash stub whose `metadata.items` is 0 is skipped with a named reason rather than fetched — GGG appears to list only non-empty substashes (64 of 64 stubs on the real account carry a count ≥ 1), so this is a guard, not a saving; in-flight change is not designed around (a count is still never proof of freshness). Type-level filters ("skip map tabs", "include unique tabs", "fetch folder children") are parked with a trigger. Evidence: the rung's five-id run left 64 substashes under the two selected parents in the store and outside the policy, closed the loop after one cycle, and the owner's intent for a named map tab was its contents. Consequence: the same policy rerun plans those 64 as cycle 2 (~15 min of limiter holds) — the first live two-cycle discovery sample. Built the same day: `TabSelection::covers` (own id or parent's; `covers_tab` over `Selection` since step (3) of the characters work), `SkipReason::EmptyStub`, plan schema **v4** (a new skip kind is an envelope shape change). Decided 2026-09-01.
//!
//! ## C38 — A Plan is a serializable, immutable authorization envelope, and plans are binding.
//!
//! **A Plan is a serializable, immutable authorization envelope, and plans are binding.** Derived from a named snapshot of facts + intent, computable with the daemon down; it carries provider + account uuid, operation kind + plan schema version, fact basis (response/listing ids or timestamps), annotation revision, the explicit action set (or a declared upper bound), generated-at, freshness assumptions, and optionally a quote with its own observation time. Work has two dimensions: `logical_requests` (exact or bounded) and `wire_sends` (a coarse range plus named prerequisites — probe, token refresh, possible 429 retries — never a precise accounting). Applying a Plan executes exactly the listed actions or a strict subset, never an unreviewed addition; new facts produce a new Plan; v1 excludes dynamic `--deep` fan-out (a vanished tab fails or is reported skipped; newly discovered tabs wait for the next plan). Operation-specific types first (`RefreshPlan`); a universal grammar waits for the second plan-bearing consumer. Binding was revisable on tracer evidence (the owner's live-use friction notes are the data). Decided 2026-08-31. **Confirmed 2026-09-01** on the tracer rung's live run (`RUN-LEDGER.md`, the 2026-09-01 row and its friction note): subset-only reconciliation produced no owner friction, the two-cycle discovery of substashes cost nothing observable, and the parking-lot trigger for dynamic fan-out ("two-cycle reconciliation genuinely hurts") did not fire. What the run did surface was a *coverage* question, ruled separately below (a policy id covers the tab and its children); binding itself stands as written.
//!
//! ## C44 — Step 7's staleness ruling: apply refuses a plan whose sync-policy revision is no longer…
//!
//! **Step 7's staleness ruling: apply refuses a plan whose sync-policy revision is no longer the stored one.** A plan is authorization *derived from intent at a revision*; intent edited since revokes the derivation — the CAS reasoning extended from writing intent to spending it, and the refusal is cheap because replanning is offline and free. Checked frontend-side against a fresh read of the policy row immediately before submit (the daemon is intent-blind by the four-layer decision, so only a frontend can compare); a missing row, a different revision, a mismatched account uuid, or a mismatched provider each refuse with the remedy named. Two accepted residuals: the check races a concurrent policy write between read and submit (the same human-boundary register as `policy set` without `--if-revision`), and **fact drift does not refuse** — the authorization is the bounded action set, not a world-state assertion; the actions stay idempotent GETs of exactly the reviewed tuples (a since-vanished tab's fetch fails its own child honestly) and the next plan reconciles what a newer listing changed. Decided 2026-09-01.
//!
//! ## C61 — Listed deleted or expired characters are skipped
//!
//! **Listed `deleted` or `expired` characters are skipped with a named
//!   reason, not fetched**, until evidence says otherwise. The docs define
//!   neither beyond "always `true` if present", and the invalid-request
//!   threshold (too many 4xx restricts the app, independent of rate
//!   limits) makes a 404 hunt a real cost. Observed: characters in ended
//!   leagues (Ancestors, Phrecia 2.0, an event) are listed with **no
//!   `expired` flag**, so `expired` does not mean "league ended", and
//!   **league names on characters are not restricted to
//!   `/account/leagues`** — the planner treats the league key as an opaque
//!   string. `Character.league` is optional (documented): a character with
//!   none is reported as uncovered, never a failure.
//!
//! ## C62 — Freshness heuristic for characters
//!
//! **Freshness heuristic, owner's call for v1:** the list entry carries
//!   `experience` (observed on all 59), monotone for a character's life; a
//!   listing newer than our fetch reporting a different `experience` proves
//!   play since — the sibling of `ListedCountDisagrees`. A `league`
//!   disagreement (a Hardcore death landing in Standard) is the same arm.
//!
//! ## C58 — Policy v3 shape (agreed 2026-09-02)
//!
//! **Policy v3 shape (agreed 2026-09-02, for step (3)) — coverage per
//! facet:** `realms.<R>.leagues.<L>.{tabs?, characters?, max_age_seconds}`
//! where each facet is `"all"` or an id list and **absent means no coverage
//! of that facet** (an empty list is the same, explicitly); an entry naming
//! neither facet is malformed ("names no work"), never a silent no-op.
//! Validation is per facet against `Family::accepts`: `tabs` is refused
//! under a realm the stash family does not take (poe2), `characters` is
//! accepted under every realm — so a character-only PoE2 entry is the
//! ordinary v3 shape. v1 and v2 upgrade to their tab coverage plus no
//! character coverage; the stored value stays as written. "Names no work"
//! is judged after normalization: `tabs: []` with characters absent or
//! empty fails the same way. (Review finding 2026-09-02: v2's required
//! `tabs` and entry-level realm check could not express the ruled PoE2
//! policy.)
//!
//! ## C62 — Planner calls made inside the characters ruling (step (3))
//!
//! Planner calls made inside the ruling (step (3)):
//!
//! - **The planner is facet-symmetric, not character-aware.** `Selection`
//!   (`all` | ids) serves both facets; `LeaguePolicy { tabs?, characters?,
//!   max_age_seconds }` with `None` meaning no coverage of that facet (an
//!   empty list normalizes to `None`); each facet compiles on its own —
//!   its listing verdict, its covered entities, its skips, its unknowns —
//!   and the actions are the tab facet's then the character facet's. The
//!   freshness rules are one function for both (`window_verdict`: never
//!   fetched, older than the window) plus each facet's own disagreement
//!   arm: `ListedCountDisagrees` for tabs; `ListedExperienceDisagrees`
//!   (judged only when the entry and the fetched body both carry
//!   `experience`) and `ListedLeagueDisagrees` (the row's listing-owned
//!   league against the body's — a Hardcore death) for characters, on the
//!   shared `FetchReason`. Skips are their own enum,
//!   `CharacterSkipReason { Fresh, AwaitingListing, NoLeague, Deleted,
//!   Expired }`, so a tab kind on a character skip is malformed.
//! - **"Names no work" applies after normalization to every version.** A
//!   stored v2 `tabs: []` now reads as malformed instead of compiling to an
//!   empty plan; nothing stored that (the MCP loop test's "empty coverage"
//!   fixture became an id the facts lack, which is the honest empty plan).
//! - **League-less characters are reported in every league plan of their
//!   realm** as a `no_league` skip (by id too — never as unknown, the facts
//!   know them): the ruling said "reported as uncovered", and a realm fact
//!   no league key reaches needs somewhere to appear. The snapshot carries
//!   them beside the league's own rows.
//! - **The store's liveness rule is consumed, not restated.**
//!   `CharacterSnapshot` offers exactly `fetched_at` (None for never
//!   fetched *or revived*), the listed entry verbatim, and the fetched
//!   envelope only while a fetch stands (`Null` after revival — the column
//!   still holds the disowned body). The planner reads those three facts
//!   and has no liveness logic. `Store::refresh_snapshot` is
//!   `stash_listing` + `tabs`,
//!   `character_listing` (per realm) + `characters`, the policy row, one
//!   read transaction.
//! - **A strictness hole the character skip test found, closed:** serde
//!   ignores extra fields beside an internally tagged *unit* variant's
//!   `kind` (`never_fetched`, `folder`, `deleted`…), `deny_unknown_fields`
//!   or not — "unknown fields at any depth are refused" held only for
//!   struct variants. `RefreshPlan::from_value` now requires the derived
//!   envelope (the quote aside — an observation with its own strict shape)
//!   to re-serialize to exactly what was read. The same shape as the
//!   store's five rounds: a rule honored on the main path, lost on one
//!   serde path.
//! - **Apply vocabulary**: `characters`, and `character` with a name; both
//!   single-request. `acq store characters` is new: the CLI had no
//!   store-side read of the character rows (only the MCP tool), and the
//!   driver's readback needs one. The mock's list entries and bodies carry
//!   `experience` (one constant: the mock account never plays).
//! - **Accepted residual:** the character list is realm-wide while plans
//!   are per league, so two league plans on one realm applied together
//!   each authorize their own `list_characters` — the same register as two
//!   realm plans listing separately; v1 lives with it.

// The lint ratchet (C47, "Panics are for broken internal invariants
// only"): the planner's production code panics on nothing external — a
// user-authored policy value or a store row is a structured error.
#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]

pub mod currency;
pub mod game_side;
pub mod listing;
pub mod price;
#[cfg(test)]
#[path = "tests/price_notes_fixture.rs"]
pub(crate) mod price_notes_fixture;
pub mod pricing;

/// The league a verb reads when none is given: one constant for the
/// planner, the verbs crate and every frontend.
pub use pricing::DEFAULT_LEAGUE;
#[cfg(test)]
#[path = "tests/real_scale_fixture.rs"]
pub(crate) mod real_scale_fixture;
pub mod shop;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use acquisition_protocol::job::{JobId, JobInfo, MAX_429_RETRIES};
use acquisition_protocol::protocol::Quote;
use acquisition_protocol::realm::{Family, Realm};
use acquisition_store::{
    AnnotationError, AnnotationRow, Annotations, CharacterSnapshot, Endpoint, Fetched, IntentValue,
    LeagueList, ListingBasis, Provenance, RefreshSnapshot, SYNC_POLICY_KEY, SYNC_POLICY_KIND,
    SYNC_POLICY_SCOPE, TabSnapshot, ValueError, check_value,
};

/// The plan envelope schema this build writes ([`RefreshPlan::plan_schema`]).
/// A consumer handed a plan stamped newer refuses it rather than guessing.
/// History: v1 = the tracer-step-4 envelope; v2 (2026-09-01) added the
/// optional `quote` enrichment; v3 (same day) added `Quote::work`, the
/// quote's verifiable work basis. A shape change anywhere in the envelope —
/// the embedded `Quote` included — is a schema bump, so an older reader
/// reports "newer schema" instead of "malformed" on a newer plan.
/// v4 added the `empty_stub` skip kind; v5 (2026-09-02) put `realm`
/// beside `league` on the envelope and on every action; v6 (same day)
/// brought characters in: the `list_characters` and `fetch_character`
/// actions, the character listing basis (`basis.character_listing`,
/// beside the renamed `basis.stash_listing`), `skipped_characters` and
/// `unknown_characters` beside the renamed `skipped_tabs` and
/// `unknown_tabs`, and two character disagreement arms on `FetchReason`.
pub const REFRESH_PLAN_SCHEMA: i64 = 6;

/// The sync-policy value schema this build reads ([`SyncPolicy::version`]).
/// The store carries the row opaquely; its shape is this crate's business.
/// v2 (2026-09-02) nests leagues under realms
/// (`realms.<R>.leagues.<L>`); a v1 policy (`leagues.<L>`) still parses,
/// upgraded on the way in as realm pc — the only realm v1 could mean. v3
/// (same day) puts `characters` beside `tabs` under each league, each
/// facet optional — absent means no coverage of that facet, and an entry
/// covering neither is malformed ("names no work"). Since 2026-09-05 a
/// v3 value is held to its canonical spelling (C66): an empty list or
/// `null` for a facet is refused naming the path, not normalized. v1 and
/// v2 upgrade to their tab coverage (an empty list there reads as
/// absent) plus no character coverage.
pub const SYNC_POLICY_VERSION: i64 = 3;

/// A planner failure with a stable kind (C47: malformed external
/// input is a structured error, never a panic and never a bare string).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanError {
    /// The snapshot carries no sync-policy row: there is no declared
    /// intent to compile. Distinct from "covered and fresh" (a valid plan
    /// with zero actions).
    NoSyncPolicy,
    /// The sync-policy value did not parse as this build's schema. The
    /// policy is user-authored intent, so an unknown field is malformed
    /// too — a typo must not be silently ignored into a different policy.
    MalformedPolicy { detail: String },
    /// The policy declares a version this build does not read. Refused,
    /// never guessed at — same rule as the store's schema stamps.
    PolicyVersionUnsupported { found: i64, supported: i64 },
    /// The policy declares nothing for the snapshot's (realm, league);
    /// there is no authorized work to derive.
    LeagueNotCovered { realm: Realm, league: String },
    /// The snapshot names a realm this build does not know. Facts are
    /// stamped with the request's realm, so this is a store written by a
    /// build with a wider table — refused, never guessed at.
    UnknownRealm { realm: String },
    /// A selection names tabs under a realm the stash endpoints do not
    /// take (poe2): refused before a plan exists, in the words the policy
    /// refuses `tabs` there with.
    TabsNotTaken { realm: Realm },
    /// A serialized plan failed validation: unknown fields, a wrong
    /// operation, a league its envelope does not name, or derived counts
    /// that do not recompute. A plan that will not parse is a plan apply
    /// must not trust.
    MalformedPlan { detail: String },
    /// A serialized plan carries a schema stamp this build does not
    /// read. Refused, never guessed at.
    PlanSchemaUnsupported { found: i64, supported: i64 },
}

impl std::fmt::Display for PlanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanError::NoSyncPolicy => {
                write!(f, "no sync policy is set for this account")
            }
            PlanError::MalformedPolicy { detail } => {
                write!(f, "malformed sync policy: {detail}")
            }
            PlanError::PolicyVersionUnsupported { found, supported } => write!(
                f,
                "sync policy declares version {found}, newer than this build's v{supported}"
            ),
            PlanError::LeagueNotCovered { realm, league } => {
                write!(
                    f,
                    "the sync policy does not cover league {league} on realm {realm}"
                )
            }
            PlanError::UnknownRealm { realm } => {
                write!(
                    f,
                    "the snapshot names realm {realm:?}, which this build does not know"
                )
            }
            PlanError::TabsNotTaken { realm } => f.write_str(&tabs_not_taken(*realm)),
            PlanError::MalformedPlan { detail } => {
                write!(f, "malformed refresh plan: {detail}")
            }
            PlanError::PlanSchemaUnsupported { found, supported } => write!(
                f,
                "refresh plan declares schema {found}, not this build's v{supported}"
            ),
        }
    }
}

impl std::error::Error for PlanError {}

/// Tabs named under a realm the stash family does not take: the one
/// sentence a policy and a selection are refused with.
fn tabs_not_taken(realm: Realm) -> String {
    format!("tabs under realm {realm}: the stash endpoints do not take it (PoE1 only)")
}

/// The per-account sync policy: an inspectable declaration of desired
/// coverage and freshness — not a scheduler. Stored as the
/// `("account", "", "sync-policy")` annotation; written by frontends,
/// compiled here into minimal requests. In memory it is always the v3
/// shape; a stored v1 or v2 value is upgraded on parse (v1 as realm pc,
/// both as tab coverage only) and stays stored as written — what the
/// human typed is what `policy show` shows — until a write merges into
/// it ([`PolicyWrite::Merge`]), which stores the merged policy as v3.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SyncPolicy {
    /// [`SYNC_POLICY_VERSION`] once parsed (older values read as v3).
    pub version: i64,
    /// Coverage per realm, then per league. A (realm, league) not named
    /// here compiles to [`PlanError::LeagueNotCovered`], never to
    /// implicit work.
    pub realms: BTreeMap<Realm, RealmPolicy>,
}

/// One realm's coverage: its leagues. The realm key is what the requests
/// are rendered with; a league entry that names work an endpoint family
/// does not take on this realm (tabs under `poe2`, PoE1-only stashes) is
/// a parse error, so no policy can ask for an unobserved URL shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RealmPolicy {
    pub leagues: BTreeMap<String, LeaguePolicy>,
}

/// One league's coverage: per facet — tabs, characters — plus the
/// freshness window they share. `None` is exactly "no coverage of that
/// facet": no listing, no fetches, nothing skipped or reported for it.
/// (An empty id list normalizes to `None` on parse.) At least one facet
/// is `Some`; the parse refuses an entry that names no work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LeaguePolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tabs: Option<Selection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<Selection>,
    /// The freshness declaration: facts older than this want refreshing.
    /// Applies to each facet's listing and to each covered fetch alike.
    pub max_age_seconds: u32,
}

/// The raw JSON shapes, one strict struct per version — `deny_unknown_fields`
/// on each, at every depth, so a stray field is refused whatever the stamp
/// (an untagged enum would lose that: review finding 2026-09-02). The
/// stamp picks exactly one shape ([`parse_policy`]), and `SyncPolicy`'s
/// `Deserialize` goes through the same function, so the version gate, the
/// unknown-field refusal, the names-no-work rule, and the per-facet
/// family check cannot be bypassed by deserializing around
/// [`SyncPolicy::from_value`].
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SyncPolicyWireV1 {
    /// The stamp [`parse_policy`] already dispatched on; named so the
    /// strict parse does not refuse the key, never read again.
    #[serde(rename = "version")]
    _version: i64,
    leagues: BTreeMap<String, LeagueWireV2>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SyncPolicyWireV2 {
    /// The stamp [`parse_policy`] already dispatched on; named so the
    /// strict parse does not refuse the key, never read again.
    #[serde(rename = "version")]
    _version: i64,
    realms: BTreeMap<Realm, RealmWire<LeagueWireV2>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SyncPolicyWireV3 {
    /// The stamp [`parse_policy`] already dispatched on; named so the
    /// strict parse does not refuse the key, never read again.
    #[serde(rename = "version")]
    _version: i64,
    realms: BTreeMap<Realm, RealmWire<LeagueWireV3>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RealmWire<L> {
    leagues: BTreeMap<String, L>,
}

/// v1 and v2 league entries: `tabs` required, characters unknown.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LeagueWireV2 {
    tabs: Selection,
    max_age_seconds: u32,
}

impl From<LeagueWireV2> for LeaguePolicy {
    fn from(w: LeagueWireV2) -> LeaguePolicy {
        LeaguePolicy {
            tabs: w.tabs.normalize(),
            characters: None,
            max_age_seconds: w.max_age_seconds,
        }
    }
}

/// v3 league entries: each facet optional.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LeagueWireV3 {
    tabs: Option<Selection>,
    characters: Option<Selection>,
    max_age_seconds: u32,
}

impl From<LeagueWireV3> for LeaguePolicy {
    fn from(w: LeagueWireV3) -> LeaguePolicy {
        LeaguePolicy {
            tabs: w.tabs.and_then(Selection::normalize),
            characters: w.characters.and_then(Selection::normalize),
            max_age_seconds: w.max_age_seconds,
        }
    }
}

fn realm_policy<L: Into<LeaguePolicy>>(leagues: BTreeMap<String, L>) -> RealmPolicy {
    RealmPolicy {
        leagues: leagues.into_iter().map(|(l, e)| (l, e.into())).collect(),
    }
}

impl<'de> Deserialize<'de> for SyncPolicy {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(d)?;
        SyncPolicy::from_value(&value).map_err(serde::de::Error::custom)
    }
}

/// The sync policy as an intent kind (C66): the store's generic — version
/// gate, exact round-trip for a v3 value, compare-and-swap — wraps this
/// crate's own strict parse, which stays exactly what it was.
impl IntentValue for SyncPolicy {
    const KIND: &'static str = SYNC_POLICY_KIND;
    const VERSION: i64 = SYNC_POLICY_VERSION;
    fn parse(value: &Value) -> Result<Self, String> {
        parse_policy(value)
    }
}

impl From<ValueError> for PlanError {
    fn from(e: ValueError) -> PlanError {
        match e {
            ValueError::VersionUnsupported {
                found, supported, ..
            } => PlanError::PolicyVersionUnsupported { found, supported },
            ValueError::MissingVersion { .. } => PlanError::MalformedPolicy {
                detail: "missing integer `version`".into(),
            },
            ValueError::Malformed { detail, .. } => PlanError::MalformedPolicy { detail },
            // The policy has no writer's rule today; one would be a
            // refusal in its own words, like a malformed value.
            e @ (ValueError::NotCanonical { .. } | ValueError::RefusedForWrite { .. }) => {
                PlanError::MalformedPolicy {
                    detail: e.to_string(),
                }
            }
        }
    }
}

/// The one parse: the `version` stamp selects the strict shape, the
/// shape is validated whole, older values upgrade (v1 to realm pc; v1
/// and v2 to tab coverage only), every entry must name work, and no
/// facet may name work its endpoint family does not take on that realm
/// (`tabs` under poe2; `characters` is taken everywhere).
fn parse_policy(value: &Value) -> Result<SyncPolicy, String> {
    let version = value
        .get("version")
        .and_then(Value::as_i64)
        .ok_or_else(|| "missing integer `version`".to_string())?;
    let realms: BTreeMap<Realm, RealmPolicy> = match version {
        1 => {
            let wire: SyncPolicyWireV1 =
                serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
            BTreeMap::from([(Realm::Pc, realm_policy(wire.leagues))])
        }
        2 => {
            let wire: SyncPolicyWireV2 =
                serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
            wire.realms
                .into_iter()
                .map(|(r, p)| (r, realm_policy(p.leagues)))
                .collect()
        }
        3 => {
            let wire: SyncPolicyWireV3 =
                serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
            wire.realms
                .into_iter()
                .map(|(r, p)| (r, realm_policy(p.leagues)))
                .collect()
        }
        other => {
            return Err(format!(
                "sync policy declares version {other}, not this build's v{SYNC_POLICY_VERSION}"
            ));
        }
    };
    for (realm, policy) in &realms {
        for (league, entry) in &policy.leagues {
            // Judged after normalization: `tabs: []` with characters
            // absent or empty names no work either — an entry that
            // covers nothing is a mistake to report, never a silent no-op.
            if entry.tabs.is_none() && entry.characters.is_none() {
                return Err(format!(
                    "{realm}/{league}: names no work — neither `tabs` nor `characters` \
                     covers anything (absent, or an empty list)"
                ));
            }
            if entry.tabs.is_some() && !Family::Stashes.accepts(*realm) {
                return Err(tabs_not_taken(*realm));
            }
        }
    }
    Ok(SyncPolicy {
        version: SYNC_POLICY_VERSION,
        realms,
    })
}

/// What a facet covers: `"all"` or an explicit id list. For tabs an id
/// covers the tab and its children (`covers_tab`): a map/unique tab's
/// substashes (their own GGG ids; the `parent/id` display convention is a
/// frontend matter) and a folder's children. For characters an id is the
/// GGG character id, matched exactly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SelectionRepr", into = "SelectionRepr")]
pub enum Selection {
    All,
    Ids(Vec<String>),
}

impl Selection {
    /// An empty id list covers nothing, which is the same declaration as
    /// leaving the facet out.
    fn normalize(self) -> Option<Selection> {
        match self {
            Selection::Ids(ids) if ids.is_empty() => None,
            some => Some(some),
        }
    }

    /// Whether `id` itself is named.
    pub fn covers_id(&self, id: &str) -> bool {
        match self {
            Selection::All => true,
            Selection::Ids(ids) => ids.iter().any(|i| i == id),
        }
    }

    /// C37 — a tab is covered when its own id is named or its parent's
    /// is; the one rule for a folder child and a substash alike. Public
    /// so a consumer that reports coverage rather than compiling it
    /// (`shop render`, C72) asks the same question the planner does.
    pub fn covers_tab(&self, id: &str, parent: Option<&str>) -> bool {
        self.covers_id(id) || parent.is_some_and(|p| self.covers_id(p))
    }
}

/// A tab is covered when its own id is named or its parent's is.
fn covers_tab(selection: &Selection, tab: &TabSnapshot) -> bool {
    selection.covers_tab(&tab.id, tab.parent.as_deref())
}

/// The JSON shape: the string `"all"` or an array of ids.
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum SelectionRepr {
    Word(String),
    Ids(Vec<String>),
}

impl TryFrom<SelectionRepr> for Selection {
    type Error = String;
    fn try_from(repr: SelectionRepr) -> Result<Self, String> {
        match repr {
            SelectionRepr::Word(w) if w == "all" => Ok(Selection::All),
            SelectionRepr::Word(w) => Err(format!(
                "coverage must be \"all\" or a list of ids, not \"{w}\""
            )),
            SelectionRepr::Ids(ids) => Ok(Selection::Ids(ids)),
        }
    }
}

impl From<Selection> for SelectionRepr {
    fn from(sel: Selection) -> SelectionRepr {
        match sel {
            Selection::All => SelectionRepr::Word("all".into()),
            Selection::Ids(ids) => SelectionRepr::Ids(ids),
        }
    }
}

impl SyncPolicy {
    /// Parse a stored sync-policy value through the store's generic
    /// ([`check_value`], C66): the version stamp is checked before the
    /// full shape so a genuinely newer policy reports
    /// [`PlanError::PolicyVersionUnsupported`], not a spurious
    /// unknown-field complaint; v1, v2 and v3 all parse (older values
    /// upgrade in memory); a v3 value must re-serialize to exactly what
    /// was read, so `tabs: []` or `characters: null` — spellings the parse
    /// would normalize away — are refused naming the path, never stored
    /// as a second spelling of the same intent.
    pub fn from_value(value: &Value) -> Result<SyncPolicy, PlanError> {
        check_value::<SyncPolicy>(value).map_err(PlanError::from)
    }

    /// One (realm, league)'s coverage, if the policy names it.
    pub fn league(&self, realm: Realm, league: &str) -> Option<&LeaguePolicy> {
        self.realms.get(&realm).and_then(|r| r.leagues.get(league))
    }
}

/// C126 — a league the list lacks is a note: a league a policy or a plan
/// names that the account's fetched league list does not hold. The list
/// is the latest successful leagues response (`LeagueList`), never an
/// accumulated table; `list_fetched_at` is when it was fetched, `None`
/// when no list was ever fetched, so every league is unread against it.
/// Nothing refuses on it: the list is stale on launch morning before the
/// user fetches it, and the note is what keeps a typo from spending a
/// request on a league that does not exist. The policy receipt says it
/// in both cases, once, when the policy is written; a plan says it only
/// when a fetched list lacks its league, and is silent when no list was
/// ever fetched, so a user who never asks for the list meets the note
/// once, not on every plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LeagueNote {
    pub league: String,
    pub list_fetched_at: Option<i64>,
    /// C53: `list_fetched_at` in words at the answer's time (`2d ago`,
    /// `never`), as the note's text says it.
    pub list_fetched_at_age: String,
}

/// C126: a note for each of `leagues` the list does not hold, in the
/// order given, each league once, the list's fetch said as an age at
/// `now`, the answer's time (C53).
pub fn league_notes<'a>(
    leagues: impl IntoIterator<Item = &'a str>,
    list: Option<&LeagueList>,
    now: i64,
) -> Vec<LeagueNote> {
    let mut notes: Vec<LeagueNote> = Vec::new();
    for league in leagues {
        let held = list.is_some_and(|l| l.ids.iter().any(|id| id == league));
        if !held && !notes.iter().any(|n| n.league == league) {
            notes.push(LeagueNote {
                league: league.to_string(),
                list_fetched_at: list.map(|l| l.fetched_at),
                list_fetched_at_age: acquisition_store::age::ago(now, list.map(|l| l.fetched_at)),
            });
        }
    }
    notes
}

impl SyncPolicy {
    /// Every league the policy names, realm by realm, each once.
    pub fn leagues(&self) -> Vec<&str> {
        let mut out: Vec<&str> = Vec::new();
        for realm in self.realms.values() {
            for league in realm.leagues.keys() {
                if !out.contains(&league.as_str()) {
                    out.push(league);
                }
            }
        }
        out
    }
}

/// What a compile saw beside the plan it made: observations of the store
/// at the snapshot, never authorization, so they ride beside the envelope
/// (as the quote's note does) and never in it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct PlanNotes {
    /// C126: the plan's league, when a fetched league list lacks it; none
    /// when no list was ever fetched (the receipt said so).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub league: Option<LeagueNote>,
    /// The policy covers this league's characters, the realm's character
    /// listing is on record, and it names none in the league: why the
    /// plan has no character work (a new league before its first
    /// character).
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub no_characters_listed: bool,
    /// C53: beside `no_characters_listed`, when the realm's character
    /// listing that names none was fetched, in words at the answer's
    /// time (the epoch is the plan's `basis.character_listing.fetched_at`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_listing_fetched_at_age: Option<String>,
}

impl PlanNotes {
    pub fn is_empty(&self) -> bool {
        self.league.is_none() && !self.no_characters_listed
    }
}

/// The notes of a compile over `snapshot` ([`PlanNotes`]). A snapshot
/// with no policy, or whose policy does not cover its league, has none:
/// the compile refuses it first. Each age a note's text prints is said
/// at `now`, the answer's time (C53).
pub fn plan_notes(snapshot: &RefreshSnapshot, now: i64) -> PlanNotes {
    let covered = snapshot
        .policy
        .as_ref()
        .and_then(|row| SyncPolicy::from_value(&row.value).ok())
        .and_then(|policy| {
            let realm = Realm::parse(&snapshot.realm)?;
            policy
                .league(realm, &snapshot.league)
                .map(|l| l.characters.is_some())
        });
    let Some(characters_covered) = covered else {
        return PlanNotes::default();
    };
    let no_characters_listed = characters_covered
        && snapshot.character_listing.is_some()
        && snapshot.characters.is_empty();
    PlanNotes {
        league: league_notes(
            [snapshot.league.as_str()],
            snapshot.league_list.as_ref(),
            now,
        )
        .pop()
        .filter(|note| note.list_fetched_at.is_some()),
        no_characters_listed,
        character_listing_fetched_at_age: snapshot
            .character_listing
            .filter(|_| no_characters_listed)
            .map(|l| acquisition_store::age::ago(now, Some(l.fetched_at))),
    }
}

/// What a sync-policy write replaces (C36, amended 2026-10-19).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyWrite {
    /// The realms the value names, each whole; a realm it names with no
    /// leagues is removed, stored or not, and a realm it does not name
    /// stays. The first write, with nothing stored, is stored as written
    /// unless it names an empty realm.
    Merge,
    /// The whole policy: what the value does not name is gone.
    Replace,
}

/// Why a sync-policy write was refused: the value is not a policy this
/// build compiles, the stored policy cannot be merged into, or the
/// store's compare-and-swap (or the store itself) refused the put. Split
/// so a caller can render a CAS conflict — which carries the current row
/// to re-read — differently from a typo.
#[derive(Debug)]
pub enum PutPolicyError {
    Invalid(PlanError),
    /// The stored policy is not one this build reads (a newer build's),
    /// so nothing can be merged into it; a whole replacement still can.
    StoredUnreadable(PlanError),
    Store(AnnotationError),
}

impl std::fmt::Display for PutPolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PutPolicyError::Invalid(e) => write!(f, "{e}"),
            PutPolicyError::StoredUnreadable(e) => write!(
                f,
                "the stored sync policy cannot be read by this build ({e}), so the realms \
                 written cannot be merged into it: write the whole policy instead (`--replace`; on the \
                 MCP, `replace: true`)"
            ),
            // the policy's words, never the store's (C116)
            PutPolicyError::Store(AnnotationError::Conflict { current: Some(row) }) => write!(
                f,
                "the sync policy is at revision {} (re-read and retry)",
                row.revision
            ),
            PutPolicyError::Store(AnnotationError::Conflict { current: None }) => {
                write!(
                    f,
                    "no sync policy is stored at that revision (re-read and retry)"
                )
            }
            PutPolicyError::Store(AnnotationError::RevisionExhausted { revision, .. }) => write!(
                f,
                "the sync policy is at revision {revision}, the last a row can take: nothing was written"
            ),
            PutPolicyError::Store(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for PutPolicyError {}

/// Write the sync policy, validated first — the one shared write path for
/// every frontend's policy surface. Validation precedes the put on
/// purpose: the policy is intent, and the planner refuses a typo'd or
/// newer-versioned value on every parse — storing one anyway would just
/// move that error to plan time, with the typo now on disk.
///
/// `expected_revision` is the store's compare-and-swap, verbatim:
/// `Some(r)` replaces exactly the revision the caller reviewed, `None`
/// creates (refused if a policy exists). A frontend that wants a softer
/// default (the CLI's "replace whatever is stored") reads the current
/// revision itself and passes it here — the blind form is a frontend
/// policy, not this function's. `provenance` names the frontend (C65).
///
/// `write` is what the value replaces (C36): under
/// [`PolicyWrite::Merge`] the stored policy is read and the realms the
/// value names replace its own, so a poe2 policy written alone leaves
/// the pc one standing. The merge is made from the row at exactly
/// `expected_revision`, read once and checked as a price write's is
/// ([`Annotations::at_revision`]: any other stored revision is the same
/// conflict the put would raise), and lands in one put, so the revision
/// advances once per write; a create reads nothing ([`put_sync_policy_over`]).
pub fn put_sync_policy(
    annotations: &mut Annotations,
    value: &Value,
    expected_revision: Option<i64>,
    write: PolicyWrite,
    provenance: &Provenance,
) -> Result<AnnotationRow, PutPolicyError> {
    let prior = expected_revision
        .map(|r| annotations.at_revision(SYNC_POLICY_SCOPE, SYNC_POLICY_KEY, SYNC_POLICY_KIND, r))
        .transpose()
        .map_err(PutPolicyError::Store)?;
    put_sync_policy_over(annotations, value, prior.as_ref(), write, provenance)
}

/// [`put_sync_policy`] over the row it replaces, already read and checked
/// by the caller — `prior`, the stored row at the revision the write
/// names, or none for a create — so one read serves the check, the merge
/// and a receipt's `prior` (round 5 of the 2g build, the second cold
/// review's finding 1): the put names `prior`'s revision, so a row that
/// moved since the read is the conflict, never a stale prior reported as
/// replaced. A create merges into nothing and the put refuses it over a
/// live row, carrying that row.
pub fn put_sync_policy_over(
    annotations: &mut Annotations,
    value: &Value,
    prior: Option<&AnnotationRow>,
    write: PolicyWrite,
    provenance: &Provenance,
) -> Result<AnnotationRow, PutPolicyError> {
    let expected_revision = prior.map(|row| row.revision);
    let merged;
    let value = match write {
        PolicyWrite::Replace => value,
        PolicyWrite::Merge => {
            let mut written = SyncPolicy::from_value(value).map_err(PutPolicyError::Invalid)?;
            let policy = match prior {
                // Nothing to merge into: the put creates, or refuses a
                // revision that names nothing, as it always has. A realm
                // named with no leagues goes here as it does from a merge;
                // a policy naming none is stored as written.
                None => {
                    let named = written.realms.len();
                    written.realms.retain(|_, entry| !entry.leagues.is_empty());
                    (written.realms.len() != named).then_some(written)
                }
                Some(row) => {
                    let mut policy = SyncPolicy::from_value(&row.value)
                        .map_err(PutPolicyError::StoredUnreadable)?;
                    for (realm, entry) in written.realms {
                        if entry.leagues.is_empty() {
                            policy.realms.remove(&realm);
                        } else {
                            policy.realms.insert(realm, entry);
                        }
                    }
                    Some(policy)
                }
            };
            match policy {
                None => value,
                Some(policy) => {
                    merged = serde_json::to_value(&policy).map_err(|e| {
                        PutPolicyError::Invalid(PlanError::MalformedPolicy {
                            detail: e.to_string(),
                        })
                    })?;
                    &merged
                }
            }
        }
    };
    annotations
        .put::<SyncPolicy>(
            SYNC_POLICY_SCOPE,
            SYNC_POLICY_KEY,
            value,
            expected_revision,
            provenance,
        )
        .map_err(|e| match e {
            AnnotationError::Invalid(v) => PutPolicyError::Invalid(v.into()),
            other => PutPolicyError::Store(other),
        })
}

/// Why the plan re-lists the league.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ListingReason {
    NeverListed,
    Stale {
        age_seconds: i64,
    },
    /// A selection plan's listing (C76): the selection is read from the
    /// listing, which every run refreshes, so a tab added since is seen.
    Selected,
}

/// Why the plan fetches a tab or a character. The first two arms are
/// facet-blind; the disagreement arms are each facet's own evidence that a
/// listing newer than our fetch proves change (never that nothing changed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum FetchReason {
    NeverFetched,
    /// A selection plan names it (C44 as amended): fetched however fresh,
    /// since it was asked for by name.
    Selected,
    Stale {
        age_seconds: i64,
    },
    /// The tab arm: a listing newer than our fetch reported a different
    /// item count, which proves the tab changed (`listed` is the listing's
    /// count, `held` what the store holds from the last fetch).
    ListedCountDisagrees {
        listed: i64,
        held: i64,
    },
    /// The character arm: a listing newer than our fetch reported a
    /// different `experience` — monotone for a character's life, so a
    /// difference proves play since (`held` is the fetched body's).
    ListedExperienceDisagrees {
        listed: i64,
        held: i64,
    },
    /// The character arm's sibling: the listing's league differs from the
    /// fetched body's — a Hardcore death landing in Standard.
    ListedLeagueDisagrees {
        listed: String,
        held: String,
    },
}

/// Why a covered tab is not in the action set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SkipReason {
    /// Fetched within the policy's window and nothing proved a change.
    Fresh { age_seconds: i64 },
    /// Folders hold tabs, never items, and are never fetched — their
    /// children are covered individually.
    Folder,
    /// The league has never been listed, so the plan is the listing
    /// alone: membership is unconfirmed, and every fetch waits for the
    /// facts the listing lands (D5a's eventual reconciliation).
    AwaitingListing,
    /// The tab's recorded parent is no longer on record (the store keeps
    /// substash rows when a listing retires their parent), so the fetch
    /// path under it cannot be rendered with confidence. The next listing
    /// or parent re-fetch reconciles it.
    OrphanedParent { parent: String },
    /// A map/unique substash whose stub reports 0 items while the store
    /// holds none: there is nothing to fetch. A guard — GGG appears to list
    /// only non-empty substashes — not a freshness claim (a count never
    /// proves a tab unchanged; a held item against a 0 count is the
    /// disagreement arm, which fetches).
    EmptyStub,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkippedTab {
    pub id: String,
    pub name: String,
    pub reason: SkipReason,
}

/// Why a covered character is not in the action set. Its own enum: a
/// `folder` reason on a character is malformed, and the strict parse
/// should say so.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum CharacterSkipReason {
    /// Fetched within the policy's window and nothing proved a change.
    Fresh { age_seconds: i64 },
    /// The realm's characters were never listed, so the plan is the
    /// listing alone — membership and addresses are unconfirmed.
    AwaitingListing,
    /// The listing gave this character no league, so no (realm, league)
    /// policy can cover it; reported by every league plan of the realm
    /// (C61: uncovered, never a failure).
    NoLeague,
    /// The listing flags it deleted: not fetched — a 404 hunt counts
    /// against the invalid-request threshold.
    Deleted,
    /// The listing flags it expired: not fetched, same reasoning.
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkippedCharacter {
    pub id: String,
    pub name: String,
    pub reason: CharacterSkipReason,
}

/// One authorized request. Self-contained on purpose: an action can be
/// rendered, reviewed, or turned into a daemon job without the envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum RefreshAction {
    ListStashes {
        realm: Realm,
        league: String,
        reason: ListingReason,
    },
    FetchTab {
        realm: Realm,
        league: String,
        id: String,
        name: String,
        tab_type: String,
        reason: FetchReason,
    },
    /// A map/unique substash, fetched under its parent tab. Its stub is
    /// already on record — v1 plans never fan out dynamically.
    FetchSubstash {
        realm: Realm,
        league: String,
        parent: String,
        id: String,
        name: String,
        tab_type: String,
        reason: FetchReason,
    },
    /// The realm's character list — realm-wide (the endpoint has no
    /// league), so it carries none and sits in any league's plan.
    ListCharacters { realm: Realm, reason: ListingReason },
    /// One character: `id` is its identity (coverage, reasons, the store
    /// row), `name` the address the request takes — the listed name, the
    /// one combination guaranteed to fetch. A name that moved fails its
    /// child honestly or lands a different id; the next listing
    /// reconciles (C55).
    FetchCharacter {
        realm: Realm,
        league: String,
        id: String,
        name: String,
        reason: FetchReason,
    },
}

impl RefreshAction {
    /// The realm this action touches — validated against the envelope's
    /// on deserialization.
    pub fn realm(&self) -> Realm {
        match self {
            RefreshAction::ListStashes { realm, .. }
            | RefreshAction::FetchTab { realm, .. }
            | RefreshAction::FetchSubstash { realm, .. }
            | RefreshAction::ListCharacters { realm, .. }
            | RefreshAction::FetchCharacter { realm, .. } => *realm,
        }
    }

    /// The league this action touches — validated against the envelope's
    /// on deserialization. `None` for a realm-wide action (the character
    /// list), which is in-envelope for any league of its realm.
    pub fn league(&self) -> Option<&str> {
        match self {
            RefreshAction::ListStashes { league, .. }
            | RefreshAction::FetchTab { league, .. }
            | RefreshAction::FetchSubstash { league, .. }
            | RefreshAction::FetchCharacter { league, .. } => Some(league),
            RefreshAction::ListCharacters { .. } => None,
        }
    }

    /// The action in the daemon's job vocabulary: `(kind, params)` exactly
    /// as `Submit` wants them. The realm is always explicit — pc included
    /// — so a tuple says where it goes without a decode default; `deep`
    /// is always false — a plan's actions are the reviewed set, and a
    /// fetch that fanned out would expand it.
    pub fn job(&self) -> (&'static str, Value) {
        match self {
            RefreshAction::ListStashes { realm, league, .. } => {
                ("stashes", json!({ "realm": realm, "league": league }))
            }
            RefreshAction::ListCharacters { realm, .. } => {
                ("characters", json!({ "realm": realm }))
            }
            RefreshAction::FetchCharacter { realm, name, .. } => {
                ("character", json!({ "realm": realm, "name": name }))
            }
            RefreshAction::FetchTab {
                realm, league, id, ..
            } => (
                "stash",
                json!({ "realm": realm, "league": league, "id": id, "deep": false }),
            ),
            RefreshAction::FetchSubstash {
                realm,
                league,
                parent,
                id,
                ..
            } => (
                "stash",
                json!({ "realm": realm, "league": league, "id": parent, "sub": id, "deep": false }),
            ),
        }
    }
}

/// What the plan was derived from — enough for apply-time checks (the
/// snapshot carries the revision so the comparison is possible) and for a
/// human to see how old the inputs were.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanBasis {
    /// When the snapshot was taken (facts as of this read).
    pub snapshot_taken_at: i64,
    /// The highest response the snapshot was read under (C111): an action
    /// is spent by a fetch that landed in a later response, never by the
    /// clock.
    pub snapshot_response: i64,
    /// The stash listing the tab set derives from; `None` when the league
    /// was never listed (then the listing is the tab facet's only action).
    /// Carried whenever the snapshot had one, whether or not the policy
    /// covers tabs — the basis is the facts as of the read.
    pub stash_listing: Option<ListingBasis>,
    /// The realm's character listing the character set derives from;
    /// `None` when the realm was never listed. Same carrying rule.
    pub character_listing: Option<ListingBasis>,
    /// The sync-policy annotation revision a policy plan was compiled
    /// from, which apply checks against the current row (C44); none for a
    /// selection plan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_revision: Option<i64>,
    /// The explicit selection a selection plan was compiled from: its
    /// authorization, in place of a policy revision (C44 as amended); none
    /// for a policy plan. Exactly one of the two is present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selection: Option<PlanSelection>,
}

/// An explicit selection, as a person ticks it or names it on a command
/// line: tabs by id, or every tab the listing names; characters by id;
/// and whether a ticked map or unique tab's pages are fetched with it.
/// Freshness never skips what a selection names: it is asked for by name
/// (C111).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanSelection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tabs: Option<Selection>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characters: Vec<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub deep: bool,
}

/// The coarse wire projection. The range covers the authorized requests
/// and their bounded 429 retries; `prerequisites` names sends the range
/// deliberately does not count — a precise accounting is the deferred
/// wire-budget feature, and pretending to one here would be false. The
/// whole of it, prerequisites included, is part of the reviewed
/// projection: deserialization recomputes it (`wire_estimate`) and
/// refuses a mismatch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WireEstimate {
    pub min: u64,
    pub max: u64,
    pub prerequisites: Vec<String>,
}

/// The one place the wire projection is computed — the compiler builds
/// it and the deserialization validator compares against it, so the two
/// cannot drift. Any change here (the retry bound, the prerequisite
/// wording) changes what serialized schema-1 plans claim, so it is a
/// plan-schema event, not a silent edit.
fn wire_estimate(logical_requests: u64) -> WireEstimate {
    WireEstimate {
        min: logical_requests,
        max: logical_requests * (1 + u64::from(MAX_429_RETRIES)),
        prerequisites: if logical_requests == 0 {
            Vec::new()
        } else {
            vec![
                "a HEAD probe on first contact with each route this daemon lifetime".into(),
                "an OAuth token refresh if the access token has expired".into(),
            ]
        },
    }
}

/// A refresh authorization: the bounded work the user reviewed, derived
/// from one snapshot of facts + intent. Immutable by convention — new
/// facts produce a new plan, never an edit — and serializable so it can
/// cross a socket, land in a journal, or be read back by apply.
///
/// Deserialization validates: every path funnels through
/// `TryFrom<RefreshPlanWire>`, which refuses unknown fields, a schema
/// stamp other than [`REFRESH_PLAN_SCHEMA`], an operation other than
/// `"refresh"`, an action whose league is not the envelope's, and
/// derived counts that do not recompute — so a tampered or hand-built
/// envelope will not parse into this type. [`RefreshPlan::from_value`]
/// is the friendly entry (a newer schema reports as such).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RefreshPlanWire")]
pub struct RefreshPlan {
    /// [`REFRESH_PLAN_SCHEMA`].
    pub plan_schema: i64,
    /// Always `"refresh"`; the envelope names its operation so a reader
    /// of the serialized form needs no out-of-band context.
    pub operation: String,
    /// Bound by the caller from the provider directory it opened — the
    /// snapshot deliberately does not carry it.
    pub provider: String,
    pub account_uuid: String,
    pub account_name: Option<String>,
    /// The coordinate above league; every action is on this realm.
    pub realm: Realm,
    pub league: String,
    pub generated_at: i64,
    pub basis: PlanBasis,
    /// The freshness assumption applied: facts younger than this (with no
    /// count disagreement) were left alone.
    pub max_age_seconds: u32,
    /// The explicit action set. Applying executes exactly this or a
    /// strict subset.
    pub actions: Vec<RefreshAction>,
    /// Covered tabs left out, each with why — the plan shows its
    /// reasoning, not just its conclusions.
    pub skipped_tabs: Vec<SkippedTab>,
    /// Tab ids the policy names that the facts on record do not: vanished
    /// tabs (or typos). Reported, never invented into actions.
    pub unknown_tabs: Vec<String>,
    /// Covered characters left out, each with why.
    pub skipped_characters: Vec<SkippedCharacter>,
    /// Character ids the policy names that the facts on record do not.
    pub unknown_characters: Vec<String>,
    /// Exact for a refresh: one logical request per action.
    pub logical_requests: u64,
    pub wire_sends: WireEstimate,
    /// Optional enrichment: the daemon's quote for this plan's actions,
    /// with its own observation time inside. An observation, not a
    /// derivation — it cannot be recomputed at parse, only carried — so
    /// validation pins what *is* checkable: the plan's provider, exactly
    /// its account, the quote's echoed `work` being exactly this plan's
    /// actions as job tuples, and scope totals that sum (checked) to the
    /// logical bound (`check_quote_matches`). Compiling never fills it
    /// (a plan needs no daemon); [`RefreshPlan::with_quote`] attaches
    /// one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<Quote>,
}

/// What a carried quote must have in common with its envelope. The quote
/// is an observation and cannot be recomputed, but it must be *about*
/// this plan: the same provider; exactly the plan's account (`None`
/// matches only a plan bound to no account — an accountless quote on an
/// account-bound plan is a quote for someone else's limiter state); the
/// quote's echoed `work` must be exactly the plan's actions rendered as
/// job tuples, in order — the verifiable work basis, where matching
/// totals alone would let a quote for other work of the same size stand
/// in; and the scope totals must still sum (checked — a malformed
/// envelope must not wrap past validation) to the plan's logical bound.
/// A mismatch on any of these would mislead exactly the review the plan
/// exists for.
fn check_quote_matches(
    quote: &Quote,
    provider: &str,
    account_name: Option<&str>,
    actions: &[RefreshAction],
) -> Result<(), String> {
    if quote.provider != provider {
        return Err(format!(
            "the quote projects provider {:?}, but the plan is for {provider:?}",
            quote.provider
        ));
    }
    if quote.account.as_deref() != account_name {
        return Err(format!(
            "the quote projects account {:?}, but the plan is for {:?}",
            quote.account, account_name
        ));
    }
    if quote.work.len() != actions.len() {
        return Err(format!(
            "the quote projects {} job(s), but the plan authorizes {} action(s)",
            quote.work.len(),
            actions.len()
        ));
    }
    for (i, (job, action)) in quote.work.iter().zip(actions).enumerate() {
        let (kind, params) = action.job();
        if job.kind != kind || job.params != params {
            return Err(format!(
                "quoted action {i} ({} {}) is not the plan's action {i} ({kind} {params})",
                job.kind, job.params
            ));
        }
    }
    let quoted = quote
        .scopes
        .iter()
        .try_fold(0u64, |sum, s| sum.checked_add(s.requests))
        .ok_or_else(|| "the quote's scope request total overflows".to_string())?;
    if quoted != actions.len() as u64 {
        return Err(format!(
            "the quote's scopes project {quoted} request(s), but the plan authorizes {}",
            actions.len()
        ));
    }
    Ok(())
}

impl RefreshPlan {
    /// Enrich the plan with the daemon's quote (decided 2026-08-31: a plan
    /// optionally carries one, with its own observation time). Consuming
    /// on purpose — the enriched plan is a new value, not an edit of a
    /// reviewed one.
    pub fn with_quote(mut self, quote: Quote) -> Result<RefreshPlan, PlanError> {
        check_quote_matches(
            &quote,
            &self.provider,
            self.account_name.as_deref(),
            &self.actions,
        )
        .map_err(|detail| PlanError::MalformedPlan { detail })?;
        self.quote = Some(quote);
        Ok(self)
    }
}

/// Why a plan must not be spent — the step-7 staleness/identity gate
/// (C44), shared by every frontend's apply
/// surface. A plan is authorization *derived from intent at a revision*;
/// intent edited since revokes the derivation, and a plan for another
/// identity is never spent here. Fact drift deliberately does not refuse:
/// the authorization is the bounded action set, not a world-state
/// assertion, and the next plan reconciles.
#[derive(Debug)]
pub enum SpendError {
    /// The plan names a different provider than this frontend runs against.
    WrongProvider { plan: String, running: String },
    /// The plan names a different account uuid than the selected account
    /// (`None`: the selected account has no recorded uuid at all).
    WrongAccount {
        plan_uuid: String,
        selected_uuid: Option<String>,
    },
    /// The sync policy the plan derives from no longer exists.
    PolicyGone { plan_revision: i64 },
    /// The sync policy moved since the plan was compiled.
    PolicyMoved {
        plan_revision: i64,
        stored_revision: i64,
    },
    /// The policy row could not be read — an error, never "no policy".
    Annotations(AnnotationError),
}

impl std::fmt::Display for SpendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpendError::WrongProvider { plan, running } => write!(
                f,
                "the plan is for provider {plan:?}, but this frontend runs against {running:?}"
            ),
            SpendError::WrongAccount {
                plan_uuid,
                selected_uuid,
            } => write!(
                f,
                "the plan is for account uuid {plan_uuid}, but the selected account is {} — \
                 replan as the right account",
                selected_uuid.as_deref().unwrap_or("unmapped")
            ),
            SpendError::PolicyGone { plan_revision } => write!(
                f,
                "the sync policy is gone (the plan cites revision {plan_revision}); \
                 declare one and replan"
            ),
            SpendError::PolicyMoved {
                plan_revision,
                stored_revision,
            } => write!(
                f,
                "the sync policy moved: the plan cites revision {plan_revision}, but revision \
                 {stored_revision} is stored — review and replan"
            ),
            SpendError::Annotations(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for SpendError {}

impl RefreshPlan {
    /// The apply-time gate, run frontend-side immediately before submit
    /// (the daemon is intent-blind, so only a frontend can compare):
    /// this plan may be spent only against the provider it names, the
    /// account uuid it was derived for, and the sync-policy revision it
    /// cites — checked against a fresh read of the stored row. The check
    /// races a concurrent policy write between read and submit; that is
    /// the accepted human-boundary residual, same register as an
    /// unconditional `policy set`.
    pub fn check_spendable(
        &self,
        provider: &str,
        selected_uuid: Option<&str>,
        annotations: &Annotations,
    ) -> Result<(), SpendError> {
        if self.provider != provider {
            return Err(SpendError::WrongProvider {
                plan: self.provider.clone(),
                running: provider.into(),
            });
        }
        if selected_uuid != Some(self.account_uuid.as_str()) {
            return Err(SpendError::WrongAccount {
                plan_uuid: self.account_uuid.clone(),
                selected_uuid: selected_uuid.map(String::from),
            });
        }
        // C44 as amended: a selection plan's authorization is the
        // selection itself, and no policy revision is checked
        let Some(plan_revision) = self.basis.policy_revision else {
            return Ok(());
        };
        match annotations
            .get(SYNC_POLICY_SCOPE, SYNC_POLICY_KEY, SYNC_POLICY_KIND)
            .map_err(SpendError::Annotations)?
        {
            None => Err(SpendError::PolicyGone { plan_revision }),
            Some(row) if row.revision != plan_revision => Err(SpendError::PolicyMoved {
                plan_revision,
                stored_revision: row.revision,
            }),
            Some(_) => Ok(()),
        }
    }

    /// C111: the actions an apply may still send, and the ones already
    /// spent — in the plan's order. An action is spent when its target's
    /// latest fetch in `snapshot` (the plan's realm and league, read now)
    /// landed in a response after the one the plan's snapshot was read
    /// under (`basis.snapshot_response`, the response watermark) — by
    /// response, never the clock, so a fetch in the snapshot's own second
    /// but before it leaves the action unspent, and a replay within one
    /// second sends nothing; or when `live` holds a waiting or running job
    /// of `account` for the same request: the same `Endpoint`, as the
    /// store decodes a job, so a door that spells the request differently
    /// (`acq stash` sends `"sub": null`, a deep fetch `"deep": true`)
    /// still spends it. A waiting or running apply of `account` holds the
    /// requests its `params.jobs` lists, before its fan-out has submitted
    /// them: a retry after an apply was accepted sends nothing it holds. A
    /// child is named before an apply that lists the same request — the
    /// job that fetches it. A spent entry names the time of the response
    /// it was spent by. A target never fetched since the snapshot is
    /// unspent, whether or not the snapshot still holds it: its fetch
    /// fails its own child honestly (C44).
    pub fn unspent(
        &self,
        snapshot: &RefreshSnapshot,
        live: &[JobInfo],
        account: &str,
    ) -> (Vec<RefreshAction>, Vec<Spent>) {
        // C111: spent by a fetch that landed in a response after the one
        // the plan's snapshot was read under — by response, never the
        // clock; the fetch's time is kept for the words alone
        let since = self.basis.snapshot_response;
        let mut unspent = Vec::new();
        let mut spent = Vec::new();
        for action in &self.actions {
            // the response each target's fetch landed in, with that
            // response's own time for the words: a listing's from its
            // basis, a fetch's from the response rows, never the target's
            // current location, which a retired row no longer carries
            let landed = match action {
                RefreshAction::ListStashes { .. } => {
                    snapshot.stash_listing.as_ref().map(|l| Fetched {
                        response_id: l.response_id,
                        fetched_at: l.fetched_at,
                    })
                }
                RefreshAction::ListCharacters { .. } => {
                    snapshot.character_listing.as_ref().map(|l| Fetched {
                        response_id: l.response_id,
                        fetched_at: l.fetched_at,
                    })
                }
                RefreshAction::FetchTab { .. }
                | RefreshAction::FetchSubstash { .. }
                | RefreshAction::FetchCharacter { .. } => {
                    let (kind, params) = action.job();
                    Endpoint::from_job(kind, &params).and_then(|e| snapshot.fetched_in(&e))
                }
            };
            let fetched = landed
                .filter(|f| f.response_id > since)
                .map(|f| f.fetched_at);
            let (kind, params) = action.job();
            let request = Endpoint::from_job(kind, &params);
            let open = || {
                live.iter().filter(|job| {
                    request.is_some()
                        && !job.state.is_terminal()
                        && job.account.as_deref() == Some(account)
                })
            };
            let holds = |job: &&JobInfo| Endpoint::from_job(&job.kind, &job.params) == request;
            let lists = |job: &&JobInfo| {
                job.kind == "apply"
                    && job.params["jobs"].as_array().is_some_and(|jobs| {
                        jobs.iter().any(|child| {
                            child["kind"].as_str().is_some_and(|kind| {
                                Endpoint::from_job(kind, &child["params"]) == request
                            })
                        })
                    })
            };
            let queued = || open().find(holds).or_else(|| open().find(lists));
            let by = match fetched {
                Some(fetched_at) => Some(SpentBy::Fetched {
                    fetched_at,
                    fetched_at_age: None,
                }),
                None => queued().map(|job| SpentBy::Queued {
                    job_id: job.id,
                    parent: job.parent,
                }),
            };
            match by {
                Some(by) => spent.push(Spent {
                    action: action.clone(),
                    by,
                }),
                None => unspent.push(action.clone()),
            }
        }
        (unspent, spent)
    }
}

/// An action of a plan that an apply did not send, and what spent it
/// (C111). Serialized as the action's own object beside `fetched_at`, or
/// beside `job_id` (and `parent`, the apply that queued it).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Spent {
    pub action: RefreshAction,
    #[serde(flatten)]
    pub by: SpentBy,
}

impl Spent {
    /// C53: a fetch that spent the request said as an age at `now`, the
    /// answer's time, as the apply's text says the latest.
    pub fn aged(mut self, now: i64) -> Spent {
        if let SpentBy::Fetched {
            fetched_at,
            fetched_at_age,
        } = &mut self.by
        {
            *fetched_at_age = Some(acquisition_store::age::ago(now, Some(*fetched_at)));
        }
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum SpentBy {
    /// The target was fetched after the plan's snapshot; `fetched_at_age`
    /// says it in words at the answer's time ([`Spent::aged`], C53).
    Fetched {
        fetched_at: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        fetched_at_age: Option<String>,
    },
    /// The same request is waiting or running in the daemon.
    Queued {
        job_id: JobId,
        #[serde(skip_serializing_if = "Option::is_none")]
        parent: Option<JobId>,
    },
}

/// Actions rendered as the `apply` parent's params — the explicit
/// `(kind, params)` child tuples the daemon admits or refuses whole at
/// submit, plus the caller's logical budget. One rendering for every
/// frontend, over the plan's unspent actions (C111), so what apply
/// submits cannot drift from what the plan authorized.
pub fn apply_params_of(actions: &[RefreshAction], max_requests: Option<u64>) -> Value {
    let jobs: Vec<Value> = actions
        .iter()
        .map(|action| {
            let (kind, params) = action.job();
            json!({ "kind": kind, "params": params })
        })
        .collect();
    let mut params = json!({ "jobs": jobs });
    if let Some(max) = max_requests {
        params["max_requests"] = json!(max);
    }
    params
}

impl RefreshPlan {
    /// Parse a serialized plan back into a trusted envelope. The schema
    /// stamp is checked before the shape so a genuinely newer plan
    /// reports [`PlanError::PlanSchemaUnsupported`], not a spurious
    /// unknown-field complaint; everything else is the `TryFrom`
    /// validation below.
    pub fn from_value(value: &Value) -> Result<RefreshPlan, PlanError> {
        let found = value
            .get("plan_schema")
            .and_then(Value::as_i64)
            .ok_or_else(|| PlanError::MalformedPlan {
                detail: "missing integer `plan_schema`".into(),
            })?;
        if found != REFRESH_PLAN_SCHEMA {
            return Err(PlanError::PlanSchemaUnsupported {
                found,
                supported: REFRESH_PLAN_SCHEMA,
            });
        }
        let plan: RefreshPlan =
            serde_json::from_value(value.clone()).map_err(|e| PlanError::MalformedPlan {
                detail: e.to_string(),
            })?;
        // "Unknown fields at any depth are refused" has one hole serde
        // cannot close: an internally tagged *unit* variant (`never_fetched`,
        // `folder`, `deleted`, …) ignores extra fields beside its `kind`,
        // `deny_unknown_fields` or not. So the derived part of the envelope
        // must be byte-for-byte what this build would write for it: the
        // parsed plan re-serializes to exactly the input (the quote aside —
        // it is an observation with its own strict shape, never rewritten
        // here). A field smuggled into a reason or a skip fails this even
        // though the type accepted it.
        let mut given = value.clone();
        if let Some(o) = given.as_object_mut() {
            o.remove("quote");
        }
        let expected = serde_json::to_value(RefreshPlan {
            quote: None,
            ..plan.clone()
        })
        .map_err(|e| PlanError::MalformedPlan {
            detail: e.to_string(),
        })?;
        if given != expected {
            return Err(PlanError::MalformedPlan {
                detail: "the envelope carries content this build would not write (a field on a \
                         reason or skip entry this build does not know, or a field it always \
                         writes left out)"
                    .into(),
            });
        }
        Ok(plan)
    }
}

/// The raw JSON shape of a plan; deserialization validates through the
/// `TryFrom` below on every path.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RefreshPlanWire {
    plan_schema: i64,
    operation: String,
    provider: String,
    account_uuid: String,
    account_name: Option<String>,
    realm: Realm,
    league: String,
    generated_at: i64,
    basis: PlanBasis,
    max_age_seconds: u32,
    actions: Vec<RefreshAction>,
    skipped_tabs: Vec<SkippedTab>,
    unknown_tabs: Vec<String>,
    skipped_characters: Vec<SkippedCharacter>,
    unknown_characters: Vec<String>,
    logical_requests: u64,
    wire_sends: WireEstimate,
    #[serde(default)]
    quote: Option<Quote>,
}

impl TryFrom<RefreshPlanWire> for RefreshPlan {
    type Error = String;
    fn try_from(wire: RefreshPlanWire) -> Result<Self, String> {
        if wire.plan_schema != REFRESH_PLAN_SCHEMA {
            return Err(format!(
                "refresh plan declares schema {}, not this build's v{REFRESH_PLAN_SCHEMA}",
                wire.plan_schema
            ));
        }
        if wire.operation != "refresh" {
            return Err(format!(
                "a RefreshPlan's operation must be \"refresh\", not {:?}",
                wire.operation
            ));
        }
        // C44 as amended: a plan is authorized by a policy revision or by
        // an explicit selection, exactly one
        match (&wire.basis.policy_revision, &wire.basis.selection) {
            (Some(_), None) | (None, Some(_)) => {}
            (Some(_), Some(_)) => {
                return Err(
                    "a plan's basis names a policy revision or a selection, never both".into(),
                );
            }
            (None, None) => {
                return Err(
                    "a plan's basis names neither a policy revision nor a selection".into(),
                );
            }
        }
        // Every action is on the envelope's realm, and every league-bearing
        // action on its league; a realm-wide action (the character list)
        // carries no league and is in-envelope for any league of its realm.
        if let Some(stray) = wire
            .actions
            .iter()
            .find(|a| a.realm() != wire.realm || a.league().is_some_and(|l| l != wire.league))
        {
            return Err(format!(
                "action for {}/{:?} inside a plan for {}/{:?}",
                stray.realm(),
                stray.league().unwrap_or("<realm-wide>"),
                wire.realm,
                wire.league
            ));
        }
        // The derived quantities are recomputed, never taken on faith:
        // admission-time budgeting (D8) trusts `logical_requests`, and a
        // forged range or prerequisite list would mislead the review the
        // plan exists for. `wire_estimate` is the same function the
        // compiler used, so nothing here can drift from it.
        let logical = wire.actions.len() as u64;
        if wire.logical_requests != logical {
            return Err(format!(
                "logical_requests says {} but the plan lists {logical} actions",
                wire.logical_requests
            ));
        }
        let expected = wire_estimate(logical);
        if wire.wire_sends != expected {
            return Err(format!(
                "wire_sends does not recompute from {logical} actions \
                 (got {}..{} with {} prerequisites, expected {}..{} with {})",
                wire.wire_sends.min,
                wire.wire_sends.max,
                wire.wire_sends.prerequisites.len(),
                expected.min,
                expected.max,
                expected.prerequisites.len()
            ));
        }
        // A carried quote is an observation and cannot recompute, but it
        // must be *about* this plan — provider, account, and the plan's
        // own actions as its work basis.
        if let Some(quote) = &wire.quote {
            check_quote_matches(
                quote,
                &wire.provider,
                wire.account_name.as_deref(),
                &wire.actions,
            )?;
        }
        Ok(RefreshPlan {
            plan_schema: wire.plan_schema,
            operation: wire.operation,
            provider: wire.provider,
            account_uuid: wire.account_uuid,
            account_name: wire.account_name,
            realm: wire.realm,
            league: wire.league,
            generated_at: wire.generated_at,
            basis: wire.basis,
            max_age_seconds: wire.max_age_seconds,
            actions: wire.actions,
            skipped_tabs: wire.skipped_tabs,
            unknown_tabs: wire.unknown_tabs,
            skipped_characters: wire.skipped_characters,
            unknown_characters: wire.unknown_characters,
            logical_requests: wire.logical_requests,
            wire_sends: wire.wire_sends,
            quote: wire.quote,
        })
    }
}

/// Compile the snapshot's own sync-policy row into a plan. The one way
/// in: plans always derive from stored intent and carry its revision —
/// an ad-hoc selection that bypasses the policy waits for a consumer
/// that demonstrably needs it.
pub fn plan_refresh(
    provider: &str,
    snapshot: &RefreshSnapshot,
    now: i64,
) -> Result<RefreshPlan, PlanError> {
    let row = snapshot.policy.as_ref().ok_or(PlanError::NoSyncPolicy)?;
    let policy = SyncPolicy::from_value(&row.value)?;
    compile(provider, snapshot, &policy, row.revision, now)
}

/// C44 as amended: a plan compiled from an explicit selection over the
/// same snapshot a policy plan reads, with no stored policy. Each named
/// tab and character is fetched however fresh it is, since it is asked
/// for by name; under `deep`, the pages on record of each named map or
/// unique tab are fetched with it (a never-fetched one has none on record
/// yet: they arrive with its fetch, for the next plan). The listing a
/// selection is read from is refreshed on every run (C76): never listed,
/// it is listed first and the fetches wait; listed, it is fetched beside
/// them, and a tab it newly lists takes the next plan. A folder and an
/// empty page stub are skipped, and an id the facts lack is unknown. The
/// basis carries the selection in place of a revision.
pub fn plan_selection(
    provider: &str,
    snapshot: &RefreshSnapshot,
    selection: &PlanSelection,
    now: i64,
) -> Result<RefreshPlan, PlanError> {
    let realm = Realm::parse(&snapshot.realm).ok_or_else(|| PlanError::UnknownRealm {
        realm: snapshot.realm.clone(),
    })?;
    let tabs = match &selection.tabs {
        Some(_) if !Family::Stashes.accepts(realm) => {
            return Err(PlanError::TabsNotTaken { realm });
        }
        Some(tabs) => select_tabs(realm, snapshot, tabs, selection.deep),
        None => Facet::empty(),
    };
    let characters = if selection.characters.is_empty() {
        Facet::empty()
    } else {
        select_characters(realm, snapshot, &selection.characters)
    };
    let actions: Vec<RefreshAction> = tabs.actions.into_iter().chain(characters.actions).collect();
    let logical_requests = actions.len() as u64;
    let wire_sends = wire_estimate(logical_requests);
    Ok(RefreshPlan {
        plan_schema: REFRESH_PLAN_SCHEMA,
        operation: "refresh".into(),
        provider: provider.into(),
        account_uuid: snapshot.account_uuid.clone(),
        account_name: snapshot.account_name.clone(),
        realm,
        league: snapshot.league.clone(),
        generated_at: now,
        basis: PlanBasis {
            snapshot_taken_at: snapshot.taken_at,
            snapshot_response: snapshot.response_watermark,
            stash_listing: snapshot.stash_listing,
            character_listing: snapshot.character_listing,
            policy_revision: None,
            selection: Some(selection.clone()),
        },
        max_age_seconds: 0,
        actions,
        skipped_tabs: tabs.skipped,
        unknown_tabs: tabs.unknown,
        skipped_characters: characters.skipped,
        unknown_characters: characters.unknown,
        logical_requests,
        wire_sends,
        quote: None,
    })
}

/// The tab facet of a selection plan: each tab the selection names, and
/// under `deep` each page on record of a named map or unique tab.
fn select_tabs(
    realm: Realm,
    snapshot: &RefreshSnapshot,
    selection: &Selection,
    deep: bool,
) -> Facet<SkippedTab> {
    let mut facet = Facet::empty();
    // C76: the listing the selection is read from is refreshed on every
    // run; never listed, the tabs wait for it, and listed, they are fetched
    // beside it, a tab it newly lists taking the next plan
    let listing_alone = snapshot.stash_listing.is_none();
    facet.actions.push(RefreshAction::ListStashes {
        realm,
        league: snapshot.league.clone(),
        reason: if listing_alone {
            ListingReason::NeverListed
        } else {
            ListingReason::Selected
        },
    });
    let container = |id: &str| {
        snapshot
            .tabs
            .iter()
            .any(|t| t.id == id && matches!(t.r#type.as_str(), "MapStash" | "UniqueStash"))
    };
    for tab in &snapshot.tabs {
        let page_of = tab.parent.as_deref().filter(|p| container(p));
        let named = match page_of {
            // a page is named by its tab, under deep
            Some(parent) => deep && selection.covers_id(parent),
            None => selection.covers_id(&tab.id),
        };
        if !named {
            continue;
        }
        let verdict = if listing_alone {
            Err(SkipReason::AwaitingListing)
        } else if tab.r#type == "Folder" {
            Err(SkipReason::Folder)
        } else if is_empty_substash(snapshot, tab) {
            Err(SkipReason::EmptyStub)
        } else {
            fetch_action(realm, snapshot, tab, FetchReason::Selected)
        };
        match verdict {
            Ok(action) => facet.actions.push(action),
            Err(reason) => facet.skipped.push(SkippedTab {
                id: tab.id.clone(),
                name: tab.name.clone(),
                reason,
            }),
        }
    }
    if let Selection::Ids(ids) = selection {
        facet.unknown = ids
            .iter()
            .filter(|w| !snapshot.tabs.iter().any(|t| &t.id == *w))
            .cloned()
            .collect();
    }
    facet
}

/// The character facet of a selection plan: each character named by id.
fn select_characters(
    realm: Realm,
    snapshot: &RefreshSnapshot,
    ids: &[String],
) -> Facet<SkippedCharacter> {
    let mut facet = Facet::empty();
    // C76, as the tabs' listing: refreshed on every run
    let listing_alone = snapshot.character_listing.is_none();
    facet.actions.push(RefreshAction::ListCharacters {
        realm,
        reason: if listing_alone {
            ListingReason::NeverListed
        } else {
            ListingReason::Selected
        },
    });
    for c in &snapshot.characters {
        if !ids.contains(&c.id) {
            continue;
        }
        let verdict = if listing_alone {
            Err(CharacterSkipReason::AwaitingListing)
        } else if c.league.is_none() {
            Err(CharacterSkipReason::NoLeague)
        } else if listed_flag(c, "deleted") {
            Err(CharacterSkipReason::Deleted)
        } else if listed_flag(c, "expired") {
            Err(CharacterSkipReason::Expired)
        } else {
            Ok(RefreshAction::FetchCharacter {
                realm,
                league: snapshot.league.clone(),
                id: c.id.clone(),
                name: c.name.clone(),
                reason: FetchReason::Selected,
            })
        };
        match verdict {
            Ok(action) => facet.actions.push(action),
            Err(reason) => facet.skipped.push(SkippedCharacter {
                id: c.id.clone(),
                name: c.name.clone(),
                reason,
            }),
        }
    }
    facet.unknown = ids
        .iter()
        .filter(|w| !snapshot.characters.iter().any(|c| &c.id == *w))
        .cloned()
        .collect();
    facet
}

fn compile(
    provider: &str,
    snapshot: &RefreshSnapshot,
    policy: &SyncPolicy,
    policy_revision: i64,
    now: i64,
) -> Result<RefreshPlan, PlanError> {
    let realm = Realm::parse(&snapshot.realm).ok_or_else(|| PlanError::UnknownRealm {
        realm: snapshot.realm.clone(),
    })?;
    let league_policy =
        policy
            .league(realm, &snapshot.league)
            .ok_or_else(|| PlanError::LeagueNotCovered {
                realm,
                league: snapshot.league.clone(),
            })?;
    let max_age = i64::from(league_policy.max_age_seconds);
    // Each facet compiles on its own: an absent facet contributes no
    // listing, no fetches, nothing skipped, nothing unknown. Tab work
    // first, then character work — deterministic, nothing more (the
    // daemon paces the two policies independently anyway).
    let tabs = match &league_policy.tabs {
        Some(selection) => compile_tabs(realm, snapshot, selection, max_age, now),
        None => Facet::empty(),
    };
    let characters = match &league_policy.characters {
        Some(selection) => compile_characters(realm, snapshot, selection, max_age, now),
        None => Facet::empty(),
    };
    let actions: Vec<RefreshAction> = tabs.actions.into_iter().chain(characters.actions).collect();
    let logical_requests = actions.len() as u64;
    let wire_sends = wire_estimate(logical_requests);
    Ok(RefreshPlan {
        plan_schema: REFRESH_PLAN_SCHEMA,
        operation: "refresh".into(),
        provider: provider.into(),
        account_uuid: snapshot.account_uuid.clone(),
        account_name: snapshot.account_name.clone(),
        realm,
        league: snapshot.league.clone(),
        generated_at: now,
        basis: PlanBasis {
            snapshot_taken_at: snapshot.taken_at,
            snapshot_response: snapshot.response_watermark,
            stash_listing: snapshot.stash_listing,
            character_listing: snapshot.character_listing,
            policy_revision: Some(policy_revision),
            selection: None,
        },
        max_age_seconds: league_policy.max_age_seconds,
        actions,
        skipped_tabs: tabs.skipped,
        unknown_tabs: tabs.unknown,
        skipped_characters: characters.skipped,
        unknown_characters: characters.unknown,
        logical_requests,
        wire_sends,
        quote: None,
    })
}

/// One facet's compiled output: its actions (listing first), the covered
/// entities it left out with why, and the policy ids the facts lack.
struct Facet<S> {
    actions: Vec<RefreshAction>,
    skipped: Vec<S>,
    unknown: Vec<String>,
}

impl<S> Facet<S> {
    fn empty() -> Facet<S> {
        Facet {
            actions: Vec::new(),
            skipped: Vec::new(),
            unknown: Vec::new(),
        }
    }
}

/// Whether a facet's listing is wanted, and whether the facet is the
/// listing **alone**: with no basis on record the plan has no membership
/// authority, so every fetch defers to the next plan (D5a's eventual
/// reconciliation) — even entities on record from direct fetches. Ages
/// saturate: a corrupt store timestamp must misread as "very stale",
/// never wrap into "fresh" (and never panic — the no-panic-on-store-rows
/// rule).
fn listing_verdict(basis: Option<&ListingBasis>, max_age: i64, now: i64) -> Option<ListingReason> {
    match basis {
        None => Some(ListingReason::NeverListed),
        Some(basis) => {
            let age = now.saturating_sub(basis.fetched_at).max(0);
            (age > max_age).then_some(ListingReason::Stale { age_seconds: age })
        }
    }
}

fn compile_tabs(
    realm: Realm,
    snapshot: &RefreshSnapshot,
    selection: &Selection,
    max_age: i64,
    now: i64,
) -> Facet<SkippedTab> {
    let mut facet = Facet::empty();
    let listing_alone = snapshot.stash_listing.is_none();
    if let Some(reason) = listing_verdict(snapshot.stash_listing.as_ref(), max_age, now) {
        facet.actions.push(RefreshAction::ListStashes {
            realm,
            league: snapshot.league.clone(),
            reason,
        });
    }
    for tab in &snapshot.tabs {
        if !covers_tab(selection, tab) {
            continue;
        }
        let verdict = if listing_alone {
            Err(SkipReason::AwaitingListing)
        } else if tab.r#type == "Folder" {
            Err(SkipReason::Folder)
        } else if is_empty_substash(snapshot, tab) {
            Err(SkipReason::EmptyStub)
        } else {
            tab_verdict(tab, max_age, now)
                .and_then(|reason| fetch_action(realm, snapshot, tab, reason))
        };
        match verdict {
            Ok(action) => facet.actions.push(action),
            Err(reason) => facet.skipped.push(SkippedTab {
                id: tab.id.clone(),
                name: tab.name.clone(),
                reason,
            }),
        }
    }
    if let Selection::Ids(ids) = selection {
        facet.unknown = ids
            .iter()
            .filter(|w| !snapshot.tabs.iter().any(|t| &t.id == *w))
            .cloned()
            .collect();
    }
    facet
}

fn compile_characters(
    realm: Realm,
    snapshot: &RefreshSnapshot,
    selection: &Selection,
    max_age: i64,
    now: i64,
) -> Facet<SkippedCharacter> {
    let mut facet = Facet::empty();
    let listing_alone = snapshot.character_listing.is_none();
    if let Some(reason) = listing_verdict(snapshot.character_listing.as_ref(), max_age, now) {
        facet
            .actions
            .push(RefreshAction::ListCharacters { realm, reason });
    }
    for c in &snapshot.characters {
        if !selection.covers_id(&c.id) {
            continue;
        }
        let verdict = if listing_alone {
            Err(CharacterSkipReason::AwaitingListing)
        } else if c.league.is_none() {
            Err(CharacterSkipReason::NoLeague)
        } else if listed_flag(c, "deleted") {
            Err(CharacterSkipReason::Deleted)
        } else if listed_flag(c, "expired") {
            Err(CharacterSkipReason::Expired)
        } else {
            character_verdict(c, max_age, now).map(|reason| RefreshAction::FetchCharacter {
                realm,
                league: snapshot.league.clone(),
                id: c.id.clone(),
                name: c.name.clone(),
                reason,
            })
        };
        match verdict {
            Ok(action) => facet.actions.push(action),
            Err(reason) => facet.skipped.push(SkippedCharacter {
                id: c.id.clone(),
                name: c.name.clone(),
                reason,
            }),
        }
    }
    if let Selection::Ids(ids) = selection {
        facet.unknown = ids
            .iter()
            .filter(|w| !snapshot.characters.iter().any(|c| &c.id == *w))
            .cloned()
            .collect();
    }
    facet
}

/// A listing-entry flag (`deleted`, `expired`): documented as "always
/// `true` if present", read as exactly that.
fn listed_flag(c: &CharacterSnapshot, key: &str) -> bool {
    c.listed.get(key).and_then(Value::as_bool) == Some(true)
}

/// A substash (its recorded parent is a map/unique tab on record) whose
/// stub counts 0 items while the store holds none at it. Only substash
/// stubs carry the count; a listing entry or a folder child never trips
/// this.
fn is_empty_substash(snapshot: &RefreshSnapshot, tab: &TabSnapshot) -> bool {
    let Some(parent) = tab.parent.as_deref() else {
        return false;
    };
    let parent_is_container = snapshot
        .tabs
        .iter()
        .any(|t| t.id == parent && matches!(t.r#type.as_str(), "MapStash" | "UniqueStash"));
    parent_is_container
        && tab.metadata.get("items").and_then(Value::as_i64) == Some(0)
        && tab.item_count == 0
}

/// The facet-blind half of the freshness rules, the one place they live:
/// never fetched → fetch; older than the window → fetch. `Err(age)` is a
/// fetch inside the window, handed to the facet's own disagreement check
/// — the only thing that can still force a fetch. `fetched_at` is the
/// store's word: `None` covers a location a listing revived, whose old
/// facts went with the retirement.
fn window_verdict(fetched_at: Option<i64>, max_age: i64, now: i64) -> Result<FetchReason, i64> {
    let Some(fetched_at) = fetched_at else {
        return Ok(FetchReason::NeverFetched);
    };
    let age = now.saturating_sub(fetched_at).max(0);
    if age > max_age {
        return Ok(FetchReason::Stale { age_seconds: age });
    }
    Err(age)
}

/// Whether a listing is *newer* than the fetch it is being read against.
/// The heuristics only read forward: a listing older than (or racing,
/// same-second) our fetch says nothing about the fetch's contents.
fn listing_is_newer(listed_at: Option<i64>, fetched_at: Option<i64>) -> bool {
    matches!((listed_at, fetched_at), (Some(l), Some(f)) if l > f)
}

/// Fetch or skip, for one covered non-folder tab. `Err` is the skip
/// reason. The tab's own arm: a listing newer than our fetch counting
/// differently → fetch (proof of change); fresh with no such proof →
/// skip. An agreeing count is not proof of freshness and cannot skip
/// anything on its own.
fn tab_verdict(tab: &TabSnapshot, max_age: i64, now: i64) -> Result<FetchReason, SkipReason> {
    let age = match window_verdict(tab.fetched_at, max_age, now) {
        Ok(reason) => return Ok(reason),
        Err(age) => age,
    };
    if listing_is_newer(tab.listed_at, tab.fetched_at)
        && let Some(listed) = tab.metadata.get("items").and_then(Value::as_i64)
        && listed != tab.item_count
    {
        return Ok(FetchReason::ListedCountDisagrees {
            listed,
            held: tab.item_count,
        });
    }
    Err(SkipReason::Fresh { age_seconds: age })
}

/// Fetch or skip, for one covered, listed, league-bearing, live character.
/// The character's own arms (C62): a listing newer
/// than our fetch reporting a different `experience` proves play since —
/// judged only when both the entry and the fetched body carry it; and
/// the listing's league (the row's, listing-owned) differing from the
/// fetched body's is the same arm. Fresh with no such proof → skip.
fn character_verdict(
    c: &CharacterSnapshot,
    max_age: i64,
    now: i64,
) -> Result<FetchReason, CharacterSkipReason> {
    let age = match window_verdict(c.fetched_at, max_age, now) {
        Ok(reason) => return Ok(reason),
        Err(age) => age,
    };
    if listing_is_newer(c.listed_at, c.fetched_at) {
        if let (Some(listed), Some(held)) = (
            c.listed.get("experience").and_then(Value::as_i64),
            c.fetched.get("experience").and_then(Value::as_i64),
        ) && listed != held
        {
            return Ok(FetchReason::ListedExperienceDisagrees { listed, held });
        }
        if let (Some(listed), Some(held)) = (
            c.league.as_deref(),
            c.fetched.get("league").and_then(Value::as_str),
        ) && listed != held
        {
            return Ok(FetchReason::ListedLeagueDisagrees {
                listed: listed.into(),
                held: held.into(),
            });
        }
    }
    Err(CharacterSkipReason::Fresh { age_seconds: age })
}

/// A fetch action for `tab`: under its parent when the parent is a
/// map/unique tab on record (a substash), by its own id when the parent
/// is anything else on record (a folder child — folders group, they
/// don't contain) or absent entirely (a top-level tab). `Err` is the one
/// unrenderable case: a recorded parent that is no longer on record —
/// the store keeps substash rows when a listing retires their parent, so
/// guessing an endpoint here would fetch the wrong path.
fn fetch_action(
    realm: Realm,
    snapshot: &RefreshSnapshot,
    tab: &TabSnapshot,
    reason: FetchReason,
) -> Result<RefreshAction, SkipReason> {
    let parent = match tab.parent.as_deref() {
        None => None,
        Some(p) => match snapshot.tabs.iter().find(|t| t.id == p) {
            None => return Err(SkipReason::OrphanedParent { parent: p.into() }),
            Some(row) => {
                matches!(row.r#type.as_str(), "MapStash" | "UniqueStash").then_some(&row.id)
            }
        },
    };
    Ok(match parent {
        Some(parent) => RefreshAction::FetchSubstash {
            realm,
            league: snapshot.league.clone(),
            parent: parent.clone(),
            id: tab.id.clone(),
            name: tab.name.clone(),
            tab_type: tab.r#type.clone(),
            reason,
        },
        None => RefreshAction::FetchTab {
            realm,
            league: snapshot.league.clone(),
            id: tab.id.clone(),
            name: tab.name.clone(),
            tab_type: tab.r#type.clone(),
            reason,
        },
    })
}

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "tests/ages.rs"]
mod ages;
