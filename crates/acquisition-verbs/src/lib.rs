//! The verbs every frontend asks, built once (C46: every frontend is an
//! adapter): what needs the daemon and the store together, and what
//! every verb shares — who is asking, the account, the opener, the world.
//! The CLI and the MCP call these and render or serialise what they get
//! back; the GUI's routes will too. In `pricing.rs`'s image: typed
//! inputs, typed documents out, nothing printed, no account resolved
//! inside a verb.
//!
//! Rulings: `decisions/frontends.md`, `decisions/plans.md`,
//! `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s. C1
//! places the crate: it links the client, the protocol crate, the store,
//! the planner and the search, never the daemon, and nothing below it
//! links it (`tools/arch-check.sh`). A verb whose semantics is one
//! crate's stays in that crate — pricing in the planner (C75), the
//! search's verbs in the search crate; their openers come through
//! [`Account`].
//!
//! - [`fault`]: the one closed [`Fault`] every verb fails with, its wire
//!   kind and fields; each frontend words the remedy in one match.
//! - [`frontend`]: [`Frontend`] (the channel, the door, the world),
//!   [`Account`] (C121, resolved per verb) and [`Opened`] (the facts and
//!   intent), the doors (C10, C13) and the policy read and write (C52).
//! - [`refresh`]: compile, quote, apply and what landed (C38, C44, C111).
//! - [`jobs`]: submit, wait, and the queue's reads, live or from the
//!   ledger on disk (C45).
//! - [`daemon`]: the daemon's four observed states, one document.
//! - [`diagnose`]: one redacted document for a bug report, composed of
//!   the others (#79).
//! - [`events`]: the item events by location (C53).
//!
//! Every field of a value a verb takes is public, so a test builds one
//! over a fixture store with no environment.

// The lint ratchet (C47, "Panics are for broken internal invariants
// only"): malformed input from a store, a daemon or a caller is a
// structured fault, never a panic.
#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]

pub mod daemon;
pub mod diagnose;
pub mod events;
pub mod fault;
pub mod frontend;
pub mod jobs;
pub mod refresh;

pub use daemon::{DaemonReport, PersistedSummary};
pub use diagnose::{Diagnosis, Redactions, Tails};
pub use fault::Fault;
pub use frontend::{Account, AccountRow, Accounts, Door, Frontend, Opened, realm};
pub use jobs::{ChildTally, FailedChild, Job, JobResult, Jobs, Progress};
pub use refresh::{Apply, Landed, Planned, QuoteNote, Skipped, StoreChanges, Submitted};
