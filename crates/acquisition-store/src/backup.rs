//! A checked copy of one SQLite file, the one mechanism under every
//! backup and every restore (C35, backup is a store-managed snapshot;
//! C201, the history in an account's facts is state the tool keeps).
//!
//! `copy_checked` writes a consistent snapshot of an open connection's
//! file where nothing stands: [`crate::annotations::Annotations::export`]
//! and [`crate::Store::export`] are it for the intent files and the
//! facts, and a restore is the same copy pointed back, from a backup's
//! file opened through its read-only door to the store's path.
//!
//! A raw file copy under WAL is no backup, and neither is a file written
//! beside another file's log: a `-wal` left by a file moved aside would be
//! replayed into the copy at its next open. So a destination stands when
//! the file or either of its companions does ([`standing`]), and nothing
//! is written there.

use std::path::{Path, PathBuf};

use rusqlite::Connection;

/// The file at `path` and the two companions SQLite keeps beside it under
/// WAL, `-wal` and `-shm`, in that order: what a person moves aside
/// together.
pub fn sqlite_files(path: &Path) -> [PathBuf; 3] {
    let with = |suffix: &str| {
        let mut name = path.as_os_str().to_owned();
        name.push(suffix);
        PathBuf::from(name)
    };
    [path.to_path_buf(), with("-wal"), with("-shm")]
}

/// Which of [`sqlite_files`] stand at `path`: the destination is taken if
/// any is. A file whose presence cannot be told is counted as standing,
/// never taken for absent.
pub fn standing(path: &Path) -> Vec<PathBuf> {
    sqlite_files(path)
        .into_iter()
        .filter(|file| file.try_exists().unwrap_or(true))
        .collect()
}

/// A consistent snapshot of `conn`'s file at `dest`, via SQLite's
/// `VACUUM INTO`, published atomically, or why not. `VACUUM INTO` writes
/// its file directly and never fsyncs it, so an interrupted copy would
/// leave a partial file that both looks whole and blocks every retry. The
/// copy is therefore written to a partial named for this invocation
/// (`<dest>.partial-<pid>-<n>-<nanos>`, so two copies never share or
/// remove each other's), checked with `quick_check`, fsynced, then
/// published by `hard_link`, which the kernel refuses if `dest` appeared
/// meanwhile: two copies to one destination end with one file and one
/// `AlreadyExists`, never a replaced file. Only on a filesystem without
/// hard links does a `rename` stand in, and that fallback is the one
/// place a simultaneous copy could still replace another's. On unix the
/// publish ends with the directory synced, so the link is durable as the
/// file is; Windows cannot sync a directory and the publish stands
/// unsynced there. Refused as `AlreadyExists` before anything is written
/// where any of [`sqlite_files`] stands at `dest`; the partial is removed
/// either way. The copy is in rollback-journal mode: reading it leaves no
/// `-wal` beside it, and the store's writing door makes it WAL at its
/// first open.
pub(crate) fn copy_checked(conn: &Connection, dest: &Path) -> std::io::Result<()> {
    if let Some(dir) = dest.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let exists = |found: &Path| {
        std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            match found == dest {
                true => format!("{} exists; a copy never overwrites a file", dest.display()),
                false => format!(
                    "{} stands beside {}; a copy is never written beside another file's log",
                    found.display(),
                    dest.display()
                ),
            },
        )
    };
    if let Some(found) = standing(dest).first() {
        return Err(exists(found));
    }
    // The partial's name is unique per process and per copy within it:
    // the clock alone is not — two threads copying within one tick
    // (microseconds on macOS) shared a name, one VACUUM failed on the
    // other's file and its cleanup removed that file before it was
    // published, so both were refused (the C35 race test, one run in
    // twelve).
    static COPIES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let mut partial = dest.as_os_str().to_owned();
    partial.push(format!(
        ".partial-{}-{}-{}",
        std::process::id(),
        COPIES.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    let partial = PathBuf::from(partial);
    let published = copy_to(conn, &partial).and_then(|()| {
        match std::fs::hard_link(&partial, dest) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Err(exists(dest)),
            // No hard links here (FAT and the like): a rename is the
            // best available, guarded by a last look.
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::Unsupported | std::io::ErrorKind::PermissionDenied
                ) =>
            {
                match standing(dest).first() {
                    Some(found) => Err(exists(found)),
                    None => std::fs::rename(&partial, dest),
                }
            }
            Err(e) => Err(e),
        }
        .and_then(|()| crate::sync_dir(dest.parent()))
    });
    let _ = std::fs::remove_file(&partial);
    published
}

/// The copy itself: vacuumed into `partial`, integrity-checked through
/// a fresh connection, and fsynced.
fn copy_to(conn: &Connection, partial: &Path) -> std::io::Result<()> {
    let sql = |e: rusqlite::Error| std::io::Error::other(e.to_string());
    conn.execute("VACUUM INTO ?1", [partial.to_string_lossy()])
        .map_err(sql)?;
    let verdict: String = Connection::open(partial)
        .and_then(|c| c.query_row("PRAGMA quick_check", [], |r| r.get(0)))
        .map_err(sql)?;
    if verdict != "ok" {
        return Err(std::io::Error::other(format!(
            "the copy {} failed its integrity check: {verdict}",
            partial.display()
        )));
    }
    // Opened for writing: Windows refuses to flush a read-only handle.
    std::fs::OpenOptions::new()
        .write(true)
        .open(partial)?
        .sync_all()
}

/// `PRAGMA quick_check` on an open connection, `ok` or what it found:
/// a backup's file is checked before a restore writes anything.
pub(crate) fn quick_check(conn: &Connection) -> rusqlite::Result<Option<String>> {
    let verdict: String = conn.query_row("PRAGMA quick_check", [], |r| r.get(0))?;
    Ok((verdict != "ok").then_some(verdict))
}
