//! The login flow and an account's end: the authorize URL and its callback,
//! the staged tokens and the profile, logout and forget.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::*;

impl Daemon {
    // ---- auth -----------------------------------------------------------

    /// Kick off a login: bind a loopback redirect listener, build the
    /// authorize URL, and spawn a task that waits for the browser callback
    /// and exchanges the code. Returns the URL for the user to open.
    pub(super) async fn auth_start(self: &Arc<Self>) -> Result<String, Refusal> {
        let internal = |e: &dyn std::fmt::Display| Refusal::new(ErrorKind::Internal, e.to_string());
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .map_err(|e| internal(&format!("could not bind loopback listener: {e}")))?;
        let port = listener.local_addr().map_err(|e| internal(&e))?.port();
        // The registered callback path; GGG accepts any loopback port.
        let redirect_uri = format!("http://127.0.0.1:{port}{CALLBACK_PATH}");
        let (verifier, challenge) = auth::pkce_pair();
        let state = auth::random_token("st");
        let mut authorize_url =
            url::Url::parse(&self.provider.authorize_url).map_err(|e| internal(&e))?;
        authorize_url
            .query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", self.provider.client_id)
            .append_pair("redirect_uri", &redirect_uri)
            .append_pair("state", &state)
            .append_pair("code_challenge", &challenge)
            .append_pair("code_challenge_method", "S256")
            .append_pair("scope", &SCOPES.join(" "));
        let authorize_url = authorize_url.to_string();
        let flow_generation = self.begin_auth_flow();
        self.log(&format!("auth flow started (callback on port {port})"));

        let daemon = self.clone();
        tokio::spawn(async move {
            let callback =
                tokio::time::timeout(Duration::from_secs(300), wait_callback(listener, &state))
                    .await;
            match callback {
                Ok(Ok(code)) => {
                    // C133: the exchange is counted from before its send
                    // until its tokens are staged, so the exit waits for it.
                    daemon.shared.lock().unwrap().auth.exchanges += 1;
                    // C25: a flow a newer login superseded sends nothing;
                    // its tokens would only be ignored, and its code spent.
                    let wanted =
                        || daemon.shared.lock().unwrap().auth.pending == Some(flow_generation);
                    let exchanged = auth::exchange_code(
                        &daemon.choke,
                        &daemon.provider,
                        &code,
                        &verifier,
                        &redirect_uri,
                        &wanted,
                    )
                    .await;
                    // A block on the exchange is announced as on any send.
                    daemon.announce_trip();
                    let staged = match exchanged {
                        Err(error) if error.cancelled => Ok(None),
                        exchanged => {
                            exchanged.map(|tokens| daemon.stage_auth_flow(flow_generation, tokens))
                        }
                    };
                    daemon.shared.lock().unwrap().auth.exchanges -= 1;
                    match staged {
                        Ok(staged) => match staged {
                            Some(staged) => {
                                daemon.login_with_profile(flow_generation, staged).await;
                            }
                            None => daemon.log("stale auth flow completion ignored"),
                        },
                        Err(e) => daemon.finish_auth_flow_error(
                            flow_generation,
                            &format!("token exchange failed: {e}"),
                        ),
                    }
                }
                Ok(Err(e)) => daemon
                    .finish_auth_flow_error(flow_generation, &format!("auth callback failed: {e}")),
                Err(_) => {
                    daemon.finish_auth_flow_error(flow_generation, "auth flow timed out after 300s")
                }
            }
        });
        Ok(authorize_url)
    }

    /// A login flow is daemon-wide (one browser at a time), not a session:
    /// it disturbs no live session until its tokens arrive and name one.
    pub(super) fn begin_auth_flow(&self) -> u64 {
        let mut s = self.shared.lock().unwrap();
        s.auth.next_flow = s.auth.next_flow.wrapping_add(1);
        let generation = s.auth.next_flow;
        s.auth.pending = Some(generation);
        // A fresh flow owns the login state: an older flow's staged tokens
        // and result must not leak into it.
        s.auth.staging = None;
        s.auth.flow_result = None;
        generation
    }

    /// Token exchange succeeded: hold the tokens in the staging slot —
    /// outside the session map, so clients cannot reach them and a live
    /// session for the same account keeps serving its jobs untouched. The
    /// flow stays pending; nothing is registered until
    /// [`Self::complete_login`] has the uuid. `None`: the flow is stale (a
    /// newer login superseded it).
    pub(super) fn stage_auth_flow(
        &self,
        generation: u64,
        tokens: auth::TokenResponse,
    ) -> Option<String> {
        let mut s = self.shared.lock().unwrap();
        if s.auth.pending != Some(generation) {
            return None;
        }
        let username = tokens.username.clone();
        s.auth.staging = Some(Staging {
            flow: generation,
            profile_job: None,
            username: username.clone(),
            access_token: tokens.access_token,
            // C47: bounded where the response is read
            // (`auth::MAX_ACCESS_TOKEN_LIFETIME_SECS`), so the sum is safe.
            access_expires_at: self.choke.wall() + Duration::from_secs(tokens.expires_in),
            refresh_token: tokens.refresh_token,
        });
        Some(username)
    }

    /// C144 — the staged login's access token, handed out to the login's
    /// own profile job alone: the job `Staging::profile_job` names, which
    /// the daemon submitted and marked where it was inserted. That job
    /// must use the *new* tokens (the account may have no registered
    /// session yet, or a dead one being replaced). Any other job, a
    /// client's `profile` for the same account included, gets `None` and
    /// runs on the account's session, or fails `not_logged_in` when there
    /// is none.
    pub(super) fn staged_profile_token(
        &self,
        job: JobId,
        account: Option<&str>,
    ) -> Option<(String, String)> {
        let s = self.shared.lock().unwrap();
        s.auth
            .staging
            .as_ref()
            .filter(|st| st.profile_job == Some(job) && Some(st.username.as_str()) == account)
            .map(|st| (st.access_token.clone(), st.username.clone()))
    }

    /// Submit the login's profile job as the daemon (C136, the daemon's own
    /// submitter) and mark it on the staging slot of `generation` in the
    /// same critical section that inserts it (C144). A slot a newer flow
    /// replaced is not marked: its job then runs on the session like any
    /// other, and the stale flow's completion is ignored.
    pub(super) fn submit_login_profile(
        &self,
        generation: u64,
        username: &str,
    ) -> Result<JobId, Refusal> {
        self.submit_and_mark(
            "profile".into(),
            json!({}),
            LOGIN_PRIORITY,
            DAEMON_SUBMITTER.into(),
            Some(username.to_string()),
            |s, id| {
                if let Some(staging) = s
                    .auth
                    .staging
                    .as_mut()
                    .filter(|st| st.flow == generation && st.username == username)
                {
                    staging.profile_job = Some(id);
                }
            },
        )
    }

    /// The second half of a login (C50): submit a
    /// profile job as the staged account — causal service of the client's
    /// `acq auth` — and register the session only when the uuid lands.
    /// Any failure fails the login whole: the staged tokens are dropped and
    /// nothing was written anywhere.
    pub(super) async fn login_with_profile(&self, generation: u64, username: String) {
        let uuid = match self.submit_login_profile(generation, &username) {
            Err(e) => Err(format!("could not queue the profile fetch: {e}")),
            Ok(id) => match self.wait_job_terminal(id, LOGIN_PROFILE_TIMEOUT).await {
                None => {
                    let _ = self.cancel(id);
                    Err(format!(
                        "profile fetch (job {id}) did not finish within {}s",
                        LOGIN_PROFILE_TIMEOUT.as_secs()
                    ))
                }
                Some(Outcome::Success { payload }) => payload
                    .get("body")
                    .and_then(|b| b.get("uuid"))
                    .and_then(Value::as_str)
                    .map(str::to_string)
                    .ok_or_else(|| "profile response carried no uuid".to_string()),
                Some(Outcome::Failure { error, .. }) => {
                    Err(format!("profile fetch failed: {error}"))
                }
                Some(Outcome::Cancelled) => Err("profile fetch was cancelled".into()),
            },
        };
        match uuid {
            Ok(uuid) => {
                self.complete_login(generation, &username, &uuid);
            }
            Err(error) => self.abort_login(generation, &username, &error),
        }
    }

    /// The uuid landed: register the login in dependency order — keyring,
    /// then the index mapping (a login the index cannot record **fails
    /// whole**, keyring rolled back: a session without its uuid mapping
    /// could neither be restored nor find its annotation file), and only
    /// then the visible session and the flow's terminal result. A stale
    /// flow registers nothing.
    pub(super) fn complete_login(&self, generation: u64, username: &str, uuid: &str) -> bool {
        enum Done {
            Stale,
            Failed(String),
            Registered { warning: Option<String> },
        }
        let done = {
            let mut s = self.shared.lock().unwrap();
            if s.auth.pending != Some(generation) {
                Done::Stale
            } else if let Some(staging) = s
                .auth
                .staging
                .take_if(|st| st.flow == generation && st.username == username)
            {
                // Keyring and index writes happen under the auth lock so
                // they are ordered with logout and re-login, through the
                // one persist a refresh uses (C144): a failed save clears
                // the entry it would have replaced, and a re-login under
                // a renamed username clears the name the index mapped
                // the uuid to.
                let persisted = self.persist_credentials_locked(
                    username,
                    &staging.refresh_token,
                    Some(uuid),
                    None,
                    acquisition_store::now(),
                );
                let keyring = persisted.keyring;
                let warning =
                    (!persisted.warnings.is_empty()).then(|| persisted.warnings.join("; "));
                match persisted.indexed {
                    Err(e) => {
                        let mut error =
                            format!("could not record the login in the account index: {e}");
                        if keyring == "ok"
                            && let Err(clear) = self
                                .credential_store
                                .clear(self.provider.keyring_service, username)
                        {
                            error = format!(
                                "{error}; the keyring entry could not be rolled back either: {clear}"
                            );
                        }
                        s.auth.pending = None;
                        s.auth.flow_result = Some(Err(error.clone()));
                        Done::Failed(error)
                    }
                    Ok(()) => {
                        // A re-login replaces the account's old session here
                        // — not before — so generations advance and an
                        // in-flight refresh for it lands stale.
                        let session = s.auth.replace(AuthSession {
                            username: Some(username.to_string()),
                            uuid: Some(uuid.to_string()),
                            keyring,
                            ..AuthSession::default()
                        });
                        session.access_token = Some(staging.access_token);
                        session.access_expires_at = Some(staging.access_expires_at);
                        session.refresh_token = Some(staging.refresh_token);
                        session.advance_access_token();
                        session.advance_refresh_token();
                        s.auth.pending = None;
                        s.auth.flow_result = Some(Ok(username.to_string()));
                        Done::Registered { warning }
                    }
                }
            } else {
                let error = "the staged tokens disappeared before the profile landed".to_string();
                s.auth.pending = None;
                s.auth.flow_result = Some(Err(error.clone()));
                Done::Failed(error)
            }
        };
        match done {
            Done::Stale => {
                self.log("stale login completion ignored");
                false
            }
            Done::Failed(error) => {
                self.note_error(&format!("login failed for {username}: {error}"));
                false
            }
            Done::Registered { warning } => {
                // Only a completed login clears the dead-grant mark: until
                // here the keyring still held the rejected token.
                self.rails().clear_refresh_failed(username);
                self.announce_trip();
                // A probe that failed for lack of a session would succeed
                // now; don't make the user sit out the cooldown.
                self.choke.forget_degraded();
                self.log(&format!("logged in as {username} ({uuid})"));
                if let Some(warning) = warning {
                    self.note_error(&warning);
                }
                true
            }
        }
    }

    /// A login failed after token exchange: drop the staged tokens (it
    /// fails whole — no provisional identity) and, if the flow is still
    /// current, close it with the error as its terminal result.
    pub(super) fn abort_login(&self, generation: u64, username: &str, error: &str) {
        let current = {
            let mut s = self.shared.lock().unwrap();
            if s.auth
                .staging
                .as_ref()
                .is_some_and(|st| st.flow == generation)
            {
                s.auth.staging = None;
            }
            if s.auth.pending == Some(generation) {
                s.auth.pending = None;
                s.auth.flow_result = Some(Err(error.to_string()));
                true
            } else {
                false
            }
        };
        if current {
            self.note_error(&format!("login failed for {username}: {error}"));
        }
    }

    /// Wait until job `id` is terminal, returning its outcome (`None` on
    /// timeout). Daemon-internal; clients wait through the protocol.
    async fn wait_job_terminal(&self, id: JobId, timeout: Duration) -> Option<Outcome> {
        let mut rx = self.events.subscribe();
        tokio::time::timeout(timeout, async {
            loop {
                let outcome = {
                    let s = self.shared.lock().unwrap();
                    s.jobs
                        .get(&id)
                        .filter(|e| e.info.state.is_terminal())
                        .and_then(|e| e.outcome.clone())
                };
                if let Some(outcome) = outcome {
                    return outcome;
                }
                match rx.recv().await {
                    Ok(_) | Err(broadcast::error::RecvError::Lagged(_)) => {}
                    Err(broadcast::error::RecvError::Closed) => std::future::pending().await,
                }
            }
        })
        .await
        .ok()
    }

    fn finish_auth_flow_error(&self, generation: u64, error: &str) {
        let current = {
            let mut s = self.shared.lock().unwrap();
            if s.auth.pending == Some(generation) {
                s.auth.pending = None;
                s.auth.flow_result = Some(Err(error.to_string()));
                true
            } else {
                false
            }
        };
        if current {
            self.note_error(error);
        }
    }

    /// Drop a session (the named account's, or the sole one), its keyring
    /// entry, and its dead-grant mark. Other sessions are untouched. C144:
    /// the clear's result reaches the caller — a clear that failed is the
    /// answer, `internal`, in words that say the session was dropped and
    /// the entry was not, and how to try the clear again. The index marks
    /// the account not persisted either way, so no later start restores a
    /// session the user logged out of.
    pub(super) fn logout(&self, account: Option<&str>) -> Result<(), Refusal> {
        let (username, clear, indexed) = {
            let mut s = self.shared.lock().unwrap();
            let username = s.auth.get(account)?.username.clone().unwrap_or_default();
            s.auth
                .by_account
                .retain(|_, x| x.username.as_deref() != Some(username.as_str()));
            if s.auth.last_login.as_deref() == Some(username.as_str()) {
                s.auth.last_login = None;
            }
            let held = self.store_dir.as_ref().and_then(|dir| {
                Index::load(dir)
                    .ok()?
                    .get(&username)
                    .and_then(|e| e.keyring_name.clone())
            });
            let (clear, released) = self.clear_account_entries(&username, held.as_deref());
            // C144: the index is written under the lock, as login writes it.
            let indexed = self.index_apply(|index| {
                index.set_persisted(&username, false)?;
                if released {
                    index.hold_keyring_name(&username, None)?;
                }
                Ok(())
            });
            (username, clear, indexed)
        };
        self.rails().clear_refresh_failed(&username);
        self.announce_trip();
        if let Err(e) = indexed {
            self.note_error(&e);
        }
        match clear {
            Ok(()) => {
                self.log(&format!("logged out {username} (keyring entry cleared)"));
                Ok(())
            }
            Err(e) => {
                let message = keyring_not_cleared(
                    &format!("logged out {username} (session dropped)"),
                    &username,
                    &e,
                );
                self.note_error(&message);
                Err(Refusal::new(ErrorKind::Internal, message))
            }
        }
    }

    /// Drop a *non-live* account's keyring entry and mark it not persisted.
    /// Nothing about the live session changes. C144: the entry is cleared
    /// whether or not the index marks it persisted (a failed save, or a
    /// logout whose clear failed, leaves an entry the mark does not
    /// show), and the log and the answer say what the clear did.
    pub(super) fn forget_account(&self, selector: &str) -> Result<(), Refusal> {
        let Some(dir) = &self.store_dir else {
            return Err(Refusal::new(
                ErrorKind::Internal,
                "no account index in this daemon",
            ));
        };
        // A malformed index's parse error can quote a stored value whole
        // (C127: no refusal quotes a value whole).
        let index = Index::load(dir).map_err(|e| {
            Refusal::new(
                ErrorKind::Internal,
                format!(
                    "accounts index: {}",
                    bounded_message(&format_args!("{e:#}"), "")
                ),
            )
        })?;
        let entry = index
            .resolve(Some(selector))
            .map_err(|e| match e {
                acquisition_store::Resolve::Ambiguous(_) => {
                    Refusal::new(ErrorKind::AmbiguousAccount, e.to_string())
                }
                // The index's own words quote the selector whole (C127).
                acquisition_store::Resolve::NotFound(selector) => Refusal::new(
                    ErrorKind::NotLoggedIn,
                    format!(
                        "no account matches {} (see `acq accounts`)",
                        describe_str(&selector)
                    ),
                ),
                acquisition_store::Resolve::Empty => {
                    Refusal::new(ErrorKind::NotLoggedIn, e.to_string())
                }
            })?
            .clone();
        let (cleared, released) =
            self.clear_account_entries(&entry.username, entry.keyring_name.as_deref());
        self.with_index(|index| {
            index.set_persisted(&entry.username, false)?;
            if released {
                index.hold_keyring_name(&entry.username, None)?;
            }
            Ok(())
        });
        match cleared {
            Ok(()) => {
                self.log(&format!(
                    "forgot account {} (keyring entry cleared)",
                    entry.username
                ));
                Ok(())
            }
            Err(e) => {
                let message = keyring_not_cleared(
                    &format!("forgot account {} (not restored at start)", entry.username),
                    &entry.username,
                    &e,
                );
                self.note_error(&message);
                Err(Refusal::new(ErrorKind::Internal, message))
            }
        }
    }
}

impl Daemon {
    /// C144 (no keyring entry out of the index's reach): clear the
    /// account's keyring entry under its username and under the old name
    /// the index holds for it (`AccountEntry::keyring_name`), if any.
    /// Both clears run; the first failure is the result. The flag says
    /// whether the held name may be released: there was none, or its
    /// clear succeeded.
    fn clear_account_entries(
        &self,
        username: &str,
        held: Option<&str>,
    ) -> (Result<(), String>, bool) {
        let service = self.provider.keyring_service;
        let cleared = self.credential_store.clear(service, username);
        let Some(held) = held.filter(|h| *h != username) else {
            return (cleared, true);
        };
        match self.credential_store.clear(service, held) {
            Ok(()) => (cleared, true),
            Err(e) => (
                cleared.and(Err(format!("under the old name {held}: {e}"))),
                false,
            ),
        }
    }
}

/// C144: the words of a logout or a forget whose keyring clear failed —
/// what was done, what was not, and the request that tries the clear
/// again (a logout of a known account that is not live is a forget).
fn keyring_not_cleared(done: &str, username: &str, error: &str) -> String {
    format!(
        "{done}, but its keyring entry could not be cleared: {error}; `acq auth logout --account {username}` tries again"
    )
}

/// Wait for the browser to hit the loopback redirect with an auth code.
/// Ignores stray requests (favicons etc.) until `/callback` arrives.
async fn wait_callback(listener: TcpListener, expected_state: &str) -> Result<String, String> {
    loop {
        let (mut stream, _) = listener.accept().await.map_err(|e| e.to_string())?;
        let Some(req) = mockggg::read_request(&mut stream).await else {
            continue;
        };
        if req.method != "GET" || req.path != CALLBACK_PATH {
            mockggg::respond(&mut stream, "404 Not Found", "text/plain", "not found").await;
            continue;
        }
        match (req.query.get("code"), req.query.get("state")) {
            (Some(code), Some(state)) if state == expected_state => {
                mockggg::respond(
                    &mut stream,
                    "200 OK",
                    "text/html",
                    "<h1>Logged in</h1><p>You can close this tab and return to the terminal.</p>",
                )
                .await;
                return Ok(code.clone());
            }
            _ => {
                mockggg::respond(
                    &mut stream,
                    "400 Bad Request",
                    "text/plain",
                    "state mismatch",
                )
                .await;
                return Err("callback state mismatch".into());
            }
        }
    }
}
