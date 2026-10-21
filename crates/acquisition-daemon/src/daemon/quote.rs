//! The quote: a read-only projection of what quoted work would meet at the
//! choke point now.
//!
//! Rulings: `decisions/plans.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::*;

impl Daemon {
    /// The `quote` request (C40, decided 2026-08-31): a read-only,
    /// non-reserving projection of what the quoted work would meet at the
    /// choke point right now. Reads the limiter and the queue, sends
    /// nothing, reserves nothing, remembers nothing — two quotes about the
    /// same work may disagree because the world moved. Account selection
    /// follows `Submit`'s rules exactly (resolved here, refused when
    /// ambiguous), so the quote keys the same limiter state a submit would.
    pub(super) fn quote(&self, jobs: &[QuoteJob], account: Option<&str>) -> Result<Quote, Refusal> {
        // The selector is judged before anything is projected — an unknown
        // or ambiguous account refuses the quote whole even when the job
        // list is empty, exactly as a submit would refuse it. Omitted with
        // several sessions live is ambiguous too: a quote is one projection
        // under one headline account, and the daemon never guesses whose.
        // The judged selector is what each job resolves against, so a
        // job the quote projects keys exactly what the same submit would key (an
        // omitted selector still runs accountless kinds accountless).
        let account = match account {
            Some(_) => self.canonical_account(account)?,
            None => {
                let s = self.shared.lock().unwrap();
                if !s.auth.by_account.is_empty() {
                    s.auth.get(None)?;
                }
                None
            }
        };
        // Group the work by scheduling scope: the policy state key once the
        // route is learned (same-name policies share counters, N6; `Account`
        // rules count per account, rung 11), else the endpoint key itself.
        let mut scopes: BTreeMap<String, (BTreeSet<String>, u64)> = BTreeMap::new();
        let mut sends_nothing: BTreeMap<String, u64> = BTreeMap::new();
        let mut resolved_account: Option<String> = account.clone();
        let mut needs_token = false;
        for job in jobs {
            let acct = self.resolve_account(&job.kind, account.as_deref())?;
            if let Some(a) = &acct {
                resolved_account.get_or_insert_with(|| a.clone());
            }
            match self.keyed_route_for(&job.kind, &job.params, acct.as_deref()) {
                None => *sends_nothing.entry(job.kind.clone()).or_insert(0) += 1,
                Some((endpoint, _)) => {
                    needs_token |= self.needs_auth(split_endpoint_key(&endpoint).0);
                    let slot = scopes.entry(self.choke.serial_key(&endpoint)).or_default();
                    slot.0.insert(endpoint);
                    slot.1 += 1;
                }
            }
        }
        // Jobs already here compete for the same scopes; the estimate puts
        // them ahead of the quoted work. Probes stay out (their own key, and
        // named under `not_covered` instead), and so does a parent holding a
        // deferred result: its own request already happened — it is only
        // waiting for its children, who count for themselves.
        let mut queued: HashMap<String, u64> = HashMap::new();
        {
            let s = self.shared.lock().unwrap();
            for e in s.jobs.values() {
                if e.info.state.is_terminal() || e.info.kind == "probe" || e.deferred.is_some() {
                    continue;
                }
                if let Some((endpoint, _)) =
                    self.keyed_route_for(&e.info.kind, &e.params, e.info.account.as_deref())
                {
                    *queued.entry(self.choke.serial_key(&endpoint)).or_insert(0) += 1;
                }
            }
        }
        let mut unprobed: BTreeSet<String> = BTreeSet::new();
        let mut out = Vec::new();
        for (key, (endpoints, requests)) in scopes {
            // Endpoints share a scope only through a learned common policy,
            // so any one of them names the scope's state. The projection is
            // taken under one limiter lock, so its rules, observation age,
            // and ETA describe the same instant.
            let sample = endpoints.iter().next().cloned().unwrap_or_default();
            let queued_ahead = queued.get(&key).copied().unwrap_or(0);
            let ahead = (queued_ahead + requests - 1) as u32;
            let projection = self.choke.project(&sample, ahead);
            let (policy, eta_seconds, notes) = match projection.state {
                EndpointState::Policy(name) => {
                    let eta = projection.eta.unwrap_or_default().as_secs();
                    (Some(name), Some(eta), Vec::new())
                }
                EndpointState::Policyless => (
                    None,
                    Some(0),
                    vec![
                        "declared policyless: paced by the send gate alone; headers, if they \
                         ever appear, are learned strictly"
                            .into(),
                    ],
                ),
                EndpointState::Unknown => {
                    let route = split_endpoint_key(&sample).0.to_string();
                    let note = if Self::route_probes(&sample) {
                        unprobed.insert(route.clone());
                        format!(
                            "policy not yet learned: a HEAD probe (its own send) precedes the \
                             first request on {route}"
                        )
                    } else {
                        format!(
                            "policy not yet learned: {route} is a declared no-probe route — \
                             its first GET teaches the limiter"
                        )
                    };
                    (None, None, vec![note])
                }
                EndpointState::Degraded { until, reason } => {
                    // The cooldown expires into `Unknown`, so the eventual
                    // replacement probe is a future send too.
                    let route = split_endpoint_key(&sample).0.to_string();
                    if Self::route_probes(&sample) {
                        unprobed.insert(route);
                    }
                    (
                        None,
                        None,
                        vec![format!(
                            "endpoint closed by a failed probe for another {}s: {reason}",
                            until.saturating_duration_since(self.choke.now()).as_secs()
                        )],
                    )
                }
            };
            out.push(QuoteScope {
                key,
                endpoints: endpoints.into_iter().collect(),
                requests,
                queued_ahead,
                policy,
                rules: projection.rules,
                observed_seconds_ago: projection.observed_seconds_ago,
                eta_seconds,
                notes,
            });
        }
        let total: u64 = out.iter().map(|s| s.requests).sum();
        let mut not_covered = Vec::new();
        if !unprobed.is_empty() {
            not_covered.push(format!(
                "a HEAD probe before the first request on {} — its own send, outside \
                 every estimate",
                unprobed.into_iter().collect::<Vec<_>>().join(", ")
            ));
        }
        if needs_token {
            not_covered.push(
                "an OAuth token refresh if the access token expires first — paced by \
                 the token policy, not quoted"
                    .into(),
            );
        }
        if total > 0 {
            not_covered.push(format!(
                "429 re-sends (up to {MAX_429_RETRIES} per request) — possible, never predicted"
            ));
        }
        for (kind, n) in sends_nothing {
            not_covered.push(format!(
                "{n} `{kind}` job(s) send nothing and are outside every scope"
            ));
        }
        Ok(Quote {
            observed_at: self
                .choke
                .wall()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0),
            provider: self.provider.name.to_string(),
            account: resolved_account,
            halted: self.rails().halted(),
            // Echoed verbatim: the quote's verifiable basis — a carrier
            // checks these tuples name exactly the work it cares about.
            work: jobs.to_vec(),
            scopes: out,
            not_covered,
        })
    }
}
