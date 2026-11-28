//! The pages (C16, a renderer, not an app; C53, text is a function of the
//! JSON): each renders the documents a verb produced, as HTML — one, but
//! for an answer opened from a saved search, which renders the answer and
//! the saved row it was opened from, and a conflict's refusal, which
//! renders the row as it stands — so the page states nothing the terminal
//! does not. The answer and item pages render their documents themselves:
//! each fact in the words the CLI's text says it in, made once in
//! `search_cmd.rs`, the layout the page's, and every way on the page's own
//! ([`answer`]), so they print no command line. An item is one card
//! ([`card`]) on both: a row that carries the item on the answer page,
//! `show`'s document on the item page. The search pages stand the answer
//! or the refusal under the page's editors ([`editor`]): the search form,
//! filled from the source the handler read (`fill.rs`), and the two boxes.
//! The place pages render the places
//! document ([`places`]), no `pre` and no command line. The plan, apply and
//! jobs pages render the plan, the apply's and the jobs documents
//! ([`plan`], [`applied`], [`jobs`]), in a player's words and with no
//! command line; a refusal's words, the CLI's, are the one `pre` a page
//! holds (#182). A page sorts, filters, groups and composes nothing; where
//! it wants what no document carries, the search crate or the verb grows
//! first. Every version a page renders is copied into its head's basis
//! line, never composed ([`Head::basis`]).

use acquisition_search::sockets::SOCKET_WORDS;
use std::collections::HashMap;

use acquisition_protocol::realm::Realm;
use acquisition_search::Classed;
use acquisition_search::answer::{
    Answer, Count, ExplainOut, Link, PLACE_ORDER_WORDS, QueryInput, Request, RowField, RowsOut,
    Shape, TradeOut, ViewOut, Zero,
};
use acquisition_search::counts::{CrossOut, Table};
use acquisition_search::places::{Counted, Filter as PlaceFilter, Holds, Kind, Place, Places, Row};
use acquisition_search::show::Shown;
use acquisition_search::trade::translate::{Fidelity, Status};
use acquisition_verbs::refusal::Way;
use acquisition_verbs::saved::{ReadRow, SavedSearches};

use super::fill::Fill;
use super::form::{Fate, Filter, Form, LIMITS};
use super::from::{FROM, FromPage};
use crate::search_cmd::{self, ItemParts, RowPiece};
use serde_json::Value;

use acquisition_search::words::plural;

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
    /// The stat pane's script: its rows and groups added and removed,
    /// and its finders (`assets/lists.js`).
    pub lists: bool,
    /// The count's script: the number beside each form that submits a
    /// search ([`COUNT`], [`SITE_COUNT`]).
    pub count: bool,
    /// The table's window script: the rows of a table asked a block at a
    /// time as they come into view (`assets/window.js`).
    pub window: bool,
    /// The site link's league select: the link a league's option holds
    /// swapped into the anchor (`assets/link.js`).
    pub link: bool,
    /// The versions of the documents the page renders (C16, every
    /// document carries its basis): each keyed by its document — `answer`,
    /// `item`, `plan`, `saved_searches`, `saved_search` — its value the
    /// document's own field as `serde_json::to_value` makes it, never a
    /// number the page composes. Written as one `<meta name="basis">` line
    /// where there is at least one, so part 3's feed has something to
    /// compare.
    pub basis: Vec<(&'static str, Value)>,
    /// The page's one way back: the `from` its URL carries, drawn above
    /// its body (the landing rule, `super::from`).
    pub back: Option<Back>,
    /// The search the page carries, which its nav bar carries on
    /// ([`nav`]).
    pub search: Option<Searched>,
}

/// The search a page carries (it survives a visit to another page, in
/// the address, never in browser storage): the search page itself, or the one its `from` names.
pub(crate) struct Searched {
    /// The search's page, under the token.
    pub href: String,
    /// The search as a `from` writes it.
    pub word: String,
    /// Whether the page is that search.
    pub own: bool,
}

/// The nav bar's links: Search, and each list the pages hold. On a page
/// that carries a search, each list's link carries it as its `from`, so
/// the page it opens carries it on; and the Search link leads back to it,
/// but on the search page itself, where it leads to the empty form.
fn nav(token: &str, search: Option<&Searched>) -> String {
    let home = match search {
        Some(search) if !search.own => search.href.clone(),
        _ => format!("/{token}/"),
    };
    let mut out = format!("<a href=\"{}\">Search</a>", esc(&home));
    for (route, words) in [
        ("saved", "Saved"),
        ("tabs", "Stash tabs"),
        ("characters", "Characters"),
        ("jobs", "Jobs"),
    ] {
        let href = match search {
            Some(search) => format!(
                "/{token}/{route}?{}={}",
                super::from::FROM,
                enc(&search.word)
            ),
            None => format!("/{token}/{route}"),
        };
        out.push_str(&format!(" <a href=\"{}\">{words}</a>", esc(&href)));
    }
    out
}

/// A way back: the page a request left, as a link in its place's words.
pub(crate) struct Back {
    pub href: String,
    pub words: &'static str,
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

/// The account a page serves, as its nav bar names it on every page (else
/// the account is learned from an error): its name, or none yet, where the
/// server waits for a login.
pub(crate) fn account_words(account: Option<&str>) -> String {
    match account {
        Some(name) => format!("account {name}"),
        None => "no account yet".to_string(),
    }
}

/// A page: the template filled, the body last so nothing in it is read
/// as a placeholder. `account` is the account the server serves, none
/// before one is found.
pub(crate) fn page(
    token: &str,
    title: &str,
    account: Option<&str>,
    head: Head,
    body: &str,
) -> String {
    let refresh = head
        .refresh
        .map(|s| format!("<meta http-equiv=\"refresh\" content=\"{s}\">\n"))
        .unwrap_or_default();
    let script: String = [
        (head.lists, "lists"),
        (head.count, "count"),
        (head.window, "window"),
        (head.link, "link"),
    ]
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
        .replace("{nav}", &nav(token, head.search.as_ref()))
        .replace("{account}", &esc(&account_words(account)))
        .replace("{basis}", &basis)
        .replace("{body}", &format!("{}{body}", back(head.back.as_ref())))
}

/// A page's way back, where it has one: one line above its body.
fn back(back: Option<&Back>) -> String {
    match back {
        Some(back) => format!(
            "<p class=\"back\"><a href=\"{}\">{}</a></p>\n",
            esc(&back.href),
            esc(&format!("Back to {}", back.words))
        ),
        None => String::new(),
    }
}

/// A refusal's words as the CLI prints them (`failure_words`), as a
/// block: each of its lines a line of the page source too (a newline just
/// inside `<pre>` is no part of its text). No page's own text is one
/// (#182).
fn pre(text: &str) -> String {
    format!("<pre>\n{}\n</pre>\n", esc(text.trim_end_matches('\n')))
}

/// A refusal, in the words the CLI would print after `Error:`.
pub(crate) fn refusal(words: &str) -> String {
    format!("<div class=\"refusal\">{}</div>\n", pre(words))
}

/// A refusal's document as a page renders it: its words and each
/// reading on a line of its own, as a terminal prints them; where it
/// carries a saved search as it stands (a conflict's `current`), that row
/// as the list renders it, so the page shows what the offered write would
/// replace; then each way on, saying what it is — a read as a link to the
/// page that takes it, a write (`{post, fields}`) as a form of hidden
/// fields with one button, posting the page's `from` (the landing rule) —
/// never a command line.
pub(crate) fn refused(doc: &Value, from: Option<&FromPage>) -> String {
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
            // a write offered with a name shows that name in its box, as
            // the save form does, so what the reader typed stands to be
            // read and changed before it is sent again (the landing rule)
            Some(post) if o["request"]["fields"].get("name").is_some() => {
                let fields = o["request"]["fields"].as_object();
                format!(
                    "<li>{}</li>\n",
                    save_form(
                        &said(post),
                        from,
                        fields
                            .into_iter()
                            .flatten()
                            .filter(|(k, _)| *k != "name")
                            .map(|(k, v)| (k.as_str(), said(v))),
                        &said(&o["request"]["fields"]["name"]),
                        &said(&o["says"])
                    )
                )
            }
            Some(post) => format!(
                "<li>{}</li>\n",
                post_form(
                    &said(post),
                    from,
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
/// reader fills) before its one button: the one maker of a form that
/// leaves its page, so each takes the page's `from` and posts it first
/// (the landing rule; no `method="post"` stands in the web module but
/// here).
pub(crate) fn post_form<'a>(
    action: &str,
    from: Option<&FromPage>,
    fields: impl IntoIterator<Item = (&'a str, String)>,
    shown: &str,
    button: &str,
) -> String {
    let hidden: String = from
        .map(|from| (FROM, from.word()))
        .into_iter()
        .chain(fields)
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
        esc(&crate::saved_cmd::query_said(row)),
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

/// The saved searches: the list's head line, then a row per search in the
/// document's order, each with a link that opens it, its rename form naming
/// the revision listed and its Clear, a link to the page that asks first
/// ([`saved_ask`]); a row this build cannot read, its Clear alone; the rows
/// left out counted, with the link that lists every one.
pub(crate) fn saved_list(
    listed: &SavedSearches,
    links: &SavedLinks,
    open: &dyn Fn(&str) -> String,
    from: Option<&FromPage>,
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
                from,
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
            esc(&super::from::onward(
                &format!("{}?id={id}", links.clear),
                from
            ))
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

/// A clear asked first (the owner: "Clear should ask first"): the row as
/// the list shows it, unlinked, and the clear's form at the revision shown,
/// beside a link back to the list. The page writes nothing; its form's POST
/// is the clear. A row this build cannot read is asked about the same way,
/// since a clear is the one thing a page can do with it.
pub(crate) fn saved_ask(row: &Value, links: &SavedLinks, from: Option<&FromPage>) -> String {
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
            from,
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

/// The saved search an answer was opened from: the id and revision its URL
/// names, and the row as it stands now at that id — readable, gone, or one
/// this build cannot read.
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
        && (row.text.as_deref() != Some(a.query.text.as_str())
            || row.scope.realm != a.scope.realm.as_str()
            || row.scope.membership != a.scope.membership.as_str())
    {
        out.push_str(&format!(
            "<p class=\"saved\">{}</p>\n",
            esc(&format!(
                "This answer's query is not the saved one's: the saved query is {} over {} · {}.",
                row.text.as_deref().unwrap_or("one this build cannot read"),
                row.scope.realm,
                row.scope.membership
            ))
        ));
    }
    out
}

/// What an answer page saves: the source of the request it answered
/// (C194), never the answer's report of it, and the saved search it was
/// opened from, where one was.
#[derive(Clone, Copy)]
pub(crate) struct Saving<'a> {
    pub source: &'a QueryInput,
    pub opened: Option<&'a Opened>,
    /// The view the page's editors carry, whose parts a save leaves
    /// behind.
    pub view: &'a BoxView,
    /// The page the save forms post as `from`: the search page itself.
    pub from: Option<&'a FromPage>,
}

/// A save form, the one maker of each: `hidden` its source, scope and the
/// row it writes over, if any; a name typed, `name` filled in; posting
/// `from`. A refused write renders it again as sent (the landing rule).
pub(crate) fn save_form<'a>(
    action: &str,
    from: Option<&FromPage>,
    hidden: impl IntoIterator<Item = (&'a str, String)>,
    name: &str,
    button: &str,
) -> String {
    post_form(
        action,
        from,
        hidden,
        &format!(
            "<label class=\"filter-title\">Name <input name=\"name\" value=\"{}\" aria-label=\"the search's name\"></label> ",
            esc(name)
        ),
        button,
    )
}

/// The save form under an answer's query box: the source of the request
/// the page answered (C194, the row keeps its source) — the text the
/// answer ran, or the site search as one word, the form's envelope as
/// JSON text — its realm and membership hidden, and a name typed; over
/// the saved search it was opened from where that row stands readable,
/// at the revision the URL names, beside a second form that saves it as
/// new.
fn save_forms(
    a: &Answer,
    source: &QueryInput,
    opened: Option<&Opened>,
    links: &SavedLinks,
    boxed: &BoxView,
    from: Option<&FromPage>,
) -> String {
    let asked = match source.trade_word() {
        Some(trade) => ("trade", trade),
        None => ("text", a.query.text.clone()),
    };
    let query = || {
        vec![
            asked.clone(),
            ("realm", a.scope.realm.as_str().to_string()),
            ("membership", a.scope.membership.as_str().to_string()),
        ]
    };
    let mut out = String::from("<div class=\"save\">\n");
    // a saved site search keeps the search as given, and its text is
    // translated again at each opening (C194)
    if asked.0 == "trade" {
        out.push_str(&format!(
            "<p class=\"save-note\">{}</p>\n",
            esc(
                "Saved, this keeps the site's search as given, translated again each time it \
                 opens."
            )
        ));
    }
    // what the query box carries of the view and a save leaves behind
    // a saved search is its query and its scope (C167)
    let left = boxed.left_by_a_save();
    if !left.is_empty() {
        let listed = match left.as_slice() {
            [one] => one.to_string(),
            [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
            [] => String::new(),
        };
        out.push_str(&format!(
            "<p class=\"save-note\">{}</p>\n",
            esc(&format!(
                "Saved without {listed}: a saved search is its query and its scope."
            ))
        ));
    }
    match opened {
        Some(Opened {
            id,
            at,
            row: OpenedRow::Read(row),
        }) => {
            let mut fields = query();
            fields.push(("id", id.clone()));
            fields.push(("if_revision", at.to_string()));
            out.push_str(&save_form(
                &links.write,
                from,
                fields,
                &row.name,
                &format!("Save over \"{}\"", row.name),
            ));
            out.push('\n');
            out.push_str(&save_form(
                &links.write,
                from,
                query(),
                "",
                "Save as a new search",
            ));
        }
        _ => out.push_str(&save_form(
            &links.write,
            from,
            query(),
            "",
            "Save this search",
        )),
    }
    out.push_str("\n</div>\n");
    out
}

/// ` data-unrun`, the mark the switch hides by (`4-full-1-form-rides.md`,
/// Decision 4), where a fate's status does not run ([`Fate::runs`]):
/// neither exact nor ambiguous.
fn unrun(fate: Option<&Fate>) -> &'static str {
    match fate.is_some_and(|f| !f.runs()) {
        true => " data-unrun",
        false => "",
    }
}

/// The switch at the page's foot (`4-full-1-form-rides.md`, Decision 3):
/// a box tied to the search form by `form=`, ticked where the URL names it,
/// that shows the fields this search does not run, which the stylesheet
/// hides while it is unticked unless one holds a value; and the line that
/// says what a mark means. It is the page's, marked `data-page`: `home`,
/// `search` and `count` take it out of the fields first, so nothing a
/// search reads, links or carries names it, and the count leaves it out.
pub(crate) fn switch(on: bool) -> String {
    format!(
        "<p class=\"switch\"><label><input type=\"checkbox\" id=\"all-fields\" name=\"{ALL_FIELDS}\" value=\"true\" form=\"{SEARCH_FORM}\" data-page{}> Show the fields this search does not run</label> <span class=\"fate\">A field with no mark runs as the trade site's; a mark says it is read differently here, or not run.</span></p>\n",
        checked(on)
    )
}

/// The switch's field ([`switch`]).
pub(crate) const ALL_FIELDS: &str = "all_fields";

/// A control's fate beside it, where it is not exact
/// (`4-full-1-form-rides.md`, Decision 2): its status as a mark, its
/// `says` as the title; a control that runs as the site's says nothing.
fn fate_span(fate: Option<&Fate>) -> String {
    match fate.filter(|f| !f.exact()) {
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

/// What fills a page's editors (`4-full-1-form-design.md` §2): the form's
/// envelope, the scope the form carries, the query box's text and whether
/// it stands open, the editor that holds the search the page shows, the
/// site box's text, the view both carry, a count's keys and the saved
/// search the page was opened from.
pub(crate) struct Filled {
    pub fill: Fill,
    pub realm: String,
    /// The scope's membership as its word; none is the form's default, live.
    pub membership: Option<String>,
    pub text: String,
    pub text_open: bool,
    /// Where "Add a column" stands (#319): the editor whose search the
    /// page shows.
    pub holder: Holder,
    pub trade: String,
    pub view: BoxView,
    pub saved: Option<(String, i64)>,
    /// Whether the page names its league: the address carries a `league`
    /// field, empty ("every league") or not, or a search was read, which
    /// names its league or none. A form that does not is fresh, and its
    /// league control selects the fullest league ([`Chosen::Fresh`]).
    pub league_named: bool,
}

impl Filled {
    /// The editors of `realm` with nothing in them: the form empty, the
    /// query box open and empty, the site box empty.
    pub(crate) fn empty(realm: &str) -> Filled {
        Filled {
            fill: Fill::empty(),
            realm: realm.to_string(),
            membership: None,
            text: String::new(),
            text_open: true,
            holder: Holder::Form,
            trade: String::new(),
            view: BoxView::default(),
            saved: None,
            league_named: false,
        }
    }
}

/// The editor that holds the search a page shows (#319): the query box
/// where the search is a text in the language, the search form where it is
/// the site's (a form search, a site search, a saved one of either kind)
/// and on a page with no search.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Holder {
    Form,
    QueryBox,
}

/// The page's editors, filled (the one page, `4-full-1-form-design.md` §2):
/// what the form's envelope holds and no control sends, named; the search
/// form, the site box in its bar; the query box, `index` its link to the
/// language's index; the league list's fetch. The answer or
/// the refusal stands under them, at [`ANSWER`]. "Add a column" stands in
/// one of them, the one that holds the search the page shows
/// ([`Holder`]): a pick sends its editor, so it keeps that search.
pub(crate) fn editor(
    form: &Form,
    filled: &Filled,
    leagues: Option<&[League]>,
    from: Option<&FromPage>,
    index: &str,
) -> String {
    let finder = value_finder();
    let (in_form, in_box) = match filled.holder {
        Holder::Form => (finder.as_str(), ""),
        Holder::QueryBox => ("", finder.as_str()),
    };
    // the form first, so its controls have taken what they hold
    let searched = search_form(form, filled, leagues, in_form);
    let mut out = unsent(&filled.fill.left());
    out.push_str(&searched);
    out.push_str(&query_box(filled, index, in_box));
    out.push_str(&fetch_leagues(from));
    out
}

/// The anchor the answer or the refusal stands at under the editors, which
/// every form that submits to `search` names in its action.
pub(crate) const ANSWER: &str = "answer";

/// What the envelope holds and no control of the form sends, each leaf by
/// the site's path with its value: a search from the form leaves it out.
fn unsent(left: &[(String, Value)]) -> String {
    if left.is_empty() {
        return String::new();
    }
    let items: String = left
        .iter()
        .map(|(path, value)| {
            format!(
                "<li><code data-path=\"{}\">{}</code> {}</li>",
                esc(path),
                esc(path),
                esc(&value.to_string())
            )
        })
        .collect();
    format!(
        "<div class=\"left-out\"><p>{}</p>\n<ul>{items}</ul></div>\n",
        esc(
            "This search holds what the form below does not send: a search from the form \
             leaves each of these out."
        )
    )
}

/// ` value="…"`, where there is one.
fn valued(value: Option<&str>) -> String {
    value
        .map(|v| format!(" value=\"{}\"", esc(v)))
        .unwrap_or_default()
}

/// ` checked`, where a box is ticked.
fn checked(ticked: bool) -> &'static str {
    if ticked { " checked" } else { "" }
}

/// The search form, in the trade site's organization (the owner's page save
/// read for its order alone, nothing of GGG's copied): the search bar
/// across the top — the item search on the left, then the site box, its
/// own form's ([`trade_box`]); the realm, the league and the status on the
/// right — then two panes, the filter groups as panels in
/// the dump's order on the left and the stat groups on the
/// right ([`stat_pane`]), then the controls with Search at the centre. A panel is a
/// `details` group, open where the site shows the group open or where it
/// holds a value. Each control reads its value at its path from the fill
/// ([`Fill`]); a listing control shows it and sends nothing.
///
/// The form is `novalidate`: what it sends is the fold's to judge, as
/// the count beside its button already reads it, so a number box holding
/// what no number reads, shown or hidden by its group's type, never holds
/// a search back (a browser sends such a box empty).
///
/// The form is one realm's render: its league list is that realm's, so
/// the realm is chosen by a form of its own that renders the page again,
/// and the search carries the realm shown. After its buttons stands
/// `finder`, "Add a column" where the form holds the page's search
/// ([`editor`]), else nothing.
pub(crate) fn search_form(
    form: &Form,
    filled: &Filled,
    leagues: Option<&[League]>,
    finder: &str,
) -> String {
    let fill = &filled.fill;
    let realm = filled.realm.as_str();
    let mut out = format!(
        "<form id=\"realm-form\" method=\"get\" action=\".\"></form>\n<form id=\"{TRADE_FORM}\" method=\"get\" action=\"search#{ANSWER}\" data-count></form>\n<form id=\"{SEARCH_FORM}\" method=\"get\" action=\"search#{ANSWER}\" novalidate data-count>\n<div class=\"search-bar\">\n<div class=\"search-left\">"
    );
    out.push_str(&format!(
        "<input class=\"item-search\" name=\"query.term\"{} placeholder=\"Search items\" aria-label=\"Search items\">{}\n{}</div>\n<div class=\"search-right\">",
        valued(fill.text("query.term").as_deref()),
        fate_span(form.fate("query.term")),
        trade_box(realm, &filled.trade)
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
    // the league the search names; else "every league" where the address
    // named it or a search was read, which names its league or none; else
    // a fresh form's fullest
    let named = fill.text("league");
    let chosen = match (&named, filled.league_named) {
        (Some(league), _) => Chosen::Named(league),
        (None, true) => Chosen::Every,
        (None, false) => Chosen::Fresh,
    };
    out.push_str(&league_select(leagues, chosen, true));
    out.push_str(&fate_span(form.fate("url.league")));
    for group in form.groups.iter().filter(|g| g.id == "status_filters") {
        for filter in &group.filters {
            out.push_str(&control(form, fill, filter, "query.status", "query.status"));
        }
    }
    // the scope's membership: the items standing, or those removed too,
    // or those removed alone
    let membership = filled.membership.as_deref().unwrap_or("live");
    out.push_str("<label class=\"control\">Membership <select name=\"membership\">");
    for m in ["live", "all", "removed"] {
        out.push_str(&format!(
            "<option value=\"{m}\"{}>{m}</option>",
            if m == membership { " selected" } else { "" }
        ));
    }
    out.push_str("</select></label>\n");
    out.push_str("</div>\n</div>\n<div class=\"panes\">\n<div class=\"pane\">\n");
    for group in form.groups.iter().filter(|g| g.id != "status_filters") {
        let title = group.title.as_deref().unwrap_or(&group.id);
        let at = format!("query.filters.{}", group.id);
        // the group's enable control, as the site's: off, the group is
        // sent with its `disabled` field and its values stand; the panel's
        // own toggle is open and close alone
        out.push_str(&format!(
            "<details class=\"panel\"{}{}><summary>{}</summary>\n<div class=\"panel-body\">\n<div class=\"filter\">{}</div>\n",
            // the armour panel opens where its quality box is unticked
            if !group.hidden
                || fill.holds_under(&at)
                || (group.id == "armour_filters" && fill.holds_under("defences"))
            {
                " open"
            } else {
                ""
            },
            // a panel every control of which is not run is marked as they are
            match group.filters.iter().all(|f| {
                form.fate(&format!("{}.{}", group.id, f.id))
                    .is_some_and(|fate| !fate.runs())
            }) && !group.filters.is_empty()
            {
                true => " data-unrun",
                false => "",
            },
            esc(title),
            on_box(
                &format!("{at}.disabled"),
                !fill.ticked(&format!("{at}.disabled"), "true"),
                "group on"
            ),
        ));
        for filter in &group.filters {
            let path = super::form::filter_path(&group.id, &filter.id);
            let key = format!("{}.{}", group.id, filter.id);
            out.push_str(&control(form, fill, filter, &path, &key));
        }
        // C142: the defences' reading beside the query, the panel's one
        // convenience the site's form has no control for
        if group.id == "armour_filters" {
            out.push_str(&quality_box(fill));
        }
        out.push_str("</div>\n</details>\n");
    }
    out.push_str("</div>\n<div class=\"pane\">\n<details class=\"panel\" open><summary>Stat Filters</summary>\n");
    out.push_str(&stat_pane(form, fill));
    out.push_str("</details>\n</div>\n</div>\n");
    // the view, as the query box carries it: the rows' number, the way they
    // show, a count's keys; and hidden, the order and the values the URL
    // named, which no control writes
    out.push_str("<div class=\"controls\">\n<div class=\"controls-left\">");
    out.push_str(&rows_shown(filled.view.limit));
    out.push_str(&format!(
        " {} <label class=\"control\" title=\"Keys as the CLI&#39;s count takes them: rarity, league, tab, class, category, line:resist,life\">Or count by <input name=\"view.count\"{} placeholder=\"rarity\"></label></div>\n<div class=\"controls-center\"><button type=\"submit\" class=\"search-btn\">Search</button> ",
        results_as(filled.view.cards),
        valued(filled.view.count.as_deref()),
    ));
    out.push_str(SITE_COUNT);
    out.push(' ');
    out.push_str(&clear(realm, filled.membership.as_deref()));
    // "Add a column", after the form's button, where the form holds the
    // page's search (the value finder, [`editor`])
    out.push_str(&format!(
        "</div>\n<div class=\"controls-right\">{finder}</div>\n</div>\n"
    ));
    out.push_str(&carried(
        &filled.view,
        filled.saved.as_ref().map(|(id, at)| (id.as_str(), *at)),
    ));
    out.push_str("</form>\n");
    out
}

/// Clear (`4-full-1-form-rides.md`, Decision 6): a link beside Search to
/// the form's page with the scope alone — the realm, and the membership
/// where it is not `live`, the form's default — so it lands on the empty
/// form of the scope shown. No other field, the switch among them, rides
/// on it.
fn clear(realm: &str, membership: Option<&str>) -> String {
    let mut to = format!(".?realm={}", enc(realm));
    if let Some(membership) = membership.filter(|m| *m != "live") {
        to.push_str(&format!("&membership={}", enc(membership)));
    }
    format!("<a class=\"clear\" href=\"{}\">Clear</a>", esc(&to))
}

/// What the search form carries hidden: the view the URL named
/// ([`view_hidden`]), its count being the form's own field, and the saved
/// search the page was opened from at the revision its URL names.
fn carried(view: &BoxView, saved: Option<(&str, i64)>) -> String {
    let mut out = view_hidden(view, false);
    if let Some((id, at)) = saved {
        out.push_str(&hidden("saved", id));
        out.push_str(&hidden("saved_revision", &at.to_string()));
    }
    if !out.is_empty() {
        out.push('\n');
    }
    out
}

/// A hidden field.
fn hidden(name: &str, value: &str) -> String {
    format!(
        "<input type=\"hidden\" name=\"{name}\" value=\"{}\">",
        esc(value)
    )
}

/// What an editor carries hidden of the view the URL named, the one
/// renderer both editors share: the order, its direction and each value
/// named, never the query's own, and with `count` a count's keys, so an
/// edit run again keeps them. A search sent with a count lets go of the
/// order, its direction and the values, which a count shows none of
/// (`form.rs`, the fold), so a count is never refused for what was
/// carried beside it; a carried order the search refuses on its own is
/// refused again, in its own words.
fn view_hidden(view: &BoxView, count: bool) -> String {
    let mut out = String::new();
    if let Some(sort) = &view.sort {
        out.push_str(&hidden("view.sort", sort));
        if view.desc {
            out.push_str(&hidden("view.desc", "true"));
        }
    }
    for value in &view.values {
        out.push_str(&hidden("view.value", value));
    }
    if count && let Some(keys) = &view.count {
        out.push_str(&hidden("view.count", keys));
    }
    out
}

/// The quality box (C142, the defences' reading beside the query;
/// `4-full-1-form-rides.md`, Decision 1), shaped as an on box: a hidden
/// `defences=displayed`, then the box, ticked by default, sending
/// `defences=q20`, the site's own reading, which the fold's normal form
/// drops. The fold reads the last sent, so the box unticked asks the
/// defences as the item stands. It is unticked where the fill holds
/// `displayed`.
fn quality_box(fill: &Fill) -> String {
    format!(
        "<div class=\"filter\"><input type=\"hidden\" name=\"defences\" value=\"displayed\"><label><input type=\"checkbox\" name=\"defences\" value=\"{}\"{}> assume 20% quality</label></div>\n",
        super::form::SITE_DEFENCES,
        checked(!fill.ticked("defences", "displayed"))
    )
}

/// An on box (`4-full-1-form-rows.md`, Decision 8), the one maker of
/// every `disabled` field of the form: a hidden `disabled=true`, then the
/// box, ticked for on, sending `disabled=false`. The fold reads the last
/// sent, so a box ticked is on and one left unticked sends the hidden
/// `true` alone.
fn on_box(name: &str, on: bool, words: &str) -> String {
    format!(
        "<input type=\"hidden\" name=\"{name}\" value=\"true\"><label><input type=\"checkbox\" name=\"{name}\" value=\"false\"{}> {words}</label>",
        checked(on)
    )
}

/// A stat's fate as a row and a finder's entry say it
/// (`4-full-1-form-rides.md`, Decision 2): its group word, which tells two
/// stats of one text apart, then its two statuses where either is not
/// exact.
fn stat_fate(e: &acquisition_search::trade::vocabulary::Entry) -> String {
    match (e.alone, e.bounded) {
        (Status::Exact, Status::Exact) => e.group.to_string(),
        (alone, bounded) => format!(
            "{} \u{b7} alone {} \u{b7} with a bound {}",
            e.group,
            status_words(alone),
            status_words(bounded)
        ),
    }
}

/// A status as a stat's fate says it.
fn status_words(status: Status) -> String {
    serde_json::to_value(status)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.replace('_', " ")))
        .unwrap_or_default()
}

/// What a stat row's boxes hold, each as its box renders it: none, an
/// empty box. `option` is the option box where the row holds one, with
/// the value it takes, if any.
#[derive(Default)]
pub(crate) struct RowValues {
    pub off: bool,
    pub min: Option<String>,
    pub max: Option<String>,
    pub weight: Option<String>,
    pub option: Option<Option<String>>,
}

/// The one maker of a stat row (`4-full-1-form-rows.md`, Decision 1), the
/// filled form's and the finder's listing's: the stat's id a hidden
/// `stat.id`, which begins the row (`form.rs`, the stat fields), then its
/// on box, its texts (the id itself where no table holds it), its min,
/// max and weight, an option box where the row holds one, its fate as the
/// vocabulary gives it ([`stat_fate`]) and its remove control. Every row renders a weight
/// box: what its group's type shows is the stylesheet's ([`type_takes`]).
/// The row is an item of its group's list (`assets/lists.js`), and its
/// min takes the focus when a pick adds it.
pub(crate) fn stat_row(
    id: &str,
    entry: Option<&acquisition_search::trade::vocabulary::Entry>,
    values: &RowValues,
) -> String {
    let (texts, fate) = match entry {
        Some(e) => (e.texts.join(" / "), stat_fate(e)),
        None => (
            id.to_string(),
            "no stat this build's tables hold".to_string(),
        ),
    };
    let option = match &values.option {
        Some(value) => format!(
            " <input name=\"stat.option\"{} size=\"10\" placeholder=\"option\" aria-label=\"option\">",
            valued(value.as_deref())
        ),
        None => String::new(),
    };
    format!(
        "<div class=\"filter stat-row\" data-item><input type=\"hidden\" name=\"stat.id\" value=\"{}\">{} <span class=\"stat-text\">{}</span> <input type=\"number\" step=\"any\" name=\"stat.min\"{} placeholder=\"min\" aria-label=\"min\" data-focus> <input type=\"number\" step=\"any\" name=\"stat.max\"{} placeholder=\"max\" aria-label=\"max\"> <input type=\"number\" step=\"any\" name=\"stat.weight\"{} placeholder=\"weight\" aria-label=\"weight\">{option} <span class=\"fate\">{}</span> <button type=\"button\" data-remove>Remove</button></div>\n",
        esc(id),
        on_box("stat.disabled", !values.off, "on"),
        esc(&texts),
        valued(values.min.as_deref()),
        valued(values.max.as_deref()),
        valued(values.weight.as_deref()),
        esc(&fate),
    )
}

/// The positional fields a group of `kind` takes, as its type's option
/// lists them for the stylesheet (`4-full-1-form-rows.md`, Decision 5):
/// its bound's, by the translation's `group_value_keys`, and its rows',
/// by `stat_value_keys`, never restated here.
pub(crate) fn type_takes(kind: &str) -> Vec<String> {
    use acquisition_search::trade::translate::{group_value_keys, stat_value_keys};
    group_value_keys(kind)
        .iter()
        .map(|k| format!("group.{k}"))
        .chain(stat_value_keys(kind).iter().map(|k| format!("stat.{k}")))
        .collect()
}

/// A stat group: its type, each option listing the fields it takes
/// ([`type_takes`]), an empty `no type` first where `kind` is none of
/// them; its on box; its count's bound, rendered for every type; its
/// remove control; then its rows, an item list, and its finder, a box no
/// form sends and its list. The group is an item of the pane's list and a
/// list of its own, whose add controls are its finder's options.
fn stat_group(
    form: &Form,
    kind: &str,
    off: bool,
    bound: (Option<String>, Option<String>),
    rows: &str,
) -> String {
    let mut out = String::from(
        "<div class=\"stat-group\" data-item data-list>\n<div class=\"filter\"><span class=\"filter-title\">Group</span> <select name=\"group.type\">",
    );
    if !form.stat_types.iter().any(|(k, _, _)| k == kind) {
        out.push_str(&format!(
            "<option value=\"\" data-takes=\"{}\" selected>no type</option>",
            type_takes("").join(" ")
        ));
    }
    for (k, text, fate) in &form.stat_types {
        out.push_str(&format!(
            "<option value=\"{}\" data-takes=\"{}\"{}{}>{}{}</option>",
            esc(k),
            type_takes(k).join(" "),
            if k == kind { " selected" } else { "" },
            unrun(Some(fate)),
            esc(text.as_deref().unwrap_or(k)),
            esc(&fate.marked())
        ));
    }
    out.push_str(&format!(
        "</select> {} <input type=\"number\" step=\"any\" name=\"group.min\"{} placeholder=\"min\" aria-label=\"group min\"> <input type=\"number\" step=\"any\" name=\"group.max\"{} placeholder=\"max\" aria-label=\"group max\"> <button type=\"button\" data-remove>Remove group</button></div>\n<div class=\"stat-rows\" data-items>\n{rows}</div>\n<div class=\"filter\" data-finder=\"{STAT_FINDER}\"><input type=\"search\" placeholder=\"+ Add Stat Filter\" aria-label=\"Add Stat Filter\" autocomplete=\"off\" data-focus><ul class=\"finder-list\" role=\"listbox\"></ul></div>\n</div>\n",
        on_box("group.disabled", !off, "group on"),
        valued(bound.0.as_deref()),
        valued(bound.1.as_deref()),
    ));
    out
}

/// The template id of a group of `kind`, which an "Add Stat Group" control
/// names.
fn group_template(kind: &str) -> String {
    format!("stat-group-{}", esc(kind))
}

/// The stat pane: the stat groups from the envelope's `query.stats` where
/// the fill renders it ([`Fill::stats`]), else one `and` group with no
/// row; each group's type, its on box and its count's bound, positional
/// (`form.rs`, the stat fields), then its rows ([`stat_row`]), each with
/// its stat's text and fate as the vocabulary gives them, a box given a
/// value only where the group's type takes it. After the last group, "Add
/// Stat Group": a menu of add controls, one a type, each naming the
/// `<template>` of an empty group of its type, which the pane carries and
/// no form sends. The script copies and removes what is rendered here
/// (`assets/lists.js`) and writes nothing of its own.
fn stat_pane(form: &Form, fill: &Fill) -> String {
    use acquisition_search::trade::translate::{group_value_keys, stat_value_keys};
    let groups = fill.stats();
    let ids: Vec<&str> = groups
        .iter()
        .flat_map(|g| g["filters"].as_array().into_iter().flatten())
        .filter_map(|row| row["id"].as_str())
        .collect();
    let entries: HashMap<String, acquisition_search::trade::vocabulary::Entry> =
        acquisition_search::trade::vocabulary::entries(&ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .map(|e| (e.id.clone(), e))
            .collect();
    let mut out = String::from(
        "<div class=\"panel-body\" data-list>\n<div class=\"stat-groups\" data-items>\n",
    );
    if groups.is_empty() {
        out.push_str(&stat_group(form, "and", false, (None, None), ""));
    }
    for (g, group) in groups.iter().enumerate() {
        let at = format!("query.stats[{g}]");
        let chosen = fill.option(&format!("{at}.type"), |kind| {
            form.stat_types.iter().any(|(k, _, _)| k == kind)
        });
        // a group whose type no option names sends none
        let kind = chosen.as_deref().unwrap_or("");
        // a group's bound where its type takes one, as the fold keeps it
        let bound = |key: &str| {
            group_value_keys(kind)
                .contains(&key)
                .then(|| fill.number(&format!("{at}.value.{key}")))
                .flatten()
        };
        let takes = stat_value_keys(kind);
        let mut rows = String::new();
        for r in 0..group["filters"].as_array().map_or(0, Vec::len) {
            let row = format!("{at}.filters[{r}]");
            let id = fill.text(&format!("{row}.id")).unwrap_or_default();
            let value = |key: &str| {
                takes
                    .contains(&key)
                    .then(|| fill.number(&format!("{row}.value.{key}")))
                    .flatten()
            };
            let option = (takes.contains(&"option")
                && fill.holds_under(&format!("{row}.value.option")))
            .then(|| fill.text(&format!("{row}.value.option")));
            rows.push_str(&stat_row(
                &id,
                entries.get(&id),
                &RowValues {
                    off: fill.ticked(&format!("{row}.disabled"), "true"),
                    min: value("min"),
                    max: value("max"),
                    weight: value("weight"),
                    option,
                },
            ));
        }
        out.push_str(&stat_group(
            form,
            kind,
            fill.ticked(&format!("{at}.disabled"), "true"),
            (bound("min"), bound("max")),
            &rows,
        ));
    }
    out.push_str("</div>\n<details class=\"add-group\"><summary>Add Stat Group</summary><div class=\"filter\">");
    for (k, text, fate) in &form.stat_types {
        out.push_str(&format!(
            "<button type=\"button\" data-add=\"{}\"{}>{}{}</button>",
            group_template(k),
            unrun(Some(fate)),
            esc(text.as_deref().unwrap_or(k)),
            esc(&fate.marked()),
        ));
    }
    out.push_str("</div></details>\n");
    for (k, _, _) in &form.stat_types {
        out.push_str(&format!(
            "<template id=\"{}\">{}</template>\n",
            group_template(k),
            stat_group(form, k, false, (None, None), "")
        ));
    }
    out.push_str("</div>\n");
    out
}

/// The finder's listing (`stats` asked for HTML): each entry an option
/// whose add control is the option itself, holding the `<template>` of
/// its row as the form renders a row of that id alone ([`stat_row`]);
/// then the entries left out, or that none holds the words, in the
/// server's words.
pub(crate) fn stat_options(listed: &acquisition_search::trade::vocabulary::Vocabulary) -> String {
    let mut out = String::new();
    for e in &listed.entries {
        out.push_str(&format!(
            "<li role=\"option\" data-add>{} <span class=\"fate\">{}</span><template>{}</template></li>\n",
            esc(&e.texts.join(" / ")),
            esc(&stat_fate(e)),
            stat_row(&e.id, Some(e), &RowValues::default()),
        ));
    }
    if listed.left_out > 0 {
        out.push_str(&format!(
            "<li class=\"fate\">{} more match: add a word</li>\n",
            listed.left_out
        ));
    } else if listed.entries.is_empty() {
        out.push_str("<li class=\"fate\">no stat holds every word asked</li>\n");
    }
    out
}

/// The route a stat finder asks, its words appended by the script: the
/// stat vocabulary's options ([`stat_options`]).
const STAT_FINDER: &str = "stats?q=";

/// The route the value finder asks, its words appended by the script: the
/// value listing's options ([`value_options`]), the same route asked for
/// values.
const VALUE_FINDER: &str = "stats?values=true&q=";

/// The field a value finder's option sends: the value a pick orders the
/// page by, which the page takes out of its fields first (`view.by`).
pub(crate) const VIEW_BY: &str = "view.by";

/// The value finder, "Add a column" (`4-full-1-form-values.md`): a box no
/// form sends and a listbox, standing inside a form that carries the
/// page's view, after that form's own submit button, so Enter in another
/// box still sends that button. Its options are the form's submit buttons
/// ([`value_options`]): a pick sends the form as it stands with one field
/// more.
pub(crate) fn value_finder() -> String {
    format!(
        "<div class=\"filter value-finder\" data-finder=\"{}\"><input type=\"search\" placeholder=\"Add a column\" aria-label=\"Add a column\" autocomplete=\"off\"><ul class=\"finder-list\" role=\"listbox\"></ul></div>",
        esc(VALUE_FINDER)
    )
}

/// The value finder's options: each value the listing offers as a submit
/// button of the form the finder stands in, sending [`VIEW_BY`] with the
/// value's text, named by the site's words and its group; then how many
/// more match, or that none does.
pub(crate) fn value_options(listed: &acquisition_search::trade::vocabulary::Values) -> String {
    let mut out = String::new();
    for e in &listed.entries {
        out.push_str(&format!(
            "<li><button type=\"submit\" role=\"option\" name=\"{VIEW_BY}\" value=\"{}\">{} <span class=\"fate\">{}</span></button></li>\n",
            esc(&e.value),
            esc(&e.texts.join(" / ")),
            esc(e.group),
        ));
    }
    if listed.left_out > 0 {
        out.push_str(&format!(
            "<li class=\"fate\">{} more match: add a word</li>\n",
            listed.left_out
        ));
    } else if listed.entries.is_empty() {
        out.push_str("<li class=\"fate\">no value holds every word asked</li>\n");
    }
    out
}

/// The count beside each search's button, on every form that submits to
/// `search`, which carries `data-count`: the words are these, the server's;
/// `count.js` writes the numbers alone, a dash until a change is answered,
/// and shows the undecided part only where it is not zero. It finds a
/// form's outputs among the form's elements, so the site box's stand in
/// the search form's bar, owned by its own form ([`count_of`]). The query
/// box's is this; a form whose search is the site's is [`SITE_COUNT`].
const COUNT: &str = count_words!("");

/// The count beside a form whose search is the site's (the search form and
/// the site box): [`COUNT`]'s and a third part, the clauses of the search
/// as it stands that would not run here (the answer's
/// `query.trade.remainder`), which `count.js` writes as it writes the
/// undecided and shows only where it is not zero, so a number counted
/// without them never stands as the site's search's. A query in the
/// language has no site clauses, and the query box no such part.
const SITE_COUNT: &str = count_words!(
    "<span hidden> \u{b7} not the site's search: <output data-count=\"unrun\">\u{2013}</output> of its clauses not run</span>"
);

/// The count's words, one literal for [`COUNT`] and [`SITE_COUNT`], with
/// `$more` after the undecided part.
macro_rules! count_words {
    ($more:literal) => {
        concat!(
            "<span class=\"count\">Matches as it stands: <output data-count=\"matched\">\u{2013}</output><span hidden> \u{b7} <output data-count=\"undecided\">\u{2013}</output> undecided</span>",
            $more,
            "</span>"
        )
    };
}
use count_words;

/// [`SITE_COUNT`], its outputs owned by the form `id` names (`form=`).
fn count_of(id: &str) -> String {
    SITE_COUNT.replace("<output ", &format!("<output form=\"{id}\" "))
}

/// The search form's id, which the switch at the page's foot names
/// (`form=`).
pub(crate) const SEARCH_FORM: &str = "search-form";

/// The site box's form: empty, standing before the search form, its
/// controls in the search form's bar owned by it through `form=`, as the
/// realm's are by `realm-form`.
const TRADE_FORM: &str = "trade-form";

/// The query box: a query in the language, run as the user's own — the one
/// place every field the language has is reachable from a page, so what the
/// site's form cannot say is typed here. On an answer it holds the text the
/// answer ran, the translation's where the search was the site's form, to
/// edit; an edited text drops the trade input, as the request's own rule
/// has it. On an answer it carries the answer's membership, and the saved
/// search the page was opened from with the revision its URL names, so the
/// loop survives an edit. Its rows' number is the big form's "Show", set on
/// an answer to the rows view it showed; "Show" offers the search's own
/// default first, sent empty, so a box left as it is asks no limit. Nothing
/// is typed to sort: on an answer it carries, hidden as it carries the
/// realm, the sort, its direction, the values the URL named and a count's
/// keys ([`view_hidden`]), so an edited query keeps the order and the
/// columns its reader chose by a head or a card; the values the query gave
/// unasked are never carried, so the edited query gives its own. Its "Results as" is the big form's: the table,
/// sent empty, or the cards, set on an answer to the way it showed, so an
/// edited query keeps it. What it carries of the view is one [`BoxView`],
/// an answer's or, on a refused search, the URL's as sent (#190), and the
/// save note reads the same. It stands in a `details`, open where the
/// page's source is a text or the page has none, closed where the form
/// holds the search (the one page, `4-full-1-form-design.md` §2). Beside
/// its summary, inside the box, `index` links the language's index
/// (`4-full-1-form-index.md`, Decision 5), in words that say what it is.
/// After its button stands `finder`, "Add a column" where the box holds
/// the page's search ([`editor`]), else nothing.
fn query_box(filled: &Filled, index: &str, finder: &str) -> String {
    let realm = filled.realm.as_str();
    let membership = filled.membership.as_deref();
    let text = filled.text.as_str();
    let saved = filled.saved.as_ref().map(|(id, at)| (id.as_str(), *at));
    let view = &filled.view;
    let mut carried = hidden("realm", realm);
    if let Some(membership) = membership {
        carried.push_str(&hidden("membership", membership));
    }
    if let Some((id, at)) = saved {
        carried.push_str(&hidden("saved", id));
        carried.push_str(&hidden("saved_revision", &at.to_string()));
    }
    // the view the URL named, hidden (the one renderer both editors
    // share), a count's keys among them, which the box has no control for
    carried.push_str(&view_hidden(view, true));
    // the presentation, as the big form's "Results as"
    let results_as = results_as(view.cards);
    format!(
        "<details class=\"query-details\"{}><summary>The query, in the language</summary><p class=\"index\"><a href=\"{}\">{}</a></p><form class=\"query-box\" method=\"get\" action=\"search#{ANSWER}\" data-count>{carried}<textarea id=\"query-text\" aria-label=\"the query, in the language\" name=\"text\" rows=\"3\">{}</textarea>{} {results_as} <button type=\"submit\">Run this query</button> {COUNT}{}</form></details>\n",
        if filled.text_open { " open" } else { "" },
        esc(index),
        esc(THE_INDEX),
        esc(text),
        rows_shown(view.limit),
        finder
    )
}

/// "Show", the rows' number, on both editors (the search form's bar and
/// the query box) by this one maker: the search's own default first, sent
/// empty, so a form left as it is asks no limit, then the form's
/// [`LIMITS`], and a number the URL named that neither offers, in its
/// place among them; the number shown chosen, the default where none was.
fn rows_shown(limit: Option<usize>) -> String {
    let listed = acquisition_store::bound::LISTED;
    let mut offered: Vec<usize> = LIMITS.to_vec();
    if let Some(n) = limit.filter(|n| *n != listed && !LIMITS.contains(n)) {
        offered.push(n);
        offered.sort_unstable();
    }
    let chosen = limit.unwrap_or(listed);
    let mut out = format!(
        "<label class=\"control\">Show <select name=\"view.limit\"><option value=\"\"{}>{listed}</option>",
        if chosen == listed { " selected" } else { "" }
    );
    for n in offered {
        out.push_str(&format!(
            "<option value=\"{n}\"{}>{n}</option>",
            if n == chosen { " selected" } else { "" }
        ));
    }
    out.push_str("</select></label>");
    out
}

/// How the rows of a search's answer are drawn: `view.fields` sent empty, the page's default, the table; or
/// [`CARDS`], selected where the answer was cards. Both editors' one
/// control.
fn results_as(cards: bool) -> String {
    format!(
        "<label class=\"control\">Results as <select name=\"view.fields\"><option value=\"\"{}>a table</option><option value=\"{CARDS}\"{}>cards</option></select></label>",
        if cards { "" } else { " selected" },
        if cards { " selected" } else { "" }
    )
}

/// The rows' fields that ask the cards, as the CLI's `--fields` names
/// them: the item with its art. The table, the art alone, is the page's
/// default and names none.
pub(crate) const CARDS: &str = "icon,item";

/// What both editors carry of the view: the sort and the values the URL
/// named, never the values the query gave unasked, the cards where the rows
/// were cards, a count's keys, and the rows' or the count's number. Made from
/// an answer's rows, a request's count, or a refused search's fields as
/// sent; the save note says what of it a save leaves behind.
#[derive(Default)]
pub(crate) struct BoxView {
    pub sort: Option<String>,
    pub desc: bool,
    pub values: Vec<String>,
    /// The rows as cards, asked (`view.fields=icon,item`); the table, the
    /// page's default, otherwise.
    pub cards: bool,
    pub limit: Option<usize>,
    /// A count's keys as the form's one field reads them.
    pub count: Option<String>,
}

impl BoxView {
    /// What an answer's rows carry into the box.
    pub fn of(rows: &RowsOut) -> BoxView {
        BoxView {
            sort: rows.sort.clone(),
            desc: rows.sort.is_some() && rows.desc,
            values: match rows.values_from_query {
                true => Vec::new(),
                false => rows.values.clone(),
            },
            cards: rows.fields == [RowField::Icon, RowField::Item],
            limit: Some(rows.limit),
            count: None,
        }
    }

    /// What a count carries into the editors: its keys, as the form's one
    /// field reads them, and its limit.
    pub fn counted(keys: String, limit: Option<usize>) -> BoxView {
        BoxView {
            count: Some(keys),
            limit,
            ..BoxView::default()
        }
    }

    /// What a search's fields name of the view, as sent.
    pub fn sent(fields: &[(String, String)]) -> BoxView {
        let named = |name: &str| {
            fields
                .iter()
                .find(|(k, v)| k == name && !v.is_empty())
                .map(|(_, v)| v.as_str())
        };
        BoxView {
            sort: named("view.sort").map(str::to_string),
            desc: named("view.desc") == Some("true"),
            values: fields
                .iter()
                .filter(|(k, v)| k == "view.value" && !v.is_empty())
                .map(|(_, v)| v.clone())
                .collect(),
            cards: named("view.fields").is_some_and(|f| {
                RowField::list(f).is_ok_and(|l| l == [RowField::Icon, RowField::Item])
            }),
            limit: named("view.limit").and_then(|n| n.parse().ok()),
            count: named("view.count").map(str::to_string),
        }
    }

    /// The parts of the view the box carries that a saved search, a
    /// query and its scope (C167), leaves behind, in the page's words.
    fn left_by_a_save(&self) -> Vec<&'static str> {
        let mut left = Vec::new();
        if self.sort.is_some() {
            left.push("its order");
        }
        if !self.values.is_empty() {
            left.push("its columns");
        }
        if self.cards {
            left.push("the cards");
        }
        if self.count.is_some() {
            left.push("its count");
        }
        left
    }
}

/// The trade site box (#188), in the search form's bar after the item
/// search (`4-full-1-form-rides.md`, Decision 5): a site search as the
/// CLI's `--trade` takes it, run as the CLI's trade answer is — the
/// translation's fidelity and every clause above the rows, the site's sort
/// reported and not applied. Each of its controls is owned by its own form
/// ([`TRADE_FORM`], by `form=`), so the search form sends none of them and
/// each form's count asks its own fields. It carries the realm the page
/// shows, which an id or the query JSON is searched in; a URL names its
/// own, which wins. It holds `text` where a site search's decode refused
/// it; one that decodes fills the form instead.
fn trade_box(realm: &str, text: &str) -> String {
    // what the box does with a search, said on hover
    const SITE_BOX_SAYS: &str =
        "Translated here and sent nowhere; a URL&#39;s realm is the search&#39;s";
    format!(
        "<span class=\"site-search\"><input type=\"hidden\" name=\"realm\" value=\"{}\" form=\"{TRADE_FORM}\"><textarea id=\"trade-text\" name=\"trade\" form=\"{TRADE_FORM}\" rows=\"1\" placeholder=\"A trade site search: its URL, its id or its query JSON\" aria-label=\"A trade site search: its URL, its id or its query JSON\" title=\"{SITE_BOX_SAYS}\">{}</textarea><button type=\"submit\" form=\"{TRADE_FORM}\" title=\"{SITE_BOX_SAYS}\">Run the site's search</button> {}</span>",
        esc(realm),
        esc(text),
        count_of(TRADE_FORM)
    )
}

/// One filter's control, named by its path, its fate beside it, its value
/// the fill's at that path; a listing-only row's disabled, its `says` as
/// the title, showing no value and holding none (a select at a dash):
/// what the fill holds there is named above the form ([`Fill::left`]).
fn control(form: &Form, fill: &Fill, filter: &Filter, path: &str, key: &str) -> String {
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
    // a control's value: held where it is sent; a listing control, which
    // sends nothing, shows none (#317): a value it showed would read as
    // one the form sends; it is named above the form with the rest no
    // control holds
    let text = |at: &str| match listing {
        true => None,
        false => fill.text(at),
    };
    let number = |at: &str| match listing {
        true => None,
        false => fill.number(at),
    };
    let mut out = format!(
        "<div class=\"filter\"{title}{}><span class=\"filter-title\"{tip}>{label}</span> ",
        unrun(fate)
    );
    if let Some(options) = &filter.options {
        let at = format!("{path}.option");
        let chosen = match listing {
            true => None,
            false => fill.option(&at, |v| {
                options.iter().any(|(id, _)| id.as_deref() == Some(v))
            }),
        };
        out.push_str(&format!("<select name=\"{at}\"{disabled}>"));
        // a listing select stands at no option of its own: none is sent
        if listing {
            out.push_str("<option value=\"\" selected>\u{2014}</option>");
        }
        for (id, text) in options {
            let value = id.as_deref().unwrap_or("");
            let fate = id.as_ref().and_then(|id| form.fate(&format!("{key}={id}")));
            let own = fate.map(|f| f.marked()).unwrap_or_default();
            out.push_str(&format!(
                "<option value=\"{}\"{}{}>{}{}</option>",
                esc(value),
                unrun(fate),
                if chosen.as_deref() == Some(value) {
                    " selected"
                } else {
                    ""
                },
                esc(text),
                esc(&own)
            ));
        }
        out.push_str("</select>");
    } else if filter.known_item || filter.input {
        out.push_str(&format!(
            "<input name=\"{path}.option\"{} size=\"16\" placeholder=\"any\"{disabled}>",
            valued(text(&format!("{path}.option")).as_deref())
        ));
    }
    if filter.min_max {
        out.push_str(&format!(
            "<input type=\"number\" step=\"any\" name=\"{path}.min\"{} placeholder=\"min\"{disabled}> <input type=\"number\" step=\"any\" name=\"{path}.max\"{} placeholder=\"max\"{disabled}>",
            valued(number(&format!("{path}.min")).as_deref()),
            valued(number(&format!("{path}.max")).as_deref()),
        ));
    }
    // a box per socket word, in the table's order, named by the site's
    // key and marked with GGG's letter: `A` the abyssal count after `W`
    if filter.sockets {
        for s in &SOCKET_WORDS {
            out.push_str(&format!(
                " <input class=\"socket\" type=\"number\" name=\"{path}.{}\"{} placeholder=\"{}\"{disabled}>",
                s.key,
                valued(number(&format!("{path}.{}", s.key)).as_deref()),
                s.letter
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

/// The answer page: the answer document rendered
/// as HTML in C115's order — the count and the badges, a site search's
/// translation, the opening (the query, the scope, the total), the rows
/// with their continuation or a count's tables (rows first, what stands
/// before them one `details` after them, closed where a row stands),
/// what a zero answer
/// offers, why items are undecided, the terms, the counts left without a
/// route, the basis — then the query box and the save form. The words of
/// each fact are the text's, made once in `search_cmd.rs`; every request
/// the document carries is a way on through `spell`, a link where a page
/// takes it and its words where none does, and no command line is
/// printed. Opened from a saved search, it says so above the answer and
/// carries the search through its query box and its continuation. Rows
/// that carry the item are cards ([`card`]), each followed by what the
/// row shows of the query; rows that do not are the table
/// ([`rows_table`]), every row in one page, its first block the
/// `window` the handler made over the page's corpus; and
/// `rows_links`, which the handler makes, are the plain links to the same
/// answer the other way and, on a continuation's page, to its first rows;
/// `view` makes the link a value's head is ([`ViewLink`]).
pub(crate) fn answer(
    a: &Answer,
    saving: Saving,
    links: &SavedLinks,
    spell: Spell,
    rows_links: &RowsLinks,
    view: ViewLinks,
    shown: &Shows,
) -> String {
    let Shows {
        window,
        placed,
        fields: table_fields,
        site,
    } = *shown;
    let Saving {
        source,
        opened,
        view: boxed,
        from: saving_from,
    } = saving;
    let now = acquisition_store::now();
    let all_realms = a.scope.realm == acquisition_search::Realm::All;
    // the answer is the search that ran, whatever the editors above hold
    // now, the count beside each button being the form's as it stands
    // (#317)
    let mut out = String::from("<h2 class=\"ran\">The search that ran</h2>\n");
    out.push_str(
        &opened
            .map(|o| opened_notice(a, o, links))
            .unwrap_or_default(),
    );
    // what stands before the rows: the count, the badges, a site search's
    // translation and the opening (the query, the scope, the total)
    let mut about = format!(
        "<p class=\"count\">{}</p>\n",
        plural(a.total.matched, "match", "matches")
    );
    let carried = opened.map(|o| (o.id.as_str(), o.at));
    let carried_query = carried
        .map(|(id, at)| format!("&saved={}&saved_revision={at}", enc(id)))
        .unwrap_or_default();
    about.push_str(&answer_badges(a));
    let (typed, typed_at) = match &a.set_aside {
        Some(aside) => (aside, "/set_aside"),
        None => (&a.query, "/query"),
    };
    if let Some(trade) = &typed.trade {
        about.push_str(&trade_section(trade, typed_at, spell, site));
    }
    about.push_str(&opening(a, now, spell));
    match &a.view {
        // rows open the answer: what stood before them is one `details` after them, closed, and open
        // where no row stands to read first
        ViewOut::Rows(_) => {}
        _ => out.push_str(&about),
    }
    match &a.view {
        ViewOut::Rows(rows) => {
            let presented: Vec<String> = rows_links
                .other
                .iter()
                .chain(&rows_links.places)
                .map(|p| {
                    format!(
                        "<a class=\"presentation\" href=\"{}\">{}</a>",
                        esc(&p.href),
                        esc(p.words)
                    )
                })
                .collect();
            if !presented.is_empty() {
                out.push_str(&format!("<p>{}</p>\n", presented.join(" · ")));
            }
            // a card wherever the rows carry the item, the table
            // otherwise: the document's view says which, never the URL.
            // The table holds every row and says so in its own line.
            if !rows.fields.contains(&RowField::Item) {
                // in a GET form holding the page's fields, so a row's
                // toggle and a card's "Close" ask the page again with
                // the opened rows the URL names
                if let Some(window) = window {
                    let hidden: String = table_fields
                        .iter()
                        .map(|(k, v)| {
                            format!(
                                "<input type=\"hidden\" name=\"{}\" value=\"{}\">",
                                esc(k),
                                esc(v)
                            )
                        })
                        .collect();
                    out.push_str(&format!(
                        "<form class=\"table\" method=\"get\" action=\"search\">{hidden}\n{}</form>\n",
                        rows_table(a, rows, window, view, placed, now)
                    ));
                }
            } else {
                out.push_str(&cards(a, view, placed, now));
            }
            // which cards are shown and in what order, wherever there are
            // rows or rows left out; then the way on, and on a
            // continuation's page the way back to the first rows
            if rows.fields.contains(&RowField::Item) && (rows.left_out > 0 || !a.rows.is_empty()) {
                let mut line = esc(&search_cmd::rows_shown_words(a, rows));
                if let Some(way) = order_way(rows, Presented::Cards).filter(|_| !a.rows.is_empty())
                {
                    line.push_str(&format!(" · {}", esc(&way)));
                }
                if rows.left_out > 0 {
                    line.push_str(&match &rows.next {
                        Some(next) => format!(
                            " · {}",
                            way_on(
                                spell,
                                "/view/rows/next",
                                &Way::Search(Box::new(next.request.clone())),
                                "more",
                                &carried_query
                            )
                        ),
                        None => " · a larger number of rows returns the rest".to_string(),
                    });
                }
                if let Some(first) = &rows_links.first {
                    line.push_str(&format!(
                        " · <a class=\"first\" href=\"{}\">first rows</a>",
                        esc(first)
                    ));
                }
                out.push_str(&format!("<p class=\"rows-shown\">{line}</p>\n"));
            }
            out.push_str(&format!(
                "<details class=\"about\"{}><summary>{}</summary>\n{about}</details>\n",
                if a.total.matched == 0 { " open" } else { "" },
                esc(&format!(
                    "Its query, scope and total: {}",
                    plural(a.total.matched, "match", "matches")
                ))
            ));
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
            esc(&search_cmd::unrouted_words(unrouted))
        ));
    }
    out.push_str(&basis_foot(&search_cmd::basis_text(&a.basis)));
    out.push_str(&save_forms(a, source, opened, links, boxed, saving_from));
    out
}

/// How an answer page shows its rows, which the handler makes: the
/// table's window, where the rows are a table; each card's place as a
/// link; and the fields the table's form carries hidden, the page's own
/// with the opened rows it keeps.
#[derive(Clone, Copy)]
pub(crate) struct Shows<'a> {
    pub window: Option<&'a Window>,
    pub placed: Placed<'a>,
    pub fields: &'a [(String, String)],
    /// A site search's link in each league the form names, the handler's
    /// ([`SiteLinks`]); none where the answer runs no site search or the
    /// composer refused it.
    pub site: Option<&'a SiteLinks>,
}

/// A site search's link back to the site once per league (C195, a link
/// may carry a league a person names for it alone), each written by the
/// search crate's composer, and the one shown first: the select beside
/// the link swaps between them (`assets/link.js`), composing nothing.
pub(crate) struct SiteLinks {
    /// Each league with its link, in the league control's order: every
    /// one a URL, with what the site cannot be asked and its notes, the
    /// same in every league.
    pub links: Vec<(String, Link)>,
    /// The one the link starts at.
    pub start: usize,
}

/// The basis at the foot of an answer or an item, in the text's words:
/// what a person quotes to an agent and an agent to a person (the
/// design's §3). Its line in the page source is the text's line, so a
/// recording masks its digests as a text's.
fn basis_foot(words: &str) -> String {
    format!("<p class=\"basis\">\n{}\n</p>\n", esc(words))
}

/// A site search's translation (C113): its source and fidelity, what it
/// runs over, its link back to the site — the one link of the pages that
/// leaves the server, opened in a new tab with no opener, the server's
/// `Referrer-Policy: same-origin` keeping the token from the site — or
/// why there is none, with what it leaves out and its notes (C195), then
/// every clause the document lists as not run, ambiguous
/// or reported, each its status, where it stands and what it says, an
/// ambiguous clause's other reading with its adds and drops as ways.
/// `at` is the pointer of the query holding it.
fn trade_section(trade: &TradeOut, at: &str, spell: Spell, site: Option<&SiteLinks>) -> String {
    let mut out = format!(
        "<section class=\"trade\">\n<p>{}</p>\n<p>{}</p>\n",
        esc(&search_cmd::fidelity_words(trade)),
        esc(&format!("Over {}", search_cmd::TRADE_OVER))
    );
    // the link drawn: the league select's first, whole, where there is
    // one, else the answer's own
    let drawn = site.map_or(&trade.link, |site| &site.links[site.start].1);
    let url_of = |link: &Link| match link {
        Link::Url { url, .. } => url.clone(),
        Link::None { .. } => String::new(),
    };
    for (word, said) in search_cmd::link_words(drawn) {
        let said = match (&trade.link, site) {
            // the link in its own league: a select beside it of the
            // leagues the form names, each option's value its link,
            // which the select swaps into the anchor; changing it asks
            // nothing here
            (_, Some(site)) if word == "link" => {
                let options: String = site
                    .links
                    .iter()
                    .enumerate()
                    .map(|(n, (league, link))| {
                        format!(
                            "<option value=\"{}\"{}>{}</option>",
                            esc(&url_of(link)),
                            if n == site.start { " selected" } else { "" },
                            esc(league)
                        )
                    })
                    .collect();
                format!(
                    "<a href=\"{}\" target=\"_blank\" rel=\"noopener\" data-link>the search on the trade site</a> in <select data-link-league aria-label=\"The trade site's league for this link\">{options}</select> <span class=\"fate\">the link's own league: changing it changes nothing searched here</span>",
                    esc(&url_of(&site.links[site.start].1))
                )
            }
            (Link::Url { url, .. }, None) if word == "link" => format!(
                "<a href=\"{}\" target=\"_blank\" rel=\"noopener\">the search on the trade site</a>",
                esc(url)
            ),
            _ => esc(&said),
        };
        out.push_str(&format!(
            "<p class=\"link\">{}: {said}</p>\n",
            esc(&format!("{}{}", word[..1].to_uppercase(), &word[1..]))
        ));
    }
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
            // the list that page shows: the never fetched
            way_on(
                spell,
                "tabs",
                &way,
                &format!("each tab of {} never fetched", pointer.league),
                "&fetched=never"
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
    for words in a.scope.releases_words(now) {
        total.push_str(&format!("<br>{}", esc(&words)));
    }
    out.push_str(&fact_row("Total", &total));
    if let Some(words) = search_cmd::before_words(a) {
        out.push_str(&fact_row("Then", &esc(&words)));
    }
    out.push_str("</table>\n");
    out
}

/// The link to the same answer in the other presentation: the page's own
/// URL, its fields as sent, with `view.fields=icon` added or removed, and
/// its words. It carries no request of the document, so it is no way: a
/// plain link of its own class.
pub(crate) struct Presentation {
    pub href: String,
    pub words: &'static str,
}

/// A change to the view, made by the handler's one maker (`mod.rs`,
/// `view_link`): the same page ordered by one
/// value, largest first or, where the rows are ordered by it, the other
/// way, the value added to the list where the view lacks it. `held` says
/// the view's values hold it; `ordered` says the rows are ordered by it,
/// and which way (`true` largest first). A page writes a head or a number
/// as this link and composes no request.
pub(crate) struct ViewLink {
    pub href: String,
    pub held: bool,
    pub ordered: Option<bool>,
}

/// The maker of a change to the view, by one value's text: none where no
/// link stands (a value outside a full list, a search the form cannot ask).
pub(crate) type ViewLinks<'a> = &'a dyn Fn(&str) -> Option<ViewLink>;

/// `text`, already HTML, marked where the rows are ordered by its value
/// (`Some(true)` largest first): an arrow before it the eye reads, hidden
/// from a screen reader, and words after it that only a screen reader
/// gets.
fn ordered(text: &str, order: Option<bool>) -> String {
    let Some(desc) = order else {
        return text.to_string();
    };
    let (arrow, words) = match desc {
        true => ("\u{25bc}", "ordered largest first"),
        false => ("\u{25b2}", "ordered smallest first"),
    };
    format!(
        "<span class=\"mark\" aria-hidden=\"true\">{arrow}</span>{text}<span class=\"sr\">, {words}</span>"
    )
}

/// The plain links beside a rows answer's rows, which the handler makes
/// (the page builds no request): the same answer the other way, and on a
/// continuation's page the same search from its first rows.
#[derive(Default)]
pub(crate) struct RowsLinks {
    pub other: Option<Presentation>,
    pub first: Option<String>,
    /// The same query by stash tab and by character.
    pub places: Vec<Presentation>,
}

/// What a row shows of the query and its sort, each line in the text's
/// words, what it leaves out a link to the item whole.
fn row_lines(a: &Answer, row: &acquisition_search::answer::Row) -> Vec<String> {
    let whole = |words: String| format!("<a href=\"item/{}\">{}</a>", enc(&row.id), esc(&words));
    let (shows, more) = search_cmd::row_shows(a, row);
    shows
        .into_iter()
        .map(|piece| match piece {
            RowPiece::Seen(s) => esc(&s),
            RowPiece::MoreSorted(n) => whole(search_cmd::more_sorted_words(n)),
            RowPiece::MoreOfValue(n, key) => whole(search_cmd::more_of_value_words(n, &key)),
        })
        .chain(
            more.iter()
                .map(|(n, path)| whole(search_cmd::more_of_term_words(*n, path))),
        )
        .collect()
}

/// The rows as cards, in the answer's order: each row's item as [`card`]
/// renders it, its name a link to the item page, then what the row shows
/// of the query.
fn cards(a: &Answer, view: ViewLinks, placed: Placed, now: i64) -> String {
    if a.rows.is_empty() {
        return String::new();
    }
    let mut out = String::from("<div class=\"cards\">\n");
    for row in &a.rows {
        let Some(shown) = &row.shown else {
            continue;
        };
        let icon = row.icon.as_ref().and_then(|icon| icon.as_deref());
        out.push_str(&card(
            ItemParts::of_row(shown),
            &row.place,
            icon,
            true,
            Some(view),
            placed(&row.place).as_deref(),
            now,
        ));
        let lines = row_lines(a, row);
        if !lines.is_empty() {
            out.push_str(&format!("<p class=\"shows\">{}</p>\n", lines.join("<br>")));
        }
    }
    out.push_str("</div>\n");
    out
}

/// One item as a card, from what `show`'s one
/// maker makes of it and where it lies, in the trade site's order: the
/// art where it was asked, the badges, the name (a link to the item page
/// where `link`), the base, the properties with the item level and the
/// requirements, the lines in the item's own order — the deriver's, by
/// source word, a group per run of one source, each line with its kind —
/// the flags, the sockets, the price, and the place with its age and
/// where the item left. Every fact in the words the CLI's text says it
/// in, made once in `search_cmd.rs`; the layout and the labels are the
/// page's. The answer page renders a card per row, the item page one.
/// With `view` (the answer page's), each number that a value
/// reads is that value's [`ViewLink`]: a line whole by its `value`, a
/// displayed row's piece by the number field it shows; one the view holds
/// is marked `valued`, the sort's as its head is. The item page has no
/// view, and its numbers are words.
pub(crate) fn card(
    parts: ItemParts<'_>,
    place: &acquisition_search::corpus::Place,
    icon: Option<&str>,
    link: bool,
    view: Option<ViewLinks>,
    placed: Option<&str>,
    now: i64,
) -> String {
    // `html`, already escaped, as the link a value's change to the view is
    let linked = |html: String, value: Option<&str>| match value
        .zip(view)
        .and_then(|(value, view)| view(value))
    {
        Some(to) => format!(
            "<a class=\"value{}\" href=\"{}\">{}</a>",
            if to.held { " valued" } else { "" },
            esc(&to.href),
            ordered(&html, to.ordered)
        ),
        None => html,
    };
    let it = parts.item;
    let name = search_cmd::title(it.name.as_deref(), it.typeline.as_deref());
    let mut out = String::from("<div class=\"card\">\n");
    if let Some(url) = icon {
        out.push_str(&format!(
            "<img class=\"art\" src=\"{}\" alt=\"{}\">\n",
            esc(url),
            esc(&name)
        ));
    }
    out.push_str(&item_badges(parts));
    let rarity = esc(&it.rarity.as_deref().unwrap_or("").to_lowercase());
    let title = esc(&search_cmd::shown_title(it));
    out.push_str(&if link {
        format!(
            "<p><a class=\"name rarity-{rarity}\" href=\"item/{}\">{title}</a></p>\n",
            enc(&it.facts.id)
        )
    } else {
        format!("<p class=\"name rarity-{rarity}\">{title}</p>\n")
    });
    if let Some(base) = &it.base {
        out.push_str(&format!("<p class=\"base\">{}</p>\n", esc(base)));
    }
    // the properties, the item level and the requirements, as
    // `shown_properties` says them, each row its pieces, a number field's
    // piece its link
    let displays = parts.displays;
    let rows: Vec<String> = (0..it.properties.len())
        .map(|at| displays.property_row(at))
        .chain([displays.item_level_row(), displays.requires_row()])
        .filter(|row| !row.is_empty())
        .map(|row| {
            row.iter()
                .map(|piece| {
                    linked(
                        esc(&piece.text.replace('\n', " / ")),
                        piece.value.as_deref(),
                    )
                })
                .collect()
        })
        .collect();
    if !rows.is_empty() {
        out.push_str(&format!(
            "<p class=\"properties\">{}</p>\n",
            rows.join("<br>")
        ));
    }
    // a group per run of one source, in the item's own order
    let mut group: Option<&str> = None;
    for l in parts.lines {
        if group != Some(l.line.source.as_str()) {
            if group.is_some() {
                out.push_str("</div>\n");
            }
            out.push_str("<div class=\"mods\">\n");
            group = Some(l.line.source.as_str());
        }
        let text: Vec<String> = l.line.text.split('\n').map(esc).collect();
        out.push_str(&format!(
            "<p class=\"mod\">{} <span class=\"mod-kind\">{}</span></p>\n",
            linked(text.join("<br>"), l.value.as_deref()),
            esc(&search_cmd::line_kind_words(l))
        ));
    }
    if group.is_some() {
        out.push_str("</div>\n");
    }
    if let Some(flags) = search_cmd::flags_words(it) {
        out.push_str(&format!("<p class=\"flags\">is {}</p>\n", esc(&flags)));
    }
    if let Some(sockets) = search_cmd::sockets_words(it) {
        out.push_str(&format!(
            "<p class=\"sockets\">sockets {}</p>\n",
            esc(&sockets)
        ));
    }
    out.push_str(&format!(
        "<p class=\"price\">price {}</p>\n",
        esc(&search_cmd::price_words(&it.facts.id, parts.price))
    ));
    // the place, a link to its page with it open where a page lists it, its
    // words otherwise; the link lands at this item's row (`at`, one
    // load's), so a long tab is not searched again for it, and a removed
    // item, which the place's live rows lack, at the place's head
    let words = esc(&search_cmd::shown_place_words(place, now));
    let mut placed = match placed {
        Some(href) => {
            let landing = match it.facts.removed_at {
                None => format!("&at={}#{}", enc(&it.facts.id), row_anchor(&it.facts.id)),
                Some(_) => format!("#{}", place_anchor(&place.id)),
            };
            format!(
                "<a class=\"placed\" href=\"{}\">{words}</a>",
                esc(&format!("{href}{landing}"))
            )
        }
        None => words,
    };
    if let Some(removed_at) = it.facts.removed_at {
        placed.push_str(&format!(
            "<br>removed {}",
            esc(&search_cmd::shown_removed_words(now, removed_at))
        ));
    }
    out.push_str(&format!("<p class=\"place\">{placed}</p>\n"));
    out.push_str("</div>\n");
    out
}

/// A table row's height in CSS pixels, written on the table and read
/// from there by the window script, which sets the stylesheet's `--row`:
/// the one source of the height.
pub(crate) const ROW_HEIGHT: u32 = 24;

/// The rows a block of the table holds, the server's first and each the
/// window script asks: written on the table beside the row height.
pub(crate) const BLOCK_ROWS: usize = 50;

/// A table's window (the module doc, "The table"): its route, to which
/// the script adds an offset and a count, and its segments, the order cut
/// after each opened row, each with the blocks the
/// server rendered over the page's corpus.
pub(crate) struct Window {
    pub route: String,
    pub segments: Vec<Segment>,
}

/// One segment of a window: the rows from `base`, `count` of them, the
/// blocks the server rendered, each by its number in the segment, and
/// whether it ends at an opened row, whose card stands after it.
pub(crate) struct Segment {
    pub base: usize,
    pub count: usize,
    pub blocks: Vec<(usize, Answer)>,
    pub opened: bool,
}

impl Window {
    /// The rows of the whole order.
    pub fn count(&self) -> usize {
        self.segments.iter().map(|s| s.count).sum()
    }

    /// The basis every block stands at, the first block's.
    pub fn basis(&self) -> Option<&acquisition_search::Basis> {
        self.segments
            .first()
            .and_then(|s| s.blocks.first())
            .map(|(_, a)| &a.basis)
    }

    /// The rows the server rendered.
    fn rendered(&self) -> usize {
        self.segments
            .iter()
            .flat_map(|s| &s.blocks)
            .map(|(_, a)| a.rows.len())
            .sum()
    }
}

/// A card's place as a link: the place page with
/// the place and each holding it open, a URL with fields and no fragment,
/// to which the card adds its landing; none where no page lists the
/// place, which then stands as words.
pub(crate) type Placed<'a> = &'a dyn Fn(&acquisition_search::corpus::Place) -> Option<String>;

/// What an opened row's card links: the
/// view's changes, made from a block's rows, its place, and the page's
/// query against it.
pub(crate) struct CardWays<'a> {
    pub view: &'a dyn Fn(&RowsOut, &str) -> Option<ViewLink>,
    pub placed: Placed<'a>,
    /// The page's query, canonical, where it has one: the card links the
    /// item page against it.
    pub against: Option<&'a str>,
}

/// A row's anchor: where a row's toggle lands, made beside
/// [`place_anchor`].
pub(crate) fn row_anchor(id: &str) -> String {
    format!("i-{}", enc(id))
}

/// A window's segments, the one renderer of the
/// flat table and of each open place: each segment of one height, its
/// route the window's, its base and count, the blocks the server
/// rendered; after a segment that ends at an opened row, that row's card
/// in the page's flow, with its "Close". Without the script, the rows
/// the server rendered alone, said so.
fn segments(window: &Window, ways: &CardWays, now: i64) -> String {
    let count = window.count();
    let rendered = window.rendered();
    let mut out = String::new();
    if rendered < count {
        let words = match window.segments.len() {
            1 => format!(
                "The first {} of {count} are shown here: the rest come in as the list scrolls, which needs the page's script.",
                plural(rendered, "item", "items")
            ),
            _ => format!(
                "{} of {count}, around each opened one, are shown here: the rest come in as the list scrolls, which needs the page's script.",
                plural(rendered, "item", "items")
            ),
        };
        out.push_str(&format!(
            "<noscript><p class=\"rows-shown\">{}</p></noscript>\n",
            esc(&words)
        ));
    }
    for seg in &window.segments {
        out.push_str(&format!(
            "<div class=\"seg\" data-route=\"{}\" data-count=\"{}\" data-base=\"{}\">",
            esc(&window.route),
            seg.count,
            seg.base
        ));
        for (b, block) in &seg.blocks {
            out.push_str(&format!(
                "<div class=\"block\" data-block=\"{b}\">\n{}</div>",
                window_rows(block, now)
            ));
        }
        out.push_str("</div>\n");
        if !seg.opened {
            continue;
        }
        // the opened row, the segment's last: in the block that holds it
        let last = seg.base + seg.count - 1;
        let held = seg.blocks.iter().find_map(|(_, block)| {
            let after = match &block.view {
                ViewOut::Rows(rows) => rows.after,
                _ => 0,
            };
            last.checked_sub(after)
                .and_then(|n| block.rows.get(n))
                .map(|row| (block, row))
        });
        if let Some((block, row)) = held {
            out.push_str(&opened_card(block, row, ways, now));
        }
    }
    out
}

/// An opened row's card: [`card`] of the row's
/// item, which the window asks, its numbers the view's changes made from
/// the block's rows, its place a link; then its "Close", the row's own
/// toggle.
fn opened_card(
    block: &Answer,
    row: &acquisition_search::answer::Row,
    ways: &CardWays,
    now: i64,
) -> String {
    let Some(shown) = &row.shown else {
        return String::new();
    };
    let view = |value: &str| match &block.view {
        ViewOut::Rows(rows) => (ways.view)(rows, value),
        _ => None,
    };
    let icon = row.icon.as_ref().and_then(|icon| icon.as_deref());
    // the query's outcome on it, term by term, one link away: `show
    // --against`'s document on the item page
    let against = ways
        .against
        .filter(|text| !text.is_empty())
        .map(|text| {
            format!(
                " <a class=\"against\" href=\"item/{}?against={}\">the query against it, term by term</a>",
                enc(&row.id),
                enc(text)
            )
        })
        .unwrap_or_default();
    format!(
        "<div class=\"opened\">\n{}<p class=\"ways\"><button type=\"submit\" name=\"toggle_item\" value=\"{}\">Close</button>{against}</p>\n</div>\n",
        card(
            ItemParts::of_row(shown),
            &row.place,
            icon,
            true,
            Some(&view),
            (ways.placed)(&row.place).as_deref(),
            now,
        ),
        esc(&row.id)
    )
}

/// The table: every row of the answer in one
/// scrolling page, at one row height. Its heads are the item, its base,
/// its price, its location and when that was fetched, as text, then each
/// value the answer carries by its own text, a link that orders the rows
/// by it and flips where they are ([`ViewLink`]), the ordered
/// one marked ([`ordered`]); its rows are one segment of the
/// answer's whole count, of which the server renders the first block and
/// the window script asks the rest by the segment's route. Its line says
/// how many items it holds, in what order and which values it left out;
/// a page without its script says that it shows the first rows alone.
fn rows_table(
    a: &Answer,
    rows: &RowsOut,
    window: &Window,
    view: ViewLinks,
    placed: Placed,
    now: i64,
) -> String {
    let heads = item_heads(rows, view);
    let matched = a.total.matched;
    // the order, said once for every row: a sort named is said here, as
    // each row of the cards says it sorts by it
    let order = match (search_cmd::order_words(rows), rows.sort.as_deref()) {
        (Some(words), _) => format!(", {words}"),
        (None, Some(sort)) => format!(
            ", by {sort}, {} first, as asked",
            if rows.desc { "largest" } else { "smallest" }
        ),
        (None, None) => String::new(),
    };
    let values = search_cmd::values_left_out_words(rows)
        .map(|words| format!("; {words}"))
        .unwrap_or_default();
    let way = order_way(rows, Presented::Table)
        .filter(|_| matched > 0)
        .map(|way| format!(" · {way}"))
        .unwrap_or_default();
    let line = format!(
        "{} in the table{order}{values}{way}",
        plural(matched, "matching item", "matching items")
    );
    let ways = CardWays {
        view: &|_, value| view(value),
        placed,
        against: Some(&a.query.text),
    };
    format!(
        "<p class=\"rows-shown\">{}</p>\n<div class=\"window\" data-row-height=\"{ROW_HEIGHT}\" data-block-rows=\"{BLOCK_ROWS}\">\n{heads}{}</div>\n",
        esc(&line),
        segments(window, &ways, now)
    )
}

/// Which rows a sentence says how to order.
#[derive(Clone, Copy)]
enum Presented {
    Cards,
    Table,
    Places,
}

/// Where rows stand as they lie in their place because no sort was asked,
/// how a page orders them: a number on an item's card orders the rows by
/// its value, and a table's or a place's item opens to its card by its
/// name. Without it a reader meets no control that sorts until a query
/// compares a value. None where a sort orders the rows, whose head flips
/// it.
fn order_way(rows: &RowsOut, shape: Presented) -> Option<String> {
    rows.sort.is_none().then(|| match shape {
        Presented::Cards => "press a number on a card to order the rows by it".to_string(),
        Presented::Table => {
            "open an item by its name and press a number on its card to order the rows by it"
                .to_string()
        }
        Presented::Places => format!(
            "Each place's items stand {PLACE_ORDER_WORDS}: open one by its name and press a number on its card to order them by it."
        ),
    })
}

/// The table's fixed heads, each with why it does not order the rows
/// (#327): the page sorts nothing itself, and
/// the language's sort takes a number alone, which none of them is. Said
/// on hover, as the head's title; the values that would order them wait
/// on the language.
const FIXED_HEADS: [(&str, &str); 5] = [
    (
        "Item",
        "Not sortable yet: the search orders rows by a number, and an item's name is text",
    ),
    (
        "Base",
        "Not sortable yet: the search orders rows by a number, and a base is text",
    ),
    (
        "Price",
        "Not sortable yet: each price is in its own currency, so an order by its amount would put 5 divines below 10 chaos, and the search holds no price in one currency",
    ),
    (
        "Location",
        "Not sortable yet: the search orders rows by a number, and a location is text",
    ),
    (
        "Fetched",
        "Not sortable yet: when a place was fetched is no value the search orders rows by",
    ),
];

/// A table's heads, one line: the item, its base, its price, its location
/// and when that was fetched, as text, none a value the sort takes, each
/// saying on hover why ([`FIXED_HEADS`]); then
/// each value the rows carry by its own text, the link that orders the
/// rows by it, the ordered one marked ([`ordered`]). The flat table's
/// and a place page's, which stands once above its places.
fn item_heads(rows: &RowsOut, view: ViewLinks) -> String {
    let fixed = FIXED_HEADS.iter().map(|(h, why)| {
        format!(
            "<span class=\"unsorted\" title=\"{}\">{}</span>",
            esc(why),
            esc(h)
        )
    });
    let valued = rows.values.iter().map(|value| match view(value) {
        Some(link) => format!(
            "<a class=\"head\" href=\"{}\">{}</a>",
            esc(&link.href),
            ordered(&esc(value), link.ordered)
        ),
        None => format!("<span>{}</span>", esc(value)),
    });
    format!(
        "<div class=\"r head\">{}</div>\n",
        fixed.chain(valued).collect::<String>()
    )
}

/// A window's rows, each a line of the table at its index in the order,
/// made from the row's one line ([`search_cmd::Line`]), the CLI's row's
/// too: the name in its rarity's colour, a link to the item, its rarity
/// in words after it as the CLI's head joins it; its base; its price in
/// the card's words, so one that is open or absent says so; its location
/// and, where it left, when; the location's age; then each value's cell,
/// empty where the item lacks it — `stack`, a stack's count, among them
/// wherever the default list carries it. A cell is one line, cut where it
/// overflows.
pub(crate) fn window_rows(a: &Answer, now: i64) -> String {
    let after = match &a.view {
        ViewOut::Rows(rows) => rows.after,
        _ => 0,
    };
    let mut out = String::new();
    for (n, row) in a.rows.iter().enumerate() {
        let search_cmd::Line {
            name,
            rarity,
            // `show`'s, on the item's card
            frame: _,
            base,
            price,
            mut place,
            removed,
            fetched,
            values,
        } = search_cmd::Line::of(row, now);
        let item = match &rarity {
            Some(word) => format!("{name} · {word}"),
            None => name,
        };
        if let Some(removed) = removed {
            place.push_str(&format!(" · removed {removed}"));
        }
        let values: String = values
            .iter()
            .map(|cell| format!("<span>{}</span>", esc(cell)))
            .collect();
        out.push_str(&format!(
            "<div class=\"r\" id=\"{}\" data-i=\"{}\" data-id=\"{}\"><button type=\"submit\" class=\"name rarity-{}\" name=\"toggle_item\" value=\"{}\">{}</button><span>{}</span><span>{}</span><span>{}</span><span>{}</span>{values}</div>\n",
            esc(&row_anchor(&row.id)),
            after + n,
            esc(&row.id),
            esc(rarity.as_deref().unwrap_or("")),
            esc(&row.id),
            esc(&item),
            esc(base.as_deref().unwrap_or("")),
            esc(price.as_deref().unwrap_or("")),
            esc(&place),
            // an age's words where there is no time to age
            esc(&fetched.unwrap_or_else(|| acquisition_store::age::ago(now, None))),
        ));
    }
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

/// One item: `show`'s document rendered
/// as HTML — the item as a card ([`card`], the one the answer page
/// renders a row's item through, its name no link), then a fact table of
/// what the card leaves out — the id, the class, the category with what
/// it left open, the stack, the note, what is unread — then the lines
/// with their template, slots, rows and limits, the item against a query
/// where one was asked, and the basis, each fact in the text's words,
/// made once in `search_cmd.rs`. The price says why there is none, with
/// no command and no link.
pub(crate) fn item(shown: &Shown, placed: Option<&str>) -> String {
    let it = &shown.item;
    let now = acquisition_store::now();
    let mut out = card(
        ItemParts::of_shown(shown),
        &shown.place,
        it.icon.as_deref(),
        false,
        None,
        placed,
        now,
    );
    out.push_str("<table class=\"shown\">\n");
    out.push_str(&fact_row("Id", &esc(&it.facts.id)));
    out.push_str(&fact_row(
        "Class",
        &esc(&search_cmd::class_words(&shown.class)),
    ));
    let mut category = esc(&search_cmd::category_words(&shown.category));
    for open in &shown.category.open {
        category.push_str(&format!("<br>{}", esc(&search_cmd::open_words(open))));
    }
    out.push_str(&fact_row("Category", &category));
    if let Some(stack) = it.stack {
        out.push_str(&fact_row("Stack", &esc(&stack.to_string())));
    }
    if let Some(note) = &it.note {
        out.push_str(&fact_row("Note", &esc(note)));
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

/// The badges on an answer: a site search
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

/// The badges on one item: a class the table
/// could not decide, each part unread, each line whose flags are unknown
/// — each a state `show`'s document carries, which its text says below.
pub(crate) fn item_badges(parts: ItemParts<'_>) -> String {
    let mut out = Vec::new();
    match parts.class {
        Classed::Is(_) | Classed::All(_) => {}
        Classed::Open(why) | Classed::Among { why, .. } => {
            out.push(badge(&format!("class undecided — {}", why.problem)))
        }
        Classed::BaseUnread | Classed::FrameUnread { .. } | Classed::BestiaryUnread { .. } => {
            out.push(badge("class undecided"))
        }
    }
    for unread in &parts.item.unread {
        out.push(badge(&format!("unread — {}", unread.problem)));
    }
    for line in parts.lines {
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

/// One league the league control lists: its id, and what the store holds
/// of it in the realm shown — its live tabs and characters, the counts the
/// place pages read (`acquisition_verbs::LeaguesHeld`) — where it holds
/// either; none for a league the league list names and the store holds
/// nothing of.
pub(crate) struct League {
    pub id: String,
    pub held: Option<(i64, i64)>,
}

impl League {
    /// The league's option words: its name, then what is on disk of it.
    fn words(&self) -> String {
        match self.held {
            Some((tabs, characters)) => format!(
                "{} \u{2014} {}, {}",
                self.id,
                plural(usize::try_from(tabs).unwrap_or(0), "tab", "tabs"),
                plural(
                    usize::try_from(characters).unwrap_or(0),
                    "character",
                    "characters"
                )
            ),
            None => format!("{} \u{2014} nothing on disk", self.id),
        }
    }
}

/// The league a fresh form selects: the
/// one holding the most tabs, a tie going to the first in the control's
/// order; none where no league holds a tab.
pub(crate) fn fullest(leagues: &[League]) -> Option<&str> {
    let mut best: Option<(&str, i64)> = None;
    for league in leagues {
        let tabs = league.held.map_or(0, |(tabs, _)| tabs);
        if tabs > 0 && best.is_none_or(|(_, most)| tabs > most) {
            best = Some((league.id.as_str(), tabs));
        }
    }
    best.map(|(id, _)| id)
}

/// Which league the control shows: one the page names, the search's
/// "every league", or, on a fresh form (an address naming no league at
/// all), the fullest ([`fullest`]).
#[derive(Clone, Copy)]
pub(crate) enum Chosen<'a> {
    Named(&'a str),
    Every,
    Fresh,
}

/// The league control both pages carry: a select over the store's league
/// list, each league saying what the realm shown holds of it on disk, its
/// tabs and characters ([`League`]); with no list on disk, the leagues
/// the store holds places in. A page's own league the control does not
/// list stands first, selected and unmarked, and alone where there is
/// nothing else to list; a page that names no league and has none to
/// list has no select, the fetch button its only control. `any` adds the
/// search's "every league", which names no league term, last; a fresh
/// form selects the fullest league, or "every league" where no league
/// holds a tab.
pub(crate) fn league_select(leagues: Option<&[League]>, chosen: Chosen, any: bool) -> String {
    let selected = match chosen {
        Chosen::Named(own) => Some(own),
        Chosen::Every => None,
        Chosen::Fresh => leagues.and_then(fullest),
    };
    // no league to offer: the page's own alone, so the selects' form sends
    // it back across a realm that holds none; none where the page names
    // none either
    let leagues = match (leagues, selected) {
        (Some(leagues), _) => leagues,
        (None, Some(_)) => &[],
        (None, None) => return String::new(),
    };
    let mut out = String::from("<label class=\"control\">League <select name=\"league\">");
    // the page's own league, where the control does not list it, stands
    // first and selected, unmarked: a select showing another league than
    // the page would send that one with its next form
    if let Some(own) = selected.filter(|own| !leagues.iter().any(|l| l.id == *own)) {
        out.push_str(&format!(
            "<option value=\"{}\" selected>{}</option>",
            esc(own),
            esc(own)
        ));
    }
    for league in leagues {
        out.push_str(&format!(
            "<option value=\"{}\"{}>{}</option>",
            esc(&league.id),
            if selected == Some(league.id.as_str()) {
                " selected"
            } else {
                ""
            },
            esc(&league.words())
        ));
    }
    if any {
        out.push_str(&format!(
            "<option value=\"\"{}>every league</option>",
            if selected.is_none() { " selected" } else { "" }
        ));
    }
    out.push_str("</select></label> ");
    out
}

/// The button that fetches the league list: the `leagues` job, one
/// request, answered on the jobs page with the way back to `from`.
pub(crate) fn fetch_leagues(from: Option<&FromPage>) -> String {
    format!(
        "{}\n",
        post_form("leagues", from, [], "", "Fetch the league list")
    )
}

// ---- the place pages ---------------------------------------------------------------------------

/// What a place page renders: the places document, and what its URL
/// asks, opens and ticks.
pub(crate) struct PlacesPage<'a> {
    /// Its route: `tabs` or `characters`.
    pub listed: &'static str,
    /// The page itself as its leaving forms post it (`from`).
    pub onward: Option<&'a FromPage>,
    pub title: &'static str,
    /// The field a tick is sent as: `tab` or `character`.
    pub tick: &'static str,
    pub realm: Realm,
    pub league: &'a str,
    /// The realm's leagues, where the league list was fetched.
    pub leagues: Option<&'a [League]>,
    pub places: &'a Places,
    /// A characters page's sentence where the league holds none.
    pub none: Option<&'a str>,
    /// The places open, and the places ticked, as the URL names them.
    pub open: &'a [String],
    pub ticked: &'a [String],
    /// The substashes box, ticked.
    pub deep: bool,
    /// The filter the URL names.
    pub filter: &'a PlaceFilter,
    /// The query's text the URL names, for its box.
    pub text: Option<&'a str>,
    /// The view's fields the URL names, which the table's form carries
    /// hidden: no control writes them.
    pub view_fields: &'a [(&'static str, String)],
    /// The fields the realm and league selects' form carries: every
    /// field of the URL but the realm, the league, the ticks and the
    /// opens.
    pub carried: &'a [(&'static str, String)],
    /// A fold's never-fetched link: the page under `fetched=never`
    /// alone, every open and tick kept.
    pub never: &'a str,
    /// Each open place's items as a segment of the page's one window, by
    /// the place's id: its route and its first block, the server's.
    pub windows: &'a HashMap<String, Window>,
    /// A value's head: the page ordered by it, from a segment's rows.
    pub view_links: &'a dyn Fn(&RowsOut, &str) -> Option<ViewLink>,
    /// The opened rows the page keeps, which its list's form
    /// carries hidden, as it carries the open places.
    pub items: &'a [String],
    /// An opened row's card's place as a link.
    pub placed: Placed<'a>,
    /// Each of the query's `elsewhere` its link, in its order: the place
    /// page of that kind and league under the same query; none for the
    /// characters with no league.
    pub elsewhere: &'a [Option<String>],
    /// Whether the sync policy names the league: "Plan under the policy"
    /// shows only then.
    pub policy: bool,
    pub spell: Spell<'a>,
    pub now: i64,
}

/// A place's head's anchor: where a toggle's answer lands.
pub(crate) fn place_anchor(id: &str) -> String {
    format!("p-{}", enc(id))
}

/// A place page: the realm and league selects, a form of their own carrying
/// every field but the ticks and the opens (one id can stand in two
/// leagues); the league list's fetch and the listing beside it; the
/// league's counts in one line; then one GET form holding the list and its
/// buttons, "Show" first. The list is the page's one `.window`, inside the
/// form: each place's head in flow, its tick, its name a link to its items
/// as a table, its opener; an open place's places under it, or its items as
/// one segment of the window, the item columns' heads standing once above
/// the places where one does; each level's fold line after its rows. A
/// league with nothing listed says so where the list would be, with the
/// listing. No `pre` and no command line (#182).
pub(crate) fn places(view: &PlacesPage) -> String {
    let realms = ["pc", "xbox", "sony", "poe2"]
        .iter()
        .map(|r| {
            format!(
                "<option{}>{r}</option>",
                if *r == view.realm.as_str() {
                    " selected"
                } else {
                    ""
                }
            )
        })
        .collect::<String>();
    let listing = format!(
        "{}\n",
        post_form(
            view.listed,
            view.onward,
            [
                ("realm", view.realm.as_str().to_string()),
                ("league", view.league.to_string()),
            ],
            "",
            match view.listed {
                "tabs" => "List this league's stash tabs",
                _ => "List characters",
            },
        )
    );
    // a listing the realm's endpoints refuse is offered nowhere: a page
    // offers no request whose refusal it has no way on from (the landing
    // rule's census; `Family::accepts`, the one place that knows)
    let family = match view.listed {
        "tabs" => acquisition_protocol::realm::Family::Stashes,
        _ => acquisition_protocol::realm::Family::Characters,
    };
    let listing = match family.accepts(view.realm) {
        true => listing,
        false => String::new(),
    };
    // the listing stands beside the league list's fetch, or, where the
    // league has nothing listed, where the list would be
    let empty = view.places.places.is_empty();
    let mut body = format!(
        "<form method=\"get\" action=\"{}\"><label class=\"control\">Realm <select name=\"realm\">{realms}</select></label> {}{}<button type=\"submit\">Show</button></form>\n<div class=\"actions\">{}{}</div>\n",
        view.listed,
        league_select(view.leagues, Chosen::Named(view.league), false),
        hidden_fields(view.carried),
        fetch_leagues(view.onward),
        if empty { "" } else { listing.as_str() },
    );
    body.push_str(&format!(
        "<p class=\"counts\">{}</p>\n",
        esc(&counts_line(view))
    ));
    if empty {
        let said = match view.none {
            Some(said) => said.to_string(),
            None => format!("No stash tabs of {} are on record.", view.league),
        };
        // a league with none of this kind still says where the query's
        // matches stand
        body.push_str(&format!(
            "<p class=\"none\">{}</p>\n{}{listing}",
            esc(&said),
            matched_line(view)
        ));
        return body;
    }
    // the list's form: the open places, the view and the substashes box
    // travel through every open and close, as the ticks do
    body.push_str(&format!(
        "<form method=\"get\" action=\"{}\">\n<input type=\"hidden\" name=\"realm\" value=\"{}\"><input type=\"hidden\" name=\"league\" value=\"{}\">{}",
        view.listed,
        esc(view.realm.as_str()),
        esc(view.league),
        hidden_fields(view.view_fields),
    ));
    for open in view.open {
        body.push_str(&format!(
            "<input type=\"hidden\" name=\"open\" value=\"{}\">",
            esc(open)
        ));
    }
    for item in view.items {
        body.push_str(&format!(
            "<input type=\"hidden\" name=\"item\" value=\"{}\">",
            esc(item)
        ));
    }
    body.push('\n');
    // the query's box and the buttons above the list, so a long list
    // never hides them; "Show" the form's first
    // button, its default, so Enter in a box sends the page again with
    // every tick and open, never the plan; the plan's two buttons send
    // `refresh`, a field of this page, which answers it with the plan (the
    // landing rule: the place form never leaves)
    body.push_str(&format!(
        "<p><label class=\"control\">Items matching <input type=\"text\" name=\"text\" value=\"{}\" placeholder=\"a query, as the search reads it\" aria-label=\"show the places holding items this query matches\"></label> <button type=\"submit\">Show</button> <button type=\"submit\" name=\"refresh\" value=\"ticked\">Refresh ticked</button>{}</p>\n",
        esc(view.text.unwrap_or("")),
        if view.policy {
            " <button type=\"submit\" name=\"refresh\" value=\"policy\">Plan under the policy</button>"
        } else {
            ""
        }
    ));
    if view.listed == "tabs" {
        body.push_str(&format!(
            "<label class=\"control\"><input type=\"checkbox\" name=\"deep\" value=\"true\"{}> the substashes box: also fetch a ticked map or unique tab's pages</label>\n",
            if view.deep { " checked" } else { "" }
        ));
    }
    let mut hidden = String::new();
    let mut rows = String::new();
    place_rows(view, &view.places.places, "", 0, &mut rows, &mut hidden);
    rows.push_str(&fold_line(
        view,
        view.places.folded,
        &view.places.places,
        ("tab", "tabs"),
        0,
    ));
    let columns: &[&str] = match view.listed {
        "tabs" => &["Tab", "Type", "Items", "Fetched"],
        _ => &["Character", "Level", "Items", "Fetched"],
    };
    body.push_str(&hidden);
    // the item columns' heads, once above the places, where one lists its
    // items, "Add a column" beside the order they say
    let heads = heads_rows(view.places, view.windows)
        .map(|rows| {
            format!(
                "{}{}\n{}",
                order_way(rows, Presented::Places)
                    .map(|way| format!("<p class=\"rows-shown\">{}</p>\n", esc(&way)))
                    .unwrap_or_default(),
                value_finder(),
                item_heads(rows, &|value| (view.view_links)(rows, value))
            )
        })
        .unwrap_or_default();
    body.push_str(&format!(
        "<details class=\"list\" open><summary>{}</summary>\n{}<div class=\"p head\"><span></span>{}</div>\n{}<div class=\"window\" data-row-height=\"{ROW_HEIGHT}\" data-block-rows=\"{BLOCK_ROWS}\">\n{heads}{rows}</div>\n</details>\n</form>\n",
        // the list named, its counts the counts line's alone: a count of
        // the rows at the top would stand beside the league's, saying
        // another thing
        view.title,
        matched_line(view),
        columns
            .iter()
            .map(|c| format!("<span>{c}</span>"))
            .collect::<String>(),
        filter_row(view),
    ));
    body
}

/// The rows a place page's item heads are drawn from: the first open
/// place's first segment's, every segment's columns the same, since a
/// place's terms compare no number. None where no open place lists items:
/// no place open, an open place of no match, a folder alone open.
pub(crate) fn heads_rows<'a>(
    places: &'a Places,
    windows: &'a HashMap<String, Window>,
) -> Option<&'a RowsOut> {
    places
        .every()
        .into_iter()
        .find_map(|p| windows.get(p.id()))
        .and_then(|w| w.segments.first()?.blocks.first())
        .and_then(|(_, first)| match &first.view {
            ViewOut::Rows(rows) => Some(&**rows),
            _ => None,
        })
}

/// Under a query, the matches the league's places hold beside the
/// query's own, the query's own request a link to every match as a
/// table.
fn matched_line(view: &PlacesPage) -> String {
    let Some(query) = &view.places.query else {
        return String::new();
    };
    let mut table = query.request.clone();
    if let Shape::Rows(rows) = &mut table.view.shape {
        rows.fields = vec![RowField::Icon];
    }
    let here = match view.listed {
        "tabs" => "in these stash tabs",
        _ => "on these characters",
    };
    // where the query's other matches stand, each a link to its page
    let elsewhere: Vec<String> = query
        .elsewhere
        .iter()
        .zip(view.elsewhere.iter().chain(std::iter::repeat(&None)))
        .map(|(e, link)| {
            let words = format!(
                "{} {}",
                e.matched,
                match (e.place.as_str(), e.league.as_deref()) {
                    ("stash", Some(league)) => format!("in the stash tabs of {league}"),
                    (_, Some(league)) => format!("on the characters of {league}"),
                    (_, None) => "on characters with no league".to_string(),
                }
            );
            match link {
                Some(href) => format!("<a href=\"{}\">{}</a>", esc(href), esc(&words)),
                None => esc(&words),
            }
        })
        .collect();
    format!(
        "<p class=\"matched\">{} {here}, of {} · {}{}</p>\n",
        esc(&plural(query.matched, "matching item", "matching items")),
        esc(&plural(query.total.matched, "match", "matches")),
        way_on(
            view.spell,
            "/query/request",
            &Way::Search(Box::new(table)),
            "every match as a table",
            ""
        ),
        match elsewhere.is_empty() {
            true => String::new(),
            false => format!(" · elsewhere: {}", elsewhere.join(", ")),
        }
    )
}

/// Fields as hidden inputs, in order.
fn hidden_fields(fields: &[(&'static str, String)]) -> String {
    fields
        .iter()
        .map(|(k, v)| {
            format!(
                "<input type=\"hidden\" name=\"{}\" value=\"{}\">",
                k,
                esc(v)
            )
        })
        .collect()
}

/// A level's fold: one line after its rows, the
/// places it does not list counted, of them those never fetched a link to
/// the page under `fetched=never`, and the ticks among them, which travel
/// hidden. None where the level folds nothing.
fn fold_line(
    view: &PlacesPage,
    folded: Option<Holds>,
    places: &[Place],
    (one, many): (&str, &str),
    depth: usize,
) -> String {
    let Some(folded) = folded.filter(|f| f.places > 0) else {
        return String::new();
    };
    let (one, many) = match view.listed {
        "characters" => ("character", "characters"),
        _ => (one, many),
    };
    let ticked: usize = places
        .iter()
        .filter(|p| !p.is_listed())
        .map(|p| usize::from(view.ticked.iter().any(|t| t == p.id())) + ticked_in(view, p))
        .sum();
    format!(
        "<p class=\"fold depth-{depth}\">{}{}{}</p>\n",
        esc(&format!("{} not shown", plural(folded.places, one, many))),
        match folded.never_fetched {
            0 => String::new(),
            n => format!(", <a href=\"{}\">{n} never fetched</a>", esc(view.never)),
        },
        match ticked {
            0 => String::new(),
            n => format!("; {n} ticked"),
        },
    )
}

/// The ticks among the places `place` holds, at any depth.
fn ticked_in(view: &PlacesPage, place: &Place) -> usize {
    place
        .places
        .iter()
        .map(|p| usize::from(view.ticked.iter().any(|t| t == p.id())) + ticked_in(view, p))
        .sum()
}

/// The league's counts in one line: the terminal's head and facts line
/// for tabs, its footer for characters.
fn counts_line(view: &PlacesPage) -> String {
    match &view.places.counts {
        Counted::Tabs(counts) => {
            let facts = crate::store_cmd::tabs_facts_line(counts, view.now);
            if counts.tabs == 0 {
                return facts.trim_end().to_string();
            }
            format!(
                "{}; {}",
                crate::store_cmd::tabs_head(counts, view.realm, view.league, None),
                facts.trim_end()
            )
        }
        Counted::Characters(counts) => crate::store_cmd::characters_counts_line(counts)
            .trim_end()
            .to_string(),
    }
}

/// What a place shows when open: its places, or its items where it holds
/// none — under a query, its matches.
fn has_items(view: &PlacesPage, place: &Place) -> bool {
    place.request.is_some()
        && match (&view.places.query, place.matched) {
            (Some(_), matched) => matched.unwrap_or(0) > 0,
            (None, _) => place.item_count() > 0,
        }
}

/// The heads of the listed `places` at `depth`, in flow: each open
/// parent's places after its head in a group of their own (`.holds`),
/// a tab's own matches and its fold line after them; each open place
/// that holds items its segment. A tick under a closed
/// parent is a hidden field in `hidden`, counted in words on the
/// parent's head, and a tick on a place not listed one there too,
/// counted on its level's fold line.
fn place_rows(
    view: &PlacesPage,
    places: &[Place],
    at: &str,
    depth: usize,
    rows: &mut String,
    hidden: &mut String,
) {
    for (n, place) in places.iter().enumerate() {
        // the JSON pointer of the object holding its request (`data-way`)
        let here = format!("{at}/places/{n}");
        let id = place.id();
        let ticked = view.ticked.iter().any(|t| t == id);
        // a place the page does not list: its ticks and its places'
        // travel hidden, counted on its level's fold line
        if !place.is_listed() {
            if ticked {
                hidden.push_str(&hidden_fields(&[(view.tick, id.to_string())]));
            }
            ticked_under(view, place, hidden);
            continue;
        }
        let name = match place.name() {
            "" => "(unnamed)",
            name => name,
        };
        // a tick is named by its place, as the row reads, never by the id
        // it sends
        let tick = match place.place {
            Kind::Tab | Kind::Character => format!(
                "<input type=\"checkbox\" aria-label=\"{}\" name=\"{}\" value=\"{}\"{}>",
                esc(name),
                view.tick,
                esc(id),
                if ticked { " checked" } else { "" }
            ),
            // a page is fetched with its tab, under the substashes box
            Kind::Page => String::new(),
            // a folder holds tabs and no items: its tabs are ticked
            Kind::Folder => String::new(),
        };
        let named = match place.request.as_ref().map(as_table) {
            Some(table) => way_on(view.spell, &here, &Way::Search(Box::new(table)), name, ""),
            None => esc(name),
        };
        let open = view.open.iter().any(|o| o == id);
        let toggle = |open: bool| {
            format!(
                "<button type=\"submit\" name=\"toggle\" value=\"{}\">{}</button>",
                esc(id),
                if open { "Close" } else { "Open" }
            )
        };
        let opener = match place.holds {
            Some(holds) => {
                let inside = if open {
                    0
                } else {
                    ticked_under(view, place, hidden)
                };
                format!(
                    " <span class=\"holds-said\">{}{}{}</span> {}",
                    esc(&plural(
                        holds.places,
                        if place.place == Kind::Folder {
                            "tab"
                        } else {
                            "page"
                        },
                        if place.place == Kind::Folder {
                            "tabs"
                        } else {
                            "pages"
                        },
                    )),
                    match holds.never_fetched {
                        0 => String::new(),
                        n => format!(", {n} never fetched"),
                    },
                    match inside {
                        0 => String::new(),
                        n => format!("; {n} ticked inside"),
                    },
                    toggle(open)
                )
            }
            // a place that holds items opens to them
            None if has_items(view, place) => format!(" {}", toggle(open)),
            None => String::new(),
        };
        // under a query, a place's matches beside what it holds
        let items = |fetched_at: Option<i64>, count: i64| match (&view.places.query, place.matched)
        {
            (Some(_), Some(matched)) if place.places.is_empty() => format!(
                "{matched} of {}",
                crate::store_cmd::items_cell(fetched_at, count)
            ),
            (Some(_), Some(matched)) => format!("{matched} matching"),
            _ => crate::store_cmd::items_cell(fetched_at, count),
        };
        let cells = match &place.row {
            // a folder holds tabs and no items, and is never fetched: no
            // count and no age stand beside it, save its matches
            Row::Tab(tab) if place.place == Kind::Folder => [
                esc(&tab.r#type),
                match place.matched {
                    Some(matched) => esc(&format!("{matched} matching")),
                    None => String::new(),
                },
                String::new(),
            ],
            Row::Tab(tab) => [
                esc(&tab.r#type),
                esc(&items(tab.fetched_at, tab.item_count)),
                esc(&acquisition_store::age::ago(view.now, tab.fetched_at)),
            ],
            Row::Character(c) => [
                c.level.map(|l| l.to_string()).unwrap_or_default(),
                esc(&items(c.fetched_at, c.item_count)),
                esc(&acquisition_store::age::ago(view.now, c.fetched_at)),
            ],
        };
        rows.push_str(&format!(
            "<div class=\"p row depth-{depth}\" id=\"{}\"><span>{tick}</span><span>{named}{opener}</span><span>{}</span><span>{}</span><span>{}</span></div>\n",
            esc(&place_anchor(id)),
            cells[0],
            cells[1],
            cells[2],
        ));
        if !open {
            continue;
        }
        if !place.places.is_empty() {
            rows.push_str("<div class=\"holds\">\n");
            place_rows(view, &place.places, &here, depth + 1, rows, hidden);
            // a tab's own matches beside its pages': said apart, its
            // request (the tab's and its pages') as a table
            if let (Some(own), Some(request)) = (place.own.filter(|n| *n > 0), &place.request) {
                rows.push_str(&format!(
                    "<p class=\"own depth-{}\">{} · {}</p>\n",
                    depth + 1,
                    esc(&format!(
                        "{} in the tab itself",
                        plural(own, "matching item", "matching items")
                    )),
                    way_on(
                        view.spell,
                        &here,
                        &Way::Search(Box::new(as_table(request))),
                        "the tab's and its pages' as a table",
                        ""
                    )
                ));
            }
            let words = match place.place {
                Kind::Folder => ("tab", "tabs"),
                _ => ("page", "pages"),
            };
            rows.push_str(&fold_line(
                view,
                place.folded,
                &place.places,
                words,
                depth + 1,
            ));
            rows.push_str("</div>\n");
        } else if let Some(window) = view.windows.get(id) {
            // an open place's items, segments of the page's one window,
            // cut after each opened row
            let ways = CardWays {
                view: view.view_links,
                placed: view.placed,
                against: view.places.query.as_ref().map(|q| q.text.as_str()),
            };
            rows.push_str(&segments(window, &ways, view.now));
        }
    }
}

/// A place's request as a table: its view's rows, each carrying its
/// icon (`view.fields=icon`), the sort and the values kept.
fn as_table(request: &Request) -> Request {
    let mut table = request.clone();
    match &mut table.view.shape {
        Shape::Rows(rows) => rows.fields = vec![RowField::Icon],
        other => {
            *other = Shape::Rows(acquisition_search::answer::Rows {
                fields: vec![RowField::Icon],
                ..Default::default()
            })
        }
    }
    table
}

/// The ticks among the places `place` holds, at any depth, each written
/// to `hidden` as the field it travels as: how many there are.
fn ticked_under(view: &PlacesPage, place: &Place, hidden: &mut String) -> usize {
    let mut n = 0;
    for inner in &place.places {
        if view.ticked.iter().any(|t| t == inner.id()) {
            hidden.push_str(&format!(
                "<input type=\"hidden\" name=\"{}\" value=\"{}\">",
                view.tick,
                esc(inner.id())
            ));
            n += 1;
        }
        n += ticked_under(view, inner, hidden);
    }
    n
}

/// The filter's controls: the second head of the
/// list, each under the column it filters — words the name holds, a
/// tab's type, never fetched — sent with the list's form, so the URL
/// names them (C16) and "Show" or Enter asks the page again.
fn filter_row(view: &PlacesPage) -> String {
    let name = format!(
        "<span><input type=\"text\" name=\"name\" value=\"{}\" placeholder=\"words in the name\" aria-label=\"show the places whose name holds these words\"></span>",
        esc(view.filter.name.as_deref().unwrap_or(""))
    );
    let typed = match &view.places.counts {
        Counted::Tabs(counts) => {
            let wanted = view.filter.r#type.as_deref();
            let mut options = format!(
                "<option value=\"\"{}>any type</option>",
                if wanted.is_none() { " selected" } else { "" }
            );
            let mut held: Vec<&str> = counts.by_type.iter().map(|t| t.kind.as_str()).collect();
            // a type the URL names that the league holds none of still
            // shows, so the page says what it filtered by
            if let Some(wanted) = wanted
                && !held.iter().any(|t| t.eq_ignore_ascii_case(wanted))
            {
                held.push(wanted);
            }
            for t in held {
                options.push_str(&format!(
                    "<option{}>{}</option>",
                    if wanted.is_some_and(|w| w.eq_ignore_ascii_case(t)) {
                        " selected"
                    } else {
                        ""
                    },
                    esc(t)
                ));
            }
            format!(
                "<span><select name=\"type\" aria-label=\"show the tabs of one type\">{options}</select></span>"
            )
        }
        Counted::Characters(_) => "<span></span>".to_string(),
    };
    let never = format!(
        "<span><label><input type=\"checkbox\" name=\"fetched\" value=\"never\"{}> never</label></span>",
        if view.filter.fetched.is_some() {
            " checked"
        } else {
            ""
        }
    );
    format!("<div class=\"p filters\"><span></span>{name}{typed}<span></span>{never}</div>\n")
}

// ---- the plan and apply pages -------------------------------------------------------------------

/// A span in the store's one vocabulary (`3m`, `2d`).
fn span(seconds: i64) -> String {
    acquisition_store::age::span(seconds)
}

/// "Standard" on pc, "Standard (xbox)" elsewhere: pc is silent, as on
/// every place page.
fn league_on(realm: Realm, league: &str) -> String {
    match realm {
        Realm::Pc => league.to_string(),
        other => format!("{league} ({})", other.as_str()),
    }
}

/// A place's name as a page says it: `(unnamed)` for none.
fn named(name: &str) -> &str {
    if name.is_empty() { "(unnamed)" } else { name }
}

/// The plan page: the plan document in a
/// player's words — what is refreshed, how old its facts are, what the
/// compile noticed, each request as a row with why, what is left out,
/// and when the daemon says the requests could go — and no command line.
/// The envelope's Apply form follows, written by the route.
pub(crate) fn plan(planned: &acquisition_verbs::Planned, now: i64) -> String {
    use acquisition_verbs::refresh::PlanSpans;
    let plan = &planned.plan;
    let account = plan.account_name.as_deref().unwrap_or("this account");
    let place = league_on(plan.realm, &plan.league);
    let what = match (&plan.basis.policy_revision, &plan.basis.selection) {
        (Some(revision), _) => format!("everything your sync policy covers (revision {revision})"),
        (None, Some(selection)) if selection.deep => {
            "the places you ticked, with the pages of a ticked map or unique tab".to_string()
        }
        _ => "the places you ticked".to_string(),
    };
    let mut body = format!(
        "<p class=\"what\">Refresh {} as {}: {}.</p>\n",
        esc(&place),
        esc(account),
        esc(&what)
    );
    // the facts' ages, measured as the plan measures them
    let spans = PlanSpans::of(plan, now);
    let listed = |s: Option<i64>| match s {
        Some(s) => format!("{} before the plan", span(s)),
        None => "never".to_string(),
    };
    body.push_str("<table class=\"facts\">\n");
    body.push_str(&fact_row(
        "Stash tabs listed",
        &esc(&listed(spans.stash_listing)),
    ));
    body.push_str(&fact_row(
        "Characters listed",
        &esc(&listed(spans.character_listing)),
    ));
    body.push_str(&fact_row(
        "Plan made",
        &esc(&acquisition_store::age::span_ago(spans.plan)),
    ));
    body.push_str("</table>\n");
    for note in plan_notes(planned, now) {
        body.push_str(&format!("<p class=\"note\">{}</p>\n", esc(&note)));
    }
    if plan.actions.is_empty() {
        body.push_str(&format!(
            "<p class=\"none\">Nothing to fetch in {}.</p>\n",
            esc(&place)
        ));
    } else {
        body.push_str(&format!(
            "<p class=\"counts\">{}:</p>\n<table class=\"actions\">\n<tr><th>What</th><th>Why</th></tr>\n",
            esc(&plural(plan.actions.len(), "request", "requests"))
        ));
        for action in &plan.actions {
            let (what, why) = action_words(plan, action);
            body.push_str(&format!(
                "<tr><td>{}</td><td>{}</td></tr>\n",
                esc(&what),
                esc(&why)
            ));
        }
        body.push_str("</table>\n");
    }
    for left in left_out(planned) {
        body.push_str(&format!("<p class=\"skipped\">{}</p>\n", esc(&left)));
    }
    // the timing, where there is a request to time
    let timing = quote_words(planned, now);
    if !plan.actions.is_empty() && !timing.is_empty() {
        body.push_str(&format!("<p class=\"quote\">{}</p>\n", esc(&timing)));
    }
    if let Some(quote) = plan.quote.as_ref().filter(|q| !q.not_covered.is_empty()) {
        // the daemon's own sentences, folded: what it may send that no
        // estimate counts
        body.push_str(
            "<details class=\"not-covered\"><summary>What no estimate includes</summary>\n<ul>\n",
        );
        for words in &quote.not_covered {
            body.push_str(&format!("<li>{}</li>\n", esc(words)));
        }
        body.push_str("</ul>\n</details>\n");
    }
    body
}

/// What the compile noticed beside the plan, each a sentence: a league
/// the league list lacks, a league with no character listed, the
/// selection's listing fetched beside it, and a deep selection's map or
/// unique tab with no page on record.
fn plan_notes(planned: &acquisition_verbs::Planned, now: i64) -> Vec<String> {
    use acquisition_plan::{ListingReason, RefreshAction};
    use acquisition_verbs::refresh::PlanSpans;
    let plan = &planned.plan;
    let mut notes = Vec::new();
    if let Some(note) = &planned.notes.league {
        let place = league_on(note.realm, &note.league);
        notes.push(match note.list_fetched_at {
            None => {
                format!("The league list was never fetched, so {place} is not checked against it.")
            }
            Some(at) => format!(
                "{place} is not among the leagues fetched {}: check the league's name.",
                acquisition_store::age::ago(now, Some(at))
            ),
        });
    }
    if planned.notes.no_characters_listed {
        let when = PlanSpans::of(plan, now)
            .character_listing
            .map(|s| format!(", listed {} before the plan", span(s)))
            .unwrap_or_default();
        notes.push(format!(
            "No character is listed in {} (the realm's character list{when}).",
            league_on(plan.realm, &plan.league)
        ));
    }
    if plan.actions.iter().any(|a| {
        matches!(
            a,
            RefreshAction::ListStashes {
                reason: ListingReason::Selected,
                ..
            } | RefreshAction::ListCharacters {
                reason: ListingReason::Selected,
                ..
            }
        )
    }) {
        notes.push(
            "The list is fetched beside what you ticked: a place it newly lists is in the next plan."
                .to_string(),
        );
    }
    if plan.basis.selection.as_ref().is_some_and(|s| s.deep) {
        for action in &plan.actions {
            if let RefreshAction::FetchTab {
                id, name, tab_type, ..
            } = action
                && matches!(tab_type.as_str(), "MapStash" | "UniqueStash")
                && !plan.actions.iter().any(
                    |a| matches!(a, RefreshAction::FetchSubstash { parent, .. } if parent == id),
                )
            {
                notes.push(format!(
                    "Pages of {}: none on record to fetch. Its fetch lists any it has; one that lists none means it has none.",
                    named(name)
                ));
            }
        }
    }
    notes
}

/// One request of a plan as the page says it: what it fetches, and why.
fn action_words(
    plan: &acquisition_plan::RefreshPlan,
    action: &acquisition_plan::RefreshAction,
) -> (String, String) {
    use acquisition_plan::{ListingReason, RefreshAction};
    let listing = |reason: &ListingReason| match reason {
        ListingReason::NeverListed => "never listed".to_string(),
        ListingReason::Stale { age_seconds } => {
            format!("last listed {} before the plan", span(*age_seconds))
        }
        ListingReason::Selected => "what you ticked is read from it".to_string(),
    };
    let fetch = fetch_words;
    match action {
        RefreshAction::ListStashes { league, reason, .. } => (
            format!("The list of stash tabs in {league}"),
            listing(reason),
        ),
        RefreshAction::FetchTab { name, reason, .. } => {
            (format!("Tab {}", named(name)), fetch(reason))
        }
        RefreshAction::FetchSubstash {
            parent,
            name,
            reason,
            ..
        } => {
            // the parent's name where the plan holds it, else its id
            let parent = plan
                .actions
                .iter()
                .find_map(|a| match a {
                    RefreshAction::FetchTab { id, name, .. } if id == parent => {
                        Some(named(name).to_string())
                    }
                    _ => None,
                })
                .or_else(|| {
                    plan.skipped_tabs
                        .iter()
                        .find(|t| &t.id == parent)
                        .map(|t| named(&t.name).to_string())
                })
                .unwrap_or_else(|| parent.clone());
            (format!("Page {} of {parent}", named(name)), fetch(reason))
        }
        RefreshAction::ListCharacters { reason, .. } => {
            ("The list of characters".to_string(), listing(reason))
        }
        RefreshAction::FetchCharacter { name, reason, .. } => {
            (format!("Character {}", named(name)), fetch(reason))
        }
    }
}

/// Why a plan fetches a tab or a character, in words.
fn fetch_words(reason: &acquisition_plan::FetchReason) -> String {
    use acquisition_plan::FetchReason;
    match reason {
        FetchReason::NeverFetched => "never fetched".to_string(),
        FetchReason::Selected => "ticked".to_string(),
        FetchReason::Stale { age_seconds } => {
            format!("last fetched {} before the plan", span(*age_seconds))
        }
        FetchReason::ListedCountDisagrees { listed, held } => format!(
            "the list says it holds {}, and {held} are on record",
            plural(usize::try_from(*listed).unwrap_or(0), "item", "items")
        ),
        // the planner says unequal, in either direction
        FetchReason::ListedExperienceDisagrees { .. } => {
            "its experience on the list differs from its last fetch's".to_string()
        }
        FetchReason::ListedLeagueDisagrees { listed, held } => {
            format!("the list says it is in {listed}, its last fetch said {held}")
        }
    }
}

/// What the plan leaves out, counted by why: covered tabs and characters
/// it skips, places the policy names that are not on record, and the
/// league's tabs outside the policy.
fn left_out(planned: &acquisition_verbs::Planned) -> Vec<String> {
    use acquisition_plan::{CharacterSkipReason, SkipReason};
    let plan = &planned.plan;
    let window = span(i64::from(plan.max_age_seconds));
    let said = |n: usize, words: &str| (n > 0).then(|| format!("{n} {words}"));
    let mut out = Vec::new();
    if !plan.skipped_tabs.is_empty() {
        let count =
            |f: fn(&SkipReason) -> bool| plan.skipped_tabs.iter().filter(|t| f(&t.reason)).count();
        let parts: Vec<String> = [
            said(
                count(|r| matches!(r, SkipReason::Fresh { .. })),
                &format!("fetched within {window}"),
            ),
            said(
                count(|r| matches!(r, SkipReason::Folder)),
                "folders, which hold tabs and no items",
            ),
            said(
                count(|r| matches!(r, SkipReason::AwaitingListing)),
                "waiting for the list",
            ),
            said(
                count(|r| matches!(r, SkipReason::OrphanedParent { .. })),
                "under a tab no longer on record",
            ),
            said(count(|r| matches!(r, SkipReason::EmptyStub)), "empty pages"),
        ]
        .into_iter()
        .flatten()
        .collect();
        out.push(format!(
            "Not fetched: {} — {}.",
            plural(plan.skipped_tabs.len(), "tab", "tabs"),
            parts.join(", ")
        ));
    }
    if !plan.skipped_characters.is_empty() {
        let count = |f: fn(&CharacterSkipReason) -> bool| {
            plan.skipped_characters
                .iter()
                .filter(|c| f(&c.reason))
                .count()
        };
        let parts: Vec<String> = [
            said(
                count(|r| matches!(r, CharacterSkipReason::Fresh { .. })),
                &format!("fetched within {window}"),
            ),
            said(
                count(|r| matches!(r, CharacterSkipReason::AwaitingListing)),
                "waiting for the list",
            ),
            said(
                count(|r| matches!(r, CharacterSkipReason::NoLeague)),
                "in no league",
            ),
            said(
                count(|r| matches!(r, CharacterSkipReason::Deleted)),
                "deleted",
            ),
            said(
                count(|r| matches!(r, CharacterSkipReason::Expired)),
                "expired",
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        out.push(format!(
            "Not fetched: {} — {}.",
            plural(plan.skipped_characters.len(), "character", "characters"),
            parts.join(", ")
        ));
    }
    // what the policy names, or the reader ticked, that is not on record
    let unknown = |n: usize, one: &str, many: &str| match plan.basis.policy_revision {
        Some(_) => format!(
            "Your sync policy names {} not on record; nothing is fetched for {}.",
            plural(n, one, many),
            if n == 1 { "it" } else { "them" }
        ),
        None => format!(
            "{} you ticked {} not on record; nothing is fetched for {}.",
            plural(n, one, many),
            if n == 1 { "is" } else { "are" },
            if n == 1 { "it" } else { "them" }
        ),
    };
    if !plan.unknown_tabs.is_empty() {
        out.push(unknown(plan.unknown_tabs.len(), "tab", "tabs"));
    }
    if !plan.unknown_characters.is_empty() {
        out.push(unknown(
            plan.unknown_characters.len(),
            "character",
            "characters",
        ));
    }
    if let Some(u) = planned.uncovered_tabs.filter(|u| u.count > 0) {
        out.push(format!(
            "Outside your sync policy: {} of the league's {}.",
            u.count,
            plural(u.of, "tab", "tabs")
        ));
    }
    out
}

/// When the daemon says the plan's requests could go, or why it says
/// nothing. A scope's estimate is when its *last* request could go, so the
/// largest known one is the time for every request whose timing is known; a
/// scope with none is counted as not yet known while its route's policy is
/// unlearned, and as paused while its route is closed (a failed probe, or a
/// run of invalid answers, C190). Only a quote the daemon refused for want
/// of a session says "not signed in": no daemon, a timeout or another miss
/// observed no session and says nothing of one.
fn quote_words(planned: &acquisition_verbs::Planned, now: i64) -> String {
    use acquisition_verbs::QuoteNote;
    let Some(quote) = &planned.plan.quote else {
        return match &planned.quote_note {
            Some(QuoteNote::NotLoggedIn(_)) => {
                "Not signed in: the daemon holds no session for this account, so it says nothing of when these requests could go. Sign in before you apply.".to_string()
            }
            Some(QuoteNote::NoDaemon) => {
                "No daemon is running, so nothing says when these requests could go; Apply starts one.".to_string()
            }
            Some(QuoteNote::NoAccountName) => {
                "The facts record no account name, so the daemon was not asked when these requests could go.".to_string()
            }
            Some(QuoteNote::Timeout(_)) => {
                "The daemon did not answer in time, so nothing says when these requests could go.".to_string()
            }
            Some(QuoteNote::Rejected(_)) => {
                "The daemon's answer was about other work, so nothing says when these requests could go.".to_string()
            }
            Some(QuoteNote::Other(_)) => {
                "The daemon could not be asked, so nothing says when these requests could go.".to_string()
            }
            None => String::new(),
        };
    };
    let asked = acquisition_store::age::ago(now, Some(quote.observed_at));
    if quote.halted.is_some() {
        return format!(
            "The daemon is sending nothing (asked {asked}): none of these requests would go until a person clears its stop."
        );
    }
    if quote.scopes.is_empty() {
        return format!("None of these requests waits on GGG's pace (asked {asked}).");
    }
    let known: Vec<_> = quote
        .scopes
        .iter()
        .filter(|s| s.eta_seconds.is_some())
        .collect();
    let requests =
        |scopes: &mut dyn Iterator<Item = &acquisition_protocol::protocol::QuoteScope>| {
            usize::try_from(scopes.map(|s| s.requests).sum::<u64>()).unwrap_or(usize::MAX)
        };
    let timed = requests(&mut known.iter().copied());
    let untimed = |unlearned: bool| {
        requests(&mut quote.scopes.iter().filter(|s| {
            // The daemon's note says a closure in these words, a failed
            // probe's and a run's alike; a closed route may be unlearned
            // too, and its closure is what holds it.
            let closed = s.notes.iter().any(|note| note.contains(" closed "));
            s.eta_seconds.is_none() && closed != unlearned
        }))
    };
    let (unlearned, paused) = (untimed(true), untimed(false));
    let queued: u64 = quote.scopes.iter().map(|s| s.queued_ahead).sum();
    let mut said = Vec::new();
    if let Some(last) = known.iter().filter_map(|s| s.eta_seconds).max() {
        let when = if last == 0 {
            "now".to_string()
        } else {
            format!("within ~{}", span(i64::try_from(last).unwrap_or(i64::MAX)))
        };
        said.push(if timed == 1 {
            format!("The 1 request whose timing is known could go {when}")
        } else {
            format!("The {timed} requests whose timing is known could all go {when}")
        });
    }
    let lead = |said: &Vec<String>| {
        if said.is_empty() {
            "The timing of "
        } else {
            "the timing of "
        }
    };
    if unlearned > 0 {
        said.push(format!(
            "{}{} is not yet known: the daemon learns GGG's pace from its first answer",
            lead(&said),
            plural(unlearned, "request", "requests")
        ));
    }
    if paused > 0 {
        said.push(format!(
            "{}{} is not known: GGG's answers have paused {} kind of request, and the daemon tries again after a wait",
            lead(&said),
            plural(paused, "request", "requests"),
            if paused == 1 { "its" } else { "their" }
        ));
    }
    let mut words = format!("{} (asked {asked}).", said.join("; "));
    if queued > 0 {
        words.push_str(&format!(
            " {} already waiting go first.",
            plural(usize::try_from(queued).unwrap_or(usize::MAX), "job", "jobs")
        ));
    }
    words
}

/// The apply page: an apply that sent nothing, said from its
/// fields — a plan with nothing to fetch, or one whose every request is
/// spent — with a link to the plan compiled again. A submitted apply
/// lands at its job and has no page.
pub(crate) fn applied(applied: &acquisition_verbs::Apply, again: &str, now: i64) -> String {
    use acquisition_verbs::Apply;
    let skipped_words = |skipped: &acquisition_verbs::Skipped| {
        let mut parts = Vec::new();
        if skipped.tabs > 0 {
            parts.push(plural(skipped.tabs, "tab", "tabs"));
        }
        if skipped.characters > 0 {
            parts.push(plural(skipped.characters, "character", "characters"));
        }
        if parts.is_empty() {
            String::new()
        } else {
            format!(" It left out {}.", parts.join(" and "))
        }
    };
    let said = match applied {
        Apply::Nothing { skipped, .. } => format!(
            "Nothing was sent: this plan has nothing to fetch.{}",
            skipped_words(skipped)
        ),
        Apply::Spent {
            spent,
            skipped,
            remaining_requests,
            ..
        } => {
            let remaining = match remaining_requests {
                Some(0) => " A plan made now would fetch nothing.".to_string(),
                Some(n) => format!(
                    " A plan made now would fetch {}.",
                    plural(
                        usize::try_from(*n).unwrap_or(usize::MAX),
                        "request",
                        "requests"
                    )
                ),
                None => {
                    " What a plan made now would fetch is known once the waiting requests land."
                        .to_string()
                }
            };
            format!(
                "Nothing was sent: every request of this plan is spent — {}.{remaining}{}",
                acquisition_verbs::refresh::spent_phrase(spent, now),
                skipped_words(skipped)
            )
        }
        Apply::Submitted(_) => String::new(),
    };
    format!(
        "<p class=\"applied\">{}</p>\n<p><a href=\"{}\">The plan made again</a></p>\n",
        esc(&said),
        esc(again)
    )
}

// ---- the jobs page ------------------------------------------------------------------------------

/// What the jobs page renders: the jobs document, the daemon's halts
/// where one answers, and the job the URL opens; its way back, the
/// `from` its URL carries, is the page's ([`Back`]).
pub(crate) struct JobsPage<'a> {
    pub jobs: &'a acquisition_verbs::Jobs,
    /// The daemon's rails, where a daemon answers.
    pub rails: Option<&'a acquisition_protocol::status::RailsStatus>,
    /// The job the URL names (`job`), where it does.
    pub opened: Option<&'a OpenedJob>,
    /// The account the page serves: a job of another says whose it is.
    pub account: &'a str,
    pub spell: Spell<'a>,
    pub now: i64,
}

/// A job the URL opens (`jobs?job=<id>`): read by id, listed or not,
/// with its result where it failed or was cancelled.
pub(crate) struct OpenedJob {
    pub id: u64,
    /// The job read by id, where the list leaves it out.
    pub job: Option<acquisition_verbs::jobs::Job>,
    pub result: Option<acquisition_verbs::jobs::JobResult>,
}

/// A job's row's anchor: where a submit lands.
pub(crate) fn job_anchor(id: u64) -> String {
    format!("j-{id}")
}

/// One job as a row says it, from the daemon's reading or the ledger's.
struct JobRow<'a> {
    id: u64,
    parent: Option<u64>,
    kind: &'a str,
    params: &'a Value,
    /// The job's account where it is not the page's: the list spans
    /// accounts.
    whose: Option<&'a str>,
    state: String,
    when: String,
}

impl<'a> JobRow<'a> {
    fn live(job: &'a acquisition_protocol::job::JobInfo, served: &str) -> JobRow<'a> {
        use acquisition_protocol::job::JobState;
        let age = |words: &Option<String>| words.clone().unwrap_or_default();
        let (state, when) = match job.state {
            JobState::Waiting => {
                let start = match job.eta_seconds {
                    Some(0) => "starts next".to_string(),
                    Some(s) => format!("starts in ~{}", span(i64::try_from(s).unwrap_or(i64::MAX))),
                    None => "its start is not yet known".to_string(),
                };
                (
                    "waiting".to_string(),
                    format!("submitted {}; {start}", age(&job.times.submitted_at_age)),
                )
            }
            JobState::Running => (
                "running".to_string(),
                format!("started {}", age(&job.times.started_at_age)),
            ),
            JobState::Done => (
                "done".to_string(),
                format!("finished {}", age(&job.times.finished_at_age)),
            ),
            JobState::Failed => (
                "failed".to_string(),
                format!("finished {}", age(&job.times.finished_at_age)),
            ),
            JobState::Cancelled => (
                "cancelled".to_string(),
                format!("finished {}", age(&job.times.finished_at_age)),
            ),
        };
        JobRow {
            id: job.id,
            parent: job.parent,
            kind: &job.kind,
            params: &job.params,
            whose: job.account.as_deref().filter(|a| *a != served),
            state,
            when: when.trim_end().to_string(),
        }
    }

    fn recorded(job: &'a acquisition_store::jobs::PersistedJob, served: &str) -> JobRow<'a> {
        let state = match job.state.as_str() {
            "running" => "running when the daemon stopped".to_string(),
            other => other.to_string(),
        };
        JobRow {
            id: job.id,
            parent: job.parent,
            kind: &job.kind,
            params: &job.params,
            whose: job.account.as_deref().filter(|a| *a != served),
            state,
            when: format!(
                "submitted {}",
                job.submitted_at_age.clone().unwrap_or_default()
            )
            .trim_end()
            .to_string(),
        }
    }
}

/// What a job does, in words, from its kind and params: no daemon word,
/// no route, no channel.
fn job_what(kind: &str, params: &Value) -> String {
    let s = |k: &str| params.get(k).and_then(Value::as_str).unwrap_or("");
    let realm = match s("realm") {
        "" | "pc" => String::new(),
        other => format!(" ({other})"),
    };
    match kind {
        "profile" => "The account's profile".to_string(),
        "leagues" => format!("The league list{realm}"),
        "characters" => format!("The list of characters{realm}"),
        "character" => format!("Character {}{realm}", s("name")),
        "stashes" => format!("The list of stash tabs in {}{realm}", s("league")),
        "stash" => match params.get("sub").and_then(Value::as_str) {
            Some(sub) => format!("Page {sub} of tab {} in {}{realm}", s("id"), s("league")),
            None => format!("Tab {} in {}{realm}", s("id"), s("league")),
        },
        "apply" => format!(
            "A refresh of {}",
            plural(
                params
                    .get("jobs")
                    .and_then(Value::as_array)
                    .map_or(0, Vec::len),
                "request",
                "requests"
            )
        ),
        "probe" => "A first check of a route before its requests".to_string(),
        "sleep" => "A wait (the test world's)".to_string(),
        "fetch" => "A test fetch (the test world's)".to_string(),
        "whoami" => "An account check (the test world's)".to_string(),
        other => other.to_string(),
    }
}

/// A failed child as its parent's result names it: its kind and target,
/// the only facts the result carries of it.
fn child_what(kind: &str, target: &str, id: u64) -> String {
    let noun = match kind {
        "" => return format!("Job {id}"),
        "stash" => "Tab",
        "stashes" => "The list of stash tabs in",
        "character" => "Character",
        "characters" => "The list of characters",
        "leagues" => "The league list",
        "profile" => "The account's profile",
        "apply" => "A refresh",
        other => other,
    };
    format!("{noun} {target}").trim_end().to_string()
}

/// A failure's kind in the page's own words (C165): never
/// the daemon's `error` nor the way on's words, which can name a
/// command; a failure with no kind says only that it failed.
fn failure_words(kind: Option<acquisition_protocol::job::FailureKind>) -> &'static str {
    use acquisition_protocol::job::FailureKind;
    match kind {
        None => "It failed.",
        Some(FailureKind::NotFound) => {
            "GGG has nothing there now; fetching again what lists it shows what is there."
        }
        Some(FailureKind::Upstream) => {
            "GGG's answer could not be used. The same request may work later."
        }
        Some(FailureKind::RateLimited) => {
            "GGG asked to slow down until the job ran out of tries. The same request can go again once that wait has passed."
        }
        Some(FailureKind::Blocked) => {
            "Cloudflare answered in GGG's place. Nothing is sent again, this or anything like it, until a person has looked and cleared it."
        }
        Some(FailureKind::NotRetried) => {
            "GGG refused it, and a refusal like this is never tried again: a person looks first. One that names the sign-in asks for signing in again."
        }
        Some(FailureKind::NotRecorded) => {
            "It was fetched, and what it fetched could not be saved; it is kept with this result."
        }
        Some(FailureKind::Children) => {
            "Some of its requests failed; each says why in its own result."
        }
        Some(FailureKind::NotLoggedIn) => {
            "Its account had no session to send with. Signing in again is a person's to do."
        }
        Some(FailureKind::Interrupted) => {
            "The daemon stopped before it finished. The same request can go again."
        }
        Some(FailureKind::Internal) => "The daemon failed on its own account; nothing is offered.",
    }
}

/// A failed or cancelled job's result under its row: its
/// failure by kind, its children's tally, the first failed children
/// each a link to its own result, and the offers its page takes.
fn result_words(result: &acquisition_verbs::jobs::JobResult, spell: Spell) -> String {
    // an offer of this result is the page it stands on
    let shown = spell(&Way::Result { id: result.job_id });
    use acquisition_protocol::job::{FailureKind, Outcome};
    let mut out = String::new();
    match &result.outcome {
        Outcome::Cancelled => out.push_str("<p>It was cancelled.</p>\n"),
        Outcome::Success { .. } => out.push_str("<p>It finished.</p>\n"),
        Outcome::Failure { kind, children, .. } => {
            out.push_str(&format!("<p>{}</p>\n", esc(failure_words(*kind))));
            if let Some(c) = children {
                let mut parts = vec![format!("{} failed", c.failed)];
                if c.cancelled > 0 {
                    parts.push(format!("{} cancelled", c.cancelled));
                }
                parts.push(format!("{} done", c.done));
                out.push_str(&format!(
                    "<p>Of its requests: {}.</p>\n",
                    esc(&parts.join(", "))
                ));
            }
            if !result.failed_children.is_empty() {
                out.push_str("<ul class=\"failed\">\n");
                for child in &result.failed_children {
                    let what = child_what(&child.kind, &child.target, child.job_id);
                    out.push_str(&format!(
                        "<li>{} failed</li>\n",
                        result_link(spell, child.job_id, &what)
                    ));
                }
                out.push_str("</ul>\n");
            }
            // a parent's offers are its children's results, linked above
            if *kind != Some(FailureKind::Children) {
                let links: Vec<String> = result
                    .offers
                    .iter()
                    .filter(|offer| {
                        offer.get("request").and_then(Value::as_str) != shown.as_deref()
                    })
                    .filter_map(|offer| {
                        Some(format!(
                            "<li><a href=\"{}\">{}</a></li>",
                            esc(offer.get("request")?.as_str()?),
                            esc(offer.get("says")?.as_str()?)
                        ))
                    })
                    .collect();
                if !links.is_empty() {
                    out.push_str(&format!(
                        "<ul class=\"offers\">\n{}\n</ul>\n",
                        links.join("\n")
                    ));
                }
            }
        }
    }
    out
}

/// The jobs page: the daemon's halts in prose, the job the
/// URL opens where the list leaves it out, the counts, and the listed
/// jobs as a table, a child under its listed parent; with no daemon, the
/// ledger's rows, how old, and that the next job resumes them. No `pre`,
/// no command line, no channel, no priority.
pub(crate) fn jobs(view: &JobsPage) -> String {
    use acquisition_verbs::Jobs;
    let mut body = String::new();
    if let Some(rails) = view.rails {
        body.push_str(&rails_words(rails, view.now));
    }
    let (rows, counts, left): (Vec<JobRow>, String, Vec<String>) = match view.jobs {
        Jobs::Live(listed) => (
            listed
                .rows
                .iter()
                .map(|j| JobRow::live(j, view.account))
                .collect(),
            live_counts(listed),
            [
                (listed.open_left_out, "earlier running or waiting"),
                (listed.finished_left_out, "earlier finished"),
            ]
            .iter()
            .filter(|(n, _)| *n > 0)
            .map(|(n, what)| {
                format!(
                    "{n} {what} {} not listed.",
                    if *n == 1 { "job is" } else { "jobs are" }
                )
            })
            .collect(),
        ),
        Jobs::Absent { persisted: None } => (
            Vec::new(),
            "No daemon is running, and no job is on record.".to_string(),
            Vec::new(),
        ),
        Jobs::Absent { persisted: Some(p) } => (
            p.jobs
                .iter()
                .map(|j| JobRow::recorded(j, view.account))
                .collect(),
            format!(
                "No daemon is running. The queue on record, written {}: {}, {} waiting, {} running when the daemon stopped. The next job submitted starts a daemon, which resumes them.",
                p.written_at_age,
                plural(p.total, "job", "jobs"),
                p.waiting,
                p.recorded_running
            ),
            (p.left_out > 0)
                .then(|| {
                    format!(
                        "{} more on record {} not listed.",
                        p.left_out,
                        if p.left_out == 1 { "is" } else { "are" }
                    )
                })
                .into_iter()
                .collect(),
        ),
    };
    // the job the URL opens, where the list leaves it out, above the list
    let opened = view.opened;
    let listed = |id: u64| rows.iter().any(|r| r.id == id);
    if let Some(open) = opened.filter(|o| !listed(o.id)) {
        let row = match &open.job {
            Some(acquisition_verbs::jobs::Job::Live(job)) => Some(JobRow::live(job, view.account)),
            Some(acquisition_verbs::jobs::Job::Absent { persisted, .. }) => {
                Some(JobRow::recorded(persisted, view.account))
            }
            None => None,
        };
        if let Some(row) = row {
            body.push_str("<p class=\"opened\">This job is not among those listed below.</p>\n");
            body.push_str(JOBS_HEAD);
            body.push_str(&job_row(&row, 0, true, open.result.as_ref(), view.spell));
            body.push_str("</table>\n");
        }
    }
    body.push_str(&format!("<p class=\"counts\">{}</p>\n", esc(&counts)));
    if !rows.is_empty() {
        body.push_str(JOBS_HEAD);
        // a child under its parent where both are listed
        let roots: Vec<&JobRow> = rows
            .iter()
            .filter(|r| r.parent.is_none_or(|p| !listed(p)))
            .collect();
        for root in roots {
            job_tree(&rows, root, 0, opened, view.spell, &mut body);
        }
        body.push_str("</table>\n");
    }
    for words in left {
        body.push_str(&format!("<p class=\"left-out\">{}</p>\n", esc(&words)));
    }
    body
}

const JOBS_HEAD: &str =
    "<table class=\"jobs\">\n<tr><th>What</th><th>State</th><th>When</th></tr>\n";

/// `row` and its listed children, each under its parent.
fn job_tree(
    rows: &[JobRow],
    row: &JobRow,
    depth: usize,
    opened: Option<&OpenedJob>,
    spell: Spell,
    body: &mut String,
) {
    let open = opened.filter(|o| o.id == row.id);
    body.push_str(&job_row(
        row,
        depth,
        open.is_some(),
        open.and_then(|o| o.result.as_ref()),
        spell,
    ));
    for child in rows.iter().filter(|r| r.parent == Some(row.id)) {
        job_tree(rows, child, depth + 1, opened, spell, body);
    }
}

/// One job's row, marked where the URL opens it, its result under it.
fn job_row(
    row: &JobRow,
    depth: usize,
    opened: bool,
    result: Option<&acquisition_verbs::jobs::JobResult>,
    spell: Spell,
) -> String {
    let class = match (depth > 0, opened) {
        (false, false) => "job",
        (true, false) => "job child",
        (false, true) => "job opened",
        (true, true) => "job child opened",
    };
    // a finished job's own result is a link where it is not opened
    let mut what = job_what(row.kind, row.params);
    if let Some(whose) = row.whose {
        what.push_str(&format!(", for {whose}"));
    }
    let what = if opened || !matches!(row.state.as_str(), "failed" | "cancelled") {
        esc(&what)
    } else {
        result_link(spell, row.id, &what)
    };
    let mut out = format!(
        "<tr class=\"{class}\" id=\"{}\"><td>{}{what}</td><td>{}</td><td>{}</td></tr>\n",
        job_anchor(row.id),
        "\u{21b3} ".repeat(depth),
        esc(&row.state),
        esc(&row.when)
    );
    if let Some(result) = result {
        out.push_str(&format!(
            "<tr class=\"result\"><td colspan=\"3\">\n{}</td></tr>\n",
            result_words(result, spell)
        ));
    }
    out
}

/// A job's words as a link to its result on this page, where the page
/// spells one; the words alone where it does not.
fn result_link(spell: Spell, id: u64, words: &str) -> String {
    match spell(&Way::Result { id }) {
        Some(link) => format!("<a href=\"{}\">{}</a>", esc(&link), esc(words)),
        None => esc(words),
    }
}

/// The live list's counts in one sentence: every job by state, and when
/// the waiting ones start.
fn live_counts(listed: &acquisition_verbs::jobs::Listed) -> String {
    let by = &listed.counts.by_state;
    let mut parts = vec![
        format!("{} running", by.running),
        format!("{} waiting", by.waiting),
        format!("{} done", by.done),
    ];
    if by.failed > 0 {
        parts.push(format!("{} failed", by.failed));
    }
    if by.cancelled > 0 {
        parts.push(format!("{} cancelled", by.cancelled));
    }
    let mut out = format!(
        "{}: {}.",
        plural(listed.counts.total, "job", "jobs"),
        parts.join(", ")
    );
    let at = |s: u64| span(i64::try_from(s).unwrap_or(i64::MAX));
    match (
        listed.counts.next_starts_in_seconds,
        listed.counts.last_starts_in_seconds,
    ) {
        (Some(next), Some(last)) if last > next => out.push_str(&format!(
            " The next waiting job starts in ~{}, the last in ~{}.",
            at(next),
            at(last)
        )),
        (Some(next), _) => out.push_str(&format!(" The next waiting job starts in ~{}.", at(next))),
        _ => {}
    }
    out
}

/// The daemon's halts in prose: a halt, each closed route,
/// and a session it cannot renew; nothing of its tripwire, its sends or
/// its journal.
fn rails_words(rails: &acquisition_protocol::status::RailsStatus, now: i64) -> String {
    let mut out = String::new();
    if let Some(cause) = &rails.halted {
        out.push_str(&format!(
            "<p class=\"halt\">The daemon is sending nothing: {}. A person clears this stop; until then no job here starts.</p>\n",
            esc(cause)
        ));
    }
    for route in &rails.closed_routes {
        let (what, account) = match route.route.split_once('@') {
            Some((what, account)) => (what, Some(account)),
            None => (route.route.as_str(), None),
        };
        let what = match what {
            "stash" => "Fetching tabs",
            "stash-list" => "Listing stash tabs",
            "character" => "Fetching characters",
            "character-list" => "Listing characters",
            "league" | "leagues" => "Fetching the league list",
            "profile" => "Fetching the profile",
            _ => "A kind of request",
        };
        let trial = if route.until > now {
            format!(
                "One waiting job tries again in ~{}",
                span(route.until - now)
            )
        } else {
            "The next job of it tries again".to_string()
        };
        let cap = if route.cooldown_secs >= 3600 {
            " It waits its longest now, an hour between tries; one that keeps failing may mean GGG's API changed, and a newer Acquisition may know it."
        } else {
            ""
        };
        out.push_str(&format!(
            "<p class=\"halt\">{what}{} is paused: GGG answered {} in a row as invalid, the last with {}. {trial}; until then every such request waits, whatever its league. Once it reopens, list the league again: what was fetched just before may be older than GGG's.{cap}</p>\n",
            esc(&account.map(|a| format!(" for {a}")).unwrap_or_default()),
            plural(usize::try_from(route.invalid).unwrap_or(usize::MAX), "request", "requests"),
            route.status,
        ));
    }
    if let Some(failed) = &rails.refresh_failed {
        out.push_str(&format!(
            "<p class=\"halt\">The daemon cannot renew a session ({}). Signing in again is a person's to do.</p>\n",
            esc(failed)
        ));
    }
    out
}

// ---- the language (`describe`) ------------------------------------------------------------------

/// The words of the link from a page that is not the index back to it.
const THE_INDEX: &str = "The language's index: every entry by its name and one line";

/// The names box's label: what a reader may name in it.
const NAMES_BOX: &str = "Names: an entry, a block, ~ and a pattern, or all";

/// `describe`'s document as a page (`4-full-1-form-index.md`, Decision 3):
/// the search crate's words, under the heads of the one table both
/// renderers read ([`search_cmd::BLOCKS`]), the document destructured
/// whole, so a field the search crate adds does not compile until it is
/// rendered here. An entry shows its name, its kind, what it is, its values
/// and its examples, which are text; each entry's name, each category's id
/// and each block's head is a link to that name described, made by `link`,
/// which hands on the page's `from` (`None` the index); a page that is not the index links
/// back to it. What `next` tells a reader to do is spelled as those links
/// and the names box, a GET form to this route carrying `from`. A view's
/// entry is named by the request's word: no flag is spelled here.
pub(crate) fn describe(
    d: &acquisition_search::Describe,
    index: bool,
    totals: bool,
    link: &dyn Fn(Option<&str>) -> String,
    from: Option<&FromPage>,
) -> String {
    use search_cmd::{BLOCKS, Block, Part};
    let acquisition_search::Describe {
        patterns,
        fields,
        line,
        linked,
        values,
        composition,
        operators,
        slots,
        computed,
        totals: table,
        counts,
        explain,
        answer,
        categories,
        trade,
        trade_rows,
        not_built,
        limits,
        next,
    } = d;
    let named = |name: &str, words: &str| {
        format!("<a href=\"{}\">{}</a>", esc(&link(Some(name))), esc(words))
    };
    let mut out = String::from("<div class=\"describe\">\n");
    for m in patterns {
        out.push_str(&format!(
            "<p class=\"pattern\">{}</p>\n",
            esc(&search_cmd::pattern_said(m))
        ));
    }
    if let Some(next) = next {
        out.push_str(&format!("<p class=\"next\">{}</p>\n", esc(next)));
    }
    let hidden = from
        .map(|from| hidden(FROM, &from.word()))
        .unwrap_or_default();
    out.push_str(&format!(
        "<form class=\"names\" method=\"get\" action=\"describe\">{hidden}<label class=\"control\">{} <input type=\"text\" name=\"names\" aria-label=\"{}\"></label> <button type=\"submit\">Describe</button></form>\n",
        esc(NAMES_BOX),
        esc(NAMES_BOX)
    ));
    if !index {
        out.push_str(&format!(
            "<p class=\"index\"><a href=\"{}\">{}</a></p>\n",
            esc(&link(None)),
            esc(THE_INDEX)
        ));
    }
    let head = |name: &str, head: &str, after: Option<&str>| {
        format!(
            "<p class=\"head\">{}{}</p>\n",
            named(name, head),
            after
                .map(|after| esc(&format!(" — {after}")))
                .unwrap_or_default()
        )
    };
    let entries = |out: &mut String,
                   block: &Block,
                   after: Option<&str>,
                   list: &[acquisition_search::describe::Named]| {
        if list.is_empty() {
            return;
        }
        out.push_str(&format!(
            "<section class=\"block\">\n{}",
            head(block.name, block.head, after)
        ));
        out.push_str(
            "<table class=\"entries\">\n<tr><th>Name</th><th>Kind</th><th>What it is</th></tr>\n",
        );
        for e in list {
            let mut what = esc(&e.what);
            if !e.values.is_empty() {
                what.push_str(&format!("<br>one of: {}", esc(&e.values.join(", "))));
            }
            if !e.examples.is_empty() {
                let examples: Vec<String> = e
                    .examples
                    .iter()
                    .map(|x| format!("<code>{}</code>", esc(x)))
                    .collect();
                what.push_str(&format!("<br>e.g. {}", examples.join(" ")));
            }
            out.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{what}</td></tr>\n",
                named(&e.name, &e.name),
                esc(e.kind)
            ));
        }
        out.push_str("</table>\n</section>\n");
    };
    for block in &BLOCKS {
        match block.part {
            Part::Fields => entries(&mut out, block, None, fields),
            Part::Line => entries(&mut out, block, None, line),
            Part::Linked => entries(&mut out, block, None, linked),
            Part::Values => entries(&mut out, block, None, values),
            Part::Composition => entries(&mut out, block, None, composition),
            Part::Operators => entries(&mut out, block, None, operators),
            Part::Slots => entries(&mut out, block, None, slots),
            // the totals' table is named once, over its block (C106's
            // clause (c))
            Part::Computed => entries(&mut out, block, Some(table), computed),
            Part::Counts => entries(&mut out, block, None, counts),
            Part::Explain => entries(&mut out, block, None, explain),
            Part::Answer => entries(&mut out, block, None, answer),
            Part::Totals => {
                if totals && computed.is_empty() {
                    out.push_str(&format!(
                        "<section class=\"block\">\n{}</section>\n",
                        head(block.name, block.head, Some(table))
                    ));
                }
            }
            Part::Categories => {
                if categories.is_empty() {
                    continue;
                }
                out.push_str(&format!(
                    "<section class=\"block\">\n{}<table class=\"entries\">\n<tr><th>Id</th><th>The site's text</th><th>What makes it hold</th></tr>\n",
                    head(block.name, block.head, None)
                ));
                for c in categories {
                    out.push_str(&format!(
                        "<tr><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                        named(c.id, c.id),
                        esc(&c.text),
                        esc(&c.holds)
                    ));
                }
                out.push_str("</table>\n</section>\n");
            }
            Part::Trade => {
                // the index names the catalogue by its rows, a link to it
                if let Some(rows) = trade_rows {
                    out.push_str(&format!(
                        "<section class=\"block\">\n{}</section>\n",
                        head(
                            block.name,
                            block.head,
                            Some(&format!(
                                "each clause a site search can carry, {}",
                                plural(*rows, "row", "rows")
                            ))
                        )
                    ));
                }
                let Some(trade) = trade else {
                    continue;
                };
                out.push_str(&format!(
                    "<section class=\"block\">\n{}",
                    head(block.name, block.head, Some(&search_cmd::trade_said(trade)))
                ));
                out.push_str(
                    "<table class=\"entries\">\n<tr><th>Key</th><th>What it says</th></tr>\n",
                );
                for e in &trade.report {
                    out.push_str(&format!(
                        "<tr><td>{}</td><td>{}</td></tr>\n",
                        esc(&e.name),
                        esc(&e.what)
                    ));
                }
                out.push_str("</table>\n<table class=\"entries\">\n<tr><th>Clause</th><th>Status</th><th>Lands as</th><th>The text it becomes</th></tr>\n");
                for r in &trade.rows {
                    let mut said: Vec<String> =
                        r.text.iter().chain(&r.says).map(|s| esc(s)).collect();
                    if let Some(other) = &r.other {
                        said.push(esc(&format!("read the other way: {other}")));
                    }
                    out.push_str(&format!(
                        "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                        esc(&r.key),
                        esc(r.status),
                        esc(r.lands.as_deref().unwrap_or("—")),
                        said.join("<br>")
                    ));
                }
                out.push_str("</table>\n</section>\n");
            }
            Part::NotBuilt => {
                if not_built.is_empty() {
                    continue;
                }
                out.push_str(&format!(
                    "<section class=\"block\">\n{}<table class=\"entries\">\n<tr><th>Name</th><th>When</th><th>What it would do</th></tr>\n",
                    head(block.name, block.head, None)
                ));
                for n in not_built {
                    out.push_str(&format!(
                        "<tr><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                        esc(n.construct),
                        esc(&n.step_text()),
                        esc(n.what)
                    ));
                }
                out.push_str("</table>\n</section>\n");
            }
            Part::Limits => {
                if limits.is_empty() {
                    continue;
                }
                out.push_str(&format!(
                    "<section class=\"block\">\n{}<ul>\n",
                    head(block.name, block.head, None)
                ));
                for l in limits {
                    out.push_str(&format!("<li>{}</li>\n", esc(l.said)));
                }
                out.push_str("</ul>\n</section>\n");
            }
        }
    }
    out.push_str("</div>\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What a group's type shows is the stylesheet's (`4-full-1-form-rows.md`,
    /// Decision 5): each positional field some group type does not take, by
    /// the translation's two key lists, has its one hide rule, over the
    /// group's checked option; no other field has one; and neither the
    /// stylesheet nor a script names a group type.
    #[test]
    fn each_field_a_type_does_not_take_has_its_hide_rule_and_no_type_is_named() {
        let css = include_str!("assets/page.css");
        let form = Form::load().unwrap();
        let every: std::collections::BTreeSet<String> = form
            .stat_types
            .iter()
            .flat_map(|(k, _, _)| type_takes(k))
            .chain(
                super::super::form::GROUP_FIELDS
                    .iter()
                    .chain(super::super::form::ROW_FIELDS.iter())
                    .filter(|f| !f.ends_with(".type") && !f.ends_with(".id"))
                    .filter(|f| !f.ends_with(".disabled"))
                    .map(|f| f.to_string()),
            )
            .collect();
        let hidden: Vec<&String> = every
            .iter()
            .filter(|field| {
                form.stat_types
                    .iter()
                    .any(|(k, _, _)| !type_takes(k).contains(field))
            })
            .collect();
        assert!(hidden.iter().any(|f| *f == "stat.weight"), "{hidden:?}");
        for field in &hidden {
            let rule = format!(
                ".stat-group:has(> .filter > select > option:checked:not([data-takes~=\"{field}\"])) [name=\"{field}\"] {{ display: none; }}"
            );
            assert!(css.contains(&rule), "no hide rule for {field}: {rule}");
        }
        assert_eq!(
            css.matches("option:checked:not(").count(),
            hidden.len(),
            "a hide rule for a field every type takes"
        );
        let scripts = [
            include_str!("assets/lists.js"),
            include_str!("assets/count.js"),
            include_str!("assets/window.js"),
            include_str!("assets/link.js"),
        ];
        for (k, _, _) in &form.stat_types {
            for named in [format!("\"{k}\""), format!("'{k}'"), format!("={k}]")] {
                assert!(!css.contains(&named), "the stylesheet names the type {k}");
                for script in scripts {
                    assert!(!script.contains(&named), "a script names the type {k}");
                }
            }
        }
    }

    /// The planner says a character's listed experience differs from its
    /// last fetch's in either direction, so the page says it differs, never
    /// that it grew.
    #[test]
    fn an_experience_that_differs_is_said_to_differ() {
        let reason = acquisition_plan::FetchReason::ListedExperienceDisagrees {
            listed: 100,
            held: 150,
        };
        let said = fetch_words(&reason);
        assert!(!said.contains("more"), "{said}");
        assert!(said.contains("differs"), "{said}");
    }
}
