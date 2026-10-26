//! The daemon's state as an observer reports it (C10, an observation
//! never spawns or replaces; C45, the ledger answers; C84, the identity;
//! C86, an unresponsive peer): four arms, one document both surfaces
//! print.

use std::path::PathBuf;

use acquisition_client::client::{DaemonId, Unresponsive};
use acquisition_protocol::protocol::{Request, Response};
use acquisition_store::world::World;
use serde::Serialize;
use serde::ser::Serializer;
use serde_json::{Value, json};

use crate::fault::Fault;
use crate::frontend::Frontend;
use crate::jobs::unexpected;

/// What the ledger on disk holds, counted, for a daemon status with no
/// daemon (C45).
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PersistedSummary {
    pub waiting: usize,
    pub recorded_running: usize,
    pub written_at: Option<i64>,
}

/// The daemon of this world, as one observation found it.
#[derive(Debug)]
pub enum DaemonReport {
    /// Nothing listens: the ledger's counts, and the log the last daemon
    /// of this world wrote, when one exists (C45, C83).
    Absent {
        persisted: Option<PersistedSummary>,
        log: Option<PathBuf>,
    },
    /// A daemon that is not this client's, by its identity alone (C84):
    /// never replaced by an observer.
    Incompatible(Box<DaemonId>),
    /// Something accepted the connection and never answered `hello`
    /// (C86).
    Unresponsive(Unresponsive),
    /// This client's daemon: its vitals (`Response::DaemonStatus`) and
    /// the identity its handshake carried.
    Running {
        status: Box<Response>,
        identity: Box<DaemonId>,
    },
}

/// The identity keys a running daemon's report carries beside its
/// vitals, the same the incompatible report has (C84): the daemon's
/// contract and artifact, the world and socket, how the artifact relates
/// to this client's sibling, and the sibling under `wanted`.
const IDENTITY_KEYS: [&str; 10] = [
    "contract",
    "artifact",
    "world",
    "socket",
    "contract_matches",
    "artifact_matches",
    "artifact_relation",
    "provider_matches",
    "world_matches",
    "wanted",
];

impl DaemonReport {
    /// C84 (the identity) and C45: the report's one JSON document,
    /// `running`, and `compatible` or `responsive`, beside the arm's own
    /// fields.
    pub fn document(&self) -> Value {
        match self {
            DaemonReport::Absent { persisted, log } => {
                let mut report = json!({ "running": false, "persisted": persisted });
                if let Some(log) = log {
                    report["log"] = json!(log);
                }
                report
            }
            DaemonReport::Incompatible(found) => {
                let mut report = found.report();
                report["running"] = json!(true);
                report["compatible"] = json!(false);
                report
            }
            DaemonReport::Unresponsive(found) => json!({
                "running": true,
                "responsive": false,
                "socket": found.at.socket,
                "lock_holder": found.holder,
            }),
            DaemonReport::Running { status, identity } => {
                let mut report = serde_json::to_value(status.as_ref()).unwrap_or(Value::Null);
                report["running"] = json!(true);
                report["compatible"] = json!(true);
                let found = identity.report();
                for key in IDENTITY_KEYS {
                    report[key] = found[key].clone();
                }
                report
            }
        }
    }
}

impl Serialize for DaemonReport {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.document().serialize(serializer)
    }
}

impl Frontend {
    /// C10, C45, C84, C86: the daemon of this world in one look — absent
    /// (with the ledger's counts and the last log), another daemon's
    /// identity, an unresponsive peer, or this client's daemon with its
    /// vitals. Never spawns or replaces.
    pub async fn daemon(&self) -> Result<DaemonReport, Fault> {
        let observed = acquisition_client::client::Client::observe_at(&self.root)
            .await
            .map_err(Fault::Transport)?;
        let mut client = match observed {
            acquisition_client::client::Observed::Compatible(client) => client,
            acquisition_client::client::Observed::Absent => {
                let persisted = self.persisted()?.map(|p| PersistedSummary {
                    waiting: p.waiting(),
                    recorded_running: p.recorded_running(),
                    written_at: p.written_at,
                });
                // Where the last daemon of this world logged, when it left
                // a log: the path this frontend's world derives, which is
                // the one a daemon it spawned used.
                let log = World::observe_at(&self.root)
                    .ok()
                    .map(|w| w.log_path(self.provider))
                    .filter(|p| p.exists());
                return Ok(DaemonReport::Absent { persisted, log });
            }
            acquisition_client::client::Observed::Incompatible(found) => {
                return Ok(DaemonReport::Incompatible(Box::new(found)));
            }
            acquisition_client::client::Observed::Unresponsive(found) => {
                return Ok(DaemonReport::Unresponsive(found));
            }
        };
        let identity = Box::new(client.daemon().clone());
        match client
            .request(&Request::DaemonStatus)
            .await
            .map_err(Fault::from_client)?
        {
            status @ Response::DaemonStatus { .. } => Ok(DaemonReport::Running {
                status: Box::new(status),
                identity,
            }),
            Response::Error { kind, message } => {
                Err(Fault::Daemon(acquisition_client::client::DaemonError {
                    kind,
                    message,
                }))
            }
            other => Err(unexpected(other)),
        }
    }
}
