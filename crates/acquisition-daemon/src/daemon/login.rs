//! The login flow and an account's end: the authorize URL and its callback,
//! the staged tokens and the profile, logout and forget. C148: a login's
//! landing and a logout's or forget's run on a blocking thread, their
//! session-table changes under `shared` and their keyring and index I/O
//! under `credentials` alone; a login being recorded refuses a second.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::*;
use acquisition_protocol::protocol::KeyringOutcome;
use acquisition_store::AccountEntry;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

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
        let flow_generation = self.start_auth_flow()?;
        self.log(&format!("auth flow started (callback on port {port})"));

        let daemon = self.clone();
        tokio::spawn(async move {
            let callback = tokio::time::timeout(
                daemon.login_within,
                wait_callback(listener, &state, CALLBACK_READ_WITHIN),
            )
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
                // C137: the browser wait's expiry, its words saying what to do.
                Err(_) => daemon.finish_auth_flow_error(
                    flow_generation,
                    &format!(
                        "the browser did not complete the login within {} s; run `acq auth` again",
                        daemon.login_within.as_secs()
                    ),
                ),
            }
        });
        Ok(authorize_url)
    }

    /// A login flow is daemon-wide (one browser at a time), not a session:
    /// it disturbs no live session until its tokens arrive and name one.
    /// A flow waiting on the browser is superseded; a login being
    /// recorded is not (C148): its credential may be saved already, and
    /// a flow cleared then would register no session for it. The start
    /// is refused instead, `refused`, as a stopping daemon refuses a
    /// submit (C133), in the same hold that would have begun the flow.
    pub(super) fn start_auth_flow(&self) -> Result<u64, Refusal> {
        let mut s = self.shared.lock().unwrap();
        if s.auth.landing.is_some() {
            return Err(Refusal::new(
                ErrorKind::Refused,
                "a login is being recorded; try again in a moment",
            ));
        }
        s.auth.next_flow = s.auth.next_flow.wrapping_add(1);
        let generation = s.auth.next_flow;
        s.auth.pending = Some(generation);
        // A fresh flow owns the login state: an older flow's staged tokens
        // and result must not leak into it.
        s.auth.staging = None;
        s.auth.flow_result = None;
        Ok(generation)
    }

    /// Test helper: begin a flow where no login is being recorded.
    #[cfg(test)]
    pub(super) fn begin_auth_flow(&self) -> u64 {
        self.start_auth_flow().expect("no login is being recorded")
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
    ) -> Option<(auth::Secret, String)> {
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
    pub(super) async fn login_with_profile(self: &Arc<Self>, generation: u64, username: String) {
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
                Some(Err(refusal)) => Err(format!(
                    "profile fetch (job {id}) finished, but its outcome could not be read: {refusal}"
                )),
                Some(Ok(Outcome::Success { payload })) => payload
                    .get("body")
                    .and_then(|b| b.get("uuid"))
                    .and_then(Value::as_str)
                    .map(str::to_string)
                    .ok_or_else(|| "profile response carried no uuid".to_string()),
                Some(Ok(Outcome::Failure { error, .. })) => {
                    Err(format!("profile fetch failed: {error}"))
                }
                Some(Ok(Outcome::Cancelled)) => Err("profile fetch was cancelled".into()),
            },
        };
        match uuid {
            // C148: the landing's keyring and index I/O blocks, so it runs
            // on a blocking thread, never on a runtime worker.
            Ok(uuid) => {
                let _ = self
                    .spawn_landing(move |daemon| {
                        daemon.complete_login(generation, &username, &uuid)
                    })
                    .await;
            }
            Err(error) => self.abort_login(generation, &username, &error),
        }
    }

    /// The uuid landed: register the login in dependency order — the
    /// index holding the name, the keyring, then the index mapping (a
    /// login the index cannot record **fails whole**: no save when the
    /// name cannot be held, the keyring rolled back when the mapping
    /// cannot be written; a session without its uuid mapping could
    /// neither be restored nor find its annotation file), and only
    /// then the visible session and the flow's terminal result. A stale
    /// flow registers nothing.
    ///
    /// C148, the landing: it holds `credentials` throughout, which orders
    /// it with every other landing (C144), and takes `shared` twice,
    /// never across the I/O. First the staging slot is taken and the
    /// flow marked as landing, so a second start is refused and the drain
    /// waits; then the keyring and the index, through the one persist a
    /// refresh uses; then the session replaced, or the flow failed, and
    /// the mark cleared. It blocks: an async caller runs it on a blocking
    /// thread.
    pub(super) fn complete_login(&self, generation: u64, username: &str, uuid: &str) -> bool {
        enum Done {
            Stale,
            Failed(String),
            Registered { warning: Option<String> },
        }
        let credentials = self.hold_credentials();
        let staged = {
            let mut s = self.shared.lock().unwrap();
            if s.auth.pending != Some(generation) {
                Err(Done::Stale)
            } else if let Some(staging) = s
                .auth
                .staging
                .take_if(|st| st.flow == generation && st.username == username)
            {
                s.auth.landing = Some(generation);
                Ok(staging)
            } else {
                let error = "the staged tokens disappeared before the profile landed".to_string();
                s.auth.pending = None;
                s.auth.flow_result = Some(Err(error.clone()));
                Err(Done::Failed(error))
            }
        };
        let done = match staged {
            Err(done) => done,
            Ok(staging) => {
                // A failed save clears the entry it would have replaced,
                // and a re-login under a renamed username clears the name
                // the index mapped the uuid to (C144).
                // C135: the staged access token and its expiry are saved
                // beside the refresh token, as the session will hold them.
                let entry = auth::KeyringEntry::new(
                    staging.refresh_token.clone(),
                    Some((staging.access_token.clone(), staging.access_expires_at)),
                );
                let persisted = self.persist_credentials(
                    &credentials,
                    username,
                    &entry,
                    Some(uuid),
                    None,
                    acquisition_store::now(),
                );
                let keyring = persisted.keyring;
                let warning =
                    (!persisted.warnings.is_empty()).then(|| persisted.warnings.join("; "));
                let indexed = persisted.indexed.map_err(|e| {
                    let mut error = format!("could not record the login in the account index: {e}");
                    if keyring == "ok"
                        && let Err(clear) = self
                            .credential_store
                            .clear(self.provider.keyring_service, username)
                    {
                        error = format!(
                            "{error}; the keyring entry could not be rolled back either: {clear}"
                        );
                    }
                    error
                });
                let mut s = self.shared.lock().unwrap();
                s.auth.landing = None;
                s.auth.pending = None;
                match indexed {
                    Err(error) => {
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
                        s.auth.flow_result = Some(Ok(username.to_string()));
                        Done::Registered { warning }
                    }
                }
            }
        };
        drop(credentials);
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
    /// timeout). A job concluded and evicted is read from the table (C6),
    /// and a read that fails is the answer, never a reason to wait on:
    /// the job has concluded, so no later event would bring its outcome.
    /// Daemon-internal; clients wait through the protocol.
    pub(super) async fn wait_job_terminal(
        &self,
        id: JobId,
        timeout: Duration,
    ) -> Option<Result<Outcome, Refusal>> {
        let mut rx = self.events.subscribe();
        tokio::time::timeout(timeout, async {
            loop {
                let held = {
                    let s = self.shared.lock().unwrap();
                    s.jobs.get(&id).map(|e| {
                        e.info
                            .state
                            .is_terminal()
                            .then(|| e.outcome.clone())
                            .flatten()
                    })
                };
                match held {
                    Some(Some(outcome)) => return Ok(outcome),
                    Some(None) => {}
                    // Not in memory: concluded and evicted, its outcome in
                    // the table (C6).
                    None => {
                        return self.stored_outcome(id).and_then(|outcome| {
                            outcome.ok_or_else(|| {
                                Refusal::new(
                                    ErrorKind::QueueFailed,
                                    format!("job {id} left memory with no outcome in the queue"),
                                )
                            })
                        });
                    }
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

    /// `acq auth logout`: drop a live session (the named account's, or the
    /// sole one), its keyring entries and its dead-grant mark; a selector
    /// that names no live session but a known account forgets it — its
    /// keyring entries cleared, its index entry marked not persisted, the
    /// live sessions untouched. C144, one landing: the live-or-known
    /// decision, the index read, every clear and the index write happen
    /// under one hold of `credentials`, the lock a login persists under,
    /// so no login lands between the decision and the mark. C148: the
    /// session is dropped under `shared` in a hold that does no I/O; a
    /// forget's index read comes before its second look at the live
    /// sessions; the clears and the write run under `credentials` alone.
    /// It blocks: the request handler runs it on a blocking thread
    /// ([`Self::logout_blocking`]). The account's index entry is found by
    /// its uuid (the session's, or the one the selector resolves to
    /// through the index), never by a username a rename may have moved,
    /// and every name a keyring entry may be under is cleared: the
    /// entry's username and held names, and the session's own name. The
    /// index then holds each name whose clear failed, releases each that
    /// succeeded and marks the account not persisted, in one write. Every
    /// result reaches the caller (`settle`).
    pub(super) fn logout(&self, account: Option<&str>) -> Result<KeyringOutcome, Refusal> {
        let credentials = self.hold_credentials();
        let named = |x: &AuthSession| (x.username.clone().unwrap_or_default(), x.uuid.clone());
        let live = {
            let mut s = self.shared.lock().unwrap();
            let live = match account {
                None => s.auth.get(None).map(named),
                Some(req) => s
                    .auth
                    .matching(req)
                    .and_then(|found| found.map(named).ok_or_else(|| no_session(req))),
            };
            if let Ok((username, _)) = &live {
                s.auth.drop_session(username);
            }
            live
        };
        let (username, uuid, dropped, entry) = match (live, account) {
            (Ok((username, uuid)), _) => {
                let entry = self.store_dir.as_ref().and_then(|dir| {
                    Index::load(dir)
                        .ok()?
                        .find(uuid.as_deref(), &username)
                        .cloned()
                });
                (username, uuid, true, entry)
            }
            (Err(_), Some(selector)) => {
                let entry = self.known_account(selector)?;
                // A known account whose uuid a live session carries
                // under a newer name is that session's logout.
                let live = {
                    let mut s = self.shared.lock().unwrap();
                    let live = entry.uuid.as_ref().and_then(|uuid| {
                        s.auth
                            .by_account
                            .values()
                            .find(|x| x.uuid.as_ref() == Some(uuid))
                            .map(named)
                    });
                    if let Some((username, _)) = &live {
                        s.auth.drop_session(username);
                    }
                    live
                };
                match live {
                    Some((username, uuid)) => (username, uuid, true, Some(entry)),
                    None => (
                        entry.username.clone(),
                        entry.uuid.clone(),
                        false,
                        Some(entry),
                    ),
                }
            }
            (Err(refusal), None) => return Err(refusal),
        };
        let mut names = vec![username.clone()];
        for name in entry.iter().flat_map(AccountEntry::keyring_reach) {
            if !names.contains(&name) {
                names.push(name);
            }
        }
        let (cleared, released, failed) = self.clear_account_entries(&names);
        let indexed = self.index_apply(&credentials, |index| {
            index.settle_keyring_names(uuid.as_deref(), &username, &failed, &released, Some(false))
        });
        drop(credentials);
        if dropped {
            self.rails().clear_refresh_failed(&username);
            self.announce_trip();
        }
        self.settle(Settled {
            username: &username,
            uuid: uuid.as_deref(),
            session_dropped: dropped,
            cleared,
            indexed,
        })
    }

    /// [`Self::logout`] on a blocking thread (C148): its keyring and index
    /// I/O never runs on a runtime worker.
    pub(super) async fn logout_blocking(
        self: &Arc<Self>,
        account: Option<String>,
    ) -> Result<KeyringOutcome, Refusal> {
        self.spawn_landing(move |daemon| daemon.logout(account.as_deref()))
            .await
            .unwrap_or_else(|e| {
                Err(Refusal::new(
                    ErrorKind::Internal,
                    format!("the logout did not finish: {e}"),
                ))
            })
    }

    /// The index entry a selector names, for a forget: the account known
    /// to the index, or the refusal that says why there is none.
    fn known_account(&self, selector: &str) -> Result<AccountEntry, Refusal> {
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
        index
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
            })
            .cloned()
    }

    /// C144 — the answer of a logout (`session_dropped`) or a forget, from
    /// what each step did. When the index marked the account not
    /// persisted and every clear succeeded, or the keyring is off and
    /// tried none, the answer is an `ack` carrying that outcome as data
    /// (`KeyringOutcome`), and the log says it in words. Otherwise — a
    /// clear that failed, or an index that could not be written — the
    /// answer is `internal`, in a sentence that says what was done and
    /// what was not: "not restored at start" only when the index's mark
    /// was written; a keyring turned off, which cleared nothing; a clear
    /// that failed, with the request that tries it again (a logout of a
    /// known account that is not live is a forget).
    fn settle(&self, done: Settled<'_>) -> Result<KeyringOutcome, Refusal> {
        let Settled {
            username,
            uuid,
            session_dropped,
            cleared,
            indexed,
        } = done;
        let what = if session_dropped {
            format!("logged out {username}")
        } else {
            format!("forgot account {username}")
        };
        match (&cleared, &indexed) {
            (Ok(()), Ok(())) => {
                self.log(&format!("{what} (keyring entry cleared)"));
                return Ok(KeyringOutcome::Cleared);
            }
            (Err(auth::NotCleared::Off(why)), Ok(())) => {
                self.log(&format!("{what} (keyring off, nothing cleared: {why})"));
                return Ok(KeyringOutcome::Off);
            }
            _ => {}
        }
        let mut facts = Vec::new();
        if session_dropped {
            facts.push("session dropped");
        }
        if indexed.is_ok() {
            facts.push("not restored at start");
        }
        if cleared.is_ok() {
            facts.push("keyring entry cleared");
        }
        let mut but = Vec::new();
        if let Err(e) = &indexed {
            but.push(format!(
                "the account index could not mark it not persisted ({e}), so a start may still try to restore it"
            ));
        }
        // The retry names the account by its uuid, a word no shell reads
        // specially, so it runs as printed whichever shell it is pasted
        // into, not only the one this daemon was started from (C156; the
        // review of words' fix round, its first finding); the username,
        // quoted, only for an entry the index holds no uuid for.
        let selector = uuid.map_or_else(|| shell_word(username), str::to_string);
        let retry = format!("`acq auth logout --account {selector}`");
        match &cleared {
            Ok(()) => {}
            Err(auth::NotCleared::Off(why)) => but.push(format!(
                "the keyring is off ({why}), so no keyring entry was cleared; with it on, {retry} clears one"
            )),
            Err(auth::NotCleared::Failed(e)) => but.push(format!(
                "its keyring entry could not be cleared: {e}; {retry} tries again"
            )),
        }
        let head = if facts.is_empty() {
            what
        } else {
            format!("{what} ({})", facts.join(", "))
        };
        let message = format!("{head}, but {}", but.join(", and "));
        self.note_error(&message);
        Err(Refusal::new(ErrorKind::Internal, message))
    }
}

/// What a logout or a forget did, for [`Daemon::settle`]'s words.
struct Settled<'a> {
    username: &'a str,
    /// The account's uuid, which the retry names where there is one.
    uuid: Option<&'a str>,
    session_dropped: bool,
    cleared: Result<(), auth::NotCleared>,
    indexed: Result<(), String>,
}

impl Daemon {
    /// C144 (no keyring entry out of the index's reach): clear the
    /// account's keyring entry under every name in `names`. Every clear
    /// runs; the first failure is the result, beside the names whose
    /// clear succeeded (released from the index) and those whose clear
    /// failed (held by it). A keyring turned off clears nothing and
    /// moves no name.
    fn clear_account_entries(
        &self,
        names: &[String],
    ) -> (Result<(), auth::NotCleared>, Vec<String>, Vec<String>) {
        let service = self.provider.keyring_service;
        let mut result = Ok(());
        let (mut released, mut failed) = (Vec::new(), Vec::new());
        for (i, name) in names.iter().enumerate() {
            match self.credential_store.clear(service, name) {
                Ok(()) => released.push(name.clone()),
                Err(auth::NotCleared::Failed(e)) => {
                    failed.push(name.clone());
                    let e = if i == 0 {
                        e
                    } else {
                        format!("under the old name {name}: {e}")
                    };
                    result = result.and(Err(auth::NotCleared::Failed(e)));
                }
                Err(off) => result = result.and(Err(off)),
            }
        }
        (result, released, failed)
    }
}

/// One argument of a printed command, quoted for the shell it will be
/// pasted into (C156): PowerShell's on Windows outside Git Bash (which
/// sets `MSYSTEM`), a POSIX shell's elsewhere; the daemon reads the
/// environment the command that spawned it handed it. The search crate's
/// `shell_quoted` is the twin every other printed command uses; C1 (the
/// daemon never links the search) keeps this one apart, the same rules
/// held by the same cases.
pub(super) fn shell_word(word: &str) -> String {
    if cfg!(windows) && std::env::var_os("MSYSTEM").is_none() {
        powershell_word(word)
    } else {
        posix_word(word)
    }
}

/// A POSIX shell's word: bare when every character is one no shell reads
/// specially, otherwise single-quoted, a quote inside written `'\''`.
/// `#` is quoted: a word that starts with it is a comment, and zsh's
/// extended globbing reads it anywhere.
pub(super) fn posix_word(word: &str) -> String {
    let plain = !word.is_empty()
        && word
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_.,/:@%+=".contains(c));
    if plain {
        word.to_string()
    } else {
        format!("'{}'", word.replace('\'', "'\\''"))
    }
}

/// A PowerShell word: bare when every character is one PowerShell's
/// argument mode passes through and the word is no number it would print
/// another way (`0123`, `1e3`), otherwise single-quoted, a quote inside
/// doubled, the typographic single quotes PowerShell also reads as
/// quotes included. `@` (a splat) and `,` (an array) are quoted.
pub(super) fn powershell_word(word: &str) -> String {
    let plain = !word.is_empty()
        && word
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_./:=".contains(c))
        && !(powershell_number(word) && !canonical_number(word));
    if plain {
        return word.to_string();
    }
    let mut out = String::from("'");
    for c in word.chars() {
        if matches!(c, '\'' | '\u{2018}' | '\u{2019}' | '\u{201a}' | '\u{201b}') {
            out.push(c);
        }
        out.push(c);
    }
    out.push('\'');
    out
}

/// Whether PowerShell's argument mode reads a bare word as a number: a
/// sign, then a hexadecimal (`0x`), binary (`0b`) or decimal literal
/// with its exponent, then a type suffix and a multiplier, each optional.
fn powershell_number(word: &str) -> bool {
    let word = word.to_ascii_lowercase();
    let word = word.trim_start_matches(['+', '-']);
    let bodies = ["kb", "mb", "gb", "tb", "pb", ""]
        .iter()
        .filter_map(|m| word.strip_suffix(m))
        .flat_map(|w| {
            ["ul", "us", "uy", "u", "l", "y", "s", "d", "n", ""]
                .iter()
                .filter_map(move |t| w.strip_suffix(t))
        });
    let digits = |s: &str, of: fn(char) -> bool| !s.is_empty() && s.chars().all(of);
    bodies.into_iter().any(|body| {
        if let Some(hex) = body.strip_prefix("0x") {
            return digits(hex, |c| c.is_ascii_hexdigit());
        }
        if let Some(bin) = body.strip_prefix("0b") {
            return digits(bin, |c| c == '0' || c == '1');
        }
        let (mantissa, exponent) = match body.split_once('e') {
            Some((m, e)) => (m, Some(e.trim_start_matches(['+', '-']))),
            None => (body, None),
        };
        let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
        let decimal = |s: &str| s.chars().all(|c| c.is_ascii_digit());
        (!whole.is_empty() || !fraction.is_empty())
            && decimal(whole)
            && decimal(fraction)
            && exponent.is_none_or(|e| digits(e, |c| c.is_ascii_digit()))
    })
}

/// A number PowerShell prints back as written: no sign, no leading zero
/// but a lone one, a fraction that ends in a digit other than zero.
fn canonical_number(word: &str) -> bool {
    let (whole, fraction) = word.split_once('.').unwrap_or((word, ""));
    let whole_ok = whole == "0"
        || (whole.starts_with(|c: char| ('1'..='9').contains(&c))
            && whole.chars().all(|c| c.is_ascii_digit()));
    let fraction_ok = !word.contains('.')
        || (!fraction.is_empty()
            && fraction.chars().all(|c| c.is_ascii_digit())
            && !fraction.ends_with('0'));
    whole_ok && fraction_ok
}

/// The bound on one callback connection's request read. A browser's
/// redirect arrives whole in milliseconds; a connection still silent
/// after this long is not the browser, and is dropped unanswered.
pub(super) const CALLBACK_READ_WITHIN: Duration = Duration::from_secs(10);

/// Wait for the browser to hit the loopback redirect with an auth code.
///
/// Each accepted connection is answered in its own task, its request
/// read under `read_within`, so a connection that sends nothing holds
/// nothing up: the next accept proceeds, and the silent connection is
/// dropped unanswered when its bound expires. The flow's `state` is the
/// one key. A request on the callback path without it, another login's
/// or none, is answered 400 and ignored, as any other path is answered
/// 404: a local process that finds the port can neither end the flow nor
/// take the code. With the state, a `code` ends the flow with the code,
/// and an `error` (the authorize page's denial, RFC 6749 section 4.1.2.1)
/// ends it failed in the browser's words. The tasks report an end on a
/// channel; the first end wins and the listener closes with the return,
/// so a task still answering writes its page and its report is dropped.
pub(super) async fn wait_callback(
    listener: TcpListener,
    expected_state: &str,
    read_within: Duration,
) -> Result<auth::Secret, String> {
    let (ended, mut ends) = mpsc::channel::<Result<auth::Secret, String>>(1);
    let expected_state: Arc<str> = Arc::from(expected_state);
    loop {
        tokio::select! {
            accepted = listener.accept() => {
                let (stream, _) = accepted.map_err(|e| e.to_string())?;
                tokio::spawn(answer_callback(
                    stream,
                    expected_state.clone(),
                    read_within,
                    ended.clone(),
                ));
            }
            Some(end) = ends.recv() => return end,
        }
    }
}

/// One callback connection ([`wait_callback`]): the request read under
/// its bound, the page the browser reads, and the flow's end when the
/// request is the flow's own.
async fn answer_callback(
    mut stream: TcpStream,
    expected_state: Arc<str>,
    read_within: Duration,
    ended: mpsc::Sender<Result<auth::Secret, String>>,
) {
    let Ok(Some(req)) = tokio::time::timeout(read_within, mockggg::read_request(&mut stream)).await
    else {
        return;
    };
    if req.method != "GET" || req.path != CALLBACK_PATH {
        mockggg::respond(&mut stream, "404 Not Found", "text/plain", "not found").await;
        return;
    }
    if req.query.get("state").map(String::as_str) != Some(&*expected_state) {
        mockggg::respond(
            &mut stream,
            "400 Bad Request",
            "text/html",
            "<h1>Not the login in progress</h1><p>This request does not carry the state of the \
             login Acquisition is waiting for. Close this tab and return to the terminal, where \
             the login is still waiting.</p>",
        )
        .await;
        return;
    }
    match (req.query.get("code"), req.query.get("error")) {
        (Some(code), _) => {
            mockggg::respond(
                &mut stream,
                "200 OK",
                "text/html",
                "<h1>Logged in</h1><p>You can close this tab and return to the terminal.</p>",
            )
            .await;
            let _ = ended.try_send(Ok(code.as_str().into()));
        }
        (None, Some(error)) => {
            mockggg::respond(
                &mut stream,
                "200 OK",
                "text/html",
                "<h1>Login denied</h1><p>The login was denied in the browser. You can close this \
                 tab; to try again, run <code>acq auth</code> from the terminal.</p>",
            )
            .await;
            // #113: the redirect's query is text the daemon received, and
            // these words reach the log: its `error` as one of OAuth's
            // codes, and never its free-text `error_description`.
            let _ = ended.try_send(Err(format!(
                "the login was denied in the browser ({}); run `acq auth` to try again",
                auth::oauth_error_words(error)
            )));
        }
        (None, None) => {
            mockggg::respond(
                &mut stream,
                "400 Bad Request",
                "text/html",
                "<h1>Not a login answer</h1><p>This request carries neither a code nor an error. \
                 Close this tab and return to the terminal, where the login is still waiting.</p>",
            )
            .await;
        }
    }
}
