//! The account index: `store/<provider>/accounts.json`, a small non-secret
//! list of the accounts this machine has logged into (GGG username,
//! `name#discriminator`), maintained by the daemon at login/logout and
//! read by frontends to resolve `--account`/`ACQ_ACCOUNT` without a
//! daemon. It is also how the daemon knows which keyring entries to load
//! at start (the keyring crate cannot enumerate). Secrets never live here.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountEntry {
    /// GGG username as the token response reports it: `name#discriminator`.
    pub username: String,
    /// Unix seconds of the most recent login.
    pub last_login: i64,
    /// A refresh token for this account is in the keyring. False for
    /// one-off sessions (`ACQ_NO_KEYRING`, a failed keyring save) and after
    /// logout; the account stays listed because its store file remains.
    pub persisted: bool,
    /// From `GET /profile`; stable across name changes. Required at login
    /// since 2026-08-31 (`record_login`) and the key that names the
    /// account's annotation file. Entries from before then may lack it:
    /// one re-auth fixes that, no migration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// C144 (no keyring entry out of the index's reach): an old name the
    /// account's keyring entry is still held under — a rename whose clear
    /// of the old name failed. Logout and forget clear it beside the
    /// username; it is released only once a clear under it succeeds.
    /// Absent in the usual case and in files written before it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyring_name: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct File {
    accounts: Vec<AccountEntry>,
}

#[derive(Debug)]
pub struct Index {
    path: PathBuf,
    entries: Vec<AccountEntry>,
}

/// How a selector failed to pick exactly one account.
#[derive(Debug, PartialEq, Eq)]
pub enum Resolve {
    /// No accounts are known at all.
    Empty,
    /// The selector matched nothing.
    NotFound(String),
    /// No selector, and more than one account is known.
    Ambiguous(Vec<String>),
}

impl std::fmt::Display for Resolve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resolve::Empty => write!(f, "no accounts known (run `acq auth`)"),
            Resolve::NotFound(s) => write!(f, "no account matches {s:?} (see `acq accounts`)"),
            Resolve::Ambiguous(names) => write!(
                f,
                "several accounts are known; pick one with --account or ACQ_ACCOUNT: {}",
                names.join(", ")
            ),
        }
    }
}

impl std::error::Error for Resolve {}

/// `<dir>/<username>.db`, with the username made filename-safe
/// (`GERWARIC#7694` → `GERWARIC_7694.db`).
pub fn account_path(dir: &Path, username: &str) -> PathBuf {
    dir.join(format!("{}.db", filename_safe(username)))
}

pub(crate) fn filename_safe(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

pub fn index_path(dir: &Path) -> PathBuf {
    dir.join("accounts.json")
}

/// Whether a selector names this account: the exact username, the name
/// without its `#discriminator` (both case-insensitive), or the uuid.
/// Never a prefix.
pub fn account_matches(selector: &str, username: &str, uuid: Option<&str>) -> bool {
    let sel = selector.to_lowercase();
    let u = username.to_lowercase();
    u == sel || u.split_once('#').is_some_and(|(name, _)| name == sel) || uuid == Some(selector)
}

impl Index {
    /// A missing file is an empty index.
    pub fn load(dir: &Path) -> Result<Index> {
        let path = index_path(dir);
        let entries = match std::fs::read_to_string(&path) {
            Ok(text) => {
                serde_json::from_str::<File>(&text)
                    .with_context(|| format!("parsing {}", path.display()))?
                    .accounts
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(e) => return Err(e).with_context(|| format!("reading {}", path.display())),
        };
        Ok(Index { path, entries })
    }

    pub fn entries(&self) -> &[AccountEntry] {
        &self.entries
    }

    pub fn get(&self, username: &str) -> Option<&AccountEntry> {
        self.entries.iter().find(|e| e.username == username)
    }

    /// Record a completed login. The uuid is required at login (C50): a
    /// login only reaches the index once the profile fetch delivered it.
    /// If the uuid is already listed under a different username, the
    /// account was renamed — the entry follows the new name (a mapping
    /// update; the uuid-named annotation file is untouched, the old
    /// username-named fact file is orphaned and refetchable).
    pub fn record_login(
        &mut self,
        username: &str,
        uuid: &str,
        persisted: bool,
        at: i64,
    ) -> Result<()> {
        // A rename would otherwise leave two entries for one uuid, which no
        // selector could tell apart.
        if let Some(e) = self
            .entries
            .iter_mut()
            .find(|e| e.uuid.as_deref() == Some(uuid))
        {
            e.username = username.to_string();
            e.last_login = at;
            e.persisted = persisted;
            // A uuid-less entry already wearing this name is a pre-uuid
            // leftover for the same account: drop it rather than leave an
            // ambiguous twin. Entries with another uuid keep their row —
            // they point at a different account's annotations.
            self.entries
                .retain(|e| e.username != username || e.uuid.is_some());
            return self.save();
        }
        match self.entries.iter_mut().find(|e| e.username == username) {
            Some(e) => {
                e.last_login = at;
                e.persisted = persisted;
                e.uuid = Some(uuid.to_string());
            }
            None => self.entries.push(AccountEntry {
                username: username.to_string(),
                last_login: at,
                persisted,
                uuid: Some(uuid.to_string()),
                keyring_name: None,
            }),
        }
        self.save()
    }

    /// Refresh-path bookkeeping (last login / keyring state) for an entry a
    /// completed login already created. Never invents a uuid.
    pub fn upsert(&mut self, username: &str, persisted: bool, at: i64) -> Result<()> {
        match self.entries.iter_mut().find(|e| e.username == username) {
            Some(e) => {
                e.last_login = at;
                e.persisted = persisted;
            }
            None => self.entries.push(AccountEntry {
                username: username.to_string(),
                last_login: at,
                persisted,
                uuid: None,
                keyring_name: None,
            }),
        }
        self.save()
    }

    /// Logout or a dead grant: the keyring no longer holds this account.
    pub fn set_persisted(&mut self, username: &str, persisted: bool) -> Result<()> {
        if let Some(e) = self.entries.iter_mut().find(|e| e.username == username) {
            e.persisted = persisted;
            self.save()?;
        }
        Ok(())
    }

    /// C144: hold `name` as the old name the account's keyring entry is
    /// still under, or release it with `None`. The username itself is no
    /// old name. Writes only on a change.
    pub fn hold_keyring_name(&mut self, username: &str, name: Option<&str>) -> Result<()> {
        if let Some(e) = self.entries.iter_mut().find(|e| e.username == username) {
            let name = name.filter(|n| *n != username).map(str::to_string);
            if e.keyring_name != name {
                e.keyring_name = name;
                self.save()?;
            }
        }
        Ok(())
    }

    pub fn set_uuid(&mut self, username: &str, uuid: &str) -> Result<()> {
        if let Some(e) = self.entries.iter_mut().find(|e| e.username == username) {
            e.uuid = Some(uuid.to_string());
            self.save()?;
        }
        Ok(())
    }

    /// Accounts with a keyring entry, most recent login first: the order
    /// a daemon restores them in.
    pub fn persisted(&self) -> Vec<&AccountEntry> {
        let mut v: Vec<&AccountEntry> = self.entries.iter().filter(|e| e.persisted).collect();
        v.sort_by_key(|e| std::cmp::Reverse(e.last_login));
        v
    }

    /// Pick one account. A selector matches the exact username, the
    /// username without its `#discriminator`, or the exact uuid — never a
    /// prefix. No selector picks the sole account, or fails as ambiguous.
    pub fn resolve(&self, selector: Option<&str>) -> Result<&AccountEntry, Resolve> {
        if self.entries.is_empty() {
            return Err(Resolve::Empty);
        }
        match selector {
            None => match self.entries.as_slice() {
                [only] => Ok(only),
                _ => Err(Resolve::Ambiguous(
                    self.entries.iter().map(|e| e.username.clone()).collect(),
                )),
            },
            Some(sel) => {
                let hits: Vec<&AccountEntry> = self
                    .entries
                    .iter()
                    .filter(|e| account_matches(sel, &e.username, e.uuid.as_deref()))
                    .collect();
                match hits.as_slice() {
                    [one] => Ok(one),
                    [] => Err(Resolve::NotFound(sel.to_string())),
                    many => Err(Resolve::Ambiguous(
                        many.iter().map(|e| e.username.clone()).collect(),
                    )),
                }
            }
        }
    }

    /// C144 (the account index is written atomically, synced): the whole
    /// file to a temporary name no other writer shares, synced, then
    /// renamed over the index and the directory synced, so a reader finds
    /// the old index or the new one, never a torn one, whatever stops the
    /// writer between. The daemon serialises its writers under its one
    /// lock; this is what keeps a crash, or a second process, from
    /// tearing the file.
    fn save(&self) -> Result<()> {
        let tmp = self.write_temp()?;
        let renamed = std::fs::rename(&tmp, &self.path)
            .with_context(|| format!("replacing {}", self.path.display()));
        if renamed.is_err() {
            let _ = std::fs::remove_file(&tmp);
        }
        renamed?;
        sync_dir(self.path.parent())
    }

    /// The first half of [`Self::save`]: the whole file written and synced
    /// under a name of its own beside the index. Returns that name.
    fn write_temp(&self) -> Result<PathBuf> {
        use std::io::Write;
        static SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let dir = self.path.parent().unwrap_or_else(|| Path::new("."));
        std::fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
        let file = File {
            accounts: self.entries.clone(),
        };
        let tmp = dir.join(format!(
            ".accounts.json.{}.{}.tmp",
            std::process::id(),
            SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        let written = (|| -> Result<()> {
            let mut f = std::fs::File::create(&tmp)?;
            f.write_all(serde_json::to_string_pretty(&file)?.as_bytes())?;
            f.sync_all()?;
            Ok(())
        })()
        .with_context(|| format!("writing {}", tmp.display()));
        if written.is_err() {
            let _ = std::fs::remove_file(&tmp);
        }
        written.map(|()| tmp)
    }
}

/// Sync a directory, so a rename in it is durable. Windows cannot open a
/// directory as a file through `std`, so there the rename goes unsynced:
/// a crash may undo it, leaving the previous index whole, never a torn
/// one.
fn sync_dir(dir: Option<&Path>) -> Result<()> {
    #[cfg(unix)]
    if let Some(dir) = dir {
        std::fs::File::open(dir)
            .and_then(|d| d.sync_all())
            .with_context(|| format!("syncing {}", dir.display()))?;
    }
    #[cfg(not(unix))]
    let _ = dir;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp() -> PathBuf {
        // A counter, not a clock: two parallel tests can share a
        // nanosecond, and the loser's directory gets deleted under it by
        // the winner's cleanup (seen intermittently 2026-09-01).
        static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!(
            "acq-index-{}-{}",
            std::process::id(),
            N.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn round_trip_and_resolution() {
        let dir = tmp();
        let mut idx = Index::load(&dir).unwrap();
        assert_eq!(idx.resolve(None), Err(Resolve::Empty));
        idx.upsert("Alice#1234", true, 10).unwrap();
        assert_eq!(idx.resolve(None).unwrap().username, "Alice#1234");
        idx.upsert("Bob#0001", false, 20).unwrap();
        assert!(matches!(idx.resolve(None), Err(Resolve::Ambiguous(ref v)) if v.len() == 2));
        assert_eq!(idx.resolve(Some("alice")).unwrap().username, "Alice#1234");
        assert_eq!(
            idx.resolve(Some("Alice#1234")).unwrap().username,
            "Alice#1234"
        );
        assert_eq!(
            idx.resolve(Some("Ali")),
            Err(Resolve::NotFound("Ali".into()))
        );
        idx.set_uuid("Bob#0001", "u-bob").unwrap();
        assert_eq!(idx.resolve(Some("u-bob")).unwrap().username, "Bob#0001");
        // Reload from disk: same content; persisted order is by recency.
        let idx = Index::load(&dir).unwrap();
        assert_eq!(idx.entries().len(), 2);
        assert_eq!(
            idx.persisted()
                .iter()
                .map(|e| e.username.as_str())
                .collect::<Vec<_>>(),
            vec!["Alice#1234"]
        );
        let mut idx = idx;
        idx.upsert("Bob#0001", true, 30).unwrap();
        assert_eq!(idx.persisted()[0].username, "Bob#0001");
        idx.set_persisted("Bob#0001", false).unwrap();
        assert_eq!(idx.persisted().len(), 1);
        assert_eq!(
            account_path(&dir, "Alice#1234").file_name().unwrap(),
            "Alice_1234.db"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn record_login_requires_the_uuid_and_follows_renames() {
        let dir = tmp();
        let mut idx = Index::load(&dir).unwrap();
        idx.record_login("Alice#1234", "u-alice", true, 10).unwrap();
        let e = idx.get("Alice#1234").unwrap();
        assert_eq!((e.uuid.as_deref(), e.persisted), (Some("u-alice"), true));
        assert_eq!(idx.resolve(Some("u-alice")).unwrap().username, "Alice#1234");
        // A later login with the same uuid under a new name is a rename:
        // the mapping updates, no second entry appears.
        idx.record_login("Alicia#9999", "u-alice", true, 20)
            .unwrap();
        assert_eq!(idx.entries().len(), 1);
        assert_eq!(
            idx.resolve(Some("u-alice")).unwrap().username,
            "Alicia#9999"
        );
        assert!(idx.get("Alice#1234").is_none());
        // A pre-uuid leftover entry gains its uuid in place.
        idx.upsert("Bob#0001", true, 30).unwrap();
        assert!(idx.get("Bob#0001").unwrap().uuid.is_none());
        idx.record_login("Bob#0001", "u-bob", true, 40).unwrap();
        assert_eq!(idx.entries().len(), 2);
        assert_eq!(idx.get("Bob#0001").unwrap().uuid.as_deref(), Some("u-bob"));
        // A rename onto a name a uuid-less leftover holds drops the twin.
        idx.upsert("Cleo#0002", false, 50).unwrap();
        idx.record_login("Cleo#0002", "u-alice", true, 60).unwrap();
        let cleos: Vec<_> = idx
            .entries()
            .iter()
            .filter(|e| e.username == "Cleo#0002")
            .collect();
        assert_eq!(cleos.len(), 1);
        assert_eq!(cleos[0].uuid.as_deref(), Some("u-alice"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn corrupt_index_is_an_error_not_a_reset() {
        let dir = tmp();
        std::fs::write(index_path(&dir), "{nope").unwrap();
        assert!(Index::load(&dir).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// C144 (the index is written atomically), issue #63: a writer that
    /// stops between writing its temporary file and the rename — a crash
    /// staged by writing the temporary and never renaming it — leaves the
    /// index as it was, readable whole, and the next save replaces it.
    #[test]
    fn c144_a_crash_between_write_and_rename_leaves_the_index_whole() {
        let dir = tmp();
        let mut idx = Index::load(&dir).unwrap();
        idx.upsert("Alice#1234", true, 10).unwrap();
        let before = std::fs::read_to_string(index_path(&dir)).unwrap();

        idx.entries.push(AccountEntry {
            username: "Bob#0001".into(),
            last_login: 20,
            persisted: true,
            uuid: None,
            keyring_name: None,
        });
        let orphan = idx.write_temp().unwrap();
        assert!(orphan.exists());
        assert_eq!(std::fs::read_to_string(index_path(&dir)).unwrap(), before);
        let read = Index::load(&dir).unwrap();
        assert_eq!(read.entries().len(), 1, "the old index, whole");

        idx.save().unwrap();
        assert_eq!(Index::load(&dir).unwrap().entries().len(), 2);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// C144, issue #63: two writers for two accounts saving at once — two
    /// processes, or a daemon writer beside a stray one — never tear the
    /// file: each save has a temporary of its own, so a reader always
    /// parses an index one of them wrote whole. (That neither loses the
    /// other's change is the daemon's lock, its every index write under
    /// `shared`.)
    #[test]
    fn c144_two_accounts_saving_at_once_never_tear_the_index() {
        let dir = tmp();
        Index::load(&dir).unwrap().upsert("seed", false, 0).unwrap();
        let writers: Vec<_> = ["Alice#1234", "Bob#0001"]
            .into_iter()
            .map(|name| {
                let dir = dir.clone();
                std::thread::spawn(move || {
                    for at in 0..50 {
                        let mut idx = Index::load(&dir).unwrap();
                        idx.upsert(name, at % 2 == 0, at).unwrap();
                    }
                })
            })
            .collect();
        let reader = {
            let dir = dir.clone();
            std::thread::spawn(move || {
                for _ in 0..100 {
                    Index::load(&dir).expect("never a torn index");
                }
            })
        };
        for w in writers {
            w.join().unwrap();
        }
        reader.join().unwrap();
        assert!(Index::load(&dir).is_ok());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
