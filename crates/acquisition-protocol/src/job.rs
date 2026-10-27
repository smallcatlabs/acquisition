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
    /// A failure that is not a parent's: the message alone, no tally.
    pub fn failure(error: impl Into<String>) -> Outcome {
        Outcome::Failure {
            error: error.into(),
            children: None,
            payload: None,
        }
    }

    /// C144: a failure that keeps what the job fetched.
    pub fn failure_keeping(error: impl Into<String>, payload: Value) -> Outcome {
        Outcome::Failure {
            error: error.into(),
            children: None,
            payload: Some(payload),
        }
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
