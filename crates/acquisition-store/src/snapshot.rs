//! Neutral snapshots: the reads `acquisition-plan` derives from. A
//! snapshot names facts and intent together and carries nothing derived.
//! [`RefreshSnapshot`] is what the planner compiles plans from — the
//! listing bases a plan cites (the league's stash listing, the realm's
//! character listing), tab and character identities with their freshness
//! and their listed entries, the sync-policy annotation row at its
//! revision, and the account uuid the pairing is bound to — with no
//! staleness verdicts and no request lists. [`PricingSnapshot`] (pricing
//! slice step 4, 2026-09-06) is what the listing state resolves from —
//! the same tabs and characters, every live item at them with its note
//! verbatim, and every `buyout` row raw at its revision — with no
//! coverage, no parse, no relation. Policy compilation and price
//! resolution live in `acquisition-plan`, never here — the store exposes
//! neutral snapshots, "never half a planner" (C39, C64).
//!
//! Liveness is settled here, not in the planner (C54): a row in the
//! snapshot is live by the latest listing's say, `fetched_at` is `None`
//! for a never-fetched *or revived* location, and the listed entry is the
//! listing's verbatim. The planner reads those three facts and adds no
//! liveness logic of its own.

use anyhow::{Context, Result, bail};
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::annotations::{AnnotationList, AnnotationRow, Annotations, UnreadableRow};
use crate::{Store, TAB_ORDER_KEYS};

/// The per-account sync policy's annotation address (scope `"account"`,
/// key `""`): the declaration of desired coverage and freshness. Its
/// value's shape is the planner's business; the store only carries it.
pub const SYNC_POLICY_KIND: &str = "sync-policy";

/// The address's other two components, named once so every frontend's
/// read and write surface resolves the same row this snapshot reads.
pub const SYNC_POLICY_SCOPE: &str = "account";
pub const SYNC_POLICY_KEY: &str = "";

/// The stash listing this snapshot's tab set derives from — the fact
/// basis a plan cites. Strict on parse (it is embedded in serialized
/// plan envelopes, which refuse unknown fields whole).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListingBasis {
    /// Row id in `responses`, stable for the life of the file.
    pub response_id: i64,
    pub fetched_at: i64,
}

/// One tab as the planner sees it: identity and freshness, plus what the
/// server said about it. Substash identity is `(parent, id)`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TabSnapshot {
    pub id: String,
    pub parent: Option<String>,
    pub name: String,
    pub r#type: String,
    pub idx: Option<i64>,
    pub listed_at: Option<i64>,
    /// `responses.id` of the listing (or, for a substash stub, the parent
    /// fetch) that last listed this tab. After a listing, every top-level
    /// tab and folder child carries the basis `response_id` — which is
    /// what lets a plan check its tab set against the basis it cites.
    /// `None`: fetched directly, never listed.
    pub listed_response: Option<i64>,
    pub fetched_at: Option<i64>,
    /// The tab's `metadata` from the listing entry / substash stub,
    /// verbatim (colour, map name, the heuristic `items` count — evidence
    /// that can prove a tab changed, never that it didn't). A fetch never
    /// overwrites it; `Null` when never listed or the entry had none.
    pub metadata: Value,
    /// Live items this store holds at the tab — what fetches produced, as
    /// opposed to what the listing promised.
    pub item_count: i64,
}

/// One character as the planner sees it: identity (the GGG `id`), the
/// address a fetch takes (`name`, listing-owned), the coverage coordinate
/// (`league`, listing-owned; `None` when the listing gave none), freshness,
/// and what the server said on each route — verbatim, nothing derived.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterSnapshot {
    pub id: String,
    pub name: String,
    pub league: Option<String>,
    pub listed_at: Option<i64>,
    /// `responses.id` of the listing that last listed this character;
    /// `None`: fetched directly, never listed.
    pub listed_response: Option<i64>,
    /// `None`: never fetched — or retired by a listing and revived by a
    /// later one, which clears it (the store's rule, not the planner's).
    pub fetched_at: Option<i64>,
    /// The listing entry, verbatim (`experience`, `deleted`, `expired`,
    /// the entry's own `league`…); `Null` when never listed or recorded
    /// before the entry was kept (facts v4).
    pub listed: Value,
    /// The fetched character's envelope (the body minus its item arrays),
    /// verbatim, only while `fetched_at` is set; `Null` otherwise — a
    /// revived row still holds its old body in the store, and the planner
    /// must not read a body the listing has disowned.
    pub fetched: Value,
}

/// A target's latest successful fetch, as a snapshot reads it from the
/// response rows (C111, spent by response): the response it landed in,
/// and that response's own time — never the target's current location's,
/// which a retired or revived row no longer carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fetched {
    pub response_id: i64,
    pub fetched_at: i64,
}

/// A named snapshot of one (realm, league)'s refresh facts — the stash
/// listing and its tabs, the realm's character listing and this league's
/// characters — plus the account's sync policy, taken with no daemon
/// involved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefreshSnapshot {
    /// The account uuid the facts file records (`/profile` lands at every
    /// login) — the identity annotation files carry internally, so a plan
    /// can cite it. Facts and intent are paired under this one uuid;
    /// [`Store::refresh_snapshot`] refuses an annotations handle whose
    /// stored uuid differs or is absent.
    /// The provider is not here: the store cannot verify it — the caller
    /// binds it by the provider directory it opened.
    pub account_uuid: String,
    /// Display name beside the uuid, when the profile carried one.
    pub account_name: Option<String>,
    /// The coordinate above league (C58): `Standard`
    /// exists in both games, so facts are read per (realm, league).
    pub realm: String,
    pub league: String,
    pub taken_at: i64,
    /// `None`: this league was never listed (tabs may still exist from
    /// direct fetches). A plan that needs a listing says so; the snapshot
    /// does not invent one.
    pub stash_listing: Option<ListingBasis>,
    /// Live tabs in listing order (same order as [`Store::tabs`]),
    /// consistent with `stash_listing` — both are read in one transaction.
    pub tabs: Vec<TabSnapshot>,
    /// The realm's character listing (the list endpoint is per realm, not
    /// per league); `None` when the realm was never listed.
    pub character_listing: Option<ListingBasis>,
    /// Live characters whose listing-owned league is this league — plus
    /// the realm's characters with no league at all, which no
    /// (realm, league) policy can cover and every league plan of the
    /// realm therefore reports. Read in the same transaction.
    pub characters: Vec<CharacterSnapshot>,
    /// The account's league list as last fetched (C126, a league the list
    /// lacks is a note), read in the same transaction; `None` when no list
    /// was ever fetched.
    pub league_list: Option<crate::LeagueList>,
    /// The sync-policy annotation at its revision — the annotation basis
    /// a plan cites. `None` means exactly "no sync policy": the
    /// annotations handle is required, so absent intent is never
    /// conflated with not having looked.
    pub policy: Option<AnnotationRow>,
    /// The highest response id the read saw (C111, spent by response): a
    /// fetch that lands in a later response is after the snapshot, told
    /// by the response and never the clock, whose whole seconds cannot
    /// order two events in one second.
    #[serde(default)]
    pub response_watermark: i64,
    /// For each fetch target of this (realm, league) — each tab and page
    /// by its coordinate, each character of the realm by name
    /// (`Endpoint::fetch_key`) — its latest successful fetch.
    #[serde(default)]
    pub fetches: std::collections::BTreeMap<String, Fetched>,
}

impl RefreshSnapshot {
    /// A target's latest successful fetch, as the snapshot read it; none
    /// where it was never fetched.
    pub fn fetched_in(&self, target: &crate::Endpoint) -> Option<Fetched> {
        target
            .fetch_key()
            .and_then(|key| self.fetches.get(&key).copied())
    }
}

/// One live item as the pricing reader sees it (the listing state,
/// `acquisition-plan`): identity, its full location, the columns a
/// price line needs, and the `note` verbatim — nothing derived, and not
/// the whole body (a league is tens of thousands of rows; the body is
/// [`Store::item`]'s to give).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemSnapshot {
    pub id: String,
    /// `stash` or `character`.
    pub location_kind: String,
    /// The tab, substash or character id.
    pub location_id: String,
    /// The array the item came from; `None` before facts v4.
    pub container: Option<String>,
    /// The parent item, for a socketed gem: it has no position of its own.
    pub socketed_in: Option<String>,
    pub name: String,
    pub type_line: String,
    pub stack_size: Option<i64>,
    pub x: Option<i64>,
    pub y: Option<i64>,
    /// The item's `note`, exactly as the API returned it; `None` when the
    /// body carried none, or carried one that is no string.
    pub note: Option<String>,
    /// The body carries a `note` that is no string — a number, an object,
    /// an array: nothing here says what it would read as, and a consumer
    /// must not read the silence as "no note" (the search's rule 8, at the
    /// note's grain; found by its generators at step 9, 2026-09-25, and
    /// the object case by the step's outside review).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub note_unread: bool,
    /// The item's `inventoryId`, verbatim: a character item's slot, the
    /// literal `Stash1` for every stash item (T13), absent on a socketed
    /// item. A forum link to a character item is addressed by it.
    pub inventory_id: Option<String>,
    /// The body carries an `inventoryId` that is no string: carried as
    /// none, and said here (the step-9 review: one such body failed the
    /// snapshot whole).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub inventory_id_unread: bool,
    /// `responses.id` of the fetch that last saw the item here.
    pub seen_response: Option<i64>,
    pub last_seen: i64,
}

/// A named snapshot of one (realm, league)'s pricing facts — the stash
/// listing basis and its tabs (metadata verbatim: `public` lives there),
/// the realm's character listing and this league's characters, every live
/// item at those locations with its note — plus every `buyout` intent row
/// the account holds, raw, at its revision. Nothing derived: which row
/// covers which item, what a note says, what a relation means, are the
/// listing state's (`acquisition-plan`, C69), never this crate's.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PricingSnapshot {
    pub account_uuid: String,
    pub account_name: Option<String>,
    pub realm: String,
    pub league: String,
    pub taken_at: i64,
    pub stash_listing: Option<ListingBasis>,
    /// Live tabs in listing order, folders and substash stubs included.
    pub tabs: Vec<TabSnapshot>,
    pub character_listing: Option<ListingBasis>,
    pub characters: Vec<CharacterSnapshot>,
    /// Live items whose location is one of `tabs` or `characters`, in
    /// location order, positioned items before socketed gems.
    pub items: Vec<ItemSnapshot>,
    /// Every live `buyout` row, whatever its scope and realm — the intent
    /// file is per account, not per league; the reader matches targets.
    pub buyouts: Vec<AnnotationRow>,
    /// Every live `buyout` row this build cannot read, named (C151), from
    /// the same read as `buyouts`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unreadable_buyouts: Vec<UnreadableRow>,
}

/// The `kind` of the price intent rows this snapshot carries (its value
/// shape is `acquisition-plan`'s, C67).
pub const BUYOUT_KIND: &str = "buyout";

impl Store {
    /// Snapshot one (realm, league)'s refresh facts and the account's
    /// sync-policy row, so a plan's fact bases and annotation revision
    /// come from one read, bound to one account. This is the neutral
    /// snapshot C39 names: rows and bases, nothing derived — the planner
    /// (`acquisition-plan`) owns every conclusion drawn from it.
    pub fn refresh_snapshot(
        &self,
        realm: &str,
        league: &str,
        annotations: &Annotations,
    ) -> Result<RefreshSnapshot> {
        // All fact reads share one read transaction: under WAL the daemon
        // commits while frontends read, and a listing landing between the
        // basis query and the tab query would pair the old response id
        // with the new tab set. (The annotations file is a separate
        // database and cannot join this transaction; its basis is the
        // policy row's revision, which the CAS write path re-checks.)
        let tx = self.conn.unchecked_transaction()?;
        let (account_uuid, account_name) = self.account_identity(&tx)?;
        check_pairing(annotations, &account_uuid)?;
        let stash_listing = stash_basis(&tx, realm, league)?;
        let character_listing = character_basis(&tx, realm)?;
        let tabs = read_tabs(&tx, realm, league)?;
        let characters = read_characters(&tx, realm, league)?;
        let league_list = crate::league_list_in(&tx)?;
        let (response_watermark, fetches) = read_fetches(&tx, realm, league)?;
        tx.finish()?;
        let policy = annotations.get(SYNC_POLICY_SCOPE, SYNC_POLICY_KEY, SYNC_POLICY_KIND)?;
        Ok(RefreshSnapshot {
            account_uuid,
            account_name,
            realm: realm.into(),
            league: league.into(),
            taken_at: crate::now(),
            stash_listing,
            tabs,
            character_listing,
            characters,
            league_list,
            policy,
            response_watermark,
            fetches,
        })
    }

    /// Snapshot one (realm, league)'s pricing facts — the same tabs and
    /// characters as [`Store::refresh_snapshot`], every live item at them
    /// with its note verbatim — and every `buyout` row the account holds,
    /// bound to one account the same way. Neutral (C39, C64): the listing
    /// state is derived from this by `acquisition-plan`, never here.
    ///
    /// The character read is the planner's: a character the listing gave
    /// no league (`league: None`) is carried under every league of its
    /// realm, so that every plan can report it as outside coverage. A
    /// consumer of this snapshot must not read that as membership — the
    /// listing state flags such characters and their items
    /// (`league_unknown`) rather than attributing them to the league
    /// (step-4 outside review, finding 1).
    pub fn pricing_snapshot(
        &self,
        realm: &str,
        league: &str,
        annotations: &Annotations,
    ) -> Result<PricingSnapshot> {
        let tx = self.conn.unchecked_transaction()?;
        let (account_uuid, account_name) = self.account_identity(&tx)?;
        check_pairing(annotations, &account_uuid)?;
        let stash_listing = stash_basis(&tx, realm, league)?;
        let character_listing = character_basis(&tx, realm)?;
        let tabs = read_tabs(&tx, realm, league)?;
        let characters = read_characters(&tx, realm, league)?;
        let items = read_items(&tx, realm, league)?;
        tx.finish()?;
        let AnnotationList {
            rows: buyouts,
            unreadable: unreadable_buyouts,
        } = annotations.list(None, Some(BUYOUT_KIND))?;
        Ok(PricingSnapshot {
            account_uuid,
            account_name,
            realm: realm.into(),
            league: league.into(),
            taken_at: crate::now(),
            stash_listing,
            tabs,
            character_listing,
            characters,
            items,
            buyouts,
            unreadable_buyouts,
        })
    }

    /// The one account identity the facts file records (`/profile` lands
    /// at every login) — the uuid intent is paired under.
    pub(crate) fn account_identity(
        &self,
        tx: &rusqlite::Transaction,
    ) -> Result<(String, Option<String>)> {
        let accounts: Vec<(String, Option<String>)> = {
            let mut stmt = tx.prepare("SELECT uuid, name FROM account")?;
            let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?;
            rows.collect::<Result<_, _>>()?
        };
        match accounts.as_slice() {
            [one] => Ok(one.clone()),
            [] => bail!(
                "facts file {} records no account identity (no profile response has landed); \
                 a snapshot cannot pair your prices and policy with it — one login fixes this",
                self.path.display()
            ),
            many => bail!(
                "facts file {} records {} account identities; refusing to pair your prices and policy with it",
                self.path.display(),
                many.len()
            ),
        }
    }
}

/// Pairing is by the uuid the annotations file carries internally
/// (`meta`), not by filename — a copied or renamed file keeps its owner.
/// A handle with no identity (a file opened from a raw path and never
/// bound) is refused, never trusted.
fn check_pairing(annotations: &Annotations, account_uuid: &str) -> Result<()> {
    match annotations.uuid() {
        Some(u) if u == account_uuid => Ok(()),
        Some(u) => bail!(
            "annotations file {} belongs to account uuid {u}, not {account_uuid}",
            annotations.path().display()
        ),
        None => bail!(
            "annotations handle {} carries no account identity; open it with \
             Annotations::open_for so the pairing is checkable",
            annotations.path().display()
        ),
    }
}

fn basis_row(r: &rusqlite::Row) -> rusqlite::Result<ListingBasis> {
    Ok(ListingBasis {
        response_id: r.get(0)?,
        fetched_at: r.get(1)?,
    })
}

/// The realm and league of a listing live in its params; omitted, they
/// defaulted to pc / "Standard" at record time (`Endpoint::from_job`), so
/// the match here defaults the same way — which is also how pre-realm
/// rows keep answering for pc.
fn stash_basis(
    tx: &rusqlite::Transaction,
    realm: &str,
    league: &str,
) -> Result<Option<ListingBasis>> {
    Ok(tx
        .query_row(
            "SELECT id, fetched_at FROM responses
              WHERE endpoint = 'stashes' AND status BETWEEN 200 AND 299
                AND COALESCE(json_extract(params, '$.realm'), 'pc') = ?1
                AND COALESCE(json_extract(params, '$.league'), 'Standard') = ?2
              ORDER BY id DESC LIMIT 1",
            [realm, league],
            basis_row,
        )
        .optional()?)
}

/// The character list is per realm; the same query the v4 migration's
/// membership re-stamp uses, so the basis cited here is the one the rows
/// are stamped to.
fn character_basis(tx: &rusqlite::Transaction, realm: &str) -> Result<Option<ListingBasis>> {
    Ok(tx
        .query_row(
            "SELECT id, fetched_at FROM responses
              WHERE endpoint = 'characters' AND status BETWEEN 200 AND 299
                AND COALESCE(json_extract(params, '$.realm'), 'pc') = ?1
              ORDER BY id DESC LIMIT 1",
            [realm],
            basis_row,
        )
        .optional()?)
}

/// The highest response id, and the response (with its time) each fetch
/// target of this (realm, league) last landed in — its tabs' and pages' fetches, its
/// realm's characters' — read from the response rows themselves, in the
/// snapshot's transaction (C111, spent by response).
fn read_fetches(
    tx: &rusqlite::Transaction,
    realm: &str,
    league: &str,
) -> Result<(i64, std::collections::BTreeMap<String, Fetched>)> {
    let watermark: i64 = tx.query_row("SELECT COALESCE(MAX(id), 0) FROM responses", [], |r| {
        r.get(0)
    })?;
    let mut fetches = std::collections::BTreeMap::new();
    let mut stmt = tx.prepare(
        "SELECT id, endpoint, params, fetched_at FROM responses
          WHERE endpoint IN ('stash', 'character') AND status BETWEEN 200 AND 299
          ORDER BY id",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok((
            r.get::<_, i64>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, String>(2)?,
            r.get::<_, i64>(3)?,
        ))
    })?;
    for row in rows {
        let (id, endpoint, params, fetched_at) = row?;
        // a row whose params do not read names no target
        let Ok(params) = serde_json::from_str::<Value>(&params) else {
            continue;
        };
        let target = crate::Endpoint::from_job(&endpoint, &params);
        let ours = match &target {
            Some(crate::Endpoint::Stash {
                realm: r,
                league: l,
                ..
            }) => r == realm && l == league,
            Some(crate::Endpoint::Character { realm: r, .. }) => r == realm,
            _ => false,
        };
        if let Some(key) = target.filter(|_| ours).and_then(|t| t.fetch_key()) {
            fetches.insert(
                key,
                Fetched {
                    response_id: id,
                    fetched_at,
                },
            );
        }
    }
    Ok((watermark, fetches))
}

fn read_tabs(tx: &rusqlite::Transaction, realm: &str, league: &str) -> Result<Vec<TabSnapshot>> {
    type RawTab = (
        String,
        Option<String>,
        String,
        String,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<String>,
        i64,
    );
    let rows: Vec<RawTab> = {
        let mut stmt = tx.prepare(&format!(
            "SELECT t.id, t.parent, COALESCE(t.name, ''), COALESCE(t.type, ''), t.idx, t.listed_at, t.listed_response, t.fetched_at, t.listed_json,
                    (SELECT count(*) FROM items i WHERE i.realm = t.realm AND i.league = t.league AND i.location_kind = 'stash' AND i.location_id = t.id AND i.removed_at IS NULL)
               FROM tabs t WHERE t.realm = ?1 AND t.league = ?2 AND t.removed_at IS NULL ORDER BY {TAB_ORDER_KEYS}"
        ))?;
        let rows = stmt.query_map([realm, league], |r| {
            Ok((
                r.get(0)?,
                r.get(1)?,
                r.get(2)?,
                r.get(3)?,
                r.get(4)?,
                r.get(5)?,
                r.get(6)?,
                r.get(7)?,
                r.get(8)?,
                r.get(9)?,
            ))
        })?;
        rows.collect::<Result<_, _>>()?
    };
    rows.into_iter()
        .map(
            |(
                id,
                parent,
                name,
                r#type,
                idx,
                listed_at,
                listed_response,
                fetched_at,
                listed_json,
                item_count,
            )| {
                // A row this store wrote that no longer parses is a
                // damaged file, reported with its address — never
                // silently read as "no metadata".
                let metadata = match &listed_json {
                    None => Value::Null,
                    Some(raw) => serde_json::from_str::<Value>(raw)
                        .with_context(|| {
                            format!("tab {realm}/{league}/{id}: malformed listing entry in store")
                        })?
                        .get("metadata")
                        .cloned()
                        .unwrap_or(Value::Null),
                };
                Ok(TabSnapshot {
                    id,
                    parent,
                    name,
                    r#type,
                    idx,
                    listed_at,
                    listed_response,
                    fetched_at,
                    metadata,
                    item_count,
                })
            },
        )
        .collect()
}

/// This league's live characters, in the same order `acq characters`
/// prints, plus the realm's league-less ones. `json` is read only when a
/// fetch stands: after revival the column still holds the disowned body.
fn read_characters(
    tx: &rusqlite::Transaction,
    realm: &str,
    league: &str,
) -> Result<Vec<CharacterSnapshot>> {
    type RawCharacter = (
        String,
        String,
        Option<String>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<String>,
        Option<String>,
    );
    let rows: Vec<RawCharacter> = {
        let mut stmt = tx.prepare(
            "SELECT c.id, c.name, c.league, c.listed_at, c.listed_response, c.fetched_at, c.listed_json,
                    CASE WHEN c.fetched_at IS NULL THEN NULL ELSE c.json END
               FROM characters c
              WHERE c.realm = ?1 AND c.removed_at IS NULL AND (c.league = ?2 OR c.league IS NULL)
              ORDER BY c.league, c.level DESC, c.name",
        )?;
        let rows = stmt.query_map([realm, league], |r| {
            Ok((
                r.get(0)?,
                r.get(1)?,
                r.get(2)?,
                r.get(3)?,
                r.get(4)?,
                r.get(5)?,
                r.get(6)?,
                r.get(7)?,
            ))
        })?;
        rows.collect::<Result<_, _>>()?
    };
    rows.into_iter()
        .map(
            |(id, name, league, listed_at, listed_response, fetched_at, listed, fetched)| {
                let parse = |raw: Option<String>, what: &str| -> Result<Value> {
                    match raw {
                        None => Ok(Value::Null),
                        Some(raw) => serde_json::from_str::<Value>(&raw).with_context(|| {
                            format!("character {realm}/{id}: malformed {what} in store")
                        }),
                    }
                };
                Ok(CharacterSnapshot {
                    listed: parse(listed, "listing entry")?,
                    fetched: parse(fetched, "fetched character")?,
                    id,
                    name,
                    league,
                    listed_at,
                    listed_response,
                    fetched_at,
                })
            },
        )
        .collect()
}

/// The string fields of a body as one multi-path `json_extract` hands them
/// over: a JSON array of the values, their JSON types kept — a string
/// quoted, an object an object, an absent path or a JSON `null` a `null`
/// (SQLite's single-path extract flattens an object to its text, which
/// read as a note; the step-9 review). A string is read, `null` is none
/// — as the search's deriver reads a body — and anything else is unread
/// at that field alone, never a failed read (C47). The array is read by
/// a scanner that never descends into a value: a decoder would refuse
/// the whole array for one field nested past its depth, and the field
/// beside it — a valid note — would read as absent (the review's second
/// look). A text this is not an array of `n` values reads as `n` unread
/// fields.
fn body_strings<const N: usize>(extracted: Option<&str>) -> [(Option<String>, bool); N] {
    let unread = || std::array::from_fn(|_| (None, true));
    let Some(text) = extracted else {
        return std::array::from_fn(|_| (None, false));
    };
    let mut out: Vec<(Option<String>, bool)> = Vec::with_capacity(N);
    let bytes = text.as_bytes();
    let mut at = match bytes.first() {
        Some(b'[') => 1,
        _ => return unread(),
    };
    loop {
        while bytes.get(at).is_some_and(u8::is_ascii_whitespace) {
            at += 1;
        }
        let Some(&first) = bytes.get(at) else {
            return unread();
        };
        let end = match first {
            // a string token: to its closing quote, escapes skipped, and
            // decoded on its own — flat, so no depth is in question
            b'"' => {
                let mut i = at + 1;
                loop {
                    match bytes.get(i) {
                        Some(b'\\') => i += 2,
                        Some(b'"') => break,
                        Some(_) => i += 1,
                        None => return unread(),
                    }
                }
                match serde_json::from_str::<String>(&text[at..=i]) {
                    Ok(value) => out.push((Some(value), false)),
                    Err(_) => return unread(),
                }
                i + 1
            }
            _ => {
                // anything else: skipped to the end of the value by
                // counting brackets outside strings, and null is none
                let mut i = at;
                let mut depth = 0i64;
                let mut in_string = false;
                loop {
                    let Some(&b) = bytes.get(i) else {
                        return unread();
                    };
                    if in_string {
                        match b {
                            b'\\' => i += 1,
                            b'"' => in_string = false,
                            _ => {}
                        }
                    } else {
                        match b {
                            b'"' => in_string = true,
                            b'[' | b'{' => depth += 1,
                            b']' | b'}' if depth > 0 => depth -= 1,
                            b',' | b']' if depth == 0 => break,
                            _ => {}
                        }
                    }
                    i += 1;
                }
                let value = text[at..i].trim();
                out.push(if value == "null" {
                    (None, false)
                } else {
                    (None, true)
                });
                i
            }
        };
        at = end;
        while bytes.get(at).is_some_and(u8::is_ascii_whitespace) {
            at += 1;
        }
        match bytes.get(at) {
            Some(b',') => at += 1,
            Some(b']') if out.len() == N && at + 1 == bytes.len() => break,
            _ => return unread(),
        }
    }
    match out.try_into() {
        Ok(fields) => fields,
        Err(_) => unread(),
    }
}

/// Live items at this league's live tabs and this league's (or
/// league-less) live characters of the realm — the same membership
/// [`read_tabs`] and [`read_characters`] report, so an item never cites a
/// location the snapshot does not carry. The note is the body's `note`,
/// verbatim; the body itself stays in the store. The two body fields are
/// one extract, so each body is parsed once (measured 2026-09-25: two
/// extracts, one two-path extract and a type check beside an extract cost
/// 80, 80 and 100 ms over 21,311 bodies).
fn read_items(tx: &rusqlite::Transaction, realm: &str, league: &str) -> Result<Vec<ItemSnapshot>> {
    let mut stmt = tx.prepare(
        "SELECT i.id, i.location_kind, i.location_id, i.container, i.socketed_in,
                COALESCE(i.name, ''), COALESCE(i.type_line, ''), i.stack_size, i.x, i.y,
                json_extract(i.json, '$.note', '$.inventoryId'),
                i.seen_response, i.last_seen
           FROM items i
          WHERE i.realm = ?1 AND i.removed_at IS NULL
            AND ((i.location_kind = 'stash' AND i.league = ?2
                  AND EXISTS (SELECT 1 FROM tabs t WHERE t.realm = i.realm AND t.league = i.league
                                                     AND t.id = i.location_id AND t.removed_at IS NULL))
              OR (i.location_kind = 'character'
                  AND EXISTS (SELECT 1 FROM characters c WHERE c.realm = i.realm AND c.id = i.location_id
                                                           AND c.removed_at IS NULL AND (c.league = ?2 OR c.league IS NULL))))
          ORDER BY i.location_kind, i.location_id, i.y IS NULL, i.y, i.x, i.id",
    )?;
    let rows = stmt.query_map([realm, league], |r| {
        // GGG's note and inventoryId are strings; a body carrying anything
        // else there is malformed at that field alone, and is carried as
        // unread rather than failing the snapshot whole (C47)
        let fields: Option<String> = r.get(10)?;
        let [(note, note_unread), (inventory_id, inventory_id_unread)] =
            body_strings::<2>(fields.as_deref());
        Ok(ItemSnapshot {
            id: r.get(0)?,
            location_kind: r.get(1)?,
            location_id: r.get(2)?,
            container: r.get(3)?,
            socketed_in: r.get(4)?,
            name: r.get(5)?,
            type_line: r.get(6)?,
            stack_size: r.get(7)?,
            x: r.get(8)?,
            y: r.get(9)?,
            note,
            note_unread,
            inventory_id,
            inventory_id_unread,
            seen_response: r.get(11)?,
            last_seen: r.get(12)?,
        })
    })?;
    Ok(rows.collect::<Result<_, _>>()?)
}

#[cfg(test)]
#[path = "../unit/tests/snapshot.rs"]
mod tests;
