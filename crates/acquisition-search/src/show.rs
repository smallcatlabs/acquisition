//! `show <id>` (C100, C103): one item as the
//! search derives it — its fields, its place, every line with its kind,
//! template and numbers, what was unread — and the stored body on request,
//! which is still there when deriving fails.
//!
//! # As built
//!
//! The item is found by the search's own read (C108), every realm, in one
//! pass that derives the one item it is looking for: so its place is
//! joined as every answer joins it (S131), its basis is the snapshot it was
//! read from, and its body is the text as stored — `Store::item` parses
//! the body, turns one that does not parse into a null, and carries the
//! league the item was stamped with. What the item is socketed in is given
//! by id, which `show` accepts in turn. The read is over membership
//! `all` (`corpus.rs`): a removed item is shown as it
//! was last seen, at the place it left, marked with when
//! (`Facts::removed_at`), its price lacked; and an id that names a tab or
//! a character, retired or not, is said to be one, with the search that
//! lists what is in it. The item's price is the
//! listing state's (`price.rs`, C81), joined after the read as a corpus
//! joins every item's, under the same check that the facts and the intent
//! still stand at the basis. Its category stands beside its class
//! (`category.rs`): the ids a count lists it under, and what left an id
//! open.
//!
//! **One maker of what `show` carries of an item** (`ShownItem::of`):
//! the item without its lines, the lines with their slots and limits,
//! the class, the category as `show` says it and the price, made from a
//! held item (`corpus::Held`) — the one this read holds, and each one a
//! rows view asking `item` holds (`answer.rs`), so a row's item is
//! `show`'s for the same id at the same basis and no second maker stands.
//! [`item_aged`] ages both. It also says the values an item's numbers are
//! read by (4-full part 1, step 6.4c), each a text a sort takes: a line's
//! `value`, the value a trade filter on the stat it displays as compares,
//! asked of the translation itself, and for a line that displays as no
//! site stat the sum over its own group by the translation's rule and
//! writer (`line_value`); and `number_fields`, each number field the item
//! displays with its number. Where a field is displayed is a row's pieces
//! ([`Displays`]), made by the one function that composes the row
//! (`derive.rs`), so a page links a number without reading a text again.
//!
//! **`--against '<query>'`** (C100: the why-not) puts beside the item the query's canonical
//! text and tree, the root's outcome on this item — match, no match,
//! undecided — and the outcome of each node of the query that has one on
//! an item (`tree::nodes`: a term, a group of terms; never a condition
//! inside `line( … )`, which holds of one occurrence): a term's in the
//! terms block's words, a group's in the root's. A term shows what a row
//! shows of it where it matched, why where it is undecided, and what the
//! item has that failed it where it failed, each from the maker an answer
//! uses and bounded as a row is (`eval::against`). The item is asked as a
//! corpus asks each of its items — the same deriver, class, category and
//! price, the one evaluator — at the basis `show` names; it compares no
//! other basis. The query is read in the item's realm
//! (`QueryInput::read`): a site search whose URL names another realm is
//! refused, as a search in that realm never holds the item, and its
//! report stands beside the query it translated to, as in an answer.

use acquisition_store::corpus::{LocationRow, Membership, RealmScope};
use acquisition_store::{Annotations, Store};
use serde::Serialize;

use crate::answer::{QueryInput, QueryOut};
use crate::bind::{Bound, Query};
use crate::class::Classed;
use crate::corpus::Held;
use crate::corpus::{
    Basis, Place, REREADS, category_tables, celled, class_table, currency_table, locations, moved,
    placed, still_at, tab_type_unread,
};
use crate::derive::{Facts, Item, Line, derive};
use crate::error::SearchError;
use crate::eval::number_json;
use crate::eval::{self, Evidence, Outcome, Truth};
use crate::price::{self, Priced};

fn without_lines<S: serde::Serializer>(item: &Item, to: S) -> Result<S::Ok, S::Error> {
    let mut item = serde_json::to_value(item).map_err(serde::ser::Error::custom)?;
    if let Some(fields) = item.as_object_mut() {
        fields.shift_remove("lines");
    }
    item.serialize(to)
}

#[derive(Debug, Clone, Serialize)]
pub struct Shown {
    pub basis: Basis,
    pub place: Place,
    /// The item without its lines, which follow with their slots: no
    /// `lines` is printed under it, an empty list there having read as an
    /// item with none (the MCP seat, step 11c).
    #[serde(serialize_with = "without_lines")]
    pub item: Item,
    /// What the class table says of it (`class.rs`).
    pub class: Classed,
    /// Its category (`category.rs`): the id or ids a count lists it
    /// under, and the reason wherever an id is left open.
    pub category: crate::category::Shown,
    /// What the listing state prices it at (`price.rs`, C81).
    pub price: Priced,
    pub lines: Vec<ShownLine>,
    /// Each number field the item displays, with its number (as
    /// [`ShownItem::number_fields`]).
    #[serde(skip_serializing_if = "serde_json::Map::is_empty")]
    pub number_fields: serde_json::Map<String, serde_json::Value>,
    #[serde(skip)]
    displayed: Vec<(Displayed, String)>,
    /// Why the item does or does not match a query, when one is given.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub against: Option<Against>,
    /// The stored body as text, when asked for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// C163: `basis.snapshot.at` in words at the answer's time, at the
    /// path `show` declares for it: the basis is identity (C98) and
    /// carries no words ([`crate::answer::snapshot_age`]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_at_age: Option<String>,
}

/// What `show` carries of one item, made by the one maker
/// (`ShownItem::of`) that `show` and a rows view asking `item` both
/// call (C103, one deriver per fact): the item without its lines, which
/// follow with their slots, its class, its category as `show` says it and
/// its price, serialized flat in that order. A row carries it after its
/// own keys; [`Shown`] carries the same five beside its basis and place.
#[derive(Debug, Clone, Serialize)]
pub struct ShownItem {
    /// The item without its lines (as [`Shown::item`]).
    #[serde(serialize_with = "without_lines")]
    pub item: Item,
    /// What the class table says of it (`class.rs`).
    pub class: Classed,
    /// Its category (`category.rs`): the id or ids a count lists it
    /// under, and the reason wherever an id is left open.
    pub category: crate::category::Shown,
    /// What the listing state prices it at (`price.rs`, C81).
    pub price: Priced,
    pub lines: Vec<ShownLine>,
    /// Each number field the item displays, by the field's text, which
    /// sorts by it, with its number: the item level, the level required,
    /// and each property the property table reads as the number it
    /// displays, in the item's order (4-full part 1, step 6.4c). Absent
    /// where it displays none.
    #[serde(skip_serializing_if = "serde_json::Map::is_empty")]
    pub number_fields: serde_json::Map<String, serde_json::Value>,
    /// Where each of `number_fields` is displayed, for a page's pieces
    /// ([`ShownItem::property_row`]); not a key of the document.
    #[serde(skip)]
    displayed: Vec<(Displayed, String)>,
}

/// Where a number field is displayed: a value of a property, or of its
/// `a/b` one side; the item level's row; a value of a requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Displayed {
    Property(usize, usize, Side),
    ItemLevel,
    Requirement(usize, usize),
}

/// Which of a value a field reads: the whole, or a side of its `a/b`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Side {
    Whole,
    Numerator,
    Denominator,
}

/// An item's displayed rows with the number fields each displays, borrowed
/// from `show`'s document or a row's item (4-full part 1, step 6.4c):
/// each row in its pieces, a piece the item displays as one of
/// `number_fields` carrying that field's text, which a page writes as a
/// link without reading the row's text again.
#[derive(Clone, Copy)]
pub struct Displays<'a> {
    item: &'a Item,
    displayed: &'a [(Displayed, String)],
}

impl Displays<'_> {
    /// The value text the number displayed at `at` is read by, where one
    /// of `number_fields` is.
    fn read_at(&self, at: Displayed) -> Option<String> {
        self.displayed
            .iter()
            .find(|(place, _)| *place == at)
            .map(|(_, field)| field.clone())
    }

    /// The item's property at `at` (of `item.properties`) in its pieces,
    /// each value the item displays as one of `number_fields` with that
    /// field's text: what a page writes as a link.
    /// A value displayed `a/b` that a field reads a side of is three
    /// pieces, each side its own field's (`properties::split_pair`, the
    /// one reading of where they are).
    pub fn property_row(&self, at: usize) -> Vec<Part> {
        let Some(p) = self.item.properties.get(at) else {
            return Vec::new();
        };
        let mut parts = Vec::new();
        for piece in &p.pieces {
            let words = || Part {
                text: piece.text.clone(),
                value: None,
            };
            let Some(v) = piece.value else {
                parts.push(words());
                continue;
            };
            let side = |side| self.read_at(Displayed::Property(at, v, side));
            let (a, b) = (side(Side::Numerator), side(Side::Denominator));
            match crate::properties::split_pair(&piece.text).filter(|_| a.is_some() || b.is_some())
            {
                Some((top, bottom)) => {
                    parts.push(Part {
                        text: top.to_string(),
                        value: a,
                    });
                    parts.push(Part {
                        text: "/".to_string(),
                        value: None,
                    });
                    parts.push(Part {
                        text: bottom.to_string(),
                        value: b,
                    });
                }
                None => parts.push(Part {
                    value: side(Side::Whole),
                    ..words()
                }),
            }
        }
        parts
    }

    /// The item level's row in its pieces, the number with `ilvl`'s text;
    /// none where the item displays no level.
    pub fn item_level_row(&self) -> Vec<Part> {
        self.item.ilvl.map_or_else(Vec::new, |ilvl| {
            crate::derive::item_level_row(ilvl)
                .into_iter()
                .map(|piece| Part {
                    value: piece.value.and_then(|_| self.read_at(Displayed::ItemLevel)),
                    text: piece.text,
                })
                .collect()
        })
    }

    /// The requirements' row in its pieces, each requirement's number with
    /// the text of the field that reads it; none where it requires nothing.
    pub fn requires_row(&self) -> Vec<Part> {
        if self.item.requirements.is_empty() {
            return Vec::new();
        }
        crate::derive::requires_row(&self.item.requirements)
            .into_iter()
            .map(|(piece, of)| Part {
                value: of
                    .zip(piece.value)
                    .and_then(|(r, v)| self.read_at(Displayed::Requirement(r, v))),
                text: piece.text,
            })
            .collect()
    }
}

/// One piece of a displayed row (4-full part 1, step 6.4c): its words,
/// and the text of the value that reads them where one does. The row's
/// string is its pieces joined, by construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    pub text: String,
    pub value: Option<String>,
}

impl ShownItem {
    /// The one maker: what `show` carries of a held item — its lines taken
    /// out of it, each with its slots and its limit; its class; its
    /// category as `show` says it, made from the ids the corpus holds; its
    /// price. It takes the item whole, so a caller that borrows its
    /// `Held` from a corpus clones it first; the place stays the caller's
    /// (`show` gives a socketed item's parent by id, a corpus by name).
    pub(crate) fn of(held: Held, categories: &crate::category::Tables) -> ShownItem {
        let Held {
            mut item,
            class,
            category,
            price,
            place: _,
        } = held;
        let lines = std::mem::take(&mut item.lines)
            .into_iter()
            .map(|line| ShownLine {
                slots: line
                    .slots()
                    .into_iter()
                    .map(|(w, n)| (w, n.map_or(serde_json::Value::Null, number_json)))
                    .collect(),
                value: line_value(&line),
                limit: (line.source == "veiled").then_some(crate::describe::VEILED),
                line,
            })
            .collect();
        let displayed = number_fields(&item);
        ShownItem {
            category: categories.shown(&item, &class, &category),
            class,
            price,
            number_fields: displayed
                .iter()
                .map(|(_, field, n)| (field.clone(), number_json(*n)))
                .collect(),
            displayed: displayed
                .into_iter()
                .map(|(at, field, _)| (at, field))
                .collect(),
            item,
            lines,
        }
    }

    /// Where each number field the item displays is, for a page's pieces.
    pub fn displays(&self) -> Displays<'_> {
        Displays {
            item: &self.item,
            displayed: &self.displayed,
        }
    }

    /// C53 and C163: the item's first and last sighting and its removal,
    /// in words at `now` ([`item_aged`]).
    pub fn aged(&mut self, now: i64) {
        item_aged(&mut self.item, now);
    }
}

/// C53 (an age the text prints stands in words beside its epoch, made at
/// the answer's own time) and C163: an item's first and last sighting
/// and its removal, said at `now` — the one function `show` and a row's
/// item are aged by.
pub fn item_aged(item: &mut Item, now: i64) {
    use acquisition_store::age::ago;
    let facts = &mut item.facts;
    facts.first_seen_age = Some(ago(now, Some(facts.first_seen)));
    facts.last_seen_age = Some(ago(now, Some(facts.last_seen)));
    facts.removed_at_age = facts.removed_at.map(|at| ago(now, Some(at)));
}

#[derive(Debug, Clone, Serialize)]
pub struct ShownLine {
    #[serde(flatten)]
    pub line: Line,
    /// The words that name this occurrence's numbers, each with its number.
    pub slots: Vec<(String, serde_json::Value)>,
    /// The value a trade filter on this line's stat compares, which sorts
    /// by it (4-full part 1, step 6.4c): `sum(line("<template>"
    /// source=<source>).<slot>)`, its slot `avg` where the template's
    /// numbers are a mean pair and else `arg1`, the sum over every line of
    /// the item its group selects (`line_value`). Absent where that
    /// slot reads no number: a line with none, or one unread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// A limit this line meets, in the register's words (C102).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<&'static str>,
}

/// The value a line's number is read by (4-full part 1, step 6.4c): the
/// value a trade filter on the stat the line displays as compares, asked
/// of the translation itself (`trade_value`), so a stat the trade form
/// filtered and the same line on an item name one value — its group, its
/// forms and which of its numbers the site's text leaves open all the
/// translation's. A line that displays as no site stat is read by the
/// same rule over its own group, `sum(line("<template>"
/// source=<source>).<slot>)`, every number of it open
/// (`template::compared_slot`, the translation's rule, made by its writer
/// of a stat's sum, `tree::sum_of_lines`). None where that slot reads no
/// number of the line.
fn line_value(line: &Line) -> Option<String> {
    let (value, slot) = match trade_value(line) {
        Some(found) => found,
        None => {
            let count = crate::template::slots(&line.template).count;
            let open: Vec<usize> = (1..=count).collect();
            let slot = crate::template::compared_slot(&line.template, &open)?;
            let group = format!(
                "line({} source={})",
                crate::print::quoted_text(&line.template),
                crate::print::quoted_text(&line.source)
            );
            let query = crate::bind::parse_query(&group).ok()?;
            let value = crate::tree::sum_of_lines(std::slice::from_ref(query.tree()), &slot)?;
            (crate::print::print_value(&value), slot)
        }
    };
    line.slots()
        .iter()
        .find(|(word, n)| *word == slot && n.is_some())?;
    Some(value)
}

/// The site stat categories a line's source is filed under: an explicit
/// line, crafted and fractured ones among them, under `explicit` (the
/// catalogue's `stat.explicit`), and the three others by their own word.
fn category_of(source: &str) -> Option<&'static str> {
    match source {
        "explicit" => Some("explicit"),
        "implicit" => Some("implicit"),
        "enchant" => Some("enchant"),
        "scourge" => Some("scourge"),
        _ => None,
    }
}

/// The value the translation compares for a filter on the first site stat
/// a line displays as (`translate::displayed_as`), with its slot; none
/// where the line displays as none, or the translation compares no value
/// of it. Asked once per stat in a process and held: the translation is a
/// pure function of the build's tables.
fn trade_value(line: &Line) -> Option<(String, String)> {
    use std::collections::HashMap;
    use std::sync::{LazyLock, Mutex};
    type Held = HashMap<String, Option<(String, String)>>;
    static ASKED: LazyLock<Mutex<Held>> = LazyLock::new(|| Mutex::new(HashMap::new()));
    let category = category_of(&line.source)?;
    let stat = format!(
        "{category}.{}",
        crate::trade::translate::displayed_as(category, &line.template).first()?
    );
    if let Some(held) = ASKED.lock().ok().and_then(|held| held.get(&stat).cloned()) {
        return held;
    }
    let filter = serde_json::json!({ "query": { "stats": [{ "type": "and", "filters": [
        { "id": stat, "value": { "min": 1 } } ] }] } });
    let compared = crate::trade::decode(&filter.to_string())
        .and_then(|decoded| crate::trade::translate::translate(&decoded))
        .ok()
        .and_then(|translation| {
            crate::values::compared(&translation.tree)
                .into_iter()
                .next()
                .and_then(|one| match &one.value {
                    crate::tree::ValueRef::Sum { slot, .. } => Some((one.text, slot.clone())),
                    _ => None,
                })
        });
    if let Ok(mut held) = ASKED.lock() {
        held.insert(stat, compared.clone());
    }
    compared
}

/// Each number field an item displays, where it is displayed, by its
/// text, with its number, in the item's order: the property table's rows
/// that read a displayed number (`number`, `numerator`, `denominator`:
/// never a percent, a q20 or a tier, which the item does not display),
/// each where it read one property, its value the one the row names; the
/// item level; the level required. A displayed number two rows read is
/// the first's; an `a/b`'s two sides are two numbers.
fn number_fields(item: &Item) -> Vec<(Displayed, String, f64)> {
    use crate::properties::{FieldRead, Reads};
    use crate::tree::ValueRef;
    let text = |name: &str| crate::print::print_value(&ValueRef::Field(name.to_string()));
    let mut found: Vec<(Displayed, String, f64)> = Vec::new();
    if let Ok(table) = crate::properties::table() {
        for field in table.fields() {
            if !matches!(
                field.reads(),
                Reads::Number | Reads::Numerator | Reads::Denominator
            ) {
                continue;
            }
            let FieldRead::Value {
                value,
                shown,
                lines,
            } = crate::properties::read(field, item)
            else {
                continue;
            };
            let ([property], []) = (shown.as_slice(), lines.as_slice()) else {
                continue;
            };
            let at = |list: &[crate::derive::Property]| {
                list.iter().position(|p| std::ptr::eq(p, *property))
            };
            let side = match field.reads() {
                Reads::Numerator => Side::Numerator,
                Reads::Denominator => Side::Denominator,
                _ => Side::Whole,
            };
            let place = match (at(&item.properties), at(&item.requirements)) {
                (Some(i), _) => Displayed::Property(i, field.value_at(), side),
                (None, Some(i)) => Displayed::Requirement(i, field.value_at()),
                (None, None) => continue,
            };
            found.push((place, text(field.name()), value.as_f64()));
        }
    }
    if let Some(ilvl) = item.ilvl {
        found.push((Displayed::ItemLevel, text("ilvl"), ilvl as f64));
    }
    if let Some(level) = item.reqlevel
        && let Some(at) = item
            .requirements
            .iter()
            .position(|r| r.name.eq_ignore_ascii_case("Level"))
    {
        found.push((
            Displayed::Requirement(at, 0),
            text("reqlevel"),
            level as f64,
        ));
    }
    found.sort_by_key(|(at, ..)| *at);
    let mut seen = std::collections::BTreeSet::new();
    found.retain(|(at, ..)| seen.insert(*at));
    found
}

/// One item against one query (the module doc).
#[derive(Debug, Clone, Serialize)]
pub struct Against {
    pub query: QueryOut,
    /// `match`, `no_match` or `undecided`: the root's on this item.
    pub outcome: &'static str,
    /// Each node of the query with an outcome on an item, a parent before
    /// its children.
    pub nodes: Vec<AgainstNode>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AgainstNode {
    pub path: String,
    /// The node, as the canonical text prints it.
    pub node: String,
    /// A term's: `matched`, `failed`, `lacked`, `undecided`. A group's:
    /// `match`, `no_match`, `undecided`.
    pub outcome: &'static str,
    /// What the term touched on the item, or why it is undecided.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub shows: Vec<Evidence>,
    /// What it touched past what is shown (invariant 5).
    #[serde(skip_serializing_if = "is_zero")]
    pub left_out: usize,
}

fn is_zero(n: &usize) -> bool {
    *n == 0
}

fn truth_word(truth: Truth) -> &'static str {
    match truth {
        Truth::True => "match",
        Truth::False => "no_match",
        Truth::Undecided => "undecided",
    }
}

fn against_held(query: &Query, held: &Held) -> Against {
    let mut outcomes = Vec::with_capacity(query.terms.len());
    eval::asked(&query.terms, held, &mut outcomes);
    let nodes = crate::tree::nodes(&query.root)
        .into_iter()
        .filter(|node| !node.inside)
        .filter_map(|node| {
            let (outcome, shows, left_out) = match query.at(&node.path)? {
                Bound::Term(at) => {
                    let (shows, left_out) = eval::against(&query.terms, *at, held, &outcomes);
                    let outcome = match outcomes.get(*at)? {
                        Outcome::Matched => "matched",
                        Outcome::Failed => "failed",
                        Outcome::Lacked => "lacked",
                        Outcome::Undecided => "undecided",
                    };
                    (outcome, shows, left_out)
                }
                group => (truth_word(eval::truth(group, &outcomes)), Vec::new(), 0),
            };
            Some(AgainstNode {
                path: node.path,
                node: node.text,
                outcome,
                shows,
                left_out,
            })
        })
        .collect();
    Against {
        query: QueryOut {
            text: query.text(),
            tree: crate::json::to_json(&query.root),
            trade: None,
        },
        outcome: truth_word(eval::truth(&query.bound, &outcomes)),
        nodes,
    }
}

/// One item by id, live or removed, derived, with its body when `body`
/// is set.
pub fn show(
    store: &Store,
    intent: &Annotations,
    id: &str,
    body: bool,
) -> Result<Shown, SearchError> {
    shown(store, intent, id, body, None)
}

/// [`show`], and beside the item why it does or does not match `query`
/// (the module doc). A query the binder refuses is that error, said
/// before the store is read; a site search is read in the item's realm
/// (the module doc).
pub fn against(
    store: &Store,
    intent: &Annotations,
    id: &str,
    body: bool,
    query: &QueryInput,
) -> Result<Shown, SearchError> {
    // an authoring error before the store is read; the query asked is
    // the one read in the item's realm, once the item is found
    query.read(None)?;
    shown(store, intent, id, body, Some(query))
}

fn shown(
    store: &Store,
    intent: &Annotations,
    id: &str,
    body: bool,
    against: Option<&QueryInput>,
) -> Result<Shown, SearchError> {
    for _ in 0..REREADS {
        if let Some(shown) = read_one(store, intent, id, body, against)? {
            return Ok(shown.aged(acquisition_store::now()));
        }
    }
    Err(moved())
}

impl Shown {
    /// Where each number field the item displays is (as
    /// [`ShownItem::displays`]).
    pub fn displays(&self) -> Displays<'_> {
        Displays {
            item: &self.item,
            displayed: &self.displayed,
        }
    }

    /// C53 (an age the text prints stands in words beside its epoch,
    /// made at the answer's own time) and C163 (every time a document
    /// carries has its words beside it): the place's fetch, the item's
    /// first and last sighting and its removal, where it left, and the
    /// basis's snapshot, at the path `show` declares for it, said at
    /// `now`.
    pub fn aged(mut self, now: i64) -> Shown {
        self.place.aged(now);
        item_aged(&mut self.item, now);
        self.snapshot_at_age = Some(crate::answer::snapshot_age(&self.basis, now));
        self
    }
}

/// One read of the item, or nothing where the facts or the intent moved
/// before its price was joined (`corpus.rs`).
fn read_one(
    store: &Store,
    intent: &Annotations,
    id: &str,
    body: bool,
    against: Option<&QueryInput>,
) -> Result<Option<Shown>, SearchError> {
    let table = class_table()?;
    let categories = category_tables()?;
    currency_table()?;
    if against.is_some() {
        // a query may ask a total, the percentile or a property field: checked as a corpus
        // checks them before it is asked anything
        crate::corpus::totals_table()?;
        crate::corpus::defence_table()?;
        crate::corpus::property_table()?;
    }
    let intent_revision = price::intent_revision(intent)?;
    let found = store
        .read_corpus_of(RealmScope::All, Membership::All, |header, rows| {
            let basis = Basis::of(store, header, intent_revision);
            let mut live = 0usize;
            for row in rows {
                let mut row = row?;
                live += usize::from(row.removed_at.is_none());
                if row.id == id {
                    let stored = std::mem::take(&mut row.body);
                    let (facts, place) = placed(&locations(header), row);
                    // the listing state of the item's league, or of every
                    // league of its realm where it has none
                    let leagues = price::leagues(
                        &header
                            .locations
                            .iter()
                            .filter(|l| {
                                l.removed_at.is_none()
                                    && l.realm == place.realm
                                    && place
                                        .league
                                        .as_ref()
                                        .is_none_or(|league| l.league.as_ref() == Some(league))
                            })
                            .cloned()
                            .collect::<Vec<LocationRow>>(),
                    );
                    let mut item = derive(facts, &stored);
                    let place = celled(place, &item);
                    item.unread.extend(tab_type_unread(&place));
                    return Ok(Ok((basis, place, item, stored, leagues)));
                }
            }
            Ok(Err((
                // a live location before a retired one of the same id
                header
                    .locations
                    .iter()
                    .filter(|l| l.id == id)
                    .min_by_key(|l| l.removed_at.is_some())
                    .cloned(),
                live,
                basis.account,
            )))
        })
        .map_err(SearchError::store)?;
    let (basis, place, item, stored, leagues) = match found {
        Ok(found) => found,
        Err((location, live, account)) => {
            return Err(not_shown(store, location, live, &account, id));
        }
    };
    let mut prices = price::join(store, intent, &leagues)?;
    if !still_at(store, intent, &basis)? {
        return Ok(None);
    }
    let (priced, public) = match item.facts.removed_at {
        Some(_) => (Priced::removed(), None),
        None => price::of_item(&mut prices, id),
    };
    let place = Place { public, ..place };
    let class = table.classify(&item);
    // the item held as a corpus holds each of its items
    let held = Held {
        category: categories.of(&item, &class),
        class,
        item,
        place,
        price: priced,
    };
    // the query read in the item's realm, its scope: a site search whose
    // URL names another is refused as a search in that realm would never
    // hold the item; then the held item asked by the one evaluator
    let against = against
        .map(|input| -> Result<Against, SearchError> {
            let read = input.read(Some(&crate::corpus::Realm::One(held.place.realm.clone())))?;
            let mut against = against_held(&read.query, &held);
            against.query.trade = read.translation.map(crate::answer::TradeOut::from);
            Ok(against)
        })
        .transpose()?;
    let place = held.place.clone();
    let ShownItem {
        item,
        class,
        category,
        price,
        lines,
        number_fields,
        displayed,
    } = ShownItem::of(held, categories);
    Ok(Some(Shown {
        basis,
        place,
        item,
        class,
        category,
        price,
        lines,
        number_fields,
        displayed,
        against,
        body: body.then_some(stored),
        snapshot_at_age: None,
    }))
}

fn not_shown(
    store: &Store,
    location: Option<LocationRow>,
    live: usize,
    account: &str,
    id: &str,
) -> SearchError {
    if let Some(location) = location {
        let what = if location.kind == "stash" {
            "tab"
        } else {
            "character"
        };
        // the request, which says the membership; a surface spells it
        let lists = crate::answer::Request {
            scope: crate::answer::Scope {
                account: Some(account.to_string()),
                realm: Some(crate::corpus::Realm::One(location.realm.clone())),
                membership: match location.removed_at {
                    Some(_) => crate::answer::Membership::All,
                    None => crate::answer::Membership::Live,
                },
            },
            query: crate::answer::QueryInput {
                text: Some(format!("id:{id}")),
                tree: None,
                trade: None,
            },
            ..Default::default()
        };
        return SearchError::scope(
            "not_an_item",
            match location.removed_at {
                Some(_) => format!(
                    "`{id}` is the {what} `{}`, which is no longer in the account, not an item: a search under membership all lists what was in it",
                    location.name
                ),
                None => format!(
                    "`{id}` is the {what} `{}`, not an item: a search lists what is in it",
                    location.name
                ),
            },
        )
        .offering(vec![(
            lists,
            match location.removed_at {
                Some(_) => format!(
                    "the search, under membership all, of what was in the {what} `{}`",
                    location.name
                ),
                None => format!("the search of what is in the {what} `{}`", location.name),
            },
        )]);
    }
    match store.item(id) {
        Ok(Some(_)) => SearchError::scope(
            "item_not_live",
            format!(
                "item `{id}` is in the store and at no tab or character the store holds: the search reads an item at its place"
            ),
        ),
        Ok(None) => SearchError::scope(
            "item_not_found",
            format!(
                "no item `{id}` in this store: an id is given whole, as an answer printed it ({live} live items, every realm)"
            ),
        ),
        Err(e) => SearchError::store(e),
    }
}

/// Derive one item outside a store: what a test or a tool holding a body
/// shows of it.
pub fn derived(facts: Facts, body: &str) -> Item {
    derive(facts, body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::ValueRef;
    use std::collections::HashMap;

    /// A line of `template` under `source`, every number of it read.
    fn line_of(template: &str, source: &str) -> Line {
        let count = crate::template::slots(template).count;
        Line {
            source: source.to_string(),
            flags: Vec::new(),
            flags_unread: false,
            flags_unknown: Vec::new(),
            template: template.to_string(),
            numbers: vec![Some(1.0); count],
            text: template.replace('#', "1"),
            rows: Vec::new(),
        }
    }

    /// 4-full part 1, step 6.4c: over every stat the table ships under a
    /// category a line's source is filed under, the value a bound on it
    /// compares is the value a line of each template its translated group
    /// names gets (`line_value`). Nothing here restates how the
    /// translation matches a line: the templates are read from the tree
    /// the translation made. Where two stats' groups name one template
    /// under one category with different values, the first in the table's
    /// order is the line's; those are counted, so a new one is seen.
    #[test]
    fn a_lines_value_is_the_translations_for_every_stat_it_displays_as() {
        let stats = crate::trade::stats::table().unwrap();
        let mut first: HashMap<(&str, String), (String, String)> = HashMap::new();
        let mut checked = 0usize;
        let mut collisions: Vec<String> = Vec::new();
        let mut wrong: Vec<String> = Vec::new();
        for (category, id, _) in stats.entries() {
            let Some(source) = ["explicit", "implicit", "enchant", "scourge"]
                .into_iter()
                .find(|s| category_of(s) == Some(category))
            else {
                continue;
            };
            let stat = format!("{category}.{id}");
            let filter = serde_json::json!({ "query": { "stats": [{ "type": "and", "filters": [
                { "id": stat, "value": { "min": 1 } } ] }] } });
            let decoded = crate::trade::decode(&filter.to_string()).unwrap();
            let translation = crate::trade::translate::translate(&decoded).unwrap();
            let Some(compared) = crate::values::compared(&translation.tree)
                .into_iter()
                .next()
            else {
                continue;
            };
            let ValueRef::Sum { lines, .. } = &compared.value else {
                continue;
            };
            for template in crate::template::named(lines) {
                let key = (category, crate::bind::folded(&template));
                let (winner, by) = first
                    .entry(key)
                    .or_insert_with(|| (compared.text.clone(), stat.clone()))
                    .clone();
                if winner != compared.text {
                    collisions.push(format!("{stat} loses \"{template}\" to {by}"));
                    continue;
                }
                checked += 1;
                let got = line_value(&line_of(&template, source));
                if got.as_deref() != Some(compared.text.as_str()) {
                    wrong.push(format!(
                        "{stat} \"{template}\": {} for {}",
                        got.unwrap_or_else(|| "none".to_string()),
                        compared.text
                    ));
                }
            }
        }
        assert!(checked > 5_000, "{checked}");
        assert!(
            wrong.is_empty(),
            "{} of {checked} lines name another value than their stat's:\n{}",
            wrong.len(),
            wrong.join("\n")
        );
        // 26 at the capture of 2026-09-12: a support gem's `Socketed Gems
        // are Supported by Level #` under two ids, `Lose # Mana per
        // second`, `#% reduced Elemental Damage Taken while stationary` and
        // a map's Rogue Exiles; each line is the first stat's
        assert_eq!(collisions.len(), 26, "{}", collisions.join("\n"));
    }

    /// The lines the reviews of step 6.4c named each pass through the stat
    /// they display as (`trade_value`), never the rule over a line's own
    /// group: a `+` the site writes before a `#`, the site's digit where
    /// an item shows another number, a plural form the bound sums, and a
    /// number the site writes as a digit.
    #[test]
    fn the_reviews_lines_pass_through_their_stat() {
        for text in [
            "+95 to maximum Life",
            "+5 Life per 4 Dexterity",
            "15% chance to gain Phasing for 4 seconds on Kill",
            "15% chance to gain Phasing for 6 seconds on Kill",
            "Drops Burning Ground while moving, dealing 2500 Fire Damage per second for 4 seconds",
        ] {
            let (template, numbers) = crate::template::typed(text);
            let line = Line {
                numbers: numbers.into_iter().map(Some).collect(),
                ..line_of(&template, "explicit")
            };
            assert!(
                trade_value(&line).is_some(),
                "{text} took the line's own group"
            );
        }
    }
}
