//! C162 — coverage and age are one derivation over a set of places, held
//! once here: given places with their observations (each location's
//! kind, type, league and fetch, the item count at each, the listing
//! bases), how many were fetched and never, the never fetched by type and
//! by league, the oldest and newest fetch with their locations, and when
//! the lists were seen, aged at the answer's time by one method
//! ([`Coverage::aged`]). The fetch and list part is any set's
//! ([`FetchSpan`]), computed by [`span`] over the places and listing bases
//! a caller names: [`of`] folds a scope's through it, and an answer its
//! subject's.
//!
//! It lives here because the store is the one crate every maker links
//! (C1), as the age vocabulary ([`crate::age`]) and the read economy's
//! bound ([`crate::bound`]) do, and because what it folds is the store's
//! own ingest facts: a location's fetch and listing, which come from the
//! store's read (C103). It reads no body (C108: the corpus read derives
//! nothing a body says), and no policy enters it (C39: the store exposes
//! neutral snapshots, never half a planner): a caller passes the places
//! it answers about, and what membership adds to the result (the removed,
//! the leagues its items sit in) is the caller's.

use serde::Serialize;

use crate::age;
use crate::corpus::{ListingSeen, LocationRow, RealmScope};

/// A location the scope names by its fetch: `stash` or `character`, its
/// id, its name as GGG gives it (none where it gives no text), and its
/// league.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FetchedLocation {
    pub kind: String,
    pub id: String,
    pub name: Option<String>,
    pub league: Option<String>,
}

impl FetchedLocation {
    fn of(l: &LocationRow) -> FetchedLocation {
        FetchedLocation {
            kind: l.kind.clone(),
            id: l.id.clone(),
            name: (!l.name.trim().is_empty()).then(|| l.name.clone()),
            league: l.league.clone(),
        }
    }
}

/// What the scope block says was searched (C96).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Coverage {
    /// The live items in scope.
    pub items: usize,
    /// Under membership `all` and `removed`, the removed items: each is
    /// a row marked with when it left.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<usize>,
    /// Every realm the store holds, whatever the scope.
    pub realms_held: Vec<String>,
    pub leagues: Vec<String>,
    /// Locations that hold items and have been fetched.
    pub fetched: usize,
    /// Locations a listing names and no fetch has filled, folders apart.
    pub never_fetched: usize,
    /// The types that have a location never fetched, the most never
    /// fetched first, each counted as `tabs` counts a type: its
    /// locations, how many were fetched — the never fetched are the
    /// difference — and the live items in scope there (C118). A tab's
    /// type is GGG's, verbatim, as `tabs` gives it; a character's is
    /// `character`. Over the scope's realm in every league, never
    /// narrowed by the query, since what a never-fetched location holds
    /// is unknown: the words are C146's.
    pub never_fetched_by_type: Vec<TabType>,
    /// The leagues holding a tab never fetched, the most first: names
    /// alone, so the league a `tabs` pointer names and how many others
    /// there are follow from the answer (C118).
    pub never_fetched_leagues: Vec<String>,
    pub folders: usize,
    /// The fetch and list ages of the scope's locations, flattened at
    /// their place in the scope's JSON.
    #[serde(flatten)]
    pub span: FetchSpan,
}

impl Coverage {
    /// C53: the scope's fetch and list ages said at `now`, the answer's
    /// time — the oldest and newest fetch where there are fetches, as the
    /// text says them, and when the lists were seen, `never` where none
    /// was.
    pub fn aged(&mut self, now: i64) {
        self.span.aged(now);
    }
}

/// The part of a coverage any set of places with their observations has:
/// the oldest and newest fetch of its fetched places, each with its
/// location, and when the lists that name it were seen ([`span`]).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FetchSpan {
    /// The earliest and the latest of the fetched locations' last
    /// fetches, each one location's; a row's place carries its own.
    pub oldest_fetch: Option<i64>,
    /// C53: `oldest_fetch` in words at the answer's time, where there is
    /// one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_fetch_age: Option<String>,
    /// The location whose last fetch is `oldest_fetch` (#11: the line
    /// named an age and no place), the first in the store's order on a
    /// tie.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_fetch_location: Option<FetchedLocation>,
    pub newest_fetch: Option<i64>,
    /// C53: `newest_fetch` in words at the answer's time, where there is
    /// one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newest_fetch_age: Option<String>,
    /// The location whose last fetch is `newest_fetch`, as for the oldest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newest_fetch_location: Option<FetchedLocation>,
    /// When the oldest of the location lists the scope reads was last
    /// seen: under a query that fixes one league, that league's tab list
    /// and the realm's character list; otherwise every list of the realm
    /// (the re-sit: a `league=Standard` query said 41 days where
    /// Standard's own list was 3 days old, mcp-opus its fault 2).
    pub list_seen: Option<i64>,
    /// C53: `list_seen` in words at the answer's time, `never` where no
    /// list was seen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_seen_age: Option<String>,
}

impl FetchSpan {
    /// C53: the span's ages said at `now`, the answer's time — the oldest
    /// and newest fetch where there are fetches, and when the lists were
    /// seen, `never` where none was.
    pub fn aged(&mut self, now: i64) {
        self.oldest_fetch_age = self.oldest_fetch.map(|at| age::ago(now, Some(at)));
        self.newest_fetch_age = self.newest_fetch.map(|at| age::ago(now, Some(at)));
        self.list_seen_age = Some(age::ago(now, self.list_seen));
    }
}

/// One tab type's counts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TabType {
    #[serde(rename = "type")]
    pub kind: String,
    pub tabs: usize,
    pub fetched: usize,
    pub items: i64,
}

pub fn is_folder(location: &LocationRow) -> bool {
    location.tab_type.as_deref() == Some("Folder")
}

/// The coverage of a set of places over `realm`: `locations` each paired
/// with the live items at it, in the store's order (the oldest and the
/// newest fetch are the first in that order on a tie), the listing bases
/// `listings`, the realms the store holds and the live item total, both
/// passed through as given. `removed` is none and `leagues` the
/// locations'; what membership makes of either is the caller's.
pub fn of(
    realm: RealmScope<'_>,
    locations: &[(&LocationRow, i64)],
    listings: &[ListingSeen],
    realms_held: &[String],
    items: usize,
) -> Coverage {
    let holds_items: Vec<(&LocationRow, i64)> = locations
        .iter()
        .copied()
        .filter(|(l, _)| !is_folder(l))
        .collect();
    let fetches = || holds_items.iter().filter_map(|(l, _)| l.fetched_at);
    let mut never_fetched_by_type = by_type(holds_items.iter().map(|(l, here)| {
        let kind = match l.kind.as_str() {
            "character" => "character",
            _ => l.tab_type.as_deref().unwrap_or_default(),
        };
        (kind, l.fetched_at.is_some(), *here)
    }));
    never_fetched_by_type.retain(|t| t.tabs > t.fetched);
    let mut by_league: Vec<(&str, usize)> = Vec::new();
    for league in holds_items
        .iter()
        .filter(|(l, _)| l.kind == "stash" && l.fetched_at.is_none())
        .filter_map(|(l, _)| l.league.as_deref())
    {
        match by_league.iter_mut().find(|(at, _)| *at == league) {
            Some((_, n)) => *n += 1,
            None => by_league.push((league, 1)),
        }
    }
    by_league.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    never_fetched_by_type.sort_by(|a, b| {
        (b.tabs - b.fetched)
            .cmp(&(a.tabs - a.fetched))
            .then_with(|| a.kind.cmp(&b.kind))
    });
    let mut leagues: Vec<String> = locations
        .iter()
        .filter_map(|(l, _)| l.league.clone())
        .collect();
    leagues.sort();
    leagues.dedup();
    Coverage {
        items,
        removed: None,
        realms_held: realms_held.to_vec(),
        leagues,
        fetched: fetches().count(),
        never_fetched: holds_items.len() - fetches().count(),
        never_fetched_by_type,
        never_fetched_leagues: by_league.into_iter().map(|(l, _)| l.to_string()).collect(),
        folders: locations.len() - holds_items.len(),
        span: span(
            &holds_items
                .iter()
                .filter_map(|(l, _)| Some((l.fetched_at?, FetchedLocation::of(l))))
                .collect::<Vec<_>>(),
            &listings
                .iter()
                .filter(|l| match realm {
                    RealmScope::One(only) => l.realm == only,
                    RealmScope::All => true,
                })
                .cloned()
                .collect::<Vec<_>>(),
        ),
    }
}

/// The fetch span of a set of places: `fetched` its fetched places, each
/// with its last fetch, in the caller's order (the oldest and the newest
/// are the first in that order on a tie), and `listings` the listing
/// bases that name the set, which the caller chooses (C162: a set's
/// observations are its caller's to name); `list_seen` is the oldest of
/// them. Unaged: [`FetchSpan::aged`] says it at the answer's time.
pub fn span(fetched: &[(i64, FetchedLocation)], listings: &[ListingSeen]) -> FetchSpan {
    // the first on a tie, both ways: `min_by_key` keeps the first,
    // `max_by_key` the last, so the newest reads the reversed order
    let oldest = fetched.iter().min_by_key(|(at, _)| *at);
    let newest = fetched.iter().rev().max_by_key(|(at, _)| *at);
    FetchSpan {
        oldest_fetch: oldest.map(|(at, _)| *at),
        oldest_fetch_age: None,
        oldest_fetch_location: oldest.map(|(_, l)| l.clone()),
        newest_fetch: newest.map(|(at, _)| *at),
        newest_fetch_age: None,
        newest_fetch_location: newest.map(|(_, l)| l.clone()),
        list_seen: listings.iter().map(|l| l.fetched_at).min(),
        list_seen_age: None,
    }
}

/// The counts by type, the most items first, of locations each given as
/// its type, whether it was fetched, and its items: the one counter of
/// `tabs` and of a search's scope, which names the never fetched by the
/// same types (C118).
pub fn by_type<'a>(locations: impl IntoIterator<Item = (&'a str, bool, i64)>) -> Vec<TabType> {
    let mut by_type: Vec<TabType> = Vec::new();
    for (kind, fetched, items) in locations {
        let at = match by_type.iter().position(|k| k.kind == kind) {
            Some(at) => at,
            None => {
                by_type.push(TabType {
                    kind: kind.to_string(),
                    tabs: 0,
                    fetched: 0,
                    items: 0,
                });
                by_type.len() - 1
            }
        };
        let counted = &mut by_type[at];
        counted.tabs += 1;
        counted.fetched += usize::from(fetched);
        counted.items += items;
    }
    by_type.sort_by(|a, b| b.items.cmp(&a.items).then_with(|| a.kind.cmp(&b.kind)));
    by_type
}

#[cfg(test)]
#[path = "../unit/tests/coverage.rs"]
mod tests;
