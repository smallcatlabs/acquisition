//! A league's places as `acq web`'s "Stash tabs" and "Characters" pages
//! list them (4-full part 1, step 6.5a; the design's §3, "a group is a
//! place"): its stash tabs as a tree in stash order, or its characters in
//! the store's order, each place its store row whole, beside the league's
//! counts.
//!
//! # As built
//!
//! One maker each over one store read: [`Places::tabs`] over
//! `Store::tabs_document`'s snapshot, [`Places::characters`] over
//! `Store::characters`. The tree is the store's listing order nested: a
//! folder holds its tabs, a map or unique tab its pages (the rule
//! `Tabs::grouped` held until this module took it, its one reader the
//! page). A page whose tab the store does not hold stands at the top, a
//! page still. A parent counts the places it holds and those of them
//! never fetched (`holds`), so a page can say what a closed parent holds
//! without listing it.
//!
//! A place that can hold items carries the request that lists them
//! (`request`): `id:<its id>` over its realm, with the league for a tab
//! or a page, made by the one function a `tab` count's bucket makes its
//! route with (`counts::place_terms`), so over one realm the two are one
//! text, save where a second spelling of the league is listed and holds
//! no item: there the place names its spelling and the bucket does not,
//! and both select the same items (that function's doc says why).
//! A folder holds tabs and no items, and carries none.
//!
//! The league's counts stand once, as `tabs` counts them
//! ([`TabCounts`]), or as `characters` does ([`CharacterCounts`], the
//! characters document's counter, held here so the search crate makes
//! the places whole). No CLI flag and no MCP tool answer this document:
//! a `tab` count already gives an agent a place's request, and the page
//! is its one reader. Nothing here is cut by the read economy's bound
//! (C53): the page is a selection, and a place it does not list cannot
//! be ticked.

use std::collections::HashMap;

use acquisition_store::{CharacterRow, Store, TabRow};
use serde::Serialize;

use crate::answer::{Context, Membership, QueryInput, Request, Scope, View};
use crate::corpus::Realm;
use crate::tabs::TabCounts;
use crate::tree::Node;

/// A league's places, counted, then listed whole.
#[derive(Debug, Clone, Serialize)]
pub struct Places {
    pub realm: String,
    pub league: String,
    /// The league's counts: a tabs document's as `tabs` counts them, a
    /// characters document's as `characters` does.
    #[serde(flatten)]
    pub counts: Counted,
    /// The top level, in stash order or the store's.
    pub places: Vec<Place>,
}

/// The counts a places document stands beside.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Counted {
    Tabs(Box<TabCounts>),
    Characters(CharacterCounts),
}

/// Characters counted (C146, each count says what it counts): the
/// characters document's counts (`acquisition_verbs::Characters`) and
/// the places document's.
#[derive(Debug, Clone, Serialize)]
pub struct CharacterCounts {
    /// Every character the selection holds.
    pub characters: usize,
    /// Of them, those a full fetch (equipment and inventory) has filled.
    pub fetched: usize,
    /// Of the fetched, those whose last full fetch carried no items
    /// (`fetched_items` 0): fetched, and empty.
    pub empty_bodies: usize,
    /// Of them, those listed and never fetched: no answer says what they
    /// hold.
    pub never_fetched: usize,
    /// The live items the store holds at every one of them.
    pub items: i64,
}

impl CharacterCounts {
    pub fn of(rows: &[CharacterRow]) -> CharacterCounts {
        let characters = rows.len();
        let fetched = rows.iter().filter(|c| c.fetched_at.is_some()).count();
        CharacterCounts {
            characters,
            fetched,
            empty_bodies: rows
                .iter()
                .filter(|c| c.fetched_at.is_some() && c.fetched_items == Some(0))
                .count(),
            never_fetched: characters - fetched,
            items: rows.iter().map(|c| c.item_count).sum(),
        }
    }
}

/// What a place is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    /// A stash folder: it holds tabs and no items, and is never fetched.
    Folder,
    /// A stash tab, a map or unique tab among them.
    Tab,
    /// A page of a map or unique tab, fetched with its tab.
    Page,
    Character,
}

/// One place: its store row whole, aged at the answer's time, what it
/// holds where it holds places, and the request that lists its items.
#[derive(Debug, Clone, Serialize)]
pub struct Place {
    pub place: Kind,
    #[serde(flatten)]
    pub row: Row,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holds: Option<Holds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<Request>,
    /// The places it holds, in their order.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub places: Vec<Place>,
}

/// A place's store row.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Row {
    Tab(TabRow),
    Character(CharacterRow),
}

/// What a parent holds: its places, and of them those never fetched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Holds {
    pub places: usize,
    pub never_fetched: usize,
}

impl Place {
    pub fn id(&self) -> &str {
        match &self.row {
            Row::Tab(t) => &t.id,
            Row::Character(c) => &c.id,
        }
    }

    pub fn name(&self) -> &str {
        match &self.row {
            Row::Tab(t) => &t.name,
            Row::Character(c) => &c.name,
        }
    }

    pub fn fetched_at(&self) -> Option<i64> {
        match &self.row {
            Row::Tab(t) => t.fetched_at,
            Row::Character(c) => c.fetched_at,
        }
    }

    pub fn item_count(&self) -> i64 {
        match &self.row {
            Row::Tab(t) => t.item_count,
            Row::Character(c) => c.item_count,
        }
    }

    /// Never fetched: a folder apart, which is never fetched and holds
    /// no items.
    pub fn never_fetched(&self) -> bool {
        self.place != Kind::Folder && self.fetched_at().is_none()
    }
}

impl Places {
    /// The league's stash tabs as a tree in stash order, from one read
    /// snapshot (`Store::tabs_document`): the rows, the facts block the
    /// counts carry (#172), and the realm's leagues, which say whether a
    /// tab's league term must name its spelling.
    pub fn tabs(store: &Store, realm: &str, league: &str, now: i64) -> anyhow::Result<Places> {
        let (rows, coverage, held) = store.tabs_document(realm, league, now)?;
        let mut counts = TabCounts::of(&rows);
        counts.span = coverage.span;
        let folded = crate::bind::folded(league);
        let twinned = held
            .iter()
            .filter_map(|h| h.league.as_deref())
            .any(|other| other != league && crate::bind::folded(other) == folded);
        Ok(Places {
            realm: realm.to_string(),
            league: league.to_string(),
            counts: Counted::Tabs(Box::new(counts)),
            places: tab_tree(rows, realm, twinned, now),
        })
    }

    /// The league's characters in the store's order.
    pub fn characters(
        store: &Store,
        realm: &str,
        league: &str,
        now: i64,
    ) -> anyhow::Result<Places> {
        let rows = store.characters(Some(realm), Some(league))?;
        Ok(Places {
            realm: realm.to_string(),
            league: league.to_string(),
            counts: Counted::Characters(CharacterCounts::of(&rows)),
            places: rows
                .into_iter()
                .map(|row| Place {
                    place: Kind::Character,
                    holds: None,
                    request: Some(request(
                        realm,
                        crate::counts::place_terms(&row.id, None, false),
                    )),
                    places: Vec::new(),
                    row: Row::Character(row.aged(now)),
                })
                .collect(),
        })
    }

    /// Every place, each before the places it holds, in order.
    pub fn every(&self) -> Vec<&Place> {
        fn walk<'a>(places: &'a [Place], out: &mut Vec<&'a Place>) {
            for place in places {
                out.push(place);
                walk(&place.places, out);
            }
        }
        let mut out = Vec::new();
        walk(&self.places, &mut out);
        out
    }
}

/// The request that lists a place's items: its terms over its realm, the
/// live items, as a count's route is asked (the default view).
fn request(realm: &str, mut terms: Vec<Node>) -> Request {
    let node = match terms.len() {
        1 => terms.remove(0),
        _ => Node::All(terms),
    };
    Request {
        scope: Scope {
            account: None,
            realm: Some(Realm::One(realm.to_string())),
            membership: Membership::Live,
        },
        query: QueryInput {
            text: Some(crate::print::print(&node)),
            tree: None,
            trade: None,
        },
        view: View::default(),
        context: Context::Matches,
        basis: None,
    }
}

/// The tabs nested: a row whose parent is a listed map or unique tab is
/// a page under it, a row whose parent is a listed folder a tab in it,
/// and a row whose parent the store does not hold a page at the top.
fn tab_tree(rows: Vec<TabRow>, realm: &str, twinned: bool, now: i64) -> Vec<Place> {
    let types: HashMap<String, String> = rows
        .iter()
        .map(|r| (r.id.clone(), r.r#type.clone()))
        .collect();
    let container = |t: &str| matches!(t, "MapStash" | "UniqueStash");
    let mut top: Vec<(Kind, TabRow)> = Vec::new();
    let mut under: HashMap<String, Vec<(Kind, TabRow)>> = HashMap::new();
    for row in rows {
        let parent = row.parent.clone().filter(|p| *p != row.id);
        let kind = if acquisition_store::is_folder_type(&row.r#type) {
            Kind::Folder
        } else {
            match parent.as_deref().map(|p| types.get(p)) {
                Some(Some(t)) if container(t) => Kind::Page,
                // a parent the store does not hold: a page whose tab is
                // gone from the listing
                Some(None) => Kind::Page,
                _ => Kind::Tab,
            }
        };
        match parent.filter(|p| {
            types
                .get(p)
                .is_some_and(|t| container(t) || acquisition_store::is_folder_type(t))
        }) {
            Some(p) => under.entry(p).or_default().push((kind, row)),
            None => top.push((kind, row)),
        }
    }
    let league = |row: &TabRow| row.league.clone();
    fn build(
        (kind, row): (Kind, TabRow),
        under: &mut HashMap<String, Vec<(Kind, TabRow)>>,
        make: &dyn Fn(&TabRow) -> Option<Request>,
        now: i64,
    ) -> Place {
        let places: Vec<Place> = under
            .remove(&row.id)
            .unwrap_or_default()
            .into_iter()
            .map(|child| build(child, under, make, now))
            .collect();
        let holds = (!places.is_empty()).then(|| Holds {
            places: places.len(),
            never_fetched: places.iter().filter(|p| p.never_fetched()).count(),
        });
        Place {
            place: kind,
            holds,
            request: (kind != Kind::Folder).then(|| make(&row)).flatten(),
            places,
            row: Row::Tab(row.aged(now)),
        }
    }
    let make = |row: &TabRow| {
        Some(request(
            realm,
            crate::counts::place_terms(&row.id, Some(&league(row)), twinned),
        ))
    };
    top.into_iter()
        .map(|place| build(place, &mut under, &make, now))
        .collect()
}
