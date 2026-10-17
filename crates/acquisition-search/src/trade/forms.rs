//! The forms table (C113, C106; step 7e of `search/TRADE-PLAN.md`): the
//! other ways the game displays a stat the trade site files under one id —
//! the opposite word below nothing, a number the display leaves out, a
//! plural, another wording — read by the translation alone
//! ([`super::translate`]), for a stat's presence over every form and to
//! say where a bound cannot be exact.
//!
//! # Decisions as recorded
//!
//! - **The choice before 7e** (the owner, 2026-10-17; the brief's section
//!   of that name, `brainstorming-notes/37-the-stat-value.md`): the
//!   translation alone ships, and the forms knowledge is data, once — "a
//!   forms table beside the catalogue — which texts have an opposite word,
//!   which elide to a numberless display, the exceptions — reference data
//!   (C106) read by the translation alone, so the park lifts it rather than
//!   finds it again". The park is "A named stat value"
//!   (`decisions/search.md`).
//! - **The game's forms first, the brief's rules beside** (the owner,
//!   2026-10-17, during 7e, choosing "Game data, rules beside"): the
//!   brief's blanket rules named forms the game never shows — `#% chance
//!   to X` elides at 100 % for 212 of the 372 texts the PoE1 export joins
//!   and not for 160 (`Suppress Spell Damage`), and 316 of 2,235 joined
//!   texts with a sign word have no opposite display — so a text the export
//!   joins takes the game's own forms, and the rules stand for a text it
//!   joins nowhere.
//!
//! # As built
//!
//! - **Two files, one table.** `reference/trade-game-forms-v1.toml` is
//!   generated (`tools/trade-forms-table.py`, the export at its pin): each
//!   joined text's forms, of a kind. `reference/trade-forms-v1.toml` is by
//!   hand: the rules, the forms the captures show and neither gives, and
//!   the refusals. Both are compiled in and parsed once, on first use
//!   ([`table`]), and the loader holds them to their headers: the version
//!   this build expects ([`FORMS_VERSION`]) and the stat table's date, a
//!   source, two words to a pair and no word in two, every elision's key
//!   once and its `drop` holding a `#`, every text a text the stat table
//!   lists, a form that is not its own text, a kind of the four and a
//!   number only on a `numberless` form, a `plural` form with its text's
//!   open `#`s, no form twice, no row the game's forms already give, every
//!   refusal a listed text under a rule the table has.
//! - **A rule is applied to the text the translation writes** — the site's,
//!   its ` (Local)` dropped — where the export joins it nowhere; the forms
//!   no rule makes are looked up by that text too. The opposite words
//!   apply to a text with a `#` the site leaves open, every combination of
//!   them where it has several; an elision where its `drop` stands, a `+`
//!   before a `#` spelling, its first letter then a capital.
//! - **Two texts are told apart as the deriver tells two templates** —
//!   [`key`]: the template of the text with its numbers as `#`, a sign
//!   before a `#` dropped, case aside — so a form is the line a body would
//!   derive, and `-# to X`, the game's spelling of a negative, is `# to X`.
//! - **A form that is a listed text is refused by name, and the
//!   vocabulary is walked for it** ([`FormsTable::walk`],
//!   `tests/trade_forms.rs`): every such form, whatever made it, and every
//!   text a rule meets carrying both words of a pair, must each be a
//!   `[[refused]]` row, and every such row one of them. A refused form is
//!   left out of its text's forms, and the refusal named beside them
//!   ([`Forms`]) for the text and for the listed text the form is: the
//!   translation calls either ambiguous, since an item's line may be filed
//!   under the other.
//! - **The version is the report's** (C98), beside the catalogue's and the
//!   stat table's; the build its basis names digests both files.

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use serde::Deserialize;

use super::stats::{self, StatTable};
use crate::template;

/// The table this build ships, verbatim (the reviewed file).
pub const FORMS_TOML: &str = include_str!("../../reference/trade-forms-v1.toml");

/// The game's forms of each joined text, verbatim (the generated file:
/// `tools/trade-forms-table.py`).
pub const GAME_FORMS_TOML: &str = include_str!("../../reference/trade-game-forms-v1.toml");

/// The table version this build expects and every report that read it
/// cites. A file stamped otherwise is a build defect.
pub const FORMS_VERSION: u32 = 1;

/// What the translation drops from a site text before it writes it (S72;
/// `stat.text.local`).
pub(crate) const LOCAL: &str = " (Local)";

/// What a form is to a bound on its stat (the file's header).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// The same numbers, a noun's plural or singular: summed with the text.
    Plural,
    /// The number below nothing, spelled by another word.
    Opposite,
    /// A number the display leaves out, known or hidden.
    Numberless,
    /// Another wording no sitting has read.
    Other,
}

impl Kind {
    const ALL: [(&'static str, Kind); 4] = [
        ("plural", Kind::Plural),
        ("opposite", Kind::Opposite),
        ("numberless", Kind::Numberless),
        ("other", Kind::Other),
    ];

    /// The word the file spells it with.
    pub fn word(self) -> &'static str {
        Kind::ALL
            .iter()
            .find(|(_, k)| *k == self)
            .map_or("", |(w, _)| w)
    }
}

/// One other display of a stat: the text as the site would write it, `#`
/// where the number varies and a digit where the text has one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Form {
    pub text: String,
    pub kind: Kind,
    /// What made it: `game`, the export; `opposite` or an elision's key, a
    /// rule; or `form`, a row.
    pub by: String,
}

/// A refusal by name (the file's `[[refused]]`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refused {
    pub text: String,
    pub rule: String,
    /// The listed text the refused rule's form would be; none for a text
    /// carrying both words of a pair.
    pub form: Option<String>,
}

/// A stat's other displays the table can say, and the refusals that leave
/// the rest to the site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Forms<'a> {
    pub known: Vec<Form>,
    /// The text's own refusal, and any whose form is this text.
    pub refused: Vec<&'a Refused>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    version: u32,
    stats: String,
    source: String,
    #[serde(default)]
    opposite: Vec<OppositeIn>,
    #[serde(default)]
    elision: Vec<ElisionIn>,
    #[serde(default)]
    form: Vec<FormIn>,
    #[serde(default)]
    refused: Vec<RefusedIn>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GameFile {
    version: u32,
    stats: String,
    source: String,
    #[allow(dead_code)]
    generated_by: String,
    #[serde(default)]
    text: Vec<GameText>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GameText {
    text: String,
    forms: Vec<GameForm>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GameForm {
    form: String,
    kind: String,
    number: Option<f64>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OppositeIn {
    words: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ElisionIn {
    key: String,
    drop: String,
    at: Option<String>,
    number: f64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FormIn {
    text: String,
    form: String,
    kind: String,
    number: Option<f64>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RefusedIn {
    text: String,
    rule: String,
    form: Option<String>,
}

#[derive(Debug)]
struct Elision {
    key: String,
    drop: String,
    start: bool,
}

/// The parsed, checked table. Read-only; one per build.
#[derive(Debug)]
pub struct FormsTable {
    version: u32,
    pairs: Vec<(String, String)>,
    elisions: Vec<Elision>,
    /// The game's forms of every text the export joins, by the text: a
    /// text here takes these and no rule.
    game: HashMap<String, Vec<Form>>,
    /// The rows' forms by the text they are of.
    rows: HashMap<String, Vec<Form>>,
    refused: Vec<Refused>,
    /// A refused row by its text, and the rows by the key of the form
    /// each names.
    by_text: HashMap<String, usize>,
    by_form: HashMap<String, Vec<usize>>,
}

/// Why a table text is not a table. Every arm names the offending row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormsError {
    Parse(String),
    Version {
        found: u32,
        expected: u32,
    },
    Stats {
        found: String,
        expected: &'static str,
    },
    /// The stat table this one is checked against does not load.
    StatTable(String),
    NoSource,
    /// A pair that is not two words, or a word in two pairs.
    Pair(String),
    /// An elision whose key, drop or place is not one the header gives.
    Elision(String),
    /// A text the stat table does not list.
    NotListed(String),
    /// A form of a row that is not one the header gives, with why.
    Form {
        text: String,
        why: &'static str,
    },
    Refused {
        text: String,
        why: &'static str,
    },
}

impl std::fmt::Display for FormsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormsError::Parse(e) => write!(f, "the forms table does not parse: {e}"),
            FormsError::Version { found, expected } => write!(
                f,
                "the forms table is version {found}; this build reads version {expected}"
            ),
            FormsError::Stats { found, expected } => write!(
                f,
                "the forms table is of the stats capture of {found}; this build's is of {expected}"
            ),
            FormsError::StatTable(e) => {
                write!(f, "the forms table is checked by the stat table: {e}")
            }
            FormsError::NoSource => write!(f, "the forms table names no source"),
            FormsError::Pair(why) => write!(f, "the forms table's opposite words: {why}"),
            FormsError::Elision(why) => write!(f, "the forms table's elisions: {why}"),
            FormsError::NotListed(text) => write!(
                f,
                "the forms table names {text:?}, which the stat table lists nowhere"
            ),
            FormsError::Form { text, why } => {
                write!(f, "the forms table's form of {text:?} {why}")
            }
            FormsError::Refused { text, why } => {
                write!(f, "the forms table's refusal of {text:?} {why}")
            }
        }
    }
}

impl std::error::Error for FormsError {}

static TABLE: LazyLock<Result<FormsTable, FormsError>> = LazyLock::new(|| {
    let stats = stats::table().map_err(|e| FormsError::StatTable(e.to_string()))?;
    FormsTable::parse(FORMS_TOML, GAME_FORMS_TOML, stats)
});

/// The shipped table, parsed and checked once.
pub fn table() -> Result<&'static FormsTable, &'static FormsError> {
    TABLE.as_ref()
}

/// A text as the deriver tells two templates apart (the module doc): its
/// numbers `#`, a sign before a `#` dropped — the number's, never the
/// template's (C90) — case aside.
pub fn key(text: &str) -> String {
    let (template, _) = template::fixed(text);
    let signs = template::signs(&template);
    template
        .char_indices()
        .filter(|(at, _)| !signs.iter().any(|(s, _)| s == at))
        .map(|(_, c)| c)
        .collect::<String>()
        .to_lowercase()
}

/// How many `#`s a text leaves open: those the site writes, not a digit.
fn open(text: &str) -> usize {
    template::fixed(text)
        .1
        .iter()
        .filter(|n| n.is_none())
        .count()
}

/// The text the translation writes of a site text: its ` (Local)` dropped.
fn written(site: &str) -> &str {
    site.strip_suffix(LOCAL).unwrap_or(site)
}

/// A text's first letter a capital.
fn capital(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// Where a whole word stands in a text, by its byte offset.
fn words_at(text: &str, word: &str) -> Vec<usize> {
    let part = |c: char| c.is_alphanumeric();
    text.match_indices(word)
        .filter(|(at, _)| {
            !text[..*at].chars().next_back().is_some_and(part)
                && !text[at + word.len()..].chars().next().is_some_and(part)
        })
        .map(|(at, _)| at)
        .collect()
}

/// What the walk of the stat table found (`tests/trade_forms.rs` pins it).
#[derive(Debug, Default)]
pub struct Walk {
    /// For each rule — `opposite` and each elision's key — the texts it
    /// applies to.
    pub applies: Vec<(String, usize)>,
    /// What must be refused by name: (text, rule, the listed form).
    pub must_refuse: Vec<(String, String, Option<String>)>,
}

impl FormsTable {
    /// Parse and check a table text against the stat table it is of. The
    /// shipped file goes through [`table`]; this is the same path over any
    /// text, for the tests that pin what the loader refuses.
    pub fn parse(text: &str, game: &str, stats: &StatTable) -> Result<FormsTable, FormsError> {
        let file: File = toml::from_str(text).map_err(|e| FormsError::Parse(e.to_string()))?;
        let game: GameFile = toml::from_str(game).map_err(|e| FormsError::Parse(e.to_string()))?;
        if game.version != FORMS_VERSION {
            return Err(FormsError::Version {
                found: game.version,
                expected: FORMS_VERSION,
            });
        }
        if game.stats != stats::STAT_TABLE_VERSION {
            return Err(FormsError::Stats {
                found: game.stats,
                expected: stats::STAT_TABLE_VERSION,
            });
        }
        if game.source.trim().is_empty() {
            return Err(FormsError::NoSource);
        }
        if file.version != FORMS_VERSION {
            return Err(FormsError::Version {
                found: file.version,
                expected: FORMS_VERSION,
            });
        }
        if file.stats != stats::STAT_TABLE_VERSION {
            return Err(FormsError::Stats {
                found: file.stats,
                expected: stats::STAT_TABLE_VERSION,
            });
        }
        if file.source.trim().is_empty() {
            return Err(FormsError::NoSource);
        }
        let mut pairs = Vec::new();
        let mut words: HashSet<String> = HashSet::new();
        for pair in file.opposite {
            let [a, b] = pair.words.as_slice() else {
                return Err(FormsError::Pair(format!(
                    "{:?} is not two words",
                    pair.words
                )));
            };
            for w in [a, b] {
                if w.is_empty() || !w.chars().all(char::is_alphabetic) {
                    return Err(FormsError::Pair(format!("{w:?} is no word")));
                }
                if !words.insert(w.clone()) {
                    return Err(FormsError::Pair(format!("{w:?} is in two pairs")));
                }
            }
            pairs.push((a.clone(), b.clone()));
        }
        let mut elisions: Vec<Elision> = Vec::new();
        for e in file.elision {
            if e.key.is_empty() || elisions.iter().any(|x| x.key == e.key) || e.key == "opposite" {
                return Err(FormsError::Elision(format!(
                    "the key {:?} is empty, twice, or a pair's",
                    e.key
                )));
            }
            if !e.drop.contains('#') {
                return Err(FormsError::Elision(format!(
                    "{:?} drops {:?}, which holds no `#`",
                    e.key, e.drop
                )));
            }
            let start = match e.at.as_deref() {
                None => false,
                Some("start") => true,
                Some(other) => {
                    return Err(FormsError::Elision(format!(
                        "{:?} is at {other:?}, which is not `start`",
                        e.key
                    )));
                }
            };
            if !e.number.is_finite() {
                return Err(FormsError::Elision(format!("{:?} has no number", e.key)));
            }
            elisions.push(Elision {
                key: e.key,
                drop: e.drop,
                start,
            });
        }
        let listed: HashSet<&str> = stats.every_text().collect();
        let is_listed = |t: &str| listed.contains(t);
        // a form as a row or the export gives it, checked alike
        let checked = |text: &str,
                       form: String,
                       kind: &str,
                       number: Option<f64>,
                       by: &str|
         -> Result<Form, FormsError> {
            let why = |why: &'static str| FormsError::Form {
                text: text.to_string(),
                why,
            };
            if form.trim().is_empty() {
                return Err(why("is empty"));
            }
            if key(&form) == key(written(text)) {
                return Err(why("is its own text"));
            }
            let Some(kind) = Kind::ALL.iter().find(|(w, _)| *w == kind).map(|(_, k)| *k) else {
                return Err(why(
                    "has a kind none of plural, opposite, numberless, other",
                ));
            };
            if number.is_some() && kind != Kind::Numberless {
                return Err(why("names a number, which only a numberless form has"));
            }
            if kind == Kind::Plural && open(&form) != open(written(text)) {
                return Err(why("is plural and does not leave its text's `#`s open"));
            }
            Ok(Form {
                text: form,
                kind,
                by: by.to_string(),
            })
        };
        let mut game_forms: HashMap<String, Vec<Form>> = HashMap::new();
        for row in game.text {
            if !is_listed(&row.text) && !is_listed(&format!("{}{LOCAL}", row.text)) {
                return Err(FormsError::NotListed(row.text));
            }
            let mut forms: Vec<Form> = Vec::with_capacity(row.forms.len());
            for f in row.forms {
                let form = checked(&row.text, f.form, &f.kind, f.number, "game")?;
                if forms.iter().any(|g| key(&g.text) == key(&form.text)) {
                    return Err(FormsError::Form {
                        text: row.text.clone(),
                        why: "is given twice",
                    });
                }
                forms.push(form);
            }
            if game_forms.insert(row.text.clone(), forms).is_some() {
                return Err(FormsError::Form {
                    text: row.text,
                    why: "is joined twice",
                });
            }
        }
        let mut rows: HashMap<String, Vec<Form>> = HashMap::new();
        for row in file.form {
            if !is_listed(&row.text) {
                return Err(FormsError::NotListed(row.text));
            }
            let text = written(&row.text).to_string();
            let form = checked(&row.text, row.form, &row.kind, row.number, "form")?;
            let given = |forms: Option<&Vec<Form>>| {
                forms.is_some_and(|f| f.iter().any(|g| key(&g.text) == key(&form.text)))
            };
            if given(game_forms.get(&text)) {
                return Err(FormsError::Form {
                    text: row.text,
                    why: "is one the game's forms give",
                });
            }
            if given(rows.get(&text)) {
                return Err(FormsError::Form {
                    text: row.text,
                    why: "is given twice",
                });
            }
            rows.entry(text).or_default().push(form);
        }
        let mut refused = Vec::new();
        let (mut by_text, mut by_form) = (HashMap::new(), HashMap::new());
        for row in file.refused {
            if !is_listed(&row.text) {
                return Err(FormsError::NotListed(row.text));
            }
            let why = |why: &'static str| FormsError::Refused {
                text: row.text.clone(),
                why,
            };
            let known = ["opposite", "game", "form"].contains(&row.rule.as_str())
                || elisions.iter().any(|e| e.key == row.rule);
            if !known {
                return Err(why("names a rule the table does not have"));
            }
            match (&row.form, row.rule.as_str()) {
                (None, "opposite") => {}
                (Some(_), "opposite") | (None, _) => {
                    return Err(why(
                        "names a form for both words of a pair, or none for a listed one",
                    ));
                }
                (Some(form), _) => {
                    if !is_listed(form) {
                        return Err(FormsError::NotListed(form.clone()));
                    }
                    by_form
                        .entry(key(written(form)))
                        .or_insert_with(Vec::new)
                        .push(refused.len());
                }
            }
            if by_text
                .insert(written(&row.text).to_string(), refused.len())
                .is_some()
            {
                return Err(why("is given twice"));
            }
            refused.push(Refused {
                text: row.text,
                rule: row.rule,
                form: row.form,
            });
        }
        Ok(FormsTable {
            version: file.version,
            pairs,
            elisions,
            game: game_forms,
            rows,
            refused,
            by_text,
            by_form,
        })
    }

    /// The version every report that read the table cites.
    pub fn version(&self) -> u32 {
        self.version
    }

    /// The refusals, in the file's order.
    pub fn refused(&self) -> &[Refused] {
        &self.refused
    }

    /// The forms a site text displays as, but those a refusal names, and
    /// the refusals: the text's own — its form that is a listed text, or
    /// every opposite form of a text carrying both words of a pair — and
    /// any naming this text as the form another's would be.
    pub fn of(&self, site: &str) -> Forms<'_> {
        let text = written(site);
        let own = self.by_text.get(text).map(|&at| &self.refused[at]);
        let mut refused: Vec<&Refused> = own.into_iter().collect();
        refused.extend(
            self.by_form
                .get(&key(text))
                .into_iter()
                .flatten()
                .map(|&at| &self.refused[at]),
        );
        let known = self
            .made(text)
            .into_iter()
            .map(|(form, _)| form)
            .filter(|form| match own {
                None => true,
                Some(r) => match &r.form {
                    Some(listed) => key(&form.text) != key(written(listed)),
                    None => form.by != "opposite",
                },
            })
            .collect();
        Forms { known, refused }
    }

    /// Every form the rules and the rows make of a text, each with whether
    /// it is one a pair's both words refuse; none refused by name.
    fn made(&self, text: &str) -> Vec<(Form, bool)> {
        let mut out: Vec<(Form, bool)> = Vec::new();
        let rows = self.rows.get(text).into_iter().flatten().cloned();
        // a text the export joins takes the game's forms, and no rule
        if let Some(game) = self.game.get(text) {
            out.extend(game.iter().cloned().chain(rows).map(|f| (f, false)));
            return out;
        }
        // the opposite words, on a text with a `#` the site leaves open
        if open(text) > 0 {
            let mut at: Vec<(usize, &str, &str)> = Vec::new();
            let mut both = false;
            for (a, b) in &self.pairs {
                let (on_a, on_b) = (words_at(text, a), words_at(text, b));
                both |= !on_a.is_empty() && !on_b.is_empty();
                at.extend(on_a.into_iter().map(|p| (p, a.as_str(), b.as_str())));
                at.extend(on_b.into_iter().map(|p| (p, b.as_str(), a.as_str())));
            }
            at.sort();
            // every combination of the words flipped, none left alone
            for mask in 1u32..(1 << at.len().min(8)) {
                let mut flipped = String::with_capacity(text.len());
                let mut from = 0;
                for (n, (p, word, other)) in at.iter().enumerate() {
                    flipped.push_str(&text[from..*p]);
                    flipped.push_str(if mask & (1 << n) != 0 { other } else { word });
                    from = p + word.len();
                }
                flipped.push_str(&text[from..]);
                out.push((
                    Form {
                        text: flipped,
                        kind: Kind::Opposite,
                        by: "opposite".to_string(),
                    },
                    both,
                ));
            }
        }
        // a `+` before a `#` is spelling (C90), so a drop is found past it
        let unsigned = text.replace("+#", "#");
        for e in &self.elisions {
            let text = unsigned.as_str();
            let elided = if e.start {
                text.strip_prefix(e.drop.as_str()).map(str::to_string)
            } else {
                words_at(text, e.drop.split(' ').next().unwrap_or_default())
                    .into_iter()
                    .find(|p| text[*p..].starts_with(e.drop.as_str()))
                    .map(|p| format!("{}{}", &text[..p], &text[p + e.drop.len()..]))
            };
            if let Some(elided) = elided.filter(|t| !t.trim().is_empty()) {
                out.push((
                    Form {
                        text: capital(&elided),
                        kind: Kind::Numberless,
                        by: e.key.clone(),
                    },
                    false,
                ));
            }
        }
        out.extend(rows.map(|f| (f, false)));
        out
    }

    /// The walk of the stat table (the module doc): each rule's reach, and
    /// every form a rule or a row makes that is itself a listed text, and
    /// every text carrying both words of a pair — what must be refused by
    /// name, each once.
    pub fn walk(&self, stats: &StatTable) -> Walk {
        let mut listed: HashMap<String, Vec<&str>> =
            stats.every_text().fold(HashMap::new(), |mut m, t| {
                m.entry(key(written(t))).or_default().push(t);
                m
            });
        for texts in listed.values_mut() {
            texts.sort();
            texts.dedup();
        }
        let mut texts: Vec<&str> = stats.every_text().collect();
        texts.sort();
        texts.dedup();
        let mut applies: Vec<(String, usize)> = std::iter::once("opposite".to_string())
            .chain(self.elisions.iter().map(|e| e.key.clone()))
            .chain(["game".to_string(), "form".to_string()])
            .map(|r| (r, 0))
            .collect();
        let mut must: Vec<(String, String, Option<String>)> = Vec::new();
        for site in texts {
            let text = written(site);
            let own = key(text);
            let mut rules: Vec<String> = Vec::new();
            let mut both = false;
            for (form, refuses_both) in self.made(text) {
                if !rules.contains(&form.by) {
                    rules.push(form.by.clone());
                }
                both |= refuses_both;
                let at = key(&form.text);
                if at == own {
                    continue;
                }
                if let Some(other) = listed.get(&at) {
                    must.push((
                        site.to_string(),
                        form.by.clone(),
                        Some(other[0].to_string()),
                    ));
                }
            }
            for rule in rules {
                if let Some((_, n)) = applies.iter_mut().find(|(r, _)| *r == rule) {
                    *n += 1;
                }
            }
            if both {
                must.push((site.to_string(), "opposite".to_string(), None));
            }
        }
        must.sort();
        must.dedup();
        Walk {
            applies,
            must_refuse: must,
        }
    }
}
