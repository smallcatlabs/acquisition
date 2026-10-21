//! The single-flight token refresh and the dead bearer: tokens installed, a
//! rejected access token forgotten, and the refresh every waiter shares.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::*;

pub(super) type AccessTokenResult = Result<(String, String), Refusal>;

pub(super) const SESSION_CHANGED_DURING_REFRESH: &str =
    "authentication session changed while token refresh was in progress";
pub(super) const REFRESH_OWNER_ABANDONED: &str =
    "token refresh owner was abandoned before producing a result";

struct RefreshOwnerGuard<'a> {
    daemon: &'a Daemon,
    account: String,
    id: u64,
    generations: AuthGenerations,
    result: Option<watch::Sender<Option<AccessTokenResult>>>,
}

impl RefreshOwnerGuard<'_> {
    fn finish(mut self, refresh: Result<auth::TokenResponse, Refusal>) -> AccessTokenResult {
        let result = self.result.as_ref().expect("refresh owner result exists");
        let outcome =
            self.daemon
                .finish_refresh(&self.account, self.id, self.generations, refresh, result);
        self.result = None;
        outcome
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
    /// Store fresh tokens in memory and mirror the refresh token (which the
    /// provider rotates on every grant) into the keyring. The caller holds
    /// the auth-state lock so keyring mutation is ordered with logout and
    /// re-authentication — and updates the account index *after* releasing
    /// it (`with_index` reports errors through `note_error`, which takes
    /// the same lock).
    pub(super) fn install_tokens_locked(
        &self,
        session: &mut AuthSession,
        tokens: auth::TokenResponse,
    ) -> Option<String> {
        let (keyring, warning) = match self.credential_store.save(
            self.provider.keyring_service,
            &tokens.refresh_token,
            &tokens.username,
        ) {
            Ok(()) => ("ok".to_string(), None),
            Err(e) => {
                let warning = format!("keyring save failed: {e} (session is in-memory only)");
                (format!("unavailable: {e}"), Some(warning))
            }
        };
        session.access_token = Some(tokens.access_token);
        session.access_expires_at =
            Some(self.choke.wall() + Duration::from_secs(tokens.expires_in));
        session.refresh_token = Some(tokens.refresh_token);
        session.username = Some(tokens.username);
        session.keyring = keyring;
        session.advance_access_token();
        session.advance_refresh_token();
        session.refresh_flight = None;
        warning
    }

    /// A 401 landed on a send that carried `token` (C87): the session
    /// forgets it and the log says so. Reached from the choke point through
    /// `Bearer::rejected`, while the rejecting send still holds its permit,
    /// so every send admitted after it reads the cleared session. The job
    /// that met the 401 fails as it did; the next send for that account is
    /// the refresh (`TESTING-NOTES.md`, the C87 scenarios). Returns the
    /// account whose token was cleared, or `None` when no session held it
    /// any more.
    pub(super) fn reject_access_token(&self, token: &str) -> Option<String> {
        let username = self
            .shared
            .lock()
            .unwrap()
            .auth
            .reject_access_token(token)?;
        self.log(&format!(
            "access token for {username} rejected (401); forgotten — the next send refreshes first"
        ));
        Some(username)
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
    pub(super) async fn valid_access_token(
        &self,
        account: Option<&str>,
        force_refresh: bool,
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
                let refresh = auth::refresh(&self.choke, &self.provider, &refresh_token).await;
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
                }
                // A rejected grant is a dead session (`not_logged_in`: the
                // remedy is login); anything else — transport, a 5xx,
                // exhausted 429s, a rails halt — is the provider not
                // answering (`upstream`), with the session still standing.
                let refresh = refresh.map_err(|e| {
                    let kind = if e.is_rejected_grant() {
                        ErrorKind::NotLoggedIn
                    } else {
                        ErrorKind::Upstream
                    };
                    Refusal::new(kind, format!("token refresh failed: {e}"))
                });
                owner.finish(refresh)
            }
        }
    }

    pub(super) fn finish_refresh(
        &self,
        account: &str,
        id: u64,
        generations: AuthGenerations,
        refresh: Result<auth::TokenResponse, Refusal>,
        sender: &watch::Sender<Option<AccessTokenResult>>,
    ) -> AccessTokenResult {
        let mut warning = None;
        let mut renamed = None;
        let mut index_update = None;
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
                session.refresh_flight = None;
                match refresh {
                    Ok(tokens) => {
                        // GGG returns the same account on a refresh; if the
                        // name ever differs (a rename), the session follows
                        // it — lookups go by the session's username.
                        if tokens.username != account {
                            renamed = Some(tokens.username.clone());
                        }
                        let result = (tokens.access_token.clone(), tokens.username.clone());
                        let username = tokens.username.clone();
                        warning = self.install_tokens_locked(session, tokens);
                        index_update = Some((username, session.uuid.clone(), warning.is_none()));
                        Ok(result)
                    }
                    Err(error) => Err(error),
                }
            }
        };
        // With the session's uuid, a refresh that renamed the account
        // updates the existing index mapping in place (`record_login`
        // follows the uuid) instead of minting a uuid-less twin entry; a
        // pre-uuid session falls back to plain bookkeeping.
        if let Some((username, uuid, persisted)) = index_update {
            self.with_index(|index| match &uuid {
                Some(uuid) => index.record_login(&username, uuid, persisted, self.unix_now()),
                None => index.upsert(&username, persisted, self.unix_now()),
            });
        }
        if let Some(new_name) = &renamed {
            self.shared.lock().unwrap().auth.rename(account, new_name);
        }
        sender.send_replace(Some(outcome.clone()));
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
