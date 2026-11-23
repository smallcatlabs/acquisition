//! The fill (`roadmap/briefs/4-full-1-form-design.md` §2): the fold's
//! inverse, the search form rendered from the site's query JSON it folds
//! into, so a search's page is its source's editor, filled.
//!
//! # As built
//!
//! - **What fills it** is an envelope, `{league, defences, query}` as the
//!   fold writes it and a site search decodes to (C141, the league beside
//!   the JSON; C142, the defences' reading beside it), put in the fold's
//!   normal form ([`form::normal`]) before any control reads it.
//! - **A control holds a value by rendering it.** Each control takes the
//!   value at its path only where that value folds back to itself exactly,
//!   read by the fold's own typing ([`form::typed`]): a select's own
//!   option, a number box's JSON number as serde writes it, a text box's
//!   string equal to its trim, a box's `true` (or the quality box's
//!   `displayed`, which leaves it unticked). The taking marks the path held. A listing control shows
//!   the value and holds nothing: a browser sends nothing of a disabled
//!   control.
//! - **What no control holds is named.** Every leaf of the normal envelope
//!   left unheld once the form is rendered ([`Fill::left`]) is listed above
//!   the form by the site's path, so the fill never drops silently. The
//!   partition is the pin: what the form holds, read as a browser submits
//!   it, folds back to the normal envelope less the paths listed, and those
//!   paths are all of the rest (`tests` below).

use std::cell::RefCell;
use std::collections::BTreeSet;

use serde_json::Value;

use super::form;

/// An envelope as the form reads it, and the paths its controls took.
pub(crate) struct Fill {
    envelope: Value,
    held: RefCell<BTreeSet<String>>,
}

impl Fill {
    /// Nothing to fill: the empty form.
    pub(crate) fn empty() -> Fill {
        Fill::of(&Value::Null)
    }

    /// `envelope` in the fold's normal form, no path held yet.
    pub(crate) fn of(envelope: &Value) -> Fill {
        Fill {
            envelope: form::normal(envelope),
            held: RefCell::new(BTreeSet::new()),
        }
    }

    /// A site search's decoding as the envelope it fills: the league and
    /// the defences' reading beside the query (C141, C142), the defences
    /// only where they are not the site's own, and the sort and every other
    /// key the envelope carried, which no control reads.
    pub(crate) fn decoded(decoded: &acquisition_search::trade::Decoded) -> Fill {
        let mut envelope = serde_json::Map::new();
        if let Some(league) = &decoded.league {
            envelope.insert("league".into(), Value::String(league.clone()));
        }
        if decoded.defences == acquisition_search::trade::Defences::Displayed {
            envelope.insert("defences".into(), Value::String("displayed".into()));
        }
        envelope.insert("query".into(), decoded.query.clone());
        if let Some(sort) = &decoded.sort {
            envelope.insert("sort".into(), sort.clone());
        }
        for (key, value) in &decoded.beside {
            envelope.insert(key.clone(), value.clone());
        }
        Fill::of(&Value::Object(envelope))
    }

    /// The value at `path` (`query.stats[0].filters[1].id`), where one
    /// stands.
    fn at(&self, path: &str) -> Option<&Value> {
        let steps = form::steps(path)?;
        let mut at = &self.envelope;
        for step in steps {
            at = match step {
                form::Step::Key(key) => at.get(key.as_str())?,
                form::Step::Index(n) => at.get(n)?,
            };
        }
        Some(at)
    }

    /// The value at `path` written as `rendered`, held where a box sends it
    /// back unchanged ([`boxed`]) and it folds back to itself by the fold's
    /// typing of `path`'s last key.
    fn hold(&self, path: &str, rendered: String) -> Option<String> {
        let value = self.at(path)?;
        let key = path.rsplit('.').next().unwrap_or(path);
        (form::typed(key, &rendered) == *value && boxed(&rendered)).then(|| {
            self.held.borrow_mut().insert(path.to_string());
            rendered
        })
    }

    /// A text box's value: a string at `path`.
    pub(crate) fn text(&self, path: &str) -> Option<String> {
        let text = self.at(path)?.as_str()?.to_string();
        self.hold(path, text)
    }

    /// A number box's value: a JSON number at `path`, as serde writes it.
    pub(crate) fn number(&self, path: &str) -> Option<String> {
        let number = self.at(path)?.as_number()?.to_string();
        self.hold(path, number)
    }

    /// A select's value: a string at `path` that is one of its options.
    pub(crate) fn option(&self, path: &str, among: impl Fn(&str) -> bool) -> Option<String> {
        let option = self.at(path)?.as_str().filter(|o| among(o))?.to_string();
        self.hold(path, option)
    }

    /// Whether a box sending `sent` is ticked: the value at `path` is what
    /// the box sends, as the fold types it (`true` for a `disabled`).
    pub(crate) fn ticked(&self, path: &str, sent: &str) -> bool {
        self.at(path).is_some() && self.hold(path, sent.to_string()).is_some()
    }

    /// What a listing control shows: the value at `path` as text, held by
    /// nothing, since the control sends nothing.
    pub(crate) fn shown(&self, path: &str) -> Option<String> {
        match self.at(path)? {
            Value::String(s) => Some(s.clone()),
            Value::Object(_) | Value::Array(_) => None,
            other => Some(other.to_string()),
        }
    }

    /// Whether anything stands under `path`: a panel holding a value
    /// renders open.
    pub(crate) fn holds_under(&self, path: &str) -> bool {
        self.at(path).is_some()
    }

    /// The query's stat groups, in the normal form, where the form can
    /// render them whole: a list of objects, each a `filters` list of
    /// objects each with an `id` its box holds ([`boxed`]), so a group and
    /// a row of the form stand at each one's place and fold back there. Any other is none: the pane is the empty
    /// form's, and every leaf of the stats is left (the module doc).
    pub(crate) fn stats(&self) -> &[Value] {
        let Some(groups) = self.at("query.stats").and_then(Value::as_array) else {
            return &[];
        };
        let rows = |group: &Value| {
            group
                .get("filters")
                .and_then(Value::as_array)
                .is_some_and(|rows| {
                    rows.iter()
                        .all(|row| row.get("id").and_then(Value::as_str).is_some_and(boxed))
                })
        };
        match groups.iter().all(|g| g.is_object() && rows(g)) {
            true => groups,
            false => &[],
        }
    }

    /// Every leaf of the envelope no control took, by its path, with its
    /// value: read after the form is rendered.
    pub(crate) fn left(&self) -> Vec<(String, Value)> {
        let held = self.held.borrow();
        leaves(&self.envelope)
            .into_iter()
            .filter(|(path, _)| !held.contains(path))
            .collect()
    }
}

/// Whether a box sends `text` back as it was written: a browser strips a
/// line break from a text box's value, and the fold trims what it reads.
fn boxed(text: &str) -> bool {
    text == text.trim() && !text.contains(['\n', '\r'])
}

/// Every leaf of a value by its path: an object's keys joined by `.`, an
/// array's elements `[n]`, a scalar a leaf.
pub(crate) fn leaves(value: &Value) -> Vec<(String, Value)> {
    fn walk(value: &Value, path: String, out: &mut Vec<(String, Value)>) {
        match value {
            Value::Object(o) => {
                for (k, v) in o {
                    let next = match path.is_empty() {
                        true => k.clone(),
                        false => format!("{path}.{k}"),
                    };
                    walk(v, next, out);
                }
            }
            Value::Array(a) => {
                for (n, v) in a.iter().enumerate() {
                    walk(v, format!("{path}[{n}]"), out);
                }
            }
            Value::Null => {}
            other => out.push((path, other.clone())),
        }
    }
    let mut out = Vec::new();
    walk(value, String::new(), &mut out);
    out
}

/// The one reader of a form as a browser submits it, the page tests'.
#[cfg(test)]
#[path = "../../tests/harness/submitted.rs"]
mod submitted;

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use serde_json::{Map, Value, json};

    use super::Fill;
    use crate::web::form::{self, Form};
    use crate::web::page::{self, BoxView, Filled};

    /// The owner's captured searches (`trade_decode.rs` reads them).
    const DECODE: &str = include_str!("../../../acquisition-search/tests/trade-decode.toml");
    /// The translation's cases (`trade_translate.rs` reads them).
    const TRANSLATE: &str = include_str!("../../../acquisition-search/tests/trade-translate.toml");
    /// The pseudo stats' captured searches (`trade_oracle.rs` reads them).
    const CAPTURES: &str = include_str!("../../../../search/pseudo-stats/data/captures.json");

    use super::submitted::{submitted, unescaped};

    /// The search form's opening tag.
    const SEARCH_FORM: &str =
        "<form method=\"get\" action=\"search#answer\" novalidate data-count>";

    /// The paths the page names as left out by a search from the form.
    fn named_left(html: &str) -> Vec<String> {
        html.split("data-path=\"")
            .skip(1)
            .map(|part| unescaped(&part[..part.find('"').unwrap()]))
            .collect()
    }

    /// The page an envelope fills, and the fill's normal envelope.
    fn filled_page(form: &Form, fill: Fill) -> (String, Value) {
        let filled = Filled {
            fill,
            ..Filled::empty("pc")
        };
        let html = page::editor(form, &filled, Some(&[("Standard".to_string(), true)]));
        (html, filled.fill.envelope.clone())
    }

    /// The partition (the module doc): the form as a browser submits it
    /// folds to the normal envelope less the paths the page names, and
    /// those paths are exactly the rest. The paths named, for a caller's
    /// own checks.
    fn parted(form: &Form, fill: Fill, what: &str) -> Vec<String> {
        let (html, normal) = filled_page(form, fill);
        let sent = submitted(&html, SEARCH_FORM);
        let folded = form::fold(form, &sent).unwrap_or_else(|e| panic!("{what}: {e}\n{sent:?}"));
        let held: BTreeMap<String, Value> = super::leaves(&form::normal(&folded.envelope))
            .into_iter()
            .collect();
        let every: BTreeMap<String, Value> = super::leaves(&normal).into_iter().collect();
        let left = named_left(&html);
        let mut whole = held.clone();
        for path in &left {
            assert!(
                !held.contains_key(path),
                "{what}: {path} is both sent and named"
            );
            let value = every
                .get(path)
                .unwrap_or_else(|| panic!("{what}: {path} is named and no leaf of the envelope"));
            assert!(
                whole.insert(path.clone(), value.clone()).is_none(),
                "{what}: {path} twice"
            );
        }
        assert_eq!(
            whole, every,
            "{what}: the form's fold and the paths named are not the envelope"
        );
        left
    }

    /// Every captured site search the trade tests hold, as the page fills
    /// from it: decoded, as the site box and an opened search decode one.
    fn captured() -> Vec<(String, Value)> {
        let mut out = Vec::new();
        for line in DECODE.lines().chain(TRANSLATE.lines()) {
            let query = line
                .strip_prefix("query = '''")
                .and_then(|q| q.strip_suffix("'''"))
                .or_else(|| {
                    line.strip_prefix("query = '")
                        .and_then(|q| q.strip_suffix('\''))
                });
            if let Some(query) = query {
                out.push((query.to_string(), serde_json::from_str(query).unwrap()));
            }
        }
        let captures: Value = serde_json::from_str(CAPTURES).unwrap();
        for (name, search) in captures["searches"].as_object().unwrap() {
            if let Some(query) = search.get("returned_query") {
                out.push((format!("captures {name}"), query.clone()));
            }
        }
        out
    }

    #[test]
    fn the_fill_parts_every_captured_site_search_into_what_the_form_sends_and_what_it_names() {
        let form = Form::load().unwrap();
        let mut filled = 0;
        let mut named = 0;
        for (what, query) in captured() {
            // a query the decode refuses stands in the site box, filling
            // nothing (the one page)
            let Ok(decoded) = acquisition_search::trade::decode_json(&query) else {
                continue;
            };
            let left = parted(&form, Fill::decoded(&decoded), &what);
            filled += 1;
            named += usize::from(!left.is_empty());
        }
        // the corpora are read whole: the owner's eight, the translation's
        // cases and the captures' searches
        assert!(filled > 1300, "{filled} searches filled");
        assert!(
            named > 0 && named < filled,
            "{named} of {filled} named a path"
        );
    }

    /// One envelope that fills every control of the form, each with a value
    /// it takes: what it names is exactly the listing controls' values and
    /// the sort, which no control reads.
    #[test]
    fn an_envelope_filling_every_control_names_only_the_listings_and_the_sort() {
        let form = Form::load().unwrap();
        let (envelope, listing) = every_control(&form);
        let left: BTreeSet<String> = parted(&form, Fill::of(&envelope), "every control")
            .into_iter()
            .collect();
        let mut expected = listing;
        expected.insert("sort.price".to_string());
        assert_eq!(left, expected);
    }

    /// The fill's fallbacks (`page.rs`, the stat groups): a type no option
    /// names is named and its rows still sent; a stats list the form cannot
    /// render whole is named whole; a number field holding text, an option
    /// no select holds, a string a box would trim: each named, the rest
    /// sent.
    #[test]
    fn what_no_control_can_hold_is_named_and_the_rest_sent() {
        let form = Form::load().unwrap();
        let cases = [
            (
                json!({"query": {"stats": [{"type": "future", "filters": [
                    {"id": "explicit.stat_3299347043", "value": {"min": 5}}]}]}}),
                vec!["query.stats[0].type"],
            ),
            (
                json!({"query": {"stats": [{"type": "and", "filters": 42}]}}),
                vec!["query.stats[0].filters", "query.stats[0].type"],
            ),
            (
                json!({"query": {"stats": [
                    {"type": "and", "filters": [{"id": "explicit.stat_3299347043"}]},
                    {"type": "and", "filters": [{"value": {"min": 1}}]}]}}),
                vec![
                    "query.stats[0].filters[0].id",
                    "query.stats[0].type",
                    "query.stats[1].filters[0].value.min",
                    "query.stats[1].type",
                ],
            ),
            (
                json!({"query": {"filters": {"misc_filters": {"filters": {
                    "ilvl": {"min": "75", "max": 80}}}}}}),
                vec!["query.filters.misc_filters.filters.ilvl.min"],
            ),
            (
                json!({"query": {"filters": {"type_filters": {"filters": {
                    "category": {"option": "nonsense"}, "rarity": {"option": "rare"}}}},
                    "term": " padded "}}),
                vec![
                    "query.filters.type_filters.filters.category.option",
                    "query.term",
                ],
            ),
            (
                json!({"query": {"stats": [
                    {"type": "and", "filters": [{"id": " padded "}]},
                    {"type": "and", "filters": [{"id": "explicit.stat_3299347043"}]}]}}),
                vec![
                    "query.stats[0].filters[0].id",
                    "query.stats[0].type",
                    "query.stats[1].filters[0].id",
                    "query.stats[1].type",
                ],
            ),
            (
                json!({"query": {"stats": [{"type": "and", "filters": [
                    {"id": "explicit.stat_329934\n7043"}]}]}}),
                vec!["query.stats[0].filters[0].id", "query.stats[0].type"],
            ),
            (json!({"query": {"term": "a\nterm"}}), vec!["query.term"]),
            (
                json!({"query": {"stats": [{"type": "and", "filters": [
                    {"id": "explicit.stat_3299347043", "value": {"weight": 2}}]}]}}),
                vec!["query.stats[0].filters[0].value.weight"],
            ),
        ];
        for (envelope, names) in cases {
            let left: BTreeSet<String> = parted(&form, Fill::of(&envelope), &envelope.to_string())
                .into_iter()
                .collect();
            let names: BTreeSet<String> = names.into_iter().map(str::to_string).collect();
            assert_eq!(left, names, "{envelope}");
        }
    }

    /// The names the fold accepts are exactly the names the form carries
    /// filled with every control, the view's untyped names among them, which
    /// no control writes and the form carries hidden where the URL named
    /// them: the two are written apart, and this holds them together.
    #[test]
    fn the_fold_accepts_exactly_the_names_the_every_control_form_carries() {
        let form = Form::load().unwrap();
        let (envelope, _) = every_control(&form);
        let filled = Filled {
            fill: Fill::of(&envelope),
            view: BoxView {
                sort: Some("ilvl".into()),
                desc: true,
                values: vec!["ilvl".into()],
                ..BoxView::default()
            },
            ..Filled::empty("pc")
        };
        let html = page::search_form(&form, &filled, Some(&[("Standard".to_string(), true)]));
        let mut rendered = BTreeSet::new();
        for part in html.split("name=\"").skip(1) {
            rendered.insert(part[..part.find('"').unwrap()].to_string());
        }
        // the site box and the query box are forms of their own
        rendered.insert("text".to_string());
        rendered.insert("trade".to_string());
        assert_eq!(rendered, form.names().clone());
    }

    /// The one row maker (`4-full-1-form-rows.md`, Decision 1): the row
    /// each option of a finder's listing carries is, byte for byte, the row
    /// the form renders from an envelope holding that id alone; and copied
    /// into an `and` group as the page's own template renders one, it is
    /// sent as a browser sends it and folds to that envelope.
    #[test]
    fn a_finders_row_is_the_forms_row_of_that_id_alone_and_folds_to_it() {
        let form = Form::load().unwrap();
        let listed = acquisition_search::trade::vocabulary::vocabulary("maximum life", 50).unwrap();
        assert!(
            listed.entries.iter().any(|e| e.group == "pseudo"),
            "{listed:?}"
        );
        assert!(
            listed.entries.iter().any(|e| e.group != "pseudo"),
            "{listed:?}"
        );
        let options = page::stat_options(&listed);
        let carried: Vec<&str> = options
            .split("<template>")
            .skip(1)
            .map(|part| &part[..part.find("</template>").unwrap()])
            .collect();
        assert_eq!(carried.len(), listed.entries.len(), "{options}");
        let items = "<div class=\"stat-rows\" data-items>\n";
        for (entry, row) in listed.entries.iter().zip(carried) {
            let envelope = json!({ "query": { "stats": [
                { "type": "and", "filters": [{ "id": entry.id }] }] } });
            let (html, _) = filled_page(&form, Fill::of(&envelope));
            let rows = &html[html.find(items).unwrap() + items.len()..];
            let rows = &rows[..rows
                .find("</div>\n<div class=\"filter\" data-finder>")
                .unwrap()];
            assert_eq!(rows, row, "{}", entry.id);
            let open = "<template id=\"stat-group-and\">";
            let group = &html[html.find(open).unwrap() + open.len()..];
            let group = &group[..group.find("</template>").unwrap()];
            let grouped = group.replacen(items, &format!("{items}{row}"), 1);
            let sent = submitted(&format!("{SEARCH_FORM}{grouped}</form>"), SEARCH_FORM);
            let folded = form::fold(&form, &sent).unwrap();
            assert_eq!(folded.envelope, envelope, "{}: {sent:?}", entry.id);
        }
    }

    /// An envelope filling every control of the form with a value it takes,
    /// and the paths of the listing controls' values among them.
    fn every_control(form: &Form) -> (Value, BTreeSet<String>) {
        let mut listing = BTreeSet::new();
        let mut query = Map::new();
        query.insert("term".into(), json!("a term"));
        let mut groups = Map::new();
        for group in &form.groups {
            for filter in &group.filters {
                let path = form::filter_path(&group.id, &filter.id);
                // the status's fate is the query's key, as its control reads it
                let key = match group.id.as_str() {
                    "status_filters" => "query.status".to_string(),
                    _ => format!("{}.{}", group.id, filter.id),
                };
                let listed = form.fate(&key).is_some_and(|f| f.status == "listing_only");
                let mut value = Map::new();
                if let Some(options) = &filter.options {
                    if let Some(id) = options.iter().find_map(|(id, _)| id.clone()) {
                        value.insert("option".into(), json!(id));
                    }
                } else if filter.known_item || filter.input {
                    value.insert("option".into(), json!("a text"));
                }
                if filter.min_max {
                    value.insert("min".into(), json!(1));
                    value.insert("max".into(), json!(2.5));
                }
                if filter.sockets {
                    for c in ["r", "g", "b", "w"] {
                        value.insert(c.into(), json!(1));
                    }
                }
                if listed {
                    listing.extend(value.keys().map(|k| format!("{path}.{k}")));
                }
                if group.id == "status_filters" {
                    query.insert("status".into(), Value::Object(value));
                } else {
                    let at = groups
                        .entry(group.id.clone())
                        .or_insert_with(|| json!({ "disabled": true, "filters": {} }));
                    at["filters"][filter.id.clone()] = Value::Object(value);
                }
            }
        }
        query.insert("filters".into(), Value::Object(groups));
        query.insert(
            "stats".into(),
            json!([
                { "type": "weight", "value": { "min": 1, "max": 9 }, "disabled": true, "filters": [
                    { "id": "explicit.stat_3299347043", "disabled": true,
                      "value": { "min": 1, "max": 2, "weight": 1.5, "option": "an option" } },
                    { "id": "pseudo.pseudo_total_life" },
                ] },
                { "type": "and", "filters": [
                    { "id": "explicit.stat_3372524247", "value": { "min": 30 } },
                ] },
            ]),
        );
        let envelope = json!({
            "league": "Standard",
            "defences": "displayed",
            "query": query,
            "sort": { "price": "asc" },
        });
        (envelope, listing)
    }
}
