//! The search form and its fold (C139; `roadmap/briefs/4b-build.md`, Do
//! 6): the form generated from the committed filter dump, each control
//! named by its path in the site's query JSON, and the submission folded
//! back into that JSON by those names.
//!
//! # As built
//!
//! - **The form is the dump's.** The trade site's filter definitions of
//!   2026-09-12 (`search/trade-query/data/filters-2026-09-12.json`) are
//!   compiled into the binary and read once when the server starts: a
//!   control per filter, in the dump's groups and order, its label the
//!   site's. No build step writes it and no page reads the dump.
//! - **Each control's fate is the catalogue's** (`--describe trade`):
//!   beside it, its row's status, its `says` as the title; an option
//!   whose own row differs carries that row's status in its label. A
//!   listing-only row is rendered disabled: a browser submits nothing of
//!   a disabled control, so the fold never meets one.
//! - **A name is a path**: dot-separated keys, `[n]` an array's element —
//!   `query.filters.type_filters.filters.category.option`,
//!   `query.stats[0].filters[1].value.min`. Three names are no path and
//!   go elsewhere: `realm` to the request's scope, `league` beside the
//!   query (C141, the league beside the trade JSON), and `view.limit`,
//!   `view.sort`, `view.desc` to the rows view.
//! - **A name is one the form emitted**: the fold refuses any other as
//!   a usage error before it builds anything (C47, malformed external
//!   input is a structured error), so no index past the form's stat rows
//!   and no path it never wrote can shape the JSON (the review of 4b,
//!   its fifth finding). The names are the form's own: a test holds the
//!   rendered form to them.
//! - **The fold drops what is empty**: a field submitted empty, then an
//!   object or array left with nothing in it, then a stat group left
//!   with no filter (its type alone says nothing). A `min`, `max`,
//!   `weight` or socket colour that reads as a number is one; anything
//!   else is the text typed, which the translation reports as it reads
//!   it. The fold decides nothing the translation decides.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Value};

/// The dump this build's form is generated from.
const FILTERS: &str = include_str!("../../../../search/trade-query/data/filters-2026-09-12.json");

/// How many stat rows the form offers; more is a refinement.
pub(crate) const STAT_ROWS: usize = 4;

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
                filters,
            });
        }
        let described = acquisition_search::describe(&["trade".to_string()])
            .map_err(|e| anyhow::anyhow!("{}", e.message))?;
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

/// Every name the form emits: the scope and the view's, each filter's
/// by its kind, and the stat rows'.
fn emitted(groups: &[Group]) -> BTreeSet<String> {
    let mut names: BTreeSet<String> = ["realm", "league", "view.limit", "view.sort", "view.desc"]
        .iter()
        .map(|n| n.to_string())
        .collect();
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
    for name in ["type", "value.min", "value.max"] {
        names.insert(format!("query.stats[0].{name}"));
    }
    for n in 0..STAT_ROWS {
        for name in ["id", "value.min", "value.max"] {
            names.insert(format!("query.stats[0].filters[{n}].{name}"));
        }
    }
    names
}

/// What a form submission asks: the trade JSON the fields fold into, and
/// the request's scope and view.
#[derive(Debug, PartialEq)]
pub(crate) struct Folded {
    pub realm: Option<String>,
    pub envelope: Value,
    pub limit: Option<usize>,
    pub sort: Option<String>,
    pub desc: bool,
}

impl Folded {
    /// The request the MCP's `search` reads, as the page asks it: the
    /// envelope as the trade input, each row with its icon (C140).
    pub(crate) fn request(&self) -> Value {
        let mut scope = Map::new();
        if let Some(realm) = &self.realm {
            scope.insert("realm".into(), Value::String(realm.clone()));
        }
        let mut rows = Map::new();
        if let Some(limit) = self.limit {
            rows.insert("limit".into(), limit.into());
        }
        if let Some(sort) = &self.sort {
            rows.insert("sort".into(), Value::String(sort.clone()));
        }
        if self.desc {
            rows.insert("desc".into(), true.into());
        }
        rows.insert("fields".into(), serde_json::json!(["icon"]));
        serde_json::json!({
            "scope": scope,
            "query": { "trade": self.envelope },
            "view": { "rows": rows },
        })
    }
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
        envelope: Value::Null,
        limit: None,
        sort: None,
        desc: false,
    };
    let mut league = None;
    for (name, value) in fields {
        let value = value.trim();
        if value.is_empty() {
            continue;
        }
        match name.as_str() {
            "realm" => folded.realm = Some(value.to_string()),
            "league" => league = Some(value.to_string()),
            "view.limit" => {
                folded.limit = Some(
                    value
                        .parse()
                        .map_err(|_| format!("`{value}` is no number of rows"))?,
                )
            }
            "view.sort" => folded.sort = Some(value.to_string()),
            "view.desc" => folded.desc = true,
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
    prune(&mut query);
    drop_empty_stat_groups(&mut query);
    let mut envelope = Map::new();
    if let Some(league) = league {
        envelope.insert("league".into(), Value::String(league));
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

#[derive(Debug, PartialEq)]
enum Step {
    Key(String),
    Index(usize),
}

/// `stats[0].filters[1].id` as its steps; none where a step is empty or
/// an index is no number.
fn steps(path: &str) -> Option<Vec<Step>> {
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
/// takes one and the text reads as one, else the text.
fn typed(key: &str, value: &str) -> Value {
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
        q.remove("stats");
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
                ("query.stats[0].type", "and"),
                ("query.stats[0].filters[0].id", ""),
                ("query.stats[0].filters[0].value.min", ""),
                ("query.stats[0].filters[1].id", "explicit.stat_3299347043"),
                ("query.stats[0].filters[1].value.min", "40.5"),
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

    #[test]
    fn a_stat_group_with_no_filter_is_dropped_and_a_bad_name_refused() {
        let form = Form::load().unwrap();
        let folded = fold(
            &form,
            &fields(&[
                ("query.stats[0].type", "count"),
                ("query.stats[0].value.min", "2"),
            ]),
        )
        .unwrap();
        assert_eq!(folded.envelope, json!({ "query": {} }));
        assert!(fold(&form, &fields(&[("scope.realm", "pc")])).is_err());
        assert!(fold(&form, &fields(&[("query.stats[x].id", "a")])).is_err());
        let past = fold(&form, &fields(&[("query.stats[1000000000].type", "and")])).unwrap_err();
        assert!(past.contains("no field of this form"), "{past}");
        let row = format!("query.stats[0].filters[{STAT_ROWS}].id");
        assert!(fold(&form, &fields(&[(row.as_str(), "x")])).is_err());
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

    #[test]
    fn the_form_loads_every_group_of_the_dump_with_its_fates() {
        let form = Form::load().unwrap();
        assert_eq!(form.groups.len(), 12);
        assert_eq!(
            form.groups.iter().map(|g| g.filters.len()).sum::<usize>(),
            93
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

    /// The names the fold accepts are exactly the names the rendered form
    /// carries: the two are written apart, and this holds them together.
    #[test]
    fn the_fold_accepts_exactly_the_names_the_rendered_form_carries() {
        let form = Form::load().unwrap();
        let page = crate::web::page::search_form(&form, "pc", &[]);
        let mut rendered = BTreeSet::new();
        for part in page.split("name=\"").skip(1) {
            rendered.insert(part[..part.find('"').unwrap()].to_string());
        }
        assert_eq!(rendered, form.names);
    }
}
