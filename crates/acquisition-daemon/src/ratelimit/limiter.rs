//! The limiter: the timing buckets, what it knows of each endpoint and
//! policy, and the deadline arithmetic that turns headers and history into
//! a wait.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.
//!
//! # Rulings held here
//!
//! - **C18** (state keyed by policy name, from headers): for the observed
//!   paired API-policy windows, classification is positional (first window
//!   initial/5s, later windows sustained/60s — ground-truth Q4
//!   hypothesis). That hypothesis does not classify N33's single-window
//!   `token-request-limit`, which takes the conservative 60s bucket until
//!   GGG confirms its hidden resolution through N14's support channel.
//! - **C19** (a headerless counted send is paced as counted): one
//!   predicted hit in every window as of that response (`assume_counted`,
//!   which moves `last_response` to it); the policy's definition is
//!   untouched; the next response with headers replaces the prediction
//!   (invariant 2). A
//!   missing, malformed or negative `Retry-After` counts as zero and an
//!   over-cap one as the cap (`install_retry_hold`). A probe's 429 is held
//!   the same way whatever its policy headers carried, beside the setup
//!   failure's degrade (`observe_probe_429`).

use super::*;

/// Server-side timing bucket for a rule's first (initial) window (N12).
pub const INITIAL_BUCKET: Duration = Duration::from_secs(5);
/// Server-side timing bucket for a rule's later (sustained) windows (N12).
pub const SUSTAINED_BUCKET: Duration = Duration::from_secs(60);
/// Extra margin on top of the bucket (N13 says the full bucket is the safe
/// margin; the shipped client adds one more second and has been clean).
pub const BUFFER: Duration = Duration::from_secs(1);
/// A retry always pays the largest timing bucket GGG has named, independent
/// of whether the response's policy headers are usable (D3/N19).
pub const RETRY_BUCKET_PAD: Duration = Duration::from_secs(60);
/// The OAuth token endpoint's route: never probed, accountless (N33).
pub const TOKEN_ROUTE: &str = "oauth-token";

/// Which timing bucket applies to the `index`-th window of a rule.
/// Positional classification (Q4, Tom's hypothesis): the first window is
/// the initial limit, every later one is sustained. A single-window rule is
/// treated as initial. Conservative on every observed policy shape (N23).
pub fn bucket_for(index: usize) -> Duration {
    if index == 0 {
        INITIAL_BUCKET
    } else {
        SUSTAINED_BUCKET
    }
}

/// N33's token policy has one window and therefore cannot be classified by
/// the paired API-policy positional rule. Until GGG confirms its hidden
/// resolution, the frozen policy uses the conservative 60-second bucket.
fn bucket_for_policy(policy: &str, window_count: usize, index: usize) -> Duration {
    if policy == "token-request-limit" && window_count == 1 {
        SUSTAINED_BUCKET
    } else {
        bucket_for(index)
    }
}

// ---- the limiter ----------------------------------------------------------

const HISTORY_CAP: usize = 256;
/// How long an endpoint whose probe came back degraded (N20) stays closed
/// before another probe is tried. Login clears it early.
pub const PROBE_COOLDOWN: Duration = Duration::from_secs(60);

/// What the limiter knows about one endpoint (URL path).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndpointState {
    /// Never heard from: needs a probe before the first real send.
    Unknown,
    /// Governed by the named policy.
    Policy(String),
    /// Answered 2xx with no rate-limit headers at all, on a route declared
    /// policyless (`Limiter::declare_policyless`; per-route knowledge, as
    /// `GET /profile` was observed on 2026-08-30). Paced by nothing but the
    /// send gate. Strict observation never creates it for any other route.
    Policyless,
    /// The probe failed or came back without a policy (N20); closed until
    /// `until`, then probed again.
    Degraded { until: Instant, reason: String },
}

/// The limiter's endpoint key: the route, plus the account the send is for
/// when there is one — `stash@Alice#1234`. The limiter never interprets the
/// account; it only decides whether a policy's *state* is per account, and
/// that comes from the policy's own `X-Rate-Limit-Rules` (rung 11:
/// `Account` rules count per account on GGG's side; `Ip` rules per IP).
pub fn endpoint_key(route: &str, account: Option<&str>) -> String {
    match account {
        Some(account) => format!("{route}@{account}"),
        None => route.to_string(),
    }
}

/// `(route, account)` of an endpoint key. Routes never contain `@`.
pub fn split_endpoint_key(key: &str) -> (&str, Option<&str>) {
    match key.split_once('@') {
        Some((route, account)) => (route, Some(account)),
        None => (key, None),
    }
}

/// The key a policy's state lives under: per account when every rule of
/// the policy is `Account`-scoped and the send was for an account; the
/// bare policy name otherwise (`Ip` rules, or a policy mixing scopes —
/// shared state over-waits at worst, never floods).
fn policy_state_key(policy: &Policy, account: Option<&str>) -> String {
    let all_account = !policy.rules.is_empty() && policy.rules.iter().all(|r| r.name == "Account");
    match account {
        Some(account) if all_account => format!("{}@{account}", policy.name),
        _ => policy.name.clone(),
    }
}

/// The policy name in a state key (`stash-request-limit@Alice#1234` →
/// `stash-request-limit`).
fn policy_name_of(state_key: &str) -> &str {
    split_endpoint_key(state_key).0
}

/// What the limiter remembers about one named policy.
pub(super) struct PolicyState {
    pub(super) policy: Policy,
    /// Arrival times of counted responses under this policy, oldest first.
    /// Shared by every endpoint that reports the same policy name (N6).
    pub(super) history: VecDeque<Instant>,
    /// When the Full policy was observed, or when `assume_counted` advanced
    /// the prediction by an assumed hit; the base for restriction state, and
    /// what the status reports as `last_observed_seconds_ago`.
    last_response: Instant,
    /// The raw headers, for the dashboard.
    pub(super) raw: serde_json::Value,
    /// A 429 hold is independent of policy validity. It is installed from
    /// every landed 429 (C19) even when the landed policy set is malformed.
    retry_hold_until: Option<Instant>,
    /// The last 429's `Retry-After` as the header carried it (none when it
    /// carried no count), for the status; the hold above is the arithmetic.
    last_retry_after_secs: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyObservationError {
    Parse(PolicyParseError),
    PolicyMismatch {
        established: String,
        observed: String,
    },
}

impl fmt::Display for PolicyObservationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PolicyObservationError::Parse(error) => write!(f, "{error}"),
            PolicyObservationError::PolicyMismatch {
                established,
                observed,
            } => {
                // #113: the names are GGG's text; the words say what
                // happened, and the journal holds the names.
                let _ = (established, observed);
                f.write_str(
                    "endpoint is established under one policy but the response reported another",
                )
            }
        }
    }
}

impl std::error::Error for PolicyObservationError {}

pub(super) struct LandedObservation<'a> {
    pub(super) policy: Result<Policy, PolicyParseError>,
    pub(super) retry_after: &'a RetryAfter,
    pub(super) raw: serde_json::Value,
    pub(super) counted: bool,
    pub(super) status: u16,
    pub(super) now: Instant,
}

#[derive(Default)]
pub struct Limiter {
    pub(super) policies: HashMap<String, PolicyState>,
    /// Endpoint key (URL path) → what we know, learned from probes and
    /// responses. Absent means `Unknown`.
    endpoints: HashMap<String, EndpointState>,
    /// Every landed 429, including setup failures and stopped/malformed
    /// responses. Bookkeeping happens before caller classification.
    violations: u64,
    /// Route-local fallback holds. A 429 holds its route (C19) while its
    /// policy headers are malformed or the route is not established yet;
    /// that must still prevent the requeued job from sending early.
    retry_holds: HashMap<String, (Instant, u64)>,
    /// Bare routes that may answer without any rate-limit header. Every
    /// other route's headerless 2xx stays a protocol failure.
    policyless_routes: HashSet<String>,
}

impl Limiter {
    pub fn new() -> Self {
        Limiter::default()
    }

    /// Declare that `route` (bare, no account) may answer a 2xx with no
    /// `X-Rate-Limit-*` headers. Such an answer makes the endpoint
    /// `Policyless` instead of a protocol failure; headers, if they ever
    /// appear, are learned strictly as on any other route.
    pub fn declare_policyless(&mut self, route: &str) {
        self.policyless_routes.insert(route.to_string());
    }

    /// How long to wait before sending to `endpoint`. An unknown endpoint is
    /// normally ready, except when a 429 installed its route-local hold.
    pub fn wait_for(&self, endpoint: &str, now: Instant) -> Duration {
        let policy = self.policy_for(endpoint).and_then(next_safe_send);
        let route = self.retry_holds.get(endpoint).map(|(until, _)| *until);
        policy
            .into_iter()
            .chain(route)
            .max()
            .map(|t| t.saturating_duration_since(now))
            .unwrap_or(Duration::ZERO)
    }

    /// Predicted wait for a request with `ahead` same-policy requests queued
    /// before it. Simulates the pacing rule forward: each simulated send is
    /// appended to a copy of the history, and window hit counts are taken
    /// from that history (a prediction of what the server will report —
    /// headers remain the truth for real sends). An estimate, not a promise.
    pub fn eta_for(&self, endpoint: &str, ahead: u32, now: Instant) -> Duration {
        self.simulate(endpoint, ahead as usize, now, true, |_| {})
    }

    /// `eta_for` for every queue position at once: element `k` is
    /// `eta_for(endpoint, k, now)`, for `k` in `0..count`. The simulation
    /// for `ahead = k` is the prefix of the one for any larger `ahead`, so
    /// one pass yields them all: a queue of `n` same-route jobs costs one
    /// forward simulation, not `n` of them (the daemon's `List`).
    pub fn etas_for(&self, endpoint: &str, count: usize, now: Instant) -> Vec<Duration> {
        let mut etas = Vec::with_capacity(count);
        if let Some(last) = count.checked_sub(1) {
            self.simulate(endpoint, last, now, true, |eta| etas.push(eta));
        }
        etas
    }

    /// `etas_for` with every simulated send kept to the end: the plain
    /// simulation, the oracle `forget_stale` is held to.
    #[cfg(test)]
    pub(crate) fn etas_for_keeping_every_send(
        &self,
        endpoint: &str,
        count: usize,
        now: Instant,
    ) -> Vec<Duration> {
        let mut etas = Vec::with_capacity(count);
        if let Some(last) = count.checked_sub(1) {
            self.simulate(endpoint, last, now, false, |eta| etas.push(eta));
        }
        etas
    }

    /// The forward simulation behind `eta_for` and `etas_for`: `each` is
    /// called with the ETA at every position `0..=ahead`, in order, and
    /// the last is returned. With `forget_stale`, a send older than the
    /// policy's longest window is dropped from the simulated history:
    /// the simulated instant never moves back, so every window count
    /// and every deadline would skip it from then on, and keeping it
    /// made a long queue's one simulation quadratic in its length.
    fn simulate(
        &self,
        endpoint: &str,
        ahead: usize,
        now: Instant,
        forget_stale: bool,
        mut each: impl FnMut(Duration),
    ) -> Duration {
        let Some(state) = self.policy_for(endpoint) else {
            for _ in 0..=ahead {
                each(Duration::ZERO);
            }
            return Duration::ZERO;
        };
        let mut history = state.history.clone();
        // The server's reported counts are the truth (invariant 2) and the
        // local arrival history can undercount them: a restart probe reads
        // hits this daemon never sent (N24), and other tools share the
        // account's windows (N23). The missing hits enter the simulation
        // as a *count* pinned at the last response time — the latest they
        // could have happened, `window_frees_at`'s own convention — so the
        // forward estimate over-waits instead of predicting sends into a
        // window the server already counts as fuller. A count, never
        // materialized entries: reported hits are an arbitrary `u32` from
        // headers, so materializing would loop unboundedly and overflow
        // `HISTORY_CAP` (truncating a 299-of-300 state into "free").
        let seed_at = state.last_response;
        let seed_n = state
            .policy
            .rules
            .iter()
            .flat_map(|rule| {
                rule.limits.iter().enumerate().map(|(i, w)| {
                    let period = Duration::from_secs(w.period_secs);
                    let known = history
                        .iter()
                        .filter(|&&h| seed_at.saturating_duration_since(h) < period)
                        .count() as u64;
                    let reported = rule.state.get(i).map_or(0, |s| u64::from(s.hits));
                    reported.saturating_sub(known)
                })
            })
            .max()
            .unwrap_or(0);
        let seeded_in = |t: Instant, period: Duration| {
            if t.saturating_duration_since(seed_at) < period {
                seed_n
            } else {
                0
            }
        };
        let longest = state
            .policy
            .rules
            .iter()
            .flat_map(|rule| rule.limits.iter())
            .map(|w| Duration::from_secs(w.period_secs))
            .max()
            .unwrap_or(Duration::ZERO);
        let mut t = now.checked_add(self.wait_for(endpoint, now)).unwrap_or(now);
        each(t.saturating_duration_since(now));
        for _ in 0..ahead {
            history.push_back(t);
            while forget_stale
                && history
                    .front()
                    .is_some_and(|&h| t.duration_since(h) >= longest)
            {
                history.pop_front();
            }
            let mut next = t;
            for rule in &state.policy.rules {
                for (i, w) in rule.limits.iter().enumerate() {
                    let period = Duration::from_secs(w.period_secs);
                    let bucket = bucket_for_policy(&state.policy.name, rule.limits.len(), i);
                    let in_window = history
                        .iter()
                        .filter(|&&h| t.duration_since(h) < period)
                        .count() as u64
                        + seeded_in(t, period);
                    if in_window >= u64::from(w.max_hits)
                        && let Some(deadline) = window_frees_at_seeded(
                            &history,
                            t,
                            w.max_hits,
                            period,
                            bucket,
                            seed_at,
                            seeded_in(t, period),
                        )
                    {
                        next = next.max(deadline);
                    }
                }
            }
            t = next;
            each(t.saturating_duration_since(now));
        }
        t.saturating_duration_since(now)
    }

    /// What the limiter knows about `endpoint` right now; an expired
    /// cooldown reads as `Unknown` (time to probe again).
    pub fn endpoint_state(&self, endpoint: &str, now: Instant) -> EndpointState {
        match self.endpoints.get(endpoint) {
            None => EndpointState::Unknown,
            Some(EndpointState::Degraded { until, .. }) if *until <= now => EndpointState::Unknown,
            Some(state) => state.clone(),
        }
    }

    /// Record a probe's outcome: a Full policy teaches the policy without
    /// counting a hit (N24); every partial/malformed set and transport/HTTP
    /// failure closes the endpoint for `PROBE_COOLDOWN` (N20/D8).
    pub fn observe_probe(
        &mut self,
        endpoint: &str,
        outcome: Result<Policy, String>,
        raw: serde_json::Value,
        now: Instant,
    ) {
        let reason = match outcome {
            Ok(policy) => match self.observe(endpoint, Ok(policy), raw, false, now) {
                Ok(()) => return,
                Err(error) => format!("probe observation failed: {error}"),
            },
            Err(e) => format!("probe failed: {e}"),
        };
        self.endpoints.insert(
            endpoint.to_string(),
            EndpointState::Degraded {
                until: now.checked_add(PROBE_COOLDOWN).unwrap_or(now),
                reason,
            },
        );
    }

    /// Frozen D4 behavior for a setup probe that lands as 429. Only a Full
    /// policy plus acceptable Retry-After establishes the endpoint; any
    /// other 429 is a setup failure that degrades it. The HEAD itself
    /// consumes no job retry, but it is a counted violation, and whatever
    /// its headers carried it installs the route's retry hold (C19), as a
    /// GET's 429 does: the next send on the route, probe or GET, waits its
    /// clamped `Retry-After` plus the retry bucket. The degrade is a
    /// separate effect, and a login that clears it leaves the hold.
    pub fn observe_probe_429(
        &mut self,
        endpoint: &str,
        observation: Result<Policy, PolicyParseError>,
        retry_after: &RetryAfter,
        raw: serde_json::Value,
        now: Instant,
    ) -> Result<(), String> {
        self.violations = self.violations.saturating_add(1);
        let outcome = self.setup_after_probe_429(endpoint, observation, retry_after, raw, now);
        // After the setup outcome, so an established route's policy state
        // carries the hold for its status as well.
        self.install_retry_hold(endpoint, retry_after, now);
        outcome
    }

    /// `observe_probe_429`'s setup outcome: establish the endpoint or
    /// degrade it. The hold is not this function's.
    fn setup_after_probe_429(
        &mut self,
        endpoint: &str,
        observation: Result<Policy, PolicyParseError>,
        retry_after: &RetryAfter,
        raw: serde_json::Value,
        now: Instant,
    ) -> Result<(), String> {
        match (observation, retry_after) {
            (Ok(policy), RetryAfter::Acceptable { .. }) => self
                .observe_impl(endpoint, Ok(policy), raw, true, now)
                .map_err(|error| error.to_string()),
            (Ok(_), retry_after) => {
                let reason = format!("HEAD returned 429 with {retry_after}");
                self.degrade_for(endpoint, reason.clone(), PROBE_COOLDOWN, now);
                Err(reason)
            }
            (Err(policy_error), retry_after) => {
                let cooldown = retry_after
                    .seconds()
                    .map(Duration::from_secs)
                    .unwrap_or(Duration::ZERO)
                    .max(PROBE_COOLDOWN);
                let reason = format!(
                    "HEAD returned 429 with non-Full policy headers ({policy_error}) and {retry_after}"
                );
                self.degrade_for(endpoint, reason.clone(), cooldown, now);
                Err(reason)
            }
        }
    }

    /// C129: a HEAD 429 that was a block (Cloudflare's, not GGG's) is no
    /// probe outcome: the route's probe and policy state are untouched and
    /// it is not degraded. It still counts as a violation and installs the
    /// route's retry hold (C19), as a blocked GET's 429 does through
    /// `observe_landed`, so a send after `reset-tripwire` inside Cloudflare's
    /// `Retry-After` is still held.
    pub fn observe_blocked_head_429(
        &mut self,
        endpoint: &str,
        retry_after: &RetryAfter,
        now: Instant,
    ) {
        self.violations = self.violations.saturating_add(1);
        self.install_retry_hold(endpoint, retry_after, now);
    }

    fn degrade_for(&mut self, endpoint: &str, reason: String, cooldown: Duration, now: Instant) {
        self.endpoints.insert(
            endpoint.to_string(),
            EndpointState::Degraded {
                until: now.checked_add(cooldown).unwrap_or(now),
                reason,
            },
        );
    }

    /// Forget every degraded endpoint (login changed what a probe would
    /// see, so don't make the user sit out the cooldown).
    pub fn forget_degraded(&mut self) {
        self.endpoints
            .retain(|_, st| !matches!(st, EndpointState::Degraded { .. }));
    }

    /// Record a response. `counted` is false for requests the server does
    /// not count against the policy (HEAD probes, N24). A 429 is recorded
    /// as counted — over-estimating the wait is the safe direction.
    pub fn observe(
        &mut self,
        endpoint: &str,
        observation: Result<Policy, PolicyParseError>,
        raw: serde_json::Value,
        counted: bool,
        now: Instant,
    ) -> Result<(), PolicyObservationError> {
        self.observe_impl(endpoint, observation, raw, counted, now)
    }

    /// Record and reconcile one landed ordinary response. Counted history
    /// and 429 violations are recorded before policy parsing/classification;
    /// every 429 then installs its hold (C19, whatever its Retry-After
    /// carried) regardless of whether reconciliation succeeds. A counted
    /// token 2xx with no rate-limit header at all, on a token route not yet
    /// established, holds the route one timing bucket (C131).
    pub(super) fn observe_landed(
        &mut self,
        endpoint: &str,
        landed: LandedObservation<'_>,
    ) -> Result<(), PolicyObservationError> {
        if landed.status == 429 {
            self.violations = self.violations.saturating_add(1);
        }
        let headerless = landed
            .raw
            .as_object()
            .is_none_or(|raw| !raw.keys().any(|name| name.starts_with("x-rate-limit")));
        let token_floor = landed.counted
            && headerless
            && (200..300).contains(&landed.status)
            && endpoint == TOKEN_ROUTE
            && !matches!(self.endpoints.get(endpoint), Some(EndpointState::Policy(_)));
        let reconciled = self.observe_impl(
            endpoint,
            landed.policy,
            landed.raw,
            landed.counted,
            landed.now,
        );
        if landed.status == 429 {
            self.install_retry_hold(endpoint, landed.retry_after, landed.now);
        }
        // C131: reached only when reconciliation failed, so after the
        // policyless arm (`/profile` is never held). An established route
        // is paced as counted instead (C19), and a malformed but present
        // header set is held by nothing.
        if token_floor && reconciled.is_err() {
            self.install_token_floor_hold(endpoint, landed.now);
        }
        reconciled
    }

    /// C131: hold the token route `RETRY_BUCKET_PAD`, the one timing
    /// bucket, from a headerless answer that leaves the route with no
    /// policy to pace the next grant by.
    fn install_token_floor_hold(&mut self, endpoint: &str, now: Instant) {
        let Some(until) = checked_deadline(now, [RETRY_BUCKET_PAD]) else {
            return;
        };
        self.retry_holds
            .entry(endpoint.to_string())
            .and_modify(|(old_until, _)| *old_until = (*old_until).max(until))
            .or_insert((until, 0));
    }

    fn observe_impl(
        &mut self,
        endpoint: &str,
        observation: Result<Policy, PolicyParseError>,
        raw: serde_json::Value,
        counted: bool,
        now: Instant,
    ) -> Result<(), PolicyObservationError> {
        let established = match self.endpoints.get(endpoint) {
            Some(EndpointState::Policy(name)) => Some(name.clone()),
            _ => None,
        };

        // The exchange was counted even when its headers are malformed or
        // name a different policy. Attribute that fact to the policy under
        // which the request was admitted, before attempting reconciliation
        // (N25/D8).
        if counted
            && let Some(state) = established
                .as_ref()
                .and_then(|name| self.policies.get_mut(name))
        {
            push_history(&mut state.history, now);
        }

        let policy = match observation {
            Ok(policy) => policy,
            // A declared-policyless route answering with no rate-limit
            // header at all (not a partial or malformed set) is what it
            // was declared to be. An established policy vanishing from a
            // route is still a protocol failure.
            Err(PolicyParseError::MissingHeader { ref name })
                if name == "x-rate-limit-policy"
                    && established.is_none()
                    && raw.as_object().is_some_and(|m| m.is_empty())
                    && self
                        .policyless_routes
                        .contains(split_endpoint_key(endpoint).0) =>
            {
                self.endpoints
                    .insert(endpoint.to_string(), EndpointState::Policyless);
                return Ok(());
            }
            Err(error) => {
                // A counted send whose response carried no usable policy
                // headers (rung 10's origin 503, 2026-08-24: an HTML error
                // page with no `X-Rate-Limit-*`) is the same situation as a
                // send lost in transport: we sent, the server may have
                // counted, and nothing tells us otherwise. Advance the
                // prediction the same way, or the next send is paced from
                // a state that predates this one and goes out immediately.
                if counted
                    && let Some(state) = established
                        .as_ref()
                        .and_then(|name| self.policies.get_mut(name))
                {
                    assume_counted(state, now);
                }
                return Err(PolicyObservationError::Parse(error));
            }
        };
        if let Some(name) = &established
            && policy_name_of(name) != policy.name
        {
            return Err(PolicyObservationError::PolicyMismatch {
                established: policy_name_of(name).to_string(),
                observed: policy.name,
            });
        }

        let state_key = policy_state_key(&policy, split_endpoint_key(endpoint).1);
        self.endpoints.insert(
            endpoint.to_string(),
            EndpointState::Policy(state_key.clone()),
        );
        let state = self
            .policies
            .entry(state_key)
            .or_insert_with(|| PolicyState {
                policy: policy.clone(),
                history: VecDeque::new(),
                last_response: now,
                raw: serde_json::Value::Null,
                retry_hold_until: None,
                last_retry_after_secs: None,
            });

        // A same-name shape transition describes a different counter set
        // (F65): discard incompatible history. Ordinary dynamic limit/state
        // changes within the established shape retain it (N9).
        if !state.policy.has_same_shape(&policy) {
            state.history.clear();
        } else if counted && established.is_none() {
            push_history(&mut state.history, now);
        }
        state.policy = policy;
        state.last_response = now;
        state.raw = raw;
        Ok(())
    }

    /// C19: a landed 429 holds its route for its `Retry-After`, clamped to
    /// the cap, plus the retry bucket and the buffer, whatever the header
    /// carried. A missing, malformed or negative value counts as zero (the
    /// hold is the bucket alone); an over-cap one counts as the cap.
    fn install_retry_hold(&mut self, endpoint: &str, retry_after: &RetryAfter, now: Instant) {
        let seconds = match retry_after {
            RetryAfter::Acceptable { seconds } => (*seconds).min(RETRY_AFTER_CAP_SECS),
            RetryAfter::OverCap { .. } => RETRY_AFTER_CAP_SECS,
            RetryAfter::Missing | RetryAfter::Malformed { .. } | RetryAfter::Negative { .. } => 0,
        };
        let Some(until) = checked_deadline(
            now,
            [Duration::from_secs(seconds), RETRY_BUCKET_PAD, BUFFER],
        ) else {
            return;
        };
        self.retry_holds
            .entry(endpoint.to_string())
            .and_modify(|(old_until, old_seconds)| {
                if until > *old_until {
                    *old_until = until;
                    *old_seconds = seconds;
                }
            })
            .or_insert((until, seconds));
        let Some(name) = self.endpoints.get(endpoint).and_then(|state| match state {
            EndpointState::Policy(name) => Some(name.clone()),
            _ => None,
        }) else {
            return;
        };
        let Some(state) = self.policies.get_mut(&name) else {
            return;
        };
        state.retry_hold_until = Some(state.retry_hold_until.map_or(until, |old| old.max(until)));
        // The status reports what the header carried, never the clamped
        // `seconds` above: a missing value is not "0", an over-cap one is
        // not the cap (invariant 2).
        state.last_retry_after_secs = retry_after.carried_secs();
    }

    pub fn violation_count(&self) -> u64 {
        self.violations
    }

    /// A counted send whose response never arrived: assume the server
    /// counted it (the safe direction) by appending one history hit under
    /// the endpoint's established policy. No-op without a policy.
    ///
    /// Pacing reads the last reported window state (invariant 2: local state
    /// is a prediction, headers correct it), so the prediction is advanced
    /// here: one more hit in every window of every rule, and one history
    /// point, as of `now`. The next landed response replaces it.
    pub fn note_lost_send(&mut self, endpoint: &str, now: Instant) {
        let established = match self.endpoints.get(endpoint) {
            Some(EndpointState::Policy(name)) => name.clone(),
            _ => return,
        };
        if let Some(state) = self.policies.get_mut(&established) {
            push_history(&mut state.history, now);
            assume_counted(state, now);
        }
    }

    fn policy_for(&self, endpoint: &str) -> Option<&PolicyState> {
        match self.endpoints.get(endpoint)? {
            EndpointState::Policy(name) => self.policies.get(name),
            _ => None,
        }
    }

    pub fn statuses(&self, now: Instant) -> Vec<PolicyStatus> {
        let mut out: Vec<PolicyStatus> = self
            .policies
            .iter()
            .map(|(name, s)| PolicyStatus {
                policy: name.clone(),
                endpoints: {
                    let mut e: Vec<String> = self
                        .endpoints
                        .iter()
                        .filter(|(_, st)| matches!(st, EndpointState::Policy(p) if p == name))
                        .map(|(k, _)| k.clone())
                        .collect();
                    e.sort();
                    e
                },
                rules: rule_statuses(s),
                next_safe_in_seconds: next_safe_send(s)
                    .map(|t| t.saturating_duration_since(now).as_secs_f64())
                    .unwrap_or(0.0),
                last_observed_seconds_ago: now
                    .saturating_duration_since(s.last_response)
                    .as_secs_f64(),
                history_len: s.history.len(),
                retry_after_secs: s
                    .retry_hold_until
                    .filter(|until| *until > now)
                    .and(s.last_retry_after_secs),
                headers: s.raw.clone(),
            })
            .collect();
        out.sort_by(|a, b| a.policy.cmp(&b.policy));
        out
    }

    /// Whether any policy still carries state worth keeping: a pending wait,
    /// or counted hits inside its longest window. The daemon declines to
    /// idle-exit while this is true, so a quick restart doesn't throw away
    /// the history that lets it wait less than a full period (N24).
    pub fn is_live(&self, now: Instant) -> bool {
        self.retry_holds.values().any(|(until, _)| *until > now)
            || self.policies.values().any(|s| {
                if next_safe_send(s).is_some_and(|t| t > now) {
                    return true;
                }
                let longest = s
                    .policy
                    .rules
                    .iter()
                    .flat_map(|r| r.limits.iter())
                    .map(|w| Duration::from_secs(w.period_secs))
                    .max()
                    .unwrap_or(Duration::ZERO);
                s.history
                    .back()
                    .is_some_and(|&h| now.saturating_duration_since(h) < longest)
            })
    }

    /// Everything a quote needs about one endpoint, computed in a single
    /// call so the rules, the observation age, and the ETA describe the
    /// same instant — separate reads could straddle a landing response.
    pub fn project(&self, endpoint: &str, ahead: u32, now: Instant) -> ScopeProjection {
        let state = self.endpoint_state(endpoint, now);
        let (rules, observed_seconds_ago, eta) = match &state {
            EndpointState::Policy(name) => {
                let (rules, observed) = self
                    .policies
                    .get(name)
                    .map(|ps| {
                        (
                            rule_statuses(ps),
                            now.saturating_duration_since(ps.last_response).as_secs(),
                        )
                    })
                    .unwrap_or_default();
                (
                    rules,
                    Some(observed),
                    Some(self.eta_for(endpoint, ahead, now)),
                )
            }
            _ => (Vec::new(), None, None),
        };
        ScopeProjection {
            state,
            rules,
            observed_seconds_ago,
            eta,
        }
    }

    /// Endpoints that have answered without any policy header.
    pub fn policyless_endpoints(&self) -> Vec<String> {
        let mut v: Vec<String> = self
            .endpoints
            .iter()
            .filter(|(_, st)| **st == EndpointState::Policyless)
            .map(|(k, _)| k.clone())
            .collect();
        v.sort();
        v
    }

    pub fn degraded_endpoints(&self, now: Instant) -> Vec<DegradedEndpoint> {
        let mut v: Vec<DegradedEndpoint> = self
            .endpoints
            .iter()
            .filter_map(|(k, st)| match st {
                EndpointState::Degraded { until, reason } if *until > now => {
                    Some(DegradedEndpoint {
                        endpoint: k.clone(),
                        seconds_left: until.saturating_duration_since(now).as_secs_f64(),
                        reason: reason.clone(),
                    })
                }
                _ => None,
            })
            .collect();
        v.sort_by(|a, b| a.endpoint.cmp(&b.endpoint));
        v
    }
}

/// Advance a policy's prediction by one hit the server may have counted but
/// never reported: one more hit in every window of every rule, as of `now`.
/// The next landed response with headers replaces it (invariant 2). Used
/// for a send lost in transport and for a landed response without usable
/// policy headers; the history point is the caller's.
fn assume_counted(state: &mut PolicyState, now: Instant) {
    for rule in &mut state.policy.rules {
        for window in &mut rule.state {
            window.hits = window.hits.saturating_add(1);
        }
    }
    state.last_response = now;
}

fn push_history(history: &mut VecDeque<Instant>, now: Instant) {
    if history.len() >= HISTORY_CAP {
        history.pop_front();
    }
    history.push_back(now);
}

/// When a saturated window frees up, given what we know. Only our hits that
/// are still inside the window (as of `as_of`, the moment the server
/// reported the count) are *known*; the rules are account-scoped (N23), so
/// other tools' hits can be in the server's count without being in our
/// history. Unknown hits are assumed to be the most recent — the window
/// can't free before its oldest hit ages out, and the oldest hit we can
/// name is the latest that oldest hit could possibly be. With no known
/// in-window hits, assume everything just happened.
fn checked_deadline(
    base: Instant,
    durations: impl IntoIterator<Item = Duration>,
) -> Option<Instant> {
    durations
        .into_iter()
        .try_fold(base, |deadline, duration| deadline.checked_add(duration))
}

fn window_frees_at(
    history: &VecDeque<Instant>,
    as_of: Instant,
    max_hits: u32,
    period: Duration,
    bucket: Duration,
) -> Option<Instant> {
    window_frees_at_seeded(history, as_of, max_hits, period, bucket, as_of, 0)
}

/// `window_frees_at` with `seed_n` extra hits pinned at `seed_at` — the
/// ETA simulation's compact stand-in for server-reported hits the local
/// history never saw. A count, not entries: reported hits are an
/// arbitrary `u32`, and the synthetic block is indexed into rather than
/// materialized, so a huge (or malformed) header can neither loop nor
/// overflow `HISTORY_CAP`.
fn window_frees_at_seeded(
    history: &VecDeque<Instant>,
    as_of: Instant,
    max_hits: u32,
    period: Duration,
    bucket: Duration,
    seed_at: Instant,
    seed_n: u64,
) -> Option<Instant> {
    let known: Vec<Instant> = history
        .iter()
        .copied()
        .filter(|&h| as_of.saturating_duration_since(h) < period)
        .collect();
    let seeded = if as_of.saturating_duration_since(seed_at) < period {
        seed_n
    } else {
        0
    };
    let total = known.len() as u64 + seeded;
    let oldest = match total.checked_sub(u64::from(max_hits)) {
        Some(idx) if total > 0 => {
            // The merged chronological list is known[..cut], then `seeded`
            // synthetic hits at `seed_at`, then known[cut..]; index into
            // it without building the synthetic block. Clamped so a
            // zero-max window cannot index past the end.
            let idx = idx.min(total - 1);
            let cut = known.iter().filter(|&&h| h <= seed_at).count() as u64;
            if idx < cut {
                known[idx as usize]
            } else if idx < cut + seeded {
                seed_at
            } else {
                known[(idx - seeded) as usize]
            }
        }
        _ => {
            let first = known.first().copied();
            let seed = (seeded > 0).then_some(seed_at);
            match (first, seed) {
                (Some(a), Some(b)) => a.min(b),
                (Some(a), None) => a,
                (None, Some(b)) => b,
                (None, None) => as_of,
            }
        }
    };
    checked_deadline(oldest, [period, bucket, BUFFER])
}

/// The earliest instant the next request under this policy may be sent,
/// or `None` if it may go now. The pacing rule, per window of each rule:
///
///
/// - restriction active (`restricted-for > 0`): last response, plus the
///   restriction, bucket and buffer;
///
/// - window saturated (`hits >= max`): the oldest hit still in the window,
///   plus period, bucket and buffer (N25: post-increment, 1:1; N13: full
///   bucket on top) — see `window_frees_at` for how hits we didn't make
///   (other tools on the account, N23; residue from before this daemon
///   started, N24) are accounted for;
///
/// - a landed 429 (N19, C19): response time, plus its Retry-After clamped
///   to the cap (zero when unusable), the unconditional 60s pad and buffer.
///   This hold does not depend on a usable policy observation.
///
/// The result is the max over everything that applies.
fn next_safe_send(s: &PolicyState) -> Option<Instant> {
    let mut next = s.retry_hold_until;
    let mut bump = |t: Instant| next = Some(next.map_or(t, |n| n.max(t)));

    for rule in &s.policy.rules {
        for (i, limit) in rule.limits.iter().enumerate() {
            let bucket = bucket_for_policy(&s.policy.name, rule.limits.len(), i);
            let Some(st) = rule.state.get(i) else {
                continue;
            };
            if st.restricted_secs > 0 {
                if let Some(deadline) = checked_deadline(
                    s.last_response,
                    [Duration::from_secs(st.restricted_secs), bucket, BUFFER],
                ) {
                    bump(deadline);
                }
            } else if st.hits >= limit.max_hits
                && let Some(deadline) = window_frees_at(
                    &s.history,
                    s.last_response,
                    limit.max_hits,
                    Duration::from_secs(limit.period_secs),
                    bucket,
                )
            {
                bump(deadline);
            }
        }
    }
    next
}

// ---- dashboard views ------------------------------------------------------

/// One endpoint's quote-relevant state, from `Limiter::project`: rules,
/// observation age, and ETA taken together, describing one instant.
#[derive(Debug, Clone)]
pub struct ScopeProjection {
    pub state: EndpointState,
    /// The governing policy's windows as last reported; empty unless
    /// `state` is `Policy`.
    pub rules: Vec<RuleStatus>,
    /// Seconds since the policy's headers were last observed — the basis
    /// of `rules`, older than any "as of now" the caller stamps.
    pub observed_seconds_ago: Option<u64>,
    /// Forward-simulated wait for a request with `ahead` others before
    /// it; `None` unless `state` is `Policy` (a `Policyless` route is the
    /// caller's call — the limiter has nothing to say about it).
    pub eta: Option<Duration>,
}

/// A policy's rules and window states as `RuleStatus` rows — the one
/// renderer, shared by the dashboard's `statuses` and a quote's scopes.
fn rule_statuses(ps: &PolicyState) -> Vec<RuleStatus> {
    ps.policy
        .rules
        .iter()
        .map(|r| RuleStatus {
            name: r.name.clone(),
            windows: r
                .limits
                .iter()
                .enumerate()
                .map(|(i, w)| {
                    let st = r.state.get(i);
                    WindowStatus {
                        hits: st.map(|s| s.hits).unwrap_or(0),
                        max_hits: w.max_hits,
                        period_secs: w.period_secs,
                        restriction_secs: w.restriction_secs,
                        restricted_secs: st.map(|s| s.restricted_secs).unwrap_or(0),
                        bucket_secs: bucket_for_policy(&ps.policy.name, r.limits.len(), i)
                            .as_secs(),
                    }
                })
                .collect(),
        })
        .collect()
}
