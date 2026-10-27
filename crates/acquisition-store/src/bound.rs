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
mod tests {
    use super::*;

    fn of(n: usize) -> Vec<usize> {
        (0..n).collect()
    }

    #[test]
    fn c53_a_list_holds_the_bound_by_default_and_counts_the_rest() {
        let (kept, left_out) = bounded(of(LISTED + 7), None, false);
        assert_eq!(kept, of(LISTED));
        assert_eq!(left_out, 7);
        // fewer than the bound: all of them, nothing left out
        let (kept, left_out) = bounded(of(3), None, false);
        assert_eq!((kept, left_out), (of(3), 0));
    }

    #[test]
    fn c53_a_limit_names_more_or_fewer_zero_included() {
        let (kept, left_out) = bounded(of(40), Some(25), false);
        assert_eq!((kept, left_out), (of(25), 15));
        let (kept, left_out) = bounded(of(40), Some(2), false);
        assert_eq!((kept, left_out), (of(2), 38));
        let (kept, left_out) = bounded(of(40), Some(0), false);
        assert_eq!((kept, left_out), (Vec::new(), 40));
        // a limit past the list keeps the list
        let (kept, left_out) = bounded(of(4), Some(25), false);
        assert_eq!((kept, left_out), (of(4), 0));
    }

    #[test]
    fn c53_expand_is_the_whole_whatever_the_limit() {
        assert_eq!(bounded(of(40), None, true), (of(40), 0));
        assert_eq!(bounded(of(40), Some(2), true), (of(40), 0));
    }

    #[test]
    fn c53_left_out_is_the_difference() {
        for n in [0, 1, LISTED - 1, LISTED, LISTED + 1, 3 * LISTED] {
            for limit in [None, Some(0), Some(1), Some(LISTED), Some(2 * LISTED)] {
                let (kept, left_out) = bounded(of(n), limit, false);
                assert_eq!(kept.len() + left_out, n, "{n} rows, limit {limit:?}");
                assert_eq!(kept, of(kept.len()), "the head is kept, in order");
            }
        }
    }
}
