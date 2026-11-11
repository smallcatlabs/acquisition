//! `acq` — the command-line frontend: clap and rendering over the
//! daemon protocol and the store's read API (C12).
//!
//! This file is the entry and its wiring: the command parsed, run, and
//! its failure said. `cli.rs` holds the command tree, `run.rs` the
//! dispatch, `terminal.rs` how a failure is said; each verb's module
//! renders its own answers.
//!
//! Rulings: `decisions/frontends.md`; the cross-cutting ones are `AGENTS.md`'s.

mod auth_cmd;
mod cli;
mod daemon_cmd;
mod dash;
mod diagnose_cmd;
mod jobs_cmd;
mod leagues_cmd;
mod plan_cmd;
mod price_cmd;
mod printed;
#[cfg(test)]
mod real_scale_fixture;
mod reference_cmd;
mod run;
mod saved_cmd;
mod search_cmd;
mod shop_cmd;
mod store_cmd;
mod terminal;
mod web;

use acquisition_client::client::{Client, DaemonError};
use acquisition_protocol::protocol::ErrorKind;
use acquisition_verbs::Fault;
use clap::Parser;

use crate::cli::Cli;
use crate::run::run;
use crate::terminal::{
    AlreadyReported, Terminal, error_report, failure_words, pasted_shell_hint, ready_accounts,
    ready_ways, refuse_usage,
};

/// Remember that this command started the daemon it talks to (C3).
pub(crate) fn note_spawn(client: &Client) {
    if client.spawned() {
        SPAWNED.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

/// Set when this command started the daemon it then talked to, so a
/// refusal for want of a login can say a daemon was started (C3 kept
/// after the release seat: cli-opus calls 100 to 101).
static SPAWNED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// The refusal of a job verb that started a daemon and was then refused
/// for want of a login, saying so: the daemon stays until idle, and the
/// line names what shows and what ends it. The kind is kept for the JSON
/// document.
fn noting_the_spawn(e: anyhow::Error) -> anyhow::Error {
    if !SPAWNED.load(std::sync::atomic::Ordering::Relaxed) {
        return e;
    }
    let refusal = match e.chain().find_map(|c| c.downcast_ref::<Fault>()) {
        Some(Fault::Daemon(refusal)) => Some(refusal.clone()),
        Some(_) => None,
        None => DaemonError::find(&e).cloned(),
    };
    match refusal {
        Some(refusal) if refusal.kind == ErrorKind::NotLoggedIn => {
            anyhow::Error::from(Fault::Daemon(DaemonError {
                kind: refusal.kind,
                message: format!(
                    "{}; a daemon was started for this command and runs until idle — \
                     `acq daemon status` shows it, `acq daemon stop` ends it",
                    failure_words(&e)
                ),
            }))
        }
        _ => e,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::try_parse().unwrap_or_else(|e| refuse_usage(e));
    let json = cli.json;
    if let Err(e) = run(cli).await.map_err(noting_the_spawn) {
        if e.downcast_ref::<AlreadyReported>().is_none() {
            if json {
                println!("{}", error_report(&e));
            } else {
                eprintln!("Error: {}", failure_words(&e));
                eprint!("{}", ready_accounts(&e, &Terminal::this_command()));
                eprint!("{}", ready_ways(&e, &Terminal::this_command()));
            }
        }
        if !json && let Some(hint) = pasted_shell_hint() {
            eprintln!("{hint}");
        }
        std::process::exit(1);
    }
}
