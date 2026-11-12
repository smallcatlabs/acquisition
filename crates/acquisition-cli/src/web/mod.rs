//! `acq web` (C139, the web UI is a verb of the CLI crate): a loopback
//! web UI that is a renderer, not an app (C16). Every page renders the
//! documents a verb already produces, and the server opens no store,
//! picks no account and compiles no plan itself: it calls the verbs
//! (C46; `tools/arch-check.sh`'s door grep reads this module). It writes
//! one kind, the saved search (C167), and that only through the verbs
//! that write it for every frontend (C168). The briefs are
//! `roadmap/briefs/4b-build.md` and, for the saved pages,
//! `roadmap/briefs/4-full-1-pages.md`.
//!
//! # As built
//!
//! - **C138, the listener** (verbatim): The web UI's listener binds
//!   127.0.0.1 on a random port, mints a per-launch token that every
//!   link and form action carries, refuses a request whose Host or
//!   Origin is not its own, serves no CORS, and mutates only by POST. Its
//!   routes are private to its page and unversioned, carrying the verbs'
//!   own documents, so they are no third surface (C12, two surfaces).
//!   Here: the token is every route's first path segment, so a request
//!   without it meets no route; the guard compares the Host header with
//!   the bound address and an Origin, where one is sent, with the
//!   server's own, before any route runs; each mutating route — apply,
//!   the league list's fetch, a saved search's write and its clear — is a
//!   POST route alone, so a GET on it is refused by its method; no
//!   response carries an `Access-Control-*` header. Every response says
//!   `Referrer-Policy: same-origin`: the browser's fetch of an item's art
//!   (C140) carries no referrer, so the token never reaches the image
//!   host, while a form posted to the server itself carries its real
//!   Origin — under `no-referrer` the Fetch standard serialises a POST's
//!   Origin as `null`, which the guard refuses (the owner's sitting, its
//!   first stop). A content policy admits images from that host alone
//!   and scripts and styles from the server alone.
//! - **The lifecycle is Ctrl-C** (the brief, Do 4): no lock, no reopen,
//!   no idle exit — a server launched from a terminal is a dev server.
//!   The URL is the first line on stdout (`--json`: `{"url": …}`, as
//!   `auth` prints its URL); `--no-browser` prints it alone.
//! - **The door is the human's** (`Frontend::human("web")`, C88): a
//!   submit from a page reaches the daemon as every CLI verb does, and
//!   its jobs say `web` as their channel (C65).
//! - **One account, named on every call**: the server resolves the
//!   account it serves once (C121), and every read and every submit
//!   names it — a selection's jobs, an apply's — so a login that comes
//!   or goes while the server runs never moves a page's submit to
//!   another account than the one the page shows (the review of 4b, its
//!   first finding). Once is at its start, or, where no account is on
//!   disk then, at the first request that finds one
//!   (`roadmap/briefs/99-web-leftovers.md`, Do 1): until then every
//!   page is the `not_logged_in` refusal with its remedy, so a person
//!   who ran `acq web` to look at the tool meets the page and signs in;
//!   from then on the account found is held for the server's life. Only
//!   a server started with no account named waits so: one named
//!   (`--account`) is the command's choice, refused
//!   before the bind whatever the refusal — `not_logged_in` over an
//!   empty index too, since a login later could make another account
//!   than the one named (the review at 49a74010, its first finding) —
//!   as is any refusal with none named but none (several accounts).
//! - **One corpus, held** (C98, the consumer holds the corpus and checks
//!   currency before every answer): the server holds the last corpus it
//!   loaded for the account it serves in the search crate's `Holder`,
//!   the one `acq-mcp` holds too, which checks `is_current` before every
//!   answer; a stale one, or one of another scope, is loaded again and
//!   swapped. No answer loads per request.
//! - **A page's document as JSON**: a request that accepts
//!   `application/json` is answered with the document the page renders —
//!   the answer, the item, the plan, the jobs — which is how the tests
//!   compare a page with the CLI's `--json` (C11).
//! - **A refusal's ways on are the pages' own** (#97,
//!   `roadmap/briefs/97-own-terms.md`): a refusal is the one refusal
//!   shape spelled by [`Paged`], the page's `Surface`, and the page renders
//!   that document — its words, its readings, then each offer saying what
//!   it is: a read as a link to the route that takes it, a write as a
//!   form posting the fields its route takes with one button in the
//!   offer's words. A search is a link where the form's own fields fold
//!   back to the request offered; a league's plan, or its tabs, a link to
//!   that page; the saved ways — the list, an opening, a write, a clear —
//!   as the C168 section below spells them; any other way on is no
//!   page's, and nothing on a page stands for it. The plan page prints no
//!   command to type next: its Apply button is that way on. An answer's
//!   ways on are spelled the same way (4-full part 1, step 4): every
//!   request the answer carries — a count's route, a removed id's, a
//!   value list's rest, a zero answer's offers — and the scope's
//!   never-fetched tabs go through [`Paged`], a link where a page takes
//!   one and the count or the words where none does; a continuation, a
//!   request with its place, is `search?next=` and its token, on an
//!   answer or a refusal alike. The words are still the CLI's
//!   (`failure_words`), as are the command lines the refresh and jobs
//!   pages print, which no route takes (#182).
//! - **C16, what a page holds** (the owner's reading of the ruling's word
//!   "state", 2026-11-06): document state is what a page shows, rebuilt
//!   from its URL; transient state — input not yet sent, the picker's
//!   asks in flight, the count beside a search's button and its ask in
//!   flight, the words in a list's filter boxes — is discarded by
//!   a reload and labels nothing. A page
//!   holds no browser storage, no cookie, no history entry of its own and
//!   no value computed from a document. Every document a page renders
//!   that carries a version — the answer's and the item's `basis`, the
//!   plan's `basis`, the saved list's `revision`, the file's, and the
//!   saved row's `revision`, an answer's opened row or a conflict's
//!   `current` — is copied into one `<meta name="basis">` line in the
//!   head, a JSON object keyed by the document, its values the documents'
//!   own fields ([`page::Head`]); a page whose documents carry none (the
//!   form's league list, the refresh page's tabs and characters, the
//!   jobs, a refusal without `current`) carries no line. The scan holds
//!   a list of words at the tests' recording seam: every page a golden
//!   records sets no cookie and names only scripts on the harness's one
//!   list (`SCRIPTS`, `tests/harness/mod.rs`), and
//!   `c16_no_served_script_names_a_storage_cookie_or_history_api`
//!   (`tests/web.rs`) fetches each script on it and refuses one that
//!   contains `localStorage`, `sessionStorage`, `indexedDB`, `caches`,
//!   `cookie`, `history`, `serviceWorker` or `.storage`. The property is that list, lexical: an API reached by
//!   another spelling, a cache held in a closure or the DOM, state the
//!   URL omits, a generation a script keeps for itself is the review's
//!   (`REVIEW-CHECKLIST.md`, the frontends' rows), until a browser-driven
//!   check exists.
//! - **C168, the saved pages** (`roadmap/briefs/4-full-1-pages.md`, Do 4
//!   to Do 8): `GET saved` is [`saved::list`]'s document, ten rows and the
//!   whole under `expand=true`, each row with a link that opens it and
//!   its rename form naming the revision listed and a Clear link. A
//!   clear asks first (the owner, 2026-11-07, "Clear should ask first"):
//!   `GET saved/clear?id=` takes an id alone, writes nothing, and shows
//!   the row with the clear's form at the revision shown and a link back
//!   to the list, its basis keyed `saved_search`; an id no row has is
//!   refused, offering the list, and a row this build cannot read is
//!   asked about like any other, a clear being all a page can do with
//!   it. `GET
//!   saved/open?id=` takes an id alone and writes nothing: the row
//!   [`saved::show`] finds is used only where its id is the URL's (a name
//!   may have an id's shape; one rule with the answer page's), and the
//!   answer is `303` to the search its request folds to, carrying `saved`
//!   and `saved_revision`, the row's revision as opened. `POST saved`
//!   takes `id`, `if_revision`, `name`, `text`, `realm` and `membership`
//!   into the verbs' one `Set` under `Expect::or(if_revision,
//!   Expect::Create)`, and `POST saved/clear` `id` and `if_revision`,
//!   both required, under `Expect::Exactly`: a field is present where the
//!   body names it, empty or not, and no page writes at `Expect::Current`
//!   (a browser is where a stale view could clobber). A write answers the
//!   browser `303` to the list, so a reload asks the list again and never
//!   posts again (the owner, 2026-11-07, "Redirect to the list"); its
//!   receipt is the JSON answer, the undo spelled as the page's form, and
//!   no page renders a receipt. A write way is spelled `{"post", "fields"}`
//!   and rendered as a button; a read way as a link. An answer opened
//!   from a saved search says so above it, and saves over that row at the
//!   revision its URL names, so a write between is refused as stale and
//!   offered again over the row as it stands.
//! - **The count** (4-full part 1, step 6.2, #245, the design's §4): every
//!   form that submits to `search` carries `data-count` and, beside its
//!   button, the words "Matches:" and an `<output>`, then a hidden
//!   undecided part with its own. `count.js` asks `GET count` with the
//!   fields the form would send, 300 ms after the last change, a new ask
//!   aborting the one in flight, and writes the answer's two numbers
//!   alone, a dash before and between; a refusal's words are the output's
//!   title. `count` folds as `search` does and answers the answer
//!   document with no row and no order ([`count`]), so its number is the
//!   one Search would give now. The picker announces its write with a
//!   bubbling `input`; neither script names the other.
//! - **The table** (4-full part 1, step 6.4b,
//!   `roadmap/briefs/4-full-1-step6-table.md`, from the window spike,
//!   `audits/web-2026-11/spike/`): an answer asked as a table
//!   (`view.fields=icon`) is every matching row in one scrolling page at
//!   one row height, the height and the block's rows written on the table
//!   ([`page::ROW_HEIGHT`], [`page::BLOCK_ROWS`]). Its line says how many
//!   items it holds and in what order; `view.limit` is the cards' and is
//!   not read for it. One maker gives a window's rows ([`Web::window`],
//!   [`window_request`]): the page's request at its answer's basis, its
//!   place the window's offset into the whole order and its limit the
//!   window's count, `item` among its fields for the price. The page
//!   answers its request and its first block over one corpus, so both
//!   stand at one basis, and renders that block, which `window.js` takes
//!   as one it holds; the script asks the rest as they near the viewport.
//!   `GET rows?next=<token>&offset=<n>&count=<n>` answers a window, the
//!   token the first window's request: the rows alone as HTML, the
//!   window request's answer document as JSON ([`rows`]). It refuses with
//!   400, in the one refusal shape, a token no answer printed, an offset
//!   or a count that is no whole number, a count of none or past the item
//!   rows' bound, and a window that runs past the rows; a basis moved
//!   since the page is the search's refusal with 409, and the script
//!   loads the page again. A table reached by a continuation and loaded
//!   again after its basis moved meets the continuation's own refusal of
//!   a moved basis, as a cards page does.
//! - **The view's links** (4-full part 1, step 6.4c,
//!   `roadmap/briefs/4-full-1-step6-sort.md`): nothing is typed to sort.
//!   The URL names the rows' values by `view.value`, one each in order as
//!   the CLI's `--value`, beside `view.sort` and `view.desc`; no control
//!   of a form writes the three ([`form::UNTYPED`]), and the query box
//!   carries what the URL named of them, hidden, so an edited query keeps
//!   them. One maker gives a change to the view ([`view_link`]): the same
//!   page ordered by one value, largest first or the other way where the
//!   rows are ordered by it, the value joining the answer's list where the
//!   list lacks it and no link where the list is full. Each value's head
//!   of the table is that link, the ordered one marked by an arrow and by
//!   words a screen reader gets. So is each number of an answer's card
//!   that a value reads (`page::card`): a line whole by its `value`, the
//!   value a trade filter on its stat compares, so a stat the form
//!   filtered lands on its own column; a displayed row's number by the
//!   field it shows, the row in the search crate's pieces
//!   (`acquisition_search::show::Displays`), never read again from its
//!   text. A number whose value the view holds is marked `valued`, the
//!   sort's as its head is; the item page has no view, and its numbers are
//!   words.

mod form;
mod page;

use std::sync::{Arc, OnceLock};

use acquisition_plan::RefreshPlan;
use acquisition_plan::pricing::Expect;
use acquisition_protocol::protocol::ListState;
use acquisition_protocol::realm::Realm;
use acquisition_search::answer::{Asked, Request};
use acquisition_search::{Holder, SearchError};
use acquisition_verbs::refusal::{Pointed, Way};
use acquisition_verbs::saved::{self, SavedRow};
use acquisition_verbs::{Account, Apply, Fault, Frontend, Jobs};

use crate::terminal::plural;
use anyhow::Result;
use axum::Router;
use axum::extract::{Form, Path, Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use serde_json::{Value, json};

use crate::run::Ctx;
use crate::{plan_cmd, store_cmd};
use page::{Head, Opened, OpenedRow, SavedLinks, enc, esc, pre, refusal};

/// The channel every write and submit from a page carries (C65).
const VIA: &str = "web";

/// The stylesheet, the picker's script, the list filter's, the count's
/// and the table's window, served as written.
const CSS: &str = include_str!("assets/page.css");
const PICKER: &str = include_str!("assets/picker.js");
const FILTER: &str = include_str!("assets/filter.js");
const COUNT: &str = include_str!("assets/count.js");
const WINDOW: &str = include_str!("assets/window.js");

/// How many stats the picker lists for one ask; the rest are counted.
const PICKED: usize = 50;

/// How often the jobs page reloads itself while a job is not done.
const JOBS_REFRESH: u32 = 3;

/// The content policy every response carries: the server's own scripts
/// and styles, images from GGG's CDN alone (C140), no frame, no other
/// form target.
const POLICY: &str = "default-src 'none'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src https://web.poecdn.com; form-action 'self'; frame-ancestors 'none'; base-uri 'none'";

/// What the server holds for its whole life.
struct Web {
    cx: Ctx,
    /// The account served: set at the start, or at the first request
    /// that finds one, and never moved after (the module doc).
    account: OnceLock<Account>,
    token: String,
    host: String,
    origin: String,
    form: form::Form,
    held: Holder,
}

/// Why a page could not render its document: a search refusal in the
/// search's words, anything else in the CLI's.
enum Failure {
    Search(SearchError),
    /// A request the pages never make: a usage error (C11's `usage`).
    Usage(String),
    Other(anyhow::Error),
}

impl From<SearchError> for Failure {
    fn from(e: SearchError) -> Failure {
        Failure::Search(e)
    }
}

impl From<acquisition_search::LanguageError> for Failure {
    fn from(e: acquisition_search::LanguageError) -> Failure {
        Failure::Search(e.into())
    }
}

impl From<Fault> for Failure {
    fn from(e: Fault) -> Failure {
        Failure::Other(e.into())
    }
}

impl From<anyhow::Error> for Failure {
    fn from(e: anyhow::Error) -> Failure {
        Failure::Other(e)
    }
}

/// `acq web`: bind, mint, print, open, serve until Ctrl-C.
pub(crate) async fn web(account: Option<String>, no_browser: bool, json: bool) -> Result<()> {
    let cx = Ctx {
        frontend: Frontend::human(VIA)?,
        account,
    };
    // no account on disk with none named is a page, every other
    // refusal the command's
    let served = match cx.account() {
        Ok(account) => OnceLock::from(account),
        Err(Fault::NoAccounts { .. }) if cx.account.is_none() => OnceLock::new(),
        Err(fault) => return Err(fault.into()),
    };
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0)).await?;
    let port = listener.local_addr()?.port();
    let token = mint()?;
    let url = format!("http://127.0.0.1:{port}/{token}/");
    let web = Arc::new(Web {
        cx,
        account: served,
        host: format!("127.0.0.1:{port}"),
        origin: format!("http://127.0.0.1:{port}"),
        token: token.clone(),
        form: form::Form::load()?,
        held: Holder::new(),
    });
    if json {
        println!("{}", json!({ "url": url }));
    } else {
        println!("{url}");
    }
    use std::io::Write as _;
    std::io::stdout().flush().ok();
    // as `auth` does: `--json` changes what is printed, never whether the
    // browser opens; stdout under `--json` is the one document
    if !no_browser {
        let opened = crate::auth_cmd::open_browser(&url);
        if !json {
            println!(
                "({} Ctrl-C stops the server)",
                if opened {
                    "opened in your browser;"
                } else {
                    "open it in your browser;"
                }
            );
            std::io::stdout().flush().ok();
        }
    }
    let at = |path: &str| format!("/{token}{path}");
    let app = Router::new()
        .route(&at("/"), get(home))
        .route(&at("/search"), get(search))
        .route(&at("/item/{id}"), get(item))
        .route(&at("/stats"), get(stats))
        .route(&at("/count"), get(count))
        .route(&at("/rows"), get(rows))
        .route(&at("/refresh"), get(tabs))
        .route(&at("/plan"), get(plan))
        .route(&at("/apply"), post(apply))
        .route(&at("/leagues"), post(fetch_leagues))
        .route(&at("/jobs"), get(jobs))
        .route(&at("/saved"), get(saved_list).post(saved_set))
        .route(&at("/saved/open"), get(saved_open))
        .route(&at("/saved/clear"), get(saved_ask).post(saved_clear))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&web),
            with_account,
        ))
        .route(&at("/assets/page.css"), get(css))
        .route(&at("/assets/picker.js"), get(picker))
        .route(&at("/assets/filter.js"), get(filter))
        .route(&at("/assets/count.js"), get(count_script))
        .route(&at("/assets/window.js"), get(window_script))
        .fallback(no_such_page)
        .layer(middleware::from_fn_with_state(Arc::clone(&web), guard))
        .with_state(web);
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}

/// A per-launch token: 128 random bits, as hex.
fn mint() -> Result<String> {
    let mut bytes = [0u8; 16];
    getrandom::fill(&mut bytes).map_err(|e| anyhow::anyhow!("no randomness for the token: {e}"))?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

/// C138, before any route: the Host must be the bound address and an
/// Origin, where one is sent, the server's own. Every response then
/// carries the referrer and content policies, and no CORS header.
async fn guard(
    State(web): State<Arc<Web>>,
    request: axum::extract::Request,
    next: Next,
) -> Response {
    // read in a block of its own: a borrow of the request (whose body is
    // not `Sync`) held across the await would make the guard's future
    // not `Send`
    let (host, origin) = {
        let said = |name: header::HeaderName| {
            request
                .headers()
                .get(name)
                .map(|v| v.to_str().unwrap_or("\u{fffd}").to_string())
        };
        (said(header::HOST), said(header::ORIGIN))
    };
    let mut response = if host.as_deref() != Some(web.host.as_str()) {
        refused(
            StatusCode::FORBIDDEN,
            &format!(
                "refused: this server answers only at {}, the address it printed",
                web.origin
            ),
        )
    } else if origin.is_some_and(|o| o != web.origin) {
        refused(
            StatusCode::FORBIDDEN,
            "refused: a page of another site sent this request",
        )
    } else {
        next.run(request).await
    };
    let headers = response.headers_mut();
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("same-origin"),
    );
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(POLICY),
    );
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

/// Every page but the assets, after the guard: the account served,
/// resolved here if the server has none yet; while none is found the
/// page is its refusal (the module doc, "One account").
async fn with_account(
    State(web): State<Arc<Web>>,
    request: axum::extract::Request,
    next: Next,
) -> Response {
    match blocking(&web, |web| web.served().map(|_| ())).await {
        Ok(()) => next.run(request).await,
        Err(fault) => web.failed("Account", &Failure::from(fault), request.headers()),
    }
}

fn refused(status: StatusCode, words: &str) -> Response {
    (
        status,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        format!("{words}\n"),
    )
        .into_response()
}

/// A path no route holds — a request without the token among them.
async fn no_such_page() -> Response {
    refused(StatusCode::NOT_FOUND, "no such page")
}

fn wants_json(headers: &HeaderMap) -> bool {
    headers
        .get(header::ACCEPT)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|a| a.contains("application/json"))
}

fn html(status: StatusCode, body: String) -> Response {
    (
        status,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        body,
    )
        .into_response()
}

fn document(value: &impl serde::Serialize) -> Response {
    match serde_json::to_string_pretty(value) {
        Ok(body) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            body,
        )
            .into_response(),
        Err(e) => refused(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}

impl Web {
    /// The account this server serves: the one it holds, else the one
    /// the index resolves now, held from then on — a request racing this
    /// one to it serves whichever was held first.
    fn served(&self) -> Result<&Account, Fault> {
        if let Some(account) = self.account.get() {
            return Ok(account);
        }
        let found = self.cx.account()?;
        Ok(self.account.get_or_init(|| found))
    }

    /// The name of the account this server serves, none before one is
    /// found: what every page's nav bar names.
    fn account_name(&self) -> Option<&str> {
        self.account.get().map(|a| a.entry.username.as_str())
    }

    fn page(&self, title: &str, head: Head, body: &str) -> Response {
        html(
            StatusCode::OK,
            page::page(&self.token, title, self.account_name(), head, body),
        )
    }

    /// A refusal's document, in the one refusal shape (round 4 of the 2g
    /// build) as a page spells it ([`Paged`]).
    fn refusal(&self, failure: &Failure) -> Value {
        match failure {
            Failure::Usage(words) => json!({ "error": words, "kind": "usage" }),
            Failure::Search(e) => acquisition_verbs::refusal::of_search(e, &Paged { web: self }),
            Failure::Other(e) => acquisition_verbs::refusal::document(e, &Paged { web: self }),
        }
    }

    /// A refusal as a page, or as its document where JSON is asked (C11):
    /// the page renders the document (C16).
    fn failed(&self, title: &str, failure: &Failure, headers: &HeaderMap) -> Response {
        if wants_json(headers) {
            return self.refused_document(failure);
        }
        let doc = self.refusal(failure);
        // a conflict's page renders the row as it stands: its revision is
        // the page's basis
        let head = match page::saved_current(&doc) {
            Some(row) => Head::default().with("saved_search", &row["revision"]),
            None => Head::default(),
        };
        html(
            StatusCode::BAD_REQUEST,
            page::page(
                &self.token,
                title,
                self.account_name(),
                head,
                &page::refused(&doc),
            ),
        )
    }

    /// A refusal as its document, whatever was asked: what a page's
    /// refusal answers where JSON is asked, and the count's every refusal.
    fn refused_document(&self, failure: &Failure) -> Response {
        (
            StatusCode::BAD_REQUEST,
            [(header::CONTENT_TYPE, "application/json")],
            self.refusal(failure).to_string(),
        )
            .into_response()
    }

    /// Whether `account` names the account this server serves, by its
    /// name or its uuid (a store's read names it by the uuid).
    fn serves(&self, account: &str) -> bool {
        self.account.get().is_some_and(|served| {
            served.entry.username == account || served.entry.uuid.as_deref() == Some(account)
        })
    }

    /// `path` on this server with `fields` as its query: a link a page
    /// prints, the token first as every link's is (C138).
    fn link(&self, path: &str, fields: &[(&str, String)]) -> String {
        let query: Vec<String> = fields
            .iter()
            .map(|(k, v)| format!("{}={}", enc(k), enc(v)))
            .collect();
        match query.is_empty() {
            true => format!("/{}/{path}", self.token),
            false => format!("/{}/{path}?{}", self.token, query.join("&")),
        }
    }

    /// The saved pages' links and forms, absolute under the token.
    fn saved_links(&self) -> SavedLinks {
        SavedLinks {
            list: self.link("saved", &[]),
            every: self.link("saved", &[("expand", "true".to_string())]),
            write: self.link("saved", &[]),
            clear: self.link("saved/clear", &[]),
        }
    }

    /// The saved search at exactly `id`, as [`saved::show`] finds it, or
    /// none where no row stands at that id: `show` reads an id or a name,
    /// and a name may have an id's shape, so a row found by its name under
    /// another id — or several rows of that name — is no row at `id`
    /// (Do 5, one rule for every page that resolves a URL's id). Any other
    /// failure is the page's refusal.
    fn saved_at(&self, id: &str) -> Result<Option<SavedRow>, Failure> {
        match saved::show(&self.cx.frontend, id) {
            Ok(row) if row.id() == id => Ok(Some(row)),
            Ok(_) => Ok(None),
            Err(e) => match e.chain().find_map(|c| c.downcast_ref::<Pointed>()) {
                Some(p)
                    if matches!(p.kind, "saved_search_not_found" | "ambiguous_saved_search") =>
                {
                    Ok(None)
                }
                _ => Err(Failure::Other(e)),
            },
        }
    }

    /// A search request as the search form's own fields — its realm, its
    /// text, and every view field the form asks: its rows' limit, order and
    /// values named, or its count's keys and limit — kept only where the form folds them
    /// back to that request, as the account this server serves: what a
    /// page can ask. Its rows' fields are `view.fields=icon` where they are
    /// the art alone, the table, and no field where they are the art and
    /// the item or none, the page's default, cards; any other list is no
    /// form's. Its membership is a field only where it is not `live`,
    /// the form's default, so every link made before the form carried one
    /// stands as it was. None for any other (a site search, a tree, another
    /// account or view: a crossed table, a sum, an explained node): no page
    /// takes it (the review of #97's build, its second finding: a count
    /// was dropped).
    fn asked_by_the_form(&self, request: &Request) -> Option<Vec<(&'static str, String)>> {
        if !request
            .scope
            .account
            .as_deref()
            .is_none_or(|a| self.serves(a))
        {
            return None;
        }
        let mut fields = Vec::new();
        if let Some(realm) = &request.scope.realm {
            fields.push(("realm", realm.as_str().to_string()));
        }
        if request.scope.membership != acquisition_search::answer::Membership::Live {
            fields.push(("membership", request.scope.membership.as_str().to_string()));
        }
        fields.push(("text", request.query.text.clone()?));
        match &request.view.shape {
            acquisition_search::answer::Shape::Rows(rows) => {
                if let Some(limit) = rows.limit {
                    fields.push(("view.limit", limit.to_string()));
                }
                if let Some(sort) = &rows.sort {
                    fields.push(("view.sort", sort.clone()));
                }
                if rows.desc {
                    fields.push(("view.desc", "true".to_string()));
                }
                // the values named, one field each in their order; the
                // query's own are the request's to fill again, so unnamed
                for value in &rows.values {
                    fields.push(("view.value", value.clone()));
                }
                // the row's fields: the table is `view.fields=icon`; the
                // cards, and none (a route carries none), the page's
                // default; any other list is no form's
                use acquisition_search::answer::RowField;
                match rows.fields.as_slice() {
                    [RowField::Icon] => fields.push(("view.fields", "icon".to_string())),
                    [] | [RowField::Icon, RowField::Item] => {}
                    _ => return None,
                }
            }
            // the one field the form reads a count from: each list as
            // `--count` takes it, joined, since the field's list reads a
            // narrowing's own `line:` after another's; an order it cannot
            // read (a field after a narrowing) folds back to another count
            // and is offered nowhere (the review at 3ce22d81)
            acquisition_search::answer::Shape::Counts(counts) => {
                fields.push(("view.count", counts.lists().join(",")));
                if let Some(limit) = counts.limit {
                    fields.push(("view.limit", limit.to_string()));
                }
            }
            _ => {}
        }
        let pairs: Vec<(String, String)> = fields
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();
        let folded = form::fold(&self.form, &pairs).ok()?;
        let asked = Asked::read(&folded.request().ok()?).ok()?.request;
        // the page's rows carry the fields the form asks, and its account
        // is the one served; a count's keys are compared as the key's one reader
        // reads them, never by their letters (`Rarity` is `rarity`)
        let mut offered = request.clone();
        offered.scope.account = None;
        if let (
            acquisition_search::answer::Shape::Rows(rows),
            acquisition_search::answer::Shape::Rows(own),
        ) = (&mut offered.view.shape, &asked.view.shape)
        {
            rows.fields = own.fields.clone();
        }
        (read_keys(asked) == read_keys(offered)).then_some(fields)
    }

    /// The request answered over the held corpus (C98): checked first,
    /// then the holder's corpus, current and of the request's scope.
    fn answer(&self, request: &Request) -> Result<acquisition_search::Answer, Failure> {
        let corpus = self.corpus(request)?;
        Ok(acquisition_search::answer(&corpus, request)?)
    }

    /// The holder's corpus for `request`, checked first: current, and of
    /// the request's scope. A page that answers two requests of one scope
    /// answers both over the one corpus this returns, at one basis.
    fn corpus(&self, request: &Request) -> Result<Arc<acquisition_search::Corpus>, Failure> {
        request.check()?;
        let opened = self.served()?.open()?;
        Ok(self.held.corpus(request, &opened.facts, &opened.intent)?)
    }

    /// A window of a table's rows (the table's design, `rows`): the page's
    /// request at its answer's `basis`, from row `offset` of its order,
    /// `count` of them, each carrying its item for its price — the one
    /// maker the `rows` route and the page's first block call, over the
    /// corpus each takes ([`window_request`]).
    fn window(
        &self,
        corpus: &acquisition_search::Corpus,
        page: &Request,
        basis: &acquisition_search::Basis,
        offset: usize,
        count: usize,
    ) -> Result<acquisition_search::Answer, Failure> {
        Ok(acquisition_search::answer(
            corpus,
            &window_request(page, basis, offset, count),
        )?)
    }

    /// The store's league list (the `leagues` job's latest answer), whole,
    /// each league marked with whether this realm has tabs of it on disk:
    /// what both pages' league control lists, through the verbs' two
    /// reads (C46), the list and what the realm holds; a word that is no
    /// realm holds nothing. None where no list was fetched.
    fn leagues(&self, realm: &str) -> Result<Option<Vec<(String, bool)>>, Failure> {
        let account = self.served()?;
        let list = account.league_list(true)?;
        if list.fetched_at.is_none() {
            return Ok(None);
        }
        let held = match Realm::parse(realm) {
            Some(realm) => {
                acquisition_verbs::LeaguesHeld::read(
                    &account.facts()?,
                    &acquisition_verbs::leagues::PolicyRead::of(None),
                    realm,
                    None,
                    true,
                )?
                .leagues
            }
            None => Vec::new(),
        };
        Ok(Some(
            list.leagues
                .into_iter()
                .map(|l| {
                    let on_disk = held
                        .iter()
                        .any(|h| h.league.as_deref() == Some(l.id.as_str()) && h.tabs > 0);
                    (l.id, on_disk)
                })
                .collect(),
        ))
    }
}

/// A page's side of the one refusal shape (#97, each surface spells its
/// own ways on): its words the CLI's, and a way on a link to the page
/// that takes it — a search the form can ask, its own fields; a league's
/// plan or its tabs and characters; the saved list, whole where asked
/// and for one row's show (no page shows one row alone); a saved search
/// opened — for the account this server serves; or, for a saved search's
/// write or clear, a form: `{"post": <the route>, "fields": {…}}`, the
/// fields that route takes as strings, `if_revision` `0` for a create at
/// an id. A way no page takes is offered nowhere on a page: another
/// account, a price or policy write, a job, `describe`, a search the form
/// cannot ask, a saved search written at the revision read.
struct Paged<'a> {
    web: &'a Web,
}

impl acquisition_verbs::refusal::Surface for Paged<'_> {
    /// The CLI's words, but where the refusal's offers are its remedy and
    /// a page takes each (a plan whose policy moved, a league the policy
    /// does not cover among ten or fewer): the cause alone, its ways on the
    /// links beside it (the review of #97's build, its first finding).
    fn words(&self, e: &anyhow::Error) -> String {
        crate::terminal::failure_words_by(e, |fault| match fault {
            Fault::Replan { .. } => fault.to_string(),
            Fault::NotCovered { covered, .. }
                if (1..=acquisition_store::bound::LISTED).contains(&covered.len()) =>
            {
                fault.to_string()
            }
            _ => crate::terminal::fault_words(fault),
        })
    }

    fn request(
        &self,
        way: &acquisition_verbs::refusal::Way,
    ) -> Option<(Value, Option<&'static str>)> {
        use acquisition_verbs::refusal::Way;
        let league = |realm: &Realm, league: &str| {
            vec![
                ("realm", realm.as_str().to_string()),
                ("league", league.to_string()),
            ]
        };
        let link = match way {
            // a continuation, a request with its place, is the one way a
            // page spells it: its token, for the account served
            Way::Search(request) if continues(request) => {
                if !request
                    .scope
                    .account
                    .as_deref()
                    .is_none_or(|a| self.web.serves(a))
                {
                    return None;
                }
                self.web.link("search", &[("next", request.token())])
            }
            Way::Search(request) => self
                .web
                .link("search", &self.web.asked_by_the_form(request)?),
            Way::RefreshPlan {
                account,
                realm,
                league: named,
            } if self.web.serves(account) => self.web.link("plan", &league(realm, named)),
            Way::Tabs {
                account,
                realm,
                league: named,
            } if self.web.serves(account) => self.web.link("refresh", &league(realm, named)),
            Way::SavedSearches { expand: false } => self.web.link("saved", &[]),
            Way::SavedSearches { expand: true } | Way::SavedSearchShow { .. } => {
                self.web.link("saved", &[("expand", "true".to_string())])
            }
            Way::SavedSearchRun { id, account }
                if account.as_deref().is_none_or(|a| self.web.serves(a)) =>
            {
                self.web.link("saved/open", &[("id", id.clone())])
            }
            Way::SavedSearchSet(set) => {
                let mut fields = serde_json::Map::new();
                if let Some(id) = &set.id {
                    fields.insert("id".into(), json!(id));
                }
                match set.expect {
                    Expect::Exactly(revision) => {
                        fields.insert("if_revision".into(), json!(revision.to_string()));
                    }
                    Expect::Create if set.id.is_some() => {
                        fields.insert("if_revision".into(), json!("0"));
                    }
                    Expect::Create => {}
                    // a page never writes at the revision read (C168)
                    Expect::Current => return None,
                }
                for (key, value) in [
                    ("name", &set.name),
                    ("text", &set.text),
                    ("realm", &set.realm),
                    ("membership", &set.membership),
                ] {
                    if let Some(value) = value {
                        fields.insert(key.into(), json!(value));
                    }
                }
                return Some((
                    json!({ "post": self.web.link("saved", &[]), "fields": fields }),
                    None,
                ));
            }
            Way::SavedSearchClear { id, if_revision } => {
                return Some((
                    json!({
                        "post": self.web.link("saved/clear", &[]),
                        "fields": { "id": id, "if_revision": if_revision.to_string() },
                    }),
                    None,
                ));
            }
            _ => return None,
        };
        Some((json!(link), None))
    }
}

/// Whether `request` is a continuation: a rows request with its place,
/// the rows it starts after and the basis that place is in, as
/// `Request::from_token` takes one.
fn continues(request: &Request) -> bool {
    request.basis.is_some()
        && matches!(
            &request.view.shape,
            acquisition_search::answer::Shape::Rows(rows) if rows.after.is_some()
        )
}

/// The request a window of a table's rows asks: the page's request at
/// `basis`, its place `offset` rows into the order and its limit
/// `count`, `item` among the row's fields beside the page's own (the
/// table's `icon`), so each row carries its price. The offset counts from
/// the order's first row whatever place the page's request held: a table
/// lists the whole order.
fn window_request(
    page: &Request,
    basis: &acquisition_search::Basis,
    offset: usize,
    count: usize,
) -> Request {
    use acquisition_search::answer::{RowField, Shape};
    let mut request = page.clone();
    request.basis = Some(basis.clone());
    if let Shape::Rows(rows) = &mut request.view.shape {
        rows.after = Some(offset);
        rows.limit = Some(count);
        if !rows.fields.contains(&RowField::Item) {
            rows.fields.push(RowField::Item);
        }
    }
    request
}

/// `request` with each count key as the key's one reader spells it
/// (`Counts::lists`, through `bind::bind_key`) and the form's list reads it
/// back (`counts::count_keys`): two spellings of one key compare equal.
fn read_keys(mut request: Request) -> Request {
    if let acquisition_search::answer::Shape::Counts(counts) = &mut request.view.shape {
        counts.keys = counts
            .keys
            .iter()
            .map(|key| {
                let alone = acquisition_search::answer::Counts {
                    keys: vec![key.clone()],
                    sum: None,
                    limit: None,
                };
                acquisition_search::counts::count_keys(&alone.lists().join(","))
                    .ok()
                    .and_then(|read| read.into_iter().next())
                    .unwrap_or_else(|| key.clone())
            })
            .collect();
    }
    request
}

/// Run a page's blocking work — store reads, a corpus load — off the
/// server's threads.
async fn blocking<T: Send + 'static>(
    web: &Arc<Web>,
    work: impl FnOnce(&Web) -> T + Send + 'static,
) -> T {
    let web = Arc::clone(web);
    match tokio::task::spawn_blocking(move || work(&web)).await {
        Ok(value) => value,
        Err(e) => std::panic::resume_unwind(e.into_panic()),
    }
}

fn pick<'a>(fields: &'a [(String, String)], name: &str) -> Option<&'a str> {
    fields
        .iter()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v.as_str())
        .filter(|v| !v.is_empty())
}

fn realm_of(fields: &[(String, String)]) -> Result<Realm, Failure> {
    match pick(fields, "realm") {
        None => Ok(Realm::DEFAULT),
        Some(word) => {
            acquisition_verbs::realm(word).map_err(|e| Failure::Other(anyhow::anyhow!(e)))
        }
    }
}

// ---- the search pages ---------------------------------------------------------------------------

/// The form (Do 6), its league list the counts by league over the corpus.
async fn home(
    State(web): State<Arc<Web>>,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    let realm = pick(&fields, "realm").unwrap_or("pc").to_string();
    let shown = realm.clone();
    let (leagues, said) = blocking(&web, move |web| match web.leagues(&realm) {
        Ok(leagues) => (leagues, String::new()),
        Err(failure) => (None, page::refused(&web.refusal(&failure))),
    })
    .await;
    let body = format!(
        "{said}{}",
        page::search_form(&web.form, &shown, leagues.as_deref())
    );
    web.page(
        "Search",
        Head {
            picker: true,
            count: true,
            ..Head::default()
        },
        &body,
    )
}

/// The answer (Do 6): a form submission folded into the trade JSON, the
/// query box's text or the trade box's site search (#188), or a
/// continuation's `next`, answered over the held corpus. A site search
/// whose URL names another realm than the one the page carried runs in
/// the URL's, and the page says so above the answer. With `saved` and
/// `saved_revision` (4-full part 1, step 3, Do 7), read apart from the
/// fold, the page says which saved search it was opened from and saves
/// over it at that revision; the JSON is the answer alone.
async fn search(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    let json = wants_json(&headers);
    let sent = fields.clone();
    let asked = blocking(&web, move |web| -> Result<_, Failure> {
        let (fields, from) = opened_from(fields)?;
        // the realm the page carried beside a site search whose URL names
        // another: not read, and said above the answer (#188)
        let mut carried = None;
        let mut continued = None;
        let request = match pick(&fields, "next") {
            Some(token) => {
                let request = Request::from_token(token)?;
                continued = Some(request.clone());
                request
            }
            None => {
                let folded = form::fold(&web.form, &fields).map_err(Failure::Usage)?;
                if let (Some(trade), Some(realm)) = (&folded.trade, &folded.realm)
                    && let Some(url) = crate::search_cmd::trade_realm(trade)
                    && Realm::parse(realm).is_none_or(|r| r.as_str() != url)
                {
                    carried = Some((realm.clone(), url));
                }
                Asked::read(&folded.request()?)?.request
            }
        };
        let corpus = web.corpus(&request)?;
        let answer = acquisition_search::answer(&corpus, &request)?;
        // a table's first block, over the same corpus: at the answer's
        // basis by construction, its route the first window's request
        let window = match &answer.view {
            acquisition_search::answer::ViewOut::Rows(rows)
                if !rows
                    .fields
                    .contains(&acquisition_search::answer::RowField::Item)
                    && answer.total.matched > 0 =>
            {
                let count = answer.total.matched.min(page::BLOCK_ROWS);
                let first = web.window(&corpus, &request, &answer.basis, 0, count)?;
                let token = window_request(&request, &answer.basis, 0, count).token();
                Some(page::Window {
                    route: web.link("rows", &[("next", token)]),
                    first,
                })
            }
            _ => None,
        };
        let opened = match from {
            Some((id, at)) if !json => {
                let row = match web.saved_at(&id)? {
                    Some(SavedRow::Read(row)) => OpenedRow::Read(row),
                    Some(SavedRow::Unreadable(_)) => OpenedRow::Unreadable,
                    None => OpenedRow::Gone,
                };
                Some(Opened { id, at, row })
            }
            _ => None,
        };
        Ok((answer, window, opened, carried, continued))
    })
    .await;
    match asked {
        Ok((answer, ..)) if json => document(&answer),
        Ok((answer, window, opened, carried, continued)) => {
            let mut head = Head {
                count: true,
                window: window.is_some(),
                ..Head::default()
            }
            .with("answer", &answer.basis);
            if let Some(Opened {
                row: OpenedRow::Read(row),
                ..
            }) = &opened
            {
                head = head.with("saved_search", row.revision);
            }
            let said = carried
                .map(|(page, url)| {
                    format!(
                        "<p class=\"notice\">{}</p>\n",
                        esc(&format!(
                            "The site search's URL names its realm, {url}: it ran there, not in \
                             {page}, the realm the page showed."
                        ))
                    )
                })
                .unwrap_or_default();
            let rows_links = rows_links(&web, &sent, continued.as_ref(), &answer);
            let view_links = |value: &str| match &answer.view {
                acquisition_search::answer::ViewOut::Rows(rows) => {
                    view_link(&web, &sent, continued.as_ref(), rows, value)
                }
                _ => None,
            };
            web.page(
                "Answer",
                head,
                &format!(
                    "{said}{}",
                    page::answer(
                        &answer,
                        opened.as_ref(),
                        &web.saved_links(),
                        &|way| {
                            use acquisition_verbs::refusal::Surface as _;
                            Paged { web: &web }
                                .request(way)
                                .and_then(|(link, _)| link.as_str().map(str::to_string))
                        },
                        &rows_links,
                        &view_links,
                        window.as_ref(),
                    )
                ),
            )
        }
        Err(failure) => web.failed("Answer", &failure, &headers),
    }
}

/// A request's fields as a page sends them, in order.
type Fields = Vec<(String, String)>;

/// The plain links beside a rows answer that has a row (4-full part 1,
/// step 5, and the page seat of 2026-11-08, its fault 6): the same answer
/// the other way — cards or the table — and, on a continuation's page,
/// the same search from its first rows. Reached by the form's fields, the
/// other way is the page's own URL, its fields as sent, with
/// `view.fields=icon` added where it shows cards and taken out where it
/// shows the table. Reached by a continuation, whose token holds its
/// view whole, both are the continued request from its start — its place
/// and basis dropped, the other way's fields swapped — spelt as the form
/// would ask it, the saved search the page was opened from carried; none
/// where the form cannot ask it (a site search, which the form reads as
/// its translated text). None where the rows carry another list than
/// the page's two, which no form asks.
fn rows_links(
    web: &Web,
    sent: &Fields,
    continued: Option<&Request>,
    answer: &acquisition_search::Answer,
) -> page::RowsLinks {
    use acquisition_search::answer::{RowField, Shape, ViewOut};
    let none = page::RowsLinks::default();
    let ViewOut::Rows(rows) = &answer.view else {
        return none;
    };
    if answer.rows.is_empty() {
        return none;
    }
    let (other, words) = match rows.fields.as_slice() {
        [RowField::Icon, RowField::Item] => (vec![RowField::Icon], "as a table"),
        [RowField::Icon] => (vec![RowField::Icon, RowField::Item], "as cards"),
        _ => return none,
    };
    let Some(continued) = continued else {
        let mut fields: Vec<(&str, &str)> = sent
            .iter()
            .filter(|(k, _)| k != "view.fields")
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        if other == [RowField::Icon] {
            fields.push(("view.fields", "icon"));
        }
        let query: Vec<String> = fields
            .iter()
            .map(|(k, v)| format!("{}={}", page::enc(k), page::enc(v)))
            .collect();
        return page::RowsLinks {
            other: Some(page::Presentation {
                href: format!("search?{}", query.join("&")),
                words,
            }),
            first: None,
        };
    };
    // the continued search from its start, as the form would ask it, the
    // saved search it was opened from carried as every link of it is
    let saved: String = sent
        .iter()
        .filter(|(k, _)| k == "saved" || k == "saved_revision")
        .map(|(k, v)| format!("&{}={}", page::enc(k), page::enc(v)))
        .collect();
    let spelt = |request: Request| {
        web.asked_by_the_form(&request)
            .map(|fields| format!("{}{saved}", web.link("search", &fields)))
    };
    let mut first = continued.clone();
    first.basis = None;
    let Shape::Rows(view) = &mut first.view.shape else {
        return none;
    };
    view.after = None;
    let mut swapped = first.clone();
    if let Shape::Rows(view) = &mut swapped.view.shape {
        view.fields = other;
    }
    page::RowsLinks {
        other: spelt(swapped).map(|href| page::Presentation { href, words }),
        first: spelt(first),
    }
}

/// The one maker of a change to the view (4-full part 1, step 6.4c,
/// Decision 2): the same page ordered by `value`, largest first, or the
/// other way where its rows are already ordered by it. A value among the
/// answer's values leaves the list as the page's URL names it; another
/// names the answer's list with it added at its end, which the sort then
/// orders by. A value outside a full list ([`VALUES`]) gets no link, since
/// the search refuses a list past it. Spelt as [`rows_links`] spells the
/// other presentation: reached by the form's fields, the page's own URL,
/// its fields as sent with the view's order and list replaced; reached by
/// a continuation, the continued search from its start as the form would
/// ask it, the saved search the page was opened from carried; none where
/// the form cannot ask it. Heads and a card's numbers both call it.
///
/// [`VALUES`]: acquisition_search::answer::VALUES
fn view_link(
    web: &Web,
    sent: &Fields,
    continued: Option<&Request>,
    rows: &acquisition_search::answer::RowsOut,
    value: &str,
) -> Option<page::ViewLink> {
    use acquisition_search::answer::{Shape, VALUES};
    let held = rows.values.iter().any(|v| v == value);
    if !held && rows.values.len() >= VALUES {
        return None;
    }
    let ordered = (rows.sort.as_deref() == Some(value)).then_some(rows.desc);
    // largest first, or the other way where the rows are ordered by it so
    let desc = ordered.is_none_or(|desc| !desc);
    let listed = |list: &mut Vec<String>| {
        if !held {
            *list = rows.values.clone();
            list.push(value.to_string());
        }
    };
    let href = match continued {
        None => {
            let mut fields: Vec<(&str, String)> = sent
                .iter()
                .filter(|(k, _)| {
                    !matches!(k.as_str(), "view.sort" | "view.desc") && (held || k != "view.value")
                })
                .map(|(k, v)| (k.as_str(), v.clone()))
                .collect();
            fields.push(("view.sort", value.to_string()));
            if desc {
                fields.push(("view.desc", "true".to_string()));
            }
            let mut values = Vec::new();
            listed(&mut values);
            fields.extend(values.into_iter().map(|v| ("view.value", v)));
            let query: Vec<String> = fields
                .iter()
                .map(|(k, v)| format!("{}={}", page::enc(k), page::enc(v)))
                .collect();
            format!("search?{}", query.join("&"))
        }
        Some(continued) => {
            let mut request = continued.clone();
            request.basis = None;
            let Shape::Rows(view) = &mut request.view.shape else {
                return None;
            };
            view.after = None;
            view.sort = Some(value.to_string());
            view.desc = desc;
            listed(&mut view.values);
            let saved: String = sent
                .iter()
                .filter(|(k, _)| k == "saved" || k == "saved_revision")
                .map(|(k, v)| format!("&{}={}", page::enc(k), page::enc(v)))
                .collect();
            format!(
                "{}{saved}",
                web.link("search", &web.asked_by_the_form(&request)?)
            )
        }
    };
    Some(page::ViewLink {
        href,
        held,
        ordered,
    })
}

/// `saved` and `saved_revision` taken out of a search's fields, which the
/// fold would refuse: together or not at all, `saved` an id's shape and
/// the revision a positive integer, else a usage refusal.
fn opened_from(fields: Vec<(String, String)>) -> Result<(Fields, Option<(String, i64)>), Failure> {
    let (opened, fields): (Vec<_>, Vec<_>) = fields
        .into_iter()
        .partition(|(k, _)| k == "saved" || k == "saved_revision");
    let named = |name: &str| {
        opened
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.clone())
    };
    let from = match (named("saved"), named("saved_revision")) {
        (None, None) => None,
        (Some(id), Some(at)) => {
            if !saved::is_id(&id) {
                return Err(Failure::Usage(format!(
                    "{id:?} is no saved search's id: a page opens a saved search by its id, \
                     twelve hex characters"
                )));
            }
            match at.parse::<i64>() {
                Ok(at) if at > 0 => Some((id, at)),
                _ => {
                    return Err(Failure::Usage(format!(
                        "{at:?} is no revision: the revision a saved search was opened at is \
                         a positive whole number"
                    )));
                }
            }
        }
        _ => {
            return Err(Failure::Usage(
                "a saved search is opened by its id and the revision it was opened at, the \
                 two together"
                    .to_string(),
            ));
        }
    };
    Ok((fields, from))
}

/// One item (Do 6): `show`'s document, with its art and its badges.
async fn item(State(web): State<Arc<Web>>, headers: HeaderMap, Path(id): Path<String>) -> Response {
    let shown = blocking(&web, move |web| -> Result<_, Failure> {
        let account = web.served()?;
        let opened = account.open()?;
        Ok(acquisition_search::show(
            &opened.facts,
            &opened.intent,
            &id,
            false,
        )?)
    })
    .await;
    match shown {
        Ok(shown) if wants_json(&headers) => document(&shown),
        Ok(shown) => web.page(
            "Item",
            Head::default().with("item", &shown.basis),
            &page::item(&shown),
        ),
        Err(failure) => web.failed("Item", &failure, &headers),
    }
}

/// A window of a table's rows (the module doc, "The table"): the
/// request a table's segment names as `next`, its rows from `offset`,
/// `count` of them, made by [`Web::window`] at the token's basis. HTML is
/// the rows alone, which the window script places; JSON is the answer
/// document of the window's request. Refused, in the one refusal shape
/// and as the pages are, with 400: a token no answer printed; an offset
/// or a count that is no whole number; a count of none or past the item
/// rows' bound; a window that does not lie inside the rows, nothing
/// clipped in silence. A basis moved since the page is the search's own
/// refusal with 409, which the script answers by loading the page again.
async fn rows(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Fields>,
) -> Response {
    let asked = blocking(&web, move |web| -> Result<_, Failure> {
        let request = Request::from_token(pick(&fields, "next").unwrap_or(""))?;
        let number = |name: &str, what: &str| {
            pick(&fields, name)
                .and_then(|n| n.parse::<usize>().ok())
                .ok_or_else(|| Failure::Usage(format!("`{name}` is {what}: a whole number")))
        };
        let offset = number("offset", "the row a window starts at, counted from the first as 0")?;
        let count = number("count", "how many rows a window holds")?;
        let bound = acquisition_store::bound::ITEM_ROWS;
        if !(1..=bound).contains(&count) {
            return Err(Failure::Usage(format!(
                "a window holds from 1 to {bound} rows, each with its item, and this one asks {count}"
            )));
        }
        let Some(basis) = request.basis.clone() else {
            // `from_token` reads only a request with its place and basis
            return Err(Failure::Usage("a window continues an answer at its basis".to_string()));
        };
        let corpus = web.corpus(&request)?;
        let answer = web.window(&corpus, &request, &basis, offset, count)?;
        let matched = answer.total.matched;
        // the window from its offset, its count beside what is left: no
        // sum of two numbers a request names, which could pass the largest
        if offset >= matched || count > matched - offset {
            return Err(Failure::Usage(format!(
                "a window lies inside the rows: {count} from row {offset} runs past the {} the answer holds",
                plural(matched, "row", "rows")
            )));
        }
        Ok(answer)
    })
    .await;
    match asked {
        Ok(answer) if wants_json(&headers) => document(&answer),
        Ok(answer) => html(
            StatusCode::OK,
            page::window_rows(&answer, acquisition_store::now()),
        ),
        Err(failure) => {
            let doc = web.refusal(&failure);
            let status = match doc["kind"].as_str() {
                Some("basis_changed") => StatusCode::CONFLICT,
                _ => StatusCode::BAD_REQUEST,
            };
            if wants_json(&headers) {
                (
                    status,
                    [(header::CONTENT_TYPE, "application/json")],
                    doc.to_string(),
                )
                    .into_response()
            } else {
                html(status, page::refused(&doc))
            }
        }
    }
}

/// The count beside a search's button (4-full part 1, step 6.2, #245):
/// a form's fields folded as [`search`] folds them, the saved search an
/// answer was opened from set aside, and answered with no row and no
/// order ([`form::fold_count`]): the answer document, JSON whatever
/// was asked, its `total` the number `count.js` writes. A refusal is the
/// search's refusal document. Asked with no basis, it is the store's
/// count now, never compared with the page's answer.
async fn count(State(web): State<Arc<Web>>, Query(fields): Query<Fields>) -> Response {
    let counted = blocking(&web, move |web| -> Result<_, Failure> {
        let (fields, _) = opened_from(fields)?;
        let folded = form::fold_count(&web.form, &fields).map_err(Failure::Usage)?;
        let request = Asked::read(&folded.request()?)?.request;
        web.answer(&request)
    })
    .await;
    match counted {
        Ok(answer) => document(&answer),
        Err(failure) => web.refused_document(&failure),
    }
}

/// The picker's listing (Do 3): the stat vocabulary for the words asked.
async fn stats(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    let ask = pick(&fields, "q").unwrap_or_default().to_string();
    match blocking(&web, move |_| {
        acquisition_search::trade::vocabulary::vocabulary(&ask, PICKED)
    })
    .await
    {
        Ok(listed) => document(&listed),
        Err(e) => web.failed("Stats", &Failure::from(e), &headers),
    }
}

// ---- the refresh pages --------------------------------------------------------------------------

/// The tabs and characters of a league with a selection (Do 7): the
/// `tabs` document whole and the `characters` document whole (C149),
/// no default ticked.
async fn tabs(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    let realm = match realm_of(&fields) {
        Ok(realm) => realm,
        Err(failure) => return web.failed("Refresh", &failure, &headers),
    };
    let league = pick(&fields, "league")
        .unwrap_or(acquisition_plan::DEFAULT_LEAGUE)
        .to_string();
    let read = blocking(&web, move |web| -> Result<_, Failure> {
        let account = web.served()?;
        let facts = account.facts()?;
        let listed = acquisition_search::tabs::Tabs::read(
            &facts,
            realm.as_str(),
            &league,
            &acquisition_search::tabs::Selection::default(),
            None,
            true,
            acquisition_store::now(),
        )?
        .0
        .grouped();
        // every character, as every tab: the page is a selection, and a
        // row it does not list cannot be ticked (C53's audit view)
        let characters = acquisition_verbs::Characters::of(
            Some(realm.as_str()),
            Some(&league),
            facts.characters(Some(realm.as_str()), Some(&league))?,
            None,
            true,
            acquisition_store::now(),
        );
        let empty = (listed.tabs == 0)
            .then(|| store_cmd::no_tabs_text(realm, &league, &facts, &account.entry.username));
        // the terminal's sentence, which reads the realm's characters
        // where the league holds none (the page seat's fault 11)
        let none = match characters.characters {
            0 => Some(store_cmd::no_characters_text(
                Some(realm),
                Some(&league),
                &facts,
            )?),
            _ => None,
        };
        let leagues = web.leagues(realm.as_str())?;
        Ok((league, listed, characters, empty, none, leagues))
    })
    .await;
    let (league, listed, characters, empty, none, leagues) = match read {
        Ok(read) => read,
        Err(failure) => return web.failed("Refresh", &failure, &headers),
    };
    if wants_json(&headers) {
        return document(&json!({ "tabs": listed, "characters": characters }));
    }
    let now = acquisition_store::now();
    let mut body = format!(
        "<form method=\"get\" action=\"refresh\"><label class=\"control\">Realm <select name=\"realm\">{}</select></label> {}<button type=\"submit\">Show</button></form>\n{}",
        ["pc", "xbox", "sony", "poe2"]
            .iter()
            .map(|r| format!(
                "<option{}>{r}</option>",
                if *r == realm.as_str() {
                    " selected"
                } else {
                    ""
                }
            ))
            .collect::<String>(),
        page::league_select(leagues.as_deref(), Some(&league), false),
        page::fetch_leagues(),
    );
    body.push_str(&format!(
        "<form method=\"get\" action=\"plan\">\n<input type=\"hidden\" name=\"realm\" value=\"{}\"><input type=\"hidden\" name=\"league\" value=\"{}\">\n",
        esc(realm.as_str()),
        esc(&league)
    ));
    // the form's two buttons above its lists, so a long list never hides
    // them (the page seat's probe)
    let buttons = "<p><button type=\"submit\" name=\"selected\" value=\"yes\">Refresh selected</button> <button type=\"submit\">Plan under the policy</button></p>\n";
    match empty {
        // an empty league's facts line too (the review of round C,
        // finding 3)
        Some(said) => {
            body.push_str(&pre(&format!(
                "{said}\n{}",
                store_cmd::tabs_facts_line(&listed, acquisition_store::now()).trim_end()
            )));
            body.push_str(buttons);
        }
        None => {
            body.push_str(&pre(&format!(
                "{}\n{}",
                store_cmd::tabs_head(&listed, realm, &league, None),
                store_cmd::tabs_facts_line(&listed, acquisition_store::now()).trim_end()
            )));
            body.push_str(buttons);
            // the stash list one group, open, the substashes box at its
            // top; each folder a group of its own tabs, open; each map or
            // unique tab's pages a group under it, closed (the 4b sitting
            // round, Do 4; the page seat's probe)
            body.push_str(&format!(
                "<details class=\"list\" open><summary>Stash tabs ({})</summary>\n<label class=\"control\"><input type=\"checkbox\" name=\"deep\" value=\"true\"> the substashes box: also fetch a ticked map or unique tab's pages</label>\n",
                listed.rows.len()
            ));
            body.push_str(&tab_groups(&listed.groups, now, true));
            body.push_str("</details>\n");
        }
    }
    if let Some(none) = &none {
        body.push_str(&pre(none));
    } else {
        body.push_str(&format!(
            "<details class=\"list\" open><summary>Characters ({})</summary>\n<table>\n<tr><th></th><th>Character</th><th>Level</th><th>Items</th><th>Fetched</th></tr>\n{}",
            characters.characters,
            filter_row(&["Character", "Level", "Items", "Fetched"])
        ));
        for c in &characters.rows {
            body.push_str(&format!(
                "<tr class=\"row\"><td><input type=\"checkbox\" name=\"character\" value=\"{}\"></td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                esc(&c.id),
                esc(&c.name),
                c.level.map(|l| l.to_string()).unwrap_or_default(),
                esc(&store_cmd::items_cell(c.fetched_at, c.item_count)),
                esc(&store_cmd::ago(now, c.fetched_at))
            ));
        }
        body.push_str("</table>\n</details>\n");
    }
    body.push_str("</form>\n");
    web.page(
        "Refresh",
        Head {
            filter: true,
            ..Head::default()
        },
        &body,
    )
}

/// The second header row of a list: a box under each named column, the
/// tick column's cell empty. A box has no name, so nothing in it is sent;
/// the list filter's script reads it (the page seat's probe).
fn filter_row(columns: &[&str]) -> String {
    let boxes: String = columns
        .iter()
        .enumerate()
        .map(|(n, column)| {
            format!(
                "<td><input class=\"col-filter\" data-col=\"{}\" placeholder=\"filter\" aria-label=\"filter the {} column\"></td>",
                n + 1,
                esc(column)
            )
        })
        .collect();
    format!("<tr class=\"filters\"><td></td>{boxes}</tr>\n")
}

/// The stash list's groups as the document gives them (the sitting
/// round's review, its fourth finding): a run of tabs one table, a
/// folder a group of its own holding its tabs' table, each map or unique
/// tab's pages nested under it. `top` is the list's own level, which
/// heads the first table with the columns and their filter boxes; every
/// table shares the columns' widths (`table.tabs`), so a folder's rows
/// line up under them, each a `member` the stylesheet indents, so a tab
/// after a folder reads as the list's own and not the folder's (the page
/// seat of 2026-11-08, its fault 15).
fn tab_groups(groups: &[acquisition_search::tabs::TabGroup], now: i64, top: bool) -> String {
    let class = if top { "row" } else { "row member" };
    let row = |tab: &acquisition_store::TabRow, tick: String| {
        format!(
            "<tr class=\"{class}\"><td>{tick}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            esc(&tab.name),
            esc(&tab.r#type),
            esc(&store_cmd::items_cell(tab.fetched_at, tab.item_count)),
            esc(&store_cmd::ago(now, tab.fetched_at))
        )
    };
    let said = |words: &str| format!("<span class=\"fate\">{}</span>", esc(words));
    let open = "<table class=\"tabs\"><colgroup><col class=\"tick\"><col><col class=\"type\"><col class=\"items\"><col class=\"fetched\"></colgroup>\n";
    let mut out = String::new();
    let mut table = false;
    if top {
        out.push_str(open);
        out.push_str("<tr><th></th><th>Tab</th><th>Type</th><th>Items</th><th>Fetched</th></tr>\n");
        out.push_str(&filter_row(&["Tab", "Type", "Items", "Fetched"]));
        table = true;
    }
    for group in groups {
        let tab = &group.tab;
        // a folder holds tabs and no items: no checkbox, since a selection
        // names its tabs, each ticked in the folder's own rows
        if acquisition_store::is_folder_type(&tab.r#type) {
            if table {
                out.push_str("</table>\n");
                table = false;
            }
            out.push_str(&format!(
                "<details class=\"folder holds\" open><summary>{} · {} · {}</summary>\n{}</details>\n",
                esc(&tab.name),
                esc(&tab.r#type),
                plural(group.tabs.len(), "tab", "tabs"),
                tab_groups(&group.tabs, now, false)
            ));
            continue;
        }
        if !table {
            out.push_str(open);
            table = true;
        }
        out.push_str(&row(
            tab,
            format!(
                "<input type=\"checkbox\" name=\"tab\" value=\"{}\">",
                esc(&tab.id)
            ),
        ));
        if !group.pages.is_empty() {
            out.push_str(&format!(
                "<tr class=\"holds\"><td></td><td colspan=\"4\"><details class=\"pages\"><summary>{} pages of {}</summary>\n<table>\n",
                group.pages.len(),
                esc(&tab.name)
            ));
            let page_said = said(&format!(
                "a page of {}: tick {} with the substashes box",
                tab.name, tab.name
            ));
            for page in &group.pages {
                out.push_str(&row(page, page_said.clone()));
            }
            out.push_str("</table>\n</details></td></tr>\n");
        }
    }
    if table {
        out.push_str("</table>\n");
    }
    out
}

/// "Fetch the league list" (the 4b sitting round, Do 3): the `leagues`
/// job, as `acq leagues` submits it — one request, no probe — through the
/// human door and as the served account, landing on the jobs page.
async fn fetch_leagues(State(web): State<Arc<Web>>, headers: HeaderMap) -> Response {
    let submitted = async {
        let mut client = crate::run::connect(&web.cx).await?;
        acquisition_verbs::jobs::submit(
            &web.cx.frontend,
            &mut client,
            "leagues",
            json!({}),
            0,
            Some(web.served()?.entry.username.as_str()),
        )
        .await?;
        anyhow::Ok(())
    }
    .await;
    match submitted {
        Ok(()) => to_jobs(&web),
        Err(e) => web.failed("Leagues", &Failure::Other(e), &headers),
    }
}

fn to_jobs(web: &Web) -> Response {
    see_other(&web.link("jobs", &[]))
}

/// The plan page (Do 7): a selection's plan or the stored policy's,
/// rendered as `--plan` renders one, the envelope whole in a hidden field
/// beside "Apply".
async fn plan(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    let realm = match realm_of(&fields) {
        Ok(realm) => realm,
        Err(failure) => return web.failed("Plan", &failure, &headers),
    };
    let league = pick(&fields, "league")
        .unwrap_or(acquisition_plan::DEFAULT_LEAGUE)
        .to_string();
    // "Refresh selected" (C44 as amended): the ticked tabs and characters
    // compiled into a selection plan; "Plan under the policy": the stored
    // policy's. Either lands here, and apply is the one apply route.
    let selection = pick(&fields, "selected").map(|_| {
        let named = |name: &str| -> Vec<String> {
            fields
                .iter()
                .filter(|(k, v)| k == name && !v.is_empty())
                .map(|(_, v)| v.clone())
                .collect()
        };
        acquisition_plan::PlanSelection {
            tabs: Some(acquisition_plan::Selection::Ids(named("tab")))
                .filter(|t| !matches!(t, acquisition_plan::Selection::Ids(ids) if ids.is_empty())),
            characters: named("character"),
            deep: pick(&fields, "deep").is_some(),
        }
    });
    if let Some(selection) = &selection
        && selection.tabs.is_none()
        && selection.characters.is_empty()
    {
        let failure = Failure::Other(anyhow::anyhow!(
            "nothing is selected: tick a tab or a character to refresh it"
        ));
        return web.failed("Plan", &failure, &headers);
    }
    let account = match web.served() {
        Ok(account) => account,
        Err(fault) => return web.failed("Plan", &Failure::from(fault), &headers),
    };
    let planned = async {
        let mut opened = account.open()?;
        match &selection {
            Some(selection) => opened.plan_selection(realm, &league, selection).await,
            None => opened.plan(realm, &league).await,
        }
    }
    .await;
    let planned = match planned {
        Ok(planned) => planned,
        Err(fault) => return web.failed("Plan", &Failure::from(fault), &headers),
    };
    if wants_json(&headers) {
        return document(&planned);
    }
    // the instant the plan's ages were said at (#211)
    let now = planned.ages.at.unwrap_or_else(acquisition_store::now);
    let mut body = pre(&plan_cmd::render_plan_on_a_page(
        &planned.plan,
        planned.quote_note.as_ref(),
        &planned.notes,
        now,
        &account.entry.username,
        planned.uncovered_tabs.as_ref(),
    ));
    if !planned.plan.actions.is_empty() {
        match serde_json::to_string(&planned.plan) {
            Ok(envelope) => body.push_str(&format!(
                "<form method=\"post\" action=\"apply\"><input type=\"hidden\" name=\"plan\" value=\"{}\"><button type=\"submit\">Apply</button></form>\n",
                esc(&envelope)
            )),
            Err(e) => body.push_str(&refusal(&e.to_string())),
        }
    }
    web.page(
        "Plan",
        Head::default().with("plan", &planned.plan.basis),
        &body,
    )
}

/// "Apply" (Do 7): the envelope posted back whole to `Opened::apply`, as
/// `--apply=FILE` reads one. A moved policy is refused in the words C44
/// gives it, a spent plan sends nothing and says so (C111), and a
/// submitted one goes to the jobs page.
async fn apply(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Form(fields): Form<Vec<(String, String)>>,
) -> Response {
    let applied = async {
        let text = pick(&fields, "plan").ok_or_else(|| {
            anyhow::anyhow!("no plan was posted: apply posts the envelope a plan page holds")
        })?;
        let value: Value = serde_json::from_str(text)?;
        let plan = RefreshPlan::from_value(&value)?;
        let mut opened = web.served()?.open()?;
        let mut slot = None;
        let applied = opened.apply(&plan, None, &mut slot).await;
        if let Some(client) = &slot {
            crate::note_spawn(client);
        }
        anyhow::Ok(applied?)
    }
    .await;
    match applied {
        Ok(Apply::Submitted(_)) => to_jobs(&web),
        Ok(done) if wants_json(&headers) => document(&done),
        Ok(Apply::Nothing { note, .. } | Apply::Spent { note, .. }) => {
            web.page("Apply", Head::default(), &pre(&note))
        }
        Err(e) => web.failed("Apply", &Failure::Other(e), &headers),
    }
}

/// The jobs (Do 7): the `jobs` document, reloading itself while a job is
/// not done. It observes and never spawns.
async fn jobs(State(web): State<Arc<Web>>, headers: HeaderMap) -> Response {
    let listed = match web.cx.frontend.jobs(ListState::All, false).await {
        Ok(listed) => listed,
        Err(fault) => return web.failed("Jobs", &Failure::from(fault), &headers),
    };
    match listed {
        Jobs::Live(ref jobs) => {
            if wants_json(&headers) {
                return document(&listed);
            }
            let running = jobs.counts.by_state.running + jobs.counts.by_state.waiting > 0;
            let head = Head {
                refresh: running.then_some(JOBS_REFRESH),
                ..Head::default()
            };
            // the daemon's rails and their halt first, as `daemon status`
            // prints them, so a queue that stands still says why
            let rails = match web.cx.frontend.daemon().await {
                Ok(acquisition_verbs::DaemonReport::Running { status, .. }) => match *status {
                    acquisition_protocol::protocol::Response::DaemonStatus { rails, .. } => {
                        crate::daemon_cmd::rails_text(&rails)
                    }
                    _ => String::new(),
                },
                _ => String::new(),
            };
            web.page(
                "Jobs",
                head,
                &pre(&format!("{rails}{}", crate::jobs_cmd::jobs_text(jobs))),
            )
        }
        Jobs::Absent { ref persisted } => {
            if wants_json(&headers) {
                return document(&listed);
            }
            match crate::jobs_cmd::persisted_queue_text(persisted.as_ref()) {
                Ok(text) => web.page("Jobs", Head::default(), &pre(&text)),
                Err(e) => web.failed("Jobs", &Failure::Other(e), &headers),
            }
        }
    }
}

// ---- the saved pages (C168) --------------------------------------------------------------------

/// A field a saved route was sent, present where the body or the query
/// names it, empty or not (Do 6: the text `""` is a query).
fn sent<'a>(fields: &'a [(String, String)], name: &str) -> Option<&'a str> {
    fields
        .iter()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v.as_str())
}

/// The fields a saved route takes, or a usage refusal naming the first
/// it does not.
fn taken(fields: &[(String, String)], names: &[&str], route: &str) -> Result<(), Failure> {
    match fields.iter().find(|(k, _)| !names.contains(&k.as_str())) {
        Some((name, _)) => Err(Failure::Usage(format!(
            "`{name}` is no field of {route}: it takes {}",
            names
                .iter()
                .map(|n| format!("`{n}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ))),
        None => Ok(()),
    }
}

/// A sent `if_revision`, read as a whole number, or a usage refusal.
fn revision_sent(fields: &[(String, String)]) -> Result<Option<i64>, Failure> {
    sent(fields, "if_revision")
        .map(|word| {
            word.trim().parse::<i64>().map_err(|_| {
                Failure::Usage(format!(
                    "`if_revision` {word:?} is no revision: it names the revision written over \
                     as a whole number"
                ))
            })
        })
        .transpose()
}

/// The saved searches (Do 4): ten, or every one under `expand=true`.
async fn saved_list(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    let listed = blocking(&web, move |web| -> Result<_, Failure> {
        taken(&fields, &["expand"], "the saved searches' list")?;
        let expand = match sent(&fields, "expand") {
            None => false,
            Some("true") => true,
            Some(other) => {
                return Err(Failure::Usage(format!(
                    "`expand` {other:?} is no value it takes: `expand=true` lists every saved \
                     search"
                )));
            }
        };
        Ok(saved::list(&web.cx.frontend, None, expand)?)
    })
    .await;
    match listed {
        Ok(listed) if wants_json(&headers) => document(&listed),
        Ok(listed) => web.page(
            "Saved searches",
            Head::default().with("saved_searches", listed.revision),
            &page::saved_list(&listed, &web.saved_links(), &|id| {
                web.link("saved/open", &[("id", id.to_string())])
            }),
        ),
        Err(failure) => web.failed("Saved searches", &failure, &headers),
    }
}

/// A saved search opened (Do 5): `303` to the search its request folds
/// to, carrying its id and the revision it was opened at. It writes
/// nothing.
async fn saved_open(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    let opened = blocking(&web, move |web| -> Result<_, Failure> {
        taken(&fields, &["id"], "a saved search's opening")?;
        let id = sent(&fields, "id").unwrap_or_default().to_string();
        if !saved::is_id(&id) {
            return Err(Failure::Usage(format!(
                "{id:?} is no saved search's id: a page opens a saved search by its id, twelve \
                 hex characters, as the saved searches list it"
            )));
        }
        let row = match web.saved_at(&id)? {
            Some(SavedRow::Read(row)) => row,
            // a page refuses from the row it read, never from a second
            // read: the words are the page's, beside the verbs' own in
            // `saved::request_of`, whose maker is private
            Some(SavedRow::Unreadable(row)) => {
                return Err(listed(
                    "refused",
                    format!(
                        "saved search {} at revision {} holds a value this build cannot read \
                         ({}); a page cannot open it",
                        row.id, row.revision, row.unreadable
                    ),
                ));
            }
            None => {
                return Err(listed(
                    "saved_search_not_found",
                    format!("no saved search has the id {id}"),
                ));
            }
        };
        let Some(mut asked) = web.asked_by_the_form(&row.request) else {
            return Err(Failure::Other(anyhow::anyhow!(
                "saved search {id} is no search a page can ask: open it with an agent's search \
                 or the terminal's"
            )));
        };
        asked.push(("saved", id));
        asked.push(("saved_revision", row.revision.to_string()));
        Ok(web.link("search", &asked))
    })
    .await;
    match opened {
        Ok(location) => see_other(&location),
        Err(failure) => web.failed("Saved search", &failure, &headers),
    }
}

/// A saved route's refusal of the row it read, in the page's words, the
/// list its one way on.
fn listed(kind: &'static str, message: String) -> Failure {
    Failure::Other(
        Pointed {
            kind,
            message,
            ways: vec![(
                Way::SavedSearches { expand: false },
                "the saved searches, each with its id".to_string(),
            )],
            fields: serde_json::Map::new(),
        }
        .into(),
    )
}

/// A saved search written (Do 6): the verbs' one `Set`, its revision
/// under `Expect::or(if_revision, Expect::Create)`.
async fn saved_set(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Form(fields): Form<Vec<(String, String)>>,
) -> Response {
    let written = blocking(&web, move |web| -> Result<_, Failure> {
        taken(
            &fields,
            &["id", "if_revision", "name", "text", "realm", "membership"],
            "a saved search's write",
        )?;
        let field = |name: &str| sent(&fields, name).map(str::to_string);
        let set = saved::Set {
            id: field("id"),
            expect: Expect::or(revision_sent(&fields)?, Expect::Create),
            name: field("name"),
            text: field("text"),
            realm: field("realm"),
            membership: field("membership"),
        };
        let account = web.served()?.entry.username.clone();
        Ok(saved::set(
            &web.cx.frontend,
            Some(&account),
            &set,
            &mut saved::mint,
        )?)
    })
    .await;
    web.written(written, &headers)
}

/// A clear asked first (the owner, 2026-11-07, "Clear should ask
/// first"): the row at the URL's id, shown with the clear's form at its
/// revision, its basis keyed `saved_search`. It writes nothing, so a
/// reload of it writes nothing. An id no row has is refused from the row
/// in hand, offering the list, as an opening is.
async fn saved_ask(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    let asked = blocking(&web, move |web| -> Result<_, Failure> {
        taken(&fields, &["id"], "a saved search's clear, asked first")?;
        let id = sent(&fields, "id").unwrap_or_default().to_string();
        if !saved::is_id(&id) {
            return Err(Failure::Usage(format!(
                "{id:?} is no saved search's id: a page clears a saved search by its id, twelve \
                 hex characters, as the saved searches list it"
            )));
        }
        match web.saved_at(&id)? {
            Some(row) => Ok(row),
            None => Err(listed(
                "saved_search_not_found",
                format!("no saved search has the id {id}"),
            )),
        }
    })
    .await;
    match asked {
        Ok(row) if wants_json(&headers) => document(&row),
        Ok(row) => {
            let doc = serde_json::to_value(&row).unwrap_or_default();
            web.page(
                "Clear a saved search",
                Head::default().with("saved_search", row.revision()),
                &page::saved_ask(&doc, &web.saved_links()),
            )
        }
        Err(failure) => web.failed("Saved search", &failure, &headers),
    }
}

/// A saved search cleared (Do 6) at exactly the revision named.
async fn saved_clear(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Form(fields): Form<Vec<(String, String)>>,
) -> Response {
    let written = blocking(&web, move |web| -> Result<_, Failure> {
        taken(&fields, &["id", "if_revision"], "a saved search's clear")?;
        let id = sent(&fields, "id")
            .ok_or_else(|| Failure::Usage("a clear names the saved search's `id`".to_string()))?;
        let revision = revision_sent(&fields)?.ok_or_else(|| {
            Failure::Usage("a clear names the revision it clears, `if_revision`".to_string())
        })?;
        Ok(saved::clear(
            &web.cx.frontend,
            id,
            Expect::Exactly(revision),
        )?)
    })
    .await;
    web.written(written, &headers)
}

impl Web {
    /// A write's answer (Do 1, the owner's "Redirect to the list"): the
    /// receipt where JSON is asked, its undo spelled as a page's form;
    /// else `303` to the list. A refusal as every refusal is.
    fn written(
        &self,
        written: Result<saved::SavedWritten, Failure>,
        headers: &HeaderMap,
    ) -> Response {
        match written {
            Ok(written) if wants_json(headers) => document(&acquisition_verbs::receipt::saved(
                &written,
                &Paged { web: self },
            )),
            Ok(_) => see_other(&self.link("saved", &[])),
            Err(failure) => self.failed("Saved searches", &failure, headers),
        }
    }
}

/// `303 See Other` to `location`, an absolute path under the token.
fn see_other(location: &str) -> Response {
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(location) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

async fn css() -> Response {
    ([(header::CONTENT_TYPE, "text/css; charset=utf-8")], CSS).into_response()
}

async fn picker() -> Response {
    (
        [(header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
        PICKER,
    )
        .into_response()
}

async fn filter() -> Response {
    (
        [(header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
        FILTER,
    )
        .into_response()
}

async fn window_script() -> Response {
    (
        [(header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
        WINDOW,
    )
        .into_response()
}

async fn count_script() -> Response {
    (
        [(header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
        COUNT,
    )
        .into_response()
}
