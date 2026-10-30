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
    /// Crate-private: the store's own tests stage a damaged row on it.
    pub(crate) conn: Connection,
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
    /// simultaneous export could still replace another's. On unix the
    /// publish ends with the directory synced (the crate's one `sync_dir`,
    /// issue #63), so the link is durable as the file is; Windows cannot
    /// sync a directory and the publish stands unsynced there. Fails if `dest`
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
#[path = "../unit/tests/annotations_kinds.rs"]
pub(crate) mod test_kinds;

#[cfg(test)]
#[path = "../unit/tests/annotations.rs"]
mod tests;
