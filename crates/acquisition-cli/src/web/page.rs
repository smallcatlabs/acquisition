//! The pages (C16, a renderer, not an app; C53, text is a function of the
//! JSON): each renders the documents a verb produced, as HTML — one, but
//! for an answer opened from a saved search, which renders the answer and
//! the saved row it was opened from, and a conflict's refusal, which
//! renders the row as it stands — with the CLI's own words where the CLI
//! has them — the answer's opening and its terms, `show`, the plan, the
//! jobs table — so the page states nothing the terminal does not. A page
//! sorts, filters, groups and composes nothing; where it wants what no
//! document carries, the search crate or the verb grows first. Every
//! version a page renders is copied into its head's basis line, never
//! composed ([`Head::basis`]).

use acquisition_search::Classed;
use acquisition_search::answer::{Answer, ViewOut};
use acquisition_search::show::Shown;
use acquisition_search::trade::translate::{Fidelity, Status};
use acquisition_verbs::saved::{ReadRow, SavedSearches};

use super::form::{Fate, Filter, Form, LIMITS, STAT_ROWS};
use crate::search_cmd;
use serde_json::Value;

use crate::plural;

/// The one template every page fills.
const TEMPLATE: &str = include_str!("assets/page.html");

/// HTML-escaped text.
pub(crate) fn esc(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            c => out.push(c),
        }
    }
    out
}

/// A URL component, percent-encoded: everything but the unreserved.
pub(crate) fn enc(text: &str) -> String {
    let mut out = String::new();
    for b in text.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            b => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// What fills the template beside the body.
#[derive(Default)]
pub(crate) struct Head {
    /// Seconds before the page reloads itself: the jobs page while a job
    /// runs, the whole of 4-lite's liveness.
    pub refresh: Option<u32>,
    /// The stat picker's script.
    pub picker: bool,
    /// The list filter's script: the refresh page's boxes under its
    /// columns.
    pub filter: bool,
    /// The versions of the documents the page renders (C16, every
    /// document carries its basis): each keyed by its document — `answer`,
    /// `item`, `plan`, `saved_searches`, `saved_search` — its value the
    /// document's own field as `serde_json::to_value` makes it, never a
    /// number the page composes. Written as one `<meta name="basis">` line
    /// where there is at least one, so part 3's feed has something to
    /// compare.
    pub basis: Vec<(&'static str, Value)>,
}

impl Head {
    /// A head carrying `document`'s version as `key`.
    pub(crate) fn with(mut self, key: &'static str, version: impl serde::Serialize) -> Head {
        if let Ok(value) = serde_json::to_value(version) {
            self.basis.push((key, value));
        }
        self
    }
}

/// A page: the template filled, the body last so nothing in it is read
/// as a placeholder.
pub(crate) fn page(token: &str, title: &str, head: Head, body: &str) -> String {
    let refresh = head
        .refresh
        .map(|s| format!("<meta http-equiv=\"refresh\" content=\"{s}\">\n"))
        .unwrap_or_default();
    let script: String = [(head.picker, "picker"), (head.filter, "filter")]
        .into_iter()
        .filter(|(wanted, _)| *wanted)
        .map(|(_, name)| format!("<script src=\"/{token}/assets/{name}.js\"></script>\n"))
        .collect();
    let basis = match head.basis.is_empty() {
        true => String::new(),
        false => {
            let versions: serde_json::Map<String, Value> = head
                .basis
                .into_iter()
                .map(|(key, value)| (key.to_string(), value))
                .collect();
            format!(
                "<meta name=\"basis\" content=\"{}\">\n",
                esc(&Value::Object(versions).to_string())
            )
        }
    };
    // the documents' words after the template's own placeholders, the
    // body last, so nothing in them is read as a placeholder
    TEMPLATE
        .replace("{refresh}", &refresh)
        .replace("{script}", &script)
        .replace("{title}", &esc(title))
        .replace("{token}", token)
        .replace("{basis}", &basis)
        .replace("{body}", body)
}

/// Text the CLI prints, as a block: each of its lines a line of the page
/// source too (a newline just inside `<pre>` is no part of its text).
pub(crate) fn pre(text: &str) -> String {
    format!("<pre>\n{}\n</pre>\n", esc(text.trim_end_matches('\n')))
}

/// A refusal, in the words the CLI would print after `Error:`.
pub(crate) fn refusal(words: &str) -> String {
    format!("<div class=\"refusal\">{}</div>\n", pre(words))
}

/// A refusal's document as a page renders it (#97): its words and each
/// reading on a line of its own, as a terminal prints them; where it
/// carries a saved search as it stands (a conflict's `current`), that row
/// as the list renders it, so the page shows what the offered write would
/// replace; then each way on, saying what it is — a read as a link to the
/// page that takes it, a write (`{post, fields}`) as a form of hidden
/// fields with one button — never a command line.
pub(crate) fn refused(doc: &Value) -> String {
    let said = |v: &Value| v.as_str().unwrap_or_default().to_string();
    let words = std::iter::once(said(&doc["error"]))
        .chain(
            doc["readings"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|r| format!("    {}", said(r))),
        )
        .collect::<Vec<_>>()
        .join("\n");
    let offers: String = doc["offers"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|o| match o["request"].get("post") {
            Some(post) => format!(
                "<li>{}</li>\n",
                post_form(
                    &said(post),
                    o["request"]["fields"]
                        .as_object()
                        .into_iter()
                        .flatten()
                        .map(|(k, v)| (k.as_str(), said(v))),
                    "",
                    &said(&o["says"])
                )
            ),
            None => format!(
                "<li><a href=\"{}\">{}</a></li>\n",
                esc(&said(&o["request"])),
                esc(&said(&o["says"]))
            ),
        })
        .collect();
    let offers = match offers.is_empty() {
        true => offers,
        false => format!("<ul class=\"offers\">\n{offers}</ul>\n"),
    };
    let standing = match saved_current(doc) {
        Some(row) => format!(
            "<p>It stands now as:</p>\n<table class=\"saved\">\n{SAVED_HEAD}<tr>{}</tr>\n</table>\n",
            saved_cells(row, None)
        ),
        None => String::new(),
    };
    format!(
        "<div class=\"refusal\">{}{standing}{offers}</div>\n",
        pre(&words)
    )
}

/// A refusal's `current` where it is a saved search as it stands (a
/// conflict's): its id with its value, or the reason this build cannot
/// read it.
pub(crate) fn saved_current(doc: &Value) -> Option<&Value> {
    doc.get("current").filter(|row| {
        row.get("id").is_some()
            && row.get("revision").is_some()
            && (row.get("unreadable").is_some() || row.pointer("/scope/membership").is_some())
    })
}

/// A form posting `fields` hidden to `action`, with `shown` (an input a
/// reader fills) before its one button.
fn post_form<'a>(
    action: &str,
    fields: impl IntoIterator<Item = (&'a str, String)>,
    shown: &str,
    button: &str,
) -> String {
    let hidden: String = fields
        .into_iter()
        .map(|(name, value)| {
            format!(
                "<input type=\"hidden\" name=\"{}\" value=\"{}\">",
                esc(name),
                esc(&value)
            )
        })
        .collect();
    format!(
        "<form class=\"write\" method=\"post\" action=\"{}\">{hidden}{shown}<button type=\"submit\">{}</button></form>",
        esc(action),
        esc(button)
    )
}

/// The saved list's header row, the forms' column last.
const SAVED_HEAD: &str = "<tr><th>Name</th><th>Scope</th><th>Query</th><th>Revision</th><th>Last written</th><th>Id</th></tr>\n";

/// One saved search's cells as the list renders them, from the row as its
/// document carries it: its name (a link that opens it, where `open` is
/// given), its scope, its query, its revision, when and through which
/// channel it was last written, and its id; a row this build cannot read
/// says why across its scope and query.
fn saved_cells(row: &Value, open: Option<&str>) -> String {
    let said = |v: &Value| v.as_str().unwrap_or_default().to_string();
    let revision = row["revision"].to_string();
    let id = esc(&said(&row["id"]));
    if let Some(why) = row.get("unreadable") {
        return format!(
            "<td>?</td><td colspan=\"2\">this build cannot read it: {}</td><td>{revision}</td><td></td><td>{id}</td>",
            esc(&said(why))
        );
    }
    let name = esc(&said(&row["name"]));
    let name = match open {
        Some(link) => format!("<a href=\"{}\">{name}</a>", esc(link)),
        None => name,
    };
    let actor = match row.get("actor").and_then(Value::as_str) {
        Some(actor) => format!(" as {actor}"),
        None => String::new(),
    };
    format!(
        "<td>{name}</td><td>{}</td><td>{}</td><td>{revision}</td><td>{}</td><td>{id}</td>",
        esc(&format!(
            "{} · {}",
            said(&row["scope"]["realm"]),
            said(&row["scope"]["membership"])
        )),
        esc(&said(&row["text"])),
        esc(&format!(
            "{} via {}{actor}",
            said(&row["updated_at_age"]),
            said(&row["written_via"])
        ))
    )
}

/// Where the saved pages' links and forms go, each absolute under the
/// token (C138): the list, the list whole, the write and the clear.
pub(crate) struct SavedLinks {
    pub list: String,
    pub every: String,
    pub write: String,
    pub clear: String,
}

/// The saved searches (4-full part 1, step 3, Do 4): the list's head
/// line, then a row per search in the document's order, each with a link
/// that opens it, its rename form naming the revision listed and its
/// Clear, a link to the page that asks first ([`saved_ask`]); a row this
/// build cannot read, its Clear alone; the rows left out counted, with
/// the link that lists every one.
pub(crate) fn saved_list(
    listed: &SavedSearches,
    links: &SavedLinks,
    open: &dyn Fn(&str) -> String,
) -> String {
    if listed.total == 0 {
        return format!(
            "<p>{}</p>\n",
            esc("No saved searches: save one from any search's answer, under its query.")
        );
    }
    let mut out = format!(
        "<p>{} · the file at revision {}</p>\n<table class=\"saved\">\n{}",
        plural(listed.total, "saved search", "saved searches"),
        listed.revision,
        SAVED_HEAD.replace("</tr>", "<th></th></tr>")
    );
    for row in &listed.searches {
        let doc = serde_json::to_value(row).unwrap_or_default();
        let id = row.id().to_string();
        let revision = row.revision().to_string();
        let keyed = || [("id", id.clone()), ("if_revision", revision.clone())];
        let rename = match row.name() {
            Some(name) => post_form(
                &links.write,
                keyed(),
                &format!(
                    "<input name=\"name\" value=\"{}\" aria-label=\"the new name\"> ",
                    esc(name)
                ),
                "Rename",
            ),
            None => String::new(),
        };
        // a clear asks first (the owner, "Clear should ask first"): a
        // link to the page that shows the row and holds the clear's form
        let clear = format!(
            "<a class=\"ask\" href=\"{}\">Clear</a>",
            esc(&format!("{}?id={id}", links.clear))
        );
        let opened = row.name().map(|_| open(&id));
        out.push_str(&format!(
            "<tr>{}<td>{rename}{clear}</td></tr>\n",
            saved_cells(&doc, opened.as_deref())
        ));
    }
    out.push_str("</table>\n");
    if listed.left_out > 0 {
        out.push_str(&format!(
            "<p>{} of {} listed · <a href=\"{}\">list every one</a></p>\n",
            listed.searches.len(),
            listed.total,
            esc(&links.every)
        ));
    }
    out
}

/// A clear asked first (the owner, 2026-11-07, "Clear should ask
/// first"): the row as the list shows it, unlinked, and the clear's form
/// at the revision shown, beside a link back to the list. The page
/// writes nothing; its form's POST is the clear. A row this build cannot
/// read is asked about the same way, since a clear is the one thing a
/// page can do with it.
pub(crate) fn saved_ask(row: &Value, links: &SavedLinks) -> String {
    let said = |v: &Value| v.as_str().unwrap_or_default().to_string();
    let named = match row.get("name").and_then(Value::as_str) {
        Some(name) => format!("the saved search \"{name}\""),
        None => format!("the saved search {}", said(&row["id"])),
    };
    format!(
        "<p>{}</p>\n<table class=\"saved\">\n{SAVED_HEAD}<tr>{}</tr>\n</table>\n{} <a href=\"{}\">Keep it</a>\n",
        esc(&format!(
            "Clear {named}? It is this store's saved search, whoever saved it: \
             clearing it takes it off the list for every page, agent and terminal \
             that reads this store."
        )),
        saved_cells(row, None),
        post_form(
            &links.clear,
            [
                ("id", said(&row["id"])),
                ("if_revision", row["revision"].to_string()),
            ],
            "",
            "Clear"
        ),
        esc(&links.list)
    )
}

/// The saved search an answer was opened from (4-full part 1, step 3,
/// Do 7): the id and revision its URL names, and the row as it stands
/// now at that id — readable, gone, or one this build cannot read.
pub(crate) struct Opened {
    pub id: String,
    pub at: i64,
    pub row: OpenedRow,
}

pub(crate) enum OpenedRow {
    Read(Box<ReadRow>),
    Gone,
    Unreadable,
}

/// What an answer opened from a saved search says above it.
fn opened_notice(a: &Answer, opened: &Opened, links: &SavedLinks) -> String {
    let (id, at) = (&opened.id, opened.at);
    let list = |words: &str| format!("<a href=\"{}\">{}</a>", esc(&links.list), esc(words));
    let mut out = match &opened.row {
        OpenedRow::Read(row) if row.revision == at => format!(
            "<p class=\"saved\">{} · {}</p>\n",
            esc(&format!(
                "Saved search \"{}\", opened at revision {at}",
                row.name
            )),
            list("the saved searches")
        ),
        OpenedRow::Read(row) => format!(
            "<p class=\"saved\">{} · {}</p>\n",
            esc(&format!(
                "Saved search {id}, opened at revision {at}, has been written since: it is now \
                 \"{}\" at revision {}, last written {} via {}. Saving over it here is refused \
                 as stale and offers the write over revision {}.",
                row.name, row.revision, row.updated_at_age, row.written_via, row.revision
            )),
            list("the saved searches")
        ),
        OpenedRow::Gone => format!(
            "<p class=\"saved\">{}</p>\n",
            esc(&format!(
                "Saved search {id} is gone: no saved search has that id now. Saving here makes \
                 a new one."
            ))
        ),
        OpenedRow::Unreadable => format!(
            "<p class=\"saved\">{}{}.</p>\n",
            esc(&format!(
                "Saved search {id} holds a value this build cannot read; clear it from "
            )),
            list("the saved searches")
        ),
    };
    if let OpenedRow::Read(row) = &opened.row
        && (row.text != a.query.text
            || row.scope.realm != a.scope.realm.as_str()
            || row.scope.membership != a.scope.membership.as_str())
    {
        out.push_str(&format!(
            "<p class=\"saved\">{}</p>\n",
            esc(&format!(
                "This answer's query is not the saved one's: the saved query is {} over {} · {}.",
                row.text, row.scope.realm, row.scope.membership
            ))
        ));
    }
    out
}

/// The save form under an answer's query box: the text the answer ran,
/// its realm and membership hidden, and a name typed; over the saved
/// search it was opened from where that row stands readable, at the
/// revision the URL names, beside a second form that saves it as new.
fn save_forms(a: &Answer, opened: Option<&Opened>, links: &SavedLinks) -> String {
    let query = || {
        vec![
            ("text", a.query.text.clone()),
            ("realm", a.scope.realm.as_str().to_string()),
            ("membership", a.scope.membership.as_str().to_string()),
        ]
    };
    let named = |name: &str| {
        format!(
            "<label class=\"filter-title\">Name <input name=\"name\" value=\"{}\" aria-label=\"the search's name\"></label> ",
            esc(name)
        )
    };
    let mut out = String::from("<div class=\"save\">\n");
    match opened {
        Some(Opened {
            id,
            at,
            row: OpenedRow::Read(row),
        }) => {
            let mut fields = query();
            fields.push(("id", id.clone()));
            fields.push(("if_revision", at.to_string()));
            out.push_str(&post_form(
                &links.write,
                fields,
                &named(&row.name),
                &format!("Save over \"{}\"", row.name),
            ));
            out.push('\n');
            out.push_str(&post_form(
                &links.write,
                query(),
                &named(""),
                "Save as a new search",
            ));
        }
        _ => out.push_str(&post_form(
            &links.write,
            query(),
            &named(""),
            "Save this search",
        )),
    }
    out.push_str("\n</div>\n");
    out
}

fn fate_span(fate: Option<&Fate>) -> String {
    match fate {
        Some(f) => format!(
            " <span class=\"fate fate-{}\"{}>{}</span>",
            esc(&f.status),
            f.says
                .as_ref()
                .map(|s| format!(" title=\"{}\"", esc(s)))
                .unwrap_or_default(),
            esc(&f.status.replace('_', " "))
        ),
        None => String::new(),
    }
}

/// The search form (Do 6), in the trade site's organization (the 4b
/// sitting round, Do 4; the owner's page save read for its order alone,
/// nothing of GGG's copied): the search bar across the top — the item
/// search on the left; the realm, the league and the status on the right
/// — then two panes, the filter groups as panels in the dump's order on
/// the left and the stat filters with the picker on the right, then the
/// controls with Search at the centre. A panel is a `details` group, open
/// where the site shows the group open.
///
/// The form is one realm's render: its league list is that realm's, so
/// the realm is chosen by a form of its own that renders the page again
/// (the review of 4b, its fourth finding), and the search carries the
/// realm shown.
pub(crate) fn search_form(form: &Form, realm: &str, leagues: Option<&[(String, bool)]>) -> String {
    let mut out = String::from(
        "<form id=\"realm-form\" method=\"get\" action=\".\"></form>\n<form method=\"get\" action=\"search\">\n<div class=\"search-bar\">\n<div class=\"search-left\">",
    );
    out.push_str(&format!(
        "<input class=\"item-search\" name=\"query.term\" placeholder=\"Search items\" aria-label=\"Search items\">{}</div>\n<div class=\"search-right\">",
        fate_span(form.fate("query.term"))
    ));
    out.push_str("<label class=\"control\">Realm <select name=\"realm\" form=\"realm-form\">");
    for r in ["pc", "xbox", "sony", "poe2"] {
        out.push_str(&format!(
            "<option value=\"{r}\"{}>{r}</option>",
            if r == realm { " selected" } else { "" }
        ));
    }
    out.push_str(&format!(
        "</select></label> <button type=\"submit\" form=\"realm-form\">Show this realm's leagues</button>\n<input type=\"hidden\" name=\"realm\" value=\"{}\">",
        esc(realm)
    ));
    out.push_str(&league_select(leagues, None, true));
    out.push_str(&fate_span(form.fate("url.league")));
    for group in form.groups.iter().filter(|g| g.id == "status_filters") {
        for filter in &group.filters {
            out.push_str(&control(form, filter, "query.status", "query.status"));
        }
    }
    // the scope's membership: the items standing, or those removed too,
    // or those removed alone
    out.push_str("<label class=\"control\">Membership <select name=\"membership\"><option value=\"live\" selected>live</option><option value=\"all\">all</option><option value=\"removed\">removed</option></select></label>\n");
    out.push_str("</div>\n</div>\n<div class=\"panes\">\n<div class=\"pane\">\n");
    for group in form.groups.iter().filter(|g| g.id != "status_filters") {
        let title = group.title.as_deref().unwrap_or(&group.id);
        // the group's enable control, as the site's: off, the group is
        // sent with its `disabled` field and its values stand; the panel's
        // own toggle is open and close alone
        out.push_str(&format!(
            "<details class=\"panel\"{}><summary>{}</summary>\n<div class=\"panel-body\">\n<div class=\"filter\"><label><input type=\"checkbox\" name=\"query.filters.{}.disabled\" value=\"true\"> disable this group</label></div>\n",
            if group.hidden { "" } else { " open" },
            esc(title),
            esc(&group.id)
        ));
        for filter in &group.filters {
            let path = super::form::filter_path(&group.id, &filter.id);
            let key = format!("{}.{}", group.id, filter.id);
            out.push_str(&control(form, filter, &path, &key));
        }
        // C142: the defences' reading beside the query, the panel's one
        // convenience the site's form has no control for
        if group.id == "armour_filters" {
            out.push_str("<div class=\"filter\"><label><input type=\"checkbox\" name=\"defences\" value=\"displayed\"> read these as the item stands, at its own quality, not the site's 20 %</label></div>\n");
        }
        out.push_str("</div>\n</details>\n");
    }
    out.push_str("</div>\n<div class=\"pane\">\n<details class=\"panel\" open><summary>Stat Filters</summary>\n<div class=\"panel-body\">\n<div class=\"filter\"><label><input type=\"checkbox\" name=\"query.stats[0].disabled\" value=\"true\"> disable this group</label></div>\n<div class=\"filter\"><span class=\"filter-title\">Group</span> <select name=\"query.stats[0].type\">");
    for (kind, text, fate) in &form.stat_types {
        out.push_str(&format!(
            "<option value=\"{}\"{}>{} ({})</option>",
            esc(kind),
            if kind == "and" { " selected" } else { "" },
            esc(text.as_deref().unwrap_or(kind)),
            esc(&fate.status.replace('_', " "))
        ));
    }
    out.push_str("</select> <input type=\"number\" step=\"any\" name=\"query.stats[0].value.min\" placeholder=\"min\" aria-label=\"group min\"> <input type=\"number\" step=\"any\" name=\"query.stats[0].value.max\" placeholder=\"max\" aria-label=\"group max\"></div>\n");
    for n in 0..STAT_ROWS {
        let id = format!("stat-{n}");
        out.push_str(&format!(
            "<div class=\"filter\"><input class=\"stat-id\" id=\"{id}\" name=\"query.stats[0].filters[{n}].id\" size=\"28\" placeholder=\"stat\" aria-label=\"stat {}\"> <input type=\"number\" step=\"any\" name=\"query.stats[0].filters[{n}].value.min\" placeholder=\"min\" aria-label=\"min\"> <input type=\"number\" step=\"any\" name=\"query.stats[0].filters[{n}].value.max\" placeholder=\"max\" aria-label=\"max\"> <span id=\"{id}-text\" class=\"fate\"></span></div>\n",
            n + 1
        ));
    }
    out.push_str("<div class=\"filter\"><label class=\"filter-title\" for=\"stat-find\">Find a stat</label> <input type=\"search\" id=\"stat-find\" autocomplete=\"off\"></div>\n<span id=\"stat-said\" class=\"fate\"></span>\n<ul id=\"stat-list\"></ul>\n</div>\n</details>\n</div>\n</div>\n");
    out.push_str("<div class=\"controls\">\n<div class=\"controls-left\"><label class=\"control\">Show <select name=\"view.limit\">");
    for limit in LIMITS {
        out.push_str(&format!("<option value=\"{limit}\">{limit}</option>"));
    }
    out.push_str("</select></label> <label class=\"control\">Sort by <input name=\"view.sort\" placeholder=\"ilvl\"></label> <label class=\"control\"><input type=\"checkbox\" name=\"view.desc\" value=\"true\"> highest first</label> <label class=\"control\">Or count by <input name=\"view.count\" placeholder=\"rarity\"></label> <span class=\"fate\">keys as the CLI's count takes them: rarity, league, tab, class, category, line:resist,life</span></div>\n<div class=\"controls-center\"><button type=\"submit\" class=\"search-btn\">Search</button></div>\n</div>\n</form>\n");
    out.push_str(&query_box(realm, None, "", None));
    out.push_str(&trade_box(realm));
    out.push_str(&fetch_leagues());
    out
}

/// The query box (the 4b sitting round, Do 6): a query in the language,
/// run as the user's own — the one place every field the language has is
/// reachable from a page, so what the site's form cannot say is typed
/// here. On an answer it holds the text the answer ran, the translation's
/// where the search was the site's form, to edit; an edited text drops the
/// trade input, as the request's own rule has it. On an answer it carries
/// the answer's membership, and the saved search the page was opened from
/// with the revision its URL names, so the loop survives an edit.
pub(crate) fn query_box(
    realm: &str,
    membership: Option<&str>,
    text: &str,
    saved: Option<(&str, i64)>,
) -> String {
    let hidden = |name: &str, value: &str| {
        format!(
            "<input type=\"hidden\" name=\"{name}\" value=\"{}\">",
            esc(value)
        )
    };
    let mut carried = hidden("realm", realm);
    if let Some(membership) = membership {
        carried.push_str(&hidden("membership", membership));
    }
    if let Some((id, at)) = saved {
        carried.push_str(&hidden("saved", id));
        carried.push_str(&hidden("saved_revision", &at.to_string()));
    }
    format!(
        "<form class=\"query-box\" method=\"get\" action=\"search\">{carried}<label class=\"filter-title\" for=\"query-text\">The query, in the language</label><textarea id=\"query-text\" name=\"text\" rows=\"3\">{}</textarea><button type=\"submit\">Run this query</button></form>\n",
        esc(text)
    )
}

/// The trade site box (#188, its first half): a site search as the
/// CLI's `--trade` takes it, run as the CLI's trade answer is — the
/// translation's fidelity and every clause above the rows, the site's sort
/// reported and not applied. It carries the realm the page shows, which an
/// id or the query JSON is searched in; a URL names its own, which wins.
fn trade_box(realm: &str) -> String {
    format!(
        "<form class=\"query-box\" method=\"get\" action=\"search\"><input type=\"hidden\" name=\"realm\" value=\"{}\"><label class=\"filter-title\" for=\"trade-text\">A trade site search: its URL, its id or its query JSON</label><textarea id=\"trade-text\" name=\"trade\" rows=\"2\"></textarea><button type=\"submit\">Run the site's search</button> <span class=\"fate\">translated here, sent nowhere; a URL's realm is the search's</span></form>\n",
        esc(realm)
    )
}

/// One filter's control, named by its path, its fate beside it; a
/// listing-only row's disabled, its `says` as the title.
fn control(form: &Form, filter: &Filter, path: &str, key: &str) -> String {
    let fate = form.fate(key);
    let listing = fate.is_some_and(|f| f.status == "listing_only");
    let disabled = if listing { " disabled" } else { "" };
    let title = fate
        .and_then(|f| f.says.as_ref())
        .filter(|_| listing)
        .map(|s| format!(" title=\"{}\"", esc(s)))
        .unwrap_or_default();
    let label = esc(filter.text.as_deref().unwrap_or(&filter.id));
    let label = if filter.text.is_none() {
        esc(form.text(key).unwrap_or(&filter.id))
    } else {
        label
    };
    let tip = filter
        .tip
        .as_ref()
        .map(|t| format!(" title=\"{}\"", esc(t)))
        .unwrap_or_default();
    let mut out =
        format!("<div class=\"filter\"{title}><span class=\"filter-title\"{tip}>{label}</span> ");
    if let Some(options) = &filter.options {
        out.push_str(&format!("<select name=\"{path}.option\"{disabled}>"));
        for (id, text) in options {
            let value = id.as_deref().unwrap_or("");
            let own = id
                .as_ref()
                .and_then(|id| form.fate(&format!("{key}={id}")))
                .map(|f| format!(" ({})", f.status.replace('_', " ")))
                .unwrap_or_default();
            out.push_str(&format!(
                "<option value=\"{}\">{}{}</option>",
                esc(value),
                esc(text),
                esc(&own)
            ));
        }
        out.push_str("</select>");
    } else if filter.known_item || filter.input {
        out.push_str(&format!(
            "<input name=\"{path}.option\" size=\"16\"{disabled}>"
        ));
    }
    if filter.min_max {
        out.push_str(&format!(
            "<input type=\"number\" step=\"any\" name=\"{path}.min\" placeholder=\"min\"{disabled}> <input type=\"number\" step=\"any\" name=\"{path}.max\" placeholder=\"max\"{disabled}>"
        ));
    }
    if filter.sockets {
        for c in ["r", "g", "b", "w"] {
            out.push_str(&format!(
                " <input class=\"socket\" type=\"number\" name=\"{path}.{c}\" placeholder=\"{}\"{disabled}>",
                c.to_uppercase()
            ));
        }
    }
    out.push_str(&fate_span(fate));
    out.push_str("</div>\n");
    out
}

/// The answer page (Do 6): the opening as the CLI prints it, the rows as
/// a table with their art, the continuation as a link, then the terms,
/// the routes and the basis as the CLI prints them; the query box, then
/// the save form. Opened from a saved search, it says so above the answer
/// and carries the search through its query box and its continuation.
pub(crate) fn answer(a: &Answer, opened: Option<&Opened>, links: &SavedLinks) -> String {
    let now = acquisition_store::now();
    let mut out = opened
        .map(|o| opened_notice(a, o, links))
        .unwrap_or_default();
    out.push_str(&format!(
        "<p class=\"count\">{}</p>\n",
        plural(a.total.matched, "match", "matches")
    ));
    let carried = opened.map(|o| (o.id.as_str(), o.at));
    let carried_query = carried
        .map(|(id, at)| format!("&saved={}&saved_revision={at}", enc(id)))
        .unwrap_or_default();
    out.push_str(&answer_badges(a));
    out.push_str(&pre(&search_cmd::answer_head(a)));
    if !a.rows.is_empty() {
        out.push_str("<table class=\"results\">\n<tr><th></th><th>Item</th><th>Lines</th><th>Place</th></tr>\n");
        for row in &a.rows {
            let name = search_cmd::title(row.name.as_deref(), row.typeline.as_deref());
            let icon = match &row.icon {
                Some(Some(url)) => format!("<img src=\"{}\" alt=\"{}\">", esc(url), esc(&name)),
                _ => String::new(),
            };
            let (shows, more) = search_cmd::row_shows(a, row);
            let mut place = search_cmd::place_text(&row.place, now);
            if let Some(removed_at) = row.removed_at {
                place.push_str(&format!(
                    " · removed {}",
                    crate::store_cmd::ago(now, Some(removed_at))
                ));
            }
            // the card's order: the name in its rarity's colour and the
            // base under it, then the lines, then where it lies
            let rarity = row.rarity.as_deref().unwrap_or("");
            out.push_str(&format!(
                "<tr><td class=\"icon\">{icon}</td><td><a class=\"name rarity-{}\" href=\"item/{}\">{}</a><br><span class=\"base\">{}</span> <span class=\"fate\">{}</span></td><td>{}</td><td>{}</td></tr>\n",
                esc(&rarity.to_lowercase()),
                enc(&row.id),
                esc(&name),
                esc(row.base.as_deref().unwrap_or("")),
                esc(&rarity.to_lowercase()),
                shows
                    .iter()
                    .chain(&more)
                    .map(|s| esc(s))
                    .collect::<Vec<_>>()
                    .join("<br>"),
                esc(&place)
            ));
        }
        out.push_str("</table>\n");
    }
    if let ViewOut::Rows(rows) = &a.view
        && rows.left_out > 0
    {
        let shown = if rows.returned == 0 {
            "none".to_string()
        } else {
            format!("rows {}–{}", rows.after + 1, rows.after + rows.returned)
        };
        out.push_str(&match &rows.next {
            Some(next) => format!(
                "<p>{shown} of {} shown · <a href=\"search?next={}{}\">more</a></p>\n",
                a.total.matched,
                enc(&next.request.token()),
                esc(&carried_query)
            ),
            None => format!(
                "<p>{shown} of {} shown · a larger number of rows returns the rest</p>\n",
                a.total.matched
            ),
        });
    }
    out.push_str(&pre(&search_cmd::answer_tail(a, None)));
    out.push_str(&query_box(
        a.scope.realm.as_str(),
        Some(a.scope.membership.as_str()),
        &a.query.text,
        carried,
    ));
    out.push_str(&save_forms(a, opened, links));
    out
}

/// One item (Do 6): its art, then `show` as the CLI prints it, the lines
/// in the order it gives.
pub(crate) fn item(shown: &Shown, badges: &str) -> String {
    let name = search_cmd::title(shown.item.name.as_deref(), shown.item.typeline.as_deref());
    let icon = shown
        .item
        .icon
        .as_ref()
        .map(|url| format!("<p><img src=\"{}\" alt=\"{}\"></p>\n", esc(url), esc(&name)))
        .unwrap_or_default();
    format!("{icon}{badges}{}", pre(&search_cmd::shown_text(shown)))
}

fn badge(words: &str) -> String {
    format!("<span class=\"badge\">{}</span>", esc(words))
}

/// The moved-world review's badges on an answer (Do 9): a site search
/// that is not the site's, and what is undecided with its reasons — each
/// a state the answer carries, which its text says below.
fn answer_badges(a: &Answer) -> String {
    let mut out = Vec::new();
    let typed = a.set_aside.as_ref().unwrap_or(&a.query);
    if let Some(trade) = &typed.trade {
        let count = |status: Status| trade.clauses.iter().filter(|c| c.status == status).count();
        match trade.fidelity {
            Fidelity::Partial => out.push(badge(&format!(
                "not the site's search: {} not run",
                plural(count(Status::Unsupported), "clause", "clauses")
            ))),
            Fidelity::Ambiguous => out.push(badge(&format!(
                "ambiguous: {} as the site may not read",
                plural(count(Status::Ambiguous), "clause", "clauses")
            ))),
            Fidelity::Exact => {}
        }
    }
    if a.total.undecided.count > 0 {
        out.push(badge(&format!("{} undecided", a.total.undecided.count)));
        for reason in &a.total.undecided_reasons {
            out.push(badge(&format!(
                "{} unread — e.g. {}",
                reason.reason.unread,
                reason.example.name.as_deref().unwrap_or("(no name)")
            )));
        }
    }
    if out.is_empty() {
        return String::new();
    }
    format!("<p>{}</p>\n", out.join(" "))
}

/// The moved-world review's badges on one item (Do 9): a class the table
/// could not decide, each part unread, each line whose flags are unknown
/// — each a state `show`'s document carries, which its text says below.
pub(crate) fn item_badges(shown: &Shown) -> String {
    let mut out = Vec::new();
    match &shown.class {
        Classed::Is(_) | Classed::All(_) => {}
        Classed::Open(why) | Classed::Among { why, .. } => {
            out.push(badge(&format!("class undecided — {}", why.problem)))
        }
        Classed::BaseUnread | Classed::FrameUnread { .. } | Classed::BestiaryUnread { .. } => {
            out.push(badge("class undecided"))
        }
    }
    for unread in &shown.item.unread {
        out.push(badge(&format!("unread — {}", unread.problem)));
    }
    for line in &shown.lines {
        let line = &line.line;
        if !line.flags_unknown.is_empty() {
            out.push(badge(&format!(
                "flags unknown: {} — {}",
                line.flags_unknown.join(", "),
                line.text.replace('\n', " / ")
            )));
        }
        if line.flags_unread {
            out.push(badge(&format!(
                "flags unread — {}",
                line.text.replace('\n', " / ")
            )));
        }
    }
    if out.is_empty() {
        return String::new();
    }
    format!("<p>{}</p>\n", out.join(" "))
}

/// The league control both pages carry (the 4b sitting round, Do 3): a
/// select over the store's league list, each league marked with whether
/// tabs of it are on disk in the realm shown; with no list on disk, no
/// select — the fetch button is then the only control. `any` adds the
/// search's "every league", which names no league term.
pub(crate) fn league_select(
    leagues: Option<&[(String, bool)]>,
    selected: Option<&str>,
    any: bool,
) -> String {
    let Some(leagues) = leagues else {
        return String::new();
    };
    let mut out = String::from("<label class=\"control\">League <select name=\"league\">");
    if any {
        out.push_str("<option value=\"\">every league</option>");
    }
    for (league, on_disk) in leagues {
        out.push_str(&format!(
            "<option value=\"{}\"{}>{} ({})</option>",
            esc(league),
            if selected == Some(league.as_str()) {
                " selected"
            } else {
                ""
            },
            esc(league),
            if *on_disk {
                "tabs on disk"
            } else {
                "no tabs on disk"
            }
        ));
    }
    out.push_str("</select></label> ");
    out
}

/// The button that fetches the league list: the `leagues` job, one
/// request, answered on the jobs page.
pub(crate) fn fetch_leagues() -> String {
    "<form method=\"post\" action=\"leagues\"><button type=\"submit\">Fetch the league list</button></form>\n".to_string()
}
