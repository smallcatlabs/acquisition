//! The property fields (C114, C101, C106; step 3 of
//! `search/TRADE-PLAN.md`): every numeric property the trade site's
//! filters read, a field named by its displayed name — `quality`,
//! `attacks_per_second`, `wings_revealed.of` — defined once, in one table,
//! and read by one reader. The quality-normalised defences, `armour.q20`
//! and its three kin (S26), and a map's tier, `map_tier`, over its base
//! type, are rows of the same table. No general accessor (C101): a
//! property is reached by a row, and a row is reviewed reference data.
//!
//! # Decisions as recorded
//!
//! - **C114.** Owner, 2026-10-15: "build all using displayed names"; the
//!   quality-normalised defences "in this step". The table is
//!   `reference/properties-v1.toml`; its header carries the rules a
//!   reviewer reads — what a row means, the one status, the q20 rule and
//!   what it was measured against — and `--describe fields` prints each
//!   field with its definition and the table's version (C106's clause
//!   (c)). The names the catalogue spells (`trade-catalogue-v1.toml`) are
//!   the table's: the catalogue's loader refuses one this build does not
//!   know (`trade::catalogue`).
//! - **S26, the q20 rule.** The site's figure for a defence is the display
//!   with the item's quality taken out and 20 % put in, and the display
//!   itself at 20 % or more or under the enchant that drops quality —
//!   within one, since the site works from the roll the display rounded:
//!   equal on 1,148 of 1,313 captured rows and one off on 165, equal on 32
//!   of the fetch census's 37 (`tests/properties.rs`). So the field is the
//!   rule, rounded half up, and never claims to be the site's number.
//!   The display standing above 20 % is the owner's, 2026-10-15,
//!   verbatim: "yes, we need to able to handle qualities >20% because
//!   there are legacy mechanics that can make this happen."
//!
//! # As built
//!
//! - **The table is compiled in** and parsed once, on first use
//!   ([`table`]). The loader checks what a reviewer would: the version this
//!   build expects ([`PROPERTY_TABLE_VERSION`]), a source, every name once,
//!   one the language parses as a field and none the language already has
//!   — a field, a word it reads in a field's place, `pseudo.`,
//!   every row reading what its `reads` takes and nothing it does not, an
//!   array the deriver keeps properties under, a property name given and
//!   none twice, no two rows reading one value the same way, the rows
//!   the search's own values read by name (`quality`,
//!   `attacks_per_second`), a definition. A table that fails is a
//!   build defect surfaced as [`PropertyTableError`] to whoever asks —
//!   no corpus is loaded over it (`corpus.rs`) — never a partial table.
//! - **One reader, at the property's grain (rule 8 of the plan).** What a
//!   field is on an item is `read`'s, and every way of asking it — a
//!   comparison, `has:`, `undecided( … )`, `--sort`, `--sum`, a count's
//!   key — asks it (`eval.rs`), so they cannot disagree. A field is lacked
//!   only where nothing that could display its property was unread: the
//!   body, the array, an element whose name could not be read or is the
//!   row's (`derive::Unread::name`). A property displayed as other than
//!   the row reads, or more than once, is unread under its array, said
//!   here; nothing else of the item leaves the field open. Which property
//!   is the one so named, and what unread may be it, are one rule here
//!   (`unread_under`, `one`) that the damage fields and the percentile
//!   (`pseudo.rs`) ask too, so no two readers take different properties.
//!   Every row reads its value the one way (`value_of`), so a row names
//!   nothing its reading ignores.
//! - **A number is read once** (`exact.rs`): the digits within the crate's
//!   rule, a `+` and a unit each read where written and only where the row
//!   names them, and a fraction made of whole numbers alone. A gem's
//!   experience is the percent cut toward nothing to the units; the q20
//!   product is rounded half up from a fraction, never from a float.
//! - **A value two fields read is read once** (rule 10 of the plan). The
//!   q20 rows read the `quality` row, and so does the base defence
//!   percentile (`pseudo.rs`): one maker of what the item's quality is;
//!   both read a displayed defence by `defence`. The damage fields read
//!   the `attacks_per_second` row. The q20 rule reads the quality and the
//!   enchant before either decides, so what is unread of one leaves the
//!   field open only where the other does not settle it.
//! - **The version is on the basis** (C98): `properties`, beside the other
//!   tables'.

use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::LazyLock;

use serde::Deserialize;

use crate::bind::{FieldDef, Kind, Thing};
use crate::derive::{Item, Line, Part, Property, Unread};
use crate::exact::{self, Exact, Ratio};

/// The table this build ships, verbatim (the reviewed file).
pub const PROPERTY_TABLE_TOML: &str = include_str!("../reference/properties-v1.toml");

/// The table version this build expects and every basis cites; a file
/// stamped otherwise is a build defect, not a runtime condition.
pub const PROPERTY_TABLE_VERSION: u32 = 1;

/// The arrays a row may read: those the deriver keeps as properties.
pub const ARRAYS: [&str; 3] = ["properties", "additionalProperties", "requirements"];

/// The quality the q20 rule puts in, in percent (S26).
const Q20: i128 = 20;

/// Words the language reads where a field's name may stand, each with a
/// meaning of its own: a field so named would be shadowed by it, or
/// shadow it — `--count line` is the vocabulary.
const WORDS: [&str; 7] = [
    "line",
    "linked",
    "sum",
    "holds",
    "undecided",
    "true",
    "false",
];

/// The rows the search's own values read by name, each with the property
/// it must read a number of: the quality, by the q20 rows and the
/// percentile (`pseudo.rs`); the attack speed, by the damage fields. A
/// value two fields read is read once, by its row (rule 10 of the plan),
/// so a table without one is refused.
const QUALITY: &str = "quality";
const ATTACKS_PER_SECOND: &str = "attacks_per_second";
const READ_BY_NAME: [(&str, &str); 2] = [
    (QUALITY, "Quality"),
    (ATTACKS_PER_SECOND, "Attacks per Second"),
];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    version: u32,
    source: String,
    #[serde(rename = "field", default)]
    fields: Vec<RowIn>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RowIn {
    name: String,
    array: Option<String>,
    #[serde(default)]
    property: Vec<String>,
    value: Option<usize>,
    select: Option<SelectIn>,
    reads: String,
    #[serde(default)]
    plus: bool,
    unit: Option<String>,
    #[serde(default)]
    whole: bool,
    #[serde(default)]
    marks: Vec<String>,
    definition: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SelectIn {
    value: usize,
    is: String,
}

/// What a row reads (the table's header).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reads {
    Number,
    Numerator,
    Denominator,
    Percent,
    Q20,
    Tier,
}

impl Reads {
    const ALL: [(&'static str, Reads); 6] = [
        ("number", Reads::Number),
        ("numerator", Reads::Numerator),
        ("denominator", Reads::Denominator),
        ("percent", Reads::Percent),
        ("q20", Reads::Q20),
        ("tier", Reads::Tier),
    ];
}

/// One field of the table.
#[derive(Debug, PartialEq, Eq)]
pub struct Field {
    name: String,
    /// `None` for a field over the base (`tier`).
    array: Option<String>,
    property: Vec<String>,
    /// Which value is read; none where the property displays one value,
    /// and one displaying several is then not this row's shape.
    value: Option<usize>,
    /// A value that must be the word for the property to be this field's.
    select: Option<(usize, String)>,
    reads: Reads,
    plus: bool,
    unit: String,
    whole: bool,
    marks: Vec<String>,
    definition: String,
    /// The definition with the table's version, as `--describe` prints it.
    what: String,
    /// What `--describe` teaches it by: a comparison and its absence.
    examples: [String; 2],
}

impl Field {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reads(&self) -> Reads {
        self.reads
    }

    /// The definition as the table states it.
    pub fn definition(&self) -> &str {
        &self.definition
    }

    /// What `--describe` prints as its examples.
    pub(crate) fn examples(&'static self) -> Vec<&'static str> {
        self.examples.iter().map(String::as_str).collect()
    }

    fn names(&self, name: &str) -> bool {
        self.property.iter().any(|p| p.eq_ignore_ascii_case(name))
    }
}

/// The parsed, checked table. Read-only; one per build.
#[derive(Debug)]
pub struct PropertyTable {
    version: u32,
    source: String,
    fields: Vec<Field>,
}

/// Why a table text is not a table. Every arm names the offending row so a
/// reviewer can fix the file, not the loader.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyTableError {
    Parse(String),
    Version {
        found: u32,
        expected: u32,
    },
    NoSource,
    NoField,
    /// A name that is no lowercase word, or not one the language parses
    /// as a field name.
    BadName(String),
    DuplicateName(String),
    /// A name the language already has: a field of its own, a word it
    /// reads where a field may stand, or a computed value's namespace.
    Taken(String),
    UnknownReads {
        field: String,
        reads: String,
    },
    /// A row that names what its `reads` does not take, or lacks what it
    /// needs, with what.
    Shape {
        field: String,
        what: &'static str,
    },
    UnknownArray {
        field: String,
        array: String,
    },
    DuplicateProperty {
        field: String,
        property: String,
    },
    /// Two rows reading one value of one property the same way.
    SameReading {
        field: String,
        other: String,
    },
    /// No row of this name reading a number of this property, which the
    /// search's own values read by it.
    NoRow {
        row: &'static str,
        property: &'static str,
    },
    NoDefinition(String),
}

impl std::fmt::Display for PropertyTableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyTableError::Parse(e) => write!(f, "the property table does not parse: {e}"),
            PropertyTableError::Version { found, expected } => write!(
                f,
                "the property table is version {found}; this build reads version {expected}"
            ),
            PropertyTableError::NoSource => write!(f, "the property table names no source"),
            PropertyTableError::NoField => write!(f, "the property table has no field"),
            PropertyTableError::BadName(name) => write!(
                f,
                "the property table names the field {name:?}, which is not lowercase letters, digits, `_` and `.` that the language reads as one field name"
            ),
            PropertyTableError::DuplicateName(name) => {
                write!(f, "the property table names the field {name:?} twice")
            }
            PropertyTableError::Taken(name) => write!(
                f,
                "the property table names the field {name:?}, which the language already has"
            ),
            PropertyTableError::UnknownReads { field, reads } => write!(
                f,
                "the property table has {field:?} read {reads:?}, which is none of number, numerator, denominator, percent, q20, tier"
            ),
            PropertyTableError::Shape { field, what } => {
                write!(f, "the property table's {field:?} {what}")
            }
            PropertyTableError::UnknownArray { field, array } => write!(
                f,
                "the property table has {field:?} read the array {array:?}, which holds no property the deriver keeps"
            ),
            PropertyTableError::DuplicateProperty { field, property } => write!(
                f,
                "the property table names {property:?} twice for {field:?}"
            ),
            PropertyTableError::SameReading { field, other } => write!(
                f,
                "the property table has {field:?} read what {other:?} reads, the same way"
            ),
            PropertyTableError::NoRow { row, property } => write!(
                f,
                "the property table has no {row:?} row reading a number of `{property}`, which the search's own values read by it"
            ),
            PropertyTableError::NoDefinition(field) => {
                write!(f, "the property table gives {field:?} no definition")
            }
        }
    }
}

impl std::error::Error for PropertyTableError {}

static TABLE: LazyLock<Result<PropertyTable, PropertyTableError>> =
    LazyLock::new(|| PropertyTable::parse(PROPERTY_TABLE_TOML));

/// The shipped table, parsed and checked once.
pub fn table() -> Result<&'static PropertyTable, &'static PropertyTableError> {
    TABLE.as_ref()
}

/// The fields as the binder knows them: every row, a number, defined by
/// its row (`bind.rs`). None where the table does not load, which no
/// corpus is loaded over.
static DEFS: LazyLock<Vec<FieldDef>> = LazyLock::new(|| match table() {
    Ok(table) => table
        .fields
        .iter()
        .map(|field| FieldDef {
            name: field.name.as_str(),
            thing: Thing::Property(field),
            kind: Kind::Number,
            what: field.what.as_str(),
        })
        .collect(),
    Err(_) => Vec::new(),
});

pub(crate) fn field_defs() -> &'static [FieldDef] {
    &DEFS
}

fn shape(field: &str, what: &'static str) -> PropertyTableError {
    PropertyTableError::Shape {
        field: field.to_string(),
        what,
    }
}

/// Whether a name is one the table may give: lowercase ASCII letters,
/// digits, `_` and `.`, a letter first and no dot at either end or beside
/// another — and one the parser reads as one field name.
fn is_name(name: &str) -> bool {
    let words_ok = name.split('.').all(|word| {
        word.bytes().next().is_some_and(|b| b.is_ascii_lowercase())
            && word
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
    });
    words_ok
        && matches!(
            crate::parse::parse(&format!("{name}>=1")),
            Ok(crate::tree::Node::Test { ref field, .. }) if field == name
        )
}

impl PropertyTable {
    /// Parse and check a table text. The shipped file goes through
    /// [`table`]; this is the same path over any text, for the tests that
    /// pin what the loader refuses.
    pub fn parse(text: &str) -> Result<PropertyTable, PropertyTableError> {
        let file: File =
            toml::from_str(text).map_err(|e| PropertyTableError::Parse(e.to_string()))?;
        if file.version != PROPERTY_TABLE_VERSION {
            return Err(PropertyTableError::Version {
                found: file.version,
                expected: PROPERTY_TABLE_VERSION,
            });
        }
        if file.source.trim().is_empty() {
            return Err(PropertyTableError::NoSource);
        }
        if file.fields.is_empty() {
            return Err(PropertyTableError::NoField);
        }
        let mut names: HashSet<String> = HashSet::new();
        let mut readings: Vec<(String, String)> = Vec::new();
        let mut fields = Vec::with_capacity(file.fields.len());
        for row in file.fields {
            let name = row.name;
            if !is_name(&name) {
                return Err(PropertyTableError::BadName(name));
            }
            if crate::tree::is_computed(&name)
                || WORDS.contains(&name.as_str())
                || crate::bind::FIELDS
                    .iter()
                    .any(|f| f.name.eq_ignore_ascii_case(&name))
            {
                return Err(PropertyTableError::Taken(name));
            }
            if !names.insert(name.clone()) {
                return Err(PropertyTableError::DuplicateName(name));
            }
            let Some(reads) = Reads::ALL
                .iter()
                .find(|(word, _)| *word == row.reads)
                .map(|(_, r)| *r)
            else {
                return Err(PropertyTableError::UnknownReads {
                    field: name,
                    reads: row.reads,
                });
            };
            let unit_given = row.unit.is_some();
            let unit = row.unit.unwrap_or_default();
            if reads == Reads::Tier {
                if row.array.is_some()
                    || !row.property.is_empty()
                    || row.value.is_some()
                    || row.select.is_some()
                {
                    return Err(shape(&name, "reads the base and names a property"));
                }
            } else {
                let Some(array) = row.array.as_deref() else {
                    return Err(shape(&name, "names no array"));
                };
                if !ARRAYS.contains(&array) {
                    return Err(PropertyTableError::UnknownArray {
                        field: name,
                        array: array.to_string(),
                    });
                }
                if row.property.is_empty() {
                    return Err(shape(&name, "names no property"));
                }
                let mut seen = HashSet::new();
                for property in &row.property {
                    if property.trim().is_empty() {
                        return Err(shape(&name, "names an empty property"));
                    }
                    if !seen.insert(property.to_ascii_lowercase()) {
                        return Err(PropertyTableError::DuplicateProperty {
                            field: name,
                            property: property.clone(),
                        });
                    }
                }
            }
            if reads != Reads::Number && (row.plus || row.whole || !row.marks.is_empty()) {
                return Err(shape(
                    &name,
                    "names a `+`, a whole number or a mark, which only `number` reads",
                ));
            }
            if reads != Reads::Number && unit_given {
                return Err(shape(&name, "names a unit, which only `number` reads"));
            }
            if unit_given && unit.is_empty() {
                return Err(shape(&name, "names an empty unit"));
            }
            if row.marks.iter().any(|m| m.trim().is_empty()) {
                return Err(shape(&name, "names an empty mark"));
            }
            let value = row.value;
            let select = match row.select {
                Some(select) if select.is.trim().is_empty() => {
                    return Err(shape(&name, "selects by an empty word"));
                }
                Some(select) if Some(select.value) == value => {
                    return Err(shape(&name, "selects by the value it reads"));
                }
                Some(select) => Some((select.value, select.is)),
                None => None,
            };
            if row.definition.trim().is_empty() {
                return Err(PropertyTableError::NoDefinition(name));
            }
            // what the row reads, the same way: two rows reading it are one
            // field under two names
            let mut property: Vec<String> = row
                .property
                .iter()
                .map(|p| p.to_ascii_lowercase())
                .collect();
            property.sort();
            let reading = format!(
                "{:?} {:?} {value:?} {:?} {reads:?}",
                row.array,
                property,
                select
                    .as_ref()
                    .map(|(at, word)| (at, word.to_ascii_lowercase())),
            );
            if let Some((other, _)) = readings.iter().find(|(_, r)| *r == reading) {
                return Err(PropertyTableError::SameReading {
                    field: name,
                    other: other.clone(),
                });
            }
            readings.push((name.clone(), reading));
            fields.push(Field {
                what: format!(
                    "{} (properties v{})",
                    row.definition.trim_end_matches('.'),
                    file.version
                ),
                examples: [format!("{name}>=1"), format!("-has:{name}")],
                name,
                array: row.array,
                property: row.property,
                value,
                select,
                reads,
                plus: row.plus,
                unit,
                whole: row.whole,
                marks: row.marks,
                definition: row.definition,
            });
        }
        for (row, property) in READ_BY_NAME {
            let read = fields.iter().any(|f| {
                f.name == row
                    && f.reads == Reads::Number
                    && f.array.as_deref() == Some("properties")
                    && f.names(property)
            });
            if !read {
                return Err(PropertyTableError::NoRow { row, property });
            }
        }
        Ok(PropertyTable {
            version: file.version,
            source: file.source,
            fields,
        })
    }

    /// The version every basis cites.
    pub fn version(&self) -> u32 {
        self.version
    }

    /// The table-level source, verbatim.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Every field, in the table's order.
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    /// A field by its name, exact.
    pub fn field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == name)
    }
}

// ---- the reader ---------------------------------------------------------------

/// A field on one item: its value with what it was read from, known
/// absence, or what left it open (the module doc).
#[derive(Debug)]
pub(crate) enum FieldRead<'a> {
    Value {
        value: Exact,
        /// The properties read, in the item's order.
        shown: Vec<&'a Property>,
        /// The enchant a q20 field found, which made the display its value.
        lines: Vec<&'a Line>,
    },
    Lacked,
    Unread(Vec<Cow<'a, Unread>>),
}

impl FieldRead<'_> {
    /// The value, where it is established.
    pub fn value(&self) -> Option<Exact> {
        match self {
            FieldRead::Value { value, .. } => Some(*value),
            FieldRead::Lacked | FieldRead::Unread(_) => None,
        }
    }
}

/// The one reading of a field on an item (the module doc).
pub(crate) fn read<'a>(field: &Field, item: &'a Item) -> FieldRead<'a> {
    match field.reads {
        Reads::Tier => tier(item),
        Reads::Q20 => q20(field, item),
        Reads::Number => match found(field, item) {
            Err(unread) => FieldRead::Unread(unread),
            Ok(None) => FieldRead::Lacked,
            Ok(Some(property)) => match value_of(field.value, property) {
                Err(unread) => FieldRead::Unread(vec![unread]),
                Ok(text) => match number(field, text) {
                    Some(value) => FieldRead::Value {
                        value,
                        shown: vec![property],
                        lines: Vec::new(),
                    },
                    None if field.whole => unread(property, "no whole number the search reads"),
                    None => unread(property, "no number the search reads"),
                },
            },
        },
        Reads::Numerator | Reads::Denominator | Reads::Percent => match found(field, item) {
            Err(unread) => FieldRead::Unread(unread),
            Ok(None) => FieldRead::Lacked,
            Ok(Some(property)) => {
                let text = match value_of(field.value, property) {
                    Ok(text) => text,
                    Err(unread) => return FieldRead::Unread(vec![unread]),
                };
                let Some((a, b)) = pair(text) else {
                    return unread(property, "no `a/b` of whole numbers the search reads");
                };
                let value = match field.reads {
                    Reads::Numerator => Exact::whole(a),
                    Reads::Denominator => Exact::whole(b),
                    _ if b == 0 => {
                        return unread(property, "a part of nothing, no percent");
                    }
                    _ => Ratio::new(a * 100, b).and_then(Exact::cut),
                };
                match value {
                    Some(value) => FieldRead::Value {
                        value,
                        shown: vec![property],
                        lines: Vec::new(),
                    },
                    None => unread(property, "past what the search computes with"),
                }
            }
        },
    }
}

/// A row the search's own values read by name (`READ_BY_NAME`), on an
/// item.
fn by_name<'a>(row: &str, item: &'a Item) -> FieldRead<'a> {
    match table().ok().and_then(|t| t.field(row)) {
        Some(field) => read(field, item),
        // the loader refuses a table without it, and no corpus is loaded
        // over a table that does not load
        None => FieldRead::Lacked,
    }
}

/// The item's attack speed as the `attacks_per_second` row reads it:
/// what the damage fields (`pseudo.rs`) multiply by.
pub(crate) fn attacks_per_second(item: &Item) -> FieldRead<'_> {
    by_name(ATTACKS_PER_SECOND, item)
}

/// The item's quality as the `quality` row reads it, a whole percent and
/// the property it was read from, none where it displays none: what the
/// q20 rows and the base defence percentile (`pseudo.rs`) take it as.
pub(crate) fn quality(item: &Item) -> Result<Option<(i128, &Property)>, Vec<Cow<'_, Unread>>> {
    match by_name(QUALITY, item) {
        FieldRead::Unread(unread) => Err(unread),
        FieldRead::Lacked => Ok(None),
        FieldRead::Value { value, shown, .. } => match (value.as_whole(), shown.first()) {
            (Some(number), Some(property)) => Ok(Some((number, *property))),
            // the `quality` row reads whole numbers of one property alone,
            // so neither arm below is met; each still says what it is
            (None, Some(property)) => {
                Err(vec![problem(property, "no whole number the search reads")])
            }
            (_, None) => Ok(None),
        },
    }
}

/// What was unread of the item that may be a property of `array` the
/// reader takes by one of these names: the body, the array itself, or an
/// element of it whose name could not be read or is one of them. Every
/// reader of a property asks it first (rule 8), so a property is lacked
/// only where nothing that could display it was unread.
pub(crate) fn unread_under<'a>(
    item: &'a Item,
    array: &str,
    names: &dyn Fn(&str) -> bool,
) -> Vec<Cow<'a, Unread>> {
    let part = Part::Properties(array.to_string());
    item.unread
        .iter()
        .filter(|u| u.part == Part::Body || (u.part == part && u.name.as_deref().is_none_or(names)))
        .map(Cow::Borrowed)
        .collect()
}

/// The one property among those displayed for one reading: none, it, or
/// — displayed more than once — unread, said under its array. Every
/// reader of a property takes it by this rule (rule 10), so no two can
/// take different ones.
fn the_one<'a>(
    mut displayed: impl Iterator<Item = &'a Property>,
) -> Result<Option<&'a Property>, Cow<'a, Unread>> {
    match (displayed.next(), displayed.next()) {
        (None, _) => Ok(None),
        (Some(one), None) => Ok(Some(one)),
        (Some(first), Some(_)) => Err(problem(first, "displayed more than once")),
    }
}

/// The one property of `array` the item displays as `name`, by
/// [`the_one`]'s rule: what a computed value of several properties reads
/// each by (`pseudo.rs`), after asking [`unread_under`] of them all.
pub(crate) fn one<'a>(
    item: &'a Item,
    array: &str,
    name: &str,
) -> Result<Option<&'a Property>, Cow<'a, Unread>> {
    the_one(
        held(item, array)
            .iter()
            .filter(|p| p.array == array && p.name.eq_ignore_ascii_case(name)),
    )
}

/// The properties the deriver keeps for an array.
fn held<'a>(item: &'a Item, array: &str) -> &'a [Property] {
    if array == "requirements" {
        &item.requirements
    } else {
        &item.properties
    }
}

/// The one property of the item the row reads, none, or what left that
/// open: what was unread that may be it, a property of the row's shape
/// that does not say whose it is, or it displayed more than once.
fn found<'a>(field: &Field, item: &'a Item) -> Result<Option<&'a Property>, Vec<Cow<'a, Unread>>> {
    let array = field.array.as_deref().unwrap_or_default();
    let unread = unread_under(item, array, &|name| field.names(name));
    if !unread.is_empty() {
        return Err(unread);
    }
    let mut its: Vec<&'a Property> = Vec::new();
    for property in held(item, array)
        .iter()
        .filter(|p| p.array == array && field.names(&p.name))
    {
        match &field.select {
            None => its.push(property),
            Some((at, word)) => match property.values.get(*at) {
                Some(value) if value.eq_ignore_ascii_case(word) => its.push(property),
                Some(_) => {}
                // a property of the shape that does not say whose it is may
                // be this field's
                None => {
                    return Err(vec![problem(property, "no value that says which it is")]);
                }
            },
        }
    }
    the_one(its.into_iter()).map_err(|unread| vec![unread])
}

/// A property displayed as the reader does not read it, said under its
/// array: the one wording every reader of a property gives.
pub(crate) fn problem<'a>(property: &Property, what: &str) -> Cow<'a, Unread> {
    Cow::Owned(Unread {
        part: Part::Properties(property.array.clone()),
        problem: format!(
            "`{}` is `{}`: {what}",
            property.name,
            property.values.join(", ")
        ),
        line: None,
        name: Some(property.name.clone()),
        socket: None,
    })
}

fn unread<'a>(property: &Property, what: &str) -> FieldRead<'a> {
    FieldRead::Unread(vec![problem(property, what)])
}

/// A displayed number as the row reads it (the table's header): one of
/// its marks, then its unit, then a `+`, each taken off where written and
/// only where the row names it, and what is left the digits alone.
fn number(field: &Field, text: &str) -> Option<Exact> {
    let mut text = text;
    if let Some(rest) = field
        .marks
        .iter()
        .find_map(|m| text.strip_suffix(m.as_str()))
    {
        text = rest;
    }
    if !field.unit.is_empty() {
        text = text.strip_suffix(field.unit.as_str()).unwrap_or(text);
    }
    if field.plus {
        text = text.strip_prefix('+').unwrap_or(text);
    }
    // no other sign, and nothing else beside the digits
    if !text.starts_with(|c: char| c.is_ascii_digit()) || !exact::reads(text) {
        return None;
    }
    if field.whole && text.contains('.') {
        return None;
    }
    text.parse::<f64>()
        .ok()
        .filter(|n| n.is_finite())
        .map(Exact::of)
}

/// The value a row reads: the one it names, or the one value of a
/// property that displays one — every row reads its value so, a q20 row
/// too, so no row names a value its reading ignores.
fn value_of<'b>(at: Option<usize>, property: &Property) -> Result<&str, Cow<'b, Unread>> {
    match at {
        Some(at) => property
            .values
            .get(at)
            .map(String::as_str)
            .ok_or_else(|| problem(property, "no such value")),
        None => match property.values.as_slice() {
            [one] => Ok(one.as_str()),
            _ => Err(problem(property, "not one value")),
        },
    }
}

/// `a/b`, two whole numbers within the crate's rule.
fn pair(text: &str) -> Option<(i128, i128)> {
    let (a, b) = text.split_once('/')?;
    let whole = |s: &str| {
        (!s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) && exact::reads(s))
            .then(|| s.parse::<i128>().ok())
            .flatten()
    };
    Some((whole(a)?, whole(b)?))
}

/// The q20 rule (the table's header, S26): the display stands where the
/// quality is 20 % or more, or under the enchant that drops it; otherwise
/// the display with the quality taken out and 20 % put in. Both facts are
/// read before either decides, and one not established leaves the field
/// open only where the other does not settle it (rule 8).
fn q20<'a>(field: &Field, item: &'a Item) -> FieldRead<'a> {
    let (display, property) = match found(field, item) {
        Err(unread) => return FieldRead::Unread(unread),
        Ok(None) => return FieldRead::Lacked,
        Ok(Some(property)) => match defence(property, field.value) {
            Ok(display) => (display, property),
            Err(unread) => return FieldRead::Unread(vec![unread]),
        },
    };
    // the quality and the enchant are asked only of an item displaying
    // the defence (rule 8)
    let quality = quality(item).map(|q| q.map_or((0, None), |(n, p)| (n, Some(p))));
    let enchant = enchant(item);
    let mut shown = vec![property];
    if let Ok((_, Some(of))) = quality {
        shown.push(of);
    }
    shown.sort_by_key(|p| item.properties.iter().position(|q| std::ptr::eq(q, *p)));
    let value = |n: Option<i128>, lines: Vec<&'a Line>| match n.and_then(Exact::whole) {
        Some(value) => FieldRead::Value {
            value,
            shown: shown.clone(),
            lines,
        },
        None => unread(property, "past what the search computes with"),
    };
    match (quality, enchant) {
        (Ok((quality, _)), _) if quality >= Q20 => value(Some(display), Vec::new()),
        (_, Ok(lines)) if !lines.is_empty() => value(Some(display), lines),
        // the display × (100 + 20) ÷ (100 + the quality), rounded half up
        (Ok((quality, _)), Ok(_)) => value(
            Ratio::new(display * (100 + Q20), 100 + quality).and_then(Ratio::half_up),
            Vec::new(),
        ),
        (quality, enchant) => FieldRead::Unread(
            quality
                .err()
                .into_iter()
                .chain(enchant.err())
                .flatten()
                .collect(),
        ),
    }
}

/// The lines of the enchant that drops quality (J6), none, or — none read
/// and an array of lines unread, which may hold it wherever it sits —
/// what was unread.
fn enchant(item: &Item) -> Result<Vec<&Line>, Vec<Cow<'_, Unread>>> {
    let enchant: Vec<&Line> = item
        .lines
        .iter()
        .filter(|line| {
            crate::defences::drops_quality(&line.template)
                || line
                    .rows
                    .iter()
                    .any(|row| crate::defences::drops_quality(&row.template))
        })
        .collect();
    if !enchant.is_empty() {
        return Ok(enchant);
    }
    let open: Vec<Cow<'_, Unread>> = item
        .unread
        .iter()
        .filter(|u| match &u.part {
            Part::Lines(_) => true,
            Part::Field(key) => key == "hybrid",
            _ => false,
        })
        .map(Cow::Borrowed)
        .collect();
    if open.is_empty() {
        Ok(enchant)
    } else {
        Err(open)
    }
}

/// A defence as displayed — the value read as every row reads one
/// (`value_of`), digits alone — or why it is not: what the q20 fields and
/// the base defence percentile (`pseudo.rs`) both read of `Armour` and its
/// kin, one reading.
pub(crate) fn defence<'a>(property: &Property, at: Option<usize>) -> Result<i128, Cow<'a, Unread>> {
    let text = value_of(at, property)?;
    whole(text).ok_or_else(|| problem(property, "no whole number the search reads"))
}

/// A whole number as a defence is displayed: digits alone, within the
/// crate's rule for a number.
fn whole(text: &str) -> Option<i128> {
    (!text.is_empty() && text.bytes().all(|b| b.is_ascii_digit()) && exact::reads(text))
        .then(|| text.parse().ok())
        .flatten()
}

/// A map's tier, the number its base type ends in: ` (Tier N)`.
fn tier(item: &Item) -> FieldRead<'_> {
    let Some(base) = item.base.as_deref() else {
        let unread: Vec<Cow<'_, Unread>> = item
            .unread
            .iter()
            .filter(|u| {
                u.part == Part::Body || matches!(&u.part, Part::Field(key) if key == "baseType")
            })
            .map(Cow::Borrowed)
            .collect();
        return if unread.is_empty() {
            FieldRead::Lacked
        } else {
            FieldRead::Unread(unread)
        };
    };
    let tier = base
        .strip_suffix(')')
        .and_then(|rest| rest.rsplit_once(" (Tier "))
        .and_then(|(_, n)| whole(n));
    match tier.and_then(Exact::whole) {
        Some(value) => FieldRead::Value {
            value,
            shown: Vec::new(),
            lines: Vec::new(),
        },
        None => FieldRead::Lacked,
    }
}

#[cfg(test)]
mod tests {
    use super::{FieldRead, PROPERTY_TABLE_TOML, PropertyTable, read};
    use crate::derive::{Facts, derive};
    use crate::exact::Exact;

    /// Every row reads its value the one way (`value_of`), a q20 row too:
    /// the `value` a row names is the value read, so no row names one its
    /// reading ignores.
    #[test]
    fn a_q20_row_reads_the_value_it_names() {
        let facts = Facts {
            id: "i".to_string(),
            realm: "pc".to_string(),
            league: Some("Standard".to_string()),
            location_kind: "stash".to_string(),
            location_id: "t".to_string(),
            container: None,
            socketed_in: None,
            first_seen: 0,
            last_seen: 0,
            removed_at: None,
        };
        let item = derive(
            facts,
            r#"{"id":"i","baseType":"B","properties":[{"name":"Armour","values":[["100",0]],"displayMode":0}]}"#,
        );
        let probe = |value: usize| {
            let text = format!(
                "{PROPERTY_TABLE_TOML}\n[[field]]\nname = \"probe\"\narray = \"properties\"\nproperty = [\"Armour\"]\nreads = \"q20\"\nvalue = {value}\ndefinition = \"d\"\n"
            );
            let table = PropertyTable::parse(&text).unwrap();
            let field = table.field("probe").unwrap();
            match read(field, &item) {
                FieldRead::Value { value, .. } => Ok(value),
                FieldRead::Unread(unread) => Err(unread[0].problem.clone()),
                FieldRead::Lacked => Err("lacked".to_string()),
            }
        };
        // the first value, 100 at no quality: 100 × 120 ÷ 100
        assert_eq!(probe(0), Ok(Exact::whole(120).unwrap()));
        assert_eq!(
            probe(1),
            Err("`Armour` is `100`: no such value".to_string())
        );
    }
}
