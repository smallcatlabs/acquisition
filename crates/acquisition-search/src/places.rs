//! A league's places as `acq web`'s "Stash tabs" and "Characters" pages
//! list them (4-full part 1, step 6.5a; the design's §3, "a group is a
//! place"): its stash tabs as a tree in stash order, or its characters in
//! the store's order, each place its store row whole, beside the league's
//! counts; and, asked a filter or a query (step 6.5b), which of them the
//! page lists and how many of the query's matches each holds.
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
//! A folder holds tabs and no items, and carries none. Its view is the
//! one asked ([`Asking::view`]): a page's sort orders the rows inside
//! each place, and the places keep their order.
//!
//! The league's counts stand once, as `tabs` counts them
//! ([`TabCounts`]), or as `characters` does ([`CharacterCounts`], the
//! characters document's counter, held here so the search crate makes
//! the places whole). No CLI flag and no MCP tool answer this document:
//! a `tab` count already gives an agent a place's request, and the page
//! is its one reader. Nothing here is cut by the read economy's bound
//! (C53): the page is a selection, and a place it does not list cannot
//! be ticked.
//!
//! # Asked: a filter and a query (step 6.5b)
//!
//! [`Asking`] carries a filter, a query's text and a view. The document
//! still lists every place, so a tick or an open on a place the page
//! does not draw is still a place of the league; asked a filter or a
//! query, each place says whether it is `listed`, and each level counts
//! what it `folded`, and of that the never fetched. A place is listed
//! where it passes the filter ([`Filter`]: words its name holds, case
//! aside; its tab type, whole; never fetched) and, under a query, holds
//! a match; or where it holds a listed place. Unasked, none of these
//! keys stands, and the document is 6.5a's byte for byte.
//!
//! Under a query (a text, refused as the search refuses it) a place's
//! request is the query and its terms, and its `matched` is that
//! request's `total.matched`: a folder's is its places' sum, the top
//! level's sum is the document's `query.matched`, and a tab with pages
//! says the matches in the tab itself apart (`own`). The document
//! carries the canonical text, the query's own request over the realm
//! and its `total`, and the basis. `matched` is counted once, not by an
//! answer per place: the query is evaluated over the corpus once, and
//! each match is tried against the terms of each place whose id it
//! carries (its own, its place's, its place's parent's, the ids an
//! `id:` term compares). The tests hold each count to its request's
//! answer.
//!
//! The document stands at the corpus's basis (C98): the corpus is the
//! holder's, the listing is read, and [`Corpus::is_current`] is asked
//! after it; where the store moved, both are read again, three times and
//! then an error, as `Corpus::read` does.

use std::collections::HashMap;
use std::sync::Arc;

use acquisition_store::{Annotations, CharacterRow, Store, TabRow};
use serde::Serialize;

use crate::answer::{Context, Membership, QueryInput, Request, Rows, Scope, Shape, Total, View};
use crate::bind::Query;
use crate::corpus::{Basis, Corpus, Held, Holder, REREADS, Realm, moved};
use crate::error::SearchError;
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
    /// The filter asked, as read; absent where none was.
    #[serde(skip_serializing_if = "Filter::is_empty")]
    pub filter: Filter,
    /// The query asked, where one was.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Queried>,
    /// The basis the counts under a query were made at (C98).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis: Option<Basis>,
    /// The top level, in stash order or the store's.
    pub places: Vec<Place>,
    /// What the top level folds, where a filter or a query was asked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folded: Option<Holds>,
}

/// What a places document is asked beside its league: a filter, a query
/// and the view each place's request takes. The default asks nothing.
#[derive(Debug, Clone, Default)]
pub struct Asking {
    pub filter: Filter,
    /// The query's text; absent or blank, no query.
    pub query: Option<String>,
    /// The view every place's request takes, the query's own too: a
    /// page's `view.value`, `view.sort` and `view.desc`.
    pub view: Rows,
}

impl Asking {
    /// The query's text, where one was asked.
    pub fn text(&self) -> Option<&str> {
        self.query
            .as_deref()
            .map(str::trim)
            .filter(|t| !t.is_empty())
    }

    /// Whether anything folds a place: a filter or a query.
    fn folds(&self) -> bool {
        !self.filter.is_empty() || self.text().is_some()
    }
}

/// A filter over the places (step 6.5b): each field present must pass.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Filter {
    /// Words the name holds, each, case aside.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A tab's type, whole, case aside.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetched: Option<Fetched>,
}

/// How a filter asks a place's fetch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Fetched {
    /// Never fetched: a folder, never fetched and holding no items, is
    /// none.
    Never,
}

impl Filter {
    /// A filter from its three fields as a page sends them: a blank field
    /// asks nothing; `fetched` is `never` or refused.
    pub fn read(
        name: Option<&str>,
        r#type: Option<&str>,
        fetched: Option<&str>,
    ) -> Result<Filter, SearchError> {
        let given = |field: Option<&str>| {
            field
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .map(str::to_string)
        };
        let fetched = match given(fetched).as_deref() {
            None => None,
            Some(word) if word.eq_ignore_ascii_case("never") => Some(Fetched::Never),
            Some(word) => {
                return Err(SearchError::scope(
                    "filter",
                    format!(
                        "`{word}` is no way a place was fetched: the filter asks for the places never fetched, `never`, or for none"
                    ),
                ));
            }
        };
        Ok(Filter {
            name: given(name),
            r#type: given(r#type),
            fetched,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.r#type.is_none() && self.fetched.is_none()
    }

    /// Whether `place` passes every field asked.
    fn passes(&self, place: &Place) -> bool {
        let name = self.name.as_deref().is_none_or(|words| {
            let held = place.name().to_lowercase();
            words
                .split_whitespace()
                .all(|word| held.contains(&word.to_lowercase()))
        });
        let typed = self
            .r#type
            .as_deref()
            .is_none_or(|wanted| match &place.row {
                Row::Tab(tab) => tab.r#type.eq_ignore_ascii_case(wanted),
                Row::Character(_) => false,
            });
        let fetched = match self.fetched {
            None => true,
            Some(Fetched::Never) => place.never_fetched(),
        };
        name && typed && fetched
    }
}

/// The query a places document was asked: its canonical text, its own
/// request over the realm (every league's places, the view asked) and
/// that request's `total`, and the matches in this league's places.
#[derive(Debug, Clone, Serialize)]
pub struct Queried {
    pub text: String,
    pub request: Request,
    pub total: Total,
    /// The matches the league's places hold: the top level's sum.
    pub matched: usize,
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
/// holds where it holds places, and the request that lists its items;
/// asked a filter or a query, whether the page lists it and what it
/// folds; under a query, its matches.
#[derive(Debug, Clone, Serialize)]
pub struct Place {
    pub place: Kind,
    #[serde(flatten)]
    pub row: Row,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holds: Option<Holds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<Request>,
    /// Asked a filter or a query: whether the page lists the place.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listed: Option<bool>,
    /// Under a query: its request's `total.matched`; a folder's, its
    /// places' sum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched: Option<usize>,
    /// Under a query, a tab that holds pages: the matches in the tab
    /// itself, beside its pages'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub own: Option<usize>,
    /// Asked a filter or a query, a parent: the places it holds that the
    /// page does not list, and of them those never fetched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folded: Option<Holds>,
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

/// Places counted, and of them those never fetched: what a parent holds,
/// or what a level folds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Holds {
    pub places: usize,
    pub never_fetched: usize,
}

impl Holds {
    fn of<'a>(places: impl Iterator<Item = &'a Place>) -> Holds {
        let mut holds = Holds {
            places: 0,
            never_fetched: 0,
        };
        for place in places {
            holds.places += 1;
            holds.never_fetched += usize::from(place.never_fetched());
        }
        holds
    }
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

    /// Whether the page lists it: unasked, every place.
    pub fn is_listed(&self) -> bool {
        self.listed != Some(false)
    }
}

/// Where a places document reads: the store, the intent its corpus is
/// priced by, and the holder of the corpus a query is answered over.
#[derive(Clone, Copy)]
pub struct Reading<'a> {
    pub store: &'a Store,
    pub intent: &'a Annotations,
    pub holder: &'a Holder,
}

impl Places {
    /// The league's stash tabs as a tree in stash order, from one read
    /// snapshot (`Store::tabs_document`): the rows, the facts block the
    /// counts carry (#172), and the realm's leagues, which say whether a
    /// tab's league term must name its spelling; asked a filter and a
    /// query, each marked (the module doc).
    pub fn tabs(
        at: Reading,
        realm: &str,
        league: &str,
        asking: &Asking,
        now: i64,
    ) -> Result<Places, SearchError> {
        let (asked, (rows, coverage, held)) = read_asked(at, realm, asking, || {
            at.store
                .tabs_document(realm, league, now)
                .map_err(SearchError::store)
        })?;
        let mut counts = TabCounts::of(&rows);
        counts.span = coverage.span;
        let folded = crate::bind::folded(league);
        let twinned = held
            .iter()
            .filter_map(|h| h.league.as_deref())
            .any(|other| other != league && crate::bind::folded(other) == folded);
        let make = |row: &TabRow| crate::counts::place_terms(&row.id, Some(&row.league), twinned);
        let places = tab_tree(rows, now, &|row| {
            request(
                realm,
                asked.as_ref().map(|a| &a.query),
                make(row),
                &asking.view,
            )
        });
        Places {
            realm: realm.to_string(),
            league: league.to_string(),
            counts: Counted::Tabs(Box::new(counts)),
            filter: asking.filter.clone(),
            query: None,
            basis: None,
            places,
            folded: None,
        }
        .marked(
            asking,
            asked,
            &|row| match row {
                Row::Tab(row) => make(row),
                Row::Character(row) => crate::counts::place_terms(&row.id, None, false),
            },
            now,
        )
    }

    /// The league's characters in the store's order; asked a filter and a
    /// query, each marked (the module doc). A character has no type, so a
    /// filter that asks one is refused.
    pub fn characters(
        at: Reading,
        realm: &str,
        league: &str,
        asking: &Asking,
        now: i64,
    ) -> Result<Places, SearchError> {
        if asking.filter.r#type.is_some() {
            return Err(SearchError::scope(
                "filter",
                "a character has no type: the characters are filtered by their name, or by those never fetched",
            ));
        }
        let (asked, rows) = read_asked(at, realm, asking, || {
            at.store
                .characters(Some(realm), Some(league))
                .map_err(SearchError::store)
        })?;
        let terms = |id: &str| crate::counts::place_terms(id, None, false);
        Places {
            realm: realm.to_string(),
            league: league.to_string(),
            counts: Counted::Characters(CharacterCounts::of(&rows)),
            filter: asking.filter.clone(),
            query: None,
            basis: None,
            places: rows
                .into_iter()
                .map(|row| Place {
                    place: Kind::Character,
                    holds: None,
                    request: Some(request(
                        realm,
                        asked.as_ref().map(|a| &a.query),
                        terms(&row.id),
                        &asking.view,
                    )),
                    listed: None,
                    matched: None,
                    own: None,
                    folded: None,
                    places: Vec::new(),
                    row: Row::Character(row.aged(now)),
                })
                .collect(),
            folded: None,
        }
        .marked(
            asking,
            asked,
            &|row| match row {
                Row::Tab(row) => crate::counts::place_terms(&row.id, Some(&row.league), false),
                Row::Character(row) => terms(&row.id),
            },
            now,
        )
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

    /// The place `id` and each place holding it, top first (step 6.5c,
    /// Decision 6): what a page opens to show the place's items, a
    /// folder's tab or a map tab's page under its tab; none where the
    /// document holds no place `id`.
    pub fn holders(&self, id: &str) -> Option<Vec<&str>> {
        fn walk<'a>(places: &'a [Place], id: &str, path: &mut Vec<&'a str>) -> bool {
            for place in places {
                path.push(place.id());
                if place.id() == id || walk(&place.places, id, path) {
                    return true;
                }
                path.pop();
            }
            false
        }
        let mut path = Vec::new();
        walk(&self.places, id, &mut path).then_some(path)
    }

    /// The places marked as `asking` asks (the module doc): under a
    /// query, each place's matches counted over the corpus, then, under a
    /// filter or a query, each place listed or folded and each level's
    /// fold counted. `terms` gives a place's own terms, its request's
    /// without the query.
    fn marked(
        mut self,
        asking: &Asking,
        asked: Option<Asked>,
        terms: &dyn Fn(&Row) -> Vec<Node>,
        now: i64,
    ) -> Result<Places, SearchError> {
        if let Some(asked) = asked {
            let matched = asked.matched(&mut self.places, terms)?;
            let mut own = asked.request.clone();
            // the total alone: no row and no order
            own.view = View::from(Shape::Rows(Rows {
                limit: Some(0),
                sort: Some(crate::answer::STORE_ORDER.to_string()),
                ..Rows::default()
            }));
            let answered = crate::answer::unaged(&asked.corpus, &own)?.aged(now);
            let mut request = asked.request;
            request.query.text = Some(answered.query.text.clone());
            self.query = Some(Queried {
                text: answered.query.text,
                request,
                total: answered.total,
                matched,
            });
            self.basis = Some(asked.corpus.basis.clone());
        }
        if asking.folds() {
            let queried = self.query.is_some();
            self.folded = Some(fold(&mut self.places, &asking.filter, queried));
        }
        Ok(self)
    }
}

/// A query asked of a places document: its own request, bound, and the
/// corpus the document stands at.
struct Asked {
    request: Request,
    query: Query,
    corpus: Arc<Corpus>,
}

impl Asked {
    /// Each place's `matched` and a tab's `own` (the module doc), from
    /// one evaluation of the query: the matches by the ids they carry,
    /// each tried against the place's terms; a folder's its places' sum.
    /// The matches the top level holds.
    fn matched(
        &self,
        places: &mut [Place],
        terms: &dyn Fn(&Row) -> Vec<Node>,
    ) -> Result<usize, SearchError> {
        let mut carrying: HashMap<&str, Vec<&Held>> = HashMap::new();
        for held in self.corpus.items() {
            if !crate::answer::is_true(&self.query, held) {
                continue;
            }
            let place = &held.place;
            let mut ids = vec![held.item.facts.id.as_str(), place.id.as_str()];
            ids.extend(place.parent.as_ref().map(|p| p.id.as_str()));
            ids.sort_unstable();
            ids.dedup();
            for id in ids {
                carrying.entry(id).or_default().push(held);
            }
        }
        fn walk(
            places: &mut [Place],
            carrying: &HashMap<&str, Vec<&Held>>,
            terms: &dyn Fn(&Row) -> Vec<Node>,
        ) -> Result<usize, SearchError> {
            let mut sum = 0;
            for place in places {
                let inner = walk(&mut place.places, carrying, terms)?;
                let matched = match place.place {
                    Kind::Folder => inner,
                    _ => {
                        let id = place.id();
                        let bound = crate::bind::bind(&node(terms(&place.row)))?;
                        let held: Vec<&&Held> = carrying
                            .get(id)
                            .into_iter()
                            .flatten()
                            .filter(|held| crate::answer::is_true(&bound, held))
                            .collect();
                        if place.place == Kind::Tab && place.holds.is_some() {
                            place.own = Some(held.iter().filter(|h| h.place.id == id).count());
                        }
                        held.len()
                    }
                };
                place.matched = Some(matched);
                sum += matched;
            }
            Ok(sum)
        }
        walk(places, &carrying, terms)
    }
}

/// Each place marked listed or folded, deepest first (the module doc),
/// each parent's fold counted: what this level folds.
fn fold(places: &mut [Place], filter: &Filter, queried: bool) -> Holds {
    for place in places.iter_mut() {
        let inner = fold(&mut place.places, filter, queried);
        if place.holds.is_some() {
            place.folded = Some(inner);
        }
        let own = filter.passes(place) && (!queried || place.matched.unwrap_or(0) > 0);
        place.listed = Some(own || place.places.iter().any(Place::is_listed));
    }
    Holds::of(places.iter().filter(|p| !p.is_listed()))
}

/// The listing read under `asking`: where it asks a query, the query's
/// own request checked (refused as the search refuses it), then the
/// listing read at the corpus's basis ([`steady`]).
fn read_asked<T>(
    at: Reading,
    realm: &str,
    asking: &Asking,
    read: impl FnMut() -> Result<T, SearchError>,
) -> Result<(Option<Asked>, T), SearchError> {
    let mut read = read;
    let Some(text) = asking.text() else {
        return Ok((None, read()?));
    };
    let request = Request {
        scope: Scope {
            account: None,
            realm: Some(Realm::One(realm.to_string())),
            membership: Membership::Live,
        },
        query: QueryInput {
            text: Some(text.to_string()),
            tree: None,
            trade: None,
        },
        view: View::from(Shape::Rows(asking.view.clone())),
        context: Context::Matches,
        basis: None,
    };
    request.check()?;
    let query = request.bind()?;
    let (corpus, listed) = steady(at, &request, |_| read())?;
    Ok((
        Some(Asked {
            request,
            query,
            corpus,
        }),
        listed,
    ))
}

/// The holder's corpus for `request`'s scope and a read made over it at
/// its basis (C98): the corpus, then `read` given it, then
/// [`Corpus::is_current`]; where the store moved between, both again,
/// three times (`corpus::REREADS`), then the error a moving store is.
/// The places document stands at its corpus's basis through it, and a
/// page that answers over the corpus beside the document reads both
/// inside one `read`, so the page stands at one basis (step 6.5b, the
/// review of f36033f5: a page's segments read a corpus of their own
/// after the document).
pub fn steady<T>(
    at: Reading,
    request: &Request,
    mut read: impl FnMut(&Corpus) -> Result<T, SearchError>,
) -> Result<(Arc<Corpus>, T), SearchError> {
    for _ in 0..REREADS {
        let corpus = at.holder.corpus(request, at.store, at.intent)?;
        let listed = read(&corpus)?;
        if corpus.is_current(at.store, at.intent)? {
            return Ok((corpus, listed));
        }
    }
    Err(moved())
}

/// The spelling a place page names a league by, where a query named it
/// in another case (`league=` is any-case; the review of f36033f5, its
/// second finding): of the leagues `held` that `=` takes `named` for, the
/// one held as written, else the one alone, else `named` as written, so
/// that two spellings held apart are never chosen between.
pub fn held_spelling<'a>(named: &'a str, held: &'a [String]) -> &'a str {
    let folded = crate::bind::folded(named);
    let mut taken = held.iter().filter(|h| crate::bind::folded(h) == folded);
    match (taken.next(), taken.next()) {
        _ if held.iter().any(|h| h == named) => named,
        (Some(one), None) => one,
        _ => named,
    }
}

/// Terms as one node: one alone, several and-ed.
fn node(mut terms: Vec<Node>) -> Node {
    match terms.len() {
        1 => terms.remove(0),
        _ => Node::All(terms),
    }
}

/// The request that lists a place's items: its terms over its realm, the
/// live items, in the view asked; under a query, the query's terms and
/// the place's, and-ed.
fn request(realm: &str, query: Option<&Query>, terms: Vec<Node>, view: &Rows) -> Request {
    let node = match query.map(|q| q.tree().clone()) {
        None => node(terms),
        Some(Node::All(mut and)) => {
            and.extend(terms);
            Node::All(and)
        }
        Some(root) => Node::All([vec![root], terms].concat()),
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
        view: View::from(Shape::Rows(view.clone())),
        context: Context::Matches,
        basis: None,
    }
}

/// The tabs nested: a row whose parent is a listed map or unique tab is
/// a page under it, a row whose parent is a listed folder a tab in it,
/// and a row whose parent the store does not hold a page at the top.
fn tab_tree(rows: Vec<TabRow>, now: i64, make: &dyn Fn(&TabRow) -> Request) -> Vec<Place> {
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
    fn build(
        (kind, row): (Kind, TabRow),
        under: &mut HashMap<String, Vec<(Kind, TabRow)>>,
        make: &dyn Fn(&TabRow) -> Request,
        now: i64,
    ) -> Place {
        let places: Vec<Place> = under
            .remove(&row.id)
            .unwrap_or_default()
            .into_iter()
            .map(|child| build(child, under, make, now))
            .collect();
        let holds = (!places.is_empty()).then(|| Holds::of(places.iter()));
        Place {
            place: kind,
            holds,
            request: (kind != Kind::Folder).then(|| make(&row)),
            listed: None,
            matched: None,
            own: None,
            folded: None,
            places,
            row: Row::Tab(row.aged(now)),
        }
    }
    top.into_iter()
        .map(|place| build(place, &mut under, make, now))
        .collect()
}

#[cfg(test)]
mod tests {
    use acquisition_store::{Annotations, Endpoint, Store};
    use serde_json::{Value, json};

    use super::{Reading, steady};
    use crate::answer::{QueryInput, Request};
    use crate::corpus::Holder;
    use crate::error::SearchError;

    /// A tab of pc Standard fetched holding `items`, at `at`.
    fn fetched(s: &mut Store, items: &[&str], at: i64) {
        let items: Vec<Value> = items
            .iter()
            .map(|id| {
                json!({ "id": id, "name": "Doom Loop", "typeLine": "Iron Ring", "baseType": "Iron Ring",
                        "rarity": "Rare", "frameTypeId": "Rare", "identified": true, "ilvl": 84, "x": 0, "y": 0 })
            })
            .collect();
        s.record(
            &Endpoint::Stash {
                realm: "pc".into(),
                league: "Standard".into(),
                id: "t1".into(),
                sub: None,
            },
            &json!({ "realm": "pc", "league": "Standard", "id": "t1" }),
            200,
            &json!({ "stash": { "id": "t1", "name": "Rings", "type": "PremiumStash", "items": items } }),
            at,
        )
        .unwrap();
    }

    /// A league a query named in another case is the spelling the store
    /// holds; held as written, or held twice apart, it is the name as
    /// written (the review of f36033f5, its second finding).
    #[test]
    fn a_league_named_in_another_case_is_the_spelling_held() {
        use super::held_spelling;
        let one = ["Standard".to_string(), "Hardcore".to_string()];
        assert_eq!(held_spelling("standard", &one), "Standard");
        assert_eq!(held_spelling("Standard", &one), "Standard");
        assert_eq!(held_spelling("Settlers", &one), "Settlers");
        let twins = ["Standard".to_string(), "standard".to_string()];
        assert_eq!(held_spelling("standard", &twins), "standard");
        assert_eq!(held_spelling("STANDARD", &twins), "STANDARD");
    }

    /// C98 (step 6.5b, Decision 2): a write between the listing's read
    /// and the check reads both again, so the listing and the corpus
    /// stand at one basis; a store that moves at every read is an error
    /// after three, as a corpus's read is.
    #[test]
    fn a_write_between_the_listing_and_the_check_reads_both_again() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("facts.db");
        let mut writer = Store::open(&path).unwrap();
        writer
            .record(
                &Endpoint::Profile,
                &json!({}),
                200,
                &json!({ "uuid": "u-1", "name": "A" }),
                1,
            )
            .unwrap();
        writer
            .record(
                &Endpoint::Stashes {
                    realm: "pc".into(),
                    league: "Standard".into(),
                },
                &json!({ "realm": "pc", "league": "Standard" }),
                200,
                &json!({ "stashes": [{ "id": "t1", "name": "Rings", "type": "PremiumStash" }] }),
                2,
            )
            .unwrap();
        fetched(&mut writer, &["a"], 3);
        let reader = Store::open(&path).unwrap();
        let intent = Annotations::open_memory_for("u-1").unwrap();
        let holder = Holder::new();
        let at = Reading {
            store: &reader,
            intent: &intent,
            holder: &holder,
        };
        let request = Request {
            query: QueryInput {
                text: Some("rarity=rare".to_string()),
                ..QueryInput::default()
            },
            ..Request::default()
        };
        let mut reads = 0;
        let (corpus, listing) = steady(at, &request, |_| {
            reads += 1;
            let listing = reader
                .tabs_document("pc", "Standard", 10)
                .map_err(SearchError::store)?;
            if reads == 1 {
                fetched(&mut writer, &["a", "b"], 4 + reads);
            }
            Ok(listing)
        })
        .unwrap();
        assert_eq!(reads, 2, "read again after the write");
        assert!(corpus.is_current(&reader, &intent).unwrap());
        assert_eq!(corpus.items().len(), 2);
        assert_eq!(
            listing.0[0].item_count, 2,
            "the listing at the corpus's basis"
        );

        let mut reads = 0;
        let moving = steady(at, &request, |_| {
            reads += 1;
            fetched(&mut writer, &["a"], 10 + reads);
            Ok(())
        });
        assert_eq!(moving.unwrap_err().kind(), "basis_moved");
        assert_eq!(usize::try_from(reads).unwrap(), super::REREADS);
    }
}
