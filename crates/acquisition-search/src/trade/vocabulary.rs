//! The stat vocabulary a form's picker lists (step 4b of the roadmap,
//! `roadmap/briefs/4b-build.md`, Do 3): the totals the site names as
//! pseudo stats, then the stat table's entries, those that hold every
//! word asked, each with what the translation makes of it alone and
//! with a bound — so a page that offers a stat says its fate before it
//! is chosen, and never reads the captured JSON.
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
    let ask = ask.trim_start();
    let ask = ask.strip_prefix('~').unwrap_or(ask);
    let words: Vec<String> = ask.split_whitespace().map(str::to_lowercase).collect();
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
        let held = words.iter().all(|word| {
            group.contains(word.as_str())
                || texts
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
