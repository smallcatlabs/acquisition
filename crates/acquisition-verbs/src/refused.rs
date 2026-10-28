//! The bodies the store refused as malformed, listed as one document
//! (C149, every answer one object, its lists counted beside their rows):
//! the rows the store's listing returns, newest first, under `rows`,
//! beside `returned` and the `limit` the request named. The rows are the
//! store's own, their bodies left on disk; a redesign of what a row says
//! (#18) is additive on this object.

use acquisition_store::{Refused, Store};
use serde::Serialize;

/// The refused bodies listed: at most `limit`, newest first.
#[derive(Clone, Debug, Serialize)]
pub struct RefusedBodies {
    /// The most rows the listing returns.
    pub limit: usize,
    /// How many it returned.
    pub returned: usize,
    pub rows: Vec<Refused>,
}

impl RefusedBodies {
    /// The store's listing of its refused bodies, at most `limit`.
    pub fn read(store: &Store, limit: usize) -> anyhow::Result<RefusedBodies> {
        let rows = store.refused_list(limit)?;
        Ok(RefusedBodies {
            limit,
            returned: rows.len(),
            rows,
        })
    }
}
