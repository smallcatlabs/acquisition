//! The choke point: the one HTTP client, the bearer a send carries, the
//! send errors and the classification of a landed response, and every send
//! through the limiter and the gate.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::*;

// ---- the choke point ------------------------------------------------------

/// The bearer of an authenticated send, as the choke point sees it (C87).
/// A job learns its token before it waits for the limiter and the gate,
/// and that wait can be minutes; in the meantime another send may have
/// carried the same token to a 401. So the token is not a string handed
/// in up front: it is *read after admission*, from the session, and the
/// session hears of a rejection *before the rejecting send's permit is
/// released* — the two together mean no send leaves the gate carrying a
/// bearer whose rejection has already landed. Sends admitted alongside
/// the rejected one are on the wire already and may land 401 too.
pub trait Bearer: Send + Sync {
    /// The token this send carries, read once the send holds its permit.
    /// `None`: the token it was going to carry is gone (rejected, C87),
    /// so the send does not go out — the caller releases the permit,
    /// refreshes outside it, and tries admission again.
    fn token(&self) -> Option<String>;
    /// A 401 landed on `token`; called while the permit is still held.
    fn rejected(&self, token: &str);
}

/// Why a HEAD probe did not leave its route with a policy.
#[derive(Debug, Clone)]
pub enum ProbeError {
    /// The bearer read after admission was gone (C87): nothing was sent.
    StaleBearer,
    /// The status the probe landed with (`None` when it never landed —
    /// refused by a halt, or lost in transport) and the limiter's reason.
    /// The status is what lets the daemon act on a 401 (C87) without
    /// parsing the reason.
    Failed { status: Option<u16>, reason: String },
    /// The probe was to carry a bearer to a URL off the provider's API
    /// origin (C128): nothing was sent, and the route is closed for a
    /// cooldown so its waiting job does not ask again at once.
    OffOrigin(String),
}

impl std::fmt::Display for ProbeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProbeError::StaleBearer => f.write_str(STALE_BEARER),
            ProbeError::Failed { reason, .. } => f.write_str(reason),
            ProbeError::OffOrigin(reason) => f.write_str(reason),
        }
    }
}

const STALE_BEARER: &str = "the bearer this send was to carry was rejected before it left the gate";

#[derive(Debug)]
pub enum SendError {
    /// The bearer read after admission was gone (C87): nothing was sent
    /// and the permit is released; refresh, then try admission again.
    StaleBearer,
    Transport(String),
    Protocol(PolicyObservationError),
    /// Refused before any permit by the live-test rails (tripwire or
    /// ceiling) or by a Cloudflare block mark (C129); nothing was sent.
    Halted(String),
    /// Refused before any permit: the send was to carry a bearer to a URL
    /// off the provider's API origin (C128). Nothing was sent.
    OffOrigin(String),
}

/// A landed exchange after its body transfer has resolved. Non-2xx statuses
/// retain their status precedence even when `body` is a transfer failure;
/// clean 2xx transfer failures are returned as `SendError::Transport` before
/// callers can inspect this package (D3/D8). A 3xx's `body` is not the
/// server's: it says where the redirect pointed, redacted (C130).
pub struct CompletedResponse {
    pub status: reqwest::StatusCode,
    pub rate: serde_json::Value,
    pub retry_after: RetryAfter,
    pub body: Result<String, String>,
    /// C129: for a non-2xx, the choke point's one judgement of what it
    /// was — its body's shape, and whether Cloudflare answered in GGG's
    /// place. `None` for a 2xx. Callers act on this, never on the body.
    pub verdict: Option<Verdict>,
}

impl fmt::Display for SendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SendError::StaleBearer => f.write_str(STALE_BEARER),
            SendError::Transport(error) => write!(f, "{error}"),
            SendError::Halted(cause) => write!(f, "{cause}"),
            SendError::OffOrigin(reason) => write!(f, "{reason}"),
            SendError::Protocol(error) => {
                write!(f, "rate-limit protocol failure: {error}")
            }
        }
    }
}

impl std::error::Error for SendError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ResponseClassification {
    /// C129: Cloudflare answered in GGG's place. Terminal, never retried,
    /// and the send's report sets the block mark.
    Blocked,
    Success,
    RateLimited(RetryAfter),
    Http(u16),
    Network,
    Protocol(PolicyObservationError),
}

/// D8 precedence in one total function. Observation is already complete when
/// this runs; it can update pacing but never override status/network outcome.
/// A block (C129, the response's [`Verdict`]) comes first: a 429 that is
/// Cloudflare's is no rate limit of GGG's.
pub(super) fn classify_response(
    status: Option<u16>,
    network_error: bool,
    observation: Result<(), PolicyObservationError>,
    retry_after: &RetryAfter,
    block: bool,
) -> ResponseClassification {
    if block {
        return ResponseClassification::Blocked;
    }
    if status == Some(429) {
        return ResponseClassification::RateLimited(retry_after.clone());
    }
    if let Some(status) = status
        && !(200..300).contains(&status)
    {
        return ResponseClassification::Http(status);
    }
    if network_error {
        return ResponseClassification::Network;
    }
    match observation {
        Ok(()) => ResponseClassification::Success,
        Err(error) => ResponseClassification::Protocol(error),
    }
}

struct SentAt {
    at: Instant,
    endpoint: String,
    method: &'static str,
    url: String,
    outcome: String,
    ok: bool,
}

const SEND_HISTORY: usize = 100;

/// Callers name the *route* they're sending on (`character-list`,
/// `stash-list`, …), not the URL: one route covers every league/id variant
/// of a path, so it gets one probe and one policy, and a per-tab route can
/// never turn into a probe per tab (the shape of the 2024 incident, N2).
pub struct ChokePoint {
    // Private on purpose: this is the only reqwest client in the workspace,
    // so every HTTP request must come through a method that consults the
    // limiter and reports the response back to it.
    http: reqwest::Client,
    pub(super) limiter: Mutex<Limiter>,
    gate: SendGate,
    sends: Mutex<VecDeque<SentAt>>,
    clock: Arc<dyn Clock>,
    rails: Arc<Rails>,
    /// C128 — The one origin a bearer may be sent to: the provider's API
    /// origin, given at construction. `None` (no API base, or one that
    /// does not parse) admits a bearer nowhere.
    bearer_origin: Option<url::Origin>,
}

/// Connect and whole-request (headers + body) timeouts on the one client
/// (L0 rail 3): a hung send must fail as a transport error instead of
/// holding its permit and scheduling key forever.
pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

/// Longest uninterrupted wait inside the choke point's admission loops, so a
/// halt that happens while a task is parked in a hold is honored within
/// about this long rather than at the end of the hold.
const HALT_POLL: Duration = Duration::from_secs(1);

fn build_client() -> reqwest::Client {
    // The user-agent goes on the client itself so no request — token
    // exchange included — can be sent without it (CONTEXT invariant 4).
    // C130: no redirect is followed. A 3xx lands as the response it is,
    // observed and journaled with its `location`; a followed 307 would
    // re-send a token request's form body wherever `Location` points, as
    // a second send under this one's permit and journal line.
    reqwest::Client::builder()
        .user_agent(crate::provider::USER_AGENT)
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(REQUEST_TIMEOUT)
        .build()
        .expect("reqwest client builds")
}

impl Default for ChokePoint {
    fn default() -> Self {
        ChokePoint::new()
    }
}

impl ChokePoint {
    /// Same construction in mock and real mode: the limiter starts empty and
    /// learns policies from responses. Rails are disabled and no API origin
    /// is known, so no bearer is sent anywhere (C128); the daemon uses
    /// `with_clock_and_rails`.
    pub fn new() -> Self {
        ChokePoint::with_rails(Arc::new(Rails::disabled()))
    }

    pub fn with_rails(rails: Arc<Rails>) -> Self {
        ChokePoint {
            http: build_client(),
            limiter: Mutex::new(Limiter::new()),
            gate: SendGate::new(),
            sends: Mutex::new(VecDeque::new()),
            clock: Arc::new(SystemClock),
            rails,
            bearer_origin: None,
        }
    }

    /// The daemon's choke point: its clock, its rails, and the provider's
    /// API base, whose origin is the only one a bearer goes to (C128).
    pub(crate) fn with_clock_and_rails(
        clock: Arc<dyn Clock>,
        rails: Arc<Rails>,
        api_base: &str,
    ) -> Self {
        ChokePoint {
            http: build_client(),
            limiter: Mutex::new(Limiter::new()),
            gate: SendGate::new(),
            sends: Mutex::new(VecDeque::new()),
            clock,
            rails,
            bearer_origin: url::Url::parse(api_base).ok().map(|base| base.origin()),
        }
    }

    /// C128 — A bearer goes only to the provider's API origin: scheme, host
    /// and port, as the URL parser compares them. Judged here, at the one
    /// place a bearer is attached, before any wait or permit, so the
    /// property holds whatever URL a caller hands in. A refused send sent
    /// nothing, so it leaves no journal line, as a halted one leaves none.
    fn admit_bearer_url(&self, url: &str) -> Result<(), String> {
        let origin = url::Url::parse(url).ok().map(|url| url.origin());
        match (&self.bearer_origin, origin) {
            (Some(allowed), Some(origin)) if *allowed == origin => Ok(()),
            (Some(allowed), _) => Err(format!(
                "refused before sending: a bearer goes only to the provider's API origin ({}), not to {}",
                allowed.ascii_serialization(),
                url_origin_words(url)
            )),
            (None, _) => Err(format!(
                "refused before sending: this choke point knows no API origin, so a bearer goes nowhere, not to {}",
                url_origin_words(url)
            )),
        }
    }

    pub fn rails(&self) -> &Arc<Rails> {
        &self.rails
    }

    pub(crate) fn now(&self) -> Instant {
        self.clock.now()
    }

    pub(crate) fn wall(&self) -> SystemTime {
        self.clock.wall()
    }

    pub(crate) async fn sleep(&self, duration: Duration) {
        self.clock.sleep(duration).await;
    }

    /// See `Limiter::declare_policyless`.
    pub fn declare_policyless(&self, route: &str) {
        self.limiter.lock().unwrap().declare_policyless(route);
    }

    /// Ask whether `route` may send now, or learn how long to wait. This is a
    /// pacing hint for the cancellation-aware dispatcher; every transport
    /// method repeats the final check under a live gate permit.
    pub fn check(&self, route: &str) -> Result<(), Duration> {
        let wait = self.limiter.lock().unwrap().wait_for(route, self.now());
        if wait.is_zero() { Ok(()) } else { Err(wait) }
    }

    pub fn eta_for(&self, route: &str, ahead: u32) -> Duration {
        self.limiter
            .lock()
            .unwrap()
            .eta_for(route, ahead, self.now())
    }

    /// Every queue position's ETA on `route` under one limiter lock —
    /// see `Limiter::etas_for`.
    pub fn etas_for(&self, route: &str, count: usize) -> Vec<Duration> {
        self.limiter
            .lock()
            .unwrap()
            .etas_for(route, count, self.now())
    }

    /// The key under which in-flight requests on `route` must be
    /// serialized: its policy name once known (same-name policies share
    /// counters across routes, N6), else the route itself.
    pub fn serial_key(&self, route: &str) -> String {
        match self
            .limiter
            .lock()
            .unwrap()
            .endpoint_state(route, self.now())
        {
            EndpointState::Policy(name) => name,
            _ => route.to_string(),
        }
    }

    /// Acquire an ordinary actual-send permit and repeat the limiter check
    /// immediately before dispatch. The pre-check keeps rate-limit sleeps
    /// permit-free. Rechecking both the route mapping and limiter state after
    /// admission closes races with landed responses, including N33's
    /// `oauth-token` -> `token-request-limit` discovery transition.
    async fn acquire_send(&self, route: &str) -> Result<SendPermit, SendError> {
        // Rails are consulted before any waiting or permit: a halted daemon
        // neither sleeps toward a send nor occupies gate capacity.
        loop {
            // Re-checked every iteration and again after admission: a task
            // parked in a hold sleep or in the gate must not send after a
            // trip that happened while it waited.
            if let Some(cause) = self.rails.halted() {
                return Err(SendError::Halted(cause));
            }
            if let Err(wait) = self.check(route) {
                self.sleep(wait.clamp(Duration::from_millis(50), HALT_POLL))
                    .await;
                continue;
            }

            let serial_key = self.serial_key(route);
            let permit = self.gate.acquire(serial_key.clone()).await;
            if let Some(cause) = self.rails.halted() {
                drop(permit);
                return Err(SendError::Halted(cause));
            }
            let mapping_is_current = self.serial_key(route) == serial_key;
            match (mapping_is_current, self.check(route)) {
                (true, Ok(())) => return Ok(permit),
                (_, pacing) => {
                    drop(permit);
                    if let Err(wait) = pacing {
                        self.sleep(wait.clamp(Duration::from_millis(50), HALT_POLL))
                            .await;
                    }
                }
            }
        }
    }

    /// HEAD uses the same permit-free pacing loop, then takes the gate's
    /// exclusive writer reservation. A final check after admission handles a
    /// landed response that installed a hold while the probe was queued.
    async fn acquire_head(&self, route: &str) -> Result<SendPermit, SendError> {
        loop {
            if let Some(cause) = self.rails.halted() {
                return Err(SendError::Halted(cause));
            }
            if let Err(wait) = self.check(route) {
                self.sleep(wait.clamp(Duration::from_millis(50), HALT_POLL))
                    .await;
                continue;
            }

            let permit = self.gate.acquire_head().await;
            if let Some(cause) = self.rails.halted() {
                drop(permit);
                return Err(SendError::Halted(cause));
            }
            match self.check(route) {
                Ok(()) => return Ok(permit),
                Err(wait) => {
                    drop(permit);
                    self.sleep(wait.clamp(Duration::from_millis(50), HALT_POLL))
                        .await;
                }
            }
        }
    }

    pub fn policy_statuses(&self) -> Vec<PolicyStatus> {
        self.limiter.lock().unwrap().statuses(self.now())
    }

    /// One endpoint's quote-relevant state under a single limiter lock —
    /// see `Limiter::project`.
    pub fn project(&self, route: &str, ahead: u32) -> ScopeProjection {
        self.limiter
            .lock()
            .unwrap()
            .project(route, ahead, self.now())
    }

    /// Requests holding N4's live send gate and its global bound.
    pub fn actual_send_occupancy(&self) -> (usize, usize) {
        self.gate.occupancy()
    }

    pub fn policyless_endpoints(&self) -> Vec<String> {
        self.limiter.lock().unwrap().policyless_endpoints()
    }

    pub fn is_live(&self) -> bool {
        self.limiter.lock().unwrap().is_live(self.now())
    }

    pub fn endpoint_state(&self, route: &str) -> EndpointState {
        self.limiter
            .lock()
            .unwrap()
            .endpoint_state(route, self.now())
    }

    pub fn degraded_endpoints(&self) -> Vec<DegradedEndpoint> {
        self.limiter.lock().unwrap().degraded_endpoints(self.now())
    }

    pub fn forget_degraded(&self) {
        self.limiter.lock().unwrap().forget_degraded();
    }

    /// Close an endpoint without a probe round-trip (e.g. no session to
    /// probe with). Same cooldown as a failed probe.
    pub fn degrade(&self, route: &str, reason: &str) {
        self.limiter.lock().unwrap().observe_probe(
            route,
            Err(reason.to_string()),
            serde_json::Value::Null,
            self.now(),
        );
    }

    /// The HEAD probe (N16). Not counted by the server (N24); teaches the
    /// limiter the endpoint's policy, or degrades the endpoint (N20). A
    /// block (C129) does neither: it sets the mark and fails the probe
    /// with the halt's words, leaving the route unknown; a blocked 429
    /// installs only the route's retry hold.
    /// Returns the raw rate headers on success for the probe job's payload;
    /// a failure carries the status the probe landed with, if it landed,
    /// so the caller can tell a rejected bearer (401, C87) from a closed
    /// route without reading the reason.
    pub async fn head(
        &self,
        route: &str,
        url: &str,
        bearer: Option<&dyn Bearer>,
        since: Instant,
    ) -> Result<(reqwest::StatusCode, Policy, serde_json::Value), ProbeError> {
        if bearer.is_some()
            && let Err(reason) = self.admit_bearer_url(url)
        {
            // Closed like a probe with no session to probe with, so the
            // job waiting on this route does not queue another at once.
            self.degrade(route, &reason);
            return Err(ProbeError::OffOrigin(reason));
        }
        let _permit = self
            .acquire_head(route)
            .await
            .map_err(|error| ProbeError::Failed {
                status: None,
                reason: error.to_string(),
            })?;
        // Read after admission (C87): the exclusive permit may have waited
        // behind sends that carried this token to a 401.
        let token = match bearer {
            Some(bearer) => match bearer.token() {
                Some(token) => Some(token),
                None => return Err(ProbeError::StaleBearer),
            },
            None => None,
        };
        let wait = self.now().saturating_duration_since(since);
        let mut req = self.http.head(url);
        if let Some(token) = &token {
            req = req.bearer_auth(token);
        }
        let sent = req.send().await.map_err(|error| error.to_string());
        let (policy, retry_after, raw, headers) = sent.as_ref().map_or_else(
            |_| {
                (
                    None,
                    RetryAfter::Missing,
                    serde_json::Value::Null,
                    serde_json::Value::Null,
                )
            },
            |response| {
                (
                    Some(Policy::from_headers(response.headers())),
                    retry_after_from_headers(response.headers()),
                    rate_limit_snapshot(response.headers()),
                    if response.status().is_success() {
                        serde_json::Value::Null
                    } else {
                        response_headers_snapshot(response.headers())
                    },
                )
            },
        );
        // A HEAD normally has no body, but draining it keeps the exclusive
        // permit live through the complete exchange even for a malformed
        // server response that does carry one.
        let completed = match sent {
            Ok(response) => {
                let status = response.status();
                let body = response.text().await.map_err(|error| error.to_string());
                Ok((status, body))
            }
            Err(error) => Err(error),
        };
        let outcome: Result<Policy, String> = match &completed {
            Ok((status, _)) if status.as_u16() == 429 => policy
                .clone()
                .expect("landed responses have a policy parse result")
                .map_err(|error| error.to_string()),
            Ok((status, Ok(_))) if status.is_success() => policy
                .clone()
                .expect("landed responses have a policy parse result")
                .map_err(|error| error.to_string()),
            Ok((status, Err(error))) if status.is_success() => {
                Err(format!("HEAD body transfer failed: {error}"))
            }
            // A HEAD has no body: the headers are the whole of the evidence.
            Ok((status, _)) => Err(format!(
                "HEAD returned {status}; response headers {headers}"
            )),
            Err(error) => Err(format!("HEAD failed: {error}")),
        };
        // C129: a HEAD has no body, so no page marker counts here; only
        // `cf-mitigated`, or a 429 with no policy header, makes a block.
        // A 403 without `cf-mitigated` (`/profile`'s HEAD, N38) is no
        // block and degrades the route as any failed probe does.
        let verdict = completed.as_ref().ok().and_then(|(status, _)| {
            (!status.is_success()).then(|| Verdict::of(status.as_u16(), &headers, None))
        });
        let blocked = verdict.is_some_and(|verdict| verdict.block);
        let now = self.now();
        if blocked {
            // No probe failure: the route learns no policy and is not
            // degraded from an answer that was not GGG's, so the jobs
            // waiting on it stay waiting under the mark, and after
            // `reset-tripwire` the route is probed afresh. A blocked 429
            // still holds the route for its `Retry-After`, as a blocked
            // GET's does.
            if completed
                .as_ref()
                .is_ok_and(|(status, _)| status.as_u16() == 429)
            {
                self.limiter
                    .lock()
                    .unwrap()
                    .observe_blocked_head_429(route, &retry_after, now);
            }
        } else if completed
            .as_ref()
            .is_ok_and(|(status, _)| status.as_u16() == 429)
        {
            let _ = self.limiter.lock().unwrap().observe_probe_429(
                route,
                policy.expect("a 429 is a landed response"),
                &retry_after,
                raw.clone(),
                now,
            );
        } else {
            self.limiter
                .lock()
                .unwrap()
                .observe_probe(route, outcome.clone(), raw.clone(), now);
        }
        let protocol_failure = match (&completed, &outcome) {
            (Ok((status, Ok(_))), Err(error)) if status.is_success() => Some(error.as_str()),
            _ => None,
        };
        self.record_completed(
            route,
            "HEAD",
            url,
            &completed,
            protocol_failure,
            &raw,
            &headers,
            verdict,
            false,
            wait,
        );
        let status = completed.as_ref().ok().map(|(status, _)| status.as_u16());
        if blocked {
            // The mark is set (`record_completed` above); a block is no
            // rejection of the bearer either.
            return Err(ProbeError::Failed {
                status,
                reason: self
                    .rails
                    .halted()
                    .unwrap_or_else(|| "Cloudflare answered in GGG's place".to_string()),
            });
        }
        // A rejection is published while this permit is still held (C87),
        // so no waiter admitted after it can read the rejected token.
        if status == Some(401)
            && let (Some(bearer), Some(token)) = (bearer, &token)
        {
            bearer.rejected(token);
        }
        // The limiter decided whether that was good enough; report what it
        // concluded so the probe job's outcome matches the endpoint state.
        match self
            .limiter
            .lock()
            .unwrap()
            .endpoint_state(route, self.now())
        {
            EndpointState::Policy(_) => {
                let Ok(policy) = outcome else {
                    unreachable!("policy state implies a parsed policy")
                };
                Ok((completed.unwrap().0, policy, raw))
            }
            EndpointState::Degraded { reason, .. } => Err(ProbeError::Failed {
                status,
                reason: format!("{reason}; endpoint closed for a cooldown"),
            }),
            other => Err(ProbeError::Failed {
                status,
                reason: format!("unexpected endpoint state after probe: {other:?}"),
            }),
        }
    }

    /// Recent sends, newest first.
    pub fn recent_sends(&self) -> Vec<SendRecord> {
        let sends = self.sends.lock().unwrap();
        sends
            .iter()
            .rev()
            .map(|s| SendRecord {
                seconds_ago: self.now().saturating_duration_since(s.at).as_secs_f64(),
                endpoint: s.endpoint.clone(),
                method: s.method.to_string(),
                url: s.url.clone(),
                outcome: s.outcome.clone(),
                ok: s.ok,
            })
            .collect()
    }

    /// Every response comes through here: the limiter learns from it and
    /// the send log records it.
    fn observe(
        &self,
        endpoint: &str,
        result: &Result<reqwest::Response, String>,
        retry_after: &RetryAfter,
        counted: bool,
    ) -> Result<(), PolicyObservationError> {
        if let Ok(r) = result {
            let policy = Policy::from_headers(r.headers());
            let raw = rate_limit_snapshot(r.headers());
            let mut limiter = self.limiter.lock().unwrap();
            let observation = limiter.observe_landed(
                endpoint,
                LandedObservation {
                    policy,
                    retry_after,
                    raw,
                    counted,
                    status: r.status().as_u16(),
                    now: self.now(),
                },
            );
            if let EndpointState::Policy(policy) = limiter.endpoint_state(endpoint, self.now()) {
                // Header discovery happens before body completion. Rekeying
                // the live permit and queued waiters atomically prevents the
                // stable route key and learned policy name from overlapping.
                self.gate.rekey_policy(endpoint, &policy);
            }
            return observation;
        }
        Ok(())
    }

    /// Observation is independent of status classification. Full matching
    /// headers update pacing on every landed response, while a malformed or
    /// mismatched observation becomes a protocol failure only when the HTTP
    /// response would otherwise be a clean success (D8/R6-3).
    async fn finish_send(
        &self,
        endpoint: &str,
        method: &'static str,
        url: &str,
        result: Result<reqwest::Response, String>,
        counted: bool,
        wait: Duration,
    ) -> Result<CompletedResponse, SendError> {
        let retry_after = result.as_ref().map_or(RetryAfter::Missing, |response| {
            retry_after_from_headers(response.headers())
        });
        let observation = self.observe(endpoint, &result, &retry_after, counted);
        if counted && result.is_err() {
            // L0 rail 3: a request that failed in transport may still have
            // been counted server-side (a timeout after the server answered
            // is the obvious case). Pace the next send as if it was.
            self.limiter
                .lock()
                .unwrap()
                .note_lost_send(endpoint, self.now());
        }
        let status = result
            .as_ref()
            .ok()
            .map(|response| response.status().as_u16());
        let rate = result
            .as_ref()
            .map(|response| rate_limit_snapshot(response.headers()))
            .unwrap_or(serde_json::Value::Null);
        let headers = result
            .as_ref()
            .ok()
            .filter(|response| !response.status().is_success())
            .map(|response| response_headers_snapshot(response.headers()))
            .unwrap_or(serde_json::Value::Null);
        let result = match result {
            Ok(response) => {
                let status = response.status();
                // C130: a 3xx's body echoes its target, query and all, and
                // every caller turns a non-2xx body into failure or refusal
                // text. It is drained, so the permit spans the whole
                // exchange, and replaced here, once, by where it pointed.
                let redirect = status
                    .is_redirection()
                    .then(|| redirect_evidence(response.headers()));
                let body = response.text().await.map_err(|error| error.to_string());
                Ok((status, body.map(|body| redirect.unwrap_or(body))))
            }
            Err(error) => Err(error),
        };
        // C129: decided once, here, from the status, the headers and the
        // whole body; the journal, the mark and the caller all read this.
        let verdict = verdict_of(&result, &headers);
        let classification = classify_response(
            status,
            match &result {
                Ok((_, body)) => body.is_err(),
                Err(_) => true,
            },
            observation,
            &retry_after,
            verdict.is_some_and(|verdict| verdict.block),
        );
        let protocol_failure = match &classification {
            ResponseClassification::Protocol(error) => Some(error.to_string()),
            _ => None,
        };
        self.record_completed(
            endpoint,
            method,
            url,
            &result,
            protocol_failure.as_deref(),
            &rate,
            &headers,
            verdict,
            counted,
            wait,
        );
        match (classification, result) {
            (ResponseClassification::Success, Ok((status, Ok(body))))
            | (ResponseClassification::Blocked, Ok((status, Ok(body))))
            | (ResponseClassification::RateLimited(_), Ok((status, Ok(body))))
            | (ResponseClassification::Http(_), Ok((status, Ok(body)))) => Ok(CompletedResponse {
                status,
                rate,
                retry_after,
                body: Ok(body),
                verdict,
            }),
            (ResponseClassification::Blocked, Ok((status, Err(error))))
            | (ResponseClassification::RateLimited(_), Ok((status, Err(error))))
            | (ResponseClassification::Http(_), Ok((status, Err(error)))) => {
                Ok(CompletedResponse {
                    status,
                    rate,
                    retry_after,
                    body: Err(error),
                    verdict,
                })
            }
            (ResponseClassification::Protocol(error), _) => Err(SendError::Protocol(error)),
            (ResponseClassification::Network, Err(error)) => Err(SendError::Transport(error)),
            (ResponseClassification::Network, Ok((_, Err(error)))) => {
                Err(SendError::Transport(error))
            }
            (classification, result) => {
                unreachable!(
                    "classification {classification:?} disagrees with transport {result:?}"
                )
            }
        }
    }

    /// Every completed exchange — landed or not — passes through here: the
    /// dashboard ring and the rails (journal, ceiling, tripwire).
    #[allow(clippy::too_many_arguments)]
    fn record_completed(
        &self,
        endpoint: &str,
        method: &'static str,
        url: &str,
        result: &Result<(reqwest::StatusCode, Result<String, String>), String>,
        protocol_failure: Option<&str>,
        rate: &serde_json::Value,
        headers: &serde_json::Value,
        verdict: Option<Verdict>,
        counted: bool,
        wait: Duration,
    ) {
        let (outcome, ok) = match (result, protocol_failure) {
            (_, Some(error)) => (format!("protocol failure: {error}"), false),
            (Ok((status, Err(error))), None) => {
                (format!("{status}; body transfer failure: {error}"), false)
            }
            (Ok((status, Ok(_))), None) => (status.to_string(), status.is_success()),
            (Err(error), None) => (format!("error: {error}"), false),
        };
        let path = url_path(url);
        let transport_error = result.as_ref().err().map(String::as_str);
        let status = result.as_ref().ok().map(|(status, _)| status.as_u16());
        // Recorded while the caller still holds the send's permit: a
        // block's mark is set before any send admitted after it can pass
        // the halt check (C129).
        self.rails.record(&SendReport {
            method,
            route: endpoint,
            url_path: &path,
            status,
            error: transport_error.or(protocol_failure),
            ok,
            counted,
            rate,
            headers,
            shape: verdict.map(|verdict| verdict.shape),
            block: verdict.is_some_and(|verdict| verdict.block),
            unreachable: verdict.is_some_and(|verdict| verdict.unreachable),
            wait,
        });
        let mut sends = self.sends.lock().unwrap();
        if sends.len() >= SEND_HISTORY {
            sends.pop_front();
        }
        sends.push_back(SentAt {
            at: self.now(),
            endpoint: endpoint.to_string(),
            method,
            url: url.to_string(),
            outcome,
            ok,
        });
    }

    /// The one way to send a form POST: waits for the limiter, then sends.
    /// The limiter lock is never held across an await.
    pub async fn post_form(
        &self,
        route: &str,
        url: &str,
        params: &[(&str, &str)],
        since: Instant,
    ) -> Result<CompletedResponse, SendError> {
        let _permit = self.acquire_send(route).await?;
        let wait = self.now().saturating_duration_since(since);
        let result = self
            .http
            .post(url)
            .form(params)
            .send()
            .await
            .map_err(|e| e.to_string());
        self.finish_send(route, "POST", url, result, true, wait)
            .await
    }

    /// Bearer-authenticated GET. The dispatcher may pre-wait while the job is
    /// cancellable; this method owns the final limiter check and live permit.
    /// The token is read from `bearer` after admission and a 401 is
    /// reported to it before the permit is released (C87).
    pub async fn get_bearer(
        &self,
        route: &str,
        url: &str,
        bearer: &dyn Bearer,
        since: Instant,
    ) -> Result<CompletedResponse, SendError> {
        self.admit_bearer_url(url).map_err(SendError::OffOrigin)?;
        let _permit = self.acquire_send(route).await?;
        let Some(token) = bearer.token() else {
            return Err(SendError::StaleBearer);
        };
        let wait = self.now().saturating_duration_since(since);
        let result = self
            .http
            .get(url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| e.to_string());
        let response = self
            .finish_send(route, "GET", url, result, true, wait)
            .await;
        if let Ok(completed) = &response
            && completed.status.as_u16() == 401
        {
            bearer.rejected(&token);
        }
        response
    }

    /// Unauthenticated GET (mock-only fake data endpoints).
    /// `since` is when the send became ready (see `SendReport::wait`).
    pub async fn get(
        &self,
        route: &str,
        url: &str,
        since: Instant,
    ) -> Result<CompletedResponse, SendError> {
        let _permit = self.acquire_send(route).await?;
        let wait = self.now().saturating_duration_since(since);
        let result = self.http.get(url).send().await.map_err(|e| e.to_string());
        self.finish_send(route, "GET", url, result, true, wait)
            .await
    }
}

/// C129: the verdict on a landed non-2xx, from its status, its headers (the
/// `response_headers_snapshot`) and its whole body; `None` for a 2xx or a
/// send that never landed.
fn verdict_of(
    result: &Result<(reqwest::StatusCode, Result<String, String>), String>,
    headers: &serde_json::Value,
) -> Option<Verdict> {
    let (status, body) = result.as_ref().ok()?;
    (!status.is_success()).then(|| Verdict::of(status.as_u16(), headers, body.as_deref().ok()))
}

/// A URL's origin in words for a refusal: its serialized origin, or the
/// fact that it does not parse as one.
fn url_origin_words(url: &str) -> String {
    match url::Url::parse(url) {
        Ok(url) => url.origin().ascii_serialization(),
        Err(_) => "a URL that does not parse".into(),
    }
}
