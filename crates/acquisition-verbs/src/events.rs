//! The item events summarised by location (C53: what changed where, the
//! decision view): a derivation over the store's event rows (C34, facts
//! through the store), moved here from the CLI so a second frontend that
//! asks for it reads the same counts.

use std::collections::{BTreeMap, HashMap};

use acquisition_store::{EventRow, Store};
use serde::Serialize;

/// One location's share of the events since a time, with the location's
/// name resolved from the store's tab and character rows.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct LocationChanges {
    pub location: String,
    pub name: Option<String>,
    pub added: usize,
    pub changed: usize,
    pub moved_in: usize,
    pub moved_out: usize,
    pub removed: usize,
    pub first: i64,
    pub last: i64,
}

/// C53 and C34: `events` grouped by location, the newest activity first,
/// each location named through the store's own reads.
pub fn summary(store: &Store, events: &[EventRow]) -> Vec<LocationChanges> {
    let mut namer = location_namer(store);
    summarize(events, &mut namer)
}

/// C53: group events by location, newest activity first. `names`
/// resolves a location string to what a person calls it.
pub fn summarize(
    events: &[EventRow],
    names: &mut dyn FnMut(&str) -> Option<String>,
) -> Vec<LocationChanges> {
    let mut by: BTreeMap<String, LocationChanges> = BTreeMap::new();
    let touch = |by: &mut BTreeMap<String, LocationChanges>, loc: &str, at: i64| {
        let e = by
            .entry(loc.to_string())
            .or_insert_with(|| LocationChanges {
                location: loc.to_string(),
                first: at,
                last: at,
                ..Default::default()
            });
        e.first = e.first.min(at);
        e.last = e.last.max(at);
    };
    for ev in events {
        match ev.kind.as_str() {
            "moved" => {
                if let Some(from) = &ev.from_location {
                    touch(&mut by, from, ev.at);
                    if let Some(e) = by.get_mut(from) {
                        e.moved_out += 1;
                    }
                }
                if let Some(to) = &ev.to_location {
                    touch(&mut by, to, ev.at);
                    if let Some(e) = by.get_mut(to) {
                        e.moved_in += 1;
                    }
                }
            }
            "removed" => {
                if let Some(from) = &ev.from_location {
                    touch(&mut by, from, ev.at);
                    if let Some(e) = by.get_mut(from) {
                        e.removed += 1;
                    }
                }
            }
            kind => {
                if let Some(to) = &ev.to_location {
                    touch(&mut by, to, ev.at);
                    if let Some(e) = by.get_mut(to) {
                        if kind == "added" {
                            e.added += 1;
                        } else {
                            e.changed += 1;
                        }
                    }
                }
            }
        }
    }
    let mut out: Vec<LocationChanges> = by.into_values().collect();
    for e in &mut out {
        e.name = names(&e.location);
    }
    out.sort_by(|a, b| {
        b.last
            .cmp(&a.last)
            .then_with(|| a.location.cmp(&b.location))
    });
    out
}

/// `stash/<realm>/<league>/<id>` → the tab's name; `character/<realm>/<id>`
/// → the character's — through the store's own row reads, cached per
/// listing so a thousand events cost a handful of queries.
fn location_namer(store: &Store) -> impl FnMut(&str) -> Option<String> + '_ {
    let mut tabs: HashMap<(String, String), HashMap<String, String>> = HashMap::new();
    let mut characters: HashMap<String, HashMap<String, String>> = HashMap::new();
    move |location: &str| {
        let parts: Vec<&str> = location.split('/').collect();
        match parts.as_slice() {
            ["stash", realm, league, id] => {
                let key = (realm.to_string(), league.to_string());
                let map = tabs.entry(key).or_insert_with(|| {
                    store
                        .tabs(realm, league)
                        .map(|rows| rows.into_iter().map(|t| (t.id, t.name)).collect())
                        .unwrap_or_default()
                });
                map.get(*id).cloned()
            }
            ["character", realm, id] => {
                let map = characters.entry(realm.to_string()).or_insert_with(|| {
                    store
                        .characters(Some(realm), None)
                        .map(|rows| rows.into_iter().map(|c| (c.id, c.name)).collect())
                        .unwrap_or_default()
                });
                map.get(*id).cloned()
            }
            _ => None,
        }
    }
}
