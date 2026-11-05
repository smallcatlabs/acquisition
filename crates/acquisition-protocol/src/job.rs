use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type JobId = u64;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobState {
    Waiting,
    Running,
    Done,
    Failed,
    Cancelled,
}

impl JobState {
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            JobState::Done | JobState::Failed | JobState::Cancelled
        )
    }
}

impl fmt::Display for JobState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            JobState::Waiting => "waiting",
            JobState::Running => "running",
            JobState::Done => "done",
            JobState::Failed => "failed",
            JobState::Cancelled => "cancelled",
        };
        f.write_str(s)
    }
}

/// Higher priority runs sooner; ties break by submission order.
pub type Priority = u8;

/// How many times a job is re-queued after a 429 before it fails for good
/// (ground truth P-A: violations are structural, so recovery is required;
/// N10: frequent violations get the application revoked, so it is bounded).
/// A promise the daemon keeps and a consumer computes with: the planner's
/// request ceiling is `logical × (1 + MAX_429_RETRIES)`.
pub const MAX_429_RETRIES: u32 = 2;

/// C127 — The kinds a client may submit in either provider mode, in the
/// order every surface lists them. The daemon's door and its refusal of
/// an unknown kind, `acq submit`'s help and the MCP's `submit_job` read
/// this list and [`MOCK_ONLY_KINDS`], so no surface names a kind the door
/// refuses or leaves out one it admits. A kind in neither list is the
/// daemon's own (`probe`).
pub const CLIENT_KINDS: &[&str] = &[
    "profile",
    "characters",
    "character",
    "leagues",
    "stashes",
    "stash",
    "apply",
];

/// C127 — The kinds a client may submit against the mock alone: `sleep`
/// sends nothing, `fetch` and `whoami` only to the mock's own endpoints.
/// The door refuses each in real mode.
pub const MOCK_ONLY_KINDS: &[&str] = &["sleep", "fetch", "whoami"];

/// The kinds a client may name in this provider mode, comma-separated:
/// [`CLIENT_KINDS`], then in mock mode [`MOCK_ONLY_KINDS`]. The words the
/// daemon's refusal lists and the frontends' descriptions print.
pub fn client_kinds(real: bool) -> String {
    let mock: &[&str] = if real { &[] } else { MOCK_ONLY_KINDS };
    [CLIENT_KINDS, mock].concat().join(", ")
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobInfo {
    pub id: JobId,
    pub kind: String,
    pub state: JobState,
    pub priority: Priority,
    pub submitted_by: String,
    /// Predicted seconds until this job starts running. Only meaningful for
    /// waiting jobs; computed from limiter state + same-route queue depth.
    pub eta_seconds: Option<u64>,
    /// The job that submitted this one (a refresh's tabs, a deep tab's
    /// substashes). A parent finishes when its last descendant does.
    pub parent: Option<JobId>,
    /// Times this job has been put back in the queue after a 429.
    pub retries: u32,
    /// The GGG account this job runs as (`name#discriminator`), fixed at
    /// submit: it selects the token it sends with and the store file its
    /// response lands in. `None` only for jobs that need no account
    /// (`sleep`, the mock's `fetch`).
    #[serde(default)]
    pub account: Option<String>,
    /// What the job was submitted with, verbatim. Public: every connected
    /// client sees it, so a job's params must never carry a secret (tokens
    /// are fetched inside the daemon, never passed in). This is what makes
    /// a queued job identifiable to a person — the prerequisite for any
    /// queue-management UI — and what lets a client label a failed child.
    pub params: Value,
    /// C165: when the job was submitted, started and finished, each
    /// absent until it happens.
    #[serde(flatten)]
    pub times: JobTimes,
}

/// C165 (a job's record says when): unix seconds on the daemon's one
/// clock, each absent until it happens — a waiting job has no
/// `started_at`, an unfinished one no `finished_at`. `started_at` is the
/// first start: a job re-queued after a 429 waits again with it kept,
/// and a job a restarted daemon re-queues keeps its predecessor's. A
/// record from before the field reads with none. Additive (C10, the
/// handshake is single-version): a reader that knows none of them reads
/// the record unchanged.
///
/// Each `_age` is that time in words at a frontend's answer (C53, C163),
/// said by the frontend from the store's one formatter; the daemon never
/// sends one.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobTimes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitted_at_age: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at_age: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at_age: Option<String>,
}

impl JobInfo {
    /// A short human label for what the job targets, derived from its
    /// params: `Standard/cur1`, `Standard/maps/m003`, `Standard (all)`.
    /// Rendering is the client's business; this is the one shared default.
    pub fn target(&self) -> String {
        target_of(&self.kind, &self.params)
    }
}

/// `JobInfo::target` for a kind and params without a `JobInfo`.
/// A realm other than pc leads the label (`xbox/Standard/cur1`,
/// `poe2/Exile`); pc is silent, as on the wire.
pub fn target_of(kind: &str, p: &Value) -> String {
    {
        let s = |k: &str| p.get(k).and_then(Value::as_str);
        // `pc` and an absent or unparseable realm all show nothing: the
        // label is for a person, and admission already refused bad values.
        let r = match s("realm") {
            Some(realm) if realm != "pc" => format!("{realm}/"),
            _ => String::new(),
        };
        match kind {
            "stash" => match (s("league"), s("id"), s("sub")) {
                (Some(l), Some(id), Some(sub)) => format!("{r}{l}/{id}/{sub}"),
                (Some(l), Some(id), None) => format!("{r}{l}/{id}"),
                _ => String::new(),
            },
            "stashes" => format!("{r}{}", s("league").unwrap_or("")),
            "characters" => r.trim_end_matches('/').to_string(),
            "character" => format!("{r}{}", s("name").unwrap_or("")),
            "apply" => format!(
                "plan ({} requests)",
                p.get("jobs").and_then(Value::as_array).map_or(0, Vec::len)
            ),
            "probe" => s("route").unwrap_or("").to_string(),
            _ => String::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum Outcome {
    Success {
        payload: Value,
    },
    /// `error` is the sentence for a reader; `children` is a parent's
    /// tally, present on every parent failure and absent on any other
    /// (C23, a parent job submits children): a client reads the field,
    /// never the sentence.
    Failure {
        error: String,
        /// C165: what kind of failure it is, from the closed list
        /// [`FailureKind`], read where the daemon failed the job, never
        /// from `error`. Absent on a failure from before the field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        kind: Option<FailureKind>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        children: Option<Children>,
        /// C144 (the daemon reports no success it did not earn): what a
        /// job fetched and could not record — the send was made and its
        /// body is kept here, so it is not wasted, while the job reports
        /// the failure it is. Absent on every other failure, so a failure
        /// from before the field reads unchanged.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        payload: Option<Value>,
    },
    Cancelled,
}

impl Outcome {
    /// A failure that is not a parent's: its kind and its message, no
    /// tally.
    pub fn failure(kind: FailureKind, error: impl Into<String>) -> Outcome {
        Outcome::Failure {
            error: error.into(),
            kind: Some(kind),
            children: None,
            payload: None,
        }
    }

    /// C144: a failure that keeps what the job fetched, of kind
    /// [`FailureKind::NotRecorded`].
    pub fn failure_keeping(error: impl Into<String>, payload: Value) -> Outcome {
        Outcome::Failure {
            error: error.into(),
            kind: Some(FailureKind::NotRecorded),
            children: None,
            payload: Some(payload),
        }
    }
}

/// C165 (a failed job says what kind of failure it is): the closed list,
/// one kind per distinct way on, each read from what the daemon knows
/// where it fails the job, never from the failure's sentence. A frontend
/// offers the way on in its own terms (C143); a kind added is a protocol
/// change, pinned by the `failure_kinds` fixture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureKind {
    /// GGG answered 404: what the job asked for is not there. The way
    /// on is the fetch of the listing that names it: a tab's league
    /// listing, a substash's parent tab, a character's list.
    NotFound,
    /// GGG answered and the answer could not be used — a status it gives
    /// for a fault of its own or a request it refuses (other than a 403
    /// or a 503), a response the daemon could not read, a send that never
    /// reached it — or a route closed after its probe failed. The way on
    /// is the same job, later.
    Upstream,
    /// A 429 ended the job: its re-queues spent, or a `Retry-After` the
    /// daemon will not wait. The way on is the same job once the
    /// limiter's hold has passed.
    RateLimited,
    /// Invariant 3: Cloudflare answered in GGG's place or could not reach
    /// it — the choke point's verdict, the block mark or the halt it sets
    /// (C129), and nothing else. No way on sends anything: not this job
    /// again, not another, until a person clears the halt.
    Blocked,
    /// GGG answered 403 or 503 without Cloudflare's signatures: an answer
    /// the daemon never retries, since one like it may be Cloudflare's
    /// (invariant 3). Sends go on; the way on is a person's look, a
    /// login where the 403 names an authorization error, and no offer
    /// sends this job again.
    NotRetried,
    /// C144: the response was fetched and the store could not record it;
    /// the body is kept in the failure's `payload`, which is where the
    /// way on reads it.
    NotRecorded,
    /// C23: a parent whose children failed or were cancelled; the way on
    /// is each failed child's own result.
    Children,
    /// The job's account has no session that can give a token, or the
    /// login it belonged to was lost: the way on is a login, a person's.
    NotLoggedIn,
    /// A daemon's end or fault stopped the job before it finished, and
    /// it was not replayed: the way on is the same job again.
    Interrupted,
    /// The daemon's own fault: a row it cannot run, a rate-limit state it
    /// could not read, a fan-out it could not finish submitting. No way
    /// on is offered.
    Internal,
}

impl FailureKind {
    /// Every kind, in declaration order: the closed set the fixture pins.
    pub const ALL: [FailureKind; 10] = [
        FailureKind::NotFound,
        FailureKind::Upstream,
        FailureKind::RateLimited,
        FailureKind::Blocked,
        FailureKind::NotRetried,
        FailureKind::NotRecorded,
        FailureKind::Children,
        FailureKind::NotLoggedIn,
        FailureKind::Interrupted,
        FailureKind::Internal,
    ];

    /// The wire name (`not_found`).
    pub fn as_str(self) -> &'static str {
        match self {
            FailureKind::NotFound => "not_found",
            FailureKind::Upstream => "upstream",
            FailureKind::RateLimited => "rate_limited",
            FailureKind::Blocked => "blocked",
            FailureKind::NotRetried => "not_retried",
            FailureKind::NotRecorded => "not_recorded",
            FailureKind::Children => "children",
            FailureKind::NotLoggedIn => "not_logged_in",
            FailureKind::Interrupted => "interrupted",
            FailureKind::Internal => "internal",
        }
    }
}

impl fmt::Display for FailureKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// C23 (a parent job submits children): what a parent's children did
/// when the last of them finished. The same four numbers ride as
/// `children` in a parent's success payload; on a failure they are this
/// typed field. `failed_ids` is sorted: they are the remedy's handles.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Children {
    pub done: u64,
    pub failed: u64,
    pub cancelled: u64,
    pub failed_ids: Vec<JobId>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// pc is silent in a job's label, as on the wire; any other realm
    /// leads it.
    #[test]
    fn target_labels_lead_with_a_non_pc_realm_only() {
        assert_eq!(
            target_of(
                "stash",
                &json!({ "league": "Standard", "id": "t1", "sub": "s1" })
            ),
            "Standard/t1/s1"
        );
        assert_eq!(
            target_of(
                "stash",
                &json!({ "realm": "pc", "league": "Standard", "id": "t1" })
            ),
            "Standard/t1"
        );
        assert_eq!(
            target_of(
                "stash",
                &json!({ "realm": "xbox", "league": "Standard", "id": "t1" })
            ),
            "xbox/Standard/t1"
        );
        assert_eq!(
            target_of("stashes", &json!({ "realm": "sony", "league": "Standard" })),
            "sony/Standard"
        );
        assert_eq!(target_of("characters", &json!({})), "");
        assert_eq!(target_of("characters", &json!({ "realm": "poe2" })), "poe2");
        assert_eq!(target_of("character", &json!({ "name": "Exile" })), "Exile");
        assert_eq!(
            target_of("character", &json!({ "realm": "poe2", "name": "Exile" })),
            "poe2/Exile"
        );
    }
}
