//! `acq search` and `acq show`: the clap adapter of `acquisition-search`
//! and its rendering. No daemon, no network: the store is read directly,
//! as `acq tabs` reads it. Rulings: `decisions/search.md`; the surface is
//! the clap help below, which `CLI-REFERENCE.md` holds whole.
//!
//! # As built
//!
//! - **Text is a function of JSON (C53).** `--json` prints the crate's
//!   [`Answer`], [`Describe`] or [`Shown`] whole; the text below is
//!   rendered from that same value and from nothing else. Ages are text,
//!   epochs JSON.
//! - **A failure is structured on both paths (C11)**: `--json` prints
//!   `{"error", "kind", "readings", "offers"}` — each way on that is a
//!   search a request too, as `--request` reads one — and exits 1; the
//!   text prints the message and each reading on a line of its own,
//!   ready to paste.
//! - **An answer never prints a command this build refuses** (the build
//!   plan, rule 5): a block invariant 5 bounds with a route to the whole
//!   prints the count left out and the unbuilt construct's name. The flags
//!   of later steps exist, so that typing one is refused by that name
//!   (`acquisition_search::not_built`) and never as an unknown flag.
//! - **`--next`, `--request` and `--print-request` exchange the
//!   request** (`answer.rs`; C104, C100): a token is a
//!   continuation's request and `--next` runs it, `--request` runs one
//!   read from a file — or a count's route, as an answer's `--json`
//!   holds it — and `--print-request` prints the request the words
//!   typed make, account and realm resolved by the store it would run
//!   on, and asks nothing. A token and a request read are whole: any
//!   word of scope, query or view beside one is an error, but `--limit`
//!   beside a token, whose place does not depend on it.
//! - **`--explain`, `--context` and `show --against` are encodings** of
//!   what the request and `show` carry (`answer.rs`, `show.rs`): the
//!   view's conflicts are the crate's to say
//!   (`Shape::of`), the context's word is the crate's to read, and what a
//!   zero answer offers is printed as the commands the crate makes of its
//!   requests (`Request::command`), each run by the test that runs every
//!   command printed.
//! - **The query may come from a file or stdin** (`--query-file`, the
//!   plan's gap 2): a template with an apostrophe cannot be typed plainly
//!   inside a shell's single quotes. A route is printed shell-quoted.
//! - **`--trade` and `--trade-file` are a query's third spelling**
//!   (C113): the site search is the request's `query.trade`, translated by
//!   the crate at every read (`answer.rs`), and `--print-request` writes
//!   the text it translates to beside it. `--realm` beside a URL is K6's
//!   conflict, said here before anything is read, the same realm
//!   included; the request's own rule, a scope beside a URL being the
//!   URL's, is the crate's. The text opens with the translation, its
//!   remainder loud (`trade_text`).
//! - **The intent file is opened beside the store** (`plan_cmd::open_intent`,
//!   as `acq price` opens it): the effective price a query reads is the
//!   listing state's (C81, C100), and every basis names the intent
//!   revision (C98). Nothing is written to it.

use std::io::Read as _;

use acquisition_search::answer::{
    Answer, Asked, ClauseOut, Context, Count, DEFAULT_LIMIT, Membership, Request, Rows, Scope,
    Shape, TradeOut, View, ViewOut,
};
use acquisition_search::corpus::Realm;
use acquisition_search::counts::{Bucket, CrossOut, Label, SumOf, Summed, Table};
use acquisition_search::describe::Describe;
use acquisition_search::show::Shown;
use acquisition_search::trade::Source;
use acquisition_search::trade::translate::{Fidelity, Status};
use acquisition_search::{Priced, SearchError, answer, describe, not_built, show};
use acquisition_store::{Store, account_path};
use anyhow::{Context as _, Result};
use serde::Serialize;

use crate::plan_cmd::open_intent;
use crate::store_cmd::ago;

/// What `acq search` was typed with.
#[derive(clap::Args)]
pub struct SearchArgs {
    /// The query (`acq search --describe` prints the language). None:
    /// every item in scope. One that starts with `-`, the language's not,
    /// goes after `--`, as every route prints it: `acq search --realm pc
    /// -- '-is:corrupted'`.
    pub query: Option<String>,
    /// The realm searched: pc, xbox, sony, poe2, or all. Over a store
    /// holding one realm it may be omitted and the answer prints it; over
    /// several it is required, one of them or `all` (C96).
    #[arg(long)]
    pub realm: Option<String>,
    /// Which items are searched: `live`, the default; `all`, which adds
    /// the removed — each row marked with when it left, at the place it
    /// was last seen; or `removed`, the removed alone. The scope block
    /// counts the live and the removed apart under either.
    #[arg(long, value_name = "live|all|removed")]
    pub membership: Option<String>,
    /// Read the query from a file, or from stdin with `-`: a query with an
    /// apostrophe needs no shell quoting this way.
    #[arg(long, value_name = "FILE|-", conflicts_with = "query")]
    pub query_file: Option<String>,
    /// A trade site search in place of a query, translated with no
    /// network to the query a person would type: its URL
    /// (`https://www.pathofexile.com/trade/search/<league>/<id>`, or
    /// `…/search/xbox/<league>/<id>` and `sony` for a console), its id
    /// alone, or its query JSON. A URL's realm is the search's scope, so
    /// `--realm` beside one is an error; an id or JSON names no realm and
    /// takes `--realm` as a query does. The answer opens with how
    /// faithful the translation is and every clause it could not run;
    /// the clauses, each with its status, are its JSON's `query.trade`,
    /// and `--describe trade` is what becomes of each. The site's sort is
    /// reported, never applied. PoE1 only.
    #[arg(long, value_name = "URL|ID", conflicts_with_all = ["query", "query_file"])]
    pub trade: Option<String>,
    /// `--trade`, read from a file or from stdin with `-`: the query JSON
    /// as the site posts it, `{"query": …, "sort": …}` or the query
    /// object alone, needs no shell quoting this way.
    #[arg(long, value_name = "FILE|-", conflicts_with_all = ["query", "query_file", "trade"])]
    pub trade_file: Option<String>,
    /// Order the rows by a value: `ilvl`, `stack`, `'line("T").arg1'`,
    /// `'sum("T")'`, `pseudo.total_res`. A line's scalar is its largest
    /// satisfying occurrence; an item with none sorts last either way
    /// (C92).
    #[arg(long, value_name = "VALUE")]
    pub sort: Option<String>,
    /// Largest first.
    #[arg(long, requires = "sort")]
    pub desc: bool,
    /// How many rows to return — or, of a count, how many values a table
    /// lists; the rest are counted.
    /// Twenty where none is given.
    #[arg(long)]
    pub limit: Option<usize>,
    /// Print every count's route — the command that returns exactly its
    /// members — instead of the few a next step hangs on; under `--json`,
    /// every count's request. Beside `--next` or `--request`, the answer
    /// it runs carries them all.
    #[arg(long)]
    pub routes: bool,
    /// The language as this build knows it. Bare, the index: fields,
    /// what a line has, operators, closed value sets, slots, how terms
    /// compose, what a count takes, what is not built and the limits
    /// stated, with each computed value and category by name and one
    /// line (about 47 KB). Named, those entries whole with their
    /// definitions (`--describe league,line,pseudo.total_res`; a block's
    /// name is one: `--describe computed`, and `--describe trade` what
    /// `--trade` makes of each clause of a site search). `--describe
    /// all` is every entry whole, over 150 KB. `--json` is whole unless
    /// entries are named.
    #[arg(long, value_name = "NAME,…", num_args = 0..=1, value_delimiter = ',')]
    pub describe: Option<Vec<String>>,
    /// Count the matches by a key, one table for each key named, and show
    /// no rows: a field (`tab`, `league`, `rarity`, `base`, …; `--describe
    /// counts` lists them), or `line`, the vocabulary — the templates the
    /// matching items carry, ranked, each with the term that selects it and
    /// the range of its numbers. `line:` takes the rest of the list as
    /// texts to narrow by, a table each, and `~` before one makes it a
    /// pattern: `--count tab,line:resist,life`. An item with no value is
    /// counted under `none`, one whose value could not be read under
    /// `undecided` (C105); every count has its route (`--routes`). Given
    /// again, the flag starts a list of its own, so a field can follow a
    /// narrowed vocabulary: `--count line:resist,life --count rarity`.
    #[arg(long, value_name = "KEY,…")]
    pub count: Vec<String>,
    /// Count the matches by two fields at once, one table of the cells
    /// that hold an item: `--cross league,tab`.
    #[arg(long, value_name = "KEY,KEY")]
    pub cross: Option<String>,
    /// Beside each count, the sum of one number over its items: `stack`,
    /// `ilvl`, `'sum("T")'`, `pseudo.total_res`. An item lacking the thing
    /// adds nothing and is counted as lacking; one unread leaves a
    /// subtotal marked incomplete (C95).
    #[arg(long, value_name = "VALUE")]
    pub sum: Option<String>,
    /// Not built, waiting for a consumer that asks: the caller names a
    /// row's fields.
    #[arg(long, value_name = "NAME,…")]
    pub fields: Option<String>,
    /// Continue an answer past its limit: the token its `more` line
    /// printed, whole. The token names the query, the view, the order and
    /// the basis it continues, so no word of scope, query or view stands
    /// beside it but `--limit`;
    /// across a changed basis — a refresh, a price written — it is
    /// refused, both named, and the way on is the search from its start.
    #[arg(long, value_name = "TOKEN")]
    pub next: Option<String>,
    /// Explain one node of the query, named by its path — a term's as the
    /// terms block prints it (`3`), a group of terms', or a condition's
    /// inside `line( … )` or `linked( … )` (`3.1`): the root's matched and
    /// undecided counts as given, with that node forced true, and with it
    /// forced false, each beside its transformed query, the node replaced
    /// by `true()` or `false()` and nothing else changed. Counts and no
    /// rows: an error beside `--count`, `--cross`, `--sum` or `--sort`. A
    /// path the query lacks lists the query's nodes with their paths.
    #[arg(long, value_name = "PATH")]
    pub explain: Option<String>,
    /// What the answer is of: `matches`, the query's, as every answer is;
    /// or `corpus`, the empty query over the same scope under rows or a
    /// count, with the query typed printed as set aside and counted
    /// nowhere; beside `--explain`, which is of the query typed, an error.
    #[arg(long, value_name = "WHICH")]
    pub context: Option<String>,
    /// Not built, waiting for a consumer that asks: `locations`, the full
    /// coverage list.
    #[arg(long, value_name = "WHAT")]
    pub view: Option<String>,
    /// Print the request as JSON, its account and realm resolved, and do
    /// not run it: what `--request` reads.
    #[arg(long)]
    pub print_request: bool,
    /// Run a request read from a file, or from stdin with `-`: what
    /// `--print-request` printed, a `next` or an offer of an answer's
    /// `--json`, or a count's route — which reports both counts where its
    /// count has moved since, and wherever the basis has where the
    /// answer's `basis` is sent beside it as `counted_at`. A request is
    /// whole: no word
    /// of scope, query or view stands beside it.
    #[arg(long, value_name = "FILE|-")]
    pub request: Option<String>,
    /// Not built, waiting for a consumer that asks: run another account's
    /// request deliberately.
    #[arg(long)]
    pub rebind: bool,
}

/// What `acq show` was typed with.
#[derive(clap::Args)]
pub struct ShowArgs {
    /// An item's id, whole, as an answer printed it.
    pub id: String,
    /// Also print the body as the store holds it, verbatim.
    #[arg(long)]
    pub body: bool,
    /// Why the item does or does not match a query: beside the item, the
    /// query's canonical text, the root's outcome on it — match, no match,
    /// undecided — and each term's and each group's, with what the term
    /// touched or why it is undecided. One that starts with `-`, the
    /// language's not, is taken as typed: `--against '-is:corrupted'`.
    #[arg(long, value_name = "QUERY", allow_hyphen_values = true)]
    pub against: Option<String>,
    /// Not built, waiting for a consumer that asks: the item as a named
    /// basis held it.
    #[arg(long, value_name = "BASIS")]
    pub basis: Option<String>,
}

/// Print a failure on the path the caller chose, and exit 1 through main.
fn fail(e: SearchError, json: bool) -> anyhow::Error {
    if json {
        println!("{}", e.to_json());
    } else {
        match &e {
            SearchError::Language(e) => {
                eprintln!("Error: {}", e.message);
                e.readings.iter().for_each(|r| eprintln!("    {r}"));
            }
            SearchError::Scope {
                message, offers, ..
            } => {
                eprintln!("Error: {message}");
                offers.iter().for_each(|o| eprintln!("    {}", o.reading));
            }
            SearchError::Store(e) => eprintln!("Error: {e:#}"),
        }
    }
    crate::AlreadyReported.into()
}

fn emit<T: Serialize>(value: &T, json: bool, text: impl FnOnce(&T) -> String) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(value)?);
    } else {
        print!("{}", text(value));
    }
    Ok(())
}

pub fn search(args: SearchArgs, json: bool) -> Result<()> {
    let unbuilt = [
        ("--fields", args.fields.is_some()),
        ("--view locations", args.view.is_some()),
        ("--rebind", args.rebind),
    ];
    if let Some((construct, _)) = unbuilt.iter().find(|(_, typed)| *typed) {
        return Err(fail(not_built(construct).into(), json));
    }
    // a continuation and a request read are whole: what else was typed
    // has nowhere to stand
    let typed = [
        ("a query", args.query.is_some()),
        ("--query-file", args.query_file.is_some()),
        ("--trade", args.trade.is_some()),
        ("--trade-file", args.trade_file.is_some()),
        ("--realm", args.realm.is_some()),
        ("--membership", args.membership.is_some()),
        ("--sort", args.sort.is_some()),
        ("--count", !args.count.is_empty()),
        ("--cross", args.cross.is_some()),
        ("--sum", args.sum.is_some()),
        ("--explain", args.explain.is_some()),
        ("--context", args.context.is_some()),
        ("--print-request", args.print_request),
        ("--describe", args.describe.is_some()),
        ("--next", args.next.is_some() && args.request.is_some()),
        ("--limit", args.limit.is_some() && args.request.is_some()),
    ];
    let whole = |flag: &str, names: &str| -> Result<()> {
        match typed.iter().find(|(_, typed)| *typed) {
            Some((beside, _)) => Err(fail(
                SearchError::Scope {
                    kind: "view",
                    message: format!("`{flag}` {names}: {beside} has nowhere to stand beside it"),
                    offers: Vec::new(),
                },
                json,
            )),
            None => Ok(()),
        }
    };
    if let Some(path) = &args.request {
        whole(
            "--request",
            "runs a request whole, its scope, its query and its view",
        )?;
        let text = if path == "-" {
            let mut text = String::new();
            std::io::stdin().read_to_string(&mut text)?;
            text
        } else {
            std::fs::read_to_string(path)
                .with_context(|| format!("reading the request from {path}"))?
        };
        let value: serde_json::Value = serde_json::from_str(&text).map_err(|e| {
            fail(
                SearchError::Scope {
                    kind: "tree",
                    message: format!("the request is no JSON: {e}"),
                    offers: Vec::new(),
                },
                json,
            )
        })?;
        let mut asked = Asked::read(&value).map_err(|e| fail(e.into(), json))?;
        asked.request.view.routes |= args.routes;
        asked.request.check().map_err(|e| fail(e.into(), json))?;
        let (store, intent) = open_store_and_intent()?;
        let corpus = asked
            .request
            .corpus(&store, &intent)
            .map_err(|e| fail(e, json))?;
        let answered = asked.answer(&corpus).map_err(|e| fail(e, json))?;
        return emit(&answered, json, answer_text);
    }
    if let Some(token) = &args.next {
        whole(
            "--next",
            "continues an answer whole, the token naming its query, its view and its order",
        )?;
        let mut request = Request::from_token(token).map_err(|e| fail(e.into(), json))?;
        if let (Some(limit), Shape::Rows(rows)) = (args.limit, &mut request.view.shape) {
            rows.limit = Some(limit);
        }
        request.view.routes |= args.routes;
        request.check().map_err(|e| fail(e.into(), json))?;
        let (store, intent) = open_store_and_intent()?;
        let corpus = request.corpus(&store, &intent).map_err(|e| fail(e, json))?;
        let answered = answer(&corpus, &request).map_err(|e| fail(e, json))?;
        return emit(&answered, json, answer_text);
    }
    if let Some(names) = &args.describe {
        if args.print_request {
            whole_describe(json)?;
        }
        // Bare is the index and `all` every entry whole: the same
        // document either way (JSON is the contract and stays whole, C53),
        // read at two densities.
        let all = names.as_slice() == ["all"];
        let index = names.is_empty();
        let asked: &[String] = if all { &[] } else { names };
        let described = describe(asked).map_err(|e| fail(e.into(), json))?;
        return emit(&described, json, |d| describe_text(d, index));
    }
    let read = |path: &str, what: &str| -> Result<String> {
        if path == "-" {
            let mut text = String::new();
            std::io::stdin().read_to_string(&mut text)?;
            Ok(text)
        } else {
            std::fs::read_to_string(path).with_context(|| format!("reading the {what} from {path}"))
        }
    };
    let text = match &args.query_file {
        Some(path) => read(path, "query")?,
        None => args.query.clone().unwrap_or_default(),
    };
    let trade = match (&args.trade, &args.trade_file) {
        (Some(trade), _) => Some(trade.clone()),
        (None, Some(path)) => Some(read(path, "site search")?),
        (None, None) => None,
    };
    // a URL's realm is the scope (K6): the flag beside one says it twice;
    // the realm the search names, read where every reader reads it, and an
    // input that does not read left to the request to refuse
    let named = |trade: &str| {
        acquisition_search::answer::QueryInput {
            trade: Some(serde_json::Value::String(trade.to_string())),
            ..Default::default()
        }
        .read(None)
        .ok()
        .and_then(|read| read.realm)
        .map(|realm| realm.as_str().to_string())
    };
    if let (Some(word), Some(trade)) = (&args.realm, &trade)
        && let Some(url) = named(trade)
    {
        return Err(fail(
            SearchError::Scope {
                kind: "trade",
                message: format!(
                    "`--realm {word}` beside a trade URL: the URL's realm, `{url}`, is the search's scope — leave `--realm` out, or give the search's id, which names no realm"
                ),
                offers: Vec::new(),
            },
            json,
        ));
    }
    let realm = match &args.realm {
        Some(word) => Some(Realm::parse(word).map_err(|e| fail(e, json))?),
        None => None,
    };
    let keys = |raw: Option<&str>| -> Result<Option<Vec<String>>> {
        raw.map(|raw| {
            keys(raw).map_err(|message| {
                fail(
                    SearchError::Scope {
                        kind: "view",
                        message,
                        offers: Vec::new(),
                    },
                    json,
                )
            })
        })
        .transpose()
    };
    let shape = Shape::of(
        // each `--count` is a list of its own, read apart and joined
        match args.count.as_slice() {
            [] => None,
            lists => {
                let mut all = Vec::new();
                for list in lists {
                    all.extend(keys(Some(list))?.unwrap_or_default());
                }
                Some(all)
            }
        },
        keys(args.cross.as_deref())?,
        args.sum.clone(),
        args.explain.clone(),
        Rows {
            limit: Some(args.limit.unwrap_or(DEFAULT_LIMIT)),
            sort: args.sort.clone(),
            desc: args.desc,
            after: None,
        },
    )
    .map_err(|e| fail(e.into(), json))?;
    let view = View {
        shape,
        routes: args.routes,
    };
    let context = match &args.context {
        Some(word) => Context::parse(word).map_err(|e| fail(e.into(), json))?,
        None => Context::default(),
    };
    let membership = match args.membership.as_deref() {
        None => Membership::Live,
        Some(word) => match Membership::parse(word) {
            Some(membership) => membership,
            None => {
                return Err(fail(
                    SearchError::Scope {
                        kind: "membership_unknown",
                        message: format!(
                            "`{word}` is no membership: live, the items in the stash; all, the removed beside them; or removed, the removed alone"
                        ),
                        offers: Vec::new(),
                    },
                    json,
                ));
            }
        },
    };
    let request = Request {
        scope: Scope {
            realm,
            membership,
            ..Scope::default()
        },
        query: acquisition_search::answer::QueryInput {
            text: trade.is_none().then(|| text.trim().to_string()),
            tree: None,
            trade: trade.map(serde_json::Value::String),
        },
        view,
        context,
        basis: None,
    };
    // an authoring error is said before the store is read
    request.check().map_err(|e| fail(e.into(), json))?;
    let (store, intent) = open_store_and_intent()?;
    // the request's own corpus, so that a refusal offers this request
    let corpus = request.corpus(&store, &intent).map_err(|e| fail(e, json))?;
    if args.print_request {
        // the request as it was bound, its account and its realm
        // resolved by the store it would be run on; nothing is asked
        let mut request = request;
        request.scope.account = Some(
            corpus
                .account_name
                .clone()
                .unwrap_or_else(|| corpus.basis.account.clone()),
        );
        request.scope.realm = Some(corpus.realm.clone());
        // a site search is printed beside the text it translates to,
        // which reading it back holds the translation to
        if request.query.trade.is_some() {
            let query = request.bind().map_err(|e| fail(e.into(), json))?;
            request.query.text = Some(query.text());
        }
        println!("{}", serde_json::to_string_pretty(&request)?);
        return Ok(());
    }
    let answered = answer(&corpus, &request).map_err(|e| fail(e, json))?;
    emit(&answered, json, answer_text)
}

/// `--print-request` prints the request the words typed make, and
/// `--describe` makes none.
fn whole_describe(json: bool) -> Result<()> {
    Err(fail(
        SearchError::Scope {
            kind: "view",
            message: "`--print-request` prints the request a search would run: --describe has nowhere to stand beside it".to_string(),
            offers: Vec::new(),
        },
        json,
    ))
}

/// The keys of `--count`, as a terminal spells them: a comma list, where
/// `line:` takes the rest of the list as its texts — `line:resist,life` is
/// the vocabulary narrowed twice, a table each (`--count`'s help,
/// `line[:text,…]`) — and a text is a pattern after `~`. A text
/// with a comma, or a `~` of its own, is quoted, `"…"`, with the
/// language's three escapes (`\"`, `\\`, `\n`) and no other; what is
/// quoted is the text as written, never syntax, and a quote that does not
/// close is an error, as it is in a query (outside audit, 2026-09-22).
fn keys(raw: &str) -> Result<Vec<String>, String> {
    // each part as characters, each marked quoted or not: what was quoted
    // is text as written, and only an unquoted character is syntax
    let mut parts: Vec<Vec<(char, bool)>> = vec![Vec::new()];
    let (mut quoted, mut escaped) = (false, false);
    for c in raw.chars() {
        let part = parts
            .last_mut()
            .unwrap_or_else(|| unreachable!("one part at least"));
        match c {
            _ if escaped => {
                part.push((
                    match c {
                        '"' | '\\' => c,
                        'n' => '\n',
                        other => {
                            return Err(format!(
                                "`\\{other}` is no escape a quoted text takes: \\\" \\\\ and \\n are the only ones"
                            ));
                        }
                    },
                    true,
                ));
                escaped = false;
            }
            '\\' if quoted => escaped = true,
            '"' => quoted = !quoted,
            ',' if !quoted => parts.push(Vec::new()),
            _ => part.push((c, quoted)),
        }
    }
    if quoted || escaped {
        return Err(format!(
            "a quoted text in `{raw}` never closes: a text with a comma or a ~ of its own is written \"…\", the language's escapes inside"
        ));
    }
    let mut narrowing = false;
    Ok(parts
        .into_iter()
        .map(|mut part| {
            // unquoted whitespace at the ends is none of the text
            while part.first().is_some_and(|(c, q)| !q && c.is_whitespace()) {
                part.remove(0);
            }
            while part.last().is_some_and(|(c, q)| !q && c.is_whitespace()) {
                part.pop();
            }
            let text = |part: &[(char, bool)]| part.iter().map(|(c, _)| *c).collect::<String>();
            let unquoted_prefix = |part: &[(char, bool)], word: &str| {
                part.len() >= word.len()
                    && part[..word.len()]
                        .iter()
                        .zip(word.chars())
                        .all(|((c, q), w)| !q && c.eq_ignore_ascii_case(&w))
            };
            let pattern_marked = |part: &[(char, bool)]| part.first() == Some(&('~', false));
            if unquoted_prefix(&part, "line:") || unquoted_prefix(&part, "line~") {
                narrowing = true;
                let pattern = part[4].0 == '~';
                let rest = &part[5..];
                // `line:~pattern` is `line~pattern`
                return if pattern {
                    format!("line~{}", text(rest))
                } else if pattern_marked(rest) {
                    format!("line~{}", text(&rest[1..]))
                } else {
                    format!("line:{}", text(rest))
                };
            }
            if !narrowing {
                text(&part)
            } else if pattern_marked(&part) {
                format!("line~{}", text(&part[1..]))
            } else {
                format!("line:{}", text(&part))
            }
        })
        .collect())
}

pub fn show_item(args: ShowArgs, json: bool) -> Result<()> {
    if args.basis.is_some() {
        return Err(fail(not_built("show --basis").into(), json));
    }
    let against = args
        .against
        .as_ref()
        .map(|text| acquisition_search::answer::QueryInput {
            trade: None,
            text: Some(text.trim().to_string()),
            tree: None,
        });
    // an authoring error is said before the store is read
    if let Some(query) = &against {
        query.read(None).map_err(|e| fail(e.into(), json))?;
    }
    let (store, intent) = open_store_and_intent()?;
    let shown = match &against {
        Some(query) => {
            acquisition_search::show::against(&store, &intent, &args.id, args.body, query)
        }
        None => show(&store, &intent, &args.id, args.body),
    }
    .map_err(|e| fail(e, json))?;
    emit(&shown, json, shown_text)
}

/// The selected account's facts and its intent file, as `acq price` opens
/// them: the search reads both and writes neither.
fn open_store_and_intent() -> Result<(Store, acquisition_store::Annotations)> {
    let (dir, entry, intent) = open_intent()?;
    let store = Store::open(&account_path(&dir, &entry.username))?;
    Ok((store, intent))
}

// ---- text, a function of the JSON ---------------------------------------------------------------

/// The basis line every answer and every `show` prints (C98).
fn basis_text(b: &acquisition_search::Basis) -> String {
    format!(
        "basis   store {} · build {} · snapshot {} · facts v{} · intent {} · derivation {} · classes v{} · categories v{} · beast colours v{} · totals v{} · defences v{} · currency v{} · notes v{}",
        b.store,
        b.build,
        b.snapshot.response,
        b.snapshot.facts_version,
        b.intent,
        b.derivation,
        b.classes,
        b.categories,
        b.beast_colours,
        b.totals,
        b.defences,
        b.currency,
        b.notes
    )
}

fn plural(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

fn number(value: &serde_json::Value) -> String {
    value.to_string()
}

/// A slot's number as `show` prints one: whole where it is, `null` where
/// it could not be read.
fn slot_number(n: Option<f64>) -> String {
    match n.map(acquisition_search::Number::from_f64) {
        None => "null".to_string(),
        Some(acquisition_search::Number::Int(n)) => n.to_string(),
        Some(acquisition_search::Number::Float(n)) => n.to_string(),
    }
}

/// A name and a type line as the game shows them together; a rendering,
/// never a field (`derive.rs`).
fn title(name: Option<&str>, typeline: Option<&str>) -> String {
    match (name, typeline) {
        (Some(name), Some(typeline)) => format!("{name} {typeline}"),
        (name, typeline) => name.or(typeline).unwrap_or("(no name)").to_string(),
    }
}

/// How many types the scope line names among the never fetched; the
/// rest are counted (C118: a map search sees `MapStash` at once).
const NEVER_FETCHED_NAMED: usize = 3;

/// The never fetched, said by type: ` never fetched: MapStash 2489 of
/// 2582, …`, the largest named and the rest counted.
fn never_fetched_text(by_type: &[acquisition_search::tabs::TabType]) -> String {
    if by_type.is_empty() {
        return " never fetched".to_string();
    }
    let mut named: Vec<String> = by_type
        .iter()
        .take(NEVER_FETCHED_NAMED)
        .map(|t| format!("{} {} of {}", t.kind, t.tabs - t.fetched, t.tabs))
        .collect();
    let rest = &by_type[by_type.len().min(NEVER_FETCHED_NAMED)..];
    if !rest.is_empty() {
        named.push(format!(
            "{} more in {}",
            rest.iter().map(|t| t.tabs - t.fetched).sum::<usize>(),
            plural(rest.len(), "other type", "other types")
        ));
    }
    format!(" never fetched: {}", named.join(", "))
}

fn place_text(place: &acquisition_search::corpus::Place, now: i64) -> String {
    let mut out = place
        .league
        .clone()
        .unwrap_or_else(|| "no league".to_string());
    if let Some(parent) = &place.parent {
        out.push_str(&format!(
            " / {}",
            parent.name.as_deref().unwrap_or(&parent.id)
        ));
    }
    // a tab's page with no name of its own is the tab's page, by its id
    // (the trade seat: cli-opus-2 calls 29, 30, mcp-opus-2 call 29)
    out.push_str(&match (&place.name, &place.parent) {
        (Some(name), _) => format!(" / {name}"),
        (None, Some(_)) => format!(" / page {}", place.id),
        (None, None) => format!(" / {}", place.id),
    });
    if place.kind == "character" {
        out.push_str(&format!(
            " ({})",
            place.container.as_deref().unwrap_or("character")
        ));
    }
    if let Some(cell) = &place.cell {
        out.push_str(&format!(" · x {}, y {}", cell.x, cell.y));
    }
    if let Some(parent) = &place.socketed_in {
        out.push_str(&format!(
            " · socketed in {}",
            parent.name.as_deref().unwrap_or(&parent.id)
        ));
    }
    // how old the place is, as `tabs` says a tab's age (C118: "where do I
    // pick it up" asked it of the trade seat, cli-opus-2 call 46)
    out.push_str(&match place.fetched_at {
        Some(at) => format!(" · fetched {}", ago(now, Some(at))),
        None => " · never fetched".to_string(),
    });
    out
}

/// One piece of a row's evidence, as a term's or a sort's.
fn evidence_text(evidence: &acquisition_search::answer::Evidence) -> String {
    use acquisition_search::answer::Evidence;
    match evidence {
        Evidence::Line {
            source,
            flags,
            text,
        } => {
            let kind: Vec<&str> = std::iter::once(source.as_str())
                .chain(flags.iter().map(String::as_str))
                .collect();
            format!("{} ({})", text.replace('\n', " / "), kind.join(", "))
        }
        Evidence::Shown { part, text } => format!("{text} ({part})"),
        Evidence::Value { name, value } => format!("{name} {}", json_text(value)),
        Evidence::Undecided { path, term, reason } => format!(
            "term {path} {term} is undecided: {} unread — {}; {}",
            reason.unread, reason.problem, reason.hint
        ),
    }
}

/// A site search's translation as an answer opens with it: one line,
/// how far the text is the site's search; one line saying the items are
/// the store's, never the site's listings, since a reader took `exact`
/// for "the site would show the same" (the trade seat: mcp-opus calls 1,
/// 11; cli-sonnet's debrief, answer 4); then each clause that did not
/// run, each that runs as the site may not read it and each with no
/// stash meaning, with its note — ten of each listed and the rest
/// counted (C53) — and, beside an ambiguous clause whose other reading
/// the catalogue holds, what that reading adds and drops (C119), its
/// routes where the answer's routes are. The whole is the JSON's
/// `query.trade`.
fn trade_text(t: &TradeOut) -> Vec<String> {
    let from = match t.source {
        Source::Url => "a URL",
        Source::Id => "a search id",
        Source::Json => "the query JSON",
    };
    let of = |status: Status| -> Vec<&ClauseOut> {
        t.clauses.iter().filter(|c| c.status == status).collect()
    };
    let (unsupported, ambiguous) = (of(Status::Unsupported), of(Status::Ambiguous));
    let reported: Vec<&ClauseOut> = t
        .clauses
        .iter()
        .filter(|c| matches!(c.status, Status::ListingOnly | Status::Disabled))
        .collect();
    let mut out = vec![match t.fidelity {
        Fidelity::Partial => format!(
            "trade   from {from}: PARTIAL — {} not run, so this is not the site's search",
            plural(unsupported.len(), "clause", "clauses")
        ),
        Fidelity::Ambiguous => format!(
            "trade   from {from}: ambiguous — every clause ran, {} as the site may not read {}",
            ambiguous.len(),
            if ambiguous.len() == 1 { "it" } else { "them" }
        ),
        Fidelity::Exact => format!(
            "trade   from {from}: exact — every clause with a stash meaning ran as the site reads it"
        ),
    }];
    out.push(
        "        over      the items you hold, in every place, priced or not: the site shows \
         public listings, so the search is the site's and the items are yours"
            .to_string(),
    );
    let width = unsupported
        .iter()
        .chain(&ambiguous)
        .chain(&reported)
        .map(|c| c.at.chars().count() + 2)
        .max()
        .unwrap_or(0);
    for (word, clauses) in [
        ("NOT RUN  ", &unsupported),
        ("ambiguous", &ambiguous),
        ("reported ", &reported),
    ] {
        for c in clauses.iter().take(10) {
            let stake = c.other.as_ref().map_or(String::new(), |o| {
                format!(
                    " · read the other way: adds {}, drops {}",
                    o.adds.count, o.drops.count
                )
            });
            out.push(format!(
                "        {word} {:<width$}{}{stake}",
                c.at,
                c.notes.last().map_or("", String::as_str)
            ));
        }
        if clauses.len() > 10 {
            out.push(format!(
                "        {word} … and {} more: the JSON's query.trade.clauses",
                clauses.len() - 10
            ));
        }
    }
    out
}

/// The path column of a terms or nodes block: five wide, as the paths of
/// a flat query fit, and wider by the longest path, with a space after it.
fn path_column<'a>(paths: impl Iterator<Item = &'a str>) -> usize {
    paths
        .map(|p| p.chars().count() + 1)
        .max()
        .unwrap_or(0)
        .max(5)
}

fn answer_text(a: &Answer) -> String {
    let now = acquisition_store::now();
    let mut out = String::new();
    let mut line = |s: String| {
        out.push_str(s.trim_end());
        out.push('\n');
    };
    // a site search's translation opens the answer, its remainder loud
    let typed = a.set_aside.as_ref().unwrap_or(&a.query);
    if let Some(trade) = &typed.trade {
        trade_text(trade).into_iter().for_each(&mut line);
    }
    let query = if a.query.text.is_empty() {
        "(every item in scope)"
    } else {
        &a.query.text
    };
    line(format!("query   {query}"));
    if let Some(aside) = &a.set_aside {
        line(format!(
            "aside   {} — set aside under --context corpus, counted nowhere",
            aside.text
        ));
    }

    let scope = &a.scope;
    let account = scope.account.name.as_deref().unwrap_or(&scope.account.uuid);
    let mut first = format!(
        "scope   account {account} · {} · {} · {} fetched",
        scope.realm.as_str(),
        match (scope.membership, scope.coverage.removed) {
            (Membership::Removed, Some(removed)) => format!(
                "removed · {removed} removed · {} live not searched",
                plural(scope.coverage.items, "item", "items"),
            ),
            (_, Some(removed)) => format!(
                "all · {} live · {} removed",
                plural(scope.coverage.items, "item", "items"),
                removed
            ),
            (_, None) => format!("live · {}", plural(scope.coverage.items, "item", "items")),
        },
        plural(scope.coverage.fetched, "location", "locations"),
    );
    if let (Some(oldest), Some(newest)) = (scope.coverage.oldest_fetch, scope.coverage.newest_fetch)
    {
        first.push_str(&format!(
            " (oldest {}, newest {})",
            ago(now, Some(oldest)),
            ago(now, Some(newest))
        ));
    }
    line(first);
    let mut never = format!(
        "        {}{} · location list seen {}",
        plural(scope.coverage.never_fetched, "location", "locations"),
        never_fetched_text(&scope.coverage.never_fetched_by_type),
        ago(now, scope.coverage.list_seen),
    );
    // the list that exists: `tabs` over the league the query fixes, or the
    // one holding the most never fetched, the others counted (C118)
    if let Some(pointer) = a.tabs_pointer() {
        never.push_str(&format!(" · each tab: {}", pointer.command));
        if pointer.other_leagues > 0 {
            never.push_str(&format!(
                " · {} never-fetched tabs",
                plural(
                    pointer.other_leagues,
                    "other league holds",
                    "other leagues hold"
                )
            ));
        }
    }
    line(never);
    if scope.realm_not_held {
        line(format!(
            "        this store holds nothing under {}: it holds {}",
            scope.realm.as_str(),
            if scope.coverage.realms_held.is_empty() {
                "no realm".to_string()
            } else {
                scope.coverage.realms_held.join(", ")
            }
        ));
    }
    let mut total = format!("total   {}", plural(a.total.matched, "match", "matches"));
    if a.total.undecided.count > 0 {
        total.push_str(&format!(" · {} undecided", a.total.undecided.count));
    }
    line(total);
    // under every total, not a zero's alone: a count of N is as blind to
    // the never-fetched as a count of none (the trade seat: cli-sonnet
    // call 6, its fault 4; mcp-sonnet's debrief, answer 5)
    if a.scope.coverage.never_fetched > 0 {
        line(format!(
            "            {} never fetched: nothing here says what they hold",
            plural(
                a.scope.coverage.never_fetched,
                "location was",
                "locations were"
            )
        ));
    }

    if let Some(before) = &a.before {
        line(match &before.counted_at {
            Some(basis) => format!(
                "then    {} where this route was printed — {}",
                plural(before.count, "match", "matches"),
                basis.against(&a.basis)
            ),
            None => format!(
                "then    {} where this route was printed",
                plural(before.count, "match", "matches")
            ),
        });
    }

    for (i, row) in a.rows.iter().enumerate() {
        let mut head = title(row.name.as_deref(), row.typeline.as_deref());
        if let Some(rarity) = &row.rarity {
            head.push_str(&format!(" · {}", rarity.to_lowercase()));
        }
        // a scope that spans realms says which one each item is in, as
        // `show` does; under one realm the scope line has said it
        if a.scope.realm == Realm::All {
            head.push_str(&format!(" · {}", row.place.realm));
        }
        head.push_str(&format!(" · {}", place_text(&row.place, now)));
        if let Some(removed_at) = row.removed_at {
            head.push_str(&format!(" · removed {}", ago(now, Some(removed_at))));
        }
        line(format!(
            "{}{head}",
            if i == 0 { "rows    " } else { "        " }
        ));
        // each part of the row — the sort, each term — shows a set of the
        // item's occurrences, so a line printed twice by one part is two
        // occurrences: the row prints each line as many times as the part
        // that shows it most, in the order the parts show it, whichever
        // order the terms were written in (the review of 9b, 2026-09-26,
        // twice: a total of 40 showed contributors of 10 and 20 alone)
        let mut parts: Vec<Vec<String>> = Vec::new();
        if let Some(sorted) = &row.sort {
            // what sorted the row (F6): a field's name before its number,
            // a line or a sum's contributors after — and the sort's own
            // value before its number where no field names it, since
            // `sorts by 78` beside a term's sum of 70 was read as the
            // filter's number (the trade seat: cli-opus call 21, cli-sonnet
            // call 15)
            // a row that sorts last says by what (the review of 9b: `sorts
            // last: incomplete` named no field)
            let key = match &a.view {
                ViewOut::Rows(rows) => rows.sort.as_deref().unwrap_or_default(),
                ViewOut::Counts(_) | ViewOut::Cross(_) | ViewOut::Explain(_) => "",
            };
            let by = match (&sorted.value, sorted.status) {
                (Some(value), None) => format!("by {}", number(value)),
                (Some(value), Some(status)) => {
                    format!("last: {status} at {} ({key})", number(value))
                }
                (None, status) => format!("last: {} ({key})", status.unwrap_or("no value")),
            };
            let mut rest = sorted.shows.iter();
            let named = match sorted.shows.first() {
                Some(acquisition_search::answer::Evidence::Value { name, .. }) => {
                    rest.next();
                    by.replacen("by ", &format!("by {name} "), 1)
                }
                _ if !key.is_empty() => by.replacen("by ", &format!("by {key} "), 1),
                _ => by,
            };
            let mut by_sort = vec![format!("sorts {named}")];
            by_sort.extend(rest.map(evidence_text));
            if sorted.left_out > 0 {
                by_sort.push(format!(
                    "{} more sorted: {}",
                    sorted.left_out,
                    a.show_command(&row.id)
                ));
            }
            parts.push(by_sort);
        }
        for touched in &row.matched {
            parts.push(touched.shows.iter().map(evidence_text).collect());
        }
        let mut need: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for part in &parts {
            let mut in_part: std::collections::HashMap<&str, usize> =
                std::collections::HashMap::new();
            for s in part {
                *in_part.entry(s.as_str()).or_default() += 1;
            }
            for (s, n) in in_part {
                let most = need.entry(s).or_default();
                *most = (*most).max(n);
            }
        }
        let mut printed: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        let mut shows: Vec<String> = Vec::new();
        for s in parts.iter().flatten() {
            let done = printed.entry(s.as_str()).or_default();
            if *done < need[s.as_str()] {
                *done += 1;
                shows.push(s.clone());
            }
        }
        if !shows.is_empty() {
            line(format!("            {}", shows.join(" · ")));
        }
        for touched in row.matched.iter().filter(|t| t.left_out > 0) {
            line(format!(
                "            {} more of term {}: {}",
                touched.left_out,
                touched.path,
                a.show_command(&row.id)
            ));
        }
        line(format!("            id {}", row.id));
    }
    match &a.view {
        ViewOut::Rows(rows) if rows.left_out > 0 => line(format!(
            "more    {} of {} shown: {}",
            if rows.returned == 0 {
                "none".to_string()
            } else {
                format!("rows {}–{}", rows.after + 1, rows.after + rows.returned)
            },
            a.total.matched,
            match &rows.next {
                Some(next) => next.command(),
                None => "a larger --limit returns the rest".to_string(),
            }
        )),
        ViewOut::Rows(_) => {}
        ViewOut::Counts(counts) => {
            if let Some(sum) = &counts.sum {
                line(format!("sum     {}", sum_of_text(sum)));
            }
            for table in &counts.tables {
                table_text(table, a.scope.realm == Realm::All)
                    .into_iter()
                    .for_each(&mut line);
            }
        }
        ViewOut::Cross(cross) => {
            if let Some(sum) = &cross.sum {
                line(format!("sum     {}", sum_of_text(sum)));
            }
            cross_text(cross, a.scope.realm == Realm::All)
                .into_iter()
                .for_each(&mut line);
        }
        // one node, forced each way: the words are literal — forced,
        // never removed, never a contribution (`answer.rs`)
        ViewOut::Explain(explain) => {
            let counted = |matched: usize, undecided: usize| {
                let mut out = plural(matched, "match", "matches");
                if undecided > 0 {
                    out.push_str(&format!(" · {undecided} undecided"));
                }
                out
            };
            line(format!("explain {}  {}", explain.path, explain.node));
            line(format!(
                "  as given      {}",
                counted(explain.given.matched, explain.given.undecided)
            ));
            for (label, forced) in [
                ("forced true ", &explain.forced_true),
                ("forced false", &explain.forced_false),
            ] {
                line(format!(
                    "  {label}  {}",
                    counted(forced.matched.count, forced.undecided.count)
                ));
                line(format!("                {}", forced.query.text));
            }
        }
    }

    if let Some(zero) = &a.zero {
        for nothing in &zero.resolved_to_nothing {
            line(format!(
                "nothing in scope carries the {} of term {}: {} — {}",
                nothing.of,
                nothing.path,
                nothing.term,
                match nothing.note {
                    Some(note) => note,
                    // an id that left: its term has said so
                    None if nothing.of == "id" => "it left the scope, as its term says",
                    None => zero.said,
                }
            ));
            for s in &nothing.suggestions {
                line(format!(
                    "            {}   ({})",
                    s.term,
                    plural(s.items, "item", "items")
                ));
            }
        }
        // what the zero answer offers, each a command and never its
        // output: explain of each part, then the corpus
        let offers = &zero.offers;
        if !offers.explain.is_empty() {
            line(
                "offers  each part of the query forced true and forced false, the part fewest items match first"
                    .to_string(),
            );
            for offer in &offers.explain {
                line(format!("    {}", offer.request.command()));
            }
            if offers.explain_left_out > 0 {
                line(format!(
                    "            {} more: --explain <path>, a part's path as the terms block numbers it",
                    plural(offers.explain_left_out, "part", "parts")
                ));
            }
        }
        if let Some(offer) = &offers.context {
            line("        every item in scope, this query set aside".to_string());
            line(format!("    {}", offer.request.command()));
        }
    }

    // why items are undecided, one line per reason with an example (V8)
    for (i, reason) in a.total.undecided_reasons.iter().enumerate() {
        line(format!(
            "{}{} unread — e.g. {} · id {}",
            if i == 0 { "undecided " } else { "          " },
            reason.reason.unread,
            reason.example.name.as_deref().unwrap_or("(no name)"),
            reason.example.id
        ));
        line(format!(
            "            {}; {}",
            reason.reason.problem, reason.reason.hint
        ));
    }
    if a.total.reasons_left_out > 0 {
        line(format!(
            "          {} more: the undecided route below lists every item with its reasons",
            plural(a.total.reasons_left_out, "reason", "reasons")
        ));
    }

    if !a.terms.is_empty() {
        line(format!(
            "terms   each term counted alone over all {} {} in all leagues, never over the matches",
            match scope.membership {
                Membership::Live => "live",
                Membership::All => "live and removed",
                Membership::Removed => "removed",
            },
            match &scope.realm {
                Realm::All => "items of every realm".to_string(),
                Realm::One(realm) => format!("{realm} items"),
            }
        ));
        let width = a
            .terms
            .iter()
            .map(|t| t.term.chars().count())
            .max()
            .unwrap_or(0)
            .min(60);
        // a path as long as its column ran into its term, `0.3.0pseudo.…`
        // (the trade seat: cli-opus call 13)
        let path_width = path_column(a.terms.iter().map(|t| t.path.as_str()));
        for term in &a.terms {
            let mut counts: Vec<String> = [
                (&term.matched, "matched"),
                (&term.failed, "failed"),
                (&term.lacked, "lacked"),
                (&term.undecided, "undecided"),
            ]
            .iter()
            .filter(|(count, word)| count.count > 0 || *word == "matched")
            .map(|(count, word)| format!("{} {word}", count.count))
            .collect();
            if let Some(together) = term.together.as_ref().filter(|t| t.count.count > 0) {
                counts.push(format!(
                    "{} {} {} only together",
                    together.count.count,
                    if together.count.count == 1 {
                        "reaches"
                    } else {
                        "reach"
                    },
                    number(&together.bound)
                ));
            }
            line(format!(
                "  {:<path_width$}{:<width$}  {}",
                term.path,
                term.term,
                counts.join(" · ")
            ));
            if let Some(removed) = &term.removed {
                line(format!(
                    "       this id is {} {} that left the scope {}{}",
                    if removed.what == "item" { "an" } else { "a" },
                    removed.what,
                    ago(now, Some(removed.removed_at)),
                    match &removed.route {
                        Some(route) => format!(": {}", route.command()),
                        None => " and held no item".to_string(),
                    }
                ));
            }
            if let Some(resolved) = &term.resolved {
                // the five most carried, where the JSON lists ten and a
                // count lists the rest; every pick of a category term,
                // which no count lists (`category.rs`): a cut there would
                // have no route to what it left out
                let shown = if resolved.of == "category" {
                    resolved.values.len()
                } else {
                    5
                };
                let mut values: Vec<String> = resolved
                    .values
                    .iter()
                    .take(shown)
                    .map(|v| format!("{} ({})", v.value.replace('\n', "\\n"), v.items))
                    .collect();
                let more = resolved.more + resolved.values.len().saturating_sub(shown);
                if more > 0 {
                    // the count that lists every value, where one does
                    values.push(match &resolved.rest {
                        Some(rest) => format!("{more} more: {}", rest.command()),
                        None => format!("{more} more"),
                    });
                }
                if !values.is_empty() {
                    line(format!("       → {}", values.join(" · ")));
                }
            }
        }
    }

    // routes: the ones the answer carries — by default those a next step
    // hangs on (`answer.rs`, C115), under `--routes` every count's — the
    // undecided, the together and the unplaced first; a count that carries
    // none is counted as left to `--routes`
    let mut first: Vec<(String, &Count)> = Vec::new();
    let mut rest: Vec<(String, &Count)> = Vec::new();
    // what an ambiguous clause's other reading would change (C119)
    let clauses = typed.trade.iter().flat_map(|t| &t.clauses);
    for (c, o) in clauses.filter_map(|c| c.other.as_ref().map(|o| (c, o))) {
        for (what, count) in [("adds", &o.adds), ("drops", &o.drops)] {
            first.push((format!("clause {} read the other way, {what}", c.at), count));
        }
    }
    first.push(("undecided at the root".to_string(), &a.total.undecided));
    for term in &a.terms {
        first.push((format!("term {} undecided", term.path), &term.undecided));
        if let Some(together) = &term.together {
            first.push((format!("term {} only together", term.path), &together.count));
        }
    }
    for term in &a.terms {
        let matched = (format!("term {} matched", term.path), &term.matched);
        if a.total.matched == 0 {
            first.push(matched)
        } else {
            rest.push(matched)
        }
        rest.push((format!("term {} failed", term.path), &term.failed));
        rest.push((format!("term {} lacked", term.path), &term.lacked));
    }
    let mut buckets: Vec<(String, &Count)> = Vec::new();
    match &a.view {
        ViewOut::Rows(_) => {}
        ViewOut::Explain(explain) => {
            for (label, forced) in [
                ("forced true", &explain.forced_true),
                ("forced false", &explain.forced_false),
            ] {
                first.push((format!("{label}, matched"), &forced.matched));
                first.push((format!("{label}, undecided"), &forced.undecided));
            }
        }
        ViewOut::Counts(counts) => {
            for table in &counts.tables {
                for bucket in &table.buckets {
                    let what = format!(
                        "{} {}",
                        table.key,
                        label_text(&bucket.label, a.scope.realm == Realm::All)
                    );
                    for kind in bucket.sources.iter().chain(&bucket.flags) {
                        buckets.push((format!("{what}, {}", kind.kind), &kind.count));
                    }
                    buckets.push((what, &bucket.count));
                }
            }
        }
        ViewOut::Cross(cross) => {
            for cell in &cross.cells {
                let of: Vec<String> = cell
                    .of
                    .iter()
                    .map(|l| label_text(l, a.scope.realm == Realm::All))
                    .collect();
                buckets.push((of.join(" × "), &cell.count));
            }
            for margin in &cross.margins {
                first.push((format!("{} (none)", margin.key), &margin.none));
                first.push((format!("{} (undecided)", margin.key), &margin.undecided));
            }
        }
    }
    // what a count could not place is what a next step hangs on
    for (what, count) in buckets {
        if what.ends_with("(none)") || what.ends_with("(undecided)") {
            first.push((what, count));
        } else {
            rest.push((what, count));
        }
    }
    let routed = |routes: Vec<(String, &'_ Count)>| -> Vec<(String, usize, String)> {
        routes
            .into_iter()
            .filter_map(|(what, count)| {
                count
                    .route
                    .as_ref()
                    .map(|r| (what, count.count, r.command()))
            })
            .collect()
    };
    let hidden = first
        .iter()
        .chain(&rest)
        .filter(|(_, count)| count.count > 0 && count.route.is_none())
        .count();
    let (first, rest) = (routed(first), routed(rest));
    let shown: Vec<&(String, usize, String)> = first.iter().chain(&rest).collect();
    if !shown.is_empty() || hidden > 0 {
        line(if hidden > 0 {
            format!("routes  each returns exactly the members counted ({hidden} more: --routes)")
        } else {
            "routes  each returns exactly the members counted".to_string()
        });
        for (what, count, command) in shown {
            line(format!("  {what} ({count}), over the scope:"));
            line(format!("    {command}"));
        }
    }
    // the basis last: what the answer was computed from, read when it is
    // asked about, never before the answer (C115)
    line(basis_text(&a.basis));
    out
}

// ---- a count's text ------------------------------------------------------------------------------------

fn json_text(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(text) => text.replace('\n', "\\n"),
        other => other.to_string(),
    }
}

/// A bucket's name: its value, or `(none)`, `(undecided)` — in brackets,
/// since a tab may be named `none`; a tab's league and id beside its
/// name, and its realm where the scope spans realms.
fn label_text(label: &Label, all_realms: bool) -> String {
    let mut out = match (label.bucket, &label.value) {
        ("value", Some(value)) => json_text(value),
        ("value", None) => "(no name)".to_string(),
        ("computed", Some(value)) => format!("{} (computed)", json_text(value)),
        (bucket, _) => format!("({bucket})"),
    };
    if let Some(realm) = label.realm.as_ref().filter(|_| all_realms) {
        out.push_str(&format!(" · {realm}"));
    }
    if let Some(id) = &label.id {
        out.push_str(&format!(
            " · {} · {id}",
            label.league.as_deref().unwrap_or("no league")
        ));
    }
    out
}

fn summed_text(sum: &Summed) -> String {
    let mut out = number(&sum.value);
    if sum.lacking > 0 {
        out.push_str(&format!(" · {} lacking", sum.lacking));
    }
    if sum.incomplete {
        out.push_str(&format!(" · incomplete: {} unread", sum.unread));
    }
    out
}

fn sum_of_text(sum: &SumOf) -> String {
    format!("{} over every match: {}", sum.name, summed_text(&sum.total))
}

fn bucket_text(bucket: &Bucket, all_realms: bool) -> Vec<String> {
    let mut first = format!(
        "  {:>7}  {}",
        bucket.count.count,
        label_text(&bucket.label, all_realms)
    );
    if let Some(sum) = &bucket.sum {
        first.push_str(&format!("   sum {}", summed_text(sum)));
    }
    let slots: Vec<String> = bucket
        .slots
        .iter()
        .map(|s| {
            let end = |n: &Option<serde_json::Value>| n.as_ref().map_or("?".to_string(), number);
            format!(
                "{} {}..{}{}",
                s.slot,
                end(&s.min),
                end(&s.max),
                if s.incomplete { " (incomplete)" } else { "" }
            )
        })
        .collect();
    if !slots.is_empty() {
        first.push_str(&format!("   {}", slots.join(" · ")));
    }
    if !bucket.tally.is_empty() {
        let tally: Vec<String> = bucket
            .tally
            .iter()
            .map(|t| format!("{} unread {}", t.unread, t.items))
            .collect();
        first.push_str(&format!("   {}", tally.join(" · ")));
    }
    let mut out = vec![first];
    // a vocabulary row carries the term that selects it, ready to paste
    if !bucket.sources.is_empty() {
        if let Some(term) = &bucket.label.term {
            out.push(format!("           {term}"));
        }
        let kinds: Vec<String> = bucket
            .sources
            .iter()
            .chain(&bucket.flags)
            .map(|k| format!("{} {}", k.kind, k.count.count))
            .collect();
        out.push(format!("           {}", kinds.join(" · ")));
        // a kind with no route says why
        for k in bucket.sources.iter().chain(&bucket.flags) {
            if let Some(needs) = &k.needs {
                out.push(format!("           {}: {needs}", k.kind));
            }
        }
    }
    if let Some(needs) = &bucket.label.needs {
        out.push(format!("           {needs}"));
    }
    out
}

fn table_text(table: &Table, all_realms: bool) -> Vec<String> {
    let (one, many) = if table.key.starts_with("line") {
        ("template", "templates")
    } else {
        ("value", "values")
    };
    let mut head = format!(
        "count   {} · {}",
        table.key,
        plural(table.values, one, many)
    );
    if table.left_out > 0 {
        head.push_str(&format!(
            " · {} shown: a larger --limit returns the rest",
            table.values - table.left_out
        ));
    }
    if table.computed > 0 {
        head.push_str(&format!(
            " · {} computed",
            plural(table.computed, "value", "values")
        ));
        if table.computed_left_out > 0 {
            head.push_str(&format!(
                " · {} shown: a larger --limit returns the rest",
                table.computed - table.computed_left_out
            ));
        }
    }
    std::iter::once(head)
        .chain(
            table
                .buckets
                .iter()
                .flat_map(|b| bucket_text(b, all_realms)),
        )
        .collect()
}

fn cross_text(cross: &CrossOut, all_realms: bool) -> Vec<String> {
    let mut head = format!(
        "cross   {} · {}",
        cross.keys.join(" × "),
        plural(cross.cells_in_all, "cell", "cells")
    );
    if cross.left_out > 0 {
        head.push_str(&format!(
            " · {} shown: a larger --limit returns the rest",
            cross.cells_in_all - cross.left_out
        ));
    }
    let mut out = vec![head];
    for cell in &cross.cells {
        let of: Vec<String> = cell.of.iter().map(|l| label_text(l, all_realms)).collect();
        let mut row = format!("  {:>7}  {}", cell.count.count, of.join(" × "));
        if let Some(sum) = &cell.sum {
            row.push_str(&format!("   sum {}", summed_text(sum)));
        }
        out.push(row);
        // a cell with no route says why, once per value that has none
        for label in &cell.of {
            if let Some(needs) = &label.needs {
                out.push(format!(
                    "           {}: {needs}",
                    label_text(label, all_realms)
                ));
            }
        }
    }
    for margin in &cross.margins {
        if margin.none.count + margin.undecided.count > 0 {
            out.push(format!(
                "           {}: {} none · {} undecided",
                margin.key, margin.none.count, margin.undecided.count
            ));
        }
    }
    if !cross.tally.is_empty() {
        let tally: Vec<String> = cross
            .tally
            .iter()
            .map(|t| format!("{} unread {}", t.unread, t.items))
            .collect();
        out.push(format!("           undecided: {}", tally.join(" · ")));
    }
    out
}

/// The head of a definition: what a computed value is, before how it is
/// computed — its text up to the first `: `, which every total's and
/// derived value's definition has after its name on the site.
fn definition_head(what: &str) -> &str {
    what.split_once(": ").map_or(what, |(head, _)| head)
}

/// `--describe` as text. `index` is the bare form (C53: more than ten
/// are counted, never listed whole): the computed values and the
/// categories, nine tenths of the page, are one line each, and the last
/// line says how to read one whole.
fn describe_text(d: &Describe, index: bool) -> String {
    let mut out = String::new();
    let mut block = |head: &str, entries: &[acquisition_search::describe::Named], brief: bool| {
        if entries.is_empty() {
            return;
        }
        out.push_str(&format!("{head}\n"));
        // the name column fits the block's longest name: a computed value's
        // is `pseudo.total_lightning_gem_levels`
        let name = entries
            .iter()
            .map(|e| e.name.chars().count() + 2)
            .max()
            .unwrap_or(0)
            .max(16);
        let kind = entries
            .iter()
            .map(|e| e.kind.chars().count() + 2)
            .max()
            .unwrap_or(0)
            .max(12);
        for e in entries {
            if brief {
                out.push_str(&format!(
                    "  {:<name$}{:<kind$}{}\n",
                    e.name,
                    e.kind,
                    definition_head(&e.what)
                ));
                continue;
            }
            out.push_str(&format!("  {:<name$}{:<kind$}{}\n", e.name, e.kind, e.what));
            if !e.values.is_empty() {
                out.push_str(&format!(
                    "  {:<width$}one of: {}\n",
                    "",
                    e.values.join(", "),
                    width = name + kind
                ));
            }
            if !e.examples.is_empty() {
                out.push_str(&format!(
                    "  {:<width$}e.g. {}\n",
                    "",
                    e.examples.join("   "),
                    width = name + kind
                ));
            }
        }
    };
    block("fields", &d.fields, false);
    block("inside line( … )", &d.line, false);
    block("inside linked( … )", &d.linked, false);
    block("values", &d.values, false);
    block("composition", &d.composition, false);
    block("operators", &d.operators, false);
    block("slots", &d.slots, false);
    // the totals' table is named once, over the block, with its version
    // and source (C106's clause (c))
    if index {
        block(
            "computed values — one line each; `--describe <name>` is its definition",
            &d.computed,
            true,
        );
    } else {
        block(
            &format!("computed values — {}", d.totals),
            &d.computed,
            false,
        );
    }
    block("counts", &d.counts, false);
    block("why an answer is what it is", &d.explain, false);
    // every category id with the site's text for it and what makes it
    // hold (C106)
    if !d.categories.is_empty() {
        out.push_str("categories — the trade site's ids, as spelled\n");
        for c in &d.categories {
            out.push_str(&format!("  {:<30}{}\n", c.id, c.text));
            if !index {
                out.push_str(&format!("  {:<30}  {}\n", "", c.holds));
            }
        }
    }
    // the catalogue is a block of its own: the index names it
    if let Some(trade) = &d.trade {
        if index {
            out.push_str(&format!(
                "trade — what `--trade` makes of each clause a site search can carry, {} rows: `--describe trade`\n",
                trade.rows.len()
            ));
        } else {
            out.push_str(&format!(
                "trade — the catalogue v{}, the site's stats of {}: each clause a site search can carry, its status when it runs, and the text it becomes\n",
                trade.version, trade.stats
            ));
            let key = trade
                .rows
                .iter()
                .map(|r| r.key.chars().count() + 2)
                .max()
                .unwrap_or(0);
            for r in &trade.rows {
                out.push_str(&format!(
                    "  {:<key$}{:<14}{}\n",
                    r.key,
                    r.status,
                    r.lands.as_deref().unwrap_or("—")
                ));
                for said in r.text.iter().chain(&r.says) {
                    out.push_str(&format!("  {:<key$}{:<14}{said}\n", "", ""));
                }
                if let Some(other) = &r.other {
                    out.push_str(&format!(
                        "  {:<key$}{:<14}read the other way: {other}\n",
                        "", ""
                    ));
                }
            }
        }
    }
    if !d.not_built.is_empty() {
        out.push_str("not built, refused by name\n");
        for n in &d.not_built {
            out.push_str(&format!(
                "  {:<22}{:<32}{}\n",
                n.construct,
                n.step_text(),
                n.what
            ));
        }
    }
    if !d.limits.is_empty() {
        out.push_str("limits, as the search states them\n");
        for l in &d.limits {
            out.push_str(&format!("  {:<6}{}\n", l.id, l.said));
        }
    }
    if index {
        out.push_str(
            "more: this is the index — `--describe <name>,…` prints an entry whole (a computed \
             value's definition, what makes a category hold), `--describe computed` or \
             `--describe category` a block, `--describe all` everything\n",
        );
    }
    out
}

fn shown_text(s: &Shown) -> String {
    let item = &s.item;
    let mut out = String::new();
    let mut head = title(item.name.as_deref(), item.typeline.as_deref());
    for (label, value) in [("", &item.rarity), ("frame ", &item.frame)] {
        if let Some(value) = value {
            head.push_str(&format!(" · {label}{}", value.to_lowercase()));
        }
    }
    out.push_str(&format!("{head}\n"));
    out.push_str(&format!("id      {}\n", item.facts.id));
    if let Some(removed_at) = item.facts.removed_at {
        out.push_str(&format!(
            "removed {}: no longer in the stash, shown as it was last seen\n",
            ago(acquisition_store::now(), Some(removed_at))
        ));
    }
    out.push_str(&format!(
        "place   {} · {}\n",
        s.place.realm,
        place_text(&s.place, acquisition_store::now())
    ));
    if let Some(base) = &item.base {
        out.push_str(&format!("base    {base}\n"));
    }
    // what the class table says (C106): the class, or why there is none
    use acquisition_search::Classed;
    match &s.class {
        Classed::Is(class) => out.push_str(&format!("class   {class}\n")),
        Classed::All(classes) => out.push_str(&format!(
            "class   {} (each: the table lists the base under both)\n",
            classes.join(", ")
        )),
        Classed::Among { why, .. } | Classed::Open(why) => {
            out.push_str(&format!("class   undecided: {}\n", why.problem));
        }
        Classed::BaseUnread => out.push_str("class   undecided: the base was not read\n"),
        Classed::FrameUnread { .. } => out.push_str(
            "class   undecided: the frame, which decides an invitation's class, was not read\n",
        ),
        Classed::BestiaryUnread { candidates, .. } => out.push_str(&format!(
            "class   undecided: the bestiary line, which tells a captured beast, was not read — {}\n",
            candidates.join(" or ")
        )),
    }
    // the category (C106): the id or ids a count lists the item under,
    // none, or undecided — and the reason wherever an id is left open
    let category = &s.category;
    out.push_str(&match category.bucket {
        "value" => format!("category {}\n", category.listed.join(", ")),
        other => format!("category {other}\n"),
    });
    for open in &category.open {
        let ids = if open.ids.len() <= 4 {
            open.ids.join(", ")
        } else {
            plural(open.ids.len(), "id", "ids")
        };
        out.push_str(&format!("         open {ids}: {}\n", open.why.problem));
    }
    // what the listing state prices it at (C81): the price, or why none
    match &s.price {
        Priced::Is(p) => out.push_str(&format!(
            "price   {} · {}{}\n",
            p.text,
            p.side,
            p.from
                .as_deref()
                .filter(|from| *from != format!("item/{}", item.facts.id))
                .map(|from| format!(" · from {from}"))
                .unwrap_or_default()
        )),
        Priced::None { kind, .. } => out.push_str(&format!("price   none ({kind})\n")),
        Priced::Open(why) => out.push_str(&format!("price   undecided: {}\n", why.problem)),
    }
    if let Some(stack) = item.stack {
        out.push_str(&format!("stack   {stack}\n"));
    }
    if let Some(note) = &item.note {
        out.push_str(&format!("note    {note}\n"));
    }
    if !item.flags.is_empty() {
        out.push_str(&format!("is      {}\n", item.flags.join(", ")));
    }
    // the sockets as the game shows them, and the largest link group
    if let Some(groups) = acquisition_search::sockets::groups(item) {
        let layout = groups.layout();
        let links = match groups.links() {
            acquisition_search::sockets::Counted::Absent => "no link group".to_string(),
            counted => match (counted.value(), counted.floor()) {
                (Some(n), _) => format!("links {n}"),
                (None, Some(floor)) => format!("links at least {floor}, not established"),
                (None, None) => "links not established".to_string(),
            },
        };
        out.push_str(&format!(
            "sockets {} · {links}\n",
            if layout.is_empty() { "none" } else { &layout }
        ));
    }
    let shown: Vec<&str> = item
        .properties
        .iter()
        .map(|p| p.text.as_str())
        .chain(item.item_level.as_deref())
        .chain(item.requires.as_deref())
        .collect();
    for (i, text) in shown.iter().enumerate() {
        out.push_str(&format!(
            "{}{}\n",
            if i == 0 { "shown   " } else { "        " },
            text.replace('\n', " / ")
        ));
    }
    for (i, l) in s.lines.iter().enumerate() {
        let kind: Vec<&str> = std::iter::once(l.line.source.as_str())
            .chain(l.line.flags.iter().map(String::as_str))
            .collect();
        out.push_str(&format!(
            "{}{:<22}{}\n",
            if i == 0 { "lines   " } else { "        " },
            kind.join(", "),
            l.line.text.replace('\n', " / ")
        ));
        let slots: Vec<String> = l
            .slots
            .iter()
            .map(|(word, n)| format!("{word} {}", number(n)))
            .collect();
        out.push_str(&format!(
            "        {:<22}\"{}\"{}\n",
            "",
            l.line.template.replace('\n', "\\n"),
            if slots.is_empty() {
                String::new()
            } else {
                format!("   {}", slots.join(" · "))
            }
        ));
        // a mod of several rows: each row is a name of it, and names its
        // own numbers
        for row in &l.line.rows {
            let slots: Vec<String> = row
                .read()
                .slots()
                .into_iter()
                .map(|(word, n)| format!("{word} {}", slot_number(n)))
                .collect();
            out.push_str(&format!(
                "        {:<22}row \"{}\"{}\n",
                "",
                row.template,
                if slots.is_empty() {
                    String::new()
                } else {
                    format!("   {}", slots.join(" · "))
                }
            ));
        }
        if let Some(limit) = l.limit {
            out.push_str(&format!("        {:<22}{limit}\n", ""));
        }
    }
    if item.unread.is_empty() {
        out.push_str("unread  nothing\n");
    }
    for (i, u) in item.unread.iter().enumerate() {
        out.push_str(&format!(
            "{}{}\n",
            if i == 0 { "unread  " } else { "        " },
            u.problem
        ));
    }
    // why the item does or does not match a query: the root's outcome,
    // then each node's, a term with what it touched or why it is open
    if let Some(against) = &s.against {
        let query = if against.query.text.is_empty() {
            "(every item in scope)"
        } else {
            &against.query.text
        };
        out.push_str(&format!("against {query}\n"));
        out.push_str(&format!("        {}\n", against.outcome.replace('_', " ")));
        let width = against
            .nodes
            .iter()
            .map(|n| n.node.chars().count())
            .max()
            .unwrap_or(0)
            .min(60);
        let path_width = path_column(against.nodes.iter().map(|n| n.path.as_str()));
        for node in &against.nodes {
            out.push_str(&format!(
                "  {:<path_width$}{:<width$}  {}\n",
                node.path,
                node.node,
                node.outcome.replace('_', " ")
            ));
            if !node.shows.is_empty() {
                let shows: Vec<String> = node.shows.iter().map(evidence_text).collect();
                out.push_str(&format!("            {}\n", shows.join(" · ")));
            }
            if node.left_out > 0 {
                out.push_str(&format!(
                    "            {} more: the item whole is above\n",
                    node.left_out
                ));
            }
        }
    }
    out.push_str(&format!("{}\n", basis_text(&s.basis)));
    if let Some(body) = &s.body {
        out.push_str(&format!("body    {body}\n"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::keys;

    /// The count list's grammar, both ways: every spelling a continuation
    /// prints (`answer::listed_text`) reads back as the key it encodes,
    /// and what `--count`'s help shows reads as it says.
    #[test]
    fn a_count_list_reads_back_what_a_continuation_prints() {
        for (typed, read) in [
            ("tab,league,rarity", vec!["tab", "league", "rarity"]),
            ("line:resist,life", vec!["line:resist", "line:life"]),
            (
                "tab,line:resist,~^adds",
                vec!["tab", "line:resist", "line~^adds"],
            ),
            ("line:~Life", vec!["line~Life"]),
            // unquoted, the comma is the list's: the review's case
            ("line~[a-z]{1,2}", vec!["line~[a-z]{1", "line:2}"]),
            (r#"line~"[a-z]{1,2}""#, vec!["line~[a-z]{1,2}"]),
            (r#"line:", half""#, vec!["line:, half"]),
            (r#"line:"a\"b""#, vec![r#"line:a"b"#]),
            (r#"line:"a\\b""#, vec![r"line:a\b"]),
            (r#"line:"~x""#, vec!["line:~x"]),
            (r#"line:"Life ""#, vec!["line:Life "]),
            (
                r#"line:life , "two words" "#,
                vec!["line:life", "line:two words"],
            ),
            (r#"line:"a\nb""#, vec!["line:a\nb"]),
        ] {
            assert_eq!(keys(typed).unwrap(), read, "{typed}");
        }
        for bad in [r#"line:"Life"#, r#"line:"Life\q""#, r#"line:"Life\"#] {
            assert!(keys(bad).is_err(), "{bad}");
        }
    }

    /// The printer and this parser are two crates' and one grammar: what
    /// `Counts::lists` spells for a view, read back list by list, is the
    /// view's keys as the binder names them — for every pair of keys from
    /// an alphabet of awkward ones, and every triple from a part of it. A
    /// prefix in any case, a text that begins like a key or a pattern, a
    /// comma, a quote, a backslash, a row break, whitespace at an end; a
    /// field or `line` alone before, between and after narrowings. Two
    /// rounds of an outside audit (2026-10-10) each found a case the
    /// printer's own case list had not: this asks the grammar, not a list.
    #[test]
    fn every_count_view_reads_back_as_the_keys_it_was_printed_for() {
        use acquisition_search::answer::Counts;
        let texts = [
            "life",
            "~x",
            "~",
            "line:foo",
            "LINE~bar",
            "line",
            "a,b",
            ",",
            "\"q\"",
            "back\\slash",
            " lead",
            "trail ",
            "two words",
            "a\nb",
            "it's",
            "x:y",
            "x~y",
        ];
        // each key, and what the binder calls it: a name in its own case,
        // the text after it as written
        let mut alphabet: Vec<(String, String)> = [
            ("tab", "tab"),
            ("Rarity", "rarity"),
            ("LINE", "line"),
            ("line", "line"),
            ("tab.type", "tab.type"),
        ]
        .iter()
        .map(|(typed, named)| (typed.to_string(), named.to_string()))
        .collect();
        for text in texts {
            for (prefix, canonical) in [
                ("line:", "line:"),
                ("line~", "line~"),
                ("LINE:", "line:"),
                ("Line~", "line~"),
            ] {
                alphabet.push((format!("{prefix}{text}"), format!("{canonical}{text}")));
            }
        }
        let check = |picked: &[&(String, String)]| {
            let view = Counts {
                keys: picked.iter().map(|(typed, _)| typed.clone()).collect(),
                sum: None,
                limit: None,
            };
            let lists = view.lists();
            let mut read = Vec::new();
            for list in &lists {
                read.extend(keys(list).unwrap_or_else(|e| panic!("{lists:?}: {e}")));
            }
            let named: Vec<&str> = picked.iter().map(|(_, named)| named.as_str()).collect();
            assert_eq!(read, named, "{:?} printed as {lists:?}", view.keys);
        };
        for a in &alphabet {
            check(&[a]);
            for b in &alphabet {
                check(&[a, b]);
            }
        }
        // every triple over a part of it: each kind of key, and the texts
        // that read as syntax
        let part: Vec<&(String, String)> = alphabet
            .iter()
            .filter(|(typed, _)| {
                [
                    "tab",
                    "LINE",
                    "line:life",
                    "LINE:life",
                    "line~~x",
                    "line:line:foo",
                    "Line~LINE~bar",
                    "line:a,b",
                    "line:~",
                    "line: lead",
                    "line:line",
                ]
                .contains(&typed.as_str())
            })
            .collect();
        assert_eq!(part.len(), 11);
        for a in &part {
            for b in &part {
                for c in &part {
                    check(&[a, b, c]);
                }
            }
        }
    }
}
