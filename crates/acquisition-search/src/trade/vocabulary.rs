//! The stat vocabulary a form's picker lists: the totals the site names
//! as pseudo stats, then the stat table's entries, those that hold every
//! word asked, each with what the translation makes of it alone and
//! with a bound — so a page that offers a stat says its fate before it
//! is chosen, and never reads the captured JSON. Beside it, the values a
//! column can be added by, found by the same words.
//!
//! # As built
//!
//! - **An entry is a stat as a query names it**: `<category>.<id>`, its
//!   category (the group a picker lists it under) and its texts.
//! - **The groups come in the trade site's order, pseudo first.** The
//!   pseudo group is the totals table's totals and readings, each by the
//!   site's id and text, in that table's order; the stat table carries
//!   none of them. The rest are the stat table's, category by category
//!   ([`stats::StatTable::entries`]).
//! - **The words are matched here, not in a page** (C46, shared semantics
//!   in Rust): each word of the ask, case aside, stands in one of the
//!   entry's texts or in its group's name, so a kind's word narrows to
//!   that kind (`pseudo life`, `frac life`); an empty ask matches every
//!   entry. One `~` before the ask is dropped: the site's mark for
//!   words in any order, which is how every ask is read here. The first
//!   `limit` are listed and the rest counted (C53).
//! - **The statuses are the translation's own** ([`translate`]), never a
//!   copy of its rules: the listed entries are put to it as one `and`
//!   group of filters, once with no value and once with a minimum of 1,
//!   as the translation's own test of every stat does, and each filter's
//!   clause status is read back. Only the listed entries are translated,
//!   so the cost is the limit's, never the table's.
//! - **The value listing** ([`values`], the finder's "Add a column") lists
//!   what a column can be added by, by the same words: the number fields
//!   first, each catalogue row the form renders as a number box by its
//!   label, under `filter`, then the stats in the order above. An entry is
//!   listed exactly where [`value_of`] answers a value for it: the value
//!   the translation compares for a bounded filter on that entry alone, of
//!   any kind, held once made (two asks at once may each make it, and
//!   agree: the translation is a pure function of the build's tables). A
//!   line's value (`show.rs`, `trade_value`) is made by the same function, keeping its
//!   own rule: a sum over the stat's lines, whose slot the line reads. No
//!   shape decides what is listed. Only the entries the words match are put
//!   to it; the first `limit` with a value are listed and the rest with one
//!   counted.

use serde::Serialize;
use serde_json::{Value as Json, json};

use crate::error::{ErrorKind, LanguageError};

use crate::totals;

use super::stats;
use super::translate::{Status, translate};
use super::{Decoded, Source};

/// The most entries one listing translates.
pub const MAX_LISTED: usize = 200;

/// The group the totals are listed under: the category the site's query
/// names a pseudo stat by.
const PSEUDO: &str = "pseudo";

/// The entries of the table that hold the words asked.
#[derive(Debug, Clone, Serialize)]
pub struct Vocabulary {
    /// The stat table's version, the capture its texts are from.
    pub stats: String,
    /// The totals table's version, where the pseudo group is from.
    pub totals: u32,
    /// The entries listed, at most the limit asked.
    pub entries: Vec<Entry>,
    /// Entries that matched past the limit.
    pub left_out: usize,
}

/// One stat as a query names it, and what a filter of it becomes.
#[derive(Debug, Clone, Serialize)]
pub struct Entry {
    /// `<category>.<id>`, as the site's query names it.
    pub id: String,
    /// The texts the site displays for it, in the capture's order.
    pub texts: Vec<String>,
    /// Its category, the group a picker lists it under.
    pub group: &'static str,
    /// Its clause's status as a filter with no value.
    pub alone: Status,
    /// Its clause's status as a filter with a minimum.
    pub bounded: Status,
}

/// The entries that hold every word of `ask`, case aside, in a text or
/// in their group's name, a `~` before the ask dropped: the pseudo
/// group first, the first `limit` (at most [`MAX_LISTED`]) with their
/// statuses and the rest counted. An error is a table this build ships
/// that does not load or a translation that broke its own guard: a build
/// defect, said.
pub fn vocabulary(ask: &str, limit: usize) -> Result<Vocabulary, LanguageError> {
    let table = stats::table().map_err(|e| defect(e.to_string()))?;
    let totals = totals::table().map_err(|e| defect(e.to_string()))?;
    let words = words(ask);
    let limit = limit.min(MAX_LISTED);
    let mut listed = Vec::new();
    let mut left_out = 0;
    let named = |site: &'static Option<String>, text: &'static Option<String>| {
        Some((
            PSEUDO,
            site.as_deref()?,
            std::slice::from_ref(text.as_ref()?),
        ))
    };
    let pseudo = totals
        .totals()
        .iter()
        .map(|total| (&total.site, &total.text))
        .chain(totals.readings().iter().map(|r| (&r.site, &r.text)))
        .filter_map(|(site, text)| named(site, text));
    for (group, id, texts) in pseudo.chain(table.entries()) {
        if !holds(&words, group, texts) {
            continue;
        }
        if listed.len() < limit {
            listed.push((group, format!("{group}.{id}"), texts.to_vec()));
        } else {
            left_out += 1;
        }
    }
    let alone = statuses(&listed, &json!({}))?;
    let bounded = statuses(&listed, &json!({ "min": 1 }))?;
    Ok(Vocabulary {
        stats: table.version().to_string(),
        totals: totals.version(),
        entries: listed
            .into_iter()
            .zip(alone.into_iter().zip(bounded))
            .map(|((group, id, texts), (alone, bounded))| Entry {
                id,
                texts,
                group,
                alone,
                bounded,
            })
            .collect(),
        left_out,
    })
}

/// Each id as an entry, in the order given, with the statuses a listing
/// gives it: a filled form's row says its text and fate as a pick's does.
/// An id no table holds, a total's or the stat table's, is none. An error
/// is [`vocabulary`]'s: a build defect, said.
pub fn entries(ids: &[&str]) -> Result<Vec<Option<Entry>>, LanguageError> {
    let table = stats::table().map_err(|e| defect(e.to_string()))?;
    let totals = totals::table().map_err(|e| defect(e.to_string()))?;
    let found: Vec<Option<(&'static str, String, Vec<String>)>> = ids
        .iter()
        .map(|id| {
            if let Some(site) = id.strip_prefix(&format!("{PSEUDO}.")) {
                let text = totals
                    .totals()
                    .iter()
                    .map(|total| (&total.site, &total.text))
                    .chain(totals.readings().iter().map(|r| (&r.site, &r.text)))
                    .find(|(s, _)| s.as_deref() == Some(site))
                    .and_then(|(_, text)| text.clone())?;
                return Some((PSEUDO, id.to_string(), vec![text]));
            }
            let (category, _) = id.split_once('.')?;
            let group = stats::HASH_CATEGORIES.iter().find(|c| **c == category)?;
            Some((*group, id.to_string(), table.texts(id)?.to_vec()))
        })
        .collect();
    let listed: Vec<(&'static str, String, Vec<String>)> =
        found.iter().flatten().cloned().collect();
    let alone = statuses(&listed, &json!({}))?;
    let bounded = statuses(&listed, &json!({ "min": 1 }))?;
    let mut made = listed.into_iter().zip(alone.into_iter().zip(bounded)).map(
        |((group, id, texts), (alone, bounded))| Entry {
            id,
            texts,
            group,
            alone,
            bounded,
        },
    );
    Ok(found
        .iter()
        .map(|f| f.as_ref().and_then(|_| made.next()))
        .collect())
}

/// Each listed entry's clause status as a filter with `value`, read from
/// one translation of them all as an `and` group.
fn statuses(
    listed: &[(&'static str, String, Vec<String>)],
    value: &Json,
) -> Result<Vec<Status>, LanguageError> {
    if listed.is_empty() {
        return Ok(Vec::new());
    }
    let filters: Vec<Json> = listed
        .iter()
        .map(|(_, id, _)| json!({ "id": id, "value": value }))
        .collect();
    let decoded = Decoded {
        source: Source::Json,
        realm: None,
        league: None,
        query: json!({ "stats": [{ "type": "and", "filters": filters }] }),
        defences: Default::default(),
        sort: None,
        beside: serde_json::Map::new(),
    };
    let translation = translate(&decoded)?;
    (0..listed.len())
        .map(|n| {
            let at = format!("stats.0.{n}");
            translation
                .clauses
                .iter()
                .find(|c| c.at == at)
                .map(|c| c.status)
                .ok_or_else(|| defect(format!("the translation reported no clause at `{at}`")))
        })
        .collect()
}

/// An entry a value is asked of: a stat as a query names it
/// (`<category>.<id>`, a pseudo total's among them), or a number field by
/// its catalogue key (`<group>.<field>`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Valued<'a> {
    Stat(&'a str),
    Field(&'a str),
}

impl Valued<'_> {
    /// The site's query holding one filter on the entry, bounded by a
    /// minimum of 1, as the site sends a filled box.
    fn bounded(self) -> Option<Json> {
        Some(match self {
            Valued::Stat(id) => json!({ "stats": [{ "type": "and", "filters": [
                { "id": id, "value": { "min": 1 } } ] }] }),
            Valued::Field(key) => {
                let (group, field) = key.split_once('.')?;
                json!({ "filters": { group: { "filters": { field: { "min": 1 } } } } })
            }
        })
    }

    fn key(self) -> &'static str {
        match self {
            Valued::Stat(_) => "stat",
            Valued::Field(_) => "field",
        }
    }
}

/// The value the translation compares for a bounded filter on one entry,
/// a stat's or a number field's, by its canonical text: the first value
/// (`values.rs`) of the translation of that filter alone, of any kind, a
/// pseudo total's computed value and a field among them. None where the
/// translation compares no value of it: a stat with no number, a clause
/// that does not run, an entry no table holds. What a line's value is
/// made from (`show.rs`, `trade_value`), and what the value listing
/// ([`values`]) lists by.
pub fn value_of(entry: Valued) -> Option<String> {
    compared_of(entry).map(|one| one.text)
}

/// [`value_of`] with the value itself, held once made: two asks at once
/// may each make it, and agree, since the translation is a pure function
/// of the build's tables.
pub(crate) fn compared_of(entry: Valued) -> Option<crate::values::Compared> {
    use std::collections::HashMap;
    use std::sync::{LazyLock, Mutex};
    type Held = HashMap<(&'static str, String), Option<crate::values::Compared>>;
    static ASKED: LazyLock<Mutex<Held>> = LazyLock::new(|| Mutex::new(HashMap::new()));
    let id = match entry {
        Valued::Stat(id) | Valued::Field(id) => id,
    };
    let key = (entry.key(), id.to_string());
    if let Some(held) = ASKED.lock().ok().and_then(|held| held.get(&key).cloned()) {
        return held;
    }
    let compared = entry.bounded().and_then(|query| {
        let decoded = Decoded {
            source: Source::Json,
            realm: None,
            league: None,
            query,
            defences: Default::default(),
            sort: None,
            beside: serde_json::Map::new(),
        };
        translate(&decoded).ok().and_then(|translation| {
            crate::values::compared(&translation.tree)
                .into_iter()
                .next()
        })
    });
    if let Ok(mut held) = ASKED.lock() {
        held.insert(key, compared.clone());
    }
    compared
}

/// The group a number field is listed under: the site's filters, beside
/// its stats.
const FILTER: &str = "filter";

/// The values a column can be added by, those that hold the words asked.
#[derive(Debug, Clone, Serialize)]
pub struct Values {
    /// The stat table's version, the capture its texts are from.
    pub stats: String,
    /// The totals table's version, where the pseudo group is from.
    pub totals: u32,
    /// The catalogue's version, where the number fields are from.
    pub catalogue: u32,
    /// The entries listed, at most the limit asked.
    pub entries: Vec<ValueEntry>,
    /// Entries with a value that matched past the limit.
    pub left_out: usize,
}

/// One entry of the value listing: what names it and the value it reads.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ValueEntry {
    /// The stat as a query names it, or the number field's catalogue key.
    pub id: String,
    /// The site's names for it: a stat's texts, a field's catalogue label.
    pub texts: Vec<String>,
    /// The group a finder lists it under: a stat's category, or `filter`
    /// for a number field.
    pub group: &'static str,
    /// The value it reads ([`value_of`]), the text a sort takes.
    pub value: String,
}

/// Every entry a value is asked of, in the listing's order, by its
/// group, its id and its names: the number fields first, each catalogue
/// row the form renders as a number box (`minmax`, `minmax_option`,
/// `sockets`) by its label, then the stats in the vocabulary's order, the
/// pseudo group first. Made once in a process and held. No shape decides
/// which are listed: [`values`] puts each it matches to [`value_of`].
/// An entry asked: its group, its id, its names.
type Asked = (&'static str, String, Vec<String>);

fn asked() -> Result<&'static [Asked], LanguageError> {
    use std::sync::LazyLock;
    static ASKED: LazyLock<Result<Vec<Asked>, String>> = LazyLock::new(|| {
        let table = stats::table().map_err(|e| e.to_string())?;
        let totals = totals::table().map_err(|e| e.to_string())?;
        let catalogue = super::catalogue::catalogue().map_err(|e| e.to_string())?;
        let mut asked: Vec<Asked> = Vec::new();
        for row in catalogue.rows() {
            let boxed = row
                .shape
                .as_deref()
                .is_some_and(|s| s.starts_with("minmax") || s == "sockets");
            if let (true, Some(text)) = (boxed, &row.text) {
                asked.push((FILTER, row.key.clone(), vec![text.clone()]));
            }
        }
        let pseudo = totals
            .totals()
            .iter()
            .map(|total| (&total.site, &total.text))
            .chain(totals.readings().iter().map(|r| (&r.site, &r.text)))
            .filter_map(|(site, text)| Some((site.as_deref()?, text.clone()?)));
        for (site, text) in pseudo {
            asked.push((PSEUDO, format!("{PSEUDO}.{site}"), vec![text]));
        }
        for (category, id, texts) in table.entries() {
            asked.push((category, format!("{category}.{id}"), texts.to_vec()));
        }
        Ok(asked)
    });
    ASKED.as_deref().map_err(|e| defect(e.clone()))
}

/// The entries a column can be added by that hold every word of `ask`,
/// case aside, in a text or in their group's name, a `~` before the ask
/// dropped (the vocabulary's rule): those [`value_of`] answers a value
/// for, in the listing's order (the number fields, then the stats as
/// the vocabulary lists them), the first `limit` listed and the rest
/// counted. Only the entries the words match are put to it, each value
/// held once made ([`value_of`]). An error is a table this build ships that does not load: a
/// build defect, said.
pub fn values(ask: &str, limit: usize) -> Result<Values, LanguageError> {
    let words = words(ask);
    let mut entries = Vec::new();
    let mut left_out = 0;
    for (group, id, texts) in asked()? {
        if !holds(&words, group, texts) {
            continue;
        }
        let entry = match *group {
            FILTER => Valued::Field(id),
            _ => Valued::Stat(id),
        };
        let Some(value) = value_of(entry) else {
            continue;
        };
        if entries.len() < limit {
            entries.push(ValueEntry {
                id: id.clone(),
                texts: texts.clone(),
                group,
                value,
            });
        } else {
            left_out += 1;
        }
    }
    Ok(Values {
        stats: stats::table()
            .map_err(|e| defect(e.to_string()))?
            .version()
            .to_string(),
        totals: totals::table()
            .map_err(|e| defect(e.to_string()))?
            .version(),
        catalogue: super::catalogue::CATALOGUE_VERSION,
        entries,
        left_out,
    })
}

/// An ask's words, lower case, one `~` before it dropped: the site's mark
/// for words in any order, which is how every ask is read here.
fn words(ask: &str) -> Vec<String> {
    let ask = ask.trim_start();
    let ask = ask.strip_prefix('~').unwrap_or(ask);
    ask.split_whitespace().map(str::to_lowercase).collect()
}

/// Whether every word, case aside, stands in one of the texts or in the
/// group's name.
fn holds(words: &[String], group: &str, texts: &[String]) -> bool {
    words.iter().all(|word| {
        group.contains(word.as_str())
            || texts
                .iter()
                .any(|text| text.to_lowercase().contains(word.as_str()))
    })
}

fn defect(message: String) -> LanguageError {
    LanguageError::new(ErrorKind::Trade, message)
}
