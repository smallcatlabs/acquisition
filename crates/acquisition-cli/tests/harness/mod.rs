//! Shared process-test harness for the tests that own a daemon: `acq`
//! under one isolation (a scratch world, runtime directory and log
//! directory under `base`, the live-run knobs scrubbed), the `acqd`
//! beside it (C82), a daemon the test starts and owns, and the sibling
//! symlink a test executable needs to judge that daemon's artifact
//! in-process (C84). One module per crate (the MCP crate's is
//! `acquisition-mcp/tests/harness`, the client's contract test carries
//! its own); each test binary uses the slice it needs.
#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};

use serde_json::Value;

/// The test's scratch directory, removed on drop — declare it before
/// the daemon, so the daemon is gone first.
pub struct Scratch(pub PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// A fresh scratch directory directly under the platform temp dir (the
/// socket path must stay short: Unix sockets cap ~104 bytes), named by
/// `tag` and the test process.
pub fn scratch(tag: &str) -> Scratch {
    let base = std::env::temp_dir().join(format!("acq-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    Scratch(base)
}

/// The scratch temp and runtime directory of one test, under `base`:
/// `TMPDIR` (macOS's runtime fallback) and `XDG_RUNTIME_DIR` (Linux's
/// runtime directory) both point here, so the sockets the daemons bind
/// — and, for a daemon a test kills rather than stops, the socket files
/// it leaves — never touch the user's own runtime directory (C83).
/// Created here, since the runtime directory is made under an existing
/// parent.
pub fn scratch_tmp(base: &Path) -> PathBuf {
    let tmp = base.join("tmp");
    let _ = std::fs::create_dir_all(&tmp);
    tmp
}

/// One isolation for `acq` and the daemon alike: the world at
/// `base/store` (its socket derives from it, C83), the runtime and log
/// directories under `base`, no keyring, no journal, a short idle
/// shutdown, and the live-run and selection knobs of the parent shell
/// scrubbed.
pub fn isolate(cmd: &mut Command, base: &Path) {
    isolate_in(cmd, base, "store");
}

/// [`isolate`] with the world at `base/<store>`: for tests that run two
/// worlds under one scratch directory.
pub fn isolate_in(cmd: &mut Command, base: &Path, store: &str) {
    cmd.env("ACQ_STORE_DIR", base.join(store))
        .env("TMPDIR", scratch_tmp(base))
        .env("XDG_RUNTIME_DIR", scratch_tmp(base))
        .env("ACQ_LOG_DIR", base.join("logs"))
        .env("ACQ_NO_KEYRING", "1")
        .env("ACQ_JOURNAL", "0")
        .env("ACQ_IDLE_SHUTDOWN", "30");
    for var in [
        "ACQ_GGG",
        "ACQ_ACCOUNT",
        "ACQ_TRIPWIRE",
        "ACQ_MAX_SENDS",
        "ACQ_NO_SPAWN",
    ] {
        cmd.env_remove(var);
    }
    cmd.env("ACQ_PROVIDER", "mock");
}

/// The binary under test, isolated under `base`.
pub fn command(base: &Path, args: &[&str]) -> Command {
    command_of(Path::new(env!("CARGO_BIN_EXE_acq")), base, args)
}

/// The same isolation for an `acq` at another path (a copy staged for
/// the artifact dimension).
pub fn command_of(exe: &Path, base: &Path, args: &[&str]) -> Command {
    let mut cmd = Command::new(exe);
    cmd.args(args);
    isolate(&mut cmd, base);
    cmd
}

/// The binary under test with the world at `base/<store>`.
pub fn command_in(base: &Path, store: &str, args: &[&str]) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_acq"));
    cmd.args(args);
    isolate_in(&mut cmd, base, store);
    cmd
}

pub fn acq(base: &Path, args: &[&str]) -> Output {
    command(base, args).output().expect("spawning acq")
}

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

/// A process the test started and owns (a daemon, or a watcher): killed
/// and waited for on drop whichever way the test ends, and named while
/// a test is panicking (C82: a failure says which executable ran).
pub struct Proc(pub Child, pub PathBuf);

impl Proc {
    pub fn id(&self) -> u32 {
        self.0.id()
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

/// The daemon `acq` would itself spawn: the `acqd` beside the binary
/// under test (C82, the locator's one rule). A missing one fails here,
/// before the test runs, naming the build step.
pub fn acqd() -> PathBuf {
    acquisition_client::locator::beside(Path::new(env!("CARGO_BIN_EXE_acq")))
        .unwrap_or_else(|e| panic!("{e}"))
}

/// [`acqd`], also placed beside *this* executable: a test that drives
/// the client in-process judges the daemon's artifact against the
/// `acqd` beside the running executable (C84), and a test executable in
/// `deps/` has none — so a link of that name is placed beside it (`place_link`),
/// pointing one level up at the same file: the test executable names
/// the daemon its build wrote on both paths (C82's clause for test
/// executables).
pub fn acqd_beside_this_test() -> PathBuf {
    let acqd = acqd();
    let exe = std::env::current_exe().expect("current exe");
    let link = exe
        .parent()
        .expect("a parent")
        .join(acquisition_client::locator::ACQD);
    place_link(&link, &acqd);
    assert!(
        same_file(&link, &acqd),
        "the sibling the in-process client sees is not the daemon acq would spawn"
    );
    acqd
}

/// The daemon started directly by the test, which owns its pid, under
/// the same isolation as [`command`]; its stdio to null, as a lazy
/// spawn's.
pub fn daemon_command(base: &Path, acqd: &Path) -> Command {
    let mut cmd = Command::new(acqd);
    isolate(&mut cmd, base);
    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    cmd
}

/// Start the mock daemon under `base` and wait until `acq daemon status`
/// sees it running, compatible and mock; returns it with its pid. The
/// sibling symlink is placed first, so the in-process client and the
/// binary agree on the artifact.
pub fn start_daemon(base: &Path) -> (Proc, u64) {
    let acqd = acqd_beside_this_test();
    let child = daemon_command(base, &acqd)
        .spawn()
        .unwrap_or_else(|e| panic!("spawning {}: {e}", acqd.display()));
    let daemon = Proc(child, acqd.clone());
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        let out = acq(base, &["daemon", "status", "--json"]);
        let status = sole_json(&out);
        if let Some(pid) = status["pid"].as_u64() {
            assert_eq!(status["running"], true, "{status}");
            assert_eq!(status["compatible"], true, "{status}");
            assert_eq!(status["provider"], "mock", "{status}");
            return (daemon, pid);
        }
        assert!(
            Instant::now() < deadline,
            "the mock daemon ({}) did not come up",
            acqd.display()
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
/// are the live pipes (C112) of the worlds a test made beside it — every
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
        .filter_map(|root| acquisition_store::world::World::at(root).ok())
        .filter_map(|world| world.socket_path().ok())
        .filter(|pipe| acquisition_client::endpoint::exists(pipe))
        .collect();
    found.sort();
    found.dedup();
    found
}

/// One plain HTTP GET (loopback only): status, lowercased headers, body.
/// Enough to click the mock's approve link.
fn http_get(url: &str) -> (u16, std::collections::HashMap<String, String>, String) {
    use std::io::{Read as _, Write as _};
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

/// The scripted mock login (the mock-session skill's recipe) as `user`:
/// `acq auth --no-browser --json`, the printed authorize URL approved at
/// the mock, the 302 followed to the daemon's callback.
pub fn login(base: &Path, user: &str) {
    login_with(base, user, &[]);
}

/// [`login`] with `env` set on the `acq auth` that starts the daemon: a
/// knob the daemon reads at its start (`ACQ_IDLE_SHUTDOWN`) goes here.
pub fn login_with(base: &Path, user: &str, env: &[(&str, &str)]) {
    use std::io::BufRead as _;
    let mut child = command(base, &["auth", "--no-browser", "--json"])
        .envs(env.iter().copied())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawning acq auth");
    let mut lines = std::io::BufReader::new(child.stdout.take().unwrap()).lines();
    let first: Value = serde_json::from_str(&lines.next().unwrap().unwrap()).unwrap();
    let authorize = first["authorize_url"].as_str().expect("authorize_url");
    let approve = format!(
        "{}&user={user}",
        authorize.replace("/authorize?", "/approve?")
    );
    let (status, headers, body) = http_get(&approve);
    assert_eq!(status, 302, "approve did not redirect: {body}");
    let (status, _, body) = http_get(&headers["location"]);
    assert_eq!(status, 200, "callback refused: {body}");
    assert!(child.wait().unwrap().success(), "login did not complete");
}

/// Place `link` as the daemon under another name (C82's clause for test
/// executables): on Unix a symlink one level up, at the same file; on
/// Windows a hard link — a symlink needs Developer Mode there, and one a
/// standard user made is an untrusted mount point to its readers (os
/// error 448) — placed again once a build has replaced the daemon. A
/// test running beside this one may place it meanwhile.
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
