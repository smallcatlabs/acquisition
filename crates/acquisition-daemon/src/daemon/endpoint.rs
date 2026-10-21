//! The daemon's endpoint on each platform: the stream, the listener and the
//! bound socket of a Unix socket or a Windows named pipe.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::*;

/// A connection to the daemon's endpoint (C112): a Unix socket, or on
/// Windows the server end of one instance of the world's named pipe.
#[cfg(unix)]
pub(super) type Stream = tokio::net::UnixStream;
#[cfg(windows)]
pub(super) type Stream = tokio::net::windows::named_pipe::NamedPipeServer;

/// The daemon's end of its endpoint (C112): bound once at start, then
/// one connection per `accept`.
///
/// - **Unix**: a socket in the private per-user runtime directory. A
///   socket file left by a crash is probed — something answering is a
///   live daemon and the start is refused — and removed; the bound
///   file's identity is kept ([`BoundSocket`]) so the exit path unlinks
///   only its own.
/// - **Windows**: the world's named pipe, every instance created under a
///   DACL granting this user alone (`world_windows`). The first instance
///   is created as such, so a name another process holds refuses the
///   start; each accepted connection is replaced by a fresh instance
///   before it is handed on. A pipe vanishes with its server: there is
///   no stale file to probe and nothing to unlink.
#[cfg(unix)]
pub(super) struct Listener(tokio::net::UnixListener);
#[cfg(windows)]
pub(super) struct Listener {
    name: PathBuf,
    next: Stream,
}

impl Listener {
    #[cfg(unix)]
    pub(super) async fn bind(path: &Path) -> Result<(Listener, BoundSocket)> {
        if path.exists() {
            // Live daemon or stale socket from a crash?
            if tokio::net::UnixStream::connect(path).await.is_ok() {
                anyhow::bail!("daemon already running on {}", path.display());
            }
            std::fs::remove_file(path)?;
        }
        let listener = tokio::net::UnixListener::bind(path)
            .map_err(|e| anyhow::anyhow!("could not bind {}: {e}", path.display()))?;
        let bound = BoundSocket::of(path.to_path_buf()).map_err(|e| {
            anyhow::anyhow!("identifying the socket {} just bound: {e}", path.display())
        })?;
        Ok((Listener(listener), bound))
    }

    #[cfg(windows)]
    pub(super) async fn bind(path: &Path) -> Result<(Listener, BoundSocket)> {
        let first = Self::instance(path, true).map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                anyhow::anyhow!(
                    "the pipe {} is already held by another process; this daemon will not serve beside it",
                    path.display()
                )
            } else {
                anyhow::anyhow!("could not create the pipe {}: {e}", path.display())
            }
        })?;
        Ok((
            Listener {
                name: path.to_path_buf(),
                next: first,
            },
            BoundSocket,
        ))
    }

    /// One instance of the pipe at `name`, this user's alone.
    #[cfg(windows)]
    fn instance(name: &Path, first: bool) -> std::io::Result<Stream> {
        acquisition_store::world_windows::with_user_only_attributes(|attrs| {
            // SAFETY: `attrs` is a SECURITY_ATTRIBUTES that lives until
            // `with_user_only_attributes` returns, after this call.
            unsafe {
                tokio::net::windows::named_pipe::ServerOptions::new()
                    .first_pipe_instance(first)
                    .reject_remote_clients(true)
                    .create_with_security_attributes_raw(name, attrs)
            }
        })
    }

    /// The next connection.
    pub(super) async fn accept(&mut self) -> std::io::Result<Stream> {
        #[cfg(unix)]
        {
            Ok(self.0.accept().await?.0)
        }
        #[cfg(windows)]
        {
            self.next.connect().await?;
            let fresh = Self::instance(&self.name, false)?;
            Ok(std::mem::replace(&mut self.next, fresh))
        }
    }
}

/// The socket a daemon bound, with the file's identity at the time, so
/// the exit path removes that file and not a successor's at the same
/// path. Unix only: a Windows pipe leaves nothing behind.
#[cfg(unix)]
pub(super) struct BoundSocket {
    path: PathBuf,
    dev: u64,
    ino: u64,
}
#[cfg(windows)]
pub(super) struct BoundSocket;

impl BoundSocket {
    /// Identify the file just bound at `path`.
    #[cfg(unix)]
    fn of(path: PathBuf) -> std::io::Result<BoundSocket> {
        use std::os::unix::fs::MetadataExt;
        let meta = std::fs::metadata(&path)?;
        Ok(BoundSocket {
            dev: meta.dev(),
            ino: meta.ino(),
            path,
        })
    }

    /// Remove the socket file if it is still the one bound; leave a
    /// successor's alone.
    pub(super) fn remove_if_still_ours(&self) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if let Ok(meta) = std::fs::symlink_metadata(&self.path)
                && meta.dev() == self.dev
                && meta.ino() == self.ino
            {
                let _ = std::fs::remove_file(&self.path);
            }
        }
    }
}
