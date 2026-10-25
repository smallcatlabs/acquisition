//! `acq web` (C139, the web UI is a verb of the CLI crate): a loopback
//! web UI that is a renderer, not an app (C16). Every page renders one
//! document a verb already produces, and the server opens no store,
//! picks no account and compiles no plan itself: it calls the verbs
//! (C46; `tools/arch-check.sh`'s door grep reads this module). The brief
//! is `roadmap/briefs/4b-build.md`.
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
//!   server's own, before any route runs; the two mutating routes are
//!   POST routes alone, so a GET on one is refused by its method; no
//!   response carries an `Access-Control-*` header. Every response says
//!   `Referrer-Policy: no-referrer`, so the browser's fetch of an item's
//!   art (C140) never carries the token to the image host, and a
//!   content policy admits images from that host alone and scripts and
//!   styles from the server alone.
//! - **The lifecycle is Ctrl-C** (the brief, Do 4): no lock, no reopen,
//!   no idle exit — a server launched from a terminal is a dev server.
//!   The URL is the first line on stdout (`--json`: `{"url": …}`, as
//!   `auth` prints its URL); `--no-browser` prints it alone.
//! - **The door is the human's** (`Frontend::human("web")`, C88): a
//!   submit from a page reaches the daemon as every CLI verb does, and
//!   its jobs say `web` as their channel (C65).
//! - **One corpus, held** (C98, the consumer holds the corpus and checks
//!   currency before every answer): the server holds the last corpus it
//!   loaded for the account it serves and checks `is_current` before
//!   every answer; a stale one, or one of another scope, is loaded again
//!   and swapped. No answer loads per request.
//! - **A page's document as JSON**: a request that accepts
//!   `application/json` is answered with the document the page renders —
//!   the answer, the item, the plan, the jobs — which is how the tests
//!   compare a page with the CLI's `--json` (C11).

mod form;
mod page;

use std::sync::{Arc, Mutex};

use acquisition_plan::RefreshPlan;
use acquisition_protocol::realm::Realm;
use acquisition_search::answer::{Asked, Request};
use acquisition_search::{Corpus, SearchError};
use acquisition_verbs::{Account, Apply, Fault, Frontend, Jobs};
use anyhow::Result;
use axum::Router;
use axum::extract::{Form, Path, Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use serde_json::{Value, json};

use crate::{Ctx, plan_cmd, search_cmd, store_cmd};
use page::{Head, esc, pre, refusal};

/// The channel every write and submit from a page carries (C65).
const VIA: &str = "web";

/// The stylesheet and the picker's script, served as written.
const CSS: &str = include_str!("assets/page.css");
const PICKER: &str = include_str!("assets/picker.js");

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
    account: Account,
    token: String,
    host: String,
    origin: String,
    form: form::Form,
    held: Mutex<Option<Held>>,
}

/// The corpus last loaded, and the scope it was loaded for.
struct Held {
    realm: Option<acquisition_search::Realm>,
    membership: acquisition_search::answer::Membership,
    corpus: Corpus,
}

/// Why a page could not render its document: a search refusal in the
/// search's words, anything else in the CLI's.
enum Failure {
    Search(SearchError),
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

impl Failure {
    fn words(&self) -> String {
        match self {
            Failure::Search(e) => search_cmd::search_error_text(e),
            Failure::Other(e) => crate::failure_words(e),
        }
    }
}

/// `acq web`: bind, mint, print, open, serve until Ctrl-C.
pub(crate) async fn web(account: Option<String>, no_browser: bool, json: bool) -> Result<()> {
    let cx = Ctx {
        frontend: Frontend::human(VIA)?,
        account,
    };
    let served = cx.account()?;
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
        held: Mutex::new(None),
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
        let opened = crate::open_browser(&url);
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
        .route(&at("/refresh"), get(tabs))
        .route(&at("/refresh/selected"), post(selected))
        .route(&at("/plan"), get(plan))
        .route(&at("/apply"), post(apply))
        .route(&at("/jobs"), get(jobs))
        .route(&at("/assets/page.css"), get(css))
        .route(&at("/assets/picker.js"), get(picker))
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
        HeaderValue::from_static("no-referrer"),
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
    fn page(&self, title: &str, head: Head, body: &str) -> Response {
        html(StatusCode::OK, page::page(&self.token, title, head, body))
    }

    /// A refusal as a page, or as `{"error"}` where JSON is asked (C11).
    fn failed(&self, title: &str, failure: &Failure, headers: &HeaderMap) -> Response {
        let words = failure.words();
        if wants_json(headers) {
            return (
                StatusCode::BAD_REQUEST,
                [(header::CONTENT_TYPE, "application/json")],
                json!({ "error": words }).to_string(),
            )
                .into_response();
        }
        html(
            StatusCode::BAD_REQUEST,
            page::page(&self.token, title, Head::default(), &refusal(&words)),
        )
    }

    /// The request answered over the held corpus (C98): checked first,
    /// then the corpus reused where it is current and of the request's
    /// scope, else loaded and swapped.
    fn answer(&self, request: &Request) -> Result<acquisition_search::Answer, Failure> {
        request.check()?;
        let opened = self.account.open()?;
        let mut held = self.held.lock().unwrap_or_else(|p| p.into_inner());
        let reuse = match held.as_ref() {
            Some(h)
                if h.realm == request.scope.realm && h.membership == request.scope.membership =>
            {
                h.corpus.is_current(&opened.facts, &opened.intent)?
            }
            _ => false,
        };
        if !reuse {
            *held = Some(Held {
                realm: request.scope.realm.clone(),
                membership: request.scope.membership,
                corpus: request.corpus(&opened.facts, &opened.intent)?,
            });
        }
        match held.as_ref() {
            Some(h) => Ok(acquisition_search::answer(&h.corpus, request)?),
            None => Err(Failure::Other(anyhow::anyhow!("no corpus is held"))),
        }
    }

    /// The leagues the corpus of a realm holds, each with its items: the
    /// counts-by-league answer, as `acq search --count league` asks it.
    fn leagues(&self, realm: &str) -> Result<Vec<(String, usize)>, Failure> {
        let asked = Asked::read(&json!({
            "scope": { "realm": realm },
            "view": { "counts": { "keys": ["league"] } },
        }))?;
        let answer =
            serde_json::to_value(self.answer(&asked.request)?).map_err(anyhow::Error::from)?;
        let buckets = answer["view"]["counts"]["tables"][0]["buckets"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        Ok(buckets
            .iter()
            .filter(|b| b["bucket"] == "value")
            .filter_map(|b| {
                let count = b["count"].as_u64().or_else(|| b["count"]["count"].as_u64());
                Some((
                    b["value"].as_str()?.to_string(),
                    count.unwrap_or(0) as usize,
                ))
            })
            .collect())
    }
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
        Err(failure) => (Vec::new(), refusal(&failure.words())),
    })
    .await;
    let body = format!("{said}{}", page::search_form(&web.form, &shown, &leagues));
    web.page(
        "Search",
        Head {
            picker: true,
            ..Head::default()
        },
        &body,
    )
}

/// The answer (Do 6): a form submission folded into the trade JSON, or a
/// continuation's `next`, answered over the held corpus.
async fn search(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    let asked = blocking(&web, move |web| -> Result<_, Failure> {
        let request = match pick(&fields, "next") {
            Some(token) => Request::from_token(token)?,
            None => {
                let folded = form::fold(&fields).map_err(|e| Failure::Other(anyhow::anyhow!(e)))?;
                Asked::read(&folded.request())?.request
            }
        };
        web.answer(&request)
    })
    .await;
    match asked {
        Ok(answer) if wants_json(&headers) => document(&answer),
        Ok(answer) => web.page("Answer", Head::default(), &page::answer(&answer)),
        Err(failure) => web.failed("Answer", &failure, &headers),
    }
}

/// One item (Do 6): `show`'s document, with its art and its badges.
async fn item(State(web): State<Arc<Web>>, headers: HeaderMap, Path(id): Path<String>) -> Response {
    let shown = blocking(&web, move |web| -> Result<_, Failure> {
        let opened = web.account.open()?;
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
            Head::default(),
            &page::item(&shown, &page::item_badges(&shown)),
        ),
        Err(failure) => web.failed("Item", &failure, &headers),
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
/// `tabs` document whole and the characters listing, no default ticked.
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
        let facts = web.account.facts()?;
        let listed = acquisition_search::tabs::Tabs::of(
            realm.as_str(),
            &league,
            facts.tabs(realm.as_str(), &league)?,
            &acquisition_search::tabs::Selection::default(),
            true,
        );
        let characters = facts.characters(Some(realm.as_str()), Some(&league))?;
        let empty = (listed.tabs == 0).then(|| store_cmd::no_tabs_text(realm, &league, &facts));
        Ok((league, listed, characters, empty))
    })
    .await;
    let (league, listed, characters, empty) = match read {
        Ok(read) => read,
        Err(failure) => return web.failed("Refresh", &failure, &headers),
    };
    if wants_json(&headers) {
        return document(&json!({ "tabs": listed, "characters": characters }));
    }
    let now = acquisition_store::now();
    let mut body = format!(
        "<form method=\"get\" action=\"refresh\"><label class=\"control\">Realm <select name=\"realm\">{}</select></label> <label class=\"control\">League <input name=\"league\" value=\"{}\"></label> <button type=\"submit\">Show</button></form>\n",
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
        esc(&league)
    );
    body.push_str(&format!(
        "<form method=\"post\" action=\"refresh/selected\">\n<input type=\"hidden\" name=\"realm\" value=\"{}\"><input type=\"hidden\" name=\"league\" value=\"{}\">\n",
        esc(realm.as_str()),
        esc(&league)
    ));
    match empty {
        Some(said) => body.push_str(&pre(&said)),
        None => {
            body.push_str(&pre(&store_cmd::tabs_head(&listed, realm, &league)));
            body.push_str("<table>\n<tr><th></th><th>Tab</th><th>Type</th><th>Items</th><th>Fetched</th></tr>\n");
            for tab in &listed.rows {
                body.push_str(&format!(
                    "<tr><td><input type=\"checkbox\" name=\"tab\" value=\"{}\"></td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                    esc(&tab.id),
                    esc(&tab.name),
                    esc(&tab.r#type),
                    esc(&store_cmd::items_cell(tab.fetched_at, tab.item_count)),
                    esc(&store_cmd::ago(now, tab.fetched_at))
                ));
            }
            body.push_str("</table>\n<label class=\"control\"><input type=\"checkbox\" name=\"deep\" value=\"true\"> also fetch a map or unique tab's substashes</label>\n");
        }
    }
    if characters.is_empty() {
        body.push_str(&pre(&format!(
            "no characters known in {}{league} (run `acq characters` or cover them in the sync policy)",
            store_cmd::realm_prefix(realm)
        )));
    } else {
        body.push_str("<table>\n<tr><th></th><th>Character</th><th>Level</th><th>Items</th><th>Fetched</th></tr>\n");
        for c in &characters {
            body.push_str(&format!(
                "<tr><td><input type=\"checkbox\" name=\"character\" value=\"{}\"></td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                esc(&c.name),
                esc(&c.name),
                c.level.map(|l| l.to_string()).unwrap_or_default(),
                esc(&store_cmd::items_cell(c.fetched_at, c.item_count)),
                esc(&store_cmd::ago(now, c.fetched_at))
            ));
        }
        body.push_str("</table>\n");
    }
    body.push_str("<button type=\"submit\">Refresh selected</button> <button type=\"submit\" formmethod=\"get\" formaction=\"plan\">Plan under the policy</button>\n</form>\n");
    web.page("Refresh", Head::default(), &body)
}

/// "Refresh selected" (Do 7): the ad-hoc `refresh` kind with the ticked
/// tabs, as `refresh --tabs` submits it, and a `character` job per
/// ticked character, as `acq character` does, through the human door.
async fn selected(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Form(fields): Form<Vec<(String, String)>>,
) -> Response {
    let realm = match realm_of(&fields) {
        Ok(realm) => realm,
        Err(failure) => return web.failed("Refresh", &failure, &headers),
    };
    let league = pick(&fields, "league").unwrap_or(acquisition_plan::DEFAULT_LEAGUE);
    let named = |name: &str| -> Vec<String> {
        fields
            .iter()
            .filter(|(k, v)| k == name && !v.is_empty())
            .map(|(_, v)| v.clone())
            .collect()
    };
    let (tabs, characters) = (named("tab"), named("character"));
    if tabs.is_empty() && characters.is_empty() {
        let failure = Failure::Other(anyhow::anyhow!(
            "nothing is selected: tick a tab or a character to refresh it"
        ));
        return web.failed("Refresh", &failure, &headers);
    }
    let deep = pick(&fields, "deep").is_some();
    let submitted = async {
        let mut client = crate::connect(&web.cx).await?;
        let account = web.cx.account.as_deref();
        if !tabs.is_empty() {
            acquisition_verbs::jobs::submit(
                &web.cx.frontend,
                &mut client,
                "refresh",
                crate::refresh_params(realm, league, false, &tabs, deep),
                0,
                account,
            )
            .await?;
        }
        for name in &characters {
            acquisition_verbs::jobs::submit(
                &web.cx.frontend,
                &mut client,
                "character",
                crate::character_params(realm, name),
                0,
                account,
            )
            .await?;
        }
        anyhow::Ok(())
    }
    .await;
    match submitted {
        Ok(()) => to_jobs(&web),
        Err(e) => web.failed("Refresh", &Failure::Other(e), &headers),
    }
}

fn to_jobs(web: &Web) -> Response {
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(&format!("/{}/jobs", web.token)) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

/// "Plan under the policy" (Do 7): `Opened::plan` rendered as `--plan`
/// renders it, the envelope whole in a hidden field beside "Apply".
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
    let planned = async {
        let mut opened = web.account.open()?;
        opened.plan(realm, &league).await
    }
    .await;
    let planned = match planned {
        Ok(planned) => planned,
        Err(fault) => return web.failed("Plan", &Failure::from(fault), &headers),
    };
    if wants_json(&headers) {
        return document(&planned);
    }
    let now = acquisition_store::now();
    let mut body = pre(&plan_cmd::render_plan(
        &planned.plan,
        planned.quote_note.as_ref(),
        &planned.notes,
        now,
        false,
        None,
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
    web.page("Plan", Head::default(), &body)
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
        let mut opened = web.account.open()?;
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
    let listed = match web.cx.frontend.jobs().await {
        Ok(listed) => listed,
        Err(fault) => return web.failed("Jobs", &Failure::from(fault), &headers),
    };
    match listed {
        Jobs::Live(jobs) => {
            if wants_json(&headers) {
                return document(&jobs);
            }
            let running = jobs.iter().any(|j| !j.state.is_terminal());
            let head = Head {
                refresh: running.then_some(JOBS_REFRESH),
                ..Head::default()
            };
            web.page("Jobs", head, &pre(&crate::jobs_text(&jobs, false)))
        }
        absent => {
            if wants_json(&headers) {
                return document(&absent);
            }
            match crate::persisted_queue_text(&absent) {
                Ok(text) => web.page("Jobs", Head::default(), &pre(&text)),
                Err(e) => web.failed("Jobs", &Failure::Other(e), &headers),
            }
        }
    }
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
