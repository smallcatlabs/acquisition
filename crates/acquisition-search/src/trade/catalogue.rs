//! The trade catalogue (C113, `decisions/trade.md`; K6, taken by `translate.rs`; step 0 of
//! `search/TRADE-PLAN.md`, its loader step 3's): one row per clause the
//! site's query can carry, with its status, the text it lands as and the
//! note the report prints — what the translation reads
//! ([`super::translate`], step 4) and `--describe trade` prints (step 5).
//!
//! # As built
//!
//! - **The table is `reference/trade-catalogue-v1.toml`** beside this
//!   crate, written by hand and reviewed as a diff, compiled in by
//!   `include_str!` and parsed once, on first use ([`catalogue`]). The
//!   file's header says what a row means; this loader holds it to that.
//! - **The loader checks what a reviewer would**: the version this build
//!   expects ([`CATALOGUE_VERSION`]) and the dates of the two captures it
//!   was written from, the stat table's among them
//!   ([`stats::STAT_TABLE_VERSION`]); every key once and in the shape the
//!   header gives; a status of the four, a shape of the five; a note
//!   beside every clause that runs as ambiguous or is unsupported, and
//!   none empty; nothing landed by a clause that does not run; every
//!   placeholder one the translation fills ([`PLACEHOLDERS`]), and none
//!   inside quotes, where the printer's quoting of its value cannot reach
//!   (step 4); every field
//!   named written in the row's text, and **every field named one this
//!   build knows** — a field of the language, a property field
//!   (`properties.rs`, C114), a computed value, or the `is:` word — so a
//!   catalogue naming a field the build lacks is a build defect, said
//!   here, on the build that writes the fields. A table that fails is a
//!   [`CatalogueError`] to whoever asks, never a partial one; the corpus
//!   never loads it, so an ordinary search neither pays for it nor fails
//!   by it.
//! - **That every site filter, stat category and group type has a row**
//!   is the captures' to say, and `tests/trade_catalogue.rs` walks them.
//! - **The version is the basis's** from the step whose answer reads the
//!   catalogue (step 4): only an answer that used it cites it.

use std::collections::HashSet;
use std::sync::LazyLock;

use serde::Deserialize;

use super::stats;

/// The catalogue this build ships, verbatim (the reviewed file).
pub const CATALOGUE_TOML: &str = include_str!("../../reference/trade-catalogue-v1.toml");

/// The catalogue version this build expects and every basis that reads it
/// cites. A file stamped otherwise is a build defect.
pub const CATALOGUE_VERSION: u32 = 1;

/// The date of the filters capture the catalogue was written from
/// (`search/trade-query/data/filters-2026-09-12.json`).
pub const FILTERS_CAPTURE: &str = "2026-09-12";

/// What a row's `lands` may be filled with (the file's header): each a
/// part of a clause the translation reads (`translate.rs`, which fills
/// every one).
pub const PLACEHOLDERS: [&str; 9] = [
    "value", "text", "option", "r", "g", "b", "w", "bounds", "total",
];

/// A clause's status when it runs (K6). `disabled`, the fifth, is the
/// site's own mark on a clause, never a row's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Disposition {
    Exact,
    Ambiguous,
    Unsupported,
    ListingOnly,
}

impl Disposition {
    const ALL: [(&'static str, Disposition); 4] = [
        ("exact", Disposition::Exact),
        ("ambiguous", Disposition::Ambiguous),
        ("unsupported", Disposition::Unsupported),
        ("listing_only", Disposition::ListingOnly),
    ];

    /// The word the file and the report spell it with.
    pub fn word(self) -> &'static str {
        Disposition::ALL
            .iter()
            .find(|(_, d)| *d == self)
            .map_or("", |(w, _)| w)
    }

    /// Whether a clause of this status runs.
    pub fn runs(self) -> bool {
        matches!(self, Disposition::Exact | Disposition::Ambiguous)
    }
}

/// The site's control for a filter, as the filters capture shapes it.
pub const SHAPES: [&str; 5] = ["option", "input", "minmax", "sockets", "minmax_option"];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    version: u32,
    filters: String,
    stats: String,
    #[serde(rename = "row", default)]
    rows: Vec<RowIn>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RowIn {
    key: String,
    text: Option<String>,
    shape: Option<String>,
    disposition: String,
    lands: Option<String>,
    #[serde(default)]
    fields: Vec<String>,
    lacked: Option<String>,
    note: Option<String>,
    says: Option<String>,
}

/// One clause of the site's query, as the catalogue states it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub key: String,
    /// The site's own label for it, where it has one.
    pub text: Option<String>,
    pub shape: Option<String>,
    pub disposition: Disposition,
    /// The text it becomes, with its placeholders.
    pub lands: Option<String>,
    pub fields: Vec<String>,
    /// An item lacking the row's one field meets a maximum alone (the
    /// file's header: `lacked = "meets_maximum"`).
    pub lacked_meets_maximum: bool,
    /// The row's evidence, its citations among it: the catalogue file's
    /// record, which neither the report nor `--describe trade` prints.
    pub note: Option<String>,
    /// What the report says beside the clause, in a reader's words: what
    /// it becomes and where it may part from the site, no citation (the
    /// trade seat, 2026-10-17: every run met the notes' records, `S26`,
    /// `t55`, a file's path, and could not read them). Given wherever a
    /// note is.
    pub says: Option<String>,
}

/// The parsed, checked catalogue. Read-only; one per build.
#[derive(Debug)]
pub struct Catalogue {
    version: u32,
    rows: Vec<Row>,
}

/// Why a catalogue text is not a catalogue. Every arm names the offending
/// row so a reviewer can fix the file, not the loader.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogueError {
    Parse(String),
    Version {
        found: u32,
        expected: u32,
    },
    /// A capture's date that is not the one this build's tables are of.
    Capture {
        which: &'static str,
        found: String,
        expected: &'static str,
    },
    NoRow,
    BadKey(String),
    DuplicateKey(String),
    UnknownDisposition {
        key: String,
        disposition: String,
    },
    UnknownShape {
        key: String,
        shape: String,
    },
    /// An ambiguous or unsupported clause with nothing for the report to
    /// say beside it.
    NoNote(String),
    /// A note with no reader's sentence beside it.
    NoSays(String),
    /// A text, a note or a landing given and empty.
    Empty {
        key: String,
        what: &'static str,
    },
    /// A clause that does not run, landing a text.
    LandsWithoutRunning(String),
    UnknownPlaceholder {
        key: String,
        placeholder: String,
    },
    /// A placeholder written inside quotes, where the printer's quoting of
    /// what fills it cannot reach.
    QuotedPlaceholder {
        key: String,
        placeholder: String,
    },
    /// Fields named by a row that lands no text.
    FieldsWithoutLands(String),
    /// A field named that the row's text does not write.
    FieldNotLanded {
        key: String,
        field: String,
    },
    /// A field named that this build does not know.
    UnknownField {
        key: String,
        field: String,
    },
    /// A `lacked` that is not `meets_maximum`, or on a row whose text is
    /// not one field's bounds.
    Lacked(String),
}

impl std::fmt::Display for CatalogueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CatalogueError::Parse(e) => write!(f, "the trade catalogue does not parse: {e}"),
            CatalogueError::Version { found, expected } => write!(
                f,
                "the trade catalogue is version {found}; this build reads version {expected}"
            ),
            CatalogueError::Capture {
                which,
                found,
                expected,
            } => write!(
                f,
                "the trade catalogue is of the {which} capture of {found}; this build's is of {expected}"
            ),
            CatalogueError::NoRow => write!(f, "the trade catalogue has no row"),
            CatalogueError::BadKey(key) => write!(
                f,
                "the trade catalogue has the key {key:?}, which is not lowercase words of letters, digits and `_` joined by `.`, with one `=<option>` at most"
            ),
            CatalogueError::DuplicateKey(key) => {
                write!(f, "the trade catalogue has the key {key:?} twice")
            }
            CatalogueError::UnknownDisposition { key, disposition } => write!(
                f,
                "the trade catalogue gives {key:?} the status {disposition:?}, which is none of exact, ambiguous, unsupported, listing_only"
            ),
            CatalogueError::UnknownShape { key, shape } => write!(
                f,
                "the trade catalogue gives {key:?} the shape {shape:?}, which is none of {}",
                SHAPES.join(", ")
            ),
            CatalogueError::NoNote(key) => write!(
                f,
                "the trade catalogue gives {key:?} no note, which the report prints beside an ambiguous or unsupported clause"
            ),
            CatalogueError::NoSays(key) => write!(
                f,
                "the trade catalogue gives {key:?} a note and no `says`, the sentence the report prints in its place"
            ),
            CatalogueError::Empty { key, what } => {
                write!(f, "the trade catalogue gives {key:?} an empty {what}")
            }
            CatalogueError::LandsWithoutRunning(key) => write!(
                f,
                "the trade catalogue has {key:?} land a text, and a clause of its status never runs"
            ),
            CatalogueError::UnknownPlaceholder { key, placeholder } => write!(
                f,
                "the trade catalogue's {key:?} lands {{{placeholder}}}, which the translation does not fill"
            ),
            CatalogueError::QuotedPlaceholder { key, placeholder } => write!(
                f,
                "the trade catalogue's {key:?} writes {{{placeholder}}} inside quotes: a placeholder stands bare, and the translation quotes what fills it"
            ),
            CatalogueError::FieldsWithoutLands(key) => write!(
                f,
                "the trade catalogue names fields for {key:?}, which lands no text"
            ),
            CatalogueError::FieldNotLanded { key, field } => write!(
                f,
                "the trade catalogue names {field:?} for {key:?}, whose text does not write it"
            ),
            CatalogueError::UnknownField { key, field } => write!(
                f,
                "the trade catalogue names {field:?} for {key:?}, which this build does not know"
            ),
            CatalogueError::Lacked(key) => write!(
                f,
                "the trade catalogue gives {key:?} a `lacked`, which is `meets_maximum` on a row landing one field's `{{bounds}}` alone"
            ),
        }
    }
}

impl std::error::Error for CatalogueError {}

static CATALOGUE: LazyLock<Result<Catalogue, CatalogueError>> =
    LazyLock::new(|| Catalogue::parse(CATALOGUE_TOML));

/// The shipped catalogue, parsed and checked once.
pub fn catalogue() -> Result<&'static Catalogue, &'static CatalogueError> {
    CATALOGUE.as_ref()
}

/// A key as the header shapes it: lowercase words of letters, digits and
/// `_`, joined by `.`, and one `=<option>` after them at most, its option
/// a word of the same letters.
fn is_key(key: &str) -> bool {
    let word = |w: &str| {
        !w.is_empty()
            && w.bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
    };
    let (path, option) = match key.split_once('=') {
        Some((path, option)) => (path, Some(option)),
        None => (key, None),
    };
    path.split('.').all(word) && option.is_none_or(word)
}

/// Whether this build knows a field the catalogue names: the `is:` word, a
/// computed value, or a field of the language — its own or the property
/// table's.
fn knows(field: &str) -> bool {
    if field == "is" {
        return true;
    }
    if let Some(name) = field.strip_prefix("pseudo.") {
        return matches!(crate::pseudo::lookup(name), Ok(Some(_)));
    }
    crate::bind::is_field(field)
}

/// The placeholders a text holds, `{bounds}` and its kin, in order; a brace
/// never closed is returned with its brace and the rest, which is none.
fn placeholders(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(open) = rest.find('{') {
        let after = &rest[open + 1..];
        match after.find('}') {
            Some(close) => {
                out.push(&after[..close]);
                rest = &after[close + 1..];
            }
            None => {
                out.push(&rest[open..]);
                break;
            }
        }
    }
    out
}

/// The first placeholder a text writes inside a quoted string, where `\"`
/// and `\\` are the only escapes (`parse.rs`).
fn quoted_placeholder(text: &str) -> Option<&str> {
    let mut quoted = false;
    let mut escaped = false;
    for (at, c) in text.char_indices() {
        match c {
            _ if escaped => escaped = false,
            '\\' if quoted => escaped = true,
            '"' => quoted = !quoted,
            '{' if quoted => {
                let rest = &text[at + 1..];
                return Some(rest.find('}').map_or(rest, |close| &rest[..close]));
            }
            _ => {}
        }
    }
    None
}

/// Whether a text writes a field: its name, where no letter, digit, `_`
/// or `.` stands on either side.
fn writes(text: &str, field: &str) -> bool {
    let part = |c: char| c.is_ascii_alphanumeric() || c == '_' || c == '.';
    text.match_indices(field).any(|(at, _)| {
        let before = text[..at].chars().next_back();
        let after = text[at + field.len()..].chars().next();
        !before.is_some_and(part) && !after.is_some_and(part)
    })
}

impl Catalogue {
    /// Parse and check a catalogue text. The shipped file goes through
    /// [`catalogue`]; this is the same path over any text, for the tests
    /// that pin what the loader refuses.
    pub fn parse(text: &str) -> Result<Catalogue, CatalogueError> {
        let file: File = toml::from_str(text).map_err(|e| CatalogueError::Parse(e.to_string()))?;
        if file.version != CATALOGUE_VERSION {
            return Err(CatalogueError::Version {
                found: file.version,
                expected: CATALOGUE_VERSION,
            });
        }
        if file.filters != FILTERS_CAPTURE {
            return Err(CatalogueError::Capture {
                which: "filters",
                found: file.filters,
                expected: FILTERS_CAPTURE,
            });
        }
        if file.stats != stats::STAT_TABLE_VERSION {
            return Err(CatalogueError::Capture {
                which: "stats",
                found: file.stats,
                expected: stats::STAT_TABLE_VERSION,
            });
        }
        if file.rows.is_empty() {
            return Err(CatalogueError::NoRow);
        }
        let mut keys: HashSet<String> = HashSet::new();
        let mut rows = Vec::with_capacity(file.rows.len());
        for row in file.rows {
            let key = row.key;
            if !is_key(&key) {
                return Err(CatalogueError::BadKey(key));
            }
            if !keys.insert(key.clone()) {
                return Err(CatalogueError::DuplicateKey(key));
            }
            let Some(disposition) = Disposition::ALL
                .iter()
                .find(|(word, _)| *word == row.disposition)
                .map(|(_, d)| *d)
            else {
                return Err(CatalogueError::UnknownDisposition {
                    key,
                    disposition: row.disposition,
                });
            };
            if let Some(shape) = &row.shape
                && !SHAPES.contains(&shape.as_str())
            {
                return Err(CatalogueError::UnknownShape {
                    key,
                    shape: shape.clone(),
                });
            }
            for (given, what) in [
                (&row.text, "text"),
                (&row.note, "note"),
                (&row.says, "says"),
                (&row.lands, "landing"),
            ] {
                if given.as_ref().is_some_and(|t| t.trim().is_empty()) {
                    return Err(CatalogueError::Empty { key, what });
                }
            }
            if matches!(
                disposition,
                Disposition::Ambiguous | Disposition::Unsupported
            ) && row.note.is_none()
            {
                return Err(CatalogueError::NoNote(key));
            }
            if row.note.is_some() && row.says.is_none() {
                return Err(CatalogueError::NoSays(key));
            }
            if !disposition.runs() && row.lands.is_some() {
                return Err(CatalogueError::LandsWithoutRunning(key));
            }
            if let Some(lands) = &row.lands
                && let Some(placeholder) = placeholders(lands)
                    .into_iter()
                    .find(|p| !PLACEHOLDERS.contains(p))
            {
                return Err(CatalogueError::UnknownPlaceholder {
                    placeholder: placeholder.to_string(),
                    key,
                });
            }
            if let Some(lands) = &row.lands
                && let Some(placeholder) = quoted_placeholder(lands)
            {
                return Err(CatalogueError::QuotedPlaceholder {
                    placeholder: placeholder.to_string(),
                    key,
                });
            }
            match &row.lands {
                None if !row.fields.is_empty() => {
                    return Err(CatalogueError::FieldsWithoutLands(key));
                }
                None => {}
                Some(lands) => {
                    if let Some(field) = row.fields.iter().find(|f| !writes(lands, f)) {
                        return Err(CatalogueError::FieldNotLanded {
                            field: field.clone(),
                            key,
                        });
                    }
                }
            }
            if let Some(field) = row.fields.iter().find(|f| !knows(f)) {
                return Err(CatalogueError::UnknownField {
                    field: field.clone(),
                    key,
                });
            }
            let lacked_meets_maximum = match row.lacked.as_deref() {
                None => false,
                Some("meets_maximum") if matches!(row.fields.as_slice(), [one] if row.lands.as_deref() == Some(format!("{one}{{bounds}}").as_str())) => {
                    true
                }
                Some(_) => return Err(CatalogueError::Lacked(key)),
            };
            rows.push(Row {
                key,
                text: row.text,
                shape: row.shape,
                disposition,
                lands: row.lands,
                fields: row.fields,
                lacked_meets_maximum,
                note: row.note,
                says: row.says,
            });
        }
        Ok(Catalogue {
            version: file.version,
            rows,
        })
    }

    /// The version every basis that read the catalogue cites.
    pub fn version(&self) -> u32 {
        self.version
    }

    /// Every row, in the file's order.
    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    /// A row by its key, exact.
    pub fn row(&self, key: &str) -> Option<&Row> {
        self.rows.iter().find(|r| r.key == key)
    }
}
