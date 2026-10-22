//! The daemon's start: the world and its locks, the log, the knobs read at
//! start, and the bound on the diagnostic files.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::*;

/// `ACQ_IDLE_SHUTDOWN=<secs>` overrides the idle exit (L0 rail 8) so a
/// live-test rung or soak runs on one daemon instead of a respawn cycle.
pub(super) fn idle_shutdown_from_env() -> Duration {
    std::env::var("ACQ_IDLE_SHUTDOWN")
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
        .map_or(IDLE_SHUTDOWN, Duration::from_secs)
}

/// The bound on each diagnostic file — the daemon log and the default
/// send journal (C83: "bounded diagnostics elsewhere"). Past it, the
/// file is rotated once at daemon start ([`rotate_if_over`]): the
/// current generation becomes `<name>.1`, replacing the previous one,
/// so a file never holds more than the cap plus one lifetime, and the
/// two generations together never more than about twice it. Sixteen
/// mebibytes: a full-league refresh is about 150 KB of either file, so a
/// season of daily refreshes fits in one generation, `tail` and an
/// editor open it instantly, and the previous generation is still there
/// when a rotation lands mid-investigation. A journal `ACQ_JOURNAL`
/// points elsewhere (a live run's evidence directory) is the caller's
/// file and is never rotated.
pub const DIAGNOSTIC_CAP_BYTES: u64 = 16 << 20;

/// Bound `path` at `cap` bytes: over it, rotate to `<path>.1`; when the
/// rename fails, truncate in place — the previous generation is lost,
/// the bound holds (review 2026-09-11: a failure that merely logged and
/// kept appending left the file unbounded). `Ok(Some)` says what was
/// done, for the log; `Ok(None)` when nothing was; `Err` when the file
/// could be neither rotated nor truncated, and the daemon must not
/// start over an unbounded diagnostic. Called only by the daemon that
/// holds the world lock, so an incumbent's live file is never moved from
/// under it.
pub fn rotate_if_over(path: &std::path::Path, cap: u64) -> Result<Option<String>, String> {
    let len = match std::fs::metadata(path) {
        Ok(meta) => meta.len(),
        Err(_) => return Ok(None),
    };
    if len <= cap {
        return Ok(None);
    }
    let mut previous = path.as_os_str().to_owned();
    previous.push(".1");
    let previous = PathBuf::from(previous);
    match std::fs::rename(path, &previous) {
        Ok(()) => Ok(Some(format!(
            "rotated {} ({len} bytes, over the {cap}-byte cap) to {}",
            path.display(),
            previous.display()
        ))),
        Err(rename) => match std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
        {
            Ok(_) => Ok(Some(format!(
                "truncated {} in place ({len} bytes, over the {cap}-byte cap; the previous generation is lost: it could not be renamed to {}: {rename})",
                path.display(),
                previous.display()
            ))),
            Err(truncate) => Err(format!(
                "{} is {len} bytes, over the {cap}-byte cap, and could be neither rotated ({rename}) nor truncated ({truncate})",
                path.display()
            )),
        },
    }
}

/// `ACQ_JOB_RETENTION_DAYS` / `ACQ_FAILED_JOB_RETENTION_DAYS`, each read
/// like a rails knob: absent means the default, a misread value is
/// reported (the second return) and the default stays.
fn retention_from_env() -> (Retention, Vec<String>) {
    let mut r = Retention::default();
    let mut problems = Vec::new();
    for (var, slot) in [
        ("ACQ_JOB_RETENTION_DAYS", &mut r.done_days),
        ("ACQ_FAILED_JOB_RETENTION_DAYS", &mut r.failed_days),
    ] {
        if let Ok(v) = std::env::var(var) {
            match v.trim().parse::<u32>() {
                Ok(days) => *slot = days,
                Err(_) => {
                    problems.push(format!("{var}={v:?} is not a number of days; using {slot}"))
                }
            }
        }
    }
    (r, problems)
}

/// Run the daemon until stopped or idle-timed-out. Never returns Ok while the
/// socket is healthy; returns Err early if another daemon already owns the
/// world, the real-mode lock or the socket.
pub async fn run() -> Result<()> {
    // The world first (C83): the use path creates the root and
    // canonicalises it — the frontend that spawned this daemon did the
    // same a moment ago, so this is the second look at an existing
    // directory. Without a world there is no log directory to write a
    // refusal to; stderr is all there is (a driver captures it).
    let world = World::create().map_err(|e| anyhow::anyhow!("{e}"))?;
    let provider_name = acquisition_protocol::provider::wanted();
    let log_path = world.log_path(provider_name);
    // The path crosses the wire exactly (`daemon_status`), so it must be
    // valid UTF-8; `ACQ_LOG_DIR` is the only way it would not be.
    let log_name = log_path
        .to_str()
        .ok_or_else(|| {
            anyhow::anyhow!(
                "the log directory is not valid UTF-8 (ACQ_LOG_DIR): {}",
                log_path.display()
            )
        })?
        .to_string();
    // The locks first (review 2026-09-11): nothing in the log directory
    // is made, rotated or opened until this daemon holds the world, so a
    // contender that loses cannot rotate the incumbent's live log or
    // rewrite its marker. A refused contender appends its refusal to
    // the log as it stands, creating nothing — a lazy-spawned daemon's
    // stderr goes to null and the log is where the CLI reads a spawn
    // failure from; with no log there yet, stderr is all there is.
    let locks = match take_locks(&world) {
        Ok(locks) => locks,
        Err(e) => {
            // Appended to a log that exists; never created (review
            // 2026-09-11, round 21: a contender under another log
            // directory must make no file there either).
            if let Ok(log) = std::fs::OpenOptions::new().append(true).open(&log_path) {
                writeln!(&log, "STARTUP: {e:#}").ok();
            }
            return Err(e);
        }
    };
    // Holding the world: the log directory and its marker, then the
    // bound — the log is rotated here, before it opens, when the last
    // lifetime left it over the cap.
    if let Some(dir) = log_path.parent() {
        std::fs::create_dir_all(dir)
            .map_err(|e| anyhow::anyhow!("creating the log directory {}: {e}", dir.display()))?;
    }
    let _ = std::fs::write(world.log_marker_path(), format!("{}\n", world.name()));
    let rotated = match rotate_if_over(&log_path, DIAGNOSTIC_CAP_BYTES) {
        Ok(rotated) => rotated,
        Err(why) => {
            let e = anyhow::anyhow!("the log cannot be bounded: {why}");
            if let Ok(log) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_path)
            {
                writeln!(&log, "STARTUP: {e:#}").ok();
            }
            return Err(e);
        }
    };
    let log = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .map_err(|e| anyhow::anyhow!("opening the log {}: {e}", log_path.display()))?;
    if let Some(what) = rotated {
        writeln!(&log, "log: {what}").ok();
    }
    let result = run_with_log(world, locks, log_name, log.try_clone()?).await;
    if let Err(e) = &result {
        writeln!(&log, "STARTUP: {e:#}").ok();
    }
    result
}

/// The world lock and, in real mode, the real-mode lock (C83), taken
/// before the log directory is made, the log rotated or opened, or
/// anything sends; held for the daemon's lifetime by the caller,
/// released by the kernel however the process ends. A
/// second daemon on this root, or a second real-mode daemon for this OS
/// user whatever its root, refuses here naming the holder.
fn take_locks(world: &World) -> Result<(Lock, Option<Lock>)> {
    let world_lock = Lock::acquire(&world.lock_path()).map_err(|e| {
        anyhow::anyhow!(
            "another daemon holds this world: {e} — one daemon per world; `acq daemon status` or `acq daemon stop` against its socket"
        )
    })?;
    // Per OS user, whatever the root: two live-test roots cannot make
    // two GGG gates (C31).
    let real_mode_lock = if ggg_mode() {
        let path = acquisition_store::world::real_mode_lock_path()
            .map_err(|e| anyhow::anyhow!("resolving the real-mode lock: {e}"))?;
        Some(Lock::acquire(&path).map_err(|e| {
            anyhow::anyhow!(
                "another real-mode daemon is running for this OS user: {e} — one real-mode daemon per OS user, whatever its world; `acq daemon stop` it first"
            )
        })?)
    } else {
        None
    };
    Ok((world_lock, real_mode_lock))
}

async fn run_with_log(
    world: World,
    locks: (Lock, Option<Lock>),
    log_path: String,
    log: std::fs::File,
) -> Result<()> {
    // The locks live here, as long as the daemon does.
    let (_world_lock, real_mode_lock) = locks;
    let provider_name = acquisition_protocol::provider::wanted();
    // The rendezvous (C112): the world's endpoint, derived from the world
    // and the user. On Unix a socket in the private per-user runtime
    // directory, made here — the daemon is the one side that creates it;
    // a client only verifies it. On Windows the directory holds only the
    // real-mode lock and the endpoint is a pipe.
    app_runtime_dir().map_err(|e| anyhow::anyhow!("the runtime directory: {e}"))?;
    let path = world
        .socket_path()
        .map_err(|e| anyhow::anyhow!("this world's socket: {e}"))?;
    // The rails state lives in the world (C83), read strictly before the
    // rails are built: a state that cannot be carried into this lifetime
    // refuses the start (fail closed, review 2026-09-11).
    let rails_state = world.rails_state_path(provider_name);
    verify_state(&rails_state)?;

    let (mut listener, bound) = Listener::bind(&path).await?;

    // Who this daemon is (C84): the executable it runs from, identified
    // and hashed once, before the journal opens — its header names the
    // hash — and before anything can send. A daemon that cannot read its
    // own file does not start: no client could use it.
    let artifact = crate::artifact::of_current_exe()?;

    // Real GGG by default (C88); under `ACQ_PROVIDER=mock` the in-process
    // mock, which in real mode is never even started.
    let provider = if ggg_mode() {
        Provider::ggg()
    } else {
        Provider::mock(&mockggg::start().await?)
    };
    // Same limiter in both modes: empty until responses teach it policies.
    let clock: Arc<dyn Clock> = Arc::new(SystemClock);
    // The default journal is bounded like the log; one `ACQ_JOURNAL`
    // names is the caller's and is left alone.
    let default_journal = world.journal_path(provider.name);
    let journal_rotated = if std::env::var_os("ACQ_JOURNAL").is_none() {
        rotate_if_over(&default_journal, DIAGNOSTIC_CAP_BYTES)
            .map_err(|why| anyhow::anyhow!("the journal cannot be bounded: {why}"))?
    } else {
        None
    };
    let mut rails_config = RailsConfig::from_env(&rails_state, &default_journal);
    rails_config.daemon_artifact = Some(artifact.sha256.clone());
    let rails = Arc::new(Rails::with_config_and_clock(rails_config, clock.clone()));
    let choke = ChokePoint::with_clock_and_rails(clock, rails, &provider.api_base);
    Daemon::declare_route_knowledge(&choke);
    let now = choke.now();

    // Sessions survive daemon restarts through the keyring, one entry per
    // account; the account index says which entries to look for, and every
    // persisted account comes back as a live session (C31). One unreadable
    // entry never blocks the rest.
    let dir = world.provider_dir(provider.name);
    let mut sessions = Sessions {
        keyring: match auth::keyring_disabled() {
            Some(why) => format!("unavailable: {why}"),
            None => "ok".into(),
        },
        ..Sessions::default()
    };
    match Index::load(&dir) {
        Ok(index) => {
            for entry in index.persisted() {
                match auth::keyring_load(provider.keyring_service, &entry.username) {
                    Ok(Some(refresh_token)) => {
                        sessions.replace(AuthSession {
                            refresh_token: Some(refresh_token),
                            username: Some(entry.username.clone()),
                            uuid: entry.uuid.clone(),
                            keyring: "ok".into(),
                            ..AuthSession::default()
                        });
                    }
                    Ok(None) => {
                        writeln!(
                            &log,
                            "ACCOUNTS: index lists {} as persisted but the keyring has no entry",
                            entry.username
                        )
                        .ok();
                    }
                    Err(e) => {
                        sessions.keyring = format!("unavailable: {e}");
                        writeln!(
                            &log,
                            "ACCOUNTS: keyring read for {} failed: {e}",
                            entry.username
                        )
                        .ok();
                    }
                }
            }
            // `persisted()` is newest first; `replace` marked the last one
            // inserted as the most recent login, which is the oldest.
            sessions.last_login = index.persisted().first().map(|e| e.username.clone());
        }
        Err(e) => {
            writeln!(&log, "ACCOUNTS: could not read {}: {e:#}", dir.display()).ok();
        }
    }

    // A daemon that cannot read its queue must not run: it would restart
    // ids at 1 and reuse them against the table's history the moment the
    // file comes back. Refusing to start is the safe failure; the log
    // names the file to repair or remove.
    let jobs_db = match JobDb::open(&acquisition_store::jobs::daemon_db_path(&dir)) {
        Ok(db) => Mutex::new(db),
        Err(e) => {
            anyhow::bail!(
                "could not open {}: {e:#} (repair or remove it; the daemon will not run without its queue)",
                acquisition_store::jobs::daemon_db_path(&dir).display()
            );
        }
    };
    let daemon = Arc::new(Daemon {
        shared: Mutex::new(Shared {
            jobs: HashMap::new(),
            next_id: 1,
            auth: sessions,
            connections: 0,
            last_activity: now,
            errors: VecDeque::new(),
            active_jobs: HashMap::new(),
        }),
        started: now,
        events: broadcast::channel(256).0,
        work: Notify::new(),
        log: Mutex::new(log),
        choke,
        provider,
        artifact: Some(artifact),
        credential_store: Arc::new(OsCredentialStore),
        world: Some(world.name()),
        log_path: Some(log_path.clone()),
        socket_path: Some(bound),
        store_dir: Some(dir.clone()),
        store: Mutex::new(None),
        jobs_db,
        queue_failure: Mutex::new(None),
        exit: Exit::default(),
    });

    {
        let artifact = daemon.artifact.as_ref().expect("run sets the artifact");
        daemon.log(&format!(
            "daemon {} contract {} artifact {} ({}) listening on {} (pid {})",
            acquisition_protocol::VERSION,
            acquisition_protocol::CONTRACT_REVISION,
            artifact.sha256,
            artifact.file.path,
            path.display(),
            std::process::id()
        ));
        daemon.log(&format!(
            "world: {} (lock {}{}) | log: {} | rails state: {}",
            world.name(),
            world.lock_path().display(),
            match &real_mode_lock {
                Some(lock) => format!("; real-mode lock {}", lock.path().display()),
                None => String::new(),
            },
            log_path,
            rails_state.display(),
        ));
        if let Some(what) = journal_rotated {
            daemon.log(&format!("journal: {what}"));
        }
    }
    let (keyring, username) = {
        let s = daemon.shared.lock().unwrap();
        let names = s.auth.usernames();
        (
            daemon.keyring_summary(&s),
            (!names.is_empty()).then(|| names.join(", ")),
        )
    };
    let provider_desc = if daemon.provider.is_real() {
        "ggg (REAL GGG — requests leave this machine)".to_string()
    } else {
        format!("mock at {}", daemon.provider.api_base)
    };
    daemon.log(&format!(
        "provider: {provider_desc} | keyring: {keyring} | session: {}",
        username.as_deref().unwrap_or("none"),
    ));
    let rails = daemon.rails().status();
    daemon.log(&format!(
        "rails: tripwire {} | halted: {} | refresh-failed: {} | ceiling: {} | journal: {} | idle exit after {}s",
        if rails.tripwire_enabled { "ON" } else { "off" },
        rails.halted.as_deref().unwrap_or("no"),
        rails.refresh_failed.as_deref().unwrap_or("no"),
        rails.max_sends.map_or("none".to_string(), |n| n.to_string()),
        rails.journal.as_deref().unwrap_or("off"),
        idle_shutdown_from_env().as_secs(),
    ));
    for warning in daemon.rails().startup_warnings() {
        daemon.note_error(&format!("RAILS CONFIG: {warning}"));
    }
    let (retention, problems) = retention_from_env();
    for problem in problems {
        daemon.note_error(&format!("JOBS CONFIG: {problem}"));
    }
    if let Err(e) = daemon.restore_jobs(retention) {
        anyhow::bail!(
            "restore of the persisted queue failed: {e:#} (repair or remove it; the daemon will not run without its queue)"
        );
    }

    tokio::spawn(daemon.clone().dispatcher());
    tokio::spawn(daemon.clone().idle_watchdog());

    loop {
        let stream = listener.accept().await?;
        tokio::spawn(daemon.clone().handle_conn(stream));
    }
}
