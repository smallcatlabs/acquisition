//! The layer against a daemon, or against its absence (C10, C45, C111,
//! C23): the quote with no daemon and with a wedged peer, the ledger's
//! answers when no daemon runs, and the apply flow over the mock daemon —
//! applied twice, held by a queued job, a parent whose child failed; and
//! the diagnosis (#79) with and without a daemon.
//!
//! The harness is the one every crate's process tests share (C122),
//! `acquisition-client/tests/harness/mod.rs`, included by `#[path]`: one
//! lock and the one isolation list applied to this process (`session`),
//! the `acqd` the gate built linked beside this test executable
//! (`acqd_built`, `start_daemon_in_process`), and a login through the
//! mock's approve page. This file sets the process environment, so it is
//! its own test binary beside `documents.rs`. Run `cargo build
//! --workspace` first: a test run that leaves out the daemon's package
//! does not build the daemon. Nothing here reaches GGG: the daemon runs
//! the mock provider.

#[path = "../../acquisition-client/tests/harness/mod.rs"]
mod harness;

use std::time::{Duration, Instant};

use acquisition_client::client::{Client, Observed};
use acquisition_plan::pricing::Expect;
use acquisition_plan::{PolicyWrite, RefreshAction};
use acquisition_protocol::job::Outcome;
use acquisition_protocol::realm::Realm;
use acquisition_store::jobs::{JobDb, JobRow, daemon_db_path};
use acquisition_store::world::{World, default_root, intended_root};
use acquisition_verbs::jobs::{NO_RESULT, submit, wait};
use acquisition_verbs::refresh::{quote, quote_within};
use acquisition_verbs::{Apply, Fault, Frontend, Job, Opened};
use harness::{Proc, Session, acqd_built, login_in_process, session, start_daemon_in_process};
use serde_json::json;

// ---- the fixtures -------------------------------------------------------

/// A daemon with `alice` logged in, her stash listing fetched and a
/// policy over Standard's tabs: the account opened, and a client.
async fn world(s: &Session) -> (Proc, Frontend, Opened, Client) {
    let daemon = start_daemon_in_process(s, &acqd_built()).await;
    let frontend = Frontend::agent("test").expect("the scratch environment");
    let mut client = frontend.connect().await.expect("the daemon");
    login_in_process(&mut client, Some("alice")).await;
    let id = submit(
        &frontend,
        &mut client,
        "stashes",
        json!({ "realm": "pc", "league": "Standard" }),
        0,
        None,
    )
    .await
    .expect("a submit");
    let listed = wait(&mut client, id, None).await.expect("the listing");
    assert!(matches!(listed, Outcome::Success { .. }), "{listed:?}");
    let account = frontend.account(None).expect("alice");
    account
        .set_policy(
            &json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard":
                { "tabs": "all", "max_age_seconds": 3600 } } } } }),
            Expect::Create,
            PolicyWrite::Replace,
        )
        .expect("the policy");
    let opened = account.open().expect("the stores");
    (daemon, frontend, opened, client)
}

fn row(id: u64, state: &str, outcome: Option<serde_json::Value>) -> JobRow {
    JobRow {
        id,
        kind: "stash".into(),
        state: state.into(),
        priority: 0,
        submitted_by: "test".into(),
        parent: None,
        retries: 0,
        account: Some("Alice#1234".into()),
        params: json!({ "league": "Standard", "id": format!("tab{id}") }),
        outcome,
        deferred: None,
        cancel_requested: false,
        submitted_at: 1_700_000_000,
        updated_at: 1_700_000_010,
    }
}

/// The ledger under the session's world, as the daemon's writer leaves it.
fn ledger(s: &Session, rows: &[JobRow]) {
    let dir = s.base.join("store").join("mock");
    let db = JobDb::open(&daemon_db_path(&dir)).unwrap();
    for r in rows {
        db.upsert(r).unwrap();
    }
    db.checkpoint().unwrap();
}

// ---- no daemon ----------------------------------------------------------

/// The quote's absent-daemon note is one sentence on every surface.
#[tokio::test]
async fn a_quote_with_no_daemon_is_the_one_note() {
    let _s = session("quote");
    let frontend = Frontend::agent("test").unwrap();
    let dir = frontend.dir();
    // A plan to quote: the documents test's fixture, in the session's world.
    acquisition_store::Index::load(&dir)
        .unwrap()
        .record_login("Alice#1234", "u-q", false, 1, &[])
        .unwrap();
    acquisition_store::Store::open(&acquisition_store::account_path(&dir, "Alice#1234"))
        .unwrap()
        .record(
            &acquisition_store::Endpoint::Profile,
            &json!({}),
            200,
            &json!({ "uuid": "u-q", "name": "Alice#1234" }),
            1,
        )
        .unwrap();
    let account = frontend.account(None).unwrap();
    account
        .set_policy(
            &json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard":
                { "tabs": "all", "max_age_seconds": 3600 } } } } }),
            Expect::Create,
            PolicyWrite::Replace,
        )
        .unwrap();
    let mut opened = account.open().unwrap();
    let planned = opened.plan(Realm::Pc, "Standard").await.unwrap();
    assert_eq!(
        planned.quote_note,
        Some(acquisition_verbs::QuoteNote::NoDaemon)
    );
}

/// A socket that accepts and never answers the handshake: the bounded
/// quote hands back the offline plan with the timeout named.
#[tokio::test]
async fn a_wedged_daemon_cannot_block_the_offline_plan() {
    let s = session("wedged");
    let sock = acquisition_store::world::World::observe()
        .unwrap()
        .socket_path()
        .unwrap();
    let _listener = acquisition_client::endpoint::Listener::bind(&sock).unwrap();
    // The plan: compiled from a hand-built snapshot.
    let snapshot = acquisition_store::RefreshSnapshot {
        account_uuid: "u-w".into(),
        account_name: Some("Alice#1234".into()),
        realm: "pc".into(),
        league: "Standard".into(),
        taken_at: 1_000,
        stash_listing: None,
        tabs: Vec::new(),
        character_listing: None,
        characters: Vec::new(),
        league_list: None,
        policy: Some(acquisition_store::AnnotationRow {
            scope: acquisition_store::SYNC_POLICY_SCOPE.into(),
            key: acquisition_store::SYNC_POLICY_KEY.into(),
            kind: acquisition_store::SYNC_POLICY_KIND.into(),
            value: json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard":
                { "tabs": "all", "max_age_seconds": 3600 } } } } }),
            revision: 1,
            created_at: 1_000,
            updated_at: 1_000,
            written_via: "test".into(),
            actor: None,
        }),
        response_watermark: 0,
        fetches: Default::default(),
    };
    let plan = acquisition_verbs::refresh::compile("mock", &snapshot, "Alice#1234", 2_000).unwrap();
    // The outer watchdog outlives the tested bound: if the production
    // timeout is ever removed, this fails instead of hanging the gate.
    let planned = tokio::time::timeout(
        Duration::from_secs(10),
        quote_within(
            &s.base.join("store"),
            plan.clone(),
            Duration::from_millis(200),
        ),
    )
    .await
    .expect("the bounded quote attempt itself never returned");
    assert_eq!(planned.plan, plan, "the compiled plan comes back untouched");
    let note = planned.quote_note.expect("a timed-out quote says why");
    assert_eq!(
        note,
        acquisition_verbs::QuoteNote::Timeout(Duration::from_millis(200))
    );
    drop(_listener);
    let _ = std::fs::remove_file(&sock);
    drop(s);
}

/// C45: with no daemon a job's status is the ledger's row with `next`,
/// a job the ledger does not hold is `unknown_job` with `next`, and a
/// finished job's result is its outcome from disk.
#[tokio::test]
async fn with_no_daemon_a_job_is_answered_from_the_ledger() {
    let s = session("ledger");
    ledger(
        &s,
        &[
            row(1, "waiting", None),
            row(
                2,
                "done",
                Some(json!({ "outcome": "success", "payload": { "n": 1 } })),
            ),
        ],
    );
    let frontend = Frontend::agent("test").unwrap();
    let Job::Absent { persisted, next } = frontend.job(1).await.unwrap() else {
        panic!("a live answer with no daemon")
    };
    assert_eq!(persisted.state, "waiting");
    assert!(next.contains("acq auth check"), "{next}");

    let fault = frontend.job(9).await.unwrap_err();
    assert!(matches!(fault, Fault::NoJobOnDisk { id: 9, .. }), "{fault}");
    assert_eq!(fault.kind(), "unknown_job");
    assert!(fault.fields()["next"].is_string());

    let result = frontend.result(2).await.unwrap();
    assert_eq!(
        serde_json::to_value(&result).unwrap(),
        json!({ "job_id": 2, "account": "Alice#1234", "outcome": "success", "payload": { "n": 1 } })
    );
    let fault = frontend.result(1).await.unwrap_err();
    assert!(matches!(fault, Fault::Unfinished { id: 1, .. }), "{fault}");
    assert_eq!(fault.kind(), "wrong_state");

    // and an observer spawned nothing
    assert!(matches!(frontend.observe().await, Ok(None)));
}

/// C45: with no daemon the queue is the ledger's: `jobs` lists the open
/// rows as the last daemon left them (two waiting, one a child, and one
/// recorded running; a finished and a cancelled row left out), their bulk
/// columns unread, and `daemon` counts them, the picture's age the newest
/// write of any row; neither spawns, and a world with no ledger yet is
/// `persisted: null`, not an empty queue.
#[tokio::test]
async fn with_no_daemon_the_queue_and_its_counts_are_the_ledgers() {
    let s = session("queue");
    let frontend = Frontend::agent("test").unwrap();
    assert_eq!(
        serde_json::to_value(frontend.jobs(false).await.unwrap()).unwrap(),
        json!({ "running": false, "persisted": null })
    );
    ledger(
        &s,
        &[
            JobRow {
                updated_at: 1_700_000_010,
                ..row(1, "waiting", None)
            },
            JobRow {
                updated_at: 1_700_000_030,
                ..row(2, "running", None)
            },
            JobRow {
                parent: Some(2),
                updated_at: 1_700_000_020,
                ..row(3, "waiting", None)
            },
            JobRow {
                updated_at: 1_700_000_005,
                ..row(
                    4,
                    "done",
                    Some(json!({ "outcome": "success", "payload": {} })),
                )
            },
            JobRow {
                updated_at: 1_700_000_040,
                ..row(5, "cancelled", None)
            },
        ],
    );
    let jobs = serde_json::to_value(frontend.jobs(false).await.unwrap()).unwrap();
    assert_eq!(jobs["running"], json!(false), "{jobs}");
    assert_eq!(
        jobs["persisted"]["written_at"],
        json!(1_700_000_040),
        "{jobs}"
    );
    let listed = jobs["persisted"]["jobs"].as_array().unwrap();
    let ids: Vec<u64> = listed.iter().map(|j| j["id"].as_u64().unwrap()).collect();
    assert_eq!(ids, [1, 2, 3], "{jobs}");
    assert_eq!(listed[1]["state"], "running");
    assert_eq!(listed[2]["parent"], 2);
    assert_eq!(listed[0]["params"]["id"], "tab1");
    assert!(
        listed[0].get("deferred").is_none(),
        "bulk columns are not read: {jobs}"
    );

    assert_eq!(
        serde_json::to_value(frontend.daemon().await.unwrap()).unwrap(),
        json!({
            "running": false,
            "persisted": { "waiting": 2, "recorded_running": 1, "written_at": 1_700_000_040 }
        })
    );
    assert!(matches!(frontend.observe().await, Ok(None)));
}

// ---- the apply flow over the mock (C38, C111, C23) -----------------------

/// C111: a plan applied, landed, and applied again sends nothing the
/// second time, and says what a plan compiled now would fetch.
#[tokio::test]
async fn a_plan_applied_twice_is_spent_the_second_time() {
    let s = session("twice");
    let (_daemon, _frontend, mut opened, mut client) = world(&s).await;
    let plan = opened
        .compile(Realm::Pc, "Standard", acquisition_store::now())
        .unwrap();
    assert!(!plan.actions.is_empty(), "{plan:?}");
    let mut slot = None;
    let Apply::Submitted(submitted) = opened.apply(&plan, None, &mut slot).await.unwrap() else {
        panic!("the first apply sent nothing")
    };
    let landed = opened
        .landed(&mut client, &plan, submitted, None)
        .await
        .unwrap();
    assert!(
        matches!(landed.outcome, Outcome::Success { .. }),
        "{landed:?}"
    );
    assert!(landed.store_changes.responses > 0, "{landed:?}");
    let again = opened.apply(&plan, None, &mut slot).await.unwrap();
    let Apply::Spent {
        spent,
        remaining_requests,
        ..
    } = &again
    else {
        panic!("{again:?}")
    };
    assert_eq!(spent.len(), plan.actions.len(), "{again:?}");
    assert!(remaining_requests.is_some(), "{again:?}");
}

/// C111: a plan whose every request a queued apply holds is spent with
/// no remaining count, since what is queued has not landed.
#[tokio::test]
async fn a_plan_held_whole_by_a_queued_job_has_no_remaining_count() {
    let s = session("held");
    let (_daemon, _frontend, mut opened, _client) = world(&s).await;
    let plan = opened
        .compile(Realm::Pc, "Standard", acquisition_store::now())
        .unwrap();
    let mut slot = None;
    let first = opened.apply(&plan, None, &mut slot).await.unwrap();
    let Apply::Submitted(submitted) = first else {
        panic!("{first:?}")
    };
    let again = opened.apply(&plan, None, &mut slot).await.unwrap();
    let Apply::Spent {
        spent,
        remaining_requests,
        note,
        ..
    } = &again
    else {
        panic!("{again:?}")
    };
    assert_eq!(spent.len(), plan.actions.len(), "{again:?}");
    assert_eq!(*remaining_requests, None, "{again:?}");
    assert!(
        note.contains(&format!("job {}", submitted.job_id)),
        "{note}"
    );
}

/// C23: a parent whose child failed carries the tally as a field, read
/// back as the daemon wrote it.
#[tokio::test]
async fn a_parent_whose_child_failed_carries_its_childrens_tally() {
    let s = session("children");
    let (_daemon, frontend, mut opened, mut client) = world(&s).await;
    let mut plan = opened
        .compile(Realm::Pc, "Standard", acquisition_store::now())
        .unwrap();
    let total = plan.actions.len() as u64;
    let bogus = plan
        .actions
        .iter_mut()
        .find_map(|a| match a {
            RefreshAction::FetchTab { id, .. } => Some(id),
            _ => None,
        })
        .expect("a tab to fetch");
    *bogus = "no-such-tab".into();
    // The quote names each job it priced; a plan edited after it carries
    // none.
    plan.quote = None;
    let mut slot = None;
    let Apply::Submitted(submitted) = opened.apply(&plan, None, &mut slot).await.unwrap() else {
        panic!("nothing sent")
    };
    let landed = opened
        .landed(&mut client, &plan, submitted, None)
        .await
        .unwrap();
    let Outcome::Failure { children, .. } = &landed.outcome else {
        panic!("{landed:?}")
    };
    let children = children.as_ref().expect("the tally as a field");
    assert_eq!((children.failed, children.failed_ids.len()), (1, 1));
    assert_eq!(children.done + children.failed + children.cancelled, total);

    // The failed child, detailed in the one answer: live, with its kind,
    // target and own error ...
    let child = children.failed_ids[0];
    assert_eq!(landed.failed_children.len(), 1, "{landed:?}");
    let live = frontend.result(landed.job_id).await.unwrap();
    assert_eq!(live.failed_children, landed.failed_children);
    let row = &live.failed_children[0];
    assert_eq!((row.job_id, row.kind.as_str()), (child, "stash"), "{row:?}");
    assert!(row.target.contains("no-such-tab"), "{row:?}");
    assert!(
        !row.error.is_empty() && row.error != "no result retained for this job",
        "{row:?}"
    );
    // ... and from the ledger once the daemon has stopped, the same (C45:
    // the ledger prints as the daemon would).
    Client::stop_any()
        .await
        .expect("a stop")
        .expect("a daemon acknowledged it");
    let deadline = Instant::now() + Duration::from_secs(10);
    while !matches!(frontend.observe().await, Ok(None)) {
        assert!(Instant::now() < deadline, "the daemon did not stop");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let from_disk = frontend.result(landed.job_id).await.unwrap();
    assert_eq!(from_disk.failed_children, live.failed_children);
}

/// The quote over a running daemon is the daemon's: no note.
#[tokio::test]
async fn a_running_daemon_quotes_the_plan() {
    let s = session("quoted");
    let (_daemon, frontend, opened, _client) = world(&s).await;
    let plan = opened
        .compile(Realm::Pc, "Standard", acquisition_store::now())
        .unwrap();
    let planned = quote(&frontend.root, plan).await;
    assert_eq!(planned.quote_note, None, "{:?}", planned.quote_note);
    assert!(planned.plan.quote.is_some());
}

/// A parent's failure with its children's tally, as the daemon writes one.
fn parent_failed(id: u64, failed_ids: &[u64]) -> JobRow {
    JobRow {
        kind: "apply".into(),
        params: json!({ "jobs": [] }),
        outcome: Some(json!({ "outcome": "failure",
            "error": format!("{} of {} child jobs failed, 0 done", failed_ids.len(), failed_ids.len()),
            "children": { "done": 0, "failed": failed_ids.len(), "cancelled": 0, "failed_ids": failed_ids } })),
        ..row(id, "failed", None)
    }
}

/// "No result retained" is said of confirmed absence alone: from the
/// ledger, a child with no row is absent, one whose row records no
/// outcome or an outcome that does not parse says so, and each keeps
/// its handle.
#[tokio::test]
async fn a_failed_childs_unreadable_result_says_why_and_absence_alone_is_no_result() {
    let s = session("child-reads");
    ledger(
        &s,
        &[
            parent_failed(10, &[11, 12, 13]),
            // 11: no row at all
            JobRow {
                parent: Some(10),
                ..row(12, "failed", None)
            },
            JobRow {
                parent: Some(10),
                ..row(13, "failed", Some(json!({ "outcome": "nonsense" })))
            },
        ],
    );
    let frontend = Frontend::agent("test").unwrap();
    let result = frontend.result(10).await.unwrap();
    let rows: Vec<(u64, &str)> = result
        .failed_children
        .iter()
        .map(|c| (c.job_id, c.error.as_str()))
        .collect();
    assert_eq!(rows.len(), 3, "{rows:?}");
    assert_eq!(rows[0], (11, NO_RESULT), "{rows:?}");
    assert_eq!(
        rows[1],
        (12, "its row on disk records no outcome yet"),
        "{rows:?}"
    );
    assert_eq!(rows[2].0, 13, "{rows:?}");
    assert!(
        rows[2]
            .1
            .starts_with("its outcome on disk does not parse: ")
            && rows[2].1.len() > 36,
        "{rows:?}"
    );
}

/// Live, the daemon's word for an id it does not know is confirmed
/// absence; the child keeps its handle.
#[tokio::test]
async fn a_failed_child_the_daemon_does_not_know_is_no_result_live() {
    let s = session("child-unknown");
    let (_daemon, _frontend, _opened, mut client) = world(&s).await;
    let outcome = Outcome::Failure {
        error: "1 of 1 child jobs failed, 0 done".into(),
        children: Some(acquisition_protocol::job::Children {
            done: 0,
            failed: 1,
            cancelled: 0,
            failed_ids: vec![987_654],
        }),
        payload: None,
    };
    let rows = acquisition_verbs::jobs::failed_children_live(&mut client, &outcome).await;
    assert_eq!(rows.len(), 1);
    assert_eq!(
        (rows[0].job_id, rows[0].error.as_str()),
        (987_654, NO_RESULT)
    );
    assert_eq!((rows[0].kind.as_str(), rows[0].target.as_str()), ("", ""));
}

// ---- the frontend's world ----------------------------------------------

/// C147, the printed start command names the frontend's world: a mock
/// frontend over the default root, built while the environment names
/// another world (the session's scratch world), prints a command that
/// names the default root as `ACQ_STORE_DIR`. Copied into that
/// environment it starts the daemon in the frontend's world, not the
/// environment's.
#[test]
fn c147_the_printed_start_command_names_the_default_world_too() {
    let s = session("printed-world");
    let a = s.base.join("store");
    assert_eq!(
        intended_root(),
        a,
        "the environment names the scratch world"
    );
    let frontend = Frontend {
        via: "test",
        door: acquisition_verbs::Door::Agent,
        provider: "mock",
        root: default_root(),
        journal: acquisition_verbs::frontend::JournalKnob::Default,
    };
    let command = frontend.start_command();
    let root = acquisition_search::answer::shell_quoted(&default_root().display().to_string());
    assert!(
        command.starts_with(&format!("ACQ_PROVIDER=mock ACQ_STORE_DIR={root} ")),
        "the command does not name the frontend's world: {command}"
    );
}

/// C147, a door and a ledger read take the world they act on: a
/// frontend built over world B while the environment names world A
/// spawns its daemon in B, and reads the facts, the intent, the quote,
/// the daemon's report and the ledger there; A is never created, and the
/// environment's own door finds no daemon in it.
#[tokio::test]
async fn c147_a_frontend_over_another_world_acts_in_its_world_not_the_environments() {
    let s = session("two-worlds");
    harness::link_beside_this_test(&acqd_built());
    let a = s.base.join("store");
    std::fs::remove_dir_all(&a).expect("the session's world, emptied of its root");
    let b = s.base.join("b");
    let frontend = Frontend {
        via: "test",
        door: acquisition_verbs::Door::Agent,
        provider: "mock",
        root: b.clone(),
        journal: acquisition_verbs::frontend::JournalKnob::Default,
    };

    // The door spawns the daemon in B, and the daemon serves B.
    let mut client = frontend.connect().await.expect("the use door over B");
    assert!(client.spawned(), "the door started the daemon");
    // B as `hello` carries it (C83): the store's canonical root, the
    // verbatim prefix dropped on Windows, never a bare `canonicalize()`.
    let world_b = World::observe_at(&b).expect("the door created B");
    assert_eq!(client.daemon().world(), world_b.name());
    login_in_process(&mut client, Some("alice")).await;
    let id = submit(
        &frontend,
        &mut client,
        "stashes",
        json!({ "realm": "pc", "league": "Standard" }),
        0,
        None,
    )
    .await
    .expect("a submit");
    let listed = wait(&mut client, id, None).await.expect("the listing");
    assert!(matches!(listed, Outcome::Success { .. }), "{listed:?}");

    // Facts and intent in B.
    let account = frontend.account(None).expect("alice, in B's index");
    account
        .set_policy(
            &json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard":
                { "tabs": "all", "max_age_seconds": 3600 } } } } }),
            Expect::Create,
            PolicyWrite::Replace,
        )
        .expect("the policy, in B's intent");
    let mut opened = account.open().expect("B's stores");
    let snapshot = opened
        .facts
        .refresh_snapshot("pc", "Standard", &opened.intent)
        .unwrap();
    assert!(
        snapshot.stash_listing.is_some(),
        "the listing B's daemon fetched"
    );
    assert!(snapshot.policy.is_some(), "the policy written to B");
    // The quote asks B's daemon.
    let planned = opened.plan(Realm::Pc, "Standard").await.unwrap();
    assert_eq!(planned.quote_note, None, "{:?}", planned.quote_note);
    // The daemon's report is B's daemon.
    match frontend.daemon().await.expect("the report") {
        acquisition_verbs::DaemonReport::Running { identity, .. } => {
            assert_eq!(identity.pid(), client.daemon().pid())
        }
        other => panic!("B's daemon is running: {other:?}"),
    }
    // The environment's world holds nothing: no daemon, no root.
    assert!(
        matches!(
            Client::observe().await.expect("observe A"),
            Observed::Absent
        ),
        "a daemon answered in the environment's world"
    );
    assert!(!a.exists(), "the environment's world was created");

    // Stopped, B's ledger answers from disk, and B's log is named.
    drop(client);
    Client::stop_any_at(&b)
        .await
        .expect("a stop")
        .expect("B's daemon acknowledged it");
    let deadline = Instant::now() + Duration::from_secs(60);
    while !matches!(frontend.observe().await, Ok(None)) {
        assert!(Instant::now() < deadline, "the daemon did not stop");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let from_disk = frontend
        .result_on_disk(id)
        .expect("the listing, in B's ledger");
    assert!(
        matches!(from_disk.outcome, Outcome::Success { .. }),
        "{from_disk:?}"
    );
    assert!(frontend.persisted().expect("B's ledger").is_some());
    match frontend.daemon().await.expect("the report") {
        acquisition_verbs::DaemonReport::Absent { persisted, log } => {
            assert!(persisted.is_some(), "B's ledger counted");
            let log = log.expect("B's daemon log");
            let world = acquisition_store::world::World::observe_at(&b).unwrap();
            assert_eq!(log, world.log_path("mock"));
        }
        other => panic!("nothing runs: {other:?}"),
    }
    assert!(!a.exists(), "the environment's world was created");
}

// ---- the diagnosis (#79) ------------------------------------------------

/// What the redaction test plants, each a string no printed diagnosis may
/// hold: an account's name and uuid, two tokens in the log, and a second
/// account the index never knew, named only by the journal's route key.
const PLANTED_USER: &str = "Alice#1234";
const PLANTED_UUID: &str = "u-diagnose-7d1e";
const PLANTED_OTHER: &str = "Bob#5678";
const PLANTED_ACCESS: &str = "sentinel-access-55d0";
const PLANTED_BEARER: &str = "sentinel-bearer-91c4";
/// The review's two (r4, finding 1): a quoted value after `=`, and a
/// JSON body's spacing.
const PLANTED_QUOTED: &str = "sentinel-refresh-q1";
const PLANTED_SPACED: &str = "sentinel-code-q2";
/// Their twins with tabs (the fix-round review, finding 1).
const PLANTED_TABBED: &str = "sentinel-code-t1";
const PLANTED_TABBED_BEARER: &str = "sentinel-bearer-t2";

/// #79: with no daemon, every section still reads — the ledger's
/// picture, the store's counts read-only, the world's rails file, the
/// journal and the log the last daemon left — and nothing planted
/// survives into the document: the account's name in the index, the
/// store file and the log, the journal's route keys (each account its
/// own placeholder), the store root and the home directory written into
/// the log, and six tokens, four of them in the reviews' shapes. A torn
/// last journal line is reported, never pasted.
#[tokio::test]
async fn a_diagnosis_with_no_daemon_reads_every_section_and_holds_nothing_planted() {
    let s = session("diagnose-absent");
    // The isolation list turns journals off; unset, the world's default
    // journal is the one read.
    s.set_knob("ACQ_JOURNAL", None);
    let frontend = Frontend::agent("test").unwrap();
    let mock = s.base.join("store").join("mock");
    acquisition_store::Index::load(&mock)
        .unwrap()
        .record_login(PLANTED_USER, PLANTED_UUID, false, 1_000, &[])
        .unwrap();
    let store_file = acquisition_store::account_path(&mock, PLANTED_USER);
    acquisition_store::Store::open(&store_file)
        .unwrap()
        .record(
            &acquisition_store::Endpoint::Profile,
            &json!({}),
            200,
            &json!({ "uuid": PLANTED_UUID, "name": PLANTED_USER }),
            1_000,
        )
        .unwrap();
    let world = World::observe_at(&s.base.join("store")).unwrap();
    let home = std::env::home_dir().expect("a home directory");
    let log = world.log_path("mock");
    std::fs::create_dir_all(log.parent().unwrap()).unwrap();
    std::fs::write(
        &log,
        format!(
            "[    0s] world: {}\n[    5s] store: {} opened for {PLANTED_USER}\n\
             [    5s] logged in as {PLANTED_USER} ({PLANTED_UUID})\n\
             [    6s] GET /oauth/token?access_token={PLANTED_ACCESS} -> 403\n\
             [    6s] Authorization: Bearer {PLANTED_BEARER}\n\
             [    6s] refresh_token=\"{PLANTED_QUOTED}\"\n\
             [    6s] body {{\"code\": \"{PLANTED_SPACED}\"}}\n\
             [    6s] body {{\"code\":\t\"{PLANTED_TABBED}\"}}\n\
             [    6s] Authorization: Bearer\t{PLANTED_TABBED_BEARER}\n\
             [    7s] config read from {}\n",
            world.root().display(),
            store_file.display(),
            home.join(".config").display(),
        ),
    )
    .unwrap();
    let send = |account: &str| {
        json!({ "method": "GET", "route": format!("stash@{account}"), "path": "/stash/Standard/a1",
                "status": 200, "counted": true, "wait_ms": 0 })
        .to_string()
    };
    std::fs::write(
        world.journal_path("mock"),
        format!(
            "{}\n{}\n{{\"route\":\"stash@{PLANTED_USER}",
            send(PLANTED_USER),
            send(PLANTED_OTHER)
        ),
    )
    .unwrap();

    let document = frontend
        .diagnose(acquisition_verbs::Tails::DEFAULT)
        .await
        .document();
    let printed = document.to_string();
    let bare_home = home.display().to_string();
    for planted in [
        PLANTED_USER,
        "Alice",
        "Alice_1234",
        PLANTED_UUID,
        PLANTED_OTHER,
        PLANTED_ACCESS,
        PLANTED_BEARER,
        PLANTED_QUOTED,
        PLANTED_SPACED,
        PLANTED_TABBED,
        PLANTED_TABBED_BEARER,
        &world.root().display().to_string(),
        &bare_home,
    ] {
        assert!(
            !printed.contains(planted),
            "the diagnosis holds {planted:?}, which the test planted in the index, the store \
             file's name, the log (the world root, the store file, the home directory, an \
             access token, a bearer token, a quoted refresh token, a JSON body's code, and \
             the last two again with tabs) and the journal's route keys: {document:#}"
        );
    }

    assert_eq!(document["daemon"]["running"], json!(false), "{document:#}");
    assert_eq!(document["store"][0]["account"], json!("<account 1>"));
    assert_eq!(document["store"][0]["status"]["responses"], json!(1));
    assert_eq!(
        (
            document["rails"]["from"].clone(),
            document["rails"]["state"].clone()
        ),
        (json!("disk"), json!(null)),
        "no rails file is nothing held: {document:#}"
    );
    assert_eq!(document["journal"]["from"], json!("world"), "{document:#}");
    let journal = &document["journal"]["lines"];
    assert_eq!(
        (
            journal[0]["route"].clone(),
            journal[1]["route"].clone(),
            journal[0]["path"].clone()
        ),
        (
            json!("stash@<account 1>"),
            json!("stash@<account 2>"),
            json!("/stash/Standard/a1")
        ),
        "each account its own placeholder, the rest of the line kept: {document:#}"
    );
    assert!(
        journal[2]["unread"]
            .as_str()
            .is_some_and(|why| why.contains("not JSON")),
        "the torn line reported: {document:#}"
    );
    let log_lines = document["log"]["lines"].as_array().unwrap();
    assert_eq!(log_lines.len(), 10, "{document:#}");
    assert_eq!(
        log_lines[1],
        json!("[    5s] store: <store>/mock/<account 1>.db opened for <account 1>")
    );
    assert_eq!(
        log_lines[2],
        json!("[    5s] logged in as <account 1> (<account 1 uuid>)")
    );
    assert_eq!(log_lines[5], json!(r#"[    6s] refresh_token="<token 3>""#));
    assert_eq!(
        log_lines[6],
        json!(r#"[    6s] body {"code": "<token 4>"}"#)
    );
    assert_eq!(
        log_lines[7],
        json!("[    6s] body {\"code\":\t\"<token 5>\"}")
    );
    assert_eq!(
        log_lines[8],
        json!("[    6s] Authorization: Bearer\t<token 6>")
    );
    assert_eq!(log_lines[9], json!("[    7s] config read from ~/.config"));
    assert_eq!(
        document["redactions"]["tokens"],
        json!(6),
        "the six planted tokens: {document:#}"
    );
}

/// #79: with this client's daemon running, the daemon section is its
/// status document, the rails are its report, the log is the one it
/// opened, and the journal it does not keep (the isolation list's
/// `ACQ_JOURNAL=0`) says so in its place; the logged-in name is in none.
#[tokio::test]
async fn a_diagnosis_with_the_daemon_running_reads_its_report() {
    let s = session("diagnose-running");
    let _daemon = start_daemon_in_process(&s, &acqd_built()).await;
    let frontend = Frontend::agent("test").unwrap();
    let mut client = frontend.connect().await.expect("the daemon");
    login_in_process(&mut client, Some("alice")).await;

    let document = frontend
        .diagnose(acquisition_verbs::Tails::both(5))
        .await
        .document();
    assert_eq!(
        (
            document["daemon"]["running"].clone(),
            document["daemon"]["compatible"].clone(),
            document["rails"]["from"].clone()
        ),
        (json!(true), json!(true), json!("daemon")),
        "{document:#}"
    );
    assert!(
        document["journal"]["unread"]
            .as_str()
            .is_some_and(|why| why.contains("ACQ_JOURNAL=0")),
        "{document:#}"
    );
    let log = document["log"]["lines"]
        .as_array()
        .expect("the daemon's log");
    assert!(!log.is_empty() && log.len() <= 5, "{document:#}");
    assert_eq!(document["store"][0]["account"], json!("<account 1>"));
    assert!(
        !document.to_string().contains("alice"),
        "the logged-in name `alice` is in the diagnosis: {document:#}"
    );
}

/// r4's review, finding 5: an index entry with an empty uuid made an
/// empty needle, which matched at every position and advanced none, so
/// the diagnosis never ended. It ends, and an empty string is replaced
/// nowhere. The work runs on a thread watched from here: a scan that
/// does not end is a busy loop no async timeout can interrupt.
#[tokio::test]
async fn an_index_entry_with_an_empty_uuid_does_not_stall_the_diagnosis() {
    let s = session("diagnose-empty-uuid");
    let mock = s.base.join("store").join("mock");
    acquisition_store::Index::load(&mock)
        .unwrap()
        .record_login(PLANTED_USER, "", false, 1_000, &[])
        .unwrap();
    let diagnosis = Frontend::agent("test")
        .unwrap()
        .diagnose(acquisition_verbs::Tails::DEFAULT)
        .await;
    let (sent, received) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let document = diagnosis.document();
        let (text, _) = diagnosis.redact_text("\nfirst line\n");
        let _ = sent.send((document, text));
    });
    let (document, text) = received
        .recv_timeout(Duration::from_secs(60))
        .expect("the redaction of a diagnosis whose index holds an empty uuid never ended");
    assert_eq!(text, "\nfirst line\n");
    assert_eq!(document["store"][0]["account"], json!("<account 1>"));
}

/// r4's review, finding 3: a peer that answers the handshake as this
/// world's daemon and then never its status held the diagnosis forever,
/// every section behind it. The status request is bounded by the
/// handshake's deadline (`ACQ_HELLO_WITHIN`, here 1 s); its expiry is
/// forced by a peer that never answers (C137, the deadline's contract is
/// its expiry behaviour), and the rest is read from disk. The peer
/// replays the hello a real daemon of this world gave, so the observe
/// door finds it compatible.
#[tokio::test]
async fn a_peer_that_answers_hello_and_not_its_status_leaves_the_rest_to_the_disk() {
    use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt as _, BufReader};

    let s = session("diagnose-stalled");
    let daemon = start_daemon_in_process(&s, &acqd_built()).await;
    let world = World::observe().unwrap();
    let socket = world.socket_path().unwrap();
    let hello = {
        let mut line = serde_json::to_string(&acquisition_protocol::protocol::Bootstrap::Hello {
            version: acquisition_protocol::VERSION.to_string(),
            contract: acquisition_protocol::CONTRACT_REVISION.to_string(),
            world: world.name(),
        })
        .unwrap();
        line.push('\n');
        line
    };
    let reply = {
        let stream = acquisition_client::endpoint::connect(&socket)
            .await
            .unwrap();
        let (read, mut write) = tokio::io::split(stream);
        write.write_all(hello.as_bytes()).await.unwrap();
        let mut reply = String::new();
        BufReader::new(read).read_line(&mut reply).await.unwrap();
        reply
    };
    drop(daemon);

    let mut listener = acquisition_client::endpoint::Listener::bind(&socket).unwrap();
    tokio::spawn(async move {
        let stream = listener.accept().await.unwrap();
        let (read, mut write) = tokio::io::split(stream);
        let mut lines = BufReader::new(read).lines();
        let _hello = lines.next_line().await.unwrap();
        write.write_all(reply.as_bytes()).await.unwrap();
        let _status = lines.next_line().await.unwrap();
        // Holds the connection, and never answers.
        std::future::pending::<()>().await;
        drop(write);
    });
    s.set_knob("ACQ_HELLO_WITHIN", Some("1"));
    let frontend = Frontend::agent("test").unwrap();

    // The watchdog outlives the tested bound: without the bound this
    // fails instead of hanging the gate.
    let document = tokio::time::timeout(
        Duration::from_secs(60),
        frontend.diagnose(acquisition_verbs::Tails::DEFAULT),
    )
    .await
    .expect("the diagnosis waited on a peer that never answers its status")
    .document();
    assert!(
        document["daemon"]["unread"]
            .as_str()
            .is_some_and(|why| why.contains("answered the handshake and then not its status")),
        "{document:#}"
    );
    assert_eq!(document["rails"]["from"], json!("disk"), "{document:#}");
    assert!(
        document["log"]["lines"]
            .as_array()
            .is_some_and(|lines| !lines.is_empty()),
        "the last daemon's log, read from disk: {document:#}"
    );
}

/// r4's review, finding 4: with no daemon, the journal read is the one a
/// daemon started from this frontend's environment writes — the path
/// `ACQ_JOURNAL` names, or none when it is `0` — read once when the
/// frontend is built (C147), never the world's default in their place.
#[tokio::test]
async fn with_no_daemon_the_journal_read_is_the_one_acq_journal_names() {
    let s = session("diagnose-journal-knob");
    let elsewhere = s.base.join("runs").join("sends.jsonl");
    std::fs::create_dir_all(elsewhere.parent().unwrap()).unwrap();
    std::fs::write(
        &elsewhere,
        "{\"method\":\"GET\",\"route\":\"leagues@Alice#1234\",\"status\":200}\n",
    )
    .unwrap();
    let world = World::observe_at(&s.base.join("store")).unwrap();
    std::fs::create_dir_all(world.log_dir("mock")).unwrap();
    std::fs::write(
        world.journal_path("mock"),
        "{\"route\":\"stash@Other#1\"}\n",
    )
    .unwrap();

    s.set_knob("ACQ_JOURNAL", Some(elsewhere.to_str().unwrap()));
    let document = Frontend::agent("test")
        .unwrap()
        .diagnose(acquisition_verbs::Tails::DEFAULT)
        .await
        .document();
    assert_eq!(
        (
            document["journal"]["from"].clone(),
            document["journal"]["lines"][0]["route"].clone()
        ),
        (json!("override"), json!("leagues@<account 1>")),
        "the journal ACQ_JOURNAL names, not the world's default: {document:#}"
    );

    s.set_knob("ACQ_JOURNAL", Some("0"));
    let document = Frontend::agent("test")
        .unwrap()
        .diagnose(acquisition_verbs::Tails::DEFAULT)
        .await
        .document();
    assert!(
        document["journal"]["unread"]
            .as_str()
            .is_some_and(|why| why.contains("ACQ_JOURNAL=0") && why.contains("journals are off")),
        "journals off in this shell, and the world's default left unread: {document:#}"
    );
}

/// r4's review, finding 2: the path needles were this shell's own log
/// directory, so a daemon started under another `ACQ_LOG_DIR` and with
/// `ACQ_JOURNAL` elsewhere had both reported paths printed whole. Every
/// diagnostic location the document prints is a needle by its directory:
/// the log as `<logs>`, the journal as `<journal>`.
#[tokio::test]
async fn a_daemon_logging_elsewhere_has_its_reported_paths_replaced() {
    let s = session("diagnose-elsewhere");
    let private_logs = s.base.join("private-logs");
    let private_runs = s.base.join("private-runs");
    std::fs::create_dir_all(&private_runs).unwrap();
    let journal = private_runs.join("sends.jsonl");
    let acqd = acqd_built();
    harness::link_beside_this_test(&acqd);
    let mut cmd = harness::daemon_command(&acqd, &s.base);
    cmd.env("ACQ_LOG_DIR", &private_logs)
        .env("ACQ_JOURNAL", &journal);
    let (_daemon, said) = harness::spawn_drained(cmd, &acqd);
    let frontend = Frontend::agent("test").unwrap();
    let deadline = Instant::now() + Duration::from_secs(120);
    let mut client = loop {
        if let Ok(Some(client)) = frontend.observe().await {
            break client;
        }
        assert!(
            Instant::now() < deadline,
            "the daemon did not come up: {}",
            String::from_utf8_lossy(&said.lock().unwrap())
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    };
    login_in_process(&mut client, Some("alice")).await;

    let document = frontend
        .diagnose(acquisition_verbs::Tails::both(5))
        .await
        .document();
    let printed = document.to_string();
    for planted in [&private_logs, &private_runs] {
        let spellings = [
            planted.display().to_string(),
            planted.canonicalize().unwrap().display().to_string(),
        ];
        for spelling in spellings {
            assert!(
                !printed.contains(&spelling),
                "the diagnosis prints {spelling}, where the test started the daemon's log \
                 (ACQ_LOG_DIR) or its journal (ACQ_JOURNAL): {document:#}"
            );
        }
    }
    let shown = |path: &serde_json::Value| path.as_str().unwrap().replace('\\', "/");
    assert_eq!(shown(&document["log"]["path"]), "<logs>/mock/daemon.log");
    assert_eq!(shown(&document["journal"]["path"]), "<journal>/sends.jsonl");
    assert_eq!(document["log"]["from"], json!("daemon"));
}

/// r4's fix-round review, finding 2: a daemon that could not open its
/// journal reports `NOT WRITTEN — journal <path> could not be opened`,
/// which the diagnosis opened as a path and whose directory it never
/// replaced. From the shell whose `ACQ_JOURNAL` named it, the journal
/// section is unread in the daemon's words and the directory is
/// `<journal>` wherever it is printed. The journal's directory is a file
/// here, so no platform can create it.
#[tokio::test]
async fn a_journal_the_daemon_could_not_open_is_unread_in_its_words_and_its_directory_replaced() {
    let s = session("diagnose-not-written");
    let blocked = s.base.join("private-runs");
    std::fs::write(&blocked, "a file where the journal's directory would be").unwrap();
    let journal = blocked.join("sends.jsonl");
    s.set_knob("ACQ_JOURNAL", Some(journal.to_str().unwrap()));
    let acqd = acqd_built();
    harness::link_beside_this_test(&acqd);
    let mut cmd = harness::daemon_command(&acqd, &s.base);
    cmd.env("ACQ_JOURNAL", &journal);
    let (_daemon, said) = harness::spawn_drained(cmd, &acqd);
    let frontend = Frontend::agent("test").unwrap();
    let deadline = Instant::now() + Duration::from_secs(120);
    while !matches!(frontend.observe().await, Ok(Some(_))) {
        assert!(
            Instant::now() < deadline,
            "the daemon did not come up: {}",
            String::from_utf8_lossy(&said.lock().unwrap())
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    let document = frontend
        .diagnose(acquisition_verbs::Tails::DEFAULT)
        .await
        .document();
    assert!(
        document["daemon"]["rails"]["journal"]
            .as_str()
            .is_some_and(|words| words.starts_with("NOT WRITTEN")),
        "the daemon reports the journal it could not open: {document:#}"
    );
    assert!(
        document["journal"]["unread"]
            .as_str()
            .is_some_and(|why| why.starts_with("the daemon running reports: NOT WRITTEN")),
        "the journal section is the daemon's words, nothing opened: {document:#}"
    );
    let printed = document.to_string();
    for spelling in [
        blocked.display().to_string(),
        blocked.canonicalize().unwrap().display().to_string(),
    ] {
        assert!(
            !printed.contains(&spelling),
            "the diagnosis prints {spelling}, the directory the test's ACQ_JOURNAL named: \
             {document:#}"
        );
    }
}
