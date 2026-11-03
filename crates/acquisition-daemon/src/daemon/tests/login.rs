//! The login flow: the session registered only when the profile's uuid
//! lands, and every way a login fails whole.
//!
//! C158, a scenario crosses the daemon's frontend boundary only: a login
//! here is `AuthStart`, the browser's callback played by the scenario
//! ([`Door::browser`], the frontend's side of the boundary), and
//! `await_auth` over `AuthStatus`; a logout is `AuthLogout`; what a
//! scenario reads is the protocol's answers, the journal, the fixture's
//! record of the wire, its own credential store and the account index on
//! disk (the store's read API, C12). A scenario that starts logged out
//! opens with no session: the door's default legacy session reads as
//! logged in.
//!
//! The component tests, each with the boundary it needs:
//! `a_superseded_login_cannot_complete_and_leaves_no_zombie` (a
//! completion superseded by generation: `complete_login`'s return and the
//! staging slot have no boundary read);
//! `c144_a_client_profile_job_in_the_staging_window_never_gets_the_staged_token`
//! (the same account's staging window, which a client job through the
//! door would wait out behind the held login profile on its scheduling
//! key); `a_callback_connection_that_sends_nothing_does_not_block_the_login`
//! and `a_callback_read_past_its_bound_is_dropped_and_the_flow_waits`
//! (`wait_callback`, the callback listener's own reads); `c144_a_forget_logs_only_the_clear_that_worked_as_cleared`
//! (the daemon log, which the door does not read);
//! `c144_restore_reads_through_the_credential_store_and_reports_a_failed_load`
//! (`restore_sessions` as a function);
//! `c144_a_login_finalised_under_an_unwritable_index_leaves_nothing_to_restore`
//! (a freeze between the profile's write and the landing);
//! `c144_a_login_cannot_land_between_a_forgets_clear_and_its_mark` (a
//! store hook calling back into the daemon); and
//! `the_retry_word_is_quoted_for_the_declared_shell` (a table).
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::door::*;
use super::harness::*;
use crate::daemon::login::wait_callback;
use crate::daemon::*;
use acquisition_protocol::protocol::KeyringOutcome;
use acquisition_protocol::status::RailsStatus;

// ---- tracer step 2: uuid-at-login (C50) ------------------------------

fn login_tokens(access: &str, user: &str) -> auth::TokenResponse {
    auth::TokenResponse {
        access_token: access.into(),
        refresh_token: format!("rt-{user}").into(),
        expires_in: 3600,
        username: user.into(),
    }
}

/// A daemon wired for the login's component tests: a recording
/// credential store, a scratch store directory, no session, on the
/// manual clock.
fn login_daemon() -> (
    Arc<Daemon>,
    Arc<RecordingCredentialStore>,
    TestPath,
    TestPath,
) {
    let credentials = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let log_path = test_log_path();
    let log = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&log_path)
        .unwrap();
    let daemon = Daemon::builder(
        Provider::mock(BASE),
        Arc::new(ParkingClock::new()),
        credentials.clone(),
        log,
    )
    .transport(Box::new(refused()))
    .store_dir(store_dir.to_path_buf())
    .log_path(log_path.to_str().map(str::to_string))
    .build();
    (daemon, credentials, store_dir, log_path)
}

/// The code exchange answered `status` with `body` and the token route's
/// headers.
fn exchange(status: &'static str, body: String) -> ScriptedResponse {
    ScriptedResponse {
        method: "POST",
        path: "/token",
        status,
        headers: TOKEN_HEADERS.into(),
        body,
    }
}

/// The login's `GET /profile` answered `status` with `body`, without
/// rate headers as GGG serves it (C32).
fn profile(status: &'static str, body: String) -> ScriptedResponse {
    ScriptedResponse {
        method: "GET",
        path: "/profile",
        status,
        headers: String::new(),
        body,
    }
}

/// A profile body naming `user` with its uuid.
fn profile_of(user: &str) -> String {
    json!({ "uuid": format!("uuid-{user}"), "name": user }).to_string()
}

/// A logged-out door on `server`, saving into `credentials`, recording
/// into `store_dir`, on `clock`.
fn logged_out(
    server: Server,
    clock: &Arc<ParkingClock>,
    credentials: &Arc<RecordingCredentialStore>,
    store_dir: &std::path::Path,
) -> DoorBuilder {
    Door::open(server, clock.clone(), credentials.clone())
        .sessions(Sessions::default())
        .store_dir(store_dir.to_path_buf())
}

/// `AuthStart` through the door: the authorize URL the user opens.
async fn authorize(door: &mut Door) -> String {
    match door.request(Request::AuthStart).await {
        Response::AuthUrl { authorize_url } => authorize_url,
        other => panic!("a login's start answered {other:?}"),
    }
}

/// A login approved in the browser with `code` (the account server's
/// account), awaited to its end.
async fn log_in(door: &mut Door, code: &str) -> AuthAnswer {
    let url = authorize(door).await;
    let (status, page) = door.browser(&code_callback(&url, code)).await;
    assert_eq!(status, 200, "{page}");
    assert!(page.contains("Logged in"), "{page}");
    door.await_auth(|answer| !answer.pending).await
}

/// `DaemonStatus`'s rails, as a frontend reads them.
async fn rails_status(door: &mut Door) -> RailsStatus {
    match door.request(Request::DaemonStatus).await {
        Response::DaemonStatus { rails, .. } => rails,
        other => panic!("the daemon's status answered {other:?}"),
    }
}

/// `Dashboard`'s errors, newest first.
async fn dashboard_errors(door: &mut Door) -> Vec<String> {
    match door.request(Request::Dashboard).await {
        Response::Dashboard { errors, .. } => errors.into_iter().map(|e| e.message).collect(),
        other => panic!("the dashboard answered {other:?}"),
    }
}

/// The accounts `AuthStatus` lists, by name and keyring word.
fn accounts(answer: &AuthAnswer) -> Vec<(String, String)> {
    answer
        .accounts
        .iter()
        .map(|a| (a.username.clone(), a.keyring.clone()))
        .collect()
}

/// A login is not a login until the profile fetch delivers the account
/// uuid: until then nothing is registered — no session, no keyring
/// entry, no index row — and when it lands, everything is, with the
/// uuid (the account server's, per username).
#[test]
fn login_registers_the_session_only_when_the_profile_uuid_lands() {
    let clock = Arc::new(ParkingClock::new());
    let credentials = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let dir = store_dir.to_path_buf();
    let store = credentials.clone();
    logged_out(
        Server::Account(("GET", "/profile", 0)),
        &clock,
        &credentials,
        &store_dir,
    )
    .run(|mut door| async move {
        let (arrived, release) = door.held();
        let url = authorize(&mut door).await;
        let (status, page) = door.browser(&code_callback(&url, "Alice#1234")).await;
        assert_eq!(status, 200, "{page}");
        tokio::time::timeout(HANG, arrived)
            .await
            .expect("the login's profile reached the server")
            .unwrap();

        // The profile held: the flow is still pending and nothing is
        // registered.
        let staged = door.await_auth(|_| true).await;
        assert!(staged.pending, "{staged:?}");
        assert!(!staged.logged_in, "{staged:?}");
        assert_eq!((staged.login_ok, staged.login_error), (None, None));
        assert!(staged.accounts.is_empty(), "no session exists yet");
        assert!(store.saves.lock().unwrap().is_empty());
        assert!(!acquisition_store::index_path(&dir).exists());

        // Released: the session visible with its keyring written, the
        // index row with the uuid, the flow's result naming the account.
        release.send(()).unwrap();
        let landed = door.await_auth(|answer| !answer.pending).await;
        assert_eq!(landed.login_ok.as_deref(), Some("Alice#1234"), "{landed:?}");
        assert_eq!(landed.login_error, None);
        assert!(landed.logged_in);
        assert_eq!(
            accounts(&landed),
            [("Alice#1234".to_string(), "ok".to_string())]
        );
        assert_eq!(store.saves.lock().unwrap().len(), 1);
        let index = Index::load(&dir).unwrap();
        let entry = index.get("Alice#1234").expect("indexed at login");
        assert_eq!(entry.uuid.as_deref(), Some("uuid-Alice#1234"));
        assert!(entry.persisted);

        // A re-login as the same account lands the same uuid, and the
        // index keeps one entry per account.
        let again = log_in(&mut door, "Alice#1234").await;
        assert_eq!(again.login_ok.as_deref(), Some("Alice#1234"), "{again:?}");
        let index = Index::load(&dir).unwrap();
        assert_eq!(index.entries().len(), 1);
        assert_eq!(
            index.get("Alice#1234").unwrap().uuid.as_deref(),
            Some("uuid-Alice#1234")
        );
        let wire = door.sent();
        assert_eq!(
            wire,
            [
                sent("Alice#1234", "POST"),
                sent("Alice#1234", "GET"),
                sent("Alice#1234", "POST"),
                sent("Alice#1234", "GET"),
            ]
        );
        assert_eq!(door.journal_methods(), wire_methods_of(&wire));
        door.finish().await;
    });
}

/// C50: a login whose profile fetch fails fails whole: no provisional
/// identity — the staged tokens are dropped, no keyring entry is
/// minted, the index is untouched, and the flow's terminal result is
/// the failure, which the daemon's errors carry.
#[test]
fn a_login_whose_profile_fetch_fails_fails_whole() {
    let clock = Arc::new(ParkingClock::new());
    let credentials = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let dir = store_dir.to_path_buf();
    let store = credentials.clone();
    let script = vec![
        exchange("200 OK", token_body("at-x", "rt-x", "Alice#1234")),
        profile(
            "404 Not Found",
            r#"{"error":{"code":1,"message":"Resource not found"}}"#.into(),
        ),
    ];
    logged_out(Server::Script(script), &clock, &credentials, &store_dir).run(
        |mut door| async move {
            let url = authorize(&mut door).await;
            let (status, page) = door.browser(&code_callback(&url, "code-x")).await;
            assert_eq!(status, 200, "{page}");
            let ended = door.await_auth(|answer| !answer.pending).await;
            assert!(!ended.logged_in && ended.login_ok.is_none(), "{ended:?}");
            let error = ended.login_error.expect("the login failed");
            assert!(error.contains("profile fetch"), "{error}");
            assert!(ended.accounts.is_empty(), "no session appeared");
            assert!(store.saves.lock().unwrap().is_empty());
            assert!(!acquisition_store::index_path(&dir).exists());
            let errors = dashboard_errors(&mut door).await;
            assert!(
                errors
                    .iter()
                    .any(|e| e.contains("login failed for Alice#1234")),
                "the failure is reported: {errors:?}"
            );
            door.finish().await;
        },
    );
}

/// An aborted *re-login* leaves the previously registered session
/// exactly as it was (staging never touches the session map), and the
/// flow's terminal result reports the failure even though `logged_in`
/// stays true — so `acq auth` cannot mistake the surviving session for
/// this login succeeding. The profile fetch really fails: the old
/// session's token is read as its expiry, the seeded one's and never the
/// exchange's.
#[test]
fn an_aborted_relogin_keeps_the_old_session_and_reports_the_failure() {
    let clock = Arc::new(ParkingClock::new());
    let credentials = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let store = credentials.clone();
    let parking = clock.clone();
    let expires = clock.wall() + Duration::from_secs(7200);
    let script = vec![
        exchange("200 OK", token_body("at-y", "rt-y", "Alice#1234")),
        profile(
            "404 Not Found",
            r#"{"error":{"code":1,"message":"Resource not found"}}"#.into(),
        ),
    ];
    logged_out(Server::Script(script), &clock, &credentials, &store_dir)
        .sessions(Sessions::with(AuthSession {
            username: Some("Alice#1234".into()),
            uuid: Some("uuid-Alice#1234".into()),
            access_token: Some("at-x".into()),
            access_expires_at: Some(expires),
            refresh_token: Some("rt-x".into()),
            keyring: "ok".into(),
            ..AuthSession::default()
        }))
        .run(|mut door| async move {
            let clock = parking;
            let ended = log_in(&mut door, "code-y").await;
            assert!(ended.logged_in, "the old session is still live");
            assert!(
                ended.login_ok.is_none(),
                "the failed flow claims no success"
            );
            let error = ended.login_error.clone().expect("the login failed");
            assert!(error.contains("profile fetch"), "{error}");
            assert_eq!(ended.username.as_deref(), Some("Alice#1234"));
            assert_eq!(
                accounts(&ended),
                [("Alice#1234".to_string(), "ok".to_string())]
            );
            let seeded = expires.duration_since(clock.wall()).unwrap().as_secs();
            assert!(seeded > 3600, "the seeded expiry outlasts the exchange's");
            assert_eq!(
                ended.access_expires_in_seconds,
                Some(seeded),
                "the old session's token is untouched: the staged tokens never reached it"
            );
            assert!(store.saves.lock().unwrap().is_empty(), "nothing saved");
            door.finish().await;
        });
}

/// A rejected refresh grant marks the account so the dead token is
/// never re-sent; only a completed login clears the mark, since until
/// then the keyring still held the rejected token. The mark is read
/// once the landing is done: the landing publishes the login before it
/// clears the mark.
#[test]
fn a_completed_login_clears_the_dead_grant_mark() {
    let clock = Arc::new(ParkingClock::new());
    let credentials = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let expired = clock.wall();
    let script = vec![
        exchange("400 Bad Request", r#"{"error":"invalid_grant"}"#.into()),
        exchange("200 OK", token_body("at-x", "rt-x", "Alice#1234")),
        profile("200 OK", profile_of("Alice#1234")),
    ];
    logged_out(Server::Script(script), &clock, &credentials, &store_dir)
        .sessions(Sessions::with(AuthSession {
            username: Some("Alice#1234".into()),
            access_token: Some("at-old".into()),
            access_expires_at: Some(expired),
            refresh_token: Some("rt-old".into()),
            keyring: "ok".into(),
            ..AuthSession::default()
        }))
        .run(|mut door| async move {
            let check = door
                .request(Request::AuthCheck {
                    account: Some("Alice#1234".into()),
                })
                .await;
            let Response::Error { kind, message } = &check else {
                panic!("a rejected refresh is the check's answer: {check:?}")
            };
            assert_eq!(*kind, ErrorKind::NotLoggedIn, "{message}");
            let marked = rails_status(&mut door).await.refresh_failed;
            assert!(
                marked.as_deref().is_some_and(|m| m.contains("Alice#1234")),
                "{marked:?}"
            );

            let landed = log_in(&mut door, "code-x").await;
            assert_eq!(landed.login_ok.as_deref(), Some("Alice#1234"), "{landed:?}");
            door.settle().await;
            assert_eq!(rails_status(&mut door).await.refresh_failed, None);
            door.finish().await;
        });
}

/// A login the account index cannot record fails whole: the session
/// never becomes visible and no keyring entry is left (the index cannot
/// hold the name, so none is saved) — a session without its uuid
/// mapping could neither be restored nor find its annotation file.
#[test]
fn a_login_the_index_cannot_record_fails_whole_and_leaves_no_keyring_entry() {
    let clock = Arc::new(ParkingClock::new());
    let credentials = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    // A directory where the index file should be: every index read and
    // write fails, while the profile's facts file opens and records (a
    // store that cannot record fails the profile job first, C144).
    std::fs::create_dir_all(acquisition_store::index_path(&store_dir)).unwrap();
    let store = credentials.clone();
    let script = vec![
        exchange("200 OK", token_body("at-x", "rt-x", "Alice#1234")),
        profile("200 OK", profile_of("Alice#1234")),
    ];
    logged_out(Server::Script(script), &clock, &credentials, &store_dir).run(
        |mut door| async move {
            let ended = log_in(&mut door, "code-x").await;
            assert!(!ended.logged_in && ended.login_ok.is_none(), "{ended:?}");
            let error = ended.login_error.expect("the login failed");
            assert!(error.contains("account index"), "{error}");
            assert!(ended.accounts.is_empty(), "the session never appeared");
            // C144: the index could not hold the name first, so no
            // keyring write happened and there is nothing to roll back.
            assert!(store.saves.lock().unwrap().is_empty());
            assert!(store.entries.lock().unwrap().is_empty());
            door.finish().await;
        },
    );
}

/// A superseded flow can neither stage nor complete, and its staged
/// tokens do not linger. A component test: a completion superseded by
/// generation has no boundary read (`complete_login`'s return and the
/// staging slot).
#[tokio::test]
async fn a_superseded_login_cannot_complete_and_leaves_no_zombie() {
    let (daemon, credentials, _store_dir, _log_path) = login_daemon();

    let g1 = daemon.begin_auth_flow();
    daemon
        .stage_auth_flow(g1, login_tokens("at-x.Alice#1234", "Alice#1234"))
        .expect("current flow stages");
    // The user restarted the login before the profile landed: the new
    // flow owns the login state from here.
    let g2 = daemon.begin_auth_flow();
    assert!(!daemon.complete_login(g1, "Alice#1234", "u-stale"));
    {
        let s = daemon.shared.lock().unwrap();
        assert!(s.auth.by_account.is_empty(), "no session appeared");
        assert!(s.auth.staging.is_none(), "no staged tokens linger");
        assert_eq!(s.auth.pending, Some(g2), "the new flow is untouched");
        assert!(
            s.auth.flow_result.is_none(),
            "the new flow's result is its own"
        );
    }
    assert!(credentials.saves.lock().unwrap().is_empty());
    // A stale token exchange cannot stage either.
    assert!(
        daemon
            .stage_auth_flow(g1, login_tokens("at-y.Alice#1234", "Alice#1234"))
            .is_none()
    );
}

/// C129 over C24, the login arm: Cloudflare's challenge on the code
/// exchange is no rejected grant. Driven from the browser's callback:
/// the flow fails with words that say Cloudflare answered and the daemon
/// stopped, not with Cloudflare's page or a word to log in again; the
/// block mark is set, and wire == journal == one POST.
#[test]
fn a_cloudflare_block_on_the_code_exchange_is_no_rejected_grant() {
    let clock = Arc::new(ParkingClock::new());
    let credentials = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let challenge = ScriptedResponse {
        method: "POST",
        path: "/token",
        ..ScriptedResponse::cloudflare_403_challenge()
    };
    logged_out(
        Server::Script(vec![challenge]),
        &clock,
        &credentials,
        &store_dir,
    )
    .run(|mut door| async move {
        let ended = log_in(&mut door, "code-x").await;
        let error = ended.login_error.expect("the login failed");
        assert!(
            error.contains("Cloudflare answered in GGG's place"),
            "{error}"
        );
        assert!(error.contains("reset-tripwire"), "{error}");
        assert!(!error.contains("acq auth"), "{error}");
        assert!(!error.contains("Just a moment"), "{error}");
        let halt = rails_status(&mut door)
            .await
            .halted
            .expect("the block mark is set");
        assert!(
            halt.contains("Cloudflare answered in GGG's place: 403 on POST oauth-token"),
            "{halt}"
        );
        assert_eq!(door.journal_methods(), ["POST"]);
        door.finish().await;
    });
}

/// C129, the login arm's announcement: a block on the code exchange is
/// announced among the daemon's errors when it lands, as a block on any
/// other send is, and is not announced again as current after
/// `reset-tripwire`: the next block's own cause is the one announced.
#[test]
fn a_block_on_the_code_exchange_is_announced_when_it_lands() {
    let clock = Arc::new(ParkingClock::new());
    let credentials = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let challenge = ScriptedResponse {
        method: "POST",
        path: "/token",
        ..ScriptedResponse::cloudflare_403_challenge()
    };
    let script = vec![
        challenge,
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse::cloudflare_403_challenge(),
    ];
    logged_out(Server::Script(script), &clock, &credentials, &store_dir).run(
        |mut door| async move {
            let ended = log_in(&mut door, "code-x").await;
            assert!(ended.login_error.is_some(), "{ended:?}");
            let logged = dashboard_errors(&mut door).await;
            assert!(
                logged
                    .iter()
                    .any(|e| e.starts_with("SENDS STOPPED: ")
                        && e.contains("403 on POST oauth-token")),
                "the exchange's block is announced: {logged:?}"
            );

            match door.request(Request::ResetTripwire).await {
                Response::Ack { .. } => {}
                other => panic!("the reset answered {other:?}"),
            }
            let before = dashboard_errors(&mut door).await.len();
            let id = door.submit("fetch", None).await;
            let (info, _) = door.await_terminal(id).await;
            assert_eq!(info.state, JobState::Failed);
            let logged = dashboard_errors(&mut door).await;
            // Newest first: the errors since the reset lead.
            let since = &logged[..logged.len() - before];
            let stopped: Vec<&String> = since
                .iter()
                .filter(|e| e.starts_with("SENDS STOPPED: "))
                .collect();
            assert!(
                stopped.len() == 1 && stopped[0].contains("403 on GET fetch"),
                "after the reset, only the next block's own cause is announced: {since:?}"
            );
            door.finish().await;
        },
    );
}

/// C25, the login arm: a flow a newer login superseded sends nothing.
/// Its callback still lands (the browser was already sent), but its code
/// exchange is turned away before the gate admits it: no POST reaches
/// the wire, the newer flow stays pending, and the journal agrees. The
/// account server records any POST that reaches it and holds none (its
/// hold matches no request); the stale end notifies nothing, so the
/// runtime is let settle.
#[test]
fn a_superseded_login_flow_sends_no_code_exchange() {
    let clock = Arc::new(ParkingClock::new());
    let credentials = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    logged_out(
        Server::Account(("POST", "/never", 0)),
        &clock,
        &credentials,
        &store_dir,
    )
    .run(|mut door| async move {
        let first = authorize(&mut door).await;
        let _second = authorize(&mut door).await;
        let (status, page) = door.browser(&code_callback(&first, "test-user")).await;
        assert_eq!(status, 200, "{page}");
        door.settle().await;

        let waiting = door.await_auth(|_| true).await;
        assert!(waiting.pending, "the newer flow stays pending: {waiting:?}");
        assert_eq!((waiting.login_ok, waiting.login_error), (None, None));
        assert!(!waiting.logged_in);
        assert!(
            door.sent().is_empty(),
            "the superseded flow sent its exchange"
        );
        assert!(door.journal_methods().is_empty());
        door.finish().await;
    });
}

// ---- the callback listener (#68) --------------------------------------

/// The login's bounds race their future against the daemon's clock
/// (`within_on`, C160) as tokio's `timeout` races it: a bound's expiry
/// never waits behind its future's cooperative budget. On the system
/// clock, whose sleep spends that budget, a bound of zero (the expiry
/// forced, C137) over a future that exhausts the budget on every poll and
/// never completes (a receive loop on a channel kept full, as a callback
/// body kept readable or job events kept coming would) expires. A
/// component test of a function: the race itself, on a plain runtime.
#[tokio::test]
async fn a_bound_expires_though_its_future_spends_every_poll_budget() {
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<u32>();
    for message in 0..1_000 {
        sender.send(message).unwrap();
    }
    let busy = async move {
        loop {
            let message = receiver.recv().await.expect("the sender is held");
            sender.send(message).expect("the receiver is held");
        }
    };
    let bounded = tokio::time::timeout(
        HANG,
        crate::ratelimit::within_on(&SystemClock, Duration::ZERO, busy),
    )
    .await
    .expect("the bound expired behind a future that spends its budget");
    assert!(bounded.is_err(), "the busy future never completes");
}

/// A callback connection that sends nothing holds nothing up: with a
/// read bound of an hour, a silent connection and then the browser's
/// redirect on a second connection, `wait_callback` returns the code
/// within the test's guard. Green for one reason: a listener that read
/// its connections one at a time would wait the hour on the silent one,
/// so the redirect could only be answered by a task of its own. A
/// component test: the callback listener's own reads.
#[tokio::test]
async fn a_callback_connection_that_sends_nothing_does_not_block_the_login() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let waiting = tokio::spawn(async move {
        wait_callback(
            listener,
            "st-expected",
            Duration::from_secs(3600),
            Arc::new(SystemClock),
        )
        .await
    });

    let silent = tokio::net::TcpStream::connect(addr).await.unwrap();
    let redirect = format!("http://{addr}{CALLBACK_PATH}?code=code-x&state=st-expected");
    let answered = tokio::time::timeout(HANG, reqwest::get(&redirect))
        .await
        .expect("the redirect was answered while the silent connection was open")
        .unwrap();
    assert!(answered.status().is_success(), "{answered:?}");
    let code = tokio::time::timeout(HANG, waiting)
        .await
        .expect("the flow ended on the redirect")
        .unwrap();
    assert_eq!(code, Ok("code-x".into()));
    drop(silent);
}

/// The read bound's expiry (C137, pinned by forcing it): a connection
/// that has sent nothing when its bound expires is dropped without a
/// response, and the flow waits on; the listener still accepts. Green
/// for one reason: the second connection's EOF, read while the flow's
/// task still runs, is written only by a task that accepted it and
/// dropped it at its own expiry; a connection left in the backlog reads
/// nothing until the listener closes. A component test: the callback
/// listener's own reads.
#[tokio::test]
async fn a_callback_read_past_its_bound_is_dropped_and_the_flow_waits() {
    use tokio::io::AsyncReadExt as _;
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let waiting = tokio::spawn(async move {
        wait_callback(
            listener,
            "st-expected",
            Duration::ZERO,
            Arc::new(SystemClock),
        )
        .await
    });

    let mut silent = tokio::net::TcpStream::connect(addr).await.unwrap();
    let mut byte = [0u8; 1];
    let read = tokio::time::timeout(HANG, silent.read(&mut byte))
        .await
        .expect("the daemon closed the silent connection");
    assert_eq!(read.ok(), Some(0), "closed without a response");
    assert!(!waiting.is_finished(), "the flow still waits");

    // The listener is still accepting: a second silent connection is
    // taken and dropped at the zero bound, so it too reads EOF; one left
    // in the backlog would read nothing while the listener stands.
    let mut second = tokio::net::TcpStream::connect(addr).await.unwrap();
    let read = tokio::time::timeout(HANG, second.read(&mut byte))
        .await
        .expect("the daemon accepted and closed the second connection");
    assert_eq!(read.ok(), Some(0), "closed without a response");
    assert!(!waiting.is_finished(), "the flow still waits");
    waiting.abort();
}

/// A denial is reported in the browser's words: a redirect with the
/// flow's state and `error=access_denied` ends the flow failed, its
/// result carrying the error as OAuth's code and a word on what to do,
/// never "state mismatch"; the browser reads a page that says so. #113:
/// the free-text description is text received, and these words reach
/// the log, so it is not carried.
#[test]
fn a_denied_login_is_reported_in_the_browsers_words() {
    let clock = Arc::new(ParkingClock::new());
    let credentials = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    logged_out(Server::None, &clock, &credentials, &store_dir).run(|mut door| async move {
        let url = authorize(&mut door).await;
        let (status, page) = door.browser(&denied_callback(&url)).await;
        assert_eq!(status, 200);
        assert!(page.contains("denied in the browser"), "{page}");
        assert!(page.contains("acq auth"), "{page}");

        let ended = door.await_auth(|answer| !answer.pending).await;
        assert!(!ended.logged_in);
        assert_eq!(ended.login_ok, None);
        let error = ended.login_error.expect("the flow failed");
        assert!(error.contains("denied in the browser"), "{error}");
        assert!(error.contains("access_denied"), "{error}");
        assert!(!error.contains("The user denied"), "{error}");
        assert!(error.contains("acq auth"), "{error}");
        assert!(!error.contains("state mismatch"), "{error}");
        door.finish().await;
    });
}

/// A request on the callback path without the flow's state is answered
/// 400 and ignored: the flow waits, and the browser's own redirect then
/// completes the login. So is one with the state and neither a code nor
/// an error. A local process that finds the port can neither end the
/// flow nor take the code.
#[test]
fn a_callback_with_another_state_is_answered_and_the_login_waits() {
    let clock = Arc::new(ParkingClock::new());
    let credentials = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    logged_out(Server::Mock, &clock, &credentials, &store_dir).run(|mut door| async move {
        let url = authorize(&mut door).await;
        for stray in stray_callbacks(&url) {
            let (status, page) = door.browser(&stray).await;
            assert_eq!(status, 400, "{stray}");
            assert!(page.contains("return to the terminal"), "{page}");
            let waiting = door.await_auth(|_| true).await;
            assert!(waiting.pending, "the flow still waits after {stray}");
            assert!(!waiting.logged_in, "{stray}");
            assert_eq!(
                (waiting.login_ok, waiting.login_error),
                (None, None),
                "{stray}"
            );
        }

        // The mock approves as its own user: its 302 followed to the
        // daemon's callback.
        let approve = url.replacen("/authorize?", "/approve?", 1);
        let (status, page) = door.browser(&approve).await;
        assert_eq!(status, 200, "{page}");
        let ended = door.await_auth(|answer| !answer.pending).await;
        assert!(ended.logged_in, "{ended:?}");
        assert_eq!(ended.login_ok.as_deref(), Some(mockggg::USERNAME));
        door.finish().await;
    });
}

/// C137, the browser wait's expiry forced: a daemon whose `login_within`
/// is zero fails the flow before any redirect, its result saying the
/// browser did not complete the login within the bound and to run the
/// login again, and the flow is no longer pending.
#[test]
fn an_expired_browser_wait_fails_the_flow_with_its_words() {
    let clock = Arc::new(ParkingClock::new());
    let credentials = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    logged_out(Server::None, &clock, &credentials, &store_dir)
        .login_within(Duration::ZERO)
        .run(|mut door| async move {
            authorize(&mut door).await;
            let ended = door.await_auth(|answer| !answer.pending).await;
            assert!(!ended.logged_in);
            assert_eq!(ended.login_ok, None);
            let error = ended.login_error.expect("the flow failed");
            assert!(
                error.contains("the browser did not complete the login within 0 s"),
                "{error}"
            );
            assert!(error.contains("run `acq auth` again"), "{error}");
            door.finish().await;
        });
}

/// C131, the login arm: a code exchange answered 200 with its tokens but
/// no rate-limit header at all installs them, and the login completes
/// on them; the exchange stays a journaled protocol failure. What the
/// session holds is read where a frontend meets it: the keyring entry the
/// login saved (C135, the access token beside the refresh token).
#[test]
fn a_headerless_code_exchange_2xx_installs_its_tokens() {
    let clock = Arc::new(ParkingClock::new());
    let credentials = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let store = credentials.clone();
    let parking = clock.clone();
    let grant = ScriptedResponse {
        headers: String::new(),
        ..exchange("200 OK", token_body("at-new", "rt-rotated", "test-user"))
    };
    let script = vec![grant, profile("200 OK", profile_of("test-user"))];
    logged_out(Server::Script(script), &clock, &credentials, &store_dir).run(
        |mut door| async move {
            let ended = log_in(&mut door, "code-x").await;
            assert_eq!(
                (
                    ended.logged_in,
                    ended.login_ok.as_deref(),
                    ended.login_error.as_deref()
                ),
                (true, Some("test-user"), None)
            );
            assert_eq!(
                accounts(&ended),
                [("test-user".to_string(), "ok".to_string())]
            );
            assert_eq!(
                saved_refresh_tokens(&store.saves.lock().unwrap())
                    .iter()
                    .map(|(_, token, user)| (token.as_str(), user.as_str()))
                    .collect::<Vec<_>>(),
                [("rt-rotated", "test-user")]
            );
            let saved = store.entries.lock().unwrap()["test-user"].clone();
            let entry = auth::KeyringEntry::from_secret(&saved.as_str().into(), parking.wall());
            assert_eq!(
                entry
                    .access_token
                    .as_ref()
                    .map(auth::Secret::exposed_for_test),
                Some("at-new")
            );
            let sends = door.journal();
            assert_eq!(sends[0]["method"], "POST");
            assert_eq!(sends[0]["ok"], false, "{}", sends[0]);
            door.finish().await;
        },
    );
}

/// C47 with C134, the login arm: a code exchange whose `expires_in` no
/// clock can add fails the login as the provider's error, stages and
/// saves nothing, gives back its count for the exit's drain, and leaves
/// the daemon answering. Before, the sum panicked under `shared` in the
/// exchange's task and wedged the daemon. The count given back is read
/// at the exit: a stop leaves before the drain's bound on the clock,
/// which a pending exchange would hold (C133); that the daemon still
/// answers is every request after the failure.
#[test]
fn an_out_of_range_expires_in_is_a_token_error() {
    let clock = Arc::new(ParkingClock::new());
    let credentials = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let store = credentials.clone();
    let parking = clock.clone();
    let grant = exchange(
        "200 OK",
        json!({
            "access_token": "at-huge",
            "refresh_token": "rt-huge",
            "expires_in": u64::MAX,
            "username": "test-user",
        })
        .to_string(),
    );
    logged_out(
        Server::Script(vec![grant]),
        &clock,
        &credentials,
        &store_dir,
    )
    .exit_bound(HELD)
    .life(IDLE_SHUTDOWN)
    .run(|mut door| async move {
        let clock = parking;
        let ended = log_in(&mut door, "code-x").await;
        assert_eq!((ended.logged_in, ended.login_ok.as_deref()), (false, None));
        let error = ended.login_error.expect("the login failed");
        assert!(error.contains("token exchange failed"), "{error}");
        assert!(error.contains("lifetime"), "{error}");
        assert!(
            !error.contains("at-huge") && !error.contains("rt-huge"),
            "{error}"
        );
        assert!(store.saves.lock().unwrap().is_empty(), "nothing saved");
        assert_eq!(door.journal_methods(), ["POST"], "one POST");

        let stopped = clock.now();
        door.stop();
        door.await_leave().await;
        let drained = clock.now() - stopped;
        assert!(
            drained < HELD,
            "a clean drain leaves before its bound: {drained:?}"
        );
        door.finish().await;
    });
}

/// C144 (a staged login token serves the login's own profile job alone),
/// issue #71: in the window between the token exchange and the login's
/// completion, a client's `profile` job for the account being logged in
/// sends on the account's session — a refresh, then its GET with the
/// session's token — never with the staged token, and writes nothing of
/// the login's. A client's profile job for an account with no session
/// fails `not_logged_in` and sends nothing. The login's own profile job,
/// submitted after, carries the staged token. A component test: the
/// window it pins is the same account's (the client's profile job for
/// the account whose token is staged), and a client job for that account
/// through the door would wait behind the held login profile on its
/// scheduling key; a boundary twin needs an interleaving the door does
/// not draw.
#[tokio::test]
async fn c144_a_client_profile_job_in_the_staging_window_never_gets_the_staged_token() {
    let (transport, record) = account_transport(("GET", "/never", 0));
    let sent_log = record.sent;
    let (mut daemon, log_path) =
        test_daemon(Boundary::Function, transport, Arc::new(ParkingClock::new()));
    logged_in(&mut daemon);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());

    let generation = daemon.begin_auth_flow();
    daemon
        .stage_auth_flow(generation, login_tokens("at-staged", "test-user"))
        .expect("current flow stages");
    let staged = |daemon: &Daemon| {
        daemon
            .shared
            .lock()
            .unwrap()
            .auth
            .staging
            .as_ref()
            .map(|st| st.username.clone())
    };
    assert_eq!(staged(&daemon).as_deref(), Some("test-user"));

    let client = daemon
        .submit(
            "profile".into(),
            json!({}),
            0,
            "cli:1".into(),
            Some("test-user".into()),
        )
        .unwrap();
    let (info, outcome) = wait_terminal(&daemon, client).await;
    assert_eq!(info.state, JobState::Done, "{outcome:?}");
    assert_eq!(
        sent_log.lock().unwrap().as_slice(),
        [sent("test-user", "POST"), sent("test-user", "GET")],
        "the client's fetch ran on the session: its refresh, then its GET"
    );
    {
        let s = daemon.shared.lock().unwrap();
        let staging = s.auth.staging.as_ref().expect("the login is still staged");
        assert_eq!(
            staging.profile_job, None,
            "the client's job was not the login's"
        );
        assert_eq!(
            s.auth.pending,
            Some(generation),
            "the login is still pending"
        );
    }

    // An account with no session: the staged slot is not its fallback.
    let generation_carol = daemon.begin_auth_flow();
    daemon
        .stage_auth_flow(generation_carol, login_tokens("at-staged", "carol"))
        .expect("current flow stages");
    let carol = daemon
        .submit(
            "profile".into(),
            json!({}),
            0,
            "cli:1".into(),
            Some("carol".into()),
        )
        .unwrap();
    let (info, outcome) = wait_terminal(&daemon, carol).await;
    assert_eq!(info.state, JobState::Failed, "{outcome:?}");
    assert_eq!(
        sent_log.lock().unwrap().len(),
        2,
        "nothing was sent for carol"
    );

    // The login's own profile job carries the staged token.
    let login = daemon
        .submit_login_profile(generation_carol, "carol")
        .unwrap();
    let (info, outcome) = wait_terminal(&daemon, login).await;
    assert_eq!(info.state, JobState::Done, "{outcome:?}");
    assert_eq!(
        sent_log.lock().unwrap().last(),
        Some(&sent("staged", "GET")),
        "the login's own fetch sent with the staged token"
    );
    finish_harness(dispatcher, &log_path);
}

// ---- C144: every credential-store result reaches the caller ----------

/// The keyring family's door: `store` holding `test-user`'s entry
/// `rt-old`, and an account index in `store_dir` that knows `test-user`
/// as `uuid-test-user` (the account server's uuid for it), persisted;
/// the session live when `live`, else none.
fn keyring_door(
    server: Server,
    store: &Arc<RecordingCredentialStore>,
    store_dir: &std::path::Path,
    live: bool,
) -> DoorBuilder {
    store
        .entries
        .lock()
        .unwrap()
        .insert("test-user".into(), "rt-old".into());
    Index::load(store_dir)
        .unwrap()
        .record_login("test-user", "uuid-test-user", true, 1, &[])
        .unwrap();
    let sessions = if live {
        logged_in_sessions()
    } else {
        Sessions::default()
    };
    Door::open(server, Arc::new(ParkingClock::new()), store.clone())
        .sessions(sessions)
        .store_dir(store_dir.to_path_buf())
}

/// `AuthLogout` of `account` (the sole session's when `None`), as the
/// CLI sends it.
async fn logout(door: &mut Door, account: Option<&str>) -> Response {
    door.request(Request::AuthLogout {
        account: account.map(str::to_string),
    })
    .await
}

/// Whether the index on disk marks `account` persisted.
fn persisted(store_dir: &std::path::Path, account: &str) -> bool {
    Index::load(store_dir)
        .unwrap()
        .get(account)
        .expect("the account is indexed")
        .persisted
}

/// C144, issue #61: a logout whose keyring clear fails answers that on
/// the wire — `internal`, in words that say the session was dropped and
/// the entry was not, and the request that tries again — where it used to
/// answer success. The index marks the account not persisted either way,
/// so no later start restores the session; the retry, a logout of the
/// now not-live account, clears the entry once the keyring works.
#[test]
fn c144_a_logout_whose_keyring_clear_fails_says_so_on_the_wire() {
    let store = Arc::new(RecordingCredentialStore::failing(
        "clear",
        "keychain locked",
    ));
    let store_dir = TestPath::dir();
    let dir = store_dir.to_path_buf();
    let credentials = store.clone();
    keyring_door(Server::None, &store, &store_dir, true).run(|mut door| async move {
        let answer = logout(&mut door, None).await;
        let Response::Error { kind, message } = &answer else {
            panic!("the failed clear is the answer: {answer:?}");
        };
        assert_eq!(*kind, ErrorKind::Internal);
        for fact in [
            "logged out test-user (session dropped, not restored at start), but",
            "its keyring entry could not be cleared: keychain locked",
            "`acq auth logout --account test-user` tries again",
        ] {
            assert!(message.contains(fact), "{fact:?} in {message}");
        }
        let after = door.await_auth(|_| true).await;
        assert!(
            !after.logged_in && after.accounts.is_empty(),
            "the session is dropped: {after:?}"
        );
        assert!(!persisted(&dir, "test-user"));
        assert!(
            credentials
                .entries
                .lock()
                .unwrap()
                .contains_key("test-user")
        );

        // The keyring works again: the retry reaches the entry.
        *credentials.fail_clear.lock().unwrap() = None;
        let retry = logout(&mut door, Some("test-user")).await;
        assert!(
            matches!(
                retry,
                Response::Ack {
                    keyring: Some(KeyringOutcome::Cleared)
                }
            ),
            "{retry:?}"
        );
        assert!(credentials.entries.lock().unwrap().is_empty());
        door.finish().await;
    });
}

/// C144: a logout whose clear works is acknowledged with the keyring's
/// outcome as data (the review of the h1 fix round, its fifth finding: no
/// frontend reads it from a sentence), and the entry is gone.
#[test]
fn a_logout_clears_the_keyring_entry_and_acks() {
    let store = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let credentials = store.clone();
    keyring_door(Server::None, &store, &store_dir, true).run(|mut door| async move {
        let answer = logout(&mut door, None).await;
        assert!(
            matches!(
                answer,
                Response::Ack {
                    keyring: Some(KeyringOutcome::Cleared)
                }
            ),
            "{answer:?}"
        );
        assert!(credentials.entries.lock().unwrap().is_empty());
        assert_eq!(
            credentials.cleared.lock().unwrap().as_slice(),
            ["test-user"]
        );
        door.finish().await;
    });
}

/// C144, issue #61: forget clears the entry whether or not the index
/// marks it persisted — a failed save or a failed logout clear leaves an
/// entry the mark does not show — and a failed clear is the answer,
/// never "cleared". What its log says of each clear is the component
/// case beside this one.
#[test]
fn c144_forget_clears_an_unmarked_entry_and_reports_a_failed_clear() {
    let store = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let credentials = store.clone();
    let door = keyring_door(Server::None, &store, &store_dir, false);
    Index::load(&store_dir)
        .unwrap()
        .set_persisted("test-user", false)
        .unwrap();
    door.run(|mut door| async move {
        let answer = logout(&mut door, Some("test-user")).await;
        assert!(
            matches!(
                answer,
                Response::Ack {
                    keyring: Some(KeyringOutcome::Cleared)
                }
            ),
            "{answer:?}"
        );
        assert!(
            credentials.entries.lock().unwrap().is_empty(),
            "an unmarked entry is cleared"
        );

        credentials
            .entries
            .lock()
            .unwrap()
            .insert("test-user".into(), "rt-old".into());
        *credentials.fail_clear.lock().unwrap() = Some("keychain locked".into());
        let answer = logout(&mut door, Some("test-user")).await;
        let Response::Error { kind, message } = &answer else {
            panic!("the failed clear is the answer: {answer:?}");
        };
        assert_eq!(*kind, ErrorKind::Internal);
        assert!(
            message.contains("its keyring entry could not be cleared: keychain locked"),
            "{message}"
        );
        door.finish().await;
    });
}

/// C144, issue #61, the log's half of the forget above: only the clear
/// that worked is logged as cleared. A component test: the daemon log has
/// no boundary surface (the door reads no log).
#[tokio::test]
async fn c144_a_forget_logs_only_the_clear_that_worked_as_cleared() {
    let (daemon, store, dir, log_path) = login_daemon();
    store
        .entries
        .lock()
        .unwrap()
        .insert("test-user".into(), "rt-old".into());
    Index::load(&dir)
        .unwrap()
        .record_login("test-user", "u-1", false, 1, &[])
        .unwrap();

    daemon.logout(Some("test-user")).unwrap();
    store
        .entries
        .lock()
        .unwrap()
        .insert("test-user".into(), "rt-old".into());
    *store.fail_clear.lock().unwrap() = Some("keychain locked".into());
    daemon.logout(Some("test-user")).unwrap_err();
    let log = std::fs::read_to_string(&log_path).unwrap();
    assert_eq!(
        log.matches("forgot account test-user (keyring entry cleared)")
            .count(),
        1,
        "only the clear that worked is logged as cleared:\n{log}"
    );
}

/// C144, issue #61: restore reads through the credential store, so a load
/// that fails reaches the keyring status `auth status` reports and the
/// log, and the account is not restored; an entry the store holds comes
/// back as a session; an entry it lacks is logged. A component test:
/// `restore_sessions` as a function, over a directory and a store.
#[test]
fn c144_restore_reads_through_the_credential_store_and_reports_a_failed_load() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    {
        let mut index = Index::load(&dir).unwrap();
        index.record_login("alice", "u-a", true, 2, &[]).unwrap();
        index.record_login("bob", "u-b", true, 1, &[]).unwrap();
    }

    let store = RecordingCredentialStore::default();
    store
        .entries
        .lock()
        .unwrap()
        .insert("alice".into(), "rt-a".into());
    let mut log = Vec::new();
    let sessions =
        start::restore_sessions(&store, "acq-mock", &dir, None, SystemTime::now(), &mut log);
    assert_eq!(sessions.keyring, "ok");
    assert_eq!(
        sessions
            .find("alice")
            .unwrap()
            .refresh_token
            .as_ref()
            .map(auth::Secret::exposed_for_test),
        Some("rt-a")
    );
    assert!(sessions.find("bob").is_none());
    let log = log.join("\n");
    assert!(
        log.contains("index lists bob as persisted but the keyring has no entry"),
        "{log}"
    );

    let failing = RecordingCredentialStore::failing("load", "keychain locked");
    let mut log = Vec::new();
    let sessions = start::restore_sessions(
        &failing,
        "acq-mock",
        &dir,
        None,
        SystemTime::now(),
        &mut log,
    );
    assert!(sessions.by_account.is_empty());
    assert_eq!(sessions.keyring, "unavailable: keychain locked");
    let log = log.join("\n");
    assert!(
        log.contains("keyring read for alice failed: keychain locked"),
        "{log}"
    );
}

/// A login of `user` staged to the point its profile's uuid lands: the
/// flow begun, the tokens in the staging slot. Returns the generation.
fn staged_login(daemon: &Daemon, user: &str) -> u64 {
    let generation = daemon.begin_auth_flow();
    daemon
        .stage_auth_flow(generation, login_tokens(&format!("at-x.{user}"), user))
        .expect("current flow stages");
    generation
}

/// C144 (the daemon reports no success it did not earn), issue #63's
/// second comment: a login's finalisation as a recovery scenario. The
/// login completes with the index directory unwritable: the index cannot
/// hold the name, so the persist saves nothing (the index knows a name
/// before the keyring does, h1's last round), the flow fails in words
/// naming the index, no credential exists under the name, and a restore
/// over the directory afterwards — the restart — finds no session and
/// reports the keyring healthy. The brief said "the keyring entry it saved
/// is cleared back"; the tree says no entry is saved at all, which the
/// assertion on the store's cleared list pins. A component test: the
/// freeze must fall between the profile's write and the landing, a point
/// no request reaches.
#[cfg(unix)]
#[tokio::test]
async fn c144_a_login_finalised_under_an_unwritable_index_leaves_nothing_to_restore() {
    let (daemon, credentials, store_dir, _log_path) = login_daemon();
    let generation = staged_login(&daemon, "Alice#1");
    let frozen = freeze(&store_dir);
    assert!(
        !daemon.complete_login(generation, "Alice#1", "u-a"),
        "the login does not complete"
    );
    drop(frozen);
    let flow = daemon.shared.lock().unwrap().auth.flow_result.clone();
    let error = match flow {
        Some(Err(error)) => error,
        other => panic!("the flow failed: {other:?}"),
    };
    assert!(
        error.contains("could not record the login in the account index"),
        "{error}"
    );
    assert!(
        credentials.entries.lock().unwrap().is_empty(),
        "no credential under a name the index could not hold"
    );
    assert!(
        credentials.cleared.lock().unwrap().is_empty(),
        "nothing was saved, so nothing had to be cleared back"
    );
    assert!(
        daemon.shared.lock().unwrap().auth.find("Alice#1").is_none(),
        "no session registered"
    );

    // The restart: a restore over the thawed directory.
    let mut log = Vec::new();
    let sessions = start::restore_sessions(
        &*credentials,
        daemon.provider.keyring_service,
        &store_dir,
        None,
        std::time::SystemTime::now(),
        &mut log,
    );
    assert!(
        sessions.by_account.is_empty(),
        "nothing to restore: {}",
        log.join("\n")
    );
    assert_eq!(sessions.keyring, "ok");
}

/// C144 (a failed save clears the entry it would have replaced), the
/// review of h1, its fourth finding: a re-login goes through the persist
/// a refresh uses, so a re-login whose save fails clears the previous
/// login's entry instead of leaving it behind while the index says the
/// account is not persisted; the session's keyring word says why.
#[test]
fn c144_a_re_login_whose_save_fails_clears_the_previous_entry() {
    let clock = Arc::new(ParkingClock::new());
    let credentials = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    Index::load(&store_dir)
        .unwrap()
        .record_login("Alice#1", "uuid-Alice#1", true, 1, &[])
        .unwrap();
    credentials
        .entries
        .lock()
        .unwrap()
        .insert("Alice#1".into(), "rt-previous".into());
    *credentials.fail_save.lock().unwrap() = Some("keychain locked".into());
    let dir = store_dir.to_path_buf();
    let store = credentials.clone();
    logged_out(
        Server::Account(("GET", "/never", 0)),
        &clock,
        &credentials,
        &store_dir,
    )
    .run(|mut door| async move {
        let landed = log_in(&mut door, "Alice#1").await;
        assert_eq!(landed.login_ok.as_deref(), Some("Alice#1"), "{landed:?}");
        assert!(
            store.entries.lock().unwrap().is_empty(),
            "the previous login's entry is cleared"
        );
        assert_eq!(store.cleared.lock().unwrap().as_slice(), ["Alice#1"]);
        assert!(!persisted(&dir, "Alice#1"));
        assert_eq!(
            accounts(&landed),
            [(
                "Alice#1".to_string(),
                "unavailable: keychain locked".to_string()
            )]
        );
        door.finish().await;
    });
}

/// C144 (a rename clears the entry it leaves behind), the review of h1,
/// its fourth finding: a re-login under a renamed username moves the
/// uuid's index entry to the new name and clears the entry under the
/// name the index mapped the uuid to, which nothing could reach again.
/// The index knows the uuid the account server answers for the new name.
#[test]
fn c144_a_re_login_under_a_renamed_username_clears_the_old_names_entry() {
    let clock = Arc::new(ParkingClock::new());
    let credentials = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    Index::load(&store_dir)
        .unwrap()
        .record_login("Old#1", "uuid-New#1", true, 1, &[])
        .unwrap();
    credentials
        .entries
        .lock()
        .unwrap()
        .insert("Old#1".into(), "rt-old".into());
    let dir = store_dir.to_path_buf();
    let store = credentials.clone();
    logged_out(
        Server::Account(("GET", "/never", 0)),
        &clock,
        &credentials,
        &store_dir,
    )
    .run(|mut door| async move {
        let landed = log_in(&mut door, "New#1").await;
        assert_eq!(landed.login_ok.as_deref(), Some("New#1"), "{landed:?}");
        assert_eq!(
            entry_refresh_tokens(&store.entries.lock().unwrap()),
            HashMap::from([("New#1".to_string(), "rt-New#1".to_string())]),
            "one entry, under the new name"
        );
        assert_eq!(store.cleared.lock().unwrap().as_slice(), ["Old#1"]);
        let index = Index::load(&dir).unwrap();
        assert!(index.get("Old#1").is_none());
        let entry = index.get("New#1").unwrap();
        assert_eq!(entry.uuid.as_deref(), Some("uuid-New#1"));
        assert!(entry.persisted);
        door.finish().await;
    });
}

/// C144 (no success the daemon did not earn), the review of h1, its
/// second finding, and the owner's ruling on the tail round: with the
/// keyring turned off (`ACQ_NO_KEYRING`) a clear tries nothing, so it is
/// never reported as one. Logged in with the keyring on, then restarted
/// with it off, a logout of the known account drops the session and
/// marks it not persisted, and its answer is an `ack` whose `keyring` is
/// `off` — a success that says nothing was cleared, where the fix round
/// answered an error and the build before it acknowledged "keyring
/// cleared" over the entry still there. Forget answers the same.
#[test]
fn c144_a_logout_with_the_keyring_off_says_nothing_was_cleared() {
    let store = Arc::new(RecordingCredentialStore::default());
    *store.keyring_off.lock().unwrap() = Some("disabled by ACQ_NO_KEYRING".into());
    let store_dir = TestPath::dir();
    let dir = store_dir.to_path_buf();
    let credentials = store.clone();
    keyring_door(Server::None, &store, &store_dir, true).run(|mut door| async move {
        for account in [None, Some("test-user")] {
            let answer = logout(&mut door, account).await;
            assert!(
                matches!(
                    answer,
                    Response::Ack {
                        keyring: Some(KeyringOutcome::Off)
                    }
                ),
                "a clear that did not happen is said as data: {answer:?}"
            );
        }
        let after = door.await_auth(|_| true).await;
        assert!(!after.logged_in && after.accounts.is_empty(), "{after:?}");
        assert!(!persisted(&dir, "test-user"));
        assert!(
            credentials
                .entries
                .lock()
                .unwrap()
                .contains_key("test-user"),
            "the entry a run with the keyring on saved is still there"
        );
        door.finish().await;
    });
}

/// C144 (every result reaches the words), the review of h1, its fifth
/// finding: forget says "not restored at start" only when its index
/// write marked the account not persisted. With the index directory
/// unwritable and the keyring clear failing, both restoration inputs
/// stand, and the answer says so, where it used to claim the account
/// would not be restored. Logout's index failure reaches its answer the
/// same way: a logout whose clear worked but whose mark was not written
/// is no `ack`. The live session the logout drops comes from a login
/// through the browser.
#[cfg(unix)]
#[test]
fn c144_forget_and_logout_say_what_the_index_write_did() {
    let store = Arc::new(RecordingCredentialStore::failing(
        "clear",
        "keychain locked",
    ));
    let store_dir = TestPath::dir();
    let dir = store_dir.to_path_buf();
    let credentials = store.clone();
    keyring_door(Server::Account(("GET", "/never", 0)), &store, &store_dir, false).run(
        |mut door| async move {
            let frozen = freeze(&dir);
            let answer = logout(&mut door, Some("test-user")).await;
            drop(frozen);
            let Response::Error { kind, message } = &answer else {
                panic!("the forget's failures are the answer: {answer:?}");
            };
            assert_eq!(*kind, ErrorKind::Internal);
            assert!(!message.contains("not restored at start"), "{message}");
            for fact in [
                "forgot account test-user, but the account index could not mark it not persisted (",
                "so a start may still try to restore it",
                "its keyring entry could not be cleared: keychain locked",
            ] {
                assert!(message.contains(fact), "{fact:?} in {message}");
            }
            assert!(persisted(&dir, "test-user"));

            // A live session whose clear works and whose mark cannot be
            // written.
            *credentials.fail_clear.lock().unwrap() = None;
            let landed = log_in(&mut door, "test-user").await;
            assert_eq!(landed.login_ok.as_deref(), Some("test-user"), "{landed:?}");
            let frozen = freeze(&dir);
            let answer = logout(&mut door, None).await;
            drop(frozen);
            let Response::Error { kind, message } = &answer else {
                panic!("an unwritten mark is not an ack: {answer:?}");
            };
            assert_eq!(*kind, ErrorKind::Internal);
            assert!(
                message.contains(
                    "logged out test-user (session dropped, keyring entry cleared), but the account index could not mark it not persisted ("
                ),
                "{message}"
            );
            door.finish().await;
        },
    );
}

/// C144 (one critical section for a forget), the review of the h1 fix
/// round, its third finding: a forget's live-or-known decision, its
/// clears and its not-persisted mark are one hold of the lock a login
/// persists under, `credentials` since C148, so a login of the same
/// account lands before the forget or after it, never between. The
/// login is staged to land at the forget's clear as soon as that lock
/// lets it: at once if it is free there, else once the forget has
/// answered. Before, the forget
/// cleared outside the lock, the login landed there — its credential
/// saved, the account marked persisted, its session live — and the
/// forget's late mark then said not persisted under a live session: a
/// start would not restore the account the user had just logged in to,
/// and the forget had acknowledged a logout that did not hold. A
/// component test: a store hook that calls back into the daemon at the
/// clear.
#[tokio::test]
async fn c144_a_login_cannot_land_between_a_forgets_clear_and_its_mark() {
    let (daemon, credentials, store_dir, _log_path) = login_daemon();
    Index::load(&store_dir)
        .unwrap()
        .record_login("Alice#1", "u-a", true, 1, &[])
        .unwrap();
    credentials
        .entries
        .lock()
        .unwrap()
        .insert("Alice#1".into(), "rt-previous".into());
    let generation = staged_login(&daemon, "Alice#1");
    let deferred = Arc::new(Mutex::new(false));
    {
        let weak = Arc::downgrade(&daemon);
        let deferred = deferred.clone();
        *credentials.on_clear.lock().unwrap() = Some(Box::new(move || {
            let daemon = weak.upgrade().expect("the daemon outlives the forget");
            let free = daemon.credentials.try_lock().is_ok();
            if free {
                assert!(daemon.complete_login(generation, "Alice#1", "u-a"));
            } else {
                *deferred.lock().unwrap() = true;
            }
        }));
    }

    let answer = daemon
        .handle_request(
            Request::AuthLogout {
                account: Some("Alice#1".into()),
            },
            &mut None,
        )
        .await;
    if *deferred.lock().unwrap() {
        assert!(daemon.complete_login(generation, "Alice#1", "u-a"));
    }
    assert!(
        !matches!(answer, Response::Error { .. }),
        "the forget held: {answer:?}"
    );
    let live = daemon.shared.lock().unwrap().auth.find("Alice#1").is_some();
    assert!(live, "the login landed");
    let entry = Index::load(&store_dir)
        .unwrap()
        .get("Alice#1")
        .cloned()
        .unwrap();
    assert!(
        entry.persisted,
        "a live session's account is restored at start: {entry:?}"
    );
    assert_eq!(
        entry_refresh_tokens(&credentials.entries.lock().unwrap())
            .get("Alice#1")
            .map(String::as_str),
        Some("rt-Alice#1"),
        "the login's credential stands"
    );
}

/// C135: a login's landing saves the staged access token and its expiry
/// as unix seconds beside the refresh token, what the session it
/// registers holds: the saved expiry, read against the clock's wall, is
/// the one `auth status` reports.
#[test]
fn a_logins_landing_saves_the_access_token_beside_the_refresh_token() {
    let clock = Arc::new(ParkingClock::new());
    let credentials = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let store = credentials.clone();
    let parking = clock.clone();
    logged_out(
        Server::Account(("GET", "/never", 0)),
        &clock,
        &credentials,
        &store_dir,
    )
    .run(|mut door| async move {
        let landed = log_in(&mut door, "Alice#1").await;
        assert_eq!(landed.login_ok.as_deref(), Some("Alice#1"), "{landed:?}");
        let wall = parking.wall();
        let secret = store.entries.lock().unwrap()["Alice#1"].clone();
        let entry = auth::KeyringEntry::from_secret(&secret.as_str().into(), wall);
        assert_eq!(
            entry
                .access_token
                .as_ref()
                .map(auth::Secret::exposed_for_test),
            Some("at-Alice#1")
        );
        assert_eq!(entry.refresh_token, "rt-Alice#1");
        let wall_seconds = wall
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let reported = landed
            .access_expires_in_seconds
            .expect("the session's expiry is reported");
        assert_eq!(entry.access_expires_at, Some(wall_seconds + reported));
        door.finish().await;
    });
}

/// #16, C156 — the logout's retry word is quoted for the shell it will
/// be pasted into by the same rules as the search crate's
/// `shell_quoted`, whose twin this is (C1 keeps them two): every row of
/// the one table both read, `shell-words.json`, for every shell
/// `ACQ_SHELL` declares, and `shell_word` the environment's shell. A
/// component test: a table.
#[test]
fn the_retry_word_is_quoted_for_the_declared_shell() {
    use crate::daemon::login::{
        cmd_word, posix_word, powershell_word, powershell5_word, shell_word,
    };
    let rows: Vec<serde_json::Value> = serde_json::from_str(include_str!(
        "../../../../acquisition-search/tests/shell-words.json"
    ))
    .unwrap();
    assert!(rows.len() > 20);
    let declared = std::env::var("ACQ_SHELL").ok();
    let here = match declared.as_deref() {
        Some(name @ ("posix" | "pwsh" | "powershell5" | "cmd")) => name,
        _ if cfg!(windows) && std::env::var_os("MSYSTEM").is_none() => "pwsh",
        _ => "posix",
    };
    for row in &rows {
        let word = row["word"].as_str().unwrap();
        let want = |shell: &str| row[shell].as_str().unwrap().to_string();
        assert_eq!(posix_word(word), want("posix"), "{word}");
        assert_eq!(powershell_word(word), want("pwsh"), "{word}");
        assert_eq!(powershell5_word(word), want("powershell5"), "{word}");
        assert_eq!(cmd_word(word), want("cmd"), "{word}");
        assert_eq!(shell_word(word), want(here), "{word}");
    }
}
