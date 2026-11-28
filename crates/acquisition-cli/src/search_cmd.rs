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
//!   `{"error", "kind"}`, a language error's `readings` and each way on
//!   under `offers` — a search a request, as `--request` reads one,
//!   saying its command — and exits 1; the text prints the message and
//!   each reading or way on on a line of its own, ready to paste.
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
//!   requests (`printed.rs`), each run by the test that runs every
//!   command printed.
//! - **The query may come from a file or stdin** (`--query-file`, the
//!   plan's gap 2): a template with an apostrophe cannot be typed plainly
//!   inside a shell's single quotes. A route is printed shell-quoted.
//! - **`--trade` and `--trade-file` are a query's third spelling**
//!   (C113): the site search is the request's `query.trade`, translated by
//!   the crate at every read (`answer.rs`), and `--print-request` writes
//!   the text it translates to beside it. `--realm` beside a URL is K6's
//!   conflict: naming the URL's own realm, a terminal's alone, it is said
//!   here before anything is read; naming another, it is the request's
//!   conflict, a scope beside a URL being the URL's, and the crate refuses
//!   it in the words every surface gets (C143). The text opens with the
//!   translation, its remainder loud (`trade_text`).
//! - **The intent file is opened beside the store** (the layer's `Account::open`,
//!   as `acq price` opens it): the effective price a query reads is the
//!   listing state's (C81, C100), and every basis names the intent
//!   revision (C98). Nothing is written to it.

use crate::printed::Printed as _;
use std::io::Read as _;

use acquisition_search::answer::{
    Answer, Asked, Context, Count, Membership, Request, RowField, Rows, Scope, Shape, TradeOut,
    View, ViewOut,
};
use acquisition_search::corpus::Realm;
use acquisition_search::counts::{Bucket, CrossOut, Table};
use acquisition_search::describe::Describe;
use acquisition_search::show::Shown;
use acquisition_search::{Priced, SearchError, answer, describe, not_built, show};
use acquisition_store::Store;
use anyhow::{Context as _, Result};
use serde::Serialize;

use crate::run::Ctx;

use acquisition_search::answer::trade_realm;
use acquisition_search::describe::{asks_totals, describe_list};
use acquisition_search::words::{
    BLOCKS, Block, CONTEXT_OFFER, EXPLAIN_OFFERS, ItemParts, Line, Part, REMOVED_HELD_NONE,
    RowPiece, TRADE_OVER, UNREAD_NOTHING, aside_words, basis_text, before_words, carried_words,
    category_words, class_words, clause_note, clauses_listed, counted, cross_head_words,
    cross_tally_words, example_name, explain_words, fetched_words, fidelity_words, flags_words,
    forced_words, kind_words, label_text, line_kind_words, line_row_quoted, line_row_slots_words,
    line_slots_words, line_template_quoted, link_words, more_of_term_words, more_of_value_words,
    more_parts_words, more_sorted_words, more_words, needs_words, never_fetched_sentence,
    never_fetched_words, node_left_out_words, node_shows_words, none_words, nothing_words,
    open_words, order_words, other_leagues_words, outcome_words, pattern_said, price_words,
    query_words, realm_not_held_words, reason_words, reasons_left_out_words, removed_words,
    row_shows, rows_shown_words, scope_words, shown_place_words, shown_removed_words, shown_title,
    slots_words, sockets_words, stake_words, suggestion_words, sum_of_text, sum_words,
    table_head_words, tally_words, terms_words, together_words, total_words, trade_said,
    undecided_words, unmatched_words,
};

/// What `acq search` was typed with.
#[derive(clap::Args)]
pub struct SearchArgs {
    /// The query: `class:helmets rarity=rare`, `line("# to maximum
    /// Life").arg1>=70`, `pseudo.total_res>=80`; `acq search --describe
    /// fields` is the place to start, and bare `--describe` the whole
    /// index. None: every item in scope. One that starts with `-`, the
    /// language's not, takes a space inside its quotes, as every route
    /// prints it: `acq search --realm pc ' -is:corrupted'`.
    pub query: Option<String>,
    /// The realm searched: pc, xbox, sony, poe2, or all. Over a store
    /// holding one realm it may be omitted and the answer prints it; over
    /// several it is required, one of them or `all`.
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
    /// faithful the translation is, its link back to the site for a
    /// person to open (where it names a league), and every clause it
    /// could not run;
    /// the clauses, each with its status, are its JSON's `query.trade`,
    /// and `--describe trade` is what becomes of each. The site's sort is
    /// reported, never applied. PoE1 only.
    #[arg(long, value_name = "URL|ID", conflicts_with_all = ["query", "query_file"])]
    pub trade: Option<String>,
    /// `--trade`, read from a file or from stdin with `-`: the query JSON
    /// as the site posts it, `{"query": …, "sort": …}` or the query
    /// object alone, needs no shell quoting this way. A `"league"` beside
    /// the query is the search's league, a term as a URL's league is, and
    /// `"defences": "displayed"` reads the Armour, Evasion, Energy Shield
    /// and Ward filters as the item stands, where the site reads them at
    /// 20 % quality.
    #[arg(long, value_name = "FILE|-", conflicts_with_all = ["query", "query_file", "trade"])]
    pub trade_file: Option<String>,
    /// Order the rows by a value: `ilvl`, `stack`, `'line("T").arg1'`,
    /// `'sum("T")'`, `pseudo.total_res`. A line's scalar is its largest
    /// satisfying occurrence; an item with none sorts last either way. A
    /// sum spans every source the line has — implicit and crafted with
    /// explicit — as the row's `sorts by` shows; one source's is its
    /// form, `'sum(line("T" source=explicit).arg1)'`, the filter's own.
    /// None named, the rows stand as they lie in their place, top row
    /// first and left to right, which `none` names too.
    #[arg(long, value_name = "VALUE")]
    pub sort: Option<String>,
    /// Largest first.
    #[arg(long, requires = "sort")]
    pub desc: bool,
    /// A value each row carries, as `--sort` takes one; given again, the
    /// next. None given, the rows carry the values the query compares, and
    /// `stack`, a stack's count, where a match has one: at most 12. A sort
    /// the list lacks joins it.
    #[arg(long, value_name = "VALUE")]
    pub value: Vec<String>,
    /// How many rows to return — or, of a count, how many values a table
    /// lists; the rest are counted.
    /// Ten where none is given.
    #[arg(long)]
    pub limit: Option<usize>,
    /// Print every count's route — the command that returns exactly its
    /// members — instead of the few a next step hangs on; under `--json`,
    /// every count's request. Beside `--next` or `--request`, the answer
    /// it runs carries them all.
    #[arg(long)]
    pub routes: bool,
    /// The language as this build knows it. Bare, the index: every entry
    /// — fields, what a line has, values, how terms compose, operators,
    /// slots, computed values, what a count takes, an answer's keys, the
    /// categories, what is not built and the limits stated — by its name
    /// and one line, and the trade catalogue by its count of rows (about
    /// 33 KB). Named, those entries whole, with their definitions, values
    /// and examples (`--describe league,line,pseudo.total_res`; a block's
    /// name is one: `--describe computed`, and `--describe trade` what
    /// `--trade` makes of each clause of a site search; a category's id
    /// is its row: `--describe accessory.ring`). `--describe=~resist` is
    /// a pattern, Rust regex syntax in any case: the entries whose name or
    /// definition it matches, and the categories whose id, text or rule
    /// does — whole when ten or fewer, their lines otherwise, counted per
    /// pattern, so a list of several may answer more whole. After `=` no
    /// shell reads `~` as a home directory; a pattern holding a space or
    /// a character the shell reads is quoted as any argument is. It takes
    /// the rest of its value, commas and all, and a name after it is
    /// another `--describe`. `--describe all` is every entry whole, over 150 KB.
    /// `--json` is the same document.
    #[arg(long, value_name = "NAME,…", num_args = 0..=1)]
    pub describe: Option<Vec<String>>,
    /// Count the matches by a key, one table for each key named, and show
    /// no rows: a field (`tab`, `league`, `rarity`, `base`, …; `--describe
    /// counts` lists them), or `line`, the vocabulary — the templates the
    /// matching items carry, ranked, each with the term that selects it and
    /// the range of its numbers. `line:` takes the rest of the list as
    /// texts to narrow by, a table each, and `~` before one makes it a
    /// pattern: `--count tab,line:resist,life`. An item with no value is
    /// counted under `none`, one whose value could not be read under
    /// `undecided`; every count has its route (`--routes`). Given
    /// again, the flag starts a list of its own, so a field can follow a
    /// narrowed vocabulary: `--count line:resist,life --count rarity`.
    /// `realm` is not a key: `--realm` scopes the search, and a count's
    /// league names its realm.
    #[arg(long, value_name = "KEY,…")]
    pub count: Vec<String>,
    /// Count the matches by two fields at once, one table of the cells
    /// that hold an item: `--cross league,tab`.
    #[arg(long, value_name = "KEY,KEY")]
    pub cross: Option<String>,
    /// Beside each count, the sum of one number over its items: `stack`,
    /// `ilvl`, `'sum("T")'`, `pseudo.total_res`. An item lacking the thing
    /// adds nothing and is counted beside the sum as lacking it, so the sum
    /// is over the items that have it; one unread leaves a subtotal marked
    /// incomplete.
    #[arg(long, value_name = "VALUE")]
    pub sum: Option<String>,
    /// What each row carries beside its compact fields, by name: `icon`,
    /// the URL of the item's art; `item`, the item as `show` gives it —
    /// its lines, class, category and price; each line's `value` and each
    /// key of its `number_fields` is a value `--value` and `--sort` take.
    /// Under `item` an answer holds
    /// at most 200 rows; the rest are its continuation.
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
    /// Print the request as JSON, its account and realm resolved, and do
    /// not run it: what `--request` reads.
    #[arg(long)]
    pub print_request: bool,
    /// Run a request read from a file, or from stdin with `-`: what
    /// `--print-request` printed, a `next` or an offer of an answer's
    /// `--json`, or a count's route — which reports both counts where its
    /// count has moved since, and wherever the basis has where the
    /// answer's `basis` is sent beside it as `counted_at`. A request is
    /// whole: no word of scope, query or view stands beside it, and with
    /// no `--account` the account its scope names selects the store.
    #[arg(long, value_name = "FILE|-")]
    pub request: Option<String>,
    /// Not built, waiting for a consumer that asks: run another account's
    /// request deliberately.
    #[arg(long)]
    pub rebind: bool,
    /// Run a saved search, by its id or its name (`acq saved list`): its
    /// query over its realm and membership, as the account `--account`
    /// names. Its scope and its query are the row's, so no query,
    /// `--query-file`, `--trade`, `--trade-file`, `--realm` or
    /// `--membership` stands beside it; every view flag does. A name
    /// several searches share is refused, each offered by its id.
    #[arg(long, value_name = "ID|NAME")]
    pub saved: Option<String>,
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
        println!(
            "{}",
            acquisition_verbs::refusal::of_search(&e, &crate::terminal::Terminal::this_command())
        );
    } else {
        eprintln!("Error: {}", search_error_text(&e));
    }
    crate::terminal::AlreadyReported.into()
}

/// A search refusal's words: its message, then each reading on a line of
/// its own — what a terminal prints after `Error:` (a page renders the
/// refusal's document, its ways on as links: `web::Paged`).
pub(crate) fn search_error_text(e: &SearchError) -> String {
    match e {
        SearchError::Language(e) => std::iter::once(e.message.clone())
            .chain(e.readings.iter().map(|r| format!("    {r}")))
            .collect::<Vec<_>>()
            .join("\n"),
        SearchError::Scope {
            message, offers, ..
        } => std::iter::once(message.clone())
            .chain(offers.iter().map(|o| match &o.describe {
                // a describe offer prints the command it is
                Some(names) => format!(
                    "    {}: {}",
                    o.reading,
                    crate::terminal::printed_line(&crate::terminal::way_words(
                        &acquisition_verbs::refusal::Way::Describe {
                            names: names.clone()
                        }
                    ))
                ),
                None => format!("    {}", terminal_reading(o)),
            }))
            .collect::<Vec<_>>()
            .join("\n"),
        SearchError::Store(e) => format!("{e:#}"),
    }
}

/// A way on as a terminal reads it (C143): the shared reading, in the
/// request's words, and after a realm's the flag this terminal spells it
/// with; a whole search, the command it is ([`Printed::command`]), which
/// this terminal prints in its place (the shared words are the MCP's
/// and the page's).
///
/// [`Printed::command`]: crate::printed::Printed::command
pub(crate) fn terminal_reading(o: &acquisition_search::error::Offered) -> String {
    match (o.realm(), &o.request) {
        (Some(realm), _) => format!("{} (--realm {realm})", o.reading),
        (None, Some(request)) => request.command(),
        (None, None) => o.reading.clone(),
    }
}

fn emit<T: Serialize>(value: &T, json: bool, text: impl FnOnce(&T) -> String) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(value)?);
    } else {
        print!("{}", text(value));
    }
    Ok(())
}

pub fn search(cx: &Ctx, mut args: SearchArgs, json: bool) -> Result<()> {
    if args.rebind {
        return Err(fail(not_built("rebind").into(), json));
    }
    // the row's fields as typed, each word read by the search's one reader
    let fields = match &args.fields {
        Some(text) => RowField::list(text).map_err(|e| fail(e.into(), json))?,
        None => Vec::new(),
    };
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
        ("--value", !args.value.is_empty()),
        ("--fields", args.fields.is_some()),
        ("--count", !args.count.is_empty()),
        ("--cross", args.cross.is_some()),
        ("--sum", args.sum.is_some()),
        ("--explain", args.explain.is_some()),
        ("--context", args.context.is_some()),
        ("--print-request", args.print_request),
        ("--describe", args.describe.is_some()),
        ("--saved", args.saved.is_some()),
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
        // the envelope as the command runs it: the request inside it with
        // this command line's overrides (`--routes`) applied
        let mut sent = value.clone();
        let run = serde_json::to_value(&asked.request)?;
        match sent.get_mut("request") {
            Some(request) => *request = run,
            None => sent = run,
        }
        let (store, intent) = open_as_asked(cx, &asked.request, &sent, json)?;
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
        let sent = serde_json::to_value(&request)?;
        let (store, intent) = open_as_asked(cx, &request, &sent, json)?;
        let corpus = request.corpus(&store, &intent).map_err(|e| fail(e, json))?;
        let answered = answer(&corpus, &request).map_err(|e| fail(e, json))?;
        return emit(&answered, json, answer_text);
    }
    if let Some(names) = &args.describe {
        if args.print_request {
            whole_describe(json)?;
        }
        // the list as typed: the search crate reads it, bare the index and
        // `all` every entry whole, in text and in JSON alike (C53)
        let names = describe_list(names);
        let described = describe(&names).map_err(|e| fail(e, json))?;
        let totals = asks_totals(&names);
        return emit(&described, json, |d| describe_text(d, totals));
    }
    // a saved search is its scope and its query whole, as a continuation
    // is: a word of either beside it has nowhere to stand; the view and the
    // account are this command's
    if let Some(saved) = args.saved.take() {
        let beside = [
            ("a query", args.query.is_some()),
            ("--query-file", args.query_file.is_some()),
            ("--trade", args.trade.is_some()),
            ("--trade-file", args.trade_file.is_some()),
            ("--realm", args.realm.is_some()),
            ("--membership", args.membership.is_some()),
        ];
        if let Some((word, _)) = beside.iter().find(|(_, typed)| *typed) {
            return Err(fail(
                SearchError::Scope {
                    kind: "view",
                    message: format!(
                        "`--saved` runs a saved search whole, its scope and its query: {word} has nowhere to stand beside it"
                    ),
                    offers: Vec::new(),
                },
                json,
            ));
        }
        let request =
            acquisition_verbs::saved::request_of(&cx.frontend, &saved, cx.account.as_deref())?;
        // the row's source as asked (C194): a text, or a site search,
        // whose URL's realm is the row's and is not said twice
        args.trade = request.query.trade_word();
        args.query = request.query.text;
        args.realm = request
            .scope
            .realm
            .map(|realm| realm.as_str().to_string())
            .filter(|_| args.trade.as_deref().and_then(trade_realm).is_none());
        args.membership = Some(request.scope.membership.as_str().to_string());
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
    // a URL's realm is the scope (K6): the flag beside one naming the same
    // realm says it twice, which only a terminal can, so this refusal is
    // the terminal's; one naming another realm is the request's conflict,
    // refused in the search's words on every surface (C143, the request's
    // words). The realm the search names is read where every reader reads
    // it, and an input that does not read is left to the request to refuse.
    if let (Some(word), Some(trade)) = (&args.realm, &trade)
        && let Some(url) = trade_realm(trade)
        // the realm as parsed, as the request will read it: `PC` is `pc`
        && Realm::parse(word).is_ok_and(|asked| asked.as_str() == url)
    {
        return Err(fail(
            SearchError::Scope {
                kind: "trade",
                message: format!(
                    "`--realm {word}` beside a trade URL says its realm again: the URL's realm, `{url}`, is the search's scope — leave `--realm` out, or give the search's id, which names no realm"
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
            acquisition_search::counts::count_keys(raw).map_err(|message| {
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
            limit: Some(args.limit.unwrap_or(acquisition_store::bound::LISTED)),
            sort: args.sort.clone(),
            values: args.value.clone(),
            desc: args.desc,
            after: None,
            fields,
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
                        message: Membership::unknown(word),
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
    let (store, intent) = open_store_and_intent(cx)?;
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
    let counted = !args.count.is_empty() || args.cross.is_some();
    let limit_beside_count = args.limit.filter(|_| counted);
    emit(&answered, json, |a| answer_text_of(a, limit_beside_count))
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

pub fn show_item(cx: &Ctx, args: ShowArgs, json: bool) -> Result<()> {
    if args.basis.is_some() {
        return Err(fail(not_built("show basis").into(), json));
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
    let account = cx.account()?;
    let opened = account.open()?;
    let (store, intent) = (opened.facts, opened.intent);
    let shown = match &against {
        Some(query) => {
            acquisition_search::show::against(&store, &intent, &args.id, args.body, query)
        }
        None => show(&store, &intent, &args.id, args.body),
    }
    .map_err(|e| fail(e, json))?;
    emit(&shown, json, |shown| {
        shown_text(shown, &account.entry.username)
    })
}

/// The selected account's facts and its intent file, as `acq price` opens
/// them, through the layer (C121): the search reads both and writes
/// neither.
fn open_store_and_intent(cx: &Ctx) -> Result<(Store, acquisition_store::Annotations)> {
    let opened = cx.account()?.open()?;
    Ok((opened.facts, opened.intent))
}

/// [`open_store_and_intent`] for a request read whole (`--request`,
/// `--next`): with no `--account`, the account its scope names selects the
/// store, as the MCP's `scope.account` does — a request is whole, its scope
/// its own, so one an answer or a refusal holds runs as printed with a
/// second account known. An `--account` given still selects, and a request
/// bound to another is refused by the search as it was. Refused, the
/// request is what it offers again as each account known
/// (`Terminal::asking`), the envelope it was read in, `sent`, kept.
fn open_as_asked(
    cx: &Ctx,
    request: &Request,
    sent: &serde_json::Value,
    json: bool,
) -> Result<(Store, acquisition_store::Annotations)> {
    let selector = cx.account.as_deref().or(request.scope.account.as_deref());
    let opened = cx
        .frontend
        .account(selector)
        .and_then(|account| account.open())
        .map_err(|fault| {
            let e = anyhow::Error::from(fault);
            if json {
                println!(
                    "{}",
                    acquisition_verbs::refusal::document(
                        &e,
                        &crate::terminal::Terminal::asking(sent.clone())
                    )
                );
            } else {
                eprintln!("Error: {}", crate::terminal::failure_words(&e));
                // the ready commands: the request the JSON offers, rebound
                // to each account
                eprint!(
                    "{}",
                    crate::terminal::ready_accounts(
                        &e,
                        &crate::terminal::Terminal::asking(sent.clone())
                    )
                );
            }
            anyhow::Error::from(crate::terminal::AlreadyReported)
        })?;
    Ok((opened.facts, opened.intent))
}

// ---- text, a function of the JSON ---------------------------------------------------------------

/// A site search's translation as an answer opens with it: one line,
/// how far the text is the site's search; one line saying the items are
/// the store's, never the site's listings, since a reader took `exact`
/// for "the site would show the same"; the link back to the site, or why
/// there is none, with what it leaves out and its notes (C195,
/// [`link_words`]); then each clause that did not
/// run, each that runs as the site may not read it and each with no
/// stash meaning, with its note — ten of each listed and the rest
/// counted (C53) — and, beside an ambiguous clause whose other reading
/// the catalogue holds, what that reading adds and drops (C119), its
/// routes where the answer's routes are. The whole is the JSON's
/// `query.trade`. The words are [`fidelity_words`], [`TRADE_OVER`],
/// [`clauses_listed`] and [`stake_words`], the answer page's too.
pub(crate) fn trade_text(t: &TradeOut) -> Vec<String> {
    let mut out = vec![
        format!("trade   {}", fidelity_words(t)),
        format!("        over      {TRADE_OVER}"),
    ];
    out.extend(
        link_words(&t.link)
            .into_iter()
            .map(|(word, said)| format!("        {word:<9} {said}")),
    );
    let listed = clauses_listed(t);
    let width = listed
        .iter()
        .flat_map(|(_, clauses)| clauses)
        .map(|(_, c)| c.at.chars().count() + 2)
        .max()
        .unwrap_or(0);
    for (word, clauses) in listed {
        let word = format!("{word:<9}");
        for (_, c) in clauses.iter().take(10) {
            let stake = c.other.as_ref().map_or(String::new(), |o| {
                format!(
                    " · {}",
                    stake_words(&o.adds.count.to_string(), &o.drops.count.to_string())
                )
            });
            out.push(format!(
                "        {word} {:<width$}{}{stake}",
                c.at,
                clause_note(c)
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
    answer_text_of(a, None)
}

/// The answer's text, saying — where `--limit` was typed beside a count —
/// that it is each table's length and that a count shows no rows: the
/// one flag means both, and a reader who asks for thirty rows beside a
/// count otherwise gets a table and no word why.
fn answer_text_of(a: &Answer, limit_beside_count: Option<usize>) -> String {
    let now = acquisition_store::now();
    let mut out = answer_head(a);
    for (i, row) in a.rows.iter().enumerate() {
        out.push_str(&row_text(a, row, i == 0, now));
    }
    if let Some(more) = answer_more(a) {
        out.push_str(&more);
    }
    out.push_str(&answer_tail(a, limit_beside_count));
    out
}

/// The answer's opening (C53, the answer first): a site search's
/// translation, the query, the scope and the total — the text's; the
/// answer page says the same facts in the same words, each made by the
/// functions this calls, in its own layout.
fn answer_head(a: &Answer) -> String {
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
    line(format!("query   {}", query_words(&a.query.text)));
    if let Some(aside) = &a.set_aside {
        line(format!(
            "aside   {}",
            aside_words(&aside.text, " under --context corpus")
        ));
    }
    line(format!("scope   {}", scope_words(&a.scope)));
    let mut never = format!("        {}", never_fetched_words(&a.scope, now));
    // the list that exists: `tabs` over a league the query names, or the
    // one holding the most never fetched, the others counted (C118)
    if let Some(pointer) = a.tabs_pointer() {
        never.push_str(&format!(
            " · each tab: {}",
            crate::printed::tabs_command(&pointer)
        ));
        if pointer.other_leagues > 0 {
            never.push_str(&format!(
                " · {}",
                other_leagues_words(pointer.other_leagues)
            ));
        }
    }
    line(never);
    if let Some(words) = realm_not_held_words(&a.scope) {
        line(format!("        {words}"));
    }
    let mut total = format!("total   {}", total_words(a, now));
    if a.total.undecided.count > 0 {
        total.push_str(&format!(" · {}", undecided_words(a.total.undecided.count)));
    }
    line(total);
    if let Some(words) = never_fetched_sentence(a.scope.coverage.never_fetched) {
        line(format!("            {words}"));
    }
    // C188: the tables' release and the store's characters', where apart
    for words in a.scope.releases_words(now) {
        line(format!("            {words}"));
    }
    if let Some(words) = before_words(a) {
        line(format!("then    {words}"));
    }
    out
}

/// One row as the text prints it: the title, the place, what each part
/// showed and the id.
fn row_text(a: &Answer, row: &acquisition_search::answer::Row, first: bool, now: i64) -> String {
    let mut out = String::new();
    let mut line = |s: String| {
        out.push_str(s.trim_end());
        out.push('\n');
    };
    let Line {
        name,
        rarity,
        frame,
        // the title holds the base, as its type line
        base: _,
        // the item under `item` prints it, after the head
        price: _,
        place,
        removed,
        fetched,
        // `row_shows` says each value with what it is, below the head
        values: _,
    } = Line::of(row, now);
    // under `item` the row is titled as `show` titles the item, its
    // frame with it
    let mut head = name;
    for (label, word) in [("", rarity), ("frame ", frame)] {
        if let Some(word) = word {
            head.push_str(&format!(" · {label}{word}"));
        }
    }
    // a scope that spans realms says which one each item is in, as
    // `show` does; under one realm the scope line has said it
    if a.scope.realm == Realm::All {
        head.push_str(&format!(" · {}", row.place.realm));
    }
    head.push_str(&format!(" · {place} · {}", fetched_words(fetched)));
    if let Some(removed) = removed {
        head.push_str(&format!(" · removed {removed}"));
    }
    line(format!(
        "{}{head}",
        if first { "rows    " } else { "        " }
    ));
    let (shows, more) = row_shows(a, row);
    let show = || crate::printed::show_command(a, &row.id);
    let shows: Vec<String> = shows
        .into_iter()
        .map(|piece| match piece {
            RowPiece::Seen(s) => s,
            RowPiece::MoreSorted(n) => format!("{}: {}", more_sorted_words(n), show()),
            RowPiece::MoreOfValue(n, key) => {
                format!("{}: {}", more_of_value_words(n, &key), show())
            }
        })
        .collect();
    if !shows.is_empty() {
        line(format!("            {}", shows.join(" · ")));
    }
    for (n, path) in more {
        line(format!(
            "            {}: {}",
            more_of_term_words(n, &path),
            show()
        ));
    }
    line(format!("            id {}", row.id));
    if let Some(icon) = &row.icon {
        line(format!(
            "            icon {}",
            icon.as_deref().unwrap_or("none")
        ));
    }
    // the item as `show` prints it, from its base to what is unread; its
    // title, id, removal and place are the row's head, and the basis the
    // answer's foot
    if let Some(shown) = &row.shown {
        out.push_str(&item_text(
            ItemParts::of_row(shown),
            &row.place.realm,
            &a.account_named(),
        ));
        out.push('\n');
    }
    out
}

/// The rows' continuation as the text says it, where rows are left out;
/// where none are, the rows shown and their order as the page says them,
/// where there is an order to say or a value left out (a complete answer
/// says them too).
fn answer_more(a: &Answer) -> Option<String> {
    match &a.view {
        ViewOut::Rows(rows)
            if rows.left_out == 0
                && rows.returned > 0
                && (order_words(rows).is_some() || rows.values_left_out > 0) =>
        {
            Some(format!("order   {}\n", rows_shown_words(a, rows)))
        }
        ViewOut::Rows(rows) if rows.left_out > 0 => Some(format!(
            "more    {}: {}\n",
            rows_shown_words(a, rows),
            match &rows.next {
                Some(next) => next.command(),
                None => "a larger --limit returns the rest".to_string(),
            }
        )),
        _ => None,
    }
}

/// Everything after the rows: a count's tables, an explain, what a zero
/// answer offers, why items are undecided, the terms, the routes and the
/// basis last — the text's; the answer page says the same facts in the
/// same words in its own layout, its ways on its own.
fn answer_tail(a: &Answer, limit_beside_count: Option<usize>) -> String {
    let now = acquisition_store::now();
    let mut out = String::new();
    let mut line = |s: String| {
        out.push_str(s.trim_end());
        out.push('\n');
    };
    match &a.view {
        ViewOut::Rows(_) => {}
        ViewOut::Counts(counts) => {
            if let Some(limit) = limit_beside_count {
                line(limit_text(limit));
            }
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
            if let Some(limit) = limit_beside_count {
                line(limit_text(limit));
            }
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
            line(format!("explain {}", explain_words(explain)));
            line(format!(
                "  as given      {}",
                forced_words(explain.given.matched, explain.given.undecided)
            ));
            for (label, forced) in [
                ("forced true ", &explain.forced_true),
                ("forced false", &explain.forced_false),
            ] {
                line(format!(
                    "  {label}  {}",
                    forced_words(forced.matched.count, forced.undecided.count)
                ));
                line(format!("                {}", forced.query.text));
            }
        }
    }

    if let Some(zero) = &a.zero {
        for nothing in &zero.resolved_to_nothing {
            line(nothing_words(zero, nothing));
            for s in &nothing.suggestions {
                line(format!("            {}", suggestion_words(s, "   ")));
            }
        }
        // what the zero answer offers, each a command and never its
        // output: explain of each part, then the corpus
        let offers = &zero.offers;
        if !offers.explain.is_empty() {
            line(format!("offers  {EXPLAIN_OFFERS}"));
            for offer in &offers.explain {
                line(format!("    {}", offer.request.command()));
            }
            if offers.explain_left_out > 0 {
                line(format!(
                    "            {}: --explain <path>, a part's path as the terms block numbers it",
                    more_parts_words(offers.explain_left_out)
                ));
            }
        }
        if let Some(offer) = &offers.context {
            line(format!("        {CONTEXT_OFFER}"));
            line(format!("    {}", offer.request.command()));
        }
    }

    // why items are undecided, one line per reason with an example (V8)
    for (i, reason) in a.total.undecided_reasons.iter().enumerate() {
        line(format!(
            "{}{} unread — e.g. {} · id {}",
            if i == 0 { "undecided " } else { "          " },
            reason.reason.unread,
            example_name(&reason.example),
            reason.example.id
        ));
        line(format!("            {}", reason_words(&reason.reason)));
    }
    if a.total.reasons_left_out > 0 {
        line(format!(
            "          {}",
            reasons_left_out_words(a.total.reasons_left_out, "below")
        ));
    }

    if !a.terms.is_empty() {
        line(format!("terms   {}", terms_words(&a.scope)));
        let width = a
            .terms
            .iter()
            .map(|t| t.term.chars().count())
            .max()
            .unwrap_or(0)
            .min(60);
        // a path as long as its column would run into its term,
        // `0.3.0pseudo.…`
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
                counts.push(together_words(together));
            }
            line(format!(
                "  {:<path_width$}{:<width$}  {}",
                term.path,
                term.term,
                counts.join(" · ")
            ));
            if let Some(removed) = &term.removed {
                line(format!(
                    "       {}{}",
                    removed_words(removed, now),
                    match &removed.route {
                        Some(route) => format!(": {}", route.command()),
                        None => format!(" {REMOVED_HELD_NONE}"),
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
                    .map(carried_words)
                    .collect();
                let more = resolved.more + resolved.values.len().saturating_sub(shown);
                if more > 0 {
                    // the count that lists every value, where one does
                    values.push(match &resolved.rest {
                        Some(rest) => format!("{}: {}", more_words(more), rest.command()),
                        None => more_words(more),
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
    let counted = counted(a);
    let hidden = counted.unrouted();
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
    let (first, rest) = (routed(counted.first), routed(counted.rest));
    let shown: Vec<&(String, usize, String)> = first.iter().chain(&rest).collect();
    if !shown.is_empty() || hidden > 0 {
        line(if hidden > 0 {
            format!("routes  {ROUTES_RETURN} ({hidden} more: --routes)")
        } else {
            format!("routes  {ROUTES_RETURN}")
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

/// What a route returns, said once above the routes.
const ROUTES_RETURN: &str = "each returns exactly the members counted";

// ---- a count's text ------------------------------------------------------------------------------------

fn bucket_text(bucket: &Bucket, all_realms: bool) -> Vec<String> {
    let mut first = format!(
        "  {:>7}  {}",
        bucket.count.count,
        label_text(&bucket.label, all_realms)
    );
    if let Some(sum) = &bucket.sum {
        first.push_str(&format!("   {}", sum_words(sum)));
    }
    if let Some(slots) = slots_words(bucket) {
        first.push_str(&format!("   {slots}"));
    }
    if let Some(tally) = tally_words(&bucket.tally) {
        first.push_str(&format!("   {tally}"));
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
            .map(kind_words)
            .collect();
        out.push(format!("           {}", kinds.join(" · ")));
        // a kind with no route says why
        for k in bucket.sources.iter().chain(&bucket.flags) {
            if let Some(needs) = &k.needs {
                out.push(format!("           {}", needs_words(&k.kind, needs)));
            }
        }
    }
    if let Some(needs) = &bucket.label.needs {
        out.push(format!("           {needs}"));
    }
    out
}

/// What `--limit` is beside a count, where the two meet.
fn limit_text(limit: usize) -> String {
    format!("limit   {limit}, the values a table lists: a count shows no rows")
}

fn table_text(table: &Table, all_realms: bool) -> Vec<String> {
    let head = format!(
        "count   {}",
        table_head_words(table, ": a larger --limit returns the rest")
    );
    std::iter::once(head)
        .chain(
            table
                .buckets
                .iter()
                .flat_map(|b| bucket_text(b, all_realms)),
        )
        .chain(unmatched_text(&table.unmatched_leagues))
        .collect()
}

/// The scope's leagues no match is in, after a table keyed by `league`:
/// a bucket is a value an item holds, so they have none.
fn unmatched_text(leagues: &[String]) -> Option<String> {
    unmatched_words(leagues).map(|words| format!("           {words}"))
}

fn cross_text(cross: &CrossOut, all_realms: bool) -> Vec<String> {
    let mut out = vec![format!(
        "cross   {}",
        cross_head_words(cross, ": a larger --limit returns the rest")
    )];
    for cell in &cross.cells {
        let of: Vec<String> = cell.of.iter().map(|l| label_text(l, all_realms)).collect();
        let mut row = format!("  {:>7}  {}", cell.count.count, of.join(" × "));
        if let Some(sum) = &cell.sum {
            row.push_str(&format!("   {}", sum_words(sum)));
        }
        out.push(row);
        // a cell with no route says why, once per value that has none
        for label in &cell.of {
            if let Some(needs) = &label.needs {
                out.push(format!(
                    "           {}",
                    needs_words(&label_text(label, all_realms), needs)
                ));
            }
        }
    }
    for margin in &cross.margins {
        if margin.none.count + margin.undecided.count > 0 {
            out.push(format!(
                "           {}: {} · {}",
                margin.key,
                none_words(margin.none.count),
                undecided_words(margin.undecided.count)
            ));
        }
    }
    if let Some(tally) = cross_tally_words(&cross.tally) {
        out.push(format!("           {tally}"));
    }
    out.extend(unmatched_text(&cross.unmatched_leagues));
    out
}

/// How a terminal asks what the document's `next` names.
/// A static example, so in the portable form no declared shell expands or
/// needs quoted (C156): `~` after `=` is no home directory anywhere.
const DESCRIBE_SPELLED: &str = "at a terminal: `--describe <name>,…` names entries or blocks, `--describe=~<pattern>` a pattern, `--describe all` everything";

fn describe_text(d: &Describe, totals: bool) -> String {
    let mut out = String::new();
    // a pattern's answer opens with what each pattern matched
    for m in &d.patterns {
        out.push_str(&format!("{}\n", pattern_said(m)));
    }
    // a view's entry is named by the request's word, and this terminal
    // spells it as its flag after it (C143)
    let entries = |out: &mut String,
                   head: &str,
                   entries: &[acquisition_search::describe::Named],
                   flags: bool| {
        if entries.is_empty() {
            return;
        }
        out.push_str(&format!("{head}\n"));
        let named = |e: &acquisition_search::describe::Named| match flags {
            true => flagged(&e.name),
            false => e.name.clone(),
        };
        // the name column fits the block's longest name: a computed value's
        // is `pseudo.total_lightning_gem_levels`
        let name = entries
            .iter()
            .map(|e| named(e).chars().count() + 2)
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
            out.push_str(&format!(
                "  {:<name$}{:<kind$}{}\n",
                named(e),
                e.kind,
                e.what
            ));
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
    for Block { part, head, .. } in &BLOCKS {
        match part {
            Part::Fields => entries(&mut out, head, &d.fields, false),
            Part::Line => entries(&mut out, head, &d.line, false),
            Part::Linked => entries(&mut out, head, &d.linked, false),
            Part::Values => entries(&mut out, head, &d.values, false),
            Part::Composition => entries(&mut out, head, &d.composition, false),
            Part::Operators => entries(&mut out, head, &d.operators, false),
            Part::Slots => entries(&mut out, head, &d.slots, false),
            // the totals' table is named once, over the block, with its
            // version and source (C106's clause (c))
            Part::Computed => entries(
                &mut out,
                &format!("{head} — {}", d.totals),
                &d.computed,
                false,
            ),
            Part::Counts => entries(&mut out, head, &d.counts, true),
            Part::Explain => entries(&mut out, head, &d.explain, true),
            Part::Answer => entries(&mut out, head, &d.answer, false),
            // `--describe totals`: the table, where no computed value's
            // head already names it
            Part::Totals => {
                if totals && d.computed.is_empty() {
                    out.push_str(&format!("{head} — {}\n", d.totals));
                }
            }
            // every category id with the site's text for it and what makes
            // it hold (C106)
            Part::Categories => {
                if !d.categories.is_empty() {
                    out.push_str(&format!("{head}\n"));
                    for c in &d.categories {
                        out.push_str(&format!("  {:<30}{}\n", c.id, c.text));
                        if !c.holds.is_empty() {
                            out.push_str(&format!("  {:<30}  {}\n", "", c.holds));
                        }
                    }
                }
            }
            Part::Trade => {
                // the catalogue is a block of its own: the index names it,
                // and this terminal spells how to ask it
                if let Some(rows) = d.trade_rows {
                    out.push_str(&format!(
                        "{head} — what `--trade` makes of each clause a site search can carry, {rows} rows: `--describe trade`\n"
                    ));
                }
                if let Some(trade) = &d.trade {
                    out.push_str(&format!("{head} — {}\n", trade_said(trade)));
                    // the report an answer carries opens the block, before
                    // the rows
                    let width = trade
                        .report
                        .iter()
                        .map(|e| e.name.chars().count() + 2)
                        .max()
                        .unwrap_or(0);
                    for e in &trade.report {
                        out.push_str(&format!("  {:<width$}{}\n", e.name, e.what));
                    }
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
            Part::NotBuilt => {
                if !d.not_built.is_empty() {
                    out.push_str(&format!("{head}\n"));
                    let width = d
                        .not_built
                        .iter()
                        .map(|n| flagged(n.construct).chars().count() + 2)
                        .max()
                        .unwrap_or(0)
                        .max(22);
                    for n in &d.not_built {
                        out.push_str(&format!(
                            "  {:<width$}{:<32}{}\n",
                            flagged(n.construct),
                            n.step_text(),
                            n.what
                        ));
                    }
                }
            }
            Part::Limits => {
                if !d.limits.is_empty() {
                    out.push_str(&format!("{head}\n"));
                    for l in &d.limits {
                        out.push_str(&format!("  {}\n", l.said));
                    }
                }
            }
        }
    }
    // how to go on, in the document's words, then this terminal's
    // spelling of them (C143: the shared words first)
    if let Some(next) = &d.next {
        out.push_str(&format!("more: {next}\n"));
        out.push_str(&format!("      {DESCRIBE_SPELLED}\n"));
    }
    out
}

/// The item as `show`'s text says it, from its `base` line to its
/// `unread` lines: `show` prints it under its title, id and place, and a
/// row carrying the item under its head. `realm` and `account` open the
/// price's why command, as every printed command opens.
pub(crate) fn item_text(parts: ItemParts<'_>, realm: &str, account: &str) -> String {
    let item = parts.item;
    let mut out = String::new();
    if let Some(base) = &item.base {
        out.push_str(&format!("base    {base}\n"));
    }
    out.push_str(&format!("class   {}\n", class_words(parts.class)));
    out.push_str(&format!("category {}\n", category_words(parts.category)));
    for open in &parts.category.open {
        out.push_str(&format!("         {}\n", open_words(open)));
    }
    // what the listing state prices it at (C81): the price, or why none —
    // and the command that says why, with the target form `price show`
    // takes, ready to paste (else `item/<id>` is found only in a refusal);
    // a removed item has no listing state for `price show` to read; the
    // command opens as every printed command does, with the account the
    // item was shown for, by its name, and its realm, since the defaults
    // are another account's or another realm's where the store holds
    // several
    let why = match parts.price {
        Priced::None { kind, .. } if kind == "removed" => String::new(),
        _ => format!(
            " · why: {} item/{}",
            crate::printed::opening("price show", Some(account), Some(realm)),
            item.facts.id
        ),
    };
    out.push_str(&format!(
        "price   {}{why}\n",
        price_words(&item.facts.id, parts.price)
    ));
    if let Some(stack) = item.stack {
        out.push_str(&format!("stack   {stack}\n"));
    }
    if let Some(note) = &item.note {
        out.push_str(&format!("note    {note}\n"));
    }
    if let Some(flags) = flags_words(item) {
        out.push_str(&format!("is      {flags}\n"));
    }
    if let Some(sockets) = sockets_words(item) {
        out.push_str(&format!("sockets {sockets}\n"));
    }
    for (i, text) in shown_properties(item).iter().enumerate() {
        out.push_str(&format!(
            "{}{}\n",
            if i == 0 { "shown   " } else { "        " },
            text
        ));
    }
    for (i, l) in parts.lines.iter().enumerate() {
        out.push_str(&format!(
            "{}{:<22}{}\n",
            if i == 0 { "lines   " } else { "        " },
            line_kind_words(l),
            l.line.text.replace('\n', " / ")
        ));
        out.push_str(&format!(
            "        {:<22}{}{}\n",
            "",
            line_template_quoted(l),
            spaced_slots(line_slots_words(l))
        ));
        for row in &l.line.rows {
            out.push_str(&format!(
                "        {:<22}{}{}\n",
                "",
                line_row_quoted(row),
                spaced_slots(line_row_slots_words(row))
            ));
        }
        if let Some(limit) = l.limit {
            out.push_str(&format!("        {:<22}{limit}\n", ""));
        }
    }
    if item.unread.is_empty() {
        out.push_str(&format!("unread  {UNREAD_NOTHING}\n"));
    }
    for (i, u) in item.unread.iter().enumerate() {
        out.push_str(&format!(
            "{}{}\n",
            if i == 0 { "unread  " } else { "        " },
            u.problem
        ));
    }
    out
}

/// `show`'s text. `account` is the name of the account the item was shown
/// for, which the price's why line opens with: the basis names the account
/// by its uuid, a fingerprint, where every printed command names it as the
/// person does. The item page says the same facts in the same words, each
/// made by the functions this calls, with no why line.
pub(crate) fn shown_text(s: &Shown, account: &str) -> String {
    let item = &s.item;
    let mut out = String::new();
    out.push_str(&format!("{}\n", shown_title(item)));
    out.push_str(&format!("id      {}\n", item.facts.id));
    if let Some(removed_at) = item.facts.removed_at {
        out.push_str(&format!(
            "removed {}\n",
            shown_removed_words(acquisition_store::now(), removed_at)
        ));
    }
    out.push_str(&format!(
        "place   {}\n",
        shown_place_words(&s.place, acquisition_store::now())
    ));
    out.push_str(&item_text(ItemParts::of_shown(s), &s.place.realm, account));
    // why the item does or does not match a query: the root's outcome,
    // then each node's, a term with what it touched or why it is open
    if let Some(against) = &s.against {
        out.push_str(&format!("against {}\n", query_words(&against.query.text)));
        out.push_str(&format!("        {}\n", outcome_words(against.outcome)));
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
                outcome_words(node.outcome)
            ));
            if !node.shows.is_empty() {
                out.push_str(&format!("            {}\n", node_shows_words(node)));
            }
            if node.left_out > 0 {
                out.push_str(&format!(
                    "            {}\n",
                    node_left_out_words(node.left_out)
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

/// The item's shown properties, its item level and its requirements.
pub(crate) fn shown_properties(item: &acquisition_search::derive::Item) -> Vec<String> {
    item.properties
        .iter()
        .map(|p| p.text.as_str())
        .chain(item.item_level.as_deref())
        .chain(item.requires.as_deref())
        .map(|text| text.replace('\n', " / "))
        .collect()
}

/// Slots after a template as the text sets them, three spaces apart.
fn spaced_slots(slots: Option<String>) -> String {
    slots.map(|s| format!("   {s}")).unwrap_or_default()
}

/// A view's or a construct's name, the request's word, as this terminal
/// prints it: the word, and the flag that spells it here after it
/// (`count (--count)`, `show against (show --against)`); a name that is
/// no flag here as it stands (C143).
fn flagged(name: &str) -> String {
    let flag = match name.split_once(' ') {
        Some(("show", word)) => format!("show --{word}"),
        Some(_) => return name.to_string(),
        None if name.ends_with(':') => return name.to_string(),
        None => format!("--{name}"),
    };
    format!("{name} ({flag})")
}
