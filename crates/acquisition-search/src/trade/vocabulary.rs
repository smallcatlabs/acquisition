//! The stat vocabulary a form's picker lists (step 4b of the roadmap,
//! `roadmap/briefs/4b-build.md`, Do 3): the stat table's entries whose
//! texts hold every word asked, each with what the translation makes of
//! it alone and with a bound — so a page that offers a stat says its
//! fate before it is chosen, and never reads the captured JSON.
//!
//! # As built
//!
//! - **An entry is a stat as a query names it**: `<category>.<id>`, its
//!   category (the group a picker lists it under) and its texts, in the
//!   table's order ([`stats::StatTable::entries`]).
//! - **The words are matched here, not in a page** (C46, shared semantics
//!   in Rust): each word of the ask, case aside, stands in one of the
//!   entry's texts; an empty ask matches every entry. The first `limit`
//!   are listed and the rest counted (C53).
//! - **The statuses are the translation's own** ([`translate`]), never a
//!   copy of its rules: the listed entries are put to it as one `and`
//!   group of filters, once with no value and once with a minimum of 1,
//!   as the translation's own test of every stat does, and each filter's
//!   clause status is read back. Only the listed entries are translated,
//!   so the cost is the limit's, never the table's.

use serde::Serialize;
use serde_json::{Value as Json, json};

use crate::error::{ErrorKind, LanguageError};

use super::stats;
use super::translate::{Status, translate};
use super::{Decoded, Source};

/// The most entries one listing translates.
pub const MAX_LISTED: usize = 200;

/// The entries of the table that hold the words asked.
#[derive(Debug, Clone, Serialize)]
pub struct Vocabulary {
    /// The stat table's version, the capture its texts are from.
    pub stats: String,
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

/// The table's entries whose texts hold every word of `ask`, case aside,
/// the first `limit` (at most [`MAX_LISTED`]) with their statuses and the
/// rest counted. An error is a table this build ships that does not load
/// or a translation that broke its own guard: a build defect, said.
pub fn vocabulary(ask: &str, limit: usize) -> Result<Vocabulary, LanguageError> {
    let table = stats::table().map_err(|e| defect(e.to_string()))?;
    let words: Vec<String> = ask.split_whitespace().map(str::to_lowercase).collect();
    let limit = limit.min(MAX_LISTED);
    let mut listed = Vec::new();
    let mut left_out = 0;
    for (group, id, texts) in table.entries() {
        let held = words.iter().all(|word| {
            texts
                .iter()
                .any(|text| text.to_lowercase().contains(word.as_str()))
        });
        if !held {
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

fn defect(message: String) -> LanguageError {
    LanguageError::new(ErrorKind::Trade, message)
}
