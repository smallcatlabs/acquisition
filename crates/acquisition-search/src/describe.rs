//! `--describe` (C97: the closed lists are printed in the help): the
//! language as this build knows it — fields, what a line has, what has a
//! value, how terms compose, operators, closed value sets, slots, computed
//! values, what a count takes, what explains an answer, what is not built,
//! and the limits it states
//! (C102). An entry is a name, a line and, where one teaches, an example —
//! every construct has a case in `tests/language.toml` — the language on one
//! page, so that the owner's seat, an agent and the parser's round-trip
//! test read the same thing and nobody invents notation. Shapes bind;
//! flag and key names are the builder's, and this prints them. Every
//! example printed is a query this build binds, or a key a count takes
//! (`tests/surface_faults.rs`, `tests/counts.rs`), and every word an entry is
//! named by can be asked for alone. Printed from the binder's own tables,
//! so the help cannot say a name the binder refuses. The category ids are
//! rows of their own (`category.rs`): each with the site's text for it and
//! what makes it hold, printed whole and beside the field `category`. The
//! trade catalogue is a block of its own (C113; `trade/catalogue.rs`):
//! every clause a site search can carry, its status and the text it
//! becomes, printed whole and where `trade` is named; the index names it
//! in one line.

use serde::Serialize;

use crate::bind::{self, ITEM_FLAGS, Kind, LINE_FLAGS, NOT_BUILT, NotBuilt, SOURCES, Thing};
use crate::error::{ErrorKind, LanguageError};

#[derive(Debug, Clone, Serialize)]
pub struct Describe {
    pub fields: Vec<Named>,
    /// What `line( … )` takes inside.
    pub line: Vec<Named>,
    /// What `linked( … )` takes inside (C101; `sockets.rs`).
    pub linked: Vec<Named>,
    /// What a comparison, a sum or `--sort` consumes.
    pub values: Vec<Named>,
    /// How terms are put together.
    pub composition: Vec<Named>,
    pub operators: Vec<Named>,
    pub slots: Vec<Named>,
    /// `pseudo.<name>`: every named total with its definition, then the
    /// derived fields (C94, C101; `pseudo.rs`).
    pub computed: Vec<Named>,
    /// The totals table's version and source (C106's clause (c)), or why
    /// it does not load: in every answer, narrowed or whole (C53: an
    /// answer's keys only grow), and the section `totals` names.
    pub totals: String,
    /// What `--count`, `--cross` and `--sum` take (C95, C97).
    pub counts: Vec<Named>,
    /// What says why an answer is what it is: `--explain`, `--context`
    /// and `show --against`.
    pub explain: Vec<Named>,
    /// An answer's keys, in the order an answer prints them, and what
    /// each counts ([`answer_keys`]): the anatomy a frontend's tool text
    /// carried, read on demand beside the language (#83).
    pub answer: Vec<Named>,
    /// Every category id with the site's text for it and what makes it
    /// hold (C106; `category.rs`): printed whole, and beside the field
    /// `category` where that is named.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<crate::category::Described>,
    /// The trade catalogue (C113; `trade/catalogue.rs`): every clause a
    /// site search can carry, with what `--trade` makes of it — printed
    /// whole, and where `trade` is named.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade: Option<TradeCatalogue>,
    pub not_built: Vec<NotBuilt>,
    pub limits: Vec<Limit>,
}

/// The sections of the whole answer, by the keys it is serialised under,
/// in the struct's order: each can be asked for alone and answers with
/// its own key filled (`tests/surface_faults.rs`). The MCP's `describe`
/// lists them from here.
pub const SECTIONS: [&str; 16] = [
    "fields",
    "line",
    "linked",
    "values",
    "composition",
    "operators",
    "slots",
    "computed",
    "totals",
    "counts",
    "explain",
    "answer",
    "categories",
    "trade",
    "not_built",
    "limits",
];

#[derive(Debug, Clone, Serialize)]
pub struct Named {
    pub name: String,
    /// `text`, `number`, `closed set`, `id`, `flag`.
    pub kind: &'static str,
    pub what: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<&'static str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<&'static str>,
}

/// The trade catalogue as `--describe trade` prints it: what an answer's
/// report of a translation says ([`trade_report`]), then its version, the
/// stat table's beside it, and a row per clause.
#[derive(Debug, Clone, Serialize)]
pub struct TradeCatalogue {
    /// The report an answer carries in `query.trade`, key by key: the
    /// words that tell a translation's fidelity from the site's items.
    pub report: Vec<Named>,
    pub version: u32,
    /// The stat table's version: the capture its ids and texts are from.
    pub stats: String,
    pub rows: Vec<TradeRow>,
}

/// One clause of a site search: its key as the query object spells it,
/// the site's label, its status when it runs, the text it becomes — with
/// the placeholders the catalogue's header defines — and what it says,
/// as the report says it, and its other reading, where it has one, whose
/// stake an answer counts (C119); the row's note, its evidence, is the
/// catalogue file's.
#[derive(Debug, Clone, Serialize)]
pub struct TradeRow {
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lands: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub says: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,
}

fn trade_catalogue() -> Result<TradeCatalogue, LanguageError> {
    let defect = |e: String| LanguageError::new(ErrorKind::Trade, e);
    let catalogue = crate::trade::catalogue::catalogue().map_err(|e| defect(e.to_string()))?;
    let stats = crate::trade::stats::table().map_err(|e| defect(e.to_string()))?;
    Ok(TradeCatalogue {
        report: trade_report(),
        version: catalogue.version(),
        stats: stats.version().to_string(),
        rows: catalogue
            .rows()
            .iter()
            .map(|row| TradeRow {
                key: row.key.clone(),
                text: row.text.clone(),
                status: row.disposition.word(),
                lands: row.lands.clone(),
                says: row.says.clone(),
                other: row.other.clone(),
            })
            .collect(),
    })
}

/// One limit of the digest's register, in its wording (C102). The
/// register numbers each (`S12` the veiled line's, `S52` a line's, `S53`
/// the class's, `S107` an unknown line's, `S177` league of origin's,
/// `S178` legacy's); the number is the builders' and never printed (C116).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Limit {
    pub said: &'static str,
}

/// A veiled line's limit (S12), which a veiled line in `show` carries.
pub const VEILED: &str =
    "a veiled line is shown as the placeholder it is; a value query never matches it";

/// A line's limit (S52), which the `template` entry carries.
pub const AS_DISPLAYED: &str = "a line is matched as displayed text and value; which mods made it is not known and never guessed";

/// The limits this build can meet (`search/DIGEST.md`, "Limits
/// register"), in the register's words.
pub const LIMITS: &[Limit] = &[
    Limit { said: VEILED },
    Limit { said: AS_DISPLAYED },
    Limit {
        said: "an item's class is a derivation the search owns — read from its base by the class table, never a fact GGG gives — and an item the table cannot class is undecided with its reason",
    },
    Limit {
        said: crate::answer::S107,
    },
    Limit {
        said: "league of origin is not a fact the search holds; a derivation from league-specific mods or first-seen is offered as a derivation, never as the answer",
    },
    Limit {
        said: "legacy is the user's knowledge; the search finds what the user names and never says whether the game can still produce it",
    },
];

/// An answer's keys, in the order an answer prints them: the words the
/// MCP's `search` text carried until #83, cut, each fact a seat stopped
/// on kept (`acquisition-mcp/tests/search_tools.rs` reads them here). A
/// key nested in another is named by its path. `tests/surface_faults.rs`
/// holds every key an answer prints to an entry here.
pub fn answer_keys() -> Vec<Named> {
    let key = |name: &str, what: &str| named(name, "key", what);
    vec![
        key(
            "query",
            "the text as understood, and its tree; a trade site search's report is `query.trade`, which the trade section explains",
        ),
        key(
            "set_aside",
            "under the corpus context, the query typed beside it: read, set aside, and counted nowhere",
        ),
        key(
            "scope",
            "what was searched: account, realm, `items` every live item in scope before any term — under membership all and removed `items` counts the live and `removed` the removed, whichever were searched — and what was and was not fetched: `fetched` counts the tabs, their pages and the characters fetched, `oldest_fetch` and `newest_fetch` the earliest and the latest of their last fetches, each one location's, where a row's `place.fetched_at` is its own, and `list_seen` when the oldest location list read was last seen. A time is seconds since 1970, UTC, and beside each the answer says as an age its words stand under the time's name and `_age` (`oldest_fetch_age`: `45s ago`, `12m ago`, `5h ago`, `9d ago`, or `never`), made when the answer was",
        ),
        key(
            "scope.never_fetched",
            "`never_fetched` counts the tabs, their pages and the characters a listing names and no fetch has filled, and no answer says what they hold, a count of none or of many",
        ),
        key(
            "scope.never_fetched_by_type",
            "`never_fetched_by_type` is the same by tab type, a character's `character`: each type that has a location never fetched, its `tabs` every location of the type and `fetched` how many of them were, so `tabs` minus `fetched` were never fetched, and `items` the live items in the fetched ones, counted as `tabs` counts a type, over the scope's realm in every league whatever the query asks, since what a never-fetched location holds is unknown; `tabs` lists a league's tabs",
        ),
        key(
            "scope.never_fetched_leagues",
            "`never_fetched_leagues` names the leagues that hold a never-fetched tab, the most first",
        ),
        key(
            "total",
            "the matches: `matched`, and `undecided` the items the query could not decide, with why in `undecided_reasons`",
        ),
        key(
            "before",
            "where a request is sent back with the `count` it was printed with, and the `counted_at` basis it came from: where the count has moved since",
        ),
        key(
            "rows",
            "id, name, base, rarity, place by name and id — a tab's page with no name of its own has `name` null and its tab in `parent` — with the cell the item sits in (`place.cell`, GGG's x and y, in a tab or an inventory) and when that place was last fetched (`place.fetched_at`, in words `place.fetched_at_age`), and under membership all and removed when the item left (`removed_at`, in words `removed_at_age`), and the lines each term touched with their values, the term named by its `path` in `terms`, never the item's body, which is `show`",
        ),
        key(
            "view",
            "the view as it ran: rows, the default, ten where no limit is given, sorted by a value; counts, a table for each key and no rows, a sum's `value` over the items that have the value and its `lacking` counting the items without it, which add nothing; cross, one crossed table of two keys; explain, the total with one node forced true and forced false, the node named by its path as `terms[].path` gives it. A key is one entry of `keys`: a terminal's list `line:resist,life` is [\"line:resist\", \"line:life\"], and a comma inside an entry is part of its text; the counts section lists the keys. A sort's value is a field (reqlevel), a computed value (pseudo.total_res), a line's number (line(\"# to maximum Life\").arg1) or a sum of one (`sum(\"# to maximum Life\")`), spanning every source unless `source=` names one; an item that lacks it is last, ascending or descending",
        ),
        key(
            "view.rows.next",
            "rows past the limit are counted in `view.rows.left_out`, and `view.rows.next` is the request for the rows after: send it back whole",
        ),
        key(
            "zero",
            "in place of rows when the total is zero: `zero.offers`, requests to send instead, each sent back as printed",
        ),
        key(
            "terms",
            "for each term how many items matched, failed, lacked the thing or were undecided — each term alone over every item in scope, never over the matches, which are `total`; a pattern term's `resolved` is what it matched there, its `rest.denominator` `scope`",
        ),
        key(
            "basis",
            "what the answer was computed from: its `intent` is the revision of the account's hand-written prices and sync policy, which every write to either moves",
        ),
        key(
            "request",
            "a count holds the request that returns exactly its members where a next step hangs on it — what is undecided, what holds only together, what a key could not place (`none`, `undecided`), what an ambiguous trade clause's other reading adds or drops, and each term's matched where nothing matched — with `denominator`, what the request searches: the scope, never the share the count is of; every other count is its number alone, and routes asks for every count's. Any request an answer holds is sent back as printed",
        ),
    ]
}

/// What an answer's `query.trade` reports of a site search's translation,
/// key by key: the words the MCP's `query` argument carried until #83,
/// cut, opening the trade section before the catalogue's rows.
pub fn trade_report() -> Vec<Named> {
    let key = |name: &str, what: &str| named(name, "key", what);
    vec![
        key(
            "query.trade",
            "a site search — its URL, its id or its query JSON as a string, or the query JSON as an object (PoE1) — translated with no network to the text a person would type, and reported: every clause with its status — exact, ambiguous, unsupported, listing_only, disabled; an exact one with a term of its own its `at`, `term` and `path` alone, any other — a sum's filter, a group, a clause not exact — the site's filter and what the catalogue says of it. A URL's realm is the scope; an id or JSON takes a realm as a text does, and a `league` beside the JSON's `query` is a term, as a URL's league is; `defences: \"displayed\"` beside it reads the four defence filters as the item stands, where the site reads them at 20 % quality",
        ),
        key("remainder", "the clauses that did not run"),
        key(
            "fidelity",
            "how far the text is the site's search: exact, ambiguous where a clause ran as the site may not read it, or partial where the remainder is not empty, the answer then not the site's search. Never how far the items are the site's: the answer is over the items the store has fetched, priced or not — what it never fetched is `scope.never_fetched` — and the site shows public listings, a tab's being `tab.public`",
        ),
        key(
            "other",
            "an ambiguous clause whose other reading is known carries `other`: that reading, and the items in scope it `adds` and `drops` against the query as run",
        ),
    ]
}

fn named(name: &str, kind: &'static str, what: &str) -> Named {
    Named {
        name: name.to_string(),
        kind,
        what: what.to_string(),
        values: Vec::new(),
        examples: Vec::new(),
    }
}

/// The whole description, or the entries named — a field, `line`, a line's
/// attribute, `is`, `has`, an operator, a slot word (`arg7` is one), a word
/// of composition, a block by its name. An unknown name is an authoring
/// error with the near ones offered.
pub fn describe(names: &[String]) -> Result<Describe, LanguageError> {
    let mut fields: Vec<Named> = bind::fields()
        // the four colour fields are one entry, `sockets.<colour>`
        .filter(|f| !matches!(f.thing, Thing::SocketColour(_)))
        .map(|f| {
            let (kind, values) = match f.kind {
                Kind::Text => ("text", Vec::new()),
                Kind::Number => ("number", Vec::new()),
                Kind::Handle => ("id", Vec::new()),
                Kind::Presence => ("flag", Vec::new()),
                Kind::Closed(list) => ("closed set", list().to_vec()),
            };
            // the class prints its definition and its source version
            // (C106's clause (c)); a table that does not load says so
            let (what, examples) = match f.thing {
                Thing::Class => (
                    match crate::class::table() {
                        Ok(table) => table.definition(),
                        Err(e) => {
                            format!("{} — the table this build ships does not load: {e}", f.what)
                        }
                    },
                    vec![
                        "class:ring",
                        "class=Rings",
                        "class:sword",
                        "undecided(class)",
                    ],
                ),
                // the category prints its definition and its tables'
                // sources and versions, as the class does
                Thing::Category => (
                    match crate::category::tables() {
                        Ok(tables) => tables.definition(),
                        Err(e) => {
                            format!("{} — the tables this build ships do not load: {e}", f.what)
                        }
                    },
                    vec![
                        "category=armour",
                        "category=\"accessory.ring\"",
                        "category:beast",
                        "(category=armour or category=accessory)",
                        "-has:category",
                        "undecided(category)",
                    ],
                ),
                Thing::ReqLevel => (f.what.to_string(), vec!["reqlevel=..30", "-has:reqlevel"]),
                Thing::Sockets => (f.what.to_string(), vec!["sockets>=5", "-has:sockets"]),
                Thing::Links => (f.what.to_string(), vec!["links=6", "-has:links"]),
                Thing::Price => (
                    f.what.to_string(),
                    vec!["has:priced", "-has:priced", "undecided(priced)"],
                ),
                Thing::PriceAmount => (
                    f.what.to_string(),
                    vec!["price.amount>=5", "price.amount=1..10"],
                ),
                Thing::PriceCurrency => (
                    f.what.to_string(),
                    vec!["price.currency=chaos", "price.currency:div"],
                ),
                Thing::PriceLot => (f.what.to_string(), vec!["price.lot>=2", "-has:price.lot"]),
                Thing::PriceSide => (
                    f.what.to_string(),
                    vec!["price.side=game", "price.side=manual"],
                ),
                Thing::Tab => (f.what.to_string(), vec!["tab:dump", "tab=\"Tier 1\""]),
                Thing::TabType => (
                    f.what.to_string(),
                    vec!["tab.type=MapStash", "tab.type:map", "-has:tab.type"],
                ),
                Thing::TabPublic => (
                    f.what.to_string(),
                    vec!["tab.public=true", "tab.public=false", "-has:tab.public"],
                ),
                Thing::TabRemoveOnly => (f.what.to_string(), vec!["tab.remove_only=false"]),
                Thing::Text => (
                    f.what.to_string(),
                    vec!["text:life", "text~\"^Item Level\""],
                ),
                Thing::Name => (
                    f.what.to_string(),
                    vec!["name:kaom", "name=\"Kaom's Heart\"", "name~\"^Kaom\""],
                ),
                Thing::Typeline => (f.what.to_string(), vec!["typeline:staunching"]),
                Thing::Base => (f.what.to_string(), vec!["base=\"Divine Life Flask\""]),
                Thing::Frame => (f.what.to_string(), vec!["frame=gem"]),
                Thing::Ilvl => (
                    f.what.to_string(),
                    vec!["ilvl>=84", "ilvl=80..84", "ilvl=..84"],
                ),
                // a property field prints its row's definition and the
                // table's version (`properties.rs`, C114)
                Thing::Property(field) => (f.what.to_string(), field.examples()),
                _ => (f.what.to_string(), Vec::new()),
            };
            Named {
                values,
                examples,
                ..named(f.name, kind, &what)
            }
        })
        .collect();
    fields.push(Named {
        values: crate::sockets::colour_words(),
        examples: vec!["sockets.red>=2", "sockets.blue=0"],
        ..named(
            "sockets.<colour>",
            "number",
            "how many sockets of one colour the item has, over every link group; GGG's letter for the word (red R, green G, blue B, white W), so an abyssal socket (A) or a resonator's (DV) counts in `sockets` and under no colour",
        )
    });
    fields.push(Named {
        values: ITEM_FLAGS.to_vec(),
        examples: vec!["is:corrupted", "-is:identified"],
        ..named(
            "is",
            "flag",
            "every yes the item's body says, as GGG spells it; `-is:` is no, silence is any",
        )
    });
    fields.push(Named {
        values: bind::fields()
            .map(|f| f.name)
            .filter(|n| !matches!(*n, "text" | "id"))
            .collect(),
        examples: vec!["has:note", "-has:name"],
        ..named(
            "has",
            "flag",
            "presence; absence only when all that could hold the thing was readable",
        )
    });
    let line = vec![
        Named {
            examples: vec![
                "line(\"# to maximum Life\" arg1>=90)",
                "line(template:resistance is:fractured)",
                "line(template~\"^Adds\")",
            ],
            ..named(
                "template",
                "text",
                &format!(
                    "the displayed line with each number a #, the sign in the number — a + typed before a #, as the game and the trade site write a line, is spelling and is dropped; a bare \"T\" means template=\"T\". A mod displayed over several rows is one line: `=` names it by its whole text, a row break written \\n, or by any one of its rows, and named by a row its numbers are that row's; `:` and `~` test the whole text, and there a pattern's ^ and $ are the ends of that text. {}",
                    AS_DISPLAYED
                ),
            )
        },
        Named {
            values: SOURCES.to_vec(),
            examples: vec!["line(\"# to maximum Life\" source=explicit)"],
            ..named(
                "source",
                "closed set",
                "the array the line came in, without its `Mods`; hybrid is a vaal gem's base skill",
            )
        },
        Named {
            values: LINE_FLAGS.to_vec(),
            examples: vec!["line(template:life -is:crafted)"],
            ..named("is", "flag", "the flags the body sets on the line")
        },
    ];
    let linked = vec![
        Named {
            values: crate::sockets::colour_words(),
            examples: vec!["linked(red>=3 green>=1)", "linked(blue>=2 or white>=1)"],
            ..named(
                "red green blue white",
                "number",
                "how many sockets of the colour one link group holds; every count named holds together on one group",
            )
        },
        Named {
            examples: vec!["linked(size>=5 blue>=2)", "-has:links"],
            ..named(
                "size",
                "number",
                "how many sockets the link group holds; an item with no link group lacks every `linked( … )`",
            )
        },
    ];
    let values = vec![
        Named {
            examples: vec![
                "line(\"Adds # to # Cold Damage\").avg>=20",
                "\"# to maximum Life\">=90",
            ],
            ..named(
                "line(P).<slot>",
                "value",
                "one occurrence's number: a comparison on it means what it means inside the group, and `\"T\">=90` is its shorthand; `--sort` takes it, the largest that satisfies P",
            )
        },
        Named {
            examples: vec![
                "sum(\"# to maximum Life\")>=90",
                "sum(line(template:resistance).arg1)>=60",
            ],
            ..named(
                "sum",
                "value",
                "the item's sum of a slot over the occurrences P selects, exact in decimals; nothing sums to zero; with a possible contributor unread it is incomplete and its comparison undecided. A sum over lines a pattern selects is your arithmetic, never a reviewed total: a line naming all three elements counts once in it and three times in pseudo.total_res",
            )
        },
    ];
    let composition = vec![
        Named {
            examples: vec!["rarity=rare class:ring", "rarity=rare and class:ring"],
            ..named(
                "and",
                "composition",
                "whitespace or the word, at the level you are in; `a b or c` unparenthesised is an error showing both readings",
            )
        },
        Named {
            examples: vec!["class:ring or class:amulet"],
            ..named("or", "composition", "either")
        },
        Named {
            examples: vec!["-is:corrupted", "not has:note"],
            ..named(
                "not -",
                "composition",
                "the term is false; at a terminal a query that starts with - goes after --",
            )
        },
        Named {
            examples: vec!["(class:ring or class:amulet) rarity=rare"],
            ..named(
                "( )",
                "composition",
                &format!(
                    "grouping, and nothing else; a query nests at most {} deep, each `( )`, `not`, `holds( )` and `undecided( )` around a term one level",
                    crate::tree::MAX_DEPTH
                ),
            )
        },
        Named {
            examples: vec!["holds(rarity=rare, is:corrupted, has:note)>=2"],
            ..named(
                "holds",
                "composition",
                "how many of its queries hold: >=n, <=n, =n, =a..b",
            )
        },
        Named {
            examples: vec!["undecided(ilvl)", "undecided(\"# to maximum Life\">=90)"],
            ..named(
                "undecided",
                "composition",
                "true when a thing's value or a term's truth cannot be established on the item — something it needs was unread; always decided itself",
            )
        },
        named("true() false()", "composition", "always true, always false"),
    ];
    let operators = vec![
        named(
            ":",
            "operator",
            "contains, in any case; on a closed set, picks among its legal values",
        ),
        named(
            "=",
            "operator",
            "the whole value, in any case; with a..b, a number inside the range, a side may be blank",
        ),
        named(
            "~",
            "operator",
            "a pattern: Rust regex syntax, unanchored, any case unless it says (?-i); ^ and $ are the ends of one displayed string. Patterns are for words, comparisons for numbers",
        ),
        named("> >= < <=", "operator", "a number against a number"),
    ];
    let slots = vec![
        named(
            "arg1 arg2 …",
            "slot",
            "every number of a template by position, counting from 1",
        ),
        named(
            "low high avg",
            "slot",
            "on a ranged line — a template with exactly one `# to #`: that pair, and their mean; avg also on a template of exactly two numbers, their mean",
        ),
    ];
    let keys: Vec<&'static str> = bind::fields()
        .filter(|f| !matches!(f.name, "text" | "id" | "priced"))
        .map(|f| f.name)
        .chain(["line"])
        .collect();
    let counts = vec![
        Named {
            values: keys,
            examples: vec!["tab,league,rarity", "base", "line", "line:resist,life"],
            ..named(
                "--count",
                "view",
                "the matches counted by each key, one table each, no rows: a field an item has or lacks — under `none` with none, `undecided` where it could not be read — or `line`, the vocabulary (every count has its route)",
            )
        },
        Named {
            examples: vec!["line:resist,life", "line~^adds", "line:life,~^adds"],
            ..named(
                "line:",
                "view",
                "the vocabulary narrowed: the templates the matching items carry whose text holds the words, ranked, with the term that selects each, its numbers' range, and its sources and flags; `line~` a pattern; the rest of the list is texts, one table each. Beside them, marked computed, the computed values whose name or own definition holds the words — a total's rows and its limit, never the rule of what no total reads, which every total's definition prints — and none under `line` alone, which `--describe` names; each counts the matches that carry it, as `has:` of it asks",
            )
        },
        Named {
            examples: vec!["league,tab", "rarity,base"],
            ..named(
                "--cross",
                "view",
                "one table of two fields: the cells that hold an item, each routed under both",
            )
        },
        Named {
            examples: vec!["stack", "ilvl", "sum(\"# to maximum Life\")"],
            ..named(
                "--sum",
                "value",
                "beside each count, one number of each item added: an item lacking it adds nothing and is counted beside the sum as lacking it, so the sum is over the items that have it; one unread leaves a subtotal marked incomplete, never a total",
            )
        },
    ];
    let explain = vec![
        named(
            "--explain",
            "view",
            "one node of the query, named by its path: the root's matched and undecided counts as given, with the node forced true, and with it forced false, each beside its transformed query — the query with that one node replaced by true() or false() and nothing else changed, answered as any query is. A path is a term's as the terms block prints it (3), a group of terms', or a condition's inside line( … ) or linked( … ), numbered under its term as a group's children are (3.1); the conditions inside sum( … ) take none; a path the query lacks lists the query's nodes with their paths. Forced is literal, never removed and never a contribution: `a or b` on an item that holds both loses nothing when a is forced false. Counts and no rows: an error beside --count, --cross, --sum or --sort",
        ),
        Named {
            values: vec!["matches", "corpus"],
            ..named(
                "--context",
                "scope",
                "what the answer is of: matches, the query's, as every answer is; corpus, the empty query over the same scope under any view — rows, a count, a crossed table — with the query typed read, printed on one line as set aside, and counted nowhere",
            )
        },
        named(
            "show --against",
            "show",
            "beside one item, a query's canonical text, the root's outcome on the item and each node's — a term's `matched`, `failed`, `lacked` or `undecided`, a group's `match`, `no_match` or `undecided` — a term with what a row shows of it where it matched, what the item has where it failed, and the reason where it is undecided; at the basis the item was read at",
        ),
    ];
    let mut out = Describe {
        fields,
        line,
        linked,
        values,
        composition,
        operators,
        slots,
        computed: crate::pseudo::described(),
        totals: crate::pseudo::provenance(),
        counts,
        explain,
        answer: answer_keys(),
        categories: crate::category::tables()
            .map(|tables| tables.described())
            .unwrap_or_default(),
        trade: Some(trade_catalogue()?),
        not_built: NOT_BUILT.to_vec(),
        limits: LIMITS.to_vec(),
    };
    if names.is_empty() {
        return Ok(out);
    }
    // narrow to what was named: an entry answers to its name and to each
    // word of it, a computed value to its name without `pseudo.`, the
    // positional slots to any `arg<N>`, a block to its own
    let answers = |entry: &str, asked: &str| {
        entry.eq_ignore_ascii_case(asked)
            // a dotted entry answers to its head: `sockets.<colour>` to `sockets`
            || entry
                .split_once('.')
                .is_some_and(|(head, _)| head.eq_ignore_ascii_case(asked))
            // and `sockets.<colour>` to each of the four (an outside review, 2026-09-24, 4)
            || (entry == "sockets.<colour>"
                && asked
                    .split_once('.')
                    .is_some_and(|(head, word)| {
                        head.eq_ignore_ascii_case("sockets")
                            && crate::sockets::letter(word).is_some()
                    }))
            // a flag answers to its word: `count` is `--count`
            || entry.trim_start_matches('-').eq_ignore_ascii_case(asked)
            || entry
                .strip_prefix("pseudo.")
                .is_some_and(|name| name.eq_ignore_ascii_case(asked))
            || entry
                .split_whitespace()
                .any(|word| word != "…" && word.eq_ignore_ascii_case(asked))
            // a flag among an entry's words answers to its word too:
            // `against` is `show --against`
            || entry.split_whitespace().any(|word| {
                word.strip_prefix("--")
                    .is_some_and(|flag| flag.eq_ignore_ascii_case(asked))
            })
            || (entry.starts_with("arg1") && crate::tree::arg_index(asked).is_some())
    };
    // every section by its key, and `pseudo` names the computed block, as
    // the namespace does
    let blocks = || SECTIONS.into_iter().chain(["pseudo"]);
    let entries = |d: &Describe| -> Vec<String> {
        [
            &d.fields,
            &d.line,
            &d.linked,
            &d.values,
            &d.composition,
            &d.operators,
            &d.slots,
            &d.computed,
            &d.counts,
            &d.explain,
            &d.answer,
        ]
        .into_iter()
        .flatten()
        .map(|n| n.name.clone())
        .collect()
    };
    let all = entries(&out);
    for name in names {
        let known = blocks().any(|b| b.eq_ignore_ascii_case(name))
            || all.iter().any(|entry| answers(entry, name));
        if !known {
            if let Some(unbuilt) = NOT_BUILT.iter().find(|n| {
                let construct = n.construct.trim_end_matches([':', '*', '.']);
                construct.eq_ignore_ascii_case(name)
                    || construct
                        .strip_prefix("pseudo.")
                        .is_some_and(|c| c.eq_ignore_ascii_case(name))
            }) {
                return Err(bind::not_built(unbuilt.construct));
            }
            let words: Vec<&str> = all
                .iter()
                .flat_map(|entry| entry.split_whitespace())
                .filter(|word| *word != "…")
                .chain(blocks())
                .collect();
            let near = bind::near(name, &words);
            return Err(LanguageError::new(
                ErrorKind::UnknownName,
                format!("`{name}` is nothing describe knows: {}", words.join(", ")),
            )
            .with_readings(near.into_iter().map(str::to_string).collect()));
        }
    }
    let wants = |block: &str, entry: &str| {
        names
            .iter()
            .any(|asked| asked.eq_ignore_ascii_case(block) || answers(entry, asked))
    };
    out.fields.retain(|n| wants("fields", &n.name));
    out.line.retain(|n| wants("line", &n.name));
    out.linked.retain(|n| wants("linked", &n.name));
    out.values.retain(|n| wants("values", &n.name));
    out.composition.retain(|n| wants("composition", &n.name));
    out.operators.retain(|n| wants("operators", &n.name));
    // a line's numbers are named by its slots
    out.slots
        .retain(|n| wants("slots", &n.name) || wants("line", &n.name));
    out.computed
        .retain(|n| wants("computed", &n.name) || wants("pseudo", &n.name));
    out.counts.retain(|n| wants("counts", &n.name));
    out.explain.retain(|n| wants("explain", &n.name));
    out.answer.retain(|n| wants("answer", &n.name));
    let named = |block: &str| names.iter().any(|asked| asked.eq_ignore_ascii_case(block));
    // the ids are the field's: printed where `category` is named
    if !named(crate::category::FIELD) && !named("categories") {
        out.categories.clear();
    }
    if !named("trade") {
        out.trade = None;
    }
    if !named("not_built") {
        out.not_built.clear();
    }
    if !named("limits") {
        out.limits.clear();
    }
    Ok(out)
}
