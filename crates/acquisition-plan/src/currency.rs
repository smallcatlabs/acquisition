//! The currency reference table — the first reference-data input (C68;
//! `decisions/pricing.md`), built by the pricing slice's plan step 1
//! (`PRICING-SLICE.md` at `b6197a82`, 2026-09-04).
//!
//! # Decisions as recorded
//!
//! **C68 — Reference data is a fifth input, a versioned, reviewed,
//! committed table whose rows cite evidence, shipped inside the binary,
//! read-only, never in a store file, enumerable by every surface built,
//! cited by version when used.** A tool may propose rows from a governed
//! source (C79); a human commits. The currency table is first: the
//! immutable `tag` intent cites; `emit`, the word GGG's client writes
//! into a note; `aliases`, the legacy C++ tag and shorthand the owner
//! checked by hand, each cited — the indexer's loose matching is not
//! modelled. A tag is never removed or reused; a dropped currency keeps
//! its row, marked; additions are reported. *Why:* not a fact, not
//! intent, not a derivation (pattern 8); the C++ list rotted without
//! evidence. Amended 2026-09-05.
//!
//! # As built
//!
//! The table is `reference/currency-v1.toml` beside this crate, compiled
//! into the binary by `include_str!` and parsed once, on first use
//! ([`table`]); the file's header states the row rules and the evidence
//! grammar, and is the human-reviewed artifact — this module only reads
//! it. Parsing is strict (`deny_unknown_fields`) and the loader checks
//! what a reviewer would: the version stamp is the one this build
//! expects, every `tag`, `emit` and alias is a single word, every word
//! resolves to exactly one row across the whole table, every row cites
//! at least one `game:` evidence entry, and a row with aliases cites
//! where they came from (`cpp:` or `browser:`). A table that fails is a
//! build defect surfaced as [`CurrencyTableError`] to every caller (the
//! crate's no-panic ratchet applies; the `c68_` tests pin the shipped
//! file), never a partial table.
//!
//! [`CurrencyTable::resolve`] is the one lookup: exact, case-sensitive,
//! over tag, emit and aliases. A retired row still resolves — intent may
//! cite it forever — and says so through [`Currency::retired`]; what a
//! consumer does with a retired currency (refuse a new price, render an
//! old one) is that consumer's rule. Enumeration is [`CurrencyTable::rows`]
//! in the file's order (the in-game dialog's, then the retired rows),
//! and every surface that shows a currency cites [`CurrencyTable::version`].
//!
//! v1 (2026-09-04): 39 active rows — the words the in-game price dialog
//! writes, with the display names the owner read in it — and three
//! retired (`chisel`, `coin`, `silver`: the C++ table had them, the dialog
//! no longer offers them). The four C++ tags that differed from the game's
//! word (`exa`, `chrom`, `jew`, `fuse`) are aliases of `exalted`, `chrome`,
//! `jewellers`, `fusing`; the owner's shorthand (`alts`, `mirr`, …) joins
//! them row by row as he checks each by hand — the count is not pinned.

use std::collections::BTreeMap;
use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

use acquisition_protocol::realm::Realm;

/// The table this build ships, verbatim (the reviewed file).
pub const CURRENCY_TABLE_TOML: &str = include_str!("../reference/currency-v1.toml");

/// The table version this build expects; a file stamped otherwise is a
/// build defect, not a runtime condition.
pub const CURRENCY_TABLE_VERSION: u32 = 1;

/// One currency: the identity intent cites, the word the game writes, the
/// name a human reads, and the evidence for each.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Currency {
    /// Immutable identity (C67 cites it). Never removed, never reused.
    pub tag: String,
    /// The name the in-game price dialog shows.
    pub display: String,
    /// The word GGG's client writes into a note after the amount — and so
    /// the word a render writes.
    pub emit: String,
    /// Extra words that resolve to this row beyond `tag` and `emit`: the
    /// legacy C++ tag where it differed, nothing more.
    #[serde(default)]
    pub aliases: Vec<String>,
    /// Realm applicability.
    pub realms: Vec<Realm>,
    /// Set when the game no longer offers the currency: when and on what
    /// evidence. The row stays; intent may still cite it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retired: Option<String>,
    /// One entry per source, in the file header's grammar; at least one
    /// `game:` entry per row.
    pub evidence: Vec<String>,
}

impl Currency {
    /// Every word that resolves to this row: tag, emit, then aliases.
    pub fn words(&self) -> impl Iterator<Item = &str> {
        std::iter::once(self.tag.as_str())
            .chain(std::iter::once(self.emit.as_str()))
            .chain(self.aliases.iter().map(String::as_str))
    }

    pub fn is_retired(&self) -> bool {
        self.retired.is_some()
    }
}

/// The parsed, checked table. Read-only; one per build.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CurrencyTable {
    version: u32,
    status: String,
    sources: Vec<String>,
    #[serde(rename = "currency", default)]
    rows: Vec<Currency>,
}

/// Why a table text is not a table. Every arm names the offending row or
/// word so a reviewer can fix the file, not the loader.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CurrencyTableError {
    /// The text is not the TOML shape the loader reads.
    Parse(String),
    /// The file's `version` is not [`CURRENCY_TABLE_VERSION`].
    Version { found: u32, expected: u32 },
    /// A tag, emit or alias is empty or holds whitespace.
    MalformedWord { tag: String, word: String },
    /// One word would resolve to two rows.
    AmbiguousWord { word: String, tags: [String; 2] },
    /// A row cites no `game:` evidence (or none at all).
    Uncited { tag: String },
    /// A row has aliases but cites neither `cpp:` nor `browser:` for them.
    UncitedAliases { tag: String },
    /// A row names no realm.
    NoRealm { tag: String },
    /// The table has no rows.
    Empty,
}

impl std::fmt::Display for CurrencyTableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrencyTableError::Parse(e) => write!(f, "currency table does not parse: {e}"),
            CurrencyTableError::Version { found, expected } => write!(
                f,
                "currency table is version {found}; this build reads version {expected}"
            ),
            CurrencyTableError::MalformedWord { tag, word } => {
                write!(f, "currency {tag:?}: {word:?} is not a single word")
            }
            CurrencyTableError::AmbiguousWord { word, tags } => write!(
                f,
                "currency word {word:?} resolves to both {:?} and {:?}",
                tags[0], tags[1]
            ),
            CurrencyTableError::Uncited { tag } => {
                write!(f, "currency {tag:?} cites no `game:` evidence")
            }
            CurrencyTableError::UncitedAliases { tag } => write!(
                f,
                "currency {tag:?} has aliases but cites no `cpp:` or `browser:` source for them"
            ),
            CurrencyTableError::NoRealm { tag } => write!(f, "currency {tag:?} names no realm"),
            CurrencyTableError::Empty => write!(f, "currency table has no rows"),
        }
    }
}

impl std::error::Error for CurrencyTableError {}

impl CurrencyTable {
    /// Parse and check a table text. The shipped file goes through
    /// [`table`]; this is the same path over any text, for the tests
    /// that pin what the loader refuses.
    pub fn parse(text: &str) -> Result<CurrencyTable, CurrencyTableError> {
        let table: CurrencyTable =
            toml::from_str(text).map_err(|e| CurrencyTableError::Parse(e.to_string()))?;
        table.check()?;
        Ok(table)
    }

    fn check(&self) -> Result<(), CurrencyTableError> {
        if self.version != CURRENCY_TABLE_VERSION {
            return Err(CurrencyTableError::Version {
                found: self.version,
                expected: CURRENCY_TABLE_VERSION,
            });
        }
        if self.rows.is_empty() {
            return Err(CurrencyTableError::Empty);
        }
        let mut seen: BTreeMap<&str, &str> = BTreeMap::new();
        for row in &self.rows {
            for word in row.words() {
                if word.is_empty() || word.chars().any(char::is_whitespace) {
                    return Err(CurrencyTableError::MalformedWord {
                        tag: row.tag.clone(),
                        word: word.to_string(),
                    });
                }
                // A row's own tag and emit are one word today; the same
                // row naming a word twice is not an ambiguity.
                if let Some(other) = seen.insert(word, &row.tag)
                    && other != row.tag
                {
                    return Err(CurrencyTableError::AmbiguousWord {
                        word: word.to_string(),
                        tags: [other.to_string(), row.tag.clone()],
                    });
                }
            }
            if !row.evidence.iter().any(|e| e.starts_with("game:")) {
                return Err(CurrencyTableError::Uncited {
                    tag: row.tag.clone(),
                });
            }
            if !row.aliases.is_empty()
                && !row
                    .evidence
                    .iter()
                    .any(|e| e.starts_with("cpp:") || e.starts_with("browser:"))
            {
                return Err(CurrencyTableError::UncitedAliases {
                    tag: row.tag.clone(),
                });
            }
            if row.realms.is_empty() {
                return Err(CurrencyTableError::NoRealm {
                    tag: row.tag.clone(),
                });
            }
        }
        Ok(())
    }

    /// The version every surface cites beside a currency.
    pub fn version(&self) -> u32 {
        self.version
    }

    /// The file's review status, verbatim ("draft" until the owner has
    /// read it). Exposed, gated on by nothing.
    pub fn status(&self) -> &str {
        &self.status
    }

    /// The table-level sources, verbatim.
    pub fn sources(&self) -> &[String] {
        &self.sources
    }

    /// Every row in the file's order: the in-game dialog's, then retired.
    pub fn rows(&self) -> &[Currency] {
        &self.rows
    }

    /// The rows the game still offers.
    pub fn active(&self) -> impl Iterator<Item = &Currency> {
        self.rows.iter().filter(|r| !r.is_retired())
    }

    /// The one lookup: exact and case-sensitive over tag, emit and aliases.
    /// A retired row resolves and says so.
    pub fn resolve(&self, word: &str) -> Option<&Currency> {
        self.rows.iter().find(|r| r.words().any(|w| w == word))
    }

    /// A row by its immutable tag only (what intent cites).
    pub fn by_tag(&self, tag: &str) -> Option<&Currency> {
        self.rows.iter().find(|r| r.tag == tag)
    }
}

static TABLE: LazyLock<Result<CurrencyTable, CurrencyTableError>> =
    LazyLock::new(|| CurrencyTable::parse(CURRENCY_TABLE_TOML));

/// The table this build ships, parsed and checked once. An error here is
/// a build defect (the file failed the loader's review checks); callers
/// surface it, they never guess a row.
pub fn table() -> Result<&'static CurrencyTable, CurrencyTableError> {
    TABLE.as_ref().map_err(Clone::clone)
}

#[cfg(test)]
mod tests;
