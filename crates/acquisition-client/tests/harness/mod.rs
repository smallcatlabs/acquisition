//! The process harness every crate's tests share (C122): the client's
//! `contract.rs` takes it as `mod harness;`, the verbs crate's
//! `tests/daemon.rs` and the CLI's and the MCP's adapters (their own
//! `tests/harness/mod.rs`, which bind their binary) include it by
//! `#[path]`. A file under `tests/`, not a crate: it names no frontend's
//! binary (`CARGO_BIN_EXE_*` exists only in the owning crate's tests), so
//! each adapter hands it the executable, and it links what every
//! consumer already links (the client, the protocol crate, the store,
//! tokio and serde_json).
//!
//! The core is one isolation list with two appliers. [`isolation`] names
//! every knob once: every `ACQ_*` the process holds removed by prefix, so
//! a new knob reaches no test daemon without a list kept in step, then
//! the world, the runtime and log directories under the scratch, no
//! keyring, no journal, a short idle shutdown and the mock provider.
//! [`isolate`] applies it to a child; [`session`] applies it to the test's
//! own process under the one lock, for the files that drive a `Client` or
//! a `Frontend` in-process. A daemon is always started from the explicit
//! list, never by inheriting a mutated environment, and a caller's later
//! `.env(..)` wins: a test still opts into the real provider's refusal
//! path, a journal path or `ACQ_NO_SPAWN`.
//!
//! The daemon a test starts is the `acqd` its build wrote: beside the
//! frontend under test ([`acqd_beside`], C82), or one level above the
//! `deps/` directory for a crate with no binary of its own
//! ([`acqd_built`]); either names `cargo build --workspace` when it is
//! missing, since a `-p` test run does not build it. A test that drives
//! the client in-process places the daemon's name beside its own
//! executable too ([`link_beside_this_test`]), since the client judges a
//! daemon's artifact against that sibling (C84). Nothing here reaches
//! GGG: the provider is the mock unless a caller says otherwise.
#![allow(dead_code)] // each test binary uses the slice it needs

mod diff;

use std::collections::HashMap;
use std::ffi::OsString;
use std::io::{BufRead as _, Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};
use std::time::{Duration, Instant};

use acquisition_client::client::{Client, Observed};
use acquisition_protocol::protocol::{Request, Response};
use serde_json::Value;

#[allow(unused_imports)] // the adapters re-export them for the goldens
pub use diff::{same_text, unified};

// ---- scratch ------------------------------------------------------------

/// The test's scratch directory, removed on drop: declare it before the
/// daemon, so the daemon is gone first.
pub struct Scratch(pub PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The temp directory as the process started, read once: a [`session`]
/// points `TMPDIR` into its scratch, and the next scratch must not nest
/// under the last (a socket path caps near 104 bytes).
pub fn original_temp_dir() -> &'static Path {
    static ORIGINAL: OnceLock<PathBuf> = OnceLock::new();
    ORIGINAL.get_or_init(std::env::temp_dir)
}

/// A fresh scratch directory directly under the original temp dir, named
/// by `tag`, the process and a counter, so tests in one binary never
/// share one and a process id that comes round again never opens what an
/// earlier run left. The tag is clipped to its first eight characters: a
/// daemon's socket lands at `<scratch>/tmp/acq-<uid>/<12 hex>.sock`, and on
/// macOS, whose temp dir is 49 bytes deep, a longer name pushes that past
/// the 103-byte cap (a fourteen-letter tag did, once the counter joined).
pub fn scratch(tag: &str) -> Scratch {
    static N: AtomicU64 = AtomicU64::new(0);
    let tag: String = tag.chars().take(8).collect();
    let base = original_temp_dir().join(format!(
        "acq-{tag}-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap_or_else(|e| panic!("scratch {}: {e}", base.display()));
    socket_fits_under(&base);
    Scratch(base)
}

/// The longest socket a test can derive under `base` fits the cap, so a
/// deeper temp dir or a longer tag fails here, by name, and not as a
/// daemon that never answers. The shape is the store's
/// (`acquisition-store/src/world.rs`): `runtime_dir_path`'s fallback
/// `<TMPDIR>/acq-<uid>`, the longer of it and Linux's
/// `<XDG_RUNTIME_DIR>/acq`, and `World::socket_path_under`'s
/// `<id>.sock`, the id twelve hex digits.
#[cfg(unix)]
fn socket_fits_under(base: &Path) {
    use std::os::unix::fs::MetadataExt as _;
    let uid = std::fs::metadata(base).map(|m| m.uid()).unwrap_or(u32::MAX);
    let socket = scratch_tmp_path(base)
        .join(format!("acq-{uid}"))
        .join(format!("{}.sock", "0".repeat(12)));
    let len = socket.as_os_str().len();
    let cap = acquisition_store::world::SOCKET_PATH_MAX;
    assert!(
        len <= cap,
        "a daemon's socket under the scratch {} would be {len} bytes, past the cap of {cap}: \
         the temp dir is too deep or the tag too long",
        base.display()
    );
}

#[cfg(not(unix))]
fn socket_fits_under(_base: &Path) {}

/// Where [`scratch_tmp`] puts the runtime directory, computed and not
/// created.
fn scratch_tmp_path(base: &Path) -> PathBuf {
    base.join("tmp")
}

/// The scratch temp and runtime directory of one test, under `base`:
/// `TMPDIR` (macOS's runtime fallback) and `XDG_RUNTIME_DIR` (Linux's
/// runtime directory) both point here, so the sockets the daemons bind,
/// and those a killed daemon leaves, never touch the user's own runtime
/// directory (C83). The real-mode lock is not here and no variable moves
/// it: it lives in the OS user's own data directory, from the home the
/// system records (C132), so a test that starts a real-mode daemon takes
/// the user's real lock. Created here, since the runtime directory is
/// made under an existing parent.
pub fn scratch_tmp(base: &Path) -> PathBuf {
    let tmp = scratch_tmp_path(base);
    let _ = std::fs::create_dir_all(&tmp);
    tmp
}

// ---- the isolation list -------------------------------------------------

/// Every knob a test process sets, once: each `ACQ_*` the process holds
/// removed (`None`), then the thirteen values. `world` is the store root
/// (its socket derives from it, C83); the runtime and log directories sit
/// under `base`: the temp directory is named for each platform's reader,
/// `TMPDIR` on macOS, `XDG_RUNTIME_DIR` on Linux, and `TEMP` and `TMP`,
/// which `std::env::temp_dir` reads on Windows, so no two tests' daemons
/// share the runner's own temp directory there. None of them moves the
/// real-mode lock (C132, [`scratch_tmp`]). The three liveness deadlines a
/// frontend runs under, the handshake's, a spawned daemon's bind and the
/// annotation file's wait, are widened to 60 s, so a starved runner
/// cannot fail a healthy path (C137, the deadline's contract is its
/// expiry behaviour); a test that pins one sets its own value after this
/// list.
pub fn isolation(world: &Path, base: &Path) -> Vec<(OsString, Option<OsString>)> {
    let mut list: Vec<(OsString, Option<OsString>)> = std::env::vars_os()
        .filter(|(key, _)| key.to_str().is_some_and(|k| k.starts_with("ACQ_")))
        .map(|(key, _)| (key, None))
        .collect();
    let tmp = scratch_tmp(base);
    for (key, value) in [
        ("ACQ_STORE_DIR", world.as_os_str().to_owned()),
        ("TMPDIR", tmp.clone().into_os_string()),
        ("TEMP", tmp.clone().into_os_string()),
        ("TMP", tmp.clone().into_os_string()),
        ("XDG_RUNTIME_DIR", tmp.into_os_string()),
        ("ACQ_LOG_DIR", base.join("logs").into_os_string()),
        ("ACQ_NO_KEYRING", "1".into()),
        ("ACQ_JOURNAL", "0".into()),
        ("ACQ_IDLE_SHUTDOWN", "30".into()),
        ("ACQ_PROVIDER", "mock".into()),
        ("ACQ_HELLO_WITHIN", "60".into()),
        ("ACQ_STARTUP_WITHIN", "60".into()),
        ("ACQ_ANNOTATION_WAIT", "60".into()),
    ] {
        list.push((key.into(), Some(value)));
    }
    list
}

/// [`isolation`] on a child, the world at `base/store`.
pub fn isolate(cmd: &mut Command, base: &Path) {
    isolate_at(cmd, &base.join("store"), base);
}

/// [`isolation`] on a child, the world at `world`.
pub fn isolate_at(cmd: &mut Command, world: &Path, base: &Path) {
    for (key, value) in isolation(world, base) {
        match value {
            Some(value) => cmd.env(key, value),
            None => cmd.env_remove(key),
        };
    }
}

/// [`isolate`], and the child never spawns a daemon (`ACQ_NO_SPAWN`).
pub fn offline(cmd: &mut Command, base: &Path) {
    isolate(cmd, base);
    cmd.env("ACQ_NO_SPAWN", "1");
}

// ---- the executables ----------------------------------------------------

/// The `acqd` beside `frontend`, the one it would itself spawn (C82, the
/// locator's one rule). A missing one fails here, before the test runs,
/// naming the build step.
pub fn acqd_beside(frontend: &Path) -> PathBuf {
    acquisition_client::locator::beside(frontend).unwrap_or_else(|e| {
        panic!("{e}: run `cargo build --workspace` first (a `-p` test run does not build acqd)")
    })
}

/// The `acqd` the gate built, for a crate with no binary of its own:
/// `target/<profile>/acqd`, one level above the `deps/` directory this
/// test executable runs from.
pub fn acqd_built() -> PathBuf {
    let exe = std::env::current_exe().expect("current exe");
    let deps_dir = exe
        .parent()
        .unwrap_or_else(|| panic!("{} has no parent directory", exe.display()));
    let profile_dir = deps_dir
        .parent()
        .unwrap_or_else(|| panic!("{} is not under target/<profile>/deps", exe.display()));
    let acqd = profile_dir.join(acquisition_client::locator::ACQD);
    assert!(
        acqd.is_file(),
        "no daemon at {}: run `cargo build --workspace` first (the gate builds before it \
         tests; a `-p` test run does not build it)",
        acqd.display()
    );
    acqd
}

/// The executable `name` beside `exe` (one build wrote both, C82); a
/// missing one fails here, naming the build step.
pub fn sibling(exe: &Path, name: &str) -> PathBuf {
    let path = exe
        .parent()
        .expect("a parent")
        .join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    assert!(
        path.exists(),
        "no {name} beside {} at {}: run `cargo build --workspace` first (C82)",
        exe.display(),
        path.display()
    );
    path
}

/// `acqd` also named beside *this* executable: a test that drives the
/// client in-process judges the daemon's artifact against the `acqd`
/// beside the running executable (C84), and a test executable in `deps/`
/// has none, so a link of that name is placed beside it, pointing one
/// level up at the same file (C82's clause for test executables).
pub fn link_beside_this_test(acqd: &Path) {
    let exe = std::env::current_exe().expect("current exe");
    let link = exe
        .parent()
        .expect("a parent")
        .join(acquisition_client::locator::ACQD);
    place_link(&link, acqd);
    let sibling = acquisition_client::locator::beside(&exe).unwrap_or_else(|e| panic!("{e}"));
    assert!(
        same_file(&sibling, acqd),
        "the sibling the in-process client sees is not the daemon the test starts"
    );
}

/// Place `link` as the daemon under another name: on Unix a symlink one
/// level up, at the same file; on Windows a hard link (a symlink needs
/// Developer Mode there, and one a standard user made is an untrusted
/// mount point to its readers, os error 448), placed again once a build
/// has replaced the daemon. A test running beside this one may place it
/// meanwhile.
#[cfg_attr(unix, allow(unused_variables))]
fn place_link(link: &Path, acqd: &Path) {
    #[cfg(unix)]
    {
        let target = Path::new("..").join(acquisition_client::locator::ACQD);
        if std::fs::read_link(link).ok().as_deref() != Some(target.as_path()) {
            let _ = std::fs::remove_file(link);
            if let Err(e) = std::os::unix::fs::symlink(&target, link)
                && std::fs::read_link(link).ok().as_deref() != Some(target.as_path())
            {
                panic!("placing {} -> {}: {e}", link.display(), target.display());
            }
        }
    }
    #[cfg(windows)]
    if !same_file(link, acqd) {
        let _ = std::fs::remove_file(link);
        if let Err(e) = std::fs::hard_link(acqd, link)
            && !same_file(link, acqd)
        {
            panic!("linking {} to {}: {e}", link.display(), acqd.display());
        }
    }
}

/// Whether two paths open one file, by its id (C84).
fn same_file(a: &Path, b: &Path) -> bool {
    let id = |p: &Path| {
        std::fs::File::open(p)
            .and_then(|f| acquisition_store::world::file_id(&f))
            .ok()
    };
    id(a).is_some() && id(a) == id(b)
}

// ---- processes ----------------------------------------------------------

/// A process the test started and owns (a daemon, a watcher): killed and
/// reaped on drop whichever way the test ends, and named while a test is
/// panicking (C82: a failure says which executable ran).
pub struct Proc(pub Child, pub PathBuf);

impl Proc {
    pub fn id(&self) -> u32 {
        self.0.id()
    }

    /// Wait for the process to exit on its own (after a stop).
    pub fn wait_exit(&mut self, within: Duration) {
        let deadline = Instant::now() + within;
        loop {
            if self.0.try_wait().expect("try_wait").is_some() {
                return;
            }
            assert!(
                Instant::now() < deadline,
                "{} (pid {}) did not exit within {within:?}",
                self.1.display(),
                self.0.id()
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }
}

impl Drop for Proc {
    fn drop(&mut self) {
        if std::thread::panicking() {
            eprintln!(
                "process under test: {} (pid {})",
                self.1.display(),
                self.0.id()
            );
        }
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// The daemon started directly by the test, under the isolation of
/// `base`: stdin and stdout to null, as a lazy spawn's, and stderr piped,
/// so a daemon that does not come up says why ([`spawn_drained`]).
pub fn daemon_command(acqd: &Path, base: &Path) -> Command {
    let mut cmd = Command::new(acqd);
    isolate(&mut cmd, base);
    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    cmd
}

/// Spawn `cmd` (a [`daemon_command`]) and drain its stderr on a thread for
/// the daemon's whole life, so a full pipe never blocks it; what it wrote
/// is kept for a start that times out.
pub fn spawn_drained(mut cmd: Command, acqd: &Path) -> (Proc, Arc<Mutex<Vec<u8>>>) {
    let mut child = cmd
        .spawn()
        .unwrap_or_else(|e| panic!("spawning {}: {e}", acqd.display()));
    let said = Arc::new(Mutex::new(Vec::new()));
    if let Some(mut stderr) = child.stderr.take() {
        let said = Arc::clone(&said);
        std::thread::spawn(move || {
            let mut chunk = [0u8; 4096];
            while let Ok(n) = stderr.read(&mut chunk) {
                if n == 0 {
                    break;
                }
                let mut said = said.lock().unwrap_or_else(|p| p.into_inner());
                if said.len() < 64 * 1024 {
                    said.extend_from_slice(&chunk[..n]);
                }
            }
        });
    }
    (Proc(child, acqd.to_path_buf()), said)
}

fn did_not_come_up(acqd: &Path, base: &Path, said: &Mutex<Vec<u8>>) -> String {
    let said = said.lock().unwrap_or_else(|p| p.into_inner());
    format!(
        "the mock daemon ({}) did not come up under {}; its stderr:\n{}",
        acqd.display(),
        base.display(),
        String::from_utf8_lossy(&said)
    )
}

/// Start the mock daemon under `base` and wait until `acq daemon status
/// --json` (the frontend at `acq`) sees it running, compatible and mock;
/// returns it with its pid. The sibling link is placed first, so an
/// in-process client and the binary agree on the artifact.
pub fn start_daemon(acq: &Path, acqd: &Path, base: &Path) -> (Proc, u64) {
    start_daemon_with(acq, acqd, base, &[])
}

/// [`start_daemon`] with `env` set last on the daemon's own command (the
/// journal path, [`journal_on`]): every other command keeps the isolation
/// list unchanged.
pub fn start_daemon_with(
    acq: &Path,
    acqd: &Path,
    base: &Path,
    env: &[(&str, &str)],
) -> (Proc, u64) {
    link_beside_this_test(acqd);
    let mut cmd = daemon_command(acqd, base);
    cmd.envs(env.iter().copied());
    let (daemon, said) = spawn_drained(cmd, acqd);
    let deadline = Instant::now() + Duration::from_secs(120);
    loop {
        let mut status = Command::new(acq);
        status.args(["daemon", "status", "--json"]);
        isolate(&mut status, base);
        let status = sole_json(&status.output().expect("spawning acq daemon status"));
        if let Some(pid) = status["pid"].as_u64() {
            assert_eq!(status["running"], true, "{status}");
            assert_eq!(status["compatible"], true, "{status}");
            assert_eq!(status["provider"], "mock", "{status}");
            return (daemon, pid);
        }
        assert!(
            Instant::now() < deadline,
            "{}",
            did_not_come_up(acqd, base, &said)
        );
        std::thread::sleep(Duration::from_millis(50));
    }
}

/// Every Unix socket under `dir`, recursively: what a daemon may have
/// bound in the scratch runtime directory.
#[cfg(unix)]
pub fn sockets_under(dir: &Path) -> Vec<PathBuf> {
    use std::os::unix::fs::FileTypeExt;
    let mut found = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return found;
    };
    for entry in entries.flatten() {
        let kind = entry.file_type().unwrap();
        if kind.is_dir() {
            found.extend(sockets_under(&entry.path()));
        } else if kind.is_socket() {
            found.push(entry.path());
        }
    }
    found.sort();
    found
}

/// Windows: there are no socket files, so the endpoints "under `dir`"
/// are the live pipes (C112) of the worlds a test made beside it: every
/// directory up to two levels under `dir`'s parent, the test's scratch
/// base, taken as a root, and its pipe kept when the pipe namespace lists
/// it. Each test has its own base, so tests running at once never see
/// each other's.
#[cfg(windows)]
pub fn sockets_under(dir: &Path) -> Vec<PathBuf> {
    fn dirs(at: &Path, depth: u32, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(at) else {
            return;
        };
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                out.push(entry.path());
                if depth > 1 {
                    dirs(&entry.path(), depth - 1, out);
                }
            }
        }
    }
    let Some(base) = dir.parent() else {
        return Vec::new();
    };
    let mut roots = Vec::new();
    dirs(base, 2, &mut roots);
    let mut found: Vec<PathBuf> = roots
        .iter()
        .filter_map(|root| acquisition_store::world::World::observe_at(root).ok())
        .filter_map(|world| world.socket_path().ok())
        .filter(|pipe| acquisition_client::endpoint::exists(pipe))
        .collect();
    found.sort();
    found.dedup();
    found
}

/// No daemon came up under `base`: nothing bound a socket in the test's
/// scratch runtime directory (C83: a daemon's socket derives from its
/// world into that directory, and nothing else binds there).
pub fn no_daemon_appeared(base: &Path) -> bool {
    sockets_under(&scratch_tmp(base)).is_empty()
}

// ---- the send journal ---------------------------------------------------

/// The journal of a daemon the test started under `base`, the GGG-side
/// contract surface (`TESTING-NOTES.md`): `base/sends.jsonl`.
pub fn journal_path(base: &Path) -> PathBuf {
    base.join("sends.jsonl")
}

/// The knob that turns the journal on at [`journal_path`], for the
/// daemon's own command ([`start_daemon_with`]) or a login that starts it.
pub fn journal_on(base: &Path) -> (&'static str, String) {
    let path = journal_path(base);
    let path = path.to_str().expect("a UTF-8 scratch path").to_string();
    ("ACQ_JOURNAL", path)
}

/// One rate-limit window a send's response reported: `hits` of `max` in
/// `period` seconds under `rule` (`account`, `ip`), and the restriction
/// active, 0 for none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    pub rule: String,
    pub hits: u64,
    pub max: u64,
    pub period: u64,
    pub restricted: u64,
}

/// One journaled send, by field: the daemon's pid, the method, the route
/// (the endpoint key with its account, `stash@ExileTester`), the status,
/// whether it counted against a window, how long it was held before
/// dispatch, and every window its response reported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Send {
    pub pid: u64,
    pub method: String,
    pub route: String,
    /// The request's path (`/stash/Rehearsal/rh-cur1`): where a stash
    /// send's league is, since its route names none.
    pub path: String,
    pub status: Option<u64>,
    pub counted: bool,
    pub wait_ms: u64,
    pub windows: Vec<Window>,
}

fn journal_lines(base: &Path) -> Vec<Value> {
    let path = journal_path(base);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the journal {}: {e}", path.display()));
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            serde_json::from_str::<Value>(l)
                .unwrap_or_else(|e| panic!("a journal line that is not JSON ({e}): {l}"))
        })
        .collect()
}

/// The journal's first line, the header a daemon writes when it opens it
/// (`event`, `pid`, `contract`, `daemon`, `clock`).
pub fn journal_header(base: &Path) -> Value {
    journal_lines(base)
        .into_iter()
        .next()
        .unwrap_or_else(|| panic!("the journal under {} is empty", base.display()))
}

/// The windows of one send's `rate` headers: for each rule
/// `x-rate-limit-rules` names, its `max:period:restriction` list beside its
/// `-state` list's `hits:period:active`.
fn windows(rate: &Value) -> Vec<Window> {
    let header = |name: &str| rate[name].as_str().unwrap_or_default().to_string();
    let triples = |list: &str| -> Vec<[u64; 3]> {
        list.split(',')
            .filter(|w| !w.is_empty())
            .map(|w| {
                let n: Vec<u64> = w
                    .split(':')
                    .map(|x| {
                        x.parse()
                            .unwrap_or_else(|_| panic!("a window {w:?} in {rate}"))
                    })
                    .collect();
                [n[0], n[1], n[2]]
            })
            .collect()
    };
    let mut out = Vec::new();
    for rule in header("x-rate-limit-rules")
        .split(',')
        .filter(|r| !r.is_empty())
    {
        let rule = rule.to_ascii_lowercase();
        let limits = triples(&header(&format!("x-rate-limit-{rule}")));
        let states = triples(&header(&format!("x-rate-limit-{rule}-state")));
        for (limit, state) in limits.iter().zip(&states) {
            out.push(Window {
                rule: rule.clone(),
                hits: state[0],
                max: limit[0],
                period: limit[1],
                restricted: state[2],
            });
        }
    }
    out
}

/// Every send the journal under `base` records, in order, by field; the
/// header and any other event line left out.
pub fn sends(base: &Path) -> Vec<Send> {
    journal_lines(base)
        .into_iter()
        .filter(|v| v.get("route").is_some())
        .map(|v| Send {
            pid: v["pid"]
                .as_u64()
                .unwrap_or_else(|| panic!("a send without a pid: {v}")),
            method: v["method"].as_str().unwrap_or_default().to_string(),
            route: v["route"].as_str().unwrap_or_default().to_string(),
            path: v["path"].as_str().unwrap_or_default().to_string(),
            status: v["status"].as_u64(),
            counted: v["counted"]
                .as_bool()
                .unwrap_or_else(|| panic!("no counted: {v}")),
            wait_ms: v["wait_ms"].as_u64().unwrap_or_default(),
            windows: windows(&v["rate"]),
        })
        .collect()
}

/// The league a send's path names: the segment after `/stash`, a realm
/// segment skipped, as it went on the wire (a space is `%20`); none for
/// a path of another family.
pub fn league_of(path: &str) -> Option<String> {
    let mut parts = path.strip_prefix("/stash/")?.split('/');
    let first = parts.next()?;
    let league = if matches!(first, "xbox" | "sony" | "poe2") {
        parts.next()?
    } else {
        first
    };
    Some(league.to_string())
}

/// The journal invariants a whole session can hold, by field
/// (`TESTING-NOTES.md`, "The journal is the contract surface"): a header
/// and the one daemon `pid` across every send; the first send per
/// `(pid, route)` a HEAD, except the token endpoint and the routes the
/// daemon never probes (`profile`, `league`); `counted` exactly when the
/// method is not HEAD; a HEAD only as that first send, so a new string
/// on a contacted route (a new league) is never probed again; no send
/// answered 429 and every reported window at or under its maximum; and
/// the GETs per route, its account left off, and per league where the
/// path names one ([`league_of`]), exactly `gets`, beside `token_posts`
/// POSTs to the token endpoint and nothing else.
pub fn assert_journal_holds(
    base: &Path,
    pid: u64,
    gets: &[(&str, Option<&str>, usize)],
    token_posts: usize,
) {
    let header = journal_header(base);
    assert_eq!(
        header["event"], "open",
        "the journal's first line: {header}"
    );
    assert_eq!(
        header["pid"].as_u64(),
        Some(pid),
        "the header's daemon: {header}"
    );
    let all = sends(base);
    assert!(
        !all.is_empty(),
        "nothing journaled under {}",
        base.display()
    );
    let name = |route: &str| route.split('@').next().unwrap_or(route).to_string();
    let mut first: Vec<(u64, String)> = Vec::new();
    let mut counted_gets: HashMap<(String, Option<String>), usize> = HashMap::new();
    let mut posts = 0;
    for send in &all {
        assert_eq!(send.pid, pid, "a send from another daemon: {send:?}");
        let key = (send.pid, send.route.clone());
        if !first.contains(&key) {
            first.push(key);
            let probed = !matches!(
                name(&send.route).as_str(),
                "oauth-token" | "profile" | "league"
            );
            if probed {
                assert_eq!(
                    send.method, "HEAD",
                    "the first send on {} is no probe: {send:?}",
                    send.route
                );
            }
        } else {
            assert_ne!(
                send.method, "HEAD",
                "a second probe on a contacted route: {send:?}"
            );
        }
        assert_eq!(
            send.counted,
            send.method != "HEAD",
            "counted is the method's: {send:?}"
        );
        assert_ne!(send.status, Some(429), "a send answered 429: {send:?}");
        for window in &send.windows {
            assert!(
                window.hits <= window.max,
                "a window over its maximum: {send:?}"
            );
        }
        match send.method.as_str() {
            "GET" => {
                *counted_gets
                    .entry((name(&send.route), league_of(&send.path)))
                    .or_default() += 1
            }
            "POST" if name(&send.route) == "oauth-token" => posts += 1,
            "HEAD" => {}
            other => panic!("a {other} send: {send:?}"),
        }
    }
    let mut expected: Vec<((String, Option<String>), usize)> = gets
        .iter()
        .map(|(r, l, n)| ((r.to_string(), l.map(str::to_string)), *n))
        .collect();
    expected.sort();
    let mut got: Vec<((String, Option<String>), usize)> = counted_gets.into_iter().collect();
    got.sort();
    assert_eq!(got, expected, "the GETs per route and league");
    assert_eq!(posts, token_posts, "the token endpoint's POSTs");
}

// ---- output -------------------------------------------------------------

/// Stdout as exactly one JSON document (C11), or the test fails naming
/// what was printed.
pub fn sole_json(out: &Output) -> Value {
    let stdout = String::from_utf8(out.stdout.clone()).expect("stdout is UTF-8");
    serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("stdout is not exactly one JSON document ({e}):\n{stdout}"))
}

/// Both streams as one string, for assertions on prose.
pub fn text(out: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

// ---- the mock login -----------------------------------------------------

/// One plain HTTP GET (loopback only): status, lowercased headers, body.
/// Enough to click the mock's approve link.
pub fn http_get(url: &str) -> (u16, HashMap<String, String>, String) {
    let rest = url.strip_prefix("http://").expect("loopback http url");
    let (host, path) = rest.split_at(rest.find('/').unwrap_or(rest.len()));
    let path = if path.is_empty() { "/" } else { path };
    let mut stream = std::net::TcpStream::connect(host).expect("connecting to mock/daemon");
    write!(
        stream,
        "GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n"
    )
    .unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    let (head, body) = response
        .split_once("\r\n\r\n")
        .unwrap_or((response.as_str(), ""));
    let mut lines = head.lines();
    let status: u16 = lines
        .next()
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| panic!("unparseable response:\n{response}"));
    let headers = lines
        .filter_map(|l| l.split_once(": "))
        .map(|(k, v)| (k.to_ascii_lowercase(), v.to_string()))
        .collect();
    (status, headers, body.to_string())
}

/// Approve a login at the mock as `user` (the mock's own name when
/// `None`): the authorize URL's approve variant, `#` encoded, then its 302
/// followed to the daemon's callback, which is what completes the login.
pub fn approve(authorize_url: &str, user: Option<&str>) {
    let mut approve = authorize_url.replacen("/authorize?", "/approve?", 1);
    if let Some(user) = user {
        approve.push_str(&format!("&user={}", user.replace('#', "%23")));
    }
    let (status, headers, body) = http_get(&approve);
    assert_eq!(status, 302, "approve did not redirect: {body}");
    let (status, _, body) = http_get(&headers["location"]);
    assert_eq!(status, 200, "callback refused: {body}");
}

/// The scripted mock login through a frontend binary (the mock-session
/// skill's recipe): `acq auth --no-browser --json` under the isolation of
/// `base` with `env` set last (a knob the daemon it starts reads), the
/// printed authorize URL approved.
pub fn login(acq: &Path, base: &Path, user: Option<&str>, env: &[(&str, &str)]) {
    let mut cmd = Command::new(acq);
    cmd.args(["auth", "--no-browser", "--json"]);
    isolate(&mut cmd, base);
    let mut child = cmd
        .envs(env.iter().copied())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawning acq auth");
    let mut lines = std::io::BufReader::new(child.stdout.take().unwrap()).lines();
    // What the login printed first, said whole when it is not the URL: a
    // cold-start failure carries its evidence. The child's stderr is
    // inherited, so it is in the test's own output beside this.
    let said = match lines.next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => format!("<stdout unreadable: {e}>"),
        None => "<stdout closed before a line>".to_string(),
    };
    let authorize = serde_json::from_str::<Value>(&said)
        .ok()
        .and_then(|first| first["authorize_url"].as_str().map(str::to_string))
        .unwrap_or_else(|| {
            panic!(
                "`acq auth --no-browser --json` printed no authorize_url first; its first stdout line: {said:?}"
            )
        });
    approve(&authorize, user);
    assert!(child.wait().unwrap().success(), "login did not complete");
}

// ---- the in-process session ---------------------------------------------

static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());

/// A scratch world held for the test's duration, the process environment
/// pointing at it while the lock is held: for the files that drive a
/// `Client` or a `Frontend` in-process, which read the world and derive
/// the socket from the environment (C83). Declared before the daemon, so
/// the daemon is gone first.
pub struct Session {
    pub base: PathBuf,
    _scratch: Scratch,
    _lock: MutexGuard<'static, ()>,
}

/// Take the one lock and apply [`isolation`] to this process, the world
/// at `base/store` (created: a scripted peer has no daemon to create it,
/// and the world it claims must exist to be this process's).
pub fn session(tag: &str) -> Session {
    let lock = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let scratch = scratch(tag);
    let base = scratch.0.clone();
    std::fs::create_dir_all(base.join("store")).expect("scratch dir");
    // SAFETY: every test that touches the environment takes
    // `ONE_AT_A_TIME` first, and a file that does is its own test binary
    // (C122), so no other thread reads these variables while they change.
    unsafe {
        for (key, value) in isolation(&base.join("store"), &base) {
            match value {
                Some(value) => std::env::set_var(key, value),
                None => std::env::remove_var(key),
            }
        }
    }
    // The daemon's step, so a socket path resolves in this process too:
    // a client never creates the runtime directory (C83). Unix only, as
    // the daemon's is: on Windows the endpoint is a pipe and the daemon
    // makes no runtime directory (C112, C132).
    if cfg!(unix) {
        acquisition_store::world::app_runtime_dir().expect("the scratch runtime directory");
    }
    Session {
        base,
        _scratch: scratch,
        _lock: lock,
    }
}

impl Session {
    /// Point this process's world at `root` (the lock is held).
    pub fn point_world_at(&self, root: &Path) {
        // SAFETY: the session's lock is held (`session`).
        unsafe {
            std::env::set_var("ACQ_STORE_DIR", root);
        }
    }

    /// Set one knob of this process over the isolation list's value, or
    /// remove it (`None`) so the code's default applies (the lock is
    /// held): a deadline's pin sets its own (C137).
    pub fn set_knob(&self, key: &str, value: Option<&str>) {
        // SAFETY: the session's lock is held (`session`).
        unsafe {
            match value {
                Some(value) => std::env::set_var(key, value),
                None => std::env::remove_var(key),
            }
        }
    }
}

/// Start the mock daemon for the session and wait until the in-process
/// client observes it compatible; the sibling link is placed first.
pub async fn start_daemon_in_process(s: &Session, acqd: &Path) -> Proc {
    link_beside_this_test(acqd);
    let (daemon, said) = spawn_drained(daemon_command(acqd, &s.base), acqd);
    let deadline = Instant::now() + Duration::from_secs(120);
    loop {
        match Client::observe().await {
            Ok(Observed::Compatible(_)) => return daemon,
            Ok(Observed::Incompatible(found)) => {
                panic!("{} is not this build's daemon: {found}", acqd.display())
            }
            _ => {}
        }
        assert!(
            Instant::now() < deadline,
            "{}",
            did_not_come_up(acqd, &s.base, &said)
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// The compatible daemon of the session, observed.
pub async fn compatible_client() -> Client {
    match Client::observe().await.expect("observe") {
        Observed::Compatible(client) => client,
        Observed::Absent => panic!("no daemon"),
        Observed::Incompatible(found) => panic!("{found}"),
        Observed::Unresponsive(found) => panic!("{found}"),
    }
}

/// Log in over the protocol as `user` (the mock's own name when `None`):
/// start the flow, [`approve`] it, and wait for the daemon to register
/// this flow's own completion.
pub async fn login_in_process(client: &mut Client, user: Option<&str>) {
    let url = match client.request(&Request::AuthStart).await.unwrap() {
        Response::AuthUrl { authorize_url } => authorize_url,
        other => panic!("{other:?}"),
    };
    let user = user.map(str::to_string);
    tokio::task::spawn_blocking(move || approve(&url, user.as_deref()))
        .await
        .expect("the approve step");
    let deadline = Instant::now() + Duration::from_secs(120);
    loop {
        match client.request(&Request::AuthStatus).await.unwrap() {
            Response::Auth {
                pending: false,
                login_ok: Some(_),
                ..
            } => return,
            Response::Auth {
                login_error: Some(e),
                pending: false,
                ..
            } => panic!("login failed: {e}"),
            _ => {}
        }
        assert!(Instant::now() < deadline, "login did not complete");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// Poll `list` until every job the daemon holds, `expected` of them, is
/// terminal.
pub async fn wait_all_terminal(client: &mut Client, expected: usize) {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        let jobs = match client.request(&Request::List).await.expect("list") {
            Response::Jobs { jobs } => jobs,
            other => panic!("{other:?}"),
        };
        if jobs.len() == expected && jobs.iter().all(|j| j.state.is_terminal()) {
            return;
        }
        assert!(Instant::now() < deadline, "jobs did not finish");
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
