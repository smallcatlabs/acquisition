//! `shop render` (C74, C72): the forum shop page as a pure function of
//! the listing state ([`ListingReport`], C69/C81), the shipped currency
//! table (C68), a template and the policy table below. Built by the
//! pricing slice's plan step 6 (`PRICING-SLICE.md` at `b6197a82`, 2026-09-07). Nothing
//! here reads a store, contacts a daemon or sends anything: the CLI
//! (`acq shop render`, `shop_cmd.rs`) writes the pages to stdout and the
//! owner pastes them by hand (`SURFACES.md`, the forum row).
//!
//! # Decisions as recorded
//!
//! **C74 — `shop render` is in scope as a derivation, publishing isn't,
//! and every item the page omits is counted with a named reason.** The
//! page is a pure function of facts, intent, reference data, a template
//! and a ruled policy; it is written to stdout in pages labelled *n* of
//! *N*; it sends nothing; the template is an input, not stored. Nothing
//! is skipped silently: what the game already lists (C81) is omitted and
//! counted, an unruled cell is blocked, a prior post is never read (T6).
//! The policy's rows cite trade claims (T7, T13) and owner observations
//! (T11, T12), never its own output. *Why:* every human surface is a
//! derivation over a machine surface (pattern 3); posting stays parked
//! behind its own session. *Details:* `shop.rs`. *Pinned:* the `c74_`
//! tests. Amended 2026-09-06.
//!
//! **C72 — Pricing never edits the sync policy, and a price never locks a
//! tab into refresh.** The relationship between the two kinds of intent is
//! reported, not enforced: the consumer that needs freshness — `shop
//! render` first — names priced locations outside the policy's coverage,
//! priced facts older than its stated window, and items moved or reindexed
//! since the render's basis, each with the remedy (the policy edit, or the
//! `RefreshPlan` it would take, C41). *Why:* C++'s "priced tabs are always
//! refreshed" is one kind of intent silently rewriting another; a report
//! keeps the policy what its author wrote (07 §10, 08 boundary 7).
//! *Details:* `shop.rs`. *Pinned:* the `c72_` test. Ruled 2026-09-03;
//! amended 2026-09-04.
//!
//! # As built
//!
//! **One cell per item.** Every item listing of the report lands in
//! exactly one [`Cell`] of the policy table, read from the effective
//! outcome — `side` and `kind` together (the render handoff,
//! `PRICING-SLICE.md` at `b6197a82`) — and then from the item's address. A cell's
//! verdict is `post`, `omit`, `block` or `off_page`; the table's opening
//! rows are the two that post (a stash item at a listed tab, a character
//! item in a slot), everything unobserved is a blocked row citing the
//! open question it waits on (`docs/design/trade-ground-truth.md`, Q1–Q7),
//! and each row carries its count, so the page's omissions are the table
//! read down. The order of the reads is the order of `cell`: what the
//! effective price says (the game decides → omitted, since the site
//! already shows it and a page must not contradict it, C81; a hand `skip`
//! → omitted, as the word means; `no_price` → blocked on Q5; nothing →
//! off the page; unresolved → blocked, never treated as unpriced), then
//! what a hand price carries (a kind other than `exact` or `negotiable`
//! → blocked, since no row of the table says how it is written; a ratio
//! → Q6; a retired tag → C68), then whether the item can be addressed at
//! all (a league-less character's item is not evidence for this league;
//! the realm must be one the site lists — `pc`, `xbox`, `sony`, T4, and
//! any other word is blocked, not assumed; a socketed item has no
//! position, T13; a substash item's link is unobserved, Q3; a stash
//! item's tab must have been listed for its index, T13, and a recorded
//! index below zero is blocked as a corrupt fact, never ranked; a
//! character item needs its slot). Vocabularies are
//! matched explicitly, so a new kind or realm lands in a blocked cell
//! until a row is ruled for it (C74) — the table fails closed. A corrupt
//! timestamp in a fact saturates the age it yields; nothing here
//! panics on a store row (C47).
//!
//! **The link code and the spoiler title.** A stash item renders as
//! `[linkItem realm="<r>" location="Stash<n>" league="<L>" x="<x>"
//! y="<y>"]`, the attributes in the order the website's own link button
//! writes them (T7, T24), where `n` is the tab's one-based rank among
//! the tabs the website lists — the top-level tabs and folder children
//! in listing order, folders and substashes left out (T24: the site's
//! stash view numbers those 48 tabs 0–47 and its link says `Stash17`
//! for web index 16; the API's `index` counts the folder too, so the
//! C++ app's `index + 1` was one too high past a folder). A character
//! item renders as `[linkItem realm="<r>" location="<inventoryId>"
//! character="<name>" x="<x>" y="<y>"]` (T7, T8). The price
//! is the title of the spoiler the links sit in — `[spoiler=" ~price
//! <amount> <word>"]` or `[spoiler=" ~b/o <amount> <word>"]`, the C++
//! app's form (T15) with its leading space, the currency table's `emit`
//! word for the row's tag (C68) and the amount's canonical text (C67).
//! A `no_price` item posts as its link alone under an empty title
//! (`[spoiler=""]`, the C++ app's no-price row): the site lists it as
//! "No Price Set" (T21). The owner's post of 2026-09-07 is the evidence
//! the forum accepts this shape and that the site prices every item in
//! a spoiler by that spoiler's title (T22).
//!
//! **Grouping and pages.** Items with exactly the same price share one
//! spoiler, their links run together on its line: groups sort `~price`
//! before `~b/o` before no price, then by tag, then by amount, in a
//! stable sort over the report's order. Each page is one spoiler titled
//! `Shop Post <n> of <N> (<k> items)` holding the price spoilers one per
//! line; newlines stand only between spoiler tags (the owner's shape,
//! 2026-09-07). Pages are cut so that each page, the template around it
//! included, holds at most `size` characters (the C++ constant 50,000 is
//! the default, and the forum's hard limit, T23) — the page title is
//! reserved at its widest, so a page never grows past the size when its
//! numbers are filled in; a price group that runs across a cut is closed
//! and reopened on the next page; an entry that would not fit an empty
//! page is blocked (`page_size`). The template must hold `[items]`
//! exactly once; each page is the template with its spoiler in that
//! place.
//!
//! **The C72 report** is over the posted items only — an omitted or
//! blocked item asks nothing of a refresh. *Coverage:* each posted item's
//! container against the sync policy's selection for this (realm,
//! league) — a tab by its own id or its parent's, the planner's rule
//! (C37, [`Selection::covers_tab`]); a character by its id. *Staleness:*
//! a posted item in a covered container whose fact is older than the
//! policy's window (`max_age_seconds`), the same declaration the planner
//! refreshes by — so the refresh the line cites is the one that fetches
//! them; an uncovered item past the window is counted beside the
//! coverage line instead, since its remedy is the policy edit first.
//! *Positions:* a posted stash item whose fetch predates the stash
//! listing the page's `Stash<n>` comes from — the two halves of its link
//! were observed at different times, and a tab reindexed between them
//! would move the link (the "moved or reindexed" C72 names; without a
//! stored render basis, parked under "what did I last post", this is the
//! basis the render has). Each line names its remedy: the policy edit,
//! or the refresh — the caller passes the plan's request count, or why
//! it could not compile one ([`PolicySource::Set`]), so the text can say
//! what `acq refresh --plan` would send without this module compiling
//! one, and a plan that fails to compile is said, never swallowed.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;

use acquisition_protocol::realm::Realm;
use acquisition_store::bound::bounded;
use serde::{Deserialize, Serialize};

use crate::currency::{self, CurrencyTableError};
use crate::listing::{Listing, ListingReport, ReportHeader, Side};
use crate::price::{Amount, Price, PriceTarget};
use crate::{Selection, SyncPolicy};

/// The render's JSON shape; changes are additive (C53) until they are
/// not, and then this moves — never for a default bound, a count beside
/// a list or an added optional field (the owner's word of 2026-10-27).
/// The shape is pinned by `reference/shop-render-schema-5.json`
/// (`the_render_json_matches_the_committed_fixture`). Within 5, since
/// 2026-10-27 (step 2g, C53's read economy): `posted`, `left_out` and
/// each list of `freshness` hold the store's one bound of ten
/// (`acquisition_store::bound::LISTED`) unless expanded, each counted
/// whole beside it (`posted_total`, `left_out_total`, `uncovered_total`,
/// `stale_total`, `position_before_listing_total`) — the readiness
/// seat's call 35 was 34,779 bytes for a 2,800-character page; and
/// beside `freshness.oldest_stale_seconds` its words,
/// `oldest_stale_seconds_age` (C53's ages, step 2g round 6). **5**
/// since 2026-10-19: `left_out` lists at most fifty items, the blocked
/// first, with `left_out_total` counting them all, unless expanded —
/// the release seat's rerun met 160 KB of them. **4** since
/// 2026-10-19: `left_out` lists the items left off for a reason and
/// leaves out the ones nothing applies to unless asked
/// ([`RenderOptions::expand`]), saying which in `off_page_listed` — the
/// release seat met 5 MB of them. **3** since 2026-09-07: the owner's
/// page shape (spoilers) renamed `Posted.price` to `title`; 2 measured
/// staleness over the stale set only; neither 1 nor 2 reached a
/// consumer.
pub const SHOP_SCHEMA: u32 = 5;

/// The forum's hard post limit in characters (T23); the C++ app's
/// constant (T15) had it right.
pub const DEFAULT_PAGE_SIZE: usize = 50_000;

/// The token a template holds exactly once; the default template is the
/// token alone.
pub const ITEMS_TOKEN: &str = "[items]";

/// The GGG tab type that groups tabs and holds no items.
const FOLDER: &str = "Folder";

/// What the table says to do with a cell's items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    /// On the page.
    Post,
    /// Left off by rule, counted.
    Omit,
    /// Left off because the cell is unobserved or the item cannot be
    /// addressed, counted; the row names what would unblock it.
    Block,
    /// Nothing applies to it: no row, no statement.
    OffPage,
}

impl Verdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Verdict::Post => "post",
            Verdict::Omit => "omit",
            Verdict::Block => "block",
            Verdict::OffPage => "off_page",
        }
    }
}

/// The policy table's rows, in the order the reads are made.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cell {
    GameLists,
    GameSkips,
    HandSkip,
    NothingApplies,
    Unresolved,
    LeagueUnknown,
    HandNoPrice,
    Ratio,
    RetiredCurrency,
    RealmUnlisted,
    Socketed,
    NoPosition,
    Substash,
    TabUnlisted,
    InvalidIndex,
    NoSlot,
    PageSize,
    UnruledKind,
    StashItem,
    CharacterItem,
}

impl Cell {
    pub const ALL: [Cell; 20] = [
        Cell::StashItem,
        Cell::CharacterItem,
        Cell::HandNoPrice,
        Cell::GameLists,
        Cell::GameSkips,
        Cell::HandSkip,
        Cell::NothingApplies,
        Cell::Unresolved,
        Cell::LeagueUnknown,
        Cell::UnruledKind,
        Cell::Ratio,
        Cell::RetiredCurrency,
        Cell::RealmUnlisted,
        Cell::Socketed,
        Cell::NoPosition,
        Cell::Substash,
        Cell::TabUnlisted,
        Cell::InvalidIndex,
        Cell::NoSlot,
        Cell::PageSize,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Cell::GameLists => "game_lists",
            Cell::GameSkips => "game_skips",
            Cell::HandSkip => "hand_skip",
            Cell::NothingApplies => "nothing_applies",
            Cell::Unresolved => "unresolved",
            Cell::LeagueUnknown => "league_unknown",
            Cell::HandNoPrice => "hand_no_price",
            Cell::Ratio => "ratio",
            Cell::RetiredCurrency => "retired_currency",
            Cell::RealmUnlisted => "realm_unlisted",
            Cell::Socketed => "socketed",
            Cell::NoPosition => "no_position",
            Cell::Substash => "substash",
            Cell::TabUnlisted => "tab_unlisted",
            Cell::InvalidIndex => "invalid_index",
            Cell::NoSlot => "no_slot",
            Cell::UnruledKind => "unruled_kind",
            Cell::PageSize => "page_size",
            Cell::StashItem => "stash_item",
            Cell::CharacterItem => "character_item",
        }
    }

    pub fn verdict(self) -> Verdict {
        match self {
            Cell::StashItem | Cell::CharacterItem | Cell::HandNoPrice => Verdict::Post,
            Cell::GameLists | Cell::GameSkips | Cell::HandSkip => Verdict::Omit,
            Cell::NothingApplies => Verdict::OffPage,
            Cell::Unresolved
            | Cell::LeagueUnknown
            | Cell::Ratio
            | Cell::RetiredCurrency
            | Cell::RealmUnlisted
            | Cell::Socketed
            | Cell::NoPosition
            | Cell::Substash
            | Cell::TabUnlisted
            | Cell::InvalidIndex
            | Cell::NoSlot
            | Cell::UnruledKind
            | Cell::PageSize => Verdict::Block,
        }
    }

    /// The rule and its citation, one sentence.
    pub fn why(self) -> &'static str {
        match self {
            Cell::StashItem => {
                "a hand-priced stash item at a listed tab: `[linkItem realm= location=\"Stash<n>\" league= x= y=]` under the price's spoiler, `n` the tab's rank among the tabs the website lists — folders and substashes left out; a forum price takes precedence over the tab's in-game price"
            }
            Cell::CharacterItem => {
                "a hand-priced character item in a slot: `[linkItem realm= location=\"<inventoryId>\" character= x= y=]` under the price's spoiler"
            }
            Cell::GameLists => {
                "the game already lists it at its own price; a page must not contradict what the site shows"
            }
            Cell::GameSkips => {
                "the game marks it do-not-index (~skip) and the game decides; what the site makes of a forum price against a game skip is not observed"
            }
            Cell::HandSkip => "a hand skip means leave it out",
            Cell::NothingApplies => "no row applies and the game states nothing",
            Cell::Unresolved => {
                "a row that cannot be read could decide: never treated as unpriced; read it with `acq price show`"
            }
            Cell::LeagueUnknown => {
                "the listing gave its character no league, so the report is not evidence it belongs to this one"
            }
            Cell::HandNoPrice => {
                "a hand `no_price` item at an address a posting row takes: its link alone under an empty spoiler title, which the site lists as \"No Price Set\""
            }
            Cell::Ratio => {
                "a ratio on the forum is unobserved; in game a typed ratio unlists the item"
            }
            Cell::RetiredCurrency => {
                "the row cites a retired currency tag: the word the site reads for it is unknown"
            }
            Cell::RealmUnlisted => "the trade site's realms are pc, xbox and sony",
            Cell::Socketed => "a socketed item has no position and cannot be linked",
            Cell::NoPosition => "the fetch gave the item no position, so no link can name it",
            Cell::Substash => {
                "the website's stash view offers no link for an item in a map or unique substash, and no other code for one has been observed: blocked until one is"
            }
            Cell::TabUnlisted => {
                "the tab is not on the current stash listing, so it has no index for `Stash<n>`"
            }
            Cell::InvalidIndex => {
                "the tab's recorded index is below zero: a corrupt fact, blocked rather than ranked"
            }
            Cell::NoSlot => "a character item without an `inventoryId` cannot be addressed",
            Cell::UnruledKind => {
                "a hand price of a kind no row of this table writes (`exact` and `negotiable` are ruled; the rest are blocked)"
            }
            Cell::PageSize => "the entry alone, with the template around it, exceeds the page size",
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One row of the policy table as rendered: the cell, its verdict, the
/// rule, and how many items it took.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyRow {
    pub cell: Cell,
    pub verdict: Verdict,
    pub count: usize,
    pub why: String,
}

/// An item on the page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Posted {
    pub target: PriceTarget,
    pub label: String,
    pub location: PriceTarget,
    pub cell: Cell,
    /// 1-based.
    pub page: usize,
    /// The spoiler title the link sits under, as written: ` ~price 5
    /// chaos`, ` ~b/o 1/5 divine`, or empty for a `no_price` item.
    pub title: String,
    /// The link code, as written on the page.
    pub link: String,
}

/// An item not on the page, and the cell that decided it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeftOut {
    pub target: PriceTarget,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<PriceTarget>,
    pub cell: Cell,
    pub verdict: Verdict,
}

/// One page to paste.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page {
    /// 1-based.
    pub number: usize,
    pub of: usize,
    /// Characters, the template included.
    pub chars: usize,
    pub items: usize,
    /// The tabs the page's links name, each by the `Stash<n>` its links
    /// write beside the tab's own name, in the order the page first
    /// names them: a rank alone names no tab a person knows (C146, a
    /// count says what it counts; the release seat counted links per
    /// `Stash<n>` by hand: cli-opus call 68, cli-sonnet call 62,
    /// cli-opus-2 call 68, mcp-opus-2 call 38). Additive to schema 5.
    #[serde(default)]
    pub tabs: Vec<PageTab>,
    pub text: String,
}

/// A tab one page links to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageTab {
    /// As the page's links write it: `Stash57`, the tab's rank among the
    /// tabs the website lists (T24).
    pub stash: String,
    pub tab: PriceTarget,
    /// The tab's name as GGG gives it.
    pub name: String,
    /// The page's links into the tab.
    pub items: usize,
}

/// What the render was told about the sync policy (C72): none is set,
/// the stored one cannot be read, or the policy at a revision — with the
/// request count of the `RefreshPlan` the caller compiled from it, if it
/// compiled one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicySource<'a> {
    NotSet,
    Unreadable {
        revision: i64,
        why: String,
    },
    Set {
        policy: &'a SyncPolicy,
        revision: i64,
        /// The request count of the `RefreshPlan` the caller compiled
        /// from this policy now, or why it could not.
        refresh: Result<u64, String>,
    },
}

/// The C72 report, over the posted items only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Freshness {
    /// `not_set`, `unreadable`, `league_not_covered` or `covers`.
    pub policy: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_revision: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_problem: Option<String>,
    /// The league's `max_age_seconds`, when the policy covers it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window_seconds: Option<u32>,
    /// What `acq refresh --plan` would send now, when the caller
    /// compiled it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_requests: Option<u64>,
    /// Why the caller could not compile that plan, when it could not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_problem: Option<String>,
    /// Containers of posted items the policy does not cover: ten unless
    /// expanded.
    pub uncovered: Vec<PriceTarget>,
    /// How many `uncovered` holds unbounded.
    #[serde(default)]
    pub uncovered_total: usize,
    /// Posted items in covered containers whose fact is older than the
    /// window — the ones the cited refresh fetches: ten unless expanded.
    pub stale: Vec<PriceTarget>,
    /// How many `stale` holds unbounded.
    #[serde(default)]
    pub stale_total: usize,
    /// Posted items in uncovered containers past the window: the policy
    /// edit comes first for these.
    pub stale_uncovered: usize,
    /// The oldest age among `stale`, seconds — the covered items the
    /// cited refresh fetches, never an uncovered one (schema 2; schema 1
    /// measured every posted item under the name `oldest_seconds`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oldest_stale_seconds: Option<i64>,
    /// C53: `oldest_stale_seconds` in words, an age at the render's time
    /// (`1h ago`), as the text says the oldest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oldest_stale_seconds_age: Option<String>,
    /// Posted stash items whose fetch predates the stash listing their
    /// `Stash<n>` comes from: ten unless expanded.
    pub position_before_listing: Vec<PriceTarget>,
    /// How many `position_before_listing` holds unbounded.
    #[serde(default)]
    pub position_before_listing_total: usize,
}

impl Freshness {
    /// C53, the read economy: each list counted whole, then cut to the
    /// store's bound unless `expand`.
    fn bounded(mut self, expand: bool) -> Freshness {
        for (list, total) in [
            (&mut self.uncovered, &mut self.uncovered_total),
            (&mut self.stale, &mut self.stale_total),
            (
                &mut self.position_before_listing,
                &mut self.position_before_listing_total,
            ),
        ] {
            *total = list.len();
            let (kept, _) = bounded(std::mem::take(list), None, expand);
            *list = kept;
        }
        self
    }
}

/// The counts a first line needs.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ShopCounts {
    pub items: usize,
    pub posted: usize,
    pub omitted: usize,
    pub blocked: usize,
    pub off_page: usize,
    pub pages: usize,
    /// Items by cell; every cell present, zeros included.
    pub by_cell: BTreeMap<Cell, usize>,
}

/// The render: the contract `--json` prints and everything the text is a
/// function of (C53) — the listing header it was resolved from, the
/// inputs (size, template), the policy table with counts, every item's
/// disposition, the pages, and the C72 report. Every item is counted by
/// its cell; `left_out` lists each item left off for a reason — omitted
/// or blocked — and the items nothing applies to only when asked, since
/// they are most of a league and no decision hangs on one of them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShopRender {
    pub schema: u32,
    pub listing: ReportHeader,
    pub rendered_at: i64,
    pub size: usize,
    pub template: String,
    pub policy: Vec<PolicyRow>,
    pub counts: ShopCounts,
    /// The items on the pages, in page order: ten unless expanded (the
    /// pages' `text` is whole either way, never a list, C53).
    pub posted: Vec<Posted>,
    /// How many `posted` holds unbounded: every item on the pages.
    #[serde(default)]
    pub posted_total: usize,
    /// The items left off the page, the blocked first, then the omitted,
    /// then — when `off_page_listed` — the items nothing applies to, each
    /// group in the listing's order. The default lists the store's bound
    /// of ten (C53); `expand` lists them all.
    pub left_out: Vec<LeftOut>,
    /// How many `left_out` would hold unbounded: every item omitted or
    /// blocked, and the items nothing applies to when `off_page_listed`.
    pub left_out_total: usize,
    /// Whether `left_out` lists the items nothing applies to
    /// ([`RenderOptions::expand`]).
    pub off_page_listed: bool,
    pub pages: Vec<Page>,
    pub freshness: Freshness,
}

/// What a render takes besides the report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderOptions<'a> {
    /// Characters per page, template included.
    pub size: usize,
    /// The template; `[items]` exactly once.
    pub template: &'a str,
    pub now: i64,
    pub policy: PolicySource<'a>,
    /// List every item left off in `left_out`, the items nothing applies
    /// to among them, past the default's bound; they are counted either
    /// way.
    pub expand: bool,
}

/// Why a page could not be rendered at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderError {
    /// The template does not hold `[items]` exactly once.
    Template {
        occurrences: usize,
    },
    /// The template alone does not fit the size.
    TemplateTooLarge {
        template_chars: usize,
        size: usize,
    },
    Currency(CurrencyTableError),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::Template { occurrences } => write!(
                f,
                "the template must hold {ITEMS_TOKEN} exactly once ({occurrences} found)"
            ),
            RenderError::TemplateTooLarge {
                template_chars,
                size,
            } => write!(
                f,
                "the template alone is {template_chars} characters, over the page size {size}"
            ),
            RenderError::Currency(e) => write!(f, "currency table: {e}"),
        }
    }
}

impl std::error::Error for RenderError {}

/// A candidate entry before pages are cut: the group it sorts into and
/// the text it puts on the page.
struct Entry {
    target: PriceTarget,
    label: String,
    location: PriceTarget,
    cell: Cell,
    /// Sort key: `~price` before `~b/o` before no price, then tag, then
    /// amount; also the group — equal keys share a spoiler.
    group: (u8, String, Option<Amount>),
    link: String,
    /// A stash item's `Stash<n>` and its tab's name, for the page's
    /// tabs ([`PageTab`]); `None` for a character's item.
    stash: Option<(String, String)>,
    /// The spoiler title; empty for no price.
    title: String,
    /// Basis, for the C72 report.
    seen_response: Option<i64>,
    seen_at: Option<i64>,
    /// The tab's parent (a folder), for coverage.
    parent: Option<String>,
}

/// A tab's number in a forum link, or why it has none.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StashNumber {
    /// One-based rank among the tabs the website lists.
    Rank(usize),
    /// A recorded index below zero.
    Invalid,
}

/// `Stash<n>` for every listed tab (T24): the website's stash view lists
/// the top-level tabs and folder children in listing order, folders and
/// substashes left out, numbered from 0, and its link writes that number
/// plus one. The API's `index` counts folders, so the rank is taken over
/// the listed tabs sorted by `index`, never `index + 1` itself.
fn stash_numbers(report: &ListingReport) -> HashMap<String, StashNumber> {
    let mut listed: Vec<(i64, &str, bool)> = report
        .listings
        .iter()
        .filter_map(|l| match (&l.subject.target, l.subject.index) {
            (PriceTarget::Tab { id, .. }, Some(index)) => Some((
                index,
                id.as_str(),
                l.subject.tab_type.as_deref() == Some(FOLDER),
            )),
            _ => None,
        })
        .collect();
    listed.sort_by_key(|(index, _, _)| *index);
    let mut out = HashMap::new();
    let mut rank = 0usize;
    for (index, id, folder) in listed {
        if index < 0 {
            out.insert(id.to_string(), StashNumber::Invalid);
        } else if !folder {
            rank += 1;
            out.insert(id.to_string(), StashNumber::Rank(rank));
        }
    }
    out
}

/// The cell an item falls in, and — when it posts — its entry.
fn cell(
    l: &Listing,
    report: &ListingReport,
    table: &currency::CurrencyTable,
    stash_numbers: &HashMap<String, StashNumber>,
) -> Result<Entry, Cell> {
    let s = &l.subject;
    let e = &l.effective;
    // The kinds a row of the table writes, matched by name: a kind this
    // build does not know how to write is blocked, never assumed exact.
    let (price, prefix, order): (Option<&Price>, &str, u8) = match (e.side, e.kind.as_str()) {
        (Some(Side::Game), "skip") => return Err(Cell::GameSkips),
        (Some(Side::Game), _) => return Err(Cell::GameLists),
        (Some(Side::Manual), "skip") => return Err(Cell::HandSkip),
        (Some(Side::Manual), "no_price") => (None, "", 2),
        (Some(Side::Manual), kind @ ("exact" | "negotiable")) => match &e.price {
            Some(p) => (
                Some(p),
                if kind == "exact" { " ~price" } else { " ~b/o" },
                u8::from(kind == "negotiable"),
            ),
            // A manual side with a priced kind carries its price; a
            // report that says otherwise is unreadable here.
            None => return Err(Cell::Unresolved),
        },
        (Some(Side::Manual), _) => return Err(Cell::UnruledKind),
        (None, "unresolved") => return Err(Cell::Unresolved),
        (None, _) => return Err(Cell::NothingApplies),
    };
    let (title, group) = match price {
        None => (String::new(), (order, String::new(), None)),
        Some(price) => {
            if matches!(price.amount, Amount::Ratio { .. }) {
                return Err(Cell::Ratio);
            }
            let word = match table.by_tag(&price.currency) {
                Some(row) if !row.is_retired() => row.emit.clone(),
                // A row cites a tag the table holds (C67's write rule); one
                // it does not is a newer table's, unknown to this build.
                _ => return Err(Cell::RetiredCurrency),
            };
            (
                format!("{prefix} {} {word}", price.amount),
                (order, price.currency.clone(), Some(price.amount)),
            )
        }
    };
    if s.league_unknown {
        return Err(Cell::LeagueUnknown);
    }
    // The realms the site lists (T4), by name: any other word is a
    // realm this table has no row for.
    let realm = report.header.realm;
    if !matches!(realm, Realm::Pc | Realm::Xbox | Realm::Sony) {
        return Err(Cell::RealmUnlisted);
    }
    if s.socketed_in.is_some() {
        return Err(Cell::Socketed);
    }
    let (Some(x), Some(y)) = (s.x, s.y) else {
        return Err(Cell::NoPosition);
    };
    let Some(location) = s.location.clone() else {
        return Err(Cell::NoPosition);
    };
    let (cell, link, parent, stash) = match &location {
        PriceTarget::Substash { .. } => return Err(Cell::Substash),
        PriceTarget::Character { .. } => {
            let Some(character) = report.find(&location) else {
                return Err(Cell::NoSlot);
            };
            let Some(slot) = s.inventory_id.as_deref() else {
                return Err(Cell::NoSlot);
            };
            (
                Cell::CharacterItem,
                format!(
                    "[linkItem realm=\"{}\" location=\"{slot}\" character=\"{}\" x=\"{x}\" y=\"{y}\"]",
                    realm.as_str(),
                    character.subject.name
                ),
                None,
                None,
            )
        }
        PriceTarget::Tab { .. } | PriceTarget::Item { .. } => {
            let Some(tab) = report.find(&location) else {
                return Err(Cell::TabUnlisted);
            };
            // `Stash<n>` is the tab's rank among the tabs the website
            // lists (T24); a tab never listed has none, a negative index
            // is a corrupt fact.
            let PriceTarget::Tab { id, .. } = &location else {
                return Err(Cell::TabUnlisted);
            };
            let stash_n = match stash_numbers.get(id) {
                Some(StashNumber::Rank(n)) => *n,
                Some(StashNumber::Invalid) => return Err(Cell::InvalidIndex),
                None => return Err(Cell::TabUnlisted),
            };
            let parent = tab.chain.get(1).and_then(|t| match t {
                PriceTarget::Tab { id, .. } => Some(id.clone()),
                _ => None,
            });
            (
                Cell::StashItem,
                format!(
                    "[linkItem realm=\"{}\" location=\"Stash{stash_n}\" league=\"{}\" x=\"{x}\" y=\"{y}\"]",
                    realm.as_str(),
                    report.header.league
                ),
                parent,
                Some((format!("Stash{stash_n}"), tab.subject.name.clone())),
            )
        }
    };
    // A no-price item's cell is the no-price row, once its address
    // passed the checks a posting row makes.
    let cell = if price.is_none() {
        Cell::HandNoPrice
    } else {
        cell
    };
    Ok(Entry {
        target: s.target.clone(),
        label: s.label(),
        location,
        cell,
        group,
        link,
        stash,
        title,
        seen_response: l.basis.response,
        seen_at: l.basis.at,
        parent,
    })
}

/// Render the page set (module doc). Pure: the report, the shipped
/// currency table and the options in; the render out.
pub fn render(report: &ListingReport, opts: &RenderOptions<'_>) -> Result<ShopRender, RenderError> {
    let occurrences = opts.template.matches(ITEMS_TOKEN).count();
    if occurrences != 1 {
        return Err(RenderError::Template { occurrences });
    }
    let template_chars = opts.template.chars().count();
    let overhead = template_chars - ITEMS_TOKEN.chars().count();
    if overhead > opts.size {
        return Err(RenderError::TemplateTooLarge {
            template_chars,
            size: opts.size,
        });
    }
    let table = currency::table().map_err(RenderError::Currency)?;

    let mut counts = ShopCounts::default();
    for c in Cell::ALL {
        counts.by_cell.insert(c, 0);
    }
    let mut left_out = Vec::new();
    let mut take = |counts: &mut ShopCounts,
                    target: &PriceTarget,
                    label: String,
                    location: Option<PriceTarget>,
                    cell: Cell| {
        *counts.by_cell.entry(cell).or_default() += 1;
        match cell.verdict() {
            Verdict::Post => counts.posted += 1,
            Verdict::Omit => counts.omitted += 1,
            Verdict::Block => counts.blocked += 1,
            Verdict::OffPage => counts.off_page += 1,
        }
        let listed = match cell.verdict() {
            Verdict::Post => false,
            Verdict::OffPage => opts.expand,
            Verdict::Omit | Verdict::Block => true,
        };
        if listed {
            left_out.push(LeftOut {
                target: target.clone(),
                label,
                location,
                cell,
                verdict: cell.verdict(),
            });
        }
    };
    // Classify first; the page cutter's size check needs the candidate
    // count, since the page title is reserved at its widest.
    let numbers = stash_numbers(report);
    let mut candidates: Vec<Entry> = Vec::new();
    for l in report.listings.iter().filter(|l| l.subject.is_item()) {
        counts.items += 1;
        match cell(l, report, table, &numbers) {
            Ok(entry) => candidates.push(entry),
            Err(cell) => take(
                &mut counts,
                &l.subject.target,
                l.subject.label(),
                l.subject.location.clone(),
                cell,
            ),
        }
    }
    let cut = cut_pages(candidates, opts.template, opts.size);
    for e in &cut.too_large {
        take(
            &mut counts,
            &e.target,
            e.label.clone(),
            Some(e.location.clone()),
            Cell::PageSize,
        );
    }
    for e in &cut.entries {
        take(
            &mut counts,
            &e.target,
            e.label.clone(),
            Some(e.location.clone()),
            e.cell,
        );
    }
    counts.pages = cut.pages.len();
    // The blocked first — a person can act on them — then the omitted,
    // then what nothing applies to; the default lists a bound of them.
    left_out.sort_by_key(|l: &LeftOut| match l.verdict {
        Verdict::Block => 0,
        Verdict::Omit => 1,
        Verdict::OffPage | Verdict::Post => 2,
    });
    let left_out_total = left_out.len();
    let (left_out, _) = bounded(left_out, None, opts.expand);
    let posted_total = cut.posted.len();
    let (posted, _) = bounded(cut.posted, None, opts.expand);

    let freshness = freshness(report, &cut.entries, opts).bounded(opts.expand);
    let policy = Cell::ALL
        .into_iter()
        .map(|c| PolicyRow {
            cell: c,
            verdict: c.verdict(),
            count: counts.by_cell.get(&c).copied().unwrap_or(0),
            why: c.why().to_string(),
        })
        .collect();
    Ok(ShopRender {
        schema: SHOP_SCHEMA,
        listing: report.header.clone(),
        rendered_at: opts.now,
        size: opts.size,
        template: opts.template.to_string(),
        policy,
        counts,
        posted,
        posted_total,
        left_out,
        left_out_total,
        off_page_listed: opts.expand,
        pages: cut.pages,
        freshness,
    })
}

/// What the page cutter returns ([`cut_pages`]).
struct Cut {
    /// The pages, numbered from 1, each within the size.
    pages: Vec<Page>,
    /// One record per entry on a page, in page order.
    posted: Vec<Posted>,
    /// The entries on the pages, in the same order — the set the C72
    /// report runs over.
    entries: Vec<Entry>,
    /// The candidates that would not fit an empty page (`page_size`),
    /// in their order.
    too_large: Vec<Entry>,
}

/// The page cutter (module doc, "Grouping and pages"): pure over the
/// candidates, the template and the size. An entry that would not fit
/// an empty page — the page spoiler reserved at its widest, the template
/// around it — is set aside; the rest sort into their groups (equal keys
/// share a spoiler) and are cut so that each page's text, the template
/// included, holds at most `size` characters, a group that runs across a
/// cut closed and reopened on the next page. The template holds `[items]`
/// once and its overhead is within `size` ([`render`] checks both before
/// calling). *Pinned:* `c74_any_entries_cut_into_pages_within_the_size…`
/// (the property test) and `c74_pages_are_cut_under_the_size…` (the boundaries).
fn cut_pages(candidates: Vec<Entry>, template: &str, size: usize) -> Cut {
    let overhead = template
        .chars()
        .count()
        .saturating_sub(ITEMS_TOKEN.chars().count());
    let fixed = page_fixed_chars(candidates.len(), overhead);
    let (mut entries, too_large): (Vec<Entry>, Vec<Entry>) =
        candidates.into_iter().partition(|entry| {
            group_open(&entry.title).chars().count()
                + GROUP_CLOSE.len()
                + entry.link.chars().count()
                + fixed
                <= size
        });
    entries.sort_by_cached_key(|e| e.group.clone());

    // Pages: a running character count with the page's fixed cost in
    // it; a group whose next link does not fit is closed here and
    // reopened on the next page.
    let mut pages: Vec<PageBuild> = Vec::new();
    let mut page = PageBuild::default();
    let mut posted = Vec::with_capacity(entries.len());
    for entry in &entries {
        let link_chars = entry.link.chars().count();
        let same_group = page.groups.last().is_some_and(|g| g.key == entry.group);
        let group_cost = if same_group {
            0
        } else {
            group_open(&entry.title).chars().count() + GROUP_CLOSE.len()
        };
        if page.items > 0 && page.chars + group_cost + link_chars + fixed > size {
            pages.push(std::mem::take(&mut page));
        }
        if page.groups.last().is_none_or(|g| g.key != entry.group) {
            page.chars += group_open(&entry.title).chars().count() + GROUP_CLOSE.len();
            page.groups.push(GroupBuild {
                key: entry.group.clone(),
                title: entry.title.clone(),
                links: Vec::new(),
            });
        }
        if let Some(g) = page.groups.last_mut() {
            g.links.push(entry.link.clone());
        }
        page.chars += link_chars;
        page.items += 1;
        if let Some((stash, name)) = &entry.stash {
            match page.tabs.iter_mut().find(|t| t.tab == entry.location) {
                Some(t) => t.items += 1,
                None => page.tabs.push(PageTab {
                    stash: stash.clone(),
                    tab: entry.location.clone(),
                    name: name.clone(),
                    items: 1,
                }),
            }
        }
        posted.push(Posted {
            target: entry.target.clone(),
            label: entry.label.clone(),
            location: entry.location.clone(),
            cell: entry.cell,
            page: pages.len() + 1,
            title: entry.title.clone(),
            link: entry.link.clone(),
        });
    }
    if page.items > 0 {
        pages.push(page);
    }
    let of = pages.len();
    let pages: Vec<Page> = pages
        .into_iter()
        .enumerate()
        .map(|(i, page)| {
            let mut body = page_open(&(i + 1).to_string(), &of.to_string(), page.items);
            for g in &page.groups {
                body.push_str(&group_open(&g.title));
                for link in &g.links {
                    body.push_str(link);
                }
                body.push_str(GROUP_CLOSE);
            }
            body.push_str(PAGE_CLOSE);
            let text = template.replacen(ITEMS_TOKEN, &body, 1);
            Page {
                number: i + 1,
                of,
                chars: text.chars().count(),
                items: page.items,
                tabs: page.tabs,
                text,
            }
        })
        .collect();
    Cut {
        pages,
        posted,
        entries,
        too_large,
    }
}

/// A price spoiler's opening tag.
fn group_open(title: &str) -> String {
    format!("[spoiler=\"{title}\"]")
}

/// Closes a price spoiler; the newline stands between spoiler tags.
const GROUP_CLOSE: &str = "[/spoiler]\n";

/// Closes the page spoiler.
const PAGE_CLOSE: &str = "[/spoiler]\n";

/// The page spoiler's opening tag and its line.
fn page_open(n: &str, of: &str, items: usize) -> String {
    format!(
        "[spoiler=\"Shop Post {n} of {of} ({items} {})\"]\n",
        if items == 1 { "item" } else { "items" }
    )
}

/// What every page costs before its groups: the page spoiler at its
/// widest (every number as many digits as the candidate count has),
/// its close, and the template around it — so filling the numbers in
/// never grows a page past the size.
fn page_fixed_chars(candidates: usize, template_overhead: usize) -> usize {
    let widest = "9".repeat(candidates.max(1).to_string().len());
    let items_widest: usize = widest.parse().unwrap_or(9);
    page_open(&widest, &widest, items_widest).chars().count() + PAGE_CLOSE.len() + template_overhead
}

/// One price spoiler under construction.
struct GroupBuild {
    key: (u8, String, Option<Amount>),
    title: String,
    links: Vec<String>,
}

/// One page under construction: its groups, its character count (the
/// groups' tags and links; the page's fixed cost is added at the cut),
/// its items.
#[derive(Default)]
struct PageBuild {
    groups: Vec<GroupBuild>,
    chars: usize,
    items: usize,
    tabs: Vec<PageTab>,
}

/// The C72 report over the posted entries (module doc).
fn freshness(report: &ListingReport, entries: &[Entry], opts: &RenderOptions<'_>) -> Freshness {
    let mut out = Freshness {
        policy: "not_set".into(),
        policy_revision: None,
        policy_problem: None,
        window_seconds: None,
        refresh_requests: None,
        refresh_problem: None,
        uncovered: Vec::new(),
        uncovered_total: 0,
        stale: Vec::new(),
        stale_total: 0,
        stale_uncovered: 0,
        oldest_stale_seconds: None,
        oldest_stale_seconds_age: None,
        position_before_listing: Vec::new(),
        position_before_listing_total: 0,
    };
    let listing_response = report.header.stash_listing.map(|b| b.response_id);
    for e in entries {
        if let (Some(seen), Some(listing)) = (e.seen_response, listing_response)
            && matches!(e.location, PriceTarget::Tab { .. })
            && seen < listing
        {
            out.position_before_listing.push(e.target.clone());
        }
    }
    let league = match &opts.policy {
        PolicySource::NotSet => return out,
        PolicySource::Unreadable { revision, why } => {
            out.policy = "unreadable".into();
            out.policy_revision = Some(*revision);
            out.policy_problem = Some(why.clone());
            return out;
        }
        PolicySource::Set {
            policy,
            revision,
            refresh,
        } => {
            out.policy_revision = Some(*revision);
            match refresh {
                Ok(n) => out.refresh_requests = Some(*n),
                Err(why) => out.refresh_problem = Some(why.clone()),
            }
            match policy.league(report.header.realm, &report.header.league) {
                Some(league) => league,
                None => {
                    out.policy = "league_not_covered".into();
                    return out;
                }
            }
        }
    };
    out.policy = "covers".into();
    out.window_seconds = Some(league.max_age_seconds);
    let window = i64::from(league.max_age_seconds);
    let mut uncovered: BTreeSet<PriceTarget> = BTreeSet::new();
    for e in entries {
        let covered = match &e.location {
            PriceTarget::Character { id } => league
                .characters
                .as_ref()
                .is_some_and(|sel: &Selection| sel.covers_id(id)),
            PriceTarget::Tab { id, .. } => league
                .tabs
                .as_ref()
                .is_some_and(|sel| sel.covers_tab(id, e.parent.as_deref())),
            _ => false,
        };
        // A corrupt timestamp saturates (C47), as the planner's does.
        let age = e.seen_at.map(|at| opts.now.saturating_sub(at).max(0));
        let past_window = age.is_some_and(|a| a > window);
        if !covered {
            uncovered.insert(e.location.clone());
            if past_window {
                out.stale_uncovered += 1;
            }
        } else if past_window {
            out.stale.push(e.target.clone());
            // The oldest of the stale set, so the CLI's "oldest" belongs
            // to the items its remedy fetches (review round 2, 2026-09-07).
            if let Some(a) = age
                && out.oldest_stale_seconds.is_none_or(|o| a > o)
            {
                out.oldest_stale_seconds = Some(a);
            }
        }
    }
    out.uncovered = uncovered.into_iter().collect();
    out.oldest_stale_seconds_age = out
        .oldest_stale_seconds
        .map(acquisition_store::age::span_ago);
    out
}

#[cfg(test)]
mod tests;
