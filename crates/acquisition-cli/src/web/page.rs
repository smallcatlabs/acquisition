//! The pages (C16, a renderer, not an app; C53, text is a function of the
//! JSON): each renders one document a verb produced, as HTML, with the
//! CLI's own words where the CLI has them — the answer's opening and its
//! terms, `show`, the plan, the jobs table — so the page states nothing
//! the terminal does not. A page sorts, filters, groups and composes
//! nothing; where it wants what no document carries, the search crate or
//! the verb grows first.

use acquisition_search::Classed;
use acquisition_search::answer::{Answer, ViewOut};
use acquisition_search::show::Shown;
use acquisition_search::trade::translate::{Fidelity, Status};

use super::form::{Fate, Filter, Form, LIMITS, STAT_ROWS};
use crate::search_cmd;

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
}

/// A page: the template filled, the body last so nothing in it is read
/// as a placeholder.
pub(crate) fn page(token: &str, title: &str, head: Head, body: &str) -> String {
    let refresh = head
        .refresh
        .map(|s| format!("<meta http-equiv=\"refresh\" content=\"{s}\">\n"))
        .unwrap_or_default();
    let script = if head.picker {
        format!("<script src=\"/{token}/assets/picker.js\"></script>\n")
    } else {
        String::new()
    };
    TEMPLATE
        .replace("{refresh}", &refresh)
        .replace("{script}", &script)
        .replace("{title}", &esc(title))
        .replace("{token}", token)
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

/// The search form (Do 6): the realm and the league, every filter of the
/// dump with its fate, the stat rows with the picker, the view.
///
/// The form is one realm's render: its league list is that realm's
/// counts, so the realm is chosen by a form of its own that renders the
/// page again (the review of 4b, its fourth finding), and the search
/// carries the realm shown.
pub(crate) fn search_form(form: &Form, realm: &str, leagues: &[(String, usize)]) -> String {
    let mut out = String::from(
        "<form method=\"get\" action=\".\"><label class=\"control\">Realm <select name=\"realm\">",
    );
    for r in ["pc", "xbox", "sony"] {
        out.push_str(&format!(
            "<option value=\"{r}\"{}>{r}</option>",
            if r == realm { " selected" } else { "" }
        ));
    }
    out.push_str(
        "</select></label> <button type=\"submit\">Show this realm's leagues</button></form>\n",
    );
    out.push_str(&format!(
        "<form method=\"get\" action=\"search\">\n<fieldset><legend>Scope</legend>\n<input type=\"hidden\" name=\"realm\" value=\"{}\"><span class=\"control\">Realm {}</span>\n",
        esc(realm),
        esc(realm)
    ));
    out.push_str("<label class=\"control\">League <select name=\"league\"><option value=\"\">every league</option>");
    for (league, items) in leagues {
        out.push_str(&format!(
            "<option value=\"{}\">{} ({items})</option>",
            esc(league),
            esc(league)
        ));
    }
    out.push_str("</select></label>");
    out.push_str(&fate_span(form.fate("url.league")));
    out.push_str("\n</fieldset>\n");
    for group in &form.groups {
        let legend = group.title.as_deref().unwrap_or("Status");
        out.push_str(&format!("<fieldset><legend>{}</legend>\n", esc(legend)));
        for filter in &group.filters {
            let path = super::form::filter_path(&group.id, &filter.id);
            let key = if group.id == "status_filters" {
                "query.status".to_string()
            } else {
                format!("{}.{}", group.id, filter.id)
            };
            out.push_str(&control(form, filter, &path, &key));
        }
        out.push_str("</fieldset>\n");
    }
    out.push_str("<fieldset><legend>Stats</legend>\n<label class=\"control\">Group <select name=\"query.stats[0].type\">");
    for (kind, text, fate) in &form.stat_types {
        out.push_str(&format!(
            "<option value=\"{}\"{}>{} ({})</option>",
            esc(kind),
            if kind == "and" { " selected" } else { "" },
            esc(text.as_deref().unwrap_or(kind)),
            esc(&fate.status.replace('_', " "))
        ));
    }
    out.push_str("</select></label>\n<label class=\"control\">Group min <input type=\"number\" step=\"any\" name=\"query.stats[0].value.min\"></label>\n<label class=\"control\">Group max <input type=\"number\" step=\"any\" name=\"query.stats[0].value.max\"></label><br>\n");
    for n in 0..STAT_ROWS {
        let id = format!("stat-{n}");
        out.push_str(&format!(
            "<div><label class=\"control\">Stat <input class=\"stat-id\" id=\"{id}\" name=\"query.stats[0].filters[{n}].id\" size=\"28\"></label> <span id=\"{id}-text\" class=\"fate\"></span>\n<label class=\"control\">min <input type=\"number\" step=\"any\" name=\"query.stats[0].filters[{n}].value.min\"></label>\n<label class=\"control\">max <input type=\"number\" step=\"any\" name=\"query.stats[0].filters[{n}].value.max\"></label></div>\n"
        ));
    }
    out.push_str("<label class=\"control\">Find a stat <input type=\"search\" id=\"stat-find\" autocomplete=\"off\"></label>\n<span id=\"stat-said\" class=\"fate\"></span>\n<ul id=\"stat-list\"></ul>\n</fieldset>\n");
    out.push_str("<fieldset><legend>Answer</legend>\n<label class=\"control\">Show <select name=\"view.limit\">");
    for limit in LIMITS {
        out.push_str(&format!("<option value=\"{limit}\">{limit}</option>"));
    }
    out.push_str("</select></label>\n<label class=\"control\">Sort by <input name=\"view.sort\" placeholder=\"ilvl\"></label>\n<label class=\"control\"><input type=\"checkbox\" name=\"view.desc\" value=\"true\"> highest first</label><br>\n<label class=\"control\">Or count the matches by <input name=\"view.count\" placeholder=\"rarity\"></label> <span class=\"fate\">one key: rarity, league, tab, class, category, line</span>\n</fieldset>\n<button type=\"submit\">Search</button>\n</form>\n");
    out
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
    let mut out = format!("<span class=\"control\"{title}><span{tip}>{label}</span> ");
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
                " <input type=\"number\" name=\"{path}.{c}\" placeholder=\"{}\" style=\"width:3rem\"{disabled}>",
                c.to_uppercase()
            ));
        }
    }
    out.push_str(&fate_span(fate));
    out.push_str("</span>\n");
    out
}

/// The answer page (Do 6): the opening as the CLI prints it, the rows as
/// a table with their art, the continuation as a link, then the terms,
/// the routes and the basis as the CLI prints them.
pub(crate) fn answer(a: &Answer) -> String {
    let now = acquisition_store::now();
    let mut out = answer_badges(a);
    out.push_str(&pre(&search_cmd::answer_head(a)));
    if !a.rows.is_empty() {
        out.push_str("<table>\n<tr><th></th><th>Name</th><th>Base</th><th>Rarity</th><th>Place</th><th>Lines</th></tr>\n");
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
            out.push_str(&format!(
                "<tr><td class=\"icon\">{icon}</td><td><a href=\"item/{}\">{}</a></td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                enc(&row.id),
                esc(&name),
                esc(row.base.as_deref().unwrap_or("")),
                esc(row.rarity.as_deref().unwrap_or("")),
                esc(&place),
                shows
                    .iter()
                    .chain(&more)
                    .map(|s| esc(s))
                    .collect::<Vec<_>>()
                    .join("<br>")
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
                "<p>{shown} of {} shown · <a href=\"search?next={}\">more</a></p>\n",
                a.total.matched,
                enc(&next.request.token())
            ),
            None => format!(
                "<p>{shown} of {} shown · a larger number of rows returns the rest</p>\n",
                a.total.matched
            ),
        });
    }
    out.push_str(&pre(&search_cmd::answer_tail(a, None)));
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

fn plural(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}
