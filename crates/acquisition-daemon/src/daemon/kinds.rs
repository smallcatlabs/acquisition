//! What each job kind does and what it records: the kinds' one match, the
//! API send and its bearers, and apply's admission.
//!
//! Rulings: `decisions/daemon.md`, `decisions/plans.md`; the cross-cutting ones are `AGENTS.md`'s.
//!
//! # Decisions as recorded
//!
//! ## C43 — Apply is its own pure fan-out parent job kind (`apply`), never the `refresh` parent.
//!
//! **Apply is its own pure fan-out parent job kind (`apply`), never the `refresh` parent.** The refresh parent re-lists by construction — it fans out from the listing it just fetched — which contradicts "executes exactly the listed actions": a plan's listing is an optional action and its fetches derive from reviewed facts. So `apply`'s params carry the plan's actions as explicit `(kind, params)` child tuples; the parent performs no send of its own, submits exactly one child per tuple, and holds for them. "Never expands" is structural, not disciplinary: the daemon stays plan-blind (it cannot link the plan crate, and it reads no intent — C34; the store it links only to write facts and keep its queue), so what it admits is **vocabulary, not meaning** — only single-request kinds (`stashes`, or `stash` with `deep` false), each of which submits no children. Admission is at submit, before a job id exists: a malformed tuple list, an empty one, one longer than `APPLY_MAX_ACTIONS`, or a logical bound over the caller's `max_requests` refuses the submit whole — the mid-fan-out terminalization path is never the budget's normal mechanism (D8). Plan validation, the staleness check, and rendering actions to tuples are the frontend's (`acq refresh --apply`, through the planner's validating parse). The ad-hoc `refresh` kind (`--all`/`--tabs`) stood beside it as the explicit client-stated-selection surface until the 4b sitting round retired it (C44 as amended: a selection is a plan too, applied here). Decided 2026-09-01.
//!

use super::*;

/// What a network call can fail with; 429 is its own arm so the job can
/// be re-queued rather than failed.
#[derive(Debug)]
pub(super) enum ApiError {
    RateLimited(String),
    Protocol(String),
    Other(String),
    /// C25: a halt refused a send this job needed, after the pick (its
    /// own, its refresh's, or its probe's), or the block that sets one
    /// answered it. GGG saw nothing of the job: it returns to waiting.
    Halted(String),
    /// C25: the job was cancelled while it waited at the choke point
    /// (its own send's admission, or its refresh's), and nothing of it
    /// was sent. It concludes cancelled.
    Cancelled,
}

/// C25: a token refusal reaching its job. A halt's turns the job away to
/// wait for the reset; a refresh called off when its owner was cancelled
/// reaches a job only once that job is cancelled too (one still wanting
/// the token asks again), and it concludes cancelled; any other fails it
/// in the refusal's words.
fn refused(refusal: Refusal) -> Result<Outcome, ApiError> {
    turned_away(refusal, |message| Ok(Outcome::failure(message)))
}

/// [`refused`]'s split, with what any other refusal becomes given by the
/// caller: the one place a refusal is turned into cancelled, halted or a
/// failure.
fn turned_away<T>(
    refusal: Refusal,
    failed: impl FnOnce(String) -> Result<T, ApiError>,
) -> Result<T, ApiError> {
    if refusal.cancelled {
        Err(ApiError::Cancelled)
    } else if refusal.halted {
        Err(ApiError::Halted(refusal.message))
    } else {
        failed(refusal.message)
    }
}

/// How a GET authenticates (C87): through the account's session, whose
/// token is read after gate admission and told of a 401; through a
/// login's staged tokens, which no session holds yet (a 401 fails the
/// login whole, C50); or not at all (the mock's fake data endpoint).
enum Auth {
    None,
    Session { token: String, username: String },
    Staged(String),
}

/// C87 — the session's token, as the choke point reads it after admission.
/// `expected` is the token the job learned before it waited; what goes out
/// is whatever the session holds *now*: the same token, or the one a
/// refresh installed meanwhile, or nothing — the token was rejected while
/// this send waited, and the send does not go out. A rejection reaches
/// the session through `rejected`, under the rejecting send's permit.
pub(super) struct SessionBearer<'a> {
    daemon: &'a Daemon,
    username: &'a str,
    expected: &'a str,
    /// Set when `rejected` cleared the session's token: the job's error
    /// says so.
    cleared: std::sync::atomic::AtomicBool,
}

impl<'a> SessionBearer<'a> {
    pub(super) fn new(daemon: &'a Daemon, username: &'a str, expected: &'a str) -> Self {
        SessionBearer {
            daemon,
            username,
            expected,
            cleared: std::sync::atomic::AtomicBool::new(false),
        }
    }

    fn was_cleared(&self) -> bool {
        self.cleared.load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl Bearer for SessionBearer<'_> {
    fn token(&self) -> Option<String> {
        let s = self.daemon.shared.lock().unwrap();
        let session = s.auth.find(self.username)?;
        let current = session.access_token.clone()?;
        if current != self.expected {
            self.daemon.log(&format!(
                "send for {} carries the token a refresh installed while it waited",
                self.username
            ));
        }
        Some(current)
    }

    fn rejected(&self, token: &str) {
        if self.daemon.reject_access_token(token).is_some() {
            self.cleared
                .store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }
}

/// A login's staged access token (C50): held outside the session map, so
/// there is nothing to re-read or to forget — a 401 fails the login whole.
struct StagedBearer(String);

impl Bearer for StagedBearer {
    fn token(&self) -> Option<String> {
        Some(self.0.clone())
    }

    fn rejected(&self, _token: &str) {}
}

impl Daemon {
    /// Apply one change to the account index, reporting failure to the
    /// caller. Called under `credentials`, never under `shared` (C148);
    /// it never calls `note_error`. No-op in tests (no store directory).
    pub(super) fn index_apply(
        &self,
        f: impl FnOnce(&mut Index) -> anyhow::Result<()>,
    ) -> Result<(), String> {
        let Some(dir) = &self.store_dir else {
            return Ok(());
        };
        Index::load(dir)
            .and_then(|mut index| f(&mut index))
            .map_err(|e| {
                format!(
                    "accounts index: {}",
                    bounded_message(&format_args!("{e:#}"), "")
                )
            })
    }

    /// Apply one change to the account index under `credentials`, on a
    /// blocking thread (C144: every index write is serialised there, as
    /// login's is, so two accounts' load-modify-save never interleave;
    /// C148: never under `shared`, never on a runtime worker), logging
    /// (never failing) on error. Not for a landing, which holds
    /// `credentials` already and uses `index_apply`.
    pub(super) async fn with_index(
        self: &Arc<Self>,
        f: impl FnOnce(&mut Index) -> anyhow::Result<()> + Send + 'static,
    ) {
        let applied = self
            .spawn_landing(move |daemon| {
                let _serialised = daemon.credentials.lock().unwrap();
                daemon.index_apply(f)
            })
            .await
            .unwrap_or_else(|e| Err(format!("accounts index: the write did not finish: {e}")));
        if let Err(e) = applied {
            self.note_error(&e);
        }
    }

    /// Hand a successful API body to the shared store. The daemon's whole
    /// involvement: endpoint + params + body; what is inside is the store's
    /// business. The result is classified: a body the store refuses as
    /// malformed (`acquisition_store::MalformedBody` — the response itself
    /// is bad, and ingesting it would have poisoned facts) comes back as
    /// the job's `Outcome::Failure`, so the caller returns it instead of
    /// reporting success; the store keeps that body verbatim in its
    /// `refused` table and the failure names the row, so the finding is
    /// readable without a re-fetch. The list-shaped jobs also pre-check
    /// their top-level array (a nicer early failure, and `refresh` needs
    /// the list anyway); entry-level judgment lives in the store alone and
    /// propagates from here.
    ///
    /// C144 — the daemon reports no success it did not earn: a store that
    /// cannot be opened (a schema newer than this daemon's, a file that is
    /// not a store) or written after the send, or a job with no account to
    /// file it under, fails the job too, in a sentence that names the
    /// store's fault. The send was made, so the failure keeps what it
    /// fetched as its payload (`Outcome::failure_keeping`): the params and
    /// the body, which reach the client that asked.
    pub(super) fn record(
        &self,
        account: Option<&str>,
        kind: &str,
        params: &Value,
        body: &Value,
    ) -> Result<(), Outcome> {
        let Some(dir) = &self.store_dir else {
            return Ok(());
        };
        let Some(endpoint) = Endpoint::from_job(kind, params) else {
            return Ok(());
        };
        let not_recorded = |fault: String| {
            self.note_error(&format!("store: {kind} not recorded: {fault}"));
            Outcome::failure_keeping(
                format!("the response was fetched, but the store could not record it: {fault}"),
                json!({ "params": params, "body": body }),
            )
        };
        // The job's account — fixed at submit — selects the file, never the
        // session at landing time: a login as B while A's refresh is still
        // landing must not file A's tabs under B.
        let Some(account) = account else {
            return Err(not_recorded(format!(
                "the job has no account to file the {kind} response under"
            )));
        };
        let mut guard = self.store.lock().unwrap();
        if guard.as_ref().is_none_or(|(u, _)| u != account) {
            let path = account_path(dir, account);
            match Store::open(&path) {
                Ok(store) => {
                    self.log(&format!("store: {} opened for {account}", path.display()));
                    *guard = Some((account.to_string(), store));
                }
                Err(e) => {
                    drop(guard);
                    return Err(not_recorded(format!(
                        "could not open {}: {e:#}",
                        path.display()
                    )));
                }
            }
        }
        let Some((_, store)) = guard.as_mut() else {
            unreachable!("the store was opened above")
        };
        let result = store.record(&endpoint, params, 200, body, acquisition_store::now());
        drop(guard);
        match result {
            Ok(ingest) => {
                self.log(&format!(
                    "store: {kind} {} -> response {} | {} items (+{} ~{} >{} -{}){}",
                    target_of(kind, params),
                    ingest.response_id,
                    ingest.items,
                    ingest.added,
                    ingest.changed,
                    ingest.moved,
                    ingest.removed,
                    match ingest.withheld {
                        Some(n) => format!(
                            " | WITHHELD: the location was retired by a listing; {n} item fact(s) kept on the response row, nothing landed"
                        ),
                        None => String::new(),
                    }
                ));
                Ok(())
            }
            Err(e) => match e.downcast_ref::<acquisition_store::MalformedBody>() {
                Some(malformed) => Err(Outcome::failure(malformed.to_string())),
                None => Err(not_recorded(format!("recording failed: {e:#}"))),
            },
        }
    }

    /// C127 — render, then authenticate: the URL a session's job sends
    /// on, its bearer, and the username it runs as. A row whose params
    /// render no URL fails in the door's words before any token is taken,
    /// so it never spends a refresh nor reports a session failure in
    /// place of its own. The login's own profile job runs on the staged
    /// tokens (C144, `staged_profile_token`): the account may have no
    /// registered session yet, or a dead one being replaced — the new
    /// tokens are the ones being proven. Any other job, a client's profile
    /// in the same window included, runs on the session. What stops the job is what
    /// `execute_inner` returns: a failure, or a halt's refusal (C25).
    async fn render_then_authenticate(
        self: &Arc<Self>,
        id: JobId,
        kind: &str,
        params: &Value,
        account: Option<&str>,
    ) -> Result<(String, Auth, String), Result<Outcome, ApiError>> {
        let Some((_, url)) = self.route_for(kind, params) else {
            return Err(Ok(Outcome::failure(why_no_url(kind, params))));
        };
        let staged = (kind == "profile")
            .then(|| self.staged_profile_token(id, account))
            .flatten();
        let (auth, username) = match staged {
            Some((token, username)) => (Auth::Staged(token), username),
            None => {
                let (token, username) = self
                    .valid_access_token(account, false, Some(id))
                    .await
                    .map_err(refused)?;
                (
                    Auth::Session {
                        token,
                        username: username.clone(),
                    },
                    username,
                )
            }
        };
        Ok((url, auth, username))
    }

    pub(super) async fn execute_inner(
        self: &Arc<Self>,
        id: JobId,
        kind: &str,
        params: Value,
        route: Option<String>,
        account: Option<&str>,
        ready: Instant,
    ) -> Result<Outcome, ApiError> {
        Ok(match kind {
            // The one real API call: GET {api_base}/character. Same code in
            // both modes; only the provider's base URL differs. The choke
            // point classifies and observes every response; the dispatcher
            // requeues only acceptable 429s, while Cloudflare-shaped blocks
            // are never retried.
            "characters" => {
                let (url, auth, username) = match self
                    .render_then_authenticate(id, kind, &params, account)
                    .await
                {
                    Ok(rendered) => rendered,
                    Err(stop) => return stop,
                };
                let route = route.as_deref().expect("characters is a network kind");
                let (v, rate) = self.api_get(id, route, &url, auth, ready).await?;
                // A 2xx without the `characters` array is a malformed
                // response, not an empty account: the job fails and
                // nothing is recorded (the store would refuse it too —
                // ingesting it as empty would retire every character).
                let Some(characters) = v.get("characters").and_then(Value::as_array) else {
                    return Ok(Outcome::failure(
                        "characters response without a `characters` array",
                    ));
                };
                let characters = json!(characters);
                if let Err(failure) = self.record(account, kind, &params, &v) {
                    return Ok(failure);
                }
                Outcome::Success {
                    payload: json!({
                        "provider": self.provider.name,
                        "username": username,
                        "characters": characters,
                        "rate_limit": rate,
                    }),
                }
            }
            "character" | "leagues" | "profile" => {
                let (url, auth, username) = match self
                    .render_then_authenticate(id, kind, &params, account)
                    .await
                {
                    Ok(rendered) => rendered,
                    Err(stop) => return stop,
                };
                let route = route.as_deref().expect("network kind");
                let (v, rate) = self.api_get(id, route, &url, auth, ready).await?;
                if let Err(failure) = self.record(account, kind, &params, &v) {
                    return Ok(failure);
                }
                // The profile's uuid is the stable account identity,
                // required at login (`complete_login` writes the index
                // entry). Recording it whenever a later profile lands too
                // keeps the mapping fresh across renames and backfills
                // pre-uuid entries.
                if kind == "profile"
                    && let (Some(account), Some(uuid)) =
                        (account, v.get("uuid").and_then(Value::as_str))
                {
                    {
                        let mut s = self.shared.lock().unwrap();
                        if let Some(session) = s.auth.find_mut(account) {
                            session.uuid = Some(uuid.to_string());
                        }
                    }
                    let (account, uuid) = (account.to_string(), uuid.to_string());
                    self.with_index(move |index| index.set_uuid(&account, &uuid))
                        .await;
                }
                Outcome::Success {
                    payload: json!({
                        "provider": self.provider.name,
                        "username": username,
                        "params": params,
                        "body": v,
                        "rate_limit": rate,
                    }),
                }
            }
            // The stash list: one request under stash-list-request-limit, the
            // second real policy. Tab contents are a later step.
            "stashes" => {
                let (url, auth, username) = match self
                    .render_then_authenticate(id, kind, &params, account)
                    .await
                {
                    Ok(rendered) => rendered,
                    Err(stop) => return stop,
                };
                let league = params
                    .get("league")
                    .and_then(Value::as_str)
                    .unwrap_or("Standard")
                    .to_string();
                let route = route.as_deref().expect("stashes is a network kind");
                let (v, rate) = self.api_get(id, route, &url, auth, ready).await?;
                // A 2xx without the `stashes` array is a malformed
                // response, not an empty account: the job fails and
                // nothing is recorded — ingested as empty it would retire
                // every tab and mint a false listing basis.
                let Some(stashes) = v.get("stashes").and_then(Value::as_array) else {
                    return Ok(Outcome::failure(
                        "stashes response without a `stashes` array",
                    ));
                };
                let stashes = json!(stashes);
                if let Err(failure) = self.record(account, kind, &params, &v) {
                    return Ok(failure);
                }
                Outcome::Success {
                    payload: json!({
                        "provider": self.provider.name,
                        "username": username,
                        "league": league,
                        "stashes": stashes,
                        "rate_limit": rate,
                    }),
                }
            }
            "stash" => {
                let (url, auth, _) = match self
                    .render_then_authenticate(id, kind, &params, account)
                    .await
                {
                    Ok(rendered) => rendered,
                    Err(stop) => return stop,
                };
                let route = route.as_deref().expect("stash is a network kind");
                let (v, rate) = self.api_get(id, route, &url, auth, ready).await?;
                if let Err(failure) = self.record(account, kind, &params, &v) {
                    return Ok(failure);
                }
                let stash = v.get("stash").cloned().unwrap_or(v);
                // Map/unique tabs carry their substashes as stubs; following
                // them is opt-in per tab (--deep) because one map tab can
                // hold hundreds. Each substash becomes a child job.
                let deep = params.get("deep").and_then(Value::as_bool).unwrap_or(false);
                let children = stash
                    .get("children")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default();
                let mut submitted = Vec::new();
                if deep {
                    let realm = Realm::from_params(&params).unwrap_or(Realm::DEFAULT);
                    let league = params.get("league").cloned().unwrap_or(json!("Standard"));
                    let tab = params.get("id").cloned().unwrap_or(Value::Null);
                    for child in &children {
                        let Some(sub) = child.get("id").and_then(Value::as_str) else {
                            continue;
                        };
                        match self.submit_child(
                            id,
                            "stash",
                            json!({ "realm": realm, "league": league, "id": tab, "sub": sub, "deep": false }),
                        ) {
                            Ok(cid) => submitted.push(cid),
                            Err(e) => {
                                return Ok(fan_out_stopped(
                                    self.cancelled(id),
                                    submitted.len(),
                                    &e.message,
                                ));
                            }
                        }
                    }
                }
                Outcome::Success {
                    payload: json!({
                        "provider": self.provider.name,
                        "stash": stash,
                        "substashes_listed": children.len(),
                        "substash_jobs": submitted,
                        "rate_limit": rate,
                    }),
                }
            }
            // A plan's action set, executed exactly (tracer step 7): a pure
            // fan-out parent. Each admitted tuple becomes one child job; the
            // parent sends nothing itself, so it has no route and no token —
            // children authenticate and pace themselves. Admission validated
            // the vocabulary at submit; it is checked again here because a
            // restored `daemon.db` row was admitted by an earlier lifetime,
            // possibly an earlier build.
            "apply" => {
                if let Err(error) = validate_apply(&params) {
                    return Ok(Outcome::failure(error.to_string()));
                }
                let jobs = params
                    .get("jobs")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default();
                let mut submitted = Vec::new();
                for job in &jobs {
                    let kind = job.get("kind").and_then(Value::as_str).unwrap_or_default();
                    let child_params = job.get("params").cloned().unwrap_or_else(|| json!({}));
                    match self.submit_child(id, kind, child_params) {
                        Ok(cid) => submitted.push(cid),
                        Err(e) => {
                            return Ok(fan_out_stopped(
                                self.cancelled(id),
                                submitted.len(),
                                &e.message,
                            ));
                        }
                    }
                }
                Outcome::Success {
                    payload: json!({
                        "provider": self.provider.name,
                        "requests": submitted.len(),
                        "child_jobs": submitted,
                    }),
                }
            }
            "sleep" => {
                // Real seconds by design, not the daemon's clock: the mock's
                // demo and the watch tests want a job that visibly takes time.
                // The door's check of `seconds` (C127), again: a row that
                // never passed the door fails in its words rather than
                // panic in the timer or run a default it would refuse.
                if let Err(words) = check_sleep(&params) {
                    return Ok(Outcome::failure(words));
                }
                let seconds = params.get("seconds").and_then(Value::as_f64).unwrap_or(3.0);
                let deadline = Instant::now() + Duration::from_secs_f64(seconds);
                while Instant::now() < deadline {
                    if self.cancelled(id) {
                        return Ok(Outcome::Cancelled);
                    }
                    tokio::time::sleep(Duration::from_millis(200)).await;
                }
                Outcome::Success {
                    payload: json!({ "slept_seconds": seconds }),
                }
            }
            "fetch" => {
                // A real request to the mock's fake data endpoint, so the
                // limiter learns a second policy from real headers. Never
                // sent to GGG: real mode has no such endpoint.
                if self.provider.is_real() {
                    return Ok(Outcome::failure(
                        "fetch is a mock-only kind; real mode has no fake data endpoint",
                    ));
                }
                let url = format!("{}/fetch", self.provider.api_base);
                let route = route.as_deref().expect("mock fetch is a network kind");
                let (v, rate) = self.api_get(id, route, &url, Auth::None, ready).await?;
                Outcome::Success {
                    payload: json!({
                        "note": "fake data from the in-process mock",
                        "params": params,
                        "items": v.get("items").cloned().unwrap_or(v),
                        "rate_limit": rate,
                    }),
                }
            }
            "whoami" => {
                // The mock-only auth-exercising kind: access-token expiry and
                // silent refresh through the daemon-owned session. Fake data,
                // so in real mode it must not cost a token POST (L0 rail 6).
                if self.provider.is_real() {
                    return Ok(Outcome::failure(
                        "whoami is a mock-only job kind; not run against real GGG",
                    ));
                }
                let (token, username) =
                    match self.valid_access_token(account, false, Some(id)).await {
                        Ok(pair) => pair,
                        Err(error) => return refused(error),
                    };
                tokio::time::sleep(Duration::from_millis(300)).await;
                if self.cancelled(id) {
                    return Ok(Outcome::Cancelled);
                }
                Outcome::Success {
                    payload: json!({
                        "note": "fake data — the mock never talks to GGG",
                        "username": username,
                        "league": "Standard",
                        "characters": [
                            { "name": "StashHoarder", "class": "Scion", "level": 97 },
                            { "name": "MuleQuadTab", "class": "Witch", "level": 12 },
                        ],
                        "authorized_with": format!("{}…", token.chars().take(11).collect::<String>()),
                    }),
                }
            }
            // The HEAD probe (N16): discovers an endpoint's policy and the
            // account's current counters before the first real send, without
            // counting against the policy (N24). Submitted by the daemon
            // itself; visible like any other job so it can be inspected.
            "probe" => {
                let (Some(route), Some(url)) = (
                    params
                        .get("route")
                        .and_then(Value::as_str)
                        .map(str::to_string),
                    params
                        .get("url")
                        .and_then(Value::as_str)
                        .map(str::to_string),
                ) else {
                    return Ok(Outcome::failure("probe needs a route and a url"));
                };
                let mut session = if self.needs_auth(&route) {
                    match self.valid_access_token(account, false, Some(id)).await {
                        Ok(pair) => Some(pair),
                        // C25: a halt closes nothing; the probe waits too,
                        // and a cancel closes nothing either.
                        Err(error) => {
                            return turned_away(error, |message| {
                                // Close the endpoint too, or the waiting job
                                // would just ask for another probe; login
                                // reopens it.
                                self.choke.degrade(&route, &message);
                                Ok(Outcome::failure(message))
                            });
                        }
                    }
                } else {
                    None
                };
                let mut retried = false;
                let wanted = || !self.cancel_asked(id);
                let (probed, cleared_for) = loop {
                    let bearer = session
                        .as_ref()
                        .map(|(token, username)| SessionBearer::new(self, username, token));
                    let probed = self
                        .choke
                        .head(
                            &route,
                            &url,
                            bearer.as_ref().map(|b| b as &dyn Bearer),
                            ready,
                            &wanted,
                        )
                        .await;
                    let cleared_for = bearer
                        .as_ref()
                        .filter(|b| b.was_cleared())
                        .map(|b| b.username.to_string());
                    match probed {
                        // C87: the token this probe was to carry was rejected
                        // while it waited for the exclusive permit. Refresh
                        // outside it and try admission once more.
                        Err(ProbeError::StaleBearer) if !retried => {
                            retried = true;
                            let (_, username) = session
                                .as_ref()
                                .expect("only a session's bearer can go stale");
                            match self
                                .valid_access_token(Some(username), false, Some(id))
                                .await
                            {
                                Ok(pair) => session = Some(pair),
                                Err(error) => {
                                    return turned_away(error, |message| {
                                        self.choke.degrade(&route, &message);
                                        Ok(Outcome::failure(message))
                                    });
                                }
                            }
                        }
                        other => break (other, cleared_for),
                    }
                };
                self.announce_trip();
                match probed {
                    Ok((status, policy, headers)) => {
                        let name = policy.name;
                        self.log(&format!(
                            "HEAD {} -> {status} | policy {name} | {headers}",
                            url_path(&url)
                        ));
                        Outcome::Success {
                            payload: json!({
                                "route": route,
                                "endpoint": url_path(&url),
                                "status": status.as_u16(),
                                "policy": name,
                                "rate_limit": headers,
                            }),
                        }
                    }
                    // C25: a probe a halt refused, or Cloudflare answered,
                    // waits like any job. Failed, it would fail every job
                    // waiting on its route.
                    // C25: cancelled before the HEAD went out; the route
                    // is untouched.
                    Err(ProbeError::Cancelled) => return Err(ApiError::Cancelled),
                    Err(error @ ProbeError::Failed { halted: true, .. }) => {
                        return Err(ApiError::Halted(format!(
                            "HEAD {}: {error}",
                            url_path(&url)
                        )));
                    }
                    Err(error) => {
                        let mut message = format!("HEAD {}: {error}", url_path(&url));
                        if let Some(username) = cleared_for {
                            message.push_str(&format!(
                                " — the access token for {username} was rejected and is forgotten; the next send refreshes it first"
                            ));
                            // The limiter closed the route with the bare
                            // HEAD status as its reason; the job waiting
                            // on this probe reports that reason, so it
                            // is re-closed with the whole story.
                            self.choke.degrade(&route, &message);
                        }
                        Outcome::failure(message)
                    }
                }
            }
            // The door refuses a kind outside the table (C127); this is a
            // restored row from a build with another table.
            other => Outcome::failure(format!(
                "unknown job kind {}; a client may submit: {}",
                describe_str(other),
                client_kinds(self.provider.is_real())
            )),
        })
    }

    /// One rate-limited GET: sends with the receipt, logs the rate headers,
    /// and turns non-2xx into a typed error. A 429 is distinguishable so the
    /// caller can re-queue the job behind the limiter's hold (P-A); a
    /// Cloudflare-shaped 403/503 is never retried (invariant 3).
    async fn api_get(
        self: &Arc<Self>,
        id: JobId,
        route: &str,
        url: &str,
        mut auth: Auth,
        ready: Instant,
    ) -> Result<(Value, Value), ApiError> {
        let mut retried = false;
        // C25: a cancel reaches the send until the gate admits it.
        let wanted = || !self.cancel_asked(id);
        loop {
            let (sent, cleared_for) = match &auth {
                Auth::Session { token, username } => {
                    let bearer = SessionBearer::new(self, username, token);
                    let sent = self
                        .choke
                        .get_bearer(route, url, &bearer, ready, &wanted)
                        .await;
                    (sent, bearer.was_cleared().then(|| username.clone()))
                }
                Auth::Staged(token) => (
                    self.choke
                        .get_bearer(route, url, &StagedBearer(token.clone()), ready, &wanted)
                        .await,
                    None,
                ),
                Auth::None => (self.choke.get(route, url, ready, &wanted).await, None),
            };
            let response = match sent {
                // C87: the token this send was to carry was rejected while
                // it waited at the gate. The permit is released; refresh
                // outside it and try admission once more.
                Err(SendError::StaleBearer) if !retried => {
                    retried = true;
                    let Auth::Session { username, .. } = &auth else {
                        unreachable!("only a session's bearer can go stale")
                    };
                    match self
                        .valid_access_token(Some(username), false, Some(id))
                        .await
                    {
                        Ok((token, username)) => {
                            auth = Auth::Session { token, username };
                            continue;
                        }
                        Err(mut refusal) => {
                            refusal.message = format!("GET {}: {}", url_path(url), refusal.message);
                            return turned_away(refusal, |message| Err(ApiError::Other(message)));
                        }
                    }
                }
                Err(SendError::StaleBearer) => {
                    return Err(ApiError::Other(format!(
                        "GET {}: {} — twice; giving up",
                        url_path(url),
                        SendError::StaleBearer
                    )));
                }
                Err(SendError::Protocol(error)) => {
                    return Err(ApiError::Protocol(format!(
                        "GET {}: rate-limit protocol failure: {error}",
                        url_path(url)
                    )));
                }
                Err(SendError::Transport(error)) => {
                    return Err(ApiError::Other(format!("GET {url} failed: {error}")));
                }
                Err(SendError::Halted(cause)) => {
                    return Err(ApiError::Halted(format!(
                        "GET {} refused: {cause}",
                        url_path(url)
                    )));
                }
                Err(SendError::OffOrigin(reason)) => {
                    return Err(ApiError::Other(format!("GET {}: {reason}", url_path(url))));
                }
                Err(SendError::Cancelled) => return Err(ApiError::Cancelled),
                Ok(response) => response,
            };
            self.announce_trip();
            let status = response.status;
            let rate = response.rate;
            let retry_after = response.retry_after;
            let path = url_path(url);
            self.log(&format!("GET {path} -> {status} | rate headers: {rate}"));
            if let Some(verdict) = response
                .verdict
                .filter(|verdict| verdict.block || verdict.unreachable)
            {
                // C129: the choke point has halted, for a block or an
                // unreachable origin. The job fails in the halt's words,
                // true of whatever else it holds (a tripwire's trip beside
                // an unreachable halt outlives a restart), and Cloudflare's
                // page is no word of GGG's to quote, as on the token route.
                return Err(cloudflare_failure(
                    &path,
                    status,
                    &rate,
                    verdict,
                    self.choke.rails().halted(),
                ));
            }
            let classified = classify_api_body(
                status,
                &retry_after,
                &path,
                rate,
                response.body,
                response.verdict,
            );
            if let Some(username) = cleared_for {
                return classified.map_err(|error| match error {
                    ApiError::Other(evidence) => ApiError::Other(format!(
                        "{evidence} — the access token for {username} was rejected and is forgotten; the next send refreshes it first"
                    )),
                    other => other,
                });
            }
            return classified;
        }
    }
}

/// The `apply` admission check (C43 — vocabulary, not meaning; C42 — the
/// logical budget at admission; decided 2026-09-01): the vocabulary a
/// plan-blind daemon can enforce. `params.jobs` must be a
/// non-empty array of `(kind, params)` tuples in which every kind is a
/// single-request one that submits no children — `stashes`, `stash` with
/// `deep` absent/false, `characters`, or `character` with a name
/// (characters joined 2026-09-02) — so no child can expand the reviewed set;
/// it carries at most `APPLY_MAX_ACTIONS` tuples, judged before any tuple
/// and before `max_requests`, so an oversized plan is told to narrow its
/// selection whatever its budget says; and when the caller declares `max_requests`,
/// the tuple count must not exceed it. Runs at submit, before a job id
/// exists, so a refusal admits nothing (mid-fan-out terminalization is
/// never the budget's mechanism); a misread budget refuses too — a limit
/// half-honored by ignoring it would spend exactly what the caller tried
/// to cap.
pub(super) fn validate_apply(params: &Value) -> Result<(), Refusal> {
    let refused = |message: String| Refusal::new(ErrorKind::Refused, message);
    let Some(jobs) = params.get("jobs").and_then(Value::as_array) else {
        return Err(refused(
            "apply needs a `jobs` array of {kind, params} tuples (a plan's actions)".into(),
        ));
    };
    if jobs.is_empty() {
        return Err(refused(
            "apply with an empty `jobs` array: a plan with no actions has nothing to execute"
                .into(),
        ));
    }
    if jobs.len() > APPLY_MAX_ACTIONS {
        return Err(refused(format!(
            "an apply of {} actions exceeds the bound of {APPLY_MAX_ACTIONS} actions — \
nothing was submitted; a sync policy that covers fewer tabs or characters plans fewer actions",
            jobs.len()
        )));
    }
    for (i, job) in jobs.iter().enumerate() {
        let kind = job.get("kind").and_then(Value::as_str).unwrap_or("");
        let params = job.get("params").cloned().unwrap_or(Value::Null);
        admit_realm(kind, &params).map_err(|e| refused(format!("action {i}: {e}")))?;
        admit_segments(kind, &params).map_err(|e| refused(format!("action {i}: {e}")))?;
        match kind {
            "stashes" => {}
            "stash" => {
                if params.get("deep").and_then(Value::as_bool).unwrap_or(false) {
                    return Err(refused(format!(
                        "action {i}: a plan's stash fetch never fans out (`deep` must be false)"
                    )));
                }
            }
            // A stash's id and a character's name are required segments,
            // judged by `admit_segments` above (C127).
            "characters" | "character" => {}
            other => {
                return Err(refused(format!(
                    "action {i}: kind {} is not in the plan vocabulary \
                     (stashes, stash, characters, character)",
                    describe_str(other)
                )));
            }
        }
    }
    if let Some(max) = params.get("max_requests") {
        let Some(max) = max.as_u64() else {
            return Err(refused(format!(
                "max_requests must be a non-negative integer, not {}",
                describe(max)
            )));
        };
        if jobs.len() as u64 > max {
            return Err(refused(format!(
                "plan exceeds the budget: {} logical request(s) against max_requests {max} — nothing was submitted",
                jobs.len()
            )));
        }
    }
    Ok(())
}

/// The outcome of a fan-out whose child submission was refused: the
/// parent was cancelled, or the queue failed. Already-submitted children
/// run either way (their sends are theirs); the parent never claims
/// success over a partial set.
fn fan_out_stopped(cancelled: bool, submitted: usize, why: &str) -> Outcome {
    if cancelled {
        Outcome::Cancelled
    } else {
        Outcome::failure(format!("fan-out stopped after {submitted} children: {why}"))
    }
}

/// C129: a GET Cloudflare answered in GGG's place, a block or an origin
/// it could not reach. The status and the rate headers are the evidence,
/// never the body (the page is Cloudflare's, no word of GGG's); `halt` is
/// the rails' own words when the daemon has them, else the remedy of the
/// halt the verdict set.
fn cloudflare_failure(
    path: &str,
    status: reqwest::StatusCode,
    rate: &Value,
    verdict: Verdict,
    halt: Option<String>,
) -> ApiError {
    let halt = halt.unwrap_or_else(|| {
        if verdict.block {
            format!(
                "Cloudflare answered in GGG's place: {}",
                crate::rails::BLOCK_REMEDY
            )
        } else {
            format!(
                "Cloudflare could not reach GGG: {}",
                crate::rails::UNREACHABLE_REMEDY
            )
        }
    });
    ApiError::Other(format!(
        "GET {path} returned {status}; rate headers {rate} — NOT retrying; {halt}"
    ))
}

/// A response's outcome for its job. `verdict` is the choke point's
/// judgement of a non-2xx (C129): a block is terminal and never re-queued,
/// whatever its `Retry-After`, and its shape is read from the whole body,
/// not from the 300 characters another failure's text keeps. A block
/// and an unreachable origin keep none of Cloudflare's page.
pub(super) fn classify_api_body(
    status: reqwest::StatusCode,
    retry_after: &RetryAfter,
    path: &str,
    rate: Value,
    body: Result<String, String>,
    verdict: Option<Verdict>,
) -> Result<(Value, Value), ApiError> {
    // Status owns non-2xx outcomes even if reading the error body also failed
    // (D8). The body error remains in the evidence rather than becoming fake
    // empty content.
    if !status.is_success() {
        if let Some(verdict) = verdict.filter(|verdict| verdict.block || verdict.unreachable) {
            return Err(cloudflare_failure(path, status, &rate, verdict, None));
        }
        let shape = verdict.map_or(BlockShape::Unclassified, |verdict| verdict.shape);
        let body = match body {
            Ok(body) => body.chars().take(300).collect(),
            Err(error) => format!("<body read transport failure: {error}>"),
        };
        let evidence = format!("GET {path} returned {status}; rate headers {rate}; body: {body}");
        return Err(match status.as_u16() {
            429 if retry_after.is_acceptable() => ApiError::RateLimited(evidence),
            429 => ApiError::Other(format!(
                "terminal rate limit: {evidence}; {retry_after}; NOT retrying"
            )),
            403 | 503 => ApiError::Other(format!(
                "{evidence} — {}; NOT retrying (invariant 3)",
                shape.describe()
            )),
            _ => ApiError::Other(evidence),
        });
    }

    let body = body.map_err(|error| {
        ApiError::Other(format!(
            "GET {path} body read transport failure after {status}: {error}"
        ))
    })?;
    serde_json::from_str::<Value>(&body)
        .map(|value| (value, rate))
        .map_err(|error| ApiError::Other(format!("bad JSON from {path}: {error}")))
}
