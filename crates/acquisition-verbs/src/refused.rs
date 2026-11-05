//! The bodies the store refused as malformed, listed as one document
//! (C149, every answer one object, its lists counted beside their rows):
//! the rows the store's listing returns, newest first, under `rows`,
//! beside `returned`, the `limit` the request named and `total`, every
//! body on record. The rows are the
//! store's own, their bodies left on disk; a redesign of what a row says
//! (#18) is additive on this object.

use crate::refusal::{Pointed, Way};
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
    /// it cannot find (`item_not_found`), in the request's words (C143),
    /// offering the list as `account`'s, which each surface spells.
    pub fn one(store: &Store, id: i64, now: i64, account: &str) -> anyhow::Result<Refused> {
        match store.refused(id)? {
            Some(row) => Ok(row.aged(now)),
            None => Err(Pointed {
                kind: "refused_body_not_found",
                message: format!("no refused body with id {id}; their list names each by its id"),
                ways: vec![(
                    Way::RefusedBodies {
                        account: account.to_string(),
                    },
                    "the refused bodies, each with its id".to_string(),
                )],
            }
            .into()),
        }
    }
}
