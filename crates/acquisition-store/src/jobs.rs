//! The persisted job queue: `store/<provider>/daemon.db`, one row per job,
//! written by the daemon at every state change and read back when a
//! daemon starts. The daemon's memory is the source of truth for its live
//! trees; this table is their mirror, and answers for a tree that has
//! concluded and left memory (C6). The queue survives a restart —
//! idle exit, `daemon stop`, a version respawn, a crash. What a
//! restarting daemon does with each row — resume, replay, hold, or fail
//! as interrupted — is its decision, not this crate's.
//!
//! This crate knows nothing about job semantics (what `running` means for
//! a restart, which kinds are per-lifetime); it stores rows and hands them
//! back. Those decisions are the daemon's (C6). Ids come from
//! `AUTOINCREMENT`, so an id is never reused even after its row is pruned:
//! a stale `acq result <id>` names nothing rather than a different job.
//!
//! The daemon is this table's only writer, through [`JobDb`]; a frontend
//! asks the daemon (`acq jobs`, `acq result`) while one answers, and reads
//! the table through [`Persisted`] only when none does.
//!
//! # Decisions as recorded
//!
//! The rulings are the decision registry — `decisions/daemon.md` for this
//! area, `AGENTS.md` for the cross-cutting ones (`C<n>`); what follows is
//! the entry's full text as recorded there, kept beside the code that
//! implements it. The registry is current.
//!
//! ## C45 — With no daemon to ask, an observer reads the queue from the ledger on disk
//!
//! **With no daemon to ask, an observer reads the queue from the ledger on
//! disk — what waits, what was recorded running, when it was last written
//! — never "no outstanding work".** This is the observe tier's absent arm
//! (C10): `acq jobs` and `acq daemon status`, the MCP's `list_jobs` and
//! `daemon_status`, answer `running: false` with the persisted queue beside
//! it, read through [`Persisted`] — a read-only connection that creates
//! nothing (no file, no schema, no world) and reads none of the bulk
//! columns (a parent's held `deferred` result can be a whole refresh) —
//! never through the daemon's open [`JobDb`], and never while a daemon
//! answers: memory is the truth then (C6), and the daemon's list carries
//! what disk cannot (ETAs, re-queue counts as they move). Rationale: C25
//! makes a halted daemon idle out with its queue on disk, and once
//! observation stopped spawning (C10, 2026-09-09) the halt procedure's
//! `acq jobs` answered only "not running" — for a ceiling halt the one way
//! to see the queue was to respawn it, which is exactly what the procedure
//! says to look before doing. Decided 2026-08-31 (a facade, unbuilt);
//! amended and built 2026-09-11.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rusqlite::{Connection, OpenFlags, OptionalExtension, params};
use serde::Serialize;
use serde_json::Value;

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS jobs (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    kind             TEXT    NOT NULL,
    state            TEXT    NOT NULL,   -- waiting|running|done|failed|cancelled
    priority         INTEGER NOT NULL,
    submitted_by     TEXT    NOT NULL,
    parent           INTEGER,
    retries          INTEGER NOT NULL,
    account          TEXT,
    params           TEXT    NOT NULL,   -- verbatim JSON
    outcome          TEXT,               -- JSON, set once terminal
    deferred         TEXT,               -- a parent's held-back outcome while its children run
    cancel_requested INTEGER NOT NULL DEFAULT 0,
    submitted_at     INTEGER NOT NULL,   -- unix seconds
    updated_at       INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS jobs_state ON jobs (state);
";

/// One job as the table holds it. Column-for-column with the daemon's
/// `JobInfo` plus the restart-relevant runtime fields.
#[derive(Clone, Debug, PartialEq)]
pub struct JobRow {
    pub id: u64,
    pub kind: String,
    pub state: String,
    pub priority: u8,
    pub submitted_by: String,
    pub parent: Option<u64>,
    pub retries: u32,
    pub account: Option<String>,
    pub params: Value,
    pub outcome: Option<Value>,
    pub deferred: Option<Value>,
    pub cancel_requested: bool,
    pub submitted_at: i64,
    pub updated_at: i64,
}

/// One job as a list names it: the columns that lead the row, before its
/// bulk ones (`outcome`, `deferred`, which can be a whole refresh), so a
/// read of many never walks a body. Column for column the daemon's
/// `JobInfo`, its ETA aside: what the daemon answers `status` and its job
/// list with for a job it no longer holds in memory (C6, a concluded tree
/// leaves memory and the table answers for it).
#[derive(Clone, Debug, PartialEq)]
pub struct ListedJob {
    pub id: u64,
    pub kind: String,
    pub state: String,
    pub priority: u8,
    pub submitted_by: String,
    pub parent: Option<u64>,
    pub retries: u32,
    pub account: Option<String>,
    pub params: Value,
}

/// How long terminal rows are kept, in days, by final state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Retention {
    pub done_days: u32,
    pub failed_days: u32,
}

impl Default for Retention {
    fn default() -> Self {
        Retention {
            done_days: 7,
            failed_days: 30,
        }
    }
}

pub fn daemon_db_path(dir: &Path) -> PathBuf {
    dir.join("daemon.db")
}

pub struct JobDb {
    conn: Connection,
    path: PathBuf,
}

impl JobDb {
    pub fn open(path: &Path) -> Result<JobDb> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
        }
        let conn = Connection::open(path).with_context(|| format!("opening {}", path.display()))?;
        Self::init(conn, path.to_path_buf())
    }

    pub fn open_memory() -> Result<JobDb> {
        Self::init(Connection::open_in_memory()?, PathBuf::from(":memory:"))
    }

    fn init(conn: Connection, path: PathBuf) -> Result<JobDb> {
        crate::ensure_wal(&conn)?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.busy_timeout(std::time::Duration::from_secs(5))?;
        conn.execute_batch(SCHEMA)?;
        Ok(JobDb { conn, path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Checkpoint and truncate the queue's WAL ([`crate::Store::checkpoint`]
    /// says why): the daemon calls it on its way out.
    pub fn checkpoint(&self) -> Result<crate::Checkpoint> {
        crate::checkpoint(&self.conn)
    }

    /// Insert or replace the row for `row.id`. One small transaction; the
    /// daemon calls this under the lock that guards its in-memory job, so
    /// the table sees every change in the order memory did.
    pub fn upsert(&self, row: &JobRow) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO jobs
             (id, kind, state, priority, submitted_by, parent, retries, account, params,
              outcome, deferred, cancel_requested, submitted_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                row.id,
                row.kind,
                row.state,
                row.priority,
                row.submitted_by,
                row.parent,
                row.retries,
                row.account,
                row.params.to_string(),
                row.outcome.as_ref().map(Value::to_string),
                row.deferred.as_ref().map(Value::to_string),
                row.cancel_requested,
                row.submitted_at,
                row.updated_at,
            ],
        )?;
        Ok(())
    }

    /// One row by id, if it exists.
    pub fn get(&self, id: u64) -> Result<Option<JobRow>> {
        let mut rows = self.select("WHERE id = ?1", params![id])?;
        Ok(rows.pop())
    }

    pub fn delete(&self, id: u64) -> Result<()> {
        self.conn
            .execute("DELETE FROM jobs WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Every row, by id.
    pub fn load(&self) -> Result<Vec<JobRow>> {
        self.select("", [])
    }

    /// The rows a restarting daemon takes back: everything not yet
    /// terminal, plus the finished children of those (a parent's own
    /// result summarizes its children, so a parent held open needs them;
    /// one level suffices — a parent only finishes once every descendant
    /// has).
    pub fn load_open(&self) -> Result<Vec<JobRow>> {
        self.select(
            "WHERE state IN ('waiting', 'running')
                OR parent IN (SELECT id FROM jobs WHERE state IN ('waiting', 'running'))",
            [],
        )
    }

    fn select(&self, filter: &str, args: impl rusqlite::Params) -> Result<Vec<JobRow>> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT id, kind, state, priority, submitted_by, parent, retries, account, params,
                    outcome, deferred, cancel_requested, submitted_at, updated_at
             FROM jobs {filter} ORDER BY id"
        ))?;
        let rows = stmt.query_map(args, |r| {
            Ok(JobRow {
                id: r.get(0)?,
                kind: r.get(1)?,
                state: r.get(2)?,
                priority: r.get(3)?,
                submitted_by: r.get(4)?,
                parent: r.get(5)?,
                retries: r.get(6)?,
                account: r.get(7)?,
                params: parse(r.get::<_, String>(8)?),
                outcome: r.get::<_, Option<String>>(9)?.map(parse),
                deferred: r.get::<_, Option<String>>(10)?.map(parse),
                cancel_requested: r.get(11)?,
                submitted_at: r.get(12)?,
                updated_at: r.get(13)?,
            })
        })?;
        Ok(rows.collect::<std::result::Result<_, _>>()?)
    }

    /// One row by id as a list names it ([`ListedJob`]).
    pub fn listed(&self, id: u64) -> Result<Option<ListedJob>> {
        let mut rows = self.list_where("id = ?1", params![id])?;
        Ok(rows.pop())
    }

    /// The terminal rows of id `first` and after, by id, as a list names
    /// them: a daemon's list reads its own work from the first id it
    /// issued. A range of the primary key, read without the bulk columns.
    pub fn listed_finished_from(&self, first: u64) -> Result<Vec<ListedJob>> {
        self.list_where(
            "id >= ?1 AND state IN ('done', 'failed', 'cancelled')",
            params![first],
        )
    }

    /// The terminal rows among `ids`, by id, as a list names them: a
    /// daemon's list reads what its restore brought back by the ids the
    /// restore took, one primary-key lookup each, never the rows between.
    pub fn listed_finished(&self, ids: &[u64]) -> Result<Vec<ListedJob>> {
        let mut stmt = self.conn.prepare_cached(
            "SELECT id, kind, state, priority, submitted_by, parent, retries, account, params
             FROM jobs WHERE id = ?1 AND state IN ('done', 'failed', 'cancelled')",
        )?;
        let mut rows = Vec::new();
        for &id in ids {
            if let Some(row) = stmt.query_row(params![id], listed_row).optional()? {
                rows.push(row);
            }
        }
        rows.sort_by_key(|row| row.id);
        Ok(rows)
    }

    fn list_where(&self, filter: &str, args: impl rusqlite::Params) -> Result<Vec<ListedJob>> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT id, kind, state, priority, submitted_by, parent, retries, account, params
             FROM jobs WHERE {filter} ORDER BY id"
        ))?;
        let rows = stmt.query_map(args, listed_row)?;
        Ok(rows.collect::<std::result::Result<_, _>>()?)
    }

    /// The next id a new job gets: one past the largest ever issued, pruned
    /// rows included (`sqlite_sequence` remembers what the table forgot).
    pub fn next_id(&self) -> Result<u64> {
        let seq: Option<u64> = self
            .conn
            .query_row(
                "SELECT seq FROM sqlite_sequence WHERE name = 'jobs'",
                [],
                |r| r.get(0),
            )
            .optional()?;
        Ok(seq.unwrap_or(0) + 1)
    }

    /// Delete terminal rows older than the retention for their state, as
    /// of `now` (unix seconds). A finished child of a still-open parent is
    /// live state, not history, and stays. Returns how many went.
    pub fn prune(&self, retention: Retention, now: i64) -> Result<usize> {
        let day = 86_400i64;
        let done_before = now - i64::from(retention.done_days) * day;
        let failed_before = now - i64::from(retention.failed_days) * day;
        let n = self.conn.execute(
            "DELETE FROM jobs
             WHERE ((state IN ('done', 'cancelled') AND updated_at < ?1)
                 OR (state = 'failed' AND updated_at < ?2))
               AND (parent IS NULL
                    OR parent NOT IN (SELECT id FROM jobs WHERE state IN ('waiting', 'running')))",
            params![done_before, failed_before],
        )?;
        Ok(n)
    }
}

/// One open job as the ledger holds it, for a reader with no daemon to
/// ask (C45): the identifying and restart-relevant columns, none of the
/// bulk ones. Serializes as the `persisted.jobs` entries of `acq jobs`
/// and the MCP's `list_jobs` when `running` is false.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PersistedJob {
    pub id: u64,
    pub kind: String,
    /// `waiting` or `running` — as recorded at the last write, which for
    /// `running` means "was running when the daemon left"; its successor
    /// decides what that becomes (C6).
    pub state: String,
    pub priority: u8,
    pub submitted_by: String,
    pub parent: Option<u64>,
    pub retries: u32,
    pub account: Option<String>,
    /// Verbatim, and public like `JobInfo::params` (C7).
    pub params: Value,
    pub cancel_requested: bool,
    pub submitted_at: i64,
    pub updated_at: i64,
}

/// The queue as the ledger holds it, read with no daemon running (C45).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Persisted {
    /// Every non-terminal row, by id.
    pub jobs: Vec<PersistedJob>,
    /// When any row was last written (unix seconds): how old this
    /// picture is. `None` for a table with no rows at all.
    pub written_at: Option<i64>,
}

/// One job of any state as the ledger holds it, with its outcome once
/// terminal — what `acq status <id>` and `acq result <id>` answer with no
/// daemon to ask (C45): a finished job's result is on disk for as long as
/// the daemon's retention keeps it, and a reader should not need a daemon
/// started to read it.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PersistedRecord {
    /// The row; its `state` is any of the five, as recorded.
    pub job: PersistedJob,
    /// The outcome as the daemon wrote it (`{"outcome": …}`), `None`
    /// until the job is terminal.
    pub outcome: Option<Value>,
}

/// The world at `root` as an observer finds it (C83: creating nothing):
/// `None` when the root does not exist, an error for one that cannot be
/// resolved.
fn observed_world(root: &Path) -> Result<Option<crate::world::World>> {
    match crate::world::World::observe_at(root) {
        Ok(world) => Ok(Some(world)),
        Err(e) if e.absent => Ok(None),
        Err(e) => Err(anyhow::anyhow!("{e}")),
    }
}

impl Persisted {
    /// One job of the provider's ledger in this shell's world, or `None`
    /// when there is no ledger or it holds no such id. Same door and same
    /// errors as [`Persisted::observe`].
    pub fn observe_job(provider: &str, id: u64) -> Result<Option<PersistedRecord>> {
        Self::observe_job_at(&crate::world::intended_root(), provider, id)
    }

    /// C147, a ledger read takes the world it acts on:
    /// [`Persisted::observe_job`] in the world at `root`, the one a
    /// frontend captured, not the one the environment names now.
    pub fn observe_job_at(root: &Path, provider: &str, id: u64) -> Result<Option<PersistedRecord>> {
        match observed_world(root)? {
            Some(world) => Self::read_job(&world.provider_dir(provider), id),
            None => Ok(None),
        }
    }

    /// One job of the ledger under `dir`, read-only as [`Persisted::read`].
    pub fn read_job(dir: &Path, id: u64) -> Result<Option<PersistedRecord>> {
        let path = daemon_db_path(dir);
        if !path.exists() {
            return Ok(None);
        }
        let conn = Connection::open_with_flags(
            &path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .with_context(|| format!("opening {} read-only", path.display()))?;
        conn.busy_timeout(std::time::Duration::from_secs(5))?;
        conn.query_row(
            "SELECT id, kind, state, priority, submitted_by, parent, retries, account,
                    params, cancel_requested, submitted_at, updated_at, outcome
             FROM jobs WHERE id = ?1",
            [id],
            |r| {
                Ok(PersistedRecord {
                    job: PersistedJob {
                        id: r.get(0)?,
                        kind: r.get(1)?,
                        state: r.get(2)?,
                        priority: r.get(3)?,
                        submitted_by: r.get(4)?,
                        parent: r.get(5)?,
                        retries: r.get(6)?,
                        account: r.get(7)?,
                        params: parse(r.get::<_, String>(8)?),
                        cancel_requested: r.get(9)?,
                        submitted_at: r.get(10)?,
                        updated_at: r.get(11)?,
                    },
                    outcome: r.get::<_, Option<String>>(12)?.map(parse),
                })
            },
        )
        .optional()
        .with_context(|| format!("reading job {id} in {}", path.display()))
    }

    /// The provider's ledger in this shell's world, or `None` when there
    /// is nothing to read: no world root yet (an observer creates none,
    /// C83) or no `daemon.db` under it. Anything else that goes wrong
    /// — a root that is not a directory, a file that is not a database —
    /// is an error, never "no jobs" (C47).
    pub fn observe(provider: &str) -> Result<Option<Persisted>> {
        Self::observe_at(&crate::world::intended_root(), provider)
    }

    /// C147, a ledger read takes the world it acts on:
    /// [`Persisted::observe`] in the world at `root`, the one a frontend
    /// captured, not the one the environment names now.
    pub fn observe_at(root: &Path, provider: &str) -> Result<Option<Persisted>> {
        match observed_world(root)? {
            Some(world) => Self::read(&world.provider_dir(provider)),
            None => Ok(None),
        }
    }

    /// The ledger under `dir` (`daemon.db`, the daemon's store directory
    /// for a provider), or `None` when the file does not exist. Opened
    /// read-only: no schema is created and a missing table is an error.
    pub fn read(dir: &Path) -> Result<Option<Persisted>> {
        let path = daemon_db_path(dir);
        if !path.exists() {
            return Ok(None);
        }
        let conn = Connection::open_with_flags(
            &path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .with_context(|| format!("opening {} read-only", path.display()))?;
        conn.busy_timeout(std::time::Duration::from_secs(5))?;
        let context = || format!("reading the queue in {}", path.display());
        let written_at: Option<i64> = conn
            .query_row("SELECT MAX(updated_at) FROM jobs", [], |r| r.get(0))
            .with_context(context)?;
        let mut stmt = conn
            .prepare(
                "SELECT id, kind, state, priority, submitted_by, parent, retries, account,
                        params, cancel_requested, submitted_at, updated_at
                 FROM jobs WHERE state IN ('waiting', 'running') ORDER BY id",
            )
            .with_context(context)?;
        let rows = stmt
            .query_map([], |r| {
                Ok(PersistedJob {
                    id: r.get(0)?,
                    kind: r.get(1)?,
                    state: r.get(2)?,
                    priority: r.get(3)?,
                    submitted_by: r.get(4)?,
                    parent: r.get(5)?,
                    retries: r.get(6)?,
                    account: r.get(7)?,
                    params: parse(r.get::<_, String>(8)?),
                    cancel_requested: r.get(9)?,
                    submitted_at: r.get(10)?,
                    updated_at: r.get(11)?,
                })
            })
            .with_context(context)?;
        let jobs = rows
            .collect::<std::result::Result<Vec<_>, _>>()
            .with_context(context)?;
        Ok(Some(Persisted { jobs, written_at }))
    }

    /// Rows recorded `waiting`.
    pub fn waiting(&self) -> usize {
        self.jobs.iter().filter(|j| j.state == "waiting").count()
    }

    /// Rows recorded `running` when the daemon left.
    pub fn recorded_running(&self) -> usize {
        self.jobs.iter().filter(|j| j.state == "running").count()
    }
}

/// One [`ListedJob`] from a select of its nine leading columns.
fn listed_row(r: &rusqlite::Row) -> rusqlite::Result<ListedJob> {
    Ok(ListedJob {
        id: r.get(0)?,
        kind: r.get(1)?,
        state: r.get(2)?,
        priority: r.get(3)?,
        submitted_by: r.get(4)?,
        parent: r.get(5)?,
        retries: r.get(6)?,
        account: r.get(7)?,
        params: parse(r.get::<_, String>(8)?),
    })
}

fn parse(text: String) -> Value {
    serde_json::from_str(&text).unwrap_or(Value::Null)
}

#[cfg(test)]
mod tests {
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
        }
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
            .listed_finished_from(2)
            .unwrap()
            .iter()
            .map(|j| j.id)
            .collect();
        assert_eq!(ids, [2, 4], "from id 2 on, terminal rows only");
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
}
