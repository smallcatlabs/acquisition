//! The client's end of the daemon's endpoint (C112): connecting to the
//! path [`World::socket_path`](acquisition_store::world::World::socket_path)
//! derives, on Unix a socket and on Windows a named pipe, and the two
//! small helpers the tests share — whether an endpoint exists, and a
//! plain listener a scripted peer serves on.
//!
//! # C112 — as built (the client)
//!
//! On Unix the socket's directory was verified this user's when the path
//! was derived ([`World::socket_path`](acquisition_store::world::World::socket_path)),
//! so connecting is enough. On Windows a pipe whose instances are all
//! busy is retried briefly (the server makes the next as it accepts), and
//! a pipe whose server is not this user's process is refused before a
//! byte is written (`acquisition_store::world_windows`). A missing
//! endpoint is `NotFound` on both, which the doors read as absence.

use std::path::Path;

/// A connection to a daemon's endpoint.
#[cfg(unix)]
pub type Stream = tokio::net::UnixStream;
/// A connection to a daemon's endpoint.
#[cfg(windows)]
pub type Stream = tokio::net::windows::named_pipe::NamedPipeClient;

/// Connect to the endpoint at `path`.
#[cfg(unix)]
pub async fn connect(path: &Path) -> std::io::Result<Stream> {
    Stream::connect(path).await
}

/// Connect to the endpoint at `path`.
#[cfg(windows)]
pub async fn connect(path: &Path) -> std::io::Result<Stream> {
    use std::os::windows::io::AsRawHandle;
    use tokio::net::windows::named_pipe::ClientOptions;
    /// `ERROR_PIPE_BUSY`: every instance is taken; the server makes the
    /// next one as soon as it accepts.
    const ERROR_PIPE_BUSY: i32 = 231;
    let mut tries = 0;
    let pipe = loop {
        match ClientOptions::new().open(path) {
            Ok(pipe) => break pipe,
            Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY) && tries < 50 => {
                tries += 1;
                tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            }
            Err(e) => return Err(e),
        }
    };
    if !acquisition_store::world_windows::server_runs_as_current_user(pipe.as_raw_handle())? {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!(
                "the pipe {} is served by another user's process; refusing to speak to it",
                path.display()
            ),
        ));
    }
    Ok(pipe)
}

/// Whether an endpoint exists at `path`, without connecting: a socket
/// file on Unix; on Windows a name in the pipe namespace, read from its
/// listing — opening a pipe to look would take one of its instances.
pub fn exists(path: &Path) -> bool {
    #[cfg(unix)]
    {
        path.exists()
    }
    #[cfg(windows)]
    {
        let Some(name) = path.file_name() else {
            return false;
        };
        std::fs::read_dir(r"\\.\pipe\")
            .map(|entries| entries.flatten().any(|e| e.file_name() == name))
            .unwrap_or(false)
    }
}

/// A plain listener at `path`: what a test's scripted peer serves on.
/// Not the daemon's — the daemon's own refuses a taken name and creates
/// every instance under this user's ACL (`acquisition-daemon`, C112).
pub struct Listener {
    #[cfg(unix)]
    inner: tokio::net::UnixListener,
    #[cfg(windows)]
    name: std::path::PathBuf,
    #[cfg(windows)]
    next: tokio::net::windows::named_pipe::NamedPipeServer,
}

/// The server end of one connection a [`Listener`] accepted.
#[cfg(unix)]
pub type Accepted = tokio::net::UnixStream;
/// The server end of one connection a [`Listener`] accepted.
#[cfg(windows)]
pub type Accepted = tokio::net::windows::named_pipe::NamedPipeServer;

impl Listener {
    /// Listen at `path`. A peer's leftover is not refused: on Unix a
    /// socket file left there is replaced, and on Windows a name still
    /// held by a closed peer's last connections is joined — the next
    /// client reaches this listener, the only one waiting for one.
    pub fn bind(path: &Path) -> std::io::Result<Listener> {
        #[cfg(unix)]
        {
            let _ = std::fs::remove_file(path);
            Ok(Listener {
                inner: tokio::net::UnixListener::bind(path)?,
            })
        }
        #[cfg(windows)]
        {
            Ok(Listener {
                name: path.to_path_buf(),
                next: tokio::net::windows::named_pipe::ServerOptions::new().create(path)?,
            })
        }
    }

    /// The next connection.
    pub async fn accept(&mut self) -> std::io::Result<Accepted> {
        #[cfg(unix)]
        {
            Ok(self.inner.accept().await?.0)
        }
        #[cfg(windows)]
        {
            self.next.connect().await?;
            let fresh = tokio::net::windows::named_pipe::ServerOptions::new().create(&self.name)?;
            Ok(std::mem::replace(&mut self.next, fresh))
        }
    }
}
