//! The world (C83; C1: "the store holds … the world (root, locks, socket
//! name)"): pure path functions and the two lock files, no IPC. A world
//! is a canonical, provider-neutral store root — `ACQ_STORE_DIR` or the
//! platform data directory, canonicalised — above the `<provider>/`
//! directories the store keeps facts, intent and `daemon.db` in. Every
//! side of the socket computes the same paths from the same environment:
//! the daemon (`acquisition-daemon`) to create, lock and write, the
//! client (`acquisition-client`) to compare the world a daemon reports
//! with its own and to read the log after a failed spawn, the frontends
//! to print them.
//!
//! # Decisions as recorded
//!
//! The rulings are the decision registry — `decisions/daemon.md` for this
//! area (`C<n>`); what follows is the entry's full text as recorded
//! there, kept beside the code that implements it.
//!
//! ## C83 — A world is a canonical, provider-neutral store root
//!
//! **C83 — A world is a canonical, provider-neutral store root; its
//! daemon holds an exclusive lock on it for its lifetime, and in real
//! mode a per-OS-user lock no root bypasses.** A use path creates the
//! root, then canonicalises and locks; observation creates nothing. Its
//! endpoint is C112; `hello` names the root and a client refuses a daemon
//! on another world. Durable state (`daemon.db`, rails state) lives in
//! the world; the log and journal are bounded diagnostics elsewhere.
//! *Why:* C6 and C31 were held by documentation, and the tripwire's state
//! lived in a directory the OS clears at reboot. *Details:* `world.rs`
//! doc. Ruled 2026-09-09; amended 2026-10-14 (the endpoint to C112).
//!
//! ## C112 — The daemon's endpoint
//!
//! **C112 — The daemon's endpoint is derived from the world and the OS
//! user, never chosen by hand (`ACQ_SOCKET` is gone); only that user can
//! serve or reach it, and a client speaks to it only after verifying the
//! server runs as that user.** *Why:* a socket name is discovery, not
//! ownership; on Windows a named pipe's namespace is machine-wide, so
//! privacy is the endpoint's property, not a directory's. *Details:*
//! `world.rs` doc (each platform's arm). Ruled 2026-10-14, splitting
//! C83's socket clause (owner: "yes").
//!
//! On Unix the arm is the socket below, in the private per-user runtime
//! directory a client verifies before it connects; the Windows arm, a
//! named pipe under a user-only ACL whose server's process user a client
//! checks, is step 0 of `PLATFORMS-SLICE.md` at `b6197a82`.
//!
//! ## C132 — The real-mode lock's path
//!
//! **C132 — The real-mode lock's path derives from the OS user alone, in
//! the user's own data directory.** The home comes from the password
//! database (on Windows, the profile directory from the process token),
//! never from a variable; the lock sits in a private `run` directory
//! beside the default store; the socket stays in the runtime directory
//! (C112, the derived endpoint). Another user or mount namespace stays
//! external concurrency (C31, one daemon, many sessions); a home that
//! cannot hold the lock means no real-mode daemon, refused by name.
//! *Why:* every temp directory is moved by a variable or age-cleaned
//! under a held lock, and two locks are two gates.
//!
//! ## C83 — as built (the daemon split's steps 5 and 6)
//!
//! - **The root.** [`intended_root`] is the path the environment names:
//!   `ACQ_STORE_DIR` (a relative value made absolute against the current
//!   directory, so the same spelling crosses into a spawned `acqd`), else
//!   the platform data directory's `store` — the base [`store_dir`] has
//!   always put the provider directories under. [`World::create`] is the
//!   use path: it creates the root (mode 0700 on Unix) and canonicalises
//!   it; [`World::observe`] is the observation path: it canonicalises
//!   what exists and creates nothing, so a missing root is an absent
//!   world, never a new one; the use path sets the root to mode 0700 on
//!   every start, not only when it creates it, and refuses if it cannot
//!   (a root that drifted, or was made before this rule, is brought to
//!   the documented state).
//!   `fs::canonicalize` is what makes two spellings of one directory one
//!   world — a symlinked data directory, macOS's `/var` and
//!   `/private/var` — and what a daemon reports in `hello` (`world`),
//!   which a client compares with its own before any other dimension. A
//!   root that is not valid UTF-8 is refused rather than carried lossily:
//!   the name crosses the wire as a string, and two distinct byte paths
//!   must never read as one world (review 2026-09-11).
//! - **The world lock.** [`World::lock_path`] is `<root>/daemon.lock`;
//!   [`Lock::acquire`] takes an exclusive advisory lock on it (`flock`,
//!   through `std::fs::File::try_lock`) and writes the holder's pid into
//!   the file; the lock lives as long as the [`Lock`] — the daemon keeps
//!   it for its lifetime, and the kernel releases it however the process
//!   ends. A second daemon on the root fails to acquire, reads the pid,
//!   and refuses to start naming the holder ([`LockError`]). This is what
//!   makes C6's "one daemon per store directory" structure rather than a
//!   sentence: two daemons on one `daemon.db` would each restore and run
//!   the same queue.
//! - **The real-mode lock** (C132, below). [`real_mode_lock_path`] is
//!   `ggg.lock` in a private `run` directory of the user's own data
//!   directory, under the home the system records for this OS user —
//!   `pw_dir` from the password database on Unix, the process token's
//!   profile directory on Windows — and never a variable:
//!   `~/Library/Application Support/com.smallcatlabs.acq/run` on macOS,
//!   `~/.local/share/acq/run` on Linux,
//!   `<profile>\AppData\Local\smallcatlabs\acq\run` on Windows. `run` is
//!   held to [`private_dir`]'s checks (made 0700 in the creating call, no
//!   symlink, this user's, exactly 0700 or restored or refused). Every
//!   real-mode daemon takes it whatever its root and whatever its
//!   environment, so two live-test roots, or a terminal and a cron job
//!   with different `TMPDIR` or `XDG_RUNTIME_DIR`, cannot make two GGG
//!   gates (C31's Cloudflare bound is per-process state). Per user, not
//!   per machine: another OS user, another mount namespace or sandbox
//!   (each its own home), the C++ Acquisition and other machines behind
//!   the same address are outside it — C31 names them as external
//!   concurrency the tripwire exists for. A home that cannot hold it (no
//!   entry, no directory, a `run` refused, a lock the filesystem refuses)
//!   is no real-mode daemon, refused by name; there is no fallback. The
//!   mock never takes it.
//! - **Durable state in the world.** `daemon.db` (`jobs::daemon_db_path`)
//!   and the rails state ([`World::rails_state_path`]:
//!   `<root>/<provider>/rails.json`) live beside the account files, where
//!   a reboot cannot clear them.
//! - **Diagnostics elsewhere, bounded.** The daemon log and the default
//!   send journal live under [`log_base_dir`] — `ACQ_LOG_DIR`, else the
//!   platform's log directory (`~/Library/Logs/acq` on
//!   macOS, the XDG state directory's `log` on Linux) — one subdirectory
//!   per world ([`World::id`], twelve hex digits of the canonical root's
//!   SHA-256, beside a `world` file naming the root for a human browsing
//!   there) and provider: [`World::log_path`] is
//!   `<base>/<id>/<provider>/daemon.log`, [`World::journal_path`]
//!   `<base>/<id>/<provider>/sends.jsonl`. They are append-only and
//!   never durable state; the daemon bounds them (rotated once at its
//!   start past a cap — the mechanism and the size are the daemon's,
//!   `daemon/start.rs`). `ACQ_JOURNAL` still overrides the journal — a live run
//!   points it into its evidence directory, where the ledger cites it —
//!   and `0` disables it (`rails.rs`).
//! - **The socket.** [`World::socket_path`] is `<runtime>/<id>.sock` —
//!   the world's twelve-hex id in the private per-user runtime directory
//!   (the split's step 6). Nothing chooses it by hand, and two spellings
//!   of one root reach one socket because the id is the canonical root's. Short under the platform's defaults: the
//!   runtime directory is `$XDG_RUNTIME_DIR` (`/run/user/<uid>`) or
//!   macOS's fixed-shape `$TMPDIR`, so the whole path is about 75 bytes
//!   under the macOS fallback — but both variables are the
//!   environment's, so the derivation ([`World::socket_path_under`])
//!   refuses by name a path over [`SOCKET_PATH_MAX`] or one that is not
//!   valid UTF-8 (the socket is named in every report; Linux permits
//!   other bytes in `XDG_RUNTIME_DIR`), rather than leaving the first to
//!   `bind`'s `ENAMETOOLONG` and the second to a report that cannot be
//!   built. The daemon creates the runtime directory
//!   ([`app_runtime_dir`]) before it binds; a client only verifies it
//!   ([`existing_private_dir`]: this user's, 0700, no symlink) before it
//!   connects, so a socket in a directory another user made is never
//!   used, and an observation creates nothing — no runtime directory yet
//!   is no daemon yet. A world that does not exist has no socket: a use
//!   door creates the root first, an observer reports absence.
//!
//! ## The Windows arm (`PLATFORMS-SLICE.md` at `b6197a82`, step 0)
//!
//! - **The endpoint** is a named pipe, not a socket (C112;
//!   [`World::socket_path`], `world_windows`).
//! - **The runtime directory** is `%TEMP%\acq` — per user, under the
//!   profile — and is not made by the daemon: the endpoint is a pipe and the
//!   real-mode lock is in the profile's `AppData\Local` (C132), its
//!   profile directory read from the process token
//!   (`world_windows::profile_dir`); the owner and mode checks of
//!   [`private_dir`] are Unix-only.
//! - **The locks** are `std`'s file locks, which on Windows are mandatory:
//!   the holder's pid is written to `<lock>.pid` beside each lock as well,
//!   where [`Lock::holder`] reads it.
//! - **The root** is canonicalised without the verbatim `\\?\` prefix
//!   where a plain spelling exists, so the name a user types is the name
//!   `hello` carries and the id hashes.
//! - **Every reported path** is spelled the same way ([`canonical`]), and
//!   a file's id ([`file_id`]) is its volume serial number and file
//!   index, read from the open handle.

use std::fs::{File, OpenOptions, TryLockError};
use std::io::{Read as _, Write as _};
use std::path::{Path, PathBuf};

use directories::{BaseDirs, ProjectDirs};
use sha2::{Digest, Sha256};

/// The `directories` identity every platform path derives from.
const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "smallcatlabs";
const APPLICATION: &str = "acq";

fn project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
}

/// The world's root before canonicalisation: `ACQ_STORE_DIR` made
/// absolute, else the platform data directory's `store` (no home
/// directory at all — a bare service account — falls back to `store`
/// under the current directory). What [`store_dir`] has always put the
/// provider directories under; the daemon and the frontends read the
/// same value.
pub fn intended_root() -> PathBuf {
    match std::env::var_os("ACQ_STORE_DIR") {
        Some(d) => absolute(Path::new(&d)),
        None => default_root(),
    }
}

/// The root a world has when `ACQ_STORE_DIR` names none: the platform
/// data directory's `store` (or `store` under the current directory with
/// no home at all), made absolute.
pub fn default_root() -> PathBuf {
    absolute(
        &project_dirs()
            .map(|p| p.data_dir().join("store"))
            .unwrap_or_else(|| PathBuf::from("store")),
    )
}

/// `path` made absolute against the current directory, lexically —
/// nothing is resolved or created.
fn absolute(path: &Path) -> PathBuf {
    if path.is_absolute() {
        return path.to_path_buf();
    }
    match std::env::current_dir() {
        Ok(cwd) => cwd.join(path),
        Err(_) => path.to_path_buf(),
    }
}

/// The provider's store directory: `<intended root>/<provider>`, the
/// path the daemon writes under and the frontends read; not
/// canonicalised (a read needs no world). One directory per provider so
/// mock data never mixes with real.
pub fn store_dir(provider: &str) -> PathBuf {
    intended_root().join(provider)
}

/// Why a world could not be resolved: the path the environment named
/// and what went wrong with it.
#[derive(Debug, Clone)]
pub struct WorldError {
    pub intended: PathBuf,
    pub io: String,
    /// True when the root does not exist and this was an observation,
    /// which creates nothing.
    pub absent: bool,
}

impl std::fmt::Display for WorldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.absent {
            write!(
                f,
                "no world at {} (the store root does not exist; a job command creates it)",
                self.intended.display()
            )
        } else {
            write!(
                f,
                "could not resolve the store root {}: {}",
                self.intended.display(),
                self.io
            )
        }
    }
}

impl std::error::Error for WorldError {}

/// A world: a canonical store root. Constructed only by [`World::create`]
/// (the use path) or [`World::observe`] (creates nothing), so a `World`
/// in hand is a directory that exists, named the one way the OS names it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
    root: PathBuf,
    /// The root as a string: exact, since a root is valid UTF-8 by
    /// construction.
    name: String,
}

impl World {
    /// The use path (C83): create the intended root if it is missing,
    /// set it to mode 0700 on Unix every time (the store holds intent and
    /// `daemon.db`; a root that drifted is brought back to that state),
    /// then canonicalise it. A failure to create or to set the mode is a
    /// refusal, never ignored.
    pub fn create() -> Result<World, WorldError> {
        Self::create_at(&intended_root())
    }

    /// C147, a door takes the world it acts on: [`World::create`] at a
    /// root the caller names (made absolute against the current
    /// directory) rather than the one the environment names.
    pub fn create_at(root: &Path) -> Result<World, WorldError> {
        let intended = absolute(root);
        let failed = |io: std::io::Error| WorldError {
            intended: intended.clone(),
            io: io.to_string(),
            absent: false,
        };
        if !intended.is_dir() {
            std::fs::create_dir_all(&intended).map_err(failed)?;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&intended, std::fs::Permissions::from_mode(0o700))
                .map_err(failed)?;
        }
        Self::canonical(intended)
    }

    /// The observation path (C83): the world as it exists, or an error
    /// saying it does not — nothing is created.
    pub fn observe() -> Result<World, WorldError> {
        Self::observe_at(&intended_root())
    }

    /// C147, a door takes the world it acts on: [`World::observe`] at a
    /// root the caller names (made absolute against the current
    /// directory) — a daemon, a frontend or a test that names its world
    /// rather than reading the environment's. Creates nothing.
    pub fn observe_at(root: &Path) -> Result<World, WorldError> {
        let intended = absolute(root);
        if !intended.exists() {
            return Err(WorldError {
                intended,
                io: "not found".into(),
                absent: true,
            });
        }
        Self::canonical(intended)
    }

    fn canonical(intended: PathBuf) -> Result<World, WorldError> {
        let root = intended.canonicalize().map_err(|e| WorldError {
            intended: intended.clone(),
            io: e.to_string(),
            absent: e.kind() == std::io::ErrorKind::NotFound,
        })?;
        let root = without_verbatim_prefix(root);
        if !root.is_dir() {
            return Err(WorldError {
                intended,
                io: "not a directory".into(),
                absent: false,
            });
        }
        let Some(name) = root.to_str().map(str::to_string) else {
            return Err(WorldError {
                intended,
                io: "the store root is not valid UTF-8; it names the world on the wire and must be"
                    .into(),
                absent: false,
            });
        };
        Ok(World { root, name })
    }

    /// The canonical root: what `hello` carries and a client compares.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The root as `hello` carries it: exact, since a root is valid UTF-8
    /// by construction.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Twelve hex digits of the SHA-256 of the canonical root's exact
    /// bytes: the world's short id, which names its subdirectory under the
    /// log base directory and its socket.
    pub fn id(&self) -> String {
        let digest = Sha256::digest(self.name.as_bytes());
        digest.iter().take(6).map(|b| format!("{b:02x}")).collect()
    }

    /// `<root>/<provider>`: the provider's directory in this world,
    /// where the account files, `accounts.json`, `daemon.db` and the
    /// rails state live.
    pub fn provider_dir(&self, provider: &str) -> PathBuf {
        self.root.join(provider)
    }

    /// `<root>/daemon.lock`: the world lock's file.
    pub fn lock_path(&self) -> PathBuf {
        self.root.join("daemon.lock")
    }

    /// `<root>/<provider>/rails.json`: the tripwire's trip, the block
    /// mark and the refresh-failed marks, durable in the world, in the
    /// format [`RailsState`] owns (`rails.rs` reads and writes it).
    pub fn rails_state_path(&self, provider: &str) -> PathBuf {
        self.provider_dir(provider).join("rails.json")
    }

    /// `<log base>/<id>/<provider>`: where this world's diagnostics for
    /// one provider live.
    pub fn log_dir(&self, provider: &str) -> PathBuf {
        log_base_dir().join(self.id()).join(provider)
    }

    /// The daemon log: `<log dir>/daemon.log`.
    pub fn log_path(&self, provider: &str) -> PathBuf {
        self.log_dir(provider).join("daemon.log")
    }

    /// The default send journal: `<log dir>/sends.jsonl` (`ACQ_JOURNAL`
    /// overrides it; `0` disables it — read in `rails.rs`).
    pub fn journal_path(&self, provider: &str) -> PathBuf {
        self.log_dir(provider).join("sends.jsonl")
    }

    /// `<log base>/<id>/world`: a file naming this world's root, written
    /// by the daemon beside its logs so a person browsing the log
    /// directory can tell the hashed subdirectories apart.
    pub fn log_marker_path(&self) -> PathBuf {
        log_base_dir().join(self.id()).join("world")
    }

    /// `<id>.sock`: the socket's file name, derived from the canonical
    /// root and nothing else.
    pub fn socket_name(&self) -> String {
        format!("{}.sock", self.id())
    }

    /// The socket this world's daemon listens on and its clients connect
    /// to (C83): [`socket_name`](Self::socket_name) in the private
    /// per-user runtime directory, which must be nameable
    /// ([`Self::socket_path_under`]: valid UTF-8, under the cap) and must
    /// already exist and be this user's ([`existing_private_dir`]) —
    /// nothing is created here, so an observer that finds no runtime
    /// directory finds no daemon (`NotFound`), and a socket in a directory
    /// someone else made is never used. The daemon makes the directory
    /// first ([`app_runtime_dir`]).
    #[cfg(unix)]
    pub fn socket_path(&self) -> std::io::Result<PathBuf> {
        let runtime = runtime_dir_path();
        let path = self.socket_path_under(&runtime)?;
        existing_private_dir(&runtime)?;
        Ok(path)
    }

    /// The Windows arm (C112): the named pipe
    /// `\\.\pipe\acq-<SID>-<world id>` — this user's SID and this
    /// world's id, so two users on one world and two worlds of one user
    /// never meet (`world_windows`). A pipe has no directory to verify;
    /// its privacy is its DACL and the client's check of the server.
    #[cfg(windows)]
    pub fn socket_path(&self) -> std::io::Result<PathBuf> {
        let sid = crate::world_windows::current_user_sid()?;
        Ok(PathBuf::from(format!(r"\\.\pipe\acq-{sid}-{}", self.id())))
    }

    /// The pure derivation: this world's socket under `runtime`, refused
    /// by name when the path is not valid UTF-8 (it names the socket in
    /// every report, on the wire as a string — a runtime directory the
    /// environment names in other bytes cannot) or longer than
    /// [`SOCKET_PATH_MAX`]. Nothing is looked at or created.
    pub fn socket_path_under(&self, runtime: &Path) -> std::io::Result<PathBuf> {
        let path = runtime.join(self.socket_name());
        if path.to_str().is_none() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "the runtime directory {} is not valid UTF-8 (XDG_RUNTIME_DIR or TMPDIR); it names the socket in every report and must be",
                    runtime.display()
                ),
            ));
        }
        let len = path.as_os_str().len();
        if len > SOCKET_PATH_MAX {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "the socket path {} is {len} bytes; a Unix socket path may be at most {SOCKET_PATH_MAX} (the runtime directory is too deep)",
                    path.display()
                ),
            ));
        }
        Ok(path)
    }
}

/// The longest path a Unix socket may have: `sun_path` is 104 bytes on
/// macOS including its terminator (108 on Linux), so 103 is the bound
/// that holds on both. [`World::socket_path`] refuses a longer one by
/// name instead of letting `bind` fail with `ENAMETOOLONG`.
pub const SOCKET_PATH_MAX: usize = 103;

/// Where this application's private per-user runtime directory is,
/// computed and not touched: `$XDG_RUNTIME_DIR/acq` where the platform
/// provides that directory (Linux; it is per user by the XDG spec), else
/// `<temp dir>/acq-<uid>` — the temp directory is shared on Linux
/// (`/tmp`) and per user on macOS (`$TMPDIR`), and the uid in the name
/// keeps the fallback per user either way.
fn runtime_dir_path() -> PathBuf {
    match BaseDirs::new().and_then(|b| b.runtime_dir().map(Path::to_path_buf)) {
        Some(runtime) => runtime.join("acq"),
        // Windows: `%TEMP%` is under the user's profile, per user by
        // construction; it holds nothing the daemon makes — C112 puts the
        // endpoint in the pipe namespace, and C132 the real-mode lock in
        // the profile's `AppData\Local`.
        None if cfg!(windows) => std::env::temp_dir().join("acq"),
        None => std::env::temp_dir().join(format!("acq-{}", current_uid())),
    }
}

/// `\\?\C:\…` as `C:\…` and `\\?\UNC\server\share` as
/// `\\server\share` (Windows; elsewhere the path as given):
/// `canonicalize` returns the verbatim form, and the world's name — what
/// `hello` carries, what the id hashes, what every report prints — is the
/// form a user types. A verbatim path with no plain spelling (a component
/// the plain form cannot carry) stays verbatim.
fn without_verbatim_prefix(path: PathBuf) -> PathBuf {
    #[cfg(windows)]
    {
        use std::path::{Component, Prefix};
        let plain = match path.components().next() {
            Some(Component::Prefix(p)) => match p.kind() {
                Prefix::VerbatimDisk(_) => path.to_str().map(|s| s[4..].to_string()),
                Prefix::VerbatimUNC(..) => path.to_str().map(|s| format!(r"\\{}", &s[8..])),
                _ => None,
            },
            _ => None,
        };
        if let Some(plain) = plain
            && plain.len() < 260
            && !plain.split('\\').any(|c| c == "." || c == "..")
        {
            return PathBuf::from(plain);
        }
    }
    path
}

/// `path` canonicalised, in the form a user types
/// (`without_verbatim_prefix`): the spelling of every path a report
/// prints — the world's root, the daemon's executable, the sibling it is
/// judged against.
pub fn canonical(path: &Path) -> std::io::Result<PathBuf> {
    Ok(without_verbatim_prefix(path.canonicalize()?))
}

/// The id of an open file, which `FileIdentity` (the protocol crate)
/// carries as `dev` and `ino` (C84): the device and inode on Unix, the
/// volume serial number and file index on Windows — so a copy is another
/// file on both, however its length and modification time compare.
pub fn file_id(file: &File) -> std::io::Result<(u64, u64)> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let meta = file.metadata()?;
        Ok((meta.dev(), meta.ino()))
    }
    #[cfg(windows)]
    {
        crate::world_windows::file_id(file)
    }
}

/// This application's private per-user runtime directory
/// (`runtime_dir_path`), created on demand and verified on every use.
/// Holds every world's socket on Unix (C112); the daemon does not make
/// it on Windows, where the endpoint is a pipe; never the real-mode lock, which lives in
/// the user's data directory ([`real_mode_lock_path`], C132). [`private_dir`]
/// does the creating and the checking; the daemon calls this before it
/// binds, a client never creates it ([`World::socket_path`]).
pub fn app_runtime_dir() -> std::io::Result<PathBuf> {
    let runtime = runtime_dir_path();
    // Nameable before it is made: a runtime directory in other bytes
    // could not name a socket in any report (review 2026-09-11).
    if runtime.to_str().is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "the runtime directory {} is not valid UTF-8 (XDG_RUNTIME_DIR or TMPDIR); it names the socket in every report and must be",
                runtime.display()
            ),
        ));
    }
    private_dir(&runtime)
}

/// The calling user's uid (Unix); 0 elsewhere, where the checks below
/// are not made.
fn current_uid() -> u32 {
    #[cfg(unix)]
    {
        // SAFETY: getuid has no preconditions and cannot fail.
        unsafe { libc::getuid() }
    }
    #[cfg(not(unix))]
    {
        0
    }
}

/// A directory private to this user at `path`: created with mode 0700
/// in the creating call if missing (its parent must exist: the runtime
/// or temp directory, or the data directory the real-mode lock's `run`
/// sits in); refused if it is a symlink or not a directory,
/// if another user owns it, or if its mode is not exactly 0700 and
/// cannot be made so — the pre-creation attack on a shared temp
/// directory, where a path the attacker made first, or made loose
/// between creation and a later chmod, would be used as ours (review
/// 2026-09-11, twice). On a non-Unix platform only the existence check
/// is made. The checks after creation, never the create, are what hold
/// that attack, so a create that meets AlreadyExists is not refused: it
/// is two of our own starts at once (a cold CI runner's first tests),
/// and the directory the other made is checked as made. Every other
/// create error is a refusal.
pub fn private_dir(path: &Path) -> std::io::Result<PathBuf> {
    use std::io::{Error, ErrorKind};
    if let Err(e) = std::fs::symlink_metadata(path)
        && e.kind() == ErrorKind::NotFound
    {
        #[cfg_attr(not(unix), allow(unused_mut))]
        let mut builder = std::fs::DirBuilder::new();
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        match builder.create(path) {
            Ok(()) => {}
            // two of our own starts at once: the other made it first, and
            // the checks below hold it as they hold any directory found
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {}
            Err(e) => {
                return Err(Error::new(
                    e.kind(),
                    format!(
                        "could not create the private directory {}: {e}",
                        path.display()
                    ),
                ));
            }
        }
    }
    existing_private_dir(path)
}

/// [`private_dir`]'s checks alone, on a directory that must already
/// exist: a missing one is `NotFound`, nothing is created. What a client
/// uses before it connects to a socket in the runtime directory.
pub fn existing_private_dir(path: &Path) -> std::io::Result<PathBuf> {
    use std::io::{Error, ErrorKind};
    match std::fs::symlink_metadata(path) {
        Err(e) => return Err(e),
        Ok(meta) if meta.file_type().is_symlink() => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "{} is a symlink; a private directory must be a plain directory this user made",
                    path.display()
                ),
            ));
        }
        Ok(meta) if !meta.is_dir() => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("{} exists and is not a directory", path.display()),
            ));
        }
        Ok(_) => {}
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        let meta = std::fs::symlink_metadata(path)?;
        let uid = current_uid();
        if meta.uid() != uid {
            return Err(Error::new(
                ErrorKind::PermissionDenied,
                format!(
                    "{} is owned by uid {}, not this user ({uid}); refusing to use it",
                    path.display(),
                    meta.uid()
                ),
            ));
        }
        if meta.mode() & 0o7777 != 0o700 {
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))?;
            let again = std::fs::symlink_metadata(path)?;
            if again.mode() & 0o7777 != 0o700 {
                return Err(Error::new(
                    ErrorKind::PermissionDenied,
                    format!(
                        "{} is mode {:04o}, not 0700, and could not be made so; refusing to use it",
                        path.display(),
                        again.mode() & 0o7777
                    ),
                ));
            }
        }
    }
    Ok(path.to_path_buf())
}

/// How a user's data directory sits under the home, per platform: the
/// fixed shape the platform's convention gives with no variable set
/// (C132). The default store's directory ([`intended_root`]) has the same
/// shape on macOS and Linux, so a user who moved nothing finds the lock's
/// `run` beside `store`; on Windows the lock takes the local, never
/// roaming, application data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HomeLayout {
    /// `<home>/Library/Application Support/com.smallcatlabs.acq`
    MacOs,
    /// `<home>/.local/share/acq`, the XDG default (Linux and every other
    /// Unix)
    Xdg,
    /// `<profile>\AppData\Local\smallcatlabs\acq`
    Windows,
}

impl HomeLayout {
    const HERE: HomeLayout = if cfg!(target_os = "macos") {
        HomeLayout::MacOs
    } else if cfg!(windows) {
        HomeLayout::Windows
    } else {
        HomeLayout::Xdg
    };
}

/// C132: the real-mode lock's private `run` directory under `home`, a
/// pure function of the home and the platform's layout — nothing here
/// reads the environment, so nothing a process sets can move it.
fn real_mode_run_dir_in(home: &Path, layout: HomeLayout) -> PathBuf {
    let data = match layout {
        HomeLayout::MacOs => home
            .join("Library")
            .join("Application Support")
            .join(format!("{QUALIFIER}.{ORGANIZATION}.{APPLICATION}")),
        HomeLayout::Xdg => home.join(".local").join("share").join(APPLICATION),
        HomeLayout::Windows => home
            .join("AppData")
            .join("Local")
            .join(ORGANIZATION)
            .join(APPLICATION),
    };
    data.join("run")
}

/// C132: this OS user's home as the system records it, never as a
/// variable names it — on Unix `pw_dir` of the password database's entry
/// for the real uid (`getpwuid_r`), never `HOME`; on Windows the profile
/// directory of the process token's user (`GetUserProfileDirectoryW`),
/// never `%USERPROFILE%` or `%LOCALAPPDATA%`.
fn user_home() -> std::io::Result<PathBuf> {
    #[cfg(unix)]
    {
        password_database_home()
    }
    #[cfg(windows)]
    {
        crate::world_windows::profile_dir()
    }
    #[cfg(not(any(unix, windows)))]
    {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "this platform has no home directory the real-mode lock can be derived from",
        ))
    }
}

/// `pw_dir` of this real uid's password database entry (`getpwuid_r`):
/// no entry, or an empty home, is an error naming the uid.
#[cfg(unix)]
fn password_database_home() -> std::io::Result<PathBuf> {
    use std::ffi::{CStr, OsStr};
    use std::io::{Error, ErrorKind};
    use std::os::unix::ffi::OsStrExt;
    let uid = current_uid();
    // SAFETY: sysconf has no preconditions; -1 (no limit) falls back below.
    let hint = unsafe { libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) };
    let mut size = usize::try_from(hint)
        .ok()
        .filter(|n| *n > 0)
        .unwrap_or(1024);
    loop {
        let mut buf: Vec<libc::c_char> = vec![0; size];
        // SAFETY: an all-zero passwd is a valid value (null pointers,
        // zero ids); getpwuid_r fills it on success.
        let mut entry: libc::passwd = unsafe { std::mem::zeroed() };
        let mut found: *mut libc::passwd = std::ptr::null_mut();
        // SAFETY: `entry`, `buf` (of `buf.len()` bytes) and `found` live
        // across the call; the strings `entry` points to are in `buf`.
        let rc =
            unsafe { libc::getpwuid_r(uid, &mut entry, buf.as_mut_ptr(), buf.len(), &mut found) };
        if rc == libc::ERANGE && size < 1 << 20 {
            size *= 2;
            continue;
        }
        if rc != 0 {
            let e = Error::from_raw_os_error(rc);
            return Err(Error::new(
                e.kind(),
                format!("could not read the password database entry for uid {uid}: {e}"),
            ));
        }
        if found.is_null() || entry.pw_dir.is_null() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("the password database has no home directory for uid {uid}"),
            ));
        }
        // SAFETY: a non-null `pw_dir` from a successful call is a
        // NUL-terminated string in `buf`, which is still alive.
        let dir = unsafe { CStr::from_ptr(entry.pw_dir) }.to_bytes();
        if dir.is_empty() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("the password database's home directory for uid {uid} is empty"),
            ));
        }
        return Ok(PathBuf::from(OsStr::from_bytes(dir)));
    }
}

/// C132: the real-mode lock under `home`: `ggg.lock` in the private
/// `run` directory of the user's own data directory — pure, as
/// [`real_mode_run_dir_in`] is, and the one derivation both the lock and
/// its test use.
fn real_mode_lock_in(home: &Path) -> PathBuf {
    real_mode_run_dir_in(home, HomeLayout::HERE).join("ggg.lock")
}

/// Where the real-mode lock is for this OS user, computed and not
/// touched: [`real_mode_lock_in`] the home the system records
/// ([`user_home`]). No variable moves it, so every real-mode daemon of
/// this user meets the same file, whatever its environment. The tests'
/// view of [`real_mode_lock_path`], which creates nothing in the home.
#[cfg(test)]
fn real_mode_lock_location() -> std::io::Result<PathBuf> {
    Ok(real_mode_lock_in(&user_home()?))
}

/// The real-mode lock's file, per OS user, whatever the root and whatever
/// the environment (C31, C83, C132): `real_mode_lock_in` the home the
/// system records (`user_home`), its
/// `run` directory made private ([`private_dir`]: 0700, no symlink, this
/// user's) and its data directory created if missing. A home that is not
/// a directory is never created; it, and a data directory or `run` that
/// cannot be made, is an error naming the path — the daemon refuses real
/// mode on it rather than lock anywhere else.
pub fn real_mode_lock_path() -> std::io::Result<PathBuf> {
    real_mode_lock_made_under(&user_home()?)
}

/// [`real_mode_lock_path`]'s making and refusals for a given `home`, the
/// seam its tests drive with a scratch home, never the user's own.
fn real_mode_lock_made_under(home: &Path) -> std::io::Result<PathBuf> {
    use std::io::{Error, ErrorKind};
    if !home.is_dir() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "this OS user's home directory {} is missing or not a directory; the real-mode lock lives under it",
                home.display()
            ),
        ));
    }
    let lock = real_mode_lock_in(home);
    let run = lock.parent().unwrap_or(home);
    if let Some(data) = run.parent() {
        std::fs::create_dir_all(data).map_err(|e| {
            Error::new(
                e.kind(),
                format!(
                    "could not create the data directory {} for the real-mode lock: {e}",
                    data.display()
                ),
            )
        })?;
    }
    private_dir(run)?;
    Ok(lock)
}

/// Where the daemon log and the default journal live: `ACQ_LOG_DIR`
/// (made absolute), else the platform's log directory —
/// `~/Library/Logs/acq` on macOS, the XDG state
/// directory's `log` on Linux, the local data directory's `logs`
/// elsewhere; `logs` under the current directory with no home at all.
/// One subdirectory per world and provider under it ([`World::log_dir`]).
pub fn log_base_dir() -> PathBuf {
    if let Some(d) = std::env::var_os("ACQ_LOG_DIR") {
        return absolute(Path::new(&d));
    }
    if cfg!(target_os = "macos")
        && let Some(home) = BaseDirs::new().map(|b| b.home_dir().to_path_buf())
    {
        return home.join("Library").join("Logs").join(APPLICATION);
    }
    match project_dirs() {
        Some(p) => match p.state_dir() {
            Some(state) => state.join("log"),
            None => p.data_local_dir().join("logs"),
        },
        None => PathBuf::from("logs"),
    }
}

/// An exclusive advisory lock on a file, held while this value lives; the
/// holder's pid is written into the file for the refusal that names it.
#[derive(Debug)]
pub struct Lock {
    _file: File,
    path: PathBuf,
}

/// The lock is held by another process, or could not be taken.
#[derive(Debug, Clone)]
pub struct LockError {
    pub path: PathBuf,
    /// The pid the holder wrote, when the file could be read and parsed.
    pub holder: Option<u32>,
    /// An I/O failure other than the lock being held.
    pub io: Option<String>,
}

impl std::fmt::Display for LockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.io, self.holder) {
            (Some(io), _) => write!(f, "could not take the lock {}: {io}", self.path.display()),
            (None, Some(pid)) => write!(
                f,
                "{} is held by another process (pid {pid})",
                self.path.display()
            ),
            (None, None) => write!(
                f,
                "{} is held by another process (its pid is not recorded yet)",
                self.path.display()
            ),
        }
    }
}

impl std::error::Error for LockError {}

impl Lock {
    /// Where a lock's holder pid is read: the lock file itself on Unix,
    /// `<lock>.pid` beside it on Windows ([`Lock::acquire`]).
    pub fn pid_path(lock: &Path) -> PathBuf {
        if cfg!(windows) {
            lock.with_extension("pid")
        } else {
            lock.to_path_buf()
        }
    }

    /// The pid the holder wrote into the lock file at `path`, read
    /// without contending for the lock: what a client names when the
    /// daemon on a world accepts a connection and never answers (C86).
    /// `None` when there is no file, or no pid in it yet.
    pub fn holder(path: &Path) -> Option<u32> {
        std::fs::read_to_string(Lock::pid_path(path))
            .ok()?
            .trim()
            .parse()
            .ok()
    }

    /// Take the lock at `path` (created if missing), or say who holds it.
    /// Never blocks.
    pub fn acquire(path: &Path) -> Result<Lock, LockError> {
        let io_error = |e: std::io::Error| LockError {
            path: path.to_path_buf(),
            holder: None,
            io: Some(e.to_string()),
        };
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .map_err(io_error)?;
        match file.try_lock() {
            Ok(()) => {}
            Err(TryLockError::WouldBlock) => {
                let holder = if cfg!(windows) {
                    Lock::holder(path)
                } else {
                    let mut text = String::new();
                    let _ = file.read_to_string(&mut text);
                    text.trim().parse().ok()
                };
                return Err(LockError {
                    path: path.to_path_buf(),
                    holder,
                    io: None,
                });
            }
            Err(TryLockError::Error(e)) => return Err(io_error(e)),
        }
        // Held: record who, for the next contender's refusal.
        let _ = file.set_len(0);
        let _ = writeln!(file, "{}", std::process::id());
        let _ = file.flush();
        // Windows locks are mandatory (`LockFileEx` over the whole file):
        // no other process can read the pid above, so it is written again
        // beside the lock, where `holder` reads it.
        if cfg!(windows) {
            let _ = std::fs::write(Lock::pid_path(path), format!("{}\n", std::process::id()));
        }
        Ok(Lock {
            _file: file,
            path: path.to_path_buf(),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// The world's rails state, as [`World::rails_state_path`] holds it: the
/// file's one format, read and written by the daemon's rails and by the
/// offline `reset-tripwire`, which links no daemon (C1). It has three
/// owners, and each one's state survives every other's writes:
///
/// - `tripped`, the live-test tripwire's ladder trip, honoured only by a
///   daemon started with the tripwire, and written back unchanged by one
///   without it;
/// - `blocked`, the Cloudflare block mark (C129), honoured by every
///   daemon;
/// - `refresh_failed_by_account`, the dead-grant marks (C24), which
///   persist across restarts and clear on login or logout, never on a
///   reset.
///
/// A field this build does not know is a later build's owner: it is
/// carried in [`RailsState::unknown`] and written back unchanged, so an
/// older daemon or reset never erases it.
#[derive(Debug, Default, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RailsState {
    #[serde(default)]
    pub tripped: Option<String>,
    /// C129: the block mark's cause (status, method, route and time).
    #[serde(default)]
    pub blocked: Option<String>,
    /// Per account (username → cause). The pre-multi-account file had one
    /// `refresh_failed` string; it is honoured by no daemon (one re-login)
    /// and carried with the unknown fields.
    #[serde(default)]
    pub refresh_failed_by_account: std::collections::HashMap<String, String>,
    /// Every field this build does not know, kept for the build that does.
    #[serde(flatten)]
    pub unknown: std::collections::BTreeMap<String, serde_json::Value>,
}

impl RailsState {
    /// No owner holds anything: the file is removed rather than written.
    pub fn holds_nothing(&self) -> bool {
        self.tripped.is_none()
            && self.blocked.is_none()
            && self.refresh_failed_by_account.is_empty()
            && self.unknown.is_empty()
    }

    /// What `reset-tripwire` clears, on either path: the trip and the
    /// block mark. The dead-grant marks are kept (C24). Returns whether
    /// anything was cleared.
    pub fn reset(&mut self) -> bool {
        let held = self.tripped.is_some() || self.blocked.is_some();
        self.tripped = None;
        self.blocked = None;
        held
    }

    /// Write the state to `path`, or remove the file when it holds
    /// nothing. The write is atomic: the whole text goes to a file beside
    /// it, is flushed to disk, and is renamed over it, so a crash leaves
    /// the old state or the new, never a torn one the next daemon refuses
    /// to start over (and the reset would remove, marks and all). A file
    /// already there keeps its permissions: the new text takes them
    /// before it is written, so a file its owner narrowed stays narrowed.
    pub fn write(&self, path: &Path) -> std::io::Result<()> {
        if self.holds_nothing() {
            return match std::fs::remove_file(path) {
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
                other => other,
            };
        }
        let text = serde_json::to_string(self).map_err(std::io::Error::other)?;
        let mut partial = path.as_os_str().to_owned();
        partial.push(".partial");
        let partial = PathBuf::from(partial);
        let written = (|| {
            let mut file = std::fs::File::create(&partial)?;
            match std::fs::metadata(path) {
                Ok(held) => file.set_permissions(held.permissions())?,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => return Err(e),
            }
            file.write_all(text.as_bytes())?;
            file.sync_all()?;
            std::fs::rename(&partial, path)
        })();
        if written.is_err() {
            let _ = std::fs::remove_file(&partial);
        }
        written
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> PathBuf {
        static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!(
            "acq-world-{tag}-{}-{}",
            std::process::id(),
            N.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    /// C83: a world is a canonical root — one id for two spellings of
    /// one directory — with its provider directories, lock file and
    /// rails state under it, and its diagnostics keyed by that id under
    /// the log base.
    #[test]
    fn c83_a_world_is_a_canonical_root_and_its_paths_derive_from_it() {
        let base = scratch("paths");
        std::fs::create_dir_all(base.join("real")).unwrap();
        let world = World::observe_at(&base.join("real")).unwrap();
        assert!(world.root().is_absolute());
        // A dotted spelling of the same directory is the same world.
        let dotted =
            World::observe_at(&base.join("real").join(".").join("..").join("real")).unwrap();
        assert_eq!(world, dotted);
        assert_eq!(world.id(), dotted.id());
        assert_eq!(world.id().len(), 12);
        assert!(world.id().bytes().all(|b| b.is_ascii_hexdigit()));
        assert_eq!(world.provider_dir("mock"), world.root().join("mock"));
        assert_eq!(world.lock_path(), world.root().join("daemon.lock"));
        assert_eq!(
            world.rails_state_path("ggg"),
            world.root().join("ggg").join("rails.json")
        );
        assert_eq!(
            world.log_path("mock"),
            log_base_dir()
                .join(world.id())
                .join("mock")
                .join("daemon.log")
        );
        assert_eq!(
            world.journal_path("mock"),
            log_base_dir()
                .join(world.id())
                .join("mock")
                .join("sends.jsonl")
        );
        // Another directory is another world.
        std::fs::create_dir_all(base.join("other")).unwrap();
        let other = World::observe_at(&base.join("other")).unwrap();
        assert_ne!(world, other);
        assert_ne!(world.id(), other.id());
        // A missing root is an error naming it, never a world.
        let err = World::observe_at(&base.join("missing")).unwrap_err();
        assert!(err.absent, "{err}");
        assert!(err.to_string().contains("missing"), "{err}");
        let _ = std::fs::remove_dir_all(&base);
    }

    /// C83: the world lock is exclusive and names its holder; it is
    /// released when the `Lock` is dropped, however that happens.
    #[test]
    fn c83_the_lock_is_exclusive_and_names_its_holder() {
        let base = scratch("lock");
        std::fs::create_dir_all(&base).unwrap();
        let path = base.join("daemon.lock");
        let held = Lock::acquire(&path).unwrap();
        assert_eq!(held.path(), path.as_path());
        assert_eq!(
            std::fs::read_to_string(Lock::pid_path(&path))
                .unwrap()
                .trim(),
            std::process::id().to_string(),
            "the holder wrote its pid"
        );
        // A second acquisition from this process is a second open file
        // description: flock refuses it, and the refusal names the holder.
        let err = Lock::acquire(&path).unwrap_err();
        assert_eq!(err.holder, Some(std::process::id()), "{err}");
        assert!(err.io.is_none(), "{err}");
        assert!(
            err.to_string().contains("held by another process")
                && err.to_string().contains(&std::process::id().to_string()),
            "{err}"
        );
        drop(held);
        let again = Lock::acquire(&path).expect("released on drop");
        drop(again);
        let _ = std::fs::remove_dir_all(&base);
    }

    /// C83: the private runtime directory is this user's: made 0700,
    /// brought back to 0700 when it drifted, refused when it is a symlink
    /// or a file.
    #[cfg(unix)]
    #[test]
    fn c83_the_private_dir_is_this_users() {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        let base = scratch("private");
        std::fs::create_dir_all(&base).unwrap();
        let fresh = private_dir(&base.join("fresh")).unwrap();
        assert_eq!(std::fs::metadata(&fresh).unwrap().mode() & 0o777, 0o700);
        for (name, drifted) in [("loose", 0o755), ("tight", 0o500), ("shut", 0o000)] {
            let dir = base.join(name);
            std::fs::create_dir(&dir).unwrap();
            std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(drifted)).unwrap();
            private_dir(&dir).unwrap();
            assert_eq!(
                std::fs::metadata(&dir).unwrap().mode() & 0o7777,
                0o700,
                "{name}: restored to exactly 0700"
            );
        }
        let err = private_dir(&base.join("absent-parent").join("acq")).unwrap_err();
        assert!(err.to_string().contains("could not create"), "{err}");
        std::os::unix::fs::symlink(&fresh, base.join("link")).unwrap();
        let err = private_dir(&base.join("link")).unwrap_err();
        assert!(err.to_string().contains("symlink"), "{err}");
        std::fs::write(base.join("file"), b"").unwrap();
        let err = private_dir(&base.join("file")).unwrap_err();
        assert!(err.to_string().contains("not a directory"), "{err}");
        let runtime = app_runtime_dir().unwrap();
        assert_eq!(std::fs::metadata(&runtime).unwrap().mode() & 0o077, 0);
        // C132: the real-mode lock is no longer in the runtime directory
        // (`c83_the_lock_path_reads_no_environment` holds where it is).
        assert_ne!(
            real_mode_lock_location().unwrap().parent(),
            Some(runtime.as_path())
        );
        let _ = std::fs::remove_dir_all(&base);
    }

    /// C132: the real-mode lock's path is a pure function of the home the
    /// system records and the platform's layout, so no variable moves it:
    /// the shape for a sample home on each platform, the home read from
    /// the password database (the process token on Windows) and nothing
    /// else, and the lock out of the runtime directory, where the socket
    /// stays (C112). Nothing here
    /// sets or reads a variable; the process test in
    /// `acquisition-cli/tests/world.rs` (`c83_c31_`) moves the variables
    /// under two real daemons. The real path is computed, not created, so
    /// the store's tests make nothing in the user's home.
    #[test]
    fn c83_the_lock_path_reads_no_environment() {
        assert_eq!(
            real_mode_run_dir_in(Path::new("/Users/someone"), HomeLayout::MacOs),
            Path::new("/Users/someone/Library/Application Support/com.smallcatlabs.acq/run")
        );
        assert_eq!(
            real_mode_run_dir_in(Path::new("/home/someone"), HomeLayout::Xdg),
            Path::new("/home/someone/.local/share/acq/run")
        );
        let windows = real_mode_run_dir_in(Path::new("C:/Users/someone"), HomeLayout::Windows);
        assert_eq!(
            windows.components().collect::<Vec<_>>(),
            Path::new("C:/Users/someone/AppData/Local/smallcatlabs/acq/run")
                .components()
                .collect::<Vec<_>>()
        );
        // The real derivation is the pure one over the system's home.
        let home = user_home().unwrap();
        assert!(home.is_absolute(), "{}", home.display());
        let lock = real_mode_lock_location().unwrap();
        assert_eq!(lock, real_mode_lock_in(&home));
        assert_eq!(
            lock.parent(),
            Some(real_mode_run_dir_in(&home, HomeLayout::HERE).as_path())
        );
        // Out of the runtime directory; the socket is still in it.
        let runtime = runtime_dir_path();
        assert_ne!(lock.parent(), Some(runtime.as_path()));
        assert!(!lock.starts_with(&runtime), "{}", lock.display());
        #[cfg(unix)]
        {
            let base = scratch("lock-home");
            std::fs::create_dir_all(base.join("real")).unwrap();
            let world = World::observe_at(&base.join("real")).unwrap();
            if let Ok(socket) = world.socket_path_under(&runtime) {
                assert_eq!(socket.parent(), Some(runtime.as_path()));
            }
            let _ = std::fs::remove_dir_all(&base);
        }
    }

    /// C132: a home that cannot hold the real-mode lock is refused by
    /// name, never worked around: a home that is a file, a data directory
    /// that cannot be made (a file where its first component under the
    /// home should be), and a `run` that fails [`private_dir`] (a symlink
    /// to a directory, a file) — each error names the path it met. A loose
    /// `run` is made 0700, not refused (private_dir's restore). Every home
    /// here is a scratch directory; the user's own is never touched.
    #[test]
    fn c132_a_home_that_cannot_hold_the_lock_is_refused_by_name() {
        let base = scratch("lock-refusals");
        std::fs::create_dir_all(&base).unwrap();

        // The home is a file.
        let file_home = base.join("home-file");
        std::fs::write(&file_home, b"").unwrap();
        let err = real_mode_lock_made_under(&file_home).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains(&file_home.display().to_string())
                && msg.contains("home directory")
                && msg.contains("not a directory"),
            "{msg}"
        );
        // A missing home is the same refusal, and nothing is created.
        let missing = base.join("home-missing");
        let msg = real_mode_lock_made_under(&missing).unwrap_err().to_string();
        assert!(msg.contains(&missing.display().to_string()), "{msg}");
        assert!(!missing.exists());

        // The data directory cannot be made: its first component under
        // the home is a file.
        let home = base.join("home-blocked");
        std::fs::create_dir_all(&home).unwrap();
        let run = real_mode_run_dir_in(&home, HomeLayout::HERE);
        let data = run.parent().unwrap();
        let first = home.join(
            data.strip_prefix(&home)
                .unwrap()
                .components()
                .next()
                .unwrap(),
        );
        std::fs::write(&first, b"").unwrap();
        let msg = real_mode_lock_made_under(&home).unwrap_err().to_string();
        assert!(
            msg.contains("could not create the data directory")
                && msg.contains(&data.display().to_string())
                && msg.contains("real-mode lock"),
            "{msg}"
        );

        // `run` is a file.
        let home = base.join("home-run-file");
        let run = real_mode_run_dir_in(&home, HomeLayout::HERE);
        std::fs::create_dir_all(run.parent().unwrap()).unwrap();
        std::fs::write(&run, b"").unwrap();
        let msg = real_mode_lock_made_under(&home).unwrap_err().to_string();
        assert!(
            msg.contains(&run.display().to_string()) && msg.contains("not a directory"),
            "{msg}"
        );

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            // `run` is a symlink to a private directory of this user's.
            let home = base.join("home-run-link");
            let run = real_mode_run_dir_in(&home, HomeLayout::HERE);
            std::fs::create_dir_all(run.parent().unwrap()).unwrap();
            let target = base.join("elsewhere");
            private_dir(&target).unwrap();
            std::os::unix::fs::symlink(&target, &run).unwrap();
            let msg = real_mode_lock_made_under(&home).unwrap_err().to_string();
            assert!(
                msg.contains(&run.display().to_string()) && msg.contains("is a symlink"),
                "{msg}"
            );

            // A loose `run` is restored to 0700, and the lock is in it.
            let home = base.join("home-run-loose");
            let run = real_mode_run_dir_in(&home, HomeLayout::HERE);
            std::fs::create_dir_all(&run).unwrap();
            std::fs::set_permissions(&run, std::fs::Permissions::from_mode(0o755)).unwrap();
            let lock = real_mode_lock_made_under(&home).unwrap();
            assert_eq!(lock, real_mode_lock_in(&home));
            let mode = std::fs::symlink_metadata(&run)
                .unwrap()
                .permissions()
                .mode();
            assert_eq!(mode & 0o7777, 0o700);
        }

        // A fresh home gets its data directory and a private `run`.
        let home = base.join("home-fresh");
        std::fs::create_dir_all(&home).unwrap();
        let lock = real_mode_lock_made_under(&home).unwrap();
        assert_eq!(lock, real_mode_lock_in(&home));
        assert!(lock.parent().unwrap().is_dir());
        let _ = std::fs::remove_dir_all(&base);
    }

    /// C83: two of our own starts making one fresh private directory at
    /// once both succeed: the one that loses the create's race meets
    /// AlreadyExists and checks the directory as made, and the directory
    /// passes the checks. Raced on a barrier, two hundred fresh paths, so
    /// the race is met (a cold CI runner's first tests did, on Windows).
    #[test]
    fn two_starts_making_one_private_dir_at_once_both_succeed() {
        let base = scratch("race");
        std::fs::create_dir_all(&base).unwrap();
        let mut refused = Vec::new();
        for i in 0..200 {
            let path = base.join(format!("acq-{i}"));
            let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
            let racers: Vec<_> = (0..2)
                .map(|_| {
                    let (path, barrier) = (path.clone(), barrier.clone());
                    std::thread::spawn(move || {
                        barrier.wait();
                        private_dir(&path)
                    })
                })
                .collect();
            for racer in racers {
                if let Err(e) = racer.join().unwrap() {
                    refused.push(format!("{i}: {e}"));
                }
            }
            existing_private_dir(&path).unwrap();
        }
        assert!(
            refused.is_empty(),
            "{} of 400 calls refused: {refused:?}",
            refused.len()
        );
        let _ = std::fs::remove_dir_all(&base);
    }

    /// C83: a non-UTF-8 root is refused as a world. Linux only: APFS
    /// refuses an invalid byte sequence in a name (EILSEQ), so on macOS
    /// no such directory can exist and the filesystem holds the property.
    #[cfg(target_os = "linux")]
    #[test]
    fn c83_a_root_that_is_not_utf8_is_refused() {
        use std::os::unix::ffi::OsStrExt;
        let base = scratch("odd-root");
        std::fs::create_dir_all(&base).unwrap();
        let odd = base.join(std::ffi::OsStr::from_bytes(b"r\xff"));
        std::fs::create_dir(&odd).unwrap();
        let err = World::observe_at(&odd).unwrap_err();
        assert!(err.to_string().contains("UTF-8"), "{err}");
        let _ = std::fs::remove_dir_all(&base);
    }

    /// C83: the socket derives from the world into the runtime directory
    /// and from nothing else — two spellings of one root reach one
    /// socket, another root another. That no knob names it is held by
    /// `tools/docs-check.sh`: a knob read anywhere in the crates needs a
    /// README row, and the socket has none. The derivation's refusals —
    /// over the cap, not UTF-8 — are pinned on the pure helper with
    /// runtime directories this test names, since the real one is the
    /// environment's (review 2026-09-11).
    #[test]
    fn c83_the_socket_derives_from_the_world_into_the_runtime_directory() {
        let base = scratch("socket");
        std::fs::create_dir_all(base.join("real")).unwrap();
        std::fs::create_dir_all(base.join("other")).unwrap();
        // The derivation under a runtime directory this test names: the
        // world's id names the socket, two spellings of one root reach
        // one socket, another root another, all in that directory.
        let named = Path::new("/run/user/1000/acq");
        let world = World::observe_at(&base.join("real")).unwrap();
        let socket = world.socket_path_under(named).unwrap();
        assert_eq!(socket, named.join(world.socket_name()));
        assert_eq!(world.socket_name(), format!("{}.sock", world.id()));
        let dotted =
            World::observe_at(&base.join("real").join(".").join("..").join("real")).unwrap();
        assert_eq!(dotted.socket_path_under(named).unwrap(), socket);
        let other = World::observe_at(&base.join("other")).unwrap();
        assert_ne!(other.socket_path_under(named).unwrap(), socket);
        assert_eq!(
            other.socket_path_under(named).unwrap().parent(),
            socket.parent()
        );
        // The environment's runtime directory, whatever it is: the real
        // door agrees with the pure derivation over it — the same path
        // when that derivation accepts it, and a refusal of the same
        // kind when it does not (a deep or non-UTF-8 runtime directory
        // is a valid environment; review 2026-09-11).
        #[cfg(unix)]
        {
            let runtime = runtime_dir_path();
            match world.socket_path_under(&runtime) {
                Ok(expected) => {
                    app_runtime_dir().unwrap();
                    assert_eq!(world.socket_path().unwrap(), expected);
                    assert_eq!(dotted.socket_path().unwrap(), expected);
                }
                Err(e) => {
                    assert_eq!(e.kind(), std::io::ErrorKind::InvalidInput, "{e}");
                    let real = world.socket_path().unwrap_err();
                    assert_eq!(real.kind(), std::io::ErrorKind::InvalidInput, "{real}");
                    assert_eq!(real.to_string(), e.to_string());
                }
            }
        }
        // Windows (C112): the real door is the pipe of this user and this
        // world, the same for two spellings of one root, another for
        // another root.
        #[cfg(windows)]
        {
            let sid = crate::world_windows::current_user_sid().unwrap();
            let pipe = world.socket_path().unwrap();
            assert_eq!(
                pipe,
                PathBuf::from(format!(r"\\.\pipe\acq-{sid}-{}", world.id()))
            );
            assert_eq!(dotted.socket_path().unwrap(), pipe);
            assert_ne!(other.socket_path().unwrap(), pipe);
        }
        let deep = PathBuf::from("/").join("x".repeat(SOCKET_PATH_MAX));
        let err = world.socket_path_under(&deep).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
        assert!(
            err.to_string().contains("at most 103") && err.to_string().contains("bytes"),
            "{err}"
        );
        // Exactly at the cap passes; one over is refused: the bound is the
        // path's byte length, terminator excluded.
        let name_len = world.socket_name().len();
        // "/" + the y's + "/" + the name: two separators.
        let at_cap = PathBuf::from("/".to_string() + &"y".repeat(SOCKET_PATH_MAX - name_len - 2));
        assert_eq!(
            world.socket_path_under(&at_cap).unwrap().as_os_str().len(),
            SOCKET_PATH_MAX
        );
        let over = PathBuf::from("/".to_string() + &"y".repeat(SOCKET_PATH_MAX - name_len - 1));
        assert!(world.socket_path_under(&over).is_err());
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            let odd = PathBuf::from(std::ffi::OsStr::from_bytes(b"/run/user/1000/acq-\xff"));
            let err = world.socket_path_under(&odd).unwrap_err();
            assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
            assert!(
                err.to_string().contains("not valid UTF-8")
                    && err.to_string().contains("XDG_RUNTIME_DIR"),
                "{err}"
            );
        }
        let _ = std::fs::remove_dir_all(&base);
    }

    /// The rails state's fields are three owners'; a field this build
    /// does not know is a later build's, and survives a read, a reset and
    /// a write unchanged. A file holding only such a field is kept. The
    /// write leaves nothing beside the file (its partial is renamed).
    #[test]
    fn the_rails_state_keeps_a_later_builds_fields() {
        let base = scratch("rails-unknown");
        std::fs::create_dir_all(&base).unwrap();
        let path = base.join("rails.json");
        let later = r#"{"since":"2027-01-01","cause":"a later halt"}"#;
        std::fs::write(
            &path,
            format!(
                r#"{{"tripped":"429 on GET /x","later_build_mark":{later},"refresh_failed_by_account":{{"A#1":"400"}}}}"#
            ),
        )
        .unwrap();
        let mut held: RailsState =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert!(held.reset(), "the trip was cleared");
        held.write(&path).unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        let written: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(
            written["later_build_mark"],
            serde_json::from_str::<serde_json::Value>(later).unwrap(),
            "{text}"
        );
        assert_eq!(written["refresh_failed_by_account"]["A#1"], "400", "{text}");
        assert!(written["tripped"].is_null(), "{text}");

        let only_later = RailsState {
            unknown: [("later_build_mark".to_string(), serde_json::json!(1))].into(),
            ..RailsState::default()
        };
        assert!(!only_later.holds_nothing());
        only_later.write(&path).unwrap();
        let reread: RailsState =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(reread, only_later);
        let beside: Vec<_> = std::fs::read_dir(&base)
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        assert_eq!(beside, ["rails.json"], "no partial left beside the file");
        let _ = std::fs::remove_dir_all(&base);
    }

    /// A rewrite keeps the file's mode: a rails state its owner narrowed
    /// to 0600 is not widened to the umask's default by the next mark.
    #[cfg(unix)]
    #[test]
    fn the_rails_state_rewrite_keeps_the_files_mode() {
        use std::os::unix::fs::PermissionsExt;
        let base = scratch("rails-mode");
        std::fs::create_dir_all(&base).unwrap();
        let path = base.join("rails.json");
        std::fs::write(&path, r#"{"tripped":"429 on GET /x"}"#).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        let state = RailsState {
            blocked: Some("a block".into()),
            ..RailsState::default()
        };
        state.write(&path).unwrap();
        let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "the rewrite changed the mode to {mode:o}");
        let _ = std::fs::remove_dir_all(&base);
    }
}
