//! `daemon status`'s text and the rails' (the tripwire's), the daemon
//! log's path, and the remedy for a daemon of another build.
//!
//! Rulings: `decisions/frontends.md`; the cross-cutting ones are `AGENTS.md`'s.

use acquisition_client::client::Client;
use acquisition_protocol::protocol::{Request, Response};
use acquisition_verbs::DaemonReport;

/// What a human does about a daemon this client will not use: a
/// mismatch of contract, artifact or provider is replaced by a job
/// command; a daemon on another world (C83) is never replaced — stop it,
/// or point this shell at its world.
pub(crate) fn mismatch_remedy(found: &acquisition_client::client::DaemonId) -> &'static str {
    if found.world_matches() {
        "`acq daemon stop` stops it, a job command (`acq profile`, `acq refresh --apply`) replaces it"
    } else {
        "a job command from this shell refuses it too (another world is never replaced) — `acq daemon stop` stops it, or point ACQ_STORE_DIR at its world"
    }
}

/// The log the running daemon opened, as it reports it (`daemon_status`,
/// C83) — never recomputed from this shell's environment, which may
/// differ from the daemon's.
pub(crate) async fn daemon_log_path(client: &mut Client) -> Option<String> {
    match client.request(&Request::DaemonStatus {}).await {
        Ok(Response::DaemonStatus { log, .. }) => log,
        _ => None,
    }
}

/// `daemon status`'s text: the daemon of this world in one of its four
/// observed states (C10, C45, C84, C86), as `acq daemon status` prints it
/// and as `acq diagnose` carries it.
pub(crate) fn daemon_text(report: &DaemonReport) -> String {
    let mut out = String::new();
    macro_rules! line {
        ($($arg:tt)*) => {{
            out.push_str(&format!($($arg)*));
            out.push('\n');
        }};
    }
    match report {
        DaemonReport::Absent { persisted, log } => {
            match persisted {
                None => line!("daemon is not running; no queue on disk"),
                Some(p) => line!(
                    "daemon is not running; on disk: {} waiting, {} recorded running, written {}",
                    p.waiting,
                    p.recorded_running,
                    acquisition_store::age::ago(acquisition_store::now(), p.written_at),
                ),
            }
            if let Some(log) = log {
                line!("last log: {}", log.display());
            }
        }
        DaemonReport::Incompatible(found) => {
            line!("{found} — running, not this client's");
            line!("socket: {}", found.socket());
            line!("next:   {}", mismatch_remedy(found));
        }
        // Something accepted and never answered (C86): a
        // state, like the others — not this verb's error.
        DaemonReport::Unresponsive(found) => line!("{found}"),
        DaemonReport::Running { status, identity } => {
            let found = identity;
            if let Response::DaemonStatus {
                pid,
                version,
                contract,
                provider,
                uptime_seconds,
                idle_shutdown_seconds,
                connections,
                jobs_waiting,
                jobs_running,
                policies_known,
                in_flight,
                max_in_flight,
                rails,
                keyring,
                log,
            } = status.as_ref()
            {
                line!(
                    "daemon {version} (contract {contract}) pid {pid}, up {uptime_seconds}s, provider {provider}, idle exit after {idle_shutdown_seconds}s"
                );
                line!("world:  {}", found.world());
                match (found.artifact(), &found.verdict().artifact) {
                    (
                        Some(a),
                        acquisition_client::artifact::ArtifactVerdict::SameBytes { sibling },
                    ) => {
                        line!(
                            "acqd:   {} (sha256 {}) — the same artifact as this client's sibling {}, another copy",
                            a.file.path,
                            a.short_hash(),
                            sibling.path
                        )
                    }
                    (Some(a), _) => line!(
                        "acqd:   {} (sha256 {}) — this client's sibling, the file a job command would start",
                        a.file.path,
                        a.short_hash()
                    ),
                    (None, _) => line!("acqd:   not reported"),
                }
                line!(
                    "connections: {connections}  waiting: {jobs_waiting}  running: {jobs_running}  in flight: {in_flight}/{max_in_flight}  policies learned: {policies_known} since this daemon started"
                );
                line!("socket: {}", found.socket());
                line!("log:    {}", log.as_deref().unwrap_or("not reported"));
                out.push_str(&rails_text(rails, acquisition_store::now()));
                if keyring != "ok" {
                    line!("KEYRING: {keyring} — a rotated refresh token may be memory-only");
                }
                if let Some(cause) = &rails.refresh_failed {
                    line!("REFRESH DISABLED: {cause}");
                    line!("        re-login with `acq auth`");
                }
            }
        }
    }
    out
}

/// The daemon's rails as `daemon status` prints them: the tripwire, the
/// sends against their ceiling, this daemon's lifetime's (C146: the
/// limiter and the count are never persisted, while the journal beside
/// them spans lifetimes), and the journal, then a halt with its
/// cause and what clears it. A journal that was not written is
/// named by its path, the status's own field, before the daemon's words
/// of why, which name none (C152).
pub(crate) fn rails_text(rails: &acquisition_protocol::status::RailsStatus, now: i64) -> String {
    let journal = match (&rails.journal_path, &rails.journal) {
        (Some(path), Some(words)) if words != path => format!("{path} {words}"),
        (_, Some(words)) => words.clone(),
        (_, None) => "off".to_string(),
    };
    let mut out = format!(
        "rails:  tripwire {} · sends {}{} since this daemon started · journal {journal}\n",
        if rails.tripwire_enabled { "ON" } else { "off" },
        rails.sends,
        rails.max_sends.map_or(String::new(), |m| format!("/{m}")),
    );
    out.push_str(&closed_routes_text(&rails.closed_routes, now));
    let Some(cause) = &rails.halted else {
        return out;
    };
    let mut line = |text: &str| {
        out.push_str(text);
        out.push('\n');
    };
    line(&format!("HALTED: {cause}"));
    // A ceiling is this lifetime's and never persisted (rails.rs): no
    // violation, nothing to reset — the next daemon starts under a fresh
    // one. Met beside another halt, the reset lifts that one only, so the
    // advice names the ceiling too.
    let ceiling_too =
        !cause.starts_with("ceiling:") && rails.max_sends.is_some_and(|max| rails.sends >= max);
    if cause.starts_with("Cloudflare answered") {
        line(&format!(
            "        every daemon holds until `acq daemon reset-tripwire`; read the cause first{}",
            if ceiling_too {
                ""
            } else {
                ": the next send goes to GGG"
            }
        ));
    } else if cause.starts_with("Cloudflare could not reach GGG") {
        if ceiling_too {
            line("        this daemon's alone, never persisted: a restart clears it");
        } else {
            line(
                "        this daemon's alone, never persisted: `acq daemon reset-tripwire` or a restart clears it; the next send goes to GGG",
            );
        }
    } else if cause.starts_with("ceiling:") {
        line(
            "        the ceiling is this daemon's alone: `acq daemon stop`, and the next one starts fresh (`acq jobs` first: queued jobs resume under it)",
        );
    } else {
        line("        clear with `acq daemon reset-tripwire` after the post-violation wait");
    }
    if ceiling_too {
        line(
            "        the ceiling is met too, and a reset does not lift it: only the next daemon starts under a fresh one (`acq daemon stop`; `acq jobs` first: queued jobs resume under it)",
        );
    }
    out
}

/// C190: each route a run of invalid answers closed, as `daemon status`
/// says it from the status's facts: what closed
/// it, that every request on it waits whatever its league, when the trial
/// goes, what to check when it stands at the cap, and to list again once
/// it reopens.
fn closed_routes_text(closed: &[acquisition_protocol::status::ClosedRoute], now: i64) -> String {
    let mut out = String::new();
    for route in closed {
        out.push_str(&format!(
            "ROUTE CLOSED: {}\n",
            closed_route_words(route, now)
        ));
        out.push_str(
            "        every request on it waits, whatever its league; once it reopens, list the league again: what was fetched just before may be older than GGG's state\n",
        );
        if route.cooldown_secs >= CAP_SECS {
            out.push_str(
                "        at the cap: one trial an hour while jobs wait; a route that keeps failing may mean GGG's API changed — check whether a newer acq exists\n",
            );
        }
    }
    out
}

/// C190: the cooldown a closure stays at (the daemon's cap).
const CAP_SECS: u64 = 3600;

/// C190: one closed route in words, at the answer's `now`: the one
/// renderer `daemon status` and a progress line share; the web jobs page
/// says it in its own words.
pub(crate) fn closed_route_words(
    route: &acquisition_protocol::status::ClosedRoute,
    now: i64,
) -> String {
    let trial = if route.until > now {
        format!(
            "one waiting job goes as the trial in ~{}",
            acquisition_store::age::span(route.until - now)
        )
    } else {
        "the next job sent on it is the trial".to_string()
    };
    format!(
        "{} for {}s — GGG answered {} requests in a row on it as invalid, the last {}; {trial}",
        route.route, route.cooldown_secs, route.invalid, route.status
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// C190: a timed closure says the trial's time from the status's
    /// facts at the answer's `now`, and that every league's requests on the
    /// route wait: the run was one league's, the route is the account's.
    /// `daemon status` says it in a halted world beside the jobs page
    /// (`the_jobs_page_opens_with_the_halt`).
    #[test]
    fn c190_a_closed_route_says_when_its_trial_goes() {
        use acquisition_protocol::status::{ClosedRoute, RailsStatus};
        let rails = RailsStatus {
            closed_routes: vec![ClosedRoute {
                route: "stash@A#1".into(),
                invalid: 6,
                status: 404,
                closed_at: 1_800_000_000,
                cooldown_secs: 120,
                until: 1_800_000_120,
            }],
            ..RailsStatus::default()
        };
        let text = rails_text(&rails, 1_800_000_080);
        assert!(
            text.contains("ROUTE CLOSED: stash@A#1 for 120s — GGG answered 6 requests in a row on it as invalid, the last 404; one waiting job goes as the trial in ~40s\n"),
            "{text}"
        );
        assert!(text.contains("whatever its league"), "{text}");
        let due = rails_text(&rails, 1_800_000_120);
        assert!(
            due.contains("the next job sent on it is the trial"),
            "{due}"
        );
        assert!(!text.contains("at the cap"), "{text}");
        // At the cap, the words say what a route that keeps failing
        // may mean, from the status's fields alone, and no verb is owed.
        let mut capped = rails.clone();
        capped.closed_routes[0].cooldown_secs = 3600;
        let at_cap = rails_text(&capped, 1_800_000_080);
        assert!(
            at_cap.contains("at the cap: one trial an hour while jobs wait")
                && at_cap.contains("GGG's API changed"),
            "{at_cap}"
        );
    }

    /// C146 (a count says what it counts): the sends and the policies
    /// learned are this daemon's lifetime's, never persisted, said beside
    /// a journal path that spans lifetimes (unsaid, `sends 17` reads as the
    /// day's count).
    #[test]
    fn c146_daemon_status_says_its_counts_are_this_daemons_lifetimes() {
        use acquisition_client::client::{DaemonId, Endpoint, Reported};
        use acquisition_protocol::status::RailsStatus;
        let rails = RailsStatus {
            tripwire_enabled: false,
            halted: None,
            refresh_failed: None,
            sends: 17,
            max_sends: None,
            journal: Some("/j/journal.jsonl".into()),
            journal_path: Some("/j/journal.jsonl".into()),
            closed_routes: vec![],
        };
        let status = Response::DaemonStatus {
            pid: 4242,
            version: "0.0.1".into(),
            contract: "ffffffffffff".into(),
            provider: "mock".into(),
            uptime_seconds: 61,
            idle_shutdown_seconds: 3600,
            connections: 1,
            jobs_waiting: 0,
            jobs_running: 0,
            policies_known: 3,
            in_flight: 0,
            max_in_flight: 2,
            rails,
            keyring: "ok".into(),
            log: None,
        };
        let identity = DaemonId::judged(
            Reported {
                pid: 4242,
                version: "0.0.1".into(),
                contract: "ffffffffffff".into(),
                artifact: None,
                provider: "mock".into(),
                world: "/w".into(),
            },
            Endpoint {
                socket: "/run/acq/w.sock".into(),
            },
            Ok("/w".into()),
        );
        let text = daemon_text(&DaemonReport::Running {
            status: Box::new(status),
            identity: Box::new(identity),
        });
        assert!(
            text.contains("  policies learned: 3 since this daemon started\n"),
            "{text}"
        );
        assert!(
            text.contains(
                "rails:  tripwire off · sends 17 since this daemon started · journal /j/journal.jsonl\n"
            ),
            "{text}"
        );
    }
}
