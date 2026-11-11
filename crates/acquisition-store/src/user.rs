//! User-scoped intent (C167): one file of its own, `user.db`, beside the
//! account files in the world's provider directory, written only through
//! this crate under revision compare-and-swap as annotations are (C35),
//! with a revision of its own that no answer's basis carries. It is the
//! world's, not an account's: nothing in it names a GGG id, so no account
//! uuid is bound to it.
//!
//! [`User`] is a newtype over [`Annotations`] opened at `<dir>/user.db`:
//! the file's mechanics (the schema, the creation race, the busy wait,
//! synchronous mode, the export) are the annotation file's, by the one
//! [`Annotations::open`], and its API is the annotation API delegated, so
//! every property the one table in `unit/tests/annotations.rs` pins holds
//! for both files. A newtype, so a signature taking `&Annotations` cannot
//! be handed the user file, nor one taking `&User` an account's.
//!
//! Its first kind is [`SavedSearch`], under the scope [`SAVED_SEARCH_SCOPE`]
//! and keyed on an id the verbs mint (the design of 4-full part 1, §3): a
//! name, the canonical text of a query and the scope it runs over. The
//! store reads the shape alone; the language is the search crate's, which
//! this crate links none of (C1).
//!
//! Rulings: `decisions/store.md` (C167, C35, C151); C66's kind trait is
//! [`IntentValue`].

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::annotations::{
    AnnotationError, AnnotationList, AnnotationRow, Annotations, IntentValue, Provenance,
};
use crate::index::HoldsAccountFacts;

/// The file's name inside the world's provider directory.
pub const USER_FILE: &str = "user.db";

/// `<dir>/user.db`: beside `daemon.db`, the rails and the account files.
pub fn user_path(dir: &Path) -> PathBuf {
    dir.join(USER_FILE)
}

/// The scope word every saved search is written under; its key is the id.
pub const SAVED_SEARCH_SCOPE: &str = "search";

/// The realm words a saved search's scope may name: each realm, or every
/// one, as a search's scope may (the search crate's `Realm::parse`).
pub const SAVED_REALMS: [&str; 5] = ["pc", "xbox", "sony", "poe2", "all"];

/// The membership words a saved search's scope may name, as a search's.
pub const SAVED_MEMBERSHIPS: [&str; 3] = ["live", "all", "removed"];

/// The most characters a saved search's name holds, after trimming.
pub const NAME_MAX: usize = 64;

/// The account name whose facts a build before the account path's escape
/// kept at `user.db`.
const USER_STEM: &str = "user";

/// A `user.db` at `path` holding an account's facts is refused; one that
/// does not exist, or that read-only SQLite cannot open, is left to the
/// opener, which creates the one and reports the other in its own words.
fn refuse_account_facts(path: &Path) -> Result<(), AnnotationError> {
    if !path.is_file() {
        return Ok(());
    }
    let Ok(conn) = rusqlite::Connection::open_with_flags(
        path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
    ) else {
        return Ok(());
    };
    match HoldsAccountFacts::probe(&conn, path, USER_STEM)? {
        Some(found) => Err(AnnotationError::HoldsAccountFacts(found)),
        None => Ok(()),
    }
}

/// The user file: [`Annotations`] at `<dir>/user.db`, no uuid bound.
pub struct User(Annotations);

/// What [`User::list`] answers: the rows, readable and named (C151), and
/// the file's revision, read in one transaction.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UserList {
    pub list: AnnotationList,
    pub revision: i64,
}

impl User {
    /// Open (or create) the world's user file under `dir`. No uuid is bound
    /// or stamped: the file is the world's.
    ///
    /// A `user.db` that holds an account's facts, from a build before the
    /// account path's escape, is refused with the deletion that frees it
    /// ([`AnnotationError::HoldsAccountFacts`]), read before anything is
    /// written.
    pub fn open_for(dir: &Path) -> Result<User, AnnotationError> {
        let path = user_path(dir);
        refuse_account_facts(&path)?;
        Annotations::open(&path).map(User)
    }

    /// C124 (a read never migrates): the user file's read-only door, over
    /// the unbound opener ([`Annotations::open_read_only_at`]): never
    /// created, nothing written, a file at another version refused.
    /// An account's facts at `user.db` are refused as [`User::open_for`]
    /// refuses them.
    pub fn open_read_only_for(dir: &Path) -> Result<User, AnnotationError> {
        let path = user_path(dir);
        refuse_account_facts(&path)?;
        Annotations::open_read_only_at(&path).map(User)
    }

    /// An in-memory user file, for tests and ephemeral use.
    pub fn open_memory() -> Result<User, AnnotationError> {
        Annotations::open_memory().map(User)
    }

    pub fn path(&self) -> &Path {
        self.0.path()
    }

    /// [`Annotations::put`]: typed at the door (C66), stamped (C65), under
    /// compare-and-swap (C35).
    pub fn put<K: IntentValue>(
        &mut self,
        scope: &str,
        key: &str,
        value: &Value,
        expected_revision: Option<i64>,
        provenance: &Provenance,
    ) -> Result<AnnotationRow, AnnotationError> {
        self.0
            .put::<K>(scope, key, value, expected_revision, provenance)
    }

    /// [`Annotations::get`].
    pub fn get(
        &self,
        scope: &str,
        key: &str,
        kind: &str,
    ) -> Result<Option<AnnotationRow>, AnnotationError> {
        self.0.get(scope, key, kind)
    }

    /// [`Annotations::at_revision`]: the live row a write naming
    /// `revision` replaces, or the conflict that write would raise.
    pub fn at_revision(
        &self,
        scope: &str,
        key: &str,
        kind: &str,
        revision: i64,
    ) -> Result<AnnotationRow, AnnotationError> {
        self.0.at_revision(scope, key, kind, revision)
    }

    /// [`Annotations::list`] and the file's revision, from one
    /// transaction: a list and a revision a write fell between would name
    /// a state that never stood.
    pub fn list(
        &self,
        scope: Option<&str>,
        kind: Option<&str>,
    ) -> Result<UserList, AnnotationError> {
        let (list, revision) = self.0.list_at_revision(scope, kind)?;
        Ok(UserList { list, revision })
    }

    /// [`Annotations::delete`]: a tombstone whose revision keeps counting.
    pub fn delete(
        &mut self,
        scope: &str,
        key: &str,
        kind: &str,
        expected_revision: i64,
        provenance: &Provenance,
    ) -> Result<i64, AnnotationError> {
        self.0
            .delete(scope, key, kind, expected_revision, provenance)
    }

    /// [`Annotations::revision`]: the sum of every row's revision.
    pub fn revision(&self) -> Result<i64, AnnotationError> {
        self.0.revision()
    }

    /// [`Annotations::export`]: a store-managed backup.
    pub fn export(&self, dest: &Path) -> Result<(), AnnotationError> {
        self.0.export(dest)
    }

    /// The annotation handle underneath, for this crate's own tests.
    #[cfg(test)]
    pub(crate) fn inner(&self) -> &Annotations {
        &self.0
    }

    /// [`User::inner`], mutably: a test stages a hook on its connection.
    #[cfg(test)]
    pub(crate) fn inner_mut(&mut self) -> &mut Annotations {
        &mut self.0
    }
}

/// A saved search (C167's first kind): a name, the canonical text of a
/// query (C104) and the scope it runs over. The name is a label and the
/// id, the row's key, its identity: nothing here or at the door refuses a
/// name another row carries (the owner, 2026-11-07, "Drop and cut").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedSearch {
    pub version: i64,
    pub name: String,
    pub text: String,
    pub scope: SavedScope,
}

/// The scope a saved search runs over: a realm word or `all` (C96), and a
/// membership. Never an account: a search runs over whichever is chosen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedScope {
    pub realm: String,
    pub membership: String,
}

impl SavedSearch {
    /// Why `name` is no saved search's name, or nothing: one to
    /// [`NAME_MAX`] characters once trimmed, with no line break.
    pub fn name_fault(name: &str) -> Option<String> {
        let trimmed = name.trim();
        let count = trimmed.chars().count();
        if count == 0 {
            return Some("a name is one character at least once trimmed".into());
        }
        if count > NAME_MAX {
            return Some(format!(
                "a name is at most {NAME_MAX} characters once trimmed; this one is {count}"
            ));
        }
        if trimmed.contains(['\n', '\r']) {
            return Some("a name holds no line break".into());
        }
        None
    }
}

impl IntentValue for SavedSearch {
    const KIND: &'static str = "saved-search";
    const VERSION: i64 = 1;

    fn parse(value: &Value) -> Result<Self, String> {
        let saved: SavedSearch =
            serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
        if saved.version != Self::VERSION {
            return Err(format!(
                "version {} is no version of this kind",
                saved.version
            ));
        }
        if let Some(fault) = SavedSearch::name_fault(&saved.name) {
            return Err(fault);
        }
        if !SAVED_REALMS.contains(&saved.scope.realm.as_str()) {
            return Err(format!(
                "`{}` is no realm: {}",
                saved.scope.realm,
                SAVED_REALMS.join(", ")
            ));
        }
        if !SAVED_MEMBERSHIPS.contains(&saved.scope.membership.as_str()) {
            return Err(format!(
                "`{}` is no membership: {}",
                saved.scope.membership,
                SAVED_MEMBERSHIPS.join(", ")
            ));
        }
        Ok(saved)
    }
}
