//! The contract between the client and the daemon, black-box (C85): a
//! daemon process on its own socket, driven through the public client —
//! `Client`, `Subscription`, `stop_any` — and through raw frames where the
//! property is about what the daemon does with a frame the client would
//! never send. Bootstrap across a contract mismatch, stop across one, a
//! subscription's events-only discipline and its lag signal, malformed and
//! oversize frames, the disconnect and the restart, and the closed error
//! kinds on the domain failures a client can stage.
//!
//! The daemon is the `acqd` the quality gate built (`cargo build
//! --workspace` before the tests, AGENTS.md): this crate has no binary of
//! its own and may not link the daemon, so the tests start the one Cargo
//! uplifts above `deps/`, `target/<profile>/acqd` (C82's rule for a test
//! executable, which has no sibling: it names the `acqd` its build
//! wrote). The process harness every crate's tests share lives here,
//! `tests/harness/mod.rs` (C122): `acqd_built` finds that daemon and
//! names the build step when it is missing, `start_daemon_in_process`
//! places a link of its name beside this executable first, since the
//! client judges a daemon's artifact against that sibling (C84), and the
//! daemon a test starts is named by path in every failure.
//!
//! The client reads the world from `ACQ_STORE_DIR` and derives the
//! socket from it (C83) into the runtime directory, process-wide
//! settings, so each test here takes the harness's `session`: the one
//! lock, and the one isolation list applied to this process (a scratch
//! world, temp, runtime and log directory, every other `ACQ_*` removed),
//! so the sockets these daemons bind never touch the user's own (the
//! real-mode lock is not moved by it, C132, and these daemons are mock,
//! which never takes it). The store root is created with the session (a
//! scripted peer has no daemon to create it, and the world it claims must
//! exist to be this process's). The raw frame client and the scripted
//! peers below stay here: they are this file's subject (C85). Nothing
//! here reaches GGG: the daemon runs the mock provider.

mod harness;

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use acquisition_client::client::{
    Client, ConnectError, ConnectOptions, NotReplaced, Observed, Signal, Subscription,
};
use acquisition_client::endpoint::{self, Listener, Stream};
use acquisition_client::frame::{Frame, read_frame};
use acquisition_protocol::job::JobState;
use acquisition_protocol::protocol::{ErrorKind, MAX_FRAME_BYTES, Request, Response};
use acquisition_protocol::{CONTRACT_REVISION, VERSION};
use acquisition_store::world::{Lock, World};
use harness::{
    acqd_built, compatible_client, login_in_process, session, start_daemon_in_process,
    wait_all_terminal,
};
use serde_json::{Value, json};
use tokio::io::{AsyncWriteExt, BufReader, ReadHalf, WriteHalf};

type OwnedReadHalf = ReadHalf<Stream>;
type OwnedWriteHalf = WriteHalf<Stream>;

/// The session's socket: derived from its world (C83) — what the daemon
/// binds and a scripted peer must bind to be found.
fn session_socket() -> PathBuf {
    World::observe()
        .expect("the session's world")
        .socket_path()
        .expect("its socket")
}

async fn open_subscription() -> Subscription {
    match Subscription::observe().await.expect("observe") {
        Observed::Compatible(subscription) => subscription,
        Observed::Absent => panic!("no daemon"),
        Observed::Incompatible(found) => panic!("{found}"),
        Observed::Unresponsive(found) => panic!("{found}"),
    }
}

/// A raw framed connection: what the client never sends, sent anyway.
struct Raw {
    reader: BufReader<OwnedReadHalf>,
    write: OwnedWriteHalf,
}

impl Raw {
    async fn open() -> Raw {
        let stream = endpoint::connect(&session_socket()).await.expect("connect");
        let (read, write) = tokio::io::split(stream);
        Raw {
            reader: BufReader::new(read),
            write,
        }
    }

    async fn send(&mut self, bytes: &[u8]) {
        self.write.write_all(bytes).await.expect("write");
        self.write.write_all(b"\n").await.expect("write");
    }

    async fn next(&mut self) -> Value {
        match read_frame(&mut self.reader, MAX_FRAME_BYTES)
            .await
            .expect("read")
        {
            Frame::Line(bytes) => serde_json::from_slice(&bytes)
                .unwrap_or_else(|e| panic!("{e}: {}", String::from_utf8_lossy(&bytes))),
            other => panic!("{other:?}"),
        }
    }

    async fn ask(&mut self, frame: Value) -> Value {
        self.send(frame.to_string().as_bytes()).await;
        self.next().await
    }

    async fn hello(&mut self) -> Value {
        let reply = self
            .ask(json!({ "req": "hello", "version": VERSION, "contract": CONTRACT_REVISION }))
            .await;
        assert_eq!(reply["resp"], "hello", "{reply}");
        reply
    }
}

fn error_kind(frame: &Value) -> ErrorKind {
    assert_eq!(frame["resp"], "error", "{frame}");
    assert!(
        frame["message"].as_str().is_some_and(|m| !m.is_empty()),
        "an error carries a message: {frame}"
    );
    serde_json::from_value(frame["kind"].clone()).unwrap_or_else(|e| panic!("{e}: {frame}"))
}

async fn submit(client: &mut Client, kind: &str, params: Value, account: Option<&str>) -> Response {
    client
        .request(&Request::Submit {
            kind: kind.into(),
            params,
            priority: 0,
            submitted_by: "contract".into(),
            account: account.map(str::to_string),
        })
        .await
        .expect("submit")
}

fn submitted(response: Response) -> u64 {
    match response {
        Response::Submitted { id } => id,
        other => panic!("{other:?}"),
    }
}

fn kind_of(response: Response) -> (ErrorKind, String) {
    match response {
        Response::Error { kind, message } => (kind, message),
        other => panic!("{other:?}"),
    }
}

// ---- the bootstrap plane --------------------------------------------------

/// The daemon side: a hello of any shape is answered with this daemon's
/// identity, a request it cannot read is `bad_request` with the
/// connection kept, and a `daemon_stop` of any shape stops it.
#[tokio::test]
async fn c85_the_daemon_identifies_itself_and_stops_across_a_contract_mismatch() {
    let s = session("boot");
    let mut daemon = start_daemon_in_process(&s, &acqd_built()).await;
    let mut raw = Raw::open().await;

    // Every connection begins with hello: a versioned request before it
    // performs nothing, and the connection waits for the greeting.
    let reply = raw.ask(json!({ "req": "list" })).await;
    assert_eq!(error_kind(&reply), ErrorKind::BadRequest, "{reply}");
    assert!(
        reply["message"].as_str().unwrap().contains("hello"),
        "{reply}"
    );

    // A client of a future revision: a foreign version, fields this build
    // has never seen — and still the daemon names itself, in every
    // dimension a client judges (C84): contract, artifact, provider.
    let reply = raw
        .ask(json!({
            "req": "hello",
            "version": "9.9.9",
            "contract": { "rev": "abc" },
            "world": "/future",
        }))
        .await;
    assert_eq!(reply["resp"], "hello");
    assert_eq!(reply["version"], VERSION);
    assert_eq!(reply["contract"], CONTRACT_REVISION);
    assert_eq!(reply["provider"], "mock");
    let artifact = &reply["artifact"];
    assert_eq!(
        Path::new(artifact["path"].as_str().unwrap())
            .canonicalize()
            .unwrap(),
        acqd_built().canonicalize().unwrap(),
        "the daemon names the executable it runs from: {reply}"
    );
    let sha = artifact["sha256"].as_str().unwrap();
    assert!(
        sha.len() == 64 && sha.bytes().all(|b| b.is_ascii_hexdigit()),
        "{reply}"
    );
    assert_eq!(
        sha,
        acquisition_client::artifact::sha256_of(&acqd_built()).unwrap(),
        "the reported hash is the file's"
    );
    assert!(
        artifact["len"].as_u64().unwrap() > 0 && artifact["ino"].as_u64().unwrap() > 0,
        "{reply}"
    );
    assert_eq!(reply["pid"], daemon.id());
    // The world (C83): the daemon's canonical root, this session's store.
    assert_eq!(
        reply["world"],
        World::observe().expect("the session's world").name(),
        "{reply}"
    );
    // resp, version, contract, artifact, pid, provider, world.
    assert_eq!(reply.as_object().unwrap().len(), 7, "{reply}");

    // A hello with nothing but its name.
    let reply = raw.ask(json!({ "req": "hello" })).await;
    assert_eq!(reply["contract"], CONTRACT_REVISION);

    // A request this version does not know is refused, not fatal.
    let reply = raw.ask(json!({ "req": "frobnicate", "x": 1 })).await;
    assert_eq!(error_kind(&reply), ErrorKind::BadRequest);
    let reply = raw.ask(json!({ "req": "list" })).await;
    assert_eq!(reply["resp"], "jobs", "the connection survived: {reply}");

    // A stop of a future shape stops this daemon.
    let reply = raw
        .ask(json!({ "req": "daemon_stop", "reason": "contract test", "world": "/future" }))
        .await;
    assert_eq!(reply, json!({ "resp": "stopping" }));
    daemon.wait_exit(Duration::from_secs(10));
    assert!(
        !endpoint::exists(&session_socket()),
        "the socket is removed on exit"
    );
}

/// A scripted daemon of a future revision on the session's socket: answers
/// every hello with `hello_reply`, every `daemon_stop` with `stop_reply`,
/// anything else with an error of a kind this build does not know. Having
/// answered `stopping` it exits as a daemon does (C133): it removes its
/// endpoint, stops listening and closes the connection, so the client's
/// stop, which returns once the connection has ended and the endpoint is
/// gone (#130), returns at once instead of waiting out its bound.
async fn foreign_daemon(hello_reply: Value, stop_reply: Value) -> tokio::task::JoinHandle<()> {
    let socket = session_socket();
    let mut listener =
        Listener::bind(&socket).unwrap_or_else(|e| panic!("bind {}: {e:?}", socket.display()));
    let exited = std::sync::Arc::new(tokio::sync::Notify::new());
    tokio::spawn(async move {
        loop {
            let stream = tokio::select! {
                stream = listener.accept() => stream.expect("accept"),
                () = exited.notified() => break,
            };
            let (read, mut write) = tokio::io::split(stream);
            let mut reader = BufReader::new(read);
            let hello_reply = hello_reply.clone();
            let stop_reply = stop_reply.clone();
            let (socket, exited) = (socket.clone(), exited.clone());
            tokio::spawn(async move {
                while let Ok(Frame::Line(bytes)) = read_frame(&mut reader, MAX_FRAME_BYTES).await {
                    let frame: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
                    let reply = match frame["req"].as_str() {
                        Some("hello") => hello_reply.clone(),
                        Some("daemon_stop") => stop_reply.clone(),
                        _ => {
                            json!({ "resp": "error", "kind": "future_kind", "message": "unknown to 9.9.9" })
                        }
                    };
                    let mut line = reply.to_string();
                    line.push('\n');
                    if write.write_all(line.as_bytes()).await.is_err() {
                        break;
                    }
                    if reply["resp"] == "stopping" {
                        // On Unix the socket file; on Windows the listener's
                        // waiting instance, which the accept loop drops.
                        let _ = std::fs::remove_file(&socket);
                        exited.notify_one();
                        break;
                    }
                }
            });
        }
    })
}

/// A peer that accepts on the session's socket and never answers: the
/// shape of a wedged daemon. Every connection is held open until the
/// task is aborted.
fn silent_peer() -> tokio::task::JoinHandle<()> {
    let socket = session_socket();
    let mut listener =
        Listener::bind(&socket).unwrap_or_else(|e| panic!("bind {}: {e:?}", socket.display()));
    tokio::spawn(async move {
        let mut held = Vec::new();
        loop {
            let stream = listener.accept().await.expect("accept");
            held.push(stream);
        }
    })
}

/// C86: a peer that accepts and never answers `hello` is reported as
/// unresponsive at the deadline, as its own verdict on every door —
/// observation, the use door (which neither spawns nor replaces over it:
/// the peer still holds the socket afterwards) and stop — naming the
/// pid the world lock's file records. The three doors run together, so
/// the test costs one deadline, not three. The deadline is the default
/// (the isolation list's wider one removed), so the words are the
/// default's; the doors are held to no upper bound, which a starved
/// runner can miss (C137, the deadline's contract is its expiry
/// behaviour).
#[tokio::test]
async fn c86_a_peer_that_never_answers_hello_is_unresponsive_on_every_door_and_never_replaced() {
    let s = session("silent");
    s.set_knob("ACQ_HELLO_WITHIN", None);
    let peer = silent_peer();
    // The lock a wedged daemon would still hold, with its pid in it.
    let world = World::observe().expect("the session's world");
    std::fs::write(world.lock_path(), "424242\n").expect("the lock file");
    // Windows reads the holder beside the lock (a mandatory lock's file
    // cannot be read by another process).
    std::fs::write(Lock::pid_path(&world.lock_path()), "424242\n").expect("the pid file");
    let socket = session_socket().to_str().expect("utf-8").to_string();

    let started = Instant::now();
    let (observed, connected, stopped) = tokio::join!(
        Client::observe(),
        Client::connect(ConnectOptions::interactive(true)),
        Client::stop_any(),
    );
    let elapsed = started.elapsed();
    assert!(
        elapsed >= Duration::from_secs(5),
        "the deadline is 5s; the doors gave up after {elapsed:?}"
    );

    let found = match observed {
        Ok(Observed::Unresponsive(found)) => found,
        Ok(Observed::Absent) => panic!("observed absence over a listening peer"),
        Ok(Observed::Compatible(_)) => panic!("observed a compatible daemon over a silent peer"),
        Ok(Observed::Incompatible(found)) => panic!("observed {found} over a silent peer"),
        Err(e) => panic!("observation failed instead of reporting: {e:#}"),
    };
    assert_eq!(found.at.socket, socket);
    assert_eq!(found.holder, Some(424242));
    let sentence = found.to_string();
    assert!(
        sentence.contains("did not answer hello within 5s")
            && sentence.contains(if cfg!(windows) {
                "taskkill /F /PID 424242"
            } else {
                "kill 424242"
            }),
        "{sentence}"
    );
    // On macOS a healthy daemon misses the deadline at Keychain's prompts
    // too: the sentence says to answer them before it says to kill.
    assert_eq!(
        sentence.contains("if Keychain is asking"),
        cfg!(target_os = "macos"),
        "{sentence}"
    );

    match connected {
        Err(ConnectError::Unresponsive(found)) => assert_eq!(found.holder, Some(424242)),
        Err(other) => panic!("the use door reported {other} instead"),
        Ok(_) => panic!("the use door opened over a silent peer"),
    }

    let err = stopped.expect_err("a stop the peer never acknowledged is an error");
    assert!(
        format!("{err:#}").contains("did not answer hello within 5s"),
        "{err:#}"
    );

    // Nothing spawned or replaced: the peer still holds the socket and
    // still accepts.
    assert!(!peer.is_finished(), "the peer is gone");
    endpoint::connect(&session_socket())
        .await
        .expect("the peer still listens on the session's socket");
    peer.abort();
    let _ = peer.await;
    drop(s);
}

/// C137: the handshake's deadline at zero (`ACQ_HELLO_WITHIN=0`) expires
/// at once against a peer that answers `hello` as a healthy daemon does:
/// every door gives the unresponsive verdict naming zero seconds, with
/// no real waiting, because a reply is in time only before the deadline.
/// The same peer under a deadline is identified, so the verdict was the
/// deadline's and not the peer's. No wall-clock bound is asserted.
#[tokio::test]
async fn c137_the_handshake_deadline_at_zero_is_unresponsive_against_a_peer_that_answers() {
    let s = session("hello-zero");
    let hello = json!({
        "resp": "hello",
        "version": VERSION,
        "contract": CONTRACT_REVISION,
        "artifact": { "rev": "abc" },
        "pid": 4242,
        "provider": "mock",
        "world": World::observe().expect("the session's world").name(),
    });
    let peer = foreign_daemon(hello, json!({ "resp": "stopping" })).await;

    s.set_knob("ACQ_HELLO_WITHIN", Some("0"));
    let (observed, connected, stopped) = tokio::join!(
        Client::observe(),
        Client::connect(ConnectOptions::interactive(true)),
        Client::stop_any(),
    );
    let found = match observed {
        Ok(Observed::Unresponsive(found)) => found,
        Ok(Observed::Absent) => panic!("observed absence over a listening peer"),
        Ok(Observed::Compatible(_)) => panic!("the deadline at zero let a reply through"),
        Ok(Observed::Incompatible(found)) => panic!("the deadline at zero let {found} through"),
        Err(e) => panic!("observation failed instead of reporting: {e:#}"),
    };
    assert_eq!(found.within, Duration::ZERO);
    let sentence = found.to_string();
    assert!(
        sentence.contains("did not answer hello within 0s"),
        "the effective deadline, not the default: {sentence}"
    );
    match connected {
        Err(ConnectError::Unresponsive(found)) => assert_eq!(found.within, Duration::ZERO),
        Err(other) => panic!("the use door reported {other} instead"),
        Ok(_) => panic!("the use door opened under a deadline of zero"),
    }
    let err = stopped.expect_err("a stop under a deadline of zero is unresponsive");
    assert!(
        format!("{err:#}").contains("did not answer hello within 0s"),
        "{err:#}"
    );

    // The same peer, the deadline the isolation list sets: identified.
    s.set_knob("ACQ_HELLO_WITHIN", Some("60"));
    match Client::observe().await {
        Ok(Observed::Incompatible(found)) => assert_eq!(found.pid(), 4242),
        Ok(Observed::Unresponsive(found)) => panic!("a peer that answers: {found}"),
        Ok(Observed::Absent) => panic!("absent"),
        Ok(Observed::Compatible(_)) => panic!("a scripted peer is never this client's"),
        Err(e) => panic!("{e:#}"),
    }
    peer.abort();
    let _ = peer.await;
    let _ = std::fs::remove_file(session_socket());
}

/// #103 (C137, the deadline's contract is its expiry behaviour; C86's
/// deadline): an observe request after the handshake runs under the
/// handshake's deadline, read at the request. At zero it expires against
/// a peer that answers at once, as a healthy daemon does — so the refusal
/// is the deadline's, not the peer's — and the refusal names the peer's
/// socket and pid, what was asked, zero seconds and the kill line. The
/// connection that met it is poisoned: a second request is refused with
/// the same words and sends no frame. A cancel and a quote under the
/// same deadline at zero are not bounded here: the peer's own answer
/// comes back.
///
/// The test is the peer, so nothing is asserted about a task's progress
/// (fix round 3, after three findings on a spawned peer): it binds the
/// session's socket, accepts and answers in the same task as the client
/// it drives (`tokio::join!`), replaying the `hello` a real daemon of
/// this world gave so the observe door finds it compatible, and when the
/// client is dropped it reads its own end of the connection to EOF, so
/// the transcript it asserts is complete by its own reading. No
/// wall-clock bound is asserted.
#[tokio::test]
async fn c137_an_observe_request_under_the_deadline_at_zero_is_refused_naming_the_peer_and_an_acting_one_is_not()
 {
    let s = session("stalled");
    // A real daemon's hello, replayed below.
    let daemon = start_daemon_in_process(&s, &acqd_built()).await;
    let world = World::observe().expect("the session's world");
    let (reply, pid) = {
        let mut raw = Raw::open().await;
        let reply = raw
            .ask(json!({
                "req": "hello",
                "version": VERSION,
                "contract": CONTRACT_REVISION,
                "world": world.name(),
            }))
            .await;
        let pid = u32::try_from(reply["pid"].as_u64().expect("the daemon's pid")).unwrap();
        (reply, pid)
    };
    drop(daemon);

    /// The test's end of one connection: what it has read, in order.
    struct Peer {
        // The accepted end's halves (`endpoint::Accepted`): one type with
        // the connecting end's on Unix, the server pipe on Windows.
        reader: BufReader<ReadHalf<endpoint::Accepted>>,
        write: WriteHalf<endpoint::Accepted>,
        transcript: Vec<String>,
    }

    impl Peer {
        /// Read the next request frame and answer it: the `req` read.
        async fn serve(&mut self, answer: &Value) -> String {
            let bytes = match read_frame(&mut self.reader, MAX_FRAME_BYTES).await {
                Ok(Frame::Line(bytes)) => bytes,
                other => panic!("the peer expected a request frame: {other:?}"),
            };
            let frame: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
            let req = frame["req"].as_str().unwrap_or("?").to_string();
            self.transcript.push(req.clone());
            let mut line = answer.to_string();
            line.push('\n');
            self.write
                .write_all(line.as_bytes())
                .await
                .expect("the peer's write");
            req
        }

        /// Read to EOF (the client has hung up): the whole transcript. A
        /// reset is that hang-up too (#144): a Unix socket reports the
        /// client closing with bytes it never read, this peer's answers,
        /// as `ConnectionReset`, and `ConnectionAborted` is its Windows
        /// twin. Whether the client should drain before it closes is the
        /// daemon area's question, not this peer's.
        async fn read_to_eof(mut self) -> Vec<String> {
            loop {
                match read_frame(&mut self.reader, MAX_FRAME_BYTES).await {
                    Ok(Frame::Line(bytes)) => {
                        let frame: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
                        self.transcript
                            .push(frame["req"].as_str().unwrap_or("?").to_string());
                    }
                    Ok(Frame::Closed) => return self.transcript,
                    Err(e)
                        if matches!(
                            e.kind(),
                            std::io::ErrorKind::ConnectionReset
                                | std::io::ErrorKind::ConnectionAborted
                        ) =>
                    {
                        return self.transcript;
                    }
                    other => panic!("the peer's read after the hang-up: {other:?}"),
                }
            }
        }
    }

    let socket = session_socket();
    let mut listener = Listener::bind(&socket).unwrap_or_else(|e| panic!("bind: {e:?}"));
    /// Accept the client's connection and answer its hello with the
    /// replayed one, in the same task as the door that connects.
    async fn accept_and_greet(listener: &mut Listener, reply: &Value) -> Peer {
        let stream = listener.accept().await.expect("accept");
        let (read, write) = tokio::io::split(stream);
        let mut peer = Peer {
            reader: BufReader::new(read),
            write,
            transcript: Vec::new(),
        };
        assert_eq!(peer.serve(reply).await, "hello");
        peer
    }
    let scripted = json!({ "resp": "error", "kind": "refused", "message": "scripted" });

    // Observed under the isolation list's deadline: this client's daemon.
    let (mut client, mut peer) =
        tokio::join!(compatible_client(), accept_and_greet(&mut listener, &reply));
    assert_eq!(client.daemon().pid(), pid);
    let socket = socket.to_str().expect("utf-8").to_string();

    // The status under the deadline at zero, the peer answering at once:
    // refused as the deadline's expiry whatever the peer did.
    s.set_knob("ACQ_HELLO_WITHIN", Some("0"));
    let (refused, served) = tokio::join!(
        client.request(&Request::DaemonStatus),
        peer.serve(&scripted)
    );
    assert_eq!(served, "daemon_status");
    let err = refused.expect_err("the deadline at zero let an answer through");
    let stalled = acquisition_client::client::Stalled::find(&err)
        .unwrap_or_else(|| panic!("not the bound's refusal: {err:#}"));
    assert_eq!(stalled.within, Duration::ZERO);
    assert_eq!(stalled.pid, pid);
    assert_eq!(stalled.at.socket, socket);
    let sentence = format!("{err:#}");
    assert!(
        sentence.contains(&format!(
            "the daemon at {socket} (pid {pid}) answered the handshake and then not its status within 0s"
        )) && sentence.contains(&format!(
            "stop it by hand (`{} {pid}`) and run the command again",
            if cfg!(windows) { "taskkill /F /PID" } else { "kill" }
        )),
        "{sentence}"
    );

    // Poisoned: the next request is refused the same way and sends no
    // frame. The client hangs up, and the test reads its own end to EOF:
    // the hello and the status, nothing after.
    let again = client
        .request(&Request::List)
        .await
        .expect_err("a request on a stalled connection");
    assert_eq!(format!("{again:#}"), sentence);
    drop(client);
    assert_eq!(
        peer.read_to_eof().await,
        vec!["hello".to_string(), "daemon_status".to_string()],
        "the poisoned connection carried a frame after its expiry"
    );

    // A cancel (acting) and a quote (waiting on the start's restore) are
    // not bounded here: under the deadline at zero the peer's own answer
    // comes back.
    s.set_knob("ACQ_HELLO_WITHIN", Some("60"));
    let (mut client, mut peer) =
        tokio::join!(compatible_client(), accept_and_greet(&mut listener, &reply));
    s.set_knob("ACQ_HELLO_WITHIN", Some("0"));
    for (req, name) in [
        (Request::Cancel { id: 1 }, "cancel"),
        (
            Request::Quote {
                jobs: Vec::new(),
                account: None,
            },
            "quote",
        ),
    ] {
        let (answered, served) = tokio::join!(client.request(&req), peer.serve(&scripted));
        assert_eq!(served, name);
        match answered {
            Ok(Response::Error { kind, message }) => {
                assert_eq!(kind, ErrorKind::Refused, "{name}");
                assert_eq!(message, "scripted", "{name}");
            }
            other => panic!("{name} did not get the peer's answer: {other:?}"),
        }
    }
    drop(client);
    assert_eq!(
        peer.read_to_eof().await,
        vec![
            "hello".to_string(),
            "cancel".to_string(),
            "quote".to_string()
        ]
    );
    drop(listener);
    let _ = std::fs::remove_file(session_socket());
}

/// The client side: a daemon whose frames carry fields this build has
/// never seen is identified as another contract — reported, not used —
/// and `stop_any` stops it; a stop it refuses is reported by message.
#[tokio::test]
async fn c85_a_client_identifies_and_stops_a_foreign_daemon_across_a_contract_mismatch() {
    let _s = session("foreign");
    // On this session's world, so the mismatch reported is the contract's
    // (another world is judged first and reported alone: the next test).
    let hello = json!({
        "resp": "hello",
        "version": "9.9.9",
        "contract": "ffffffffffff",
        "artifact": { "rev": "abc" },
        "pid": 4242,
        "provider": "mock",
        "world": World::observe().expect("the session's world").name(),
    });
    let peer = foreign_daemon(
        hello.clone(),
        json!({ "resp": "stopping", "world": "/future" }),
    )
    .await;

    let found = match Client::observe().await.expect("observe") {
        Observed::Incompatible(found) => found,
        Observed::Absent => panic!("absent"),
        Observed::Compatible(_) => panic!("a foreign daemon is never this client's"),
        Observed::Unresponsive(found) => panic!("{found}"),
    };
    assert_eq!(found.version(), "9.9.9");
    assert_eq!(found.contract(), "ffffffffffff");
    assert_eq!(
        found.artifact(),
        None,
        "an unreadable artifact reads as none"
    );
    assert_eq!(found.pid(), 4242);
    assert!(
        !found.contract_matches() && found.provider_matches(),
        "{found}"
    );

    let err = match Client::connect(ConnectOptions {
        spawn: false,
        replace: false,
    })
    .await
    {
        Err(e) => e.to_string(),
        Ok(_) => panic!("a foreign daemon is never used"),
    };
    assert!(err.contains("another contract"), "{err}");

    let stopped = Client::stop_any().await.expect("stop").expect("a daemon");
    assert_eq!(stopped.pid(), 4242);
    peer.abort();
    let _ = peer.await;

    // The same daemon refusing the stop with an error of a future kind:
    // the message reaches the human; nothing counts as a stop.
    let peer = foreign_daemon(
        hello,
        json!({ "resp": "error", "kind": "future_kind", "message": "not while a world lock is held" }),
    )
    .await;
    let err = Client::stop_any().await.unwrap_err().to_string();
    assert!(
        err.contains("refused to stop") && err.contains("world lock"),
        "{err}"
    );
    peer.abort();
    let _ = peer.await;
    let _ = std::fs::remove_file(session_socket());
}

/// C83 at the client's door. The socket derives from the world, so a
/// daemon on another world is not on this shell's socket at all: from
/// another root it is absent, and a shell whose own root does not exist
/// has no world, no socket, and creates nothing by looking. What the
/// wire still pins: a daemon that answers on *this* world's socket
/// claiming another root (a scripted peer) is judged on the world first
/// and reported alone; a use door that would replace any other mismatch
/// answers `OtherWorld` and leaves it running; `stop_any` still stops it.
#[tokio::test]
async fn c83_a_daemon_on_another_world_is_refused_and_never_replaced() {
    let s = session("world");
    let mut daemon = start_daemon_in_process(&s, &acqd_built()).await;
    let pid = daemon.id();
    let home = World::observe().expect("the session's world");
    let home_socket = session_socket();

    // Another world is another socket: this shell moves to another root
    // and finds nothing there — the daemon's socket derives from `home`.
    let elsewhere = s.base.join("elsewhere");
    std::fs::create_dir_all(&elsewhere).unwrap();
    s.point_world_at(&elsewhere);
    assert_ne!(
        session_socket(),
        home_socket,
        "another world, another socket"
    );
    assert!(
        matches!(Client::observe().await.expect("observe"), Observed::Absent),
        "a daemon on another world is not on this world's socket"
    );
    // A shell whose root does not exist has no world and no socket:
    // observation finds nothing and creates nothing.
    s.point_world_at(&s.base.join("nowhere"));
    assert!(matches!(
        Client::observe().await.expect("observe"),
        Observed::Absent
    ));
    assert!(
        !s.base.join("nowhere").exists(),
        "observation created the root"
    );

    // Back home: the same daemon, untouched, is this client's; stopped
    // from home, on its own socket.
    s.point_world_at(&s.base.join("store"));
    let client = compatible_client().await;
    assert_eq!(client.daemon().pid(), pid, "the daemon was replaced");
    assert_eq!(
        client.daemon().socket(),
        home_socket.to_str().unwrap(),
        "{}",
        client.daemon()
    );
    drop(client);
    let stopped = Client::stop_any().await.expect("stop").expect("a daemon");
    assert_eq!(stopped.pid(), pid);
    daemon.wait_exit(Duration::from_secs(10));
    assert!(
        !endpoint::exists(&home_socket),
        "the socket is removed on exit"
    );

    // What the wire pins: a peer on this world's socket claiming another
    // root — the world judged first and named alone, never replaced by
    // the use door, stopped by `stop_any`.
    let peer = foreign_daemon(
        json!({
            "resp": "hello", "version": VERSION, "contract": CONTRACT_REVISION,
            "pid": 4242, "provider": "mock", "world": "/somewhere/else",
        }),
        json!({ "resp": "stopping" }),
    )
    .await;
    let found = match Client::observe().await.expect("observe") {
        Observed::Incompatible(found) => found,
        Observed::Absent => panic!("absent"),
        Observed::Compatible(_) => panic!("a daemon on another world is never this client's"),
        Observed::Unresponsive(found) => panic!("{found}"),
    };
    assert_eq!(found.pid(), 4242);
    assert_eq!(found.world(), "/somewhere/else", "{found}");
    assert!(
        !found.world_matches() && found.contract_matches(),
        "{found}"
    );
    let report = found.report();
    assert_eq!(report["world_matches"], false, "{report}");
    assert_eq!(report["wanted"]["world"], home.name(), "{report}");
    assert_eq!(
        report["socket"],
        home_socket.display().to_string(),
        "{report}"
    );
    let text = found.to_string();
    assert!(
        text.contains("another world") && !text.contains("another contract"),
        "{text}"
    );
    let err = match Client::connect(ConnectOptions::interactive(true)).await {
        Err(e) => e,
        Ok(_) => panic!("a daemon on another world was used"),
    };
    assert!(
        matches!(&err, ConnectError::OtherWorld { found } if found.pid() == 4242),
        "{err:?}"
    );
    assert!(err.to_string().contains("never replaces it"), "{err}");
    let stopped = Client::stop_any().await.expect("stop").expect("a daemon");
    assert_eq!(stopped.pid(), 4242);
    peer.abort();
    let _ = peer.await;
    let _ = std::fs::remove_file(session_socket());
}

// ---- the child a door spawns (C147) ---------------------------------------

/// The session's daemon log, created empty: a daemon that loses the
/// world lock appends its refusal to a log that exists and creates none.
fn session_log() -> PathBuf {
    let log = World::observe()
        .expect("the session's world")
        .log_path("mock");
    std::fs::create_dir_all(log.parent().expect("a log directory")).expect("the log directory");
    std::fs::write(&log, "").expect("the log");
    log
}

/// The daemons this test process started and has not reaped: its
/// children named `acqd`, or any of its children left a zombie (`ps`
/// itself excluded).
#[cfg(unix)]
fn unreaped_daemons() -> Vec<String> {
    let ps = std::process::Command::new("ps")
        .args(["-A", "-o", "pid=,ppid=,stat=,comm="])
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("ps");
    let own = ps.id().to_string();
    let out = ps.wait_with_output().expect("ps");
    let me = std::process::id().to_string();
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter(|line| {
            let cols: Vec<&str> = line.split_whitespace().collect();
            cols.len() >= 4
                && cols[1] == me
                && cols[0] != own
                && (cols[2].starts_with('Z') || cols[3].contains("acqd"))
        })
        .map(str::to_string)
        .collect()
}

/// C147, a lost spawn race looks once more for the winner, however late
/// the winner starts: the world lock is held (here by the test, standing
/// in for a daemon between its lock and its bind), so the daemon the use
/// door spawns exits on it. The winner is started only once the door has
/// reaped that child — the door has seen it exit, and is looking — and
/// then only after a hold, so its start comes late. The door's answer is
/// the look's: it opens on the winner, a daemon it did not start, rather
/// than reporting a failed spawn.
///
/// No step waits on a clock (C137, a deadline's contract is its
/// behaviour at expiry). The loser's exit is the lock's doing, seen in
/// its refusal in the log; the reaper's wait is seen in the process
/// table, the child gone from it; the winner's start is the test's own
/// act after both. The look's bound is the handshake's knob at its
/// largest, not the isolation list's sixty seconds, so however long a
/// starved runner takes to start the winner, the look outlasts it; and
/// the startup deadline is its knob at its largest too, so however long
/// the loser takes to reach the lock and exit, the door is still
/// waiting on it. A door that never answers is ended by the runner's
/// hang bound (nextest's profile), not by an assertion on time.
#[cfg(unix)]
#[tokio::test]
async fn c147_a_spawn_race_is_won_however_late_the_winner_starts() {
    let s = session("lost-race");
    harness::link_beside_this_test(&acqd_built());
    s.set_knob("ACQ_HELLO_WITHIN", Some(LONGEST_WITHIN));
    s.set_knob("ACQ_STARTUP_WITHIN", Some(LONGEST_WITHIN));
    let world = World::observe().expect("the session's world");
    let log = session_log();
    let held = Lock::acquire(&world.lock_path()).expect("the world lock");
    let winner = async {
        // The loser's refusal in the log, then the loser reaped by the
        // door: it saw the exit, and is looking for the winner. A state
        // waited on, not a time.
        while !std::fs::read_to_string(&log)
            .unwrap_or_default()
            .contains("another daemon holds this world")
            || !unreaped_daemons().is_empty()
        {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        // The winner's start, held: late, as a starved runner's is.
        tokio::time::sleep(Duration::from_secs(2)).await;
        drop(held);
        let (winner, _said) = harness::spawn_drained(
            harness::daemon_command(&acqd_built(), &s.base),
            &acqd_built(),
        );
        winner
    };
    let (connected, mut winner) =
        tokio::join!(Client::connect(ConnectOptions::interactive(true)), winner);
    let client = connected.unwrap_or_else(|e| panic!("the door reported {e} over the winner"));
    assert_eq!(client.daemon().pid(), winner.id(), "the winner answered");
    assert!(
        !client.spawned(),
        "the daemon the door found is not the one it started"
    );
    drop(client);
    Client::stop_any().await.expect("stop").expect("a daemon");
    // The stop ends the winner; its exit is waited, not timed.
    winner.0.wait().expect("the winner's exit");
}

/// A deadline knob at its largest (C137): a look, a handshake or a
/// startup under it outlasts any runner, so the hang bound is the
/// runner's.
#[cfg(unix)]
const LONGEST_WITHIN: &str = "2147483";

/// C147 at the deadline: a spawned daemon that exited on a world lock no
/// daemon then binds for fails the spawn when the look's deadline — the
/// handshake's knob, forced to zero (C137) — has passed, naming the
/// lock's holder.
#[tokio::test]
async fn c147_a_spawn_that_lost_the_world_lock_and_finds_no_winner_fails_naming_the_holder() {
    let s = session("lost-alone");
    harness::link_beside_this_test(&acqd_built());
    let world = World::observe().expect("the session's world");
    let _log = session_log();
    let _held = Lock::acquire(&world.lock_path()).expect("the world lock");
    s.set_knob("ACQ_HELLO_WITHIN", Some("0"));
    let err = match Client::connect(ConnectOptions::interactive(true)).await {
        Err(e) => e,
        Ok(client) => panic!("a door opened on {}", client.daemon()),
    };
    assert!(matches!(err, ConnectError::SpawnFailed { .. }), "{err:?}");
    let text = err.to_string();
    assert!(
        text.contains(&format!(
            "the world's lock names pid {}, and no daemon answered on its socket within 0s",
            std::process::id()
        )) && text.contains("another daemon holds this world"),
        "{text}"
    );
}

/// C137, the startup deadline at zero (`ACQ_STARTUP_WITHIN=0`): a use
/// door that spawned a daemon reports the spawn failed, naming zero
/// seconds, at its first look after the spawn, whatever the daemon is
/// doing — a bind is in time only if it is seen before the deadline. The
/// world lock is held, so the daemon the door spawned exits on it rather
/// than running on, and the test waits for its refusal in the log, a
/// state, before letting the lock go. The same door under the isolation
/// list's deadline then opens on the daemon it spawns, so the failure
/// was the deadline's. No wall-clock bound is asserted.
#[tokio::test]
async fn c137_the_startup_deadline_at_zero_fails_the_spawn_whatever_the_daemon_does() {
    let s = session("startup-zero");
    harness::link_beside_this_test(&acqd_built());
    let world = World::observe().expect("the session's world");
    let log = session_log();
    let held = Lock::acquire(&world.lock_path()).expect("the world lock");
    s.set_knob("ACQ_STARTUP_WITHIN", Some("0"));
    let err = match Client::connect(ConnectOptions::interactive(true)).await {
        Err(e) => e,
        Ok(client) => panic!("a door opened on {}", client.daemon()),
    };
    assert!(matches!(err, ConnectError::SpawnFailed { .. }), "{err:?}");
    let text = err.to_string();
    assert!(
        text.contains("did not bind") && text.contains("within 0s"),
        "the effective deadline, not the default: {text}"
    );
    // The spawned daemon's exit on the lock, waited as a state.
    while !std::fs::read_to_string(&log)
        .unwrap_or_default()
        .contains("another daemon holds this world")
    {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    drop(held);

    // The same door, the deadline the isolation list sets: it opens.
    s.set_knob("ACQ_STARTUP_WITHIN", Some("60"));
    let client = Client::connect(ConnectOptions::interactive(true))
        .await
        .unwrap_or_else(|e| panic!("the door reported {e} under the list's deadline"));
    assert!(client.spawned(), "the door started the daemon");
    drop(client);
    Client::stop_any().await.expect("stop").expect("a daemon");
}

/// C147, a failed stop is reported: a daemon of another contract on this
/// world refuses the stop the use door sends to replace it, and stays.
/// The door's answer is that stop's error, not "still, after a respawn"
/// — nothing was respawned.
#[tokio::test]
async fn c147_a_stop_that_fails_on_the_replace_path_is_reported() {
    let _s = session("stop-fails");
    let hello = json!({
        "resp": "hello",
        "version": "9.9.9",
        "contract": "ffffffffffff",
        "pid": 4242,
        "provider": "mock",
        "world": World::observe().expect("the session's world").name(),
    });
    let peer = foreign_daemon(
        hello,
        json!({ "resp": "error", "kind": "future_kind", "message": "not while a world lock is held" }),
    )
    .await;
    let err = match Client::connect(ConnectOptions::interactive(true)).await {
        Err(e) => e,
        Ok(_) => panic!("a foreign daemon is never used"),
    };
    match &err {
        ConnectError::Incompatible {
            found,
            because: NotReplaced::StopFailed(error),
        } => {
            assert_eq!(found.pid(), 4242);
            assert!(
                error.contains("refused to stop") && error.contains("world lock"),
                "{error}"
            );
        }
        other => panic!("the door reported {other:?}"),
    }
    assert!(
        err.to_string()
            .contains("the stop sent to replace it failed"),
        "{err}"
    );
    peer.abort();
    let _ = peer.await;
    let _ = std::fs::remove_file(session_socket());
}

/// C147, a client reaps the child it spawns: the daemon a use door
/// started, stopped after the client that started it is gone, does not
/// stay behind as this process's zombie. The exit is forced by the stop;
/// the test then waits on the reaper's wait, seen in the process table
/// (the pid gone from it), and on no clock (C137): a reaper that never
/// waits leaves the pid in state Z, and the runner's hang bound
/// (nextest's profile) ends the test, the state it last saw printed. The
/// door's startup and handshake deadlines are their knobs at their
/// largest, so a daemon slow to bind or to answer on a starved runner
/// is waited for, not failed.
#[cfg(unix)]
#[tokio::test]
async fn c147_a_daemon_the_door_spawned_is_reaped_when_it_exits() {
    let s = session("reaped");
    harness::link_beside_this_test(&acqd_built());
    s.set_knob("ACQ_HELLO_WITHIN", Some(LONGEST_WITHIN));
    s.set_knob("ACQ_STARTUP_WITHIN", Some(LONGEST_WITHIN));
    let client = Client::connect(ConnectOptions::interactive(true))
        .await
        .unwrap_or_else(|e| panic!("{e}"));
    assert!(client.spawned(), "the door started the daemon");
    let pid = client.daemon().pid();
    drop(client);
    let stopped = Client::stop_any().await.expect("stop").expect("a daemon");
    assert_eq!(stopped.pid(), pid);
    // A zombie still answers `ps` (state Z); a reaped process does not.
    let mut seen = String::new();
    loop {
        let out = std::process::Command::new("ps")
            .args(["-o", "stat=", "-p", &pid.to_string()])
            .output()
            .expect("ps");
        let stat = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if stat.is_empty() {
            break;
        }
        if stat != seen {
            eprintln!("the spawned daemon (pid {pid}) is still this process's child, state {stat}");
            seen = stat;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

// ---- frames ---------------------------------------------------------------

/// Malformed and oversize frames get `bad_request` and the connection
/// stays; frames are answered in order, one at a time; a blank line is
/// nothing.
#[tokio::test]
async fn c85_malformed_and_oversize_frames_get_bad_request_and_the_connection_stays() {
    let s = session("frames");
    let _daemon = start_daemon_in_process(&s, &acqd_built()).await;
    let mut raw = Raw::open().await;
    raw.hello().await;

    let malformed: [&[u8]; 6] = [
        b"not json",
        b"[1, 2]",
        br#"{"req":"submit"}"#,
        br#"{"req":"cancel","id":"forty-two"}"#,
        br#"{"resp":"ack"}"#,
        b"\xff\xfe{}",
    ];
    for frame in malformed {
        raw.send(frame).await;
        let reply = raw.next().await;
        assert_eq!(
            error_kind(&reply),
            ErrorKind::BadRequest,
            "{}: {reply}",
            String::from_utf8_lossy(frame)
        );
    }
    let reply = raw.ask(json!({ "req": "list" })).await;
    assert_eq!(reply["resp"], "jobs", "{reply}");

    // Over the bound by one byte, and valid JSON all the way: discarded
    // through its newline, answered, and the next frame is read intact.
    let head = br#"{"req":"list","pad":""#;
    let tail = br#""}"#;
    let mut oversize = Vec::with_capacity(MAX_FRAME_BYTES + 1);
    oversize.extend_from_slice(head);
    oversize.resize(MAX_FRAME_BYTES + 1 - tail.len(), b'x');
    oversize.extend_from_slice(tail);
    assert_eq!(oversize.len(), MAX_FRAME_BYTES + 1);
    raw.send(&oversize).await;
    let reply = raw.next().await;
    assert_eq!(error_kind(&reply), ErrorKind::BadRequest, "{reply}");
    assert!(
        reply["message"].as_str().unwrap().contains("exceeds"),
        "{reply}"
    );
    let reply = raw.ask(json!({ "req": "list" })).await;
    assert_eq!(reply["resp"], "jobs", "{reply}");

    // Exactly the bound is a frame.
    let mut at_bound = Vec::with_capacity(MAX_FRAME_BYTES);
    at_bound.extend_from_slice(head);
    at_bound.resize(MAX_FRAME_BYTES - tail.len(), b'x');
    at_bound.extend_from_slice(tail);
    raw.send(&at_bound).await;
    let reply = raw.next().await;
    assert_eq!(reply["resp"], "jobs", "{reply}");

    // Blank lines are skipped; pipelined frames are answered in order.
    raw.send(b"").await;
    raw.send(br#"{"req":"list"}"#).await;
    raw.send(br#"{"req":"daemon_status"}"#).await;
    assert_eq!(raw.next().await["resp"], "jobs");
    assert_eq!(raw.next().await["resp"], "daemon_status");
}

/// C127 (no refusal echoes a client value whole), on the wire: a request
/// that fits its frame but carries 20 MiB of backslashes where the
/// daemon refuses it is answered with a refusal that fits the response
/// frame, of the right kind, naming the value's type and a size. Quoted
/// whole, each escape doubles every backslash twice over, and the refusal
/// was over the frame: the client discarded it and the structured error
/// was lost. The first two are frames that do not decode (a string where
/// a number belongs); the third is an account the index does not know.
#[tokio::test]
async fn c127_a_refusal_quoting_a_huge_client_value_fits_the_response_frame() {
    let s = session("huge");
    let _daemon = start_daemon_in_process(&s, &acqd_built()).await;
    let mut client = compatible_client().await;
    // An indexed account, so the logout below reaches the index's lookup.
    login_in_process(&mut client, None).await;
    let mut raw = Raw::open().await;
    raw.hello().await;

    let huge = "\\".repeat(20 << 20);
    let huge_len = huge.len().to_string();
    let cases = [
        (
            "a status whose id is a string",
            json!({ "req": "status", "id": huge }),
            ErrorKind::BadRequest,
            vec!["bad request", "invalid type: string", "bytes, cut)"],
        ),
        (
            "a submit whose priority is a string",
            json!({
                "req": "submit",
                "kind": "sleep",
                "params": {},
                "priority": huge,
                "submitted_by": "contract",
            }),
            ErrorKind::BadRequest,
            vec!["bad request", "invalid type: string", "bytes, cut)"],
        ),
        (
            "a logout of an account nobody knows",
            json!({ "req": "auth_logout", "account": huge }),
            ErrorKind::NotLoggedIn,
            vec!["no account matches", "(a string of", &huge_len],
        ),
    ];
    for (what, request, kind, said) in cases {
        let frame = request.to_string();
        assert!(
            frame.len() < MAX_FRAME_BYTES,
            "{what}: the request fits its frame ({} bytes)",
            frame.len()
        );
        raw.send(frame.as_bytes()).await;
        let reply = match read_frame(&mut raw.reader, MAX_FRAME_BYTES)
            .await
            .expect("read")
        {
            Frame::Line(bytes) => serde_json::from_slice::<Value>(&bytes).expect("json"),
            other => panic!("{what}: the refusal did not fit the response frame: {other:?}"),
        };
        assert_eq!(error_kind(&reply), kind, "{what}");
        let message = reply["message"].as_str().unwrap();
        assert!(
            message.len() <= 1024,
            "{what}: the refusal quotes a bounded head, not {} bytes",
            message.len()
        );
        for words in said {
            assert!(message.contains(words), "{what}: says {words:?}: {message}");
        }
    }
    // The bound cuts no message the decoder writes for a short value: an
    // unknown request is answered with the whole list of the known ones.
    let reply = raw.ask(json!({ "req": "nonesuch" })).await;
    assert_eq!(error_kind(&reply), ErrorKind::BadRequest, "{reply}");
    let message = reply["message"].as_str().unwrap();
    assert!(
        message.contains("`nonesuch`")
            && message.contains("`dashboard`")
            && !message.contains("cut)"),
        "{message}"
    );
    // The connection stays.
    let reply = raw.ask(json!({ "req": "list" })).await;
    assert_eq!(reply["resp"], "jobs", "{reply}");
}

// ---- subscriptions ----------------------------------------------------------

/// After `subscribed` a connection carries events only — a request there
/// is `bad_request`, the bootstrap plane still answers — and a subscriber
/// the daemon's event channel overran is told `resync_required` with the
/// count, after which the snapshot over a request connection is whole.
#[tokio::test]
async fn c85_a_subscription_carries_events_only_and_lag_is_resync_required() {
    let s = session("lag");
    let _daemon = start_daemon_in_process(&s, &acqd_built()).await;

    let mut raw = Raw::open().await;
    raw.hello().await;
    assert_eq!(
        raw.ask(json!({ "req": "subscribe" })).await,
        json!({ "resp": "subscribed" })
    );
    let reply = raw.ask(json!({ "req": "list" })).await;
    assert_eq!(error_kind(&reply), ErrorKind::BadRequest, "{reply}");
    assert!(
        reply["message"].as_str().unwrap().contains("subscription"),
        "{reply}"
    );
    assert_eq!(
        raw.hello().await["provider"],
        "mock",
        "the bootstrap plane works on any connection"
    );
    drop(raw);

    // Subscribe, then snapshot (the pinned order), then leave the
    // subscription unread while the daemon runs more jobs than its event
    // channel holds: the socket fills, the receiver falls behind, and the
    // gap is reported — never skipped.
    let mut subscription = open_subscription().await;
    let mut client = compatible_client().await;
    assert!(
        matches!(client.request(&Request::List).await.unwrap(), Response::Jobs { jobs } if jobs.is_empty())
    );
    const JOBS: usize = 800;
    for _ in 0..JOBS {
        submitted(submit(&mut client, "sleep", json!({ "seconds": 0 }), None).await);
    }
    wait_all_terminal(&mut client, JOBS).await;

    let mut events_before = 0usize;
    let missed = loop {
        match tokio::time::timeout(Duration::from_secs(10), subscription.next()).await {
            Ok(Ok(Some(Signal::Event(_)))) => events_before += 1,
            Ok(Ok(Some(Signal::ResyncRequired { missed }))) => break missed,
            Ok(other) => panic!("{other:?}"),
            Err(_) => panic!(
                "no resync_required after {events_before} events of {} — the channel never overran",
                JOBS * 3
            ),
        }
    };
    assert!(missed >= 1, "{missed}");
    assert!(
        events_before + (missed as usize) <= JOBS * 3,
        "{events_before} seen + {missed} missed exceeds the {} emitted",
        JOBS * 3
    );
    // The snapshot is the authority; the stream was never complete.
    let jobs = match client.request(&Request::List).await.unwrap() {
        Response::Jobs { jobs } => jobs,
        other => panic!("{other:?}"),
    };
    assert_eq!(jobs.len(), JOBS);
    assert!(jobs.iter().all(|j| j.state == JobState::Done));

    // The rest of the sequence: the lagged subscription is dropped — what
    // it still holds predates the snapshot — and a fresh one carries only
    // what happens after it was opened.
    drop(subscription);
    let mut subscription = open_subscription().await;
    let jobs = match client.request(&Request::List).await.unwrap() {
        Response::Jobs { jobs } => jobs,
        other => panic!("{other:?}"),
    };
    let newest = jobs.iter().map(|j| j.id).max().unwrap();
    let next = submitted(submit(&mut client, "sleep", json!({ "seconds": 0 }), None).await);
    assert!(next > newest);
    let first = tokio::time::timeout(Duration::from_secs(10), subscription.next())
        .await
        .expect("an event")
        .expect("open");
    assert!(
        matches!(&first, Some(Signal::Event(job)) if job.id == next),
        "the fresh subscription's first event is the new job, not a leftover: {first:?}"
    );
}

/// A subscriber sees the daemon go as the end of its stream, the request
/// connection fails on its next request, and after a restart a new
/// subscription and a new snapshot work — with the old job's result still
/// answered (C27).
#[tokio::test]
async fn c85_a_subscriber_sees_the_disconnect_and_subscribes_and_snapshots_again_after_a_restart() {
    let s = session("restart");
    let mut daemon = start_daemon_in_process(&s, &acqd_built()).await;
    let mut subscription = open_subscription().await;
    let mut client = compatible_client().await;

    let id = submitted(submit(&mut client, "sleep", json!({ "seconds": 0 }), None).await);
    wait_all_terminal(&mut client, 1).await;
    let mut done = false;
    while !done {
        match tokio::time::timeout(Duration::from_secs(10), subscription.next())
            .await
            .expect("an event")
            .expect("open")
        {
            Some(Signal::Event(job)) => done = job.id == id && job.state == JobState::Done,
            other => panic!("{other:?}"),
        }
    }

    let stopped = Client::stop_any().await.expect("stop").expect("a daemon");
    assert_eq!(stopped.pid(), daemon.id());
    daemon.wait_exit(Duration::from_secs(10));
    let end = tokio::time::timeout(Duration::from_secs(10), subscription.next())
        .await
        .expect("the stream ends");
    assert!(matches!(end, Ok(None) | Err(_)), "{end:?}");
    assert!(
        client.request(&Request::List).await.is_err(),
        "the request connection is gone"
    );
    assert!(matches!(Client::observe().await.unwrap(), Observed::Absent));

    let _daemon = start_daemon_in_process(&s, &acqd_built()).await;
    let mut subscription = open_subscription().await;
    let mut client = compatible_client().await;
    let live = match client.request(&Request::List).await.unwrap() {
        Response::Jobs { jobs } => jobs,
        other => panic!("{other:?}"),
    };
    assert!(
        live.is_empty(),
        "terminal jobs are history, not state: {live:?}"
    );
    assert!(matches!(
        client.request(&Request::Result { id }).await.unwrap(),
        Response::Result { id: got, .. } if got == id
    ));
    let next = submitted(submit(&mut client, "sleep", json!({ "seconds": 0 }), None).await);
    assert!(next > id, "ids continue across the restart");
    let event = tokio::time::timeout(Duration::from_secs(10), subscription.next())
        .await
        .expect("an event")
        .expect("open");
    assert!(
        matches!(&event, Some(Signal::Event(job)) if job.id == next),
        "{event:?}"
    );
}

// ---- error kinds ------------------------------------------------------------

/// Every domain failure a client can stage carries its kind, and none of
/// them is `internal`. (`queue_failed`, `upstream` and `internal` need a
/// fault the wire cannot inject; the daemon's unit tests stage those.)
#[tokio::test]
async fn c85_error_kinds_classify_the_domain_failures_a_client_can_stage() {
    let s = session("kinds");
    let _daemon = start_daemon_in_process(&s, &acqd_built()).await;
    let mut client = compatible_client().await;

    let (kind, message) = kind_of(
        submit(
            &mut client,
            "stashes",
            json!({ "league": "Standard" }),
            None,
        )
        .await,
    );
    assert_eq!(kind, ErrorKind::NotLoggedIn);
    assert!(message.contains("acq auth"), "{message}");
    let (kind, _) = kind_of(submit(&mut client, "stashes", json!({}), Some("nobody")).await);
    assert_eq!(kind, ErrorKind::NotLoggedIn);
    let (kind, _) = kind_of(
        client
            .request(&Request::AuthCheck { account: None })
            .await
            .unwrap(),
    );
    assert_eq!(kind, ErrorKind::NotLoggedIn);
    let (kind, _) = kind_of(
        client
            .request(&Request::Quote {
                jobs: vec![],
                account: Some("nobody".into()),
            })
            .await
            .unwrap(),
    );
    assert_eq!(kind, ErrorKind::NotLoggedIn);

    for request in [
        Request::Status { id: 999 },
        Request::Result { id: 999 },
        Request::Cancel { id: 999 },
        Request::SetPriority {
            id: 999,
            priority: 1,
        },
    ] {
        let (kind, message) = kind_of(client.request(&request).await.unwrap());
        assert_eq!(kind, ErrorKind::UnknownJob, "{request:?}");
        assert_eq!(message, "no job 999");
    }

    let id = submitted(submit(&mut client, "sleep", json!({ "seconds": 30 }), None).await);
    let (kind, message) = kind_of(client.request(&Request::Result { id }).await.unwrap());
    assert_eq!(kind, ErrorKind::WrongState);
    assert!(message.contains("still"), "{message}");
    assert!(matches!(
        client.request(&Request::Cancel { id }).await.unwrap(),
        Response::Ack { .. }
    ));
    // A running job is cancelled at its next slice; until then a second
    // cancel is the same request again, not a wrong state.
    wait_all_terminal(&mut client, 1).await;
    let (kind, message) = kind_of(client.request(&Request::Cancel { id }).await.unwrap());
    assert_eq!(kind, ErrorKind::WrongState);
    assert!(message.contains("already"), "{message}");
    let (kind, _) = kind_of(
        client
            .request(&Request::SetPriority { id, priority: 1 })
            .await
            .unwrap(),
    );
    assert_eq!(kind, ErrorKind::WrongState);

    // Admission refusals need a session: the mock's login page accepts any
    // name, and the flow completes once its own profile job lands (C50).
    login_in_process(&mut client, None).await;
    let (kind, message) = kind_of(
        submit(
            &mut client,
            "apply",
            json!({ "jobs": [{ "kind": "sleep", "params": {} }] }),
            None,
        )
        .await,
    );
    assert_eq!(kind, ErrorKind::Refused);
    assert!(message.contains("vocabulary"), "{message}");
    let (kind, message) = kind_of(
        submit(
            &mut client,
            "apply",
            json!({ "jobs": [{ "kind": "stashes", "params": {} }], "max_requests": 0 }),
            None,
        )
        .await,
    );
    assert_eq!(kind, ErrorKind::Refused);
    assert!(message.contains("budget"), "{message}");
    let (kind, message) = kind_of(
        submit(
            &mut client,
            "stashes",
            json!({ "realm": "ps5", "league": "Standard" }),
            None,
        )
        .await,
    );
    assert_eq!(kind, ErrorKind::Refused);
    assert!(message.contains("ps5"), "{message}");

    // Several sessions and no selector: ambiguous, never guessed. A bare
    // name two of them share is ambiguous too (C51; review finding
    // 2026-09-10); the full name selects.
    login_in_process(&mut client, Some("Alice#1234")).await;
    login_in_process(&mut client, Some("Alice#5678")).await;
    let (kind, message) = kind_of(
        submit(
            &mut client,
            "stashes",
            json!({ "league": "Standard" }),
            None,
        )
        .await,
    );
    assert_eq!(kind, ErrorKind::AmbiguousAccount);
    assert!(message.contains("--account"), "{message}");
    let (kind, _) = kind_of(
        client
            .request(&Request::AuthLogout { account: None })
            .await
            .unwrap(),
    );
    assert_eq!(kind, ErrorKind::AmbiguousAccount);
    for request in [
        Request::Submit {
            kind: "stashes".into(),
            params: json!({ "league": "Standard" }),
            priority: 0,
            submitted_by: "contract".into(),
            account: Some("alice".into()),
        },
        Request::Quote {
            jobs: vec![],
            account: Some("alice".into()),
        },
        Request::AuthCheck {
            account: Some("alice".into()),
        },
        Request::AuthLogout {
            account: Some("alice".into()),
        },
    ] {
        let (kind, message) = kind_of(client.request(&request).await.unwrap());
        assert_eq!(kind, ErrorKind::AmbiguousAccount, "{request:?}");
        assert!(message.contains("Alice#1234, Alice#5678"), "{message}");
    }
    submitted(
        submit(
            &mut client,
            "stashes",
            json!({ "league": "Standard" }),
            Some("Alice#5678"),
        )
        .await,
    );
}

/// C83: a daemon exiting removes the socket file it bound and never a
/// successor's at the same path. Staged directly: with a connection to
/// the daemon held open, its socket is unlinked and another listener
/// bound in its place — what a successor on the same world does when it
/// starts while the predecessor is still on its way out — then the
/// daemon is stopped over the held connection; the new listener's file
/// must survive the exit (seen in a rehearsal, 2026-09-11: a stop
/// followed within a second by a start on the same world lost the new
/// daemon's rendezvous).
#[cfg(unix)]
#[tokio::test]
async fn c83_a_daemon_on_its_way_out_never_unlinks_a_successors_socket() {
    let s = session("exit");
    let mut daemon = start_daemon_in_process(&s, &acqd_built()).await;
    let socket = session_socket();
    let mut held = Raw::open().await;
    held.hello().await;

    // The successor's socket, in the predecessor's place.
    std::fs::remove_file(&socket).expect("unlink the live daemon's socket");
    let successor = tokio::net::UnixListener::bind(&socket).expect("bind in its place");

    let reply = held
        .ask(json!({ "req": "daemon_stop", "reason": "exit pin" }))
        .await;
    assert_eq!(reply, json!({ "resp": "stopping" }));
    daemon.wait_exit(Duration::from_secs(10));

    assert!(
        socket.exists(),
        "the exiting daemon unlinked the successor's socket"
    );
    let probe = tokio::net::UnixStream::connect(&socket).await;
    assert!(probe.is_ok(), "the successor's socket no longer connects");
    let (_accepted, _) = successor.accept().await.expect("the successor accepts");
    drop(successor);
    let _ = std::fs::remove_file(&socket);
}
