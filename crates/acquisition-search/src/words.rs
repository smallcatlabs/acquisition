//! The words a frontend says this crate's documents in: an answer's, a
//! shown item's, a count's, a site search's and the language's index's
//! fragments, each a function of the document it reads (C53, text is a
//! function of the JSON) and the moment it is said at, made once here so
//! every door that links the crate says them alike (C46, shared semantics
//! in Rust). A fragment names no frontend's flag or printed command
//! (C143): a door assembles its own ways on around them.

pub mod facts;

use acquisition_store::age::ago;

use crate::Priced;
use crate::answer::{
    Answer, ClauseOut, Count, Link, Membership, PLACE_ORDER_WORDS, TradeOut, ViewOut,
};
use crate::corpus::Realm;
use crate::counts::{Bucket, CrossOut, Label, SumOf, Summed, Table};
use crate::show::{Shown, ShownItem, ShownLine};
use crate::trade::Source;
use crate::trade::translate::{Fidelity, Status};

/// `n` and its noun, `one` for exactly one and `many` otherwise: the
/// count words every text prints, from one copy.
pub fn plural(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

/// The basis line every answer and every `show` prints (C98). The intent
/// file's revision is said as what it holds, the hand-written prices and
/// the sync policy (C146, a count says what it counts), where `intent 33`
/// reads as a count nobody can name.
pub fn basis_text(b: &crate::Basis) -> String {
    format!(
        "basis   store {} · build {} · snapshot {} · facts v{} · prices and policy revision {} · derivation {} · classes v{} · categories v{} · beast colours v{} · totals v{} · defences v{} · currency v{} · notes v{}",
        b.store,
        b.build,
        b.snapshot.response,
        b.snapshot.facts_version,
        b.intent,
        b.derivation,
        b.classes,
        b.categories,
        b.beast_colours,
        b.totals,
        b.defences,
        b.currency,
        b.notes
    )
}

fn number(value: &serde_json::Value) -> String {
    value.to_string()
}

/// A slot's number as `show` prints one: whole where it is, `null` where
/// it could not be read.
fn slot_number(n: Option<f64>) -> String {
    match n.map(crate::Number::from_f64) {
        None => "null".to_string(),
        Some(crate::Number::Int(n)) => n.to_string(),
        Some(crate::Number::Float(n)) => n.to_string(),
    }
}

/// A name and a type line as the game shows them together; a rendering,
/// never a field (`derive.rs`).
pub fn title(name: Option<&str>, typeline: Option<&str>) -> String {
    match (name, typeline) {
        (Some(name), Some(typeline)) => format!("{name} {typeline}"),
        (name, typeline) => name.or(typeline).unwrap_or("(no name)").to_string(),
    }
}

/// How many types the scope line names among the never fetched; the
/// rest are counted (C118: a map search sees `MapStash` at once).
const NEVER_FETCHED_NAMED: usize = 3;

/// What the never-fetched count spans, and why: said beside the count on
/// every surface (the MCP's `never_fetched_by_type`, the field's doc).
const NEVER_FETCHED_SPAN: &str = "counted over every location in the realm, in every league, whatever the query asks, since what a never-fetched location holds is unknown";

/// The never fetched, said by type: ` never fetched: MapStash 2489 of
/// 2582, …`, the largest named and the rest counted.
fn never_fetched_text(by_type: &[crate::tabs::TabType]) -> String {
    if by_type.is_empty() {
        return " never fetched".to_string();
    }
    let mut named: Vec<String> = by_type
        .iter()
        .take(NEVER_FETCHED_NAMED)
        .map(|t| format!("{} {} of {}", t.kind, t.tabs - t.fetched, t.tabs))
        .collect();
    let rest = &by_type[by_type.len().min(NEVER_FETCHED_NAMED)..];
    if !rest.is_empty() {
        named.push(format!(
            "{} more in {}",
            rest.iter().map(|t| t.tabs - t.fetched).sum::<usize>(),
            plural(rest.len(), "other type", "other types")
        ));
    }
    // why a ring query lists map tabs: the count ignores the query, and
    // the line says so as the MCP description does (C146)
    format!(
        " never fetched: {} ({NEVER_FETCHED_SPAN})",
        named.join(", ")
    )
}

pub fn place_text(place: &crate::corpus::Place, now: i64) -> String {
    format!(
        "{} · {}",
        place_words(place),
        fetched_words(place.fetched_at.map(|at| ago(now, Some(at))))
    )
}

/// How old a place is, from its fetch's age, as `tabs` says a tab's age
/// (C118: "where do I pick it up" asks it).
pub fn fetched_words(age: Option<String>) -> String {
    match age {
        Some(age) => format!("fetched {age}"),
        None => "never fetched".to_string(),
    }
}

/// Where an item lies, without how old that is: its league, its tab or
/// character and the tab's page, its cell, what it is socketed in. The
/// table's location column, and the head of [`place_text`].
pub fn place_words(place: &crate::corpus::Place) -> String {
    let mut out = place
        .league
        .clone()
        .unwrap_or_else(|| "no league".to_string());
    if let Some(parent) = &place.parent {
        out.push_str(&format!(
            " / {}",
            parent.name.as_deref().unwrap_or(&parent.id)
        ));
    }
    // a tab's page with no name of its own is the tab's page, by its id
    out.push_str(&match (&place.name, &place.parent) {
        (Some(name), _) => format!(" / {name}"),
        (None, Some(_)) => format!(" / page {}", place.id),
        (None, None) => format!(" / {}", place.id),
    });
    if place.kind == "character" {
        out.push_str(&format!(
            " ({})",
            place.container.as_deref().unwrap_or("character")
        ));
    }
    if let Some(cell) = &place.cell {
        out.push_str(&format!(" · x {}, y {}", cell.x, cell.y));
    }
    if let Some(parent) = &place.socketed_in {
        out.push_str(&format!(
            " · socketed in {}",
            parent.name.as_deref().unwrap_or(&parent.id)
        ));
    }
    out
}

/// One piece of a row's evidence, as a term's or a sort's.
fn evidence_text(evidence: &crate::answer::Evidence) -> String {
    use crate::answer::Evidence;
    match evidence {
        Evidence::Line {
            source,
            flags,
            text,
        } => {
            let kind: Vec<&str> = std::iter::once(source.as_str())
                .chain(flags.iter().map(String::as_str))
                .collect();
            format!("{} ({})", text.replace('\n', " / "), kind.join(", "))
        }
        Evidence::Shown { part, text } => format!("{text} ({part})"),
        Evidence::Value { name, value } => format!("{name} {}", json_text(value)),
        Evidence::Undecided { path, term, reason } => format!(
            "term {path} {term} is undecided: {} unread — {}; {}",
            reason.unread, reason.problem, reason.hint
        ),
    }
}

/// How far a translated text is the site's search, from where it came.
pub fn fidelity_words(t: &TradeOut) -> String {
    let from = match t.source {
        Source::Url => "a URL",
        Source::Id => "a search id",
        Source::Json => "the query JSON",
    };
    let count = |status: Status| t.clauses.iter().filter(|c| c.status == status).count();
    match t.fidelity {
        Fidelity::Partial => format!(
            "from {from}: PARTIAL — {} not run, so this is not the site's search",
            plural(count(Status::Unsupported), "clause", "clauses")
        ),
        Fidelity::Ambiguous => {
            let ambiguous = count(Status::Ambiguous);
            format!(
                "from {from}: ambiguous — every clause ran, {ambiguous} as the site may not read {}",
                if ambiguous == 1 { "it" } else { "them" }
            )
        }
        Fidelity::Exact => format!(
            "from {from}: exact — every clause with a stash meaning ran as the site reads it"
        ),
    }
}

/// A site search's link back to the site (C195), as each surface says
/// it: the URL, or `none` and why; what beside the query the site cannot
/// be asked; each note. A word and what it says, the page drawing the
/// URL as its link.
pub fn link_words(link: &Link) -> Vec<(&'static str, String)> {
    match link {
        Link::None { none } => vec![("link", format!("none — {none}"))],
        Link::Url {
            url,
            left_out,
            notes,
        } => {
            let mut out = vec![("link", url.clone())];
            if !left_out.is_empty() {
                out.push((
                    "left out",
                    format!(
                        "{} — beside the query, the site cannot be asked {}",
                        left_out.join(", "),
                        if left_out.len() == 1 { "it" } else { "them" }
                    ),
                ));
            }
            out.extend(notes.iter().map(|n| ("note", n.clone())));
            out
        }
    }
}

/// What a translated search runs over, said beside every one.
pub const TRADE_OVER: &str = "the items the store has fetched, priced or not — what it never fetched is \
     the scope's: the site shows public listings, a tab's being tab.public, so the search is the \
     site's and the items are yours";
/// The clauses a translation lists, by status in the text's order — not
/// run, ambiguous, reported — each with its index among the report's
/// clauses: every one the document holds; and, last, each exact clause
/// that matches nothing, a range that runs backwards, whose term
/// never says why — known by its row, whatever the term's spelling.
pub fn clauses_listed(t: &TradeOut) -> [(&'static str, Vec<(usize, &ClauseOut)>); 4] {
    let of = |keep: &dyn Fn(Status) -> bool| -> Vec<(usize, &ClauseOut)> {
        t.clauses
            .iter()
            .enumerate()
            .filter(|(_, c)| keep(c.status))
            .collect()
    };
    [
        ("NOT RUN", of(&|s| s == Status::Unsupported)),
        ("ambiguous", of(&|s| s == Status::Ambiguous)),
        (
            "reported",
            of(&|s| matches!(s, Status::ListingOnly | Status::Disabled)),
        ),
        (
            "nothing",
            t.clauses
                .iter()
                .enumerate()
                .filter(|(_, c)| c.status == Status::Exact && c.matches_nothing())
                .collect(),
        ),
    ]
}

/// What a listed clause says: its last note.
pub fn clause_note(c: &ClauseOut) -> &str {
    c.notes.last().map_or("", String::as_str)
}

/// An ambiguous clause's other reading, its adds and drops as each
/// surface writes a count.
pub fn stake_words(adds: &str, drops: &str) -> String {
    format!("read the other way: adds {adds}, drops {drops}")
}

/// The query as an answer states it: its text, or what an empty one is.
pub fn query_words(text: &str) -> &str {
    if text.is_empty() {
        "(every item in scope)"
    } else {
        text
    }
}

/// The query typed beside the corpus context, set aside `under` the
/// surface's spelling of that context.
pub fn aside_words(text: &str, under: &str) -> String {
    format!("{text} — set aside{under}, counted nowhere")
}

/// The scope's first line: the account, the realm, the membership with
/// its counts and the locations fetched.
pub fn scope_words(scope: &crate::answer::ScopeOut) -> String {
    let account = scope.account.name.as_deref().unwrap_or(&scope.account.uuid);
    format!(
        "account {account} · {} · {} · {} fetched",
        scope.realm.as_str(),
        match (scope.membership, scope.coverage.removed) {
            (Membership::Removed, Some(removed)) => format!(
                "removed · {removed} removed · {} live not searched",
                plural(scope.coverage.items, "item", "items"),
            ),
            (_, Some(removed)) => format!(
                "all · {} live · {} removed",
                plural(scope.coverage.items, "item", "items"),
                removed
            ),
            (_, None) => format!("live · {}", plural(scope.coverage.items, "item", "items")),
        },
        plural(scope.coverage.fetched, "location", "locations"),
    )
}

/// The scope's never-fetched locations by type and the lists' age. An
/// age over several lists names the set it is the oldest of
/// (else `location list seen 57d ago` reads as the stash list's, 53 days
/// old, when it is a character list's).
pub fn never_fetched_words(scope: &crate::answer::ScopeOut, now: i64) -> String {
    format!(
        "{}{} · {}",
        plural(scope.coverage.never_fetched, "location", "locations"),
        never_fetched_text(&scope.coverage.never_fetched_by_type),
        match scope.coverage.span.list_seen {
            Some(_) => format!(
                "location lists seen, the oldest {}",
                ago(now, scope.coverage.span.list_seen)
            ),
            None => "no location list seen".to_string(),
        },
    )
}

/// The other leagues holding never-fetched tabs, beside the one pointed at.
pub fn other_leagues_words(n: usize) -> String {
    format!(
        "{} never-fetched tabs",
        plural(n, "other league holds", "other leagues hold")
    )
}

/// A named realm the store holds nothing under, and what it holds.
pub fn realm_not_held_words(scope: &crate::answer::ScopeOut) -> Option<String> {
    scope.realm_not_held.then(|| {
        format!(
            "this store holds nothing under {}: it holds {}",
            scope.realm.as_str(),
            if scope.coverage.realms_held.is_empty() {
                "no realm".to_string()
            } else {
                scope.coverage.realms_held.join(", ")
            }
        )
    })
}

/// The total and the matches' fetch span, the undecided apart.
pub fn total_words(a: &Answer, now: i64) -> String {
    let mut total = plural(a.total.matched, "match", "matches");
    if let Some(span) = &a.total.span
        && let (Some(oldest), Some(newest)) = (span.oldest_fetch, span.newest_fetch)
    {
        // the matches' ages, not the scope's: the scope's oldest fetch
        // reads as the matches'. Each age is one location's last fetch
        // (C146: "newest 4d ago" names no location); a row's place carries
        // its own age, which the line need not say again (a line carries
        // its counts, not an instruction); each names its location as a
        // row's place does, league first, and one location is named once
        total.push_str(&format!(
            " ({})",
            span_words(span, ago(now, Some(oldest)), ago(now, Some(newest)))
        ));
    }
    total
}

/// A count of items undecided.
pub fn undecided_words(n: usize) -> String {
    format!("{n} undecided")
}

/// Under every total, not a zero's alone: a count of N is as blind to
/// the never-fetched as a count of none. The number agrees through the
/// sentence: "it holds" for one.
pub fn never_fetched_sentence(n: usize) -> Option<String> {
    match n {
        0 => None,
        1 => Some("1 location was never fetched: nothing here says what it holds".to_string()),
        n => Some(format!(
            "{n} locations were never fetched: nothing here says what they hold"
        )),
    }
}

/// What a route said when it was printed, where it runs on a later basis.
pub fn before_words(a: &Answer) -> Option<String> {
    let before = a.before.as_ref()?;
    Some(match &before.counted_at {
        Some(basis) => format!(
            "{} where this route was printed — {}",
            plural(before.count, "match", "matches"),
            basis.against(&a.basis)
        ),
        None => format!(
            "{} where this route was printed",
            plural(before.count, "match", "matches")
        ),
    })
}

/// A fetch span's extremes as a line says them, each with its own age:
/// two locations named apart, one named once, each named as a row's place
/// is, realm and league first (else a poe2 character reads as pc's).
pub fn span_words(
    span: &acquisition_store::coverage::FetchSpan,
    oldest: String,
    newest: String,
) -> String {
    let at = |l: &Option<acquisition_store::coverage::FetchedLocation>| {
        l.as_ref().map_or(String::new(), |l| {
            format!(
                ", {} · {} / {}{}",
                l.realm,
                l.league.as_deref().unwrap_or("no league"),
                l.name.as_deref().unwrap_or(&l.id),
                if l.kind == "character" {
                    " (character)"
                } else {
                    ""
                }
            )
        })
    };
    if span.oldest_fetch_location == span.newest_fetch_location {
        format!("fetched {oldest}{}", at(&span.oldest_fetch_location))
    } else {
        format!(
            "the oldest fetch {oldest}{}; the newest {newest}{}",
            at(&span.oldest_fetch_location),
            at(&span.newest_fetch_location),
        )
    }
}

/// A row's line in its parts, each in words: the one maker of what the
/// CLI's row and the page's table row say of an answer's row, so a part
/// one door adds the other has (#296). Each door destructures it whole,
/// and a part it leaves out is named there with why.
pub struct Line {
    /// The name and the type line as the game shows them together.
    pub name: String,
    /// The rarity in words (`rare`), where the item has one: currency
    /// has none.
    pub rarity: Option<String>,
    /// The frame in words, where the row carries the item (`--fields
    /// item`), as `show`'s title says it after the rarity.
    pub frame: Option<String>,
    pub base: Option<String>,
    /// The price in the card's words, where the row carries the item.
    pub price: Option<String>,
    /// Where it lies ([`place_words`]), without how old that is.
    pub place: String,
    /// When it left, as an age, where it was removed.
    pub removed: Option<String>,
    /// How long ago its place was fetched, as an age; none where never.
    pub fetched: Option<String>,
    /// Each value's cell ([`value_cell`]), in `view.rows.values`' order:
    /// empty where the item lacks it ([`lacks`]).
    pub values: Vec<String>,
}

impl Line {
    pub fn of(row: &crate::answer::Row, now: i64) -> Line {
        Line {
            name: title(row.name.as_deref(), row.typeline.as_deref()),
            rarity: row.rarity.as_deref().map(str::to_lowercase),
            frame: row
                .shown
                .as_ref()
                .and_then(|shown| shown.item.frame.as_deref())
                .map(str::to_lowercase),
            base: row.base.clone(),
            price: row
                .shown
                .as_ref()
                .map(|shown| price_words(&row.id, &shown.price)),
            place: place_words(&row.place),
            removed: row.removed_at.map(|at| ago(now, Some(at))),
            fetched: row.place.fetched_at.map(|at| ago(now, Some(at))),
            values: row
                .values
                .iter()
                .map(|valued| match lacks(valued) {
                    true => String::new(),
                    false => value_cell(valued),
                })
                .collect(),
        }
    }
}

/// One value of a row in words, as the table's cell under the value's
/// head says it and [`row_shows`] says it after the value's name: the
/// number alone; its status before it where one stands beside it (C94,
/// an incomplete subtotal is marked: `incomplete at 40`), never a bare
/// number where a status stands; the status alone where there is no
/// number. None of the value's evidence, which a card shows.
pub fn value_cell(valued: &crate::answer::Valued) -> String {
    match (&valued.value, valued.status) {
        (Some(value), None) => number(value),
        (Some(value), Some(status)) => format!("{status} at {}", number(value)),
        (None, status) => status.unwrap_or("no value").to_string(),
    }
}

/// The item has nothing the value reads: no number, and no occurrence
/// that satisfies it. Unsaid where a row shows its values, an empty cell
/// in the table; a sort's says the row stands last.
pub fn lacks(valued: &crate::answer::Valued) -> bool {
    valued.value.is_none() && valued.status == Some("no satisfying occurrence")
}

/// One piece of what a row shows: a line it shows, or a value's count of
/// what it left out, the sort's or another's, which each surface follows
/// with its own way to the item whole.
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum RowPiece {
    Seen(String),
    MoreSorted(usize),
    /// A value's lines past those shown, and the value's text.
    MoreOfValue(usize, String),
}

/// What a row shows of its values and its matched terms, each line once
/// for each occurrence the part showing it most shows, and, for each term
/// that touched more than it shows, how many more and the term's path —
/// the words a surface follows with its way to the item whole (the text's
/// `acq show` line, the page's link to the item). A row that carries the
/// item whole says no "more": every line stands in its item.
pub fn row_shows(a: &Answer, row: &crate::answer::Row) -> (Vec<RowPiece>, Vec<(usize, String)>) {
    // each part of the row — the sort, each term — shows a set of the
    // item's occurrences, so a line printed twice by one part is two
    // occurrences: the row prints each line as many times as the part
    // that shows it most, in the order the parts show it, whichever
    // order the terms were written in (else a total of 40 shows
    // contributors of 10 and 20 alone)
    let mut parts: Vec<Vec<RowPiece>> = Vec::new();
    // each value as it is said bare, `name number`: a term's own
    // evidence of the same number is said once, by the value
    let mut said_values: std::collections::HashSet<String> = std::collections::HashSet::new();
    let (texts, sort) = match &a.view {
        ViewOut::Rows(rows) => (rows.values.as_slice(), rows.sort.as_deref()),
        ViewOut::Counts(_) | ViewOut::Cross(_) | ViewOut::Explain(_) => (&[][..], None),
    };
    for (valued, key) in row.values.iter().zip(texts) {
        // what each value is: a field's name before its number, a
        // line or a sum's contributors after — and the value's own text
        // before its number where no field names it, since `sorts by 78`
        // beside a term's sum of 70 reads as the filter's number; the
        // sort's says it sorts, and a row that sorts last says by what
        // (`sorts last: incomplete` names no field)
        let sorts = sort == Some(key.as_str());
        // a column the item has nothing in is an empty cell, unsaid; the
        // sort's says the row is last, and why
        if !sorts && lacks(valued) {
            continue;
        }
        let cell = value_cell(valued);
        let by = match (valued.value.is_some() && valued.status.is_none(), sorts) {
            (true, _) => format!("by {cell}"),
            (false, true) => format!("last: {cell} ({key})"),
            (false, false) => format!("{key}: {cell}"),
        };
        let mut rest = valued.shows.iter();
        let named = match valued.shows.first() {
            Some(crate::answer::Evidence::Value { name, .. }) => {
                rest.next();
                by.replacen("by ", &format!("by {name} "), 1)
            }
            _ => by.replacen("by ", &format!("by {key} "), 1),
        };
        let bare = named.strip_prefix("by ").unwrap_or(&named).to_string();
        let said = match sorts {
            true => format!("sorts {named}"),
            false => bare.clone(),
        };
        said_values.insert(bare);
        let mut part = vec![RowPiece::Seen(said)];
        part.extend(rest.map(|e| RowPiece::Seen(evidence_text(e))));
        if valued.left_out > 0 {
            part.push(match sorts {
                true => RowPiece::MoreSorted(valued.left_out),
                false => RowPiece::MoreOfValue(valued.left_out, key.clone()),
            });
        }
        // the sort's part first, wherever its value stands in the list
        match sorts {
            true => parts.insert(0, part),
            false => parts.push(part),
        }
    }
    for touched in &row.matched {
        parts.push(
            touched
                .shows
                .iter()
                .map(evidence_text)
                .filter(|seen| !said_values.contains(seen))
                .map(RowPiece::Seen)
                .collect(),
        );
    }
    let mut need: std::collections::HashMap<&RowPiece, usize> = std::collections::HashMap::new();
    for part in &parts {
        let mut in_part: std::collections::HashMap<&RowPiece, usize> =
            std::collections::HashMap::new();
        for s in part {
            *in_part.entry(s).or_default() += 1;
        }
        for (s, n) in in_part {
            let most = need.entry(s).or_default();
            *most = (*most).max(n);
        }
    }
    let mut printed: std::collections::HashMap<&RowPiece, usize> = std::collections::HashMap::new();
    let mut shows: Vec<RowPiece> = Vec::new();
    for s in parts.iter().flatten() {
        let done = printed.entry(s).or_default();
        if *done < need[s] {
            *done += 1;
            shows.push(s.clone());
        }
    }
    // where the row carries the item whole, nothing it shows points to
    // the item for more: the item stands beside it
    if row.shown.is_some() {
        shows.retain(|piece| matches!(piece, RowPiece::Seen(_)));
        return (shows, Vec::new());
    }
    let more = row
        .matched
        .iter()
        .filter(|t| t.left_out > 0)
        .map(|touched| (touched.left_out, touched.path.clone()))
        .collect();
    (shows, more)
}

/// The counts a page shows with no link, and why, plainly ("carries those
/// routes only where every route is asked" is not understood).
pub fn unrouted_words(n: usize) -> String {
    format!(
        "{} no link: a count's route is listed only when every route is asked for, which a page never does",
        if n == 1 {
            "1 count has".to_string()
        } else {
            format!("{n} counts have")
        }
    )
}

/// The sort's lines a row leaves out.
pub fn more_sorted_words(n: usize) -> String {
    format!("{n} more sorted")
}

/// A value's lines a row leaves out.
pub fn more_of_value_words(n: usize, key: &str) -> String {
    format!("{n} more of {key}")
}

/// A term's lines a row leaves out.
pub fn more_of_term_words(n: usize, path: &str) -> String {
    format!("{n} more of term {path}")
}

/// Which rows an answer shows of its matches; where a limit asked was
/// lowered to the item rows' bound, so; and, where no sort was asked,
/// the order they stand in.
pub fn rows_shown_words(a: &Answer, rows: &crate::answer::RowsOut) -> String {
    let shown = if rows.returned == 0 {
        "none".to_string()
    } else {
        format!("rows {}–{}", rows.after + 1, rows.after + rows.returned)
    };
    let capped = capped_words(rows)
        .map(|words| format!(" ({words})"))
        .unwrap_or_default();
    let order = order_words(rows)
        .filter(|_| rows.returned > 0)
        .map(|words| format!(", {words}"))
        .unwrap_or_default();
    let values = values_left_out_words(rows)
        .map(|words| format!("; {words}"))
        .unwrap_or_default();
    format!(
        "{shown} of {} shown{capped}{order}{values}",
        a.total.matched
    )
}

/// The order rows stand in where no value orders them: as they lie in
/// their place (`answer.rs`, "Rows"), said so a reader does not look for
/// one. None where a sort was named, which each row says it sorts by.
pub fn order_words(rows: &crate::answer::RowsOut) -> Option<String> {
    match rows.sort.as_deref() {
        Some(sort) if sort == crate::answer::PLACE_ORDER => {
            Some(format!("{PLACE_ORDER_WORDS}, as asked"))
        }
        Some(_) => None,
        None => Some(format!("{PLACE_ORDER_WORDS} (no sort asked)")),
    }
}

/// The values the query compares past the most a row carries, where
/// there are any: counted, never dropped unsaid.
pub fn values_left_out_words(rows: &crate::answer::RowsOut) -> Option<String> {
    let n = rows.values_left_out;
    (n > 0).then(|| {
        format!(
            "{n} more value{} the query compares {} carried: a row carries at most {}, the sort among them",
            if n == 1 { "" } else { "s" },
            if n == 1 { "is not" } else { "are not" },
            crate::answer::VALUES
        )
    })
}

/// C53 (an answer says the limit it applied): where the limit asked was
/// more than the item rows' bound, the bound and what was asked, in
/// words; none where the limit asked stands.
pub fn capped_words(rows: &crate::answer::RowsOut) -> Option<String> {
    rows.limit_asked.map(|asked| {
        format!(
            "{} is the most rows an answer holds where each carries the item; {asked} asked",
            rows.limit
        )
    })
}

/// Every count an answer carries, each with the words that name it
/// ("term 1 undecided", "clause stats.0.0 read the other way, drops"):
/// those a next step hangs on first — the undecided, the together, a
/// stake, what a count could not place — and the rest after. The one
/// enumeration both renderers read: the text prints the routes among
/// them, and both say how many carry none ([`Counted::unrouted`]).
pub struct Counted<'a> {
    pub first: Vec<(String, &'a Count)>,
    pub rest: Vec<(String, &'a Count)>,
}

impl Counted<'_> {
    /// The counts that are not zero and carry no route: an answer carries
    /// those routes only where every route is asked.
    pub fn unrouted(&self) -> usize {
        self.first
            .iter()
            .chain(&self.rest)
            .filter(|(_, count)| count.count > 0 && count.route.is_none())
            .count()
    }
}

/// The counts `a` carries ([`Counted`]).
pub fn counted(a: &Answer) -> Counted<'_> {
    let typed = a.set_aside.as_ref().unwrap_or(&a.query);
    let mut first: Vec<(String, &Count)> = Vec::new();
    let mut rest: Vec<(String, &Count)> = Vec::new();
    // what an ambiguous clause's other reading would change (C119)
    let clauses = typed.trade.iter().flat_map(|t| &t.clauses);
    for (c, o) in clauses.filter_map(|c| c.other.as_ref().map(|o| (c, o))) {
        for (what, count) in [("adds", &o.adds), ("drops", &o.drops)] {
            first.push((format!("clause {} read the other way, {what}", c.at), count));
        }
    }
    first.push(("undecided at the root".to_string(), &a.total.undecided));
    for term in &a.terms {
        first.push((format!("term {} undecided", term.path), &term.undecided));
        if let Some(together) = &term.together {
            first.push((format!("term {} only together", term.path), &together.count));
        }
    }
    for term in &a.terms {
        let matched = (format!("term {} matched", term.path), &term.matched);
        if a.total.matched == 0 {
            first.push(matched)
        } else {
            rest.push(matched)
        }
        rest.push((format!("term {} failed", term.path), &term.failed));
        rest.push((format!("term {} lacked", term.path), &term.lacked));
    }
    let mut buckets: Vec<(String, &Count)> = Vec::new();
    match &a.view {
        ViewOut::Rows(_) => {}
        ViewOut::Explain(explain) => {
            for (label, forced) in [
                ("forced true", &explain.forced_true),
                ("forced false", &explain.forced_false),
            ] {
                first.push((format!("{label}, matched"), &forced.matched));
                first.push((format!("{label}, undecided"), &forced.undecided));
            }
        }
        ViewOut::Counts(counts) => {
            for table in &counts.tables {
                for bucket in &table.buckets {
                    let what = format!(
                        "{} {}",
                        table.key,
                        label_text(&bucket.label, a.scope.realm == Realm::All)
                    );
                    for kind in bucket.sources.iter().chain(&bucket.flags) {
                        buckets.push((format!("{what}, {}", kind.kind), &kind.count));
                    }
                    buckets.push((what, &bucket.count));
                }
            }
        }
        ViewOut::Cross(cross) => {
            for cell in &cross.cells {
                let of: Vec<String> = cell
                    .of
                    .iter()
                    .map(|l| label_text(l, a.scope.realm == Realm::All))
                    .collect();
                buckets.push((of.join(" × "), &cell.count));
            }
            for margin in &cross.margins {
                first.push((format!("{} (none)", margin.key), &margin.none));
                first.push((format!("{} (undecided)", margin.key), &margin.undecided));
            }
        }
    }
    // what a count could not place is what a next step hangs on
    for (what, count) in buckets {
        if what.ends_with("(none)") || what.ends_with("(undecided)") {
            first.push((what, count));
        } else {
            rest.push((what, count));
        }
    }
    Counted { first, rest }
}

/// An explained node's root counts, as given or forced.
pub fn forced_words(matched: usize, undecided: usize) -> String {
    let mut out = plural(matched, "match", "matches");
    if undecided > 0 {
        out.push_str(&format!(" · {}", undecided_words(undecided)));
    }
    out
}

/// A selector nothing in scope carries, and why.
pub fn nothing_words(zero: &crate::answer::Zero, nothing: &crate::answer::Nothing) -> String {
    format!(
        "nothing in scope carries the {} of term {}: {} — {}",
        nothing.of,
        nothing.path,
        nothing.term,
        match nothing.note {
            Some(note) => note,
            // an id that left: its term has said so
            None if nothing.of == "id" => "it left the scope, as its term says",
            None => zero.said,
        }
    )
}

/// What a zero answer's offers to explain are, said once above them.
pub const EXPLAIN_OFFERS: &str =
    "each part of the query forced true and forced false, the part fewest items match first";
/// What a zero answer's offer of the corpus context is.
pub const CONTEXT_OFFER: &str = "every item in scope, this query set aside";

/// The parts a zero answer's offers leave out.
pub fn more_parts_words(n: usize) -> String {
    format!("{} more", plural(n, "part", "parts"))
}

/// An undecided reason's example item by its name.
pub fn example_name(example: &crate::answer::Example) -> &str {
    example.name.as_deref().unwrap_or("(no name)")
}

/// What an undecided reason's problem is, and what may help.
pub fn reason_words(reason: &crate::answer::Reason) -> String {
    format!("{}; {}", reason.problem, reason.hint)
}

/// The undecided reasons left out, and where the route that lists every
/// item with its reasons stands on the surface: `below`, the text's
/// routes block; `above`, the page's total line.
pub fn reasons_left_out_words(n: usize, at: &str) -> String {
    format!(
        "{} more: the undecided route {at} lists every item with its reasons",
        plural(n, "reason", "reasons")
    )
}

/// What the terms block counts over.
pub fn terms_words(scope: &crate::answer::ScopeOut) -> String {
    format!(
        "each term counted alone over all {} {} in all leagues, never over the matches",
        match scope.membership {
            Membership::Live => "live",
            Membership::All => "live and removed",
            Membership::Removed => "removed",
        },
        match &scope.realm {
            Realm::All => "items of every realm".to_string(),
            Realm::One(realm) => format!("{realm} items"),
        }
    )
}

/// A term's only-together count with its bound.
pub fn together_words(together: &crate::answer::Together) -> String {
    format!(
        "{} {} {} only together",
        together.count.count,
        if together.count.count == 1 {
            "reaches"
        } else {
            "reach"
        },
        number(&together.bound)
    )
}

/// An `id:` term's id that left the scope, and when.
pub fn removed_words(removed: &crate::answer::Removed, now: i64) -> String {
    format!(
        "this id is {} {} that left the scope {}",
        if removed.what == "item" { "an" } else { "a" },
        removed.what,
        ago(now, Some(removed.removed_at)),
    )
}

/// What a retired place that left matching nothing held.
pub const REMOVED_HELD_NONE: &str = "and held no item";

/// One value a value list carries, with its items.
pub fn carried_words(v: &crate::answer::Carried) -> String {
    format!("{} ({})", v.value.replace('\n', "\\n"), v.items)
}

/// What a list leaves out.
pub fn more_words(n: usize) -> String {
    format!("{n} more")
}

fn json_text(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(text) => text.replace('\n', "\\n"),
        other => other.to_string(),
    }
}

/// A bucket's name: its value, or `(none)`, `(undecided)` — in brackets,
/// since a tab may be named `none`; a tab's league and id beside its
/// name, and its realm where the scope spans realms.
pub fn label_text(label: &Label, all_realms: bool) -> String {
    let mut out = match (label.bucket, &label.value) {
        ("value", Some(value)) => json_text(value),
        ("value", None) => "(no name)".to_string(),
        ("computed", Some(value)) => format!("{} (computed)", json_text(value)),
        (bucket, _) => format!("({bucket})"),
    };
    if let Some(realm) = label.realm.as_ref().filter(|_| all_realms) {
        out.push_str(&format!(" · {realm}"));
    }
    if let Some(id) = &label.id {
        out.push_str(&format!(
            " · {} · {id}",
            label.league.as_deref().unwrap_or("no league")
        ));
    }
    out
}

/// A sum says what it covers (C146, a count says what it counts): with
/// items lacking the value, the sum is over the items that have it and
/// the rest are counted beside it, where `sum 0 · 3 lacking` read as a
/// measured zero over every item.
pub fn summed_text(sum: &Summed) -> String {
    let mut out = number(&sum.value);
    if sum.lacking > 0 {
        out.push_str(&format!(
            " over the items that have it · {} {} it, adding nothing",
            sum.lacking,
            if sum.lacking == 1 { "lacks" } else { "lack" }
        ));
    }
    if sum.incomplete {
        out.push_str(&format!(" · incomplete: {} unread", sum.unread));
    }
    out
}

pub fn sum_of_text(sum: &SumOf) -> String {
    format!("{} over every match: {}", sum.name, summed_text(&sum.total))
}

/// A bucket's or a cell's sum.
pub fn sum_words(sum: &Summed) -> String {
    format!("sum {}", summed_text(sum))
}

/// A vocabulary row's kind, with its count.
pub fn kind_words(kind: &crate::counts::OfKind) -> String {
    format!("{} {}", kind.kind, kind.count.count)
}

/// What a kind or a value with no route needs, said beside it.
pub fn needs_words(what: &str, needs: &str) -> String {
    format!("{what}: {needs}")
}

/// A crossed table's items with no value of a key.
pub fn none_words(n: usize) -> String {
    format!("{n} none")
}

/// A crossed table's items it could not place, by what was unread.
pub fn cross_tally_words(tally: &[crate::counts::Tallied]) -> Option<String> {
    tally_words(tally).map(|tally| format!("undecided: {tally}"))
}

/// A zero answer's suggestion: the term that selects a value, `gap`, and
/// the items that carry it.
pub fn suggestion_words(s: &crate::answer::Suggestion, gap: &str) -> String {
    format!("{}{gap}({})", s.term, plural(s.items, "item", "items"))
}

/// The node an explain forces, by its path.
pub fn explain_words(explain: &crate::answer::ExplainOut) -> String {
    format!("{}  {}", explain.path, explain.node)
}

/// A bucket's slots, each its range.
pub fn slots_words(bucket: &Bucket) -> Option<String> {
    let slots: Vec<String> = bucket
        .slots
        .iter()
        .map(|s| {
            let end = |n: &Option<serde_json::Value>| n.as_ref().map_or("?".to_string(), number);
            format!(
                "{} {}..{}{}",
                s.slot,
                end(&s.min),
                end(&s.max),
                if s.incomplete { " (incomplete)" } else { "" }
            )
        })
        .collect();
    (!slots.is_empty()).then(|| slots.join(" · "))
}

/// What a count could not read, by kind, with its items.
pub fn tally_words(tally: &[crate::counts::Tallied]) -> Option<String> {
    let tally: Vec<String> = tally
        .iter()
        .map(|t| format!("{} unread {}", t.unread, t.items))
        .collect();
    (!tally.is_empty()).then(|| tally.join(" · "))
}

/// A count table's heading: its key, the values it lists — every
/// special bucket among them — and, where values are left out, how many
/// are shown, followed by `rest`, the surface's words for how to see the
/// rest.
pub fn table_head_words(table: &Table, rest: &str) -> String {
    let (one, many) = if table.key.starts_with("line") {
        ("template", "templates")
    } else {
        ("value", "values")
    };
    // the heading counts the lines it prints, every special bucket among
    // them: a `none` and an `undecided` line are values (C146; else
    // `4 values` heads five lines);
    // `values` keeps its count of the values the items carry
    let special = table
        .buckets
        .iter()
        .filter(|b| matches!(b.label.bucket, "none" | "undecided"))
        .count();
    let mut head = format!(
        "{} · {}",
        table.key,
        plural(table.values + special, one, many)
    );
    if table.left_out > 0 {
        head.push_str(&format!(
            " · {} shown{rest}",
            table.values - table.left_out + special
        ));
    }
    if table.computed > 0 {
        head.push_str(&format!(
            " · {} computed",
            plural(table.computed, "value", "values")
        ));
        if table.computed_left_out > 0 {
            head.push_str(&format!(
                " · {} shown{rest}",
                table.computed - table.computed_left_out
            ));
        }
    }
    head
}

pub fn unmatched_words(leagues: &[String]) -> Option<String> {
    (!leagues.is_empty()).then(|| {
        format!(
            "the scope's leagues no match is in have no bucket: {}",
            leagues.join(", ")
        )
    })
}

/// A crossed table's heading, as [`table_head_words`] says a table's.
pub fn cross_head_words(cross: &CrossOut, rest: &str) -> String {
    let mut head = format!(
        "{} · {}",
        cross.keys.join(" × "),
        plural(cross.cells_in_all, "cell", "cells")
    );
    if cross.left_out > 0 {
        head.push_str(&format!(
            " · {} shown{rest}",
            cross.cells_in_all - cross.left_out
        ));
    }
    head
}

/// A block of `describe`'s document, as a renderer prints it under its
/// head ([`BLOCKS`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Fields,
    Line,
    Linked,
    Values,
    Composition,
    Operators,
    Slots,
    Computed,
    Counts,
    Explain,
    Answer,
    Totals,
    Categories,
    Trade,
    NotBuilt,
    Limits,
}

/// One block of `describe`'s document: its part, the name that asks for
/// it whole (the document's key, `crate::describe::SECTIONS`)
/// and the head a renderer prints over it.
pub struct Block {
    pub part: Part,
    pub name: &'static str,
    pub head: &'static str,
}

/// The blocks' heads and their order, the one home both renderers read
/// (`describe_text` and the page's, `4-full-1-form-index.md`, Decision 3):
/// words that name no flag, a terminal adding its own after a head where
/// it spells one. A head that carries a fact of the document (the totals
/// table's version, the catalogue's) is followed by it, in each renderer.
pub const BLOCKS: [Block; 16] = [
    Block {
        part: Part::Fields,
        name: "fields",
        head: "fields",
    },
    Block {
        part: Part::Line,
        name: "line",
        head: "inside line( … )",
    },
    Block {
        part: Part::Linked,
        name: "linked",
        head: "inside linked( … )",
    },
    Block {
        part: Part::Values,
        name: "values",
        head: "values",
    },
    Block {
        part: Part::Composition,
        name: "composition",
        head: "composition",
    },
    Block {
        part: Part::Operators,
        name: "operators",
        head: "operators",
    },
    Block {
        part: Part::Slots,
        name: "slots",
        head: "slots",
    },
    Block {
        part: Part::Computed,
        name: "computed",
        head: "computed values",
    },
    Block {
        part: Part::Counts,
        name: "counts",
        head: "counts",
    },
    Block {
        part: Part::Explain,
        name: "explain",
        head: "why an answer is what it is",
    },
    Block {
        part: Part::Answer,
        name: "answer",
        head: "an answer, key by key",
    },
    Block {
        part: Part::Totals,
        name: "totals",
        head: "totals",
    },
    Block {
        part: Part::Categories,
        name: "categories",
        head: "categories — the trade site's ids, as spelled",
    },
    Block {
        part: Part::Trade,
        name: "trade",
        head: "trade",
    },
    Block {
        part: Part::NotBuilt,
        name: "not_built",
        head: "not built, refused by name",
    },
    Block {
        part: Part::Limits,
        name: "limits",
        head: "limits, as the search states them",
    },
];
/// What a pattern of a list matched, as its answer opens: the pattern,
/// how many entries, whole or at one line.
pub fn pattern_said(m: &crate::describe::Matched) -> String {
    format!(
        "pattern {} — {}, {}",
        m.pattern,
        plural(m.matched, "entry", "entries"),
        match m.whole {
            true => "each whole",
            false => "one line each",
        }
    )
}

/// What the trade catalogue's block holds, after its head where it is
/// whole: its version, the stats table's, and what it lists.
pub fn trade_said(trade: &crate::describe::TradeCatalogue) -> String {
    format!(
        "the catalogue v{}, the site's stats of {}: what an answer reports of a translation, then each clause a site search can carry, its status when it runs, and the text it becomes",
        trade.version, trade.stats
    )
}

/// The five parts `show` carries of an item, borrowed from `show`'s
/// document or from a row that carries the item ([`ShownItem`]): what
/// the item's block of text and the page's card are made from, so a row's
/// item and `show`'s are said by the same calls.
#[derive(Clone, Copy)]
pub struct ItemParts<'a> {
    pub item: &'a crate::derive::Item,
    pub class: &'a crate::Classed,
    pub category: &'a crate::category::Shown,
    pub price: &'a Priced,
    pub lines: &'a [ShownLine],
    /// Its displayed rows in their pieces, each number field's with its
    /// text: what a card links.
    pub displays: crate::show::Displays<'a>,
}

impl<'a> ItemParts<'a> {
    pub fn of_shown(s: &'a Shown) -> ItemParts<'a> {
        ItemParts {
            item: &s.item,
            class: &s.class,
            category: &s.category,
            price: &s.price,
            lines: &s.lines,
            displays: s.displays(),
        }
    }

    pub fn of_row(s: &'a ShownItem) -> ItemParts<'a> {
        ItemParts {
            item: &s.item,
            class: &s.class,
            category: &s.category,
            price: &s.price,
            lines: &s.lines,
            displays: s.displays(),
        }
    }
}

/// An item's title as `show` heads it: its name and type, its rarity and
/// its frame.
pub fn shown_title(item: &crate::derive::Item) -> String {
    let mut head = title(item.name.as_deref(), item.typeline.as_deref());
    for (label, value) in [("", &item.rarity), ("frame ", &item.frame)] {
        if let Some(value) = value {
            head.push_str(&format!(" · {label}{}", value.to_lowercase()));
        }
    }
    head
}

/// When a removed item left, and what is shown of it.
pub fn shown_removed_words(now: i64, removed_at: i64) -> String {
    format!(
        "{}: no longer in the stash, shown as it was last seen",
        ago(now, Some(removed_at))
    )
}

/// What the class table says (C106): the class, or why there is none.
pub fn class_words(class: &crate::Classed) -> String {
    use crate::Classed;
    match class {
        Classed::Is(class) => class.to_string(),
        Classed::All(classes) => format!(
            "{} (each: the table lists the base under both)",
            classes.join(", ")
        ),
        Classed::Among { why, .. } | Classed::Open(why) => format!("undecided: {}", why.problem),
        Classed::BaseUnread => "undecided: the base was not read".to_string(),
        Classed::FrameUnread { .. } => {
            "undecided: the frame, which decides an invitation's class, was not read".to_string()
        }
        Classed::BestiaryUnread { candidates, .. } => format!(
            "undecided: the bestiary line, which tells a captured beast, was not read — {}",
            candidates.join(" or ")
        ),
    }
}

/// The category (C106): the id or ids a count lists the item under,
/// none, or undecided.
pub fn category_words(category: &crate::category::Shown) -> String {
    match category.bucket {
        "value" => category.listed.join(", "),
        other => other.to_string(),
    }
}

/// A category's ids left open, and why.
pub fn open_words(open: &crate::category::Open) -> String {
    let ids = if open.ids.len() <= 4 {
        open.ids.join(", ")
    } else {
        plural(open.ids.len(), "id", "ids")
    };
    format!("open {ids}: {}", open.why.problem)
}

/// Where a shown item lies: its realm, then its place.
pub fn shown_place_words(place: &crate::corpus::Place, now: i64) -> String {
    format!("{} · {}", place.realm, place_text(place, now))
}

/// What the listing state prices the item at (C81): the price, its side,
/// and the row it is priced from where that is not the item's own; or none
/// with the reason, in the words the JSON carries (`none (none)` reads as
/// two empty fields); or undecided with the problem.
pub fn price_words(id: &str, price: &Priced) -> String {
    let target = format!("item/{id}");
    match price {
        Priced::Is(p) => format!(
            "{} · {}{}",
            p.text,
            p.side,
            p.from
                .as_deref()
                .filter(|from| *from != target)
                .map(|from| format!(" · from {from}"))
                .unwrap_or_default()
        ),
        Priced::None { why: reason, .. } => format!("none ({reason})"),
        Priced::Open(reason) => format!("undecided: {}", reason.problem),
    }
}

/// What the item is (`identified`, `corrupted`, …), its flags as the
/// body names them, none where it names none.
pub fn flags_words(item: &crate::derive::Item) -> Option<String> {
    (!item.flags.is_empty()).then(|| item.flags.join(", "))
}

/// The sockets as the game shows them, and the largest link group.
pub fn sockets_words(item: &crate::derive::Item) -> Option<String> {
    let groups = crate::sockets::groups(item)?;
    let layout = groups.layout();
    let links = match groups.links() {
        crate::sockets::Counted::Absent => "no link group".to_string(),
        counted => match (counted.value(), counted.floor()) {
            (Some(n), _) => format!("links {n}"),
            (None, Some(floor)) => format!("links at least {floor}, not established"),
            (None, None) => "links not established".to_string(),
        },
    };
    Some(format!(
        "{} · {links}",
        if layout.is_empty() { "none" } else { &layout }
    ))
}

/// A line's source and flags.
pub fn line_kind_words(l: &crate::show::ShownLine) -> String {
    std::iter::once(l.line.source.as_str())
        .chain(l.line.flags.iter().map(String::as_str))
        .collect::<Vec<_>>()
        .join(", ")
}

/// A line's template, quoted.
pub fn line_template_quoted(l: &crate::show::ShownLine) -> String {
    format!("\"{}\"", l.line.template.replace('\n', "\\n"))
}

/// A line's slots, each its word and its number.
pub fn line_slots_words(l: &crate::show::ShownLine) -> Option<String> {
    let slots: Vec<String> = l
        .slots
        .iter()
        .map(|(word, n)| format!("{word} {}", number(n)))
        .collect();
    (!slots.is_empty()).then(|| slots.join(" · "))
}

/// A row of a mod of several, quoted: each row is a name of it.
pub fn line_row_quoted(row: &crate::derive::Row) -> String {
    format!("row \"{}\"", row.template)
}

/// A row's own numbers.
pub fn line_row_slots_words(row: &crate::derive::Row) -> Option<String> {
    let slots: Vec<String> = row
        .read()
        .slots()
        .into_iter()
        .map(|(word, n)| format!("{word} {}", slot_number(n)))
        .collect();
    (!slots.is_empty()).then(|| slots.join(" · "))
}

/// What an item with nothing unread says of it.
pub const UNREAD_NOTHING: &str = "nothing";

/// An outcome against a query, as words.
pub fn outcome_words(outcome: &str) -> String {
    outcome.replace('_', " ")
}

/// What a node of `show --against` touched.
pub fn node_shows_words(node: &crate::show::AgainstNode) -> String {
    node.shows
        .iter()
        .map(evidence_text)
        .collect::<Vec<_>>()
        .join(" · ")
}

/// What a node touched past what it shows.
pub fn node_left_out_words(n: usize) -> String {
    format!("{n} more: the item whole is above")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The heads' one table names each block of the document once, by the
    /// key the search crate serialises it under and asks it by: a block the
    /// search crate adds has no head until it is given one here.
    #[test]
    fn the_heads_table_names_every_block_of_the_document_once() {
        let mut named: Vec<&str> = BLOCKS.iter().map(|b| b.name).collect();
        let mut sections = crate::describe::SECTIONS.to_vec();
        named.sort_unstable();
        sections.sort_unstable();
        assert_eq!(named, sections);
    }

    /// C53 (an answer says the limit it applied): where a limit asked
    /// past the item rows' bound was lowered, the words name the bound and
    /// what was asked, beside the rows shown; none where the limit stood.
    /// The bound is too many rows for a golden's world to reach, so the
    /// words are held here.
    #[test]
    fn a_limit_lowered_to_the_item_rows_bound_is_said() {
        let rows = |limit_asked| crate::answer::RowsOut {
            sort: None,
            desc: false,
            values: Vec::new(),
            values_from_query: false,
            values_left_out: 0,
            fields: vec![crate::answer::RowField::Item],
            limit: acquisition_store::bound::ITEM_ROWS,
            limit_asked,
            returned: 200,
            after: 0,
            left_out: 300,
            next: None,
        };
        assert_eq!(
            capped_words(&rows(Some(500))).as_deref(),
            Some("200 is the most rows an answer holds where each carries the item; 500 asked")
        );
        assert_eq!(capped_words(&rows(None)), None);
    }

    /// C146 — a sum with items lacking the value says it is over the
    /// items that have it and counts the rest beside it, where `sum 0 ·
    /// 3 lacking` read as a measured zero.
    #[test]
    fn c146_a_sum_says_it_covers_the_items_that_have_the_value() {
        let sum = |lacking| Summed {
            value: serde_json::json!(0),
            lacking,
            incomplete: false,
            unread: 0,
        };
        assert_eq!(summed_text(&sum(0)), "0");
        assert_eq!(
            summed_text(&sum(3)),
            "0 over the items that have it · 3 lack it, adding nothing"
        );
        assert_eq!(
            summed_text(&sum(1)),
            "0 over the items that have it · 1 lacks it, adding nothing"
        );
    }
}
