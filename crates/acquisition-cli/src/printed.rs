//! The search's ways on as this terminal prints them (C143: a shared
//! value, spelled by the surface that prints it; #97). A request, a
//! continuation, a route, the tabs an answer never fetched and an item
//! to show are values the search crate makes; the command lines here are
//! the CLI's alone, as the MCP's tools and a page's links are theirs.
//! Every line is the one way the CLI prints a command (the build plan's
//! rule 5, a command printed is a command that runs; C156, each word
//! quoted for the declared shell).

use acquisition_search::answer::{
    Answer, Context, Membership, Next, Request, Route, Shape, TabsPointer, shell_quoted,
};
use acquisition_search::corpus::Realm;

/// A value of the search's as the command line this terminal prints.
pub(crate) trait Printed {
    /// The command a terminal takes for it.
    fn command(&self) -> String;
}

/// A command a terminal takes: the one way the search prints one (the
/// build plan's rule 5 — a command printed is a command that runs). It
/// names the account it was answered for, since a second account makes a
/// command without one an error, and a last word that starts with `-` —
/// the language's not, a terminal's flag — is printed after a space
/// inside its quotes, or `-has:note` is read as `-h`: the query reads the
/// space as nothing, and the word reads the same in every shell C156
/// names, with no `--` to carry.
pub(crate) fn command(
    verb: &str,
    account: Option<&str>,
    realm: Option<&str>,
    last: &str,
) -> String {
    command_with(verb, account, realm, last, "")
}

/// [`command`] with the flags a view and a context add: after the query,
/// as a reader types them.
fn command_with(
    verb: &str,
    account: Option<&str>,
    realm: Option<&str>,
    last: &str,
    flags: &str,
) -> String {
    let mut out = opening(verb, account, realm);
    out.push(' ');
    if last.starts_with('-') {
        out.push_str(&shell_quoted(&format!(" {last}")));
    } else {
        out.push_str(&shell_quoted(last));
    }
    out.push_str(flags);
    out
}

/// How every command the search prints opens — `show`'s `price show`
/// pointer too: the verb, the account it was answered for, the realm.
pub(crate) fn opening(verb: &str, account: Option<&str>, realm: Option<&str>) -> String {
    let mut out = format!("acq {verb}");
    if let Some(account) = account {
        out.push_str(&format!(" --account {}", shell_quoted(account)));
    }
    if let Some(realm) = realm {
        out.push_str(&format!(" --realm {realm}"));
    }
    out
}

impl Printed for Next {
    /// The continuation as a command a terminal takes: the token whole,
    /// and the account it was answered for (`command`).
    fn command(&self) -> String {
        // the realm is the token's to carry
        let mut out = opening("search", self.request.scope.account.as_deref(), None);
        out.push_str(&format!(" --next {}", shell_quoted(&self.request.token())));
        out
    }
}

impl Printed for Route {
    /// The route as a command a terminal takes: its request's.
    fn command(&self) -> String {
        self.request.command()
    }
}

impl Printed for Request {
    /// The request as a command a terminal takes: the query, then the
    /// view where it is not the default rows, then the context where it
    /// is not the matches. A request that carries a tree and no text has
    /// none of its own: an answer returns the text. One that carries a
    /// site search is `--trade` and that search, and no realm where its
    /// URL names one: `--realm` beside a URL is a conflict (K6).
    fn command(&self) -> String {
        let scope = &self.scope;
        let mut out = String::new();
        if scope.membership != Membership::Live {
            out.push_str(" --membership ");
            out.push_str(scope.membership.as_str());
        }
        view_flags(self, &mut out);
        if self.context == Context::Corpus {
            out.push_str(" --context corpus");
        }
        if let Some(word) = self.query.trade_word() {
            // the realm the site search names by itself, read where every
            // reader reads it
            let url = self.query.read(None).ok().and_then(|read| read.realm);
            let mut command = opening(
                "search",
                scope.account.as_deref(),
                match url {
                    Some(_) => None,
                    None => scope.realm.as_ref().map(Realm::as_str),
                },
            );
            command.push_str(&format!(" --trade {}", shell_quoted(&word)));
            command.push_str(&out);
            return command;
        }
        command_with(
            "search",
            scope.account.as_deref(),
            scope.realm.as_ref().map(Realm::as_str),
            self.query.text.as_deref().unwrap_or_default(),
            &out,
        )
    }
}

fn view_flags(request: &Request, out: &mut String) {
    shape_flags(request, out);
    if request.view.routes {
        out.push_str(" --routes");
    }
}

fn shape_flags(request: &Request, out: &mut String) {
    let (flag, counts) = match &request.view.shape {
        Shape::Rows(rows) => {
            if let Some(sort) = &rows.sort {
                out.push_str(&format!(" --sort {}", shell_quoted(sort)));
                if rows.desc {
                    out.push_str(" --desc");
                }
            }
            if let Some(limit) = rows.limit {
                out.push_str(&format!(" --limit {limit}"));
            }
            return;
        }
        Shape::Explain(explain) => {
            out.push_str(&format!(" --explain {}", shell_quoted(&explain.path)));
            return;
        }
        Shape::Counts(c) => ("--count", c),
        Shape::Cross(c) => ("--cross", c),
    };
    for list in counts.lists() {
        out.push_str(&format!(" {flag} {}", shell_quoted(&list)));
    }
    if let Some(sum) = &counts.sum {
        out.push_str(&format!(" --sum {}", shell_quoted(sum)));
    }
    if let Some(limit) = counts.limit {
        out.push_str(&format!(" --limit {limit}"));
    }
}

/// The never-fetched tabs an answer points at, as the `tabs` command a
/// terminal takes: opened as every command is, its league and the
/// never-fetched filter after.
pub(crate) fn tabs_command(pointer: &TabsPointer) -> String {
    let mut command = opening("tabs", Some(&pointer.account), Some(&pointer.realm));
    command.push_str(&format!(
        " --league {} --never-fetched",
        shell_quoted(&pointer.league)
    ));
    command
}

/// The command that shows one of this answer's items whole: where a
/// bounded block of a row sends its reader (invariant 5).
pub(crate) fn show_command(answer: &Answer, id: &str) -> String {
    command("show", Some(&answer.account_named()), None, id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn request(value: serde_json::Value) -> Request {
        serde_json::from_value(value).unwrap()
    }

    /// The lines the search crate's tests held before the spelling moved
    /// here (#97, Part B): a membership other than live is spelled after
    /// the query and live left unsaid; a request from its start is its
    /// scope, its empty query and its limit; the tabs an answer never
    /// fetched are `tabs` over the league with the never-fetched filter.
    #[test]
    fn c143_a_search_value_is_the_command_this_terminal_prints() {
        let scope = |membership: &str| {
            json!({ "query": { "text": "id:t2" },
                    "scope": { "account": "u-1", "realm": "pc", "membership": membership } })
        };
        assert_eq!(
            request(scope("all")).command(),
            "acq search --account u-1 --realm pc id:t2 --membership all"
        );
        assert_eq!(
            request(scope("removed")).command(),
            "acq search --account u-1 --realm pc id:t2 --membership removed"
        );
        assert_eq!(
            request(scope("live")).command(),
            "acq search --account u-1 --realm pc id:t2"
        );
        assert_eq!(
            request(json!({ "query": { "text": "" },
                            "scope": { "account": "A", "realm": "pc" },
                            "view": { "rows": { "limit": 3 } } }))
            .command(),
            "acq search --account A --realm pc '' --limit 3"
        );
        assert_eq!(
            tabs_command(&TabsPointer {
                account: "A".into(),
                realm: "pc".into(),
                league: "Allflame".into(),
                other_leagues: 1,
            }),
            "acq tabs --account A --realm pc --league Allflame --never-fetched"
        );
    }
}
