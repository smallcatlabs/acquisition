//! The listing state (C69, C70, C80): what every item, tab, substash and
//! character in one (realm, league) is listed as — the manual side, the
//! game side, and their relation — resolved by one pure function,
//! [`resolve`], over a [`PricingSnapshot`]. Built by the pricing slice's
//! plan step 4 (`PRICING-SLICE.md` at `b6197a82`, 2026-09-06). Nothing here reads a
//! store or writes anything; the CLI (`acq price status | show | list`,
//! `price_cmd.rs`) renders the report under C53.
//!
//! # Rulings held here
//!
//! - **C69** (a listing's two sides).
//! - **C70** (a priced tab covers its children).
//! - **C81** (the most specific statement wins): a prior forum post is
//!   never a side (T6).
//! - **C80** (the nameable tab): a map or unique substash has the game's
//!   own name and no `public`, so an item in one reads its note, then the
//!   parent's name and `public`, the substash's name reported verbatim
//!   beside it.
//!
//! # As built
//!
//! **One subject per row of the facts.** Every live tab (folders and
//! substash stubs included), every live character of the league, and
//! every live item at them gets a [`Listing`]; containers come first in
//! the snapshot's order, then items. A listing is its [`Subject`] (the
//! [`PriceTarget`] it would be priced at, plus what a line needs: name,
//! type line, stack, tab type, the container an item sits in), the two
//! sides, the relation with a sentence naming both sides, and the
//! [`Basis`] the facts came from (the fetch that last saw an item; the
//! listing that last named a tab or character). The report stamps the
//! parser and reference versions every reading was made under
//! ([`NOTE_PARSER_VERSION`], [`CURRENCY_TABLE_VERSION`]) and the snapshot's
//! bases, so a later reading of the same text can be told from a later
//! fact.
//!
//! **The manual side walks the specificity chain (C70)** and stops at the
//! first row it meets: an item's own row, then its substash's, then the
//! tab's, then the folder the tab sits in (a folder is a grouping, but a
//! row on it covers the tabs it groups the way a policy id does, C37); a
//! character item's own row, then its character's. A row on a target is
//! any `buyout` value, `skip` and `no_price` included — coverage is the
//! row's existence, never its type. The side names the row it took
//! (`from`, with `inherited` set when that is not the subject's own
//! target), its revision, when it was written and through what (C65), so
//! a `set` can cite the revision it read. A row that cannot be read (a
//! newer value schema, a malformed address) never yields to a less
//! specific one: the walk stops, `manual` is empty and `manual_problem`
//! says why — the report's `rows.unreadable` lists every such row once.
//! Rows whose realm is not the snapshot's are counted `other_realm`; valid
//! rows naming nothing in these facts (another league's item, a removed
//! tab) are listed `unmatched` — every nothing says which nothing (C53) —
//! each whole, in the manual side's shape with `from` the row's own
//! target, since an orphan is intent to look at (C35) and no listing
//! carries it; a caller holding the facts moves the first kind to
//! `other_league`, by address ([`ListingReport::place_other_leagues`]).
//!
//! **The game side is a statement only where the trade index can see it
//! (C81, T1).** For a subject in a public tab, [`game_side::read`] reads
//! the note, then the tab name C80 chooses (a top-level tab or a folder
//! child its own; a substash its parent's, its own name carried in
//! `substash_name`; a folder never), and the statement is the first that
//! reads as a price or `skip`: a note that reads as anything else — free
//! text, the dialog's empty-amount residue, an unknown word, a ratio the
//! site would refuse — has no effect and the tab applies (T18; the C++
//! app substituted the tab for a broken note, and so does the site).
//! Where there is no public tab — a non-public tab, a character's items,
//! a folder, a substash whose parent is not on record — the statement is
//! `none` whatever the texts say; a text that would have read as a price
//! or `skip` is marked `residue`, so a remove-only tab's old price is
//! shown but never a side. Both texts ride verbatim with their parser
//! readings beside the statement. `public` is the listing's
//! `metadata.public` being exactly `true` (absent means not public,
//! census 2c); it is `None` where there is no stash to publish. A note
//! the store could not read — the body carries one that is no string
//! (`ItemSnapshot::note_unread`) — is a statement of unknown content at
//! the most specific level there is, so where the game side can speak —
//! a public tab — the effective price is `unresolved` whatever the rows
//! and the tab say, as an unreadable row leaves it (C81); where it cannot
//! — a non-public tab, a character's items — the note is residue at most
//! whatever it says, and the rows decide as they would. The game side
//! states nothing from it either way and says so (`note_unread`). Found
//! by the search's generators at step 9 (2026-09-25), when the snapshot
//! still failed whole on such a body; the public gate by the step's
//! outside review, which found the override deciding a private tab's
//! item.
//!
//! **The relation is over the two sides' presence and content.** `none`:
//! no row applies and the game states nothing. `manual_only` and
//! `game_only`: one side speaks. `agree`: both do and say the same thing —
//! the same price under the same prefix, or `skip` beside `~skip`.
//! `conflict`: both speak and differ. A row the walk could not read is
//! named in the sentence where "no row applies" would otherwise stand.
//! The relation describes; the effective price decides — and `list`'s
//! default selection is every relation but `none` plus every unresolved
//! item, so an unreadable row is never hidden by the relation word.
//!
//! **The effective price (C81)** is the more specific of the two
//! statements, the game's on a tie. A statement's level is its position
//! on the subject's chain: the manual side's is the row's target (an
//! item's own row is level 0, its substash's 1, its tab's 2, its folder's
//! 3; a character item's character is 1); the game's is 0 for a note and
//! the C80 tab's position for a tab name. So an item row beats a game tab
//! price, a substash row beats its parent's game name, a game note beats
//! an item row, a game tab price beats a tab row or a folder row.
//! [`Effective`] names the winner, its side, the target it came from and
//! why. A row the walk stopped at because it cannot be read is a
//! statement of unknown content: when it sits at a level that could beat
//! the game's (or there is no game statement) the effective price is
//! `unresolved`, naming the row — never the game's by default, since that
//! is the fall-through C++ had; when the game's statement is at least as
//! specific, the game decides as it would against any row there. What a
//! page does with the winner — omit what the game already lists — is the
//! render's (C74).
//!
//! **League unknown.** The store carries a character the listing gave no
//! league under every league of its realm (the planner's rule, so every
//! plan can report it as outside coverage); the pricing snapshot inherits
//! that read. Here such a character and its items are flagged
//! `league_unknown` and counted, in every league's report: the report is
//! not evidence they belong to its league, and a render blocks them by
//! name rather than placing them on a page. None of the owner's
//! characters is one (census 2c); the flag exists for the store's rule,
//! not for a case in hand.
//!
//! **Two item sets per container, and the views.** Every listing carries
//! its manual chain, so a container has two sets: the items physically in
//! it ([`ListingReport::items_in`]) and the items a row on it would cover
//! ([`ListingReport::items_covered_by`]) — the same for an ordinary tab,
//! different for a folder (nothing here, its tabs' items covered) and for
//! a map or unique tab (its own items here, its substashes' covered too).
//! [`ListView`] and [`ShowView`] are `list`'s and `show`'s JSON contracts
//! and everything their text is a function of (C53): the header, the
//! selection, the containers the selected items sit in, both item sets.

use std::collections::{BTreeMap, HashMap};
use std::fmt;

use acquisition_protocol::realm::Realm;
use acquisition_store::age;
use acquisition_store::bound::bounded;
use acquisition_store::{
    AnnotationRow, CharacterSnapshot, ItemSnapshot, ListingBasis, PricingSnapshot, TabSnapshot,
    ValueError, check_value,
};
use serde::{Deserialize, Serialize};

use crate::currency::{self, CURRENCY_TABLE_VERSION, CurrencyTable, CurrencyTableError};
use crate::game_side::{self, GamePrice, NOTE_PARSER_VERSION, Source};
use crate::price::{Buyout, Price, PriceTarget};

/// The report's JSON shape, stamped on every [`ListingReport`] and view;
/// changes are additive (C53) until they are not, and then this moves.
/// **2** since 2026-09-06: schema 1 never left development, and the
/// second step-4 review's count fields (`by_game_statement`,
/// `game_priced`) replaced rather than joined `by_game_reading`,
/// `game_priced_public` and `game_priced_not_public` — a rename, so a
/// bump, not a compatibility shim nobody would read. **3** since
/// 2026-10-19: `rows.unmatched` lists each row whole (the manual side's
/// shape) where it listed the bare address — a replaced shape, so a
/// bump. The shape is pinned by `reference/listing-report-schema-3.json`
/// (`the_report_json_matches_the_committed_fixture`); a change that
/// fails it is either additive (regenerate the fixture) or a bump.
/// Within 3 (step 2g round 6, C53's ages, additive): beside an epoch a
/// view's text says as an age, its words under the epoch's name and
/// `_age` — `status`'s `taken_at_age` and listings', `show`'s
/// `taken_at_age`, `manual.updated_at_age` and `basis.at_age` — absent
/// from the report itself and from `list`, which say none.
pub const LISTING_SCHEMA: u32 = 3;

/// The GGG tab type that groups tabs and holds no items.
const FOLDER: &str = "Folder";

/// What a listing is about, and what a line about it needs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Subject {
    /// The address a row on this subject itself would have.
    pub target: PriceTarget,
    /// The item's name (empty for most non-unique items), the tab's, or
    /// the character's.
    pub name: String,
    /// Items: the type line. Empty for a container.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub type_line: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stack_size: Option<i64>,
    /// Tabs and substashes: the GGG type (`PremiumStash`, `MapStash`,
    /// `Folder`, …).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tab_type: Option<String>,
    /// Items: the container they sit in — a tab, substash or character.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<PriceTarget>,
    /// Items: the array they came from (`items`, `equipment`, …).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// Items: the item this one is socketed in; such an item has no
    /// position of its own.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub socketed_in: Option<String>,
    /// Items: the position in the tab or the character slot, as fetched;
    /// absent on a socketed item (T13). A forum link names it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
    /// Items: the `inventoryId` verbatim — a character item's slot
    /// (`BodyArmour`, `MainInventory`, …), the literal `Stash1` for every
    /// stash item (T13). A forum link to a character item is addressed
    /// by it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inventory_id: Option<String>,
    /// Tabs and substashes: the listing's `index`, from which a forum
    /// link's `Stash<n>` is derived (T13, T15: `index + 1`). `None` when
    /// the tab was fetched directly and never listed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// A character the listing gave no league, and its items: the store
    /// carries them under every league of the realm, so this report is
    /// not evidence they belong to its league (module doc, "League
    /// unknown").
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub league_unknown: bool,
}

impl Subject {
    /// An item, as opposed to a container.
    pub fn is_item(&self) -> bool {
        matches!(self.target, PriceTarget::Item { .. })
    }

    /// The name a line shows: the name, else the type line, else the
    /// address.
    pub fn label(&self) -> String {
        if !self.name.is_empty() {
            self.name.clone()
        } else if !self.type_line.is_empty() {
            self.type_line.clone()
        } else {
            self.target.to_string()
        }
    }
}

/// The manual side: the row that applies, by specificity (C70).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManualSide {
    pub value: Buyout,
    /// The row's own target — the subject's, or the container's it
    /// inherits from.
    pub from: PriceTarget,
    pub inherited: bool,
    pub revision: i64,
    pub updated_at: i64,
    /// C53: `updated_at` in words at the answer's time, on the target
    /// `show` shows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at_age: Option<String>,
    pub written_via: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
}

/// One text the game side read, verbatim, and what it read as.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reading {
    pub text: String,
    pub reading: GamePrice,
}

/// The game side: what the note, then the tab name, says (C69), with
/// both texts verbatim, the tab C80 chose, and whether it is public.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameSide {
    /// The reading that applies: the note's unless it is `none`, else
    /// the tab name's, else `none`.
    pub reading: GamePrice,
    /// Where `reading` came from; absent when it is `none`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// The item's note, verbatim, with its own reading.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<Reading>,
    /// The tab whose name and `public` were read (C80): the subject's own
    /// tab, or a substash's parent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tab: Option<String>,
    /// That tab's name, verbatim, with its own reading.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tab_name: Option<Reading>,
    /// A substash's own name, verbatim, beside the parent's (C80).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substash_name: Option<String>,
    /// The listing's `metadata.public`, exactly `true`; `None` where
    /// there is no stash to publish.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /// A note or tab name that reads as a price or `skip` where the
    /// index cannot see it (no public tab): shown, never a statement
    /// (C81).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub residue: bool,
    /// The item's note is there and could not be read: the effective
    /// price is unresolved by it (the module doc).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub note_unread: bool,
}

/// How the two sides stand to each other (C69).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Relation {
    None,
    ManualOnly,
    GameOnly,
    Agree,
    Conflict,
}

impl Relation {
    pub const ALL: [Relation; 5] = [
        Relation::None,
        Relation::ManualOnly,
        Relation::GameOnly,
        Relation::Agree,
        Relation::Conflict,
    ];

    /// The word, as in JSON.
    pub fn as_str(self) -> &'static str {
        match self {
            Relation::None => "none",
            Relation::ManualOnly => "manual_only",
            Relation::GameOnly => "game_only",
            Relation::Agree => "agree",
            Relation::Conflict => "conflict",
        }
    }

    pub fn parse(s: &str) -> Option<Relation> {
        Relation::ALL.into_iter().find(|r| r.as_str() == s)
    }
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Whose statement the effective price is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Game,
    Manual,
}

impl Side {
    pub fn as_str(self) -> &'static str {
        match self {
            Side::Game => "game",
            Side::Manual => "manual",
        }
    }
}

/// What applies (C81): the more specific statement, the game's on a tie.
/// A consumer reads `side` and `kind` together — `none` and `unresolved`
/// both carry no side, and a manual `skip` is not a price to post
/// (`PRICING-SLICE.md` at `b6197a82`, the render handoff).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Effective {
    /// `exact`, `negotiable`, `no_price`, `skip`, `none`, or `unresolved`
    /// (a row that cannot be read could decide; `from` names it).
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    /// Where the winning statement sits: the row's target, or the item
    /// whose note or the tab whose name the game read.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<PriceTarget>,
    /// One sentence naming the winner and the rule that chose it.
    pub why: String,
}

impl Effective {
    /// The side word `Counts::by_effective` and `ListFilter::effective`
    /// use: `game`, `manual`, `none` or `unresolved`.
    pub fn side_word(&self) -> &'static str {
        match (self.side, self.kind.as_str()) {
            (Some(side), _) => side.as_str(),
            (None, "unresolved") => "unresolved",
            (None, _) => "none",
        }
    }
}

impl fmt::Display for Effective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.price, self.kind.as_str()) {
            (Some(p), "negotiable") => write!(f, "{p} b/o"),
            (Some(p), _) => write!(f, "{p}"),
            (None, "no_price") => f.write_str("no price"),
            (None, kind) => f.write_str(kind),
        }
    }
}

/// The fact the subject's texts came from: the fetch that last saw an
/// item, or the listing that last named a tab or character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Basis {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<i64>,
    /// C53: `at` in words at the answer's time (`never` where none), on
    /// the target `show` shows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at_age: Option<String>,
}

/// One subject's listing state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Listing {
    pub subject: Subject,
    /// The manual side's walk (C70), most specific first: the subject's
    /// own target, then each container whose row would cover it. A row
    /// on any of these is what `manual` reports; a target's coverage is
    /// every listing whose chain holds it ([`ListingReport::items_covered_by`]).
    pub chain: Vec<PriceTarget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual: Option<ManualSide>,
    /// A row that would apply could not be read; the walk stopped there.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual_problem: Option<String>,
    pub game: GameSide,
    pub relation: Relation,
    /// One sentence naming both sides.
    pub why: String,
    /// What applies (C81).
    pub effective: Effective,
    pub basis: Basis,
}

/// A `buyout` row the state could not use.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RowProblem {
    pub scope: String,
    pub key: String,
    pub revision: i64,
    pub why: String,
}

/// Where every `buyout` row of the account went.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RowAccounting {
    pub total: usize,
    /// Rows whose target is a subject of these facts.
    pub applied: usize,
    /// Tab and substash rows of another realm.
    pub other_realm: usize,
    /// Readable rows naming nothing in these facts, each whole: no
    /// listing carries them, so this is where they are read. The status
    /// lists ten unless expanded ([`ListingReport::status`]).
    pub unmatched: Vec<ManualSide>,
    /// How many rows name nothing in these facts, `unmatched` whole.
    #[serde(default)]
    pub unmatched_total: usize,
    /// Rows that cannot be read, each with why — a value the buyout kind
    /// refuses, or one the store could not parse at all (C151; the one
    /// kind of unreadable, whichever parse failed): the status lists ten
    /// unless expanded.
    pub unreadable: Vec<RowProblem>,
    /// How many rows cannot be read, `unreadable` whole.
    #[serde(default)]
    pub unreadable_total: usize,
    /// Rows the facts hold in another league of this realm: taken out of
    /// `unmatched`, by address, by [`ListingReport::place_other_leagues`], which a
    /// caller holding the facts runs (the snapshot is one league's and
    /// cannot say); empty until then. The status lists ten unless
    /// expanded.
    #[serde(default)]
    pub other_league: Vec<PriceTarget>,
    /// How many rows the facts hold in another league, `other_league`
    /// whole.
    #[serde(default)]
    pub other_league_total: usize,
}

impl RowAccounting {
    /// Each list's whole count, from the lists as they stand.
    fn counted(&mut self) {
        self.unmatched_total = self.unmatched.len();
        self.unreadable_total = self.unreadable.len();
        self.other_league_total = self.other_league.len();
    }

    /// C53, the read economy (step 2g): each list cut to the store's
    /// bound of ten unless `expand`, its whole count standing beside it.
    fn bounded(mut self, expand: bool) -> RowAccounting {
        self.unmatched = bounded(std::mem::take(&mut self.unmatched), None, expand).0;
        self.unreadable = bounded(std::mem::take(&mut self.unreadable), None, expand).0;
        self.other_league = bounded(std::mem::take(&mut self.other_league), None, expand).0;
        self
    }
}

/// Counts over the items (containers are not items), so `status` needs
/// no listing to say its line.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Counts {
    pub items: usize,
    pub containers: usize,
    /// Items by relation word.
    pub by_relation: BTreeMap<String, usize>,
    /// Items by the game side's statement (`exact`, `negotiable`, `skip`,
    /// `none`) — a statement exists only in a public tab (C81); a note's
    /// raw reading is `invalid_notes` and `residue`.
    pub by_game_statement: BTreeMap<String, usize>,
    /// Items whose note starts like a price (`~price`, `~b/o`, `~skip`)
    /// and does not read as one: no effect, the tab's name applies.
    pub invalid_notes: usize,
    /// Items with a game-side price statement (exact or negotiable);
    /// public by construction.
    pub game_priced: usize,
    /// Tabs (folders and substashes excluded) whose own name reads as a
    /// price, public or not; `priced_tabs_public` are the statements.
    pub priced_tabs: usize,
    pub priced_tabs_public: usize,
    /// Items whose manual side is inherited.
    pub inherited: usize,
    /// Items of a character the listing gave no league.
    pub league_unknown: usize,
    /// Items by the effective price's side: `game`, `manual`, `none`,
    /// `unresolved` (a row that cannot be read could decide).
    pub by_effective: BTreeMap<String, usize>,
    /// Items carrying price text the trade site cannot see (C81): a note
    /// on a character's item, which decides no price until the item is
    /// moved into a public stash tab, or a note or tab name in a tab that
    /// is not public, which decides none until the tab is made public;
    /// the field's name is the contract (C53), its words are these
    /// (C146).
    pub residue: usize,
}

/// What every view of the state says first: whose facts, which
/// (realm, league), when, on which bases, under which versions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportHeader {
    pub schema: u32,
    pub account_uuid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    pub realm: Realm,
    pub league: String,
    pub taken_at: i64,
    /// C53: `taken_at` in words at the answer's time, on a view whose
    /// text says it (`status`, `show`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taken_at_age: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stash_listing: Option<ListingBasis>,
    /// C53: `stash_listing.fetched_at` in words at the answer's time
    /// (`never` where none), on `status`, whose text says it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stash_listing_fetched_at_age: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub character_listing: Option<ListingBasis>,
    /// C53: `character_listing.fetched_at`, the same way.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub character_listing_fetched_at_age: Option<String>,
    pub note_parser_version: u32,
    pub currency_table_version: u32,
}

/// The listing state of one (realm, league): every subject, the row
/// accounting, the counts, and the bases and versions it was resolved
/// under.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListingReport {
    #[serde(flatten)]
    pub header: ReportHeader,
    pub counts: Counts,
    pub rows: RowAccounting,
    /// Containers first (tabs in listing order, then characters), then
    /// items in the snapshot's order.
    pub listings: Vec<Listing>,
}

/// Which items `list` selects: by relation, by the effective price's
/// side (when neither is given: every relation but `none`, plus every
/// unresolved item whatever its relation), physically in one container,
/// or covered by a row on one target (C70) — the two are different sets
/// for a folder or a parent tab — and the first ten of them in listing
/// order (C53, the read economy), `limit` naming how many, `expand` every
/// one.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ListFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<Relation>,
    /// The effective price's side: `game`, `manual`, `none` or
    /// `unresolved`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#in: Option<PriceTarget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub covered_by: Option<PriceTarget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Every item of the selection, whatever `limit` says.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub expand: bool,
}

/// `list`'s contract: everything its text is a function of (C53) — the
/// header, the filter, the containers on the listed items' chains (the
/// one each sits in and those above it, in the report's order, each with
/// its own listing), and the items: `total` selected, `listed` of them
/// here — ten by default, the filter's `limit`, or all under its
/// `expand` — and `counts`, the whole selection counted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListView {
    #[serde(flatten)]
    pub header: ReportHeader,
    pub filter: ListFilter,
    /// How many items the report holds, so an empty selection can say
    /// which nothing it is.
    pub items_on_record: usize,
    /// How many items the selection holds, before any limit.
    pub total: usize,
    /// How many of them `items` lists.
    pub listed: usize,
    /// The whole selection counted, before the bound (C53): `total`,
    /// `returned` (`listed`), `left_out`, `by_relation`, `unresolved`.
    #[serde(default)]
    pub counts: SetCounts,
    pub containers: Vec<Listing>,
    pub items: Vec<Listing>,
}

/// `show`'s contract: the listing, the container an item sits in, and
/// for a container both item sets — physically here, and covered by a
/// row on it (C70) — each counted whole beside the rows it lists (C53,
/// the read economy: ten by default, more on `limit`, all on `expand`;
/// [`ShowView::bounded`]).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShowView {
    #[serde(flatten)]
    pub header: ReportHeader,
    pub listing: Listing,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<Listing>,
    /// A container's items, as many as the bound lists; empty for an
    /// item.
    pub items_here: Vec<Listing>,
    /// `items_here` counted whole, before the bound.
    #[serde(default)]
    pub items_here_counts: SetCounts,
    /// Items whose chain holds this target but that do not sit in it: a
    /// folder's, through its tabs; a parent tab's, through its substashes.
    /// As many as the bound lists; empty for an item.
    pub items_covered_below: Vec<Listing>,
    /// `items_covered_below` counted whole, before the bound.
    #[serde(default)]
    pub items_covered_below_counts: SetCounts,
}

/// C53 — one item set of a `show`, counted whole before it is cut, so a
/// reader never takes the rows listed for the set (step 2g; the cold
/// review of its brief, finding 6). The words are the search answer's
/// (`returned`, `left_out`) and `price_list`'s (`total`); the field's
/// name is the contract (C146).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SetCounts {
    /// Every item the set holds.
    pub total: usize,
    /// How many of them the set lists.
    pub returned: usize,
    /// How many it does not list: `expand` lists them.
    pub left_out: usize,
    /// The whole set by relation word, zeros left out, an item whose
    /// price is unresolved (a row that cannot be read could decide)
    /// never counted as `none`.
    pub by_relation: BTreeMap<String, usize>,
    /// The whole set's items whose price is unresolved, whatever their
    /// relation.
    pub unresolved: usize,
}

impl SetCounts {
    /// A whole set's counts, every item listed.
    fn of(items: &[Listing]) -> SetCounts {
        let unresolved = |l: &Listing| l.effective.kind == "unresolved";
        let mut by_relation = BTreeMap::new();
        for l in items {
            if l.relation == Relation::None && unresolved(l) {
                continue;
            }
            *by_relation
                .entry(l.relation.as_str().to_string())
                .or_insert(0) += 1;
        }
        SetCounts {
            total: items.len(),
            returned: items.len(),
            left_out: 0,
            by_relation,
            unresolved: items.iter().filter(|l| unresolved(l)).count(),
        }
    }
}

impl ShowView {
    /// C53 (an age the text prints stands in words beside its epoch,
    /// made at the answer's own time): the target's hand row's last
    /// write, the fact its texts came from, and the snapshot, said at
    /// `now`, as `show`'s text says them.
    pub fn aged(mut self, now: i64) -> ShowView {
        self.header.taken_at_age = Some(age::ago(now, Some(self.header.taken_at)));
        if let Some(manual) = &mut self.listing.manual {
            manual.updated_at_age = Some(age::ago(now, Some(manual.updated_at)));
        }
        self.listing.basis.at_age = Some(age::ago(now, self.listing.basis.at));
        self
    }

    /// The view under the read economy's one rule (C53): each set cut to
    /// the request's `limit`, else ten, every item under `expand`, its
    /// counts standing whole beside it.
    pub fn bounded(mut self, limit: Option<usize>, expand: bool) -> ShowView {
        for (set, counts) in [
            (&mut self.items_here, &mut self.items_here_counts),
            (
                &mut self.items_covered_below,
                &mut self.items_covered_below_counts,
            ),
        ] {
            let (kept, left_out) = bounded(std::mem::take(set), limit, expand);
            counts.returned = kept.len();
            counts.left_out = left_out;
            *set = kept;
        }
        self
    }
}

impl ListingReport {
    /// Move the unmatched rows the facts hold in another league into
    /// `rows.other_league`, so that "names nothing" is left meaning it:
    /// an orphan is a row to look at (C35), another league's row is
    /// intent in its place. `elsewhere` answers for one target whether
    /// the facts hold it live outside this report's league.
    pub fn place_other_leagues<E>(
        &mut self,
        mut elsewhere: impl FnMut(&PriceTarget) -> Result<bool, E>,
    ) -> Result<(), E> {
        for row in std::mem::take(&mut self.rows.unmatched) {
            if elsewhere(&row.from)? {
                self.rows.other_league.push(row.from);
            } else {
                self.rows.unmatched.push(row);
            }
        }
        self.rows.counted();
        Ok(())
    }

    /// The listing of one target, if it is a subject of these facts.
    pub fn find(&self, target: &PriceTarget) -> Option<&Listing> {
        self.listings.iter().find(|l| &l.subject.target == target)
    }

    /// The item listings sitting in one container.
    pub fn items_in<'a>(&'a self, location: &'a PriceTarget) -> impl Iterator<Item = &'a Listing> {
        self.listings
            .iter()
            .filter(move |l| l.subject.location.as_ref() == Some(location))
    }

    /// The item listings a row on `target` would cover (C70): every item
    /// whose chain holds it, its own items and its children's.
    pub fn items_covered_by<'a>(
        &'a self,
        target: &'a PriceTarget,
    ) -> impl Iterator<Item = &'a Listing> {
        self.listings
            .iter()
            .filter(move |l| l.subject.is_item() && l.chain.contains(target))
    }

    /// The report without its listings.
    pub fn summary(&self) -> ListingReport {
        ListingReport {
            listings: Vec::new(),
            ..self.clone()
        }
    }

    /// What `status` is: the report without its listings, each list of
    /// its row accounting ten unless `expand`, beside its whole count
    /// (C53, the read economy), and the snapshot's and the listings' ages
    /// its text says, in words at `now`, the answer's time (C53, ages).
    pub fn status(&self, expand: bool, now: i64) -> ListingReport {
        let mut status = self.summary();
        status.rows = status.rows.bounded(expand);
        let h = &mut status.header;
        let listing = |l: Option<ListingBasis>| Some(age::ago(now, l.map(|l| l.fetched_at)));
        h.taken_at_age = Some(age::ago(now, Some(h.taken_at)));
        h.stash_listing_fetched_at_age = listing(h.stash_listing);
        h.character_listing_fetched_at_age = listing(h.character_listing);
        status
    }

    /// `list`'s view under a filter. `None` when the filter names a
    /// container that is not a subject of these facts — a refusal, never
    /// an empty selection.
    pub fn list_view(&self, filter: ListFilter) -> Option<ListView> {
        for named in [&filter.r#in, &filter.covered_by].into_iter().flatten() {
            self.find(named)?;
        }
        let items: Vec<Listing> = self
            .listings
            .iter()
            .filter(|l| l.subject.is_item())
            // The default suppression of `none` applies only when neither
            // selector was given: `--effective none` must be able to select
            // exactly the items nothing applies to.
            .filter(|l| match (filter.relation, &filter.effective) {
                (Some(rel), _) => l.relation == rel,
                (None, Some(_)) => true,
                (None, None) => l.relation != Relation::None || l.effective.kind == "unresolved",
            })
            .filter(|l| {
                filter
                    .effective
                    .as_deref()
                    .is_none_or(|side| l.effective.side_word() == side)
            })
            .filter(|l| {
                filter
                    .r#in
                    .as_ref()
                    .is_none_or(|loc| l.subject.location.as_ref() == Some(loc))
            })
            .filter(|l| {
                filter
                    .covered_by
                    .as_ref()
                    .is_none_or(|t| l.chain.contains(t))
            })
            .cloned()
            .collect();
        let total = items.len();
        let mut counts = SetCounts::of(&items);
        let (items, left_out) = bounded(items, filter.limit, filter.expand);
        counts.returned = items.len();
        counts.left_out = left_out;
        // Every container on a listed item's chain — the one it sits
        // in and the ones above it — so a substash's parent, whose name
        // is the one read (C80), is in the document the text renders.
        let containers: Vec<Listing> = self
            .listings
            .iter()
            .filter(|c| !c.subject.is_item())
            .filter(|c| items.iter().any(|l| l.chain.contains(&c.subject.target)))
            .cloned()
            .collect();
        Some(ListView {
            header: self.header.clone(),
            filter,
            items_on_record: self.counts.items,
            total,
            listed: items.len(),
            counts,
            containers,
            items,
        })
    }

    /// `show`'s view of one target, if it is a subject of these facts.
    pub fn show_view(&self, target: &PriceTarget) -> Option<ShowView> {
        let listing = self.find(target)?.clone();
        let container = listing
            .subject
            .location
            .as_ref()
            .and_then(|loc| self.find(loc))
            .cloned();
        // The two sets belong to a container; an item has neither.
        let (items_here, items_covered_below) = if listing.subject.is_item() {
            (Vec::new(), Vec::new())
        } else {
            (
                self.items_in(target).cloned().collect(),
                self.items_covered_by(target)
                    .filter(|l| l.subject.location.as_ref() != Some(target))
                    .cloned()
                    .collect(),
            )
        };
        Some(ShowView {
            header: self.header.clone(),
            listing,
            container,
            items_here_counts: SetCounts::of(&items_here),
            items_here,
            items_covered_below_counts: SetCounts::of(&items_covered_below),
            items_covered_below,
        })
    }
}

/// Why a snapshot could not be resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListingError {
    /// The shipped currency table is not usable.
    Currency(CurrencyTableError),
    /// The snapshot's realm is not a word [`Realm`] knows.
    UnknownRealm { realm: String },
}

impl fmt::Display for ListingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListingError::Currency(e) => write!(f, "currency table: {e}"),
            ListingError::UnknownRealm { realm } => write!(f, "{realm:?} is not a realm"),
        }
    }
}

impl std::error::Error for ListingError {}

/// A stored row, read through the typed door (C66).
struct ManualRow {
    value: Buyout,
    revision: i64,
    updated_at: i64,
    written_via: String,
    actor: Option<String>,
}

impl ManualRow {
    /// The row as a manual side: on `from`, inherited or its own.
    fn side(&self, from: &PriceTarget, inherited: bool) -> ManualSide {
        ManualSide {
            value: self.value.clone(),
            from: from.clone(),
            inherited,
            revision: self.revision,
            updated_at: self.updated_at,
            updated_at_age: None,
            written_via: self.written_via.clone(),
            actor: self.actor.clone(),
        }
    }
}

/// The rows, by target, with what could not be indexed.
struct Rows {
    by_target: BTreeMap<PriceTarget, ManualRow>,
    /// Targets whose row exists but could not be read, with why.
    unreadable: BTreeMap<PriceTarget, String>,
    accounting: RowAccounting,
}

fn index_rows(
    rows: &[AnnotationRow],
    unparsed: &[acquisition_store::UnreadableRow],
    realm: Realm,
) -> Rows {
    let mut out = Rows {
        by_target: BTreeMap::new(),
        unreadable: BTreeMap::new(),
        accounting: RowAccounting {
            total: rows.len() + unparsed.len(),
            ..RowAccounting::default()
        },
    };
    // C151: a row the store could not parse is unreadable as a row the
    // kind refuses is — one path: its target's manual side is a problem
    // (the item resolves unresolved, C81, never falls through to the next
    // row up), and the accounting lists it with the parse fault as why.
    for row in unparsed {
        let problem = RowProblem {
            scope: row.scope.clone(),
            key: row.key.clone(),
            revision: row.revision,
            why: row.fault.clone(),
        };
        match PriceTarget::from_address(&row.scope, &row.key) {
            Err(_) => out.accounting.unreadable.push(problem),
            Ok(PriceTarget::Tab { realm: r, .. } | PriceTarget::Substash { realm: r, .. })
                if r != realm =>
            {
                out.accounting.other_realm += 1;
            }
            Ok(target) => {
                out.accounting.unreadable.push(problem);
                out.unreadable.insert(target, row.fault.clone());
            }
        }
    }
    for row in rows {
        let problem = |why: String| RowProblem {
            scope: row.scope.clone(),
            key: row.key.clone(),
            revision: row.revision,
            why,
        };
        let target = match PriceTarget::from_address(&row.scope, &row.key) {
            Ok(t) => t,
            Err(e) => {
                out.accounting.unreadable.push(problem(e.to_string()));
                continue;
            }
        };
        match &target {
            PriceTarget::Tab { realm: r, .. } | PriceTarget::Substash { realm: r, .. }
                if *r != realm =>
            {
                out.accounting.other_realm += 1;
                continue;
            }
            _ => {}
        }
        match check_value::<Buyout>(&row.value) {
            Ok(value) => {
                out.by_target.insert(
                    target,
                    ManualRow {
                        value,
                        revision: row.revision,
                        updated_at: row.updated_at,
                        written_via: row.written_via.clone(),
                        actor: row.actor.clone(),
                    },
                );
            }
            Err(e) => {
                let why = match &e {
                    ValueError::VersionUnsupported { .. } => {
                        format!("{e} — a newer build wrote it")
                    }
                    _ => e.to_string(),
                };
                out.accounting.unreadable.push(problem(why.clone()));
                out.unreadable.insert(target, why);
            }
        }
    }
    out
}

/// How a tab sits in the stash (the store's rule: a parent that is a
/// folder makes a folder child; any other parent makes a substash).
#[derive(Clone, Copy, PartialEq, Eq)]
enum TabKind {
    Folder,
    Top,
    FolderChild,
    Substash,
}

struct TabInfo<'a> {
    tab: &'a TabSnapshot,
    kind: TabKind,
    target: PriceTarget,
    /// The manual side's walk from this tab up (C70): itself, then its
    /// parent tab (a substash), then the folder.
    chain: Vec<PriceTarget>,
    /// The tab whose name and `public` the game side reads (C80).
    game_tab: Option<&'a TabSnapshot>,
}

fn tab_infos<'a>(tabs: &'a [TabSnapshot], realm: Realm) -> Vec<TabInfo<'a>> {
    let by_id: HashMap<&str, &TabSnapshot> = tabs.iter().map(|t| (t.id.as_str(), t)).collect();
    let tab_target = |id: &str| PriceTarget::Tab {
        realm,
        id: id.into(),
    };
    tabs.iter()
        .map(|tab| {
            let parent = tab.parent.as_deref().map(|p| (p, by_id.get(p).copied()));
            let kind = if tab.r#type == FOLDER {
                TabKind::Folder
            } else {
                match parent {
                    None => TabKind::Top,
                    Some((_, Some(p))) if p.r#type == FOLDER => TabKind::FolderChild,
                    Some(_) => TabKind::Substash,
                }
            };
            let (target, chain, game_tab) = match kind {
                TabKind::Substash => {
                    let (parent_id, parent_tab) = parent.unwrap_or(("", None));
                    let target = PriceTarget::Substash {
                        realm,
                        parent: parent_id.into(),
                        id: tab.id.clone(),
                    };
                    let mut chain = vec![target.clone(), tab_target(parent_id)];
                    if let Some(folder) = parent_tab.and_then(|p| p.parent.as_deref()) {
                        chain.push(tab_target(folder));
                    }
                    (target, chain, parent_tab)
                }
                TabKind::FolderChild => {
                    let target = tab_target(&tab.id);
                    let mut chain = vec![target.clone()];
                    if let Some((folder, _)) = parent {
                        chain.push(tab_target(folder));
                    }
                    (target, chain, Some(tab))
                }
                TabKind::Top => (tab_target(&tab.id), vec![tab_target(&tab.id)], Some(tab)),
                TabKind::Folder => (tab_target(&tab.id), vec![tab_target(&tab.id)], None),
            };
            TabInfo {
                tab,
                kind,
                target,
                chain,
                game_tab,
            }
        })
        .collect()
}

fn is_public(tab: &TabSnapshot) -> bool {
    tab.metadata
        .get("public")
        .and_then(serde_json::Value::as_bool)
        == Some(true)
}

/// Whether a text's reading is a statement the site would act on: a
/// price or `skip`.
fn speaks(reading: &GamePrice) -> bool {
    matches!(
        reading,
        GamePrice::Exact(_) | GamePrice::Negotiable(_) | GamePrice::Skip
    )
}

/// The tab half of a game side: the C80 tab's name read as a tab name,
/// and its `public`. The name is a statement only when the tab is
/// public (C81); otherwise it is residue at most.
fn tab_game_side(info: &TabInfo<'_>, table: &CurrencyTable) -> GameSide {
    let substash_name = (info.kind == TabKind::Substash).then(|| info.tab.name.clone());
    let mut side = GameSide {
        reading: GamePrice::None,
        source: None,
        note: None,
        tab: None,
        tab_name: None,
        substash_name,
        public: None,
        residue: false,
        note_unread: false,
    };
    if let Some(t) = info.game_tab {
        let reading = game_side::read(Source::TabName, &t.name, table);
        let public = is_public(t);
        if speaks(&reading) {
            if public {
                side.reading = reading.clone();
                side.source = Some(Source::TabName);
            } else {
                side.residue = true;
            }
        }
        side.tab = Some(t.id.clone());
        side.tab_name = Some(Reading {
            text: t.name.clone(),
            reading,
        });
        side.public = Some(public);
    }
    side
}

/// The note in front of the tab half (C69): a note that reads as a price
/// or `skip` is the statement where the tab is public; any other note
/// has no effect and the tab applies (T18). Where there is no public
/// tab the note is residue at most. A note the store could not read
/// states nothing here and is marked, so the effective price can say it
/// is unresolved by it (the module doc).
fn with_note(
    mut side: GameSide,
    note: Option<&str>,
    note_unread: bool,
    table: &CurrencyTable,
) -> GameSide {
    side.note_unread = note_unread;
    if let Some(text) = note {
        let reading = game_side::read(Source::Note, text, table);
        if speaks(&reading) {
            if side.public == Some(true) {
                side.reading = reading.clone();
                side.source = Some(Source::Note);
            } else {
                side.residue = true;
            }
        }
        side.note = Some(Reading {
            text: text.into(),
            reading,
        });
    }
    side
}

/// A row the walk stopped at because it cannot be read: where it sits
/// and why.
struct Unreadable {
    target: PriceTarget,
    why: String,
}

/// The manual side's walk (C70): the first row on the chain, or the
/// row that stopped it.
fn manual_side(chain: &[PriceTarget], rows: &Rows) -> (Option<ManualSide>, Option<Unreadable>) {
    for (i, target) in chain.iter().enumerate() {
        if let Some(row) = rows.by_target.get(target) {
            return (Some(row.side(target, i > 0)), None);
        }
        if let Some(why) = rows.unreadable.get(target) {
            return (
                None,
                Some(Unreadable {
                    target: target.clone(),
                    why: why.clone(),
                }),
            );
        }
    }
    (None, None)
}

/// The relation and its sentence (module doc, "The relation"). A row the
/// walk could not read is named in the sentence: "no row applies" would
/// be false, and the effective price says what the row could do.
fn relate(
    manual: Option<&Buyout>,
    unreadable: Option<&Unreadable>,
    game: &GamePrice,
) -> (Relation, String) {
    let unread = |tail: &str| match unreadable {
        Some(u) => format!("the row on {} cannot be read{tail}", u.target),
        None => format!("no row applies{tail}"),
    };
    match (manual, game) {
        (None, GamePrice::None) => (Relation::None, unread(" and the game states nothing")),
        (Some(m), GamePrice::None) => (
            Relation::ManualOnly,
            format!("by hand: {m}; the game states nothing"),
        ),
        (None, g) => (Relation::GameOnly, format!("in game: {g}; {}", unread(""))),
        (Some(m), g) => {
            let agree = match (m, g) {
                (Buyout::Exact(a), GamePrice::Exact(b)) => a == b,
                (Buyout::Negotiable(a), GamePrice::Negotiable(b)) => a == b,
                (Buyout::Skip, GamePrice::Skip) => true,
                _ => false,
            };
            if agree {
                (Relation::Agree, format!("by hand and in game: {g}"))
            } else {
                (Relation::Conflict, format!("by hand: {m}; in game: {g}"))
            }
        }
    }
}

/// The effective price (C81): the more specific statement, the game's on
/// a tie, each statement's level being its target's position on the
/// chain.
fn effective(
    chain: &[PriceTarget],
    manual: Option<&ManualSide>,
    unreadable: Option<&Unreadable>,
    game: &GameSide,
    subject: &PriceTarget,
) -> Effective {
    let level_of = |t: &PriceTarget| chain.iter().position(|c| c == t).unwrap_or(chain.len());
    let game_from: Option<PriceTarget> = match game.source {
        None => None,
        Some(Source::Note) => Some(subject.clone()),
        Some(Source::TabName) => {
            let id = game.tab.clone().unwrap_or_default();
            chain
                .iter()
                .find(|t| matches!(t, PriceTarget::Tab { id: i, .. } if *i == id))
                .cloned()
        }
    };
    let game_level = game_from.as_ref().map(level_of);
    let from_manual = |m: &ManualSide, why: String| Effective {
        kind: m.value.kind().into(),
        price: m.value.price().cloned(),
        side: Some(Side::Manual),
        from: Some(m.from.clone()),
        why,
    };
    let from_game = |why: String| Effective {
        kind: game.reading.kind().into(),
        price: game.reading.price().cloned(),
        side: Some(Side::Game),
        from: game_from.clone(),
        why,
    };
    let level_word = |t: &PriceTarget| match t {
        PriceTarget::Item { .. } => "item",
        PriceTarget::Substash { .. } => "substash",
        PriceTarget::Tab { .. } => "tab",
        PriceTarget::Character { .. } => "character",
    };
    // A row that cannot be read is a statement of unknown content: if it
    // could beat the game's statement, nothing is decided here.
    if let Some(u) = unreadable {
        let ul = level_of(&u.target);
        let could_decide = match game_level {
            None => true,
            Some(gl) => ul < gl,
        };
        if could_decide {
            return Effective {
                kind: "unresolved".into(),
                price: None,
                side: None,
                from: Some(u.target.clone()),
                why: format!(
                    "unresolved: the row on {} cannot be read ({}) and would decide{}",
                    u.target,
                    u.why,
                    match game_level {
                        None => "",
                        Some(_) => " over the game's less specific statement",
                    }
                ),
            };
        }
    }
    match (manual, game_level) {
        (None, None) => Effective {
            kind: "none".into(),
            price: None,
            side: None,
            from: None,
            why: if game.residue {
                "nothing applies: the price text in game is not where the index can see it".into()
            } else {
                "nothing applies".into()
            },
        },
        (Some(m), None) => from_manual(
            m,
            if game.residue {
                format!(
                    "{} by hand (row on {}); the price text in game is not where the index can see it",
                    m.value, m.from
                )
            } else {
                format!(
                    "{} by hand (row on {}); the game states nothing",
                    m.value, m.from
                )
            },
        ),
        (None, Some(_)) => from_game(match unreadable {
            Some(u) => format!(
                "{} in game: the game wins against the row on {}, readable or not, at its level",
                game.reading, u.target
            ),
            None => format!("{} in game; no row applies", game.reading),
        }),
        (Some(m), Some(gl)) => {
            let ml = level_of(&m.from);
            let gf = game_from.clone().unwrap_or_else(|| subject.clone());
            if ml < gl {
                from_manual(
                    m,
                    format!(
                        "{} by hand: the row on the {} is more specific than the game's {} name",
                        m.value,
                        level_word(&m.from),
                        level_word(&gf)
                    ),
                )
            } else if gl < ml {
                from_game(format!(
                    "{} in game: the {} is more specific than the row on the {}",
                    game.reading,
                    match game.source {
                        Some(Source::Note) => "note".to_string(),
                        _ => format!("{} name", level_word(&gf)),
                    },
                    level_word(&m.from)
                ))
            } else {
                from_game(format!(
                    "{} in game: the game wins a tie with the row on the {}",
                    game.reading,
                    level_word(&m.from)
                ))
            }
        }
    }
}

fn listing(
    subject: Subject,
    chain: &[PriceTarget],
    rows: &Rows,
    game: GameSide,
    basis: Basis,
) -> Listing {
    let (manual, unreadable) = manual_side(chain, rows);
    let (relation, why) = relate(
        manual.as_ref().map(|m| &m.value),
        unreadable.as_ref(),
        &game.reading,
    );
    let effective = if game.note_unread && game.public == Some(true) {
        // the note is the most specific statement there is (C81): one that
        // cannot be read could decide over anything — where the index
        // could see it at all
        Effective {
            kind: "unresolved".into(),
            price: None,
            side: None,
            from: Some(subject.target.clone()),
            why: format!(
                "unresolved: the note on {} cannot be read (it is no string) and would decide",
                subject.target
            ),
        }
    } else {
        effective(
            chain,
            manual.as_ref(),
            unreadable.as_ref(),
            &game,
            &subject.target,
        )
    };
    let why = if game.note_unread {
        format!("{why}; the note cannot be read")
    } else {
        why
    };
    let manual_problem =
        unreadable.map(|u| format!("the row on {} cannot be read: {}", u.target, u.why));
    Listing {
        subject,
        chain: chain.to_vec(),
        manual,
        manual_problem,
        game,
        relation,
        why,
        effective,
        basis,
    }
}

fn character_subject(c: &CharacterSnapshot) -> Subject {
    Subject {
        target: PriceTarget::Character { id: c.id.clone() },
        name: c.name.clone(),
        type_line: String::new(),
        stack_size: None,
        tab_type: None,
        location: None,
        container: None,
        socketed_in: None,
        x: None,
        y: None,
        inventory_id: None,
        index: None,
        league_unknown: c.league.is_none(),
    }
}

fn item_subject(item: &ItemSnapshot, location: PriceTarget, league_unknown: bool) -> Subject {
    Subject {
        target: PriceTarget::Item {
            id: item.id.clone(),
        },
        name: item.name.clone(),
        type_line: item.type_line.clone(),
        stack_size: item.stack_size,
        tab_type: None,
        location: Some(location),
        container: item.container.clone(),
        socketed_in: item.socketed_in.clone(),
        x: item.x,
        y: item.y,
        inventory_id: item.inventory_id.clone(),
        index: None,
        league_unknown,
    }
}

/// Resolve the listing state of one snapshot: every subject's two sides
/// and relation, the row accounting, the counts. Pure — the snapshot and
/// the shipped currency table in, the report out; the only failures are
/// a table this build cannot load and a realm word it does not know.
pub fn resolve(snapshot: &PricingSnapshot) -> Result<ListingReport, ListingError> {
    let table = currency::table().map_err(ListingError::Currency)?;
    let realm = Realm::parse(&snapshot.realm).ok_or_else(|| ListingError::UnknownRealm {
        realm: snapshot.realm.clone(),
    })?;
    let rows = index_rows(&snapshot.buyouts, &snapshot.unreadable_buyouts, realm);
    let tabs = tab_infos(&snapshot.tabs, realm);
    let tab_by_id: HashMap<&str, &TabInfo<'_>> =
        tabs.iter().map(|i| (i.tab.id.as_str(), i)).collect();
    let character_by_id: HashMap<&str, &CharacterSnapshot> = snapshot
        .characters
        .iter()
        .map(|c| (c.id.as_str(), c))
        .collect();

    let mut listings =
        Vec::with_capacity(tabs.len() + snapshot.characters.len() + snapshot.items.len());
    let mut counts = Counts::default();
    for info in &tabs {
        let subject = Subject {
            target: info.target.clone(),
            name: info.tab.name.clone(),
            type_line: String::new(),
            stack_size: None,
            tab_type: Some(info.tab.r#type.clone()),
            location: None,
            container: None,
            socketed_in: None,
            x: None,
            y: None,
            inventory_id: None,
            index: info.tab.idx,
            league_unknown: false,
        };
        let game = tab_game_side(info, table);
        // A name that reads as a price, public or not: the owner's
        // remove-only tabs are the case (census 2c), and the public
        // count beside it says how many the index can see.
        if info.kind != TabKind::Folder
            && info.kind != TabKind::Substash
            && game
                .tab_name
                .as_ref()
                .is_some_and(|t| t.reading.price().is_some())
        {
            counts.priced_tabs += 1;
            if game.public == Some(true) {
                counts.priced_tabs_public += 1;
            }
        }
        let basis = Basis {
            response: info.tab.listed_response,
            at: info.tab.listed_at,
            at_age: None,
        };
        listings.push(listing(subject, &info.chain, &rows, game, basis));
    }
    for c in &snapshot.characters {
        let subject = character_subject(c);
        let chain = [subject.target.clone()];
        let game = GameSide {
            reading: GamePrice::None,
            source: None,
            note: None,
            tab: None,
            tab_name: None,
            substash_name: None,
            public: None,
            residue: false,
            note_unread: false,
        };
        let basis = Basis {
            response: c.listed_response,
            at: c.listed_at,
            at_age: None,
        };
        listings.push(listing(subject, &chain, &rows, game, basis));
    }
    counts.containers = listings.len();
    for item in &snapshot.items {
        let own = PriceTarget::Item {
            id: item.id.clone(),
        };
        let (location, chain, game) = if item.location_kind == "character" {
            let location = PriceTarget::Character {
                id: item.location_id.clone(),
            };
            let chain = vec![own, location.clone()];
            let game = with_note(
                GameSide {
                    reading: GamePrice::None,
                    source: None,
                    note: None,
                    tab: None,
                    tab_name: None,
                    substash_name: None,
                    public: None,
                    residue: false,
                    note_unread: false,
                },
                item.note.as_deref(),
                item.note_unread,
                table,
            );
            (location, chain, game)
        } else {
            match tab_by_id.get(item.location_id.as_str()) {
                Some(info) => {
                    let mut chain = Vec::with_capacity(info.chain.len() + 1);
                    chain.push(own);
                    chain.extend(info.chain.iter().cloned());
                    let game = with_note(
                        tab_game_side(info, table),
                        item.note.as_deref(),
                        item.note_unread,
                        table,
                    );
                    (info.target.clone(), chain, game)
                }
                None => {
                    // The snapshot's items are at its tabs by construction;
                    // an item that is not is reported, never dropped.
                    let location = PriceTarget::Tab {
                        realm,
                        id: item.location_id.clone(),
                    };
                    let chain = vec![own, location.clone()];
                    let game = with_note(
                        GameSide {
                            reading: GamePrice::None,
                            source: None,
                            note: None,
                            tab: None,
                            tab_name: None,
                            substash_name: None,
                            public: None,
                            residue: false,
                            note_unread: false,
                        },
                        item.note.as_deref(),
                        item.note_unread,
                        table,
                    );
                    (location, chain, game)
                }
            }
        };
        let league_unknown = item.location_kind == "character"
            && character_by_id
                .get(item.location_id.as_str())
                .is_some_and(|c| c.league.is_none());
        let subject = item_subject(item, location, league_unknown);
        let basis = Basis {
            response: item.seen_response,
            at: Some(item.last_seen),
            at_age: None,
        };
        let l = listing(subject, &chain, &rows, game, basis);
        counts.items += 1;
        if league_unknown {
            counts.league_unknown += 1;
        }
        *counts
            .by_relation
            .entry(l.relation.to_string())
            .or_default() += 1;
        *counts
            .by_game_statement
            .entry(l.game.reading.kind().to_string())
            .or_default() += 1;
        if l.game
            .note
            .as_ref()
            .is_some_and(|n| matches!(n.reading, GamePrice::Invalid { .. }))
        {
            counts.invalid_notes += 1;
        }
        if l.game.reading.price().is_some() {
            counts.game_priced += 1;
        }
        if l.manual.as_ref().is_some_and(|m| m.inherited) {
            counts.inherited += 1;
        }
        *counts
            .by_effective
            .entry(l.effective.side_word().to_string())
            .or_default() += 1;
        if l.game.residue {
            counts.residue += 1;
        }
        listings.push(l);
    }
    for relation in Relation::ALL {
        counts.by_relation.entry(relation.to_string()).or_default();
    }
    for side in ["game", "manual", "none", "unresolved"] {
        counts.by_effective.entry(side.into()).or_default();
    }

    let mut accounting = rows.accounting;
    let subjects: std::collections::BTreeSet<&PriceTarget> =
        listings.iter().map(|l| &l.subject.target).collect();
    for (target, row) in &rows.by_target {
        if subjects.contains(target) {
            accounting.applied += 1;
        } else {
            accounting.unmatched.push(row.side(target, false));
        }
    }
    accounting.counted();

    Ok(ListingReport {
        header: ReportHeader {
            schema: LISTING_SCHEMA,
            account_uuid: snapshot.account_uuid.clone(),
            account_name: snapshot.account_name.clone(),
            realm,
            league: snapshot.league.clone(),
            taken_at: snapshot.taken_at,
            taken_at_age: None,
            stash_listing_fetched_at_age: None,
            character_listing_fetched_at_age: None,
            stash_listing: snapshot.stash_listing,
            character_listing: snapshot.character_listing,
            note_parser_version: NOTE_PARSER_VERSION,
            currency_table_version: CURRENCY_TABLE_VERSION,
        },
        counts,
        rows: accounting,
        listings,
    })
}

#[cfg(test)]
#[path = "../unit/tests/listing.rs"]
mod tests;
