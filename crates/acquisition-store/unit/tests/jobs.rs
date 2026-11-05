use super::*;
use serde_json::json;

fn row(id: u64, state: &str, at: i64) -> JobRow {
    JobRow {
        id,
        kind: "stash".into(),
        state: state.into(),
        priority: 3,
        submitted_by: "test".into(),
        parent: (id > 1).then_some(1),
        retries: 0,
        account: Some("Alice#1234".into()),
        params: json!({ "league": "Standard", "id": format!("t{id}") }),
        outcome: (state == "done").then(|| json!({ "outcome": "success", "payload": {} })),
        deferred: None,
        cancel_requested: false,
        submitted_at: at,
        updated_at: at,
        started_at: None,
        finished_at: None,
    }
}

/// The ledger as schema v1 wrote it: no `started_at` or `finished_at`
/// column, one row of each kind a reader meets, stamped 1.
fn v1_file(path: &Path) {
    let conn = Connection::open(path).unwrap();
    conn.execute_batch(
        "CREATE TABLE jobs (
            id INTEGER PRIMARY KEY AUTOINCREMENT, kind TEXT NOT NULL,
            state TEXT NOT NULL, priority INTEGER NOT NULL,
            submitted_by TEXT NOT NULL, parent INTEGER,
            retries INTEGER NOT NULL, account TEXT, params TEXT NOT NULL,
            outcome TEXT, deferred TEXT,
            cancel_requested INTEGER NOT NULL DEFAULT 0,
            submitted_at INTEGER NOT NULL, updated_at INTEGER NOT NULL);
         INSERT INTO jobs (id, kind, state, priority, submitted_by, retries, params,
                           outcome, submitted_at, updated_at)
         VALUES (1, 'stash', 'waiting', 0, 'test', 0, '{}', NULL, 100, 100),
                (2, 'stash', 'done', 0, 'test', 0, '{}',
                 '{\"outcome\":\"cancelled\"}', 100, 110);
         PRAGMA user_version = 1;",
    )
    .unwrap();
}

/// C165 (a job's record says when): the ledger keeps a job's first
/// start and its finish beside its submission. A file of schema v1, with
/// no such columns, is read by a frontend with no daemon as it stands,
/// its rows with neither time; a daemon's open adds both columns, its
/// old rows with none, and stamps it.
#[test]
fn c165_a_v1_ledger_reads_with_no_start_or_finish_and_a_daemon_adds_the_columns() {
    let temp = tempfile::tempdir().unwrap();
    let path = daemon_db_path(temp.path());
    v1_file(&path);
    let persisted = Persisted::read(temp.path()).unwrap().expect("a ledger");
    assert_eq!(persisted.jobs.len(), 1);
    assert_eq!(
        (persisted.jobs[0].started_at, persisted.jobs[0].finished_at),
        (None, None)
    );
    let record = Persisted::read_job(temp.path(), 2).unwrap().expect("job 2");
    assert_eq!(
        (record.job.submitted_at, record.job.finished_at),
        (100, None)
    );

    let db = JobDb::open(&path).unwrap();
    let rows = db.load().unwrap();
    assert_eq!(
        rows.iter()
            .map(|r| (r.started_at, r.finished_at))
            .collect::<Vec<_>>(),
        [(None, None), (None, None)]
    );
    let mut ran = row(3, "done", 200);
    ran.started_at = Some(201);
    ran.finished_at = Some(205);
    db.upsert(&ran).unwrap();
    assert_eq!(db.get(3).unwrap(), Some(ran));
    let listed = db.listed(3).unwrap().expect("row 3");
    assert_eq!(
        (listed.submitted_at, listed.started_at, listed.finished_at),
        (200, Some(201), Some(205))
    );
    drop(db);
    let stamp: i64 = Connection::open(&path)
        .unwrap()
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap();
    assert_eq!(stamp, JOBS_SCHEMA_VERSION);
    let record = Persisted::read_job(temp.path(), 3).unwrap().expect("job 3");
    let aged = record.job.aged(215);
    assert_eq!(
        (
            aged.started_at,
            aged.started_at_age.as_deref(),
            aged.finished_at,
            aged.finished_at_age.as_deref()
        ),
        (Some(201), Some("14s ago"), Some(205), Some("10s ago"))
    );
}

#[test]
fn rows_round_trip_and_replace_in_place() {
    let db = JobDb::open_memory().unwrap();
    let mut r = row(1, "waiting", 100);
    db.upsert(&r).unwrap();
    r.state = "running".into();
    r.deferred = Some(json!({ "outcome": "success", "payload": { "tabs": 2 } }));
    db.upsert(&r).unwrap();
    db.upsert(&row(2, "waiting", 101)).unwrap();
    assert_eq!(db.load().unwrap(), vec![r, row(2, "waiting", 101)]);
}

/// C6, the table answers for a concluded tree: the listed reads name
/// a row by its leading columns, and the finished range starts at the
/// first id asked and leaves out the open rows.
#[test]
fn listed_reads_name_the_finished_rows_from_an_id_on() {
    let db = JobDb::open_memory().unwrap();
    for (id, state) in [(1, "done"), (2, "failed"), (3, "running"), (4, "cancelled")] {
        db.upsert(&row(id, state, 100)).unwrap();
    }
    let listed = db.listed(2).unwrap().expect("row 2");
    assert_eq!(
        (
            listed.id,
            listed.state.as_str(),
            listed.parent,
            &listed.params
        ),
        (
            2,
            "failed",
            Some(1),
            &json!({ "league": "Standard", "id": "t2" })
        )
    );
    assert_eq!(db.listed(9).unwrap(), None);
    let ids: Vec<u64> = db
        .listed_finished_from(2, 10)
        .unwrap()
        .iter()
        .map(|j| j.id)
        .collect();
    assert_eq!(ids, [2, 4], "from id 2 on, terminal rows only");
    let ids: Vec<u64> = db
        .listed_finished_from(1, 2)
        .unwrap()
        .iter()
        .map(|j| j.id)
        .collect();
    assert_eq!(ids, [1, 2], "a page is the first `limit` of the range");
    let ids: Vec<u64> = db
        .listed_finished(&[4, 3, 1, 9])
        .unwrap()
        .iter()
        .map(|j| j.id)
        .collect();
    assert_eq!(ids, [1, 4], "the ids asked, terminal rows only, by id");
}

#[test]
fn next_id_never_reuses_a_pruned_id() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("daemon.db");
    {
        let db = JobDb::open(&path).unwrap();
        assert_eq!(db.next_id().unwrap(), 1, "an empty table starts at 1");
        db.upsert(&row(7, "done", 0)).unwrap();
        assert_eq!(db.prune(Retention::default(), 10 * 86_400).unwrap(), 1);
        assert!(db.load().unwrap().is_empty());
    }
    let db = JobDb::open(&path).unwrap();
    assert_eq!(
        db.next_id().unwrap(),
        8,
        "the sequence survives prune and reopen"
    );
}

/// Issue #63 (`roadmap/briefs/63-store-remainder.md`, Do 4): the
/// queue's file carries its schema version. A fresh file is stamped;
/// one a build before the stamp wrote (the table, rows, no stamp)
/// opens with its rows and is stamped; one stamped newer than this
/// build is refused in words naming the file and what to do, its
/// stamp untouched.
#[test]
fn the_queue_file_is_stamped_and_a_newer_stamp_is_refused() {
    let temp = tempfile::tempdir().unwrap();
    let stamp = |path: &Path| -> i64 {
        Connection::open(path)
            .unwrap()
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .unwrap()
    };

    let fresh = temp.path().join("fresh.db");
    drop(JobDb::open(&fresh).unwrap());
    assert_eq!(
        stamp(&fresh),
        JOBS_SCHEMA_VERSION,
        "a fresh file is stamped"
    );

    let legacy = temp.path().join("legacy.db");
    {
        let db = JobDb::open(&legacy).unwrap();
        db.upsert(&row(3, "waiting", 100)).unwrap();
    }
    Connection::open(&legacy)
        .unwrap()
        .pragma_update(None, "user_version", 0)
        .unwrap();
    let db = JobDb::open(&legacy).unwrap();
    assert_eq!(
        db.load().unwrap(),
        vec![row(3, "waiting", 100)],
        "an unstamped file opens with its rows"
    );
    drop(db);
    assert_eq!(stamp(&legacy), JOBS_SCHEMA_VERSION, "and is stamped");

    let newer = temp.path().join("newer.db");
    drop(JobDb::open(&newer).unwrap());
    Connection::open(&newer)
        .unwrap()
        .pragma_update(None, "user_version", 99)
        .unwrap();
    let refused = JobDb::open(&newer).map(drop).unwrap_err();
    assert_eq!(
        refused.downcast_ref::<QueueSchemaTooNew>(),
        Some(&QueueSchemaTooNew {
            found: 99,
            supported: JOBS_SCHEMA_VERSION
        })
    );
    let words = format!("{refused:#}");
    assert!(
        words.contains("daemon.db uses schema v99")
            && words.contains("run the build that wrote it"),
        "{words}"
    );
    assert_eq!(stamp(&newer), 99, "a refused file is left as found");
}

#[test]
fn open_rows_are_the_queue_and_get_finds_history() {
    let db = JobDb::open_memory().unwrap();
    db.upsert(&row(1, "done", 0)).unwrap();
    db.upsert(&row(2, "running", 0)).unwrap();
    db.upsert(&row(3, "waiting", 0)).unwrap();
    db.upsert(&row(4, "failed", 0)).unwrap();
    let mut child = row(5, "done", 0);
    child.parent = Some(2);
    db.upsert(&child).unwrap();
    let open: Vec<u64> = db.load_open().unwrap().iter().map(|r| r.id).collect();
    assert_eq!(open, [2, 3, 5], "an open parent's finished child comes too");
    assert_eq!(db.get(1).unwrap(), Some(row(1, "done", 0)));
    assert_eq!(db.get(9).unwrap(), None);
}

#[test]
fn prune_keeps_failures_longer_than_successes() {
    let db = JobDb::open_memory().unwrap();
    let day = 86_400;
    let now = 100 * day;
    db.upsert(&row(1, "done", now - 8 * day)).unwrap();
    db.upsert(&row(2, "cancelled", now - 8 * day)).unwrap();
    db.upsert(&row(3, "failed", now - 8 * day)).unwrap();
    db.upsert(&row(4, "failed", now - 31 * day)).unwrap();
    db.upsert(&row(5, "done", now - 6 * day)).unwrap();
    db.upsert(&row(6, "waiting", now - 40 * day)).unwrap();
    let mut child = row(7, "done", now - 40 * day);
    child.parent = Some(6);
    db.upsert(&child).unwrap();
    assert_eq!(db.prune(Retention::default(), now).unwrap(), 3);
    let left: Vec<u64> = db.load().unwrap().iter().map(|r| r.id).collect();
    assert_eq!(
        left,
        [3, 5, 6, 7],
        "a waiting row is never pruned by age, nor an open parent's child"
    );
}

/// C137: the store wait at zero (`ACQ_STORE_WAIT=0`) refuses a writer
/// the queue file's lock is held from at once, as busy, the wait read
/// back from [`JobDb::open`]'s connection so a knob unwired from its
/// busy timeout fails here (the shape of the fact store's pin, in
/// `unit/tests/lib.rs`, which runs the test again as a child with the
/// knob set).
#[test]
fn c137_the_store_wait_at_zero_refuses_a_held_queue_writer_at_once() {
    if crate::tests::ran_as_a_child_with_the_store_wait_at_zero(
        module_path!(),
        "c137_the_store_wait_at_zero_refuses_a_held_queue_writer_at_once",
    ) {
        return;
    }
    let temp = tempfile::tempdir().unwrap();
    let path = daemon_db_path(temp.path());
    let db = JobDb::open(&path).unwrap();
    assert_eq!(
        crate::annotations::enforced_wait(&db.conn),
        Some(std::time::Duration::ZERO),
        "the wait the connection enforced, not the default"
    );
    // `b` holds the write lock in an open transaction the whole time.
    let b = Connection::open(&path).unwrap();
    b.execute_batch("BEGIN IMMEDIATE").unwrap();
    let err = db.upsert(&row(1, "waiting", 100)).unwrap_err();
    assert!(crate::tests::refused_as_busy(&err), "{err:?}");
    b.execute_batch("ROLLBACK").unwrap();
    db.upsert(&row(1, "waiting", 100)).unwrap();
    assert_eq!(db.load().unwrap().len(), 1);
}

/// C137: the store wait at zero refuses a frontend's read-only read of
/// the queue at once when a writer holds the file exclusively (a
/// rollback-journal file; a WAL file's readers never wait). The wait is
/// read back from the one read-only opener both [`Persisted::read`]
/// and [`Persisted::read_job`] share, so a knob unwired from it fails
/// here, and the door itself is refused through it.
#[test]
fn c137_the_store_wait_at_zero_refuses_a_blocked_queue_reader_at_once() {
    if crate::tests::ran_as_a_child_with_the_store_wait_at_zero(
        module_path!(),
        "c137_the_store_wait_at_zero_refuses_a_blocked_queue_reader_at_once",
    ) {
        return;
    }
    let temp = tempfile::tempdir().unwrap();
    let path = daemon_db_path(temp.path());
    drop(JobDb::open(&path).unwrap());
    crate::tests::rollback_mode(&path);
    let reader = open_read_only(&path).unwrap();
    assert_eq!(
        crate::annotations::enforced_wait(&reader),
        Some(std::time::Duration::ZERO),
        "the wait the connection enforced, not the default"
    );
    let b = Connection::open(&path).unwrap();
    b.execute_batch("BEGIN EXCLUSIVE").unwrap();
    let err = reader
        .query_row("SELECT COUNT(*) FROM jobs", [], |r| r.get::<_, i64>(0))
        .unwrap_err();
    assert!(
        matches!(
            &err,
            rusqlite::Error::SqliteFailure(f, _) if f.code == rusqlite::ErrorCode::DatabaseBusy
        ),
        "{err:?}"
    );
    let err = Persisted::read(temp.path()).unwrap_err();
    assert!(crate::tests::refused_as_busy(&err), "{err:?}");
    let err = Persisted::read_job(temp.path(), 1).unwrap_err();
    assert!(crate::tests::refused_as_busy(&err), "{err:?}");
    b.execute_batch("ROLLBACK").unwrap();
    assert!(Persisted::read(temp.path()).unwrap().is_some());
}
