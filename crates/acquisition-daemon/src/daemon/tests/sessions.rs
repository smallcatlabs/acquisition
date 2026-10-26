//! The session layer's tests, with their own harness: account selection,
//! the single-flight refresh, and logout or re-login during a refresh.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::TestPath;
use crate::daemon::refresh::{REFRESH_OWNER_ABANDONED, SESSION_CHANGED_DURING_REFRESH};
use crate::daemon::*;

/// C51: a bare name two live sessions share is refused, never guessed
/// (review finding 2026-09-10); the full name and the uuid select.
#[test]
fn a_selector_matching_several_sessions_is_ambiguous() {
    let mut sessions = Sessions::default();
    for (name, uuid) in [
        ("Alice#1234", "u-1"),
        ("Alice#5678", "u-2"),
        ("Bob#0001", "u-3"),
    ] {
        sessions.replace(AuthSession {
            username: Some(name.into()),
            uuid: Some(uuid.into()),
            ..AuthSession::default()
        });
    }
    let err = match sessions.matching("alice") {
        Err(err) => err,
        Ok(_) => panic!("a shared bare name selected a session"),
    };
    assert_eq!(err.kind, ErrorKind::AmbiguousAccount);
    assert!(
        err.message.contains("Alice#1234, Alice#5678") && err.message.contains("full name"),
        "{err}"
    );
    assert_eq!(
        sessions
            .matching("Alice#5678")
            .unwrap()
            .and_then(|s| s.uuid.clone()),
        Some("u-2".into())
    );
    assert_eq!(
        sessions
            .matching("u-1")
            .unwrap()
            .and_then(|s| s.username.clone()),
        Some("Alice#1234".into())
    );
    assert_eq!(
        sessions
            .matching("bob")
            .unwrap()
            .and_then(|s| s.username.clone()),
        Some("Bob#0001".into())
    );
    assert!(sessions.matching("carol").unwrap().is_none());
}
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::oneshot;

#[derive(Default)]
struct MemoryCredentialStore {
    saves: Mutex<Vec<(String, String, String)>>,
    cleared: AtomicBool,
}

impl CredentialStore for MemoryCredentialStore {
    fn save(&self, service: &str, refresh_token: &str, username: &str) -> Result<(), String> {
        self.saves.lock().unwrap().push((
            service.to_string(),
            refresh_token.to_string(),
            username.to_string(),
        ));
        Ok(())
    }

    fn load(&self, _service: &str, _username: &str) -> Result<Option<String>, String> {
        Ok(None)
    }

    fn clear(&self, _service: &str, _username: &str) -> Result<(), auth::NotCleared> {
        self.cleared.store(true, Ordering::SeqCst);
        Ok(())
    }
}

struct DelayedTokenResponse {
    base: String,
    arrived: oneshot::Receiver<()>,
    release: oneshot::Sender<()>,
    server: tokio::task::JoinHandle<()>,
}

async fn delayed_token_response(status: &'static str, body: String) -> DelayedTokenResponse {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let (arrived_tx, arrived) = oneshot::channel();
    let (release, release_rx) = oneshot::channel();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let request = mockggg::read_request(&mut stream).await.unwrap();
        assert_eq!(request.method, "POST");
        assert_eq!(request.path, "/token");
        assert!(request.body.contains("grant_type=refresh_token"));
        assert!(request.body.contains("refresh_token=rt-old"));
        arrived_tx.send(()).unwrap();
        release_rx.await.unwrap();
        let headers = concat!(
            "X-Rate-Limit-Policy: token-request-limit\r\n",
            "X-Rate-Limit-Rules: Ip\r\n",
            "X-Rate-Limit-Ip: 60:30:30\r\n",
            "X-Rate-Limit-Ip-State: 1:30:0\r\n",
        );
        mockggg::respond_with(&mut stream, status, "application/json", headers, &body).await;
    });
    DelayedTokenResponse {
        base,
        arrived,
        release,
        server,
    }
}

struct AbandonThenSuccessTokenResponses {
    base: String,
    first_arrived: oneshot::Receiver<()>,
    release_first: oneshot::Sender<()>,
    requests: Arc<Mutex<Vec<String>>>,
    server: tokio::task::JoinHandle<()>,
}

async fn abandon_then_success_token_responses(
    success_body: String,
) -> AbandonThenSuccessTokenResponses {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let (first_arrived_tx, first_arrived) = oneshot::channel();
    let (release_first, release_first_rx) = oneshot::channel();
    let requests = Arc::new(Mutex::new(Vec::new()));
    let captured = requests.clone();
    let server = tokio::spawn(async move {
        let headers = concat!(
            "X-Rate-Limit-Policy: token-request-limit\r\n",
            "X-Rate-Limit-Rules: Ip\r\n",
            "X-Rate-Limit-Ip: 60:30:30\r\n",
            "X-Rate-Limit-Ip-State: 1:30:0\r\n",
        );

        let (mut first, _) = listener.accept().await.unwrap();
        let request = mockggg::read_request(&mut first).await.unwrap();
        assert_eq!(request.method, "POST");
        assert_eq!(request.path, "/token");
        captured.lock().unwrap().push(request.body);
        first_arrived_tx.send(()).unwrap();
        release_first_rx.await.unwrap();
        mockggg::respond_with(
            &mut first,
            "200 OK",
            "application/json",
            headers,
            &success_body,
        )
        .await;

        let (mut second, _) = listener.accept().await.unwrap();
        let request = mockggg::read_request(&mut second).await.unwrap();
        assert_eq!(request.method, "POST");
        assert_eq!(request.path, "/token");
        captured.lock().unwrap().push(request.body);
        mockggg::respond_with(
            &mut second,
            "200 OK",
            "application/json",
            headers,
            &success_body,
        )
        .await;
    });
    AbandonThenSuccessTokenResponses {
        base,
        first_arrived,
        release_first,
        requests,
        server,
    }
}

fn tokens(access_token: &str, refresh_token: &str, username: &str) -> auth::TokenResponse {
    auth::TokenResponse {
        access_token: access_token.into(),
        refresh_token: refresh_token.into(),
        expires_in: 3600,
        username: username.into(),
    }
}

fn token_body(access_token: &str, refresh_token: &str, username: &str) -> String {
    json!({
        "access_token": access_token,
        "refresh_token": refresh_token,
        "expires_in": 3600,
        "username": username,
    })
    .to_string()
}

fn test_daemon(base: &str) -> (Arc<Daemon>, Arc<MemoryCredentialStore>, TestPath) {
    let log_path = TestPath::file("daemon.log");
    let log = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&log_path)
        .unwrap();
    let credential_store = Arc::new(MemoryCredentialStore::default());
    let choke =
        ChokePoint::with_clock_and_rails(Arc::new(SystemClock), Arc::new(Rails::disabled()), base);
    let now = choke.now();
    let daemon = Arc::new(Daemon {
        shared: Mutex::new(Shared {
            jobs: HashMap::new(),
            next_id: 1,
            auth: Sessions::with(AuthSession {
                access_token: Some("at-expired".into()),
                access_expires_at: Some(SystemTime::now()),
                refresh_token: Some("rt-old".into()),
                username: Some("old-user".into()),
                keyring: "ok".into(),
                generations: AuthGenerations {
                    session: 7,
                    access_token: 11,
                    refresh_token: 13,
                },
                next_refresh_flight: 1,
                ..AuthSession::default()
            }),
            connections: 0,
            last_activity: now,
            errors: VecDeque::new(),
            active_jobs: HashMap::new(),
        }),
        started: now,
        events: broadcast::channel(16).0,
        work: Notify::new(),
        log: Mutex::new(log),
        choke,
        provider: Provider::mock(base),
        artifact: None,
        credential_store: credential_store.clone(),
        world: None,
        log_path: log_path.to_str().map(str::to_string),
        socket_path: None,
        store_dir: None,
        store: Mutex::new(None),
        jobs_db: Mutex::new(JobDb::open_memory().unwrap()),
        queue_failure: Mutex::new(None),
        exit: Exit::default(),
        refresh_landed_hook: Mutex::new(None),
    });
    (daemon, credential_store, log_path)
}

/// Realm on the wire (C58, C59): pc is omitted, so a pc
/// URL and route label are byte-identical whether the param is absent
/// or explicit — every live send so far stays the same; any other
/// realm is a segment before the league or name on the URL and a
/// suffix on the label, so it gets its own probe; a realm a family
/// does not take renders nothing and is refused at admission.
#[test]
fn realm_is_a_segment_before_league_or_name_and_pc_is_silent() {
    let (daemon, _, _) = test_daemon("http://mock");
    let route = |kind: &str, params: Value| daemon.route_for(kind, &params);
    for (kind, absent, explicit) in [
        ("characters", json!({}), json!({ "realm": "pc" })),
        (
            "character",
            json!({ "name": "Exile" }),
            json!({ "realm": "pc", "name": "Exile" }),
        ),
        (
            "stashes",
            json!({ "league": "Standard" }),
            json!({ "realm": "pc", "league": "Standard" }),
        ),
        (
            "stash",
            json!({ "league": "Standard", "id": "t1", "sub": "s1" }),
            json!({ "realm": "pc", "league": "Standard", "id": "t1", "sub": "s1" }),
        ),
    ] {
        let a = route(kind, absent).unwrap();
        assert_eq!(
            a,
            route(kind, explicit).unwrap(),
            "{kind}: pc must add nothing"
        );
        assert!(
            !a.0.contains('/'),
            "{kind}: pc label {:?} carries no realm",
            a.0
        );
    }
    assert_eq!(
        route("characters", json!({ "realm": "poe2" })).unwrap(),
        (
            "character-list/poe2".into(),
            "http://mock/character/poe2".into()
        )
    );
    assert_eq!(
        route("character", json!({ "realm": "poe2", "name": "Exile" })).unwrap(),
        (
            "character/poe2".into(),
            "http://mock/character/poe2/Exile".into()
        )
    );
    assert_eq!(
        route("stashes", json!({ "realm": "xbox", "league": "Standard" })).unwrap(),
        (
            "stash-list/xbox".into(),
            "http://mock/stash/xbox/Standard".into()
        )
    );
    assert_eq!(
        route(
            "stash",
            json!({ "realm": "sony", "league": "Standard", "id": "t1", "sub": "s1" })
        )
        .unwrap(),
        (
            "stash/sony".into(),
            "http://mock/stash/sony/Standard/t1/s1".into()
        )
    );
    // PoE1-only families never render a poe2 URL.
    assert_eq!(
        route("stashes", json!({ "realm": "poe2", "league": "Standard" })),
        None
    );
    assert_eq!(
        route(
            "stash",
            json!({ "realm": "poe2", "league": "Standard", "id": "t1" })
        ),
        None
    );
    // Admission refuses before a job id exists — and inside an apply's
    // tuple list, where the vocabulary check already lives.
    let refused = daemon
        .submit(
            "stashes".into(),
            json!({ "realm": "poe2", "league": "Standard" }),
            0,
            "test".into(),
            None,
        )
        .unwrap_err();
    assert!(
        refused
            .message
            .contains("stashes endpoints do not take realm poe2"),
        "{refused}"
    );
    let refused = daemon
        .submit(
            "characters".into(),
            json!({ "realm": "ps5" }),
            0,
            "test".into(),
            None,
        )
        .unwrap_err();
    assert!(
        refused.message.contains("unknown realm \"ps5\""),
        "{refused}"
    );
    let refused = validate_apply(&json!({ "jobs": [
        { "kind": "stashes", "params": { "realm": "xbox", "league": "Standard" } },
        { "kind": "stash", "params": { "realm": "poe2", "league": "Standard", "id": "t1" } },
    ] }))
    .unwrap_err();
    assert!(
        refused
            .message
            .starts_with("action 1: the stashes endpoints"),
        "{refused}"
    );
    assert_eq!(
        daemon.shared.lock().unwrap().jobs.len(),
        0,
        "refusals admit nothing"
    );
}

async fn wait_for_refresh_waiters(daemon: &Daemon, expected: usize) {
    tokio::time::timeout(Duration::from_secs(1), async {
        loop {
            let receivers = daemon
                .shared
                .lock()
                .unwrap()
                .auth
                .one()
                .refresh_flight
                .as_ref()
                .map_or(0, |flight| flight.result.receiver_count());
            if receivers == expected {
                return;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("refresh waiters joined the flight");
}

#[tokio::test]
async fn concurrent_expiry_has_one_refresh_owner_and_shared_waiter_result() {
    let delayed = delayed_token_response(
        "200 OK",
        token_body("at-rotated", "rt-rotated", "shared-user"),
    )
    .await;
    let (daemon, store, _log_path) = test_daemon(&delayed.base);

    let owner_daemon = daemon.clone();
    let owner =
        tokio::spawn(async move { owner_daemon.valid_access_token(None, false, None).await });
    delayed.arrived.await.unwrap();
    let waiter_daemon = daemon.clone();
    let waiter =
        tokio::spawn(async move { waiter_daemon.valid_access_token(None, false, None).await });
    tokio::task::yield_now().await;
    delayed.release.send(()).unwrap();

    let owner_result = owner.await.unwrap().unwrap();
    let waiter_result = waiter.await.unwrap().unwrap();
    assert_eq!(owner_result, ("at-rotated".into(), "shared-user".into()));
    assert_eq!(waiter_result, owner_result);
    assert_eq!(store.saves.lock().unwrap().len(), 1);
    delayed.server.await.unwrap();
}

#[tokio::test]
async fn authentication_completes_before_the_api_send_takes_a_gate_slot() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let (hold_arrived_tx, hold_arrived) = oneshot::channel();
    let (release_hold, release_hold_rx) = oneshot::channel();
    let (token_arrived_tx, token_arrived) = oneshot::channel();
    let server = tokio::spawn(async move {
        let (mut hold, _) = listener.accept().await.unwrap();
        let request = mockggg::read_request(&mut hold).await.unwrap();
        assert_eq!(request.path, "/hold");
        let headers = concat!(
            "HTTP/1.1 200 OK\r\n",
            "X-Rate-Limit-Policy: hold-policy\r\n",
            "X-Rate-Limit-Rules: Ip\r\n",
            "X-Rate-Limit-Ip: 60:30:30\r\n",
            "X-Rate-Limit-Ip-State: 1:30:0\r\n",
            "Content-Type: application/json\r\n",
            "Content-Length: 2\r\n",
            "Connection: close\r\n\r\n",
        );
        hold.write_all(headers.as_bytes()).await.unwrap();
        hold_arrived_tx.send(()).unwrap();

        let (mut token, _) = listener.accept().await.unwrap();
        let request = mockggg::read_request(&mut token).await.unwrap();
        assert_eq!(request.method, "POST");
        assert_eq!(request.path, "/token");
        token_arrived_tx.send(()).unwrap();
        let token_headers = concat!(
            "X-Rate-Limit-Policy: token-request-limit\r\n",
            "X-Rate-Limit-Rules: Ip\r\n",
            "X-Rate-Limit-Ip: 60:30:30\r\n",
            "X-Rate-Limit-Ip-State: 1:30:0\r\n",
        );
        mockggg::respond_with(
            &mut token,
            "200 OK",
            "application/json",
            token_headers,
            &token_body("at-live", "rt-rotated", "test-user"),
        )
        .await;

        let (mut api, _) = listener.accept().await.unwrap();
        let request = mockggg::read_request(&mut api).await.unwrap();
        assert_eq!(request.method, "GET");
        assert_eq!(request.path, "/character");
        let api_headers = concat!(
            "X-Rate-Limit-Policy: character-list-request-limit\r\n",
            "X-Rate-Limit-Rules: Account\r\n",
            "X-Rate-Limit-Account: 2:10:60,5:300:300\r\n",
            "X-Rate-Limit-Account-State: 1:10:0,1:300:0\r\n",
        );
        mockggg::respond_with(
            &mut api,
            "200 OK",
            "application/json",
            api_headers,
            r#"{"characters":[]}"#,
        )
        .await;

        release_hold_rx.await.unwrap();
        hold.write_all(b"{}").await.unwrap();
    });

    let (daemon, _, _log_path) = test_daemon(&base);
    let hold = {
        let daemon = daemon.clone();
        let url = format!("{base}/hold");
        tokio::spawn(async move {
            daemon
                .choke
                .get(
                    "hold-route",
                    &url,
                    daemon.choke.now(),
                    crate::ratelimit::ALWAYS_WANTED,
                )
                .await
        })
    };
    hold_arrived.await.unwrap();
    let api = {
        let daemon = daemon.clone();
        tokio::spawn(async move {
            daemon
                .execute_inner(
                    1,
                    "characters",
                    serde_json::Value::Null,
                    Some("character-list".into()),
                    None,
                    daemon.choke.now(),
                )
                .await
        })
    };

    tokio::time::timeout(Duration::from_secs(1), token_arrived)
        .await
        .expect("refresh can use the remaining gate slot before the API send")
        .unwrap();
    assert!(matches!(
        api.await.unwrap().unwrap(),
        Outcome::Success { .. }
    ));
    release_hold.send(()).unwrap();
    hold.await.unwrap().unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn abandoned_refresh_owner_completes_all_waiters_and_allows_retry() {
    let responses =
        abandon_then_success_token_responses(token_body("at-retried", "rt-retried", "retry-user"))
            .await;
    let (daemon, store, _log_path) = test_daemon(&responses.base);
    let before = daemon.shared.lock().unwrap().auth.one().generations;

    let owner_daemon = daemon.clone();
    let owner =
        tokio::spawn(async move { owner_daemon.valid_access_token(None, false, None).await });
    responses.first_arrived.await.unwrap();

    let mut waiters = Vec::new();
    for _ in 0..3 {
        let waiter_daemon = daemon.clone();
        waiters.push(tokio::spawn(async move {
            waiter_daemon.valid_access_token(None, false, None).await
        }));
    }
    wait_for_refresh_waiters(&daemon, waiters.len()).await;

    owner.abort();
    assert!(owner.await.unwrap_err().is_cancelled());
    let mut waiter_errors = Vec::new();
    for waiter in waiters {
        waiter_errors.push(
            tokio::time::timeout(Duration::from_secs(1), waiter)
                .await
                .expect("abandoned refresh waiter completed")
                .unwrap()
                .unwrap_err(),
        );
    }
    assert_eq!(waiter_errors, vec![REFRESH_OWNER_ABANDONED; 3]);
    assert!(waiter_errors.iter().all(|e| e.kind == ErrorKind::Internal));
    {
        let s = daemon.shared.lock().unwrap();
        assert!(s.auth.one().refresh_flight.is_none());
        assert_eq!(s.auth.one().generations, before);
        assert_eq!(s.auth.one().refresh_token.as_deref(), Some("rt-old"));
        assert_eq!(s.auth.one().access_token.as_deref(), Some("at-expired"));
    }
    assert!(store.saves.lock().unwrap().is_empty());

    responses.release_first.send(()).unwrap();
    let retry = tokio::time::timeout(
        Duration::from_secs(2),
        daemon.valid_access_token(None, false, None),
    )
    .await
    .expect("retry after owner abandonment completed")
    .unwrap();
    assert_eq!(retry, ("at-retried".into(), "retry-user".into()));
    responses.server.await.unwrap();
    let requests = responses.requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    assert!(
        requests
            .iter()
            .all(|body| body.contains("refresh_token=rt-old"))
    );
    assert_eq!(store.saves.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn abandoned_refresh_owner_without_waiters_allows_retry() {
    let responses =
        abandon_then_success_token_responses(token_body("at-retried", "rt-retried", "retry-user"))
            .await;
    let (daemon, store, _log_path) = test_daemon(&responses.base);
    let before = daemon.shared.lock().unwrap().auth.one().generations;

    let owner_daemon = daemon.clone();
    let owner =
        tokio::spawn(async move { owner_daemon.valid_access_token(None, false, None).await });
    responses.first_arrived.await.unwrap();
    owner.abort();
    assert!(owner.await.unwrap_err().is_cancelled());
    {
        let s = daemon.shared.lock().unwrap();
        assert!(s.auth.one().refresh_flight.is_none());
        assert_eq!(s.auth.one().generations, before);
        assert_eq!(s.auth.one().refresh_token.as_deref(), Some("rt-old"));
        assert_eq!(s.auth.one().access_token.as_deref(), Some("at-expired"));
    }
    assert!(store.saves.lock().unwrap().is_empty());

    responses.release_first.send(()).unwrap();
    let retry = tokio::time::timeout(
        Duration::from_secs(2),
        daemon.valid_access_token(None, false, None),
    )
    .await
    .expect("retry after owner abandonment completed")
    .unwrap();
    assert_eq!(retry, ("at-retried".into(), "retry-user".into()));
    responses.server.await.unwrap();
    let requests = responses.requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    assert!(
        requests
            .iter()
            .all(|body| body.contains("refresh_token=rt-old"))
    );
    assert_eq!(store.saves.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn successful_refresh_rotation_advances_token_generations_and_persists_new_token() {
    let delayed = delayed_token_response(
        "200 OK",
        token_body("at-rotated", "rt-rotated", "rotated-user"),
    )
    .await;
    let (daemon, store, _log_path) = test_daemon(&delayed.base);
    let before = daemon.shared.lock().unwrap().auth.one().generations;

    let refresh_daemon = daemon.clone();
    let refresh =
        tokio::spawn(async move { refresh_daemon.valid_access_token(None, false, None).await });
    delayed.arrived.await.unwrap();
    delayed.release.send(()).unwrap();
    assert_eq!(
        refresh.await.unwrap().unwrap(),
        ("at-rotated".into(), "rotated-user".into())
    );

    {
        let s = daemon.shared.lock().unwrap();
        assert_eq!(s.auth.one().refresh_token.as_deref(), Some("rt-rotated"));
        assert_eq!(s.auth.one().access_token.as_deref(), Some("at-rotated"));
        assert_eq!(s.auth.one().generations.session, before.session);
        assert_ne!(s.auth.one().generations.access_token, before.access_token);
        assert_ne!(s.auth.one().generations.refresh_token, before.refresh_token);
    }
    assert_eq!(
        store.saves.lock().unwrap().as_slice(),
        [(
            "acq-mock".into(),
            "rt-rotated".into(),
            "rotated-user".into()
        )]
    );
    delayed.server.await.unwrap();
}

#[tokio::test]
async fn refresh_failure_is_shared_and_leaves_token_state_unchanged() {
    let delayed =
        delayed_token_response("400 Bad Request", r#"{"error":"invalid_grant"}"#.into()).await;
    let (daemon, store, _log_path) = test_daemon(&delayed.base);
    let before = daemon.shared.lock().unwrap().auth.one().generations;

    let owner_daemon = daemon.clone();
    let owner =
        tokio::spawn(async move { owner_daemon.valid_access_token(None, false, None).await });
    delayed.arrived.await.unwrap();
    let waiter_daemon = daemon.clone();
    let waiter =
        tokio::spawn(async move { waiter_daemon.valid_access_token(None, false, None).await });
    tokio::task::yield_now().await;
    delayed.release.send(()).unwrap();

    let owner_error = owner.await.unwrap().unwrap_err();
    let waiter_error = waiter.await.unwrap().unwrap_err();
    assert_eq!(waiter_error, owner_error);
    assert!(owner_error.message.contains("400 Bad Request"));
    {
        let s = daemon.shared.lock().unwrap();
        assert_eq!(s.auth.one().generations, before);
        assert_eq!(s.auth.one().refresh_token.as_deref(), Some("rt-old"));
        assert_eq!(s.auth.one().access_token.as_deref(), Some("at-expired"));
        assert!(s.auth.one().refresh_flight.is_none());
    }
    assert!(store.saves.lock().unwrap().is_empty());
    delayed.server.await.unwrap();
}

#[tokio::test]
async fn logout_during_refresh_rejects_stale_completion_in_memory_and_keyring() {
    let delayed =
        delayed_token_response("200 OK", token_body("at-stale", "rt-stale", "stale-user")).await;
    let (daemon, store, _log_path) = test_daemon(&delayed.base);

    let refresh_daemon = daemon.clone();
    let refresh =
        tokio::spawn(async move { refresh_daemon.valid_access_token(None, false, None).await });
    delayed.arrived.await.unwrap();
    daemon.logout(None).unwrap();
    delayed.release.send(()).unwrap();

    assert_eq!(
        refresh.await.unwrap().unwrap_err(),
        SESSION_CHANGED_DURING_REFRESH
    );
    assert!(
        daemon.shared.lock().unwrap().auth.by_account.is_empty(),
        "the session is gone"
    );
    assert!(store.cleared.load(Ordering::SeqCst));
    assert!(store.saves.lock().unwrap().is_empty());
    delayed.server.await.unwrap();
}

#[tokio::test]
async fn reauthentication_during_refresh_keeps_new_session_and_rejects_old_completion() {
    // A re-login as the *same* account while its refresh is in flight:
    // the new session replaces the old one (generations advance), and
    // the stale completion is rejected. Another account's login would
    // not touch this session at all (see the two-session tests).
    let delayed =
        delayed_token_response("200 OK", token_body("at-stale", "rt-stale", "old-user")).await;
    let (daemon, store, _log_path) = test_daemon(&delayed.base);
    let before = daemon.shared.lock().unwrap().auth.one().generations;

    let refresh_daemon = daemon.clone();
    let refresh =
        tokio::spawn(async move { refresh_daemon.valid_access_token(None, false, None).await });
    delayed.arrived.await.unwrap();
    let flow_generation = daemon.begin_auth_flow();
    let staged = daemon
        .stage_auth_flow(
            flow_generation,
            tokens("at-reauth", "rt-reauth", "old-user"),
        )
        .expect("current flow stages");
    assert!(daemon.complete_login(flow_generation, &staged, "u-old"));
    delayed.release.send(()).unwrap();

    assert_eq!(
        refresh.await.unwrap().unwrap_err(),
        SESSION_CHANGED_DURING_REFRESH
    );
    assert_eq!(
        daemon.shared.lock().unwrap().auth.one().uuid.as_deref(),
        Some("u-old"),
        "the registered session carries its uuid"
    );
    {
        let s = daemon.shared.lock().unwrap();
        assert_eq!(s.auth.one().access_token.as_deref(), Some("at-reauth"));
        assert_eq!(s.auth.one().refresh_token.as_deref(), Some("rt-reauth"));
        assert_eq!(s.auth.one().username.as_deref(), Some("old-user"));
        assert_ne!(s.auth.one().generations.session, before.session);
    }
    assert_eq!(
        store.saves.lock().unwrap().as_slice(),
        [("acq-mock".into(), "rt-reauth".into(), "old-user".into())]
    );
    delayed.server.await.unwrap();
}

/// A refresh that renames the account updates the existing index
/// mapping in place (the session carries its uuid, so `record_login`
/// follows it) — never a second, uuid-less entry that would make both
/// the username and uuid selectors ambiguous.
#[tokio::test]
async fn a_refresh_rename_updates_the_index_mapping_in_place() {
    let delayed =
        delayed_token_response("200 OK", token_body("at-new", "rt-new", "NewName#2")).await;
    let (mut daemon, _, _log_path) = test_daemon(&delayed.base);
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    Arc::get_mut(&mut daemon).unwrap().store_dir = Some(dir.clone());
    // As a completed login left things: indexed with the uuid, and the
    // session carrying it.
    {
        let mut index = Index::load(&dir).unwrap();
        index.record_login("old-user", "u-1", true, 1, &[]).unwrap();
    }
    daemon.shared.lock().unwrap().auth.one_mut().uuid = Some("u-1".into());

    let refresh_daemon = daemon.clone();
    let refresh =
        tokio::spawn(async move { refresh_daemon.valid_access_token(None, false, None).await });
    delayed.arrived.await.unwrap();
    delayed.release.send(()).unwrap();
    assert_eq!(
        refresh.await.unwrap().unwrap(),
        ("at-new".into(), "NewName#2".into())
    );

    let index = Index::load(&dir).unwrap();
    assert_eq!(index.entries().len(), 1, "one entry, moved — no twin");
    let entry = index.get("NewName#2").expect("the entry follows the name");
    assert_eq!(entry.uuid.as_deref(), Some("u-1"));
    assert!(
        daemon
            .shared
            .lock()
            .unwrap()
            .auth
            .find("NewName#2")
            .is_some_and(|x| x.uuid.as_deref() == Some("u-1")),
        "the session moved with its uuid"
    );
    delayed.server.await.unwrap();
}

// ---- multi-account step (2): the job's account is what runs ---------

#[test]
fn submit_resolves_the_account_against_the_live_session() {
    let (daemon, _, _log_path) = test_daemon("http://127.0.0.1:1");
    // Session is "old-user" (test_daemon).
    assert_eq!(
        daemon.resolve_account("characters", None),
        Ok(Some("old-user".into()))
    );
    assert_eq!(
        daemon.resolve_account("characters", Some("OLD-USER")),
        Ok(Some("old-user".into()))
    );
    let err = daemon
        .resolve_account("characters", Some("Other#1"))
        .unwrap_err();
    assert!(
        err.message.contains("Other#1") && err.message.contains("old-user"),
        "{err}"
    );
    // Two sessions live: no selector is ambiguous for a token kind.
    daemon.shared.lock().unwrap().auth.replace(AuthSession {
        username: Some("Other#1".into()),
        ..AuthSession::default()
    });
    let err = daemon.resolve_account("characters", None).unwrap_err();
    assert!(err.message.contains("several accounts"), "{err}");
    assert_eq!(
        daemon.resolve_account("characters", Some("other")),
        Ok(Some("Other#1".into()))
    );
    daemon.shared.lock().unwrap().auth.by_account.clear();
    // No session: an auth-required kind is refused at submit; a kind
    // that never sends with a token simply has no account.
    let err = daemon.resolve_account("characters", None).unwrap_err();
    assert!(err.message.contains("not logged in"), "{err}");
    assert_eq!(daemon.resolve_account("sleep", None), Ok(None));
    let err = daemon
        .resolve_account("characters", Some("Other#1"))
        .unwrap_err();
    assert!(err.message.contains("not logged in"), "{err}");
}

#[tokio::test]
async fn a_token_is_refused_once_the_session_no_longer_matches_the_job() {
    let (daemon, _, _log_path) = test_daemon("http://127.0.0.1:1");
    {
        let mut s = daemon.shared.lock().unwrap();
        s.auth.one_mut().access_token = Some("at-live".into());
        s.auth.one_mut().access_expires_at = Some(SystemTime::now() + Duration::from_secs(3600));
    }
    // The job's account has a session: its live token, no refresh.
    let (token, user) = daemon
        .valid_access_token(Some("old-user"), false, None)
        .await
        .unwrap();
    assert_eq!((token.as_str(), user.as_str()), ("at-live", "old-user"));
    // Another account logging in changes nothing for old-user's jobs.
    daemon.shared.lock().unwrap().auth.replace(AuthSession {
        username: Some("new-user".into()),
        access_token: Some("at-new".into()),
        access_expires_at: Some(SystemTime::now() + Duration::from_secs(3600)),
        ..AuthSession::default()
    });
    let (token, _) = daemon
        .valid_access_token(Some("old-user"), false, None)
        .await
        .unwrap();
    assert_eq!(token, "at-live");
    // No selector with two sessions live: refused, never guessed.
    let err = daemon
        .valid_access_token(None, false, None)
        .await
        .unwrap_err();
    assert!(
        err.message.contains("several accounts") && err.message.contains("--account"),
        "{err}"
    );
    // old-user logs out: its jobs fail at token time, before any send.
    daemon.logout(Some("old-user")).unwrap();
    let err = daemon
        .valid_access_token(Some("old-user"), false, None)
        .await
        .unwrap_err();
    assert!(err.message.contains("no session for \"old-user\""), "{err}");
    // The other session is untouched, and is now the sole one.
    assert!(daemon.valid_access_token(None, false, None).await.is_ok());
}

#[test]
fn session_keys_follow_renames_so_replace_finds_the_right_session() {
    let mut sessions = Sessions::with(AuthSession {
        username: Some("A#1".into()),
        refresh_token: Some("rt-a".into()),
        ..AuthSession::default()
    });
    let before = sessions.find("A#1").unwrap().generations;
    // A refresh reported a new name: the session moves under it.
    sessions.rename("A#1", "B#2");
    assert!(sessions.find("A#1").is_none());
    assert_eq!(
        sessions.find("B#2").unwrap().refresh_token.as_deref(),
        Some("rt-a")
    );
    assert_eq!(sessions.usernames(), vec!["B#2".to_string()]);
    // A login as B now replaces that one session (generations advance)
    // instead of leaving a stale twin behind.
    sessions.replace(AuthSession {
        username: Some("B#2".into()),
        refresh_token: Some("rt-b".into()),
        ..AuthSession::default()
    });
    assert_eq!(sessions.by_account.len(), 1);
    let after = sessions.find("B#2").unwrap();
    assert_eq!(after.refresh_token.as_deref(), Some("rt-b"));
    assert_ne!(after.generations.session, before.session);
}

#[test]
fn a_response_is_filed_under_the_job_account_not_the_session() {
    let (mut daemon, _, _log_path) = test_daemon("http://127.0.0.1:1");
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    Arc::get_mut(&mut daemon).unwrap().store_dir = Some(dir.clone());
    // Session is "old-user"; the job was submitted as "A#1" (a login
    // happened in between). The body lands in A#1's file.
    daemon
        .record(
            Some("A#1"),
            "stashes",
            &json!({ "league": "Standard" }),
            &json!({ "stashes": [ { "id": "t1", "name": "T", "type": "PremiumStash" } ] }),
        )
        .unwrap();
    assert!(acquisition_store::account_path(&dir, "A#1").exists());
    assert!(!acquisition_store::account_path(&dir, "old-user").exists());
    // No account at all: nothing recorded, and the job fails keeping its
    // body (C144), an error noted.
    let failed = daemon
        .record(None, "stashes", &json!({}), &json!({ "stashes": [] }))
        .unwrap_err();
    assert!(
        matches!(
            &failed,
            Outcome::Failure {
                payload: Some(_),
                ..
            }
        ),
        "{failed:?}"
    );
    assert!(
        daemon
            .shared
            .lock()
            .unwrap()
            .errors
            .iter()
            .any(|(_, m)| m.contains("no account"))
    );
    // The daemon holds the store open, and Windows removes no open file.
    drop(daemon);
}
