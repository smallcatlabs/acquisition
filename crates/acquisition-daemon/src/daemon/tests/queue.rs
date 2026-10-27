//! The queue: what a client may submit, a restart, a restore, and a
//! queue write that fails.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::queue::{
    APPLY_MAX_ACTIONS, PARAMS_MAX_BYTES, SEGMENT_MAX_BYTES, SUBMITTER_MAX_BYTES,
};
use crate::daemon::*;

// ---- admission (C127) ------------------------------------------------

/// C127 (a client never submits a daemon-only kind): a client's `probe`,
/// naming a route and a URL of its own, is refused at the door with the
/// refused-work kind, before a job id exists, and nothing reaches the
/// wire. The scripted server would answer the HEAD the probe would send.
#[tokio::test]
async fn c127_a_client_probe_is_refused_and_sends_nothing() {
    let (base, requests, server) =
        scripted_server(vec![ScriptedResponse::full("HEAD", "200 OK", None, "")]).await;
    let clock = Arc::new(ManualClock::new());
    let (daemon, log_path) = test_daemon(&base, clock);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let response = daemon
        .handle_request(
            Request::Submit {
                kind: "probe".into(),
                params: json!({ "route": "fetch", "url": format!("{base}/fetch") }),
                priority: 0,
                submitted_by: "client".into(),
                account: None,
            },
            &mut None,
        )
        .await;
    match response {
        Response::Error { kind, message } => {
            assert_eq!(kind, ErrorKind::Refused, "{message}");
            assert!(
                message.contains("probe")
                    && message.contains("daemon itself")
                    && message.contains("a client cannot submit"),
                "the refusal says the rule in words: {message}"
            );
        }
        other => panic!("a client's probe was admitted: {other:?}"),
    }
    assert_nothing_admitted(&daemon, "probe");
    for _ in 0..50 {
        tokio::task::yield_now().await;
    }
    server.abort();
    assert!(
        requests.lock().unwrap().is_empty(),
        "nothing reached the wire"
    );
    finish_harness_wire(dispatcher, &log_path, &requests);
}

/// A refused submit admits no job and spends no id: the refusal came
/// before `next_id` moved, so the next admitted job is still job 1.
fn assert_nothing_admitted(daemon: &Daemon, what: &str) {
    let s = daemon.shared.lock().unwrap();
    assert!(s.jobs.is_empty(), "{what}: a refused submit admits no job");
    assert_eq!(s.next_id, 1, "{what}: a refused submit spent a job id");
}

/// A client's submit through the door every frontend uses.
async fn client_submit(
    daemon: &Arc<Daemon>,
    kind: &str,
    params: Value,
    account: Option<&str>,
) -> Response {
    daemon
        .handle_request(
            Request::Submit {
                kind: kind.into(),
                params,
                priority: 0,
                submitted_by: "client".into(),
                account: account.map(str::to_string),
            },
            &mut None,
        )
        .await
}

/// The response is a refusal with the refused-work kind whose words
/// include every one of `said`.
fn assert_refused(response: Response, what: &str, said: &[&str]) {
    match response {
        Response::Error { kind, message } => {
            assert_eq!(kind, ErrorKind::Refused, "{what}: {message}");
            for words in said {
                assert!(
                    message.contains(words),
                    "{what}: the refusal says {words:?}: {message}"
                );
            }
        }
        other => panic!("{what} was admitted: {other:?}"),
    }
}

/// A harness daemon with one live session holding a fresh access
/// token, so a network job it admitted would send at once.
fn live_daemon(base: &str) -> (Arc<Daemon>, TestPath) {
    let (daemon, log_path) = test_daemon(base, Arc::new(ManualClock::new()));
    {
        let mut s = daemon.shared.lock().unwrap();
        s.auth.rename("", "Alice#1234");
        let session = s.auth.one_mut();
        session.username = Some("Alice#1234".into());
        session.access_token = Some("at-test.Alice#1234".into());
        session.access_expires_at = Some(daemon.choke.wall() + Duration::from_secs(3600));
    }
    (daemon, log_path)
}

/// C127 (a path param is one segment): a client's path param that is not
/// exactly one segment is refused at submit, through `Submit` and through
/// an `apply` tuple, before a job id exists, and nothing reaches the
/// wire. The first case is the shape that steered a character fetch onto
/// `/account/leagues` with the bearer: `..` to climb, `?` to cut the rest
/// of the path off as a query. The server would answer anything sent.
#[tokio::test]
async fn c127_a_path_param_that_is_not_one_segment_is_refused_and_sends_nothing() {
    let (base, requests, server) = scripted_server(vec![
        ScriptedResponse::full("HEAD", "200 OK", None, ""),
        ScriptedResponse::full("GET", "200 OK", None, "{}"),
    ])
    .await;
    let (daemon, log_path) = live_daemon(&base);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let account = Some("Alice#1234");
    let cases = [
        (
            "character",
            json!({ "name": "../../account/leagues?x=" }),
            "`name`",
        ),
        ("character", json!({ "name": "a/b" }), "`name`"),
        (
            "stashes",
            json!({ "league": "../../character" }),
            "`league`",
        ),
        ("stashes", json!({ "league": ".." }), "`league`"),
        ("stashes", json!({ "league": "" }), "`league`"),
        ("stashes", json!({ "league": "\t.." }), "`league`"),
        ("stashes", json!({ "league": "a\\b" }), "`league`"),
        ("stashes", json!({ "league": 7 }), "`league`"),
        (
            "stash",
            json!({ "league": "Standard", "id": "../../profile" }),
            "`id`",
        ),
        (
            "stash",
            json!({ "league": "Standard", "id": "t1", "sub": ".." }),
            "`sub`",
        ),
        (
            "stash",
            json!({ "league": "../Standard", "id": "t1", "deep": true }),
            "`league`",
        ),
    ];
    for (kind, params, param) in cases {
        let what = format!("{kind} {params}");
        let response = client_submit(&daemon, kind, params.clone(), account).await;
        assert_refused(response, &what, &[param, "one path segment"]);
        assert_nothing_admitted(&daemon, &what);
        // The same tuple as a plan's action, where the vocabulary takes it.
        if matches!(kind, "character" | "stash") && params.get("deep").is_none() {
            let apply = json!({ "jobs": [{ "kind": kind, "params": params }] });
            let what = format!("apply of {what}");
            let response = client_submit(&daemon, "apply", apply, account).await;
            assert_refused(response, &what, &["action 0", param, "one path segment"]);
            assert_nothing_admitted(&daemon, &what);
        }
    }
    for _ in 0..50 {
        tokio::task::yield_now().await;
    }
    server.abort();
    assert!(
        requests.lock().unwrap().is_empty(),
        "nothing reached the wire"
    );
    finish_harness_wire(dispatcher, &log_path, &requests);
}

/// C127 (a path param is one segment, encoded): a league whose name holds
/// a space is a legitimate segment; it is admitted, and the probe and the
/// GET carry it percent-encoded as one segment, as the journal records.
#[tokio::test]
async fn c127_a_league_with_a_space_is_admitted_and_sent_encoded() {
    let policy = "X-Rate-Limit-Policy: stash-list-test\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 100:1:60\r\nX-Rate-Limit-Account-State: 0:1:0\r\n";
    let (base, requests, server) = scripted_server(vec![
        ScriptedResponse {
            method: "HEAD",
            path: "/stash/Hard%20Mode",
            status: "204 No Content",
            headers: policy.into(),
            body: String::new(),
        },
        ScriptedResponse {
            method: "GET",
            path: "/stash/Hard%20Mode",
            status: "200 OK",
            headers: policy.replace("0:1:0", "1:1:0"),
            body: r#"{"stashes":[]}"#.into(),
        },
    ])
    .await;
    let (daemon, log_path) = live_daemon(&base);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let response = client_submit(
        &daemon,
        "stashes",
        json!({ "league": "Hard Mode" }),
        Some("Alice#1234"),
    )
    .await;
    let Response::Submitted { id } = response else {
        panic!("a league with a space was refused: {response:?}")
    };
    let (info, outcome) = wait_terminal(&daemon, id).await;
    assert_eq!(info.state, JobState::Done, "{outcome:?}");
    server.await.unwrap();
    let paths: Vec<Value> = journal_sends(&log_path)
        .iter()
        .map(|send| send["path"].clone())
        .collect();
    assert_eq!(
        paths,
        [json!("/stash/Hard%20Mode"), json!("/stash/Hard%20Mode")]
    );
    finish_harness_wire(dispatcher, &log_path, &requests);
}

/// C127 (a path param is one segment, encoded): where a URL is rendered,
/// each path param is percent-encoded as one segment, so `%`, `#` and `?`
/// stay inside it; a value that is not one segment renders no URL at all,
/// so a row that never passed the door (a restored row, a child whose id
/// came from a response) sends nothing.
#[test]
fn c127_a_rendered_path_param_is_one_encoded_segment_or_no_url() {
    let (daemon, _) = test_daemon("http://mock", Arc::new(ManualClock::new()));
    assert_eq!(
        daemon
            .route_for("stash", &json!({ "league": "50% Off #2", "id": "a?b" }))
            .unwrap()
            .1,
        "http://mock/stash/50%25%20Off%20%232/a%3Fb"
    );
    assert_eq!(
        daemon
            .route_for("character", &json!({ "name": "Ünïcode_Exile" }))
            .unwrap()
            .1,
        "http://mock/character/%C3%9Cn%C3%AFcode_Exile"
    );
    for (kind, params) in [
        ("character", json!({ "name": "../../account/leagues?x=" })),
        ("stashes", json!({ "league": ".." })),
        (
            "stash",
            json!({ "league": "Standard", "id": "t1", "sub": "\t.." }),
        ),
    ] {
        assert_eq!(daemon.route_for(kind, &params), None, "{kind} {params}");
    }
}

/// C127 (a path param is one segment, as a server may read it): a server
/// that strips `;` path parameters (Tomcat, Jetty) reads `..;x` as `..`,
/// and one that trims trailing dots and spaces (IIS) reads `.. .` as `..`.
/// A segment whose head, the part before any `;`, is only dots and spaces
/// is refused at submit before a job id exists, and renders no URL.
#[tokio::test]
async fn c127_a_dot_segment_behind_a_semicolon_or_a_space_is_refused() {
    let (daemon, _) = test_daemon("http://127.0.0.1:9", Arc::new(ManualClock::new()));
    for league in [
        "..;",
        ";..",
        "..;x=1",
        ".;",
        ";",
        ";Standard",
        ".. .",
        "...",
        " ",
        " .. ",
    ] {
        let params = json!({ "league": league });
        let what = format!("stashes {params}");
        let response = client_submit(&daemon, "stashes", params.clone(), None).await;
        assert_refused(response, &what, &["`league`", "one path segment"]);
        assert_nothing_admitted(&daemon, &what);
        assert_eq!(daemon.route_for("stashes", &params), None, "{what}");
    }
}

/// C127 (a path param is bounded): a segment param longer than the bound
/// in UTF-8 bytes is refused at submit before a job id exists, and renders
/// no URL; one at the bound renders. Multi-byte characters count as bytes,
/// since each goes out as its percent-encoded bytes.
#[tokio::test]
async fn c127_a_path_param_longer_than_the_bound_is_refused() {
    let (daemon, _) = test_daemon("http://127.0.0.1:9", Arc::new(ManualClock::new()));
    let at_bound = "a".repeat(SEGMENT_MAX_BYTES);
    assert!(
        daemon
            .route_for("stashes", &json!({ "league": at_bound }))
            .is_some(),
        "a league at the bound renders"
    );
    for (kind, params) in [
        (
            "stashes",
            json!({ "league": "a".repeat(SEGMENT_MAX_BYTES + 1) }),
        ),
        (
            "character",
            json!({ "name": "é".repeat(SEGMENT_MAX_BYTES / 2 + 1) }),
        ),
        (
            "stash",
            json!({ "league": "Standard", "id": "t1", "sub": "f".repeat(100_000) }),
        ),
    ] {
        let what = format!("{kind} of {} bytes", params.to_string().len());
        let response = client_submit(&daemon, kind, params.clone(), None).await;
        assert_refused(
            response,
            &what,
            &[
                "one path segment",
                &format!("at most {SEGMENT_MAX_BYTES} bytes"),
                // Described by its head, type and size, never echoed whole.
                "… (a string of",
                "bytes) is not one path segment",
            ],
        );
        assert_nothing_admitted(&daemon, &what);
        assert_eq!(daemon.route_for(kind, &params), None, "{what}");
    }
}

/// C127 (a required segment is judged at the door): a `stash` without an
/// id, with or without a `sub`, and a `character` without a name are
/// refused at submit before a job id exists, as an `apply` tuple already
/// was; neither could render a URL.
#[tokio::test]
async fn c127_a_stash_without_an_id_is_refused_at_submit() {
    let (daemon, _) = test_daemon("http://127.0.0.1:9", Arc::new(ManualClock::new()));
    for (kind, params, said) in [
        (
            "stash",
            json!({ "league": "Standard", "sub": "s1" }),
            "stash needs an id",
        ),
        (
            "stash",
            json!({ "league": "Standard" }),
            "stash needs an id",
        ),
        (
            "stash",
            json!({ "id": null, "sub": "s1" }),
            "stash needs an id",
        ),
        (
            "character",
            json!({ "realm": "pc" }),
            "character needs a name",
        ),
    ] {
        let what = format!("{kind} {params}");
        let response = client_submit(&daemon, kind, params, None).await;
        assert_refused(response, &what, &[said]);
        assert_nothing_admitted(&daemon, &what);
    }
}

/// C127 (a row that renders no URL says why): a row that never passed the
/// client's door (here the daemon's own submit, as a restored row or a
/// child whose params came from a response) and whose params render no
/// URL fails naming the real cause in the door's words, and sends nothing.
#[tokio::test]
async fn c127_a_row_that_renders_no_url_fails_naming_the_cause() {
    let (base, requests, server) = scripted_server(vec![
        ScriptedResponse::full("HEAD", "200 OK", None, ""),
        ScriptedResponse::full("GET", "200 OK", None, "{}"),
    ])
    .await;
    let (daemon, log_path) = live_daemon(&base);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    for (kind, params, said) in [
        (
            "stash",
            json!({ "league": "Standard", "id": "t1", "sub": ".." }),
            "`sub` \"..\" is not one path segment",
        ),
        (
            "stash",
            json!({ "league": "..;", "id": "t1" }),
            "`league` \"..;\" is not one path segment",
        ),
        (
            "character",
            json!({ "name": "a/b" }),
            "`name` \"a/b\" is not one path segment",
        ),
        (
            "stashes",
            json!({ "league": 7 }),
            "`league` must be a string that is one path segment",
        ),
    ] {
        let what = format!("{kind} {params}");
        let id = daemon
            .submit(
                kind.into(),
                params,
                0,
                "daemon".into(),
                Some("Alice#1234".into()),
            )
            .unwrap_or_else(|e| panic!("{what}: the daemon's own submit was refused: {e}"));
        let (info, outcome) = wait_terminal(&daemon, id).await;
        assert_eq!(info.state, JobState::Failed, "{what}: {outcome:?}");
        let Outcome::Failure { error, .. } = &outcome else {
            panic!("{what}: {outcome:?}")
        };
        assert!(
            error.contains(said),
            "{what}: the failure says {said:?}: {error}"
        );
        for wrong in ["needs an id", "needs a name", "unrenderable realm"] {
            assert!(
                !error.contains(wrong),
                "{what}: a misleading cause: {error}"
            );
        }
    }
    server.abort();
    assert!(
        requests.lock().unwrap().is_empty(),
        "nothing reached the wire"
    );
    finish_harness_wire(dispatcher, &log_path, &requests);
}

/// C127 (a row that renders no URL spends no token): a row whose params
/// render no URL fails in the door's words before the job authenticates,
/// so a session whose refresh is due sends no refresh for it. Every kind
/// whose path params can render no URL past the daemon's own submit is in
/// the table, so a handler that authenticates first is caught. Not here:
/// `leagues` and `profile` always render, and `characters` renders none
/// only for a realm the daemon's own submit refuses.
#[tokio::test]
async fn c127_a_row_that_renders_no_url_fails_before_any_refresh() {
    let (base, requests, server) = token_server_answering(vec![]).await;
    let (mut daemon, log_path) = test_daemon(&base, Arc::new(ManualClock::new()));
    logged_in(&mut daemon);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    for (kind, params, said) in [
        (
            "stash",
            json!({ "league": "Standard", "id": "t1", "sub": ".." }),
            "`sub` \"..\" is not one path segment",
        ),
        (
            "character",
            json!({ "name": "a/b" }),
            "`name` \"a/b\" is not one path segment",
        ),
        (
            "stashes",
            json!({ "league": 7 }),
            "`league` must be a string that is one path segment",
        ),
    ] {
        let what = format!("{kind} {params}");
        let id = daemon
            .submit(
                kind.into(),
                params,
                0,
                "daemon".into(),
                Some("test-user".into()),
            )
            .unwrap_or_else(|e| panic!("{what}: the daemon's own submit was refused: {e}"));
        let (info, outcome) = wait_terminal(&daemon, id).await;
        assert_eq!(info.state, JobState::Failed, "{what}: {outcome:?}");
        let Outcome::Failure { error, .. } = &outcome else {
            panic!("{what}: {outcome:?}")
        };
        assert!(
            error.contains(said),
            "{what}: the failure says {said:?}: {error}"
        );
    }
    server.abort();
    assert!(
        requests.lock().unwrap().is_empty(),
        "the token server saw no request"
    );
    finish_harness_wire(dispatcher, &log_path, &requests);
}

/// C127 and the 4b sitting round: the ad-hoc `refresh` kind is retired
/// (a selection is a plan, applied as `apply`), so a client naming it is
/// refused at the door as any kind the table does not list, before an id
/// exists, and nothing is admitted or sent.
#[tokio::test]
async fn c127_the_retired_refresh_kind_is_refused_at_submit() {
    let (base, requests, server) = token_server_answering(vec![]).await;
    let (mut daemon, log_path) = test_daemon(&base, Arc::new(ManualClock::new()));
    logged_in(&mut daemon);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    for params in [
        json!({ "league": "Standard", "all": true }),
        json!({ "league": "Standard", "tabs": ["t1"] }),
    ] {
        let what = format!("refresh {params}");
        let response = client_submit(&daemon, "refresh", params, Some("test-user")).await;
        assert_refused(response, &what, &["unknown job kind"]);
        assert_nothing_admitted(&daemon, &what);
    }
    for _ in 0..50 {
        tokio::task::yield_now().await;
    }
    server.abort();
    assert!(
        requests.lock().unwrap().is_empty(),
        "the token server saw no request"
    );
    finish_harness_wire(dispatcher, &log_path, &requests);
}

/// C127 (a row that renders no URL spends no token), for `characters`:
/// its one no-URL case is a realm the daemon's own submit refuses, so the
/// handler is called directly, on a session whose refresh is due. It fails
/// in the door's words, and the journal equals the wire, both empty.
#[tokio::test]
async fn c127_a_characters_row_that_renders_no_url_fails_before_any_refresh() {
    let (base, requests, server) = token_server_answering(vec![]).await;
    let (mut daemon, log_path) = test_daemon(&base, Arc::new(ManualClock::new()));
    logged_in(&mut daemon);
    let params = json!({ "realm": "nope" });
    let said = admit_realm("characters", &params)
        .expect_err("the door refuses the realm")
        .message;
    let outcome = daemon
        .execute_inner(
            9,
            "characters",
            params,
            None,
            Some("test-user"),
            daemon.choke.now(),
        )
        .await
        .unwrap_or_else(|e| panic!("characters: {e:?}"));
    let Outcome::Failure { error, .. } = &outcome else {
        panic!("characters with realm \"nope\": {outcome:?}")
    };
    assert_eq!(error, &said, "the failure is the door's words");
    assert!(error.contains("nope"), "{error}");
    server.abort();
    assert!(
        requests.lock().unwrap().is_empty(),
        "the token server saw no request"
    );
    assert_journal_matches_wire(&log_path, &requests.lock().unwrap());
    finish_harness(None, &log_path);
}

/// C127 (no legitimate value is refused or changed): realistic league,
/// character, stash and substash values on every realm a family takes,
/// each run through `route_for` and then `url::Url::parse` (what reqwest
/// does with the string), map to exactly these wire paths. A guard: it
/// holds before and after any tightening of the segment rule.
#[test]
fn c127_legitimate_values_render_to_these_wire_paths() {
    let (daemon, _) = test_daemon("https://api.pathofexile.com", Arc::new(ManualClock::new()));
    let table = [
        (
            "stashes",
            json!({ "league": "Standard" }),
            "/stash/Standard",
        ),
        (
            "stashes",
            json!({ "league": "Hardcore" }),
            "/stash/Hardcore",
        ),
        ("stashes", json!({}), "/stash/Standard"),
        (
            "stashes",
            json!({ "league": "Phrecia 2.0" }),
            "/stash/Phrecia%202.0",
        ),
        (
            "stashes",
            json!({ "league": "SSF Standard" }),
            "/stash/SSF%20Standard",
        ),
        (
            "stashes",
            json!({ "league": "HC SSF Settlers of Kalguur" }),
            "/stash/HC%20SSF%20Settlers%20of%20Kalguur",
        ),
        (
            "stashes",
            json!({ "league": "My League (PL12345)" }),
            "/stash/My%20League%20(PL12345)",
        ),
        (
            "stashes",
            json!({ "league": "Event #2" }),
            "/stash/Event%20%232",
        ),
        (
            "stashes",
            json!({ "league": "Standard", "realm": "xbox" }),
            "/stash/xbox/Standard",
        ),
        (
            "stash",
            json!({ "league": "Standard", "id": "0a1b2c3d4e" }),
            "/stash/Standard/0a1b2c3d4e",
        ),
        (
            "stash",
            json!({ "league": "Phrecia 2.0", "id": "0a1b2c3d4e", "sub": "f9e8d7c6b5", "deep": false }),
            "/stash/Phrecia%202.0/0a1b2c3d4e/f9e8d7c6b5",
        ),
        (
            "stash",
            json!({ "league": "Hardcore", "id": "0a1b2c3d4e", "realm": "xbox" }),
            "/stash/xbox/Hardcore/0a1b2c3d4e",
        ),
        ("characters", json!({}), "/character"),
        ("characters", json!({ "realm": "poe2" }), "/character/poe2"),
        ("characters", json!({ "realm": "xbox" }), "/character/xbox"),
        (
            "character",
            json!({ "name": "Éowyn_Tempête" }),
            "/character/%C3%89owyn_Temp%C3%AAte",
        ),
        (
            "character",
            json!({ "name": "Søren_the_Exile", "realm": "poe2" }),
            "/character/poe2/S%C3%B8ren_the_Exile",
        ),
        (
            "character",
            json!({ "name": "Witch_HC", "realm": "xbox" }),
            "/character/xbox/Witch_HC",
        ),
    ];
    for (kind, params, path) in table {
        let what = format!("{kind} {params}");
        assert_eq!(admit_segments(kind, &params), Ok(()), "{what} is admitted");
        let (_, rendered) = daemon
            .route_for(kind, &params)
            .unwrap_or_else(|| panic!("{what} rendered no URL"));
        let wire = url::Url::parse(&rendered).unwrap();
        assert_eq!(wire.host_str(), Some("api.pathofexile.com"), "{what}");
        assert_eq!(wire.query(), None, "{what}");
        assert_eq!(wire.fragment(), None, "{what}");
        assert_eq!(wire.path(), path, "{what}");
        assert_eq!(wire.as_str(), rendered, "{what}: the parse changed nothing");
    }
}

/// C127 (a mock-only kind is the mock's): in real mode a client's
/// `sleep`, `fetch` and `whoami` are refused at the door, before a job id
/// exists, and nothing reaches the wire. Real in name only: every URL is
/// the local server's, which would answer one request of any kind.
#[tokio::test]
async fn c127_a_mock_only_kind_is_refused_in_real_mode() {
    let (base, requests, server) =
        scripted_server(vec![ScriptedResponse::full("GET", "200 OK", None, "{}")]).await;
    let provider = Provider {
        name: acquisition_protocol::provider::GGG,
        ..Provider::mock(&base)
    };
    assert!(provider.is_real());
    let (daemon, log_path) = test_daemon_with(
        provider,
        Arc::new(ManualClock::new()),
        RailsConfig::default(),
    );
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    for kind in ["sleep", "fetch", "whoami"] {
        let response = client_submit(&daemon, kind, json!({ "seconds": 3600 }), None).await;
        assert_refused(response, kind, &[kind, "mock-only", "real mode"]);
        assert_nothing_admitted(&daemon, kind);
    }
    for _ in 0..50 {
        tokio::task::yield_now().await;
    }
    server.abort();
    assert!(
        requests.lock().unwrap().is_empty(),
        "nothing reached the wire"
    );
    finish_harness_wire(dispatcher, &log_path, &requests);
}

/// C127 (a sleep is bounded): the mock's `sleep` with seconds that are
/// negative, beyond the bound or not a number is refused at submit, before
/// a job id exists; a sleep within the bound is admitted as job 1. A row
/// that never passed the door fails as a job instead of panicking.
#[tokio::test]
async fn c127_a_sleep_with_bad_seconds_is_refused_at_submit() {
    let (daemon, log_path) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    for seconds in [
        json!(-1),
        json!(1e300),
        json!(3601),
        json!("5"),
        json!(true),
    ] {
        let what = format!("sleep {seconds}");
        let response = client_submit(&daemon, "sleep", json!({ "seconds": seconds }), None).await;
        assert_refused(response, &what, &["sleep", "`seconds`", "from 0 to 3600"]);
        assert_nothing_admitted(&daemon, &what);
    }
    let response = client_submit(&daemon, "sleep", json!({ "seconds": 0 }), None).await;
    let Response::Submitted { id } = response else {
        panic!("a sleep within the bound was refused: {response:?}")
    };
    assert_eq!(id, 1, "the refusals spent no id");
    let (info, _) = wait_terminal(&daemon, id).await;
    assert_eq!(info.state, JobState::Done);
    let outcome = daemon
        .execute_inner(
            9,
            "sleep",
            json!({ "seconds": -1 }),
            None,
            None,
            daemon.choke.now(),
        )
        .await
        .unwrap();
    assert!(
        matches!(&outcome, Outcome::Failure { error, .. } if error.contains("seconds")),
        "{outcome:?}"
    );
    finish_harness(dispatcher, &log_path);
}

/// C127 (the kind table): a kind the daemon does not run is refused at
/// the door, before a job id exists, naming only the kinds a client may
/// submit in this provider mode: never the daemon's own `probe`, and in
/// real mode none of the mock-only kinds the door would refuse there.
#[tokio::test]
async fn c127_an_unknown_kind_is_refused_naming_only_client_kinds() {
    for real in [false, true] {
        let provider = if real {
            Provider {
                name: acquisition_protocol::provider::GGG,
                ..Provider::mock("http://127.0.0.1:1")
            }
        } else {
            Provider::mock("http://127.0.0.1:1")
        };
        assert_eq!(provider.is_real(), real);
        let (daemon, log_path) = test_daemon_with(
            provider,
            Arc::new(ManualClock::new()),
            RailsConfig::default(),
        );
        let what = if real {
            "bogus in real mode"
        } else {
            "bogus in mock mode"
        };
        let response = client_submit(&daemon, "bogus", json!({}), None).await;
        let message = match &response {
            Response::Error { message, .. } => message.clone(),
            other => panic!("{what}: an unknown kind was admitted: {other:?}"),
        };
        let (named, unnamed): (&[&str], &[&str]) = if real {
            (
                &["profile", "stash", "apply"],
                &["probe", "sleep", "fetch", "whoami"],
            )
        } else {
            (
                &["profile", "stash", "apply", "sleep", "fetch", "whoami"],
                &["probe"],
            )
        };
        assert_refused(response, what, &["unknown job kind \"bogus\""]);
        for kind in named {
            assert!(message.contains(kind), "{what}: names {kind}: {message}");
        }
        for kind in unnamed {
            assert!(!message.contains(kind), "{what}: names {kind}: {message}");
        }
        assert_nothing_admitted(&daemon, what);
        finish_harness(None, &log_path);
    }
}

/// C127 (one bound on `sleep`): a row that never passed the door (a
/// restored row) whose `seconds` the door would refuse fails when run, in
/// the door's own words, rather than run a default: a string `seconds`
/// once slept 3 s.
#[tokio::test]
async fn c127_a_restored_sleep_the_door_would_refuse_fails_in_its_words() {
    let (daemon, log_path) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
    for seconds in [json!("1"), json!(true), json!(-1), json!(3601)] {
        let what = format!("sleep {seconds}");
        let params = json!({ "seconds": seconds });
        let door = match client_submit(&daemon, "sleep", params.clone(), None).await {
            Response::Error { message, .. } => message,
            other => panic!("{what}: the door admitted it: {other:?}"),
        };
        let outcome = daemon
            .execute_inner(9, "sleep", params, None, None, daemon.choke.now())
            .await
            .unwrap();
        match &outcome {
            Outcome::Failure { error, .. } => assert_eq!(error, &door, "{what}"),
            other => panic!("{what}: the runner did not fail: {other:?}"),
        }
    }
    finish_harness(None, &log_path);
}

/// C127 (a refused value is described, never echoed whole): a client's
/// value the door refuses is quoted by its head, its JSON type and its
/// size, so the refusal fits the response bound whatever the request
/// held. Each value here fits a request frame, and each is 20 MiB of
/// backslashes, which every escape doubles: quoted whole, its refusal
/// would exceed the response frame and be lost.
#[tokio::test]
async fn c127_a_refused_value_is_described_by_its_type_and_size_within_the_response_bound() {
    let (daemon, log_path) = test_daemon("http://127.0.0.1:1", Arc::new(ManualClock::new()));
    let huge = "\\".repeat(20 << 20);
    let huge_len = huge.len().to_string();
    let submit = |kind: &str, params: Value, account: Option<&str>| Request::Submit {
        kind: kind.into(),
        params,
        priority: 0,
        submitted_by: "client".into(),
        account: account.map(str::to_string),
    };
    let cases = [
        (
            "a league array",
            submit("stashes", json!({ "league": [huge] }), None),
            ErrorKind::Refused,
            vec![
                "`league` must be a string",
                "(an array of",
                "bytes as JSON)",
            ],
        ),
        (
            "a sleep's array seconds",
            submit("sleep", json!({ "seconds": [huge] }), None),
            ErrorKind::Refused,
            vec![
                "`seconds` must be a number",
                "(an array of",
                "bytes as JSON)",
            ],
        ),
        (
            "a substash id that is no segment",
            submit("stash", json!({ "id": "t1", "sub": huge }), None),
            ErrorKind::Refused,
            vec!["not one path segment", "(a string of", &huge_len],
        ),
        (
            "an unknown kind",
            submit(&huge, json!({}), None),
            ErrorKind::Refused,
            vec!["unknown job kind", "(a string of", &huge_len],
        ),
        (
            "an apply's budget",
            submit(
                "apply",
                json!({ "jobs": [{ "kind": "stashes", "params": {} }], "max_requests": [huge] }),
                None,
            ),
            ErrorKind::Refused,
            vec!["max_requests must be", "(an array of", "bytes as JSON)"],
        ),
        (
            "an apply's kind",
            submit(
                "apply",
                json!({ "jobs": [{ "kind": huge, "params": {} }] }),
                None,
            ),
            ErrorKind::Refused,
            vec!["not in the plan vocabulary", "(a string of", &huge_len],
        ),
        (
            "a realm",
            submit("stashes", json!({ "realm": huge }), None),
            ErrorKind::Refused,
            vec!["realm must be a string", "(a string of", &huge_len],
        ),
        (
            "an account selector",
            submit("profile", json!({}), Some(&huge)),
            ErrorKind::NotLoggedIn,
            vec!["account", "not logged in", "(a string of", &huge_len],
        ),
    ];
    for (what, request, kind, said) in cases {
        let request_bytes = serde_json::to_vec(&request).unwrap().len() + 1;
        assert!(
            request_bytes <= MAX_FRAME_BYTES,
            "{what}: the request fits its frame ({request_bytes} bytes)"
        );
        let response = daemon.handle_request(request, &mut None).await;
        let response_bytes = serde_json::to_vec(&response).unwrap().len() + 1;
        assert!(
            response_bytes <= MAX_FRAME_BYTES,
            "{what}: the refusal fits the response frame ({response_bytes} bytes)"
        );
        let Response::Error { kind: got, message } = response else {
            panic!("{what} was admitted: {response:?}")
        };
        assert_eq!(got, kind, "{what}: {message}");
        assert!(
            message.len() <= 1024,
            "{what}: the refusal quotes a bounded preview, not {} bytes",
            message.len()
        );
        for words in said {
            assert!(message.contains(words), "{what}: says {words:?}: {message}");
        }
        assert_nothing_admitted(&daemon, what);
    }
    finish_harness(None, &log_path);
}

/// C127 (a submit's params have a bound): params one byte over
/// `PARAMS_MAX_BYTES` as JSON are refused at submit, naming their size and
/// not their value, and spend no id, as is a `submitted_by` over
/// `SUBMITTER_MAX_BYTES`; params at the bound are admitted,
/// and so is an `apply` the size of the largest plan observed, the
/// owner's Standard: 3,347 tabs, 3,009 of them substashes, and 59
/// characters (N47).
#[tokio::test]
async fn c127_params_over_their_bound_are_refused_and_a_large_plan_is_admitted() {
    let (daemon, log_path) = live_daemon("http://127.0.0.1:1");
    // `{"pad":"…"}` is ten bytes around the string.
    let pad = |len: usize| json!({ "pad": "x".repeat(len - 10) });
    let over = pad(PARAMS_MAX_BYTES + 1);
    assert_eq!(
        serde_json::to_vec(&over).unwrap().len(),
        PARAMS_MAX_BYTES + 1
    );
    let response = client_submit(&daemon, "sleep", over, None).await;
    let Response::Error { kind, message } = &response else {
        panic!("params over the bound were admitted: {response:?}")
    };
    assert_eq!(*kind, ErrorKind::Refused, "{message}");
    assert!(
        message.contains(&(PARAMS_MAX_BYTES + 1).to_string())
            && message.contains(&PARAMS_MAX_BYTES.to_string()),
        "the refusal names the size and the bound: {message}"
    );
    assert!(
        message.len() <= 1024 && !message.contains("xxxx"),
        "the refusal quotes nothing of the value: {message}"
    );
    assert_nothing_admitted(&daemon, "params over the bound");
    // A submit's other stored and echoed string, its submitter, has a
    // bound of its own, and is described, never echoed.
    let submitter = "y".repeat(20 << 20);
    let response = daemon
        .handle_request(
            Request::Submit {
                kind: "sleep".into(),
                params: json!({}),
                priority: 0,
                submitted_by: submitter,
                account: None,
            },
            &mut None,
        )
        .await;
    assert_refused(
        response,
        "a submitter over its bound",
        &[
            "submitted_by must be at most",
            &SUBMITTER_MAX_BYTES.to_string(),
            "(a string of 20971520 bytes)",
        ],
    );
    assert_nothing_admitted(&daemon, "a submitter over its bound");

    let mut jobs = vec![
        json!({ "kind": "stashes", "params": { "realm": "pc", "league": "Standard" } }),
        json!({ "kind": "characters", "params": { "realm": "pc" } }),
    ];
    for i in 0..338 {
        jobs.push(json!({ "kind": "stash", "params": {
            "realm": "pc", "league": "Standard", "id": format!("{i:010x}"), "deep": false,
        } }));
    }
    for i in 0..3009 {
        jobs.push(json!({ "kind": "stash", "params": {
            "realm": "pc", "league": "Standard", "id": format!("{:010x}", i % 14),
            "sub": format!("{i:010x}"), "deep": false,
        } }));
    }
    for i in 0..59 {
        jobs.push(json!({ "kind": "character", "params": {
            "realm": "pc", "name": format!("ACharacterName{i}"),
        } }));
    }
    let plan = json!({ "jobs": jobs, "max_requests": 3408 });
    let response = client_submit(&daemon, "apply", plan, None).await;
    assert!(
        matches!(response, Response::Submitted { id: 1 }),
        "the largest plan observed is admitted, as job 1: {response:?}"
    );
    let response = client_submit(&daemon, "sleep", pad(PARAMS_MAX_BYTES), None).await;
    assert!(
        matches!(response, Response::Submitted { id: 2 }),
        "params at the bound are admitted: {response:?}"
    );
    finish_harness(None, &log_path);
}

/// C136 (the submitter `daemon` is the daemon's own): a client's submit
/// that names it is refused at the door with the refused-work kind,
/// before an id exists, in words that say the rule, and nothing reaches
/// the wire. The daemon holds a live session, so the
/// profile fetch, admitted, would send at once: the same submit under a
/// frontend's name is then admitted as job 1 and sends its one GET.
#[tokio::test]
async fn c136_a_client_submit_as_the_daemon_is_refused_and_sends_nothing() {
    let (base, sent_log, _arrived, _release) = account_server(("GET", "/never", 0)).await;
    let (daemon, log_path) = live_daemon(&base);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    let submit_as = |submitted_by: &str| Request::Submit {
        kind: "profile".into(),
        params: json!({}),
        priority: 0,
        submitted_by: submitted_by.into(),
        account: None,
    };

    let response = daemon.handle_request(submit_as("daemon"), &mut None).await;
    assert_refused(
        response,
        "a client submit as the daemon",
        &["reserved for the daemon's own work", "a client cannot"],
    );
    assert_nothing_admitted(&daemon, "a client submit as the daemon");
    for _ in 0..50 {
        tokio::task::yield_now().await;
    }
    assert!(
        sent_log.lock().unwrap().is_empty(),
        "nothing reached the wire"
    );

    let response = daemon.handle_request(submit_as("cli:1"), &mut None).await;
    assert!(
        matches!(response, Response::Submitted { id: 1 }),
        "a frontend's submit is admitted as job 1: {response:?}"
    );
    let (info, _) = wait_terminal(&daemon, 1).await;
    assert_eq!(info.state, JobState::Done);
    assert_eq!(
        wire_methods(&sent_log),
        ["GET"],
        "the admitted fetch sends once"
    );
    assert_journal_matches_wire(&log_path, &wire_methods(&sent_log));
    finish_harness(Some(dispatcher), &log_path);
}

/// C127 (an apply's actions have a bound): an `apply` of one action over
/// `APPLY_MAX_ACTIONS` is refused at submit, naming its count and the
/// bound and pointing to a narrower selection, and spends no id, though
/// its tiny tuples sit far under `PARAMS_MAX_BYTES`; the bound wins over a
/// `max_requests` the plan also exceeds. An `apply` of exactly `APPLY_MAX_ACTIONS` is
/// admitted (the owner-sized plan is admitted above).
#[tokio::test]
async fn c127_an_apply_over_its_action_bound_is_refused_and_one_at_it_is_admitted() {
    let (daemon, log_path) = live_daemon("http://127.0.0.1:1");
    let jobs = |n: usize| -> Vec<Value> { vec![json!({ "kind": "stashes", "params": {} }); n] };
    let over = APPLY_MAX_ACTIONS + 1;
    for (what, plan) in [
        (
            "an apply over its action bound",
            json!({ "jobs": jobs(over) }),
        ),
        (
            "an apply over its action bound and its budget",
            json!({ "jobs": jobs(over), "max_requests": 1 }),
        ),
    ] {
        assert!(serde_json::to_vec(&plan).unwrap().len() < PARAMS_MAX_BYTES);
        let response = client_submit(&daemon, "apply", plan, None).await;
        assert_refused(
            response,
            what,
            &[
                &format!("an apply of {over} actions"),
                &format!("bound of {APPLY_MAX_ACTIONS} actions"),
                "nothing was submitted",
                "covers fewer tabs or characters",
            ],
        );
        assert_nothing_admitted(&daemon, what);
    }
    let plan = json!({ "jobs": jobs(APPLY_MAX_ACTIONS), "max_requests": APPLY_MAX_ACTIONS });
    let response = client_submit(&daemon, "apply", plan, None).await;
    assert!(
        matches!(response, Response::Submitted { id: 1 }),
        "an apply at its action bound is admitted, as job 1: {response:?}"
    );
    finish_harness(None, &log_path);
}

// ---- persistence (C6) ------------------------------------------------

/// A harness daemon on a file-backed `daemon.db` (test daemons default
/// to a throwaway in-memory one), restored from it like `run` does.
fn persisting_daemon(base: &str, db_path: &std::path::Path) -> (Arc<Daemon>, TestPath) {
    let (mut daemon, log_path) = test_daemon(base, Arc::new(ManualClock::new()));
    Arc::get_mut(&mut daemon).unwrap().jobs_db = Mutex::new(JobDb::open(db_path).unwrap());
    daemon.restore_jobs(Retention::default()).unwrap();
    (daemon, log_path)
}

/// Make every later statement on the daemon's queue fail: the table
/// is dropped through a second connection to the same file, as a
/// neighbour corrupting `daemon.db` under a running daemon would, so
/// the queue-failure handling meets a real failure and the store
/// crate needs no test-only door (its `test-hooks` feature, retired
/// 2026-09-11: a feature unified into `--all-targets` builds gave
/// every binary two forms).
fn break_queue(db: &std::path::Path) {
    rusqlite::Connection::open(db)
        .unwrap()
        .execute_batch("DROP TABLE jobs")
        .unwrap();
}

#[tokio::test]
async fn the_queue_and_results_survive_a_daemon_restart() {
    let db = test_db_path();
    // Lifetime 1: one job finished, one running, one waiting. No
    // dispatcher runs — the running state is written the way
    // `process` writes it — so nothing of this lifetime survives the
    // drop to write behind lifetime 2's back.
    let (daemon, log1) = persisting_daemon("http://127.0.0.1:1", &db);
    let done = daemon
        .submit(
            "sleep".into(),
            json!({ "seconds": 0.01 }),
            0,
            "a".into(),
            None,
        )
        .unwrap();
    daemon.start_and_finish(
        done,
        Outcome::Success {
            payload: json!({ "slept_seconds": 0.01 }),
        },
    );
    let running = daemon
        .submit(
            "sleep".into(),
            json!({ "seconds": 0.3 }),
            2,
            "b".into(),
            None,
        )
        .unwrap();
    {
        let mut s = daemon.shared.lock().unwrap();
        let entry = s.jobs.get_mut(&running).unwrap();
        entry.info.state = JobState::Running;
        daemon.persist(entry);
    }
    let waiting = daemon
        .submit(
            "sleep".into(),
            json!({ "seconds": 0.01 }),
            1,
            "c".into(),
            None,
        )
        .unwrap();
    daemon.set_priority(waiting, 5).unwrap();
    drop(daemon);

    // Lifetime 2: the running job is re-queued, the waiting one kept
    // with its reprioritization, ids continue, and the finished job's
    // result is still answerable over the protocol.
    let (daemon, log2) = persisting_daemon("http://127.0.0.1:1", &db);
    let jobs = daemon.shared.lock().unwrap().list(&daemon);
    let summary: Vec<(JobId, JobState, Priority)> =
        jobs.iter().map(|j| (j.id, j.state, j.priority)).collect();
    assert_eq!(
        summary,
        [
            (running, JobState::Waiting, 2),
            (waiting, JobState::Waiting, 5)
        ],
        "open jobs come back; terminal ones stay in the table"
    );
    assert_eq!(jobs[0].submitted_by, "b", "every field rides along");
    let fresh = daemon
        .submit(
            "sleep".into(),
            json!({ "seconds": 0.01 }),
            0,
            "d".into(),
            None,
        )
        .unwrap();
    assert_eq!(fresh, waiting + 1, "ids continue across the restart");
    match daemon
        .handle_request(Request::Result { id: done }, &mut None)
        .await
    {
        Response::Result {
            outcome: Outcome::Success { payload },
            ..
        } => {
            assert_eq!(payload["slept_seconds"], json!(0.01))
        }
        other => panic!("previous lifetime's result not served: {other:?}"),
    }
    match daemon
        .handle_request(Request::Result { id: 999 }, &mut None)
        .await
    {
        Response::Error { message, .. } => assert_eq!(message, "no job 999"),
        other => panic!("{other:?}"),
    }

    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    for id in [running, waiting, fresh] {
        let (info, _) = wait_terminal(&daemon, id).await;
        assert_eq!(info.state, JobState::Done, "job {id} runs to completion");
    }
    dispatcher.abort();
    // Lifetime 3 has nothing to restore.
    drop(daemon);
    let (daemon, log3) = persisting_daemon("http://127.0.0.1:1", &db);
    assert!(daemon.shared.lock().unwrap().jobs.is_empty());
    for p in [log1, log2, log3] {
        finish_harness(None, &p);
    }
}

/// C50 (the fetch belongs to its login's lifetime): a login's profile
/// fetch the last lifetime left waiting (a stop that landed after GGG
/// answered the code exchange, #59) fails at restore, in words that name
/// the lost login and send the user to log in again, and nothing is sent
/// for it. The account's session restores with a refresh token and no
/// access token, so a replay would cost a token POST and a GET with no
/// one waiting. The same fetch a frontend submitted, waiting beside it,
/// still replays: one POST and one GET reach the wire, the account
/// server's, and the journal matches them.
#[tokio::test]
async fn c50_a_lost_logins_profile_fetch_fails_at_restore_and_sends_nothing() {
    let (base, sent_log, _arrived, _release) = account_server(("GET", "/never", 0)).await;
    let db = test_db_path();
    {
        let db = JobDb::open(&db).unwrap();
        let waiting_profile = |id: JobId, submitted_by: &str| acquisition_store::jobs::JobRow {
            id,
            kind: "profile".into(),
            state: "waiting".into(),
            priority: 0,
            submitted_by: submitted_by.into(),
            parent: None,
            retries: 0,
            account: Some("test-user".into()),
            params: json!({}),
            outcome: None,
            deferred: None,
            cancel_requested: false,
            submitted_at: 0,
            updated_at: 0,
        };
        db.upsert(&waiting_profile(1, "daemon")).unwrap();
        db.upsert(&waiting_profile(2, "cli:1")).unwrap();
    }
    let (mut daemon, log_path) = persisting_daemon(&base, &db);
    logged_in(&mut daemon);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());

    let (login, outcome) = wait_terminal(&daemon, 1).await;
    assert_eq!(login.state, JobState::Failed, "{outcome:?}");
    let Outcome::Failure { error, .. } = &outcome else {
        panic!("{outcome:?}")
    };
    assert!(
        error.contains("login") && error.contains("log in again") && !error.contains('1'),
        "the failure names the lost login and the remedy, no id: {error}"
    );
    let (client, outcome) = wait_terminal(&daemon, 2).await;
    assert_eq!(client.state, JobState::Done, "{outcome:?}");
    assert_eq!(
        sent_log.lock().unwrap().as_slice(),
        [sent("test-user", "POST"), sent("test-user", "GET")],
        "the frontend's fetch replays; the lost login's sends nothing"
    );
    assert_eq!(
        JobDb::open(&db).unwrap().get(1).unwrap().unwrap().state,
        JobState::Failed.to_string(),
        "the failure is on disk"
    );
    assert_eq!(
        JobDb::open(&db).unwrap().get(2).unwrap().unwrap().state,
        JobState::Done.to_string(),
        "the wire's two sends are the frontend's fetch"
    );
    assert_journal_matches_wire(&log_path, &wire_methods(&sent_log));
    finish_harness(dispatcher, &log_path);
}

#[tokio::test]
async fn restore_drops_probes_and_finishes_a_parent_whose_children_are_done() {
    let db = test_db_path();
    {
        let db = JobDb::open(&db).unwrap();
        let row = |id: u64, kind: &str, state: &str, parent: Option<u64>| {
            acquisition_store::jobs::JobRow {
                id,
                kind: kind.into(),
                state: state.into(),
                priority: 0,
                submitted_by: "t".into(),
                parent,
                retries: 0,
                account: Some("A#1".into()),
                params: json!({ "route": "fetch@A#1", "league": "Standard" }),
                outcome: (state == "done").then(|| json!({ "outcome": "success", "payload": {} })),
                deferred: None,
                cancel_requested: false,
                submitted_at: 0,
                updated_at: 0,
            }
        };
        // An apply whose last child finished just before the daemon
        // died: its own result is held, deferred.
        let mut parent = row(1, "apply", "running", None);
        parent.deferred = Some(json!({ "outcome": "success", "payload": { "tabs_listed": 1 } }));
        db.upsert(&parent).unwrap();
        db.upsert(&row(2, "stash", "done", Some(1))).unwrap();
        // Per-lifetime: a probe from the previous daemon.
        db.upsert(&row(3, "probe", "waiting", None)).unwrap();
        // A running job that was asked to cancel: it never ran again.
        let mut cancelling = row(4, "sleep", "running", None);
        cancelling.cancel_requested = true;
        db.upsert(&cancelling).unwrap();
        // Running on a declared no-probe route: a replay would send
        // against an empty limiter, so it is not replayed.
        db.upsert(&row(5, "leagues", "running", None)).unwrap();
        // Mid-fan-out: children submitted, held result not yet
        // written. Re-running would duplicate the children.
        db.upsert(&row(6, "apply", "running", None)).unwrap();
        db.upsert(&row(7, "stash", "waiting", Some(6))).unwrap();
    }
    let (daemon, log) = persisting_daemon("http://127.0.0.1:1", &db);
    let jobs = daemon.job_list().unwrap();
    let states: Vec<(JobId, JobState)> = jobs.iter().map(|j| (j.id, j.state)).collect();
    assert_eq!(
        states,
        [
            (1, JobState::Done),
            (2, JobState::Done),
            (4, JobState::Cancelled),
            (5, JobState::Failed),
            (6, JobState::Running),
            (7, JobState::Waiting)
        ],
        "no-probe replays fail; a mid-fan-out parent holds for its children"
    );
    let Outcome::Failure { error, .. } = held(&daemon, 5).unwrap().1.unwrap() else {
        panic!("the leagues replay did not fail")
    };
    assert!(error.contains("not replayed"), "{error}");
    // The held parent finishes when its remaining child does — as
    // interrupted, never success: the full child set is unknown.
    daemon.cancel(7).unwrap();
    let Outcome::Failure {
        error, children, ..
    } = held(&daemon, 6).unwrap().1.unwrap()
    else {
        panic!("held parent must finish as interrupted, not success")
    };
    assert!(error.contains("mid fan-out"), "{error}");
    // C23: the deferred failure keeps its message and gains the tally.
    assert_eq!(
        children,
        Some(Children {
            done: 0,
            failed: 0,
            cancelled: 1,
            failed_ids: vec![],
        }),
        "the interrupted parent carries its children's tally as a field"
    );
    let Outcome::Success { payload } = held(&daemon, 1).unwrap().1.unwrap() else {
        panic!("parent did not finish with its held result")
    };
    assert_eq!(payload["tabs_listed"], json!(1));
    assert_eq!(payload["children"]["done"], json!(1));
    assert!(
        JobDb::open(&db).unwrap().get(3).unwrap().is_none(),
        "the probe row is gone from the table too"
    );
    assert_eq!(daemon.shared.lock().unwrap().next_id, 8);
    finish_harness(None, &log);
}

#[tokio::test]
async fn a_failed_queue_write_refuses_new_jobs_and_stops_dispatch() {
    let db = test_db_path();
    let (daemon, log) = persisting_daemon("http://127.0.0.1:1", &db);
    daemon
        .submit("sleep".into(), json!({}), 0, "t".into(), None)
        .unwrap();
    break_queue(&db);
    let err = daemon
        .submit("sleep".into(), json!({}), 0, "t".into(), None)
        .unwrap_err();
    assert!(err.message.contains("refuses new jobs"), "{err}");
    assert_eq!(err.kind, ErrorKind::QueueFailed);
    assert_eq!(
        daemon.shared.lock().unwrap().next_id,
        2,
        "the refused submit rolled its id back"
    );
    assert!(
        daemon.pick_runnable().is_empty(),
        "a failed queue dispatches nothing"
    );
    assert!(
        !Daemon::has_live_jobs(&daemon.shared.lock().unwrap(), true),
        "the parked queue does not hold the daemon up"
    );
    finish_harness(None, &log);
}

/// C6 as amended: a concluded tree leaves memory only when every row is
/// persisted. A job that concludes after a failed queue write stays in
/// memory, which answers for it: its row was not written.
#[tokio::test]
async fn a_job_concluded_after_a_failed_queue_write_stays_in_memory() {
    let db = test_db_path();
    let (daemon, log) = persisting_daemon("http://127.0.0.1:1", &db);
    let id = daemon
        .submit("sleep".into(), json!({}), 0, "t".into(), None)
        .unwrap();
    break_queue(&db);
    daemon.start_and_finish(id, Outcome::Success { payload: json!({}) });
    assert!(daemon.queue_failed().is_some());
    let s = daemon.shared.lock().unwrap();
    assert_eq!(
        s.jobs.get(&id).map(|e| e.info.state),
        Some(JobState::Done),
        "memory still holds the job, concluded"
    );
    drop(s);
    finish_harness(None, &log);
}

/// A row as a previous daemon (or a neighbour connection) wrote it.
fn disk_row(id: u64, kind: &str, state: &str, parent: Option<u64>) -> JobRow {
    JobRow {
        id,
        kind: kind.into(),
        state: state.into(),
        priority: 0,
        submitted_by: "t".into(),
        parent,
        retries: 0,
        account: None,
        params: json!({}),
        outcome: matches!(state, "done" | "failed" | "cancelled")
            .then(|| json!({ "outcome": "success", "payload": {} })),
        deferred: None,
        cancel_requested: false,
        submitted_at: 0,
        updated_at: 0,
    }
}

/// C6: a job not in memory is read from the table and never acted on.
/// A row still waiting on disk that memory does not hold (a probe whose
/// delete failed at the restore) is no job to `cancel` and
/// `set-priority`, as it was before the table answered; a finished row
/// is the `wrong_state` refusal. Nothing panics, and the open row is
/// left as it was.
#[tokio::test]
async fn a_waiting_row_on_disk_and_not_in_memory_is_never_acted_on() {
    let db = test_db_path();
    let (daemon, log) = persisting_daemon("http://127.0.0.1:1", &db);
    {
        let other = JobDb::open(&db).unwrap();
        other
            .upsert(&disk_row(40, "probe", "waiting", None))
            .unwrap();
        other.upsert(&disk_row(41, "sleep", "done", None)).unwrap();
    }
    for refusal in [daemon.cancel(40), daemon.set_priority(40, 3)] {
        let refusal = refusal.unwrap_err();
        assert_eq!(
            (refusal.kind, refusal.message.as_str()),
            (ErrorKind::UnknownJob, "no job 40")
        );
    }
    assert_eq!(
        JobDb::open(&db).unwrap().get(40).unwrap().unwrap().state,
        "waiting",
        "the open row is left as it was"
    );
    let cancel = daemon.cancel(41).unwrap_err();
    assert_eq!(
        (cancel.kind, cancel.message.as_str()),
        (ErrorKind::WrongState, "job 41 already done")
    );
    let priority = daemon.set_priority(41, 3).unwrap_err();
    assert_eq!(
        (priority.kind, priority.message.as_str()),
        (ErrorKind::WrongState, "job 41 is done, not waiting")
    );
    finish_harness(None, &log);
}

/// C6, uniform eviction: a tree the restore brought back leaves memory
/// when it concludes, as any tree does. An apply held for one waiting
/// child is restored with its done child; the child concludes, the
/// parent finishes, and memory holds nothing. The list reads the table
/// from the smallest id the restore brought back: the tree, and the
/// earlier lifetime's finished row after it, which the restore
/// inherited; the finished row before it is not listed.
#[tokio::test]
async fn a_restored_tree_that_concludes_leaves_memory() {
    let db = test_db_path();
    // The finished rows outside the tree are recent: the restore's prune
    // keeps them, so only the list's bound leaves the earlier one out.
    let recent = |id| JobRow {
        updated_at: MANUAL_WALL_ORIGIN.as_secs() as i64,
        ..disk_row(id, "sleep", "done", None)
    };
    {
        let other = JobDb::open(&db).unwrap();
        other.upsert(&recent(9)).unwrap();
        let mut parent = disk_row(11, "apply", "running", None);
        parent.deferred = Some(json!({ "outcome": "success", "payload": {} }));
        other.upsert(&parent).unwrap();
        other
            .upsert(&disk_row(12, "sleep", "waiting", Some(11)))
            .unwrap();
        other
            .upsert(&disk_row(13, "sleep", "done", Some(11)))
            .unwrap();
        other.upsert(&recent(14)).unwrap();
    }
    let (daemon, log) = persisting_daemon("http://127.0.0.1:1", &db);
    let mut held_ids: Vec<JobId> = daemon.shared.lock().unwrap().jobs.keys().copied().collect();
    held_ids.sort_unstable();
    assert_eq!(
        held_ids,
        [11, 12, 13],
        "the restore brought back the open tree"
    );

    daemon.start_and_finish(12, Outcome::Success { payload: json!({}) });
    assert!(
        daemon.shared.lock().unwrap().jobs.is_empty(),
        "the restored tree left memory when it concluded"
    );
    let (info, outcome) = held(&daemon, 11).unwrap();
    assert_eq!(info.state, JobState::Done);
    let Some(Outcome::Success { payload }) = outcome else {
        panic!("the restored parent did not succeed: {outcome:?}")
    };
    assert_eq!(payload["children"]["done"], json!(2), "{payload}");
    let listed: Vec<JobId> = daemon.job_list().unwrap().iter().map(|j| j.id).collect();
    assert_eq!(
        listed,
        [11, 12, 13, 14],
        "from the smallest id brought back"
    );
    assert!(
        JobDb::open(&db).unwrap().get(9).unwrap().is_some(),
        "the row before the bound is still on disk, for `result`"
    );
    finish_harness(None, &log);
}

#[tokio::test]
async fn a_broken_queue_is_fatal_to_restore() {
    let db = test_db_path();
    let (daemon, log) = persisting_daemon("http://127.0.0.1:1", &db);
    break_queue(&db);
    let err = daemon.restore_jobs(Retention::default()).unwrap_err();
    assert!(err.to_string().contains("could not read"), "{err:#}");
    finish_harness(None, &log);
}

#[tokio::test]
async fn a_transition_that_cannot_persist_does_not_run() {
    // waiting → running is the transition that gates a send: if its
    // write fails, disk still says waiting, and after a restart a
    // no-probe job would replay blind. The job must not run.
    let db = test_db_path();
    let (daemon, log) = persisting_daemon("http://127.0.0.1:1", &db);
    let id = daemon
        .submit(
            "sleep".into(),
            json!({ "seconds": 0.01 }),
            0,
            "t".into(),
            None,
        )
        .unwrap();
    break_queue(&db);
    let dispatcher = tokio::spawn(daemon.clone().dispatcher());
    tokio::time::sleep(Duration::from_millis(100)).await;
    let state = daemon.shared.lock().unwrap().jobs[&id].info.state;
    assert_eq!(
        state,
        JobState::Waiting,
        "the job reverted instead of running"
    );
    assert!(
        daemon.queue_failed().is_some(),
        "the failed write tripped the queue"
    );
    finish_harness(dispatcher, &log);
}

#[tokio::test]
async fn cancel_and_reprioritize_report_a_failed_queue_write() {
    let db = test_db_path();
    let (daemon, log) = persisting_daemon("http://127.0.0.1:1", &db);
    let id = daemon
        .submit("sleep".into(), json!({}), 3, "t".into(), None)
        .unwrap();
    break_queue(&db);
    let err = daemon.set_priority(id, 9).unwrap_err();
    assert!(err.message.contains("priority unchanged"), "{err}");
    assert_eq!(err.kind, ErrorKind::QueueFailed);
    assert_eq!(daemon.shared.lock().unwrap().jobs[&id].info.priority, 3);
    let err = daemon.cancel(id).unwrap_err();
    assert!(err.message.contains("may not survive"), "{err}");
    assert_eq!(err.kind, ErrorKind::QueueFailed);
    assert_eq!(
        daemon.shared.lock().unwrap().jobs[&id].info.state,
        JobState::Cancelled,
        "the cancellation still holds for this lifetime"
    );
    finish_harness(None, &log);
}

#[tokio::test]
async fn a_result_read_failure_is_reported_not_no_job() {
    let db = test_db_path();
    let (daemon, log) = persisting_daemon("http://127.0.0.1:1", &db);
    break_queue(&db);
    match daemon
        .handle_request(Request::Result { id: 42 }, &mut None)
        .await
    {
        Response::Error { message, .. } => assert!(
            message.contains("could not read the persisted queue"),
            "{message}"
        ),
        other => panic!("{other:?}"),
    }
    finish_harness(None, &log);
}
