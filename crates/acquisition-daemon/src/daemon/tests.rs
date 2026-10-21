//! The daemon's tests, one file per theme under `tests/`, and the harness
//! most of them share.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

mod bearer;
mod diagnostics;
mod dispatch;
mod harness;
mod idle;
mod login;
mod parents;
mod queue;
mod quote;
mod rails;
mod refresh;
mod responses;
mod sessions;
mod stress;
