//! Intent (annotations): the only irreplaceable local state (C35). Facts
//! are refetchable at the cost of requests; intent has no server to
//! refetch from, so it lives in its own per-account file — named by the
//! account **uuid** (stable across renames), never the username — and is
//! written only through this API, with integer-revision compare-and-swap
//! so two frontends cannot silently clobber each other.
//!
//! No fact-side event ever deletes intent: an annotation whose item is
//! removed is kept and surfaceable as orphaned
//! ([`crate::Store::orphaned_item_annotations`]). Deleting an annotation is
//! itself an act of intent, so [`Annotations::delete`] exists — for
//! frontends, under the same compare-and-swap — and it *tombstones* rather
//! than removes: the revision sequence survives a delete/recreate, so a
//! writer holding a pre-delete revision can never silently overwrite the
//! recreated value (the ABA hole a hard delete would open).
//!
//! Rows are addressed `(scope, key, kind)`:
//! - `scope` is what kind of thing the key names: `"item"` (key = GGG item
//!   id), `"character"` (key = GGG character id), `"tab"` (key =
//!   `<realm>/<id>`), `"substash"` (key = `<realm>/<parent>/<id>`),
//!   `"account"` (key = `""` for per-account singletons like the sync
//!   policy). The realm-bearing keys are rendered and parsed by one type,
//!   `acquisition_plan::price::PriceTarget` (C67), defined before the
//!   first tab-scoped row landed; the store carries them as text.
//! - `kind` is what the annotation says: `"buyout"`, `"note"`,
//!   `"sync-policy"`, …
//! - `value` is JSON; its shape is the kind's business, declared through
//!   [`IntentValue`] and enforced at the write door ([`Annotations::put`]).
//!
//! Backup is store-managed: [`Annotations::export`] writes a consistent
//! snapshot via `VACUUM INTO` — a raw file copy under WAL is not a backup.
//!
//! # Decisions as recorded
//!
//! The rulings are the decision registry — `decisions/plans.md` and
//! `decisions/pricing.md` for this area, `AGENTS.md` for the cross-cutting
//! ones (`C<n>`); what follows is each entry's full text as recorded there,
//! moved here because the mechanism it describes is this module's. The
//! registry is current; this is the mechanism as decided, kept beside the
//! code that implements it.
//!
//! ## C35 — Annotations are the only irreplaceable local state.
//!
//! **Annotations are the only irreplaceable local state.** A separate per-account file named by the account uuid (identity decision in "Multi-account design"), keyed on stable GGG ids, written only through the store crate with integer-revision compare-and-swap; no fact-side event ever deletes intent — an annotation whose item is removed is kept and surfaceable as orphaned; export/backup is a store-managed consistent snapshot (`VACUUM INTO` / SQLite backup API — a raw file copy under WAL is not a backup). Rationale: facts are refetchable at the cost of requests; intent has no server to refetch from — the C++ legacy-buyout saga is the full price of getting this wrong. Decided 2026-08-31.
//!
//! ## C151 — An annotation row this build cannot read is named, never dropped, never a wall, and never repaired by a read.
//!
//! Every door reads a row as stored first (`Stored`, its value still
//! text). [`Annotations::list`] parses each and answers an
//! [`AnnotationList`]: the readable rows, and beside them each
//! [`UnreadableRow`] by its scope, key, kind, revision and parse fault.
//! [`Annotations::get`] answers [`AnnotationError::Unreadable`] with the
//! same facts. [`Annotations::put`] and [`Annotations::delete`] naming
//! the row's revision never parse the value they replace, so the one
//! compare-and-swap (C35) can clear or replace such a row; at a wrong
//! revision they answer `Unreadable` with the revision the row stands
//! at, where a readable row would answer a `Conflict` carrying itself.
//! Nothing rewrites a value in place and no read repairs. Rationale: one
//! damaged row hid every price and could not be cleared by the only API
//! that may write the file (issue #63).
//!
//! ## C65 — Every intent write carries structured provenance
//!
//! **Every intent write carries structured provenance: the channel it came
//! through (`written_via`) and an optional claimed `actor`; the hash of the
//! plan that landed it (`applied_plan`, C71) joins when receipts exist.**
//! Stored on the row (annotations v3), returned on every read;
//! `written_via` required by the write API; `actor` is untrusted audit
//! metadata, never identity or authorization; origin detail lives on the
//! receipt (C78). v3 is the floor: a file below it is refused, never
//! migrated — every row on record has a writer. Ruled 2026-09-03;
//! `applied_plan` deferred 2026-09-04; the floor 2026-09-05.
//!
//! As built: [`Provenance`] is a required argument of [`Annotations::put`]
//! and [`Annotations::delete`] (a tombstone records who cleared the row);
//! `written_via` must be a non-empty single word. Files below v3 were
//! written only by development builds (the owner's held one policy row),
//! so rather than carry a migration that would have to invent a writer
//! for rows that had none, the open refuses them
//! ([`AnnotationError::SchemaTooOld`]) naming the file and the fix; a
//! future column is added by a stepwise `ALTER TABLE` in the same
//! IMMEDIATE transaction, the way the facts store migrates.
//!
//! ## C66 — Intent values are typed at the write API
//!
//! **Intent values are typed at the write API: a kind declares its schema
//! version and a strict parser, a value that does not parse under its
//! stamp never lands, and a current-schema value re-serializes to exactly
//! what was read.** The generic (version gate, unknown fields refused,
//! exact round-trip, compare-and-swap) lives in the store crate over a
//! per-kind trait; a kind's shape stays its owner's; an older value
//! upgrades in memory, stored as written. A kind may also refuse as a
//! *new write* what it reads (`check_write`; C67's retired tag first) —
//! at the door, every write, never a read. Ruled 2026-09-03; amended
//! 2026-09-07.
//!
//! As built: [`IntentValue`] is the per-kind trait (the kind's name, the
//! version this build writes, its strict parse); [`check_value`] is the
//! generic, in this order: an integer `version` stamp, refused above the
//! kind's version *before* the shape is read (a newer value is reported as
//! such, never as a typo); the kind's own parse (each kind's wire structs
//! carry `deny_unknown_fields` at every depth); then, for a value stamped
//! with the current version only, the parsed value must re-serialize to
//! exactly what was read — the first differing path is named — which
//! closes the holes serde leaves open (`null` for an absent field, an
//! empty list for "none", an extra field beside a unit variant's tag). A
//! value stamped older is upgraded in memory by the kind's parse and
//! stored as written. [`Annotations::put`] runs the generic before the
//! compare-and-swap, so there is no untyped write door: the store crate
//! itself cannot land a value its kind refuses. A kind may also refuse a
//! value *as a new write* that it reads without complaint
//! ([`IntentValue::check_write`], 2026-09-07): the writer's rules — the
//! first is C67's, a new price never names a retired currency — run at
//! the door on every write and never on a read, so a stored row keeps
//! saying what it said and no frontend can write past the rule.

use std::path::{Path, PathBuf};

use rusqlite::{Connection, ErrorCode, OpenFlags, OptionalExtension, TransactionBehavior, params};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::index::filename_safe;

/// The schema this build reads and writes. A file stamped newer is refused
/// (never auto-migrated: this is the one file that must not be damaged);
/// additions like the deferred event log bump this and migrate forward
/// from [`SCHEMA_FLOOR`]. v2 added `meta`, which carries the account uuid
/// *inside* the file so a copied or renamed database cannot silently pair
/// with another account's facts — the filename convention alone was
/// bypassable. v3 (2026-09-05, C65) added `written_via` and `actor` to
/// every row.
const SCHEMA_VERSION: i64 = 3;

/// The oldest schema this build opens. Files below it were written only
/// by development builds and are refused rather than migrated: a row
/// without provenance would need a writer invented for it (C65).
const SCHEMA_FLOOR: i64 = 3;

/// C124: the read-only door's remedy for a file below [`SCHEMA_FLOOR`],
/// in the words [`AnnotationError::SchemaTooOld`] gives for it, so the
/// read-only door and the writing door tell the owner the same thing.
const BELOW_FLOOR_REMEDY: &str = "a development build wrote it; delete it and set the policy again";

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS annotations (
    scope       TEXT    NOT NULL,
    key         TEXT    NOT NULL,
    kind        TEXT    NOT NULL,
    value       TEXT    NOT NULL,   -- JSON
    revision    INTEGER NOT NULL,   -- 1 on create, +1 per write, delete included
    created_at  INTEGER NOT NULL,   -- unix seconds
    updated_at  INTEGER NOT NULL,
    deleted_at  INTEGER,            -- tombstone; the revision keeps counting
    written_via TEXT    NOT NULL,   -- the channel of the last write
    actor       TEXT,               -- the writer's claim, untrusted
    PRIMARY KEY (scope, key, kind)
);
CREATE TABLE IF NOT EXISTS meta (
    key    TEXT PRIMARY KEY,        -- 'account_uuid'
    value  TEXT NOT NULL
);
";

/// The `meta` key holding the owning account's uuid.
const META_UUID: &str = "account_uuid";

/// How long a writer waits for another connection's write lock before
/// giving up as [`AnnotationError::Busy`], by default.
const BUSY_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

/// C137, the deadline's contract is its expiry behaviour:
/// `ACQ_ANNOTATION_WAIT=<secs>` overrides [`BUSY_TIMEOUT`], read at each
/// open ([`parse_annotation_wait`]). The test isolation list widens it,
/// so a starved runner cannot fail a healthy writer; zero refuses a held
/// file at once.
fn annotation_wait() -> std::time::Duration {
    std::env::var("ACQ_ANNOTATION_WAIT")
        .ok()
        .map_or(BUSY_TIMEOUT, |v| parse_annotation_wait(&v))
}

/// The largest wait a knob may name, in seconds: the largest whose
/// milliseconds fit SQLite's signed 32-bit busy timeout, and far below
/// any `Instant` addition.
const MAX_WAIT_SECS: u64 = 2_147_483;

/// C137: the knob's value as the wait it names. A value the consumer
/// cannot represent — over [`MAX_WAIT_SECS`], or not a whole number of
/// seconds — is unsupported and the default stands; zero is valid and
/// forces the expiry.
fn parse_annotation_wait(value: &str) -> std::time::Duration {
    value
        .trim()
        .parse::<u64>()
        .ok()
        .filter(|secs| *secs <= MAX_WAIT_SECS)
        .map_or(BUSY_TIMEOUT, std::time::Duration::from_secs)
}

/// The busy wait SQLite enforces on `conn`, read back from it
/// (`PRAGMA busy_timeout`, milliseconds): what a busy refusal reports,
/// so its words are the connection's fact and not the knob's (C137).
fn enforced_wait(conn: &Connection) -> Option<std::time::Duration> {
    conn.query_row("PRAGMA busy_timeout", [], |r| r.get::<_, i64>(0))
        .ok()
        .and_then(|ms| u64::try_from(ms).ok())
        .map(std::time::Duration::from_millis)
}

/// A result of work on `conn`, its busy refusal given the wait the
/// connection enforced. Every door that takes the write lock reports
/// through here.
fn reported<T>(conn: &Connection, r: Result<T, AnnotationError>) -> Result<T, AnnotationError> {
    match r {
        Err(AnnotationError::Busy { waited: None }) => Err(AnnotationError::Busy {
            waited: enforced_wait(conn),
        }),
        other => other,
    }
}

/// `<dir>/<uuid>.annotations.db` — beside the username-named fact files,
/// but keyed by the identity that survives a rename.
pub fn annotations_path(dir: &Path, uuid: &str) -> PathBuf {
    dir.join(format!("{}.annotations.db", filename_safe(uuid)))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnnotationRow {
    pub scope: String,
    pub key: String,
    pub kind: String,
    pub value: Value,
    pub revision: i64,
    pub created_at: i64,
    pub updated_at: i64,
    /// The channel the last write came through (C65): `cli`, `mcp`, ….
    pub written_via: String,
    /// What the writer claimed about who was acting. Untrusted audit
    /// metadata: never identity, never authorization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
}

/// A live row whose stored value this build cannot parse as JSON (C151):
/// where it is, the revision a write must name to clear or replace it,
/// and the parse fault in the parser's words (which never quote the
/// value, C127).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnreadableRow {
    pub scope: String,
    pub key: String,
    pub kind: String,
    pub revision: i64,
    pub fault: String,
}

/// What [`Annotations::list`] answers (C151): every readable live row,
/// and beside them every live row this build cannot read, named, so one
/// damaged row neither hides the rest nor disappears.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnnotationList {
    pub rows: Vec<AnnotationRow>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unreadable: Vec<UnreadableRow>,
}

/// One row as the table holds it, its value still the text as written:
/// what every door reads first (C151), so a write naming the row's
/// revision never parses the value it replaces, and a read that cannot
/// parse it names the row instead of failing.
struct Stored {
    scope: String,
    key: String,
    kind: String,
    value: String,
    revision: i64,
    created_at: i64,
    updated_at: i64,
    written_via: String,
    actor: Option<String>,
    deleted: bool,
}

impl Stored {
    /// The row with its value parsed, or the row named as unreadable.
    fn parsed(self) -> Result<AnnotationRow, UnreadableRow> {
        match serde_json::from_str(&self.value) {
            Ok(value) => Ok(self.with_value(value)),
            Err(e) => Err(UnreadableRow {
                scope: self.scope,
                key: self.key,
                kind: self.kind,
                revision: self.revision,
                fault: e.to_string(),
            }),
        }
    }

    /// The row's metadata around a value given, the stored text unread:
    /// what a write naming the revision builds on.
    fn with_value(self, value: Value) -> AnnotationRow {
        AnnotationRow {
            scope: self.scope,
            key: self.key,
            kind: self.kind,
            value,
            revision: self.revision,
            created_at: self.created_at,
            updated_at: self.updated_at,
            written_via: self.written_via,
            actor: self.actor,
        }
    }
}

/// Who is writing, as the write API requires it (C65). Built once per
/// frontend surface, passed by reference to every write.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    /// The channel: a non-empty single word (`cli`, `mcp`, a test's name).
    pub written_via: String,
    /// An optional claim about the actor, recorded verbatim.
    pub actor: Option<String>,
}

impl Provenance {
    /// A write through `channel` with no actor claim.
    pub fn via(channel: &str) -> Provenance {
        Provenance {
            written_via: channel.into(),
            actor: None,
        }
    }

    /// The same channel, with an actor claim attached.
    pub fn as_actor(mut self, actor: &str) -> Provenance {
        self.actor = Some(actor.into());
        self
    }

    fn check(&self) -> Result<(), AnnotationError> {
        let via = &self.written_via;
        let detail = if via.is_empty() {
            "`written_via` is empty"
        } else if via.chars().any(char::is_whitespace) {
            "`written_via` must be a single word"
        } else {
            return Ok(());
        };
        Err(AnnotationError::Provenance {
            detail: detail.into(),
        })
    }
}

/// One kind of intent value (C66): the row's `kind`, the schema version
/// this build writes, and the strict parse that turns a stored JSON value
/// into the kind's own type. The generic checks — the version gate, the
/// exact round-trip, the compare-and-swap — are [`check_value`] and
/// [`Annotations::put`]; a kind supplies only its shape.
///
/// `parse` receives a value whose integer `version` is at most
/// [`IntentValue::VERSION`] and returns the in-memory form: for the
/// current version, exactly what was read; for an older one, the upgrade.
/// The wire structs behind it carry `deny_unknown_fields` at every depth,
/// so a typo is a structured error, never intent half-honored. `Serialize`
/// must produce the current wire shape, since a current-schema value is
/// held to re-serializing to exactly what was read.
pub trait IntentValue: Sized + Serialize {
    /// The `kind` column.
    const KIND: &'static str;
    /// The `version` stamp this build writes.
    const VERSION: i64;
    /// The kind's own strict parse; the detail names what was wrong.
    fn parse(value: &Value) -> Result<Self, String>;
    /// The writer's rules: what a value that parses may still not say
    /// when *newly written* — a stored row may say it forever (C67's
    /// retired currency tag is the first). Run by [`Annotations::put`]
    /// after the parse and before the compare-and-swap, on every write
    /// through every frontend, and never by a read: `get_as` and
    /// [`check_value`] do not call it. The detail names what was refused.
    fn check_write(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Why a value is not a `K` (C66). Stable kinds so a frontend can render
/// "newer than this build" differently from "a typo".
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueError {
    /// No integer `version` stamp.
    MissingVersion { kind: &'static str },
    /// Stamped newer than this build's parser for the kind.
    VersionUnsupported {
        kind: &'static str,
        found: i64,
        supported: i64,
    },
    /// The kind's parse refused it.
    Malformed { kind: &'static str, detail: String },
    /// Stamped with the current version, but the parsed value
    /// re-serializes to something else: `path` is the first difference
    /// (`/realms/pc/leagues/Standard/tabs`), `read` what the value said
    /// there, `canonical` what the kind writes there — `None` on either
    /// side is "absent" (`null` and absent are not the same value).
    NotCanonical {
        kind: &'static str,
        path: String,
        read: Option<Value>,
        canonical: Option<Value>,
    },
    /// Parses, but the kind's writer's rule refuses it as a new write
    /// ([`IntentValue::check_write`]); a stored row saying the same still
    /// reads.
    RefusedForWrite { kind: &'static str, detail: String },
}

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueError::MissingVersion { kind } => {
                write!(f, "{kind}: missing integer `version`")
            }
            ValueError::VersionUnsupported {
                kind,
                found,
                supported,
            } => write!(
                f,
                "{kind} declares version {found}, newer than this build's v{supported}"
            ),
            ValueError::Malformed { kind, detail } => write!(f, "{kind}: {detail}"),
            ValueError::RefusedForWrite { kind, detail } => write!(f, "{kind}: {detail}"),
            ValueError::NotCanonical {
                kind,
                path,
                read,
                canonical,
            } => {
                let side = |v: &Option<Value>| match v {
                    Some(v) => v.to_string(),
                    None => "absent".into(),
                };
                write!(
                    f,
                    "{kind}: not in canonical form at {path}: read {}, the canonical form has {}",
                    side(read),
                    side(canonical)
                )
            }
        }
    }
}

impl std::error::Error for ValueError {}

/// The generic half of every typed write and read (C66): the version
/// gate, the kind's strict parse, and — for a current-schema value — the
/// exact round-trip. Pure; the store calls it before the compare-and-swap.
pub fn check_value<K: IntentValue>(value: &Value) -> Result<K, ValueError> {
    let found = value
        .get("version")
        .and_then(Value::as_i64)
        .ok_or(ValueError::MissingVersion { kind: K::KIND })?;
    if found > K::VERSION {
        return Err(ValueError::VersionUnsupported {
            kind: K::KIND,
            found,
            supported: K::VERSION,
        });
    }
    let parsed = K::parse(value).map_err(|detail| ValueError::Malformed {
        kind: K::KIND,
        detail,
    })?;
    if found == K::VERSION {
        let canonical = serde_json::to_value(&parsed).map_err(|e| ValueError::Malformed {
            kind: K::KIND,
            detail: format!("does not serialize: {e}"),
        })?;
        if let Some((path, read, canonical)) = first_difference(value, &canonical, "") {
            return Err(ValueError::NotCanonical {
                kind: K::KIND,
                path,
                read,
                canonical,
            });
        }
    }
    Ok(parsed)
}

/// The first JSON path at which `read` and `canonical` differ, with both
/// sides there (`None`: absent); `None` overall when equal. Object keys
/// are compared as sets (an object's key order is not a difference),
/// arrays element by element.
type Difference = (String, Option<Value>, Option<Value>);

fn first_difference(read: &Value, canonical: &Value, path: &str) -> Option<Difference> {
    match (read, canonical) {
        (Value::Object(a), Value::Object(b)) => {
            let mut keys: Vec<&String> = a.keys().chain(b.keys()).collect();
            keys.sort();
            keys.dedup();
            for k in keys {
                let sub = format!("{path}/{k}");
                match (a.get(k), b.get(k)) {
                    (Some(x), Some(y)) => {
                        if let Some(d) = first_difference(x, y, &sub) {
                            return Some(d);
                        }
                    }
                    (Some(x), None) => return Some((sub, Some(x.clone()), None)),
                    (None, Some(y)) => return Some((sub, None, Some(y.clone()))),
                    (None, None) => {}
                }
            }
            None
        }
        (Value::Array(a), Value::Array(b)) => {
            for (i, (x, y)) in a.iter().zip(b).enumerate() {
                if let Some(d) = first_difference(x, y, &format!("{path}/{i}")) {
                    return Some(d);
                }
            }
            (a.len() != b.len()).then(|| {
                let i = a.len().min(b.len());
                (format!("{path}/{i}"), a.get(i).cloned(), b.get(i).cloned())
            })
        }
        (a, b) => (a != b).then(|| {
            (
                if path.is_empty() {
                    "/".into()
                } else {
                    path.into()
                },
                Some(a.clone()),
                Some(b.clone()),
            )
        }),
    }
}

#[derive(Debug)]
pub enum AnnotationError {
    /// The compare-and-swap failed: someone else wrote (or removed) this
    /// annotation since the caller read it. `current` is the row as it
    /// exists now (`None`: no such annotation) — enough to re-read, merge,
    /// and retry without another round trip. Boxed so the error stays small.
    Conflict {
        current: Option<Box<AnnotationRow>>,
    },
    /// The file was written by a newer build. Refused rather than guessed
    /// at: annotations are the state that must never be damaged.
    SchemaTooNew {
        found: i64,
        supported: i64,
    },
    /// The file predates the schema floor (a development build wrote it),
    /// or — `found: 0` — exists with content and no stamp at all. Refused
    /// rather than migrated or adopted: its rows have no writer to record,
    /// and its content is not this build's to discard.
    SchemaTooOld {
        found: i64,
        floor: i64,
        path: PathBuf,
    },
    /// C124 (a read never migrates): the read-only door met a file at
    /// another schema version than this build's, older or newer; nothing
    /// was written to it.
    NotCurrent(crate::SchemaNotCurrent),
    /// The file carries another account's uuid: a copy or rename cannot
    /// silently pair one account's intent with another account's facts.
    WrongAccount {
        stored: String,
        requested: String,
    },
    /// The value is not one its kind accepts (C66); nothing landed.
    Invalid(ValueError),
    /// The write named no acceptable channel (C65); nothing landed.
    Provenance {
        detail: String,
    },
    /// Another connection held the write lock past the busy timeout. The
    /// row is untouched and unread: retry later — distinct from
    /// [`AnnotationError::Conflict`], whose remedy is re-read and retry.
    /// `waited` is the wait the connection enforced, read back from it;
    /// `None` only where no door that takes the write lock met it.
    Busy {
        waited: Option<std::time::Duration>,
    },
    /// The row stands at the last revision an `i64` holds, so no write can
    /// take the next one (C47: a stored value at the bound is a structured
    /// refusal, never an overflow); nothing was written. Reached only by a
    /// damaged revision or a write count no use makes.
    RevisionExhausted {
        scope: String,
        key: String,
        kind: String,
        revision: i64,
    },
    /// C151: a live row this build cannot read, met by a read or by a
    /// write that did not name its revision. The row stands; a clear or a
    /// write naming the revision carried here replaces it.
    Unreadable(UnreadableRow),
    Db(rusqlite::Error),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for AnnotationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnotationError::Conflict { current: Some(row) } => write!(
                f,
                "annotation {}/{}/{} is at revision {} (re-read and retry)",
                row.scope, row.key, row.kind, row.revision
            ),
            AnnotationError::Conflict { current: None } => {
                write!(f, "annotation no longer exists (re-read and retry)")
            }
            AnnotationError::SchemaTooNew { found, supported } => write!(
                f,
                "annotation file uses schema v{found}, newer than this build's v{supported}"
            ),
            AnnotationError::SchemaTooOld { found: 0, path, .. } => write!(
                f,
                "annotation file {} exists with content but no schema stamp; not opened — \
                 move it aside if it holds nothing you need, then retry",
                path.display()
            ),
            AnnotationError::SchemaTooOld { found, floor, path } => write!(
                f,
                "annotation file {} uses schema v{found}, below this build's floor v{floor} \
                 (a development build wrote it); delete it and set the policy again",
                path.display()
            ),
            AnnotationError::NotCurrent(e) => write!(f, "{e}"),
            AnnotationError::WrongAccount { stored, requested } => write!(
                f,
                "annotation file belongs to account uuid {stored}, not {requested}"
            ),
            AnnotationError::Invalid(e) => write!(f, "{e}"),
            AnnotationError::Provenance { detail } => write!(f, "annotation write: {detail}"),
            AnnotationError::Busy {
                waited: Some(waited),
            } => write!(
                f,
                "annotation file is busy: another writer held it past {} s (retry later)",
                waited.as_secs()
            ),
            AnnotationError::Busy { waited: None } => write!(
                f,
                "annotation file is busy: another writer held it (retry later)"
            ),
            AnnotationError::RevisionExhausted {
                scope,
                key,
                kind,
                revision,
            } => write!(
                f,
                "annotation {scope}/{key}/{kind} is at revision {revision}, the last a row can \
                 take: nothing was written"
            ),
            AnnotationError::Unreadable(row) => write!(
                f,
                "annotation {}/{}/{} at revision {} cannot be read ({}); a clear or a write naming \
                 that revision replaces it",
                row.scope, row.key, row.kind, row.revision, row.fault
            ),
            AnnotationError::Db(e) => write!(f, "annotation store: {e}"),
            AnnotationError::Io(e) => write!(f, "annotation store: {e}"),
            AnnotationError::Json(e) => write!(f, "annotation store: {e}"),
        }
    }
}

impl std::error::Error for AnnotationError {}

impl From<UnreadableRow> for AnnotationError {
    fn from(row: UnreadableRow) -> Self {
        AnnotationError::Unreadable(row)
    }
}

impl From<rusqlite::Error> for AnnotationError {
    fn from(e: rusqlite::Error) -> Self {
        match &e {
            rusqlite::Error::SqliteFailure(f, _)
                if matches!(f.code, ErrorCode::DatabaseBusy | ErrorCode::DatabaseLocked) =>
            {
                AnnotationError::Busy { waited: None }
            }
            _ => AnnotationError::Db(e),
        }
    }
}

impl From<std::io::Error> for AnnotationError {
    fn from(e: std::io::Error) -> Self {
        AnnotationError::Io(e)
    }
}

impl From<serde_json::Error> for AnnotationError {
    fn from(e: serde_json::Error) -> Self {
        AnnotationError::Json(e)
    }
}

impl From<ValueError> for AnnotationError {
    fn from(e: ValueError) -> Self {
        AnnotationError::Invalid(e)
    }
}

pub struct Annotations {
    conn: Connection,
    path: PathBuf,
    /// The owning account's uuid as stored in the file's `meta` table;
    /// `None` for a file created through [`Annotations::open`] on a raw
    /// path and never bound via [`Annotations::open_for`].
    uuid: Option<String>,
}

impl Annotations {
    /// Open (or create) the account's annotations file under `dir`, bound
    /// to `uuid`: the uuid is stored inside the file, and a file already
    /// carrying a different account's uuid is refused
    /// ([`AnnotationError::WrongAccount`]) — a copy or rename cannot
    /// silently pair another account's intent. A file with no stored uuid
    /// (created through [`Annotations::open`] on a raw path) is stamped
    /// with the uuid it is addressed by on this open.
    /// This is the way to open annotations for real use; [`Annotations::open`]
    /// on a raw path yields a handle without a verified identity, which
    /// [`crate::Store::refresh_snapshot`] refuses.
    pub fn open_for(dir: &Path, uuid: &str) -> Result<Annotations, AnnotationError> {
        let mut a = Self::open(&annotations_path(dir, uuid))?;
        let bound = a.bind(uuid);
        reported(&a.conn, bound)?;
        Ok(a)
    }

    /// C124 (a read never migrates): the annotations file's read-only
    /// door, beside [`crate::Store::open_read_only`], for a tool that
    /// presents itself as a read. The account's file under `dir` is opened
    /// with SQLite's read-only flag and never created; nothing is written
    /// — no journal-mode switch, no stamp, no uuid bound — and only the
    /// busy timeout is set. A file at any version but this build's is
    /// refused with [`AnnotationError::NotCurrent`], whose remedy for an
    /// older file is the writing door's: below the schema floor the words
    /// [`AnnotationError::SchemaTooOld`] gives, from it up the migration
    /// `init` would run. One carrying another account's uuid is refused
    /// with [`AnnotationError::WrongAccount`]. A file with
    /// no stored uuid, which [`Annotations::open_for`] would stamp, opens
    /// with none, and a read that pairs intent with facts refuses it.
    pub fn open_read_only_for(dir: &Path, uuid: &str) -> Result<Annotations, AnnotationError> {
        let path = annotations_path(dir, uuid);
        if !path.is_file() {
            return Err(AnnotationError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "no annotation file at {}: a read-only open never creates one",
                    path.display()
                ),
            )));
        }
        let conn = Connection::open_with_flags(
            &path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        conn.busy_timeout(annotation_wait())?;
        let found: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
        if found != SCHEMA_VERSION {
            return Err(AnnotationError::NotCurrent(crate::SchemaNotCurrent {
                file: "annotation",
                found,
                supported: SCHEMA_VERSION,
                remedy: if found < SCHEMA_FLOOR {
                    BELOW_FLOOR_REMEDY
                } else {
                    crate::MIGRATE_REMEDY
                },
            }));
        }
        let stored: Option<String> = conn
            .query_row("SELECT value FROM meta WHERE key = ?1", [META_UUID], |r| {
                r.get(0)
            })
            .optional()?;
        if let Some(stored) = &stored
            && stored != uuid
        {
            return Err(AnnotationError::WrongAccount {
                stored: stored.clone(),
                requested: uuid.to_string(),
            });
        }
        Ok(Annotations {
            conn,
            path,
            uuid: stored,
        })
    }

    /// An in-memory file bound to `uuid`, for tests and ephemeral use.
    pub fn open_memory_for(uuid: &str) -> Result<Annotations, AnnotationError> {
        let mut a = Self::open_memory()?;
        let bound = a.bind(uuid);
        reported(&a.conn, bound)?;
        Ok(a)
    }

    /// Open a file by path, creating it if it is missing. A file that
    /// holds tables but carries no schema stamp is refused, not adopted
    /// as empty intent: whatever it holds is not this build's to discard
    /// ([`AnnotationError::SchemaTooOld`] with `found: 0`). A zero-byte
    /// or table-less file holds nothing and is created over — the C++ app
    /// has left such a file beside a real one before, and our own create
    /// is one transaction, so a crash mid-create leaves exactly that.
    pub fn open(path: &Path) -> Result<Annotations, AnnotationError> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        Self::init(Connection::open(path)?, path.to_path_buf())
    }

    pub fn open_memory() -> Result<Annotations, AnnotationError> {
        Self::init(Connection::open_in_memory()?, PathBuf::from(":memory:"))
    }

    /// Whether the file holds any table — committed state, as this
    /// connection sees it. A stampless file with tables is someone else's;
    /// one without is empty, or a creation another process has not yet
    /// committed.
    fn has_tables(conn: &Connection) -> rusqlite::Result<bool> {
        let n: i64 = conn.query_row(
            "SELECT count(*) FROM sqlite_master WHERE type = 'table'",
            [],
            |r| r.get(0),
        )?;
        Ok(n > 0)
    }

    /// The two boundaries of a first open (C35, the creation race and its
    /// two windows, `PRICING-SLICE.md` at `b6197a82` findings 7.3 and the 2026-09-09
    /// review): a losing first-time opener waits for the winner's create
    /// to commit and then proceeds; a stampless file that holds tables is
    /// refused and left byte for byte as found. So nothing that writes
    /// to the file — the journal-mode switch above all — runs before the
    /// refusal, and the refusal is decided under the write lock, where a
    /// creation in flight has either committed its stamp or not begun.
    ///
    /// A third window, closed 2026-09-11 (the daemon split's review
    /// round 25): the read-only gate below read the stamp and the tables
    /// as two implicit statements, and a winner's commit could land
    /// between them — the loser saw stamp 0 from before the commit and
    /// tables from after it, and refused a well-formed file as "content
    /// but no schema stamp" (seen once in the gate, `price_story.rs`,
    /// two blind writers on one row). The gate now reads both facts
    /// inside one deferred transaction — one snapshot, still no write —
    /// so a stamp and its tables are always seen from the same moment.
    fn init(mut conn: Connection, path: PathBuf) -> Result<Annotations, AnnotationError> {
        let prepared = Self::prepare(&mut conn, &path);
        let uuid = reported(&conn, prepared)?;
        Ok(Annotations { conn, path, uuid })
    }

    /// [`Annotations::init`]'s work on the connection: the settings, the
    /// version gate, the create under the lock, the switch to WAL, and
    /// the stored uuid.
    fn prepare(conn: &mut Connection, path: &Path) -> Result<Option<String>, AnnotationError> {
        // Connection settings, not file writes: the busy timeout is what
        // makes the lock below a wait rather than a refusal; FULL, not the
        // fact store's NORMAL, because this is the one file with no server
        // to refetch from (C35) and a batch is one human-paced commit.
        conn.busy_timeout(annotation_wait())?;
        conn.pragma_update(None, "synchronous", "FULL")?;
        // The version gate, read-only, before any pragma that writes: a
        // file this build refuses is left exactly as it was found. A stamp
        // of 0 with tables is a foreign file; 0 without is empty, or a
        // create another process holds uncommitted — the lock decides.
        // Both facts from one snapshot: a deferred transaction writes
        // nothing and holds the read across the two statements, so a
        // create committing between them cannot show its tables beside a
        // stamp read before it (round 25).
        let (found, tables) = {
            let gate = conn.transaction_with_behavior(TransactionBehavior::Deferred)?;
            let found: i64 = gate.query_row("PRAGMA user_version", [], |r| r.get(0))?;
            let tables = found == 0 && Self::has_tables(&gate)?;
            gate.rollback()?;
            (found, tables)
        };
        if found > SCHEMA_VERSION {
            return Err(AnnotationError::SchemaTooNew {
                found,
                supported: SCHEMA_VERSION,
            });
        }
        if found < SCHEMA_FLOOR && (found != 0 || tables) {
            return Err(AnnotationError::SchemaTooOld {
                found,
                floor: SCHEMA_FLOOR,
                path: path.to_path_buf(),
            });
        }
        // Creation (and, from the floor up, migration) serializes under
        // one immediate transaction, taken in whatever journal mode the
        // file is in: two processes creating one file wait here, and the
        // loser sees the winner's stamp. A later version adds its columns
        // here by a stepwise `ALTER TABLE` per version, never by rewriting
        // a row. BEGIN IMMEDIATE writes nothing; a refusal from inside it
        // rolls back to the file as found.
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let found: i64 = tx.query_row("PRAGMA user_version", [], |r| r.get(0))?;
        match found {
            0 if Self::has_tables(&tx)? => {
                // Tables landed between the read-only gate and the lock,
                // and no stamp with them: a foreign writer, refused here.
                return Err(AnnotationError::SchemaTooOld {
                    found: 0,
                    floor: SCHEMA_FLOOR,
                    path: path.to_path_buf(),
                });
            }
            0 => {
                tx.execute_batch(SCHEMA)?;
                tx.pragma_update(None, "user_version", SCHEMA_VERSION)?;
            }
            v if v == SCHEMA_VERSION => {}
            v if v < SCHEMA_FLOOR => {
                return Err(AnnotationError::SchemaTooOld {
                    found: v,
                    floor: SCHEMA_FLOOR,
                    path: path.to_path_buf(),
                });
            }
            v => {
                return Err(AnnotationError::SchemaTooNew {
                    found: v,
                    supported: SCHEMA_VERSION,
                });
            }
        }
        tx.commit()?;
        // The file is ours from here: WAL like the fact store — one writer
        // at a time, any number of readers — under `crate::ensure_wal`'s
        // retry, since the two openers above may reach this switch
        // together.
        crate::ensure_wal(conn)?;
        Ok(conn
            .query_row("SELECT value FROM meta WHERE key = ?1", [META_UUID], |r| {
                r.get(0)
            })
            .optional()?)
    }

    /// Bind this handle to `uuid`: stamp it into a file that has none, or
    /// verify it against the stored one.
    fn bind(&mut self, uuid: &str) -> Result<(), AnnotationError> {
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let stored: Option<String> = tx
            .query_row("SELECT value FROM meta WHERE key = ?1", [META_UUID], |r| {
                r.get(0)
            })
            .optional()?;
        match stored {
            None => {
                tx.execute(
                    "INSERT INTO meta (key, value) VALUES (?1, ?2)",
                    params![META_UUID, uuid],
                )?;
            }
            Some(ref u) if u == uuid => {}
            Some(u) => {
                return Err(AnnotationError::WrongAccount {
                    stored: u,
                    requested: uuid.into(),
                });
            }
        }
        tx.commit()?;
        self.uuid = Some(uuid.into());
        Ok(())
    }

    /// The owning account's uuid, when the file (or this handle) carries
    /// one. `None` means the pairing is uncheckable — a file opened from
    /// a raw path and never bound — and consumers that pair intent with
    /// facts refuse such handles.
    pub fn uuid(&self) -> Option<&str> {
        self.uuid.as_deref()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Write one annotation of kind `K` under compare-and-swap (C35),
    /// typed at the door (C66) and stamped with who wrote it (C65). The
    /// value is checked whole before the transaction opens — the version
    /// gate, `K`'s strict parse, the exact round-trip for a current-schema
    /// value — and a value that fails never lands
    /// ([`AnnotationError::Invalid`]); what lands is `value` as given,
    /// never a re-serialization, so an older stamp stays stored as written.
    ///
    /// `expected_revision` is what the caller last read: `None` creates
    /// (fails if the annotation currently exists), `Some(r)` updates (fails
    /// unless the current revision is exactly `r`). Returns the row as
    /// written; a mismatch is [`AnnotationError::Conflict`] carrying the
    /// current row. Creating over a tombstone continues its revision
    /// sequence — revisions are monotonic for the life of the file, never
    /// reset by delete/recreate — so a writer holding any revision of a
    /// live row conflicts once that row moves, and a `clear` then `set`
    /// on one target works without the caller knowing the tombstone is
    /// there. The one thing a caller cannot say is *which* tombstone it
    /// read past: `None` creates over a tombstone of any generation, so
    /// two writers who both read "nothing there" after a clear both land,
    /// in sequence (2b constraint (3), reduced; the generation belongs to
    /// the plan/receipt work, C71/C78).
    ///
    /// Between the parse and the compare-and-swap the kind's
    /// [`IntentValue::check_write`] runs — the writer's rules a stored row
    /// is exempt from — and a refusal is [`ValueError::RefusedForWrite`]
    /// under [`AnnotationError::Invalid`], so no frontend can write past
    /// them: the rule is the door's, not each caller's discipline.
    pub fn put<K: IntentValue>(
        &mut self,
        scope: &str,
        key: &str,
        value: &Value,
        expected_revision: Option<i64>,
        provenance: &Provenance,
    ) -> Result<AnnotationRow, AnnotationError> {
        let r = self.put_unreported::<K>(scope, key, value, expected_revision, provenance);
        reported(&self.conn, r)
    }

    /// The work of the public door above it, before its busy refusal is
    /// reported ([`reported`]).
    fn put_unreported<K: IntentValue>(
        &mut self,
        scope: &str,
        key: &str,
        value: &Value,
        expected_revision: Option<i64>,
        provenance: &Provenance,
    ) -> Result<AnnotationRow, AnnotationError> {
        provenance.check()?;
        check_value::<K>(value)?
            .check_write()
            .map_err(|detail| ValueError::RefusedForWrite {
                kind: K::KIND,
                detail,
            })?;
        let kind = K::KIND;
        let now = crate::now();
        // BEGIN IMMEDIATE: the write lock is taken up front, so two
        // connections racing the same row serialize here (bounded by the
        // busy timeout) and the loser reads the winner's committed state —
        // a real Conflict, not a snapshot-upgrade error at commit.
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        // C151: the row as stored, its value unread — a write naming the
        // revision replaces what it cannot parse.
        let current = stored(&tx, scope, key, kind)?;
        let stamped = |row: AnnotationRow| AnnotationRow {
            value: value.clone(),
            updated_at: now,
            written_via: provenance.written_via.clone(),
            actor: provenance.actor.clone(),
            ..row
        };
        let row = match (expected_revision, current) {
            (None, None) => stamped(AnnotationRow {
                scope: scope.into(),
                key: key.into(),
                kind: kind.into(),
                value: Value::Null,
                revision: 1,
                created_at: now,
                updated_at: now,
                written_via: String::new(),
                actor: None,
            }),
            // Recreation after a delete: the tombstone's revision carries on.
            (None, Some(tombstone)) if tombstone.deleted => {
                let revision = next_revision(&tombstone)?;
                stamped(AnnotationRow {
                    revision,
                    created_at: now,
                    ..tombstone.with_value(Value::Null)
                })
            }
            (Some(expected), Some(current)) if !current.deleted && current.revision == expected => {
                let revision = next_revision(&current)?;
                stamped(AnnotationRow {
                    revision,
                    ..current.with_value(Value::Null)
                })
            }
            (_, current) => return Err(conflict(current)),
        };
        tx.execute(
            "INSERT OR REPLACE INTO annotations
                (scope, key, kind, value, revision, created_at, updated_at, deleted_at, written_via, actor)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL, ?8, ?9)",
            params![
                row.scope,
                row.key,
                row.kind,
                row.value.to_string(),
                row.revision,
                row.created_at,
                row.updated_at,
                row.written_via,
                row.actor
            ],
        )?;
        tx.commit()?;
        Ok(row)
    }

    /// Remove one annotation, under the same compare-and-swap as `put`.
    /// This is a frontend expressing intent — the fact side has no delete
    /// path at all. The row becomes a tombstone (invisible to `get`/`list`)
    /// whose revision keeps counting, so no later writer can slip a stale
    /// value past the delete; the tombstone records who cleared it (C65).
    /// Returns the tombstone's revision, the one this delete's own
    /// transaction wrote: the generation the row sequence has reached,
    /// which a later conflict names — never a read taken after it, which
    /// another writer could have moved (round 5 of the 2g build). The next
    /// write is a create and names no revision ([`Self::put`]).
    pub fn delete(
        &mut self,
        scope: &str,
        key: &str,
        kind: &str,
        expected_revision: i64,
        provenance: &Provenance,
    ) -> Result<i64, AnnotationError> {
        let r = self.delete_unreported(scope, key, kind, expected_revision, provenance);
        reported(&self.conn, r)
    }

    /// The work of the public door above it, before its busy refusal is
    /// reported ([`reported`]).
    fn delete_unreported(
        &mut self,
        scope: &str,
        key: &str,
        kind: &str,
        expected_revision: i64,
        provenance: &Provenance,
    ) -> Result<i64, AnnotationError> {
        provenance.check()?;
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        // C151: read as stored; a delete naming the revision clears a row
        // it cannot parse.
        let tombstone = match stored(&tx, scope, key, kind)? {
            Some(current) if !current.deleted && current.revision == expected_revision => {
                let tombstone = next_revision(&current)?;
                tx.execute(
                    "UPDATE annotations
                        SET deleted_at = ?4, updated_at = ?4, revision = ?7,
                            written_via = ?5, actor = ?6
                      WHERE scope = ?1 AND key = ?2 AND kind = ?3",
                    params![
                        scope,
                        key,
                        kind,
                        crate::now(),
                        provenance.written_via,
                        provenance.actor,
                        tombstone
                    ],
                )?;
                tombstone
            }
            current => return Err(conflict(current)),
        };
        tx.commit()?;
        Ok(tombstone)
    }

    /// The stored row, raw: the value as written, with its provenance. A
    /// live row this build cannot read is [`AnnotationError::Unreadable`]
    /// (C151), never "no such annotation".
    pub fn get(
        &self,
        scope: &str,
        key: &str,
        kind: &str,
    ) -> Result<Option<AnnotationRow>, AnnotationError> {
        match stored(&self.conn, scope, key, kind)? {
            Some(row) if !row.deleted => Ok(Some(row.parsed()?)),
            _ => Ok(None),
        }
    }

    /// The live row a write naming `revision` replaces: the row read now,
    /// which must stand at exactly that revision, else the conflict the
    /// write's own compare-and-swap would raise, carrying the row that
    /// stands (none over a tombstone or no row). Revisions only grow and
    /// the swap holds the row at `revision` until the write lands, so a
    /// write naming it that lands replaced the very row returned: the one
    /// read a receipt's `prior` is taken from, for a price and a policy
    /// alike (C35, C78; round 5 of the 2g build).
    pub fn at_revision(
        &self,
        scope: &str,
        key: &str,
        kind: &str,
        revision: i64,
    ) -> Result<AnnotationRow, AnnotationError> {
        match stored(&self.conn, scope, key, kind)? {
            Some(row) if !row.deleted && row.revision == revision => Ok(row.parsed()?),
            current => Err(conflict(current)),
        }
    }

    /// The revision a clear left at this key — its tombstone's, which the
    /// next write's revision continues from — or `None` while a row
    /// stands or none was ever written. A refusal reads it to say a row
    /// was cleared, and at which revision, rather than never written.
    pub fn cleared(
        &self,
        scope: &str,
        key: &str,
        kind: &str,
    ) -> Result<Option<i64>, AnnotationError> {
        Ok(stored(&self.conn, scope, key, kind)?
            .and_then(|row| row.deleted.then_some(row.revision)))
    }

    /// The stored row of kind `K` and its typed value (C66): the same
    /// generic as the write door, so a row a newer build wrote is reported
    /// as such and an older stamp upgrades in memory. A stored row that
    /// fails is an error, never "no such annotation".
    pub fn get_as<K: IntentValue>(
        &self,
        scope: &str,
        key: &str,
    ) -> Result<Option<(AnnotationRow, K)>, AnnotationError> {
        self.get(scope, key, K::KIND)?
            .map(|row| {
                check_value::<K>(&row.value)
                    .map(|v| (row, v))
                    .map_err(Into::into)
            })
            .transpose()
    }

    /// Every live annotation (tombstones excluded), optionally restricted
    /// to one scope and/or one kind, ordered by (scope, key, kind): the
    /// readable rows, and beside them each row this build cannot read,
    /// named (C151), so one damaged row never fails the list. A pricing
    /// read wants one kind across four scopes; at 10k rows the scan is
    /// 35 ms, so a filter is needed and an index is not.
    pub fn list(
        &self,
        scope: Option<&str>,
        kind: Option<&str>,
    ) -> Result<AnnotationList, AnnotationError> {
        let mut stmt = self.conn.prepare(
            "SELECT scope, key, kind, value, revision, created_at, updated_at, written_via, actor
               FROM annotations
              WHERE deleted_at IS NULL AND (?1 IS NULL OR scope = ?1) AND (?2 IS NULL OR kind = ?2)
              ORDER BY scope, key, kind",
        )?;
        let rows: Vec<Stored> = stmt
            .query_map(params![scope, kind], |r| {
                Ok(Stored {
                    scope: r.get(0)?,
                    key: r.get(1)?,
                    kind: r.get(2)?,
                    value: r.get(3)?,
                    revision: r.get(4)?,
                    created_at: r.get(5)?,
                    updated_at: r.get(6)?,
                    written_via: r.get(7)?,
                    actor: r.get(8)?,
                    deleted: false,
                })
            })?
            .collect::<Result<_, _>>()?;
        let mut list = AnnotationList::default();
        for row in rows {
            match row.parsed() {
                Ok(row) => list.rows.push(row),
                Err(unreadable) => list.unreadable.push(unreadable),
            }
        }
        Ok(list)
    }

    /// The file's revision: the sum of every row's revision, tombstones
    /// included, which every write through this API raises by exactly one
    /// — a create lands at 1, an update, a delete and a create over a
    /// tombstone each add one — so two reads that agree bracket a state no
    /// write has touched. What a consumer that joins intent to facts names
    /// in its basis (C98; the search's `corpus.rs`), read in one statement
    /// so it is one moment's; never a row's own revision, which the
    /// compare-and-swap uses.
    pub fn revision(&self) -> Result<i64, AnnotationError> {
        Ok(self.conn.query_row(
            "SELECT COALESCE(SUM(revision), 0) FROM annotations",
            [],
            |r| r.get(0),
        )?)
    }

    /// Store-managed backup (C35): a consistent snapshot of this file at
    /// `dest`, via SQLite's `VACUUM INTO`, published atomically. `VACUUM
    /// INTO` writes `dest` directly and never fsyncs it, so an interrupted
    /// export would leave a partial file that both looks like a backup and
    /// blocks every retry. The copy is therefore written to a partial
    /// named for this invocation (`<dest>.partial-<pid>-<nanos>`, so two
    /// exporters never share or remove each other's), checked with
    /// `quick_check`, fsynced, then published by `hard_link` — which the
    /// kernel refuses if `dest` appeared meanwhile, so two exports to one
    /// destination end with one backup and one `AlreadyExists`, never a
    /// replaced backup. Only on a filesystem without hard links does a
    /// `rename` stand in, and that fallback is the one place a
    /// simultaneous export could still replace another's. The publish
    /// ends with the directory synced (the crate's one `sync_dir`, issue #63), so
    /// the link is durable as the file is. Fails if `dest`
    /// already exists, before anything is written; the partial is removed
    /// either way.
    pub fn export(&self, dest: &Path) -> Result<(), AnnotationError> {
        if let Some(dir) = dest.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let exists = || {
            std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!(
                    "{} exists; a backup never overwrites another backup",
                    dest.display()
                ),
            )
        };
        if dest.exists() {
            return Err(exists().into());
        }
        // The partial's name is unique per process and per export within
        // it: the clock alone is not — two threads exporting within one
        // tick (microseconds on macOS) shared a name, one VACUUM failed
        // on the other's file and its cleanup removed that file before
        // it was published, so both were refused (the C35 race test,
        // one run in twelve; found by plan step 7's gate, 2026-09-07).
        static EXPORTS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let mut partial = dest.as_os_str().to_owned();
        partial.push(format!(
            ".partial-{}-{}-{}",
            std::process::id(),
            EXPORTS.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        let partial = PathBuf::from(partial);
        let published = self.export_to(&partial).and_then(|()| {
            match std::fs::hard_link(&partial, dest) {
                Ok(()) => Ok(()),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Err(exists()),
                // No hard links here (FAT and the like): a rename is the
                // best available, guarded by a last look.
                Err(e)
                    if matches!(
                        e.kind(),
                        std::io::ErrorKind::Unsupported | std::io::ErrorKind::PermissionDenied
                    ) =>
                {
                    if dest.exists() {
                        Err(exists())
                    } else {
                        std::fs::rename(&partial, dest)
                    }
                }
                Err(e) => Err(e),
            }
            .and_then(|()| crate::sync_dir(dest.parent()))
            .map_err(AnnotationError::Io)
        });
        let _ = std::fs::remove_file(&partial);
        published
    }

    /// The copy itself: vacuumed into `partial`, integrity-checked through
    /// a fresh connection, and fsynced.
    fn export_to(&self, partial: &Path) -> Result<(), AnnotationError> {
        self.conn
            .execute("VACUUM INTO ?1", [partial.to_string_lossy()])?;
        let verdict: String =
            Connection::open(partial)?.query_row("PRAGMA quick_check", [], |r| r.get(0))?;
        if verdict != "ok" {
            return Err(std::io::Error::other(format!(
                "backup {} failed its integrity check: {verdict}",
                partial.display()
            ))
            .into());
        }
        // Opened for writing: Windows refuses to flush a read-only handle.
        std::fs::OpenOptions::new()
            .write(true)
            .open(partial)?
            .sync_all()?;
        Ok(())
    }
}

/// A conflict, exposing only a live row: a tombstone reads as "no such
/// annotation", but its revision still gates the next write.
/// The revision a write over `row` takes: one past it, computed once
/// before anything is written and bound into the write, the same number
/// the write returns — or, at the last revision an `i64` holds, the
/// refusal that says so with nothing written ([`AnnotationError::RevisionExhausted`]).
/// The one increment both doors take, `put` and `delete`.
fn next_revision(row: &Stored) -> Result<i64, AnnotationError> {
    row.revision
        .checked_add(1)
        .ok_or_else(|| AnnotationError::RevisionExhausted {
            scope: row.scope.clone(),
            key: row.key.clone(),
            kind: row.kind.clone(),
            revision: row.revision,
        })
}

/// What a write that did not match answers: a `Conflict` carrying the
/// live row as it stands, none for a tombstone or no row at all — or,
/// for a live row this build cannot read, `Unreadable` with the revision
/// a write must name (C151).
fn conflict(current: Option<Stored>) -> AnnotationError {
    match current {
        Some(row) if !row.deleted => match row.parsed() {
            Ok(row) => AnnotationError::Conflict {
                current: Some(Box::new(row)),
            },
            Err(unreadable) => AnnotationError::Unreadable(unreadable),
        },
        _ => AnnotationError::Conflict { current: None },
    }
}

/// The stored row, tombstones included, its value unread (C151).
fn stored(
    conn: &Connection,
    scope: &str,
    key: &str,
    kind: &str,
) -> Result<Option<Stored>, AnnotationError> {
    // value, revision, created, updated, deleted, written_via, actor
    type Columns = (String, i64, i64, i64, Option<i64>, String, Option<String>);
    let found: Option<Columns> = conn
        .query_row(
            "SELECT value, revision, created_at, updated_at, deleted_at, written_via, actor
               FROM annotations
              WHERE scope = ?1 AND key = ?2 AND kind = ?3",
            params![scope, key, kind],
            |r| {
                Ok((
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                    r.get(6)?,
                ))
            },
        )
        .optional()?;
    Ok(found.map(
        |(value, revision, created_at, updated_at, deleted_at, written_via, actor)| Stored {
            scope: scope.into(),
            key: key.into(),
            kind: kind.into(),
            value,
            revision,
            created_at,
            updated_at,
            written_via,
            actor,
            deleted: deleted_at.is_some(),
        },
    ))
}

/// Kinds for this crate's tests: a value of any shape under a named kind,
/// stamped `version: 1`. The store crate owns no real kind (each kind's
/// shape is its owner's, C66), so its tests declare these.
#[cfg(test)]
pub(crate) mod test_kinds {
    use super::IntentValue;
    use serde::Serialize;
    use serde_json::Value;

    macro_rules! loose_kind {
        ($name:ident, $kind:literal) => {
            #[derive(Debug, Clone, PartialEq, Serialize)]
            #[serde(transparent)]
            pub(crate) struct $name(pub Value);

            impl IntentValue for $name {
                const KIND: &'static str = $kind;
                const VERSION: i64 = 1;
                fn parse(value: &Value) -> Result<Self, String> {
                    Ok($name(value.clone()))
                }
            }
        };
    }

    loose_kind!(Buyout, "buyout");
    loose_kind!(Note, "note");
    loose_kind!(Policy, "sync-policy");

    /// A `note` that reads any shape but refuses, as a new write, a value
    /// carrying `"retired": true` — the shape of a writer's rule.
    #[derive(Debug, Clone, PartialEq, Serialize)]
    #[serde(transparent)]
    pub(crate) struct GuardedNote(pub Value);

    impl IntentValue for GuardedNote {
        const KIND: &'static str = "note";
        const VERSION: i64 = 1;
        fn parse(value: &Value) -> Result<Self, String> {
            Ok(GuardedNote(value.clone()))
        }
        fn check_write(&self) -> Result<(), String> {
            match self.0.get("retired") {
                Some(Value::Bool(true)) => Err("names a retired thing".into()),
                _ => Ok(()),
            }
        }
    }

    /// The provenance every test write carries.
    pub(crate) fn via_test() -> super::Provenance {
        super::Provenance::via("test")
    }
}

#[cfg(test)]
mod tests {
    use super::test_kinds::{Buyout, GuardedNote, Note, Policy, via_test};
    use super::*;
    use serde_json::json;

    /// C66 — a kind's writer's rule runs at the door on every write and
    /// never on a read: a value it refuses as new never lands, the same
    /// value already stored (written before the rule, or by a kind
    /// without it) still reads through `get_as`, and is replaced or
    /// deleted like any other row.
    #[test]
    fn c66_a_writer_s_rule_refuses_a_new_write_and_never_a_read() {
        let mut a = Annotations::open_memory().unwrap();
        let retired = json!({ "version": 1, "retired": true });
        let err = a
            .put::<GuardedNote>("item", "i1", &retired, None, &via_test())
            .unwrap_err();
        match err {
            AnnotationError::Invalid(ValueError::RefusedForWrite { kind, detail }) => {
                assert_eq!((kind, detail.as_str()), ("note", "names a retired thing"))
            }
            other => panic!("expected RefusedForWrite, got {other}"),
        }
        assert!(a.get("item", "i1", "note").unwrap().is_none());
        // Stored by a kind without the rule: the guarded kind reads it.
        a.put::<Note>("item", "i1", &retired, None, &via_test())
            .unwrap();
        let (row, read) = a.get_as::<GuardedNote>("item", "i1").unwrap().unwrap();
        assert_eq!((row.revision, read.0), (1, retired));
        let row = a
            .put::<GuardedNote>(
                "item",
                "i1",
                &json!({ "version": 1, "retired": false }),
                Some(1),
                &via_test(),
            )
            .unwrap();
        assert_eq!(row.revision, 2);
    }

    /// The file's revision moves by one on every write — create, update,
    /// delete, create over the tombstone — and by nothing else.
    #[test]
    fn the_files_revision_rises_by_one_per_write_tombstones_included() {
        let mut a = Annotations::open_memory().unwrap();
        assert_eq!(a.revision().unwrap(), 0);
        a.put::<Buyout>("item", "i1", &price("1"), None, &via_test())
            .unwrap();
        assert_eq!(a.revision().unwrap(), 1);
        a.put::<Buyout>("item", "i2", &price("1"), None, &via_test())
            .unwrap();
        assert_eq!(a.revision().unwrap(), 2);
        a.put::<Buyout>("item", "i1", &price("2"), Some(1), &via_test())
            .unwrap();
        assert_eq!(a.revision().unwrap(), 3);
        a.delete("item", "i1", "buyout", 2, &via_test()).unwrap();
        assert_eq!(a.revision().unwrap(), 4);
        // the tombstone still counts; a create over it carries on from it
        a.put::<Buyout>("item", "i1", &price("3"), None, &via_test())
            .unwrap();
        assert_eq!(a.revision().unwrap(), 5);
        // a read moves nothing
        a.list(None, None).unwrap();
        a.get("item", "i1", "buyout").unwrap();
        assert_eq!(a.revision().unwrap(), 5);
    }

    fn conflict_revision(e: AnnotationError) -> Option<i64> {
        match e {
            AnnotationError::Conflict { current } => current.map(|r| r.revision),
            other => panic!("expected a conflict, got {other}"),
        }
    }

    fn price(p: &str) -> Value {
        json!({ "version": 1, "price": p })
    }

    /// C35 — integer-revision compare-and-swap: create requires absence, update the exact revision, delete the same.
    #[test]
    fn compare_and_swap_creates_updates_and_conflicts() {
        let mut a = Annotations::open_memory().unwrap();
        let via = via_test();
        // Create requires "does not exist yet".
        let row = a
            .put::<Buyout>("item", "i1", &price("1 divine"), None, &via)
            .unwrap();
        assert_eq!(row.revision, 1);
        let err = a
            .put::<Buyout>("item", "i1", &price("2 divine"), None, &via)
            .unwrap_err();
        assert_eq!(conflict_revision(err), Some(1));
        // Update requires the exact current revision.
        let row = a
            .put::<Buyout>("item", "i1", &price("2 divine"), Some(1), &via)
            .unwrap();
        assert_eq!(row.revision, 2);
        assert!(row.created_at <= row.updated_at);
        let stale = a
            .put::<Buyout>("item", "i1", &price("3c"), Some(1), &via)
            .unwrap_err();
        assert_eq!(conflict_revision(stale), Some(2));
        // The stored value is the last accepted write.
        let got = a.get("item", "i1", "buyout").unwrap().unwrap();
        assert_eq!(got.value, price("2 divine"));
        // The same key under another kind is its own row.
        a.put::<Note>(
            "item",
            "i1",
            &json!({ "version": 1, "text": "keep" }),
            None,
            &via,
        )
        .unwrap();
        assert_eq!(a.list(Some("item"), None).unwrap().rows.len(), 2);
        // Delete is CAS too; a stale delete conflicts and changes nothing.
        let err = a.delete("item", "i1", "buyout", 1, &via).unwrap_err();
        assert_eq!(conflict_revision(err), Some(2));
        a.delete("item", "i1", "buyout", 2, &via).unwrap();
        assert!(a.get("item", "i1", "buyout").unwrap().is_none());
        // Updating what is gone reports "gone", not "wrong revision".
        let err = a
            .put::<Buyout>("item", "i1", &price("x"), Some(2), &via)
            .unwrap_err();
        assert_eq!(conflict_revision(err), None);
    }

    /// C35 — delete tombstones; the revision sequence survives delete/recreate, so a writer holding any revision of the live row conflicts once it moves (a create over the tombstone lands, whatever its generation — see `put`).
    /// A live row's value overwritten with text that is not JSON, as a
    /// damaged file holds it: the fixture every `c151_` test stages.
    fn damaged(a: &Annotations, key: &str) {
        a.conn
            .execute(
                "UPDATE annotations SET value = '{nope' WHERE key = ?1",
                [key],
            )
            .unwrap();
    }

    /// C151 — an annotation row this build cannot read is named, never
    /// dropped, never a wall: `list` answers the readable rows and names
    /// the damaged one by its coordinate, revision and fault (the
    /// parser's words, which never quote the value, C127), where it used
    /// to fail whole; `get` answers the same facts as a structured
    /// refusal, in words that say what replaces the row; the file's
    /// revision still sums every row; a tombstone is never the row.
    #[test]
    fn c151_list_names_an_unreadable_row_beside_the_readable_ones_and_get_refuses_it() {
        let mut a = Annotations::open_memory().unwrap();
        let via = via_test();
        a.put::<Buyout>("item", "i-good", &price("1c"), None, &via)
            .unwrap();
        let bad = a
            .put::<Buyout>("item", "i-bad", &price("2c"), None, &via)
            .unwrap();
        a.put::<Buyout>("item", "i-bad", &price("3c"), Some(bad.revision), &via)
            .unwrap();
        damaged(&a, "i-bad");

        let list = a.list(None, None).unwrap();
        let readable: Vec<&str> = list.rows.iter().map(|r| r.key.as_str()).collect();
        assert_eq!(readable, ["i-good"], "the readable rows, whole");
        let [named] = list.unreadable.as_slice() else {
            panic!("one unreadable row named: {:?}", list.unreadable)
        };
        assert_eq!(
            (
                named.scope.as_str(),
                named.key.as_str(),
                named.kind.as_str(),
                named.revision
            ),
            ("item", "i-bad", "buyout", 2)
        );
        assert!(
            !named.fault.is_empty() && !named.fault.contains("nope"),
            "the fault is the parser's and never quotes the value: {}",
            named.fault
        );
        assert_eq!(
            a.revision().unwrap(),
            1 + 2,
            "the file's revision sums every row"
        );

        let err = a.get("item", "i-bad", "buyout").unwrap_err();
        assert!(
            matches!(&err, AnnotationError::Unreadable(row) if row.revision == 2 && row.key == "i-bad"),
            "{err:?}"
        );
        let words = err.to_string();
        assert!(
            words.contains("item/i-bad/buyout")
                && words.contains("revision 2")
                && words.contains("naming that revision replaces it"),
            "{words}"
        );
        assert_eq!(a.cleared("item", "i-bad", "buyout").unwrap(), None);
        assert_eq!(
            a.get("item", "i-good", "buyout").unwrap().unwrap().value,
            price("1c")
        );
    }

    /// C151 — a write naming the row's revision does not parse its value:
    /// `put` at the revision replaces an unreadable row and `delete` at
    /// the revision clears it, through the one compare-and-swap (C35);
    /// at a wrong revision, or a create over the live row, each answers
    /// `Unreadable` with the revision the row stands at, never a
    /// `Conflict` that would need the row it cannot read; a create over
    /// the tombstone of a damaged row continues the sequence without
    /// reading it. Nothing repairs a value in place.
    #[test]
    fn c151_a_write_naming_the_revision_replaces_or_clears_an_unreadable_row() {
        let mut a = Annotations::open_memory().unwrap();
        let via = via_test();
        let first = a
            .put::<Buyout>("item", "i-bad", &price("2c"), None, &via)
            .unwrap();
        a.put::<Buyout>("item", "i-bad", &price("3c"), Some(first.revision), &via)
            .unwrap();
        damaged(&a, "i-bad");
        let unreadable_at = |err: AnnotationError| match err {
            AnnotationError::Unreadable(row) => row.revision,
            other => panic!("expected the row named as unreadable, got {other:?}"),
        };

        // The wrong revision, and a create over the live row: named.
        let stale = a
            .put::<Buyout>("item", "i-bad", &price("4c"), Some(1), &via)
            .unwrap_err();
        assert_eq!(unreadable_at(stale), 2);
        let blind = a
            .put::<Buyout>("item", "i-bad", &price("4c"), None, &via)
            .unwrap_err();
        assert_eq!(unreadable_at(blind), 2);
        let stale_clear = a.delete("item", "i-bad", "buyout", 1, &via).unwrap_err();
        assert_eq!(unreadable_at(stale_clear), 2);
        assert!(
            matches!(
                a.get("item", "i-bad", "buyout"),
                Err(AnnotationError::Unreadable(_))
            ),
            "nothing was written"
        );

        // The right revision: replaced, readable again, the sequence kept.
        let replaced = a
            .put::<Buyout>("item", "i-bad", &price("4c"), Some(2), &via)
            .unwrap();
        assert_eq!((replaced.revision, &replaced.value), (3, &price("4c")));
        assert_eq!(
            a.get("item", "i-bad", "buyout").unwrap().unwrap().value,
            price("4c")
        );
        assert!(a.list(None, None).unwrap().unreadable.is_empty());

        // Damaged again, cleared at its revision; the tombstone's value is
        // still unreadable and a create over it never reads it.
        damaged(&a, "i-bad");
        let tombstone = a.delete("item", "i-bad", "buyout", 3, &via).unwrap();
        assert_eq!(tombstone, 4);
        let list = a.list(None, None).unwrap();
        assert!(
            list.rows.is_empty() && list.unreadable.is_empty(),
            "{list:?}"
        );
        assert_eq!(a.cleared("item", "i-bad", "buyout").unwrap(), Some(4));
        let again = a
            .put::<Buyout>("item", "i-bad", &price("5c"), None, &via)
            .unwrap();
        assert_eq!(
            again.revision, 5,
            "the sequence continues past the damaged tombstone"
        );
    }

    #[test]
    fn delete_and_recreate_never_reset_the_revision() {
        let mut a = Annotations::open_memory().unwrap();
        let via = via_test();
        // A stale reader remembers revision 1...
        let stale = a
            .put::<Buyout>("item", "i1", &price("1c"), None, &via)
            .unwrap()
            .revision;
        // ...while someone else deletes and recreates the annotation.
        a.delete("item", "i1", "buyout", 1, &via).unwrap();
        let recreated = a
            .put::<Buyout>("item", "i1", &price("5 divine"), None, &via)
            .unwrap();
        assert!(
            recreated.revision > stale,
            "recreation continues the sequence ({} > {stale})",
            recreated.revision
        );
        // The stale writer's update conflicts instead of silently landing
        // its pre-delete value over the recreated one (the ABA hole).
        let err = a
            .put::<Buyout>("item", "i1", &price("1c"), Some(stale), &via)
            .unwrap_err();
        assert_eq!(conflict_revision(err), Some(recreated.revision));
        assert_eq!(
            a.get("item", "i1", "buyout").unwrap().unwrap().value,
            price("5 divine")
        );
        // A stale delete cannot remove the recreated value either.
        assert!(a.delete("item", "i1", "buyout", stale, &via).is_err());
    }

    /// C35, C78 — a delete returns the revision its own transaction gave
    /// the tombstone, the generation a later conflict names and the next
    /// create continues past; and the row a write names is read at exactly
    /// that revision or refused as the write would be, carrying the live
    /// row (none over a tombstone), so a receipt's prior is the row
    /// replaced and never one a racing writer moved (round 5 of the 2g
    /// build).
    #[test]
    fn a_delete_returns_its_tombstones_revision_and_a_write_reads_only_the_row_it_names() {
        let mut a = Annotations::open_memory().unwrap();
        let via = via_test();
        a.put::<Buyout>("item", "i1", &price("1c"), None, &via)
            .unwrap();
        a.put::<Buyout>("item", "i1", &price("2c"), Some(1), &via)
            .unwrap();
        let at = a.at_revision("item", "i1", "buyout", 2).unwrap();
        assert_eq!((at.revision, &at.value), (2, &price("2c")));
        let err = a.at_revision("item", "i1", "buyout", 1).unwrap_err();
        assert_eq!(
            conflict_revision(err),
            Some(2),
            "a moved row is the conflict"
        );
        let tombstone = a.delete("item", "i1", "buyout", 2, &via).unwrap();
        assert_eq!(tombstone, 3);
        assert_eq!(a.cleared("item", "i1", "buyout").unwrap(), Some(tombstone));
        let err = a.at_revision("item", "i1", "buyout", 3).unwrap_err();
        assert_eq!(
            conflict_revision(err),
            None,
            "a tombstone is no row to replace"
        );
        let err = a.at_revision("item", "i9", "buyout", 1).unwrap_err();
        assert_eq!(conflict_revision(err), None, "nor is a row never written");
        let recreated = a
            .put::<Buyout>("item", "i1", &price("3c"), None, &via)
            .unwrap();
        assert_eq!(recreated.revision, tombstone + 1);
    }

    /// C47 (a stored value at the bound is a structured refusal) — a row
    /// at the last revision an `i64` holds takes no next one: a replace, a
    /// delete and a create over such a tombstone each refuse as
    /// `RevisionExhausted`, and nothing is written; one below it moves to
    /// the bound, the number written the number returned (round 5 of the
    /// 2g build, its review's finding 3).
    #[test]
    fn a_row_at_the_last_revision_refuses_every_write_and_changes_nothing() {
        let mut a = Annotations::open_memory().unwrap();
        let via = via_test();
        a.put::<Buyout>("item", "i1", &price("1c"), None, &via)
            .unwrap();
        let at = |a: &Annotations, revision: i64| {
            a.conn
                .execute(
                    "UPDATE annotations SET revision = ?1 WHERE key = 'i1'",
                    [revision],
                )
                .unwrap();
        };
        let exhausted = |e: AnnotationError| {
            assert!(
                matches!(
                    e,
                    AnnotationError::RevisionExhausted {
                        revision: i64::MAX,
                        ..
                    }
                ),
                "{e}"
            );
        };
        at(&a, i64::MAX - 1);
        let row = a
            .put::<Buyout>("item", "i1", &price("2c"), Some(i64::MAX - 1), &via)
            .unwrap();
        assert_eq!(row.revision, i64::MAX);
        assert_eq!(a.get("item", "i1", "buyout").unwrap(), Some(row.clone()));
        exhausted(
            a.put::<Buyout>("item", "i1", &price("3c"), Some(i64::MAX), &via)
                .unwrap_err(),
        );
        exhausted(
            a.delete("item", "i1", "buyout", i64::MAX, &via)
                .unwrap_err(),
        );
        assert_eq!(
            a.get("item", "i1", "buyout").unwrap(),
            Some(row),
            "untouched"
        );
        at(&a, i64::MAX - 1);
        assert_eq!(
            a.delete("item", "i1", "buyout", i64::MAX - 1, &via)
                .unwrap(),
            i64::MAX
        );
        assert_eq!(a.cleared("item", "i1", "buyout").unwrap(), Some(i64::MAX));
        exhausted(
            a.put::<Buyout>("item", "i1", &price("4c"), None, &via)
                .unwrap_err(),
        );
        assert_eq!(a.cleared("item", "i1", "buyout").unwrap(), Some(i64::MAX));
    }

    /// C35 — two writers on one file serialize under BEGIN IMMEDIATE: one wins, the other gets a Conflict, never a clobber.
    #[test]
    fn two_connections_conflict_deterministically_never_clobber() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        let path = annotations_path(&dir, "u-race");
        let mut a = Annotations::open(&path).unwrap();
        let mut b = Annotations::open(&path).unwrap();
        let via = via_test();
        // Both connections race the same create. BEGIN IMMEDIATE
        // serializes them, so exactly one wins and the loser gets the
        // documented Conflict carrying the winner's row — never a raw
        // SQLite busy/snapshot error, never two revision-1 writes.
        let results = std::thread::scope(|scope| {
            let ta = scope.spawn(|| {
                a.put::<Buyout>(
                    "item",
                    "i1",
                    &json!({"version": 1, "from": "a"}),
                    None,
                    &via,
                )
            });
            let tb = scope.spawn(|| {
                b.put::<Buyout>(
                    "item",
                    "i1",
                    &json!({"version": 1, "from": "b"}),
                    None,
                    &via,
                )
            });
            [ta.join().unwrap(), tb.join().unwrap()]
        });
        let (ok, err): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);
        assert_eq!((ok.len(), err.len()), (1, 1), "one winner, one conflict");
        let winner = ok.into_iter().next().unwrap().unwrap();
        assert_eq!(winner.revision, 1);
        match err.into_iter().next().unwrap().unwrap_err() {
            AnnotationError::Conflict { current: Some(row) } => assert_eq!(*row, winner),
            other => panic!("the loser must see a Conflict, got {other}"),
        }
    }

    /// A writer that cannot get the lock inside the busy timeout is told
    /// to retry later — its own kind, not a bare SQLite error — because
    /// the remedy differs from a Conflict's "re-read and retry".
    #[test]
    fn a_writer_held_past_the_busy_timeout_is_busy_not_db() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        let path = annotations_path(&dir, "u-busy");
        let mut a = Annotations::open(&path).unwrap();
        let b = Annotations::open(&path).unwrap();
        a.conn
            .busy_timeout(std::time::Duration::from_millis(50))
            .unwrap();
        // `b` holds the write lock in an open transaction the whole time.
        b.conn.execute_batch("BEGIN IMMEDIATE").unwrap();
        let err = a
            .put::<Buyout>("item", "i1", &price("1c"), None, &via_test())
            .unwrap_err();
        assert!(matches!(err, AnnotationError::Busy { .. }), "{err}");
        assert!(err.to_string().contains("retry later"), "{err}");
        b.conn.execute_batch("ROLLBACK").unwrap();
        // The row is untouched: the same create succeeds once the lock is free.
        assert_eq!(
            a.put::<Buyout>("item", "i1", &price("1c"), None, &via_test())
                .unwrap()
                .revision,
            1
        );
    }

    /// C137: the annotation wait at zero (`ACQ_ANNOTATION_WAIT=0`) refuses
    /// a writer the file's lock is held from at once, as busy, naming
    /// zero seconds: the expiry forced, no real waiting, and no
    /// wall-clock bound asserted. The seconds named are the wait SQLite
    /// enforced, read back from the connection, so a knob unwired from
    /// the busy timeout leaves the default's five and fails here. The
    /// knob is the process's, so the test runs itself again as a child
    /// with it set, and no other test of this binary reads it changed.
    #[test]
    fn c137_the_annotation_wait_at_zero_refuses_a_held_writer_at_once() {
        if std::env::var("ACQ_ANNOTATION_WAIT").as_deref() != Ok("0") {
            let module = module_path!()
                .split_once("::")
                .map_or(module_path!(), |(_, rest)| rest);
            let out = std::process::Command::new(std::env::current_exe().unwrap())
                .args([
                    format!(
                        "{module}::c137_the_annotation_wait_at_zero_refuses_a_held_writer_at_once"
                    ),
                    "--exact".into(),
                    "--nocapture".into(),
                    "--test-threads=1".into(),
                ])
                .env("ACQ_ANNOTATION_WAIT", "0")
                .output()
                .unwrap();
            let said = String::from_utf8_lossy(&out.stdout);
            assert!(
                out.status.success() && said.contains("1 passed"),
                "the child with the wait at zero: {}\n{said}\n{}",
                out.status,
                String::from_utf8_lossy(&out.stderr)
            );
            return;
        }
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        let path = annotations_path(&dir, "u-wait");
        let mut a = Annotations::open(&path).unwrap();
        let b = Annotations::open(&path).unwrap();
        // `b` holds the write lock in an open transaction the whole time.
        b.conn.execute_batch("BEGIN IMMEDIATE").unwrap();
        let err = a
            .put::<Buyout>("item", "i1", &price("1c"), None, &via_test())
            .unwrap_err();
        assert!(
            matches!(
                err,
                AnnotationError::Busy {
                    waited: Some(std::time::Duration::ZERO)
                }
            ),
            "the wait the connection enforced, not the default: {err:?}"
        );
        assert_eq!(
            err.to_string(),
            "annotation file is busy: another writer held it past 0 s (retry later)"
        );
        b.conn.execute_batch("ROLLBACK").unwrap();
    }

    /// C137: the knob names only a wait SQLite can enforce. Over the
    /// bound (whose milliseconds overflow SQLite's signed 32-bit timeout),
    /// `u64::MAX` and garbage keep the default; zero stays zero, the
    /// forced expiry; the bound itself is honoured.
    #[test]
    fn c137_the_annotation_wait_keeps_the_default_for_a_value_sqlite_cannot_hold() {
        use std::time::Duration;
        assert_eq!(parse_annotation_wait("2147484"), BUSY_TIMEOUT);
        assert_eq!(parse_annotation_wait(&u64::MAX.to_string()), BUSY_TIMEOUT);
        assert_eq!(parse_annotation_wait("0"), Duration::ZERO);
        assert_eq!(parse_annotation_wait("five"), BUSY_TIMEOUT);
        assert_eq!(parse_annotation_wait("-1"), BUSY_TIMEOUT);
        assert_eq!(
            parse_annotation_wait(" 2147483 "),
            Duration::from_secs(MAX_WAIT_SECS)
        );
        // The bound is one SQLite holds: set and read back, unchanged.
        let a = Annotations::open_memory().unwrap();
        a.conn
            .busy_timeout(Duration::from_secs(MAX_WAIT_SECS))
            .unwrap();
        assert_eq!(
            enforced_wait(&a.conn),
            Some(Duration::from_secs(MAX_WAIT_SECS))
        );
    }

    #[test]
    fn account_singletons_use_the_empty_key() {
        let mut a = Annotations::open_memory().unwrap();
        let policy = json!({ "version": 1, "leagues": ["Standard"], "deep": false });
        a.put::<Policy>("account", "", &policy, None, &via_test())
            .unwrap();
        let row = a.get("account", "", "sync-policy").unwrap().unwrap();
        assert_eq!(row.value, policy);
    }

    /// `list` filters by scope, by kind, or both: a pricing read wants one
    /// kind across every scope.
    #[test]
    fn list_filters_by_scope_and_by_kind() {
        let mut a = Annotations::open_memory().unwrap();
        let via = via_test();
        a.put::<Buyout>("item", "i1", &price("1c"), None, &via)
            .unwrap();
        a.put::<Buyout>("tab", "pc/t1", &price("2c"), None, &via)
            .unwrap();
        a.put::<Note>(
            "item",
            "i1",
            &json!({ "version": 1, "text": "n" }),
            None,
            &via,
        )
        .unwrap();
        let keys = |rows: Vec<AnnotationRow>| -> Vec<String> {
            rows.into_iter()
                .map(|r| format!("{}/{}/{}", r.scope, r.key, r.kind))
                .collect()
        };
        assert_eq!(
            keys(a.list(None, None).unwrap().rows),
            ["item/i1/buyout", "item/i1/note", "tab/pc/t1/buyout"]
        );
        assert_eq!(
            keys(a.list(None, Some("buyout")).unwrap().rows),
            ["item/i1/buyout", "tab/pc/t1/buyout"]
        );
        assert_eq!(
            keys(a.list(Some("item"), Some("buyout")).unwrap().rows),
            ["item/i1/buyout"]
        );
        assert!(
            a.list(Some("account"), Some("buyout"))
                .unwrap()
                .rows
                .is_empty()
        );
    }

    /// C65 — every write carries who wrote it; a delete stamps the
    /// tombstone; an empty or whitespace channel is refused before
    /// anything lands.
    #[test]
    fn c65_every_write_carries_its_provenance() {
        let mut a = Annotations::open_memory().unwrap();
        let cli = Provenance::via("cli").as_actor("tom");
        let row = a
            .put::<Buyout>("item", "i1", &price("1c"), None, &cli)
            .unwrap();
        assert_eq!(
            (row.written_via.as_str(), row.actor.as_deref()),
            ("cli", Some("tom"))
        );
        let read = a.get("item", "i1", "buyout").unwrap().unwrap();
        assert_eq!(read, row);
        // The next writer's stamp replaces it, actor included.
        let mcp = Provenance::via("mcp");
        let row = a
            .put::<Buyout>("item", "i1", &price("2c"), Some(1), &mcp)
            .unwrap();
        assert_eq!((row.written_via.as_str(), row.actor), ("mcp", None));
        // The tombstone records who cleared the row.
        a.delete("item", "i1", "buyout", 2, &cli).unwrap();
        let tomb = stored(&a.conn, "item", "i1", "buyout").unwrap().unwrap();
        assert!(tomb.deleted);
        assert_eq!(tomb.written_via, "cli");
        assert_eq!(tomb.actor.as_deref(), Some("tom"));
        // Refused channels: nothing lands, nothing is deleted.
        for bad in ["", "two words"] {
            let err = a
                .put::<Buyout>("item", "i2", &price("1c"), None, &Provenance::via(bad))
                .unwrap_err();
            assert!(
                matches!(err, AnnotationError::Provenance { .. }),
                "{bad:?}: {err}"
            );
            assert!(a.get("item", "i2", "buyout").unwrap().is_none());
        }
        a.put::<Buyout>("item", "i2", &price("1c"), None, &mcp)
            .unwrap();
        let err = a
            .delete("item", "i2", "buyout", 1, &Provenance::via(""))
            .unwrap_err();
        assert!(matches!(err, AnnotationError::Provenance { .. }), "{err}");
        assert!(a.get("item", "i2", "buyout").unwrap().is_some());
        // The row's JSON carries the stamp; a missing actor is omitted, not null.
        let json = serde_json::to_value(a.get("item", "i2", "buyout").unwrap().unwrap()).unwrap();
        assert_eq!(json["written_via"], "mcp");
        assert!(json.get("actor").is_none());
    }

    /// C66 — the generic at the write door: no version, a newer version,
    /// the kind's own refusal, and a current-schema value that does not
    /// re-serialize to what was read all fail before anything lands.
    #[test]
    fn c66_the_write_door_is_typed_and_a_value_that_fails_never_lands() {
        /// A kind with one field, `n`, that must be an integer; its
        /// canonical form has nothing else.
        #[derive(Debug, Serialize)]
        struct Strict {
            version: i64,
            n: i64,
        }
        impl IntentValue for Strict {
            const KIND: &'static str = "strict";
            const VERSION: i64 = 2;
            fn parse(value: &Value) -> Result<Self, String> {
                #[derive(Deserialize)]
                #[serde(deny_unknown_fields)]
                struct Wire {
                    version: i64,
                    n: i64,
                    // v2 only; v1 read `m` and upgrades.
                    #[serde(default)]
                    m: Option<i64>,
                }
                let w: Wire = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
                match (w.version, w.m) {
                    (1, Some(m)) => Ok(Strict { version: 2, n: m }),
                    (1, None) => Err("v1 needs `m`".into()),
                    (2, None) => Ok(Strict { version: 2, n: w.n }),
                    (2, Some(_)) => Err("v2 has no `m`".into()),
                    (v, _) => Err(format!("version {v}")),
                }
            }
        }
        let mut a = Annotations::open_memory().unwrap();
        let via = via_test();
        let refused = |a: &mut Annotations, v: Value| {
            let err = a.put::<Strict>("item", "i1", &v, None, &via).unwrap_err();
            assert!(
                a.get("item", "i1", "strict").unwrap().is_none(),
                "{v} landed"
            );
            match err {
                AnnotationError::Invalid(e) => e,
                other => panic!("{v}: expected Invalid, got {other}"),
            }
        };
        assert_eq!(
            refused(&mut a, json!({ "n": 1 })),
            ValueError::MissingVersion { kind: "strict" }
        );
        // Newer is reported as newer, before the shape is read.
        assert_eq!(
            refused(&mut a, json!({ "version": 3, "n": 1, "future": true })),
            ValueError::VersionUnsupported {
                kind: "strict",
                found: 3,
                supported: 2
            }
        );
        // The kind's own strictness: an unknown field.
        assert!(matches!(
            refused(&mut a, json!({ "version": 2, "n": 1, "typo": 1 })),
            ValueError::Malformed { detail, .. } if detail.contains("typo")
        ));
        // Current schema, parses, but the canonical form differs
        // (`m: null` is read as absent and written as nothing): refused
        // naming the path and both sides — null is not absent.
        let err = refused(&mut a, json!({ "version": 2, "n": 1, "m": null }));
        assert_eq!(
            err,
            ValueError::NotCanonical {
                kind: "strict",
                path: "/m".into(),
                read: Some(Value::Null),
                canonical: None,
            }
        );
        assert_eq!(
            err.to_string(),
            "strict: not in canonical form at /m: read null, the canonical form has absent"
        );
        // An older stamp upgrades in memory and lands as written.
        let v1 = json!({ "version": 1, "n": 0, "m": 7 });
        a.put::<Strict>("item", "i1", &v1, None, &via).unwrap();
        let (row, typed) = a.get_as::<Strict>("item", "i1").unwrap().unwrap();
        assert_eq!(row.value, v1, "stored as written");
        assert_eq!((typed.version, typed.n), (2, 7), "upgraded in memory");
        // A current-schema value lands and reads back typed.
        let v2 = json!({ "version": 2, "n": 9 });
        a.put::<Strict>("item", "i1", &v2, Some(1), &via).unwrap();
        let (_, typed) = a.get_as::<Strict>("item", "i1").unwrap().unwrap();
        assert_eq!(typed.n, 9);
        // A stored row a newer build wrote is an error on read, never "none".
        a.conn
            .execute(
                "UPDATE annotations SET value = ?1 WHERE kind = 'strict'",
                [json!({ "version": 3, "n": 1 }).to_string()],
            )
            .unwrap();
        assert!(matches!(
            a.get_as::<Strict>("item", "i1").unwrap_err(),
            AnnotationError::Invalid(ValueError::VersionUnsupported { found: 3, .. })
        ));
        assert!(a.get_as::<Note>("item", "nope").unwrap().is_none());
    }

    /// The difference finder names the first path that differs, both
    /// sides, and treats key order as no difference.
    #[test]
    fn the_first_difference_names_the_path_and_both_sides() {
        let same = json!({ "b": [1, { "c": 2 }], "a": 1 });
        let reordered = json!({ "a": 1, "b": [1, { "c": 2 }] });
        assert_eq!(first_difference(&same, &reordered, ""), None);
        assert_eq!(
            first_difference(
                &json!({ "a": { "b": [1, 2] } }),
                &json!({ "a": { "b": [1, 3] } }),
                ""
            ),
            Some(("/a/b/1".into(), Some(json!(2)), Some(json!(3))))
        );
        assert_eq!(
            first_difference(&json!({ "a": [1] }), &json!({ "a": [1, 2] }), ""),
            Some(("/a/1".into(), None, Some(json!(2))))
        );
        assert_eq!(
            first_difference(&json!({ "a": 1, "x": null }), &json!({ "a": 1 }), ""),
            Some(("/x".into(), Some(Value::Null), None))
        );
        assert_eq!(
            first_difference(&json!(1), &json!("1"), ""),
            Some(("/".into(), Some(json!(1)), Some(json!("1"))))
        );
    }

    /// C35 — a backup is a whole, standalone annotation file that leaves
    /// no partial and never overwrites. That the copy is consistent under
    /// a concurrent writer is `VACUUM INTO`'s and no test here drives a
    /// writer through an export: a torn copy fails too rarely to pin.
    #[test]
    fn export_is_a_whole_standalone_file_and_never_overwrites() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        let uuid = "00000000-0000-4000-8000-000000000001";
        let mut a = Annotations::open(&annotations_path(&dir, uuid)).unwrap();
        a.put::<Buyout>("item", "i1", &price("1c"), None, &via_test())
            .unwrap();
        let backup = dir.join("backup.db");
        a.export(&backup).unwrap();
        let leftovers: Vec<_> = std::fs::read_dir(&dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().contains(".partial"))
            .collect();
        assert!(
            leftovers.is_empty(),
            "no partial outlives the export: {leftovers:?}"
        );
        // The snapshot is a complete, standalone annotation file.
        let restored = Annotations::open(&backup).unwrap();
        assert_eq!(restored.list(None, None).unwrap().rows.len(), 1);
        // A second export to the same path is refused, not an overwrite —
        // and refused before anything is written.
        assert!(a.export(&backup).is_err());
    }

    /// C35 — two exports racing one destination: each writes its own
    /// partial, exactly one is published, the other is refused as
    /// AlreadyExists, and the published file is a whole backup.
    #[test]
    fn simultaneous_exports_to_one_destination_publish_exactly_one() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        let path = annotations_path(&dir, "u-x");
        let mut a = Annotations::open(&path).unwrap();
        a.put::<Buyout>("item", "i1", &price("1c"), None, &via_test())
            .unwrap();
        let b = Annotations::open(&path).unwrap();
        let backup = dir.join("backup.db");
        let dest = &backup;
        let results = std::thread::scope(|scope| {
            let ta = scope.spawn(move || a.export(dest));
            let tb = scope.spawn(move || b.export(dest));
            [ta.join().unwrap(), tb.join().unwrap()]
        });
        let (ok, err): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);
        assert_eq!((ok.len(), err.len()), (1, 1), "one backup, one refusal");
        match err.into_iter().next().unwrap().unwrap_err() {
            AnnotationError::Io(e) => assert_eq!(e.kind(), std::io::ErrorKind::AlreadyExists),
            other => panic!("expected AlreadyExists, got {other}"),
        }
        let restored = Annotations::open(&backup).unwrap();
        assert_eq!(restored.list(None, None).unwrap().rows.len(), 1);
        let leftovers = std::fs::read_dir(&dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().contains(".partial"))
            .count();
        assert_eq!(leftovers, 0);
    }

    /// A file that exists with content but no schema stamp is refused and
    /// left as found — never adopted as empty intent; a zero-byte file
    /// holds nothing and is created over.
    #[test]
    fn an_unstamped_file_with_content_is_refused_and_an_empty_one_is_created_over() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        let path = annotations_path(&dir, "u-1");
        {
            let conn = Connection::open(&path).unwrap();
            conn.execute_batch(
                "CREATE TABLE annotations (scope TEXT, key TEXT, kind TEXT, value TEXT);
                 INSERT INTO annotations VALUES ('item', 'i1', 'buyout', '{}');",
            )
            .unwrap();
            // no user_version stamp
        }
        let before = std::fs::read(&path).unwrap();
        match Annotations::open(&path).err() {
            Some(AnnotationError::SchemaTooOld {
                found: 0, path: p, ..
            }) => {
                assert_eq!(p, path);
            }
            other => panic!("expected SchemaTooOld found 0, got {other:?}"),
        }
        assert_eq!(std::fs::read(&path).unwrap(), before, "left as found");
        let err = match Annotations::open(&path) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("opened"),
        };
        assert!(err.contains("no schema stamp"), "{err}");
        // Zero bytes: nothing to preserve.
        let empty = annotations_path(&dir, "u-2");
        std::fs::write(&empty, b"").unwrap();
        let a = Annotations::open_for(&dir, "u-2").unwrap();
        assert_eq!(a.uuid(), Some("u-2"));
    }

    /// C35 — the creation race's second window, held open by hand: the
    /// winner has created the file and its tables inside its immediate
    /// transaction and has not yet committed the stamp. A losing first
    /// opener must wait on the lock and then proceed as if it had found
    /// the stamp — never be refused as a foreign file (seen once in five
    /// gate runs, 2026-09-09, before this pin).
    #[test]
    fn c35_a_losing_first_opener_waits_for_the_winners_stamp_and_proceeds() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        let path = annotations_path(&dir, "u-w");
        // Scoped, the winner inside: a failing assertion drops the winner,
        // so the loser gets the lock and finishes before the folder goes.
        let opened = std::thread::scope(|s| {
            let winner = Connection::open(&path).unwrap();
            // As a creator does: the journal-mode switch writes the file's
            // first page, so the file exists with content before the stamp.
            winner.pragma_update(None, "journal_mode", "WAL").unwrap();
            winner.execute_batch("BEGIN IMMEDIATE").unwrap();
            winner.execute_batch(SCHEMA).unwrap();
            // The file exists with the winner's uncommitted tables; no stamp.
            let (tx, rx) = std::sync::mpsc::channel();
            let dir = &dir;
            let loser = s.spawn(move || {
                let opened =
                    Annotations::open_for(dir, "u-w").map(|a| a.uuid().map(str::to_string));
                tx.send(()).unwrap();
                opened
            });
            // The loser is on the lock, not refused: nothing arrives while
            // the winner holds it.
            assert!(
                rx.recv_timeout(std::time::Duration::from_millis(400))
                    .is_err(),
                "the loser returned while the winner still held its create"
            );
            winner
                .pragma_update(None, "user_version", SCHEMA_VERSION)
                .unwrap();
            winner.execute_batch("COMMIT").unwrap();
            drop(winner);
            loser.join().unwrap()
        });
        match opened {
            Ok(uuid) => assert_eq!(uuid.as_deref(), Some("u-w")),
            Err(e) => panic!("the loser was refused: {e}"),
        }
        // And the file is a v3 file both can use.
        let again = Annotations::open_for(&dir, "u-w").unwrap();
        assert_eq!(again.uuid(), Some("u-w"));
    }

    /// C35 — the other boundary under the same hold: a writer that lands
    /// tables under the lock and commits *without* a stamp is a foreign
    /// file by the time the loser gets the lock. The loser is refused
    /// there, and the file is byte for byte what the foreign writer left —
    /// no journal-mode switch, no schema, no stamp of ours.
    #[test]
    fn c35_a_foreign_writer_landing_under_the_lock_is_refused_and_left_untouched() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        let path = annotations_path(&dir, "u-f");
        // Scoped, the foreign writer inside: a failing assertion drops it,
        // so the loser gets the lock and finishes before the folder goes.
        let (after_commit, loser) = std::thread::scope(|s| {
            let foreign = Connection::open(&path).unwrap();
            foreign.execute_batch("BEGIN IMMEDIATE").unwrap();
            foreign
                .execute_batch("CREATE TABLE theirs (x INTEGER); INSERT INTO theirs VALUES (1);")
                .unwrap();
            let (tx, rx) = std::sync::mpsc::channel();
            let path = &path;
            let loser = s.spawn(move || {
                let r = Annotations::open(path)
                    .map(|_| ())
                    .map_err(|e| e.to_string());
                tx.send(()).unwrap();
                r
            });
            assert!(
                rx.recv_timeout(std::time::Duration::from_millis(400))
                    .is_err(),
                "the loser returned while the foreign writer still held its lock"
            );
            foreign.execute_batch("COMMIT").unwrap();
            drop(foreign);
            let after_commit = std::fs::read(path).unwrap();
            (after_commit, loser.join().unwrap())
        });
        let err = loser.expect_err("a stampless file with tables opened");
        assert!(err.contains("no schema stamp"), "{err}");
        assert_eq!(std::fs::read(&path).unwrap(), after_commit, "left as found");
        assert!(
            !path.with_extension("annotations.db-wal").exists()
                && !dir.join("u-f.annotations.db-wal").exists(),
            "a journal-mode switch touched the foreign file"
        );
    }

    /// C35 — the intent file is fully synchronous: a commit that returned
    /// survives a power loss, not only a process crash (the fact store's
    /// NORMAL is fine for refetchable facts, not here).
    #[test]
    fn c35_the_intent_file_is_fully_synchronous() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        let a = Annotations::open_for(&dir, "u-1").unwrap();
        let sync: i64 = a
            .conn
            .query_row("PRAGMA synchronous", [], |r| r.get(0))
            .unwrap();
        assert_eq!(sync, 2, "PRAGMA synchronous must be FULL (2)");
        let mode: String = a
            .conn
            .query_row("PRAGMA journal_mode", [], |r| r.get(0))
            .unwrap();
        assert_eq!(mode, "wal");
    }

    /// C35 — the file is bound to the account uuid inside it; a copy under another account's name is refused.
    #[test]
    fn open_for_stamps_the_uuid_and_refuses_a_foreign_file() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        // First open stamps; reopening from the raw path still knows the
        // owner, because the uuid lives inside the file.
        {
            let a = Annotations::open_for(&dir, "u-1").unwrap();
            assert_eq!(a.uuid(), Some("u-1"));
        }
        let raw = Annotations::open(&annotations_path(&dir, "u-1")).unwrap();
        assert_eq!(raw.uuid(), Some("u-1"));
        // u-1's file copied over u-2's path: open_for(u-2) refuses to
        // rebind it rather than adopting the copy.
        std::fs::copy(annotations_path(&dir, "u-1"), annotations_path(&dir, "u-2")).unwrap();
        match Annotations::open_for(&dir, "u-2").err() {
            Some(AnnotationError::WrongAccount { stored, requested }) => {
                assert_eq!((stored.as_str(), requested.as_str()), ("u-1", "u-2"));
            }
            other => panic!("expected WrongAccount, got {other:?}"),
        }
    }

    /// C65 — v3 is the floor: a file a development build wrote below it
    /// is refused naming the file and the fix, never migrated with a
    /// writer invented for its rows, and never touched.
    #[test]
    fn c65_a_file_below_the_floor_is_refused_never_migrated() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        let path = annotations_path(&dir, "u-1");
        let v2_table = "CREATE TABLE annotations (
                scope TEXT NOT NULL, key TEXT NOT NULL, kind TEXT NOT NULL,
                value TEXT NOT NULL, revision INTEGER NOT NULL,
                created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL,
                deleted_at INTEGER, PRIMARY KEY (scope, key, kind));
            CREATE TABLE meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);
            INSERT INTO meta VALUES ('account_uuid', 'u-1');
            INSERT INTO annotations VALUES ('account', '', 'sync-policy', '{\"version\":1}', 9, 1, 2, NULL);";
        {
            let conn = Connection::open(&path).unwrap();
            conn.execute_batch(v2_table).unwrap();
            conn.pragma_update(None, "user_version", 2).unwrap();
        }
        let before = std::fs::read(&path).unwrap();
        match Annotations::open_for(&dir, "u-1").err() {
            Some(AnnotationError::SchemaTooOld {
                found: 2,
                floor: 3,
                path: p,
            }) => assert_eq!(p, path),
            other => panic!("expected SchemaTooOld, got {other:?}"),
        }
        let err = match Annotations::open(&path) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("a v2 file opened"),
        };
        assert!(
            err.contains("v2") && err.contains("floor v3") && err.contains("delete it"),
            "{err}"
        );
        assert_eq!(
            std::fs::read(&path).unwrap(),
            before,
            "refused, not touched"
        );
        // A fresh file is created at the current version outright.
        let a = Annotations::open_for(&dir, "u-2").unwrap();
        let v: i64 = a
            .conn
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .unwrap();
        assert_eq!(v, SCHEMA_VERSION);
    }

    /// C35 — a file a newer build wrote is refused, never guessed at.
    #[test]
    fn a_newer_schema_is_refused_not_guessed_at() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        let path = annotations_path(&dir, "u-1");
        {
            let a = Annotations::open(&path).unwrap();
            a.conn.pragma_update(None, "user_version", 99).unwrap();
        }
        match Annotations::open(&path).err() {
            Some(AnnotationError::SchemaTooNew { found: 99, .. }) => {}
            other => panic!("expected SchemaTooNew, got {other:?}"),
        }
    }

    fn file_hash(path: &Path) -> Vec<u8> {
        use sha2::Digest;
        sha2::Sha256::digest(std::fs::read(path).unwrap()).to_vec()
    }

    /// C124 (a read never migrates): the annotations file's read-only
    /// door refuses every older file and one a version ahead with
    /// `NotCurrent`, in words naming both versions and the remedy, and
    /// leaves its bytes as found. The remedy is the writing door's: a file
    /// below the floor (v1, v2) is told to be deleted and its policy set
    /// again, never to be migrated, as `init` refuses it; one from the
    /// floor up, which `init` would migrate, is told to open it with `acq`.
    /// An unstamped (empty) file is refused with the next step and left
    /// empty; a missing file is refused, not created.
    #[test]
    fn c124_a_read_only_open_refuses_another_version_and_leaves_the_file_unchanged() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        assert!(Annotations::open_read_only_for(&dir, "u-0").is_err());
        assert!(
            !annotations_path(&dir, "u-0").exists(),
            "a read-only open created the file"
        );
        for stamp in (1..SCHEMA_VERSION).chain([SCHEMA_VERSION + 1]) {
            let uuid = format!("u-{stamp}");
            {
                let a = Annotations::open_for(&dir, &uuid).unwrap();
                a.conn.pragma_update(None, "user_version", stamp).unwrap();
            }
            let path = annotations_path(&dir, &uuid);
            let before = file_hash(&path);
            let err = Annotations::open_read_only_for(&dir, &uuid).err();
            match &err {
                Some(AnnotationError::NotCurrent(e)) => assert_eq!(
                    (e.file, e.found, e.supported),
                    ("annotation", stamp, SCHEMA_VERSION)
                ),
                other => panic!("expected NotCurrent, got {other:?}"),
            }
            let text = err.map(|e| e.to_string()).unwrap_or_default();
            assert!(
                text.contains(&format!("schema version {stamp}"))
                    && text.contains(&format!("this build's version {SCHEMA_VERSION}")),
                "{text}"
            );
            let remedy = if stamp < SCHEMA_FLOOR {
                "a development build wrote it; delete it and set the policy again"
            } else if stamp < SCHEMA_VERSION {
                "open it with `acq` once to migrate, or use the build that wrote it"
            } else {
                "use the build that wrote it"
            };
            assert!(text.contains(remedy), "{text}");
            assert_eq!(
                file_hash(&path),
                before,
                "the refused v{stamp} file changed"
            );
            if stamp < SCHEMA_FLOOR {
                assert!(!text.contains("migrat"), "{text}");
                // The writing door refuses it in the same words.
                let writing = Annotations::open_for(&dir, &uuid).err();
                assert!(
                    matches!(writing, Some(AnnotationError::SchemaTooOld { .. })),
                    "{writing:?}"
                );
                let writing = writing.map(|e| e.to_string()).unwrap_or_default();
                assert!(
                    writing.contains("delete it and set the policy again"),
                    "{writing}"
                );
            }
        }
        let empty = annotations_path(&dir, "u-empty");
        std::fs::write(&empty, b"").unwrap();
        match Annotations::open_read_only_for(&dir, "u-empty").err() {
            Some(AnnotationError::NotCurrent(e)) => {
                assert_eq!(e.found, 0);
                let text = e.to_string();
                assert!(
                    text.contains("carries no schema version")
                        && text.contains(
                            "check that `--dir` and `--account` name an existing Acquisition store"
                        ),
                    "{text}"
                );
            }
            other => panic!("expected NotCurrent, got {other:?}"),
        }
        assert!(
            std::fs::read(&empty).unwrap().is_empty(),
            "the empty file was written"
        );
    }

    /// C124: a current file opens through the read-only door bound to
    /// the uuid it stores and reads; a write through it is refused; its
    /// bytes are unchanged; another account's file is refused as
    /// `WrongAccount`, as `open_for` refuses it.
    #[test]
    fn c124_a_read_only_open_reads_a_current_file_and_writes_nothing() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        {
            let mut a = Annotations::open_for(&dir, "u-1").unwrap();
            a.put::<Buyout>("item", "i1", &price("1 divine"), None, &via_test())
                .unwrap();
        }
        let path = annotations_path(&dir, "u-1");
        let before = file_hash(&path);
        let mut ro = Annotations::open_read_only_for(&dir, "u-1").unwrap();
        assert_eq!(ro.uuid(), Some("u-1"));
        assert_eq!(ro.list(None, None).unwrap().rows.len(), 1);
        assert!(
            ro.put::<Buyout>("item", "i2", &price("2 divine"), None, &via_test())
                .is_err(),
            "a write through the read-only door landed"
        );
        drop(ro);
        assert_eq!(file_hash(&path), before, "the current file changed");
        std::fs::copy(&path, annotations_path(&dir, "u-2")).unwrap();
        match Annotations::open_read_only_for(&dir, "u-2").err() {
            Some(AnnotationError::WrongAccount { stored, requested }) => {
                assert_eq!((stored.as_str(), requested.as_str()), ("u-1", "u-2"));
            }
            other => panic!("expected WrongAccount, got {other:?}"),
        }
    }

    /// C35 — named by the account uuid, never the username.
    #[test]
    fn the_file_is_named_by_uuid() {
        let p = annotations_path(Path::new("/store/mock"), "0000-4000#odd");
        assert_eq!(p.file_name().unwrap(), "0000-4000_odd.annotations.db");
    }

    /// C35, the first-open race's third window (round 25), staged
    /// exactly: the loser's connection carries an authorizer that fires
    /// when its second read — the table count — is prepared, after the
    /// stamp was read; there the winner attempts, on its own connection,
    /// to create the schema, stamp it and commit. Under the round-24
    /// gate the loser held no lock between its two statements, so the
    /// attempt commits and the loser then sees stamp 0 beside the
    /// winner's tables and refuses a well-formed file as "content but no
    /// schema stamp". Under the fixed gate the loser's stamp read already
    /// holds the snapshot, so the attempt cannot commit between the
    /// reads — it waits on the loser and gives up after its busy
    /// timeout, which the pass path asserts — and the loser opens the
    /// file. What this test asserts is the fixed outcome — the attempt
    /// could not commit and the loser opened the file; the old outcome
    /// was observed by breaking the gate (the round-24 shape staged
    /// back, review rounds 25 and 26: the attempt committed and the
    /// loser refused with the production message), not asserted here. A
    /// stress test of 3000 concurrent first opens did not reproduce the
    /// window; this does, every run.
    #[test]
    fn c35_a_create_attempted_between_the_gates_two_reads_cannot_commit_or_cause_a_refusal() {
        use rusqlite::hooks::{AuthAction, AuthContext, Authorization};
        use std::sync::{
            Arc,
            atomic::{AtomicBool, AtomicU32, Ordering},
        };
        let temp = tempfile::tempdir().unwrap();
        let base = temp.path().to_path_buf();
        let path = base.join("u-race.annotations.db");
        let loser = Connection::open(&path).unwrap();
        let fired = Arc::new(AtomicBool::new(false));
        let committed = Arc::new(AtomicU32::new(0));
        let (winner_path, fired_in, committed_in) =
            (path.clone(), fired.clone(), committed.clone());
        loser.authorizer(Some(move |ctx: AuthContext<'_>| {
            if let AuthAction::Read { table_name, .. } = ctx.action
                && table_name == "sqlite_master"
                && !fired_in.swap(true, Ordering::SeqCst)
            {
                // The winner, between the loser's two reads: a short
                // busy timeout, since under the fixed gate its commit
                // waits on the loser's snapshot and must give up.
                let winner = Connection::open(&winner_path).unwrap();
                winner
                    .busy_timeout(std::time::Duration::from_millis(200))
                    .unwrap();
                let create = (|| -> rusqlite::Result<()> {
                    winner.execute_batch("BEGIN IMMEDIATE")?;
                    winner.execute_batch(SCHEMA)?;
                    winner.pragma_update(None, "user_version", SCHEMA_VERSION)?;
                    winner.execute_batch("COMMIT")
                })();
                if create.is_ok() {
                    committed_in.fetch_add(1, Ordering::SeqCst);
                } else {
                    let _ = winner.execute_batch("ROLLBACK");
                }
            }
            Authorization::Allow
        }));
        let opened = Annotations::init(loser, path.clone());
        assert!(fired.load(Ordering::SeqCst), "the staging never fired");
        match opened {
            // The fixed gate: the attempt could not commit between the
            // loser's reads, and the loser opened the file.
            Ok(_) => assert_eq!(
                committed.load(Ordering::SeqCst),
                0,
                "the winner committed between the loser's two reads"
            ),
            Err(e) => panic!(
                "the loser refused the winner's file (winner committed between the reads: {}): {e}",
                committed.load(Ordering::SeqCst) == 1
            ),
        }
        // Whoever created it, the file is one well-formed store afterwards.
        if let Err(e) = Annotations::open(&path) {
            panic!("the file is not a well-formed store afterwards: {e}");
        }
    }
}
