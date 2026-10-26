//! The daemon's start: the world and its locks, the log, the knobs read at
//! start, the bound on the diagnostic files, and the accept loop.
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

/// C133: the terminating signals, SIGINT, SIGTERM and SIGHUP, run the
/// exit `daemon stop` runs; a second one ends the process at once, and
/// the bound's own thread ([`Exit::arm`]) ends it if the exit overruns.
/// Armed at the top of `run`, before the world, a lock or a keyring read
/// is taken: once a handler is registered the signal no longer kills the
/// process, so the path that ends it at once must exist before anything
/// can block. The watcher runs on a thread and a runtime of its own and
/// takes no daemon lock: a keyring prompt under `shared` can block every
/// worker of the daemon's runtime, and a watcher there would never run.
/// It arms the exit itself, without a lock, and hands the rest to the
/// daemon's runtime. The daemon is attached when it is built, before
/// restore; a signal before that ends the process at once, as it did
/// before the handlers, since nothing has run that a drain would wait
/// for. Registering replaces a disposition the parent set to ignored: a
/// script's background child, as the live drivers start `acqd`, inherits
/// SIGINT ignored, and `nohup` SIGHUP; each now runs the exit.
///
/// Windows: nothing is armed. A daemon spawned detached from its
/// frontend's console receives no console control event, and
/// `TerminateProcess`, how a host ends it there, cannot be caught.
pub(super) struct Signals {
    #[cfg(unix)]
    attached: Arc<OnceLock<Arc<Daemon>>>,
}

impl Signals {
    #[cfg(unix)]
    pub(super) fn arm() -> Result<Signals> {
        use tokio::signal::unix::{SignalKind, signal};
        let main = tokio::runtime::Handle::current();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        // Registered here, on the watcher's runtime, so the handlers are
        // in place before `arm` returns.
        let streams = {
            let _watcher = runtime.enter();
            Streams {
                int: signal(SignalKind::interrupt())?,
                term: signal(SignalKind::terminate())?,
                hup: signal(SignalKind::hangup())?,
            }
        };
        let attached = Arc::new(OnceLock::new());
        let watched = attached.clone();
        std::thread::Builder::new()
            .name("acqd-signals".into())
            .spawn(move || runtime.block_on(watch(streams, watched, main)))?;
        Ok(Signals { attached })
    }

    #[cfg(not(unix))]
    pub(super) fn arm() -> Result<Signals> {
        Ok(Signals {})
    }

    /// Hand the watcher the daemon.
    pub(super) fn attach(&self, daemon: &Arc<Daemon>) {
        #[cfg(unix)]
        let _ = self.attached.set(daemon.clone());
        #[cfg(not(unix))]
        let _ = daemon;
    }
}

#[cfg(unix)]
struct Streams {
    int: tokio::signal::unix::Signal,
    term: tokio::signal::unix::Signal,
    hup: tokio::signal::unix::Signal,
}

#[cfg(unix)]
impl Streams {
    /// The next signal: its name, and its number (POSIX fixes these three).
    async fn next(&mut self) -> (&'static str, i32) {
        tokio::select! {
            Some(()) = self.int.recv() => ("SIGINT", 2),
            Some(()) = self.term.recv() => ("SIGTERM", 15),
            Some(()) = self.hup.recv() => ("SIGHUP", 1),
            else => std::future::pending().await,
        }
    }
}

/// C133: the first signal arms the exit, without a lock, and runs the
/// rest of it on the daemon's runtime; the second ends the process at
/// once.
#[cfg(unix)]
async fn watch(
    mut streams: Streams,
    attached: Arc<OnceLock<Arc<Daemon>>>,
    main: tokio::runtime::Handle,
) {
    let (first, number) = streams.next().await;
    let Some(daemon) = attached.get() else {
        // A write whose failure is ignored: `eprintln!` panics on a broken
        // stderr, and this thread is the one that ends the process.
        let _ = writeln!(
            std::io::stderr(),
            "acqd: {first} before the daemon was built; exiting at once"
        );
        std::process::exit(128 + number);
    };
    let why = format!("{first} received; exiting");
    let exiting = daemon.clone();
    if daemon.exit.arm(&why) {
        main.spawn(async move {
            exiting.exit_armed(&why).await;
        });
    } else {
        main.spawn(async move {
            exiting.log(&format!("{first} received; the exit is already under way"));
        });
    }
    let (second, number) = streams.next().await;
    daemon.exit.end_at_once(
        &format!(
            "{second} received during the exit, a second signal: exiting at once, with no checkpoint"
        ),
        128 + number,
    )
}

/// C133: what ends the process at once, installed on the exit: one line
/// through a clone of the log's file (never the log's mutex), the socket
/// removed, the exit code. What it cuts is left running on disk for the
/// persisted queue (C6).
fn end_at_once(daemon: &Arc<Daemon>, log: std::fs::File) -> EndAtOnce {
    let daemon = Arc::downgrade(daemon);
    Arc::new(move |why: &str, code: i32| {
        if let Some(daemon) = daemon.upgrade() {
            let uptime = daemon.uptime_seconds();
            let _ = writeln!(&log, "[{uptime:>5}s] {why}");
            if let Some(socket) = &daemon.socket_path {
                socket.remove_if_still_ours();
            }
        }
        std::process::exit(code);
    })
}

/// C134: a panic is a broken internal invariant (C47), so it ends the
/// process instead of leaving a task dead and the daemon half alive. The
/// hook writes, in order: the fault marker naming the job the panic ran
/// in ([`RUNNING_JOB`]), when one is known and `marker` is given (a
/// daemon with a world); one line to the log through `log`, a clone of
/// the log's file; then the default hook's report to stderr; then
/// `end`, `std::process::abort` in `run`. It takes no daemon lock and
/// holds no `Daemon`: the panicking thread may hold any of them, and a
/// hook that waited on one would wedge the process instead of ending
/// it. The marker carries the job id alone, never a panic's words,
/// which may quote a body. A panic outside a job writes no marker: only
/// a job is replayed (C6), so only a job can bring the fault back.
pub(super) fn install_panic_hook(
    log: std::fs::File,
    marker: Option<PathBuf>,
    started: Instant,
    end: fn() -> !,
) {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let job = RUNNING_JOB.try_with(|id| *id).ok();
        let marked = match (job, &marker) {
            (Some(id), Some(path)) => match std::fs::write(path, format!("{{\"job\":{id}}}\n")) {
                Ok(()) => format!("; the fault marker {} names it", path.display()),
                Err(e) => format!(
                    "; the fault marker {} could not be written: {e}",
                    path.display()
                ),
            },
            _ => String::new(),
        };
        let place = info.location().map_or_else(
            || "an unknown place".to_string(),
            |at| format!("{}:{}", at.file(), at.line()),
        );
        let words = info.payload_as_str().unwrap_or("a panic with no message");
        let within = job.map_or_else(
            || "outside any job".to_string(),
            |id| format!("in job {id}"),
        );
        let uptime = started.elapsed().as_secs();
        let _ = writeln!(
            &log,
            "[{uptime:>5}s] PANIC {within} at {place}: {words}{marked}; the daemon aborts"
        );
        previous(info);
        end()
    }));
}

/// Run the daemon until stopped or idle-timed-out. Never returns Ok while the
/// socket is healthy; returns Err early if another daemon already owns the
/// world, the real-mode lock or the socket.
pub async fn run() -> Result<()> {
    // C133: the signals first, before the world, a lock or a keyring
    // read ([`Signals`]).
    let signals = Signals::arm()?;
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
    let result = run_with_log(world, locks, log_name, log.try_clone()?, signals).await;
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
    signals: Signals,
) -> Result<()> {
    // The locks live here, as long as the daemon does.
    let (_world_lock, real_mode_lock) = locks;
    let provider_name = acquisition_protocol::provider::wanted();
    // The rendezvous (C112): the world's endpoint, derived from the world
    // and the user. On Unix a socket in the private per-user runtime
    // directory, made here — the daemon is the one side that creates it;
    // a client only verifies it. On Windows the endpoint is a pipe and the
    // runtime directory would hold nothing (the real-mode lock is in the
    // user's data directory, C132), so it is not made.
    if cfg!(unix) {
        app_runtime_dir().map_err(|e| anyhow::anyhow!("the runtime directory: {e}"))?;
    }
    let path = world
        .socket_path()
        .map_err(|e| anyhow::anyhow!("this world's socket: {e}"))?;
    // The rails state lives in the world (C83), read strictly before the
    // rails are built: a state that cannot be carried into this lifetime
    // refuses the start (fail closed, review 2026-09-11).
    let rails_state = world.rails_state_path(provider_name);
    verify_state(&rails_state)?;

    let (listener, bound) = Listener::bind(&path).await?;

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

    // Sessions survive daemon restarts through the keyring (C144: read
    // through the credential store, as every keyring call is).
    let dir = world.provider_dir(provider.name);
    let credential_store: Arc<dyn CredentialStore> = Arc::new(OsCredentialStore);
    let sessions = restore_sessions(
        credential_store.as_ref(),
        provider.keyring_service,
        &dir,
        auth::keyring_disabled(),
        &mut &log,
    );

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
    let signal_log = log.try_clone()?;
    let log_for_panics = log.try_clone()?;
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
        credential_store,
        world: Some(world.name()),
        log_path: Some(log_path.clone()),
        socket_path: Some(bound),
        store_dir: Some(dir.clone()),
        store: Mutex::new(None),
        jobs_db,
        queue_failure: Mutex::new(None),
        exit: Exit::default(),
        #[cfg(test)]
        refresh_landed_hook: Mutex::new(None),
    });
    // C133: the exit's at-once end, then the signals: a signal from here
    // on runs the exit, before restore or any lock is taken.
    let _ = daemon.exit.end.set(end_at_once(&daemon, signal_log));
    signals.attach(&daemon);
    // C134: a panic from here on is logged, marks its job and aborts,
    // before restore reads the last lifetime's marker or anything sends.
    install_panic_hook(
        log_for_panics,
        daemon.fault_marker_path(),
        daemon.started,
        std::process::abort,
    );

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

    daemon.serve(listener, ACCEPT_PAUSE_CAP).await
}

/// C134: the longest the accept loop pauses after an error before it
/// accepts again; a run of errors doubles the pause up to it.
const ACCEPT_PAUSE_CAP: Duration = Duration::from_secs(1);

/// C134: the pause after the first error of a run.
const ACCEPT_PAUSE_FIRST: Duration = Duration::from_millis(10);

impl Daemon {
    /// C134, the accept loop: every connection gets its own task, and an
    /// error from `accept` never ends the daemon. Each kind of error is
    /// reported the first time, then at its 2nd, 4th, 8th… occurrence
    /// ([`AcceptErrors`]), since the log is bounded only at start and a
    /// daemon with live jobs never idles out; after each error the loop
    /// pauses, doubling from [`ACCEPT_PAUSE_FIRST`] to `pause_cap` over a
    /// run, so a lasting error (no descriptor left) is no busy loop. On
    /// Windows the endpoint has already replaced the broken pipe instance
    /// with a fresh one. A daemon no client can reach is ended by C3's
    /// idle exit. It never returns.
    pub(super) async fn serve(
        self: Arc<Self>,
        mut listener: impl Accept,
        pause_cap: Duration,
    ) -> ! {
        let mut errors = AcceptErrors::default();
        let mut run: u32 = 0;
        loop {
            match listener.accept().await {
                Ok(stream) => {
                    run = 0;
                    tokio::spawn(self.clone().handle_conn(stream));
                }
                Err(e) => {
                    if let Some(line) = errors.note(&e) {
                        self.note_error(&line);
                    }
                    let pause = ACCEPT_PAUSE_FIRST
                        .saturating_mul(1u32 << run.min(16))
                        .min(pause_cap);
                    run = run.saturating_add(1);
                    tokio::time::sleep(pause).await;
                }
            }
        }
    }
}

/// C134: how many times each kind of accept error has been seen this
/// lifetime, keyed on the OS error code where there is one.
#[derive(Default)]
struct AcceptErrors {
    seen: HashMap<(std::io::ErrorKind, Option<i32>), u64>,
}

impl AcceptErrors {
    /// Count `e`; the line to log when this occurrence is the first of
    /// its kind or a power of two, `None` otherwise.
    fn note(&mut self, e: &std::io::Error) -> Option<String> {
        let n = self.seen.entry((e.kind(), e.raw_os_error())).or_default();
        *n += 1;
        match *n {
            1 => Some(format!(
                "listener: accept failed: {e}; serving on (this error is logged again at its 2nd, 4th, 8th… occurrence)"
            )),
            n if n.is_power_of_two() => Some(format!(
                "listener: accept failed {n} times so far with this error: {e}; serving on"
            )),
            _ => None,
        }
    }
}

/// C144 (no keyring entry out of the index's reach): an account's refresh
/// token, read under its username and, when that read fails or finds
/// nothing, under each name the index holds for it in turn, so a rotated
/// token saved under a name a rename left behind is still reached. The
/// username's own result is the answer when no held name has one; a
/// token found under a held name is logged.
fn load_reached(
    store: &dyn CredentialStore,
    service: &str,
    entry: &acquisition_store::AccountEntry,
    log: &mut dyn std::io::Write,
) -> Result<Option<String>, String> {
    let first = store.load(service, &entry.username);
    if let Ok(Some(_)) = first {
        return first;
    }
    for name in entry.keyring_names.iter().filter(|n| **n != entry.username) {
        if let Ok(Some(token)) = store.load(service, name) {
            writeln!(
                log,
                "ACCOUNTS: {} restored from its keyring entry under the old name {name}",
                entry.username
            )
            .ok();
            return Ok(Some(token));
        }
    }
    first
}

/// The sessions a start restores: every account the index marks
/// persisted comes back as a live session from its credential-store entry
/// (C31), and one unreadable entry never blocks the rest. Every result of
/// a load reaches the words (C144): an entry the store does not hold, a
/// load that fails (the keyring status then says so, as `auth status`
/// reads it), an index that cannot be read, each a log line.
pub(super) fn restore_sessions(
    store: &dyn CredentialStore,
    service: &str,
    dir: &std::path::Path,
    disabled: Option<&str>,
    log: &mut dyn std::io::Write,
) -> Sessions {
    let mut sessions = Sessions {
        keyring: match disabled {
            Some(why) => format!("unavailable: {why}"),
            None => "ok".into(),
        },
        ..Sessions::default()
    };
    match Index::load(dir) {
        Ok(index) => {
            for entry in index.persisted() {
                match load_reached(store, service, entry, log) {
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
                            log,
                            "ACCOUNTS: index lists {} as persisted but the keyring has no entry",
                            entry.username
                        )
                        .ok();
                    }
                    Err(e) => {
                        sessions.keyring = format!("unavailable: {e}");
                        writeln!(
                            log,
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
            writeln!(log, "ACCOUNTS: could not read {}: {e:#}", dir.display()).ok();
        }
    }
    sessions
}
