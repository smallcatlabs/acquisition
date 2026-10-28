//! The credentials lock as a type. C144 (the account index is written
//! under the one lock) and C148 (the keyring and index I/O under one
//! credentials lock, never under `shared`) used to be a sentence on
//! `index_apply`, "called under `credentials`", held by whoever read it.
//! Now the hold is a value: [`CredentialsHeld`] is made by
//! [`Daemon::hold_credentials`] alone (its field is private to this
//! module, so no sibling can build one), and every index write and every
//! persist of a keyring entry takes one by reference. A writer outside the
//! lock has nothing to pass and does not compile; the pin is the type, not
//! a test (issue #63, `roadmap/briefs/63-store-remainder.md`).

use std::sync::MutexGuard;

use super::Daemon;

/// Proof that the caller holds the daemon's credentials lock, for as long
/// as the value lives. Dropping it releases the lock.
pub(super) struct CredentialsHeld<'a> {
    _held: MutexGuard<'a, ()>,
}

impl Daemon {
    /// Take the credentials lock (C148: on a blocking thread, never under
    /// `shared`). The one way to a [`CredentialsHeld`].
    pub(super) fn hold_credentials(&self) -> CredentialsHeld<'_> {
        CredentialsHeld {
            _held: self.credentials.lock().unwrap(),
        }
    }
}
