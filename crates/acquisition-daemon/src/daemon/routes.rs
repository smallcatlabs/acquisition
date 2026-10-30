//! The routes a job sends on, the per-route knowledge headers cannot teach,
//! and the probes that come before a route's first send.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.
//!
//! # Decisions as recorded
//!
//! ## C32 — Per-route knowledge about GGG that headers cannot teach lives in one place (`Daemon::de…
//!
//! **Per-route knowledge about GGG that headers cannot teach lives in one place (`Daemon::declare_route_knowledge`), and strict observation is the default everywhere else.** `GET /profile` (first contact 2026-08-30) answers 200 with no `X-Rate-Limit-*` headers at all and 403 to HEAD, which strict observation ("every endpoint has a policy", post-N33) classed as a protocol failure and discarded. Now: a route *declared* policyless accepts a 2xx with **no** rate-limit header (a partial set is still a failure; a policy that later appears is learned strictly), becomes `EndpointState::Policyless`, and is paced by nothing but the send gate; a declared no-probe route goes straight to its GET. Only `/profile` is declared, and it is called at most once per login. Not generalised on purpose: "any headerless 2xx is fine" reopens the blind spot strict observation closed. Owner decision 2026-08-30; GGG confirmed the same day (Q12/N38): `/profile` is not rate limited at present, so the declaration is confirmed and stays until headers ever appear — strict observation covers that arm.
//!

use super::*;

impl Daemon {
    /// The route label and URL a job sends on, if it sends at all. Routes,
    /// not URLs, key the limiter: every league shares `stash-list`. A
    /// realm other than pc is a segment on the URL *and* on the label
    /// (`stash-list/xbox`): a realm's URL shape gets its own free HEAD
    /// probe before its first counted send, and whether it shares the pc
    /// policy is learned from its headers (same-name policies already
    /// share state, N6). pc adds nothing to either, so every pc URL and
    /// journal route is byte-identical to the pre-realm ones. A param on
    /// the URL is one percent-encoded segment, or there is no URL (C127,
    /// `with_segments`): params the client's door would refuse
    /// (`admit_segments`) render none, and `why_no_url` says why. `fetch`
    /// is a fake data endpoint that exists only on the mock.
    pub(super) fn route_for(&self, kind: &str, params: &Value) -> Option<(String, String)> {
        admit_segments(kind, params).ok()?;
        let base = &self.provider.api_base;
        // Admission (`admit_realm`) refused anything a family does not
        // take before a job existed; a row that still fails here (a
        // persisted job from a build with a different table) sends nothing.
        let realm = |family: Family| family.realm_of(params).ok().map(Realm::infix);
        match kind {
            "characters" => {
                let r = realm(Family::Characters)?;
                Some((format!("character-list{r}"), format!("{base}/character{r}")))
            }
            // One character with its inventory/equipment: its own policy.
            "character" => {
                let r = realm(Family::Characters)?;
                let name = params.get("name").and_then(Value::as_str)?;
                Some((
                    format!("character{r}"),
                    with_segments(&format!("{base}/character{r}"), &[name])?,
                ))
            }
            // The account's leagues: `GET /account/leagues` (account:leagues).
            // `/league` is the public league list and needs `service:leagues`,
            // which the registration does not have (first contact
            // 2026-08-30: 403 `insufficient_scope`). Not realm-aware: no
            // consumer asks, and the plan never lists leagues.
            "leagues" => Some(("league".into(), format!("{base}/account/leagues"))),
            // The account profile (account:profile).
            "profile" => Some(("profile".into(), format!("{base}/profile"))),
            "stashes" => {
                let r = realm(Family::Stashes)?;
                let league = params
                    .get("league")
                    .and_then(Value::as_str)
                    .unwrap_or("Standard");
                Some((
                    format!("stash-list{r}"),
                    with_segments(&format!("{base}/stash{r}"), &[league])?,
                ))
            }
            // One tab, or one substash of a map/unique tab: same route, same
            // policy (stash-request-limit), one probe for all of them.
            "stash" => {
                let r = realm(Family::Stashes)?;
                let league = params
                    .get("league")
                    .and_then(Value::as_str)
                    .unwrap_or("Standard");
                let id = params.get("id").and_then(Value::as_str)?;
                let prefix = format!("{base}/stash{r}");
                let url = match params.get("sub").and_then(Value::as_str) {
                    Some(sub) => with_segments(&prefix, &[league, id, sub])?,
                    None => with_segments(&prefix, &[league, id])?,
                };
                Some((format!("stash{r}"), url))
            }
            "fetch" if !self.provider.is_real() => Some(("fetch".into(), format!("{base}/fetch"))),
            _ => None,
        }
    }

    /// `route_for`, keyed by the job's account: the limiter paces
    /// `Account`-scoped policies per account (rung 11) and serializes per
    /// `(account, policy)`; probes are per `(account, route)`.
    pub(super) fn keyed_route_for(
        &self,
        kind: &str,
        params: &Value,
        account: Option<&str>,
    ) -> Option<(String, String)> {
        let (route, url) = self.route_for(kind, params)?;
        Some((endpoint_key(&route, account), url))
    }

    /// Whether a route needs the bearer token. The mock's `fetch` is open;
    /// everything on the real API is not.
    pub(super) fn needs_auth(&self, route: &str) -> bool {
        self.provider.is_real() || route != "fetch"
    }

    /// What a job must not overlap with: probes with each other (N18),
    /// network jobs with anything under the same policy, everything else
    /// only with itself.
    pub(super) fn serial_key(&self, e: &Entry) -> String {
        if e.info.kind == "probe" {
            return "probe".into();
        }
        match self.keyed_route_for(&e.info.kind, &e.info.params, e.info.account.as_deref()) {
            Some((route, _)) => self.choke.serial_key(&route),
            None => format!("solo:{}", e.info.id),
        }
    }

    pub(super) fn probe_pending(s: &Shared, route: &str) -> bool {
        s.jobs.values().any(|e| {
            e.info.kind == "probe"
                && !e.info.state.is_terminal()
                && e.info.params.get("route").and_then(Value::as_str) == Some(route)
        })
    }

    /// Per-route knowledge about GGG that headers cannot teach, kept in
    /// one place (`RUN-LEDGER.md`, 2026-08-30):
    /// - `/profile`: HEAD is answered 403, and GET answers 200 with no
    ///   `X-Rate-Limit-*` headers at all — not probed, may be policyless.
    /// - `/account/leagues`: HEAD is answered 200 and **counted** as a hit
    ///   (state `1:10:0` after the probe; N24's free HEAD is per endpoint),
    ///   so a probe costs what the GET costs — not probed; the GET teaches.
    ///
    /// Every other route is probed (HEAD uncounted) and observed strictly.
    /// GGG's word on both is recorded: N38 (`/profile`) and N39
    /// (`/account/leagues`).
    pub(super) fn declare_route_knowledge(choke: &ChokePoint) {
        choke.declare_policyless("profile");
    }

    /// Routes whose first use is not preceded by a HEAD probe (see
    /// `declare_route_knowledge`).
    pub(super) const NO_PROBE_ROUTES: &'static [&'static str] = &["profile", "league"];

    /// The kinds that have no route of their own and may still send: a
    /// probe (its HEAD) and the mock's `whoami` (a refresh when its token
    /// is due). A halt holds them as it holds the network kinds (C25).
    pub(super) fn sends_unrouted(kind: &str) -> bool {
        matches!(kind, "probe" | "whoami")
    }

    /// Whether a route's first use is preceded by a HEAD probe. `route` is
    /// the endpoint key (`profile@user`).
    pub(super) fn route_probes(route: &str) -> bool {
        !Self::NO_PROBE_ROUTES.contains(&crate::ratelimit::split_endpoint_key(route).0)
    }

    /// Make sure a probe for `route` is queued or running; submit one if not.
    /// One probe per route per daemon lifetime in the normal case — the
    /// sanctioned "one HEAD at startup" (N16), sent lazily on first use.
    pub(super) fn ensure_probe(&self, route: &str, url: &str, account: Option<String>) {
        let pending = Self::probe_pending(&self.shared.lock().unwrap(), route);
        if !pending {
            self.log(&format!(
                "route {route} unknown; probing {} first",
                url_path(url)
            ));
            if let Err(e) = self.submit(
                "probe".into(),
                json!({ "route": route, "url": url }),
                PROBE_PRIORITY,
                DAEMON_SUBMITTER.into(),
                account,
            ) {
                // Queue failed: the dispatcher has stopped picking, so the
                // jobs behind this probe wait rather than spin.
                self.log(&format!("JOBS: could not queue a probe for {route}: {e}"));
            }
        }
    }
}

/// C127 — `prefix` with each param appended as one percent-encoded path
/// segment (`%`, `/`, `?` and `#` among the encoded), so a param can
/// neither leave its segment nor start a query. It only encodes: the one
/// gate is `admit_segments`, which `route_for` runs on the same params
/// first, so a param that is not one segment never reaches here.
fn with_segments(prefix: &str, segments: &[&str]) -> Option<String> {
    let mut url = url::Url::parse(prefix).ok()?;
    url.path_segments_mut().ok()?.extend(segments);
    Some(url.into())
}

/// C127 — Why `route_for` rendered no URL for a row that never passed the
/// client's door (a restored row, or a child whose params came from a
/// response), in the door's own words: the realm refusal, or the segment
/// refusal. A job that cannot render its URL fails with these words.
pub(super) fn why_no_url(kind: &str, params: &Value) -> String {
    if let Err(refusal) = admit_realm(kind, params) {
        return refusal.message;
    }
    if let Err(words) = admit_segments(kind, params) {
        return words;
    }
    format!("{kind}: no URL renders from these params")
}
