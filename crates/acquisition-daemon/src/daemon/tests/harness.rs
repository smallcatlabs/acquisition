//! The harness the daemon's scenario tests share: the clocks, the
//! credential stores, the scripted servers, the test daemons, the journal
//! readers and the four journal oracles.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use crate::daemon::*;
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

    fn clear(&self, _service: &str, _username: &str) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Default)]
pub(super) struct RecordingCredentialStore {
    pub(super) saves: Mutex<Vec<(String, String, String)>>,
    pub(super) cleared: Mutex<Vec<String>>,
}

impl CredentialStore for RecordingCredentialStore {
    fn save(&self, service: &str, refresh_token: &str, username: &str) -> Result<(), String> {
        self.saves.lock().unwrap().push((
            service.to_string(),
            refresh_token.to_string(),
            username.to_string(),
        ));
        Ok(())
    }

    fn clear(&self, _service: &str, username: &str) -> Result<(), String> {
        self.cleared.lock().unwrap().push(username.to_string());
        Ok(())
    }
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
    pub(super) fn full(
        method: &'static str,
        status: &'static str,
        retry_after: Option<u64>,
        body: &str,
    ) -> Self {
        let retry_after = retry_after
            .map(|seconds| format!("Retry-After: {seconds}\r\n"))
            .unwrap_or_default();
        ScriptedResponse {
            method,
            path: "/fetch",
            status,
            headers: format!(
                "X-Rate-Limit-Policy: dispatcher-test-policy\r\nX-Rate-Limit-Rules: Account\r\nX-Rate-Limit-Account: 100:1:60\r\nX-Rate-Limit-Account-State: 0:1:0\r\n{retry_after}"
            ),
            body: body.into(),
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

pub(super) static TEST_LOG_ID: AtomicU64 = AtomicU64::new(1);

pub(super) fn test_daemon(base: &str, clock: Arc<dyn Clock>) -> (Arc<Daemon>, PathBuf) {
    test_daemon_with(Provider::mock(base), clock, RailsConfig::default())
}

pub(super) fn test_log_path() -> PathBuf {
    std::env::temp_dir().join(format!(
        "acquisition-n1-dispatcher-{}-{}.log",
        std::process::id(),
        TEST_LOG_ID.fetch_add(1, Ordering::Relaxed)
    ))
}

/// The send journal every harness daemon writes, beside its log.
pub(super) fn journal_of(log_path: &std::path::Path) -> PathBuf {
    log_path.with_extension("jsonl")
}

/// Every harness daemon journals, on the test's clock, so that
/// `assert_journal_matches_wire` can hold the daemon's own account of
/// what it sent against what the server received. The rails handle
/// comes back through `daemon.choke.rails()`.
pub(super) fn test_daemon_with(
    provider: Provider,
    clock: Arc<dyn Clock>,
    config: RailsConfig,
) -> (Arc<Daemon>, PathBuf) {
    let log_path = test_log_path();
    let _ = std::fs::remove_file(journal_of(&log_path));
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
        Arc::new(OsCredentialStore),
        log_path,
    )
}

pub(super) fn test_daemon_scenario(
    provider: Provider,
    clock: Arc<dyn Clock>,
    rails: Arc<Rails>,
    credential_store: Arc<dyn CredentialStore>,
) -> (Arc<Daemon>, PathBuf) {
    test_daemon_scenario_at(provider, clock, rails, credential_store, test_log_path())
}

pub(super) fn test_daemon_scenario_at(
    provider: Provider,
    clock: Arc<dyn Clock>,
    rails: Arc<Rails>,
    credential_store: Arc<dyn CredentialStore>,
    log_path: PathBuf,
) -> (Arc<Daemon>, PathBuf) {
    let log = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&log_path)
        .unwrap();
    let now = clock.now();
    let daemon = Arc::new(Daemon {
        shared: Mutex::new(Shared {
            jobs: HashMap::new(),
            next_id: 1,
            auth: Sessions::with(AuthSession::default()),
            connections: 0,
            last_activity: now,
            errors: VecDeque::new(),
            active_jobs: HashMap::new(),
        }),
        started: now,
        events: broadcast::channel(256).0,
        work: Notify::new(),
        log: Mutex::new(log),
        choke: ChokePoint::with_clock_and_rails(clock, rails),
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
/// it to the contract oracle and the teardown's pacing oracle; remove
/// the log and the journal. A daemon whose rails journal nothing has no
/// file beside its log, and the read is skipped.
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
    remove_harness_files(log_path);
}

fn remove_harness_files(log_path: &std::path::Path) {
    let _ = std::fs::remove_file(log_path);
    let _ = std::fs::remove_file(journal_of(log_path));
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

/// The pacing invariant, derived from N19 rather than from the code: a
/// send on a route is held only because the previous landed response on
/// that route was a 429, and then for at least its `Retry-After` and at
/// most `Retry-After + RETRY_BUCKET_PAD + BUFFER`, the largest hold the
/// limiter is allowed to impose — or because it was counted and came
/// back without rate headers (rung 10's origin 503), in which case the
/// hit is assumed counted and the hold may reach the longest window
/// period of the route's last known policy plus the same pad. Everything
/// else goes out at once. Runs over every journal a scripted server's
/// or a serverless daemon writes (`finish_harness`), so a rewrite
/// that paces slower (or stops pacing) fails here without any test
/// naming the numbers.
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
        match last_on_route.get(&route) {
            Some(prev) if headerless(prev) => {
                let ceiling = longest_period_ms.get(&route).copied().unwrap_or(0) + pad;
                assert!(
                    wait <= ceiling,
                    "after a headerless counted response the next send on {route} \
                         waited {wait} ms; allowed at most {ceiling}: {send}"
                );
            }
            Some(prev) if prev["status"] == 429 => {
                let retry_after: u64 = prev["rate"]["retry-after"]
                    .as_str()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                let floor = retry_after * 1000;
                assert!(
                    (floor..=floor + pad).contains(&wait),
                    "after a 429 with Retry-After {retry_after} the next send on \
                         {route} waited {wait} ms; allowed {floor}..={}: {send}",
                    floor + pad
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

/// Session in memory and the OS keyring swapped out: these tests must
/// never touch the real keychain (slow, and may prompt).
pub(super) fn logged_in(daemon: &mut Arc<Daemon>) {
    Arc::get_mut(daemon).unwrap().credential_store = Arc::new(RecordingCredentialStore::default());
    let mut s = daemon.shared.lock().unwrap();
    s.auth.one_mut().refresh_token = Some("rt-old".into());
    s.auth.rename("", "test-user");
    s.auth.keyring = "ok".into();
}
