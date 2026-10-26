//! OAuth client mechanics: PKCE, token exchange/refresh, keyring storage.
//!
//! The flow is authorization code + PKCE with a loopback redirect, refresh
//! rotation, and OS keyring storage. Which provider it points at — the
//! localhost mock or real GGG — is entirely the `Provider` passed in.

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use rand::RngCore as _;
use serde::Deserialize;
use sha2::{Digest, Sha256};

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

/// Returns (verifier, challenge).
pub fn pkce_pair() -> (String, String) {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    let verifier = URL_SAFE_NO_PAD.encode(bytes);
    let challenge = s256(&verifier);
    (verifier, challenge)
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

/// Holds both tokens, so its `Debug` names neither (C144: no secret is
/// formatted where a log or a panic could carry it).
#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
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
    code: &str,
    verifier: &str,
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
            ("code", code),
            ("code_verifier", verifier),
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
    refresh_token: &str,
    wanted: Wanted<'_>,
) -> Result<TokenResponse, TokenError> {
    token_request(
        choke,
        provider,
        &[
            ("grant_type", "refresh_token"),
            ("client_id", provider.client_id),
            ("refresh_token", refresh_token),
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
        let rate = response.rate;
        let body = response
            .body
            .unwrap_or_else(|error| format!("<body read transport failure: {error}>"));
        let body: String = body.chars().take(300).collect();
        return Err(TokenError {
            status: Some(status.as_u16()),
            blocked: false,
            halted: false,
            cancelled: false,
            message: format!("token endpoint returned {status} (rate headers {rate}): {body}"),
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
    let tokens =
        serde_json::from_str::<TokenResponse>(&body).map_err(|e| refused(e.to_string()))?;
    if tokens.expires_in > MAX_ACCESS_TOKEN_LIFETIME_SECS {
        return Err(refused(format!(
            "the token endpoint gave the access token a lifetime of {} seconds, past the {MAX_ACCESS_TOKEN_LIFETIME_SECS} any grant is taken to carry; nothing was installed",
            tokens.expires_in
        )));
    }
    Ok(tokens)
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

pub fn keyring_save(service: &str, refresh_token: &str, username: &str) -> Result<(), String> {
    entry(service, username)?
        .set_password(refresh_token)
        .map_err(|e| e.to_string())
}

/// The stored refresh token for one account; Ok(None) means the keyring
/// works but holds no entry for it.
pub fn keyring_load(service: &str, username: &str) -> Result<Option<String>, String> {
    match entry(service, username)?.get_password() {
        Ok(secret) => Ok(Some(secret)),
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
    /// against an unlocked gnome-keyring (`.github/workflows/gate-linux.yml`)
    /// and Credential Manager (`.github/workflows/gate-windows.yml`).
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
        let token = random_token("rt");
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

        let error = refresh(&choke, &provider, "rt-test", ALWAYS_WANTED)
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
                    "code",
                    "verifier",
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
            tokio::spawn(async move { refresh(&choke, &provider, "rt-old", ALWAYS_WANTED).await })
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
