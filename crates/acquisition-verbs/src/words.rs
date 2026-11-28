//! Words over a verb's document that take the protocol's realm, which
//! the search crate cannot name (C1, the crate edges): a league's place
//! in a message and the tabs' first line, made once here for every door
//! that says them (C46, shared semantics in Rust; C53, text is a function
//! of the JSON).

use acquisition_protocol::realm::Realm;

/// `xbox/` before a league in a message; nothing for pc, as on the wire.
pub fn realm_prefix(realm: Realm) -> String {
    match realm.segment() {
        Some(seg) => format!("{seg}/"),
        None => String::new(),
    }
}

/// `tabs`' first line: the league's counts, the answer first (C53). Its
/// items are the tabs' alone and it says so, with `characters` the
/// command that counts the rest where the surface prints one: a league's
/// count elsewhere holds its characters' items too (C146, a count says
/// what it counts).
pub fn tabs_head(
    counts: &acquisition_search::tabs::TabCounts,
    realm: Realm,
    league: &str,
    characters: Option<&str>,
) -> String {
    let mut parts = vec![format!("{} fetched", counts.fetched)];
    if counts.never_fetched > 0 {
        parts.push(format!("{} never fetched", counts.never_fetched));
    }
    if counts.folders > 0 {
        parts.push(format!(
            "{} folder{} (never fetched)",
            counts.folders,
            if counts.folders == 1 { "" } else { "s" }
        ));
    }
    format!(
        "{} tabs in {}{league}: {}; {} items in tabs (characters apart{})",
        counts.tabs,
        realm_prefix(realm),
        parts.join(", "),
        counts.items,
        characters
            .map(|line| format!(": `{line}`"))
            .unwrap_or_default()
    )
}
