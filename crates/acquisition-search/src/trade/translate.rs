//! The translation (C113, C99, `decisions/trade.md`; the contract detail K6, taken here; step 4
//! of `search/TRADE-PLAN.md`): a decoded site search to the query text a
//! person would have typed, and a report, clause by clause, of what each
//! became.
//!
//! # Decisions as recorded
//!
//! - **C113 (K6), taken from the contract detail at step 4** (verbatim;
//!   what a URL is and its scope are `answer.rs`'s, taken at step 5).
//!   Every clause has one of five statuses — exact, ambiguous, unsupported,
//!   listing-only, disabled — from one catalogue table the translation
//!   reads, `--describe trade` prints and a test walks, an unknown key or
//!   option unsupported by name. `fidelity` is the worst status among the
//!   clauses with stash meaning — exact, ambiguous, or partial when the
//!   remainder is nonempty — promising the item predicate alone. A stat
//!   group translates whole or not at all, `and` excepted. A hash-category
//!   stat is `line("T" …)` with the category as source or flag, `min` and
//!   `max` on `arg1`, on `avg` for a ranged template (S47), presence for a
//!   text with no number; `+#` and a `(Local)` suffix are spelling, the
//!   suffix making the clause ambiguous (S72), as does a text with two ids
//!   (S66); an id of two texts in one category is unsupported (owner,
//!   2026-10-15: "Unsupported until the sitting"); a pseudo the totals
//!   table names is that total or reading, with no bound `has:` it (C94),
//!   any other unsupported. `not` is `-( … or … )`, `if` is `(-line(T) or
//!   line(T v))` per stat, `count` is `holds(…)=min..max`, a weighted group
//!   as K5. Every clause that runs owns one term path inside its group, a
//!   tautology its own, and every path is a clause's. What no sitting has
//!   asked — which number a many-numbered template compares, and each row
//!   whose note leaves it to one — is ambiguous until one has.
//! - **What step 7's sitting settled, built at 7e** (the owner, 2026-10-16:
//!   "I accept all your recommendations"; 2026-10-17, "The choice before
//!   7e" in the brief at `77666ea4` and `brainstorming-notes/37-the-stat-value.md`:
//!   the translation alone ships, a named
//!   stat value is parked). A site stat is the item's stat however
//!   displayed: a bound on it is `sum(line(T source=…).arg1)` over the
//!   sources the id admits — an explicit id holds the stat's crafted and
//!   fractured lines — with the line's presence beside a maximum, since a
//!   sum of nothing is zero (C95); a stat with a numberless or
//!   opposite-sign form is ambiguous with a note naming the park — label,
//!   never compute an alternative from the bound; `No Physical Damage`,
//!   and any form no sitting read, is presence alone. The forms are one
//!   reference table beside the catalogue (C106), read here alone
//!   (`forms.rs`).
//!
//! # As built
//!
//! - **One maker of a clause's text: the catalogue's row** ([`catalogue`]).
//!   A clause's `lands` is filled as the catalogue's header states — a
//!   text through the printer's quoting, a number as the printer writes
//!   it, in one pass from the left so that what fills one placeholder is
//!   never read for another; a term holding a placeholder the clause lacks
//!   dropped, `true()` where the clause fills none, a key absent and one
//!   sent null alike; every bound made by one maker, `{bounds}`, as a
//!   person types it (`ilvl>=60`) — and the filled
//!   text is parsed and bound alone, so that no clause can
//!   reach into another's and a text the binder refuses (an option no
//!   closed set holds) is that clause unsupported by its name, with the
//!   binder's words (`query.unknown`), never an error of the whole.
//! - **A site object is read where it stands** (`read`): each level names
//!   the keys it reads — a filter's value by its row's `shape`, a filter
//!   group, a stat group and its value, a stat filter and its value, the
//!   weight in a weighted sum alone — and the site's `disabled` is read
//!   before anything else of the clause, which it marks disabled whatever
//!   else it carries. Any other key, or a value that is no object, makes
//!   unsupported, by name, the clause it could change: a filter, every
//!   filter of its group and the group, a stat group whole, the envelope's
//!   own key. What the translation does not read it never drops unsaid.
//! - **A group is composed here from its filters' texts**, whole or not at
//!   all, `and` excepted (K6): an unsupported filter of any other group
//!   puts the group and every filter of it in the remainder. Every node a
//!   group composes is bound alone, as a clause's text is: one the binder
//!   refuses — a count's or a sum's bounds that run backwards — is the
//!   group unsupported with the binder's words, never an error of the
//!   whole. `and` and
//!   `if` leave each filter a term of the query; `not` is `-(a or b …)`,
//!   `-a` for one; `count` is `holds(a, b …)` with the group's bounds, and
//!   with none `true()` for each filter (t37); a weighted sum is
//!   `sum(line(… or …).slot)` against the group's bounds over its one
//!   weight (K5), and with none `(P1 or P2 …)`, one of its stats present
//!   (t38); the sum made by `tree::sum_of_lines`, since
//!   a group's tree is named by the tree's own modules alone
//!   (`tools/docs-check.sh` §7). `if` is, per stat, not the stat's presence or
//!   the stat with its bounds — `(-line(T) or line(T b))`,
//!   `(-has:pseudo.X or pseudo.X>=v)` — and `true()` for a stat with none,
//!   the site's own no-op; the presence is landed only for these two
//!   groups. The group
//!   rows' notes state each shape; this is its one maker.
//! - **A stat's line** is the site's text for its id (`stats.rs`), with
//!   its `(Local)` suffix dropped, and each of its forms (`forms.rs`), one
//!   group's alternatives, each read by the deriver's own scan of a
//!   displayed string (`template::fixed`): a number a display writes as a
//!   digit is a number of the line, the template has `#` there and its
//!   slot is open — the site's twenty Blight-ravaged maps of x01 and x03
//!   show `at 50% value` where its text writes 25 — and a bound compares
//!   the site's own `#`, never the digit (t30, t31). A text two ids share
//!   once its digits are `#` is the text two ids share
//!   (`stat.text.collision`). The row's `lands` is that line under the
//!   category's source or flag.
//! - **A stat filter with no bound is the stat's presence**: a line's is
//!   the line over every form, a pseudo's `has:pseudo.X` — the site shows
//!   no pseudo for a total of nothing, which the search reads as lacked
//!   (C94).
//! - **A bound is a sum** (`stat.text.bounds`): over the line of the text
//!   and its plural forms alone, `sum(L.slot)` compared as the site's
//!   `min` and `max` say, made by `tree::sum_of_lines` as a weighted sum's
//!   is, the slot the text's own `#` — `avg` on a ranged pair of them, the
//!   first of several — and none where the text has no `#`, which is
//!   presence alone and drops the bound. Beside a maximum the line `L`
//!   itself, conjoined, since a sum of nothing is zero and meets it where
//!   the site's stat of nothing meets none — by the bound's side, so an
//!   edit of its number keeps the meaning — and beside a minimum at or
//!   below zero, which its number decides: a person editing such a minimum
//!   above zero leaves a guard that no longer changes the answer, and one
//!   editing a minimum down to zero must add it. A form no sum can hold — opposite, numberless,
//!   another wording — labels the bound `stat.text.forms`, a refused one
//!   the clause `stat.text.refused`, bounded or not, and another wording
//!   the game data gives the clause `stat.text.wording`, bounded or not:
//!   the line matches it, and that the site files it under the id is the
//!   generator's reading, which no capture shows (the outside review of
//!   7e). A wording the captures show is a row of the hand table, exact.
//! - **Statuses are read where the clause is read.** A filter's status is
//!   its category's row, worsened by each line rule that applies to its
//!   text (a `(Local)` suffix, a text two ids share, several `#`, a form
//!   no sum holds under a bound, a refused form) and, in a `not`, by an
//!   eldritch form's text — a `not` with a value excludes the bound's
//!   meeting, as the site does (t24): the site lets
//!   some items carrying that form through a `not` on its id, a fact of
//!   the captures (`search/pseudo-stats/README.md`, round four) the
//!   oracle met on 377 items (step 6, `tests/trade_oracle.rs`), so the
//!   exclusion is ambiguous, never exact. A group's is its row's. The
//!   site's `disabled` is `disabled` whatever the row says. A stat id the
//!   site lists with two texts in one category is unsupported (the
//!   owner, 2026-10-15: "Unsupported until the sitting").
//! - **Every number the site sends is held to the search's rule for a
//!   number** (`exact::reads`): one outside it is the clause unsupported,
//!   never rounded. A weighted sum's quotient is exact or the group is
//!   unsupported.
//! - **A field the site's filter reads past a lack** (`lacked =
//!   "meets_maximum"` on its row): with a maximum and no minimum, an item
//!   lacking the field meets the clause, `(F<=max or -has:F)` — no
//!   requirement meets any maximum (u061).
//! - **The text is the product.** The clauses' nodes are the conjuncts of
//!   the query in the report's order — the URL's league, then the query
//!   object's `status`, `term`, `name`, `type`, its filters in the
//!   catalogue's order of their rows, its stat groups in the site's order,
//!   any key no row names, then the envelope's sort — printed by the one
//!   printer, parsed and bound again by the one parser and binder, and
//!   refused as a build defect unless it reads back to the same tree
//!   (rule 5 of the plan: what the answer prints, the same build runs).
//! - **The guard** (K6, "every clause that runs owns one term path inside
//!   its group, a tautology its own, and every path is a clause's") is
//!   checked on every translation before it is returned (`guard`): each
//!   running clause owns one path of the tree (`tree::nodes`), none
//!   twice, a `not`'s or a `count`'s filter strictly under its group's;
//!   every path of the tree lies under a clause's; and the clauses that
//!   own none are exactly those the tree cannot give one — an `and` or
//!   an `if` group, a `count` with no bound (each filter its own `true()`),
//!   a weighted sum with no bound whose one stat is its presence alone,
//!   and a group with no filter the site leaves enabled, which write no
//!   node of their own, and a bounded weighted sum's filters, conditions
//!   inside `sum( … )`, which take no path. A query
//!   that is one clause owns the root, written `""`.
//! - **An ambiguous clause's other reading** (C119, `TRADE-SLICE.md` row
//!   8b): its row's `other`, filled as `lands` is, `{term}` the text of
//!   the node it stands in for, once the clause stands in the query
//!   (`readings`). It applies where the clause has every property of the
//!   shape it was asked for, the review checklist's rule: that node a
//!   conjunct of the query whole — the clause's own, or the bounded
//!   weighted sum's it is a line of — never one inside a `not`, a count
//!   or an `if`; `{forms}` a minimum alone on a filter of an `and`
//!   group, every form no sum holds the opposite word, their line under
//!   the category's source (the trade seat's `reduced` Rarity); and
//!   `{without}` a scourge line beside another of its sum, the sum
//!   recomposed without it by the one maker of a weighted sum. A reading
//!   with a placeholder its clause leaves unfilled, or on a clause that
//!   states nothing, is none, never a reading equal to the clause. What
//!   it changes is the answer's to count (`answer.rs`).
//! - **Fidelity** is the worst status among the clauses with a stash
//!   meaning — `exact`, `ambiguous`, or `partial` when the remainder, the
//!   unsupported clauses, is not empty; listing-only and disabled clauses
//!   leave it alone. It promises the item predicate and nothing else.

use std::collections::{HashMap, HashSet};

use serde::Serialize;
use serde_json::{Map, Value as Json};

use super::catalogue::{self, Catalogue, Disposition, Row};
use super::forms::{self, FormsTable, Kind};
use super::stats::{self, StatTable};
use super::{Decoded, Source};
use crate::error::{ErrorKind, LanguageError};
use crate::exact::{self, Exact, Ratio};
use crate::totals::{self, TotalsTable};
use crate::tree::{self, Node, Number, Op, Value, ValueRef};
use crate::{bind, print, template};

/// A clause's status in the report (K6): the four a catalogue row gives,
/// and `disabled`, the site's own mark.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Exact,
    Ambiguous,
    Unsupported,
    ListingOnly,
    Disabled,
}

impl Status {
    fn of(d: Disposition) -> Status {
        match d {
            Disposition::Exact => Status::Exact,
            Disposition::Ambiguous => Status::Ambiguous,
            Disposition::Unsupported => Status::Unsupported,
            Disposition::ListingOnly => Status::ListingOnly,
        }
    }

    /// Whether a clause of this status writes a term.
    pub fn runs(self) -> bool {
        matches!(self, Status::Exact | Status::Ambiguous)
    }

    /// The worse of two statuses of a clause with a stash meaning.
    fn worse(self, other: Status) -> Status {
        let rank = |s: Status| match s {
            Status::Exact => 0,
            Status::Ambiguous => 1,
            Status::Unsupported => 2,
            Status::ListingOnly | Status::Disabled => 3,
        };
        if rank(other) > rank(self) {
            other
        } else {
            self
        }
    }
}

/// How far the text promises the site's item predicate (K6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Fidelity {
    Exact,
    Ambiguous,
    Partial,
}

/// One clause of the site's search, as the report prints it.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Clause {
    /// Where it stands in the site's request: `url.league`, `status`,
    /// `term`, `name`, `type`, `<group>.<filter>`, `stats.<n>` a stat
    /// group, `stats.<n>.<m>` a filter of it, `sort`, or a key no row
    /// names.
    pub at: String,
    /// What the site wrote for it.
    pub site: Json,
    /// The catalogue's rows read for it, the deciding one first.
    pub rows: Vec<String>,
    pub status: Status,
    /// The text written for it, as the canonical text prints it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    /// The path of its term in the query (`tree.rs`); `""` the root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
    /// The clause's other reading (C119), where its row carries one and
    /// the clause has the shape it was asked for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Reading>,
}

/// An ambiguous clause's other reading (C119): the fragment the clause
/// would be read as the other way, and the path of the conjunct of the
/// query it stands in for — the clause's own, or the weighted sum's it is
/// a line of.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Reading {
    /// The fragment, as the canonical text prints it.
    pub text: String,
    /// The path of the conjunct it replaces; `""` the root.
    pub path: String,
    #[serde(skip)]
    pub node: Node,
}

/// A site search translated: the query text and its tree, the report, and
/// the scope the URL carried.
#[derive(Debug, Clone, Serialize)]
pub struct Translation {
    pub source: Source,
    /// The realm a URL names, which is the search's scope (C96); none for
    /// an id or JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,
    /// The canonical text, parsed and bound by this build.
    pub text: String,
    #[serde(serialize_with = "tree_json")]
    pub tree: Node,
    pub clauses: Vec<Clause>,
    /// The `at` of every unsupported clause.
    pub remainder: Vec<String>,
    pub fidelity: Fidelity,
    /// The catalogue's version, the stat table's and the forms table's,
    /// for the basis (C98).
    pub catalogue: u32,
    pub stats: String,
    pub forms: u32,
}

fn tree_json<S: serde::Serializer>(tree: &Node, s: S) -> Result<S::Ok, S::Error> {
    crate::json::to_json(tree).serialize(s)
}

fn trade(message: impl Into<String>) -> LanguageError {
    LanguageError::new(ErrorKind::Trade, message)
}

/// A decoded site search to its translation. An error is a table this
/// build ships that does not load, or a translation that broke its own
/// guard — a build defect, said, never a partial answer.
pub fn translate(decoded: &Decoded) -> Result<Translation, LanguageError> {
    // a key a site object sends null is absent (u059), said once, here,
    // before anything reads a key: every reader below meets the query as if
    // the key were never sent
    let decoded = &Decoded {
        query: absent_nulls(&decoded.query),
        sort: decoded
            .sort
            .as_ref()
            .filter(|s| !s.is_null())
            .map(absent_nulls),
        beside: decoded
            .beside
            .iter()
            .filter(|(_, v)| !v.is_null())
            .map(|(k, v)| (k.clone(), absent_nulls(v)))
            .collect(),
        ..decoded.clone()
    };
    let catalogue = catalogue::catalogue().map_err(|e| trade(e.to_string()))?;
    let stats = stats::table().map_err(|e| trade(e.to_string()))?;
    let totals = totals::table().map_err(|e| trade(e.to_string()))?;
    let forms = forms::table().map_err(|e| trade(e.to_string()))?;
    let mut t = Translator {
        catalogue,
        stats,
        totals,
        forms,
        clauses: Vec::new(),
        own: Vec::new(),
        pieces: Vec::new(),
        pending: Vec::new(),
    };
    if let Some(league) = &decoded.league {
        t.input("url.league", "url.league", &Json::String(league.clone()))?;
    }
    let Json::Object(query) = &decoded.query else {
        return Err(trade("the query is not an object"));
    };
    for key in ["status", "term", "name", "type"] {
        if let Some(value) = query.get(key) {
            t.top(key, value)?;
        }
    }
    if let Some(filters) = query.get("filters") {
        t.filters(filters)?;
    }
    if let Some(groups) = query.get("stats") {
        match groups {
            Json::Array(groups) => {
                for (n, group) in groups.iter().enumerate() {
                    t.group(n, group)?;
                }
            }
            other => t.unknown("stats", other, "`stats` is no list of groups")?,
        }
    }
    for (key, value) in query {
        if !matches!(
            key.as_str(),
            "status" | "term" | "name" | "type" | "filters" | "stats"
        ) {
            t.unknown(key, value, &format!("no row names the key `{key}`"))?;
        }
    }
    if let Some(sort) = &decoded.sort {
        t.reported("sort", sort, "query.sort", None)?;
    }
    for (key, value) in &decoded.beside {
        t.unknown(
            &format!("envelope.{key}"),
            value,
            &format!("the envelope's `{key}`, which no row reads"),
        )?;
    }
    t.finish(
        decoded,
        catalogue.version(),
        stats.version(),
        forms.version(),
    )
}

/// A site object with every key sent null taken out, at every depth; an
/// array's elements are kept where they stand, a null among them read as
/// what it is.
fn absent_nulls(value: &Json) -> Json {
    match value {
        Json::Object(o) => Json::Object(
            o.iter()
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| (k.clone(), absent_nulls(v)))
                .collect(),
        ),
        Json::Array(a) => Json::Array(a.iter().map(absent_nulls).collect()),
        other => other.clone(),
    }
}

/// What a clause owns of the tree, for the guard.
#[derive(Debug, Clone, PartialEq)]
enum Own {
    /// A path, once the root is laid out; `parent` the group clause it
    /// lies strictly under, if any.
    Path { parent: Option<usize> },
    /// A group that writes no node of its own: `and`, `if`, and any group
    /// with no filter the site leaves enabled.
    Conjuncts,
    /// A condition inside `sum( … )`, which takes no path.
    InsideSum,
    /// A clause that does not run.
    Nothing,
}

/// A conjunct of the query, and the clauses that own its nodes, each at a
/// path relative to it (`""` the piece itself).
struct Piece {
    node: Node,
    owners: Vec<(usize, String)>,
}

/// A clause as its row reads it, before it is placed.
struct Landed {
    status: Status,
    rows: Vec<String>,
    notes: Vec<String>,
    node: Option<Node>,
    /// Its row's other reading, as far as the clause fills it (C119).
    reading: Option<Pending>,
}

/// An other reading as a row writes it (C119), with what the clause has
/// filled of it so far; `{term}` is filled once the clause's node stands
/// in the query (`finish`).
#[derive(Debug, Clone)]
struct Pending {
    template: String,
    fills: HashMap<String, String>,
    /// Whether it stands in for the clause's group's node — a weighted
    /// sum's, the clause a line inside it — and not the clause's own.
    group: Option<usize>,
}

impl Pending {
    /// A row's reading, what fills its placeholders so far beside it.
    fn of(row: &Row, fills: impl IntoIterator<Item = (&'static str, String)>) -> Option<Pending> {
        Some(Pending {
            template: row.other.clone()?,
            fills: fills.into_iter().map(|(k, v)| (k.to_string(), v)).collect(),
            group: None,
        })
    }
}

/// A stat filter, read: what each group takes of it.
struct StatRead {
    at: String,
    site: Json,
    landed: Landed,
    /// The stat on the item at all — its line over every form, `has:` a
    /// value — for `if`.
    presence: Option<Node>,
    /// The line a bound sums: the text and its plural forms (`forms.rs`);
    /// for a weighted sum.
    summed: Option<Node>,
    /// The slot a bound compares; none for a text with no `#`.
    slot: Option<String>,
    bounded: bool,
    weight: Option<Number>,
}

/// How a line's two eldritch forms open: the line under an eldritch
/// influence, which the site counts as the line and which a `not` on its
/// id does not always exclude (`search/pseudo-stats/README.md`, round
/// four).
const ELDRITCH: [&str; 2] = [
    "While a Unique Enemy is in your Presence, ",
    "While a Pinnacle Atlas Boss is in your Presence, ",
];

/// Whether a group of `kind` reads its filters' presence beside them.
fn presence_too(kind: &str) -> bool {
    matches!(kind, "if" | "weight" | "weight2")
}

/// A stat filter's own keys and its value's, read (K6): the value, or none
/// where it is absent or null.
fn read_stat<'j>(filter: &'j Json, kind: &str) -> Result<Option<&'j Map<String, Json>>, String> {
    read(Some(filter), &STAT_FILTER)?;
    read(filter.get("value"), stat_value_keys(kind)).map_err(|why| format!("its value: {why}"))
}

struct Translator<'a> {
    catalogue: &'a Catalogue,
    stats: &'a StatTable,
    totals: &'a TotalsTable,
    forms: &'a FormsTable,
    clauses: Vec<Clause>,
    own: Vec<Own>,
    pieces: Vec<Piece>,
    /// Each clause's other reading, as its row reads it (C119).
    pending: Vec<Option<Pending>>,
}

/// What fills a placeholder (the catalogue's header).
enum Fill {
    Text(String),
    Number(Number),
    Name(String),
    Made(String),
    /// `{bounds}` on a value a stat filter names with no bound: the term
    /// `X{bounds}` is `has:X`, the value's presence (K6, C94).
    Presence,
}

impl Fill {
    fn written(&self) -> String {
        match self {
            Fill::Text(t) => print::quoted_text(t),
            Fill::Number(n) => print::number_text(*n),
            Fill::Name(n) | Fill::Made(n) => n.clone(),
            Fill::Presence => String::new(),
        }
    }
}

/// A row's `lands` filled: the text, or none where the clause fills none
/// of the row's placeholders and so states nothing.
fn fill(lands: &str, get: &dyn Fn(&str) -> Option<Fill>) -> Option<String> {
    let mut seen = 0;
    let mut filled = 0;
    let text = fill_sequence(lands, get, &mut seen, &mut filled);
    (seen == 0 || filled > 0).then_some(text)
}

/// The terms of a sequence, split where whitespace stands outside quotes
/// and parentheses; a call's inside, `linked( … )` or `line( … )`, filled
/// as a sequence of its own.
fn fill_sequence(
    text: &str,
    get: &dyn Fn(&str) -> Option<Fill>,
    seen: &mut usize,
    filled: &mut usize,
) -> String {
    let mut out: Vec<String> = Vec::new();
    for token in terms(text) {
        let call = token
            .find('(')
            .filter(|open| {
                *open > 0
                    && token[..*open].bytes().all(|b| b.is_ascii_alphabetic())
                    && token.ends_with(')')
            })
            .map(|open| (&token[..open], &token[open + 1..token.len() - 1]));
        if let Some((name, inside)) = call {
            let inside = fill_sequence(inside, get, seen, filled);
            if !inside.is_empty() {
                out.push(format!("{name}({inside})"));
            }
            continue;
        }
        let names = placeholders(token);
        if names.is_empty() {
            out.push(token.to_string());
            continue;
        }
        *seen += names.len();
        let values: Vec<Option<Fill>> = names.iter().map(|n| get(n)).collect();
        let present = values.iter().filter(|v| v.is_some()).count();
        *filled += present;
        // a term needs all it names
        if present < names.len() {
            continue;
        }
        let presence = values.iter().any(|v| matches!(v, Some(Fill::Presence)));
        let written = if presence {
            format!("has:{token}")
        } else {
            token.to_string()
        };
        out.push(substituted(&written, &|name| {
            names
                .iter()
                .position(|n| *n == name)
                .and_then(|at| values[at].as_ref())
                .map(Fill::written)
                .unwrap_or_default()
        }));
    }
    out.join(" ")
}

/// A term with each placeholder replaced by what fills it, in one pass
/// from the left, so that what fills one is never read for another.
fn substituted(term: &str, with: &dyn Fn(&str) -> String) -> String {
    let mut out = String::with_capacity(term.len());
    let mut rest = term;
    while let Some(open) = rest.find('{') {
        let Some(close) = rest[open..].find('}') else {
            break;
        };
        out.push_str(&rest[..open]);
        out.push_str(&with(&rest[open + 1..open + close]));
        rest = &rest[open + close + 1..];
    }
    out.push_str(rest);
    out
}

/// The whitespace-separated terms of a text at its own level: quotes and
/// parentheses hold what is inside them.
fn terms(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let (mut depth, mut quoted, mut escaped) = (0i32, false, false);
    let mut start = None;
    for (at, c) in text.char_indices() {
        if quoted {
            match c {
                _ if escaped => escaped = false,
                '\\' => escaped = true,
                '"' => quoted = false,
                _ => {}
            }
            continue;
        }
        match c {
            '"' => quoted = true,
            '(' => depth += 1,
            ')' => depth -= 1,
            c if c.is_whitespace() && depth == 0 => {
                if let Some(s) = start.take() {
                    out.push(&text[s..at]);
                }
                continue;
            }
            _ => {}
        }
        if start.is_none() {
            start = Some(at);
        }
    }
    if let Some(s) = start {
        out.push(&text[s..]);
    }
    out
}

/// The placeholders a term writes, `{bounds}` and its kin, in order.
fn placeholders(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(open) = rest.find('{') {
        let after = &rest[open + 1..];
        let Some(close) = after.find('}') else { break };
        out.push(&after[..close]);
        rest = &after[close + 1..];
    }
    out
}

/// A number the site sent, as the search reads one (`exact::reads`): none
/// for a value absent or null, an error naming it for one outside the rule.
fn number(value: Option<&Map<String, Json>>, key: &str) -> Result<Option<Number>, String> {
    match value.and_then(|v| v.get(key)) {
        None | Some(Json::Null) => Ok(None),
        Some(Json::Number(n)) => {
            let literal = n.to_string();
            match n.as_f64() {
                Some(f) if exact::reads(&literal) => Ok(Some(Number::from_f64(f))),
                _ => Err(format!(
                    "`{key}` is {literal}, no number the search reads: digits, at most ten whole and four decimal"
                )),
            }
        }
        Some(other) => Err(format!("`{key}` is {other}, not a number")),
    }
}

/// `{bounds}`, the one maker of a bound's text: `>=min`, `<=max` or
/// `=min..max`, as a person types it, or none where the clause has neither.
fn bounds(min: Option<Number>, max: Option<Number>) -> Option<String> {
    let n = print::number_text;
    match (min, max) {
        (None, None) => None,
        (Some(a), None) => Some(format!(">={}", n(a))),
        (None, Some(b)) => Some(format!("<={}", n(b))),
        (Some(a), Some(b)) => Some(format!("={}..{}", n(a), n(b))),
    }
}

/// What a bound compares a value with: `>=min`, `<=max` or `=min..max`,
/// none where the clause has neither — the one maker of a comparison from
/// the site's bounds, a stat's sum's and a weighted sum's.
fn bounds_of(min: Option<Number>, max: Option<Number>) -> Option<(Op, Value)> {
    match (min, max) {
        (None, None) => None,
        (Some(a), None) => Some((Op::Ge, Value::Number(a))),
        (None, Some(b)) => Some((Op::Le, Value::Number(b))),
        (from, to) => Some((Op::Eq, Value::Range { from, to })),
    }
}

/// One display of a stat, as a line's group names it: its template, a
/// number its text writes as a digit a `#` of it like any other (round x:
/// a written digit is a number of the line), and the slots of the `#`s the
/// site's text has, counting from 1 — the numbers a bound compares.
#[derive(Clone)]
struct Shown {
    template: String,
    open: Vec<usize>,
}

impl Shown {
    fn of(text: &str) -> Shown {
        let (template, fixed) = template::fixed(text);
        let open = (1..=fixed.len())
            .filter(|n| fixed[n - 1].is_none())
            .collect();
        Shown { template, open }
    }

    /// Whether the text writes a number as a digit (`stat.text.numbers`).
    fn writes_a_digit(&self) -> bool {
        self.open.len() < template::slots(&self.template).count
    }

    /// Displays as the conditions of one line's group: the one alone,
    /// `"T"`, or each an alternative, `("T" or "F")`.
    fn alternatives(all: &[Shown]) -> String {
        let one = |s: &Shown| print::quoted_text(&s.template);
        match all {
            [only] => one(only),
            _ => format!("({})", all.iter().map(one).collect::<Vec<_>>().join(" or ")),
        }
    }
}

/// A site object where it stands, read: a key it carries that no list
/// here names is unsupported by name (K6), but one sent null, which is
/// absent (u059), and so is anything but an object; a value absent or null
/// is an empty one.
fn read<'j>(
    value: Option<&'j Json>,
    keys: &[&str],
) -> Result<Option<&'j Map<String, Json>>, String> {
    match value {
        None | Some(Json::Null) => Ok(None),
        Some(Json::Object(o)) => {
            // a key sent null is absent, whatever its name (u059: the site
            // posts a socket filter's unset `a` and `d` null)
            let strays: Vec<String> = o
                .iter()
                .filter(|(k, v)| !v.is_null() && !keys.contains(&k.as_str()))
                .map(|(k, _)| format!("`{k}`"))
                .collect();
            if strays.is_empty() {
                Ok(Some(o))
            } else {
                Err(format!(
                    "it carries {}, which no row reads",
                    strays.join(", ")
                ))
            }
        }
        Some(other) => Err(format!("{other} is no object")),
    }
}

/// The site's `disabled` mark, read before anything else of its clause:
/// a clause it marks is disabled whatever else it carries.
fn disabled(value: &Json) -> Result<bool, String> {
    match value.get("disabled") {
        None | Some(Json::Null) | Some(Json::Bool(false)) => Ok(false),
        Some(Json::Bool(true)) => Ok(true),
        Some(other) => Err(format!("its `disabled` is {other}, no yes or no")),
    }
}

/// The keys each site object reads (K6): a filter group's, a filter's by
/// its row's shape, a stat group's and its value's, a stat filter's and
/// its value's, the weight only in a weighted sum.
const FILTER_GROUP: [&str; 2] = ["filters", "disabled"];
const STAT_GROUP: [&str; 4] = ["type", "filters", "value", "disabled"];
const GROUP_VALUE: [&str; 2] = ["min", "max"];
const STAT_FILTER: [&str; 3] = ["id", "value", "disabled"];

fn filter_keys(shape: &str) -> &'static [&'static str] {
    match shape {
        "option" => &["option", "disabled"],
        "minmax" => &["min", "max", "disabled"],
        "sockets" => &["min", "max", "r", "g", "b", "w", "disabled"],
        "minmax_option" => &["min", "max", "option", "disabled"],
        _ => &["disabled"],
    }
}

fn stat_value_keys(kind: &str) -> &'static [&'static str] {
    match kind {
        "weight" | "weight2" => &["min", "max", "option", "weight"],
        _ => &["min", "max", "option"],
    }
}

impl Translator<'_> {
    fn row(&self, key: &str) -> Result<&Row, LanguageError> {
        self.catalogue
            .row(key)
            .ok_or_else(|| trade(format!("the trade catalogue has no row `{key}`")))
    }

    fn note_of(&self, key: &str) -> Result<Vec<String>, LanguageError> {
        Ok(self.row(key)?.says.iter().cloned().collect())
    }

    /// A clause, recorded: its index.
    fn push(&mut self, at: &str, site: &Json, landed: &Landed, own: Own) -> usize {
        self.clauses.push(Clause {
            at: at.to_string(),
            site: site.clone(),
            rows: landed.rows.clone(),
            status: landed.status,
            term: None,
            path: None,
            notes: landed.notes.clone(),
            other: None,
        });
        self.own.push(own);
        // a reading stands in for a node the clause, or its weighted sum,
        // writes; a clause that does not run, or writes none, has none
        self.pending.push(landed.reading.clone().filter(|p| {
            landed.status.runs()
                && (p.group.is_some() || landed.node.as_ref().is_some_and(|n| !is_const(n)))
        }));
        self.clauses.len() - 1
    }

    /// A clause that is a conjunct of the query alone.
    fn conjunct(&mut self, at: &str, site: &Json, landed: Landed) {
        let own = if landed.node.is_some() {
            Own::Path { parent: None }
        } else {
            Own::Nothing
        };
        let i = self.push(at, site, &landed, own);
        if let Some(node) = landed.node {
            self.pieces.push(Piece {
                node,
                owners: vec![(i, String::new())],
            });
        }
    }

    /// Unsupported by name (`query.unknown`).
    fn unknown_landed(&self, why: &str) -> Result<Landed, LanguageError> {
        let mut notes = self.note_of("query.unknown")?;
        notes.push(why.to_string());
        Ok(Landed {
            status: Status::Unsupported,
            rows: vec!["query.unknown".to_string()],
            notes,
            node: None,
            reading: None,
        })
    }

    fn unknown(&mut self, at: &str, site: &Json, why: &str) -> Result<(), LanguageError> {
        let landed = self.unknown_landed(why)?;
        self.conjunct(at, site, landed);
        Ok(())
    }

    /// A clause whose row does not run, or one the site disabled: reported.
    fn reported(
        &mut self,
        at: &str,
        site: &Json,
        key: &str,
        status: Option<Status>,
    ) -> Result<(), LanguageError> {
        let row = self.row(key)?;
        let landed = Landed {
            status: status.unwrap_or(Status::of(row.disposition)),
            rows: vec![key.to_string()],
            notes: row.says.iter().cloned().collect(),
            node: None,
            reading: None,
        };
        self.conjunct(at, site, landed);
        Ok(())
    }

    /// A row's text filled and read: the row's status, or unsupported with
    /// the binder's words where the text it makes is refused.
    fn land(&self, key: &str, get: &dyn Fn(&str) -> Option<Fill>) -> Result<Landed, LanguageError> {
        let row = self.row(key)?;
        let mut landed = Landed {
            status: Status::of(row.disposition),
            rows: vec![key.to_string()],
            notes: row.says.iter().cloned().collect(),
            node: None,
            reading: None,
        };
        if !landed.status.runs() {
            return Ok(landed);
        }
        let Some(lands) = &row.lands else {
            return Ok(landed);
        };
        let text = fill(lands, get).unwrap_or_else(|| "true()".to_string());
        // the row's other reading, its placeholders the clause fills as
        // it fills `lands`'s; `{term}` waits for the clause's node (C119)
        landed.reading = row.other.as_ref().and_then(|template| {
            Pending::of(
                row,
                catalogue::PLACEHOLDERS
                    .iter()
                    .filter(|name| template.contains(&format!("{{{name}}}")))
                    .filter_map(|name| get(name).map(|f| (*name, f.written()))),
            )
        });
        match bind::parse_query(&text) {
            Ok(q) => landed.node = Some(q.tree().clone()),
            Err(e) => {
                let mut unknown =
                    self.unknown_landed(&format!("`{text}` is refused: {}", e.message))?;
                unknown.rows.insert(0, key.to_string());
                return Ok(unknown);
            }
        }
        Ok(landed)
    }

    /// The URL's league, a query's `term`, `name` or `type`: a text, or
    /// null, which fills nothing.
    fn input(&mut self, at: &str, key: &str, value: &Json) -> Result<(), LanguageError> {
        let text = match value {
            Json::String(text) => Some(text.clone()),
            Json::Null => None,
            _ => return self.unknown(at, value, &format!("`{at}` is {value}, not a text")),
        };
        let landed = self.land(key, &|name| match name {
            "value" => text.clone().map(Fill::Text),
            _ => None,
        })?;
        self.conjunct(at, value, landed);
        Ok(())
    }

    fn top(&mut self, key: &str, value: &Json) -> Result<(), LanguageError> {
        let at = self.clauses.len();
        self.top_read(key, value)?;
        // a Base naming a map's tier: the site returns its blighted form
        // under it too (t55, `blighted-base`), which `base=` does not
        let named = match value {
            Json::Object(o) => o.get("option"),
            other => Some(other),
        };
        if key == "type"
            && let Some(Json::String(base)) = named
            && crate::properties::tier_of(base).is_some()
        {
            let r = self.row("query.type.map")?;
            let (disposition, note) = (r.disposition, r.says.clone());
            if let Some(c) = self.clauses.get_mut(at).filter(|c| c.status.runs()) {
                c.status = c.status.worse(Status::of(disposition));
                c.rows.push("query.type.map".to_string());
                c.notes.extend(note);
            }
        }
        Ok(())
    }

    fn top_read(&mut self, key: &str, value: &Json) -> Result<(), LanguageError> {
        match key {
            "status" => self.reported("status", value, "query.status", None),
            "term" => self.input("term", "query.term", value),
            _ => {
                let row = format!("query.{key}");
                match value {
                    Json::Object(o) => {
                        if let Err(why) = read(Some(value), &["option", "discriminator"]) {
                            return self.unknown(key, value, &why);
                        }
                        let option = o.get("option").cloned().unwrap_or(Json::Null);
                        self.input(key, &row, &option)?;
                        if let Some(d) = o.get("discriminator").filter(|d| !d.is_null()) {
                            self.reported(
                                &format!("{key}.discriminator"),
                                d,
                                "query.name.discriminator",
                                None,
                            )?;
                        }
                        Ok(())
                    }
                    other => self.input(key, &row, other),
                }
            }
        }
    }

    // ---- filters ---------------------------------------------------------------------------

    fn filters(&mut self, filters: &Json) -> Result<(), LanguageError> {
        let Json::Object(groups) = filters else {
            return self.unknown("filters", filters, "`filters` is no object of groups");
        };
        // each filter in the catalogue's order of its row; one no row names
        // after every one a row names, by its key
        let order = |key: &str| {
            self.catalogue
                .rows()
                .iter()
                .position(|r| r.key == key)
                .unwrap_or(usize::MAX)
        };
        // a filter, or a group no filter of which can be read: by its key
        // alone, unsupported, its filters beside it
        enum Entry {
            Filter { disabled: bool, why: Option<String> },
            Group(String),
        }
        let mut all: Vec<(usize, String, Json, Entry)> = Vec::new();
        for (group_id, group) in groups {
            // the site's mark first: a disabled group's filters are
            // disabled whatever else it carries
            let (off, why) = match disabled(group) {
                Ok(true) => (true, None),
                Ok(false) => match read(Some(group), &FILTER_GROUP) {
                    Ok(_) => match group.get("filters") {
                        None | Some(Json::Null | Json::Object(_)) => (false, None),
                        Some(other) => {
                            (false, Some(format!("its `filters` is {other}, no object")))
                        }
                    },
                    Err(why) => (false, Some(why)),
                },
                Err(why) => (false, Some(why)),
            };
            if let Some(why) = &why {
                all.push((
                    usize::MAX,
                    group_id.clone(),
                    group.clone(),
                    Entry::Group(why.clone()),
                ));
            }
            if let Some(Json::Object(each)) = group.get("filters") {
                for (id, value) in each {
                    let key = format!("{group_id}.{id}");
                    let why = why
                        .as_ref()
                        .map(|why| format!("its group `{group_id}` is unread: {why}"));
                    all.push((
                        order(&key),
                        key,
                        value.clone(),
                        Entry::Filter { disabled: off, why },
                    ));
                }
            }
        }
        all.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));
        for (_, key, value, entry) in all {
            match entry {
                Entry::Filter { disabled, why } => self.filter(&key, &value, disabled, why)?,
                Entry::Group(why) => self.unknown(&key, &value, &why)?,
            }
        }
        Ok(())
    }

    /// One filter: the site's `disabled` read first, then its group's
    /// reading, its row, and its value by the keys its row's shape reads.
    fn filter(
        &mut self,
        key: &str,
        value: &Json,
        group_disabled: bool,
        group_why: Option<String>,
    ) -> Result<(), LanguageError> {
        let known = self.catalogue.row(key).is_some();
        let off = disabled(value);
        if group_disabled || off == Ok(true) {
            let mut rows = vec!["stats.filter.disabled".to_string()];
            if known {
                rows.insert(0, key.to_string());
            }
            let mut landed = Landed {
                status: Status::Disabled,
                rows,
                notes: self.note_of("stats.filter.disabled")?,
                node: None,
                reading: None,
            };
            landed
                .notes
                .insert(0, "the site sends it disabled".to_string());
            self.conjunct(key, value, landed);
            return Ok(());
        }
        if !known {
            return self.unknown(key, value, &format!("no row names the filter `{key}`"));
        }
        if let Some(why) = group_why {
            return self.unknown_beside(key, value, &why);
        }
        if let Err(why) = off {
            return self.unknown_beside(key, value, &why);
        }
        let row = self.row(key)?;
        if !Status::of(row.disposition).runs() {
            return self.reported(key, value, key, None);
        }
        let shape = row.shape.clone().unwrap_or_default();
        let fields = match read(Some(value), filter_keys(&shape)) {
            Ok(fields) => fields,
            Err(why) => return self.unknown_beside(key, value, &format!("its value: {why}")),
        };
        if shape == "option" {
            return self.option(key, value, fields.and_then(|f| f.get("option")));
        }
        let mut numbers: Vec<(&str, Option<Number>)> = Vec::new();
        for name in ["min", "max", "r", "g", "b", "w"] {
            match number(fields, name) {
                Ok(n) => numbers.push((name, n)),
                Err(why) => return self.unknown_beside(key, value, &why),
            }
        }
        let n = |name: &str| {
            numbers
                .iter()
                .find(|(n, _)| *n == name)
                .and_then(|(_, v)| *v)
        };
        let made = bounds(n("min"), n("max"));
        let mut landed = self.land(key, &|name| match name {
            "bounds" => made.clone().map(Fill::Made),
            _ => n(name).map(Fill::Number),
        })?;
        // an item lacking the field meets a maximum alone (u061: no
        // requirement meets any maximum), and fails a minimum as ever
        if row.lacked_meets_maximum
            && n("min").is_none()
            && n("max").is_some()
            && let (Some(node), [field]) = (landed.node.take(), row.fields.as_slice())
        {
            landed.node = Some(Node::Any(vec![
                node,
                Node::Not(Box::new(Node::Has(field.clone()))),
            ]));
        }
        self.conjunct(key, value, landed);
        Ok(())
    }

    /// A clause of a row this build reads, unsupported by a value no row
    /// reads: the row named beside `query.unknown`.
    fn unknown_beside(&mut self, key: &str, value: &Json, why: &str) -> Result<(), LanguageError> {
        let mut landed = self.unknown_landed(why)?;
        landed.rows.insert(0, key.to_string());
        self.conjunct(key, value, landed);
        Ok(())
    }

    /// An option filter: an option with a row of its own lands that row;
    /// a row writing `{option}` lands the option; any other row reads
    /// `true` as its text and `false` as the not of it.
    fn option(
        &mut self,
        key: &str,
        value: &Json,
        option: Option<&Json>,
    ) -> Result<(), LanguageError> {
        let option = match option {
            // no option, or the site's "Any": the clause fills nothing and
            // so states nothing, whatever its row writes for an option
            None | Some(Json::Null) => {
                let mut landed = self.land(key, &|_| None)?;
                landed.node = landed.node.map(|_| Node::Const(true));
                self.conjunct(key, value, landed);
                return Ok(());
            }
            Some(Json::String(s)) => s.clone(),
            Some(Json::Bool(b)) => b.to_string(),
            Some(Json::Number(n)) => n.to_string(),
            Some(other) => {
                return self.unknown_beside(key, value, &format!("the option {other} is no word"));
            }
        };
        let own = format!("{key}={option}");
        if self.catalogue.row(&own).is_some() {
            let landed = self.land(&own, &|_| None)?;
            self.conjunct(key, value, landed);
            return Ok(());
        }
        let row = self.row(key)?;
        let writes_option = row.lands.as_deref().is_some_and(|l| l.contains("{option}"));
        if writes_option {
            let landed = self.land(key, &|name| {
                (name == "option").then(|| Fill::Text(option.clone()))
            })?;
            self.conjunct(key, value, landed);
            return Ok(());
        }
        let negated = match option.as_str() {
            "true" => false,
            "false" => true,
            other => {
                return self.unknown_beside(
                    key,
                    value,
                    &format!("the option `{other}` is neither `true` nor `false`"),
                );
            }
        };
        let mut landed = self.land(key, &|_| None)?;
        if negated {
            landed.node = landed.node.map(|n| Node::Not(Box::new(n)));
        }
        self.conjunct(key, value, landed);
        Ok(())
    }

    // ---- stats ---------------------------------------------------------------------------

    /// One stat filter of a group of `kind`, read alone, with its presence:
    /// the stat on the item at all, which it is where it has no bound;
    /// `in_a_sum` where a weighted sum's bound is read on it.
    fn stat(
        &self,
        at: String,
        filter: &Json,
        kind: &str,
        in_a_sum: bool,
    ) -> Result<StatRead, LanguageError> {
        let mut read = StatRead {
            at,
            site: filter.clone(),
            landed: Landed {
                status: Status::Unsupported,
                rows: Vec::new(),
                notes: Vec::new(),
                node: None,
                reading: None,
            },
            presence: None,
            summed: None,
            slot: None,
            bounded: false,
            weight: None,
        };
        // the site's mark first, then the keys the filter and its value read
        match disabled(filter) {
            Ok(true) => {
                read.landed = Landed {
                    status: Status::Disabled,
                    rows: vec!["stats.filter.disabled".to_string()],
                    notes: self.note_of("stats.filter.disabled")?,
                    node: None,
                    reading: None,
                };
                return Ok(read);
            }
            Ok(false) => {}
            Err(why) => {
                read.landed = self.unknown_landed(&why)?;
                return Ok(read);
            }
        }
        let value = match read_stat(filter, kind) {
            Ok(value) => value,
            Err(why) => {
                read.landed = self.unknown_landed(&why)?;
                return Ok(read);
            }
        };
        let Some(Json::String(id)) = filter.get("id") else {
            read.landed = self.unknown_landed("a stat filter with no id")?;
            return Ok(read);
        };
        if value
            .and_then(|v| v.get("option"))
            .is_some_and(|o| !o.is_null())
        {
            read.landed = Landed {
                status: Status::Unsupported,
                rows: vec!["stats.filter.option".to_string()],
                notes: self.note_of("stats.filter.option")?,
                node: None,
                reading: None,
            };
            return Ok(read);
        }
        let (min, max, weight) = match (
            number(value, "min"),
            number(value, "max"),
            number(value, "weight"),
        ) {
            (Ok(min), Ok(max), Ok(weight)) => (min, max, weight),
            (Err(why), _, _) | (_, Err(why), _) | (_, _, Err(why)) => {
                read.landed = self.unknown_landed(&why)?;
                return Ok(read);
            }
        };
        read.bounded = min.is_some() || max.is_some();
        read.weight = weight;
        let made = bounds(min, max);
        let Some((category, rest)) = id.split_once('.') else {
            read.landed = self.stat_unknown(id)?;
            return Ok(read);
        };
        if category == "pseudo" {
            let Some(total) = self.total_named(rest) else {
                let mut notes = self.note_of("stat.pseudo")?;
                notes.push(format!(
                    "no total or reading of the search is the site's `{rest}`"
                ));
                read.landed = Landed {
                    status: Status::Unsupported,
                    rows: vec!["stat.pseudo".to_string()],
                    notes,
                    node: None,
                    reading: None,
                };
                return Ok(read);
            };
            let with = |b: Fill| {
                let total = total.clone();
                move |name: &str| match name {
                    "total" => Some(Fill::Name(total.clone())),
                    "bounds" => Some(match &b {
                        Fill::Made(m) => Fill::Made(m.clone()),
                        _ => Fill::Presence,
                    }),
                    _ => None,
                }
            };
            read.landed = self.land(
                "stat.pseudo",
                &with(made.clone().map_or(Fill::Presence, Fill::Made)),
            )?;
            if presence_too(kind) && read.landed.status.runs() {
                read.presence = self.land("stat.pseudo", &with(Fill::Presence))?.node;
            }
            return Ok(read);
        }
        let key = format!("stat.{category}");
        let Some(row) = self.catalogue.row(&key) else {
            read.landed = self.stat_unknown(id)?;
            return Ok(read);
        };
        if !Status::of(row.disposition).runs() {
            read.landed = Landed {
                status: Status::of(row.disposition),
                rows: vec![key],
                notes: row.says.iter().cloned().collect(),
                node: None,
                reading: None,
            };
            return Ok(read);
        }
        let Some(texts) = self.stats.texts(id) else {
            read.landed = self.stat_unknown(id)?;
            return Ok(read);
        };
        let mut status = Status::of(row.disposition);
        let mut rows = vec![key.clone()];
        let mut notes: Vec<String> = row.says.iter().cloned().collect();
        let rule = |rows: &mut Vec<String>,
                    notes: &mut Vec<String>,
                    status: &mut Status,
                    rule: &str|
         -> Result<(), LanguageError> {
            let r = self.row(rule)?;
            rows.push(rule.to_string());
            notes.extend(r.says.iter().cloned());
            *status = status.worse(Status::of(r.disposition));
            Ok(())
        };
        if texts.len() > 1 {
            rule(&mut rows, &mut notes, &mut status, "stat.text.two")?;
            notes.push(format!(
                "its texts: {}",
                texts
                    .iter()
                    .map(|t| print::quoted_text(t))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            read.landed = Landed {
                status,
                rows,
                notes,
                node: None,
                reading: None,
            };
            return Ok(read);
        }
        let site_text = &texts[0];
        let mut text = site_text.as_str();
        if let Some(stripped) = text.strip_suffix(forms::LOCAL) {
            text = stripped;
            rule(&mut rows, &mut notes, &mut status, "stat.text.local")?;
        }
        let shared = self.stats.ids_of(category, site_text);
        if shared.len() > 1 {
            rule(&mut rows, &mut notes, &mut status, "stat.text.collision")?;
            let others: Vec<String> = shared
                .iter()
                .filter(|other| format!("{category}.{other}") != *id)
                .map(|other| format!("{category}.{other}"))
                .collect();
            notes.push(format!("the text's other id: {}", others.join(", ")));
        }
        if text.contains('\n') {
            rule(&mut rows, &mut notes, &mut status, "stat.text.rows")?;
        }
        // a scourge line in a weighted sum: whether the site sums one is
        // unasked (e043, f005: `scourge-in-sum`)
        if in_a_sum && category == "scourge" {
            rule(&mut rows, &mut notes, &mut status, "stat.scourge.summed")?;
        }
        if kind == "not" && ELDRITCH.iter().any(|form| text.starts_with(form)) {
            rule(&mut rows, &mut notes, &mut status, "stat.text.eldritch")?;
        }
        // a number the site's text writes as a digit is a number of the
        // line: the template has `#` there, its slot open, and the bounds
        // compare the site's own `#` (t30, t31; x01, x03)
        let own = Shown::of(text);
        if own.writes_a_digit() {
            rule(&mut rows, &mut notes, &mut status, "stat.text.numbers")?;
        }
        let shape = template::slots(&own.template);
        let ranged = shape
            .ranged
            .filter(|(low, high)| own.open.contains(low) && own.open.contains(high));
        read.slot = match (own.open.as_slice(), ranged) {
            ([], _) => None,
            (_, Some(_)) => Some("avg".to_string()),
            ([one], None) => Some(format!("arg{one}")),
            ([first, ..], None) => {
                rule(&mut rows, &mut notes, &mut status, "stat.text.many")?;
                Some(format!("arg{first}"))
            }
        };
        if read.slot.is_none() && read.bounded {
            rule(&mut rows, &mut notes, &mut status, "stat.text.bounds")?;
            notes.push("a bound on a text with no `#` is dropped: presence alone".to_string());
        }
        // the stat's other displays (`forms.rs`): every one is its
        // presence, and its plurals alone are summed with the text
        let mut every = vec![own.clone()];
        let mut summed = vec![own];
        let mut unsummed: Vec<String> = Vec::new();
        // the forms no sum holds, where every one is the opposite word: the
        // other reading's line (C119)
        let mut opposite: Option<Vec<Shown>> = Some(Vec::new());
        let forms = self.forms.of(site_text);
        if !forms.refused.is_empty() {
            rule(&mut rows, &mut notes, &mut status, "stat.text.refused")?;
        }
        for refused in &forms.refused {
            notes.push(match &refused.form {
                Some(form) => format!(
                    "{} by the `{}` rule displays as {}, a text the site lists too",
                    print::quoted_text(&refused.text),
                    refused.rule,
                    print::quoted_text(form)
                ),
                None => format!(
                    "{} carries both words of a pair, and which is its number's sign is not said",
                    print::quoted_text(&refused.text)
                ),
            });
        }
        // another wording the game data gives and no capture or sitting has
        // read: matched, and never vouched for
        let inferred: Vec<String> = forms
            .known
            .iter()
            .filter(|f| f.kind == Kind::Other && f.by == "game")
            .map(|f| print::quoted_text(&f.text))
            .collect();
        if !inferred.is_empty() {
            rule(&mut rows, &mut notes, &mut status, "stat.text.wording")?;
            notes.push(format!(
                "its wordings the game data gives: {}",
                inferred.join(", ")
            ));
        }
        for form in forms.known {
            let shown = Shown::of(&form.text);
            if form.kind == Kind::Plural {
                summed.push(shown.clone());
            } else {
                opposite = opposite
                    .filter(|_| form.kind == Kind::Opposite)
                    .map(|mut o| {
                        o.push(shown.clone());
                        o
                    });
                unsummed.push(format!(
                    "{} ({})",
                    print::quoted_text(&form.text),
                    form.kind.word()
                ));
            }
            every.push(shown);
        }
        // a bound read on a stat a form of which no sum can hold: the
        // filter's own, or a weighted sum's over it
        let bound_read = read.slot.is_some() && (read.bounded || in_a_sum);
        if bound_read && !unsummed.is_empty() {
            rule(&mut rows, &mut notes, &mut status, "stat.text.forms")?;
            notes.push(format!(
                "the bound sums the text and its plurals; not its forms {}",
                unsummed.join(", ")
            ));
        }
        let line = |alternatives: &[Shown]| {
            self.land(&key, &|name| match name {
                "text" => Some(Fill::Made(Shown::alternatives(alternatives))),
                _ => None,
            })
        };
        let presence = line(&every)?;
        if presence.status == Status::Unsupported {
            // the binder refused the text: its words, beside the row
            read.landed = presence;
            return Ok(read);
        }
        let summed = line(&summed)?.node;
        read.presence = presence.node.clone();
        read.summed = summed.clone();
        let node = match (&read.slot, bounds_of(min, max), summed) {
            (Some(slot), Some((op, rhs)), Some(lines)) => {
                let Some(value) = tree::sum_of_lines(std::slice::from_ref(&lines), slot) else {
                    return Err(trade(format!("`{}` lands no line to sum", read.at)));
                };
                let compare = Node::Compare { value, op, rhs };
                // a sum of nothing is zero (C95): where the bound admits
                // zero, the line's presence beside it
                if max.is_some() || min.is_some_and(|m| m.as_f64() <= 0.0) {
                    Some(Node::All(vec![lines, compare]))
                } else {
                    Some(compare)
                }
            }
            _ => presence.node,
        };
        // the node made here is bound alone, as a row's text and a group's
        // node are: one the binder refuses — a bound that runs backwards —
        // is this clause unsupported with its words, never the whole
        // translation's defect (the outside review of 6fdcc581)
        if let Some(Err(why)) = node.as_ref().map(alone) {
            let mut unknown = self.unknown_landed(&why)?;
            unknown.rows.insert(0, key.clone());
            read.landed = unknown;
            return Ok(read);
        }
        // the other reading of a bound the opposite word may move (C119):
        // the shape the trade seat asked, a minimum alone on a filter of
        // an `and` group, every form no sum holds the opposite word — an
        // item carrying one the site may put below the bound
        let reading = match opposite {
            Some(forms)
                if kind == "and"
                    && bound_read
                    && !forms.is_empty()
                    && min.is_some()
                    && max.is_none() =>
            {
                match line(&forms)?.node {
                    Some(forms) => Pending::of(
                        self.row("stat.text.forms")?,
                        [("forms", print::print(&forms))],
                    ),
                    None => None,
                }
            }
            _ => None,
        };
        read.landed = Landed {
            status,
            rows,
            notes,
            node,
            reading,
        };
        Ok(read)
    }

    fn stat_unknown(&self, id: &str) -> Result<Landed, LanguageError> {
        let mut notes = self.note_of("stat.unknown")?;
        notes.push(format!("no row of the stat table is `{id}`"));
        Ok(Landed {
            status: Status::Unsupported,
            rows: vec!["stat.unknown".to_string()],
            notes,
            node: None,
            reading: None,
        })
    }

    /// The total or reading the totals table names by the site's pseudo.
    fn total_named(&self, site: &str) -> Option<String> {
        let site = Some(site);
        self.totals
            .totals()
            .iter()
            .find(|t| t.site.as_deref() == site)
            .map(|t| t.name.clone())
            .or_else(|| {
                self.totals
                    .readings()
                    .iter()
                    .find(|r| r.site.as_deref() == site)
                    .map(|r| r.name.clone())
            })
    }

    fn group(&mut self, n: usize, group: &Json) -> Result<(), LanguageError> {
        let at = format!("stats.{n}");
        let Json::Object(object) = group else {
            return self.unknown(&at, group, &format!("{group} is no group"));
        };
        let kind = object.get("type").and_then(Json::as_str).unwrap_or("");
        let group_key = format!("stats.{kind}");
        let site = Json::Object(
            object
                .iter()
                .filter(|(k, _)| k.as_str() != "filters")
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<Map<_, _>>(),
        );
        // the site's mark first, before anything else the group carries: a
        // disabled group is disabled, and so is each filter of it
        let off = disabled(group);
        if off == Ok(true) {
            let notes = self.note_of("stats.filter.disabled")?;
            let landed = |rows: Vec<String>| Landed {
                status: Status::Disabled,
                rows,
                notes: notes.clone(),
                node: None,
                reading: None,
            };
            let disabled = "stats.filter.disabled".to_string();
            self.push(
                &at,
                &site,
                &landed(vec![group_key.clone(), disabled.clone()]),
                Own::Nothing,
            );
            if let Some(Json::Array(filters)) = object.get("filters") {
                for (m, filter) in filters.iter().enumerate() {
                    self.push(
                        &format!("{at}.{m}"),
                        filter,
                        &landed(vec![disabled.clone()]),
                        Own::Nothing,
                    );
                }
            }
            return Ok(());
        }
        // why the group, read whole, cannot be: its mark, a key no row
        // reads, its filters no list, its value; the bounds only a count
        // or a sum reads
        let mut why: Option<String> = off.err();
        let filters: &[Json] = match object.get("filters") {
            Some(Json::Array(f)) => f,
            None | Some(Json::Null) => &[],
            Some(other) => {
                why.get_or_insert(format!("its `filters` is {other}, no list"));
                &[]
            }
        };
        let value_keys: &[&str] = match kind {
            "count" | "weight" | "weight2" => &GROUP_VALUE,
            _ => &[],
        };
        let group_value = match read(Some(group), &STAT_GROUP).and_then(|_| {
            read(object.get("value"), value_keys).map_err(|w| format!("its value: {w}"))
        }) {
            Ok(value) => value,
            Err(w) => {
                why.get_or_insert(w);
                None
            }
        };
        // a weighted sum's bound is read on its stats only where the group
        // has one: with none it sums nothing (t38)
        let bound =
            |key: &str| group_value.is_some_and(|v| v.get(key).is_some_and(|b| !b.is_null()));
        let summed = matches!(kind, "weight" | "weight2") && (bound("min") || bound("max"));
        let reads: Vec<StatRead> = filters
            .iter()
            .enumerate()
            .map(|(m, f)| self.stat(format!("{at}.{m}"), f, kind, summed))
            .collect::<Result<_, _>>()?;
        let row = match self.catalogue.row(&group_key) {
            Some(row) if Status::of(row.disposition).runs() => row,
            found => {
                let landed = match found {
                    Some(row) => Landed {
                        status: Status::of(row.disposition),
                        rows: vec![group_key.clone()],
                        notes: row.says.iter().cloned().collect(),
                        node: None,
                        reading: None,
                    },
                    None => {
                        self.unknown_landed(&format!("no row names the group type `{kind}`"))?
                    }
                };
                return self.refuse_group(&at, &site, landed, reads);
            }
        };
        let group_status = Status::of(row.disposition);
        let group_rows = vec![group_key.clone()];
        let group_notes: Vec<String> = row.says.iter().cloned().collect();
        let group_landed = || Landed {
            status: group_status,
            rows: group_rows.clone(),
            notes: group_notes.clone(),
            node: None,
            reading: None,
        };
        let active: Vec<&StatRead> = reads
            .iter()
            .filter(|r| r.landed.status != Status::Disabled)
            .collect();
        // a group's filters are those the site leaves enabled; with none,
        // `and`, `not` and `if` restrict nothing and write no node — and no
        // clause where none was posted, the page's own empty group — and a
        // count or a sum of none is unasked
        if active.is_empty() && why.is_none() {
            if matches!(kind, "and" | "not" | "if") {
                if !reads.is_empty() {
                    self.push(&at, &site, &group_landed(), Own::Conjuncts);
                    for read in reads {
                        self.conjunct(&read.at, &read.site, read.landed);
                    }
                }
                return Ok(());
            }
            why = Some(
                "no filter of it is enabled, and what the site makes of a group of none is unasked"
                    .to_string(),
            );
        }
        if kind == "and" && why.is_none() {
            self.push(&at, &site, &group_landed(), Own::Conjuncts);
            for read in reads {
                self.conjunct(&read.at, &read.site, read.landed);
            }
            return Ok(());
        }
        if why.is_none() {
            why = active
                .iter()
                .find(|r| r.landed.status == Status::Unsupported)
                .map(|r| format!("its filter `{}` is unsupported", r.at));
        }
        let (gmin, gmax) = match (number(group_value, "min"), number(group_value, "max")) {
            (Ok(a), Ok(b)) => (a, b),
            (Err(e), _) | (_, Err(e)) => {
                why.get_or_insert(e);
                (None, None)
            }
        };
        // a group with no bound of its own reads as the sitting found it: a
        // count restricts nothing (t37), each stat the site's no-op as an
        // `if` writes one; a weighted sum needs one of its stats present
        // (t38), the stats' presences one or the other
        let kind = match (kind, gmin.is_none() && gmax.is_none()) {
            ("count", true) => "count_none",
            // only where no stat carries a bound of its own: a guarded
            // stat in a sum of no bound is no shape a sitting has asked,
            // and stays K5's refusal (the outside review of 8f293ff3)
            ("weight" | "weight2", true) if !active.iter().any(|r| r.bounded) => "present",
            (kind, _) => kind,
        };
        // what the group writes, each node bound alone as a filter's text
        // is: one the binder refuses is the group unsupported, its words
        let composed = match why {
            Some(why) => Err(why),
            None => compose(kind, &active, gmin, gmax)?
                .and_then(|nodes| nodes.iter().try_for_each(alone).map(|()| nodes)),
        };
        let mut nodes = match composed {
            Ok(nodes) => nodes,
            Err(why) => {
                let mut landed = group_landed();
                landed.status = Status::Unsupported;
                landed.notes.push(format!("unsupported whole: {why}"));
                return self.refuse_group(&at, &site, landed, reads);
            }
        };
        match kind {
            // one stat present is that stat's presence, a term of its own
            "present" if active.len() == 1 => {
                self.push(&at, &site, &group_landed(), Own::Conjuncts);
                let mut nodes = nodes.into_iter();
                for read in &reads {
                    let mut landed = clone_landed(&read.landed);
                    if read.landed.status != Status::Disabled {
                        landed.node = nodes.next();
                        self.conjunct(&read.at, &read.site, landed);
                    } else {
                        self.push(&read.at, &read.site, &landed, Own::Nothing);
                    }
                }
            }
            "not" | "count" | "present" => {
                let mut filters = Vec::new();
                for read in &active {
                    let mut landed = clone_landed(&read.landed);
                    landed.node = None;
                    filters.push(landed);
                }
                let g = self.push(&at, &site, &group_landed(), Own::Path { parent: None });
                let mut owners = vec![(g, String::new())];
                let one = active.len() == 1;
                self.place(&reads, filters, g, &mut owners, |m| match (kind, one) {
                    ("not", true) => "0".to_string(),
                    ("not", false) => format!("0.{m}"),
                    _ => m.to_string(),
                });
                self.pieces.push(Piece {
                    node: nodes.remove(0),
                    owners,
                });
            }
            "if" | "count_none" => {
                self.push(&at, &site, &group_landed(), Own::Conjuncts);
                let mut nodes = nodes.into_iter();
                for read in &reads {
                    let mut landed = clone_landed(&read.landed);
                    if read.landed.status == Status::Disabled {
                        self.push(&read.at, &read.site, &landed, Own::Nothing);
                        continue;
                    }
                    landed.node = nodes.next();
                    self.conjunct(&read.at, &read.site, landed);
                }
            }
            _ => {
                let g = self.push(&at, &site, &group_landed(), Own::Path { parent: None });
                for read in &reads {
                    let own = if read.landed.status == Status::Disabled {
                        Own::Nothing
                    } else {
                        Own::InsideSum
                    };
                    let mut landed = clone_landed(&read.landed);
                    landed.reading = self.without(read, &active, gmin, gmax, g)?;
                    self.push(&read.at, &read.site, &landed, own);
                }
                self.pieces.push(Piece {
                    node: nodes.remove(0),
                    owners: vec![(g, String::new())],
                });
            }
        }
        Ok(())
    }

    /// Each filter of a group that writes a node, placed under it; a
    /// disabled one reported alone.
    fn place(
        &mut self,
        reads: &[StatRead],
        filters: Vec<Landed>,
        group: usize,
        owners: &mut Vec<(usize, String)>,
        path: impl Fn(usize) -> String,
    ) {
        let mut filters = filters.into_iter();
        let mut m = 0;
        for read in reads {
            if read.landed.status == Status::Disabled {
                let landed = clone_landed(&read.landed);
                self.push(&read.at, &read.site, &landed, Own::Nothing);
                continue;
            }
            let Some(mut landed) = filters.next() else {
                break;
            };
            landed.node = read.landed.node.clone();
            let i = self.push(
                &read.at,
                &read.site,
                &landed,
                Own::Path {
                    parent: Some(group),
                },
            );
            owners.push((i, path(m)));
            m += 1;
        }
    }

    /// The other reading of a scourge line in a bounded weighted sum
    /// (C119): the sum with that line left out, against the group's
    /// bounds, standing in for the group's node; none for any other line,
    /// or where no other line is left to sum.
    fn without(
        &self,
        read: &StatRead,
        active: &[&StatRead],
        min: Option<Number>,
        max: Option<Number>,
        group: usize,
    ) -> Result<Option<Pending>, LanguageError> {
        if !read.landed.rows.iter().any(|r| r == "stat.scourge.summed") {
            return Ok(None);
        }
        let others: Vec<&StatRead> = active
            .iter()
            .copied()
            .filter(|other| other.at != read.at)
            .collect();
        if others.is_empty() {
            return Ok(None);
        }
        let Ok(Weighted { sum, op, rhs }) = weighted(&others, min, max) else {
            return Ok(None);
        };
        let node = Node::Compare {
            value: sum,
            op,
            rhs,
        };
        Ok(Pending::of(
            self.row("stat.scourge.summed")?,
            [("without", print::print(&node))],
        )
        .map(|p| Pending {
            group: Some(group),
            ..p
        }))
    }

    /// A group in the remainder, every filter of it beside it: a filter
    /// unsupported for itself keeps its own words, every other one says its
    /// group's.
    fn refuse_group(
        &mut self,
        at: &str,
        site: &Json,
        group: Landed,
        reads: Vec<StatRead>,
    ) -> Result<(), LanguageError> {
        self.push(at, site, &group, Own::Nothing);
        for read in reads {
            let mut landed = read.landed;
            landed.node = None;
            if landed.status.runs() {
                landed.status = Status::Unsupported;
                landed.notes = vec![format!(
                    "its group `{at}` translates whole or not at all, and is unsupported"
                )];
            }
            self.push(&read.at, &read.site, &landed, Own::Nothing);
        }
        Ok(())
    }

    // ---- the whole ----------------------------------------------------------------------

    fn finish(
        mut self,
        decoded: &Decoded,
        catalogue: u32,
        stats: &str,
        forms: u32,
    ) -> Result<Translation, LanguageError> {
        let pieces = std::mem::take(&mut self.pieces);
        let single = pieces.len() == 1;
        let mut nodes = Vec::with_capacity(pieces.len());
        for (k, piece) in pieces.into_iter().enumerate() {
            let at = match (single, &piece.node) {
                (true, Node::All(_)) => String::new(),
                (true, _) => tree::sole_path(""),
                (false, _) => tree::child_path("", k),
            };
            for (i, rel) in &piece.owners {
                let path = match (at.is_empty(), rel.is_empty()) {
                    (_, true) => at.clone(),
                    (true, false) => rel.clone(),
                    (false, false) => format!("{at}.{rel}"),
                };
                self.clauses[*i].path = Some(path);
            }
            nodes.push(piece.node);
        }
        let root = match nodes.len() {
            1 => nodes.remove(0),
            _ => Node::All(nodes),
        };
        let text = print::print(&root);
        let read = bind::parse_query(&text).map_err(|e| {
            trade(format!(
                "the translation wrote `{text}`, which this build refuses: {}",
                e.message
            ))
        })?;
        if read.tree() != &root {
            return Err(trade(format!(
                "the translation wrote `{text}`, which does not read back as the tree it printed"
            )));
        }
        // each running clause's term: its node, as the text prints it
        let printed: HashMap<String, String> = tree::nodes(&root)
            .into_iter()
            .map(|p| (p.path, p.text))
            .collect();
        for clause in &mut self.clauses {
            clause.term = match &clause.path {
                Some(p) if p.is_empty() => Some(text.clone()),
                Some(p) => printed.get(p).cloned(),
                None => None,
            };
        }
        self.readings(&printed, &text)?;
        guard(&root, &self.clauses, &self.own).map_err(|why| {
            trade(format!(
                "the translation of `{text}` breaks its guard: {why}"
            ))
        })?;
        let remainder: Vec<String> = self
            .clauses
            .iter()
            .filter(|c| c.status == Status::Unsupported)
            .map(|c| c.at.clone())
            .collect();
        let fidelity = if !remainder.is_empty() {
            Fidelity::Partial
        } else if self.clauses.iter().any(|c| c.status == Status::Ambiguous) {
            Fidelity::Ambiguous
        } else {
            Fidelity::Exact
        };
        Ok(Translation {
            source: decoded.source,
            realm: decoded.realm.clone(),
            text,
            tree: root,
            clauses: self.clauses,
            remainder,
            fidelity,
            catalogue,
            stats: stats.to_string(),
            forms,
        })
    }
}

impl Translator<'_> {
    /// Each running clause's other reading (C119), its `{term}` the text
    /// of the node it stands in for: where that node is a conjunct of the
    /// query whole — the root's child, or the root — and every placeholder
    /// fills; any other clause has none. A reading this build's binder
    /// refuses is a table defect, said.
    fn readings(
        &mut self,
        printed: &HashMap<String, String>,
        text: &str,
    ) -> Result<(), LanguageError> {
        for i in 0..self.clauses.len() {
            let Some(pending) = self.pending[i].take() else {
                continue;
            };
            let path = match pending.group {
                Some(g) => self.clauses[g].path.clone(),
                None => self.clauses[i].path.clone(),
            };
            let Some(path) = path.filter(|p| !p.contains('.')) else {
                continue;
            };
            let term = if path.is_empty() {
                text.to_string()
            } else {
                match printed.get(&path) {
                    Some(t) => t.clone(),
                    None => continue,
                }
            };
            let mut fills = pending.fills;
            fills.insert("term".to_string(), term);
            if placeholders(&pending.template)
                .iter()
                .any(|name| !fills.contains_key(*name))
            {
                continue;
            }
            let written = substituted(&pending.template, &|name| {
                fills.get(name).cloned().unwrap_or_default()
            });
            let node = bind::parse_query(&written)
                .map(|q| q.tree().clone())
                .map_err(|e| {
                    trade(format!(
                        "the other reading of `{}` wrote `{written}`, which this build refuses: {}",
                        self.clauses[i].at, e.message
                    ))
                })?;
            self.clauses[i].other = Some(Reading {
                text: print::print(&node),
                path,
                node,
            });
        }
        Ok(())
    }
}

/// Whether a node is a constant: a clause that states nothing.
fn is_const(node: &Node) -> bool {
    matches!(node, Node::Const(_))
}

/// What a group other than `and` writes (K6; the group rows' notes): `not`
/// and `count` one node over its filters' nodes, a weighted sum one
/// comparison, `if` one node per filter; why it cannot be written, or a
/// build defect.
fn compose(
    kind: &str,
    active: &[&StatRead],
    min: Option<Number>,
    max: Option<Number>,
) -> Result<Result<Vec<Node>, String>, LanguageError> {
    let each = || {
        active
            .iter()
            .map(|r| written(r))
            .collect::<Result<Vec<_>, _>>()
    };
    Ok(match kind {
        "not" => {
            let mut nodes = each()?;
            let inner = if nodes.len() == 1 {
                nodes.remove(0)
            } else {
                Node::Any(nodes)
            };
            Ok(vec![Node::Not(Box::new(inner))])
        }
        "count" => match count_bounds(min, max) {
            Ok((lo, hi)) => Ok(vec![Node::Holds {
                of: each()?,
                min: lo,
                max: hi,
            }]),
            Err(why) => Err(why),
        },
        "count_none" => Ok(active.iter().map(|_| Node::Const(true)).collect()),
        "present" => {
            let mut presences = active
                .iter()
                .map(|r| {
                    r.presence
                        .clone()
                        .ok_or_else(|| trade(format!("`{}` runs and writes no presence", r.at)))
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(vec![if presences.len() == 1 {
                presences.remove(0)
            } else {
                Node::Any(presences)
            }])
        }
        // per stat: not on the item, or on it within its bounds; a stat
        // with none is the site's own no-op
        "if" => Ok(active
            .iter()
            .map(|read| match (&read.presence, &read.landed.node) {
                _ if !read.bounded => Ok(Node::Const(true)),
                (Some(presence), Some(bounded)) => Ok(Node::Any(vec![
                    Node::Not(Box::new(presence.clone())),
                    bounded.clone(),
                ])),
                _ => Err(trade(format!("`{}` runs and writes no presence", read.at))),
            })
            .collect::<Result<Vec<_>, _>>()?),
        _ => weighted(active, min, max).map(|Weighted { sum, op, rhs }| {
            vec![Node::Compare {
                value: sum,
                op,
                rhs,
            }]
        }),
    })
}

/// A node a group composes, bound alone as a filter's text is (`land`):
/// the binder's words where it refuses it.
fn alone(node: &Node) -> Result<(), String> {
    let text = print::print(node);
    bind::parse_query(&text)
        .map(|_| ())
        .map_err(|e| format!("`{text}` is refused: {}", e.message))
}

/// A running filter's node: every running stat row lands a text, so one
/// with none is a build defect, said, never a term made up for it.
fn written(read: &StatRead) -> Result<Node, LanguageError> {
    read.landed
        .node
        .clone()
        .ok_or_else(|| trade(format!("`{}` runs and writes no text", read.at)))
}

fn clone_landed(l: &Landed) -> Landed {
    Landed {
        status: l.status,
        rows: l.rows.clone(),
        notes: l.notes.clone(),
        node: l.node.clone(),
        reading: l.reading.clone(),
    }
}

/// A count's bounds: whole numbers, none below nothing, one at least.
fn count_bounds(
    min: Option<Number>,
    max: Option<Number>,
) -> Result<(Option<u32>, Option<u32>), String> {
    let whole = |n: Option<Number>| -> Result<Option<u32>, String> {
        match n {
            None => Ok(None),
            Some(Number::Int(i)) => u32::try_from(i)
                .map(Some)
                .map_err(|_| format!("its bound {i} is no count")),
            Some(Number::Float(f)) => Err(format!("its bound {f} is no whole count")),
        }
    };
    match (whole(min)?, whole(max)?) {
        (None, None) => Err(
            "it has no bound, and what the site makes of a count with none is unasked".to_string(),
        ),
        bounds => Ok(bounds),
    }
}

/// What a weighted sum compares: the sum over its lines, and the group's
/// bounds over the weight.
struct Weighted {
    sum: ValueRef,
    op: Op,
    rhs: Value,
}

/// A weighted sum's terms (K5): every filter a line, no filter bounded,
/// one weight above nothing on every filter, one slot, and the group's
/// bounds over the weight exact decimals.
fn weighted(
    filters: &[&StatRead],
    min: Option<Number>,
    max: Option<Number>,
) -> Result<Weighted, String> {
    let mut weight: Option<Number> = None;
    let mut slots = std::collections::BTreeSet::new();
    let mut lines = Vec::new();
    for read in filters {
        let Some(line @ Node::Members { .. }) = &read.summed else {
            return Err(format!("its filter `{}` is no line", read.at));
        };
        if read.bounded {
            return Err(format!(
                "its filter `{}` carries a bound of its own, a guard with no representation",
                read.at
            ));
        }
        let Some(w) = read.weight else {
            return Err(format!("its filter `{}` has no weight", read.at));
        };
        match weight {
            Some(seen) if seen != w => {
                return Err("its filters carry different weights".to_string());
            }
            _ => weight = Some(w),
        }
        let Some(slot) = &read.slot else {
            return Err(format!("its filter `{}` has no number to sum", read.at));
        };
        slots.insert(slot.clone());
        lines.push(line.clone());
    }
    let Some(w) = weight else {
        return Err("it has no filter".to_string());
    };
    if w.as_f64() <= 0.0 {
        return Err(format!(
            "its weight {} is not above zero",
            print::number_text(w)
        ));
    }
    let slot = match slots.into_iter().collect::<Vec<_>>().as_slice() {
        [one] => one.clone(),
        _ => return Err("its lines are ranged and not ranged together: no one slot".to_string()),
    };
    let sum =
        tree::sum_of_lines(&lines, &slot).ok_or_else(|| "a filter of it is no line".to_string())?;
    if min.is_none() && max.is_none() {
        return Err(
            "it has no bound, and what the site makes of a sum with none is unasked".to_string(),
        );
    }
    let over = |n: Option<Number>| -> Result<Option<Number>, String> {
        let Some(n) = n else { return Ok(None) };
        let q = Ratio::of(Exact::of(n.as_f64()))
            .over(Ratio::of(Exact::of(w.as_f64())))
            .ok_or_else(|| "its bound over its weight overflows".to_string())?;
        let cut = Exact::cut(q).ok_or_else(|| "its bound over its weight overflows".to_string())?;
        if Ratio::of(cut) != q {
            return Err(format!(
                "its bound {} over its weight {} is no exact decimal",
                print::number_text(n),
                print::number_text(w)
            ));
        }
        Ok(Some(Number::from_f64(cut.as_f64())))
    };
    let (op, rhs) =
        bounds_of(over(min)?, over(max)?).ok_or_else(|| "it has no bound".to_string())?;
    Ok(Weighted { sum, op, rhs })
}

/// K6's guard over a translation (the module doc).
fn guard(root: &Node, clauses: &[Clause], own: &[Own]) -> Result<(), String> {
    let paths: Vec<String> = tree::nodes(root).into_iter().map(|p| p.path).collect();
    let known: HashSet<&str> = paths.iter().map(String::as_str).collect();
    let under = |path: &str, owner: &str| {
        owner.is_empty() || path == owner || path.starts_with(&format!("{owner}."))
    };
    let mut owned: HashSet<&str> = HashSet::new();
    for (clause, own) in clauses.iter().zip(own) {
        let runs = clause.status.runs();
        match (own, &clause.path) {
            (Own::Path { parent }, Some(path)) if runs => {
                if !path.is_empty() && !known.contains(path.as_str()) {
                    return Err(format!(
                        "`{}` owns {path:?}, no path of the tree",
                        clause.at
                    ));
                }
                if !owned.insert(path) {
                    return Err(format!("{path:?} is owned twice"));
                }
                if let Some(g) = parent {
                    let gp = clauses[*g].path.as_deref().unwrap_or("-");
                    if path == gp || !under(path, gp) {
                        return Err(format!(
                            "`{}` at {path:?} is not inside its group's {gp:?}",
                            clause.at
                        ));
                    }
                }
            }
            (Own::Conjuncts | Own::InsideSum, None) if runs => {}
            (Own::Nothing, None) if !runs => {}
            _ => {
                return Err(format!(
                    "`{}` ({:?}) owns {:?}",
                    clause.at, clause.status, clause.path
                ));
            }
        }
    }
    // a path is a clause's when it or a path above it is owned, the root
    // `""` above every one
    let owner_of = |path: &str| {
        owned.contains("")
            || owned.contains(path)
            || path
                .match_indices('.')
                .any(|(at, _)| owned.contains(&path[..at]))
    };
    if let Some(stray) = paths.iter().find(|p| !owner_of(p)) {
        return Err(format!("{stray:?} is no clause's"));
    }
    Ok(())
}
