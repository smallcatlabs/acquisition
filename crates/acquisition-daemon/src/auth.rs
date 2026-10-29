//! OAuth client mechanics: PKCE, token exchange/refresh, keyring storage.
//!
//! The flow is authorization code + PKCE with a loopback redirect, refresh
//! rotation, and OS keyring storage. Which provider it points at — the
//! localhost mock or real GGG — is entirely the `Provider` passed in.

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use rand::RngCore as _;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{Duration, SystemTime};

use crate::provider::{Provider, SCOPES};
use crate::ratelimit::{ChokePoint, SendError, Wanted};

pub fn random_token(prefix: &str) -> String {
    let mut bytes = [0u8; 24];
    rand::rng().fill_bytes(&mut bytes);
    format!("{prefix}-{}", URL_SAFE_NO_PAD.encode(bytes))
}

/// S256 code-challenge transform.
pub fn s256(verifier: &str) -> String {
    URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()))
}

/// Returns (verifier, challenge): the verifier is the login's secret, the
/// challenge its public digest.
pub fn pkce_pair() -> (Secret, String) {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    let verifier = URL_SAFE_NO_PAD.encode(bytes);
    let challenge = s256(&verifier);
    (Secret(verifier), challenge)
}

/// #113, a log safe by construction: a credential the daemon holds — an
/// access or a refresh token, the keyring's secret that holds them, a
/// login's code or its PKCE verifier — from where it enters the daemon
/// (a token response's body, a keyring read, the login's callback, the
/// PKCE pair) to where it leaves (the bearer header, the token POST's
/// form, a keyring write). All of those are built in this module, the
/// only place its text is read (`Secret::expose` is private to it).
/// It formats as a placeholder under `Debug` and `Display`, and has no
/// `Serialize`, `Deref` or `AsRef<str>`; so no string the rest of the
/// daemon builds, for its log or anything else, can hold a token, and
/// the log's one writer (`daemon::write_log_line`) needs no rule of its
/// own about them.
#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct Secret(String);

impl Secret {
    /// The text: read here alone, where a header, a form or a keyring
    /// write is built from it.
    fn expose(&self) -> &str {
        &self.0
    }

    /// `request` carrying this token as its bearer: reqwest's own header,
    /// marked sensitive.
    pub fn authorize(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request.bearer_auth(self.expose())
    }

    /// The text, for a test that asserts on what the wire or the keyring
    /// carried.
    #[cfg(test)]
    pub fn exposed_for_test(&self) -> &str {
        self.expose()
    }
}

impl From<String> for Secret {
    fn from(text: String) -> Self {
        Secret(text)
    }
}

impl From<&str> for Secret {
    fn from(text: &str) -> Self {
        Secret(text.to_string())
    }
}

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<redacted>")
    }
}

impl std::fmt::Display for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<redacted>")
    }
}

/// A test compares a secret with the text it expects.
#[cfg(test)]
impl PartialEq<&str> for Secret {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

/// A failed token request, keeping the HTTP status (when one landed) so
/// the daemon can tell a rejected grant from a network blip (L0 rail 2),
/// and the choke point's verdict that Cloudflare answered in GGG's place
/// (C129), which no status alone can tell.
#[derive(Debug, Clone)]
pub struct TokenError {
    pub status: Option<u16>,
    /// C129: the answer was a block, Cloudflare's and not GGG's. Its
    /// status says nothing about the grant.
    pub blocked: bool,
    /// C25: the choke point refused the request under a halt; nothing
    /// was sent.
    pub halted: bool,
    /// C25: the request's own check said it was no longer wanted (a
    /// refresh's is its flight owner's job, cancelled) when the choke
    /// point would have admitted it; nothing was sent. Another job waiting
    /// on that refresh is not cancelled by it, and asks again.
    pub cancelled: bool,
    pub message: String,
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl TokenError {
    fn transport(message: String) -> TokenError {
        TokenError {
            status: None,
            blocked: false,
            halted: false,
            cancelled: false,
            message,
        }
    }

    /// C25: a halt refused the request, or the answer was Cloudflare's and
    /// set one (a block, or an origin it could not reach). The job that
    /// asked waits for the reset.
    pub fn met_a_halt(&self) -> bool {
        self.halted || self.blocked
    }

    /// A grant the provider rejected outright: 4xx other than 429 that is
    /// not a block (C24). 5xx, 429, and transport failures are not (they
    /// may be transient); a block is not, for GGG never saw the grant
    /// (C129): marking it dead would send the user to `acq auth`, whose
    /// code exchange goes into the same block.
    pub fn is_rejected_grant(&self) -> bool {
        !self.blocked
            && self
                .status
                .is_some_and(|status| (400..500).contains(&status) && status != 429)
    }
}

/// Holds both tokens as [`Secret`]s, so its `Debug` names neither (C144:
/// no secret is formatted where a log or a panic could carry it).
#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: Secret,
    pub refresh_token: Secret,
    /// Seconds, at most [`MAX_ACCESS_TOKEN_LIFETIME_SECS`] in any response
    /// [`exchange_code`] or [`refresh`] returns, so the daemon may add it
    /// to its wall clock unchecked.
    pub expires_in: u64,
    pub username: String,
}

impl std::fmt::Debug for TokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenResponse")
            .field("access_token", &"<redacted>")
            .field("refresh_token", &"<redacted>")
            .field("expires_in", &self.expires_in)
            .field("username", &self.username)
            .finish()
    }
}

/// C47: the longest access-token lifetime a token response is taken to
/// carry. GGG's is ten hours (ground truth N33); a year is far past any
/// grant and far inside every platform's `SystemTime`, so the sum the
/// daemon makes of it cannot overflow. A larger value is the provider's
/// malformed input, refused like a body that is no token response.
pub const MAX_ACCESS_TOKEN_LIFETIME_SECS: u64 = 366 * 24 * 60 * 60;

/// `wanted` is the login flow's check (C25): the POST is turned away
/// before it is sent once a newer login has superseded the flow.
pub async fn exchange_code(
    choke: &ChokePoint,
    provider: &Provider,
    code: &Secret,
    verifier: &Secret,
    redirect_uri: &str,
    wanted: Wanted<'_>,
) -> Result<TokenResponse, TokenError> {
    // GGG's public-client docs include `scope` in the exchange; no secret is
    // ever sent (and must not be — an empty client_secret is a server error).
    let scope = SCOPES.join(" ");
    token_request(
        choke,
        provider,
        &[
            ("grant_type", "authorization_code"),
            ("client_id", provider.client_id),
            ("code", code.expose()),
            ("code_verifier", verifier.expose()),
            ("redirect_uri", redirect_uri),
            ("scope", &scope),
        ],
        wanted,
    )
    .await
}

/// `wanted` is the refresh flight's check (C25): the POST is turned away
/// before it is sent once no one waiting on the flight still wants it.
pub async fn refresh(
    choke: &ChokePoint,
    provider: &Provider,
    refresh_token: &Secret,
    wanted: Wanted<'_>,
) -> Result<TokenResponse, TokenError> {
    token_request(
        choke,
        provider,
        &[
            ("grant_type", "refresh_token"),
            ("client_id", provider.client_id),
            ("refresh_token", refresh_token.expose()),
        ],
        wanted,
    )
    .await
}

async fn token_request(
    choke: &ChokePoint,
    provider: &Provider,
    params: &[(&str, &str)],
    wanted: Wanted<'_>,
) -> Result<TokenResponse, TokenError> {
    // Accountless on purpose: the token endpoint is `Ip`-scoped (N33) and
    // never probed, so every account's login shares this one known key.
    let response = choke
        .post_form(
            crate::ratelimit::TOKEN_ROUTE,
            &provider.token_url,
            params,
            choke.now(),
            wanted,
        )
        .await
        .map_err(|error| match error {
            SendError::Halted(_) => TokenError {
                halted: true,
                ..TokenError::transport(error.to_string())
            },
            SendError::Cancelled => TokenError {
                cancelled: true,
                ..TokenError::transport(error.to_string())
            },
            _ => TokenError::transport(error.to_string()),
        })?;
    let status = response.status;
    if response.verdict.is_some_and(|verdict| verdict.block) {
        // C129: the choke point has set the block mark. Cloudflare's page
        // is not GGG's word on the grant, so it is not shown; the words
        // say what stopped and what clears it.
        return Err(TokenError {
            status: Some(status.as_u16()),
            blocked: true,
            halted: false,
            cancelled: false,
            message: format!(
                "Cloudflare answered in GGG's place at the token endpoint ({status}), so the daemon has stopped sending: {}",
                crate::rails::BLOCK_REMEDY
            ),
        });
    }
    if response.verdict.is_some_and(|verdict| verdict.unreachable) {
        // C129: the choke point has halted this lifetime. Cloudflare's
        // page says nothing of GGG's, so it is not shown either; the words
        // are the halt's own, true of whatever else it holds (a tripwire's
        // trip beside it outlives a restart). The job that asked waits for
        // the reset, as under a block (C25). GGG never saw the request on
        // most of Cloudflare's statuses, but on a 520 or a 524 the origin
        // was reached and may have spent the grant: a refresh token
        // rotates on every grant (N33), so the next refresh after the reset
        // can meet a 400, which marks the grant dead and asks for one
        // login (C24); a login's one-use code may be spent the same way,
        // and the login is repeated. Nothing is installed from this answer.
        return Err(TokenError {
            status: Some(status.as_u16()),
            blocked: false,
            halted: true,
            cancelled: false,
            message: choke.rails().halted().unwrap_or_else(|| {
                format!(
                    "Cloudflare could not reach GGG at the token endpoint ({status}): {}",
                    crate::rails::UNREACHABLE_REMEDY
                )
            }),
        });
    }
    if !status.is_success() {
        // #113: the server's text never reaches the words, which go to
        // the log; only its status, its rate line as the limiter parses it
        // (`rate_words`) and the OAuth `error` code as one of OAuth's. A 3xx's body is no server text: the choke point put its
        // own words there, where the redirect pointed with its query
        // redacted (C130), and they stand. The choke point has already
        // read the whole body for what it recognises (a block or a 429,
        // `rails::Verdict::of`).
        let rate = crate::ratelimit::rate_words(&response.rate);
        let code = match &response.body {
            Ok(evidence) if status.is_redirection() => evidence.clone(),
            Ok(body) => oauth_error_code(body).to_string(),
            Err(_) => "a body that could not be read".to_string(),
        };
        return Err(TokenError {
            status: Some(status.as_u16()),
            blocked: false,
            halted: false,
            cancelled: false,
            message: format!("token endpoint returned {status} ({rate}): {code}"),
        });
    }
    let body = response
        .body
        .expect("clean 2xx responses have a completed body");
    // C131: a 2xx whose pacing headers were unusable still carries the
    // grant, and the refresh token it replaces is spent (N33), so the
    // tokens are installed; the choke point has journaled the send as
    // the protocol failure it is. Only a whole token response counts:
    // all four fields, so a portal or Cloudflare page installs nothing.
    // C47: an `expires_in` past the bound is refused here, before any
    // token is installed or saved; the sum it would make panicked under
    // `shared` (C134 turns a panic into an abort).
    let refused = |words: String| TokenError {
        status: Some(status.as_u16()),
        blocked: false,
        halted: false,
        cancelled: false,
        message: match &response.protocol {
            Some(protocol) => {
                format!("{words}; and the answer's rate-limit headers were unusable: {protocol}")
            }
            None => words,
        },
    };
    // #113: serde's message can quote a value of the body, so it is not
    // the words.
    let tokens = serde_json::from_str::<TokenResponse>(&body).map_err(|_| {
        refused("the token endpoint answered with a malformed body; nothing was installed".into())
    })?;
    if tokens.expires_in > MAX_ACCESS_TOKEN_LIFETIME_SECS {
        return Err(refused(format!(
            "the token endpoint gave the access token a lifetime of {} seconds, past the {MAX_ACCESS_TOKEN_LIFETIME_SECS} any grant is taken to carry; nothing was installed",
            tokens.expires_in
        )));
    }
    Ok(tokens)
}

/// #113: what the words of a token endpoint's refusal carry of its body:
/// the OAuth `error` code when the body is JSON holding one
/// ([`oauth_error_words`]), else "an unreadable body". Nothing else of
/// the text is kept, so a body that echoes a token cannot carry it into
/// the log.
fn oauth_error_code(body: &str) -> &'static str {
    match serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|value| value.get("error")?.as_str().map(str::to_string))
    {
        Some(code) => oauth_error_words(&code),
        None => "an unreadable body",
    }
}

/// #113, a log line's words are values the daemon chose: an OAuth `error`
/// code, from a token endpoint's body or an authorize redirect's query,
/// as one of RFC 6749's own (sections 4.1.2.1 and 5.2), else "an error
/// code outside OAuth's". Never the text received, whatever its shape: a
/// token can be a lowercase word too.
pub fn oauth_error_words(code: &str) -> &'static str {
    const CODES: [&str; 10] = [
        "invalid_request",
        "invalid_client",
        "invalid_grant",
        "unauthorized_client",
        "unsupported_grant_type",
        "unsupported_response_type",
        "invalid_scope",
        "access_denied",
        "server_error",
        "temporarily_unavailable",
    ];
    CODES
        .iter()
        .find(|known| **known == code)
        .copied()
        .unwrap_or("an error code outside OAuth's")
}

// ---- keyring ------------------------------------------------------------
//
// Refresh tokens live in the OS keyring (invariant 5): macOS Keychain
// Services, the Windows Credential Manager, and on Linux the freedesktop
// Secret Service over D-Bus (GNOME Keyring; KWallet since KDE Frameworks
// 5.97) — the `keyring` crate's backends, selected by feature in Cargo.toml.
// A Linux build without the Secret Service features got the crate's mock
// store — in-memory, per process — while reporting `keyring: ok` (found
// 2026-09-12); the round-trip test below is the tripwire. `ACQ_NO_KEYRING=1`
// degrades to in-memory-only sessions (still never plaintext on disk), and
// so does a keyring the daemon cannot reach: cron, ssh and headless shells
// have no macOS keychain and no Linux session bus (README, `ACQ_NO_SPAWN`),
// and the failure is surfaced as `keyring: unavailable` (rail 7). The
// service name comes from the provider, so mock and real sessions can
// never cross.
//
// The store is a choice, made for the release (2026-10-14; the park "the
// credential store as an explicit choice", `PLATFORMS-SLICE.md` at `b6197a82` step 2):
// keyring 3's feature-selected backends, not keyring 4's `keyring-core`
// with a store crate per platform. That alternative's two reasons do not
// ship. The macOS protected keychain needs `keychain-access-groups`, a
// restricted entitlement only a provisioning profile authorizes, which a
// bare command-line tool cannot carry (Apple DTS, read 2026-10-14) — so
// the file-based keychain `apple-native` uses is the only macOS store
// either way. A Flatpak's Secret portal serves a sandbox the release does
// not have (archives only, note 36 at `b6197a82`). Each backend is held by what touched
// it: macOS by live runs (tokens stored, restored and rotated,
// `RUN-LEDGER.md` 2026-08-22 rungs 1 and 2; a signed `acqd`'s prompts,
// 2026-09-12), Windows by the round trip against Credential Manager
// (`.github/workflows/gate-windows.yml`), Linux by the round trip against
// gnome-keyring (`gate-linux.yml`, and on the Ubuntu VM). Owner,
// 2026-10-14, to that recommendation: "yes, let's go ahead". Reopened by
// a sandboxed package (`decisions/daemon.md`, "Parked").
//
// One keyring entry per account: the entry's user is the GGG username
// (`name#discriminator`), so two accounts on one provider never share a
// secret. The account index (`acquisition_store::Index`) is how a daemon
// knows which entries exist — the keyring cannot enumerate.
//
// The calls block, and they run on the daemon's runtime threads (a save
// happens under the auth lock). On Linux the crate bridges its async D-Bus
// client with `block_on`: on the tokio flavour that enters a second runtime
// from inside ours and panics; on async-io it only blocks the thread —
// hence the `async-io` feature.
//
// macOS identifies a trusted app by its code signature, and the linker's
// ad-hoc signature changes on every rebuild, so a debug `acqd` is asked
// twice per login (the read at start, the modify on save) until
// `tools/sign-acqd.sh` gives it a stable identity after the build.
/// C135 — what one keyring entry holds: the refresh token, and beside it
/// the access token and its expiry as unix seconds, so a restored session
/// sends no token POST until within the refresh margin of that access
/// token's expiry. The entry is one
/// JSON object the daemon alone writes ([`Self::to_secret`]) and reads
/// ([`Self::from_secret`]); a secret that is no such object is the bare
/// refresh token an earlier daemon saved, which restores with no access
/// token and is rewritten in this form at the first save. No version
/// number: the object's shape is the version, a later field is optional,
/// and the bare string is the form before it. Holds both tokens, so its
/// `Debug` names neither (C144).
#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct KeyringEntry {
    pub refresh_token: Secret,
    #[serde(default)]
    pub access_token: Option<Secret>,
    /// Unix seconds. Read back, at most the reader's clock plus
    /// [`MAX_ACCESS_TOKEN_LIFETIME_SECS`] (C47), so [`Self::expires_at`]
    /// adds it to the epoch unchecked.
    #[serde(default)]
    pub access_expires_at: Option<u64>,
}

/// #113: the form a [`KeyringEntry`] is written in, built here alone. The
/// entry itself has no `Serialize`, so no code outside this module can
/// turn its tokens into a string.
#[derive(Serialize)]
struct KeyringEntryForm<'a> {
    refresh_token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_expires_at: Option<u64>,
}

impl KeyringEntry {
    /// The entry for a session's tokens: the refresh token, and the
    /// access token with its wall-clock expiry when the session holds one.
    pub fn new(refresh_token: Secret, access: Option<(Secret, SystemTime)>) -> Self {
        let (access_token, access_expires_at) = match access {
            Some((token, expires)) => (
                Some(token),
                expires
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .ok()
                    .map(|d| d.as_secs()),
            ),
            None => (None, None),
        };
        KeyringEntry {
            refresh_token,
            access_token: access_expires_at.and(access_token),
            access_expires_at,
        }
    }

    /// The secret the keyring holds: the entry as one JSON object.
    pub fn to_secret(&self) -> Secret {
        let form = KeyringEntryForm {
            refresh_token: self.refresh_token.expose(),
            access_token: self.access_token.as_ref().map(Secret::expose),
            access_expires_at: self.access_expires_at,
        };
        Secret(serde_json::to_string(&form).expect("an entry of strings and a number serializes"))
    }

    /// The entry a secret holds, read at `now` on the daemon's wall
    /// clock. A secret that is not the entry's object is a bare refresh
    /// token. An access token without its expiry, or one whose expiry is
    /// past `now` plus [`MAX_ACCESS_TOKEN_LIFETIME_SECS`], is malformed
    /// input (C47) and restores no access token: the refresh token still
    /// does, and the first use refreshes. An expiry already past is kept:
    /// the daemon's one expiry check decides at the first use.
    pub fn from_secret(secret: &Secret, now: SystemTime) -> Self {
        let Ok(mut entry) = serde_json::from_str::<KeyringEntry>(secret.expose()) else {
            return KeyringEntry {
                refresh_token: secret.clone(),
                access_token: None,
                access_expires_at: None,
            };
        };
        let bound = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs())
            .saturating_add(MAX_ACCESS_TOKEN_LIFETIME_SECS);
        let usable =
            entry.access_token.is_some() && entry.access_expires_at.is_some_and(|at| at <= bound);
        if !usable {
            entry.access_token = None;
            entry.access_expires_at = None;
        }
        entry
    }

    /// The access token's expiry on the wall clock, when the entry holds
    /// one.
    pub fn expires_at(&self) -> Option<SystemTime> {
        self.access_expires_at
            .and_then(|secs| SystemTime::UNIX_EPOCH.checked_add(Duration::from_secs(secs)))
    }
}

impl std::fmt::Debug for KeyringEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyringEntry")
            .field("refresh_token", &"<redacted>")
            .field(
                "access_token",
                &self.access_token.as_ref().map(|_| "<redacted>"),
            )
            .field("access_expires_at", &self.access_expires_at)
            .finish()
    }
}

/// Why the keyring is off before any entry is tried, if it is: the
/// daemon's status starts from this, so a daemon with nothing to restore
/// never reports a keyring it was told not to use as `ok`.
pub fn keyring_disabled() -> Option<&'static str> {
    std::env::var_os("ACQ_NO_KEYRING").map(|_| "disabled by ACQ_NO_KEYRING")
}

fn entry(service: &str, username: &str) -> Result<keyring::Entry, String> {
    if let Some(why) = keyring_disabled() {
        return Err(why.into());
    }
    keyring::Entry::new(service, username).map_err(|e| e.to_string())
}

/// Save `secret` (a [`KeyringEntry`]'s, which this never parses) as one
/// account's entry.
pub fn keyring_save(service: &str, secret: &Secret, username: &str) -> Result<(), String> {
    entry(service, username)?
        .set_password(secret.expose())
        .map_err(|e| e.to_string())
}

/// The stored secret for one account, read by [`KeyringEntry::from_secret`];
/// Ok(None) means the keyring works but holds no entry for it.
pub fn keyring_load(service: &str, username: &str) -> Result<Option<Secret>, String> {
    match entry(service, username)?.get_password() {
        Ok(secret) => Ok(Some(Secret(secret))),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

/// Why a clear left a keyring entry where it was (C144: a clear that did
/// not happen is never reported as one).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotCleared {
    /// The keyring is turned off (`ACQ_NO_KEYRING`): nothing was tried,
    /// and an entry a run with the keyring on saved may still be there.
    Off(String),
    /// The keyring refused the delete.
    Failed(String),
}

impl std::fmt::Display for NotCleared {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotCleared::Off(why) => write!(f, "the keyring is off ({why})"),
            NotCleared::Failed(e) => f.write_str(e),
        }
    }
}

/// Remove one account's entry; an entry already gone is cleared. With the
/// keyring disabled (`ACQ_NO_KEYRING`) nothing is tried, and that is
/// `NotCleared::Off`, never a clear: an entry a run with the keyring on
/// saved is still there.
pub fn keyring_clear(service: &str, username: &str) -> Result<(), NotCleared> {
    if let Some(why) = keyring_disabled() {
        return Err(NotCleared::Off(why.into()));
    }
    match entry(service, username)
        .map_err(NotCleared::Failed)?
        .delete_credential()
    {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(NotCleared::Failed(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// C144: a token response's `Debug` names neither token.
    #[test]
    fn a_token_responses_debug_names_no_token() {
        let tokens = TokenResponse {
            access_token: "at-secret".into(),
            refresh_token: "rt-secret".into(),
            expires_in: 3600,
            username: "test-user".into(),
        };
        let shown = format!("{tokens:?}");
        assert!(!shown.contains("secret"), "{shown}");
        assert!(
            shown.contains("test-user") && shown.contains("<redacted>"),
            "{shown}"
        );
    }

    /// C135: an entry written is the entry read, both tokens and the
    /// expiry, and its `Debug` names neither token.
    #[test]
    fn a_keyring_entry_round_trips_through_its_secret() {
        let now = SystemTime::UNIX_EPOCH + Duration::from_secs(946_684_800);
        let entry = KeyringEntry::new(
            "rt-kept".into(),
            Some(("at-kept".into(), now + Duration::from_secs(3600))),
        );
        let secret = entry.to_secret();
        assert!(secret.expose().starts_with('{'), "{secret}");
        let read = KeyringEntry::from_secret(&secret, now);
        assert_eq!(read, entry);
        assert_eq!(
            read.access_token.as_ref().map(Secret::exposed_for_test),
            Some("at-kept")
        );
        assert_eq!(read.expires_at(), Some(now + Duration::from_secs(3600)));
        let shown = format!("{read:?}");
        assert!(!shown.contains("kept"), "{shown}");
    }

    /// Whether `T` implements `Serialize`, decided at compile time by
    /// method resolution: the probe's own method needs the bound, and the
    /// reference's fallback answers when it does not hold.
    struct SerializeProbe<T>(std::marker::PhantomData<T>);
    trait Serializes {
        fn serializes(&self) -> bool {
            true
        }
    }
    impl<T: Serialize> Serializes for SerializeProbe<T> {}
    trait DoesNotSerialize {
        fn serializes(&self) -> bool {
            false
        }
    }
    impl<T> DoesNotSerialize for &SerializeProbe<T> {}

    /// #113: neither a token nor the entry that holds two can be
    /// serialized outside this module: a `serde_json::to_string` of
    /// either does not compile. The probe answers yes for a type that
    /// serializes, so its no is the trait's absence.
    #[test]
    // The borrow picks the fallback when the bound fails: it is the probe.
    #[allow(clippy::needless_borrow)]
    fn a_secret_and_a_keyring_entry_have_no_serialize() {
        assert!(
            (&SerializeProbe::<KeyringEntryForm<'static>>(std::marker::PhantomData)).serializes()
        );
        assert!(!(&SerializeProbe::<KeyringEntry>(std::marker::PhantomData)).serializes());
        assert!(!(&SerializeProbe::<Secret>(std::marker::PhantomData)).serializes());
    }

    /// C135: a secret an earlier daemon saved, the bare refresh token,
    /// reads as that refresh token with no access token; so does an
    /// expiry past the lifetime bound (C47), keeping its refresh token.
    #[test]
    fn a_bare_secret_reads_as_a_refresh_token() {
        let now = SystemTime::UNIX_EPOCH + Duration::from_secs(946_684_800);
        let read = KeyringEntry::from_secret(&"rt-old".into(), now);
        assert_eq!(read.refresh_token, "rt-old");
        assert_eq!(
            (
                read.access_token.as_ref().map(Secret::exposed_for_test),
                read.expires_at()
            ),
            (None, None)
        );

        let far = serde_json::json!({
            "refresh_token": "rt-far",
            "access_token": "at-far",
            "access_expires_at": u64::MAX,
        })
        .to_string();
        let read = KeyringEntry::from_secret(&far.into(), now);
        assert_eq!(read.refresh_token, "rt-far");
        assert_eq!(
            (
                read.access_token.as_ref().map(Secret::exposed_for_test),
                read.expires_at()
            ),
            (None, None)
        );
    }

    use crate::ratelimit::{ALWAYS_WANTED, EndpointState};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::{mpsc, oneshot};

    /// The keyring the daemon saves into is the one it loads from. Every
    /// call makes a fresh `Entry`, as `keyring_save` and `keyring_load` do,
    /// so a backend that only remembers within one `Entry` — the crate's
    /// mock store, which a Linux build got until 2026-09-12 while reporting
    /// `keyring: ok` — fails here. Runs only under `ACQ_KEYRING_ROUNDTRIP=1`,
    /// against the real OS store, so the gate stays hermetic; CI runs it
    /// against an unlocked gnome-keyring (`.github/workflows/gate-linux.yml`),
    /// Credential Manager (`.github/workflows/gate-windows.yml`), and the
    /// macOS Keychain on every release, which `draft` waits on
    /// (`.github/workflows/release.yml`, `keyring-macos`).
    #[test]
    fn keyring_round_trip_survives_a_fresh_entry() {
        if std::env::var_os("ACQ_KEYRING_ROUNDTRIP").is_none() {
            eprintln!(
                "keyring round trip skipped: ACQ_KEYRING_ROUNDTRIP=1 runs it against the OS keyring"
            );
            return;
        }
        let service = "acquisition-keyring-check";
        let username = random_token("roundtrip");
        let token = Secret(random_token("rt"));
        keyring_save(service, &token, &username).expect("save");
        assert_eq!(
            keyring_load(service, &username).expect("load"),
            Some(token),
            "the token saved through one Entry must load through another"
        );
        keyring_clear(service, &username).expect("clear");
        assert_eq!(
            keyring_load(service, &username).expect("load after clear"),
            None
        );
    }

    #[tokio::test]
    async fn oauth_clean_200_waits_for_body_completion_before_recording_success() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = [0; 4096];
            let _ = stream.read(&mut request).await.unwrap();
            let response = concat!(
                "HTTP/1.1 200 OK\r\n",
                "X-Rate-Limit-Policy: token-request-limit\r\n",
                "X-Rate-Limit-Rules: Ip\r\n",
                "X-Rate-Limit-Ip: 60:30:30\r\n",
                "X-Rate-Limit-Ip-State: 1:30:0\r\n",
                "Content-Type: application/json\r\n",
                "Content-Length: 100\r\n",
                "Connection: close\r\n\r\n",
                "{}"
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });
        let choke = ChokePoint::new();
        let provider = Provider::mock(&base);

        let error = refresh(&choke, &provider, &"rt-test".into(), ALWAYS_WANTED)
            .await
            .unwrap_err();
        assert!(!error.message.contains("expected value"), "{error}");
        assert_eq!(
            choke.endpoint_state("oauth-token"),
            EndpointState::Policy("token-request-limit".into())
        );
        let send = choke.recent_sends().pop().unwrap();
        assert!(!send.ok);
        assert!(send.outcome.contains("body transfer failure"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn code_exchange_and_refresh_serialize_across_token_policy_discovery() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let (arrived_tx, mut arrived_rx) = mpsc::unbounded_channel();
        let (release_first, release_first_rx) = oneshot::channel();
        let (release_second, release_second_rx) = oneshot::channel();
        let server = tokio::spawn(async move {
            let releases = [release_first_rx, release_second_rx];
            let mut handlers = Vec::new();
            for (index, release) in releases.into_iter().enumerate() {
                let (mut stream, _) = listener.accept().await.unwrap();
                let request = crate::mockggg::read_request(&mut stream).await.unwrap();
                assert_eq!(request.method, "POST", "token traffic must not HEAD-probe");
                arrived_tx.send(request.body).unwrap();
                handlers.push(tokio::spawn(async move {
                    let body = serde_json::json!({
                        "access_token": format!("at-{index}"),
                        "refresh_token": format!("rt-{index}"),
                        "expires_in": 3600,
                        "username": "test-user",
                    })
                    .to_string();
                    let headers = format!(
                        concat!(
                            "HTTP/1.1 200 OK\r\n",
                            "X-Rate-Limit-Policy: token-request-limit\r\n",
                            "X-Rate-Limit-Rules: Ip\r\n",
                            "X-Rate-Limit-Ip: 60:30:30\r\n",
                            "X-Rate-Limit-Ip-State: {}:30:0\r\n",
                            "Content-Type: application/json\r\n",
                            "Content-Length: {}\r\n",
                            "Connection: close\r\n\r\n"
                        ),
                        index + 1,
                        body.len(),
                    );
                    stream.write_all(headers.as_bytes()).await.unwrap();
                    release.await.unwrap();
                    stream.write_all(body.as_bytes()).await.unwrap();
                }));
            }
            for handler in handlers {
                handler.await.unwrap();
            }
        });

        let choke = std::sync::Arc::new(ChokePoint::new());
        let provider = std::sync::Arc::new(Provider::mock(&base));
        let exchange = {
            let choke = choke.clone();
            let provider = provider.clone();
            tokio::spawn(async move {
                exchange_code(
                    &choke,
                    &provider,
                    &"code".into(),
                    &"verifier".into(),
                    "http://callback",
                    ALWAYS_WANTED,
                )
                .await
            })
        };
        let first = arrived_rx.recv().await.unwrap();
        assert!(first.contains("grant_type=authorization_code"));
        tokio::time::timeout(std::time::Duration::from_secs(1), async {
            while choke.endpoint_state("oauth-token")
                != EndpointState::Policy("token-request-limit".into())
            {
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("the first response headers discover the token policy");

        let refresh = {
            let choke = choke.clone();
            let provider = provider.clone();
            tokio::spawn(async move {
                refresh(&choke, &provider, &"rt-old".into(), ALWAYS_WANTED).await
            })
        };
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(50), arrived_rx.recv())
                .await
                .is_err(),
            "token grants overlapped before policy discovery completed"
        );

        release_first.send(()).unwrap();
        exchange.await.unwrap().unwrap();
        let second = tokio::time::timeout(std::time::Duration::from_secs(1), arrived_rx.recv())
            .await
            .expect("refresh proceeds under the learned token policy")
            .unwrap();
        assert!(second.contains("grant_type=refresh_token"));
        release_second.send(()).unwrap();
        refresh.await.unwrap().unwrap();

        assert_eq!(
            choke.endpoint_state("oauth-token"),
            EndpointState::Policy("token-request-limit".into())
        );
        server.await.unwrap();
    }
}
