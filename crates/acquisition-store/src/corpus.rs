//! The search read (C108, C98): every live item's stored body with its
//! ingest facts, streamed under one snapshot with the locations it sits
//! at, and the revision that snapshot is labelled with. What a body says
//! is derived in `acquisition-search`, never here; this crate names no
//! search type, and the search crate copies a [`CorpusItem`]'s columns
//! into a struct of its own. Rulings: `decisions/store.md`, C108;
//! `decisions/search.md`, C103 (the deriver) and C98 (the basis).
//!
//! # As built
//!
//! - **One snapshot.** [`Store::read_corpus`] runs inside one read
//!   transaction, as [`Store::refresh_snapshot`] does. Its first statement
//!   reads the revision, which is what fixes the WAL snapshot: the header
//!   and every body the caller is then handed describe that state, however
//!   long the caller takes and whatever lands meanwhile.
//! - **Streamed.** The caller's closure is handed the rows one at a time
//!   ([`CorpusItems`]); a returned stream would have to own the statement
//!   and the transaction it borrows. Ordered by location kind and id, then
//!   realm and item id, where realm first sorted the whole corpus, bodies
//!   included (43 MB against 10 MB resident over 22,721 items,
//!   2026-09-20). The live read walks `items_live_at` (facts v9), which
//!   holds the live alone in that order, so it sorts nothing and passes
//!   no removed row; the read of all items (**All**, below) walks
//!   `items_location` with a sort per location. Every key is an ingest
//!   fact or the id, so [`Store::rebuild`] cannot reorder a read.
//! - **League is joined as `read_items` joins it** (S131): a stash item's
//!   league is its tab's; a character item's is the character's current
//!   listing-owned league — never the stamp the item took at ingest — and
//!   a league-less character's items carry none. That is the live read
//!   and every live item; a removed character item is the one exception,
//!   and deliberate (the bullet on **All**, below).
//! - **The header is what coverage is stated from**: every live location,
//!   and the latest listing of each realm's characters and of each (realm,
//!   league)'s tabs — the bases the snapshots cite (`stash_basis`,
//!   `character_basis`), an empty listing included, since a location's own
//!   `listed_at` cannot say when a list that names nothing was seen, and a
//!   substash's is its parent's fetch. A location is live with its parent
//!   (C54): a substash whose tab a listing retired keeps its row for the
//!   planner's orphan report, has no live item, cannot be fetched, and is
//!   not in the header. A folder is a row no fetch ever fills, so a tab
//!   carries its `type` as GGG gave it, as the snapshots do, and the
//!   consumer counts never-fetched locations without the folders.
//! - **Live** is the item and its location both, the membership
//!   `read_items` reports: a location a listing retired takes its items
//!   with it (C54). It is the read's default, and what it leaves out it
//!   names: [`CorpusHeader::gone`], every removed item's id and every
//!   retired location's with when it left, so that an id held from an
//!   earlier read never looks like one the file never held.
//! - **All** ([`Store::read_corpus_of`], [`Membership::All`]) is every
//!   item the file holds, at the place it was last seen: the same joins
//!   without the tests of liveness, in the same order — but a removed
//!   character item's league, which is the one its last sighting stamped
//!   on it and never the league its character has moved to since — each
//!   removed item with when it left — its own time, or its location's where only that
//!   says so, which no ingest path leaves and no item of the owner's copy
//!   shows (2026-10-09) — and the retired locations in the header,
//!   marked, a substash that went with its tab among them. A read that
//!   hands the removed over changes under a prune (below).
//! - **The body is text, unparsed**, exactly as `items.json` holds it (the
//!   item minus its lifted `socketedItems`). One that does not parse is
//!   the search crate's to report on that item.
//! - **The realms** are those the file has a tab or a character under,
//!   live or retired, whatever the scope: what C96's "a search names its
//!   realm" is asked of. An item lands only beside its location's row, so
//!   `items` adds no realm, and reading it for one cost more than the rest
//!   of the header (8 ms of 13 over 22,721 items, a release build).
//!
//! # Decisions as recorded
//!
//! ## C108, C98 — the revision is the highest `responses.id`
//!
//! Ruled 2026-09-19 (owner: "I agree with (a)"): no schema change for
//! it, and a stamped counter stays the fallback. It advances in the
//! transaction that changes bodies, locations or membership, as read line
//! by line on 2026-09-20:
//!
//! - [`Store::record`] is the one door that writes `items`, `tabs` or
//!   `characters` outside a migration, and every path through it that
//!   commits has inserted exactly one `responses` row in that transaction;
//!   every earlier exit is an error, which rolls back whole. A refused
//!   body's row goes to `refused` alone, in a transaction of its own, and
//!   the id it did not keep is the next record's — no reader ever saw it.
//! - `acq store import` goes through `record`. [`Store::rebuild`] rewrites
//!   the derived columns alone, none of which this read hands over or
//!   orders by. No code deletes a response. No other crate opens a facts
//!   file to write it.
//! - A listing, a profile or a withheld fetch advances the revision though
//!   no item changed: a held corpus is reloaded once more than it needed.
//! - A schema migration moves rows with no response, so the facts version
//!   is part of the [`Revision`] and is printed beside it.
//! - `responses.id` is a plain `INTEGER PRIMARY KEY`, so SQLite hands out
//!   the highest id plus one, and an id comes back only if the highest row
//!   is deleted. **Whatever prunes this file keeps two rules: it never
//!   deletes the highest `responses` row, and a prune that changes what a
//!   basis read returns advances the revision.** A prune of events or of
//!   old responses changes nothing this read returns while it keeps the
//!   latest listing of each realm and league, which the header and the
//!   snapshots read. A prune of removed items or of retired locations
//!   changes what the read returns under `All` and what the live read
//!   names as gone, so it advances the revision — and the highest
//!   response id does not move when rows are deleted, so that prune
//!   brings the stamped counter with it or writes a response of its own
//!   (`decisions/store.md`, "Parked").
//! - A facts file deleted and refetched, or restored from a copy, counts
//!   from an earlier number again, so the revision carries the time the
//!   highest response was recorded beside its id (B2, `search/LEDGER.md`;
//!   owner, 2026-10-10): no schema change, `fetched_at` being every
//!   response's since the first version. An id the file itself carried
//!   would travel with a copy and miss the restore. Not covered: two
//!   histories whose last response has one id and one second. The same
//!   snapshot imported again lands at the same ids and times, and is the
//!   same state.

use anyhow::Result;
use rusqlite::OptionalExtension as _;
use serde::{Deserialize, Serialize};

use crate::{Store, TAB_ORDER_KEYS};

/// What a read of the facts is labelled with (C98's basis, the facts
/// half): equal revisions of one file are one state of its items, their
/// locations and their membership.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Revision {
    /// The highest `responses.id`; 0 before any response has landed.
    pub response: i64,
    /// When that response was recorded; 0 before any. It tells one file's
    /// history from another's that reached the same id.
    pub at: i64,
    /// The facts schema the file is stamped with: a migration moves rows
    /// and writes no response.
    pub facts_version: i64,
}

/// Which realms a read covers (C58, C96): one, or every realm the file
/// holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RealmScope<'a> {
    One(&'a str),
    All,
}

/// Which items a read hands over (C108; the search crate's
/// `corpus.rs`): the live ones, or every item the file holds, each
/// removed one with when it left.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Membership {
    #[default]
    Live,
    All,
}

/// An id a live read does not hand over and the file holds: an item
/// removed, or a tab or a character a listing retired, with when it left.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Gone {
    pub realm: String,
    /// `item`, `stash` or `character`.
    pub kind: String,
    pub id: String,
    pub removed_at: i64,
}

/// What [`Store::read_corpus`] says before the first item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CorpusHeader {
    pub revision: Revision,
    /// The account the facts file records, as the snapshots name it.
    pub account_uuid: String,
    pub account_name: Option<String>,
    /// Every realm the file has a tab or a character under, sorted,
    /// whatever the scope.
    pub realms: Vec<String>,
    /// The latest listing in scope of each realm's characters and of each
    /// (realm, league)'s tabs, by realm; one that named nothing included.
    pub listings: Vec<ListingSeen>,
    /// Every live location in scope, items or none: tabs in listing order
    /// by realm and league, then characters as `acq store characters`
    /// orders them. Under [`Membership::All`] the retired ones too, each
    /// marked ([`LocationRow::removed_at`]).
    pub locations: Vec<LocationRow>,
    /// Under [`Membership::Live`], every id in scope the read leaves out:
    /// what an id that matches nothing live is looked up in. Empty under
    /// `All`, where they are items and locations.
    pub gone: Vec<Gone>,
}

/// When a list of locations was last seen: the `responses` row the
/// snapshots cite as a plan's basis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ListingSeen {
    pub realm: String,
    /// A tab listing's league; `None` on a character listing, which is
    /// its realm's.
    pub league: Option<String>,
    /// `stash` or `character`: what the list names, as
    /// [`LocationRow::kind`] spells it.
    pub kind: String,
    pub response_id: i64,
    pub fetched_at: i64,
}

/// One live location: a folder, a tab, a substash or a character, by its
/// full coordinate (C54), with what coverage needs to say about it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LocationRow {
    pub realm: String,
    /// A tab's league; a character's current listing-owned league, `None`
    /// when the listing gave none (C61).
    pub league: Option<String>,
    /// `stash` or `character`, as [`CorpusItem::location_kind`] spells it.
    pub kind: String,
    pub id: String,
    /// A tab's folder, or a substash's tab.
    pub parent: Option<String>,
    pub name: String,
    /// A tab's `type`, verbatim; `None` on a character. `Folder` is the
    /// one that holds no items and is never fetched.
    pub tab_type: Option<String>,
    pub listed_at: Option<i64>,
    /// `None`: never fetched — or retired and revived since (C54).
    pub fetched_at: Option<i64>,
    /// When a listing retired it — its own, or its tab's where a substash
    /// went with it; always `None` under [`Membership::Live`].
    pub removed_at: Option<i64>,
}

/// One item — live, or under [`Membership::All`] removed too: its ingest
/// facts (S128) and its body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CorpusItem {
    pub id: String,
    pub realm: String,
    /// A live item's is joined, never its own stamp: a stash item's is
    /// its tab's, a character item's its character's today (S131). A
    /// removed character item's, under [`Membership::All`], IS its own
    /// stamp — the league its last sighting gave it, which its character
    /// may have left since (C108; K12 in `search/LEDGER.md`): joining it
    /// to the character's league today would move it out of the league it
    /// was last seen in. `None` on a league-less character's item.
    pub league: Option<String>,
    pub location_kind: String,
    pub location_id: String,
    /// The array the item came from; `None` on a row from before facts v4.
    pub container: Option<String>,
    /// The item this one is socketed in.
    pub socketed_in: Option<String>,
    pub first_seen: i64,
    pub last_seen: i64,
    /// When it left: a fetch of its location no longer had it, or the
    /// location was retired; always `None` under [`Membership::Live`].
    pub removed_at: Option<i64>,
    /// `items.json` as stored, unparsed.
    pub body: String,
}

/// The items of one [`Store::read_corpus`], one row at a time.
pub struct CorpusItems<'a> {
    rows: rusqlite::Rows<'a>,
}

impl Iterator for CorpusItems<'_> {
    type Item = Result<CorpusItem>;

    fn next(&mut self) -> Option<Self::Item> {
        let row = match self.rows.next() {
            Ok(Some(row)) => row,
            Ok(None) => return None,
            Err(e) => return Some(Err(e.into())),
        };
        Some(corpus_item(row).map_err(Into::into))
    }
}

fn corpus_item(r: &rusqlite::Row<'_>) -> rusqlite::Result<CorpusItem> {
    Ok(CorpusItem {
        id: r.get(0)?,
        realm: r.get(1)?,
        league: r.get(2)?,
        location_kind: r.get(3)?,
        location_id: r.get(4)?,
        container: r.get(5)?,
        socketed_in: r.get(6)?,
        first_seen: r.get(7)?,
        last_seen: r.get(8)?,
        body: r.get(9)?,
        removed_at: r.get(10)?,
    })
}

/// Every item the file holds, at the place it was last seen: the same
/// joins without the tests of liveness, and when each removed one left —
/// its own time, or its location's where only that says so. A live
/// character item's league is its character's today, as the live read
/// joins it (S131); a removed one's is the league its last sighting
/// stamped on it, as a removed stash item's is its tab's.
const ALL_ITEMS_SQL: &str = "SELECT i.id, i.realm,
        CASE WHEN i.location_kind = 'character' AND i.removed_at IS NULL AND c.removed_at IS NULL
             THEN c.league ELSE i.league END,
        i.location_kind, i.location_id, i.container, i.socketed_in, i.first_seen, i.last_seen, i.json,
        COALESCE(i.removed_at, t.removed_at, c.removed_at,
                 (SELECT p.removed_at FROM tabs p WHERE p.realm = t.realm AND p.league = t.league AND p.id = t.parent))
   FROM items i
   LEFT JOIN tabs t ON i.location_kind = 'stash' AND t.realm = i.realm AND t.league = i.league
                   AND t.id = i.location_id
   LEFT JOIN characters c ON i.location_kind = 'character' AND c.realm = i.realm
                   AND c.id = i.location_id
  WHERE (?1 IS NULL OR i.realm = ?1)
    AND (t.id IS NOT NULL OR c.id IS NOT NULL)
  ORDER BY i.location_kind, i.location_id, i.realm, i.id";

const ITEMS_SQL: &str = "SELECT i.id, i.realm,
        CASE WHEN i.location_kind = 'character' THEN c.league ELSE i.league END,
        i.location_kind, i.location_id, i.container, i.socketed_in, i.first_seen, i.last_seen, i.json, NULL
   FROM items i
   LEFT JOIN tabs t ON i.location_kind = 'stash' AND t.realm = i.realm AND t.league = i.league
                   AND t.id = i.location_id AND t.removed_at IS NULL
   LEFT JOIN characters c ON i.location_kind = 'character' AND c.realm = i.realm
                   AND c.id = i.location_id AND c.removed_at IS NULL
  WHERE i.removed_at IS NULL AND (?1 IS NULL OR i.realm = ?1)
    AND (t.id IS NOT NULL OR c.id IS NOT NULL)
  ORDER BY i.location_kind, i.location_id, i.realm, i.id";

fn read_revision(tx: &rusqlite::Transaction) -> Result<Revision> {
    let (response, at) = tx
        .query_row(
            "SELECT id, fetched_at FROM responses ORDER BY id DESC LIMIT 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()?
        .unwrap_or((0, 0));
    Ok(Revision {
        response,
        at,
        facts_version: tx.query_row("PRAGMA user_version", [], |r| r.get(0))?,
    })
}

fn read_realms(tx: &rusqlite::Transaction) -> Result<Vec<String>> {
    let mut stmt =
        tx.prepare("SELECT realm FROM tabs UNION SELECT realm FROM characters ORDER BY 1")?;
    let rows = stmt.query_map([], |r| r.get(0))?;
    Ok(rows.collect::<Result<_, _>>()?)
}

/// The realm and league of a listing live in its params and default as
/// `stash_basis` and `character_basis` default them; the bare `fetched_at`
/// is the row `MAX(id)` chose, which SQLite guarantees.
fn read_listings(tx: &rusqlite::Transaction, realm: Option<&str>) -> Result<Vec<ListingSeen>> {
    let mut stmt = tx.prepare(
        "SELECT COALESCE(json_extract(params, '$.realm'), 'pc'),
                CASE WHEN endpoint = 'stashes' THEN COALESCE(json_extract(params, '$.league'), 'Standard') END,
                CASE WHEN endpoint = 'stashes' THEN 'stash' ELSE 'character' END,
                MAX(id), fetched_at
           FROM responses
          WHERE endpoint IN ('stashes', 'characters') AND status BETWEEN 200 AND 299
            AND (?1 IS NULL OR COALESCE(json_extract(params, '$.realm'), 'pc') = ?1)
          GROUP BY 1, 2, 3 ORDER BY 1, 3 DESC, 2",
    )?;
    let rows = stmt.query_map([realm], |r| {
        Ok(ListingSeen {
            realm: r.get(0)?,
            league: r.get(1)?,
            kind: r.get(2)?,
            response_id: r.get(3)?,
            fetched_at: r.get(4)?,
        })
    })?;
    Ok(rows.collect::<Result<_, _>>()?)
}

/// When a tab left: its own retirement, or its parent's, which takes a
/// substash with it though the substash keeps its row (C54).
const TAB_GONE: &str =
    "COALESCE(t.removed_at, (SELECT p.removed_at FROM tabs p WHERE p.realm = t.realm
                                  AND p.league = t.league AND p.id = t.parent))";

fn read_locations(
    tx: &rusqlite::Transaction,
    realm: Option<&str>,
    membership: Membership,
) -> Result<Vec<LocationRow>> {
    let all = membership == Membership::All;
    let mut locations = Vec::new();
    let mut tabs = tx.prepare(&format!(
        "SELECT t.realm, t.league, 'stash', t.id, t.parent, COALESCE(t.name, ''), t.listed_at, t.fetched_at, t.type, {TAB_GONE}
           FROM tabs t WHERE (?2 OR {TAB_GONE} IS NULL) AND (?1 IS NULL OR t.realm = ?1)
          ORDER BY t.realm, t.league, {TAB_ORDER_KEYS}"
    ))?;
    let mut characters = tx.prepare(
        "SELECT c.realm, c.league, 'character', c.id, NULL, c.name, c.listed_at, c.fetched_at, NULL, c.removed_at
           FROM characters c WHERE (?2 OR c.removed_at IS NULL) AND (?1 IS NULL OR c.realm = ?1)
          ORDER BY c.realm, c.league, c.level DESC, c.name",
    )?;
    for stmt in [&mut tabs, &mut characters] {
        let rows = stmt.query_map(rusqlite::params![realm, all], |r| {
            Ok(LocationRow {
                realm: r.get(0)?,
                league: r.get(1)?,
                kind: r.get(2)?,
                id: r.get(3)?,
                parent: r.get(4)?,
                name: r.get(5)?,
                listed_at: r.get(6)?,
                fetched_at: r.get(7)?,
                tab_type: r.get(8)?,
                removed_at: r.get(9)?,
            })
        })?;
        for row in rows {
            locations.push(row?);
        }
    }
    Ok(locations)
}

/// Every id in scope a live read leaves out, with when it left: removed
/// items, retired tabs — a substash that went with its tab among them —
/// and retired characters.
fn read_gone(tx: &rusqlite::Transaction, realm: Option<&str>) -> Result<Vec<Gone>> {
    let mut stmt = tx.prepare(&format!(
        "SELECT realm, 'item', id, removed_at FROM items
          WHERE removed_at IS NOT NULL AND (?1 IS NULL OR realm = ?1)
         UNION ALL
         SELECT t.realm, 'stash', t.id, {TAB_GONE} FROM tabs t
          WHERE {TAB_GONE} IS NOT NULL AND (?1 IS NULL OR t.realm = ?1)
         UNION ALL
         SELECT realm, 'character', id, removed_at FROM characters
          WHERE removed_at IS NOT NULL AND (?1 IS NULL OR realm = ?1)
         ORDER BY 2, 3, 1"
    ))?;
    let rows = stmt.query_map([realm], |r| {
        Ok(Gone {
            realm: r.get(0)?,
            kind: r.get(1)?,
            id: r.get(2)?,
            removed_at: r.get(3)?,
        })
    })?;
    Ok(rows.collect::<Result<_, _>>()?)
}

impl Store {
    /// The revision of the facts as they stand: what a consumer holding a
    /// corpus compares before every answer (C98).
    pub fn revision(&self) -> Result<Revision> {
        let tx = self.conn.unchecked_transaction()?;
        let revision = read_revision(&tx)?;
        tx.finish()?;
        Ok(revision)
    }

    /// Read every live item of the scope under one snapshot: `read` is
    /// handed the header and then the items, and what it returns is
    /// returned. A facts file that records no account identity is refused,
    /// as the snapshots refuse it.
    pub fn read_corpus<T>(
        &self,
        realm: RealmScope<'_>,
        read: impl FnOnce(&CorpusHeader, &mut CorpusItems<'_>) -> Result<T>,
    ) -> Result<T> {
        self.read_corpus_of(realm, Membership::Live, read)
    }

    /// [`Store::read_corpus`] over a membership: under `All`, every item
    /// the file holds at the place it was last seen, each removed one with
    /// when it left, and the retired locations in the header, marked.
    pub fn read_corpus_of<T>(
        &self,
        realm: RealmScope<'_>,
        membership: Membership,
        read: impl FnOnce(&CorpusHeader, &mut CorpusItems<'_>) -> Result<T>,
    ) -> Result<T> {
        let realm = match realm {
            RealmScope::One(realm) => Some(realm),
            RealmScope::All => None,
        };
        let tx = self.conn.unchecked_transaction()?;
        // First, so that the number names the snapshot everything below is
        // read from.
        let revision = read_revision(&tx)?;
        let (account_uuid, account_name) = self.account_identity(&tx)?;
        let header = CorpusHeader {
            revision,
            account_uuid,
            account_name,
            realms: read_realms(&tx)?,
            listings: read_listings(&tx, realm)?,
            locations: read_locations(&tx, realm, membership)?,
            gone: match membership {
                Membership::Live => read_gone(&tx, realm)?,
                Membership::All => Vec::new(),
            },
        };
        let out = {
            let mut stmt = tx.prepare(match membership {
                Membership::Live => ITEMS_SQL,
                Membership::All => ALL_ITEMS_SQL,
            })?;
            let mut items = CorpusItems {
                rows: stmt.query([realm])?,
            };
            read(&header, &mut items)?
        };
        tx.finish()?;
        Ok(out)
    }
}

#[cfg(test)]
#[path = "../unit/tests/corpus.rs"]
mod tests;
