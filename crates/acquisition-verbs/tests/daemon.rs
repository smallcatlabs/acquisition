//! The layer against a daemon, or against its absence (C10, C45, C111,
//! C23): the quote with no daemon and with a wedged peer, the ledger's
//! answers when no daemon runs, and the apply flow over the mock daemon —
//! applied twice, held by a queued job, a parent whose child failed.
//!
//! The harness is the client's contract tests' (`acquisition-client/
//! tests/contract.rs`), copied, not shared: one lock and a scratch world
//! set in the environment under it (`session`), the `acqd` the gate built
//! linked beside this test executable (`acqd_for_tests`, `place_link`),
//! `start_daemon`, and a login through the mock's approve page. Run
//! `cargo build --workspace` first: a `-p` test run does not build the
//! daemon. Nothing here reaches GGG: `ACQ_PROVIDER=mock` is set.

use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{Duration, Instant};

use acquisition_client::client::{Client, Observed};
use acquisition_plan::pricing::Expect;
use acquisition_plan::{PolicyWrite, RefreshAction};
use acquisition_protocol::job::Outcome;
use acquisition_protocol::protocol::{Request, Response};
use acquisition_protocol::realm::Realm;
use acquisition_store::jobs::{JobDb, JobRow, daemon_db_path};
use acquisition_verbs::jobs::{NO_RESULT, submit, wait};
use acquisition_verbs::refresh::{quote, quote_within};
use acquisition_verbs::{Apply, Fault, Frontend, Job, Opened};
use serde_json::json;

// ---- the harness, copied from the client's contract tests ---------------

/// The `acqd` the gate built: `target/<profile>/acqd`, one level above the
/// `deps/` directory this test executable runs from, linked beside it so
/// the client finds the same file as its sibling (C84).
fn acqd_for_tests() -> PathBuf {
    let exe = std::env::current_exe().expect("current exe");
    let deps_dir = exe.parent().expect("a deps directory");
    let profile_dir = deps_dir.parent().expect("a profile directory");
    let acqd = profile_dir.join(acquisition_client::locator::ACQD);
    assert!(
        acqd.is_file(),
        "no daemon at {}: run `cargo build --workspace` first (a `-p` test run does not build it)",
        acqd.display()
    );
    place_link(&deps_dir.join(acquisition_client::locator::ACQD), &acqd);
    acqd
}

/// The link beside the test executable (C82's clause for a test
/// executable): a symlink one level up on Unix, a hard link on Windows.
#[cfg_attr(unix, allow(unused_variables))]
fn place_link(link: &Path, acqd: &Path) {
    #[cfg(unix)]
    {
        let target = Path::new("..").join(acquisition_client::locator::ACQD);
        if std::fs::read_link(link).ok().as_deref() != Some(target.as_path()) {
            let _ = std::fs::remove_file(link);
            if let Err(e) = std::os::unix::fs::symlink(&target, link)
                && std::fs::read_link(link).ok().as_deref() != Some(target.as_path())
            {
                panic!("placing {} -> {}: {e}", link.display(), target.display());
            }
        }
    }
    #[cfg(windows)]
    {
        let same = |a: &Path, b: &Path| {
            let id = |p: &Path| {
                std::fs::File::open(p)
                    .and_then(|f| acquisition_store::world::file_id(&f))
                    .ok()
            };
            id(a).is_some() && id(a) == id(b)
        };
        if !same(link, acqd) {
            let _ = std::fs::remove_file(link);
            if let Err(e) = std::fs::hard_link(acqd, link)
                && !same(link, acqd)
            {
                panic!("linking {} to {}: {e}", link.display(), acqd.display());
            }
        }
    }
}

static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());
static ORIGINAL_TMP: OnceLock<PathBuf> = OnceLock::new();

/// A scratch store, temp and runtime directory, held for the test's
/// duration: the process environment points at them while the lock is
/// held.
struct Session {
    _lock: MutexGuard<'static, ()>,
    base: PathBuf,
}

fn session(tag: &str) -> Session {
    let lock = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let original = ORIGINAL_TMP.get_or_init(std::env::temp_dir);
    let base = original.join(format!("acq-v{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(base.join("store")).expect("scratch dir");
    std::fs::create_dir_all(base.join("tmp")).expect("scratch dir");
    // SAFETY: every test in this binary takes `ONE_AT_A_TIME` before
    // touching the environment or the client.
    unsafe {
        std::env::set_var("ACQ_STORE_DIR", base.join("store"));
        std::env::set_var("ACQ_LOG_DIR", base.join("logs"));
        std::env::set_var("TMPDIR", base.join("tmp"));
        std::env::set_var("XDG_RUNTIME_DIR", base.join("tmp"));
        std::env::set_var("ACQ_NO_KEYRING", "1");
        std::env::set_var("ACQ_JOURNAL", "0");
        std::env::set_var("ACQ_IDLE_SHUTDOWN", "30");
        for var in [
            "ACQ_GGG",
            "ACQ_ACCOUNT",
            "ACQ_TRIPWIRE",
            "ACQ_MAX_SENDS",
            "ACQ_NO_SPAWN",
        ] {
            std::env::remove_var(var);
        }
        std::env::set_var("ACQ_PROVIDER", "mock");
    }
    acquisition_store::world::app_runtime_dir().expect("the scratch runtime directory");
    Session { _lock: lock, base }
}

impl Drop for Session {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.base);
    }
}

/// The mock daemon, killed on drop.
struct Daemon(Child);

impl Drop for Daemon {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

async fn start_daemon(s: &Session) -> Daemon {
    let acqd = acqd_for_tests();
    let child = Command::new(&acqd)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap_or_else(|e| panic!("spawning {}: {e}", acqd.display()));
    let daemon = Daemon(child);
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        match Client::observe().await {
            Ok(Observed::Compatible(_)) => return daemon,
            Ok(Observed::Incompatible(found)) => {
                panic!("{} is not this build's daemon: {found}", acqd.display())
            }
            _ => {}
        }
        assert!(
            Instant::now() < deadline,
            "the daemon did not come up on {}",
            s.base.display()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// Log in through the mock's approve page as `user` and wait for the
/// daemon to register the session.
async fn login(client: &mut Client, user: &str) {
    let url = match client.request(&Request::AuthStart).await.unwrap() {
        Response::AuthUrl { authorize_url } => authorize_url,
        other => panic!("{other:?}"),
    };
    let approve = format!(
        "{}&user={}",
        url.replacen("/authorize?", "/approve?", 1),
        user.replace('#', "%23")
    );
    let response = reqwest::get(&approve).await.expect("approve");
    assert!(response.status().is_success(), "{}", response.status());
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        match client.request(&Request::AuthStatus).await.unwrap() {
            Response::Auth {
                pending: false,
                login_ok: Some(_),
                ..
            } => return,
            Response::Auth {
                login_error: Some(e),
                pending: false,
                ..
            } => panic!("login failed: {e}"),
            _ => {}
        }
        assert!(Instant::now() < deadline, "login did not complete");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

// ---- the fixtures -------------------------------------------------------

/// A daemon with `alice` logged in, her stash listing fetched and a
/// policy over Standard's tabs: the account opened, and a client.
async fn world(s: &Session) -> (Daemon, Frontend, Opened, Client) {
    let daemon = start_daemon(s).await;
    let frontend = Frontend::agent("test").expect("the scratch environment");
    let mut client = frontend.connect().await.expect("the daemon");
    login(&mut client, "alice").await;
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
    let dir = frontend.dir.clone();
    // A plan to quote: the documents test's fixture, in the session's world.
    acquisition_store::Index::load(&dir)
        .unwrap()
        .record_login("Alice#1234", "u-q", false, 1)
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
    };
    let plan = acquisition_verbs::refresh::compile("mock", &snapshot, "Alice#1234", 2_000).unwrap();
    // The outer watchdog outlives the tested bound: if the production
    // timeout is ever removed, this fails instead of hanging the gate.
    let planned = tokio::time::timeout(
        Duration::from_secs(10),
        quote_within(plan.clone(), Duration::from_millis(200)),
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
    let (_daemon, _frontend, opened, _client) = world(&s).await;
    let plan = opened
        .compile(Realm::Pc, "Standard", acquisition_store::now())
        .unwrap();
    let planned = quote(plan).await;
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
    };
    let rows = acquisition_verbs::jobs::failed_children_live(&mut client, &outcome).await;
    assert_eq!(rows.len(), 1);
    assert_eq!(
        (rows[0].job_id, rows[0].error.as_str()),
        (987_654, NO_RESULT)
    );
    assert_eq!((rows[0].kind.as_str(), rows[0].target.as_str()), ("", ""));
}
