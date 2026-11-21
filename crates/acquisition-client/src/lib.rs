//! The client side of the daemon protocol (C1, C12's first surface as a
//! frontend reaches it): the IPC over tokio and the three policy doors of
//! C10 ([`client`]), the `acqd` locator ([`locator`], C82), the artifact
//! comparison against that sibling ([`artifact`], C84), the world
//! comparison against this process's own (C83; the world itself is the
//! store's `world.rs`), the bounded frame reader ([`frame`]) and the
//! typed failures at the door ([`client::ConnectError`]). Every frontend
//! links this crate and the protocol crate; this crate links the protocol
//! crate, the store and tokio, never the daemon (`acquisition-daemon`) or
//! the planner, and `tools/arch-check.sh` refuses both edges.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.
//!
//! A crate of its own because a client or locator edit must not recompile
//! the daemon, and the GUI and the MCP server need typed failures rather
//! than strings. It links the store for the world: the socket, the log and
//! the root every side reads are one definition there
//! (`acquisition_store::world`), so no second copy of the socket convention
//! stands.

pub mod artifact;
pub mod client;
mod door;
pub mod endpoint;
pub mod frame;
#[cfg(test)]
#[path = "tests/generated.rs"]
mod generated;
pub mod locator;
