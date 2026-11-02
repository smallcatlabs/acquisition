//! The session layer: account selection, the single-flight refresh, and
//! logout or re-login during a refresh.
//!
//! C158, a scenario crosses the daemon's frontend boundary only: a
//! refresh here is `Request::AuthCheck` through the door, against the
//! delayed token server, whose `arrived` and `release` are the
//! scenario's barriers; a second caller is a request on a connection of
//! its own (`Door::spawn_request`), and the waiters joined are the
//! runtime gone quiet under the held refresh (`Door::quiet`). What a
//! scenario reads is the protocol's answers, the journal, its own
//! credential store and the account index on disk. A session's expiry is
//! written from the door's clock's wall.
//!
//! The component tests, each with the boundary it needs:
//! `a_selector_matching_several_sessions_is_ambiguous` and
//! `session_keys_follow_renames_so_replace_finds_the_right_session`
//! (`Sessions` as a struct);
//! `realm_is_a_segment_before_league_or_name_and_pc_is_silent`
//! (`route_for` and `validate_apply` as functions);
//! `submit_resolves_the_account_against_the_live_session`
//! (`resolve_account`); `a_token_is_refused_once_the_session_no_longer_matches_the_job`
//! (fabricated live tokens, their expiries from the clock's wall);
//! `a_response_is_filed_under_the_job_account_not_the_session`
//! (`record` as a function);
//! `authentication_completes_before_the_api_send_takes_a_gate_slot`
//! (the choke point and `execute_inner` called directly);
//! `abandoned_refresh_owner_completes_all_waiters_and_allows_retry` and
//! `abandoned_refresh_owner_without_waiters_allows_retry` (a refresh
//! owner aborted by the test, which production never does: the guard's
//! property); and
//! `reauthentication_during_refresh_keeps_new_session_and_rejects_old_completion`
//! (a re-login as the same account while its refresh is held: the
//! exchange's POST would wait behind the held refresh on the token
//! route's one permit, so no boundary interleaving lands before the
//! release). They run on the harness's daemon and the manual clock.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::door::*;
use super::harness::*;
use crate::daemon::refresh::{REFRESH_OWNER_ABANDONED, SESSION_CHANGED_DURING_REFRESH};
use crate::daemon::*;
use crate::transport::{Reply, reply};
use tokio::sync::oneshot;

/// C51: a bare name two live sessions share is refused, never guessed
/// (review finding 2026-09-10); the full name and the uuid select. A
/// component test: `Sessions` as a struct.
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
struct AbandonThenSuccessTokenResponses {
    first_arrived: oneshot::Receiver<()>,
    release_first: oneshot::Sender<()>,
    requests: Arc<Mutex<Vec<String>>>,
}

/// Two refreshes answered with `success_body`: the first held, its
/// arrival announced, until `release_first` fires (the release kept by
/// the fixture, so it stands when the first request's future is
/// dropped); the second answered at once. Each request's form is
/// recorded; a request that is not a `POST /token` is a fault that fails
/// it. In-process (C161).
fn abandon_then_success_token_responses(
    success_body: String,
) -> (InProcess, AbandonThenSuccessTokenResponses) {
    let (first_arrived_tx, first_arrived) = oneshot::channel();
    let (release_first, release_first_rx) = oneshot::channel();
    let first = Mutex::new(Some(first_arrived_tx));
    let release = Arc::new(tokio::sync::Mutex::new(release_first_rx));
    let requests = Arc::new(Mutex::new(Vec::new()));
    let captured = requests.clone();
    let transport = InProcess::new(&Hold::new(), move |request: mockggg::HttpRequest| {
        let token = (request.method.as_str(), request.path.as_str()) == ("POST", "/token");
        if token {
            captured.lock().unwrap().push(request.body.clone());
        }
        let arrived = first.lock().unwrap().take();
        let (release, answer) = (
            release.clone(),
            reply("200 OK", TOKEN_HEADERS, &success_body),
        );
        let words = format!(
            "the token fixture answers POST /token, and {} {} came",
            request.method, request.path
        );
        async move {
            if !token {
                return Reply::Failed(words);
            }
            if let Some(arrived) = arrived {
                arrived.send(()).unwrap();
                let _ = (&mut *release.lock().await).await;
            }
            answer
        }
    });
    let responses = AbandonThenSuccessTokenResponses {
        first_arrived,
        release_first,
        requests,
    };
    (transport, responses)
}

fn tokens(access_token: &str, refresh_token: &str, username: &str) -> auth::TokenResponse {
    auth::TokenResponse {
        access_token: access_token.into(),
        refresh_token: refresh_token.into(),
        expires_in: 3600,
        username: username.into(),
    }
}

/// The component tests' daemon, on the manual clock, saving into a
/// recording credential store, recording into `store_dir` when given:
/// its one session `old-user`'s, its access token expired at the clock's
/// wall, its refresh token `rt-old`. With `journal`, it journals beside
/// its log as every harness daemon does, and the test ends through
/// `finish_harness`, so the journal written is read by the oracles;
/// without, it writes none, for a test whose sends are fabricated
/// (driven through the choke point on a route of the test's own), over
/// which no oracle of the daemon's wire may run.
fn session_daemon(
    base: &str,
    transport: InProcess,
    store_dir: Option<&std::path::Path>,
    journal: bool,
) -> (Arc<Daemon>, Arc<RecordingCredentialStore>, TestPath) {
    let clock = Arc::new(ParkingClock::new());
    let expired = clock.wall();
    let log_path = test_log_path();
    let log = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&log_path)
        .unwrap();
    let rails = Arc::new(Rails::with_config_and_clock(
        RailsConfig {
            journal_path: journal.then(|| journal_of(&log_path)),
            ..RailsConfig::default()
        },
        clock.clone(),
    ));
    let credentials = Arc::new(RecordingCredentialStore::default());
    let mut builder = Daemon::builder(Provider::mock(base), clock, credentials.clone(), log)
        .transport(Box::new(transport))
        .rails(rails)
        .log_path(log_path.to_str().map(str::to_string))
        .events_capacity(16)
        .sessions(Sessions::with(AuthSession {
            access_token: Some("at-expired".into()),
            access_expires_at: Some(expired),
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
        }));
    if let Some(store_dir) = store_dir {
        builder = builder.store_dir(store_dir.to_path_buf());
    }
    (builder.build(), credentials, log_path)
}

/// The scenarios' one session: `test-user`, its access token expired at
/// `clock`'s wall, its refresh token `rt-old` (what the delayed token
/// server checks), its keyring `ok`, its uuid `uuid`.
fn expired_session(clock: &ParkingClock, uuid: Option<&str>) -> Sessions {
    Sessions::with(AuthSession {
        access_token: Some("at-expired".into()),
        access_expires_at: Some(clock.wall()),
        refresh_token: Some("rt-old".into()),
        username: Some("test-user".into()),
        uuid: uuid.map(str::to_string),
        keyring: "ok".into(),
        ..AuthSession::default()
    })
}

/// A door on the delayed token server answering `status` with `body`,
/// facing `expired_session`, saving into `store`.
fn refresh_door(
    status: &'static str,
    body: String,
    store: &Arc<RecordingCredentialStore>,
    uuid: Option<&str>,
) -> DoorBuilder {
    let clock = Arc::new(ParkingClock::new());
    let sessions = expired_session(&clock, uuid);
    Door::open(
        Server::Token(Token::Delayed(status, body)),
        clock,
        store.clone(),
    )
    .sessions(sessions)
}

/// `AuthCheck` of the sole session, as `acq auth check` sends it.
fn check() -> Request {
    Request::AuthCheck { account: None }
}

/// A spawned request's answer, awaited under the hang ceiling.
async fn answer(request: tokio::task::JoinHandle<Response>) -> Response {
    tokio::time::timeout(HANG, request)
        .await
        .expect("the request was answered")
        .expect("the request's task ran to its end")
}

/// An answer's stable fields, as `auth status` prints them: logged in,
/// the account, the access token's expiry. Never the keyring word, which
/// reads `saving` for a caller answered before the persist lands.
fn stable(answer: &Response) -> (bool, Option<String>, Option<u64>) {
    match answer {
        Response::Auth {
            logged_in,
            username,
            access_expires_in_seconds,
            ..
        } => (*logged_in, username.clone(), *access_expires_in_seconds),
        other => panic!("an auth check answered {other:?}"),
    }
}

/// Whether every live session's keyring word is `ok`: the persist landed.
fn saved(answer: &AuthAnswer) -> bool {
    !answer.accounts.is_empty() && answer.accounts.iter().all(|a| a.keyring == "ok")
}

/// C87: concurrent callers of an expired session share one refresh: the
/// owner's `AuthCheck` reaches the token server and is held there; three
/// more join it while it is held (the runtime gone quiet under the held
/// send is the word that they joined); released, every caller is
/// answered alike, one POST is journaled, and one save lands.
#[test]
fn concurrent_expiry_has_one_refresh_owner_and_shared_waiter_result() {
    let store = Arc::new(RecordingCredentialStore::default());
    let credentials = store.clone();
    refresh_door(
        "200 OK",
        token_body("at-rotated", "rt-rotated", "test-user"),
        &store,
        None,
    )
    .run(|mut door| async move {
        let (arrived, release) = door.held();
        let owner = door.spawn_request(check());
        tokio::time::timeout(HANG, arrived)
            .await
            .expect("the refresh reached the server")
            .unwrap();
        let waiters: Vec<_> = (0..3).map(|_| door.spawn_request(check())).collect();
        door.quiet().await;
        release.send(()).unwrap();

        let owner = stable(&answer(owner).await);
        assert!(owner.0, "logged in");
        assert_eq!(owner.1.as_deref(), Some("test-user"));
        assert_eq!(owner.2, Some(3600), "the rotated token's lifetime");
        for waiter in waiters {
            assert_eq!(stable(&answer(waiter).await), owner, "the owner's result");
        }
        door.await_auth(saved).await;
        assert_eq!(credentials.saves.lock().unwrap().len(), 1);
        assert_eq!(door.journal_methods(), ["POST"], "one refresh on the wire");
        door.finish().await;
    });
}

/// Realm on the wire (C58, C59): pc is omitted, so a pc
/// URL and route label are byte-identical whether the param is absent
/// or explicit — every live send so far stays the same; any other
/// realm is a segment before the league or name on the URL and a
/// suffix on the label, so it gets its own probe; a realm a family
/// does not take renders nothing and is refused at admission. A
/// component test: `route_for` and `validate_apply` as functions, beside
/// the admission's refusals of the same realms.
#[test]
fn realm_is_a_segment_before_league_or_name_and_pc_is_silent() {
    let (daemon, _, log_path) = session_daemon("http://mock", refused(), None, true);
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
    finish_harness(None, &log_path);
}

async fn wait_for_refresh_waiters(daemon: &Daemon, expected: usize) {
    tokio::time::timeout(HANG, async {
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

/// A refresh needed by an API send completes before that send takes a
/// gate slot, so the token request can use the slot that remains. A
/// component test: the choke point and `execute_inner` are called
/// directly, beside a held send on a route of the test's own. Its daemon
/// journals nothing: that send is fabricated (no probe precedes it), and
/// the wire contract's oracles judge the daemon's own sends alone.
#[tokio::test]
async fn authentication_completes_before_the_api_send_takes_a_gate_slot() {
    let (hold_arrived_tx, hold_arrived) = oneshot::channel();
    let (release_hold, release_hold_rx) = oneshot::channel();
    let (token_arrived_tx, token_arrived) = oneshot::channel();
    // The three requests in the order they must come, each answered as
    // it arrives: the held send's head at once, its body on the release;
    // then the refresh; then the API's GET. A request out of that order
    // is refused in words that name it. In-process (C161).
    type Step = Box<dyn FnOnce() -> Reply + Send>;
    let steps: Vec<(&str, &str, Step)> = vec![
        (
            "GET",
            "/hold",
            Box::new(move || {
                let head = concat!(
                    "HTTP/1.1 200 OK\r\n",
                    "X-Rate-Limit-Policy: hold-policy\r\n",
                    "X-Rate-Limit-Rules: Ip\r\n",
                    "X-Rate-Limit-Ip: 60:30:30\r\n",
                    "X-Rate-Limit-Ip-State: 1:30:0\r\n",
                    "Content-Type: application/json\r\n",
                    "Content-Length: 2\r\n",
                    "Connection: close\r\n\r\n{}",
                );
                hold_arrived_tx.send(()).unwrap();
                Reply::Held(head.to_string(), release_hold_rx)
            }),
        ),
        (
            "POST",
            "/token",
            Box::new(move || {
                token_arrived_tx.send(()).unwrap();
                reply(
                    "200 OK",
                    TOKEN_HEADERS,
                    &token_body("at-live", "rt-rotated", "test-user"),
                )
            }),
        ),
        (
            "GET",
            "/character",
            Box::new(|| {
                let api_headers = concat!(
                    "X-Rate-Limit-Policy: character-list-request-limit\r\n",
                    "X-Rate-Limit-Rules: Account\r\n",
                    "X-Rate-Limit-Account: 2:10:60,5:300:300\r\n",
                    "X-Rate-Limit-Account-State: 1:10:0,1:300:0\r\n",
                );
                reply("200 OK", api_headers, r#"{"characters":[]}"#)
            }),
        ),
    ];
    let steps = Mutex::new(std::collections::VecDeque::from(steps));
    let seen: Arc<Mutex<Vec<String>>> = Arc::default();
    let took = seen.clone();
    let transport = InProcess::new(&Hold::new(), move |request: mockggg::HttpRequest| {
        took.lock()
            .unwrap()
            .push(format!("{} {}", request.method, request.path));
        let answer = match steps.lock().unwrap().pop_front() {
            Some((method, path, step))
                if (method, path) == (&request.method[..], &request.path[..]) =>
            {
                step()
            }
            _ => Reply::Failed(format!(
                "the fixture expected its three requests in order, and {} {} came",
                request.method, request.path
            )),
        };
        async move { answer }
    });

    let base = BASE;
    let (daemon, _, _log_path) = session_daemon(BASE, transport, None, false);
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

    tokio::time::timeout(HANG, token_arrived)
        .await
        .expect("refresh can use the remaining gate slot before the API send")
        .unwrap();
    assert!(matches!(
        api.await.unwrap().unwrap(),
        Outcome::Success { .. }
    ));
    release_hold.send(()).unwrap();
    hold.await.unwrap().unwrap();
    assert_eq!(
        *seen.lock().unwrap(),
        ["GET /hold", "POST /token", "GET /character"]
    );
}

/// N2, an owner that leaves without publishing: a refresh owner aborted
/// while its POST is held completes every waiter with the abandonment,
/// leaves the session's tokens and generations as they were, and a retry
/// refreshes. A component test: the owner is aborted by the test, which
/// production never does (`RefreshOwnerGuard`'s property); the waiters
/// joined are read from the flight's receiver count.
#[tokio::test]
async fn abandoned_refresh_owner_completes_all_waiters_and_allows_retry() {
    let (transport, responses) =
        abandon_then_success_token_responses(token_body("at-retried", "rt-retried", "retry-user"));
    let (daemon, store, log_path) = session_daemon(BASE, transport, None, true);
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
            tokio::time::timeout(HANG, waiter)
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
        assert_eq!(
            s.auth
                .one()
                .refresh_token
                .as_ref()
                .map(auth::Secret::exposed_for_test),
            Some("rt-old")
        );
        assert_eq!(
            s.auth
                .one()
                .access_token
                .as_ref()
                .map(auth::Secret::exposed_for_test),
            Some("at-expired")
        );
    }
    assert!(store.saves.lock().unwrap().is_empty());

    responses.release_first.send(()).unwrap();
    let retry = tokio::time::timeout(HANG, daemon.valid_access_token(None, false, None))
        .await
        .expect("retry after owner abandonment completed")
        .unwrap();
    assert_eq!(retry, ("at-retried".into(), "retry-user".into()));
    let requests = responses.requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    assert!(
        requests
            .iter()
            .all(|body| body.contains("refresh_token=rt-old"))
    );
    assert_eq!(store.saves.lock().unwrap().len(), 1);
    finish_harness(None, &log_path);
}

/// N2 with no waiter: an aborted refresh owner closes its flight, so a
/// retry refreshes, and that successful refresh keeps the session's
/// generation and advances both token generations. A component test:
/// the owner is aborted by the test, as above, and the generations have
/// no boundary read.
#[tokio::test]
async fn abandoned_refresh_owner_without_waiters_allows_retry() {
    let (transport, responses) =
        abandon_then_success_token_responses(token_body("at-retried", "rt-retried", "retry-user"));
    let (daemon, store, log_path) = session_daemon(BASE, transport, None, true);
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
        assert_eq!(
            s.auth
                .one()
                .refresh_token
                .as_ref()
                .map(auth::Secret::exposed_for_test),
            Some("rt-old")
        );
        assert_eq!(
            s.auth
                .one()
                .access_token
                .as_ref()
                .map(auth::Secret::exposed_for_test),
            Some("at-expired")
        );
    }
    assert!(store.saves.lock().unwrap().is_empty());

    responses.release_first.send(()).unwrap();
    let retry = tokio::time::timeout(HANG, daemon.valid_access_token(None, false, None))
        .await
        .expect("retry after owner abandonment completed")
        .unwrap();
    assert_eq!(retry, ("at-retried".into(), "retry-user".into()));
    // A successful refresh rotates the tokens within the session: the
    // session's generation stays, both token generations advance (the
    // property `successful_refresh_rotation_…`'s scenario cannot read).
    let after = daemon.shared.lock().unwrap().auth.one().generations;
    assert_eq!(after.session, before.session);
    assert_ne!(after.access_token, before.access_token);
    assert_ne!(after.refresh_token, before.refresh_token);
    let requests = responses.requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    assert!(
        requests
            .iter()
            .all(|body| body.contains("refresh_token=rt-old"))
    );
    assert_eq!(store.saves.lock().unwrap().len(), 1);
    finish_harness(None, &log_path);
}

/// A refresh that rotates the token persists the new one: one save, the
/// rotated refresh token under the name the token response gave (a
/// rename the session follows), which `auth status` reports; one POST
/// journaled.
#[test]
fn successful_refresh_rotation_advances_token_generations_and_persists_new_token() {
    let store = Arc::new(RecordingCredentialStore::default());
    let credentials = store.clone();
    refresh_door(
        "200 OK",
        token_body("at-rotated", "rt-rotated", "rotated-user"),
        &store,
        None,
    )
    .run(|mut door| async move {
        let (arrived, release) = door.held();
        let refresh = door.spawn_request(check());
        tokio::time::timeout(HANG, arrived)
            .await
            .expect("the refresh reached the server")
            .unwrap();
        release.send(()).unwrap();
        assert_eq!(
            stable(&answer(refresh).await).1.as_deref(),
            Some("rotated-user")
        );
        let after = door.await_auth(saved).await;
        assert_eq!(after.username.as_deref(), Some("rotated-user"));
        assert_eq!(
            saved_refresh_tokens(&credentials.saves.lock().unwrap()),
            [(
                "acq-mock".into(),
                "rt-rotated".into(),
                "rotated-user".into()
            )]
        );
        assert_eq!(door.journal_methods(), ["POST"]);
        door.finish().await;
    });
}

/// C24 with C87: a rejected refresh is shared by the callers on its
/// flight and leaves the session as it was. A caller that joined the held
/// flight gets the owner's 400; a caller after the rejection gets the
/// dead grant's refusal ("token refresh disabled") and sends no second
/// POST; `daemon status` reports the mark; nothing is saved.
#[test]
fn refresh_failure_is_shared_and_leaves_token_state_unchanged() {
    let store = Arc::new(RecordingCredentialStore::default());
    let credentials = store.clone();
    refresh_door(
        "400 Bad Request",
        r#"{"error":"invalid_grant"}"#.into(),
        &store,
        None,
    )
    .run(|mut door| async move {
        let before = door.await_auth(|_| true).await;
        let (arrived, release) = door.held();
        let owner = door.spawn_request(check());
        tokio::time::timeout(HANG, arrived)
            .await
            .expect("the refresh reached the server")
            .unwrap();
        let waiter = door.spawn_request(check());
        door.quiet().await;
        release.send(()).unwrap();

        let owner = answer(owner).await;
        let Response::Error { kind, message } = &owner else {
            panic!("the rejection is the owner's answer: {owner:?}")
        };
        assert_eq!(*kind, ErrorKind::NotLoggedIn);
        assert!(message.contains("400 Bad Request"), "{message}");
        let waiter = answer(waiter).await;
        assert!(
            matches!(&waiter, Response::Error { kind: k, message: m } if k == kind && m == message),
            "the waiter shares the owner's answer: {waiter:?}"
        );
        let sends = door.journal();
        assert_eq!(sends.len(), 1, "{sends:?}");
        assert_eq!(
            (&sends[0]["method"], &sends[0]["status"]),
            (&json!("POST"), &json!(400))
        );

        let late = door.request(check()).await;
        let Response::Error { kind, message } = &late else {
            panic!("a dead grant refuses the late caller: {late:?}")
        };
        assert_eq!(*kind, ErrorKind::NotLoggedIn);
        assert!(
            message.contains("token refresh disabled for test-user")
                && message.contains("refresh token rejected with HTTP 400"),
            "{message}"
        );
        assert_eq!(door.journal().len(), 1, "no second POST");
        let rails = match door.request(Request::DaemonStatus).await {
            Response::DaemonStatus { rails, .. } => rails,
            other => panic!("the daemon's status answered {other:?}"),
        };
        assert_eq!(
            rails.refresh_failed.as_deref(),
            Some("test-user: refresh token rejected with HTTP 400")
        );
        let after = door.await_auth(|_| true).await;
        assert_eq!(
            (
                after.logged_in,
                after.username.clone(),
                after.access_expires_in_seconds,
                after.keyring.clone()
            ),
            (
                before.logged_in,
                before.username.clone(),
                before.access_expires_in_seconds,
                before.keyring.clone()
            ),
            "the session as it was"
        );
        assert!(credentials.saves.lock().unwrap().is_empty());
        door.finish().await;
    });
}

/// A logout while the session's refresh is held rejects the refresh's
/// completion when it lands: the caller is told the session changed, the
/// session stays gone, and the keyring is cleared and never written.
#[test]
fn logout_during_refresh_rejects_stale_completion_in_memory_and_keyring() {
    let store = Arc::new(RecordingCredentialStore::default());
    let credentials = store.clone();
    refresh_door(
        "200 OK",
        token_body("at-stale", "rt-stale", "test-user"),
        &store,
        None,
    )
    .run(|mut door| async move {
        let (arrived, release) = door.held();
        let refresh = door.spawn_request(check());
        tokio::time::timeout(HANG, arrived)
            .await
            .expect("the refresh reached the server")
            .unwrap();
        let logout = door.request(Request::AuthLogout { account: None }).await;
        assert!(matches!(logout, Response::Ack { .. }), "{logout:?}");
        release.send(()).unwrap();

        let refused = answer(refresh).await;
        let Response::Error { kind, message } = &refused else {
            panic!("the stale completion is refused: {refused:?}")
        };
        assert_eq!(*kind, ErrorKind::NotLoggedIn);
        assert_eq!(message, SESSION_CHANGED_DURING_REFRESH);
        let after = door.await_auth(|_| true).await;
        assert!(
            !after.logged_in && after.accounts.is_empty(),
            "the session is gone"
        );
        assert_eq!(
            credentials.cleared.lock().unwrap().as_slice(),
            ["test-user"]
        );
        assert!(credentials.saves.lock().unwrap().is_empty());
        assert_eq!(door.journal_methods(), ["POST"]);
        door.finish().await;
    });
}

/// A component test: a re-login as the same account while its refresh
/// is held. Through the door the exchange's POST would wait behind the
/// held refresh on the token route's one permit, so no boundary
/// interleaving lands the login before the release; the login completes
/// here through the private `complete_login`.
#[tokio::test]
async fn reauthentication_during_refresh_keeps_new_session_and_rejects_old_completion() {
    // A re-login as the *same* account while its refresh is in flight:
    // the new session replaces the old one (generations advance), and
    // the stale completion is rejected. Another account's login would
    // not touch this session at all (see the two-session tests).
    let (transport, record) = crate::transport::delayed_token(
        &Hold::new(),
        "200 OK",
        token_body("at-stale", "rt-stale", "old-user"),
    );
    let (arrived, release, ends) = (record.arrived, record.release, record.ends);
    let (daemon, store, log_path) = session_daemon(BASE, transport, None, true);
    let before = daemon.shared.lock().unwrap().auth.one().generations;

    let refresh_daemon = daemon.clone();
    let refresh =
        tokio::spawn(async move { refresh_daemon.valid_access_token(None, false, None).await });
    arrived.await.unwrap();
    let flow_generation = daemon.begin_auth_flow();
    let staged = daemon
        .stage_auth_flow(
            flow_generation,
            tokens("at-reauth", "rt-reauth", "old-user"),
        )
        .expect("current flow stages");
    assert!(daemon.complete_login(flow_generation, &staged, "u-old"));
    release.send(()).unwrap();

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
        assert_eq!(
            s.auth
                .one()
                .access_token
                .as_ref()
                .map(auth::Secret::exposed_for_test),
            Some("at-reauth")
        );
        assert_eq!(
            s.auth
                .one()
                .refresh_token
                .as_ref()
                .map(auth::Secret::exposed_for_test),
            Some("rt-reauth")
        );
        assert_eq!(s.auth.one().username.as_deref(), Some("old-user"));
        assert_ne!(s.auth.one().generations.session, before.session);
    }
    assert_eq!(
        saved_refresh_tokens(&store.saves.lock().unwrap()),
        [("acq-mock".into(), "rt-reauth".into(), "old-user".into())]
    );
    assert_whole(&ends);
    finish_harness(None, &log_path);
}

/// A refresh that renames the account updates the existing index
/// mapping in place (the session carries its uuid, so `record_login`
/// follows it) — never a second, uuid-less entry that would make both
/// the username and uuid selectors ambiguous.
#[test]
fn a_refresh_rename_updates_the_index_mapping_in_place() {
    let store = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    Index::load(&store_dir)
        .unwrap()
        .record_login("test-user", "u-1", true, 1, &[])
        .unwrap();
    let dir = store_dir.to_path_buf();
    refresh_door(
        "200 OK",
        token_body("at-new", "rt-new", "NewName#2"),
        &store,
        Some("u-1"),
    )
    .store_dir(store_dir.to_path_buf())
    .run(|mut door| async move {
        let (arrived, release) = door.held();
        let refresh = door.spawn_request(check());
        tokio::time::timeout(HANG, arrived)
            .await
            .expect("the refresh reached the server")
            .unwrap();
        release.send(()).unwrap();
        assert_eq!(
            stable(&answer(refresh).await).1.as_deref(),
            Some("NewName#2")
        );
        let after = door.await_auth(saved).await;
        assert_eq!(
            after
                .accounts
                .iter()
                .map(|a| a.username.as_str())
                .collect::<Vec<_>>(),
            ["NewName#2"],
            "the session moved with the name"
        );
        let index = Index::load(&dir).unwrap();
        assert_eq!(index.entries().len(), 1, "one entry, moved — no twin");
        let entry = index.get("NewName#2").expect("the entry follows the name");
        assert_eq!(entry.uuid.as_deref(), Some("u-1"));
        door.finish().await;
    });
}

// ---- multi-account step (2): the job's account is what runs ---------

/// C51: a submit's account resolves against the live sessions. A
/// component test: `resolve_account` as a function.
#[test]
fn submit_resolves_the_account_against_the_live_session() {
    let (daemon, _, log_path) = session_daemon(BASE, refused(), None, true);
    // Session is "old-user" (session_daemon).
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
    finish_harness(None, &log_path);
}

/// A job's token is its own account's, and none once that account has
/// logged out. A component test: fabricated live tokens, their expiries
/// from the clock's wall, read through `valid_access_token`.
#[tokio::test]
async fn a_token_is_refused_once_the_session_no_longer_matches_the_job() {
    let (daemon, _, log_path) = session_daemon(BASE, refused(), None, true);
    {
        let mut s = daemon.shared.lock().unwrap();
        s.auth.one_mut().access_token = Some("at-live".into());
        s.auth.one_mut().access_expires_at = Some(daemon.choke.wall() + Duration::from_secs(3600));
    }
    // The job's account has a session: its live token, no refresh.
    let (token, user) = daemon
        .valid_access_token(Some("old-user"), false, None)
        .await
        .unwrap();
    assert_eq!(
        (token.exposed_for_test(), user.as_str()),
        ("at-live", "old-user")
    );
    // Another account logging in changes nothing for old-user's jobs.
    daemon.shared.lock().unwrap().auth.replace(AuthSession {
        username: Some("new-user".into()),
        access_token: Some("at-new".into()),
        access_expires_at: Some(daemon.choke.wall() + Duration::from_secs(3600)),
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
    finish_harness(None, &log_path);
}

/// A rename moves the session's key, so a later login replaces the one
/// session. A component test: `Sessions` as a struct.
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
        sessions
            .find("B#2")
            .unwrap()
            .refresh_token
            .as_ref()
            .map(auth::Secret::exposed_for_test),
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
    assert_eq!(
        after
            .refresh_token
            .as_ref()
            .map(auth::Secret::exposed_for_test),
        Some("rt-b")
    );
    assert_ne!(after.generations.session, before.session);
}

/// A response is recorded under the job's account, and a job with none
/// fails keeping its body (C144). A component test: `record` as a
/// function.
#[test]
fn a_response_is_filed_under_the_job_account_not_the_session() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let (daemon, _, log_path) = session_daemon(BASE, refused(), Some(&dir), true);
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
    finish_harness(None, &log_path);
    drop(daemon);
}
