//! The harness the daemon's scenario tests share: the clocks, the
//! credential stores, the scripted servers, the test daemons, the journal
//! readers and the four journal oracles.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use crate::daemon::life::{Effect, Event as LifeEvent, Life};
use crate::daemon::server::{InFlight, Leave};
use crate::daemon::*;
/// The check of a send no test cancels (C25).
pub(super) use crate::ratelimit::ALWAYS_WANTED;
use crate::ratelimit::Clock;
use acquisition_transport::Transport;
use std::collections::HashSet;
use std::future::Future;
use tokio::sync::oneshot;

pub(super) use crate::testing::{HANG, MANUAL_WALL_ORIGIN, ParkingClock};
pub(super) use crate::transport::{
    BASE, Ends, Hold, InProcess, Received, ScriptedResponse, Sent, TOKEN_HEADERS, UNREAD_ACCOUNT,
};

pub(super) struct NoopCredentialStore;

impl CredentialStore for NoopCredentialStore {
    fn save(&self, _service: &str, _secret: &auth::Secret, _username: &str) -> Result<(), String> {
        Ok(())
    }

    fn load(&self, _service: &str, _username: &str) -> Result<Option<auth::Secret>, String> {
        Ok(None)
    }

    fn clear(&self, _service: &str, _username: &str) -> Result<(), auth::NotCleared> {
        Ok(())
    }
}

/// An in-memory keyring that records every call and can be told to fail
/// any of the three (C144: every result reaches the caller). A save
/// records its attempt and, when it succeeds, holds the entry; a clear
/// records its attempt and, when it succeeds, removes it. It keeps each
/// secret as its text, for a test to assert on (#113: the text is read
/// through the test-only door).
#[derive(Default)]
pub(super) struct RecordingCredentialStore {
    pub(super) saves: Mutex<Vec<(String, String, String)>>,
    pub(super) cleared: Mutex<Vec<String>>,
    /// The entries a load reads, by username.
    pub(super) entries: Mutex<HashMap<String, String>>,
    pub(super) fail_save: Mutex<Option<String>>,
    pub(super) fail_load: Mutex<Option<String>>,
    /// A load under one of these names fails with its reason; a load
    /// under any other name reads as usual.
    pub(super) fail_load_under: Mutex<HashMap<String, String>>,
    pub(super) fail_clear: Mutex<Option<String>>,
    /// The keyring turned off, as `ACQ_NO_KEYRING` turns it off: a clear
    /// tries nothing and says why.
    pub(super) keyring_off: Mutex<Option<String>>,
    /// Run once at the next clear, before it acts: a test's pause where
    /// a clear is under way.
    pub(super) on_clear: Mutex<Option<Box<dyn FnOnce() + Send>>>,
    /// Run once at the next save, before it acts: a test's pause where a
    /// save is under way, as a keychain prompt holds one (C148).
    pub(super) on_save: Mutex<Option<Box<dyn FnOnce() + Send>>>,
    /// Run once at the next load, before it acts: a test's pause where
    /// the start's restore reads the keyring, as a keychain prompt holds
    /// it (C148).
    pub(super) on_load: Mutex<Option<Box<dyn FnOnce() + Send>>>,
}

impl RecordingCredentialStore {
    /// A store whose every `op` (`save`, `load`, `clear`) fails with `why`.
    pub(super) fn failing(op: &str, why: &str) -> Self {
        let store = Self::default();
        let slot = match op {
            "save" => &store.fail_save,
            "load" => &store.fail_load,
            "clear" => &store.fail_clear,
            other => panic!("no such credential-store call: {other}"),
        };
        *slot.lock().unwrap() = Some(why.to_string());
        store
    }
}

impl CredentialStore for RecordingCredentialStore {
    fn save(&self, service: &str, secret: &auth::Secret, username: &str) -> Result<(), String> {
        let secret = secret.exposed_for_test();
        let hook = self.on_save.lock().unwrap().take();
        if let Some(hook) = hook {
            hook();
        }
        self.saves.lock().unwrap().push((
            service.to_string(),
            secret.to_string(),
            username.to_string(),
        ));
        if let Some(why) = self.fail_save.lock().unwrap().clone() {
            return Err(why);
        }
        self.entries
            .lock()
            .unwrap()
            .insert(username.to_string(), secret.to_string());
        Ok(())
    }

    fn load(&self, _service: &str, username: &str) -> Result<Option<auth::Secret>, String> {
        let hook = self.on_load.lock().unwrap().take();
        if let Some(hook) = hook {
            hook();
        }
        if let Some(why) = self.fail_load.lock().unwrap().clone() {
            return Err(why);
        }
        if let Some(why) = self.fail_load_under.lock().unwrap().get(username) {
            return Err(why.clone());
        }
        Ok(self
            .entries
            .lock()
            .unwrap()
            .get(username)
            .map(|text| text.as_str().into()))
    }

    fn clear(&self, _service: &str, username: &str) -> Result<(), auth::NotCleared> {
        let hook = self.on_clear.lock().unwrap().take();
        if let Some(hook) = hook {
            hook();
        }
        if let Some(why) = self.keyring_off.lock().unwrap().clone() {
            return Err(auth::NotCleared::Off(why));
        }
        self.cleared.lock().unwrap().push(username.to_string());
        if let Some(why) = self.fail_clear.lock().unwrap().clone() {
            return Err(auth::NotCleared::Failed(why));
        }
        self.entries.lock().unwrap().remove(username);
        Ok(())
    }
}

/// C135: the saves a store recorded, each secret read as the restore
/// reads it and shown as the refresh token it holds.
pub(super) fn saved_refresh_tokens(
    saves: &[(String, String, String)],
) -> Vec<(String, String, String)> {
    saves
        .iter()
        .map(|(service, secret, username)| {
            let entry = auth::KeyringEntry::from_secret(&secret.as_str().into(), SystemTime::now());
            (
                service.clone(),
                entry.refresh_token.exposed_for_test().to_string(),
                username.clone(),
            )
        })
        .collect()
}

/// C135: a store's entries, each secret shown as the refresh token it
/// holds.
pub(super) fn entry_refresh_tokens(entries: &HashMap<String, String>) -> HashMap<String, String> {
    entries
        .iter()
        .map(|(username, secret)| {
            let entry = auth::KeyringEntry::from_secret(&secret.as_str().into(), SystemTime::now());
            (
                username.clone(),
                entry.refresh_token.exposed_for_test().to_string(),
            )
        })
        .collect()
}

/// The recording store a harness daemon from `test_daemon_with` saves
/// into. Panics if the daemon holds any other store.
pub(super) fn harness_credentials(daemon: &Daemon) -> &RecordingCredentialStore {
    let store: &dyn std::any::Any = &*daemon.credential_store;
    store
        .downcast_ref()
        .expect("the harness daemon's credential store is the test's recorder")
}

impl ScriptedResponse {
    /// A GGG response on `/fetch`, with the full rate-limit header set.
    pub(super) fn full(
        method: &'static str,
        status: &'static str,
        retry_after: Option<u64>,
        body: &str,
    ) -> Self {
        ScriptedResponse::full_at(method, "/fetch", status, retry_after, body)
    }

    /// `full`'s GGG response on any method and path.
    pub(super) fn full_at(
        method: &'static str,
        path: &'static str,
        status: &'static str,
        retry_after: Option<u64>,
        body: &str,
    ) -> Self {
        let retry_after = retry_after
            .map(|seconds| format!("Retry-After: {seconds}\r\n"))
            .unwrap_or_default();
        ScriptedResponse {
            method,
            path,
            status,
            headers: format!(
                "X-Rate-Limit-Policy: dispatcher-test-policy\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 100:1:60\r\nX-Rate-Limit-Account-State: 0:1:0\r\n{retry_after}"
            ),
            body: body.into(),
        }
    }

    /// The same response carrying `Retry-After` exactly as given, for a
    /// value `full` cannot spell (garbled, negative, over-cap).
    pub(super) fn with_raw_retry_after(mut self, raw: &str) -> Self {
        self.headers.push_str(&format!("Retry-After: {raw}\r\n"));
        self
    }

    /// Cloudflare's error 1015 as its docs describe it (N48): a 429 with
    /// `Retry-After: 30`, an HTML page, and no GGG header. The page's one
    /// marker, its "Ray ID", comes after its 300th character, past what a
    /// job's failure text keeps of a body, so a shape read from the kept
    /// text would miss it.
    pub(super) fn cloudflare_1015_429() -> Self {
        let filler = "You are being rate limited. ".repeat(12);
        ScriptedResponse {
            method: "GET",
            path: "/fetch",
            status: "429 Too Many Requests",
            headers: "Retry-After: 30\r\n".into(),
            body: format!(
                "<!DOCTYPE html><html><head><title>Access denied</title></head><body><h1>Access denied</h1><p>{filler}</p><p>Ray ID: 8a1f2c3d4e5f6a7b</p></body></html>"
            ),
        }
    }

    /// A Cloudflare challenge in GGG's place (N28): a 403 carrying
    /// `cf-mitigated: challenge` and no GGG header.
    pub(super) fn cloudflare_403_challenge() -> Self {
        ScriptedResponse {
            method: "GET",
            path: "/fetch",
            status: "403 Forbidden",
            headers: "cf-mitigated: challenge\r\n".into(),
            body: "<!DOCTYPE html><html><head><title>Just a moment...</title></head><body></body></html>".into(),
        }
    }

    /// Cloudflare answering for an origin it cannot reach: a 522 with
    /// Cloudflare's own error page ("Ray ID" in its footer) and no GGG
    /// header. The page's text is from knowledge of it, not a capture.
    pub(super) fn cloudflare_522() -> Self {
        ScriptedResponse {
            method: "GET",
            path: "/fetch",
            status: "522 Origin Connection Time-out",
            headers: String::new(),
            body: "<!DOCTYPE html><html><head><title>api.pathofexile.com | 522: Connection timed out</title></head><body><h1>Connection timed out</h1><p>Cloudflare was unable to establish a connection to the origin web server.</p><p>Cloudflare Ray ID: 8a1f2c3d4e5f6a7b</p></body></html>".into(),
        }
    }

    /// GGG's origin 503 as rung 10 met it (N35): openresty's page, no rate
    /// header, no Cloudflare marker.
    pub(super) fn origin_503() -> Self {
        ScriptedResponse {
            method: "GET",
            path: "/fetch",
            status: "503 Service Unavailable",
            headers: String::new(),
            body: "<html><head><title>503 Service Temporarily Unavailable</title></head><body><center><h1>503 Service Temporarily Unavailable</h1></center><hr><center>openresty</center></body></html>".into(),
        }
    }

    pub(super) fn malformed_head_429() -> Self {
        ScriptedResponse {
            method: "HEAD",
            path: "/fetch",
            status: "429 Too Many Requests",
            headers: "X-Rate-Limit-Policy: dispatcher-test-policy\r\nRetry-After: 0\r\n".into(),
            body: String::new(),
        }
    }
}

/// A token answer in the scripted server's shape: `POST /token`, the
/// given status and headers, and the rotated grant (`at-new`, `rt-rotated`
/// for `test-user`, an hour) or `body` as given.
pub(super) fn token_answer(
    status: &'static str,
    headers: &str,
    body: Option<&str>,
) -> ScriptedResponse {
    ScriptedResponse {
        method: "POST",
        path: "/token",
        status,
        headers: headers.into(),
        body: body
            .map(str::to_string)
            .unwrap_or_else(|| token_body("at-new", "rt-rotated", "test-user")),
    }
}

/// A token body as GGG answers a grant: the two tokens, an hour's
/// lifetime and the account's name.
pub(super) fn token_body(access_token: &str, refresh_token: &str, username: &str) -> String {
    json!({
        "access_token": access_token,
        "refresh_token": refresh_token,
        "expires_in": 3600,
        "username": username,
    })
    .to_string()
}

/// C158, which boundary keeps a component test off the door: the value
/// the three constructors take first, so the census
/// (`tools/daemon-tests-census.py`) reads it where the test builds its
/// daemon, never a sentence. One variant per boundary C158 and the door's
/// module doc name, and two the tests showed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Boundary {
    /// Several workers: the park hook fires per worker and never means
    /// all are idle.
    Workers,
    /// A child process.
    Child,
    /// The daemon's own frontend or callback socket, which no scripted
    /// GGG server stands in for.
    #[cfg_attr(not(unix), allow(dead_code))] // its tests, the listener's, are Unix's alone
    Socket,
    /// A function or a table: the test builds a daemon only to call one,
    /// or to read or stage state no frontend reaches (an internal, a file
    /// the daemon writes that no frontend reads).
    Function,
    /// A hand-driven clock the test reads or moves itself.
    Clock,
    /// Two fixtures at once, which the door's one `Server` cannot host.
    Fixtures,
}

/// A harness daemon facing `transport` (C161: an in-process fixture of
/// the test support's, answering at [`BASE`]) on the mock provider's
/// name, built for a component test that says which `boundary` keeps it
/// off the door.
pub(super) fn test_daemon(
    boundary: Boundary,
    transport: impl Transport + 'static,
    clock: Arc<dyn Clock>,
) -> (Arc<Daemon>, TestPath) {
    test_daemon_with(
        boundary,
        Provider::mock(BASE),
        transport,
        clock,
        RailsConfig::default(),
    )
}

/// The refusing transport on a hold of the test's own: a component
/// daemon that sends nothing, or whose every send must fail as a connect
/// nothing answers would.
pub(super) fn refused() -> InProcess {
    crate::transport::refusing(&Hold::new())
}

/// The scripted transport on a hold of the test's own, the responses in
/// order: the transport, the methods it received (what the journal is
/// held equal to) and its ends ([`assert_whole`]).
pub(super) fn scripted_transport(
    responses: Vec<ScriptedResponse>,
) -> (InProcess, Arc<Mutex<Vec<String>>>, Ends) {
    let script = responses.into_iter().map(Into::into).collect();
    let (transport, record) = crate::transport::scripted(&Hold::new(), script);
    (transport, record.wire, record.ends)
}

/// The account transport on a hold of the test's own
/// ([`crate::transport::account`]): the request numbered `held.2` of
/// those on `held`'s method and path held behind its barriers.
pub(super) fn account_transport(
    held: (&'static str, &'static str, usize),
) -> (InProcess, crate::transport::AccountRecord) {
    crate::transport::account(&Hold::new(), held)
}

/// A fixture asked whole and without a fault (a request it did not
/// expect): what a component test awaited a scripted server's end for.
pub(super) fn assert_whole(ends: &Ends) {
    assert!(ends.whole(), "the fixture was asked whole");
    assert_eq!(ends.faults(), Vec::<String>::new(), "the fixture's faults");
}

/// A path in a folder of the test's own: the folder is a
/// `tempfile::TempDir`, removed when this value drops, so a test that
/// fails leaves nothing behind either. It derefs to the path. A helper
/// that makes one returns it, never the bare path, which would outlive
/// its folder. Whatever writes into the folder (a thread, a permission
/// taken away) ends before it drops; the one exception is the daemon
/// itself. Its log, journal and `daemon.db` stay open while its tasks
/// hold it, past the test body and so past this value, and Windows
/// refuses to delete an open file: there the folder stays behind, as
/// the hand-written removes it replaced did. Unix deletes it.
pub(super) struct TestPath {
    path: PathBuf,
    _dir: tempfile::TempDir,
}

impl TestPath {
    /// The folder itself.
    pub(super) fn dir() -> TestPath {
        let dir = tempfile::tempdir().unwrap();
        TestPath {
            path: dir.path().to_path_buf(),
            _dir: dir,
        }
    }

    /// A file `name` in a folder made inside `parent`, not yet written:
    /// for a child process that ends by `exit`, which runs no `Drop`, so
    /// the folder is removed with the parent's, never left behind.
    pub(super) fn file_in(parent: &std::path::Path, name: &str) -> TestPath {
        let dir = tempfile::tempdir_in(parent).unwrap();
        TestPath {
            path: dir.path().join(name),
            _dir: dir,
        }
    }

    /// A file `name` in the folder, not yet written.
    pub(super) fn file(name: &str) -> TestPath {
        let dir = tempfile::tempdir().unwrap();
        TestPath {
            path: dir.path().join(name),
            _dir: dir,
        }
    }
}

impl std::ops::Deref for TestPath {
    type Target = std::path::Path;
    fn deref(&self) -> &std::path::Path {
        &self.path
    }
}

impl AsRef<std::path::Path> for TestPath {
    fn as_ref(&self) -> &std::path::Path {
        &self.path
    }
}

/// A harness daemon's log; its journal lands beside it ([`journal_of`]).
pub(super) fn test_log_path() -> TestPath {
    TestPath::file("daemon.log")
}

/// A `daemon.db` for one test.
pub(super) fn test_db_path() -> TestPath {
    TestPath::file("daemon.db")
}

/// The send journal every harness daemon writes, beside its log.
pub(super) fn journal_of(log_path: &std::path::Path) -> PathBuf {
    log_path.with_extension("jsonl")
}

/// Every harness daemon journals, on the test's clock, so that
/// `assert_journal_matches_wire` can hold the daemon's own account of
/// what it sent against what the server received. The rails handle
/// comes back through `daemon.choke.rails()`. Its credential store is a
/// `RecordingCredentialStore` (`harness_credentials` reads it): a harness
/// daemon never writes the developer's keychain unless its test names
/// the OS store. One empty session ([`test_daemon_at`] takes others).
pub(super) fn test_daemon_with(
    boundary: Boundary,
    provider: Provider,
    transport: impl Transport + 'static,
    clock: Arc<dyn Clock>,
    config: RailsConfig,
) -> (Arc<Daemon>, TestPath) {
    test_daemon_at(
        boundary,
        provider,
        transport,
        clock,
        config,
        test_log_path(),
        Sessions::with(AuthSession::default()),
    )
}

/// [`test_daemon_with`], its log (and the journal beside it) at
/// `log_path`, which a child process names inside its own directory, and
/// its sessions `sessions`, as the daemon's builder takes them. The
/// daemon carries its sends through `transport` (the builder's test-only
/// option, C161).
pub(super) fn test_daemon_at(
    _boundary: Boundary,
    provider: Provider,
    transport: impl Transport + 'static,
    clock: Arc<dyn Clock>,
    config: RailsConfig,
    log_path: TestPath,
    sessions: Sessions,
) -> (Arc<Daemon>, TestPath) {
    let rails = Arc::new(Rails::with_config_and_clock(
        RailsConfig {
            journal_path: Some(journal_of(&log_path)),
            ..config
        },
        clock.clone(),
    ));
    let log = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&log_path)
        .unwrap();
    let credentials: Arc<dyn CredentialStore> = Arc::new(RecordingCredentialStore::default());
    let daemon = Daemon::builder(provider, clock, credentials, log)
        .transport(Box::new(transport))
        .rails(rails)
        .sessions(sessions)
        .log_path(log_path.to_str().map(str::to_string))
        .build();
    (daemon, log_path)
}

pub(super) fn read_journal(path: &std::path::Path) -> Vec<Value> {
    std::fs::read_to_string(path)
        .expect("journal was written")
        .lines()
        .map(|l| serde_json::from_str(l).expect("journal line is JSON"))
        .collect()
}

/// Job `id` as a client reads it: from memory, or from the table once
/// its tree has concluded and left memory (C6); its outcome once
/// terminal. `None` for an id neither holds.
pub(super) fn held(daemon: &Daemon, id: JobId) -> Option<(JobInfo, Option<Outcome>)> {
    let in_memory = daemon
        .shared
        .lock()
        .unwrap()
        .jobs
        .get(&id)
        .map(|e| (e.info.clone(), e.outcome.clone()));
    in_memory.or_else(|| {
        let info = daemon.stored_job(id).expect("the table reads")?;
        Some((info, daemon.stored_outcome(id).expect("the table reads")))
    })
}

/// The state of job `id` as a client reads it ([`held`]).
pub(super) fn state_of(daemon: &Daemon, id: JobId) -> JobState {
    held(daemon, id).expect("a job the daemon issued").0.state
}

pub(super) async fn wait_terminal(daemon: &Daemon, id: JobId) -> (JobInfo, Outcome) {
    tokio::time::timeout(HANG, async {
        loop {
            if let Some((info, Some(outcome))) = held(daemon, id)
                && info.state.is_terminal()
            {
                return (info, outcome);
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("dispatcher completed the scripted job")
}

/// C158, a component test's hand on its parking clock where the daemon
/// waits on it (a landed 429's hold, a full window): until `reached`
/// holds, the clock moves to its earliest parked deadline, then the test
/// yields so what woke runs. Each sleeper wakes at its own deadline, so a
/// hold reads on the journal exactly as long as it was; nothing parked,
/// nothing moves. The shape of the scenario's park hook, by the test's
/// hand, for a test on a plain runtime, which has no hook. Under the hang
/// ceiling.
pub(super) async fn advance_until(clock: &ParkingClock, mut reached: impl FnMut() -> bool) {
    tokio::time::timeout(HANG, async {
        while !reached() {
            clock.advance_to_next();
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("the state was reached as the clock moved");
}

/// The bound of a drain a test holds open and releases: far past
/// anything the drain waits for, so it cannot expire before the release,
/// and safely added to an instant. Virtual: on the daemon's clock. A
/// clean leave reads less than it on the clock (round 2's rule).
pub(super) const HELD: Duration = Duration::from_secs(60 * 60);

/// C157, the runner: a lifecycle, a script of events, every effect asked
/// for, from its start.
pub(super) fn drive(machine: &mut Life, script: &[LifeEvent]) -> Vec<Effect> {
    let mut effects = machine.start();
    for event in script {
        effects.extend(machine.step(event.clone()));
    }
    effects
}

/// C157: the lifecycle's driver on `daemon`, the idle knob at `knob`,
/// spawned, its leave recorded instead of performed: the receiver gets
/// what the bound cut (`None` when the drain was clean) when the driver
/// reaches `Leave`. The exit's bound is the daemon's (`Exit::with_bound`,
/// where a test sets it), since a bound replaced here would replace an
/// exit a cause may have armed already.
pub(super) fn spawn_life(
    daemon: &Arc<Daemon>,
    knob: Duration,
) -> (
    tokio::task::JoinHandle<()>,
    oneshot::Receiver<Option<InFlight>>,
) {
    let (left, leave) = recorded_leave();
    (tokio::spawn(daemon.clone().life(knob, left)), leave)
}

/// C133, the drain of an exit a test began with `begin_exit`, as the stop's
/// connection hands it: its event given to the lifecycle's driver, which
/// runs in place to its leave, recorded; what the bound cut, `None` when
/// nothing was. The idle knob is the default's, which never expires here.
pub(super) fn drain(daemon: &Arc<Daemon>) -> impl Future<Output = Option<InFlight>> + use<> {
    daemon.exit.hand(LifeEvent::StopRequested { won: true });
    let daemon = daemon.clone();
    async move {
        let (left, leave) = recorded_leave();
        daemon.life(IDLE_SHUTDOWN, left).await;
        leave.await.expect("the driver reached its leave")
    }
}

/// C158, coordinated driving: a drain on a parking clock a component
/// test drives by hand, awaited to its end. `drain` is the harness's
/// [`drain`] awaited in place, a spawned drain's handle or a spawned
/// driver's recorded leave. The clock moves only after the test has seen
/// the state the driver waits for, nothing in flight (`InFlight` empty,
/// read through `try_lock`, so the test never waits on `shared`), and
/// only to a deadline the driver has registered before the drain's own
/// (its next look, `DRAIN_POLL` on): never to the drain's deadline, so
/// work still landing is never cut, and never a wait for a sleeper the
/// driver may not register (a driver that found quiet on its first look
/// has left, and `drain` answers at once). A look that finds the lock
/// held again re-arms, and is advanced in turn.
pub(super) async fn drive_drain<T>(
    clock: &ParkingClock,
    daemon: &Daemon,
    drain: impl Future<Output = T>,
) -> T {
    let mut drain = std::pin::pin!(drain);
    tokio::time::timeout(HANG, async {
        loop {
            // The poll's interval: a component test on a plain runtime has no
            // park hook to wake it when the state it waits for holds.
            tokio::select! {
                biased;
                out = &mut drain => return out,
                () = tokio::time::sleep(Duration::from_millis(1)) => {}
            }
            let quiet = daemon
                .shared
                .try_lock()
                .is_ok_and(|s| InFlight::of(&s) == InFlight::default());
            if quiet
                && let (Some(look), Some(&bound)) =
                    (clock.next_deadline(), daemon.exit.deadline.get())
                && look < bound
            {
                clock.advance_to(look);
            }
        }
    })
    .await
    .expect("the drain ended")
}

/// C158: wait, under the hang ceiling, until the drain's driver has
/// registered a sleep on `clock` (its first look, parked because what it
/// waits for is still in flight or `shared` is held).
pub(super) async fn await_a_look(clock: &ParkingClock) {
    tokio::time::timeout(HANG, async {
        while clock.sleepers() == 0 {
            // The poll's interval: a component test on a plain runtime has no
            // park hook to wake it when the state it waits for holds.
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    })
    .await
    .expect("the driver registered its first look");
}

/// A leave that records what the bound cut, and its receiver.
fn recorded_leave() -> (Leave, oneshot::Receiver<Option<InFlight>>) {
    let (tx, rx) = oneshot::channel();
    (
        Box::new(move |_daemon, cut| {
            let _ = tx.send(cut);
        }),
        rx,
    )
}

/// The teardown of a harness daemon that faces a scripted server or no
/// server. Every journaling harness daemon ends through this or
/// `finish_harness_mock`, so no journal a harness daemon writes goes
/// unread. The two differ in one oracle, because the two pacing oracles
/// are complements: "pacing follows responses" is real only against a
/// scripted server that echoes what the script names, and "never over
/// the limit" only against a counting mock, where the limiter holds
/// before a violation and a held send is the expected shape. Which
/// teardown a test calls is the one thing it chooses; nothing is
/// skipped by a flag. Both hold the journal to `assert_wire_contract`.
pub(super) fn finish_harness(
    dispatcher: impl Into<Option<tokio::task::JoinHandle<()>>>,
    log_path: &std::path::Path,
) {
    finish_with(dispatcher, log_path, assert_pacing_follows_responses);
}

/// The teardown of a harness daemon that faces the counting mock
/// (`mockggg`): the contract oracle and `assert_never_over_the_limit`.
/// See `finish_harness` for why the two teardowns differ.
pub(super) fn finish_harness_mock(
    dispatcher: impl Into<Option<tokio::task::JoinHandle<()>>>,
    log_path: &std::path::Path,
) {
    finish_with(dispatcher, log_path, assert_never_over_the_limit);
}

/// Abort the dispatcher, if the test started one and still holds it,
/// so no task writes while the oracles read; read the journal and hold
/// it to the contract oracle and the teardown's pacing oracle. A daemon
/// whose rails journal nothing has no file beside its log, and the read
/// is skipped. The log and the journal go with the test's [`TestPath`].
fn finish_with(
    dispatcher: impl Into<Option<tokio::task::JoinHandle<()>>>,
    log_path: &std::path::Path,
    pacing: fn(&[Value], bool),
) {
    if let Some(dispatcher) = dispatcher.into() {
        dispatcher.abort();
    }
    if journal_of(log_path).exists() {
        let sends = journal_sends(log_path);
        // A manual clock journals an exact wait; a real one journals
        // the scheduler's too (Gate (Windows) run 41243257231: 2 ms on
        // a send nothing held), so the pacing oracle is told which.
        let exact = read_journal(&journal_of(log_path))
            .first()
            .is_some_and(|header| header["clock"] == "manual");
        assert_wire_contract(&sends);
        pacing(&sends, exact);
    }
}

/// The token servers record bodies, not methods; every one was a POST.
pub(super) fn wire_posts(requests: &Arc<Mutex<Vec<String>>>) -> Vec<String> {
    vec!["POST".to_string(); requests.lock().unwrap().len()]
}

/// Journal == wire. The recorder is what the server received; the
/// journal is what the daemon *claims* it sent. Every send must appear
/// in both, in order, and a journal line that no server saw is as much
/// a defect as a send the journal missed (risk R4 of
/// `spikes/rust-playground/LIVE-TESTING.md` at `9fa99459`). Wire
/// entries are compared by method, which is what the scripted servers
/// record; the journal is read back here, before the harness deletes it.
pub(super) fn assert_journal_matches_wire(log_path: &std::path::Path, wire: &[String]) {
    let journal = journal_of(log_path);
    let lines = read_journal(&journal);
    let (header, sends) = lines.split_first().expect("journal has a header");
    assert_eq!(header["event"], "open", "first journal line is the header");
    assert_eq!(
        header["clock"], "manual",
        "harness daemons run on the test clock"
    );
    let journaled: Vec<&str> = sends
        .iter()
        .map(|l| l["method"].as_str().expect("journal method"))
        .collect();
    assert_eq!(
        journaled, wire,
        "journal (left) disagrees with what the server received (right)"
    );
}

/// `finish_harness` for tests that hold a wire recorder: the journal
/// held equal to the wire, then the rest of the teardown.
pub(super) fn finish_harness_wire(
    dispatcher: tokio::task::JoinHandle<()>,
    log_path: &std::path::Path,
    wire: &Arc<Mutex<Vec<String>>>,
) {
    assert_journal_matches_wire(log_path, &wire.lock().unwrap());
    finish_harness(dispatcher, log_path);
}

/// The journal's send lines, header dropped.
pub(super) fn journal_sends(log_path: &std::path::Path) -> Vec<Value> {
    let mut lines = read_journal(&journal_of(log_path));
    assert_eq!(
        lines.first().map(|l| l["event"].clone()),
        Some("open".into())
    );
    lines.remove(0);
    lines
}

pub(super) fn full_hold_ms() -> u64 {
    (crate::ratelimit::RETRY_BUCKET_PAD + crate::ratelimit::BUFFER).as_millis() as u64
}

/// The rest of the wire contract the register walk (TESTING-NOTES,
/// experiment 3) found expressible over the journal alone and true of
/// the product with rails off:
///
/// - N16/N24, probe-before-send, and C159, its exempt routes: within one
///   daemon lifetime the first send on a route is a HEAD, unless the
///   route is the token endpoint (`TOKEN_ROUTE`) or one the daemon
///   declares (`Daemon::NO_PROBE_ROUTES`, C32), whose first send is
///   never a HEAD ([`probe_exempt`]); the route's name is the part of its
///   endpoint key before `@`, as the client harness's rule reads it.
/// - N24, accounting: a HEAD is never counted; everything else is.
///
/// Not here, though it was from 2026-08-24 to 2026-09-12: "after a
/// 401 the next send is the refresh". The journal records completed
/// sends in completion order, and the gate is two wide, so a send
/// admitted beside the rejected one lands after it — as a second 401,
/// or as another account's send. What C87 promises is about dispatch,
/// not adjacency: no send leaves the gate carrying a bearer whose
/// rejection has landed. The sequential shape is pinned over the
/// journal by the `rejected_access_token_*` scenarios, the concurrent
/// shape at the server by `rejected_access_token_is_not_dispatched_*`.
fn assert_wire_contract(sends: &[Value]) {
    let mut seen: HashSet<(u64, String)> = HashSet::new();
    for send in sends {
        let pid = send["pid"].as_u64().unwrap();
        let route = send["route"].as_str().unwrap().to_string();
        let method = send["method"].as_str().unwrap();
        if seen.insert((pid, route.clone())) {
            if probe_exempt(&route) {
                assert_ne!(
                    method, "HEAD",
                    "first send on {route} in pid {pid} was a probe, on a route whose first \
                     send carries none (C159): {send}"
                );
            } else {
                assert_eq!(
                    method, "HEAD",
                    "first send on {route} in pid {pid} was not a probe: {send}"
                );
            }
        }
        assert_eq!(
            send["counted"],
            method != "HEAD",
            "HEADs are not counted and everything else is: {send}"
        );
    }
}

/// C159: whether `route`'s first send in a lifetime carries no probe:
/// the token endpoint and the routes the daemon declares, read from the
/// daemon's own constants, the one set (the copies outside the daemon
/// crate are held equal to it by check 11 of `tools/arch-check.sh`).
fn probe_exempt(route: &str) -> bool {
    let name = crate::ratelimit::split_endpoint_key(route).0;
    name == crate::ratelimit::TOKEN_ROUTE || Daemon::NO_PROBE_ROUTES.contains(&name)
}

/// The pacing invariant, derived from N19 and C19 rather than from the
/// code: a send on a route is held only because the previous landed
/// response on that route was a 429, or was counted and came back
/// without rate headers (rung 10's origin 503). After a 429, a GET's or
/// a HEAD's, the hold is at least C19's: its `Retry-After` clamped to the
/// cap (a missing, malformed or negative value counting as zero) plus
/// `RETRY_BUCKET_PAD + BUFFER`, whatever the header carried, and at most
/// that, unless the 429 was also headerless and counted and the longer
/// hold below applies. After a headerless counted response the hit is
/// assumed counted and the hold may reach the longest window period of
/// the route's last known policy plus the same pad. Everything else goes
/// out at once.
/// Runs over every journal a scripted server's or a serverless daemon
/// writes (`finish_harness`), so a rewrite that paces slower (or stops
/// pacing) fails here without any test naming the numbers.
fn assert_pacing_follows_responses(sends: &[Value], exact: bool) {
    let pad = full_hold_ms();
    let mut last_on_route: HashMap<String, &Value> = HashMap::new();
    let mut longest_period_ms: HashMap<String, u64> = HashMap::new();
    let headerless = |send: &Value| {
        send["counted"] == true
            && send["status"].as_u64().is_some()
            && send["rate"]
                .as_object()
                .is_none_or(|rate| !rate.keys().any(|k| k.starts_with("x-rate-limit-")))
    };
    for send in sends {
        let route = send["route"].as_str().unwrap().to_string();
        let wait = send["wait_ms"].as_u64().expect("wait_ms");
        let prev = last_on_route.get(&route);
        let raw_retry_after = prev.and_then(|prev| prev["rate"]["retry-after"].as_str());
        let headerless_ceiling = prev
            .filter(|prev| headerless(prev))
            .map(|_| longest_period_ms.get(&route).copied().unwrap_or(0) + pad);
        match prev {
            Some(prev) if prev["status"] == 429 => {
                let floor = c19_retry_secs(raw_retry_after) * 1000 + pad;
                let ceiling = floor.max(headerless_ceiling.unwrap_or(0));
                assert!(
                    (floor..=ceiling).contains(&wait),
                    "after a {} 429 with Retry-After {raw_retry_after:?} the next send on \
                         {route} waited {wait} ms; allowed {floor}..={ceiling}: {send}",
                    prev["method"].as_str().unwrap_or("?")
                );
            }
            Some(_) if headerless_ceiling.is_some() => {
                let ceiling = headerless_ceiling.unwrap_or(0);
                assert!(
                    wait <= ceiling,
                    "after a headerless counted response the next send on {route} \
                         waited {wait} ms; allowed at most {ceiling}: {send}"
                );
            }
            // Exact on a manual clock; on a real one, under the pacing
            // floor (C137: no wall-clock bound a slow machine can miss,
            // and a wait the limiter asked for is never shorter).
            _ if exact => assert_eq!(
                wait, 0,
                "nothing demanded a hold on {route}, yet the send waited: {send}"
            ),
            _ => assert!(
                wait < crate::ratelimit::PACING_FLOOR.as_millis() as u64,
                "nothing demanded a hold on {route}, yet the send waited past the \
                 pacing floor: {send}"
            ),
        }
        // Window periods from every `X-Rate-Limit-<rule>` header seen on
        // the route: "hits:period:restriction" triplets, comma-separated.
        if let Some(rate) = send["rate"].as_object() {
            for (key, value) in rate {
                if !key.starts_with("x-rate-limit-") || key.ends_with("-state") {
                    continue;
                }
                for triplet in value.as_str().unwrap_or("").split(',') {
                    if let Some(period) = triplet
                        .split(':')
                        .nth(1)
                        .and_then(|p| p.parse::<u64>().ok())
                    {
                        let slot = longest_period_ms.entry(route.clone()).or_default();
                        *slot = (*slot).max(period * 1000);
                    }
                }
            }
        }
        last_on_route.insert(route, send);
    }
}

/// C19's reading of a landed 429's `Retry-After`, written from the
/// ruling's words rather than from the parser: a whole number of seconds,
/// clamped to the cap; a missing, malformed or negative value counts as
/// zero, and a run of digits too long to count is over the cap.
fn c19_retry_secs(raw: Option<&str>) -> u64 {
    let cap = crate::ratelimit::RETRY_AFTER_CAP_SECS;
    let Some(value) = raw.map(str::trim) else {
        return 0;
    };
    if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
        return 0;
    }
    value.parse::<u64>().map_or(cap, |seconds| seconds.min(cap))
}

/// N4/N25 over the journal: no send is answered 429, and every window
/// state a response reports is within its limit with no restriction
/// active. Against the scripted server this is vacuous (it echoes what
/// the script says); against `mockggg` on the test clock the state is
/// the mock's own count, so a limiter that sends one too many fails
/// here. Runs in `finish_harness_mock`, over every daemon facing it. Rule names come from `X-Rate-Limit-Rules`, so this reads any
/// rule set, not only `Account`.
fn assert_never_over_the_limit(sends: &[Value], _exact: bool) {
    for send in sends {
        assert_ne!(send["status"], 429, "a send was answered 429: {send}");
        let rate = &send["rate"];
        let Some(rules) = rate["x-rate-limit-rules"].as_str() else {
            continue;
        };
        for rule in rules.split(',').map(|r| r.trim().to_ascii_lowercase()) {
            let limits = rate[format!("x-rate-limit-{rule}")]
                .as_str()
                .unwrap_or_else(|| panic!("limits for rule {rule}: {send}"));
            let states = rate[format!("x-rate-limit-{rule}-state")]
                .as_str()
                .unwrap_or_else(|| panic!("state for rule {rule}: {send}"));
            for (limit, state) in limits.split(',').zip(states.split(',')) {
                let max: u64 = limit.split(':').next().unwrap().parse().unwrap();
                let mut parts = state.split(':');
                let hits: u64 = parts.next().unwrap().parse().unwrap();
                let _period = parts.next();
                let restricted: u64 = parts.next().unwrap().parse().unwrap();
                assert!(
                    hits <= max,
                    "{rule} window {limit} reported {state}: over the limit: {send}"
                );
                assert_eq!(
                    restricted, 0,
                    "{rule} window {limit} reported an active restriction: {send}"
                );
            }
        }
    }
}

pub(super) fn tripwire_config() -> RailsConfig {
    RailsConfig {
        tripwire: true,
        ..RailsConfig::default()
    }
}

/// Session in memory, and a fresh recording store in place of whatever
/// the test's daemon held: these tests must never touch the real keychain
/// (slow, and may prompt).
pub(super) fn logged_in(daemon: &mut Arc<Daemon>) {
    Arc::get_mut(daemon).unwrap().credential_store = Arc::new(RecordingCredentialStore::default());
    let mut s = daemon.shared.lock().unwrap();
    s.auth.one_mut().refresh_token = Some("rt-old".into());
    s.auth.rename("", "test-user");
    s.auth.keyring = "ok".into();
}

/// The sessions [`logged_in`] leaves a harness daemon with, for a door:
/// `test-user`, its refresh token `rt-old`, the keyring `ok`.
pub(super) fn logged_in_sessions() -> Sessions {
    let mut sessions = Sessions::with(AuthSession {
        refresh_token: Some("rt-old".into()),
        ..AuthSession::default()
    });
    sessions.rename("", "test-user");
    sessions.keyring = "ok".into();
    sessions
}

/// Job `id` as an observer reads it from the ledger under `dir`, with no
/// daemon answering (C45, the store's `Persisted` view): a scenario's
/// disk read, once its door's `run` has returned, on the directory whose
/// `daemon.db` it gave the door; never while a daemon answers.
pub(super) fn on_disk(
    dir: &std::path::Path,
    id: JobId,
) -> Option<acquisition_store::jobs::PersistedRecord> {
    acquisition_store::jobs::Persisted::read_job(dir, id).expect("the ledger reads")
}

/// One live session, `Alice#1234`, holding the access token
/// `at-test.Alice#1234` until an hour past `clock`'s wall (as an install
/// writes the expiry) and no refresh token: a job admitted for it sends at
/// once, with no refresh first. [`logged_in_sessions`] is the
/// refresh-due shape.
pub(super) fn live_sessions(clock: &ParkingClock) -> Sessions {
    Sessions::with(AuthSession {
        username: Some("Alice#1234".into()),
        access_token: Some("at-test.Alice#1234".into()),
        access_expires_at: Some(clock.wall() + Duration::from_secs(3600)),
        ..AuthSession::default()
    })
}

/// Two refresh-due sessions: `test-user` and `other`, each holding a
/// refresh token (`rt-old`, `rt-other`) and no access token, so each
/// one's first job refreshes. The cancel family's (`dispatch.rs`) and
/// the rails' two-account scenarios (`rails.rs`).
pub(super) fn two_accounts() -> Sessions {
    let mut sessions = logged_in_sessions();
    sessions.replace(AuthSession {
        username: Some("other".into()),
        refresh_token: Some("rt-other".into()),
        ..AuthSession::default()
    });
    sessions
}

/// `path` at `mode` until this drops, then back at the mode it had. A
/// test that takes write away from a folder holds one, declared after the
/// folder's own guard: it drops first, so the folder can go however the
/// test ends. Restoring by hand after the call ran only when the call
/// returned.
#[cfg(unix)]
pub(super) struct WithMode<'a> {
    path: &'a std::path::Path,
    was: std::fs::Permissions,
}

#[cfg(unix)]
pub(super) fn with_mode(path: &std::path::Path, mode: u32) -> WithMode<'_> {
    use std::os::unix::fs::PermissionsExt;
    let was = std::fs::metadata(path).unwrap().permissions();
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).unwrap();
    WithMode { path, was }
}

#[cfg(unix)]
impl Drop for WithMode<'_> {
    fn drop(&mut self) {
        let _ = std::fs::set_permissions(self.path, self.was.clone());
    }
}

/// The index directory made read-only while the guard lives, so the
/// index's next write fails while its reads work.
#[cfg(unix)]
pub(super) fn freeze(dir: &std::path::Path) -> WithMode<'_> {
    with_mode(dir, 0o555)
}

pub(super) fn sent(account: &str, method: &str) -> Sent {
    (method.to_string(), account.to_string())
}

/// The methods the server saw, for the journal oracle.
pub(super) fn wire_methods(sent: &Arc<Mutex<Vec<Sent>>>) -> Vec<String> {
    sent.lock()
        .unwrap()
        .iter()
        .map(|(m, _)| m.clone())
        .collect()
}
