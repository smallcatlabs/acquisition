//! A backup a person makes and puts back (C35, backup is a store-managed
//! snapshot; C201, the history in an account's facts is state the tool
//! keeps): an account's facts, its intent (prices and the sync policy)
//! and the world's saved searches (C167), each a checked copy of its file
//! (`acquisition_store::backup`), written where nothing stands. Backup and
//! restore are that one copy pointed two ways; neither overwrites, and no
//! merge exists.
//!
//! - **A backup is whole or it is no backup.** [`backup`] makes the
//!   directory it is named, refusing one that exists, so nothing else
//!   writes inside it; it copies each file the store holds, then writes
//!   its [`Manifest`] last, through a partial file and a link, so a
//!   half-written manifest never stands. A directory with no manifest is
//!   an interrupted backup: [`restore`] refuses it, saying to remove it.
//!   No resume, no repair. A failed backup removes the directory it made.
//! - **The manifest counts what the backup holds**, read from the copies
//!   themselves: the account's name and uuid, when it was made, the build,
//!   and for each of the three files what it holds, or `null` where the
//!   store had none, in which case nothing is made for it in either
//!   direction.
//! - **A restore checks everything, then puts back each file the store
//!   lacks** ([`restore`]). Before the first file is written: the
//!   manifest reads, its uuid is the account's, and each file it names
//!   opens through its read-only door, passes `quick_check`, holds what
//!   the manifest counts, and, for the facts, records no other account. A
//!   file that stands is no failure: the answer names it beside each one
//!   put back, with what a person moves aside before it can be restored:
//!   the file, and its `-wal` and `-shm` where they stand. A store file stands when it or its `-wal` or `-shm` does:
//!   a copy written beside another file's log would take that log's
//!   frames at its next open. Run again, a restore puts back what is still
//!   lacking. It writes no index entry: a store whose files are all gone
//!   is logged into first, and the facts file the login made is moved
//!   aside before the restore.
//! - **C193** (a frontend's writes of the facts go through one door in
//!   the verbs crate, which refuses while a daemon is live): a restore
//!   writes facts, so for its whole length it holds the world lock a
//!   daemon's start takes (`acquisition_store::world::Lock` on the
//!   world's `daemon.lock`, as the offline tripwire reset holds it). A
//!   live daemon holds it, so the restore refuses and says to stop the
//!   daemon; while the restore holds it, no daemon starts and no second
//!   restore runs. The lock is the kernel's: a restore that dies leaves
//!   none held. A frontend writing intent directly (the pages) is not
//!   held by it, but every copy is published by `hard_link`, which the
//!   kernel refuses where a file appeared meanwhile, so such a race ends
//!   with that file named as standing, never replaced.
//! - **CLI only** (no MCP tool): each verb answers one document (C149),
//!   [`BackupMade`] and [`Restored`], whose text leads with what was kept
//!   or put back, counted (C146).

use std::io::Write as _;
use std::path::{Path, PathBuf};

use acquisition_store::backup::standing;
use acquisition_store::world::{Lock, World};
use acquisition_store::{
    Annotations, BUYOUT_KIND, SAVED_SEARCH_SCOPE, SYNC_POLICY_KEY, SYNC_POLICY_KIND,
    SYNC_POLICY_SCOPE, Store, User, account_path, annotations_path, user_path,
};
use serde::{Deserialize, Serialize};
use serde_json::Map;

use crate::fault::Fault;
use crate::frontend::Account;
use crate::refusal::Pointed;

/// The manifest's name inside a backup's directory.
pub const MANIFEST: &str = "manifest.json";

/// The manifest's own format, written into every one and refused where
/// another stands: a later build's backup is not read as this one's.
pub const FORMAT: i64 = 1;

/// What a backup holds, written last into its directory (decision 2 of
/// the store-backup step: a directory without one is no backup).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    /// [`FORMAT`].
    pub format: i64,
    /// The account the backup is of.
    pub account: Named,
    /// When it was made, seconds since 1970, UTC.
    pub made_at: i64,
    /// The build that made it.
    pub build: Build,
    /// The account's facts file, or `null` where the store had none.
    pub facts: Option<FactsHeld>,
    /// The account's intent file, or `null` where the store had none.
    pub intent: Option<IntentHeld>,
    /// The world's saved searches, or `null` where the store had none.
    pub saved: Option<SavedHeld>,
}

/// An account by its name and its uuid.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Named {
    pub name: String,
    pub uuid: String,
}

/// A build by its version and its contract revision.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Build {
    pub version: String,
    pub contract: String,
}

/// The facts file a backup holds: its name in the directory, its items
/// on record now, the items since removed, and the item events.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FactsHeld {
    pub file: String,
    pub items: i64,
    pub removed_items: i64,
    pub events: i64,
}

/// The intent file a backup holds: its name in the directory, the prices,
/// whether a sync policy is among them, and every live row, those two
/// kinds among them. A row this build cannot read is in the file, so it
/// is in each count (C146, a count says what it counts) and counted again
/// apart, under `unreadable` (C151): never read as absence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntentHeld {
    pub file: String,
    pub prices: usize,
    pub policy: bool,
    pub rows: usize,
    /// The live rows among `rows` this build cannot read.
    pub unreadable: usize,
}

/// The user file a backup holds: its name in the directory and the saved
/// searches in it, a search this build cannot read among them and counted
/// again apart (C146, C151).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedHeld {
    pub file: String,
    pub saved_searches: usize,
    /// The saved searches among `saved_searches` this build cannot read.
    pub unreadable: usize,
}

/// A backup made: where, and its manifest whole.
#[derive(Clone, Debug, Serialize)]
pub struct BackupMade {
    /// The directory the backup made.
    pub dir: String,
    /// When it was made, in words at the answer's time (C53).
    pub made_at_age: String,
    pub manifest: Manifest,
}

/// One of the three files, by the word a backup names it with.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Held {
    Facts,
    Intent,
    Saved,
}

/// A file a restore put back: which, and where in the store.
#[derive(Clone, Debug, Serialize)]
pub struct PutBack {
    pub file: Held,
    pub path: String,
}

/// A file a restore left alone because the store holds it: which, where,
/// and what stands there, to be moved aside together before it can be
/// restored.
#[derive(Clone, Debug, Serialize)]
pub struct Standing {
    pub file: Held,
    pub path: String,
    /// What stands now, each to be moved aside: of the file, its `-wal`
    /// and its `-shm`, those present.
    pub move_aside: Vec<String>,
}

/// A restore's answer: what it put back and what it left standing,
/// counted, and the backup's manifest whole.
#[derive(Clone, Debug, Serialize)]
pub struct Restored {
    /// The backup's directory.
    pub dir: String,
    /// When the backup was made, in words at the answer's time (C53).
    pub made_at_age: String,
    /// The files put back, in the manifest's order.
    pub restored: Vec<PutBack>,
    /// The files left standing, in the manifest's order.
    pub standing: Vec<Standing>,
    /// The files the backup holds none of (`null` in its manifest), for
    /// which nothing was made.
    pub absent: Vec<Held>,
    pub manifest: Manifest,
}

/// A refusal in the request's words, with no way on to offer: what a
/// person does is in its words.
fn refused(kind: &'static str, message: String) -> anyhow::Error {
    Pointed {
        kind,
        message,
        ways: Vec::new(),
        fields: Map::new(),
    }
    .into()
}

/// The account's uuid, or [`Fault::NoUuid`] for an entry that predates
/// uuid-at-login.
fn uuid_of(account: &Account) -> Result<String, Fault> {
    account.entry.uuid.clone().ok_or_else(|| Fault::NoUuid {
        username: account.entry.username.clone(),
    })
}

/// The file's name, as a manifest records it.
fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// What a backup's copy of the facts holds, read from the copy.
fn facts_held(path: &Path) -> anyhow::Result<FactsHeld> {
    let status = Store::open_read_only(path)?.status()?;
    Ok(FactsHeld {
        file: file_name(path),
        items: status.items,
        removed_items: status.items_removed,
        events: status.events,
    })
}

/// What an intent file holds, read through `a`: every live row, the
/// readable and the unreadable alike, each by its address.
fn intent_held(a: &Annotations, path: &Path) -> anyhow::Result<IntentHeld> {
    let list = a.list(None, None)?;
    let addresses: Vec<(&str, &str, &str)> = list
        .rows
        .iter()
        .map(|r| (r.scope.as_str(), r.key.as_str(), r.kind.as_str()))
        .chain(
            list.unreadable
                .iter()
                .map(|r| (r.scope.as_str(), r.key.as_str(), r.kind.as_str())),
        )
        .collect();
    Ok(IntentHeld {
        file: file_name(path),
        prices: addresses
            .iter()
            .filter(|(_, _, kind)| *kind == BUYOUT_KIND)
            .count(),
        policy: addresses.contains(&(SYNC_POLICY_SCOPE, SYNC_POLICY_KEY, SYNC_POLICY_KIND)),
        rows: addresses.len(),
        unreadable: list.unreadable.len(),
    })
}

/// What a user file holds, read through `u`: every saved search, the
/// readable and the unreadable alike.
fn saved_held(u: &User, path: &Path) -> anyhow::Result<SavedHeld> {
    let list = u.list(Some(SAVED_SEARCH_SCOPE), None)?.list;
    Ok(SavedHeld {
        file: file_name(path),
        saved_searches: list.rows.len() + list.unreadable.len(),
        unreadable: list.unreadable.len(),
    })
}

/// Back `account` up into `dir`, made here, at `now`: its facts, its
/// intent and the world's saved searches, each the store holds, then the
/// manifest. A directory that exists is refused before anything is
/// written; any other failure removes the directory this call made.
pub fn backup(account: &Account, dir: &Path, now: i64) -> anyhow::Result<BackupMade> {
    let uuid = uuid_of(account)?;
    let store = account.frontend.dir();
    if let Some(parent) = dir.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    match std::fs::create_dir(dir) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            return Err(refused(
                "backup_exists",
                format!(
                    "{} exists: a backup makes its own directory and writes into no other, so \
                     name one that does not exist",
                    dir.display()
                ),
            ));
        }
        Err(e) => return Err(anyhow::Error::new(e).context(format!("making {}", dir.display()))),
    }
    let made = write_backup(account, &uuid, &store, dir, now);
    if made.is_err() {
        // The directory is this call's: nothing else wrote inside it.
        let _ = std::fs::remove_dir_all(dir);
    }
    let manifest = made?;
    Ok(BackupMade {
        dir: dir.display().to_string(),
        made_at_age: acquisition_store::age::ago(now, Some(manifest.made_at)),
        manifest,
    })
}

fn write_backup(
    account: &Account,
    uuid: &str,
    store: &Path,
    dir: &Path,
    now: i64,
) -> anyhow::Result<Manifest> {
    let facts_at = account_path(store, &account.entry.username);
    let facts = match facts_at.try_exists()? {
        false => None,
        true => {
            let copy = dir.join(file_name(&facts_at));
            Store::open_read_only(&facts_at)?.export(&copy)?;
            Some(facts_held(&copy)?)
        }
    };
    let intent_at = annotations_path(store, uuid);
    let intent = match intent_at.try_exists()? {
        false => None,
        true => {
            Annotations::open_read_only_for(store, uuid)?.export(&annotations_path(dir, uuid))?;
            let copy = Annotations::open_read_only_for(dir, uuid)?;
            Some(intent_held(&copy, &annotations_path(dir, uuid))?)
        }
    };
    let saved = match user_path(store).try_exists()? {
        false => None,
        true => {
            User::open_read_only_for(store)?.export(&user_path(dir))?;
            Some(saved_held(
                &User::open_read_only_for(dir)?,
                &user_path(dir),
            )?)
        }
    };
    let manifest = Manifest {
        format: FORMAT,
        account: Named {
            name: account.entry.username.clone(),
            uuid: uuid.to_string(),
        },
        made_at: now,
        build: Build {
            version: acquisition_protocol::VERSION.to_string(),
            contract: acquisition_protocol::CONTRACT_REVISION.to_string(),
        },
        facts,
        intent,
        saved,
    };
    write_manifest(dir, &manifest)?;
    Ok(manifest)
}

/// The manifest written last: to a partial, fsynced, then linked to its
/// name, so a half-written manifest never stands.
fn write_manifest(dir: &Path, manifest: &Manifest) -> anyhow::Result<()> {
    let partial = dir.join(format!("{MANIFEST}.partial"));
    let mut file = std::fs::File::create(&partial)?;
    file.write_all(serde_json::to_string_pretty(manifest)?.as_bytes())?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    drop(file);
    let linked = std::fs::hard_link(&partial, dir.join(MANIFEST))
        .or_else(|_| std::fs::rename(&partial, dir.join(MANIFEST)));
    let _ = std::fs::remove_file(&partial);
    linked?;
    #[cfg(unix)]
    std::fs::File::open(dir)?.sync_all()?;
    Ok(())
}

/// A backup's file, opened through its read-only door and checked:
/// what a restore copies from.
enum Source {
    Facts(Store),
    Intent(Annotations),
    Saved(User),
}

impl Source {
    fn export(&self, dest: &Path) -> anyhow::Result<()> {
        match self {
            Source::Facts(s) => s.export(dest),
            Source::Intent(a) => Ok(a.export(dest)?),
            Source::Saved(u) => Ok(u.export(dest)?),
        }
    }
}

/// Put back into `account`'s store each file of the backup at `dir` that
/// the store lacks, at `now`, and name each it left standing. Everything
/// is checked before the first file is written, under the world lock
/// (C193; the module doc).
pub fn restore(account: &Account, dir: &Path, now: i64) -> anyhow::Result<Restored> {
    let uuid = uuid_of(account)?;
    let world = World::observe_at(&account.frontend.root).map_err(|e| {
        refused(
            "refused",
            format!("there is no store to restore into: {e}; log in first, then restore"),
        )
    })?;
    let _held = Lock::acquire(&world.lock_path()).map_err(|e| match e.io {
        Some(_) => anyhow::anyhow!("{e}; nothing was restored"),
        None => refused(
            "refused",
            format!(
                "a daemon runs on this store ({e}), and a restore writes only where none \
                 does: stop the daemon, then restore again; nothing was restored"
            ),
        ),
    })?;
    let manifest = read_manifest(dir)?;
    if manifest.account.uuid != uuid {
        return Err(refused(
            "backup_of_another_account",
            format!(
                "{} is a backup of {}, not of {}: a restore puts back only the account's own",
                dir.display(),
                manifest.account.name,
                account.entry.username
            ),
        ));
    }
    let store = account.frontend.dir();
    // Every file the manifest names, opened and checked, before any is
    // written.
    let mut sources: Vec<(Held, PathBuf, Source)> = Vec::new();
    let damaged = |what: String| {
        refused(
            "backup_damaged",
            format!(
                "{} cannot be restored: {what}; nothing was restored",
                dir.display()
            ),
        )
    };
    if let Some(facts) = &manifest.facts {
        let path = dir.join(&facts.file);
        let opened = Store::open_read_only(&path).map_err(|e| damaged(format!("{e:#}")))?;
        if let Some(found) = opened.quick_check()? {
            return Err(damaged(format!(
                "{} failed its check: {found}",
                path.display()
            )));
        }
        if opened.account_uuids()?.iter().any(|u| *u != uuid) {
            return Err(damaged(format!(
                "{} records another account's facts",
                path.display()
            )));
        }
        if facts_held(&path)? != *facts {
            return Err(damaged(format!(
                "{} holds other counts than its manifest says",
                path.display()
            )));
        }
        sources.push((
            Held::Facts,
            account_path(&store, &account.entry.username),
            Source::Facts(opened),
        ));
    }
    if let Some(intent) = &manifest.intent {
        let path = dir.join(&intent.file);
        if path != annotations_path(dir, &uuid) {
            return Err(damaged(format!(
                "its intent file is named {}, not the account's",
                intent.file
            )));
        }
        let opened = Annotations::open_read_only_for(dir, &uuid)
            .map_err(|e| damaged(format!("{}: {e}", path.display())))?;
        if let Some(found) = opened.quick_check()? {
            return Err(damaged(format!(
                "{} failed its check: {found}",
                path.display()
            )));
        }
        if intent_held(&opened, &path)? != *intent {
            return Err(damaged(format!(
                "{} holds other counts than its manifest says",
                path.display()
            )));
        }
        sources.push((
            Held::Intent,
            annotations_path(&store, &uuid),
            Source::Intent(opened),
        ));
    }
    if let Some(saved) = &manifest.saved {
        let path = dir.join(&saved.file);
        if path != user_path(dir) {
            return Err(damaged(format!(
                "its saved searches are named {}, not the world's file",
                saved.file
            )));
        }
        let opened = User::open_read_only_for(dir)
            .map_err(|e| damaged(format!("{}: {e}", path.display())))?;
        if let Some(found) = opened.quick_check()? {
            return Err(damaged(format!(
                "{} failed its check: {found}",
                path.display()
            )));
        }
        if saved_held(&opened, &path)? != *saved {
            return Err(damaged(format!(
                "{} holds other counts than its manifest says",
                path.display()
            )));
        }
        sources.push((Held::Saved, user_path(&store), Source::Saved(opened)));
    }
    // Each file the store lacks, put back; each that stands, named.
    let (mut restored, mut left) = (Vec::new(), Vec::new());
    for (held, dest, source) in &sources {
        let stands = |found: Vec<PathBuf>| Standing {
            file: *held,
            path: dest.display().to_string(),
            move_aside: found.iter().map(|p| p.display().to_string()).collect(),
        };
        let found = standing(dest);
        if !found.is_empty() {
            left.push(stands(found));
            continue;
        }
        match source.export(dest) {
            Ok(()) => restored.push(PutBack {
                file: *held,
                path: dest.display().to_string(),
            }),
            // A file appeared meanwhile (a page wrote intent): it stands.
            Err(e)
                if e.downcast_ref::<std::io::Error>()
                    .or_else(
                        || match e.downcast_ref::<acquisition_store::AnnotationError>() {
                            Some(acquisition_store::AnnotationError::Io(io)) => Some(io),
                            _ => None,
                        },
                    )
                    .is_some_and(|io| io.kind() == std::io::ErrorKind::AlreadyExists) =>
            {
                left.push(stands(standing(dest)));
            }
            Err(e) => return Err(e.context(format!("restoring {}", dest.display()))),
        }
    }
    let absent = [
        (Held::Facts, manifest.facts.is_none()),
        (Held::Intent, manifest.intent.is_none()),
        (Held::Saved, manifest.saved.is_none()),
    ]
    .into_iter()
    .filter_map(|(held, none)| none.then_some(held))
    .collect();
    Ok(Restored {
        dir: dir.display().to_string(),
        made_at_age: acquisition_store::age::ago(now, Some(manifest.made_at)),
        restored,
        standing: left,
        absent,
        manifest,
    })
}

/// A backup's manifest, read whole, or the refusal that says what the
/// directory is instead.
fn read_manifest(dir: &Path) -> anyhow::Result<Manifest> {
    if !dir.is_dir() {
        return Err(refused(
            "not_a_backup",
            format!("there is no backup at {}", dir.display()),
        ));
    }
    let path = dir.join(MANIFEST);
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(refused(
                "not_a_backup",
                format!(
                    "{} holds no manifest, so it is no backup: a backup interrupted before its \
                     end, which is to be removed and made again, or a directory a backup never \
                     made",
                    dir.display()
                ),
            ));
        }
        Err(e) => return Err(anyhow::Error::new(e).context(format!("reading {}", path.display()))),
    };
    let manifest: Manifest = serde_json::from_str(&text).map_err(|e| {
        refused(
            "not_a_backup",
            format!("{} is not a manifest this build reads: {e}", path.display()),
        )
    })?;
    if manifest.format != FORMAT {
        return Err(refused(
            "not_a_backup",
            format!(
                "{} is a backup of format {}, and this build reads format {FORMAT}",
                dir.display(),
                manifest.format
            ),
        ));
    }
    Ok(manifest)
}
