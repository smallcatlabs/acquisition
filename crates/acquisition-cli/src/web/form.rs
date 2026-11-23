//! The search form and its fold (C139): the form generated from the
//! committed filter dump, each control named by its path in the site's
//! query JSON, and the submission folded back into that JSON by those
//! names.
//!
//! # As built
//!
//! - **The form is the dump's.** The trade site's filter definitions of
//!   2026-11-17, shipped by the search crate
//!   (`acquisition_search::trade::catalogue::FILTERS_JSON`), are
//!   compiled into the binary and read once when the server starts: a
//!   control per filter, in the dump's groups and order, its label the
//!   site's. No build step writes it and no page reads the dump.
//! - **Each control's fate is the catalogue's** (`--describe trade`),
//!   said where it is not exact: beside it, its row's status, its `says`
//!   as the title; an option whose own row is not exact carries that
//!   row's status in its label, as a stat group's type and its "Add Stat
//!   Group" control do. A control that runs as the site's says nothing. A
//!   listing-only row is rendered disabled: a browser submits nothing of
//!   a disabled control, so the fold never meets one.
//! - **A name is a path**: dot-separated keys —
//!   `query.filters.type_filters.filters.category.option`. No name carries
//!   an index.
//! - **The stat fields are positional**, as `view.value` is: `group.type`
//!   begins a stat group and `stat.id` a row in the group last begun, each
//!   by its presence, its value empty or not; `group.min`, `group.max` and
//!   `group.disabled` belong to the group last begun, and `stat.min`,
//!   `stat.max`, `stat.weight`, `stat.option` and `stat.disabled` to the
//!   row last begun in it. A field before the thing it belongs to is a
//!   usage error. So the names stay a fixed set, a row added names
//!   nothing, and a row removed leaves no hole: a page's every link and
//!   hidden field keeps its fields' order. A value key a group's type does
//!   not take is dropped, asked of the translation's own lists
//!   (`stat_value_keys`, `group_value_keys`), never restated here.
//! - **A `disabled` field is an on box** (`page.rs`, `on_box`): a hidden
//!   `disabled=true`, then a box ticked for on that sends
//!   `disabled=false`. Every field is read in the order sent and the last
//!   stands, so a box ticked is on, and a `disabled` of false is dropped
//!   as the normal form drops it.
//! - **Other names are no path** and go elsewhere: `realm` and `membership` to the request's scope (the
//!   membership as the word typed, which the request's reader reads or
//!   refuses in the search's words), `league` and
//!   `defences` beside the query (C141, the league beside the trade JSON;
//!   C142, the defences' reading beside it), and `view.limit`,
//!   `view.sort`, `view.desc` and `view.value` to the rows view — the
//!   last one value each, in order, given again for the next as the CLI's
//!   `--value` is, since a value's text may hold a comma; no control of
//!   the form writes the three, which the URL names and a page's links and
//!   the query box carry — or `view.count`, a key, to the counts view in
//!   its place, whose tables the answer page prints as the CLI does; and
//!   `view.fields`, the row's fields as the CLI's `--fields` names them
//!   (`icon`, or `icon,item`), read by the search's one reader, which
//!   refuses a list naming no field: sent empty or not at all, the page's
//!   default, the item with its art, so the answer page renders cards;
//!   `icon`, the compact table. A count asks no row's fields, and one sent
//!   beside it is the view selection's refusal. A search sent with a
//!   count lets go of `view.sort`, `view.desc` and `view.value`, which a
//!   page carries hidden from the answer it stands under and a count shows
//!   none of (#304): no refusal comes of what a reader could not see. And
//!   `text`, the query box:
//!   a query in the language, run as the user's own in place of the trade
//!   input. And `trade`, the trade site box: a site search as the CLI's
//!   `--trade` takes it — its URL, its bare id or its query JSON — run in
//!   place of the form's trade input by the same rule (#188, its first
//!   half). A URL's realm is the search's scope, so where the box names one the
//!   page's carried `realm` is not read; the two boxes together, or the
//!   site box sent empty, are a usage error.
//! - **A name is one the form emitted**: the fold refuses any other as
//!   a usage error before it builds anything (C47, malformed external
//!   input is a structured error), so no path it never wrote can shape
//!   the JSON. The names are the form's own: a test holds the form, filled
//!   with every control, to them (`fill.rs`).
//! - **The fold drops what is empty**: every field submitted empty but
//!   `text`, then an object or array left with nothing in it, then a stat
//!   group left with no filter (its type alone says nothing). `text`
//!   sent is the text, empty or not: the text `""` is a query, every item
//!   in its scope, so a saved search over every item folds back. A `min`,
//!   `max`, `weight` or socket colour that reads as a number is one;
//!   anything else is the text typed, which the translation reports as it
//!   reads it. The fold decides nothing the translation decides. Its
//!   envelope is in one normal form ([`normal`]): a null, an empty string,
//!   a `disabled` of false and a `defences` of `q20`, the site's own
//!   reading, dropped, then what is left empty, then a stat group with no
//!   filter; the fill reads that form (`fill.rs`).
//! - **The quality box is shaped as an on box** (`page.rs`,
//!   `quality_box`): a hidden `defences=displayed`, then "assume 20%
//!   quality", ticked by default, sending `defences=q20`. The last sent
//!   stands, so the box unticked asks the defences as the item stands.

use std::collections::{BTreeMap, BTreeSet};

use acquisition_search::LanguageError;
use acquisition_search::answer::{RowField, Rows, Shape, View};
use acquisition_search::trade::catalogue::Disposition;
use serde_json::{Map, Value};

/// The dump this build's form is generated from, as the search crate
/// ships it.
const FILTERS: &str = acquisition_search::trade::catalogue::FILTERS_JSON;

/// The rows view's limits a form offers, the first the default.
pub(crate) const LIMITS: [usize; 4] = [20, 50, 100, 200];

/// One row of the catalogue as a control shows it: its status, and what
/// it says.
#[derive(Clone, Debug)]
pub(crate) struct Fate {
    pub status: String,
    pub says: Option<String>,
    /// The site's label as the catalogue writes it.
    pub text: Option<String>,
}

impl Fate {
    /// The catalogue's status as its word names it.
    fn disposition(&self) -> Option<Disposition> {
        [
            Disposition::Exact,
            Disposition::Ambiguous,
            Disposition::Unsupported,
            Disposition::ListingOnly,
        ]
        .into_iter()
        .find(|d| d.word() == self.status)
    }

    /// Whether the control runs as the site's: an exact one says nothing
    /// (`4-full-1-form-rides.md`, Decision 2).
    pub(crate) fn exact(&self) -> bool {
        self.disposition() == Some(Disposition::Exact)
    }

    /// Whether a clause of this status runs (`Disposition::runs`, exact or
    /// ambiguous): a control whose fate does not is marked, and the switch
    /// hides it (`4-full-1-form-rides.md`, Decision 4).
    pub(crate) fn runs(&self) -> bool {
        self.disposition().is_some_and(Disposition::runs)
    }

    /// The status in a label's words, ` (ambiguous)`, where it is not
    /// exact; nothing where it is.
    pub(crate) fn marked(&self) -> String {
        match self.exact() {
            true => String::new(),
            false => format!(" ({})", self.status.replace('_', " ")),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Form {
    pub groups: Vec<Group>,
    /// The stat group types, each with its fate: `stats.<type>`.
    pub stat_types: Vec<(String, Option<String>, Fate)>,
    /// The catalogue's rows by key.
    fates: BTreeMap<String, Fate>,
    /// Every field name the form emits: what the fold accepts.
    names: BTreeSet<String>,
}

#[derive(Debug)]
pub(crate) struct Group {
    pub id: String,
    pub title: Option<String>,
    /// The dump's `hidden`: a group the site shows closed until opened.
    pub hidden: bool,
    pub filters: Vec<Filter>,
}

#[derive(Debug)]
pub(crate) struct Filter {
    pub id: String,
    pub text: Option<String>,
    pub tip: Option<String>,
    pub options: Option<Vec<(Option<String>, String)>>,
    /// An option the site fills from its own item lists: a text field.
    pub known_item: bool,
    pub min_max: bool,
    pub sockets: bool,
    pub input: bool,
}

impl Form {
    /// The form from the dump and the catalogue this build ships: an
    /// error is a build defect, said when the server starts.
    pub(crate) fn load() -> anyhow::Result<Form> {
        let dump: Value = serde_json::from_str(FILTERS)?;
        let result = dump["result"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("the filter dump holds no `result` list"))?;
        let mut groups = Vec::new();
        for g in result {
            let filters = g["filters"]
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("a filter group holds no `filters` list"))?
                .iter()
                .map(|f| Filter {
                    id: f["id"].as_str().unwrap_or_default().to_string(),
                    text: f["text"].as_str().map(str::to_string),
                    tip: f["tip"].as_str().map(str::to_string),
                    options: f["option"]["options"].as_array().map(|options| {
                        options
                            .iter()
                            .map(|o| {
                                (
                                    o["id"].as_str().map(str::to_string),
                                    o["text"].as_str().unwrap_or_default().to_string(),
                                )
                            })
                            .collect()
                    }),
                    known_item: f["option"]["knownItem"].is_object(),
                    min_max: f["minMax"].as_bool().unwrap_or(false),
                    sockets: f["sockets"].as_bool().unwrap_or(false),
                    input: f["input"].is_object(),
                })
                .collect();
            groups.push(Group {
                id: g["id"].as_str().unwrap_or_default().to_string(),
                title: g["title"].as_str().map(str::to_string),
                hidden: g["hidden"].as_bool().unwrap_or(false),
                filters,
            });
        }
        let described = acquisition_search::describe(&["trade".to_string()])
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        let rows = described
            .trade
            .ok_or_else(|| anyhow::anyhow!("the trade catalogue is not described"))?
            .rows;
        let mut fates = BTreeMap::new();
        let mut stat_types = Vec::new();
        for row in rows {
            let fate = Fate {
                status: row.status.to_string(),
                says: row.says.clone(),
                text: row.text.clone(),
            };
            if let Some(kind) = row.key.strip_prefix("stats.")
                && !kind.contains('.')
            {
                stat_types.push((kind.to_string(), row.text.clone(), fate.clone()));
            }
            fates.insert(row.key, fate);
        }
        let names = emitted(&groups);
        Ok(Form {
            groups,
            stat_types,
            fates,
            names,
        })
    }

    /// The catalogue's row for a key, `query.status`, `<group>.<filter>`,
    /// `<group>.<filter>=<option>`.
    pub(crate) fn fate(&self, key: &str) -> Option<&Fate> {
        self.fates.get(key)
    }

    /// Every field name the fold accepts.
    #[cfg(test)]
    pub(crate) fn names(&self) -> &BTreeSet<String> {
        &self.names
    }

    /// The site's label for a key, as the catalogue writes it.
    pub(crate) fn text(&self, key: &str) -> Option<&str> {
        self.fates.get(key).and_then(|f| f.text.as_deref())
    }
}

/// A filter's path in the query, as the form names its fields.
pub(crate) fn filter_path(group: &str, filter: &str) -> String {
    if group == "status_filters" {
        "query.status".to_string()
    } else {
        format!("query.filters.{group}.filters.{filter}")
    }
}

/// The view's names no control of a form writes: the URL names them, and a
/// page's links and the query box carry them (nothing is typed to sort).
pub(crate) const UNTYPED: [&str; 3] = ["view.sort", "view.desc", "view.value"];

/// Every name the form emits: the scope and the view's, each filter's
/// by its kind, and the stat rows'; and the view's [`UNTYPED`] names.
fn emitted(groups: &[Group]) -> BTreeSet<String> {
    let mut names: BTreeSet<String> = [
        "realm",
        "membership",
        "league",
        "query.term",
        "view.limit",
        "view.count",
        "view.fields",
        "text",
        "trade",
        "defences",
    ]
    .iter()
    .chain(UNTYPED.iter())
    .map(|n| n.to_string())
    .collect();
    for group in groups.iter().filter(|g| g.id != "status_filters") {
        names.insert(format!("query.filters.{}.disabled", group.id));
    }
    for group in groups {
        for f in &group.filters {
            let path = filter_path(&group.id, &f.id);
            if f.options.is_some() || f.known_item || f.input {
                names.insert(format!("{path}.option"));
            }
            if f.min_max {
                names.insert(format!("{path}.min"));
                names.insert(format!("{path}.max"));
            }
            if f.sockets {
                for c in ["r", "g", "b", "w"] {
                    names.insert(format!("{path}.{c}"));
                }
            }
        }
    }
    names.extend(
        GROUP_FIELDS
            .iter()
            .chain(ROW_FIELDS.iter())
            .map(|n| n.to_string()),
    );
    names
}

/// A stat group's fields, positional: `group.type` begins a group, and the
/// rest belong to the group last begun (the module doc).
pub(crate) const GROUP_FIELDS: [&str; 4] =
    ["group.type", "group.min", "group.max", "group.disabled"];

/// A stat row's fields, positional: `stat.id` begins a row in the group
/// last begun, and the rest belong to the row last begun in it.
pub(crate) const ROW_FIELDS: [&str; 6] = [
    "stat.id",
    "stat.min",
    "stat.max",
    "stat.weight",
    "stat.option",
    "stat.disabled",
];

/// What a form submission asks: the trade JSON the fields fold into, and
/// the request's scope and view.
#[derive(Debug, PartialEq)]
pub(crate) struct Folded {
    pub realm: Option<String>,
    /// The scope's membership, as the word typed: the request's reader
    /// reads it or refuses it.
    pub membership: Option<String>,
    pub envelope: Value,
    pub limit: Option<usize>,
    pub sort: Option<String>,
    /// The values each row carries, one `view.value` each, in the order
    /// sent: the CLI's `--value`, given again for the next (a value's text
    /// may hold a comma, so no list).
    pub values: Vec<String>,
    pub desc: bool,
    /// A count by these keys in place of the rows: the counts view, the
    /// keys read as the CLI's `--count` reads its list (`count_keys`).
    pub count: Option<Vec<String>>,
    /// The row's fields where `view.fields` was sent with a value, read
    /// by the search's one reader; none where it was not, the default.
    pub fields: Option<Vec<RowField>>,
    /// The query box's text: where given,
    /// the search is that text, the user's own query, and the trade
    /// input is dropped — the rule the request already has for an edited
    /// text.
    pub text: Option<String>,
    /// The trade site box: a site search, its URL, its id or its query
    /// JSON, run in place of the form's trade input as an edited text is.
    pub trade: Option<String>,
}

impl Folded {
    /// The request the MCP's `search` reads, as the page asks it: the
    /// envelope as the trade input, each row with its icon (C140) and its
    /// item where no other fields were sent, none under a count. The view
    /// is built by the shared view selection, `Shape::of`, as the CLI
    /// builds its own, so the page inherits its refusals — a sort beside a
    /// count among them — and builds no view itself (C46). A site search
    /// whose URL names a realm runs there: the realm the page carried
    /// beside it is not read (the owner's words in #188, "the URL's realm
    /// is the search's scope"), and the answer page says so where they
    /// differ.
    pub(crate) fn request(&self) -> Result<Value, LanguageError> {
        let mut scope = Map::new();
        let named = self
            .trade
            .as_deref()
            .and_then(crate::search_cmd::trade_realm);
        if let Some(realm) = &self.realm
            && named.is_none()
        {
            scope.insert("realm".into(), Value::String(realm.clone()));
        }
        if let Some(membership) = &self.membership {
            scope.insert("membership".into(), Value::String(membership.clone()));
        }
        let shape = Shape::of(
            self.count.clone(),
            None,
            None,
            None,
            Rows {
                limit: self.limit,
                sort: self.sort.clone(),
                values: self.values.clone(),
                desc: self.desc,
                after: None,
                // the fields sent, which a count refuses; else none for a
                // count, which shows no rows, and the item with its art for
                // rows, the page's cards
                fields: match (&self.fields, &self.count) {
                    (Some(fields), _) => fields.clone(),
                    (None, Some(_)) => Vec::new(),
                    (None, None) => vec![RowField::Icon, RowField::Item],
                },
            },
        )?;
        // a view serialises whole; a null would be refused by the read
        let view = serde_json::to_value(View::from(shape)).unwrap_or_default();
        Ok(serde_json::json!({
            "scope": scope,
            "query": match (&self.text, &self.trade) {
                (Some(text), _) => serde_json::json!({ "text": text }),
                (None, Some(trade)) => serde_json::json!({ "trade": trade }),
                (None, None) => serde_json::json!({ "trade": self.envelope }),
            },
            "view": view,
        }))
    }
}

/// A form's fields folded for their count alone: the form's view fields set
/// aside before the fold, so the view the form names is never read, nor
/// refused where its values are no view's (that is the Search's to say),
/// and a name the form never emits still refused; the view is rows at limit
/// 0 with `sort` `none`, as `acq search --limit 0 --sort none` asks, so no
/// row is walked for a number.
pub(crate) fn fold_count(form: &Form, fields: &[(String, String)]) -> Result<Folded, String> {
    let searched: Vec<(String, String)> = fields
        .iter()
        .filter(|(name, _)| !(name.starts_with("view.") && form.names.contains(name)))
        .cloned()
        .collect();
    Ok(Folded {
        limit: Some(0),
        sort: Some(acquisition_search::answer::STORE_ORDER.to_string()),
        ..fold(form, &searched)?
    })
}

/// A form's fields folded by their names (the module doc). A name the
/// form never emits, or a value no field of its kind takes, is a usage
/// error, said before anything is built.
pub(crate) fn fold(form: &Form, fields: &[(String, String)]) -> Result<Folded, String> {
    if let Some((name, _)) = fields.iter().find(|(name, _)| !form.names.contains(name)) {
        return Err(format!(
            "`{name}` is no field of this form: a search is the form's own fields"
        ));
    }
    let mut query = Value::Object(Map::new());
    let mut folded = Folded {
        realm: None,
        membership: None,
        envelope: Value::Null,
        limit: None,
        sort: None,
        values: Vec::new(),
        desc: false,
        count: None,
        fields: None,
        text: None,
        trade: None,
    };
    let mut league = None;
    let mut defences = None;
    let mut groups: Vec<StatGroup> = Vec::new();
    for (name, value) in fields {
        let value = value.trim();
        // a stat field, read by its place among the fields: its presence
        // begins a group or a row, its value empty or not
        if let Some(field) = name.strip_prefix("group.") {
            if field == "type" {
                groups.push(StatGroup::default());
            }
            let group = groups.last_mut().ok_or_else(|| {
                format!(
                    "`{name}` comes before any `group.type`: a stat group's fields follow the \
                     type that begins it"
                )
            })?;
            group.fields.push((field.to_string(), value.to_string()));
            continue;
        }
        if let Some(field) = name.strip_prefix("stat.") {
            let group = groups.last_mut().ok_or_else(|| {
                format!(
                    "`{name}` comes before any `group.type`: a stat row stands in a group, which \
                     its type begins"
                )
            })?;
            if field == "id" {
                group.rows.push(Vec::new());
            }
            let row = group.rows.last_mut().ok_or_else(|| {
                format!(
                    "`{name}` comes before any `stat.id` of its group: a row's fields follow \
                     the stat that begins it"
                )
            })?;
            row.push((field.to_string(), value.to_string()));
            continue;
        }
        // the text sent is the query, empty or not (the module doc)
        if name == "text" {
            folded.text = Some(value.to_string());
            continue;
        }
        if name == "trade" {
            if value.is_empty() {
                return Err(
                    "the trade site search box is empty: paste a search's URL, its id \
                     or its query JSON"
                        .to_string(),
                );
            }
            folded.trade = Some(value.to_string());
            continue;
        }
        if value.is_empty() {
            continue;
        }
        match name.as_str() {
            "realm" => folded.realm = Some(value.to_string()),
            "membership" => folded.membership = Some(value.to_string()),
            "league" => league = Some(value.to_string()),
            "view.limit" => {
                folded.limit = Some(
                    value
                        .parse()
                        .map_err(|_| format!("`{value}` is no number of rows"))?,
                )
            }
            "view.sort" => folded.sort = Some(value.to_string()),
            "view.value" => folded.values.push(value.to_string()),
            "defences" => defences = Some(value.to_string()),
            "view.desc" => folded.desc = true,
            "view.count" => folded.count = Some(acquisition_search::counts::count_keys(value)?),
            "view.fields" => {
                folded.fields = Some(RowField::list(value).map_err(|e| e.message)?);
            }
            _ => {
                let path = name
                    .strip_prefix("query.")
                    .ok_or_else(|| format!("`{name}` is no field of this form"))?;
                let steps = steps(path).ok_or_else(|| format!("`{name}` is no field's path"))?;
                let last = match steps.last() {
                    Some(Step::Key(key)) => key.as_str(),
                    _ => "",
                };
                put(&mut query, &steps, typed(last, value));
            }
        }
    }
    // a count shows no rows: a search a page sends that asks one lets go
    // of the order, its direction and the values the page carried, which
    // the reader may not see (#304, the owner's rule); the shared refusal
    // of a sort beside a count stands at every other door
    if folded.count.is_some() {
        folded.sort = None;
        folded.desc = false;
        folded.values.clear();
    }
    if folded.text.is_some() && folded.trade.is_some() {
        return Err(
            "a search is a query in the language or a trade site search, not both: \
             run one box at a time"
                .to_string(),
        );
    }
    let stats: Vec<Value> = groups.iter().map(StatGroup::folded).collect();
    if !stats.is_empty() {
        put(
            &mut query,
            &[Step::Key("stats".into())],
            Value::Array(stats),
        );
    }
    // an on box sends `disabled=false` after its hidden `true`: the last
    // sent stands, and a `false` is dropped as the normal form drops it
    let mut query = strip(&query);
    prune(&mut query);
    drop_empty_stat_groups(&mut query);
    let mut envelope = Map::new();
    if let Some(league) = league {
        envelope.insert("league".into(), Value::String(league));
    }
    // C142: the defences' reading beside the query, as the quality box
    // names it, the site's own dropped as the normal form drops it
    if let Some(defences) = defences.map(Value::String)
        && !dropped("defences", &defences)
    {
        envelope.insert("defences".into(), defences);
    }
    envelope.insert(
        "query".into(),
        match query {
            Value::Null => Value::Object(Map::new()),
            query => query,
        },
    );
    folded.envelope = Value::Object(envelope);
    Ok(folded)
}

/// A stat group as its fields came, each field's name past `group.` or
/// `stat.`, in the order sent.
#[derive(Default)]
struct StatGroup {
    fields: Vec<(String, String)>,
    rows: Vec<Vec<(String, String)>>,
}

impl StatGroup {
    /// The group as the site's JSON spells it: its type, its count's
    /// bound and its `disabled`, then each row, a value key the group's
    /// type does not take dropped by the translation's own list
    /// (`stat_value_keys`), and the empty dropped as every field's is.
    fn folded(&self) -> Value {
        use acquisition_search::trade::translate::{group_value_keys, stat_value_keys};
        let mut group = Value::Object(Map::new());
        // the type the group was sent with: its last, as every field's
        let kind = self
            .fields
            .iter()
            .rev()
            .find(|(field, _)| field == "type")
            .map_or("", |(_, value)| value.as_str());
        let bounds = group_value_keys(kind);
        for (field, value) in &self.fields {
            if value.is_empty() {
                continue;
            }
            let path: &[&str] = match field.as_str() {
                "type" => &["type"],
                key @ ("min" | "max") if bounds.contains(&key) => &["value", key],
                "min" | "max" => continue,
                _ => &["disabled"],
            };
            put_keys(&mut group, path, value);
        }
        let takes = stat_value_keys(kind);
        let rows: Vec<Value> = self
            .rows
            .iter()
            .map(|fields| {
                let mut row = Value::Object(Map::new());
                for (field, value) in fields {
                    if value.is_empty() {
                        continue;
                    }
                    match field.as_str() {
                        "id" => put_keys(&mut row, &["id"], value),
                        "disabled" => put_keys(&mut row, &["disabled"], value),
                        key if takes.contains(&key) => put_keys(&mut row, &["value", key], value),
                        _ => {}
                    }
                }
                row
            })
            .collect();
        put(
            &mut group,
            &[Step::Key("filters".into())],
            Value::Array(rows),
        );
        group
    }
}

/// `value`, typed by its last key, put at `keys` under `at`.
fn put_keys(at: &mut Value, keys: &[&str], value: &str) {
    let steps: Vec<Step> = keys.iter().map(|k| Step::Key(k.to_string())).collect();
    put(at, &steps, typed(keys.last().copied().unwrap_or(""), value));
}

/// The fold's normal form of an envelope, what the fill reads and its pin
/// compares (`4-full-1-form-design.md` §2): what [`dropped`] names dropped
/// at every depth, then what is left empty,
/// then a stat group with no filter. What the fold writes is in it.
pub(crate) fn normal(envelope: &Value) -> Value {
    let mut out = strip(envelope);
    prune(&mut out);
    if let Some(query) = out.get_mut("query") {
        drop_empty_stat_groups(query);
    }
    out
}

/// The defences' reading that is the site's own (C142), which the quality
/// box sends ticked and the normal form drops, as it drops a `disabled` of
/// false.
pub(crate) const SITE_DEFENCES: &str = "q20";

/// Whether the normal form drops `value` at `key`: a null, an empty string,
/// a `disabled` of false and the site's own defences' reading
/// ([`SITE_DEFENCES`]).
fn dropped(key: &str, value: &Value) -> bool {
    value.is_null()
        || value.as_str() == Some("")
        || (key == "disabled" && *value == Value::Bool(false))
        || (key == "defences" && value.as_str() == Some(SITE_DEFENCES))
}

/// What the normal form drops ([`dropped`]) dropped at every depth: the
/// first move of the normal form ([`normal`]).
fn strip(value: &Value) -> Value {
    match value {
        Value::Object(o) => Value::Object(
            o.iter()
                .filter(|(k, v)| !dropped(k, v))
                .map(|(k, v)| (k.clone(), strip(v)))
                .collect(),
        ),
        Value::Array(a) => Value::Array(
            a.iter()
                .filter(|v| !(v.is_null() || v.as_str() == Some("")))
                .map(strip)
                .collect(),
        ),
        other => other.clone(),
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Step {
    Key(String),
    Index(usize),
}

/// `stats[0].filters[1].id` as its steps; none where a step is empty or
/// an index is no number.
pub(crate) fn steps(path: &str) -> Option<Vec<Step>> {
    let mut out = Vec::new();
    for part in path.split('.') {
        let (key, mut rest) = match part.find('[') {
            Some(at) => (&part[..at], &part[at..]),
            None => (part, ""),
        };
        if key.is_empty() {
            return None;
        }
        out.push(Step::Key(key.to_string()));
        while let Some(open) = rest.strip_prefix('[') {
            let close = open.find(']')?;
            out.push(Step::Index(open[..close].parse().ok()?));
            rest = &open[close + 1..];
        }
        if !rest.is_empty() {
            return None;
        }
    }
    Some(out)
}

/// A field's value as the site's JSON spells it: a number where the key
/// takes one and the text reads as one, a group's `disabled` a boolean,
/// else the text.
pub(crate) fn typed(key: &str, value: &str) -> Value {
    // the site's own `disabled` is a boolean
    if key == "disabled" {
        match value {
            "true" => return Value::Bool(true),
            "false" => return Value::Bool(false),
            _ => {}
        }
    }
    let numeric = matches!(key, "min" | "max" | "weight" | "r" | "g" | "b" | "w");
    if numeric {
        if let Ok(n) = value.parse::<i64>() {
            return n.into();
        }
        if let Some(n) = value
            .parse::<f64>()
            .ok()
            .and_then(serde_json::Number::from_f64)
        {
            return Value::Number(n);
        }
    }
    Value::String(value.to_string())
}

fn put(at: &mut Value, steps: &[Step], value: Value) {
    let Some((first, rest)) = steps.split_first() else {
        *at = value;
        return;
    };
    let next = match first {
        Step::Key(key) => {
            if !at.is_object() {
                *at = Value::Object(Map::new());
            }
            at.as_object_mut()
                .map(|o| o.entry(key.clone()).or_insert(Value::Null))
        }
        Step::Index(n) => {
            if !at.is_array() {
                *at = Value::Array(Vec::new());
            }
            at.as_array_mut().map(|a| {
                if a.len() <= *n {
                    a.resize(*n + 1, Value::Null);
                }
                &mut a[*n]
            })
        }
    };
    if let Some(next) = next {
        put(next, rest, value);
    }
}

/// Take out what holds nothing: a null, an empty object or array, at
/// every depth, deepest first; an array's elements close up.
fn prune(value: &mut Value) {
    match value {
        Value::Object(o) => {
            o.values_mut().for_each(prune);
            o.retain(|_, v| !empty(v));
        }
        Value::Array(a) => {
            a.iter_mut().for_each(prune);
            a.retain(|v| !empty(v));
        }
        _ => {}
    }
}

fn empty(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::Object(o) => o.is_empty(),
        Value::Array(a) => a.is_empty(),
        _ => false,
    }
}

/// A stat group with no filter left: its type, or its count's bound,
/// says nothing of an item without one.
fn drop_empty_stat_groups(query: &mut Value) {
    let Some(groups) = query.get_mut("stats").and_then(Value::as_array_mut) else {
        return;
    };
    groups.retain(|g| g.get("filters").is_some_and(|f| !empty(f)));
    if groups.is_empty()
        && let Some(q) = query.as_object_mut()
    {
        q.shift_remove("stats");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn fields(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn the_fold_writes_each_field_at_its_path_and_drops_the_empty() {
        let form = Form::load().unwrap();
        let folded = fold(
            &form,
            &fields(&[
                ("realm", "pc"),
                ("league", "Standard"),
                (
                    "query.filters.type_filters.filters.category.option",
                    "accessory.ring",
                ),
                ("query.filters.type_filters.filters.rarity.option", ""),
                ("query.filters.misc_filters.filters.ilvl.min", "75"),
                ("query.filters.misc_filters.filters.ilvl.max", ""),
                ("group.type", "and"),
                ("stat.id", ""),
                ("stat.min", ""),
                ("stat.id", "explicit.stat_3299347043"),
                ("stat.min", "40.5"),
                ("view.limit", "50"),
                ("view.sort", ""),
            ]),
        )
        .unwrap();
        assert_eq!(folded.realm.as_deref(), Some("pc"));
        assert_eq!(folded.limit, Some(50));
        assert_eq!(folded.sort, None);
        assert_eq!(
            folded.envelope,
            json!({
                "league": "Standard",
                "query": {
                    "filters": {
                        "type_filters": { "filters": { "category": { "option": "accessory.ring" } } },
                        "misc_filters": { "filters": { "ilvl": { "min": 75 } } },
                    },
                    "stats": [{ "type": "and", "filters": [
                        { "id": "explicit.stat_3299347043", "value": { "min": 40.5 } },
                    ] }],
                },
            })
        );
    }

    /// `view.value` is one value each, in the order sent, into
    /// the rows view's `values`, as the CLI's `--value` given again; an
    /// empty one is dropped as every empty field is.
    #[test]
    fn the_fold_reads_each_value_in_its_order() {
        let form = Form::load().unwrap();
        let folded = fold(
            &form,
            &fields(&[
                ("text", "class:rings"),
                ("view.value", "sum(line(\"# to #\").avg)"),
                ("view.value", ""),
                ("view.value", "ilvl"),
                ("view.sort", "ilvl"),
            ]),
        )
        .unwrap();
        assert_eq!(folded.values, ["sum(line(\"# to #\").avg)", "ilvl"]);
        let request = folded.request().unwrap();
        assert_eq!(
            request["view"]["rows"]["values"],
            json!(["sum(line(\"# to #\").avg)", "ilvl"]),
            "{request}"
        );
    }

    /// The quality box (C142, the defences' reading beside the query), as
    /// a browser sends it: ticked, its hidden `displayed` then its `q20`,
    /// which folds to no `defences`, the site's own reading dropped as the
    /// normal form drops it; unticked, the hidden `displayed` alone.
    #[test]
    fn the_quality_box_ticked_folds_to_no_defences_and_unticked_to_displayed() {
        let form = Form::load().unwrap();
        let armour = ("query.filters.armour_filters.filters.ar.min", "300");
        let ticked = fold(
            &form,
            &fields(&[("defences", "displayed"), ("defences", "q20"), armour]),
        )
        .unwrap();
        assert_eq!(
            ticked.envelope.get("defences"),
            None,
            "{:?}",
            ticked.envelope
        );
        let unticked = fold(&form, &fields(&[("defences", "displayed"), armour])).unwrap();
        assert_eq!(unticked.envelope["defences"], "displayed");
        assert_eq!(
            normal(&json!({ "defences": "q20", "query": { "term": "a" } })),
            json!({ "query": { "term": "a" } })
        );
    }

    #[test]
    fn a_stat_group_with_no_filter_is_dropped_and_a_bad_name_refused() {
        let form = Form::load().unwrap();
        let folded = fold(
            &form,
            &fields(&[("group.type", "count"), ("group.min", "2")]),
        )
        .unwrap();
        assert_eq!(folded.envelope, json!({ "query": {} }));
        // a group turned off is sent as the site sends it, its values
        // standing beside its `disabled`
        let folded = fold(
            &form,
            &fields(&[
                ("query.filters.misc_filters.disabled", "true"),
                ("query.filters.misc_filters.filters.ilvl.min", "75"),
            ]),
        )
        .unwrap();
        assert_eq!(
            folded.envelope["query"]["filters"]["misc_filters"],
            json!({ "disabled": true, "filters": { "ilvl": { "min": 75 } } })
        );
        // the stat group's the same: off, its rows stand
        let folded = fold(
            &form,
            &fields(&[
                ("group.type", "and"),
                ("group.disabled", "true"),
                ("stat.id", "explicit.stat_3299347043"),
                ("stat.min", "40"),
            ]),
        )
        .unwrap();
        assert_eq!(
            folded.envelope["query"]["stats"],
            json!([{ "type": "and", "disabled": true, "filters": [
                { "id": "explicit.stat_3299347043", "value": { "min": 40 } }
            ] }])
        );
        // an on box ticked sends `false` after its hidden `true`: the last
        // stands, and a `false` is dropped; left unticked, the hidden
        // `true` alone
        let folded = fold(
            &form,
            &fields(&[
                ("query.filters.misc_filters.disabled", "true"),
                ("query.filters.misc_filters.disabled", "false"),
                ("query.filters.misc_filters.filters.ilvl.min", "75"),
                ("group.type", "and"),
                ("group.disabled", "true"),
                ("group.disabled", "false"),
                ("stat.id", "explicit.stat_3299347043"),
                ("stat.disabled", "true"),
                ("stat.id", "explicit.stat_3372524247"),
                ("stat.disabled", "true"),
                ("stat.disabled", "false"),
            ]),
        )
        .unwrap();
        assert_eq!(
            folded.envelope["query"],
            json!({
                "filters": { "misc_filters": { "filters": { "ilvl": { "min": 75 } } } },
                "stats": [{ "type": "and", "filters": [
                    { "id": "explicit.stat_3299347043", "disabled": true },
                    { "id": "explicit.stat_3372524247" },
                ] }],
            })
        );
        assert!(fold(&form, &fields(&[("scope.realm", "pc")])).is_err());
        // an indexed path is no name of the form: the stat fields are
        // positional
        let indexed = fold(&form, &fields(&[("query.stats[0].type", "and")])).unwrap_err();
        assert!(indexed.contains("no field of this form"), "{indexed}");
        // a number field that does not read as one is the text typed
        let folded = fold(
            &form,
            &fields(&[("query.filters.misc_filters.filters.ilvl.min", "lots")]),
        )
        .unwrap();
        assert_eq!(
            folded.envelope["query"]["filters"]["misc_filters"]["filters"]["ilvl"]["min"],
            "lots"
        );
    }

    /// The stat fields are positional (the module doc): `group.type` begins
    /// a group and `stat.id` a row in it, each by its presence, its value
    /// empty or not; a group's fields belong to the group last begun and a
    /// row's to the row last begun in it, so two groups and their rows fold
    /// in the order sent and an emptied row leaves no hole. A field before
    /// what it belongs to is a usage error. A value key a group's type does
    /// not take is dropped, by the translation's own lists: a row's weight
    /// outside a weighted sum, a group's bound outside a count or a sum.
    #[test]
    fn the_stat_fields_fold_by_their_place_and_a_type_drops_what_it_does_not_take() {
        let form = Form::load().unwrap();
        let folded = fold(
            &form,
            &fields(&[
                ("group.type", "and"),
                ("stat.id", "explicit.stat_3299347043"),
                ("stat.min", "40"),
                ("stat.weight", "2"),
                ("stat.id", ""),
                ("stat.min", ""),
                ("stat.id", "explicit.stat_3372524247"),
                ("stat.disabled", "true"),
                ("group.min", "1"),
                ("group.type", "weight"),
                ("group.min", "60"),
                ("stat.id", "explicit.stat_4220027924"),
                ("stat.weight", "1.5"),
                ("stat.option", "x"),
                ("group.disabled", "true"),
                ("stat.max", "9"),
            ]),
        )
        .unwrap();
        assert_eq!(
            folded.envelope["query"]["stats"],
            json!([
                { "type": "and", "filters": [
                    { "id": "explicit.stat_3299347043", "value": { "min": 40 } },
                    { "id": "explicit.stat_3372524247", "disabled": true },
                ] },
                { "type": "weight", "value": { "min": 60 }, "disabled": true, "filters": [
                    { "id": "explicit.stat_4220027924",
                      "value": { "weight": 1.5, "option": "x", "max": 9 } },
                ] },
            ])
        );
        for (sent, says) in [
            (
                vec![("stat.id", "explicit.stat_3299347043")],
                "before any `group.type`",
            ),
            (vec![("group.min", "1")], "before any `group.type`"),
            (
                vec![("group.type", "and"), ("stat.min", "1")],
                "before any `stat.id` of its group",
            ),
            (
                vec![
                    ("group.type", "and"),
                    ("stat.id", "explicit.stat_3299347043"),
                    ("group.type", "and"),
                    ("stat.min", "1"),
                ],
                "before any `stat.id` of its group",
            ),
        ] {
            let refused = fold(&form, &fields(&sent)).unwrap_err();
            assert!(refused.contains(says), "{sent:?}: {refused}");
        }
    }

    /// `view.fields` is the row's fields as the CLI's `--fields` names
    /// them. Absent or empty, the page's default: the item with its art
    /// (cards); `icon`, the table; an unknown word, or a list naming no
    /// field, a usage error; any of them beside a count, the view
    /// selection's refusal; a count alone asks none.
    #[test]
    fn the_fold_reads_the_rows_fields_and_a_count_asks_none() {
        let form = Form::load().unwrap();
        let asked = |pairs: &[(&str, &str)]| {
            fold(&form, &fields(pairs)).map(|folded| folded.request().map(|r| r["view"].clone()))
        };
        let cards = json!({ "rows": { "fields": ["icon", "item"] } });
        assert_eq!(asked(&[]).unwrap().unwrap(), cards);
        assert_eq!(asked(&[("view.fields", "")]).unwrap().unwrap(), cards);
        assert_eq!(
            asked(&[("view.fields", "icon")]).unwrap().unwrap(),
            json!({ "rows": { "fields": ["icon"] } })
        );
        assert_eq!(
            asked(&[("view.fields", "icon,item")]).unwrap().unwrap(),
            cards
        );
        let unknown = asked(&[("view.fields", "price")]).unwrap_err();
        assert!(
            unknown.contains("`price` is no field a row carries"),
            "{unknown}"
        );
        // sent empty is the default; a list naming no field is refused
        let none = asked(&[("view.fields", ",")]).unwrap_err();
        assert!(none.contains("`fields` names no field"), "{none}");
        let beside = asked(&[("view.fields", "icon"), ("view.count", "rarity")])
            .unwrap()
            .unwrap_err();
        assert!(beside.message.contains("`fields`"), "{}", beside.message);
        let counted = asked(&[("view.count", "rarity")]).unwrap().unwrap();
        assert!(counted.get("rows").is_none(), "{counted}");
    }

    #[test]
    fn the_form_loads_every_group_of_the_dump_with_its_fates() {
        let form = Form::load().unwrap();
        assert_eq!(form.groups.len(), 12);
        assert_eq!(
            form.groups.iter().map(|g| g.filters.len()).sum::<usize>(),
            94
        );
        assert_eq!(form.fate("query.status").unwrap().status, "listing_only");
        assert_eq!(form.stat_types.len(), 8);
        for f in &form
            .groups
            .iter()
            .find(|g| g.id == "trade_filters")
            .unwrap()
            .filters
        {
            let fate = form.fate(&format!("trade_filters.{}", f.id)).unwrap();
            assert_eq!(fate.status, "listing_only", "{}", f.id);
            assert!(fate.says.is_some(), "{}", f.id);
        }
    }
}
