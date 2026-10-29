//! The single-flight token refresh and the dead bearer: tokens installed, a
//! rejected access token forgotten, and the refresh every waiter shares.
//! C148: a refresh lands on a blocking thread, its tokens installed and
//! published under `shared`, its keyring save and index writes under
//! `credentials` alone, its keyring mark written under `shared` after.
//! C135: what a landing saves is one keyring entry, the access token and
//! its expiry beside the refresh token (`auth::KeyringEntry`), so a start
//! restores a session that refreshes only within the refresh margin of
//! that token's expiry.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::*;

pub(super) type AccessTokenResult = Result<(String, String), Refusal>;

pub(super) const SESSION_CHANGED_DURING_REFRESH: &str =
    "authentication session changed while token refresh was in progress";
pub(super) const REFRESH_OWNER_ABANDONED: &str =
    "token refresh owner was abandoned before producing a result";
pub(super) const REFRESH_CALLED_OFF: &str =
    "token refresh called off before it was sent: the job that asked for it was cancelled";

struct RefreshOwnerGuard<'a> {
    daemon: &'a Arc<Daemon>,
    account: String,
    id: u64,
    generations: AuthGenerations,
    result: Option<watch::Sender<Option<AccessTokenResult>>>,
}

impl RefreshOwnerGuard<'_> {
    /// Hand the flight's sender to its landing. A refresh that failed
    /// lands where it is: it writes nothing but the session table. One
    /// that brought tokens lands on a blocking thread (C148), which runs
    /// to its end whether or not this owner is still awaiting it, so from
    /// here the guard has nothing to publish on drop: the landing does.
    async fn finish(mut self, refresh: Result<auth::TokenResponse, Refusal>) -> AccessTokenResult {
        let sender = self.result.take().expect("refresh owner result exists");
        let (account, id, generations) = (self.account.clone(), self.id, self.generations);
        if refresh.is_err() {
            return self
                .daemon
                .finish_refresh(&account, id, generations, refresh, &sender);
        }
        self.daemon
            .spawn_landing(move |daemon| {
                daemon.finish_refresh(&account, id, generations, refresh, &sender)
            })
            .await
            .unwrap_or_else(|_| Err(Refusal::new(ErrorKind::Internal, REFRESH_OWNER_ABANDONED)))
    }
}

impl Drop for RefreshOwnerGuard<'_> {
    fn drop(&mut self) {
        let Some(result) = self.result.take() else {
            return;
        };
        if result.borrow().is_some() {
            return;
        }
        {
            let mut s = self.daemon.shared.lock().unwrap();
            if let Some(session) = s.auth.find_mut(&self.account) {
                let owns_current_flight = session.generations == self.generations
                    && session.refresh_flight.as_ref().is_some_and(|flight| {
                        flight.id == self.id && flight.generations == self.generations
                    });
                if owns_current_flight {
                    session.refresh_flight = None;
                }
            }
        }
        result.send_replace(Some(Err(Refusal::new(
            ErrorKind::Internal,
            REFRESH_OWNER_ABANDONED,
        ))));
    }
}

impl Daemon {
    /// Install fresh tokens in a session, under the caller's hold of
    /// `shared`: the access token and its expiry, the refresh token the
    /// provider rotated, the username, the generations advanced. Nothing
    /// is persisted here (C148): the landing saves the session's tokens
    /// under `credentials` alone ([`Self::persist_credentials`]).
    pub(super) fn install_tokens_locked(
        &self,
        session: &mut AuthSession,
        tokens: auth::TokenResponse,
    ) {
        session.access_token = Some(tokens.access_token);
        // C47: `expires_in` is bounded where the response is read
        // (`auth::MAX_ACCESS_TOKEN_LIFETIME_SECS`), so the sum is safe.
        session.access_expires_at =
            Some(self.choke.wall() + Duration::from_secs(tokens.expires_in));
        session.refresh_token = Some(tokens.refresh_token);
        session.username = Some(tokens.username);
        session.advance_access_token();
        session.advance_refresh_token();
    }

    /// C144 — the one persist of a keyring entry, login's and refresh's
    /// alike: C135, the refresh token with the access token and its expiry
    /// beside it, written as the entry's secret here and read back in
    /// `start::restore_sessions` alone. Takes the caller's hold of
    /// `credentials` (`held.rs`: the value only the lock gives, so a
    /// persist outside it does not compile) and never runs
    /// under `shared` (C148), so the keyring and the index land together,
    /// ordered with logout and forget. The
    /// index holds `username` before the token is saved under it
    /// ([`Index::hold_keyring_name`]): an index that cannot be written
    /// then means no save, the keyring status and a warning say so, and
    /// no credential exists that the index does not know, however many
    /// renames follow. The token is saved under `username`; a failed save clears the entry it
    /// would have replaced, which holds a token the grant rotated away or
    /// the previous login's. The account index then records the login at
    /// `at` (by the uuid when there is one, so a rename moves the entry)
    /// and, in that same write, every name a keyring entry for the
    /// account may now be under (`AccountEntry::keyring_names`): the name
    /// just saved, the session's old name (`old_name`, a refresh that
    /// renamed), the name the index mapped the uuid to (a re-login under
    /// a renamed username), and every name the index already held. Only
    /// then is each old name cleared, and a name leaves the index only
    /// once its clear succeeds: one whose clear fails stays held, however
    /// many renames follow, so logout and forget still reach it. An index
    /// that cannot be written leaves the old names alone: the index still
    /// reaches them. Every clear that fails is a warning.
    pub(super) fn persist_credentials(
        &self,
        credentials: &held::CredentialsHeld<'_>,
        username: &str,
        entry: &auth::KeyringEntry,
        uuid: Option<&str>,
        old_name: Option<&str>,
        at: i64,
    ) -> Persisted {
        let service = self.provider.keyring_service;
        let mut old_names: Vec<String> = old_name.map(str::to_string).into_iter().collect();
        // An index that cannot be read fails the write below, which says so.
        if let Some(dir) = &self.store_dir
            && let Ok(index) = Index::load(dir)
        {
            let known = index.entries().iter().filter(|e| match uuid {
                Some(uuid) => e.uuid.as_deref() == Some(uuid),
                None => e.username == username || Some(e.username.as_str()) == old_name,
            });
            for entry in known {
                for name in entry.keyring_reach() {
                    if !old_names.contains(&name) {
                        old_names.push(name);
                    }
                }
            }
        }
        old_names.retain(|old| !old.is_empty() && old != username);
        // The index knows the name before the keyring does: an index that
        // cannot hold it means no save, so no credential lands where no
        // verb can find it. The session keeps its tokens in memory.
        if let Err(e) = self.index_apply(credentials, |index| {
            index.hold_keyring_name(uuid, username, old_name, at)
        }) {
            return Persisted {
                keyring: format!("unavailable: not saved, the account index could not hold its name: {e}"),
                warnings: vec![
                    "keyring save skipped: the account index could not hold the name first (session is in-memory only)"
                        .to_string(),
                ],
                indexed: Err(e),
            };
        }
        let saved = self
            .credential_store
            .save(service, &entry.to_secret(), username);
        let mut warnings = Vec::new();
        let mut released = Vec::new();
        let keyring = match &saved {
            Ok(()) => "ok".to_string(),
            Err(e) => {
                warnings.push(format!(
                    "keyring save failed: {e} (session is in-memory only)"
                ));
                // A keyring turned off fails the save too, and its warning
                // says so; only a clear the keyring refused adds one.
                match self.credential_store.clear(service, username) {
                    Ok(()) => released.push(username.to_string()),
                    Err(auth::NotCleared::Failed(clear)) => warnings.push(format!(
                        "the entry the save would have replaced, for {username}, could not be cleared: {clear}"
                    )),
                    Err(auth::NotCleared::Off(_)) => {}
                }
                format!("unavailable: {e}")
            }
        };
        // Persisted is the save's result alone: a clear that fails beside
        // a good save warns, and the entry the index points at is the one
        // just saved.
        let persisted = saved.is_ok();
        let mut hold = old_names.clone();
        if persisted {
            hold.push(username.to_string());
        }
        let indexed = self.index_apply(credentials, |index| match uuid {
            Some(uuid) => index.record_login(username, uuid, persisted, at, &hold),
            None => index.upsert(username, persisted, at, &hold),
        });
        if indexed.is_ok() {
            for old in &old_names {
                match self.credential_store.clear(service, old) {
                    Ok(()) => released.push(old.clone()),
                    Err(auth::NotCleared::Failed(clear)) => warnings.push(format!(
                        "the keyring entry under the old name {old} could not be cleared: {clear}"
                    )),
                    Err(auth::NotCleared::Off(_)) => {}
                }
            }
            if !released.is_empty()
                && let Err(e) = self.index_apply(credentials, |index| {
                    index.settle_keyring_names(uuid, username, &[], &released, None)
                })
            {
                warnings.push(e);
            }
        }
        Persisted {
            keyring,
            warnings,
            indexed,
        }
    }

    /// A 401 landed on a send that carried `token` (C87): the session
    /// forgets it and the log says so. Reached from the choke point through
    /// `Bearer::rejected`, while the rejecting send still holds its permit,
    /// so every send admitted after it reads the cleared session. The job
    /// that met the 401 fails as it did; the next send for that account is
    /// the refresh (`TESTING-NOTES.md`, the C87 scenarios). Returns the
    /// account whose token was cleared and the session's generations then,
    /// for the job to land the clear in the keyring entry
    /// ([`Self::land_rejection`]), or `None` when no session held it any
    /// more.
    pub(super) fn reject_access_token(&self, token: &str) -> Option<(String, AuthGenerations)> {
        let (username, generations) = {
            let mut s = self.shared.lock().unwrap();
            let username = s.auth.reject_access_token(token)?;
            let generations = s.auth.find(&username)?.generations;
            (username, generations)
        };
        self.log(&format!(
            "access token for {username} rejected (401); forgotten — the next send refreshes first"
        ));
        Some((username, generations))
    }

    /// C135 with C87 — the keyring entry holds what the session holds,
    /// so a rejected access token leaves the entry as it leaves the
    /// session, and no restart restores a bearer GGG already rejected.
    /// The job whose send met the 401 lands it, after the send, as a
    /// landing (C148: under `credentials`, on a blocking thread, counted
    /// in `landings` for the exit's drain): the entry is rewritten with
    /// the refresh token alone through the one persist, only while the
    /// session's generations are those of the clear (a refresh, a
    /// re-login or a logout in the gap landed its own entry, or none) and
    /// only for a session whose keyring holds its entry (`ok`). One more
    /// keyring write, on a 401 alone, which GGG answers only to a revoked
    /// token.
    pub(super) async fn land_rejection(
        self: &Arc<Self>,
        username: &str,
        generations: AuthGenerations,
    ) {
        let username = username.to_string();
        // A landing that panics aborts the daemon (C134); there is no
        // other way for it to end without returning.
        let _ = self
            .spawn_landing(move |daemon| daemon.persist_rejection(&username, generations))
            .await;
    }

    /// [`Self::land_rejection`]'s body, on the landing's thread.
    ///
    /// C135 with C87, #110: the write is a narrowing, saved without the account index.
    /// The index-first hold of [`Self::persist_credentials`] guards a
    /// name: no credential lands under a name no verb can find. This
    /// rewrite adds none: it saves under the session's own name, which
    /// its `ok` keyring already holds, the refresh token the entry held
    /// and no access token. So an index that cannot be read at the 401
    /// does not keep the rejected bearer for the next lifetime. A save
    /// that fails leaves the entry and warns: its refresh token is still
    /// the live grant, and clearing it would trade one 401 after a
    /// restart for a login.
    fn persist_rejection(&self, username: &str, generations: AuthGenerations) {
        let credentials = self.hold_credentials();
        let entry = {
            let s = self.shared.lock().unwrap();
            let Some(session) = s.auth.find(username) else {
                return;
            };
            if session.generations != generations
                || session.keyring != "ok"
                || session.access_token.is_some()
            {
                return;
            }
            let Some(entry) = session.keyring_entry() else {
                return;
            };
            entry
        };
        let saved =
            self.credential_store
                .save(self.provider.keyring_service, &entry.to_secret(), username);
        if let Err(e) = &saved {
            let mut s = self.shared.lock().unwrap();
            if let Some(session) = s.auth.find_mut(username)
                && session.generations == generations
            {
                session.keyring = format!("unavailable: {e}");
            }
        }
        drop(credentials);
        if let Err(e) = saved {
            self.note_error(&format!(
                "keyring save failed: {e}: the entry for {username} still holds the access token GGG rejected, so a restart before the next refresh sends it once more"
            ));
        }
    }

    /// Current access token, refreshing through the provider if it is
    /// expired (or about to be). Jobs call this; clients never see tokens.
    /// `force_refresh` skips the cached token so the provider round-trip is
    /// guaranteed — that's what makes `auth check` an actual proof.
    /// `account` is the job's account (fixed at submit): the token handed
    /// back is that account's session's, looked up at the moment the token
    /// is taken — a logout of A after A's job was submitted fails A's job
    /// here, before any send ("no session for A"). `None` is the sole
    /// session (auth verbs), refused when several are live.
    ///
    /// `party` is the job that asks, or `None` for a caller that is no
    /// job (C25). The flight's owner carries its POST to the choke point
    /// with its own job's cancel check, so a cancel of the owner turns the
    /// POST away unsent and the flight ends `cancelled`. A refresh is
    /// shared, and a cancel of one job never cancels it for another: every
    /// waiter on a called-off flight that still wants the token asks
    /// again, the first as the owner of a fresh flight. No send is added;
    /// the one POST the flight would have made is made by the new owner.
    pub(super) async fn valid_access_token(
        self: &Arc<Self>,
        account: Option<&str>,
        force_refresh: bool,
        party: Option<JobId>,
    ) -> Result<(String, String), Refusal> {
        loop {
            let result = self
                .valid_access_token_once(account, force_refresh, party)
                .await;
            match &result {
                Err(refusal)
                    if refusal.cancelled && party.is_none_or(|job| !self.cancel_asked(job)) => {}
                _ => return result,
            }
        }
    }

    async fn valid_access_token_once(
        self: &Arc<Self>,
        account: Option<&str>,
        force_refresh: bool,
        party: Option<JobId>,
    ) -> Result<(String, String), Refusal> {
        enum Decision {
            Owner {
                id: u64,
                generations: AuthGenerations,
                refresh_token: String,
                result: watch::Sender<Option<AccessTokenResult>>,
            },
            Waiter(watch::Receiver<Option<AccessTokenResult>>),
        }

        let (username, decision) = {
            let mut s = self.shared.lock().unwrap();
            let wall = self.choke.wall();
            let session = s.auth.get_mut(account)?;
            let username = session.username.clone().unwrap_or_default();
            if !force_refresh
                && let (Some(token), Some(expires)) =
                    (&session.access_token, session.access_expires_at)
                && expires
                    .duration_since(wall)
                    .is_ok_and(|left| left > Duration::from_secs(5))
            {
                return Ok((token.clone(), username));
            }
            let refresh_token = match &session.refresh_token {
                Some(rt) => rt.clone(),
                None => {
                    return Err(Refusal::new(
                        ErrorKind::NotLoggedIn,
                        format!("no refresh token for {username} — run `acq auth`"),
                    ));
                }
            };
            // A grant the provider already rejected is terminal (C24):
            // it is not sent again until login or logout replaces it.
            if let Some(cause) = self.rails().refresh_failed(&username) {
                return Err(Refusal::new(
                    ErrorKind::NotLoggedIn,
                    format!("token refresh disabled for {username}: {cause}; run `acq auth`"),
                ));
            }
            let generations = session.generations;
            let decision = if let Some(flight) = &session.refresh_flight
                && flight.generations == generations
            {
                Decision::Waiter(flight.result.subscribe())
            } else {
                let id = session.next_refresh_flight;
                session.next_refresh_flight = session.next_refresh_flight.wrapping_add(1);
                let (result, _) = watch::channel(None);
                session.refresh_flight = Some(RefreshFlight {
                    id,
                    generations,
                    result: result.clone(),
                });
                Decision::Owner {
                    id,
                    generations,
                    refresh_token,
                    result,
                }
            };
            (username, decision)
        };
        match decision {
            Decision::Waiter(result) => wait_for_refresh(result).await,
            Decision::Owner {
                id,
                generations,
                refresh_token,
                result,
            } => {
                let owner = RefreshOwnerGuard {
                    daemon: self,
                    account: username.clone(),
                    id,
                    generations,
                    result: Some(result),
                };
                // May wait on the token endpoint's limiter; the shared lock is
                // not held here, so every concurrent caller can join this owner.
                let wanted = || party.is_none_or(|job| !self.cancel_asked(job));
                let refresh =
                    auth::refresh(&self.choke, &self.provider, &refresh_token, &wanted).await;
                self.announce_trip();
                // L0 rail 2: mark a rejected grant before the flight is
                // closed, so no caller can pass the fast path and re-send the
                // dead token in between; and only while this flight is still
                // the current one, so a stale flight cannot disable a session
                // that re-authenticated meanwhile. The persisted cause is the
                // status plus a fixed reason, never the response body.
                if let Some(error) = refresh.as_ref().err().filter(|e| e.is_rejected_grant()) {
                    let current = {
                        let s = self.shared.lock().unwrap();
                        s.auth.find(&username).is_some_and(|session| {
                            session.generations == generations
                                && session.refresh_flight.as_ref().is_some_and(|f| f.id == id)
                        })
                    };
                    let cause = format!(
                        "refresh token rejected with HTTP {}",
                        error.status.unwrap_or_default()
                    );
                    if current && self.rails().mark_refresh_failed(&username, &cause) {
                        self.note_error(&format!(
                            "AUTH: {username}: {cause}; further refreshes disabled until `acq auth` ({error})"
                        ));
                    }
                    // The mark's write, if it failed, is logged.
                    self.announce_trip();
                }
                // A rejected grant is a dead session (`not_logged_in`: the
                // remedy is login); anything else — transport, a 5xx,
                // exhausted 429s, a rails halt, a Cloudflare block (C129:
                // no rejected grant, and no dead-grant mark above) — is the
                // provider not answering (`upstream`), with the session
                // still standing. A halt's refusal, or the block that set
                // one, carries the mark that turns a job away (C25): the
                // job and every waiter on this flight return to waiting.
                let refresh = refresh.map_err(|e| {
                    if e.cancelled {
                        return Refusal {
                            cancelled: true,
                            ..Refusal::new(ErrorKind::Upstream, REFRESH_CALLED_OFF)
                        };
                    }
                    let kind = if e.is_rejected_grant() {
                        ErrorKind::NotLoggedIn
                    } else {
                        ErrorKind::Upstream
                    };
                    Refusal {
                        halted: e.met_a_halt(),
                        ..Refusal::new(kind, format!("token refresh failed: {e}"))
                    }
                });
                owner.finish(refresh).await
            }
        }
    }

    /// The refresh's landing (C144, C148). A refresh that failed only
    /// closes its flight. One that brought tokens holds `credentials`
    /// throughout, which orders it with logout, forget and login, and
    /// takes `shared` twice, never across the I/O: first it checks its
    /// flight is current, installs the tokens, follows a rename and marks
    /// the session's keyring `saving`, closes its flight and publishes the
    /// token to the flight's waiters, so a waiter and a bearer read are
    /// served at the install; then the keyring and the index
    /// ([`Self::persist_credentials`]); then the keyring mark. The exit's
    /// drain waits for it as for every landing (`spawn_landing`). A
    /// caller that finds no valid token in the gap starts a refresh of
    /// its own, whose landing waits for this one. A logout lands wholly before it (the refresh is
    /// stale) or wholly after (its not-persisted mark stands). It blocks
    /// when it brings tokens: the owner runs it on a blocking thread.
    pub(super) fn finish_refresh(
        &self,
        account: &str,
        id: u64,
        generations: AuthGenerations,
        refresh: Result<auth::TokenResponse, Refusal>,
        sender: &watch::Sender<Option<AccessTokenResult>>,
    ) -> AccessTokenResult {
        // Held only for a refresh that succeeded: the landing below is
        // only then, and it takes this hold.
        let credentials = refresh.is_ok().then(|| self.hold_credentials());
        let mut renamed = None;
        let mut landing = None;
        let mut installed = None;
        let outcome = {
            let mut s = self.shared.lock().unwrap();
            let Some(session) = s.auth.find_mut(account) else {
                return finish_stale(sender);
            };
            let owns_current_flight = session
                .refresh_flight
                .as_ref()
                .is_some_and(|flight| flight.id == id && flight.generations == generations);
            if !owns_current_flight || session.generations != generations {
                Err(session_changed())
            } else {
                match refresh {
                    Ok(tokens) => {
                        // GGG returns the same account on a refresh; if the
                        // name ever differs (a rename), the session follows
                        // it — lookups go by the session's username — and
                        // the index follows the session's uuid
                        // (`record_login`) instead of minting a uuid-less
                        // twin entry; a pre-uuid session falls back to
                        // plain bookkeeping.
                        if tokens.username != account {
                            renamed = Some(tokens.username.clone());
                        }
                        let result = (tokens.access_token.clone(), tokens.username.clone());
                        let username = tokens.username.clone();
                        let old_name = session
                            .username
                            .clone()
                            .filter(|old| !old.is_empty() && *old != tokens.username);
                        self.install_tokens_locked(session, tokens);
                        // C135: what is saved is what the session holds,
                        // the access token and its expiry beside the
                        // refresh token, taken at the install.
                        landing = session.keyring_entry().map(|entry| Landing {
                            username,
                            entry,
                            uuid: session.uuid.clone(),
                            old_name,
                        });
                        // C144: `auth status` says no `ok` for a save that
                        // has not landed.
                        session.keyring = "saving".into();
                        // The flight closes at the install: its published
                        // token serves only the waiters it already had,
                        // never one that comes after the token may have
                        // left the session (a 401, C87). The save in flight
                        // is the drain's through `landings`.
                        session.refresh_flight = None;
                        installed = Some(session.generations);
                        if let Some(new_name) = &renamed {
                            s.auth.rename(account, new_name);
                        }
                        Ok(result)
                    }
                    Err(error) => {
                        session.refresh_flight = None;
                        Err(error)
                    }
                }
            }
        };
        sender.send_replace(Some(outcome.clone()));
        let mut warning = None;
        if let (Some(landing), Some(credentials)) = (landing, credentials.as_ref()) {
            let persisted = self.persist_credentials(
                credentials,
                &landing.username,
                &landing.entry,
                landing.uuid.as_deref(),
                landing.old_name.as_deref(),
                self.unix_now(),
            );
            let mut warnings = persisted.warnings;
            if let Err(e) = persisted.indexed {
                warnings.push(e);
            }
            warning = (!warnings.is_empty()).then(|| warnings.join("; "));
            let mut s = self.shared.lock().unwrap();
            // Only the session this landing installed, by its generations:
            // no other landing ran meanwhile (`credentials`), so it is
            // still there, and a refresh that began in the gap installs
            // only after this landing releases.
            if let Some(session) = s.auth.find_mut(&landing.username)
                && Some(session.generations) == installed
            {
                session.keyring = persisted.keyring;
            }
        }
        drop(credentials);
        #[cfg(test)]
        self.refresh_landed();
        if outcome.is_ok() {
            self.log(&format!("access token refreshed for {account}"));
        }
        if let Some(new_name) = renamed {
            self.note_error(&format!(
                "AUTH: refresh for {account} returned username {new_name}; session follows the new name"
            ));
        }
        if let Some(warning) = warning {
            self.note_error(&warning);
        }
        outcome
    }

    /// The test's pause where a refresh's landing releases `credentials`,
    /// after its mark (C148): the review of h1 staged a logout here.
    #[cfg(test)]
    fn refresh_landed(&self) {
        let hook = self.refresh_landed_hook.lock().unwrap().take();
        if let Some(hook) = hook {
            hook();
        }
    }
}

/// What a refresh's landing persists, taken from the session at its
/// install.
struct Landing {
    username: String,
    entry: auth::KeyringEntry,
    uuid: Option<String>,
    old_name: Option<String>,
}

/// What [`Daemon::persist_credentials`] did: the session's keyring
/// status (`ok` or `unavailable: …`), every warning beside it, and the
/// index write's result.
pub(super) struct Persisted {
    pub(super) keyring: String,
    pub(super) warnings: Vec<String>,
    pub(super) indexed: Result<(), String>,
}

/// A refresh whose session vanished (logged out) while it was in flight.
/// The session a refresh was for is gone or replaced: whoever waited on
/// it has no session to use — `not_logged_in`.
fn session_changed() -> Refusal {
    Refusal::new(ErrorKind::NotLoggedIn, SESSION_CHANGED_DURING_REFRESH)
}

fn finish_stale(sender: &watch::Sender<Option<AccessTokenResult>>) -> AccessTokenResult {
    let outcome: AccessTokenResult = Err(session_changed());
    sender.send_replace(Some(outcome.clone()));
    outcome
}

async fn wait_for_refresh(
    mut result: watch::Receiver<Option<AccessTokenResult>>,
) -> AccessTokenResult {
    loop {
        if let Some(outcome) = result.borrow().clone() {
            return outcome;
        }
        if result.changed().await.is_err() {
            return Err(Refusal::new(ErrorKind::Internal, REFRESH_OWNER_ABANDONED));
        }
    }
}
