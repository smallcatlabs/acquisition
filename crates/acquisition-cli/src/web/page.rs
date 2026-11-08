//! The pages (C16, a renderer, not an app; C53, text is a function of the
//! JSON): each renders the documents a verb produced, as HTML — one, but
//! for an answer opened from a saved search, which renders the answer and
//! the saved row it was opened from, and a conflict's refusal, which
//! renders the row as it stands — so the page states nothing the terminal
//! does not. The answer and item pages render their documents themselves
//! (4-full part 1, step 4): each fact in the words the CLI's text says it
//! in, made once in `search_cmd.rs`, the layout the page's, and every way
//! on the page's own ([`answer`]), so they print no command line. The
//! plan, refresh and jobs pages still print the CLI's text, in a `pre`
//! (`mod.rs` renders them through [`pre`]), until they are rendered from
//! their documents too (#182). A page sorts, filters, groups and composes
//! nothing; where it wants what no document carries, the search crate or
//! the verb grows first. Every version a page renders is copied into its
//! head's basis line, never composed ([`Head::basis`]).

use acquisition_search::Classed;
use acquisition_search::answer::{
    Answer, Count, ExplainOut, Request, RowsOut, TradeOut, ViewOut, Zero,
};
use acquisition_search::counts::{CrossOut, Table};
use acquisition_search::show::Shown;
use acquisition_search::trade::translate::{Fidelity, Status};
use acquisition_verbs::refusal::Way;
use acquisition_verbs::saved::{ReadRow, SavedSearches};

use super::form::{Fate, Filter, Form, LIMITS, STAT_ROWS};
use crate::search_cmd::{self, RowPiece};
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
    out.push_str(&query_box(realm, None, "", None, None));
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
/// with the revision its URL names, so the loop survives an edit. Its
/// rows' order and number are the big form's three controls, "Show",
/// "Sort by" and "highest first" (the page seat's fault 6), set on an
/// answer to the rows view it showed; "Show" offers the search's own
/// default first, sent empty, so a box left as it is asks no limit.
pub(crate) fn query_box(
    realm: &str,
    membership: Option<&str>,
    text: &str,
    saved: Option<(&str, i64)>,
    rows: Option<&RowsOut>,
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
    let listed = acquisition_store::bound::LISTED;
    let limit = rows.map_or(listed, |r| r.limit);
    let mut limits = vec![(String::new(), listed)];
    limits.extend(LIMITS.iter().map(|n| (n.to_string(), *n)));
    if !limits.iter().any(|(_, n)| *n == limit) {
        limits.push((limit.to_string(), limit));
        limits.sort_by_key(|(_, n)| *n);
    }
    let shown: String = limits
        .iter()
        .map(|(value, n)| {
            format!(
                "<option value=\"{value}\"{}>{n}</option>",
                if *n == limit { " selected" } else { "" }
            )
        })
        .collect();
    let sort = rows.and_then(|r| r.sort.as_deref()).unwrap_or_default();
    let desc = if rows.is_some_and(|r| r.desc) {
        " checked"
    } else {
        ""
    };
    format!(
        "<form class=\"query-box\" method=\"get\" action=\"search\">{carried}<label class=\"filter-title\" for=\"query-text\">The query, in the language</label><textarea id=\"query-text\" name=\"text\" rows=\"3\">{}</textarea><label class=\"control\">Show <select name=\"view.limit\">{shown}</select></label> <label class=\"control\">Sort by <input name=\"view.sort\" value=\"{}\" placeholder=\"ilvl\"></label> <label class=\"control\"><input type=\"checkbox\" name=\"view.desc\" value=\"true\"{desc}> highest first</label> <button type=\"submit\">Run this query</button></form>\n",
        esc(text),
        esc(sort)
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

/// How a page spells a way on: the link `Paged` makes of it, or none
/// where no page takes it (`mod.rs`'s `Paged`, the page's `Surface`).
pub(crate) type Spell<'a> = &'a dyn Fn(&Way) -> Option<String>;

/// A way on where it stands on a page: a link where the page's surface
/// spells one, its words in a span where it spells none; each marked
/// `class="way"` with `data-way`, the JSON pointer of the object holding
/// its request in the page's document (or `tabs`, the scope's
/// never-fetched tabs), `href` the link's last attribute. `extra` is
/// appended to the link: the saved search an answer was opened from.
fn way_on(spell: Spell, at: &str, way: &Way, words: &str, extra: &str) -> String {
    match spell(way) {
        Some(link) => format!(
            "<a class=\"way\" data-way=\"{}\" href=\"{}\">{}</a>",
            esc(at),
            esc(&format!("{link}{extra}")),
            esc(words)
        ),
        None => format!(
            "<span class=\"way\" data-way=\"{}\">{}</span>",
            esc(at),
            esc(words)
        ),
    }
}

/// A search's way on at `at`.
fn searched(spell: Spell, at: &str, request: &Request, words: &str) -> String {
    way_on(
        spell,
        at,
        &Way::Search(Box::new(request.clone())),
        words,
        "",
    )
}

/// A count as a page writes it, in `words`: a way where its holder
/// carries a request, else the words alone.
fn count_at(spell: Spell, at: &str, count: &Count, words: &str) -> String {
    match &count.route {
        Some(route) => searched(spell, at, &route.request, words),
        None => esc(words),
    }
}

/// One row of a two-column table: a label and its cell, already HTML.
fn fact_row(label: &str, cell: &str) -> String {
    format!("<tr><th>{}</th><td>{cell}</td></tr>\n", esc(label))
}

/// The answer page (4-full part 1, step 4): the answer document rendered
/// as HTML in C115's order — the count and the badges, a site search's
/// translation, the opening (the query, the scope, the total), the rows
/// with their continuation or a count's tables, what a zero answer
/// offers, why items are undecided, the terms, the counts left without a
/// route, the basis — then the query box and the save form. The words of
/// each fact are the text's, made once in `search_cmd.rs`; every request
/// the document carries is a way on through `spell`, a link where a page
/// takes it and its words where none does, and no command line is
/// printed. Opened from a saved search, it says so above the answer and
/// carries the search through its query box and its continuation.
pub(crate) fn answer(
    a: &Answer,
    opened: Option<&Opened>,
    links: &SavedLinks,
    spell: Spell,
) -> String {
    let now = acquisition_store::now();
    let all_realms = a.scope.realm == acquisition_search::Realm::All;
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
    let (typed, typed_at) = match &a.set_aside {
        Some(aside) => (aside, "/set_aside"),
        None => (&a.query, "/query"),
    };
    if let Some(trade) = &typed.trade {
        out.push_str(&trade_section(trade, typed_at, spell));
    }
    out.push_str(&opening(a, now, spell));
    match &a.view {
        ViewOut::Rows(rows) => {
            out.push_str(&rows_table(a, now));
            if rows.left_out > 0 {
                let shown = esc(&search_cmd::rows_shown_words(a, rows));
                out.push_str(&match &rows.next {
                    Some(next) => format!(
                        "<p>{shown} · {}</p>\n",
                        way_on(
                            spell,
                            "/view/rows/next",
                            &Way::Search(Box::new(next.request.clone())),
                            "more",
                            &carried_query
                        )
                    ),
                    None => format!("<p>{shown} · a larger number of rows returns the rest</p>\n"),
                });
            }
        }
        ViewOut::Counts(counts) => {
            if let Some(sum) = &counts.sum {
                out.push_str(&format!(
                    "<p class=\"sum\">Sum {}</p>\n",
                    esc(&search_cmd::sum_of_text(sum))
                ));
            }
            for (t, table) in counts.tables.iter().enumerate() {
                out.push_str(&count_table(table, t, all_realms, spell));
            }
        }
        ViewOut::Cross(cross) => out.push_str(&cross_table(cross, all_realms, spell)),
        ViewOut::Explain(explain) => out.push_str(&explained(explain, spell)),
    }
    if let Some(zero) = &a.zero {
        out.push_str(&zero_block(zero, spell));
    }
    out.push_str(&reasons(a));
    out.push_str(&terms(a, now, spell));
    let unrouted = search_cmd::counted(a).unrouted();
    if unrouted > 0 {
        out.push_str(&format!(
            "<p class=\"unrouted\">{}</p>\n",
            esc(&format!(
                "{} {} no link: an answer carries those routes only where every route is asked, which no page asks",
                plural(unrouted, "count", "counts"),
                if unrouted == 1 { "carries" } else { "carry" }
            ))
        ));
    }
    out.push_str(&basis_foot(&search_cmd::basis_text(&a.basis)));
    out.push_str(&query_box(
        a.scope.realm.as_str(),
        Some(a.scope.membership.as_str()),
        &a.query.text,
        carried,
        match &a.view {
            ViewOut::Rows(rows) => Some(rows),
            _ => None,
        },
    ));
    out.push_str(&save_forms(a, opened, links));
    out
}

/// The basis at the foot of an answer or an item, in the text's words:
/// what a person quotes to an agent and an agent to a person (the
/// design's §3). Its line in the page source is the text's line, so a
/// recording masks its digests as a text's.
fn basis_foot(words: &str) -> String {
    format!("<p class=\"basis\">\n{}\n</p>\n", esc(words))
}

/// A site search's translation (C113): its source and fidelity, what it
/// runs over, then every clause the document lists as not run, ambiguous
/// or reported, each its status, where it stands and what it says, an
/// ambiguous clause's other reading with its adds and drops as ways.
/// `at` is the pointer of the query holding it.
fn trade_section(trade: &TradeOut, at: &str, spell: Spell) -> String {
    let mut out = format!(
        "<section class=\"trade\">\n<p>{}</p>\n<p>{}</p>\n",
        esc(&search_cmd::fidelity_words(trade)),
        esc(&format!("Over {}", search_cmd::TRADE_OVER))
    );
    let listed = search_cmd::clauses_listed(trade);
    if listed.iter().any(|(_, clauses)| !clauses.is_empty()) {
        out.push_str(
            "<table class=\"clauses\">\n<tr><th>Status</th><th>Clause</th><th>Says</th></tr>\n",
        );
        for (word, clauses) in &listed {
            for (i, c) in clauses {
                let stake = c.other.as_ref().map_or(String::new(), |o| {
                    let of = |what: &str, count: &Count| {
                        count_at(
                            spell,
                            &format!("{at}/trade/clauses/{i}/other/{what}"),
                            count,
                            &count.count.to_string(),
                        )
                    };
                    format!(
                        " · {}",
                        search_cmd::stake_words(&of("adds", &o.adds), &of("drops", &o.drops))
                    )
                });
                out.push_str(&format!(
                    "<tr><td>{}</td><td>{}</td><td>{}{stake}</td></tr>\n",
                    esc(word),
                    esc(&c.at),
                    esc(search_cmd::clause_note(c))
                ));
            }
        }
        out.push_str("</table>\n");
    }
    out.push_str("</section>\n");
    out
}

/// The answer's opening: the query, the query set aside, the scope with
/// its never-fetched tabs as a way, the total with its undecided as a
/// way, and what a route said when it was printed.
fn opening(a: &Answer, now: i64, spell: Spell) -> String {
    let mut out = String::from("<table class=\"opening\">\n");
    out.push_str(&fact_row(
        "Query",
        &esc(search_cmd::query_words(&a.query.text)),
    ));
    if let Some(aside) = &a.set_aside {
        out.push_str(&fact_row(
            "Set aside",
            &esc(&search_cmd::aside_words(&aside.text, "")),
        ));
    }
    let mut scope = format!(
        "{}<br>{}",
        esc(&search_cmd::scope_words(&a.scope)),
        esc(&search_cmd::never_fetched_words(&a.scope, now))
    );
    // the list that exists: the tabs page of a league the query names, or
    // of the one holding the most never fetched, the others counted
    if let Some(pointer) = a.tabs_pointer()
        && let Some(realm) = acquisition_protocol::realm::Realm::parse(&pointer.realm)
    {
        let way = Way::Tabs {
            account: pointer.account.clone(),
            realm,
            league: pointer.league.clone(),
        };
        scope.push_str(&format!(
            " · {}",
            way_on(
                spell,
                "tabs",
                &way,
                &format!("each tab of {}", pointer.league),
                ""
            )
        ));
        if pointer.other_leagues > 0 {
            scope.push_str(&format!(
                " · {}",
                esc(&search_cmd::other_leagues_words(pointer.other_leagues))
            ));
        }
    }
    if let Some(words) = search_cmd::realm_not_held_words(&a.scope) {
        scope.push_str(&format!("<br>{}", esc(&words)));
    }
    out.push_str(&fact_row("Scope", &scope));
    let mut total = esc(&search_cmd::total_words(a, now));
    if a.total.undecided.count > 0 {
        total.push_str(&format!(
            " · {}",
            count_at(
                spell,
                "/total/undecided",
                &a.total.undecided,
                &search_cmd::undecided_words(a.total.undecided.count)
            )
        ));
    }
    if let Some(words) = search_cmd::never_fetched_sentence(a.scope.coverage.never_fetched) {
        total.push_str(&format!("<br>{}", esc(&words)));
    }
    out.push_str(&fact_row("Total", &total));
    if let Some(words) = search_cmd::before_words(a) {
        out.push_str(&fact_row("Then", &esc(&words)));
    }
    out.push_str("</table>\n");
    out
}

/// The rows, a row per item with its art, its name a link to the item
/// page, what it shows of its terms and its sort — what it leaves out a
/// link to the item whole — and where it lies.
fn rows_table(a: &Answer, now: i64) -> String {
    if a.rows.is_empty() {
        return String::new();
    }
    let mut out = String::from(
        "<table class=\"results\">\n<tr><th></th><th>Item</th><th>Lines</th><th>Place</th></tr>\n",
    );
    for row in &a.rows {
        let name = search_cmd::title(row.name.as_deref(), row.typeline.as_deref());
        let icon = match &row.icon {
            Some(Some(url)) => format!("<img src=\"{}\" alt=\"{}\">", esc(url), esc(&name)),
            _ => String::new(),
        };
        let whole =
            |words: String| format!("<a href=\"item/{}\">{}</a>", enc(&row.id), esc(&words));
        let (shows, more) = search_cmd::row_shows(a, row);
        let lines: Vec<String> = shows
            .into_iter()
            .map(|piece| match piece {
                RowPiece::Seen(s) => esc(&s),
                RowPiece::MoreSorted(n) => whole(search_cmd::more_sorted_words(n)),
            })
            .chain(
                more.iter()
                    .map(|(n, path)| whole(search_cmd::more_of_term_words(*n, path))),
            )
            .collect();
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
            lines.join("<br>"),
            esc(&place)
        ));
    }
    out.push_str("</table>\n");
    out
}

/// One count table: its heading, then a row per bucket — its count, a
/// way where it carries a request, its label, sum, slots and what it
/// could not read; a vocabulary row's term and its kinds, each a way
/// where it carries one; what a value needs.
fn count_table(table: &Table, t: usize, all_realms: bool, spell: Spell) -> String {
    let mut out = format!(
        "<p class=\"count-head\">{}</p>\n<table class=\"counted\">\n<tr><th>Items</th><th>{}</th></tr>\n",
        esc(&format!(
            "Count {}",
            search_cmd::table_head_words(table, "")
        )),
        esc(&table.key)
    );
    for (b, bucket) in table.buckets.iter().enumerate() {
        let at = format!("/view/counts/tables/{t}/buckets/{b}");
        let mut cell = esc(&search_cmd::label_text(&bucket.label, all_realms));
        if let Some(sum) = &bucket.sum {
            cell.push_str(&esc(&format!(" · {}", search_cmd::sum_words(sum))));
        }
        for words in [
            search_cmd::slots_words(bucket),
            search_cmd::tally_words(&bucket.tally),
        ]
        .into_iter()
        .flatten()
        {
            cell.push_str(&format!(" · {}", esc(&words)));
        }
        if !bucket.sources.is_empty() {
            if let Some(term) = &bucket.label.term {
                cell.push_str(&format!("<br>{}", esc(term)));
            }
            let kinds: Vec<String> = bucket
                .sources
                .iter()
                .enumerate()
                .map(|(k, kind)| (format!("{at}/sources/{k}"), kind))
                .chain(
                    bucket
                        .flags
                        .iter()
                        .enumerate()
                        .map(|(k, kind)| (format!("{at}/flags/{k}"), kind)),
                )
                .map(|(at, kind)| count_at(spell, &at, &kind.count, &search_cmd::kind_words(kind)))
                .collect();
            cell.push_str(&format!("<br>{}", kinds.join(" · ")));
            for kind in bucket.sources.iter().chain(&bucket.flags) {
                if let Some(needs) = &kind.needs {
                    cell.push_str(&format!(
                        "<br>{}",
                        esc(&search_cmd::needs_words(&kind.kind, needs))
                    ));
                }
            }
        }
        if let Some(needs) = &bucket.label.needs {
            cell.push_str(&format!("<br>{}", esc(needs)));
        }
        out.push_str(&format!(
            "<tr><td>{}</td><td>{cell}</td></tr>\n",
            count_at(spell, &at, &bucket.count, &bucket.count.count.to_string())
        ));
    }
    out.push_str("</table>\n");
    if let Some(words) = search_cmd::unmatched_words(&table.unmatched_leagues) {
        out.push_str(&format!("<p>{}</p>\n", esc(&words)));
    }
    out
}

/// A crossed table: its heading, a row per cell — its count, a way where
/// it carries a request, its values and sum, what a value needs — then
/// each key's items it could not place, each a way where it carries one.
fn cross_table(cross: &CrossOut, all_realms: bool, spell: Spell) -> String {
    let mut out = String::new();
    if let Some(sum) = &cross.sum {
        out.push_str(&format!(
            "<p class=\"sum\">Sum {}</p>\n",
            esc(&search_cmd::sum_of_text(sum))
        ));
    }
    out.push_str(&format!(
        "<p class=\"count-head\">{}</p>\n<table class=\"counted\">\n<tr><th>Items</th><th>{}</th></tr>\n",
        esc(&format!("Cross {}", search_cmd::cross_head_words(cross, ""))),
        esc(&cross.keys.join(" × "))
    ));
    for (c, cell) in cross.cells.iter().enumerate() {
        let of: Vec<String> = cell
            .of
            .iter()
            .map(|l| search_cmd::label_text(l, all_realms))
            .collect();
        let mut words = esc(&of.join(" × "));
        if let Some(sum) = &cell.sum {
            words.push_str(&esc(&format!(" · {}", search_cmd::sum_words(sum))));
        }
        for label in &cell.of {
            if let Some(needs) = &label.needs {
                words.push_str(&format!(
                    "<br>{}",
                    esc(&search_cmd::needs_words(
                        &search_cmd::label_text(label, all_realms),
                        needs
                    ))
                ));
            }
        }
        out.push_str(&format!(
            "<tr><td>{}</td><td>{words}</td></tr>\n",
            count_at(
                spell,
                &format!("/view/cross/cells/{c}"),
                &cell.count,
                &cell.count.count.to_string()
            )
        ));
    }
    out.push_str("</table>\n");
    for (m, margin) in cross.margins.iter().enumerate() {
        let carried = margin.none.route.is_some() || margin.undecided.route.is_some();
        if margin.none.count + margin.undecided.count > 0 || carried {
            let at = |what: &str| format!("/view/cross/margins/{m}/{what}");
            out.push_str(&format!(
                "<p>{}: {} · {}</p>\n",
                esc(&margin.key),
                count_at(
                    spell,
                    &at("none"),
                    &margin.none,
                    &search_cmd::none_words(margin.none.count)
                ),
                count_at(
                    spell,
                    &at("undecided"),
                    &margin.undecided,
                    &search_cmd::undecided_words(margin.undecided.count)
                )
            ));
        }
    }
    if let Some(tally) = search_cmd::cross_tally_words(&cross.tally) {
        out.push_str(&format!("<p>{}</p>\n", esc(&tally)));
    }
    if let Some(words) = search_cmd::unmatched_words(&cross.unmatched_leagues) {
        out.push_str(&format!("<p>{}</p>\n", esc(&words)));
    }
    out
}

/// One node explained: the root's counts as given, then forced true and
/// forced false, each count a way where it carries a request, beside the
/// transformed query.
fn explained(explain: &ExplainOut, spell: Spell) -> String {
    let mut out = format!(
        "<p class=\"count-head\">{}</p>\n<table class=\"explained\">\n",
        esc(&format!("Explain {}", search_cmd::explain_words(explain)))
    );
    out.push_str(&fact_row(
        "As given",
        &esc(&search_cmd::forced_words(
            explain.given.matched,
            explain.given.undecided,
        )),
    ));
    for (label, key, forced) in [
        ("Forced true", "forced_true", &explain.forced_true),
        ("Forced false", "forced_false", &explain.forced_false),
    ] {
        let mut cell = count_at(
            spell,
            &format!("/view/explain/{key}/matched"),
            &forced.matched,
            &plural(forced.matched.count, "match", "matches"),
        );
        if forced.undecided.count > 0 || forced.undecided.route.is_some() {
            cell.push_str(&format!(
                " · {}",
                count_at(
                    spell,
                    &format!("/view/explain/{key}/undecided"),
                    &forced.undecided,
                    &search_cmd::undecided_words(forced.undecided.count)
                )
            ));
        }
        cell.push_str(&format!("<br>{}", esc(&forced.query.text)));
        out.push_str(&fact_row(label, &cell));
    }
    out.push_str("</table>\n");
    out
}

/// What a zero answer says: its undecided as a way, each selector nothing
/// carries with its suggestions, then its offers — each part to explain
/// and the corpus, in words, as no page asks either.
fn zero_block(zero: &Zero, spell: Spell) -> String {
    let mut out = String::from("<div class=\"zero\">\n");
    if zero.undecided.count > 0 || zero.undecided.route.is_some() {
        out.push_str(&format!(
            "<p>No match · {}</p>\n",
            count_at(
                spell,
                "/zero/undecided",
                &zero.undecided,
                &search_cmd::undecided_words(zero.undecided.count)
            )
        ));
    }
    for nothing in &zero.resolved_to_nothing {
        out.push_str(&format!(
            "<p>{}</p>\n",
            esc(&search_cmd::nothing_words(zero, nothing))
        ));
        if !nothing.suggestions.is_empty() {
            out.push_str("<ul>\n");
            for s in &nothing.suggestions {
                out.push_str(&format!(
                    "<li>{}</li>\n",
                    esc(&search_cmd::suggestion_words(s, " "))
                ));
            }
            out.push_str("</ul>\n");
        }
    }
    let offers = &zero.offers;
    if !offers.explain.is_empty() {
        out.push_str(&format!(
            "<p>{}</p>\n<ul class=\"offers\">\n",
            esc(&format!("Offers: {}", search_cmd::EXPLAIN_OFFERS))
        ));
        for (i, offer) in offers.explain.iter().enumerate() {
            out.push_str(&format!(
                "<li>{}</li>\n",
                searched(
                    spell,
                    &format!("/zero/offers/explain/{i}"),
                    &offer.request,
                    &format!("part {}: {}", offer.path, offer.node)
                )
            ));
        }
        if offers.explain_left_out > 0 {
            out.push_str(&format!(
                "<li>{}</li>\n",
                esc(&search_cmd::more_parts_words(offers.explain_left_out))
            ));
        }
        out.push_str("</ul>\n");
    }
    if let Some(offer) = &offers.context {
        out.push_str(&format!(
            "<p>{}</p>\n",
            searched(
                spell,
                "/zero/offers/context",
                &offer.request,
                search_cmd::CONTEXT_OFFER
            )
        ));
    }
    out.push_str("</div>\n");
    out
}

/// Why items are undecided: each reason, its example a link to the item
/// page, its problem and what may help; the rest counted.
fn reasons(a: &Answer) -> String {
    if a.total.undecided_reasons.is_empty() {
        return String::new();
    }
    let mut out = String::from(
        "<table class=\"reasons\">\n<tr><th>Unread</th><th>Example</th><th>Why</th></tr>\n",
    );
    for reason in &a.total.undecided_reasons {
        out.push_str(&format!(
            "<tr><td>{}</td><td><a href=\"item/{}\">{}</a></td><td>{}</td></tr>\n",
            esc(&reason.reason.unread),
            enc(&reason.example.id),
            esc(search_cmd::example_name(&reason.example)),
            esc(&search_cmd::reason_words(&reason.reason))
        ));
    }
    out.push_str("</table>\n");
    if a.total.reasons_left_out > 0 {
        out.push_str(&format!(
            "<p>{}</p>\n",
            esc(&search_cmd::reasons_left_out_words(
                a.total.reasons_left_out,
                "above"
            ))
        ));
    }
    out
}

/// The terms: what they count over, then a row per term — its path, its
/// term, its four counts and its only-together count, each a way where it
/// carries a request — its value list beneath it, every value the
/// document lists and the rest a way where it carries one, and its
/// removed note with its way.
fn terms(a: &Answer, now: i64, spell: Spell) -> String {
    if a.terms.is_empty() {
        return String::new();
    }
    let mut out = format!(
        "<p class=\"terms-head\">{}</p>\n<table class=\"terms\">\n<tr><th>Path</th><th>Term</th><th>Matched</th><th>Failed</th><th>Lacked</th><th>Undecided</th><th>Only together</th></tr>\n",
        esc(&format!("Terms: {}", search_cmd::terms_words(&a.scope)))
    );
    for (i, term) in a.terms.iter().enumerate() {
        let at = |what: &str| format!("/terms/{i}/{what}");
        let cell = |what: &str, count: &Count| {
            format!(
                "<td>{}</td>",
                count_at(spell, &at(what), count, &count.count.to_string())
            )
        };
        let together = match &term.together {
            Some(t) if t.count.count > 0 || t.count.route.is_some() => count_at(
                spell,
                &at("together"),
                &t.count,
                &search_cmd::together_words(t),
            ),
            _ => String::new(),
        };
        out.push_str(&format!(
            "<tr><td>{}</td><td>{}</td>{}{}{}{}<td>{together}</td></tr>\n",
            esc(&term.path),
            esc(&term.term),
            cell("matched", &term.matched),
            cell("failed", &term.failed),
            cell("lacked", &term.lacked),
            cell("undecided", &term.undecided)
        ));
        if let Some(resolved) = &term.resolved {
            let mut values: Vec<String> = resolved
                .values
                .iter()
                .map(|v| esc(&search_cmd::carried_words(v)))
                .collect();
            let more = search_cmd::more_words(resolved.more);
            match &resolved.rest {
                Some(rest) => {
                    values.push(searched(spell, &at("resolved/rest"), &rest.request, &more))
                }
                None if resolved.more > 0 => values.push(esc(&more)),
                None => {}
            }
            if !values.is_empty() {
                out.push_str(&format!(
                    "<tr class=\"values\"><td></td><td colspan=\"6\">→ {}</td></tr>\n",
                    values.join(" · ")
                ));
            }
        }
        if let Some(removed) = &term.removed {
            let then = match &removed.route {
                Some(route) => format!(
                    " · {}",
                    searched(
                        spell,
                        &at("removed"),
                        &route.request,
                        "find it under membership all"
                    )
                ),
                None => format!(" {}", esc(search_cmd::REMOVED_HELD_NONE)),
            };
            out.push_str(&format!(
                "<tr class=\"removed\"><td></td><td colspan=\"6\">{}{then}</td></tr>\n",
                esc(&search_cmd::removed_words(removed, now))
            ));
        }
    }
    out.push_str("</table>\n");
    out
}

/// One item (4-full part 1, step 4): `show`'s document rendered as HTML
/// — its art and its badges, its title, then each part `show` states in
/// the text's words, made once in `search_cmd.rs`: the price and why
/// there is none, with no command and no link — the lines with their
/// source, template and slots, what is unread, the item against a query
/// where one was asked, and the basis.
pub(crate) fn item(shown: &Shown, badges: &str) -> String {
    let it = &shown.item;
    let now = acquisition_store::now();
    let name = search_cmd::title(it.name.as_deref(), it.typeline.as_deref());
    let icon = it
        .icon
        .as_ref()
        .map(|url| format!("<p><img src=\"{}\" alt=\"{}\"></p>\n", esc(url), esc(&name)))
        .unwrap_or_default();
    let rarity = it.rarity.as_deref().unwrap_or("").to_lowercase();
    let mut out = format!(
        "{icon}{badges}<p class=\"name rarity-{}\">{}</p>\n<table class=\"shown\">\n",
        esc(&rarity),
        esc(&search_cmd::shown_title(shown))
    );
    out.push_str(&fact_row("Id", &esc(&it.facts.id)));
    if let Some(removed_at) = it.facts.removed_at {
        out.push_str(&fact_row(
            "Removed",
            &esc(&search_cmd::shown_removed_words(now, removed_at)),
        ));
    }
    out.push_str(&fact_row(
        "Place",
        &esc(&search_cmd::shown_place_words(shown, now)),
    ));
    if let Some(base) = &it.base {
        out.push_str(&fact_row("Base", &esc(base)));
    }
    out.push_str(&fact_row(
        "Class",
        &esc(&search_cmd::class_words(&shown.class)),
    ));
    let mut category = esc(&search_cmd::category_words(&shown.category));
    for open in &shown.category.open {
        category.push_str(&format!("<br>{}", esc(&search_cmd::open_words(open))));
    }
    out.push_str(&fact_row("Category", &category));
    out.push_str(&fact_row("Price", &esc(&search_cmd::price_words(shown))));
    if let Some(stack) = it.stack {
        out.push_str(&fact_row("Stack", &esc(&stack.to_string())));
    }
    if let Some(note) = &it.note {
        out.push_str(&fact_row("Note", &esc(note)));
    }
    if !it.flags.is_empty() {
        out.push_str(&fact_row("Is", &esc(&it.flags.join(", "))));
    }
    if let Some(sockets) = search_cmd::sockets_words(it) {
        out.push_str(&fact_row("Sockets", &esc(&sockets)));
    }
    let properties = search_cmd::shown_properties(it);
    if !properties.is_empty() {
        let cell: Vec<String> = properties.iter().map(|p| esc(p)).collect();
        out.push_str(&fact_row("Shown", &cell.join("<br>")));
    }
    let unread = match it.unread.is_empty() {
        true => esc(search_cmd::UNREAD_NOTHING),
        false => it
            .unread
            .iter()
            .map(|u| esc(&u.problem))
            .collect::<Vec<_>>()
            .join("<br>"),
    };
    out.push_str(&fact_row("Unread", &unread));
    out.push_str("</table>\n");
    if !shown.lines.is_empty() {
        out.push_str(
            "<table class=\"lines\">\n<tr><th>Source</th><th>Line</th><th>Template</th><th>Slots</th></tr>\n",
        );
        // the template and its slots each a cell, a row of a mod of
        // several and a limit a line of the template's cell beside its
        // own slots, so no gap of the text's is lost to HTML's spaces
        for l in &shown.lines {
            let mut template = vec![esc(&search_cmd::line_template_quoted(l))];
            let mut slots = vec![esc(&search_cmd::line_slots_words(l).unwrap_or_default())];
            for row in &l.line.rows {
                template.push(esc(&search_cmd::line_row_quoted(row)));
                slots.push(esc(
                    &search_cmd::line_row_slots_words(row).unwrap_or_default()
                ));
            }
            template.extend(l.limit.map(esc));
            out.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                esc(&search_cmd::line_kind_words(l)),
                esc(&l.line.text.replace('\n', " / ")),
                template.join("<br>"),
                slots.join("<br>")
            ));
        }
        out.push_str("</table>\n");
    }
    if let Some(against) = &shown.against {
        out.push_str(&format!(
            "<p class=\"against\">{}</p>\n<table class=\"against\">\n<tr><th>Path</th><th>Node</th><th>Outcome</th></tr>\n",
            esc(&format!(
                "Against {}: {}",
                search_cmd::query_words(&against.query.text),
                search_cmd::outcome_words(against.outcome)
            ))
        ));
        for node in &against.nodes {
            let mut outcome = esc(&search_cmd::outcome_words(node.outcome));
            if !node.shows.is_empty() {
                outcome.push_str(&format!("<br>{}", esc(&search_cmd::node_shows_words(node))));
            }
            if node.left_out > 0 {
                outcome.push_str(&format!(
                    "<br>{}",
                    esc(&search_cmd::node_left_out_words(node.left_out))
                ));
            }
            out.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{outcome}</td></tr>\n",
                esc(&node.path),
                esc(&node.node)
            ));
        }
        out.push_str("</table>\n");
    }
    out.push_str(&basis_foot(&search_cmd::basis_text(&shown.basis)));
    out
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
