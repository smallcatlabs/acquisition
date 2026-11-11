//! The saved search (C167, user-scoped intent in a file of its own; the
//! design of 4-full part 1, §3): the verbs every frontend asks of the
//! world's `user.db`, stated once here so the CLI now and the MCP and the
//! pages after it inherit one semantics (C46).
//!
//! - **The row.** Keyed on an id these verbs mint, twelve hex characters
//!   ([`mint`]); the value a name, the canonical text of a query (C104)
//!   and its scope, a realm or `all` (C96) and a membership. The id is
//!   the identity and the name a label: no write reads another row's
//!   name, nothing refuses a name, two live rows may carry one (the
//!   owner, 2026-11-07, "Drop and cut"). The one place a shared name is
//!   noticed is opening by name, [`request_of`] and [`show`], refused as
//!   `ambiguous_saved_search`.
//! - **The documents.** [`list`] answers [`SavedSearches`], one object
//!   (C149), its rows ten unless `limit` or `expand` (C53) beside `total`,
//!   `left_out` and the file's `revision`; each row ([`SavedRow`]) its
//!   `id`, `name`, `text`, `scope`, `revision`, provenance (C65), its
//!   times with their words (C163), and `request`, the search request it
//!   is; a row this build cannot read is named by its id, revision and
//!   fault, never dropped (C151).
//! - **One write shape on every surface** ([`Set`], [`set`]): an optional
//!   id, an expectation ([`Expect`]) and the fields, each optional. No id
//!   and [`Expect::Create`] creates, a name and a text required; an id
//!   and [`Expect::Exactly`] reads the row at exactly that revision
//!   (`at_revision`, as `set_policy` reads its row) and merges the fields
//!   named over it, the write landing at that revision; an id and
//!   [`Expect::Current`] the same at the revision read, the human's
//!   default at a terminal, which guards against a write racing in
//!   between and nothing more; an id and [`Expect::Create`] the whole
//!   value, landing only where no live row stands at the id (a clear's
//!   undo). Every other pair is refused as `arguments`. The text is read
//!   with the language's reader in the row's realm and stored canonical;
//!   an authoring error refuses before any file is opened. A create's
//!   realm, where none is named, is resolved as `acq search` resolves
//!   one (C96), over the account the caller names.
//! - **The receipt** ([`SavedWritten`], spelled by
//!   [`crate::receipt::saved`]): `target`, `written`, `prior`, `revision`,
//!   and the `undo` that puts the prior back — a replace's a [`Set`] of
//!   the prior whole at the revision written, a create's its clear, a
//!   clear's a create of the prior whole at the id — or `no_undo` where
//!   the prior is a row this build could not read (C151).
//! - **The refusals**: `saved_search_not_found` (offering the list) and
//!   `ambiguous_saved_search` (the rows of the name, ten beside `total`
//!   and `left_out`, each offered by id); a stale revision is `conflict`
//!   with `revision` and `current` (C11), offered again at the revision
//!   that stands.

use acquisition_plan::pricing::Expect;
use acquisition_search::SearchError;
use acquisition_search::answer::{Membership, QueryInput, Request, Scope};
use acquisition_search::corpus;
use acquisition_store::{
    AnnotationError, AnnotationRow, IntentValue as _, Provenance, SAVED_SEARCH_SCOPE, SavedScope,
    SavedSearch, User, check_value,
};
use serde::Serialize;
use serde_json::{Map, Value, json};

use crate::fault::Fault;
use crate::frontend::Frontend;
use crate::refusal::{Arguments, Pointed, Way};

/// The characters of an id: twelve lowercase hex digits.
pub const ID_LEN: usize = 12;

/// The kind's word in the user file.
const KIND: &str = <SavedSearch as acquisition_store::IntentValue>::KIND;

/// A saved search written: the one shape a write takes on every surface
/// (the module doc). `id` none is a create; the fields not named keep the
/// row's values as read at the revision the write names.
#[derive(Debug, Clone, PartialEq)]
pub struct Set {
    pub id: Option<String>,
    pub expect: Expect,
    pub name: Option<String>,
    pub text: Option<String>,
    pub realm: Option<String>,
    pub membership: Option<String>,
}

impl Set {
    /// The whole value `saved` written at `id` under `expect`: what an
    /// undo and a re-aimed write send.
    fn whole(id: &str, saved: &SavedSearch, expect: Expect) -> Set {
        Set {
            id: Some(id.to_string()),
            expect,
            name: Some(saved.name.clone()),
            text: Some(saved.text.clone()),
            realm: Some(saved.scope.realm.clone()),
            membership: Some(saved.scope.membership.clone()),
        }
    }
}

/// One saved search as every document carries it: readable, or named by
/// its id and revision with the reason this build cannot read it (C151).
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum SavedRow {
    Read(Box<ReadRow>),
    Unreadable(UnreadRow),
}

impl SavedRow {
    pub fn id(&self) -> &str {
        match self {
            SavedRow::Read(row) => &row.id,
            SavedRow::Unreadable(row) => &row.id,
        }
    }

    pub fn revision(&self) -> i64 {
        match self {
            SavedRow::Read(row) => row.revision,
            SavedRow::Unreadable(row) => row.revision,
        }
    }

    /// The name, where this build can read it.
    pub fn name(&self) -> Option<&str> {
        match self {
            SavedRow::Read(row) => Some(&row.name),
            SavedRow::Unreadable(_) => None,
        }
    }
}

/// A saved search this build reads.
#[derive(Clone, Debug, Serialize)]
pub struct ReadRow {
    pub id: String,
    pub name: String,
    pub text: String,
    pub scope: SavedScope,
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
    /// The search request the row is: its scope and its text, no account.
    pub request: Request,
}

/// A saved search this build cannot read (C151): its id, the revision a
/// clear or a write must name, and why.
#[derive(Clone, Debug, Serialize)]
pub struct UnreadRow {
    pub id: String,
    pub revision: i64,
    pub unreadable: String,
}

/// The saved searches listed (C149, C53): ten unless `limit` names more
/// or `expand` asks for all, readable ones by name then id, the
/// unreadable after them; `total` every row, `left_out` those not listed,
/// and the file's `revision`, read in the rows' one transaction.
#[derive(Clone, Debug, Serialize)]
pub struct SavedSearches {
    pub searches: Vec<SavedRow>,
    pub total: usize,
    pub left_out: usize,
    /// What lists them all: the request's `expand`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_out_needs: Option<&'static str>,
    /// The user file's revision (C167): the sum every write raises.
    pub revision: i64,
}

/// What a write wrote (C78's receipt, [`crate::receipt::saved`]): the
/// row's id, the row now standing (none after a clear), the row before
/// (none on a create), and the revision the next write over it names.
#[derive(Clone, Debug, Serialize)]
pub struct SavedWritten {
    pub target: SavedTarget,
    pub written: Option<SavedRow>,
    pub prior: Option<SavedRow>,
    pub revision: i64,
    /// The prior's value, where this build reads it: what the undo puts
    /// back.
    #[serde(skip)]
    pub prior_value: Option<SavedSearch>,
    /// C187: where a create's realm was resolved over the main account,
    /// taken since none was named, which ([`crate::main_account::taken`]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_account: Option<Value>,
}

/// What a saved search's write names: its id.
#[derive(Clone, Debug, Serialize)]
pub struct SavedTarget {
    pub id: String,
}

impl SavedWritten {
    /// What puts the prior back, with what the offer says; or why nothing
    /// does. A create's undo is its clear at the revision written; a
    /// replace's the prior whole at the revision written, so it undoes
    /// this write and not a later one; a clear's a create of the prior
    /// whole at the id, which recreates over the tombstone as a price's
    /// undo does. A prior this build cannot read has no value to put
    /// back (C151), nor has a prior this build would refuse as a write —
    /// a stored text the language does not read (a recovery write is
    /// offered only where it would pass the writer's own checks). No
    /// write reads another row's name, so no other prior is refused.
    pub fn undo(&self) -> Result<(Way, String), String> {
        let id = &self.target.id;
        match (&self.written, &self.prior, &self.prior_value) {
            (Some(written), None, _) => Ok((
                Way::SavedSearchClear {
                    id: id.clone(),
                    if_revision: written.revision(),
                },
                "undoes it".to_string(),
            )),
            (Some(written), Some(_), Some(prior)) => {
                match refused_as_written(&Set::whole(id, prior, Expect::Current)) {
                    None => Ok((
                        Way::SavedSearchSet(Box::new(Set::whole(
                            id,
                            prior,
                            Expect::Exactly(written.revision()),
                        ))),
                        "puts the prior back, whole".to_string(),
                    )),
                    Some(why) => Err(format!(
                        "the prior cannot be put back: this build would refuse it as a write ({why})"
                    )),
                }
            }
            (None, Some(_), Some(prior)) => {
                match refused_as_written(&Set::whole(id, prior, Expect::Current)) {
                    None => Ok((
                        Way::SavedSearchSet(Box::new(Set::whole(id, prior, Expect::Create))),
                        "puts it back (a create, which lands only where no saved search stands at \
                     its id; the row's count continues past the clear)"
                            .to_string(),
                    )),
                    Some(why) => Err(format!(
                        "the cleared search cannot be put back: this build would refuse it as a \
                     write ({why})"
                    )),
                }
            }
            (Some(_), Some(_), None) => {
                Err("the prior cannot be put back: this build cannot read it".to_string())
            }
            (None, Some(_), None) => {
                Err("the cleared search cannot be put back: this build cannot read it".to_string())
            }
            (None, None, _) => Err("nothing was written".to_string()),
        }
    }
}

/// The production mint: twelve hex characters from the system's
/// randomness, as the web mints its token; fallible, as that is.
pub fn mint() -> Result<String, String> {
    let mut bytes = [0u8; ID_LEN / 2];
    getrandom::fill(&mut bytes).map_err(|e| format!("no randomness for an id: {e}"))?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

/// Whether `word` has an id's shape: twelve lowercase hex digits.
pub fn is_id(word: &str) -> bool {
    word.len() == ID_LEN
        && word
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// Every saved search, listed (C53, C149).
pub fn list(
    frontend: &Frontend,
    limit: Option<usize>,
    expand: bool,
) -> anyhow::Result<SavedSearches> {
    let user = frontend.user()?;
    let (rows, revision) = rows(&user)?;
    let total = rows.len();
    let (searches, left_out) = acquisition_store::bound::bounded(rows, limit, expand);
    Ok(SavedSearches {
        searches,
        total,
        left_out,
        left_out_needs: (left_out > 0).then_some("expand"),
        revision,
    })
}

/// One saved search, by its id or its name.
pub fn show(frontend: &Frontend, id_or_name: &str) -> anyhow::Result<SavedRow> {
    let user = frontend.user()?;
    let (rows, _) = rows(&user)?;
    let row = resolve(rows, id_or_name, |id| Way::SavedSearchShow { id })?;
    Ok(row)
}

/// The search request a saved search is, by its id or its name: what
/// every `--saved` runs. `account`, where one is named, is the account a
/// refusal's offers run the row as. A row this build cannot read is
/// refused here, when run (C151).
pub fn request_of(
    frontend: &Frontend,
    id_or_name: &str,
    account: Option<&str>,
) -> anyhow::Result<Request> {
    let user = frontend.user()?;
    let (rows, _) = rows(&user)?;
    let account = account.map(str::to_string);
    match resolve(rows, id_or_name, |id| Way::SavedSearchRun {
        id,
        account: account.clone(),
    })? {
        SavedRow::Read(row) => Ok(row.request),
        SavedRow::Unreadable(row) => Err(Pointed {
            kind: "refused",
            message: format!(
                "saved search {} at revision {} holds a value this build cannot read ({}); it \
                 is listed and cannot be run",
                row.id, row.revision, row.unreadable
            ),
            ways: vec![list_way(false)],
            fields: Map::new(),
        }
        .into()),
    }
}

/// Write a saved search ([`Set`]'s rule, the module doc). `account` is
/// the account a create resolves its realm over where `set.realm` names
/// none; `mint` makes a create's id, called at most twice.
pub fn set(
    frontend: &Frontend,
    account: Option<&str>,
    set: &Set,
    mint: &mut dyn FnMut() -> Result<String, String>,
) -> anyhow::Result<SavedWritten> {
    // every argument read before any file is opened
    match (&set.id, set.expect) {
        (None, Expect::Create) => {
            if set.name.is_none() || set.text.is_none() {
                return Err(arguments(
                    "a new saved search names its name and its query's text".into(),
                ));
            }
        }
        (None, Expect::Exactly(_) | Expect::Current) => {
            return Err(arguments(
                "a revision names the saved search it replaces by its id; a new one names none"
                    .into(),
            ));
        }
        (Some(id), expect) => {
            check_id(id)?;
            if expect == Expect::Create
                && (set.name.is_none()
                    || set.text.is_none()
                    || set.realm.is_none()
                    || set.membership.is_none())
            {
                return Err(arguments(format!(
                    "a create at the id {id} names the whole search: its name, its text, its \
                     realm and its membership"
                )));
            }
            if expect != Expect::Create
                && set.name.is_none()
                && set.text.is_none()
                && set.realm.is_none()
                && set.membership.is_none()
            {
                return Err(arguments(format!(
                    "a write over {id} names what it changes: a name, a text, a realm or a \
                     membership"
                )));
            }
        }
    }
    if let Some(name) = &set.name
        && let Some(fault) = SavedSearch::name_fault(name)
    {
        return Err(arguments(format!("{fault}: {:?}", name.trim())));
    }
    let realm = set.realm.as_deref().map(realm_word).transpose()?;
    let membership = set.membership.as_deref().map(membership_word).transpose()?;
    if let Some(text) = &set.text {
        canonical(text, realm.as_deref())?;
    }
    let mut user = frontend.user()?;
    let via = Provenance::via(frontend.via);
    let now = acquisition_store::now();
    let Some(id) = &set.id else {
        let (realm, main_account) = match realm {
            Some(realm) => (realm, None),
            None => resolved_realm(frontend, account, membership.as_deref(), set)?,
        };
        let value = SavedSearch {
            version: SavedSearch::VERSION,
            name: set.name.as_deref().unwrap_or_default().trim().to_string(),
            text: canonical(set.text.as_deref().unwrap_or_default(), Some(&realm))?,
            scope: SavedScope {
                realm,
                membership: membership.unwrap_or_else(|| "live".to_string()),
            },
        };
        let row = created(&mut user, &value, &via, mint)?;
        let mut receipt = written(&row.key.clone(), Some(row), None, None, now);
        receipt.main_account = main_account;
        return Ok(receipt);
    };
    // the row the write replaces, read once: at exactly the revision
    // named, or at the revision read; none for a create at the id
    let (prior, expected) = match set.expect {
        Expect::Create => (None, None),
        Expect::Exactly(revision) => (
            Some(stood(
                user.at_revision(SAVED_SEARCH_SCOPE, id, KIND, revision),
                revision,
                id,
                |e| refused_write(e, id, set, now),
            )?),
            Some(revision),
        ),
        Expect::Current => match user.get(SAVED_SEARCH_SCOPE, id, KIND) {
            Ok(None) => return Err(not_found(id)),
            Ok(Some(row)) => match check_value::<SavedSearch>(&row.value) {
                Ok(value) => {
                    let revision = row.revision;
                    (Some(Prior::Read(Box::new(row), value)), Some(revision))
                }
                Err(e) => return Err(blind(id, row.revision, &e.to_string())),
            },
            Err(AnnotationError::Unreadable(row)) => {
                return Err(blind(id, row.revision, &row.fault));
            }
            Err(e) => return Err(Fault::Other(e.into()).into()),
        },
    };
    let kept = match &prior {
        Some(Prior::Read(_, value)) => Some(value),
        _ => None,
    };
    if matches!(prior, Some(Prior::Unread { .. }))
        && (set.name.is_none()
            || set.text.is_none()
            || set.realm.is_none()
            || set.membership.is_none())
    {
        return Err(arguments(format!(
            "saved search {id} holds a value this build cannot read, so a write over it names \
             the whole search: its name, its text, its realm and its membership"
        )));
    }
    let realm = realm
        .or_else(|| kept.map(|v| v.scope.realm.clone()))
        .unwrap_or_default();
    let value = SavedSearch {
        version: SavedSearch::VERSION,
        name: set
            .name
            .as_deref()
            .map(|n| n.trim().to_string())
            .or_else(|| kept.map(|v| v.name.clone()))
            .unwrap_or_default(),
        text: canonical(
            set.text
                .as_deref()
                .or(kept.map(|v| v.text.as_str()))
                .unwrap_or_default(),
            Some(&realm),
        )?,
        scope: SavedScope {
            realm,
            membership: membership
                .or_else(|| kept.map(|v| v.scope.membership.clone()))
                .unwrap_or_default(),
        },
    };
    let row = user
        .put::<SavedSearch>(
            SAVED_SEARCH_SCOPE,
            id,
            &serde_json::to_value(&value)?,
            expected,
            &via,
        )
        .map_err(|e| refused_write(e, id, set, now))?;
    let (prior_row, prior_value) = match prior {
        Some(Prior::Read(row, value)) => (Some(row_of(*row, now)), Some(value)),
        Some(Prior::Unread { revision, fault }) => (
            Some(SavedRow::Unreadable(UnreadRow {
                id: id.clone(),
                revision,
                unreadable: fault,
            })),
            None,
        ),
        None => (None, None),
    };
    Ok(written(id, Some(row), prior_row, prior_value, now))
}

/// Clear a saved search at the revision named, or at the revision read
/// under [`Expect::Current`]; the clear's receipt names the prior and
/// its undo, the prior whole created again at the id.
pub fn clear(frontend: &Frontend, id: &str, expect: Expect) -> anyhow::Result<SavedWritten> {
    check_id(id)?;
    let mut user = frontend.user()?;
    let via = Provenance::via(frontend.via);
    let now = acquisition_store::now();
    let prior = match expect {
        Expect::Create => {
            return Err(arguments(
                "a clear names the revision it clears, or none for the revision that stands".into(),
            ));
        }
        Expect::Exactly(revision) => stood(
            user.at_revision(SAVED_SEARCH_SCOPE, id, KIND, revision),
            revision,
            id,
            |e| refused_clear(e, id, now),
        )?,
        Expect::Current => match user.get(SAVED_SEARCH_SCOPE, id, KIND) {
            Ok(None) => return Err(not_found(id)),
            Ok(Some(row)) => match check_value::<SavedSearch>(&row.value) {
                Ok(value) => Prior::Read(Box::new(row), value),
                Err(e) => return Err(blind(id, row.revision, &e.to_string())),
            },
            Err(AnnotationError::Unreadable(row)) => {
                return Err(blind(id, row.revision, &row.fault));
            }
            Err(e) => return Err(Fault::Other(e.into()).into()),
        },
    };
    let revision = match &prior {
        Prior::Read(row, _) => row.revision,
        Prior::Unread { revision, .. } => *revision,
    };
    let tombstone = user
        .delete(SAVED_SEARCH_SCOPE, id, KIND, revision, &via)
        .map_err(|e| refused_clear(e, id, now))?;
    let (prior_row, prior_value) = match prior {
        Prior::Read(row, value) => (row_of(*row, now), Some(value)),
        Prior::Unread { revision, fault } => (
            SavedRow::Unreadable(UnreadRow {
                id: id.to_string(),
                revision,
                unreadable: fault,
            }),
            None,
        ),
    };
    Ok(SavedWritten {
        target: SavedTarget { id: id.to_string() },
        written: None,
        prior: Some(prior_row),
        revision: tombstone,
        prior_value,
        main_account: None,
    })
}

// ---- the rows ---------------------------------------------------------------

/// A prior row as a write read it: readable, with its value; or a row
/// this build cannot read, its revision and why (C151).
enum Prior {
    Read(Box<AnnotationRow>, SavedSearch),
    Unread { revision: i64, fault: String },
}

/// The row a write naming a revision replaces, or the refusal the write
/// would raise: the conflict where another revision stands (C11).
fn stood(
    read: Result<AnnotationRow, AnnotationError>,
    revision: i64,
    id: &str,
    refuse: impl FnOnce(AnnotationError) -> anyhow::Error,
) -> anyhow::Result<Prior> {
    match read {
        Ok(row) => Ok(match check_value::<SavedSearch>(&row.value) {
            Ok(value) => Prior::Read(Box::new(row), value),
            Err(e) => Prior::Unread {
                revision: row.revision,
                fault: e.to_string(),
            },
        }),
        // the row at exactly the revision named, its value no JSON this
        // build reads: a write naming that revision replaces it (C151)
        Err(AnnotationError::Unreadable(row)) if row.revision == revision && row.key == id => {
            Ok(Prior::Unread {
                revision: row.revision,
                fault: row.fault,
            })
        }
        Err(e) => Err(refuse(e)),
    }
}

/// Every live row with the file's revision, from one transaction: the
/// readable by name then id, the unreadable after them by id.
fn rows(user: &User) -> Result<(Vec<SavedRow>, i64), Fault> {
    let listed = user
        .list(Some(SAVED_SEARCH_SCOPE), Some(KIND))
        .map_err(|e| Fault::Other(e.into()))?;
    let now = acquisition_store::now();
    let mut read = Vec::new();
    let mut unread = Vec::new();
    for row in listed.list.rows {
        match row_of_any(row, now) {
            SavedRow::Read(row) => read.push(row),
            SavedRow::Unreadable(row) => unread.push(row),
        }
    }
    for row in listed.list.unreadable {
        unread.push(UnreadRow {
            id: row.key,
            revision: row.revision,
            unreadable: row.fault,
        });
    }
    read.sort_by(|a, b| a.name.cmp(&b.name).then_with(|| a.id.cmp(&b.id)));
    unread.sort_by(|a, b| a.id.cmp(&b.id));
    let rows = read
        .into_iter()
        .map(SavedRow::Read)
        .chain(unread.into_iter().map(SavedRow::Unreadable))
        .collect();
    Ok((rows, listed.revision))
}

/// A stored row as a document carries it, readable or named (C151).
fn row_of_any(row: AnnotationRow, now: i64) -> SavedRow {
    match check_value::<SavedSearch>(&row.value) {
        Ok(_) => row_of(row, now),
        Err(e) => SavedRow::Unreadable(UnreadRow {
            id: row.key,
            revision: row.revision,
            unreadable: e.to_string(),
        }),
    }
}

/// A row this build reads, its times said at `now` (C163).
fn row_of(row: AnnotationRow, now: i64) -> SavedRow {
    let Ok(value) = check_value::<SavedSearch>(&row.value) else {
        return row_of_any(row, now);
    };
    let request = request(&value);
    SavedRow::Read(Box::new(ReadRow {
        id: row.key,
        name: value.name,
        text: value.text,
        scope: value.scope,
        revision: row.revision,
        created_at: row.created_at,
        created_at_age: acquisition_store::age::ago(now, Some(row.created_at)),
        updated_at: row.updated_at,
        updated_at_age: acquisition_store::age::ago(now, Some(row.updated_at)),
        written_via: row.written_via,
        actor: row.actor,
        request,
    }))
}

/// The search request a saved search is: its realm and membership, its
/// text; no account (a search names none until it is run as one).
fn request(value: &SavedSearch) -> Request {
    Request {
        scope: Scope {
            account: None,
            realm: corpus::Realm::parse(&value.scope.realm).ok(),
            membership: Membership::parse(&value.scope.membership).unwrap_or_default(),
        },
        query: QueryInput {
            text: Some(value.text.clone()),
            tree: None,
            trade: None,
        },
        ..Request::default()
    }
}

/// A write's receipt over the row it wrote.
fn written(
    id: &str,
    row: Option<AnnotationRow>,
    prior: Option<SavedRow>,
    prior_value: Option<SavedSearch>,
    now: i64,
) -> SavedWritten {
    let written = row.map(|row| row_of(row, now));
    SavedWritten {
        target: SavedTarget { id: id.to_string() },
        revision: written.as_ref().map_or(0, SavedRow::revision),
        written,
        prior,
        prior_value,
        main_account: None,
    }
}

/// A create under a minted id: the mint called once, and once more where
/// its id met a live row; a second meeting is refused, so the mint is
/// called at most twice and never loops.
fn created(
    user: &mut User,
    value: &SavedSearch,
    via: &Provenance,
    mint: &mut dyn FnMut() -> Result<String, String>,
) -> anyhow::Result<AnnotationRow> {
    let value = serde_json::to_value(value)?;
    for attempt in 0..2 {
        let id = mint().map_err(|why| refused(format!("no id could be minted: {why}")))?;
        match user.put::<SavedSearch>(SAVED_SEARCH_SCOPE, &id, &value, None, via) {
            Ok(row) => return Ok(row),
            Err(
                AnnotationError::Conflict { current: Some(_) } | AnnotationError::Unreadable(_),
            ) if attempt == 0 => {}
            Err(
                AnnotationError::Conflict { current: Some(_) } | AnnotationError::Unreadable(_),
            ) => {
                break;
            }
            Err(e) => return Err(Fault::Other(e.into()).into()),
        }
    }
    Err(refused(
        "two minted ids were each taken by a saved search standing; nothing was written: try \
         again"
            .into(),
    ))
}

// ---- the refusals -------------------------------------------------------------

/// The one row `id_or_name` opens: the row of that id, else the one row
/// of that name, the two names compared trimmed (a name is one to
/// sixty-four characters once trimmed, and another writer may store one
/// padded); `saved_search_not_found` where none holds it, and
/// `ambiguous_saved_search` where several rows carry the name, each
/// offered by its id as `by_id` spells the way.
fn resolve(
    rows: Vec<SavedRow>,
    id_or_name: &str,
    by_id: impl Fn(String) -> Way,
) -> anyhow::Result<SavedRow> {
    let asked = id_or_name.trim();
    let mut named = Vec::new();
    for row in rows {
        if row.id() == asked {
            return Ok(row);
        }
        // trimmed on both sides, as a name is read; the stored value stands
        if row.name().map(str::trim) == Some(asked) {
            named.push(row);
        }
    }
    match named.len() {
        0 => Err(not_found(asked)),
        1 => Ok(named.remove(0)),
        total => {
            let (listed, left_out) = acquisition_store::bound::bounded(named, None, false);
            let mut ways: Vec<(Way, String)> = listed
                .iter()
                .map(|row| {
                    (
                        by_id(row.id().to_string()),
                        format!(
                            "the saved search {} named {asked}, at revision {}",
                            row.id(),
                            row.revision()
                        ),
                    )
                })
                .collect();
            if left_out > 0 {
                ways.push(list_way(true));
            }
            let mut fields = Map::new();
            fields.insert(
                "searches".into(),
                Value::Array(
                    listed
                        .iter()
                        .map(|row| json!({ "id": row.id(), "name": asked }))
                        .collect(),
                ),
            );
            fields.insert("total".into(), json!(total));
            fields.insert("left_out".into(), json!(left_out));
            Err(Pointed {
                kind: "ambiguous_saved_search",
                message: format!(
                    "{total} saved searches are named {asked}; open one by its id{}",
                    match left_out {
                        0 => String::new(),
                        n => format!(" ({n} more are left out of the ten offered)"),
                    }
                ),
                ways,
                fields,
            }
            .into())
        }
    }
}

/// The list, or every row of it: a refusal's way to the ids.
fn list_way(expand: bool) -> (Way, String) {
    (
        Way::SavedSearches { expand },
        match expand {
            false => "the saved searches, each with its id".to_string(),
            true => "every saved search, each with its id".to_string(),
        },
    )
}

/// No live row holds `asked` as its id or its name.
fn not_found(asked: &str) -> anyhow::Error {
    Pointed {
        kind: "saved_search_not_found",
        message: format!("no saved search has the id or the name {asked}"),
        ways: vec![list_way(false)],
        fields: Map::new(),
    }
    .into()
}

/// A write naming no revision over a row this build cannot read: never
/// replaced blind (C35, C151).
fn blind(id: &str, revision: i64, fault: &str) -> anyhow::Error {
    refused(format!(
        "saved search {id} holds a value this build cannot read (revision {revision}: {fault}); \
         a write naming that revision replaces it, and a clear naming it clears it"
    ))
}

/// The store's refusal of a write as the verbs say it: a revision that
/// moved is `conflict` with the revision that stands and the row as it
/// stands (C11), offered again over that revision; a create meeting a
/// row standing at its id is the same conflict, offering that row's show.
fn refused_write(e: AnnotationError, id: &str, set: &Set, now: i64) -> anyhow::Error {
    // the write again over the row that stands, merged as `set` merges:
    // offered only where it would pass the writer's own checks (a
    // recovery write is offered only where it would land), so never a
    // partial write over a row this build cannot read
    let standing = match &e {
        AnnotationError::Conflict { current: Some(row) } => {
            check_value::<SavedSearch>(&row.value).ok()
        }
        _ => None,
    };
    let mut again = set.clone();
    if let Some(standing) = &standing {
        again.name = again.name.or_else(|| Some(standing.name.clone()));
        again.text = again.text.or_else(|| Some(standing.text.clone()));
        again.realm = again.realm.or_else(|| Some(standing.scope.realm.clone()));
        again.membership = again
            .membership
            .or_else(|| Some(standing.scope.membership.clone()));
    }
    let passes = set.expect != Expect::Create && refused_as_written(&again).is_none();
    let ways = |revision: i64| {
        let show = (
            Way::SavedSearchShow { id: id.to_string() },
            format!("the saved search {id} as it stands"),
        );
        match passes {
            true => {
                let mut set = set.clone();
                set.expect = Expect::Exactly(revision);
                vec![(
                    Way::SavedSearchSet(Box::new(set)),
                    format!("the same write over revision {revision}, the row as it stands"),
                )]
            }
            false => vec![show],
        }
    };
    conflicted(e, id, now, ways)
}

/// Why the write `set` names, its fields whole, would be refused by this
/// build's own checks — a name, a realm, a membership or a text the
/// language does not read — or nothing: what a recovery write is held to
/// before it is offered. A field it does not name is refused as missing.
fn refused_as_written(set: &Set) -> Option<String> {
    let (Some(name), Some(text), Some(realm), Some(membership)) =
        (&set.name, &set.text, &set.realm, &set.membership)
    else {
        return Some("it does not name the whole search".to_string());
    };
    if let Some(fault) = SavedSearch::name_fault(name) {
        return Some(fault);
    }
    if let Err(e) = membership_word(membership) {
        return Some(format!("{e:#}"));
    }
    let realm = match realm_word(realm) {
        Ok(realm) => realm,
        Err(e) => return Some(format!("{e:#}")),
    };
    canonical(text, Some(&realm))
        .err()
        .map(|e| format!("{e:#}"))
}

/// [`refused_write`] for a clear: the same clear offered at the revision
/// that stands.
fn refused_clear(e: AnnotationError, id: &str, now: i64) -> anyhow::Error {
    conflicted(e, id, now, |revision| {
        vec![(
            Way::SavedSearchClear {
                id: id.to_string(),
                if_revision: revision,
            },
            format!("the same clear at revision {revision}, the row as it stands"),
        )]
    })
}

/// A write's store refusal: `conflict` with `revision` (null where no row
/// stands) and `current` where a row this build reads stands; any other
/// store failure a fault of its own.
fn conflicted(
    e: AnnotationError,
    id: &str,
    now: i64,
    ways: impl Fn(i64) -> Vec<(Way, String)>,
) -> anyhow::Error {
    let (revision, current) = match e {
        AnnotationError::Conflict { current: Some(row) } => (Some(row.revision), Some(*row)),
        AnnotationError::Conflict { current: None } => (None, None),
        AnnotationError::Unreadable(row) => (Some(row.revision), None),
        other => return Fault::Other(other.into()).into(),
    };
    let mut fields = Map::new();
    fields.insert("revision".into(), json!(revision));
    if let Some(row) = current {
        fields.insert(
            "current".into(),
            serde_json::to_value(row_of_any(row, now)).unwrap_or(Value::Null),
        );
    }
    let (message, ways) = match revision {
        Some(revision) => (
            format!(
                "saved search {id} stands at revision {revision}, not the revision the write \
                 named: nothing was written"
            ),
            ways(revision),
        ),
        None => (
            format!("no saved search stands at {id}: nothing was written"),
            vec![list_way(false)],
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

/// The realm a create names none of, resolved as a search resolves one
/// over the account's store (C96): the one realm it holds, `all` where it
/// holds none or no account is known, and refused naming the realms where
/// it holds several, each offered as the same create in that realm.
fn resolved_realm(
    frontend: &Frontend,
    account: Option<&str>,
    membership: Option<&str>,
    set: &Set,
) -> anyhow::Result<(String, Option<Value>)> {
    let account = match frontend.account(account) {
        Ok(account) => account,
        Err(Fault::NoAccounts { .. }) => {
            return Ok((corpus::Realm::All.as_str().to_string(), None));
        }
        Err(fault) => return Err(fault.into()),
    };
    // C187: a realm resolved over the main account says so, in the
    // receipt and in the refusal alike (the cold review of a3bc5c38, its
    // fifth finding)
    let taken = crate::main_account::taken(&account);
    let opened = account.open()?;
    let request = Request {
        scope: Scope {
            membership: membership.and_then(Membership::parse).unwrap_or_default(),
            ..Scope::default()
        },
        ..Request::default()
    };
    match request.corpus(&opened.facts, &opened.intent) {
        Ok(corpus) => Ok((corpus.realm.as_str().to_string(), taken)),
        Err(SearchError::Scope {
            kind: "realm_needed",
            message,
            offers,
        }) => {
            let realms: Vec<String> = offers
                .iter()
                .filter_map(|o| o.request.as_ref()?.scope.realm.as_ref())
                .map(|r| r.as_str().to_string())
                .collect();
            let ways = realms
                .iter()
                .map(|realm| {
                    let mut set = set.clone();
                    set.realm = Some(realm.clone());
                    (
                        Way::SavedSearchSet(Box::new(set)),
                        format!("the same search saved over {realm}"),
                    )
                })
                .collect();
            let mut fields = Map::new();
            if let Some(taken) = taken {
                fields.insert("main_account".into(), taken);
            }
            Err(Pointed {
                kind: "realm_needed",
                message,
                ways,
                fields,
            }
            .into())
        }
        Err(e) => Err(e.into()),
    }
}

/// A realm word as a search's scope reads it, refused in the search's
/// words (C143).
fn realm_word(word: &str) -> anyhow::Result<String> {
    Ok(corpus::Realm::parse(word)?.as_str().to_string())
}

/// A membership word as a search's scope reads it, refused in its words.
fn membership_word(word: &str) -> anyhow::Result<String> {
    match Membership::parse(word) {
        Some(membership) => Ok(membership.as_str().to_string()),
        None => Err(SearchError::Scope {
            kind: "membership_unknown",
            message: Membership::unknown(word),
            offers: Vec::new(),
        }
        .into()),
    }
}

/// `text` read with the language's reader in `realm` and printed
/// canonical (C104); an authoring error is the language's refusal.
fn canonical(text: &str, realm: Option<&str>) -> anyhow::Result<String> {
    let realm = realm.map(corpus::Realm::parse).transpose()?;
    let read = QueryInput {
        text: Some(text.trim().to_string()),
        tree: None,
        trade: None,
    }
    .read(realm.as_ref())
    .map_err(SearchError::from)?;
    Ok(read.query.text())
}

/// An id is twelve hex characters, as the list prints it.
fn check_id(id: &str) -> anyhow::Result<()> {
    match is_id(id) {
        true => Ok(()),
        false => Err(arguments(format!(
            "{id:?} is no saved search's id: an id is twelve hex characters, as the list prints it"
        ))),
    }
}

fn arguments(message: String) -> anyhow::Error {
    Arguments(message).into()
}

fn refused(message: String) -> anyhow::Error {
    Pointed {
        kind: "refused",
        message,
        ways: Vec::new(),
        fields: Map::new(),
    }
    .into()
}
