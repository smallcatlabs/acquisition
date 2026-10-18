//! A league's tabs as `tabs` gives them on both surfaces (C115, C53):
//! counted first — how many, how many fetched and never, how many items,
//! and the same by tab type — then the tabs listed: every one where the
//! league holds few or the whole list is asked, else the ten with the most
//! items and the rest counted.
//!
//! # As built
//!
//! One maker for both surfaces: `acq tabs` prints its text from this
//! document and `--json` prints it, and the MCP's `tabs` returns it. The
//! whole list stayed the JSON's alone, the counts-first shape the text's,
//! so the MCP's answer for the owner's Standard was 745,672 bytes and no
//! limit, cut to its head in three of the trade seat's runs (mcp-opus
//! call 9, both rounds; mcp-sonnet-2 call 6). The rows are the store's
//! own (`TabRow`), whole; what is left out is the request's `expand`, as
//! a table's rest is its limit.

use acquisition_store::TabRow;
use serde::Serialize;

/// A league of this many tabs or fewer is listed whole. The owner's
/// Standard is 3,347.
pub const LISTED_WHOLE: usize = 30;
/// How many a larger league lists: the most items first.
pub const MOST_ITEMS: usize = 10;

/// A league's tabs, counted and then listed.
#[derive(Debug, Clone, Serialize)]
pub struct Tabs {
    pub realm: String,
    pub league: String,
    /// Every live tab the league's listing names, its substashes among
    /// them.
    pub tabs: usize,
    pub fetched: usize,
    /// Never fetched, folders apart: a folder holds tabs and no items,
    /// and is never fetched.
    pub never_fetched: usize,
    pub folders: usize,
    /// The live items in the league's tabs.
    pub items: i64,
    /// The same counts by GGG's tab type, the most items first.
    pub by_type: Vec<TabType>,
    /// Which tabs `rows` holds: `all`, in the listing's order, or
    /// `most_items`, the ten with the most.
    pub listed: &'static str,
    pub rows: Vec<TabRow>,
    /// Tabs not in `rows`.
    pub left_out: usize,
    /// What lists them: the request's `expand`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_out_needs: Option<&'static str>,
}

/// One tab type's counts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TabType {
    #[serde(rename = "type")]
    pub kind: String,
    pub tabs: usize,
    pub fetched: usize,
    pub items: i64,
}

impl Tabs {
    /// The league's tabs, as the store lists them, counted and listed:
    /// whole where `expand` or where there are few, else the ten with
    /// the most items.
    pub fn of(realm: &str, league: &str, rows: Vec<TabRow>, expand: bool) -> Tabs {
        let folders = rows.iter().filter(|t| t.r#type == "Folder").count();
        let fetched = rows.iter().filter(|t| t.fetched_at.is_some()).count();
        let mut by_type: Vec<TabType> = Vec::new();
        for t in &rows {
            let at = match by_type.iter().position(|k| k.kind == t.r#type) {
                Some(at) => at,
                None => {
                    by_type.push(TabType {
                        kind: t.r#type.clone(),
                        tabs: 0,
                        fetched: 0,
                        items: 0,
                    });
                    by_type.len() - 1
                }
            };
            let kind = &mut by_type[at];
            kind.tabs += 1;
            kind.fetched += usize::from(t.fetched_at.is_some());
            kind.items += t.item_count;
        }
        by_type.sort_by(|a, b| b.items.cmp(&a.items).then_with(|| a.kind.cmp(&b.kind)));
        let tabs = rows.len();
        let items = rows.iter().map(|t| t.item_count).sum();
        let whole = expand || tabs <= LISTED_WHOLE;
        let rows = if whole {
            rows
        } else {
            let mut most = rows;
            most.sort_by(|a, b| {
                b.item_count
                    .cmp(&a.item_count)
                    .then_with(|| a.id.cmp(&b.id))
            });
            most.truncate(MOST_ITEMS);
            most
        };
        let left_out = tabs - rows.len();
        Tabs {
            realm: realm.to_string(),
            league: league.to_string(),
            tabs,
            fetched,
            never_fetched: tabs.saturating_sub(fetched + folders),
            folders,
            items,
            by_type,
            listed: if whole { "all" } else { "most_items" },
            rows,
            left_out,
            left_out_needs: (left_out > 0).then_some("expand"),
        }
    }
}
