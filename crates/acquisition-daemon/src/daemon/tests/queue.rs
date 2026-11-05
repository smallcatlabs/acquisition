//! The queue: what a client may submit, a restart, a restore, and a
//! queue write that fails. Through the door (C158): the admission table
//! and its positive twins, the restored rows that never passed the
//! client's door, the queue file failing, the cancel and the priority
//! after a failed write, and the restores; a scenario's disk read is an
//! observer's once its door's run has returned (C45). The component
//! tests each say in their doc which boundary they need.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::door::*;
use super::harness::*;
use crate::daemon::queue::{
    APPLY_MAX_ACTIONS, Door as KindDoor, KINDS, PARAMS_MAX_BYTES, SEGMENT_MAX_BYTES,
    SUBMITTER_MAX_BYTES, door as kind_door,
};
use crate::daemon::*;
use acquisition_store::jobs::daemon_db_path;

// ---- admission (C127) ------------------------------------------------

/// The response is a refusal of `kind` whose words carry every one of
/// `carries` and none of `lacks`, quoted within a bounded preview (C127,
/// a refused value is described, never echoed whole); its message, for a
/// row that compares it.
fn assert_refused(
    response: Response,
    what: &str,
    kind: ErrorKind,
    carries: &[String],
    lacks: &[String],
) -> String {
    let Response::Error {
        kind: answered,
        message,
    } = response
    else {
        panic!("{what} was admitted: {response:?}")
    };
    assert_eq!(answered, kind, "{what}: {message}");
    assert!(
        message.len() <= 1024,
        "{what}: the refusal quotes a bounded preview, not {} bytes",
        message.len()
    );
    for words in carries {
        assert!(
            message.contains(words.as_str()),
            "{what}: the refusal says {words:?}: {message}"
        );
    }
    for words in lacks {
        assert!(
            !message.contains(words.as_str()),
            "{what}: the refusal says {words:?}: {message}"
        );
    }
    message
}

/// `Request::Submit` as a frontend sends it.
fn submit_of(
    kind: &str,
    params: Value,
    priority: Priority,
    submitted_by: &str,
    account: Option<&str>,
) -> Request {
    Request::Submit {
        kind: kind.into(),
        params,
        priority,
        submitted_by: submitted_by.into(),
        account: account.map(str::to_string),
    }
}

/// The id a submit was answered with; anything else fails the scenario.
fn submitted(answer: Response, what: &str) -> JobId {
    match answer {
        Response::Submitted { id } => id,
        other => panic!("{what} answered {other:?}"),
    }
}

/// Every job `List` answers, page by page under `all`, by id.
async fn listed(door: &mut Door) -> Vec<JobInfo> {
    door.every_job().await
}

/// The account server's hold that no request matches: it records every
/// well-formed send and holds none, so its record is the wire and an
/// empty record is nothing sent.
const NO_HOLD: (&str, &str, usize) = ("GET", "/never", 0);

/// The bait's rule (the account server, [`NO_HOLD`]): a recording fixture
/// records a request before it interprets it, so one the daemon should
/// never send is on its record rather than lost in a panicked task. Token
/// requests whose grant it cannot read (an empty body; a refresh grant
/// with no token) are each answered and recorded, the account unread. A
/// component test of the account transport alone, called directly: no
/// daemon sends such a request.
#[tokio::test]
async fn the_account_bait_records_a_request_it_cannot_read() {
    use acquisition_transport::{Prepared, Transport};
    let (bait, record) = account_transport(NO_HOLD);
    let sent_log = record.sent;
    for form in [
        None,
        Some(vec![("grant_type".into(), "refresh_token".into())]),
    ] {
        let mut post = Prepared::new(http::Method::POST, &format!("{BASE}/token"), "the-bait");
        post.form = form;
        let answer = tokio::time::timeout(HANG, bait.send(post))
            .await
            .expect("the bait answered")
            .expect("the bait's answer arrived");
        assert!(answer.status.is_success(), "{}", answer.status);
    }
    assert_eq!(
        sent_log.lock().unwrap().as_slice(),
        [sent(UNREAD_ACCOUNT, "POST"), sent(UNREAD_ACCOUNT, "POST")],
        "each request the bait could not read is on its record"
    );
}

/// The provider an admission row's daemon runs as.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Mode {
    /// The mock's name.
    Mock,
    /// The real provider's name on the fixture's base
    /// ([`DoorBuilder::real_named`]).
    RealNamed,
}

/// The sessions a row's daemon starts with.
#[derive(Clone, Copy, Debug)]
enum Session {
    /// One empty session, the door's default.
    None,
    /// `Alice#1234`, its access token fresh ([`live_sessions`]): a job
    /// admitted for it sends at once.
    Live,
    /// `test-user`, its refresh due ([`logged_in_sessions`]): a job
    /// admitted for it refreshes first.
    RefreshDue,
}

impl Session {
    fn on(self, clock: &ParkingClock) -> Sessions {
        match self {
            Session::None => Sessions::with(AuthSession::default()),
            Session::Live => live_sessions(clock),
            Session::RefreshDue => logged_in_sessions(),
        }
    }
}

/// C127, a row of the admission table: a provider mode, the sessions, a
/// client's submit (kind, params, priority, submitter, account) as
/// `Request::Submit` carries it, and the refusal it is answered with:
/// its kind, words it carries and words it must not. Named for the test
/// it replaced and its case.
struct Admission {
    name: String,
    mode: Mode,
    session: Session,
    submit: Request,
    refused: ErrorKind,
    carries: Vec<String>,
    lacks: Vec<String>,
}

impl Admission {
    /// A row refused with the refused-work kind, on the mock with one
    /// empty session, carrying `carries`.
    fn new(name: String, submit: Request, carries: &[&str]) -> Admission {
        Admission {
            name,
            mode: Mode::Mock,
            session: Session::None,
            submit,
            refused: ErrorKind::Refused,
            carries: carries.iter().map(|words| words.to_string()).collect(),
            lacks: Vec::new(),
        }
    }

    fn mode(self, mode: Mode) -> Admission {
        Admission { mode, ..self }
    }

    fn session(self, session: Session) -> Admission {
        Admission { session, ..self }
    }

    fn answered(self, refused: ErrorKind) -> Admission {
        Admission { refused, ..self }
    }

    fn lacking(self, lacks: &[&str]) -> Admission {
        Admission {
            lacks: lacks.iter().map(|words| words.to_string()).collect(),
            ..self
        }
    }
}

/// The admission table's rows, each written from the submits and the
/// words of the test it replaced.
fn admission_rows() -> Vec<Admission> {
    let mut rows = Vec::new();
    let client = |kind: &str, params: Value, account: Option<&str>| {
        submit_of(kind, params, 0, "client", account)
    };

    // C127 (a client never submits a daemon-only kind): a client's
    // `probe`, naming a route and a URL of its own.
    rows.push(Admission::new(
        "c127_a_client_probe_is_refused_and_sends_nothing".into(),
        client(
            "probe",
            json!({ "route": "fetch", "url": "http://127.0.0.1:1/fetch" }),
            None,
        ),
        &["probe", "daemon itself", "a client cannot submit"],
    ));

    // C127 (a path param is one segment), through `Submit` and through an
    // `apply` tuple, for a live session whose admitted job would send at
    // once. The first case is the shape that steered a character fetch
    // onto `/account/leagues` with the bearer: `..` to climb, `?` to cut
    // the rest of the path off as a query.
    let name = "c127_a_path_param_that_is_not_one_segment_is_refused_and_sends_nothing";
    for (kind, params, param) in [
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
    ] {
        let account = Some("Alice#1234");
        rows.push(
            Admission::new(
                format!("{name}: {kind} {params}"),
                client(kind, params.clone(), account),
                &[param, "one path segment"],
            )
            .session(Session::Live),
        );
        // The same tuple as a plan's action, where the vocabulary takes it.
        if matches!(kind, "character" | "stash") && params.get("deep").is_none() {
            let apply = json!({ "jobs": [{ "kind": kind, "params": params }] });
            rows.push(
                Admission::new(
                    format!("{name}: apply of {kind} {params}"),
                    client("apply", apply, account),
                    &["action 0", param, "one path segment"],
                )
                .session(Session::Live),
            );
        }
    }

    // C127 (a path param is one segment, as a server may read it): a
    // server that strips `;` path parameters (Tomcat, Jetty) reads `..;x`
    // as `..`, and one that trims trailing dots and spaces (IIS) reads
    // `.. .` as `..`: a segment whose head, the part before any `;`, is
    // only dots and spaces.
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
        rows.push(Admission::new(
            format!(
                "c127_a_dot_segment_behind_a_semicolon_or_a_space_is_refused: stashes {params}"
            ),
            client("stashes", params, None),
            &["`league`", "one path segment"],
        ));
    }

    // C127 (a path param is bounded): longer than the bound in UTF-8
    // bytes, multi-byte characters counted as bytes, since each goes out
    // as its percent-encoded bytes; described by its head, type and size,
    // never echoed whole.
    let bound = format!("at most {SEGMENT_MAX_BYTES} bytes");
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
        rows.push(Admission::new(
            format!(
                "c127_a_path_param_longer_than_the_bound_is_refused: {kind} of {} bytes",
                params.to_string().len()
            ),
            client(kind, params, None),
            &[
                "one path segment",
                &bound,
                "… (a string of",
                "bytes) is not one path segment",
            ],
        ));
    }

    // C127 (a required segment is judged at the door): a `stash` without
    // an id, with or without a `sub`, and a `character` without a name,
    // as an `apply` tuple already was.
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
        rows.push(Admission::new(
            format!("c127_a_stash_without_an_id_is_refused_at_submit: {kind} {params}"),
            client(kind, params, None),
            &[said],
        ));
    }

    // C127 and the 4b sitting round: the ad-hoc `refresh` kind is retired
    // (a selection is a plan, applied as `apply`), refused as any kind the
    // table does not list, for a session whose refresh is due.
    for params in [
        json!({ "league": "Standard", "all": true }),
        json!({ "league": "Standard", "tabs": ["t1"] }),
    ] {
        rows.push(
            Admission::new(
                format!("c127_the_retired_refresh_kind_is_refused_at_submit: refresh {params}"),
                client("refresh", params, Some("test-user")),
                &["unknown job kind"],
            )
            .session(Session::RefreshDue),
        );
    }

    // C127 (a mock-only kind is the mock's): in real mode a client's
    // `sleep`, `fetch` and `whoami`. Real in name only: every URL is the
    // fixture's.
    for kind in ["sleep", "fetch", "whoami"] {
        rows.push(
            Admission::new(
                format!("c127_a_mock_only_kind_is_refused_in_real_mode: {kind}"),
                client(kind, json!({ "seconds": 3600 }), None),
                &[kind, "mock-only", "real mode"],
            )
            .mode(Mode::RealNamed),
        );
    }

    // C127 (a sleep is bounded): seconds that are negative, beyond the
    // bound or not a number.
    for seconds in [
        json!(-1),
        json!(1e300),
        json!(3601),
        json!("5"),
        json!(true),
    ] {
        rows.push(Admission::new(
            format!("c127_a_sleep_with_bad_seconds_is_refused_at_submit: sleep {seconds}"),
            client("sleep", json!({ "seconds": seconds }), None),
            &["sleep", "`seconds`", "from 0 to 3600"],
        ));
    }

    // C127 (the kind table): a kind the daemon does not run names only
    // the kinds a client may submit in this provider mode: never the
    // daemon's own `probe`, and in real mode none of the mock-only kinds.
    let name = "c127_an_unknown_kind_is_refused_naming_only_client_kinds";
    rows.push(
        Admission::new(
            format!("{name}: bogus in mock mode"),
            client("bogus", json!({}), None),
            &[
                "unknown job kind \"bogus\"",
                "profile",
                "stash",
                "apply",
                "sleep",
                "fetch",
                "whoami",
            ],
        )
        .lacking(&["probe"]),
    );
    rows.push(
        Admission::new(
            format!("{name}: bogus in real mode"),
            client("bogus", json!({}), None),
            &["unknown job kind \"bogus\"", "profile", "stash", "apply"],
        )
        .mode(Mode::RealNamed)
        .lacking(&["probe", "sleep", "fetch", "whoami"]),
    );

    // C127 (a refused value is described, never echoed whole): each value
    // here fits a request frame, and each is 20 MiB of backslashes, which
    // every escape doubles: quoted whole, its refusal would exceed the
    // response frame and be lost.
    let name = "c127_a_refused_value_is_described_by_its_type_and_size_within_the_response_bound";
    let huge = "\\".repeat(20 << 20);
    let huge_len = huge.len().to_string();
    for (what, submit, said) in [
        (
            "a league array",
            client("stashes", json!({ "league": [huge] }), None),
            vec![
                "`league` must be a string",
                "(an array of",
                "bytes as JSON)",
            ],
        ),
        (
            "a sleep's array seconds",
            client("sleep", json!({ "seconds": [huge] }), None),
            vec![
                "`seconds` must be a number",
                "(an array of",
                "bytes as JSON)",
            ],
        ),
        (
            "a substash id that is no segment",
            client("stash", json!({ "id": "t1", "sub": huge }), None),
            vec!["not one path segment", "(a string of", &huge_len],
        ),
        (
            "an unknown kind",
            client(&huge, json!({}), None),
            vec!["unknown job kind", "(a string of", &huge_len],
        ),
        (
            "an apply's budget",
            client(
                "apply",
                json!({ "jobs": [{ "kind": "stashes", "params": {} }], "max_requests": [huge] }),
                None,
            ),
            vec!["max_requests must be", "(an array of", "bytes as JSON)"],
        ),
        (
            "an apply's kind",
            client(
                "apply",
                json!({ "jobs": [{ "kind": huge, "params": {} }] }),
                None,
            ),
            vec!["not in the plan vocabulary", "(a string of", &huge_len],
        ),
        (
            "a realm",
            client("stashes", json!({ "realm": huge }), None),
            vec!["realm must be a string", "(a string of", &huge_len],
        ),
    ] {
        rows.push(Admission::new(format!("{name}: {what}"), submit, &said));
    }
    rows.push(
        Admission::new(
            format!("{name}: an account selector"),
            client("profile", json!({}), Some(&huge)),
            &["account", "not logged in", "(a string of", &huge_len],
        )
        .answered(ErrorKind::NotLoggedIn),
    );

    // C127 (a submit's params have a bound): params one byte over
    // `PARAMS_MAX_BYTES` as JSON, named by their size and the bound and not
    // their value; a submitter over `SUBMITTER_MAX_BYTES`, described.
    let name = "c127_params_over_their_bound_are_refused_and_a_large_plan_is_admitted";
    let over = pad(PARAMS_MAX_BYTES + 1);
    assert_eq!(
        serde_json::to_vec(&over).unwrap().len(),
        PARAMS_MAX_BYTES + 1
    );
    rows.push(
        Admission::new(
            format!("{name}: params over the bound"),
            client("sleep", over, None),
            &[
                &(PARAMS_MAX_BYTES + 1).to_string(),
                &PARAMS_MAX_BYTES.to_string(),
            ],
        )
        .session(Session::Live)
        .lacking(&["xxxx"]),
    );
    rows.push(
        Admission::new(
            format!("{name}: a submitter over its bound"),
            submit_of("sleep", json!({}), 0, &"y".repeat(20 << 20), None),
            &[
                "submitted_by must be at most",
                &SUBMITTER_MAX_BYTES.to_string(),
                "(a string of 20971520 bytes)",
            ],
        )
        .session(Session::Live),
    );

    // C127 (an apply's actions have a bound): one action over
    // `APPLY_MAX_ACTIONS`, though its tiny tuples sit far under
    // `PARAMS_MAX_BYTES`; the bound wins over a `max_requests` the plan
    // also exceeds.
    let over = APPLY_MAX_ACTIONS + 1;
    let actions = vec![json!({ "kind": "stashes", "params": {} }); over];
    let carries = [
        format!("an apply of {over} actions"),
        format!("bound of {APPLY_MAX_ACTIONS} actions"),
        "nothing was submitted".to_string(),
        "covers fewer tabs or characters".to_string(),
    ];
    let carries: Vec<&str> = carries.iter().map(String::as_str).collect();
    for (what, plan) in [
        ("an apply over its action bound", json!({ "jobs": actions })),
        (
            "an apply over its action bound and its budget",
            json!({ "jobs": actions, "max_requests": 1 }),
        ),
    ] {
        assert!(serde_json::to_vec(&plan).unwrap().len() < PARAMS_MAX_BYTES);
        rows.push(
            Admission::new(
                format!("c127_an_apply_over_its_action_bound_is_refused_and_one_at_it_is_admitted: {what}"),
                client("apply", plan, None),
                &carries,
            )
            .session(Session::Live),
        );
    }

    // C43 (an apply's admission is vocabulary, not meaning, at submit
    // before a job id exists) and C42 (a logical bound over
    // `max_requests` refuses whole): an apply is refused whole, for a
    // live session whose admitted children would send at once; nothing is
    // admitted and no id is spent (the table's `List` and id-1 reads). The
    // whole vocabulary (characters joined 2026-09-02): a character list
    // under poe2 is a legal tuple, the character family takes it.
    let name = "apply_admission_refuses_bad_vocabulary_and_blown_budgets_whole";
    let ok_jobs = json!([
        { "kind": "stashes", "params": { "league": "Standard" } },
        { "kind": "stash", "params": { "league": "Standard", "id": "t1", "deep": false } },
        { "kind": "characters", "params": { "realm": "poe2" } },
        { "kind": "character", "params": { "realm": "pc", "name": "Exile" } },
    ]);
    for (params, said) in [
        // The tuple list itself is required and non-empty.
        (json!({}), "needs a `jobs` array"),
        (json!({ "jobs": [] }), "empty"),
        // Only single-request kinds: a nested parent (apply), the retired
        // ad-hoc refresh, or any other kind would expand or sidestep the
        // reviewed set.
        (
            json!({ "jobs": [{ "kind": "refresh", "params": { "all": true } }] }),
            "not in the plan vocabulary",
        ),
        (
            json!({ "jobs": [{ "kind": "apply", "params": { "jobs": [] } }] }),
            "not in the plan vocabulary",
        ),
        (
            json!({ "jobs": [{ "kind": "sleep", "params": {} }] }),
            "not in the plan vocabulary",
        ),
        // A stash fetch that fans out expands the set; one without an id
        // could not have come from a plan.
        (
            json!({ "jobs": [{ "kind": "stash", "params": { "league": "Standard", "id": "m1", "deep": true } }] }),
            "never fans out",
        ),
        (
            json!({ "jobs": [{ "kind": "stash", "params": { "league": "Standard" } }] }),
            "needs an id",
        ),
        // A character fetch without a name could not have come from a
        // plan either.
        (
            json!({ "jobs": [{ "kind": "character", "params": { "realm": "pc" } }] }),
            "needs a name",
        ),
        // The budget: a bound under the tuple count refuses; a misread
        // bound refuses too, never "the limit was ignored".
        (
            json!({ "jobs": ok_jobs.clone(), "max_requests": 1 }),
            "exceeds the budget",
        ),
        (
            json!({ "jobs": ok_jobs.clone(), "max_requests": "ten" }),
            "non-negative integer",
        ),
    ] {
        rows.push(
            Admission::new(
                format!("{name}: apply {params}"),
                client("apply", params, None),
                &[said],
            )
            .session(Session::Live),
        );
    }

    // C136 (the submitter `daemon` is the daemon's own): a client's
    // profile fetch for a live session, which, admitted, would send at
    // once.
    rows.push(
        Admission::new(
            "c136_a_client_submit_as_the_daemon_is_refused_and_sends_nothing".into(),
            submit_of("profile", json!({}), 0, "daemon", None),
            &["reserved for the daemon's own work", "a client cannot"],
        )
        .session(Session::Live),
    );
    rows
}

/// Params of exactly `len` bytes as JSON: `{"pad":"…"}` is ten bytes
/// around the string.
fn pad(len: usize) -> Value {
    json!({ "pad": "x".repeat(len - 10) })
}

/// C127, admission through the door: each row's daemon faces the account
/// server holding nothing, so any send the daemon made is on its record,
/// and is sent the row's submit as a frontend's frame (the request
/// itself within a frame). The answer is the row's refusal, within the
/// response frame; then nothing is admitted (`List` answers no job) and,
/// once the runtime has settled, nothing is on the wire or in the
/// journal. On the mock a sleep is then admitted as job 1: the refusal
/// spent no id. A real-named daemon refuses a client's `sleep`, so its
/// rows read the list and the wire alone.
#[test]
fn c127_admission_through_the_door() {
    for row in admission_rows() {
        let Admission {
            name,
            mode,
            session,
            submit,
            refused,
            carries,
            lacks,
        } = row;
        let clock = Arc::new(ParkingClock::new());
        let sessions = session.on(&clock);
        let mut builder = Door::open(
            Server::Account(NO_HOLD),
            clock,
            Arc::new(RecordingCredentialStore::default()),
        )
        .sessions(sessions);
        if mode == Mode::RealNamed {
            builder = builder.real_named();
        }
        let name = &name;
        builder.run(|mut door| async move {
            let request_bytes = serde_json::to_vec(&submit).unwrap().len() + 1;
            assert!(
                request_bytes <= MAX_FRAME_BYTES,
                "{name}: the request fits its frame ({request_bytes} bytes)"
            );
            let answer = door.request(submit).await;
            let response_bytes = serde_json::to_vec(&answer).unwrap().len() + 1;
            assert!(
                response_bytes <= MAX_FRAME_BYTES,
                "{name}: the refusal fits the response frame ({response_bytes} bytes)"
            );
            assert_refused(answer, name, refused, &carries, &lacks);
            assert!(
                listed(&mut door).await.is_empty(),
                "{name}: a refused submit admits no job"
            );
            door.settle().await;
            assert!(door.sent().is_empty(), "{name}: nothing reached the wire");
            assert!(door.journal().is_empty(), "{name}: nothing was journaled");
            if mode == Mode::Mock {
                let sleep = submit_of("sleep", json!({ "seconds": 0 }), 0, "client", None);
                let answer = door.request(sleep).await;
                assert_eq!(
                    submitted(answer, "the sleep after the refusal"),
                    1,
                    "{name}: a refused submit spent a job id"
                );
                let (job, _) = door.await_terminal(1).await;
                assert_eq!(job.state, JobState::Done, "{name}");
            }
            door.finish().await;
        });
    }
}

/// C136, the admission table's row of this name beside it: the same
/// profile fetch a client may not submit as the daemon, submitted under a
/// frontend's name for a live session, is admitted as job 1 and sends its
/// one GET, which the journal records.
#[test]
fn c136_a_frontends_submit_of_the_same_fetch_is_admitted_and_sends_once() {
    let clock = Arc::new(ParkingClock::new());
    let sessions = live_sessions(&clock);
    Door::open(
        Server::Account(NO_HOLD),
        clock,
        Arc::new(RecordingCredentialStore::default()),
    )
    .sessions(sessions)
    .run(|mut door| async move {
        let answer = door
            .request(submit_of("profile", json!({}), 0, "cli:1", None))
            .await;
        assert_eq!(submitted(answer, "a frontend's profile fetch"), 1);
        let (job, outcome) = door.await_terminal(1).await;
        assert_eq!(job.state, JobState::Done, "{outcome:?}");
        let wire = wire_methods_of(&door.sent());
        assert_eq!(wire, ["GET"], "the admitted fetch sends once");
        assert_eq!(door.journal_methods(), wire);
        door.finish().await;
    });
}

/// C164 (#166; the answer-facts seat, MCP run 1 calls 57 to 60, a stash
/// job for a tab its listing never named sent and failed on the 404):
/// a client's stash job is judged against the account's stored listing
/// of its realm and league. Alice's store lists A, B and map tab M (its
/// substash S1 on record from M's fetch) in pc Standard, after a listing
/// that named G too. Through the door, Alice's session live: a tab the
/// listing does not name, one it names as gone, and a substash of M its
/// rows do not name are each refused in words that name the listing, its
/// age and the `stashes` job that fetches it again, the tab quoted
/// bounded; nothing is admitted and nothing reaches the wire. Then A,
/// M's S1, a substash of B (no substash of B on record), a tab of a
/// league and of a realm no listing of which is stored are admitted, and
/// the refused tabs are on no send the journal holds.
#[test]
fn c164_a_stash_job_for_a_tab_the_stored_listing_does_not_name_is_refused_and_sends_nothing() {
    let store_dir = TestPath::dir();
    let mut store = Store::open(&account_path(&store_dir, "Alice#1234")).unwrap();
    let listing = Endpoint::Stashes {
        realm: "pc".into(),
        league: "Standard".into(),
    };
    let tab = |id: &str, kind: &str, index: usize| json!({ "id": id, "name": id, "type": kind, "index": index });
    let at = acquisition_store::now() - 7200;
    store
        .record(
            &listing,
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": [ tab("A", "PremiumStash", 0), tab("B", "PremiumStash", 1),
                                  tab("M", "MapStash", 2), tab("G", "PremiumStash", 3) ] }),
            at,
        )
        .unwrap();
    store
        .record(
            &Endpoint::Stash {
                realm: "pc".into(),
                league: "Standard".into(),
                id: "M".into(),
                sub: None,
            },
            &json!({ "league": "Standard", "id": "M" }),
            200,
            &json!({ "stash": { "id": "M", "name": "M", "type": "MapStash", "items": [],
                     "children": [ { "id": "S1", "name": "", "type": "MapStash", "parent": "M",
                                     "metadata": { "items": 1 } } ] } }),
            at,
        )
        .unwrap();
    store
        .record(
            &listing,
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": [ tab("A", "PremiumStash", 0), tab("B", "PremiumStash", 1),
                                  tab("M", "MapStash", 2) ] }),
            at + 60,
        )
        .unwrap();
    // a league longer than a refusal's preview (C127; the review of round
    // D, finding 3): a valid segment, quoted bounded
    let long = format!("League {}", "w".repeat(193));
    store
        .record(
            &Endpoint::Stashes {
                realm: "pc".into(),
                league: long.clone(),
            },
            &json!({ "league": long }),
            200,
            &json!({ "stashes": [ tab("A", "PremiumStash", 0) ] }),
            at,
        )
        .unwrap();
    drop(store);
    let clock = Arc::new(ParkingClock::new());
    let sessions = live_sessions(&clock);
    Door::open(
        Server::Account(NO_HOLD),
        clock,
        Arc::new(RecordingCredentialStore::default()),
    )
    .sessions(sessions)
    .store_dir(store_dir.to_path_buf())
    .run(|mut door| async move {
        let stash = |params: Value| submit_of("stash", params, 0, "client", Some("Alice#1234"));
        let tab_again = r#"a stashes job for pc "Standard" fetches the listing again"#;
        let sub_again = r#"a stash job for tab "M" fetches its substashes again"#;
        for (params, quoted, again) in [
            (
                json!({ "league": "Standard", "id": "C" }),
                r#""C""#,
                tab_again,
            ),
            (
                json!({ "realm": "pc", "league": "Standard", "id": "G" }),
                r#""G""#,
                tab_again,
            ),
            (
                json!({ "league": "Standard", "id": "M", "sub": "S9" }),
                r#""S9""#,
                sub_again,
            ),
        ] {
            let what = format!("c164: stash {params}");
            assert_refused(
                door.request(stash(params)).await,
                &what,
                ErrorKind::Refused,
                &[
                    quoted.to_string(),
                    r#"pc "Standard""#.to_string(),
                    " ago".to_string(),
                    again.to_string(),
                ],
                &[],
            );
            assert!(listed(&mut door).await.is_empty(), "{what}: admitted");
        }
        let what = "c164: a tab of a league longer than the preview";
        assert_refused(
            door.request(stash(json!({ "league": long, "id": "C" })))
                .await,
            what,
            ErrorKind::Refused,
            &[long[..40].to_string(), r#""C""#.to_string()],
            std::slice::from_ref(&long),
        );
        assert!(listed(&mut door).await.is_empty(), "{what}: admitted");
        door.settle().await;
        assert!(
            door.sent().is_empty(),
            "a refused stash job reached the wire"
        );
        assert!(
            door.journal().is_empty(),
            "a refused stash job was journaled"
        );

        for params in [
            json!({ "league": "Standard", "id": "A" }),
            json!({ "league": "Standard", "id": "M", "sub": "S1" }),
            json!({ "league": "Standard", "id": "B", "sub": "X" }),
            json!({ "league": "Other", "id": "Z" }),
            json!({ "realm": "xbox", "league": "Standard", "id": "Z" }),
        ] {
            let what = format!("c164: stash {params}");
            let id = submitted(door.request(stash(params)).await, &what);
            door.await_terminal(id).await;
        }
        let paths: Vec<String> = door
            .journal()
            .iter()
            .filter_map(|send| send["path"].as_str().map(str::to_string))
            .collect();
        for refused in ["/C", "/G", "/S9"] {
            assert!(
                paths.iter().all(|path| !path.ends_with(refused)),
                "{refused} was sent: {paths:?}"
            );
        }
        assert!(
            paths.iter().any(|path| path.ends_with("/Standard/A")),
            "the admitted tab was sent: {paths:?}"
        );
        door.finish().await;
    });
}

/// C164, the review of round D, finding 1: a substash's refusal names the
/// fetch that makes the retry pass. Its parent's stored rows are what
/// left it out, and a `stashes` job keeps them; so the refusal names a
/// `stash` job for the parent, and the recovery runs as a client follows
/// it: M's rows name S1, S9 is asked for and refused, M is fetched (the
/// server lists S1 and S9 now), and S9 is then admitted and fetched.
#[test]
fn c164_a_refused_substash_names_its_parents_fetch_which_lets_it_through() {
    let store_dir = TestPath::dir();
    let mut store = Store::open(&account_path(&store_dir, "Alice#1234")).unwrap();
    let at = acquisition_store::now() - 7200;
    store
        .record(
            &Endpoint::Stashes {
                realm: "pc".into(),
                league: "Standard".into(),
            },
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": [ { "id": "M", "name": "M", "type": "MapStash", "index": 0 } ] }),
            at,
        )
        .unwrap();
    let parent = |subs: &[&str]| {
        json!({ "stash": { "id": "M", "name": "M", "type": "MapStash", "items": [],
                 "children": subs.iter().map(|id| json!({ "id": id, "name": "", "type": "MapStash",
                     "parent": "M", "metadata": { "items": 1 } })).collect::<Vec<_>>() } })
    };
    store
        .record(
            &Endpoint::Stash {
                realm: "pc".into(),
                league: "Standard".into(),
                id: "M".into(),
                sub: None,
            },
            &json!({ "league": "Standard", "id": "M" }),
            200,
            &parent(&["S1"]),
            at,
        )
        .unwrap();
    drop(store);
    let policy = "X-Rate-Limit-Policy: stash-request-limit\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 100:1:60\r\nX-Rate-Limit-Account-State: 0:1:0\r\n";
    let script = vec![
        ScriptedResponse {
            method: "HEAD",
            path: "/stash/Standard/M",
            status: "204 No Content",
            headers: policy.into(),
            body: String::new(),
        },
        ScriptedResponse {
            method: "GET",
            path: "/stash/Standard/M",
            status: "200 OK",
            headers: policy.replace("0:1:0", "1:1:0"),
            body: parent(&["S1", "S9"]).to_string(),
        },
        ScriptedResponse {
            method: "GET",
            path: "/stash/Standard/M/S9",
            status: "200 OK",
            headers: policy.replace("0:1:0", "2:1:0"),
            body: json!({ "stash": { "id": "S9", "name": "", "type": "MapStash", "parent": "M", "items": [] } })
                .to_string(),
        },
    ];
    let clock = Arc::new(ParkingClock::new());
    let sessions = live_sessions(&clock);
    Door::open(
        Server::Script(script),
        clock,
        Arc::new(RecordingCredentialStore::default()),
    )
    .sessions(sessions)
    .store_dir(store_dir.to_path_buf())
    .run(|mut door| async move {
        let stash = |params: Value| submit_of("stash", params, 0, "client", Some("Alice#1234"));
        let s9 = json!({ "league": "Standard", "id": "M", "sub": "S9" });
        let refusal = assert_refused(
            door.request(stash(s9.clone())).await,
            "c164: S9 before M is fetched again",
            ErrorKind::Refused,
            &[
                r#"no substash "S9""#.to_string(),
                r#"a stash job for tab "M" fetches its substashes again"#.to_string(),
            ],
            &["a stashes job".to_string()],
        );
        assert!(listed(&mut door).await.is_empty(), "{refusal}");
        let id = submitted(
            door.request(stash(json!({ "league": "Standard", "id": "M" })))
                .await,
            "the parent's fetch the refusal names",
        );
        let (job, outcome) = door.await_terminal(id).await;
        assert_eq!(job.state, JobState::Done, "{outcome:?}");
        let id = submitted(door.request(stash(s9)).await, "S9 after M's fetch");
        let (job, outcome) = door.await_terminal(id).await;
        assert_eq!(job.state, JobState::Done, "{outcome:?}");
        door.finish().await;
    });
}

/// C164, the review of round D, finding 2: a store whose presence cannot
/// be told is refused as an unreadable one is, never taken for absent.
/// Alice's store, its listing naming A alone, sits in a store directory
/// that has lost its search permission, so asking whether the file is
/// there fails; a stash job for a tab that listing does not name is
/// refused, nothing admitted and nothing sent. The permission is put
/// back when the test ends, failure included, so the directory can go.
#[cfg(unix)]
#[test]
fn c164_a_store_whose_presence_cannot_be_told_is_refused_and_sends_nothing() {
    use std::os::unix::fs::PermissionsExt as _;
    /// Puts the directory's permission back on drop.
    struct Searchable<'a>(&'a std::path::Path);
    impl Drop for Searchable<'_> {
        fn drop(&mut self) {
            let _ = std::fs::set_permissions(self.0, std::fs::Permissions::from_mode(0o700));
        }
    }
    let store_dir = TestPath::dir();
    let file = account_path(&store_dir, "Alice#1234");
    let mut store = Store::open(&file).unwrap();
    store
        .record(
            &Endpoint::Stashes {
                realm: "pc".into(),
                league: "Standard".into(),
            },
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": [ { "id": "A", "name": "A", "type": "PremiumStash", "index": 0 } ] }),
            acquisition_store::now() - 7200,
        )
        .unwrap();
    drop(store);
    std::fs::set_permissions(&*store_dir, std::fs::Permissions::from_mode(0o600)).unwrap();
    let _searchable = Searchable(&store_dir);
    assert!(
        file.try_exists().is_err(),
        "the seam holds: whether the store is there cannot be told"
    );
    Door::open(
        Server::Account(NO_HOLD),
        Arc::new(ParkingClock::new()),
        Arc::new(RecordingCredentialStore::default()),
    )
    .sessions(live_sessions(&ParkingClock::new()))
    .store_dir(store_dir.to_path_buf())
    .run(|mut door| async move {
        let submit = submit_of(
            "stash",
            json!({ "league": "Standard", "id": "C" }),
            0,
            "client",
            Some("Alice#1234"),
        );
        let what = "c164: a stash job beside a store that cannot be told present";
        assert_refused(
            door.request(submit).await,
            what,
            ErrorKind::Refused,
            &["could not be read".to_string()],
            &[],
        );
        // the league as the refusal quotes it: bounded (the review of
        // round D, finding 3)
        let long = format!("League {}", "w".repeat(193));
        let submit = submit_of(
            "stash",
            json!({ "league": long, "id": "C" }),
            0,
            "client",
            Some("Alice#1234"),
        );
        assert_refused(
            door.request(submit).await,
            what,
            ErrorKind::Refused,
            &[long[..40].to_string(), "could not be read".to_string()],
            std::slice::from_ref(&long),
        );
        // a realm outside the closed set is judged before any refusal
        // quotes it: refused as an unknown realm, never echoed whole (C127;
        // the review of round D's fix round)
        let realm = "r".repeat(200);
        let submit = submit_of(
            "stash",
            json!({ "realm": realm, "league": "Standard", "id": "C" }),
            0,
            "client",
            Some("Alice#1234"),
        );
        assert_refused(
            door.request(submit).await,
            what,
            ErrorKind::Refused,
            &["realm must be a string naming one of".to_string()],
            &[realm.clone(), "could not be read".to_string()],
        );
        assert!(listed(&mut door).await.is_empty(), "{what}: admitted");
        door.settle().await;
        assert!(door.sent().is_empty(), "{what}: sent");
        assert!(door.journal().is_empty(), "{what}: journaled");
        door.finish().await;
    });
}

/// C127 (a sleep is bounded), the admission table's rows of
/// `c127_a_sleep_with_bad_seconds_is_refused_at_submit` beside it: a
/// sleep within the bound, zero seconds, is admitted as job 1 and runs to
/// its end.
#[test]
fn c127_a_sleep_within_the_bound_is_admitted_and_done() {
    Door::open(
        Server::None,
        Arc::new(ParkingClock::new()),
        Arc::new(RecordingCredentialStore::default()),
    )
    .run(|mut door| async move {
        let sleep = submit_of("sleep", json!({ "seconds": 0 }), 0, "client", None);
        assert_eq!(
            submitted(door.request(sleep).await, "a zero-second sleep"),
            1
        );
        let (job, outcome) = door.await_terminal(1).await;
        assert_eq!(job.state, JobState::Done, "{outcome:?}");
        door.finish().await;
    });
}

/// C127, the admissions at the bounds whose refusals are the admission
/// table's rows: an `apply` the size of the largest plan observed, the
/// owner's Standard (3,347 tabs, 3,009 of them substashes, and 59
/// characters, N47), params at `PARAMS_MAX_BYTES`, and an `apply` of
/// exactly `APPLY_MAX_ACTIONS`, each admitted with the next id. A
/// component test, its boundary `handle_request` with no dispatcher: an
/// admitted job here must not run, and through the door it would or its
/// submit would never be answered (an `apply` needs an account, so its
/// submit waits on the start's restore before the account is resolved,
/// and a restore held unlanded holds it forever; landed, the `sleep`
/// runs on real time and an `apply`'s children reach the wire).
#[tokio::test]
async fn c127_a_large_plan_and_params_and_actions_at_their_bounds_are_admitted() {
    let (daemon, log_path) =
        test_daemon(Boundary::Function, refused(), Arc::new(ParkingClock::new()));
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
    let at_bound = vec![json!({ "kind": "stashes", "params": {} }); APPLY_MAX_ACTIONS];
    for (id, what, kind, params) in [
        (
            1,
            "the largest plan observed",
            "apply",
            json!({ "jobs": jobs, "max_requests": 3408 }),
        ),
        (2, "params at the bound", "sleep", pad(PARAMS_MAX_BYTES)),
        (
            3,
            "an apply at its action bound",
            "apply",
            json!({ "jobs": at_bound, "max_requests": APPLY_MAX_ACTIONS }),
        ),
    ] {
        let answer = daemon
            .handle_request(submit_of(kind, params, 0, "client", None), &mut None)
            .await;
        assert_eq!(
            submitted(answer, what),
            id,
            "{what} is admitted as job {id}"
        );
    }
    finish_harness(None, &log_path);
}
/// C127 (a path param is one segment, encoded): a league whose name holds
/// a space is a legitimate segment; it is admitted, and the probe and the
/// GET carry it percent-encoded as one segment, as the scripted server's
/// paths and the journal record.
#[test]
fn c127_a_league_with_a_space_is_admitted_and_sent_encoded() {
    let policy = "X-Rate-Limit-Policy: stash-list-test\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 100:1:60\r\nX-Rate-Limit-Account-State: 0:1:0\r\n";
    let script = vec![
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
    ];
    let clock = Arc::new(ParkingClock::new());
    let sessions = live_sessions(&clock);
    Door::open(
        Server::Script(script),
        clock,
        Arc::new(RecordingCredentialStore::default()),
    )
    .sessions(sessions)
    .run(|mut door| async move {
        let submit = submit_of(
            "stashes",
            json!({ "league": "Hard Mode" }),
            0,
            "client",
            Some("Alice#1234"),
        );
        let id = submitted(door.request(submit).await, "a league with a space");
        let (job, outcome) = door.await_terminal(id).await;
        assert_eq!(job.state, JobState::Done, "{outcome:?}");
        let paths: Vec<Value> = door
            .journal()
            .iter()
            .map(|send| send["path"].clone())
            .collect();
        assert_eq!(
            paths,
            [json!("/stash/Hard%20Mode"), json!("/stash/Hard%20Mode")]
        );
        door.finish().await;
    });
}
/// C127 (a path param is one segment, encoded): where a URL is rendered,
/// each path param is percent-encoded as one segment, so `%`, `#` and `?`
/// stay inside it; a value that is not one segment renders no URL at all,
/// so a row that never passed the door (a restored row, a child whose id
/// came from a response) sends nothing. That holds of a dot segment
/// behind a `;` or a space, and of a segment over the bound in UTF-8
/// bytes, while one at the bound renders (the admission table refuses
/// each at submit). A component test: `route_for` is a function.
#[test]
fn c127_a_rendered_path_param_is_one_encoded_segment_or_no_url() {
    let (daemon, _) = test_daemon_with(
        Boundary::Function,
        Provider::mock("http://mock"),
        refused(),
        Arc::new(ParkingClock::new()),
        RailsConfig::default(),
    );
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
        assert_eq!(daemon.route_for("stashes", &params), None, "{params}");
    }
    let at_bound = json!({ "league": "a".repeat(SEGMENT_MAX_BYTES) });
    assert!(
        daemon.route_for("stashes", &at_bound).is_some(),
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
        assert_eq!(daemon.route_for(kind, &params), None, "{what}");
    }
}

/// What a restored row's failure must say.
enum Said {
    /// It carries these words and none of the misleading causes given.
    Carries(&'static str, &'static [&'static str]),
    /// It is, word for word, the refusal a client's submit of the same
    /// kind, params and account is answered with, and it names the value
    /// refused (these words): a row that fails for a value names the
    /// value, which the equality alone would not hold if the door's own
    /// words omitted it.
    TheDoors(String),
}

/// C127, a row of the restored-rows table: a waiting row as an earlier
/// lifetime's `daemon.db` holds it (kind, params, account, submitter),
/// never having passed the client's door, the sessions it restores
/// beside, and what its failure says. Named for the test it replaced and
/// its case.
struct Restored {
    name: String,
    session: Session,
    kind: &'static str,
    params: Value,
    account: Option<&'static str>,
    submitted_by: &'static str,
    said: Said,
}

/// The restored-rows table's rows, written from the rows and the words of
/// the tests they replaced.
fn restored_rows() -> Vec<Restored> {
    let mut rows = Vec::new();
    // C127 (a row that renders no URL says why): the daemon's own work,
    // as a restored row or a child whose params came from a response, for
    // a live session whose admitted job would send at once.
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
        rows.push(Restored {
            name: format!("c127_a_row_that_renders_no_url_fails_naming_the_cause: {kind} {params}"),
            session: Session::Live,
            kind,
            params,
            account: Some("Alice#1234"),
            submitted_by: "daemon",
            said: Said::Carries(said, &["needs an id", "needs a name", "unrenderable realm"]),
        });
    }
    // C127 (a row that renders no URL spends no token): every kind whose
    // path params can render no URL past the daemon's own submit, on a
    // session whose refresh is due, so a handler that authenticates first
    // would send a refresh. Not here: `leagues` and `profile` always
    // render, and `characters` renders none only for a realm the daemon's
    // own submit refuses (the row after these).
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
        rows.push(Restored {
            name: format!(
                "c127_a_row_that_renders_no_url_fails_before_any_refresh: {kind} {params}"
            ),
            session: Session::RefreshDue,
            kind,
            params,
            account: Some("test-user"),
            submitted_by: "daemon",
            said: Said::Carries(said, &[]),
        });
    }
    // The same, for `characters`: its one no-URL case is a realm the
    // daemon's own submit refuses, which a restored row never met.
    rows.push(Restored {
        name: "c127_a_characters_row_that_renders_no_url_fails_before_any_refresh".into(),
        session: Session::RefreshDue,
        kind: "characters",
        params: json!({ "realm": "nope" }),
        account: Some("test-user"),
        submitted_by: "daemon",
        said: Said::TheDoors("nope".into()),
    });
    // C127 (one bound on `sleep`): a restored sleep whose `seconds` the
    // door would refuse fails in the door's own words rather than run a
    // default: a string `seconds` once slept 3 s.
    for seconds in [json!("1"), json!(true), json!(-1), json!(3601)] {
        rows.push(Restored {
            name: format!(
                "c127_a_restored_sleep_the_door_would_refuse_fails_in_its_words: sleep {seconds}"
            ),
            session: Session::None,
            kind: "sleep",
            params: json!({ "seconds": seconds }),
            account: None,
            submitted_by: "client",
            said: Said::TheDoors(format!("not {seconds}")),
        });
    }
    rows
}

/// C127 and C6, the rows that never passed the client's door: each row
/// is seeded waiting into a `daemon.db` the scenario opened, which the
/// door's daemon restores as a start does (C6), its sessions beside it;
/// the dispatcher runs it, and it fails in the door's words: its
/// `Result` carries the row's words, or equals the refusal a client's
/// submit of the same kind, params and account is answered with. Once
/// the runtime has settled, the account server's record and the journal
/// are empty: nothing was sent for it, no refresh among it. A restored
/// row is a persisted input, trusted as the daemon trusts it.
#[test]
fn c127_rows_that_never_passed_the_door_fail_in_its_words() {
    for row in restored_rows() {
        let Restored {
            name,
            session,
            kind,
            params,
            account,
            submitted_by,
            said,
        } = row;
        let dir = TestPath::dir();
        let db = JobDb::open(&daemon_db_path(&dir)).unwrap();
        db.upsert(&JobRow {
            id: 1,
            kind: kind.into(),
            state: "waiting".into(),
            priority: 0,
            submitted_by: submitted_by.into(),
            parent: None,
            retries: 0,
            account: account.map(str::to_string),
            params: params.clone(),
            outcome: None,
            deferred: None,
            cancel_requested: false,
            submitted_at: 0,
            updated_at: 0,
            started_at: None,
            finished_at: None,
        })
        .unwrap();
        let clock = Arc::new(ParkingClock::new());
        let sessions = session.on(&clock);
        let name = &name;
        Door::open(
            Server::Account(NO_HOLD),
            clock,
            Arc::new(RecordingCredentialStore::default()),
        )
        .sessions(sessions)
        .jobs_db(db)
        .restore_jobs(Retention::default())
        .run(|mut door| async move {
            let (job, outcome) = door.await_terminal(1).await;
            assert_eq!(job.state, JobState::Failed, "{name}: {outcome:?}");
            let Outcome::Failure {
                error,
                kind: Some(FailureKind::Internal),
                ..
            } = &outcome
            else {
                panic!("{name}: {outcome:?}")
            };
            match said {
                Said::Carries(said, wrong) => {
                    assert!(error.contains(said), "{name}: says {said:?}: {error}");
                    for wrong in wrong {
                        assert!(
                            !error.contains(wrong),
                            "{name}: a misleading cause: {error}"
                        );
                    }
                }
                Said::TheDoors(value) => {
                    let submit = submit_of(kind, params, 0, "client", account);
                    let answer = door.request(submit).await;
                    let refusal = assert_refused(answer, name, ErrorKind::Refused, &[], &[]);
                    assert_eq!(error, &refusal, "{name}: the failure is the door's words");
                    assert!(error.contains(&value), "{name}: names {value:?}: {error}");
                }
            }
            door.settle().await;
            assert!(door.sent().is_empty(), "{name}: nothing reached the wire");
            assert!(door.journal().is_empty(), "{name}: nothing was journaled");
            door.finish().await;
        });
    }
}
/// C127 (no legitimate value is refused or changed): realistic league,
/// character, stash and substash values on every realm a family takes,
/// each run through `route_for` and then `url::Url::parse` (what reqwest
/// does with the string), map to exactly these wire paths. A guard: it
/// holds before and after any tightening of the segment rule.
#[test]
fn c127_legitimate_values_render_to_these_wire_paths() {
    let (daemon, _) = test_daemon_with(
        Boundary::Function,
        Provider::mock("https://api.pathofexile.com"),
        refused(),
        Arc::new(ParkingClock::new()),
        RailsConfig::default(),
    );
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

/// C127 (one list of the kinds a client may submit): every kind the
/// protocol names, the list each frontend prints and the refusal above
/// lists, is a row of the daemon's table, so none is refused as unknown
/// in the words that name it; and the one row no list names is the
/// daemon's own. A component test: a table, no daemon built.
#[test]
fn c127_every_kind_the_protocol_names_is_a_row_of_the_table() {
    for kind in CLIENT_KINDS.iter().chain(MOCK_ONLY_KINDS) {
        assert!(
            KINDS.iter().any(|row| row.kind == *kind),
            "{kind} is listed and not run"
        );
    }
    let own: Vec<&str> = KINDS
        .iter()
        .filter(|row| kind_door(row.kind) == KindDoor::DaemonOnly)
        .map(|row| row.kind)
        .collect();
    assert_eq!(own, ["probe"]);
}

/// C47, #56: an answer that cannot be serialized is answered, not a
/// panic in the connection's task: the writer puts the protocol's
/// `internal` refusal on the line in its place, one frame a client reads
/// as an `error` of kind `internal`, and a write after it is a frame of
/// its own, so the connection carries on. A component test of the
/// writer: no request's answer fails to serialize today, so no request
/// reaches the fallback.
#[tokio::test]
async fn a_response_that_cannot_be_serialized_is_answered_internal() {
    use crate::daemon::server::write_line;
    struct Unserializable;
    impl serde::Serialize for Unserializable {
        fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
            Err(serde::ser::Error::custom("a value with no JSON form"))
        }
    }
    let mut wire = Vec::new();
    write_line(&mut wire, &Unserializable).await.unwrap();
    write_line(&mut wire, &Response::ResyncRequired { missed: 1 })
        .await
        .unwrap();
    let text = String::from_utf8(wire).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 2, "{text}");
    match serde_json::from_str::<Response>(lines[0]).unwrap() {
        Response::Error { kind, message } => {
            assert_eq!(kind, ErrorKind::Internal, "{message}");
            assert!(message.contains("could not serialize"), "{message}");
        }
        other => panic!("the fallback reads as {other:?}"),
    }
    assert!(
        matches!(
            serde_json::from_str::<Response>(lines[1]).unwrap(),
            Response::ResyncRequired { missed: 1 }
        ),
        "the next frame is intact: {}",
        lines[1]
    );
}

/// C166: an answer whose line would pass the frame bound is never
/// written: the writer puts the protocol's `internal` refusal in its
/// place, which says it was too large and that the connection stays, and
/// the next frame is a frame of its own. A component test of the writer
/// at a bound of 100 bytes: a line of exactly the bound is written as it
/// is, one byte more is answered in the refusal's place.
#[tokio::test]
async fn c166_an_answer_past_the_frame_bound_is_answered_internal() {
    use crate::daemon::server::write_line_within;
    const BOUND: usize = 100;
    // {"resp":"error","kind":"refused","message":"…"} is 46 bytes and
    // the message
    let answer = |message_bytes: usize| Response::Error {
        kind: ErrorKind::Refused,
        message: "m".repeat(message_bytes),
    };
    assert_eq!(serde_json::to_string(&answer(54)).unwrap().len(), BOUND);
    let mut wire = Vec::new();
    write_line_within(&mut wire, &answer(54), BOUND)
        .await
        .unwrap();
    write_line_within(&mut wire, &answer(55), BOUND)
        .await
        .unwrap();
    write_line_within(&mut wire, &Response::ResyncRequired { missed: 1 }, BOUND)
        .await
        .unwrap();
    let text = String::from_utf8(wire).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 3, "{text}");
    assert_eq!(
        lines[0],
        serde_json::to_string(&answer(54)).unwrap(),
        "the bound is a frame"
    );
    match serde_json::from_str::<Response>(lines[1]).unwrap() {
        Response::Error { kind, message } => {
            assert_eq!(kind, ErrorKind::Internal, "{message}");
            assert!(message.contains("too large to send"), "{message}");
            assert!(message.contains("the connection stays open"), "{message}");
        }
        other => panic!("the answer past the bound reads as {other:?}"),
    }
    assert!(
        matches!(
            serde_json::from_str::<Response>(lines[2]).unwrap(),
            Response::ResyncRequired { missed: 1 }
        ),
        "the next frame is intact: {}",
        lines[2]
    );
}

// ---- persistence (C6) ------------------------------------------------

/// A harness daemon on a file-backed `daemon.db` (test daemons default
/// to a throwaway in-memory one), restored from it like `run` does.
fn persisting_daemon(db_path: &std::path::Path) -> (Arc<Daemon>, TestPath) {
    let (mut daemon, log_path) =
        test_daemon(Boundary::Function, refused(), Arc::new(ParkingClock::new()));
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

/// A door on the `daemon.db` under `dir`, as a start opens it: the
/// account server holding nothing, one empty session, the queue file
/// the scenario's own, which it may break (`break_queue`), seed behind
/// the daemon, or read as an observer once the run is over (C45).
fn queue_door(dir: &std::path::Path) -> DoorBuilder {
    Door::open(
        Server::Account(NO_HOLD),
        Arc::new(ParkingClock::new()),
        Arc::new(RecordingCredentialStore::default()),
    )
    .jobs_db(JobDb::open(&daemon_db_path(dir)).unwrap())
}

/// The refusal a request is answered with: its kind and its words.
fn refusal_of(answer: Response, what: &str) -> (ErrorKind, String) {
    match answer {
        Response::Error { kind, message } => (kind, message),
        other => panic!("{what} answered {other:?}"),
    }
}

/// C6: the queue and its results survive a restart: a finished job's
/// result is answered by the next lifetime, a running job is re-queued,
/// a waiting one keeps its reprioritisation, ids continue, and a third
/// lifetime has nothing to restore. A component test: three lifetimes in
/// one test, the second's Running fabricated under `shared` as `process`
/// writes it, which no door of one lifetime reaches. The jobs the second
/// lifetime runs sleep zero seconds: a sleep is on the daemon's clock
/// (C160), which nothing moves in a component test.
#[tokio::test]
async fn the_queue_and_results_survive_a_daemon_restart() {
    let db = test_db_path();
    // Lifetime 1: one job finished, one running, one waiting. No
    // dispatcher runs — the running state is written the way
    // `process` writes it — so nothing of this lifetime survives the
    // drop to write behind lifetime 2's back.
    let (daemon, log1) = persisting_daemon(&db);
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
            json!({ "seconds": 0.0 }),
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
            json!({ "seconds": 0.0 }),
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
    let (daemon, log2) = persisting_daemon(&db);
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
            json!({ "seconds": 0.0 }),
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
    let (daemon, log3) = persisting_daemon(&db);
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
/// server's, and the journal matches them; after the run, an observer
/// reads both rows' ends on disk.
#[test]
fn c50_a_lost_logins_profile_fetch_fails_at_restore_and_sends_nothing() {
    let dir = TestPath::dir();
    let seed = JobDb::open(&daemon_db_path(&dir)).unwrap();
    let waiting_profile = |id: JobId, submitted_by: &str| JobRow {
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
        started_at: None,
        finished_at: None,
    };
    seed.upsert(&waiting_profile(1, "daemon")).unwrap();
    seed.upsert(&waiting_profile(2, "cli:1")).unwrap();
    drop(seed);
    queue_door(&dir)
        .sessions(logged_in_sessions())
        .restore_jobs(Retention::default())
        .run(|mut door| async move {
            let (login, outcome) = door.await_terminal(1).await;
            assert_eq!(login.state, JobState::Failed, "{outcome:?}");
            let Outcome::Failure {
                error,
                kind: Some(FailureKind::NotLoggedIn),
                ..
            } = &outcome
            else {
                panic!("{outcome:?}")
            };
            assert!(
                error.contains("login") && error.contains("log in again") && !error.contains('1'),
                "the failure names the lost login and the remedy, no id: {error}"
            );
            let (client, outcome) = door.await_terminal(2).await;
            assert_eq!(client.state, JobState::Done, "{outcome:?}");
            let wire = door.sent();
            assert_eq!(
                wire,
                [sent("test-user", "POST"), sent("test-user", "GET")],
                "the frontend's fetch replays; the lost login's sends nothing"
            );
            assert_eq!(door.journal_methods(), wire_methods_of(&wire));
            door.finish().await;
        });
    let state = |id| on_disk(&dir, id).expect("the row").job.state;
    assert_eq!(state(1), "failed", "the failure is on disk");
    assert_eq!(
        state(2),
        "done",
        "the wire's two sends are the frontend's fetch"
    );
}

/// C6, the restore: a probe from the previous daemon is dropped (gone
/// from the table too); a running job that was asked to cancel concludes
/// cancelled; a running job on a declared no-probe route is not replayed
/// (a replay would send against an empty limiter) and fails; an apply
/// whose last child finished before the daemon died finishes with its
/// held result; and an apply caught mid-fan-out holds for its waiting
/// child, finishing as interrupted, never success (the full child set is
/// unknown), with its children's tally, once that child ends. Ids
/// continue past the restored rows. Read through `List`, `Result`, a
/// cancel and a submit, the restore held unlanded so the dispatcher runs
/// nothing restored; after the run, an observer finds no probe row.
#[test]
fn restore_drops_probes_and_finishes_a_parent_whose_children_are_done() {
    let dir = TestPath::dir();
    let seed = JobDb::open(&daemon_db_path(&dir)).unwrap();
    let row = |id: u64, kind: &str, state: &str, parent: Option<u64>| JobRow {
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
        started_at: None,
        finished_at: None,
    };
    // An apply whose last child finished just before the daemon
    // died: its own result is held, deferred.
    let mut parent = row(1, "apply", "running", None);
    parent.deferred = Some(json!({ "outcome": "success", "payload": { "tabs_listed": 1 } }));
    seed.upsert(&parent).unwrap();
    seed.upsert(&row(2, "stash", "done", Some(1))).unwrap();
    // Per-lifetime: a probe from the previous daemon.
    seed.upsert(&row(3, "probe", "waiting", None)).unwrap();
    // A running job that was asked to cancel: it never ran again.
    let mut cancelling = row(4, "sleep", "running", None);
    cancelling.cancel_requested = true;
    seed.upsert(&cancelling).unwrap();
    // Running on a declared no-probe route: a replay would send
    // against an empty limiter, so it is not replayed.
    seed.upsert(&row(5, "leagues", "running", None)).unwrap();
    // Mid-fan-out: children submitted, held result not yet
    // written. Re-running would duplicate the children.
    seed.upsert(&row(6, "apply", "running", None)).unwrap();
    seed.upsert(&row(7, "stash", "waiting", Some(6))).unwrap();
    drop(seed);
    queue_door(&dir)
        .restore_jobs(Retention::default())
        .restore_landed(false)
        .run(|mut door| async move {
            let states: Vec<(JobId, JobState)> = listed(&mut door)
                .await
                .iter()
                .map(|j| (j.id, j.state))
                .collect();
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
            // C165 (the review of round E, finding 4): the rows record no
            // start, as schema v1 wrote them, and a parent the restore
            // persists still running, its result held or its children
            // waiting, never ran again: none is invented
            for job in listed(&mut door).await {
                if [1, 6].contains(&job.id) {
                    assert_eq!(job.times.started_at, None, "{job:?}");
                }
            }
            let (_, outcome) = door.await_terminal(5).await;
            let Outcome::Failure {
                error,
                kind: Some(FailureKind::Interrupted),
                ..
            } = outcome
            else {
                panic!("the leagues replay did not fail: {outcome:?}")
            };
            assert!(error.contains("not replayed"), "{error}");
            let (_, outcome) = door.await_terminal(1).await;
            let Outcome::Success { payload } = outcome else {
                panic!("parent did not finish with its held result: {outcome:?}")
            };
            assert_eq!(payload["tabs_listed"], json!(1));
            assert_eq!(payload["children"]["done"], json!(1));
            // The held parent finishes when its remaining child does — as
            // interrupted, never success: the full child set is unknown.
            match door.request(Request::Cancel { id: 7 }).await {
                Response::Ack { .. } => {}
                other => panic!("the cancel of job 7 answered {other:?}"),
            }
            let (_, outcome) = door.await_terminal(6).await;
            let Outcome::Failure {
                error, children, ..
            } = outcome
            else {
                panic!("held parent must finish as interrupted, not success: {outcome:?}")
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
            let next = submit_of("sleep", json!({}), 0, "client", None);
            assert_eq!(
                submitted(door.request(next).await, "a sleep after the restore"),
                8,
                "ids continue past the restored rows"
            );
            door.finish().await;
        });
    assert!(
        on_disk(&dir, 3).is_none(),
        "the probe row is gone from the table too"
    );
}
/// C6: a submit whose queue write fails rolls its id back, and a failed
/// queue dispatches nothing, its waiting job left unpicked. Its refusal,
/// `QueueFailed` saying the daemon refuses new jobs, is a row of
/// `c6_the_queue_file_failing_through_the_door`. A component test: the
/// rolled-back id and the empty pick have no boundary read (after the
/// break no submit is admitted to show the next id, and the job never
/// picked reads Waiting as one not yet picked does).
#[tokio::test]
async fn a_failed_queue_write_refuses_new_jobs_and_stops_dispatch() {
    let db = test_db_path();
    let (daemon, log) = persisting_daemon(&db);
    daemon
        .submit("sleep".into(), json!({}), 0, "t".into(), None)
        .unwrap();
    break_queue(&db);
    daemon
        .submit("sleep".into(), json!({}), 0, "t".into(), None)
        .unwrap_err();
    assert_eq!(
        daemon.shared.lock().unwrap().next_id,
        2,
        "the refused submit rolled its id back"
    );
    assert!(
        daemon.pick_runnable().is_empty(),
        "a failed queue dispatches nothing"
    );
    finish_harness(None, &log);
}

/// C6 as amended: a concluded tree leaves memory only when every row is
/// persisted. A job that concludes after a failed queue write stays in
/// memory, which answers for it: its row was not written. A component
/// test: the conclusion is fabricated after the break
/// (`start_and_finish`), and memory's membership has no boundary read.
#[tokio::test]
async fn a_job_concluded_after_a_failed_queue_write_stays_in_memory() {
    let db = test_db_path();
    let (daemon, log) = persisting_daemon(&db);
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
        started_at: None,
        finished_at: None,
    }
}

/// C6, uniform eviction: a tree the restore brought back leaves memory
/// when it concludes, as any tree does, and the list names this daemon's
/// work and what the restore brought back, never the rows between. A
/// waiting job and an apply held for one waiting child (with its done
/// child) are restored; two of the earlier lifetime's finished jobs sit
/// on disk, one before the restored ids and one between them and this
/// daemon's first id. The child concludes, the parent finishes, and
/// memory holds only the waiting job. The list names it, the tree, and
/// a job this daemon then runs; neither earlier finished job, which stay
/// on disk for `result` (a list of every such row is what would carry a
/// long history past the frame). A component test: memory's membership
/// (`shared.jobs`) is what it asserts, which no protocol answer reads
/// apart from the table's.
#[tokio::test]
async fn a_restored_tree_leaves_memory_and_the_list_skips_the_rows_between() {
    let db = test_db_path();
    // The finished rows outside the tree are recent: the restore's prune
    // keeps them, so only the list's rule leaves them out.
    let recent = |id| JobRow {
        updated_at: MANUAL_WALL_ORIGIN.as_secs() as i64,
        ..disk_row(id, "sleep", "done", None)
    };
    {
        let other = JobDb::open(&db).unwrap();
        other
            .upsert(&disk_row(5, "sleep", "waiting", None))
            .unwrap();
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
    let (daemon, log) = persisting_daemon(&db);
    let in_memory = || {
        let mut ids: Vec<JobId> = daemon.shared.lock().unwrap().jobs.keys().copied().collect();
        ids.sort_unstable();
        ids
    };
    assert_eq!(
        in_memory(),
        [5, 11, 12, 13],
        "the restore brought back the open work"
    );

    daemon.start_and_finish(12, Outcome::Success { payload: json!({}) });
    assert_eq!(
        in_memory(),
        [5],
        "the restored tree left memory when it concluded"
    );
    let (info, outcome) = held(&daemon, 11).unwrap();
    assert_eq!(info.state, JobState::Done);
    let Some(Outcome::Success { payload }) = outcome else {
        panic!("the restored parent did not succeed: {outcome:?}")
    };
    assert_eq!(payload["children"]["done"], json!(2), "{payload}");

    let own = daemon
        .submit("sleep".into(), json!({}), 0, "t".into(), None)
        .unwrap();
    assert_eq!(own, 15, "this daemon's first id");
    daemon.start_and_finish(own, Outcome::Success { payload: json!({}) });
    let listed: Vec<JobId> = daemon.job_list().unwrap().iter().map(|j| j.id).collect();
    assert_eq!(
        listed,
        [5, 11, 12, 13, 15],
        "what the restore brought back and this daemon's work, nothing between"
    );
    // C166: the finished pages read the same rows, newest first, the
    // restored by their ids below this daemon's own; the open job is
    // memory's.
    let page = |state, after| {
        let page = daemon.job_page(state, 2, after).unwrap();
        (
            page.jobs.iter().map(|j| j.id).collect::<Vec<_>>(),
            page.next,
        )
    };
    assert_eq!(page(ListState::Finished, None), (vec![15, 13], Some(13)));
    assert_eq!(page(ListState::Finished, Some(13)), (vec![12, 11], None));
    assert_eq!(page(ListState::Open, None), (vec![5], None));
    let counts = daemon.job_page(ListState::Open, 1, None).unwrap().counts;
    assert_eq!((counts.total, counts.waiting, counts.done), (5, 1, 4));
    for id in [9, 14] {
        assert!(
            JobDb::open(&db).unwrap().get(id).unwrap().is_some(),
            "row {id} is still on disk, for `result`"
        );
    }
    finish_harness(None, &log);
}

/// C6: a queue the restore cannot read is fatal to the start, in words
/// that say so. A component test: a restore that fails fails the door's
/// construction, which the door cannot answer.
#[tokio::test]
async fn a_broken_queue_is_fatal_to_restore() {
    let db = test_db_path();
    let (daemon, log) = persisting_daemon(&db);
    break_queue(&db);
    let err = daemon.restore_jobs(Retention::default()).unwrap_err();
    assert!(err.to_string().contains("could not read"), "{err:#}");
    finish_harness(None, &log);
}

/// C6: waiting → running is the transition that gates a send: if its
/// write fails, disk still says waiting, and after a restart a no-probe
/// job would replay blind. The job must not run: it reverts to waiting,
/// and the failed write trips the queue. A component test: the break
/// must land after the admission and before the dispatcher's
/// transition, which no boundary step orders (a door holding its restore
/// unlanded never transitions; one landed transitions at the submit).
#[tokio::test]
async fn a_transition_that_cannot_persist_does_not_run() {
    let db = test_db_path();
    let (daemon, log) = persisting_daemon(&db);
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
    // The dispatcher's transition, awaited: its failed write sets the
    // queue failure and reverts in one critical section, so once the
    // failure is read the revert has landed.
    tokio::time::timeout(HANG, async {
        while daemon.queue_failed().is_none() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("the failed write tripped the queue");
    let state = daemon.shared.lock().unwrap().jobs[&id].info.state;
    assert_eq!(
        state,
        JobState::Waiting,
        "the job reverted instead of running"
    );
    finish_harness(dispatcher, &log);
}

/// C6 (a queue write that fails is never silent), the queue file failing
/// through the door: the scenario breaks the `daemon.db` its door's
/// daemon holds, and an operation after the break is answered
/// `QueueFailed` with its words. A result the table cannot read is
/// reported, never "no job"; a submit whose write fails refuses new jobs.
#[test]
fn c6_the_queue_file_failing_through_the_door() {
    for (name, request, said) in [
        (
            "a_result_read_failure_is_reported_not_no_job",
            Request::Result { id: 42 },
            "could not read the persisted queue",
        ),
        (
            "a_failed_queue_write_refuses_new_jobs_and_stops_dispatch: the submit",
            submit_of("sleep", json!({}), 0, "client", None),
            "refuses new jobs",
        ),
    ] {
        let dir = TestPath::dir();
        let path = daemon_db_path(&dir);
        queue_door(&dir).run(|mut door| async move {
            break_queue(&path);
            let (kind, message) = refusal_of(door.request(request).await, name);
            assert_eq!(kind, ErrorKind::QueueFailed, "{name}: {message}");
            assert!(message.contains(said), "{name}: says {said:?}: {message}");
            door.finish().await;
        });
    }
}

/// C6: a cancel and a reprioritisation whose queue write fails say so,
/// `QueueFailed`, instead of claiming success: the priority is unchanged
/// and the cancellation holds for this lifetime only, as `Status` reads
/// them. The door's restore is held unlanded, so the dispatcher picks
/// nothing and the sleep never runs; a sleep's submit with no account,
/// a cancel, a reprioritisation and a status never wait on the restore.
#[test]
fn cancel_and_reprioritize_report_a_failed_queue_write() {
    let dir = TestPath::dir();
    let path = daemon_db_path(&dir);
    queue_door(&dir)
        .restore_landed(false)
        .run(|mut door| async move {
            let submit = submit_of("sleep", json!({}), 3, "client", None);
            let id = submitted(door.request(submit).await, "a sleep");
            break_queue(&path);
            let (kind, message) = refusal_of(
                door.request(Request::SetPriority { id, priority: 9 }).await,
                "a reprioritisation after the break",
            );
            assert_eq!(kind, ErrorKind::QueueFailed, "{message}");
            assert!(message.contains("priority unchanged"), "{message}");
            let (kind, message) = refusal_of(
                door.request(Request::Cancel { id }).await,
                "a cancel after the break",
            );
            assert_eq!(kind, ErrorKind::QueueFailed, "{message}");
            assert!(message.contains("may not survive"), "{message}");
            let Response::Status { job } = door.request(Request::Status { id }).await else {
                panic!("job {id}'s status did not read")
            };
            assert_eq!(
                job.priority, 3,
                "the priority as it was before the failed change"
            );
            assert_eq!(
                job.state,
                JobState::Cancelled,
                "the cancellation still holds for this lifetime"
            );
            door.finish().await;
        });
}

/// C6: a job not in memory is read from the table and never acted on.
/// A row still waiting on disk that memory does not hold (a probe whose
/// delete failed at the restore) is refused to `cancel` and
/// `set-priority` as `queue_failed`, in words saying the daemon does not
/// hold it and the next start drops or replays it; a finished row
/// is the `wrong_state` refusal. Nothing panics, and the open row is
/// left as it was: after the run, an observer reads it waiting. The two
/// rows are written behind the daemon, as a neighbour connection would,
/// after the door opened: a staged input.
#[test]
fn a_waiting_row_on_disk_and_not_in_memory_is_never_acted_on() {
    let dir = TestPath::dir();
    let path = daemon_db_path(&dir);
    queue_door(&dir).run(|mut door| async move {
        {
            let other = JobDb::open(&path).unwrap();
            other
                .upsert(&disk_row(40, "probe", "waiting", None))
                .unwrap();
            other.upsert(&disk_row(41, "sleep", "done", None)).unwrap();
        }
        for request in [
            Request::Cancel { id: 40 },
            Request::SetPriority {
                id: 40,
                priority: 3,
            },
        ] {
            let what = format!("{request:?}");
            let (kind, message) = refusal_of(door.request(request).await, &what);
            assert_eq!(kind, ErrorKind::QueueFailed, "{what}: {message}");
            for words in [
                "job 40 is open on disk",
                "does not hold it",
                "next start of the daemon drops or replays it",
            ] {
                assert!(message.contains(words), "{what}: {message}");
            }
        }
        let (kind, message) = refusal_of(
            door.request(Request::Cancel { id: 41 }).await,
            "a cancel of the finished row",
        );
        assert_eq!(
            (kind, message.as_str()),
            (ErrorKind::WrongState, "job 41 already done")
        );
        let (kind, message) = refusal_of(
            door.request(Request::SetPriority {
                id: 41,
                priority: 3,
            })
            .await,
            "a reprioritisation of the finished row",
        );
        assert_eq!(
            (kind, message.as_str()),
            (ErrorKind::WrongState, "job 41 is done, not waiting")
        );
        door.finish().await;
    });
    let row = on_disk(&dir, 40).expect("row 40");
    assert_eq!(row.job.state, "waiting", "the open row is left as it was");
}

/// C165 (a job's record says when): each time is set at its transition
/// on the daemon's one clock, through the door — the submission at
/// submit, the first start when the job runs, the finish when it is
/// terminal — and the result and the ledger's row carry the same. A
/// `sleep` of 10 s on the parked clock: running, it has started and not
/// finished; done, it finished 10 s after it started.
#[test]
fn c165_each_time_is_set_at_its_transition_and_the_result_and_row_carry_them() {
    let store_dir = TestPath::dir();
    let clock = Arc::new(ParkingClock::new());
    Door::open(
        Server::Account(NO_HOLD),
        clock,
        Arc::new(RecordingCredentialStore::default()),
    )
    .store_dir(store_dir.to_path_buf())
    .run(|mut door| async move {
        let id = submitted(
            door.request(submit_of(
                "sleep",
                json!({ "seconds": 10 }),
                0,
                "client",
                None,
            ))
            .await,
            "a sleep",
        );
        let running = door
            .await_job(id, |job| job.state == JobState::Running)
            .await;
        let submitted_at = running.times.submitted_at.expect("submitted");
        assert_eq!(
            (running.times.started_at, running.times.finished_at),
            (Some(submitted_at), None),
            "{running:?}"
        );
        let (done, _) = door.await_terminal(id).await;
        assert_eq!(
            (
                done.times.submitted_at,
                done.times.started_at,
                done.times.finished_at
            ),
            (
                Some(submitted_at),
                Some(submitted_at),
                Some(submitted_at + 10)
            ),
            "{done:?}"
        );
        assert!(
            done.times.submitted_at_age.is_none(),
            "the daemon says no age"
        );
        match door.request(Request::Result { id }).await {
            Response::Result { times, .. } => assert_eq!(times, done.times),
            other => panic!("{other:?}"),
        }
        let row = door
            .daemon()
            .jobs_db
            .lock()
            .unwrap()
            .get(id)
            .unwrap()
            .expect("the row");
        assert_eq!(
            (row.submitted_at, row.started_at, row.finished_at),
            (submitted_at, Some(submitted_at), Some(submitted_at + 10))
        );
        door.finish().await;
    });
}

/// C165, a component test of `Entry::start` and `Entry::stamp` (each
/// transition by hand, which a scenario cannot stage without a 429
/// script): a waiting job has no start; its first run sets one, which a
/// 429's re-queue and a second run keep; a persist of a running job
/// starts nothing; its end sets the finish, once.
#[test]
fn c165_a_requeued_job_keeps_its_first_start() {
    let fresh = |state: JobState| Entry {
        info: JobInfo {
            id: 1,
            kind: "stash".into(),
            state,
            priority: 0,
            submitted_by: "client".into(),
            eta_seconds: None,
            parent: None,
            retries: 0,
            account: None,
            params: json!({}),
            times: JobTimes {
                submitted_at: Some(100),
                ..JobTimes::default()
            },
        },
        outcome: None,
        cancel_requested: false,
        deferred: None,
    };
    let mut entry = fresh(JobState::Waiting);
    let at = |entry: &Entry| (entry.info.times.started_at, entry.info.times.finished_at);
    entry.stamp(101);
    assert_eq!(at(&entry), (None, None), "waiting: no start");
    entry.start(105);
    entry.stamp(105);
    assert_eq!(at(&entry), (Some(105), None), "the first start");
    entry.info.state = JobState::Waiting;
    entry.info.retries = 1;
    entry.stamp(106);
    entry.start(140);
    entry.stamp(140);
    assert_eq!(at(&entry), (Some(105), None), "a 429's re-queue keeps it");
    // a persist of a running job is no start (the review of round E,
    // finding 4)
    let mut held = fresh(JobState::Running);
    held.stamp(150);
    assert_eq!(at(&held), (None, None), "a persist starts nothing");
    entry.info.state = JobState::Done;
    entry.stamp(141);
    entry.stamp(150);
    assert_eq!(at(&entry), (Some(105), Some(141)), "the finish, once");
}
