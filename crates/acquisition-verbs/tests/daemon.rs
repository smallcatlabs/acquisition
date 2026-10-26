//! The layer against a daemon, or against its absence (C10, C45, C111,
//! C23): the quote with no daemon and with a wedged peer, the ledger's
//! answers when no daemon runs, and the apply flow over the mock daemon —
//! applied twice, held by a queued job, a parent whose child failed.
//!
//! The harness is the one every crate's process tests share (C122),
//! `acquisition-client/tests/harness/mod.rs`, included by `#[path]`: one
//! lock and the one isolation list applied to this process (`session`),
//! the `acqd` the gate built linked beside this test executable
//! (`acqd_built`, `start_daemon_in_process`), and a login through the
//! mock's approve page. This file sets the process environment, so it is
//! its own test binary beside `documents.rs`. Run `cargo build
//! --workspace` first: a `-p` test run does not build the daemon. Nothing
//! here reaches GGG: the daemon runs the mock provider.

#[path = "../../acquisition-client/tests/harness/mod.rs"]
mod harness;

use std::time::{Duration, Instant};

use acquisition_client::client::{Client, Observed};
use acquisition_plan::pricing::Expect;
use acquisition_plan::{PolicyWrite, RefreshAction};
use acquisition_protocol::job::Outcome;
use acquisition_protocol::realm::Realm;
use acquisition_store::jobs::{JobDb, JobRow, daemon_db_path};
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
        serde_json::to_value(frontend.jobs().await.unwrap()).unwrap(),
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
    let jobs = serde_json::to_value(frontend.jobs().await.unwrap()).unwrap();
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
    };

    // The door spawns the daemon in B, and the daemon serves B.
    let mut client = frontend.connect().await.expect("the use door over B");
    assert!(client.spawned(), "the door started the daemon");
    let canonical_b = b.canonicalize().expect("the door created B");
    assert_eq!(client.daemon().world(), canonical_b.to_str().unwrap());
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
