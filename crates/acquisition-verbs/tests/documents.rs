//! The layer's documents and faults, with no environment and no daemon:
//! every value is built by hand over a fixture store in a scratch
//! directory, as the layer's public fields allow (C46). The fault kinds
//! and fields (C11, C85, C121), the account resolver's refusals, the
//! policy write's blind refusal (C35), the apply arms that need no daemon
//! (C38, C44, C111), the ledger mapping (C45) and the documents' shapes.

use std::path::{Path, PathBuf};

use acquisition_plan::pricing::Expect;
use acquisition_plan::{PolicyWrite, RefreshAction};
use acquisition_protocol::job::{Children, JobState, Outcome};
use acquisition_protocol::realm::Realm;
use acquisition_store::jobs::PersistedJob;
use acquisition_store::{
    AccountEntry, Annotations, Endpoint, Index, SYNC_POLICY_KIND, Store, account_path,
};
use acquisition_verbs::jobs::job_info;
use acquisition_verbs::refresh::{NOTHING_TO_DO, StoreChanges, compile, quote, spent_phrase};
use acquisition_verbs::{
    Account, Apply, DaemonReport, Door, Fault, Frontend, Job, JobResult, Jobs, Opened,
    PersistedSummary, QuoteNote,
};
use serde_json::{Value, json};

const USER: &str = "Alice#1234";
const UUID: &str = "u-verbs";

/// A scratch world's mock provider directory, `<root>/mock`; the root is
/// the test's own and goes when the test ends.
struct Scratch {
    mock: PathBuf,
    _root: tempfile::TempDir,
}

fn scratch() -> Scratch {
    let root = tempfile::tempdir().unwrap();
    let mock = root.path().join("mock");
    std::fs::create_dir_all(&mock).unwrap();
    Scratch { mock, _root: root }
}

/// A frontend over the world `dir` is the mock directory of, by hand: no
/// environment read.
fn frontend(dir: &Path) -> Frontend {
    Frontend {
        via: "test",
        door: Door::Agent,
        provider: "mock",
        root: dir
            .parent()
            .expect("a provider directory's root")
            .to_path_buf(),
        journal: acquisition_verbs::frontend::JournalKnob::Default,
    }
}

fn login(dir: &Path, username: &str, uuid: &str) {
    Index::load(dir)
        .unwrap()
        .record_login(username, uuid, false, 1_000, &[])
        .unwrap();
}

fn policy() -> Value {
    json!({ "version": 3, "realms": { "pc": { "leagues": { "Standard":
        { "tabs": "all", "max_age_seconds": 3600 } } } } })
}

/// One account logged in, its profile on record: no policy yet.
fn profiled(dir: &Path) -> Account {
    login(dir, USER, UUID);
    Store::open(&account_path(dir, USER))
        .unwrap()
        .record(
            &Endpoint::Profile,
            &json!({}),
            200,
            &json!({ "uuid": UUID, "name": USER }),
            1_000,
        )
        .unwrap();
    frontend(dir).account(None).unwrap()
}

/// One account with its profile on record and a policy over Standard's
/// tabs, opened.
fn opened(dir: &Path) -> Opened {
    let account = profiled(dir);
    account
        .set_policy(&policy(), Expect::Create, PolicyWrite::Replace)
        .unwrap();
    account.open().unwrap()
}

fn listing(store: &mut Store, at: i64, tabs: Value) {
    store
        .record(
            &Endpoint::Stashes {
                realm: "pc".into(),
                league: "Standard".into(),
            },
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stashes": tabs }),
            at,
        )
        .unwrap();
}

// ---- the faults ---------------------------------------------------------

/// C11, C85, C121: each fault's wire word and the fields beside it.
#[test]
fn a_faults_kind_and_fields_are_the_protocols_words() {
    let next = "a person starts one from a terminal: `acq auth check`".to_string();
    let cases: Vec<(Fault, Value)> = vec![
        (
            Fault::NoAccounts {
                dir: "/x".into(),
                provider: "mock".into(),
            },
            json!({ "kind": "not_logged_in", "accounts": [] }),
        ),
        (
            Fault::UnknownAccount {
                selector: "carol".into(),
                known: vec![USER.into()],
            },
            json!({ "kind": "not_logged_in", "accounts": [USER] }),
        ),
        (
            Fault::AmbiguousAccount {
                candidates: vec![USER.into(), "Bob#9".into()],
            },
            json!({ "kind": "ambiguous_account", "accounts": [USER, "Bob#9"] }),
        ),
        (
            Fault::NoUuid {
                username: USER.into(),
            },
            json!({ "kind": "refused" }),
        ),
        (
            Fault::Unopened(anyhow::anyhow!("disk")),
            json!({ "kind": "store" }),
        ),
        (
            Fault::NoDaemon {
                because: None,
                next: next.clone(),
            },
            json!({ "kind": "no_daemon", "next": next }),
        ),
        (
            Fault::NoJobOnDisk {
                id: 7,
                next: next.clone(),
            },
            json!({ "kind": "unknown_job", "next": next }),
        ),
        (
            Fault::Unfinished {
                id: 7,
                state: "waiting".into(),
                next: next.clone(),
            },
            json!({ "kind": "wrong_state", "next": next }),
        ),
        (
            Fault::Daemon(acquisition_client::client::DaemonError {
                kind: acquisition_protocol::protocol::ErrorKind::NotLoggedIn,
                message: "no session".into(),
            }),
            json!({ "kind": "not_logged_in" }),
        ),
        (
            Fault::Other(anyhow::anyhow!("anything")),
            json!({ "kind": "refused" }),
        ),
    ];
    for (fault, fields) in cases {
        assert_eq!(Value::Object(fault.fields()), fields, "{fault}");
        assert_eq!(fault.kind(), fields["kind"], "{fault}");
    }
}

// ---- the account (C121) -------------------------------------------------

#[test]
fn no_index_file_is_not_logged_in_with_an_empty_list() {
    let s = scratch();
    let fault = frontend(&s.mock).account(None).unwrap_err();
    assert!(matches!(fault, Fault::NoAccounts { .. }), "{fault}");
    assert!(
        fault.to_string().contains(&s.mock.display().to_string()),
        "{fault}"
    );
    assert_eq!(frontend(&s.mock).accounts().unwrap().accounts.len(), 0);
}

#[test]
fn two_accounts_and_no_selector_is_ambiguous_with_both() {
    let s = scratch();
    login(&s.mock, USER, UUID);
    login(&s.mock, "Bob#9", "u-bob");
    let fault = frontend(&s.mock).account(None).unwrap_err();
    assert_eq!(fault.kind(), "ambiguous_account", "{fault}");
    assert_eq!(fault.fields()["accounts"], json!([USER, "Bob#9"]));
    // and the listing names each with its store
    let accounts = frontend(&s.mock).accounts().unwrap();
    assert_eq!(accounts.accounts[1].store, account_path(&s.mock, "Bob#9"));
}

#[test]
fn a_selector_matching_none_is_not_logged_in_with_the_known() {
    let s = scratch();
    login(&s.mock, USER, UUID);
    let fault = frontend(&s.mock).account(Some("carol")).unwrap_err();
    assert!(matches!(fault, Fault::UnknownAccount { .. }), "{fault}");
    assert_eq!(
        Value::Object(fault.fields()),
        json!({ "kind": "not_logged_in", "accounts": [USER] })
    );
}

#[test]
fn a_bare_name_shared_by_two_discriminators_is_ambiguous_with_both() {
    let s = scratch();
    login(&s.mock, "Alice#1", "u-1");
    login(&s.mock, "Alice#2", "u-2");
    let fault = frontend(&s.mock).account(Some("alice")).unwrap_err();
    assert_eq!(
        Value::Object(fault.fields()),
        json!({ "kind": "ambiguous_account", "accounts": ["Alice#1", "Alice#2"] })
    );
    let picked = frontend(&s.mock).account(Some("Alice#2")).unwrap();
    assert_eq!(picked.entry.uuid.as_deref(), Some("u-2"));
}

#[test]
fn an_entry_without_a_uuid_asked_for_intent_is_refused() {
    let s = scratch();
    let account = Account {
        frontend: frontend(&s.mock),
        entry: AccountEntry {
            username: USER.into(),
            last_login: 1,
            persisted: false,
            uuid: None,
            keyring_names: Vec::new(),
        },
    };
    let fault = account.intent().err().expect("refused");
    assert!(matches!(fault, Fault::NoUuid { .. }), "{fault}");
    assert!(matches!(account.open(), Err(Fault::NoUuid { .. })));
    assert!(matches!(account.policy(), Err(Fault::NoUuid { .. })));
}

// ---- the policy write (C52, C35) ----------------------------------------

#[test]
fn a_blind_policy_write_over_a_stored_row_this_build_cannot_read_is_refused() {
    let s = scratch();
    login(&s.mock, USER, UUID);
    let account = frontend(&s.mock).account(None).unwrap();
    account
        .set_policy(&policy(), Expect::Current, PolicyWrite::Merge)
        .unwrap();
    // The store's own door refuses a newer version at put, so the newer
    // build's value is planted under it, as that build would.
    rusqlite::Connection::open(acquisition_store::annotations_path(&s.mock, UUID))
        .unwrap()
        .execute(
            "UPDATE annotations SET value = '{\"version\":99,\"realms\":{}}' WHERE kind = ?1",
            [SYNC_POLICY_KIND],
        )
        .unwrap();
    for write in [PolicyWrite::Merge, PolicyWrite::Replace] {
        let fault = account
            .set_policy(&policy(), Expect::Current, write)
            .unwrap_err();
        assert!(
            matches!(fault, Fault::UnreadablePolicy { revision: 1, .. }),
            "{fault}"
        );
    }
    // Naming the revision is the reviewed path past it.
    let row = account
        .set_policy(&policy(), Expect::Exactly(1), PolicyWrite::Replace)
        .unwrap();
    assert_eq!(row.row.revision, 2);
    // And the create-only form refuses a row standing.
    let fault = account
        .set_policy(&policy(), Expect::Create, PolicyWrite::Replace)
        .unwrap_err();
    assert!(matches!(fault, Fault::Policy(_)), "{fault}");
    assert_eq!(
        Annotations::open_for(&s.mock, UUID)
            .unwrap()
            .get("account", "", SYNC_POLICY_KIND)
            .unwrap()
            .map(|r| r.revision),
        Some(2)
    );
}

/// The CLI's blind default (moved here with its write): a typo'd policy
/// is refused by the planner's strict parse and nothing lands; a write
/// naming no revision continues the sequence; a named stale revision is
/// a conflict naming the current one, the stored value untouched.
#[test]
fn a_policy_write_is_validated_and_compares_its_revision() {
    let s = scratch();
    login(&s.mock, USER, UUID);
    let account = frontend(&s.mock).account(None).unwrap();
    let typo =
        json!({ "version": 1, "leagues": { "Standard": { "tabs": "all", "max_age_secs": 60 } } });
    let fault = account
        .set_policy(&typo, Expect::Current, PolicyWrite::Merge)
        .unwrap_err();
    assert!(matches!(fault, Fault::Policy(_)), "{fault}");
    assert!(account.policy().unwrap().is_none());
    let first = account
        .set_policy(&policy(), Expect::Current, PolicyWrite::Merge)
        .unwrap();
    let second = account
        .set_policy(&policy(), Expect::Current, PolicyWrite::Merge)
        .unwrap();
    assert_eq!((first.row.revision, second.row.revision), (1, 2));
    let fault = account
        .set_policy(
            &json!({ "version": 3, "realms": {} }),
            Expect::Exactly(first.row.revision),
            PolicyWrite::Replace,
        )
        .unwrap_err();
    assert!(fault.to_string().contains("revision 2"), "{fault}");
    assert_eq!(account.policy().unwrap().unwrap().value, policy());
    let row = account
        .set_policy(&policy(), Expect::Exactly(2), PolicyWrite::Merge)
        .unwrap();
    assert_eq!(
        row.row.written_via, "test",
        "stamped with the frontend's channel"
    );
}

// ---- the refresh flow, no daemon (C38, C44, C111) -----------------------

#[test]
fn an_account_with_no_policy_compiles_to_no_sync_policy_naming_it() {
    let s = scratch();
    let opened = profiled(&s.mock).open().unwrap();
    let fault = opened.compile(Realm::Pc, "Standard", 2_000).unwrap_err();
    assert!(matches!(fault, Fault::NoSyncPolicy { .. }), "{fault}");
    assert!(fault.to_string().contains(USER), "{fault}");
}

#[tokio::test]
async fn a_plan_whose_policy_revision_moved_is_refused_before_any_daemon() {
    let s = scratch();
    let opened = opened(&s.mock);
    let plan = opened.compile(Realm::Pc, "Standard", 2_000).unwrap();
    opened
        .account
        .set_policy(&policy(), Expect::Current, PolicyWrite::Merge)
        .unwrap();
    // The intent the open handle holds is the file's; re-open to read it.
    let mut opened = opened.account.open().unwrap();
    let mut client = None;
    let fault = opened.apply(&plan, None, &mut client).await.unwrap_err();
    assert!(
        matches!(
            fault,
            Fault::Spend(acquisition_plan::SpendError::PolicyMoved { .. })
        ),
        "{fault}"
    );
    assert!(client.is_none(), "the gate ran after a connect");
}

#[tokio::test]
async fn an_empty_plan_is_nothing_with_its_skipped_and_no_daemon() {
    let s = scratch();
    let mut opened = opened(&s.mock);
    listing(&mut opened.facts, 1_900, json!([]));
    let plan = opened.compile(Realm::Pc, "Standard", 2_000).unwrap();
    assert!(plan.actions.is_empty(), "{plan:?}");
    let mut client = None;
    let applied = opened.apply(&plan, None, &mut client).await.unwrap();
    assert_eq!(
        serde_json::to_value(&applied).unwrap(),
        json!({ "applied": false, "requests": 0, "note": NOTHING_TO_DO,
                "skipped": { "tabs": 0, "characters": 0 } })
    );
    assert!(client.is_none());
}

/// C111: a plan whose every request the facts show fetched after it was
/// compiled is spent with no daemon contact, and says what a plan
/// compiled now would fetch.
#[tokio::test]
async fn a_plan_the_facts_have_spent_is_spent_with_its_remaining_count() {
    let s = scratch();
    let mut opened = opened(&s.mock);
    let plan = opened
        .compile(Realm::Pc, "Standard", acquisition_store::now())
        .unwrap();
    assert!(
        matches!(plan.actions.as_slice(), [RefreshAction::ListStashes { .. }]),
        "{plan:?}"
    );
    listing(
        &mut opened.facts,
        acquisition_store::now(),
        json!([{ "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 }]),
    );
    let mut client = None;
    let applied = opened.apply(&plan, None, &mut client).await.unwrap();
    let Apply::Spent {
        spent,
        remaining_requests,
        note,
        ..
    } = &applied
    else {
        panic!("{applied:?}")
    };
    assert_eq!(spent.len(), 1);
    assert_eq!(
        *remaining_requests,
        Some(1),
        "the one tab the listing revealed"
    );
    assert!(
        note.starts_with("nothing to do: every request of this plan is spent — 1 fetched"),
        "{note}"
    );
    assert!(
        !note.contains("acq "),
        "a frontend's command in the shared note: {note}"
    );
    assert!(client.is_none(), "a spent plan contacted a daemon");
}

/// Two applies landing in one second (C34, facts through the store):
/// each apply's `changed:` counts the events committed after its own
/// submit, never the same second's earlier ones. The fault this names:
/// the count read events by whole seconds, so the second apply counted
/// the first one's events again (9 of 20 journeys).
#[test]
fn two_applies_in_one_second_do_not_count_each_others_events() {
    let s = scratch();
    let mut opened = opened(&s.mock);
    let at = 5_000;
    listing(
        &mut opened.facts,
        at,
        json!([{ "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 },
               { "id": "t2", "name": "Two", "type": "PremiumStash", "index": 1 }]),
    );
    let fetch = |store: &mut Store, id: &str, n: usize| {
        let items: Vec<Value> = (0..n)
            .map(|i| {
                json!({ "id": format!("{id}-i{i}"), "name": "", "typeLine": "Chaos Orb",
                             "baseType": "Chaos Orb", "x": i, "y": 0, "w": 1, "h": 1 })
            })
            .collect();
        store
            .record(
                &Endpoint::Stash {
                    realm: "pc".into(),
                    league: "Standard".into(),
                    id: id.into(),
                    sub: None,
                },
                &json!({}),
                200,
                &json!({ "stash": { "id": id, "name": id, "type": "PremiumStash", "index": 0, "items": items } }),
                at,
            )
            .unwrap();
    };
    let first = opened.facts.status().unwrap();
    fetch(&mut opened.facts, "t1", 2);
    let second = opened.facts.status().unwrap();
    fetch(&mut opened.facts, "t2", 3);

    let changes = StoreChanges::since(&opened.facts, &second).unwrap();
    assert_eq!(
        (changes.added, changes.locations, changes.responses),
        (3, 1, 1),
        "the second apply counted only the second ingest: {changes:?}"
    );
    // The first apply still counts what landed while it ran, the second
    // ingest included: the watermark excludes the past, never a neighbour.
    let changes = StoreChanges::since(&opened.facts, &first).unwrap();
    assert_eq!((changes.added, changes.locations), (5, 2), "{changes:?}");
}

/// C111 in words: the queued half names the jobs, never a command.
#[test]
fn the_spent_phrase_names_the_jobs_holding_a_request() {
    let action = RefreshAction::ListStashes {
        realm: Realm::Pc,
        league: "Standard".into(),
        reason: acquisition_plan::ListingReason::NeverListed,
    };
    let spent = vec![
        acquisition_plan::Spent {
            action: action.clone(),
            by: acquisition_plan::SpentBy::Queued {
                job_id: 9,
                parent: Some(4),
            },
        },
        acquisition_plan::Spent {
            action,
            by: acquisition_plan::SpentBy::Fetched { fetched_at: 100 },
        },
    ];
    assert_eq!(
        spent_phrase(&spent, 130),
        "1 fetched after it was compiled, the latest 30s ago, 1 waiting or running in job 4"
    );
}

/// The quote's one sentence for a plan with no account name to quote
/// as: no daemon is asked.
#[tokio::test]
async fn a_plan_with_no_account_name_is_quoted_with_its_note() {
    let s = scratch();
    let opened = opened(&s.mock);
    let mut snapshot = opened
        .facts
        .refresh_snapshot("pc", "Standard", &opened.intent)
        .unwrap();
    snapshot.account_name = None;
    let plan = compile("mock", &snapshot, USER, 2_000).unwrap();
    assert_eq!(plan.account_name, None);
    let planned = quote(&opened.account.frontend.root, plan).await;
    assert_eq!(planned.quote_note, Some(QuoteNote::NoAccountName));
    // The JSON note is the sentence, on every surface.
    assert_eq!(
        serde_json::to_value(&planned).unwrap()["quote_note"],
        json!("no quote: the facts record no account name to quote as")
    );
}

/// Each cause's sentence, word for word: the JSON `quote_note` an agent
/// reads did not change when the note became typed.
#[test]
fn a_quote_notes_sentence_is_its_cause_in_words() {
    for (note, words) in [
        (
            QuoteNote::NoDaemon,
            "no quote: no daemon running (the plan needs none; a running daemon adds its ETA)",
        ),
        (
            QuoteNote::Timeout(std::time::Duration::from_secs(5)),
            "no quote: the daemon did not answer within 5s — plan compiled offline",
        ),
        (
            QuoteNote::Rejected("names another account".into()),
            "daemon quote rejected: names another account",
        ),
        (
            QuoteNote::Other("a daemon of another contract".into()),
            "no quote: a daemon of another contract — plan compiled offline",
        ),
    ] {
        assert_eq!(note.to_string(), words);
        assert_eq!(serde_json::to_value(&note).unwrap(), json!(words));
    }
}

// ---- the ledger (C45) and the documents ---------------------------------

fn persisted_job(state: &str) -> PersistedJob {
    PersistedJob {
        id: 3,
        kind: "stash".into(),
        state: state.into(),
        priority: 0,
        submitted_by: "cli:1".into(),
        parent: Some(1),
        retries: 2,
        account: Some(USER.into()),
        params: json!({ "id": "t1" }),
        cancel_requested: false,
        submitted_at: 10,
        updated_at: 20,
    }
}

#[test]
fn a_ledger_row_maps_to_the_live_shape_and_an_unknown_state_is_a_fault() {
    for (word, state) in [
        ("waiting", JobState::Waiting),
        ("running", JobState::Running),
        ("done", JobState::Done),
        ("failed", JobState::Failed),
        ("cancelled", JobState::Cancelled),
    ] {
        let info = job_info(&persisted_job(word)).unwrap();
        assert_eq!(
            (info.state, info.eta_seconds, info.parent, info.retries),
            (state, None, Some(1), 2)
        );
    }
    let fault = job_info(&persisted_job("paused")).unwrap_err();
    assert!(fault.to_string().contains("`paused`"), "{fault}");
}

#[test]
fn the_absent_documents_say_running_false_beside_what_the_ledger_holds() {
    let job = Job::Absent {
        persisted: persisted_job("done"),
        next: "n".into(),
    };
    let doc = serde_json::to_value(&job).unwrap();
    assert_eq!(doc["running"], false);
    assert_eq!(doc["persisted"]["state"], "done");
    assert_eq!(doc["next"], "n");
    assert_eq!(
        serde_json::to_value(Jobs::Absent { persisted: None }).unwrap(),
        json!({ "running": false, "persisted": null })
    );
    let report = DaemonReport::Absent {
        persisted: Some(PersistedSummary {
            waiting: 1,
            recorded_running: 0,
            written_at: Some(5),
        }),
        log: Some("/l/daemon.log".into()),
    };
    assert_eq!(
        serde_json::to_value(&report).unwrap(),
        json!({ "running": false, "log": "/l/daemon.log",
                "persisted": { "waiting": 1, "recorded_running": 0, "written_at": 5 } })
    );
}

/// C84 and C10: a running daemon's report is one flat document, its
/// vitals and the identity keys of its handshake side by side under
/// `running` and `compatible`; another daemon's is its identity alone,
/// `compatible: false`, each dimension named (the frontends print it and
/// pin what they add: never replaced, the sibling under `wanted`).
#[test]
fn a_daemons_report_is_flat_whether_it_is_this_clients_or_another() {
    use acquisition_client::client::{DaemonId, Endpoint, Reported};
    use acquisition_protocol::protocol::Response;
    use acquisition_protocol::status::RailsStatus;
    let identity = |provider: &str| {
        DaemonId::judged(
            Reported {
                pid: 4242,
                version: "0.0.1".into(),
                contract: "ffffffffffff".into(),
                artifact: None,
                provider: provider.into(),
                world: "/w".into(),
            },
            Endpoint {
                socket: "/run/acq/w.sock".into(),
            },
            Ok("/w".into()),
        )
    };
    let status = Response::DaemonStatus {
        pid: 4242,
        version: "0.0.1".into(),
        contract: "ffffffffffff".into(),
        provider: "mock".into(),
        uptime_seconds: 61,
        idle_shutdown_seconds: 60,
        connections: 1,
        jobs_waiting: 0,
        jobs_running: 0,
        policies_known: 0,
        in_flight: 0,
        max_in_flight: 2,
        rails: RailsStatus {
            tripwire_enabled: false,
            halted: None,
            refresh_failed: None,
            sends: 0,
            max_sends: None,
            journal: None,
        },
        keyring: "ok".into(),
        log: Some("/l/daemon.log".into()),
    };
    let identity_keys = [
        "contract",
        "artifact",
        "world",
        "socket",
        "contract_matches",
        "artifact_matches",
        "artifact_relation",
        "provider_matches",
        "world_matches",
        "wanted",
    ];
    let running = serde_json::to_value(DaemonReport::Running {
        status: Box::new(status),
        identity: Box::new(identity("mock")),
    })
    .unwrap();
    assert_eq!(
        (&running["running"], &running["compatible"]),
        (&json!(true), &json!(true))
    );
    for vital in [
        "pid",
        "uptime_seconds",
        "idle_shutdown_seconds",
        "rails",
        "keyring",
        "log",
    ] {
        assert!(running.get(vital).is_some(), "{vital}: {running}");
    }
    for key in identity_keys {
        assert!(running.get(key).is_some(), "{key}: {running}");
    }
    assert_eq!(running["socket"], "/run/acq/w.sock");
    assert_eq!(running["world"], "/w");
    assert_eq!(running["world_matches"], true);
    assert!(running["wanted"].is_object(), "{running}");

    let other =
        serde_json::to_value(DaemonReport::Incompatible(Box::new(identity("ggg")))).unwrap();
    assert_eq!(
        (&other["running"], &other["compatible"]),
        (&json!(true), &json!(false))
    );
    assert_eq!(other["pid"], 4242, "{other}");
    for key in identity_keys {
        assert!(other.get(key).is_some(), "{key}: {other}");
    }
    assert_eq!(other["provider"], "ggg", "{other}");
    assert_eq!(other["contract_matches"], false, "{other}");
    assert!(other.get("uptime_seconds").is_none(), "{other}");
}

/// A result carries its job's id and account beside the outcome, and a
/// parent's failure its children's tally as a field (C23) and its first
/// failed children detailed.
#[test]
fn a_job_result_is_the_outcome_with_its_id_and_account() {
    let result = JobResult {
        job_id: 4,
        account: Some(USER.into()),
        outcome: Outcome::Failure {
            error: "1 of 2 child jobs failed, 1 done".into(),
            children: Some(Children {
                done: 1,
                failed: 1,
                cancelled: 0,
                failed_ids: vec![6],
            }),
            payload: None,
        },
        failed_children: vec![acquisition_verbs::FailedChild {
            job_id: 6,
            kind: "stash".into(),
            target: "Standard/t6".into(),
            error: "GET /stash/Standard/t6 returned 404".into(),
        }],
    };
    assert_eq!(
        serde_json::to_value(&result).unwrap(),
        json!({ "job_id": 4, "account": USER, "outcome": "failure",
                "error": "1 of 2 child jobs failed, 1 done",
                "children": { "done": 1, "failed": 1, "cancelled": 0, "failed_ids": [6] },
                "failed_children": [{ "job_id": 6, "kind": "stash", "target": "Standard/t6",
                                      "error": "GET /stash/Standard/t6 returned 404" }] })
    );
}

/// A failure that is not a parent's carries no `failed_children`: the
/// field is absent, not an empty list (C53, additive).
#[test]
fn a_plain_failure_serialises_without_failed_children() {
    let result = JobResult {
        job_id: 9,
        account: None,
        outcome: Outcome::failure("GET /stash/Standard/t9 returned 404"),
        failed_children: Vec::new(),
    };
    assert_eq!(
        serde_json::to_value(&result).unwrap(),
        json!({ "job_id": 9, "outcome": "failure", "error": "GET /stash/Standard/t9 returned 404" })
    );
}

// ---- the store's reads as documents (C149) ------------------------------

/// C149 and C53: the events of a window are one document, ten by default
/// beside how many it returned and whether the limit was reached; a
/// `limit` names more, and `limit: 0` returns none, its limit reached.
#[test]
fn c149_events_are_one_document_ten_by_default_with_the_limit_said() {
    let dir = tempfile::tempdir().unwrap();
    let mut store = Store::open(&dir.path().join("facts.db")).unwrap();
    let now = acquisition_store::now();
    let items: Vec<Value> = (0..12)
        .map(|n| {
            json!({ "id": format!("i{n:02}"), "typeLine": "Iron Ring", "baseType": "Iron Ring",
                         "frameType": 0, "identified": true, "ilvl": 1, "x": n, "y": 0 })
        })
        .collect();
    store
        .record(
            &Endpoint::Stash {
                realm: "pc".into(),
                league: "Standard".into(),
                id: "t1".into(),
                sub: None,
            },
            &json!({ "league": "Standard" }),
            200,
            &json!({ "stash": { "id": "t1", "name": "One", "type": "PremiumStash", "items": items } }),
            now - 60,
        )
        .unwrap();
    let read = |limit| acquisition_verbs::events::Events::read(&store, now, 24.0, limit).unwrap();
    let ten = read(None);
    assert_eq!((ten.limit, ten.returned, ten.limit_reached), (10, 10, true));
    assert_eq!(ten.rows.len(), 10);
    let doc = serde_json::to_value(&ten).unwrap();
    assert!(doc.is_object(), "{doc}");
    assert_eq!(doc["rows"].as_array().map(Vec::len), Some(10), "{doc}");
    let none = read(Some(0));
    assert_eq!(
        (none.limit, none.returned, none.limit_reached),
        (0, 0, true)
    );
    let all = read(Some(50));
    assert_eq!((all.returned, all.limit_reached), (12, false));
    let summary = acquisition_verbs::events::EventSummary::of(
        &all,
        acquisition_verbs::events::summary(&store, &all.rows),
        false,
    );
    assert_eq!((summary.events, summary.locations), (12, 1));
    assert_eq!(summary.by_kind.added, 12);
}

/// One ledger row of `state` with `id`, in the live list's shape.
fn job_row(id: u64, state: &str) -> PersistedJob {
    PersistedJob {
        id,
        ..persisted_job(state)
    }
}

/// C149 and C53, and the brief's second review, finding 2 (no exemption
/// for the open jobs): the live list is one document, its counts made
/// over every job before the cut — 800 done, 12 waiting with predicted
/// starts, one running — beside ten open rows (the first by id, not by
/// when they run) and
/// the ten most recent finished, in id order, with what each left out;
/// `expand` lists every job. The ledger's queue is ten of its open rows
/// beside its counts.
#[test]
fn c149_the_job_list_counts_every_job_beside_ten_open_and_ten_finished() {
    let mut jobs: Vec<_> = (1..=800)
        .map(|id| job_info(&job_row(id, "done")).unwrap())
        .collect();
    for id in 801..=812 {
        let mut waiting = job_info(&job_row(id, "waiting")).unwrap();
        waiting.eta_seconds = Some(id - 800);
        jobs.push(waiting);
    }
    jobs.push(job_info(&job_row(813, "running")).unwrap());
    let listed = acquisition_verbs::Listed::of(jobs.clone(), false);
    assert_eq!(listed.counts.total, 813);
    assert_eq!(
        (
            listed.counts.by_state.done,
            listed.counts.by_state.waiting,
            listed.counts.by_state.running
        ),
        (800, 12, 1)
    );
    assert_eq!(listed.counts.next_starts_in_seconds, Some(1));
    assert_eq!(listed.counts.last_starts_in_seconds, Some(12));
    let ids: Vec<u64> = listed.rows.iter().map(|j| j.id).collect();
    let expected: Vec<u64> = (791..=810).collect();
    assert_eq!(
        ids, expected,
        "the ten most recent done, then the ten first open"
    );
    assert_eq!((listed.open_left_out, listed.finished_left_out), (3, 790));
    // the first ten open by id, not by when they run: the one running job
    // (813) is among those left out, and counted (round 3's review, P3)
    assert!(
        !listed.rows.iter().any(|j| j.id == 813),
        "{:?}",
        listed.rows
    );
    assert_eq!(listed.counts.by_state.running, 1);
    assert_eq!(listed.left_out_needs, Some("expand"));
    let doc = serde_json::to_value(Jobs::Live(listed)).unwrap();
    assert_eq!(doc["running"], true, "{doc}");
    assert_eq!(doc["total"], 813, "{doc}");
    assert_eq!(doc["by_state"]["done"], 800, "{doc}");
    assert_eq!(doc["rows"].as_array().map(Vec::len), Some(20));

    let all = acquisition_verbs::Listed::of(jobs, true);
    assert_eq!(all.rows.len(), 813);
    assert_eq!((all.open_left_out, all.finished_left_out), (0, 0));
    assert_eq!(all.left_out_needs, None);

    let persisted = acquisition_store::jobs::Persisted {
        jobs: (1..=12).map(|id| job_row(id, "waiting")).collect(),
        written_at: Some(20),
    };
    let queue = acquisition_verbs::PersistedJobs::of(persisted.clone(), false);
    assert_eq!(
        (queue.total, queue.waiting, queue.recorded_running),
        (12, 12, 0)
    );
    assert_eq!((queue.jobs.len(), queue.left_out), (10, 2));
    let doc = serde_json::to_value(Jobs::Absent {
        persisted: Some(queue),
    })
    .unwrap();
    assert_eq!(doc["running"], false, "{doc}");
    assert_eq!(doc["persisted"]["total"], 12, "{doc}");
    let whole = acquisition_verbs::PersistedJobs::of(persisted, true);
    assert_eq!((whole.jobs.len(), whole.left_out), (12, 0));
}

/// C149 and C53, and the brief's second review, finding 2 (no exemption
/// for a handful of names): the accounts listing counts every account
/// beside ten of them, every one under `expand`.
#[test]
fn c149_the_accounts_are_counted_beside_ten() {
    let s = scratch();
    for n in 0..12 {
        login(&s.mock, &format!("Exile{n:02}#1"), &format!("u-{n}"));
    }
    let whole = frontend(&s.mock).accounts().unwrap();
    assert_eq!(
        (whole.total, whole.accounts.len(), whole.left_out),
        (12, 12, 0)
    );
    let listed = whole.clone().bounded(false);
    assert_eq!((listed.total, listed.accounts.len()), (12, 10));
    assert_eq!(listed.left_out, 2);
    let doc = serde_json::to_value(&listed).unwrap();
    assert_eq!(doc["left_out_needs"], "expand", "{doc}");
    assert_eq!(doc["provider"], "mock", "{doc}");
    let all = whole.bounded(true);
    assert_eq!((all.accounts.len(), all.left_out), (12, 0));
    assert!(
        serde_json::to_value(&all)
            .unwrap()
            .get("left_out_needs")
            .is_none()
    );
}

/// C149, the pin: every document a verb returns serializes to one JSON
/// object, never a bare array or a bare null — each answer this step
/// wrapped (characters, the events and their summary, the job list live
/// and absent, the accounts, the policy shown, the refused bodies) and
/// the daemon's report beside them, each over an empty world, where a
/// bare list would have been `[]` and a missing row `null`.
#[test]
fn c149_every_document_a_verb_returns_is_one_json_object() {
    use acquisition_verbs::events::{EventSummary, Events};
    let s = scratch();
    login(&s.mock, USER, UUID);
    let store = Store::open(&s.mock.join("facts.db")).unwrap();
    let events = Events::read(&store, acquisition_store::now(), 24.0, None).unwrap();
    let documents: Vec<(&str, Value)> = vec![
        (
            "characters",
            serde_json::to_value(acquisition_verbs::Characters::of(
                None,
                None,
                Vec::new(),
                None,
                false,
            ))
            .unwrap(),
        ),
        (
            "events",
            serde_json::to_value(Events::read(&store, 0, 1.0, Some(0)).unwrap()).unwrap(),
        ),
        (
            "event summary",
            serde_json::to_value(EventSummary::of(&events, Vec::new(), false)).unwrap(),
        ),
        (
            "jobs, live",
            serde_json::to_value(Jobs::Live(acquisition_verbs::Listed::of(Vec::new(), false)))
                .unwrap(),
        ),
        (
            "jobs, no queue",
            serde_json::to_value(Jobs::Absent { persisted: None }).unwrap(),
        ),
        (
            "jobs, the ledger's",
            serde_json::to_value(Jobs::Absent {
                persisted: Some(acquisition_verbs::PersistedJobs::of(
                    acquisition_store::jobs::Persisted {
                        jobs: Vec::new(),
                        written_at: None,
                    },
                    false,
                )),
            })
            .unwrap(),
        ),
        (
            "accounts",
            serde_json::to_value(frontend(&s.mock).accounts().unwrap().bounded(false)).unwrap(),
        ),
        (
            "policy shown",
            serde_json::to_value(acquisition_verbs::PolicyShown { policy: None }).unwrap(),
        ),
        (
            "refused bodies",
            serde_json::to_value(acquisition_verbs::RefusedBodies::read(&store, 50).unwrap())
                .unwrap(),
        ),
        (
            "daemon, absent",
            serde_json::to_value(DaemonReport::Absent {
                persisted: None,
                log: None,
            })
            .unwrap(),
        ),
    ];
    for (name, doc) in &documents {
        assert!(doc.is_object(), "{name}: {doc}");
    }
    assert_eq!(documents[7].1, json!({ "policy": null }));
}

// ---- the one refusal shape (C11, C149; round 4 of the 2g build) ---------

/// A surface for the shape's own tests: its words the chain whole, and
/// a way spelled as its debug text, the tool named for every way but the
/// same call again.
struct Plain;

impl acquisition_verbs::refusal::Surface for Plain {
    fn words(&self, e: &anyhow::Error) -> String {
        format!("{e:#}")
    }

    fn request(
        &self,
        way: &acquisition_verbs::refusal::Way,
    ) -> Option<(Value, Option<&'static str>)> {
        use acquisition_verbs::refusal::Way;
        let tool = match way {
            Way::Again { .. } => None,
            Way::Search(_) => Some("search"),
            Way::Tabs { .. } => Some("tabs"),
            Way::PriceShow { .. } => Some("price_show"),
            Way::PriceStatus { .. } => Some("price_status"),
        };
        Some((json!(format!("{way:?}")), tool))
    }
}

/// The keys a refusal may hold: `error` and `kind` always; the four the
/// shape adds — `offers`, `readings`, `next`, the kind's own fields —
/// where they have something to say.
const REFUSAL_KEYS: [&str; 9] = [
    "error", "kind", "offers", "readings", "next", "accounts", "revision", "current", "span",
];

/// A not_found's refusal as the planner makes it, over a scope named.
fn missing(elsewhere: Vec<(Realm, String)>, own_row: Option<i64>) -> anyhow::Error {
    use acquisition_plan::pricing::{Missing, Refused, RefusedKind, read_target};
    anyhow::Error::new(Refused {
        kind: RefusedKind::NotFound(Box::new(Missing {
            target: read_target("tab/xbox/nope").unwrap(),
            account: USER.into(),
            realm: Realm::Xbox,
            league: "Allflame".into(),
            elsewhere,
            own_row,
        })),
        message: "tab/xbox/nope is not in the facts".into(),
    })
}

/// Every refusal the layer, the planner, the search and the daemon make
/// is one document from one function: `error` and `kind` from the closed
/// lists, at most the shape's keys, an added key absent and never null,
/// and C11's `revision: null` kept on a conflict over no row ever written.
#[test]
fn every_refusal_is_one_shape_from_one_function() {
    use acquisition_plan::pricing::{Refused, RefusedKind};
    use acquisition_search::error::ErrorKind as SearchKind;
    use acquisition_search::{LanguageError, SearchError};
    use acquisition_verbs::refusal::{Arguments, document, of_search};
    let next = "a person starts one from a terminal: `acq auth check`".to_string();
    let cases: Vec<(anyhow::Error, &str)> = vec![
        (
            Fault::UnknownAccount {
                selector: "GERWARIK#7694".into(),
                known: vec!["GERWARIC#7694".into()],
            }
            .into(),
            "not_logged_in",
        ),
        (
            Fault::AmbiguousAccount {
                candidates: vec![USER.into(), "Bob#9".into()],
            }
            .into(),
            "ambiguous_account",
        ),
        (
            Fault::NoDaemon {
                because: None,
                next: next.clone(),
            }
            .into(),
            "no_daemon",
        ),
        (Fault::Unopened(anyhow::anyhow!("disk")).into(), "store"),
        (
            anyhow::Error::new(Refused {
                kind: RefusedKind::Conflict {
                    revision: None,
                    current: None,
                },
                message: "item/x has no price of its own".into(),
            }),
            "conflict",
        ),
        (missing(vec![], Some(3)), "not_found"),
        (
            anyhow::Error::new(acquisition_client::client::DaemonError {
                kind: acquisition_protocol::protocol::ErrorKind::UnknownJob,
                message: "no job 7".into(),
            }),
            "unknown_job",
        ),
        (Arguments("not an argument".into()).into(), "arguments"),
        (anyhow::anyhow!("anything else"), "refused"),
        (
            anyhow::Error::new(SearchError::Language(LanguageError {
                kind: SearchKind::BareWord,
                message: "a word with no operator".into(),
                span: Some((0, 4)),
                readings: vec!["rarity=rare".into()],
            })),
            "bare_word",
        ),
    ];
    for (e, kind) in cases {
        let doc = document(&e, &Plain);
        let fields = doc.as_object().unwrap();
        assert_eq!(fields["kind"], kind, "{doc}");
        assert!(fields["error"].is_string(), "{doc}");
        for (key, value) in fields {
            assert!(REFUSAL_KEYS.contains(&key.as_str()), "{key} in {doc}");
            assert!(
                !value.is_null() || key == "revision",
                "{key} is null in {doc}"
            );
        }
        for offer in fields
            .get("offers")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let keys: Vec<&String> = offer.as_object().unwrap().keys().collect();
            assert!(
                keys == ["request", "says"] || keys == ["request", "says", "tool"],
                "{offer}"
            );
        }
        // a search's refusal by itself is the same document
        if let Some(search) = e.downcast_ref::<SearchError>() {
            assert_eq!(of_search(search, &Plain), doc);
        }
    }
    let conflict = document(
        &anyhow::Error::new(Refused {
            kind: RefusedKind::Conflict {
                revision: None,
                current: None,
            },
            message: "nothing to clear".into(),
        }),
        &Plain,
    );
    assert_eq!(
        conflict,
        json!({ "error": "nothing to clear", "kind": "conflict", "revision": null })
    );
}

/// C121 kept: a name no login made is `not_logged_in` with the names
/// known, its words the two readings, and an offer per name: the request
/// refused, sent again as that account; several to pick from, the same.
#[test]
fn an_account_that_cannot_be_picked_offers_the_request_as_each_name() {
    use acquisition_verbs::refusal::document;
    let doc = document(
        &Fault::UnknownAccount {
            selector: "GERWARIK#7694".into(),
            known: vec!["GERWARIC#7694".into(), "Bob#9".into()],
        }
        .into(),
        &Plain,
    );
    assert_eq!(
        doc["error"],
        "no login made a store for `GERWARIK#7694`; the names known: GERWARIC#7694, Bob#9"
    );
    assert_eq!(doc["accounts"], json!(["GERWARIC#7694", "Bob#9"]));
    assert_eq!(
        doc["offers"],
        json!([
            { "request": "Again { account: \"GERWARIC#7694\" }", "says": "the same request as GERWARIC#7694" },
            { "request": "Again { account: \"Bob#9\" }", "says": "the same request as Bob#9" },
        ])
    );
    let doc = document(
        &Fault::AmbiguousAccount {
            candidates: vec![USER.into(), "Bob#9".into()],
        }
        .into(),
        &Plain,
    );
    assert_eq!(doc["offers"].as_array().unwrap().len(), 2, "{doc}");
}

/// A target the facts do not hold offers what would find it, made from
/// the scope the refusal carries: held elsewhere, its show in each place;
/// held nowhere, the status that lists a row of its own, a search of the
/// realm and the league's tabs — each naming the account, realm and
/// league resolved, never one read again.
#[test]
fn a_not_found_offers_what_would_find_it_in_the_scope_it_carries() {
    use acquisition_verbs::refusal::document;
    let doc = document(&missing(vec![], Some(3)), &Plain);
    let tools: Vec<&Value> = doc["offers"]
        .as_array()
        .unwrap()
        .iter()
        .map(|o| &o["tool"])
        .collect();
    assert_eq!(
        tools,
        [&json!("price_status"), &json!("search"), &json!("tabs")]
    );
    let tabs = doc["offers"][2]["request"].as_str().unwrap();
    assert!(
        tabs.contains("Xbox") && tabs.contains("Allflame") && tabs.contains(USER),
        "{tabs}"
    );
    let search = doc["offers"][1]["request"].as_str().unwrap();
    assert!(search.contains("xbox") && search.contains(USER), "{search}");
    let doc = document(&missing(vec![], None), &Plain);
    assert_eq!(doc["offers"].as_array().unwrap().len(), 2, "{doc}");
    let doc = document(
        &missing(vec![(Realm::Pc, "Standard".into())], Some(3)),
        &Plain,
    );
    assert_eq!(
        doc["offers"],
        json!([{
            "request": format!("PriceShow {{ target: \"tab/xbox/nope\", account: \"{USER}\", realm: Pc, league: \"Standard\" }}"),
            "says": "tab/xbox/nope in pc Standard, where the facts hold it",
            "tool": "price_show",
        }])
    );
}

/// C11: a policy write refused for its revision is `conflict`, with the
/// stored row's revision and the row itself as `policy` prints it.
#[test]
fn a_policy_write_refused_for_its_revision_is_a_conflict_with_the_row() {
    let s = scratch();
    login(&s.mock, USER, UUID);
    let account = frontend(&s.mock).account(None).unwrap();
    account
        .set_policy(&policy(), Expect::Current, PolicyWrite::Merge)
        .unwrap();
    let fault = account
        .set_policy(&policy(), Expect::Exactly(7), PolicyWrite::Replace)
        .unwrap_err();
    let doc = acquisition_verbs::refusal::document(&fault.into(), &Plain);
    assert_eq!(doc["kind"], "conflict", "{doc}");
    assert_eq!(doc["revision"], 1, "{doc}");
    assert_eq!(
        doc["current"],
        json!(account.policy().unwrap().unwrap()),
        "{doc}"
    );
    assert_eq!(
        doc["error"],
        "the sync policy is at revision 1 (re-read and retry)"
    );
}
