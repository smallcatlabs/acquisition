//! The refresh grant: a rejected grant stops further refreshes until
//! logout, a transient failure marks nothing; a refresh's landing (C144)
//! and the saved access token's lifetime (C135).
//!
//! C158, a scenario crosses the daemon's frontend boundary only: a
//! refresh here is `AuthCheck` (which always forces one) or a client's
//! `profile` job (the non-forcing path a frontend has); a logout is
//! `AuthLogout`; a restart is a door that restores as a start does
//! ([`DoorBuilder::restore`]). What a scenario reads is the protocol's
//! answers (the check's, `AuthStatus`, `DaemonStatus.rails`,
//! `Dashboard.errors`), the journal, what the scripted server received
//! (the grant a token `POST` carried, the bearer a `GET` carried:
//! [`Door::received`]), its own credential store and the account index
//! on disk (the store's read API, C12). A token's value is a boundary
//! property only through a send that carries it: a row that says a
//! session stands after a failure makes the next send happen and reads
//! the grant on the wire.
//!
//! The token table, [`c158_the_token_table_through_the_door`], holds
//! what the token endpoint answers: each row a script of token answers,
//! the checks a frontend sends and what each is answered, the rails'
//! marks, the journal, the grants on the wire and the saves.
//!
//! The component tests, each with the boundary it needs:
//! `a_bare_harness_daemon_saves_a_rotated_token_into_the_test_store`
//! (a harness property: the bare constructor's default store, which the
//! door takes as an argument);
//! `c144_a_logout_where_a_refresh_releases_the_lock_keeps_its_mark` (the
//! landing's own hook, `refresh_landed`, on the blocking thread: the
//! staging point has no surface); `c144_restore_reaches_a_token_under_a_held_name`
//! and `c144_restore_reports_a_held_names_failed_load` (`restore_sessions`
//! as a function); and `a_token_3xx_keeps_its_target_out_of_the_daemon_log`
//! (the daemon log, which the door does not read).
//!
//! Rulings: `decisions/network.md`, `decisions/daemon.md`; the
//! cross-cutting ones are `AGENTS.md`'s.

use super::door::*;
use super::harness::*;
use crate::daemon::*;
use acquisition_protocol::protocol::KeyringOutcome;

// ---- the token table ----------------------------------------------------

/// One row of the token table: the token answers the scripted server
/// plays in order, the rails the daemon opens with, the steps a frontend
/// takes through the door, and what crosses the boundary back. At the
/// row's end `DaemonStatus.rails` reads `refresh_failed` and `halted`
/// (words each carries, or `None`); the journal's sends are `journal` as
/// [`token_projection`] reads them, the send numbered in
/// `journal_errors` naming the words given in its `error`; the grants
/// the token `POST`s carried on the wire are `grants`, in order; the
/// refresh tokens saved into the scenario's store are `saves`, in order;
/// and each fragment of `forbids` is absent from every surface it names.
struct Row {
    /// The test the row replaced, by name (a ruling's *Pinned* names it).
    name: &'static str,
    script: Vec<ScriptedResponse>,
    rails: RailsConfig,
    steps: Vec<Step>,
    refresh_failed: Option<&'static str>,
    halted: Option<&'static str>,
    journal: Vec<Value>,
    journal_errors: Vec<(usize, &'static str)>,
    grants: Vec<&'static str>,
    saves: Vec<&'static str>,
    forbids: Vec<Forbid>,
}

/// One step of a row, as a frontend takes it through the door.
enum Step {
    /// `AuthCheck` of the sole session, answered as [`Answer`] says.
    Check(Answer),
    /// `DaemonStatus.rails` read mid-row: `refresh_failed` and `halted`
    /// carry these words, or are `None`.
    Rails {
        refresh_failed: Option<&'static str>,
        halted: Option<&'static str>,
    },
    /// `ResetTripwire`, acknowledged.
    Reset,
    /// `AuthLogout` of the sole session, acknowledged.
    Logout,
    /// The lid closes for this long ([`Door::laptop_sleep`]).
    LaptopSleep(Duration),
}

/// What a check is answered.
enum Answer {
    /// `Response::Auth` naming this account.
    Auth(&'static str),
    /// `Response::Error` of this kind, its message carrying each of the
    /// words.
    Error {
        kind: ErrorKind,
        carries: Vec<&'static str>,
    },
}

/// A fragment that must be absent from each surface it names.
struct Forbid {
    fragment: &'static str,
    from: &'static [Surface],
}

/// A surface a [`Forbid`] names.
#[derive(Debug)]
enum Surface {
    /// Every check's answer, whole.
    Answers,
    /// The journal file, header included ([`Door::journal_text`]).
    Journal,
    /// `Dashboard.errors`, read at the row's end.
    DashboardErrors,
}

fn forbid(fragment: &'static str, from: &'static [Surface]) -> Forbid {
    Forbid { fragment, from }
}

/// An error answer of `kind` carrying `carries`.
fn refused(kind: ErrorKind, carries: &[&'static str]) -> Answer {
    Answer::Error {
        kind,
        carries: carries.to_vec(),
    }
}

/// Each send as a row compares it: method, path, status, ok and
/// `wait_ms` (the hold before it reached the transport, on the clock).
fn token_projection(sends: &[Value]) -> Vec<Value> {
    sends
        .iter()
        .map(|l| json!([l["method"], l["path"], l["status"], l["ok"], l["wait_ms"]]))
        .collect()
}

/// A token `POST` as the journal projects it.
fn post(status: u16, ok: bool, wait_ms: u64) -> Value {
    json!(["POST", "/token", status, ok, wait_ms])
}

/// GGG's rejection of a dead grant.
const INVALID_GRANT: &str = r#"{"error":"invalid_grant"}"#;

/// What C130's token 307 echoes in its body and its `Location`'s query.
const C130_TOKEN_SENTINEL: &str = "c130-token-sentinel";

/// C130: a token `POST`'s 307, the token route's headers and a
/// `Location` whose query, like the body that echoes it, carries
/// [`C130_TOKEN_SENTINEL`].
fn token_3xx() -> ScriptedResponse {
    let target = format!("https://www.pathofexile.com/oauth/token?state={C130_TOKEN_SENTINEL}");
    token_answer(
        "307 Temporary Redirect",
        &format!("{TOKEN_HEADERS}Location: {target}\r\n"),
        Some(&format!(r#"{{"moved_to":"{target}"}}"#)),
    )
}

/// The rows, each written from the ruling it pins and the assertions of
/// the test it replaced, never from a run of the door.
fn token_rows() -> Vec<Row> {
    let huge = json!({
        "access_token": "at-huge",
        "refresh_token": "rt-huge",
        "expires_in": u64::MAX,
        "username": "test-user",
    })
    .to_string();
    let mut challenge = ScriptedResponse::cloudflare_403_challenge();
    challenge.method = "POST";
    challenge.path = "/token";
    vec![
        // C24: a 400 is a rejected grant, a dead session: the check says
        // so, the mark is set; the next check sends nothing and says the
        // refresh is disabled; a logout clears the mark. The grant after
        // the mark has no boundary read: no send carries it until login
        // or logout.
        Row {
            name: "rejected_refresh_grant_disables_further_refreshes_until_logout",
            script: vec![token_answer(
                "400 Bad Request",
                TOKEN_HEADERS,
                Some(INVALID_GRANT),
            )],
            rails: RailsConfig::default(),
            steps: vec![
                Step::Check(refused(ErrorKind::NotLoggedIn, &["400 Bad Request"])),
                Step::Rails {
                    refresh_failed: Some("HTTP 400"),
                    halted: None,
                },
                Step::Check(refused(
                    ErrorKind::NotLoggedIn,
                    &["token refresh disabled for test-user: refresh token rejected with HTTP 400"],
                )),
                Step::Logout,
            ],
            refresh_failed: None,
            halted: None,
            journal: vec![post(400, false, 0)],
            journal_errors: vec![],
            grants: vec!["rt-old"],
            saves: vec![],
            forbids: vec![],
        },
        // A 5xx is the provider, not the session: no mark. The tripwire
        // trips on it; reset, the next check refreshes on the grant
        // that stood.
        Row {
            name: "transient_refresh_failures_do_not_mark_the_session",
            script: vec![
                token_answer(
                    "503 Service Unavailable",
                    TOKEN_HEADERS,
                    Some(INVALID_GRANT),
                ),
                token_answer("200 OK", TOKEN_HEADERS, None),
            ],
            rails: tripwire_config(),
            steps: vec![
                Step::Check(refused(ErrorKind::Upstream, &["503"])),
                Step::Rails {
                    refresh_failed: None,
                    halted: Some("503"),
                },
                Step::Reset,
                Step::Check(Answer::Auth("test-user")),
            ],
            refresh_failed: None,
            halted: None,
            journal: vec![post(503, false, 0), post(200, true, 0)],
            journal_errors: vec![],
            grants: vec!["rt-old", "rt-old"],
            saves: vec!["rt-rotated"],
            forbids: vec![],
        },
        // C130: a token POST's 307 is refused upstream with the status and
        // the redirect's target shown only redacted; the body, which
        // echoes the target with its query, is kept nowhere. One send, no
        // follow. The daemon log's read is the component case
        // `a_token_3xx_keeps_its_target_out_of_the_daemon_log`.
        Row {
            name: "a_token_3xx_is_refused_with_the_target_redacted_and_no_body",
            script: vec![token_3xx()],
            rails: RailsConfig::default(),
            steps: vec![Step::Check(refused(
                ErrorKind::Upstream,
                &["307", "https://www.pathofexile.com/oauth/token?…"],
            ))],
            refresh_failed: None,
            halted: None,
            journal: vec![post(307, false, 0)],
            journal_errors: vec![],
            grants: vec!["rt-old"],
            saves: vec![],
            forbids: vec![forbid(
                C130_TOKEN_SENTINEL,
                &[Surface::Answers, Surface::Journal, Surface::DashboardErrors],
            )],
        },
        // C129 over C24: Cloudflare's challenge on the token endpoint is
        // no rejected grant. The block mark, no dead-grant mark; the
        // refusal says Cloudflare answered and the daemon stopped, never
        // `acq auth`; the next check sends nothing (the halt refuses it).
        // Reset, the session stood through the block: the next check's
        // POST carries the same grant.
        Row {
            name: "a_cloudflare_block_on_the_token_route_is_no_rejected_grant",
            script: vec![challenge, token_answer("200 OK", TOKEN_HEADERS, None)],
            rails: RailsConfig::default(),
            steps: vec![
                Step::Check(refused(
                    ErrorKind::Upstream,
                    &["Cloudflare answered in GGG's place", "reset-tripwire"],
                )),
                Step::Rails {
                    refresh_failed: None,
                    halted: Some("Cloudflare answered in GGG's place: 403 on POST oauth-token"),
                },
                Step::Check(refused(ErrorKind::Upstream, &[])),
                Step::Reset,
                Step::Check(Answer::Auth("test-user")),
            ],
            refresh_failed: None,
            halted: None,
            journal: vec![post(403, false, 0), post(200, true, 0)],
            journal_errors: vec![],
            grants: vec!["rt-old", "rt-old"],
            saves: vec!["rt-rotated"],
            forbids: vec![
                forbid("acq auth", &[Surface::Answers]),
                forbid("Just a moment", &[Surface::Answers]),
            ],
        },
        // C131: a token 2xx with no rate-limit header at all installs its
        // tokens and the refresh succeeds on them; the send stays a
        // journaled protocol failure, and the unestablished route is held
        // one timing bucket: the next grant, the rotated one, waits 60 s.
        Row {
            name: "a_headerless_token_2xx_installs_its_tokens_and_holds_the_route",
            script: vec![
                token_answer("200 OK", "", None),
                token_answer("200 OK", TOKEN_HEADERS, None),
            ],
            rails: RailsConfig::default(),
            steps: vec![
                Step::Check(Answer::Auth("test-user")),
                // Crosses the access token's expiry on the wall (an hour,
                // nothing parks on it) and leaves the C131 hold, parked on
                // the monotonic face, ahead: an advance would expire the
                // hold too. The check forces its refresh whatever the
                // expiry; the hold is what its POST meets.
                Step::LaptopSleep(Duration::from_secs(3600)),
                Step::Check(Answer::Auth("test-user")),
            ],
            refresh_failed: None,
            halted: None,
            journal: vec![post(200, false, 0), post(200, true, 60_000)],
            journal_errors: vec![(0, "x-rate-limit-policy")],
            grants: vec!["rt-old", "rt-rotated"],
            saves: vec!["rt-rotated", "rt-rotated"],
            forbids: vec![],
        },
        // C131's limit: a 200 that is no token response (a portal page)
        // installs nothing; the refresh fails upstream, nothing is saved
        // or marked, and the session stands: the next check's POST
        // carries the same grant and lands, after the route's hold (the
        // page came with no rate-limit header).
        Row {
            name: "a_token_2xx_that_is_no_token_response_installs_nothing",
            script: vec![
                token_answer(
                    "200 OK",
                    "",
                    Some("<!DOCTYPE html><html><body>Path of Exile</body></html>"),
                ),
                token_answer("200 OK", TOKEN_HEADERS, None),
            ],
            rails: RailsConfig::default(),
            steps: vec![
                Step::Check(refused(ErrorKind::Upstream, &[])),
                Step::Rails {
                    refresh_failed: None,
                    halted: None,
                },
                Step::Check(Answer::Auth("test-user")),
            ],
            refresh_failed: None,
            halted: None,
            journal: vec![post(200, false, 0), post(200, true, 60_000)],
            journal_errors: vec![],
            grants: vec!["rt-old", "rt-old"],
            saves: vec!["rt-rotated"],
            forbids: vec![],
        },
        // C131's hold is for an answer with no rate-limit header at all:
        // a header set present but malformed installs the tokens and
        // holds nothing, so the next grant goes out at once.
        Row {
            name: "a_token_2xx_with_malformed_rate_headers_installs_its_tokens_without_a_hold",
            script: vec![
                token_answer(
                    "200 OK",
                    "X-Rate-Limit-Policy: token-request-limit\r\n",
                    None,
                ),
                token_answer("200 OK", TOKEN_HEADERS, None),
            ],
            rails: RailsConfig::default(),
            steps: vec![
                Step::Check(Answer::Auth("test-user")),
                // The access token's expiry on the wall, as the headerless
                // row's; nothing is held here to stay ahead of.
                Step::LaptopSleep(Duration::from_secs(3600)),
                Step::Check(Answer::Auth("test-user")),
            ],
            refresh_failed: None,
            halted: None,
            journal: vec![post(200, false, 0), post(200, true, 0)],
            journal_errors: vec![],
            grants: vec!["rt-old", "rt-rotated"],
            saves: vec!["rt-rotated", "rt-rotated"],
            forbids: vec![],
        },
        // C47 with C134: an `expires_in` no clock can add is GGG's input,
        // not a broken invariant. The refresh fails upstream before
        // anything is installed or saved, its words name no token, and
        // the session stands; the next check answers (nothing is
        // poisoned), its POST carrying the same grant.
        Row {
            name: "an_out_of_range_expires_in_is_a_token_error",
            script: vec![
                token_answer("200 OK", TOKEN_HEADERS, Some(&huge)),
                token_answer("200 OK", TOKEN_HEADERS, None),
            ],
            rails: RailsConfig::default(),
            steps: vec![
                Step::Check(refused(ErrorKind::Upstream, &["lifetime"])),
                Step::Rails {
                    refresh_failed: None,
                    halted: None,
                },
                Step::Check(Answer::Auth("test-user")),
            ],
            refresh_failed: None,
            halted: None,
            journal: vec![post(200, true, 0), post(200, true, 0)],
            journal_errors: vec![],
            grants: vec!["rt-old", "rt-old"],
            saves: vec!["rt-rotated"],
            forbids: vec![
                forbid(
                    "at-huge",
                    &[Surface::Answers, Surface::Journal, Surface::DashboardErrors],
                ),
                forbid(
                    "rt-huge",
                    &[Surface::Answers, Surface::Journal, Surface::DashboardErrors],
                ),
            ],
        },
    ]
}

/// `DaemonStatus.rails`'s two marks, as a frontend reads them.
async fn marks(door: &mut Door) -> (Option<String>, Option<String>) {
    match door.request(Request::DaemonStatus {}).await {
        Response::DaemonStatus { rails, .. } => (rails.refresh_failed, rails.halted),
        other => panic!("the daemon's status reads {other:?}"),
    }
}

/// A mark as a row expects it: carrying `want`'s words, or none.
fn assert_mark(name: &str, mark: &str, read: Option<&str>, want: Option<&str>) {
    match want {
        Some(words) => assert!(
            read.is_some_and(|read| read.contains(words)),
            "{name}: {mark} lacks {words:?}: {read:?}"
        ),
        None => assert_eq!(read, None, "{name}: {mark}"),
    }
}

/// `Dashboard.errors`, newest first.
async fn dashboard_errors(door: &mut Door) -> Vec<String> {
    match door.request(Request::Dashboard {}).await {
        Response::Dashboard { errors, .. } => errors.into_iter().map(|e| e.message).collect(),
        other => panic!("the dashboard reads {other:?}"),
    }
}

/// The grants the token `POST`s carried on the wire, in order.
fn grants_on_the_wire(door: &Door) -> Vec<String> {
    door.received()
        .iter()
        .filter(|request| request.method == "POST" && request.path == "/token")
        .map(|request| request.grant().expect("a refresh grant's body"))
        .collect()
}

/// C158, the token table: each row through the door, a logged-in
/// `test-user` (its grant `rt-old`, no access token) facing the row's
/// script with a recording store of the scenario's. It reads nothing
/// but the protocol's answers, the journal, the scripted server's record
/// of the wire and its own store. What the hook did prints under
/// `--no-capture`, asserted by nothing.
#[test]
fn c158_the_token_table_through_the_door() {
    for row in token_rows() {
        let Row {
            name,
            script,
            rails,
            steps,
            refresh_failed,
            halted,
            journal,
            journal_errors,
            grants,
            saves,
            forbids,
        } = row;
        let store = Arc::new(RecordingCredentialStore::default());
        let saved = store.clone();
        let hook = Door::open(Server::Script(script), Arc::new(ParkingClock::new()), store)
            .rails(rails)
            .sessions(logged_in_sessions())
            .run(|mut door| async move {
                let mut answers = Vec::new();
                for step in steps {
                    match step {
                        Step::Check(want) => {
                            let answer = door.request(Request::AuthCheck { account: None }).await;
                            match (&want, &answer) {
                                (Answer::Auth(user), Response::Auth { username, .. }) => {
                                    assert_eq!(username.as_deref(), Some(*user), "{name}")
                                }
                                (
                                    Answer::Error { kind, carries },
                                    Response::Error {
                                        kind: answered,
                                        message,
                                    },
                                ) => {
                                    assert_eq!(answered, kind, "{name}: {message}");
                                    for words in carries {
                                        assert!(
                                            message.contains(words),
                                            "{name}: the answer lacks {words:?}: {message}"
                                        );
                                    }
                                }
                                _ => panic!("{name}: a check answered {answer:?}"),
                            }
                            answers.push(format!("{answer:?}"));
                        }
                        Step::Rails {
                            refresh_failed,
                            halted,
                        } => {
                            let (marked, halt) = marks(&mut door).await;
                            assert_mark(
                                name,
                                "the dead-grant mark",
                                marked.as_deref(),
                                refresh_failed,
                            );
                            assert_mark(name, "the halt", halt.as_deref(), halted);
                        }
                        Step::Reset => {
                            let answer = door.request(Request::ResetTripwire {}).await;
                            assert!(matches!(answer, Response::Ack { .. }), "{name}: {answer:?}");
                        }
                        Step::Logout => {
                            let answer = door.request(Request::AuthLogout { account: None }).await;
                            assert!(matches!(answer, Response::Ack { .. }), "{name}: {answer:?}");
                        }
                        Step::LaptopSleep(lid) => door.laptop_sleep(lid),
                    }
                }

                let (marked, halt) = marks(&mut door).await;
                assert_mark(
                    name,
                    "the dead-grant mark",
                    marked.as_deref(),
                    refresh_failed,
                );
                assert_mark(name, "the halt", halt.as_deref(), halted);
                let sends = door.journal();
                assert_eq!(token_projection(&sends), journal, "{name}: the journal");
                for (send, words) in journal_errors {
                    assert!(
                        sends[send]["error"]
                            .as_str()
                            .is_some_and(|error| error.contains(words)),
                        "{name}: send {send} names {words:?}: {}",
                        sends[send]
                    );
                }
                assert_eq!(grants_on_the_wire(&door), grants, "{name}: the grants");
                let rotated: Vec<String> = saved_refresh_tokens(&saved.saves.lock().unwrap())
                    .into_iter()
                    .map(|(_, token, user)| {
                        assert_eq!(user, "test-user", "{name}");
                        token
                    })
                    .collect();
                assert_eq!(rotated, saves, "{name}: the saves");
                let errors = dashboard_errors(&mut door).await.join("\n");
                let journal_text = door.journal_text();
                for Forbid { fragment, from } in &forbids {
                    for surface in *from {
                        let texts: Vec<&str> = match surface {
                            Surface::Answers => answers.iter().map(String::as_str).collect(),
                            Surface::Journal => vec![&journal_text],
                            Surface::DashboardErrors => vec![&errors],
                        };
                        for text in texts {
                            assert!(
                                !text.contains(fragment),
                                "{name}: {surface:?} keeps {fragment:?}: {text}"
                            );
                        }
                    }
                }
                door.finish().await;
            });
        eprintln!("{name}: the park hook: {hook:?}");
    }
}

/// C130, the daemon log: a component case of the log file, which no
/// frontend reads through the protocol (the refusal, the journal and
/// `Dashboard.errors` are the token table's row
/// `a_token_3xx_is_refused_with_the_target_redacted_and_no_body`). The
/// check enters by the protocol request; a token 307 whose body and
/// `Location` carry the sentinel leaves none of it in the log.
#[tokio::test]
async fn a_token_3xx_keeps_its_target_out_of_the_daemon_log() {
    let (transport, requests, ends) = scripted_transport(vec![token_3xx()]);
    let (mut daemon, log_path) = test_daemon_with(
        Boundary::Function,
        Provider::mock(BASE),
        transport,
        Arc::new(ParkingClock::new()),
        RailsConfig::default(),
    );
    logged_in(&mut daemon);
    let answer = daemon
        .handle_request(Request::AuthCheck { account: None }, &mut None)
        .await;
    assert!(matches!(answer, Response::Error { .. }), "{answer:?}");
    let log = std::fs::read_to_string(&log_path).unwrap();
    assert!(log.contains("307"), "the log names the 307: {log}");
    assert!(
        !log.contains(C130_TOKEN_SENTINEL),
        "the daemon log kept it: {log}"
    );
    assert_whole(&ends);
    assert_journal_matches_wire(&log_path, &requests.lock().unwrap());
    finish_harness(None, &log_path);
}

/// Invariant 5's test side: a harness daemon whose test names no store
/// saves a refresh's rotated token into the harness's recorder, never the
/// developer's keychain. The store is read before anything is sent, so a
/// harness that defaulted to the OS store fails there, with nothing saved.
/// A component test: the bare constructor's default store is a harness
/// property, and the door takes its store as an argument.
#[tokio::test]
async fn a_bare_harness_daemon_saves_a_rotated_token_into_the_test_store() {
    let (transport, requests, ends) =
        scripted_transport(vec![token_answer("200 OK", TOKEN_HEADERS, None)]);
    let (daemon, log_path) = test_daemon_with(
        Boundary::Function,
        Provider::mock(BASE),
        transport,
        Arc::new(ParkingClock::new()),
        RailsConfig::default(),
    );
    let credentials = harness_credentials(&daemon);
    {
        let mut s = daemon.shared.lock().unwrap();
        s.auth.one_mut().refresh_token = Some("rt-old".into());
        s.auth.rename("", "test-user");
    }

    let (token, _) = daemon.valid_access_token(None, false, None).await.unwrap();
    assert_eq!(token, "at-new");
    assert_eq!(
        saved_refresh_tokens(&credentials.saves.lock().unwrap()),
        [(
            daemon.provider.keyring_service.to_string(),
            "rt-rotated".to_string(),
            "test-user".to_string()
        )],
        "the rotated refresh token is saved into the test store"
    );
    assert_whole(&ends);
    assert_journal_matches_wire(&log_path, &requests.lock().unwrap());
    finish_harness(None, &log_path);
}

// ---- C144: a refresh's landing ------------------------------------------

/// `test-user` logged in with the grant `rt-old` and no access token (a
/// refresh is due), its uuid `u-1`, the keyring `ok`.
fn indexed_sessions() -> Sessions {
    let mut sessions = Sessions::with(AuthSession {
        refresh_token: Some("rt-old".into()),
        uuid: Some("u-1".into()),
        ..AuthSession::default()
    });
    sessions.rename("", "test-user");
    sessions.keyring = "ok".into();
    sessions
}

/// The landing family's door: `script` answering, `store` holding
/// `test-user`'s entry `rt-old`, an account index in `store_dir` listing
/// `test-user` (uuid `u-1`) persisted, and that session live.
fn landing_door(
    script: Vec<ScriptedResponse>,
    store: &Arc<RecordingCredentialStore>,
    store_dir: &std::path::Path,
) -> DoorBuilder {
    store
        .entries
        .lock()
        .unwrap()
        .insert("test-user".into(), "rt-old".into());
    Index::load(store_dir)
        .unwrap()
        .record_login("test-user", "u-1", true, 1, &[])
        .unwrap();
    Door::open(
        Server::Script(script),
        Arc::new(ParkingClock::new()),
        store.clone(),
    )
    .sessions(indexed_sessions())
    .store_dir(store_dir.to_path_buf())
}

/// A token answer that renames the account to `username`, rotating the
/// refresh token to `rt-<username>`.
fn renamed_answer(username: &str) -> ScriptedResponse {
    let body = token_body(
        &format!("at-{username}"),
        &format!("rt-{username}"),
        username,
    );
    token_answer("200 OK", TOKEN_HEADERS, Some(&body))
}

/// `AuthCheck` of the sole session, answered with the account it
/// names; panics on any other answer.
async fn check(door: &mut Door) -> String {
    match door.request(Request::AuthCheck { account: None }).await {
        Response::Auth { username, .. } => username.expect("the check names its account"),
        other => panic!("the check answered {other:?}"),
    }
}

/// `AuthLogout` of `account`, as the CLI sends it.
async fn logout(door: &mut Door, account: &str) -> Response {
    door.request(Request::AuthLogout {
        account: Some(account.into()),
    })
    .await
}

/// `account`'s keyring word in `AuthStatus.accounts`, once the save the
/// refresh landed is published (no longer `saving`).
async fn keyring_of(door: &mut Door, account: &str) -> String {
    let landed = door
        .await_auth(|answer| {
            answer
                .accounts
                .iter()
                .any(|a| a.username == account && a.keyring != "saving")
        })
        .await;
    landed
        .accounts
        .into_iter()
        .find(|a| a.username == account)
        .map(|a| a.keyring)
        .expect("the account is listed")
}

/// The store's entries, each shown as the refresh token it holds.
fn entries(store: &RecordingCredentialStore) -> HashMap<String, String> {
    entry_refresh_tokens(&store.entries.lock().unwrap())
}

/// C144 (a rename clears the entry it leaves behind), issue #61: a
/// refresh whose answer names the account differently saves under the
/// new name and clears the old name's entry, which nothing could reach
/// again: restore reads the index, and the index follows the new name.
#[test]
fn c144_a_refresh_that_renames_clears_the_old_names_entry() {
    let store = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let read = store.clone();
    landing_door(vec![renamed_answer("NewName#2")], &store, &store_dir).run(
        |mut door| async move {
            assert_eq!(check(&mut door).await, "NewName#2");
            assert_eq!(
                entries(&read),
                HashMap::from([("NewName#2".to_string(), "rt-NewName#2".to_string())]),
                "one entry, under the new name; the old name's is cleared"
            );
            assert_eq!(read.cleared.lock().unwrap().as_slice(), ["test-user"]);
            door.finish().await;
        },
    );
}

/// C144 (a failed save clears the entry it would have replaced), issue
/// #61: the grant rotated the refresh token, so the entry the save would
/// have overwritten holds a token GGG no longer honours; it is cleared,
/// the session runs in memory, and the keyring status and the error
/// list say why.
#[test]
fn c144_a_failed_save_clears_the_entry_it_would_have_replaced() {
    let store = Arc::new(RecordingCredentialStore::failing("save", "keychain locked"));
    let store_dir = TestPath::dir();
    let read = store.clone();
    landing_door(
        vec![token_answer("200 OK", TOKEN_HEADERS, None)],
        &store,
        &store_dir,
    )
    .run(|mut door| async move {
        assert_eq!(
            check(&mut door).await,
            "test-user",
            "the session runs on, in memory"
        );
        assert_eq!(
            keyring_of(&mut door, "test-user").await,
            "unavailable: keychain locked"
        );
        let errors = dashboard_errors(&mut door).await;
        assert!(
            errors
                .iter()
                .any(|m| m.contains("keyring save failed: keychain locked")),
            "the failure is reported: {errors:?}"
        );
        assert!(
            read.entries.lock().unwrap().is_empty(),
            "no stale entry stays"
        );
        assert_eq!(read.cleared.lock().unwrap().as_slice(), ["test-user"]);
        door.finish().await;
    });
}

/// C144: a clear that fails beside a rename reaches the error list in
/// words that name the old entry; the save itself stood, so the account
/// stays persisted under its new name.
#[test]
fn c144_a_rename_whose_old_entry_cannot_be_cleared_says_so() {
    let store = Arc::new(RecordingCredentialStore::failing(
        "clear",
        "keychain locked",
    ));
    let store_dir = TestPath::dir();
    let read = store.clone();
    landing_door(vec![renamed_answer("NewName#2")], &store, &store_dir).run(
        |mut door| async move {
            assert_eq!(check(&mut door).await, "NewName#2");
            assert_eq!(keyring_of(&mut door, "NewName#2").await, "ok");
            let errors = dashboard_errors(&mut door).await;
            assert!(
                errors.iter().any(|m| m.contains(
                    "the keyring entry under the old name test-user could not be cleared: keychain locked"
                )),
                "{errors:?}"
            );
            assert!(read.entries.lock().unwrap().contains_key("NewName#2"));
            door.finish().await;
        },
    );
}

/// C144 (no keyring entry out of the index's reach), the review of h1,
/// its third finding: a rename whose clear of the old name fails leaves
/// the index holding that name beside the new one, with the account
/// still persisted under the new name, so a later logout of the account
/// clears both. Before, the index followed the new name alone, and the
/// old name's entry was out of every verb's reach.
#[test]
fn c144_a_rename_whose_old_clear_fails_stays_in_the_logouts_reach() {
    let store = Arc::new(RecordingCredentialStore::failing(
        "clear",
        "keychain locked",
    ));
    let store_dir = TestPath::dir();
    let (read, dir) = (store.clone(), store_dir.to_path_buf());
    landing_door(vec![renamed_answer("NewName#2")], &store, &store_dir).run(
        |mut door| async move {
            assert_eq!(check(&mut door).await, "NewName#2");
            {
                let index = Index::load(&dir).unwrap();
                assert!(
                    index.get("test-user").is_none(),
                    "the entry follows the uuid"
                );
                let entry = index.get("NewName#2").unwrap();
                assert!(entry.persisted);
                assert!(
                    entry.keyring_names.iter().any(|n| n == "test-user"),
                    "{entry:?}"
                );
            }
            assert_eq!(read.entries.lock().unwrap().len(), 2);

            // The keyring works again: the logout reaches both names.
            *read.fail_clear.lock().unwrap() = None;
            let answer = logout(&mut door, "NewName#2").await;
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
                read.entries.lock().unwrap().is_empty(),
                "no entry is left behind"
            );
            let index = Index::load(&dir).unwrap();
            let entry = index.get("NewName#2").unwrap();
            assert!(!entry.persisted);
            assert!(
                entry.keyring_names.is_empty(),
                "every held name is released: {entry:?}"
            );
            door.finish().await;
        },
    );
}

/// C144 (no keyring entry out of the index's reach), the review of the h1
/// fix round, its fourth finding: two renames, test-user to B and B to C,
/// each failing its old names' clears, leave the index holding every
/// uncleared name, so a logout of C with the keyring working again clears
/// all three. Before, the index held one old name, the second rename
/// replaced test-user with B, and test-user's entry was out of every
/// verb's reach.
#[test]
fn c144_two_renames_whose_clears_fail_keep_every_name_in_reach() {
    let store = Arc::new(RecordingCredentialStore::failing(
        "clear",
        "keychain locked",
    ));
    let store_dir = TestPath::dir();
    let (read, dir) = (store.clone(), store_dir.to_path_buf());
    landing_door(
        vec![renamed_answer("B#2"), renamed_answer("C#3")],
        &store,
        &store_dir,
    )
    .run(|mut door| async move {
        assert_eq!(check(&mut door).await, "B#2");
        assert_eq!(check(&mut door).await, "C#3");
        assert_eq!(grants_on_the_wire(&door), ["rt-old", "rt-B#2"]);
        assert_eq!(read.entries.lock().unwrap().len(), 3);
        {
            let index = Index::load(&dir).unwrap();
            let entry = index.get("C#3").unwrap();
            for name in ["test-user", "B#2", "C#3"] {
                assert!(
                    entry.keyring_reach().iter().any(|n| n == name),
                    "{name} in reach of {entry:?}"
                );
            }
        }

        *read.fail_clear.lock().unwrap() = None;
        let answer = logout(&mut door, "C#3").await;
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
            read.entries.lock().unwrap().is_empty(),
            "no entry is left behind: {:?}",
            read.entries.lock().unwrap()
        );
        let index = Index::load(&dir).unwrap();
        let entry = index.get("C#3").unwrap();
        assert!(!entry.persisted);
        assert!(entry.keyring_names.is_empty(), "{entry:?}");
        door.finish().await;
    });
}

/// C144 (logout finds the account by its uuid), the review of the h1 fix
/// round, its first finding: a refresh that renames test-user to B cannot
/// write the index, which still lists the account as test-user,
/// persisted, with test-user's entry uncleared (and B unsaved: the index
/// could not hold its name first). A logout of B, once the index is
/// writable again, finds the entry by the session's uuid, clears every
/// name and marks it not persisted, so a start restores nothing: a
/// second door on the same store directory and store, restoring as a
/// start does, lists no account. Before, the logout looked the entry up
/// by B, found none, marked nothing and acknowledged, and a restart
/// restored test-user from its old entry.
#[cfg(unix)]
#[test]
fn c144_a_logout_after_an_unindexed_rename_finds_the_account_by_its_uuid() {
    let store = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let (read, dir) = (store.clone(), store_dir.to_path_buf());
    landing_door(vec![renamed_answer("B#2")], &store, &store_dir).run(|mut door| async move {
        let frozen = freeze(&dir);
        assert_eq!(check(&mut door).await, "B#2");
        drop(frozen);
        assert_eq!(
            Index::load(&dir)
                .unwrap()
                .get("test-user")
                .map(|e| e.persisted),
            Some(true),
            "the rename's index write failed"
        );
        assert_eq!(
            read.entries.lock().unwrap().len(),
            1,
            "the index could not hold B, so nothing was saved under it"
        );

        let answer = logout(&mut door, "B#2").await;
        assert!(matches!(answer, Response::Ack { .. }), "{answer:?}");
        assert!(
            read.entries.lock().unwrap().is_empty(),
            "both names cleared: {:?}",
            read.entries.lock().unwrap()
        );
        let index = Index::load(&dir).unwrap();
        assert!(index.persisted().is_empty(), "{:?}", index.entries());
        door.finish().await;
    });

    Door::open(Server::None, Arc::new(ParkingClock::new()), store)
        .store_dir(store_dir.to_path_buf())
        .restore()
        .run(|mut door| async move {
            let restored = door.await_auth(|_| true).await;
            assert!(
                restored.accounts.is_empty(),
                "a start restores nothing: {restored:?}"
            );
            door.finish().await;
        });
}

/// C144 (no keyring entry out of the index's reach), the review of the
/// h1 tail round, its first finding: two renames, test-user to B and B
/// to C, while the index directory cannot be written and the keyring
/// saves work, then a logout of C with the index writable again. The
/// index holds a name before a credential is saved under it, so neither
/// rename saved anything: no credential is left under any of the three
/// names and the logout's `cleared` is earned. Before, each rename saved
/// under its new name and failed its index write, the index still named
/// test-user, the logout cleared test-user and C, answered `cleared`, and
/// B's credential stayed where no verb could reach it.
#[cfg(unix)]
#[test]
fn c144_renames_under_an_unwritable_index_leave_no_credential_behind() {
    let store = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let (read, dir) = (store.clone(), store_dir.to_path_buf());
    landing_door(
        vec![renamed_answer("B#2"), renamed_answer("C#3")],
        &store,
        &store_dir,
    )
    .run(|mut door| async move {
        let frozen = freeze(&dir);
        assert_eq!(check(&mut door).await, "B#2");
        assert_eq!(
            check(&mut door).await,
            "C#3",
            "the session runs on, in memory"
        );
        drop(frozen);
        let keyring = keyring_of(&mut door, "C#3").await;
        assert!(keyring.starts_with("unavailable: not saved"), "{keyring}");
        let errors = dashboard_errors(&mut door).await;
        assert!(
            errors.iter().any(|m| m.contains("keyring save skipped")),
            "{errors:?}"
        );

        let answer = logout(&mut door, "C#3").await;
        let left = read.entries.lock().unwrap().clone();
        assert!(left.is_empty(), "no credential under any name: {left:?}");
        assert!(
            matches!(
                answer,
                Response::Ack {
                    keyring: Some(KeyringOutcome::Cleared)
                }
            ),
            "{answer:?}"
        );
        door.finish().await;
    });
}

/// Issue #74 (C144, no keyring entry out of the index's reach; C51, a
/// selector names one account): a refresh renames test-user to B while
/// the index cannot be written, so the index still names the account
/// test-user. A logout of B, with the index writable again and the
/// keyring's clears failing, holds B among the entry's names and prints
/// `acq auth logout --account 'B#2'`. That command, run as printed once
/// the session is gone and the keyring works, reaches the account by the
/// held name, clears it and the index releases it. Before, the retry
/// matched neither the username test-user nor the uuid and was refused
/// as "no account matches".
#[cfg(unix)]
#[test]
fn the_printed_retry_runs_after_a_rename_whose_index_write_failed() {
    let store = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let (read, dir) = (store.clone(), store_dir.to_path_buf());
    landing_door(vec![renamed_answer("B#2")], &store, &store_dir).run(|mut door| async move {
        let frozen = freeze(&dir);
        assert_eq!(
            check(&mut door).await,
            "B#2",
            "the session runs on, in memory"
        );
        drop(frozen);
        {
            let index = Index::load(&dir).unwrap();
            let entry = index
                .get("test-user")
                .expect("the index still names test-user");
            assert_eq!(entry.keyring_reach(), ["test-user"], "{entry:?}");
        }

        *read.fail_clear.lock().unwrap() = Some("keychain locked".into());
        let answer = logout(&mut door, "B#2").await;
        let Response::Error { kind, message } = &answer else {
            panic!("a failed clear is the logout's answer: {answer:?}")
        };
        assert_eq!(*kind, ErrorKind::Internal, "{message}");
        {
            let index = Index::load(&dir).unwrap();
            let entry = index.get("test-user").unwrap();
            assert!(
                entry.keyring_names.iter().any(|n| n == "B#2"),
                "the failed clear holds B: {entry:?}"
            );
        }
        let printed = message
            .split_once("`acq auth logout --account ")
            .and_then(|(_, rest)| rest.split_once('`'))
            .map(|(word, _)| word)
            .unwrap_or_else(|| panic!("a retry command in {message}"));
        // What a POSIX shell makes of the printed word.
        let account = printed
            .strip_prefix('\'')
            .and_then(|w| w.strip_suffix('\''))
            .unwrap_or(printed);
        assert!(!account.contains('\''), "{printed}");
        let status = door.await_auth(|_| true).await;
        assert!(
            status.accounts.iter().all(|a| a.username != "B#2"),
            "the session is gone: {status:?}"
        );

        *read.fail_clear.lock().unwrap() = None;
        let answer = logout(&mut door, account).await;
        assert!(
            matches!(
                answer,
                Response::Ack {
                    keyring: Some(KeyringOutcome::Cleared)
                }
            ),
            "the printed retry runs: {answer:?}"
        );
        assert!(
            read.entries.lock().unwrap().is_empty(),
            "{:?}",
            read.entries.lock().unwrap()
        );
        let index = Index::load(&dir).unwrap();
        let entry = index.get("test-user").unwrap();
        assert!(!entry.persisted);
        assert!(entry.keyring_names.is_empty(), "B is released: {entry:?}");
        door.finish().await;
    });
}

/// C144 (the daemon reports no success it did not earn), the review of
/// h1, its first finding: a refresh's landing — the token install, the
/// index write, the session's rename — is one critical section, as a
/// login's is, so a logout that lands where the refresh releases the
/// lock finds the landing whole, and its not-persisted mark stands.
/// Before, the refresh saved its token and released the lock; the
/// logout dropped the session, failed its clear and marked the account
/// not persisted; then the refresh's index write marked it persisted
/// again, and a restart restored the account the user had logged out
/// of. The logout is run at the landing's test pause, never timed. A
/// component test: the landing's own hook (`refresh_landed`), on the
/// blocking thread, is a staging point no surface reaches.
#[tokio::test]
async fn c144_a_logout_where_a_refresh_releases_the_lock_keeps_its_mark() {
    let (transport, requests, ends) =
        scripted_transport(vec![token_answer("200 OK", TOKEN_HEADERS, None)]);
    let (mut daemon, log_path) = test_daemon_with(
        Boundary::Function,
        Provider::mock(BASE),
        transport,
        Arc::new(ParkingClock::new()),
        RailsConfig::default(),
    );
    logged_in(&mut daemon);
    let store = Arc::new(RecordingCredentialStore::failing(
        "clear",
        "keychain locked",
    ));
    store
        .entries
        .lock()
        .unwrap()
        .insert("test-user".into(), "rt-old".into());
    let dir = TestPath::dir();
    Index::load(&dir)
        .unwrap()
        .record_login("test-user", "u-1", true, 1, &[])
        .unwrap();
    {
        let daemon = Arc::get_mut(&mut daemon).unwrap();
        daemon.credential_store = store.clone();
        daemon.store_dir = Some(dir.to_path_buf());
    }
    daemon.shared.lock().unwrap().auth.one_mut().uuid = Some("u-1".into());
    let logout = Arc::new(Mutex::new(None));
    {
        let weak = Arc::downgrade(&daemon);
        let logout = logout.clone();
        *daemon.refresh_landed_hook.lock().unwrap() = Some(Box::new(move || {
            let daemon = weak.upgrade().expect("the daemon outlives its refresh");
            *logout.lock().unwrap() = Some(daemon.logout(Some("test-user")));
        }));
    }

    daemon.valid_access_token(None, false, None).await.unwrap();
    let refusal = logout
        .lock()
        .unwrap()
        .take()
        .expect("the logout ran at the pause")
        .unwrap_err();
    assert!(
        refusal
            .message
            .contains("its keyring entry could not be cleared: keychain locked"),
        "{refusal}"
    );
    assert!(
        daemon
            .shared
            .lock()
            .unwrap()
            .auth
            .find("test-user")
            .is_none()
    );
    let index = Index::load(&dir).unwrap();
    let entry = index.get("test-user").unwrap();
    assert!(
        !entry.persisted,
        "the logout's mark stands: no start restores the account"
    );
    assert_eq!(
        entry_refresh_tokens(&store.entries.lock().unwrap())
            .get("test-user")
            .map(String::as_str),
        Some("rt-rotated"),
        "the refresh's save landed before the logout"
    );
    assert_whole(&ends);
    assert_journal_matches_wire(&log_path, &requests.lock().unwrap());
    finish_harness(None, &log_path);
}

/// C144: a start whose username's keyring entry is missing reads the
/// names the index holds, in order, so a token saved under a name a
/// rename left behind is restored under the account's current name. A
/// component test: `restore_sessions` as a function.
#[test]
fn c144_restore_reaches_a_token_under_a_held_name() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    Index::load(&dir)
        .unwrap()
        .record_login("New#2", "u-1", true, 1, &["Old#1".into()])
        .unwrap();
    let store = RecordingCredentialStore::default();
    store
        .entries
        .lock()
        .unwrap()
        .insert("Old#1".into(), "rt-held".into());
    let mut log = Vec::new();
    let sessions =
        start::restore_sessions(&store, "acq-mock", &dir, None, SystemTime::now(), &mut log);
    assert_eq!(
        sessions
            .find("New#2")
            .unwrap()
            .refresh_token
            .as_ref()
            .map(auth::Secret::exposed_for_test),
        Some("rt-held")
    );
    let log = log.join("\n");
    assert!(
        log.contains("New#2 restored from its keyring entry under the old name Old#1"),
        "{log}"
    );
}

/// C144 (every load result reaches the words), the review of the h1 tail
/// round, its third finding: the index lists B persisted, holding the old
/// name A; the keyring has no entry under B and refuses the read under A.
/// The start reports A's failure as it reports a username's own, and the
/// keyring status follows it. Before, the held name's failure was
/// dropped: the start logged that the keyring had no entry and called
/// the keyring healthy. A component test: `restore_sessions` as a
/// function.
#[test]
fn c144_restore_reports_a_held_names_failed_load() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    Index::load(&dir)
        .unwrap()
        .record_login("B#2", "u-1", true, 1, &["A#1".into()])
        .unwrap();
    let store = RecordingCredentialStore::default();
    store
        .fail_load_under
        .lock()
        .unwrap()
        .insert("A#1".into(), "access denied".into());
    let mut log = Vec::new();
    let sessions =
        start::restore_sessions(&store, "acq-mock", &dir, None, SystemTime::now(), &mut log);
    assert!(sessions.by_account.is_empty());
    assert_eq!(sessions.keyring, "unavailable: access denied");
    let log = log.join("\n");
    assert!(
        log.contains("keyring read for B#2 under the old name A#1 failed: access denied"),
        "{log}"
    );
    assert!(!log.contains("has no entry"), "{log}");
}

// ---- C135: the saved access token's lifetime -----------------------------

/// The unix seconds of a wall-clock time, as the keyring entry holds it.
fn unix_secs(at: SystemTime) -> u64 {
    at.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

/// `GET /profile` answered with `test-user`'s profile (uuid `u-1`),
/// without rate headers as GGG serves it (C32; `profile` is never
/// probed).
fn profile_answer() -> ScriptedResponse {
    ScriptedResponse {
        method: "GET",
        path: "/profile",
        status: "200 OK",
        headers: String::new(),
        body: json!({ "uuid": "u-1", "name": "test-user" }).to_string(),
    }
}

/// The restart family's door: `script` answering, on `clock`, a store
/// holding `secret` under `test-user`, an account index in `store_dir`
/// listing `test-user` (uuid `u-1`) persisted, and the daemon restoring
/// its sessions from them as a start does.
fn restored_door(
    script: Vec<ScriptedResponse>,
    clock: &Arc<ParkingClock>,
    store: &Arc<RecordingCredentialStore>,
    store_dir: &std::path::Path,
    secret: &str,
) -> DoorBuilder {
    store
        .entries
        .lock()
        .unwrap()
        .insert("test-user".into(), secret.into());
    Index::load(store_dir)
        .unwrap()
        .record_login("test-user", "u-1", true, 1, &[])
        .unwrap();
    Door::open(Server::Script(script), clock.clone(), store.clone())
        .store_dir(store_dir.to_path_buf())
        .restore()
}

/// The keyring entry an hour-old login saved: `at-kept` expiring an hour
/// past `clock`'s wall, beside `rt-old`.
fn kept_entry(clock: &ParkingClock) -> String {
    json!({
        "refresh_token": "rt-old",
        "access_token": "at-kept",
        "access_expires_at": unix_secs(clock.wall()) + 3600,
    })
    .to_string()
}

/// A client's `profile` job, the non-forcing path a frontend has,
/// submitted and awaited done.
async fn profile_job(door: &mut Door) {
    let id = door.submit("profile", None).await;
    let (job, outcome) = door.await_terminal(id).await;
    assert_eq!(job.state, JobState::Done, "{outcome:?}");
}

/// The bearer the last `GET` on the wire carried.
fn last_bearer(door: &Door) -> String {
    door.received()
        .iter()
        .rev()
        .find(|request| request.method == "GET")
        .and_then(|request| request.bearer.clone())
        .expect("a GET carried a bearer")
}

/// C135 (the access token kept beside the refresh token and restored
/// with it), issue #55: a start restores, from the keyring entry, the
/// access token and its expiry beside the refresh token, so the session
/// sends no token POST while the token has an hour left, and one once
/// its expiry is reached (the margin itself is
/// `a_restored_session_refreshes_within_the_margin_of_expiry`'s). Before,
/// the entry held the refresh token alone and every lifetime's first
/// token use was a `POST oauth-token`.
#[test]
fn a_restored_session_sends_no_refresh_before_expiry() {
    let clock = Arc::new(ParkingClock::new());
    let store = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let secret = kept_entry(&clock);
    let script = vec![
        profile_answer(),
        token_answer("200 OK", TOKEN_HEADERS, None),
        profile_answer(),
    ];
    restored_door(script, &clock, &store, &store_dir, &secret).run(|mut door| async move {
        let restored = door.await_auth(|_| true).await;
        assert_eq!(
            restored.access_expires_in_seconds,
            Some(3600),
            "{restored:?}"
        );
        profile_job(&mut door).await;
        assert_eq!(
            door.journal_methods(),
            ["GET"],
            "no token POST before expiry"
        );
        assert_eq!(last_bearer(&door), "at-kept", "the restored token is sent");

        // The access token's expiry on the wall (the origin plus an
        // hour): nothing parks on it; the hand moves it.
        door.advance(Duration::from_secs(3600));
        profile_job(&mut door).await;
        assert_eq!(
            door.journal_methods(),
            ["GET", "POST", "GET"],
            "one token POST, at expiry"
        );
        assert_eq!(grants_on_the_wire(&door), ["rt-old"]);
        assert_eq!(
            last_bearer(&door),
            "at-new",
            "the expired token is refreshed"
        );
        door.finish().await;
    });
}

/// C135 at the refresh margin: a restored access token serves while more
/// than the margin (5 s) of its life is left and is refreshed, with one
/// token POST, once no more than the margin is left, though it has not
/// yet expired.
#[test]
fn a_restored_session_refreshes_within_the_margin_of_expiry() {
    let clock = Arc::new(ParkingClock::new());
    let store = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let secret = kept_entry(&clock);
    let script = vec![
        profile_answer(),
        profile_answer(),
        token_answer("200 OK", TOKEN_HEADERS, None),
        profile_answer(),
    ];
    restored_door(script, &clock, &store, &store_dir, &secret).run(|mut door| async move {
        let restored = door.await_auth(|_| true).await;
        assert_eq!(
            restored.access_expires_in_seconds,
            Some(3600),
            "{restored:?}"
        );
        profile_job(&mut door).await;
        assert_eq!(door.journal_methods(), ["GET"]);
        assert_eq!(last_bearer(&door), "at-kept");

        // To 6 s before the access token's expiry on the wall: outside
        // the refresh margin; nothing parks on it.
        door.advance(Duration::from_secs(3594));
        profile_job(&mut door).await;
        assert_eq!(door.journal_methods(), ["GET", "GET"], "6 s left: no POST");
        assert_eq!(last_bearer(&door), "at-kept");

        // To 5 s before the expiry: the refresh margin reached.
        door.advance(Duration::from_secs(1));
        profile_job(&mut door).await;
        assert_eq!(
            door.journal_methods(),
            ["GET", "GET", "POST", "GET"],
            "5 s left: one POST"
        );
        assert_eq!(last_bearer(&door), "at-new");
        door.finish().await;
    });
}

/// C135: an entry an earlier daemon saved, the bare refresh token,
/// restores as before, with no access token: the first use refreshes
/// once, on that grant, and that refresh's landing rewrites the entry in
/// the new form, the access token and its expiry beside the rotated
/// refresh token.
#[test]
fn an_entry_an_earlier_daemon_saved_restores_and_is_rewritten_at_its_first_refresh() {
    let clock = Arc::new(ParkingClock::new());
    let store = Arc::new(RecordingCredentialStore::default());
    let store_dir = TestPath::dir();
    let (read, wall) = (store.clone(), clock.clone());
    restored_door(
        vec![token_answer("200 OK", TOKEN_HEADERS, None)],
        &clock,
        &store,
        &store_dir,
        "rt-old",
    )
    .run(|mut door| async move {
        let restored = door.await_auth(|_| true).await;
        assert_eq!(restored.username.as_deref(), Some("test-user"));
        assert_eq!(restored.access_expires_in_seconds, None, "{restored:?}");

        assert_eq!(check(&mut door).await, "test-user");
        assert_eq!(door.journal_methods(), ["POST"], "one token POST");
        assert_eq!(grants_on_the_wire(&door), ["rt-old"]);
        let secret = read.entries.lock().unwrap()["test-user"].clone();
        assert!(
            secret.starts_with('{'),
            "rewritten in the new form: {secret:?}"
        );
        let entry = auth::KeyringEntry::from_secret(&secret.as_str().into(), wall.wall());
        assert_eq!(entry.refresh_token, "rt-rotated");
        assert_eq!(
            entry
                .access_token
                .as_ref()
                .map(auth::Secret::exposed_for_test),
            Some("at-new")
        );
        assert_eq!(entry.access_expires_at, Some(unix_secs(wall.wall()) + 3600));
        door.finish().await;
    });
}

/// C135: a refresh's landing saves what the session holds after the
/// install: the installed access token, its expiry as unix seconds (the
/// clock's wall plus what `AuthCheck` reads left of it) and the rotated
/// refresh token.
#[test]
fn a_refreshs_landing_saves_the_access_token_beside_the_refresh_token() {
    let clock = Arc::new(ParkingClock::new());
    let store = Arc::new(RecordingCredentialStore::default());
    let (read, wall) = (store.clone(), clock.clone());
    Door::open(
        Server::Script(vec![token_answer("200 OK", TOKEN_HEADERS, None)]),
        clock,
        store,
    )
    .sessions(logged_in_sessions())
    .run(|mut door| async move {
        let left = match door.request(Request::AuthCheck { account: None }).await {
            Response::Auth {
                access_expires_in_seconds: Some(left),
                ..
            } => left,
            other => panic!("the check answered {other:?}"),
        };
        let secret = read.entries.lock().unwrap()["test-user"].clone();
        let entry = auth::KeyringEntry::from_secret(&secret.as_str().into(), wall.wall());
        assert_eq!(
            entry
                .access_token
                .as_ref()
                .map(auth::Secret::exposed_for_test),
            Some("at-new")
        );
        assert_eq!(entry.access_expires_at, Some(unix_secs(wall.wall()) + left));
        assert_eq!(entry.refresh_token, "rt-rotated");
        door.finish().await;
    });
}
