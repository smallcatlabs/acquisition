//! The main account (C187): the account a verb, a tool or a page takes
//! where several are known and none is named, kept in the world's user
//! file (C167) under the scope `default`, a uuid and nothing else. The
//! verbs every frontend asks of it are stated once here (C46):
//!
//! - **The read.** [`Frontend::account`] takes it where it is needed;
//!   [`shown`] answers the stored row, [`MainRow`], beside the index:
//!   the uuid, the name the index holds for it or none, whether the index
//!   still holds it (`known`), its revision, provenance (C65) and its
//!   times with their words (C163). A row this build cannot read is named
//!   by its revision and why (C151).
//! - **The write** ([`set`]): an account, named as any verb names one
//!   (C121) and stored as its uuid, under an expectation ([`Expect`]):
//!   [`Expect::Create`] lands only where none stands, the agent's default
//!   (C52); [`Expect::Exactly`] over that revision; [`Expect::Current`]
//!   over whatever stands, or as a create where none does, the human's
//!   default at a terminal.
//! - **The clear** ([`clear`]): at the revision named, or the one that
//!   stands under [`Expect::Current`]; refused where none is set.
//! - **The receipt** ([`MainWritten`], spelled by
//!   [`crate::receipt::main_account`]): `target`, `written`, `prior`,
//!   `revision` and the `undo` that puts the prior back, or `no_undo`.
//!
//! Rulings: `decisions/frontends.md` (C187, C121), `decisions/store.md`
//! (C167).

use acquisition_plan::pricing::Expect;
use acquisition_store::{
    AnnotationError, AnnotationRow, DEFAULT_SCOPE, Index, IntentValue as _, MAIN_ACCOUNT_KEY,
    MainAccount, Provenance, check_value,
};
use serde::Serialize;
use serde_json::{Map, Value, json};

use crate::fault::Fault;
use crate::frontend::{Account, Frontend};
use crate::refusal::{Pointed, Way};

/// C187: the one signal that an answer or a refusal took the main
/// account, several being known and none named: `{name, uuid}`, or none
/// where the account was named or is the sole one. Every frontend's
/// answer carries it as `main_account` through [`mark`] and nothing else
/// composes it (C46).
pub fn taken(account: &Account) -> Option<Value> {
    account.main.then(|| {
        json!({
            "name": account.entry.username,
            "uuid": account.entry.uuid,
        })
    })
}

/// `document`, one JSON object (C149), with `main_account` added where
/// `taken` is the signal [`taken`] made: additive, and never over a key
/// the document already holds.
pub fn mark(document: &mut Value, taken: Option<&Value>) {
    if let (Some(taken), Value::Object(object)) = (taken, document)
        && !object.contains_key("main_account")
    {
        object.insert("main_account".into(), taken.clone());
    }
}

/// The kind's word in the user file.
const KIND: &str = <MainAccount as acquisition_store::IntentValue>::KIND;

/// The stored main account as every document carries it.
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct MainRow {
    /// The account's uuid; none where this build cannot read the row.
    pub uuid: Option<String>,
    /// The name the index holds for the uuid; none where it holds none.
    pub username: Option<String>,
    /// The index still holds the account, so a verb naming none takes it.
    pub known: bool,
    pub revision: i64,
    pub created_at: i64,
    /// C163: `created_at` in words at the answer's time.
    pub created_at_age: String,
    pub updated_at: i64,
    /// C163: `updated_at` in words at the answer's time.
    pub updated_at_age: String,
    /// The channel of the last write (C65).
    pub written_via: String,
    /// The last writer's claim, untrusted (C65).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    /// Why this build cannot read the row (C151); absent where it can.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unreadable: Option<String>,
}

impl MainRow {
    /// The row as stored, named against `index`, its times said at `now`.
    fn of(row: AnnotationRow, index: Option<&Index>, now: i64) -> MainRow {
        let (uuid, unreadable) = match check_value::<MainAccount>(&row.value) {
            Ok(main) => (Some(main.uuid), None),
            Err(e) => (None, Some(e.to_string())),
        };
        let username = uuid.as_deref().and_then(|uuid| {
            index?
                .entries()
                .iter()
                .find(|e| e.uuid.as_deref() == Some(uuid))
                .map(|e| e.username.clone())
        });
        MainRow {
            known: username.is_some(),
            uuid,
            username,
            revision: row.revision,
            created_at: row.created_at,
            created_at_age: acquisition_store::age::ago(now, Some(row.created_at)),
            updated_at: row.updated_at,
            updated_at_age: acquisition_store::age::ago(now, Some(row.updated_at)),
            written_via: row.written_via,
            actor: row.actor,
            unreadable,
        }
    }

    /// The word a write names the account by: its name where the index
    /// holds one, else its uuid.
    fn named(&self) -> Option<String> {
        self.username.clone().or_else(|| self.uuid.clone())
    }
}

/// What a write wrote (C78's receipt): the row now standing (none after
/// a clear), the row before (none on a create), and the revision the
/// next write over it names.
#[derive(Clone, Debug, Serialize)]
pub struct MainWritten {
    pub target: MainTarget,
    pub written: Option<MainRow>,
    pub prior: Option<MainRow>,
    pub revision: i64,
}

/// What a main account's write names: the one main account of the world.
#[derive(Clone, Debug, Serialize)]
pub struct MainTarget {
    pub main_account: bool,
}

impl MainWritten {
    /// What puts the prior back, with what the offer says, or why nothing
    /// does: a create's undo is its clear at the revision written; a
    /// replace's the prior account at the revision written; a clear's a
    /// create of the prior account. A prior this build cannot read has
    /// no account to put back (C151).
    pub fn undo(&self) -> Result<(Way, String), String> {
        match (&self.written, &self.prior) {
            (Some(written), None) => Ok((
                Way::MainAccountClear {
                    if_revision: written.revision,
                },
                "undoes it: no main account".to_string(),
            )),
            (Some(written), Some(prior)) => match prior.named() {
                Some(account) => Ok((
                    Way::MainAccountSet {
                        account,
                        if_revision: written.revision,
                    },
                    "puts the prior main account back".to_string(),
                )),
                None => Err("the prior cannot be put back: this build cannot read it".to_string()),
            },
            (None, Some(prior)) => match prior.named() {
                Some(account) => Ok((
                    Way::MainAccountSet {
                        account,
                        if_revision: 0,
                    },
                    "puts it back (a create, which lands only where no main account is set)"
                        .to_string(),
                )),
                None => Err(
                    "the cleared main account cannot be put back: this build cannot read it"
                        .to_string(),
                ),
            },
            (None, None) => Err("nothing was written".to_string()),
        }
    }
}

/// The stored main account, or none: read only, never creating the user
/// file (C124).
pub fn shown(frontend: &Frontend) -> Result<Option<MainRow>, Fault> {
    let dir = frontend.dir();
    if !acquisition_store::user_path(&dir).is_file() {
        return Ok(None);
    }
    let user =
        acquisition_store::User::open_read_only_for(&dir).map_err(|e| Fault::Unopened(e.into()))?;
    let row = match user.get(DEFAULT_SCOPE, MAIN_ACCOUNT_KEY, KIND) {
        Ok(row) => row,
        // a row whose JSON this build cannot read is named, never dropped
        Err(AnnotationError::Unreadable(row)) => {
            return Ok(Some(MainRow {
                uuid: None,
                username: None,
                known: false,
                revision: row.revision,
                created_at: 0,
                created_at_age: String::new(),
                updated_at: 0,
                updated_at_age: String::new(),
                written_via: String::new(),
                actor: None,
                unreadable: Some(row.fault),
            }));
        }
        Err(e) => return Err(Fault::Unopened(e.into())),
    };
    let index = Index::load(&dir).ok();
    Ok(row.map(|row| MainRow::of(row, index.as_ref(), acquisition_store::now())))
}

/// Make `account` the main account: named as any verb names one (C121),
/// stored as its uuid, under `expect` (the module doc).
pub fn set(frontend: &Frontend, account: &str, expect: Expect) -> anyhow::Result<MainWritten> {
    let picked = frontend.account(Some(account))?;
    let Some(uuid) = picked.entry.uuid.clone() else {
        return Err(Fault::NoUuid {
            username: picked.entry.username,
        }
        .into());
    };
    let mut user = frontend.user()?;
    let via = Provenance::via(frontend.via);
    let now = acquisition_store::now();
    let index = Index::load(&frontend.dir()).ok();
    let (prior, expected) = match expect {
        Expect::Create => (None, None),
        Expect::Exactly(revision) => {
            match user.at_revision(DEFAULT_SCOPE, MAIN_ACCOUNT_KEY, KIND, revision) {
                Ok(row) => (Some(row), Some(revision)),
                Err(AnnotationError::Unreadable(row)) if row.revision == revision => {
                    (None, Some(revision))
                }
                Err(e) => return Err(conflicted(e, Some(account), index.as_ref(), now)),
            }
        }
        Expect::Current => match user.get(DEFAULT_SCOPE, MAIN_ACCOUNT_KEY, KIND) {
            Ok(None) => (None, None),
            Ok(Some(row)) => {
                let revision = row.revision;
                (Some(row), Some(revision))
            }
            Err(AnnotationError::Unreadable(row)) => {
                return Err(blind(row.revision, &row.fault));
            }
            Err(e) => return Err(Fault::Other(e.into()).into()),
        },
    };
    let value = serde_json::to_value(MainAccount {
        version: MainAccount::VERSION,
        uuid,
    })?;
    let row = user
        .put::<MainAccount>(DEFAULT_SCOPE, MAIN_ACCOUNT_KEY, &value, expected, &via)
        .map_err(|e| conflicted(e, Some(account), index.as_ref(), now))?;
    let written = MainRow::of(row, index.as_ref(), now);
    Ok(MainWritten {
        target: MainTarget { main_account: true },
        revision: written.revision,
        written: Some(written),
        prior: prior.map(|row| MainRow::of(row, index.as_ref(), now)),
    })
}

/// Clear the main account at the revision named, or at the one that
/// stands under [`Expect::Current`]: afterwards a verb naming no account
/// among several is refused, as before one was set.
pub fn clear(frontend: &Frontend, expect: Expect) -> anyhow::Result<MainWritten> {
    let mut user = frontend.user()?;
    let via = Provenance::via(frontend.via);
    let now = acquisition_store::now();
    let index = Index::load(&frontend.dir()).ok();
    let prior = match expect {
        Expect::Create => {
            return Err(crate::refusal::Arguments(
                "a clear names the revision it clears, or none for the revision that stands".into(),
            )
            .into());
        }
        Expect::Exactly(revision) => {
            match user.at_revision(DEFAULT_SCOPE, MAIN_ACCOUNT_KEY, KIND, revision) {
                Ok(row) => MainRow::of(row, index.as_ref(), now),
                Err(AnnotationError::Unreadable(row)) if row.revision == revision => {
                    unread(row.revision, row.fault)
                }
                Err(e) => return Err(conflicted(e, None, index.as_ref(), now)),
            }
        }
        Expect::Current => match user.get(DEFAULT_SCOPE, MAIN_ACCOUNT_KEY, KIND) {
            Ok(None) => return Err(not_set()),
            Ok(Some(row)) => MainRow::of(row, index.as_ref(), now),
            Err(AnnotationError::Unreadable(row)) => unread(row.revision, row.fault),
            Err(e) => return Err(Fault::Other(e.into()).into()),
        },
    };
    let tombstone = user
        .delete(DEFAULT_SCOPE, MAIN_ACCOUNT_KEY, KIND, prior.revision, &via)
        .map_err(|e| conflicted(e, None, index.as_ref(), now))?;
    Ok(MainWritten {
        target: MainTarget { main_account: true },
        written: None,
        prior: Some(prior),
        revision: tombstone,
    })
}

/// A row this build cannot read, named by its revision and why (C151).
fn unread(revision: i64, fault: String) -> MainRow {
    MainRow {
        uuid: None,
        username: None,
        known: false,
        revision,
        created_at: 0,
        created_at_age: String::new(),
        updated_at: 0,
        updated_at_age: String::new(),
        written_via: String::new(),
        actor: None,
        unreadable: Some(fault),
    }
}

/// A clear where no main account is set.
fn not_set() -> anyhow::Error {
    Pointed {
        kind: "refused",
        message: "no main account is set: nothing was cleared".to_string(),
        ways: Vec::new(),
        fields: Map::new(),
    }
    .into()
}

/// A write naming no revision over a row this build cannot read: never
/// replaced blind (C35, C151).
fn blind(revision: i64, fault: &str) -> anyhow::Error {
    Pointed {
        kind: "refused",
        message: format!(
            "the main account holds a value this build cannot read (revision {revision}: \
             {fault}); a write naming that revision replaces it, and a clear naming it clears it"
        ),
        ways: Vec::new(),
        fields: Map::new(),
    }
    .into()
}

/// A write's store refusal: `conflict` with `revision` (null where none
/// is set) and `current`, the row that stands, offered again at that
/// revision — the same set, or the same clear (C11).
fn conflicted(
    e: AnnotationError,
    account: Option<&str>,
    index: Option<&Index>,
    now: i64,
) -> anyhow::Error {
    let (revision, current) = match e {
        AnnotationError::Conflict { current: Some(row) } => {
            (Some(row.revision), Some(MainRow::of(*row, index, now)))
        }
        AnnotationError::Conflict { current: None } => (None, None),
        AnnotationError::Unreadable(row) => (Some(row.revision), None),
        other => return Fault::Other(other.into()).into(),
    };
    let mut fields = Map::new();
    fields.insert("revision".into(), json!(revision));
    if let Some(current) = &current {
        fields.insert(
            "current".into(),
            serde_json::to_value(current).unwrap_or_default(),
        );
    }
    let (message, ways) = match (revision, account) {
        (Some(revision), Some(account)) => (
            format!(
                "the main account stands at revision {revision}, not the revision the write \
                 named: nothing was written"
            ),
            vec![(
                Way::MainAccountSet {
                    account: account.to_string(),
                    if_revision: revision,
                },
                format!("the same write over revision {revision}, the main account as it stands"),
            )],
        ),
        (Some(revision), None) => (
            format!(
                "the main account stands at revision {revision}, not the revision the clear \
                 named: nothing was cleared"
            ),
            vec![(
                Way::MainAccountClear {
                    if_revision: revision,
                },
                format!("the same clear at revision {revision}, the main account as it stands"),
            )],
        ),
        (None, Some(account)) => (
            "no main account is set at the revision the write named: nothing was written"
                .to_string(),
            vec![(
                Way::MainAccountSet {
                    account: account.to_string(),
                    if_revision: 0,
                },
                "the same write as a create, which lands only where none is set".to_string(),
            )],
        ),
        (None, None) => (
            "no main account is set: nothing was cleared".to_string(),
            Vec::new(),
        ),
    };
    Pointed {
        kind: "conflict",
        message,
        ways,
        fields,
    }
    .into()
}
