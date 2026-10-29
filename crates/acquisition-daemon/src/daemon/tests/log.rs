//! The daemon log holds no token (#113): every credential is an
//! `auth::Secret`, and the log has one writer.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::*;

const ACCESS_AT_LOGIN: &str = "at-login-q7Zr4n";
const REFRESH_AT_LOGIN: &str = "rt-login-p3Xk8v";
const ACCESS_REFRESHED: &str = "at-fresh-m9Wd2s";
const REFRESH_REFRESHED: &str = "rt-fresh-k2Vb6h";
const CODE: &str = "code-j5Ty1c";

/// A provider that answers by what was sent: the code exchange grants
/// the login's tokens, a refresh grants fresh ones, the profile names
/// the user, and the login's access token is rejected on the API's GET
/// (401) while a probe passes. Every form a token POST carried is kept,
/// so the test knows the verifier the daemon made.
async fn token_aware_server() -> (String, Arc<Mutex<Vec<String>>>, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let forms = Arc::new(Mutex::new(Vec::new()));
    let kept = forms.clone();
    let task = tokio::spawn(async move {
        loop {
            let (mut stream, _) = listener.accept().await.unwrap();
            let Some(request) = mockggg::read_request(&mut stream).await else {
                continue;
            };
            let bearer = request
                .headers
                .get("authorization")
                .and_then(|v| v.strip_prefix("Bearer "))
                .unwrap_or("")
                .to_string();
            let api_headers = "X-Rate-Limit-Policy: log-policy\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 100:1:60\r\nX-Rate-Limit-Account-State: 0:1:0\r\n";
            let token_headers = "X-Rate-Limit-Policy: token-request-limit\r\nX-Rate-Limit-Rules: Ip\r\nX-Rate-Limit-Ip: 60:30:30\r\nX-Rate-Limit-Ip-State: 1:30:0\r\n";
            let (status, headers, body) = if request.path == "/token" {
                kept.lock().unwrap().push(request.body.clone());
                let (access, refresh) = if request.body.contains("grant_type=authorization_code") {
                    (ACCESS_AT_LOGIN, REFRESH_AT_LOGIN)
                } else {
                    (ACCESS_REFRESHED, REFRESH_REFRESHED)
                };
                let body = json!({
                    "access_token": access,
                    "refresh_token": refresh,
                    "expires_in": 3600,
                    "username": "log-user",
                })
                .to_string();
                ("200 OK", token_headers, body)
            } else if request.path == "/profile" {
                let body = json!({ "uuid": "uuid-log-user", "name": "log-user" }).to_string();
                ("200 OK", "", body)
            } else if request.method == "HEAD" {
                ("204 No Content", api_headers, String::new())
            } else if bearer == ACCESS_AT_LOGIN {
                (
                    "401 Unauthorized",
                    api_headers,
                    r#"{"error":"expired"}"#.to_string(),
                )
            } else {
                ("200 OK", api_headers, r#"{"characters":[]}"#.to_string())
            };
            mockggg::respond_with(&mut stream, status, "application/json", headers, &body).await;
        }
    });
    (base, forms, task)
}

/// #113: the login (the code and verifier sent in the exchange, the
/// tokens it grants, the profile fetched under its bearer), a send whose
/// bearer GGG rejects (401, the token forgotten and the keyring entry
/// narrowed), and the refresh the next send makes (the refresh token
/// sent, fresh tokens installed and saved), and then the whole log: no
/// token, no code and no verifier anywhere in it. Each value is first
/// seen on the wire, so the paths ran.
#[tokio::test]
async fn no_token_reaches_the_log_on_the_login_send_and_refresh_paths() {
    let (base, forms, server) = token_aware_server().await;
    let clock = Arc::new(ManualClock::new());
    let log_path = test_log_path();
    let rails = Arc::new(Rails::with_config_and_clock(
        RailsConfig {
            journal_path: Some(journal_of(&log_path)),
            ..RailsConfig::default()
        },
        clock.clone(),
    ));
    let credentials = Arc::new(RecordingCredentialStore::default());
    let (mut daemon, log_path) = test_daemon_scenario_at(
        Provider::mock(&base),
        clock.clone(),
        rails,
        credentials.clone() as Arc<dyn CredentialStore>,
        log_path,
    );
    let store_dir = TestPath::dir();
    Arc::get_mut(&mut daemon).expect("fresh daemon").store_dir = Some(store_dir.to_path_buf());
    daemon.shared.lock().unwrap().auth.by_account.clear();
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());

    // The login.
    let authorize = url::Url::parse(&daemon.auth_start().await.unwrap()).unwrap();
    let query: HashMap<String, String> = authorize.query_pairs().into_owned().collect();
    let callback = format!(
        "{}?code={CODE}&state={}",
        query["redirect_uri"], query["state"]
    );
    assert!(reqwest::get(&callback).await.unwrap().status().is_success());
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            if let Response::Auth {
                login_ok: Some(_), ..
            } = daemon.auth_status()
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("the login finished");

    // A send under the login's bearer, rejected.
    let first = daemon
        .submit(
            "characters".into(),
            json!({}),
            0,
            "test".into(),
            Some("log-user".into()),
        )
        .unwrap();
    let (info, _) = wait_terminal(&daemon, first).await;
    assert_eq!(info.state, JobState::Failed, "the 401 fails the job");
    // The next send refreshes first.
    let second = daemon
        .submit(
            "characters".into(),
            json!({}),
            0,
            "test".into(),
            Some("log-user".into()),
        )
        .unwrap();
    let (info, outcome) = wait_terminal(&daemon, second).await;
    assert_eq!(info.state, JobState::Done, "{outcome:?}");
    server.abort();

    let forms = forms.lock().unwrap().clone();
    let exchange = forms
        .iter()
        .find(|form| form.contains("grant_type=authorization_code"))
        .expect("the code exchange was sent");
    assert!(exchange.contains(&format!("code={CODE}")), "{exchange}");
    let verifier = exchange
        .split('&')
        .find_map(|pair| pair.strip_prefix("code_verifier="))
        .expect("the exchange carries the verifier")
        .to_string();
    assert!(
        forms
            .iter()
            .any(|form| form.contains(&format!("refresh_token={REFRESH_AT_LOGIN}"))),
        "the refresh was sent: {forms:?}"
    );
    assert!(
        credentials
            .entries
            .lock()
            .unwrap()
            .get("log-user")
            .is_some_and(|secret| secret.contains(REFRESH_REFRESHED)),
        "the refreshed tokens were saved"
    );

    let log = std::fs::read_to_string(&*log_path).unwrap();
    assert!(
        log.contains("logged in as log-user")
            && log.contains("rejected (401)")
            && log.contains("access token refreshed for log-user"),
        "the log saw the three paths: {log}"
    );
    for (what, needle) in [
        ("the login's access token", ACCESS_AT_LOGIN),
        ("the login's refresh token", REFRESH_AT_LOGIN),
        ("the refreshed access token", ACCESS_REFRESHED),
        ("the refreshed refresh token", REFRESH_REFRESHED),
        ("the login's code", CODE),
        ("the login's verifier", verifier.as_str()),
    ] {
        assert!(!log.contains(needle), "{what} is in the log:\n{log}");
    }
    finish_harness(dispatcher, &log_path);
}

/// A provider whose token endpoint answers every refresh with `status`
/// and `body`; any other request is a 200.
async fn token_endpoint_answering(
    status: &'static str,
    body: &'static str,
) -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        loop {
            let (mut stream, _) = listener.accept().await.unwrap();
            let Some(request) = mockggg::read_request(&mut stream).await else {
                continue;
            };
            let headers = "X-Rate-Limit-Policy: token-request-limit\r\nX-Rate-Limit-Rules: Ip\r\nX-Rate-Limit-Ip: 60:30:30\r\nX-Rate-Limit-Ip-State: 1:30:0\r\n";
            let (status, body) = if request.path == "/token" {
                (status, body)
            } else {
                ("200 OK", r#"{"characters":[]}"#)
            };
            mockggg::respond_with(&mut stream, status, "application/json", headers, body).await;
        }
    });
    (base, task)
}

/// A session whose access token has expired asks `base`'s token endpoint
/// for a refresh through a job; returns the log once the job is done.
async fn refresh_answered_by(status: &'static str, body: &'static str) -> String {
    let (base, server) = token_endpoint_answering(status, body).await;
    let clock = Arc::new(ManualClock::new());
    let log_path = test_log_path();
    let rails = Arc::new(Rails::with_config_and_clock(
        RailsConfig {
            journal_path: Some(journal_of(&log_path)),
            ..RailsConfig::default()
        },
        clock.clone(),
    ));
    let (daemon, log_path) = test_daemon_scenario_at(
        Provider::mock(&base),
        clock.clone(),
        rails,
        Arc::new(NoopCredentialStore),
        log_path,
    );
    {
        let mut s = daemon.shared.lock().unwrap();
        s.auth.rename("", "scenario-user");
        daemon.install_tokens_locked(
            s.auth.one_mut(),
            auth::TokenResponse {
                access_token: "at-expired".into(),
                refresh_token: "rt-held".into(),
                expires_in: 60,
                username: "scenario-user".into(),
            },
        );
    }
    clock.advance(Duration::from_secs(120));
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let job = daemon
        .submit(
            "characters".into(),
            json!({}),
            0,
            "test".into(),
            Some("scenario-user".into()),
        )
        .unwrap();
    let (info, _) = wait_terminal(&daemon, job).await;
    assert_eq!(info.state, JobState::Failed, "the refresh failed the job");
    server.abort();
    let log = std::fs::read_to_string(&*log_path).unwrap();
    finish_harness(dispatcher, &log_path);
    log
}

/// #113, the text a daemon receives: a token endpoint's refusal whose
/// body echoes tokens, one in a field and one as its `error`, reaches the
/// log as its status and no more of its text. The code is kept when it is
/// a word, so the refusal still says what GGG said.
#[tokio::test]
async fn a_token_refusal_whose_body_echoes_a_token_logs_none_of_its_text() {
    let log = refresh_answered_by(
        "400 Bad Request",
        r#"{"error":"invalid_grant","refresh_token":"rt-echoed-w4Qe"}"#,
    )
    .await;
    assert!(
        log.contains("400") && log.contains("invalid_grant"),
        "{log}"
    );
    assert!(
        !log.contains("rt-echoed-w4Qe"),
        "the echoed token is in the log:\n{log}"
    );

    let log = refresh_answered_by("400 Bad Request", r#"{"error":"at-echoed-z8Lm"}"#).await;
    assert!(
        !log.contains("at-echoed-z8Lm"),
        "the echoed token is in the log:\n{log}"
    );
    assert!(log.contains("an unreadable error code"), "{log}");
}

/// #113, the text a daemon receives: a 2xx from the token endpoint that
/// is no token response, whose parser's message would quote a value
/// (here a token where a number goes), reaches the log as a malformed
/// body, quoting nothing.
#[tokio::test]
async fn a_malformed_token_answer_logs_none_of_its_text() {
    let log = refresh_answered_by(
        "200 OK",
        r#"{"access_token":"at-x","refresh_token":"rt-x","expires_in":"rt-quoted-n3Jp","username":"scenario-user"}"#,
    )
    .await;
    assert!(
        !log.contains("rt-quoted-n3Jp"),
        "the quoted token is in the log:\n{log}"
    );
    assert!(log.contains("malformed body"), "{log}");
}
