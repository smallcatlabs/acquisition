//! The one fault every verb fails with (C11, the failure document; C85,
//! the closed kinds; C121, an account that cannot be picked).
//!
//! A [`Fault`] says what is wrong and knows the two things a failure
//! document carries beside its words on every surface: its wire `kind`
//! (the protocol's word where the protocol has one) and its fields
//! (`accounts`, `next`). It never says what to do in a frontend's verbs:
//! each frontend has one function from `Fault` to its failure document,
//! matching this enum exhaustively and adding, per variant, the remedy in
//! its own words (the CLI's `--account`, the MCP's `account` argument).
//! A cause added here later cannot be printed without a kind and a
//! remedy, because those matches will not compile.
//!
//! `Display` is the cause in plain words, and the human's command where
//! the remedy is a human's act alone (a login). No id stands in it
//! (C116). A fault is the end of its chain: `source` is none, and an
//! inner error's whole chain is in the words already, so a frontend that
//! prints the chain above it prints this once.

use std::fmt;
use std::path::PathBuf;

use acquisition_client::client::{ConnectError, DaemonError, DaemonId, NotSpawned, Unresponsive};
use acquisition_plan::{PlanError, PutPolicyError, SpendError};
use serde_json::{Map, Value, json};

/// Why a verb did not answer. One variant per cause the layer knows;
/// `Other` is the escape hatch for what it does not name yet.
#[derive(Debug)]
pub enum Fault {
    /// `ACQ_PROVIDER` names no provider, or a retired knob is set (C88).
    /// Kind `refused`.
    Environment(String),
    /// The index under `dir` knows no account at all. Kind
    /// `not_logged_in`, `accounts: []` (C121).
    NoAccounts { dir: PathBuf, provider: String },
    /// The selector names no account the index holds. Kind
    /// `not_logged_in`, `accounts` the names known (C121).
    UnknownAccount {
        selector: String,
        known: Vec<String>,
    },
    /// No selector among several accounts, or one naming several. Kind
    /// `ambiguous_account`, `accounts` the names to pick from (C121).
    AmbiguousAccount { candidates: Vec<String> },
    /// The index entry predates uuid-at-login, so no intent file can be
    /// bound to it (C50). Kind `refused`.
    NoUuid { username: String },
    /// A facts, intent or index file would not open. Kind `store`.
    Unopened(anyhow::Error),
    /// The account has no sync policy to compile. Kind `refused`.
    NoSyncPolicy { username: String },
    /// A malformed plan or policy, an uncovered league. Kind `refused`.
    Plan(PlanError),
    /// A plan that may not be spent here: its policy gone or moved, the
    /// wrong provider or account (C44). Kind `refused`.
    Spend(SpendError),
    /// The policy write refused: invalid, unmergeable, or the store's
    /// compare-and-swap. Kind `refused` (a revision conflict's own kind
    /// waits for the one refusal shape).
    Policy(PutPolicyError),
    /// A write that names no revision over a stored policy this build
    /// cannot read: never replaced blind (C35, intent is irreplaceable).
    /// Kind `refused`.
    UnreadablePolicy { revision: i64, detail: String },
    /// No daemon answers, and this door does not start one: `because` is
    /// the use door's reason, `None` for an observation. Kind
    /// `no_daemon`, with `next` (C10, C13).
    NoDaemon {
        because: Option<NotSpawned>,
        next: String,
    },
    /// No daemon to ask and the ledger on disk holds no such job. Kind
    /// `unknown_job` (the daemon's word for an id it does not remember),
    /// with `next` (C45).
    NoJobOnDisk { id: u64, next: String },
    /// No daemon to ask, and the ledger records the job unfinished: it
    /// has no outcome until a daemon runs it. Kind `wrong_state` (the
    /// daemon's word for a result asked too early), with `next` (C45).
    Unfinished {
        id: u64,
        state: String,
        next: String,
    },
    /// The use door did not open (C10): a mismatch it may not resolve, a
    /// spawn that failed, the transport. Its kind is the door's word.
    Door(ConnectError),
    /// An observation found a daemon that is not this client's: never
    /// replaced by an observer (C10). Kind `incompatible`, or
    /// `other_world` for a daemon on another world (C83).
    AnotherDaemon(Box<DaemonId>),
    /// An observation found a peer that never answered `hello` (C86).
    /// Kind `unresponsive`.
    Unresponsive(Unresponsive),
    /// The daemon's own refusal, its closed kind (C85).
    Daemon(DaemonError),
    /// The conversation with the daemon failed: a closed socket, an
    /// answer of the wrong shape. Kind `transport`.
    Transport(anyhow::Error),
    /// What the layer does not name yet: a store read that failed, the
    /// rest. Kind `refused`.
    Other(anyhow::Error),
}

impl Fault {
    /// The wire word for this fault: the protocol's [`ErrorKind`] word
    /// for the daemon's own refusals and for the causes the daemon names
    /// the same way (C85, C121), the use door's word for a door.
    ///
    /// [`ErrorKind`]: acquisition_protocol::protocol::ErrorKind
    pub fn kind(&self) -> &'static str {
        match self {
            Fault::Environment(_)
            | Fault::NoUuid { .. }
            | Fault::NoSyncPolicy { .. }
            | Fault::Plan(_)
            | Fault::Spend(_)
            | Fault::Policy(_)
            | Fault::UnreadablePolicy { .. }
            | Fault::Other(_) => "refused",
            Fault::NoAccounts { .. } | Fault::UnknownAccount { .. } => "not_logged_in",
            Fault::AmbiguousAccount { .. } => "ambiguous_account",
            Fault::Unopened(_) => "store",
            Fault::NoDaemon { .. } => "no_daemon",
            Fault::NoJobOnDisk { .. } => "unknown_job",
            Fault::Unfinished { .. } => "wrong_state",
            Fault::Door(door) => door_word(door),
            Fault::AnotherDaemon(found) if !found.world_matches() => "other_world",
            Fault::AnotherDaemon(_) => "incompatible",
            Fault::Unresponsive(_) => "unresponsive",
            Fault::Daemon(refused) => refused.kind.as_str(),
            Fault::Transport(_) => "transport",
        }
    }

    /// The fields a failure document carries beside `error`: `kind`, and
    /// `accounts` or `next` where the variant has them (C11, C121).
    pub fn fields(&self) -> Map<String, Value> {
        let mut out = Map::new();
        out.insert("kind".into(), json!(self.kind()));
        match self {
            Fault::NoAccounts { .. } => {
                out.insert("accounts".into(), json!([]));
            }
            Fault::UnknownAccount { known, .. } => {
                out.insert("accounts".into(), json!(known));
            }
            Fault::AmbiguousAccount { candidates } => {
                out.insert("accounts".into(), json!(candidates));
            }
            Fault::NoDaemon { next, .. }
            | Fault::NoJobOnDisk { next, .. }
            | Fault::Unfinished { next, .. } => {
                out.insert("next".into(), json!(next));
            }
            _ => {}
        }
        out
    }

    /// A client call's failure: the daemon's own refusal when the chain
    /// holds one (C85), the conversation's failure otherwise.
    pub fn from_client(e: anyhow::Error) -> Fault {
        match DaemonError::find(&e) {
            Some(refused) => Fault::Daemon(refused.clone()),
            None => Fault::Transport(e),
        }
    }
}

/// C10 (the doors): the use door's word for why it did not open, the
/// vocabulary the MCP's JSON-RPC errors carry as `data.connect`.
pub fn door_word(door: &ConnectError) -> &'static str {
    match door {
        ConnectError::Absent { .. } => "absent",
        ConnectError::Incompatible { .. } => "incompatible",
        ConnectError::OtherWorld { .. } => "other_world",
        ConnectError::SpawnFailed { .. } => "spawn_failed",
        ConnectError::Unresponsive(_) => "unresponsive",
        ConnectError::Transport(_) => "transport",
    }
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fault::Environment(why) => f.write_str(why),
            Fault::NoAccounts { dir, provider } => write!(
                f,
                "no account's store in {} (provider {provider})",
                dir.display()
            ),
            Fault::UnknownAccount { selector, .. } => {
                write!(f, "no account matches {selector:?}")
            }
            Fault::AmbiguousAccount { candidates } => write!(
                f,
                "{} accounts are known and one is to be picked: {}",
                candidates.len(),
                candidates.join(", ")
            ),
            Fault::NoUuid { username } => write!(
                f,
                "account {username} has no recorded uuid (a login predating uuid-at-login)"
            ),
            Fault::Unopened(e) | Fault::Transport(e) | Fault::Other(e) => write!(f, "{e:#}"),
            Fault::NoSyncPolicy { username } => {
                write!(f, "no sync policy is set for {username}")
            }
            Fault::Plan(e) => write!(f, "{e}"),
            Fault::Spend(e) => write!(f, "{e}"),
            Fault::Policy(e) => write!(f, "{e}"),
            Fault::UnreadablePolicy { revision, detail } => write!(
                f,
                "the stored sync policy holds a value this build cannot read (revision \
                 {revision}: {detail}); not replaced blind"
            ),
            Fault::NoDaemon { because: None, .. } => f.write_str("daemon is not running"),
            Fault::NoDaemon {
                because: Some(because),
                ..
            } => write!(f, "{}", ConnectError::Absent { because: *because }),
            Fault::NoJobOnDisk { id, .. } => {
                write!(f, "no job {id} on disk, and the daemon is not running")
            }
            Fault::Unfinished { id, state, .. } => write!(
                f,
                "job {id} is recorded {state} on disk and the daemon is not running"
            ),
            Fault::Door(door) => write!(f, "{door}"),
            Fault::AnotherDaemon(found) => write!(f, "{found}"),
            Fault::Unresponsive(found) => write!(f, "{found}"),
            Fault::Daemon(refused) => write!(f, "{refused}"),
        }
    }
}

impl std::error::Error for Fault {}

impl From<PlanError> for Fault {
    fn from(e: PlanError) -> Fault {
        match e {
            // The planner's own words carry no account; the compile that
            // meets it names one (`refresh::compile`).
            PlanError::NoSyncPolicy => Fault::NoSyncPolicy {
                username: "this account".into(),
            },
            other => Fault::Plan(other),
        }
    }
}

impl From<SpendError> for Fault {
    fn from(e: SpendError) -> Fault {
        Fault::Spend(e)
    }
}

impl From<PutPolicyError> for Fault {
    fn from(e: PutPolicyError) -> Fault {
        Fault::Policy(e)
    }
}
