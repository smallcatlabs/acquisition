//! The game side of a listing (C69): what an item's `note` or a tab's
//! name says, read as one pure function. Built by the pricing slice's
//! plan step 3 (`PRICING-SLICE.md` at `b6197a82`, 2026-09-06). Nothing here reads a
//! store; the listing state (`listing.rs`, plan step 4) calls [`read`]
//! once per note and once per tab name and keeps the raw text beside
//! the result.
//!
//! # Decisions as recorded
//!
//! **C69 — A listing is 2 independent resolutions, their relation, and the
//! effective price (C81).** The manual side resolves by specificity (C70);
//! the game side reads note, then tab name, as a price (exact or
//! negotiable; a ratio too, T2) or `skip` (T10); any other text — free
//! text, an empty amount, an unknown word, a ratio in a tab name — has no
//! effect (T11, T16, T18, T19), is shown verbatim, and the tab applies.
//! The relation is manual-only, game-only, agree, conflict or none. Each
//! result carries both sides with causes, revisions, basis, versions, the
//! raw texts. *Why:* four statements C++ fused and needed locks; the
//! site's display states are not modelled. *Details:* `game_side.rs`,
//! `listing.rs`. *Pinned:* the `c69_` tests. Amended 2026-09-06.
//!
//! # As built
//!
//! **The grammar is the game's, exactly** (T10: "the prefix, one space,
//! the amount, one space, the word — that is the whole grammar the game
//! wrote"). `~price <amount> <word>` is [`GamePrice::Exact`], `~b/o
//! <amount> <word>` is [`GamePrice::Negotiable`], `~skip` is
//! [`GamePrice::Skip`]; the amount is [`Amount`]'s grammar (a decimal of
//! at most four places, or `wanted/lot`, T2) and the word is any word
//! the shipped currency table resolves — tag, emit or a cited alias
//! (C68) — the price carrying the row's **tag**, so a hand-typed `~price
//! 5 exa` and the game's `~price 5 exalted` are one price. A retired
//! word still reads as a price: the table says it is retired, the
//! consumer decides what that means.
//!
//! **A leading `~` opens a price note.** Text that does not start with
//! `~` is [`GamePrice::None`]: not a price, whatever it says (the 0.18
//! corpus's one such note is `50`). Text that starts with `~` and is not
//! the grammar is [`GamePrice::Invalid`] naming why: the game's own
//! residue `~price  <word>` (an empty amount, two spaces — what the
//! dialog leaves after an invalid entry, T10), a ratio in a tab name
//! (T11: it unlists the whole tab), an amount [`Amount`] refuses (a fifth
//! decimal, a zero, a sign), a word the table does not know (spelled
//! the table's way, case-sensitive — the indexer's loose matching is not
//! modelled, C68), and a prefix the game does not write (`~c/o`, `~gb/o`:
//! the C++ app's, never observed in a note or on the site). A misread is
//! one more fixture line (`reference/price-notes-2026-09-04.txt`), never
//! a special case.
//!
//! **Text after the word is tolerated by both sources, not read.** A tab
//! name carries the game's own `(Remove-only)` and the owner's letters
//! `(A)`…`(G)` (T11). A note carries whatever was typed after the word:
//! the game stores it whole and its dialog displays only the parsed
//! part, the API serves the whole, and the trade site reads the price
//! out of it and labels the listing "Price with Note" — the owner's
//! `~price 777 chaos testing`, read by all three the same way on
//! 2026-09-08 (`PRICING-SLICE.md` at `b6197a82`, "What the site taught", 5). Parser v1
//! refused such a note on the dialog's display alone (T17 as first
//! read); v2 reads it as the site does, the text still shown verbatim.
//! **The two sources differ in one rule** ([`Source`]): a ratio is a
//! price in a note (T2) but invalid in a tab name (T11). Trailing
//! whitespace is trimmed from both before reading (`~skip ` is how the
//! game writes skip); leading whitespace and text are not tolerated —
//! the marker comes first or the text is not a price note.
//!
//! **Nothing is decided here.** `~skip` in a tab name, `~b/o` with a
//! ratio, a retired currency, a price on a non-public tab, which tab a
//! nested item's game side reads (C80: a substash its parent, a folder
//! child itself), whether an `invalid` reading has any effect (it has
//! none: the tab applies, T18, and the listing state says so): the parser
//! reports what one text says under [`NOTE_PARSER_VERSION`], and every
//! consumer (the listing state, the render's policy table) cites that
//! version beside its own rule (C74). The indexer's loose word matching
//! (T16: `exa`, `exal`) is not modelled — an unknown word reads `invalid`
//! and is shown; an alias row is the fix when one matters.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::currency::CurrencyTable;
use crate::price::{Amount, Price};

/// The version of this grammar, cited by every result that carries a
/// game-side reading (C69). Bumped when a reading of the same text
/// changes.
pub const NOTE_PARSER_VERSION: u32 = 2;

/// Where a text came from; the two sources read under one differing rule
/// (module doc, "The two sources differ in one rule").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    /// An item's `note`, as the API returns it.
    Note,
    /// A stash tab's `name`.
    TabName,
}

impl Source {
    /// The word a result is labelled with.
    pub fn as_str(self) -> &'static str {
        match self {
            Source::Note => "note",
            Source::TabName => "tab_name",
        }
    }
}

/// What a note or a tab name says about a price: C69's four outcomes,
/// the price split by its prefix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum GamePrice {
    /// `~price <amount> <word>`.
    Exact(Price),
    /// `~b/o <amount> <word>`.
    Negotiable(Price),
    /// `~skip`: the dialog's "Do not index" (T10).
    Skip,
    /// A `~` text that is not the grammar; `why` names the reason.
    Invalid { why: String },
    /// Not a price note: no leading `~`.
    None,
}

impl GamePrice {
    /// The price, for the two outcomes that carry one.
    pub fn price(&self) -> Option<&Price> {
        match self {
            GamePrice::Exact(p) | GamePrice::Negotiable(p) => Some(p),
            GamePrice::Skip | GamePrice::Invalid { .. } | GamePrice::None => None,
        }
    }

    /// The outcome word: `exact`, `negotiable`, `skip`, `invalid`, `none`.
    pub fn kind(&self) -> &'static str {
        match self {
            GamePrice::Exact(_) => "exact",
            GamePrice::Negotiable(_) => "negotiable",
            GamePrice::Skip => "skip",
            GamePrice::Invalid { .. } => "invalid",
            GamePrice::None => "none",
        }
    }
}

impl fmt::Display for GamePrice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GamePrice::Exact(p) => write!(f, "{p}"),
            GamePrice::Negotiable(p) => write!(f, "{p} b/o"),
            GamePrice::Skip => write!(f, "skip"),
            GamePrice::Invalid { why } => write!(f, "invalid: {why}"),
            GamePrice::None => write!(f, "none"),
        }
    }
}

/// The prefixes the game writes (T10), with the outcome each opens.
const EXACT: &str = "~price";
const NEGOTIABLE: &str = "~b/o";
const SKIP: &str = "~skip";

/// Read one note or tab name. Pure: the text, its source and the shipped
/// currency table in; one [`GamePrice`] out. Never fails — a text this
/// cannot read is an outcome, not an error.
pub fn read(source: Source, text: &str, table: &CurrencyTable) -> GamePrice {
    let text = text.trim_end();
    if !text.starts_with('~') {
        return GamePrice::None;
    }
    let invalid = |why: String| GamePrice::Invalid { why };

    // The prefix is the text up to the first space (or all of it).
    let (prefix, rest) = match text.split_once(' ') {
        Some((p, r)) => (p, Some(r)),
        None => (text, None),
    };
    if prefix == SKIP {
        return match rest {
            None => GamePrice::Skip,
            Some(more) => match source {
                Source::TabName => GamePrice::Skip,
                Source::Note => invalid(format!("`{SKIP}` takes nothing; found {more:?} after it")),
            },
        };
    }
    let negotiable = match prefix {
        EXACT => false,
        NEGOTIABLE => true,
        other => {
            return invalid(format!(
                "{other:?} is not a prefix the game writes ({EXACT}, {NEGOTIABLE}, {SKIP})"
            ));
        }
    };

    // `<amount> <word>`, then nothing (a note) or anything (a tab name).
    let Some(rest) = rest else {
        return invalid(format!(
            "`{prefix}` takes `<amount> <word>`; found nothing after it"
        ));
    };
    let Some((amount_text, after_amount)) = rest.split_once(' ') else {
        return invalid(format!(
            "`{prefix}` takes `<amount> <word>`; found only {rest:?} after it"
        ));
    };
    if amount_text.is_empty() {
        return invalid(
            "the amount is empty — what the dialog leaves after an invalid entry".into(),
        );
    }
    let amount = match amount_text.parse::<Amount>() {
        Ok(a) => a,
        Err(e) => return invalid(e.to_string()),
    };
    if source == Source::TabName && matches!(amount, Amount::Ratio { .. }) {
        return invalid(format!(
            "a ratio ({amount_text}) in a tab name is invalid and lists nothing"
        ));
    }
    // The suffix after the word is tolerated, not read, under both
    // sources (module doc); the word is what stands before the first
    // whitespace.
    let word = after_amount
        .split_once(char::is_whitespace)
        .map_or(after_amount, |(word, _suffix)| word);
    if word.is_empty() {
        return invalid(format!("`{prefix} {amount_text}` names no currency word"));
    }
    let Some(row) = table.resolve(word) else {
        return invalid(format!(
            "{word:?} is not a word of currency table v{} (tag, emit or alias)",
            table.version()
        ));
    };
    let price = Price {
        amount,
        currency: row.tag.clone(),
    };
    if negotiable {
        GamePrice::Negotiable(price)
    } else {
        GamePrice::Exact(price)
    }
}

#[cfg(test)]
mod tests;
