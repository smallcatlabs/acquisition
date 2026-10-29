//! The item events summarised by location (C53: what changed where, the
//! decision view): a derivation over the store's event rows (C34, facts
//! through the store), moved here from the CLI so a second frontend that
//! asks for it reads the same counts.
//!
//! Both forms are one document each (C149, every answer one object, its
//! lists counted beside their rows): [`Events`], the rows as the store
//! reads them, and [`EventSummary`], those rows by location, each with
//! the counts the CLI's text prints, under C53's bound of ten.

use std::collections::{BTreeMap, HashMap};

use acquisition_store::bound::{LISTED, bounded};
use acquisition_store::{EventRow, Store};
use serde::Serialize;

use crate::refusal::Arguments;

/// How far back a window reaches when the request names none: a day,
/// the one default both frontends read.
pub const DEFAULT_HOURS: f64 = 24.0;

/// C149 and C53: the item events of a window as one document — the
/// newest first, as many as the request's `limit` or ten, beside how
/// many it returned and whether the limit was reached (then the window
/// may hold more: a larger `limit` reads them). The window's total is
/// not counted: the store's read is bounded by the limit and has none
/// (step 2g's brief, round 3; a count read waits for a reader who asks).
#[derive(Clone, Debug, Serialize)]
pub struct Events {
    /// How far back the window reaches, in hours.
    pub hours: f64,
    /// The most events the read returns.
    pub limit: usize,
    /// How many it returned.
    pub returned: usize,
    /// Whether it returned as many as `limit`: the window may hold more.
    pub limit_reached: bool,
    /// The events, newest first.
    pub rows: Vec<EventRow>,
}

impl Events {
    /// The newest `limit` (else ten) events of the last `hours` before
    /// `now`, from the store's own read (C34), each event's time said as
    /// an age at `now` (C53). An `hours` below zero or not a number is
    /// refused as `arguments` (C47: a request's input is a structured
    /// error, never a panic); a window past the epoch reads every event,
    /// the cast and the subtraction saturating (#91: `-1e300` hours
    /// overflowed the subtraction).
    pub fn read(
        store: &Store,
        now: i64,
        hours: f64,
        limit: Option<usize>,
    ) -> anyhow::Result<Events> {
        if hours.is_nan() || hours < 0.0 {
            return Err(Arguments(format!(
                "`hours` is how far back from now the events are read, 0 or more, and this one is negative or not a number: name 0 or more hours, or leave it out for the last {DEFAULT_HOURS} hours"
            ))
            .into());
        }
        let limit = limit.unwrap_or(LISTED);
        let since = now.saturating_sub((hours * 3600.0) as i64);
        let rows: Vec<EventRow> = store
            .events_since(since, limit)?
            .into_iter()
            .map(|row| row.aged(now))
            .collect();
        Ok(Events {
            hours,
            limit,
            returned: rows.len(),
            limit_reached: rows.len() == limit,
            rows,
        })
    }
}

/// The per-kind totals of a summary, over every location row: `moved`
/// counts the moves in (one move is one location's out and another's
/// in).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct KindTotals {
    pub added: usize,
    pub changed: usize,
    pub moved: usize,
    pub removed: usize,
}

/// C149 and C53: the same events by location — a second cut with its
/// own accounting, since one move touches two locations, so ten events
/// can make twenty location rows: the events summarized and the
/// locations counted apart, the per-kind totals made over every location
/// before the rows are cut, then ten locations, every one under
/// `expand`.
#[derive(Clone, Debug, Serialize)]
pub struct EventSummary {
    pub hours: f64,
    pub limit: usize,
    /// The events summarized: those the read returned.
    pub events: usize,
    pub limit_reached: bool,
    pub by_kind: KindTotals,
    /// Every location the events touched.
    pub locations: usize,
    /// The locations listed, the newest activity first: ten, or every
    /// one under `expand`.
    pub rows: Vec<LocationChanges>,
    /// The locations not in `rows`.
    pub left_out: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_out_needs: Option<&'static str>,
}

impl EventSummary {
    /// `events` grouped by location (`summary`), counted whole, then cut,
    /// each listed location's latest event said as an age at `now`, the
    /// answer's time (C53).
    pub fn of(events: &Events, rows: Vec<LocationChanges>, expand: bool, now: i64) -> EventSummary {
        let by_kind = KindTotals {
            added: rows.iter().map(|r| r.added).sum(),
            changed: rows.iter().map(|r| r.changed).sum(),
            moved: rows.iter().map(|r| r.moved_in).sum(),
            removed: rows.iter().map(|r| r.removed).sum(),
        };
        let locations = rows.len();
        let (rows, left_out) = bounded(rows, None, expand);
        let rows = rows
            .into_iter()
            .map(|mut row| {
                row.last_age = Some(acquisition_store::age::ago(now, Some(row.last)));
                row
            })
            .collect();
        EventSummary {
            hours: events.hours,
            limit: events.limit,
            events: events.returned,
            limit_reached: events.limit_reached,
            by_kind,
            locations,
            rows,
            left_out,
            left_out_needs: (left_out > 0).then_some("expand"),
        }
    }
}

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
    /// C53: `last` in words at the answer's time, on a location a
    /// summary lists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_age: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn moved(n: usize, at: i64) -> EventRow {
        EventRow {
            at,
            at_age: None,
            item_id: format!("i{n}"),
            kind: "moved".into(),
            from_location: Some(format!("stash/pc/Standard/from{n:02}")),
            to_location: Some(format!("stash/pc/Standard/to{n:02}")),
            name: None,
            type_line: None,
        }
    }

    /// The second review of the brief, finding 4: one move touches two
    /// locations, so ten events can make twenty location rows, whose first
    /// ten cannot say the whole. The summary counts the events and the
    /// locations apart, makes its per-kind totals over every location
    /// before it cuts, and lists the rest under `expand`.
    #[test]
    fn c149_ten_moves_between_twenty_locations_are_counted_whole_beside_ten() {
        let rows: Vec<EventRow> = (0..10).map(|n| moved(n, 1_000 + n as i64)).collect();
        let events = Events {
            hours: 24.0,
            limit: LISTED,
            returned: rows.len(),
            limit_reached: true,
            rows,
        };
        let locations = summarize(&events.rows, &mut |_| None);
        assert_eq!(locations.len(), 20);
        let summary = EventSummary::of(&events, locations.clone(), false, 2_000);
        assert_eq!(summary.events, 10);
        assert_eq!(summary.locations, 20);
        assert_eq!(summary.rows.len(), LISTED);
        assert_eq!(summary.left_out, 10);
        assert_eq!(summary.left_out_needs, Some("expand"));
        assert_eq!(
            summary.by_kind,
            KindTotals {
                moved: 10,
                ..KindTotals::default()
            }
        );
        let doc = serde_json::to_value(&summary).unwrap();
        assert!(doc.is_object(), "{doc}");
        let whole = EventSummary::of(&events, locations, true, 2_000);
        assert_eq!((whole.rows.len(), whole.left_out), (20, 0));
        assert_eq!(whole.by_kind, summary.by_kind);
        assert_eq!(whole.left_out_needs, None);
    }

    /// C53 (an age the text prints stands in words beside its epoch,
    /// made at the answer's own time): each listed location says its
    /// latest event as `acq store events` prints it, under `last_age`.
    #[test]
    fn c53_a_listed_location_says_its_latest_event_as_an_age() {
        let rows: Vec<EventRow> = vec![moved(0, 1_000), moved(1, 1_000 - 3_600)];
        let events = Events {
            hours: 24.0,
            limit: LISTED,
            returned: rows.len(),
            limit_reached: false,
            rows,
        };
        let summary = EventSummary::of(
            &events,
            summarize(&events.rows, &mut |_| None),
            false,
            1_045,
        );
        let ages: Vec<(i64, Option<&str>)> = summary
            .rows
            .iter()
            .map(|r| (r.last, r.last_age.as_deref()))
            .collect();
        assert_eq!(
            ages,
            [
                (1_000, Some("45s ago")),
                (1_000, Some("45s ago")),
                (-2_600, Some("60m ago")),
                (-2_600, Some("60m ago")),
            ]
        );
    }
}
