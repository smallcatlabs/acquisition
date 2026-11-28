//! A facts line's words: the counts and the fetch span every facts line
//! says (C162), `price status`'s, a shop render's, `store status`'s and
//! the tabs' and characters' pages' alike, and a table's items cell.

use acquisition_store::age::ago;

/// A facts line's fetched part (C162): how many places were fetched and
/// the extremes of their last fetches, each with its own age, the
/// document's own where it holds one, said at `now` otherwise (C53).
/// The one formatter of it, for every facts line: `price status`'s, a
/// shop render's, `store status`'s and `tabs`'.
pub fn fetched_words(
    fetched: usize,
    span: &acquisition_store::coverage::FetchSpan,
    now: i64,
) -> String {
    if fetched == 0 {
        return "none fetched".to_string();
    }
    let said = |age: &Option<String>, at: Option<i64>| age.clone().unwrap_or_else(|| ago(now, at));
    format!(
        "{fetched} fetched ({})",
        super::span_words(
            span,
            said(&span.oldest_fetch_age, span.oldest_fetch),
            said(&span.newest_fetch_age, span.newest_fetch),
        )
    )
}

/// `tabs`' facts line: the league's tabs, how many were fetched
/// and their extremes, and when its stash listing was seen, in the words
/// the other facts lines print ([`fetched_words`]).
pub fn tabs_facts_line(counts: &crate::tabs::TabCounts, now: i64) -> String {
    let places = counts.tabs - counts.folders;
    let listing = match counts.span.list_seen {
        Some(at) => format!(
            "stash listing {}",
            counts
                .span
                .list_seen_age
                .clone()
                .unwrap_or_else(|| ago(now, Some(at)))
        ),
        None => "stash never listed".to_string(),
    };
    // an empty league says when its listing was seen, or that it never
    // was
    if counts.tabs == 0 {
        return format!("facts: no tabs; {listing}\n");
    }
    format!(
        "facts: {}, {}; {listing}\n",
        super::plural(places, "tab", "tabs"),
        fetched_words(counts.fetched, &counts.span, now)
    )
}

/// The items column: `-` when never fetched (nothing is known), the live
/// count otherwise — a fetched-empty body reads `0`, never the same as
/// unfetched (C53).
pub fn items_cell(fetched_at: Option<i64>, item_count: i64) -> String {
    match fetched_at {
        None => "-".into(),
        Some(_) => item_count.to_string(),
    }
}

/// `store characters`' footer: the counts, in words (C146), the
/// characters page's counts line too.
pub fn characters_counts_line(counts: &crate::places::CharacterCounts) -> String {
    let mut summary = format!("{} fetched", counts.fetched);
    if counts.empty_bodies > 0 {
        summary.push_str(&format!(" ({} with empty bodies)", counts.empty_bodies));
    }
    if counts.never_fetched > 0 {
        summary.push_str(&format!(", {} never fetched", counts.never_fetched));
    }
    format!(
        "{} characters: {summary}; {} items\n",
        counts.characters, counts.items
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_items_cell_tells_unfetched_from_empty() {
        assert_eq!(items_cell(None, 0), "-");
        assert_eq!(items_cell(Some(1), 0), "0");
        assert_eq!(items_cell(Some(1), 7), "7");
    }
}
