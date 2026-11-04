//! A league's tabs as `tabs` gives them on both surfaces (C115, C53):
//! counted first — how many, how many fetched and never, how many items,
//! and the same by tab type — then the tabs listed under the read
//! economy's one rule (C53): the ten with the most items, more where
//! `limit` names more, every one under `expand`, and the rest counted.
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
//! a table's rest is its limit. A league of thirty tabs or fewer was
//! listed whole until step 2g folded that into the one rule: the number
//! is the store's (`acquisition_store::bound::LISTED`), not this file's.
//!
//! The counter by type ([`by_type`]) is also a search's: its scope names
//! the never fetched by the same types, so the two say one thing (C118).
//! It is the store's (`acquisition_store::coverage`, C162), beside the
//! coverage it counts for.
//!
//! A [`Selection`] narrows `rows` to one type, to the never fetched, or
//! both: the never fetched by name, which every run of the re-sit asked
//! for first or second (`audits/trade-2026-10/re-sit/`, T7: "0 matches"
//! on a unique, and no way to say which unique-stash pages a refresh
//! should read). The counts stay the league's, so a reader sees the
//! selection against the whole; `selected` counts the rows it holds, and
//! the same listing rule cuts them (a never-fetched tab has no items, so
//! the ten are by id).

use acquisition_store::TabRow;
use acquisition_store::bound::bounded;
// C162: the counter by type is the store's, under the path it had here
use acquisition_store::coverage::FetchSpan;
pub use acquisition_store::coverage::{TabType, by_type};
use serde::Serialize;

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
    /// How old the league's tab facts are (#172; C162, the store's one
    /// derivation): the oldest and newest fetch of its tabs with their
    /// locations, and when its stash listing was seen, each age in words
    /// at the answer's time, under the keys a search's scope names them
    /// by. Filled by [`Tabs::read`]; none on a document made from rows
    /// alone.
    #[serde(flatten)]
    pub span: FetchSpan,
    /// The same counts by GGG's tab type, the most items first.
    pub by_type: Vec<TabType>,
    /// The selection `rows` is cut from, where one was asked, and how
    /// many tabs it holds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<Selection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<usize>,
    /// Which of the selected tabs `rows` holds: `all`, in the listing's
    /// order, or `most_items`, those with the most items, as many as the
    /// request's `limit` or ten.
    pub listed: &'static str,
    pub rows: Vec<TabRow>,
    /// Tabs not in `rows`.
    pub left_out: usize,
    /// What lists them all: the request's `expand` (its `limit` lists
    /// more).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_out_needs: Option<&'static str>,
    /// `rows` grouped, where a reader asks for groups ([`Tabs::grouped`]):
    /// each listed tab in order with the pages of it that are listed
    /// nested under it, a page in no listed tab a group of its own. Empty,
    /// and left out, unless asked, so the flat answer is the same.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<TabGroup>,
}

/// One listed tab and its pages (the 4b sitting round): a map or unique
/// tab with the pages of it the listing names, in their order; any other
/// tab with none.
#[derive(Debug, Clone, Serialize)]
pub struct TabGroup {
    pub tab: TabRow,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pages: Vec<TabRow>,
}

/// Which of a league's tabs `rows` may hold: of one type (GGG's,
/// any case), the never fetched, or both; empty, every tab.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Selection {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub never_fetched: bool,
}

impl Selection {
    pub fn is_empty(&self) -> bool {
        self.kind.is_none() && !self.never_fetched
    }

    fn holds(&self, tab: &TabRow) -> bool {
        self.kind
            .as_deref()
            .is_none_or(|kind| kind.eq_ignore_ascii_case(&tab.r#type))
            && (!self.never_fetched || (tab.fetched_at.is_none() && tab.r#type != "Folder"))
    }

    /// The selection in words, for a line of text: `never-fetched
    /// UniqueStash tabs`.
    pub fn words(&self) -> String {
        format!(
            "{}{} tabs",
            if self.never_fetched {
                "never-fetched "
            } else {
                ""
            },
            self.kind.as_deref().unwrap_or("")
        )
        .replace("  ", " ")
    }
}

impl Tabs {
    /// The league's tabs, as the store lists them, counted and listed
    /// under the one rule (C53), each listed row's fetch said as an age at
    /// `now`, the answer's time: the selection's rows whole, in the
    /// listing's order, where `expand` is asked or where they fit the
    /// bound (`limit`, else ten); else those with the most items, as many
    /// as the bound, the rest counted.
    pub fn of(
        realm: &str,
        league: &str,
        rows: Vec<TabRow>,
        select: &Selection,
        limit: Option<usize>,
        expand: bool,
        now: i64,
    ) -> Tabs {
        let folders = rows.iter().filter(|t| t.r#type == "Folder").count();
        let fetched = rows.iter().filter(|t| t.fetched_at.is_some()).count();
        let by_type = acquisition_store::coverage::by_type(
            rows.iter()
                .map(|t| (t.r#type.as_str(), t.fetched_at.is_some(), t.item_count)),
        );
        let tabs = rows.len();
        let items = rows.iter().map(|t| t.item_count).sum();
        let rows: Vec<TabRow> = rows.into_iter().filter(|t| select.holds(t)).collect();
        let selected = rows.len();
        // the most items first, then by id, cut by the one bound; where
        // nothing is cut, the listing's own order
        let mut most: Vec<(usize, TabRow)> = rows.into_iter().enumerate().collect();
        most.sort_by(|(_, a), (_, b)| {
            b.item_count
                .cmp(&a.item_count)
                .then_with(|| a.id.cmp(&b.id))
        });
        let (mut kept, left_out) = bounded(most, limit, expand);
        let whole = left_out == 0;
        if whole {
            kept.sort_by_key(|(at, _)| *at);
        }
        let rows: Vec<TabRow> = kept.into_iter().map(|(_, row)| row.aged(now)).collect();
        Tabs {
            realm: realm.to_string(),
            league: league.to_string(),
            tabs,
            fetched,
            never_fetched: tabs.saturating_sub(fetched + folders),
            folders,
            items,
            span: FetchSpan::default(),
            by_type,
            selection: (!select.is_empty()).then(|| select.clone()),
            selected: (!select.is_empty()).then_some(selected),
            listed: if whole { "all" } else { "most_items" },
            rows,
            left_out,
            left_out_needs: (left_out > 0).then_some("expand"),
            groups: Vec::new(),
        }
    }

    /// [`Tabs::of`] the league's tabs as `store` holds them, with the
    /// facts block over them (#172), and what the store holds of each of
    /// the realm's leagues, for the wrapper beside it (`leagues_held`),
    /// all from one read snapshot (`Store::tabs_document`): the one read
    /// every surface makes, `acq tabs`, the MCP's `tabs` and the web UI's
    /// refresh page.
    pub fn read(
        store: &acquisition_store::Store,
        realm: &str,
        league: &str,
        select: &Selection,
        limit: Option<usize>,
        expand: bool,
        now: i64,
    ) -> anyhow::Result<(Tabs, Vec<acquisition_store::LeagueHeld>)> {
        // the rows, their facts and the holdings from one read snapshot
        // (the review of round C, finding 1; of its fix round, finding 3)
        let (rows, coverage, held) = store.tabs_document(realm, league, now)?;
        let mut tabs = Tabs::of(realm, league, rows, select, limit, expand, now);
        tabs.span = coverage.span;
        Ok((tabs, held))
    }

    /// The document with its rows grouped (`groups`): a page — a row
    /// whose parent is a map or unique tab among the rows — is nested
    /// under that tab, and every other row is a group in the rows' order.
    /// A folder's children are tabs of their own, never its pages. The
    /// web UI's refresh page renders these groups and computes none
    /// (the sitting round's review, its fourth finding).
    pub fn grouped(mut self) -> Tabs {
        let container = |id: &str| {
            self.rows
                .iter()
                .any(|r| r.id == id && matches!(r.r#type.as_str(), "MapStash" | "UniqueStash"))
        };
        let page_of = |row: &TabRow| {
            row.parent
                .as_deref()
                .filter(|p| container(p))
                .map(str::to_string)
        };
        let mut groups: Vec<TabGroup> = Vec::new();
        let mut pages: Vec<(String, TabRow)> = Vec::new();
        for row in &self.rows {
            match page_of(row) {
                Some(parent) => pages.push((parent, row.clone())),
                None => groups.push(TabGroup {
                    tab: row.clone(),
                    pages: Vec::new(),
                }),
            }
        }
        for (parent, page) in pages {
            if let Some(group) = groups.iter_mut().find(|g| g.tab.id == parent) {
                group.pages.push(page);
            }
        }
        self.groups = groups;
        self
    }
}
