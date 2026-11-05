//! The bodies the store refused as malformed, listed as one document
//! (C149, every answer one object, its lists counted beside their rows):
//! the rows the store's listing returns, newest first, under `rows`,
//! beside `returned`, the `limit` the request named and `total`, every
//! body on record. The rows are the
//! store's own, their bodies left on disk; a redesign of what a row says
//! (#18) is additive on this object.

use acquisition_search::SearchError;
use acquisition_store::{Refused, Store};
use serde::Serialize;

/// The refused bodies listed: at most `limit`, newest first.
#[derive(Clone, Debug, Serialize)]
pub struct RefusedBodies {
    /// The most rows the listing returns.
    pub limit: usize,
    /// How many it returned.
    pub returned: usize,
    /// How many are on record, `returned` among them (C146: whether more
    /// exist than were returned).
    pub total: usize,
    pub rows: Vec<Refused>,
}

impl RefusedBodies {
    /// The store's listing of its refused bodies, at most `limit`, each
    /// row's fetch said as an age at `now`, the answer's time (C53).
    pub fn read(store: &Store, limit: usize, now: i64) -> anyhow::Result<RefusedBodies> {
        let rows: Vec<Refused> = store
            .refused_list(limit)?
            .into_iter()
            .map(|row| row.aged(now))
            .collect();
        Ok(RefusedBodies {
            limit,
            returned: rows.len(),
            total: store.refused_total()?,
            rows,
        })
    }

    /// One refused body in full, its fetch aged at `now` (C53), by the
    /// row id the listing gave. An id the store holds no body under is
    /// refused as `refused_body_not_found`, the kind `show` gives an id
    /// it cannot find (`item_not_found`), in words a person at a terminal
    /// and an agent both can follow (C143: the MCP speaks the command
    /// as its tool).
    pub fn one(store: &Store, id: i64, now: i64) -> anyhow::Result<Refused> {
        match store.refused(id)? {
            Some(row) => Ok(row.aged(now)),
            None => Err(SearchError::Scope {
                kind: "refused_body_not_found",
                message: format!("no refused body with id {id} (see `acq store refused`)"),
                offers: Vec::new(),
            }
            .into()),
        }
    }
}
