//! C53 — the read economy's bound, held once: how many like entities a
//! list in an answer holds when the request names no number, and the one
//! arithmetic every maker applies to it.
//!
//! It lives here because the store is the one crate every maker links
//! (C1: the search links the store and the planner, the planner the
//! protocol crate and the store, the verbs all of them). The store itself
//! cuts nothing with it: a store read returns what its caller asked for,
//! and the maker that answers a frontend bounds the list it answers.

/// C53 — how many like entities a list holds by default, on every
/// surface: the rows a search returns where no limit is given, the tabs
/// `tabs` lists, and every other list an answer carries beside its
/// counts. A request's `limit` names more or fewer, its `expand` the
/// whole.
pub const LISTED: usize = 10;

/// C53 — a list cut to what the request asked: the whole where `expand`
/// is asked (whatever `limit` says), else the first `limit` rows, else
/// the first [`LISTED`]. Returns the rows kept, in the order given, and
/// how many it left out. The caller orders the list first: the cut keeps
/// its head.
pub fn bounded<T>(mut rows: Vec<T>, limit: Option<usize>, expand: bool) -> (Vec<T>, usize) {
    if expand {
        return (rows, 0);
    }
    let keep = limit.unwrap_or(LISTED);
    let left_out = rows.len().saturating_sub(keep);
    rows.truncate(keep);
    (rows, left_out)
}

#[cfg(test)]
#[path = "../unit/tests/bound.rs"]
mod tests;
