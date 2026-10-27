//! The harness the daemon's scenario tests share: the clocks, the
//! credential stores, the scripted servers, the test daemons, the journal
//! readers and the four journal oracles.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use crate::daemon::*;
/// The check of a send no test cancels (C25).
pub(super) use crate::ratelimit::ALWAYS_WANTED;
use crate::ratelimit::Clock;
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::oneshot;

/// Wall-clock origin for scenarios: 2000-01-01T00:00:00Z. Deliberately
/// synthetic so a journal written under this clock is recognizable as a
/// scenario even if someone strips the header line.
pub(super) const MANUAL_WALL_ORIGIN: Duration = Duration::from_secs(946_684_800);

pub(super) struct ManualClock {
    now: Mutex<Instant>,
    wall: Mutex<SystemTime>,
}

impl ManualClock {
    pub(super) fn new() -> Self {
        ManualClock {
            now: Mutex::new(Instant::now()),
            wall: Mutex::new(SystemTime::UNIX_EPOCH + MANUAL_WALL_ORIGIN),
        }
    }

    /// Time passes normally: both faces move.
    pub(super) fn advance(&self, duration: Duration) {
        let mut now = self.now.lock().unwrap();
        *now = now.checked_add(duration).expect("bounded test deadline");
        *self.wall.lock().unwrap() += duration;
    }

    /// The lid closes: the wall clock moves, the monotonic clock does
    /// not. This is the R8 shape.
    pub(super) fn laptop_sleep(&self, duration: Duration) {
        *self.wall.lock().unwrap() += duration;
    }
}

impl Clock for ManualClock {
    fn now(&self) -> Instant {
        *self.now.lock().unwrap()
    }

    fn wall(&self) -> SystemTime {
        *self.wall.lock().unwrap()
    }

    fn kind(&self) -> &'static str {
        "manual"
    }

    fn sleep(&self, duration: Duration) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            self.advance(duration);
            tokio::task::yield_now().await;
        })
    }
}

pub(super) struct NoopCredentialStore;

impl CredentialStore for NoopCredentialStore {
    fn save(&self, _service: &str, _refresh_token: &str, _username: &str) -> Result<(), String> {
        Ok(())
    }

    fn load(&self, _service: &str, _username: &str) -> Result<Option<String>, String> {
        Ok(None)
    }

    fn clear(&self, _service: &str, _username: &str) -> Result<(), auth::NotCleared> {
        Ok(())
    }
}

/// An in-memory keyring that records every call and can be told to fail
/// any of the three (C144: every result reaches the caller). A save
/// records its attempt and, when it succeeds, holds the entry; a clear
/// records its attempt and, when it succeeds, removes it.
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
    fn save(&self, service: &str, refresh_token: &str, username: &str) -> Result<(), String> {
        let hook = self.on_save.lock().unwrap().take();
        if let Some(hook) = hook {
            hook();
        }
        self.saves.lock().unwrap().push((
            service.to_string(),
            refresh_token.to_string(),
            username.to_string(),
        ));
        if let Some(why) = self.fail_save.lock().unwrap().clone() {
            return Err(why);
        }
        self.entries
            .lock()
            .unwrap()
            .insert(username.to_string(), refresh_token.to_string());
        Ok(())
    }

    fn load(&self, _service: &str, username: &str) -> Result<Option<String>, String> {
        if let Some(why) = self.fail_load.lock().unwrap().clone() {
            return Err(why);
        }
        if let Some(why) = self.fail_load_under.lock().unwrap().get(username) {
            return Err(why.clone());
        }
        Ok(self.entries.lock().unwrap().get(username).cloned())
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

/// The recording store a harness daemon from `test_daemon_with` saves
/// into. Panics if the daemon holds any other store.
pub(super) fn harness_credentials(daemon: &Daemon) -> &RecordingCredentialStore {
    let store: &dyn std::any::Any = &*daemon.credential_store;
    store
        .downcast_ref()
        .expect("the harness daemon's credential store is the test's recorder")
}

pub(super) struct ScriptedResponse {
    pub(super) method: &'static str,
    /// The request path this response answers; the server asserts it.
    pub(super) path: &'static str,
    pub(super) status: &'static str,
    pub(super) headers: String,
    pub(super) body: String,
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

pub(super) async fn scripted_server(
    responses: Vec<ScriptedResponse>,
) -> (String, Arc<Mutex<Vec<String>>>, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let requests = Arc::new(Mutex::new(Vec::new()));
    let captured = requests.clone();
    let task = tokio::spawn(async move {
        for response in responses {
            let (mut stream, _) = listener.accept().await.unwrap();
            let request = mockggg::read_request(&mut stream).await.unwrap();
            assert_eq!(request.method, response.method);
            assert_eq!(request.path, response.path);
            captured.lock().unwrap().push(request.method);
            mockggg::respond_with(
                &mut stream,
                response.status,
                "application/json",
                &response.headers,
                &response.body,
            )
            .await;
        }
    });
    (base, requests, task)
}

pub(super) async fn delayed_token_server() -> (
    String,
    oneshot::Receiver<()>,
    oneshot::Sender<()>,
    tokio::task::JoinHandle<()>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let (arrived_tx, arrived_rx) = oneshot::channel();
    let (release_tx, release_rx) = oneshot::channel();
    let task = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let request = mockggg::read_request(&mut stream).await.unwrap();
        assert_eq!(request.method, "POST");
        assert_eq!(request.path, "/token");
        assert!(request.body.contains("grant_type=refresh_token"));
        assert!(request.body.contains("refresh_token=rt-old"));
        arrived_tx.send(()).unwrap();
        release_rx.await.unwrap();
        let body = json!({
            "access_token": "at-new",
            "refresh_token": "rt-rotated",
            "expires_in": 3600,
            "username": "test-user",
        })
        .to_string();
        let headers = concat!(
            "X-Rate-Limit-Policy: token-request-limit\r\n",
            "X-Rate-Limit-Rules: Ip\r\n",
            "X-Rate-Limit-Ip: 60:30:30\r\n",
            "X-Rate-Limit-Ip-State: 1:30:0\r\n",
        );
        mockggg::respond_with(&mut stream, "200 OK", "application/json", headers, &body).await;
    });
    (base, arrived_rx, release_tx, task)
}

pub(super) fn test_daemon(base: &str, clock: Arc<dyn Clock>) -> (Arc<Daemon>, TestPath) {
    test_daemon_with(Provider::mock(base), clock, RailsConfig::default())
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
/// the OS store.
pub(super) fn test_daemon_with(
    provider: Provider,
    clock: Arc<dyn Clock>,
    config: RailsConfig,
) -> (Arc<Daemon>, TestPath) {
    let log_path = test_log_path();
    let rails = Arc::new(Rails::with_config_and_clock(
        RailsConfig {
            journal_path: Some(journal_of(&log_path)),
            ..config
        },
        clock.clone(),
    ));
    test_daemon_scenario_at(
        provider,
        clock,
        rails,
        Arc::new(RecordingCredentialStore::default()),
        log_path,
    )
}

pub(super) fn test_daemon_scenario(
    provider: Provider,
    clock: Arc<dyn Clock>,
    rails: Arc<Rails>,
    credential_store: Arc<dyn CredentialStore>,
) -> (Arc<Daemon>, TestPath) {
    test_daemon_scenario_at(provider, clock, rails, credential_store, test_log_path())
}

pub(super) fn test_daemon_scenario_at(
    provider: Provider,
    clock: Arc<dyn Clock>,
    rails: Arc<Rails>,
    credential_store: Arc<dyn CredentialStore>,
    log_path: TestPath,
) -> (Arc<Daemon>, TestPath) {
    let log = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&log_path)
        .unwrap();
    let now = clock.now();
    let daemon = Arc::new(Daemon {
        credentials: Mutex::new(()),
        shared: Mutex::new(Shared {
            jobs: HashMap::new(),
            next_id: 1,
            auth: Sessions::with(AuthSession::default()),
            landings: 0,
            errors: VecDeque::new(),
            active_jobs: HashMap::new(),
        }),
        connections: AtomicUsize::new(0),
        last_activity: AtomicU64::new(0),
        started: now,
        events: broadcast::channel(256).0,
        work: Notify::new(),
        log: Mutex::new(log),
        choke: ChokePoint::with_clock_and_rails(clock, rails, &provider.api_base),
        provider,
        artifact: None,
        credential_store,
        world: None,
        log_path: log_path.to_str().map(str::to_string),
        socket_path: None,
        store_dir: None,
        store: Mutex::new(None),
        jobs_db: Mutex::new(JobDb::open_memory().unwrap()),
        queue_failure: Mutex::new(None),
        exit: Exit::default(),
        refresh_landed_hook: Mutex::new(None),
    });
    // Test daemons know what the real one knows about routes.
    Daemon::declare_route_knowledge(&daemon.choke);
    (daemon, log_path)
}

pub(super) fn read_journal(path: &std::path::Path) -> Vec<Value> {
    std::fs::read_to_string(path)
        .expect("journal was written")
        .lines()
        .map(|l| serde_json::from_str(l).expect("journal line is JSON"))
        .collect()
}

pub(super) async fn wait_terminal(daemon: &Daemon, id: JobId) -> (JobInfo, Outcome) {
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            if let Some(done) = {
                let shared = daemon.shared.lock().unwrap();
                shared.jobs.get(&id).and_then(|entry| {
                    entry
                        .info
                        .state
                        .is_terminal()
                        .then(|| (entry.info.clone(), entry.outcome.clone().unwrap()))
                })
            } {
                return done;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("dispatcher completed the scripted job")
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
    pacing: fn(&[Value]),
) {
    if let Some(dispatcher) = dispatcher.into() {
        dispatcher.abort();
    }
    if journal_of(log_path).exists() {
        let sends = journal_sends(log_path);
        assert_wire_contract(&sends);
        pacing(&sends);
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
/// - N16/N24, probe-before-send: within one daemon lifetime the first
///   send on any API route is a HEAD. The token endpoint is never
///   probed, nor are `profile` and `league`, the two routes
///   `Daemon::NO_PROBE_ROUTES` declares (C32); the route's name is the
///   part of its endpoint key before `@`, as the client harness's rule
///   reads it.
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
        let probed = !matches!(
            crate::ratelimit::split_endpoint_key(&route).0,
            "oauth-token" | "profile" | "league"
        );
        if probed && seen.insert((pid, route.clone())) {
            assert_eq!(
                method, "HEAD",
                "first send on {route} in pid {pid} was not a probe: {send}"
            );
        }
        assert_eq!(
            send["counted"],
            method != "HEAD",
            "HEADs are not counted and everything else is: {send}"
        );
    }
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
fn assert_pacing_follows_responses(sends: &[Value]) {
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
            _ => assert_eq!(
                wait, 0,
                "nothing demanded a hold on {route}, yet the send waited: {send}"
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
fn assert_never_over_the_limit(sends: &[Value]) {
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

pub(super) async fn token_server_answering(
    statuses: Vec<&'static str>,
) -> (String, Arc<Mutex<Vec<String>>>, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let requests = Arc::new(Mutex::new(Vec::new()));
    let captured = requests.clone();
    let task = tokio::spawn(async move {
        for status in statuses {
            let (mut stream, _) = listener.accept().await.unwrap();
            let request = mockggg::read_request(&mut stream).await.unwrap();
            assert_eq!(request.path, "/token");
            captured.lock().unwrap().push(request.body.clone());
            let headers = concat!(
                "X-Rate-Limit-Policy: token-request-limit\r\n",
                "X-Rate-Limit-Rules: Ip\r\n",
                "X-Rate-Limit-Ip: 60:30:30\r\n",
                "X-Rate-Limit-Ip-State: 1:30:0\r\n",
            );
            let body = if status.starts_with("200") {
                json!({
                    "access_token": "at-new",
                    "refresh_token": "rt-rotated",
                    "expires_in": 3600,
                    "username": "test-user",
                })
                .to_string()
            } else {
                r#"{"error":"invalid_grant"}"#.to_string()
            };
            mockggg::respond_with(&mut stream, status, "application/json", headers, &body).await;
        }
    });
    (base, requests, task)
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

/// One send as the account server saw it: the method, and the account it
/// was for (the token POST's grant, or the GET's bearer).
pub(super) type Sent = (String, String);

/// A server for the cancel scenarios that answers by what was sent, each
/// connection on its own task: `POST /token` with the grant's account's
/// tokens (`rt-old` is `test-user`'s, `rt-<name>` is `<name>`'s, and the
/// rotated token is `rt-<name>`), and `GET /profile` with the bearer's
/// account, without rate headers as GGG serves it. Of the requests
/// matching `hold`'s method and path, the one numbered `hold.2` (from 0)
/// is held: `arrived` fires when it is in, and it is answered when
/// `release` fires.
pub(super) async fn account_server(
    hold: (&'static str, &'static str, usize),
) -> (
    String,
    Arc<Mutex<Vec<Sent>>>,
    oneshot::Receiver<()>,
    oneshot::Sender<()>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let sent: Arc<Mutex<Vec<Sent>>> = Arc::default();
    let (arrived_tx, arrived_rx) = oneshot::channel();
    let (release_tx, release_rx) = oneshot::channel::<()>();
    let held = Arc::new(Mutex::new(Some((arrived_tx, release_rx))));
    let matched = Arc::new(AtomicU64::new(0));
    let recorded = sent.clone();
    tokio::spawn(async move {
        loop {
            let (mut stream, _) = listener.accept().await.unwrap();
            let recorded = recorded.clone();
            let held = held.clone();
            let matched = matched.clone();
            tokio::spawn(async move {
                let Some(request) = mockggg::read_request(&mut stream).await else {
                    return;
                };
                let account = if request.path == "/token" {
                    let grant: HashMap<String, String> =
                        url::form_urlencoded::parse(request.body.as_bytes())
                            .into_owned()
                            .collect();
                    match grant["refresh_token"].as_str() {
                        "rt-old" => "test-user".to_string(),
                        token => token.trim_start_matches("rt-").to_string(),
                    }
                } else {
                    request
                        .headers
                        .get("authorization")
                        .map(|bearer| bearer.trim_start_matches("Bearer at-").to_string())
                        .unwrap_or_default()
                };
                recorded
                    .lock()
                    .unwrap()
                    .push((request.method.clone(), account.clone()));
                if (request.method.as_str(), request.path.as_str()) == (hold.0, hold.1)
                    && matched.fetch_add(1, Ordering::SeqCst) == hold.2 as u64
                {
                    let taken = held.lock().unwrap().take();
                    if let Some((arrived, release)) = taken {
                        arrived.send(()).unwrap();
                        let _ = release.await;
                    }
                }
                if request.path == "/token" {
                    let body = json!({
                        "access_token": format!("at-{account}"),
                        "refresh_token": format!("rt-{account}"),
                        "expires_in": 3600,
                        "username": account,
                    })
                    .to_string();
                    let headers = concat!(
                        "X-Rate-Limit-Policy: token-request-limit\r\n",
                        "X-Rate-Limit-Rules: Ip\r\n",
                        "X-Rate-Limit-Ip: 60:30:30\r\n",
                        "X-Rate-Limit-Ip-State: 1:30:0\r\n",
                    );
                    mockggg::respond_with(
                        &mut stream,
                        "200 OK",
                        "application/json",
                        headers,
                        &body,
                    )
                    .await;
                } else {
                    let body = json!({ "uuid": format!("uuid-{account}"), "name": account });
                    mockggg::respond(&mut stream, "200 OK", "application/json", &body.to_string())
                        .await;
                }
            });
        }
    });
    (base, sent, arrived_rx, release_tx)
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
